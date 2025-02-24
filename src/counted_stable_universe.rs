// TODO: might be better to implement this using a separate freelist
// instead of this counter?

use std::ops::{Index, IndexMut};

use crate::{
    Idx,
    stable_universe::{
        StableUniverse, StableUniverseEnumeratedIter,
        StableUniverseEnumeratedIterMut, StableUniverseIndexIter,
        StableUniverseIter, StableUniverseIterMut,
    },
};

pub struct CountedStableUniverse<I, T> {
    universe: StableUniverse<I, T>,
    occupied_entries: usize,
}

impl<I: Idx, T> CountedStableUniverse<I, T> {
    pub const fn new() -> Self {
        Self {
            universe: StableUniverse::new(),
            occupied_entries: 0,
        }
    }
    pub fn release(&mut self, id: I) -> T {
        self.occupied_entries -= 1;
        self.universe.release(id)
    }
    pub fn used_capacity(&mut self) -> usize {
        self.universe.used_capacity()
    }
    pub fn clear(&mut self) {
        self.universe.clear();
        self.occupied_entries = 0;
    }
    pub fn indices(&self) -> StableUniverseIndexIter<I, T> {
        self.universe.indices()
    }
    pub fn iter(&self) -> StableUniverseIter<I, T> {
        self.universe.iter()
    }
    pub fn iter_mut(&mut self) -> StableUniverseIterMut<I, T> {
        self.universe.iter_mut()
    }
    pub fn iter_enumerated(&self) -> StableUniverseEnumeratedIter<I, T> {
        self.universe.iter_enumerated()
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> StableUniverseEnumeratedIterMut<I, T> {
        self.universe.iter_enumerated_mut()
    }
    pub fn any_used(&mut self) -> Option<&mut T> {
        self.universe.any_used()
    }
    pub fn reserve_id_with(&self, id: I, func: impl FnOnce() -> T) {
        self.universe.reserve_id_with(id, func);
    }
    pub fn peek_claim_id(&self) -> I {
        self.universe.peek_claim_id()
    }
    pub fn claim_with(&mut self, f: impl FnOnce() -> T) -> I {
        self.occupied_entries += 1;
        self.universe.claim_with(f)
    }
    pub fn claim_with_value(&mut self, value: T) -> I {
        self.claim_with(|| value)
    }
    pub fn get(&self, id: I) -> Option<&T> {
        self.universe.get(id)
    }
    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        self.universe.get_mut(id)
    }
    pub fn get_two_distinct_mut(
        &mut self,
        id1: I,
        id2: I,
    ) -> (Option<&mut T>, Option<&mut T>) {
        self.universe.get_two_distinct_mut(id1, id2)
    }
    pub fn two_distinct_mut(&mut self, id1: I, id2: I) -> (&mut T, &mut T) {
        self.universe.two_distinct_mut(id1, id2)
    }
    pub fn is_empty(&self) -> bool {
        self.occupied_entries == 0
    }
    pub fn occupied_entry_count(&self) -> usize {
        self.occupied_entries
    }
    pub fn next_index_phys(&self) -> I {
        self.universe.next_index_phys()
    }
}

// autoderiving this currently fails on stable
impl<I: Idx, T> Default for CountedStableUniverse<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

// separate impl since only available if T: Default
impl<I: Idx, T: Default> CountedStableUniverse<I, T> {
    pub fn claim(&mut self) -> I {
        self.claim_with(Default::default)
    }
    pub fn reserve_id(&mut self, id: I) {
        self.reserve_id_with(id, Default::default);
    }
}

impl<I: Idx, T: Clone> Clone for CountedStableUniverse<I, T> {
    fn clone(&self) -> Self {
        Self {
            universe: self.universe.clone(),
            occupied_entries: self.occupied_entries,
        }
    }
}

impl<I: Idx, T> Index<I> for CountedStableUniverse<I, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.universe.index(index)
    }
}

impl<I: Idx, T> IndexMut<I> for CountedStableUniverse<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.universe.index_mut(index)
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a CountedStableUniverse<I, T> {
    type Item = &'a T;
    type IntoIter = StableUniverseIter<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T> IntoIterator for &'a mut CountedStableUniverse<I, T> {
    type Item = &'a mut T;
    type IntoIter = StableUniverseIterMut<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
