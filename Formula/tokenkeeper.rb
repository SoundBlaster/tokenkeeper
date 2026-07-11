class Tokenkeeper < Formula
  desc "Read-only metadata auditor for AI-agent credentials and configuration"
  homepage "https://github.com/SoundBlaster/tokenkeeper"
  url "https://github.com/SoundBlaster/tokenkeeper/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "2556a3f259415f0cac2cbc153fb39ff93a195295afac922531b96acc7670bd6c"
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
