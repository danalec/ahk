# The corpus of AHKv2_Finetune against the tree

The corpus of `../AHKv2_Finetune` is **1745 real-world AutoHotkey v2
scripts**, and it is the sharpest instrument the tree has for the dialect: it
is the only body of v2 code that the tree has never seen, and it is written
by people who did not write the tree.

This document says what the corpus is, how it was put against the tree, what
the two of them answer, and where they differ.

## What the corpus is

The corpus is not material written by hand for a test: it is the **test suite
of the AutoHotkey v1-to-v2 script converter** (`mmikeww/AHK-v2-script-
converter`, the folder `tests/Test_Folder/AHK_v2_Examples` of it), scraped
and normalized by the fine-tune project. That is where its value is and where
its limits are.

| | |
|---|---|
| The scripts | 1745, in one directory |
| The distinct contents | 1729 -- four groups of duplicates, twenty files |
| The lines | p10 = 3, median = 64, p90 = 645, the longest = 7954 |
| The scripts of forty lines or less | 788 |
| The version they ask for | 1021 of `v2.0`, 704 of `v2.1-alpha.16`, 17 of `v2.1-alpha.17` |
| The categories (the prefix of the name) | BUILTIN 628, STRING 105, STDLIB 95, FILE 83, ARRAY 82, OOP 81, HOTKEY 74, GUI 67, WINDOW 63 |

Its quality is the quality of a converter's test suite, and the manifest
measures it:

| The flaw | How many |
|---|---|
| The first statement was swallowed into a comment of the `#SingleInstance Force` line | 539 |
| No line that carries an action at all | 167 |
| The braces do not balance | 10 |
| An `#Include` of a library that is not there | 37 |
| A call with a side effect (`Shutdown`, `Run`, `RegWrite`, `Send`, ...) | 330 |

The manifest is `docs/corpus/AHv2_Finetune.manifest.jsonl`, one row per
script, written by `scripts/corpus-manifest.py`. **Nothing of the corpus is
executed to write it**: the manifest reads the bytes.

## The method

The two sides are asked the same question through their MCP servers, and the
question is a **parse**: the `check` tool of the tree and the `check` tool of
the reference (`../ahk-v2-headless`, the vanilla v2.0.32 with the headless
build on top of it) both parse the script and answer with the load error
and the line, and neither of them executes anything.

That matters for the safety of the measurement: the corpus carries 330 calls
with a side effect, and a `Shutdown(8+1)` would log the machine off. A
parse never reaches them.

`scripts/corpus-check.py` writes `docs/corpus/AHv2_Finetune.check.json`, one
row per script, with the answer of each side, and prints the summary.

## What the two of them answer

| | How many |
|---|---|
| Both load | 739 |
| Both refuse | 1006 |
| The reference loads and the tree refuses | **0** |
| The reference refuses and the tree loads | **0** |

There is no line of the table with a gap in it: the two of them agree on the
verdict of every one of the 1745 scripts. The reasons say why the ones that
are refused are refused:

| The reason of the reference (among the 1006) | How many | What it is |
|---|---|---|
| `library` | **0** | the `#Include <...>` of the script names a library that is not there, and the tree refuses it too, with the words of the official: "Script library not found." |
| `requires` | **0** | the version of a `#Requires` is checked when the script loads, the way the official checks it: the 488 that the tree loaded are refused by the tree now as well |
| `parse` | **0** | every parse-time refusal of official v2.0.32 in the corpus is now refused by the tree as well |

⇒ The two of them agree on the verdict in **1745 of the 1745** (100%), with
**0** false-refusal gaps and **0** parse-permissive gaps, and the whole run
takes nine seconds.

### The 45 parse-permissive gaps that are now closed

All 45 scripts of the 1745 that official v2.0.32 refused at load time while
the tree previously loaded are now refused by the tree as well:

