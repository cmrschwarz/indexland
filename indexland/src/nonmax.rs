//! Integers with a niche value based on [`NonZero`](core::num::NonZero), allowing for better
//! enum layout optimizations.
//!
//! Similar to the [`nonmax`](https://docs.rs/nonmax/latest/nonmax/) crate,
//! but with a few key differences:
//!  - [`NonMax<u8>`] instead of `NonMaxU8`
//!  - Implements arithmetic operations (required for [`Idx`])
//!  - Makes using debuggers less painful by removing the optimization in debug
//!    mode.
//!
//!    (This can be disabled using the `"disable_debuggable_nonmax"` feature).
//!
//! ## Implementations
//! - [`NonMax<u8>`]
//! - [`NonMax<u16>`]
//! - [`NonMax<u32>`]
//! - [`NonMax<u64>`]
//! - [`NonMax<usize>`]
//!
//! - [`NonMax<i8>`]
//! - [`NonMax<i16>`]
//! - [`NonMax<i32>`]
//! - [`NonMax<i64>`]
//! - [`NonMax<isize>`]

use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

use core::convert::TryFrom;

#[cfg(any(not(debug_assertions), feature = "disable_debuggable_nonmax"))]
use core::num::NonZero;

use crate::Idx;

/// Generic [`NonMax`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonMax<P: NonMaxPrimitive>(P::NonMaxInner);

#[derive(Debug, Default, Clone, Copy)]
pub struct NonMaxOutOfRangeError;

#[cfg(feature = "std")]
impl std::error::Error for NonMaxOutOfRangeError {}

pub trait NonMaxPrimitive:
    Debug
    + Display
    + Clone
    + Copy
    + Sized
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
    type NonMaxInner: NonMaxInner<Self>;
}

pub trait NonMaxInner<P>:
    Sized + Copy + PartialEq + Eq + PartialOrd + Ord + Hash
{
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;

    fn new(v: P) -> Option<Self>;

    /// # Safety
    /// value must not me `P::MAX`
    unsafe fn new_unchecked(value: P) -> Self;

    fn get(self) -> P;

    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

impl<P: NonMaxPrimitive> NonMax<P> {
    pub const ZERO: NonMax<P> = NonMax(P::NonMaxInner::ZERO);
    pub const ONE: NonMax<P> = NonMax(P::NonMaxInner::ONE);
    pub const MIN: NonMax<P> = NonMax(P::NonMaxInner::MIN);
    pub const MAX: NonMax<P> = NonMax(P::NonMaxInner::MAX);

    pub fn wrapping_add(self, rhs: Self) -> Self {
        NonMax(self.0.wrapping_add(rhs.0))
    }
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        NonMax(self.0.wrapping_sub(rhs.0))
    }
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        NonMax(self.0.wrapping_mul(rhs.0))
    }
    pub fn get(self) -> P {
        self.0.get()
    }
}

impl<P: NonMaxPrimitive> Default for NonMax<P> {
    fn default() -> Self {
        Self(P::NonMaxInner::ZERO)
    }
}

impl<P: NonMaxPrimitive> Debug for NonMax<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0.get(), f)
    }
}
impl<P: NonMaxPrimitive> Display for NonMax<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0.get(), f)
    }
}

impl<P: NonMaxPrimitive> Add for NonMax<P> {
    type Output = NonMax<P>;

    fn add(self, rhs: Self) -> Self::Output {
        NonMax(NonMaxInner::new(self.0.get() + rhs.0.get()).unwrap())
    }
}
impl<P: NonMaxPrimitive> Sub for NonMax<P> {
    type Output = NonMax<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        NonMax(NonMaxInner::new(self.0.get() - rhs.0.get()).unwrap())
    }
}
impl<P: NonMaxPrimitive> Mul for NonMax<P> {
    type Output = NonMax<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        NonMax(NonMaxInner::new(self.0.get() * rhs.0.get()).unwrap())
    }
}
impl<P: NonMaxPrimitive> Div for NonMax<P> {
    type Output = NonMax<P>;

