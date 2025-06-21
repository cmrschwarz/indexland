#![allow(clippy::inline_always)]

pub trait Idx: 'static + Copy + Ord {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    // We can't use `From<usize>` because e.g. u32 does not implement
    // that, and we can't implement it for it (orphan rule).
    // We also can't have a blanket impl of this trait for types that implement
    // `From<usize>` because then we can't add any manual ones (orphan rule
    // again).

    fn from_usize(v: usize) -> Self;
    fn from_usize_unchecked(v: usize) -> Self;

    fn into_usize(self) -> usize;
    fn into_usize_unchecked(self) -> usize;

    /// Careful with signed integers as this might make them negative.
    ///
    /// That would cause the next `into_usize` conversion to panic.
    fn wrapping_add(self, other: Self) -> Self;

    /// Careful with signed integers as this might make them negative.
    ///
    /// That would cause the next `into_usize` conversion to panic.
    fn wrapping_sub(self, other: Self) -> Self;

    fn saturating_add(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize()
                .saturating_add(other.into_usize())
                .min(Self::MAX.into_usize()),
        )
    }
    fn saturating_sub(self, other: Self) -> Self {
        Self::from_usize(
            self.into_usize()
                .saturating_sub(other.into_usize())
                .min(Self::MAX.into_usize()),
        )
    }
}

pub trait IdxEnum: Idx {
    const VARIANT_COUNT: usize;
    const VARIANTS: &'static [Self];

    /// Helper type to construct [`EnumIndexArray`](crate::index_array::EnumIndexArray)
    /// on stable Rust without const generics.
    /// This should always be equivalent to `IndexArray<Self, T, { Self::VARIANT_COUNT }>`.
    /// Please make sure to honor this when implementing this trait manually.
    type EnumIndexArray<T>; // = `IndexArray<Self, T, { Self::VARIANT_COUNT }>`

    fn iter() -> core::iter::Copied<core::slice::Iter<'static, Self>> {
        Self::VARIANTS.iter().copied()
    }
}

pub trait IdxNewtype: Idx {
    type Base: Idx;
    fn new(inner: Self::Base) -> Self;
    fn into_inner(self) -> Self::Base;
}

pub trait IdxCompat<I>: Idx {
    fn idx_cast(self) -> I;
}

impl<I: Idx> IdxCompat<I> for I {
    #[inline(always)]
    fn idx_cast(self) -> I {
        self
    }
}

pub trait ArithCompat<I>: Idx {
    fn to_idx(self) -> I;
}

impl<I: Idx> ArithCompat<I> for I {
    #[inline(always)]
    fn to_idx(self) -> I {
        self
    }
}

impl Idx for usize {
    const ZERO: usize = 0;
    const ONE: usize = 1;
    const MAX: usize = usize::MAX;
    #[inline(always)]
    fn into_usize(self) -> usize {
        self
    }
    #[inline(always)]
    fn from_usize(v: usize) -> Self {
        v
    }
    #[inline(always)]
    fn into_usize_unchecked(self) -> usize {
        self
    }
    #[inline(always)]
    fn from_usize_unchecked(v: usize) -> Self {
        v
    }
    #[inline(always)]
    fn wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    #[inline(always)]
    fn wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    #[inline(always)]
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
    #[inline(always)]
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}

macro_rules! primitive_idx_implemenation_unsized {
    ($($primitive: ident),*) => {$(
        impl Idx for $primitive {
            const ZERO: $primitive = 0;
            const ONE: $primitive = 1;
            const MAX: $primitive = $primitive::MAX;
            #[inline(always)]
            fn into_usize(self) -> usize {
                ::core::convert::TryInto::<usize>::try_into(self).unwrap()
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                ::core::convert::TryInto::<$primitive>::try_into(v).unwrap()
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                self as usize
            }
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                v as $primitive
            }
            #[inline(always)]
            fn wrapping_add(self, other: Self) -> Self {
                $primitive::wrapping_add(self, other)
            }
            #[inline(always)]
            fn wrapping_sub(self, other: Self) -> Self {
                $primitive::wrapping_sub(self, other)
            }
            #[inline(always)]
            fn saturating_add(self, other: Self) -> Self {
                $primitive::saturating_add(self, other)
            }
            #[inline(always)]
            fn saturating_sub(self, other: Self) -> Self {
                $primitive::saturating_sub(self, other)
            }
        }
    )*};
}

