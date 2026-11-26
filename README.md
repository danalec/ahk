<p align="center">
  <img src="resources/logo.png" width="128" height="128" alt="ahk" />
</p>

# ahk

An AutoHotkey interpreter written in Rust that **runs AHK scripts in every
dialect** — v1, v2, v2.1 and the KeySharp extensions — on Windows and on
Linux. A lexer, an AST, a parser, a runtime, the whole built-in surface and
class library of each dialect, plus a desktop backend: a keyboard, a
pointer, screen captures, files, windows, menus, hotkeys, sound, the
clipboard, COM on Windows and D-Bus on Linux.

The v2 is the default; the v1, the v2.1 mechanisms and the KeySharp
extensions live behind flags.

The documents of the tree are in [`docs/`](docs/README.md); the surface --
every name of the three dialects, with what this tree answers for it -- is
in [`docs/surface/`](docs/surface/README.md), and the full per-name
reference (description, syntax, parameters, return, errors, remarks,
example, related) in thirteen languages is in
[`docs/reference/`](docs/reference/index.md).

## How to build

The tree is a Rust workspace, and it builds with a stable toolchain.

  - Get the source code.
  - Build the workspace: `cargo build --workspace`.
  - Build the runner: `cargo build --release -p ahk-cli`.
  - The runner is at `target/release/ahk`, and it is on crates.io as
    `ahk-cli`: `cargo install ahk-cli`.

The language crate builds without the standard library, for a target
that has none:

```sh
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
```

## Install

The runner is packaged for the usual managers; each installs the portable
binary of the release (the AUR and Nix also offer a from-source build).

### Windows

winget:

```sh
winget install danalec.ahk
```

scoop:

```sh
scoop bucket add alts https://github.com/danalec/scoop-alts
scoop install alts/ahk
```

### macOS

```sh
brew install danalec/ahk/ahk
```

### Linux

Homebrew:

```sh
brew install danalec/ahk/ahk
```

Nix:

```sh
nix run github:danalec/ahk
```

### Arch Linux (AUR)

```sh
yay -S ahk                        # prebuilt binary
yay -S ahk-git                    # from source, native SIMD
```

### Any platform (from source)

```sh
cargo install ahk-cli                       # crates.io
nix build github:danalec/ahk#ahk-simd       # Nix, SIMD
```

## The platforms

The language crate does no I/O of its own, and it needs no operating
system at all: it builds without the standard library, and it builds for
a microcontroller (`thumbv7em-none-eabihf`, `x86_64-unknown-none`) and
for a 32-bit Windows (`i686-pc-windows-msvc`, `i686-pc-windows-gnu`).
The clock -- the time, and the pause -- is the one thing the host has to
supply.

The runner and the desktop backend use the standard library. On Windows
the backend is **complete**: the machine, the windows and their controls,
the menus, the keyboard, the pointer, the screen, the clipboard, the
sound, the dialogs, the processes, the registry, the drives and the COM
of `user32`/`gdi32`/`advapi32`/`ole32`. The modern build's floor is
Windows 10 1703; the **Windows 7** and **Windows XP** floors are carried by
the tier-3 `win7` targets with the standard library emended for the era
(the XP binary runs both dialects on the XP guest of the lab, measured).

On Linux there are two desktops: the **lab box** (uinput keyboard and
pointer, `grim` captures, Wayland toplevels, the GTK helper for the GUI
and the menus, D-Bus as the object model of the session) and an **X11
path** (XTEST input, `XGetImage`, the Xlib window list, X11 client
messages, XFixes cursor). What runs where, and the floor of each build,
are in [docs/platforms.md](docs/platforms.md); the older floors are in
[docs/windows-legacy.md](docs/windows-legacy.md).

SIMD builds (x86-64-v2/v3/v4, ARM NEON) are gated in CI and packaged for
the AUR (`ahk-git`, `target-cpu=native`) and Nix (`nix build .#ahk-simd`);
what they speed up is in [docs/simd.md](docs/simd.md).

