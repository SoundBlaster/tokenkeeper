# In Progress: P5-T6 — Make Bounded Traversal Fail-closed and Resource-bounded

**Priority:** P0
**Phase:** Post-release Hardening
**Effort:** 8–16 hours
**Dependencies:** P5-T1
**Status:** Selected

## Description

Make bounded traversal fail closed without partial success and enforce resource bounds.

## Recently Archived

- `P5-T1` — Audit v0.1.0 Release — `PASS` (2026-07-12)
- `P5-T2` — Correct Native macOS ACL Evaluation — `PASS with repository coverage gap` (2026-07-12)
- `P5-T3` — Inspect the Complete Anchored Ancestor Chain — `PASS` (2026-07-12)
- `P5-T4` — Compose Confidentiality and Integrity Policies — `PASS` (2026-07-12)
- `P5-T5` — Resolve Canonical User Home and Elevated Invocation — `PASS` (2026-07-12)

## Next Step

PLAN: stream bounded directory traversal, enforce entry limits before emitting paths, and preserve fail-closed errors.