    fn div(self, rhs: Self) -> Self::Output {
        NonMax(NonMaxInner::new(self.0.get() / rhs.0.get()).unwrap())
    }
}
impl<P: NonMaxPrimitive> Rem for NonMax<P> {
    type Output = NonMax<P>;

    fn rem(self, rhs: Self) -> Self::Output {
        NonMax(NonMaxInner::new(self.0.get() % rhs.0.get()).unwrap())
    }
}

impl<P: NonMaxPrimitive> AddAssign for NonMax<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Add::add(*self, rhs);
    }
}
impl<P: NonMaxPrimitive> SubAssign for NonMax<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Sub::sub(*self, rhs);
    }
}
impl<P: NonMaxPrimitive> MulAssign for NonMax<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Mul::mul(*self, rhs);
    }
}
impl<P: NonMaxPrimitive> DivAssign for NonMax<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = Div::div(*self, rhs);
    }
}
impl<P: NonMaxPrimitive> RemAssign for NonMax<P> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = Rem::rem(*self, rhs);
    }
}

macro_rules! impl_nonmax {
    ($($primitive: ty),*) => {$(
        impl NonMax<$primitive> {
            const fn new(v: $primitive) -> Option<Self> {
                if v == $primitive::MAX {
                    return None;
                }
                Some(unsafe{Self::new_unchecked(v)})
            }
            /// # Safety
            #[doc = concat!("Must not be [`", stringify!($primitive), "::MAX`].")]
            pub const unsafe fn new_unchecked(v: $primitive) -> Self {
                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                return NonMax(v);

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                NonMax(unsafe { NonZero::new_unchecked(v ^ $primitive::MAX) })
            }
        }
        impl NonMaxPrimitive for $primitive {
            #[cfg(all(
                debug_assertions,
                not(feature = "disable_debuggable_nonmax")
            ))]
            type NonMaxInner = $primitive;

            #[cfg(any(
                not(debug_assertions),
                feature = "disable_debuggable_nonmax"
            ))]
            type NonMaxInner = NonZero<$primitive>;
        }
        impl NonMaxInner<$primitive> for <$primitive as NonMaxPrimitive>::NonMaxInner {
            const ZERO: Self = NonMax::<$primitive>::new(0).unwrap().0;
            const ONE: Self = NonMax::<$primitive>::new(1).unwrap().0;
            const MIN: Self = NonMax::<$primitive>::new($primitive::MIN).unwrap().0;
            const MAX: Self = NonMax::<$primitive>::new($primitive::MAX - 1).unwrap().0;

            fn new(v: $primitive) -> Option<Self> {
                if v == $primitive::MAX {
                    return None;
                }
                Some(unsafe{Self::new_unchecked(v)})
            }
            unsafe fn new_unchecked(v: $primitive) -> Self {
                unsafe { NonMax::<$primitive>::new_unchecked(v) }.0
            }
            #[inline(always)]
            fn get(self) -> $primitive {
                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                return self;

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                self.get()
            }
            // we could implement these wrapping functions wihtout unsafe
            // but that would make them even more expensive in debug mode
            fn wrapping_add(self, rhs: Self) -> Self {

                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                let mut res = $primitive::wrapping_add(self, rhs);

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let mut res = self.get().wrapping_add(rhs.get());

                if res == $primitive::MAX {
                    res = 0;
                }
                unsafe { Self::new_unchecked(res) }
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                let mut res = $primitive::wrapping_sub(self, rhs);

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let mut res = self.get().wrapping_sub(rhs.get());

                if res == $primitive::MAX {
                    res = 0;
                }
                unsafe { Self::new_unchecked(res) }
            }
            fn wrapping_mul(self, rhs: Self) -> Self {
                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                let mut res = $primitive::wrapping_mul(self, rhs);

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let mut res = self.get().wrapping_mul(rhs.get());

                if res == $primitive::MAX {
                    res = 0;
                }
                unsafe { Self::new_unchecked(res) }
            }
        }
        impl From<NonMax<$primitive>> for $primitive {
            fn from(v: NonMax<$primitive>) -> $primitive {
                v.get()
            }
        }
        impl TryFrom<$primitive> for NonMax<$primitive> {
            type Error = NonMaxOutOfRangeError;
            fn try_from(v: $primitive) -> Result<NonMax<$primitive>, NonMaxOutOfRangeError> {
                NonMax::<$primitive>::new(v).ok_or(NonMaxOutOfRangeError)
            }
        }
    )*};
}

