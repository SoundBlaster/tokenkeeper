# Homebrew distribution

The maintainer-owned tap is `SoundBlaster/homebrew-tap`. Until the formula is copied to that tap, install it locally for validation with:

```bash
brew install --build-from-source ./Formula/tokenkeeper.rb
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

The formula pins the SemVer `v0.1.0` release to an immutable GitHub source archive and SHA-256 checksum. It builds with Cargo, installs only the `tokenkeeper` binary, and its `test do` invokes only `--version` and `profiles`. It has no `post_install`, service setup, implicit Home scan, credential access or remediation mutation.

For a new release, create and push a signed `vX.Y.Z` tag, download that exact archive, calculate `shasum -a 256`, update `url`, `sha256` and `version`, then run `brew audit --new`, `brew style`, source install and `brew test`. Homebrew bottles and homebrew-core submission are separate future work.
