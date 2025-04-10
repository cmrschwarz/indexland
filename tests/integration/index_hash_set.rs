use crate::integration::{idx_manual::IdxManual, OneByteHasher};
use indexland::{index_hash_set, IndexHashSet};

#[test]
fn macro_works() {
    let ihs: IndexHashSet<u32, _, OneByteHasher> =
        index_hash_set!["foo", "bar"];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of("foo"), Some(0));
    assert_eq!(ihs.as_slice(), &["foo", "bar"]);
}

#[test]
fn macro_works_with_explicit_indices() {
    let ihs: IndexHashSet<_, _, OneByteHasher> = index_hash_set![
        1 => "bar",
        0 => "foo"
    ];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of("foo"), Some(0));
    assert_eq!(ihs.as_slice(), &["foo", "bar"]);
}

#[test]
fn empty_map_works() {
    let ihs: IndexHashSet<u32, &'static str, OneByteHasher> =
        index_hash_set![];
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
    assert_eq!(*ihs.get_index(IdxManual(0)).unwrap(), 42);
    assert_eq!(ihs.len(), 1);
}

#[test]
fn indexing_works() {
    let av: IndexHashSet<IdxManual, IdxManual, OneByteHasher> =
        indexland::index_hash_set![IdxManual(42)];

    assert_eq!(av[IdxManual(0)], IdxManual(42));
}
