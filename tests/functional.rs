use std::process::Command;

#[test]
fn hello_ferrumweave_cli_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_ferrumweave"))
        .output()
        .expect("FerrumWeave binary should start");

    assert!(output.status.success(), "binary should exit successfully");
    assert!(
        output.stderr.is_empty(),
        "binary should not write to stderr"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    let normalized = stdout.trim_end_matches(&['\r', '\n'][..]);
    assert_eq!(normalized, "Hello FerrumWeave");
}
