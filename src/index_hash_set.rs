use super::{idx::Idx, index_range::IndexRange};
use crate::{index_enumerate::IndexEnumerate, IndexRangeBounds};
use alloc::boxed::Box;
use core::{
    cmp::Ordering,
    fmt::Debug,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXor, Deref, Index},
};

use indexmap::{
    set::{
        Difference, Intersection, Slice, Splice, SymmetricDifference, Union,
    },
    Equivalent, IndexSet, TryReserveError,
};

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
    ($($anything: tt)*) => {
        $crate::IndexHashSet::from(::indexmap::indexset![$($anything)*])
    };
}

#[cfg(feature = "std")]
pub struct IndexHashSet<I, T, S = RandomState> {
    data: IndexSet<T, S>,
    _phantom: PhantomData<fn(I) -> T>,
}

#[cfg(not(feature = "std"))]
pub struct IndexHashSet<I, T, S> {
    data: IndexSet<T, S>,
    _phantom: PhantomData<fn(I) -> T>,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexSlice<I, T> {
    _phantom: PhantomData<fn(I) -> T>,
    #[allow(unused)]
    data: Slice<T>,
}

impl<I, T> IndexSlice<I, T> {
    #[inline]
    fn from_slice(s: &Slice<T>) -> &Self {
        unsafe { &*(core::ptr::from_ref(s) as *const Self) }
    }
    #[inline]
    fn from_slice_mut(s: &mut Slice<T>) -> &mut Self {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Self) }
    }
    #[inline]
    fn into_slice(s: &Self) -> &Slice<T> {
        unsafe { &*(core::ptr::from_ref(s) as *const Slice<T>) }
    }
    #[inline]
    fn into_slice_mut(s: &mut Self) -> &mut Slice<T> {
        unsafe { &mut *(core::ptr::from_mut(s) as *mut Slice<T>) }
    }
    fn from_boxed_slice(slice_box: Box<Slice<T>>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice_box) as *mut Self) }
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
    pub fn get_index(&self, index: I) -> Option<&T>
    where
        I: Idx,
    {
        self.data.get_index(index.into_usize())
    }

    pub fn get_range<R: IndexRangeBounds<I>>(
        &self,
        range: R,
    ) -> Option<&Self> {
        Some(Self::from_slice(
            self.data.get_range(range.canonicalize(self.len()))?,
        ))
    }

    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn split_at(&self, index: I) -> (&Self, &Self)
    where
        I: Idx,
    {
        let (a, b) = self.data.split_at(index.into_usize());

        (Self::from_slice(a), Self::from_slice(b))
    }

    pub fn split_first(&self) -> Option<(&T, &Self)> {
        let (first, rest) = self.data.split_first()?;
        Some((first, Self::from_slice(rest)))
    }

    pub fn split_last(&self) -> Option<(&T, &Self)> {
        let (first, rest) = self.data.split_last()?;
        Some((first, Self::from_slice(rest)))
    }

    pub fn iter(&self) -> indexmap::set::Iter<T> {
        self.data.iter()
    }

    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.data.binary_search(x)
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.data.binary_search_by(f)
    }

    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        f: F,
    ) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.data.binary_search_by_key(b, f)
    }

    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.data.partition_point(pred)
    }
}

impl<'a, I, T> From<&'a Slice<T>> for &'a IndexSlice<I, T> {
    fn from(data: &'a Slice<T>) -> Self {
        IndexSlice::from_slice(data)
    }
}
impl<'a, I, T> From<&'a IndexSlice<I, T>> for &'a Slice<T> {
    fn from(data: &'a IndexSlice<I, T>) -> Self {
        IndexSlice::into_slice(data)
    }
}
impl<'a, I, T> From<&'a mut Slice<T>> for &'a mut IndexSlice<I, T> {
    fn from(data: &'a mut Slice<T>) -> Self {
        IndexSlice::from_slice_mut(data)
    }
}
impl<'a, I, T> From<&'a mut IndexSlice<I, T>> for &'a mut Slice<T> {
    fn from(data: &'a mut IndexSlice<I, T>) -> Self {
        IndexSlice::into_slice_mut(data)
    }
}

