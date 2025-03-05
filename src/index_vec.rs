use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeInclusive,
        RangeTo, RangeToInclusive,
    },
};

use crate::enumerated_index_iter::EnumeratedIndexIter;

use super::{
    idx::{Idx, IdxRange},
    index_slice::IndexSlice,
};

#[macro_export]
macro_rules! index_vec {
    ($($anything: tt)+) => {
        IndexVec::from(vec![$($anything)+])
    };
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexVec<I, T> {
    data: Vec<T>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I: Idx, T> Deref for IndexVec<I, T> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_slice(&self.data)
    }
}
impl<I: Idx, T> DerefMut for IndexVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::from_slice_mut(&mut self.data)
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

impl<I, T> From<IndexVec<I, T>> for Vec<T> {
    fn from(value: IndexVec<I, T>) -> Self {
        value.data
    }
}

impl<I, T> Default for IndexVec<I, T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T: Debug> Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T> IndexVec<I, T> {
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
    pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
        self.data.extend(iter);
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
    }
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn push(&mut self, v: T) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T) {
        self.data.resize_with(new_len, f);
    }
    pub fn truncate(&mut self, new_end_index: I) {
        self.data.truncate(new_end_index.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn swap_remove(&mut self, idx: I) -> T {
        self.data.swap_remove(idx.into_usize())
    }
    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }
    pub fn as_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
    pub fn into_boxed_slice(self) -> Box<IndexSlice<I, T>> {
        IndexSlice::from_boxed_slice(self.data.into_boxed_slice())
    }
    pub fn push_get_id(&mut self, v: T) -> I {
        let id = self.len_idx();
        self.data.push(v);
        id
    }

    pub fn iter_enumerated(
        &self,
    ) -> EnumeratedIndexIter<I, std::slice::Iter<T>> {
        EnumeratedIndexIter::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> EnumeratedIndexIter<I, std::slice::IterMut<T>> {
        EnumeratedIndexIter::new(I::ZERO, &mut self.data)
    }
    pub fn into_iter_enumerated(
        self,
    ) -> EnumeratedIndexIter<I, std::vec::IntoIter<T>> {
        EnumeratedIndexIter::new(I::ZERO, self.data)
    }
    pub fn indices(&self) -> IdxRange<I> {
        IdxRange::new(I::ZERO..self.len_idx())
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(&self.data)
    }
    pub fn as_index_slice_mut(&mut self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice_mut(&mut self.data)
    }
}

impl<I: Idx, T> IntoIterator for IndexVec<I, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a IndexVec<I, T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut IndexVec<I, T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<I, T> FromIterator<T> for IndexVec<I, T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<IndexVec<I, T>>
    for [T; N]
{
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<[T; N]>
    for IndexVec<I, T>
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<IndexSlice<I, T>> for IndexVec<I, T> {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<IndexVec<I, T>> for IndexSlice<I, T> {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<IndexVec<I, T>> for [T] {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<[T]> for IndexVec<I, T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexVec<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<I: Idx, T> Index<Range<I>> for IndexVec<I, T> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::from_slice(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T> IndexMut<Range<I>> for IndexVec<I, T> {
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::from_slice_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

use crate::idx::RangeBoundsAsRange;
macro_rules! slice_index_impl {
    ($($range_type: ident),+) => {$(
        impl<I: Idx, T> Index<$range_type<I>> for IndexVec<I, T> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::from_slice(&self.data[rb.as_usize_range(self.len())])
            }
        }

        impl<I: Idx, T> IndexMut<$range_type<I>> for IndexVec<I, T> {
            #[inline]
            fn index_mut(&mut self, rb: $range_type<I>) -> &mut Self::Output {
                let range = rb.as_usize_range(self.len());
                IndexSlice::from_slice_mut(&mut self.data[range])
            }
        }
    )*};
}
slice_index_impl!(RangeInclusive, RangeFrom, RangeTo, RangeToInclusive);

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I: Idx, T> Serialize for IndexVec<I, T>
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
impl<'de, I: Idx, T> Deserialize<'de> for IndexVec<I, T>
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
