use indexland::{idx_newtype, Idx, IndexArray};

#[test]
fn declarative_idx_newtype() {
    idx_newtype! {
        pub struct FooId(u32);
    }
    idx_newtype! {
        pub struct BarId(u32);
        pub struct BazId(u32);
    }

    let foo = IndexArray::<FooId, i32, 3>::new([0, 1, 2]);
    assert_eq!(foo[FooId::ONE], 1);

    let bar = IndexArray::<BazId, i32, 1>::new([42]);
    assert_eq!(bar[BazId::ZERO], 42);
}
