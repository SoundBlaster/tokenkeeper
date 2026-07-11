# P4-T3 — Evaluate Linux Support

## Objective

Verify that the Unix resolver, metadata policy core and CLI compile/test on Linux, while making the unsupported macOS ACL backend an explicit incomplete result rather than claiming full Linux support.

## Deliverables

- Linux CI job running the core Rust tests and quality gates.
- Non-macOS behavior that reports ACL coverage as unsupported/incomplete while preserving mode/resolver diagnostics.
- Platform support documentation distinguishing macOS supported ACL behavior from Linux best-effort mode-only behavior.
- Validation evidence for local target availability and CI configuration.

## Constraints

- Do not claim Linux ACL support without native ACL semantics/tests.
- Do not silently convert unsupported ACL evaluation to a security `PASS` in CLI output.
- Keep macOS FFI isolated and unchanged; no Linux-specific unsafe ABI is introduced.

## Test-first plan

1. Add platform-aware tests for unsupported ACL reporting and preserve mode/resolver tests.
2. Add Ubuntu CI matrix job for test, Clippy and formatting.
3. Document supported/best-effort status and run local checks where a Linux target is installed.

## Acceptance criteria

- Unix core builds and tests on Linux CI.
- Linux ACL behavior is explicitly reported unsupported/incomplete; macOS ACL remains the only full backend.
- Documentation distinguishes supported behavior from best-effort compilation.

## Out of scope

Linux POSIX/NFS ACL implementation, distribution packaging and changes to macOS ACL semantics.

---
**Archived:** 2026-07-11
**Verdict:** PASS
