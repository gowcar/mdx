# This file is a template for the Homebrew formula.
# To use: create a repo at github.com/gowcar/homebrew-tap
# and place this file as Formula/mdx.rb
#
# Update the url, sha256, and version for each release.

class Mdx < Formula
  desc "A fast, beautiful terminal markdown viewer"
  homepage "https://github.com/gowcar/mdx"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/gowcar/mdx/releases/download/v#{version}/mdx-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/gowcar/mdx/releases/download/v#{version}/mdx-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/gowcar/mdx/releases/download/v#{version}/mdx-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/gowcar/mdx/releases/download/v#{version}/mdx-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "mdx"
  end

  test do
    assert_match "mdx", shell_output("#{bin}/mdx --version")
  end
end
