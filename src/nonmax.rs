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
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use core::convert::TryFrom;

#[cfg(any(not(debug_assertions), feature = "disable_debuggable_nonmax"))]
use core::num::NonZero;

use crate::Idx;

/// An Integer value that's dynamically guaranteed to never be MAX. This enables
/// [Niche Layout Optimizations](https://doc.rust-lang.org/std/option/index.html#representation),
/// meaning that e.g. [`Option<NonMax<u32>>`] takes up 4 bytes,
/// whereas [`Option<u32>`] will ususally use 8.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonMax<P: NonMaxPrimitive>(P::NonMaxInner);

#[derive(Debug, Default, Clone, Copy)]
pub struct NonMaxOutOfRangeError;

#[cfg(feature = "std")]
impl std::error::Error for NonMaxOutOfRangeError {}

impl core::fmt::Display for NonMaxOutOfRangeError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "value out of range for NonMax integer type")
    }
}

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

pub trait NonMaxInner<P>: Sized + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {
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

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
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
    pub fn saturating_add(self, rhs: Self) -> Self {
        NonMax(self.0.saturating_add(rhs.0))
    }
    pub fn saturating_sub(self, rhs: Self) -> Self {
        NonMax(self.0.saturating_sub(rhs.0))
    }
    pub fn saturating_mul(self, rhs: Self) -> Self {
        NonMax(self.0.saturating_mul(rhs.0))
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

macro_rules! impl_wrapping_fn {
    ($primitive: ty => $($func_name: ident),* $(,)?) => {$(
        fn $func_name(self, rhs: Self) -> Self {
            #[cfg(all(
                debug_assertions,
                not(feature = "disable_debuggable_nonmax")
            ))]
            let mut res = <$primitive>::$func_name(self, rhs);

            #[cfg(any(
                not(debug_assertions),
                feature = "disable_debuggable_nonmax"
            ))]
            let mut res = self.get().$func_name(rhs.get());

            if res == <$primitive>::MAX {
                res = 0;
            }
            unsafe { Self::new_unchecked(res) }
        }
    )*};
}

macro_rules! impl_nonmax {
    ($($primitive: ty),*) => {$(
        impl NonMax<$primitive> {
            pub const fn new(v: $primitive) -> Option<Self> {
                if v == <$primitive>::MAX {
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
                NonMax(unsafe { NonZero::new_unchecked(v ^ <$primitive>::MAX) })
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
            const MIN: Self = NonMax::<$primitive>::new(<$primitive>::MIN).unwrap().0;
            const MAX: Self = NonMax::<$primitive>::new(<$primitive>::MAX - 1).unwrap().0;

            fn new(v: $primitive) -> Option<Self> {
                if v == <$primitive>::MAX {
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
                {
                    self.get() ^ <$primitive>::MAX
                }
            }
            impl_wrapping_fn![ $primitive =>
                wrapping_add, wrapping_sub, wrapping_mul,
                saturating_add, saturating_sub, saturating_mul,
            ];
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
            const MAX_USIZE: usize = <$primitive as Idx>::MAX_USIZE;

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
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_sign_loss,
                )]
                NonMax::<$primitive>::new(v as $primitive).unwrap_or(NonMax::ZERO)
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_sign_loss,
                )]
                self.get() as usize
            }
            fn wrapping_add(self, other: Self) -> Self {
                NonMax::<$primitive>::wrapping_add(self, other)
            }
            fn wrapping_sub(self, other: Self) -> Self {
                NonMax::<$primitive>::wrapping_sub(self, other)
            }
            fn saturating_add(self, other: Self) -> Self {
                NonMax::<$primitive>::saturating_add(self, other)
            }
            fn saturating_sub(self, other: Self) -> Self {
                NonMax::<$primitive>::saturating_sub(self, other)
            }
        }
    )*};
}

impl_nonmax![u8, u16, u32, u64, u128, usize];
impl_nonmax![i8, i16, i32, i64, i128, isize];

impl_nonmax_idx![u8, u16, u32, u64, u128, usize];
impl_nonmax_idx![i8, i16, i32, i64, i128, isize];

