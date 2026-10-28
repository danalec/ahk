# AGENTS.md — working notes for ahk

This repository is **the AHK**: the AutoHotkey engine and the box
primitives, in Rust. It was split out of `yoko-rust`, which keeps the lab
MCP server, and it is a port of the Python implementation, which now lives
in the archive (`../archived/ahk-python`).

The Rust is the implementation that runs. **The dialect is AutoHotkey
v2.0.19**: the names, the classes, the directives and the operators are
the ones the binary of v2.0.19 has, and the tables in
`crates/ahk/reference/` are read out of it.

The Python in the archive is the record of what the dialect meant before
the port to v2. It is kept to be read, not to be run. **The two do not
speak the same dialect any more, and that is the point of the port.** The
rule that said they had to agree is in section 1, and it has changed.

**The box** is the lab machine at `192.168.0.40`: Linux, a Wayland
session, the COSMIC compositor. Everything in the second part of this file
happens there, not on the workstation that edits the code.

The first part of this file is the map: the commands, the layout, the
rules. The rest is the working notes -- the things that are easy to break
by accident, each one saying what the rule is and why.

## Setup commands

- Build:         `cargo build --workspace`
- Test:          `cargo test --workspace`
- Lint:          `cargo clippy --workspace --all-targets`
- Without `std`:
  `cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`

The crates depend on each other by path, so there is nothing to install.
The runner takes a script on stdin and writes the report on stdout.

A build on the box is section 12, and it is two steps, not one: copy the
source there first, or the build compiles the sources the box still has,
and the result is a stale binary with no error.

## Project layout

- `crates/ahk/` -- the language: `lib.rs` (the public surface),
  `lexer.rs`, `ast.rs`, `parser.rs`, `runtime.rs`, `keys.rs`,
  `builtins.rs` (the built-in table), `classes/` (the class types),
  `desktop.rs` (the capability traits), `variables.rs` (the built-in
  variables), `regex.rs` (the regular expressions). No I/O of its own.
- `crates/ahk/reference/` -- the tables of v2.0.19: the surface, the
  operators and the pending list. They are read when the tests are
  compiled.
- `crates/ahk-box/` -- the box primitives: `lib.rs` (the public surface),
  `keyboard.rs` (the uinput keyboard), `pointer.rs` (the absolute
  pointer), `desktop.rs` (the captures and the screen reads).
- `crates/ahk-cli/` -- the runner. The binary is `ahk`.
- `crates/ahk/tests/language.rs` -- the dialect tests.
- `crates/ahk/tests/builtins.rs` -- the cases of the built-ins.
- `crates/ahk/tests/variables.rs` -- the cases of the built-in variables.
- `crates/ahk/tests/surface.rs` -- the parity with the surface of v2.
- `crates/ahk/tests/class_surface.rs` -- the probe of the class types.
- `crates/ahk/tests/class_*.rs` -- the cases of each class type.
- `crates/ahk-box/src/pointer.rs` -- the cases of the pointer ABI.
- `crates/ahk-cli/tests/runner.rs` -- the cases of the runner.
- `../archived/ahk-python/` -- the reference implementation. Frozen;
  nothing new lands there.

EGM artifact directory: `egm-review/`

The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`, and `ahk`
depends on nothing in the workspace. The language crate must not reach for
the box: it does no I/O, and it has to keep building without `std`.

## Code style

- The parser and the runtime do no I/O. The desktop is an interface the
  runtime is handed, which is why the language can be tested against a
  fake.
- The uinput syscalls -- opening `/dev/uinput`, writing ioctls, reading an
  event node -- are behind `#[cfg(target_os = "linux")]`, per function: on
  another target they are not compiled and the device answers `NotLinux`.
  `ahk-box/src/desktop.rs` is not behind that gate at all: it compiles on
  Windows too, and fails there when it is used.
- The language crate runs without the standard library. `ahk` has a `std`
  feature, on by default; with it off the crate is `#![no_std]` and builds
  for a target that has no `std` at all. The host then supplies one thing,
  through the `Clock` trait: the clock -- the time the runtime measures
  itself with, and the pause `Sleep` asks for. `now_ms` is the
  milliseconds since an arbitrary origin and never goes backwards;
  `sleep_ms` waits at least that long. The same clock measures the timeout
  and the steps. On a target without `std` the constructor is
  `Runtime::with_clock(desktop, clock, timeout)`; `Runtime::new` and
  `SystemClock` exist only with the feature. `ahk-box` and `ahk-cli`
  always pull `std` in, so the path without it is `-p ahk` alone. The
  crate's own doc has a worked example of the trait.
- A float function that `std` owns -- `sqrt`, `floor`, `ceil`, `round`,
  `fract` -- goes through the helper in `runtime.rs`, which is `libm`
  under `no_std`. Only `abs`, `min`, `max` and `clamp` may be called on an
  `f64` directly: they are in `core`.
- The tree is not in rustfmt's shape, and `cargo fmt --all` would rewrite
  it: the code is written compactly, with a struct variant and a line of
  logic on one line, and the defaults would expand all of that into
  something else. There is no `rustfmt.toml` to say so. The formatter is
  therefore not part of the loop: match the code you see next door, and do
  not run it over the tree.
- The comments and the commits say *why*, in plain sentences, in the same
  voice as this file.
- **The built-in table is the list of what the dialect carries.** Every name
  of v2.0.19 is in `builtins.rs`, with its kind, its arity and the handler
  that answers it. A name that is shaped for Windows is in the table too,
  and the handler answers it with the reason: a question the desktop cannot
  answer fails, and it does not answer `false`.
- **The `Desktop` trait is the sum of nine capability traits** -- `Keyboard`,
  `Pointer`, `Screen`, `Windows`, `Processes`, `Files`, `Clipboard`,
  `Timers` and `Hotkeys`. There is no blanket implementation: every
  implementor writes the nine of them, and every method that a capability
  does not have has a default that says so. A new capability goes in its
  own trait, and `Desktop` grows by naming it.
