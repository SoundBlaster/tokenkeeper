use std::fs;
use std::path::{Path, PathBuf};

use tokenkeeper::profiles::{LocationSpec, NodeKind, Policy, ProfileRegistry, ProfileSpec, Root};
use tokenkeeper::resolver::{ResolveError, Resolver};

fn temp_home(label: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("tokenkeeper-p1-t3-{label}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create test home");
    path
}

fn profile(id: &str, path: &str) -> ProfileSpec {
    ProfileSpec::new(
        id,
        id,
        vec![tokenkeeper::profiles::Platform::MacOs],
        vec![LocationSpec::exact(
            Root::Home,
            path,
            NodeKind::File,
            Policy::CredentialConfig,
            true,
        )],
    )
}

#[test]
fn registry_rejects_duplicate_ids_and_parent_traversal() {
    let duplicate = ProfileRegistry::new(vec![profile("codex", "one"), profile("codex", "two")]);
    let errors = duplicate.validate().expect_err("duplicate IDs must fail");
    assert!(errors.iter().any(|error| error.is_duplicate_id()));

    let invalid_id = ProfileRegistry::new(vec![profile("Bad Profile", "one")]);
    let errors = invalid_id.validate().expect_err("invalid IDs must fail");
    assert!(errors.iter().any(|error| matches!(
        error,
        tokenkeeper::profiles::ValidationError::InvalidId { .. }
    )));

    let invalid = ProfileRegistry::new(vec![profile("bad", "../outside")]);
    let errors = invalid.validate().expect_err("parent traversal must fail");
    assert!(errors.iter().any(|error| error.is_parent_traversal()));
}

#[test]
fn registry_rejects_absolute_paths_and_unbounded_limits() {
    let absolute = ProfileRegistry::new(vec![profile("absolute", "/tmp/secret")]);
    let errors = absolute.validate().expect_err("absolute paths must fail");
    assert!(errors.iter().any(|error| error.is_absolute_path()));

    let location = LocationSpec::bounded(
        Root::Home,
        "tree",
        NodeKind::Directory,
        Policy::TrustedConfig,
        true,
        0,
        10,
    );
    let invalid = ProfileRegistry::new(vec![ProfileSpec::new(
        "bounded",
        "Bounded",
        vec![tokenkeeper::profiles::Platform::MacOs],
        vec![location],
    )]);
    let errors = invalid.validate().expect_err("zero depth must fail");
    assert!(errors.iter().any(|error| error.is_invalid_limit()));
}

#[test]
fn resolver_maps_semantic_roots_without_using_environment_paths() {
    let home = temp_home("roots");
    let resolver = Resolver::new(&home).expect("absolute home is valid");

    let config = LocationSpec::exact(
        Root::XdgConfig,
        "opencode/opencode.json",
        NodeKind::File,
        Policy::CredentialConfig,
        true,
    );
    let resolved = resolver.resolve(&config).expect("resolve config");
    assert_eq!(
        resolved[0].path(),
        home.join(".config/opencode/opencode.json").as_path()
    );
    assert!(!resolved[0].exists());
}

#[test]
fn resolver_rejects_target_and_intermediate_symlinks() {
    let home = temp_home("symlink");
    fs::create_dir(home.join("real")).expect("create real directory");
    fs::write(home.join("real/secret.json"), "fixture").expect("create fixture");
    std::os::unix::fs::symlink(home.join("real"), home.join("alias"))
        .expect("create directory symlink");
    std::os::unix::fs::symlink(home.join("real/secret.json"), home.join("secret-link"))
        .expect("create file symlink");
    let resolver = Resolver::new(&home).expect("absolute home is valid");

    let intermediate = LocationSpec::exact(
        Root::Home,
        "alias/secret.json",
        NodeKind::File,
        Policy::CredentialConfig,
        true,
    );
    assert!(matches!(
        resolver.resolve(&intermediate),
        Err(ResolveError::SymlinkComponent(_))
    ));

    let target = LocationSpec::exact(
        Root::Home,
        "secret-link",
        NodeKind::File,
        Policy::CredentialConfig,
        true,
    );
    assert!(matches!(
        resolver.resolve(&target),
        Err(ResolveError::SymlinkComponent(_))
    ));
}

#[test]
fn bounded_resolver_honors_depth_and_entry_limits() {
    let home = temp_home("bounded");
    fs::create_dir(home.join("tree")).expect("create tree");
    fs::write(home.join("tree/one.json"), "fixture").expect("create child");
    fs::create_dir(home.join("tree/nested")).expect("create nested");
    fs::write(home.join("tree/nested/two.json"), "fixture").expect("create nested child");
    let resolver = Resolver::new(&home).expect("absolute home is valid");

    let bounded = LocationSpec::bounded(
        Root::Home,
        "tree",
        NodeKind::Directory,
        Policy::TrustedConfig,
        true,
        1,
        10,
    );
    let resolved = resolver.resolve(&bounded).expect("bounded traversal");
    assert_eq!(resolved.len(), 3);
    assert!(resolved.iter().all(|item| item.exists()));

    let limited = LocationSpec::bounded(
        Root::Home,
        "tree",
        NodeKind::Directory,
        Policy::TrustedConfig,
        true,
        1,
        1,
    );
    assert!(matches!(
        resolver.resolve(&limited),
        Err(ResolveError::TraversalLimitExceeded { .. })
    ));
}

#[test]
fn resolver_requires_an_absolute_home() {
    let error = Resolver::new(Path::new("relative-home")).expect_err("relative home must fail");
    assert_eq!(error, ResolveError::HomeNotAbsolute);
}

#[test]
fn resolver_rejects_missing_and_non_directory_homes() {
    let missing = std::env::temp_dir().join(format!("tokenkeeper-missing-{}", std::process::id()));
    let _ = std::fs::remove_file(&missing);
    assert!(matches!(
        Resolver::new(&missing),
        Err(ResolveError::HomeMissing(_))
    ));

    let file = std::env::temp_dir().join(format!("tokenkeeper-home-file-{}", std::process::id()));
    std::fs::write(&file, "fixture").unwrap();
    assert!(matches!(
        Resolver::new(&file),
        Err(ResolveError::HomeNotDirectory(_))
    ));
    let _ = std::fs::remove_file(file);
}

#[test]
fn resolver_rejects_invalid_relative_location_and_reports_missing_target() {
    let root = temp_home("invalid-location");
    let resolver = Resolver::new(&root).unwrap();
    let invalid = LocationSpec::exact(
        Root::Home,
        "../escape",
        NodeKind::File,
        Policy::SecretFile,
        false,
    );
    assert!(matches!(
        resolver.resolve(&invalid),
        Err(ResolveError::InvalidRelativePath(_))
    ));
    let missing = LocationSpec::exact(
        Root::Home,
        "missing/file",
        NodeKind::File,
        Policy::SecretFile,
        true,
    );
    let resolved = resolver.resolve(&missing).unwrap();
    assert_eq!(resolved.len(), 1);
    assert!(!resolved[0].exists());
    let _ = std::fs::remove_dir_all(root);
}
