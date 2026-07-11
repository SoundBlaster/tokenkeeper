#[cfg(unix)]
pub fn current_uid() -> u32 {
    unsafe extern "C" {
        fn getuid() -> u32;
    }
    // getuid is a read-only libc query and has no ownership or allocation obligations.
    unsafe { getuid() }
}

/// Resolve the audited user's canonical home. A root/elevated invocation must
/// identify the original user explicitly; it never trusts root's `HOME`.
pub fn canonical_home() -> Result<std::path::PathBuf, String> {
    let uid = current_uid();
    if uid == 0 {
        let user = std::env::var("SUDO_USER")
            .map_err(|_| "elevated invocation requires SUDO_USER".to_owned())?;
        if user.is_empty() || user == "root" {
            return Err("elevated invocation does not identify a non-root user".into());
        }
        let output = std::process::Command::new("/usr/bin/dscl")
            .args([".", "-read", &format!("/Users/{user}"), "NFSHomeDirectory"])
            .output()
            .map_err(|error| format!("cannot resolve canonical home: {error}"))?;
        if !output.status.success() {
            return Err("directory service could not resolve canonical home".into());
        }
        let text = String::from_utf8_lossy(&output.stdout);
        let home = text
            .split_whitespace()
            .last()
            .filter(|value| value.starts_with('/'))
            .ok_or_else(|| "directory service returned no canonical home".to_owned())?;
        return Ok(std::path::PathBuf::from(home));
    }
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| "HOME is not set".to_owned())
}
