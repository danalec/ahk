# ahk

The language crate of the workspace: the lexer, the AST, the parser and the
interpreter for AutoHotkey-style scripts. It holds the dialect and nothing
else -- no devices, no processes, no report writing. What drives the desktop
is handed to it, which is why the crate builds and runs on a target that has
no standard library at all.

## Overview

`ahk` is the crate the other two depend on, and it depends on nothing in the
workspace. The arrow runs one way, from the outside in: the runner
(`ahk-cli`) drives the box primitives (`ahk-box`), and `ahk-box` drives the
language.

**The dialect here is AutoHotkey v2.0.19.** The names, the classes, the
directives and the operators are the ones the binary of v2.0.19 has, and the
tables in `crates/ahk/reference/` are read out of it.

The Python implementation in the archive is the record of what the dialect
meant before the port to v2. The two do not speak the same dialect any more,
and that is the point of the port.

Two things live in this directory:

| Path | What it holds |
| --- | --- |
| `Cargo.toml` | the crate manifest: the `std` feature and the three dependencies |
| `src/` | the source: the public surface and the modules behind it |

The suite of the dialect lives in `tests/`, which is test-only and carries no
contract of its own.

## Subdirectories & Boundaries

- `src`: [the language crate](./src/EGM.md) -- the public surface (`lib.rs`)
  and the modules behind it: the lexer, the AST, the parser, the runtime and
  the key table.

## Interface

The crate's public surface is `lib.rs`, and it is small on purpose: the
modules are public, and the symbols below are re-exported from the root.

| Symbol | Module | What it is |
| --- | --- | --- |
| `parse` | `parser` | a source string into the statements of a script |
| `Parser` | `parser` | the parser itself, for a caller that wants to drive it |
| `tokenize` | `lexer` | a source string into tokens |
| `Token`, `TokenKind` | `lexer` | what the lexer produces |
| `SyntaxError` | `lexer` | the error the lexer and the parser raise |
| `Node`, `LiteralValue` | `ast` | the tree the parser builds |
| `Parameter` | `ast` | one parameter of a function or of a method |
| `UnaryOp`, `BinaryOp`, `AssignOp` | `ast` | the operators that hang on the tree |
| `Runtime` | `runtime` | the interpreter |
| `Report`, `Step` | `runtime` | what a run reports |
| `RuntimeError`, `Timeout` | `runtime` | the two ways a run fails |
| `precedence` | `parser` | the precedence of the operators, the way v2 has it |
| `Value` | `runtime` | a value a script can hold |
| `Desktop` | `desktop` | the machine the runtime drives: the sum of the fourteen capability traits |
| `WindowAnswer` | `desktop` | the answer about a window: it is active, it is there, it is not there, or the desktop cannot tell |
| `Clock` | `runtime` | the clock the runtime measures itself with |
| `SystemClock` | `runtime` | the clock of a hosted platform, behind the `std` feature |
| `KeyStroke`, `UnknownKey` | `keys` | a key press, and the key the table does not have |
| `parse_spec`, `resolve_name` | `keys` | the key table and the key names a script may use |
| `builtins::names`, `builtins::find` | `builtins` | the built-in table: every name of v2 the dialect carries |
| `builtins::refusal_of` | `builtins` | the reason the tree does not answer a name, when it does not |
| `classes::names`, `classes::carries` | `classes` | the class types: the names each one answers |
| `variables::carries` | `variables` | the built-in variables: whether the tree carries one |
| `variables::refusal_of` | `variables` | the reason the tree does not answer a built-in variable |
| `surface::entries`, `surface::summary` | `surface` | the state of every name of v2: the value it answers, or the reason it does not |

Reachable, but not re-exported at the crate root: `ahk::runtime::WindowAnswer`,
the answer about a window, `ahk::runtime::op_name`, the name of a
statement for the report, and `ahk::runtime::{Scope, Reference, TypeName}`,
the scope of a variable, the reference to it and the kind of a value. The
`ast` module keeps `Branch` and `Case` the same way, in `ahk::ast`.

