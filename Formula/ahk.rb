# SPDX-License-Identifier: Apache-2.0
#
# The Homebrew formula for ahk.
#
# The package installs the prebuilt macOS archives of the release, pinned to
# the SHA256 of the file each one names. The URLs and the digests below are
# the ones of the 0.3.2 release, and the tap is a distribution point, not a
# second source of truth: the canonical file is this one, in the repository of
# the project.
class Ahk < Formula
  desc "An AutoHotkey interpreter written in Rust, with a desktop backend for driving a Linux machine"
  homepage "https://github.com/danalec/ahk"
  license "Apache-2.0"
  version "0.3.2"

  on_arm do
    url "https://github.com/danalec/ahk/releases/download/v0.3.2/ahk-v0.3.2-aarch64-macos.tar.gz"
    sha256 "bbcc93c98d386bb0519c0293f9b4c8dc739d9be716c3c4bdc74db35326133a7f"
  end

  on_intel do
    url "https://github.com/danalec/ahk/releases/download/v0.3.2/ahk-v0.3.2-x86_64-macos.tar.gz"
    sha256 "d5272fd0c47692d6dd6804bda021adf1e1750854a760eba75526216f036ab013"
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
