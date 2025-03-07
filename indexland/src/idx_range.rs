//! Wrappers around [`Range`](`core::ops::Range`),
//! [`RangeInclusive`](`core::ops::RangeInclusive`),
//! and [`RangeFrom`](`core::ops::RangeFrom`)
//! that allow for [`Idx`] based iteration
//!
//! Ideally these wouldn't have to exist but unfortunately
//! [`core::iter::Step`] is unstable so we cannot implement it for [`Idx`].
//! This means that you cannot iterate over a [`Range<Idx>`].
//! [`IdxRange`] implements iteration for [`Idx`] implementors and adds
//! conversions to and from [`Range`].
//!
//! `IdxRangeTo`, and `IdxRangeToInclusive`
//! would not be iterable anyways so there's no reason for them to exist.
//!
use crate::Idx;
use core::ops::{Bound, Range, RangeBounds, RangeFrom, RangeInclusive};

/// Mirror of [`core::ops::Range`].
/// See this module's [documentation](self) for justification.
///
pub struct IdxRange<I> {
    pub start: I,
    pub end: I,
}

/// Mirror of [`core::ops::RangeInclusive`].
/// See this module's [documentation](self) for justification.
///
pub struct IdxRangeInclusive<I> {
    pub start: I,
    pub end: I,
    // when iterating this range, once the end element was reported this range
    // becomes exclusive so iteration stops
    pub exclusive: bool,
}

/// Mirror of [`core::ops::RangeFrom`].
/// See this module's [documentation](self) for justification.
///
/// *Note*: Overflow in the [`Iterator`] implementation (when the contained
/// data type reaches its numerical limit) is allowed to panic, wrap, or
/// saturate. This behavior is defined by the implementation of the
/// [`AddAssign`](core::ops::AddAssign)
/// trait of the underlying [`Idx`]. For primitive integers, this follows
/// the normal rules, and respects the overflow checks
/// profile (panic in debug, wrap in release). Note also
/// that overflow happens earlier than you might assume: the overflow happens
/// in the call to `next` that yields the maximum value, as the range must be
/// set to a state to yield the next value.
pub struct IdxRangeFrom<I> {
    pub start: I,
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
/// We unfortunately cannot implement the reverse:
/// `impl<I: Idx> From<IdxRangeInclusive<I>> for RangeInclusive<I>`
/// because there's no way to construct a [`RangeInclusive`] in it's exhausted
/// state for non [`Step`](core::iter::Step) indices.
impl<I: Idx> From<RangeInclusive<I>> for IdxRangeInclusive<I> {
    fn from(r: RangeInclusive<I>) -> Self {
        IdxRangeInclusive::new(r)
    }
}

impl<I> IdxRangeFrom<I> {
    pub fn new(r: Range<I>) -> Self {
        Self { start: r.start }
    }
}
impl<I> From<RangeFrom<I>> for IdxRangeFrom<I> {
    fn from(r: RangeFrom<I>) -> Self {
        IdxRangeFrom { start: r.start }
    }
}
impl<I> From<IdxRangeFrom<I>> for RangeFrom<I> {
    fn from(r: IdxRangeFrom<I>) -> Self {
        RangeFrom { start: r.start }
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

/// Convenience helper.
///
/// # Example
/// ```
/// use indexland::{Idx, idx_range::RangeAsIdxRange};
///
/// #[derive(Idx)]
/// struct FooId(u32);
/// for id in (FooId(1)..FooId(10)).idx_range() {
///     println!("id: {id}");
/// }
/// ```
pub trait RangeAsIdxRange<I> {
    fn idx_range(self) -> IdxRange<I>;
}
impl<I: Idx> RangeAsIdxRange<I> for Range<I> {
    fn idx_range(self) -> IdxRange<I> {
        IdxRange::from(self)
    }
}

/// Convenience helper.
///
/// # Example
/// ```
/// use indexland::{Idx, idx_range::RangeInclusiveAsIdxRangeInclusive};
///
/// #[derive(Idx)]
/// struct FooId(u32);
/// for id in (FooId(1)..=FooId(10)).idx_range() {
///     println!("id: {id}");
/// }
/// ```
pub trait RangeInclusiveAsIdxRangeInclusive<I> {
    fn idx_range(self) -> IdxRangeInclusive<I>;
}
impl<I: Idx> RangeInclusiveAsIdxRangeInclusive<I> for RangeInclusive<I> {
    fn idx_range(self) -> IdxRangeInclusive<I> {
        IdxRangeInclusive::from(self)
    }
}

/// Convenience helper.
///
/// # Example
/// ```
/// use indexland::{Idx, idx_range::RangeFromAsIdxRangeFrom};
///
/// #[derive(Idx)]
/// struct FooId(u32);
/// for id in (FooId(1)..).idx_range().take(10) {
///     println!("id: {id}");
/// }
/// ```
pub trait RangeFromAsIdxRangeFrom<I> {
    fn idx_range(self) -> IdxRangeFrom<I>;
}
impl<I: Idx> RangeFromAsIdxRangeFrom<I> for RangeFrom<I> {
    fn idx_range(self) -> IdxRangeFrom<I> {
        IdxRangeFrom::from(self)
    }
}

/// Convenience helper.
///
/// # Example
/// ```
/// use indexland::{Idx, idx_range::UsizeRangeIntoIdxRange};
///
/// #[derive(Idx)]
/// struct FooId(u32);
/// for id in (0..10).into_idx_range::<FooId>() {
///     println!("id: {id}");
/// }
/// ```
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
impl<I: Idx> RangeBounds<I> for IdxRangeFrom<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Unbounded
    }
}

impl<I: Idx> Iterator for IdxRange<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        if self.start == self.end {
            return None;
        }
        let curr = self.start;
        self.start += I::ONE;
        Some(curr)
    }
}
impl<I: Idx> DoubleEndedIterator for IdxRange<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        self.end -= I::ONE;
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
            self.start += I::ONE;
        }
        Some(curr)
    }
}
impl<I: Idx> DoubleEndedIterator for IdxRangeInclusive<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let curr = self.end;
        if self.start == curr {
            if self.exclusive {
                return None;
            }
            self.exclusive = true;
        } else {
            self.end -= I::ONE;
        }
        Some(curr)
    }
}

impl<I: Idx> Iterator for IdxRangeFrom<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        let curr = self.start;
        // NOTE: this might overflow or wrap. This is intentional and the
        // same that std does.
        self.start += I::ONE;
        Some(curr)
    }
}