The manifest is part of the interface too, because the feature is a promise:

| Feature | Default | What it turns on |
| --- | --- | --- |
| `std` | yes | `Runtime::new` and `SystemClock`; the crate becomes a `std` crate |

Without `std` the crate is `#![no_std]` and the host supplies the clock
through `Clock`. The dependencies are `serde` for the report and `thiserror`
for the errors, both without their default features, and `libm` for the float
functions that `std` owns.

## Invariants

- **The crate does no I/O of its own.** The parser and the runtime read and
  write nothing. The desktop is an interface the runtime is handed, and that
  is what makes the language testable against a fake.
- **The dependency runs one way.** `ahk-cli` -> `ahk-box` -> `ahk`, and `ahk`
  depends on nothing in the workspace. The language crate must not reach for
  the box.
- **The crate keeps building without `std`.** `cargo build -p ahk
  --no-default-features --target thumbv7em-none-eabihf` is part of the loop.
  Only `abs`, `min`, `max` and `clamp` may be called on an `f64` directly;
  `sqrt`, `floor`, `ceil`, `round` and `fract` go through the helper in
  `runtime.rs`, which is `libm` under `no_std`.
- **The guard rails are numbers, and they are checked.** `Runtime::MAX_STEPS`
  is 20 000 statements in one script and `Runtime::MAX_ITERATIONS` is 1 000
  turns through a `While` body; `Sleep` is capped at 60 000 ms, in the
  statement and in the builtin. A limit that trips is reported as an `error`
  with the line of the statement that tripped it.
- **A question that cannot be answered fails.** `WinActive` and `WinExist`
  answer *unknown* rather than `false`, and the script fails with the reason.
  A window that is not there is not a failure: the desktop says that it is
  not there, and the answer is `false`. `WinGetTitle` is the exception: with
  no active window it answers an empty string, because there is no window to
  ask about.
- **The names are the names of v2.0.19, and only those.** `StrUpper` and not
  `upper`, `PixelGetColor` and not `PixelGet`, `MsgBox` and not `Msg`. There
  are no aliases, and the built-in table of the crate is the list of what the
  dialect carries. The one name outside the v2 namespace is `Screenshot`:
  the dialect writes a picture of the screen to a file, and v2 has no word
  for it.
- **The parity with v2 is checked, not asserted.** `tests/surface.rs` asks the
  tree, name by name, whether it carries every name of the surface table, and
  `tests/class_surface.rs` asks every name of every class, as a call and as a
  read, and the answer may not be "this value has no such method".
- **The report is the whole truth.** Every statement that completes produces a
  step, and a statement that fails is reported once in `error` and
  `error_line` -- it is not a step, and `ok` is true on every step that
  exists.
- **A reference is a value of its own kind, and the marker of a by-reference
  parameter is in the definition.** `Name(a, &b)` is an alias of the variable
  of the caller, and `Name(&x)` passes the reference. A reference is a
  `VarRef`: it is neither a string nor a number, and the operations that
  want one of those say that they got one.

## Side Effects

- **None of its own.** The crate opens no file, starts no process, writes
  nothing to disk and touches no network.
- **The calls it makes into the injected `Desktop`.** The runtime types,
  clicks, presses keys, moves the pointer, reads pixels, captures the screen
  and runs commands -- all of them through the `Desktop` it was handed, and
  all of them an error when the desktop cannot do it.
- **The pause it asks of the injected `Clock`.** `Sleep` waits on the clock,
  and the runtime measures its own steps and its timeout on the same clock.
- **The state it keeps while it runs.** The variables, the scopes, the
  functions and the steps live in the `Runtime` and come out in the `Report`;
  nothing survives the run.

## Verification

- Run: `cargo test --workspace`
- Without the standard library: `cargo build -p ahk --no-default-features
  --target thumbv7em-none-eabihf`
