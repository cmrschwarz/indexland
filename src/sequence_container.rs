use core::ops::{Range, RangeFull};

use crate::{
    idx::IdxCompat, Idx, IndexRange, IndexRangeBounds, IndexRangeFrom, IndexRangeInclusive,
};

/// ## Safety
/// `get_unchecked` and `get_range_unchecked` are trusted to return valid pointers
/// into the container if they received valid input
#[allow(clippy::len_without_is_empty)]
pub unsafe trait SequenceContainer {
    type Element: ?Sized;
    type Slice: ?Sized;

    /// ## Safety
    /// `this` must be a valid container pointer
    ///
    /// ## Tree Borrows
    /// For now, the caller must assume that this may turn the provided
    /// pointer into a `&Self` temporarily to perform this operation.
    unsafe fn len_from_ptr(this: *const Self) -> usize;

    fn len(&self) -> usize {
        unsafe { Self::len_from_ptr(core::ptr::from_ref(self)) }
    }

    fn get(&self, idx: usize) -> Option<&Self::Element>;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_unchecked(this: *const Self, idx: usize) -> *const Self::Element;

    fn index(&self, idx: usize) -> &Self::Element;

    fn get_range(&self, r: Range<usize>) -> Option<&Self::Slice>;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_range_unchecked(this: *const Self, r: Range<usize>) -> *const Self::Slice;

    fn index_range(&self, r: Range<usize>) -> &Self::Slice;
}

/// ## Safety
/// `get_unchecked_mut` and `get_range_unchecked_mut` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait SequenceContainerMut: SequenceContainer {
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Element>;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_unchecked_mut(this: *mut Self, idx: usize) -> *mut Self::Element;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Element;

    fn get_range_mut(&mut self, r: Range<usize>) -> Option<&mut Self::Slice>;

    /// ## Safety
    /// `this` must be a valid container pointer
    unsafe fn get_range_unchecked_mut(this: *mut Self, r: Range<usize>) -> *mut Self::Slice;

    fn index_range_mut(&mut self, r: Range<usize>) -> &mut Self::Slice;
}

unsafe impl<T> SequenceContainer for [T] {
    type Element = T;

    type Slice = [T];

    #[inline(always)]
    unsafe fn len_from_ptr(this: *const Self) -> usize {
        this.len()
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> Option<&Self::Element> {
        <[T]>::get(self, idx)
    }

    #[inline(always)]
    unsafe fn get_unchecked(this: *const Self, idx: usize) -> *const Self::Element {
        this.cast::<T>().add(idx)
    }

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Element {
        core::ops::Index::index(self, idx)
    }

    #[inline(always)]
    fn get_range(&self, r: Range<usize>) -> Option<&Self::Slice> {
        <[T]>::get(self, r)
    }

    #[inline(always)]
    unsafe fn get_range_unchecked(this: *const Self, r: Range<usize>) -> *const Self::Slice {
        unsafe {
            core::ptr::slice_from_raw_parts(this.cast::<T>().add(r.start), r.end - r.start) as _
        }
    }

    #[inline(always)]
    fn index_range(&self, r: Range<usize>) -> &Self::Slice {
        core::ops::Index::index(self, r)
    }
}

unsafe impl<T> SequenceContainerMut for [T] {
    #[inline(always)]
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Element> {
        <[T]>::get_mut(self, idx)
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(this: *mut Self, idx: usize) -> *mut Self::Element {
        unsafe { this.cast::<T>().add(idx) }
    }

    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Element {
        core::ops::IndexMut::index_mut(self, idx)
    }

    #[inline(always)]
    fn get_range_mut(&mut self, r: Range<usize>) -> Option<&mut Self::Slice> {
        <[T]>::get_mut(self, r)
    }

    #[inline(always)]
    unsafe fn get_range_unchecked_mut(this: *mut Self, r: Range<usize>) -> *mut Self::Slice {
        unsafe {
            core::ptr::slice_from_raw_parts_mut(this.cast::<T>().add(r.start), r.end - r.start) as _
        }
    }

    #[inline(always)]
    fn index_range_mut(&mut self, r: Range<usize>) -> &mut Self::Slice {
        core::ops::IndexMut::index_mut(self, r)
    }
}

/// ## Safety
/// `get_unchecked` and `get_unchecked_mut` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait SequenceContainerIndex<I, C: ?Sized>: Sized {
    type Output: ?Sized;
    fn get(self, container: &C) -> Option<&Self::Output>
    where
        C: SequenceContainer;

    /// ## Safety
    /// the container pointer must be valid
    unsafe fn get_unchecked<FS, FR>(self, container: *const C) -> *const Self::Output
    where
        C: SequenceContainer;

