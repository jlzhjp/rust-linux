#[test]
fn cli() {
    trycmd::TestCases::new()
        .register_bin("echo", trycmd::cargo::cargo_bin!("echo"))
        .case("tests/cmd/*.trycmd");
}
