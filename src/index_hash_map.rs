use crate::{index_enumerate::IndexEnumerate, index_range::IndexRangeBounds};
use alloc::boxed::Box;
use core::{
    fmt::Debug,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    ops::{Index, RangeBounds},
};

use indexmap::{map::Slice, Equivalent, IndexMap};

use super::{idx::Idx, index_range::IndexRange};

/// Create an [`IndexHashMap`] containing the arguments.
///
/// The syntax is identical to [`indexmap!`](::indexmap::indexmap!).
///
/// The index type cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// # Example
/// ```
/// use indexland::{index_hash_map, IndexHashMap};
/// let map: IndexHashMap<u32, _, _> = index_hash_map! {
///     "a" => 17,
///     "b" => 42,
/// };
/// ```
#[macro_export]
macro_rules! index_hash_map {
    ($($anything: tt)*) => {
        $crate::IndexHashMap::from($crate::indexmap::indexmap![$($anything)*])
    }
}

#[cfg(feature = "std")]
use std::collections::hash_map::RandomState;

#[cfg(feature = "std")]
#[repr(transparent)]
#[derive(Clone)]
pub struct IndexHashMap<I, K, V, S = RandomState> {
    data: IndexMap<K, V, S>,
    _phantom: PhantomData<fn(I) -> (K, V)>,
}

#[cfg(not(feature = "std"))]
#[repr(transparent)]
#[derive(Clone)]
pub struct IndexHashMap<I, K, V, S> {
    data: IndexMap<K, V, S>,
    _phantom: PhantomData<fn(I) -> (K, V)>,
}

#[repr(transparent)]
pub struct IndexHashMapSlice<I, K, V> {
    _phantom: PhantomData<fn(I) -> (K, V)>,
    #[allow(unused)]
    data: Slice<K, V>,
}

impl<I, K, V> IndexHashMapSlice<I, K, V> {
    #[inline]
    pub fn from_index_map_slice(s: &Slice<K, V>) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    pub fn from_index_map_slice_mut(s: &mut Slice<K, V>) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Self) }
    }
    #[inline]
    pub fn into_index_map_slice(s: &Self) -> &Slice<K, V> {
        unsafe { &*(core::ptr::from_ref(s) as *const Slice<K, V>) }
    }
    #[inline]
    pub fn into_index_map_slice_mut(s: &mut Self) -> &mut Slice<K, V> {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Slice<K, V>) }
    }
    pub fn from_boxed_slice(slice_box: Box<Slice<K, V>>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice_box) as *mut Self) }
    }
    pub fn into_boxed_slice(self: Box<Self>) -> Box<Slice<K, V>> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut Slice<K, V>) }
    }
}

impl<'a, I, K, V> From<&'a Slice<K, V>> for &'a IndexHashMapSlice<I, K, V> {
    fn from(data: &'a Slice<K, V>) -> Self {
        IndexHashMapSlice::from_index_map_slice(data)
    }
}
impl<'a, I, K, V> From<&'a IndexHashMapSlice<I, K, V>> for &'a Slice<K, V> {
    fn from(data: &'a IndexHashMapSlice<I, K, V>) -> Self {
        IndexHashMapSlice::into_index_map_slice(data)
    }
}
impl<'a, I, K, V> From<&'a mut Slice<K, V>>
    for &'a mut IndexHashMapSlice<I, K, V>
{
    fn from(data: &'a mut Slice<K, V>) -> Self {
        IndexHashMapSlice::from_index_map_slice_mut(data)
    }
}
impl<'a, I, K, V> From<&'a mut IndexHashMapSlice<I, K, V>>
    for &'a mut Slice<K, V>
{
    fn from(data: &'a mut IndexHashMapSlice<I, K, V>) -> Self {
        IndexHashMapSlice::into_index_map_slice_mut(data)
    }
}

