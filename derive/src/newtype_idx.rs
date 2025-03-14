use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Generics, Ident, Type};

use crate::{
    context::{Attrs, BoundsChecksMode, Context, ErrorList},
    utils::{token_stream_to_compact_string, Derivations},
};

struct NewtypeCtx<'a> {
    error_list: ErrorList,
    attrs: Attrs,
    name: &'a Ident,
    generics: &'a Generics,
    base_type: &'a Type,
}

type NewtypeTraitDerivation = fn(&NewtypeCtx) -> TokenStream;

fn derive_idx(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;

    let (impl_generics, ty_generics, where_clause) =
        ctx.generics.split_for_impl();

    let base_type = &ctx.base_type;

    let checked_conversions = match ctx.attrs.bounds_checks_mode {
        BoundsChecksMode::Never => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #name(<#base_type as #indexland::Idx>::from_usize_unchecked(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    <#base_type as #indexland::Idx>::into_usize_unchecked(self.0)
                }
            }
        }
        BoundsChecksMode::DebugOnly => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #[cfg(debug_assertions)]
                    return #name(<#base_type as #indexland::Idx>::from_usize(v));

                    #[cfg(not(debug_assertions))]
                    #name(<#base_type as #indexland::Idx>::from_usize_unchecked(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    #[cfg(debug_assertions)]
                    return <#base_type as #indexland::Idx>::into_usize(self.0);

                    #[cfg(not(debug_assertions))]
                    <#base_type as #indexland::Idx>::into_usize_unchecked(self.0)
                }
            }
        }
        BoundsChecksMode::Always => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #name(<#base_type as #indexland::Idx>::from_usize(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    <#base_type as #indexland::Idx>::into_usize(self.0)
                }
            }
        }
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name(<#base_type as #indexland::Idx>::ZERO);
            const ONE: Self = #name(<#base_type as #indexland::Idx>::ONE);
            const MAX: Self = #name(<#base_type as #indexland::Idx>::MAX);
            #checked_conversions
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #name(<#base_type as #indexland::Idx>::from_usize_unchecked(v))
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                <#base_type as #indexland::Idx>::into_usize_unchecked(self.0)
            }
            fn wrapping_add(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::wrapping_sub(self.0, other.0))
            }
        }
    }
}

fn derive_idx_newtype(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    let (impl_generics, ty_generics, where_clause) =
        ctx.generics.split_for_impl();
    let base_type = &ctx.base_type;
    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::IdxNewtype for #name #ty_generics #where_clause {
            type Base = #base_type;
            #[inline]
            fn new(v: #base_type) -> Self {
                #name(v)
            }
            #[inline]
            fn into_inner(self) -> #base_type {
                self.0
            }
        }
    }
}

fn derive_default(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::default::Default for #name {
            fn default() -> Self {
                #indexland::Idx::ZERO
            }
        }
    }
}

fn derive_clone(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
               *self
            }
        }
    }
}

fn derive_copy(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
    }
}

fn derive_hash(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::hash::Hash for #name {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

    }
}

fn derive_from_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    let base_type = &ctx.base_type;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #name(<#base_type as #indexland::Idx>::from_usize(v))
            }
        }
    }
}

fn derive_from_self_for_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    let base_type = &ctx.base_type;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                <#base_type as #indexland::Idx>::into_usize(v.0)
            }
        }
    }
}

fn derive_debug(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
    }
}

fn derive_display(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    }
}

fn derive_add(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #name(self.0 + rhs.0)
            }
        }
    }
}

fn derive_sub(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #name(self.0 - rhs.0)
            }
        }
    }
}

fn derive_add_assign(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    }
}

fn derive_sub_assign(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    }
}

fn derive_partial_ord(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
    }
}

fn derive_ord(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
    }
}

fn derive_partial_eq(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    }
}

fn derive_eq(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}
    }
}

fn derive_add_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add<usize> for #name {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) + rhs,
                )
            }
        }
    }
}

fn derive_sub_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub<usize> for #name {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) - rhs,
                )
            }
        }
    }
}

