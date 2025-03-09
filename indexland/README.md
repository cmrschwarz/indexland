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
  wrapping [`smallvec::SmallVec<[T; CAP]>`](smallvec::SmallVec) (optional)
- [`IndexArrayVec<I, T, CAP>`](crate::IndexArrayVec)
  wrapping [`arrayvec::ArrayVec<T, CAP>`](arrayvec::ArrayVec) (optional)
- [`IndexHashMap<I, K, V>`](crate::IndexHashMap)
  wrapping [`indexmap::IndexMap<K, V>`](indexmap::IndexMap) (optional)
- [`IndexHashSet<I, T>`](crate::IndexHashSet)
  wrapping [`indexmap::IndexSet<T>`](indexmap::IndexSet) (optional)


## Additional Features

- Cheaply convert to/from underlying collections when necessary.

  Never feel boxed in by this dependency.

- All basic integer types implement `Idx`.

  No complaints if your main usecase is `IndexVec<u32, T>`.

- First class embedded support though `#[no_std]` and even optional `alloc`.

- [`Idx`](crate::Idx) compatible [`NonMax<T>`](crate::nonmax) Integer Types for Niche Optimizations

- [`serde`](::serde) implementations for all Collections.

- All crate dependencies optional through feature flags.

## FAQ

### Why?
Using indices instead of references or smart pointers like `Box` or `Rc`
is an incredibly powerful idiom, especially in Rust.
They avoid borrow checker issues around cycles and can even increase
performance due to their smaller size, better locality and fewer allocations.

Code that makes heavy use of this pattern can suffer in readability though,
because instead of `&Node` the code now just reads `usize`. Using type aliases like
`type NodeId = usize` solves half of this problem, but does not provide type
safety, as `NodeId` is fundamentally still `usize`. This can lead do subtle
bugs, especially for functions that take in multiple ids as parameters.
Using newtypes solves these issues, but introduces ergonomic problems.
This crate is an attempt to solve these.


### Why not use [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
- Ergonomic newtype indices require support for all standard collections in one place.
  Sometimes an index is used for multiple datastructures.
  Sometimes you want to switch from a `Vec` to a `VecDeque`.

- Unlike `index_vec`, we don't implicitly implement `Add<usize> for Idx`,
  which we believe breaks the type safety that's the whole point of this.
  We support it as an opt-in configuration though.

- Our `Idx` derivation syntax is significantly nicer.

### Is there a runtime cost to this?
- There is very litle runtime overhead. The core index wrapper functions are marked `#[inline(always)]`,
so the compiler can reliably eliminate them, even in debug mode.

- By default, index conversions that might overflow will be bounds checked.
  This only affects index types smaller than `usize`.
  It can also be fully disabled through `#[indexland(disable_bounds_checks)]`,
  in which case the indices will silently wrap around, just like
  `my_usize as u32` would.



## License
[MIT](../../LICENSE)
