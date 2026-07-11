# Tokenkeeper

Tokenkeeper is a Rust CLI for auditing permissions on AI-agent, MCP, and utility configuration files. The planned product is read-only and metadata-only: it will never read token contents or apply remediation commands automatically.

The repository is currently bootstrapping the CLI delivery skeleton. Filesystem auditing, profiles, ACL support, and Homebrew packaging are tracked in [`SPECS/Workplan.md`](SPECS/Workplan.md).

## Development

```bash
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

Run the current CLI baseline with:

```bash
cargo run -- --help
cargo run -- --version
```
