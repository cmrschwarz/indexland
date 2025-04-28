use super::Idx;
use crate::{
    index_enumerate::IndexEnumerate, index_slice_index::IndexSliceIndex, IndexArray,
    IndexRangeBounds,
};

use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Index, IndexMut, Range, RangeInclusive},
};

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, borrow::ToOwned, boxed::Box, rc::Rc, sync::Arc, vec::Vec};

#[cfg(feature = "alloc")]
use crate::{IndexVec, IndexVecDeque};

#[repr(transparent)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    pub(crate) data: [T],
}

impl<I, T> IndexSlice<I, T> {
    #[inline(always)]
    pub const fn from_slice(s: &[T]) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline(always)]
    pub const fn from_mut_slice(s: &mut [T]) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Self) }
    }
    #[cfg(feature = "alloc")]
    pub fn from_boxed_slice(slice_box: Box<[T]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice_box) as *mut Self) }
    }
    #[cfg(feature = "alloc")]
    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [T]) }
    }
    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        &self.data
    }
    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len_idx(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.len())
    }
    pub fn last_idx(&self) -> Option<I>
    where
        I: Idx,
    {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub const fn first(&self) -> Option<&T> {
        self.data.first()
    }
    pub const fn first_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }
    pub const fn split_first(&self) -> Option<(&T, &IndexSlice<I, T>)> {
        match self.data.split_first() {
            Some((first, rest)) => Some((first, IndexSlice::from_slice(rest))),
            None => None,
        }
    }
    pub const fn split_first_mut(&mut self) -> Option<(&T, &mut IndexSlice<I, T>)> {
        match self.data.split_first_mut() {
            Some((first, rest)) => Some((first, IndexSlice::from_mut_slice(rest))),
            None => None,
        }
    }
    pub const fn split_last(&self) -> Option<(&T, &IndexSlice<I, T>)> {
        match self.data.split_last() {
            Some((first, rest)) => Some((first, IndexSlice::from_slice(rest))),
            None => None,
        }
    }
    pub const fn split_last_mut(&mut self) -> Option<(&T, &mut IndexSlice<I, T>)> {
        match self.data.split_last_mut() {
            Some((first, rest)) => Some((first, IndexSlice::from_mut_slice(rest))),
            None => None,
        }
    }
    pub const fn last(&self) -> Option<&T> {
        self.data.last()
    }
    pub const fn last_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
    pub const fn first_chunk<const N: usize>(&self) -> Option<&IndexArray<I, T, N>> {
        match self.data.first_chunk() {
            Some(arr) => Some(IndexArray::from_array_ref(arr)),
            None => None,
        }
    }
    pub const fn first_chunk_mut<const N: usize>(&mut self) -> Option<&IndexArray<I, T, N>> {
        match self.data.first_chunk_mut() {
            Some(arr) => Some(IndexArray::from_mut_array_ref(arr)),
            None => None,
        }
    }
    pub const fn split_first_chunk<const N: usize>(
        &self,
    ) -> Option<(&IndexArray<I, T, N>, &IndexSlice<I, T>)> {
        match self.data.split_first_chunk() {
            Some((arr, slice)) => Some((
                IndexArray::from_array_ref(arr),
                IndexSlice::from_slice(slice),
            )),
            None => None,
        }
    }
    pub const fn split_first_chunk_mut<const N: usize>(
        &mut self,
    ) -> Option<(&mut IndexArray<I, T, N>, &mut IndexSlice<I, T>)> {
        match self.data.split_first_chunk_mut() {
            Some((arr, slice)) => Some((
                IndexArray::from_mut_array_ref(arr),
                IndexSlice::from_mut_slice(slice),
            )),
            None => None,
        }
    }
    pub const fn split_last_chunk<const N: usize>(
        &self,
    ) -> Option<(&IndexSlice<I, T>, &IndexArray<I, T, N>)> {
        match self.data.split_last_chunk() {
            Some((slice, arr)) => Some((
                IndexSlice::from_slice(slice),
                IndexArray::from_array_ref(arr),
            )),
            None => None,
        }
    }
    pub const fn split_last_chunk_mut<const N: usize>(
        &mut self,
    ) -> Option<(&mut IndexSlice<I, T>, &mut IndexArray<I, T, N>)> {
        match self.data.split_last_chunk_mut() {
            Some((slice, arr)) => Some((
                IndexSlice::from_mut_slice(slice),
                IndexArray::from_mut_array_ref(arr),
            )),
            None => None,
        }
    }
    pub const fn last_chunk<const N: usize>(&self) -> Option<&IndexArray<I, T, N>> {
        match self.data.last_chunk() {
            Some(arr) => Some(IndexArray::from_array_ref(arr)),
            None => None,
        }
    }
    pub const fn last_chunk_mut<const N: usize>(&mut self) -> Option<&IndexArray<I, T, N>> {
        match self.data.last_chunk_mut() {
            Some(arr) => Some(IndexArray::from_mut_array_ref(arr)),
            None => None,
        }
    }
    pub fn get(&self, idx: I) -> Option<&T>
    where
        I: Idx,
    {
        self.data.get(idx.into_usize())
    }
    pub fn get_mut(&mut self, idx: I) -> Option<&mut T>
    where
        I: Idx,
    {
        self.data.get_mut(idx.into_usize())
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior,
    /// even if the resulting reference is not used.
    pub unsafe fn get_unchechecked(&self, idx: I) -> &T
    where
        I: Idx,
    {
        unsafe { self.data.get_unchecked(idx.into_usize()) }
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior,
    /// even if the resulting reference is not used.
    pub unsafe fn get_unchecked_mut(&mut self, idx: I) -> &mut T
    where
        I: Idx,
    {
        unsafe { self.data.get_unchecked_mut(idx.into_usize()) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *const T {
        self.data.as_mut_ptr()
    }

    pub fn as_ptr_range(&self) -> Range<*const T> {
        self.data.as_ptr_range()
    }

    pub fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        self.data.as_mut_ptr_range()
    }

    pub fn swap(&mut self, a: I, b: I)
    where
        I: Idx,
    {
        self.data.swap(a.into_usize(), b.into_usize());
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }

    pub fn windows(&self, size: usize) -> Windows<'_, I, T> {
        Windows::new(&self.data, size)
    }

    pub fn chunks(&self, size: usize) -> Chunks<'_, I, T> {
        Chunks::new(&self.data, size)
    }

    pub fn chunks_mut(&mut self, size: usize) -> ChunksMut<'_, I, T> {
        ChunksMut::new(&mut self.data, size)
    }

    pub fn chunks_exact(&self, size: usize) -> ChunksExact<'_, I, T> {
        ChunksExact::new(&self.data, size)
    }

    pub fn chunks_exact_mut(&mut self, size: usize) -> ChunksExactMut<'_, I, T> {
        ChunksExactMut::new(&mut self.data, size)
    }

    pub fn rchunks(&self, chunk_size: usize) -> RChunks<'_, I, T> {
        RChunks::new(&self.data, chunk_size)
    }
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, I, T> {
        RChunksMut::new(&mut self.data, chunk_size)
    }

    pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, I, T> {
        RChunksExact::new(&self.data, chunk_size)
    }
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, I, T> {
        RChunksExactMut::new(&mut self.data, chunk_size)
    }

    pub fn chunk_by<P>(&self, pred: P) -> ChunkBy<'_, I, T, P>
    where
        P: FnMut(&T, &T) -> bool,
    {
        ChunkBy::new(&self.data, pred)
    }

    pub fn chunk_by_mut<P>(&mut self, pred: P) -> ChunkByMut<'_, I, T, P>
    where
        P: FnMut(&T, &T) -> bool,
    {
        ChunkByMut::new(&mut self.data, pred)
    }

    pub fn split_at(&self, mid: I) -> (&IndexSlice<I, T>, &IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at(mid.into_usize());
        (a.into(), b.into())
    }

    pub fn split_at_mut(&mut self, mid: I) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at_mut(mid.into_usize());
        (a.into(), b.into())
    }

    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    /// The caller has to ensure that `0 <= mid <= self.len()`.
    pub unsafe fn split_at_unchecked(&self, mid: I) -> (&IndexSlice<I, T>, &IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at_unchecked(mid.into_usize_unchecked());
        (a.into(), b.into())
    }

    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    /// The caller has to ensure that `0 <= mid <= self.len()`.
    pub unsafe fn split_at_mut_unchecked(
        &mut self,
        mid: I,
    ) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at_mut_unchecked(mid.into_usize_unchecked());
        (a.into(), b.into())
    }

    pub fn split_at_checked(&self, mid: I) -> Option<(&IndexSlice<I, T>, &IndexSlice<I, T>)>
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at_checked(mid.into_usize())?;
        Some((a.into(), b.into()))
    }

    pub fn split_at_mut_checked(
        &mut self,
        mid: I,
    ) -> Option<(&mut IndexSlice<I, T>, &mut IndexSlice<I, T>)>
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at_mut_checked(mid.into_usize())?;
        Some((a.into(), b.into()))
    }

    pub fn split<F>(&self, pred: F) -> Split<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        Split::new(&self.data, pred)
    }

    pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitMut::new(&mut self.data, pred)
    }

    pub fn split_inclusive<F>(&self, pred: F) -> SplitInclusive<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitInclusive::new(&self.data, pred)
    }

    pub fn split_inclusive_mut<F>(&mut self, pred: F) -> SplitInclusiveMut<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitInclusiveMut::new(&mut self.data, pred)
    }

    pub fn rsplit<F>(&self, pred: F) -> RSplit<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplit::new(&self.data, pred)
    }

    pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitMut::new(&mut self.data, pred)
    }

    pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitN::new(&self.data, n, pred)
    }

    pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitNMut::new(&mut self.data, n, pred)
    }

    pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitN::new(&self.data, n, pred)
    }

    pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<'_, I, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitNMut::new(&mut self.data, n, pred)
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        self.data.contains(x)
    }

    pub fn starts_with<S: AsRef<[T]>>(&self, needle: &S) -> bool
    where
        T: PartialEq,
    {
        self.data.starts_with(needle.as_ref())
    }

    pub fn ends_with<S: AsRef<[T]>>(&self, needle: &S) -> bool
    where
        T: PartialEq,
    {
        self.data.ends_with(needle.as_ref())
    }

    pub fn strip_prefix<P: AsRef<[T]>>(&self, prefix: &P) -> Option<&IndexSlice<I, T>>
    where
        T: PartialEq,
    {
        self.data
            .strip_prefix(prefix.as_ref())
            .map(IndexSlice::from_slice)
    }

    pub fn strip_suffix<S: AsRef<[T]>>(&self, suffix: &S) -> Option<&IndexSlice<I, T>>
    where
        T: PartialEq,
    {
        self.data
            .strip_suffix(suffix.as_ref())
            .map(IndexSlice::from_slice)
    }

    pub fn binary_search(&self, x: &T) -> Result<I, I>
    where
        I: Idx,
        T: Ord,
    {
        self.data
            .binary_search(x)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn binary_search_by<F>(&self, f: F) -> Result<I, I>
    where
        I: Idx,
        F: FnMut(&T) -> core::cmp::Ordering,
    {
        self.data
            .binary_search_by(f)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<I, I>
    where
        I: Idx,
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.data
            .binary_search_by_key(b, f)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.data.sort_unstable();
    }

    pub fn sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        self.data.sort_unstable_by(compare);
    }

    pub fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.data.sort_unstable_by_key(f);
    }

    pub fn select_nth_unstable(
        &mut self,
        index: I,
    ) -> (&mut T, &mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
        T: Ord,
    {
        let (left, pivot, right) = self.data.select_nth_unstable(index.into_usize());
        (pivot, left.into(), right.into())
    }

    pub fn select_nth_unstable_by<F>(
        &mut self,
        index: I,
        compare: F,
    ) -> (&mut T, &mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        let (left, pivot, right) = self
            .data
            .select_nth_unstable_by(index.into_usize(), compare);
        (pivot, left.into(), right.into())
    }

    pub fn select_nth_unstable_by_key<K, F>(
        &mut self,
        index: I,
        f: F,
    ) -> (&mut T, &mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let (left, pivot, right) = self.data.select_nth_unstable_by_key(index.into_usize(), f);
        (pivot, left.into(), right.into())
    }

    pub fn rotate_left(&mut self, mid: I)
    where
        I: Idx,
    {
        self.data.rotate_left(mid.into_usize());
    }

    pub fn rotate_right(&mut self, k: I)
    where
        I: Idx,
    {
        self.data.rotate_right(k.into_usize());
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
    }

    pub fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        self.data.fill_with(f);
    }

    pub fn clone_from_slice<S>(&mut self, src: &S)
    where
        T: Clone,
        S: AsRef<[T]>,
    {
        self.data.clone_from_slice(src.as_ref());
    }

    pub fn copy_from_slice<S>(&mut self, src: &S)
    where
        T: Copy,
        S: AsRef<[T]>,
    {
        self.data.copy_from_slice(src.as_ref());
    }

    pub fn copy_within<R>(&mut self, src: R, dest: I)
    where
        I: Idx,
        T: Copy,
        R: IndexRangeBounds<I>,
    {
        self.data
            .copy_within(R::canonicalize(src, self.len()), dest.into_usize());
    }

    pub fn swap_with_slice(&mut self, other: &mut IndexSlice<I, T>) {
        self.data.swap_with_slice(&mut other.data);
    }

    /// ## Safety
    /// This method is essentially a [transmute](core::mem::transmute)
    /// with respect to the elements in the returned middle slice,
    /// so all the usual caveats pertaining to `transmute::<T, U>` also apply here.
    pub unsafe fn align_to<U>(&self) -> (&IndexSlice<I, T>, &IndexSlice<I, U>, &IndexSlice<I, T>) {
        let (head, mid, tail) = unsafe { self.data.align_to::<U>() };
        (head.into(), mid.into(), tail.into())
    }

    /// ## Safety
    /// This method is essentially a [transmute](core::mem::transmute)
    /// with respect to the elements in the returned middle slice,
    /// so all the usual caveats pertaining to `transmute::<T, U>` also apply here.
    pub unsafe fn align_to_mut<U>(
        &mut self,
    ) -> (
        &mut IndexSlice<I, T>,
        &mut IndexSlice<I, U>,
        &mut IndexSlice<I, T>,
    ) {
        let (head, mid, tail) = unsafe { self.data.align_to_mut::<U>() };
        (head.into(), mid.into(), tail.into())
    }

    pub fn is_sorted(&self) -> bool
    where
        T: PartialOrd,
    {
        self.data.is_sorted()
    }

    pub fn is_sorted_by<F>(&self, compare: F) -> bool
    where
        F: FnMut(&T, &T) -> bool,
    {
        self.data.is_sorted_by(compare)
    }

    pub fn is_sorted_by_key<'a, F, K>(&'a self, f: F) -> bool
    where
        F: FnMut(&'a T) -> K,
        K: PartialOrd,
    {
        self.data.is_sorted_by_key(f)
    }

    pub fn partition_point<P>(&self, pred: P) -> I
    where
        I: Idx,
        P: FnMut(&T) -> bool,
    {
        I::from_usize(self.data.partition_point(pred))
    }

    /// Get multiple mutable references to elements or subslices of the slice.
    ///
    /// # Safety
    /// Calling this method with overlapping indices is undefined behavior
    /// even if the resulting references are not used.
    #[allow(clippy::needless_pass_by_value)]
    pub unsafe fn get_disjoint_unchecked_mut<ISI, const N: usize>(
        &mut self,
        indices: [ISI; N],
    ) -> [&mut ISI::Output; N]
    where
        I: Idx,
        ISI: IndexSliceIndex<I, T> + GetDisjointMutIndex<I>,
    {
        let slice = self as *mut IndexSlice<I, T>;
        let mut arr: core::mem::MaybeUninit<[&mut ISI::Output; N]> =
            core::mem::MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `indices` to be disjunct and in bounds
        unsafe {
            for i in 0..N {
                let idx = indices.get_unchecked(i);
                arr_ptr
                    .cast::<&mut ISI::Output>()
                    .add(i)
                    .write(&mut *idx.clone().get_unchecked_mut(slice));
            }
            arr.assume_init()
        }
    }

    /// Get multiple mutable references to elements or subslices of the slice.
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_disjoint_mut<ISI, const N: usize>(
        &mut self,
        indices: [ISI; N],
    ) -> Result<[&mut ISI::Output; N], GetDisjointMutError>
    where
        I: Idx,
        ISI: IndexSliceIndex<I, T> + GetDisjointMutIndex<I>,
    {
        let len = self.len_idx();
        // NB: The optimizer should inline the loops into a sequence
        // of instructions without additional branching.
        for (i, idx) in indices.iter().enumerate() {
            if !idx.is_in_bounds(len) {
                return Err(GetDisjointMutError::IndexOutOfBounds);
            }
            for idx2 in &indices[..i] {
                if idx.is_overlapping(idx2) {
                    return Err(GetDisjointMutError::OverlappingIndices);
                }
            }
        }
        // SAFETY: We've checked that all indices are disjunct and in bounds
        unsafe { Ok(self.get_disjoint_unchecked_mut(indices)) }
    }
}

