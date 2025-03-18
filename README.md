# `indexland`

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

Newtype Indices for Rust Collection Types.

## Features
- Strongly typed collection indices for better type safety and readability.

- All array-based `std::collections` wrapped in one place through a single
  [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html) trait,

  with optional support for the popular `arrayvec`, `smallvec`, and `indexmap` crates.

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

## Supported Collections

| `indexland` | Base Collection | Feature Flag |
|----------|-----------------------|:------------------:|
| [`IndexSlice<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexSlice.html) | [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html) | - |
| [`IndexArray<I, T, N>`](https://docs.rs/indexland/latest/indexland/struct.IndexArray.html) | [`[T; N]`](https://doc.rust-lang.org/std/primitive.array.html) | - |
| [`IndexVec<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVec.html) | [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) | `alloc` |
| [`IndexVecDeque<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVecDeque.html) | [`VecDeque<T>`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) |  `alloc` |
| [`IndexArrayVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexArrayVec.html) | [`arrayvec::ArrayVec<T, CAP>`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) | `arrayvec` |
| [`IndexSmallVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexSmallVec.html) | [`smallvec::SmallVec<[T; CAP]>`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html)  | `smallvec`  |
| [`IndexHashMap<I, K, V>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashMap.html) | [`indexmap::IndexMap<K, V>`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html) | `indexmap` |
| [`IndexHashSet<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashSet.html) | [`indexmap::IndexSet<T>`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html) | `indexmap` |

`std` and therefore `alloc` are enabled by default.
Use the `full` feature to enable all collections.

## Additional Features

- Every wrapper has an escape hatch
  to the underlying collection, as well as bidirectional [`From`](core::convert::From)
  implementations.

- First-class embedded support through
  [`#[no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html)
  and even optional
  [`alloc`](https://doc.rust-lang.org/core/alloc/index.html).

- `Idx` compatible
  [`NonMax<T>`](https://docs.rs/indexland/latest/indexland/struct.NonMax.html) for
  [Niche Optimizations](https://doc.rust-lang.org/std/option/index.html#representation).

- [`serde`](::serde) implementations for all Collections.

## Why use `indexland` ?
Indexed collections are a powerful alternative to references or smart pointers.
Popularized by
[Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design),
and even used by
[the Rust Compiler itself](https://github.com/rust-lang/rust/blob/2b285cd5f0877e30ad1d83e04f8cc46254e43391/compiler/rustc_index/src/vec.rs#L40),
the pattern solves many borrow checker issues and often increases performance.
Indexed collections can often reduce allocations and memory usage while increasing
data locality.

Heavy use of this pattern can cause issues though. The standard approach is to
use `type NodeId = usize`, but this negatively affects:

  1. **Type Safety**: Type aliases are not unique types.
     This makes it easy to accidentally use the wrong index or container
     without getting any compiler errors. Rust excells at
     enabling fearless refactoring, the compiler errors become your todo list.
     Non-newtype type aliases reduce the robustness and refactorability of your code.

  2. **Readability**: Container type definitions don't tell us what index
     should be used to access them. When structs contain multiple collections
     this becomes hard to read quickly.

`indexland` solves both of these issues.

## Comparison with [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
1.  **Unified API**: `indexland` offers all common collections in one place,
    **using a single `Idx` trait**. Sometimes the same index type is used
    for multiple data structures. Sometimes you want to switch from a `Vec<T>`
    to a `VecDeque<T>`, or can use a static array.

2.  **Type Safety**: We deliberately **don't** implement
    `Index<usize> for IndexSlice` and `Add<usize> for Idx`,
    as they negatively impact type safety. Opt-in support is available
    via
    [`#[indexland(compatible(usize))]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandcompatible)
    and
    [`#[indexland(usize_arith)]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandusize_arith)
    respectively for cases where it makes sense.

3.  **Cleaner Syntax**: Our `Idx` derive macro is much nicer to use than
    `index_vec`'s [`define_index_newtype!`](https://docs.rs/index_vec/latest/index_vec/macro.define_index_type.html),
    while also offering more customization options.
    We offer a declarative alternative if you dislike proc-macros though.

## Performance
There's close to zero runtime overhead. The core index conversion functions are
marked `#[inline(always)]` to reliably eliminate them, even in debug mode.

Type conversions follow the same rules as Rust integer overflow.
Overflows will panic in debug mode and wrap in release.
Newtypes can customize this via
[`#[indexland(bounds_checks = "..")]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandbounds_checks--),
or bypass it through
[`into_usize_unchecked`](https://docs.rs/indexland/latest/indexland/trait.Idx.html#tymethod.into_usize).


## License
[MIT](./LICENSE-MIT) or [Apache Version 2.0](./LICENSE-APACHE), at your option.
