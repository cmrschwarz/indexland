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

Collections based on newtype indices for increased type safety and self
documenting code.

Part of the [Typeline](https://github.com/cmrschwarz/typeline) project,
not ready for public use yet.


## Usage Examles
```rust
use indexland::{NewtypeIdx, index_vec::IndexVec};
#[derive(NewtypeIdx)]
struct FooId(u32);
struct Foo{ /*...*/ };
struct FooContainer {
    foos: IndexVec<FooId, Foo>,
}

use indexland::{EnumIdx, index_array::{IndexArray, EnumIndexArray}};
#[derive(EnumIdx)]
enum Bar{
    A,
    B,
    C
};
let BAR_MAPPING: EnumIndexArray<Bar, i32> = IndexArray::new([1, 2, 3]);
```

## License
[MIT](../../LICENSE)