impl<'a, I, T> IntoIterator for &'a IndexSlice<I, T> {
    type Item = &'a T;

    type IntoIter = indexmap::set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<I: Idx, T> Index<I> for IndexSlice<I, T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

macro_rules! range_impls {
    ($($range: path),* $(,)?) => {$(
        impl<I: Idx, T> Index<$range> for IndexSlice<I, T> {
            type Output = IndexSlice<I, T>;
            fn index(&self, index: $range) -> &Self::Output {
                let range = IndexRangeBounds::<I>::canonicalize(index, self.len());
                IndexSlice::from_slice(&self.data[range])
            }
        }
    )*};
}
range_impls![
    core::ops::Range<I>,
    core::ops::RangeInclusive<I>,
    core::ops::RangeFrom<I>,
    core::ops::RangeTo<I>,
    core::ops::RangeToInclusive<I>,
    core::ops::RangeFull,
    indexland::IndexRange<I>,
    indexland::IndexRangeInclusive<I>,
    indexland::IndexRangeFrom<I>,
];

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<I, T> IndexHashSet<I, T> {
    /// This is not `const` because the default [`RandomState`] used for the
    /// hasher might need to get random bits from the OS.
    /// You can use [`IndexHashSet::with_hasher`] as a const constructor.
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

impl<I: Idx, T, S> IndexHashSet<I, T, S> {
    pub fn with_capacity_and_hasher(cap: usize, hasher: S) -> Self {
        Self {
            data: IndexSet::with_capacity_and_hasher(cap, hasher),
            _phantom: PhantomData,
        }
    }
    pub const fn with_hasher(hash_builder: S) -> Self {
        Self {
            data: IndexSet::with_hasher(hash_builder),
            _phantom: PhantomData,
        }
    }
    pub fn capacity(&mut self) -> usize {
        self.data.capacity()
    }
    pub fn capacity_idx(&mut self) -> I {
        I::from_usize(self.data.capacity())
    }
    pub fn hasher(&self) -> &S {
        self.data.hasher()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn len_idx(&self) -> I {
        I::from_usize(self.data.len())
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn last_idx(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::from_usize)
    }
    pub fn iter(&self) -> indexmap::set::Iter<T> {
        self.data.iter()
    }
    pub fn iter_enumerated(
        &self,
    ) -> IndexEnumerate<I, indexmap::set::Iter<T>> {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_range<R: IndexRangeBounds<I>>(
        &self,
        range: R,
    ) -> IndexEnumerate<I, indexmap::set::Iter<T>> {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &self.data[range])
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IndexEnumerate<I, indexmap::set::IntoIter<T>> {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn truncate(&mut self, end: I) {
        self.data.truncate(end.into_usize());
    }
    pub fn truncate_len(&mut self, len: usize) {
        self.data.truncate(len);
    }
    pub fn drain<R: IndexRangeBounds<I>>(
        &mut self,
        range: R,
    ) -> indexmap::set::Drain<T> {
        self.data.drain(range.canonicalize(self.len()))
    }
    pub fn drain_enumerated<R: IndexRangeBounds<I>>(
        &mut self,
        range: R,
    ) -> IndexEnumerate<I, indexmap::set::Drain<T>> {
        let start = match range.start_bound() {
            core::ops::Bound::Included(s) => *s,
            core::ops::Bound::Excluded(e) => e.saturating_add(I::ONE),
            core::ops::Bound::Unbounded => I::ZERO,
        };
        IndexEnumerate::new(start, self.drain(range))
    }
    pub fn split_off(&mut self, at: I) -> Self
    where
        S: Clone,
    {
        Self::from(self.data.split_off(at.into_usize()))
    }
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }
    pub fn reserve_total(&mut self, capacity_idx_min: I) {
        self.data.reserve(capacity_idx_min.into_usize());
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional.into_usize());
    }
    pub fn reserve_exact_total(&mut self, capacity_idx: I) {
        self.data.reserve_exact(capacity_idx.into_usize());
    }
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        self.data.try_reserve(additional)
    }
    pub fn try_reserve_total(
        &mut self,
        capacity_idx_min: I,
    ) -> Result<(), TryReserveError> {
        self.data.try_reserve(capacity_idx_min.into_usize())
    }
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(additional)
    }
    pub fn try_reserve_exact_total(
        &mut self,
        capacity_idx: I,
    ) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(capacity_idx.into_usize())
    }
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    pub fn shrink_to(&mut self, min_cap_idx: I) {
        self.data.shrink_to(min_cap_idx.into_usize());
    }
    pub fn shrink_to_len(&mut self, min_cap: usize) {
        self.data.shrink_to(min_cap);
    }
    pub fn insert(&mut self, value: T) -> bool
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        self.data.insert(value)
    }
    pub fn insert_full(&mut self, value: T) -> (I, bool)
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        let (idx, newly_inserted) = self.data.insert_full(value);
        (I::from_usize(idx), newly_inserted)
    }
    pub fn insert_sorted(&mut self, value: T) -> (I, bool)
    where
        T: Ord + Hash,
        S: BuildHasher,
    {
        let (idx, newly_inserted) = self.data.insert_sorted(value);
        (I::from_usize(idx), newly_inserted)
    }
    pub fn insert_before(&mut self, idx: I, value: T) -> (I, bool)
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        let (idx, newly_inserted) =
            self.data.insert_before(idx.into_usize(), value);
        (I::from_usize(idx), newly_inserted)
    }
    pub fn shift_insert(&mut self, idx: I, value: T) -> (I, bool)
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        let (idx, newly_inserted) =
            self.data.insert_before(idx.into_usize(), value);
        (I::from_usize(idx), newly_inserted)
    }
    pub fn replace(&mut self, value: T) -> Option<T>
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        self.data.replace(value)
    }
    pub fn replace_full(&mut self, value: T) -> (I, Option<T>)
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        let (idx, prev) = self.data.replace_full(value);
        (I::from_usize(idx), prev)
    }

    /// NOTE: in case you need the `difference` of an `IndexHashSet`
    /// with an `IndexSet` you can use `index_hash_set.as_index_set_mut().difference(index_set)`
    pub fn difference<'a, I2, S2>(
        &'a self,
        other: &'a IndexHashSet<I2, T, S2>,
    ) -> Difference<'a, T, S2>
    where
        T: Hash + Eq,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.difference(&other.data)
    }

    /// NOTE: in case you need the `symmetric_difference` of an `IndexHashSet`
    /// with an `IndexSet` you can use `index_hash_set.as_index_set_mut().symmetric_difference(index_set)`
    pub fn symmetric_difference<'a, I2, S2>(
        &'a self,
        other: &'a IndexHashSet<I2, T, S2>,
    ) -> SymmetricDifference<'a, T, S, S2>
    where
        T: Hash + Eq,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.symmetric_difference(&other.data)
    }

    /// NOTE: in case you need to intersect an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().intersection(index_set)`
    pub fn intersection<'a, I2, S2>(
        &'a self,
        other: &'a IndexHashSet<I2, T, S2>,
    ) -> Intersection<'a, T, S2>
    where
        T: Hash + Eq,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.intersection(&other.data)
    }

    /// NOTE: in case you need to `union` an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().union(index_set)`
    pub fn union<'a, I2, S2>(
        &'a self,
        other: &'a IndexHashSet<I2, T, S2>,
    ) -> Union<'a, T, S>
    where
        T: Hash + Eq,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.union(&other.data)
    }

    pub fn splice<R: IndexRangeBounds<I>, II, S2>(
        &mut self,
        range: R,
        replace_with: II,
    ) -> Splice<II::IntoIter, T, S>
    where
        T: Hash + Eq,
        S: BuildHasher,
        S2: BuildHasher,
        II: IntoIterator<Item = T>,
    {
        self.data
            .splice(range.canonicalize(self.len()), replace_with)
    }

    /// NOTE: to `append` to an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().append(index_set)`
    pub fn append<I2, S2>(&mut self, other: &mut IndexHashSet<I2, T, S2>)
    where
        T: Hash + Eq,
        S: BuildHasher,
    {
        self.data.append(&mut other.data);
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.contains(value)
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.get(value)
    }

    pub fn get_full<Q>(&self, value: &Q) -> Option<(I, &T)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data
            .get_full(value)
            .map(|(i, v)| (I::from_usize(i), v))
    }

    pub fn get_index_of<Q>(&self, value: &Q) -> Option<I>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.get_index_of(value).map(I::from_usize)
    }

    pub fn swap_remove<Q>(&mut self, key: &Q) -> bool
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.swap_remove(key)
    }

    pub fn shift_remove<Q>(&mut self, key: &Q) -> bool
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.shift_remove(key)
    }

    pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.swap_take(value)
    }

    pub fn shift_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data.shift_take(value)
    }

    pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(I, T)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data
            .swap_remove_full(value)
            .map(|(i, v)| (I::from_usize(i), v))
    }

    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(I, T)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.data
            .shift_remove_full(value)
            .map(|(i, v)| (I::from_usize(i), v))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.data.retain(keep);
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.data.sort();
    }

    pub fn sort_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sort_by(cmp);
    }

    pub fn sorted_by<F>(self, cmp: F) -> indexmap::set::IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sorted_by(cmp)
    }

    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.data.sort_unstable();
    }

    pub fn sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sort_unstable_by(cmp);
    }

    pub fn sorted_unstable_by<F>(self, cmp: F) -> indexmap::set::IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sorted_unstable_by(cmp)
    }

    pub fn sort_by_cached_key<K, F>(&mut self, sort_key: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.data.sort_by_cached_key(sort_key);
    }

    pub fn binary_search(&self, x: &T) -> Result<I, I>
    where
        T: Ord,
    {
        self.data
            .binary_search(x)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<I, I>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        self.data
            .binary_search_by(f)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        f: F,
    ) -> Result<I, I>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.data
            .binary_search_by_key(b, f)
            .map(I::from_usize)
            .map_err(I::from_usize)
    }

    pub fn partition_point<P>(&self, pred: P) -> I
    where
        P: FnMut(&T) -> bool,
    {
        I::from_usize(self.data.partition_point(pred))
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn as_slice(&self) -> &Slice<T> {
        self.data.as_slice()
    }

    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(self.data.as_slice())
    }

    pub fn into_boxed_slice(self) -> Box<Slice<T>> {
        self.data.into_boxed_slice()
    }
    pub fn into_boxed_index_slice(self) -> Box<IndexSlice<I, T>> {
        IndexSlice::from_boxed_slice(self.data.into_boxed_slice())
    }

    pub fn get_index(&self, index: I) -> Option<&T> {
        self.data.get_index(index.into_usize())
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<T>>
    where
        R: IndexRangeBounds<I>,
    {
        self.data.get_range(range.canonicalize(self.len()))
    }

    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn swap_remove_index(&mut self, index: I) -> Option<T> {
        self.data.swap_remove_index(index.into_usize())
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<T> {
        self.data.swap_remove_index(index.into_usize())
    }

    pub fn swap_indices(&mut self, from: I, to: I) {
        self.data.swap_indices(from.into_usize(), to.into_usize());
    }

    /// NOTE: to call `is_disjoint` on an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().is_disjoint(index_set)`
    pub fn is_disjoint<I2, S2>(&self, other: &IndexHashSet<I2, T, S2>) -> bool
    where
        T: Eq + Hash,
        I2: Idx,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.is_disjoint(&other.data)
    }

    /// NOTE: to call `is_subset` on an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().is_subset(index_set)`
    pub fn is_subset<I2, S2>(&self, other: &IndexHashSet<I2, T, S2>) -> bool
    where
        T: Eq + Hash,
        I2: Idx,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.is_subset(&other.data)
    }

    /// NOTE: to `is_superset` to an `IndexHashSet` with an `IndexSet`
    /// you can use `index_hash_set.as_index_set_mut().is_superset(index_set)`
    pub fn is_superset<I2, S2>(&self, other: &IndexHashSet<I2, T, S2>) -> bool
    where
        T: Eq + Hash,
        I2: Idx,
        S: BuildHasher,
        S2: BuildHasher,
    {
        self.data.is_superset(&other.data)
    }

    pub fn as_index_set(&self) -> &IndexSet<T, S> {
        &self.data
    }
    pub fn as_index_set_mut(&mut self) -> &mut IndexSet<T, S> {
        &mut self.data
    }

    pub fn indices(&self) -> IndexRange<I> {
        IndexRange::new(I::ZERO..self.len_idx())
    }
}

