# What is missing in the v1 and in the v2

> **Measured on 2026-11-05.** The state of the tree is the one of the commit
> `d41dec5` (the branch `port/v2-dialect`). The numbers come from the tree
> itself: `ahk --surface --dialect=v1` and `ahk --surface --dialect=v2`, and
> the comparisons are `egm-review/run_all_comparisons.py`, which runs the
> script in the binary of the dialect and in the tree, and puts the two
> answers side by side.

This report has two parts: the **names** (what the surface of each dialect
carries) and the **behaviour** (what the tree does with the scripts that the
binary accepts). The names are counted by the tree; the behaviour is measured
against the binaries — the v1.1.37.02 and the v2.0.19.

## The summary

| | The v2 | The v1 |
|---|---|---|
| The names of the surface | 828 | 405 |
| Answer with a value | 638 | 299 |
| Refused with a reason | 190 | 106 |
| Pending (not carried yet) | 0 | 0 |
| The refusals that are not the machine's | 0 | 0 |
| The behaviour that differs from the binary | 0 measured | 0 measured, 1 known |

The two dialects are close to done. What is left is small and is written
down here with the evidence that says it is missing.

## 1. The names

### 1.1 No name of the surface is refused for lack of implementation

The `ObjBindMethod` was the last name of the surface of either dialect whose
refusal was not the machine's, and it is carried now: the tree binds the
method of an object, and the value of it is of the class `BoundFunc`. The
name of the method is not checked when the binding is made -- the
`ObjBindMethod(t, "Nope")` is made, and the call of it fails -- and the call
of the value calls the method with the object as its `this`.

No name of either surface refuses with *"this runtime does not carry that
name yet."* any more, and the pending table of the two dialects holds the
header that explains them and no name.
### 1.2 The refusals that are the right answer

Every other refusal is the machine's, not the tree's:

| The reason | The v2 | The v1 |
|---|---|---|
| *this is a name of Windows, and this desktop is not Windows.* | 169 | 96 |
| *the registry is a thing of Windows, and this runtime does not carry it.* | 5 | — |
| *this runtime has no tray icon.* | 3 | 2 |
| *this runtime runs one script in one thread, and it cannot start another.* (`Thread`) | 1 | 1 |
| *this runtime runs one script in one thread, and it cannot make it critical.* (`Critical`) | 1 | 1 |
| *this runtime has no window of its own.* (`ScriptHwnd`, `A_ScriptHwnd`) | 1 | 1 |
| *the lines/variables are listed in a window of v2.* (`ListLines`, `ListVars`) | 2 | — |
| *this runtime cannot wait for a key to continue.* (`Pause`) | 1 | — |
| *this runtime is a script that ends when it ends.* (`Persistent`) | 1 | — |
| *this runtime runs a script once.* (`Reload`) | 1 | — |
| *this runtime cannot stop itself.* (`Suspend`) | 1 | — |
| *this runtime reads no text of a window.* (`WinGetText`) | 1 | — |
| *this runtime is not AutoHotkey.* (`AhkPath`, `A_AhkPath`) | 1 | 1 |
| *the shape of the cursor is not read.* (`Cursor`, `A_Cursor`) | 1 | 1 |
| *the desktop does not know the scale of the output.* (`ScreenDPI`, `A_ScreenDPI`) | 1 | 1 |
| *the desktop does not know the address of the network.* (`A_IPAddress1`) | — | 1 |
| *this name is about Windows.* (`A_OSType`) | — | 1 |

The rule of the plan is the one of `docs/ahk-v1-dialect.md` §7: *no name that
the dialect does and that the box can do should be in the refusal for lack of
implementation*. The table above is that rule checked: no refusal is for lack
of implementation.

The two dialects have no pending name: the tables
(`crates/ahk/reference/ahk-v2.0.19.pending.tsv` and
`ahk-v1.1.37.02.pending.tsv`) hold the header that explains them and no name.

