use indexland::{index_array_vec, IndexArrayVec};

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
fn empty_array_works() {
    let iav: IndexArrayVec<u32, i32, 0> = index_array_vec![];
    assert_eq!(iav.len(), 0);
}

// TODO: allow sizes other than the array cap, see #5
