use indexland::{Idx, IndexHashSet, identity_hasher::IdentityHasher, index_hash_set};
#[test]
fn macro_works() {
    let ihs: IndexHashSet<u32, _, IdentityHasher> = index_hash_set![42, 17];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of(&42), Some(0));
    assert_eq!(ihs.as_slice(), &[42, 17]);
}

#[test]
fn macro_works_with_explicit_indices() {
    #[derive(Idx)]
    enum Foo {
        A,
        B,
    }

    let ihs: IndexHashSet<_, _, IdentityHasher> = index_hash_set![
        1 => Foo::B,
        0 => Foo::A
    ];
    assert_eq!(ihs.len(), 2);
    assert_eq!(ihs.get_index_of(&Foo::B), Some(1));
    assert_eq!(ihs.as_slice(), &[Foo::A, Foo::B]);
}

#[test]
fn empty_map_works() {
    let ihs: IndexHashSet<u32, &'static str, IdentityHasher> = index_hash_set![];
    assert_eq!(ihs.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn hasher_deduction_works_for_std() {
    #[derive(Idx)]
    struct Foo(usize);

    let ihs: IndexHashSet<Foo, &str> = index_hash_set![];
    assert!(ihs.is_empty());
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std() {
    #[derive(Idx)]
    struct Foo(usize);

    let ihs: IndexHashSet<Foo, _> = index_hash_set![42,];
    assert_eq!(*ihs.get_index(Foo(0)).unwrap(), 42);
    assert_eq!(ihs.len(), 1);
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std_with_indices() {
    #[derive(Idx)]
    struct Foo(usize);

    let ihs: IndexHashSet<_, _> = index_hash_set![
        Foo(0) => 42
    ];
    assert_eq!(ihs[Foo(0)], 42);
    assert_eq!(ihs.len(), 1);
}

#[test]
fn indexing_works() {
    #[derive(Idx)]
    struct Foo(usize);

    let av: IndexHashSet<Foo, Foo, IdentityHasher> = indexland::index_hash_set![Foo(42)];

    assert_eq!(av[Foo(0)], Foo(42));
}
