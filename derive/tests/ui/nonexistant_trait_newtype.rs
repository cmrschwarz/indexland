use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(omit(From<whatever<{3>2}>))]
    pub struct Foo(u32);
}
