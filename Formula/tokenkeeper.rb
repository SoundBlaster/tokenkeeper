class Tokenkeeper < Formula
  desc "Read-only metadata auditor for AI-agent credentials and configuration"
  homepage "https://github.com/SoundBlaster/tokenkeeper"
  url "https://github.com/SoundBlaster/tokenkeeper/releases/download/v0.2.2/tokenkeeper-v0.2.2-aarch64-apple-darwin.tar.gz"
  sha256 "f1e5fd6d73b4d895d5ccc21ce0574568c55c7702d972accf89362e7a2c1b7b9b"
  license "MIT"

  def install
    bin.install "tokenkeeper"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/tokenkeeper --version")
    assert_match "codex", shell_output("#{bin}/tokenkeeper profiles")
  end
end
