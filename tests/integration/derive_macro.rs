use indexland::{index_array, index_array::EnumIndexArray, Idx, IdxEnum};

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
