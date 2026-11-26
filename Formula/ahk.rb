# SPDX-License-Identifier: Apache-2.0
#
# The Homebrew formula for ahk.
#
# The package installs the prebuilt macOS archives of the release, pinned to
# the SHA256 of the file each one names. The URLs and the digests below are
# the ones of the release, and the tap is a distribution point, not a
# second source of truth: the canonical file is this one, in the repository of
# the project.
class Ahk < Formula
  desc "An AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine"
  homepage "https://github.com/danalec/ahk"
  license "Apache-2.0"
  version "0.6.7"

  on_arm do
    url "https://github.com/danalec/ahk/releases/download/v0.6.7/ahk-v0.6.7-aarch64-macos.tar.gz"
    sha256 "c8dbcc8acadc738ba3c4f2066d23a65b409b243a992008403a50bf22893d22f4"
  end

  on_intel do
    url "https://github.com/danalec/ahk/releases/download/v0.6.7/ahk-v0.6.7-x86_64-macos.tar.gz"
    sha256 "5890380c4ed59c08fcf229475f75ee860262a73b1f6bfe6a8437c797ec3e955b"
  end

  def install
    bin.install "ahk"
    doc.install "README.md"
    license.install "LICENSE"
  end

  test do
    assert_match "hello from homebrew", shell_output("#{bin}/ahk <<< 'Msg \"hello from homebrew\"'")
  end
end
