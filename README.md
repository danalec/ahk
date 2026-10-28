# ahk

AutoHotkey v2, in Rust: a lexer, an AST, a parser, an interpreter, and
the key table.

The dialect speaks the language of **AutoHotkey v2.0.19**. The tables in
`crates/ahk/reference/` are read out of the binary of v2.0.19 itself,
and they are the contract of the port: the names, the classes, the
directives and the operators are the ones the binary has.

```
crates/ahk/       the language
crates/ahk-box/   the box primitives -- the uinput keyboard, the absolute pointer, the captures
crates/ahk-cli/   the runner: a script on stdin, a report on stdout (the binary is `ahk`)
```

The parser is portable and does no I/O of its own. The runtime runs
**on the box**, because that is where the desktop is.

## The crates

- **`ahk`** -- the language: the lexer, the AST, the parser, the
  interpreter, the class types and the key table. It does no I/O at all,
  and it does not know what a desktop is: it runs a script against any
  implementation of the desktop interface. It also runs without the
  standard library: turn the `std` feature off and the crate builds for
  a target that has no `std` at all, and the host supplies the clock.
- **`ahk-box`** -- the lab box as a script target: a keyboard of our own
  (uinput), an absolute pointer, and the screen captures (grim). The code
  uinput syscalls are behind `#[cfg(target_os = "linux")]`, so a Windows
  build leaves them out and the device answers `NotLinux` when it is
  used. `desktop.rs` is not behind that gate at all, and the native call
  is not either: it loads a library of the machine and calls into it, on
  both platforms.
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
ssh danalec@192.168.0.40 'mkdir -p ~/ahk'
scp -r crates Cargo.toml danalec@192.168.0.40:~/ahk/
ssh danalec@192.168.0.40 'cd ~/ahk && cargo build --release -p ahk-cli'
```

## The language

AutoHotkey's, cut down to what a script can do to a Linux desktop.

**Statements.** `Send`, `Click`, `MouseMove`, `MouseDown`, `MouseUp`,
`Sleep`, `Run`, `PixelGetColor`, `PixelSearch`, `Screenshot`, `MsgBox`,
`If`/`ElseIf`/`Else`, `Switch`/`Case`/`Default`, `While`, `Loop`,
`For`, `Break`, `Continue`, `Return`, `Exit`, `ExitApp`, the class
definitions,
and the assignment operators (`:=`, `+=`, `-=`, `*=`, `/=`, `//=`, `.=`,
`&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`).

**Names.** The names are v2's names, and only v2's names: `StrUpper` and
not `upper`, `PixelGetColor` and not `pixelget`, `MsgBox` and not `msg`.
There are no aliases. The built-in table of the crate is the list of
what the dialect carries -- 243 commands and 103 functions -- and every
name in it is a name of v2.0.19, with one exception: `Screenshot` writes
a picture of the screen to a file, and v2 has no word for it.

**Functions.** Your own, with a block and a `Return`; and the built-ins,
which are values: `f := Abs` gives a `Func`, and `f.Call(3)` calls it.
A parameter may carry a default -- `MyFunc(a, b := 1)` -- or take the
rest of the arguments -- `MyFunc(a, rest*)` -- and the rest is an
array.

**References.** A parameter may be written by reference -- `Swap(&Left,
&Right)` -- and the caller passes a reference -- `Swap(&a, &b)`. The
parameter is then the variable of the caller, and a write inside the
function is a write outside it. The built-ins that write back take the
same: `SplitPath "C:\dir\file.txt", &name, &dir, &ext` fills the
variables of the caller, and `StrReplace("aaa", "a", "b", true, &count)`
leaves the number of the swaps in `count`.

**Calls.** The parentheses may be left out when the answer is not needed
and the name is at the start of the line: the rest of the line is the
list of the parameters. So `Abs 5` and `Abs(5)` are the same call, and
so is a function of the script -- `MyFunc` calls `MyFunc()`.

The positions count from one, the way AutoHotkey counts them:
`SubStr("hello", 2)` is `ello`, and `InStr("hello", "ll")` is `3`.

**Native calls.** `DllCall` calls a function of a library of the machine:
`DllCall("kernel32\GetTickCount")` answers the count of the clock of
Windows, and `DllCall("libc.so.6\getpid")` answers the number of the
process on Linux. The library and the function go in one string, the
types of the arguments alternate with the values, and the type of the
answer is the last parameter. A `*` at the end of a type is the address
of a variable, and the value comes back from the call:
`DllCall("kernel32\QueryPerformanceCounter", "Int64*", &counter)`
leaves the count of the clock in `counter`. The call is made by the
desktop, and the box makes it on both platforms.

**Windows libraries.** A library of Windows is called by the
raven, a program that emulates a Windows process on Linux: the box
calls the libraries of both kinds. `DllCall("kernel32\GetTickCount")`
answers the count of the clock of Windows, and
`DllCall("libc.so.6\getpid")` answers the number of the process on
Linux. On the box the name of the library decides: a name that carries a
slash or `.so` in it is a library of this machine, and every other name
is a library of Windows. The raven and the helper that runs inside it
are named by the environment -- `AHK_RAVEN` and `AHK_RAVEN_HELPER` --
and a box that has not them says so. On Windows there is no raven:
the machine is the Windows, and every library is called directly.

