#![cfg(target_os = "macos")]

use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::process::Command;

use tokenkeeper::acl::AclDecision;
use tokenkeeper::inspector::{FindingReason, InspectionResult, MetadataInspector};
use tokenkeeper::profiles::{LocationSpec, NodeKind, Policy, Root};

fn chmod_acl(action: &str, rule: &str, path: &std::path::Path) {
    let status = Command::new("/bin/chmod")
        .args([action, rule])
        .arg(path)
        .status()
        .expect("chmod must be available on macOS");
    assert!(status.success(), "chmod {action} failed for {path:?}");
}

#[test]
fn native_target_acl_allow_is_a_finding() {
    let root = std::env::temp_dir().join(format!("tokenkeeper-native-acl-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let file = root.join("secret");
    fs::write(&file, "fixture").unwrap();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    chmod_acl("+a", "everyone allow read", &file);

    let owner = fs::symlink_metadata(&file).unwrap().uid();
    let decision = tokenkeeper::acl::evaluate_path(&file, owner, Policy::SecretFile);
    assert!(matches!(decision, AclDecision::Finding { .. }));
    chmod_acl("-a", "everyone allow read", &file);
    let _ = fs::remove_dir_all(root);
}

#[test]
fn native_parent_acl_allow_is_retained_with_target_result() {
    let root =
        std::env::temp_dir().join(format!("tokenkeeper-native-parent-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let parent = root.join("parent");
    let file = parent.join("secret");
    fs::create_dir(&parent).unwrap();
    fs::write(&file, "fixture").unwrap();
    chmod_acl("+a", "everyone allow add_file", &parent);

    let owner = fs::symlink_metadata(&root).unwrap().uid();
    let inspector = MetadataInspector::new(&root, owner).unwrap();
    let location = LocationSpec::exact(
        Root::Home,
        "parent/secret",
        NodeKind::File,
        Policy::CredentialConfig,
        false,
    );
    let result = inspector.inspect_location(&location).unwrap().remove(0);
    assert!(
        matches!(result, InspectionResult::Finding { ref reasons, .. } if reasons.iter().any(|reason| matches!(reason, FindingReason::AncestorAclAccess { .. })))
    );
    chmod_acl("-a", "everyone allow add_file", &parent);
    let _ = fs::remove_dir_all(root);
}
