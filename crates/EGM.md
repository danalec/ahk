# crates

## Overview

The three crates of the workspace. The dependency runs one way: `ahk-cli` ->
`ahk-box` -> `ahk`, and `ahk` depends on nothing in the workspace. The
language crate must not reach for the box: it does no I/O, and it has to keep
building without `std`.

Each crate is a package of its own: it carries a manifest, and the sources
live in a `src` directory of their own, with the cases of the crate beside
them. The `tests` directory of a crate is test-only and carries no contract.

## Subdirectories & Boundaries

- [ahk](./ahk/EGM.md) - The language: a lexer, an AST, a parser and an
  interpreter for AutoHotkey-style scripts. Portable, and it does no I/O of
  its own.
- [ahk-box](./ahk-box/EGM.md) - The box primitives: the uinput keyboard of
  our own, the absolute pointer, the captures and the screen reads, and the
  windows of the Wayland compositor. The parts that talk to the kernel and
  the parts that talk to the compositor are Linux-only.
- [ahk-cli](./ahk-cli/EGM.md) - The runner: a script on stdin, a report on
  stdout. The binary is `ahk`.

## Interface

What the three crates between them expose, crate by crate. The details are
in the contract of each crate; this is the index.

| Crate | Public surface |
| --- | --- |
| `ahk` | re-exported at the root: `parse`, `Parser`, `precedence`, `tokenize`, `Token`, `TokenKind`, `SyntaxError`, `Node`, `LiteralValue`, `UnaryOp`, `BinaryOp`, `AssignOp`, `Parameter`, `Runtime`, `Report`, `Step`, `RuntimeError`, `Timeout`, `Value`, the `Desktop` and `Clock` traits, `WindowAnswer`, `SystemClock` (behind the `std` feature), and the key table: `KeyStroke`, `UnknownKey`, `parse_spec`, `resolve_name`. Public in the modules and not re-exported: `ahk::runtime::op_name`, `ahk::runtime::Scope`, `ahk::runtime::Reference`, `ahk::runtime::TypeName`, `ahk::ast::Branch`, `ahk::ast::Case`, `ahk::builtins::names`, `ahk::builtins::find`, `ahk::builtins::reference_parameters`, `ahk::builtins::wants`, `ahk::builtins::Wants`, `ahk::classes::names`, `ahk::classes::carries`, `ahk::variables::carries`, `ahk::builtins::refusal_of`, `ahk::builtins::the_extra_of`, `ahk::variables::refusal_of`, `ahk::surface::entries`, `ahk::surface::summary`, and the thirteen capability traits of `ahk::desktop` |
| `ahk-box` | `LabDesktop`, `BoxError`, `Keyboard`, `KeyboardError`, `Pointer`, `PointerError`, `ENUMERATION_WAIT`, `KEY_GAP`, `KEY_HOLD`, `PRESS_SETTLE`, `PRESS_HOLD` |
| `ahk-cli` | the binary `ahk`: the script on stdin, the report on stdout, the `--timeout=N` argument, the `--surface` argument, and the exit code of the process |

## Invariants

- The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The language
  crate must not reach for the box, and the box does not reach back into the
  internals of the language.
- The language crate does no I/O of its own, and it has to keep building
  without `std`. The host then supplies one thing: the clock, through the
  `Clock` trait.
- A script that runs away is stopped. The bounds are 20 000 statements, 1 000
  turns through a `While` body, a `Sleep` of at most 60 000 ms, and the
  timeout of the runner.
- A question the desktop cannot answer fails with the reason. It does not
  answer `false`.
- The archived Python at `../archived/ahk-python` is frozen and it is the
  record of the dialect before the port to v2.0.19. The dialect here is the one
  of the binary of v2.0.19, and the two trees do not speak the same dialect:
  the rename is the point of the port.

## Side Effects

- The language crate has none of its own. It opens no file, starts no
  process and writes nothing to disk; everything it does goes through the
  `Desktop` it was handed.
- The box crate touches the machine: it opens `/dev/uinput` and creates the
  keyboard and the absolute pointer, it reads the screen, it connects to the
  Wayland compositor to ask about its windows, and it runs the commands of
  the script.
- The runner writes a pid file while the script runs, removes it when the
  script ends, and prints one JSON report on stdout.

## Verification

- Run: `cargo test --workspace`
- Without the standard library:
  `cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`
