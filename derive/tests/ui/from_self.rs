use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(omit(From<Bar>))]
    pub enum Bar {
        A,
        B,
    }
}
