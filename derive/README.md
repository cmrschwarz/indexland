# `indexland_derive`

Provides derive macros for `indexland`. For better ergonomics add the
`"derive"` feature to `indexland` instead of depending on this directly.
```rust
use indexland::{IdxNewtype, index_vec::IndexVec};
#[derive(IdxNewtype)]
struct FooId(u32);
struct Foo{ /*...*/ };
struct FooContainer {
    foos: IndexVec<FooId, Foo>,
}

use indexland::{IdxEnum, index_array::{IndexArray, EnumIndexArray}};
#[derive(IdxEnum)]
enum Bar{
    A,
    B,
    C
};
let BAR_MAPPING: EnumIndexArray<Bar, i32> = IndexArray::new([1, 2, 3]);
```

## License
[MIT](../../LICENSE)
