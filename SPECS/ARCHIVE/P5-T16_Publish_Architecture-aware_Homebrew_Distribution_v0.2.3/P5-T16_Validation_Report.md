# P5-T16 Validation Report

**Date:** 2026-07-12  
**Verdict:** PASS with host-toolchain caveat

## Release identity

| Item | Evidence |
|---|---|
| Cargo/package version | `0.2.3` in `Cargo.toml` and `Cargo.lock` |
| Git tag | immutable annotated `v0.2.3`, pushed to `origin` |
| arm64 asset | `tokenkeeper-v0.2.3-aarch64-apple-darwin.tar.gz` |
| arm64 SHA-256 | `c8741f35d6f7bdfe9a07b95693ab86c13bfac809ce2c8e532ba4f9c7f961b9d9` |
| source asset | `tokenkeeper-v0.2.3-source.tar.gz` |
| source SHA-256 | `e523c29918c2a334ba6fef2c91505922bfc067e8413ecf47ae46ef7991c2c151` |
| Homebrew tap Formula | commit `e0566a8a420c8217ad135984eba1b1a61c7459a3` |
| installed binary | `tokenkeeper 0.2.3` |
| preserved release | `v0.2.2` asset digest remains `f1e5fd6d73b4d895d5ccc21ce0574568c55c7702d972accf89362e7a2c1b7b9b` |

## Quality gates

- `cargo fmt --all --check` — PASS (rustup-managed toolchain)
- `cargo test --all-targets --all-features` — PASS, 60 tests
- `cargo clippy --all-targets --all-features -- -D warnings` — PASS
- `brew style Formula/tokenkeeper.rb` — PASS
- `brew reinstall soundblaster/tap/tokenkeeper` — PASS; installed from arm64 release asset
- `brew test soundblaster/tap/tokenkeeper` — PASS
- `/opt/homebrew/bin/tokenkeeper --version` — `tokenkeeper 0.2.3`

## Caveat

The unqualified Homebrew `cargo` command on this macOS host still reaches `/opt/homebrew/opt/rust/bin/rustc`, which exits with SIGKILL/code-signature-invalid. The release build and all Rust gates were therefore run with the rustup-managed pinned compiler. This is the reason Apple Silicon Formula installation now avoids a source build.