impl<I, T, const N: usize> IndexSlice<I, [T; N]> {
    /// Returns a view of the slice where N successive elements are flattened into one.
    pub fn as_flattened<'a>(&'a self) -> &'a IndexSlice<I, T>
    where
        T: 'a,
    {
        self.data.as_flattened().into()
    }

    /// Returns a mutable view of the slice where N successive elements are flattened into one.
    pub fn as_flattened_mut<'a>(&'a mut self) -> &'a mut IndexSlice<I, T>
    where
        T: 'a,
    {
        self.data.as_flattened_mut().into()
    }
}

impl<I, T> IndexSlice<I, T> {
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.data.sort_unstable();
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> core::cmp::Ordering,
    {
        self.data.sort_unstable_by(compare);
    }

    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.data.sort_unstable_by_key(f);
    }

    #[cfg(feature = "alloc")]
    pub fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.data.sort_by_cached_key(f);
    }

    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> alloc::vec::Vec<T>
    where
        T: Clone,
    {
        self.data.to_vec()
    }

    #[cfg(feature = "alloc")]
    pub fn to_index_vec(&self) -> IndexVec<I, T>
    where
        T: Clone,
    {
        self.data.to_vec().into()
    }

    #[cfg(feature = "alloc")]
    pub fn into_vec(self: alloc::boxed::Box<Self>) -> alloc::vec::Vec<T> {
        Self::into_boxed_slice(self).into_vec()
    }

    #[cfg(feature = "alloc")]
    pub fn into_index_vec<A>(self: alloc::boxed::Box<Self>) -> IndexVec<I, T> {
        self.into_vec().into()
    }

    #[cfg(feature = "alloc")]
    pub fn repeat(&self, n: usize) -> IndexVec<I, T>
    where
        T: Copy,
    {
        self.data.repeat(n).into()
    }

    pub fn concat<Item>(&self) -> <IndexSlice<I, T> as Concat<Item>>::Output
    where
        IndexSlice<I, T>: Concat<Item>,
        Item: ?Sized,
    {
        Concat::concat(self)
    }

    pub fn join<Separator>(&self, sep: Separator) -> <IndexSlice<I, T> as Join<Separator>>::Output
    where
        IndexSlice<I, T>: Join<Separator>,
    {
        Join::join(self, sep)
    }
}

