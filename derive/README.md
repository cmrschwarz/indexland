# `indexland_derive`

This crate provides Indexland's derive and attribute macros
```rust
use indexland_derive::{EnumIdx, make_enum_idx};

// implements the indexland::EnumIdx trait
// expects all neccessary impls to be present
#[derive(EnumIdx)]
enum Direction{
    West,
    North,
    East,
    South,
};

// implements the indexland::EnumIdx trait and adds all neccessary derives
#[make_enum_idx]
enum Color{
    Red,
    Green,
    Blue
};
```

## License
[MIT](../../LICENSE)
