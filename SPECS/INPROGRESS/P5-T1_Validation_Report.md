# P5-T1 Validation Report

## Scope

Validated the formal `v0.1.0` post-release audit artifacts against the P5-T1 PRD. This verdict evaluates whether the audit was completed accurately; it does not approve the audited `v0.1.0` product behavior.

## Configured FLOW Quality Gates

| Command | Result |
| --- | --- |
| `cargo test --all-targets --all-features` | PASS — 39 passed, 0 failed, 0 ignored |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all --check` | PASS |

Additional type/build diagnostic:

| Command | Result |
| --- | --- |
| `cargo check --all-targets --all-features` | PASS |

## Coverage Diagnostic

The first coverage attempt used the Homebrew Rust toolchain and could not locate `llvm-tools-preview`. The repeat used the configured rustup `stable` toolchain, completed successfully, and failed the project REVIEW threshold as expected:

```text
TOTAL regions:    66.76%
TOTAL functions:  78.49%
TOTAL lines:      71.08%
Required lines:   80.00%
Result:           FAIL
```

Coverage is an additional REVIEW diagnostic, not a configured `.flow/params.yaml` EXECUTE gate. The failure is preserved as follow-up evidence and does not invalidate completion of this documentation-only audit task.

## Native macOS Reproductions

All fixtures were created below temporary `/tmp/tokenkeeper-*` directories and removed after execution.

### Target ACL

Fixture: credential mode `0600` with native ACL `everyone allow read`.

```text
custom: UNKNOWN  .../acl-target/credential
Summary: 0 passed, 0 finding(s), 1 unknown, 0 skipped
exit=2
```

Expected by PRD: deterministic non-owner read `FINDING`. The result confirms the native ACL parser gap while remaining fail-closed for this case.

### Ancestor ACL

Fixture: credential mode `0600`, parent mode `0700`, parent ACL granting `search`, `add_file`, and `delete_child` to everyone.

```text
custom: PASS  .../acl-parent/credential
Summary: 1 passed, 0 finding(s), 0 unknown, 0 skipped
exit=0
```

This is a confirmed unsafe false `PASS`.

### Credential Policy

Fixture: built-in Codex config mode `0644`.

```text
codex: PASS  .../.codex/config.toml
Summary: 1 passed, 0 finding(s), 0 unknown, 1 skipped
exit=0
```

This confirms that selecting only `ExecutableConfig` weakens the documented credential confidentiality requirement.

### Home Authority

Fixture: arbitrary empty temporary directory owned by the current UID supplied as `$HOME`.

```text
Summary: 0 passed, 0 finding(s), 0 unknown, 11 skipped
exit=0
```

The OS account Home was not consulted and the checked scope was clean despite checking no real profile location.

## Distribution and Release Evidence

| Check | Result |
| --- | --- |
| `brew style Formula/tokenkeeper.rb` | PASS |
| `brew audit --new tokenkeeper` | PASS |
| `brew test tokenkeeper` | PASS — version and profiles smoke tests |
| Clean source reinstall | NOT RUN |
| Upgrade validation | NOT RUN |
| Uninstall validation | NOT RUN |

The unrun lifecycle steps would mutate installed user software and were not authorized by this review-only task. They remain required follow-up/CI work.

Git release identity:

```text
v0.1.0 peeled commit: f3ef34015f5379c90d07362337f8cf69887edd00
main commit:          c783916ddda709bbf3b3e769eb7e64d95bf25ac2
tag ancestor of main: no
```

## Deliverable Validation

- Permanent standalone review created at `docs/v0.1.0-post-release-review.md`.
- PRD/Workplan traceability, security, code-quality, testing, CI, distribution, and release-lineage evidence recorded.
- Findings use stable audit IDs `TK-REV-001` through `TK-REV-018` with severity and remediation direction.
- Production source, tests, Formula behavior, tags, installed packages, and credentials were not modified.
- Actionable findings are ready for the required FOLLOW-UP phase after structured REVIEW.

## Verdict

**PASS** — P5-T1 successfully produced an evidence-backed audit and validation record. The audited release itself has Blocker/High findings and is not approved for acceptance closure.
