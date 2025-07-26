use core::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use alloc::{
    collections::{TryReserveError, VecDeque},
    vec::Vec,
};

use crate::{index_enumerate::IndexEnumerate, IdxCompat, IndexArray, IndexRangeBounds, IndexVec};

use super::{idx::Idx, index_range::IndexRange, index_slice::IndexSlice};

/// Create an [`IndexVecDeque`] containing the arguments.
///
/// The syntax is identical to [`vec!`](alloc::vec!).
///
/// The index type cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// # Example
/// ```
/// use indexland::{index_vec_deque, IndexVecDeque};
///
/// let vd: IndexVecDeque<u32, _> = index_vec_deque![-1, 2, 3];
/// ```
#[macro_export]
macro_rules! index_vec_deque {
    () => {
        $crate::IndexVecDeque::from([])
    };
    ($value:expr; $count: expr) => {
        $crate::IndexVecDeque::from($crate::alloc::vec![$value; $count])
    };
    ($($value:expr),+ $(,)?) => {
        $crate::IndexVecDeque::from([$($value),*])
    };
    ($($index:expr => $value:expr),* $(,)?) => {{
        let indices = [ $($index as usize),* ];
        let mut values = [ $($value),* ];
        let data = $crate::__private::array_from_values_and_distinct_indices(
            indices,
            ::core::mem::ManuallyDrop::new(values)
        );
        $crate::IndexVecDeque::from(data)
    }};
}

