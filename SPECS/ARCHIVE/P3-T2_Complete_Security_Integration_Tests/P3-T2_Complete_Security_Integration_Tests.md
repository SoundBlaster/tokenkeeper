# P3-T2 — Complete Security Integration Tests

## Objective

Exercise the complete read-only audit path against adversarial temporary filesystem trees and malformed CLI input, proving the PRD security invariants rather than only the happy-path policy matrix.

## Deliverables

- Integration tests for symlink escape, symlink loops, writable ancestors, ACL ambiguity, odd filenames and traversal limits.
- Tests proving target contents are not required, files are not mutated, and no remediation command is executed.
- CLI malformed-input and profile-selection tests with no panic.
- Validation report documenting the complete suite and known platform boundaries.

## Constraints

- Use temporary fixtures only; never copy real home files or credentials.
- Tests must not invoke network services or external remediation commands.
- Keep assertions on metadata/results and before/after filesystem state.
- macOS ACL fixtures use the pure evaluator where privileged ACL mutation is unavailable; real FFI coverage remains compile/runtime gated by macOS.

## Test-first plan

1. Add adversarial resolver and inspector tests for escape/loop/symlink behavior, traversal bounds and unusual filenames.
2. Add no-content-read/no-mutation and malformed CLI tests.
3. Run the full Cargo suite, Clippy and formatting, then audit PRD acceptance coverage.

## Acceptance criteria

- Symlink escape and loop attempts never resolve outside trusted Home or follow targets.
- Traversal limits produce an incomplete/error result rather than partial pass.
- Spaces, quotes, leading dash, newline and ANSI/control filenames cannot inject output or commands.
- No target content read or filesystem mutation is needed for a passing audit.
- Malformed input does not panic and returns exit code 2.

## Out of scope

New product profiles, ACL semantics implementation, content secret scanning, network tests and Linux-specific acceptance.

---
**Archived:** 2026-07-11
**Verdict:** PASS
