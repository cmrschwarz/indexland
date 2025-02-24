use std::{marker::PhantomData, ptr::NonNull};

use arrayvec::ArrayVec;

use crate::{
    universe::{Universe, UniverseEntry},
    Idx,
};

pub struct UniverseMultiRefMutHandout<'a, I, T, const CAP: usize> {
    pub(crate) universe_data: NonNull<UniverseEntry<I, T>>,
    pub(crate) first_vacant_entry: &'a mut Option<I>,
    pub(crate) len: usize,
    pub(crate) handed_out: ArrayVec<I, CAP>,
}

pub unsafe trait UniverseRefHandoutStack<I, T> {
    type Child<'b>: UniverseRefHandoutStack<I, T>
    where
        Self: 'b;
    fn claim(&mut self, idx: I) -> (Self::Child<'_>, &mut T);
    fn assert_unused(&mut self, idx: I) -> NonNull<UniverseEntry<I, T>>;
}

pub struct UniverseRefHandoutStackNode<'a, I, T, P> {
    base: &'a mut P,
    id: I,
    _phantom: PhantomData<fn() -> T>,
}

pub struct UniverseRefHandoutStackBase<'a, I, T> {
    pub(crate) universe_data: NonNull<UniverseEntry<I, T>>,
    pub(crate) len: usize,
    pub(crate) _phantom: PhantomData<&'a mut Universe<I, T>>,
}
impl<'a, I, T> UniverseRefHandoutStackBase<'a, I, T> {
    pub(crate) fn new(universe: &'a mut Universe<I, T>) -> Self {
        Self {
            len: universe.data.len(),
            universe_data: NonNull::new(universe.data.as_mut_ptr()).unwrap(),
            _phantom: PhantomData,
        }
    }
}

unsafe impl<I: Idx, T> UniverseRefHandoutStack<I, T>
    for UniverseRefHandoutStackBase<'_, I, T>
{
    type Child<'b>
        = UniverseRefHandoutStackNode<'b, I, T, Self>
    where
        Self: 'b;

    fn claim(&mut self, id: I) -> (Self::Child<'_>, &mut T) {
        let idx = id.into_usize();
        assert!(idx < self.len);
        let elem = unsafe { &mut *self.universe_data.as_ptr().add(idx) };
        let UniverseEntry::Occupied(elem) = elem else {
            panic!("attempted to claim vacant universe entry")
        };
        let child = UniverseRefHandoutStackNode {
            base: self,
            id,
            _phantom: PhantomData,
        };
        (child, elem)
    }

    fn assert_unused(&mut self, id: I) -> NonNull<UniverseEntry<I, T>> {
        assert!(id.into_usize() < self.len);
        self.universe_data
    }
}

unsafe impl<I: Idx, T, P: UniverseRefHandoutStack<I, T>>
    UniverseRefHandoutStack<I, T>
    for UniverseRefHandoutStackNode<'_, I, T, P>
{
    type Child<'b>
        = UniverseRefHandoutStackNode<'b, I, T, Self>
    where
        Self: 'b;

    fn claim(&mut self, id: I) -> (Self::Child<'_>, &mut T) {
        let idx = id.into_usize();
        let universe_data = self.assert_unused(id);
        let elem = unsafe { &mut *universe_data.as_ptr().add(idx) };
        let UniverseEntry::Occupied(elem) = elem else {
            panic!("attempted to claim vacant universe entry")
        };

        let child = UniverseRefHandoutStackNode {
            base: self,
            id,
            _phantom: PhantomData,
        };
        (child, elem)
    }

    fn assert_unused(&mut self, idx: I) -> NonNull<UniverseEntry<I, T>> {
        assert!(idx != self.id);
        self.base.assert_unused(idx)
    }
}

impl<'a, I: Idx, T, const CAP: usize>
    UniverseMultiRefMutHandout<'a, I, T, CAP>
{
    pub fn new(universe: &'a mut Universe<I, T>) -> Self {
        // SAFETY: `UniverseMultiRefMutHandout` supports claiming additional
        // elements using claim_new. We have to ensure that this is possible
        // without reallocation as that would invalidate the previously handed
        // out refs.
        universe.reserve(CAP.into_usize());
        UniverseMultiRefMutHandout {
            len: universe.data.len(),
            universe_data: NonNull::new(universe.data.as_mut_ptr()).unwrap(),
            first_vacant_entry: &mut universe.first_vacant_entry,
            handed_out: arrayvec::ArrayVec::new(),
        }
    }

    pub fn claim(&mut self, i: I) -> &'a mut T {
        let idx = i.into_usize();
        assert!(self.len > idx);

        assert!(!self.handed_out.contains(&i));
        self.handed_out.push(i);

        // SAFETY: this type dynamically ensures that each index is handed out
        // exactly once through the assert above
        let v = unsafe { &mut *self.universe_data.as_ptr().add(idx) };

        let UniverseEntry::Occupied(v) = v else {
            panic!("attempted to claim vacant universe entry");
        };
        v
    }

    pub fn claim_new(&mut self, value: T) -> (I, &'a mut T) {
        // first_vacant_entry is guaranteed to be valid because
        // we called `Universe::reserve(CAP)` before creating this type
        let idx = self.first_vacant_entry.unwrap().into_usize();
        let id = I::from_usize(idx);
        self.handed_out.push(id);
        let entry = unsafe { &mut *self.universe_data.as_ptr().add(idx) };

        let UniverseEntry::Vacant(next) = *entry else {
            unreachable!()
        };
        *self.first_vacant_entry = next;

        *entry = UniverseEntry::Occupied(value);

        let UniverseEntry::Occupied(v) = entry else {
            unreachable!()
        };
        (id, v)
    }
}
