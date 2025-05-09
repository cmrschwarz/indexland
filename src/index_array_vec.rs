use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
};

use arrayvec::{ArrayVec, CapacityError};

use crate::{index_enumerate::IndexEnumerate, IdxCompat, IndexArray, IndexRange, IndexRangeBounds};

use super::{idx::Idx, index_slice::IndexSlice};

/// Create an [`IndexArrayVec`] containing the arguments.
///
/// The syntax is identical to [`index_array!`](crate::index_array!).
#[macro_export]
macro_rules! index_array_vec {
    () => {
        $crate::IndexArrayVec::from_array([])
    };
    ($value:expr; $count: expr) => {
        $crate::IndexArrayVec::from_array([ $value; $count])
    };
    ($($value:expr),+ $(,)?) => {
        $crate::IndexArrayVec::from_array([$($value),*])
    };
    ($($index:expr => $value:expr),* $(,)?) => {{
        let indices = [ $($index as usize),* ];
        let mut values = [ $($value),* ];
        let data = $crate::__private::array_from_values_and_distinct_indices(
            indices,
            ::core::mem::ManuallyDrop::new(values)
        );
        $crate::IndexArrayVec::from_array(data)
    }};
}

#[cfg(target_pointer_width = "16")]
type IndexArrayVecLen = u16;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
type IndexArrayVecLen = u32;

#[repr(C)]
pub struct IndexArrayVec<I, T, const CAP: usize> {
    // NOTE: We would much prefer to reuse `ArrayVec` directly,
    // But their `const` support is severely lacking due to their MSRV.
    // This would make the `index_array_vec` macro borderline unuseable.
    // Once something this lands: https://github.com/bluss/arrayvec/pull/294
    // this implementation can hopefully be replaced with a simple wrapper again.
    // No need to maintain the same code in two places.
    len: IndexArrayVecLen,
    data: [MaybeUninit<T>; CAP],
    _phantom: PhantomData<fn(I) -> T>,
}

pub struct IntoIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    // NOTE: for this to implement double ended iterator we need start and end.
    // Rust std does the same tradeoff:
    // https://doc.rust-lang.org/1.83.0/src/core/array/iter.rs.html#14
    alive: core::ops::Range<usize>,
}

impl<I, T, const CAP: usize> Clone for IndexArrayVec<I, T, CAP>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut res = Self {
            len: self.len,
            data: [const { MaybeUninit::uninit() }; CAP],
            _phantom: PhantomData,
        };
        res.extend_from_slice(&self.as_slice()[0..self.len as usize]);
        res
    }
}
impl<I, T, const CAP: usize> Copy for IndexArrayVec<I, T, CAP> where T: Copy {}

impl<I, T, const CAP: usize> Deref for IndexArrayVec<I, T, CAP> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        IndexSlice::from_raw_slice(self.as_slice())
    }
}
impl<I, T, const CAP: usize> DerefMut for IndexArrayVec<I, T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IndexSlice::from_mut_raw_slice(self.as_mut_slice())
    }
}

impl<I, T, const CAP: usize> From<ArrayVec<T, CAP>> for IndexArrayVec<I, T, CAP> {
    fn from(v: ArrayVec<T, CAP>) -> Self {
        let mut res = IndexArrayVec::new();
        let v = ManuallyDrop::new(v);
        unsafe {
            core::ptr::copy_nonoverlapping(v.as_ptr(), res.as_mut_ptr(), v.len());
            res.set_len(v.len());
        }
        res
    }
}

impl<I, T, const CAP: usize> From<[T; CAP]> for IndexArrayVec<I, T, CAP> {
    fn from(value: [T; CAP]) -> Self {
        IndexArrayVec::from(ArrayVec::from(value))
    }
}
impl<I, T, const CAP: usize> From<IndexArray<I, T, CAP>> for IndexArrayVec<I, T, CAP> {
    fn from(value: IndexArray<I, T, CAP>) -> Self {
        IndexArrayVec::from(<[T; CAP]>::from(value))
    }
}

impl<I, T, const CAP: usize> From<IndexArrayVec<I, T, CAP>> for ArrayVec<T, CAP> {
    fn from(value: IndexArrayVec<I, T, CAP>) -> Self {
        value.into_array_vec()
    }
}

