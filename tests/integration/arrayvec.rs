use indexland::{index_array_vec, Idx, IndexArrayVec};

#[test]
fn macro_works() {
    let iav: IndexArrayVec<u32, i32, 3> = index_array_vec![1, 2, 3];
    assert_eq!(iav.len(), 3);
    assert_eq!(iav.iter().sum::<i32>(), 6);
}

#[test]
fn array_like_macro_works() {
    let iav: IndexArrayVec<u32, i32, 10> = index_array_vec![42; 10];
    assert_eq!(iav.len(), 10);
    assert_eq!(iav.iter().sum::<i32>(), 420);
}

#[test]
fn empty_array_works_empty_cap() {
    let iav: IndexArrayVec<u32, i32, 0> = index_array_vec![];
    assert_eq!(iav.len(), 0);
}

#[test]
fn empty_array_works_non_empty_cap() {
    let iav: IndexArrayVec<u32, i32, 5> = index_array_vec![];
    assert_eq!(iav.len(), 0);
}

#[test]
fn shorter_array_works() {
    let iav: IndexArrayVec<u32, i32, 10> = index_array_vec![1, 2, 3];
    assert_eq!(iav.len(), 3);
}

#[test]
fn indexing_works() {
    #[derive(Idx)]
    struct FooId(u32);

    let av: IndexArrayVec<FooId, _, 5> =
        indexland::index_array_vec![0, 1, 2, 3, 4];

    assert_eq!(av[FooId(2)], 2);
}

// TODO: allow sizes other than the array cap, see #5
