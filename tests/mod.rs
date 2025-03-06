use derive_more::{Add, AddAssign, Sub, SubAssign};
use indexland::{
    idx_newtype, index_array,
    index_array::{EnumIndexArray, IndexArray},
    index_vec::IndexVec,
    Idx, IdxEnum, IdxNewtype,
};

#[test]
fn derive_enum_idx() {
    #[derive(IdxEnum)]
    enum Foo {
        A,
        B,
    }

    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}

#[test]
fn derive_idx_newtype() {
    #[derive(IdxNewtype)]
    pub struct FooId(u32);

    let foo = IndexVec::<FooId, i32>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooId::ONE], 1);
}

#[test]
fn declarative_idx_newtype() {
    idx_newtype! {
        pub struct FooId(u32);
    }
    idx_newtype! {
        pub struct BarId(u32);
        pub struct BazId(u32);
    }

    let foo = IndexVec::<FooId, i32>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooId::ONE], 1);
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
    impl IdxEnum for Foo {
        const COUNT: usize = 2;
        const VARIANTS: &'static [Self] = &[Foo::A, Foo::B];
        type EnumIndexArray<T> = IndexArray<Self, T, 2>;
    }
    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
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
