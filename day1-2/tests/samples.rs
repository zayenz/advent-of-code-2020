use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "1721
979
366
299
675
1456
",
        )
        .stdout()
        .is("241861950")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("12747392")
        .unwrap();
}