impl<I, T, S> Deref for IndexHashSet<I, T, S> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_slice(self.data.as_slice())
    }
}

impl<T, I1, I2, S1, S2> BitAnd<&IndexHashSet<I2, T, S2>>
    for &IndexHashSet<I1, T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexHashSet<I1, T, S1>;

    fn bitand(self, rhs: &IndexHashSet<I2, T, S2>) -> Self::Output {
        IndexHashSet::from(self.data.bitand(&rhs.data))
    }
}

impl<T, I1, I2, S1, S2> BitOr<&IndexHashSet<I2, T, S2>>
    for &IndexHashSet<I1, T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexHashSet<I1, T, S1>;

    fn bitor(self, rhs: &IndexHashSet<I2, T, S2>) -> Self::Output {
        IndexHashSet::from(self.data.bitor(&rhs.data))
    }
}

impl<T, I1, I2, S1, S2> BitXor<&IndexHashSet<I2, T, S2>>
    for &IndexHashSet<I1, T, S1>
where
    T: Eq + Hash + Clone,
    S1: BuildHasher + Default,
    S2: BuildHasher,
{
    type Output = IndexHashSet<I1, T, S1>;

    fn bitxor(self, rhs: &IndexHashSet<I2, T, S2>) -> Self::Output {
        IndexHashSet::from(self.data.bitxor(&rhs.data))
    }
}

