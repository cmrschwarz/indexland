use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use alloc::{collections::VecDeque, vec::Vec};

use crate::{idx_enumerate::IdxEnumerate, IndexVec};

use super::{idx::Idx, idx_range::IdxRange, index_slice::IndexSlice};

/// Create an [`IndexVecDeque`] containing the arguments.
///
/// The syntax is identical to [`vec!`](alloc::vec!).
#[macro_export]
macro_rules! index_vec_deque {
    ($($anything: tt)+) => {
        $crate::IndexVecDeque::from(::alloc::vec![$($anything)+])
    };
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

impl<I: Idx, T: Debug> Debug for IndexVecDeque<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T> IndexVecDeque<I, T> {
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
    pub fn len_idx(&self) -> I {
        I::from_usize(self.data.len())
    }
    pub fn last_idx(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub fn reserve(&mut self, additional: I) {
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
    pub fn truncate(&mut self, end: I) {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn swap_remove_front(&mut self, idx: I) -> Option<T> {
        self.data.swap_remove_front(idx.into_usize())
    }
    pub fn swap_remove_back(&mut self, idx: I) -> Option<T> {
        self.data.swap_remove_back(idx.into_usize())
    }
    pub fn as_vec_deque(&self) -> &VecDeque<T> {
        &self.data
    }
    pub fn as_vec_deque_mut(&mut self) -> &mut VecDeque<T> {
        &mut self.data
    }
    pub fn push_back_get_id(&mut self, v: T) -> I {
        let id = self.len_idx();
        self.data.push_back(v);
        id
    }
    pub fn iter_enumerated(
        &self,
    ) -> IdxEnumerate<I, alloc::collections::vec_deque::Iter<T>> {
        IdxEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IdxEnumerate<I, alloc::collections::vec_deque::IterMut<T>> {
        IdxEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IdxEnumerate<I, alloc::collections::vec_deque::IntoIter<T>> {
        IdxEnumerate::new(I::ZERO, self.data)
    }
    pub fn indices(&self) -> IdxRange<I> {
        IdxRange::new(I::ZERO..self.len_idx())
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.data.as_slices()
    }
    pub fn as_slices_mut(&mut self) -> (&mut [T], &mut [T]) {
        self.data.as_mut_slices()
    }
    pub fn as_index_slices(&self) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
        let (s1, s2) = self.data.as_slices();
        (IndexSlice::from_slice(s1), IndexSlice::from_slice(s2))
    }
    pub fn as_index_slices_mut(
        &mut self,
    ) -> (&IndexSlice<I, T>, &IndexSlice<I, T>) {
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

impl<I: Idx, T> IntoIterator for IndexVecDeque<I, T> {
    type Item = T;

    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a IndexVecDeque<I, T> {
    type Item = &'a T;

    type IntoIter = alloc::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut IndexVecDeque<I, T> {
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
impl<I: Idx, T> Serialize for IndexVecDeque<I, T>
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
impl<'de, I: Idx, T> Deserialize<'de> for IndexVecDeque<I, T>
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
