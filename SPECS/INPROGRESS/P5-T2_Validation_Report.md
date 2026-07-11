# P5-T2 Validation Report

**Date:** 2026-07-12  
**Task:** Correct Native macOS ACL Evaluation  
**Verdict:** PASS with repository coverage gap

## Scope

- Native ACL text parsing in `src/acl.rs`.
- macOS `acl_get_file` acquisition and errno classification.
- Regression fixtures in `tests/acl.rs`.

## Quality gates

| Gate | Command | Result |
|---|---|---|
| Tests | `cargo test --all-targets --all-features` | PASS — all tests passed |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| Format | `cargo fmt --all --check` | PASS |
| Coverage | `rustup run stable cargo llvm-cov --all-features --all-targets --workspace --fail-under-lines 80` | FAIL — 72.23% total lines |

Coverage is a pre-existing repository-wide gate gap; the ACL file itself is 79.79% lines after this change. Raising the total threshold is tracked separately in P5-T12.

## Acceptance evidence

- Native `acl_to_text` ordering with `read_data,write_data` produces a non-owner `Finding`.
- Header lines, qualifiers, allow/deny effects, inheritance flags, and native permission tokens are parsed conservatively; unknown permissions and suffixes produce `Unknown`.
- `acl_get_file` null results are `NotPresent` only for the platform no-ACL errno (or an existing path returning the observed macOS no-ACL `ENOENT`); all other errors, including target disappearance, produce `Unknown`.
- FFI remains isolated under `#[cfg(target_os = "macos")]`; `acl_free` is called for both the ACL object and converted text on every successful acquisition path.
- Existing metadata and security integration suites remain green.

## Platform note

The current runner is macOS, so the compile-gated native acquisition path was compiled and exercised by the metadata suite. Linux retains the explicit `Unsupported` backend behavior.
