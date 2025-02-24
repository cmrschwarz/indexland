use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use super::{
    idx::Idx,
    stable_vec::{self, StableVec, StableVecIter, StableVecIterMut},
    temp_vec::TransmutableContainer,
    universe::UniverseEntry,
};

pub struct StableUniverse<I, T> {
    data: StableVec<UnsafeCell<UniverseEntry<I, T>>>,
    first_vacant_entry: Cell<Option<I>>,
    _phantom_data: PhantomData<I>,
}

impl<I: Idx, T: Clone> Clone for StableUniverse<I, T> {
    fn clone(&self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|entry| match unsafe { &*entry.get() } {
                    UniverseEntry::Occupied(v) => {
                        UniverseEntry::Occupied(v.clone())
                    }
                    UniverseEntry::Vacant(next) => {
                        UniverseEntry::Vacant(*next)
                    }
                })
                .map(UnsafeCell::new)
                .collect(),
            first_vacant_entry: Cell::new(self.first_vacant_entry.get()),
            _phantom_data: PhantomData,
        }
    }
}

// if we autoderive this, I would have to implement Default
impl<I: Idx, T> Default for StableUniverse<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, T> StableUniverse<I, T> {
    pub const fn new() -> Self {
        Self {
            data: StableVec::new(),
            first_vacant_entry: Cell::new(None),
            _phantom_data: PhantomData,
        }
    }
    fn build_vacant_entry(&self, index: usize) -> UniverseEntry<I, T> {
        let res = UniverseEntry::Vacant(self.first_vacant_entry.get());
        self.first_vacant_entry.set(Some(I::from_usize(index)));
        res
    }
    pub fn release(&mut self, id: I) -> T {
        let index = id.into_usize();
        let entry = if self.data.len() == index + 1 {
            self.data.pop().unwrap().into_inner()
        } else {
            let vacant = self.build_vacant_entry(index);
            std::mem::replace(self.data[index].get_mut(), vacant)
        };
        match entry {
            UniverseEntry::Occupied(v) => v,
            UniverseEntry::Vacant(_) => panic!("released non occupied entry"),
        }
    }
    pub fn used_capacity(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.first_vacant_entry.set(None);
    }
    pub fn indices(&self) -> StableUniverseIndexIter<I, T> {
        StableUniverseIndexIter {
            index: I::ZERO,
            base: self.data.iter(),
        }
    }
    pub fn iter(&self) -> StableUniverseIter<I, T> {
        StableUniverseIter {
            base: self.data.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> StableUniverseIterMut<I, T> {
        StableUniverseIterMut {
            base: self.data.iter_mut(),
        }
    }
    pub fn iter_enumerated(&self) -> StableUniverseEnumeratedIter<I, T> {
        StableUniverseEnumeratedIter {
            base: &self.data,
            idx: I::from_usize(0),
        }
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> StableUniverseEnumeratedIterMut<I, T> {
        StableUniverseEnumeratedIterMut {
            base: &mut self.data,
            idx: I::from_usize(0),
        }
    }
    pub fn any_used(&mut self) -> Option<&mut T> {
        self.iter_mut().next()
    }
    pub fn reserve(&self, additional: usize) {
        let mut len = self.data.len();
        for _ in 0..additional {
            let ve = self.build_vacant_entry(len);
            self.data.push(UnsafeCell::new(ve));
            len += 1;
        }
    }
    /// If id is smaller than `used_capacity()`,
    /// this function is on average O(n) over the amount of vacant
    /// slots in the universe. Avoid where possible.
    pub fn reserve_id_with(&self, id: I, f: impl FnOnce() -> T) {
        let idx = id.into_usize();
        let used_cap = self.used_capacity();
        if idx >= used_cap {
            self.reserve((idx - used_cap).saturating_sub(1));
            self.data
                .push(UnsafeCell::new(UniverseEntry::Occupied(f())));
        } else {
            let mut vacant_index =
                self.first_vacant_entry.get().unwrap().into_usize();
            let UniverseEntry::Vacant(next) =
                (unsafe { &*self.data[vacant_index].get() })
            else {
                unreachable!()
            };
            let mut next = *next;
            if vacant_index == idx {
                self.first_vacant_entry.set(next);
            } else {
                loop {
                    let next_idx = next.unwrap().into_usize();
                    let UniverseEntry::Vacant(next_next) =
                        (unsafe { &*self.data[next_idx].get() })
                    else {
                        unreachable!()
                    };
                    let next_next = *next_next;
                    if next_idx == idx {
                        unsafe {
                            *self.data[vacant_index].get() =
                                UniverseEntry::Vacant(next_next);
                        };
                        break;
                    }
                    vacant_index = next_idx;
                    next = next_next;
                }
            }
            unsafe { *self.data[idx].get() = UniverseEntry::Occupied(f()) };
        }
    }
    // returns the id that will be used by the next claim
    // useful for cases where claim_with needs to know the id beforehand
    pub fn peek_claim_id(&self) -> I {
        I::from_usize(if let Some(id) = self.first_vacant_entry.get() {
            id.into_usize()
        } else {
            self.data.len()
        })
    }

    pub fn claim_with(&self, f: impl FnOnce() -> T) -> I {
        if let Some(id) = self.first_vacant_entry.get() {
            let index = id.into_usize();
            match unsafe { &*self.data[index].get() } {
                UniverseEntry::Vacant(next) => {
                    self.first_vacant_entry.set(*next);
                }
                UniverseEntry::Occupied(_) => unreachable!(),
            }
            unsafe { *self.data[index].get() = UniverseEntry::Occupied(f()) };
            I::from_usize(index)
        } else {
            let id = self.data.len();
            self.data
                .push(UnsafeCell::new(UniverseEntry::Occupied(f())));
            I::from_usize(id)
        }
    }
    pub fn claim_with_value(&self, value: T) -> I {
        self.claim_with(|| value)
    }
    pub fn get(&self, id: I) -> Option<&T> {
        match self.data.get(id.into_usize()).map(|c| unsafe { &*c.get() }) {
            Some(UniverseEntry::Occupied(v)) => Some(v),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        match self.data.get_mut(id.into_usize()).map(UnsafeCell::get_mut) {
            Some(UniverseEntry::Occupied(v)) => Some(v),
            _ => None,
        }
    }
    pub fn get_two_distinct_mut(
        &mut self,
        id1: I,
        id2: I,
    ) -> (Option<&mut T>, Option<&mut T>) {
        let id1 = id1.into_usize();
        let id2 = id2.into_usize();
        let (a, b) = unsafe {
            let a_ptr = self.data.get_element_pointer_unchecked(id1);
            let b_ptr = self.data.get_element_pointer_unchecked(id2);
            ((*a_ptr).get_mut(), (*b_ptr).get_mut())
        };
        (a.as_option_mut(), b.as_option_mut())
    }
    pub fn get_three_distinct_mut(
        &mut self,
        id1: I,
        id2: I,
        id3: I,
    ) -> (Option<&mut T>, Option<&mut T>, Option<&mut T>) {
        let id1 = id1.into_usize();
        let id2 = id2.into_usize();
        let id3 = id3.into_usize();

        let (a, b, c) = unsafe {
            let a_ptr = self.data.get_element_pointer_unchecked(id1);
            let b_ptr = self.data.get_element_pointer_unchecked(id2);
            let c_ptr = self.data.get_element_pointer_unchecked(id3);
            ((*a_ptr).get_mut(), (*b_ptr).get_mut(), (*c_ptr).get_mut())
        };
        (a.as_option_mut(), b.as_option_mut(), c.as_option_mut())
    }
    pub fn two_distinct_mut(&mut self, id1: I, id2: I) -> (&mut T, &mut T) {
        let (a, b) = self.get_two_distinct_mut(id1, id2);
        (a.unwrap(), b.unwrap())
    }
    pub fn three_distinct_mut(
        &mut self,
        id1: I,
        id2: I,
        id3: I,
    ) -> (&mut T, &mut T, &mut T) {
        let (a, b, c) = self.get_three_distinct_mut(id1, id2, id3);
        (a.unwrap(), b.unwrap(), c.unwrap())
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn next_index_phys(&self) -> I {
        I::from_usize(self.data.len())
    }

    fn extend(&self, iter: impl IntoIterator<Item = T>) {
        for v in iter {
            self.claim_with_value(v);
        }
    }
}

// separate impl since only available if T: Default
impl<I: Idx, T: Default> StableUniverse<I, T> {
    pub fn claim(&self) -> I {
        self.claim_with(Default::default)
    }
}

impl<I: Idx, T> Index<I> for StableUniverse<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        match unsafe { &*self.data[index.into_usize()].get() } {
            UniverseEntry::Occupied(v) => v,
            UniverseEntry::Vacant(_) => panic!("index out of bounds"),
        }
    }
}

impl<I: Idx, T> IndexMut<I> for StableUniverse<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        match self.data[index.into_usize()].get_mut() {
            UniverseEntry::Occupied(v) => v,
            UniverseEntry::Vacant(_) => panic!("index out of bounds"),
        }
    }
}

#[derive(Clone)]
pub struct StableUniverseIter<'a, I, T> {
    base: StableVecIter<
        'a,
        UnsafeCell<UniverseEntry<I, T>>,
        { stable_vec::DEFAULT_CHUNK_SIZE },
    >,
}

