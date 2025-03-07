#![doc = include_str!("../README.md")]

mod context;
mod enum_idx;
mod newtype_idx;

use enum_idx::derive_idx_enum_inner;
use newtype_idx::derive_idx_newtype_inner;
use proc_macro2::{Span, TokenStream};

use syn::{Data, DeriveInput};

/// Implements the following traits:
/// - [`IdxNewtype`] + [`Idx`]
/// - [`Default`]
/// - [`Debug`] + [`Display`](core::fmt::Display)
/// - [`Clone`] + [Copy]
/// - [`PartialOrd`] + [`Ord`]
/// - [`PartialEq`] + [`Eq`]
/// - [`Hash`]
/// - [`Add`](core::ops::Add) + [`AddAssign`](core::ops::AddAssign)
/// - [`Sub`](core::ops::Sub) + [`SubAssign`](core::ops::SubAssign)
/// - [`From<usize>`](core::convert::From) + [`From<Self> for usize`](core::convert::From)
///
/// ## Example
/// ```
/// use indexland::IdxNewtype;
///
/// #[derive(IdxNewtype)]
/// struct FooId(u32);
/// ```
#[proc_macro_derive(IdxNewtype)]
pub fn derive_idx_newtype(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_idx_newtype_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_idx_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
    match &ast.data {
        Data::Enum(_) => derive_idx_enum_inner(ast),
        Data::Struct(_) => derive_idx_newtype_inner(ast),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums and structs",
        )),
    }
}

/// Implements the following traits:
/// - [`IdxEnum`] + [`Idx`]
/// - [`Default`] (uses first variant)
/// - [`Debug`] + ( [`Display`](core::fmt::Display) intentionally omitted, implement as desired)
/// - [`Clone`] + [`Copy`]
/// - [`PartialOrd`] + [`Ord`]
/// - [`PartialEq`] + [`Eq`]
/// - [`Hash`]
/// - [`Add`](core::ops::Add) + [`AddAssign`](core::ops::AddAssign)
/// - [`Sub`](core::ops::Sub) + [`SubAssign`](core::ops::SubAssign)
/// - [`From<usize>`](core::convert::From) + [`From<Self> for usize`](core::convert::From)
///
///
/// ## Example
/// ```
/// use indexland::IdxEnum;
///
/// #[derive(IdxEnum)]
/// enum PrimaryColor {
///     Red,
///     Green,
///     Blue,
/// };
/// ```
#[proc_macro_derive(IdxEnum)]
pub fn derive_idx_enum(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_idx_enum_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// For structs this is equivalent to [`#[derive(IdxNewtype)]`](crate::IdxNewtype),
/// for enums to [`#[derive(IdxEnum)]`](crate::IdxEnum).
/// ## Example
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// struct NodeId(u32);
///
/// #[derive(Idx)]
/// enum PrimaryColor {
///     Red,
///     Green,
///     Blue,
/// };
/// ```
#[proc_macro_derive(Idx)]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
