use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{idx::Idx, index_slice::IndexSlice};
use crate::{index_enumerate::IndexEnumerate, IndexRange, IndexRangeBounds};

use smallvec::SmallVec;

/// Create an [`IndexSmallVec`] containing the arguments.
///
/// The syntax is identical to [`smallvec!`](::smallvec::smallvec!).
///
/// The index type and capacity cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// ## Example
/// ```
/// use indexland::{index_small_vec, IndexSmallVec};
///
/// let v: IndexSmallVec<u32, _, 3> = index_small_vec![-1, 2, 3];
/// ```
#[macro_export]
macro_rules! index_small_vec {
    ($($anything: tt)+) => {
        $crate::IndexSmallVec::from(::smallvec::smallvec![$($anything)+])
    };
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexSmallVec<I, T, const CAP: usize> {
    data: SmallVec<[T; CAP]>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I: Idx, T, const CAP: usize> Deref for IndexSmallVec<I, T, CAP> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_slice(&self.data)
    }
}
impl<I: Idx, T, const CAP: usize> DerefMut for IndexSmallVec<I, T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::from_slice_mut(&mut self.data)
    }
}

impl<I, T, const CAP: usize> From<SmallVec<[T; CAP]>>
    for IndexSmallVec<I, T, CAP>
{
    fn from(v: SmallVec<[T; CAP]>) -> Self {
        IndexSmallVec {
            data: v,
            _phantom: PhantomData,
        }
    }
}

impl<I, T, const CAP: usize> From<IndexSmallVec<I, T, CAP>>
    for SmallVec<[T; CAP]>
{
    fn from(value: IndexSmallVec<I, T, CAP>) -> Self {
        value.data
    }
}

impl<I, T, const CAP: usize> Default for IndexSmallVec<I, T, CAP> {
    fn default() -> Self {
        Self {
            data: SmallVec::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T: Debug, const CAP: usize> Debug for IndexSmallVec<I, T, CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T, const CAP: usize> IndexSmallVec<I, T, CAP> {
    pub const fn new() -> Self {
        Self {
            data: SmallVec::new_const(),
            _phantom: PhantomData,
        }
    }
    pub fn swap_remove(&mut self, idx: I) -> T {
        self.data.swap_remove(idx.into_usize())
    }
    pub fn reserve(&mut self, additional: I) {
        self.data.reserve(additional.into_usize());
    }
    pub fn reserve_len(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.reserve(slice.len());

        let mut ptr = unsafe { self.data.as_mut_ptr().add(self.data.len()) };

        // the compiler should replace this with `memcopy` if `T: Copy`
        for v in slice {
            unsafe {
                *ptr = v.clone();
                ptr = ptr.add(1);
                // to avoid leaks in case of panic unwinding
                self.data.set_len(self.data.len() + 1);
            }
        }
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
    pub fn as_array_vec(&self) -> &SmallVec<[T; CAP]> {
        &self.data
    }
    pub fn as_array_vec_mut(&mut self) -> &mut SmallVec<[T; CAP]> {
        &mut self.data
    }
    pub fn into_array_vec(self) -> SmallVec<[T; CAP]> {
        self.data
    }
    pub fn push_get_id(&mut self, v: T) -> I {
        let id = self.len_idx();
        self.data.push(v);
        id
    }
    pub fn truncate(&mut self, end: I) {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, std::slice::Iter<T>> {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexEnumerate<I, std::slice::IterMut<T>> {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::Iter<T>> {
        IndexEnumerate::new(
            I::ZERO,
            &self.data[range.canonicalize(self.len())],
        )
    }
    pub fn iter_enumerated_range_mut(
        &mut self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, core::slice::IterMut<T>> {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, smallvec::IntoIter<[T; CAP]>> {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn indices(&self) -> IndexRange<I> {
        IndexRange::new(I::ZERO..self.len_idx())
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

impl<I, T, const CAP: usize> Extend<T> for IndexSmallVec<I, T, CAP> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.data.extend(iter);
    }
}

impl<I: Idx, T, const CAP: usize> IntoIterator for IndexSmallVec<I, T, CAP> {
    type Item = T;

    type IntoIter = smallvec::IntoIter<[T; CAP]>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: Idx, T, const CAP: usize> IntoIterator
    for &'a IndexSmallVec<I, T, CAP>
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T, const CAP: usize> IntoIterator
    for &'a mut IndexSmallVec<I, T, CAP>
{
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I, T, const CAP: usize> FromIterator<T> for IndexSmallVec<I, T, CAP> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(SmallVec::from_iter(iter))
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize, const N: usize>
    PartialEq<IndexSmallVec<I, T, CAP>> for [T; N]
{
    fn eq(&self, other: &IndexSmallVec<I, T, CAP>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize, const N: usize> PartialEq<[T; N]>
    for IndexSmallVec<I, T, CAP>
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize> PartialEq<IndexSlice<I, T>>
    for IndexSmallVec<I, T, CAP>
{
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize>
    PartialEq<IndexSmallVec<I, T, CAP>> for IndexSlice<I, T>
{
    fn eq(&self, other: &IndexSmallVec<I, T, CAP>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize>
    PartialEq<IndexSmallVec<I, T, CAP>> for [T]
{
    fn eq(&self, other: &IndexSmallVec<I, T, CAP>) -> bool {
        self == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq, const CAP: usize> PartialEq<[T]>
    for IndexSmallVec<I, T, CAP>
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I: Idx, T, const CAP: usize> Serialize for IndexSmallVec<I, T, CAP>
where
    SmallVec<[T; CAP]>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I: Idx, T, const CAP: usize> Deserialize<'de>
    for IndexSmallVec<I, T, CAP>
where
    SmallVec<[T; CAP]>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(SmallVec::deserialize(deserializer)?))
    }
}