pub struct StableUniverseIndexIter<'a, I, T> {
    index: I,
    base: StableVecIter<
        'a,
        UnsafeCell<UniverseEntry<I, T>>,
        { stable_vec::DEFAULT_CHUNK_SIZE },
    >,
}

impl<'a, I, T> Iterator for StableUniverseIter<'a, I, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.base.next().map(|e| unsafe { &*e.get() }) {
                Some(UniverseEntry::Occupied(v)) => return Some(v),
                Some(UniverseEntry::Vacant(_)) => continue,
                None => return None,
            }
        }
    }
}

impl<I: Idx, T> Iterator for StableUniverseIndexIter<'_, I, T> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.base.next()?;
            let res = self.index;
            self.index = I::from_usize(res.into_usize() + 1);
            if matches!(unsafe { &*next.get() }, UniverseEntry::Vacant(_)) {
                continue;
            }
            return Some(res);
        }
    }
}

pub struct StableUniverseIterMut<'a, I, T> {
    base: StableVecIterMut<
        'a,
        UnsafeCell<UniverseEntry<I, T>>,
        { stable_vec::DEFAULT_CHUNK_SIZE },
    >,
}

impl<'a, I, T> Iterator for StableUniverseIterMut<'a, I, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.base.next() {
                Some(entry) => match entry.get_mut() {
                    UniverseEntry::Vacant(_) => continue,
                    UniverseEntry::Occupied(v) => {
                        return Some(v);
                    }
                },
                None => return None,
            }
        }
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a StableUniverse<I, T> {
    type Item = &'a T;
    type IntoIter = StableUniverseIter<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut StableUniverse<I, T> {
    type Item = &'a mut T;
    type IntoIter = StableUniverseIterMut<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Clone)]
