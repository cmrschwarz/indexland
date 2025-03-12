use core::ops::{Index, IndexMut, Range};

use crate::{
    index_range::{IndexRangeBounds, IndexRangeFrom, IndexRangeInclusive},
    Idx, IndexRange, IndexSlice,
};

/// # Safety
/// if the input of `get_unchecked(_mut)` was a valid pointer, so must be the output
pub unsafe trait IndexSliceIndex<S: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &S) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut S) -> Option<&mut Self::Output>;
    /// # Safety
    /// `slice` must be a valid pointer for the expected range
    unsafe fn get_unchecked(self, slice: *const S) -> *const Self::Output;
    /// # Safety
    /// `slice` must be a valid pointer for the expected range
    unsafe fn get_unchecked_mut(self, slice: *mut S) -> *mut Self::Output;
    fn index(self, slice: &S) -> &Self::Output;
    fn index_mut(self, slice: &mut S) -> &mut Self::Output;
}

unsafe impl<I: Idx, T> IndexSliceIndex<IndexSlice<I, T>> for I {
    type Output = T;
    #[inline]
    fn get(self, slice: &IndexSlice<I, T>) -> Option<&Self::Output> {
        slice.data.get(self.into_usize())
    }
    #[inline]
    fn get_mut(
        self,
        slice: &mut IndexSlice<I, T>,
    ) -> Option<&mut Self::Output> {
        slice.data.get_mut(self.into_usize())
    }
    #[inline]
    unsafe fn get_unchecked(
        self,
        slice: *const IndexSlice<I, T>,
    ) -> *const Self::Output {
        unsafe { slice.cast::<T>().add(self.into_usize()) }
    }
    #[inline]
    unsafe fn get_unchecked_mut(
        self,
        slice: *mut IndexSlice<I, T>,
    ) -> *mut Self::Output {
        unsafe { slice.cast::<T>().add(self.into_usize()) }
    }
    #[inline]
    fn index(self, slice: &IndexSlice<I, T>) -> &Self::Output {
        slice.data.index(self.into_usize())
    }
    #[inline]
    fn index_mut(self, slice: &mut IndexSlice<I, T>) -> &mut Self::Output {
        slice.data.index_mut(self.into_usize())
    }
}

