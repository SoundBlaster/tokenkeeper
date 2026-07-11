# P5-T3 Validation Report

**Date:** 2026-07-12
**Task:** Inspect the Complete Anchored Ancestor Chain
**Verdict:** PASS

## Quality gates

- `cargo test --all-targets --all-features` — PASS (all tests passed).
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all --check` — PASS.

## Acceptance evidence

- Ancestors are inspected from target parent through canonical Home, including ownership, directory type, mode, metadata errors, and ACL replacement/search rights.
- `add_file`, `add_subdirectory`, `delete_child`, `search`, and write ACL permissions are policy-relevant and produce ancestor findings.
- Ancestor symlinks, non-directories, foreign owners, writable modes, and metadata failures are surfaced as `FindingReason::AncestorUnsafe`/`WritableAncestor`; target findings remain in the same result.
- ACL errors are retained as incomplete ancestor findings; non-macOS unsupported behavior remains explicit and ignored only on the platform where ACLs are unavailable.
- No target content or metadata is mutated.

## Limitations

The current test runner is macOS, so native ACL acquisition is available. A fully adversarial parent ACL fixture still requires platform-specific ACL setup and remains part of P5-T11's acceptance suite.
