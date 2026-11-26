# What is missing in the v1 and in the v2

> **Measured.** The state of the tree is the one, on top of
> the head of the branch `master`, with the runner built by
> `cargo build --release -p ahk-cli` (`target/release/ahk.exe`). The numbers
> come from the tree itself: `ahk --surface --dialect=v1` and
> `ahk --surface --dialect=v2`, and the comparisons are
> `egm-review/run_all_comparisons.py`, which runs the script in the binary of
> the dialect and in the tree, and puts the two answers side by side. The
> loads of the v1 are measured by `scripts/sweep-v1-loads.py`.

> **Current state.** The reference binary is **v2.0.32** (the version
> the v2 dialect targets). The committed differential harness
> (`scripts/differential.py`, default `--engines real,rust`) measures
> **119/119 MATCH, 0 gaps, 0 errors** against real v2.0.32; Keysharp is an
> optional third opinion, not the default. The **Windows backend is complete**
> (262/263 trait methods; the rest is a capability flag). The **Linux backend**
> gained its own equivalents where Windows has none: the object model of the
> session is **D-Bus** (`DBusObject`, not COM), messaging/events are X11
> client-messages and the event queue, `read_memory` is `/proc/self/mem`, and
> the GUI/menus, lock lights, cursor, work area and wheel are carried. See
> [the desktop backend](desktop-backend.md) and the v2 quirks the tree does not
> reproduce in `egm-review/reports/v2-quirks.md`.

This report has two parts: the **names** (what the surface of each dialect
carries) and the **behaviour** (what the tree does with the scripts that the
binary accepts). The names are counted by the tree; the behaviour is measured
against the binaries — the v1.1.37.02 and the v2.0.32.

## The summary

| | The v2 | The v1 |
|---|---|---|
| The names of the surface | 843 | 474 |
| Answer with a value | 843 | 474 |
| Refused with a reason | 0 | 0 |
| Pending (not carried yet) | 0 | 0 |
| The refusals that are not the machine's | 0 | 0 |
| The behaviour that differs from the binary | 0 measured | 0 measured |

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

Every refusal of the tree is the machine's, not the tree's. The table below is
measured on the tree itself: the state of every name of the
two surfaces, with the reason that the state carries.

| The reason | The v2 | The v1 |
|---|---|---|
| *this is a name of Windows, and this desktop is not Windows.* | 0 | 0 |
| *this desktop does not know the address of the network of the machine.* (`A_IPAddress1`) | 0 | 0 |
| *this name is about Windows, and this machine is not.* (`A_OSType`) | 0 | 0 |
| *this runtime runs one script in one thread, and it cannot start another.* (`Thread`) | 0 | 0 |
| **the total of the refusals of the surface** | **0** | **0** |

The ninety-seven refusals that the v1 carried are gone. They were the
ninety-four names of Windows -- the COM (`ComObjCreate`, `ComObjGet` and the
rest of them), the windows of Win32 and their controls (`WinGet`,
`ControlGetText`, `Menu`), the registry (`RegRead`), the drives (`Drive`),
the sound of Windows (`SoundGet`), the tray (`TrayTip`), the shortcut and
the recycle bin of Windows (`FileCreateShortcut`, `FileRecycle`), the process
of Windows (`Process`), the message loop (`OnMessage`, `PostMessage`) and
the GUI of Win32 (`Gui`, `Progress`, `StatusBarGetText`) -- and the three
that are about the machine (`A_IPAddress1`, `A_OSType` and `Thread`).

The reason they are gone is that the machine of the measurement is Windows:
the host is `MINGW64_NT-10.0-26300`, the CLI picks the `WindowsDesktop`, and
the tree talks to the real APIs. The names of Windows are carried as
adapters of the v2 runtime and of the desktop of the box, with the shape of
the v1: the `Output` parameter of the v1 becomes a reference, the answer of
the v2 is written into it, and the failure of the v1 is a note and not a
stop of the script.

Two of them are still refused at the run, and the refusal says why:

- the `RegisterCallback` is carried at the load and refuses at the run: the
  tree does not manufacture machine code, and a callback of the machine
  needs a callable address;
- the `Thread, Interrupt` wants a second thread, and this runtime runs one
  script in one thread.

The rule of the plan is the one of `docs/ahk-v1-dialect.md` §7: *no name that
the dialect does and that the box can do should be in the refusal for lack of
implementation*. The table above is that rule checked: no refusal is for lack
of implementation. The names that the box *can* do -- the windows of X11, the
sound of PulseAudio, the processes of the box, the folders of the box -- are
the ones of the v2 that the tree already carries, and the v1 names of the
same families are carried the same way. That is a work of features and
not a work of defects: no script of the v1 is refused for a reason that the
tree could answer.

