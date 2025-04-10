use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use alloc::{collections::VecDeque, vec::Vec};

use crate::{index_enumerate::IndexEnumerate, IndexArray, IndexRangeBounds, IndexVec};

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
        $crate::IndexVecDeque::from([ $value; $count])
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexVecDeque<I, T> {
    data: VecDeque<T>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I, T> From<Vec<T>> for IndexVecDeque<I, T> {
    fn from(value: Vec<T>) -> Self {
        IndexVecDeque {
            data: VecDeque::from(value),
            _phantom: PhantomData,
        }
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

impl<I, T> From<IndexVecDeque<I, T>> for VecDeque<T> {
    fn from(value: IndexVecDeque<I, T>) -> Self {
        value.data
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

impl<I, T: Debug> Debug for IndexVecDeque<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
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
    pub fn reserve(&mut self, additional: I)
    where
        I: Idx,
    {
        self.data.reserve(additional.into_usize());
    }
    pub fn reserve_len(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn push_front(&mut self, v: T) {
        self.data.push_front(v);
    }
    pub fn push_back(&mut self, v: T) {
        self.data.push_back(v);
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T) {
        self.data.resize_with(new_len, f);
    }
    pub fn truncate(&mut self, end: I)
    where
        I: Idx,
    {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
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
    pub fn as_vec_deque(&self) -> &VecDeque<T> {
        &self.data
    }
    pub fn as_mut_vec_deque(&mut self) -> &mut VecDeque<T> {
        &mut self.data
    }
    pub fn push_back_get_id(&mut self, v: T) -> I
    where
        I: Idx,
    {
        let id = self.len_idx();
        self.data.push_back(v);
        id
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, alloc::collections::vec_deque::Iter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::IterMut<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::Iter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.range(range.canonicalize(self.len())))
    }
    pub fn iter_enumerated_range_mut(
        &mut self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, alloc::collections::vec_deque::IterMut<T>>
    where
        I: Idx,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, self.data.range_mut(range))
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
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.data.as_slices()
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.data.as_mut_slices()
    }
    pub fn as_index_slices(&self) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
        let (s1, s2) = self.data.as_slices();
        (IndexSlice::from_slice(s1), IndexSlice::from_slice(s2))
    }
    pub fn as_mut_index_slices(&mut self) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
        let (s1, s2) = self.data.as_mut_slices();
        (IndexSlice::from_slice(s1), IndexSlice::from_slice(s2))
    }
    pub fn iter(&self) -> alloc::collections::vec_deque::Iter<T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> alloc::collections::vec_deque::IterMut<T> {
        self.data.iter_mut()
    }
}

impl<I, T> Extend<T> for IndexVecDeque<I, T> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}

impl<I, T> IntoIterator for IndexVecDeque<I, T> {
    type Item = T;

    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
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

impl<I, T> FromIterator<T> for IndexVecDeque<I, T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(VecDeque::from_iter(iter))
    }
}

impl<I: Idx, T> Index<I> for IndexVecDeque<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexVecDeque<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, T> Serialize for IndexVecDeque<I, T>
where
    VecDeque<T>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T> Deserialize<'de> for IndexVecDeque<I, T>
where
    VecDeque<T>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(VecDeque::deserialize(deserializer)?))
    }
}
