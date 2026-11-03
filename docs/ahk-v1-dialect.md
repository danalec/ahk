# The v1 dialect beside the v2: the gate, the phases, and what stays out

> The plan. The authority is the binary of the **AutoHotkey v1.1.37.02** and
> the documentation of the v1 — measured on 2026-10-28. The tree speaks
> **v2.0.19** today; this document is about putting the **v1** beside it,
> without touching the v2.

---

## 0. What this plan is, and what it inverts

The tree is the v2.0.19. This plan proposes a **second dialect** , with the v1
locked behind a *gate* , and the v2 as the default.

This inverts a line written in `docs/ahk-x11-comparison.md` :

> | Do not port | Why |
> |---|---|
> | The legacy v1 syntax | Our dialect is v2.0.19, and that is the competitive advantage |

The reason stays valid for the **v2** : the v2 is the main dialect and the
competitive advantage. The change is that the v1 becomes an **option** — for
the scripts that already exist and that nobody will rewrite.

**The comparison is under review** — the wave of 2026-10-29 rewrites that
document, and the numbers of its sections change with it. The decision is
written in both places: the comparison says that the line is true of the v2
and only of the v2, and points to this plan; this plan says the same and
points to the comparison. And the `AGENTS.md` points here. Two other rows of
the same table are also inverted — for the dialect of the v1, and only for it:

| Row of the comparison | For the v2 | For the v1 |
|---|---|---|
| The "output var" convention (`StringLen, out, in`) | the return value, and the `&` when a write is needed | the convention is the one of the measured v1, and the table of the built-ins gains the column of the shape of the parameter (§3.2) |
| `#NoEnv` as the default | the environment is explicit, `EnvGet` is there | the `#NoEnv` exists and turns the substitution off (§3) |

### The coordination with the wave of 2026-10-29

Another session works in this tree at the same time, and its work is **other**
: closing the gaps of the port into the **v2** — the clipboard, the sound, the
`NumGet` and the `NumPut` , the files, the date — and building the command
`ahk --surface` , which prints the state of each v2 name. It is rewriting
`docs/ahk-x11-comparison.md` at the same time.

The two plans are **orthogonal** : the wave ports *capabilities* into the v2;
this plan adds a *dialect* beside the v2, without touching any capability. Two
things are agreed:

1. **The `--surface` belongs to the wave, and this plan uses it.** The phases
   F1 and F5 need the state of each name — and the command already exists (
   `crates/ahk/src/surface.rs` , `ahk --surface` ). This plan
   **extends it with a parameter** ; it does not make a second command.
2. **The comparison is rewritten, and the plan does not depend on the number of the sections.**
   The plan quotes the text; the `AGENTS.md` points to the plan.

The extension that F1 needs is small. The `surface::entries()` of today reads
the v2 table that is embedded and asks the tree, name by name —
`builtins::find` and `builtins::refusal_of` for the commands and the
functions, `variables::carries` and `variables::refusal_of` for the variables,
and `classes::carries` for the methods. With a dialect, each one of those
questions comes to carry the dialect, and `entries()` comes to be
`entries_for(dialect)` . The vocabulary stays as it is — `Value` ,
`Refusal(reason)` , `Pending(reason)` ; it is the sentence of the summary that
swaps the "of v2" for the name of the dialect.

And the `Pending` is what F1 needs: the pending table of the v2 is **empty** —
the port carries the whole v2 — and the one of the v1 will be the first that
is not. That is where the names of the v1 that the v2 does not have come in,
and the `Pending` is the right answer for each one of them. The first one of
them is the `SetBatchLines` (§5).

### The authority

| Source | What it is | Where |
|---|---|---|
| `AutoHotkeyU64.exe` (v1.1.37.02) | the truth: the messages, the shapes, the names | downloaded from `autohotkey.com/download/1.1/` on 2026-10-28; portable, nothing installed |
| `autohotkey.com/docs/v1/` | the list of the commands and the functions, and the grammar |same |
| `docs/ahk-x11-comparison.md` | the names that changed from the v1 to the v2 (the section "The names that changed from the v1 to the v2") | in the repository, under review |
| `AutoHotkey.chm` (in the zip of the v1.1) | the offline documentation | in the zip |
| `AutoHotkey64.exe` (v2.0.19) | the truth of the **v2**: the messages of the run that are not on the console | `E:\autohotkey\AutoHotkey64.exe` on the workstation, and `%TEMP%\ahk-v2-measure\AutoHotkey64.exe` (the two are the same bytes); the package of scoop (`2.0.30`) is a junction without files, and the shims are dead |
| the source of the v1 (`script.cpp` and `script_expression.cpp`) | the mechanism of the dialect, when the measurement says what happens and the source says why | `raw.githubusercontent.com/Lexikos/AutoHotkey_L/master/source/`; it is GPL, so it is read and never copied, and it stays out of the repository |

**The archive `../archived/ahk-python` is not a reference.** It is not an
implementation of the v1: its own note says
*"The dialect is the one of AutoHotkey v2. `:=` assigns, `=` compares"* (
`archived/ahk-python/AGENTS.md:38-41` ), and its 9 commits are all of
2026-10-25 — the first one was already v2. It is the record of the *yoko-ahk*
dialect before the port, not of the v1.

### What the gate has to resolve

1. The **surface** : the names, the arities and the shapes of the parameters.
   The two dialects do not have the same names,
   **nor the same kind for the same name** (see §1.4).
2. The **grammar** : the legacy command with commas ( `Msg, texto` ), the `=`
   that assigns, the *labels* , the one-statement `if (x)` , the hotstring as
   a **statement** and not as a command.
3. The **semantics** : the `=` of the v1 compares without distinguishing case,
   the `%nome%` is the *environment variable* , the variables are global by
   default, and the v1 has *threads* — the v1.1 runs each hotkey in its own
   thread, and the v2 has no threads.

---

## 1. What was measured on the v1.1.37.02 (2026-10-28)

Everything that follows was measured on the binary, with `/ErrorStdOut` (the
v1 opens a dialog when a script fails; with the option, the reason comes out
on stdout and the process ends).

### 1.1 The rules of the syntax

| Rule | v1.1.37.02 measured | v2.0.19 (the tree) |
|---|---|---|
| Legacy command | `FileAppend, hello, C:\...\out.txt` writes "hello" | `FileAppend "hello", "..."` — the comma after the name is a syntax error (`parser.rs:462-499`) |
| Parameter of a command | `StringLen, n, x` → `n` = 5: the parameter is the **name of a variable** | the parameter is an expression |
| Text parameter | `StringLen, n, %x%` → `n` = 0 (the `%…%` form is the *legacy* substitution) | there is none |
| Expression parameter | `StringLen, n, (x)` → refused: *"The following variable name contains an illegal character: "(x)""* — the `(…)` form exists only in the parameters that are expressions | the whole expression |
| Assignment | `x = 5` assigns (and `y := 7` as well) | `:=` assigns; `=` compares |
| Comparison | `"ABC" = "abc"` → **true**; `"ABC" == "abc"` → false | the same as the v1: `=` does not distinguish the case and `==` does (`runtime.rs:2798-2809`, measured on the binary of v2.0.19 on 2026-11-01: `"ABC" = "abc"` is 1 and `"ABC" == "abc"` is 0) |
| `if` | `if (x > 1)` with **one** statement and `else` — without braces | `if` with an expression and a block `{ }` |
| *Labels* | `MyLabel:` and `goto MyLabel2` work | they do not exist |
| Hotstring | it is a **statement**: `::btw::by the way`, or `::btw::` followed by the lines and the `return` | `Hotstring(":btw:", Callback)` — a command |
| Hotkey | `Hotkey, ^a, MyCallback` — the target is a *label* | `Hotkey("^a", Callback)` — the target is the object of a function |
| Threads | `#MaxThreads 10`, `#MaxThreadsBuffer 100`, `Thread, Priority, 1` — all exist | the directives of the v1 are refused (`runtime.rs:2131-2132`) |
| Directives | `#NoEnv`, `#Persistent`, `#SingleInstance Force` exist | the set is the one of the v2 (19) |
| Objects | `o := {}`, `o.x := 5`, `Object()` — exist | `Map`, `Array`, `Buffer`, `Func`, `Gui`, `Menu`, `Hotkey` — nine classes, and the v1 has none |
| Errors | unknown function: *"Call to nonexistent function. Specifically: X"*; unknown command: *"This line does not contain a recognized action. Specifically: X"* | *"Call to nonexistent function."* |

### 1.2 The variables of the v1

`A_AhkVersion` is `1.1.37.02` . The v1 has variables that the v2 does not
have, and values that the v2 does not share:

| Variable | v1.1.37.02 | v2.0.19 |
|---|---|---|
| `A_SendMode` | `Event` | `Input` |
| `A_StoreCapsLockMode` | `On` | `1` |
| `A_ControlDelay` | `20` | `20` — a setting of the tree |
| `A_WinDelay` | `100` | `100` — a setting of the tree |
| `A_DetectHiddenWindows` / `A_DetectHiddenText` | `Off` / `On` | `0` / `1` — settings of the tree |
| `A_TitleMatchMode` / `A_TitleMatchModeSpeed` | they exist | `2` / `Fast` — settings of the tree |
| `A_RegView` / `A_Cursor` | they exist | `Default` / `Arrow` — settings of the tree |
| `A_FormatFloat` / `A_FormatInteger` | `0.6` / `D` | do not exist |
| `A_IsAdmin`, `A_IsCompiled`, `A_IsCritical`, `A_IsPaused`, `A_IsSuspended` | they exist | they exist, with the same names |
| `A_IconFile`, `A_IconHidden`, `A_IconNumber`, `A_IconTip` | they exist | they exist, with the same names |
| `A_ToolTipDelay`, `A_ToolTipTimeout` | **do not exist** | do not exist |
| `A_LineFile`, `A_LineNumber`, `A_LoopFile*`, `A_LoopReg*`, `A_LoopReadLine` | they exist | they exist, with the same names |
| `A_KeyDelay`, `A_MouseDelay`, `A_Now`, `A_NowUTC`, `A_WDay`, `A_YWeek`, `A_MMM`, `A_MMMM` | the same | the same |
| `A_CoordModeCaret`, `A_CoordModeMenu`, `A_FileEncoding` | they exist | they exist, with the same names |
| `A_Clipboard` | the text of the clipboard | the text of the clipboard — the same name |

The names of the variables of v2 are the ones of its documentation --
`A_IsAdmin`, `A_AppData`, `A_ScreenWidth` -- and the table of the tree
carries them **without** the prefix. That is the convention of the table,
and not the spelling of the dialect: `variables::spelled` drops the `A_`
when the script writes a name, and the table holds what is left.

**The spelling of the dialect is the prefix, for every one of them.**
The documentation of the v2 lists **116** built-in variables and every
one of them has the prefix -- `A_IsAdmin`, `A_AppData`, `A_ScreenWidth`,
`A_Clipboard` -- and there is no variable named `Clipboard` at all
(`autohotkey.com/docs/v2/Variables.htm`, read on 2026-10-29).

Measured on the binary of the v2.0.19 on the same day (`A_AhkVersion`
answers `2.0.19`): `x := A_IsAdmin` answers `0`, and `x := IsAdmin`
answers *"Warning: This global variable appears to never be assigned a
value. Specifically: IsAdmin"*. The clipboard is `A_Clipboard`, like the
clipboard of the v1: `x := A_Clipboard` answers the text of the clipboard,
and `x := Clipboard` answers *"Warning: This global variable appears to
never be assigned a value. Specifically: Clipboard"*.

**And the tree asks for the prefix, as the dialect does.** Its reading
path wants the name the dialect spells, so it answers to `A_IsAdmin` and
not to `IsAdmin`, and to `A_Clipboard` and not to `Clipboard`. A name
without the prefix is a variable of the script, and the tree says that it
has no value; the dialect warns about the same name and answers an empty
value. A script of the v2 that uses the spelling of the dialect works on
both. For the table of the v1 this matters: the names of the surface are
the **spelled** ones, `A_Clipboard` and not `Clipboard`.

The variables of the v1 are **137** names, and they were counted on the
binary on 2026-10-29: 134 answer `1` to `IsSet` and three do not answer at
all — `A_Is64bitOS` , `A_IsUnicode` and `A_PtrSize` . They are variables
all the same, and the reason the question does not answer is a detail of the
v1 that the probing has to know: **the parameter of the `IsSet` of the v1 is
an output variable** , and these three are built-in variables that cannot be
written. The binary says *"Parameter #1 invalid"* and opens a dialog — a
*runtime* error, which `/ErrorStdOut` does not cover. A direct read answers
them: `x := A_Is64bitOS` is `1` , `x := A_IsUnicode` is `1` and
`x := A_PtrSize` is `8` . A probe that asks `IsSet` and nothing else calls
the three of them "not variables" , and that would be wrong.

### 1.3 The surface: what is measured, and how

Two pieces of news, one good and one bad.

**The bad one: the v2 extraction does not repeat.** The surface of the v2 was
read **from inside the binary** : the tables of names and of handlers. In the
v1 **the names of the commands are not plain text in the executable** — a
search for `WinActivate` , `SendMode` , `MouseMove` , `Click` , `MsgBox` and
`Hotkey` by ASCII bytes (and by case) finds nothing; `SetTimer` and
`BlockInput` appear only inside the help messages. The table of commands of
the v1 is obfuscated in the binary.

**The good one: the direct probing is cheap and it is the truth.** In the v1,
an unknown function is a **load** error, not a runtime error — the `try` /
`catch` does not catch it. The question is then one per process:

```
x := StrLen("abc")     # it exists: the file receives "3"
x := Upper("abc")      # it does not exist: "Call to nonexistent function."
```

And the **kind** of each name comes out of three questions that the v1 asks
in three different ways (measured on 2026-10-29):

| Question | What answers |
|---|---|
| `Name, arg` | is it a **command**? — *"This line does not contain a recognized action"* says it is not |
| `x := Name()` | is it a **function**? — *"Call to nonexistent function"* says it is not |
| `x := IsSet(Name)` | is it a **variable** of the runtime? — 1 says it is, and 0 says it is not (a name that is made up gives 0) |

It is with these three questions that F1 fills the column of the kind of the
table of the v1. And it is for that reason that a probe by name is more than
a yes or no: the same name answers differently according to its shape.

**The whole surface of the v1 was measured on 2026-10-29: 421 names, in 22
seconds, on the workstation.** The box is not needed and was never needed: the
v1 is a Windows binary, and the probing never runs anything — the two
first questions are inside a dead branch, and the resolution of a name in the
v1 is static, so the answer is a *load* error and `/ErrorStdOut` prints it on
stdout. A `Click` that got out of hand would click wherever the mouse is, and
nothing got out of hand: nothing ran.

The three questions have to be read by the error that answers them, and there
are more of those errors than the first probe expected. The rules that came
out of the measurement:

| Question | The answer that says no | The answer that says yes |
|---|---|---|
| `Name, 0` | *"This line does not contain a recognized action"* | the script loads, or the error is a complaint about a parameter (*"Parameter #2 required"* , *"requires at least 2 parameters"*) |
| `x := Name()` | *"Call to nonexistent function"* | *"Too few parameters passed to function"* — the name resolved, the arity was wrong |
| `x := IsSet(Name)` | the value `0` | the value `1` |

