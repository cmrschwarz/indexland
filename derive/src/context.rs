use std::{cell::RefCell, fmt::Display};

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{
    parenthesized, punctuated::Punctuated, spanned::Spanned, DeriveInput,
    Ident, LitStr, PathSegment,
};

const INDEXLAND: &str = "indexland";
const CRATE: &str = "crate";
const ONLY: &str = "only";
const OMIT: &str = "omit";
const EXTRA: &str = "extra";
const BOUNDS_CHECKS: &str = "bounds_checks";

const USIZE_ARITH: &str = "usize_arith";

#[derive(Default)]
pub enum BoundsChecksMode {
    #[default]
    DebugOnly,
    Always,
    Never,
}

#[derive(Default)]
pub struct ErrorList {
    pub errors: RefCell<Option<syn::Error>>,
}
pub struct Attrs {
    pub indexland_path: syn::Path,
    pub blacklist: Vec<TokenStream>,
    pub whitelist: Vec<TokenStream>,
    pub extra_list: Vec<TokenStream>,
    // could be active despite being empty
    pub whitelist_active: bool,
    pub bounds_checks_mode: BoundsChecksMode,
    pub enable_usize_arith: bool,
}

pub struct Context {
    pub error_list: ErrorList,
    pub attrs: Attrs,
}

impl ErrorList {
    pub fn is_empty(&self) -> bool {
        self.errors.borrow().is_none()
    }
    pub fn push(&self, e: syn::Error) {
        let mut errs = self.errors.borrow_mut();
        if let Some(prev) = &mut *errs {
            prev.combine(e);
        } else {
            *errs = Some(e);
        }
    }
    pub fn error(&self, span: Span, message: impl Display) {
        self.push(syn::Error::new(span, message));
    }

    pub fn error_spanned(&self, tokens: impl ToTokens, message: impl Display) {
        self.push(syn::Error::new_spanned(tokens, message));
    }
    pub fn check(&self) -> syn::Result<()> {
        match self.errors.take() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl Drop for ErrorList {
    fn drop(&mut self) {
        assert!(self.is_empty())
    }
}

fn split_at_commas(tokens: TokenStream) -> Vec<TokenStream> {
    let mut groups = Vec::new();
    let mut current_group = TokenStream::new();

    let mut angle_bracket_depth = 0usize;

    for token in tokens {
        if let TokenTree::Punct(punct) = &token {
            let c = punct.as_char();
            if c == '<' {
                angle_bracket_depth += 1;
            }
            if c == '>' {
                angle_bracket_depth = angle_bracket_depth.saturating_sub(1);
            }
            if c == ',' && angle_bracket_depth == 0 {
                groups.push(current_group);
                current_group = TokenStream::new();
                continue;
            }
        }
        current_group.extend(Some(token));
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

impl Context {
    pub fn from_input(ast: &DeriveInput) -> Context {
        let errs = ErrorList::default();
        let mut indexland_path = None;
        let mut blacklist = Vec::new();
        let mut first_blacklist = None;
        let mut whitelist = Vec::new();
        let mut first_whitelist = None;
        let mut extra_list = Vec::new();
        let mut first_extra_list = None;
        let mut bounds_checks_mode = BoundsChecksMode::default();
        let mut enable_usize_arith = false;
        for attr in &ast.attrs {
            if !attr.path().is_ident(INDEXLAND) {
                continue;
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(CRATE) {
                    // #[indexland(crate = path::to::indexland)]
                    let v = meta.value()?;
                    let path = v.parse()?;
                    indexland_path = Some(path);
                } else if meta.path.is_ident(BOUNDS_CHECKS) {
                    // e.g. #[indexland(bounds_checks = "always")]
                    let literal: LitStr = meta.value()?.parse()?;
                    let value = literal.value();
                    match &*value {
                        "debug_only" => {
                            bounds_checks_mode = BoundsChecksMode::DebugOnly
                        }
                        "always" => {
                            bounds_checks_mode = BoundsChecksMode::Always
                        }
                        "never" => {
                            bounds_checks_mode = BoundsChecksMode::Never
                        }
                        _ => errs.push(meta.error(format!(
                            r#"unknown bounds checks mode "{value}", expected {}"#,
                            r#""debug_only", "always", or "never""#
                        ))),
                    }
                } else if meta.path.is_ident(USIZE_ARITH) {
                    // #[indexland(usize_arith)]
                    enable_usize_arith = true;
                } else if meta.path.is_ident(OMIT) {
                    // e.g. #[indexland(omit(Display))]
                    let omit;
                    parenthesized!(omit in meta.input);
                    let variants =
                        split_at_commas(omit.cursor().token_stream());

                    // the cursor above is a copy
                    while omit.parse::<TokenTree>().is_ok() {}

                    if first_blacklist.is_none() {
                        first_blacklist = Some(meta.path.span());
                    }
                    blacklist.extend(variants);
                } else if meta.path.is_ident(ONLY) {
                    // e.g. #[indexland(only(Idx))]
                    let only;
                    parenthesized!(only in meta.input);
                    let elements =
                        split_at_commas(only.cursor().token_stream());

                    // the cursor above is a copy
                    while only.parse::<TokenTree>().is_ok() {}

                    if first_whitelist.is_none() {
                        first_whitelist = Some(meta.path.span());
                    }
                    whitelist.extend(elements);
                } else if meta.path.is_ident(EXTRA) {
                    // e.g. #[indexland(extra(Display))]
                    let extra;
                    parenthesized!(extra in meta.input);
                    let elements =
                        split_at_commas(extra.cursor().token_stream());

                    // the cursor above is a copy
                    while extra.parse::<TokenTree>().is_ok() {}

                    if first_extra_list.is_none() {
                        first_extra_list = Some(meta.path.span());
                    }
                    extra_list.extend(elements);
                } else {
                    errs.push(meta.error(format!(
                        "unknown {INDEXLAND} attribute {}",
                        meta.path.to_token_stream()
                    )));
                }
                Ok(())
            });
            if let Err(e) = res {
                errs.push(e);
            }
        }

        let indexland_path = indexland_path.unwrap_or_else(|| {
            let ps: PathSegment =
                Ident::new(INDEXLAND, Span::call_site()).into();
            syn::Path {
                leading_colon: Some(Default::default()),
                segments: Punctuated::from_iter([ps]),
            }
        });

        if let (Some(wl), Some(bl)) = (first_blacklist, first_whitelist) {
            for span in [wl, bl] {
                errs.error(span, "omit() and only() are mutually exclusive");
            }
        }

        Context {
            error_list: errs,
            attrs: Attrs {
                indexland_path,
                whitelist,
                blacklist,
                extra_list,
                whitelist_active: first_whitelist.is_some(),
                bounds_checks_mode,
                enable_usize_arith,
            },
        }
    }
}
