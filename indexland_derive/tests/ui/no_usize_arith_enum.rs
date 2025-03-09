use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(extra(Display))]
    enum Foo {
        A,
        B,
        C,
    }

    let idx = Foo::A;

    idx += 1 as usize;

    println!("{idx}");
}
