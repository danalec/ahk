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
    sha256 "d81bcbdbb279745781d8a458d1efabbf953ae0fe3112d4bdacef615cdb0c1f36"
  end

  on_intel do
    url "https://github.com/danalec/ahk/releases/download/v0.6.7/ahk-v0.6.7-x86_64-macos.tar.gz"
    sha256 "d3ae8cafe8bd1bee07456511269abc7e0d3d7396f31eead651086f055860e346"
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
