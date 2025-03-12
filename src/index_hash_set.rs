use super::{idx::Idx, index_range::IndexRange};
use crate::{index_enumerate::IndexEnumerate, IndexRangeBounds};
use alloc::boxed::Box;
use core::{
    fmt::Debug,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    ops::Index,
};

use indexmap::{set::Slice, Equivalent, IndexSet};

#[cfg(feature = "std")]
use std::hash::RandomState;

/// Create an [`IndexHashSet`] containing the arguments.
///
/// The syntax is identical to [`indexset!`](::indexmap::indexset!).
///
/// The index type cannot be inferred from the macro so you
/// might have to add type annotations.
///
/// # Example
/// ```
/// use indexland::{index_hash_set, IndexHashSet};
/// let set: IndexHashSet<u32, _> = index_hash_set! {
///     "a",
///     "b",
/// };
/// ```
#[macro_export]
macro_rules! index_hash_set {
    ($($anything: tt)+) => {
        $crate::IndexHashSet::from(::indexmap::indexset![$($anything)+])
    };
}

#[cfg(feature = "std")]
#[derive(Clone)]
pub struct IndexHashSet<I, T, S = RandomState> {
    data: IndexSet<T, S>,
    _phantom: PhantomData<fn(I) -> T>,
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct IndexHashSet<I, T, S> {
    data: IndexSet<T, S>,
    _phantom: PhantomData<fn(I) -> T>,
}

pub struct IndexHashSetSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    #[allow(unused)]
    data: Slice<T>,
}

impl<I, T> IndexHashSetSlice<I, T> {
    #[inline]
    pub fn from_index_map_slice(s: &Slice<T>) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    pub fn from_index_map_slice_mut(s: &mut Slice<T>) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Self) }
    }
    #[inline]
    pub fn into_index_map_slice(s: &Self) -> &Slice<T> {
        unsafe { &*(core::ptr::from_ref(s) as *const Slice<T>) }
    }
    #[inline]
    pub fn into_index_map_slice_mut(s: &mut Self) -> &mut Slice<T> {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Slice<T>) }
    }
    pub fn from_boxed_slice(slice_box: Box<Slice<T>>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice_box) as *mut Self) }
    }
    pub fn into_boxed_slice(self: Box<Self>) -> Box<Slice<T>> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut Slice<T>) }
    }
}

impl<'a, I, T> From<&'a Slice<T>> for &'a IndexHashSetSlice<I, T> {
    fn from(data: &'a Slice<T>) -> Self {
        IndexHashSetSlice::from_index_map_slice(data)
    }
}
impl<'a, I, T> From<&'a IndexHashSetSlice<I, T>> for &'a Slice<T> {
    fn from(data: &'a IndexHashSetSlice<I, T>) -> Self {
        IndexHashSetSlice::into_index_map_slice(data)
    }
}
impl<'a, I, T> From<&'a mut Slice<T>> for &'a mut IndexHashSetSlice<I, T> {
    fn from(data: &'a mut Slice<T>) -> Self {
        IndexHashSetSlice::from_index_map_slice_mut(data)
    }
}
impl<'a, I, T> From<&'a mut IndexHashSetSlice<I, T>> for &'a mut Slice<T> {
    fn from(data: &'a mut IndexHashSetSlice<I, T>) -> Self {
        IndexHashSetSlice::into_index_map_slice_mut(data)
    }
}

impl<I, T, S> From<IndexSet<T, S>> for IndexHashSet<I, T, S> {
    fn from(v: IndexSet<T, S>) -> Self {
        Self {
            data: v,
            _phantom: PhantomData,
        }
    }
}
impl<I, T, S> From<IndexHashSet<I, T, S>> for IndexSet<T, S> {
    fn from(v: IndexHashSet<I, T, S>) -> Self {
        v.data
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, T: Hash + Eq, const N: usize> From<[T; N]>
    for IndexHashSet<I, T, RandomState>
{
    fn from(arr: [T; N]) -> IndexHashSet<I, T, RandomState> {
        IndexHashSet::from(IndexSet::from(arr))
    }
}

impl<I, T, S: Default> Default for IndexHashSet<I, T, S> {
    fn default() -> Self {
        Self {
            data: IndexSet::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T: Debug, S> Debug for IndexHashSet<I, T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, T> IndexHashSet<I, T> {
    /// This is not const because the default [`RandomState`] used for the hasher
    /// might get random bits from the OS.
    /// Look at [`IndexHashSet::with_hasher`] for a const constructor for this.
    pub fn new() -> Self {
        Self {
            data: IndexSet::new(),
            _phantom: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: IndexSet::with_capacity(cap),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T: Hash + Eq, S: BuildHasher> IndexHashSet<I, T, S> {
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            data: IndexSet::with_hasher(hash_builder),
            _phantom: PhantomData,
        }
    }
    pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
        Self {
            data: IndexSet::with_capacity_and_hasher(cap, hasher),
            _phantom: PhantomData,
        }
    }
    pub fn reserve(&mut self, additional: I) {
        self.data.reserve(additional.into_usize());
    }
    pub fn reserve_len(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn insert(&mut self, value: T) -> bool {
        self.data.insert(value)
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn len_idx(&self) -> I {
        I::from_usize(self.data.len())
    }
    pub fn last_idx(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub fn truncate(&mut self, end: I) {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn swap_remove<Q: ?Sized + Hash + Equivalent<T>>(
        &mut self,
        key: &T,
    ) -> bool {
        self.data.swap_remove(key)
    }
    pub fn as_index_set(&self) -> &IndexSet<T, S> {
        &self.data
    }
    pub fn as_index_set_mut(&mut self) -> &mut IndexSet<T, S> {
        &mut self.data
    }
    pub fn iter_enumerated(
        &self,
    ) -> IndexEnumerate<I, indexmap::set::Iter<T>> {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_range(
        &self,
        range: impl IndexRangeBounds<I>,
    ) -> IndexEnumerate<I, indexmap::set::Iter<T>> {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &self.data[range])
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, indexmap::set::IntoIter<T>> {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn indices(&self) -> IndexRange<I> {
        IndexRange::new(I::ZERO..self.len_idx())
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn iter(&self) -> indexmap::set::Iter<T> {
        self.data.iter()
    }
    // TODO: wrap more IndexSet stuff
}

impl<'a, Idx, T, S> Extend<&'a T> for IndexHashSet<Idx, T, S>
where
    T: Hash + Eq + Copy,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<I, T, S> IntoIterator for IndexHashSet<I, T, S> {
    type Item = T;

    type IntoIter = indexmap::set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I, T, S> IntoIterator for &'a IndexHashSet<I, T, S> {
    type Item = &'a T;
    type IntoIter = indexmap::set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<I, T: Hash + Eq, S: BuildHasher + Default> FromIterator<T>
    for IndexHashSet<I, T, S>
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(IndexSet::from_iter(iter))
    }
}

impl<I: Idx, T, S: BuildHasher> Index<I> for IndexHashSet<I, T, S> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        self.data.index(idx.into_usize())
    }
}

// TODO: implement slicing shenanegans

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, T, S> Serialize for IndexHashSet<I, T, S>
where
    IndexSet<T, S>: Serialize,
{
    fn serialize<SR: Serializer>(
        &self,
        serializer: SR,
    ) -> Result<SR::Ok, SR::Error> {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T, S> Deserialize<'de> for IndexHashSet<I, T, S>
where
    IndexSet<T, S>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(IndexSet::deserialize(deserializer)?))
    }
}
