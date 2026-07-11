# P4-T3 Validation Report

## Scope

Evaluated Linux portability of the Unix resolver, metadata policy core and CLI. Added Ubuntu/macOS CI matrix coverage, explicit non-macOS ACL unsupported behavior, and platform documentation.

## Checks

```text
cargo test --all-targets --all-features (macOS)   PASS (37 tests)
cargo clippy --all-targets --all-features -- -D warnings (macOS)   PASS
cargo fmt --all --check                           PASS
cargo check --target x86_64-unknown-linux-gnu --all-targets --all-features   PASS
cargo clippy --target x86_64-unknown-linux-gnu --all-targets --all-features -- -D warnings   PASS
```

The Linux target test binary cannot link on macOS because the Darwin linker rejects Linux ELF flags; the real Ubuntu CI matrix job is the authoritative Linux runtime test.

## Support decision

Linux is best-effort mode/resolver support only. The macOS extended ACL backend returns explicit `Unsupported`; Linux CLI checks print an incomplete-audit warning and exit `2`, so unsupported ACL coverage cannot appear as a clean `PASS`. Full Linux ACL semantics are not claimed.

## Verdict

PASS — Linux evaluation complete with documented best-effort boundary; ready for archive and review.
