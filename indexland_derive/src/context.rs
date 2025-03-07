use std::fmt::Display;

use proc_macro2::Span;
use quote::ToTokens;
use syn::{punctuated::Punctuated, DeriveInput, Ident, PathSegment};

const CRATE: &str = "crate";
const INDEXLAND: &str = "indexland";

#[derive(Default)]
pub struct ErrorList {
    pub errors: Option<syn::Error>,
}
pub struct Attrs {
    pub indexland_path: syn::Path,
    //TODO: implement black and whitelist of traits to implement
}

pub struct Context {
    pub error_list: ErrorList,
    pub attrs: Attrs,
}

impl ErrorList {
    pub fn is_empty(&self) -> bool {
        self.errors.is_none()
    }
    pub fn push(&mut self, e: syn::Error) {
        if let Some(prev) = &mut self.errors {
            prev.combine(e);
        } else {
            self.errors = Some(e);
        }
    }
    fn error(&mut self, span: Span, message: impl Display) {
        self.push(syn::Error::new(span, message));
    }
    pub fn error_spanned_by(
        &mut self,
        pos: impl ToTokens,
        message: impl Display,
    ) {
        self.push(syn::Error::new_spanned(pos, message));
    }
    pub fn check(&mut self) -> syn::Result<()> {
        match self.errors.take() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
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

        Context {
            error_list: errs,
            attrs: Attrs { indexland_path },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        assert!(self.error_list.is_empty())
    }
}