impl<I, T, const CAP: usize> Default for IndexArrayVec<I, T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I, T: Debug, const CAP: usize> Debug for IndexArrayVec<I, T, CAP> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T, const CAP: usize> IndexArrayVec<I, T, CAP> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            data: [const { MaybeUninit::uninit() }; CAP],
            _phantom: PhantomData,
        }
    }
    pub fn from_array_vec(av: ArrayVec<T, CAP>) -> Self {
        let av = ManuallyDrop::new(av);
        let mut res = IndexArrayVec::new();
        unsafe {
            core::ptr::copy_nonoverlapping(av.as_ptr(), res.as_mut_ptr(), av.len());
            res.set_len(av.len());
        }
        res
    }

    /// Note: we would like to offer `as_array_vec` in the future.
    /// See this module's [docs](index_array_vec) as to why.
    pub fn into_array_vec(self) -> ArrayVec<T, CAP> {
        let this = ManuallyDrop::new(self);
        let mut av = ArrayVec::new();
        unsafe {
            core::ptr::copy_nonoverlapping(this.as_ptr(), av.as_mut_ptr(), this.len());
            av.set_len(this.len());
        }
        av
    }

    pub const fn into_array(self) -> [MaybeUninit<T>; CAP] {
        self.data
    }

    /// unlike `From<[T; N]>::from`, this is a `const fn`
    pub const fn from_array<const N: usize>(arr: [T; N]) -> Self {
        pub struct AssertArrayBounds<const N: usize, const CAP: usize>;
        impl<const N: usize, const CAP: usize> AssertArrayBounds<N, CAP> {
            pub const OK: () = assert!(N <= CAP);
        }
        let _: () = AssertArrayBounds::<N, CAP>::OK;

        let mut res = [const { MaybeUninit::uninit() }; CAP];
        let src = (&raw const arr).cast::<T>();

        let tgt = res.as_mut_ptr().cast::<T>();
        unsafe {
            core::ptr::copy_nonoverlapping(src, tgt, N);
        }
        core::mem::forget(arr);
        #[allow(clippy::cast_possible_truncation)]
        IndexArrayVec {
            len: N as IndexArrayVecLen,
            data: res,
            _phantom: PhantomData,
        }
    }

    /// unlike `From<IndexArray<I, T, N>::from`, this is a `const fn`
    pub const fn from_index_array<const N: usize>(arr: IndexArray<I, T, N>) -> Self {
        Self::from_array(arr.into_inner())
    }

    pub fn try_extend_from_slice(&mut self, slice: &[T]) -> Result<(), arrayvec::CapacityError>
    where
        T: Clone,
    {
        let len = self.len as usize;
        let slice_len = slice.len();
        if len + slice_len < CAP {
            return Err(CapacityError::new(()));
        }

        let ptr = unsafe { self.as_mut_ptr().add(len) };

        #[allow(clippy::needless_range_loop)]
        for i in 0..slice_len {
            unsafe {
                *ptr.add(i) = slice[i].clone();
            };
        }
        unsafe { self.set_len(len + slice_len) };
        Ok(())
    }

    /// # Safety
    /// `self.len()` must be less than CAP before calling
    pub const unsafe fn push_unchecked(&mut self, v: T) {
        let len = self.len();
        debug_assert!(len < CAP);
        unsafe {
            core::ptr::write(self.as_mut_ptr().add(len), v);
            self.set_len(len + 1);
        }
    }
    pub fn push(&mut self, v: T) {
        self.try_push(v).unwrap();
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(self.data[0].assume_init_read()) }
    }
    pub fn swap_remove(&mut self, idx: I) -> T
    where
        I: Idx,
    {
        let idx = idx.into_usize();
        assert!(idx < self.len as usize);
        let last = self.len as usize - 1;
        unsafe {
            let res = self.data[idx].assume_init_read();
            core::ptr::copy(&raw const self.data[last], &raw mut self.data[idx], 1);
            self.len -= 1;
            res
        }
    }
    pub fn clear(&mut self) {
        unsafe {
            self.len = 0;
            core::ptr::drop_in_place(core::ptr::from_mut::<[T]>(self.as_mut_slice()));
        }
    }
    pub fn truncate_len(&mut self, len: usize) {
        // > instead of >= is intentional, this improves codegen. See
        // https://doc.rust-lang.org/1.85.0/src/alloc/vec/mod.rs.html#1533

        if len > self.len as usize {
            return;
        }
        unsafe {
            let rem = core::ptr::slice_from_raw_parts_mut(
                self.as_mut_ptr().add(len),
                self.len as usize - len,
            );
            // SAFETY: len is guaranteed to be smaller than CAP
            #[allow(clippy::cast_possible_truncation)]
            {
                self.len = len as IndexArrayVecLen;
            }
            core::ptr::drop_in_place(rem);
        }
    }
    pub fn push_get_idx(&mut self, v: T) -> I
    where
        I: Idx,
    {
        let id = self.len_idx();
        self.push(v);
        id
    }
    pub fn truncate(&mut self, new_end_index: I)
    where
        I: Idx,
    {
        self.truncate_len(new_end_index.into_usize());
    }
    pub fn iter_enumerated(&self) -> IndexEnumerate<I, core::slice::Iter<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.as_slice())
    }
    pub fn iter_enumerated_mut(&mut self) -> IndexEnumerate<I, core::slice::IterMut<T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.as_mut_slice())
    }
    pub fn iter_enumerated_range<C, R: IndexRangeBounds<C>>(
        &self,
        range: R,
    ) -> IndexEnumerate<I, core::slice::Iter<T>>
    where
        I: Idx,
        C: IdxCompat<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &self.as_slice()[range])
    }
    pub fn iter_enumerated_range_mut<C, R: IndexRangeBounds<C>>(
        &mut self,
        range: R,
    ) -> IndexEnumerate<I, core::slice::IterMut<T>>
    where
        I: Idx,
        C: IdxCompat<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.as_mut_slice()[range])
    }
    pub fn into_iter_enumerated(self) -> IndexEnumerate<I, IntoIter<T, CAP>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self)
    }
    pub fn indices(&self) -> IndexRange<I>
    where
        I: Idx,
    {
        IndexRange::new(I::ZERO..self.len_idx())
    }
    pub fn capacity(&self) -> usize {
        CAP
    }
    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_raw_slice(self.as_slice())
    }
    pub fn as_mut_index_slice(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_mut_raw_slice(self.as_mut_slice())
    }

    fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.try_extend_from_slice(slice).unwrap();
    }

    pub const fn len(&self) -> usize {
        self.len as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The deref coersion to `IndexSlice` is not `const`, so these are convenient.
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr().cast::<T>()
    }

    /// The deref coersion to `IndexSlice` is not `const`, so these are convenient.
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr().cast::<T>()
    }

    /// # Safety
    /// - `len` must be less than or equal to `CAP`
    /// - The elements at `self.len()..new_len` must be initialized.
    pub const unsafe fn set_len(&mut self, new_len: usize) {
        #![allow(clippy::cast_possible_truncation)]
        self.len = new_len as IndexArrayVecLen;
    }

    pub const fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    pub fn try_push(&mut self, element: T) -> Result<(), CapacityError<T>> {
        if self.len as usize == CAP {
            return Err(CapacityError::new(element));
        }
        unsafe { self.push_unchecked(element) };
        Ok(())
    }
}

