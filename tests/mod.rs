use indexland::index_array::{EnumIndexArray, IndexArray};
use indexland_derive::make_enum_idx;

#[make_enum_idx]
enum Foo {
    A,
    B,
}

const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([1, 2]);

#[test]
fn asdf() {
    println!("{:?}", FOO[Foo::A])
}