pub struct StableUniverseEnumeratedIter<'a, I, T> {
    base: &'a StableVec<UnsafeCell<UniverseEntry<I, T>>>,
    idx: I,
}

impl<'a, I: Idx, T> Iterator for StableUniverseEnumeratedIter<'a, I, T> {
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx.into_usize()..self.base.len() {
            let idx = self.idx;
            self.idx = I::from_usize(i + 1);
            match unsafe { &*self.base[i].get() } {
                UniverseEntry::Occupied(v) => return Some((idx, v)),
                UniverseEntry::Vacant(_) => continue,
            }
        }
        None
    }
}

pub struct StableUniverseEnumeratedIterMut<'a, I, T> {
    base: &'a mut StableVec<UnsafeCell<UniverseEntry<I, T>>>,
    idx: I,
}

impl<'a, I: Idx, T> Iterator for StableUniverseEnumeratedIterMut<'a, I, T> {
    type Item = (I, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx.into_usize()..self.base.len() {
            let idx = self.idx;
            self.idx = I::from_usize(i + 1);
            match unsafe { &*self.base[i].get() } {
                UniverseEntry::Occupied(_) => {
                    // polonius lifetime issue workaround
                    // SAFETY: the iterator makes sure that each element
                    // is only handed out once
                    let v = unsafe {
                        (*self.base.get_element_pointer_unchecked(i)).get_mut()
                    };
                    let UniverseEntry::Occupied(v) = v else {
                        unreachable!()
                    };
                    return Some((idx, v));
                }
                UniverseEntry::Vacant(_) => continue,
            }
        }
        None
    }
}

impl<I: Idx, T, II: IntoIterator<Item = T>> From<II> for StableUniverse<I, T> {
    fn from(ii: II) -> Self {
        let u = StableUniverse::default();
        u.extend(ii);
        u
    }
}

impl<I: Idx, T> TransmutableContainer for StableUniverse<I, T> {
    type ElementType = T;

    type ContainerType<Q> = StableUniverse<I, Q>;

    fn transmute<Q>(
        self,
    ) -> <Self as TransmutableContainer>::ContainerType<Q> {
        StableUniverse {
            data: self.data.transmute(),
            first_vacant_entry: Cell::new(None),
            _phantom_data: PhantomData,
        }
    }

    fn transmute_from<Q>(
        src: <Self as TransmutableContainer>::ContainerType<Q>,
    ) -> Self {
        Self {
            data: src.data.transmute(),
            first_vacant_entry: Cell::new(None),
            _phantom_data: PhantomData,
        }
    }
}