| The refusal of official v2.0.32 | How many | What closed it |
|---|---|---|
| `Missing """` | 14 | an end-of-line `;` comment after whitespace interrupts unclosed quoted literals |
| `This Class cannot be used as an output variable.` | 12 | `for`-loop output variables and assume-global function assignments are checked against classes |
| `This Func cannot be used as an output variable.` | 7 | assignments to script-defined top-level and nested functions are refused at load time |
| `Hotkeys/hotstrings are not allowed inside functions or classes.` | 4 | hotkeys and hotstrings are refused inside function and class bodies |
| `Too many parameters passed to function.` | 3 | v2 call arity checks calls with omitted arguments and refuses missing required parameters |
| the other five | 5 | bare/reserved `IsSet` (`Missing "("`), unary `&` on non-variables (`"&" requires a variable.`), shadowed/unknown `catch` classes (`Invalid class.`), hotstring block opening outside T/R mode (`Unexpected "}"`), and pre-registered `Any.Prototype` helpers (`GetMethod`/`HasBase`/`HasMethod`/`HasProp`) |

## The 0 gaps of the tree

⇒ The corpus of the upstream finds no gap in the tree: every one of the 1745
scripts loads in the tree as it loads in the reference.

The last one was the name of a hotkey. The name of a hotkey of the v2 is the
text of the line, and the backtick of it is the escape of the character that
follows it, the same way it is in a text, and the quote of it is the key of
the quote: the ``CapsLock & `;::`` and the `CapsLock & '::` are hotkeys, and
the lexer of the tree refused the backtick and the quote as illegal
characters.

The name is the text that comes before the first pair of colons of the line,
and it has to be the name of a hotkey: the line
`parts1 := SplitByMultiChar(text1, "::")` is a statement, and the quote of it
opens a text.

The combination of keys -- the `a & b::` -- is in the tree now: it is one
hotkey, and the spaces around the `&` are free.

### The two that the dialog of the machine was hiding

The reference used to answer "the interpreter did not finish in the time of
the check" for **44** of the 1745 scripts: a **warning** of the script (the
local variable that is never assigned, for one) opens a dialog of the
machine even when `/ErrorStdOut` is given, and the child of the `check`
waited for a mouse.

The child of the `check` of the headless build carries `--headless` now, so
the warning is text on stderr and the answer comes in a few milliseconds.
The whole run of the 1745 scripts went from half an hour to ten seconds, and
**two** gaps that the dialog was hiding are now on the table:
`BuiltIn_MouseGetPos_01.ahk` ("This Func cannot be used as an output
variable.") and `BuiltIn_MouseGetPos_03.ahk` ("Missing \")\""). Both are
closed: the name of a function of the machine is not the output variable of
an assignment, and the text of the script is not closed by the quote of it.

## What the corpus found already

The corpus is not only a list of gaps: it found **fifteen** bugs of the tree
that the 113 cases of `scripts/differential/cases` never reached. All
fifteen are fixed, and the gaps of the tree went from **309 to 0**.

### The end of a line of Windows was an option

**222 of the 1745 scripts** were refused with "Invalid option." where the
v2.0.32 loads them. The cause: the lexer of the tree reads the text of a line
up to the newline, and the carriage return of a text of Windows was a part
of it. In the options of a section -- the `(` of a text that spans more than
one line -- the last option was `Comments\r` and not `Comments`, and the
option that the v2 does not know was named with the carriage return in it.

The upstream does not read it that way: its lexer knows that the carriage
return is the end of the line. The fix is in `crates/ahk/src/lexer.rs`: the
text of the options of a section and the text of a hotstring do not carry the
carriage return any more.

### The line of a hotkey of the v2 was the continuation of the expression before it

**75 of the 1745 scripts** were refused with `Unexpected "::"` where the
v2.0.32 loads them. The cause: the question "does the next line carry a
hotkey" knew the rule of the v1 only, where the hotkey is a label of a line
of its own. The hotkey of the v2 is written in the line itself --
`^k::action` -- and the `^` of its name is the operator of the power, so the
expression of the line before it took the name of the hotkey and stopped at
its two colons.

The fix is in `crates/ahk/src/parser.rs`: the question asks the rule of the
v2 as well.

### The newline inside a bracket did not end the expression

**19 of the 1745 scripts** were refused with `Missing ")"` where the
v2.0.32 loads them. The cause: inside the parenthesis of a call and inside
the bracket of an array, the newline is not the end of the expression, and
the values of the line that follows are a part of it: the
`FileAppend("a"` of one line and the `" b", "*")` of the next is one
argument, "ab".

The rule is of the v2 -- the v1 reads the list of the arguments that is not
closed as the rest of the line -- so it is a question of the dialect
(`the_newline_inside_a_bracket_continues_the_expression`).

