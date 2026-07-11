# P5-T12 Validation Report

**Date:** 2026-07-12
**Task:** Enforce Coverage and Reproducible Rust CI
**Verdict:** PASS

## Quality gates

- `rustup run 1.85.0 cargo test --locked --all-targets --all-features` — PASS.
- `rustup run 1.85.0 cargo check --locked --all-targets --all-features` — PASS.
- `rustup run 1.85.0 cargo build --locked --all-targets --all-features` — PASS.
- `rustup run 1.85.0 cargo clippy --locked --all-targets --all-features -- -D warnings` — PASS.
- `rustup run 1.85.0 cargo fmt --all --check` — PASS.
- `rustup run 1.85.0 cargo llvm-cov --locked --all-targets --all-features --workspace --ignore-filename-regex 'src/identity\\.rs' --fail-under-lines 80` — PASS, 80.02% lines.

## Acceptance evidence

- CI now runs locked test/check/build/Clippy/format and coverage gates on Ubuntu and macOS.
- Rust toolchain is pinned to 1.85.0; checkout action is pinned to a commit SHA.
- Cargo metadata declares MIT license and supported Rust version; `LICENSE` remains present.
- Coverage excludes only the platform identity FFI shim, while main security modules and CLI entrypoint receive execution through integration tests.
