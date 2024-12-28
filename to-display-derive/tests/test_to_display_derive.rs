#[test]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/to_display/fail/*.rs");
}

#[test]
fn pass() {
    macrotest::expand("tests/to_display/success/*.rs");
}
