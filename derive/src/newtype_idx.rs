use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

use crate::{
    attrs::{Attrs, BoundsChecksMode},
    derive_context::DeriveContext,
};

struct NewtypeCtxCustom<'a> {
    base_type: &'a Type,
}

type NewtypeCtx<'a> = DeriveContext<NewtypeCtxCustom<'a>>;

fn derive_idx(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;

    let (impl_generics, ty_generics, where_clause) =
        ctx.base.generics.split_for_impl();

    let base_type = &ctx.custom.base_type;

    let checked_conversions = match ctx.base.attrs.bounds_checks_mode {
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
            fn saturating_add(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::saturating_add(self.0, other.0))
            }
            fn saturating_sub(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::saturating_sub(self.0, other.0))
            }
        }
    }
}

fn derive_idx_newtype(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let (impl_generics, ty_generics, where_clause) =
        ctx.base.generics.split_for_impl();
    let base_type = &ctx.custom.base_type;
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
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
    }
}

fn derive_hash(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
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
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let base_type = &ctx.custom.base_type;
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
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let base_type = &ctx.custom.base_type;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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

fn derive_rem(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem for #name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                #name(self.0 % rhs.0)
            }
        }
    }
}

fn derive_add_assign(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    }
}

fn derive_rem_assign(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign for #name {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
    }
}

fn derive_partial_ord(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
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
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}
    }
}

fn derive_add_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_rem_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem<usize> for #name {
            type Output = Self;
            fn rem(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) % rhs,
                )
            }
        }
    }
}

fn derive_add_assign_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign<usize> for #name {
            fn sub_assign(&mut self, rhs: usize) {
                *self = *self - <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn derive_rem_assign_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign<usize> for #name {
            fn rem_assign(&mut self, rhs: usize) {
                *self = *self % <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn fill_derivation_list(ctx: &mut NewtypeCtx) {
    let usize_arith = ctx.base.attrs.enable_usize_arith;
    ctx.add_deriv_custom(true, "Idx", derive_idx);
    ctx.add_deriv_custom(true, "IdxEnum", derive_idx_newtype);
    ctx.add_deriv_custom(true, "Debug", derive_debug);
    ctx.add_deriv_custom(true, "Display", derive_display);
    ctx.add_deriv_custom(true, "Default", derive_default);
    ctx.add_deriv_custom(true, "Clone", derive_clone);
    ctx.add_deriv_custom(true, "Copy", derive_copy);
    ctx.add_deriv_custom(true, "Add", derive_add);
    ctx.add_deriv_custom(true, "AddAssign", derive_add_assign);
    ctx.add_deriv_custom(true, "Sub", derive_sub);
    ctx.add_deriv_custom(true, "SubAssign", derive_sub_assign);
    ctx.add_deriv_custom(true, "Rem", derive_rem);
    ctx.add_deriv_custom(true, "RemAssign", derive_rem_assign);
    ctx.add_deriv_custom(true, "Hash", derive_hash);
    ctx.add_deriv_custom(true, "PartialOrd", derive_partial_ord);
    ctx.add_deriv_custom(true, "Ord", derive_ord);
    ctx.add_deriv_custom(true, "PartialEq", derive_partial_eq);
    ctx.add_deriv_custom(true, "Eq", derive_eq);
    ctx.add_deriv_custom(true, "From<usize>", derive_from_usize);
    ctx.add_deriv_custom(
        true,
        "From<Self> for usize",
        derive_from_self_for_usize,
    );
    ctx.add_deriv_custom(usize_arith, "Add<usize>", derive_add_usize);
    ctx.add_deriv_custom(usize_arith, "Sub<usize>", derive_sub_usize);
    ctx.add_deriv_custom(usize_arith, "Rem<usize>", derive_rem_usize);
    ctx.add_deriv_custom(
        usize_arith,
        "AddAssign<usize>",
        derive_add_assign_usize,
    );
    ctx.add_deriv_custom(
        usize_arith,
        "SubAssign<usize>",
        derive_sub_assign_usize,
    );
    ctx.add_deriv_custom(
        usize_arith,
        "RemAssign<usize>",
        derive_rem_assign_usize,
    );
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
    let attrs = Attrs::from_input(&ast);
    let base_type = &inner.ty;
    let name = ast.ident;

    let mut ctx = NewtypeCtx::new(
        attrs,
        name,
        ast.generics,
        NewtypeCtxCustom { base_type },
    );

    // we don't derive if the type definition is already borked
    ctx.base.attrs.error_list.check()?;

    fill_derivation_list(&mut ctx);

    let res = ctx.generate();

    ctx.base.attrs.error_list.check()?;

    Ok(res)
}
