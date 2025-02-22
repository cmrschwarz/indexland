#![allow(clippy::inline_always)]

use std::{
    hash::Hash,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
};

use super::debuggable_nonmax::{
    DebuggableNonMaxU32, DebuggableNonMaxU64, DebuggableNonMaxUsize,
};

pub trait IndexingType:
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
    type IndexBaseType;
    const ZERO: Self;
    const ONE: Self;
    const MAX_VALUE: Self;
    // We can't use `From<usize>` because e.g. u32 does not implement
    // that, and we can't implement it for it (orphan rule).
    // We also can't have a blanket impl of this trait for types that implement
    // `From<usize>` because then we can't add any manual ones
    // (might conflict in future if `From` is added ...).
    fn from_usize(v: usize) -> Self;
    fn into_usize(self) -> usize;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn zero() -> Self {
        <Self as IndexingType>::from_usize(0)
    }
    fn one() -> Self {
        <Self as IndexingType>::from_usize(1)
    }
    fn range_to(&self, end: Self) -> IndexingTypeRange<Self> {
        IndexingTypeRange::new(*self..end)
    }
    fn range_from(&self, start: Self) -> IndexingTypeRange<Self> {
        IndexingTypeRange::new(start..*self)
    }
    fn range_from_zero(&self) -> IndexingTypeRange<Self> {
        self.range_from(Self::zero())
    }
}

pub struct IndexingTypeRange<I> {
    pub start: I,
    pub end: I,
}

impl<I: IndexingType> IndexingTypeRange<I> {
    pub fn new(r: Range<I>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
    pub fn from_zero(end: I) -> Self {
        Self {
            start: I::from_usize(0),
            end,
        }
    }
}

impl<I: IndexingType> Iterator for IndexingTypeRange<I> {
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

impl<I: IndexingType> DoubleEndedIterator for IndexingTypeRange<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        self.end = I::from_usize(self.end.into_usize() - 1);
        Some(self.end)
    }
}

impl<I: IndexingType> From<Range<I>> for IndexingTypeRange<I> {
    fn from(r: Range<I>) -> Self {
        IndexingTypeRange::new(r)
    }
}

impl IndexingType for usize {
    type IndexBaseType = Self;
    const MAX_VALUE: usize = usize::MAX;
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

impl IndexingType for u8 {
    type IndexBaseType = Self;
    const ZERO: u8 = 0;
    const ONE: u8 = 1;
    const MAX_VALUE: u8 = u8::MAX;
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

impl IndexingType for u16 {
    type IndexBaseType = Self;
    const ZERO: u16 = 0;
    const ONE: u16 = 1;
    const MAX_VALUE: u16 = u16::MAX;
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

impl IndexingType for u32 {
    type IndexBaseType = Self;
    const ZERO: u32 = 0;
    const ONE: u32 = 1;
    const MAX_VALUE: u32 = u32::MAX;
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

impl IndexingType for u64 {
    type IndexBaseType = Self;
    const MAX_VALUE: Self = 0;
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

macro_rules! indexing_type_for_nonmax {
    ($nonmax: ident, $primitive: ident) => {
        impl IndexingType for $nonmax {
            type IndexBaseType = Self;
            const MAX_VALUE: Self = $nonmax::MAX;
            const ZERO: Self = $nonmax::ZERO;
            const ONE: Self = $nonmax::ONE;

            #[inline(always)]
            fn into_usize(self) -> usize {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                self.get() as usize
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                $nonmax::new(v as $primitive).unwrap()
            }
            fn wrapping_add(self, other: Self) -> Self {
                $nonmax::wrapping_add(&self, other)
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $nonmax::wrapping_add(&self, other)
            }
        }
    };
}

indexing_type_for_nonmax!(DebuggableNonMaxUsize, usize);
indexing_type_for_nonmax!(DebuggableNonMaxU32, u32);
indexing_type_for_nonmax!(DebuggableNonMaxU64, u64);

#[macro_export]
macro_rules! index_newtype {
    { $( $(#[$attrs: meta])* $type_vis: vis struct $name: ident ($base_vis: vis $base_type: path); )* } => {$(
        $(#[$attrs])*
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $type_vis struct $name ($base_vis $base_type);

        impl $crate::indexing_type::IndexingType for $name {
            type IndexBaseType = $base_type;
            const ZERO: Self = $name(<$base_type as $crate::indexing_type::IndexingType>::ZERO);
            const ONE: Self = $name(<$base_type as $crate::indexing_type::IndexingType>::ONE);
            const MAX_VALUE: Self = $name(<$base_type as $crate::indexing_type::IndexingType>::MAX_VALUE);
            #[inline(always)]
            fn into_usize(self) -> usize {
                <$base_type as $crate::indexing_type::IndexingType>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                $name(<$base_type as $crate::indexing_type::IndexingType>::from_usize(v))
            }
            fn wrapping_add(self, other: Self) -> Self {
               $name(<$base_type as  $crate::indexing_type::IndexingType>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::indexing_type::IndexingType>::wrapping_sub(self.0, other.0))
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
                $name(<$base_type as $crate::indexing_type::IndexingType>::from_usize(v))
            }
        }
        impl From<$name> for usize {
            #[inline(always)]
            fn from(v: $name) -> usize {
                <$base_type as $crate::indexing_type::IndexingType>::into_usize(v.0)
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
