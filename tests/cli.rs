#[test]
fn cli() {
    trycmd::TestCases::new()
        .register_bin("echo", trycmd::cargo::cargo_bin!("echo"))
        .register_bin("cat", trycmd::cargo::cargo_bin!("cat"))
        .case("tests/cmd/*.trycmd")
        .case("tests/cmd/*.toml");
}
