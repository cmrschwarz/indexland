use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(omit(From<Foo<qq, bb>>))]
    pub enum Bar {
        A,
        B,
    }
}
