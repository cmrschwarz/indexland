//! Wrappers around [`Range`](`core::ops::Range`),
//! [`RangeInclusive`](`core::ops::RangeInclusive`),
//! and [`RangeFrom`](`core::ops::RangeFrom`)
//! that allow for [`Idx`] based iteration.
//!
//! Ideally these wouldn't have to exist but unfortunately
//! [`core::iter::Step`] is unstable so we cannot implement it for [`Idx`].
//! This means that you cannot iterate over a [`Range<Idx>`].
//! [`IndexRange`] implements iteration for [`Idx`] types and adds
//! conversions to and from [`Range`].
//!
//! You normally don't need this but it's there for you if you do.
//!
//! ## Motivating Example
//! ```compile_fail
//! use indexland::{Idx, IndexVec};
//!
//! #[derive(Idx)]
//! struct MyId(u32);
//!
//! // !! Ranged iteration for custom types does **not** compile in stable Rust.
//! for i in MyId(1)..MyId(3) {
//!     println!("{i}");
//! }
//! ```
//!
//! ## Fix using [`IndexRangeBounds`]:
//! ```
//! use indexland::{Idx, IndexRangeBounds, IndexVec};
//!
//! #[derive(Idx)]
//! struct MyId(u32);
//!
//! // `index_range` is provided by the `IndexRangeBounds` extension trait
//! for i in (MyId(1)..MyId(3)).index_range() {
//!     println!("{i}");
//! }
//! ```
//!
//! ## Alternative for Collections
//! ```
//! # #![cfg(feature="derive")]
//! use indexland::{Idx, IndexVec};
//!
//! #[derive(Idx)]
//! struct MyId(u32);
//!
//! // example values
//! let myvec: IndexVec<MyId, i32> = IndexVec::from_iter(0..10);
//! let start = MyId(1);
//! let end = MyId(3);
//!
//! // Where applicable, this is the recommended style.
//! for (i, &v) in myvec.iter_enumerated_range(start..end) {
//!     println!("myvec[{i}] = {v}");
//! }
//! ```
use crate::Idx;
use core::ops::{
    Add, Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive, Sub,
};

pub trait IndexRangeBounds<I>: RangeBounds<I> {
    type BaseRange: IndexRangeBounds<I>;
    type IndexRange: IndexRangeBounds<I>;
    type UsizeRange: IndexRangeBounds<usize>;
    fn base_range(self) -> Self::BaseRange;
    fn index_range(self) -> Self::IndexRange;
    fn usize_range(self) -> Self::UsizeRange;
    fn canonicalize(self, len: usize) -> Range<usize>;
}

/// Mirror of [`core::ops::Range`].
/// See this module's [documentation](self) for justification.
#[derive(Clone, Default, PartialEq, Eq, Hash)] // not `Copy`, mirroring std
pub struct IndexRange<I> {
    pub start: I,
    pub end: I,
}

