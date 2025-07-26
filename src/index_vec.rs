use crate::{
    index_enumerate::IndexEnumerate, sequence::SequenceIndex, IdxCompat, IndexArray,
    IndexRangeBounds, IndexVecDeque,
};

use alloc::{
    borrow::{Cow, ToOwned},
    collections::{BinaryHeap, VecDeque},
    rc::Rc,
    sync::Arc,
    vec::Splice,
};
use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use alloc::{
    boxed::Box,
    collections::TryReserveError,
    vec::{Drain, Vec},
};

use super::{idx::Idx, index_range::IndexRange, index_slice::IndexSlice};

/// Create an [`IndexVec`] containing the arguments.
///
/// The syntax is identical to [`vec!`](alloc::vec!).
/// The index type cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// # Example
/// ```
/// use indexland::{index_vec, IndexVec};
///
/// let v: IndexVec<u32, _> = index_vec![-1, 2, 3];
/// ```
#[macro_export]
macro_rules! index_vec {
    () => {
        $crate::IndexVec::new()
    };
    ($value:expr; $count: expr) => {
        $crate::IndexVec::from_vec($crate::alloc::vec![$value; $count])
    };
    ($($value:expr),+ $(,)?) => {
        $crate::IndexVec::from([$($value),*])
    };
    ($($index:expr => $value:expr),* $(,)?) => {{
        let indices = [ $($index),* ];
        let mut values = [ $($value),* ];
        let data = $crate::__private::index_array_from_values_and_distinct_indices(
            indices,
            ::core::mem::ManuallyDrop::new(values)
        );
        $crate::IndexVec::from_index_array(data)
    }};
}

