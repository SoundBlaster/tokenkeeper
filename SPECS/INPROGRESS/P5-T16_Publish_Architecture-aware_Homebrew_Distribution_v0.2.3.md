# P5-T16 — Publish Architecture-aware Homebrew Distribution v0.2.3

## Objective

Give the post-`v0.2.2` Homebrew distribution change an immutable release identity. The release must preserve `v0.2.2` while making the Apple Silicon prebuilt path and Intel source fallback reproducible.

## Deliverables

- Bump the Rust package and lockfile to `0.2.3`.
- Build and publish `tokenkeeper-v0.2.3-aarch64-apple-darwin.tar.gz`.
- Update the repository Formula and the `SoundBlaster/homebrew-tap` Formula.
- Update release/checklist documentation and record validation evidence.

## Acceptance criteria

1. `Cargo.toml`, `Cargo.lock`, `tokenkeeper --version`, release tag, asset name, Formula URLs/checksums, and documentation identify `0.2.3`.
2. `v0.2.2` is not moved or overwritten; its asset digest remains `f1e5fd6d73b4d895d5ccc21ce0574568c55c7702d972accf89362e7a2c1b7b9b`.
3. Apple Silicon Homebrew installation consumes the release asset and does not invoke Homebrew's broken `rustc`.
4. Intel Formula logic retains a locked source build with `rust` as a build dependency.
5. `cargo test --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all --check` pass.
6. `brew style`, `brew reinstall`, and `brew test` pass on the current Apple Silicon host.

## Dependencies and risks

- Depends on the published `v0.2.2` release and existing architecture-aware Formula.
- The arm64 binary is built with the rustup-managed toolchain because the Homebrew `rustc` is code-signature-invalid on this host.
- Release tags and assets are immutable; failed publication must use a new version rather than rewriting `v0.2.3`.
