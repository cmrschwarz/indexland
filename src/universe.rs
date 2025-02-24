#![allow(clippy::match_on_vec_items)]

use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[cfg(feature = "multi_ref_mut_handout")]
use crate::universe_multi_ref_mut_handout::{
    UniverseMultiRefMutHandout, UniverseRefHandoutStackBase,
};

use super::{get_three_distinct_mut, temp_vec::TransmutableContainer, Idx};

use super::get_two_distinct_mut;

#[derive(Clone)]
pub enum UniverseEntry<I, T> {
    Occupied(T),
    Vacant(Option<I>),
}

#[derive(Clone)]
pub struct Universe<I, T> {
    pub(crate) data: Vec<UniverseEntry<I, T>>,
    pub(crate) first_vacant_entry: Option<I>,
    pub(crate) _phantom_data: PhantomData<I>,
}

impl<I, T> UniverseEntry<I, T> {
    pub fn as_option_mut(&mut self) -> Option<&mut T> {
        match self {
            UniverseEntry::Occupied(v) => Some(v),
            UniverseEntry::Vacant(_) => None,
        }
    }
}

// if we autoderive this, I would have to implement Default
impl<I: Idx, T> Default for Universe<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, T> Universe<I, T> {
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            first_vacant_entry: None,
            _phantom_data: PhantomData,
        }
    }

    #[cfg(feature = "multi_ref_mut_handout")]
    pub fn multi_ref_mut_handout<const CAP: usize>(
        &mut self,
    ) -> UniverseMultiRefMutHandout<I, T, CAP> {
        UniverseMultiRefMutHandout::new(self)
    }

    #[cfg(feature = "multi_ref_mut_handout")]
    pub fn ref_mut_handout_stack(
        &mut self,
    ) -> UniverseRefHandoutStackBase<I, T> {
        UniverseRefHandoutStackBase::new(self)
    }
    fn build_vacant_entry(&mut self, index: usize) -> UniverseEntry<I, T> {
        let res = UniverseEntry::Vacant(self.first_vacant_entry);
        self.first_vacant_entry = Some(I::from_usize(index));
        res
    }
    pub fn release(&mut self, id: I) {
        let index = id.into_usize();
        if self.data.len() == index + 1 {
            self.data.pop();
            return;
        }
        self.data[index] = self.build_vacant_entry(index);
    }
    pub fn used_capacity(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.first_vacant_entry = None;
    }
    pub fn indices(&self) -> UniverseIndexIter<I, T> {
        UniverseIndexIter {
            index: I::ZERO,
            base: self.data.iter(),
        }
    }
    pub fn iter(&self) -> UniverseIter<I, T> {
        UniverseIter {
            base: self.data.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> UniverseIterMut<I, T> {
        UniverseIterMut {
            base: self.data.iter_mut(),
        }
    }
    pub fn iter_enumerated(&self) -> UniverseEnumeratedIter<I, T> {
        UniverseEnumeratedIter {
            base: &self.data,
            idx: I::from_usize(0),
        }
    }
    pub fn iter_enumerated_mut(&mut self) -> UniverseEnumeratedIterMut<I, T> {
        UniverseEnumeratedIterMut {
            base: &mut self.data,
            idx: I::from_usize(0),
        }
    }
    pub fn any_used(&mut self) -> Option<&mut T> {
        self.iter_mut().next()
    }
    pub fn reserve(&mut self, additional: usize) {
        let mut len = self.data.len();
        for _ in 0..additional {
            let ve = self.build_vacant_entry(len);
            self.data.push(ve);
            len += 1;
        }
    }
    /// If id is smaller than `used_capacity()`,
    /// this function is on average O(n) over the amount of vacant
    /// slots in the universe. Avoid where possible.
    pub fn reserve_id_with(&mut self, id: I, f: impl FnOnce() -> T) -> &mut T {
        let idx = id.into_usize();
        let used_cap = self.used_capacity();
        if idx >= used_cap {
            self.reserve((idx - used_cap).saturating_sub(1));
            self.data.push(UniverseEntry::Occupied(f()));
        } else {
            let mut vacant_index =
                self.first_vacant_entry.unwrap().into_usize();
            let UniverseEntry::Vacant(mut next) = self.data[vacant_index]
            else {
                unreachable!()
            };
            if vacant_index == idx {
                self.first_vacant_entry = next;
            } else {
                loop {
                    let next_idx = next.unwrap().into_usize();
                    let UniverseEntry::Vacant(next_next) = self.data[next_idx]
                    else {
                        unreachable!()
                    };
                    if next_idx == idx {
                        self.data[vacant_index] =
                            UniverseEntry::Vacant(next_next);
                        break;
                    }
                    vacant_index = next_idx;
                    next = next_next;
                }
            }
            self.data[idx] = UniverseEntry::Occupied(f());
        }
        let UniverseEntry::Occupied(v) = &mut self.data[idx] else {
            unreachable!()
        };
        v
    }
    // returns the id that will be used by the next claim
    // useful for cases where claim_with needs to know the id beforehand
    pub fn peek_claim_id(&self) -> I {
        I::from_usize(if let Some(id) = self.first_vacant_entry {
            id.into_usize()
        } else {
            self.data.len()
        })
    }

    pub fn claim_with(&mut self, f: impl FnOnce() -> T) -> I {
        if let Some(id) = self.first_vacant_entry {
            let index = id.into_usize();
            match self.data[index] {
                UniverseEntry::Vacant(next) => self.first_vacant_entry = next,
                UniverseEntry::Occupied(_) => unreachable!(),
            }
            self.data[index] = UniverseEntry::Occupied(f());
            I::from_usize(index)
        } else {
            let id = self.data.len();
            self.data.push(UniverseEntry::Occupied(f()));
            I::from_usize(id)
        }
    }
    pub fn claim_with_value(&mut self, value: T) -> I {
        self.claim_with(|| value)
    }
    pub fn calc_id(&self, entry: &T) -> I {
        let offset_in_entry = if let UniverseEntry::Occupied(v) = &self.data[0]
        {
            unsafe {
                std::ptr::from_ref(v)
                    .cast::<u8>()
                    .offset_from(self.data.as_ptr().cast())
            }
        } else {
            panic!("element not in Universe")
        };
        let ptr = unsafe {
            std::ptr::from_ref(entry)
                .cast::<u8>()
                .sub(usize::try_from(offset_in_entry).unwrap_unchecked())
                .cast()
        };
        let range = self.data.as_ptr_range();
        assert!(range.contains(&ptr));
        #[allow(clippy::cast_sign_loss)]
        I::from_usize(unsafe { ptr.offset_from(range.start) } as usize)
    }
    pub fn get(&self, id: I) -> Option<&T> {
        match self.data.get(id.into_usize()) {
            Some(UniverseEntry::Occupied(v)) => Some(v),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        match self.data.get_mut(id.into_usize()) {
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

        let (a, b) = get_two_distinct_mut(&mut self.data, id1, id2);
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

        let (a, b, c) = get_three_distinct_mut(&mut self.data, id1, id2, id3);
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
}

// separate impl since only available if T: Default
impl<I: Idx, T: Default> Universe<I, T> {
    pub fn claim(&mut self) -> I {
        self.claim_with(Default::default)
    }
}

impl<I: Idx, T> Index<I> for Universe<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        match &self.data[index.into_usize()] {
            UniverseEntry::Occupied(v) => v,
            UniverseEntry::Vacant(_) => panic!("index out of bounds"),
        }
    }
}

impl<I: Idx, T> IndexMut<I> for Universe<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        match &mut self.data[index.into_usize()] {
            UniverseEntry::Occupied(v) => v,
            UniverseEntry::Vacant(_) => panic!("index out of bounds"),
        }
    }
}

