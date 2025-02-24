#![allow(clippy::inline_always)]

use std::{
    hash::Hash,
    ops::{Add, AddAssign, Range, RangeInclusive, Sub, SubAssign},
};

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
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn range_to(&self, end: Self) -> IdxRange<Self> {
        IdxRange::new(*self..end)
    }
    fn range_through(&self, end: Self) -> IdxRangeInclusive<Self> {
        IdxRangeInclusive::new(*self..=end)
    }
}

pub struct IdxRange<I> {
    pub start: I,
    pub end: I,
}

impl<I: Idx> IdxRange<I> {
    pub fn new(r: Range<I>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

impl<I: Idx> Iterator for IdxRange<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.start == self.end {
            return None;
        }
        let curr = self.start;
        self.start = I::from_usize(curr.into_usize() + 1);
        Some(curr)
    }
}

impl<I: Idx> DoubleEndedIterator for IdxRange<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        self.end = I::from_usize(self.end.into_usize() - 1);
        Some(self.end)
    }
}

impl<I: Idx> From<Range<I>> for IdxRange<I> {
    fn from(r: Range<I>) -> Self {
        IdxRange::new(r)
    }
}

pub struct IdxRangeInclusive<I> {
    pub start: I,
    pub end: I,
    pub exhausted: bool,
}

impl<I: Idx> IdxRangeInclusive<I> {
    pub fn new(r: RangeInclusive<I>) -> Self {
        Self {
            start: *r.start(),
            end: *r.end(),
            exhausted: false,
        }
    }
}

impl<I: Idx> Iterator for IdxRangeInclusive<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.exhausted {
            return None;
        }
        let curr = self.start;
        if curr == self.end {
            self.exhausted = true;
        } else {
            self.start = I::from_usize(curr.into_usize() + 1);
        }
        Some(curr)
    }
}

impl<I: Idx> From<RangeInclusive<I>> for IdxRangeInclusive<I> {
    fn from(r: RangeInclusive<I>) -> Self {
        IdxRangeInclusive::new(r)
    }
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

#[macro_export]
macro_rules! index_newtype {
    { $( $(#[$attrs: meta])* $type_vis: vis struct $name: ident ($base_vis: vis $base_type: path); )* } => {$(
        $(#[$attrs])*
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $type_vis struct $name ($base_vis $base_type);

        impl $crate::idx::Idx for $name {
            const ZERO: Self = $name(<$base_type as $crate::idx::Idx>::ZERO);
            const ONE: Self = $name(<$base_type as $crate::idx::Idx>::ONE);
            const MAX: Self = $name(<$base_type as $crate::idx::Idx>::MAX);
            #[inline(always)]
            fn into_usize(self) -> usize {
                <$base_type as $crate::idx::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                $name(<$base_type as $crate::idx::Idx>::from_usize(v))
            }
            fn wrapping_add(self, other: Self) -> Self {
               $name(<$base_type as  $crate::idx::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::idx::Idx>::wrapping_sub(self.0, other.0))
            }
        }

        #[allow(unused)]
        impl $name {
            pub const fn new(v: $base_type) -> Self {
                $name(v)
            }
            pub fn into_inner(self) -> $base_type {
                self.0
            }
        }

        impl From<usize> for $name {
            #[inline(always)]
            fn from(v: usize) -> $name {
                $name(<$base_type as $crate::idx::Idx>::from_usize(v))
            }
        }
        impl From<$name> for usize {
            #[inline(always)]
            fn from(v: $name) -> usize {
                <$base_type as $crate::idx::Idx>::into_usize(v.0)
            }
        }

        impl std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $name(self.0 + other.0)
            }
        }
        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                $name(self.0 - other.0)
            }
        }
        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }
        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
    )*};
}
