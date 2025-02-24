use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use super::{
    index_slice::IndexSlice, index_vec::IndexVec, Idx,
};

/// Very crude generalization over `Vec<T>` and `VecDeque<T>`,
/// used for cases where algorithms need to work on both, like
/// `merge_action_lists`
pub trait RandomAccessContainer<I: Idx, T>:
    Index<I, Output = T> + IndexMut<I, Output = T>
{
    fn get(&self, index: I) -> Option<&T>;
    fn last_mut(&mut self) -> Option<&mut T>;
    fn push(&mut self, v: T);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<I: Idx, T> RandomAccessContainer<I, T> for IndexVec<I, T> {
    fn get(&self, index: I) -> Option<&T> {
        <IndexSlice<I, T>>::get(self, index)
    }
    fn last_mut(&mut self) -> Option<&mut T> {
        <IndexSlice<I, T>>::last_mut(self)
    }
    fn push(&mut self, v: T) {
        <IndexVec<I, T>>::push(self, v);
    }
    fn len(&self) -> usize {
        <IndexSlice<I, T>>::len(self)
    }
}
impl<T> RandomAccessContainer<usize, T> for Vec<T> {
    fn get(&self, index: usize) -> Option<&T> {
        <[T]>::get(self, index)
    }
    fn last_mut(&mut self) -> Option<&mut T> {
        <[T]>::last_mut(self)
    }
    fn push(&mut self, v: T) {
        <Vec<_>>::push(self, v);
    }
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}
impl<T> RandomAccessContainer<usize, T> for VecDeque<T> {
    fn get(&self, index: usize) -> Option<&T> {
        <VecDeque<_>>::get(self, index)
    }
    fn last_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }
    fn push(&mut self, v: T) {
        <VecDeque<_>>::push_back(self, v);
    }
    fn len(&self) -> usize {
        <VecDeque<_>>::len(self)
    }
}
