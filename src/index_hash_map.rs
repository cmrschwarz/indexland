use crate::{
    IdxCompat,
    index_enumerate::IndexEnumerate,
    index_range::IndexRangeBounds,
    sequence::{Sequence, SequenceIndex, SequenceMut},
};
use alloc::boxed::Box;
use core::{
    fmt,
    fmt::Debug,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    ops::{Index, IndexMut, RangeBounds},
};

use indexmap::{Equivalent, IndexMap, map::Slice};

use super::{idx::Idx, index_range::IndexRange};

/// Create an [`IndexHashMap`] containing the arguments.
///
/// The syntax is identical to [`indexmap!`](::indexmap::indexmap!).
///
/// The index (and hasher) type cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// # Example
/// ```
/// use indexland::{IndexHashMap, index_hash_map};
/// let map: IndexHashMap<u32, _, _> = index_hash_map! {
///     "a" => 17,
///     "b" => 42,
/// };
/// ```
#[macro_export]
macro_rules! index_hash_map {
    () => {
        $crate::IndexHashMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        const CAP: usize = <[()]>::len(&[$({ stringify!($key); }),*]);
        let mut map = $crate::IndexHashMap::with_capacity(CAP);
        $(
            map.insert($key, $value);
        )*
        map
    }};
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
pub struct IndexSlice<I, K, V> {
    _phantom: PhantomData<fn(I) -> (K, V)>,
    #[allow(unused)]
    data: Slice<K, V>,
}

impl<I, K, V> IndexSlice<I, K, V> {
    #[inline(always)]
    pub fn from_slice(s: &Slice<K, V>) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline(always)]
    pub fn from_mut_slice(s: &mut Slice<K, V>) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Self) }
    }
    #[inline(always)]
    pub fn into_slice(s: &Self) -> &Slice<K, V> {
        unsafe { &*(core::ptr::from_ref(s) as *const Slice<K, V>) }
    }
    #[inline(always)]
    pub fn into_mut_slice(s: &mut Self) -> &mut Slice<K, V> {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Slice<K, V>) }
    }
    pub fn from_boxed_slice(slice_box: Box<Slice<K, V>>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice_box) as *mut Self) }
    }
    pub fn into_boxed_slice(self: Box<Self>) -> Box<Slice<K, V>> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut Slice<K, V>) }
    }
}

impl<'a, I, K, V> From<&'a Slice<K, V>> for &'a IndexSlice<I, K, V> {
    fn from(data: &'a Slice<K, V>) -> Self {
        IndexSlice::from_slice(data)
    }
}
impl<'a, I, K, V> From<&'a IndexSlice<I, K, V>> for &'a Slice<K, V> {
    fn from(data: &'a IndexSlice<I, K, V>) -> Self {
        IndexSlice::into_slice(data)
    }
}
impl<'a, I, K, V> From<&'a mut Slice<K, V>> for &'a mut IndexSlice<I, K, V> {
    fn from(data: &'a mut Slice<K, V>) -> Self {
        IndexSlice::from_mut_slice(data)
    }
}
impl<'a, I, K, V> From<&'a mut IndexSlice<I, K, V>> for &'a mut Slice<K, V> {
    fn from(data: &'a mut IndexSlice<I, K, V>) -> Self {
        IndexSlice::into_mut_slice(data)
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

impl<I, K, V> Sequence for IndexSlice<I, K, V> {
    type Index = I;
    type Element = V;
    type Slice<X: IdxCompat<I>> = IndexSlice<X, K, V>;

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, idx: usize) -> Option<&Self::Element> {
        self.data.get_index(idx).map(|(_k, v)| v)
    }

    fn index(&self, idx: usize) -> &Self::Element {
        &self.data[idx]
    }
    fn get_range<X: IdxCompat<I>>(&self, r: core::ops::Range<usize>) -> Option<&Self::Slice<X>> {
        Some(IndexSlice::from_slice(self.data.get_range(r)?))
    }

    fn index_range<X: IdxCompat<I>>(&self, r: core::ops::Range<usize>) -> &Self::Slice<X> {
        IndexSlice::from_slice(&self.data[r])
    }
}