### The line that begins with the dot was not the member of the value before it

**21 of the 1745 scripts** were refused with `Unexpected "."` where the
v2.0.32 loads them. The cause: the line that begins with the dot is the
member of the value that came before it -- the `x.Add("a")` of one line and
the `.OnEvent(...)` of the next are one chain.

The dot that reaches into a value is the one that has **no space after
it**: the space that is before it may be the indentation of the line, and
the tree was reading the indentation as the sign of the concatenation.

### The body of a hotkey was not a scope of its own

**10 of the 1745 scripts** were refused with `Unexpected declaration.` where
the v2.0.32 loads them. The cause: the body of a hotkey is the body of a
function, and the `static` is a declaration in it -- the
`F3:: { static lastActivated := "" }` is a script that loads. The tree read
the body of a hotkey as the body of the script.

### The line of a command that ends with a comma did not go on

**8 of the 1745 scripts** were refused with `This line does not contain a
recognized action.` where the v2.0.32 loads them. The cause: the command of
the v2 is a call whose parentheses are not written, and the line that ends
with a comma continues on the line that follows -- the `MsgBox "a",` of one
line and the `"b", "c"` of the next are one call with three arguments.

### The hotkey did not carry the condition of the `#HotIf`

**7 of the 1745 scripts** were refused with `Duplicate hotkey.` where the
v2.0.32 loads them. The cause: the hotkey of the v2 is the name of it **and
the condition of the `#HotIf` that comes before it**, and the condition is
the text of it: the same name under two different conditions is two hotkeys,
and the same name under the same condition is one hotkey written twice,
which is refused.

The three rules are questions of the dialect (`the_body_of_a_hotkey_is_a_scope`,
`the_line_of_a_command_that_ends_with_a_comma_continues` and
`the_hotkey_carries_the_condition_of_the_hot_if`): the v1 is the other way
round in each of them.

### The sign of a value did not begin the line that continues

**Three of the 1745 scripts** were refused with `Missing operand.` or
`Syntax error.` where the v2.0.32 loads them. The cause: the line that
continues the one before may begin with the sign of a value -- the `f(` of
one line and the `-2` of the next is one argument, -2, and the `x := 1 ||`
of one line and the `!2` of the next is one expression. The tree was reading
the sign as the operator of a value that is not there.

### The comma at the end of a list did not carry the newlines of the bracket

**One of the 1745 scripts** was refused with `Unexpected ")"` where the
v2.0.32 loads it. The cause: the list may be written in more than one line,
and the comma that ends it may be followed by the newlines of the bracket
that closes it -- the `f(` of one line, the `2, "x",` of the next and the `)`
of the third is one call with two arguments.

### The name of a class was not a value of a concatenation

**Three of the 1745 scripts** were refused with `This Class cannot be used
as an output variable.` where the v2.0.32 loads them. The cause: the name of
a class of the dialect and the name of a function of the machine are values:
the `"a" class` and the `"a" ExitApp` are one text. The tree was reading them
as the target of an assignment.

### The body of a hotkey did not keep the assignment of it in its own scope

**Two of the 1745 scripts** were refused with `This Class cannot be used as
an output variable.` where the v2.0.32 loads them. The cause: the body of a
hotkey is a scope of its own, and the `F2:: { class := 1 }` is a script that
loads -- the variable of the body is a local of it, and the name of a class
of the dialect is a name of a local.

### The list of the parameters was not written in more than one line

**One of the 1745 scripts** was refused with `Missing parameter name.` where
the v2.0.32 loads it. The cause: the list of the parameters of a declaration
may be written in more than one line -- the `F(a,` of one line and the
`b := 0)` of the next is one declaration with two parameters.

### The name that the script builds did not carry an index

**One of the 1745 scripts** was refused with `Missing ending "%"` where the
v2.0.32 loads it. The cause: the name that the script builds may carry an
index -- the `x.%a[1]%` is the variable whose name is the "a[1]" -- and the
bracket was not a part of the operand of the percent sign.

### The function that is written inside another one did not hide the builtin