### 1.3 What the numbers of the surface do not say

The surface counts the *names*. Some of the names answer and their call does
not — this is written out in `docs/v2-capability-report.md`:

- The classes of the windows (`Gui`, `Gui.Control`, `Gui.ListView`,
  `Gui.StatusBar`, `Menu`) are carried and every call of them fails: *"a
  window is a Windows window, and this desktop is a Wayland session without
  one"*.
- The `ComObject` and the `ComValue` are classes of Windows that this runtime
  does not carry: *"the COM objects are a thing of Windows, and this runtime
  does not carry them."*.
- The `BoundFunc`, the `Closure` and the `RegExMatchInfo` are classes that the
  runtime makes and a script may not make: the call answers *"Invalid base."*.

## 2. The behaviour

### 2.1 The harness

`egm-review/run_all_comparisons.py` runs thirty-six scripts of comparison,
more than six hundred cases. The differences that are left:

| The script | The cases | The different | What they are |
|---|---|---|---|
| `compare_v1_values.py` | 33 | 7 | The environment: `GroupActivate` (3), `GroupClose`, `GroupDeactivate` and `KeyHistory` are names of Windows, and `MouseClickDrag` needs the `grim` of the box. |
| `compare_v1_if_else_corner.py` | 15 | 3 | **The probe, not the tree.** The v1 writes to the console without a newline and the tree's report has one line per write; the probe joins the lines of the tree with a space and the writes of the v1 with nothing. Joined the same way, the two are the same. |
| `compare_v1_messages.py` | 26 | 1 | The environment: the hotkey that is written twice opens a dialog in the v1, and the tree refuses it with the reason of the box. |
| `compare_v1_rest_of_line2.py` | 40 | 1 | The environment: the `Send` is the keyboard of the machine, and the box's keyboard is a uinput device. |

So the harness finds **no difference of behaviour** between the tree and the
binaries that is not the machine.

### 2.2 The holes of the v1

**The quote that a string begins with** — *closed on 2026-11-05*. The name of
the v1 is permissive: it holds the quote, and the check of the name refuses
it. The tree ended the name at the quote, and the script that the v1 refuses
at the load was accepted.

Measured on the binary of v1.1.37.02 on 2026-11-05:

| The script | The v1 | The tree |
|---|---|---|
| `x := ""a""` | refused at the load: *"The following variable name contains an illegal character:\n\"a\"\"\""* | refused at the load, with the same words |
| `x := ""a,b"",c"` | refused at the load: *"The following variable name contains an illegal character:\n\"b\"\"\""* | refused at the load, with the same words (2026-11-05) |
| `x := """"` | the text is `"` | the text is `"` |

The way of the v1 that works is `x := """a""b"""`, and the tree agrees with
the v1 on it. The hole was only for the scripts that the v1 refuses.

The work was in the check of the name and not in the lexer: the v1 reads the
tail of a list as symbols, and the name of each one of them is checked, and
the tree swallowed the complaint because the tail of the v1 is a text. The
parser asks the dialect now (`the_names_of_a_tail_are_checked`), and the
refusal of the v1 is in the parser.

The holes that the probes of 2026-11-05 found after that one:

| The script | The v1 | The tree |
|---|---|---|
| `FileAppend, %A_Index%, *` | `0` | `0` (2026-11-05) |
| `x := a.b%c` | refused at the load: *"Ambiguous or invalid use of \".\"\n     Specifically: .b%c"* | refused at the load, with the same words (2026-11-05) |
| `x := a."b"` | refused at the load: *"This character is not allowed here.\n     Specifically: \"b\""* | refused at the load, with the same words (2026-11-05) |

The first one is a name of the v1 that is a number outside a loop, and it was
closed on 2026-11-05: the substitution of the v1 did not ask the runtime for
the variables that it owns. The other two are the complaints of the v1 for the
point and for the quote of a member, and they were closed on the same day.

