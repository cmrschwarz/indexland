# `Indexland`

[![github]](https://github.com/cmrschwarz/typeline/tree/main/crates/indexland)&ensp;
[![github-build]](https://github.com/cmrschwarz/typeline/actions/workflows/ci.yml)&ensp;
[![crates-io]](https://crates.io/crates/indexland)&ensp;
[![msrv]](https://crates.io/crates/indexland)&ensp;
[![docs-rs]](https://docs.rs/indexland)&ensp;

[github]: https://img.shields.io/badge/cmrschwarz/typeline-8da0cb?&labelColor=555555&logo=github
[github-build]: https://github.com/cmrschwarz/typeline/actions/workflows/ci.yml/badge.svg
[crates-io]: https://img.shields.io/crates/v/indexland.svg?logo=rust
[msrv]: https://img.shields.io/crates/msrv/indexland?logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-indexland-66c2a5?logo=docs.rs

Wrappers for common collection types based on newtype indices.
Increased type safety and code readability without runtime overhead.

Part of the [Typeline](https://github.com/cmrschwarz/typeline) project,
not ready for public use yet.


## Newtype Indices
```rust
use indexland::{NewtypeIdx, index_vec::IndexVec};

#[derive(NewtypeIdx)]
struct NodeId(u32);

struct Node<T> {
    prev: NodeId,
    next: NodeId,
    data: T,
};
struct DoublyLinkedList<T> {
    nodes: IndexVec<NodeId, Node>,
}
```

## Enums as Indices
```rust
use indexland::{EnumIdx, index_array::{IndexArray, EnumIndexArray}};

#[derive(EnumIdx)]
enum PrimaryColor{
    Red,
    Green,
    Blue
};

const COLOR_MAPPING: EnumIndexArray<PrimaryColor, u32> = index_array![
    PrimaryColor::Red => 0xFF0000,
    PrimaryColor::Green => 0x00FF00,
    PrimaryColor::Blue => 0x0000FF,
];

let my_color = COLOR_MAPPING[PrimaryColor::Red];
```

## Optional Integration with Popular Crates
- `IndexSmallVec` based on [SmallVec](https://docs.rs/smallvec/latest/smallvec)
- `IndexArrayVec` based on [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/)
- [Serde](https://docs.rs/serde/latest/serde/) support for all Collections

## License
[MIT](../../LICENSE)
