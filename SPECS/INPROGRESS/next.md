# In Progress: P5-T2 — Correct Native macOS ACL Evaluation

**Priority:** P0
**Phase:** Post-release Hardening
**Effort:** 8–16 hours
**Dependencies:** P5-T1
**Status:** Selected

## Description

Correct native macOS ACL evaluation first because ancestor safety and the full security acceptance suite depend on reliable ACL acquisition, permissions, and error semantics.

## Recently Archived

- `P5-T1` — Audit v0.1.0 Release — `PASS` (2026-07-12)

## Next Step

PLAN complete: implement conservative native `acl_to_text` parsing, explicit acquisition errors, and regression fixtures.
