#![allow(clippy::inline_always)]

use core::ops::{Range, RangeBounds, RangeInclusive};

use crate::Idx;

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
            core::ops::Bound::Included(i) => *i,
            core::ops::Bound::Excluded(i) => *i + I::ONE,
            core::ops::Bound::Unbounded => I::ZERO,
        };
        let end = match self.end_bound() {
            core::ops::Bound::Included(i) => *i + I::ONE,
            core::ops::Bound::Excluded(i) => *i,
            core::ops::Bound::Unbounded => len,
        };
        start..end
    }
    fn as_usize_range(&self, len: usize) -> Range<usize> {
        let start = match self.start_bound() {
            core::ops::Bound::Included(i) => i.into_usize(),
            core::ops::Bound::Excluded(i) => i.into_usize() + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match self.end_bound() {
            core::ops::Bound::Included(i) => i.into_usize() + 1,
            core::ops::Bound::Excluded(i) => i.into_usize(),
            core::ops::Bound::Unbounded => len,
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
