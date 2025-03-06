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

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

#[cfg(any(not(debug_assertions), feature = "disable_debuggable_nonmax"))]
use std::num::NonZero;

use crate::Idx;

/// Generic [`NonMax`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonMax<T: NonMaxPrimitive>(T::NonMaxInner);

#[derive(Debug)]
pub struct NonMaxOutOfRangeError;

/// # Safety
/// [`into_inner_unchecked`](NonMaxPrimitive::into_inner_unchecked) and
/// [`from_inner`](NonMaxPrimitive::from_inner)
/// must be the inverse of each other.
pub unsafe trait NonMaxPrimitive:
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
    type NonMaxInner: Sized + Copy + PartialEq + Eq + PartialOrd + Ord + Hash;
    const ZERO: Self::NonMaxInner;
    const ONE: Self::NonMaxInner;
    const MIN: Self::NonMaxInner;
    const MAX: Self::NonMaxInner;

    /// # Safety
    /// `into_inner_unchecked` and `from_inner` must be inverse
    unsafe fn into_inner_unchecked(self) -> Self::NonMaxInner;

    fn into_inner(self) -> Self::NonMaxInner {
        self.try_into_inner().unwrap()
    }
    fn try_into_inner(self) -> Option<Self::NonMaxInner>;

    fn from_inner(inner: Self::NonMaxInner) -> Self;

    fn wrapping_add(
        lhs: Self::NonMaxInner,
        rhs: Self::NonMaxInner,
    ) -> Self::NonMaxInner;
    fn wrapping_sub(
        lhs: Self::NonMaxInner,
        rhs: Self::NonMaxInner,
    ) -> Self::NonMaxInner;
    fn wrapping_mul(
        lhs: Self::NonMaxInner,
        rhs: Self::NonMaxInner,
    ) -> Self::NonMaxInner;
}

impl<P: NonMaxPrimitive> NonMax<P> {
    pub const ZERO: NonMax<P> = NonMax(P::ZERO);
    pub const ONE: NonMax<P> = NonMax(P::ONE);
    pub const MIN: NonMax<P> = NonMax(P::MIN);
    pub const MAX: NonMax<P> = NonMax(P::MAX);
    pub fn new(v: P) -> Option<Self> {
        Some(NonMax(P::try_into_inner(v)?))
    }
    /// # Safety
    /// value must not be `MAX`
    pub unsafe fn new_unchecked(v: P) -> Self {
        NonMax(unsafe { P::into_inner_unchecked(v) })
    }
    pub fn wrapping_add(self, rhs: Self) -> Self {
        NonMax(P::wrapping_add(self.0, rhs.0))
    }
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        NonMax(P::wrapping_sub(self.0, rhs.0))
    }
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        NonMax(P::wrapping_mul(self.0, rhs.0))
    }
    pub fn get(self) -> P {
        P::from_inner(self.0)
    }
}

impl<P: NonMaxPrimitive> Default for NonMax<P> {
    fn default() -> Self {
        Self(P::ZERO)
    }
}

impl<P: NonMaxPrimitive> Debug for NonMax<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&P::from_inner(self.0), f)
    }
}
impl<P: NonMaxPrimitive> Display for NonMax<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&P::from_inner(self.0), f)
    }
}

impl<P: NonMaxPrimitive> Add for NonMax<P> {
    type Output = NonMax<P>;

    fn add(self, rhs: Self) -> Self::Output {
        NonMax(P::into_inner(P::from_inner(self.0) + P::from_inner(rhs.0)))
    }
}
impl<P: NonMaxPrimitive> Sub for NonMax<P> {
    type Output = NonMax<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        NonMax(P::into_inner(P::from_inner(self.0) - P::from_inner(rhs.0)))
    }
}
impl<P: NonMaxPrimitive> Mul for NonMax<P> {
    type Output = NonMax<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        NonMax(P::into_inner(P::from_inner(self.0) * P::from_inner(rhs.0)))
    }
}
impl<P: NonMaxPrimitive> Div for NonMax<P> {
    type Output = NonMax<P>;

