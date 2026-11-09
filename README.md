# ahk

An AutoHotkey interpreter written in Rust, with a desktop backend for driving
a Linux machine: a lexer, an AST, a parser, a runtime, the built-in names and
the class library, plus a keyboard, a pointer, screen captures, files, windows
and hotkeys.

**The default dialect is AutoHotkey v2.** A second dialect, **AutoHotkey v1**,
lives behind a flag and is described in detail below. One binary serves both.

The names, the classes, the directives and the operators are read out of the
reference binaries into tables under `crates/ahk/reference/`. Those tables are
the contract: the test suite asks the tree about every name in them.

## Quick start

```sh
cargo build --workspace
cargo build --release -p ahk-cli      # the runner: ./target/release/ahk
```

The runner reads a script on **stdin** and writes a JSON report on **stdout**:

```sh
echo 'MsgBox "hello"' | ahk
```

```json
{"ok":true,"steps":[{"line":1,"op":"MsgBox","detail":"hello","ok":true,"elapsed_ms":0}],"output":["hello"],"notes":[],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":0,"variables":{},"warnings":[]}
```

A v1 script needs `--dialect=v1`:

```sh
printf 'MsgBox, hello\n' | ahk --dialect=v1
```

Other flags:

| Flag | What it does |
|---|---|
| `--dialect=v2` | v2 (default; may be omitted) |
| `--dialect=v1` | v1 |
| `--timeout=N` | Stop after N seconds (default 60) |
| `--surface` | Print the state of every name of the surface, one line each |
| `--surface --json` | The same, as JSON, for CI |
| `--surface --dialect=v1` | The surface of the v1 instead |

Exit codes: `0` the script finished, `1` it failed, `2` it did not parse.
`exit_code` in the report is what `ExitApp` set, not the process exit code.

## The crates

| Crate | What it is |
|---|---|
| `crates/ahk/` | The language. Lexer, AST, parser, runtime, built-in table, class library, variables, regex, dates, files, settings, hotkeys, and the fifteen `Desktop` capability traits. **No I/O of its own**, and it builds without the standard library. |
| `crates/ahk-box/` | The desktop backend: a uinput keyboard, an absolute pointer, `grim` captures, Wayland toplevel enumeration, an X11 path, files, native calls, and a Windows-library helper. Linux syscalls are behind `#[cfg(target_os = "linux")]`; on other targets the devices answer `NotLinux`. |
| `crates/ahk-cli/` | The `ahk` runner. A script on stdin, a report on stdout. |

The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The language
crate knows nothing about the box: it is handed a `Desktop` implementation.
That is why the language can be tested against a fake that records calls.

Without the standard library:

```sh
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
```

With `std` off the crate is `#![no_std]`; the host supplies the clock through
the `Clock` trait (`now_ms`, `sleep_ms`, and optionally the wall clock). See
the crate-level docs in `crates/ahk/src/lib.rs` for a worked example.

## v2: the default dialect

This is AutoHotkey v2. Write a v2 script and hand it to the runner with
no flag at all.

```ahk
; expressions, functions, classes, arrays, maps
name := StrUpper("hello")
a := [1, 2, 3]
m := Map("x", 1, "y", 2)
for value in a
    FileAppend value, "*"        ; "*" is stdout
```

A few things worth knowing:

- **Names are v2's names, and only v2's.** `StrUpper`, not `Upper`;
  `PixelGetColor`, not `PixelGet`; `MsgBox`, not `Msg`. There are no aliases.
  The built-in table carries 243 commands and 103 functions; one extension,
  `Screenshot`, writes a screen capture to a file because v2 has no word for
  it.
- **A function call may leave out the parentheses.** A name at the start of a
  line, with no parentheses, is a call statement whose parameters are the rest
  of the line: `Abs 5`, `FileAppend "hello", "*"`.
- **Assignment is `:=`; `=` compares.** `==` compares with case sensitivity,
  `=` without it.
- **References.** A parameter written `&Left` is an alias of the caller's
  variable: `Swap(&a, &b)`. Built-ins that write back use the same marker:
  `SplitPath "C:\dir\file.txt", &name, &dir, &ext`.
- **Functions are values.** `f := Abs` gives a `Func`, and `f.Call(3)` calls
  it. Parameters may have defaults (`MyFunc(a, b := 1)`) or take the rest
  (`MyFunc(a, rest*)`).