impl<I, K, V> SequenceMut for IndexSlice<I, K, V> {
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Element> {
        self.data.get_index_mut(idx).map(|(_k, v)| v)
    }

    fn index_mut(&mut self, idx: usize) -> &mut Self::Element {
        &mut self.data[idx]
    }

    fn get_range_mut<X: IdxCompat<Self::Index>>(
        &mut self,
        r: core::ops::Range<usize>,
    ) -> Option<&mut Self::Slice<X>> {
        Some(IndexSlice::from_mut_slice(self.data.get_range_mut(r)?))
    }

    fn index_range_mut<X: IdxCompat<Self::Index>>(
        &mut self,
        r: core::ops::Range<usize>,
    ) -> &mut Self::Slice<X> {
        IndexSlice::from_mut_slice(&mut self.data[r])
    }
}

impl<I, K, V, X> Index<X> for IndexSlice<I, K, V>
where
    X: SequenceIndex<I, IndexSlice<I, K, V>>,
{
    type Output = X::Output;

    fn index(&self, index: X) -> &Self::Output {
        index.index(self)
    }
}

impl<I, K, V, X> IndexMut<X> for IndexSlice<I, K, V>
where
    X: SequenceIndex<I, IndexSlice<I, K, V>>,
{
    fn index_mut(&mut self, index: X) -> &mut Self::Output {
        index.index_mut(self)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, K: Hash + Eq, V, const N: usize> From<[(K, V); N]> for IndexHashMap<I, K, V, RandomState> {
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

impl<I, K, V, S> Debug for IndexHashMap<I, K, V, S>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, K, V, S> IndexHashMap<I, K, V, S> {
    pub fn new() -> Self
    where
        S: Default,
    {
        Self {
            data: IndexMap::default(),
            _phantom: PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self
    where
        S: Default,
    {
        Self {
            data: IndexMap::with_capacity_and_hasher(cap, S::default()),
            _phantom: PhantomData,
        }
    }
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
    pub fn iter(&self) -> indexmap::map::Iter<'_, K, V> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<'_, K, V> {
        self.data.iter_mut()
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, indexmap::map::Iter<'_, K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(&mut self) -> IndexEnumerate<I, indexmap::map::IterMut<'_, K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range<R>(
        &self,
        range: R,
    ) -> IndexEnumerate<I, indexmap::map::Iter<'_, K, V>>
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
    ) -> IndexEnumerate<I, indexmap::map::IterMut<'_, K, V>>
    where
        I: Idx,
        R: IndexRangeBounds<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn into_iter_enumerated(self) -> IndexEnumerate<I, indexmap::map::IntoIter<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn keys(&self) -> indexmap::map::Keys<'_, K, V> {
        self.data.keys()
    }
    pub fn keys_enumerated(&self) -> IndexEnumerate<I, indexmap::map::Keys<'_, K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.keys())
    }
    pub fn into_keys(self) -> indexmap::map::IntoKeys<K, V> {
        self.data.into_keys()
    }
    pub fn into_keys_enumerated(self) -> IndexEnumerate<I, indexmap::map::IntoKeys<K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.into_keys())
    }
    pub fn values(&self) -> indexmap::map::Values<'_, K, V> {
        self.data.values()
    }
    pub fn values_enumerated(&self) -> IndexEnumerate<I, indexmap::map::Values<'_, K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.values())
    }
    pub fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, K, V> {
        self.data.values_mut()
    }
    pub fn values_mut_enumerated(&mut self) -> IndexEnumerate<I, indexmap::map::ValuesMut<'_, K, V>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data.values_mut())
    }
    pub fn into_values(self) -> indexmap::map::IntoValues<K, V> {
        self.data.into_values()
    }
    pub fn into_values_emumerated(self) -> IndexEnumerate<I, indexmap::map::IntoValues<K, V>>
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
    pub fn drain<R: IndexRangeBounds<I>>(&mut self, range: R) -> indexmap::map::Drain<'_, K, V> {
        self.data
            .drain(range.canonicalize(self.len()).usize_range())
    }
    pub fn drain_len<R: RangeBounds<usize>>(&mut self, range: R) -> indexmap::map::Drain<'_, K, V> {
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
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.data.insert(key, value)
    }

    pub fn get_index(&self, i: I) -> Option<(&K, &V)>
    where
        I: Idx,
    {
        self.data.get_index(i.into_usize())
    }

    pub fn get_index_mut(&mut self, i: I) -> Option<(&K, &mut V)>
    where
        I: Idx,
    {
        self.data.get_index_mut(i.into_usize())
    }

    pub fn swap_remove<Q: ?Sized + Hash + Equivalent<K>>(&mut self, key: &Q) -> Option<V>
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

    pub fn entry(&mut self, key: K) -> Entry<'_, I, K, V>
    where
        K: Hash + Eq,
        S: BuildHasher,
    {
        match self.data.entry(key) {
            indexmap::map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry {
                data: entry,
                _phantom: PhantomData,
            }),
            indexmap::map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                data: entry,
                _phantom: PhantomData,
            }),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        S: BuildHasher,
    {
        self.data.get(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
        S: BuildHasher,
    {
        self.data.get_mut(key)
    }

    /// Return item index, if it exists in the map
    pub fn get_index_of<Q>(&self, key: &Q) -> Option<I>
    where
        I: Idx,
        Q: ?Sized + Hash + Equivalent<K>,
        S: BuildHasher,
    {
        self.data.get_index_of(key).map(I::from_usize)
    }

    /// Return item index, key and value
    pub fn get_full<Q>(&self, key: &Q) -> Option<(I, &K, &V)>
    where
        I: Idx,
        Q: ?Sized + Hash + Equivalent<K>,
        S: BuildHasher,
    {
        let (idx, key, value) = self.data.get_full(key)?;
        Some((I::from_usize(idx), key, value))
    }

    pub fn swap_indices(&mut self, a: I, b: I)
    where
        I: Idx,
    {
        self.data.swap_indices(a.into_usize(), b.into_usize());
    }

    #[track_caller]
    pub fn move_index(&mut self, from: I, to: I)
    where
        I: Idx,
    {
        self.data.move_index(from.into_usize(), to.into_usize());
    }

    pub fn get_index_entry(&mut self, index: I) -> Option<IndexedEntry<'_, I, K, V>>
    where
        I: Idx,
    {
        if index >= self.len_idx() {
            return None;
        }
        Some(IndexedEntry {
            data: self.data.get_index_entry(index.into_usize())?,
            _phantom: PhantomData,
        })
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

impl<K, I1, V1, S1, I2, V2, S2> PartialEq<IndexHashMap<I2, K, V2, S2>>
    for IndexHashMap<I1, K, V1, S1>
where
    K: Hash + Eq,
    V1: PartialEq<V2>,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &IndexHashMap<I2, K, V2, S2>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).is_some_and(|v| *value == *v))
    }
}

