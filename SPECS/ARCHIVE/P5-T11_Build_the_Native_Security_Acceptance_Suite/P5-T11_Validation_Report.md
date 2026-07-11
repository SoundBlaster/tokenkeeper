# P5-T11 Validation Report

**Date:** 2026-07-12
**Task:** Build the Native Security Acceptance Suite
**Verdict:** PASS

## Quality gates

- `cargo test --all-targets --all-features` — PASS (including two native macOS ACL fixtures).
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all --check` — PASS.

## Acceptance evidence

- macOS-only tests create real extended ACLs with `chmod +a`, verify target non-owner allows as findings, and remove the ACL afterward.
- A parent `add_file` ACL is surfaced as `AncestorAclAccess` while retaining the target result.
- Existing suites cover symlink escape, ownership/mode policies, bounded traversal, structured reporting, and non-mutation.
- Native fixture cleanup is performed after assertions; fixtures contain no real credential data.
