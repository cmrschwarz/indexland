use indexland::{index_small_vec, IndexSmallVec};

#[test]
fn macro_works() {
    let _sv: IndexSmallVec<u32, i32, 3> = index_small_vec![1, 2, 3];
}
