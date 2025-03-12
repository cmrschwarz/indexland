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

- Underlying APIs faithfully wrapped and adapted for `Idx` Types.

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

- Every wrapper has an escape hatch function to mutably access the underlying
  collection, aswell as bidirectional [`From`](core::convert::From) implementations.
  Never feel boxed in by this dependency.

- All basic integer types implement [`Idx`](crate::Idx).

  No complaints if your main usecase for this crate is `IndexVec<u32, T>`.

- First class embedded support though `#[no_std]` and even optional `alloc`.

- [`Idx`](crate::Idx) compatible [`NonMax<T>`](crate::nonmax) Integer Types
  for those sweet sweet [Niche Optimizations](https://doc.rust-lang.org/std/option/index.html#representation).

- [`serde`](::serde) implementations for all Collections.

- All crate dependencies optional through feature flags.

## FAQ

### Why?
Using indices into collections instead of references or
smart pointers is an incredibly powerful idiom popularized by
[Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design).

Many places make use this pattern, including
[the Rust Compiler itself](https://github.com/rust-lang/rust/blob/2b285cd5f0877e30ad1d83e04f8cc46254e43391/compiler/rustc_index/src/vec.rs#L40).

In Rust in particular, indices can be a fantastic way to avoid most borrow
checker issues while simultaneously *increasing* performance.
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

Using newtypes for the indices and adding them as generic parameters to
the container types elegantly solves both of these issues.

### Why not use [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
The goal of `indexland` is to offer all the most common array based collections
in a single place, **using the same `Idx` trait**.
Sometimes the same index type is used for multiple data structures.
Sometimes you want to switch from a `Vec` to a `VecDeque`.
Doing so is not possible with single container wrappers like `index_vec`.

Unlike `index_vec`, we don't implicitly implement `Add<usize> for Idx`,
which partly breaks the type safety that's the whole point of this.
We do offer opt-in support for it ([`#[indexland(usize_arith)]`](indexland_derive::Idx#attributes))
in case it makes sense for your usecase.


Our `Idx` derivation syntax
is also much nicer to use than `index_vec`'s
[`define_index_newtype!`](https://docs.rs/index_vec/latest/index_vec/macro.define_index_type.html).

### Is there a runtime cost to this?
There is very little runtime overhead compared to using the
underlying containers directly.
The core index wrapper functions are marked `#[inline(always)]`,
so the compiler can reliably eliminate them, even in debug mode.

By default, index conversions that might overflow will be bounds checked.
This only affects index types smaller than `usize`,
and is avoided internally wherever possible.
If this is unacceptable for performance reasons,
these checks can be disabled on a per type basis through
[`#[indexland(disable_bounds_checks)]`](crate::indexland_derive::Idx),
or in a single spot through [`into_usize_unchecked`](crate::idx::Idx::into_usize_unchecked).
This causes the indices to silently wrap around, just like
`my_usize as u32` would.



## License
[MIT](../../LICENSE)
