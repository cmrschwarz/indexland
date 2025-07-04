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

- All array-like `std::collections` wrapped in one place through a single
  [`Idx`](https://docs.rs/indexland/latest/indexland/trait.Idx.html) trait.

- Opt-in support for the popular `arrayvec`, `smallvec`, `indexmap` and `slab` crates.

- **All** underlying APIs available and faithfully adapted for `Idx` types.

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
use indexland::{Idx, IdxEnum, IndexArray, index_array};

#[derive(Idx)]
enum Status { Idle, Running, Done }

// Alternatively, the `EnumIndexArray` type alias could be used to to avoid specifying the array len
const STATUS_MESSAGE: IndexArray<Status, &str, { Status::VARIANT_COUNT }> = index_array![
    Status::Idle => "Waiting for input...",
    Status::Running => "Processing your request...",
    Status::Done => "Operation complete!",
];

let message = STATUS_MESSAGE[Status::Running];
```

## Supported Collections

| `indexland` | Base Collection | Feature Flag |
|----------|-----------------------|:------------------:|
| [`IndexVec<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVec.html) | [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) | `alloc` |
| [`IndexVecDeque<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexVecDeque.html) | [`VecDeque<T>`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) |  `alloc` |
| [`IndexArray<I, T, N>`](https://docs.rs/indexland/latest/indexland/struct.IndexArray.html) | [`[T; N]`](https://doc.rust-lang.org/std/primitive.array.html) | - |
| [`IndexSlice<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexSlice.html) | [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html) | - |
| [`IndexArrayVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexArrayVec.html) | [`arrayvec::ArrayVec<T, CAP>`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) | `arrayvec` |
| [`IndexSmallVec<I, T, CAP>`](https://docs.rs/indexland/latest/indexland/struct.IndexSmallVec.html) | [`smallvec::SmallVec<[T; CAP]>`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html)  | `smallvec`  |
| [`IndexHashMap<I, K, V>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashMap.html) | [`indexmap::IndexMap<K, V>`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html) | `indexmap` |
| [`IndexHashSet<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexHashSet.html) | [`indexmap::IndexSet<T>`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html) | `indexmap` |
| [`IndexSlab<I, T>`](https://docs.rs/indexland/latest/indexland/struct.IndexSlab.html) | [`slab::Slab<T>`](https://docs.rs/slab/latest/slab/struct.Slab.html) | `slab` |

`std` and therefore `alloc` are enabled by default.
Use the `full` feature to enable all collections.

## Additional Features

- Every wrapper has an escape hatch to the underlying collection,
  as well as bidirectional [`From`](core::convert::From) implementations.

- First-class embedded support through
  [`#[no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html)
  and optional [`alloc`](https://doc.rust-lang.org/core/alloc/index.html).

- Opt-in
  [`NonMax<T>`](https://docs.rs/indexland/latest/indexland/struct.NonMax.html) for
  `Idx` compatible [Niche Optimizations](https://doc.rust-lang.org/std/option/index.html#representation).

- Opt-in [`serde`](::serde) implementations for all Collections.

## Why ?
Indexed collections are a powerful alternative to references or smart pointers.
The pattern was popularized by
[Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design),
and is even used by [the Rust Compiler itself](https://github.com/rust-lang/rust/blob/2b285cd5f0877e30ad1d83e04f8cc46254e43391/compiler/rustc_index/src/vec.rs#L40).
Using indices can solve many borrow checker issues and often improve performance
due to reduced allocations and memory usage aswell as increased data locality.

Heavy use of this pattern can cause issues though. The standard approach is to
use `type NodeId = usize`, but this negatively affects:

  1. **Type Safety**: Type aliases are not unique types.
     This makes it easy to accidentally confuse them without getting any compiler errors.
     An index is essentially a relative pointer.
     Disabling type checking on all your pointers is a terrible idea.
     Rust normally excells at enabling fearless refactoring,
     but non-newtype indices threaten this by severely reducing the robustness of your code.
     They act similarly to `void*` in C, or `any` in typescript,
     and effectively turn of the type checker. Newtype indices solve this problem.

  2. **Readability**: Container type definitions don't tell us what index
     should be used to access them. When structs contain multiple collections,
     this becomes hard to read quickly. By forcing you to add the index type
     as a generic parameter to your container definitions, `indexland` solves this problem.


## Comparison with [index_vec](https://docs.rs/index_vec/latest/index_vec/index.html)
1.  **Unified API**: `indexland` offers all common collections in one place,
    **using a single `Idx` trait**. Sometimes the same index type is used
    for multiple data structures. Sometimes you want to switch from a `Vec<T>`
    to a `VecDeque<T>` without rewriting all your code. `indexland` has you covered.

2.  **Cleaner Syntax**: Our `Idx` derive macro is much simpler to use than
    `index_vec`'s [`define_index_newtype!`](https://docs.rs/index_vec/latest/index_vec/macro.define_index_type.html),
    while also offering more customization options.
    We still offer a (specced down) declarative alternative for crates
    that need to avoid proc-macro dependencies.

2.  **Better Type Safety**: We deliberately **don't** implement
    `Index<usize> for IndexSlice` and `Add<usize> for Idx`,
    as they circumvent many of the type safety benefits of newtypes, which even `index_vec`
    [acknowledges](https://docs.rs/index_vec/0.1.4/src/index_vec/indexing.rs.html#113).

    We disable them by default, but offer opt-in support via
    [`#[indexland(idx_compat(usize))]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandcompatible)
    and
    [`#[indexland(arith_compat(usize))]`](https://docs.rs/indexland_derive/latest/indexland_derive/derive.Idx.html#indexlandusize_arith)
    respectively for cases where the ergonomic benefits are worth it.



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

<sub>
Contributions are implied to be dual licensed under the same terms unless stated otherwise.
</sub>
