use core::ops::{Range, RangeFull};

use crate::{
    Idx, IndexRange, IndexRangeBounds, IndexRangeFrom, IndexRangeInclusive, idx::IdxCompat,
};

#[allow(clippy::len_without_is_empty)]
pub trait Sequence {
    type Index;
    type Element: ?Sized;
    type Slice<X: IdxCompat<Self::Index>>: ?Sized;

    fn len(&self) -> usize;

    fn get(&self, idx: usize) -> Option<&Self::Element>;

    fn index(&self, idx: usize) -> &Self::Element;

    fn get_range<X: IdxCompat<Self::Index>>(&self, r: Range<usize>) -> Option<&Self::Slice<X>>;

    fn index_range<X: IdxCompat<Self::Index>>(&self, r: Range<usize>) -> &Self::Slice<X>;
}

/// ## Safety
/// `get_unchecked` and `get_range_unchecked` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait UnsafeSequence: Sequence {
    /// ## Safety
    /// `this` must be a valid container pointer
    ///
    /// ## Tree Borrows
    /// For now, the caller must assume that this may turn the provided
    /// pointer into a `&Self` temporarily to perform this operation.
    unsafe fn len_from_ptr(this: *const Self) -> usize;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_unchecked(this: *const Self, idx: usize) -> *const Self::Element;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_range_unchecked<X: IdxCompat<Self::Index>>(
        this: *const Self,
        r: Range<usize>,
    ) -> *const Self::Slice<X>;
}

pub trait SequenceMut: Sequence {
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Element>;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Element;

    fn get_range_mut<X: IdxCompat<Self::Index>>(
        &mut self,
        r: Range<usize>,
    ) -> Option<&mut Self::Slice<X>>;

    fn index_range_mut<X: IdxCompat<Self::Index>>(
        &mut self,
        r: Range<usize>,
    ) -> &mut Self::Slice<X>;
}

/// ## Safety
/// `get_unchecked_mut` and `get_range_unchecked_mut` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait UnsafeSequenceMut: UnsafeSequence + SequenceMut {
    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_unchecked_mut(this: *mut Self, idx: usize) -> *mut Self::Element;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_range_unchecked_mut<X: IdxCompat<Self::Index>>(
        this: *mut Self,
        r: Range<usize>,
    ) -> *mut Self::Slice<X>;
}

impl<T> Sequence for [T] {
    type Index = usize;
    type Element = T;
    type Slice<I: IdxCompat<usize>> = [T];

    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> Option<&Self::Element> {
        <[T]>::get(self, idx)
    }

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Element {
        core::ops::Index::index(self, idx)
    }

    #[inline(always)]
    fn get_range<X: IdxCompat<usize>>(&self, r: Range<usize>) -> Option<&Self::Slice<X>> {
        <[T]>::get(self, r)
    }

    #[inline(always)]
    fn index_range<X: IdxCompat<usize>>(&self, r: Range<usize>) -> &Self::Slice<X> {
        core::ops::Index::index(self, r)
    }
}

unsafe impl<T> UnsafeSequence for [T] {
    #[inline(always)]
    unsafe fn len_from_ptr(this: *const Self) -> usize {
        this.len()
    }

    #[inline(always)]
    unsafe fn get_unchecked(this: *const Self, idx: usize) -> *const Self::Element {
        unsafe { this.cast::<T>().add(idx) }
    }

    #[inline(always)]
    unsafe fn get_range_unchecked<X: IdxCompat<usize>>(
        this: *const Self,
        r: Range<usize>,
    ) -> *const Self::Slice<X> {
        unsafe {
            core::ptr::slice_from_raw_parts(this.cast::<T>().add(r.start), r.end - r.start) as _
        }
    }
}

impl<T> SequenceMut for [T] {
    #[inline(always)]
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Element> {
        <[T]>::get_mut(self, idx)
    }

    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Element {
        core::ops::IndexMut::index_mut(self, idx)
    }

