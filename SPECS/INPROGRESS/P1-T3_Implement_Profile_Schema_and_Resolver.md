# P1-T3 — Implement Profile Schema and Resolver

## Objective

Реализовать reusable Rust core для data-only embedded profiles и безопасного разрешения file locations из P1-T2 inventory. Core должен быть независимым от конкретных agents, не выполнять callbacks и не читать contents.

## Deliverables

- `src/profiles.rs` с typed schema для `ProfileSpec`, `LocationSpec`, semantic roots, platform, node kind, policy и traversal.
- `ProfileRegistry` с validation duplicate IDs, identifier format, empty fields, absolute paths, `..` и invalid bounds.
- `src/resolver.rs` с trusted absolute Home, roots `Home`/`XdgConfig`/`MacApplicationSupport`, exact and bounded resolution.
- `ResolvedPath` result, explicit missing-path state и errors for symlink components, invalid roots, IO failures и traversal limits.
- Integration tests в `tests/profile_resolver.rs`.

## Constraints

- Use stable Rust and standard library only for this task.
- Resolver receives a trusted Home path explicitly; it must not silently substitute an arbitrary environment path.
- Relative profile paths cannot escape their semantic root.
- Every existing path component is checked with no-follow metadata; target and intermediate symlinks stop resolution.
- Bounded traversal must enforce both maximum depth and entry count.
- Concrete Codex/Claude/OpenCode/Cursor profile definitions are deferred to P3-T1.

## Test-first plan

1. Add failing integration tests for registry validation, semantic roots, missing paths, symlink rejection, bounded traversal, and relative-home rejection.
2. Implement the smallest data-only profile schema and validation API needed by those tests.
3. Implement resolver root mapping and component-wise no-follow checks.
4. Add deterministic bounded directory walking and explicit traversal-limit errors.
5. Run Cargo tests, Clippy, formatting, and record results in the validation report.

## Acceptance criteria

- Invalid IDs, duplicate IDs, empty fields, absolute paths, parent traversal, zero depth, and zero entry limits are rejected.
- `Home`, `XdgConfig`, and `MacApplicationSupport` resolve below the supplied trusted Home.
- Missing exact locations return an explicit non-existing result.
- Target and intermediate symlinks return an error and are never traversed.
- Bounded traversal honors depth and entry limits deterministically.
- No agent-specific conditionals, executable plugin code, content reads, ACL evaluation, or remediation behavior are introduced.
- `cargo test --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` pass.

## Out of scope

Unix mode/owner policy evaluation, macOS ACL backend, CLI profile selection, concrete embedded profiles, content secret scanning, and automatic fixes.

## Notes

The archived P1-T2 inventory is the source for later profile definitions. This task establishes the schema and resolver contract those profiles will consume.
