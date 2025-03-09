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

Newtype Index Support for Rust Collection Types.

## Features
- Strongly typed indices prevent accidental mix-ups at compile time.

  Let's not make `usize` be the new `void*` !

- Readable, self-documenting code through explicit indexing semantics.

  No more ```// indexed by `NodeId` ``` comments.

- Underlying APIs available and working with `Idx` Types.

  No need to learn a new collections API.


## Examples
### Newtype Indices
```rust
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
```

### Enums as Indices
```rust
use indexland::{Idx, EnumIndexArray, enum_index_array};

#[derive(Idx)]
enum Status { Idle, Running, Done }

const STATUS_MESSAGE: EnumIndexArray<Status, &str> = enum_index_array![
    Status::Idle => "Waiting for input...",
    Status::Running => "Processing your request...",
    Status::Done => "Operation complete!",
];

let message = STATUS_MESSAGE[Status::Running];
```

## Indexable Collection Wrappers
- [`IndexSlice<I, T>`](crate::IndexSlice)
  wrapping [`&[T]`](std::slice)
- [`IndexArray<I, T, LEN>`](crate::IndexArray)
  wrapping [`[T; LEN]`](std::array) (Convenience alias [`EnumIndexArray<E, T>`](crate::EnumIndexArray))
- [`IndexVec<I, T>`](crate::IndexVec)
  wrapping [`Vec<T>`](alloc::vec::Vec)
- [`IndexVecDeque<I, T>`](crate::IndexVecDeque)
  wrapping[`VecDeque<T>`](std::collections::VecDeque)
- [`IndexSmallVec<I, T, CAP>`](crate::IndexSmallVec)
  wrapping [`SmallVec<[T;CAP]>`](smallvec::SmallVec)
- [`IndexArrayVec<I, T, CAP>`](crate::IndexArrayVec)
  wrapping [`ArrayVec<T, CAP>`](arrayvec::ArrayVec)
- [`IndexHashMap<I, K, V>`](crate::IndexHashMap)
  wrapping [`IndexMap<K, V>`](indexmap::IndexMap)
- [`IndexHashSet<I, T>`](crate::IndexHashSet)
  wrapping [`IndexSet<T>`](indexmap::IndexSet)


## Additional Features

- Cheaply convert to/from underlying collections when necessary.

  Never feel boxed in by this dependency.

- All basic integer types implement `Idx`.

  No complaints if your main usecase is `IndexVec<u32, T>`.

- First class embedded support though `#[no_std]` and even optional `alloc`.

- [`Idx`](crate::Idx) compatible [`NonMax<T>`](crate::nonmax) Integer Types for Niche Optimizations

- [`serde`](::serde) implementations for all Collections.

- All crate dependencies optional through feature flags.


## License
[MIT](../../LICENSE)
