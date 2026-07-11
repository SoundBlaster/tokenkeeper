use tokenkeeper::acl::{evaluate_text, AclDecision};
use tokenkeeper::profiles::Policy;

#[test]
fn owner_only_acl_passes_relevant_policies() {
    let text = "owner:allow:read,write\ngroup:staff:deny:read\n";
    assert_eq!(
        evaluate_text(text, 501, Policy::SecretFile),
        AclDecision::Pass
    );
}

#[test]
fn non_owner_read_is_finding_for_secret_but_write_for_trusted() {
    assert!(matches!(
        evaluate_text("user:alice:allow:read", 501, Policy::SecretFile),
        AclDecision::Finding { .. }
    ));
    assert!(matches!(
        evaluate_text("user:alice:allow:write", 501, Policy::TrustedConfig),
        AclDecision::Finding { .. }
    ));
}

#[test]
fn inherited_and_allow_deny_ambiguity_is_unknown() {
    assert!(matches!(
        evaluate_text("user:alice:allow:read:inherited", 501, Policy::SecretFile),
        AclDecision::Unknown { .. }
    ));
    assert!(matches!(
        evaluate_text(
            "user:alice:deny:read\nuser:bob:allow:read",
            501,
            Policy::SecretFile
        ),
        AclDecision::Unknown { .. }
    ));
}

#[test]
fn malformed_acl_never_passes() {
    assert!(matches!(
        evaluate_text("not-an-acl", 501, Policy::SecretFile),
        AclDecision::Unknown { .. }
    ));
}

#[test]
fn native_acl_to_text_order_is_evaluated_conservatively() {
    let text =
        "flags:0x0\nuser:alice:allow:read_data,write_data\nowner@:allow:read,write,execute\n";
    assert!(matches!(
        evaluate_text(text, 501, Policy::SecretFile),
        AclDecision::Finding { .. }
    ));
}

#[test]
fn unknown_native_permission_never_passes() {
    assert!(matches!(
        evaluate_text(
            "user:alice:allow:future_permission",
            501,
            Policy::SecretFile
        ),
        AclDecision::Unknown { .. }
    ));
}

#[test]
fn native_inheritance_flag_is_incomplete() {
    assert!(matches!(
        evaluate_text(
            "user:alice:allow:read:file_inherit",
            501,
            Policy::SecretFile
        ),
        AclDecision::Unknown { .. }
    ));
}

#[cfg(not(target_os = "macos"))]
#[test]
fn non_macos_backend_is_explicitly_unsupported() {
    let decision = tokenkeeper::acl::evaluate_path(
        std::path::Path::new("/tmp/config"),
        501,
        Policy::TrustedConfig,
    );
    assert!(matches!(decision, AclDecision::Unsupported { .. }));
}
