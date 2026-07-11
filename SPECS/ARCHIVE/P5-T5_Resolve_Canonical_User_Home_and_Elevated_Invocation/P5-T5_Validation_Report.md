# P5-T5 Validation Report

**Date:** 2026-07-12
**Task:** Resolve Canonical User Home and Elevated Invocation
**Verdict:** PASS

## Quality gates

- `cargo test --all-targets --all-features` — PASS.
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all --check` — PASS.

## Acceptance evidence

- Normal invocations use the OS user's `HOME` after ownership validation.
- UID 0 invocations reject root's ambient `HOME` and require `SUDO_USER`; the canonical path is resolved through macOS Directory Services (`dscl`) and remains read-only.
- Missing or malformed identity data produces an explicit exit-2 error instead of silently auditing an arbitrary same-UID directory.
- Help/version/profile commands remain available without resolving Home.

## Platform note

The elevated resolver uses macOS `/usr/bin/dscl`; Linux retains the non-root HOME path and explicit unsupported ACL warning.
