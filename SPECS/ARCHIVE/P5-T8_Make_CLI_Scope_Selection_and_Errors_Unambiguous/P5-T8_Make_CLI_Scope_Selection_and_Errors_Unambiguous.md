# P5-T8 — Make CLI Scope Selection and Errors Unambiguous

## Objective

Reject conflicting or duplicated scope flags instead of silently selecting one branch.

## Acceptance Criteria

- Custom `--path`/`--policy` and profile scopes are mutually exclusive.
- Duplicate and empty values produce deterministic errors.
- Existing help, version, profile, and repeatable-profile behavior remains stable.

## Dependency

P5-T7 — Implement the Structured Finding and Report Contract.