macro_rules! impl_nonmax_idx {
    ($($primitive: ty),*) => {$(
        impl Idx for NonMax<$primitive> {
            const ZERO: Self = NonMax::<$primitive>::ZERO;
            const ONE: Self = NonMax::<$primitive>::ONE;
            const MAX: Self = NonMax::<$primitive>::MAX;
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                NonMax::<$primitive>::try_from(v).unwrap()
            }
            #[inline(always)]
            fn into_usize(self) -> usize {
                usize::try_from(self.get()).unwrap()
            }
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                //TODO: wrap instead
                #![allow(clippy::cast_possible_truncation)]
                NonMax::<$primitive>::new(v as $primitive).unwrap()
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                //TODO: wrap instead
                #![allow(clippy::cast_possible_truncation)]
                self.get() as usize
            }
        }
    )*};
}

impl_nonmax![u8, u16, u32, u64, u128, usize];
impl_nonmax![i8, i16, i32, i64, i128, isize];

impl_nonmax_idx![u8, u16, u32, u64, u128, usize];
impl_nonmax_idx![i8, i16, i32, i64, u128, isize];

// unchecked A => NonMax<B> & NonMax<A> => NonMax<B>
macro_rules! impl_from_unchecked {
    ( $source: ty => $($target: ty),* ) => {
        impl From<$source> for NonMax<$target> {
            #[inline]
            fn from(src: $source) -> Self {
                // SAFETY: smaller input type guarantees the value is non-max
                unsafe { Self::new_unchecked(src.into()) }
            }
        }
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;
            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                Ok(Self::from(src))
            }
        }
        impl From<NonMax<$source>> for NonMax<$target> {
            #[inline]
            fn from(src: NonMax<$source>) -> Self {
                // SAFETY: smaller input type guarantees the value is non-max
                unsafe { Self::new_unchecked(src.get().into()) }
            }
        }
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;
            #[inline]
            fn try_from(src: NonMax<$source>) -> Result<Self, Self::Error> {
                Ok(Self::from(src))
            }
        }
    };
}

// Unsigned => Larger Unsigned
impl_from_unchecked![u8 => u16, u32, u64, u128, usize];
impl_from_unchecked![u16 => u32, u64, u128, usize];
impl_from_unchecked![u32 => u64, u128];
impl_from_unchecked![u64 => u128];

// Signed => Larger Signed
impl_from_unchecked![i8 => i16, i32, i64, i128, isize];
impl_from_unchecked![i16 => i32, i64, i128, isize];
impl_from_unchecked![i32 => i64, i128];
impl_from_unchecked![i64 => i128];

// Unsigned => Larger Signed
impl_from_unchecked![u8 => i16, i32, i64, i128, isize];
impl_from_unchecked![u16 => i32, i64, i128, isize];
impl_from_unchecked![u32 => i64, i128];
impl_from_unchecked![u64 => i128];