fn derive_add_assign_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign<usize> for #name {
            fn add_assign(&mut self, rhs: usize) {
                *self = *self + <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn derive_sub_assign_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign<usize> for #name {
            fn sub_assign(&mut self, rhs: usize) {
                *self = *self -  <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn derivation_list(ctx: &NewtypeCtx) -> Derivations<NewtypeTraitDerivation> {
    let usize_arith = ctx.attrs.enable_usize_arith;
    let mut derivs = Derivations::<NewtypeTraitDerivation>::default();
    derivs.add(true, "Idx", derive_idx);
    derivs.add(true, "IdxEnum", derive_idx_newtype);
    derivs.add(true, "Debug", derive_debug);
    derivs.add(true, "Display", derive_display);
    derivs.add(true, "Default", derive_default);
    derivs.add(true, "Clone", derive_clone);
    derivs.add(true, "Copy", derive_copy);
    derivs.add(true, "Add", derive_add);
    derivs.add(true, "AddAssign", derive_add_assign);
    derivs.add(true, "Sub", derive_sub);
    derivs.add(true, "SubAssign", derive_sub_assign);
    derivs.add(true, "Hash", derive_hash);
    derivs.add(true, "PartialOrd", derive_partial_ord);
    derivs.add(true, "Ord", derive_ord);
    derivs.add(true, "PartialEq", derive_partial_eq);
    derivs.add(true, "Eq", derive_eq);
    derivs.add(true, "From<usize>", derive_from_usize);
    derivs.add(true, "From<Self> for usize", derive_from_self_for_usize);
    derivs.add(usize_arith, "Add<usize>", derive_add_usize);
    derivs.add(usize_arith, "Sub<usize>", derive_sub_usize);
    derivs.add(usize_arith, "AddAssign<usize>", derive_add_assign_usize);
    derivs.add(usize_arith, "SubAssign<usize>", derive_sub_assign_usize);
    derivs
}

fn push_unknown_entry_error(
    ctx: &NewtypeCtx,
    entry: &TokenStream,
    descr: &str,
) {
    let from_enum = format!("From<{}", ctx.name);
    if descr.starts_with(&from_enum) {
        ctx.error_list.error_spanned(
            entry,
            format!("Use `From<Self>` instead of `From<{}>`", ctx.name),
        );
    } else {
        ctx.error_list.error_spanned(
            entry,
            format!("`{descr}` does not name a trait that can be derived"),
        );
    }
}

pub fn derive_idx_newtype_inner(
    ast: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let Data::Struct(struct_data) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports newtype structs",
        ));
    };
    let inner = match &struct_data.fields {
        Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "This macro only supports newtype structs with exactly one member",
                ));
            }
            &fields_unnamed.unnamed[0]
        }
        Fields::Named(_) | Fields::Unit => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports newtype structs",
            ));
        }
    };
    let ctx = Context::from_input(&ast);
    let base_type = &inner.ty;
    let name = &ast.ident;

    let newtype_ctx = NewtypeCtx {
        error_list: ctx.error_list,
        attrs: ctx.attrs,
        name,
        generics: &ast.generics,
        base_type,
    };

    let mut derivs_list = derivation_list(&newtype_ctx);
    for entry in &newtype_ctx.attrs.blacklist {
        let descr = token_stream_to_compact_string(entry);
        if derivs_list.catalog.remove(&*descr).is_none() {
            push_unknown_entry_error(&newtype_ctx, entry, &descr);
        }
    }

    let mut derivations = Vec::new();
    if newtype_ctx.attrs.whitelist_active {
        for entry in &newtype_ctx.attrs.whitelist {
            let descr = token_stream_to_compact_string(entry);
            match derivs_list.catalog.get(&*descr) {
                Some(deriv) => {
                    derivations.push(deriv(&newtype_ctx));
                }
                None => push_unknown_entry_error(&newtype_ctx, entry, &descr),
            }
        }
    } else {
        for deriv_descr in derivs_list.default_derivations {
            if let Some(deriv) = derivs_list.catalog.get(deriv_descr) {
                derivations.push(deriv(&newtype_ctx));
            }
        }
    }

    for entry in &newtype_ctx.attrs.extra_list {
        let descr = token_stream_to_compact_string(entry);
        match derivs_list.catalog.get(&*descr) {
            Some(deriv) => {
                derivations.push(deriv(&newtype_ctx));
            }
            None => push_unknown_entry_error(&newtype_ctx, entry, &descr),
        }
    }

    newtype_ctx.error_list.check()?;

    let output = quote! {
        #(#derivations)*
    };

    Ok(output)
}
