# P5-T2 — Correct Native macOS ACL Evaluation

## Objective

Replace synthetic-only ACL assumptions with conservative parsing of the native macOS `acl_to_text` representation. Any acquisition or conversion failure must be reported as incomplete (`Unknown`) and never as an absent ACL or a passing check.

## Scope

- `src/acl.rs`: native ACL acquisition, lifetime handling, and text parser.
- `tests/acl.rs`: native grammar regression fixtures and malformed/error semantics.
- No production behavior changes on non-macOS platforms beyond preserving explicit `Unsupported`.

## Deliverables

1. Parse both documented synthetic fixtures and macOS native entry ordering (`principal:permissions:allow|deny[:flags]`).
2. Recognize native permission tokens conservatively, including data-directory forms, and reject unknown tokens.
3. Distinguish a missing extended ACL from `acl_get_file` failure using `errno`; conversion failures return `Unknown`.
4. Release ACL and text allocations on every native path; keep FFI isolated behind the macOS backend.
5. Add tests proving non-owner native allow is a finding and malformed/unknown native input cannot pass.

## Acceptance Criteria

- A native non-owner read/write allow produces a policy-appropriate finding.
- Header, qualifier, flags, permission, inherited, allow, and deny forms are handled conservatively without relying on synthetic-only grammar.
- `acl_get_file`/conversion failures, unsupported filesystems, permission denial, allocation errors, and target disappearance produce `Unknown`, never `NotPresent` or `Pass`.
- Native ACL allocations and qualifiers are released on all paths.
- `cargo test`, `cargo clippy -D warnings`, and `cargo fmt --check` pass on the supported host; macOS-only paths remain compile-gated.

## Dependencies

- P5-T1 — Audit v0.1.0 Release.

## Risks and validation

The Linux runner cannot execute macOS ACL syscalls, so macOS behavior is validated through parser fixtures and compile-gated FFI code. The validation report must explicitly identify this platform boundary.
