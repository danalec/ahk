# ahk-rust

The AutoHotkey language, in Rust: a lexer, an AST, a parser, an
interpreter, and the key table.

```
crates/ahk/       the language
crates/ahk-box/   the box primitives -- the uinput keyboard, the absolute pointer, the captures
crates/ahk-cli/   the runner: a script on stdin, a report on stdout (the binary is `ahk`)
```

The parser is portable and does no I/O of its own. The runtime runs
**on the box**, because that is where the desktop is.

## The crates

- **`ahk`** -- the language: the lexer, the AST, the parser, the
  interpreter and the key table. It does no I/O at all, and it does not
  know what a desktop is: it runs a script against any implementation of
  the desktop interface.
- **`ahk-box`** -- the lab box as a script target: a keyboard of our own
  (uinput), an absolute pointer, and the screen captures (grim). The code
  that touches the box is behind `#[cfg(target_os = "linux")]`, so a
  Windows build does not compile it.
- **`ahk-cli`** -- the runner. A script on stdin, a report on stdout. The
  binary is `ahk`, and it is the one that runs on the box.

## The two commands that matter

```sh
cargo test --workspace          # the dialect tests: they need no box
cargo build --release -p ahk-cli # the runner, for the box
```

The tests do not need the box: the desktop is a fake that records the
calls, so they are about what the language means, not about what the
desktop does. The box code is checked on the box:

```sh
ssh 192.168.0.40 'cd ~/ahk-rust && cargo build --release -p ahk-cli'
```

## The Python is the reference

The Python implementation next door -- `C:\Users\danalec\Documents\src\ahk`
-- is the reference. **The two have to speak the same dialect, with the
same test cases.** A feature that exists in only one of them is a bug in
the port, not a difference.

When the language grows, the same cases go into
`crates/ahk/tests/language.rs` and into the Python test suite.

## The language

AutoHotkey's, cut down to what a script can do to a Linux desktop.

**Statements.** `Send`, `Click`, `MouseMove`, `MouseDown`, `MouseUp`,
`Sleep`, `Run`, `PixelGet`, `Screenshot`, `Msg`, `If`/`ElseIf`/`Else`,
`Switch`/`Case`/`Default`, `While`, `Loop`, `Break`, `Continue`,
`Return`, `ExitApp`, and the assignment operators (`=`, `:=`, `+=`, `-=`,
`.=`).

**Keys.** A key is written `{Name}`, and the modifiers are the ones
AutoHotkey uses: `^` is ctrl, `+` is shift, `!` is alt and `#` is the
super key. A modifier on its own is a literal character, and so is one
in braces.

```ahk
Send "^a"        ; ctrl+a
Send "+{Tab}"    ; shift+tab
Send "{+}"        ; a literal plus
Send "{a down}"   ; the key goes down and stays down
```

**Functions.** Your own, with a block and a `Return`; and the builtins
`Abs`, `Sqrt`, `Mod`, `Floor`, `Ceil`, `Round`, `Min`, `Max`, `StrLen`,
`SubStr`, `StrReplace`, `InStr`, `Trim`, `Upper`, `Lower`, `Number`,
`Format`, `Sleep`.

The positions count from one, the way AutoHotkey counts them:
`SubStr("hello", 2)` is `ello`, and `InStr("hello", "ll")` is `3`.

## The report

Every statement produces a step: the line, what it did, whether it
worked, how long it took. A question the box cannot answer fails with
the reason -- it does not answer `false`.

```json
{
  "ok": true,
  "steps": [
    {"line": 1, "op": "Msg", "detail": "hello from the box", "ok": true, "elapsed_ms": 0}
  ],
  "output": ["hello from the box"],
  "notes": [],
  "error": null,
  "elapsed_ms": 380,
  "variables": {"colour": "#12101b"}
}
```

## Where the rest went

The lab MCP server -- the discovery, the screen, the input, the Sunshine
API, the Moonlight client -- lives in the `yoko-rust` repository, next
door. It drives this runtime on the box, so there is one implementation
of "drive the box" and it is the one that works there.

The working notes of the box -- the devices, the timings, the traps -- are
in `AGENTS.md`.
