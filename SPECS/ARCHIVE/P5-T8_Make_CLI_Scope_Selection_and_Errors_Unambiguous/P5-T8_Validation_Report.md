# P5-T8 Validation Report

**Date:** 2026-07-12
**Task:** Make CLI Scope Selection and Errors Unambiguous
**Verdict:** PASS

## Quality gates

- `cargo test --all-targets --all-features` — PASS.
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all --check` — PASS.

## Acceptance evidence

- `--path` and `--policy` remain an inseparable custom scope and cannot be combined with `--profile`.
- Duplicate `--path`/`--policy` flags and empty values return explicit exit-2 errors.
- Repeatable profile selection remains supported; unknown profile and malformed scope errors remain deterministic.