macro_rules! primitive_idx_implemenation_sized {
    ($($primitive: ident),*) => {$(
        impl Idx for $primitive {
            const ZERO: $primitive = 0;
            const ONE: $primitive = 1;
            const MAX: $primitive = $primitive::MAX;
            #[inline(always)]
            fn into_usize(self) -> usize {
                ::core::convert::TryInto::<usize>::try_into(self).unwrap()
            }
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                ::core::convert::TryInto::<$primitive>::try_into(v).unwrap()
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                self as usize
            }
             #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #![allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                v as $primitive
            }
            fn wrapping_add(self, other: Self) -> Self {
                $primitive::wrapping_add(self, other)
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $primitive::wrapping_sub(self, other)
            }
            fn saturating_add(self, other: Self) -> Self {
                $primitive::saturating_add(self, other)
            }
            fn saturating_sub(self, other: Self) -> Self {
                $primitive::saturating_sub(self, other)
            }
        }
    )*};
}

primitive_idx_implemenation_unsized![u8, u16, u32, u64];
primitive_idx_implemenation_sized![isize, i8, i16, i32, i64];

/// Declarative alternative to [`#[derive(IdxNewtype)]`](indexland_derive::IdxNewtype).
///
/// Allows generating multiple indices at once and does not require
/// proc-macros.
///
/// # Example
/// ```rust
/// # use indexland::idx_newtype;
/// idx_newtype! {
///     struct FooId(usize);
///     struct BarId(u32);
/// }
/// ```
#[macro_export]
macro_rules! idx_newtype {
    { $( $(#[$attrs: meta])* $type_vis: vis struct $name: ident ($base_vis: vis $base_type: path); )* } => {$(
        $(#[$attrs])*
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $type_vis struct $name ($base_vis $base_type);

        impl $crate::Idx for $name {
            const ZERO: Self = $name(<$base_type as $crate::Idx>::ZERO);
            const ONE: Self = $name(<$base_type as $crate::Idx>::ONE);
            const MAX: Self = $name(<$base_type as $crate::Idx>::MAX);
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                $name(<$base_type as $crate::Idx>::from_usize(v))
            }
            #[inline(always)]
            fn into_usize(self) -> usize {
                <$base_type as $crate::Idx>::into_usize(self.0)
            }
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                $name(<$base_type as $crate::Idx>::from_usize_unchecked(v))
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                <$base_type as $crate::Idx>::into_usize_unchecked(self.0)
            }
            fn saturating_add(self, other: Self) -> Self {
               $name(<$base_type as  $crate::Idx>::saturating_add(self.0, other.0))
            }
            fn saturating_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::Idx>::saturating_sub(self.0, other.0))
            }
            fn wrapping_add(self, other: Self) -> Self {
                $name(<$base_type as  $crate::Idx>::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                $name(<$base_type as $crate::Idx>::wrapping_sub(self.0, other.0))
            }
        }
        impl $crate::IdxNewtype for $name {
            type Base = $base_type;
            fn new(v: $base_type) -> Self {
                $name(v)
            }
            fn into_inner(self) -> $base_type {
                self.0
            }
        }
        impl ::core::convert::From<usize> for $name {
            #[inline(always)]
            fn from(v: usize) -> $name {
                $name(<$base_type as $crate::Idx>::from_usize(v))
            }
        }
        impl ::core::convert::From<$name> for usize {
            #[inline(always)]
            fn from(v: $name) -> usize {
                <$base_type as $crate::Idx>::into_usize(v.0)
            }
        }
        impl ::core::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl ::core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::core::ops::Add for $name {
            type Output = Self;
            #[inline]
            fn add(self, other: Self) -> Self {
                $name(self.0 + other.0)
            }
        }
        impl ::core::ops::Sub for $name {
            type Output = Self;
            #[inline]
            fn sub(self, other: Self) -> Self {
                $name(self.0 - other.0)
            }
        }
        impl ::core::ops::Rem for $name {
            type Output = Self;
            #[inline]
            fn rem(self, other: Self) -> Self {
                $name(self.0 % other.0)
            }
        }
        impl ::core::ops::AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }
        impl ::core::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }
        impl ::core::ops::RemAssign for $name {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                self.0 %= other.0;
            }
        }
    )*};
}

