#![warn(clippy::pedantic)]

mod macros;

#[rustversion::attr(not(stable(1.87)), ignore = "ui tests only run on 1.87 (MSRV)")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[rustversion::attr(not(stable(1.87)), ignore = "ui tests only run on 1.87 (MSRV)")]
#[rustversion::attr(
    stable(1.87),
    cfg_attr(
        not(all(feature = "derive", feature = "std")),
        ignore = "ui_derive tests require the \"derive\" and \"std\" feature flags"
    )
)]
#[test]
fn ui_derive() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui_derive/*.rs");
}
