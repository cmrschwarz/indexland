#![allow(clippy::inline_always)]

use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign},
};

use crate::{index_range::IndexRangeInclusive, IndexRange};

pub trait Idx:
    Default
    + Debug
    + Copy
    + Ord
    + Hash
    + Add<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + RemAssign
{
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    // We can't use `From<usize>` because e.g. u32 does not implement
    // that, and we can't implement it for it (orphan rule).
    // We also can't have a blanket impl of this trait for types that implement
    // `From<usize>` because then we can't add any manual ones (orphan rule
    // again).

    fn from_usize(v: usize) -> Self;
    fn from_usize_unchecked(v: usize) -> Self;

    fn into_usize(self) -> usize;
    fn into_usize_unchecked(self) -> usize;

    fn saturating_add(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize()
                .saturating_add(other.into_usize())
                .min(Self::MAX.into_usize()),
        )
    }
    fn saturating_sub(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize().saturating_add(other.into_usize()).max(0),
        )
    }
    fn range_to(&self, end: Self) -> IndexRange<Self> {
        IndexRange::new(*self..end)
    }
    fn range_through(&self, end: Self) -> IndexRangeInclusive<Self> {
        IndexRangeInclusive::new(*self..=end)
    }
}

pub trait IdxEnum: Idx + 'static {
    const COUNT: usize;
    const VARIANTS: &'static [Self];

    /// Helper type to construct `IndexArray<Self, T, Self::COUNT>`
    /// on stable Rust without const generics.
    /// See [`EnumIndexArray`](crate::index_array::EnumIndexArray)
    type EnumIndexArray<T>;

    fn iter() -> core::iter::Copied<core::slice::Iter<'static, Self>> {
        Self::VARIANTS.iter().copied()
    }
}

pub trait IdxNewtype: Idx {
    type Base: Idx;
    fn new(inner: Self::Base) -> Self;
    fn into_inner(self) -> Self::Base;
}

pub trait IdxCompatible<I>: Copy {
    fn idx_cast(self) -> I;
    fn into_usize(self) -> usize;
    fn into_usize_unchecked(self) -> usize;
}

impl<I: Idx> IdxCompatible<I> for I {
    fn idx_cast(self) -> I {
        self
    }

    fn into_usize(self) -> usize {
        Idx::into_usize(self)
    }

    fn into_usize_unchecked(self) -> usize {
        Idx::into_usize_unchecked(self)
    }
}

impl Idx for usize {
    const ZERO: usize = 0;
    const ONE: usize = 1;
    const MAX: usize = usize::MAX;
    #[inline(always)]
    fn into_usize(self) -> usize {
        self
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v
    }
    #[inline(always)]
    fn into_usize_unchecked(self) -> usize {
        self
    }
    #[inline(always)]
    fn from_usize_unchecked(v: usize) -> Self {
        v
    }
}

macro_rules! primitive_idx_implemenation_unsized {
    ($($primitive: ident),*) => {$(
        impl Idx for $primitive {
            const ZERO: $primitive = 0;
            const ONE: $primitive = 1;
            const MAX: $primitive = $primitive::MAX;
            #[inline(always)]
            fn into_usize(self) -> usize {
                ::core::convert::TryInto::<usize>::try_into(self).unwrap()
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                ::core::convert::TryInto::<$primitive>::try_into(v).unwrap()
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                self as usize
            }
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                v as $primitive
            }
            fn saturating_add(self, other: Self) -> Self {
                $primitive::saturating_add(self, other)
            }
            fn saturating_sub(self, other: Self) -> Self {
                $primitive::saturating_sub(self, other)
            }
        }
    )*};
}

macro_rules! primitive_idx_implemenation_sized {
    ($($primitive: ident),*) => {$(
        impl Idx for $primitive {
            const ZERO: $primitive = 0;
            const ONE: $primitive = 1;
            const MAX: $primitive = $primitive::MAX;
            #[inline(always)]
            fn into_usize(self) -> usize {
                ::core::convert::TryInto::<usize>::try_into(self).unwrap()
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                ::core::convert::TryInto::<$primitive>::try_into(v).unwrap()
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                self as usize
            }
             #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                v as $primitive
            }
            fn saturating_add(self, other: Self) -> Self {
                $primitive::saturating_add(self, other)
            }
            fn saturating_sub(self, other: Self) -> Self {
                $primitive::saturating_sub(self, other)
            }
        }
    )*};
}

primitive_idx_implemenation_unsized![u8, u16, u32, u64];
primitive_idx_implemenation_sized![isize, i8, i16, i32, i64];

/// Declarative alternative to [`#[derive(IdxNewtype)]`](indexland_derive::IdxNewtype).
///
/// Allows generating multiple indices at once and does not require
/// proc-macros.
///
/// # Example
/// ```rust
/// # use indexland::idx_newtype;
/// idx_newtype! {
///     struct FooId(usize);
///     struct BarId(u32);
/// }
/// ```
#[macro_export]
macro_rules! idx_newtype {
    { $( $(#[$attrs: meta])* $type_vis: vis struct $name: ident ($base_vis: vis $base_type: path); )* } => {$(
        $(#[$attrs])*
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $type_vis struct $name ($base_vis $base_type);

        impl $crate::Idx for $name {
            const ZERO: Self = $name(<$base_type as $crate::Idx>::ZERO);
            const ONE: Self = $name(<$base_type as $crate::Idx>::ONE);
            const MAX: Self = $name(<$base_type as $crate::Idx>::MAX);
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                $name(<$base_type as $crate::Idx>::from_usize(v))
            }
            #[inline(always)]
            fn into_usize(self) -> usize {
                <$base_type as $crate::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                $name(<$base_type as $crate::Idx>::from_usize_unchecked(v))
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                <$base_type as $crate::Idx>::into_usize_unchecked(self.0)
            }
            fn saturating_add(self, other: Self) -> Self {
               $name(<$base_type as  $crate::Idx>::saturating_add(self.0, other.0))
            }
            fn saturating_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::Idx>::saturating_sub(self.0, other.0))
            }
        }
        impl $crate::IdxNewtype for $name {
            type Base = $base_type;
            fn new(v: $base_type) -> Self {
                $name(v)
            }
            fn into_inner(self) -> $base_type {
                self.0
            }
        }
        impl ::core::convert::From<usize> for $name {
            #[inline(always)]
            fn from(v: usize) -> $name {
                $name(<$base_type as $crate::Idx>::from_usize(v))
            }
        }
        impl ::core::convert::From<$name> for usize {
            #[inline(always)]
            fn from(v: $name) -> usize {
                <$base_type as $crate::Idx>::into_usize(v.0)
            }
        }
        impl ::core::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::core::ops::Add for $name {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                $name(self.0 + other.0)
            }
        }
        impl ::core::ops::Sub for $name {
            type Output = Self;
            #[inline]
            fn sub(self, other: Self) -> Self {
                $name(self.0 - other.0)
            }
        }
        impl ::core::ops::AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }
        impl ::core::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
    )*};
}