impl<I, K, V, S> From<IndexMap<K, V, S>> for IndexHashMap<I, K, V, S> {
    fn from(v: IndexMap<K, V, S>) -> Self {
        Self {
            data: v,
            _phantom: PhantomData,
        }
    }
}
impl<I, K, V, S> From<IndexHashMap<I, K, V, S>> for IndexMap<K, V, S> {
    fn from(v: IndexHashMap<I, K, V, S>) -> Self {
        v.data
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, K: Hash + Eq, V, const N: usize> From<[(K, V); N]>
    for IndexHashMap<I, K, V, RandomState>
{
    fn from(arr: [(K, V); N]) -> IndexHashMap<I, K, V, RandomState> {
        IndexHashMap::from(IndexMap::from(arr))
    }
}

impl<I, K, V, S: Default> Default for IndexHashMap<I, K, V, S> {
    fn default() -> Self {
        Self {
            data: IndexMap::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I, K: Debug, V: Debug, S> Debug for IndexHashMap<I, K, V, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, K, V> IndexHashMap<I, K, V> {
    pub fn new() -> Self {
        Self {
            data: IndexMap::default(),
            _phantom: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: IndexMap::with_capacity(cap),
            _phantom: PhantomData,
        }
    }
}

impl<I, K, V, S> IndexHashMap<I, K, V, S> {
    pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
        Self {
            data: IndexMap::with_capacity_and_hasher(cap, hasher),
            _phantom: PhantomData,
        }
    }
    pub fn with_hasher(hasher: S) -> Self {
        Self {
            data: IndexMap::with_hasher(hasher),
            _phantom: PhantomData,
        }
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn hasher(&self) -> &S {
        self.data.hasher()
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
    pub fn last_idx(&self) -> Option<I>
    where
        I: Idx,
    {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn iter(&self) -> indexmap::map::Iter<K, V> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<K, V> {
        self.data.iter_mut()
    }
    pub fn iter_enumerated(
        &self,
    ) -> IndexEnumerate<I, indexmap::map::Iter<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IndexEnumerate<I, indexmap::map::IterMut<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range<R>(
        &self,
        range: R,
    ) -> IndexEnumerate<I, indexmap::map::Iter<K, V>>
    where
        I: Idx,
        R: IndexRangeBounds<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &self.data[range])
    }
    pub fn iter_enumerated_range_mut<R>(
        &mut self,
        range: R,
    ) -> IndexEnumerate<I, indexmap::map::IterMut<K, V>>
    where
        I: Idx,
        R: IndexRangeBounds<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, indexmap::map::IntoIter<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn keys(&self) -> indexmap::map::Keys<K, V> {
        self.data.keys()
    }
    pub fn keys_enumerated(
        &self,
    ) -> IndexEnumerate<I, indexmap::map::Keys<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.keys())
    }
    pub fn into_keys(self) -> indexmap::map::IntoKeys<K, V> {
        self.data.into_keys()
    }
    pub fn into_keys_enumerated(
        self,
    ) -> IndexEnumerate<I, indexmap::map::IntoKeys<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.into_keys())
    }
    pub fn values(&self) -> indexmap::map::Values<K, V> {
        self.data.values()
    }
    pub fn values_enumerated(
        &self,
    ) -> IndexEnumerate<I, indexmap::map::Values<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.values())
    }
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<K, V> {
        self.data.values_mut()
    }
    pub fn values_mut_enumerated(
        &mut self,
    ) -> IndexEnumerate<I, indexmap::map::ValuesMut<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.values_mut())
    }
    pub fn into_values(self) -> indexmap::map::IntoValues<K, V> {
        self.data.into_values()
    }
    pub fn into_values_emumerated(
        self,
    ) -> IndexEnumerate<I, indexmap::map::IntoValues<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.into_values())
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn truncate(&mut self, end: I)
    where
        I: Idx,
    {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn drain<R: IndexRangeBounds<I>>(
        &mut self,
        range: R,
    ) -> indexmap::map::Drain<K, V> {
        self.data
            .drain(range.canonicalize(self.len()).usize_range())
    }
    pub fn drain_len<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> indexmap::map::Drain<K, V> {
        self.data.drain(range)
    }
    pub fn split_off(&mut self, at: I) -> Self
    where
        S: Clone,
        I: Idx,
    {
        Self::from(self.data.split_off(at.into_usize()))
    }
    pub fn split_off_len(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self::from(self.data.split_off(at))
    }
    pub fn reserve(&mut self, additional: I)
    where
        I: Idx,
    {
        self.data.reserve(additional.into_usize());
    }
    pub fn reserve_len(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.data.insert(key, value)
    }

    pub fn swap_remove<Q: ?Sized + Hash + Equivalent<K>>(
        &mut self,
        key: &Q,
    ) -> Option<V>
    where
        S: BuildHasher,
    {
        self.data.swap_remove(key)
    }
    pub fn as_index_map(&self) -> &IndexMap<K, V, S> {
        &self.data
    }
    pub fn as_mut_index_map(&mut self) -> &mut IndexMap<K, V, S> {
        &mut self.data
    }

    pub fn indices(&self) -> IndexRange<I>
    where
        I: Idx,
    {
        IndexRange::new(I::ZERO..self.len_idx())
    }

    pub fn entry(&mut self, key: K) -> indexmap::map::Entry<'_, K, V>
    where
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.data.entry(key)
    }
}

impl<'a, Idx, K, V, S> Extend<(&'a K, &'a V)> for IndexHashMap<Idx, K, V, S>
where
    K: Hash + Eq + Copy,
    V: Copy,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<I, K, V, S> IntoIterator for IndexHashMap<I, K, V, S> {
    type Item = (K, V);

    type IntoIter = indexmap::map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I, K, V, S> IntoIterator for &'a IndexHashMap<I, K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = indexmap::map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I, K, V, S> IntoIterator for &'a mut IndexHashMap<I, K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = indexmap::map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<I, K: Hash + Eq, V, S: BuildHasher + Default> FromIterator<(K, V)>
    for IndexHashMap<I, K, V, S>
{
    fn from_iter<IT: IntoIterator<Item = (K, V)>>(iter: IT) -> Self {
        Self::from(IndexMap::from_iter(iter))
    }
}

impl<I, K, V, Q: ?Sized, S> Index<&Q> for IndexHashMap<I, K, V, S>
where
    Q: Hash + Equivalent<K>,
    S: BuildHasher,
{
    type Output = V;
    fn index(&self, key: &Q) -> &V {
        self.data.index(key)
    }
}

// TODO: implement slicing shenanegans

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, K, V, S> Serialize for IndexHashMap<I, K, V, S>
where
    IndexMap<K, V, S>: Serialize,
{
    fn serialize<SR: Serializer>(
        &self,
        serializer: SR,
    ) -> Result<SR::Ok, SR::Error> {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, K, V, S> Deserialize<'de> for IndexHashMap<I, K, V, S>
where
    IndexMap<K, V, S>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(IndexMap::deserialize(deserializer)?))
    }
}