The `A_LineNumber` and the `A_ThisFunc` were closed on the same day: the tree
did not write the line of the statement that runs, and the name of the
function that runs was an empty text in the dialect of the v1 and a failure in
the dialect of the v2.

**The point of the operand and the text of a member** -- *closed on 2026-11-05*.
The v1 reads the operand of an expression -- the text that goes from its first
character to the first character that ends it, and the point does not end it --
and it refuses the operand that has a point and a percent sign after the point,
before it refuses anything else of it. The name of a member may not be a text
either.

The work was in the parser: the tree computes the operand of the v1 now, and
it asks the dialect for the two refusals
(`the_refusal_of_the_point_of_an_operand` and
`the_refusal_of_the_text_of_a_member`), and the refusals of the v1 are in
`crates/ahk/src/v1/parameters.rs`. The closing percent sign of a name has to
be in the operand of the one that opens it, and that was the work of the
parser too.

**The `Loop Files` of the v1** -- *closed on 2026-11-05*. The v1 has two forms
of the loop of the files: the `Loop, <pattern>, <mode>` whose mode is a number,
and the `Loop Files, <pattern>, <mode>` whose mode is a set of letters. The
tree carried the first one and not the second one: the pattern of the files
was the word "Files" and the mode was the pattern.

**The `#Warn` of the v1** -- *closed on 2026-11-05*. The v1 has a `#Warn` of
its own, and its types are not the ones of the v2: the "UseUnsetLocal", the
"UseUnsetGlobal", the "UseEnv", the "LocalSameAsGlobal", the "ClassOverwrite",
the "Unreachable" and the "All". The modes are the "MsgBox", the "StdOut", the
"OutputDebug" and the "Off". The words of the refusal are the ones of the v2 --
"Parameter #1 invalid." and the source of the line, "Parameter #2 invalid." and
the text that was written.

The local that has the name of a global one is warned about inside a class as
well: the v1 names the method "A.m" and the v2 names the method of the prototype
"A.Prototype.m". The local of the v1 is a name that the function uses: the name
that it only reads is a local of it too, and the v2 reads the scope of the script
when the name is only read.

The tree carries all of them: the variable of the script that is read and
never written, the variable of a function that is read and never written, the
environment variable that is accessed, the local that has the name of a global
and the class that is overwritten.

The difference that stays: the v1 warns twice for a read that is the value of
an assignment, and the tree warns once. The text of the warning is the same.

The number of the warnings of the v1 is the number of the times that it
materializes the empty content of the variable, and that depends on the
operator: the value of an assignment is materialized twice, and the value of
an object and the value of a comparison too. The tree warns once per read.

### 2.3 The holes of the v2 that are known

**`ObjBindMethod`** — *closed on 2026-11-05*. The tree binds the method of an
object, and the value of it is of the class `BoundFunc`. The call of it calls
the method with the object as its `this`, and the name of the method is not
checked when the binding is made.

**The `A_LineNumber` and the `A_ThisFunc` of the v2** — *closed on 2026-11-05*.
The tree did not write the line of the statement that runs, and it refused the
two names with *"This global variable has not been assigned a value."*. The
runtime writes them now, and they are seen in a function.

**The warnings of the v2** — *closed on 2026-11-05, for the "Unreachable" and
the "VarUnset"*. The `#Warn` takes the type of the warning and the mode of it,
and the type that is not one of them is refused at the load with "Parameter #1
invalid." and the source of the line, and the mode that is not one of them
with "Parameter #2 invalid." and the text that was written. The warning of the
line that never runs is of the line that comes right after the statement that
leaves the level, and it is computed before the script runs. The warning of
the variable that is read and never written is given when the variable is
read, and before the failure of the run.

The tree has no dialog, and the warning goes to the report: the report has the
warnings of the script, and the mode of the "StdOut" writes them to the
standard output as well. The type that was left — the "LocalSameAsGlobal" —
is carried too: the local that has the name of a global one is warned about
when the script is read.

