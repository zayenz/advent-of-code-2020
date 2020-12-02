use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
")
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("666")
        .unwrap();
}
