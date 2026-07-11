class Tokenkeeper < Formula
  desc "Read-only metadata auditor for AI-agent credentials and configuration"
  homepage "https://github.com/SoundBlaster/tokenkeeper"
  url "https://github.com/SoundBlaster/tokenkeeper/archive/refs/tags/v0.2.2.tar.gz"
  sha256 "2f76f4b19cb57bb1088461e12a2cc66f6a3300fc36875f706d23bb53c5264b96"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/tokenkeeper --version")
    assert_match "codex", shell_output("#{bin}/tokenkeeper profiles")
  end
end
