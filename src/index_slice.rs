use super::Idx;
use crate::{idx_enumerate::IdxEnumerate, idx_range::RangeBoundsAsRange};

use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    data: [T],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetManyMutError {
    IndexOutOfBounds,
    OverlappingIndices,
}

/// `IndexSlice` version of the unstable `std::slice::get_many_mut` API
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

impl<I: Idx, T> IndexSlice<I, T> {
    #[inline]
    pub fn from_slice(s: &[T]) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    pub fn from_slice_mut(s: &mut [T]) -> &mut Self {
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
    pub fn iter_enumerated(
        &self,
        initial_offset: I,
    ) -> IdxEnumerate<I, core::slice::Iter<T>> {
        IdxEnumerate::new(initial_offset, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
        initial_offset: I,
    ) -> IdxEnumerate<I, core::slice::IterMut<T>> {
        IdxEnumerate::new(initial_offset, &mut self.data)
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn len_idx(&self) -> I {
        I::from_usize(self.data.len())
    }
    pub fn last_idx(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub fn get(&self, idx: I) -> Option<&T> {
        self.data.get(idx.into_usize())
    }
    pub fn get_mut(&mut self, idx: I) -> Option<&mut T> {
        self.data.get_mut(idx.into_usize())
    }
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }
    pub fn split_at_mut(
        &mut self,
        idx: I,
    ) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>) {
        let (l, r) = self.data.split_at_mut(idx.into_usize());
        (IndexSlice::from_slice_mut(l), IndexSlice::from_slice_mut(r))
    }
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
        let mut arr: core::mem::MaybeUninit<[&mut T; N]> =
            core::mem::MaybeUninit::uninit();
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

impl<I: Idx, T: Debug> Debug for IndexSlice<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T> Index<I> for IndexSlice<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexSlice<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a IndexSlice<I, T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut IndexSlice<I, T> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I: Idx, T> Index<Range<I>> for IndexSlice<I, T> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::from_slice(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T> IndexMut<Range<I>> for IndexSlice<I, T> {
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::from_slice_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<IndexSlice<I, T>>
    for [T; N]
{
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == &other.data
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<[T; N]>
    for IndexSlice<I, T>
{
    fn eq(&self, other: &[T; N]) -> bool {
        &self.data == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<IndexSlice<I, T>> for [T] {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self == &other.data
    }
}

impl<I: Idx, T: PartialEq> PartialEq<[T]> for IndexSlice<I, T> {
    fn eq(&self, other: &[T]) -> bool {
        &self.data == other
    }
}

impl<'a, I: Idx, T> From<&'a IndexSlice<I, T>> for &'a [T] {
    fn from(value: &'a IndexSlice<I, T>) -> Self {
        value.as_slice()
    }
}
impl<'a, I: Idx, T> From<&'a mut IndexSlice<I, T>> for &'a mut [T] {
    fn from(value: &'a mut IndexSlice<I, T>) -> Self {
        value.as_slice_mut()
    }
}

impl<'a, I: Idx, T> From<&'a [T]> for &'a IndexSlice<I, T> {
    fn from(value: &'a [T]) -> Self {
        IndexSlice::from_slice(value)
    }
}
impl<'a, I: Idx, T> From<&'a mut [T]> for &'a mut IndexSlice<I, T> {
    fn from(value: &'a mut [T]) -> Self {
        IndexSlice::from_slice_mut(value)
    }
}

macro_rules! slice_index_impl {
    ($($range_type: ident),*) => {$(
        impl<I: Idx, T> Index<$range_type<I>> for IndexSlice<I, T> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::from_slice(&self.data[rb.as_usize_range(self.len())])
            }
        }

        impl<I: Idx, T> IndexMut<$range_type<I>> for IndexSlice<I, T> {
            #[inline]
            fn index_mut(&mut self, rb: $range_type<I>) -> &mut Self::Output {
                let range = rb.as_usize_range(self.len());
                IndexSlice::from_slice_mut(&mut self.data[range])
            }
        }
    )*};
}

slice_index_impl!(RangeInclusive, RangeFrom, RangeTo, RangeToInclusive);