unsafe impl<I: Idx, T> IndexSliceIndex<IndexSlice<I, T>>
    for core::ops::Range<I>
{
    type Output = IndexSlice<I, T>;

    #[inline]
    fn get(self, slice: &IndexSlice<I, T>) -> Option<&IndexSlice<I, T>> {
        Some(IndexSlice::from_slice(
            slice.as_slice().get(self.usize_range())?,
        ))
    }

    #[inline]
    fn get_mut(
        self,
        slice: &mut IndexSlice<I, T>,
    ) -> Option<&mut IndexSlice<I, T>> {
        Some(IndexSlice::from_slice_mut(
            slice.as_slice_mut().get_mut(self.usize_range())?,
        ))
    }

    #[inline]
    unsafe fn get_unchecked(
        self,
        slice: *const IndexSlice<I, T>,
    ) -> *const IndexSlice<I, T> {
        let slice = slice as *const [T];
        let start = self.start.into_usize();
        let end = self.start.into_usize();

        unsafe {
            core::ptr::slice_from_raw_parts(
                slice.cast::<T>().add(start),
                end - start,
            ) as _
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(
        self,
        slice: *mut IndexSlice<I, T>,
    ) -> *mut IndexSlice<I, T> {
        let slice = slice as *mut [T];
        let start = self.start.into_usize();
        let end = self.start.into_usize();

        unsafe {
            core::ptr::slice_from_raw_parts(
                slice.cast::<T>().add(start),
                end - start,
            ) as _
        }
    }

    #[inline]
    fn index(self, slice: &IndexSlice<I, T>) -> &IndexSlice<I, T> {
        IndexSlice::from_slice(&slice.as_slice()[self.usize_range()])
    }

    #[inline]
    fn index_mut(self, slice: &mut IndexSlice<I, T>) -> &mut IndexSlice<I, T> {
        IndexSlice::from_slice_mut(
            &mut slice.as_slice_mut()[self.usize_range()],
        )
    }
}

// NOTE: We could theoretically return a &[T] here instead for cases where
// the offset does not start from zero to prevent the user from accidentally
// messing up their index space. This is probably more confusing and
// inconsistent than useful though. Probably.
macro_rules! index_slice_partial_range_impl {
    ($($range: path),*) => {$(
        unsafe impl<I: Idx, T> IndexSliceIndex<IndexSlice<I, T>> for $range {
            type Output = IndexSlice<I, T>;
            #[inline]
            fn get(self, slice: &IndexSlice<I, T>) -> Option<&IndexSlice<I, T>> {
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                Some(IndexSlice::from_slice(
                    slice.as_slice().get(range)?,
                ))
            }
            #[inline]
            fn get_mut(
                self,
                slice: &mut IndexSlice<I, T>,
            ) -> Option<&mut IndexSlice<I, T>> {
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                Some(IndexSlice::from_slice_mut(
                    slice.as_slice_mut().get_mut(range)?,
                ))
            }
            #[inline]
            unsafe fn get_unchecked(
                self,
                slice: *const IndexSlice<I, T>,
            ) -> *const IndexSlice<I, T> {
                let slice = slice as *mut [T];
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                unsafe {
                    core::ptr::slice_from_raw_parts(
                        slice.cast::<T>().add(range.start),
                        range.end - range.start,
                    ) as _
                }
            }
            #[inline]
            unsafe fn get_unchecked_mut(
                self,
                slice: *mut IndexSlice<I, T>,
            ) -> *mut IndexSlice<I, T> {
                let slice = slice as *mut [T];
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                unsafe {
                    core::ptr::slice_from_raw_parts_mut(
                        slice.cast::<T>().add(range.start),
                        range.end - range.start,
                    ) as _
                }
            }
            #[inline(always)]
            fn index(self, slice: &IndexSlice<I, T>) -> &IndexSlice<I, T> {
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                IndexSlice::from_slice(&slice.as_slice()[range])
            }
            #[inline]
            fn index_mut(self, slice: &mut IndexSlice<I, T>) -> &mut IndexSlice<I, T> {
                let range = IndexRangeBounds::<I>::canonicalize(self, slice.len());
                IndexSlice::from_slice_mut(&mut slice.as_slice_mut()[range])
            }
        }
    )*};
}

index_slice_partial_range_impl![
    core::ops::RangeInclusive<I>,
    core::ops::RangeFrom<I>,
    core::ops::RangeTo<I>,
    core::ops::RangeToInclusive<I>,
    core::ops::RangeFull,
    IndexRangeInclusive<I>,
    IndexRangeFrom<I>
];

unsafe impl<I: Idx, T> IndexSliceIndex<IndexSlice<I, T>> for IndexRange<I> {
    type Output = IndexSlice<I, T>;
    #[inline]
    fn get(self, slice: &IndexSlice<I, T>) -> Option<&IndexSlice<I, T>> {
        Range::from(self).get(slice)
    }
    #[inline]
    fn get_mut(
        self,
        slice: &mut IndexSlice<I, T>,
    ) -> Option<&mut IndexSlice<I, T>> {
        Range::from(self).get_mut(slice)
    }
    #[inline]
    unsafe fn get_unchecked(
        self,
        slice: *const IndexSlice<I, T>,
    ) -> *const IndexSlice<I, T> {
        unsafe { Range::from(self).get_unchecked(slice) }
    }
    #[inline]
    unsafe fn get_unchecked_mut(
        self,
        slice: *mut IndexSlice<I, T>,
    ) -> *mut IndexSlice<I, T> {
        unsafe { Range::from(self).get_unchecked_mut(slice) }
    }
    #[inline]
    fn index(self, slice: &IndexSlice<I, T>) -> &IndexSlice<I, T> {
        Range::from(self).index(slice)
    }
    #[inline]
    fn index_mut(self, slice: &mut IndexSlice<I, T>) -> &mut IndexSlice<I, T> {
        Range::from(self).index_mut(slice)
    }
}
