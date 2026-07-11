use std::path::Path;

use crate::inspector::{FindingReason, InspectionResult, NodeType};
use crate::profiles::{NodeKind, Policy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Finding,
    Unknown,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Summary {
    pub pass: usize,
    pub finding: usize,
    pub unknown: usize,
    pub skip: usize,
}

impl Summary {
    pub fn add(&mut self, result: &InspectionResult) {
        match status_of(result) {
            Status::Pass => self.pass += 1,
            Status::Finding => self.finding += 1,
            Status::Unknown => self.unknown += 1,
            Status::Skip => self.skip += 1,
        }
    }
    pub fn exit_code(self) -> u8 {
        if self.unknown > 0 {
            2
        } else if self.finding > 0 {
            1
        } else {
            0
        }
    }
}

pub fn status_of(result: &InspectionResult) -> Status {
    match result {
        InspectionResult::Pass { .. } => Status::Pass,
        InspectionResult::Finding { .. } => Status::Finding,
        InspectionResult::MissingOptional { .. } => Status::Skip,
        InspectionResult::MissingRequired { .. }
        | InspectionResult::AccessDenied { .. }
        | InspectionResult::Unknown { .. } => Status::Unknown,
    }
}

pub fn label(status: Status) -> &'static str {
    match status {
        Status::Pass => "PASS",
        Status::Finding => "FINDING",
        Status::Unknown => "UNKNOWN",
        Status::Skip => "SKIP",
    }
}

pub fn summary_line(summary: Summary) -> String {
    format!(
        "Summary: {} passed, {} finding(s), {} unknown, {} skipped",
        summary.pass, summary.finding, summary.unknown, summary.skip
    )
}

pub fn shell_escape(path: &Path) -> Option<String> {
    let value = path.to_str()?;
    if value.chars().any(char::is_control) {
        return None;
    }
    Some(format!("'{}'", value.replace('\'', "'\\''")))
}

pub fn remediation(result: &InspectionResult, policy: Policy) -> Option<String> {
    let InspectionResult::Finding {
        path,
        metadata: Some(metadata),
        reasons,
    } = result
    else {
        return None;
    };
    if !path.is_absolute()
        || metadata.node == NodeType::Symlink
        || !matches!(metadata.node, NodeType::RegularFile | NodeType::Directory)
    {
        return None;
    }
    if reasons.iter().any(|reason| {
        matches!(
            reason,
            FindingReason::WrongOwner { .. }
                | FindingReason::UnexpectedNodeType { .. }
                | FindingReason::WritableAncestor { .. }
                | FindingReason::SymlinkComponent { .. }
        )
    }) {
        return None;
    }
    let escaped = shell_escape(path)?;
    let mode = match policy {
        Policy::SecretFile | Policy::CredentialConfig | Policy::PrivateDirectory => "go-rwx",
        Policy::TrustedConfig | Policy::ExecutableConfig => "go-w",
    };
    Some(format!("chmod {mode} {escaped}"))
}

pub fn render(result: &InspectionResult, policy: Option<Policy>) -> String {
    let mut output = format!(
        "{}  {}\n",
        label(status_of(result)),
        display_path(result_path(result))
    );
    if let InspectionResult::Finding { reasons, .. } = result {
        output.push_str(&format!("         reasons: {reasons:?}\n"));
    }
    if let Some(policy) = policy.and_then(|policy| remediation(result, policy)) {
        output.push_str(&format!("         suggested: {policy}\n"));
    }
    output
}

fn display_path(path: &Path) -> String {
    let Some(value) = path.to_str() else {
        return "<non-utf8 path>".into();
    };
    value
        .chars()
        .map(|character| match character {
            '\n' => "\\n".to_owned(),
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            character if character.is_control() => format!("\\u{{{:x}}}", character as u32),
            character => character.to_string(),
        })
        .collect()
}

fn result_path(result: &InspectionResult) -> &Path {
    match result {
        InspectionResult::Pass { path, .. }
        | InspectionResult::Finding { path, .. }
        | InspectionResult::MissingOptional { path }
        | InspectionResult::MissingRequired { path }
        | InspectionResult::AccessDenied { path, .. }
        | InspectionResult::Unknown { path, .. } => path,
    }
}

#[allow(dead_code)]
fn _node_kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::File => "file",
        NodeKind::Directory => "directory",
        NodeKind::Either => "either",
    }
}
