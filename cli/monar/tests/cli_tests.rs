use assert_cmd::Command;
use insta::assert_snapshot;

#[test]
fn test_cli_output() {
    let mut cmd = Command::cargo_bin("monar").unwrap();
    let output = cmd.output().unwrap();
    assert_snapshot!(String::from_utf8(output.stdout).unwrap());
}

#[test]
fn test_cli_output_with_name() {
    let mut cmd = Command::cargo_bin("monar").unwrap();
    cmd.arg("--name").arg("test");
    let output = cmd.output().unwrap();
    assert_snapshot!(String::from_utf8(output.stdout).unwrap());
}

#[test]
fn test_invalid_argument() {
    let mut cmd = Command::cargo_bin("monar").unwrap();
    cmd.arg("--invalid-arg");
    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    assert_snapshot!(String::from_utf8(output.stderr).unwrap());
}

#[test]
fn test_regression_bug_123() {
    // This test ensures that a previously fixed bug (e.g., related to specific input) does not reappear.
    let mut cmd = Command::cargo_bin("monar").unwrap();
    cmd.arg("--some-specific-input-that-caused-a-bug");
    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    assert_snapshot!(String::from_utf8(output.stderr).unwrap());
}
