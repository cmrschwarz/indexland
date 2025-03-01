# `indexland_derive`

Provides derive macros for `indexland`. For better ergonomics add the
`"derive"` feature to `indexland` instead of depending on this directly.
```rust
// re-exported by indexland aswell
use indexland_derive::{NewtypeIdx, EnumIdx};

#[derive(NewtypeIdx)]
struct NodeId(u32);

#[derive(EnumIdx)]
enum PrimaryColor{
    Red,
    Green,
    Blue
};
```

## License
[MIT](../../LICENSE)
