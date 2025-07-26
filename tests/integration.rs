#![warn(clippy::pedantic)]

mod macros;

// only enable ui tests on nightly to avoid failures due to
// differing error messages
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[cfg(feature = "derive")]
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui_derive() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui_derive/*.rs");
}