#[repr(transparent)]
pub struct IndexVecDeque<I, T> {
    data: VecDeque<T>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I, T> IndexVecDeque<I, T> {
    pub const fn new() -> Self {
        Self {
            data: VecDeque::new(),
            _phantom: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(cap),
            _phantom: PhantomData,
        }
    }
    pub fn get<C>(&self, index: C) -> Option<&T>
    where
        C: IdxCompat<I>,
    {
        self.data.get(index.into_usize())
    }
    pub fn get_mut<C>(&mut self, index: C) -> Option<&mut T>
    where
        C: IdxCompat<I>,
    {
        self.data.get_mut(index.into_usize())
    }
    pub fn swap(&mut self, i: I, j: I)
    where
        I: Idx,
    {
        self.data.swap(i.into_usize(), j.into_usize());
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn capacity_idx(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.capacity())
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }
    pub fn reserve_exact_total(&mut self, cap_idx: I)
    where
        I: Idx,
    {
        self.data
            .reserve_exact(cap_idx.into_usize().saturating_sub(self.len()));
    }
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn reserve_total(&mut self, cap_idx: I)
    where
        I: Idx,
    {
        self.data
            .reserve(cap_idx.into_usize().saturating_sub(self.len()));
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(additional)
    }
    pub fn try_reserve_exact_total(&mut self, cap_idx: I) -> Result<(), TryReserveError>
    where
        I: Idx,
    {
        self.data
            .try_reserve_exact(cap_idx.into_usize().saturating_sub(self.len()))
    }
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve(additional)
    }
    pub fn try_reserve_total(&mut self, cap_idx: I) -> Result<(), TryReserveError>
    where
        I: Idx,
    {
        self.data
            .try_reserve(cap_idx.into_usize().saturating_sub(self.len()))
    }
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    pub fn shrink_to(&mut self, idx: I)
    where
        I: Idx,
    {
        self.data.shrink_to(idx.into_usize());
    }
    pub fn truncate(&mut self, end: I)
    where
        I: Idx,
    {
        self.data.truncate(end.into_usize());
    }
    pub fn iter(&self) -> alloc::collections::vec_deque::Iter<'_, T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> alloc::collections::vec_deque::IterMut<'_, T> {
        self.data.iter_mut()
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, alloc::collections::vec_deque::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::IterMut<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.range(range.canonicalize(self.len())))
    }
    pub fn iter_enumerated_range_mut(
        &mut self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::IterMut<'_, T>>
    where
        I: Idx,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, self.data.range_mut(range))
    }
    pub fn as_slices(&self) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
        let (s1, s2) = self.data.as_slices();
        (
            IndexSlice::from_raw_slice(s1),
            IndexSlice::from_raw_slice(s2),
        )
    }
    pub fn as_mut_slices(&mut self) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
        let (s1, s2) = self.data.as_mut_slices();
        (
            IndexSlice::from_raw_slice(s1),
            IndexSlice::from_raw_slice(s2),
        )
    }
    pub fn as_raw_slices(&self) -> (&[T], &[T]) {
        self.data.as_slices()
    }
    pub fn as_mut_raw_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.data.as_mut_slices()
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
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn range<R>(&self, range: R) -> alloc::collections::vec_deque::Iter<'_, T>
    where
        R: IndexRangeBounds<I>,
    {
        self.data.range(range.usize_range())
    }
    pub fn range_mut<R>(&mut self, range: R) -> alloc::collections::vec_deque::IterMut<'_, T>
    where
        R: IndexRangeBounds<I>,
    {
        self.data.range_mut(range.usize_range())
    }
    pub fn drain<R>(&mut self, range: R) -> alloc::collections::vec_deque::Drain<'_, T>
    where
        R: IndexRangeBounds<usize>,
    {
        self.data.drain(range.usize_range())
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        self.data.contains(x)
    }
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }
    pub fn pop_front_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        // VecDeque::pop_front_if is unstable, so we implement it manually
        if let Some(front) = self.data.front_mut() {
            if predicate(front) {
                return self.data.pop_front();
            }
        }
        None
    }
    pub fn pop_back_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        // VecDeque::pop_back_if is unstable, so we implement it manually
        if let Some(back) = self.data.back_mut() {
            if predicate(back) {
                return self.data.pop_back();
            }
        }
        None
    }
    pub fn push_front(&mut self, v: T) {
        self.data.push_front(v);
    }

    pub fn push_back(&mut self, v: T) {
        self.data.push_back(v);
    }
    pub fn push_back_get_idx(&mut self, v: T) -> I
    where
        I: Idx,
    {
        let id = self.len_idx();
        self.data.push_back(v);
        id
    }
    pub fn swap_remove_front(&mut self, idx: I) -> Option<T>
    where
        I: Idx,
    {
        self.data.swap_remove_front(idx.into_usize())
    }
    pub fn swap_remove_back(&mut self, idx: I) -> Option<T>
    where
        I: Idx,
    {
        self.data.swap_remove_back(idx.into_usize())
    }
    pub fn insert(&mut self, index: I, value: T)
    where
        I: Idx,
    {
        self.data.insert(index.into_usize(), value);
    }
    pub fn remove(&mut self, index: I) -> Option<T>
    where
        I: Idx,
    {
        self.data.remove(index.into_usize())
    }
    pub fn split_off(&mut self, at: I) -> IndexVecDeque<I, T>
    where
        I: Idx,
    {
        IndexVecDeque::from(self.data.split_off(at.into_usize()))
    }
    pub fn append<D: AsMut<VecDeque<T>>>(&mut self, other: &mut D) {
        self.data.append(other.as_mut());
    }
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.data.retain(f);
    }
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.data.retain_mut(f);
    }

    pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T) {
        self.data.resize_with(new_len, f);
    }

    pub fn make_contiguous(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_mut_raw_slice(self.data.make_contiguous())
    }

    pub fn rotate_left(&mut self, n: I)
    where
        I: Idx,
    {
        self.data.rotate_left(n.into_usize());
    }

    pub fn rotate_right(&mut self, n: I)
    where
        I: Idx,
    {
        self.data.rotate_right(n.into_usize());
    }

    pub fn binary_search(&self, x: &T) -> Result<I, I>
    where
        T: Ord,
        I: Idx,
    {
        match self.data.binary_search(x) {
            Ok(idx) => Ok(I::from_usize(idx)),
            Err(idx) => Err(I::from_usize(idx)),
        }
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> core::cmp::Ordering,
        I: Idx,
    {
        match self.data.binary_search_by(f) {
            Ok(idx) => Ok(I::from_usize(idx)),
            Err(idx) => Err(I::from_usize(idx)),
        }
    }

    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
        I: Idx,
    {
        match self.data.binary_search_by_key(b, f) {
            Ok(idx) => Ok(I::from_usize(idx)),
            Err(idx) => Err(I::from_usize(idx)),
        }
    }

    pub fn partition_point<P>(&self, pred: P) -> I
    where
        P: FnMut(&T) -> bool,
        I: Idx,
    {
        I::from_usize(self.data.partition_point(pred))
    }

    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.data.resize(new_len, value);
    }

    pub fn as_vec_deque(&self) -> &VecDeque<T> {
        &self.data
    }
    pub fn as_mut_vec_deque(&mut self) -> &mut VecDeque<T> {
        &mut self.data
    }

    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::IntoIter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn indices(&self) -> IndexRange<I>
    where
        I: Idx,
    {
        IndexRange::new(I::ZERO..self.len_idx())
    }

    /// same as [`From<IndexArray<I, T, N>>::from`], useful for better type inference
    pub fn from_index_array<const N: usize>(arr: IndexArray<I, T, N>) -> Self {
        Self::from_iter(arr.into_inner())
    }

    #[inline(always)]
    pub const fn from_vec_deque(v: VecDeque<T>) -> Self {
        Self {
            data: v,
            _phantom: PhantomData,
        }
    }

    pub const fn into_vec_deque(self) -> VecDeque<T> {
        // required because this function is const
        let res = unsafe { core::ptr::read(&raw const self.data) };
        core::mem::forget(self);
        res
    }

    // Additional missing VecDeque functions
    pub fn resize_to_idx_with<F>(&mut self, new_len_idx: I, f: F)
    where
        I: Idx,
        F: FnMut() -> T,
    {
        self.data.resize_with(new_len_idx.into_usize(), f);
    }

    pub fn resize_to_idx(&mut self, len_idx: I, value: T)
    where
        T: Clone,
        I: Idx,
    {
        self.data.resize(len_idx.into_usize(), value);
    }
}

