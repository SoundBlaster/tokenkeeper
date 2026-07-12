# P5-T16 — Publish Architecture-aware Homebrew Distribution v0.2.3

**Priority:** P0
**Phase:** Post-release Hardening
**Effort:** 8–16 hours
**Dependencies:** P5-T1
**Status:** INPROGRESS

## Description

Publish a new immutable release because the Homebrew distribution path changed after `v0.2.2`: Apple Silicon will consume a prebuilt release asset while Intel retains a source-build fallback. The existing `v0.2.2` tag and release remain unchanged.

## Recently Archived

- `P5-T1` — Audit v0.1.0 Release — `PASS` (2026-07-12)
- `P5-T2` — Correct Native macOS ACL Evaluation — `PASS with repository coverage gap` (2026-07-12)
- `P5-T3` — Inspect the Complete Anchored Ancestor Chain — `PASS` (2026-07-12)
- `P5-T4` — Compose Confidentiality and Integrity Policies — `PASS` (2026-07-12)
- `P5-T5` — Resolve Canonical User Home and Elevated Invocation — `PASS` (2026-07-12)
- `P5-T6` — Make Bounded Traversal Fail-closed and Resource-bounded — `PASS` (2026-07-12)
- `P5-T7` — Implement the Structured Finding and Report Contract — `PASS` (2026-07-12)
- `P5-T8` — Make CLI Scope Selection and Errors Unambiguous — `PASS` (2026-07-12)
- `P5-T9` — Implement Platform-aware Profile Availability — `PASS` (2026-07-12)
- `P5-T10` — Complete Per-location Profile Evidence and Validation — `PASS` (2026-07-12)
- `P5-T11` — Build the Native Security Acceptance Suite — `PASS` (2026-07-12)
- `P5-T12` — Enforce Coverage and Reproducible Rust CI — `PASS` (2026-07-12)
- `P5-T13` — Automate the Homebrew Release Lifecycle — `PASS` (2026-07-12)
- `P5-T14` — Reconcile the Specification Lifecycle — `PASS` (2026-07-12)
- `P5-T15` — Publish a Traceable Successor Release — `PASS` (2026-07-12)

## Deliverables

- `v0.2.3` release with an Apple Silicon asset
- Formula and tap documentation updated to `v0.2.3`
- Validation report covering Cargo, release, and Homebrew gates
