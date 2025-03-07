use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

use crate::context::Context;

fn get_single_struct_member(
    struct_data: &DataStruct,
) -> Result<&syn::Field, syn::Error> {
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
    Ok(inner)
}

fn derive_idx_for_struct(
    ctx: &mut Context,
    ast: &DeriveInput,
    struct_data: &DataStruct,
) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let inner = get_single_struct_member(struct_data)?;

    let base_type = &inner.ty;

    let indexland = &ctx.attrs.indexland_path;

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name(<#base_type as #indexland::Idx>::ZERO);
            const ONE: Self = #name(<#base_type as #indexland::Idx>::ONE);
            const MAX: Self = #name(<#base_type as #indexland::Idx>::MAX);
            #[inline(always)]
            fn into_usize(self) -> usize {
                <#base_type as #indexland::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                #name(<#base_type as #indexland::Idx>::from_usize(v))
            }
            fn wrapping_add(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                #name(<#base_type as #indexland::Idx>::wrapping_sub(self.0, other.0))
            }
        }
    };
    Ok(output)
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

    let mut ctx = Context::from_input(&ast);

    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let inner = get_single_struct_member(struct_data)?;

    let base_type = &inner.ty;

    let name = &ast.ident;

    let idx_derivation = derive_idx_for_struct(&mut ctx, &ast, struct_data)?;

    ctx.error_list.check()?;

    let indexland = &ctx.attrs.indexland_path;

    let output = quote! {
        #[automatically_derived]
        impl ::core::default::Default for #name {
            fn default() -> Self {
                #indexland::Idx::ZERO
            }
        }

        #[automatically_derived]
        impl ::core::marker::Copy for #name {}

        #[automatically_derived]
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }

        #[automatically_derived]
        impl ::core::hash::Hash for #name {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        #[automatically_derived]
        impl ::core::convert::From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #name(<#base_type as #indexland::Idx>::from_usize(v))
            }
        }

        #[automatically_derived]
        impl ::core::convert::From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                <#base_type as #indexland::Idx>::into_usize(v.0)
            }
        }

        #[automatically_derived]
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }

        #[automatically_derived]
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #name(self.0 + rhs.0)
            }
        }

        #[automatically_derived]
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #name(self.0 - rhs.0)
            }
        }

        #[automatically_derived]
        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        #[automatically_derived]
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        #[automatically_derived]
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }

        #[automatically_derived]
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }

        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}

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
        #idx_derivation
    };

    Ok(output)
}
