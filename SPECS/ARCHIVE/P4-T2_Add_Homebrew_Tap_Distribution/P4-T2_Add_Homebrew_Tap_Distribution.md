# P4-T2 — Add Homebrew Tap Distribution

## Objective

Publish the release contract and a maintainer-owned Homebrew formula that builds the versioned Rust source without scanning or mutating a user Home.

## Deliverables

- `Formula/tokenkeeper.rb` with immutable source URL/checksum, SemVer, MIT license and Cargo build/test steps.
- Release/tap instructions covering tag, checksum, audit, install, upgrade and uninstall.
- Formula tests that run only safe commands (`--version`, `profiles`) in Homebrew's test environment.

## Constraints

- No `post_install`, service/daemon setup, implicit Home scan, credential access or remediation mutation.
- Formula must not use an unversioned branch archive or a placeholder checksum.
- Source archive and checksum must correspond to the exact published commit/tag.
- Tap ownership is explicit (`SoundBlaster/homebrew-tap`); publishing a formula here does not claim homebrew-core inclusion.

## Test-first plan

1. Add formula and release documentation using the current immutable GitHub commit archive.
2. Run `brew audit --new-formula`/style and formula test where local Homebrew permits.
3. Verify source checksum, Cargo build contract and README install/upgrade/uninstall commands.
4. Run Rust quality gates and record platform/tooling limitations.

## Acceptance criteria

- Formula has version, source URL, SHA-256, license, source build and safe `test do`.
- `brew audit` and `brew test` pass or any environment blocker is documented with reproducible evidence.
- README documents `brew tap`, install, upgrade, update and uninstall.

## Out of scope

Bottle publishing, homebrew-core submission, automatic release CI and Linux packaging.

---
**Archived:** 2026-07-11
**Verdict:** PASS