/// Mirror of [`core::ops::RangeInclusive`].
/// See this module's [documentation](self) for justification.
/// Note: there's no way for us to implement
/// `From<IndexRangeInclusive<I>> for RangeInclusive<I>`
/// as there's unfortunately no way to construct an exhausted inclusive range
/// for an `I` that does not implement [`Step`](core::iter::Step).
// NB: the above holds even if we tried to implement this by wrapping
// `RangeInclusive<I>` itself. Then there's no way for us to
// correctly implement `Iterator`.
#[derive(Clone, Default, PartialEq, Eq, Hash)] // not `Copy`, mirroring std
pub struct IndexRangeInclusive<I> {
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
#[derive(Clone, Default, PartialEq, Eq, Hash)] // not `Copy`, mirroring std
pub struct IndexRangeFrom<I> {
    pub start: I,
}

impl<I> IndexRange<I> {
    pub fn new(r: Range<I>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}
impl<I> From<Range<I>> for IndexRange<I> {
    fn from(r: Range<I>) -> Self {
        IndexRange {
            start: r.start,
            end: r.end,
        }
    }
}
impl<I> From<IndexRange<I>> for Range<I> {
    fn from(r: IndexRange<I>) -> Self {
        Range {
            start: r.start,
            end: r.end,
        }
    }
}

impl<I> IndexRangeInclusive<I> {
    pub fn new(r: RangeInclusive<I>) -> Self
    where
        I: Copy,
    {
        Self {
            start: *r.start(),
            end: *r.end(),
            exclusive: matches!(r.end_bound(), Bound::Excluded(_)),
        }
    }
}

/// We unfortunately cannot implement the reverse:
/// `impl<I> From<IndexRangeInclusive<I>> for RangeInclusive<I>`
/// because there's no way to construct a [`RangeInclusive`] in it's exhausted
/// state for non [`Step`](core::iter::Step) indices.
impl<I> From<RangeInclusive<I>> for IndexRangeInclusive<I>
where
    I: Copy,
{
    fn from(r: RangeInclusive<I>) -> Self {
        IndexRangeInclusive::new(r)
    }
}

impl<I> IndexRangeFrom<I> {
    pub fn new(r: RangeFrom<I>) -> Self {
        Self { start: r.start }
    }
}
impl<I> From<RangeFrom<I>> for IndexRangeFrom<I> {
    fn from(r: RangeFrom<I>) -> Self {
        IndexRangeFrom { start: r.start }
    }
}
impl<I> From<IndexRangeFrom<I>> for RangeFrom<I> {
    fn from(r: IndexRangeFrom<I>) -> Self {
        RangeFrom { start: r.start }
    }
}

impl<I> RangeBounds<I> for IndexRange<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Excluded(&self.end)
    }
}
impl<I: Idx> IndexRangeBounds<I> for IndexRange<I> {
    type BaseRange = Range<I>;
    type IndexRange = IndexRange<I>;
    type UsizeRange = Range<usize>;
    fn base_range(self) -> Self::BaseRange {
        Range::from(self)
    }
    fn index_range(self) -> Self::IndexRange {
        self
    }
    fn usize_range(self) -> Self::UsizeRange {
        Range {
            start: self.start.into_usize(),
            end: self.end.into_usize(),
        }
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        self.usize_range()
    }
}

impl<I> RangeBounds<I> for IndexRangeInclusive<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Included(&self.end)
    }
}
impl<I: Idx> IndexRangeBounds<I> for IndexRangeInclusive<I> {
    type BaseRange = IndexRangeInclusive<I>;
    type IndexRange = IndexRangeInclusive<I>;
    type UsizeRange = IndexRangeInclusive<usize>;
    /// !NOTE: this is a hack. Unfortunately, there's no way to construct an
    /// exhaustive [`RangeInclusive`] for a T that isn't `Step`, which we can't
    /// implement for `Idx` because it is unstable.
    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        self
    }
    fn usize_range(self) -> Self::UsizeRange {
        IndexRangeInclusive {
            start: self.start.into_usize(),
            end: self.end.into_usize(),
            exclusive: self.exclusive,
        }
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        Range {
            start: self.start.into_usize(),
            end: self.end.into_usize() + usize::from(self.exclusive),
        }
    }
}

impl<I> RangeBounds<I> for IndexRangeFrom<I> {
    fn start_bound(&self) -> Bound<&I> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&I> {
        Bound::Unbounded
    }
}
impl<I: Idx> IndexRangeBounds<I> for IndexRangeFrom<I> {
    type BaseRange = RangeFrom<I>;
    type IndexRange = IndexRangeFrom<I>;
    type UsizeRange = RangeFrom<usize>;
    fn base_range(self) -> Self::BaseRange {
        RangeFrom::from(self)
    }
    fn index_range(self) -> Self::IndexRange {
        self
    }
    fn usize_range(self) -> Self::UsizeRange {
        RangeFrom {
            start: self.start.into_usize(),
        }
    }
    fn canonicalize(self, len: usize) -> Range<usize> {
        Range {
            start: self.start.into_usize(),
            end: len,
        }
    }
}

impl<I: Idx> IndexRangeBounds<I> for Range<I> {
    type BaseRange = Range<I>;
    type IndexRange = IndexRange<I>;
    type UsizeRange = Range<usize>;
    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        IndexRange::from(self)
    }
    fn usize_range(self) -> Self::UsizeRange {
        Range {
            start: self.start.into_usize(),
            end: self.end.into_usize(),
        }
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        self.usize_range()
    }
}

