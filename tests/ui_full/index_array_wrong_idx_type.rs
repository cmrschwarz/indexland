use indexland::{index_array, Idx, IndexArray};

fn main() {
    #[derive(Idx)]
    struct MyIdx(u32);

    #[derive(Idx)]
    struct MyIdx2(u32);

    let arr: IndexArray<MyIdx, u8, 3> = index_array![1, 2, 3];

    println!("{}", arr[MyIdx2(42)]);
}
