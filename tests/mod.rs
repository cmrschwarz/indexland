use indexland::{index_array::IndexArray, EnumIdx};
use indexland_derive::make_enum_idx;

#[make_enum_idx]
enum Foo {
    A,
    B,
}

const FOO: IndexArray<Foo, i32, { Foo::COUNT }> = IndexArray::new([1, 2]);

#[test]
fn asdf() {
    println!("{:?}", FOO[Foo::A])
}