- **Classes.** `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`,
  `Hotkey` and `Enumerator` are runtime classes a script cannot extend.
  Script classes have prototypes, `this`, `base`, `is`, and property getters
  and setters. Every value carries `__Class`.
- **Values.** Two values side by side concatenate: `"a" 1 + 2` is `a3`.
  `[1, 2]` is an array, `{a: 1}` an object. Positions count from one:
  `SubStr("hello", 2)` is `ello`.
- **Errors.** `try`/`catch` with a tree of error classes (`Error`,
  `TypeError`, `ValueError`, `OSError`, `IndexError`, ...).
- **Directives.** Nineteen of them (`#Include`, `#Warn`, `#Requires`,
  `#SingleInstance`, `#HotIf`, ...). A directive asking for something the
  runtime does not have fails the script with the reason.
- **Keys.** `{Name}` with the usual modifiers: `^` ctrl, `+` shift, `!` alt,
  `#` super. `Send "^a"`, `Send "+{Tab}"`, `Send "{Raw}text"`.
- **The desktop.** `Send`, `Click`, `MouseMove`, `PixelGetColor`,
  `PixelSearch`, `Screenshot`, `WinExist`, `WinActive`, `WinGetTitle`,
  `FileAppend`, `IniRead`, `DllCall`, `Run`, `Hotkey`, `Hotstring` and friends
  act on the box. A name about a Windows API is still in the surface, and it
  answers with the reason the desktop cannot do it -- it never answers `false`.

The full state of every name is in `docs/v2-capability-report.md`, generated
from `ahk --surface`.

## v1: how to use it

**Pass `--dialect=v1` to the runner.** v2 stays the default; nothing about a
v2 script changes. The dialect is a property of the script, not of the binary,
so one build runs both:

```sh
# v2 (default)
echo 'x := StrLen("abc")' | ahk

# v1
printf 'StringLen, n, abc\nFileAppend, %n%, *\n' | ahk --dialect=v1
```

The v1 dialect is **AutoHotkey v1 as measured on the reference binary**.
It has its own surface table, its own built-in table, its own grammar and its
own built-in variables. Its surface is complete: every name of the v1 either
answers with a value or is refused with a reason, and the pending table of the
v1 is empty.

### What changes when you switch dialects

| | v2 (default) | v1 (`--dialect=v1`) |
|---|---|---|
| Legacy command | `FileAppend "hello", "*"` | `FileAppend, hello, *` -- commas, no parentheses |
| Assignment | `x := 5` | `x = 5` assigns a **text**; `x := 5` also works |
| Comparison | `=` without case, `==` with case | same, plus legacy `if x = 1 + 1` compares against the **text** `1 + 1` |
| Parameters | expressions | the **name of a variable** for output variables: `StringLen, n, x` writes `n` |
| Substitution | none inside strings | `%name%` is the variable of the script (or the environment with `#NoEnv` off) |
| `if` | expression + block | one statement on the next line, legacy `if`/`IfInString` forms, `else` |
| Labels | none | `MyLabel:` and `goto MyLabel` |
| Hotstrings | `Hotstring(":btw:", Callback)` | a statement: `::btw::by the way`, or a multi-line body ending in `return` |
| Hotkeys | `Hotkey("^a", Callback)` | `Hotkey, ^a, MyLabel` and a label definition |
| Directives | the 19 of v2 | the 29 of v1 (`#NoEnv`, `#Persistent`, `#SingleInstance`, `#Warn`, `SetFormat`-era settings, ...) |
| Classes | runtime class types | only classes of the script (`new`, `this`, `__Init`); no `Map`/`Array`/`Buffer` |
| Numbers | integer and float apart | "pure numbers" that keep the text that was written (`0x10`, `00123`) |
| Failure | an error stops the script | many failures are **blank values** instead: unset variable, division by zero, bad index |
| Errors | `try`/`catch` | `OnError` callback; load errors on stderr, runtime errors in a dialog in the reference binary |
| `ErrorLevel` | not a variable | a global variable many commands write |

### A first v1 script

```ahk
; hello, with the legacy command syntax
MsgBox, hello
FileAppend, hello, *

; output variables and legacy assignment
name = world
StringLen, n, name
FileAppend, %name% is %n% letters, *

; labels and goto
goto Done
    FileAppend, never, *
Done:
FileAppend, done, *

; a loop that counts
Loop, 3
    FileAppend, [turn], *
```

