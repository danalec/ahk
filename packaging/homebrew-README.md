# homebrew-ahk

A personal Homebrew **tap** for [ahk](https://github.com/danalec/ahk): an
AutoHotkey interpreter written in Rust, with a desktop backend for driving a
Linux machine.

This is NOT Homebrew/homebrew-core. The formula here is maintained by the
ahk project itself and installs the prebuilt macOS archives from the ahk
GitHub release; nothing is submitted to Homebrew's own repository.

## Install

```sh
brew tap danalec/ahk
brew install ahk
```

The tap name is `danalec/ahk` because Homebrew strips the `homebrew-`
prefix from `danalec/homebrew-ahk`.

## What it installs

`ahk`, the runner: it reads the script on the standard input and writes the
report on the standard output.

```sh
echo 'Msg "hello"' | ahk
```

Both Apple silicon and Intel Macs are supported, pinned to the SHA256 of
their release archives.

## Updating

The canonical formula lives in the ahk repository at `Formula/ahk.rb`. On a
new release its version and digests are rewritten from the combined
`SHA256SUMS` of the release, and the updated file is copied here. This tap is
a distribution point, not a second source of truth.
