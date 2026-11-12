# The packaging of ahk

The tree is published in four places, and each one installs the **prebuilt
archive of the release**, pinned to the SHA256 of the file it names. Nothing
here builds from the sources: the CI of the GitHub builds the five archives
(x86_64 and aarch64, Linux and macOS, and Windows) and smokes them, and the
packaging names those files.

| Where | The file | What it is |
|---|---|---|
| Nix | [`../flake.nix`](../flake.nix) | a flake with one package per platform, and a `checks.smoke` that runs the runner |
| Homebrew | [`../Formula/ahk.rb`](../Formula/ahk.rb) | the formula of the tap [`danalec/homebrew-ahk`](https://github.com/danalec/homebrew-ahk) |
| WinGet | [`../.winget/`](../.winget) | the three manifests of the package `danalec.ahk`, submitted to the winget-pkgs repository |
| Arch | [`aur/PKGBUILD`](aur/PKGBUILD) | the package of the AUR, `ahk` |

## The archives

The release of the version `0.3.2` carries one archive per platform, and the
SHA256SUMS beside them is the truth:

```
ahk-v0.3.2-x86_64-linux-gnu.tar.gz    ed40f96be2ef103697fec3fc3a92235b11ab8698297d03fef9fa734a4cd65efa
ahk-v0.3.2-aarch64-linux-gnu.tar.gz   8ed785a950a400b83b2d4bd29f0ce979fbf9a947d77227b891b650c15c66482c
ahk-v0.3.2-x86_64-macos.tar.gz        d5272fd0c47692d6dd6804bda021adf1e1750854a760eba75526216f036ab013
ahk-v0.3.2-aarch64-macos.tar.gz       bbcc93c98d386bb0519c0293f9b4c8dc739d9be716c3c4bdc74db35326133a7f
ahk-v0.3.2-x86_64-windows.zip         d28e7c7ceb57069afb7c7f5d314fec8478eb638bcd1218962bbc6df66deccfbc
```

The archive holds the runner (`ahk`, or `ahk.exe`), the `README.md` and the
`LICENSE`, under a directory named after the release.

## Installing

```sh
# Nix
nix run github:danalec/ahk

# Homebrew
brew tap danalec/ahk
brew install ahk

# WinGet
winget install danalec.ahk

# Arch
yay -S ahk
```

The runner reads the script on the standard input and writes the report on the
standard output:

```sh
echo 'Msg "hello"' | ahk
```
