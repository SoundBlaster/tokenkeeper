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
