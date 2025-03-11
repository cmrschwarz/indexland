use indexland::{Idx, IdxEnum, IndexArray};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct IdxManual(pub usize);
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
impl core::ops::Add for IdxManual {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() + rhs.into_usize())
    }
}
impl core::ops::Sub for IdxManual {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() - rhs.into_usize())
    }
}
impl core::ops::Rem for IdxManual {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() % rhs.into_usize())
    }
}
impl core::ops::AddAssign for IdxManual {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl core::ops::SubAssign for IdxManual {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl core::ops::RemAssign for IdxManual {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum EnumIdxManual2 {
    #[default]
    A,
    B,
}
impl Idx for EnumIdxManual2 {
    const ZERO: Self = Self::A;
    const ONE: Self = Self::B;
    const MAX: Self = Self::B;
    fn from_usize(v: usize) -> Self {
        match v {
            0 => Self::A,
            1 => Self::B,
            _ => panic!(),
        }
    }
    fn from_usize_unchecked(v: usize) -> Self {
        match v {
            0 => Self::A,
            1 => Self::B,
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
impl IdxEnum for EnumIdxManual2 {
    const COUNT: usize = 2;

    const VARIANTS: &'static [Self] = &[Self::A, Self::B];

    type EnumIndexArray<T> = IndexArray<Self, T, { Self::COUNT }>;
}
impl core::ops::Add for EnumIdxManual2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() + rhs.into_usize())
    }
}
impl core::ops::Sub for EnumIdxManual2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() - rhs.into_usize())
    }
}
impl core::ops::Rem for EnumIdxManual2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() % rhs.into_usize())
    }
}
impl core::ops::AddAssign for EnumIdxManual2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl core::ops::SubAssign for EnumIdxManual2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl core::ops::RemAssign for EnumIdxManual2 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum EnumIdxManual3 {
    #[default]
    A,
    B,
    C,
}
impl IdxEnum for EnumIdxManual3 {
    const COUNT: usize = 3;
    const VARIANTS: &'static [Self] = &[Self::A, Self::B, Self::B];
    type EnumIndexArray<T> = IndexArray<Self, T, { Self::COUNT }>;
}
impl Idx for EnumIdxManual3 {
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
            0 => Self::A,
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
impl core::ops::Add for EnumIdxManual3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() + rhs.into_usize())
    }
}
impl core::ops::Sub for EnumIdxManual3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() - rhs.into_usize())
    }
}
impl core::ops::Rem for EnumIdxManual3 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Idx::from_usize(self.into_usize() % rhs.into_usize())
    }
}
impl core::ops::AddAssign for EnumIdxManual3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl core::ops::SubAssign for EnumIdxManual3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl core::ops::RemAssign for EnumIdxManual3 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}
