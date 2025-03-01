# `indexland_derive`

Provides derive macros for `indexland`. For better ergonomics add the
`"derive"` feature to `indexland` instead of depending on this directly.
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
