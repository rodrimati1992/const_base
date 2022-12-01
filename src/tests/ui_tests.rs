#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/tests/ui/*err.rs");
    t.pass("src/tests/ui/*fine.rs");
}