impl<I, T> IndexSlice<I, T> {
    /// The slice version of `iter_enumerated` takes an `initial_offset`
    /// parameter to avoid the following common mistake:
    /// ``` compile fail
    /// # use indexland::{index_vec, Idx, IndexVec};
    /// # #[derive(Idx)]
    /// # struct MyId(u32);
    /// #
    /// # let myvec = IndexVec::from_iter(0..10);
    ///
    /// // !!! BUG: `i` would start at zero !!!
    /// for (i, &v) in myvec[MyId(1)..MyId(3)].iter_enumerated() {
    ///     println!("myvec[i] = {v}");
    /// }
    /// ```
    ///
    /// Instead, using [`iter_enumerated_range`](crate::IndexVec::iter_enumerated_range)
    /// on the container itself is preferred:
    /// ```
    /// # use indexland::{index_vec, Idx, IndexVec};
    /// # #[derive(Idx)]
    /// # struct MyId(u32);
    /// #
    /// # let myvec = IndexVec::from_iter(0..10);
    /// // instead, use the following code:
    /// for (i, &v) in myvec.iter_enumerated_range(MyId(1)..MyId(3)) {
    ///     println!("myvec[i] = {v}");
    /// }
    /// ```
    ///
    /// If you actualy wanted to enumerate a slice starting from zero,
    /// simply pass [`Idx::ZERO`] as the initial offset.
    pub fn iter_enumerated(&self, initial_offset: I) -> IndexEnumerate<I, core::slice::Iter<T>> {
        IndexEnumerate::new(initial_offset, &self.data)
    }

