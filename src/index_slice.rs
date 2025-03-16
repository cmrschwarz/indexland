use super::Idx;
use crate::{
    index_enumerate::IndexEnumerate, index_slice_index::IndexSliceIndex,
};

use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut, Range, RangeInclusive},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    pub(crate) data: [T],
}

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

impl<I, T> IndexSlice<I, T> {
    #[inline]
    pub fn from_slice(s: &[T]) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    pub fn from_mut_slice(s: &mut [T]) -> &mut Self {
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
    pub fn iter_enumerated(
        &self,
        initial_offset: I,
    ) -> IndexEnumerate<I, core::slice::Iter<T>> {
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
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
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
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
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
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }
    pub fn split_at_mut(
        &mut self,
        idx: I,
    ) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (l, r) = self.data.split_at_mut(idx.into_usize());
        (IndexSlice::from_mut_slice(l), IndexSlice::from_mut_slice(r))
    }
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

impl<I: Idx, T> IndexSlice<I, T> {
    /// # Safety
    /// Calling this method with overlapping keys is undefined behavior
    /// even if the resulting references are not used.
    #[allow(clippy::needless_pass_by_value)]
    pub unsafe fn get_disjoint_unchecked_mut<
        ISI: IndexSliceIndex<IndexSlice<I, T>> + GetDisjointMutIndex<I>,
        const N: usize,
    >(
        &mut self,
        indices: [ISI; N],
    ) -> [&mut ISI::Output; N] {
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

    #[allow(clippy::needless_pass_by_value)]
    pub fn get_disjoint_mut<
        ISI: IndexSliceIndex<IndexSlice<I, T>> + GetDisjointMutIndex<I>,
        const N: usize,
    >(
        &mut self,
        indices: [ISI; N],
    ) -> Result<[&mut ISI::Output; N], GetDisjointMutError> {
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
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_disjoint_unchecked_mut(indices)) }
    }
}

impl<I: Idx, T: Debug> Debug for IndexSlice<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T, Idx: IndexSliceIndex<IndexSlice<I, T>>> Index<Idx>
    for IndexSlice<I, T>
{
    type Output = Idx::Output;
    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        index.index(self)
    }
}

impl<I, T, ISI: IndexSliceIndex<IndexSlice<I, T>>> IndexMut<ISI>
    for IndexSlice<I, T>
{
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

impl<I, T: PartialEq, const N: usize> PartialEq<IndexSlice<I, T>> for [T; N] {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == &other.data
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<[T; N]> for IndexSlice<I, T> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self.data == other.as_slice()
    }
}

impl<I, T: PartialEq> PartialEq<IndexSlice<I, T>> for [T] {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self == &other.data
    }
}

impl<I, T: PartialEq> PartialEq<[T]> for IndexSlice<I, T> {
    fn eq(&self, other: &[T]) -> bool {
        &self.data == other
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

#[cfg(test)]
mod test {
    #[cfg(feature = "derive")]
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

        let [arr_slice_1, arr_slice_2] =
            arr.get_disjoint_mut([0..=1, 3..=4]).unwrap();

        assert_eq!(arr_slice_1.iter().copied().sum::<i32>(), 3);
        assert_eq!(arr_slice_2.iter().copied().sum::<i32>(), 9);
    }
}