**Libraries of thirty-two bits.** The raven runs a guest of the width
of the program it is given, so the box calls a library of thirty-two
bits through a second guest: `AHK_RAVEN_HELPER32` is the helper of the
thirty-two bits, and the box asks it when the guest of the sixty-four
bits does not have the library. The libraries of the two widths live in
the same folder -- the folder of the helper -- and each guest takes the
one that is of its own width.

**Values.** Two values side by side are one value, the way v2 joins them:
`"a" 1 + 2` is "a3". The dot does the same, and the dot is looser than
the arithmetic.

**Arrays.** `[a, b]` is the array of two, and `[]` is the array of
nothing. The bracket with a space opens the array and the bracket
without one is the item of the value: `Send [1, 2]` sends an array, and
`Send[1, 2]` is the item of `Send`.

**Classes.** `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu` and
`Hotkey` are classes of the runtime, and a script cannot extend them.
A class type owns its own state and its own names; the base object
answers the names every object has -- `HasProp`, `HasMethod`,
`GetMethod`, `HasBase`, `HasOwnProp`, `OwnProps`, `DefineProp`,
`DeleteProp`, `GetOwnPropDesc`.

**Objects.** Every class has a prototype -- `Alpha.Prototype` -- and the
base of an instance is the prototype of its class. The `is` operator asks
whether an object is of a class: `b is Beta` is 1 when `b` is an instance
of `Beta` or of a class that extends it, and 0 otherwise.

**Directives.** Nineteen of them: `#Include`, `#IncludeAgain`,
`#DllLoad`, `#NoTrayIcon`, `#SingleInstance`, `#UseHook`,
`#HotIfTimeout`, `#Hotstring`, `#MaxThreadsPerHotkey`,
`#MaxThreadsBuffer`, `#MaxThreads`, `#SuspendExempt`,
`#ClipboardTimeout`, `#WinActivateForce`, `#ErrorStdOut`, `#InputLevel`,
`#Warn`, `#Requires` and `#HotIf`. A directive that asks for what this
runtime has not got fails the script with the reason; one that asks for
what is already the case is accepted.

**Loops.** `Loop` counts, and `Loop Files`, `Loop Reg`, `Loop Read` and
`Loop Parse` walk what they walk. `for x in y` walks what `y`
enumerates.

**Keys.** A key is written `{Name}`, and the modifiers are the ones
AutoHotkey uses: `^` is ctrl, `+` is shift, `!` is alt and `#` is the
super key. A modifier on its own is a literal character, and so is one
in braces.

```ahk
Send "^a"        ; ctrl+a
Send "+{Tab}"    ; shift+tab
Send "{+}"        ; a literal plus
Send "{a down}"   ; the key goes down and stays down
Send "{Raw}text"  ; the text is sent as it is written
Send "{DEL 4}"    ; the key four times
```

**Strings.** A string is text, and a `%name%` inside it is the value of
the name. The name is read where the string is written, so a string a
function returns still knows its own parameter. A name that is not set
is left as it is written.

This last one is a dialect feature and not v2's: v2.0.19 does not
interpolate inside a double-quoted string, and the tree keeps the
interpolation because the archive had it. The difference is written down
in `AGENTS.md`.

## The report

Every statement produces a step: the line, what it did, whether it
worked, how long it took. A question the box cannot answer fails with
the reason -- it does not answer `false`.

```json
{
  "ok": true,
  "steps": [
    {"line": 1, "op": "MsgBox", "detail": "hello from the box", "ok": true, "elapsed_ms": 0}
  ],
  "output": ["hello from the box"],
  "notes": [],
  "error": null,
  "elapsed_ms": 380,
  "variables": {"colour": "#12101b"}
}
```

## The reference tables

`crates/ahk/reference/` holds what the binary of v2.0.19 has:

- `ahk-v2.0.19.surface.tsv` -- every name: its kind, the class a method
  belongs to, and the address of its handler in the binary.
- `ahk-v2.0.19.operators.tsv` -- the operators and their token kinds.
- `ahk-v2.0.19.pending.tsv` -- the names of v2 this tree does not carry
  yet, and why. The table is empty: the port carries the whole of v2.

The tables are read when the tests are compiled, so a name that moves
between them breaks the build of a test and not a run of it.

## Where the rest went

The lab MCP server -- the discovery, the screen, the input, the Sunshine
API, the Moonlight client -- lives in the `yoko-rust` repository, next
door. It drives this runtime on the box, so there is one implementation
of "drive the box" and it is the one that works there.

The working notes of the box -- the devices, the timings, the traps --
are in `AGENTS.md`.

## The archive

The Python implementation -- `../archived/ahk-python` -- is the port
this tree came from, and it is archived: it is kept to be read, not to
be run. The two spoke the same dialect before the port to v2, and they
do not speak it now: the rename to the names of v2 is the point of the
port, and the archive is frozen at the dialect it had. The archive is
the record of what the dialect meant before, and this tree is the record
of what it means now.

## License

Apache 2.0. The text is in `LICENSE`, and the manifests of the crates say
the same thing, so the crates can be published as they are.