impl<I, T, S> Clone for IndexHashSet<I, T, S>
where
    T: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T: Debug, S> Debug for IndexHashSet<I, T, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T, S> Default for IndexHashSet<I, T, S>
where
    S: Default,
{
    fn default() -> Self {
        Self::from(IndexSet::default())
    }
}

impl<'a, I, T, S> Extend<&'a T> for IndexHashSet<I, T, S>
where
    T: Hash + Eq + Copy,
    S: BuildHasher,
{
    fn extend<II: IntoIterator<Item = &'a T>>(&mut self, iter: II) {
        self.data.extend(iter);
    }
}

impl<I, T, S> Extend<T> for IndexHashSet<I, T, S>
where
    T: Hash + Eq + Copy,
    S: BuildHasher,
{
    fn extend<II: IntoIterator<Item = T>>(&mut self, iter: II) {
        self.data.extend(iter);
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

impl<I, T, S> FromIterator<T> for IndexHashSet<I, T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    fn from_iter<II: IntoIterator<Item = T>>(iter: II) -> Self {
        IndexHashSet::from(IndexSet::from_iter(iter))
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

impl<I: Idx, T, S: BuildHasher> Index<I> for IndexHashSet<I, T, S> {
    type Output = T;
    fn index(&self, idx: I) -> &T {
        self.data.index(idx.into_usize())
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, T> Serialize for IndexSlice<I, T>
where
    Slice<T>: Serialize,
{
    fn serialize<SR: Serializer>(
        &self,
        serializer: SR,
    ) -> Result<SR::Ok, SR::Error> {
        self.data.serialize(serializer)
    }
}

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
