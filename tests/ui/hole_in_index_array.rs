use indexland::{index_array, IndexArray};

fn main() {
    let arr: IndexArray<u32, u32, 3> = index_array! {
        0 => 2,
        2 => 3,
    };
}