    /// See [`iter_enumerated`](IndexSlice::iter_enumerated) for why this api
    /// deviates by having an `initial_offset`.
    pub fn iter_enumerated_mut(
        &mut self,
        initial_offset: I,
    ) -> IndexEnumerate<I, core::slice::IterMut<T>> {
        IndexEnumerate::new(initial_offset, &mut self.data)
    }
}

// ===== Windows =====

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windows<'a, I, T> {
    windows: core::slice::Windows<'a, T>,
    _phantom: PhantomData<&'a IndexSlice<I, T>>,
}

impl<I, T> Clone for Windows<'_, I, T> {
    fn clone(&self) -> Self {
        Self {
            windows: self.windows.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T: 'a> Windows<'a, I, T> {
    #[inline]
    fn new(slice: &'a [T], size: usize) -> Self {
        Self {
            windows: slice.windows(size),
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T> Iterator for Windows<'a, I, T> {
    type Item = &'a IndexSlice<I, T>;

    #[inline]
    fn next(&mut self) -> Option<&'a IndexSlice<I, T>> {
        Some(IndexSlice::from_slice(self.windows.next()?))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.windows.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.windows.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(IndexSlice::from_slice(self.windows.nth(n)?))
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        Some(IndexSlice::from_slice(self.windows.last()?))
    }
}

impl<'a, I, T> DoubleEndedIterator for Windows<'a, I, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a IndexSlice<I, T>> {
        Some(IndexSlice::from_slice(self.windows.next_back()?))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        Some(IndexSlice::from_slice(self.windows.nth_back(n)?))
    }
}

// ===== get_disjoint_mut =====
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetDisjointMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}

/// `IndexSlice` version of the [`slice::get_disjoint_mut`] API
/// # Safety
/// If `is_in_bounds()` returns `true` it must be safe to index the slice with
/// the indices.
/// If `is_overlapping()` returns `false` for two (in bounds) indices it must
/// be safe to access a slice mutably at both indices the same time.
/// !! These validations must hold *after* the
/// `into_usize` conversion of the `Idx`, even if that conversion has changed
/// the value / ordering.
pub unsafe trait GetDisjointMutIndex<I>: Clone {
    fn is_in_bounds(&self, len: I) -> bool;
    fn is_overlapping(&self, other: &Self) -> bool;
}

unsafe impl<I: Idx> GetDisjointMutIndex<I> for I {
    #[inline]
    fn is_in_bounds(&self, len: I) -> bool {
        self.into_usize() < len.into_usize()
    }

    #[inline]
    fn is_overlapping(&self, other: &Self) -> bool {
        self.into_usize() == other.into_usize()
    }
}

unsafe impl<I: Idx> GetDisjointMutIndex<I> for Range<I> {
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

unsafe impl<I: Idx> GetDisjointMutIndex<I> for RangeInclusive<I> {
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

// ===== Concat =====
pub trait Concat<Item>
where
    Item: ?Sized,
{
    type Output;

    fn concat(slice: &Self) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<I, T: Clone, V: Borrow<[T]>> Concat<T> for IndexSlice<I, V> {
    type Output = IndexVec<I, T>;

    fn concat(slice: &Self) -> Self::Output {
        slice.data.concat().into()
    }
}

// ===== Join =====
pub trait Join<Separator> {
    type Output;

    fn join(slice: &Self, sep: Separator) -> Self::Output;
}

#[cfg(feature = "alloc")]
impl<I, T: Clone, V: Borrow<[T]>> Join<&IndexSlice<I, T>> for IndexSlice<I, V> {
    type Output = IndexVec<I, T>;

    fn join(slice: &Self, sep: &IndexSlice<I, T>) -> IndexVec<I, T> {
        slice.data.join(sep.as_slice()).into()
    }
}

#[cfg(feature = "alloc")]
impl<I, T: Clone, V: Borrow<[T]>> Join<&[T]> for IndexSlice<I, V> {
    type Output = IndexVec<I, T>;

    fn join(slice: &Self, sep: &[T]) -> IndexVec<I, T> {
        slice.data.join(sep).into()
    }
}

#[cfg(feature = "alloc")]
impl<I, T: Clone, V: Borrow<[T]>> Join<&T> for IndexSlice<I, V> {
    type Output = IndexVec<I, T>;

    fn join(slice: &Self, sep: &T) -> IndexVec<I, T> {
        slice.data.join(sep).into()
    }
}

// ===== Iters =====
macro_rules! wrap_chunk_iter {
    ($slice_fn: ident, $name: ident, $slice_fn_mut: ident, $name_mut: ident) => {
        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name<'a, I, T: 'a> {
            base: core::slice::$name<'a, T>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a> $name<'a, I, T> {
            #[inline]
            pub fn new(slice: &'a [T], size: usize) -> Self {
                Self {
                    base: slice.$slice_fn(size),
                    _phantom: PhantomData,
                }
            }
        }

        impl<I, T> Clone for $name<'_, I, T> {
            fn clone(&self) -> Self {
                Self {
                    base: self.base.clone(),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T> Iterator for $name<'a, I, T> {
            type Item = &'a IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<&'a IndexSlice<I, T>> {
                Some(IndexSlice::from_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.last()?))
            }
        }

        impl<'a, I, T> DoubleEndedIterator for $name<'a, I, T> {
            #[inline]
            fn next_back(&mut self) -> Option<&'a IndexSlice<I, T>> {
                Some(IndexSlice::from_slice(self.base.next_back()?))
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.nth_back(n)?))
            }
        }

        impl<I, T> ExactSizeIterator for $name<'_, I, T> {}
        impl<I, T> FusedIterator for $name<'_, I, T> {}

        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name_mut<'a, I, T: 'a> {
            base: core::slice::$name_mut<'a, T>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a> $name_mut<'a, I, T> {
            #[inline]
            pub fn new(slice: &'a mut [T], size: usize) -> Self {
                Self {
                    base: slice.$slice_fn_mut(size),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T> Iterator for $name_mut<'a, I, T> {
            type Item = &'a mut IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<&'a mut IndexSlice<I, T>> {
                Some(IndexSlice::from_mut_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.last()?))
            }
        }

        impl<'a, I, T> DoubleEndedIterator for $name_mut<'a, I, T> {
            #[inline]
            fn next_back(&mut self) -> Option<&'a mut IndexSlice<I, T>> {
                Some(IndexSlice::from_mut_slice(self.base.next_back()?))
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.nth_back(n)?))
            }
        }

        impl<I, T> ExactSizeIterator for $name_mut<'_, I, T> {}
        impl<I, T> FusedIterator for $name_mut<'_, I, T> {}
    };
}
macro_rules! wrap_pred_iter {
    (
        $slice_fn: ident, $name: ident,
        $slice_fn_mut: ident, $name_mut: ident,
        $($pred_ty: tt)*
    ) => {
        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name<'a, I, T: 'a, P: $($pred_ty)*> {
            base: core::slice::$name<'a, T, P>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a, P> $name<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            pub fn new(slice: &'a [T], pred: P) -> Self
            where
                P: $($pred_ty)*,
            {
                Self {
                    base: slice.$slice_fn(pred),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Clone for $name<'a, I, T, P>
        where
            core::slice::$name<'a, T, P>: Clone,
            P: $($pred_ty)*,
        {
            fn clone(&self) -> Self {
                Self {
                    base: self.base.clone(),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Iterator for $name<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            type Item = &'a IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.last()?))
            }
        }

        impl<'a, I, T, P> DoubleEndedIterator for $name<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.next_back()?))
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.nth_back(n)?))
            }
        }

        impl<I, T, P> FusedIterator for $name<'_, I, T, P> where P: $($pred_ty)* {}

        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name_mut<'a, I, T: 'a, P: $($pred_ty)*> {
            base: core::slice::$name_mut<'a, T, P>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a, P> $name_mut<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            pub fn new(slice: &'a mut [T], pred: P) -> Self
            where
                P: $($pred_ty)*,
            {
                Self {
                    base: slice.$slice_fn_mut(pred),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Iterator for $name_mut<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            type Item = &'a mut IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.last()?))
            }
        }

        impl<'a, I, T, P> DoubleEndedIterator for $name_mut<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            fn next_back(&mut self) -> Option<&'a mut IndexSlice<I, T>> {
                Some(IndexSlice::from_mut_slice(self.base.next_back()?))
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.nth_back(n)?))
            }
        }

        impl<I, T, P> FusedIterator for $name_mut<'_, I, T, P> where P: $($pred_ty)* {}
    };
}

