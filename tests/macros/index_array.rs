use std::boxed::Box;

use indexland::{enum_index_array, index_array, EnumIndexArray, Idx};

#[derive(Idx)]
enum EnumAB {
    A,
    B,
}

#[derive(Idx)]
enum EnumABC {
    A,
    B,
    C,
}

#[test]
fn enum_idx_array_macro() {
    const FOO: EnumIndexArray<EnumAB, i32> = index_array![
        EnumAB::A => 1,
        EnumAB::B => 2,
    ];

    assert_eq!(FOO[EnumAB::B], 2);
}

#[test]
fn enum_idx_array_macro_non_copy() {
    let foo: EnumIndexArray<EnumAB, Box<i32>> = index_array![
        EnumAB::A => Box::new(1),
        EnumAB::B => Box::new(2),
    ];

    assert_eq!(*foo[EnumAB::B], 2);
}

#[test]
fn nested_enum_idx_array() {
    // make sure type inference works for nested index_array! macros
    let foo: EnumIndexArray<EnumAB, EnumIndexArray<EnumABC, i32>> = index_array![
        EnumAB::A => index_array![
            EnumABC::A => 1,
            EnumABC::B => 2,
            EnumABC::C => 3
        ],
        EnumAB::B => index_array![
            EnumABC::A => 4,
            EnumABC::B => 5,
            EnumABC::C => 6
        ],
    ];

    assert_eq!(foo[EnumAB::B][EnumABC::A], 4);
}

#[test]
#[should_panic(expected = "index `1` was initialized twice")]
fn enum_index_array_macro_works() {
    let _: EnumIndexArray<EnumAB, i32> = enum_index_array![
        EnumAB::B => 1,
        EnumAB::B => 2,
    ];
}