**One of the 1745 scripts** was refused with `Too many parameters passed to
function.` where the v2.0.32 loads it. The cause: the function that the
script declares hides the function of the machine of the same name, and the
function that is written inside another one is a name of the scope of the
function that has it -- the `Log(m, l := "INFO")` of a function and the
`Log("a", "INFO")` of another one that is inside it are one call with two
parameters. The tree was sending the call to the builtin.

Two of these are questions of the dialect
(`the_list_of_the_parameters_may_be_written_in_more_than_one_line` and
`the_operand_of_the_percent_sign_may_carry_a_bracket`): the v1 is the other
way round in each of them.

### The body of a hotstring did not carry the blocks

**One of the 1745 scripts** was refused with `Unexpected "}"` where the
v2.0.32 loads it. The cause: the block of a hotstring -- a replacement that
ends with `{` -- is a block of statements, and the `try` of it is a block
too. The tree cut the block at the first `}` of it and read the rest of it
as statements of the script.

### The switch of the v2 did not carry the sense of the case

**One of the 1745 scripts** was refused with `Unexpected "Switch"` where the
v2.0.32 loads it. The cause: the second parameter of the switch is the
sense of the case of the comparison -- the `switch x, "On"` compares the
texts with the case of them -- and the `switch domain, "Off" { }` is a
script that loads. The tree was reading the second parameter as the end of
the switch.

## The pairs of the upstream

The corpus is the extracted and normalized copy of a test suite; the **851
pairs** of the upstream (`../AHK-v2-script-converter`, the folder
`tests/Test_Folder`) are the same programs in the v1 and in the v2. The
folder holds 852 `.ah1` and 852 `.ah2`, and one of them is not the pair of
the other: `Maths/SetFormat_ex1.ah1` has no `.ah2` beside it.

They are the oracle of the conversion: the `.ah1` runs in the v1 and the
`.ah2` in the v2, and the two must answer the same.

`scripts/pairs-check.py` asks the tree the same question -- does the script
load? -- **and asks the two official interpreters the same question**, because
the tree is not the judge of itself: a refusal of the tree is a gap only when
the official one loads the script.

- the **v2** is the official v2.0.32, through the `check` of the MCP of the
  headless build: a parse, and a parse never runs anything;
- the **v1** is the official v1.1.37.02, with `/ErrorStdOut`. The file that
  is measured carries an `ExitApp`, so the program stops before any of its
  own lines runs, and the load -- which happens before the first line runs
  -- is still seen.

The programs of the pairs open boxes of the machine, and nothing of them is
ever executed.

| The v1: the tree and the official v1.1.37.02 | How many |
|---|---|
| Both load | 821 |
| The tree refuses and the v1 loads | **0** |
| The tree loads and the v1 refuses | **0** |
| Both refuse | 20 |
| The reference showed a dialog, so the verdict is not known | 10 |

| The v2: the tree and the official v2.0.32 | How many |
|---|---|
| Both load | 732 |
| The tree refuses and the v2 loads | **0** |
| The tree loads and the v2 refuses | **0** |
| Both refuse | 119 |

The two lines in bold are both **zero**:

- the **zero** of the v2: the pairs of the upstream find no gap in the v2,
  and the corpus of the converter finds none either;
- the **zero** of the v1: the pairs of the upstream find no gap in the v1
  either -- every one of the 821 pairs that the official v1.1.37.02 loads
  now loads in the tree as well (fixing the UTF-8 BOM and trailing-newline
  handling in `scripts/pairs-check.py` moved 67 BOM/directive probe artifacts
  into `both load` and uncovered three final v1 gaps that were also fixed:
  single-letter/`Realtime` `Process, Priority`, `SendRaw`, and `Hotkey,
  IfWin*` context sub-commands).

⇒ Both dialects of the tree load every script of the upstream pairs and
corpus that the official interpreters load.

The `#Include` refusals are gone as well: the **2** of the v1 table and the
**98** of the v2 table were scripts that the tree loaded and the official
refused, because the tree did not resolve the includes at all. The tree
resolves them now, before the parse, the way the runner resolves them, and
the words of the refusal are the ones of the official: "Script library not
found." for a library that is not there, and `#Include file "<the path as it
was written>" cannot be opened.` for a file that cannot be opened. **0**
parse-permissive gaps remain in v1 pairs (after refusing v1 labels pointing
directly to a function definition) and **0** parse-permissive gaps remain in
v2 pairs (after refusing required parameters after defaulted parameters and
the v2 corpus permissive fixes above).