    fn index(self, container: &C) -> &Self::Output
    where
        C: SequenceContainer;

    fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
    where
        C: SequenceContainerMut;

    /// ## Safety
    /// the container pointer must be valid
    unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
    where
        C: SequenceContainerMut;

    fn index_mut(self, container: &mut C) -> &mut Self::Output
    where
        C: SequenceContainerMut;
}

unsafe impl<I: Idx, C: SequenceContainer + ?Sized> SequenceContainerIndex<I, C> for I {
    type Output = C::Element;

    fn get(self, container: &C) -> Option<&Self::Output> {
        C::get(container, self.into_usize())
    }

    unsafe fn get_unchecked<FS, FR>(self, container: *const C) -> *const Self::Output {
        C::get_unchecked(container, self.into_usize())
    }

    fn index(self, container: &C) -> &Self::Output {
        C::index(container, self.into_usize())
    }

    fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
    where
        C: SequenceContainerMut,
    {
        C::get_mut(container, self.into_usize())
    }

    unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::get_unchecked_mut(container, self.into_usize())
    }

    fn index_mut(self, container: &mut C) -> &mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::index_mut(container, self.into_usize())
    }
}

unsafe impl<I: Idx, C: SequenceContainer + ?Sized, X: IdxCompat<I>> SequenceContainerIndex<I, C>
    for Range<X>
{
    type Output = C::Slice;

    fn get(self, container: &C) -> Option<&Self::Output> {
        C::get_range(container, self.usize_range())
    }

    unsafe fn get_unchecked<FS, FR>(self, container: *const C) -> *const Self::Output {
        C::get_range_unchecked(container, self.usize_range())
    }

    fn index(self, container: &C) -> &Self::Output {
        C::index_range(container, self.usize_range())
    }

    fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
    where
        C: SequenceContainerMut,
    {
        C::get_range_mut(container, self.usize_range())
    }

    unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::get_range_unchecked_mut(container, self.usize_range())
    }

    fn index_mut(self, container: &mut C) -> &mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::index_range_mut(container, self.usize_range())
    }
}

unsafe impl<I: Idx, C: SequenceContainer + ?Sized> SequenceContainerIndex<I, C> for RangeFull {
    type Output = C::Slice;

    fn get(self, container: &C) -> Option<&Self::Output> {
        C::get_range(container, 0..container.len())
    }

    unsafe fn get_unchecked<FS, FR>(self, container: *const C) -> *const Self::Output {
        C::get_range_unchecked(container, 0..C::len_from_ptr(container))
    }

    fn index(self, container: &C) -> &Self::Output {
        C::index_range(container, 0..container.len())
    }

    fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
    where
        C: SequenceContainerMut,
    {
        C::get_range_mut(container, 0..container.len())
    }

    unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::get_range_unchecked_mut(container, 0..C::len_from_ptr(container))
    }

    fn index_mut(self, container: &mut C) -> &mut Self::Output
    where
        C: SequenceContainerMut,
    {
        C::index_range_mut(container, 0..container.len())
    }
}

macro_rules! index_slice_partial_range_impl {
    ($($range: path),*) => {$(
        unsafe impl<
            I: Idx,
            C: SequenceContainer + ?Sized,
            X: IdxCompat<I>
        >
            SequenceContainerIndex<I, C> for $range
        {
            type Output = C::Slice;

            fn get(self, container: &C) -> Option<&Self::Output> {
                let r = IndexRangeBounds::<X>::canonicalize(self, container.len());
                C::get_range(container, r)
            }

            unsafe fn get_unchecked<FS, FR>(
                self,
                container: *const C,
            ) -> *const Self::Output {
                let r = IndexRangeBounds::<X>::canonicalize(self, C::len_from_ptr(container));
                C::get_range_unchecked(container, r)
            }

            fn index(self, container: &C) -> &Self::Output {
                let r = IndexRangeBounds::<X>::canonicalize(self, container.len());
                C::index_range(container, r)
            }

            fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
            where
                C: SequenceContainerMut,
            {
                let r = IndexRangeBounds::<X>::canonicalize(self, container.len());
                C::get_range_mut(container, r)
            }

            unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
            where
                C: SequenceContainerMut,
            {
                let r = IndexRangeBounds::<X>::canonicalize(self, C::len_from_ptr(container));
                C::get_range_unchecked_mut(container, r)
            }

            fn index_mut(self, container: &mut C) -> &mut Self::Output
            where
                C: SequenceContainerMut,
            {
                let r = IndexRangeBounds::<X>::canonicalize(self, container.len());
                C::index_range_mut(container, r)
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
