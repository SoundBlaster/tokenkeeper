# P3-T1 Validation Report

## Scope

Added typed embedded profiles for Codex, Claude Code, OpenCode, Cursor and MCP/utility credential configurations. Each profile carries research evidence metadata and uses semantic roots, optional exact locations and conservative policies. CLI `profiles` lists the registry; `check` audits all profiles or repeatable `--profile` selections.

## Checks

```text
cargo test --all-targets --all-features   PASS (32 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check   PASS
tokenkeeper profiles   PASS (five required profile IDs listed)
```

Tests verify registry validation, required IDs, evidence dates, semantic roots, policy assignment and absence of fixture secret content. Existing ACL, metadata, resolver and report suites remain green.

## Coverage boundaries

File-backed credential candidates from the research inventory are represented without reading contents. Keychain-only, environment-only and project-root-dependent storage remains explicitly outside the embedded exact-path scope. GitHub Copilot remains optional and is not claimed.

## Verdict

PASS — ready for archive and review.