The warning of the variable that is read and never written is given when the
script is read, and not when the variable is read. The v2 measures it as the
binary measures it: the `if 0 { x := nope }` is warned about, and the name that
the scope writes is not warned about.

**The second parameter of the `Loop Read` of the v2** — *closed on 2026-11-05*.
The v2 does not open the file of the second parameter when the loop begins:
the file that is not there is not made, and the one that has text keeps it.
The `FileAppend` of the body writes to it, and the tree does the same.

**The mode of the `Loop Files` of the v2** — *closed on 2026-11-05*. The mode
is a set of letters -- the "R" walks into the folders, the "D" and the "K"
give the folders, the "F" and the "V" give the files -- and the spaces and
the tabs between them are ignored. The letter that is not one of them is
refused when the loop runs, and the "A" is not one of them: the tree took it
and gave the files and the folders, and the v2 refuses it with a dialog.

### 2.4 What is not a hole

- The **names of Windows**: 169 of the v2 and 96 of the v1. The box is a
  Wayland session on Linux, and the refusal is the right answer for it.
- The **groups** (`GroupActivate`, `GroupClose`, `GroupDeactivate`), the
  **`KeyHistory`**, the **`MouseClickDrag`** (the `grim` of the box), the
  **`Send`** (the uinput of the box) and the **hotkey written twice** (the
  watcher of the box is a Linux device).
- The **encoding of a file**: the tree knows UTF-8 and UTF-16, and the v2
  knows the code page of the machine (`AGENTS.md` §43).

## 3. The order of the work

1. **The quote that a string of the v1 begins with** (§2.2) — *closed on
   2026-11-05*. The check of the name was the work: the v1 reads the tail of a
   list as symbols, and the tree swallowed the complaint.
2. **The point of the operand and the text of a member of the v1** (§2.2) —
   *closed on 2026-11-05*. The operand of the v1 was the work: it does not end
   at the point, and the tree ended the name at it.
3. **The `ObjBindMethod`** (§1.1) — *closed on 2026-11-05*. The tree binds
   the method of an object, and the value of it is of the class `BoundFunc`.
4. **The mode of the `Loop Files` of the v2** (§2.3) — *closed on 2026-11-05*.
   The mode is a set of letters, and the tree refuses the letter that the v2
   refuses.
5. **The second parameter of the `Loop Read` of the v2** (§2.3) — *closed on
   2026-11-05*. The tree does what the v2 does: the file is not opened when
   the loop begins, and the `FileAppend` of the body writes to it.
6. **The `Loop Files` of the v1** (§2.2) — *closed on 2026-11-05*. The v1
   has two forms of the loop of the files, and the tree carried the one whose
   mode is a number and not the one whose mode is a set of letters.
7. **The warnings of the v2** (§2.3) — *closed on 2026-11-05*. The runtime
   has a warning channel now: the warnings go to the report, and the mode of
   the "StdOut" writes them to the standard output. The three types of the v2
   are carried.
8. **The `#Warn` of the v1** (§2.2) — *closed on 2026-11-05*. The directive
   of the v1 is carried, and its six types are: the variable of the script that
   is read and never written, the variable of a function that is read and
   never written, the environment variable that is accessed, the local that has
   the name of a global, the class that is overwritten and the line that never
   runs.

The v1 is done: every name of its surface is carried, the pending table is
empty, the harness finds no difference of behaviour that is not the machine,
and the `#Warn` of the v1 is carried with all of its types. The one difference
that is left is the number of the warnings of the v1 when the variable that is
read is the value of an assignment: the v1 warns twice and the tree warns
once, and the text of the warning is the same.

The v2 has no name that the tree refuses for lack of implementation either,
and the three types of its `#Warn` are carried. The warning of the variable
that is read and never written is given when the script is read, and the tree
gives it the way the binary gives it.