macro_rules! wrap_pred_iter_n {
    (
        $slice_fn: ident, $name: ident,
        $slice_fn_mut: ident, $name_mut: ident,
        $($pred_ty: tt)*
    ) => {
        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name<'a, I, T: 'a, P: $($pred_ty)*> {
            base: core::slice::$name<'a, T, P>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a, P> $name<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            pub fn new(slice: &'a [T], n: usize, pred: P) -> Self
            where
                P: $($pred_ty)*,
            {
                Self {
                    base: slice.$slice_fn(n, pred),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Clone for $name<'a, I, T, P>
        where
            core::slice::$name<'a, T, P>: Clone,
            P: $($pred_ty)*,
        {
            fn clone(&self) -> Self {
                Self {
                    base: self.base.clone(),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Iterator for $name<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            type Item = &'a IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_slice(self.base.last()?))
            }
        }
        impl<I, T, P> FusedIterator for $name<'_, I, T, P> where P: $($pred_ty)* {}

        #[derive(Debug)]
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        pub struct $name_mut<'a, I, T: 'a, P: $($pred_ty)*> {
            base: core::slice::$name_mut<'a, T, P>,
            _phantom: PhantomData<&'a IndexSlice<I, T>>,
        }

        impl<'a, I, T: 'a, P> $name_mut<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            #[inline]
            pub fn new(slice: &'a mut [T], n: usize, pred: P) -> Self
            where
                P: $($pred_ty)*,
            {
                Self {
                    base: slice.$slice_fn_mut(n, pred),
                    _phantom: PhantomData,
                }
            }
        }

        impl<'a, I, T, P> Iterator for $name_mut<'a, I, T, P>
        where
            P: $($pred_ty)*,
        {
            type Item = &'a mut IndexSlice<I, T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.next()?))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.base.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.base.count()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.nth(n)?))
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                Some(IndexSlice::from_mut_slice(self.base.last()?))
            }
        }

        impl<I, T, P> FusedIterator for $name_mut<'_, I, T, P> where P: $($pred_ty)* {}
    };
}

wrap_chunk_iter!(chunks, Chunks, chunks_mut, ChunksMut);
wrap_chunk_iter!(chunks_exact, ChunksExact, chunks_exact_mut, ChunksExactMut);
wrap_chunk_iter!(rchunks, RChunks, rchunks_mut, RChunksMut);
wrap_chunk_iter!(
    rchunks_exact,
    RChunksExact,
    rchunks_exact_mut,
    RChunksExactMut
);

wrap_pred_iter!(chunk_by, ChunkBy, chunk_by_mut, ChunkByMut, FnMut(&T, &T) -> bool);
wrap_pred_iter!(split, Split, split_mut, SplitMut, FnMut(&T) -> bool);
wrap_pred_iter!(split_inclusive, SplitInclusive, split_inclusive_mut, SplitInclusiveMut, FnMut(&T) -> bool);
wrap_pred_iter!(rsplit, RSplit, rsplit_mut, RSplitMut, FnMut(&T) -> bool);

wrap_pred_iter_n!(splitn, SplitN, splitn_mut, SplitNMut, FnMut(&T) -> bool);
wrap_pred_iter_n!(rsplitn, RSplitN, rsplitn_mut, RSplitNMut, FnMut(&T) -> bool);

// ===== traits ======

impl<I, T> AsRef<[T]> for IndexSlice<I, T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<I, T> AsMut<[T]> for IndexSlice<I, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<I, T> Borrow<[T]> for IndexSlice<I, T> {
    fn borrow(&self) -> &[T] {
        &self.data
    }
}

impl<I, T> BorrowMut<[T]> for IndexSlice<I, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<I, T: Debug> Debug for IndexSlice<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T> Default for &IndexSlice<I, T> {
    fn default() -> Self {
        IndexSlice::from_slice(&[])
    }
}
impl<I, T> Default for &mut IndexSlice<I, T> {
    fn default() -> Self {
        IndexSlice::from_mut_slice(&mut [])
    }
}
#[cfg(feature = "alloc")]
impl<I, T> Default for alloc::boxed::Box<IndexSlice<I, T>> {
    fn default() -> Self {
        IndexSlice::from_boxed_slice(Box::<[T]>::default())
    }
}

#[cfg(feature = "alloc")]
impl<I, T> From<&[T]> for alloc::boxed::Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        IndexSlice::from_boxed_slice(alloc::boxed::Box::from(value))
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&mut [T]> for alloc::boxed::Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &mut [T]) -> Self {
        IndexSlice::from_boxed_slice(alloc::boxed::Box::from(value))
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&IndexSlice<I, T>> for alloc::boxed::Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &IndexSlice<I, T>) -> Self {
        IndexSlice::from_boxed_slice(alloc::boxed::Box::from(value.as_slice()))
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&mut IndexSlice<I, T>> for alloc::boxed::Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &mut IndexSlice<I, T>) -> Self {
        IndexSlice::from_boxed_slice(alloc::boxed::Box::from(value.as_slice()))
    }
}

