use indexland::{index_array, IndexArray};

fn main() {
    const ARR: IndexArray<u32, i32, 3> = index_array! {
        0 => 1,
        0 => 2,
        1 => 3,
    };
}
