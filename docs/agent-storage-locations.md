# Agent and Integration Storage Locations

**Research date:** 2026-07-11
**Platform:** macOS
**Method:** upstream documentation plus path-only local metadata inspection
**Version context:** official documentation pages were fetched on the research date; agent binaries were not launched, so local path confirmation does not claim a runtime version.

This inventory is input for embedded Tokenkeeper profiles. `CredentialConfig` means that a file is known or declared to potentially contain credentials; Tokenkeeper does not read the file to prove that a token is present. `confirmed-local` means the path exists locally and its metadata was inspected. No file contents, environment dumps, Keychain records, or credentials were read.

## Confirmed and documented locations

| Category | Location | Use / backend | Policy | Local evidence |
| --- | --- | --- | --- | --- |
| Codex | `~/.codex/auth.json` | File-backed Codex login cache | `SecretFile` / `CredentialConfig` | `confirmed-local`, owner-only `0600` |
| Codex + MCP | `~/.codex/config.toml` | User config and MCP definitions | `CredentialConfig` / `ExecutableConfig` | `confirmed-local`, `0600` |
| Codex project | `**/.codex/config.toml` under trusted projects | Project-scoped config and possible integrations | `CredentialConfig` / `ExecutableConfig` | `documented`, bounded selector required |
| Claude Code MCP | `~/.claude.json` | User and local-scope MCP definitions | `CredentialConfig` | `confirmed-local`, `0600` |
| Claude Code MCP | `.mcp.json` in a project root | Project-scope MCP configuration | `CredentialConfig` / `TrustedConfig` | `documented`, not present in this repository |
| Claude Code | `~/.claude/settings.json` | User settings, permissions and hooks | `TrustedConfig` / `ExecutableConfig` | `confirmed-local`, `0600` |
| Claude Code | `~/.claude/settings.local.json` | Local settings and possible hooks | `ExecutableConfig` | `confirmed-local`, `0644` — exposure finding candidate |
| Claude Desktop MCP | `~/Library/Application Support/Claude/claude_desktop_config.json` | Desktop MCP configuration | `CredentialConfig` | `confirmed-local`, `0600` |
| Cursor MCP | `~/.cursor/mcp.json` | Global MCP servers and auth environment values | `CredentialConfig` | `confirmed-local`, `0600` |
| Cursor MCP | `.cursor/mcp.json` in a project root | Project MCP servers | `CredentialConfig` / `TrustedConfig` | `documented`, not present in this repository |
| OpenCode | `~/.config/opencode/opencode.json` | Global JSON/JSONC config and MCP | `CredentialConfig` | `confirmed-local`, `0644` — exposure finding candidate |
| OpenCode credentials | `~/.local/share/opencode/auth.json` | Provider credentials | `SecretFile` / `CredentialConfig` | `documented`, absent locally |
| OpenCode project | `opencode.json` and bounded `.opencode/` | Project config, plugins and tools | `CredentialConfig` / `ExecutableConfig` | `documented`, project-scoped |
| AWS utility | `~/.aws/credentials` | Shared AWS access credentials used by SDKs and tools | `CredentialConfig` | `documented`, absent locally |
| AWS utility | `~/.aws/config` | AWS profiles, credential sources and regions | `CredentialConfig` / `ExecutableConfig` | `documented`, absent locally |

### Local directory metadata

The following parent directories were observed with owner `egor`, group `staff`, and mode `0755`: `~/.codex`, `~/.claude`, `~/.config`, `~/.config/opencode`, and `~/.cursor`. `~/Library/Application Support/Claude` and `~/Library/Application Support/Cursor` were observed as owner-only `0700` directories.

Directory mode is recorded because a `0600` child file can still have filename and traversal exposure through a world-readable parent. The profile evaluator must report file confidentiality and parent-directory integrity separately; this document does not apply remediation.

## Storage limitations

- Codex can store credentials in an OS keyring instead of `auth.json`; keyring-only state is outside this metadata-only file audit.
- Claude Code documents macOS Keychain storage for API/OAuth credentials. MCP headers, environment values, and project/user configuration remain file-based candidates.
- Cursor supports environment-variable authentication and OAuth; the profile covers `mcp.json` exposure, not the provider’s external credential store.
- OpenCode supports environment/file substitutions and stores provider credentials in `auth.json`; both the referenced file and the config declaring it are relevant.
- Environment-only credentials, process arguments, remote MCP servers, and secrets embedded in unknown files are out of scope until a user supplies an explicit path/policy or a future profile.
- GitHub Copilot is intentionally deferred as an optional profile and does not block the required inventory.

## Evidence sources

- [Codex authentication](https://developers.openai.com/codex/auth), [Codex configuration basics](https://developers.openai.com/codex/config-basic), [Codex advanced configuration](https://developers.openai.com/codex/config-advanced), and [Codex MCP](https://developers.openai.com/codex/mcp)
- [Claude Code MCP](https://code.claude.com/docs/en/mcp) and [Claude Code IAM/credential management](https://code.claude.com/docs/en/iam)
- [OpenCode configuration](https://opencode.ai/docs/config/) and [OpenCode CLI/auth](https://dev.opencode.ai/docs/cli/)
- [Cursor MCP configuration](https://docs.cursor.com/context/model-context-protocol)
- [MCP local server configuration](https://modelcontextprotocol.io/docs/develop/connect-local-servers)
- [AWS CLI configuration and credential files](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)

## Profile implementation notes

1. Treat the confirmed credential-bearing files as `CredentialConfig` even when their content is not inspected.
2. Keep project selectors bounded and no-follow for symlinks.
3. Preserve `documented`, `confirmed-local`, `missing`, and `Keychain-only` as separate evidence states.
4. Never copy a real file into fixtures; use metadata-only temporary trees for policy tests.
