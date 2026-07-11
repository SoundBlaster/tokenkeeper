use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use tokenkeeper::inspector::{FindingReason, InspectionResult, MetadataInspector, NodeType};
use tokenkeeper::profiles::{LocationSpec, NodeKind, Policy, Root};

fn temp_home(label: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("tokenkeeper-p2-t1-{label}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create test home");
    path
}

fn inspector(home: &Path) -> MetadataInspector {
    let owner_uid = fs::symlink_metadata(home).expect("home metadata").uid();
    MetadataInspector::new(home, owner_uid).expect("create inspector")
}

fn file_location(path: &str, policy: Policy, optional: bool) -> LocationSpec {
    LocationSpec::exact(Root::Home, path, NodeKind::File, policy, optional)
}

fn directory_location(path: &str, policy: Policy, optional: bool) -> LocationSpec {
    LocationSpec::exact(Root::Home, path, NodeKind::Directory, policy, optional)
}

fn first<E: std::fmt::Debug>(result: Result<Vec<InspectionResult>, E>) -> InspectionResult {
    result
        .expect("inspection should resolve")
        .into_iter()
        .next()
        .unwrap()
}

#[test]
fn secret_file_modes_are_evaluated_without_reading_contents() {
    let home = temp_home("secret");
    let path = home.join("credentials.json");
    fs::write(&path, "fixture that must not be read").expect("write fixture");
    let inspector = inspector(&home);
    let location = file_location("credentials.json", Policy::SecretFile, false);

    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("set safe mode");
    assert!(first(inspector.inspect_location(&location)).is_pass());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o400)).expect("set stricter mode");
    assert!(first(inspector.inspect_location(&location)).is_pass());

    for mode in [0o640, 0o604, 0o666] {
        fs::set_permissions(&path, fs::Permissions::from_mode(mode)).expect("set unsafe mode");
        let result = first(inspector.inspect_location(&location));
        assert!(result.is_finding());
        assert!(result
            .reasons()
            .iter()
            .any(|reason| matches!(reason, FindingReason::GroupOrOtherAccess { .. })));
    }
}

#[test]
fn credential_config_uses_secret_file_confidentiality_policy() {
    let home = temp_home("credential-config");
    let path = home.join("mcp.json");
    fs::write(&path, "fixture").expect("write fixture");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("set mode");
    let result = first(inspector(&home).inspect_location(&file_location(
        "mcp.json",
        Policy::CredentialConfig,
        false,
    )));
    assert!(result.is_finding());
}

#[test]
fn private_directory_requires_owner_only_access() {
    let home = temp_home("private-directory");
    let path = home.join("private");
    fs::create_dir(&path).expect("create directory");
    let inspector = inspector(&home);
    let location = directory_location("private", Policy::PrivateDirectory, false);

    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).expect("set safe mode");
    assert!(first(inspector.inspect_location(&location)).is_pass());
    for mode in [0o750, 0o707, 0o777] {
        fs::set_permissions(&path, fs::Permissions::from_mode(mode)).expect("set unsafe mode");
        assert!(first(inspector.inspect_location(&location)).is_finding());
    }
}

#[test]
fn trusted_config_allows_read_access_but_not_foreign_write() {
    let home = temp_home("trusted-config");
    let path = home.join("config.toml");
    fs::write(&path, "fixture").expect("write fixture");
    let inspector = inspector(&home);
    let location = file_location("config.toml", Policy::TrustedConfig, false);

    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("set readable mode");
    assert!(first(inspector.inspect_location(&location)).is_pass());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o664)).expect("set writable mode");
    let result = first(inspector.inspect_location(&location));
    assert!(result.is_finding());
    assert!(result
        .reasons()
        .iter()
        .any(|reason| matches!(reason, FindingReason::GroupOrOtherWrite { .. })));
}

#[test]
fn wrong_owner_and_node_type_are_explicit_findings() {
    let home = temp_home("owner-type");
    let file = home.join("file");
    let directory = home.join("directory");
    fs::write(&file, "fixture").expect("write file");
    fs::create_dir(&directory).expect("create directory");

    let owner_uid = fs::symlink_metadata(&home).expect("home metadata").uid();
    let wrong_owner = MetadataInspector::new(&home, owner_uid.saturating_add(1)).unwrap();
    let wrong_owner_result =
        first(wrong_owner.inspect_location(&file_location("file", Policy::SecretFile, false)));
    assert!(wrong_owner_result
        .reasons()
        .iter()
        .any(|reason| matches!(reason, FindingReason::WrongOwner { .. })));

    let inspector = inspector(&home);
    let wrong_kind = first(inspector.inspect_location(&file_location(
        "directory",
        Policy::TrustedConfig,
        false,
    )));
    assert!(wrong_kind.reasons().iter().any(|reason| matches!(
        reason,
        FindingReason::UnexpectedNodeType {
            actual: NodeType::Directory,
            ..
        }
    )));
}

#[test]
fn writable_ancestor_is_reported_for_executable_config() {
    let home = temp_home("ancestor");
    let parent = home.join("hooks");
    let file = parent.join("config.json");
    fs::create_dir(&parent).expect("create parent");
    fs::write(&file, "fixture").expect("write file");
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o777)).expect("set writable parent");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).expect("set safe file");

    let result = first(inspector(&home).inspect_location(&file_location(
        "hooks/config.json",
        Policy::ExecutableConfig,
        false,
    )));
    assert!(result.is_finding());
    assert!(result
        .reasons()
        .iter()
        .any(|reason| matches!(reason, FindingReason::WritableAncestor { .. })));
}

#[test]
fn missing_required_and_optional_locations_are_distinct() {
    let home = temp_home("missing");
    let inspector = inspector(&home);
    let optional = first(inspector.inspect_location(&file_location(
        "optional.json",
        Policy::CredentialConfig,
        true,
    )));
    let required = first(inspector.inspect_location(&file_location(
        "required.json",
        Policy::CredentialConfig,
        false,
    )));
    assert!(optional.is_missing_optional());
    assert!(required.is_missing_required());
}

#[test]
fn symlink_is_a_finding_and_not_a_resolved_file() {
    let home = temp_home("symlink");
    let real = home.join("real.json");
    let link = home.join("link.json");
    fs::write(&real, "fixture").expect("write target");
    std::os::unix::fs::symlink(&real, &link).expect("create link");

    let result = first(inspector(&home).inspect_location(&file_location(
        "link.json",
        Policy::CredentialConfig,
        false,
    )));
    assert!(result.is_finding());
    assert!(result
        .reasons()
        .iter()
        .any(|reason| matches!(reason, FindingReason::SymlinkComponent { .. })));
}
