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
//! use indexland::{IdxNewtype, IndexVec};
//!
//! #[derive(IdxNewtype)]
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
//! use indexland::{IdxEnum, EnumIndexArray, enum_index_array};
//!
//! #[derive(IdxEnum)]
//! enum PrimaryColor {
//!     Red,
//!     Green,
//!     Blue
//! };
//!
//! const COLOR_MAPPING: EnumIndexArray<PrimaryColor, u32> = enum_index_array![
//!     PrimaryColor::Red => 0xFF0000,
//!     PrimaryColor::Green => 0x00FF00,
//!     PrimaryColor::Blue => 0x0000FF,
//! ];
//!
//! // index using enum variants
//! let my_color = COLOR_MAPPING[PrimaryColor::Red];
//! ```
//!
//! # Support for most common Array Based Collections
//! - [`IndexSlice<I, T>`](crate::IndexSlice)
//!   wrapping [`&[T]`](std::slice)
//! - [`IndexArray<I, T, LEN>`](crate::IndexArray)
//!   wrapping [`[T; LEN]`](std::array)
//! - [`IndexVec<I, T>`](crate::IndexVec)
//!   wrapping [`Vec<T>`]
//! - [`IndexVecDeque<I, T>`](crate::IndexVecDeque)
//!   wrapping[`VecDeque<T>`](std::collections::VecDeque)
//! - [`IndexSmallVec<I, T, CAP>`]
//!   wrapping [`SmallVec<[T;CAP]>`](smallvec::SmallVec) (Optional)
//! - [`IndexArrayVec<I, T, CAP>`](crate::IndexArrayVec)
//!   wrapping [`ArrayVec<T, CAP>`](arrayvec::ArrayVec) (Optional)
//! - [`IndexHashMap<I, K, V>`](crate::IndexHashMap)
//!   wrapping [`IndexMap<K, V>`](indexmap::IndexMap) (Optional)
//! - [`IndexHashSet<I, T>`](crate::IndexHashSet)
//!   wrapping [`IndexSet<T>`](indexmap::IndexSet) (Optional)
//! - [`NonMax<T>`](crate::nonmax) Integer Types for Niche Optimizations (Optional)
//! - [`serde`] support for all Collections (Optional)

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::module_name_repetitions)]
// nostd
#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod idx;
pub mod idx_enumerate;
pub mod idx_range;
pub mod index_array;

pub mod index_slice;

#[cfg(feature = "alloc")]
pub mod index_vec;

#[cfg(feature = "alloc")]
pub mod index_vec_deque;

#[cfg(feature = "nonmax")]
pub mod nonmax;

#[cfg(feature = "arrayvec")]
pub mod index_array_vec;

#[cfg(feature = "smallvec")]
pub mod index_small_vec;

#[cfg(feature = "indexmap")]
pub mod index_hash_map;

#[cfg(feature = "indexmap")]
pub mod index_hash_set;

// convenience exports

#[doc(inline)]
pub use crate::{
    idx::{Idx, IdxEnum, IdxNewtype},
    idx_range::IdxRange,
    idx_range::IdxRangeInclusive,
};

#[doc(inline)]
pub use index_array::{EnumIndexArray, IndexArray};

#[doc(inline)]
pub use index_slice::IndexSlice;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use index_vec::IndexVec;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use index_vec_deque::IndexVecDeque;

#[cfg(feature = "derive")]
#[doc(inline)]
pub use indexland_derive::{Idx, IdxEnum, IdxNewtype};

#[cfg(feature = "nonmax")]
pub use nonmax::NonMax;

#[cfg(feature = "arrayvec")]
#[doc(inline)]
pub use index_array_vec::IndexArrayVec;

#[cfg(feature = "smallvec")]
#[doc(inline)]
pub use index_small_vec::IndexSmallVec;

#[cfg(feature = "indexmap")]
#[doc(inline)]
pub use {index_hash_map::IndexHashMap, index_hash_set::IndexHashSet};

// used in macros, not public api
#[doc(hidden)]
pub mod __private {
    use core::mem::MaybeUninit;

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
