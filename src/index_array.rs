use ref_cast::RefCast;

use crate::IndexSlice;

use super::idx::Idx;
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

#[derive(ref_cast::RefCast, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexArray<I, T, const SIZE: usize> {
    data: [T; SIZE],
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I, T, const SIZE: usize> Default for IndexArray<I, T, SIZE>
where
    [T; SIZE]: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T, const SIZE: usize> IndexArray<I, T, SIZE> {
    pub const fn new(data: [T; SIZE]) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
    pub fn as_array(&self) -> &[T; SIZE] {
        &self.data
    }
    pub fn as_array_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.data
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::ref_cast(&self.data)
    }
    pub fn as_index_slice_mut(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::ref_cast_mut(&mut self.data)
    }
    pub fn into_array(self) -> [T; SIZE] {
        self.data
    }
}

impl<I: Idx, T, const SIZE: usize> From<[T; SIZE]> for IndexArray<I, T, SIZE> {
    fn from(value: [T; SIZE]) -> Self {
        Self::new(value)
    }
}
impl<I: Idx, T, const SIZE: usize> From<IndexArray<I, T, SIZE>> for [T; SIZE] {
    fn from(value: IndexArray<I, T, SIZE>) -> Self {
        value.data
    }
}

impl<I: Idx, T, const SIZE: usize> Deref for IndexArray<I, T, SIZE> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        self.as_index_slice()
    }
}

impl<I: Idx, T, const SIZE: usize> DerefMut for IndexArray<I, T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_index_slice_mut()
    }
}

impl<I: Idx, T: Debug, const SIZE: usize> Debug for IndexArray<I, T, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T, const SIZE: usize> Index<I> for IndexArray<I, T, SIZE> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T, const SIZE: usize> IndexMut<I> for IndexArray<I, T, SIZE> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<I: Idx, T, const SIZE: usize> IntoIterator for IndexArray<I, T, SIZE> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: Idx, T, const SIZE: usize> IntoIterator
    for &'a IndexArray<I, T, SIZE>
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T, const SIZE: usize> IntoIterator
    for &'a mut IndexArray<I, T, SIZE>
{
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I: Idx, T, const SIZE: usize> Index<Range<I>> for IndexArray<I, T, SIZE> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::ref_cast(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T, const SIZE: usize> IndexMut<Range<I>>
    for IndexArray<I, T, SIZE>
{
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::ref_cast_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T: PartialEq, const SIZE: usize> PartialEq<IndexArray<I, T, SIZE>>
    for [T; SIZE]
{
    fn eq(&self, other: &IndexArray<I, T, SIZE>) -> bool {
        self == &other.data
    }
}

impl<I: Idx, T: PartialEq, const SIZE: usize> PartialEq<[T; SIZE]>
    for IndexArray<I, T, SIZE>
{
    fn eq(&self, other: &[T; SIZE]) -> bool {
        &self.data == other
    }
}

impl<I: Idx, T: PartialEq, const SIZE: usize> PartialEq<IndexArray<I, T, SIZE>>
    for [T]
{
    fn eq(&self, other: &IndexArray<I, T, SIZE>) -> bool {
        self == other.data
    }
}

impl<I: Idx, T: PartialEq, const SIZE: usize> PartialEq<[T]>
    for IndexArray<I, T, SIZE>
{
    fn eq(&self, other: &[T]) -> bool {
        self.data == other
    }
}