`*` is stdout in both dialects. With `--dialect=v1` the report carries the
same fields as a v2 run:

```json
{"ok":true,"steps":[...],"output":["hello","hello","world is 5 letters","done","[turn]","[turn]","[turn]"],"notes":[],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":1,"variables":{"name":"world","n":5,"errorlevel":0}}
```

### The parts of v1 that matter most in practice

- **Commands are comma-separated lines.** The first parameter of many
  commands is the *name of a variable to write* (`OutputVar`), not a value:
  `StringLen, n, text`, `MouseGetPos, x, y`, `SplitPath, path, name, dir`.
  `%name%` substitutes a variable into a parameter; `FileAppend, % x, *`
  forces an expression.
- **Legacy `if` and flow.** `if (x > 1)` with its body on the next line,
  `else`, `IfInString`, `IfNotInString`, `IfMsgBox`, `IfWinActive`, `Loop`,
  `While`, `Until`, `For`, `Break`, `Continue`, `Return`, `Switch`.
- **Loop flavours.** `Loop` (count or files), `Loop Files`, `Loop, Read, file`,
  `Loop, Parse, var, delimiters`, `Loop, Reg`. The loop variables
  (`A_Index`, `A_LoopFileName`, `A_LoopFileFullPath`, `A_LoopReadLine`,
  `A_LoopField`, ...) are set for the body.
- **Settings and formats.** `SetFormat`, `StringCaseSense`, `AutoTrim`,
  `SetBatchLines`, `CoordMode`, `DetectHiddenWindows` and the corresponding
  `A_*` variables. `FileEncoding`, `EnvAdd`/`EnvSub` do date arithmetic with
  `YYYYMMDDHH24MISS` values.
- **Files and the machine.** `FileAppend`, `FileRead`, `FileReadLine`,
  `FileCopy`, `FileMove`, `FileDelete`, `FileGetSize`, `IniRead`, `IniWrite`,
  `Run`, `RunWait`, `DllCall`, `ClipWait`, `PixelGetColor`, `MouseMove`,
  `WinExist`, `WinActivate` and the rest of the v1 commands act on the same
  desktop as v2. Failures that the machine reports set `ErrorLevel` and the
  script goes on, the way v1 does.
- **Built-in variables.** The v1 has its own set (137 of them) plus
  `ErrorLevel`, `Clipboard`/`A_Clipboard`/`ClipboardAll`, `A_FormatFloat`,
  `A_FormatInteger`, `A_StringCaseSense`, `A_AutoTrim`, `A_NumBatchLines` and
  the loop variables.
- **Warnings.** `#Warn` with the v1's types (`UseUnsetLocal`,
  `UseUnsetGlobal`, `UseEnv`, `LocalSameAsGlobal`, `ClassOverwrite`,
  `Unreachable`, `All`) and modes (`MsgBox`, `StdOut`, `OutputDebug`, `Off`).
- **Objects and script classes.** `Object()`, `{}`, arrays as objects with a
  `Length()` **method** (not a property), `new C`, `this`, `__Init`. A member
  that does not exist answers a blank text instead of failing.

### Limitations of v1, by design

These are deliberate decisions documented in `docs/ahk-v1-dialect.md`:

- **No threads.** v1.1 runs each hotkey in its own thread. This runtime is
  cooperative and single-threaded. `Thread`, `Critical`, `#MaxThreads` and
  the like are carried but refused with the reason; `SetBatchLines` is
  accepted as a no-op.
- **No Windows-only machinery.** The registry, COM, `Gui`/`Menu` controls,
  `Control*` and other Windows API names are refused with the reason, as they
  are in v2.
- **Machine-dependent differences.** A name whose answer belongs to the
  machine (short file names on Linux, the clipboard binary formats, the
  screen the compositor exposes) is answered by the desktop, not invented.
- **A few measured corners of the reference binary are not reproduced.**
  They are listed in the plan; the ones that remain are esoteric parse-time
  corners, not everyday scripts.

### Checking the v1 surface

```sh
ahk --surface --dialect=v1          # one line per name, with a summary
ahk --surface --dialect=v1 --json   # the same for CI
```

Both dialects are covered by a parity test that reads the reference tables
at compile time: a name that the tree stops carrying fails the build of a
test, not a run of a script.

## The report