#[repr(transparent)]
pub struct IndexVec<I, T> {
    data: Vec<T>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I, T> IndexVec<I, T> {
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            _phantom: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            _phantom: PhantomData,
        }
    }
    /// ## Safety
    ///  See [`Vec::from_raw_parts`] for the invariants that this has to uphold.
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Self {
            data: Vec::from_raw_parts(ptr, length, capacity),
            _phantom: PhantomData,
        }
    }
    pub fn into_raw_parts(mut self) -> (*mut T, usize, usize) {
        let len = self.data.len();
        let cap = self.data.capacity();
        let ptr = self.data.as_mut_ptr();
        core::mem::forget(self);
        (ptr, len, cap)
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
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn reserve_total(&mut self, cap_idx: I)
    where
        I: Idx,
    {
        self.data
            .reserve(self.data.len().saturating_sub(cap_idx.into_usize()));
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }
    pub fn reserve_exact_total(&mut self, cap_idx: I)
    where
        I: Idx,
    {
        self.data
            .reserve_exact(self.data.len().saturating_sub(cap_idx.into_usize()));
    }
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve(additional)
    }
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(additional)
    }
    pub fn try_reserve_total(&mut self, cap_idx: I) -> Result<(), TryReserveError>
    where
        I: Idx,
    {
        self.data
            .try_reserve(self.data.len().saturating_sub(cap_idx.into_usize()))
    }
    pub fn try_reserve_exact_total(&mut self, cap_idx: I) -> Result<(), TryReserveError>
    where
        I: Idx,
    {
        self.data
            .try_reserve_exact(self.data.len().saturating_sub(cap_idx.into_usize()))
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, cap: I)
    where
        I: Idx,
    {
        self.data.shrink_to(cap.into_usize());
    }

    pub fn into_boxed_slice(self) -> Box<IndexSlice<I, T>> {
        IndexSlice::from_boxed_raw_slice(self.data.into_boxed_slice())
    }

    pub fn into_boxed_raw_slice(self) -> Box<[T]> {
        self.data.into_boxed_slice()
    }

    pub fn truncate(&mut self, len: I)
    where
        I: Idx,
    {
        self.data.truncate(len.into_usize());
    }

    pub const fn as_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_raw_slice(self.data.as_slice())
    }

    pub const fn as_mut_slice(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_mut_raw_slice(self.data.as_mut_slice())
    }

    pub fn as_raw_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_raw_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// # Safety
    /// - `len` must be less than or equal to [`Vec::capacity()`].
    /// - The elements at [`Vec::len()`]..`len` must be initialized.
    /// - `len` must map to a valid usize
    pub unsafe fn set_len(&mut self, len: I)
    where
        I: Idx,
    {
        unsafe {
            self.data.set_len(len.into_usize_unchecked());
        }
    }

    pub fn swap_remove(&mut self, index: I) -> T
    where
        I: Idx,
    {
        self.data.swap_remove(index.into_usize())
    }

    pub fn insert(&mut self, index: I, element: T)
    where
        I: Idx,
    {
        self.data.insert(index.into_usize(), element);
    }

    pub fn remove(&mut self, index: I) -> T
    where
        I: Idx,
    {
        self.data.remove(index.into_usize())
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

    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.data.dedup_by_key(key);
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.data.dedup_by(same_bucket);
    }

    pub fn push(&mut self, v: T) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        // TODO: if we decide to bump MSRV to 1.86, repace with call to `Vec::pop_if`
        let last = self.last_mut()?;
        if predicate(last) {
            self.pop()
        } else {
            None
        }
    }

    pub fn append<V: AsMut<Vec<T>>>(&mut self, mut other: V) {
        self.data.append(other.as_mut());
    }

    pub fn drain<X, R>(&mut self, range: R) -> Drain<'_, T>
    where
        X: IdxCompat<I>,
        R: IndexRangeBounds<X>,
    {
        self.data.drain(range.canonicalize(self.data.len()))
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub const fn len(&self) -> usize {
        // TODO: make this const once that stabilizes
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: make this const once that stabilizes
        self.data.is_empty()
    }

    pub fn split_off(&mut self, at: I) -> IndexVec<I, T>
    where
        I: Idx,
    {
        IndexVec::from(self.data.split_off(at.into_usize()))
    }

    pub fn resize_with<F>(&mut self, new_len: I, f: F)
    where
        I: Idx,
        F: FnMut() -> T,
    {
        self.data.resize_with(new_len.into_usize(), f);
    }

    pub fn leak<'a>(self) -> &'a mut IndexSlice<I, T> {
        self.data.leak().into()
    }

    pub fn spare_capacity_mut(&mut self) -> &mut IndexSlice<I, MaybeUninit<T>> {
        self.data.spare_capacity_mut().into()
    }

    pub fn resize(&mut self, len: I, value: T)
    where
        I: Idx,
        T: Clone,
    {
        self.data.resize(len.into_usize(), value);
    }

    pub fn extend_from_slice<S: AsRef<[T]>>(&mut self, slice: S)
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice.as_ref());
    }

    pub fn extend_from_within<X, R>(&mut self, src: R)
    where
        X: IdxCompat<I>,
        R: IndexRangeBounds<X>,
        T: Clone,
    {
        self.data
            .extend_from_within(src.canonicalize(self.data.len()));
    }
}
impl<I, T, const N: usize> IndexVec<I, [T; N]> {
    pub fn into_flattened(self) -> Vec<T> {
        self.data.into_flattened()
    }
}

impl<I, T> IndexVec<I, T> {
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.data.dedup();
    }

    pub fn splice<X, R, It>(
        &mut self,
        range: R,
        replace_with: It,
    ) -> Splice<'_, <It as IntoIterator>::IntoIter>
    where
        X: IdxCompat<I>,
        R: IndexRangeBounds<X>,
        It: IntoIterator<Item = T>,
    {
        self.data
            .splice(range.canonicalize(self.data.len()), replace_with)
    }

    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn push_get_idx(&mut self, v: T) -> I
    where
        I: Idx,
    {
        let id = self.len_idx();
        self.data.push(v);
        id
    }

    // We have these because the slice deref version takes an offset parameter.
    // TODO: get rid of that.
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data[range.canonicalize(self.data.len())])
    }
    pub fn iter_enumerated_range_mut(
        &mut self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::IterMut<'_, T>>
    where
        I: Idx,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn iter_enumerated_mut(&mut self) -> IndexEnumerate<I, core::slice::IterMut<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, core::slice::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }

    pub fn into_iter_enumerated(self) -> IndexEnumerate<I, alloc::vec::IntoIter<T>>
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

    pub const fn from_vec(v: Vec<T>) -> Self {
        Self {
            data: v,
            _phantom: PhantomData,
        }
    }

    pub const fn into_vec(self) -> Vec<T> {
        // required because this function is const
        let res = unsafe { core::ptr::read(&raw const self.data) };
        core::mem::forget(self);
        res
    }
}

