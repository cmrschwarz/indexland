// the submodule is used to combine all integration tests into a single binary

mod integration;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