impl<K, I, V, S> Eq for IndexHashMap<I, K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, K, V, S> Serialize for IndexHashMap<I, K, V, S>
where
    IndexMap<K, V, S>: Serialize,
{
    fn serialize<SR: Serializer>(&self, serializer: SR) -> Result<SR::Ok, SR::Error> {
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

// ========== Entry ==========

/// Entry for an existing key-value pair in an [`IndexHashMap`]
/// or a vacant location to insert one.
pub enum Entry<'a, I, K, V> {
    /// Existing slot with equivalent key.
    Occupied(OccupiedEntry<'a, I, K, V>),
    /// Vacant slot (no equivalent key in the map).
    Vacant(VacantEntry<'a, I, K, V>),
}

impl<'a, I, K, V> Entry<'a, I, K, V> {
    /// Return the index where the key-value pair exists or will be inserted.
    pub fn index(&self) -> I
    where
        I: Idx,
    {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }

    /// Sets the value of the entry (after inserting if vacant), and returns an `OccupiedEntry`.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, I, K, V> {
        match self {
            Entry::Occupied(mut entry) => {
                _ = entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }

    /// Inserts the given default value in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Inserts the result of the `call` function in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert_with<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(call()),
        }
    }

    /// Inserts the result of the `call` function with a reference to the entry's key if it is
    /// vacant, and returns a mutable reference to the new value. Otherwise a mutable reference to
    /// an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_insert_with_key<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = call(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Gets a reference to the entry's key, either within the map if occupied,
    /// or else the new key that was used to find the entry.
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Modifies the entry if it is occupied.
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }
        self
    }

    /// Inserts a default-constructed value in the entry if it is vacant and returns a mutable
    /// reference to it. Otherwise a mutable reference to an already existent value is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<I, K: fmt::Debug, V: fmt::Debug> fmt::Debug for Entry<'_, I, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("Entry");
        match self {
            Entry::Vacant(v) => tuple.field(v),
            Entry::Occupied(o) => tuple.field(o),
        }
        .finish()
    }
}

