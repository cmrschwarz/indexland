use indexland::{IndexVec, index_vec};

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
fn array_like_from_non_const() {
    let dyn_val = std::env::args().len().max(2) + 10;
    let sv: IndexVec<u32, i32> = index_vec![dyn_val.try_into().unwrap(); dyn_val];
    assert_eq!(sv.len(), dyn_val);
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
