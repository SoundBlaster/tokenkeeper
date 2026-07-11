# P5-T5 — Resolve Canonical User Home and Elevated Invocation

## Objective

Prevent elevated or arbitrary environment context from selecting an unrelated same-UID directory for audit.

## Acceptance Criteria

- Root invocations require an explicit non-root target user and resolve its macOS Directory Services home.
- Normal invocations retain ownership checks and fail explicitly when HOME is unavailable.
- Help/version/profile commands do not require Home resolution.

## Dependency

P5-T1 — Audit v0.1.0 Release.
