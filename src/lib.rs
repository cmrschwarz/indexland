#![cfg_attr(
    any(feature="derive", not(doctest)),
    doc = include_str!("../README.md")
)]
#![warn(clippy::pedantic)]
#![warn(unused_results)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::inline_always)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::type_complexity)]
#![allow(clippy::return_self_not_must_use)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// nostd
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
pub extern crate alloc;

// allow this crate to refer to itself as ::indexland for macros to work
// within tests, see https://github.com/rust-lang/rust/issues/54647
extern crate self as indexland;

pub mod idx;

pub mod index_range;

pub mod index_enumerate;

pub mod identity_hasher;
pub mod index_slice;
pub mod index_slice_index;
pub mod raw_index_container;

pub mod index_array;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod index_vec;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod index_vec_deque;

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

#[cfg(feature = "slab")]
#[cfg_attr(docsrs, doc(cfg(feature = "slab")))]
pub mod index_slab;

#[cfg(feature = "nonmax")]
#[cfg_attr(docsrs, doc(cfg(feature = "nonmax")))]
pub mod nonmax;

// convenience exports

// traits
#[doc(inline)]
pub use crate::{
    idx::{ArithCompat, Idx, IdxCompat, IdxEnum, IdxNewtype},
    index_range::IndexRangeBounds,
};

// structs
#[doc(inline)]
pub use crate::index_range::{IndexRange, IndexRangeFrom, IndexRangeInclusive};

#[doc(inline)]
pub use index_slice::IndexSlice;

#[doc(inline)]
pub use index_array::IndexArray;

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

pub use identity_hasher::IdentityHasher;

// type aliases
#[doc(inline)]
pub use index_array::EnumIndexArray;

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
    use core::mem::{ManuallyDrop, MaybeUninit};

    use crate::{Idx, IndexArray};

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

    const fn usize_to_ascii(mut n: usize, buf: &mut [u8]) -> usize {
        let digits = if n == 0 { 1 } else { n.ilog10() as usize + 1 };
        let mut i = digits;
        #[allow(clippy::cast_possible_truncation)]
        while i > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        digits
    }

    #[track_caller]
    const fn panic_index_initialized_twice(index: usize) {
        unsafe {
            const MSG_PART_1: &str = "index `";
            const MSG_PART_2: &str = "` was initialized twice";
            const MSG_BUF_LEN: usize = 100;
            const _: () = assert!(
                MSG_PART_1.len() + (usize::MAX.ilog10() as usize) + MSG_PART_2.len() < MSG_BUF_LEN
            );

            let mut msg_buf = [0u8; MSG_BUF_LEN];

            core::ptr::copy_nonoverlapping(
                MSG_PART_1.as_ptr(),
                msg_buf.as_mut_ptr(),
                MSG_PART_1.len(),
            );

            let digits = usize_to_ascii(
                index,
                core::slice::from_raw_parts_mut(
                    msg_buf.as_mut_ptr().add(MSG_PART_1.len()),
                    MSG_BUF_LEN - MSG_PART_1.len(),
                ),
            );

            core::ptr::copy_nonoverlapping(
                MSG_PART_2.as_ptr(),
                msg_buf.as_mut_ptr().add(MSG_PART_1.len() + digits),
                MSG_PART_2.len(),
            );

            panic!(
                "{}",
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    msg_buf.as_ptr(),
                    MSG_PART_1.len() + digits + MSG_PART_2.len()
                ))
            );
        }
    }

    #[track_caller]
    pub const fn array_from_values_and_distinct_indices<T, const N: usize>(
        indices: [usize; N],
        values: ManuallyDrop<[T; N]>,
    ) -> [T; N] {
        let values = ManuallyDrop::into_inner(values);
        let mut data: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let mut initialized = [false; N];
        let mut i = 0;
        while i < N {
            let target_index = indices[i];
            if initialized[target_index] {
                panic_index_initialized_twice(target_index);
            }
            initialized[target_index] = true;
            // SAFETY: the pointer comes from an array
            data[target_index] = MaybeUninit::new(unsafe { core::ptr::read(&raw const values[i]) });
            i += 1;
        }
        // SAFETY: we just successfully initialized `N` distinct slots in an
        // array of `N` elements so we must have initialized all slots
        core::mem::forget(values); // this is empty now
        unsafe { transpose_assume_uninit(data) }
    }

    // NOTE: this is unfortunately not const because `Idx::into_usize` is
    // a trait method :(.
    #[track_caller]
    pub fn index_array_from_values_and_distinct_indices<I, T, const N: usize>(
        indices: [I; N],
        values: ManuallyDrop<[T; N]>,
    ) -> IndexArray<I, T, N>
    where
        I: Idx,
    {
        let values = ManuallyDrop::into_inner(values);
        let mut data: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let mut initialized = [false; N];
        let mut i = 0;
        while i < N {
            let target_index = indices[i].into_usize();

            assert!(
                !initialized[target_index],
                "index `{target_index}` was initialized twice"
            );

            initialized[target_index] = true;

            // SAFETY: the pointer comes from an array
            data[indices[i].into_usize()] =
                MaybeUninit::new(unsafe { core::ptr::read(&raw const values[i]) });
            i += 1;
        }
        // SAFETY: we just successfully initialized `N` distinct slots in an
        // array of `N` elements so we must have initialized all slots
        core::mem::forget(values); // this is empty now
        IndexArray::from(unsafe { transpose_assume_uninit(data) })
    }

    #[cfg(test)]
    mod test {
        use super::panic_index_initialized_twice;

        #[test]
        #[should_panic(expected = "index `0` was initialized twice")]
        fn index_initialized_twice_0() {
            panic_index_initialized_twice(0);
        }

        #[test]
        #[should_panic(expected = "index `1` was initialized twice")]
        fn index_initialized_twice_1() {
            panic_index_initialized_twice(1);
        }

        #[test]
        #[should_panic(expected = "index `9` was initialized twice")]
        fn index_initialized_twice_9() {
            panic_index_initialized_twice(9);
        }

        #[test]
        #[should_panic(expected = "index `10` was initialized twice")]
        fn index_initialized_twice_10() {
            panic_index_initialized_twice(10);
        }

        #[test]
        #[should_panic(expected = "index `11` was initialized twice")]
        fn index_initialized_twice_11() {
            panic_index_initialized_twice(11);
        }

        #[cfg(target_pointer_width = "64")]
        #[test]
        #[should_panic(expected = "index `18446744073709551615` was initialized twice")]
        fn index_initialized_twice_max() {
            panic_index_initialized_twice(18_446_744_073_709_551_615);
        }

        #[cfg(target_pointer_width = "32")]
        #[test]
        #[should_panic(expected = "index `4294967295` was initialized twice")]
        fn index_initialized_twice_max() {
            panic_index_initialized_twice(4_294_967_295);
        }

        #[cfg(target_pointer_width = "16")]
        #[test]
        #[should_panic(expected = "index `65535` was initialized twice")]
        fn index_initialized_twice_max() {
            panic_index_initialized_twice(65_535);
        }
    }
}
