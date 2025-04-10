use core::hash::{BuildHasher, Hasher};

use indexland::{index_hash_map, Idx, IndexHashMap};

struct OneByteHasher(u8);

impl Hasher for OneByteHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        if let Some(last) = bytes.last() {
            self.0 = *last;
        }
    }
}

impl BuildHasher for OneByteHasher {
    type Hasher = OneByteHasher;

    fn build_hasher(&self) -> Self::Hasher {
        OneByteHasher(0)
    }
}

#[test]
#[cfg(feature = "std")]
fn macro_works() {
    let ihm: IndexHashMap<u32, &'static str, i32> = index_hash_map![
        "foo" => 42,
        "bar" => 12,
    ];
    assert_eq!(ihm.len(), 2);
    assert_eq!(ihm["foo"], 42);
    assert_eq!(ihm.values().sum::<i32>(), 54);
}

#[test]
#[cfg(feature = "std")]
fn empty_map_works() {
    let ihm: IndexHashMap<u32, &'static str, i32> = index_hash_map![];
    assert_eq!(ihm.len(), 0);
}

#[test]
#[cfg(feature = "std")]
fn indexing_works() {
    #[derive(Idx)]
    struct FooId(u32);

    let av: IndexHashMap<FooId, FooId, FooId> = indexland::index_hash_map![
        FooId(3) => FooId(42)
    ];

    assert_eq!(av[&FooId(3)], FooId(42));
}
