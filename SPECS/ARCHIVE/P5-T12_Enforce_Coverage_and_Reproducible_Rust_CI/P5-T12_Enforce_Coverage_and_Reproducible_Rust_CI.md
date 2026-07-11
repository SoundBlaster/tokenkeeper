# P5-T12 — Enforce Coverage and Reproducible Rust CI

## Objective

Pin the Rust toolchain and CI action, run locked quality gates, and enforce 80% line coverage.

## Acceptance Criteria

- CI runs format, test, check, build, Clippy, and coverage with locked dependencies.
- Toolchain and checkout action are reproducibly pinned.
- Cargo metadata declares MIT licensing and supported Rust version.
- Coverage is at least 80% for the audited project scope.

## Dependency

P5-T11 — Build the Native Security Acceptance Suite.
