use indexland::{index_hash_map, Idx, IndexHashMap, IndexRangeBounds, IndexVec};

#[derive(Idx)]
struct MyId(u32);

fn range_iteration() {
    // example values
    let myvec: IndexVec<MyId, _> = IndexVec::from_iter(0..10);
    let start = MyId(1);
    let end = MyId(3);

    // NOTE: without https://github.com/rust-lang/rust/issues/42168
    // this unfortunately won't compile
    // for i in start..end {
    //     println!("myvec[{i}] = {}", myvec[i]);
    // }

    // you can use this instead:
    // this requires the `IndexRangeBounds` trait to be in scope
    for i in (start..end).index_range() {
        println!("myvec[{i}] = {}", myvec[i]);
    }

    // below are a few helpers to make the version above less neccessary:

    // for full iteration:
    for i in myvec.indices() {
        println!("myvec[{i}] = {}", myvec[i]);
    }

    // for enumerated iteration
    for (i, &v) in myvec.iter_enumerated() {
        println!("myvec[{i}] = {v}");
    }

    // for enumerated iteration over a range
    for (i, &v) in myvec.iter_enumerated_range(start..end) {
        println!("myvec[{i}] = {v}");
    }
}

fn index_hash_map_iteration() {
    let map: IndexHashMap<MyId, &str, i32> = index_hash_map! {
        "foo" => 42,
        "bar" => 1337
    };

    // to iterate over kv pairs and their index
    for (i, (k, v)) in map.iter_enumerated() {
        println!("{i}: myvec[{k}] = {v}");
    }
}

fn main() {
    range_iteration();
    index_hash_map_iteration();
}