/// A view into an occupied entry in an [`IndexHashMap`].
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, I, K, V> {
    data: indexmap::map::OccupiedEntry<'a, K, V>,
    _phantom: PhantomData<I>,
}

impl<'a, I, K, V> OccupiedEntry<'a, I, K, V> {
    /// Return the index of the key-value pair
    #[inline]
    pub fn index(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.index())
    }

    //TODO:
    // #[inline]
    // fn into_ref_mut(self) -> RefMut<'a, K, V> {
    //     todo!()
    // }

    /// Gets a reference to the entry's key in the map.
    ///
    /// Note that this is not the key that was used to find the entry. There may be an observable
    /// difference if the key type has any distinguishing features outside of `Hash` and `Eq`, like
    /// extra fields or the memory address of an allocation.
    #[inline(always)]
    pub fn key(&self) -> &K {
        self.data.key()
    }

    /// Gets a reference to the entry's value in the map.
    #[inline(always)]
    pub fn get(&self) -> &V {
        self.data.get()
    }

    /// Gets a mutable reference to the entry's value in the map.
    ///
    /// If you need a reference which may outlive the destruction of the
    /// [`Entry`] value, see [`into_mut`][Self::into_mut].
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut V {
        self.data.get_mut()
    }

    /// Converts into a mutable reference to the entry's value in the map,
    /// with a lifetime bound to the map itself.
    #[inline(always)]
    pub fn into_mut(self) -> &'a mut V {
        self.data.into_mut()
    }

    /// Sets the value of the entry to `value`, and returns the entry's old value.
    #[inline(always)]
    pub fn insert(&mut self, value: V) -> V {
        self.data.insert(value)
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::swap_remove`][std::vec::Vec::swap_remove], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    #[inline(always)]
    pub fn swap_remove(self) -> V {
        self.data.swap_remove()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::remove`][std::vec::Vec::remove], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    #[inline(always)]
    pub fn shift_remove(self) -> V {
        self.data.shift_remove()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::swap_remove`][std::vec::Vec::swap_remove], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    #[inline(always)]
    pub fn swap_remove_entry(self) -> (K, V) {
        self.data.swap_remove_entry()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::remove`][std::vec::Vec::remove], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    pub fn shift_remove_entry(self) -> (K, V) {
        self.data.shift_remove_entry()
    }

    /// Moves the position of the entry to a new index
    /// by shifting all other entries in-between.
    ///
    /// This is equivalent to [`IndexHashMap::move_index`][`crate::IndexHashMap::move_index`]
    /// coming `from` the current [`.index()`][Self::index].
    ///
    /// * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.
    /// * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.
    ///
    /// ***Panics*** if `to` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    #[track_caller]
    #[inline(always)]
    pub fn move_index(self, to: I)
    where
        I: Idx,
    {
        self.data.move_index(to.into_usize());
    }

    /// Swaps the position of entry with another.
    ///
    /// This is equivalent to [`IndexHashMap::swap_indices`][`crate::IndexHashMap::swap_indices`]
    /// with the current [`.index()`][Self::index] as one of the two being swapped.
    ///
    /// ***Panics*** if the `other` index is out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    #[track_caller]
    #[inline(always)]
    pub fn swap_indices(self, other: I)
    where
        I: Idx,
    {
        self.data.swap_indices(other.into_usize());
    }
}

