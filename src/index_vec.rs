use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeInclusive,
        RangeTo, RangeToInclusive,
    },
};

use ref_cast::RefCast;

use super::{
    index_slice::{IndexIterEnumerated, IndexSlice},
    indexing_type::{IndexingType, IndexingTypeRange},
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

impl<I, T> Deref for IndexVec<I, T> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::ref_cast(&*self.data)
    }
}
impl<I, T> DerefMut for IndexVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::ref_cast_mut(&mut *self.data)
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

impl<I: IndexingType, T: Debug> Debug for IndexVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: IndexingType, T> IndexVec<I, T> {
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
        self.data.extend(iter)
    }
    pub fn push(&mut self, v: T) {
        self.data.push(v)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T) {
        self.data.resize_with(new_len, f)
    }

    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
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
        let id = self.next_idx();
        self.data.push(v);
        id
    }
    pub fn truncate(&mut self, new_end_index: I) {
        self.data.truncate(new_end_index.into_usize());
    }
    pub fn iter_enumerated(
        &self,
    ) -> IndexIterEnumerated<I, std::slice::Iter<T>> {
        IndexIterEnumerated::new(I::ZERO, self.data.iter())
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexIterEnumerated<I, std::slice::IterMut<T>> {
        IndexIterEnumerated::new(I::ZERO, self.data.iter_mut())
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexIterEnumerated<I, std::vec::IntoIter<T>> {
        IndexIterEnumerated::new(I::ZERO, self.data.into_iter())
    }
    pub fn indices(&self) -> IndexingTypeRange<I> {
        IndexingTypeRange::new(I::zero()..self.next_idx())
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl<I: IndexingType, T> IntoIterator for IndexVec<I, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: IndexingType, T> IntoIterator for &'a IndexVec<I, T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: IndexingType, T> IntoIterator for &'a mut IndexVec<I, T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I, T> FromIterator<T> for IndexVec<I, T> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<I: IndexingType, T: PartialEq, const N: usize> PartialEq<IndexVec<I, T>>
    for [T; N]
{
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: IndexingType, T: PartialEq, const N: usize> PartialEq<[T; N]>
    for IndexVec<I, T>
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: IndexingType, T: PartialEq> PartialEq<IndexSlice<I, T>>
    for IndexVec<I, T>
{
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: IndexingType, T: PartialEq> PartialEq<IndexVec<I, T>>
    for IndexSlice<I, T>
{
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I: IndexingType, T: PartialEq> PartialEq<IndexVec<I, T>> for [T] {
    fn eq(&self, other: &IndexVec<I, T>) -> bool {
        self == other.as_slice()
    }
}

impl<I: IndexingType, T: PartialEq> PartialEq<[T]> for IndexVec<I, T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<I: IndexingType, T> Index<I> for IndexVec<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: IndexingType, T> IndexMut<I> for IndexVec<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<I: IndexingType, T> Index<Range<I>> for IndexVec<I, T> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::ref_cast(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: IndexingType, T> IndexMut<Range<I>> for IndexVec<I, T> {
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::ref_cast_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

macro_rules! slice_index_impl {
    ($range_type: ident) => {
        impl<I: IndexingType, T> Index<$range_type<I>> for IndexVec<I, T> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::ref_cast(&self.data[$crate::range_bounds_to_range_usize(rb, self.len())])
            }
        }

        impl<I: IndexingType, T> IndexMut<$range_type<I>> for IndexVec<I, T> {
            #[inline]
            fn index_mut(&mut self, rb: $range_type<I>) -> &mut Self::Output {
                let range = $crate::range_bounds_to_range_usize(rb, self.len());
                IndexSlice::ref_cast_mut(&mut self.data[range])
            }
        }
    };
    ($($range_types: ident),+) => {
        $( slice_index_impl!($range_types); ) *
    };
}

slice_index_impl!(RangeInclusive, RangeFrom, RangeTo, RangeToInclusive);
