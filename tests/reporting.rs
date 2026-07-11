use std::path::PathBuf;

use tokenkeeper::inspector::{FindingReason, InspectionResult, MetadataSummary, NodeType};
use tokenkeeper::profiles::Policy;
use tokenkeeper::report::{
    label, remediation, status_of, structured_findings, Severity, Status, Summary,
};

fn finding(path: &str, reasons: Vec<FindingReason>) -> InspectionResult {
    InspectionResult::Finding {
        path: PathBuf::from(path),
        metadata: Some(MetadataSummary {
            uid: 1,
            gid: 1,
            mode: 0o644,
            node: NodeType::RegularFile,
        }),
        reasons,
    }
}

#[test]
fn statuses_and_exit_codes_are_stable() {
    assert_eq!(label(Status::Pass), "PASS");
    let mut summary = Summary::default();
    summary.add(&InspectionResult::Pass {
        path: "/tmp/a".into(),
        metadata: MetadataSummary {
            uid: 1,
            gid: 1,
            mode: 0o600,
            node: NodeType::RegularFile,
        },
    });
    summary.add(&InspectionResult::MissingOptional {
        path: "/tmp/missing".into(),
    });
    assert_eq!(
        status_of(&InspectionResult::MissingOptional {
            path: "/tmp/missing".into()
        }),
        Status::Skip
    );
    assert_eq!(summary.exit_code(), 0);
    summary.add(&InspectionResult::MissingRequired {
        path: "/tmp/required".into(),
    });
    assert_eq!(summary.exit_code(), 2);
}

#[test]
fn remediation_is_minimal_and_shell_safe() {
    let result = finding(
        "/tmp/credentials with 'quote'",
        vec![FindingReason::GroupOrOtherAccess { mode: 0o644 }],
    );
    let command = remediation(&result, Policy::CredentialConfig).expect("safe command");
    assert!(command.starts_with("chmod go-rwx '/tmp/credentials with "));
    assert!(command.contains("\\'"));
    let control = finding(
        "/tmp/line\nfeed",
        vec![FindingReason::GroupOrOtherAccess { mode: 0o644 }],
    );
    assert!(remediation(&control, Policy::CredentialConfig).is_none());
}

#[test]
fn unsafe_context_suppresses_remediation() {
    let result = finding(
        "/tmp/config",
        vec![FindingReason::WritableAncestor {
            path: "/tmp".into(),
            mode: 0o777,
        }],
    );
    assert!(remediation(&result, Policy::TrustedConfig).is_none());
}

#[test]
fn acl_findings_suppress_chmod_guidance() {
    let result = finding(
        "/tmp/config",
        vec![FindingReason::AclNonOwnerAccess {
            detail: "foreign read".into(),
        }],
    );
    assert!(remediation(&result, Policy::CredentialConfig).is_none());
}

#[test]
fn findings_have_stable_structured_contract() {
    let result = finding(
        "/tmp/config",
        vec![FindingReason::GroupOrOtherWrite { mode: 0o666 }],
    );
    let records = structured_findings(&result);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].rule_id, "TK-META-003");
    assert_eq!(records[0].severity, Severity::High);
    assert!(records[0].current.contains("666"));
    assert!(records[0].expected.contains("write"));
    assert!(records[0].scope.contains("/tmp/config"));
}
