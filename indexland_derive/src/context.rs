use std::fmt::Display;

use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, DeriveInput, Ident,
    PathSegment,
};

const CRATE: &str = "crate";
const INDEXLAND: &str = "indexland";

#[derive(Default)]
pub struct ErrorList {
    pub errors: Vec<syn::Error>,
}
pub struct Attrs {
    pub indexland_path: syn::Path,
}

pub struct Context {
    pub errors: ErrorList,
    pub attrs: Attrs,
}

impl ErrorList {
    pub fn push_error(&mut self, e: syn::Error) {
        self.errors.push(e);
    }
    pub fn error_spanned_by(
        &mut self,
        pos: impl ToTokens,
        message: impl Display,
    ) {
        self.errors.push(syn::Error::new_spanned(pos, message));
    }
}

fn get_lit_str(
    errs: &mut ErrorList,
    attr_name: &str,
    meta_item_name: &str,
    meta: &ParseNestedMeta,
) -> syn::Result<Option<syn::LitStr>> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    {
        let suffix = lit.suffix();
        if !suffix.is_empty() {
            errs.error_spanned_by(
                lit,
                format!("unexpected suffix `{}` after string literal", suffix),
            );
        }
        Ok(Some(lit.clone()))
    } else {
        errs.error_spanned_by(
            expr,
            format!(
                "expected {} {} attribute to be a string: `{} = \"...\"`",
                INDEXLAND, attr_name, meta_item_name
            ),
        );
        Ok(None)
    }
}

fn parse_lit_into_path(
    errs: &mut ErrorList,
    attr_name: &str,
    meta: &ParseNestedMeta,
) -> syn::Result<Option<syn::Path>> {
    let string = match get_lit_str(errs, attr_name, attr_name, meta)? {
        Some(string) => string,
        None => return Ok(None),
    };

    Ok(match string.parse() {
        Ok(path) => Some(path),
        Err(_) => {
            errs.error_spanned_by(
                &string,
                format!("failed to parse path: {:?}", string.value()),
            );
            None
        }
    })
}

impl Context {
    pub fn from_input(ast: &DeriveInput) -> Context {
        let mut errs = ErrorList::default();
        let mut indexland_path = None;
        for attr in &ast.attrs {
            if !attr.path().is_ident(INDEXLAND) {
                continue;
            }
            let res = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(CRATE) {
                    if let Some(path) =
                        parse_lit_into_path(&mut errs, CRATE, &meta)?
                    {
                        indexland_path = Some(path);
                    }
                } else {
                    errs.push_error(meta.error(format!(
                        "unknown indexland attribute {}",
                        meta.path.to_token_stream().to_string()
                    )));
                }
                Ok(())
            });
            if let Err(e) = res {
                errs.push_error(e);
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
            errors: errs,
            attrs: Attrs { indexland_path },
        }
    }
}
