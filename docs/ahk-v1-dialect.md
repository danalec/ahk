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
| **F5 — the verification** | Done | The parity test per dialect (`tests/surface.rs`), the suite of the v1 (`tests/v1.rs`, 146 cases), and the `--surface --json` for the CI. |

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
the pid of the process by the block of the pid (2026-11-02).

**The parameter of a command of the v1.** The v1 checks the value of the
parameter when the script loads, and the script does not run at all: the
`Exit, 3` that comes before a command with a bad parameter does not run. The
complaint of the v1 is "Parameter #N invalid." with the text that the script
wrote -- the text of the parameter, and not its value -- and the command
that reads one parameter takes the whole rest of the line. The rule is one
hook of the dialect (`the_refusal_of_an_invalid_parameter`), the rules are
in `crates/ahk/src/v1/parameters.rs`, and the commands that the v1 does not
check have handlers of their own. The name of a key and the name of the
label of the `SetTimer` are of other families, and they are not carried.

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

The surface of the v1 has **405** entries: **296** answer with a value,
**108** are refused and **1** is pending. The surface of the v2 did not
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
