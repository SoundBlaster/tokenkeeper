use std::process::Command;

fn tokenkeeper() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tokenkeeper"))
}

#[test]
fn help_is_available_without_a_home_directory() {
    let output = tokenkeeper()
        .arg("--help")
        .env("HOME", "/definitely/missing/tokenkeeper-test-home")
        .output()
        .expect("tokenkeeper should start");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: tokenkeeper"));
    assert!(stdout.contains("--version"));
    assert!(output.stderr.is_empty());
}

#[test]
fn version_reports_the_package_version() {
    let output = tokenkeeper()
        .arg("--version")
        .output()
        .expect("tokenkeeper should start");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "tokenkeeper 0.1.0"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn unknown_argument_returns_usage_error_without_panicking() {
    let output = tokenkeeper()
        .arg("--definitely-unknown")
        .output()
        .expect("tokenkeeper should start");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown argument"));
    assert!(!stderr.contains("panic"));
}
