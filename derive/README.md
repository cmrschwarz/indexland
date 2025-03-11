# `indexland_derive`

Provides derive macros for `indexland`. For better ergonomics add the
`"derive"` feature to `indexland` instead of depending on this directly.

## Example
```rust
// `indexland_derive::Idx` is re-exported as `indexland::Idx`
use indexland::Idx;

#[derive(Idx)]
struct NodeId(u32);

#[derive(Idx)]
enum PrimaryColor {
    Red,
    Green,
    Blue,
};
```

## License
[MIT](../../LICENSE)
