use indexland::{enum_index_array, index_array, EnumIndexArray};

use crate::integration::idx_manual::{EnumIdxManual2, EnumIdxManual3};

#[test]
fn enum_idx_array_macro() {
    const FOO: EnumIndexArray<EnumIdxManual2, i32> = index_array![
        EnumIdxManual2::A => 1,
        EnumIdxManual2::B => 2,
    ];

    assert_eq!(FOO[EnumIdxManual2::B], 2);
}

#[test]
fn enum_idx_array_macro_non_copy() {
    let foo: EnumIndexArray<EnumIdxManual2, Box<i32>> = index_array![
        EnumIdxManual2::A => Box::new(1),
        EnumIdxManual2::B => Box::new(2),
    ];

    assert_eq!(*foo[EnumIdxManual2::B], 2);
}

#[test]
fn nested_enum_idx_array() {
    // make sure type inference works for nested index_array! macros
    let foo: EnumIndexArray<EnumIdxManual2, EnumIndexArray<EnumIdxManual3, i32>> = index_array![
        EnumIdxManual2::A => index_array![
            EnumIdxManual3::A => 1,
            EnumIdxManual3::B => 2,
            EnumIdxManual3::C => 3
        ],
        EnumIdxManual2::B => index_array![
            EnumIdxManual3::A => 4,
            EnumIdxManual3::B => 5,
            EnumIdxManual3::C => 6
        ],
    ];

    assert_eq!(foo[EnumIdxManual2::B][EnumIdxManual3::A], 4);
}

#[test]
#[should_panic(expected = "index `1` was initialized twice")]
fn enum_index_array_macro_works() {
    let _: EnumIndexArray<EnumIdxManual2, i32> = enum_index_array![
        EnumIdxManual2::B => 1,
        EnumIdxManual2::B => 2,
    ];
}
