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
- Strongly typed collection indices for better type safety and readability.

- All array based `std::collections` wrapped in one place through a single
  [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html) trait.

- All underlying APIs faithfully adapted for `Idx` types.

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
- [`IndexSlice<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexSlice.html)
  wrapping [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html)
- [`IndexArray<I, T, N>`](https://docs.rs/indexland/latest/indexland/struct.IndexArray.html)
  wrapping [`[T; N]`](https://doc.rust-lang.org/std/primitive.array.html)
  (Convenience alias
  [`EnumIndexArray<E, T>`](https://docs.rs/indexland/latest/indexland/type.EnumIndexArray.html))
- [`IndexVec<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVec.html)
  wrapping [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`IndexVecDeque<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVecDeque.html)
  wrapping[`VecDeque<T>`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [`IndexSmallVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexSmallVec.html)
  wrapping [`smallvec::SmallVec<[T; CAP]>`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html) (optional)
- [`IndexArrayVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexArrayVec.html)
  wrapping [`arrayvec::ArrayVec<T, CAP>`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) (optional)
- [`IndexHashMap<I, K, V>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashMap.html)
  wrapping [`indexmap::IndexMap<K, V>`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html) (optional)
- [`IndexHashSet<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashSet.html)
  wrapping [`indexmap::IndexSet<T>`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html) (optional)


## Additional Features

- Every wrapper has an escape hatch
  to the underlying collection, aswell as bidirectional [`From`](core::convert::From)
  implementations.

- First class embedded support though
  [`#[no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html)
  and even optional
  [`alloc`](https://doc.rust-lang.org/core/alloc/index.html).

- [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html) compatible
  [`NonMax<T>`](https://docs.rs/indexland/latest/indexland/struct.NonMax.html) for
  [Niche Optimizations](https://doc.rust-lang.org/std/option/index.html#representation).

- [`serde`](https://serde.rs/) implementations for all Collections.

- All crate dependencies optional through feature flags.

## FAQ

### Why?
Using indexed collections instead of references or
smart pointers is a powerful idiom popularized by
[Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design),
and even used by
[the Rust Compiler itself](https://github.com/rust-lang/rust/blob/2b285cd5f0877e30ad1d83e04f8cc46254e43391/compiler/rustc_index/src/vec.rs#L40).
The pattern solves many borrow checker issues and often increases performance.
Indexed collections can reduce allocations and memory usage while increasing
data locality.

Heavy use of this pattern can cause issues though. The standard approach is to
use `type NodeId = usize`, but this negatively affects:

  1. Type Safety: Type aliases are not unique types.
     This makes it easy to accidentally use the wrong index or container
     without getting any compiler errors. One of Rust's main strenghts is to
     enable fearless refactoring because the compiler will guide you through it.
     Non-newtype type aliases make that a lot harder.

  2. Readability: Container type definitions don't tell us what index
     should be used to access them. When structs contain multiple collections
     this becomes hard to read quickly.

Newtypes elegantly solve both of these issues.

### Why not use [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
1.  Indexland offers all common collections in one place,
    **using the same [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html) trait**. Sometimes the same index type is used
    for multiple data structures. Sometimes you want to switch from a [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
    to a [`VecDeque<T>`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html).

2.  We deliberately **don't** implement
    `Index<usize> for IndexSlice` and `Add<usize> for Idx`,
    as they compromize type safety. Opt-in support is availabe
    via [`#[indexland(usize_arith)]`]([indexland_derive::Idx#](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#)indexlandusize_arith).

3.  Our
    [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html)
    derivation syntax is much cleaner than
    [`index_vec`'s `define_index_newtype!`](https://docs.rs/index_vec/latest/index_vec/macro.define_index_type.html).

### Is there a runtime cost to this?
There's minimal overhead. The core wrapper functions are
marked [`#[inline(always)]`](https://nnethercote.github.io/perf-book/inlining.html) to reliably eliminate them, even in debug mode.

Type conversions follow the same rules as Rust integer overflow.
Overflows will panic in debug mode and wrap in release.
Newtypes can customize this via
[`#[indexland(bounds_checks = "..")]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandbounds_checks--),
or bypass it through
[`into_usize_unchecked`](https://docs.rs/indexland/latest/indexland/trait.Idx.html#tymethod.into_usize).


## License
Licensed under the terms of either the [Apache License Version 2.0](./LICENSE-APACHE), or the [MIT License](./LICENSE-MIT) at your option.
