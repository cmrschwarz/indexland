use crate::{index_enumerate::IndexEnumerate, IndexRangeBounds};

use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

use alloc::{boxed::Box, vec::Vec};

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
    ($($anything: tt)*) => {
        $crate::IndexVec::from($crate::__private::alloc::vec![$($anything)*])
    };
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
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
    pub fn truncate(&mut self, end: I)
    where
        I: Idx,
    {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn remove(&mut self, index: I) -> T
    where
        I: Idx,
    {
        self.data.remove(index.into_usize())
    }
    pub fn swap_remove(&mut self, idx: I) -> T
    where
        I: Idx,
    {
        self.data.swap_remove(idx.into_usize())
    }
    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
    pub fn into_boxed_slice(self) -> Box<IndexSlice<I, T>> {
        IndexSlice::from_boxed_slice(self.data.into_boxed_slice())
    }
    pub fn push_get_idx(&mut self, v: T) -> I
    where
        I: Idx,
    {
        let id = self.len_idx();
        self.data.push(v);
        id
    }
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::Iter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(
            I::ZERO,
            &self.data[range.canonicalize(self.len())],
        )
    }
    pub fn iter_enumerated_range_mut(
        &mut self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::IterMut<T>>
    where
        I: Idx,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexEnumerate<I, core::slice::IterMut<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, core::slice::Iter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, alloc::vec::IntoIter<T>>
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
    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(&self.data)
    }
    pub fn as_mut_index_slice(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_mut_slice(&mut self.data)
    }
}

impl<I, T> AsMut<IndexSlice<I, T>> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}
impl<I, T> AsMut<[T]> for IndexVec<I, T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<I, T> AsRef<IndexSlice<I, T>> for IndexVec<I, T> {
    fn as_ref(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}
impl<I, T> AsRef<[T]> for IndexVec<I, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<I, T> Borrow<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}
impl<I, T> Borrow<[T]> for IndexVec<I, T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}
impl<I, T> BorrowMut<IndexSlice<I, T>> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}
impl<I, T> BorrowMut<[T]> for IndexVec<I, T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<I, T> Deref for IndexVec<I, T> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_slice(&self.data)
    }
}
impl<I, T> DerefMut for IndexVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::from_mut_slice(&mut self.data)
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

impl<I, T: Debug> Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T> Extend<T> for IndexVec<I, T> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}

impl<I, T> IntoIterator for IndexVec<I, T> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
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

impl<I, T> FromIterator<T> for IndexVec<I, T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<IndexVec<I, T>> for [T; N] {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<[T; N]> for IndexVec<I, T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq> PartialEq<IndexSlice<I, T>> for IndexVec<I, T> {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq> PartialEq<IndexVec<I, T>> for IndexSlice<I, T> {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq> PartialEq<IndexVec<I, T>> for [T] {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self == other.as_slice()
    }
}

impl<I, T: PartialEq> PartialEq<[T]> for IndexVec<I, T> {
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
        IndexSlice::from_mut_slice(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

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