#[derive(Clone)]
pub struct UniverseIter<'a, I, T> {
    base: std::slice::Iter<'a, UniverseEntry<I, T>>,
}

#[derive(Clone)]
pub struct UniverseIndexIter<'a, I, T> {
    index: I,
    base: std::slice::Iter<'a, UniverseEntry<I, T>>,
}

impl<'a, I, T> Iterator for UniverseIter<'a, I, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.base.next() {
                Some(UniverseEntry::Occupied(v)) => return Some(v),
                Some(UniverseEntry::Vacant(_)) => continue,
                None => return None,
            }
        }
    }
}

impl<I: Idx, T> Iterator for UniverseIndexIter<'_, I, T> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.base.next()?;
            let res = self.index;
            self.index = I::from_usize(res.into_usize() + 1);
            if matches!(next, UniverseEntry::Vacant(_)) {
                continue;
            }
            return Some(res);
        }
    }
}

pub struct UniverseIterMut<'a, I, T> {
    base: std::slice::IterMut<'a, UniverseEntry<I, T>>,
}

impl<'a, I, T> Iterator for UniverseIterMut<'a, I, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.base.next() {
                Some(UniverseEntry::Occupied(v)) => return Some(v),
                Some(UniverseEntry::Vacant(_)) => continue,
                None => return None,
            }
        }
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a Universe<I, T> {
    type Item = &'a T;
    type IntoIter = UniverseIter<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut Universe<I, T> {
    type Item = &'a mut T;
    type IntoIter = UniverseIterMut<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Clone)]
pub struct UniverseEnumeratedIter<'a, I, T> {
    base: &'a [UniverseEntry<I, T>],
    idx: I,
}

impl<'a, I: Idx, T> Iterator for UniverseEnumeratedIter<'a, I, T> {
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx.into_usize()..self.base.len() {
            let idx = self.idx;
            self.idx = I::from_usize(i + 1);
            match &self.base[i] {
                UniverseEntry::Occupied(v) => return Some((idx, v)),
                UniverseEntry::Vacant(_) => continue,
            }
        }
        None
    }
}

pub struct UniverseEnumeratedIterMut<'a, I, T> {
    base: &'a mut [UniverseEntry<I, T>],
    idx: I,
}

impl<'a, I: Idx, T> Iterator for UniverseEnumeratedIterMut<'a, I, T> {
    type Item = (I, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx.into_usize()..self.base.len() {
            let idx = self.idx;
            self.idx = I::from_usize(i + 1);
            match &self.base[i] {
                UniverseEntry::Occupied(_) => {
                    // SAFETY: the iterator makes sure that each element
                    // is only handed out once
                    let v = unsafe { &mut *self.base.as_mut_ptr().add(i) };
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

impl<I: Idx, T, II: IntoIterator<Item = T>> From<II> for Universe<I, T> {
    fn from(ii: II) -> Self {
        let mut u = Universe::default();
        for i in ii {
            u.claim_with_value(i);
        }
        u
    }
}

impl<I: Idx, T> TransmutableContainer for Universe<I, T> {
    type ElementType = T;

    type ContainerType<Q> = Universe<I, Q>;

    fn transmute<Q>(
        self,
    ) -> <Self as TransmutableContainer>::ContainerType<Q> {
        Universe {
            data: self.data.transmute(),
            first_vacant_entry: None,
            _phantom_data: PhantomData,
        }
    }

    fn transmute_from<Q>(
        src: <Self as TransmutableContainer>::ContainerType<Q>,
    ) -> Self {
        Self {
            data: src.data.transmute(),
            first_vacant_entry: None,
            _phantom_data: PhantomData,
        }
    }
}
