#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]

mod context;
mod enum_idx;
mod newtype_idx;
mod utils;

use enum_idx::derive_idx_enum_inner;
use newtype_idx::derive_idx_newtype_inner;
use proc_macro2::{Span, TokenStream};

use syn::{Data, DeriveInput};

/// Derives `IdxNewtype` and associated traits.
/// See [`#[derive[Idx]]`](crate::Idx) the attributes explanation.
///
/// ## Implemented Traits
/// - [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
/// - [`indexland::IdxNewtype`](https://docs.rs/indexland/latest/indexland/trait.IdxNewtype.html)
/// - [`Default`](core::default::Default)
/// - [`Debug`](core::fmt::Debug) +
///   [`Display`](core::fmt::Display)
/// - [`Clone`](core::clone::Clone) +
///   [`Copy`](core::marker::Copy)
/// - [`PartialOrd`](core::cmp::PartialOrd) +
///   [`Ord`](core::cmp::Ord)
/// - [`PartialEq`](core::cmp::PartialEq) +
///   [`Eq`](core::cmp::Eq)
/// - [`Hash`](core::hash::Hash)
/// - [`Add`](core::ops::Add) +
///   [`AddAssign`](core::ops::AddAssign)
/// - [`Sub`](core::ops::Sub) +
///   [`SubAssign`](core::ops::SubAssign)
/// - [`From<usize>`](core::convert::From) +
///   [`From<Self> for usize`](core::convert::From)
///
/// ## Example
/// ```
/// use indexland::IdxNewtype;
///
/// #[derive(IdxNewtype)]
/// struct FooId(u32);
/// ```
#[proc_macro_derive(IdxNewtype, attributes(indexland))]
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

/// Derives `IdxEnum` and associated traits.
/// See [`#[derive[Idx]]`](crate::Idx) for the attributes explanation.
///
/// ## Implemented Traits
/// - [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
/// - [`indexland::IdxEnum`](https://docs.rs/indexland/latest/indexland/trait.IdxEnum.html)
/// - [`Default`](core::default::Default) (uses first variant)
/// - [`Debug`](core::fmt::Debug)
///   (enable [`Display`](core::fmt::Display) through `#[indexland(extra(Display))]`)
/// - [`Clone`](core::clone::Clone) +
///   [`Copy`](core::marker::Copy)
/// - [`PartialOrd`](core::clone::Clone) +
///   [`Ord`](core::cmp::Ord)
/// - [`PartialEq`](core::clone::Clone)
///   [`Eq`](core::cmp::Eq) +
/// - [`Hash`](core::hash::Hash)
/// - [`Add`](core::ops::Add) +
///   [`AddAssign`](core::ops::AddAssign)
/// - [`Sub`](core::ops::Sub) +
///   [`SubAssign`](core::ops::SubAssign)
/// - [`From<usize>`](core::convert::From) +
///   [`From<Self> for usize`](core::convert::From)
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
#[proc_macro_derive(IdxEnum, attributes(indexland))]
pub fn derive_idx_enum(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_idx_enum_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// For structs this is equivalent to [`#[derive(IdxNewtype)]`](crate::IdxNewtype),
/// for enums to [`#[derive(IdxEnum)]`](crate::IdxEnum).
///
/// ## Basic Usage
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
///
/// ## Attributes
///
/// #### `#[indexland(crate = ..)]`
/// Change the crate name used within the derive macro. The default value is `::indexland`.
///
/// #### `#[indexland(disable_bounds_checks)]`
/// Disable bounds checking on conversion from your `Idx` to and from `usize`
/// for increased performance. The index will wrap around instead.
/// This is meaningless for indices that wrap usize in the first place.
///
/// #### `#[indexland(usize_arith)]`
/// Implement [`Add<usize>`](core::ops::Add),
/// [`Sub<usize>`](core::ops::Sub), [`AddAssign<usize>`](core::ops::AddAssign),
/// and [`SubAssign<usize>`](core::ops::SubAssign).
///
/// #### `#[indexland(extra(..))]`
/// Enable the derivation of optional traits, see
/// [`#[derive(IdxNewtype)]`](crate::IdxNewtype),
/// and [`#[derive(IdxEnum)]`](crate::IdxEnum) for options.
///
/// #### `#[indexland(omit(..))]`
/// Suppress the derivation of certain traits (blacklist).
///
/// #### `#[indexland(only(..))]`
/// Suppress the derivation of all traits except the specified ones (whitelist).
///
/// ## Attributes Example
/// ```
/// use indexland as foobar;
///
/// use foobar::Idx;
///
/// #[derive(Idx)]
/// #[indexland(crate = foobar, disable_bounds_checks)]
/// #[indexland(omit(Debug, From<Self> for usize))]
/// struct Foo(u32);
///
/// #[derive(Idx)]
/// #[indexland(crate = foobar)]
/// #[indexland(extra(Display))]
/// enum Bar {
///     A,
///     B,
///     C,
/// };
///
/// println!("{}", Bar::A);
/// ```
#[proc_macro_derive(Idx, attributes(indexland))]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
