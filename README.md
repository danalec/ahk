# ahk

An AutoHotkey interpreter written in Rust, with a desktop backend for
driving a Linux machine: a lexer, an AST, a parser, a runtime, the
built-in names and the class library, plus a keyboard, a pointer, screen
captures, files, windows and hotkeys.

**The dialect is v1 or v2.** The v2 is the default and the v1 lives
behind a flag, and the v2 has two modes behind flags of their own: the
mechanisms of the v2.1, and the extensions of KeySharp.

## Quick start

The runner is on crates.io:

```sh
cargo install ahk-cli
```

Or build it from this tree:

```sh
cargo build --workspace
cargo build --release -p ahk-cli      # the runner: ./target/release/ahk
```

The runner reads a script on **stdin** and writes a JSON report on
**stdout**:

```sh
echo 'MsgBox "hello"' | ahk
```

```json
{"ok":true,"steps":[{"line":1,"op":"MsgBox","detail":"hello","ok":true,"elapsed_ms":0}],"output":["hello"],"notes":[],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":0,"variables":{},"warnings":[]}
```

## The dialects

| Flag | What it carries |
|---|---|
| `--dialect=v2` | the v2 -- the default |
| `--dialect=v1` | the v1 |
| `--dialect=v2.1` | the v2 with the mechanisms of the v2.1 |
| `--dialect=ks` | the v2 with the mechanisms of the v2.1 and the extensions of KeySharp |
| `--dialect=ks20` | the extensions of KeySharp without the mechanisms of the v2.1 |

A script says which dialect it needs, on a line of its own:

```ahk
#Requires v2
```

The line names the dialect, and it is the fact the script actually
depends on. The dialects are **v1** and **v2** -- names, not
sub-versions.

## The crates

| Crate | What it is |
|---|---|
| [`ahk`](https://crates.io/crates/ahk) | The language. Lexer, AST, parser, runtime, built-in table, class library, variables, regex, dates, files, settings, hotkeys, and the fifteen `Desktop` capability traits. **No I/O of its own**, and it builds without the standard library. |
| [`ahk-box`](https://crates.io/crates/ahk-box) | The desktop backend: a uinput keyboard, an absolute pointer, `grim` captures, Wayland toplevel enumeration, an X11 path, files, native calls, and a Windows-library helper. |
| [`ahk-cli`](https://crates.io/crates/ahk-cli) | The `ahk` runner. A script on stdin, a report on stdout. |

The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The
language crate knows nothing about the box: it is handed a `Desktop`
implementation, and that is why the language can be tested against a
fake that records calls.

Without the standard library:

```sh
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
```

## The report

Every statement that completes produces a step; a failure stops the
script and is reported once, in `error` and `error_line`. A question
the desktop cannot answer fails with the **reason**; it does not answer
`false`. The whole shape of the report is in
[docs/runner-and-report.md](docs/runner-and-report.md).

## The box

The language is portable; the desktop is not. On a Linux Wayland session
the backend drives a uinput keyboard of its own, an absolute pointer, the
screen through `grim`, the windows through the compositor, the clipboard
through `wl-copy`, the dialogs through `zenity`, and the network through
`curl`. What each one answers is in
[docs/desktop-backend.md](docs/desktop-backend.md).

## Documentation

The documents are in [`docs/`](docs/README.md), and that page indexes
them: the language guide, the errors, the dialects, the two reference
chapters (one per dialect), the surface (every name of the three
dialects, with the state of the tree, in eight languages), the runner and
its report, the desktop backend, and the records of the design and of the
measurements.

| Document | What it holds |
|---|---|
| [language-guide.md](docs/language-guide.md) | the language itself, from the lexical structure to the classes and the statements |
| [errors.md](docs/errors.md) | the error model, the fourteen classes of the v2, and the guard rails |
| [dialects.md](docs/dialects.md) | the gate between the two dialects, and how to port a script either way |
| [ahk-ks-dialect.md](docs/ahk-ks-dialect.md) | the extensions of KeySharp, the table of their surface, and the machine behind them |
| [surface/README.md](docs/surface/README.md) | the surface: it indexes every name of the v1, of the v2 and of KeySharp, with the state this tree answers for each, and the crosswalk between the three. It is written in eight languages: English, Español, Português (Brasil), Français, Deutsch, Русский, 日本語 and 中文 |
| [ahk-v2-reference.md](docs/ahk-v2-reference.md) | every name of the v2, with the state the tree answers for each |
| [ahk-v1-reference.md](docs/ahk-v1-reference.md) | the same catalogue for the v1 |
| [runner-and-report.md](docs/runner-and-report.md) | the command, its flags, and the whole shape of the report |
| [desktop-backend.md](docs/desktop-backend.md) | how a script touches the machine, and what each capability answers |

The tables of the surfaces are in `crates/ahk/reference/`: the v2, the
v1, and the surface of KeySharp. They are the contract of the tree --
the tests read them when they are compiled, so a name that moves
between them breaks the build of a test.

## Development

```sh
cargo test --workspace                         # the suite, no box needed
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
ahk --surface --json                           # the parity of the v2, for CI
ahk --surface --dialect=v1 --json              # the parity of the v1, for CI
ahk --surface --dialect=ks --json              # the parity of the ks, for CI
```

The tests need no machine: the desktop is a fake that records calls.

## License

Apache 2.0. The text is in `LICENSE`, and every crate manifest says the
same.