// unchecked A => NonMax<B> & NonMax<A> => NonMax<B>
macro_rules! impl_from_unchecked {
    ( $source: ty => $($target: ty),* ) => {$(
        impl From<$source> for NonMax<$target> {
            #[inline]
            fn from(src: $source) -> Self {
                #[allow(clippy::cast_lossless)]
                unsafe { Self::new_unchecked(src as $target) }
            }
        }
        impl From<NonMax<$source>> for NonMax<$target> {
            #[inline]
            fn from(src: NonMax<$source>) -> Self {
                #[allow(clippy::cast_lossless)]
                unsafe { Self::new_unchecked(src.get() as $target) }
            }
        }
    )*};
}

// Unsigned => Larger Unsigned
impl_from_unchecked![u8 => u16, u32, u64, u128];
impl_from_unchecked![u16 => u32, u64, u128];
impl_from_unchecked![u32 => u64, u128];
impl_from_unchecked![u64 => u128];

// Signed => Larger Signed
impl_from_unchecked![i8 => i16, i32, i64, i128];
impl_from_unchecked![i16 => i32, i64, i128];
impl_from_unchecked![i32 => i64, i128];
impl_from_unchecked![i64 => i128];

// Unsigned => Larger Signed
impl_from_unchecked![u8 => i16, i32, i64, i128];
impl_from_unchecked![u16 => i32, i64, i128];
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
                    #[allow(clippy::cast_sign_loss)]
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
                    #[allow(clippy::cast_sign_loss)]
                    Ok(unsafe { Self::new_unchecked(src as $target) })
                } else {
                    Err(NonMaxOutOfRangeError)
                }
            }
        }
    )*}
}

// Signed => Smaller Unsigned, Same Size Unsigned
impl_try_from_check_gte_0!(i8 => u8, u16, u32, u64, u128);
impl_try_from_check_gte_0!(i16 => u16, u32, u64, u128);
impl_try_from_check_gte_0!(i32 => u32, u64, u128);
impl_try_from_check_gte_0!(i64 => u64, u128);
impl_try_from_check_gte_0!(i128 => u128);

// A => NonMax<B> & NonMax<A> => NonMax<B>  if A < B::MAX
macro_rules! impl_try_from_check_lt_max {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;
            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                #![allow(
                    clippy::cast_sign_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_lossless
                )]
                if src < <$target>::MAX as $source {
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
                #![allow(
                    clippy::cast_sign_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_lossless
                )]
                let src = src.get();
                if src < <$target>::MAX as $source {
                    Ok(unsafe { Self::new_unchecked(src as $target) })
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

// A => NonMax<B> & NonMax<A> => NonMax<B>  if A >= B::MIN && A < B::MAX
macro_rules! impl_try_from_check_gte_min_lt_max {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;

            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                #![allow(
                    clippy::cast_sign_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_lossless
                )]
                if src >= (<$target>::MIN as $source) && src < (<$target>::MAX as $source) {

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
                #![allow(
                    clippy::cast_sign_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_lossless
                )]
                let src = src.get();
                if src >= (<$target>::MIN as $source) && src < (<$target>::MAX as $source) {
                    Ok(unsafe { Self::new_unchecked(src as $target) })
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

macro_rules! impl_try_from_target_dependant {
    ($source:ty => $($target:ty),+) => {$(
        impl TryFrom<$source> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;

            #[inline]
            fn try_from(src: $source) -> Result<Self, Self::Error> {
                if let Ok(src) = <$target>::try_from(src) {
                    if src != <$target>::MAX {
                        return Ok(unsafe { Self::new_unchecked(src) });
                    }
                }
                Err(NonMaxOutOfRangeError)
            }
        }
        impl TryFrom<NonMax<$source>> for NonMax<$target> {
            type Error = NonMaxOutOfRangeError;

            #[inline]
            fn try_from(src: NonMax<$source>) -> Result<Self, Self::Error> {
                if let Ok(src) = <$target>::try_from(src.get()) {
                    if src != <$target>::MAX {
                        return Ok(unsafe { Self::new_unchecked(src) });
                    }
                }
                Err(NonMaxOutOfRangeError)
            }
        }
    )*}
}

