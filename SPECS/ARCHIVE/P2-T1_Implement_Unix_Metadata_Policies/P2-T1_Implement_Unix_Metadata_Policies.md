# P2-T1 — Implement Unix Metadata Policies

## Objective

Реализовать platform-neutral Unix metadata inspector для profile locations. Inspector должен проверять owner UID, node type, permission bits, target symlinks и writable ancestors, не читая содержимое файлов и не оценивая ACL до отдельного P2-T3 backend.

## Deliverables

- `src/inspector.rs` с `MetadataInspector`, metadata summary и explicit result states.
- Policy evaluation для `SecretFile`, `CredentialConfig`, `PrivateDirectory`, `TrustedConfig` и `ExecutableConfig`.
- Distinct results для `MissingOptional`, `MissingRequired`, `AccessDenied`, `Finding`, `Unknown` и mode-only `Pass`.
- Finding reasons для wrong owner, unexpected node type, group/other access, group/other write, writable ancestor и symlink component.
- Resolver `AccessDenied` distinction, если metadata/read_dir получает permission error.
- Integration tests for temporary Unix trees, modes, owner mismatch, ancestors, missing paths and node kinds.

## Constraints

- Use `symlink_metadata` and Unix `MetadataExt`/`PermissionsExt`; never open target contents.
- Inspector receives trusted Home and expected owner UID explicitly; do not infer identity from arbitrary `$HOME`.
- All evaluated paths must remain below trusted Home.
- Target and intermediate symlinks are findings and are never followed.
- `group/others` write on an ancestor is an integrity finding; ACL is not evaluated yet and must be documented as a later completeness layer.
- No CLI output, remediation commands, Keychain access, network, subprocesses, or concrete agent profiles in this task.

## Test-first execution plan

1. Add failing integration tests for safe/unsafe SecretFile, CredentialConfig, PrivateDirectory and TrustedConfig modes.
2. Add tests for wrong owner, wrong node kind, missing optional/required paths, writable ancestors and symlink results.
3. Implement metadata summary, result states and policy evaluator using no-follow metadata.
4. Reuse resolver outputs and map permission errors without collapsing them into missing paths.
5. Run Cargo tests, Clippy, formatting and record the validation report.

## Acceptance criteria

- `0600` and `0400` SecretFile/CredentialConfig pass mode checks; `0640`, `0604` and `0666` produce findings.
- `0700` PrivateDirectory passes; `0750`, `0707` and `0777` produce findings.
- `0644` TrustedConfig is allowed; group/other write produces a finding.
- Wrong owner and unexpected node type produce explicit findings.
- Writable ancestors are reported, especially for `ExecutableConfig`.
- Missing optional and missing required locations remain distinct.
- Access denied is not reported as missing or pass.
- Target/intermediate symlinks are reported without traversal.
- All Cargo quality gates pass; no ACL `PASS` claim is made before P2-T3.

## Out of scope

macOS extended ACL evaluation, report rendering, exit-code aggregation, profile selection, content secret scanning, remediation commands, automatic fixes and Linux-specific ACL behavior.

## Notes

The inspector API should be reusable by P2-T2 reporting and P2-T3 ACL composition. Keep metadata types stable and avoid embedding product names in policy code.

---
**Archived:** 2026-07-11
**Verdict:** PASS
