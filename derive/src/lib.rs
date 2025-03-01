//! Provides derive macros for `indexland`. For better ergonomics add the
//! `"derive"` feature to `indexland` instead of depending on this directly.
//! ```rust
//! use indexland::{NewtypeIdx, index_vec::IndexVec};
//! #[derive(NewtypeIdx)]
//! struct FooId(u32);
//! struct Foo{ /*...*/ };
//! struct FooContainer {
//!     foos: IndexVec<FooId, Foo>,
//! }
//!
//! use indexland::{EnumIdx, index_array::{IndexArray, EnumIndexArray}};
//! #[derive(EnumIdx)]
//! enum Bar{
//!     A,
//!     B,
//!     C
//! };
//! let BAR_MAPPING: EnumIndexArray<Bar, i32> = IndexArray::new([1, 2, 3]);
//! ```

// TODO: add macro to supporess specific implementations to be able to
// customize them for EnumIdx and NewtypeIdx. Then also add default Display
// implementation to EnumIdx.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields};

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

fn derive_idx_for_enum(
    ast: &DeriveInput,
    enum_data: &DataEnum,
) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let mut idents = Vec::new();

    for variant in &enum_data.variants {
        let Fields::Unit = variant.fields else {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro does not support enum variants with payload.",
            ));
        };

        idents.push(&variant.ident);
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

fn derive_idx_for_struct(
    ast: &DeriveInput,
    struct_data: &DataStruct,
) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let inner = get_single_struct_member(struct_data)?;

    let base_type = &inner.ty;

    let output = quote! {
        impl #impl_generics ::indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name(<#base_type as ::indexland::Idx>::ZERO);
            const ONE: Self = #name(<#base_type as ::indexland::Idx>::ONE);
            const MAX: Self = #name(<#base_type as ::indexland::Idx>::MAX);
            #[inline(always)]
            fn into_usize(self) -> usize {
                <#base_type as ::indexland::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                #name(<#base_type as ::indexland::Idx>::from_usize(v))
            }
            fn wrapping_add(self, other: Self) -> Self {
                #name(<#base_type as ::indexland::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                #name(<#base_type as ::indexland::Idx>::wrapping_sub(self.0, other.0))
            }
        }
    };
    Ok(output)
}

fn derive_idx_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    match &ast.data {
        Data::Enum(enum_data) => derive_idx_for_enum(&ast, enum_data),
        Data::Struct(struct_data) => derive_idx_for_struct(&ast, struct_data),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums and structs",
        )),
    }
}

/// Only derives the `Idx` trait, not it's requireed super traits.
/// For more oppinionionated defaults use
/// `#[derive(NewtypeIdx)]` or `#[derive(EnumIdx)]` instead.
#[proc_macro_derive(Idx)]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_idx_newtype_inner(
    ast: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let Data::Struct(struct_data) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports newtype structs",
        ));
    };

    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let inner = get_single_struct_member(struct_data)?;

    let base_type = &inner.ty;

    let name = &ast.ident;

    let idx_derivation = derive_idx_for_struct(&ast, struct_data)?;

    let output = quote! {
        impl core::default::Default for #name {
            fn default() -> Self {
                ::indexland::Idx::ZERO
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl Clone for #name {
            fn clone(&self) -> Self {
                #name(self.0)
            }
        }
        impl Copy for #name {}
        impl core::hash::Hash for #name {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
        impl From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #name(<#base_type as ::indexland::Idx>::from_usize(v))
            }
        }
        impl From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                <#base_type as ::indexland::Idx>::into_usize(v.0)
            }
        }
        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #name(self.0 + rhs.0)
            }
        }
        impl core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #name(self.0 - rhs.0)
            }
        }
        impl core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        impl core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        impl core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl core::cmp::Eq for #name {}
        impl #impl_generics ::indexland::NewtypeIdx for #name #ty_generics #where_clause {
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
    // Err(syn::Error::new(Span::call_site(), output.to_string()))
}

/// Implements the following traits:
/// - `NewtypeIdx` + `Idx`
/// - `Default`
/// - `Debug` + `Display`
/// - `Clone` + `Copy`
/// - `Hash`
/// - `PartialOrd` + `Ord`
/// - `PartialEq` + `Eq`
/// - `Add` + `AddAssign`
/// - `Sub` + `SubAssign`
/// - `From<usize>` + `From<Self> for usize`
#[proc_macro_derive(NewtypeIdx)]
pub fn derive_idx_newtype(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_idx_newtype_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_idx_enum_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let Data::Enum(enum_data) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums.",
        ));
    };

    let mut idents = Vec::new();
    let mut ident_strings = Vec::new();

    for variant in &enum_data.variants {
        idents.push(&variant.ident);
        ident_strings.push(variant.ident.to_string());
    }

    let count = idents.len();

    let idx_derivation = derive_idx_for_enum(&ast, enum_data)?;

    let output = quote! {
        impl core::default::Default for #name {
            fn default() -> Self {
                ::indexland::Idx::ZERO
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl Clone for #name {
            fn clone(&self) -> Self {
                ::indexland::Idx::from_usize(::indexland::Idx::into_usize(*self))
            }
        }
        impl Copy for #name {}
        impl core::hash::Hash for #name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#name::#idents => f.write_str(#ident_strings)),*
                }
            }
        }
        impl core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                ::indexland::Idx::from_usize(
                    ::indexland::Idx::into_usize(self) + ::indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                ::indexland::Idx::from_usize(
                    ::indexland::Idx::into_usize(self) - ::indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                ::indexland::Idx::into_usize(*self)
                    .partial_cmp(&::indexland::Idx::into_usize(*other))
            }
        }
        impl core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                ::indexland::Idx::into_usize(*self)
                    .cmp(&::indexland::Idx::into_usize(*other))
            }
        }
        impl core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
        impl core::cmp::Eq for #name {}
        impl #impl_generics ::indexland::EnumIdx for #name #ty_generics #where_clause {
            const COUNT: usize = #count;
            type EnumIndexArray<T> = ::indexland::index_array::IndexArray<Self, T, #count>;
            const VARIANTS: &'static [Self] = &[ #(#name::#idents),* ];
        }
        #idx_derivation
    };
    Ok(output)
    // Err(syn::Error::new(Span::call_site(), output.to_string()))
}

/// Implements the following traits:
/// - `EnumIdx` + `Idx`
/// - `Default` (uses first variant)
/// - `Debug` + (`Display` intentionally omitted, implement as desired)
/// - `Clone + Copy`
/// - `PartialOrd + Ord`
/// - `PartialEq + Eq`
/// - `Hash`
/// - `Add + AddAssign`
/// - `Sub + SubAssign`
/// - `From<usize>` + `From<Self> for usize`
#[proc_macro_derive(EnumIdx)]
pub fn derive_idx_enum(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_idx_enum_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
