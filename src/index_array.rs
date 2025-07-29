use super::Idx;
use crate::{IdxEnum, IndexRangeBounds, index_enumerate::IndexEnumerate, index_slice::IndexSlice};

use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[repr(transparent)]
pub struct IndexArray<I, T, const N: usize> {
    data: [T; N],
    _phantom: PhantomData<fn(I) -> T>,
}

/// Helper to construct `IndexArray<E, T, { <E as IdxEnum>::VARIANT_COUNT } >`.
///
/// Use [`IndexArray`] instead for Arrays that don't have exactly `VARIANT_COUNT` elements.
///
/// # Example:
/// ```
/// # #![cfg(feature="derive")]
/// # use indexland::{Idx, index_array::{IndexArray, EnumIndexArray}};
/// #[derive(Idx)]
/// enum Foo {
///     A,
///     B,
///     C,
/// }
/// const FOOS: EnumIndexArray<Foo, i32> = IndexArray::new([1, 2, 3]);
/// ```
pub type EnumIndexArray<E, T> = <E as IdxEnum>::EnumIndexArray<T>;

/// Create a [`IndexArray`] containing the arguments.
///
/// If the inputs are constant this creates a compile time constant array.
/// # Examples:
/// ```
/// # #![cfg(feature="derive")]
/// use indexland::{Idx, IdxEnum, IndexArray, index_array};
///
/// const FOO: IndexArray<u8, i32, 3> = index_array![1, 2, 3];
///
/// const BAR: IndexArray<u8, f32, 42> = index_array![0.0; 42];
///
/// #[derive(Idx)]
/// enum MyId {
///     A,
///     B,
///     C,
/// }
///
/// const BAZ: IndexArray<MyId, i32, { MyId::VARIANT_COUNT }> = index_array![
///     MyId::A => 1,
///     MyId::B => 2,
///     MyId::C => 3,
/// ];
/// ```
#[macro_export]
macro_rules! index_array {
    () => {
        $crate::IndexArray::new([])
    };
    ($value:expr; $count: expr) => {
        $crate::IndexArray::new([ $value; $count])
    };
    ($($value:expr),+ $(,)?) => {
        $crate::IndexArray::new([$($value),*])
    };
    // TODO: figure out better syntax that would also work for IndexArrayVec
    //($idx: ty => $($value:expr),+ $(,)?) => {
    //    const LEN: usize = <[()]>::len(&[$({ stringify!($key); }),*]);
    //    $crate::IndexArray::<$idx, _, LEN>::new([$($value),*])
    //};
    ($($index:expr => $value:expr),* $(,)?) => {{
        let keys = [ $($index as usize),* ];
        let values = [ $($value),* ];
        let data = $crate::__private::array_from_values_and_distinct_indices(
            keys,
            core::mem::ManuallyDrop::new(values)
        );
        $crate::IndexArray::new(data)
    }};
}

/// Create a [`EnumIndexArray`] containing the arguments.
///
/// If the inputs are constant this creates a compile time constant array.
/// This is an alias for [`index_array!`]
/// # Examples:
/// ```
/// use indexland::{EnumIndexArray, Idx, enum_index_array};
///
/// #[derive(Idx)]
/// enum MyId {
///     A,
///     B,
///     C,
/// }
///
/// const BAZ: EnumIndexArray<MyId, i32> = enum_index_array![
///     MyId::A => 1,
///     MyId::B => 2,
///     MyId::C => 3,
/// ];
/// ```
#[macro_export]
macro_rules! enum_index_array {
    ($($anything: tt)*) => {
        $crate::index_array![$($anything)*]
    };
}

impl<I, T, const N: usize> IndexArray<I, MaybeUninit<T>, N> {
    pub const fn transpose(self) -> MaybeUninit<IndexArray<I, T, N>> {
        unsafe {
            // SAFETY: T and MaybeUninit<T> have the same layout.
            // We don't have to `forget()` the original value since `MaybeUninit<T>` does not drop `T`.
            // See https://github.com/rust-lang/rust/issues/62875#issuecomment-513834029 for
            // why we need the `_copy` version
            core::mem::transmute_copy(&self)
        }
    }
}

impl<I, T, const N: usize> IndexArray<I, T, N> {
    #[inline(always)]
    pub const fn new(data: [T; N]) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    pub fn map<F, U>(self, f: F) -> IndexArray<I, U, N>
    where
        F: FnMut(T) -> U,
    {
        IndexArray::new(self.data.map(f))
    }

