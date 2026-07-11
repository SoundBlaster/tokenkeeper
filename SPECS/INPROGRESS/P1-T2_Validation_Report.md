# P1-T2 Validation Report

**Verdict:** PASS
**Date:** 2026-07-11
**Task:** Research Agent and Integration Storage Locations

## Evidence collected

- Official OpenAI Codex auth/config/MCP documentation confirms `~/.codex/auth.json`, `~/.codex/config.toml`, `CODEX_HOME`, project `.codex/config.toml`, and keyring alternatives.
- Official Claude Code documentation confirms user/local MCP in `~/.claude.json`, project `.mcp.json`, user settings, and Keychain-backed credentials.
- Official Model Context Protocol documentation confirms the macOS Claude Desktop path under `~/Library/Application Support/Claude/claude_desktop_config.json`.
- Official OpenCode documentation confirms `~/.config/opencode/opencode.json`, project `opencode.json`/`.opencode`, and `~/.local/share/opencode/auth.json` for provider credentials.
- Official Cursor documentation confirms global `~/.cursor/mcp.json` and project `.cursor/mcp.json`.
- Official AWS documentation confirms `~/.aws/credentials` and `~/.aws/config` as shared credential/config locations.

## Local metadata observations

Path-only `stat` inspection on macOS confirmed:

| Path | Owner | Mode | Status |
| --- | --- | --- | --- |
| `~/.codex/auth.json` | current user | `0600` | confirmed-local |
| `~/.codex/config.toml` | current user | `0600` | confirmed-local |
| `~/.claude.json` | current user | `0600` | confirmed-local |
| `~/.claude/settings.json` | current user | `0600` | confirmed-local |
| `~/.claude/settings.local.json` | current user | `0644` | confirmed-local, finding candidate |
| `~/.config/opencode/opencode.json` | current user | `0644` | confirmed-local, finding candidate |
| `~/.cursor/mcp.json` | current user | `0600` | confirmed-local |
| `~/Library/Application Support/Claude/claude_desktop_config.json` | current user | `0600` | confirmed-local |
| `~/.local/share/opencode/auth.json` | — | — | documented, missing locally |
| `~/.aws/credentials` | — | — | documented, missing locally |

Parent directories `.codex`, `.claude`, `.config/opencode`, and `.cursor` were `0755`; Claude and Cursor Application Support directories were `0700`. Parent exposure is recorded separately from child confidentiality.

## Safety checks

- No config, database, Keychain record, environment dump, token value, or credential content was read.
- No user-home file or permission was changed.
- Evidence text contains normalized `$HOME` paths and no real username or secret material.
- GitHub Copilot remains optional and does not affect the PASS verdict.

## Result

`docs/agent-storage-locations.md` is ready as the input for P1-T3 profile schema and resolver work. `confirmed-local`, `documented`, `missing`, and Keychain-only states remain distinct; unknown storage is not claimed as covered.
