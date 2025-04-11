use indexland::{identity_hasher::IdentityHasher, index_hash_map, Idx, IndexHashMap};

#[test]
fn macro_works() {
    let ihm: IndexHashMap<u32, _, _, IdentityHasher> = index_hash_map![
        -1 => 42,
        -2 => 12,
    ];
    assert_eq!(ihm.len(), 2);
    assert_eq!(ihm[&-1], 42);
    assert_eq!(ihm.values().sum::<i32>(), 54);
}

#[test]
fn empty_map_works() {
    let ihm: IndexHashMap<u32, &'static str, i32, IdentityHasher> = index_hash_map![];
    assert_eq!(ihm.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn hasher_deduction_works_for_std() {
    let ihm: IndexHashMap<u32, &str, i32> = index_hash_map![];
    assert!(ihm.is_empty());
}

#[test]
#[cfg(feature = "std")]
fn deduction_works_for_std() {
    let ihm: IndexHashMap<u32, _, _> = index_hash_map![
        "foo" => 42,
    ];
    assert_eq!(*ihm.get_index(0).unwrap().1, 42);
    assert_eq!(ihm.len(), 1);
}

#[test]
fn indexing_works() {
    #[derive(Idx)]
    struct Foo(usize);

    let av: IndexHashMap<Foo, Foo, Foo, IdentityHasher> = indexland::index_hash_map![
        Foo(3) => Foo(42)
    ];

    assert_eq!(av[&Foo(3)], Foo(42));
}
