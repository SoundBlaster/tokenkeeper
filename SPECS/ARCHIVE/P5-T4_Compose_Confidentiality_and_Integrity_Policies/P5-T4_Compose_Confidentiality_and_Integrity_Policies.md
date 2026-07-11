# P5-T4 — Compose Confidentiality and Integrity Policies

## Objective

Represent confidentiality and integrity as independent policy requirements and assign the combined policy to credential-bearing built-in locations.

## Acceptance Criteria

- `CredentialConfig` requires both confidentiality and integrity.
- Inspector and ACL evaluation consume independent requirements.
- Codex, Claude settings, and AWS config locations containing credentials use the combined policy.
- Tests and validation document the policy mapping.

## Dependency

P5-T1 — Audit v0.1.0 Release.