    fn div(self, rhs: Self) -> Self::Output {
        NonMax(P::into_inner(P::from_inner(self.0) / P::from_inner(rhs.0)))
    }
}
impl<P: NonMaxPrimitive> Rem for NonMax<P> {
    type Output = NonMax<P>;

    fn rem(self, rhs: Self) -> Self::Output {
        NonMax(P::into_inner(P::from_inner(self.0) % P::from_inner(rhs.0)))
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

macro_rules! nonmax_impl {
    ($($primitive: ident),*) => {$(
        impl NonMax<$primitive> {
            /// # Safety
            #[doc = concat!("Must not be [`", stringify!($primitive), "::MAX`].")]
            pub const unsafe fn new_unchecked_const(v: $primitive) -> Self {
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
        unsafe impl NonMaxPrimitive for $primitive {
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

            const ZERO: Self::NonMaxInner =
                unsafe { NonMax::<$primitive>::new_unchecked_const(0) }.0;

            const ONE: Self::NonMaxInner =
                unsafe { NonMax::<$primitive>::new_unchecked_const(1) }.0;

            const MIN: Self::NonMaxInner = unsafe {
                NonMax::<$primitive>::new_unchecked_const($primitive::MIN)
            }
            .0;

            const MAX: Self::NonMaxInner = unsafe {
                NonMax::<$primitive>::new_unchecked_const($primitive::MAX - 1)
            }
            .0;

            unsafe fn into_inner_unchecked(self) -> Self::NonMaxInner {
                unsafe { NonMax::<$primitive>::new_unchecked_const(self) }.0
            }

            fn try_into_inner(self) -> Option<Self::NonMaxInner> {
                if self == $primitive::MAX {
                    return None;
                }
                Some(unsafe { self.into_inner_unchecked() })
            }

            fn from_inner(inner: Self::NonMaxInner) -> Self {
                #[cfg(all(
                    debug_assertions,
                    not(feature = "disable_debuggable_nonmax")
                ))]
                return inner;

                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                inner.get()
            }

            fn wrapping_add(
                lhs: Self::NonMaxInner,
                rhs: Self::NonMaxInner,
            ) -> Self::NonMaxInner {
                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let (lhs, rhs) = (lhs.get(), rhs.get());

                let mut r = lhs.wrapping_add(rhs);

                if r == $primitive::MAX {
                    r = 0;
                }
                unsafe { NonMax::<$primitive>::new_unchecked_const(r) }.0
            }

            fn wrapping_sub(
                lhs: Self::NonMaxInner,
                rhs: Self::NonMaxInner,
            ) -> Self::NonMaxInner {
                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let (lhs, rhs) = (lhs.get(), rhs.get());

                let mut r = lhs.wrapping_sub(rhs);
                if r == $primitive::MAX {
                    r = 0;
                }
                unsafe { NonMax::<$primitive>::new_unchecked_const(r) }.0
            }

            fn wrapping_mul(
                lhs: Self::NonMaxInner,
                rhs: Self::NonMaxInner,
            ) -> Self::NonMaxInner {
                #[cfg(any(
                    not(debug_assertions),
                    feature = "disable_debuggable_nonmax"
                ))]
                let (lhs, rhs) = (lhs.get(), rhs.get());

                let mut r = lhs.wrapping_mul(rhs);
                if r == $primitive::MAX {
                    r = 0;
                }
                unsafe { NonMax::<$primitive>::new_unchecked_const(r) }.0
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
                NonMax::new(v).ok_or(NonMaxOutOfRangeError)
            }
        }
    )*};
}

macro_rules! nonmax_idx_impl {
    ($($primitive: ident),*) => {$(
        impl Idx for NonMax<$primitive> {
            const ZERO: Self = NonMax::<$primitive>::ZERO;
            const ONE: Self = NonMax::<$primitive>::ONE;
            const MAX: Self = NonMax::<$primitive>::MAX;

            fn from_usize(v: usize) -> Self {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                NonMax::new(v as $primitive).unwrap()
            }
            fn into_usize(self) -> usize {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                self.get() as usize
            }
        }
    )*};
}

nonmax_idx_impl![u8, u16, u32, u64, usize];

nonmax_impl![u8, u16, u32, u64, usize];
nonmax_impl![i8, i16, i32, i64, isize];
