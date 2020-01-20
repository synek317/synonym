#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/cases/pass-*.rs");
    t.compile_fail("tests/cases/fail-*.rs");
}