Every statement that completes produces a step (line, what it did, whether it
worked, how long it took). A failure stops the script and is reported once in
`error` and `error_line` -- it is not a step. `notes` record where the machine
was not what the script assumed. A question the desktop cannot answer fails
with the **reason**; it does not answer `false`. A window that is not there is
not such a question: `WinExist` and `WinActive` answer `0`.

Fields: `ok`, `steps`, `output`, `notes`, `error`, `error_line`, `exit_code`,
`elapsed_ms`, `variables`, `warnings`. A parse error is a reduced payload with
`column` and no `elapsed_ms`, `exit_code` or `variables`. `warnings` holds the
warnings a `#Warn` directive asked for (both dialects have one).

The runner also has guard rails: a limit on statements (20 000), on loop
iterations (1 000), on nested calls (1 000), a stack budget, a 60 000 ms cap
on `Sleep`, and the `--timeout` bound.

## The box (desktop backend)

The language is portable; the desktop is not. On a Linux Wayland session the
backend uses:

- a **uinput keyboard** of its own, typed one key at a time with a US layout;
- an **absolute pointer** that declares `INPUT_PROP_DIRECT` and absolute axes;
- **`grim`** for screen captures and pixel reads, and the `png` crate to
  search for images;
- **Wayland toplevel enumeration** (`ext_foreign_toplevel_list_v1`, plus the
  COSMIC extension for focus) for `WinExist`/`WinActive`/`WinGetTitle`, with
  an X11 path behind `YOKO_DESKTOP=x11`;
- **`wl-copy`/`wl-paste`** for the clipboard, **zenity** for dialogs,
  **curl** for downloads, and a native-call layer that loads libraries of the
  machine (and, on Linux, a helper that can load Windows libraries).

To build the runner on the box, copy `crates/` and the workspace `Cargo.toml`
to the machine and run `cargo build --release -p ahk-cli` there. The copy
matters: building without copying compiles whatever sources the box still
has. See `docs/wayland-toplevel-enumeration.md` for the window protocols and
`AGENTS.md` for the box notes.

Hotkeys watch a keyboard device named by `AHK_HOTKEY_DEVICE` -- normally the
physical keyboard -- so a script typing "a" can fire the hotkey "a" without a
person at the keyboard.

## The reference tables

`crates/ahk/reference/` holds what the binaries have:

| File | What it is |
|---|---|
| `ahk-v2.0.19.surface.tsv` | Every v2 name: kind, class for a method, handler address |
| `ahk-v2.0.19.operators.tsv` | Operators and their token kinds |
| `ahk-v2.0.19.pending.tsv` | Names v2 has that the tree does not carry -- **empty** |
| `ahk-v1.1.37.02.surface.tsv` | Every v1 name, with kind, arity and parameter shape |
| `ahk-v1.1.37.02.pending.tsv` | Names v1 has that the tree does not carry -- **empty** |

The tables are read when the tests are compiled, so a name that moves between
them breaks the build of a test.

## Development

```sh
cargo test --workspace                         # ~2000 tests, no box needed
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
ahk --surface --json                           # v2 parity for CI
ahk --surface --dialect=v1 --json              # v1 parity for CI
```

The tests need no machine: the desktop is a fake that records calls. Where a
case goes: a statement in `crates/ahk/tests/language.rs`, a built-in in
`tests/builtins.rs`, a built-in variable in `tests/variables.rs`, a class type
in `tests/class_*.rs`, the v1 dialect in `tests/v1.rs`, a grammar rule in
`parser.rs`, a token in `lexer.rs`, the pointer ABI in
`crates/ahk-box/src/pointer.rs`, the runner in `crates/ahk-cli/tests/runner.rs`.

`AGENTS.md` is the working-notes journal for the tree. The full documentation
is in `docs/`; start at [docs/README.md](docs/README.md), which indexes the
language guide, the two reference chapters, the runner and report chapter, the
desktop backend, the errors and the dialect differences. `docs/` also keeps the
design and measurement records of the port.

## License

Apache 2.0. The text is in `LICENSE`, and every crate manifest says the same.

## Star history

<a href="https://star-history.com/#danalec/ahk&Date">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=danalec/ahk&type=Date&theme=dark">
<source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=danalec/ahk&type=Date">
<img alt="Star History Chart" src="https://api.star-history.com/svg?repos=danalec/ahk&type=Date">
</picture>
</a>