#[cfg(test)]
mod test {
    use crate::{enum_index_array, index_array, EnumIndexArray, IndexArray};

    use super::{Idx, IdxEnum};

    #[test]
    fn idx_manual() {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct IdxManual(usize);
        impl Idx for IdxManual {
            const ZERO: Self = IdxManual(0);
            const ONE: Self = IdxManual(1);
            const MAX: Self = IdxManual(usize::MAX);
            fn from_usize(v: usize) -> Self {
                IdxManual(v)
            }
            fn from_usize_unchecked(v: usize) -> Self {
                IdxManual(v)
            }
            fn into_usize(self) -> usize {
                self.0
            }
            fn into_usize_unchecked(self) -> usize {
                self.0
            }
            fn wrapping_add(self, other: Self) -> Self {
                IdxManual(self.0.wrapping_add(other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                IdxManual(self.0.wrapping_sub(other.0))
            }
            fn saturating_add(self, other: Self) -> Self {
                IdxManual(self.0.saturating_add(other.0))
            }
            fn saturating_sub(self, other: Self) -> Self {
                IdxManual(self.0.saturating_sub(other.0))
            }
        }

        let x: IndexArray<IdxManual, i32, 3> = index_array![1, 2, 3];
        assert_eq!(x[IdxManual(1)], 2);
    }

    #[test]
    fn enum_idx_manual() {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum EnumIdxManual {
            A,
            B,
            C,
        }
        impl IdxEnum for EnumIdxManual {
            const VARIANT_COUNT: usize = 3;
            const VARIANTS: &'static [Self] = &[Self::A, Self::B, Self::B];
            type EnumIndexArray<T> = IndexArray<Self, T, { Self::VARIANT_COUNT }>;
        }
        impl Idx for EnumIdxManual {
            const ZERO: Self = Self::A;
            const ONE: Self = Self::B;
            const MAX: Self = Self::C;
            fn from_usize(v: usize) -> Self {
                match v {
                    0 => Self::A,
                    1 => Self::B,
                    2 => Self::C,
                    _ => panic!(),
                }
            }
            fn from_usize_unchecked(v: usize) -> Self {
                match v {
                    1 => Self::B,
                    2 => Self::C,
                    _ => Self::A,
                }
            }
            fn into_usize(self) -> usize {
                self as usize
            }
            fn into_usize_unchecked(self) -> usize {
                self as usize
            }
            fn wrapping_add(self, _other: Self) -> Self {
                unimplemented!()
            }
            fn wrapping_sub(self, _other: Self) -> Self {
                unimplemented!()
            }
            fn saturating_add(self, _other: Self) -> Self {
                unimplemented!()
            }
            fn saturating_sub(self, _other: Self) -> Self {
                unimplemented!()
            }
        }

        let x: IndexArray<EnumIdxManual, i32, 3> = index_array![1, 2, 3];
        assert_eq!(x[EnumIdxManual::C], 3);

        let x: EnumIndexArray<EnumIdxManual, i32> = enum_index_array![1, 2, 3];
        assert_eq!(x[EnumIdxManual::C], 3);

        let x: EnumIndexArray<EnumIdxManual, i32> =
            enum_index_array![EnumIdxManual::A => 1,EnumIdxManual::B =>  2, EnumIdxManual::C => 3];
        assert_eq!(x[EnumIdxManual::A], 1);
    }
}