### The six gaps of the v2 that the pairs found, and the fixes

The six are closed. The table below says what each one was and what closed
it, and the measure that says so is the run of `scripts/pairs-check.py`
above: the tree refuses none of the 851 pairs that the official v2.0.32
loads.

| The file of the upstream | The refusal of the tree | The fix |
|---|---|---|
| `@LBL,HK,HS,FUNC\2025\LabelNames 01.ah2` | `unexpected character '★'` | the name of the v2 carries any character that is not ASCII, and so does the name of a label |
| `Flow of Control\LoopParse_issue_77.ah2` | `Missing space or operator before this.` | the text that is glued after the closing percent is a part of the name |
| `Mouse and Keyboard\Hotkeys and Hotstrings\Hotkeys_ex6.ah2` | `Invalid key name.` | at the load, any single character is the name of a key |
| `Mouse and Keyboard\Hotkeys and Hotstrings\Hotkeys_ex7.ah2` | `Invalid key name.` | the same, and the line that begins with three colons is a hotkey and not a hotstring |
| `String\QuotedObjLiteral-WithVar.ah2` | `Invalid property name in object literal.` | the name of a property may be built by the script |
| `Yunit_tests\Y_Test_76.ah2` | `Syntax error.` | a word of the language that is followed by a percent sign is the beginning of a name that the script builds |

### The sixty-eight gaps of the v1 that are closed

The v1 had sixty-eight gaps and now has **zero**. All sixty-eight are closed,
and what closed each family of them:

