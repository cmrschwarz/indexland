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

- All array based std::collections in one place through a single `Idx` trait.

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

- Every wrapper has an escape hatch
  to the underlying collection, aswell as bidirectional [`From`](core::convert::From)
  implementations.

- First class embedded support though `#[no_std]` and even optional `alloc`.

- [`Idx`](crate::Idx) compatible [`NonMax<T>`](crate::nonmax) for [Niche Optimizations](https://doc.rust-lang.org/std/option/index.html#representation).

- [`serde`](::serde) implementations for all Collections.

- All crate dependencies optional through feature flags.

## FAQ

### Why?
Using indices into collections instead of references or
smart pointers is a powerful idiom popularized by
[Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design).
Many places make use this pattern, including
[the Rust Compiler itself](https://github.com/rust-lang/rust/blob/2b285cd5f0877e30ad1d83e04f8cc46254e43391/compiler/rustc_index/src/vec.rs#L40).

The pattern can solve many borrow checker issues
while simultaneously *increasing* performance.
They frequently reduce allocations, lower the memory usage and increase
data locality.

When using this pattern heavily in Rust today there are a few issues though.
The common approach is to use `type NodeId = usize` to denote different index
types, but this leaves two big things to be desired:

  1. Type Safety: Type aliases are not unique types.
     It's very easy to accidentally use the wrong index or the wrong
     container. Indices are essentially relative pointers. Using the same type
     for all of them is like writing a C program using exclusively `void*`.
     It is antithetical to robustness and fearless refactoring capabilities
     that are usually enabled by Rust's strong type system.

  2. Readability: Container type definitions don't tell us what index
     should be used to access them. When structs contain multiple collections
     this becomes hard to read quickly.

Newtypes indices elegantly solve both of these issues.

### Why not use [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
1.  Indexland offers all common collections in one place,
    **using the same `Idx` trait**. Sometimes the same index type is used
    for multiple data structures. Sometimes you want to switch from a `Vec`
    to a `VecDeque`.

2.  We deliberately **don't** implement
    `Index<usize> for IndexSlice` and `Add<usize> for Idx`,
    as they compromizes type safety. Opt-in support is availabe
    via [`#[indexland(usize_arith)]`](indexland_derive::Idx#attributes).

3.  Our `Idx` derivation syntax is much cleaner than `index_vec`'s
    [`define_index_newtype!`](https://docs.rs/index_vec/latest/index_vec/macro.define_index_type.html).

### Is there a runtime cost to this?
There's minimal overhead. The core wrapper functions are
marked `#[inline(always)]` to reliably eliminate them, even in debug mode.

Type conversions follow the same rules as Rust integer overflow.
Overflows will panic in debug mode and wrap in release.
Newtypes can customize this via
[`#[indexland(bounds_checks = "..")]`](crate::indexland_derive::Idx#indexlandbounds_checks--),
or bypass it through [`into_usize_unchecked`](crate::idx::Idx::into_usize_unchecked).


## License
Licensed under the terms of either the [Apache License Version 2.0](./LICENSE-APACHE), or the [MIT License](./LICENSE-MIT) at your option.
