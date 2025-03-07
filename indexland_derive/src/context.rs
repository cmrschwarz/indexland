use std::{cell::RefCell, fmt::Display};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, spanned::Spanned, DeriveInput, Ident, PathSegment,
    Token,
};

const INDEXLAND: &str = "indexland";
const CRATE: &str = "crate";
const ONLY: &str = "only";
const OMIT: &str = "omit";
const WHITELIST_AND_BLACKLIST_ERROR: &str =
    "omit and only are mutually exclusive";

#[derive(Default)]
pub struct ErrorList {
    pub errors: RefCell<Option<syn::Error>>,
}
pub struct Attrs {
    pub indexland_path: syn::Path,
    pub blacklist: Vec<syn::Expr>,
    pub whitelist: Vec<syn::Expr>,
    // could be active despite being empty
    pub whitelist_active: bool,
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

impl Context {
    pub fn from_input(ast: &DeriveInput) -> Context {
        let errs = ErrorList::default();
        let mut indexland_path = None;
        let mut blacklist = Vec::new();
        let mut first_blacklist = None;
        let mut whitelist = Vec::new();
        let mut first_whitelist = None;
        for attr in &ast.attrs {
            if !attr.path().is_ident(INDEXLAND) {
                continue;
            }

            let res = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(CRATE) {
                    // #[indexland(crate = path::to::indexland)]
                    let v = meta.value()?;
                    let path = v.parse()?;
                    if !v.is_empty() {
                        errs.error(
                            v.span(),
                            format!(
                                "unexpected suffix `{}` after indexland crate path",
                                v
                            ),
                        );
                    }
                    indexland_path = Some(path);
                }
                else if meta.path.is_ident(OMIT) {
                    // #[indexland(omit(Display))]
                    let idents = meta.input.parse_terminated(|p| p.parse(), Token![,])?;
                    if first_blacklist.is_none()  {
                        first_blacklist = Some(meta.path.span());
                        if let Some(first) = first_whitelist {
                            errs.error(first, WHITELIST_AND_BLACKLIST_ERROR);
                        }
                    }
                    if first_whitelist.is_some() {
                        errs.error(meta.path.span(), WHITELIST_AND_BLACKLIST_ERROR);
                    }
                    blacklist.extend(idents);
                }
                else if meta.path.is_ident(ONLY) {
                    // #[indexland(only(Idx))]
                    let idents = meta.input.parse_terminated(|p| p.parse(), Token![,])?;
                    if first_whitelist.is_none()  {
                        first_whitelist = Some(meta.path.span());
                        if let Some(first) = first_blacklist {
                            errs.error(first, WHITELIST_AND_BLACKLIST_ERROR);
                        }
                    }
                    if first_blacklist.is_some() {
                        errs.error(meta.path.span(), WHITELIST_AND_BLACKLIST_ERROR);
                    }
                    whitelist.extend(idents);
                }
                else {
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

        Context {
            error_list: errs,
            attrs: Attrs {
                indexland_path,
                whitelist,
                blacklist,
                whitelist_active: first_whitelist.is_some(),
            },
        }
    }
}