- **A class type of the runtime is a class too.** `Map`, `Array`, `Buffer`,
  `File`, `Func`, `Gui`, `Menu` and `Hotkey` are classes the script cannot
  extend. Their state lives in the property table of the object under a
  reserved key, and only the module that owns the class touches it.
- **The class module is asked before the base object.** A class that
  answers `OwnProps` itself is taken at its word; the base object answers
  the names every object has only when the module does not.
- **A function and a method are checked apart.** Measured on the binary
  of v2.0.19 on 2026-10-27: a function of the script called with too few
  arguments fails with "Too few parameters passed to function.
  Specifically: MyFunc", and a method called with too few arguments runs.
  The tree does the same, and a parameter may carry a default
  (`Name(a, b := 1)`) or take the rest of the arguments (`Name(a,
  rest*)`), which is an array.
- **The marker of a by-reference parameter is in the definition.** v2
  writes it on the parameter -- `Swap(&Left, &Right)` -- and the caller
  passes a reference -- `Swap(&a, &b)`. The parameter is then an alias of
  the variable of the caller: reading it reads there and writing it writes
  there. A parameter that is not written that way holds the reference as a
  value of its own, and reading it fails.
- **A reference is a value of its own kind, and v2 names it a VarRef.** It
  points at a variable by its name and by its scope, and it is neither a
  string nor a number: the operations that want one of those say that they
  got a VarRef. The reference of a built-in variable, of a function and of a
  class is refused, each with the reason v2 gives.
- **The flag of the table of v2 is the place of the first parameter that is
  written back.** `builtins::reference_parameters` is that column turned into
  the places of the parameters, and a reference in any other place is a
  value where a variable is wanted.
- **The pattern of v2 is exact, and `(?i)` takes the case away.** The engine
  of the tree carries that one option of PCRE, and a pattern that asks for
  another one is refused with the reason.
- **There is no kind of command in v2.** The built-in table has a `kind`
  column, because the binary keeps its names in two tables and the
  column is the record of that. The parser decides by the syntax: a
  name at the start of a line, without parentheses, is a call statement,
  and the v2 documentation says that every function may be written that
  way -- a built-in and a function of the script alike. `Abs 5` is a
  valid v2 script.
- **A parameter that the tree cannot honour fails with the reason.** The
  arity in a row is the arity of the binary, so a call that asks for
  more than the tree can do is taken by the table and refused by the
  handler. It is never ignored: a call that answers with a wrong value
  and says nothing is the worst outcome.
- **An item is written by the class that owns it, and only by that one.**
  Measured on the binary of v2.0.19 on 2026-10-27: `m["a"] := 1` writes an
  entry of a `Map` and `a[1] := 5` writes a place of an `Array` whose
  index is already there. A class of the script has no way to write one:
  `__Set` is not called for it, and `x[y]` asks only the `__Item`, which
  reads and never writes. The tree follows that, and a class that cannot
  write an item fails the statement with the reason.

## Testing instructions

- `cargo test --workspace` -- the dialect tests. They need no box: the
  desktop is a fake that records the calls.
- The Rust suite was the port of the dialect cases, and it is not a copy of
  the archived suite: the archive has cases for the pointer, the CLI and
  the MCP addon that have no Rust counterpart, and the Rust suite has
  builtin cases the archive does not have. The parser and lexer cases are
  the archive's, and in Rust they live in the source files, not in
  `tests/language.rs`. **Since the port to v2 the two trees do not speak
  the same dialect, so a name that is in one and not in the other is not
  a bug any more**: v2's names are the ones that are here, and the cases
  of the archive that are about the old names stay in the archive.
- **The parity with v2 is a test of its own.** `tests/surface.rs` reads the
  surface table and asks the tree, name by name, whether it carries it: the
  built-in table for the commands and the functions, the table of the
  built-in variables for the variables, and the class types for the
  methods. A name that the tree does not carry has to be in the pending
  table, with the reason.
- **The class types are probed, not counted.** `tests/class_surface.rs`
  asks every name of every class of the surface, as a call and as a read,
  and the answer may not be "this value has no such method": the class does
  the thing or it says why it cannot.
- Where a case goes depends on what it is about: a statement goes in
  `crates/ahk/tests/language.rs`, a builtin in `crates/ahk/tests/builtins.rs`,
  a built-in variable in `crates/ahk/tests/variables.rs`, a class type in
  `crates/ahk/tests/class_*.rs`, a grammar rule in `parser.rs`, a token in
  `lexer.rs`, a key in `keys.rs`. The ported archive cases are in those
  places.
- A new language feature gets its cases in the Rust tree. **Not** in the
  archive: that tree is frozen, and it is the record of what the dialect
  meant at that point. The README of this repository still says the
  opposite on this point; this file wins.
- The guard rails and the timeout are language cases too, and they are in
  `tests/language.rs`: a script that loops forever must fail with `error`
  holding "ran more than" and `error_line` set. The timeout is the odd one
  out: it stops the script between two statements, so it knows no line,
  and `error_line` is null.
- The runner's exit codes and the pid file are not language cases: they
  are in `crates/ahk-cli/tests/runner.rs`, next to the cases of the
  archived Python runner.
- The pointer's numbers are in `crates/ahk-box/src/pointer.rs`: the ioctl
  requests, the sizes of the structs they carry, the codes and the clamp.
  Those are the numbers that fail silently, with an `EINVAL` and nothing
  else, so they are pinned against the C definitions here and not on the
  box. The archive's addon cases have no place in this tree: the addon
  lives in `yoko-rust`. The device itself is checked on the box, by hand:
  section 12.

## The guard rails

The runtime stops a script that does not finish, and says so when it does.
A limit is reported as an `error` with the line of the statement that
tripped it.

- `Runtime::MAX_STEPS` -- 20 000 statements in one script. The statement
  that would have been the 20 001st is the one reported: "the script ran
  more than 20000 statements".
