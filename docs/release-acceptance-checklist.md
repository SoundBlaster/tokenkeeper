# Release Acceptance Checklist

This checklist is authoritative for successor releases after `v0.1.0`.

- [ ] PRD and Workplan status agree; every Phase 5 task is archived with validation and review.
- [ ] `cargo +1.85.0 fmt --all --check`, locked test/check/build/Clippy, and LLVM coverage (≥80%) pass.
- [ ] Native macOS ACL, ancestor, identity, policy, traversal, CLI, and structured-report acceptance suites pass.
- [ ] Formula style/audit/test pass; macOS Intel and Apple Silicon Homebrew lifecycle jobs are green.
- [ ] Version, binary `--version`, immutable tag, source archive checksum, Formula URL/checksum, and docs agree.
- [ ] Existing `v0.1.0` tag is unchanged and explicitly marked superseded.
- [ ] Release notes document unsupported platforms, point-in-time scope, Keychain limitations, and root/sudo semantics.
