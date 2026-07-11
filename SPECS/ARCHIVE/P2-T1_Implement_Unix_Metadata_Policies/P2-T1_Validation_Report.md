# P2-T1 Validation Report

## Scope

Implemented Unix metadata inspection for profile locations without opening or reading target file contents. The inspector evaluates owner UID, node kind, mode bits, symlink components, and writable ancestors.

## Checks

```text
cargo test --all-targets --all-features   PASS (20 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check   PASS
```

Integration coverage includes safe and unsafe secret/config modes, private directories, trusted configs, wrong owners and node types, missing required/optional paths, writable ancestors, and target symlinks.

## Acceptance

All P2-T1 acceptance criteria pass. Permission-denied resolver errors are represented separately from missing paths. The implementation uses `symlink_metadata` and never follows target or intermediate symlinks.

## Security boundary

`Pass` is mode-only for this task. macOS extended ACL evaluation is intentionally deferred to P2-T3; no result claims complete ACL coverage.

## Verdict

PASS — ready for archive and review.
