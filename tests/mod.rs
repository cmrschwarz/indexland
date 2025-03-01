use std::mem::MaybeUninit;

use derive_more::{Add, AddAssign, Sub, SubAssign};
use indexland::{
    index_array::{EnumIndexArray, IndexArray},
    index_vec::IndexVec,
    EnumIdx, Idx, NewtypeIdx,
};

#[test]
fn derive_idx_enum() {
    #[derive(EnumIdx)]
    enum Foo {
        A,
        B,
    }

    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}

#[test]
fn derive_idx_newtype() {
    #[derive(NewtypeIdx)]
    pub struct FooIdx(u32);

    let foo = IndexVec::<FooIdx, i32>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooIdx::ONE], 1);
}

#[test]
fn derive_idx_newtype_manual() {
    #[derive(
        Default,
        Idx,
        Add,
        Sub,
        AddAssign,
        SubAssign,
        Clone,
        Copy,
        Hash,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
    )]
    pub struct FooIdx(u32);

    let foo = IndexVec::<FooIdx, i32>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooIdx::ONE], 1);
}

#[test]
fn derive_idx_enum_manual() {
    #[derive(
        Default, Idx, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,
    )]
    enum Foo {
        #[default]
        A,
        B,
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
    impl EnumIdx for Foo {
        const COUNT: usize = 2;
        const VARIANTS: &'static [Self] = &[Foo::A, Foo::B];
        type EnumIndexArray<T> = IndexArray<Self, T, 2>;
    }
    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}

#[test]
fn enum_idx_array_macro() {
    #[derive(EnumIdx)]
    enum Foo {
        A,
        B,
        C,
    }

    const FOO: EnumIndexArray<Foo, i32> = indexland::enum_index_array![
        Foo::A => 1,
        Foo::B => 2,
        Foo::C => 3,
    ];

    assert_eq!(FOO[Foo::B], 2);
}
