use std::path::Path;

use crate::profiles::Policy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AclDecision {
    NotPresent,
    Pass,
    Finding { detail: String },
    Unknown { detail: String },
    Unsupported { detail: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    principal: String,
    effect: Effect,
    permissions: String,
    inherited: bool,
}

pub fn evaluate_text(text: &str, owner_uid: u32, policy: Policy) -> AclDecision {
    let mut entries = Vec::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if line.starts_with("flags:") {
            continue;
        }
        match parse_entry(line) {
            Ok(entry) => entries.push(entry),
            Err(detail) => return AclDecision::Unknown { detail },
        }
    }
    if entries.is_empty() {
        return AclDecision::NotPresent;
    }
    let mut relevant_allow = false;
    let mut relevant_deny = false;
    let mut inherited = false;
    for entry in &entries {
        if is_owner(&entry.principal, owner_uid) || !relevant_permission(&entry.permissions, policy)
        {
            continue;
        }
        if entry.inherited {
            inherited = true;
        }
        match entry.effect {
            Effect::Allow => relevant_allow = true,
            Effect::Deny => relevant_deny = true,
        }
    }
    if !relevant_allow {
        return AclDecision::Pass;
    }
    if inherited || relevant_deny {
        return AclDecision::Unknown {
            detail: "inherited or ambiguous allow/deny ordering".into(),
        };
    }
    AclDecision::Finding {
        detail: "non-owner ACL allow grants policy-relevant access".into(),
    }
}

fn parse_entry(line: &str) -> Result<Entry, String> {
    let fields: Vec<_> = line.split(':').map(str::trim).collect();
    if fields.len() < 3 {
        return Err(format!("malformed ACL entry: {line}"));
    }
    let (principal, effect_index, permission_index) =
        if matches!(fields[1].to_ascii_lowercase().as_str(), "allow" | "deny") {
            (fields[0].to_owned(), 1, 2)
        } else if fields.len() >= 4
            && matches!(fields[2].to_ascii_lowercase().as_str(), "allow" | "deny")
        {
            (format!("{}:{}", fields[0], fields[1]), 2, 3)
        } else {
            return Err(format!("unknown ACL effect: {line}"));
        };
    if fields[permission_index].is_empty() {
        return Err(format!("malformed ACL entry: {line}"));
    }
    let effect = match fields[effect_index].to_ascii_lowercase().as_str() {
        "allow" => Effect::Allow,
        "deny" => Effect::Deny,
        _ => unreachable!(),
    };
    let inherited = fields
        .get(permission_index + 1)
        .is_some_and(|value| value.eq_ignore_ascii_case("inherited"));
    if fields.len() > permission_index + 1 && !inherited {
        return Err(format!("unknown ACL entry suffix: {line}"));
    }
    Ok(Entry {
        principal,
        effect,
        permissions: fields[permission_index].to_ascii_lowercase(),
        inherited,
    })
}

fn is_owner(principal: &str, owner_uid: u32) -> bool {
    principal == "owner"
        || principal == "owner@"
        || principal == "user::"
        || principal == format!("user:{owner_uid}")
}

fn relevant_permission(permissions: &str, policy: Policy) -> bool {
    match policy {
        Policy::SecretFile | Policy::CredentialConfig | Policy::PrivateDirectory => {
            permissions.contains("read")
                || permissions.contains("write")
                || permissions.contains("execute")
        }
        Policy::TrustedConfig | Policy::ExecutableConfig => permissions.contains("write"),
    }
}

#[cfg(target_os = "macos")]
pub fn evaluate_path(path: &Path, owner_uid: u32, policy: Policy) -> AclDecision {
    match read_acl_text(path) {
        Ok(None) => AclDecision::NotPresent,
        Ok(Some(text)) => evaluate_text(&text, owner_uid, policy),
        Err(detail) => AclDecision::Unknown { detail },
    }
}

#[cfg(not(target_os = "macos"))]
pub fn evaluate_path(_path: &Path, _owner_uid: u32, _policy: Policy) -> AclDecision {
    AclDecision::Unsupported {
        detail: "macOS extended ACL backend is unavailable on this platform".into(),
    }
}

#[cfg(target_os = "macos")]
fn read_acl_text(path: &Path) -> Result<Option<String>, String> {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_void};
    const ACL_TYPE_EXTENDED: c_int = 0x0000_0100;
    unsafe extern "C" {
        fn acl_get_file(path: *const c_char, acl_type: c_int) -> *mut c_void;
        fn acl_to_text(acl: *mut c_void, len: *mut usize) -> *mut c_char;
        fn acl_free(value: *mut c_void) -> c_int;
    }
    let path = path
        .to_str()
        .ok_or_else(|| "path is not valid UTF-8".to_owned())?;
    let path = CString::new(path).map_err(|_| "path contains NUL".to_owned())?;
    let acl = unsafe { acl_get_file(path.as_ptr(), ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        return Ok(None);
    }
    let mut length = 0usize;
    let text = unsafe { acl_to_text(acl, &mut length) };
    if text.is_null() {
        unsafe {
            acl_free(acl);
        }
        return Err("acl_to_text failed".into());
    }
    let value = unsafe { CStr::from_ptr(text) }
        .to_str()
        .map_err(|_| "ACL text is not UTF-8".to_owned())
        .map(str::to_owned);
    unsafe {
        acl_free(text.cast());
        acl_free(acl);
    }
    value.map(Some)
}
