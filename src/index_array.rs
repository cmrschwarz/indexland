use super::Idx;
use crate::{
    idx_enumerate::IdxEnumerate, idx_range::RangeBoundsAsRange,
    index_slice::IndexSlice, IdxEnum,
};

use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeInclusive,
        RangeTo, RangeToInclusive,
    },
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IndexArray<I, T, const LEN: usize> {
    data: [T; LEN],
    _phantom: PhantomData<fn(I) -> T>,
}

/// Helper to construct `IndexArray<E, T, { <E as IdxEnum>::COUNT } >`
/// without const generics.
///
/// Use `IndexArray` instead for Arrays that don't have exactly `COUNT` elements.
///
/// ### Example:
/// ```
/// # use indexland::{IdxEnum, index_array::{IndexArray, EnumIndexArray}};
/// #[derive(IdxEnum)]
/// enum Foo { A, B, C }
/// const FOOS: EnumIndexArray<Foo, i32> = IndexArray::new([1, 2, 3]);
/// ```
pub type EnumIndexArray<E, T> = <E as IdxEnum>::EnumIndexArray<T>;

/// Create a [`IndexArray`] containing the arguments.
///
/// If the inputs are constant this creates a compile time constant array.
/// ### Examples:
/// ```
/// use indexland::{IdxEnum, IndexArray, index_array};
///
/// const FOO: IndexArray<u8, i32, 3> = index_array![1, 2, 3];
///
/// const BAR: IndexArray<u8, f32, 42> = index_array![0.0; 42];
///
/// #[derive(IdxEnum)]
/// enum MyId { A, B, C }
///
/// const BAZ: IndexArray<MyId, i32, {MyId::COUNT}> = index_array![
///     MyId::A => 1,
///     MyId::B => 2,
///     MyId::C => 3,
/// ];
/// ```
#[macro_export]
macro_rules! index_array {
    ($($value:expr),+ $(,)?) => {
        $crate::IndexArray::new([$($value),*])
    };
    ($value:expr; $count: expr) => {
        $crate::IndexArray::new([ $value; $count])
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        use core::mem::MaybeUninit;
        const LEN: usize = <[()]>::len(&[$({ stringify!($key); }),*]);
        let keys = [ $($key),* ];
        let mut values = [ $(Some($value)),* ];
        let mut data: [MaybeUninit<_>; LEN] = [
            const { MaybeUninit::uninit() }; LEN
        ];
        let mut i = 0;
        while i < LEN {
            data[keys[i] as usize] = MaybeUninit::new(
                values[i].take().unwrap()
            );
            i += 1;
        }
        // SAFETY: we called `take()` LEN times so all slots must be filled
        let data = unsafe { $crate::__private::transpose_assume_uninit(data) };
        $crate::IndexArray::new(data)
    }};

}

/// Create a [`EnumIndexArray`] containing the arguments.
///
/// If the inputs are constant this creates a compile time constant array.
/// This is an alias for [`index_array!`]
/// ### Examples:
/// ```
/// use indexland::{Idx, EnumIndexArray, enum_index_array};
///
/// #[derive(Idx)]
/// enum MyId { A, B, C }
///
/// const BAZ: EnumIndexArray<MyId, i32> = enum_index_array![
///     MyId::A => 1,
///     MyId::B => 2,
///     MyId::C => 3,
/// ];
/// ```
#[macro_export]
macro_rules! enum_index_array {
    ($($anything: tt)+) => {
        $crate::index_array![$($anything)+]
    };
}

impl<I, T, const LEN: usize> Default for IndexArray<I, T, LEN>
where
    [T; LEN]: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<I: Idx, T, const LEN: usize> IndexArray<I, T, LEN> {
    pub const fn new(data: [T; LEN]) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
    pub fn as_array(&self) -> &[T; LEN] {
        &self.data
    }
    pub fn as_array_mut(&mut self) -> &mut [T; LEN] {
        &mut self.data
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    pub fn as_index_slice(&self) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(&self.data)
    }
    pub fn as_index_slice_mut(&mut self) -> &mut IndexSlice<I, T> {
        IndexSlice::from_slice_mut(&mut self.data)
    }
    pub fn iter_enumerated(&self) -> IdxEnumerate<I, core::slice::Iter<T>> {
        IdxEnumerate::new(I::ZERO, &self.data)
    }
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> IdxEnumerate<I, core::slice::IterMut<T>> {
        IdxEnumerate::new(I::ZERO, &mut self.data)
    }
    pub fn into_iter_enumerated(
        self,
    ) -> IdxEnumerate<I, core::array::IntoIter<T, LEN>> {
        IdxEnumerate::new(I::ZERO, self.data)
    }
    pub fn into_array(self) -> [T; LEN] {
        self.data
    }
}

