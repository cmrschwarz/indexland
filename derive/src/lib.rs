#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]

mod attrs;
mod derive_context;
mod enum_idx;
mod newtype_idx;
mod shared_derives;
mod utils;

use enum_idx::derive_idx_enum_inner;
use newtype_idx::derive_idx_newtype_inner;
use proc_macro2::{Span, TokenStream};

use syn::{Data, DeriveInput};

fn derive_idx_inner(ast: DeriveInput, rich_defaults: bool) -> Result<TokenStream, syn::Error> {
    match &ast.data {
        Data::Enum(_) => derive_idx_enum_inner(ast, rich_defaults),
        Data::Struct(_) => derive_idx_newtype_inner(ast, rich_defaults),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums and structs",
        )),
    }
}

/// Derives
/// [`indexland::IdxNewtype`](https://docs.rs/indexland/latest/indexland/trait.IdxNewtype.html)
/// and [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
///
/// This macro supports the same attributes as [`#[derive[Idx]]`](crate::Idx).
///
/// In order for `Idx` to be satisfied, the type must implement `Copy` and `Ord`.
///
/// It is generally preferred to use [`#[derive[Idx]]`](crate::Idx) which derives all required
/// traits for you (aswell as some additional ones for convenience).
/// See it's documentation for details.
///
/// # Example
/// ```
/// use indexland::IdxNewtype;
///
/// #[derive(IdxNewtype, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// struct FooId(u32);
/// ```
#[proc_macro_derive(IdxNewtype, attributes(indexland))]
pub fn derive_idx_newtype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_newtype_inner(syn::parse_macro_input!(input as DeriveInput), false)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derives
/// [`indexland::IdxEnum`](https://docs.rs/indexland/latest/indexland/trait.IdxEnum.html)
/// and [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
///
/// This macro supports the same attributes as [`#[derive[Idx]]`](crate::Idx).
///
/// In order for `Idx` to be satisfied, the type must implement `Copy` and `Ord`.
///
/// It is generally preferred to use [`#[derive[Idx]]`](crate::Idx), which derives all required
/// traits for you (aswell as some additional ones for convenience).
/// See it's documentation for details.
///
/// # Example
/// ```
/// use indexland::IdxEnum;
///
/// #[derive(IdxEnum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// enum PrimaryColor {
///     Red,
///     Green,
///     Blue,
/// };
/// ```
#[proc_macro_derive(IdxEnum, attributes(indexland))]
pub fn derive_idx_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_enum_inner(syn::parse_macro_input!(input as DeriveInput), false)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derives
/// [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
/// aswell as the required super traits and operator overloads (see the full trait list below).
///
/// If you want to manually derive supertraits, use [`indexland::IdxNewtype`](https://docs.rs/indexland/latest/indexland/trait.IdxNewtype.html)
/// or [`indexland::IdxEnum`](https://docs.rs/indexland/latest/indexland/trait.IdxEnum.html) directly,
/// or customize the derived traits using [`#[indexland(omit(..))]`](Idx#indexlandomit)
/// and [`#[indexland(only(..))]`](Idx#indexlandonly)
///
///
/// # Implemented Traits
/// - [`indexland::Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
/// - [`indexland::IdxNewtype`](https://docs.rs/indexland/latest/indexland/trait.IdxNewtype.html) (structs)
///   / [`indexland::IdxEnum`](https://docs.rs/indexland/latest/indexland/trait.IdxEnum.html) (enums)
/// - [`Default`](core::default::Default)
/// - [`Debug`](core::fmt::Debug)
/// - [`Display`](core::fmt::Display) (for structs, opt-in for enums)
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
/// - [`Rem<usize>`](core::ops::Rem) +
///   [`RemAssign<usize>`](core::ops::RemAssign)
/// - [`From<usize>`](core::convert::From) +
///   [`From<Self> for usize`](core::convert::From)
///
/// # Opt-in Extra Traits ([`#[indexland(extra(..))]`](Idx#indexlandextra))
/// - [`Add<usize>`](core::ops::Add),
/// - [`Sub<usize>`](core::ops::Sub)
/// - [`Rem<usize>`](core::ops::Rem)
/// - [`AddAssign<usize>`](core::ops::AddAssign)
/// - [`SubAssign<usize>`](core::ops::SubAssign)
/// - [`RemAssign<usize>`](core::ops::RemAssign)
/// - [`Display`](core::fmt::Display) (for enums, structs have this enabled by default)
///
/// # Example
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
///
/// ## `#[indexland(bounds_checks = "..")]`
/// Modify the default bounds checking behavior. There's currently three modes:
/// - `"debug_only"`: The default. Enable bounds checks in debug mode, but not in
///   release builds.
/// - `"always"`: Enable bounds checks regardless of build type.
/// - `"never"`: Disable all bounds checks. Always silently wrap around.
///
/// ### Example
/// ```should_panic
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// #[indexland(bounds_checks = "always")]
/// struct SafeId(u8);
///
/// // !!! These will now panic even in release builds, instead of wrapping around.
/// let _ = SafeId::MAX + SafeId(1);
/// let _ = SafeId::from(usize::MAX);
/// ```
///
/// ## `#[indexland(arith = "..")]`
/// - `"basic"`: The default. Implement
///   [`Add`](core::ops::Add) + [`AddAssign`](core::ops::AddAssign),
///   [`Sub<T>`](core::ops::Sub) + [`SubAssign<T>`](core::ops::SubAssign), and
///   [`Rem`](core::ops::Rem) + [`RemAssign`](core::ops::RemAssign)
///
/// - `"full"`: Implement [`Mul`](core::ops::Mul) + [`MulAssign`](core::ops::MulAssign) and
///   [`Div`](core::ops::Div) + [`DivAssign`](core::ops::DivAssign) in addition to
///   the traits derived by "basic".
///
/// - `"disabled"`: Don't derive any arithmetic traits.
///
/// If [`#[indexland(arith_compat(T))]`](Idx#indexlandarith_compatt) is specified,
/// also implements the respective versions for `Rhs = T`.
///
/// ### Example
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// #[indexland(arith = "full")]
/// struct ArithId(u32);
///
/// // multiplication and division are now enabled
/// assert_eq!(ArithId(4) / ArithId(2), ArithId(2));
/// assert_eq!(ArithId(2) * ArithId(3), ArithId(6));
/// ```
///
/// ## `#[indexland(arith_compat(T))]`
/// Implement [`Add<T>`](core::ops::Add) + [`AddAssign<T>`](core::ops::AddAssign),
/// and [`Sub<T>`](core::ops::Sub) + [`SubAssign<T>`](core::ops::SubAssign).
///
/// If [`#[indexland(arith = "full")]`](Idx#indexlandarith) is specified, also implements
/// [`Mul`](core::ops::Mul) + [`MulAssign`](core::ops::MulAssign),
/// [`Div`](core::ops::Div) + [`DivAssign`](core::ops::DivAssign),
/// and [`Rem`](core::ops::Rem) + [`RemAssign`](core::ops::RemAssign).
///
/// The primary usecase is `#[indexland(arith_compat(usize))]`.
///
/// ### Example
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// #[indexland(arith_compat(usize))]
/// struct FuzzyId(u32);
///
/// let id = FuzzyId(5);
/// // Can now use arithmetic with usize directly
/// assert_eq!(id + 3, FuzzyId(8));
/// ```
///
/// ## `#[indexland(extra(..))]`
/// Enable the derivation of optional traits, see
/// [`#[derive(IdxNewtype)]`](crate::IdxNewtype),
/// and [`#[derive(IdxEnum)]`](crate::IdxEnum) for options.
///
/// ### Example
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// #[indexland(extra(Display))]
/// enum Status {
///     Active,
///     Inactive,
/// }
///
/// // The `Display` trait is now available for enums
/// assert_eq!(format!("{}", Status::Active), "Active");
/// ```
///
/// ## `#[indexland(omit(..))]`
/// Suppress the derivation of certain traits (blacklist).
///
/// ### Example
/// ```compile_fail
/// use indexland::Idx;
///
/// #[derive(Idx)]
/// #[indexland(omit(Display))]
/// struct SecretId(u32);
///
/// // Add a custom `Display` impl or don't derive it at all
/// impl Display for SecretId{
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "SecretId(<redacted>)", self.0)
///     }
/// }
/// ```
///
/// ## `#[indexland(only(..))]`
/// Suppress the derivation of all traits except the specified ones (whitelist).
///
/// ### Example
/// ```
/// use indexland::Idx;
///
/// #[derive(Idx, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// #[indexland(only(Idx))]
/// struct MinimalId(u32); // Minimal set of traits.
/// ```
///
/// ## `#[indexland(idx_compat(..))]`
/// Allow other types to be used to index containers of this type.
///
/// ### Example
/// ```
/// use indexland::{Idx, IndexVec};
///
/// #[derive(Idx)]
/// #[indexland(idx_compat(usize))]
/// struct ItemId(usize);
///
/// // Can use usize directly to index containers
/// let items = IndexVec::<ItemId, u8>::from_iter(0..10);
/// let item_0 = items[0]; // Allowed due to `idx_compat(usize)`.
/// let item_1 = items[ItemId(1)]; // Still works.
/// ```
///
/// ### `#[indexland(crate = ..)]`
/// Change the crate name used within the derive macro,
/// similar to the way [serde does this](https://serde.rs/container-attrs.html#crate).
/// Only needed if you renamed the indexland crate in your Cargo.toml,
/// or are using it through a re-export.
///
/// ### Example
/// ```
/// use indexland as foobar;
///
/// #[derive(foobar::Idx)]
/// #[indexland(crate = foobar)] // the generated code will use `foobar::` instead of `indexland::`
/// struct Foo(u32);
/// ```
#[proc_macro_derive(Idx, attributes(indexland))]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput), true)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