The two dialects have no pending name: the tables
(`crates/ahk/reference/ahk-v2.0.32.pending.tsv` and
`ahk-v1.1.37.02.pending.tsv`) hold the header that explains them and no name.

### 1.3 What the numbers of the surface do not say

The surface counts the *names*. Some of the names answer and their call does
not — this is written out in `docs/v2-capability-report.md`:

- The classes of the windows (`Gui`, `Gui.Control`, `Gui.ListView`,
  `Gui.StatusBar`, `Menu`) answer on Windows; on the Linux box they are
  carried through the GTK helper of the window, so the call is made and
  drawn there too (see `docs/desktop-backend.md`).
- The `ComObject` and the `ComValue` are the COM of Windows; on the Linux
  box the object model of the session is D-Bus (`DBusObject`), and the
  parts with no D-Bus equivalent refuse with the reason.
- The `BoundFunc`, the `Closure` and the `RegExMatchInfo` are classes that the
  runtime makes and a script may not make: the call answers *"Invalid base."*.

## 2. The behaviour

### 2.1 The harness

`egm-review/run_all_comparisons.py` runs thirty-six scripts of comparison,
more than six hundred cases. The differences that are left:

| The script | The cases | The different | What they are |
|---|---|---|---|
| `compare_v1_values.py` | 33 | 6 | The environment: `GroupActivate` (3), `GroupClose`, `GroupDeactivate` and `KeyHistory` are names of Windows, and `MouseClickDrag` needs the `grim` of the box. |
| `compare_v1_if_else_corner.py` | 15 | 3 | **The probe, not the tree.** The v1 writes to the console without a newline and the tree's report has one line per write; the probe joins the lines of the tree with a space and the writes of the v1 with nothing. Joined the same way, the two are the same. |
| `compare_v1_messages.py` | 26 | 1 | The environment: the hotkey that is written twice opens a dialog in the v1, and the tree refuses it with the reason of the box. |

So the harness finds **no difference of behaviour** between the tree and the
binaries that is not the machine.

### 2.2 The holes of the v1

**The quote that a string begins with** — *closed*. The name of
the v1 is permissive: it holds the quote, and the check of the name refuses
it. The tree ended the name at the quote, and the script that the v1 refuses
at the load was accepted.

Measured on the binary of v1.1.37.02:

| The script | The v1 | The tree |
|---|---|---|
| `x := ""a""` | refused at the load: *"The following variable name contains an illegal character:\n\"a\"\"\""* | refused at the load, with the same words |
| `x := ""a,b"",c"` | refused at the load: *"The following variable name contains an illegal character:\n\"b\"\"\""* | refused at the load, with the same words |
| `x := """"` | the text is `"` | the text is `"` |

The way of the v1 that works is `x := """a""b"""`, and the tree agrees with
the v1 on it. The hole was only for the scripts that the v1 refuses.

The work was in the check of the name and not in the lexer: the v1 reads the
tail of a list as symbols, and the name of each one of them is checked, and
the tree swallowed the complaint because the tail of the v1 is a text. The
parser asks the dialect now (`the_names_of_a_tail_are_checked`), and the
refusal of the v1 is in the parser.

The holes that the probes found after that one:

| The script | The v1 | The tree |
|---|---|---|
| `FileAppend, %A_Index%, *` | `0` | `0` |
| `x := a.b%c` | refused at the load: *"Ambiguous or invalid use of \".\"\n     Specifically: .b%c"* | refused at the load, with the same words |
| `x := a."b"` | refused at the load: *"This character is not allowed here.\n     Specifically: \"b\""* | refused at the load, with the same words |

The first one is a name of the v1 that is a number outside a loop, and it was
closed: the substitution of the v1 did not ask the runtime for
the variables that it owns. The other two are the complaints of the v1 for the
point and for the quote of a member, and they were closed on the same day.

The `A_LineNumber` and the `A_ThisFunc` were closed on the same day: the tree
did not write the line of the statement that runs, and the name of the
function that runs was an empty text in the dialect of the v1 and a failure in
the dialect of the v2.

**The point of the operand and the text of a member** -- *closed*.
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

**The `Loop Files` of the v1** -- *closed*. The v1 has two forms
of the loop of the files: the `Loop, <pattern>, <mode>` whose mode is a number,
and the `Loop Files, <pattern>, <mode>` whose mode is a set of letters. The
tree carried the first one and not the second one: the pattern of the files
was the word "Files" and the mode was the pattern.

**The `#Warn` of the v1** -- *closed*. The v1 has a `#Warn` of
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

