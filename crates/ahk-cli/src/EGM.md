# ahk-cli::src

## Overview

The runner itself: one file, `main.rs`, and one binary, `ahk`. The binary is
what runs on the lab box -- the MCP of the lab server hands it a script over
stdin, in memory and never as a file, and reads the report back.

```sh
echo 'MsgBox "hello"' | ahk
```

The file has four jobs and no more: it reads the script, it parses it, it runs
the statements against the lab box, and it prints the report. The language is
not here -- `ahk::parse` and `ahk::runtime::Runtime` do the parsing and the
running -- and the desktop is `ahk_box::LabDesktop`.

## Interface

- The binary `ahk`, declared by the `[[bin]]` of the manifest of the package.
- `fn main() -> std::process::ExitCode` -- the whole run, from the script on
  stdin to the report on stdout.
- `fn pid_file() -> PathBuf` -- where the pid file lives:
  `$XDG_CACHE_HOME/ahk/run.pid`, then `$HOME/.cache/ahk/run.pid` (or
  `$USERPROFILE/.cache/ahk/run.pid`), then the temporary directory.
- `fn write_pid_file()` -- writes the pid of the process with a newline, and
  creates the directory when it is not there.
- `fn clear_pid_file()` -- removes the file.
- `struct PidGuard` -- a guard with a `Drop` implementation; it removes the pid
  file when the run ends, however it ends.
- The command line: `--timeout=N`, the timeout of the run in seconds, 60 by
  default. An argument that is not a number is ignored and the default stands.
- The input: the whole script is read from stdin with `read_to_string`.
- The output: the report, as JSON, on stdout.

The report is the `ahk::runtime::Report` of the language crate, serialized
with `serde_json`:

| Field | What it holds |
| --- | --- |
| `ok` | whether the script finished |
| `steps` | the statements that completed |
| `output` | what `MsgBox` printed |
| `notes` | the places where the box was not what the script assumed |
| `error` | the reason the run failed, when it failed |
| `error_line` | the line of the statement that failed |
| `exit_code` | what `ExitApp` set |
| `elapsed_ms` | how long the run took |
| `variables` | the variables of the script at the end |

## Invariants

- The exit code of the process is 0 when the script finished, 1 when it
  failed, and 2 when it did not parse.
- The script runs on a thread with a stack of its own: the frames of the
  tree are heavier than the ones of v2, and the recursion rail of the
  runtime has to fit in the stack of the run. The report is written down
  on that thread, because a value of the runtime is not `Send`.
- A script that cannot be read from stdin gets the reason on stderr
  (`cannot read the script: ...`) and the exit code 2, and no report at all.
- The code `ExitApp` sets is the `exit_code` of the report and not the exit
  code of the process: a script that calls `ExitApp 3` exits 0.
- The pid file is written only after the script parses: a script that does
  not parse leaves no file at all.
- The pid file is removed when the run ends, however it ends. The exit code
  goes out through the return of `main`, and not through `process::exit`,
  because the second leaves without running the destructors -- and the guard
  that removes the pid file is a destructor.
- The pid file never lands in the working directory: it goes to
  `$XDG_CACHE_HOME`, then to the cache directory of the home directory, then
  to the temporary directory.
- The write of the pid file is not checked: a run whose pid file could not be
  written says nothing about it.
- A parse error is the reduced payload -- `ok` is false, and it carries
  `error`, `error_line`, `column`, `steps`, `output` and `notes`, with no
  `elapsed_ms`, no `exit_code` and no `variables`.
- The notes of the runtime come first, and the notes of the box are added to
  them: a host that wrote a note of its own keeps it.
- The timeout is checked between two statements, so a `Sleep` can overshoot
  it; the message is "the script ran for more than Ns" and `error_line` is
  null.

## Side Effects

- Writes the pid file and removes it when the run ends.
- Prints the report on stdout.
- Writes the reason on stderr when the script cannot be read from stdin.
- Drives the real desktop through `ahk-box`: it types, it clicks and it
  presses keys for real. The uinput keyboard and the absolute pointer are
  opened on the first use, and each one waits for the compositor to adopt it.
- `ensure_session_env` sets `XDG_RUNTIME_DIR` and `WAYLAND_DISPLAY` in the
  environment of the process when `WAYLAND_DISPLAY` is missing.

## Verification

- Run: `cargo test --workspace`
