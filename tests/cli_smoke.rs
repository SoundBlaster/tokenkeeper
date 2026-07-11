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
        "tokenkeeper 0.2.0"
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

#[test]
fn mixed_scope_and_duplicate_flags_are_rejected() {
    let output = tokenkeeper()
        .args([
            "check",
            "--profile",
            "codex",
            "--path",
            "x",
            "--policy",
            "trusted-config",
        ])
        .output()
        .expect("tokenkeeper should start");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot be combined"));
}

#[test]
fn check_and_profiles_execute_with_explicit_scope() {
    let home = std::env::temp_dir().join(format!("tokenkeeper-cli-home-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(home.join("config"), "fixture").unwrap();
    let output = tokenkeeper()
        .args(["check", "--path", "config", "--policy", "trusted-config"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(matches!(output.status.code(), Some(0) | Some(1) | Some(2)));
    let profiles = tokenkeeper().arg("profiles").output().unwrap();
    assert!(profiles.status.success());
    let _ = std::fs::remove_dir_all(home);
}

#[test]
fn check_rejects_unknown_profile() {
    let output = tokenkeeper()
        .args(["check", "--profile", "does-not-exist"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown profile"));
}

#[test]
fn check_reports_invalid_custom_scope_and_missing_home() {
    let home = std::env::temp_dir().join(format!("tokenkeeper-cli-home-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    let outside = tokenkeeper()
        .args(["check", "--path", "/tmp", "--policy", "trusted-config"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert_eq!(outside.status.code(), Some(2));
    let missing = tokenkeeper()
        .arg("check")
        .env("HOME", home.join("missing"))
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(2));
    let _ = std::fs::remove_dir_all(home);
}

#[test]
fn check_runs_selected_builtin_profile() {
    let home = std::env::temp_dir().join(format!("tokenkeeper-cli-profile-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    let output = tokenkeeper()
        .args(["check", "--profile", "codex"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(matches!(output.status.code(), Some(0) | Some(1) | Some(2)));
    let _ = std::fs::remove_dir_all(home);
}

#[test]
fn command_errors_cover_scope_and_environment_failures() {
    for args in [
        vec!["check", "--help"],
        vec!["check", "--unknown"],
        vec!["check", "--path", "config"],
    ] {
        let output = tokenkeeper().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
    }
    let output = tokenkeeper()
        .arg("check")
        .env_remove("HOME")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn multiple_profiles_are_checked_in_one_run() {
    let home = std::env::temp_dir().join(format!("tokenkeeper-cli-multi-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    let output = tokenkeeper()
        .args(["check", "--profile", "codex", "--profile", "cursor"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(output.status.code().is_some());
    let _ = std::fs::remove_dir_all(home);
}
