use std::marker::PhantomData;

use arrayvec::ArrayVec;

use super::{index_slice::IndexSlice, indexing_type::IndexingType};

pub struct MultiRefMutHandout<'a, I, T, const CAP: usize = 2> {
    data: &'a mut IndexSlice<I, T>,
    handed_out: ArrayVec<I, CAP>,
}

impl<'a, I: IndexingType, T, const CAP: usize>
    MultiRefMutHandout<'a, I, T, CAP>
{
    pub fn new(data: &'a mut IndexSlice<I, T>) -> Self {
        Self {
            data,
            handed_out: ArrayVec::new(),
        }
    }

    pub fn claim(&mut self, i: I) -> &'a mut T {
        assert!(!self.handed_out.contains(&i));
        self.handed_out.push(i);
        // SAFETY: this type dynamically ensures that each index is handed out
        // exactly once through the assert above
        unsafe {
            std::mem::transmute::<&'_ mut T, &'a mut T>(&mut self.data[i])
        }
    }
}

pub unsafe trait RefHandoutStack<I, T> {
    type Child<'b>: RefHandoutStack<I, T>
    where
        Self: 'b;
    fn claim(&mut self, idx: I) -> (Self::Child<'_>, &mut T);
    fn assert_unused(&mut self, idx: I) -> &mut IndexSlice<I, T>;
}

pub struct RefHandoutStackNode<'a, I, T, P> {
    base: &'a mut P,
    idx: I,
    _phantom: PhantomData<fn() -> T>,
}

pub struct RefHandoutStackBase<'a, I, T> {
    data: &'a mut IndexSlice<I, T>,
}

impl<'a, I, T> RefHandoutStackBase<'a, I, T> {
    pub fn new(data: &'a mut IndexSlice<I, T>) -> Self {
        Self { data }
    }
}

unsafe impl<I: IndexingType, T> RefHandoutStack<I, T>
    for RefHandoutStackBase<'_, I, T>
{
    type Child<'b>
        = RefHandoutStackNode<'b, I, T, Self>
    where
        Self: 'b;

    fn claim<'b>(&'b mut self, idx: I) -> (Self::Child<'b>, &'b mut T) {
        let elem = unsafe {
            std::mem::transmute::<&'_ mut T, &'b mut T>(&mut self.data[idx])
        };
        let child = RefHandoutStackNode {
            base: self,
            idx,
            _phantom: PhantomData,
        };
        (child, elem)
    }

    fn assert_unused(&mut self, _idx: I) -> &mut IndexSlice<I, T> {
        self.data
    }
}

unsafe impl<I: IndexingType, T, P: RefHandoutStack<I, T>> RefHandoutStack<I, T>
    for RefHandoutStackNode<'_, I, T, P>
{
    type Child<'b>
        = RefHandoutStackNode<'b, I, T, Self>
    where
        Self: 'b;

    fn claim<'b>(&'b mut self, idx: I) -> (Self::Child<'b>, &'b mut T) {
        let universe = self.assert_unused(idx);
        let elem = unsafe {
            std::mem::transmute::<&'_ mut T, &'b mut T>(&mut universe[idx])
        };

        let child = RefHandoutStackNode {
            base: self,
            idx,
            _phantom: PhantomData,
        };
        (child, elem)
    }

    fn assert_unused(&mut self, idx: I) -> &mut IndexSlice<I, T> {
        assert!(idx != self.idx);
        self.base.assert_unused(idx)
    }
}
