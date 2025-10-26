use assert_cmd::Command;
use std::time::Duration;

#[test]
fn test_performance_startup_time() {
    let mut cmd = Command::cargo_bin("monar").unwrap();
    cmd.timeout(Duration::from_secs(1));
    cmd.assert().success();
}
