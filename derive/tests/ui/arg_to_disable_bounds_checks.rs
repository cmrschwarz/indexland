use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(bounds_checks = true)]
    pub enum Bar {
        A,
        B,
    }
}
