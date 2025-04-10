use crate::integration::idx_manual::{EnumIdxManual2, IdxManual};
use indexland::{identity_hasher::IdentityHasher, index_hash_set, IndexHashSet};

#[test]
fn macro_works() {
    let ihs: IndexHashSet<u32, _, IdentityHasher> = index_hash_set![42, 17];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of(&42), Some(0));
    assert_eq!(ihs.as_slice(), &[42, 17]);
}

#[test]
fn macro_works_with_explicit_indices() {
    let ihs: IndexHashSet<_, _, IdentityHasher> = index_hash_set![
        1 => EnumIdxManual2::B,
        0 => EnumIdxManual2::A
    ];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of(&EnumIdxManual2::B), Some(1));
    assert_eq!(ihs.as_slice(), &[EnumIdxManual2::A, EnumIdxManual2::B]);
}

#[test]
fn empty_map_works() {
    let ihs: IndexHashSet<u32, &'static str, IdentityHasher> = index_hash_set![];
    assert_eq!(ihs.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn hasher_deduction_works_for_std() {
    let ihs: IndexHashSet<IdxManual, &str> = index_hash_set![];
    assert!(ihs.is_empty());
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std() {
    let ihs: IndexHashSet<IdxManual, _> = index_hash_set![42,];
    assert_eq!(*ihs.get_index(IdxManual(0)).unwrap(), 42);
    assert_eq!(ihs.len(), 1);
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std_with_indices() {
    let ihs: IndexHashSet<_, _> = index_hash_set![
        IdxManual(0) => 42
    ];
    assert_eq!(ihs[IdxManual(0)], 42);
    assert_eq!(ihs.len(), 1);
}

#[test]
fn indexing_works() {
    let av: IndexHashSet<IdxManual, IdxManual, IdentityHasher> =
        indexland::index_hash_set![IdxManual(42)];

    assert_eq!(av[IdxManual(0)], IdxManual(42));
}
