# Homebrew distribution

The maintainer-owned tap is `SoundBlaster/homebrew-tap`. Install the published formula from the tap for validation with:

```bash
brew tap SoundBlaster/homebrew-tap
brew install SoundBlaster/homebrew-tap/tokenkeeper
```

The published tap workflow is:

```bash
brew tap SoundBlaster/homebrew-tap
brew install SoundBlaster/homebrew-tap/tokenkeeper
brew update
brew upgrade tokenkeeper
brew uninstall tokenkeeper
brew untap SoundBlaster/homebrew-tap
```

The current formula pins the immutable `v0.2.3` release assets and SHA-256 checksums. On Apple Silicon it installs the prebuilt `tokenkeeper-v0.2.3-aarch64-apple-darwin.tar.gz` asset; on Intel it uses the locked `tokenkeeper-v0.2.3-source.tar.gz` fallback and builds with Cargo. It installs only the `tokenkeeper` binary, and its `test do` invokes only `--version` and `profiles`. It has no `post_install`, service setup, implicit Home scan, credential access or remediation mutation.

For a new release, create and push a signed `vX.Y.Z` tag, publish architecture-specific release assets, calculate `shasum -a 256` for each asset, update the Formula's conditional `url` and `sha256` values, then run `brew audit --new`, `brew style`, install and `brew test` on supported architectures. The Formula derives its version from the asset URL; there is no separate `version` field to update. Homebrew bottles and homebrew-core submission are separate future work.
