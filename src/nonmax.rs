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
pub struct NonMax<P: NonMaxPrimitive>(P::NonMaxInner);

#[derive(Debug)]
pub struct NonMaxOutOfRangeError;

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

    pub fn new(v: P) -> Option<Self> {
        let inner = P::NonMaxInner::new(v)?;
        Some(NonMax(inner))
    }
    /// # Safety
    /// value must not be `MAX`
    pub unsafe fn new_unchecked(v: P) -> Self {
        NonMax(unsafe { P::NonMaxInner::new_unchecked(v) })
    }

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.0.get(), f)
    }
}
impl<P: NonMaxPrimitive> Display for NonMax<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            const ZERO: Self =
                unsafe { NonMax::<$primitive>::new_unchecked_const(0) }.0;

            const ONE: Self =
                unsafe { NonMax::<$primitive>::new_unchecked_const(1) }.0;

            const MIN: Self = unsafe {
                NonMax::<$primitive>::new_unchecked_const($primitive::MIN)
            }
            .0;
            const MAX: Self = unsafe {
                NonMax::<$primitive>::new_unchecked_const($primitive::MAX - 1)
            }
            .0;
            fn new(v: $primitive) -> Option<Self> {
                if v == $primitive::MAX {
                    return None;
                }
                Some(unsafe{Self::new_unchecked(v)})
            }
            unsafe fn new_unchecked(v: $primitive) -> Self {
                unsafe { NonMax::<$primitive>::new_unchecked_const(v) }.0
            }
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
