// This is similar to the nonmax crate, but makes debugging much less painful
// through the "debuggable_nonmax" feature of this crate with removes the
// optimization in debug mode to allow debuggers to report the correct integer
// values.

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::Idx;

#[cfg(not(all(feature = "debuggable_nonmax", debug_assertions)))]
use std::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

#[derive(Debug)]
pub struct NonMaxOutOfRangeError;

macro_rules! nonmax_impl {
    ($nonmax: ident, $nonzero: ident, $primitive: ident) => {
        #[cfg(all(feature = "debuggable_nonmax", debug_assertions))]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $nonmax($primitive);

        #[cfg(not(all(feature = "debuggable_nonmax", debug_assertions)))]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $nonmax($nonzero);

        impl From<$nonmax> for $primitive {
            fn from(value: $nonmax) -> Self {
                value.get()
            }
        }

        impl TryFrom<$primitive> for $nonmax {
            type Error = NonMaxOutOfRangeError;

            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                $nonmax::new(value).ok_or(NonMaxOutOfRangeError)
            }
        }

        #[cfg(all(feature = "debuggable_nonmax", debug_assertions))]
        impl $nonmax {
            #[inline]
            pub const fn get(self) -> $primitive {
                self.0
            }
            #[inline]
            pub const unsafe fn new_unchecked(value: $primitive) -> Self {
                Self(value)
            }
        }

        #[cfg(not(all(feature = "debuggable_nonmax", debug_assertions)))]
        impl $nonmax {
            #[inline]
            pub const fn get(self) -> $primitive {
                self.0.get() ^ $primitive::MAX
            }

            #[inline]
            pub const unsafe fn new_unchecked(value: $primitive) -> Self {
                Self(unsafe {
                    $nonzero::new_unchecked(value ^ $primitive::MAX)
                })
            }
        }

        impl $nonmax {
            pub const ZERO: $nonmax = unsafe { Self::new_unchecked(0) };
            pub const ONE: $nonmax = unsafe { Self::new_unchecked(1) };
            pub const MIN: $nonmax = Self::ZERO;
            pub const MAX: $nonmax =
                unsafe { Self::new_unchecked($primitive::MAX - 1) };

            #[inline]
            pub fn new(value: $primitive) -> Option<$nonmax> {
                if value == $primitive::MAX {
                    None
                } else {
                    Some(unsafe { Self::new_unchecked(value) })
                }
            }

            #[inline]
            pub fn set(&mut self, value: $primitive) {
                assert!(value != $primitive::MAX);
                *self = unsafe { Self::new_unchecked(value) };
            }

            pub fn try_set(
                &mut self,
                value: $primitive,
            ) -> Result<(), NonMaxOutOfRangeError> {
                if value == $primitive::MAX {
                    return Err(NonMaxOutOfRangeError);
                }
                *self = unsafe { Self::new_unchecked(value) };
                Ok(())
            }

            pub const fn checked_add(&self, rhs: $nonmax) -> Option<$nonmax> {
                let Some(res) = self.get().checked_add(rhs.get()) else {
                    return None;
                };
                if res == $primitive::MAX {
                    return None;
                }
                Some(unsafe { Self::new_unchecked(res) })
            }

            pub const fn checked_sub(&self, rhs: $nonmax) -> Option<$nonmax> {
                let Some(res) = self.get().checked_sub(rhs.get()) else {
                    return None;
                };
                if res == $primitive::MAX {
                    return None;
                }
                Some(unsafe { Self::new_unchecked(res) })
            }

            pub const fn wrapping_add(&self, rhs: $nonmax) -> $nonmax {
                let mut res = self.get().wrapping_add(rhs.get());
                if res == $primitive::MAX {
                    res = 0;
                }
                unsafe { Self::new_unchecked(res) }
            }

            pub const fn wrapping_sub(&self, rhs: $nonmax) -> $nonmax {
                let mut res = self.get().wrapping_sub(rhs.get());
                if res == $primitive::MAX {
                    res = 0;
                }
                unsafe { Self::new_unchecked(res) }
            }

            pub const fn saturating_add(&self, rhs: $nonmax) -> $nonmax {
                let mut res = self.get().saturating_add(rhs.get());
                if res == $primitive::MAX {
                    res -= 1;
                }
                unsafe { Self::new_unchecked(res) }
            }

            pub const fn saturating_sub(&self, rhs: $nonmax) -> $nonmax {
                let mut res = self.get().wrapping_sub(rhs.get());
                if res == $primitive::MAX {
                    res -= 1;
                }
                unsafe { Self::new_unchecked(res) }
            }
        }

        impl Debug for $nonmax {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                Debug::fmt(&self.get(), f)
            }
        }
        impl Display for $nonmax {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                Display::fmt(&self.get(), f)
            }
        }
        impl Ord for $nonmax {
            fn cmp(&self, other: &Self) -> Ordering {
                self.get().cmp(&other.get())
            }
        }
        impl PartialOrd for $nonmax {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Default for $nonmax {
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl std::ops::Add for $nonmax {
            type Output = $nonmax;

            fn add(self, rhs: Self) -> Self::Output {
                self.checked_add(rhs).unwrap()
            }
        }
        impl std::ops::Sub for $nonmax {
            type Output = $nonmax;

            fn sub(self, rhs: Self) -> Self::Output {
                self.checked_sub(rhs).unwrap()
            }
        }
        impl std::ops::AddAssign for $nonmax {
            fn add_assign(&mut self, rhs: Self) {
                *self = std::ops::Add::add(*self, rhs);
            }
        }
        impl std::ops::SubAssign for $nonmax {
            fn sub_assign(&mut self, rhs: Self) {
                *self = std::ops::Sub::sub(*self, rhs);
            }
        }
    };
}

macro_rules! nonmax_idx_impl {
    ($nonmax: ident, $primitive: ident) => {
        impl Idx for $nonmax {
            const MAX: Self = $nonmax::MAX;
            const ZERO: Self = $nonmax::ZERO;
            const ONE: Self = $nonmax::ONE;

            #[inline(always)]
            fn into_usize(self) -> usize {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                self.get() as usize
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                // TODO: maybe add features where we assert this?
                #![allow(clippy::cast_possible_truncation)]
                $nonmax::new(v as $primitive).unwrap()
            }
            fn wrapping_add(self, other: Self) -> Self {
                $nonmax::wrapping_add(&self, other)
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $nonmax::wrapping_add(&self, other)
            }
        }
    };
}

nonmax_impl!(NonMaxUsize, NonZeroUsize, usize);
nonmax_impl!(NonMaxIsize, NonZeroIsize, isize);

nonmax_impl!(NonMaxI8, NonZeroI8, i8);
nonmax_impl!(NonMaxI16, NonZeroI16, i16);
nonmax_impl!(NonMaxI32, NonZeroI32, i32);
nonmax_impl!(NonMaxI64, NonZeroI64, i64);

nonmax_impl!(NonMaxU8, NonZeroU8, u8);
nonmax_impl!(NonMaxU16, NonZeroU16, u16);
nonmax_impl!(NonMaxU32, NonZeroU32, u32);
nonmax_impl!(NonMaxU64, NonZeroU64, u64);

nonmax_idx_impl!(NonMaxUsize, usize);
nonmax_idx_impl!(NonMaxU8, u8);
nonmax_idx_impl!(NonMaxU16, u16);
nonmax_idx_impl!(NonMaxU32, u32);
nonmax_idx_impl!(NonMaxU64, u64);
