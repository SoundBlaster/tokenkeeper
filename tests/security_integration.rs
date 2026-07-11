use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;

use tokenkeeper::cli::{parse, Command};
use tokenkeeper::inspector::MetadataInspector;
use tokenkeeper::profiles::{LocationSpec, NodeKind, Policy, Root, Traversal};
use tokenkeeper::report::{remediation, render};
use tokenkeeper::resolver::{ResolveError, Resolver};

fn home(label: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("tokenkeeper-p3-t2-{label}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("fixture home");
    path
}

#[test]
fn symlink_escape_and_loop_never_leave_trusted_home() {
    let root = home("symlinks");
    let outside = std::env::temp_dir().join(format!("tokenkeeper-outside-{}", std::process::id()));
    fs::create_dir_all(&outside).expect("outside fixture");
    std::os::unix::fs::symlink(&outside, root.join("escape")).expect("escape link");
    std::os::unix::fs::symlink("loop", root.join("loop")).expect("loop link");
    let resolver = Resolver::new(&root).expect("resolver");
    for path in ["escape/secret", "loop/secret"] {
        assert!(matches!(
            resolver.resolve(&LocationSpec::exact(
                Root::Home,
                path,
                NodeKind::File,
                Policy::SecretFile,
                false
            )),
            Err(ResolveError::SymlinkComponent(_))
        ));
    }
    let _ = fs::remove_dir_all(outside);
}

#[test]
fn bounded_traversal_does_not_return_partial_pass() {
    let root = home("bounds");
    fs::create_dir(root.join("tree")).expect("tree");
    fs::write(root.join("tree/one"), "fixture").expect("one");
    fs::write(root.join("tree/two"), "fixture").expect("two");
    let resolver = Resolver::new(&root).expect("resolver");
    let location = LocationSpec {
        root: Root::Home,
        path: "tree".into(),
        kind: NodeKind::Either,
        policy: Policy::TrustedConfig,
        optional: false,
        traversal: Traversal::Bounded {
            max_depth: 1,
            max_entries: 1,
        },
    };
    assert!(matches!(
        resolver.resolve(&location),
        Err(ResolveError::TraversalLimitExceeded { .. })
    ));
}

#[test]
fn odd_filenames_are_sanitized_and_never_become_commands() {
    let root = home("odd");
    let path = root.join("leading -dash 'quote'\nansi");
    fs::write(&path, "secret fixture").expect("odd file");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("mode");
    let owner = fs::symlink_metadata(&root).unwrap().uid();
    let inspector = MetadataInspector::new(&root, owner).unwrap();
    let location = LocationSpec::exact(
        Root::Home,
        path.strip_prefix(&root).unwrap(),
        NodeKind::File,
        Policy::CredentialConfig,
        false,
    );
    let result = inspector.inspect_location(&location).unwrap().remove(0);
    let output = render(&result, Some(Policy::CredentialConfig));
    assert!(!output.contains("\nansi"));
    assert!(output.contains("\\nansi"));
    assert!(remediation(&result, Policy::CredentialConfig).is_none());
}

#[test]
fn inspector_does_not_mutate_target_metadata_or_content() {
    let root = home("readonly");
    let path = root.join("credentials");
    let contents = b"must not be read or changed";
    fs::write(&path, contents).expect("file");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("mode");
    let before_mode = fs::symlink_metadata(&path).unwrap().permissions().mode();
    let owner = fs::symlink_metadata(&root).unwrap().uid();
    let inspector = MetadataInspector::new(&root, owner).unwrap();
    let location = LocationSpec::exact(
        Root::Home,
        "credentials",
        NodeKind::File,
        Policy::SecretFile,
        false,
    );
    assert!(inspector.inspect_location(&location).unwrap()[0].is_pass());
    assert_eq!(fs::read(&path).unwrap(), contents);
    assert_eq!(
        fs::symlink_metadata(&path).unwrap().permissions().mode(),
        before_mode
    );
}

#[test]
fn malformed_cli_input_returns_error_without_panic() {
    assert!(parse(["check".into(), "--policy".into(), "unknown".into()]).is_err());
    assert!(parse(["check".into(), "--path".into(), "-dash".into()]).is_err());
    assert!(!matches!(
        parse(["check".into(), "--profile".into(), "codex".into()]),
        Ok(Command::Help)
    ));
}
