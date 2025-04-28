use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Generics, Ident};

use crate::{
    attrs::Attrs,
    shared_derives::{derive_arith_compat, derive_idx_compat},
    utils::token_stream_to_compact_string,
};

pub struct DeriveContextBase {
    pub attrs: Attrs,
    pub name: Ident,
    pub generics: Generics,
    pub self_as_idx: TokenStream,
}

pub struct DeriveContext<C> {
    pub base: DeriveContextBase,
    pub derivs_catalog: HashMap<String, DeriveCatalogEntry<C>>,
    pub derivs_default: Vec<String>,
    pub custom: C,
}

#[allow(clippy::type_complexity)]
pub enum DeriveCatalogEntry<C> {
    Base(Box<dyn FnOnce(&DeriveContextBase) -> TokenStream>),
    Custom(Box<dyn FnOnce(&DeriveContext<C>) -> TokenStream>),
}

impl<C> DeriveCatalogEntry<C> {
    fn call(self, ctx: &DeriveContext<C>) -> TokenStream {
        match self {
            DeriveCatalogEntry::Base(f) => f(&ctx.base),
            DeriveCatalogEntry::Custom(f) => f(ctx),
        }
    }
}

impl<C> DeriveContext<C> {
    pub fn new(attrs: Attrs, name: Ident, generics: Generics, custom: C) -> Self {
        let indexland = &attrs.indexland_path;
        let self_as_idx = quote! { <Self as #indexland::Idx> };
        Self {
            base: DeriveContextBase {
                attrs,
                name,
                generics,
                self_as_idx,
            },
            derivs_catalog: Default::default(),
            derivs_default: Default::default(),
            custom,
        }
    }
    pub fn add_deriv(&mut self, default: bool, name: String, f: DeriveCatalogEntry<C>) {
        if default {
            self.derivs_default.push(name.clone());
        }
        self.derivs_catalog.insert(name, f);
    }
    pub fn add_deriv_shared(
        &mut self,
        default: bool,
        name: impl Into<String>,
        f: impl FnOnce(&DeriveContextBase) -> TokenStream + 'static,
    ) {
        self.add_deriv(default, name.into(), DeriveCatalogEntry::Base(Box::new(f)));
    }
    pub fn add_deriv_custom(
        &mut self,
        default: bool,
        name: impl Into<String>,
        f: impl FnOnce(&Self) -> TokenStream + 'static,
    ) {
        self.add_deriv(
            default,
            name.into(),
            DeriveCatalogEntry::Custom(Box::new(f)),
        );
    }

    pub fn push_unknown_entry_error(&self, entry: &TokenStream, descr: &str) {
        let from_enum = format!("From<{}", self.base.name);
        if descr.starts_with(&from_enum) {
            self.base.attrs.error_list.error_spanned(
                entry,
                format!("Use `From<Self>` instead of `From<{}>`", self.base.name),
            );
        } else {
            self.base.attrs.error_list.error_spanned(
                entry,
                format!("`{descr}` does not name a trait that can be derived"),
            );
        }
    }

    pub fn generate(&mut self) -> TokenStream {
        for entry in &self.base.attrs.blacklist {
            let descr = token_stream_to_compact_string(entry);
            if self.derivs_catalog.remove(&*descr).is_none() {
                self.push_unknown_entry_error(entry, &descr);
            }
        }

        let mut derivations = Vec::new();
        if self.base.attrs.whitelist_active {
            for entry in &self.base.attrs.whitelist {
                let descr = token_stream_to_compact_string(entry);
                match self.derivs_catalog.remove(&*descr) {
                    Some(deriv) => {
                        derivations.push(deriv.call(self));
                    }
                    None => self.push_unknown_entry_error(entry, &descr),
                }
            }
        } else {
            for deriv_descr in &self.derivs_default {
                if let Some(deriv) = self.derivs_catalog.remove(deriv_descr) {
                    derivations.push(deriv.call(self));
                }
            }
        }

        for entry in &self.base.attrs.extra_list {
            let descr = token_stream_to_compact_string(entry);
            match self.derivs_catalog.remove(&*descr) {
                Some(deriv) => {
                    derivations.push(deriv.call(self));
                }
                None => self.push_unknown_entry_error(entry, &descr),
            }
        }

        for arith_compat in &self.base.attrs.arith_compat_list {
            derivations.push(derive_arith_compat(
                &self.base.attrs.indexland_path,
                &self.base.name,
                arith_compat,
            ));
        }

        for idx_compat in &self.base.attrs.idx_compat_list {
            derivations.push(derive_idx_compat(
                &self.base.attrs.indexland_path,
                &self.base.name,
                idx_compat,
            ));
        }

        quote! {
            #(#derivations)*
        }
    }
}
