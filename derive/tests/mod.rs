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
    #[indexland(arith_compat(usize))]
    struct FooId(u32);

    let mut idx = FooId(12);

    idx += 1usize;

    assert_eq!(idx.into_usize(), 13);
}

#[test]
fn usize_compatible() {
    #[derive(Idx)]
    #[indexland(idx_compat(usize))]
    struct FooId(u32);

    let v: IndexArray<FooId, i32, 3> = index_array![1, 2, 3];

    assert_eq!(v[1], 2);
}

#[test]
fn multi_integer_compatible() {
    #![allow(clippy::unnecessary_cast)]

    #[derive(Idx)]
    #[indexland(idx_compat(usize, u8))]
    struct FooId(u32);

    let v: IndexArray<FooId, i32, 3> = index_array![1, 2, 3];

    assert_eq!(v[1 as usize], 2);

    assert_eq!(v[2 as u8], 3);
}

#[test]
fn enum_idx_array_macro() {
    #[derive(IdxEnum)]
    enum Foo {
        A,
        B,
        C,
    }

    const FOO: EnumIndexArray<Foo, i32> = index_array![
        Foo::A => 1,
        Foo::B => 2,
        Foo::C => 3,
    ];

    assert_eq!(FOO[Foo::B], 2);
}

#[test]
fn enum_idx_array_macro_non_copy() {
    #[derive(IdxEnum)]
    enum Foo {
        A,
        B,
        C,
    }

    let foo: EnumIndexArray<Foo, Box<i32>> = index_array![
        Foo::A => Box::new(1),
        Foo::B => Box::new(2),
        Foo::C => Box::new(3),
    ];

    assert_eq!(*foo[Foo::B], 2);
}

#[test]
fn nested_enum_idx_array() {
    #[derive(IdxEnum)]
    enum Foo {
        A,
        B,
    }

    #[derive(IdxEnum)]
    enum Bar {
        X,
        Y,
    }

    // make sure type inference works for nested index_array! macros
    let foo: EnumIndexArray<Foo, EnumIndexArray<Bar, i32>> = index_array![
        Foo::A => index_array![Bar::X => 1, Bar::Y => 2],
        Foo::B => index_array![Bar::X => 3, Bar::Y => 4],
    ];

    assert_eq!(foo[Foo::B][Bar::Y], 4);
}

#[test]
pub fn wrapping_add_on_enum() {
    metamatch::quote! {
        #[derive(Idx)]
        enum E256 {
            [<for x in 0..256>]
                [< ident("_" + str(x))>],
            [</for>]
        }
    }
    assert_eq!(E256::_200.wrapping_add(E256::_100), E256::_44);
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}
