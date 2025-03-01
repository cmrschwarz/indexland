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
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::module_name_repetitions)]

pub mod counted_stable_universe;
pub mod counted_universe;
pub mod enumerated_index_iter;
pub mod idx;
pub mod index_array;
pub mod index_slice;
pub mod index_vec;
pub mod nonmax;
pub mod offset_vec_deque;
pub mod phantom_slot;
pub mod random_access_container;
pub mod stable_universe;
pub mod stable_vec;
pub mod temp_vec;
pub mod universe;

#[cfg(feature = "arrayvec")]
pub mod index_array_vec;

#[cfg(feature = "smallvec")]
pub mod index_small_vec;

#[cfg(feature = "multi_ref_mut_handout")]
pub mod multi_ref_mut_handout;

#[cfg(feature = "multi_ref_mut_handout")]
pub mod universe_multi_ref_mut_handout;

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

use std::{
    mem::MaybeUninit,
    ops::{Range, RangeBounds},
};

pub enum GetManyMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}

// miscellaneous utility functions

pub fn get_two_distinct_mut<T>(
    slice: &mut [T],
    idx1: usize,
    idx2: usize,
) -> (&mut T, &mut T) {
    assert!(idx1 != idx2, "indices must be unique");
    assert!(
        idx1 < slice.len() && idx2 < slice.len(),
        "indices out of bounds"
    );
    unsafe {
        let ptr = slice.as_mut_ptr();
        (&mut *ptr.add(idx1), &mut *ptr.add(idx2))
    }
}

pub fn get_three_distinct_mut<T>(
    slice: &mut [T],
    idx1: usize,
    idx2: usize,
    idx3: usize,
) -> (&mut T, &mut T, &mut T) {
    assert!(idx1 != idx2 && idx2 != idx3, "indices must be unique");
    assert!(
        idx1 < slice.len() && idx2 < slice.len() && idx3 < slice.len(),
        "indices out of bounds"
    );
    unsafe {
        let ptr = slice.as_mut_ptr();
        (
            &mut *ptr.add(idx1),
            &mut *ptr.add(idx2),
            &mut *ptr.add(idx3),
        )
    }
}

pub fn subslice_slice_pair<'a, T>(
    slices: (&'a [T], &'a [T]),
    range: Range<usize>,
) -> (&'a [T], &'a [T]) {
    let (s1, s2) = slices;
    let s1_len = s1.len();
    if range.start > s1_len {
        (&[], &s2[range.start - s1_len..range.end - s1_len])
    } else if range.end <= s1_len {
        (&s1[range.start..range.end], &[])
    } else {
        (
            &s1[range.start..],
            &s2[..range.len() - (s1_len - range.start)],
        )
    }
}

pub fn subslice_slice_pair_mut<'a, T>(
    slices: (&'a mut [T], &'a mut [T]),
    range: Range<usize>,
) -> (&'a mut [T], &'a mut [T]) {
    let (s1, s2) = slices;
    let s1_len = s1.len();
    if range.start > s1_len {
        (&mut [], &mut s2[range.start - s1_len..range.end - s1_len])
    } else {
        (
            &mut s1[range.start..],
            &mut s2[..range.len() - (s1_len - range.start)],
        )
    }
}

pub fn range_bounds_to_range_wrapping<I: Idx>(
    rb: impl RangeBounds<I>,
    len: I,
) -> Range<I> {
    let start = match rb.start_bound() {
        std::ops::Bound::Included(i) => *i,
        std::ops::Bound::Excluded(i) => i.wrapping_add(I::ONE),
        std::ops::Bound::Unbounded => I::ZERO,
    };
    let end = match rb.end_bound() {
        std::ops::Bound::Included(i) => i.wrapping_add(I::ONE),
        std::ops::Bound::Excluded(i) => *i,
        std::ops::Bound::Unbounded => len,
    };
    start..end
}

pub fn range_bounds_to_range_usize<I: Idx>(
    rb: impl RangeBounds<I>,
    len: usize,
) -> Range<usize> {
    let start = match rb.start_bound() {
        std::ops::Bound::Included(i) => i.into_usize(),
        std::ops::Bound::Excluded(i) => i.into_usize() + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match rb.end_bound() {
        std::ops::Bound::Included(i) => i.into_usize() + 1,
        std::ops::Bound::Excluded(i) => i.into_usize(),
        std::ops::Bound::Unbounded => len,
    };
    start..end
}

pub fn range_contains<I: PartialOrd>(
    range: Range<I>,
    subrange: Range<I>,
) -> bool {
    range.start <= subrange.start && range.end >= subrange.end
}

/// [`std::mem::MaybeUninit::transpose`] implementation in stable Rust. Replace
/// once [maybe_uninit_uninit_array_transpose](https://github.com/rust-lang/rust/issues/96097)
/// is stabilized.
#[allow(clippy::needless_pass_by_value)]
pub const unsafe fn transpose_maybe_uninit<T, const N: usize>(
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
