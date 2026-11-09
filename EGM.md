# ahk Blueprint

## System Overview

The AutoHotkey language, in Rust, in three crates: the language itself, the
primitives of the lab box it drives, and the runner that reads a script on
stdin and writes a report on stdout.

The language is portable and does no I/O of its own: the script is parsed in
the `ahk` crate and executed against a desktop that is handed to it. The real
desktop lives in `ahk-box`, on the lab box; the tests use a fake that records
the calls. The runner is the only thing that lives in a process of its own,
and it is the way a script is stopped from outside.

**The dialect is AutoHotkey v2.0.19.** The names, the classes, the directives
and the operators are the ones the binary of v2.0.19 has, and the tables in
`crates/ahk/reference/` are read out of it.

The Python implementation this was ported from is archived at
`../archived/ahk-python`. It is kept to be read, not to be run, and it is the
record of what the dialect meant before the port to v2. **The two trees do not
speak the same dialect any more, and that is the point of the port.**

The working notes of the box -- the devices, the timings, the traps, and
every measurement with the date it was taken -- are in `AGENTS.md`.

## Subdirectories & System Boundaries

- [crates](./crates/EGM.md) - The three crates of the workspace, and the
  direction the dependency runs between them.

## Global Invariants

- The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The language
  crate must not reach for the box.
- The language crate does no I/O of its own, and it has to keep building
  without `std`. The host then supplies one thing: the clock, through the
  `Clock` trait.
- The archived Python is frozen and nothing new lands there. It is the record
  of the dialect before the port to v2.0.19, and the dialect here is the one of
  the binary of v2.0.19: the names, the classes, the directives and the
  operators are read out of it.
- A name is carried when the tree answers it. The answer may be a value or the
  reason the tree cannot give one, so a name that is shaped for Windows is
  carried the same way.
- A script that runs away is stopped. The bounds are 20 000 statements, 1 000
  turns through a `While` body, a `Sleep` of at most 60 000 ms, and the
  timeout of the runner, which is 60 seconds by default.
- A question the desktop cannot answer fails with the reason. It does not
  answer `false`.
- The devices on the box have to declare what they are: the keyboard
  announces the same 112 codes as the physical one, with the scan codes; the
  pointer announces `INPUT_PROP_DIRECT` and absolute axes. The keyboard and
  the pointer are two devices, because libinput picks one role per device.
- A script that types forever has to be stoppable from outside: the runner
  writes its pid to a file while the script runs, and the stop reads that
  file and kills the process.
- The notes in `AGENTS.md` change in the same commit as the behaviour they
  describe, and a measurement without its date is not evidence.

## Global Side Effects

- On the box, a script drives the desktop for real: it types, it clicks, it
  presses keys. The emergency stop in `AGENTS.md` is the way out.
- The box crate opens `/dev/uinput` and creates two devices, a keyboard and
  an absolute pointer. The compositor adopts each of them, and that takes
  four seconds.
- The box crate reads the screen: it captures the framebuffer and reads the
  colour of a pixel out of it.
- The runner writes a pid file while the script runs and removes it when the
  script ends, and it reads the script from stdin.
- The runner prints one JSON report on stdout. The exit code says whether
  the script finished, failed or did not parse.

## System Verification

- Run: `cargo test --workspace`
- Without the standard library:
  `cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`