impl<I, K: fmt::Debug, V: fmt::Debug> fmt::Debug for OccupiedEntry<'_, I, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

//TODO
/*
impl<'a, K, V> From<IndexedEntry<'a, K, V>> for OccupiedEntry<'a, K, V> {
    fn from(other: IndexedEntry<'a, K, V>) -> Self {
        let IndexedEntry {
            map: RefMut { indices, entries },
            index,
        } = other;
        let hash = entries[index].hash;
        Self {
            entries,
            index: indices
                .find_entry(hash.get(), move |&i| i == index)
                .expect("index not found"),
        }
    }
}
*/

/// A view into a vacant entry in an [`IndexHashMap`].
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, I, K, V> {
    data: indexmap::map::VacantEntry<'a, K, V>,
    _phantom: PhantomData<I>,
}

impl<'a, I, K, V> VacantEntry<'a, I, K, V> {
    /// Return the index where a key-value pair may be inserted.
    #[inline(always)]
    pub fn index(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.index())
    }

    /// Gets a reference to the key that was used to find the entry.
    #[inline(always)]
    pub fn key(&self) -> &K {
        self.data.key()
    }

    /// Takes ownership of the key, leaving the entry vacant.
    #[inline(always)]
    pub fn into_key(self) -> K {
        self.data.into_key()
    }

    /// Inserts the entry's key and the given value into the map, and returns a mutable reference
    /// to the value.
    ///
    /// Computes in **O(1)** time (amortized average).
    #[inline(always)]
    pub fn insert(self, value: V) -> &'a mut V {
        self.data.insert(value)
    }

    /// Inserts the entry's key and the given value into the map, and returns an `OccupiedEntry`.
    ///
    /// Computes in **O(1)** time (amortized average).
    #[inline(always)]
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, I, K, V> {
        OccupiedEntry {
            data: self.data.insert_entry(value),
            _phantom: PhantomData,
        }
    }

    /// Inserts the entry's key and the given value into the map at its ordered
    /// position among sorted keys, and returns the new index and a mutable
    /// reference to the value.
    ///
    /// If the existing keys are **not** already sorted, then the insertion
    /// index is unspecified (like [`slice::binary_search`]), but the key-value
    /// pair is inserted at that position regardless.
    ///
    /// Computes in **O(n)** time (average).
    pub fn insert_sorted(self, value: V) -> (I, &'a mut V)
    where
        I: Idx,
        K: Ord,
    {
        let (idx, v) = self.data.insert_sorted(value);
        (I::from_usize(idx), v)
    }

    /// Inserts the entry's key and the given value into the map at the given index,
    /// shifting others to the right, and returns a mutable reference to the value.
    ///
    /// ***Panics*** if `index` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    #[track_caller]
    #[inline(always)]
    pub fn shift_insert(self, index: I, value: V) -> &'a mut V
    where
        I: Idx,
    {
        self.data.shift_insert(index.into_usize(), value)
    }
}

impl<I, K: fmt::Debug, V> fmt::Debug for VacantEntry<'_, I, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

/// A view into an occupied entry in an [`IndexHashMap`] obtained by index.
///
/// This `struct` is created from the [`get_index_entry`][crate::IndexHashMap::get_index_entry] method.
pub struct IndexedEntry<'a, I, K, V> {
    data: indexmap::map::IndexedEntry<'a, K, V>,
    _phantom: PhantomData<I>,
}