    pub fn as_array(&self) -> &[T; N] {
        &self.data
    }
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.data
    }
    pub const fn as_slice(&self) -> &[T] {
        &self.data
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub const fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(self.data.as_slice())
    }
    // TODO: const once https://github.com/rust-lang/rust/issues/133333 lands
    pub fn as_mut_index_slice(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_mut_slice(self.data.as_mut_slice())
    }
    pub fn each_ref(&self) -> IndexArray<I, &T, N> {
        self.data.each_ref().into()
    }
    pub fn each_mut(&mut self) -> IndexArray<I, &mut T, N> {
        self.data.each_mut().into()
    }

    pub fn iter_enumerated(&self) -> IndexEnumerate<I, core::slice::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(&mut self) -> IndexEnumerate<I, core::slice::IterMut<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn iter_enumerated_range<R: IndexRangeBounds<I>>(
        &self,
        range: R,
    ) -> IndexEnumerate<I, core::slice::Iter<'_, T>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, &self.data[range.canonicalize(self.len())])
    }
    pub fn iter_enumerated_range_mut<R>(
        &mut self,
        range: R,
    ) -> IndexEnumerate<I, core::slice::IterMut<'_, T>>
    where
        I: Idx,
        R: IndexRangeBounds<I>,
    {
        let range = range.canonicalize(self.len());
        IndexEnumerate::new(I::ZERO, &mut self.data[range])
    }
    pub fn into_iter_enumerated(self) -> IndexEnumerate<I, core::array::IntoIter<T, N>>
    where
        I: Idx,
    {
        IndexEnumerate::new(I::ZERO, self.data)
    }
    pub const fn into_array(self) -> [T; N] {
        let res = unsafe { core::ptr::read(&raw const self.data) };
        core::mem::forget(self);
        res
    }
    pub const fn from_array_ref(arr: &[T; N]) -> &IndexArray<I, T, N> {
        unsafe { &*arr.as_ptr().cast() }
    }
    pub const fn from_mut_array_ref(arr: &mut [T; N]) -> &mut IndexArray<I, T, N> {
        unsafe { &mut *arr.as_mut_ptr().cast() }
    }
}

impl<I, T, const N: usize> AsRef<[T]> for IndexArray<I, T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<I, T, const N: usize> AsRef<[T; N]> for IndexArray<I, T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.data
    }
}
impl<I, T, const N: usize> AsRef<IndexSlice<I, T>> for IndexArray<I, T, N> {
    fn as_ref(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}

impl<I, T, const N: usize> AsMut<[T]> for IndexArray<I, T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
impl<I, T, const N: usize> AsMut<IndexSlice<I, T>> for IndexArray<I, T, N> {
    fn as_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}
impl<I, T, const N: usize> AsMut<[T; N]> for IndexArray<I, T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.data
    }
}

impl<I, T, const N: usize> Borrow<[T]> for IndexArray<I, T, N> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}
impl<I, T, const N: usize> Borrow<IndexSlice<I, T>> for IndexArray<I, T, N> {
    fn borrow(&self) -> &IndexSlice<I, T> {
        self.as_index_slice()
    }
}

impl<I, T, const N: usize> BorrowMut<[T]> for IndexArray<I, T, N> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
impl<I, T, const N: usize> BorrowMut<IndexSlice<I, T>> for IndexArray<I, T, N> {
    fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
        self.as_mut_index_slice()
    }
}

impl<I, T, const N: usize> Clone for IndexArray<I, T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _phantom: PhantomData,
        }
    }
}
impl<I, T, const N: usize> Copy for IndexArray<I, T, N> where T: Copy {}

impl<I, T, const N: usize> Deref for IndexArray<I, T, N> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        self.as_index_slice()
    }
}

impl<I, T, const N: usize> DerefMut for IndexArray<I, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_index_slice()
    }
}

impl<I, T: Debug, const N: usize> Debug for IndexArray<I, T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I, T, const N: usize> Default for IndexArray<I, T, N>
where
    [T; N]: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T, const LEN: usize> From<[T; LEN]> for IndexArray<I, T, LEN> {
    fn from(value: [T; LEN]) -> Self {
        Self::new(value)
    }
}