- `Runtime::MAX_ITERATIONS` -- 1 000 turns through a `While` body: "the
  loop ran more than 1000 times". A counted `Loop` has no iteration guard
  of its own: it is bounded by the step count and by the timeout.
- `Runtime::MAX_DEPTH` -- 1 000 nested calls. v2.0.19 stops at about 1 105,
  and it says "Function recursion limit exceeded."; the tree stops at the
  same place and says the same thing.
- The stack is the other bound of a recursion. The frames of the tree are
  heavier than the ones of v2 -- about 50 KB of stack per call in a debug
  build and about 5 KB in a release build -- so a host with a small stack
  would run out of it before the depth says so. `Runtime::MAX_STACK` is how
  much stack a run may use (512 KB, when the host does not say), and
  `with_stack_budget` is how the host says otherwise. The runner gives its
  thread 128 MB of stack and says 64 MB.
- `Sleep` is capped at 60 000 ms, in the statement and in the builtin.
- The timeout is the other bound: `--timeout=N` on the runner, in seconds,
  60 by default. It is checked before every statement, so a `Sleep` can
  overshoot it. The message is "the script ran for more than Ns" and
  `error_line` is **null**: the timeout path never sets it.

The runner's exit code is 0 when the script finished, 1 when it failed,
and 2 when it did not parse. The report is JSON on stdout in every case
but one: if the script cannot be read from stdin, the runner writes the
reason to stderr and exits 2 without a report.

## The surface of v2

The dialect is AutoHotkey v2.0.19, and the tables in
`crates/ahk/reference/` say what that means. They are read out of the
binary of v2.0.19, and they are the contract of the port.

- `ahk-v2.0.19.surface.tsv` -- every name of v2: its kind (`command`,
  `function`, `variable` or `method`), the class a method belongs to, and
  the address of its handler in the binary.
- `ahk-v2.0.19.operators.tsv` -- the operators and their token kinds. The
  kinds are **not** a precedence: the precedence is the table in
  `ahk::precedence`, which follows the documentation of v2.
- `ahk-v2.0.19.pending.tsv` -- the names of v2 this tree does not carry
  yet, with the reason. It is empty: the port carries the whole of v2.

**A name is carried when the tree answers it.** The answer may be a value
or the reason the tree cannot give one. A name that is shaped for Windows
is carried the same way, and it is not pending: a script that asks for
`WinActive` is told that the answer is unknown, and not that the window is
not active.

**Linux is the focus, and Windows is kept.** The box is a Linux desktop,
and the primitives of the box are the Linux ones: the uinput keyboard,
the absolute pointer, the `grim` capture. The names of v2 that are about
Windows stay in the surface all the same, and they answer with the
reason they cannot answer: a script that calls `DllCall` or
`ControlSendText` runs, and it is told what the desktop of this tree
cannot do. The names are the names of v2 on every platform, and the
desktop is the one thing that differs.

**The tables are read when the tests are compiled**, so a name that moves
between them breaks the build of a test and not a run of it.

### The tables that the extraction read at the wrong place

The extraction that read the tables out of the binary got two of the class
tables wrong, and the tree would have carried the wrong names with it.

- **The `Buffer` table.** The extraction gave it fifteen rows: `Ptr`,
  `__Item`, `__New`, `__Enum`, `__Get`, `Count`, `Len`, `Mark`, `Name` and
  `Pos` among them. Measured on the binary of v2.0.19 on 2026-10-27 with
  `HasProp` and `HasMethod`, a buffer has **`Ptr` and `Size`** and the
  method **`__New`**, and nothing else: `Pos`, `Len`, `Mark`, `Name`,
  `Count`, `__Get`, `__Set`, `__Enum` and `__Item` are all absent. The
  table was read at the wrong place.
- **The `Array` table.** All thirteen of its rows carry one handler
  address, and the `Buffer` table carries one for its last eight rows: a
  table whose members are methods and properties mixed cannot have one
  handler for all of them. The names of the array are right, and the
  handlers were not resolved.

The lesson is in the tree now: the probe of `tests/class_surface.rs` asks
the tree what it answers, and the answer is what counts -- not the
address in the table.

## PR and commit conventions

- Branch from `master`; never push to `master` directly.
- The commit subject is a full sentence that says what changed and, when
  it is not obvious, why. No `feat:`/`fix:` prefixes, in this history or
  in any other commit in the tree.
- The remote is `forgejo` (`http://192.168.0.10:3000/danalec/ahk.git`).
- The notes in this file change in the same commit as the behaviour they
  describe. The box notes are measurements: a measurement without its date
  is not evidence.

## License

The project is Apache 2.0. The text is in `LICENSE`, and the manifests of
the crates say the same, so they can be published as they are.

## Security

- No secrets in the tree. The credentials of the lab server live in
  `yoko-rust`, not here.
- A script drives the desktop for real: it types, it clicks, it presses
  keys. Read the script before running it, and keep the emergency stop of
  section 8 in mind.

## 1. The three trees, and what may be changed in them

All three crates are ours to change freely: `ahk` (the language),
`ahk-box` (the box primitives) and `ahk-cli` (the runner). The archive is
not: it is frozen, and it is kept to be read.

`yoko-rust` is the lab server. It is ours as well, but it is a server and
not part of this tree, so it is changed when the public shape of this tree
breaks it, and not otherwise. Section 11 says how that happens and how it
is checked.

**The archive is the record of the old dialect.** It lives in
`../archived/ahk-python`, and it is frozen. It is kept to be read: it says
what the dialect meant before the port to v2, and the port to v2 is why
the two do not agree any more.

The rule that the two have to speak the same dialect held until the port
to v2.0.19 and does not hold now: v2's names are the names here, and a
name of the old dialect that v2 does not have -- `Msg`, `Upper`,
`PixelGet` -- is not carried. The cases of the archive stay in the archive,
and the cases of the new dialect go into the Rust tree, under the rule in
"Testing instructions".

