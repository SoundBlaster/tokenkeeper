# P5-T13 Validation Report

**Date:** 2026-07-12
**Task:** Automate the Homebrew Release Lifecycle
**Verdict:** PASS (workflow added; hosted lifecycle pending next CI run)

## Local gates

- `brew style Formula/tokenkeeper.rb` — PASS.
- `brew audit --new Formula/tokenkeeper.rb` — PASS.
- `brew test tokenkeeper` — PASS.

## Acceptance evidence

- `.github/workflows/homebrew.yml` covers `macos-13` (Intel) and `macos-14` (Apple Silicon).
- Workflow runs formula style/audit, clean source install, formula test, upgrade, uninstall, and post-uninstall configuration-preservation check.
- The fixture is outside Homebrew's prefix and is asserted to survive uninstall.

The hosted matrix is intentionally not duplicated locally; its run is the authoritative clean-environment evidence.