impl<I, T> AsMut<IndexVec<I, T>> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut IndexVec<I, T> {
        self
    }
}
impl<I, T> AsMut<Vec<T>> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.as_mut_vec()
    }
}
impl<I, T> AsMut<[T]> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_raw_slice()
    }
}
impl<I, T> AsMut<IndexSlice<I, T>> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_slice()
    }
}

impl<I, T> AsRef<IndexVec<I, T>> for IndexVec<I, T> {
    fn as_ref(&self) -> &IndexVec<I, T> {
        self
    }
}
impl<I, T> AsRef<Vec<T>> for IndexVec<I, T> {
    fn as_ref(&self) -> &Vec<T> {
        self.as_vec()
    }
}
impl<I, T> AsRef<[T]> for IndexVec<I, T> {
    fn as_ref(&self) -> &[T] {
        self.as_raw_slice()
    }
}
impl<I, T> AsRef<IndexSlice<I, T>> for IndexVec<I, T> {
    fn as_ref(&self) -> &IndexSlice<I, T> {
        self.as_slice()
    }
}

impl<I, T> Borrow<Vec<T>> for IndexVec<I, T> {
    fn borrow(&self) -> &Vec<T> {
        self.as_vec()
    }
}
impl<I, T> Borrow<[T]> for IndexVec<I, T> {
    fn borrow(&self) -> &[T] {
        self.as_raw_slice()
    }
}
impl<I, T> Borrow<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow(&self) -> &IndexSlice<I, T> {
        self.as_slice()
    }
}

impl<I, T> BorrowMut<Vec<T>> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        self.as_mut_vec()
    }
}
impl<I, T> BorrowMut<[T]> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_raw_slice()
    }
}
impl<I, T> BorrowMut<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_slice()
    }
}

impl<I, T> Clone for IndexVec<I, T>
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

impl<I, T: Debug> Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.as_slice(), f)
    }
}

impl<I, T> Default for IndexVec<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I, T> Deref for IndexVec<I, T> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_raw_slice(&self.data)
    }
}
impl<I, T> DerefMut for IndexVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::from_mut_raw_slice(&mut self.data)
    }
}

impl<'a, I, T> Extend<&'a T> for IndexVec<I, T>
where
    T: Copy + 'a,
{
    fn extend<It: IntoIterator<Item = &'a T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}
impl<I, T> Extend<T> for IndexVec<I, T> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}

impl<I, T> From<IndexVec<I, T>> for Vec<T> {
    fn from(value: IndexVec<I, T>) -> Self {
        value.data
    }
}
impl<I, T> From<Vec<T>> for IndexVec<I, T> {
    fn from(value: Vec<T>) -> Self {
        IndexVec {
            data: value,
            _phantom: PhantomData,
        }
    }
}
impl<I, T> From<&[T]> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &[T]) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T, const N: usize> From<&[T; N]> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &[T; N]) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T, const N: usize> From<&IndexArray<I, T, N>> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &IndexArray<I, T, N>) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<'a, I, T> From<&'a IndexVec<I, T>> for Cow<'a, IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &'a IndexVec<I, T>) -> Self {
        Cow::Borrowed(value.as_slice())
    }
}
impl<'a, I, T> From<&'a IndexVec<I, T>> for Cow<'a, [T]>
where
    T: Clone,
{
    fn from(value: &'a IndexVec<I, T>) -> Self {
        Cow::Borrowed(value.as_raw_slice())
    }
}

impl<I, T> From<&mut [T]> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &mut [T]) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T, const N: usize> From<&mut [T; N]> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &mut [T; N]) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T, const N: usize> From<&mut IndexArray<I, T, N>> for IndexVec<I, T>
where
    T: Clone,
{
    fn from(value: &mut IndexArray<I, T, N>) -> Self {
        Self::from(Vec::from(value))
    }
}

