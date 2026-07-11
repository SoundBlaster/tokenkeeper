# P4-T1 Validation Report

## Scope

Published README usage/install guidance and `docs/security.md` covering supported profiles, report statuses, manual remediation, privacy boundary, threat model and point-in-time limitations.

## Checks

```text
cargo test --all-targets --all-features   PASS (37 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check   PASS
```

Documentation review confirms commands match the current CLI, Homebrew is not claimed before P4-T2, and examples contain no real usernames, tokens or credential values.

## Coverage notes

README points to the research inventory and security guide. The guide explicitly covers same-UID/root limits, Keychain, backup/sync, race windows, unsupported Linux ACL semantics, manual remediation and no-content/no-network behavior.

## Verdict

PASS — ready for archive and review.
