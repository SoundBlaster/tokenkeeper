# Tokenkeeper

Tokenkeeper is a minimal Rust CLI for a read-only, metadata-only audit of AI-agent, MCP and utility configuration files under the current user’s Home. It checks ownership, Unix modes, macOS ACLs, node types, symlinks and writable ancestors; it never reads token contents or executes remediation.

## Install from source

Rust stable and Cargo are required until the Homebrew release work is complete:

```bash
git clone https://github.com/SoundBlaster/tokenkeeper.git
cd tokenkeeper
cargo install --path .
```

The maintainer-owned tap contract is documented in [`docs/homebrew.md`](docs/homebrew.md). After the formula is published to the tap:

```bash
brew tap SoundBlaster/homebrew-tap
brew install SoundBlaster/homebrew-tap/tokenkeeper
brew update && brew upgrade tokenkeeper
brew uninstall tokenkeeper
```

## Usage

List the embedded profiles and their research evidence:

```bash
tokenkeeper profiles
```

Audit all built-in profiles, or select one repeatedly:

```bash
tokenkeeper check
tokenkeeper check --profile codex --profile cursor
```

Check an explicit absolute path with a declared policy:

```bash
tokenkeeper check --path "$HOME/.codex/auth.json" --policy credential-config
```

The command prints `PASS`, `FINDING`, `UNKNOWN` or `SKIP`. Exit code `0` means a complete clean audit, `1` means complete findings, and `2` means incomplete/unknown state or invalid usage. A suggested `chmod` line is only printed when the target and parent chain are unambiguous; Tokenkeeper never runs it.

## Supported scope

The required built-in profiles cover Codex, Claude Code, OpenCode, Cursor, and MCP/utility file-backed configurations. macOS is the supported platform with ACL evaluation; Linux reuses the Unix mode/resolver core as best-effort and reports ACL coverage as incomplete. Source locations, evidence dates and Keychain/project-scope limitations are recorded in [`docs/agent-storage-locations.md`](docs/agent-storage-locations.md). Security assumptions and limitations are in [`docs/security.md`](docs/security.md).

## Development

```bash
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

See [`SPECS/Workplan.md`](SPECS/Workplan.md) for the delivery plan and platform support status.
