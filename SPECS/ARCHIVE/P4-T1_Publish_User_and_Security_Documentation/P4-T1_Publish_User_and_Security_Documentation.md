# P4-T1 — Publish User and Security Documentation

## Objective

Document installation/development use, supported audit scope, report interpretation, limitations and the manual remediation workflow without exposing credentials.

## Deliverables

- README quick start for source builds and current CLI commands.
- `docs/security.md` threat model, privacy boundary and point-in-time limitations.
- Supported profile/evidence and platform status pointers.
- Manual remediation examples that users review and execute themselves.

## Constraints

- Do not claim Homebrew availability before P4-T2 release work is complete.
- Examples use placeholders and fixture paths only; no real usernames, tokens or credentials.
- Explain same-UID, root, Keychain, backup/sync, ACL and race limitations explicitly.

## Test-first plan

1. Add documentation and security guide with command examples and output meanings.
2. Check every command against the current CLI and every support claim against the research inventory/workplan.
3. Search examples for secrets/usernames and run the full code quality gates.

## Acceptance criteria

- README has source installation and minimal `profiles`/`check` examples.
- Security documentation covers supported scope, manual remediation, threat model and known limitations.
- Evidence and platform status are discoverable; examples contain no real sensitive data.

## Out of scope

Homebrew formula/release (P4-T2), Linux support decision (P4-T3), new code features and automatic remediation.