## 2. The uinput syscalls only run on the box

Everything that opens `/dev/uinput`, writes ioctls, or reads an event node
is behind `#[cfg(target_os = "linux")]`. The crate still compiles
elsewhere: the gated parts answer `NotLinux` -- "the keyboard is a Linux
uinput device; this is not Linux" -- when they are used. The writing of
the events is compiled everywhere; it simply cannot reach a device that is
not there.

The build on the box, and the check that matters, are section 12.

## 3. The uinput devices have to declare what they are

An event travels: the kernel reads our device, libinput turns it into a
description for the compositor, and the compositor hands the key to the
client -- the application the user is looking at. A device declares what
it can do -- its key codes and its flags -- and the compositor builds its
keymap from that declaration. A wrong declaration is accepted by the
kernel and stops at the compositor.

- **The virtual keyboard declares the same 112 codes as the box's physical
  keyboard.** The list was read out of `/proc/bus/input/devices` on
  2026-10-25 (the `AT Translated Set 2 keyboard`). With a sparse list, or
  with codes a real keyboard does not have, libinput builds a keymap it
  cannot use: the key events arrive at the compositor and are reported
  with the key code `-1`, which a client cannot use. The events are in the
  event node the whole time -- `cat /dev/input/eventN` shows them -- and
  they stop there.
- **The scan codes ride with the key events.** The device emits
  `EV_MSC`/`MSC_SCAN` before each key event -- the kernel's channel for
  the hardware scan code -- and declares the `MSC_SCAN` bit in its
  capabilities.
- **The pointer announces `INPUT_PROP_DIRECT` and absolute axes**, like
  the device Sunshine creates on that box. ydotool's device is a
  *relative* mouse with a full keyboard key bitmap and no
  `INPUT_PROP_POINTER`, so libinput classifies it as a keyboard and
  ignores the relative axes: the click arrives and moves nothing.
- **The pointer is a device of its own.** libinput picks one role per
  device, and the role with the richer key bitmap wins, so the keyboard
  and the pointer cannot be one device.

The numbers behind that -- the ioctl requests and the sizes of the structs
they carry -- are pinned by the cases in `pointer.rs`. They fail silently,
with an `EINVAL` from the kernel and nothing else, so a mistake in them is
caught on the workstation and not on the box.

## 4. The waits the box needs

- **A new device needs four seconds before the compositor adopts it**
  (`ENUMERATION_WAIT`, in `keyboard.rs`). A click that arrives before the
  compositor adopts the device lands nowhere -- and the report still says
  the click happened, because the device accepted the event even though
  the compositor ignored it. The keyboard and the pointer both wait. *The
  archive's pointer constant says five seconds (`src/ahk/box.py:131`),
  although its own rule says four; the Rust waits four, the number the box
  was measured with. Open: whether the pointer needs five.*
- **The pointer needs 200 ms between the motion and the press**
  (`PRESS_SETTLE`, in `pointer.rs`). A press that arrives in the same
  frame as the motion is applied at the position the pointer had before
  the motion. Measured on 2026-10-25: with a 20 ms gap the click landed
  hundreds of pixels away from where it was aimed.
- **A client coalesces a burst of key events** (`KEY_GAP`, 20 ms). A
  hundred backspaces that arrive back to back are counted as a handful,
  not as a hundred. Measured on the lab box on 2026-10-25, a client lost
  most of a burst of 320 backspaces; with this gap between the release and
  the next press, every one of them counts.
- **A key stays down for 20 ms** (`KEY_HOLD`) and **the button for 40 ms**
  (`PRESS_HOLD`): long enough for the compositor to see the press, short
  enough that the auto-repeat never starts.
- **The kernel does not repeat an axis value that did not move.** The Rust
  pointer moves once and then presses, so a click at the position the
  pointer is already at carries no motion with it. Whether the click still
  lands in that case is **open**: it has not been measured.

## 5. The text goes through our own keyboard

`ydotool type` was exact while `ydotoald` was running on that box. It is
not any more: `/usr/bin/ydotoald` was removed from the disk of that box,
the process that was still running has died, and `ydotool type` without a
daemon writes into a socket nobody is listening on and reports nothing.

**`ydotool key` is never used.** ydotool 1.x reads a key *name* as zero
and does nothing at all (the command exits 0 and the screen does not
change), and a numeric *code* presses the key without ever releasing it,
so the compositor's auto-repeat types it forever. One `ydotool key 22`
produced about a hundred and thirty `u`s and left the key held, on
2026-10-25. The modifier suffix (`30:1` for ctrl+a) is not understood at
all.

**The text is typed one key at a time, through the keyboard of our own,
and the layout is the US one.** A character the layout does not have fails
the statement with "the keyboard of this box has no key for it" -- it is
not typed as something else.

## 6. Modifiers reach the client

An earlier version of this file said "a modifier never reaches the client
on this box". That was measured with the **ydotool** device, whose
`key 30:1` ignores the suffix: the chord was never sent.

With a keyboard of our own the chord is sent -- the modifier goes down,
the key goes down, the key comes up, the modifier comes up -- and a client
acts on it: measured on 2026-10-25, `ctrl+a` selected the text of a GTK
field and the next key replaced it; measured again on 2026-10-27, `ctrl+a`
selected the text of a field of the Steam store window and a backspace
deleted it.

**The shift key is what makes capitals work** in the text path, and it
works for the same reason.

## 7. The focus check, and the window questions it cannot answer

`PixelGet` and `PixelSearch` are read from the screen, so they work
whatever window is in front. `MouseGetPos` is not: it reads the position
of the pointer this runtime drives, which is the only pointer on that box
whose place can be read at all -- the physical mouse is relative, so its
place cannot be read.

