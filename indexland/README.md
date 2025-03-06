# Indexland

[![github]](https://github.com/cmrschwarz/indexland/tree/main/crates/indexland)&ensp;
[![github-build]](https://github.com/cmrschwarz/indexland/actions/workflows/ci.yml)&ensp;
[![crates-io]](https://crates.io/crates/indexland)&ensp;
[![msrv]](https://crates.io/crates/indexland)&ensp;
[![docs-rs]](https://docs.rs/indexland)&ensp;

[github]: https://img.shields.io/badge/cmrschwarz/indexland-8da0cb?&labelColor=555555&logo=github
[github-build]: https://github.com/cmrschwarz/indexland/actions/workflows/ci.yml/badge.svg
[crates-io]: https://img.shields.io/crates/v/indexland.svg?logo=rust
[msrv]: https://img.shields.io/crates/msrv/indexland?logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-indexland-66c2a5?logo=docs.rs

Wrappers for common collection types based on newtype indices.
Increased type safety and code readability without runtime overhead.



## Newtype Indices
```rust
# #[cfg(feature = "alloc")] {
use indexland::{Idx, IndexVec};

#[derive(Idx)]
struct NodeId(u32);

struct Node<T> {
    prev: NodeId,
    next: NodeId,
    data: T,
}

struct DoublyLinkedList<T> {
    nodes: IndexVec<NodeId, Node<T>>,
}
# }
```

## Enums as Indices
```rust
use indexland::{enum_index_array, EnumIndexArray, Idx};

#[derive(Idx)]
enum PrimaryColor {
    Red,
    Green,
    Blue,
}

const COLOR_MAPPING: EnumIndexArray<PrimaryColor, u32> = enum_index_array![
    PrimaryColor::Red => 0xFF0000,
    PrimaryColor::Green => 0x00FF00,
    PrimaryColor::Blue => 0x0000FF,
];

let my_color = COLOR_MAPPING[PrimaryColor::Red];
```

## Support for most common Array Based Collections
- [`IndexSlice<I, T>`](crate::IndexSlice)
  wrapping [`&[T]`](std::slice)
- [`IndexArray<I, T, LEN>`](crate::IndexArray)
  wrapping [`[T; LEN]`](std::array)
- [`IndexVec<I, T>`](crate::IndexVec)
  wrapping [`Vec<T>`](alloc::vec::Vec)
- [`IndexVecDeque<I, T>`](crate::IndexVecDeque)
  wrapping[`VecDeque<T>`](std::collections::VecDeque)
- [`IndexSmallVec<I, T, CAP>`](crate::IndexSmallVec)
  wrapping [`SmallVec<[T;CAP]>`](smallvec::SmallVec) (Optional)
- [`IndexArrayVec<I, T, CAP>`](crate::IndexArrayVec)
  wrapping [`ArrayVec<T, CAP>`](arrayvec::ArrayVec) (Optional)
- [`IndexHashMap<I, K, V>`](crate::IndexHashMap)
  wrapping [`IndexMap<K, V>`](indexmap::IndexMap) (Optional)
- [`IndexHashSet<I, T>`](crate::IndexHashSet)
  wrapping [`IndexSet<T>`](indexmap::IndexSet) (Optional)
- [`NonMax<T>`](crate::nonmax) Integer Types for Niche Optimizations (Optional)
- [`serde`](::serde) support for all Collections (Optional)

## License
[MIT](../../LICENSE)
