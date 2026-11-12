# Errors: what fails, what it says, and how a script catches it

The error model of the two dialects this binary speaks: the classes of an
error, the place where a script fails, the object a script catches, the
words that catch it, and the differences between v2 and v1.

- [What an error is](#what-an-error-is)
- [A load error and a run error](#a-load-error-and-a-run-error)
  - [The three ways a run ends](#the-three-ways-a-run-ends)
  - [The shape of each report](#the-shape-of-each-report)
- [The tree of the error classes](#the-tree-of-the-error-classes)
  - [The class of a failure is decided by its message](#the-class-of-a-failure-is-decided-by-its-message)
  - [Making an error of a class](#making-an-error-of-a-class)
- [The error object](#the-error-object)
- [Catching: `try` / `catch` / `catch as` / `else` / `finally`](#catching-try--catch--catch-as--else--finally)
  - [Which catch takes the value](#which-catch-takes-the-value)
- [`throw`](#throw)
- [`OnError`](#onerror)
- [The guard rails](#the-guard-rails)
- [The failures the desktop raises](#the-failures-the-desktop-raises)
- [v1: the lenient dialect](#v1-the-lenient-dialect)
- [The per-dialect table](#the-per-dialect-table)
- [Where the code is](#where-the-code-is)

---

## What an error is

An error is a failure of the run that **stops the script at the statement
that raised it**. The statement is not a step of the report: the report
carries the failure once, in its `error` and `error_line`.

An error class, an error object, an error message and the words that catch
one are four different things, and this document keeps them apart:

| Word | What it is |
|---|---|
| a **failure** | the event: `1 / 0`, a variable with no value, a file that is not there |
| a **message** | the text of the failure: `Divide by zero.` |
| a **class** | the kind of the failure: `ZeroDivisionError` |
| the **error object** | what a `catch` holds: an object of that class, with `Message`, `What`, `Extra`, `File`, `Line` and `Stack` |

---

## A load error and a run error

**A load error** is a failure of the script before the first statement
runs. The parser refuses the text, the script does not run at all, the
report is the reduced payload of a parse error, and the runner exits **2**.

What a load error includes, in the order the tree checks it:

1. **The text of the script is not text.** The runner reads the script from
   standard input, and a stream that is not UTF-8 fails before the parser is
   asked anything:

   ```text
   cannot read the script: stream did not contain valid UTF-8
   ```

   This is the one case with **no report**: the reason goes to standard
   error and the runner exits 2.

2. **An `#Include` that cannot be resolved.** The report says so, with
   `error_line` 0:

   ```json
   ; echo '#Include "no\\such\\file.ahk"' | ahk
   {"ok": false, "error": "the include \"\"no\\such\\file.ahk\"\" cannot be found.",
    "error_line": 0, "column": 0, "steps": [], "output": [], "notes": []}
   ```

3. **The grammar.** This is the load error a script meets in practice: a
   bracket that is not closed, a word that is out of place, a name that is
   reserved, a command of v1 that is not one, a module-level call of a
   function of v1 that does not exist.

4. **The checks of the dialect.** v1 checks some things at the load that v2
   checks at the run or does not check at all. A write of a built-in
   variable, a label that a command names, a parameter of a command and the
   name of a hotkey are load errors of v1, and they are load errors even
   though they are about the meaning of a line and not its grammar. The
   examples are in [dialects.md](dialects.md).

**A run error** is everything else: the script loaded, the first statements
ran, and one statement failed. The report carries it in `error` and
`error_line`, the runner exits **1**, and the statements before it are
steps of the report.

A load error **cannot be caught**, in either dialect: the script never
starts. There is no `try` around the parser.

### The three ways a run ends

| Step | What it is | Runner exit |
|---|---|---|
| the load | the text is not text, an include is missing, the grammar refuses the script, or the dialect refuses a line before the run | 2 |
| the run | a statement failed, and no `catch` took it | 1 |
| the end | every statement finished | 0 |

The exit code of the process is not `ExitApp`'s: `Exit 3` ends the script
and the report says `"exit_code": 3`, and the runner still exits 0.

### The shape of each report

A **load error** is a reduced payload -- no `elapsed_ms`, no `exit_code`, no
`variables`, and no `ok` that is true:

```json
; printf 'x := (1' | ahk
{"column": 1, "error": "Missing \")\"", "error_line": 2, "notes": [],
 "ok": false, "output": [], "steps": []}
```

A **run error** is the whole payload, with the steps that completed:

```json
; printf 'x := 1/0' | ahk
{"elapsed_ms": 0, "error": "Divide by zero.", "error_line": 1,
 "exit_code": null, "notes": ["no uid on this platform; ..."], "ok": false,
 "output": [], "steps": [], "variables": {"a_linenumber": 1}, "warnings": []}
```

The `column` of a load error is where the refusal starts, and `error_line`
is where the tree was when it gave up -- which for a bracket is often the
line **after** the last line of the script. The full shape of both payloads
is the subject of [runner-and-report.md](runner-and-report.md).

---

## The tree of the error classes

The classes are the ones of AutoHotkey v2.0.19, and they derive from one
another. Every one of them is an `Error`.

| Class | Derives from |
|---|---|
| `Error` | -- (the root) |
| `MemoryError` | `Error` |
| `OSError` | `Error` |
| `TargetError` | `Error` |
| `TimeoutError` | `Error` |
| `TypeError` | `Error` |
| `UnsetError` | `Error` |
| `MemberError` | `UnsetError` |
| `PropertyError` | `MemberError` |
| `MethodError` | `MemberError` |
| `UnsetItemError` | `UnsetError` |
| `ValueError` | `Error` |
| `IndexError` | `ValueError` |
| `ZeroDivisionError` | `Error` |

Because of the tree, one failure can be of several classes at once: a
property that is not there is a `PropertyError`, a `MemberError`, an
`UnsetError` and an `Error`, and a `catch` of any of the four takes it.

```ahk
; A property error is a member error, an unset error and an error.
e := PropertyError("the message")
MsgBox e is MemberError  ; 1
MsgBox e is UnsetError  ; 1
MsgBox e is Error  ; 1
```

The class names are matched without case, and they are spelled the way v2
spells them: `TypeError`, not `Typeerror`.

### The class of a failure is decided by its message

v2 decides the class of a failure by the kind of the failure. The tree has
one table of rules, and it reads the **message**, because the message of v2
says what the failure is:

| The message | The class |
|---|---|
| is `Divide by zero.` | `ZeroDivisionError` |
| begins with `(` -- the code of the machine, `(2) The system cannot find the file specified.` | `OSError` |
| holds `has no property named` | `PropertyError` |
| holds `has no method named` | `MethodError` |
| is `Item has no value.` | `UnsetItemError` |
| ends with `has not been assigned a value.` | `UnsetError` |
| is `Invalid index.` | `IndexError` |
| is `Invalid value.`, or ends with ` is invalid.`, or is `Invalid parameter(s).`, `Invalid option.`, or ends with ` must not be omitted in this case.` | `ValueError` |
| begins with `Expected a `, begins with `Expected an `, holds ` requires `, or is `Value not enumerable.`, `Type mismatch.` | `TypeError` |
| `This dynamic assignment requires a "global" declaration.` | `Error` (not a `TypeError`, measured on the binary) |
| anything else | `Error` |

The four measured cases of the table:

```ahk
; Each line, in its own script, is caught and asked for its class.
try {
    x := 1 + "a"
} catch Any as e {
    MsgBox e is TypeError  ; 1: "Expected a Number but got a String."
}

try {
    m := Map()
    m.CaseSense := "x"
} catch Any as e {
    MsgBox e is ValueError  ; 1: "Invalid value."
}

try {
    f := Abs
    f.Call()
} catch Any as e {
    MsgBox e is Error  ; 1, and not a TypeError
}
```

**A failure of the files is still of the class `Error`.** The message of the
machine is not the message of v2, and the class of those failures is not
carried: `(2) The system cannot find the file specified.` begins with `(`, so
it is an `OSError`, but only for the messages that do.

### Making an error of a class

Every class is made the way `Error` is: `Class(Message, What, Extra)`.

```ahk
; The message, the thing that raised it, and the detail.
e := TypeError("the message", "the what", "the extra")
MsgBox e.Message . " " . e.What . " " . e.Extra  ; the message the what the extra
```

- A message that is not given is the **name of the class**: `TypeError()`
  has the message `TypeError`.
- `What` and `Extra` default to the empty text.
- The number of the arguments is checked, and the failure is a plain
  `Error` that names the call and the constructor:

  ```ahk
  try {
      x := Error("a", "b", "c", "d")
  } catch Any as e {
      MsgBox e.Message . " | " . e.What . " | " . e.Extra
      ; Too many parameters passed to function. | Object.Call | Error.Prototype.__New
  }
  ```

---

## The error object

The object a `catch` holds has exactly six own properties, spelled the way
v2 spells them:

| Property | What it holds |
|---|---|
| `Message` | the text of the failure |
| `What` | the name of the thing that raised it: a function, a method, a built-in; the empty text when the failure has no name |
| `Extra` | the detail that goes with it: the value of a parameter that failed, the key of a map, the name of a method |
| `File` | the file of the script -- **always empty here**, because the runner hands the runtime a script on standard input and not a file |
| `Line` | the line of the script where the error was made or thrown, not the line where it is read |
| `Stack` | every frame of the call that failed, one line for each |

The properties are own properties of the object, and they are ordinary
properties: `HasProp` and `HasOwnProp` are 1 for them, `ObjOwnProps` lists
them in the order `[Extra][File][Line][Message][Stack][What]`, and they may
be written.

```ahk
; The own properties of an error, in the order the tree keeps them.
e := Error("m", "w", "x")
s := ""
for p in ObjOwnProps(e)
    s .= "[" p "]"
MsgBox s  ; [Extra][File][Line][Message][Stack][What]

; And they are written like any other property.
e.Message := "w"
MsgBox e.Message  ; w
```

The `Stack` of a script that fails at the top level is two lines: the frame
of the statement, and the section that started the script.

```ahk
; The stack names the line, the frame, and the section that starts the script.
e := Error("the message")
MsgBox e.Stack
;  (1) : [] e := Error("the message")
; > Auto-execute
```

Each frame is written as ` (line) : [name] source-of-the-line`: the file of
the frame is empty here, `name` is the function or the method, and the source
is the text of the line without its indentation. The last line is
`> Auto-execute`.

An error has **no `Prototype` of its own**: `e.Prototype` is "This value of
type \"Error\" has no property named \"Prototype\".", while the class has
one: `Error.Prototype.__Class` is `Error`, its `Base` is `Object`, and
`Error.Base.__Class` is `Class`.

---

## Catching: `try` / `catch` / `catch as` / `else` / `finally`

```ahk
x := 1
try {
    y := x.nothing
} catch Any as e {
    MsgBox e.Message  ; This value of type "Integer" has no property named "nothing".
}
```

| Part | What it does |
|---|---|
| `try { }` | the guarded block. While it runs, the functions of `OnError` are not called. |
| `catch { }` | takes every error of the runtime, and nothing else |
| `catch as e { }` | the same, and the caught value goes into `e`; the class may be written as well: `catch Error as e` |
| `catch Class as e` | takes the value when it is of that class or of a class that derives from it |
| `else { }` | runs when the guarded block finished **without** a value being thrown |
| `finally { }` | runs whatever happened, and the failure it leaves is the one that continues |

The class of a `catch` follows four rules:

- **`catch` with no class is `catch Error`**, and a `catch Error` takes every
  error the runtime raises. It does **not** take a value that the script
  threw as a text: that value is not an error object.

  ```ahk
  ; A text that is thrown is not an Error: this catch does not take it.
  try {
      throw "the error of the script"
  } catch {
      MsgBox "in the catch"  ; not reached
  }
  ; the run fails with: Unhandled exception.\n     Specifically: the error of the script
  ```

- **`catch Any` takes everything**, the errors of the runtime and the values
  the script threw.

  ```ahk
  try {
      throw "the error of the script"
  } catch Any as e {
      MsgBox e  ; the error of the script
  }
  MsgBox "after"  ; after
  ```

- **A class of a `catch` that is not a class of the runtime is itself a
  failure**: "the class of a catch is not a class of this runtime:
  NoSuchClass". The run stops there, with that reason.

- **A class of the script takes its own objects, and the objects of the
  classes that extend it.**

### Which catch takes the value

The catches are tried in the order the script wrote them, and the first one
that takes the value runs. The value goes into the variable of `catch as`,
and the rest of that catch's block sees it.

The `Line` of the error is the line where it was **made**, not where it was
read:

```ahk
e := Error("m")


MsgBox e.Line  ; 1
```

and it is the line of the `throw` when the error was thrown rather than made:

```ahk
try {
    throw Error("m")
} catch as e {
    MsgBox e.Line  ; 2
}
```

---

## `throw`

`throw Value` stops the block and carries the value -- any value, not only an
error object -- to the nearest catch that takes it. A `throw` that no catch
takes is the error of the run.

| The value that was thrown | The `error` of the report |
|---|---|
| a text | `Unhandled exception.\n     Specifically: the text` |
| anything else | `Unhandled exception.` |

```ahk
; A throw that nobody catches, with error_line null.
throw 42
; {"error": "Unhandled exception.", "error_line": null, "ok": false}
```

**`throw` with no value throws again** the value that the catch of the
running `try` took, which is how a catch passes an error on:

```ahk
try {
    throw "the error of the script"
} catch Any as e {
    throw
}
; the run fails with: Unhandled exception.\n     Specifically: the error of the script
```

A `throw` with no value outside a catch is a failure of its own: "there is
nothing to throw again: no catch took a value."

When the value is an **error object of the class `Error`** and its `What` is
empty, the `throw` writes the name of the frame that threw it into the
`What`: the name of the function, or `Thing.Prototype.Method`. An error that
already carries a `What` keeps it.

---

## `OnError`

`OnError` registers a function that the runtime calls for a failure that no
`catch` took. It is a function of both dialects, and it is the only way the
v1 has: v1 has no `try`.

```ahk
Handler(Thrown, Mode) {
    MsgBox Mode . " | " . Thrown.Message
    return -1
}
OnError Handler
MsgBox NoSuchVariable
MsgBox "after"
; the output is: Return | This global variable has not been assigned a value.
;                after
```

The callback takes two parameters:

| Parameter | What it is |
|---|---|
| `Thrown` | the error object of the failure, or the value the script threw |
| `Mode` | `"Return"` for a failure of the runtime, `"Exit"` for a value the script threw, `"ExitApp"` for the timeout of the runtime |

And it answers a number that says what happens next:

| The answer | What the runtime does |
|---|---|
| `0` (or nothing) | the failure goes on: it is the error of the run |
| `1` | the failure is hidden: the report has no `error`, and a note says "the functions of the error hid the failure: ..." |
| `-1` | the failure is hidden and **the thread continues** at the statement after the one that failed -- for `Mode` `"Return"` only. A thrown value cannot be continued: `-1` hides the `"Exit"` failure like `1` does. |

The second parameter of `OnError` is the order: `1` registers the function,
`-1` registers it at the front, and `0` removes it. The order that is not
there is 1.

```ahk
; A function of the error that continues the thread after the failure.
Handler(Thrown, Mode) {
    return -1
}
OnError Handler
Fail()
MsgBox "after the call"
Fail() {
    MsgBox NoSuchVariable
    MsgBox "inside the function"
}
; the output is: inside the function
;                after the call
```

Three things to know:

- **`OnError` is not called while a `try` runs.** The catch of the script
  takes the failure first; the functions of the error are called only when
  no catch took it.
- **The argument is a function object, not a name.** A text is refused:
  "Parameter #1 of OnError requires an Object, but received a String." The
  value comes from the bare name of the function (`OnError Handler`), from a
  variable that holds it, or from `ObjBindMethod`.
- **A function that fails does not hide the failure.**

---

## The guard rails

The runtime stops a script that does not finish. A limit is a run error with
a line of its own, and it is one the script can catch:

| Limit | The message |
|---|---|
| 20 000 statements | `the script ran more than 20000 statements` |
| 1 000 turns of a `While` body | `the loop ran more than 1000 times` |
| 1 000 nested calls | `Function recursion limit exceeded.` |
| the timeout of the runner (`--timeout=N`, 60 s by default) | `the script ran for more than Ns` |

The first three carry the line of the statement that tripped them. **The
timeout does not**: it stops the script between two statements, so the report
has `"error_line": null`.

```ahk
; The recursion rail, caught like any other error.
F(k) {
    return F(k + 1)
}
try {
    F(1)
} catch as e {
    MsgBox e.Message  ; Function recursion limit exceeded.
}
```

The bounds are in `crates/ahk/src/runtime.rs` (`MAX_STEPS`,
`MAX_ITERATIONS`, `MAX_DEPTH`, `MAX_STACK`), and the timeout is the runner's.
A long `Sleep` is not stopped by the timeout until it returns: the timeout is
checked before every statement, so `--timeout=0.1` with `Sleep 5000` finishes
normally after five seconds.

---

## The failures the desktop raises

A built-in that asks the machine and cannot be answered **fails with the
reason**; it never answers `false`. The reason is the error of the run, and
the catch takes it like any other.

| What was asked | The message |
|---|---|
| a window, where the compositor has no window list | `WinExist cannot answer: ...` (the reason names the compositor and the session) |
| a keyboard that is not a Linux uinput device | `the keyboard: the keyboard is a Linux uinput device; this is not Linux` |
| a name of Windows, on a machine that is not | `this is a name of Windows, and this desktop is not Windows.` |
| a dialog, with no dialog program on the machine | `zenity is not on this machine, and the box has no other dialog` |
| a file the machine cannot open | the code of the machine, `(2) The system cannot find the file specified.` |

A window that is simply not there is **not** a failure: `WinExist` and
`WinActive` answer 0. The failures of the desktop are described in
[desktop-backend.md](desktop-backend.md).

---

## v1: the lenient dialect

The v1 is lenient in a way the v2 is not, and the difference is deliberate:
v1 scripts in the world depend on it. **A blank result is not an error.**

| What the v2 fails on | What the v1 answers |
|---|---|
| a variable with no value: `This global variable has not been assigned a value.` | the empty text |
| `Divide by zero.` | the empty text |
| a property that is not there: `This value of type "String" has no property named "nothing".` | the empty text |
| an index that is out of range: `Invalid index.` | the empty text |
| a value that is not a number where a number is wanted: `Expected a Number but got a String.` | the empty text |
| an operand that is not there, `x := 1 +` | the empty text (v2 refuses the script at the load) |
| a member that is not there, `a.HasProp("b")` | the empty text (v1 objects have no `HasProp`) |

Measured side by side:

```ahk
; The same script, run in the two dialects.
x := "abc"
y := x + 1
```

```json
; ahk --dialect=v1: the run finishes, and y is the empty text.
{"error": null, "ok": true, "steps": [{"detail": "x = abc", "line": 1}, {"detail": "y = ", "line": 2}]}

; ahk: the run fails at the load, because the two values are concatenated.
{"column": 6, "error": "Unexpected operator following literal string.\n     Specifically: + 1", "ok": false}
```

Things the v1 does **not** make lenient, and that are errors there too:

- **a call of a name that is not a function**: `Call to nonexistent function.
  \n     Specifically: foo()`, and it is a **load error**;
- **a name of the v2 that the v1 does not have**: there is no `Map` and no
  `try`, and the call of `Map()` is a load error of the same kind as the call
  of any name that the v1 does not know;
- **the class type names**: `a.Length` on a v1 array is the empty text, and
  the way to count is `a.Length()`;
- **the failures of the desktop**, which are the failures of the machine and
  not of the dialect.

### The words of v1

v1 has no `try`, and its `catch` is not one. The tree reads the `try` of a
v1 script as a block that runs, and it **drops the `catch` that follows it**:
the block of the `try` runs, the block of the `catch` does not, and the
script goes on.

```ahk
; v1: the try block runs, and the catch block is dropped.
try {
    z := "no throw"
}
catch {
    z := "caught"
}
FileAppend, %z%, *
; ["no throw"]
```

A `catch` does not catch anything here, which is why a v1 script that wants
to handle a failure registers `OnError` -- the name of the function, in
quotes, which is the form of v1.

```ahk
; v1: the handler is named, and the script goes on after the failure.
OnError("Handler")
x := 1 / 0
FileAppend, still here, *

Handler(err, mode) {
    return 0
}
; ["still here"]
```

The error handling of v1 is `OnError`, and the errors that the commands
report are the **value of `ErrorLevel`**, not an exception:

```ahk
; A command of v1 that fails sets ErrorLevel and goes on.
FileRead, t, nosuchfile.txt
MsgBox, [%t%] [%ErrorLevel%]
; the msgbox shows: [] [1]
```

The `error_line` of the report is the same field for both dialects, and it is
`null` for the errors `OnError` hides and for the timeout.

The differences are measured on the binaries of both dialects; the plan and
the measurements behind the leniency are in
[ahk-v1-dialect.md](ahk-v1-dialect.md), section 1.7.

---

## The per-dialect table

| | v2 (the default) | v1 (`--dialect=v1`) |
|---|---|---|
| the classes of the errors | fourteen, in a tree | none: an error has no class |
| the object a catch holds | an error object with `Message`, `What`, `Extra`, `File`, `Line`, `Stack` | an object with `Message`, `What`, `Extra`, `File` and `Line` |
| `try` | yes, with `catch`, `else` and `finally` | the `try` block alone runs; `catch` is refused at the load |
| `throw` | yes, of any value | yes, of any value |
| catching a failure | a `catch` | `OnError`, with a function object |
| the errors of a command | a failure that stops the statement | `ErrorLevel`, and the script goes on |
| a variable with no value | an error, `UnsetError` | the empty text |
| a division by zero | an error, `ZeroDivisionError` | the empty text |
| a property that is not there | an error, `PropertyError` | the empty text |
| an index out of range | an error, `IndexError` | the empty text |
| a wrong kind of value | an error, `TypeError` | a blank result, or the number that the text starts with |
| `ExitApp 3` | a run that ends with `exit_code` 3 | `Exit, 3` does the same |
| the load errors | the same payload | the same payload, with the words of v1 |

---

## Where the code is

| What | Where |
|---|---|
| the classes, the tree, the constructor, the six properties | `crates/ahk/src/classes/error.rs` |
| the table of the class of a message, and the failure | `the_class_of` in the same file |
| the `try` and which catch takes the value | `run_try` and `catch_takes` in `crates/ahk/src/runtime.rs` |
| the error object the runtime hands to a catch, and the stack | `error_value`, `error_value_with`, `the_stack` in the same file |
| `throw`, and the `What` of a thrower | `Node::Throw` in `execute`, and `name_the_thrower` |
| the functions of `OnError` | `ask_the_error_callbacks` in the same file |
| the message of a throw nobody caught | `thrown_message` in the same file |
| the guard rails | `MAX_STEPS`, `MAX_ITERATIONS`, `MAX_DEPTH`, `MAX_STACK` in the same file |
| the load errors of v1 | `crates/ahk/src/v1/parameters.rs`, `sub_commands.rs`, `labels.rs` |
| the leniency of v1 | `unset_is_blank`, `division_by_zero_is_blank`, `a_value_that_is_not_a_number_is_blank` in `crates/ahk/src/dialect.rs` |
| the cases of every message and class | `crates/ahk/tests/class_error.rs`, `crates/ahk/tests/language.rs`, `crates/ahk/tests/v1.rs` |

See also [dialects.md](dialects.md) for the rest of what the two dialects do
differently, and [ahk-v2-reference.md](ahk-v2-reference.md) for the names of
the error classes among the names of v2.
