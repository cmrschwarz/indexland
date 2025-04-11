use indexland::{index_vec, IndexVec};

#[test]
fn basic() {
    let sv: IndexVec<u32, i32> = index_vec![1, 2, 3];
    assert_eq!(sv.len(), 3);
    assert_eq!(sv.iter().sum::<i32>(), 6);
}

#[test]
fn array_like() {
    let sv: IndexVec<u32, i32> = index_vec![42; 10];
    assert_eq!(sv.len(), 10);
    assert_eq!(sv.iter().sum::<i32>(), 420);
}

#[test]
fn explicit_indices() {
    let sv: IndexVec<u32, i32> = index_vec![
        0 => 1,
        1 => 2,
        2 => 3
    ];
    assert_eq!(sv.len(), 3);
    assert_eq!(sv[1], 2);
}

#[test]
fn empty() {
    let sv: IndexVec<u32, i32> = index_vec![];
    assert_eq!(sv.len(), 0);
}