impl<I, T, const N: usize> AsRef<[T]> for IndexArrayVec<I, T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<I, T, const N: usize> AsRef<IndexSlice<I, T>> for IndexArrayVec<I, T, N> {
    fn as_ref(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}

impl<I, T, const N: usize> AsMut<[T]> for IndexArrayVec<I, T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
impl<I, T, const N: usize> AsMut<IndexSlice<I, T>> for IndexArrayVec<I, T, N> {
    fn as_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}

impl<I, T, const N: usize> Borrow<[T]> for IndexArrayVec<I, T, N> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}
impl<I, T, const N: usize> Borrow<IndexSlice<I, T>> for IndexArrayVec<I, T, N> {
    fn borrow(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}

impl<I, T, const N: usize> BorrowMut<[T]> for IndexArrayVec<I, T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
impl<I, T, const N: usize> BorrowMut<IndexSlice<I, T>> for IndexArrayVec<I, T, N> {
    fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}

impl<I, T, const CAP: usize> Extend<T> for IndexArrayVec<I, T, CAP> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        let mut iter = iter.into_iter();
        let ptr = self.as_mut_ptr();
        let mut len = self.len as usize;
        while len < CAP {
            let Some(next) = iter.next() else {
                unsafe { self.set_len(len) };
                return;
            };
            unsafe {
                core::ptr::write(ptr.add(len), next);
            };
            len += 1;
        }
        unsafe { self.set_len(len) };
    }
}

