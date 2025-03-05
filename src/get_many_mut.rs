/// Mirror of the unstable `std::slice::get_many_mut` API for `IndexSlice`
use core::ops::{Range, RangeInclusive};

use crate::{Idx, IndexSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetManyMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}

/// # Safety
/// If `is_in_bounds()` returns `true` it must be safe to index the slice with
/// the indices.
/// If `is_overlapping()` returns `false` for two (in bounds) indices it must
/// be safe to access a slice mutably at both indices the same time.
/// !! These validations must hold *after* the
/// `into_usize` conversion of the `Idx`, even if that conversion has changed
/// the value / ordering.
pub unsafe trait GetManyMutIndex<I>: Clone {
    fn is_in_bounds(&self, len: I) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}

unsafe impl<I: Idx> GetManyMutIndex<I> for I {
    #[inline]
    fn is_in_bounds(&self, len: I) -> bool {
        self.into_usize() < len.into_usize()
    }

    #[inline]
    fn is_overlapping(&self, other: &Self) -> bool {
        self.into_usize() == other.into_usize()
    }
}

unsafe impl<I: Idx> GetManyMutIndex<I> for Range<I> {
    #[inline]
    fn is_in_bounds(&self, len: I) -> bool {
        (self.start.into_usize() <= self.end.into_usize())
            & (self.end.into_usize() <= len.into_usize())
    }

    #[inline]
    fn is_overlapping(&self, other: &Self) -> bool {
        (self.start.into_usize() < other.end.into_usize())
            & (other.start.into_usize() < self.end.into_usize())
    }
}

unsafe impl<I: Idx> GetManyMutIndex<I> for RangeInclusive<I> {
    #[inline]
    fn is_in_bounds(&self, len: I) -> bool {
        (self.start().into_usize() <= self.end().into_usize())
            & (self.end().into_usize() < len.into_usize())
    }

    #[inline]
    fn is_overlapping(&self, other: &Self) -> bool {
        (self.start() <= other.end()) & (other.start() <= self.end())
    }
}

impl<I: Idx, T> IndexSlice<I, T> {
    /// # Safety
    /// Calling this method with overlapping keys is undefined behavior
    /// even if the resulting references are not used.
    pub unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [I; N],
    ) -> [&mut T; N] {
        let slice: *mut T = self.as_slice_mut().as_mut_ptr();
        let mut arr: std::mem::MaybeUninit<[&mut T; N]> =
            std::mem::MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `indices` to be disjunct and in bounds
        unsafe {
            for i in 0..N {
                let idx = indices.get_unchecked(i).into_usize();
                arr_ptr.cast::<&mut T>().add(i).write(&mut *slice.add(idx));
            }
            arr.assume_init()
        }
    }

    pub fn get_many_mut<const N: usize>(
        &mut self,
        indices: [I; N],
    ) -> Result<[&mut T; N], GetManyMutError> {
        let len = self.len_idx();
        // NB: The optimizer should inline the loops into a sequence
        // of instructions without additional branching.
        for (i, idx) in indices.iter().enumerate() {
            if !idx.is_in_bounds(len) {
                return Err(GetManyMutError::IndexOutOfBounds);
            }
            for idx2 in &indices[..i] {
                if idx.is_overlapping(idx2) {
                    return Err(GetManyMutError::OverlappingIndices);
                }
            }
        }
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_many_unchecked_mut(indices)) }
    }
}
