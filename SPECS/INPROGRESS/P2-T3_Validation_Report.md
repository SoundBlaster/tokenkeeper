# P2-T3 Validation Report

## Scope

Implemented the macOS ACL backend and composed it with metadata inspection. ACL text is parsed conservatively: relevant non-owner `ALLOW` entries become findings, while inherited entries, ambiguous deny/allow ordering and backend/parser failures become unknown. macOS FFI declarations and cleanup are isolated behind `cfg(target_os = "macos")`; non-macOS builds report unsupported coverage.

## Checks

```text
cargo test --all-targets --all-features   PASS (29 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check   PASS
```

Pure ACL fixtures cover owner-only entries, non-owner read/write allows, inherited entries, deny/allow ambiguity and malformed text. Existing metadata and CLI suites remain green.

## Acceptance

No ambiguous ACL state can produce `PASS`; unsupported and failed ACL inspection becomes `UNKNOWN`. The backend performs no content reads, subprocesses or mutations. ACL rules are evaluated in addition to Unix mode checks, and remediation is still suppressed for unsafe context.

## Platform note

The active target is macOS and includes the isolated FFI adapter. Linux ACL semantics remain explicitly unsupported and are evaluated separately in P4-T3.

## Verdict

PASS — ready for archive and review.
