use indexland::{IndexVecDeque, index_vec_deque};

#[test]
fn basic() {
    let sv: IndexVecDeque<u32, i32> = index_vec_deque![1, 2, 3];
    assert_eq!(sv.len(), 3);
    assert_eq!(sv.iter().sum::<i32>(), 6);
}

#[test]
fn array_like() {
    let sv: IndexVecDeque<u32, i32> = index_vec_deque![42; 10];
    assert_eq!(sv.len(), 10);
    assert_eq!(sv.iter().sum::<i32>(), 420);
}

#[test]
fn array_like_from_non_const() {
    let dyn_val = std::env::args().len().max(2) + 10;
    let sv: IndexVecDeque<u32, i32> = index_vec_deque![dyn_val.try_into().unwrap(); dyn_val];
    assert_eq!(sv.len(), dyn_val);
}

#[test]
fn explicit_indices() {
    let sv: IndexVecDeque<u32, i32> = index_vec_deque![
        0 => 1,
        1 => 2,
        2 => 3
    ];
    assert_eq!(sv.len(), 3);
    assert_eq!(sv[1], 2);
}

#[test]
fn empty() {
    let sv: IndexVecDeque<u32, i32> = index_vec_deque![];
    assert_eq!(sv.len(), 0);
}
