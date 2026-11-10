# The runner, and the report

`ahk` is the binary that runs a script. It takes the source on standard
input, runs it against the desktop of the machine, and writes the report
as JSON on standard output. It never reads a file of the script and never
writes one: the source goes in memory and is gone when the run is over.

This document is about the runner itself -- the command, the flags, the
report, the guard rails, the surface command, the exit codes, the pid
file and `#Include`. The language is `README.md` and the names are
`docs/v2-reference.md` and `docs/v1-reference.md`; the machine the script
drives is [the desktop backend](desktop-backend.md).

- [The command](#the-command)
- [The flags](#the-flags)
- [Standard input and standard output](#standard-input-and-standard-output)
- [The report](#the-report)
- [A step, and an error](#a-step-and-an-error)
- [The notes](#the-notes)
- [The exit codes](#the-exit-codes)
- [The surface command](#the-surface-command)
- [The includes](#the-includes)
- [The pid file and the emergency stop](#the-pid-file-and-the-emergency-stop)
- [The guard rails](#the-guard-rails)
- [The timeout](#the-timeout)

## The command

The runner reads its script from standard input.

```sh
echo 'MsgBox "hello from the box"' | ahk
```

On a Windows workstation the binary of a debug build is
`target\debug\ahk.exe`, and the same command is:

```pwsh
'MsgBox "hello from the box"' | target\debug\ahk.exe
```

The report of that run is one line of JSON:

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":["hello from the box"],"steps":[{"detail":"hello from the box","elapsed_ms":0,"line":1,"ok":true,"op":"MsgBox"}],"variables":{"a_linenumber":1},"warnings":[]}
```

The keys of the object come out in alphabetical order -- `elapsed_ms`,
`error`, `error_line`, `exit_code`, `notes`, `ok`, `output`, `steps`,
`variables`, `warnings` -- and that is the order of the examples here. Read
a report by its keys and not by their places.

## The flags

Every flag is read out of the arguments, and the order of them does not
matter. A flag that the runner does not know is ignored -- it is not a
failure -- except `--dialect=`, which the runner answers when the value
is not one it speaks.

| Flag | What it does |
| --- | --- |
| `--dialect=v2` | The dialect of the run: AutoHotkey v2.0.19. This is the default. |
| `--dialect=v1` | The legacy dialect, measured on the binary of v1.1.37.02. |
| `--timeout=N` | How many seconds the script may run, as a decimal number. 60 by default. |
| `--surface` | Do not run a script: print the table of the names of the dialect. |
| `--json` | With `--surface`, print the table as JSON instead of text. |

`--dialect=` is matched with the case taken away: `--dialect=V1` is the
v1. A value that is neither `v1` nor `v2` is written to standard error and
the runner exits 2, before a script is read:

```
the dialect v3 is not one this tree speaks: the tree speaks v1 and v2.
```

`--surface` is answered before the script is read, so a run that asks for
the surface does not wait for a script that is not coming. `--json` is
only read when `--surface` is there; on its own it does nothing.

## Standard input and standard output

Three things come out of the runner, and they go to three places.

| Stream | What goes there |
| --- | --- |
| Standard input | The source of the script, in UTF-8. |
| Standard output | The report, as one line of JSON. With `--surface`, the surface of the dialect instead. |
| Standard error | The reason of a run whose script cannot be read, and the reason of a dialect the tree does not speak. |

The report is JSON on standard output in every case but one: when the
source cannot be read from standard input at all, the runner writes
`cannot read the script: <reason>` on standard error and exits 2 without
a report. Everything else -- a script that does not parse, a script that
fails, a script that trips a guard rail -- has a report.

A parse error leaves a report that is not the report of a run. It has
`ok`, `error`, `error_line`, `column`, `steps`, `output` and `notes`, and
it has none of `elapsed_ms`, `exit_code`, `variables` or `warnings`,
because no runtime was made.

```sh
echo 'MsgBox "unterminated' | ahk
```

```json
{"column":8,"error":"Missing \"\"\"\n     Specifically: MsgBox \"unterminated","error_line":1,"notes":[],"ok":false,"output":[],"steps":[]}
```

An include that cannot be resolved leaves the same reduced report, with
`error_line` and `column` both zero: the failure happens before the
script has a line.

## The report

The report of a run is an object with these fields.

| Field | Type | What it is |
| --- | --- | --- |
| `ok` | boolean | Whether the script finished without a failure. |
| `steps` | array of step | What every completed statement did, in order. |
| `output` | array of string | What the script wrote to its output -- the text of every `MsgBox`, and the write of `FileAppend` to `*`. |
| `warnings` | array of warning | The warnings of the script: `{"line": N, "message": "..."}`. |
| `notes` | array of string | The places where the machine was not what the script assumed. |
| `error` | string or null | The message of the failure that stopped the script. |
| `error_line` | number or null | The line of that failure. |
| `exit_code` | number or null | The code that `ExitApp` asked for. |
| `elapsed_ms` | number | How long the run took, in milliseconds. |
| `variables` | object | The variables of the script at the end, by name, in the lower case. |

A report of a run that failed, with the step that ran before the failure:

```pwsh
@'
MsgBox "one"
c := PixelGetColor(10, 10)
MsgBox "two"
'@ | target\debug\ahk.exe
```

```json
{"elapsed_ms":62,"error":"grim failed: sh: line 0: grim: not found\n","error_line":2,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":["one"],"steps":[{"detail":"one","elapsed_ms":0,"line":1,"ok":true,"op":"MsgBox"}],"variables":{"a_linenumber":2},"warnings":[]}
```

The third statement never ran, so it has neither a step nor a place in
`output`: `output` is `["one"]` and not `["one", "two"]`.

### `exit_code` is not the exit code of the process

`exit_code` is the number the script asked `ExitApp` for. The runner
still exits 0 when the script finished, whatever the number was.

```pwsh
'ExitApp 3' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":3,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

The exit code of the process is 0. A client that wants the number the
script asked for reads `exit_code`, and one that wants to know whether
the run was good reads `ok`.

### `variables`

Every variable of the script that has a value at the end, with its name
in the lower case. The built-in variable `A_LineNumber` is there in
almost every run, because the runtime keeps it as its own name.

The value is written the way the dialect writes it: a number as a
number, a text as a string, and an object as its serialised shape. A
variable that holds an object of the script is not a flat name and value,
so the report of a script that makes objects is deeper than the report
of a script that does not.

## A step, and an error

Every statement that *completes* produces one step. A statement that
fails does not: it stops the script and is reported once, in `error` and
`error_line`.

That is the rule a client has to know: **the failure is not in `steps`.**
Every step that is in the array has `ok: true`, because a step that would
have had `ok: false` was never pushed. A client that looks for the
failure in the array will not find it.

A step is an object with five fields.

| Field | Type | What it is |
| --- | --- | --- |
| `line` | number | The line of the script the statement is on. |
| `op` | string | What the statement is: `MsgBox`, `Assign`, `FunctionDef`, `Sleep`, `DllCall`. |
| `detail` | string | What the statement did, in the words of the handler that ran it. |
| `ok` | boolean | True. Every step in the array is one that completed. |
| `elapsed_ms` | number | How long the statement took. |

The `detail` of a step is the answer of the handler, and it is a sentence
of the runtime and not a format: `MsgBox "hello"` has the detail
`hello`, an assignment has `x = 5`, and a `Sleep` of 3000 has `3000`.

The `op` of a statement is the kind of the statement and not the name of
the function it calls, so a line that defines a function has the `op`
`FunctionDef` and a line that calls one has the `op` of the call.

A failure that happens outside a statement -- a `Goto` to a label that is
not there, a hotkey whose registration the machine refused -- still has
its `error_line`. The registration of a hotkey that the machine cannot
watch is checked before the first statement runs, so `error_line` is 0
and there are no steps at all:

```pwsh
@'
Hotkey "#j", F
F() {
}
'@ | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":"the hotkeys of this box watch a Linux device; this is not Linux","error_line":0,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[{"detail":"F is defined","elapsed_ms":0,"line":2,"ok":true,"op":"FunctionDef"}],"variables":{"a_linenumber":1},"warnings":[]}
```

## The notes

The notes are the places where the machine was not what the script
assumed: the environment that was found, the device that had to be
waited for, the position that the kernel would not answer for. They are
not failures, and a run with notes can be `ok`.

The runtime writes its own notes as it runs, and the runner adds the
notes of the box at the end: the notes of the runtime come first, and
the notes of the box after them.

On a host that is not Linux the box has one note of its own, and it is
in every report:

```
no uid on this platform; the session environment is unknown
```

That note is the honest answer to "did the runner find the graphical
session of the user": it cannot look, because there is no user id to look
under. A run on the Linux box that had to find the Wayland socket says
which one it found instead:

```
WAYLAND_DISPLAY=wayland-1 (XDG_RUNTIME_DIR=/run/user/1000)
```

A note that the runtime writes is the place where the machine answered
something that is not what it should be. The pointer, for one:

```
the kernel did not say where the pointer is (<reason>); this is the position the runtime put it at
```

The report of a run that asks `MouseGetPos` on a host without `grim`
carries both notes:

```pwsh
'p := MouseGetPos()' | target\debug\ahk.exe
```

```json
{"elapsed_ms":120,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown","the kernel did not say where the pointer is (grim failed: sh: line 0: grim: not found\n); this is the position the runtime put it at"],"ok":true,"output":[],"steps":[{"detail":"p = ","elapsed_ms":120,"line":1,"ok":true,"op":"Assign"}],"variables":{"a_linenumber":1,"p":""},"warnings":[]}
```

## The exit codes

| Code | When |
| --- | --- |
| 0 | The script finished. `ok` is true in the report. |
| 1 | The script failed. `error` is in the report, and `ok` is false. |
| 2 | The script did not parse, an include could not be resolved, the source could not be read, or the dialect is not one the tree speaks. |

The exit code is 0 for a script that asked `ExitApp 3`: the number the
script asked for is in `exit_code`, and the code of the process says
whether the run was good.

The exit code goes out through the return of `main` and not through a
call that ends the process at once, because the destructor that removes
the pid file has to run. A run that the runtime kills with a signal has
no code of its own and no report.

## The surface command

`--surface` does not run a script: it prints the table of the names of
the dialect, with the state of each one in this tree. "What is missing"
becomes a number and not an opinion.

```sh
ahk --surface
```

The text is a line for every name, and a summary at the end:

```
Any class value
Array class value
BoundFunc class value
Buffer class value
Class class value
...
Abs function value
...
BlockInput command refusal: this is a name of Windows, and this desktop is not Windows.
CallbackCreate command refusal: this is a name of Windows, and this desktop is not Windows.
...
ClipboardAll.Size method value
the surface of v2 has 833 entries: 674 answer with a value, 159 are refused, 0 are pending.
```

A line is `name kind state`, and a name that is not answered with a
value carries the reason after it. A method carries the class of it, and
the kind of a name is `command`, `function`, `variable`, `method`,
`class` or `directive`.

The three states are:

| State | What it means |
| --- | --- |
| `value` | The tree answers the name with a value. |
| `refusal: <reason>` | The tree carries the name and refuses it, with the reason of the handler. |
| `pending: <reason>` | The tree does not carry the name at all. The pending table says why. |

**The state is not the behaviour.** A name can answer with a value and its
call can still fail -- the audit of the dialect found ten such names, the
`RegExMatch` that cannot write its output variables and the `Sort` that
refuses its options among them. `--behavior` runs a probe of the names that
have one and prints the answer of the call beside the state:

```sh
ahk --surface --behavior
```

```
Format function value	behaviour=value
RegExMatch function value	behaviour=error: the matches of the pattern are not carried into a variable yet.
Sort function value	behaviour=error: the options of the sort are not carried yet.
```

The probe is a small script of the dialect that calls the name the way a
script does, run against a desktop that does nothing. A name whose probe is
not written yet carries no `behaviour=`, and the table of the probes is
`crates/ahk/src/behavior.rs`. The JSON of `--json --behavior` carries the
same answer in a `behavior` member of the row. A class-qualified name is
probed under its class: `Array.Length` and `File.Length` are two probes.

Of the 190 refusals of the v2, 169 are one reason --
`this is a name of Windows, and this desktop is not Windows.` -- and the
rest are the names a runtime that is a script and not a desktop cannot
answer: the registry, the tray icon, the window of the script, the
listing of the variables.

```pwsh
target\debug\ahk.exe --surface --json
```

The JSON is for the CI, and it carries the numbers of the summary and
the state of every name:

```json
{"dialect":"v2","total":833,"values":674,"refusals":159,"pending":0,"entries":[{"kind":"class","name":"Any","state":"value"},{"kind":"function","name":"Abs","state":"value"},{"class":"Map","kind":"method","name":"Clear","state":"value"},{"kind":"command","name":"ControlSendText","reason":"this is a name of Windows, and this desktop is not Windows.","state":"refusal"}]}
```

(The `entries` array above is cut to four of its 833 members, and the
four are not adjacent in the real output.)

A row of the JSON is an object with `name`, `kind` and `state`, with
`class` when the name is a method, and with `reason` when the state is
`refusal` or `pending`.

The v1 has a surface of its own:

```pwsh
target\debug\ahk.exe --surface --json --dialect=v1
```

```json
{"dialect":"v1","total":408,"values":301,"refusals":107,"pending":0,"entries":[...]}
```

Neither dialect has a pending name: both tables are empty, and the whole
surface of each one is carried. `docs/v2-capability-report.md` groups the
same numbers by the area a name belongs to, and `docs/v2-reference.md` and
`docs/v1-reference.md` list the names themselves.

## The includes

`#Include` is resolved by the runner and not by the language: the language
crate does no I/O, and the runner is the one that has a disk to read from.
Before the script is parsed, every line that starts with `#Include` or
`#include` is replaced by the text of the file it names.

- **The path is relative to the file that asks for it,** and not to the
  working directory of the run. An include inside an included file is
  resolved from the folder of that file.
- **The path may be between double quotes or be the rest of the line.**
  `#Include "lib.ahk"` and `#Include lib.ahk` are the same line.
- **An include that is already in the tree is not put in twice.** The
  second time the file is read it is skipped. The path is canonicalised
  before it is compared, so two spellings of the same file are one file.
- **`#IncludeAgain` puts the file in every time it is asked for,** even
  when the file is already in the tree. That is the difference between the
  two lines, and the two are matched by their whole name: `#Include` is a
  prefix of `#IncludeAgain` and it is not the same line.
- **The nesting of the includes is bounded.** A file that includes itself,
  or a group of files that include one another, is refused with the reason
  rather than making the runner recurse until the stack is gone.
- **A line that is not an include is copied as it is,** with its own line
  ending. The lines of the script that the report names are the lines of
  the source after the includes went in, so a line inside an included file
  is the line of the included file as it stands in the whole.
- **The directives of the file are not copied:** the line of the include
  is replaced by the text and the `#Include` itself is gone.

A file of two functions, `lib.ahk`:

```ahk
greet(name) {
    return "hello " name
}
```

and a script that includes it:

```ahk
#Include "lib.ahk"
MsgBox greet("world")
```

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":["hello world"],"steps":[{"detail":"greet is defined","elapsed_ms":0,"line":1,"ok":true,"op":"FunctionDef"},{"detail":"hello world","elapsed_ms":0,"line":5,"ok":true,"op":"MsgBox"}],"variables":{"a_linenumber":2},"warnings":[]}
```

The `MsgBox` is on the fifth line: four lines of the included file, and
then the line of the script.

An include that cannot be found is not a run: the report is the reduced
one, and the runner exits 2.

```pwsh
'#Include "nope.ahk"
MsgBox "x"' | target\debug\ahk.exe
```

```json
{"column":0,"error":"the include \"\".\\\\nope.ahk\"\" cannot be found.","error_line":0,"notes":[],"ok":false,"output":[],"steps":[]}
```

A line of `#Include` with nothing after it is refused before the disk is
touched, with `an include needs a path.`

## The pid file and the emergency stop

A script that types forever has to be stoppable from outside, and the
only thing that can stop it is the process. The runner writes its pid to
a file, and a tool that wants to stop the run reads the file and kills
the pid.

The file is written after the script parses and before the first
statement runs, and it is removed when the run ends. Its place is the
first of these that the host has:

| Order | Where the file goes |
| --- | --- |
| 1 | `$XDG_CACHE_HOME/ahk/run.pid` |
| 2 | `$HOME/.cache/ahk/run.pid` |
| 3 | `$USERPROFILE/.cache/ahk/run.pid` |
| 4 | `<the temporary directory>/ahk/run.pid` |

The working directory is never the place, whatever the host has: the file
has to be where the stop looks for it, and the working directory changes
from one call to the next.

The content is the pid and a newline.

Three things about the file are worth knowing.

- **It is written only after the script parses.** A script that does not
  parse leaves no file at all, because no runtime was made.
- **The write is not checked.** A run whose pid file could not be written
  is not stoppable by this route, and the report says nothing about it.
- **The file can be stale.** A process killed by a signal never runs the
  destructor that removes it, and the runner installs no signal handler.
  The file can then hold the pid of a process that is gone, and pids are
  reused. A stop that trusts the file kills whatever took the pid; a stop
  that checks the command line of the pid first, as the lab server's stop
  tool does, reports the stale file and kills nothing.
- **The slot is for one runner.** Two runs on the same host write the
  same file, and the first one to end deletes the second one's file.

The three other bounds are `--timeout`, `MAX_STEPS` and `MAX_ITERATIONS`,
and they are below.

## The guard rails

A script that does not finish is stopped, and the stopping is a failure
of the run: `ok` is false, `error` says which bound was hit, and
`error_line` is the line of the statement that tripped it.

| Rail | Limit | The message |
| --- | --- | --- |
| `MAX_STEPS` | 20 000 statements | `the script ran more than 20000 statements` |
| `MAX_ITERATIONS` | 1 000 turns of a `While` body | `the loop ran more than 1000 times` |
| `MAX_DEPTH` | 1 000 nested calls | `Function recursion limit exceeded.` |
| The stack | 128 MB of thread, 64 MB of budget | The depth rail says it first. |
| `Sleep` | 60 000 ms | The pause is clamped; it is not a failure. |
| The timeout | `--timeout=N`, 60 s by default | `the script ran for more than Ns` |

The rails are the constants of `ahk::runtime::Runtime`, and a host that
embeds the runtime can change the stack budget with `with_stack_budget`.
The other numbers are not a setting.

### The steps

`MAX_STEPS` counts the statements that *begin*, so the statement that
would have been the 20 001st is the one reported. A `Loop` without a
count counts each of its turns as a statement, because the body of it may
be empty and the step count is the bound that still moves.

```pwsh
@'
Loop {
  i := 1
}
'@ | target\debug\ahk.exe
```

The run of that script stops with `the script ran more than 20000
statements` on line 2, and the report is 9 999 steps long: the array
stops where the rail stopped the body, because the array is what
completed.

A script that types is not a script that runs away: a thousand
characters are a thousand statements, and a script that clears a field
of a thousand characters is doing its job. The rails are for the runaway,
and the timeout is the bound for the slow one.

### The iterations

`MAX_ITERATIONS` is the `While` loop alone. A counted `Loop` has no
iteration guard of its own: it is bounded by the step count and by the
timeout.

```pwsh
'While true {
}' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":"the loop ran more than 1000 times","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_index":1000,"a_linenumber":1},"warnings":[]}
```

### The recursion

`MAX_DEPTH` is the nested calls, and the message is the one v2.0.19 gives
for the same bounding, because v2 stops at about 1 105 nested calls and
this tree stops at the same place with the same words.

```ahk
F(k) {
  return F(k + 1)
}
F(1)
```

```json
{"elapsed_ms":27,"error":"Function recursion limit exceeded.","error_line":2,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[{"detail":"F is defined","elapsed_ms":0,"line":1,"ok":true,"op":"FunctionDef"}],"variables":{"a_linenumber":2},"warnings":[]}
```

The stack is the other bound of a recursion, and it is behind the depth.
The frames of the tree are heavier than the ones of v2 -- about 50 KB of
stack per call in a debug build and about 5 KB in a release build -- so
the runner gives the thread that runs the script 128 MB of stack and
declares a budget of 64 MB. A run whose stack budget is gone stops the
same way the depth stops it, and the report is made instead of the
process dying: before that budget was there, the runner died with a stack
overflow and no report at all.

### The pause

`Sleep` is clamped and not refused: a pause that asks for more than
60 000 ms gets 60 000 ms, and the step says `60000`.

```pwsh
'Sleep 99999' | target\debug\ahk.exe --timeout=120
```

```json
{"elapsed_ms":60000,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":[],"steps":[{"detail":"60000","elapsed_ms":60000,"line":1,"ok":true,"op":"Sleep"}],"variables":{"a_linenumber":1},"warnings":[]}
```

The clamp is in the built-in, so a call and a statement both take it.

## The timeout

`--timeout=N` is how long the script may run, in seconds, as a decimal
number. It is 60 seconds when the flag is not there. A value that is not
a number is not read, and the default is used.

The timeout is checked **before every statement**, and that is the one
thing to know about it:

- **The check is between two statements, so a statement can overshoot
  it.** A `Sleep 3000` under `--timeout=1` runs its three seconds and
  finishes; the check that follows it is the one that would fail.
- **`error_line` is null.** The timeout stops the script between two
  statements, so it knows no line, and the report says so.

A script that runs for longer than the timeout stops when the next
statement begins. This script is a loop that sleeps 50 ms a turn, under a
timeout of one second:

```ahk
i := 0
while true {
  i := i + 1
  Sleep 50
}
```

```pwsh
@'
i := 0
while true {
  i := i + 1
  Sleep 50
}
'@ | target\debug\ahk.exe --timeout=1
```

The nineteen turns that completed are in `steps`, and the twentieth is
not, because the check before it is the one that failed:

```json
{"elapsed_ms":1007,"error":"the script ran for more than 1s","error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[{"detail":"i = 0","elapsed_ms":0,"line":1,"ok":true,"op":"Assign"},{"detail":"i = 1","elapsed_ms":0,"line":3,"ok":true,"op":"Assign"},{"detail":"50","elapsed_ms":50,"line":4,"ok":true,"op":"Sleep"}],"variables":{"a_index":21,"a_linenumber":3,"i":20},"warnings":[]}
```

(The `steps` array above is cut to its first three members; the real run
has 41.)

The `elapsed_ms` of the run is 1007 and not 1000: the check happens at the
start of a statement, so the run overshoots the timeout by the statement
it had already started.

The number in the message is the timeout as a number of seconds, with a
fraction when the flag had one: `--timeout=2.5` says `2.5s`.

The timeout is measured by the same clock as `A_TickCount` and as the
`elapsed_ms` of the report. A host that embeds the runtime hands its own
clock in, and the timeout is measured by that one.

---

The names of the dialect, and the state of each of them, are
`docs/v2-reference.md` and `docs/v1-reference.md`. What the runner can do
to the machine -- the keyboard, the pointer, the screen, the windows and
the rest -- is [the desktop backend](desktop-backend.md).