#[cfg(feature = "std")]
impl<I> std::io::BufRead for IndexVecDeque<I, u8> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.data.fill_buf()
    }

    fn consume(&mut self, amount: usize) {
        self.data.consume(amount);
    }
}

impl<I, T> Clone for IndexVecDeque<I, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _phantom: PhantomData,
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<I, T: Debug> Debug for IndexVecDeque<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T> Default for IndexVecDeque<I, T> {
    fn default() -> Self {
        Self {
            data: VecDeque::new(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T> Extend<&'a T> for IndexVecDeque<I, T>
where
    T: 'a + Copy,
{
    #[inline]
    fn extend<It: IntoIterator<Item = &'a T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}

impl<I, T> Extend<T> for IndexVecDeque<I, T> {
    #[inline]
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}
impl<I, T, const N: usize> From<[T; N]> for IndexVecDeque<I, T> {
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}
impl<I, T, const N: usize> From<IndexArray<I, T, N>> for IndexVecDeque<I, T> {
    fn from(value: IndexArray<I, T, N>) -> Self {
        Self::from_iter(value)
    }
}

impl<I, T> From<Vec<T>> for IndexVecDeque<I, T> {
    fn from(value: Vec<T>) -> Self {
        Self::from_vec_deque(VecDeque::from(value))
    }
}
impl<I, T> From<IndexVec<I, T>> for IndexVecDeque<I, T> {
    fn from(value: IndexVec<I, T>) -> Self {
        IndexVecDeque {
            data: VecDeque::from(Vec::from(value)),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> From<VecDeque<T>> for IndexVecDeque<I, T> {
    fn from(value: VecDeque<T>) -> Self {
        IndexVecDeque {
            data: value,
            _phantom: PhantomData,
        }
    }
}
impl<I, T> From<IndexVecDeque<I, T>> for VecDeque<T> {
    fn from(value: IndexVecDeque<I, T>) -> Self {
        value.data
    }
}

impl<I, T> FromIterator<T> for IndexVecDeque<I, T> {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        Self::from_vec_deque(VecDeque::from_iter(iter))
    }
}

impl<I, T> core::hash::Hash for IndexVecDeque<I, T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<I, C, T> Index<C> for IndexVecDeque<I, T>
where
    C: IdxCompat<I>,
{
    type Output = T;
    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I, C, T> IndexMut<C> for IndexVecDeque<I, T>
where
    C: IdxCompat<I>,
{
    #[inline]
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<'a, I, T> IntoIterator for &'a IndexVecDeque<I, T> {
    type Item = &'a T;
    type IntoIter = alloc::collections::vec_deque::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut IndexVecDeque<I, T> {
    type Item = &'a mut T;
    type IntoIter = alloc::collections::vec_deque::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<I, T> IntoIterator for IndexVecDeque<I, T> {
    type Item = T;
    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<I, T> core::cmp::Ord for IndexVecDeque<I, T>
where
    T: core::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl<I, T, U> PartialEq<&[U]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U, const N: usize> PartialEq<&[U; N]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U; N]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<&mut [U]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut [U]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U, const N: usize> PartialEq<&mut [U; N]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut [U; N]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U, const N: usize> PartialEq<[U; N]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; N]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<Vec<U>> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<VecDeque<U>> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &VecDeque<U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<IndexVecDeque<I, U>> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVecDeque<I, U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<[U]> for IndexVecDeque<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<IndexVecDeque<I, U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVecDeque<I, U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T, U> PartialEq<IndexVecDeque<I, U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVecDeque<I, U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I1, I2, T, U> PartialEq<IndexVec<I2, U>> for IndexVecDeque<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I2, U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I, T> PartialOrd for IndexVecDeque<I, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

#[cfg(feature = "std")]
impl<I> std::io::Read for IndexVecDeque<I, u8> {
    /// Fill `buf` with the contents of the "front" slice as returned by
    /// [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are
    /// discontiguous, multiple calls to `read` will be needed to read the entire content.
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.data.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.data.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut alloc::string::String) -> std::io::Result<usize> {
        self.data.read_to_string(buf)
    }
}

#[cfg(feature = "std")]
impl<I> std::io::Write for IndexVecDeque<I, u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.data.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.data.extend(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<I, T> Eq for IndexVecDeque<I, T> where T: Eq {}
