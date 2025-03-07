//! This is a wrapper around [`core::ops::Range`].
//!
//! Ideally this wouldn't have to exist but unfortunately
//! [`core::iter::Step`] is unstable so we cannot implement it for [`Idx`],
//! meaning that you cannot iterate over a `Range<IdxNewtype>`.
//!
//! [`IdxRange`] implements iteration for `Idx` implementors and adds
//! convenient conversions to and from [`Range`].

#![allow(clippy::inline_always)]
use core::ops::{Bound, Range, RangeBounds, RangeInclusive};

use crate::Idx;

pub struct IdxRange<I> {
    pub start: I,
    pub end: I,
}
pub struct IdxRangeInclusive<I> {
    pub start: I,
    pub end: I,
    // when iterating this range, once the end element was reported this range
    // becomes exclusive so iteration stops
    pub exclusive: bool,
}

// `IdxRangeFrom`, `IdxRangeTo`, and `IdxRangeToIncludive`
// would not be iterable so there's no reason for them to exist.

pub struct IdxRangeFrom<I> {
    pub start: I,
}
pub struct IdxRangeTo<I> {
    pub to: I,
}

impl<I> IdxRange<I> {
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

impl<I: Copy> IdxRangeInclusive<I> {
    pub fn new(r: RangeInclusive<I>) -> Self {
        Self {
            start: *r.start(),
            end: *r.end(),
            exclusive: matches!(r.end_bound(), Bound::Excluded(_)),
        }
    }
}

/// we cannot implement the reverse,
/// `impl<I: Idx> From<IdxRangeInclusive<I>> for RangeInclusive<I>`
/// because there's no way to construct a [`RangeInclusive`] in it's exhausted
/// state for non [`Step`](core::iter::Step) indices
impl<I: Idx> From<RangeInclusive<I>> for IdxRangeInclusive<I> {
    fn from(r: RangeInclusive<I>) -> Self {
        IdxRangeInclusive::new(r)
    }
}

// From<IdxRange<I>> for Range<usize> would be overlapping
pub trait RangeAsUsizeRange: Sized {
    fn usize_range(&self) -> Range<usize>;
}

impl<I: Idx> RangeAsUsizeRange for Range<I> {
    fn usize_range(&self) -> Range<usize> {
        Range {
            start: self.start.into_usize(),
            end: self.end.into_usize(),
        }
    }
}
impl<I: Idx> RangeAsUsizeRange for IdxRange<I> {
    fn usize_range(&self) -> Range<usize> {
        Range {
            start: self.start.into_usize(),
            end: self.end.into_usize(),
        }
    }
}

pub trait RangeAsIdxRange<I> {
    fn idx_range(self) -> IdxRange<I>;
}
impl<I: Idx> RangeAsIdxRange<I> for Range<I> {
    fn idx_range(self) -> IdxRange<I> {
        IdxRange::from(self)
    }
}
pub trait RangeInclusiveAsIdxRangeInclusive<I> {
    fn idx_range_inclusive(self) -> IdxRangeInclusive<I>;
}
impl<I: Idx> RangeInclusiveAsIdxRangeInclusive<I> for RangeInclusive<I> {
    fn idx_range_inclusive(self) -> IdxRangeInclusive<I> {
        IdxRangeInclusive::from(self)
    }
}

pub trait UsizeRangeIntoIdxRange: Sized {
    fn into_idx_range<I: Idx>(self) -> IdxRange<I>;
}
impl UsizeRangeIntoIdxRange for Range<usize> {
    fn into_idx_range<I: Idx>(self) -> IdxRange<I> {
        IdxRange::from(Range {
            start: I::from_usize(self.start),
            end: I::from_usize(self.start),
        })
    }
}

pub trait RangeBoundsAsRange<I> {
    fn as_usize_range(&self, len: usize) -> Range<usize>;
    fn as_range(&self, len: I) -> Range<I>;
    fn as_idx_range(&self, len: I) -> IdxRange<I>;
    fn as_idx_range_inclusive(&self, len: I) -> IdxRangeInclusive<I>;
}

impl<I: Idx, RB: RangeBounds<I>> RangeBoundsAsRange<I> for RB {
    fn as_range(&self, len: I) -> Range<I> {
        let start = match self.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + I::ONE,
            Bound::Unbounded => I::ZERO,
        };
        let end = match self.end_bound() {
            Bound::Included(i) => *i + I::ONE,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => len,
        };
        start..end
    }
    fn as_idx_range(&self, len: I) -> IdxRange<I> {
        IdxRange::from(self.as_range(len))
    }
    fn as_idx_range_inclusive(&self, last_idx: I) -> IdxRangeInclusive<I> {
        let mut exclusive = false;
        let start = match self.start_bound() {
            Bound::Included(i) => *i,
            // TODO: we should do some checked add stuff here instead
            // this might overflow for enum indices
            Bound::Excluded(i) => *i + I::ONE,
            Bound::Unbounded => I::ZERO,
        };
        let end = match self.end_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => {
                exclusive = true;
                *i
            }
            Bound::Unbounded => last_idx,
        };
        IdxRangeInclusive {
            start,
            end,
            exclusive,
        }
    }
    fn as_usize_range(&self, len: usize) -> Range<usize> {
        let start = match self.start_bound() {
            Bound::Included(i) => i.into_usize(),
            Bound::Excluded(i) => i.into_usize() + 1,
            Bound::Unbounded => 0,
        };
        let end = match self.end_bound() {
            Bound::Included(i) => i.into_usize() + 1,
            Bound::Excluded(i) => i.into_usize(),
            Bound::Unbounded => len,
        };
        start..end
    }
}

impl<I: Idx> RangeBounds<I> for IdxRange<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Excluded(&self.end)
    }
}
impl<I: Idx> RangeBounds<I> for IdxRangeInclusive<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Included(&self.end)
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

impl<I: Idx> Iterator for IdxRangeInclusive<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        let curr = self.start;
        if curr == self.end {
            if self.exclusive {
                return None;
            }
            self.exclusive = true;
        } else {
            self.start = I::from_usize(curr.into_usize() + 1);
        }
        Some(curr)
    }
}
