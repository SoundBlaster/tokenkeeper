# P2-T2 — Implement CLI Reporting and Guidance

## Objective

Expose the audit core through a minimal, manual CLI that reports stable statuses and prints conservative remediation guidance without executing commands.

## Deliverables

- Typed CLI parser for `check`, repeatable `--profile`, `--path`, `--policy`, `profiles`, help and version.
- Human-readable report model with `PASS`, `FINDING`, `UNKNOWN`, `SKIP`, summary counts and exit-code mapping.
- Shell-safe remediation rendering for unambiguous mode findings only.
- Custom path execution using the trusted Home and metadata inspector; profile execution remains registry-driven.
- Tests for malformed arguments, path/policy pairing, shell metacharacters/control characters, status summaries and exit codes.

## Constraints

- Never execute, mutate, or suggest `sudo`, `chmod -R`, recursive ACL removal, or automatic `chown`.
- Never read target contents, credentials, Keychain records, or invoke a network/subprocess.
- Absolute paths and terminal control characters must be handled without injection; if lossless shell escaping is impossible, omit a command.
- Missing optional locations map to `SKIP`; missing required and operationally incomplete states map to exit code 2.
- Keep profile-specific data outside the CLI parser and reporting core.

## Test-first plan

1. Add parser tests for commands, repeatable options, policy names, invalid combinations and unknown flags.
2. Add report tests for all statuses, stable summaries, exit codes and safe/unsafe remediation paths.
3. Implement typed parser and reporting/guidance modules.
4. Wire `main` for custom checks and profile listing while preserving the no-argument help contract.
5. Run Cargo tests, Clippy and formatting; record the result in the validation report.

## Acceptance criteria

- `tokenkeeper check`, `tokenkeeper check --profile ID`, `tokenkeeper check --path PATH --policy POLICY`, and `tokenkeeper profiles` parse deterministically.
- Results distinguish PASS, FINDING, UNKNOWN and SKIP; exit codes are 0 clean, 1 findings, 2 incomplete/error.
- Guidance is printed only for absolute, regular no-symlink targets with a provably safe minimal mode command.
- Spaces, quotes, control characters and shell metacharacters cannot inject commands or terminal output.
- Tokenkeeper never executes remediation.

## Out of scope

Built-in agent profiles (P3-T1), macOS ACL evaluation (P2-T3), JSON/SARIF output, content scanning, automatic fixes and Linux-specific behavior.
