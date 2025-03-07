use std::ops::Add;

use indexland::{
    enum_index_array, EnumIndexArray, Idx, IdxEnum, IndexArray, IndexArrayVec,
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
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}
