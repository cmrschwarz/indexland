use indexland::{EnumIndexArray, Idx, IdxEnum, IndexArray};

#[test]
fn derive_idx_enum_manual() {
    #[derive(
        Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
    )]
    enum Foo {
        #[default]
        A,
        B,
    }
    impl Idx for Foo {
        const ZERO: Self = Foo::A;
        const ONE: Self = Foo::B;
        const MAX: Self = Foo::B;
        fn from_usize(v: usize) -> Self {
            match v {
                0 => Foo::A,
                1 => Foo::B,
                _ => panic!(),
            }
        }
        fn from_usize_unchecked(v: usize) -> Self {
            match v {
                0 => Foo::A,
                1 => Foo::B,
                _ => Foo::A,
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

    impl core::ops::Add for Foo {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Idx::from_usize(self.into_usize() + rhs.into_usize())
        }
    }
    impl core::ops::Sub for Foo {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Idx::from_usize(self.into_usize() - rhs.into_usize())
        }
    }
    impl core::ops::Rem for Foo {
        type Output = Self;

        fn rem(self, rhs: Self) -> Self::Output {
            Idx::from_usize(self.into_usize() % rhs.into_usize())
        }
    }
    impl core::ops::AddAssign for Foo {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl core::ops::SubAssign for Foo {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl core::ops::RemAssign for Foo {
        fn rem_assign(&mut self, rhs: Self) {
            *self = *self % rhs;
        }
    }
    impl IdxEnum for Foo {
        const COUNT: usize = 2;
        const VARIANTS: &'static [Self] = &[Foo::A, Foo::B];
        type EnumIndexArray<T> = IndexArray<Self, T, 2>;
    }
    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}
