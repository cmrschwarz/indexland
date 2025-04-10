use indexland::{index_hash_map, IndexHashMap};

use crate::integration::{idx_manual::IdxManual, OneByteHasher};

#[test]
fn macro_works() {
    let ihm: IndexHashMap<u32, _, _, OneByteHasher> = index_hash_map![
        "foo" => 42,
        "bar" => 12,
    ];
    assert_eq!(ihm.len(), 2);
    assert_eq!(ihm["foo"], 42);
    assert_eq!(ihm.values().sum::<i32>(), 54);
}

#[test]
fn empty_map_works() {
    let ihm: IndexHashMap<u32, &'static str, i32, OneByteHasher> =
        index_hash_map![];
    assert_eq!(ihm.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn hasher_deduction_works_for_std() {
    let ihm: IndexHashMap<IdxManual, &str, i32> = index_hash_map![];
    assert!(ihm.is_empty());
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std() {
    let ihm: IndexHashMap<IdxManual, _, _> = index_hash_map![
        "foo" => 42,
    ];
    assert_eq!(*ihm.get_index(IdxManual(0)).unwrap().1, 42);
    assert_eq!(ihm.len(), 1);
}

#[test]
fn indexing_works() {
    let av: IndexHashMap<IdxManual, IdxManual, IdxManual, OneByteHasher> = indexland::index_hash_map![
        IdxManual(3) => IdxManual(42)
    ];

    assert_eq!(av[&IdxManual(3)], IdxManual(42));
}
