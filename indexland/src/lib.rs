#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// nostd
#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

// allow this crate to refer to itself as ::indexland for macros to work
// within tests, see https://github.com/rust-lang/rust/issues/54647
extern crate self as indexland;

pub mod idx;
pub mod index_array;
pub mod index_enumerate;
pub mod index_range;

pub mod index_slice;
pub mod index_slice_index;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod index_vec;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod index_vec_deque;

#[cfg(feature = "nonmax")]
#[cfg_attr(docsrs, doc(cfg(feature = "nonmax")))]
pub mod nonmax;

#[cfg(feature = "arrayvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "arrayvec")))]
pub mod index_array_vec;

#[cfg(feature = "smallvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
pub mod index_small_vec;

#[cfg(feature = "indexmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap")))]
pub mod index_hash_map;

#[cfg(feature = "indexmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap")))]
pub mod index_hash_set;

// convenience exports

#[doc(inline)]
pub use crate::{
    idx::{Idx, IdxEnum, IdxNewtype},
    index_range::IndexRange,
    index_range::IndexRangeBounds,
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
extern crate indexland_derive;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[doc(inline)]
pub use indexland_derive::{Idx, IdxEnum, IdxNewtype};

#[cfg(feature = "nonmax")]
#[doc(inline)]
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

// re-export the utility crates that we bundle
#[cfg(feature = "arrayvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "arrayvec")))]
pub use arrayvec;

#[cfg(feature = "smallvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
pub use smallvec;

#[cfg(feature = "indexmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap")))]
pub use indexmap;

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
    // reexport for vec!
    pub extern crate alloc;
}
