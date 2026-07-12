class Tokenkeeper < Formula
  desc "Read-only metadata auditor for AI-agent credentials and configuration"
  homepage "https://github.com/SoundBlaster/tokenkeeper"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/SoundBlaster/tokenkeeper/releases/download/v0.2.3/tokenkeeper-v0.2.3-aarch64-apple-darwin.tar.gz"
    sha256 "c8741f35d6f7bdfe9a07b95693ab86c13bfac809ce2c8e532ba4f9c7f961b9d9"
  else
    url "https://github.com/SoundBlaster/tokenkeeper/releases/download/v0.2.3/tokenkeeper-v0.2.3-source.tar.gz"
    sha256 "e523c29918c2a334ba6fef2c91505922bfc067e8413ecf47ae46ef7991c2c151"
    depends_on "rust" => :build
  end

  def install
    if Hardware::CPU.arm?
      bin.install "tokenkeeper"
    else
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    end
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/tokenkeeper --version")
    assert_match "codex", shell_output("#{bin}/tokenkeeper profiles")
  end
end