impl<I: Idx> IndexRangeBounds<I> for RangeInclusive<I> {
    type BaseRange = RangeInclusive<I>;
    type IndexRange = IndexRangeInclusive<I>;
    type UsizeRange = RangeInclusive<usize>;
    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        IndexRangeInclusive::from(self)
    }
    fn usize_range(self) -> Self::UsizeRange {
        let mut range = RangeInclusive::new(self.start().into_usize(), self.end().into_usize());
        if matches!(self.end_bound(), Bound::Excluded(_)) {
            if range.start() != range.end() {
                return RangeInclusive::new(*range.start(), *range.end() - 1);
            }
            let _ = range.next();
            return range;
        }
        range
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        Range {
            start: self.start().into_usize(),
            end: self.end().into_usize()
                + usize::from(matches!(self.end_bound(), Bound::Included(_))),
        }
    }
}

impl<I: Idx> IndexRangeBounds<I> for RangeFrom<I> {
    type BaseRange = RangeFrom<I>;
    type IndexRange = IndexRangeFrom<I>;
    type UsizeRange = RangeFrom<usize>;
    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        IndexRangeFrom::from(self)
    }
    fn usize_range(self) -> Self::UsizeRange {
        RangeFrom {
            start: self.start.into_usize(),
        }
    }
    fn canonicalize(self, len: usize) -> Range<usize> {
        Range {
            start: self.start.into_usize(),
            end: len,
        }
    }
}

impl<I: Idx> IndexRangeBounds<I> for RangeTo<I> {
    type BaseRange = RangeTo<I>;
    type IndexRange = RangeTo<I>;
    type UsizeRange = RangeTo<usize>;
    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        self
    }
    fn usize_range(self) -> Self::UsizeRange {
        RangeTo {
            end: self.end.into_usize(),
        }
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        Range {
            start: 0,
            end: self.end.into_usize(),
        }
    }
}

impl<I: Idx> IndexRangeBounds<I> for RangeToInclusive<I> {
    type BaseRange = RangeToInclusive<I>;
    type IndexRange = RangeToInclusive<I>;
    type UsizeRange = RangeToInclusive<usize>;

    fn base_range(self) -> Self::BaseRange {
        self
    }
    fn index_range(self) -> Self::IndexRange {
        self
    }
    fn usize_range(self) -> Self::UsizeRange {
        RangeToInclusive {
            end: self.end.into_usize(),
        }
    }
    fn canonicalize(self, _len: usize) -> Range<usize> {
        Range {
            start: 0,
            end: self.end.into_usize() + 1,
        }
    }
}

impl<I> IndexRangeBounds<I> for RangeFull {
    type BaseRange = RangeFull;
    type IndexRange = RangeFull;
    type UsizeRange = RangeFull;
    fn base_range(self) -> Self::BaseRange {
        Self
    }
    fn index_range(self) -> Self::IndexRange {
        Self
    }
    fn usize_range(self) -> Self::UsizeRange {
        Self
    }
    fn canonicalize(self, len: usize) -> Range<usize> {
        Range { start: 0, end: len }
    }
}

impl<I: Idx + Ord + Add<Output = I>> Iterator for IndexRange<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        if self.start == self.end {
            return None;
        }
        let curr = self.start;
        self.start = self.start + I::ONE;
        Some(curr)
    }
}
impl<I: Idx + Ord + Add<Output = I> + Sub<Output = I>> DoubleEndedIterator for IndexRange<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        self.end = self.end - I::ONE;
        Some(self.end)
    }
}

impl<I: Idx + Ord + Add<Output = I>> Iterator for IndexRangeInclusive<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        let curr = self.start;
        if curr == self.end {
            if self.exclusive {
                return None;
            }
            self.exclusive = true;
        } else {
            self.start = self.start + I::ONE;
        }
        Some(curr)
    }
}
impl<I: Idx + Ord + Add<Output = I> + Sub<Output = I>> DoubleEndedIterator
    for IndexRangeInclusive<I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let curr = self.end;
        if self.start == curr {
            if self.exclusive {
                return None;
            }
            self.exclusive = true;
        } else {
            self.end = self.end - I::ONE;
        }
        Some(curr)
    }
}

impl<I: Idx + Add<Output = I>> Iterator for IndexRangeFrom<I> {
    type Item = I;
    fn next(&mut self) -> Option<I> {
        let curr = self.start;
        // NOTE: this might overflow or wrap. This is intentional and the
        // same that std does.
        self.start = self.start + I::ONE;
        Some(curr)
    }
}
