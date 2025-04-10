use std::ops::Add;

use indexland::{
    enum_index_array, index_array, EnumIndexArray, Idx, IdxEnum, IdxNewtype, IndexArray,
    IndexArrayVec,
};

#[test]
fn derive_enum_idx() {
    #[derive(Idx)]
    enum Foo {
        A,
        B,
    }
    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}

#[test]
fn derive_idx_newtype() {
    #[derive(Idx)]
    pub struct FooId(u32);

    let foo = IndexArrayVec::<FooId, i32, 3>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooId::ONE], 1);
}

#[test]
fn derive_enum_omit() {
    #[derive(IdxEnum, Default)]
    #[indexland(omit(Default), omit(Add))]
    pub enum Bar {
        A,
        #[default]
        B,
    }

    impl Add for Bar {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::from_usize(self.into_usize() + rhs.into_usize())
        }
    }

    let foo: EnumIndexArray<Bar, i32> = enum_index_array![
        Bar::A => 1,
        Bar::B => 2,
    ];
    assert_eq!(foo[Bar::default()], 2);
}

#[test]
fn derive_enum_omit_from() {
    #[derive(Idx, Default)]
    #[indexland(omit(From<Self> for usize, Default))]
    pub enum Bar {
        A,
        #[default]
        B,
    }

    impl From<Bar> for usize {
        fn from(value: Bar) -> Self {
            Idx::into_usize(value)
        }
    }

    let foo: EnumIndexArray<Bar, i32> = enum_index_array![
        Bar::A => 1,
        Bar::B => 2,
    ];

    assert_eq!(foo[Bar::default()], 2);
}

#[test]
fn derive_newtype_omit() {
    #[derive(Idx)]
    #[indexland(omit(Add))]
    pub struct FooId(u32);

    impl Add for FooId {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            FooId(self.0 + rhs.0)
        }
    }

    let arr: IndexArray<FooId, FooId, 3> = index_array![FooId::ONE, FooId::ONE, FooId::ONE];
    assert_eq!(arr.into_iter().fold(FooId::ZERO, Add::add), FooId::new(3));
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn bounds_checks_debug_only() {
    #[derive(Idx)]
    struct FooId(u32);
    FooId::from_usize(usize::MAX);
}

#[test]
#[should_panic]
fn bounds_checks_always() {
    #[derive(Idx)]
    #[indexland(bounds_checks = "always")]
    struct FooId(u32);

    FooId::from_usize(usize::MAX);
}

#[test]
fn bounds_checks_never() {
    #[derive(Idx)]
    #[indexland(bounds_checks = "never")]
    struct FooId(u32);

    assert_eq!(FooId::from_usize(u32::MAX as usize + 2).into_usize(), 1);
}

#[test]
fn usize_arith() {
    #[derive(Idx)]
    #[indexland(usize_arith)]
    struct FooId(u32);

    let mut idx = FooId(12);

    idx += 1usize;

    assert_eq!(idx.into_usize(), 13);
}

#[test]
fn usize_compatible() {
    #[derive(Idx)]
    #[indexland(compatible(usize))]
    struct FooId(u32);

    let v: IndexArray<FooId, i32, 3> = index_array![1, 2, 3];

    assert_eq!(v[1], 2);
}

#[test]
fn multi_integer_compatible() {
    #![allow(clippy::unnecessary_cast)]

    #[derive(Idx)]
    #[indexland(compatible(usize, u8))]
    struct FooId(u32);

    let v: IndexArray<FooId, i32, 3> = index_array![1, 2, 3];

    assert_eq!(v[1 as usize], 2);

    assert_eq!(v[2 as u8], 3);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}
