#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]

mod context;
mod enum_idx;
mod newtype_idx;

use std::fmt::Write;

use enum_idx::derive_idx_enum_inner;
use newtype_idx::derive_idx_newtype_inner;
use proc_macro2::{Span, TokenStream};

use quote::ToTokens;
use syn::{Data, DeriveInput};

// to get "From<usize>" instead of "From < usize >"
// this does not solve the general problem of pretty printing and will
// produce ugly results for some expressions but it is good enough
// for our usecases so I won't add a dependency on prettyplease
fn compact_token_stream_stringify(res: &mut String, ts: TokenStream) {
    #[derive(PartialEq, Eq)]
    enum Prev {
        Comma,
        ClosingAngleBracket,
        IdentOrLiteral,
        Other,
    }
    let mut prev = Prev::Other;
    for t in ts.to_token_stream() {
        match t {
            proc_macro2::TokenTree::Ident(ident) => {
                if matches!(
                    prev,
                    Prev::Comma
                        | Prev::IdentOrLiteral
                        | Prev::ClosingAngleBracket
                ) {
                    res.push(' ');
                }
                res.write_fmt(format_args!("{ident}")).unwrap();
                prev = Prev::IdentOrLiteral;
            }
            proc_macro2::TokenTree::Punct(punct) => {
                let p = punct.as_char();
                res.push(p);
                prev = match p {
                    ',' => Prev::Comma,
                    '>' => Prev::ClosingAngleBracket,
                    _ => Prev::Other,
                };
            }
            proc_macro2::TokenTree::Group(group) => {
                match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => {
                        res.push('(');
                        compact_token_stream_stringify(res, group.stream());
                        res.push(')');
                    }
                    proc_macro2::Delimiter::Brace => {
                        res.push('{');
                        compact_token_stream_stringify(res, group.stream());
                        res.push('}');
                    }
                    proc_macro2::Delimiter::Bracket => {
                        res.push('[');
                        compact_token_stream_stringify(res, group.stream());
                        res.push(']');
                    }
                    proc_macro2::Delimiter::None => {
                        compact_token_stream_stringify(res, group.stream())
                    }
                }
                prev = Prev::Other;
            }
            proc_macro2::TokenTree::Literal(literal) => {
                if prev == Prev::Comma || prev == Prev::IdentOrLiteral {
                    res.push(' ');
                }
                res.write_fmt(format_args!("{literal}")).unwrap();
                prev = Prev::IdentOrLiteral;
            }
        }
    }
}

fn token_stream_to_compact_string(path: &TokenStream) -> String {
    let mut res = String::new();
    compact_token_stream_stringify(&mut res, path.clone());
    res
}

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
///   (intentionally not [`Display`](core::fmt::Display), implement as desired)
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
/// - `#[indexland(crate = ..)]`: Change the crate name used within the derive
///    macro. The default value is `::indexland`.
/// - `#[indexland(omit(..))]`: Suppress the derivation of certain traits (blacklist).
/// - `#[indexland(only(..))]`: Suppress the derivation of all traits except the specified ones (whitelist).
///
/// ## Attributes Example
/// ```
/// use indexland as foobar;
///
/// use foobar::Idx;
///
/// #[derive(Idx)]
/// #[indexland(crate = foobar)]
/// #[indexland(omit(Debug, From<Self> for usize))]
/// enum Foo {
///     A,
///     B,
///     C,
/// }
/// ```
#[proc_macro_derive(Idx, attributes(indexland))]
pub fn derive_idx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_idx_inner(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
