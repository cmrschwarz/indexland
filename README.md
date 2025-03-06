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
use indexland::{IdxNewtype, IndexVec};

#[derive(IdxNewtype)]
struct NodeId(u32);

struct Node<T> {
    prev: NodeId,
    next: NodeId,
    data: T,
};
struct DoublyLinkedList<T> {
    nodes: IndexVec<NodeId, Node<T>>,
}
```

## Enums as Indices
```rust
use indexland::{IdxEnum, IndexArray, EnumIndexArray, index_array};

#[derive(IdxEnum)]
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

// index using enum variants
let my_color = COLOR_MAPPING[PrimaryColor::Red];

// use convenience constants for iteration etc.
assert_eq!(PrimaryColor::COUNT, PrimaryColor::VARIANTS.len());
```

## Support for all common Array Based Collections
- `&IndexSlice<I, T>` wrapping `&[T]`
- `IndexArray<I, T, LEN>` wrapping `[T; LEN]`
- `IndexVec<I, T>` wrapping `Vec<T>`
- `IndexSmallVec<I, T, CAP>` wrapping [`SmallVec<[T; CAP]>`](https://docs.rs/smallvec/latest/smallvec)
  (Optional)
- `IndexArrayVec<I, T, CAP>` based on [`ArrayVec<T, CAP>`](https://docs.rs/arrayvec/latest/arrayvec/)
  (Optional)
- [Serde](https://docs.rs/serde/latest/serde/) support for all Collections

## License
[MIT](../../LICENSE)
