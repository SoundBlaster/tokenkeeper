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
    if fields.len() < 3 || fields.iter().any(|field| field.is_empty()) {
        return Err(format!("malformed ACL entry: {line}"));
    }
    let effect_index = fields
        .iter()
        .position(|field| matches!(field.to_ascii_lowercase().as_str(), "allow" | "deny"))
        .ok_or_else(|| format!("unknown ACL effect: {line}"))?;
    if effect_index == 0 || (effect_index + 1 >= fields.len() && effect_index < 2) {
        return Err(format!("malformed ACL entry: {line}"));
    }
    let effect_before_permissions = effect_index + 1 < fields.len();
    let permission_index = if effect_before_permissions {
        effect_index + 1
    } else {
        effect_index - 1
    };
    let principal_end = if effect_before_permissions {
        effect_index
    } else {
        permission_index
    };
    let principal = fields[..principal_end].join(":");
    let effect = match fields[effect_index].to_ascii_lowercase().as_str() {
        "allow" => Effect::Allow,
        "deny" => Effect::Deny,
        _ => unreachable!(),
    };
    let suffix_start = if effect_before_permissions {
        permission_index + 1
    } else {
        effect_index + 1
    };
    let inherited = fields[suffix_start..].iter().any(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "inherited" | "file_inherit" | "directory_inherit" | "inherit_only" | "limit_inherit"
        )
    });
    if fields[suffix_start..]
        .iter()
        .any(|value| !is_acl_flag(value))
    {
        return Err(format!("unknown ACL entry suffix: {line}"));
    }
    let permissions = fields[permission_index]
        .split([',', '/', ' ', '\t'])
        .filter(|permission| !permission.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    if permissions.is_empty()
        || permissions
            .iter()
            .any(|permission| !is_acl_permission(permission))
    {
        return Err(format!("unknown ACL permission: {line}"));
    }
    Ok(Entry {
        principal,
        effect,
        permissions: permissions.join(","),
        inherited,
    })
}

fn is_acl_permission(permission: &str) -> bool {
    matches!(
        permission,
        "read"
            | "write"
            | "execute"
            | "read_data"
            | "write_data"
            | "append_data"
            | "readattr"
            | "writeattr"
            | "readextattr"
            | "writeextattr"
            | "readsecurity"
            | "writesecurity"
            | "chown"
            | "delete"
            | "add_file"
            | "add_subdirectory"
            | "delete_child"
            | "list"
            | "search"
            | "approach"
    )
}

fn is_acl_flag(flag: &str) -> bool {
    matches!(
        flag.to_ascii_lowercase().as_str(),
        "inherited"
            | "file_inherit"
            | "directory_inherit"
            | "inherit_only"
            | "limit_inherit"
            | "no_inherit"
    )
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
    const ENOATTR: c_int = 93;
    unsafe extern "C" {
        fn acl_get_file(path: *const c_char, acl_type: c_int) -> *mut c_void;
        fn acl_to_text(acl: *mut c_void, len: *mut usize) -> *mut c_char;
        fn acl_free(value: *mut c_void) -> c_int;
        fn __error() -> *mut c_int;
    }
    let original_path = path;
    let path = path
        .to_str()
        .ok_or_else(|| "path is not valid UTF-8".to_owned())?;
    let path = CString::new(path).map_err(|_| "path contains NUL".to_owned())?;
    let acl = unsafe { acl_get_file(path.as_ptr(), ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let errno = unsafe { *__error() };
        let path_still_exists = std::fs::symlink_metadata(original_path).is_ok();
        return if errno == ENOATTR || (errno == 2 && path_still_exists) {
            Ok(None)
        } else {
            Err(format!("acl_get_file failed: errno {errno}"))
        };
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