impl<I, T, const CAP: usize> IntoIterator for IndexArrayVec<I, T, CAP> {
    type Item = T;

    type IntoIter = IntoIter<T, CAP>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            alive: 0..self.len(),
            data: self.data,
        }
    }
}

impl<'a, I, T, const CAP: usize> IntoIterator for &'a IndexArrayVec<I, T, CAP> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I, T, const CAP: usize> IntoIterator for &'a mut IndexArrayVec<I, T, CAP> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I, T, const CAP: usize> FromIterator<T> for IndexArrayVec<I, T, CAP> {
    fn from_iter<ITER: IntoIterator<Item = T>>(iter: ITER) -> Self {
        Self::from(ArrayVec::from_iter(iter))
    }
}

impl<I, T: PartialEq, const CAP: usize, const N: usize> PartialEq<IndexArrayVec<I, T, CAP>>
    for [T; N]
{
    fn eq(&self, other: &IndexArrayVec<I, T, CAP>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq, const CAP: usize, const N: usize> PartialEq<[T; N]>
    for IndexArrayVec<I, T, CAP>
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq, const CAP: usize> PartialEq<IndexSlice<I, T>> for IndexArrayVec<I, T, CAP> {
    fn eq(&self, other: &IndexSlice<I, T>) -> bool {
        self.as_slice() == other.as_raw_slice()
    }
}

impl<I, T: PartialEq, const CAP: usize> PartialEq<IndexArrayVec<I, T, CAP>> for IndexSlice<I, T> {
    fn eq(&self, other: &IndexArrayVec<I, T, CAP>) -> bool {
        self.as_raw_slice() == other.as_slice()
    }
}

impl<I, T: PartialEq, const CAP: usize> PartialEq<IndexArrayVec<I, T, CAP>> for [T] {
    fn eq(&self, other: &IndexArrayVec<I, T, CAP>) -> bool {
        self == other.as_slice()
    }
}

impl<I, T: PartialEq, const CAP: usize> PartialEq<[T]> for IndexArrayVec<I, T, CAP> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.alive
            .next()
            .map(|i| unsafe { self.data[i].assume_init_read() })
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.alive
            .next_back()
            .map(|i| unsafe { self.data[i].assume_init_read() })
    }
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        unsafe { core::ptr::drop_in_place(self.as_mut_slice()) }
    }
}

impl<T, const N: usize> IntoIter<T, N> {
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }
    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        core::ptr::slice_from_raw_parts_mut(
            unsafe { self.data.as_mut_ptr().cast::<T>().add(self.alive.start) },
            self.alive.end - self.alive.start,
        )
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I, T, const CAP: usize> Serialize for IndexArrayVec<I, T, CAP>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T, const CAP: usize> Deserialize<'de> for IndexArrayVec<I, T, CAP>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};

        #[allow(clippy::type_complexity)]
        struct IndexArrayVecVisitor<'de, I, T, const CAP: usize>(
            PhantomData<(&'de (), fn(I) -> T, [T; CAP])>,
        );

        impl<'de, I, T, const CAP: usize> Visitor<'de> for IndexArrayVecVisitor<'de, I, T, CAP>
        where
            T: Deserialize<'de>,
        {
            type Value = IndexArrayVec<I, T, CAP>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "an array with no more than {CAP} items")
            }

            fn visit_seq<SA>(self, mut seq: SA) -> Result<Self::Value, SA::Error>
            where
                SA: SeqAccess<'de>,
            {
                let mut values = IndexArrayVec::<I, T, CAP>::new();

                while let Some(value) = seq.next_element()? {
                    if values.try_push(value).is_err() {
                        return Err(SA::Error::invalid_length(CAP + 1, &self));
                    }
                }

                Ok(values)
            }
        }

        deserializer.deserialize_seq(IndexArrayVecVisitor::<I, T, CAP>(PhantomData))
    }
}