macro_rules! rev {
    ($mac:ident, $($target:ty),+ => $source:ty) => {$(
        $mac!($target => $source);
    )*}
}

// usize => xx
impl_try_from_check_lt_max![usize => u8, u16];
impl_try_from_check_lt_max!(usize => i8, i16, isize);
impl_try_from_target_dependant![usize => u32, u64, u128];
impl_try_from_target_dependant![usize => i32, i64, i128];

// xx => usize
rev![impl_from_unchecked, u8, u16 => usize];
rev![impl_try_from_check_gte_0, i8, i16 => usize];
rev![impl_try_from_target_dependant, u32, u64, u128 => usize];
rev![impl_try_from_target_dependant, i32, i64, i128 => usize];

// isize => xx
impl_try_from_check_gte_min_lt_max![isize => i8];
impl_try_from_check_gte_min_lt_max!(isize => u8);
impl_try_from_target_dependant![isize => u16, u32, u64, u128];
impl_try_from_target_dependant![isize => i16, i32, i64, i128];
impl_try_from_check_gte_0!(isize => usize);

// xx => isize
rev![impl_from_unchecked, u8 => isize];
rev![impl_from_unchecked, i8, i16 => isize];
rev![impl_try_from_target_dependant, u16, u32, u64, u128 => isize];
rev![impl_try_from_target_dependant, i32, i64, i128 => isize];

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
    #[should_panic(expected = "NonMaxOutOfRangeError")]
    fn nonmax_oob() {
        let _ = NonMax::<u8>::from_usize(255);
    }

    #[test]
    fn all_conversions_possible() {
        macro_rules! assert_conv_works {
            ($($t: ty),*) => {
                assert_conv_works!(@expand, ($($t),*), ($($t),*));
            };
            (@expand, ($($from: ty),*), $all: tt) => {
                $(
                    assert_conv_works!(@impl, $from, $all);
                )*
            };
            (@impl, $from: ty, ($($to: ty),*)) => {
                let from_s = stringify!($from);

                #[allow(
                    irrefutable_let_patterns,
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_lossless,
                    clippy::cast_possible_wrap
                )]
                for v in [ 0, 1, 2, -1i128 as $from, $(<$to>::MAX as $from),*, $((<$to>::MAX - 1) as $from),* ] {
                    $(
                        let to_s = stringify!($to);

                        let from_primitive = <NonMax<$to>>::try_from(v).ok();

                        let from_primitive_expected = if let Ok(v_cast) = <$to>::try_from(v) {
                            if v_cast >= <$to>::MAX {
                                None
                            }
                            else {
                                Some(v_cast)
                            }
                        } else {
                            None
                        };

                        assert_eq!(
                            from_primitive.map(|v| v.get()),
                            from_primitive_expected,
                            "NonMax<{to_s}>::try_from({v} as {from_s}).map(|v|v.get())  == {from_primitive_expected:?}",
                        );

                        assert_eq!(
                            from_primitive,
                            from_primitive_expected.and_then(|v| NonMax::<$to>::new(v)),
                            "NonMax<{to_s}>::try_from({v} as {from_s}) == {from_primitive_expected:?}",
                        );

                        let from_nonmax = NonMax::<$from>::new(v).and_then(|from| <NonMax<$to>>::try_from(from).ok());

                        let from_nonmax_expected = if v == <$from>::MAX {
                            None
                        } else {
                            from_primitive_expected
                        };

                        assert_eq!(
                            from_nonmax.map(|v|v.get()),
                            from_nonmax_expected,
                            "NonMax<{from_s}>::new({v}).and_then(|from| NonMax<{to_s}>).map(|v|v.get()) == {from_nonmax_expected:?}",
                        );

                        assert_eq!(
                            from_nonmax,
                            from_nonmax_expected.and_then(|v| NonMax::<$to>::new(v)),
                            "NonMax<{to_s}>::try_from({v} as {from_s}) == {from_primitive_expected:?}",
                        );
                    )*
                }
            }
        }
        assert_conv_works![
            u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
        ];
    }
}