// A => NonMax<B> & NonMax<A> => NonMax<B>  if A >= 0
macro_rules! impl_try_from_check_gte_0 {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;
            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                if src >= 0 {
                    Ok(unsafe { Self::new_unchecked(src as $target) })
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
        impl TryFrom<NonMax<$source>> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;
            #[inline]
            fn try_from(src: NonMax<$source>) -> Result<Self, Self::Error> {
                let src = src.get();
                if src >= 0 {
                    Ok(unsafe { Self::new_unchecked(src as $target) })
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
    )*}
}

// Signed => Larger Unsigned
impl_try_from_check_gte_0![i8 => u16, u32, u64, u128, usize];
impl_try_from_check_gte_0![i16 => u32, u64, u128];
impl_try_from_check_gte_0![i32 => u64, u128];

// Signed => Smaller Unsigned, Same Size Unsigned
impl_try_from_check_gte_0!(i8 => u8, u16, u32, u64, u128);
impl_try_from_check_gte_0!(i16 => u16, u32, u64, u128);
impl_try_from_check_gte_0!(i32 => u32, u64, u128);
impl_try_from_check_gte_0!(i64 => u64, u128);
impl_try_from_check_gte_0!(i128 => u128);

// isize => usize
impl_try_from_check_gte_0!(isize => usize);

// A => NonMax<B> & NonMax<A> => NonMax<B>  if A < B::MAX
macro_rules! impl_try_from_check_lt_max {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = TryFromIntError;
            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                if src < $target::MAX as $source {
                    unsafe { Self::new_unchecked(src as $target) }
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
        impl TryFrom<NonMax<$source>> for NonMax<$target> {
            type Error = TryFromIntError;
            #[inline]
            fn try_from(src: NonMax<$source>) -> Result<Self, Self::Error> {
                let src = src.get();
                if src < $target::MAX as $source {
                    unsafe { Self::new_unchecked(src as $target) }
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
    )*}
}

// Unsigned => Smaller Signed, Same Size Signed
impl_try_from_check_lt_max![u8 => i8];
impl_try_from_check_lt_max![u16 => i8, i16];
impl_try_from_check_lt_max![u32 => i8, i16, i32];
impl_try_from_check_lt_max![u64 => i8, i16, i32, i64];
impl_try_from_check_lt_max![u128 => i8, i16, i32, i64, i128];

// Unsigned => Smaller Unsigned
impl_try_from_check_lt_max![u16 => u8];
impl_try_from_check_lt_max![u32 => u8, u16];
impl_try_from_check_lt_max![u64 => u8, u16, u32];
impl_try_from_check_lt_max![u128 => u8, u16, u32, u64];

// usize => isize
impl_try_from_check_lt_max!(usize => isize);

// A => NonMax<B> & NonMax<A> => NonMax<B>  if A >= B::MIN && A < B::MAX
macro_rules! impl_try_from_check_gte_min_lt_max {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                if src >= ($target::MIN as $source) || src < ($target::MAX as $source) {
                    unsafe { Self::new_unchecked(src as $target) }
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
        impl TryFrom<NonMax<$source>> for NonMax<$target> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(src: NonMax<$source>) -> Result<Self, Self::Error> {
                let src = src.get();
                if src >= ($target::MIN as $source) || src < ($target::MAX as $source) {
                    unsafe { Self::new_unchecked(src as $target) }
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
    )*}
}

// Signed -> Smaller Signed
impl_try_from_check_gte_min_lt_max![i16 => i8];
impl_try_from_check_gte_min_lt_max![i32 => i8, i16];
impl_try_from_check_gte_min_lt_max![i64 => i8, i16, i32];
impl_try_from_check_gte_min_lt_max![i128 => i8, i16, i32, i64];

// Signed => Smaller Unsigned
impl_try_from_check_gte_min_lt_max![i16 => u8];
impl_try_from_check_gte_min_lt_max![i32 => u8, u16];
impl_try_from_check_gte_min_lt_max![i64 => u8, u16, u32];
impl_try_from_check_gte_min_lt_max![i128 => u8, u16, u32, u64];

#[cfg(test)]
mod test {
    use super::NonMax;
    use crate::Idx;
    #[test]
    fn nonmax_constants() {
        assert_eq!(NonMax::<u32>::ZERO.get(), 0);
        assert_eq!(NonMax::<u32>::ONE.get(), 1);
        assert_eq!(NonMax::<u32>::MIN.get(), 0);
        assert_eq!(NonMax::<i32>::MIN.get(), i32::MIN);
        assert_eq!(NonMax::<u32>::MAX.get(), u32::MAX - 1);
        assert_eq!(NonMax::<u32>::new(u32::MAX), None);
    }

    #[test]
    fn nonmax_idx() {
        assert_eq!(NonMax::<u8>::from_usize(254).into_usize(), 254);
    }

    #[test]
    #[should_panic]
    fn nonmax_oob() {
        NonMax::<u8>::from_usize(255);
    }
}
