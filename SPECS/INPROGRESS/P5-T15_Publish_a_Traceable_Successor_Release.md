# P5-T15 — Publish a Traceable Successor Release

## Objective

Publish the post-release hardening work as a new immutable version without moving `v0.1.0`.

## Acceptance Criteria

- `Cargo.toml`, lockfile, binary, tag, release archive, Formula, checksum, and docs all identify `0.2.0`.
- `v0.1.0` remains unchanged and documented as superseded.
- Full Rust, native security, coverage, and Homebrew lifecycle gates pass before publication.
- Successor tag points to the reviewed main-line tree.

## Dependencies

P5-T7 through P5-T14.