impl<I, T, const N: usize> From<[T; N]> for IndexVec<I, T> {
    fn from(value: [T; N]) -> Self {
        IndexVec::from_iter(value)
    }
}
impl<I, T, const N: usize> From<IndexArray<I, T, N>> for IndexVec<I, T> {
    fn from(value: IndexArray<I, T, N>) -> Self {
        IndexVec::from_iter(value)
    }
}

impl<I, T> From<BinaryHeap<T>> for IndexVec<I, T> {
    fn from(value: BinaryHeap<T>) -> Self {
        Self::from(Vec::from(value))
    }
}

impl<I, T> From<Box<[T]>> for IndexVec<I, T> {
    fn from(value: Box<[T]>) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T> From<Box<IndexSlice<I, T>>> for IndexVec<I, T> {
    fn from(value: Box<IndexSlice<I, T>>) -> Self {
        Self::from(Vec::from(value.into_boxed_raw_slice()))
    }
}
impl<'a, I, T> From<Cow<'a, [T]>> for IndexVec<I, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(value: Cow<'a, [T]>) -> Self {
        Self::from(value.into_owned())
    }
}
impl<'a, I, T> From<Cow<'a, IndexSlice<I, T>>> for IndexVec<I, T>
where
    IndexSlice<I, T>: ToOwned<Owned = IndexVec<I, T>>,
{
    fn from(value: Cow<'a, IndexSlice<I, T>>) -> Self {
        value.into_owned()
    }
}

impl<I, T> From<IndexVec<I, T>> for Cow<'_, [T]>
where
    T: Clone,
{
    fn from(value: IndexVec<I, T>) -> Self {
        Cow::from(value.data)
    }
}
impl<I, T> From<IndexVec<I, T>> for Cow<'_, IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: IndexVec<I, T>) -> Self {
        Cow::Owned(value)
    }
}

impl<I, T> From<IndexVec<I, T>> for Arc<[T]> {
    fn from(value: IndexVec<I, T>) -> Self {
        Arc::from(value.data)
    }
}
impl<I, T> From<IndexVec<I, T>> for Arc<IndexSlice<I, T>> {
    fn from(value: IndexVec<I, T>) -> Self {
        let res = Arc::from(value.data);
        // SAFETY: `IndexSlice<I, T>` is `#[repr(transparent)]`
        unsafe { core::mem::transmute::<Arc<[T]>, Arc<IndexSlice<I, T>>>(res) }
    }
}

impl<I, T> From<IndexVec<I, T>> for BinaryHeap<T>
where
    T: Ord,
{
    fn from(value: IndexVec<I, T>) -> Self {
        BinaryHeap::from(value.data)
    }
}

impl<I, T> From<IndexVec<I, T>> for Box<[T]> {
    fn from(value: IndexVec<I, T>) -> Self {
        value.into_boxed_raw_slice()
    }
}
impl<I, T> From<IndexVec<I, T>> for Box<IndexSlice<I, T>> {
    fn from(value: IndexVec<I, T>) -> Self {
        value.into_boxed_slice()
    }
}

impl<I, T> From<IndexVec<I, T>> for Rc<[T]> {
    fn from(value: IndexVec<I, T>) -> Self {
        Rc::from(value.data)
    }
}
impl<I, T> From<IndexVec<I, T>> for Rc<IndexSlice<I, T>> {
    fn from(value: IndexVec<I, T>) -> Self {
        let res = Rc::from(value.data);
        // SAFETY: `IndexSlice<I, T>` is `#[repr(transparent)]`
        unsafe { core::mem::transmute::<Rc<[T]>, Rc<IndexSlice<I, T>>>(res) }
    }
}

impl<I, T> From<IndexVec<I, T>> for VecDeque<T> {
    fn from(value: IndexVec<I, T>) -> Self {
        VecDeque::from(value.data)
    }
}
impl<I, T> From<VecDeque<T>> for IndexVec<I, T> {
    fn from(value: VecDeque<T>) -> Self {
        Self::from(Vec::from(value))
    }
}
impl<I, T> From<IndexVecDeque<I, T>> for IndexVec<I, T> {
    fn from(value: IndexVecDeque<I, T>) -> Self {
        Self::from(Vec::from(value.into_vec_deque()))
    }
}