impl<I, T, const LEN: usize> From<IndexArray<I, T, LEN>> for [T; LEN] {
    fn from(value: IndexArray<I, T, LEN>) -> Self {
        value.data
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, I, T, const N: usize> From<&'a IndexArray<I, T, N>> for Cow<'a, IndexSlice<I, T>>
where
    T: Clone,
{
    fn from(value: &'a IndexArray<I, T, N>) -> Self {
        Cow::Borrowed(value.as_index_slice())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<I, T, const N: usize> From<&IndexArray<I, T, N>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &IndexArray<I, T, N>) -> Self {
        Vec::from(&value.data)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<I, T, const N: usize> From<&mut IndexArray<I, T, N>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &mut IndexArray<I, T, N>) -> Self {
        Vec::from(&value.data)
    }
}

impl<I, T, const N: usize> Hash for IndexArray<I, T, N>
where
    [T; N]: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<I, T, const N: usize> IntoIterator for IndexArray<I, T, N> {
    type Item = T;

    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I, T, const N: usize> IntoIterator for &'a IndexArray<I, T, N> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I, T, const N: usize> IntoIterator for &'a mut IndexArray<I, T, N> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<IndexArray<I, T, N>> for IndexArray<I, T, N> {
    fn eq(&self, other: &IndexArray<I, T, N>) -> bool {
        self.data == other.data
    }
}
impl<I, T: Eq, const N: usize> Eq for IndexArray<I, T, N> {}

impl<I, T: PartialEq, const N: usize> PartialEq<IndexArray<I, T, N>> for [T; N] {
    fn eq(&self, other: &IndexArray<I, T, N>) -> bool {
        self == &other.data
    }
}
impl<I, T: PartialEq, const N: usize> PartialEq<[T; N]> for IndexArray<I, T, N> {
    fn eq(&self, other: &[T; N]) -> bool {
        &self.data == other
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<IndexArray<I, T, N>> for [T] {
    fn eq(&self, other: &IndexArray<I, T, N>) -> bool {
        self == other.data
    }
}

impl<I, T: PartialEq, const N: usize> PartialEq<[T]> for IndexArray<I, T, N> {
    fn eq(&self, other: &[T]) -> bool {
        self.data == other
    }
}

impl<'a, I, T, const N: usize> TryFrom<&'a [T]> for &'a IndexArray<I, T, N> {
    type Error = core::array::TryFromSliceError;

    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        Ok(IndexArray::from_array_ref(<&'a [T; N]>::try_from(value)?))
    }
}
impl<'a, I, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut IndexArray<I, T, N> {
    type Error = core::array::TryFromSliceError;

    fn try_from(value: &'a mut [T]) -> Result<Self, Self::Error> {
        Ok(IndexArray::from_mut_array_ref(<&'a mut [T; N]>::try_from(
            value,
        )?))
    }
}
impl<'a, I, T, const N: usize> TryFrom<&'a IndexSlice<I, T>> for &'a IndexArray<I, T, N> {
    type Error = core::array::TryFromSliceError;

    fn try_from(value: &'a IndexSlice<I, T>) -> Result<Self, Self::Error> {
        Ok(IndexArray::from_array_ref(<&'a [T; N]>::try_from(
            value.as_slice(),
        )?))
    }
}
impl<'a, I, T, const N: usize> TryFrom<&'a mut IndexSlice<I, T>> for &'a mut IndexArray<I, T, N> {
    type Error = core::array::TryFromSliceError;

    fn try_from(value: &'a mut IndexSlice<I, T>) -> Result<Self, Self::Error> {
        Ok(IndexArray::from_mut_array_ref(<&'a mut [T; N]>::try_from(
            value.as_mut_slice(),
        )?))
    }
}

#[cfg(feature = "serde")]
impl<I, T, const N: usize> Serialize for IndexArray<I, T, N>
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
struct IndexArrayVisitor<I, T, const N: usize>(PhantomData<(I, T)>);

// We unfortunately can't reuse the serde array implementation because it only supports arrays up
// to length 32. This is loosely based on https://docs.rs/serde_arrays/latest/src/serde_arrays/lib.rs.html#179
#[cfg(feature = "serde")]
impl<'de, I, T, const N: usize> serde::de::Visitor<'de> for IndexArrayVisitor<I, T, N>
where
    T: Deserialize<'de>,
{
    type Value = IndexArray<I, T, N>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "an array of size {N}")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut filled = 0;
        let mut err = None;

        let mut arr = [const { MaybeUninit::<T>::uninit() }; N];

        for slot in &mut arr {
            match seq.next_element() {
                Err(e) => {
                    err = Some(e);
                    break;
                }
                Ok(None) => {
                    err = Some(serde::de::Error::invalid_length(filled, &self));
                    break;
                }
                Ok(Some(e)) => {
                    *slot = MaybeUninit::new(e);
                    filled += 1;
                }
            }
        }

        if err.is_none() {
            match seq.next_element::<T>() {
                Ok(None) => (),
                Ok(Some(_)) => {
                    err = Some(serde::de::Error::invalid_length(filled + 1, &self));
                }
                Err(e) => err = Some(e),
            }
        }

        if let Some(err) = err {
            if core::mem::needs_drop::<T>() {
                for e in arr.into_iter().take(filled) {
                    // Safety: the loop above ensures the first `filled` elements are initialized
                    unsafe {
                        let _ = e.assume_init();
                    }
                }
            }
            return Err(err);
        }

        // SAFETY: we ensured that filled == N
        // We don't need to drop arr because MaybeUninit doesn't drop.
        // See https://github.com/rust-lang/rust/issues/62875#issuecomment-513834029
        // for why we need the `_copy` version
        let array_init = unsafe { core::mem::transmute_copy(&arr) };

        Ok(IndexArray::new(array_init))
    }
}

