use core::ops::{Range, RangeFull};

use crate::{
    idx::IdxCompatible, Idx, IndexRange, IndexRangeBounds, IndexRangeFrom,
    IndexRangeInclusive,
};

/// # Safety
/// `get_unchecked` and `get_unchecked_mut` are trusted to return valid pointers
/// into the container if they received valid input
pub unsafe trait GenericIndex<I, S: ?Sized, R: ?Sized, C: ?Sized>:
    Sized
{
    type Output: ?Sized;
    fn get<FS, FR>(
        self,
        container: &C,
        len: usize,
        single: FS,
        range: FR,
    ) -> Option<&Self::Output>
    where
        FS: Fn(&C, usize) -> Option<&S>,
        FR: Fn(&C, Range<usize>) -> Option<&R>;

    fn get_mut<FS, FR>(
        self,
        container: &mut C,
        len: usize,
        single: FS,
        range: FR,
    ) -> Option<&mut Self::Output>
    where
        FS: Fn(&mut C, usize) -> Option<&mut S>,
        FR: Fn(&mut C, Range<usize>) -> Option<&mut R>;

    /// # Safety
    /// must call either single or range exactly once and return the output
    unsafe fn get_unchecked<FS, FR>(
        self,
        container: *const C,
        len: usize,
        single: FS,
        range: FR,
    ) -> *const Self::Output
    where
        FS: Fn(*const C, usize) -> *const S,
        FR: Fn(*const C, Range<usize>) -> *const R;

    /// # Safety
    /// must call either single or range exactly once and return the output
    unsafe fn get_unchecked_mut<FS, FR>(
        self,
        container: *mut C,
        len: usize,
        single: FS,
        range: FR,
    ) -> *mut Self::Output
    where
        FS: Fn(*mut C, usize) -> *mut S,
        FR: Fn(*mut C, Range<usize>) -> *mut R;

    fn index<FS, FR>(
        self,
        container: &C,
        len: usize,
        single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&C, usize) -> &S,
        FR: Fn(&C, Range<usize>) -> &R;

    fn index_mut<FS, FR>(
        self,
        container: &mut C,
        len: usize,
        single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&mut C, usize) -> &mut S,
        FR: Fn(&mut C, Range<usize>) -> &mut R;
}

