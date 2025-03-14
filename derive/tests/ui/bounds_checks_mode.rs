use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(bounds_checks = "off")]
    struct Foo(u32);
}