## The runner

The runner reads a script on **stdin** and writes a JSON report on
**stdout**:

```sh
echo 'MsgBox "hello"' | ahk
```

```json
{"ok":true,"steps":[{"line":1,"op":"MsgBox","detail":"hello","ok":true,"elapsed_ms":0}],"output":["hello"],"notes":[],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":0,"variables":{},"warnings":[]}
```

Every statement that completes produces a step; a failure stops the
script and is reported once, in `error` and `error_line`. A question the
desktop cannot answer fails with the **reason**; it does not answer
`false`. The whole shape of the report is in
[docs/runner-and-report.md](docs/runner-and-report.md).

## The dialects

  - `--dialect=v2`: the v2, the default.
  - `--dialect=v1`: the v1.
  - `--dialect=v2.1`: the v2 with the mechanisms of the v2.1.
  - `--dialect=ks`: the v2 with the mechanisms of the v2.1 and the
    extensions of KeySharp.
  - `--dialect=ks20`: the extensions of KeySharp without the mechanisms
    of the v2.1.

A script says which dialect it needs, on a line of its own:

```ahk
#Requires v2
```

The line names the dialect, and it is the fact the script actually
depends on. The dialects are **v1** and **v2** -- names, not
sub-versions.

## The crates

  - [`ahk`](https://crates.io/crates/ahk): the language. Lexer, AST,
    parser, runtime, built-in table, class library, variables, regex,
    dates, files, settings, hotkeys, and the fifteen `Desktop`
    capability traits. It does no I/O of its own, and it builds without
    the standard library.
  - [`ahk-box`](https://crates.io/crates/ahk-box): the desktop backend.
    A uinput keyboard of its own, an absolute pointer, `grim` captures,
    the Wayland toplevel enumeration, an X11 path, the files, the
    native calls and the Windows-library helper.
  - [`ahk-cli`](https://crates.io/crates/ahk-cli): the runner. A script
    on stdin, a report on stdout.

The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The
language crate knows nothing about the box: it is handed a `Desktop`
implementation, and that is why the language can be tested against a
fake that records the calls.

## The box

The language is portable; the desktop is not. On a Linux Wayland session
the backend drives a uinput keyboard of its own, an absolute pointer,
the screen through `grim`, the windows through the compositor, the GUI
and the menus through the GTK helper, the clipboard through `wl-copy`,
the dialogs through `zenity`, the sound through `pactl`, the object model
through D-Bus and the network through `curl`; on X11 it uses XTEST,
`XGetImage` and the Xlib window list. On Windows it is the real machine.
What each one answers is in
[docs/desktop-backend.md](docs/desktop-backend.md).

## The documentation

All of it is in [`docs/`](docs/README.md), which indexes every document:
the language guide, the error model, the dialects, the surface and the
per-name reference (thirteen languages), the runner and the report, the
desktop backends and the SIMD builds.

## Development

```sh
cargo test --workspace                         # the suite, no box needed
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
ahk --surface --json                           # the parity of the v2, for CI
ahk --surface --dialect=v1 --json              # the parity of the v1, for CI
ahk --surface --dialect=v2.1 --json            # the parity of the v2.1, for CI
ahk --surface --dialect=ks --json              # the parity of the ks, for CI
ahk --surface --dialect=ks20 --json            # the parity of the ks20, for CI
```

The tests need no machine: the desktop is a fake that records the calls.

## Star history

<a href="https://star-history.com/#danalec/ahk&Date">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=danalec/ahk&type=Date&theme=dark">
<source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=danalec/ahk&type=Date">
<img alt="Star History Chart" src="https://api.star-history.com/svg?repos=danalec/ahk&type=Date">
</picture>
</a>

## License

Apache 2.0. The text is in `LICENSE`, and every crate manifest says the
same.