    #[inline(always)]
    fn get_range_mut<X: IdxCompat<usize>>(
        &mut self,
        r: Range<usize>,
    ) -> Option<&mut Self::Slice<X>> {
        <[T]>::get_mut(self, r)
    }

    #[inline(always)]
    fn index_range_mut<X: IdxCompat<usize>>(&mut self, r: Range<usize>) -> &mut Self::Slice<X> {
        core::ops::IndexMut::index_mut(self, r)
    }
}

unsafe impl<T> UnsafeSequenceMut for [T] {
    #[inline(always)]
    unsafe fn get_unchecked_mut(this: *mut Self, idx: usize) -> *mut Self::Element {
        unsafe { this.cast::<T>().add(idx) }
    }

    #[inline(always)]
    unsafe fn get_range_unchecked_mut<X: IdxCompat<usize>>(
        this: *mut Self,
        r: Range<usize>,
    ) -> *mut Self::Slice<X> {
        unsafe {
            core::ptr::slice_from_raw_parts_mut(this.cast::<T>().add(r.start), r.end - r.start) as _
        }
    }
}

/// ## Safety
/// `get_unchecked` and `get_unchecked_mut` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait SequenceIndex<I, S: ?Sized>: Sized {
    type Output: ?Sized;

    fn get(self, container: &S) -> Option<&Self::Output>
    where
        S: Sequence;

    fn get_mut(self, container: &mut S) -> Option<&mut Self::Output>
    where
        S: SequenceMut;

    fn index(self, container: &S) -> &Self::Output
    where
        S: Sequence;

    fn index_mut(self, container: &mut S) -> &mut Self::Output
    where
        S: SequenceMut;

    /// ## Safety
    /// the container pointer must be valid
    unsafe fn get_unchecked<FS, FR>(self, container: *const S) -> *const Self::Output
    where
        S: UnsafeSequence;

    /// ## Safety
    /// the container pointer must be valid
    unsafe fn get_unchecked_mut(self, container: *mut S) -> *mut Self::Output
    where
        S: UnsafeSequenceMut;
}

unsafe impl<I, S, X> SequenceIndex<I, S> for X
where
    I: Idx,
    S: Sequence<Index = I> + ?Sized,
    X: IdxCompat<I>,
{
    type Output = S::Element;

    fn get(self, container: &S) -> Option<&Self::Output> {
        S::get(container, self.into_usize())
    }

    fn get_mut(self, container: &mut S) -> Option<&mut Self::Output>
    where
        S: SequenceMut,
    {
        S::get_mut(container, self.into_usize())
    }

    fn index(self, container: &S) -> &Self::Output {
        S::index(container, self.into_usize())
    }

    fn index_mut(self, container: &mut S) -> &mut Self::Output
    where
        S: SequenceMut,
    {
        S::index_mut(container, self.into_usize())
    }

    unsafe fn get_unchecked<FS, FR>(self, container: *const S) -> *const Self::Output
    where
        S: UnsafeSequence,
    {
        unsafe { S::get_unchecked(container, self.into_usize()) }
    }

    unsafe fn get_unchecked_mut(self, container: *mut S) -> *mut Self::Output
    where
        S: UnsafeSequenceMut,
    {
        unsafe { S::get_unchecked_mut(container, self.into_usize()) }
    }
}

unsafe impl<I, S, X> SequenceIndex<I, S> for Range<X>
where
    I: Idx,
    S: Sequence<Index = I> + ?Sized,
    X: IdxCompat<I>,
{
    type Output = S::Slice<X>;

    fn get(self, container: &S) -> Option<&Self::Output> {
        S::get_range(container, self.usize_range())
    }

    fn get_mut(self, container: &mut S) -> Option<&mut Self::Output>
    where
        S: SequenceMut,
    {
        S::get_range_mut(container, self.usize_range())
    }

    fn index(self, container: &S) -> &Self::Output {
        S::index_range(container, self.usize_range())
    }

    fn index_mut(self, container: &mut S) -> &mut Self::Output
    where
        S: SequenceMut,
    {
        S::index_range_mut(container, self.usize_range())
    }

    unsafe fn get_unchecked<FS, FR>(self, container: *const S) -> *const Self::Output
    where
        S: UnsafeSequence,
    {
        unsafe { S::get_range_unchecked(container, self.usize_range()) }
    }

    unsafe fn get_unchecked_mut(self, container: *mut S) -> *mut Self::Output
    where
        S: UnsafeSequenceMut,
    {
        unsafe { S::get_range_unchecked_mut(container, self.usize_range()) }
    }
}