// unfortunately cannot implement From<&[T]> for Rc<IndexSlice<I, T>>
// unfortunately cannot implement From<&mut [T]> for Rc<IndexSlice<I, T>>
#[cfg(feature = "alloc")]
impl<I, T> From<&IndexSlice<I, T>> for Rc<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &IndexSlice<I, T>) -> Self {
        let res = From::<&[T]>::from(value.as_slice());
        unsafe { core::mem::transmute::<Rc<[T]>, _>(res) }
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&mut IndexSlice<I, T>> for Rc<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &mut IndexSlice<I, T>) -> Self {
        let res = From::<&[T]>::from(value.as_slice());
        unsafe { core::mem::transmute::<Rc<[T]>, _>(res) }
    }
}

// unfortunately cannot implement From<&[T]> for Arc<IndexSlice<I, T>>
// unfortunately cannot implement From<&mut [T]> for Arc<IndexSlice<I, T>>
#[cfg(feature = "alloc")]
impl<I, T> From<&IndexSlice<I, T>> for Arc<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &IndexSlice<I, T>) -> Self {
        let res = From::<&[T]>::from(value.as_slice());
        unsafe { core::mem::transmute::<Arc<[T]>, _>(res) }
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&mut IndexSlice<I, T>> for Arc<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &mut IndexSlice<I, T>) -> Self {
        let res = From::<&[T]>::from(value.as_slice());
        unsafe { core::mem::transmute::<Arc<[T]>, _>(res) }
    }
}