impl<I: Idx, T, const LEN: usize> From<[T; LEN]> for IndexArray<I, T, LEN> {
    fn from(value: [T; LEN]) -> Self {
        Self::new(value)
    }
}
impl<I: Idx, T, const LEN: usize> From<IndexArray<I, T, LEN>> for [T; LEN] {
    fn from(value: IndexArray<I, T, LEN>) -> Self {
        value.data
    }
}

impl<I: Idx, T, const LEN: usize> Deref for IndexArray<I, T, LEN> {
    type Target = IndexSlice<I, T>;

    fn deref(&self) -> &Self::Target {
        self.as_index_slice()
    }
}

impl<I: Idx, T, const LEN: usize> DerefMut for IndexArray<I, T, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_index_slice_mut()
    }
}

impl<I: Idx, T: Debug, const LEN: usize> Debug for IndexArray<I, T, LEN> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<I: Idx, T, const LEN: usize> Index<I> for IndexArray<I, T, LEN> {
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index.into_usize()]
    }
}

impl<I: Idx, T, const LEN: usize> IndexMut<I> for IndexArray<I, T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index.into_usize()]
    }
}

impl<I: Idx, T, const LEN: usize> IntoIterator for IndexArray<I, T, LEN> {
    type Item = T;

    type IntoIter = core::array::IntoIter<T, LEN>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I: Idx, T, const LEN: usize> IntoIterator
    for &'a IndexArray<I, T, LEN>
{
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, I: Idx, T, const LEN: usize> IntoIterator
    for &'a mut IndexArray<I, T, LEN>
{
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I: Idx, T, const LEN: usize> Index<Range<I>> for IndexArray<I, T, LEN> {
    type Output = IndexSlice<I, T>;

    fn index(&self, index: Range<I>) -> &Self::Output {
        IndexSlice::from_slice(
            &self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T, const LEN: usize> IndexMut<Range<I>>
    for IndexArray<I, T, LEN>
{
    fn index_mut(&mut self, index: Range<I>) -> &mut Self::Output {
        IndexSlice::from_slice_mut(
            &mut self.data[index.start.into_usize()..index.end.into_usize()],
        )
    }
}

impl<I: Idx, T: PartialEq, const LEN: usize> PartialEq<IndexArray<I, T, LEN>>
    for [T; LEN]
{
    fn eq(&self, other: &IndexArray<I, T, LEN>) -> bool {
        self == &other.data
    }
}

impl<I: Idx, T: PartialEq, const LEN: usize> PartialEq<[T; LEN]>
    for IndexArray<I, T, LEN>
{
    fn eq(&self, other: &[T; LEN]) -> bool {
        &self.data == other
    }
}

impl<I: Idx, T: PartialEq, const LEN: usize> PartialEq<IndexArray<I, T, LEN>>
    for [T]
{
    fn eq(&self, other: &IndexArray<I, T, LEN>) -> bool {
        self == other.data
    }
}

impl<I: Idx, T: PartialEq, const LEN: usize> PartialEq<[T]>
    for IndexArray<I, T, LEN>
{
    fn eq(&self, other: &[T]) -> bool {
        self.data == other
    }
}

macro_rules! slice_index_impl {
    ($($range_type: ident),+) => {$(
        impl<I: Idx, T, const LEN: usize> Index<$range_type<I>> for IndexArray<I, T, LEN> {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn index(&self, rb: $range_type<I>) -> &Self::Output {
                IndexSlice::from_slice(&self.data[rb.as_usize_range(self.len())])
            }
        }

        impl<I: Idx, T, const LEN: usize> IndexMut<$range_type<I>> for IndexArray<I, T, LEN> {
            #[inline]
            fn index_mut(&mut self, rb: $range_type<I>) -> &mut Self::Output {
                let range = rb.as_usize_range(self.len());
                IndexSlice::from_slice_mut(&mut self.data[range])
            }
        }
    )*};
}

slice_index_impl!(RangeInclusive, RangeFrom, RangeTo, RangeToInclusive);

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl<I: Idx, T, const LEN: usize> Serialize for IndexArray<I, T, LEN>
where
    [T; LEN]: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, I: Idx, T, const LEN: usize> Deserialize<'de>
    for IndexArray<I, T, LEN>
where
    [T; LEN]: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(<[T; LEN]>::deserialize(deserializer)?))
    }
}
