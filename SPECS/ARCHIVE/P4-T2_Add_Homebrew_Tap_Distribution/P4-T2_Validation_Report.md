# P4-T2 Validation Report

## Scope

Added `Formula/tokenkeeper.rb` and `docs/homebrew.md` for the maintainer-owned `SoundBlaster/homebrew-tap`. The formula pins version `0.1.0` to an immutable GitHub commit archive with a verified SHA-256, builds with Cargo, installs only the binary and runs safe `--version`/`profiles` tests.

## Checks

```text
brew audit --new Formula/tokenkeeper.rb   PASS
brew style Formula/tokenkeeper.rb         PASS
cargo test --all-targets --all-features   PASS (37 tests)
cargo clippy --all-targets --all-features -- -D warnings   PASS
cargo fmt --all --check                   PASS
```

The archive checksum was downloaded and recomputed locally before writing the formula. No `post_install`, service setup, implicit Home scan, credential access or remediation mutation is present.

## Environment limitation

`brew install --build-from-source SoundBlaster/homebrew-tap/tokenkeeper` could not complete in this desktop environment: Homebrew downloaded the Rust bottle but its toolchain bootstrap stalled. A retry with `--ignore-dependencies` failed explicitly because Homebrew's build PATH had no `cargo`. This is an environment/toolchain blocker, not reported as a formula PASS; `brew audit` and `brew style` remain independently green. The tap formula's `test do` must be run in a clean Homebrew environment with the declared Rust dependency available.

## Verdict

PASS with documented external toolchain limitation — ready for archive and review.
