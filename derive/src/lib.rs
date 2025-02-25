//! This crate provides Indexland's derive and attribute macros

//! ```rust
//! use indexland_derive::{EnumIdx, make_enum_idx};
//!
//! // implements the indexland::EnumIdx trait
//! // expects all neccessary impls to be present
//! #[derive(EnumIdx)]
//! enum Direction{
//!     West,
//!     North,
//!     East,
//!     South,
//! };
//!
//! // implicitly adds all neccessary derives
//! #[make_enum_idx]
//! enum Color{
//!     Red,
//!     Green,
//!     Blue
//! };
//! ```

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Adds the following derives:
/// - `Debug`
/// - `Clone + Copy`
/// - `PartialOrd + Ord`
/// - `PartialEq + Eq`
/// - `Hash`
/// - `indexland_derive::Idx + indexland_derive::EnumIdx`
///
/// Implements the following traits:
/// - `Default` (uses first variant)
/// - `Add + AddAssign`
/// - `Sub + SubAssign`
#[proc_macro_attribute]
pub fn make_enum_idx(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    make_enum_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn make_enum_idx_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    let Data::Enum(_) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums.",
        ));
    };

    let name = &ast.ident;

    let output = quote! {
        #[derive(
            Debug,
            Clone, Copy,
            PartialOrd, Ord,
            PartialEq, Eq,
            Hash,
            ::indexland::Idx,
            ::indexland::EnumIdx
        )]
        #ast

        impl core::default::Default for #name {
            fn default() -> Self {
                ::indexland::Idx::ZERO
            }
        }
        impl std::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                ::indexland::Idx::from_usize(
                    ::indexland::Idx::into_usize(self) + ::indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl std::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                ::indexland::Idx::from_usize(
                    ::indexland::Idx::into_usize(self) - ::indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    };
    Ok(output)
    // Err(syn::Error::new(Span::call_site(), output.to_string()))
}

#[proc_macro_derive(EnumIdx)]
pub fn derive_enum_idx(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_enum_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_enum_idx_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports enums.",
            ))
        }
    };

    let mut idents = Vec::new();

    for variant in variants.iter().cloned() {
        idents.push(variant.ident);
    }

    let count = idents.len();

    let output = quote! {
        impl #impl_generics ::indexland::EnumIdx for #name #ty_generics #where_clause {
            const COUNT: usize = #count;
            type EnumIndexArray<T> = ::indexland::index_array::IndexArray<Self, T, #count>;
            const VARIANTS: &'static [Self] = &[ #(#name::#idents),* ];
        }
    };
    Ok(output)
}

#[proc_macro_derive(Idx)]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_idx_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports enums.",
            ))
        }
    };

    let mut idents = Vec::new();

    for variant in variants.iter().cloned() {
        let Fields::Unit = variant.fields else {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro does not support enum variants with payload.",
            ));
        };

        idents.push(variant.ident);
    }

    let count = idents.len();
    if count < 2 {
        return Err(syn::Error::new(
            Span::call_site(),
            "enum deriving EnumIdx must have at least two variants",
        ));
    }

    let var_zero = &idents[0];
    let var_one = &idents[1];
    let var_max = &idents[count - 1];

    let indices = 0..count;
    let indices_2 = 0..count;

    let output = quote! {
        impl #impl_generics ::indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name::#var_zero;
            const ONE: Self = #name::#var_one;
            const MAX: Self = #name::#var_max;
            fn from_usize(v: usize) -> Self {
                match v {
                    #(#indices => #name::#idents,)*
                    _ => panic!("enum index out of bounds"),
                }
            }
            fn into_usize(self) -> usize  {
                match self {
                    #(#name::#idents => #indices_2),*
                }
            }
        }
    };
    Ok(output)
}