#[cfg(feature = "serde")]
impl<'de, I, T, const N: usize> Deserialize<'de> for IndexArray<I, T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(N, IndexArrayVisitor(PhantomData))
    }
}

#[cfg(feature = "serde")]
pub mod serde_map {
    //! Functions to serialize and deserialize an [`IndexArray`] as a map.
    //!
    //! The default `serde` implementation serializes `IndexArray` as a sequence.
    //! This module uses a map of indices to values instead. It is mainly indended for
    //! making human readable formats like json more readable, for example if the index is an enum.
    //!
    //! Use [`serde(with = "indexland::index_array::serde_map")`](https://serde.rs/field-attrs.html#serialize_with)
    //! to apply this to a field.
    //!
    //! # Example
    //!
    //! ```
    //! # use indexland::IndexArray;
    //! # use serde::{Deserialize, Serialize};
    //!
    //! #[derive(Deserialize, Serialize)]
    //! struct Data {
    //!     #[serde(with = "indexland::index_array::serde_map")]
    //!     map: IndexArray<usize, String, 42>,
    //! }
    //! ```

    use core::{marker::PhantomData, mem::MaybeUninit};

    use crate::{Idx, IndexArray};
    use serde::{
        de::{Deserialize, Deserializer, Visitor},
        ser::{Serialize, Serializer},
    };

    /// Serializes an [`IndexArray`] as an ordered sequence.
    ///
    /// This function may be used in a field attribute for deriving [`Serialize`]:
    ///
    /// ```
    /// # use indexland::IndexArray;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct Data {
    ///     #[serde(serialize_with = "indexland::index_array::serde_map::serialize")]
    ///     map: IndexArray<i32, u64, 42>,
    ///     // ...
    /// }
    /// ```
    pub fn serialize<S, I, T, const N: usize>(
        array: &IndexArray<I, T, N>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        I: Idx + Serialize,
        T: Serialize,
        S: Serializer,
    {
        serializer.collect_map(array.iter_enumerated())
    }

    struct MapVisitor<I, T, const N: usize>(PhantomData<IndexArray<I, T, N>>);

    impl<'de, I: Idx + Deserialize<'de>, T: Deserialize<'de>, const N: usize> Visitor<'de>
        for MapVisitor<I, T, N>
    {
        type Value = IndexArray<I, T, N>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a map with {N} entries")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            // PERF: use a bitfield for this?
            let mut initialized = [false; N];
            let mut arr = [const { MaybeUninit::uninit() }; N];

            let mut len = 0;

            let mut err = None;

            loop {
                match map.next_entry::<I, T>() {
                    Ok(None) => break,
                    Err(e) => {
                        err = Some(e);
                        break;
                    }
                    Ok(Some((k, v))) => {
                        let index = k.into_usize();
                        if initialized[index] {
                            let _ = unsafe { std::ptr::read(&raw const arr[index]).assume_init() };
                        } else {
                            if len == N {
                                err = Some(serde::de::Error::invalid_length(len + 1, &self));
                                break;
                            }
                            len += 1;
                            initialized[index] = true;
                        }
                        arr[len] = MaybeUninit::new(v);
                    }
                }
            }

            if err.is_none() && len != N {
                err = Some(serde::de::Error::invalid_length(len, &self));
            }

            if let Some(err) = err {
                if core::mem::needs_drop::<T>() {
                    for (i, &initialized) in initialized.iter().enumerate() {
                        if initialized {
                            let _ = unsafe { std::ptr::read(&raw const arr[i]).assume_init() };
                        }
                    }
                }
                return Err(err);
            }

            Ok(IndexArray::new(unsafe {
                //SAFETY: we ensure above that all `N` slots are initialized
                core::mem::transmute_copy(&arr)
            }))
        }
    }

    /// Deserializes an [`IndexArray`] from a map from index to value.
    ///
    /// This function may be used in a field attribute for deriving [`Deserialize`]:
    ///
    /// ```
    /// # use indexland::IndexArray;
    /// # use serde::Deserialize;
    /// #[derive(Deserialize)]
    /// struct Data {
    ///     #[serde(serialize_with = "indexland::index_array::serde_map::serialize")]
    ///     map: IndexArray<i32, u64, 42>,
    ///     // ...
    /// }
    /// ```
    pub fn deserialize<'de, D, I, T, const N: usize>(
        deserializer: D,
    ) -> Result<IndexArray<I, T, N>, D::Error>
    where
        D: Deserializer<'de>,
        I: Idx + Deserialize<'de>,
        T: Deserialize<'de>,
    {
        deserializer.deserialize_seq(MapVisitor(PhantomData))
    }
}