#[cfg(feature = "alloc")]
impl<I, T> From<&IndexSlice<I, T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &IndexSlice<I, T>) -> Self {
        Vec::from(value.as_slice())
    }
}
#[cfg(feature = "alloc")]
impl<I, T> From<&mut IndexSlice<I, T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &mut IndexSlice<I, T>) -> Self {
        Vec::from(value.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<'a, I, T> From<&'a IndexSlice<I, T>> for Cow<'a, IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &'a IndexSlice<I, T>) -> Self {
        Cow::Borrowed(value)
    }
}

// unfortunately cannot implement Cow<[T]> for Box<IndexSlice<I, T>>
#[cfg(feature = "alloc")]
impl<I, T> From<Cow<'_, IndexSlice<I, T>>> for Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: Cow<'_, IndexSlice<I, T>>) -> Self {
        value.into_owned().into_boxed_index_slice()
    }
}

#[cfg(feature = "alloc")]
impl<I, T, const N: usize> From<[T; N]> for Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: [T; N]) -> Self {
        IndexSlice::from_boxed_slice(Box::from(value))
    }
}
#[cfg(feature = "alloc")]
impl<I, T, const N: usize> From<IndexArray<I, T, N>> for Box<IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: IndexArray<I, T, N>) -> Self {
        IndexSlice::from_boxed_slice(Box::from(value.into_array()))
    }
}

