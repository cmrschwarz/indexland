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
/// # Implemented Traits
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
/// # Opt-in Extra Traits ([`#[indexland(extra(..))]`](Idx#indexlandextra))
/// - [`Add<usize>`](core::ops::Add),
/// - [`Sub<usize>`](core::ops::Sub)
/// - [`AddAssign<usize>`](core::ops::AddAssign)
/// - [`SubAssign<usize>`](core::ops::SubAssign)
/// - [`Display`](core::fmt::Display)
///
/// # Example
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
///   (no [`Display`](core::fmt::Display) by default, enable through [`#[indexland(extra(Display))]`](Idx#indexlandextra))
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
/// # Opt-in Extra Traits ([`#[indexland(extra(..))]`](Idx#indexlandextra))
/// - [`Add<usize>`](core::ops::Add),
/// - [`Sub<usize>`](core::ops::Sub)
/// - [`AddAssign<usize>`](core::ops::AddAssign)
/// - [`SubAssign<usize>`](core::ops::SubAssign)
/// - [`Display`](core::fmt::Display)
///
/// # Example
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
/// # Basic Usage
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
/// # Attributes
///
/// #### `#[indexland(crate = ..)]`
/// Change the crate name used within the derive macro.
/// The default value is `::indexland`.
///
/// #### `#[indexland(bounds_checks = "..")]`
/// Modify the default bounds checking behavior. There's currently three modes:
/// - "debug_only": The default. Enable bounds checks in debug mode, but not in
///   release builds.
/// - "always": Enable bounds checks regardless of build type.
/// - "never": Disable all bounds checks. Always silently wrap around.
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
/// # Attributes Examples
///
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx, Default)]
/// #[indexland(omit(Default, From<Self> for usize), extra(Display))]
/// enum Bar {
///     A,
///     B,
///     // using omit(Default) + derive(Default) allows us to change the default
///     // to an element other than the first.
///     #[default]
///     C,
/// };
///
/// println!("{}", Bar::A);
/// ```
///
/// ```
/// use indexland as foobar;
/// # // prevent rustfmt from reordering these two lines ...
/// use foobar::Idx;
///
/// #[derive(Idx)]
/// #[indexland(crate = foobar)] // serde style crate renaming
/// #[indexland(bounds_checks = "never")] // perf: make the `u32` implicitly wrap
/// struct Foo(u32);
/// ```
#[proc_macro_derive(Idx, attributes(indexland))]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