unsafe impl<I: Idx, S: ?Sized, R: ?Sized, C: ?Sized> GenericIndex<I, S, R, C>
    for I
{
    type Output = S;

    #[inline]
    fn get<FS, FR>(
        self,
        container: &C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> Option<&Self::Output>
    where
        FS: Fn(&C, usize) -> Option<&S>,
        FR: Fn(&C, Range<usize>) -> Option<&R>,
    {
        single(container, self.into_usize())
    }

    #[inline]
    fn get_mut<FS, FR>(
        self,
        container: &mut C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> Option<&mut Self::Output>
    where
        FS: Fn(&mut C, usize) -> Option<&mut S>,
        FR: Fn(&mut C, Range<usize>) -> Option<&mut R>,
    {
        single(container, self.into_usize())
    }

    #[inline]
    unsafe fn get_unchecked<FS, FR>(
        self,
        container: *const C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> *const Self::Output
    where
        FS: Fn(*const C, usize) -> *const S,
        FR: Fn(*const C, Range<usize>) -> *const R,
    {
        single(container, self.into_usize())
    }

    #[inline]
    unsafe fn get_unchecked_mut<FS, FR>(
        self,
        container: *mut C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> *mut Self::Output
    where
        FS: Fn(*mut C, usize) -> *mut S,
        FR: Fn(*mut C, Range<usize>) -> *mut R,
    {
        single(container, self.into_usize())
    }

    #[inline]
    fn index<FS, FR>(
        self,
        container: &C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&C, usize) -> &S,
        FR: Fn(&C, Range<usize>) -> &R,
    {
        single(container, self.into_usize())
    }

    #[inline]
    fn index_mut<FS, FR>(
        self,
        container: &mut C,
        _len: usize,
        single: FS,
        _range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&mut C, usize) -> &mut S,
        FR: Fn(&mut C, Range<usize>) -> &mut R,
    {
        single(container, self.into_usize())
    }
}

unsafe impl<I: Idx, S: ?Sized, R: ?Sized, C: ?Sized> GenericIndex<I, S, R, C>
    for Range<I>
{
    type Output = R;

    #[inline]
    fn get<FS, FR>(
        self,
        container: &C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> Option<&Self::Output>
    where
        FS: Fn(&C, usize) -> Option<&S>,
        FR: Fn(&C, Range<usize>) -> Option<&R>,
    {
        range(container, self.usize_range())
    }

    #[inline]
    fn get_mut<FS, FR>(
        self,
        container: &mut C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> Option<&mut Self::Output>
    where
        FS: Fn(&mut C, usize) -> Option<&mut S>,
        FR: Fn(&mut C, Range<usize>) -> Option<&mut R>,
    {
        range(container, self.usize_range())
    }

    #[inline]
    unsafe fn get_unchecked<FS, FR>(
        self,
        container: *const C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> *const Self::Output
    where
        FS: Fn(*const C, usize) -> *const S,
        FR: Fn(*const C, Range<usize>) -> *const R,
    {
        range(container, self.usize_range())
    }

    #[inline]
    unsafe fn get_unchecked_mut<FS, FR>(
        self,
        container: *mut C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> *mut Self::Output
    where
        FS: Fn(*mut C, usize) -> *mut S,
        FR: Fn(*mut C, Range<usize>) -> *mut R,
    {
        range(container, self.usize_range())
    }

    #[inline]
    fn index<FS, FR>(
        self,
        container: &C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&C, usize) -> &S,
        FR: Fn(&C, Range<usize>) -> &R,
    {
        range(container, self.usize_range())
    }

    #[inline]
    fn index_mut<FS, FR>(
        self,
        container: &mut C,
        _len: usize,
        _single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&mut C, usize) -> &mut S,
        FR: Fn(&mut C, Range<usize>) -> &mut R,
    {
        range(container, self.usize_range())
    }
}

unsafe impl<I: Idx, S: ?Sized, R: ?Sized, C: ?Sized> GenericIndex<I, S, R, C>
    for RangeFull
{
    type Output = R;

    #[inline]
    fn get<FS, FR>(
        self,
        container: &C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> Option<&Self::Output>
    where
        FS: Fn(&C, usize) -> Option<&S>,
        FR: Fn(&C, Range<usize>) -> Option<&R>,
    {
        range(container, 0..len)
    }

    #[inline]
    fn get_mut<FS, FR>(
        self,
        container: &mut C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> Option<&mut Self::Output>
    where
        FS: Fn(&mut C, usize) -> Option<&mut S>,
        FR: Fn(&mut C, Range<usize>) -> Option<&mut R>,
    {
        range(container, 0..len)
    }

    #[inline]
    unsafe fn get_unchecked<FS, FR>(
        self,
        container: *const C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> *const Self::Output
    where
        FS: Fn(*const C, usize) -> *const S,
        FR: Fn(*const C, Range<usize>) -> *const R,
    {
        range(container, 0..len)
    }

    #[inline]
    unsafe fn get_unchecked_mut<FS, FR>(
        self,
        container: *mut C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> *mut Self::Output
    where
        FS: Fn(*mut C, usize) -> *mut S,
        FR: Fn(*mut C, Range<usize>) -> *mut R,
    {
        range(container, 0..len)
    }

    #[inline]
    fn index<FS, FR>(
        self,
        container: &C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&C, usize) -> &S,
        FR: Fn(&C, Range<usize>) -> &R,
    {
        range(container, 0..len)
    }

    #[inline]
    fn index_mut<FS, FR>(
        self,
        container: &mut C,
        len: usize,
        _single: FS,
        range: FR,
    ) -> &Self::Output
    where
        FS: Fn(&mut C, usize) -> &mut S,
        FR: Fn(&mut C, Range<usize>) -> &mut R,
    {
        range(container, 0..len)
    }
}

macro_rules! index_slice_partial_range_impl {
    ($($range: path),*) => {$(
        unsafe impl<
            I: Idx,
            S: ?Sized,
            R: ?Sized,
            C: ?Sized,
            X: IdxCompatible<I>
        >
            GenericIndex<I, S, R, C> for $range
        {
            type Output = R;

            #[inline]
            fn get<FS, FR>(
                self,
                container: &C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> Option<&Self::Output>
            where
                FS: Fn(&C, usize) -> Option<&S>,
                FR: Fn(&C, Range<usize>) -> Option<&R>,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
            }

            #[inline]
            fn get_mut<FS, FR>(
                self,
                container: &mut C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> Option<&mut Self::Output>
            where
                FS: Fn(&mut C, usize) -> Option<&mut S>,
                FR: Fn(&mut C, Range<usize>) -> Option<&mut R>,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
            }

            #[inline]
            unsafe fn get_unchecked<FS, FR>(
                self,
                container: *const C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> *const Self::Output
            where
                FS: Fn(*const C, usize) -> *const S,
                FR: Fn(*const C, Range<usize>) -> *const R,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
            }

            #[inline]
            unsafe fn get_unchecked_mut<FS, FR>(
                self,
                container: *mut C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> *mut Self::Output
            where
                FS: Fn(*mut C, usize) -> *mut S,
                FR: Fn(*mut C, Range<usize>) -> *mut R,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
            }

            #[inline]
            fn index<FS, FR>(
                self,
                container: &C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> &Self::Output
            where
                FS: Fn(&C, usize) -> &S,
                FR: Fn(&C, Range<usize>) -> &R,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
            }

            #[inline]
            fn index_mut<FS, FR>(
                self,
                container: &mut C,
                len: usize,
                _single: FS,
                range: FR,
            ) -> &Self::Output
            where
                FS: Fn(&mut C, usize) -> &mut S,
                FR: Fn(&mut C, Range<usize>) -> &mut R,
            {
                let r = IndexRangeBounds::<I, X>::canonicalize(self, len);
                range(container, r)
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