unsafe impl<I, S> SequenceIndex<I, S> for RangeFull
where
    I: Idx,
    S: Sequence<Index = I> + ?Sized,
{
    type Output = S::Slice<I>;

    fn get(self, seq: &S) -> Option<&Self::Output> {
        S::get_range(seq, 0..seq.len())
    }

    fn get_mut(self, seq: &mut S) -> Option<&mut Self::Output>
    where
        S: SequenceMut,
    {
        S::get_range_mut(seq, 0..seq.len())
    }

    fn index(self, seq: &S) -> &Self::Output {
        S::index_range(seq, 0..seq.len())
    }

    fn index_mut(self, seq: &mut S) -> &mut Self::Output
    where
        S: SequenceMut,
    {
        S::index_range_mut(seq, 0..seq.len())
    }

    unsafe fn get_unchecked<FS, FR>(self, seq: *const S) -> *const Self::Output
    where
        S: UnsafeSequence,
    {
        unsafe { S::get_range_unchecked(seq, 0..S::len_from_ptr(seq)) }
    }

    unsafe fn get_unchecked_mut(self, seq: *mut S) -> *mut Self::Output
    where
        S: UnsafeSequenceMut,
    {
        unsafe { S::get_range_unchecked_mut(seq, 0..S::len_from_ptr(seq)) }
    }
}

macro_rules! index_slice_partial_range_impl {
    ($($range: path),*) => {$(
        unsafe impl<I, S, X> SequenceIndex<I, S> for $range
        where
            I: Idx,
            S: Sequence<Index = I> + ?Sized,
            X: IdxCompat<I>
        {
            type Output = S::Slice<X>;

            fn get(self, seq: &S) -> Option<&Self::Output> {
                let r = IndexRangeBounds::<X>::canonicalize(self, seq.len());
                S::get_range(seq, r)
            }

            fn get_mut(self, seq: &mut S) -> Option<&mut Self::Output>
            where
                S: SequenceMut,
            {
                let r = IndexRangeBounds::<X>::canonicalize(self, seq.len());
                S::get_range_mut(seq, r)
            }

            fn index(self, seq: &S) -> &Self::Output {
                let r = IndexRangeBounds::<X>::canonicalize(self, seq.len());
                S::index_range(seq, r)
            }


            fn index_mut(self, seq: &mut S) -> &mut Self::Output
            where
                S: SequenceMut,
            {
                let r = IndexRangeBounds::<X>::canonicalize(self, seq.len());
                S::index_range_mut(seq, r)
            }

            unsafe fn get_unchecked<FS, FR>(
                self,
                seq: *const S,
            ) -> *const Self::Output
            where
                S: UnsafeSequence
            {
                unsafe {
                    let r = IndexRangeBounds::<X>::canonicalize(self, S::len_from_ptr(seq));
                    S::get_range_unchecked(seq, r)
                }
            }

            unsafe fn get_unchecked_mut(self, seq: *mut S) -> *mut Self::Output
            where
                S: UnsafeSequenceMut,
            {
                unsafe {
                    let r = IndexRangeBounds::<X>::canonicalize(self, S::len_from_ptr(seq));
                    S::get_range_unchecked_mut(seq, r)
                }
            }

        }
    )*};
}

index_slice_partial_range_impl![
    core::ops::RangeInclusive<X>,
    core::ops::RangeFrom<X>,
    core::ops::RangeTo<X>,
    core::ops::RangeToInclusive<X>,
    IndexRangeInclusive<X>,
    IndexRangeFrom<X>,
    IndexRange<X>
];
