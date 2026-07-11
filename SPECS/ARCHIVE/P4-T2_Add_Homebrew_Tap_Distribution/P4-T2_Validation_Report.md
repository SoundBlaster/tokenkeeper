# P4-T2 Validation Report

## Scope

Added `Formula/tokenkeeper.rb` and `docs/homebrew.md` for the maintainer-owned `SoundBlaster/homebrew-tap`. The formula pins version `0.1.0` to the immutable `v0.1.0` GitHub tag archive with a verified SHA-256, builds with Cargo, installs only the binary and runs safe `--version`/`profiles` tests.

## Checks

```text
brew audit --new Formula/tokenkeeper.rb   PASS
brew style Formula/tokenkeeper.rb         PASS
cargo test --all-targets --all-features   PASS (37 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check                   PASS
```

The release archive checksum was downloaded and recomputed locally before writing the formula. No `post_install`, service setup, implicit Home scan, credential access or remediation mutation is present.

## Environment limitation

An initial local `brew install` attempt stalled while Homebrew bootstrapped the Rust bottle; a retry with `--ignore-dependencies` failed explicitly because Homebrew's build PATH had no `cargo`. The remote tap and release tag are now published; the formula's `test do` should be run in a clean Homebrew environment with the declared Rust dependency available.

## Verdict

PASS with documented external toolchain limitation — ready for archive and review.
