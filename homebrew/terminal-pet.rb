class TerminalPet < Formula
  desc "A tiny terminal pet"
  homepage "https://github.com/omapem/terminal-pet"
  url "https://github.com/omapem/terminal-pet/releases/download/v0.1.0/terminal-pet-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
  sha256 "REPLACE_WITH_REAL_SHA256"
  version "0.1.0"

  def install
    bin.install "terminal-pet"
  end
end
