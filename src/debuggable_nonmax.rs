use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

#[cfg(not(debug_assertions))]
use std::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

#[derive(Debug)]
pub struct NonMaxOutOfRangeError;

macro_rules! debuggable_nonmax {
    ($nonmax: ident, $nonzero: ident, $primitive: ident) => {
        #[cfg(debug_assertions)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $nonmax($primitive);

        #[cfg(not(debug_assertions))]
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

        #[cfg(debug_assertions)]
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

        #[cfg(not(debug_assertions))]
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

debuggable_nonmax!(DebuggableNonMaxUsize, NonZeroUsize, usize);
debuggable_nonmax!(DebuggableNonMaxU32, NonZeroU32, u32);
debuggable_nonmax!(DebuggableNonMaxU64, NonZeroU64, u64);