#[cfg(feature = "alloc")]
impl<I, T> FromIterator<T> for Box<IndexSlice<I, T>> {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        IndexSlice::from_boxed_slice(Box::from_iter(iter))
    }
}

impl<I, T, Idx: IndexSliceIndex<I, T>> Index<Idx> for IndexSlice<I, T> {
    type Output = Idx::Output;
    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        index.index(self)
    }
}

impl<I, T, ISI: IndexSliceIndex<I, T>> IndexMut<ISI> for IndexSlice<I, T> {
    #[inline]
    fn index_mut(&mut self, index: ISI) -> &mut Self::Output {
        index.index_mut(self)
    }
}

impl<'a, I, T> IntoIterator for &'a IndexSlice<I, T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut IndexSlice<I, T> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<I, T> IntoIterator for Box<IndexSlice<I, T>> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

#[cfg(feature = "alloc")]
impl<'a, I, T> IntoIterator for &'a Box<IndexSlice<I, T>> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

#[cfg(feature = "alloc")]
impl<'a, I, T> IntoIterator for &'a mut Box<IndexSlice<I, T>> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<I, T, U> PartialEq<IndexSlice<I, U>> for IndexSlice<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexSlice<I, U>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
impl<I, T> Eq for IndexSlice<I, T> where T: PartialEq {}

impl<I, T> PartialOrd for IndexSlice<I, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
impl<I, T> Ord for IndexSlice<I, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}
impl<I, T> Hash for IndexSlice<I, T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<I, T, U, const N: usize> PartialEq<IndexArray<I, U, N>> for IndexSlice<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexArray<I, U, N>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<I, T, U> PartialEq<Cow<'_, IndexSlice<I, U>>> for IndexSlice<I, T>
where
    T: PartialEq<U>,
    U: Clone,
{
    fn eq(&self, other: &Cow<'_, IndexSlice<I, U>>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<I, T, U> PartialEq<IndexSlice<I, U>> for Cow<'_, IndexSlice<I, T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &IndexSlice<I, U>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<I, T, U> PartialEq<IndexVec<I, U>> for IndexSlice<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<I, T, U> PartialEq<IndexVecDeque<I, U>> for IndexSlice<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVecDeque<I, U>) -> bool {
        let s = self.as_slice();
        let (s1, s2) = other.as_slices();
        s1.len() + s2.len() == s.len() && &s[0..s1.len()] == s1 && &s[s1.len()..] == s2
    }
}

impl<'a, I, T> From<&'a IndexSlice<I, T>> for &'a [T] {
    fn from(value: &'a IndexSlice<I, T>) -> Self {
        value.as_slice()
    }
}
impl<'a, I, T> From<&'a mut IndexSlice<I, T>> for &'a mut [T] {
    fn from(value: &'a mut IndexSlice<I, T>) -> Self {
        value.as_mut_slice()
    }
}

impl<'a, I, T> From<&'a [T]> for &'a IndexSlice<I, T> {
    fn from(value: &'a [T]) -> Self {
        IndexSlice::from_slice(value)
    }
}
impl<'a, I, T> From<&'a mut [T]> for &'a mut IndexSlice<I, T> {
    fn from(value: &'a mut [T]) -> Self {
        IndexSlice::from_mut_slice(value)
    }
}

#[cfg(feature = "alloc")]
impl<I, T> ToOwned for IndexSlice<I, T>
where
    T: Clone,
{
    type Owned = IndexVec<I, T>;

    fn to_owned(&self) -> Self::Owned {
        IndexVec::from(self.as_slice().to_vec())
    }
}

// ===== serde =====
#[cfg(feature = "serde")]
impl<I, T> serde::Serialize for IndexSlice<I, T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(all(test, feature = "derive"))]
mod test {
    #[test]
    fn get_disjoint_mut() {
        use crate::{index_array, Idx, IndexArray};

        #[derive(Idx)]
        enum I {
            A,
            B,
            C,
            D,
            E,
        }

        let mut arr: IndexArray<usize, i32, 5> = index_array![1, 2, 3, 4, 5];

        let [arr_slice_1, arr_slice_2] = arr.get_disjoint_mut([0..=1, 3..=4]).unwrap();

        assert_eq!(arr_slice_1.iter().copied().sum::<i32>(), 3);
        assert_eq!(arr_slice_2.iter().copied().sum::<i32>(), 9);
    }
}
