use super::{
    get_two_distinct_mut,
    idx::Idx,
    multi_ref_mut_handout::{MultiRefMutHandout, RefHandoutStackBase},
};
use ref_cast::RefCast;
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

#[derive(ref_cast::RefCast, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    data: [T],
}

pub struct IndexIterEnumerated<I, IT> {
    pos: I,
    base_iter: IT,
}

impl<I: Idx, IT: Iterator> IndexIterEnumerated<I, IT> {
    pub fn new(pos: I, base_iter: IT) -> Self {
        Self { pos, base_iter }
    }
}

impl<I: Idx, IT: Iterator> Iterator for IndexIterEnumerated<I, IT> {
    type Item = (I, IT::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.base_iter.next()?;
        let idx = self.pos;
        self.pos = I::from_usize(self.pos.into_usize() + 1);
        Some((idx, value))
    }
}

impl<I: Idx, T> IndexSlice<I, T> {
    pub fn iter_enumerated(
        &self,
        initial_offset: I,
    ) -> IndexIterEnumerated<I, std::slice::Iter<T>> {
        IndexIterEnumerated {
            base_iter: self.data.iter(),
            pos: initial_offset,
        }
    }
    pub fn iter_enumerated_mut(
        &mut self,
        initial_offset: I,
    ) -> IndexIterEnumerated<I, std::slice::IterMut<T>> {
        IndexIterEnumerated {
            base_iter: self.data.iter_mut(),
            pos: initial_offset,
        }
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
    pub fn from_boxed_slice(slice_box: Box<[T]>) -> Box<Self> {
        unsafe {
            let base_box_raw = Box::into_raw(slice_box);
            let index_box_raw = IndexSlice::ref_cast_mut(&mut *base_box_raw)
                as *mut IndexSlice<I, T>;
            Box::from_raw(index_box_raw)
        }
    }
    pub fn next_idx(&self) -> I {
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
    pub fn two_distinct_mut(&mut self, a: I, b: I) -> (&mut T, &mut T) {
        get_two_distinct_mut(&mut self.data, a.into_usize(), b.into_usize())
    }
    pub fn multi_ref_mut_handout<const CAP: usize>(
        &mut self,
    ) -> MultiRefMutHandout<I, T, CAP> {
        MultiRefMutHandout::new(self)
    }
    pub fn ref_handout_stack(&mut self) -> RefHandoutStackBase<I, T> {
        RefHandoutStackBase::new(self)
    }

    pub fn split_at_mut(
        &mut self,
        idx: I,
    ) -> (&mut IndexSlice<I, T>, &mut IndexSlice<I, T>) {
        let (l, r) = self.data.split_at_mut(idx.into_usize());
        (IndexSlice::ref_cast_mut(l), IndexSlice::ref_cast_mut(r))
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
        IndexSlice::ref_cast(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T> IndexMut<Range<I>> for IndexSlice<I, T> {
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::ref_cast_mut(
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

macro_rules! slice_index_impl {
    ($range_type: ident) => {
        impl<I: Idx, T> Index<$range_type<I>> for IndexSlice<I, T> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::ref_cast(&self.data[$crate::range_bounds_to_range_usize(rb, self.len())])
            }
        }

        impl<I: Idx, T> IndexMut<$range_type<I>> for IndexSlice<I, T> {
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