`WinActive` and `WinExist` need a window API, and the box has none. The
session is Wayland; the COSMIC desktop's D-Bus service answers
`Introspect` with an empty list; the Xwayland server beside it
(gamescope's rootless X server) reports an empty client list. **The answer
is therefore "unknown", and the runtime says so out loud** -- a script
that cannot answer the question fails; it does not quietly take the wrong
branch.

`WinGetTitle` is the exception: with no active X11 window it answers an
empty string, because there is no X11 window to ask about. A native
Wayland window may hold the focus all the same.

## 8. The emergency stop

A script that types forever has to be stoppable from outside, and the only
thing that can stop it is the process.

The runner writes its pid to `$XDG_CACHE_HOME/ahk/run.pid`, or to
`$HOME/.cache/ahk/run.pid` when that variable is not set, and removes it
when the script ends. A host with neither -- a Windows one that nobody
configured -- gets the temporary directory, and never the working
directory: the file has to be where the stop looks for it, and the working
directory changes from one call to the next. The `ahk_stop` tool of the
lab MCP (in the `yoko-rust` repo) reads that file, sends `TERM`, then
`KILL`, then sweeps up whatever is left.

Three things about that file:

- **It is written only after the script parses.** A script that does not
  parse leaves no file at all. The write itself is not checked: a run
  whose pid file could not be written is not stoppable by this route, and
  says nothing.
- **It can be stale.** A process killed by a signal never runs the
  destructor that removes it, and the runner installs no signal handler.
  The file can then hold the pid of a process that is gone -- and pids are
  reused. The `ahk_stop` tool of the lab MCP checks the pid before it
  kills anything: the pid has to be a live process whose command line
  names the runtime. A pid that is not that is reported as stale, the file
  is removed, and nothing is killed.
- **The slot is for one runner.** Two runs on the same box write the same
  file, and the first one to end deletes the second one's file.

From a shell, when the MCP itself is stuck and cannot answer:

```sh
ssh danalec@192.168.0.40 'pkill -f release/ahk'
```

That kills every `ahk` process of that user, not just the runaway one:
`pkill -f` matches the whole command line. The two other bounds are
`--timeout` and `MAX_STEPS`.

## 9. The traps of the environment probe

The environment probe is the command that reports the session in a fixed,
positional format: the screen geometry, and the environment every Wayland
tool needs. The format is positional, so the arguments have to be given in
the order it asks for.

- **The arguments are in the order the format asks for them.** In the
  wrong order the environment is set to the wrong values, and every
  Wayland tool on the box fails to connect without saying why.
- **`~` is not expanded inside quotes.** A quoted `~` is a file named `~`,
  and the shell says "no such file or directory". The path of the runtime
  is passed unquoted for that reason.
- **The geometry goes after the command, not before it.** The environment
  goes before the command: `env ... -g ...` would ask `env` to run a
  program called `-g`.
- **The box has BusyBox, not GNU coreutils.** `base64` has no `-w0` (pipe
  it through `tr -d '\n'`), `ls` has no GNU options, and there is no
  `pgrep -o` (oldest process first).
- **`grim -o` is the output *format*, not the file.** `grim -o /tmp/x.png`
  answers "unknown output". The file is the argument: `grim /tmp/x.png`.
  grim is the screenshot tool, and `grim -` writes the PNG to stdout.

## 10. What the report says

A script that drives a desktop has to be able to say what it did, not just
that it finished.

- Every statement that *completes* produces a step: the line, what it did,
  how long it took. A statement that fails stops the script and is
  reported once, in `error` and `error_line` -- it is not a step, and `ok`
  is true on every step that exists. A client that looks for the failure
  in `steps` will not find it; it has to read `error`.
- The `notes` are the places where the box was not what the script assumed
  -- the environment that was found, the device that had to be waited for.
  The runtime writes its own notes with `Runtime::note`, and the runner
  adds the notes of the box to them.
- A question that cannot be answered fails, with the reason. It does not
  answer `false`. (`WinGetTitle` is the exception; section 7.)

The report is JSON on stdout: `ok`, `steps`, `output`, `notes`, `error`,
`error_line`, `exit_code`, `elapsed_ms`, `variables`. `exit_code` is what
`ExitApp` set, not the exit code of the process: the runner still exits 0.
A parse error is a reduced payload -- `error`, `error_line`, `column`,
`steps`, `output`, `notes` -- with no `elapsed_ms`, `exit_code` or
`variables`.

## 11. The servers live next door

The MCP servers are not in this repository: they live in `yoko-rust`,
which keeps `crates/yoko-mcp` -- the discovery, the screen, the input, the
Sunshine API and the Moonlight client -- and depends on this repository
for the `ahk` crate.

The input tools of that server go through this runtime on the box, so
there is exactly one implementation of "drive the box".

**The server takes the crate by path, so it breaks when the shape of this
tree changes.** The port to v2 did exactly that: a command is a call now,
and the nodes `Send` and `Click` are gone, so the arms of the sibling that
named them did not compile and the server did not build. The sibling was
fixed in its own tree, in the same week -- its own notes say that it may
be changed freely -- and the fix was three arms and two helpers.

So a change to the public shape of this crate -- a node that is gone, a
name that is renamed, a trait whose methods change -- has to be checked
against the server that depends on it:

```sh
cd ../yoko-rust && cargo check --locked --offline --workspace --all-targets
```

The failure is a compile error and not a wrong answer, so it cannot pass
unnoticed: the server does not build, and the tool that drives the box is
not there.

## 12. The box commands

The workspace commands are in "Setup commands". On the box:

```sh
ssh danalec@192.168.0.40 'mkdir -p ~/ahk'
scp -r crates danalec@192.168.0.40:~/ahk/
scp Cargo.toml danalec@192.168.0.40:~/ahk/
ssh danalec@192.168.0.40 'cd ~/ahk && cargo build --release -p ahk-cli'
```

The copy is the point: a build that does not copy the source builds
whatever the box still has, and the result is a stale binary with no
error. `crates/` carries the crate manifests; the workspace `Cargo.toml`
has to be copied too, because it holds the `[workspace.dependencies]` the
crates point at. `Cargo.lock` is not copied: the box resolves the
dependencies itself.

A script goes over as base64, and the report comes back on the ssh's
stdout:

```sh
printf %s '<base64>' | base64 -d | ssh danalec@192.168.0.40 '~/ahk/target/release/ahk --timeout=60'
```

The check that matters is on the box, and it is read-only: a script that
asks for the pointer and for a pixel touches the screen and nothing else.

```ahk
PixelGet 100, 100
pos := MouseGetPos()
Msg "pixel %pixel% at %pos%"
```

The report to expect is `ok: true`, a note saying that the absolute
pointer was opened, and the colour of the pixel. The pointer's creation
takes four seconds, so the step that asks for the position is slow on
purpose.

## 13. What is verified, and what is not

Verified on the box on 2026-10-25, on 192.168.0.40. The colours and the
coordinates below are the ones that were on the screen that day -- the
Orion wallpaper -- so they are the record of a measurement and not what
another screen will answer. What was verified is the method; the values
are here so that a second measurement can be compared with the first.

- `PixelGet 100, 100` reads `#12101b` -- the same colour the archived
  Python implementation reads, and the one the Python `yoko-mcp` README
  records for that pixel of the Orion wallpaper: three implementations,
  one answer.
- `Send "Teste-Yoko 42!"` types the whole string, capitals and punctuation
  included.
- Seven backspaces delete the seven characters before the caret.
- `ctrl+a` selects the text of a GTK field, and the next key replaces it.
- A click at `(518, 305)` selects the "Documents" entry of the sidebar of
  the file manager (the Files app).
- A script that read the pointer's place and the colour under it answered
  `pointer at 960,540 - colour #1b1b1b`. That string is the script's own
  wording, not a format the runtime produces.
- The emergency stop kills a script that is typing forever.

Verified again on 2026-10-27, on the same box, with the screen showing the
Steam store window:

- `PixelGet 100, 100` still reads `#12101b`, and a raw decode of the
  `grim` capture agrees with it: two independent paths, one answer.
- `Send "Teste-Ahk 42!"` types the whole string, capitals and punctuation
  included -- into the search field of the Steam store window, read back
  off a `grim` capture.
- `ctrl+a` followed by a backspace selects the text of that field and
  deletes it: the chord arrives at this client too, not only at a GTK one.
- `Click 830, 316` clicks through the absolute pointer, and the report
  says where it clicked.
- The final build types the same way: `Send "Ahk-Final-42"` landed in that
  field, the capitals and the hyphen included, and `ctrl+a` with a
  backspace emptied it again.
- The emergency stop kills the runtime: the process is gone a moment after
  the kill, and the report of the killed run is empty. The pid file that
  the kill leaves behind is stale, and the check says so instead of
  killing whatever took the pid.
- The reference operator works on the box: `SplitPath` writes the five
  pieces of a path into the variables of the caller, and `MouseGetPos`
  writes the place of the pointer. The report carries the variables, and
  the pixel of the check is still `#12101b`.
- The loop over the files works on the box: `Loop Files "*.tsv", "R"`
  finds the three tables of the reference under `~/ahk`, and the
  variables of v2 hold the name, the extension and the folder of the
  file.

Verified on 2026-10-28, on the same box, with the sources of the tree
copied over it and the release rebuilt from them:

- A recursion stops and says so: `F(k) { return F(k + 1) }` answers
  "Function recursion limit exceeded." on line 2, the report comes out,
  and the runner exits 1. Before this the runner died with a stack
  overflow and no report at all.
- The dialect says what the binary of v2.0.19 says: `-2 ** 2` is -4,
  `"x" 1 + 2` is "x3" and `1 + 2 "d"` is "3d" -- the same three
  answers the binary gives, measured on the same day.
- The size of the screen is the geometry of the box: `A_ScreenWidth` is
  1920 and `A_ScreenHeight` is 1080.
- The box writes the files a script asks for: `FileAppend("hello",
  "ahk_probe.txt")` leaves the file in the working directory of the run,
  and an absolute path writes where it says.
- The loop kinds take the comma as well: `Loop Files, "*.txt"` and
  `Loop Files "*.txt"` both run.

Not verified:

- The compositor on the box is COSMIC; nothing here was measured on GNOME
  or on X11.
- Whether the pointer needs five seconds to be adopted, or four: the
  archive says five in one constant and four in its own rule, and the box
  was measured at four.

## 14. Where the archive and this tree differ

The rule was that they agree, and the port to v2.0.19 is what broke it.
These are the places where they differ:

- **The dialect.** The names here are the names of v2.0.19: `MsgBox` and
  not `Msg`, `StrUpper` and not `Upper`, `PixelGetColor` and not
  `PixelGet`. The archive keeps the old ones, and it is frozen there.
- **The interpolation.** The archive writes `%name%` inside a string and
  the value of the name goes there. v2.0.19 does not: measured on the
  binary on 2026-10-27, `"var:%x%"` is the text `var:%x%`. The tree keeps
  the interpolation of the archive, and it happens where the literal is
  written, so a string a function returns still knows its own parameter.
  This is the one place where the tree is deliberately not v2.
- **`Screenshot`** is a name of the dialect that v2 does not have. It is
  in the built-in table and it is listed in `tests/surface.rs` as the one
  name the dialect invents.
- **The classes.** The archive has no class types; the eleven tables of
  the binary are here now.
- **The tree has no `for ... in` and no `[]`.** v2 has both: `for x in y`
  walks the values of `y`, and `[a, b]` is an array of two. The tree has
  the `Array` class -- `Array(1, 2, 3)` is an array of three -- and the
  loops of v2 (`Loop Files`, `Loop Reg`, `Loop Read` and `Loop Parse`), and
  the two forms are not in it.
- **`yield` is not in the tree.** v2 has it: a function that yields gives a
  generator, and a class that enumerates one is a generator of its own.
  The tree has no `yield`, so a class cannot write an `__Enum` that yields.
  The word is not a name of v2 either -- it is a word of the language -- so
  the tables of the surface do not carry it.

And these are the places where the two still differ in the box, each one
a bug in the port until it is measured on the box:

- **The pointer is optional in the archive and mandatory here.** The
  archive catches a pointer that cannot be created, notes it, and falls
  back to `ydotool mousemove --absolute`. The Rust propagates the error
  and the click fails. There is no `ydotool` fallback anywhere in this
  tree.
- **`Run` has no timeout here.** The archive runs the command with
  `timeout=120`; the Rust waits for it. The script's own timeout is
  checked between statements, so a `Run` that hangs past it produces no
  report at all.
- **`ensure_session_env` overwrites `XDG_RUNTIME_DIR` here.** The archive
  only sets it when it is not set already.
- **The archive's pointer can be closed, and this one cannot.** The
  archive raises an error when a closed pointer is asked to move; the Rust
  `Pointer` holds the file of the device, so there is no such state to be
  in.
- **The pointer's enumeration wait**, in section 4.

## 15. The loop over the files

Measured on the binary of v2.0.19 on 2026-10-27, with probes that were run
against the real thing.

- **`Loop Files` takes the pattern first and the mode second.** The folder
  is the one the script runs in, and it is not a parameter: `Loop Files
  "*.txt"` walks the working directory. The tree had the shape of v1,
  with the folder first, and a v2 script failed on it.
- **The mode is a set of flags.** "R" walks into the subfolders as well,
  and "D" gives the directories instead of the files. The letters are
  read without regard to the case, and a third parameter is refused:
  "\"Loop Files\" accepts at most 2 parameters."
- **The loop writes the variables of v2**: `A_LoopFileName`,
  `A_LoopFileFullPath`, `A_LoopFileExt`, `A_LoopFileDir` and
  `A_LoopFilePath`, beside `A_Index`. The tree wrote a name of its own,
  `A_LoopFile`, which v2 does not have.

The other loop kinds were measured with the archive: `Loop Parse` takes
the text and the delimiter, `Loop Read` takes the file, and `Loop Reg` is
the registry of Windows, which this box does not have -- the tree says so
and the script fails with that reason.

## 16. The reference operator, and the built-ins that write back

Measured on the binary of v2.0.19 on 2026-10-27, with probes that were run
against the real thing and read off its dialogs.

- **The marker is in the definition.** `Swap(&Left, &Right)` is a parameter
  written `&`, and the caller passes `Swap(&a, &b)`. The parameter is then
  an alias of the variable of the caller: a write inside the function is a
  write outside it. A method may have such a parameter too.
- **A reference is a value of its own kind.** `y := &x` is a valid v2 script,
  and using the reference as a value fails with "Expected a String but got a
  VarRef." A parameter that is not written `&` holds the reference and does
  not write through it.
- **`&x` is only for a variable.** A property, an item and a variable of a
  class are refused with "\"&\" requires a variable." A built-in variable, a
  function and a class are refused each with its own reason.
- **The built-ins that write back take the variable by reference.**
  `SplitPath` writes the five pieces of a path, `MouseGetPos` writes the
  place of the pointer, `StrReplace` and `RegExReplace` write the number of
  the swaps, `IsSetRef` asks whether a variable is set, and
  `VarSetStrCapacity` sets the capacity of a string. A value where a
  reference is wanted fails with the place of the parameter and the value
  that was there: "Parameter #5 of StrReplace requires a variable reference,
  but received an Integer. Specifically: 5".
- **`SplitPath` cuts the drive off the path before it cuts the name.**
  Measured on the binary of v2.0.19: the path "C:/Users/danalec/file.txt"
  gives the name "/Users/danalec/file.txt" and the directory "C:". The
  slash is not a separator of a file, and the drive is not part of the
  name.
- **The flag of the table of v2 is the place of the first parameter that is
  written back.** `CaretGetPos`, `IsSetRef` and `VarSetStrCapacity` have it
  at the first, `RegExMatch` at the third, `RegExReplace` and `RunWait` at
  the fourth, `SplitPath` at the second and `StrReplace` at the fifth.
  `MouseGetPos` is a command and carries no flag, and the four places it
  writes are its first four parameters.
- **One thing is not carried.** v2 writes into the variable of `RegExMatch`
  an object of the class `RegExMatchInfo` -- the class is in the string pool
  of the binary, beside `Map` and `Closure` -- and the tree does not carry
  that class, so the call fails with the reason. The types of the
  parameters are in a table of their own now, and section 18 is about it.

The shape of the built-ins is the shape of v2 and not of the archive:
`StrReplace` takes the case sense as its fourth parameter and writes the
number of the swaps into the fifth, and `RegExReplace` writes the number of
the swaps into its fourth and takes the limit and the place where the
search starts after it. The tree had them the way v1 had them, and a v2
script that used them was answered with something else.

## 17. How v2 is measured

The binary of v2.0.19 is at `E:\autohotkey\AutoHotkey64.exe`, and it is the
reference: every measurement in these notes came out of it.

- **A probe writes its answer to a file.** The binary forks, so the
  answer has to survive the process: the script writes to a file and the
  driver reads the file after the run.
- **A runtime error opens a dialog, and the dialog holds only the
  buttons.** The window tree of the dialog carries no text of the
  message: the message is in the picture, and it has to be read off a
  screenshot. A run that times out is a dialog and not a missing name.
- **A load-time error opens a dialog as well, and so does a load-time
  warning.** `/ErrorStdOut` catches some of the errors and not the
  others.
- **A try/catch is the way to catch a message.** The catch is written
  `catch as err` and the message is `err.What`. For the parameter error
  of a built-in it comes back empty, and the text has to be read off the
  dialog.

## 18. The type errors

Measured on the binary of v2.0.19 on 2026-10-28, with probes that were run
against the real thing and read off its dialogs.

- **There are two templates, and they are not the same.** An operation of
  the language says "Expected a Number but got a String.", and a built-in
  says "Parameter #1 of Abs requires a Number, but received a String.". The
  second one names the built-in and the place of the parameter too.
- **A built-in that casts an object uses the first template.**
  `ObjOwnProps` says "Expected an Object but got a String." and not the
  second one.
- **An empty text is an empty string.** `Abs("")` says "received an empty
  string." and not "received a String."
- **The type a parameter wants is a number, a text or an object.** The
  table in `builtins.rs` holds the built-ins whose types were measured, and
  the guard on a reference asks it: a reference where a number is wanted
  says "Parameter #1 of Abs requires a Number, but received a VarRef." A
  built-in that is not in the table takes a value of any kind, and its
  guard says "requires a value".
- **A number that is not there is not a zero.** `Abs("abc")` used to
  answer 0, and it fails now with the message of v2. The same goes for
  `Sqrt`, `Floor`, `Ceil`, `Round`, `Min`, `Max` and the trigonometry.
- **Two of the built-ins say something of their own.** `Mod` says
  "Invalid parameter(s)." when one of its arguments is not a number, and
  `Sqrt` says "Parameter #1 of Sqrt is invalid." when the number is
  negative. `Ln` and `Log` say the same of a negative number, and so do
  `ASin` and `ACos` of a number outside the unit range.
- **A text that holds a number is a number, and it keeps its kind.**
  `Abs("3")` answers the integer 3 and `Abs("3.5")` the float 3.5.

## 19. The object model

Measured on the binary of v2.0.19 on 2026-10-28, with probes that were run
against the real thing.

- **A class has a prototype, and the base of an instance is it.**
  `Alpha.Prototype` is an object of the class `Prototype`, and
  `ObjGetBase(instance)` answers it.
- **A class is one object for the whole run.** The runtime keeps the class
  values it made, so that two names of a class are the same object and the
  `is` operator can tell a class from an instance of one.
- **The `is` operator asks whether an object is of a class.** `b is Beta`
  is 1 when `b` is an instance of `Beta` or of a class that extends it,
  and 0 otherwise. Every object is an `Object`: it is the root of the
  chain. A number and a text are not objects.
- **The `is` operator wants a class.** A text on its right fails with
  "Expected a Class but got a String."
- **There is no `base` variable in v2.** This is the one that surprised the
  most: `base` inside a method of v2.0.19 is a local variable that was
  never assigned, and the runtime says "This local variable has not been
  assigned a value." A script that has a class named `Base` reads the
  class, because the names of v2 do not differ in their case. The tree does
  not carry a `base` variable, and it must not grow one.
- **The base of an object is in the property table, under a reserved
  key.** `ObjSetBase` puts it there, and `ObjOwnProps` does not name it:
  the state of an object is not a property of it.
- **The type of an object is the name of its class.** `Type(box)` is
  "Box" and `Type(Map())` is "Map", and a class itself is of the type
  "Class".
- **The `Obj` family reaches the object model.** `ObjGetBase`,
  `ObjSetBase`, `ObjOwnProps`, `ObjOwnPropCount` and `ObjHasOwnProp` are
  the tree's own. `ObjAddRef` counts the references of a COM object, and
  an object of a script has no such count: the built-in says "Parameter #1
  of ObjAddRef is invalid." The names of COM -- `ObjFromPtr`,
  `ObjFromPtrAddRef`, `ObjPtr`, `ObjPtrAddRef` and `ObjRelease` -- are
  names of Windows, and the tree says so.

## 20. The precedence of the operators

The table in `ahk::precedence` is the one of the v2 documentation, and it
was measured against the binary of v2.0.19 on 2026-10-28.

- **The power binds tighter than the sign.** `-2 ** 2` is -4 and not 4:
  the unary minus takes the whole power, the way `-(2 ** 2)` does. So the
  operand of a unary operator is parsed with the power in it, and the
  increment and the decrement -- which bind tighter than the power --
  take only the value.
- **The rest of the table is the one of the documentation.** Measured on
  the same day: `2 ** 3 ** 2` is 512 (the power is right-associative),
  `1 + 2 . 3` is "33" (the addition binds tighter than the
  concatenation), `2 * -3 ** 2` is -18, and `!1 == 1` is 0 (the `!`
  binds tighter than the comparison, while the word `not` binds looser).
- **Two values side by side are one value.** That is the implicit
  concatenation of v2, and the dot is not the only way to write it.
  Measured on the binary on 2026-10-28: `"a" 1 + 2` is "a3" and
  `1 + 2 "d"` is "3d", so the concatenation binds like the dot, and the dot
  is looser than the arithmetic.

## 21. The screen, the files and the machine

- **The size of the screen is the desktop's.** `A_ScreenWidth` and
  `A_ScreenHeight` ask the desktop, and the box answers with the geometry
  it measures from the compositor. A desktop that cannot tell says so:
  "this desktop cannot tell how big the screen is".
- **The box writes the files a script asks for.** `FileAppend` and the
  `File` class write through the desktop, and the path is the one the
  script wrote. Measured on the box on 2026-10-28: `FileAppend("hello",
  "ahk_probe.txt")` writes the file in the working directory of the run,
  and an absolute path writes where it says.
- **The machine is not carried.** `A_ComputerName`, `A_UserName`,
  `A_OsVersion` and the rest are names of v2, and the tree carries the
  names. The values are not there: the runtime keeps no machine, and it
  says so.

## Where to read more

- `README.md` -- the language: the statements, the keys, the functions,
  the shape of the report.
- `../archived/ahk-python/README.md` and its `AGENTS.md` -- the archive:
  the shape of the port, the limits, and the rule that nothing new lands
  in the archive.
- `crates/ahk/reference/` -- the tables of v2.0.19: the surface, the
  operators and the pending list.
- This file -- the box: the devices, the timings, the traps, and what was
  measured and when.
