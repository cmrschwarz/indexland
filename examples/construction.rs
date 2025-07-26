use indexland::{Idx, IndexVec, index_vec};

#[derive(Idx)]
struct MyIdx(u32);

fn inferred_from_use() {
    // Rust is able to infer the index type for this.
    let vec = index_vec![1, 2, 3];

    println!("{}", vec[MyIdx(2)]);
}

fn explicit() {
    // Rust is able to infer the index type for this.
    let vec: IndexVec<MyIdx, _> = index_vec![1, 2, 3];

    println!("{}", vec[MyIdx(2)]);
}

fn main() {
    inferred_from_use();
    explicit();
}
