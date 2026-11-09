# The documents of this tree

The guides, the plans and the reports behind the code. `AGENTS.md` at the root
says which one to read before touching what; this page is the index of the
folder.

## The reference of each dialect

- [ahk-v2-reference.md](ahk-v2-reference.md) — every name of AutoHotkey
  v2.0.19, by commands, functions, built-in variables, class types and the
  methods of the class types, with the state this tree answers for each.
- [ahk-v1-reference.md](ahk-v1-reference.md) — the same catalogue for
  v1.1.37.02, by commands, functions, built-in variables and directives.

## What the tree carries, and what it does not

- [v2-capability-report.md](v2-capability-report.md) — how much of each area
  of the v2 is carried: the parser and the run, the files, the input, the
  windows, the screen, the native calls, the registry and the COM objects.
  It is the summary; the references above are the list of names.
- [ahk-gaps-report.md](ahk-gaps-report.md) — the gaps that remain, with the
  reason each one is a gap.

## Plans and comparisons

- [ahk-v1-dialect.md](ahk-v1-dialect.md) — the plan for the v1 dialect
  beside the v2: the gate (`--dialect=v1`), the phases, and what was
  measured on the binary of v1.1.37.02. Read it before touching the parser
  or the built-in table.
- [ahk-x11-comparison.md](ahk-x11-comparison.md) — the AHK_X11 (the AHK
  v1.0.24 for X11) against this tree, and what to port from it. Its code is
  GPL-2.0 and this tree is Apache 2.0: the behaviour may be ported, the code
  may not.

## How the box is driven

- [wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md) — how a
  Wayland compositor lists its windows, and the Rust that does it. Read it
  before touching `WinExist` or `WinActive`.
- [runner-and-report.md](runner-and-report.md) — the runner and the shape of
  the report: the command, the flags, standard input and output, `exit_code`,
  `variables`, a step, an error, the notes, the exit codes and the pid file.
  Read it before changing what the runner prints.