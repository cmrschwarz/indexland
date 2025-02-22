use std::{
    collections::VecDeque,
    ops::{Index, IndexMut, RangeBounds},
};

use super::{indexing_type::IndexingType, range_bounds_to_range_wrapping};

pub struct OffsetVecDeque<I, T> {
    data: VecDeque<T>,
    offset: I,
}

impl<I: IndexingType, T> Index<I> for OffsetVecDeque<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize().wrapping_sub(self.offset.into_usize())]
    }
}
impl<I: IndexingType, T> IndexMut<I> for OffsetVecDeque<I, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data
            [index.into_usize().wrapping_sub(self.offset.into_usize())]
    }
}
impl<I: IndexingType, T> Default for OffsetVecDeque<I, T> {
    fn default() -> Self {
        Self {
            data: VecDeque::default(),
            offset: I::from_usize(0),
        }
    }
}
impl<I: IndexingType, T: Clone> Clone for OffsetVecDeque<I, T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            offset: self.offset,
        }
    }
}

impl<I: IndexingType, T> OffsetVecDeque<I, T> {
    pub fn index_from_phys(&self, phys_idx: usize) -> I {
        I::from_usize(phys_idx).wrapping_add(self.offset)
    }
    pub fn index_to_phys(&self, index: I) -> usize {
        index.wrapping_sub(self.offset).into_usize()
    }
    pub fn last_used_index(&self) -> I {
        self.next_free_index().wrapping_sub(I::one())
    }
    pub fn next_free_index(&self) -> I {
        self.index_from_phys(self.len())
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn offset(&self) -> I {
        self.offset
    }
    pub fn push_back(&mut self, v: T) {
        self.data.push_back(v);
    }

    pub fn range(
        &self,
        range: impl RangeBounds<I>,
    ) -> impl Iterator<Item = &T> {
        let range =
            range_bounds_to_range_wrapping(range, self.next_free_index());
        let start = self.index_to_phys(range.start);
        let end = self.index_to_phys(range.end);
        self.data.range(start..end)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.data.as_slices()
    }
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.data.as_mut_slices()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.offset = self.offset.wrapping_add(I::one());
        self.data.pop_front()
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn drop_front(&mut self, count: usize) {
        self.offset = self.offset.wrapping_add(I::from_usize(count));
        self.data.drain(0..count);
    }
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    pub fn drop_front_until(&mut self, id: I) {
        self.drop_front(self.index_to_phys(id));
    }
    pub fn drop_back_until(&mut self, id: I) {
        self.truncate(self.index_to_phys(id));
    }

    pub fn get_phys(&self, idx: usize) -> &T {
        &self.data[idx]
    }
    pub fn get_phys_mut(&mut self, idx: usize) -> &mut T {
        &mut self.data[idx]
    }
}
