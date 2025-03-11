use indexland::Idx;

fn main() {
    #[derive(Idx)]
    struct Foo(u32);

    let idx = Foo(12);

    idx += 1 as usize;

    println!("{idx}");
}
