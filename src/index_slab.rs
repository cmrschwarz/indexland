use core::{
    fmt::{self, Debug},
    iter::{FromIterator, FusedIterator},
    marker::PhantomData,
    ops,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use slab::Drain;
use slab::{GetDisjointMutError, Slab};

use crate::{Idx, idx::IdxCompat};

#[repr(transparent)]
pub struct IndexSlab<I, T> {
    data: Slab<T>,
    _phantom: PhantomData<fn(I) -> T>,
}

impl<I, T> Clone for IndexSlab<I, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _phantom: PhantomData,
        }
    }
    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<I, T> Default for IndexSlab<I, T> {
    fn default() -> Self {
        Self {
            data: Slab::new(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> IndexSlab<I, T> {
    pub const fn new() -> Self {
        Self {
            data: Slab::new(),
            _phantom: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Slab<T> {
        Slab::with_capacity(capacity)
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn compact<F>(&mut self, mut rekey: F)
    where
        I: Idx,
        F: FnMut(&mut T, I, I) -> bool,
    {
        self.data
            .compact(|val, from, to| rekey(val, I::from_usize(from), I::from_usize(to)));
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn len_idx(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.len())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, I, T>
    where
        I: Idx,
    {
        Iter {
            base: self.data.iter(),
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, I, T>
    where
        I: Idx,
    {
        IterMut {
            base: self.data.iter_mut(),
            _phantom: PhantomData,
        }
    }

    pub fn get(&self, key: I) -> Option<&T>
    where
        I: Idx,
    {
        self.data.get(key.into_usize())
    }

    pub fn get_mut(&mut self, key: I) -> Option<&mut T>
    where
        I: Idx,
    {
        self.data.get_mut(key.into_usize())
    }

    pub fn get_disjoint_mut<const N: usize, X>(
        &mut self,
        indices: [X; N],
    ) -> Result<[&mut T; N], GetDisjointMutError>
    where
        X: IdxCompat<I>,
    {
        self.data.get_disjoint_mut(indices.map(X::into_usize))
    }

    /// ## Safety
    /// The key must be within bounds, and convert to usize.
    pub unsafe fn get_unchecked(&self, key: I) -> &T
    where
        I: Idx,
    {
        unsafe { self.data.get_unchecked(key.into_usize_unchecked()) }
    }

    /// ## Safety
    /// The key must be within bounds, and convert to usize.
    pub unsafe fn get_unchecked_mut(&mut self, key: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(key.into_usize_unchecked()) }
    }

    pub fn key_of(&self, present_element: &T) -> usize {
        self.data.key_of(present_element)
    }

    pub fn insert(&mut self, val: T) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.insert(val))
    }

    pub fn vacant_key(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.vacant_key())
    }

    pub fn vacant_entry(&mut self) -> VacantEntry<'_, I, T> {
        VacantEntry {
            base: self.data.vacant_entry(),
            _phantom: PhantomData,
        }
    }

    pub fn try_remove(&mut self, key: I) -> Option<T>
    where
        I: Idx,
    {
        self.data.try_remove(key.into_usize())
    }

    pub fn remove(&mut self, key: I) -> T
    where
        I: Idx,
    {
        self.data.remove(key.into_usize())
    }

    pub fn contains(&self, key: I) -> bool
    where
        I: Idx,
    {
        self.data.contains(key.into_usize())
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(I, &mut T) -> bool,
        I: Idx,
    {
        self.data.retain(|idx, val| f(I::from_usize(idx), val));
    }

    pub fn drain(&mut self) -> Drain<'_, T> {
        self.data.drain()
    }
}

impl<I, X, T> ops::Index<X> for IndexSlab<I, T>
where
    X: IdxCompat<I>,
{
    type Output = T;

    fn index(&self, key: X) -> &T {
        self.data.index(key.into_usize())
    }
}

impl<I, X, T> ops::IndexMut<X> for IndexSlab<I, T>
where
    X: IdxCompat<I>,
{
    fn index_mut(&mut self, key: X) -> &mut T {
        self.data.index_mut(key.into_usize())
    }
}

impl<I, T> IntoIterator for IndexSlab<I, T>
where
    I: Idx,
{
    type Item = (I, T);
    type IntoIter = IntoIter<I, T>;

    fn into_iter(self) -> IntoIter<I, T> {
        IntoIter {
            base: self.data.into_iter(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, I, T> IntoIterator for &'a IndexSlab<I, T>
where
    I: Idx,
{
    type Item = (I, &'a T);
    type IntoIter = Iter<'a, I, T>;

    fn into_iter(self) -> Iter<'a, I, T> {
        self.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut IndexSlab<I, T>
where
    I: Idx,
{
    type Item = (I, &'a mut T);
    type IntoIter = IterMut<'a, I, T>;

    fn into_iter(self) -> IterMut<'a, I, T> {
        self.iter_mut()
    }
}

impl<I, T> FromIterator<(I, T)> for IndexSlab<I, T>
where
    I: Idx,
{
    fn from_iter<It>(iterable: It) -> Self
    where
        It: IntoIterator<Item = (I, T)>,
    {
        Self {
            data: iterable
                .into_iter()
                .map(|(key, val)| (key.into_usize(), val))
                .collect(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T> fmt::Debug for IndexSlab<I, T>
where
    I: Idx + Debug,
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if fmt.alternate() {
            fmt.debug_map().entries(self.iter()).finish()
        } else {
            fmt.debug_struct("IndexSlab")
                .field("len", &self.data.len())
                .field("cap", &self.data.capacity())
                .finish()
        }
    }
}

impl<I, T> fmt::Debug for IntoIter<I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("IntoIter")
            .field("remaining", &self.base.len())
            .finish()
    }
}

impl<I, T> fmt::Debug for Iter<'_, I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Iter")
            .field("remaining", &self.base.len())
            .finish()
    }
}

impl<I, T> fmt::Debug for IterMut<'_, I, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("IterMut")
            .field("remaining", &self.base.len())
            .finish()
    }
}

impl<I, T> From<Slab<T>> for IndexSlab<I, T> {
    fn from(value: Slab<T>) -> Self {
        IndexSlab {
            data: value,
            _phantom: PhantomData,
        }
    }
}

// ===== VacantEntry =====
pub struct VacantEntry<'a, I, T> {
    base: slab::VacantEntry<'a, T>,
    _phantom: PhantomData<fn(I) -> &'a T>,
}

impl<'a, I, T> VacantEntry<'a, I, T> {
    pub fn insert(self, val: T) -> &'a mut T {
        self.base.insert(val)
    }

    pub fn key(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.base.key())
    }
}

// ===== IntoIter =====
pub struct IntoIter<I, T> {
    base: slab::IntoIter<T>,
    _phantom: PhantomData<fn(I) -> T>,
}
impl<I, T> Iterator for IntoIter<I, T>
where
    I: Idx,
{
    type Item = (I, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.base
            .next()
            .map(|(key, value)| (I::from_usize(key), value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<I, T> DoubleEndedIterator for IntoIter<I, T>
where
    I: Idx,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base
            .next_back()
            .map(|(key, value)| (I::from_usize(key), value))
    }
}

impl<I, T> ExactSizeIterator for IntoIter<I, T>
where
    I: Idx,
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<I, T> FusedIterator for IntoIter<I, T> where I: Idx {}

// ===== Iter =====
pub struct Iter<'a, I, T> {
    base: slab::Iter<'a, T>,
    _phantom: PhantomData<fn(I) -> &'a T>,
}

impl<'a, I, T> Iterator for Iter<'a, I, T>
where
    I: Idx,
{
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.base
            .next()
            .map(|(key, value)| (I::from_usize(key), value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<I, T> DoubleEndedIterator for Iter<'_, I, T>
where
    I: Idx,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base
            .next_back()
            .map(|(key, value)| (I::from_usize(key), value))
    }
}

impl<I, T> ExactSizeIterator for Iter<'_, I, T>
where
    I: Idx,
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<I, T> FusedIterator for Iter<'_, I, T> where I: Idx {}

// ===== IterMut =====
pub struct IterMut<'a, I, T> {
    base: slab::IterMut<'a, T>,
    _phantom: PhantomData<fn(I) -> &'a mut T>,
}
impl<'a, I, T> Iterator for IterMut<'a, I, T>
where
    I: Idx,
{
    type Item = (I, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.base
            .next()
            .map(|(key, value)| (I::from_usize(key), value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<I, T> DoubleEndedIterator for IterMut<'_, I, T>
where
    I: Idx,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base
            .next_back()
            .map(|(key, value)| (I::from_usize(key), value))
    }
}

impl<I, T> ExactSizeIterator for IterMut<'_, I, T>
where
    I: Idx,
{
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<I, T> FusedIterator for IterMut<'_, I, T> where I: Idx {}

#[cfg(feature = "serde")]
impl<I, T> Serialize for IndexSlab<I, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T> Deserialize<'de> for IndexSlab<I, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let slab = Slab::<T>::deserialize(deserializer)?;

        Ok(Self::from(slab))
    }
}
