# P2-T3 — Implement macOS ACL Backend

## Objective

Add an isolated macOS ACL evaluator that composes with Unix mode inspection and never turns an unproven ACL state into `PASS`.

## Deliverables

- `src/acl.rs` with a platform-neutral decision model and macOS-only FFI adapter.
- Conservative parsing of extended ACL entries for non-owner allow/deny, inherited and ambiguous rules.
- Inspector integration: incompatible non-owner allow becomes a finding; ambiguous or failed ACL inspection becomes `UNKNOWN`.
- Non-macOS fallback that reports the backend as unsupported/unknown rather than claiming ACL coverage.
- Fixture tests for owner-only ACL, non-owner read/write allow, inherited entries and deny/allow ambiguity.

## Constraints

- Keep all `unsafe` and C ABI declarations inside the macOS adapter module; document ownership and `acl_free` cleanup.
- Use no subprocess, shell command, network, target-content read or filesystem mutation.
- Do not infer effective access from a partial ACL. Unknown identity, inherited ordering, parse failure or backend failure is never a pass.
- Preserve the existing mode-only policy behavior when no extended ACL is present.

## Test-first plan

1. Add pure parser/evaluator fixtures for secret/private read access and trusted/executable write access.
2. Add tests proving deny/allow ordering, inherited entries and malformed text become unknown.
3. Implement the decision model and macOS FFI text acquisition with explicit cleanup.
4. Compose ACL decisions into `MetadataInspector` results and preserve existing report statuses.
5. Run Cargo tests, Clippy and formatting; record platform limitations in validation.

## Acceptance criteria

- Relevant non-owner allow entries produce `FINDING` for the affected policy.
- Inherited or ambiguous allow/deny ordering never produces a false `PASS`.
- ACL read/parse failures produce `UNKNOWN/INCOMPLETE`.
- FFI is isolated, documented and covered by parser tests plus macOS compile/integration checks.

## Out of scope

Linux ACL semantics, profile data, JSON output, remediation execution and ACL mutation.
