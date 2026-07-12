# Workplan Complete — successor release v0.2.3 published

**Priority:** P0
**Phase:** Post-release Hardening
**Effort:** 8–16 hours
**Dependencies:** P5-T1
**Status:** Complete

## Description

Publish a new immutable release because the Homebrew distribution path changed after `v0.2.2`: Apple Silicon will consume a prebuilt release asset while Intel retains a source-build fallback. The existing `v0.2.2` tag and release remain unchanged.

## Recently Archived

- `P5-T1`–`P5-T15` — Phase 5 hardening tasks — archived (2026-07-12)
- `P5-T16` — Publish Architecture-aware Homebrew Distribution v0.2.3 — `PASS with host-toolchain caveat` (2026-07-12)

## Deliverables

- `v0.2.3` release with an Apple Silicon asset
- Formula and tap documentation updated to `v0.2.3`
- Validation report covering Cargo, release, and Homebrew gates

No open Workplan tasks remain.
