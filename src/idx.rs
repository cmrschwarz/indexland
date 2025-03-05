#![allow(clippy::inline_always)]

use std::{
    hash::Hash,
    ops::{
        Add, AddAssign, Range, RangeBounds, RangeInclusive, Sub, SubAssign,
    },
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

pub trait EnumIdx: Idx + 'static {
    const COUNT: usize;
    const VARIANTS: &'static [Self];

    /// Helper type to construct `IndexArray<Self, T, Self::COUNT>`
    /// on stable Rust without const generics.
    /// See [`indexland::index_array::EnumIndexArray`]
    type EnumIndexArray<T>;

    fn iter() -> core::iter::Copied<core::slice::Iter<'static, Self>> {
        Self::VARIANTS.iter().copied()
    }
}

pub trait NewtypeIdx: Idx {
    type Base: Idx;
    fn new(inner: Self::Base) -> Self;
    fn into_inner(self) -> Self::Base;
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
impl<I> From<Range<I>> for IdxRange<I> {
    fn from(r: Range<I>) -> Self {
        IdxRange {
            start: r.start,
            end: r.end,
        }
    }
}
impl<I> From<IdxRange<I>> for Range<I> {
    fn from(r: IdxRange<I>) -> Self {
        Range {
            start: r.start,
            end: r.end,
        }
    }
}

pub trait UsizeRangeAsIdxRange: Sized {
    fn idx_range<I: Idx>(&self) -> IdxRange<I>;
}

impl UsizeRangeAsIdxRange for Range<usize> {
    fn idx_range<I: Idx>(&self) -> IdxRange<I> {
        IdxRange::from(Range {
            start: I::from_usize(self.start),
            end: I::from_usize(self.start),
        })
    }
}

pub trait RangeBoundsAsRange<I> {
    fn as_usize_range(&self, len: usize) -> Range<usize>;
    fn as_range(&self, len: I) -> Range<I>;
    fn as_idx_range(&self, len: I) -> IdxRange<I> {
        IdxRange::from(self.as_range(len))
    }
}

impl<I: Idx, RB: RangeBounds<I>> RangeBoundsAsRange<I> for RB {
    fn as_range(&self, len: I) -> Range<I> {
        let start = match self.start_bound() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => *i + I::ONE,
            std::ops::Bound::Unbounded => I::ZERO,
        };
        let end = match self.end_bound() {
            std::ops::Bound::Included(i) => *i + I::ONE,
            std::ops::Bound::Excluded(i) => *i,
            std::ops::Bound::Unbounded => len,
        };
        start..end
    }
    fn as_usize_range(&self, len: usize) -> Range<usize> {
        let start = match self.start_bound() {
            std::ops::Bound::Included(i) => i.into_usize(),
            std::ops::Bound::Excluded(i) => i.into_usize() + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match self.end_bound() {
            std::ops::Bound::Included(i) => i.into_usize() + 1,
            std::ops::Bound::Excluded(i) => i.into_usize(),
            std::ops::Bound::Unbounded => len,
        };
        start..end
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

/// Declarative alternative to `#[derive(NewtypeIdx)]` that allows generating
/// multiple indices at once and does not require proc-macros.
/// ### Example
/// ```rust
/// # use indexland::{newtype_idx, index_vec::IndexVec};
/// newtype_idx!{
///     struct FooId(usize);
///     struct BarId(u32);
/// }
///
/// struct Foo { /*...*/ }
/// struct Bar { /*...*/ }
/// struct Container {
///     foos: IndexVec<FooId, Foo>,
///     bars: IndexVec<BarId, Bar>,
/// }
/// ```
#[macro_export]
macro_rules! newtype_idx {
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
        impl $crate::NewtypeIdx for $name {
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
