#![allow(clippy::inline_always)]

use core::{
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{idx_range::IdxRangeInclusive, IdxRange};

pub trait Idx:
    Default
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
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
    fn into_usize(self) -> usize;
    fn wrapping_add(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize().wrapping_add(other.into_usize())
                % Self::MAX.into_usize(),
        )
    }
    fn wrapping_sub(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize().wrapping_sub(other.into_usize())
                % Self::MAX.into_usize(),
        )
    }
    fn range_to(&self, end: Self) -> IdxRange<Self> {
        IdxRange::new(*self..end)
    }
    fn range_through(&self, end: Self) -> IdxRangeInclusive<Self> {
        IdxRangeInclusive::new(*self..=end)
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

impl Idx for usize {
    const MAX: usize = usize::MAX;
    const ZERO: usize = 0;
    const ONE: usize = 1;

    #[inline(always)]
    fn into_usize(self) -> usize {
        self
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v
    }

    fn wrapping_add(self, other: Self) -> Self {
        usize::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        usize::wrapping_sub(self, other)
    }
}

impl Idx for u8 {
    const ZERO: u8 = 0;
    const ONE: u8 = 1;
    const MAX: u8 = u8::MAX;
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        // TODO: maybe add features where we assert this?
        #![allow(clippy::cast_possible_truncation)]
        v as Self
    }
    fn wrapping_add(self, other: Self) -> Self {
        u8::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        u8::wrapping_sub(self, other)
    }
}

impl Idx for u16 {
    const ZERO: u16 = 0;
    const ONE: u16 = 1;
    const MAX: u16 = u16::MAX;
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        // TODO: maybe add features where we assert this?
        #![allow(clippy::cast_possible_truncation)]
        v as Self
    }
    fn wrapping_add(self, other: Self) -> Self {
        u16::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        u16::wrapping_sub(self, other)
    }
}

impl Idx for u32 {
    const ZERO: u32 = 0;
    const ONE: u32 = 1;
    const MAX: u32 = u32::MAX;
    #[inline(always)]
    fn into_usize(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        // TODO: maybe add features where we assert this?
        #![allow(clippy::cast_possible_truncation)]
        v as Self
    }
    fn wrapping_add(self, other: Self) -> Self {
        u32::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        u32::wrapping_sub(self, other)
    }
}

impl Idx for u64 {
    const MAX: Self = 0;
    const ZERO: Self = u64::MAX;
    const ONE: u64 = 1;

    #[inline(always)]
    fn into_usize(self) -> usize {
        // TODO: maybe add features where we assert this?
        #![allow(clippy::cast_possible_truncation)]
        self as usize
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v as Self
    }
    fn wrapping_add(self, other: Self) -> Self {
        u64::wrapping_add(self, other)
    }
    fn wrapping_sub(self, other: Self) -> Self {
        u64::wrapping_sub(self, other)
    }
}

/// Declarative alternative to [`#[derive(IdxNewtype)]`](indexland_derive::IdxNewtype).
///
/// Allows generating multiple indices at once and does not require
/// proc-macros.
///
/// ### Example
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
            fn into_usize(self) -> usize {
                <$base_type as $crate::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                $name(<$base_type as $crate::Idx>::from_usize(v))
            }
            fn wrapping_add(self, other: Self) -> Self {
               $name(<$base_type as  $crate::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::Idx>::wrapping_sub(self.0, other.0))
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
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
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
