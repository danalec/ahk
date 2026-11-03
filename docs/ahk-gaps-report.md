# What is missing in the v1 and in the v2

> **Measured on 2026-11-05.** The state of the tree is the one of the commit
> `5745a6f` (the branch `port/v2-dialect`). The numbers come from the tree
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
| Answer with a value | 637 | 298 |
| Refused with a reason | 191 | 107 |
| Pending (not carried yet) | 0 | 0 |
| The refusals that are not the machine's | 1 (`ObjBindMethod`) | 1 (`ObjBindMethod`) |
| The behaviour that differs from the binary | 0 measured | 0 measured, 1 known |

The two dialects are close to done. What is left is small and is written
down here with the evidence that says it is missing.

## 1. The names

### 1.1 The one name that the tree does not carry

`ObjBindMethod` is the only name of the surface of either dialect whose
refusal is not the machine's: the tree answers it with *"this runtime does
not carry that name yet."*, the wording of the pending table.

It is the same hole in the two dialects: the tree has no reference to a method
of the script, so a method is not a thing it can bind (`AGENTS.md` §35 says
the same about the `Bind` of the class `Func`, which refuses too).
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
| *the runtime does not carry that name yet.* (`ObjBindMethod`) | 1 | 1 |

The rule of the plan is the one of `docs/ahk-v1-dialect.md` §7: *no name that
the dialect does and that the box can do should be in the refusal for lack of
implementation*. The table above is that rule checked: the only refusal for
lack of implementation is `ObjBindMethod`.

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

### 2.2 The one hole of the v1 that is known

**The quote that a string begins with.** The name of the v1 is permissive: it
holds the quote, and the check of the name refuses it. The tree ends the name
at the quote, and the script that the v1 refuses at the load is accepted.

Measured on the binary of v1.1.37.02 on 2026-11-05:

| The script | The v1 | The tree |
|---|---|---|
| `x := ""a""` | refused at the load: *"The following variable name contains an illegal character:\n\"a\"\"\""* | refused at the load, with the same words |
| `x := ""a,b"",c"` | refused at the load: *"The following variable name contains an illegal character:\n\"b\"\"\""* | **accepted**, and `x` is `a,b` |
| `x := """"` | the text is `"` | the text is `"` |

The way of the v1 that works is `x := """a""b"""`, and the tree agrees with
the v1 on it. The hole is only for the scripts that the v1 refuses.

The size of the work: the lexer of the v1 (the name holds the quote) and the
check of the name (`crates/ahk/src/v1/parameters.rs`).

### 2.3 The holes of the v2 that are known

**`ObjBindMethod`** — §1.1. The tree has no reference to a method of the
script.

**The warnings of the v2** — the `#Warn` and the other directives of the
warnings are refused with *"the warnings are not carried by this runtime."*.
This is not a hole of a name: the runtime has no warning channel at all, and
the v2 shows a dialog before the error that caused it. It is a feature, and
it is a big one: the warning of the v2 says what is wrong with the script
before it stops.

**The second parameter of the `Loop Read` of the v2** — the v2 keeps the file
open for the duration of the loop, and the `FileAppend` of the body writes to
it. The tree reads the file and writes nothing: the file of the second
parameter is not opened (`AGENTS.md` §102).

**The mode of the `Loop Files` of the v2** — the tree accepts the mode "A"
(the files and the folders), which is the mode of the v1, and the v2 refuses
it with a dialog. The modes of the v2 are the "R" and the "D", and the "D"
(the folders) is measured and agrees with the tree
(`egm-review/probe_v2_loop_files_d2.py`).

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

1. **The quote that a string of the v1 begins with** (§2.2). The lexer of the
   v1 and the check of the name. Small, and it is the last hole of the v1 that
   is known.
2. **The `ObjBindMethod`** (§1.1). Small, and it is the last name of the
   surface that the tree does not carry, in the two dialects.
3. **The mode of the `Loop Files` of the v2** (§2.3). Small: the tree refuses
   the mode that the v2 refuses.
4. **The second parameter of the `Loop Read` of the v2** (§2.3). The tree
   opens the file and the `FileAppend` of the body writes to it.
5. **The warnings of the v2** (§2.3). Big: the runtime needs a warning
   channel and the v2 shows a dialog before the error.

The v1 is otherwise done: every name of its surface is carried, the pending
table is empty, and the harness finds no difference of behaviour that is not
the machine.
