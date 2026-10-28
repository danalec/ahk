# ahk-cli

## Overview

`ahk-cli` is the runner of the workspace: the binary that runs on the lab
box. It takes an AHK script on stdin and writes the report on stdout, so the
MCP of the lab server drives the desktop without a script file ever touching
the disk.

The package holds no source of its own -- the code is in [`src`](./src/EGM.md)
-- and the manifest declares one binary:

| Manifest | Value |
| --- | --- |
| `[[bin]] name` | `ahk` |
| `[[bin]] path` | `src/main.rs` |
| dependencies | `ahk`, `ahk-box`, `serde`, `serde_json` |

The dependency runs one way: the runner pulls in the language and in the box
primitives, and nothing in the workspace points back at it.

## Subdirectories & Boundaries

- `src`: [The runner](./src/EGM.md) - the whole package. `main.rs` reads the
  script from stdin, runs it against the lab box, and prints the report.

The cases of the runner live in `tests/`; that directory holds test code only
and carries no contract of its own.

## Interface

- The binary `ahk` -- the script goes in on stdin and the report comes out on
  stdout.
- `--timeout=N` -- the timeout of the run, in seconds, 60 by default. The
  argument is read from the command line of the process and handed to the
  runtime, which checks it between two statements.

## Invariants

- The exit code of the process is 0 when the script finished, 1 when it
  failed, and 2 when it did not parse. A script that cannot be read from
  stdin exits 2 with the reason on stderr and no report at all.
- `ExitApp` sets the `exit_code` of the report, not the exit code of the
  process: the runner still exits 0.
- The report is JSON on stdout, with the fields `ok`, `steps`, `output`,
  `notes`, `error`, `error_line`, `exit_code`, `elapsed_ms` and `variables`.
  A parse error is the reduced payload: `ok` is false, and it carries `error`,
  `error_line`, `column`, `steps`, `output` and `notes`, with no `elapsed_ms`,
  no `exit_code` and no `variables`.
- The notes of the runtime come first, and the notes of the box are added to
  them, so a host that wrote a note of its own keeps it.
- The pid file is written only after the script parses, and it is removed when
  the run ends. It lives in `$XDG_CACHE_HOME/ahk/run.pid`, then in
  `$HOME/.cache/ahk/run.pid`, then in the temporary directory -- never in the
  working directory.

## Side Effects

- Writes the pid file and removes it when the run ends.
- Prints the report on stdout, and the reason on stderr when the script cannot
  be read.
- Drives the real desktop through `ahk-box`: it types, it clicks and it presses
  keys for real.

## Verification

- Run: `cargo test --workspace`