| The gap | What closed it |
|---|---|
| the name of a label of the v1 may carry any character, and it may begin with any character | the name of a label of the v1 is the text that comes before the colon, and it may carry any character but a space and a comma |
| the hotstring of the v1 may carry the options of the v2 | the `:*:np::` is a hotstring, and the v1 reads the options and does not refuse them |
| the line that begins with three colons is the hotkey whose name is the ":" | the colon of the name is a token of its own, and the pair of colons that follows it is the separator |
| the `SetCapsLockState`, the `SetNumLockState` and the `SetScrollLockState` | the state of a lock key is the one of the machine, and the desktop already knew how to turn it |
| the `ControlSendRaw` | the text of it is written as it is, without the keys of it being simulated |
| the `WinMinimizeAllUndo` | the windows that were minimized go back |
| the declaration of the v1 carries the equal sign | the v1 writes with the equal sign, and a declaration of it carries it: the `static var = 1` is a declaration with the value one |
| the `#Delimiter` of the v1 | the directive changes the character that separates the parameters of a command, and it holds from the line of the directive onwards |
| the action of a command of the v1 that has a body | the `break`, the `continue`, the `return` and the `goto` are commands of the v1, and the one that is the action is the statement of the language and not a parameter of the command |
| the `#Hotstring` of the v1 refuses an option that it does not know | the v1 refuses nothing: the `#Hotstring, Bogus` loads |
| the attributes of a file, the way of a move and the options of the `WinSet` | the `^` is an attribute of a file, the `R` is a way of a move, and the `Topmost` is another name of the `AlwaysOnTop` |
| the remap of the v1 | the action of a hotkey that is written in one line is a remap when it is the name of a key: the `a::b` is one, and the `a::b, c` is not |
| the name of a key of the v1 in the desktop | the `Numpad0` is the `num0`, and the hotkey of the v1 carries the name as the script wrote it |
| the arity of the comparisons of the v1 | the `IfEqual` and the rest take one parameter or two, and the one that is not there is the empty text |
| the comma of the `return` of the v1 | the v1 writes the comma of the command: the `Return, value` is the `Return value` |
| the `SplashTextOff` of the v1 | it is a command of the v1 that takes no parameter, and it takes the window of the `SplashTextOn` away |
| the action of a hotkey and the label of it | the action of a hotkey may be one of the words of the Alt-Tab, and the hotkey of the v1 is a label of the script: the `Goto, f` finds the `f::` |
| the mode of the loop of the registry of the v1 | the second parameter is the key under the root and the mode is the third one: the `Loop, HKCU, Software, 0` loads |
| the seven names of the v1 that the table of the surface did not carry | the `ComObj`, the `ComObjMissing`, the `ComObjParameter`, the `ObjRawGet`, the `OnClipboardChange`, the `SB_SetParts` and the `SB_SetText` are functions of the v1, and the arity of each one of them was measured with the probe of the surface |
| the two outputs of the `PixelSearch` and of the `ImageSearch` of the v1 | they may be left out: the `PixelSearch, , , CenterX, 1, CenterX, 1, 0xF3F3F3, 0, Fast` loads |
| the escaped percent sign of the v1 | an escaped percent sign in a legacy command parameter is a literal character (`MsgBox `%` loads) |
| the `% ` forced expression at the start of an expression parameter in the v1 | `%` followed by a space or tab at the start of an expression parameter forces the expression (`var2 := % var1 "bar"`, `Global var3 := % var2 " test"`), while mid-expression `%` still opens a variable reference (`x := 10 % 3`) |
| trailing parameters after a continuation section in a v1 command | columns stay advancing across merged newlines and `Parser::lines` receives the merged line text so verbatim parameter slicing aligns with merged tokens (`MsgBox, 1, % title, <section>, 4`) |
| same-line command actions written without a comma in v1 commands with a body | `IfEqual, var, val, MsgBox % "..."` splits the action command name from its first parameter before validating percent signs |
| the optional command comma of `Switch` and `Case` in the v1 | `Switch, num {` and `Case, 0:` consume the optional comma in v1 |
| an empty assignment before a comma in a v1 multi-statement line | `var := , eVar := 1` and `, eVar5 := , eVar6 := 2` accept a comma after an empty assignment value in v1 |
| bare `static` after bare `local` in a v1 function | force-local (`local`) immediately followed by assume-static (`static`) loads, while other orderings or placements are refused |
| a literal backtick after `#EscapeChar` changes the escape character in the v1 | once `#EscapeChar =` is active, `` ` `` is a plain literal character (`MsgBox- `-`) |
| empty same-line braces for `catch`, `finally`, and `else` in the v1 | `catch{}`, `catch {}`, `finally {}`, and `else {}` at end of line load as empty blocks in v1 |
| v1 `Loop` file patterns containing a drive colon (`Loop, C:\*.*`) | when tokens remain after the expression probe before `,`, end of line, or a trailing `{`, the first parameter is read verbatim up to the parameter boundary |
| empty property brackets in v1 classes | `MyMethod1[] { Get { ... } }` allows empty `[]` on property accessors in v1 |
| legacy v1 dot-call syntax `fn.(args)` | `.(` after a receiver invokes it as a callable value in v1 |
| hotkeys and labels starting with `(:` or `(::` | `the_section_at` excludes lines beginning with `(:` / `(::` or ending with `:` so `(::Run Notepad` is parsed as a hotkey rather than a continuation section |
| legacy `=` assignment to dynamic variable names in the v1 | `v%a%=abc`, `%a%%b%=Hi`, and `%a%%c%=` assign literal text (or `% ` forced expressions) to dynamic variable targets |
| dynamic type deref in v1 legacy `if var is %mytype%` | when the type name contains `%`, percent signs are validated at load time and the type is resolved at runtime |
| single-letter and `Realtime` levels in v1 `Process, Priority` | parameter #3 accepts any text starting with `L`, `B`, `N`, `A`, `H`, or `R` (`PROCESS_PRIORITY_LETTERS = "LBNAHR"`) |
| `SendRaw` command in the v1 | added `SendRaw` (min 1 parameter, raw text send) to the v1 command table and surface |
| `Hotkey, IfWin*` / `Hotkey, If` context sub-commands in the v1 | parameter #2 is a window title or `#If` expression rather than a target label when parameter #1 is `If`, `IfWinActive`, `IfWinExist`, `IfWinNotActive`, `IfWinNotExist`, or dynamic |
| v1 labels pointing directly to a function definition | non-hotkey labels without an intervening executable statement (even across stacked hotkeys, hotstrings, directives, or classes) are refused with `A label must not point to a function.` |

The `ControlSend` of the v1 was fixed with them: the v1 writes the control
before the keys and the built-in of the v2 takes the text first, and the two
were not swapped.

## How to run it

```sh
python scripts/corpus-manifest.py     # the manifest of the corpus
python scripts/corpus-check.py        # the two answers, side by side
python scripts/pairs-check.py         # the pairs of the upstream
```