impl<I, T> FromIterator<T> for IndexVec<I, T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<I, T> Hash for IndexVec<I, T>
where
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<I, S, T> Index<S> for IndexVec<I, T>
where
    S: SequenceIndex<I, IndexSlice<I, T>>,
{
    type Output = S::Output;

    fn index(&self, index: S) -> &Self::Output {
        index.index(self.as_slice())
    }
}

impl<I, S, T> IndexMut<S> for IndexVec<I, T>
where
    S: SequenceIndex<I, IndexSlice<I, T>>,
{
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        index.index_mut(self.as_mut_slice())
    }
}

impl<'a, I, T> IntoIterator for &'a IndexVec<I, T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut IndexVec<I, T> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<I, T> IntoIterator for IndexVec<I, T> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<I, T> Ord for IndexVec<I, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_raw_slice().cmp(other.as_raw_slice())
    }
}

impl<I, T, U> PartialEq<&[U]> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U]) -> bool {
        self.as_raw_slice() == *other
    }
}

impl<I1, I2, T, U> PartialEq<&IndexSlice<I2, U>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&IndexSlice<I2, U>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<&mut [U]> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut [U]) -> bool {
        self.as_raw_slice() == *other
    }
}

impl<I1, I2, T, U> PartialEq<&mut IndexSlice<I2, U>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut IndexSlice<I2, U>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<[U]> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        self.as_raw_slice() == other
    }
}

impl<I1, I2, T, U> PartialEq<IndexSlice<I2, U>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexSlice<I2, U>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U, const N: usize> PartialEq<[U; N]> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; N]) -> bool {
        self.as_raw_slice() == other.as_slice()
    }
}

impl<I1, I2, T, U, const N: usize> PartialEq<IndexArray<I2, U, N>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexArray<I2, U, N>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for &[T]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        *self == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for &mut [T]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        *self == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for [T]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for Cow<'_, [T]>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        *self == other.as_raw_slice()
    }
}

impl<I1, I2, T, U> PartialEq<IndexVec<I2, U>> for Cow<'_, IndexSlice<I1, T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &IndexVec<I2, U>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self == other.as_vec()
    }
}

// std has no corresponding `PartialEq<VecDeque<U, A>> for Vec<T, A>`, but it just makes sense
impl<I, T, U> PartialEq<VecDeque<U>> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &VecDeque<U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I1, I2, T, U> PartialEq<IndexVecDeque<I2, U>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVecDeque<I2, U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<I1, I2, T, U> PartialEq<IndexVec<I2, U>> for IndexVec<I1, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I2, U>) -> bool {
        self.as_raw_slice() == other.as_raw_slice()
    }
}

impl<I, T, U> PartialEq<Vec<U>> for IndexVec<I, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        self.as_raw_slice() == other.as_slice()
    }
}

impl<I, T, U> PartialEq<IndexVec<I, U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self.as_slice() == other.as_raw_slice()
    }
}

// std has no corresponding `PartialEq<Vec<U, A>> for [T; N]`, but it just makes sense
impl<I, T, U, const N: usize> PartialEq<IndexVec<I, U>> for [T; N]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self.as_slice() == other.as_raw_slice()
    }
}

// std has no corresponding `PartialEq<Vec<U, A>> for &[T; N]`, but it just makes sense
impl<I, T, U, const N: usize> PartialEq<IndexVec<I, U>> for &[T; N]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self.as_slice() == other.as_raw_slice()
    }
}

// std has no corresponding `PartialEq<Vec<U, A>> for &mut [T; N]`, but it just makes sense
impl<I, T, U, const N: usize> PartialEq<IndexVec<I, U>> for &mut [T; N]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &IndexVec<I, U>) -> bool {
        self.as_slice() == other.as_raw_slice()
    }
}

impl<I1, I2, T> PartialOrd<IndexVec<I2, T>> for IndexVec<I1, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &IndexVec<I2, T>) -> Option<core::cmp::Ordering> {
        self.as_raw_slice().partial_cmp(other.as_raw_slice())
    }
}