Two things that the first probe got wrong, and that the table of the v1 does
not carry:

- **A name that is not a command is not always a name that the v1 does not
  know.** The statements of the language — `Catch` , `Break` , `Return` ,
  `Gosub` , `For` , `Switch` , `Until` — answer with their own message
  (*"CATCH with no matching TRY"* , *"Break/Continue must be enclosed by a
  Loop."*) , and a rule that only looked for *"does not contain a recognized
  action"* called all of them commands. The rule that separates them is the
  complaint about a **parameter** : a command says that its parameter is
  wrong, a statement says what it wants instead.
- **A statement that takes a body swallows the next line.** `Loop, 0`
  inside the dead branch takes the marker as its body, the body never runs,
  and the script ends without an error and without the marker. That silence
  is the signature of a statement (`Loop` , `While` , `IfEqual` , `IfExist` ,
  `IfWinActive` , `IfWinExist` ).

The probing has one more use: the `--surface` of the wave prints the state of
each **v2** name; the same mechanics, with the table of the v1 inside it,
prints the one of the v1 — and the probing is what fills that table.

### 1.4 The names: most of them match, the kind does not

Measured on the two binaries:

- **In the v1 and not in the v2** : `Asc` , `IsSet` , `Object` , `MenuGetName`
  , `Exception` .
- **In the v2 and not in the v1** : `StrUpper` , `StrLower` —
  *the v1.1.37.02 has no case function at all* : neither `Upper` , nor
  `StrUpper` , nor `Lower` , nor `StrLower*. ` Number `, ` Transform `, `
  StringReplace ` and ` ComValue
  ` are not of the v1.1 either (the first two are of v1.0 or of the v2). `
  HasProp ` and ` HasMethod ` are not either — and the v1 has no ` ObjHasProp`
  either (measured:
  *"Call to nonexistent function. Specifically: ObjHasProp(o, "a")"* ).
- `SetBatchLines` is the other way round: it is a name of the v1 that the v2
  does not have (measured on 2026-10-29: `SetBatchLines, 10` runs, and
  `SetBatchLines(10)` gives *"Call to nonexistent function"*).
- **The same name, another kind** — and this is where the gate hurts:

| Name | in the v1.1.37.02 | in the v2.0.19 |
|---|---|---|
| `StrSplit` | **function** | **command** |
| `FileExist` | **function** | **command** |
| `FileRead` | **command** (with an output variable) | **command** |
| `GetKeyState` | **command and function** | **command** |
| `StrReplace` | **function** (parameters in a different order) | **function** |
| `Clipboard` | **variable** (the text of the clipboard, since v1.1.35) | **does not exist** either |
| `A_Clipboard` | **variable** (the text; the alias of the `Clipboard`) | **variable** (the text) — the same name |
| `ClipboardAll` | **variable** (an object with all the formats) | does not exist |
| `Length` of the array | **method**: `a.Length()` | **property**: `a.Length` |

The clipboard is not a difference of names: the two dialects spell the text
`A_Clipboard` , and what the v1 has and the v2 does not is `ClipboardAll` ,
the object with all the formats. The name `Clipboard` is not a command and
not a function of the v1 — measured on the v1: `Clipboard, 0, hello` →
*"This line does not contain a recognized action. Specifically: Clipboard, 0, hello"*
, and `Clipboard()` → *"Call to nonexistent function"* ; measured on the
v2.0.19: `x := Clipboard` answers *"Warning: This global variable appears
to never be assigned a value. Specifically: Clipboard"*. It is a **variable**
of the v1 all the same, since v1.1.35: the documentation of the v1 says that
the `Clipboard` is the built-in variable of the clipboard and that the
`A_Clipboard` is its alias, and the binary agrees — the `global Clipboard` is
refused with *"Built-in variables must not be declared. Specifically:
Clipboard"* , and so is the `global A_Clipboard`. The two variables of the v1
were confirmed with the `IsSet` of the v1: `IsSet(A_Clipboard)`
and `IsSet(ClipboardAll)` give 1, and an invented name gives 0.

One table per dialect, with the column of the kind, is what solves this.

### 1.5 The classes: the v1 has no runtime classes

The v2 has eleven classes in the surface — `Map` , `Array` , `Buffer` , `File`
, `Func` , `Gui` (with `Gui.Control` , `Gui.ListView` , `Gui.Menu` and
`Gui.StatusBar` ) and `Hotkey` — and they are **runtime types** : the script
does not extend them, and the methods are the ones the runtime answers.

The v1 has none of those types. What it has are **objects** , and measured on
the binary on 2026-10-29 their `.__Class` is **empty** : the `File` of
`FileOpen` , the object of `Object()` , the literal `{a: 1}` and the object of
the variable `ClipboardAll` all give an empty `.__Class` . The `Object` of the
v1 is a **function** ( `Object()` runs), and the base object has no class
name. The array of the v1 is also an object: `a := [1, 2, 3]` gives
`a.Length()` = 3 (a **method** ) and an empty `a.__Class` .

The v1 has classes, but they are **of the script** : `class` , `new` , `this`
and `__Init` exist — measured: `class Probe { __Init() { this.x := 1 } }` ,
`new Probe` , and `o.x` is 1 — and there the `.__Class` is the name of the
class ("Probe"). What the v1 does not have is a class *surface* behind a name:
the `Map` and the `Array` of the v2 are objects with methods; in the v1 they
are objects with keys.

### 1.6 The surface of the v1, counted

The whole surface of the v1 was measured on 2026-10-29 and it is **421
names** : the 284 pages of the documentation of the v1 (the CHM of the zip) and
the 137 built-in variables of the page of the variables. The kind of each one
is the answer of the binary, and the table is in
`crates/ahk/reference/ahk-v1.1.37.02.surface.tsv` :

| Kind | Names | What they are |
|---|---|---|
| command | 166 | the names a script writes without parentheses — `Send` , `MsgBox` , `StringLen` |
| function | 50 | the names a script writes with parentheses — `StrLen` , `Format` , `DllCall` |
| command and function | 1 | `GetKeyState` , the only one that is both |
| variable | 137 | the built-in variables, `A_Clipboard` and the rest |
| directive | 29 | the `#NoEnv` and the rest |
| — | 38 | **not names** : the statements of the language and the topics of the docs |

**The 38 are not a failure of the measurement.** Each one carries the message
of the binary that says what it is, and they are of two kinds:

- **The statements of the language** (23): `Break` , `Catch` , `Continue` ,
  `Else` , `Finally` , `For` , `Gosub` , `Goto` , `IfBetween` , `IfEqual` ,
  `IfExist` , `IfExpression` , `IfIn` , `IfIs` , `IfWinActive` , `IfWinExist` ,
  `Loop` , `Return` , `Switch` , `Throw` , `Try` , `Until` , `While` . They
  are the grammar of the v1, and the table of the v2 does not carry them
  either — the grammar is F2's business, not the surface's.
- **The topics of the documentation** (14): `Block` , `Enumerator` , `File` ,
  `GetKey` , `GuiControls` , `ListView` , `LoopFile` , `LoopParse` ,
  `LoopReadFile` , `LoopReg` , `Math` , `SetExpression` ,
  `SetNumScrollCapsLockState` , `TreeView` . They are pages, not names.

**A page that carries two names is a blind spot of the list.** The `Run` and
the `RunWait` are documented together, in the same page of the CHM, and the
list of the names took one of them and left the other out. The `RunWait` was
measured on the binary on 2026-11-02 -- it is a command of the v1, with the
shape of the `Run` -- and the table carries it.

And the last one is a **directive that the docs still carry and that the binary
removed** : `#AllowSameLineComments` . The page of the CHM says *" [v1.1.09+]:
This directive was removed"* , and the binary says *"This line does not contain
a recognized action"* . The list of the docs is a **candidate** ; the binary is
the authority. (`#DllLoad` is the other way round: it is a directive of the
**v2** — *"New Directives"* , in the page of the changes — and the
v1.1.37.02 does not have it.)

**Against the v2** , the numbers are these:

| | The v1.1.37.02 | The v2.0.19 |
|---|---|---|
| the names of the surface | 383 | 725 entries, 644 names |
| of which the v2 has the name | 266 | — |
| only in this one | **117** | 214 (without the methods) |
| the same name, another kind | **21** | — |

The **117** are 48 commands, 13 functions, 27 variables and the 29 directives.
The **21** whose kind the v2 does not have are the ones the gate hurts the
most:

| Name | in the v1.1.37.02 | in the v2.0.19 |
|---|---|---|
| `Click` | command | **function** |
| `FileExist` | **function** | command |
| `FormatTime` | command | **function** |
| `GetKeyState` | **command and function** | command |
| `Gui` | command | method (of `Gui.Control`) |
| `Hotstring` | **function** | command |
| `Input` | command | method (of `Hotkey`) |
| `IsByRef` | **function** | method (of `Func`) |
| `IsLabel` | **function** | command |
| `LoadPicture` | **function** | command |
| `OnError` | **function** | command |
| `OnMessage` | **function** | command |
| `Random` | command | **function** |
| `RegDelete` , `RegRead` , `RegWrite` | command | **function** |
| `SetFormat` | command | method (of `Gui.Control`) |
| `Sort` , `SplitPath` | command | **function** |
| `StrSplit` | **function** | command |

**The v1.1 is a hybrid, and the rule of the hybrid is the name.** The old name
is a **command** and the new name is a **function** , and the two live in the
same dialect:

| The old name | The new name |
|---|---|
| `StringLen` (command) | `StrLen` (function) |
| `StringMid` (command) | `SubStr` (function) |
| `StringGetPos` (command) | `InStr` (function) |
| `StringReplace` (command) | `StrReplace` (function) |
| `StringSplit` (command) | `StrSplit` (function) |
| `StringTrimLeft` (command) | `Trim` (function) |
| `SetFormat` (command) | `Format` (function) |

**The arity is measured too, and it comes out of the same binary.** The count
of the parameters is what the binary complains about, and the complaint is a
different one for each case: *"requires at least N parameters"* and *"Too few
parameters passed to function"* say that the count is short, *"requires at
most N parameters"* and *"Too many parameters passed to function"* say that it
is long, and every other answer — a value that is not accepted, a name
that is not a variable, a label that does not exist — says that the
count was fine. A scan of the count gives the two bounds: `StrLen` is 1..1 ,
`InStr` is 2..5 , `SubStr` is 2..3 , `RegExReplace` is 2..6 , `FileRead` is
2..any , `MsgBox` is 0..any.

**The commands have no upper bound, and that is a fact of the v1.** The
parameters that a command does not need are ignored: `MsgBox, a, b, c, d, e`
loads. The table carries 255 for those, the way the table of the v2 carries
it for a function that takes any number of them.

This is what a dialect that is a **hybrid** looks like from the inside: the
v1.1 kept the command of the v1.0 and put the function of the v2 beside it.
The tree carries the names of the v2; the table of the v1 has to carry
**both** . And the tree has no row for a name that is a command *and* a
function — the table of the v2 already carries a name twice when it is
two things (`SendMode` is a command **and** a variable of the v2) , and the
table of the v1 follows that rule.

---

### 1.7 The errors of the v1, and how they are told apart

The v1 tells an error in two ways, and the tree has to tell them the same way.
Measured on 2026-10-29, on the two binaries.

**A load-time error comes on stderr.** The script does not parse, the binary
writes the complaint on stderr with the exit code 2, and the line of the
failure is in it: `* (2) : ==> Missing ")"`. Nothing of the script runs.

**A runtime error is caught by `OnError`.** The v1 registers a callback —
`OnError("AhkErrorHandler")` , with the name of the function in quotes; the
old form, a label, is refused. The callback is called only for an error that
would otherwise show the dialog, and **it cannot be called for a load-time
error**: `OnError` cannot run before the script has loaded.

The callback receives **the thrown value**. When the value is an object, it
carries `Message` , `What` , `Extra` , `File` and `Line` , and the `Line` is
the line of the script that failed. The handler written *after* the script
gives the exact line, because the functions of a v1 script are hoisted:
measured, `ComObjCreate("NoSuch.ProgID.1.2")` on the line 2 reaches the
handler with `line=2`.

When the value is **not** an object — a thrown string — it carries no message
and no line: the value itself is the error, and the tree has nothing else to
say about it.

**The v1 is lenient, and the tree must be too.** These are not errors in the
v1, they are blank results: a variable that was never assigned, an index out
of range, a property that does not exist, and a division by zero. The
documentation says it of the last one: *"Division by zero yields a blank
result"*. Measured: `x / 0` in the v1 gives no error at all, while the same
line in the v2 throws *"Divide by zero."*.

**The v2, for the comparison.** The v2 has no handler that can be registered
before the script runs: the only way to take a runtime error is a `try`
around it. The object that is caught carries `Message` , `Line` and `Stack`
(the `What` is empty), and it has no methods — `e.GetClass()` opens the
error dialog itself. A load-time error of the v2 also comes on stderr, with
the exit code 2.

**The tool that measured this** is `ahk_error` , in the lab server
(`../yoko-rust`) : it runs the script under the real binary and returns what
it says, without waiting for the window to be read.

## 2. The gate: three shapes, and the one that is chosen

### (a) Compile-time feature — `--features v1`

One binary per dialect. Zero runtime cost, and the v1 does not even enter the
binary of the v2. In exchange: changing dialect means recompiling, the server
of the lab needs both builds, and the box has to keep them.

### (b) Runtime flag — `--dialect=v1` ← **the recommended one**

One binary, the two dialects. The dialect is a **value** that the parser and
the runtime consult; the v2 is the default and the path of the v1 stays inert
for a v2 script. The box and the MCP change dialect without recompiling.

It costs: a predicate in the parser and another in the runtime, and the two
tables in the binary.

### (c) Separate crate — `ahk-v1`

The surface of the v1 stays isolated. In exchange: the runtime, the
interpreter, the classes and the `Desktop` duplicated and kept in parallel —
twice the work forever, and the tree stops having *one* table and *one*
dialect.

### Why (b)

Three reasons, in the order of weight:

1. **The dialect is a property of the script, not of the binary.** The lab
   runs v1 and v2 scripts on the same day; with (a) they would be two binaries
   to keep on the box.
2. **The surface is already data.** `builtins.rs` is a table; `variables.rs`
   is a table; the classes are a list. A second dialect is, above all,
   **a second set of rows** — and the table is exactly what the architecture
   of the tree already asks for.
3. **The runtime cost is a predicate.** The `Dialect` is an
   `&'static dyn Dialect` ; the parser asks it once per statement and the
   runtime once per operation that depends on the dialect.

(b) has a cost that (a) does not have:
**the two grammars are always compiled** . It is the price of the legacy
grammar (the `Cmd, a, b` statement and the hotstring-statement `::btw::` ),
which is the most expensive piece of the plan and which would be paid anyway.

---

## 3. The shape of the code

### 3.1 The trait

```
/// The dialect that the runtime speaks.
///
/// The v2 is the dialect of the Serialize: the `V2` is the one that the
/// runner uses by default, and the `V1` is only chosen when the script
/// asks for it.
pub trait Dialect {
    /// The name of the dialect, for the report.
    fn name(&self) -> &'static str;
    /// The names that the dialect carries.
    fn builtins(&self) -> &'static [Builtin];
    /// The variables that the dialect carries.
    fn variables(&self) -> &'static [Variable];
    /// The class types that the dialect carries.
    fn classes(&self) -> &'static [&'static str];

    /// The legacy command: `Msg, text`.
    fn legacy_commands(&self) -> bool;
    /// The `=` assigns (and does not compare).
    fn assign_with_equal(&self) -> bool;
    /// The `=` compares without a distinction of case.
    fn equal_is_case_insensitive(&self) -> bool;
    /// The `%name%` is the environment variable.
    fn percent_is_environment(&self) -> bool;
    /// The variables of a function are local by default.
    fn scope_is_local(&self) -> bool;
    /// The hotstring is a statement: `::btw::`.
    fn hotstrings_are_statements(&self) -> bool;
}
```

The `Runtime` comes to carry `dialect: &'static dyn Dialect` (beside the
`desktop` ), and the `Parser` receives it in the constructor. The `ahk-cli`
gains `--dialect=v1|v2` , with the v2 by default.

### 3.2 The table of the built-ins gains the shape of the parameters

The v1 has three kinds of parameter: **variable** (the name of a variable of
the script), **output variable** (the `n` of `StringLen, n, x` ) and
**expression** . The v2 has two: value and reference. One column solves the
two:

```
/// The shape of a parameter.
pub enum Parameter {
    /// An expression: the value reaches the handler.
    Value,
    /// The name of a variable of the script: the handler receives the reference.
    Variable,
    /// The name of a variable of the script where the handler writes.
    Output,
}
```

The row of today is `Builtin { name, kind, min, max, variant, handler }` — the
`variant` is the flavor of the handler ( `Ceil` and `Floor` are the same
handler with two flavors). The new column goes beside it:
`parameters: &[Parameter]` . The current `reference_parameters` —
`builtins.rs:457` — comes to be read from that column. The table of the v1
fills the column with the shape that the measured v1 has.

### 3.3 What does **not** change

The `Desktop` and its ten traits, the `ahk-box` , the report, the guard rails,
the `File` class, the `DllCall` , the raven, the box.
**The v1 runs on the same engine** — it is what makes the plan possible
instead of it being a rewrite.

### 3.4 The reference tables

They come to be per dialect, in `crates/ahk/reference/` :

```
ahk-v1.1.37.02.surface.tsv      # the names of the v1, with the kind and the arity
ahk-v1.1.37.02.pending.tsv      # the names of the v1 that the tree does not carry
ahk-v2.0.19.surface.tsv         # what is already there
ahk-v2.0.19.pending.tsv
```

And the parity test ( `tests/surface.rs` ) comes to run
**against the chosen dialect** : for the v2 it is what it is today; for the v1
it is the same mechanics with the other table.

---

## 4. The phases

Each phase has a **verifiable** result, and none of them touches the behaviour
of the v2.

### F0 — The gate, changing nothing

**Content.** The `Dialect` trait, the `V2` (which wraps the tables of today
without moving them), the parser and the runtime asking it, the `--dialect=v2`
in the runner, and the table of the parameters in the table of the built-ins
(filled with `Value` / `Variable` , which is what the v2 already does).

**Result.** The suite stays green without a change of body — the 622 cases of
the commit `6def58f` (the wave of 2026-10-29 changed them too, and the number
only goes up); the `--dialect=v1` still answers
*"this runtime does not speak that dialect"* .

**Risk.** Low. It is the phase that proves the gate costs nothing to the v2.

### F1 — The surface of the v1 (data)

**Content.** The table of the names of the v1: the list of the documentation,
the kind of each one, the shape and the arity of each parameter —
**verified on the binary** , with the probing of one process per name (§1.3).
The variables of the v1 (§1.2). The classes of the v1: **none**
(§1.5) — the v1 has no runtime classes, and what it has are objects and the
script classes. The pending table of the v1, with the reason of each name that
is not carried — and this is the first pending table that is not empty (§0).

**Done (2026-10-29).** The list of the names and the kind of each one: **421
names, 383 of them of the surface** , in
`crates/ahk/reference/ahk-v1.1.37.02.surface.tsv` (§1.6). The 29 directives of
the v1 are measured too, and the one that the documentation still carries and
that the binary removed is written down.

The **arity** of every name is measured as well, on the same day and with the
same probe: the columns `min` and `max` of the table are filled.

The **shape of the parameters** is measured too, on the same day and with the
same probe: the column `shape` is filled. The v1 has three shapes and the v2
has two (§3.2) , and the shape of a name is what decides whether the handler
writes into a variable of the script or returns a value. The binary says
that a parameter is the *name* of a variable when the value is not one (the
error is *"The following variable name contains an illegal character"*) , and
the documentation says whether that name is written into or read from ---
`OutputVar` is written into, `Var` is read.

**What the measurement showed, on 2026-10-29:**

- **A command is checked at load time.** A parameter that wants a name refuses
  a quoted string, and the complaint is on stderr. The probe gives each
  position a text of its own, so the complaint says which one it is, and it
  goes on with the next. Nothing runs: the question is asked in a dead branch,
  and the resolution of a name in the v1 is static (§1.3).
- **A function is checked at runtime**, so the shape of a function is the
  documentation's alone, and the measurement cannot confirm it.
- **The names that the commands read are the ones the convention of the v1
  calls `InputVar`** : `StringLen` , `StringMid` , `StringGetPos` ,
  `StringReplace` , `StringSplit` , `StringTrimLeft` , `StringLeft` ,
  `StringLower` , `SplitPath` , `Sort` , `IfInString` — and the shape of the
  command is `Output,Variable` where the function of the same family is
  `Value`.
- **The output variable of a command is not always the first parameter** :
  `SplitPath` reads one and writes five, `FileGetShortcut` writes seven,
  `MouseGetPos` writes four and `WinGetPos` four.
- **Seven commands take no parameter at all** (`Edit` , `EnvUpdate` ,
  `KeyHistory` , `ListHotkeys` , `ListVars` , `Reload` , `WinMinimizeAll` ),
  and their shape is empty.
- **A directive has no shape.** It is not a row of the built-in table, and
  every parameter of a directive is a value: there is no name of a variable
  to read from and no variable to write into. The 29 directives carry an
  empty shape, and the seven commands above carry an empty shape too ---
  **211 rows of the 384 carry a shape**: the 160 commands that take a
  parameter and the 51 functions.
- **The page of a name may carry the syntax of another name.** The
  documentation puts the directives on the pages with a leading underscore
  (`_Hotstring` , `_IfWinActive` ) , and the page of a command may carry
  the syntax of the command that sits beside it: the page of `PostMessage`
  carries the line of `SendMessage` , and the page of `Progress` the one of
  `SplashImage` . The probe asks the page for the syntax of the name it
  wants, and the row of `PostMessage` has eight parameters and not the nine
  of that other line.

The **pending table of the v1** is written as well, in
`crates/ahk/reference/ahk-v1.1.37.02.pending.tsv` : **72 names** that the
tree does not carry yet — 23 commands, 6 functions, 15 directives and 28
built-in variables. The names that are about a Windows API are not in it: the
tree carries the Windows-shaped names of the v2 and refuses them with the
reason, and the v1 will do the same.

**Left.** Nothing of the data: the shape, the arity, the kind and the pending
table are all there. What is left is the *use* of it --- the table of the
built-ins of the v1, and the `--dialect=v1` of the command that prints the
surface (§5).

**Result.** `ahk --surface --dialect=v1` prints the list with the state of
each name — the command that the wave of 2026-10-29 built for the v2, with one
dialect more; the number of "really carried" ones is visible in each commit.

**Risk.** Medium: it is mechanical work, but it is work. And it is the phase
that F2 depends on.

### F2 — The grammar of the v1

**Content.**
- The `Cmd, a, b` statement (the `parser.rs:462-499` is v2-only) and the
  disambiguation with the comma.
- The shape of the parameters: variable, output variable, expression.
- The `=` that assigns ( `parser.rs:353-361` and `:501-504` already read `=`
  and `:=` for the same arm — it is almost half of the work).
- The one-statement `if (x)` and the `else` .
- The *labels* and the `goto` .
- The hotstring as a statement ( `::btw::` ).
- The `Hotkey, ^a, Label` with the target in a label.
- The directives of the v1 ( `#MaxThreads*` , `#NoEnv` , `#Persistent` ,
  `#SingleInstance` , `#UseHook` , `#HotIf` , …).

**Result.** A v1 script from the documentation runs in the runner:
`MsgBox, text` writes the message, `StringLen, n, s` fills `n` , `if (a = b)`
compares without distinguishing case, `goto` jumps. (The `Msg` is not a
command of the v1 — measured on 2026-10-29:
*"This line does not contain a recognized action. Specifically: Msg, hello"* .
The `MsgBox` is, and it opens a window.)

**Risk.** High — it is the most expensive phase. The `=` is the hard point:
the same token is assignment or comparison depending on the position, and the
v1 uses the two in the same place ( `if (a = b)` compares; `a = b` assigns).

### F3 — The semantics of the v1

**Content.**
- The `=` of the v1 compares without distinguishing case and the `==`
  distinguishes (the opposite of the v2) — `runtime.rs:2798-2809` .
- The `%nome%` is the environment variable, not the one of the script (the
  `#NoEnv` is the switch).
  **The tree today interpolates `%nome%` with the variable of the script** (
  `runtime.rs:3073` ) — it is the rule of the v1 with the sense of the v2, and
  the plan corrects it per dialect.
- The variables of a function are **local** by default, in the v1 as in the v2 ( `runtime.rs:1648-1661` creates a local one): measured on the binary of v1.1.37.02 on 2026-11-01, a variable that a function writes is not seen outside of it, and the word `global` is what makes it so. The plan said the opposite, and the binary says otherwise.
- The text is everything: the v1 has no integer and float apart, and the
  `Value` of the runtime has them ( `runtime.rs:57-82` ).
- The classes of the v1: `Object` and the literal `{}` .
- **The error of the v1** : the `OnError` of the v1 and the object that it
  receives, the line that it carries and the errors that it cannot see
  (§1.7) . The v2 catches with a `try` ; the v1 registers a callback, and
  the callback never sees a load-time error.

**Result.** The `=` and the `%nome%` of a v1 script behave as the measured v1,
and a v2 script feels nothing.

**Risk.** Medium-high: the value model and the scope model are structural.

### F4 — The box

**Content.** The v1 uses the same capabilities, with the names of the v1:
`Send` , `Click` , `PixelGet` , `WinExist` , `FileRead` , `FileAppend` ,
`IniRead` , `StrReplace` , `StrLen` , `Format` , `RegExMatch` . The
capabilities that the v1 has and the v2 does not ( `Control*` , `Win*` , the
registry) are refused with the reason, as the v2 does.

The clipboard changes shape, not path: in the v1 the text is the variable
`A_Clipboard` and the complete object is the variable `ClipboardAll` ; in the
v2 the text is the variable `A_Clipboard` too -- the same name -- and
`ClipboardAll` does not exist. The path is the same on both sides -- the
`wl-paste` and the `wl-copy` of the box, and **not** the protocol of the
compositor (§27 of the notes).

**Result.** A v1 script that reads a pixel, writes a file and sends a key
works on the box.

**Risk.** Low: it is the layer of the `Desktop` , which already knows how to
do almost all of this.

### F5 — The verification

**Content.**
- The parity test per dialect, with the two tables.
- The suite of the v1 ( `crates/ahk/tests/v1/` ) mirroring the one of the v2.
- The `--surface` per dialect, in text and in JSON.
- The measurement on the box: the cases of the v1 through the runner, like the
  35 of the v2 (2026-10-29).

**Result.** `ahk --surface --dialect=v1` says what is missing, and the number
does not get worse without being seen.

### The state of the phases — 2026-11-02

| Phase | State | What is in the tree |
|---|---|---|
| **F0 — the gate** | Done | The trait `Dialect`, the `V1` and the `V2`, the column `parameters` of the table of the built-ins and the flag `--dialect` of the runner. Commit `9d57bf8`. |
| **F1 — the surface** | Done | The table of the v1 (`crates/ahk/src/v1/table.rs`), the call in the shape of the v1, and the pending table of the v1. Commit `48ef559`. |
| **F2 — the grammar** | Done | The `=` that assigns, the line of a command that continues when the line that comes begins with an operator, the hotstring as a statement, and the directives of the v1. Commit `12331ab`, corrected by the block of the body of the action. |
| **F3 — the semantics** | Done | A variable with no value is blank, a division by zero is blank, `%name%` is the variable of the script, and the `+` of a text is a concatenation. Commit `50710cd`. |
| **F4 — the box** | Done | The runner with `--dialect=v1` on the box (192.168.0.40), with the `PixelGetColor`, the `MouseGetPos`, the `FileRead`, the `StrLen`, the `IsSet`, the `IsFunc` and the `Object` against the real machine. |
| **F5 — the verification** | Done | The parity test per dialect (`tests/surface.rs`), the suite of the v1 (`tests/v1.rs`, 187 cases), and the `--surface --json` for the CI. |

**What is left.** One name of the pending table of the v1: the
`SetBatchLines`, and it is a decision of the plan (§5, the threads of the
v1). The `IfInString`, the `IfNotInString`, the `IfMsgBox` and the `Else`
were carried by the block of the body of the action (2026-11-02), the
`StringGetPos`, the `StringReplace` and the `StrReplace` by the block of the
searches (2026-11-02), the call of a function by the block of the call
(2026-11-02), the `ErrorLevel` by the block of the `ErrorLevel`
(2026-11-02), the name of a function by the block of the name of a
function (2026-11-02), and the commands of the machine by the block of the
machine commands (2026-11-02), and the objects by the block of the objects
(2026-11-02), and the name of a class by the block of the name of a class
(2026-11-02), and the grammar of a class by the block of the grammar of a
class (2026-11-02), and the scope of a function by the block of the scope of
a function (2026-11-02), and the name that begins with a number by the block
of the name that begins with a number (2026-11-02), and the number that
keeps the text that was written by the block of the number (2026-11-02), and
the path of the class of the `new` by the block of the path (2026-11-02), and
the parameter of a command by the block of the parameters (2026-11-02), and
the pid of the process by the block of the pid (2026-11-02), and the code of
the failure of a native call by the block of the code (2026-11-02), and the
patterns of the files by the block of the patterns (2026-11-02), and the number of the parameters of a command by the block of the arity (2026-11-02), and the arithmetic of the v1 by the block of the arithmetic (2026-11-02), and the environment of the v1 by the block of the environment (2026-11-02), and the last parameter of a command by the block of the rest of the line (2026-11-02), and the name of a variable by the block of the name of a variable (2026-11-02), and the value of the parameter of a command by the block of the values (2026-11-02), and the marks of the name of a variable by the block of the marks (2026-11-02), and the sub-command of a command by the block of the sub-command (2026-11-02), and the label that a command names by the block of the label (2026-11-03), and the goto by the block of the goto (2026-11-03), and the `GetKeyState` by the block of the key state (2026-11-03).

**The parameter of a command of the v1.** The v1 checks the value of the
parameter when the script loads, and the script does not run at all: the
`Exit, 3` that comes before a command with a bad parameter does not run. The
complaint of the v1 is "Parameter #N invalid." with the text that the script
wrote -- the text of the parameter, and not its value -- and the last
parameter that the command reads takes the whole rest of the line. The rule is one
hook of the dialect (`the_refusal_of_an_invalid_parameter`), the rules are
in `crates/ahk/src/v1/parameters.rs`, and the commands that the v1 does not
check have handlers of their own. The name of a key and the name of the
label of the `SetTimer` are of other families, and they are not carried.

**The sub-command of a command of the v1.** The command whose parameter
names a sub-command refuses the sub-command that the v1 does not know when
the script loads: "Parameter #N invalid." with the text of the parameter
after it. The sub-command that the v1 knows has rules of its own -- the
parameter that it does not take has to be blank, the parameter that it
needs may not be blank, and the parameter that it does not take and the
ones after it have to be left out -- and the complaint of the v1 is one of
three families. The sub-command may carry the name of the window before it,
and it may be a list of options that begins with a sign. The rules are in
`crates/ahk/src/v1/sub_commands.rs`, and the parameter that may not be
blank is another hook of the dialect
(`the_parameter_may_not_be_blank`). The block of the sub-command
(2026-11-02) carried it, and the differential sweep of the v1 went from
88 divergences of the load to 2: the `GroupAdd`, whose label the parser
does not know yet.

**The label that a command of the v1 names.** The command whose parameter
names a label refuses the label that is not there when the script loads:
"Target label does not exist.", and it names no text. The `GroupAdd` names
it in its fourth parameter -- the label of the subroutine that the group runs
when it has no window -- the `Hotkey` in its second -- the action of the key
-- and the `SetTimer` in its first -- the body of the timer. The label may be
written anywhere in the script, and the name of it does not know its case;
the text that is empty names no label, and the percent sign is the
substitution of the v1, and the label that comes out of it is not checked at
the load. The action of a hotkey and the body of a timer may be a function of
the script, and the name of the function is accepted there -- the function
with arguments is not -- and the `GroupAdd` names a label and nothing else.
The refusal comes after the other refusals of the load: the label that is
written twice and the call of a name that is not a function are refused
before it. The rules are in `crates/ahk/src/v1/labels.rs`, the hook of the
dialect is `the_label_of_the_command`, and the check is at the end of the
parse, because the names of the labels of the script are known only then. The
block of the label (2026-11-03) carried it, and the differential sweep of the
v1 has no divergence of the load any more.

**The goto of the v1.** The goto names a label, and the label that is not
there is refused when the script loads: "Target label does not exist." with
the label after it. The jump that may not be made is refused too: the label
of a block that does not enclose the jump is "A Goto/Gosub must not jump into
a block that doesn't enclose it.", and the jump that leaves a function is
"Cannot jump from inside a function to outside.". The block of a label is
the chain of the blocks that enclose it, and the label may be the target of
the jump when its chain is the beginning of the chain of the jump. The
refusal of the goto comes after the other refusals of the load, and the
refusal that comes first in the script is the one that the v1 says. The
rules are in `crates/ahk/src/v1/labels.rs`, the hook of the dialect is
`the_goto_is_refused_with_the_labels`, and the complaint of the jump that
the runtime refuses is the one of the dialect too
(`the_refusal_of_a_jump_into_a_block`). The block of the goto (2026-11-03)
carried it.

**The `GetKeyState` of the v1.** The name is two things in the v1: the call of
an expression is the function -- it takes one parameter and it answers the
state of the key -- and the command statement is the command: it writes the
letter D or U into the variable of the caller. The command has no maximum,
because the v1 ignores the parameters that a command does not need, and the
table says 255 when a built-in takes any number of them. The name of the key
is checked when the script loads, with the complaint of a parameter:
"Parameter #2 invalid." The name is one of the keys of the list of the
documentation of the v1, a single character, or the spelling of a virtual key
or of a scan code, and the number of the code is the byte that it ends in.
The function answers an empty text for a name that is not a key. The names
are in `crates/ahk/src/v1/keys.rs`, the rule of the parameter is in
`crates/ahk/src/v1/parameters.rs`, and the two forms of the name are two rows
of the table of the v1. The block of the key state (2026-11-03) carried it.

**The `if` of the v1** (2026-11-04). The `if` of the v1 is an expression,
and the condition of it is the expression of the line: the `if (1) x = 5`
compares the text "1" with the number 5 -- the concatenation of the v1 is a
part of the expression -- and the condition is false. The text that the
expression does not reach is the tail of the `if`, and the v1 parses it and
does not run it: the `if (1) FileDelete, marker.txt` leaves the file where it
is. The body of the `if` is the line that comes next, or the block, and the
tail that ends with an operator takes the line that comes with it, the way an
expression of the v1 continues.

The `else` of the v1 takes no parameter, and the text that comes right after it
is the body of it: the `else FileAppend, hello, *` runs the command and not a
parameter. The `else` that has no `if` before it is refused at the load with
"ELSE with no matching IF", and the `else` that follows anything that is not
the body of an `if` is refused the same way. The text that comes after a comma
is a parameter, and it is named: the `else, something` says "This line does
not contain a recognized action.".

**The legacy `if` of the v1** (2026-11-04). The legacy `if` is the one that
compares a variable with a value -- the `if x = 1`, the `if x != 1` and the
`if x < 1` -- and the right side of it is the text of the rest of the line,
and not an expression: the `if x = 1 + 1` compares the value of `x` with the
text "1 + 1", and the condition is false. The text goes through the legacy
substitution, and the `=` and the `!=` of it ignore the case. The `==` of the
v2 is not an operator of the legacy `if`: the `=` is the operator and the
second equal sign is a part of the text.

The legacy `if` has no One True Brace: the brace of the line of the condition
is a part of the text, the block is not opened, and the brace that closes it
is refused with "Unexpected \"}\"". The block that comes on the line that
comes is the body of it. The v1 knows the `<>`, which is the `!=` of the v2,
and the v2 does not know it.

**The forms of the legacy `if` and the tail of the `if`** (2026-11-04). The
legacy `if` has three forms that are not the comparison of a variable with a
value -- the `if x between low and high`, the `if x in a, b` and the `if x
contains a` -- and the `not` may come before the word of the form. The texts
of them are the texts of the script and not expressions, and they go through
the legacy substitution. The `in` compares as text and never as a number, the
list is cut at the commas with the spaces around them inside the item, and
two commas in a row are one comma in the item. The `contains` asks whether
the text of the value has the text of the item in it. The `between` takes the
bounds inclusively, compares as a number when the value and the two bounds
are numbers, and cuts the text at the first word `and`. The three forms and
the `=` of the legacy `if` follow `StringCaseSense`.

The tail of the `if`, when it ends with an operator, takes the line that comes
with it, and the `if` has no body then: the statement that comes after the
tail is a statement of its own, and it runs whether the condition is true or
false. The `else` that comes right after the tail is the other side of the
`if`, and the body of it runs whether the condition is true or false; the
`else` that comes with a statement between it and the tail is refused with
"ELSE with no matching IF".

**The truth of the v1** (2026-11-04). The text of a condition of the v1 is a
number when the syntax of the v1 accepts it, and the condition is false when
that number is zero: the "00" is false, and the "0.0" is false. The v2 keeps
the text as it was written, and the "00" is true there.

The `!` and the `&&` and the `||` of the v1 see the text that was written
between quotes as a string, and a string is true when it is not empty: the
`!("00")` is zero. The value of a variable is a number when it can be one, so
the `!x` with `x` holding "00" is one. The `&&` and the `||` of the v1 answer
one and zero, and not the operand: the `"a" && "b"` is 1.

**The number on the left of the legacy `if`** (2026-11-04). The names of the
v1 may begin with a number, and the number that comes first in the legacy
`if` is the name of a variable: the `if 1 = 1` compares the variable "1"
with the text "1", and the variable "1" has no value, and the condition is
false. The number is a name when the text of it has no point: the `if 1.0 =
1.0` is the form of the expression, and the `if 0x0 = 0x0` is the legacy
form.

The variable that is named "0" holds the text "0" when it has no value, and
the `if 0 = 0` is true because of it, and the `if 0 < 0` is false. The
writing of it wins: the `0 := 9` turns the `if 0 = 0` into a false
condition. The `%0%` of a command reads the same variable.

**The `if` and the `while` inside the line of the condition** (2026-11-04).
The v1 accepts the `if (0) if (1) x := 1`, and the construct that comes
inside the line has no body of its own: its condition is a part of the
condition that holds it, and the line that comes after is the body of the
`if` that holds it. The assignment that comes after it is not a body of it:
the `if (0) if (0) x := 1` writes the variable "x".

The condition of the `if` that comes inside the line may be written without
parentheses: the `if (0) if 1 = 1` is the form of the expression there, and
the comparison is true. The `while` comes inside the line the same way.

The truth of the whole is the truth of the text of the line, because the
`if` of the v1 reads the line of the condition as one expression and joins
the parts of it: the `if (0) if (0)` is false, because the text of it is
"00", and the `if (0) if (1)` is true, because the text of it is "01".

**The number that opens a statement** (2026-11-04). The names of the v1 may
begin with a number, and the number that opens a statement is the name of a
variable: the `0 := 9` writes the variable "0", and the `0 = 9` writes it
too, and the `0++` is the increment of it, and the `++0` is the increment of
it as well. The number is a name only where the statement may write: the
`x := 0 + 3` is the sum of two numbers, and the variable "0" keeps the value
it had. A statement that is only the name holds no action: the `0` alone is
refused with "This line does not contain a recognized action." and the line
is named with it.

The increment of the v1 reads the value that is there as a number, the way
the operators that write a variable read it: the text that is not a number
is the number zero, and the `a++` of the "abc" is one, and the `a++` of a
variable that has no value is one too.

**The items that come after the comma** (2026-11-04). The line of the
condition of the `if` of the v1 is one expression, and the items that come
after the comma are evaluated: the `if (0) x := 1, y := 3` writes the
variable "y". They are evaluated whether the condition is true or false, and
the value of them does not count for the truth: the `if (0) x := 0, y := 3`
does not run the body of the `if`.

The items are expressions, and the command of the v1 is not one of them: the
`if (0) x := 1, FileAppend, A1, done` does not write the file, and the `if
(0) x := 1, ExitApp 5` does not leave the script. The name of the command is
a variable when it is read as an expression, and the variable has no value.
The call of an item runs: the `if (0) x := 1, x := StrLen("abc")` writes the
variable "x" with the value 3.

The item takes the line that comes with it when the expression continues
there: the `if (0) x := 1, y := 3` with a `+ 4` under it writes the variable
"y" with the value 7.

**The messages of the load** (2026-11-04). The v1 names the bracket that is
not closed and nothing else: the `x := (1` is a "Missing \")\"" and the line
of the script is not under it. The v2 names the line of the script under
the name of the bracket. The bracket that closes nothing is an "Unexpected
\")\"" with nothing under it either, and the text that is not closed is a
"Missing close-quote" that names the text that was read -- the `"abc` -- and
not the line of the script.

**The assignment that has no value** (2026-11-04). The `y :=` of the v1
writes the empty text, and the line that comes is a statement of its own:
the `y :=` with a `3 + 4` under it is a line that holds no action. The v2
continues the assignment on the line that comes, and it writes the variable
"y" with the value 7.

The `:=` at the end of the tail of the `if` is an operator of the v1, and
the tail takes the line that comes with it. And the line that comes with
the tail is an action: the `if (0) x := 1, FileAppend, A, *` with a `B`
under it is a "This line does not contain a recognized action. Specifically:
B".

**The values that are side by side** (2026-11-04). The v1 does not ask for
the space that tells two values apart: the `x := "a"5` is the text `a5`, and
the `x := 5 3` is the text `53`. The v2 asks for the space, and it refuses
the line with "Missing space or operator before this.".

The quote of the v1 escapes itself: the `x := "a""b"` is the text `a"b`.

**The name that carries a quote** (2026-11-04). The name of a variable of the
v1 may not carry a quote. The `x := a"b"` is refused at the load with "The
following variable name contains an illegal character:" and the name of it --
the `"a"b""`. The name is the symbol that comes with the word: the `a"b"x`
is the symbol `a"b"x`, and the `a"b", y` is the symbol `a"b"`.

The refusal is where the name is a variable: the target of an assignment, the
value that is read and the member. The name of a command of the v1 is a text,
and the `FileAppend, a"b", *` runs; and the name of a function is not a
variable, and the `a"b"(1)` is a "Call to nonexistent function.".

The number that is the name of a variable carries the quote too: the `x := 5"a"`
is the symbol `5"a"`. The number with a point is a number and not the name of
a variable, and the quote is not allowed there: the `x := 5.5"a"` is a "This
character is not allowed here. Specifically: "a"".

**The words that are names** (2026-11-04). The words of the v1 are not
reserved: the `while := 5` writes the variable "while", and the `x := while`
reads it. The words that keep their meaning in the v1 are the operators
(`and`, `or` and `not`), the values (`true` and `false`) and the `if`, that
is a command of the v1. The `true := 5` is refused with "Not allowed as an
output variable. Specifically: true", and the `is`, the `as`, the `in`, the
`contains` and the `super` are names of the v1.

**The count of the `Loop`** (2026-11-04). The count of the `Loop` of the v1
is the text that the script wrote, and the loop is skipped when the text is
not a number. The `loop n` is skipped, and so are the `loop 1+1` and the
`loop 2.7`. The hex is a number: the `loop 0x3` is three turns. The `%` and
a space force the expression: the `loop % Count + 1`. And the `%name%` is the
value of the variable of the script: the `loop %n%` is the value of `n`.


**The illegal character of an expression** (2026-11-04). The v1 says "The
leftmost character above is illegal in an expression." and names the symbol
that begins at the character -- the `;c` of the `x := 1;c`. The v2 says
"Illegal character in expression." and names the rest of the expression.

The symbol ends at the space and at the punctuation: the `;` of the `x := 1;
y := 2` is the symbol, and the `;c` of the `x := 1;c,d` is one. The point, the
semicolon and the percent sign are a part of it: the `;c.` of the `x := 1;c.`
is the symbol, and the `;"a"` of the `x := 1;"a"` is one.

**The single quote is not a text** (2026-11-04). The single quote is not a
delimiter of a text in the v1: it begins a symbol, and the symbol that begins
with it is refused where an expression wants an operand -- the `x := 'abc'` is
refused with "The leftmost character above is illegal in an expression.
Specifically: 'abc'".

The symbol ends at the space and at the punctuation, like the symbol of the
illegal character: the `x := 'a b'` names the `'a`, and the `x := 'a''b'` names
the `'a''b'`.

Where the symbol is a name, the refusal is the one of the name: the `'abc' :=
5` is refused with "The following variable name contains an illegal
character: "'abc'". The statement that holds the symbol alone is a line that
holds no action, and the name of a command of the v1 is a text: the
`FileAppend, 'abc', *` runs and writes `'abc'`.


**The `for`** (2026-11-04). The `for` of the v1 walks the keys of an array:
the `for i in [10,20]` gives 1 and 2 to the `i`, and the `for k, v in
[10,20]` gives the key and the value. The name that gets the key is the
first one that is not empty, and the rest of the line after the source is
dropped: the `for i in 1,2,3` walks nothing and it does not fail.

The `in` of the v1 is the name of a variable, and the parser of the v2 asks
for a word. The v1 names the word that is missing: "This \"For\" is missing
its \"in\".".

And one corner of the `for`: the list between parentheses is not of the v1.
The v1 reads the name as the symbol "(i" and refuses it with "The following
variable name contains an illegal character:", and the tree reads the
parenthesis and answers the loop.

**The escape of the v1** (2026-11-04). The escape of the v1 is the backtick,
and the character that follows it is what it stands for: the `a`b` is the text
"a<backspace>". The set of the escapes is the one of the v1 and not the one of
the v2: the `a is a bell and the `s is the letter itself, because the v1 has
no `s.

The escape is expanded in the text of the v1, and in the parameter of a
command: the `FileAppend, a`b, *` writes "a<backspace>". The escape that
carries a comma is a literal, and it does not separate the parameters: the
`FileAppend, a`,b, *` writes "a,b". The escape that carries an operator is
the operator: the `x := 1`+2` is the sum of the two. The escape that carries
nothing is dropped: the `x := a` is the name "a".

The name of the v1 may carry the escape, and the escape is expanded before
the name is looked at: the `a`c` is the name "ac", and the `x := a`b` is the
name "a<backspace>", which is not a legal name of a variable. The refusal is
the one of the name, and it names the name that the escape made. The number
may be the name of a variable, and it may carry the escape too: the `x :=
5`b` is the name "5<backspace>".

The backtick that the escape carries is a backtick, and it is an illegal
character in an expression: the `x := a``b` is refused with "The leftmost
character above is illegal in an expression. Specifically: `b", and the symbol
begins at the backtick that the escape carries. The target of an assignment is
a name and not an expression: the `a``b := 5` is refused with "The following
variable name contains an illegal character: "a`b"".

The escape of the v1 does not protect the quote of a text: the `x := "a`"b"`
is the text "a" and the name `b"`, which is not a legal name of a variable.

The percent sign that the escape carries is a literal, and it does not open
the name of a variable: the `FileAppend, a`%b, *` writes "a%b", and the
`FileAppend, a`%b%, *` is refused when the script loads with "This parameter
contains a variable name missing its ending percent sign. Specifically:
a%b%". The v1 checks the percent sign of every parameter of its commands,
and not only the ones that want the name of a variable.

**The `Until` of the v1** (2026-11-04). The `Until` of the v1 is the
statement that comes right after the body of a loop, and it is the condition
that ends it: the body runs, and the condition is read after every turn of
it. The `Loop 3` / `i := i + 1` / `Until i == 2` leaves 2 in `i`, and the
`Loop` without a count goes on until the condition is true. The body that is
a block is followed by the condition too.

The `Until` that no loop takes is refused when the script loads, with "UNTIL
with no matching LOOP", and the refusal waits for the end of the load: the
error of a line that comes later is the one that is reported. The `Until`
that is the body of the loop is not after it, and the one that comes after
another statement is not either.

The `Until` of the v1 may be the name of a variable: the `until := 2` writes
the variable "until", and it is not the condition of the loop that comes
before it. The `Until` alone is refused with '"Until" requires at least 1
parameter. Specifically: until', and the parameter of it is the rest of the
line: the `until x := 5 {` is refused with "Unexpected \"{\"".

And the `if` of the v1 has no name of its own: the `if` alone is refused
with '"" requires at least 1 parameter. Specifically: if'. The other words
that want a parameter name themselves: the `until` above, the `while` and
the `goto`.

**What is left of the v1.** One corner of the `if` is measured on the binary
of v1.1.37.02 and the tree does not do it yet.

The brace that comes after the `if` that is inside the line of the
condition: the v1 refuses it with "Unexpected \"{\"", and the tree refuses
the line with "Unexpected \"If\"".

And one corner that is not of the `if`: the order of the refusals. The v1
reads the line before it says that the line does not make sense, and the
refusal of the name comes before the refusal of the sense: the `return a"b"`
is the name in the v1, and the tree says "Return's parameter should be blank
except inside a function.".

And one more that is not of the `if`: the text of a command of the v1 that is
not closed. The v1 takes the parameter as it was written: the `FileAppend,
"abc, *` writes `"abc`, and the tree refuses the line with "Missing
close-quote".

And the member that is written: the v1 has no objects, and the write of a
property of a value that is not an object is dropped -- the `a.b := 5` is
accepted, and the value of the `a` does not change. The tree reads the member
as a property, and the `a.b := 5` fails at the run with "This value of type
"String" has no property named "b".". And the member whose name is a symbol
-- the `a.'b' := 5` -- is refused by the v1 with "This line does not contain
a recognized action. Specifically: a.'b' := 5", and the tree refuses it the
same way.

And one more still: the call of a name that is not a function, when the
bracket that closes it is not there. The v1 says "Call to nonexistent
function. Specifically: abc(c" and the tree says "Missing \")\"".

And one more still: the order of the refusals of the load. The v1 refuses the
call of a name that is not a function before the parenthesis that is not
closed, and the tree refuses the parenthesis at the parse: the `x := abc(c`
and the `x := (` are refused by the v1 with "Call to nonexistent function."
and by the tree with "Missing ")"". The message of the parenthesis is the
same, and the order of the two refusals is the one that differs.

And one more: the operator of the v1 without an operand. The v1 answers with
the empty text: the `x := and` is empty, and the tree refuses the line with
"Syntax error. Specifically: and". The `x := not` is empty in the v1, and the
tree refuses it with "Missing operand. Specifically: not".

And the `if` of the v1 with an assignment: the v1 takes the line as the
command that it is, and the tree refuses the condition with "Syntax error.
Specifically: := 2".

**The hotkey of the v1** (2026-11-03). The v1 writes a hotkey in a line of
its own, and the two colons that separate the name from the action are what
says so. The name is a set of prefixes -- the `$` of the hook, the `~` of the
tilde and the `*` of the wildcard -- a set of modifiers -- the `^` of the
control, the `!` of the alt, the `+` of the shift and the `#` of the win,
with the `<` and the `>` naming the side of the key -- and the name of a key.
The word "up" may follow the name of the key, and the hotkey fires when the
key comes up. The name may not have a space in it: the "Browser Back" is not
a key, and the "Browser_Back" is. The combination of two keys -- the `RCtrl &
RShift` -- is a hotkey of its own, and a modifier is not one of its keys.

The name that is not the name of a key is refused when the script loads, and
the complaint of the v1 is "Invalid hotkey." with the line of it. The hotkey
that is declared twice is refused too, and the complaint is "Duplicate
hotkey." with the name of it, as the script spelled it: the `$` and the `~`
do not make the hotkey another one, and the case of the name does not either;
the `*` does. The constructs of the v1 and the contexts of it -- the `if`,
the `while`, the `loop`, the `for`, the `ifwinactive` and the `ifequal` --
are refused in the action of a hotkey that is written in one line, and the
complaint is "Invalid single-line hotkey/hotstring." with the line.

The action of the hotkey is the body that comes after the line, or the
command that the line carries. The body is the code that comes after the
line, and the `return` of it is what ends it: the body of the hotkey is not
a block, the variables of it are the variables of the script, and the
`return` ends the thread of the hotkey and not the script. The section of
the script that runs ends at the first hotkey or at the first hotstring of
the v1, and the hotkey is registered when the script loads: the hotkey that
is written in the body of another one is registered too.

The tree does it now: the line of the hotkey is a node of its own
(`Node::HotkeyDefinition`), the names of them are in
`crates/ahk/src/v1/hotkeys.rs`, and the hotkeys of the v1 are registered with
the desktop when the script loads.

What is not done yet: the script of the v1 that has a hotkey waits for the
keyboard, and the tree ends the script when the section that runs ends. The
waiting is a thing of the runtime and not of the dialect, and the v2 has it
missing as well: the hotkey of the v2 is registered and the script goes on.

**The settings of the v1.** The v1 and the v2 do not begin with the same
values: the mode of the coordinates is the "Window" in the v1, the method of
the sending is the "Event" and the mode of the matching of the titles is the
1. The state of the settings lives in a static, and the runtime writes the
defaults of its dialect when it is made -- the rule is the hook
`the_defaults_of_the_settings`. Three names of the v1 have another shape:
the flags are the "On" and the "Off" and not the number one and zero, the
encoding of the files is the empty text for the code page of the machine,
and the key that closes a menu is the empty text. The block of the settings
(2026-11-02) carried it.

**The `Run` of the v1.** The `Run` of the v1 starts the program and goes on,
and the `RunWait` waits for it. The pid of the process goes into the fourth
parameter of the command -- the variable of the caller -- in the two of them.
The machine of the runtime gained two capabilities for it: `launch`, which
starts a program and answers the pid of it without waiting for it, and
`run_and_wait`, which starts a program, waits for it and answers the pid, the
code and what the program wrote. The box starts the program through the
shell of the machine, the way its `run` does. The block of the pid
(2026-11-02) carried it.

**The `DllCall` of the v1.** The `ErrorLevel` of the `DllCall` of the v1 is
zero when the call works, and the code of the failure when it does not: the
minus three when the library is not there, the minus four when the function
is not in it, and the minus two when the call itself is not valid. The
failure of a native call carries the kind of it -- the call itself, the
library or the function -- and the reason of the machine, in the words of the
v2. The v1 turns the kind into the code; the v2 does not use the code, and
it keeps the reason. The block of the code (2026-11-02) carried it.

**The patterns of the files of the v1.** The source of the `FileCopy`, of the
`FileMove` and of the `FileDelete` may be a pattern, and the command does
the thing to every file that matches it -- the copy and the move into the
folder of the destination, with the name of the file. The `ErrorLevel` is
the number of the files that failed, and the v1 goes on: the `ErrorLevel` is
the only report of it. A pattern that matches nothing is not a failure, and
neither is a file that is not there. The block of the patterns (2026-11-02)
carried it.

**The number of the parameters of a command of the v1.** Every command of the
table was asked, one by one, and the complaint of the binary was read: 105 of
the 111 commands that want a parameter agree with the table, and six do not.
The command of the `GetKeyState` wants two parameters -- the key and the
mode -- and the row of the table had the numbers of the function. The v1 has
two forms of the complaint: the one that names the command ("\"X\" requires
at least N parameter(s).") and the one that names the parameter that has to
be there ("Parameter #N required"), and the `EnvSub`, the `ControlSend`, the
`GuiControl` and the `WinMenuSelectItem` are the commands of the second form.
The name of the `SetStoreCapsLockMode` is the one that the binary writes:
"SetStoreCapslockMode". The rule is a hook of the dialect
(`the_refusal_of_the_arity_of_a_command`) and the rules of the v1 are in
`crates/ahk/src/v1/parameters.rs`. A call that is a statement is a call too:
the `MyFunc()` is refused at the load with "Call to nonexistent function." and
the text of the call is named. The block of the arity (2026-11-02) carried it.

**The family that the sweep found and that is not carried yet.** The
differential sweep of the commands of the table against the binary
(2026-11-02) found one more family: the v1 checks the *value* of a parameter
of many of its commands when the script loads. The `Control`, the `WinGet`,
the `Menu`, the `RegRead`, the `SysGet`, the `SoundGet`, the `GroupActivate`,
the `Drive` and the others refuse with "Parameter #N invalid." at the load,
and the tree refuses them at the run with the reason of the machine. The
family is not carried yet: it needs the table of the values of each command
of the v1, and the text of the complaint needs the rule of the last parameter
of a command -- the block that comes after this one.

**The arithmetic of the v1 is on a variable of the script.** The `EnvAdd`, the
`EnvSub`, the `EnvMult` and the `EnvDiv` of the v1 are the arithmetic of the
v1 on a variable of the script -- the `EnvAdd, x, 3` with `x` at five leaves
`x` at eight -- and not the arithmetic of a variable of the environment: the
tree was writing a variable of the environment, and it was reading the value
of the variable of the script. The number that is not there is a zero, the
two whole numbers give a whole number, and the division of them is rounded
towards the zero. The division by zero is refused when the script loads
("Divide by zero"), and the divisor that the v1 does not know at the load is
not a zero. The units of the time are the third parameter of the `EnvAdd` and
of the `EnvSub`, and the unit that is not one of the four is refused at the
load; the date arithmetic itself is not carried yet, and the command says so
when it runs. The block of the arithmetic (2026-11-02) carried it.

**The environment of the v1.** The `EnvSet` writes the `ErrorLevel` of the
v1 -- one when there was a problem and zero otherwise -- and the v1 goes on:
the name of the variable of the environment may not carry the equal sign, the
machine refuses it, and the reason of it is a note. The name that is blank is
refused when the script loads, and the tree has a shape for it that is not
the one of a variable of the script: the `Name`, the text of it reaches the
command, and it may not be blank. The `EnvGet` does not write the
`ErrorLevel`, and the `EnvUpdate` has nothing to do -- the tree shares the
environment of the process with the script -- and it is carried now: the
surface of the v1 has one more name that answers with a value. The block of
the environment (2026-11-02) carried it.

**The path of the class.** The `new` of the v1 takes the path of the class:
the name that follows the word and the names of the classes that it holds.
The `new C.D` is an instance of D, and the `new o.D` is an instance of the
class that the instance holds, and the `new this.D` is an instance of the
class that the instance of the method holds. The class that a class holds is
a member of the instance of it too, and the class of the v1 has no `Call`:
the call of it makes nothing. The block of the path (2026-11-02) carried it,
with the path of the `new` (`crates/ahk/src/ast.rs`) and the walk of it
(`crates/ahk/src/runtime.rs`).

**The number of the v1.** The number of the v1 is a "pure number": it keeps
the text that was written -- the `x := 0x10` is "0x10", and the
`x := 00123` is "00123", and the `x := 1.50` is "1.50" -- and the value of
it is the number that the text holds. The sign of it is part of the number,
the number that does not fit is the one that fits, the whole numbers wrap
the way the machine does, and the operators that write a variable read the
number. The block of the number (2026-11-02) carried it, with the literal
of a number (`crates/ahk/src/ast.rs`), the coercion of a text
(`crates/ahk/src/v1/numbers.rs`) and the format of the integers.

The surface of the v1 has **405** entries: **298** answer with a value,
**107** are refused and none is pending. The surface of the v2 did not
move: **828** entries, **637** values, **191** refusals and no pending.

**The legacy assignment.** The `=` of the v1 assigns a text and not an
expression: `x = 5 + 3` leaves `x` at "5 + 3". The tree carries it since the
block of the `AutoTrim` (2026-11-02), with the text of the script, the trim
of the `AutoTrim`, the number that a text of base ten is, and the
substitution that finds the built-in variables.

**The format of the numbers.** The `SetFormat` of the v1 rounds the floats
that are computed to the decimals of the format, and the whole numbers are
in the base of the format. The tree carries it since the block of the
formats (2026-11-02), with the `A_FormatFloat` and the `A_FormatInteger`
that say which one is in force. The block of the number (2026-11-02) added
the number that is written: the whole number that is written without the
zeros in front of it is a number and the format writes it, and the pure
numbers keep the text that was written.

**The arithmetic with a text.** The v1 does not concatenate when an operand
is a text: the text that is written between quotes is a string, and the
value of a variable is a number when it can be one. The tree carries it
since the block of the arithmetic (2026-11-02): the sum of a text that is
written and of a number is a concatenation, the sum of a number and of a
text that is written is blank, and a value that is not a number makes the
operation blank.

**The sense of the case.** The `StringCaseSense` of the v1 turns the sense of
the case on and off, and the `A_StringCaseSense` says the one that is in
force. The tree carries it since the block of the sense of the case
(2026-11-02), with the comparison operators of the expressions: they follow
the sense, the `==` is always with the case, and the `=` is always without
it. The names that follow the sense in the v1 and that the tree carries with
the shape of the v2 — the `StringReplace`, the `StringGetPos`, the
`StrReplace` and the `InStr` — follow it since the block of the searches
(2026-11-02), with the shape of the v1.

**The math and the capacity.** The `Transform` of the v1 is a command with a
sub-command, and the `VarSetCapacity` is a function that empties the variable
of the caller. The tree carries both since the block of the math (2026-11-02),
with the kind of the answer of each sub-command, the quotes of the
parameters and the bits of thirty-two and of sixty-four. The tree keeps
no capacity of its own, and the number that it answers is the one that was
asked for.

**The body of the action.** The commands of the v1 that have a body are the
`IfInString`, the `IfNotInString` and the `IfMsgBox`, and the `else` is the
action of the other side. The tree carries them since the block of the body
of the action (2026-11-02), with the action of the same line, the block and
the statement of the next line, and with the refusal of the load: the
`else` that has no `if` before it, the number of the parameters of a command,
and the parameter that wants the name of a variable and is blank. The
measurement of the binary changed the grammar of the v1 that the tree
carried: the line of a command of the v1 continues when the line that comes
begins with an operator, and not when it ends with a comma -- the comma at
the end of the line is the rule of the v2.

**What stays out.** The operator that comes after a text that is written is
refused at the load, and the v1 accepts it: the v1 answers "5-1" and "20"
for `"5" - 1` and `"5" * 2`, and the documentation of the v1 does not say
what it means. The `if (expression)` of the v1 takes the statement of the
next line as its body and not the statement of the same line, and the tree
refuses the script that writes it; the `else if` that comes after a command
that has a body is refused with it. The `SetBatchLines` is a decision of the
plan (§5).

**The searches of the v1.** The `StringGetPos`, the `StringReplace` and the
`StrReplace` are the names that find a text in another one, and the v1 gives
each one of them a shape of its own. The tree carries them since the block of
the searches (2026-11-02): the place of the `StringGetPos` counts from zero
and is minus one when the text is not there, the occurrence is "L2" or "R1"
and the number that is alone is the first from the right, and the offset is on
the side that the occurrence names; the `StringReplace` replaces the first
occurrence of the text, and the parameter that says "all" says it with a
letter — the letter "a" in any case, the digit "1", or a parameter that
begins with "useerrorlevel"; the `StrReplace` is a function whose fourth
parameter is the variable that receives the number of the replacements, and
whose fifth is the number of the replacements that the caller allows.

The refusal of a command of the v1 names **the first parameter** of it, and
not the parameter that the refusal is about: `StringReplace, out, a` names
`StringReplace, out` when the command has too few parameters, and
`StringReplace, out, , b` names it too when the second parameter is blank.

**What stays out of the searches.** The `ErrorLevel` is a variable of the v1
that the tree does not have, and the `StringGetPos` and the `StringReplace`
of the v1 put their answer there — it comes with the block of the
`ErrorLevel` of the machine, and it touches the two dialects.

**The `ErrorLevel` of the v1.** It is the one built-in variable of the v1
that has no prefix, and the measurement of the surface took the names of the
page of the built-in variables and left it out. The tree carries it since
the block of the `ErrorLevel` (2026-11-02): it is the global variable of the
script, it is zero when the script begins, a script may write it, and the
`StringGetPos`, the `StringReplace` and the file commands write it the way
the v1 does.

**What stays out of the `ErrorLevel`.** The commands of the machine that
write it as well -- the `ClipWait`, the `DllCall` and the `Run` -- are not
carried yet: theirs is the answer of the machine and not a count of the
failures. The `global ErrorLevel` is refused by the v1 -- "Built-in variables
must not be declared." -- and the tree refuses it since the block of the
`global` (2026-11-02).

**The `ErrorLevel` of the machine.** The commands of the machine write it
too, and the tree carries it since the block of the machine commands
(2026-11-02): the `ClipWait` writes one when the wait expires and zero when
the clipboard has data, the `Run` writes nothing unless the options say
"useerrorlevel" -- and then it writes zero when the program starts and the
word "ERROR" when it does not -- and the `RunWait` writes the code of the
program that ran. The `RunWait` is a command of the v1 that the surface did
not carry: the page of the documentation has the `Run` and the `RunWait`
together, and the list of the names took one of them.

**The call of a function of the v1.** The v1 refuses the call of a function
at the load, and the refusal names the call as it was written. The tree
carries it since the block of the call (2026-11-02): the number of the
arguments that is wrong is refused with "Too few parameters passed to
function." or with "Too many parameters passed to function.", and the name
that is not a function is refused with "Call to nonexistent function.". A
command of the v1 is not a function, and a function of the v1 is not a
command.

**The text that the refusal of a call names** (2026-11-04). The text is the
rest of the line, from the name of the call, and not only the call: the
`x := abc(c) + 1` is refused with "Call to nonexistent function.\n
     Specifically: abc(c) + 1", and the same is true of the complaint of the
number of the arguments that is wrong -- the `x := Abs() + 1` is refused with
"Too few parameters passed to function.\n     Specifically: Abs() + 1". The
spaces that the script wrote are in it: the `x := InStr( "ABCabc" )` names
the `InStr( "ABCabc" )` and not only the call.

**The definition of a function of the v1** (2026-11-04). The definition is
the line `f(a) {` and the line `f(a)` whose brace is on the line that comes
next, and the rest of the line after the parenthesis that closes the list
decides: when there is anything else there, the line is a call. The `f(a) 1`
is refused with "Call to nonexistent function.\n     Specifically: f(a) 1",
and the `f(a)` alone is refused with "f(a)". The rule is a hook of the
dialect (`a_line_is_a_definition`), and the v2 decides with the token that
comes after the parenthesis: the brace and the fat arrow make a definition
of it.

Two corners of it came with the block of the tail of the call (2026-11-04):
the `f(a) { return a }` is a call in the v1, and the tree refuses it with
"Call to nonexistent function.\n     Specifically: f(a) { return a }", and the
`f(a) { x := 1` is refused the same way.

**The brace that follows a value** (2026-11-04). The v1 refuses the brace
at once, and it does not look for the brace that closes the literal of an
object: the `x := 1 {` is refused with "Unexpected "{"", and not with
"Missing "}"". The literal of an object is a literal of the v1 too, when it
begins the expression: the `x := {a: 1}` is a literal.

The rule is a hook of the dialect (`the_brace_after_a_value_opens_a_literal`),
and the v2 looks for the closing brace first.

**The call whose parenthesis that closes it is not there** (2026-11-04). The
v1 does not need the parenthesis that closes the call of a name that is not a
function, and the call is refused at the load: the `x := abc(c` is refused
with "Call to nonexistent function.\n     Specifically: abc(c", and the parse
of it does not fail. The arguments of the call are the rest of the line, and
the list that is not closed does not go on to the line that comes next.

The call of a name that is a function needs the parenthesis, and the failure
is "Missing ")"": the `x := f(1` of a function of the script, and the
`x := Abs(1` of a built-in. The name of a function of the script may be
defined after the call, so the failure of it waits for the end of the parse,
and it is made with the other refusals of the load and in the order of the
script: the call that does not close comes before the call that does not
exist when it is written first.

The rule is a hook of the dialect
(`the_closing_parenthesis_of_a_call_is_needed`), and the parser reads the
arguments of the call with it
(`the_arguments_of_a_call`).

**The tail of a call of the v1** (2026-11-04). The v1 reads the rest of the line
as text when the name of the call is not a function, and the refusal of the
call is the only one of the line: the `MyFunc() { return 2 }` is refused at the
load with "Call to nonexistent function.\n     Specifically: MyFunc() { return 2 }",
and the brace of it is not refused. When the name is a function, the rest of
the line is read, and the `f(1) { return 2 }` is refused with "Unexpected "{"".
The name of a function that the script defines further down is known when the
line is read: the `f(1) { return 2 }` with the definition of `f` after it is
refused with "Unexpected "{"", and not with "Call to nonexistent function.".

The brace that opens a block is not a part of the text that the refusal names:
the `zzz(1) {` is refused with "Call to nonexistent function.\n     Specifically:
zzz(1)". The brace of an expression is a part of it: the `x := abc(c) {` names
the "abc(c) {".

Two more rules of the v1 came with it: the definition of the v1 needs a body
-- the `f(a) {` at the end of the file is a call, and the `f(a) {` with a body
is a definition -- and the name of a parameter of the v1 may begin with a
number: the `f(1) {` is a definition, and the parameter of it is the variable
"1".

The brace that opens the body of a construct names the brace in the v1: the
`if 1 { return 2 }` and the `while 0 { return 2 }` are refused with
"Unexpected "{"".

The rules are hooks of the dialect
(`the_tail_of_a_call_to_a_name_that_is_not_a_function_is_text`,
`the_definition_needs_a_body` and
`the_message_of_the_brace_that_opens_the_body`), and the parser knows the names
of the functions of the script before it reads the lines of it
(`the_functions_of_the_script`).

**The tail of a call whose name is a function** (2026-11-04). The tail of a
statement is a list of items in the v1, and the commas are what separate them.
The items that are expressions are evaluated, and an item that is not an
expression is a text -- the parameter of a command -- that the v1 does not run
and does not refuse. The `f(1) FileAppend, [tail], *` runs the call and
nothing else, and the load does not fail, and the `f(1) x := 2` writes the
variable "x". The items that come after the text are texts as well, and they
do not run.

The tree reads the tail of a statement the way the v1 reads it. The rule is a
hook of the dialect (`the_tail_of_a_list_is_text`), and the v2 refuses the
item of the list that is not an expression with "Syntax error.\n     Specifically: *".

**The failure of the condition of the v1 is not reported** (2026-11-04). The v1
does not report the failure of the condition of an `if` or of a `while`: the
`if := 5`, the `if 1 := 5`, the `if 1/0` and the `if x.y` are accepted, the
condition is false, and the body of the construct does not run. The failure of
a bracket that is not closed is reported: the `if (x` is refused with
"Missing ")"", and the `if [1` with "Missing "]"".

The failure of the run is caught by the runtime (`the_value_of_the_condition`),
and the failure of the parse is caught by the parser
(`the_condition_that_is_not_reported`). The rule is a hook of the dialect
(`the_failure_of_a_condition_is_not_reported`), and the v2 reports the failure.

**The write of a property of a value that is not an object is dropped** (2026-11-04).
The v1 does not refuse it: the `a.b := 5` is accepted when the `a` has no value,
when the `a` is a text and when the `a` is a number, and the write is dropped --
the value of the `a` does not change. The write of a member of an object is kept.

The v2 refuses the write with "This value of type \"String\" has no property
named \"b\".".

The rule is a hook of the dialect
(`the_write_of_a_property_of_a_value_that_is_not_an_object_is_dropped`), and the
runtime drops the write.

**The operand that is not there** (2026-11-04). The v1 does not refuse the
expression whose operand is not there: the `x := 1 +` and the `x := not` are
accepted, and the value of the expression is the empty text.

The operand that is not there poisons the operation that holds it: the
`not (1 +)` is the empty text, and not the one that the `not` of the empty
text answers. The `&&` and the `||` are the exception: when the left side
decides, the answer is one or zero -- the `0 and` is zero, and the `1 or` is
one.

The word that is an operator and that begins the expression has no left
operand, and it takes the value of the right one: the `x := and 2` is one,
and the `x := or 2` is one. The word alone is the operand that is not there.

The line that begins with the operator holds no action: the v1 refuses it
with "This line does not contain a recognized action." and names the whole
line -- the `not` names the "not", and the `not 1` names the "not 1".

The v1 does not continue the line that ends with an operator: it merges the
line that BEGINS with an operator with the one before it. The `x := 1 +` and
the `2` that comes after it are two lines, and the `2` is refused with "This
line does not contain a recognized action.". The comma does not continue the
line either.

The rules are hooks of the dialect
(`the_operand_that_is_not_there_is_not_refused`,
`a_line_that_ends_with_an_operator_continues` and
`the_line_that_begins_with_the_word_holds_no_action`), and the parser makes
the operand that is not there a node of its own (`Node::Unset`).

**What stays out of the operand that is not there.** The value that holds the
operand that is not there is the operand that is not there: the `StrLen(1 +)`
is the empty text, and not the zero that the `StrLen` of the empty text
answers; the `f(1 +).y` and the `f(1 +)[1]` are the empty text; and the
`a := [1 +]` is not an object, and the `IsObject(a)` is zero. The item of a
value that is not an object is the empty text in the v1, and the tree reads it
as one.

Two things stay out of it. The write of the item of a value that is not an
object is dropped by the v1 -- the `x[1] := 5` of the `x := ""` runs and the
write is lost -- and the tree refuses it with "This value of type "String" has
no property named "__Item".". The length of the array of the v1 is a method
and not a property: the `a.Length()` of the `a := [1, 2]` is two, and the
`a.Length` is the empty text; the tree reads the `a.Length` as the length.

**The word that is alone at the start of a statement holds no action**
(2026-11-04). The v1 refuses a line that begins with a reserved word and
that holds no action, and it says "This line does not contain a
recognized action." The `true` alone and the `false` alone are refused the
same way as the `not` and the `and`: the word is named, and the line that
begins with it is named whole -- the `true` names the "true", and the
`true + 1` names the "true + 1". The comment of the line is not a part of
the text that is named.

The declaration is refused the same way. The `global` alone at the top
level of the script is refused, and the `local` alone and the `static`
alone too: they are declarations of a function, and the script is not a
function. The `local x` and the `static x` are refused the same way, and
the text that is named is the whole declaration -- the "local x" and the
"local x := 1".

The declaration that has no name is the first statement of the body of the
function, and it is accepted there: the `local` alone and the `static`
alone declare nothing, and the `global` alone is the mode of the function.
Anywhere else it is refused, and the word is named.

The v2 keeps its own messages: the `true` alone is refused with
"Unexpected \"true\"", the `local` alone with "Unexpected declaration.\n
Specifically: local", and the `global` alone at the top level of the script
is accepted.

The rules are hooks of the dialect
(`the_line_that_begins_with_the_word_holds_no_action`,
`the_message_of_the_refused_declaration`,
`the_bare_local_is_a_declaration_of_no_name`,
`the_bare_global_is_the_mode_of_the_script` and
`the_declaration_with_no_name_is_the_first_statement`), and the parser asks
them.

**What stays out of the word alone.** The call of a word that is a
reserved word is a call and not a line that holds no action: the `true()`
of the v1 is refused with "Call to nonexistent function.\n     Specifically:
true()", and the tree says that the line holds no action. The same is true
of the label of a word that is a reserved word -- the `true:` is a label of
the v1 -- and of the command of a word that is a declaration: the `static,
x` is a line that holds no action in the v1, and the tree says that a
declaration needs a name.

**What stays out of the call.** The call of a function of the v1 that is
written without the parentheses and inside an expression -- `x := InStr
"abc"` -- is not a call: it is the name of the function and the text, side by
side. The measurement of the binary corrected the plan, which said that the
call happens: the name of a function is not a value, and the text is what is
left of it. It came with the block of the name of a function.

**The name of a function.** The name of a function of the v1 is not a value:
`x := StrLen` is a blank text, and so is the name of a function of the script
and the name of a command. The tree carries it since the block of the name of
a function (2026-11-02): the `IsFunc` of the v1 takes a value and not a name,
a command of the v1 is not a function, and the `Func` of the v1 is the
reference to the function -- it carries the name of it, whether it is built
in, and the number of its parameters -- and the name that is not a function
is the number zero.

**What stays out of the name of a function.** The text of an object of the
v1 is the empty text -- `Object()` is a blank text, and `Object() . "a"` is
"a" -- and the tree gives the class of it. It comes with the block of the
objects of the v1.

**The objects of the v1.** The v1 does not refuse a member that a value does
not have: the answer is a blank text -- the property of an object that does
not have it, the property of a value that is not an object, and the item of
an object. And the text of an object of the v1 is the empty text, so the
concatenation of an object and of "a" is "a", and a built-in that wants a
text takes it: `StrLen(Object())` is zero. The tree carries it since the
block of the objects (2026-11-02).

**What stays out of the objects.** The built-in that wants a number refuses
an object with the words of the v2, where the v1 answers a blank text:
`Abs(Object())` is a blank text in the v1.

**The classes of the script of the v1.** The tree carries them since the
block of the classes (2026-11-02): the `new` makes an instance -- `new C` and
`new C(5)` , and the word is not reserved -- the arguments of the `new` are
the ones of the `__New` , the `__Init` takes none and runs before it, the
`this` of a method is the object the method was called on, the method of the
class object runs (`C.Say()`) and the v2 refuses it, the `__Class` of the v1
is a property of an object, the call of a method does not check the number of
the arguments, the `base` is the base of the class where the method was
defined, and the default value of a parameter may be written with the equal
sign. The probe that compares the two is
`egm-review/probe_both_v1_classes_all.py` , and every one of its 28 cases
answers the same in the binary and in the tree.

**What stays out of the classes.** The refusals of the body of a class of the
v1 -- the duplicate of a name and the class that is declared twice -- come
with the block of the refusals.

**The `global` of a built-in variable.** The v1 refuses it at the load, and
the refusal names the variable: "Built-in variables must not be declared.
Specifically: ErrorLevel". The set of the names that are refused is the set
of the built-in variables of the v1, spelled the way the v1 spells them:
every one of them has the prefix `A_` , and four of them have a name without
it -- the `Clipboard` (the `A_Clipboard` is its alias, and the documentation
of the v1 says so), the `ComSpec` , the `ProgramFiles` and the `ErrorLevel` .
A name without the prefix that is not one of those four is a variable of the
script: the v1 reads it through the environment, and the environment of the
machine has no `Cursor` of its own. The `A_Args` is the one built-in variable
that is not refused -- the interpreter creates it at the load, and it is a
variable of the script -- and the names that the v1 does not have as
variables are not refused either: five of them are directives of the v1 and
the `A_TrayMenu` is a variable of the v2. The tree carries it since the
block of the `global` (2026-11-02), and the test that walks the surface of
the v1 asks the tree about every variable of it.

**The coercion of the built-ins of the v1.** The built-in of the v1 does not
check the kind of its arguments -- the `Abs("65")` is 65 and the
`Abs("abc")` is the empty text -- and the v1 reads a text as a number in
three ways. The `Abs` , the `Max` , the `Min` and the `Mod` want a text that
*is* a number, the other thirteen functions of the math want the number that
the text *starts* with ("65abc" is 65, and "1e3" is 1000), and the `Chr`
wants the whole number that the text starts with ("1e3" is 1). The exponent
of the v1 needs the point of the number: "1.5e2" is 150 and "1e2" is not a
number -- the documentation of the v1 says the same. The `Chr` is a function
of the v1 and not a row of the v2: the code that is zero, that is negative
or that is not a character is the empty text.

**The text of a float of the v1.** The number of the v1 has seventeen digits,
and the format rounds that number to its decimals -- a half goes away from
the zero. `Exp(65)` is "16948892444103338000000000000.000000" and not
"16948892444103337803358666752.000000", and the exponent of the scientific
notation has three digits: "1.068647e+013". The whole number of the v1 that
does not fit in sixty-four bits is the largest one:
`x := 100000000000000000000` is 9223372036854775807.

**The block carries.** The tree carries these three rules since the block of
the coercion (2026-11-02), and the probes that compare the two are
`egm-review/probe_both_v1_number_coercion.py` (215 of 216) ,
`egm-review/probe_both_v1_math.py` (174 of 174) and
`egm-review/probe_both_v1_float_text.py` (120 of 120).

**What stays out of the coercion.** The `Chr` of a code that is outside of the
plane of the basic multilingual plane: the v1 answers with the pair of
surrogates of its UTF-16 and the tree with the character. And the format of
the floats that the v1 takes with something after the "e" ("0.6e3"): the v1
accepts it and writes the rest of the format at the end of the number, and
the tree refuses it.

**The refusals of the body of a class of the v1.** The body of a class of the
v1 has one set of names: a name that is declared twice is refused, and it
does not matter what the member is -- a method and a variable with the same
name are a duplicate, and so are a static member and one that is not. The
refusal names the member that was written: "Duplicate declaration.\n
Specifically: x", and the v2 names the path of it ("C.Prototype.Foo"). The
`__Init` that has a property before it is refused with the name of it, and a
class that is declared twice is refused with the words of the v1: "Duplicate
class definition.\n Specifically: C".

**The block carries.** The tree carries these three rules since the block of
the refusals (2026-11-02), and the probes that compare the two are
`egm-review/probe_both_v1_class_members.py` and
`egm-review/probe_both_v1_class_names.py`.

**What stays out of the refusals.** The `__Init` that is static, which the v1
refuses with "Invalid class variable declaration.\n Specifically: __Init() {",
and the property that has a getter, which the v1 refuses with "Not a valid
property getter/setter.\n Specifically: get => 1" -- the tree takes both. The
name of a class and the name of a function may be the same in the v1, and
the name of a class may be the output of an assignment, and the tree refuses
the two.


**The number of a parameter of the v1.** The built-in of the v1 does not check
the kind of its arguments, and the parameter that wants a number takes the
number that the text of it starts with: the `SubStr("abcdef", "2abc")` is
"bcdef", and the hex of the v1 is a number -- the `SubStr("abcdef", "0x2")` is
"bcdef" as well. The `SubStr` of the v1 counts the places the way the v1
counts them: the place that is zero is the last character of the text, and a
place that is negative counts from the end of it and takes one character more
than the v2 takes -- the `SubStr("abcdef", -1)` is "ef". The sense of the case
of the `InStr` of the v1 is the number that the text holds -- the
`InStr("ABCdef", "cd", "1")` is 0 and the `InStr("ABCdef", "cd", "Off")` is 3
-- and the needle that is empty is at the first character of the haystack.

**The block carries.** The tree carries these rules since the block of the
number of a parameter (2026-11-02), with the table of the parameters that want
a number (`crates/ahk/src/v1/kinds.rs`), the handlers of the `SubStr` and of
the `InStr` of the v1 (`crates/ahk/src/v1/strings.rs`), and the conversion of
the argument in the runtime, before the body of the built-in runs. The probes
that compare the two are `egm-review/probe_v1_number_coercion.py` and
`egm-review/probe_v1_exit_code.py`.

**What stays out of the number of a parameter.** The parameter of a *command*
of the v1 is an expression -- the documentation of the v1 says so, and the
`Sleep, 100+50` waits for a hundred and fifty milliseconds -- and the tree
reads it as the text that it is: the `Exit, "3"` is zero in the tree and three
in the v1. And the grammar of the expression of the v1 is the old one: the
`x := 1e2` is empty in the v1 and a hundred in the tree, and the number that
comes from a literal in hexadecimal keeps the format of it -- the `x := 0x10`
is sixteen and the text of it is "0x10".


**The parameter of a command of the v1 that wants a number is an expression.**
The documentation of the v1 says so: "Numeric parameters accept a literal
number or an expression, and can be identified by phrasing like 'This
parameter can be an expression.'". The `Exit, 1+1` leaves the code at 2, the
`Exit, StrLen("abcd")` leaves it at 4, and the `Random, x, n, n` leaves `x` at
the value of `n`. A parameter that is not an expression is the number zero --
the `Exit, 100abc` and the `Exit, 1+` -- and a parameter that is a text is the
number that the text holds: the `Exit, "3"` is 3 and the `Exit, 0x10` is 16.
The substitution of the v1 is a text and not an expression: the `Exit, %n%` is
the value of `n`, and the `Exit, %n%+50` is fifty and not a hundred.

**The block carries.** The tree carries these rules since the block of the
parameter of a command (2026-11-02), with the table of the parameters that
want a number (`crates/ahk/src/v1/kinds.rs`), the parser that reads the
parameter as an expression (`crates/ahk/src/parser.rs`), the tree that carries
the parameter as a text or as an expression (`crates/ahk/src/ast.rs`), and the
runtime that evaluates it (`crates/ahk/src/runtime.rs`). The probes that
compare the two are `egm-review/probe_v1_bad_parameter.py` and
`egm-review/check_v1_u.py`.

**What stays out of the parameter of a command.** The refusal of the load that
the v1 makes for a parameter that is not an expression -- the `Exit, )` is
refused with "Unexpected \")\"" and the tree gives zero -- and the grammar of
the expression of the v1, which is the old one: the `x := 1e2` is empty in the
v1 and a hundred in the tree.


**The body of a class of the v1.** The class of the v1 has no static method,
and the v1 refuses it with "Invalid class variable declaration.\n
Specifically: __Init() {" -- the text is the member, from the name to the brace
that opens the body. The class of the v1 has no property with a getter, and
the v1 refuses it too: the getter that is on the same line as the name with
"Not a valid method, class or property definition.\n Specifically: x { get =>
1 }", and the getter that is in a block with "Not a valid property
getter/setter.\n Specifically: get => 1".

**The block carries.** The tree carries these two rules since the block of the
body of a class (2026-11-02), with the two hooks of the dialect
(`crates/ahk/src/dialect.rs`) and the parser that asks them
(`crates/ahk/src/parser.rs`). The probes that compare the two are
`egm-review/probe_v1_class_refusals3.py` and
`egm-review/check_v1_class_body.py`.

**What stays out of the body of a class.** The name of a class and the name of
a variable of the script may be the same in the v1 -- the `C := 5` with the
class `C` is taken, and the value of `C` is five -- and the tree refuses it.

**What stays out of the return of the v1.** The form of the command -- the
`return, value` -- is not carried: the tree reads the `return` of the v1 as
the keyword of the v2, and the `return, hello` is refused with "Missing
operand.". What the tree carries is the order of the two refusals: the v1
reads the parameter of the return before it refuses it, and so the
`return a"b` is refused with the name that is not legal, and not with the
refusal of the blank parameter.

**What stays out of the object literal of the v1.** The literal of the v1 is
not the literal of the v2: the v1 refuses the `a := {,}` with "Blank
parameter", and it takes the name of a property that is a text or a number
with a point -- the `a := {"b": 1}` and the `a := {1.5: 1}` are accepted. The
tree carries the word of the complaint that the two have in common: the v1
says 'Missing "key:" in object literal.', and the v2 says 'Missing
"propertyname:" in object literal.'.

**What stays out of the loop of the v1.** The body of a construct of the v1 is
the statement that comes next, and the brace that closes a block is not one:
the `f() { loop }` and the `if (1) { loop }` are refused at the load with
'Unexpected "}"', and the tree accepts them. What the tree carries is the loop
that has no body at the end of the script: the v1 ends the script at it, and
the tree went on until the guard rail.

**What stays out of the text of a command of the v1.** The text of a parameter
of a command of the v1 is carried, and so is the quote that is not closed in
the line of a command. Two things stay out. The order of the refusals of the
`f("abc`: the v1 refuses the quote before it says that the function does not
exist, and the tree says it the other way round. And the text that the
refusal of the hotkey names: the v1 names the action -- the `"abc` -- and the
tree names the line.

**The legacy reference of the v1** (2026-11-04). The `%name%` of a command
parameter of the v1 is the value of the variable of the script, and the tree
does that: the `FileAppend, %a%, *` writes the "1" of the `a := 1`, and the
`FileAppend, %ZZTEST%, *` writes the value of the environment. In an
expression it is a double dereference: the text of the value of the variable
is the name of the variable that the expression reads, and the tree does that
too -- the `a := 1` and the `1 := 99` and the `x := %a%` read the variable
"1", and the `x` is ninety-nine. The name that is not there is not refused
in the v1: the variable of that name is created and read, and the answer is
the empty text. The write of a name that the script builds creates the
variable as well: the `a := 1` and the `%a% := 7` write the variable "1".
Inside a function the v1 refuses it, and it refuses it whatever the variable
of that name is.

**What stays out of the legacy reference of the v1.** The `%` of an expression
is not the modulo operator: the v1 refuses the `x := 10 % 3` at the load
with "This parameter contains a variable name missing its ending percent
sign." and it names the word that holds the `%`, and the tree says "Missing
ending "%"" and names the rest of the line. The `x := % 3` is accepted by the
v1 -- the percent and the space are left out of the expression -- and the tree
refuses it. And the name that is not legal: the `x := %a.b%` is refused by
the v1 at the load with "Ambiguous or invalid use of \".\"", and the tree
reads it and answers the empty text. The refusal of the write inside a
function names the name that the script built and not the reference as it was
written: the v1 says "Specifically: %a%" and the tree says the name "1".

**The literal of an object of the v1 is a list of parameters** (2026-11-04).
The literal is a list of parameters, and the number of them has to be even:
the key of a property and its value are two, and a parameter that is in
blank is one. The number is checked after every parameter that is not in
blank, and a number that is odd is refused with "Missing "key:" in object
literal.": the `a := {, b: 1}`, the `a := {, 1, 2}`, the `a := {b: 1, , c: 2}`,
the `a := {b: 1, c}` and the `a := {1, 2}` are refused that way. The
`a := {, 1}` and the `a := {, "b"}` are accepted, and the object of them has
no property: a value that has no name is evaluated and dropped, and the
`a := {, f()}` calls f. The blank at the end of the list is a parameter in
blank, and the v1 refuses it with "Blank parameter", naming the whole
literal: the `a := {b: 1,}`, the `a := {,}` and the `a := {b: 1,,}` are
refused that way. The v2 accepts the comma at the end of the literal, and the
`x := {a: 1,}` is an object of one property.

The tree does all of it: the literal of the tree is a list of items, and an
item that has no name is a value that the runtime evaluates and drops.

**The key of a property of the literal of an object of the v1** is the value
of the expression that is written until the colon (2026-11-04). The v1 takes
it as a value and not as the text of it: the `{1 + 1: 1}` has the property
"2", and the `{a.b: 1}` has the property of the value of `a.b`. A key that is
a single name is the name itself: the `{b: 1}` has the property "b", and the
value of the variable `b` is not the key. The tree takes the value of the
expression too.

**What stays out of the kind of the key.** A key that is a number keeps its
kind: the `{1: 1}` has the item 1, and the `a["1"]` of it is the empty text,
because the item of a number is not the property of that name. The tree keeps
the property of the text: the `{1: 1}` has the property "1", which the
`a["1"]` finds.

**The comma at the end of the list of the arguments of a call of the v1** is a
parameter (2026-11-04). The v1 counts it: the `f(1,)` is refused with "Too
many parameters passed to function." when f takes one parameter, and the name
of it is the call. The tree counts it too.

**The blank parameter of a call of the v1** is refused with "Blank parameter"
(2026-11-04). The v1 counts it: the `g(,)` and the `g(, 1)` are refused that
way, and the name of it is the call. The blank is a parameter that is not
there, and the v1 refuses it when the list ends right after it or when the
number of the parameters that came before it is less than the least number of
them. The complaint of the number of the parameters comes before it -- the
`g(1,)` is refused with "Too many parameters passed to function." when g takes
one parameter -- and the name that is not a function comes before both: the
`zzz(,)` is refused with "Call to nonexistent function.". The tree carries it:
the blank keeps its place in the list of the arguments, and the place where it
is decides the complaint.

**The list of the arguments of a method of the v1** that ends with a blank is
refused at the load (2026-11-04). The number of the arguments of a method is
not checked there -- the `o.Say(1, 2)` is accepted -- and the blank at the end
of the list is refused all the same: the `o.Say(1,)` is refused with "Blank
parameter", and the name of it is the text of the call, from the dot. The
tree refuses it too.

**The brace that closes a block of the v1 names the brace and nothing else**
(2026-11-04). The v1 does not name the line of the script under the refusal:
the `y := 2 }` that comes after the `if (1) {` is an "Unexpected "}"" with
nothing under it, and so are the brace that closes the body of a function and
the brace that closes nothing. The v2 names the line of the script under the
same refusal. The tree carries it: the hook of the dialect is
`the_closing_brace_names_the_text`, and the v1 turns it off.

**The text of a parameter of a command of the v1 is cut at the commas**
(2026-11-04). The quotes and the groups of parentheses do not hold the comma
back: the `FileAppend, "a,b", *` has three parameters -- `"a`, `b"` and `*` --
and the v1 refuses it with "Parameter #3 invalid." and the text "*". The group
is the same: the `FileAppend, (a, b), *` has three parameters, and the
`FileAppend, (a, b), c, *` complains about the third parameter with the text
"c, *".

The parameter that wants a number is an expression, and there the comma that
is inside a text or inside a group does not separate it: the `Sleep, "1,2"` is
the text "1,2" and not two parameters. The escape holds the comma back in the
two: the `FileAppend, a`, b, *` is the text "a, b".

The third parameter of the `FileAppend` of the v1 is the encoding of the file,
and not the options of v2: the `FileAppend, a, b, extra` is refused with
"Parameter #3 invalid." and the text "extra".

The tree carries it: the parser cuts the text of a parameter at the character
level, and the piece of a text that it cuts is not a range of tokens. The
probe of the block is `egm-review/probe_v1_command_cut.py`, and it is eleven
cases of the binary and the tree, all the same.

**The quote that is not closed in the line of a command of the v1 is not an
error** (2026-11-04). The v1 has no texts in the line of a command: the
`FileAppend, "abc, *` writes the text `"abc`, and the right side of the
legacy assignment is a text too: the `x = "abc` leaves `x` at `"abc` with the
quote that was written.

The text is refused when it is read as an expression: the `x := "abc` is
refused with "Missing close-quote" and the text that was read, the `"abc`.

The tree carries it: the lexer of the v1 does not fail with the quote that is
not closed, and gives the text to the parser, which refuses it when it reads
it as an expression. The v2 keeps the refusal of the lexer.

The probe of the block is `egm-review/probe_v1_unclosed_quote.py`, and it is
ten cases of the binary and the tree: five are the same, and the five that
are not are the ones that the paragraph above names, and the `FileAppend` that
has no name of file -- the v1 writes it and the tree refuses it.

**The brace that opens the body of the loop of the v1 is the one that is at
the end of the line** (2026-11-04). The v1 opens the body of the loop with
the brace that ends the line of it: the `loop {` opens the body and the loop
goes on until a break, and the `loop 3 {` opens it with three turns. The
command of the v1 takes the count after a comma, and the `Loop, 3 {` opens
the body with three turns as well.

The text that follows the brace on the line is the text of the count: the
`loop { break }` is a loop of files whose pattern is the text "{ break }",
and it has no turn. The count of the loop of the v1 is the text of the line
and not the expression, and the tree already carried that: the text that
the script wrote is the count, and the loop is skipped when the text is
not a number.

The tree carries the rule of the brace: the hook of the dialect is
`the_brace_of_the_body_of_the_loop_is_at_the_end_of_the_line`, and the v1
turns it on. The v2 reads the body of the loop on the line of it, and the
brace that is not at the end of the line is refused with 'Unexpected "Loop"'.

Two things stay out, and they are the `while` of the v1: the `While, 1 {`
opens the body of it, and the tree refuses it with 'Unexpected "}"'; and the
`While, 1 { break }` is refused by the v1 with 'Unexpected "{"', and the
tree accepts it. The paragraph that comes next closes them.

The probe of the block is `egm-review/probe_v1_loop_brace.py`, and it is
eight cases of the binary and the tree: seven are the same, and the one that
is not is the loop without a count and without a break -- the v1 goes on and
the tree stops it with the guard rail of the statements. The probe of the
forms is `egm-review/probe_v1_loop_forms.py`, and it is eleven cases: nine
are the same, and the two that are not are the `while` that the paragraph
above names.

**The condition of the while of the v1 comes after a comma** (2026-11-04).
The `While, n < 2` is the while of the v1: the body of it is the line that
comes next or the block that the brace opens, and the brace that is not at
the end of the line is refused with 'Unexpected "{"'.

The condition that is not there is refused at the load: the `While,` is
refused with '"While" requires at least 1 parameter.' and the text of the
line, the "While,". The blank condition is refused the same way.

The tree carries it: the parser takes the comma of the v1 before it reads
the condition. The v2 has no comma there, and the `while , 1` is refused
with "Missing operand.".

The probe of the block is `egm-review/probe_v1_while_forms.py`, and it is
ten cases of the binary and the tree, all the same.

**The arithmetic of the date of the v1 is the arithmetic of the machine**
(2026-11-04). The `EnvAdd` and the `EnvSub` of the v1 have two forms: with
two parameters they are the arithmetic of a number on a variable of the
script, and with three they are the arithmetic of the date of the machine.
The third parameter is the unit of the time -- the "S", the "M", the "H" and
the "D" -- and the unit is the first letter of the text that is written.

The `EnvAdd` adds the amount to the date of the machine and answers with the
date -- the text of the `YYYYMMDDHH24MISS` -- and the `EnvSub` takes the date
of the second parameter and answers with the number of the units that are
between it and the date of the machine.

The amount that is not a number leaves the variable empty, and the date that
is not a date leaves it empty too: the text that is empty is zero, and the
text that is not empty and is not a date is nothing. The date is checked when
the script loads: the second parameter of the `EnvSub` that is a number and
is not a date is refused with "Parameter #2 invalid." and the text that was
written. The text that is not a number is not a date, and the v1 lets it
through.

The time of the machine is a number of a hundred of nanoseconds since the
first of January of 1601, and the arithmetic of it goes around the largest
number of the machine: the amount of the biggest number of the machine leaves
the date of the machine where it was. The machine reads the years from 1601
to 30827, and the date that is not one of them leaves the variable empty.

The tree carries it: the hook of the dialect is
`the_parameter_keeps_its_text`, and the amount of the `EnvAdd` and of the
`EnvSub` keeps its text when it is not an expression -- the "5abc" is not a
number, and the command leaves the variable empty. The date of the v1 is in
`crates/ahk/src/v1/dates.rs`, and the reach of the machine is in
`crates/ahk/src/v1/mod.rs`.

The probes of the block are `egm-review/probe_v1_env_text.py`,
`egm-review/probe_v1_env_corners.py` and `egm-review/probe_v1_env_reach.py`,
and they are the cases of the binary and the tree: all the same.

**The MouseMove of the v1 takes the place and the speed** (2026-11-04). The
`MouseMove` of the v1 takes three parameters -- the place, the place of the
y, and the speed of the move -- and the fourth one is refused when it is not
blank: the "1" is refused with "Parameter #4 invalid." and the text that was
written. The place may not be blank, and the speed may not be negative: the
"-1" is refused with "Parameter #3 invalid." and the text that was written.
The place is checked before the speed, and the speed before the fourth
parameter.

The speed of the tree is not carried: the box moves the pointer at its own
speed, and the `MouseMove, 10, 20, 100` is refused when the script runs. The
speed of zero is the speed of nothing, and the tree moves the pointer.

The probe of the block is `egm-review/probe_v1_mouse_move.py`, and it is
twenty-seven cases of the binary and the tree, all the same. The script of
the probe leaves before the command runs, so the mouse of the machine stays
where it is.

---

## 5. The risks

| Risk | Severity | What is done |
|---|---|---|
| **The threads of the v1.** The v1 runs each hotkey in its own thread, with `#MaxThreads` and `#MaxThreadsBuffer`; the v2 is cooperative and has guard rails. | **High** | **Decided: the v1 runs in the cooperative model of the tree**, and the criterion is the one of the rest of the tree — *the dialect decides the names; what each name does is the runtime's, and it is the same in the two dialects*. `Thread` and `Critical` are **commands of the v1** (measured on 2026-10-29) and the v2 has them as commands too: the surface of the v1 carries the two and **refuses them with the reason** that the v2 uses — *"this runtime runs one script in one thread, and it cannot start another."* for `Thread`, and *"this runtime runs one script in one thread, and it cannot make it critical."* for `Critical`. The `SetBatchLines`, which the v1 has and the tree does not, goes to the **pending table**. A real thread model is another project, and this is not it. |
| **The `=` of the v1.** Assignment and comparison with the same token. | High | The parser decides by the position (the statement starts with `=` → assigns; inside an expression → compares). Measured: `"ABC" = "abc"` is true. |
| **`%nome%` is the environment variable in the v1.** | Medium | The tree interpolates the variable of the script. With the v1 dialect it becomes the environment one, and the `#NoEnv` turns the substitution off. It is a change of *sense* of a rule that the tree already has. |
| **The surface does not come out of the binary.** The names of the v1 are obfuscated. | Medium | The list comes from the documentation and is verified by the binary, on the box — §1.3. |
| **Two tables to maintain.** | Medium | The parity test per dialect and the `--surface` make the drift visible. |
| **The wave and the plan on the same `--surface`.** The wave of 2026-10-29 built the command; this plan extends it. | Medium | The extension is a parameter and four questions that come to carry the dialect. There is no second command, and the `AGENTS.md` says which piece is whose. |
| **Two grammars in the same parser.** | Medium | The legacy grammar stays behind the predicate of the dialect; the v2 does not see it. |
| **The license of the AHK_X11.** GPL-2.0. | Low | It is already written in the comparison (the license note): the *behaviour* may be ported, the code may not. This plan only uses the list of names and the measured behaviour. |
| **The `v1` and the `v2` diverging over time.** | Low | The gate is the dialect, not the version: a new table of the v1 (1.1.37.03) is one more row. |

---

## 6. What stays out

- **The threads of the v1** (above): the v1 runs in the cooperative model. The
  names that the v1 has about threads are carried and **refused with the
  reason of the runtime** — `Thread` and `Critical` — and the `SetBatchLines`,
  which the tree does not have, stays **pending**. The directives
  `#MaxThreads` and `#MaxThreadsBuffer` are refused with the same reason.
- **The classes of the v2** ( `Map` , `Array` , `Buffer` , `Func` , `Gui` ,
  `Menu` , `Hotkey` ) do not exist in the v1 — and the v1 does not gain new
  classes.
- **The `Gui`, the `Menu` and the `TrayTip`** : in the v1 they are commands
  and system menus; the tree refuses them (the v2 also refuses the `Gui` ).
- **The Windows registry** ( `RegRead` , `RegWrite` , `RegDelete` ): refused
  with the reason, as in the v2.
- **The COM** ( `ComObj` , `ComValue` ): refused.
- * *The `Control* `**: refused — the box has no accessibility layer (see `
  docs/wayland-toplevel-enumeration.md`).
- * *The * environment variables * as a mechanism* *: the `#NoEnv` is
  accepted, and the substitution is the one of the v1.

---

## 7. How the parity is measured

The mechanics already exist on both sides:

- `crates/ahk/tests/surface.rs` reads the table of the surface and asks the
  tree, name by name, whether it carries it. With the gate, the same question
  runs for the two dialects.
- The **command** also already exists: the wave of 2026-10-29 built the
  `ahk --surface` ( `crates/ahk/src/surface.rs` ), which prints one line per
  name and the summary at the end. The gate does not create a second command —
  it gives it a parameter:

```
ahk --surface --dialect=v1

WinActivate command refusal: this is a name of Windows, and this desktop is not Windows.
MsgBox command value
StringLen function pending: this runtime does not carry that name yet.
...
the surface of v1 has <n> entries: <v> answer with a value, <r> are refused, <p> are pending.
```

The shape is the one of today: the name, the kind, and the state — `value` ,
`refusal: ...` or `pending: ...` , with the reason when there is one. The
three lines are the format: a name refused with the reason, a name that
answers, and a name that the dialect does not yet carry. The numbers of the
summary are what matters — and the middle one is what says what is missing.
The surface of the v1 has been counted (2026-10-29, §1.6): **383 names of the
surface** , of which 266 the v2 has as well, 117 are only of the v1 and 21
are the same name with another kind. The summary of the v1 will say the same
thing the one of the v2 says, with the number of the dialect in it.

The baseline is the one of the v2, at the commit `feb0d5a` (2026-10-29):

```
the surface of v2 has 725 entries: 502 answer with a value, 223 are refused, 0 are pending.
```

The tree moved after that baseline: the prefix of the built-in variables was
closed on 2026-10-29 (`c391430`) , and the surface is 725 entries with **506**
values and **219** refusals.

The 223 refusals are almost all names of Windows — and the refusal is the
right answer for them. The rule of the plan is on the other side: no name that
the v1 does and that the box can do should be in the refusal for lack of
implementation.

The JSON, for the CI, is a piece of F5: the command of today prints the text.

The second number:
**how many names of the surface of the v1 are really implemented** . The
honest rule is the same as the one of the comparison with the X11 —
*no name that the v1 does and that the box can do should be in the refusal for lack of implementation*
.

---

## 8. The summary

| | |
|---|---|
| **The gate** | one value: `--dialect=v1`, the v2 by default |
| **The shape** | the `Dialect` is a trait; the tables are data; the parser and the runtime ask |
| **The phases** | F0 the gate · F1 the surface · F2 the grammar · F3 the semantics · F4 the box · F5 the verification |
| **The expensive phase** | F2, the legacy grammar |
| **What does not change** | the v2, the `Desktop`, the box, the report, the guard rails |
| **The classes of the v1** | none: the v1 has objects and script classes, not runtime types |
| **The divergence that stays written** | the v1 runs in the cooperative model: no threads |
| **The authority** | the binary of the v1.1.37.02 and the documentation of the v1 |
| **The dependency** | the `--surface` of the wave of 2026-10-29, with one dialect more |
| **The coordination** | the wave ports capabilities into the v2; this plan adds the dialect beside it |