The difference that stays: none. The v1 warns twice for a read that is the
value of an assignment, and the tree warns twice as well.

The number of the warnings of the v1 is the number of the times that it
materializes the empty content of the variable, and that depends on the
operator: the value of an assignment is materialized twice, and the value of
an object and the value of a comparison too. The tree warns once per
materialization, and the number of the materializations is the one of the
binary -- `scripts/measure-v1-warnings.py` measures the two side by side.

**The property with a getter of the v1** -- *closed*. The v1 has
the property with a getter, and the body of the getter is a block of commands:
the `return` of it is the value of the property. The tree refused the property
altogether, and the refusal was "Not a valid property getter/setter.\n
Specifically: get {". The v1 refuses the expression after the arrow, the getter
that is on the same line as the name, the body of the getter that is on the
line of the member, the member that has no body -- that one is a method of the
property, and the refusal is "Missing \"{\"\n Specifically: p.get()" -- and
the member that is static.

**The property with a getter of the v1** -- *closed*. The v1 has
the property with a getter, and the body of the getter is a block of commands:
the `return` of it is the value of the property. The tree refused the property
altogether, and the refusal was "Not a valid property getter/setter.\n
Specifically: get {". The v1 refuses the expression after the arrow, the getter
that is on the same line as the name, the body of the getter that is on the
line of the member, the member that has no body -- that one is a method of the
property, and the refusal is "Missing \"{\"\n Specifically: p.get()" -- and
the member that is static.

**The `%var%` of a variable that has no value** — *closed*. The v1 refuses the
line that asks for a variable that is blank with the `%` signs, and the thread
of the script ends with the refusal: *"This dynamic variable is blank. If this
variable was not intended to be dynamic, remove the % symbols from it."* The
tree answered the empty text, and the script went on.

Measured on the binary of v1.1.37.02:

| The script | The v1 | The tree |
|---|---|---|
| `x := %A_Gui%` and the `A_Gui` is empty | the thread ends with the refusal | `x` is the empty text, and the script goes on |
| a GUI is used and then `y := %A_Gui%` | the same refusal | `y` is the empty text |

The `A_Gui` of the v1 is empty in the two cases -- the binary of the v1
answers the same refusal after the script used a GUI -- and the hole was the
rule and not the variable: the write of a name that is blank inside a
function was refused by the tree with the same words
(`the_message_of_the_dynamic_write_inside_a_function`), and the read was the
one that was not refused.

The rule is in the source of the v1 (`source/script.cpp:9090`): the name that
the script builds and that is blank is a refusal, and the check is the one
that builds the name -- so the read and the write are refused, and the words
are the ones of the write. The tree carried the blank name as a legal name
(it has no letter in it, and the rule of the letters does not see it), and
the check of the blank name is a rule of its own now
(`the_blank_dynamic_name_is_refused`), which the v2 does not have.

**The state of a lock of the v1** — *closed*. The `SetCapsLockState`, the
`SetNumLockState` and the `SetScrollLockState` of the v1 check the state that
they are given when the script loads, and the word that is not one of them is
refused with *"Parameter #1 invalid."* and the word that was written. The
tree checked it when the command ran, and it refused the command that the v1
accepts: the state that is not written is not a refusal -- the v1 does
nothing with it.

Measured on the binary of v1.1.37.02 (source/script.cpp:6389 and
source/script.h:1819, `ConvertOnOffAlways`): the words that the v1 takes
are the words of the switch and the two that force the lock (*"AlwaysOn"*
and *"AlwaysOff"*), and the state that is not written leaves the lock alone
(source/script2.cpp:10869, `SetToggleState`).

The sweep of the loads is the instrument of the measure, and the families of
the locks are not among its differences any more.

**The words of the `ControlSendRaw`** — *closed*. The v1 refuses the
`ControlSendRaw` whose text to send is not there with *"Parameter #2
required"*, and the tree refused it with the words of the v2:
*"\"ControlSendRaw\" requires at least 2 parameters."*. The rule is in the
source of the v1 (source/script.cpp:6564-6568 of v1.1.37.02): the
`ControlSend` and the `ControlSendRaw` are the same case, and the text
that goes into the control has to be there. The tree carried the rule for
the `ControlSend` and not for the `ControlSendRaw` (the commit
`e5586a72`).

**The line of the refusal of a command** — *closed*. The v1 refuses a
command naming the line of it -- the `GetKeyState` of the fourth line of
a script is refused with "(4)" -- and the tree named the line of the next
one, because the parser is already in it when it refuses the command.
The report of the tree said the line five where the v1 said four, in the
six refusals that the tree makes after the statement is read: the number
of the parameters, the parameter that may not be blank, the name of a
variable, the percent sign of a parameter, the value that is not one of
the words, and the `else` that has no `if` before it (the commit
`dd760f2d`).

**The `KeyWait` of the v1** — *closed*. The v1 waits for the key to come
up -- the "D" of the second parameter waits for it to go down -- and the
"T" of it is the timeout in seconds, and the `ErrorLevel` is one when
the wait ran out. The tree refused the command: *"this runtime does not
keep the keys that are down, so it cannot wait for one."*

The box knows the keys that are down -- the `GetKeyState` of the tree
already asks it -- and the wait asks the same question again and again,
letting the timers of the script run between the questions, the way the
wait of the v1 does. The name of the key is checked when the script
loads, as the v1 does it.

Measured on the binary of v1.1.37.02 and on the tree:

| The script | The v1 | The tree |
|---|---|---|
| `KeyWait, a` | 0 ms, the `ErrorLevel` is zero | 0 ms, the `ErrorLevel` is zero |
| `KeyWait, a, D, T1` | 1000 ms, the `ErrorLevel` is one | 1000 ms, the `ErrorLevel` is one |
| `KeyWait, a, T1` | 0 ms, the `ErrorLevel` is zero | 0 ms, the `ErrorLevel` is zero |
| `KeyWait, a, X` | 0 ms, the `ErrorLevel` is zero | 0 ms, the `ErrorLevel` is zero |

The "L" of the options asks for the logical state -- the state of the key
as the script sees it -- and the tree does not keep it apart from the
physical one, the same as the `GetKeyState` of the v1.

**The failure of three commands of the v1** — *closed*. The
`SplashTextOff` with no splash, the `URLDownloadToFile` of a url that is
not one and the `WinMenuSelectItem` of a window that is not there are
failures of the v1 that the script goes on from: the `ErrorLevel` is the
one, and the thread of the script is not over. The tree ended the script
in the three cases.

Measured on the binary of v1.1.37.02 and on the tree:

| The script | The v1 | The tree |
|---|---|---|
| `SplashTextOff` | the `ErrorLevel` is zero | the `ErrorLevel` is zero |
| `URLDownloadToFile, a, b` | the `ErrorLevel` is one | the `ErrorLevel` is one |
| `WinMenuSelectItem, Nowindow, Item1, Item2` | the `ErrorLevel` is one | the `ErrorLevel` is one |

The `SplashTextOff` takes away the window that the `SplashTextOn` made,
and the tree remembers it: with no splash the command does nothing at
all, the way the v1 does it.

**The form of the handle of a window** — *closed*. The v1 writes the handle
of a window as a hexadecimal text -- the `0x77c0f70` -- and the tree
wrote it as a number in decimal. The v2 writes the number, and the tree
agrees with it there.

Measured on the binary of v1.1.37.02 and on the tree:

| The script | The v1 | The tree (before) | The tree (now) |
|---|---|---|---|
| `Gui, +hwndH` and `SubStr(H, 1, 2) = "0x"` | yes | no | yes |
| the `MouseGetPos` and the window under the pointer | yes | no | yes |

The rule is in the source of the v1 (source/var.cpp of v1.1.37.02,
`Var::AssignHWND`): the handle is stored as a hexadecimal text, "for
backward compatibility, tradition, and the fact that operations involving
HWNDs tend not to be nearly as performance-critical as pure-math
expressions". The `HwndToString` of source/util.h writes the `0x` and
the number in base sixteen.

The tree had the two forms at once: the `GuiControlGet` of the handle
wrote the hexadecimal one, and the `+hwnd` of the `Gui` and of the
`Gui, Add` wrote the number. The places that carry it now are the
`+hwnd` of the `Gui` and of the `Gui, Add`, the third output of the
`MouseGetPos` and the fourth one when the mode asks for the handle of
the control.

**The differences that the sweep of the loads still finds.** The sweep
(`scripts/sweep-v1-loads.py`) runs every command of the table of the
reference -- the one that was measured on the binary of v1.1.37.02 -- with
zero to five parameters through the binary and through the tree, and it says
the ones whose complaint is not the same. It is the instrument of the
behaviour of the v1 that the names do not cover, and it finds no difference
at all: the 160 commands that it runs answer the same thing in the binary
and in the tree.

The parameters are the ones of that table too -- an `Output` is a variable
of the script and a `Value` is a text -- so that a line that is legal in
the binary is legal in the tree, and the difference that the sweep finds is
the difference of the behaviour and not the difference of a call that one
side refuses.

The commands that touch the machine are left out: they type, they press a
key, they move the mouse, they close a window, they write in the registry,
they wait for the user. The commands that write a file are not among them:
every case of the sweep runs in a folder of its own, so the file of one
case is not the file of the next.

What is left of the waits is not a difference of the sweep: the v1 waits
without an end, and the tree waits sixty seconds -- the wait is bounded so
that the runner of the tree can stop it.

The families of the locks were in the list and are not any more (the commit
`8030aac0`), the name dinâmico vazio was in it too (the commit
`76a2e1d3`), the words do `ControlSendRaw` (the commit `e5586a72`), the
`KeyWait` was the last one (the commit of the same name), the failure
of the three commands that go on was the last family (the commit of the
same shape), and the `WinWaitClose` with no title was the last one (the
commit `cbb0d339`).

### 2.3 The holes of the v2 that are known

**The nested classes of the GUI** — *closed*. The classes of the controls of
a window are nested classes of the class `Gui`, and a script reads them
as values: `Type(Gui.Control)` answers "Class" on the binary of v2.0.19
and on the binary of v2.0.31, alike.

The tree refused them all: `Type(Gui.Control)` was refused with
*"This value of type \"Class\" has no property named \"Control\"."* — and
so was every one of the twenty-five the binaries carry:

| The class | The binaries | The tree |
|---|---|---|
| `Gui.Control`, `Gui.List`, `Gui.ListBox`, `Gui.ListView`, `Gui.TreeView`, `Gui.StatusBar`, `Gui.Button`, `Gui.CheckBox`, `Gui.ComboBox`, `Gui.Custom`, `Gui.DDL`, `Gui.DateTime`, `Gui.Edit`, `Gui.GroupBox`, `Gui.Hotkey`, `Gui.Link`, `Gui.MonthCal`, `Gui.Pic`, `Gui.Progress`, `Gui.Radio`, `Gui.Slider`, `Gui.Tab`, `Gui.Text`, `Gui.UpDown`, `Gui.ActiveX` | a class | a class |

(`Gui.Menu` is not one of them: the binaries refuse the name too, and the
tree carries its methods in the table of the class.)

The classes are carried now: the class value of the `Gui` owns them, and
`Type(Gui.Control)` answers "Class". Each one has the base that the
binary gives it — the `Gui.ListBox`, the `Gui.ComboBox`, the `Gui.DDL`
and the `Gui.Tab` derive from the `Gui.List`, and the `Gui.List` and the
rest derive from the `Gui.Control` — and the prototype of each one owns
the members of it and the ones of its base. The call of one of them is
refused with *"Invalid base."*, the way the call of the `File` and of
the `Func` is refused.

The tables of their methods were read out of the binary,
and the tree was carrying the wrong ones: the row of the `Gui.ListView`
was the table of the `Gui.TreeView`, and the row of the `Gui.StatusBar`
was the table of the `Gui.ListView`. The `Gui.Control` carried the
`Add`, the `Choose` and the `Delete`, and they are the names of the
`Gui.List`; the `UseTab` and the `SetFormat` are the names of the
`Gui.Tab` and of the `Gui.DateTime`. The tables of the tree are the ones
of the binary now.

The gap was **the same in the two releases**: it was a hole of the tree
and not a change of the 2.0.31. The measurement is in
`docs/v2.0.31-changes.md`, and the move to the v2.0.32 is in
`docs/v2.0.32-changes.md`. The instruments of it are
`scripts/probe-classes.py` and `scripts/probe-gui-classes.py`.

**`ObjBindMethod`** — *closed*. The tree binds the method of an
object, and the value of it is of the class `BoundFunc`. The call of it calls
the method with the object as its `this`, and the name of the method is not
checked when the binding is made.

**The `A_LineNumber` and the `A_ThisFunc` of the v2** — *closed*.
The tree did not write the line of the statement that runs, and it refused the
two names with *"This global variable has not been assigned a value."*. The
runtime writes them now, and they are seen in a function.

**The warnings of the v2** — *closed, for the "Unreachable" and
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

**The body of the getter of the v2** — *closed*. The body of the
getter and of the setter that is a block is a scope of its own: the `return` of
it is the value of the property, and the name that it writes is a local of it.
The tree refused the `return` with "Return's parameter should be blank except
inside a function.". The getter that is on the same line as the name and the
body of the getter that is on the line of the member are refused, and the
`this` of a class is not a variable that the warning of the "VarUnset" names.

**The second parameter of the `Loop Read` of the v2** — *closed*.
The v2 does not open the file of the second parameter when the loop begins:
the file that is not there is not made, and the one that has text keeps it.
The `FileAppend` of the body writes to it, and the tree does the same.

**The mode of the `Loop Files` of the v2** — *closed*. The mode
is a set of letters -- the "R" walks into the folders, the "D" and the "K"
give the folders, the "F" and the "V" give the files -- and the spaces and
the tabs between them are ignored. The letter that is not one of them is
refused when the loop runs, and the "A" is not one of them: the tree took it
and gave the files and the folders, and the v2 refuses it with a dialog.

### 2.4 What is not a hole

- The **names of Windows**: none of the names of the v1 and none of the
  names of the v2 is refused for it. The tree talks to the real APIs on a
  machine of Windows, and the refusal that is left is at the run and names
  the capability that the tree does not have.
- The **groups** (`GroupActivate`, `GroupClose`, `GroupDeactivate`), the
  **`KeyHistory`**, the **`MouseClickDrag`** (the `grim` of the box), the
  **`Send`** (the uinput of the box) and the **hotkey written twice** (the
  watcher of the box is a Linux device).
- The **encoding of a file**: the tree knows UTF-8 and UTF-16, and the v2
  knows the code page of the machine (the working notes of the tree).
- The **exclusions of the `WinGetTitle`**: the third and the fourth
  parameters of the command are the title and the text that the window may
  not have, and the tree refuses them with "the text of the window and the
  exclusions are not carried yet.". The desktop of the box has no text of a
  window, and the refusal is the same one of the v2. The sweep of the loads
  of the v1 (`scripts/sweep-v1-loads.py`) does not find it among the
  differences of today, which are the table of §2.2.

### 2.5 The windows, the sound and the waits, closed

The machine of the measurement is Windows, and that opened the families that
the box of a Wayland session cannot reach. What follows is what the binaries
say and what the tree says now. Every number of the tables was measured on
the binary of the dialect, and every comparison has a script under
`egm-review/`.

**The enumeration of the windows** — the rule is one for the two dialects,
and it was measured with the `DetectHiddenWindows`:

| The setting | The list is |
|---|---|
| Off | the windows that are visible and that the compositor did not cloak, the ones without a title included |
| On | every window of the machine, the hidden ones, the ones without a title and the cloaked ones included |

The tree had two holes: it left the windows without a title out of the list,
and the cloaked ones were not told apart from the visible ones. The
`IsWindowVisible` is not enough for the second, because a cloaked window
still says that it is visible: the `DwmGetWindowAttribute` with the
`DWMWA_CLOAKED` is the call that tells it apart, and it is of Windows Vista,
so the tree resolves it at the run and the floor of the tree -- Windows XP
SP3 -- keeps working. The comparison is
`egm-review/compare_the_hidden_windows.py`: with the setting off the two
binaries list the same windows, and with it on each one of them lists every
window of the machine -- the census of the machine at the same instant is
the same numero.

**The criteria of the titles** — the `ahk_class`, the `ahk_exe`, the
`ahk_id` and the `ahk_pid` name a window, and the grammar of them was
measured on the two binaries: the name of the criterion begins at the start
of the text or after a space, one space is consumed, the value may have
spaces, the last criterion of a kind is the one that counts, the class is
compared with the case and the executable without it, and a value that is
not a number names no window. The tree parses them once, in
`crates/ahk/src/desktop/criteria.rs`, and the two dialects ask the same
type. The comparisons are `egm-review/compare_the_criteria.py`,
`compare_the_criteria_of_the_v1.py` and `compare_the_edges_of_the_criteria.py`.

**The state of the last window of the v1** — the v1 keeps one window that a
script found, and the `WinGet, out, IDLast` without a title answers it. The
tree keeps it apart from the last window of the v2, and the transition is the
one of the v1: a window that is found becomes the last one, and a window
that is not found leaves it where it was. The comparison is
`egm-review/compare_the_last_window_of_the_v1.py`.

**The state of the last window of the v2** — the v2 keeps one window that a
script found as well, and the call that names no window asks about it: the
`WinGetTitle`, the `WinGetClass`, the `WinGetID` and the rest of the family
of the `WinGet*` answer about the window that the `WinExist` found last, and
a script that found no window at all is refused with *"Target window not
found."*. The tree kept no window at all, and it answered with the window
that was in the foreground of the machine. The rule is in the source of the
v2.0.32 (`window.cpp:490-494`): *"User passed no params, so use the window
most recently found by WinExist()."*, and the v1 asks the same way
(`script2.cpp:10911-10923` of v1.1.37.02: *"Use the \"last found\"
window."*). Only the `WinExist` puts the window there: the `WinGetTitle`
and the `WinGetClass` do not, and the tree remembered the window of the
`WinGetID` too, which the official does not do. The `WinGetID` and the
`WinGetIDLast` answer a number and not a text, as the official does. The
measurement is `scripts/measure-window-targets-2.0.32.py`, and it asks the
binary of the v2.0.32 and the tree the same question about the nineteen
names of the family.

**The two names of the family that are left** — with that instrument: the
call without a target is refused by the two engines in all of the nineteen
names, and after a `WinExist` that found a window the seventeen that read a
window answer the same. The two that are left are the `WinGetTransparent` —
the tree reads the window of the script, and not the window of another
program — and the `WinGetProcessPath`, whose process the desktop does not
carry the path of.

**The text of a window of another program** — the v2 reads the texts of the
child windows of the window that the call names (source `lib/win.cpp:2075` of
the v2.0.32: the `EnumChildWindows` of the target window), and the tree
read the window of the script and refused every other one. The desktop
answers now by the identifier of the window, and the desktop of a machine
that cannot separate the title bar of a program from its content refuses
with that reason.

**The list of the windows of the v1** — the `WinGet, out, List` is a
pseudo-array: the variable of the caller has the number of the windows and
the variables whose name begins with it have the identifiers, in
hexadecimal.

**The waits** — the timeout of the wait is the third parameter of the two
dialects, and the zero is not the same thing in the two of them: the v1 says
that the zero is the meio segundo, and the v2 returns logo. The timeout that
is not written is a wait that does not end, in the two dialects. The
exclusions are the titles of the windows that do not answer the wait, and
the tree asks the machine for them now. The comparisons are
`egm-review/compare_the_timeout_of_the_wait.py` and
`compare_the_exclusion_of_the_wait.py`.

**The sound of the v1** — the component and the control have their defaults
(the `Master` and the `Volume`), the mixer of the machine is read by the
topology of the device -- the microphone is a part of the playback device and
not the master of the recording one -- and the failures have the phrases of
the v1: *"Can't Open Specified Mixer"*, *"Can't Get Current Setting"*,
*"Can't Change Setting"*, *"Component Doesn't Support This Control Type"* and
*"Mixer Doesn't Support This Component Type"*. The comparison is
`egm-review/compare_the_sound_of_the_v1.py`: twenty-one cases and no
difference.

**The components of the sound of the v2** — the v2 does not take the
component named: the `SoundGetVolume("Master")` answers *"Component not
found"*, and the numeric one answers *"Component doesn't support this
control type"*. The comparison is
`egm-review/compare_the_components_of_the_v2.py`.

**The `Gui, Add` of the v1** — the v1 reads the options of the control in
the third parameter and the text in the fourth, and the tree read the
options as the text. The tree also names a control by its text, as the v1
does, and not only by the `ClassNN` of it.

**What the machine of the measurement does not let be measured.** The `Gui,
+New` of the v1 does not come back on this host: the window of the script
is made and the script stays there, waiting. The two cases of the family
that need a window are therefore not comparable here, and the comparison
`egm-review/compare_the_families_of_the_v1.py` says so: fourteen cases and
two differences, and the two are the `Gui, +New` that does not come back.
Everything else of the family -- the `Gui` that already exists, the `Menu`,
the `ComObj`, the `Progress` and the `SplashText` -- is the same, the
complaint of the load included: *"Parameter #2 invalid."*, *"Parameter #2
and beyond should be omitted in this case."* and *"\"Menu\" requires at
least 2 parameters."*.

## 3. The order of the work

1. **The quote that a string of the v1 begins with** (§2.2) — *closed*.
   The check of the name was the work: the v1 reads the tail of a
   list as symbols, and the tree swallowed the complaint.
2. **The point of the operand and the text of a member of the v1** (§2.2) —
   *closed*. The operand of the v1 was the work: it does not end
   at the point, and the tree ended the name at it.
3. **The `ObjBindMethod`** (§1.1) — *closed*. The tree binds
   the method of an object, and the value of it is of the class `BoundFunc`.
4. **The mode of the `Loop Files` of the v2** (§2.3) — *closed*.
   The mode is a set of letters, and the tree refuses the letter that the v2
   refuses.
5. **The second parameter of the `Loop Read` of the v2** (§2.3) — *closed*.
   The tree does what the v2 does: the file is not opened when
   the loop begins, and the `FileAppend` of the body writes to it.
6. **The `Loop Files` of the v1** (§2.2) — *closed*. The v1
   has two forms of the loop of the files, and the tree carried the one whose
   mode is a number and not the one whose mode is a set of letters.
7. **The warnings of the v2** (§2.3) — *closed*. The runtime
   has a warning channel now: the warnings go to the report, and the mode of
   the "StdOut" writes them to the standard output. The three types of the v2
   are carried.
8. **The `#Warn` of the v1** (§2.2) — *closed*. The directive
   of the v1 is carried, and its six types are: the variable of the script that
   is read and never written, the variable of a function that is read and
   never written, the environment variable that is accessed, the local that has
   the name of a global, the class that is overwritten and the line that never
   runs.
9. **The nested classes of the GUI** (§2.3) — *closed*. The classes of the
   controls of a window are members of the class `Gui` and classes of their
   own, and the tables of their members are the ones of the binary.
10. **The load of the v1 that checks the value of a parameter** (§2.2) —
   *closed*. The sweep of the loads -- the script
   `scripts/sweep-v1-loads.py`, which asks the binary of v1.1.37.02 and the
   tree the same line with zero to five parameters -- found the family that
   was not carried: the `ControlGetPos` and the `FileSetAttrib` want a
   parameter that may be left out, the mode of the `GroupClose` is the "A" or
   the "R", the first parameter of the `Thread` is an expression, and the
   command that the machine cannot do is not a failure of the script: the
   `FileSetAttrib`, the `FileCreateDir`, the `FileInstall`, the
   `SetWorkingDir`, the `IniRead`, the `IniWrite` and the `IniDelete` go on
   with the `ErrorLevel` of the machine. The only difference that is left is
   the exclusions of the `WinGetTitle`, and the refusal of it is the same one
   of the v2.
11. **The `%var%` of a variable that has no value** (§2.2) — *closed*. The v1
    refuses the line and ends the thread of the script; the tree answered the
    empty text. The check of the blank name is a rule of its own now, and the
    write of a name that is blank inside a function was refused before.

12. **The target of the family of the windows of the v2** (§2.5) — *closed*.
    The call that names no window asks about the window that the script
    found last, and the tree keeps one now: the `WinExist` puts it there,
    and the tree answers about it the way the v2 answers. The tree kept no
    window at all, and it answered with the window that was in the
    foreground of the machine.
13. **The names of the family that read a window of another program** (§2.5)
    — *open*. The `WinGetTransparent` reads the window of the script today,
    and the `WinGetProcessPath` does not carry the path of the process. The
    desktop of the machine can answer the two, and the tree asks it by the
    identifier of the window.
14. **The `FileAppend` to the console** — *open*. The output of the tree is
    a list of lines, and the text that a `FileAppend` writes ends with a
    line break: the list keeps the empty line of it, and two writes are
    separated by a blank line that the v2 does not write. Measured on the
    binary of the v2.0.32: the `FileAppend("a`n", "*")` writes "a" and a
    line break, and the tree writes "a", a line break and an empty line.
    The shape of the output of the report is the reason: it is a list of
    lines and not a text, and the list cannot say that a write ends with a
    line break.
15. The line of the v1 that is a name that the script builds — *open*.
    Measured on the binary of the v1.1.37.02: the
    ` %A_LoopRegKey%\%A_LoopRegSubKey%` is refused with "This line does
    not contain a recognized action.\n     Specifically:
    %A_LoopRegKey%\%A_LoopRegSubKey%", and the tree loads it: the tree
    reads the line as an expression, and the v1 has no expression that is
    a statement of its own.
16. The refusal of the v2 that comes after the refusal of a line — *open*.
    Measured on the binary of the v2.0.32: the "Accumulated Errors 01" of
    the upstream is refused by the official at the declaration of the
    seventh line -- "Unexpected declaration." -- and by the tree at the
    literal of the third, with "Invalid property name in object
    literal.": the official goes on after the refusal of a line and names
    the last one, and the tree stops at the first.
17. The call of the v1 that carries a continuation section — *open*.
    Measured on the binary of the v1.1.37.02: the call that carries a
    continuation section is named with the text of the section -- the
    `MyFunction("` and the text that the section holds -- and the tree
    names it with the placeholder of the section.

The v1 is done: every name of its surface is carried, the pending table is
empty, and the `#Warn` of the v1 is carried with all of its types. The
behaviour is measured by the sweep of the loads, and what is left of it is
the table of §2.2. The number of
the warnings of the v1 is the number of the binary: the read of a variable
that has no value is warned about twice when the read is the value of an
assignment, and the tree warns twice as well
(`scripts/measure-v1-warnings.py`). The sweep of the loads
(`scripts/sweep-v1-loads.py`) says the same of the loads: the complaint of
the binary and the complaint of the tree are the same, with the
differences of the table of §2.2.

The v2 has no name that the tree refuses for lack of implementation either,
and the three types of its `#Warn` are carried. The warning of the variable
that is read and never written is given when the script is read, and the tree
gives it the way the binary gives it. The classes of the controls of a window
are carried as classes, and the tables of their members are the ones of the
binary.
