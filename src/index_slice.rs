use super::Idx;
use crate::{index_enumerate::IndexEnumerate, index_slice_index::IndexSliceIndex, IndexArray};

use core::{
    fmt::Debug,
    marker::PhantomData,
    num::NonZero,
    ops::{Index, IndexMut, Range, RangeInclusive},
};

#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, boxed::Box};

#[cfg(feature = "alloc")]
use crate::IndexVec;

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

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windows<'a, I, T> {
    v: &'a [T],
    size: NonZero<usize>,
    _phantom: PhantomData<&'a IndexSlice<I, T>>,
}

impl<I, T> Clone for Windows<'_, I, T> {
    fn clone(&self) -> Self {
        Self {
            v: self.v,
            size: self.size,
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T: 'a> Windows<'a, I, T> {
    #[inline]
    pub fn new(slice: &'a IndexSlice<I, T>, size: NonZero<usize>) -> Self {
        Self {
            v: slice.as_slice(),
            size,
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T> Iterator for Windows<'a, I, T> {
    type Item = &'a IndexSlice<I, T>;

    #[inline]
    fn next(&mut self) -> Option<&'a IndexSlice<I, T>> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let ret = Some(IndexSlice::from_slice(&self.v[..self.size.get()]));
            self.v = &self.v[1..];
            ret
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size.get() > self.v.len() {
            (0, Some(0))
        } else {
            let size = self.v.len() - self.size.get() + 1;
            (size, Some(size))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.get().overflowing_add(n);
        if end > self.v.len() || overflow {
            self.v = &[];
            None
        } else {
            let nth = &self.v[n..end];
            self.v = &self.v[n + 1..];
            Some(IndexSlice::from_slice(nth))
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let start = self.v.len() - self.size.get();
            Some(IndexSlice::from_slice(&self.v[start..]))
        }
    }
}

impl<'a, I, T> DoubleEndedIterator for Windows<'a, I, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a IndexSlice<I, T>> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let ret = Some(IndexSlice::from_slice(
                &self.v[self.v.len() - self.size.get()..],
            ));
            self.v = &self.v[..self.v.len() - 1];
            ret
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.v.len().overflowing_sub(n);
        if end < self.size.get() || overflow {
            self.v = &[];
            None
        } else {
            let ret = IndexSlice::from_slice(&self.v[end - self.size.get()..end]);
            self.v = &self.v[..end - 1];
            Some(ret)
        }
    }
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
            Some(arr) => Some(IndexArray::from_array_ref_mut(arr)),
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
                IndexArray::from_array_ref_mut(arr),
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
                IndexArray::from_array_ref_mut(arr),
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
            Some(arr) => Some(IndexArray::from_array_ref_mut(arr)),
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

    pub fn split_at_mut(&mut self, idx: I) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>)
    where
        I: Idx,
    {
        let (l, r) = self.data.split_at_mut(idx.into_usize());
        (IndexSlice::from_mut_slice(l), IndexSlice::from_mut_slice(r))
    }

    pub fn windows(&self, size: usize) -> Windows<'_, I, T> {
        let size = NonZero::new(size).expect("window size must be non-zero");
        Windows::new(self, size)
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

impl<I, T> ExactSizeIterator for Windows<'_, I, T> {}

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

impl<I, T> IndexSlice<I, T> {
    /// # Safety
    /// Calling this method with overlapping keys is undefined behavior
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
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_disjoint_unchecked_mut(indices)) }
    }
}

impl<I, T: Debug> Debug for IndexSlice<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
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

        let [arr_slice_1, arr_slice_2] = arr.get_disjoint_mut([0..=1, 3..=4]).unwrap();

        assert_eq!(arr_slice_1.iter().copied().sum::<i32>(), 3);
        assert_eq!(arr_slice_2.iter().copied().sum::<i32>(), 9);
    }
}
