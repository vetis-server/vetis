#[test]
#[cfg(not(tarpaulin))]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/security-config-pass-1.rs");
    //t.compile_fail("tests/ui/security-config-compile-fail-1.rs");
    //t.compile_fail("tests/ui/security-config-compile-fail-2.rs");
    //t.compile_fail("tests/ui/security-config-compile-fail-3.rs");
}
