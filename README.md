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
use indexland::{index_newtype, index_vec::IndexVec};
index_newtype!{
    struct NodeId(u32);
}
struct Graph<T>{
    nodes: IndexVec<NodeId, T>,
    edge: IndexVec<NodeId, Vec<NodeId>>,
}
```

## License
[MIT](../../LICENSE)