impl<'a, I, K, V> IndexedEntry<'a, I, K, V> {
    /// Return the index of the key-value pair
    #[inline(always)]
    pub fn index(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.data.index())
    }

    /// Gets a reference to the entry's key in the map.
    #[inline(always)]
    pub fn key(&self) -> &K {
        self.data.key()
    }

    /// Gets a reference to the entry's value in the map.
    #[inline(always)]
    pub fn get(&self) -> &V {
        self.data.get()
    }

    /// Gets a mutable reference to the entry's value in the map.
    ///
    /// If you need a reference which may outlive the destruction of the
    /// `IndexedEntry` value, see [`into_mut`][Self::into_mut].
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut V {
        self.data.get_mut()
    }

    /// Sets the value of the entry to `value`, and returns the entry's old value.
    #[inline(always)]
    pub fn insert(&mut self, value: V) -> V {
        self.data.insert(value)
    }

    /// Converts into a mutable reference to the entry's value in the map,
    /// with a lifetime bound to the map itself.
    #[inline(always)]
    pub fn into_mut(self) -> &'a mut V {
        self.data.into_mut()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::swap_remove`][std::vec::Vec::swap_remove], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    #[inline(always)]
    pub fn swap_remove_entry(self) -> (K, V) {
        self.data.swap_remove_entry()
    }

    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// Like [`Vec::remove`][std::vec::Vec::remove], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    #[inline(always)]
    pub fn shift_remove_entry(self) -> (K, V) {
        self.data.shift_remove_entry()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::swap_remove`][std::vec::Vec::swap_remove], the pair is removed by swapping it with
    /// the last element of the map and popping it off.
    /// **This perturbs the position of what used to be the last element!**
    ///
    /// Computes in **O(1)** time (average).
    #[inline(always)]
    pub fn swap_remove(self) -> V {
        self.data.swap_remove()
    }

    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// Like [`Vec::remove`][std::vec::Vec::remove], the pair is removed by shifting all of the
    /// elements that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**
    ///
    /// Computes in **O(n)** time (average).
    #[inline(always)]
    pub fn shift_remove(self) -> V {
        self.data.shift_remove()
    }

    /// Moves the position of the entry to a new index
    /// by shifting all other entries in-between.
    ///
    /// This is equivalent to [`IndexHashMap::move_index`][`crate::IndexHashMap::move_index`]
    /// coming `from` the current [`.index()`][Self::index].
    ///
    /// * If `self.index() < to`, the other pairs will shift down while the targeted pair moves up.
    /// * If `self.index() > to`, the other pairs will shift up while the targeted pair moves down.
    ///
    /// ***Panics*** if `to` is out of bounds.
    ///
    /// Computes in **O(n)** time (average).
    #[track_caller]
    #[inline(always)]
    pub fn move_index(self, to: I)
    where
        I: Idx,
    {
        self.data.move_index(to.into_usize());
    }

    /// Swaps the position of entry with another.
    ///
    /// This is equivalent to [`IndexHashMap::swap_indices`][`crate::IndexHashMap::swap_indices`]
    /// with the current [`.index()`][Self::index] as one of the two being swapped.
    ///
    /// ***Panics*** if the `other` index is out of bounds.
    ///
    /// Computes in **O(1)** time (average).
    #[track_caller]
    #[inline(always)]
    pub fn swap_indices(self, other: I)
    where
        I: Idx,
    {
        self.data.swap_indices(other.into_usize());
    }
}

impl<I, K: fmt::Debug, V: fmt::Debug> fmt::Debug for IndexedEntry<'_, I, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<'a, I, K, V> From<OccupiedEntry<'a, I, K, V>> for IndexedEntry<'a, I, K, V> {
    fn from(other: OccupiedEntry<'a, I, K, V>) -> Self {
        Self {
            data: other.data.into(),
            _phantom: PhantomData,
        }
    }
}
