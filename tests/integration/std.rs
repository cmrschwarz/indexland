use indexland::IndexRangeBounds;

#[test]
fn range_addressing() {
    use indexland::{index_vec, Idx};
    #[derive(Idx)]

    struct MyId(u32);
    let myvec = index_vec![0, 1, 2, 3, 4, 5];

    // preferred way of doing index based iteration
    for (i, &v) in myvec.iter_enumerated_range(MyId(1)..MyId(3)) {
        println!("myvec[{i}] = {v}");
    }

    // alternative for some lifetime situations
    for i in myvec.indices().skip(1).take(2) {
        println!("myvec[{i}] = {}", myvec[i]);
    }

    // version using a the `IndexRange` conversions if you need it:
    for i in (MyId(0)..MyId(3)).index_range() {
        println!("myvec[{i}] = {}", myvec[i]);
    }
}
