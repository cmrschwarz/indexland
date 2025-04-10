use indexland::{index_array, IndexArray};

fn main() {
    let arr: IndexArray<Foo, _, 3> = index_array! {
        0 => 2,
        2 => 3,
    };
}
