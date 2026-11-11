# The dialects: one binary, two languages

This binary speaks two dialects of AutoHotkey -- **v2.0.19 by default** and
**v1.1.37.02 behind a flag** -- and this document is the gate between them,
what each dialect does differently, and how to move a script from one to the
other. Every rule below is either measured on the binary named in the text or
read from the code named at the end.

- [The gate](#the-gate)
  - [`--dialect`](#--dialect)
  - [What the gate changes, and what it does not](#what-the-gate-changes-and-what-it-does-not)
  - [How the tree is built for it](#how-the-tree-is-built-for-it)
  - [The surface, dialect by dialect](#the-surface-dialect-by-dialect)
- [The two dialects at a glance](#the-two-dialects-at-a-glance)
- [1. The syntax](#1-the-syntax)
  - [Commands, and the commas](#commands-and-the-commas)
  - [What a line is](#what-a-line-is)
  - [The assignment](#the-assignment)
  - [The values: `%name%` and expressions](#the-values-name-and-expressions)
  - [The text of a value](#the-text-of-a-value)
  - [The control flow](#the-control-flow)
  - [Labels, `goto` and `gosub`](#labels-goto-and-gosub)
  - [Hotkeys and hotstrings](#hotkeys-and-hotstrings)
  - [Classes](#classes)
  - [The directives](#the-directives)
- [2. The semantics](#2-the-semantics)
  - [Numbers](#numbers)
  - [Truth](#truth)
  - [The operators the two dialects do not share](#the-operators-the-two-dialects-do-not-share)
  - [The members, the items and the arrays](#the-members-the-items-and-the-arrays)
  - [Objects and classes](#objects-and-classes)
- [3. The names](#3-the-names)
  - [The built-in variables](#the-built-in-variables)
  - [The names of commands that the two do not share](#the-names-of-commands-that-the-two-do-not-share)
  - [The functions that are one name and two meanings](#the-functions-that-are-one-name-and-two-meanings)
- [4. Scoping](#4-scoping)
- [5. The errors](#5-the-errors)
- [6. The libraries](#6-the-libraries)
  - [Where a name lives, and the `--surface`](#where-a-name-lives-and-the---surface)
- [Porting a v1 script to v2](#porting-a-v1-script-to-v2)
  - [The order to do it in](#the-order-to-do-it-in)
  - [The translation table](#the-translation-table)
  - [What has no direct v2 word](#what-has-no-direct-v2-word)
- [Porting a v2 script to v1](#porting-a-v2-script-to-v1)
- [What the v1 deliberately does not have](#what-the-v1-deliberately-does-not-have)
- [Where this rests](#where-this-rests)

---

## The gate

### `--dialect`

The gate is one flag of the runner, `ahk`:

| Command | The dialect |
|---|---|
| `ahk` | v2, the dialect of the tree |
| `ahk --dialect=v2` | v2, spelled out |
| `ahk --dialect=v1` | v1, the legacy dialect |
| `ahk --dialect=v3` | refused before the script is read: `the dialect v3 is not one this tree speaks: the tree speaks v1 and v2.` and exit 2 on standard error |

The case of the value does not matter, and the flag is read from the command
line before standard input is touched. The same flag reaches `--surface`:

```sh
# The names of the dialect the run speaks, with the state of each one.
echo '' | ahk --surface --dialect=v1 --json
```

```json
; the summary of the two dialects, measured with the two commands
{"dialect": "v2", "total": 833, "values": 685, "refusals": 148, "pending": 0}
{"dialect": "v1", "total": 408, "values": 302, "refusals": 106, "pending": 0}
```

A name that the tree does not carry yet would be `pending`; neither table has
one, and a name that the desktop of this machine cannot answer is
`refusal`, with the reason it gives.

### What the gate changes, and what it does not

A dialect is a value the parser and the runtime ask, and the answer changes
five things and nothing else:

| The gate changes | The gate does not change |
|---|---|
| the grammar the parser reads | the runner, its flags and its report |
| the built-in table the parser matches names in | the `Desktop` and the primitives of the box |
| the built-in variables the runtime answers | the guard rails: the steps, the iterations, the depth, the timeout |
| the defaults of the settings of the script | the exit codes: 0, 1 and 2 |
| the rules of the runtime that are not the same in the two grammars | the files, the pixels, the keyboard: one implementation for both |

So a v1 script and a v2 script drive the same machine through the same code,
and the report of both has the same shape.

### How the tree is built for it

`Dialect` is a trait in `crates/ahk/src/dialect.rs`, with two implementors,
`V2` and `V1`. Every rule that is not the same in the two dialects is one
method of the trait, and its doc comment is the measurement behind it:

```rust
/// The legacy command: a name at the start of a line, a comma, and the
/// parameters of the command. Measured on the binary of v1.1.37.02 on
/// 2026-11-01: `FileAppend, hello, *` writes `hello`, and the
/// parameters of a command of the v1 are texts and not expressions.
fn legacy_commands(&self) -> bool;
```

A method that the v2 answers with `false` is a rule the v1 alone has, and a
method that the v1 answers with the other value is a rule the v2 alone has.
The tables are data as well: `V1::builtins` is `crate::v1::BUILTINS`,
`V2::builtins` is `crate::builtins::BUILTINS`, and `tables()` names the two
TSV files the `--surface` reports read.

The dependency runs one way, and the gate does not break it: `ahk-cli` asks
for a dialect, the parser and the runtime consult it, and the language crate
still does no I/O. The v1 part of the language is `crates/ahk/src/v1/` -- a
table (`table.rs`) and the rules of the dialect that its commands need
(`parameters.rs`, `sub_commands.rs`, `labels.rs`, `numbers.rs`, `strings.rs`,
`kinds.rs`, `names.rs`, `loops.rs`, `variables.rs`, `chars.rs`, `dates.rs`,
`files.rs`, `hotkeys.rs`, `keys.rs`, `math.rs`).

### The surface, dialect by dialect

`ahk --surface` prints one line for every name of the dialect the run
speaks, `ahk --surface --json` prints the same as JSON, and
`tests/surface.rs` asks the tree, name by name, whether it carries what the
table says it should. The tables are `crates/ahk/reference/`:

| File | What it is |
|---|---|
| `ahk-v2.0.19.surface.tsv` | every name of the v2: its kind, its class, the address of its handler in the binary |
| `ahk-v2.0.19.pending.tsv` | the names of the v2 the tree does not carry: empty |
| `ahk-v1.1.37.02.surface.tsv` | the surface of the v1 as measured on the binary: the kind, the arity and the shape of each parameter (`Value`, `Variable` or `Output`) |
| `ahk-v1.1.37.02.pending.tsv` | the names of the v1 the tree does not carry: empty |

The `Value`, `Variable` and `Output` shapes are the v1 convention, and they
are the reason a v1 command cannot be written as a v2 call without thinking:
`StringLen, n, x` writes into `n`, and `n` is a **name**, not a value.

---

## The two dialects at a glance

| | v2.0.19 | v1.1.37.02 |
|---|---|---|
| every statement | a call or an expression | a command, or a call |
| `MsgBox "hi"` | yes | no: `MsgBox, hi` |
| `MsgBox, hi` | no: a syntax error | yes |
| the parameters of a command | expressions | **texts**, with `%name%` substitution |
| `x := 5` | assigns | assigns |
| `x = 5` | compares | assigns the text `5` |
| `x += 1` | arithmetic | arithmetic on a variable, and the rest of the line is a list |
| a variable with no value | an error | the empty text |
| `1 / 0` | an error | the empty text |
| the classes of the error | fourteen | none |
| catching a failure | `try` / `catch` | `OnError` |
| a command that fails | the failure stops the statement | `ErrorLevel` is set, the script goes on |
| the class types | `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey`, `Enumerator`, the errors | none |
| an object of a class of the script | an instance of it | an object with no class |
| the built-in variables | `A_` prefixed (`A_Clipboard`) | `A_` prefixed, and four without (`Clipboard`, `ComSpec`, `ErrorLevel`, `ProgramFiles`) |
| the settings of the script | `CoordMode "Mouse", "Screen"` | `CoordMode, Mouse, Screen` |
| a hotkey | `Hotkey "^a", Fn` | `^a::Fn()` or a label |
| a hotstring | `Hotstring ":btw:", Fn` | `::btw::by the way` |
| the default `CoordMode` | `"Client"` | `"Window"` |
| the default `SendMode` | `"Input"` | `"Event"` |
| the default `TitleMatchMode` | 2 | 1 |

---

## 1. The syntax

### Commands, and the commas

v1 is a language of commands: a name at the start of a line, a comma, and the
parameters of the command as text. v2 has no commands at all -- every name is
a function that is called, and a name at the start of a line without
parentheses is a call statement.

```ahk
; v1: the comma belongs to the command, and the parameters are texts.
FileAppend, hello world, *
```

```ahk
; v2: the same write is a call with two expressions.
FileAppend "hello world", "*"
```

The v1 parameter is the **text** of it, and the quotes are part of the text:

```ahk
; v1 writes the quotes too, because a command parameter is a text.
FileAppend, "hello", *
; the output line is: "hello"
```

A command of the v1 that reads one parameter takes the whole rest of the
line, and the commas that are in it stay:

- `AutoTrim, On, extra` is the parameter `On, extra`, which the command then
  refuses with `Parameter #1 invalid.\n     Specifically: On, extra`.
- `FileAppend, hello world, *` is two parameters: `hello world` and `*`.

A `,` after the name is optional in the v1: `Loop, 3` and `Loop 3` are the
same statement, and the tree reads both.

**A v2 script with a comma after the name does not load:**

```json
; echo 'MsgBox, "hi"' | ahk
{"column": 7, "error": "Function calls require a space or \"(\". Use comma only between parameters.\n     Specifically: MsgBox, \"hi\"", "error_line": 1, "ok": false}
```

And **a v1 script with a function call where a command is expected does not
load**, because a function is not a command:

```json
; echo 'StrLen "abc"' | ahk --dialect=v1
{"error": "This line does not contain a recognized action.\n     Specifically: StrLen \"abc\"", "ok": false}
```

The rule underneath is a method of the dialect: `the_command_has_to_be_a_command`
is `false` in the v2 and `true` in the v1, and a line of the v1 that begins
with a name that is a function of the v2 but not a command of the v1 is a
line that holds no action.

### What a line is

| | v2 | v1 |
|---|---|---|
| a line that ends with an operator | continues on the next line | does not: `x := 1 +` then `2` is two lines |
| a line that **begins** with an operator | does not join | joins the line before it |
| a line that ends with `(`, `{` or `,` | continues | the bracket and the comma do not |
| a comment | `;` to the end of the line | `;` to the end of the line (the flag is `#CommentFlag`) |
| a block comment | `/* ... */` | a `/*` in an expression is a missing operand |
| a name with `` ` `` | refused: the backtick is the escape of a text | the escape is expanded before the name is read: `` a`c `` is the name `ac` |
| a name with `#`, `$`, `@` | refused | allowed |
| a name that is a reserved word | refused: `while := 5` | allowed: `while := 5` writes the variable `while` |
| a name that begins with a number | refused | allowed: `0 := 9` writes the variable `0` |
| a single-quoted text | `'abc'` is a text | refused: `The leftmost character above is illegal in an expression.` |
| the quote of a text | `` ` `` escapes, `""` is two texts | `""` is a quote inside a text: `"a""b"` is `a"b` |
| the same quote in the line of a command | -- | the parameter is a text and the quotes are part of it: `MsgBox, "quoted"` shows `"quoted"`, quotes included |
| a text that is not closed | `Missing "\""` at the load | `Missing close-quote`, except in the line of a command |

```ahk
; v1: the line that begins with an operator joins the one before it.
x := 1
+ 2
FileAppend, [%x%], *
```

The two dialects disagree on which direction a line continues, and the v1 is
the one that goes the other way: a line that **begins** with an operator is
merged with the line before it, and a line that ends with one is not merged
with the line after it.

**The quote is the most common trap.** In the v1, `"a""b"` is the text
`a"b`; in the v2 it is the text `a` followed by the text `"b"`, and the
parser refuses the pair because there is no space between them:

```json
; echo 'x := "a""b"' | ahk
{"error": "Missing space or operator before this.\n     Specifically: \"b\"", "ok": false}
```

and the way to write the same text in the v2 is `"a``"b"` -- the backtick
escapes the quote.

### The assignment

| | v2 | v1 |
|---|---|---|
| `x := 5` | assigns the number 5 | assigns, and the value is an expression |
| `x = 5` | compares `x` with 5 | assigns the **text** `5`, with `%name%` substitution |
| `x :=` with nothing after it | continues on the next line | assigns the empty text, and the next line is a statement of its own |
| `x += 1` | `x := x + 1` | a command of its own: `EnvAdd`; the rest of the line is a list and its value is thrown away |
| `x := "abc"` | the text | the text, and `x` is a **pure** value |

The legacy assignment of the v1 is the one that differs most:

```ahk
; v1: the equal sign assigns, and the right side is a text with %name% in it.
x = 5
TEMP = %TEMP%
FileAppend, [%x%] [%TEMP%], *
```

The right side is a **text**: `x = "abc"` leaves `x` holding `"abc"`, with the
quotes as characters, because a legacy assignment is a command parameter
and the quotes of a command parameter are part of the text.

In the v2 the same line compares: `x = 5` is a comparison, and the value of
it is thrown away, so the script does nothing. A v2 script that meant to
assign and wrote `=` is a script with a mistake that **does not fail**.

The tree also knows the compound forms, in both dialects:

| The form | v2 | v1 |
|---|---|---|
| `x += 1`, `x -= 1`, `x *= 1`, `x /= 1` | arithmetic on the value of `x` | `EnvAdd`, `EnvSub`, `EnvMult`, `EnvDiv` |
| `x .= "a"` | concatenates | the same |
| `x := "abc"` then `x = 5` | a comparison | `x` is the text `5` |

### The values: `%name%` and expressions

The v1 substitutes `%name%` in a command parameter and in the legacy
assignment, and a `%` that does not close is a load error:

```json
; echo 'MsgBox, 50% %x%' | ahk --dialect=v1
{"error": "This parameter contains a variable name missing its ending percent sign.\n     Specifically: 50% %x%", "ok": false}
```

`%%` is the escape for a literal `%` in a command parameter, and a name that
is empty is refused with `Empty variable reference (%%).`

The `%x%` of an **expression** is the v1's dynamic name: `x := %name%` reads
the variable whose name is the value of `name`. In the v2, `%name%` does not
exist, and `%` is a member of `Format` and the escape of the hotstring.

A name that the script builds and that is not there is answered in the v1 --
it creates the variable and reads it, which is the empty text -- and the v2
refuses it: `Variable not found.`, with the name in the `Extra`.

The **`()`** of a command parameter is a text as well: `FileAppend, (a, b), *`
writes `(a, b)` in the v1; in v2 it is the two expressions `a` and `b`.

**The environment lookup.** A `%name%` of the v1 that is not a variable of
the script is looked up in the environment of the machine:

```ahk
; v1: TEMP is not a variable of the script, so it is the environment's.
FileAppend, %TEMP%, *
; v1: and #NoEnv turns the lookup off, so the same line writes nothing.
#NoEnv
FileAppend, %TEMP%, *
```

The v2 does not look at the environment for a variable, and the command that
reads it is `EnvGet "TEMP"`.

### The text of a value

| The value | v2 | v1 |
|---|---|---|
| `1/3` as a text | `0.3333333333333333` | `0.333333` (six decimals, the default of `SetFormat`) |
| `7/2` | `3.5` | `3.500000` |
| `0.1 + 0.2` | `0.30000000000000004` | `0.300000` |
| `1e2` | `100.0` | the empty text (`1e2` is a name) |
| `1.0e2` | `100.0` | `1.0e2` |
| `0x10` | `16` | `0x10`, and `StrLen(x)` is `4` |
| `100000000000000000000` | a float | `9223372036854775807` |

The v1 number is a **pure** number: it keeps the text that was written, and
that text is what the value answers when it is read as a text. The pure
numbers are the float, the hex and the integer with zeros in front
(`007` stays `007`). `SetFormat` changes how the numbers are written, and
`A_FormatInteger` and `A_FormatFloat` say what is in force.

### The control flow

| The construct | v2 | v1 |
|---|---|---|
| `if (cond)` with a body | one statement or a block | one statement or a block |
| the legacy `if` | none | `if x = 5`, `if x > 5`, `if x contains a`, `if x in a,b`, `if x between 1 and 10`, and `not` before any of them |
| the text after the `if` on its own line | refused: `Unexpected "If"` | read and **not run**; the body is the next line |
| an `if` with no body at all | a load error | a script that loads |
| `else` | a word of the `if` | a command of its own, with no comma |
| `while` | a condition and a body | a condition and a body |
| `loop` with an expression | `Loop 3` or `Loop 3 { }` | `Loop 3`; the count is a **number, not an expression** |
| `loop n` where `n` is a variable | three turns if `n` is 3 | **no turn at all**: the count is the text `n` |
| `loop 1+1`, `loop 2.7` | a turn count | no turn |
| a `loop` with no body | a load error | the script ends at the loop |
| `for x in source` | the **values** | the **keys**: `for i in [10, 20]` gives 1 and 2 |
| `for x in 5` | an error | no turn, and no failure |
| `until` | the condition of a loop | the condition of a loop, and `UNTIL with no matching LOOP` when there is none |
| `try`, `catch` | yes | the `try` block runs and so does a `finally`; a `catch` is refused |
| `switch` and `case` | yes | a `switch` that is written as the v2 writes it is read as a call, and it loads |
| a line that is only `true`, `false`, `not`, `and`, `or` | a call: `not()` | a line that holds no action, refused with its own text |

```ahk
; v1: the legacy if compares the value of x with the text that follows.
x := 10
if x > 5
    FileAppend, [big], *
else
    FileAppend, [small], *
return
```

The legacy `if` is not an expression: `if x = 1 + 1` compares the value of
`x` with the **text** `1 + 1`, and not with 2. The v2 reads `x = 1 + 1` as a
comparison with the number 2.

A construct of the v1 may also come **inside** the condition line of another
one: `if (0) if (1) x := 1` loads in the v1, and the v2 refuses the inner
`if`.

The `A_Index` of a loop is a built-in variable in both dialects: it counts
the turns from one, and a nested loop has an index of its own.

### Labels, `goto` and `gosub`

| | v2 | v1 |
|---|---|---|
| a label, `Name:` | yes | yes |
| `Goto Name` | yes | yes, and `Goto, Name` as a command |
| `Gosub Name` | no, and the v1 does not carry it either: `gosub Label` is a line that holds no action | -- |
| the refusal of a label that is not there | `Label not found in current scope.` | the refusal of the v1, at the end of the load |
| the refusal of a jump into a block | `A Goto must not jump into a block that doesn't enclose it.` | `A Goto/Gosub must not jump into a block that doesn't enclose it.` |
| a label that a command names | none | `GroupAdd` (the fourth parameter), `Hotkey` (the second), `SetTimer` (the first), checked at the load |

```ahk
; v1: a label, and a jump to it.
goto Start
FileAppend, [skipped], *
Start:
FileAppend, [here], *
```

### Hotkeys and hotstrings

| | v2 | v1 |
|---|---|---|
| a hotkey | `Hotkey "^a", Handler` | `^a::Handler()` on a line of its own, `^a::` with the statements under it, or `^a::Label` |
| the action of a hotkey | a function object | a **label**, or the name of a function |
| a hotstring | `Hotstring ":btw:", Handler` | `::btw::by the way` as a statement |
| the end of the section that runs | -- | the first hotkey or the first hotstring: the code after it does not run |
| the section that runs | the whole script | the code before the first hotkey |

The v2 has no section that ends: a hotkey is a statement like any other, and
the script runs to its end. The v1 stops at the first hotkey and registers
it there; the code that comes after it runs only when a key or a hotstring
fires, and only through a label or a function.

```ahk
; v1: a hotkey, written the v1 way, with the statements under it.
^a::
   FileAppend, [fired], *
   return
```

**The hotkeys need a keyboard to watch.** On a machine that is not Linux
there is none, and both dialects answer
`the hotkeys of this box watch a Linux device; this is not Linux`. The
registration is a step of the report, and the failure comes with it.

### Classes

| | v2 | v1 |
|---|---|---|
| the moment of construction | `C()` | `new C()` |
| the `new` word | not a word: assignment to a variable named `new` fails with `This global variable has not been assigned a value.` | a word, and `new := 5` is allowed |
| a static method | yes | refused: `Invalid class variable declaration.\n     Specifically: __Init() {` |
| the members of the body | one name of each kind | one set of names: a method and a variable of the same name are a duplicate |
| the base of a class | a class, and `Object` is one | a class of the script: `class C extends Object` is `Unknown class.\n     Specifically: Object` |
| the name of a class | not a reserved word | any text: `class if { }` is a class of the v1 |
| the class of an object of a class | its class | `"C"`, and the `__Class` of a plain object is empty |
| `this` | yes | yes |
| `super` | yes | no: the word is `base` |
| the class of the class object | `"Class"` | -- |
| the getter and setter | a block or an expression after `=>` | a block only |
| a duplicate member | `Duplicate declaration.\n     Specifically: C.Prototype.x` | `Duplicate declaration.\n     Specifically: x` |
| a class declared twice | `This class declaration conflicts with an existing Class.` | `Duplicate class definition.\n     Specifically: C` |
| a function declared twice | `This function declaration conflicts with an existing Func.` | `Duplicate function definition.\n     Specifically: f()` |
| a class and a function of the same name | refused | allowed, and the call goes to the function |
| the default of a parameter | `b := 1` | `b := 1` or `b = 1` |

The v1 class is a variable of the script, and the `new` reads it; the class
of a class is not an entity of its own in the same way, so `C.Say()` calls
the method of the class object with the class as `this` in the v1, and the
v2 refuses it with `This value of type "Class" has no method named "Say".`

### The directives

A directive is a `#name` at the start of a line. The two dialects share most
of the set, and each has some of its own.

| Directive | v2 | v1 |
|---|---|---|
| `#Warn` | yes, with the types `VarUnset`, `LocalSameAsGlobal`, `Unreachable` | yes, with the types `UseUnsetLocal`, `UseUnsetGlobal`, `UseEnv`, `LocalSameAsGlobal`, `ClassOverwrite`, `Unreachable` |
| `#NoEnv` | not a directive of the v1's meaning: the environment is not read | yes, and it turns the `%name%` lookup of the environment off |
| `#Persistent` | -- | yes, and it does nothing here: the runner is one process |
| `#CommentFlag` | -- | `;` only, and the flag that is not `;` is refused with the reason |
| `#EscapeChar` | -- | refused with the reason: the comment flags are not carried |
| `#If`, `#IfWinActive` | not carried | the directives of the v1, and the tree refuses them with the reason of the hotkeys |
| `#MaxThreads`, `#MaxThreadsBuffer`, `#MaxThreadsPerHotkey` | not carried | the directives of the v1, and the tree refuses the threads with the reason |
| `#SingleInstance`, `#Requires`, `#UseHook` | the v2 set | the v1 set, plus `#InstallKeybdHook`, `#InstallMouseHook`, `#KeyHistory`, `#MaxHotkeysPerInterval`, `#HotkeyInterval`, `#HotkeyModifierTimeout`, `#InputLevel`, `#MenuMaskKey`, `#MaxMem` |

A directive that the dialect does not know is not a directive at all: the
line that holds it holds no action, and the load refuses it.

---

## 2. The semantics

### Numbers

| Case | v2 | v1 |
|---|---|---|
| `1 / 0` | an error, `Divide by zero.` | the empty text |
| `1 + "a"` | an error, `Expected a Number but got a String.` | the empty text |
| `"5" + 1` | the number 6 | the text `51` (the `+` of a text is a concatenation) |
| `"abc" + 1` | an error at the load | the text `abc1` |
| `"5abc" + 1` | an error | the text `5abc1` (a literal is not a number, so the `+` concatenates) |
| `0x10 + 1` | 17 | 17, and the value is still written `0x10` if it is written |
| the number that does not fit | a float | the largest that fits, `9223372036854775807` |
| `7 / 2` | `3.5` in a text too | `3.500000` in a text, and 3.5 when it is counted with |
| `Mod(7, 0)` | an error | the empty text |
| the integer format | the text as written | `SetFormat, Integer, H` makes `255` the text `0xFF` |

The `+` of the v1 is the one that surprises: if either side of the `+` is a
text that is not a number, the answer is the concatenation of the two texts,
and a value that is not a number where a number is wanted makes the
operation blank instead of failing.

### Truth

| The value | v2 | v1 |
|---|---|---|
| the text `"0"` | false | false |
| the text `"00"` | **true**: only the text `"0"` is false | false: the text is a number, and the number is zero |
| the text `"0.0"` | **true** | false |
| the text `""` | false | false |
| `("00")` in a condition | true | `!("00")` is 0, because a **literal** text is a string and a string is true when it is not empty |
| `x` holding `"00"` | true | false |
| `"a" && "b"` | the last one that was true: `"b"` | 1 |
| `"a" || "0"` | `"a"` | 1 |
| `!x` | the number | the number |

The v1 distinguishes a **literal** text from the value of a variable: a
literal is a string, and a string is true when it is not empty, while the
value of a variable is a number when it can be one. The v2 has no such
difference, and for the v2 the rule is one line: the text `"0"` is false and
every other text is true.

### The operators the two dialects do not share

| Operator | v2 | v1 |
|---|---|---|
| `=` | compares, and the case does not matter: `"ABC" = "abc"` is 1 | compares in the legacy `if` only, and the case does not matter there either |
| `==` | compares the case: `"ABC" == "abc"` is 0 | the same |
| `<>` | refused: a missing operand | `!=`: `x <> 2` is 1 when `x` is 1 |
| `!==` | refuses the wrong values | -- |
| `~=` | a regex match | -- |
| `??` | the v2 operator | -- |
| `5--3` | 5 minus `-3` = 8 | the decrement of the variable `5`, then `3`; the statement is a list |
| `y := x ++` | refused at the load: `Unexpected "++"` after the call of `x` | the increment of `x`, and `y` is the value that `x` had before it |
| `++5` | refused at the load | the empty text |
| `0++` | an error | the increment of the variable `0` |
| the `and` / `or` / `not` words | the operators | the operators |
| the `&&` / `||` of two values | the operand | 1 or 0 |

### The members, the items and the arrays

| The read | v2 | v1 |
|---|---|---|
| `a.Length` of an array | the number of the places | the empty text: the v1 counts with `a.Length()` and `a.Count()` |
| `a[5]` where `a` has two places | an error, `Invalid index.` | the empty text |
| `a[5] := 7` where `a` has two places | an error, `Invalid index.` | the array grows, and the places that are not there are holes |
| `a["b"]` of an array | an error, `Invalid index.` | the property `b` of the object |
| `o["b"]` of a plain object | an error: a plain object has no `__Item` | the property `b` |
| `o.HasProp("b")` | 1 or 0 | the empty text: the object of the v1 has none of the eleven methods of the base object |
| `o.Insert(3)` of an array | the v2 name is `InsertAt` | the v1 name is `Insert`, and `InsertAt` is the empty text |
| `x.nothing` where `x` is a text | an error | the empty text |
| `x.b := 5` where `x` is a text | an error | the write is dropped |

```ahk
; v1: the array grows with the write of an item that is not there.
a := []
a[3] := 7
FileAppend, [% a.Length()], *
```

The v2 has `InsertAt` and `RemoveAt`; the v1 has `Insert` and `Remove`. The
v1 array has neither `Length` nor `Capacity` as properties.

### Objects and classes

| | v2 | v1 |
|---|---|---|
| `{a: 1}` | an object of the class `Object` | an object with no class |
| the name of a property in a literal | a name, or a built name (`{%x%: 1}`) | the text until the colon: `{1.5: 1}`, `{1: 1}` and `{1 + 1: 1}` are all allowed, and `{%x%: 1}` is `Quote marks are required around this key.` |
| the comma at the end of a literal | allowed: `{a: 1,}` | `Blank parameter` |
| `{}` as a text | the class of the object | the empty text |
| `__Class` | every value has one: `"Integer"`, `"String"`, `"Array"`, the class of the script | a property of an object, and the plain object's is empty |
| a function as a value | `f := StrLen` is a `Func` object | `StrLen` is the empty text, and `Func("StrLen")` makes one |
| the name of a class in an assignment | `C := 5` is refused: `This Class cannot be used as an output variable.\n     Specifically: C` | allowed |
| the name of a function in an assignment | `Abs := 5` is refused: `This Func cannot be used as an output variable.\n     Specifically: Abs` | allowed, and `Abs(3)` still calls the built-in |
| `base` | -- | the base of the class where the method was defined |
| `super` | the v2 word | -- |
| a nested class | a member of the class value | a member of the instance as well |
| the methods of a method | `ObjBindMethod`, `Bind` | `ObjBindMethod` |

---

## 3. The names

### The built-in variables

Both dialects prefix a built-in variable with `A_`, and the two lists meet
almost everywhere. The v1 has four names without the prefix -- `Clipboard`,
`ComSpec`, `ErrorLevel`, `ProgramFiles` -- and `ErrorLevel` is the v1's own.

| Name | v2 | v1 |
|---|---|---|
| `A_Clipboard` | the clipboard | the clipboard, and `Clipboard` is the same |
| `ErrorLevel` | **not a name**: reading it fails with `This global variable has not been assigned a value.` | the last error of a command, `0` when there was none, and a script may write it |
| the flags, `A_DetectHiddenText` | the number 1 or 0 | the word `On` or `Off` |
| `A_AutoTrim`, `A_FormatFloat`, `A_FormatInteger`, `A_StringCaseSense`, `A_NumBatchLines` | -- (the settings are read with `A_` names of the v2, or not at all) | measured values: `On`, `0.6`, `D`, `Off`, `10ms` |
| `A_OSType`, `A_IPAddress1` | the Windows type, the address of the adapter | the tree refuses both with the reason of the machine |
| the names of the v2 that the v1 lacks | `A_Args` is an array | `A_Args` is a variable of the script |
| a read-only name that a script writes | refused for the read-only names, with the v2 wording | refused for **every** built-in variable, with `Not allowed as an output variable.\n     Specifically: A_AhkPath` |
| `x .= 1` on a built-in variable | the v2 rule | accepted, and the variable keeps its value |

### The names of commands that the two do not share

The names are the hardest part of a port, because a v1 name and a v2 name can
be the same word with different meanings. The v1 names that the v2 does not
have are all in `crates/ahk/src/v1/table.rs`; the important ones are:

| The v1 name | The v2 name |
|---|---|
| `MsgBox, text` | `MsgBox("text")` |
| `StringLen, out, in` | `StrLen("text")` |
| `StringMid`, `StringLeft`, `StringRight` | `SubStr` |
| `StringReplace, out, in, a, b` | `StrReplace("text", "a", "b")` |
| `StringGetPos, out, in, a` | `InStr("text", "a")` |
| `StringUpper`, `StringLower` | `StrUpper`, `StrLower` |
| `StringSplit` | `StrSplit` |
| `StringTrimLeft`, `StringTrimRight` | `Trim` and `LTrim` / `RTrim` |
| `InStr` | `InStr` (the same name) |
| `EnvGet`, `EnvSet` | `EnvGet`, `EnvSet` |
| `EnvAdd`, `EnvSub` | the operators `+=` and `-=` |
| `SetEnv` | `EnvSet` |
| `FileRead, out, file` | `FileRead("file")` |
| `FileAppend, text, file` | `FileAppend("text", "file")` |
| `FileDelete`, `FileCopy`, `FileMove`, `FileCreateDir` | `FileDelete`, `FileCopy`, `FileMove`, `DirCreate` |
| `FileGetSize`, `FileGetAttrib`, `FileGetTime` | `FileGetSize`, `FileGetAttrib`, `FileGetTime` |
| `FileSetAttrib`, `FileSetTime` | the same names |
| `IniRead`, `IniWrite`, `IniDelete` | the same names |
| `RegExMatch`, `RegExReplace` | the same names |
| `PixelGetColor, x, y` | `PixelGetColor(x, y)` |
| `MouseGetPos, x, y` | `MouseGetPos(&x, &y)` |
| `SplitPath, path, a, b, c, d, e` | `SplitPath("path", &a, &b, &c, &d, &e)` |
| `Send, text` | `Send("text")` |
| `Click, x, y` | `Click("x y")` |
| `WinExist`, `WinActive`, `WinActivate` | the same names |
| `Process`, `Run`, `RunWait` | `Process*`, `Run`, `RunWait` |
| `Exit, code` | `ExitApp(code)` |
| `Sleep, ms` | `Sleep(ms)` |
| `Transform, out, Chr, n` | `Chr(n)` |
| `VarSetCapacity` | `Buffer` |
| `IsSet(x)` | `IsSet(x)` (the same name; the `IsSet` of the v1 asks about the variable, and the v2 asks about the value) |
| `Func("F")` | `Func("F")` |
| `Format` | `Format` |

The full list, with the arity of each name, is in
[ahk-v1-reference.md](ahk-v1-reference.md) and
[ahk-v2-reference.md](ahk-v2-reference.md).

### The functions that are one name and two meanings

Some names are in both tables and mean two different things. They are the
ones a port gets wrong without a message:

| The name | v2 | v1 |
|---|---|---|
| `IsSet` | asks whether the **value** it is given is a variable that has one | asks about the **name**: `IsSet(x)` asks whether `x` has a value |
| `VarSetCapacity` | -- | takes the **variable** of the caller, not its value |
| `StrReplace` | answers the text | answers the text, and writes the number of the replacements into the fourth argument |
| `FileSetTime` | the file may not be left out | the file may be left out, and the file of the loop runs |
| `MaxParams` of a built-in | the number of the table | ten thousand for the ones that take any number of arguments |
| `IsFunc` | a `Func` object or 0 | a number: the minimum number of the parameters |

---

## 4. Scoping

| | v2 | v1 |
|---|---|---|
| a variable of the script | global, and read from anywhere | global, and **not** read from a function |
| a variable of a function | local unless declared | local unless declared |
| the scope that a function reads | its own, and then the scope of the script | its own only |
| `global x` | makes the variable of the script visible in the function | the same, and `global` alone is the mode of the whole function |
| `local` | a declaration of the names | the same, and `local` alone is a declaration of no name, first statement only |
| `static` | a variable that keeps its value between the calls | the same |
| a `global` of a built-in variable | allowed | refused: `Built-in variables must not be declared.\n     Specifically: ErrorLevel` |
| the write of a name that the script builds, inside a function | refused, with `This dynamic assignment requires a "global" declaration.` | refused, with `This dynamic variable is blank. If this variable was not intended to be dynamic, remove the % symbols from it.` |
| the case of a name | does not matter | does not matter |

The difference that bites a port is the first row: a v1 function that reads a
variable of the script gets the **empty text**, and the same function in the
v2 gets the value.

```ahk
; v1: the function does not see the variable of the script.
Outer := 5
F() {
    return Outer
}
y := F()
FileAppend, [%y%], *
; the output line is: []
```

```ahk
; v1: the global declaration is what makes it visible.
Outer := 5
F() {
    global Outer
    return Outer
}
y := F()
FileAppend, [%y%], *
; the output line is: [5]
```

In the v2 the first of the two reads 5, and the `global` is not needed to
read. A v2 script that relied on the read-through has a bug in the v1, and a
v1 script that declared `global` for a read is a v2 script that works.

The mode of a function is the `global` alone on its first line in the v1: the
whole body is then the scope of the script.

---

## 5. The errors

The whole model is in [errors.md](errors.md). The three differences that
change how a script is written:

- **The failures of a command.** A v1 command that fails does not stop the
  script: it writes `ErrorLevel` and the script goes on. The v2 name fails
  with an error, and the script stops unless a `catch` takes it.

  ```ahk
  ; v1: the failing command sets ErrorLevel and the script goes on.
  FileRead, t, nosuchfile.txt
  MsgBox, [%t%] [%ErrorLevel%]
  ; the msgbox shows: [] [1]
  ```

- **The leniency.** A variable with no value, a division by zero, a property
  that is not there and an index out of range are the empty text in the v1
  and errors in the v2.

- **The catching.** v1 has no `try` that catches: it has `OnError`, and the
  callback takes the error object and the mode. The v2 has `OnError` too --
  it is a function of both dialects -- and its own `try` / `catch`.
  `OnError` takes the object of the function in the v2, and the **name** of
  the function in the v1, which is the form that the v1 scripts write:
  `OnError("Handler")` registers the handler of that name, and the object
  form `OnError Handler` works in both. A v1 script that writes it as a
  command, with the comma, `OnError, MyHandler`, is a line that holds no
  action, and it is refused the way the v1 refuses it.

The load errors are the same shape in both dialects, with the words of the
dialect.

---

## 6. The libraries

The two dialects answer the same machine and, mostly, the same names, so the
libraries differ in three ways:

1. **The dialect of the call.** A v1 command takes texts and writes into
   variables of the caller; a v2 function takes expressions and answers a
   value, or takes a reference (`&x`) where it must write.
2. **The names that only one dialect has.** `SetBatchLines`, `AutoTrim`,
   `SetFormat`, `StringCaseSense`, `DetectHiddenWindows`, `CoordMode`, the
   `Env*` commands, `Transform`, `VarSetCapacity`, `Gosub`, `Pause`,
   `Reload`, `Suspend`, `Critical`, `Thread` are v1. `Map`, `Array`,
   `Buffer`, `StrSplit`, `FormatTime`, `DateAdd` and the class types are v2.
3. **The names that name Windows.** A name shaped for Windows -- `WinSet`,
   `ControlSend`, `RegRead`, `ComObjCreate` -- is carried in both tables, and
   the handler answers with the reason the machine cannot do it. It does not
   answer `false`.

The one that is the same and worth calling out is `DllCall`: both dialects
call the library of the machine, and both do it on the machine they run on.

### Where a name lives, and the `--surface`

The `--surface` of a dialect says what its table holds and how the tree
answers each name. A `refusal` is a name the tree carries and cannot answer
on this machine; a `value` is a name the tree answers; `pending` is a name
the tree does not carry, and neither table has one.

```sh
# What the tree answers of the v1, with the reasons.
echo '' | ahk --surface --dialect=v1 | tail -1
# the surface of v1 has 408 entries: 301 answer with a value, 107 are refused, 0 are pending.
```

The reports of the areas -- [v2-capability-report.md](v2-capability-report.md)
-- describe what each refusal is about.

---

## Porting a v1 script to v2

### The order to do it in

1. **Run it as v1 first, and keep the report.** The script that already works
   in `--dialect=v1` is the reference: its steps and its `output` are what
   the v2 script has to reproduce.
2. **Change the statements, not the words.** A v1 command is a line; a v2
   call is an expression. Do the shape first, and the names second.
3. **Turn every command that fails into a value or a `try`.** The v1's
   `ErrorLevel` does not exist in the v2: the read fails, and the write fails
   the load.
4. **Turn the `%name%` into the value.** A `%x%` of a v1 command parameter is
   the value of `x` in the v2, and a `%name%` of an expression is the v2's
   `%name%` built name, which the v2 refuses with `Quote marks are required`.
5. **Check every function that read a variable of the script.** In the v2 it
   reads it; in the v1 it needed a `global`.
6. **Run the `--surface` before and after** and compare the names.

### The translation table

| v1 | v2 |
|---|---|
| `MsgBox, text` | `MsgBox text` |
| `StringLen, n, x` | `n := StrLen(x)` |
| `StringMid, out, x, start, len` | `out := SubStr(x, start, len)` |
| `StringReplace, out, x, a, b` | `out := StrReplace(x, a, b)` |
| `x = 5` | `x := 5` |
| `x = %y%` | `x := y` |
| `x = "abc"` (a quoted text, quotes included) | `x := "abc"` (the text without the quotes) |
| `if x = 5` | `if (x = 5)` |
| `if x contains a` | `if InStr(x, "a")` |
| `Loop, 3` | `Loop 3` |
| `Loop, %n%` | `Loop n` |
| `FileRead, t, f` | `t := FileRead(f)` |
| `FileAppend, t, f` | `FileAppend t, f` |
| `PixelGetColor, c, x, y` | `c := PixelGetColor(x, y)` |
| `MouseGetPos, x, y` | `MouseGetPos &x, &y` |
| `SplitPath, p, a, b` | `SplitPath p, &a, &b` |
| `EnvGet, v, NAME` | `v := EnvGet("NAME")` |
| `Exit, 3` | `ExitApp 3` |
| `x ++` | `x++` |
| `for i in [10, 20]` | `for v in [10, 20]` (and `A_Index` for the key) |
| `::btw::by the way` | `Hotstring ":btw:", (*) => Send("by the way")` |
| `^a::Fn()` | `Hotkey "^a", Fn` |
| `class C { }` and `new C()` | `class C { }` and `C()` |
| `super` | `super` (the v1 word is `base`) |
| `ErrorLevel` | nothing: the failure is an error now |
| `OnError("Handler")` | `OnError Handler` |
| `Func("F")` | `Func("F")` |
| `StrLen(x)` where `x` may be a number | `StrLen(x)` (both take the text of the number) |

A port that begins with the table is a port that gets the names right and the
shape wrong. Begin with the shape: a call answers a value, and a script that
wrote `StringLen, n, x` was writing into `n` with the value in the answer.

### What has no direct v2 word

- **`ErrorLevel`.** The v1 script that checks `ErrorLevel` after a command is
  a script that has to wrap the v2 call in a `try`, because the v2 call
  fails.
- **The `%` substitution inside a command parameter.** A v2 call takes an
  expression, and the substitutes become the expression.
- **The `gosub` of a label.** The v2 has `Goto` and not `Gosub`: a return is
  a function's.
- **The legacy `if` and its `contains` / `in` / `between`.** The v2 has
  `InStr` and the comparison operators.
- **The hotstring as a statement.** The v2 writes `Hotstring`, and the
  action is a function.
- **The section that ends at the first hotkey.** The v2 runs the whole
  script, so a v1 script that ended its running code with the first hotkey
  needs an `ExitApp` there, or the code after it runs.
- **`SetBatchLines`, `Critical`, `Thread`.** The v2 has no threads, and the
  tree refuses the names of the v1 with the reason.

---

## Porting a v2 script to v1

This is the rarer direction, and it is where the dialect shows its age. The
order that works is the reverse of the one above, and it has three shapes of
change:

1. **The calls become commands, or stay calls.** A v2 call of a name that the
   v1 carries as a **function** stays a call: `StrLen(x)` is a call in both
   dialects. A v2 call of a name that the v1 carries as a **command** -- 
   `MsgBox`, `FileAppend`, `Send`, `Click`, `Sleep` -- has to be written the
   v1 way, with the texts and the commas.
2. **The values become texts.** A v1 command parameter is a text: `MsgBox x`
   in the v2 shows the value of `x`, and `MsgBox, %x%` in the v1 does the
   same.
3. **The errors become `ErrorLevel`, or they stop the script.** A v2 script
   that relies on a failure stopping the statement has to check
   `ErrorLevel` in the v1, or wrap the call in a way that the v1 can.

The traps in this direction:

| The v2 thing | Why it fails in the v1 |
|---|---|
| `try` / `catch` | the `try` block runs, the `catch` is refused |
| `Map`, `Array`, `Buffer`, `File` as class types | not names of the v1: `Map()` is `Call to nonexistent function.` |
| `a.Length`, `a.InsertAt` | the v1 has no `Length` property and no `InsertAt`: `a.Length()` and `Insert` |
| `x :=` with the value on the next line | the v1 assigns the empty text |
| `"a""b"` | the v1 reads the text `a"b`; the same line is not the same value |
| `[1, 2]` and `{a: 1}` | the literals are of the v1 too, and the item of a plain object is its property |
| `C()` | `new C()` |
| `ExitApp 3` | `Exit, 3` |
| `OnError Handler` | the same, and the form with a command and a comma is refused: `OnError, MyHandler` is a line that holds no action |
| `#Warn` types | the types are the v1's |
| `A_Clipboard` | the same name; and `Clipboard` is the same variable in the v1 |

---

## What the v1 deliberately does not have

The plan of the dialect, [ahk-v1-dialect.md](ahk-v1-dialect.md), names what
is out on purpose, and the tree answers those names with the reason:

- **the threads of the v1**: the v1 runs a hotkey in a thread of its own, and
  this runtime does not carry threads. `Thread`, `Critical` and the
  `#MaxThreads` directives are carried and refused with the reason;
- **the `Gui`, the `Menu` and the `TrayTip`** as the v1 commands: the tree
  refuses them, and the v2 refuses the `Gui` too;
- **the registry** (`RegRead`, `RegWrite`, `RegDelete`): refused with the
  reason of the machine;
- **the COM objects** (`ComObj*`, `ComValue`): refused;
- **the `Control*` commands**: refused, because the box has no accessibility
  layer -- see [wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md);

A name that is refused is still a name the dialect carries: the script that
uses it loads, runs, and is told what the machine cannot do.

---

## Where this rests

| What | Where |
|---|---|
| the dialect, the two implementors, every rule and its measurement | `crates/ahk/src/dialect.rs` |
| the table of the v1 names, and the shapes of the parameters | `crates/ahk/src/v1/table.rs` |
| the rules the v1 commands need | `crates/ahk/src/v1/parameters.rs`, `sub_commands.rs`, `labels.rs`, `numbers.rs`, `strings.rs`, `kinds.rs`, `names.rs`, `loops.rs`, `variables.rs` |
| the built-in variables of the v1 | `crates/ahk/src/v1/variables.rs` |
| the gate | `crates/ahk-cli/src/main.rs`, `the_dialect` |
| the cases of the v1 | `crates/ahk/tests/v1.rs` |
| the cases of the v2 | `crates/ahk/tests/language.rs` |
| the plan and the measurements | [ahk-v1-dialect.md](ahk-v1-dialect.md) |
| the names of each dialect | [ahk-v2-reference.md](ahk-v2-reference.md), [ahk-v1-reference.md](ahk-v1-reference.md) |
| the errors of the two | [errors.md](errors.md) |
| the report of a run | [runner-and-report.md](runner-and-report.md) |
