use crate::enumerated_index_iter::EnumeratedIndexIter;

use super::Idx;

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    data: [T],
}

impl<I: Idx, T> IndexSlice<I, T> {
    #[inline]
    pub fn from_slice(s: &[T]) -> &Self {
        unsafe { &*(std::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    pub fn from_slice_mut(s: &mut [T]) -> &mut Self {
        unsafe { &mut *(std::ptr::from_mut(s) as *mut Self) }
    }
    pub fn from_boxed_slice(slice_box: Box<[T]>) -> Box<Self> {
        unsafe {
            let base_box_raw = Box::into_raw(slice_box);
            let index_box_raw = IndexSlice::from_slice_mut(&mut *base_box_raw)
                as *mut IndexSlice<I, T>;
            Box::from_raw(index_box_raw)
        }
    }
    pub fn iter_enumerated(
        &self,
        initial_offset: I,
    ) -> EnumeratedIndexIter<I, std::slice::Iter<T>> {
        EnumeratedIndexIter::new(initial_offset, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
        initial_offset: I,
    ) -> EnumeratedIndexIter<I, std::slice::IterMut<T>> {
        EnumeratedIndexIter::new(initial_offset, &mut self.data)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub fn len_idx(&self) -> I {
        I::from_usize(self.data.len())
    }
    pub fn last_idx(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::from_usize)
    }

    pub fn get(&self, idx: I) -> Option<&T> {
        self.data.get(idx.into_usize())
    }
    pub fn get_mut(&mut self, idx: I) -> Option<&mut T> {
        self.data.get_mut(idx.into_usize())
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    pub fn split_at_mut(
        &mut self,
        idx: I,
    ) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>) {
        let (l, r) = self.data.split_at_mut(idx.into_usize());
        (IndexSlice::from_slice_mut(l), IndexSlice::from_slice_mut(r))
    }
}

impl<I: Idx, T: Debug> Debug for IndexSlice<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T> Index<I> for IndexSlice<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexSlice<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a IndexSlice<I, T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut IndexSlice<I, T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I: Idx, T> Index<Range<I>> for IndexSlice<I, T> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::from_slice(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T> IndexMut<Range<I>> for IndexSlice<I, T> {
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::from_slice_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<IndexSlice<I, T>>
    for [T; N]
{
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == &other.data
    }
}

impl<I: Idx, T: PartialEq, const N: usize> PartialEq<[T; N]>
    for IndexSlice<I, T>
{
    fn eq(&self, other: &[T; N]) -> bool {
        &self.data == other.as_slice()
    }
}

impl<I: Idx, T: PartialEq> PartialEq<IndexSlice<I, T>> for [T] {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self == &other.data
    }
}

impl<I: Idx, T: PartialEq> PartialEq<[T]> for IndexSlice<I, T> {
    fn eq(&self, other: &[T]) -> bool {
        &self.data == other
    }
}

impl<'a, I: Idx, T> From<&'a IndexSlice<I, T>> for &'a [T] {
    fn from(value: &'a IndexSlice<I, T>) -> Self {
        value.as_slice()
    }
}
impl<'a, I: Idx, T> From<&'a mut IndexSlice<I, T>> for &'a mut [T] {
    fn from(value: &'a mut IndexSlice<I, T>) -> Self {
        value.as_slice_mut()
    }
}

impl<'a, I: Idx, T> From<&'a [T]> for &'a IndexSlice<I, T> {
    fn from(value: &'a [T]) -> Self {
        IndexSlice::from_slice(value)
    }
}
impl<'a, I: Idx, T> From<&'a mut [T]> for &'a mut IndexSlice<I, T> {
    fn from(value: &'a mut [T]) -> Self {
        IndexSlice::from_slice_mut(value)
    }
}

use crate::idx::RangeBoundsAsRange;

macro_rules! slice_index_impl {
    ($($range_type: ident),*) => {$(
        impl<I: Idx, T> Index<$range_type<I>> for IndexSlice<I, T> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::from_slice(&self.data[rb.as_usize_range(self.len())])
            }
        }

        impl<I: Idx, T> IndexMut<$range_type<I>> for IndexSlice<I, T> {
            #[inline]
            fn index_mut(&mut self, rb: $range_type<I>) -> &mut Self::Output {
                let range = rb.as_usize_range(self.len());
                IndexSlice::from_slice_mut(&mut self.data[range])
            }
        }
    )*};
}

slice_index_impl!(RangeInclusive, RangeFrom, RangeTo, RangeToInclusive);
