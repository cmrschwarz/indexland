use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(disable_bounds_checks = true)]
    pub enum Bar {
        A,
        B,
    }
}
