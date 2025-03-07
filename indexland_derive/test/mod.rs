#[test]
fn derive_enum_idx() {
    #[derive(IdxEnum)]
    enum Foo {
        A,
        B,
    }

    const FOO: EnumIndexArray<Foo, i32> = IndexArray::new([0, 1]);

    assert_eq!(FOO[Foo::B], 1);
}

#[test]
fn derive_idx_newtype() {
    #[derive(IdxNewtype)]
    pub struct FooId(u32);

    let foo = IndexArrayVec::<FooId, i32, 3>::from_iter([0, 1, 2]);
    assert_eq!(foo[FooId::ONE], 1);
}
