//! [![github]](https://github.com/cmrschwarz/typeline/tree/main/crates/indexland)&ensp;
//! [![github-build]](https://github.com/cmrschwarz/typeline/actions/workflows/ci.yml)&ensp;
//! [![crates-io]](https://crates.io/crates/indexland)&ensp;
//! [![msrv]](https://crates.io/crates/indexland)&ensp;
//! [![docs-rs]](https://docs.rs/indexland)&ensp;
//!
//! [github]: https://img.shields.io/badge/cmrschwarz/typeline-8da0cb?&labelColor=555555&logo=github
//! [github-build]: https://github.com/cmrschwarz/typeline/actions/workflows/ci.yml/badge.svg
//! [crates-io]: https://img.shields.io/crates/v/indexland.svg?logo=rust
//! [msrv]: https://img.shields.io/crates/msrv/indexland?logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-indexland-66c2a5?logo=docs.rs
//!
//! Wrappers for common collection types based on newtype indices.
//! Increased type safety and code readability without runtime overhead.
//!
//! Part of the [Typeline](https://github.com/cmrschwarz/typeline) project,
//! not ready for public use yet.
//!
//!
//! # Newtype Indices
//! ```rust
//! use indexland::{NewtypeIdx, IndexVec};
//!
//! #[derive(NewtypeIdx)]
//! struct NodeId(u32);
//!
//! struct Node<T> {
//!     prev: NodeId,
//!     next: NodeId,
//!     data: T,
//! };
//! struct DoublyLinkedList<T> {
//!     nodes: IndexVec<NodeId, Node<T>>,
//! }
//! ```
//!
//! # Enums as Indices
//! ```rust
//! use indexland::{EnumIdx, IndexArray, EnumIndexArray, index_array};
//!
//! #[derive(EnumIdx)]
//! enum PrimaryColor{
//!     Red,
//!     Green,
//!     Blue
//! };
//!
//! const COLOR_MAPPING: EnumIndexArray<PrimaryColor, u32> = index_array![
//!     PrimaryColor::Red => 0xFF0000,
//!     PrimaryColor::Green => 0x00FF00,
//!     PrimaryColor::Blue => 0x0000FF,
//! ];
//!
//! // index using enum variants
//! let my_color = COLOR_MAPPING[PrimaryColor::Red];
//!
//! // use convenience constants for iteration etc.
//! assert_eq!(PrimaryColor::COUNT, PrimaryColor::VARIANTS.len());
//! ```
//!
//! # Support for all common Array Based Collections
//! - `&IndexSlice<I, T>` wrapping `&[T]`
//! - `IndexArray<I, T, LEN>` wrapping `[T; LEN]`
//! - `IndexVec<I, T>` wrapping `Vec<T>`
//! - `IndexSmallVec<I, T, CAP>` wrapping [`SmallVec<[T; CAP]>`](https://docs.rs/smallvec/latest/smallvec)
//!   (Optional)
//! - `IndexArrayVec<I, T, CAP>` based on [`ArrayVec<T, CAP>`](https://docs.rs/arrayvec/latest/arrayvec/)
//!   (Optional)
//! - [Serde](https://docs.rs/serde/latest/serde/) support for all Collections

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::module_name_repetitions)]

pub mod enumerated_index_iter;
pub mod idx;
pub mod index_array;
pub mod index_slice;
pub mod index_vec;

#[cfg(feature = "arrayvec")]
pub mod index_array_vec;

pub mod get_many_mut;
#[cfg(feature = "smallvec")]
pub mod index_small_vec;

#[cfg(feature = "derive")]
pub use indexland_derive::{EnumIdx, Idx, NewtypeIdx};

pub use crate::idx::*;
pub use index_array::{EnumIndexArray, IndexArray};
pub use index_slice::IndexSlice;
pub use index_vec::IndexVec;

#[cfg(feature = "arrayvec")]
pub use index_array_vec::IndexArrayVec;

#[cfg(feature = "smallvec")]
pub use index_small_vec::IndexSmallVec;

// used in macros, not public api
#[doc(hidden)]
pub mod __private {
    use std::mem::MaybeUninit;

    /// Essentially [`std::mem::MaybeUninit::transpose`] in stable Rust. Will
    /// be removed once [maybe_uninit_uninit_array_transpose](https://github.com/rust-lang/rust/issues/96097)
    /// is stabilized.
    #[allow(clippy::needless_pass_by_value)]
    pub const unsafe fn transpose_assume_uninit<T, const N: usize>(
        v: [MaybeUninit<T>; N],
    ) -> [T; N] {
        let mut res = MaybeUninit::<[T; N]>::uninit();
        let mut i = 0;
        while i < v.len() {
            unsafe {
                res.as_mut_ptr()
                    .cast::<T>()
                    .add(i)
                    .write(v.as_ptr().add(i).read().assume_init());
            };
            i += 1;
        }
        unsafe { res.assume_init() }
    }
}
