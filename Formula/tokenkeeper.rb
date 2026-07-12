class Tokenkeeper < Formula
  desc "Read-only metadata auditor for AI-agent credentials and configuration"
  homepage "https://github.com/SoundBlaster/tokenkeeper"
  license "MIT"

  on_arm do
    url "https://github.com/SoundBlaster/tokenkeeper/releases/download/v0.2.2/tokenkeeper-v0.2.2-aarch64-apple-darwin.tar.gz"
    sha256 "f1e5fd6d73b4d895d5ccc21ce0574568c55c7702d972accf89362e7a2c1b7b9b"
  end

  on_intel do
    url "https://github.com/SoundBlaster/tokenkeeper/archive/refs/tags/v0.2.2.tar.gz"
    sha256 "2f76f4b19cb57bb1088461e12a2cc66f6a3300fc36875f706d23bb53c5264b96"
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
