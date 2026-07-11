# P3-T1 — Add Built-in Agent and Integration Profiles

## Objective

Add validated, data-only profiles for the researched credential-bearing agent, MCP and utility configuration locations and wire them into `profiles` and `check --profile`.

## Deliverables

- Built-in registry entries for Codex, Claude Code, OpenCode, Cursor and MCP/integration utility configs.
- Evidence/source metadata and verification date for each profile.
- Appropriate `CredentialConfig`, `SecretFile`, `TrustedConfig` and `ExecutableConfig` policies with optionality.
- Profile selection in the CLI with platform filtering and stable listing output.
- Fixtures/tests proving IDs, validation, paths, policies, evidence and no secret contents.

## Constraints

- Profiles are typed data only; no product-name branches in resolver/inspector.
- Use only paths and evidence from `docs/agent-storage-locations.md`; do not read credential contents.
- Keychain-only, environment-only and project-root-dependent locations remain explicitly documented limitations.
- Every selector is exact or bounded; no unbounded scan of Home or arbitrary project trees.
- GitHub Copilot remains optional and is not required for completion.

## Test-first plan

1. Add registry and CLI tests for required IDs, evidence metadata, policies, optional missing paths and platform visibility.
2. Implement profile metadata and the embedded registry.
3. Wire profile listing and selected/all profile execution into the CLI.
4. Run Cargo tests, Clippy and formatting; record validation.

## Acceptance criteria

- `tokenkeeper profiles` lists Codex, Claude Code, OpenCode, Cursor and MCP/integration profiles.
- `tokenkeeper check --profile ID` audits the selected built-in profile without reading contents.
- Credential-bearing configs use `CredentialConfig`; executable/hook-sensitive configs use `ExecutableConfig` where appropriate.
- All locations are bounded and validated; fixtures contain no secrets.
- Optional, version-dependent, Keychain and unsupported storage are not represented as falsely checked files.

## Out of scope

ACL implementation (P2-T3), full project discovery, GitHub Copilot, content scanning, custom profile files and Linux-specific profiles.
