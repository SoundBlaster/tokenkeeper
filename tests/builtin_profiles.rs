use tokenkeeper::profiles::{builtin_registry, Policy, Root};

#[test]
fn required_profiles_are_embedded_with_evidence() {
    let registry = builtin_registry();
    registry.validate().expect("built-in profiles validate");
    for id in [
        "codex",
        "claude-code",
        "opencode",
        "cursor",
        "mcp-integrations",
    ] {
        let profile = registry.find(id).expect("required profile");
        assert!(profile.source.is_some());
        assert_eq!(profile.verified_on.as_deref(), Some("2026-07-11"));
        assert!(!profile.locations.is_empty());
    }
}

#[test]
fn credential_locations_use_conservative_policies_and_semantic_roots() {
    let registry = builtin_registry();
    let codex = registry.find("codex").unwrap();
    assert!(
        codex
            .locations
            .iter()
            .any(|location| location.policy == Policy::CredentialConfig
                && location.root == Root::Home)
    );
    assert!(codex
        .locations
        .iter()
        .all(|location| location.policy == Policy::CredentialConfig));
    let opencode = registry.find("opencode").unwrap();
    assert!(opencode
        .locations
        .iter()
        .any(|location| location.root == Root::XdgConfig));
}

#[test]
fn builtins_do_not_embed_secret_content() {
    let debug = format!("{:?}", builtin_registry());
    assert!(!debug.contains("sk-"));
    assert!(!debug.contains("Bearer "));
}