impl<I, T, const N: usize> TryFrom<IndexVec<I, T>> for Box<[T; N]> {
    type Error = IndexVec<I, T>;

    fn try_from(value: IndexVec<I, T>) -> Result<Self, Self::Error> {
        Box::try_from(value.into_vec()).map_err(|v: Vec<T>| v.into())
    }
}

impl<I, T, const N: usize> TryFrom<IndexVec<I, T>> for [T; N] {
    type Error = IndexVec<I, T>;

    fn try_from(value: IndexVec<I, T>) -> Result<Self, Self::Error> {
        <[T; N]>::try_from(value.into_vec()).map_err(|v: Vec<T>| v.into())
    }
}

#[cfg(feature = "std")]
impl<I> std::io::Write for IndexVec<I, u8> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.data.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.data.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.data.write_fmt(fmt)
    }
}

impl<I, T> Eq for IndexVec<I, T> where T: Eq {}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, T> Serialize for IndexVec<I, T>
where
    Vec<T>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T> Deserialize<'de> for IndexVec<I, T>
where
    Vec<T>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(Vec::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod test {
    use indexland::Idx;

    use crate::{IndexSlice, IndexVec};

    #[test]
    fn index() {
        #[derive(Idx)]
        struct Foo(usize);

        let v = index_vec![0, 1, 2, 3];

        assert_eq!(v[Foo(1)], 1);
        assert_eq!(&v[..], IndexSlice::<Foo, _>::from_raw_slice(&[0, 1, 2, 3]));
        assert_eq!(
            &v[Foo(1)..],
            IndexSlice::<Foo, _>::from_raw_slice(&[1, 2, 3])
        );
        assert_eq!(&v[..Foo(2)], IndexSlice::<Foo, _>::from_raw_slice(&[0, 1]));
        assert_eq!(
            &v[..=Foo(2)],
            IndexSlice::<Foo, _>::from_raw_slice(&[0, 1, 2])
        );
        assert_eq!(
            &v[Foo(1)..Foo(3)],
            IndexSlice::<Foo, _>::from_raw_slice(&[1, 2])
        );
        assert_eq!(
            &v[Foo(1)..=Foo(3)],
            IndexSlice::<Foo, _>::from_raw_slice(&[1, 2, 3])
        );
    }

    #[test]
    fn index_compat() {
        #[derive(Idx)]
        struct Foo2(usize);

        #[derive(Idx)]
        #[indexland(idx_compat(Foo2, usize))]
        struct Foo(usize);

        let v: IndexVec<Foo, u32> = index_vec![0, 1, 2, 3];

        assert_eq!(&v[..], IndexSlice::<Foo, _>::from_raw_slice(&[0, 1, 2, 3]));

        assert_eq!(v[Foo2(1)], 1);
        assert_eq!(
            &v[Foo2(1)..],
            IndexSlice::<Foo2, _>::from_raw_slice(&[1, 2, 3])
        );
        assert_eq!(
            &v[..Foo2(2)],
            IndexSlice::<Foo2, _>::from_raw_slice(&[0, 1])
        );
        assert_eq!(
            &v[..=Foo2(2)],
            IndexSlice::<Foo2, _>::from_raw_slice(&[0, 1, 2])
        );
        assert_eq!(
            &v[Foo2(1)..Foo2(3)],
            IndexSlice::<Foo2, _>::from_raw_slice(&[1, 2])
        );
        assert_eq!(
            &v[Foo2(1)..=Foo2(3)],
            IndexSlice::<Foo2, _>::from_raw_slice(&[1, 2, 3])
        );

        assert_eq!(v[1], 1);
        assert_eq!(&v[1..], IndexSlice::<usize, _>::from_raw_slice(&[1, 2, 3]));
        assert_eq!(&v[..2], IndexSlice::<usize, _>::from_raw_slice(&[0, 1]));
        assert_eq!(&v[..=2], IndexSlice::<usize, _>::from_raw_slice(&[0, 1, 2]));
        assert_eq!(&v[1..3], IndexSlice::<usize, _>::from_raw_slice(&[1, 2]));
        assert_eq!(
            &v[1..=3],
            IndexSlice::<usize, _>::from_raw_slice(&[1, 2, 3])
        );
    }
}
