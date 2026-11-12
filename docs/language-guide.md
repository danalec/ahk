# The language

This is the guide to the language the `ahk` binary runs. It is written for
someone who wants to learn the language, or to look up how a construct
behaves before writing a script.

One binary speaks **two dialects**: AutoHotkey v2.0.19, which is the default,
and AutoHotkey v1.1.37.02, which is chosen with `--dialect=v1`. The same
lexer, the same parser and the same runtime serve both, and every rule that
differs between them is a question the parser and the runtime ask the
dialect before they act. Where a construct behaves differently, this guide
says so with a line of its own:

**v2:** how it works in the default dialect.
**v1:** how it works behind the flag.

Every example in this guide was run through the runner, in the dialect the
example belongs to, and the answer beside it is the answer the runner gave.
Where an example is refused, the refusal is the real text of the refusal,
word for word. The two other places to look are the name tables:
`docs/ahk-v2-reference.md` and `docs/ahk-v1-reference.md` list every name of
a dialect with an arity and a state, and this guide links to them instead of
repeating their rows.

The rules were read out of `crates/ahk/src/lexer.rs`, `parser.rs`,
`runtime.rs` and `dialect.rs`, and the behaviours were then checked against
the runner rather than trusted from the reading. Where the two disagreed,
the runner won, and the reading was corrected. The places where this
runtime does not do what the reference dialect of the same version does are
collected at the end, under [Known gaps](#known-gaps).

## Contents

- [The two dialects](#the-two-dialects)
- [Lexical structure](#lexical-structure)
- [Values and types](#values-and-types)
- [Variables](#variables)
- [Operators](#operators)
- [Statements](#statements)
- [Control flow](#control-flow)
- [Loops](#loops)
- [Functions](#functions)
- [Classes and objects](#classes-and-objects)
- [The class types of the runtime](#the-class-types-of-the-runtime)
- [References](#references)
- [Errors](#errors)
- [Directives](#directives)
- [Hotkeys and hotstrings](#hotkeys-and-hotstrings)
- [Cheat sheets](#cheat-sheets)
- [Known gaps](#known-gaps)

## The two dialects

The runner picks the dialect with a flag, and the flag is the only gate: a
script written for v2 runs with no flag at all, and a script written for v1
runs when the runner is told so.

```ahk
; this is a v2 script: the names, the operators and the literals are v2's
name := "world"
FileAppend "hello, " name, "*"
```

```json
["hello, world"]
```

```ahk
; the same script in the v1 dialect: a command, commas, a text
name = world
FileAppend, hello %name%, *
```

```json
["hello world"]
```

The v1 dialect is not an interpretation of the v2 grammar. It is a dialect
of its own: the parser reads a line of the v1 as a command with positions
separated by commas, the parameters are texts and not expressions, and the
assignment writes a text. The two grammars do not meet, so a script is one
or the other.

**How one parser serves both.** The lexer is handed the dialect, and it
reads the tokens the dialect knows: `<>` is an operator for v1 and an
illegal character for v2, the single quote begins a text for v2 and a
symbol for v1, and the backtick escape table is the dialect's. The parser
asks the dialect, construct by construct, whether a line is a command or a
call, whether an `if` may carry a legacy comparison, whether a hotkey is a
label. The runtime asks the dialect about values: whether a text that is a
number is false, whether `"5" + 1` concatenates, whether a member that is
not there is refused or answered with an empty text. `crates/ahk/src/dialect.rs`
is the trait that holds every one of those questions, and each method
carries a doc comment saying what the dialect answers and why.

**What is shared, and what is not.**

| | v2 (default) | v1 (`--dialect=v1`) |
|---|---|---|
| Names | the v2 table | the v1 table |
| A statement | an expression, a call, or a call without parentheses | a command with comma-separated positions |
| Assignment | `:=`, `+=`, `++` | `=`, `:=`, `+=`, `++` |
| Strings | `"..."` or `'...'`, backtick escapes, `""` escapes a quote | `"..."`, `""` escapes a quote inside a command position |
| Members | refused when they are not there | answered with an empty text |
| Failures | an error stops the script | many failures are a blank value instead |
| Classes | class definitions and runtime class types | class definitions only; no `Map`/`Array`/`Buffer` |

The name tables are the difference in bulk. v2 carries 243 commands and 103
functions; the class types are `Map`, `Array`, `Buffer`, `File`, `Func`,
`Gui`, `Menu`, `Hotkey`, `Enumerator`, the error classes and the rest. The
v1 carries its own commands and functions, its own 137 built-in variables,
and no class type at all. See `docs/ahk-v2-reference.md` and
`docs/ahk-v1-reference.md`.

**A note on what "v1" means here.** It is AutoHotkey v1.1.37.02 as measured
on the reference binary, not the archived Python implementation this tree
was ported from. The archive speaks a dialect of its own; it is not the v1,
and nothing here follows it.

## Lexical structure

### Comments

A comment begins with a semicolon and runs to the end of the line. There is
no block comment.

```ahk
; a comment on a line of its own
x := 1   ; and one after a statement
FileAppend x, "*"
```

```json
["1"]
```

A `/* */` block is not a comment. The line is refused:

```ahk
/*
block
*/
FileAppend "never", "*"
```

```json
{"ok":false,"error":"This line does not contain a recognized action.\n     Specifically: /*","error_line":1}
```

The semicolon only begins a comment outside a text. Inside quotes it is
part of the text.

**v2:** in an expression, `x := 1;c` is refused: the text after the semicolon
is not a symbol of the dialect.
**v1:** the same line is refused with "The leftmost character above is
illegal in an expression.", and the refusal names the symbol that begins at
the semicolon.

The text of a command position of the v1 is free text, so a semicolon there
still begins a comment.

### Whitespace and the end of a line

A newline ends a statement. Spaces and tabs separate tokens, and the
distinction between "a space came before this token" and "it did not" is
kept by the lexer, because the dot and the increment depend on it: `x . y`
joins two values, `x.y` reaches into one; `x++` is an increment and `x ++` is
a call of `x` with no arguments.

**v2:** `x ++` is a call statement, so the space matters.
**v1:** there is no call without parentheses, so `x ++` is the increment of
`x`.

A trailing `\r` is whitespace, so a script saved with CRLF line endings
reads the same as one with LF.

### Line continuation

**v2:** a line that ends with an operator continues on the next line, and so
does a line that ends inside a pair of brackets or after a comma.

```ahk
; a line that ends with an operator continues
x := 1 +
     2
FileAppend x, "*"
```

```json
["3"]
```

```ahk
; a line that ends inside an array literal continues
a := [1,
      2,
      3]
FileAppend a.Length, "*"
```

```json
["3"]
```

**v1:** the rule is inverted. The line that *begins* with an operator is
merged with the one before it, and a line that ends with an operator does
not continue. `x := 1 +` and a `2` on the next line are two lines, and the
second is refused with "This line does not contain a recognized action."

```ahk
; v1: the bracket does not continue the line either
x := (1 +
     2)
FileAppend, %x%, *
```

```json
{"ok":false,"error":"Missing \")\"","error_line":3}
```

### Continuation sections

A continuation section is a block of lines that belong to a text or to the
statement above it. It opens with a line that holds a `(` and a list of
options, and it closes with a line whose first non-blank character is `)`.
The lines between are the content.

**v2:** the section may follow a text -- the lines become part of the text,
with the newlines kept -- or it may follow any line that has code, and the
tokens of the section are the tokens of that line. The options are `Join`,
`LTrim`, `LTrim0`, `RTrim0`, `Comments` and the backtick, which turns every
escape in the section into a literal character.

```ahk
; a section that continues a text: the lines are the text, newlines and all
t := "
(
  a
  b
)"
FileAppend t, "*"
```

```json
["a\nb"]
```

```ahk
; the section continues the line above it, and the name is separated by a space
t := "x
(
  a
  b
)y"
FileAppend t, "*"
```

```json
["xa\nby"]
```

An option the dialect does not know is refused with the option named:

```json
{"ok":false,"error":"Invalid option.\n     Specifically: Join"}
```

The section is a thing of the v2. The v1 reads the line differently, and a
`(` that opens one is a bracket of an expression, so the construct does not
carry over. Write the v1 equivalent with `%name%` and concatenation, or
with an expression between brackets.

### Escapes

**v2:** the backtick introduces an escape, both inside a text and outside
one.

| Escape | Means |
|---|---|
| `` `n `` | a linefeed |
| `` `r `` | a carriage return |
| `` `t `` | a tab |
| `` `b `` | a backspace |
| `` `f `` | a form feed |
| `` `v `` | a vertical tab |
| `` `a `` | a bell |
| `` `0 `` | a null character |
| `` `s `` | a space |
| `` `` `` `` | a literal backtick |
| `` `" `` | a literal double quote |
| `` `; `` | a literal semicolon |

An escape for a character that is not in the table is the character itself.
A backtick that is not in a name and not in a text is refused:
`unexpected character '`'`.

**v1:** the escape table is the same but for two entries: `` `a `` is a bell
(it is the same in both here) and `` `s `` is the letter `s`, because the
v1 has no `` `s ``. The escape may also appear inside a name, and it is
expanded before the name is looked at: `x := a`c` writes the name `ac`, and
`x := a`b` writes the name `a` followed by a backspace, which is not a legal
name of a variable.

### Texts

**v2:** a text is between double quotes or between single quotes. The source
keeps the quote that was written, which matters for `Goto`: a label looked
for as `'done'` is a label whose name carries the quotes.

```ahk
; both quote characters make a text
a := "double"
b := 'single'
FileAppend a " " b, "*"
```

```json
["double single"]
```

A quote inside a text of the same kind is escaped with the backtick:

```ahk
; a quote inside a text of the same kind, and one that is not
a := "he said `"hi`""
b := 'it`'s'
FileAppend a " / " b, "*"
```

```json
["he said \"hi\" / it's"]
```

A text does not continue on the next line. A quote that is not closed on the
line it opens is refused. The lines after it are a continuation section
only when the line that comes next opens one, with a `(` and its options.

**v1:** the quote escapes itself: `"a""b"` is the text `a"b`.

```ahk
; v1: the quote escapes itself
x := "a""b"
FileAppend, %x%, *
```

```json
["a\"b"]
```

The same line in v2 is refused, because the two texts have no space between
them:

```json
{"ok":false,"error":"Missing space or operator before this.\n     Specifically: \"b\"","error_line":1}
```

**v2:** `x := "a""b"` needs the space. **v1:** it does not, and the two
quotes are one escape.

**v1:** the single quote is not a delimiter of a text. It begins a symbol,
and the symbol is refused where an expression wants an operand with "The
leftmost character above is illegal in an expression."

**v1:** a quote that is not closed inside a command position is not an
error, because the position is free text. `FileAppend, "abc, *` writes the
text `"abc`. In an expression it is refused with "Missing close-quote",
and the refusal names the text that was read.

**v2:** a quote that is not closed is a failure of the lexer, and the script
does not load.

### Numbers

**v2:** a number is a whole number or a float. A whole number is written in
decimal, or in hexadecimal with a leading `0x`; a leading zero does not make
it octal, so `010` is ten. A float has a point and may have an exponent. A
number may begin with a point, `.5`, but only where a value is expected and
not right after a value: `x.5` is the member `5` of `x`.

```ahk
; the number literals of v2: hex, a leading zero, an exponent, a float
FileAppend 0x10 . " " . 007 . " " . 010 . " " . 1e2 . " " . (1.5 + 1), "*"
```

```json
["16 7 10 100.0 2.5"]
```

A whole number that does not fit in sixty-four bits becomes a float:

```ahk
FileAppend 9223372036854775808, "*"
```

```json
["9223372036854776000"]
```

`0b101` and `1_000` are not v2 numbers: the first is refused as "This
variable name starts with a number, which is not allowed", and the second
the same, because a name may not begin with a digit.

**v1:** a number is a "pure number": it keeps the text that was written, and
that text is what the value answers when it is read as a text. `x := 0x10`
makes `x` the text `0x10`, and `StrLen(x)` is four. A whole number that does
not fit is the largest one that does.

```ahk
; v1: the number keeps its text, and an overflow is clamped
x := 0x10
StringLen, n, x
FileAppend, %x% has %n% letters, *
y := 100000000000000000000
FileAppend, |%y%, *
```

```json
["0x10 has 4 letters","|9223372036854775807"]
```

The scientific notation of the v1 needs a point: `1.0e4` is a number and
`1e2` is a name. The number that a number literal becomes when it is used as
a number is the number, so `x := 0x10` and `x + 0` is 16.

**v1:** a name may begin with a number, and the number alone may be the name
of a variable: `0 := 9` writes the variable "0".

### Names

A name begins with a letter or an underscore, and continues with letters,
digits and underscores. The byte order mark is a letter of a name, and a
name may begin with it. Names are not case sensitive, and the built-in
names are matched the same way: `abs` and `Abs` are one name.

**v1:** the name of a variable may also carry `#`, `$` and `@`, at any
position, and a name may begin with a digit.

**v2:** there are no such marks in a name, and a name that begins with a
digit is refused.

### Keywords

The words the language gives a meaning to are reserved:

```
if else switch case return exitapp while until loop try catch finally throw
for break continue goto true false and or not is in contains as class static
global local super
```

**v2:** a reserved word may not be the name of a variable, and the refusal
says which kind of word it is: a word that begins a statement says that it
may not be the name of a variable, an operator and `not`/`true`/`false`/
`super` are a syntax error, and `in` and `contains` are reserved for a
future of v2 and say so.

**v2:** `elseif` is not a word. A branch of an `if` is written with two
words, `else if`.

**v1:** the words may be the name of a variable. `while := 5` writes the
variable "while", and `x := while` reads it. Only the operators (`and`,
`or`, `not`), the values (`true`, `false`) and `if` keep their meaning.

```ahk
; v2: a reserved word is not a variable name
while := 5
```

```json
{"ok":false,"error":"The following reserved word must not be used as a variable name:\n\"while\"","error_line":2}
```

```ahk
; v1: the words may be the name of a variable
while := 5
x := while
FileAppend, %x%, *
```

```json
["5"]
```

## Values and types

A value is an integer, a float, a text, an object, a reference, or the one
value that means "no value at all". There is no separate boolean type:
`true` and `false` are the integers 1 and 0.

```ahk
; every value has a Type, and a class
FileAppend Type(5) " " Type(1.5) " " Type("s") " " Type([1]) " " Type({}), "*"
```

```json
["Integer Float String Array Object"]
```

| Kind | Written as | `Type()` | `__Class` |
|---|---|---|---|
| Integer | `42`, `0x10` | `Integer` | `Integer` |
| Float | `1.5`, `1e2` | `Float` | `Float` |
| String | `"abc"` | `String` | `String` |
| Object | `{}`, `[1]`, `Map()`, an instance of a class | `Object`, `Array`, `Map`, the class name | the same |
| Reference | `&x`, `IsSetRef` | `VarRef` | `VarRef` |
| A class | `Map`, `Thing` | `Class` | `Class` |
| No value | `unset` | -- | -- |

`Type` of an object is its class. `Type(Map())` is `Map`, `Type([1])` is
`Array`, `Type({})` is `Object`, `Type(Abs)` is `Func`, and `Type(Thing())`
is `Thing`.

**v2:** every value carries `__Class`, and writing it on a value that is not
an object is refused with "Property is read-only.".

**v1:** only an object has `__Class`, and a value that is not an object has
none.

### Integers and floats

An integer and a float are two different values and they stay different: a
division that comes out whole is still a float. `7 // 2` is the whole
division and gives `3`; `7 / 2` gives `3.5`; `4 / 2` gives `2.0`.

```ahk
; a division that comes out whole is still a float
FileAppend (7 / 2) . " " . (7 // 2) . " " . (4 / 2), "*"
```

```json
["3.5 3 2.0"]
```

Only two values that are both numbers compare with `<`, `>`, `<=`, `>=`. A
text and a number do compare, because the text is read as a number first:

```ahk
FileAppend 5 < "6", "*"
```

```json
["1"]
```

But two texts do not:

```json
{"ok":false,"error":"Expected a Number but got a String."}
```

**v1:** the comparison of two texts is a text comparison, and `<` works on
texts.

### Texts and coercion

A number turns into a text when a text is wanted, and a text turns into a
number when a number is wanted. That is what makes the arithmetic of the v2
strict: an operation that wants a number refuses a text that is not one.

```ahk
FileAppend "the answer is " 42, "*"
```

```json
["the answer is 42"]
```

A text and a number on either side of `=` compare by value and without
regard to the case. `==` compares with regard to the case.

```ahk
; the equality without and with the case, and a text against a number
FileAppend ("abc" = "ABC") . " " . ("abc" == "ABC") . " " . ("5" = 5), "*"
```

```json
["1 0 1"]
```

**v1:** the case of a comparison is a setting. `=` is always without regard
to the case, `==` follows `StringCaseSense`, and `<>` is the "not equal" of
the v1.

### Truthiness

A value is true unless it is empty or zero.

| Value | True? |
|---|---|
| `0` | no |
| any other number, including `-1` and `0.0` | yes |
| `""` | no |
| any other text | yes |
| an object, a reference | yes |

**v2:** `"0"` is false, because a text that is the number zero is the number
zero. `"00"` and `"0.0"` are true: the text is kept as it was written.

```ahk
; only the exact text "0" is false in v2
FileAppend ("00" ? 1 : 0) " " ("0" ? 1 : 0) " " ("" ? 1 : 0) " " ("0.0" ? 1 : 0), "*"
```

```json
["1 0 0 1"]
```

**v1:** the text of a condition is a number when the syntax of the v1
accepts it, and the condition is false when that number is zero. The text
that was written between quotes is a string, and a string is true when it is
not empty.

```ahk
; v1: "00" is a number, so it is false
if ("00")
  FileAppend, truthy, *
else
  FileAppend, falsy, *
```

```json
["falsy"]
```

### Objects are not texts

**v2:** reading an object as a text is not allowed, and the refusal names
the class. `FileAppend {}` fails with "Expected a String but got an Object.".

**v1:** the text of an object is the empty text. `Object()` is a blank text,
and `Object() . "a"` is "a".

## Variables

A variable is a name and a value. A name that was never given a value has no
value, and reading it is a failure in v2 and an empty text in v1.

```ahk
; v2: reading a variable that has no value fails
FileAppend nothere, "*"
```

```json
{"ok":false,"error":"This global variable has not been assigned a value.","error_line":2}
```

**v1:** the same read is the empty text, and the script goes on.

```ahk
; v1: a variable that has no value is blank
FileAppend, [%nothere%], *
```

```json
["[]"]
```

### Assignment

Assignment is `:=`. The compound forms are `+= -= *= /= //= &= |= ^= <<= >>=
>>>=` and the concatenation `.=`. The increments are `++` and `--`, before
or after the name.

```ahk
; assignment and its compound forms
x := 5
x += 2
x *= 3
x .= "!" 
FileAppend x, "*"
```

```json
["21!"]
```

One statement may assign several names, separated by commas:

```ahk
x := 1, y := 2
FileAppend x "," y, "*"
```

```json
["1,2"]
```

**v1:** `=` assigns too, and it assigns a **text**: `x = 5` makes `x` the
text `5`, and `x = 5 + 5` makes it the text `5 + 5`. `:=` assigns an
expression.

```ahk
; v1: the equals sign assigns a text, the colon-equals an expression
a = 5
b = 5 + 5
c := 5 + 5
FileAppend, %a%|%b%|%c%, *
```

```json
["5|5 + 5|10"]
```

### Global, local and static

At the top level of a script, a name is a global. Inside a function the name
is a local, and it is a new variable: the function does not see the globals
unless it is told to.

`global` names one or more globals for the function, or, alone, makes every
name in the function a global. `static` makes a variable that keeps its
value between the calls.

```ahk
; a static keeps its value between the calls
Counter() {
  static n := 0
  n += 1
  return n
}
Counter()
Counter()
FileAppend Counter(), "*"
```

```json
["3"]
```

```ahk
; a global inside a function reaches the variable of the script
set_it() {
  global g
  g := 42
}
set_it()
FileAppend g, "*"
```

```json
["42"]
```

**v2:** a function that only *reads* a name sees the global, and a write
makes a local. The rule is not "the name is local because the function
mentions it"; the read of a name that is not a local of the function looks
in the script. Writing to it needs the declaration.

```ahk
; the function reads the global the script wrote...
g := 7
read_it() {
  return g
}
FileAppend read_it(), "*"
```

```json
["7"]
```

**v1:** every function has its own scope. The function does not see the
scope of the script, nor the scopes around the call, and only a `global`
declaration makes a name of the script visible in it.

```ahk
; v1: a function does not see the variable of the script
g = 7
r := ReadIt()
FileAppend, [%r%], *
ReadIt()
{
  return g
}
```

```json
["[]"]
```

```ahk
; v1: the global declaration is what makes the name visible
g = 7
r := ReadIt()
FileAppend, [%r%], *
ReadIt()
{
  global g
  return g
}
```

```json
["[7]"]
```

A v1 definition is `f(a)` with the brace on the line that comes next, or
`f(a) {` with the brace on the line of the parenthesis and nothing after it.
The v1 decides with the rest of the line: `f(a) { return a }` is a **call**
of `f`, and it is refused with "Call to nonexistent function.".

**The declarations in v2.**

| Form | Means |
|---|---|
| `global` | every name of the function is a global |
| `global x, y` | `x` and `y` are the globals of the script |
| `local` | no name of the function reaches the script |
| `local x` | `x` is a local, whatever the script holds |
| `static` | every name of the function that is assigned is static |
| `static x := 1` | `x` keeps its value between the calls |

**v1:** a bare `local` is a declaration of no name inside a function, and a
bare `local` or `static` is only accepted as the first statement of the
body. At the top level of the script the bare `global` is refused with "This
line does not contain a recognized action.", because the script is not a
function.

**v2:** the bare `global` at the top level is accepted.

### Writing a name the script builds

The `%name%` form in an expression reads the variable whose name is the
value of `name`.

```ahk
; a name built by the script
which := "target"
target := 99
FileAppend %which%, "*"
```

```json
["99"]
```

A write of such a name is `%name% := value`.

**v2:** the write inside a function is refused unless the name is a local of
the function or the script has the declaration: "This dynamic assignment
requires a \"global\" declaration.".

**v1:** the write inside a function is refused with "This dynamic variable is
blank. If this variable was not intended to be dynamic, remove the % symbols
from it.", whatever the variable of that name is.

### The built-in variables

The dialect carries a set of variables that the runtime answers: `A_Index`,
`A_LineNumber`, `A_Now`, `A_ScreenWidth`, `A_Clipboard`, the loop variables
(`A_LoopFileName`, `A_LoopReadLine`, `A_LoopField`, ...) and the rest. The
full list with the state of each name is in `docs/ahk-v2-reference.md` for
v2 and `docs/ahk-v1-reference.md` for v1.

```ahk
FileAppend A_LineNumber, "*"
```

```json
["1"]
```

**v2:** a built-in variable that the dialect marks as read-only is refused
when a script writes it, and the refusal names the name as the script spelled
it. A built-in that the dialect lets a script write is kept in the table of
the script.

**v1:** no built-in variable of the v1 may be written, and the refusal is
"Not allowed as an output variable." with the name. The concatenation is the
one exception: `A_X .= "y"` is accepted and the built-in keeps the value it
had.

**v1:** the `%name%` of a parameter looks at the environment when the script
has no variable of that name, and `#NoEnv` turns the lookup off.

**v2:** the `%name%` never looks at the environment.

`ErrorLevel` is the variable the v1 commands write. It is not a variable of
v2.

```ahk
; v1: a command that failed leaves the code in ErrorLevel
FileDelete, no_such_file_xyz.txt
FileAppend, ErrorLevel is %ErrorLevel%, *
```

```json
["ErrorLevel is 1"]
```

## Operators

### Arithmetic

| Operator | Means | Example | Answer |
|---|---|---|---|
| `+` | add | `1 + 2` | `3` |
| `-` | subtract | `5 - 2` | `3` |
| `*` | multiply | `3 * 4` | `12` |
| `/` | divide | `7 / 2` | `3.5` |
| `//` | whole divide | `7 // 2` | `3` |
| `**` | power | `2 ** 10` | `1024` |
| `**` | power | `2 ** 10` | `1024` |
| `-x` | negate | `-3` | `-3` |
| `+x` | unary plus | `+3` | `3` |

There is no remainder **operator**: `%` is the deref of a name. The remainder
is the function `Mod`:

```ahk
; the remainder is a function, and the power binds tighter than the minus
FileAppend Mod(7, 3) . " " . (2 ** 10) . " " . (-2 ** 2), "*"
```

```json
["1 1024 -4"]
```

A script that writes the remainder operator is refused:

```ahk
; `%` is the deref of a name, not a remainder
FileAppend (7 % 3), "*"
```

```json
{"ok":false,"error":"Missing ending \"%\"\n     Specifically: % 3), \"*\")","error_line":2}
```

`-2 ** 2` is `-4`: the power binds tighter than the unary minus.

A division by zero is a failure in v2, and a blank in v1.

```ahk
try
  x := 1 / 0
catch as e
  FileAppend e.__Class ": " e.Message, "*"
```

```json
["ZeroDivisionError: Divide by zero."]
```

```ahk
; v1: a division by zero is blank
x := 1 / 0
FileAppend, [%x%], *
```

```json
["[]"]
```

**v1:** the sum of a text and a number is a concatenation. `"5" + 1` is `51`,
and so is `5 + "1"`.

```ahk
; v1: the plus of a written text concatenates
FileAppend, % ("5" + 1), *
```

```json
["51"]
```

**v2:** the same expression refuses a text on the left of the `+` that is
not a number: "Expected a Number but got a String.".

### Concatenation

Two values side by side are one value. The space between them is what makes
it so, and the dot does the same thing with an explicit operator.

```ahk
; side by side, and with the dot
FileAppend ("a" 1 + 2) . " " . (1 + 2 "d") . " " . (5 . 6), "*"
```

```json
["a3 3d 56"]
```

The precedence is why `"a" 1 + 2` is `a3` and `1 + 2 "d"` is `3d`: the
concatenation binds looser than the addition, so the addition is done first
on each side, and then the pieces are joined.

**v2:** a pair of values with no space between them is refused with "Missing
space or operator before this."
**v1:** the space is not needed.

### Comparison

| Operator | Means |
|---|---|
| `=` | equal, without regard to the case |
| `==` | equal, with regard to the case |
| `!=` | not equal, with regard to the case |
| `!==` | not equal, including the type |
| `<`, `>`, `<=`, `>=` | the order of two numbers |
| `~=` | the left text matches the right pattern |
| `is` | the left value is of the right class |
| `in`, `contains` | reserved for a future of v2; they do not work yet |

```ahk
; v2: the equality with the case, the without, and a case-insensitive difference
FileAppend ("abc" = "ABC") . " " . ("abc" == "ABC") . " " . ("abc" != "ABC"), "*"
```

```json
["1 0 0"]
```

The pattern of `~=` is exact, and the one option the engine carries is
`(?i)`, which takes the case away:

```ahk
FileAppend "A" ~= "(?i)a", "*"
```

```json
["1"]
```

`is` compares a value with a class, and every object is an `Object`:

```ahk
; a value is of the class of its prototype chain
class Alpha {
}
class Beta extends Alpha {
}
b := Beta()
FileAppend (b is Beta) " " (b is Alpha) " " (b is Object) " " (3 is Object), "*"
```

```json
["1 1 1 0"]
```

See [Known gaps](#known-gaps) for the values that are not objects.

**v1:** `<>` is the not-equal of the v1, and the v2 does not know it.
**v1:** `<` and `>` compare two texts when both sides are texts.

### Logic

`and`/`&&`, `or`/`||`, `not`/`!`, and the or-maybe `??`.

**v2:** `&&` and `||` answer the operand and not a truth: `1 && 2` is `2`,
and `0 or 5` is `5`.

```ahk
; the logical operators answer the operand
FileAppend (1 && 2) . " " . (0 or 5) . " " . (not 0), "*"
```

```json
["2 5 1"]
```

**v1:** `&&` and `||` answer one and zero.

```ahk
; v1: the logic answers a number
FileAppend, % ("a" && "b") "|" (1 || 0), *
```

```json
["1|1"]
```

`??` answers the left side when the variable has a value, and the right side
when it does not:

```ahk
; the or-maybe of a global that was never assigned
F() {
  global q
  return q ?? 7
}
FileAppend F(), "*"
```

```json
["7"]
```

`??` is guarded by v2's unset rule: the left side must be a name.

### Bitwise and shifting

`|`, `^`, `&`, `<<`, `>>`, `>>>`, `~`, and the compound forms of each. The
`&` in front of a value is the reference: `&x` is a pointer to the variable
`x` and not the bit-and of anything.

```ahk
; the bitwise operators and a shift
FileAppend (6 & 3) . " " . (6 | 3) . " " . (1 << 4), "*"
```

```json
["2 7 16"]
```

### Assignment operators

`:=`, `+=`, `-=`, `*=`, `/=`, `//=`, `|=`, `^=`, `&=`, `<<=`, `>>=`, `>>>=`,
`.=`, and `++`/`--`.

### Increment

`x++` and `x--` are postfix, and `++x` and `--x` are prefix. The difference
is the value the expression leaves: the postfix form leaves the value the
variable had, and the prefix form leaves the new one.

```ahk
x := 5
FileAppend x++ . " " . x . " " . (++x), "*"
```

```json
["5 6 7"]
```

**v2:** `x ++`, with a space, is a call of `x` and not an increment.
**v1:** the space does not matter. `5--3` is five minus three in v2 and an
increment in v1, where `++` and `--` after a number are the increment of the
variable of that name.

**v2:** the increment of a literal is refused at the load.
**v1:** the increment of a literal is accepted, and the value of the
expression is the empty text.

### Precedence

From the loosest to the tightest. The numbers are the ones in
`ahk::precedence`, which is the table of the v2 documentation.

| Precedence | Operators |
|---|---|
| 1 | `,` |
| 2 | `:=` `+=` `-=` `*=` `/=` `//=` `|=` `^=` `&=` `<<=` `>>=` `>>>=` `.=` |
| 3 | `? :` |
| 4 | `??` |
| 5 | `or` `\|\|` |
| 6 | `and` `&&` |
| 7 | `not` |
| 8 | `is` `in` `contains` |
| 9 | `=` `==` `!=` `!==` |
| 10 | `<` `>` `<=` `>=` |
| 11 | `~=` |
| 12 | `,` the concatenation of a space, and `.` |
| 13 | `\|` |
| 14 | `^` |
| 15 | `&` |
| 16 | `<<` `>>` `>>>` |
| 17 | `+` `-` |
| 18 | `*` `/` `//` |
| 19 | `-x` `+x` `!x` `~x` `&x` |
| 20 | `**` |
| 21 | `++` `--` |

The power binds tighter than the unary minus, which is why `-2 ** 2` is `-4`.
The concatenation binds tighter than the comparison and looser than the
addition, which is why `"a" 1 + 2` is `a3`.

## Statements

A v2 statement is a line. The forms are:

| Form | Example |
|---|---|
| an assignment | `x := 1` |
| an expression | `1 + 1` |
| a call, with parentheses | `Abs(-3)` |
| a call, without parentheses | `Abs 5` |
| a member call, without parentheses | `obj.Method 1, 2` |
| a block | `{ ... }` |
| a control-flow word | `if`, `loop`, `while`, `for`, `switch`, `try`, `return`, `break`, `continue`, `throw`, `goto`, `exitapp` |
| a function definition | `Name(a) { ... }` |
| a class definition | `class Name { ... }` |
| a declaration | `global x`, `static y := 1` |
| a directive | `#Warn` |

### The command-call form

A name at the start of a line, with no parentheses, is a call statement whose
parameters are the rest of the line, separated by commas. Every function may
be written that way: a built-in and a function of the script alike.

```ahk
; FileAppend and Abs as call statements
FileAppend "hello", "*"
Abs 5
FileAppend "done", "*"
```

```json
["hello","done"]
```

The parameters may be expressions, and a comma separates them. The space
before the first parameter is what tells the parser that this is a call and
not the bare name of a variable:

```ahk
; the space is the operator, and a member call works the same way
class Point {
  __New(x, y) {
    this.x := x
    this.y := y
  }
  Show(gap) {
    return this.x gap this.y
  }
}
p := Point(1, 2)
p.Show ":"
FileAppend "done", "*"
```

```json
["done"]
```

**v2:** there is no "command" as a kind of name. The parser decides by the
syntax, and the v2 documentation says that every function may be written
this way.

### The legacy command form

The v1 has commands, and a command is a name, a comma, and the positions of
the command. A position is **arguments of the command**, not expressions:
most of them are free text, and the ones that name a variable to write are
names.

```ahk
; v1: a command with its positions, the first one a text
FileAppend, hello world, *
```

```json
["hello world"]
```

```ahk
; v1: the second position is the name of the variable to write
text = hello world
StringLen, n, text
FileAppend, %text% is %n% letters, *
```

```json
["hello world is 11 letters"]
```

The positions come in three shapes, and the reference table says which shape
each one has:

| Shape | Means | Example |
|---|---|---|
| Value | an expression, or the text of the position | `FileAppend, %x%, *` |
| Output | the name of a variable the command writes | `StringLen, n, text` writes `n` |
| Variable | the name of a variable the command reads or writes | `VarSetCapacity(y, 100)` empties `y` |

**The `%name%` substitution.** A position is text, and `%name%` inside it is
the value of that variable. A position that begins with `%` and a space is an
**expression**: `FileAppend, % x + 1, *` writes the value of the expression.

A position that begins with `%` and a space is an **expression**:

```ahk
; v1: the percent-substitution in a position, and a forced expression
x = 5
FileAppend, x is %x%, *
FileAppend, % "x+1 is " (x + 1), *
```

```json
["x is 5","x+1 is 6"]
```

**v1:** the command must be a command of the v1. `StrLen "abc"` is a line
that holds no action, and the load refuses it with the text of the line,
because `StrLen` is a function of the v1 and not a command.

**v1:** the command's number of positions is checked when the script loads,
and the complaint is "X requires at least N parameter(s)." or "Parameter #N
required".

**v2:** a call is checked when it runs, and the complaint names the function
when the script writes the call, and does not when the call goes through a
value.

### Blocks

A block is a brace, statements, and a brace that closes it. The brace is on
the line of the construct that takes the block, and a statement after the
closing brace is not part of the block.

```ahk
; a block
if (1) {
  x := 1
  y := 2
}
FileAppend x "," y, "*"
```

```json
["1,2"]
```

**v2:** content on the line of the opening brace is refused: `if (1) { return
2 }` fails with "Unexpected \"If\"".
**v1:** the same line is refused with "Unexpected \"{\"".

### The semicolon and the comma in a list

A statement may be a list of items separated by commas, and the items run in
order:

```ahk
; a comma at the start of a line continues the list of the line above
a := 1, b := 2, c := a + b
FileAppend c, "*"
```

```json
["3"]
```

## Control flow

### if and else

`if` takes a condition, and the body is a statement or a block. `else` takes
the other side, and `else if` chains them.

```ahk
; if, else if and else
n := 2
if (n = 1)
  FileAppend "one", "*"
else if (n = 2)
  FileAppend "two", "*"
else
  FileAppend "many", "*"
```

```json
["two"]
```

The condition is an expression. The parentheses may be left out, and the
last word of the condition may be the operand:

```ahk
x := 1
if x
  FileAppend "true", "*"
```

```json
["true"]
```

**v1:** the `if` is a command and its forms are its own:

| v1 form | Means |
|---|---|
| `if (expr)` | the expression is the condition |
| `if Var = Text` | the legacy comparison: the value of `Var` against the **text** that follows |
| `IfInString, var, text` | the value contains the text |
| `IfNotInString, var, text` | it does not |
| `IfWinActive` | a window is active |
| `IfMsgBox` | the last message box answered a button |
| `IfEqual`, `IfGreater`, ... | the legacy comparisons |

```ahk
; v1: the legacy comparison is against the text, not the expression
x = 5
if x = 1 + 1
  FileAppend, compared-with-text-1+1, *
else
  FileAppend, not-equal, *
```

```json
["not-equal"]
```

```ahk
; v1: the contains commands, with the body on the next line
x = abc
IfInString, x, b
  FileAppend, has-b, *
```

```json
["has-b"]
```

**v1:** the `if` of the v1 parses the text that follows the condition and
does not run it. `if (1) FileDelete, marker.txt` on one line does not delete
anything; the body is the line that comes next.

**v1:** the v1 does not report a failure of the condition. `if 1/0` is false
and the body does not run. **v2:** the failure of a condition is reported.

**v1:** `else` is a command of its own, and it takes no parameter. It is the
other side of the `if` that comes before it.

### The ternary

`cond ? a : b` is an expression. It evaluates one side only.

```ahk
n := 0
FileAppend (n ? "yes" : "no"), "*"
```

```json
["no"]
```

### switch

`switch` compares the value of the subject with each `case`, and runs the
body of the first that matches. `default` takes the rest. A case may list
several values with commas, and a case with no body shares the body of the
next one.

```ahk
; switch with a default
n := 5
switch n {
case 1:
  FileAppend "one", "*"
case 2, 3:
  FileAppend "two or three", "*"
default:
  FileAppend "something else", "*"
}
```

```json
["something else"]
```

**v2:** the cases are compared without regard to the case, and a case may be
an expression or a range.

**v1:** `switch` is a command with its own grammar, and `case` and `default`
are names there.

### goto and labels

**v2:** `goto` names a label, and a label is a name followed by a colon at the
start of a line. A goto must not jump into a block that does not enclose it.

```ahk
; a label and a goto
goto Done
FileAppend "never", "*"
Done:
FileAppend "done", "*"
```

```json
["done"]
```

**v1:** the labels are how a hotkey and a timer name their action, and
`goto` reaches them the same way.

**v1:** `Gosub` is not carried by this runtime. A line that begins with it
is refused as "This line does not contain a recognized action.", because the
name is not a command of the v1 table here. Write a function instead, and
call it.

### exitapp

`ExitApp` ends the script with a code, and the code goes into the `exit_code`
of the report:

```ahk
; the code of the end of the script goes into the report
FileAppend "stopping", "*"
ExitApp 3
```

```json
{"ok":true,"output":["stopping"],"exit_code":3}
```

**v1:** `ExitApp, code` is a command.

### return, break and continue

`return` ends the call and answers a value. `break` leaves the loop.
`continue` goes to the next turn of the loop.

```ahk
; break and continue
Loop 5 {
  if (A_Index = 2)
    continue
  if (A_Index = 4)
    break
  FileAppend A_Index, "*"
}
```

```json
["1","3"]
```

`return` outside a function ends the script, and so does `return` with no
value at the top level.

## Loops

### The counted loop

`Loop N` runs the body N times, and `A_Index` counts the turns from one.

```ahk
; a counted loop
Loop 3
    FileAppend A_Index, "*"
```

```json
["1","2","3"]
```

**v2:** the count is an expression.
**v1:** the count is a number and not an expression: `loop n` is skipped
unless `n` is a number, and so are `loop 1+1` and `loop 2.7`.

### The conditional loop

`while cond` runs the body while the condition is true.

```ahk
; a while loop
i := 0
while (i < 3) {
  i += 1
  FileAppend i, "*"
}
```

```json
["1","2","3"]
```

**v2:** there is no `until` clause. `Loop` without a count goes on until a
`break`, and the `until` word that no loop takes is refused with "Unexpected
\"Until\"".

**v1:** `until cond` is the condition of the loop whose body comes before it,
and it is checked at the end of each turn:

```ahk
; v1: until is the condition of the loop, checked at the end of the turn
i = 0
Loop
{
  i += 1
}
Until i >= 3
FileAppend, %i%, *
```

```json
["3"]
```

A v1 `until` that no loop takes is refused with "UNTIL with no matching
LOOP".

### The for loop

`for value in source` walks the values of an enumerable. `for key, value in
source` walks the key and the value together.

```ahk
; for over an array, and over a map with its keys
a := [10, 20]
for v in a
    FileAppend v, "*"
m := Map("a", 1, "b", 2)
FileAppend "|", "*"
for k, v in m
    FileAppend k "=" v " ", "*"
```

```json
["10","20","|","a=1 ","b=2 "]
```

`A_Index` counts the turns of a `for` loop too.

```ahk
; the index of a for loop
for v in [7, 8]
    FileAppend A_Index ":" v " ", "*"
```

```json
["1:7 ","2:8 "]
```

**v2:** the source must be enumerable. A source that is not is refused with
"Value not enumerable." on the line of the loop.

**v1:** the `for` of the v1 walks the **keys** of an array and nothing else:
`for i in [10, 20]` gives `i` the values 1 and 2. The source that is not an
array walks nothing and does not fail. The v1 also names the word that is
missing: "This \"For\" is missing its \"in\".".

```ahk
; v1: the for variable takes the key, not the value
a := [10, 20]
for i in a
  FileAppend, %i%, *
```

```json
["1","2"]
```

### Loop Files

`Loop Files` walks the files of a folder. The loop variables of v2 carry the
parts of each name.

```ahk
; the loop variables of a file loop
FileAppend Type(A_LoopFileName), "*"
```

```json
["String"]
```

**v1:** the flavours of the v1 loop are `Loop Files`, `Loop, Read, file`,
`Loop, Parse, var, delimiters` and `Loop, Reg`. The loop variables of the v1
are `A_LoopFileName`, `A_LoopFileFullPath`, `A_LoopReadLine`, `A_LoopField`
and the rest of the family.

### The guard rails

A loop cannot run forever. The runtime stops a script that does not finish:
20 000 statements, 1 000 turns of a `while` body, 1 000 nested calls. A loop
that trips a limit reports it as an error on the line of the statement:

```ahk
; a loop that never ends trips the statement limit
Loop {
  x := 1
}
```

```json
{"ok":false,"error":"the script ran more than 20000 statements","error_line":3}
```

## Functions

A function is a name, a list of parameters, and a body. In v2 it is a
definition at the top level of the script:

```ahk
; a function, called with and without parentheses
Greet(name) {
  return "hello, " name
}
FileAppend Greet("world"), "*"
FileAppend Greet("again"), "*"
```

```json
["hello, world","hello, again"]
```

The two calls are different. `Greet("world")` is the call of the name.
`Greet "again"`, with no parentheses, is a call statement whose parameter is
the text `"again"`. A bare `Greet` with nothing after it is the **function as
a value**, and v2 hands the function over instead of calling it:

```ahk
; a bare name in a value position is the function, not a call
Greet(name) {
  return "hello, " name
}
FileAppend " " Greet "again", "*"
```

```json
{"ok":false,"error":"Expected a String but got a Func.","error_line":5}
```

```ahk
; the name of a function is a value, and its Call method calls it
f := Abs
FileAppend f.Call(-3), "*"
```

```json
["3"]
```

### Parameters

| Form | Means |
|---|---|
| `a` | a required parameter |
| `a := 1` | a parameter with a default |
| `a?` | a parameter that may be left out, and has no value when it is |
| `rest*` | the rest of the arguments, as an array |
| `byref` | a parameter that holds a reference |
| `&target` | a parameter that is an alias of the variable of the caller |

```ahk
; a default and a rest parameter
Sum(a, b := 10, more*) {
  total := a + b
  for v in more
    total += v
  return total
}
FileAppend Sum(1) . " / " . Sum(1, 2) . " / " . Sum(1, 2, 3, 4), "*"
```

```json
["11 / 3 / 10"]
```

The number of the arguments is checked. A call that the script writes is
checked, and the failure names the function:

```json
{"ok":false,"error":"Too few parameters passed to function.\n     Specifically: Abs"}
```

A method is checked at the run and says it in other words: "Missing a
required parameter." for too few, and "Too many parameters passed to
function." for too many.

**v1:** the default of a parameter may be written with the equals sign too:
`f(a = 5)` and `f(a := 5)` are the same. The call is checked when the
script loads, and a name that is not a function is refused there with "Call
to nonexistent function.".

### By-reference parameters

A parameter written `&Left` is an alias of the variable of the caller:
reading it reads there, and writing it writes there. The caller passes a
reference, `&a`.

```ahk
; a by-reference swap
Swap(&Left, &Right) {
  t := Left
  Left := Right
  Right := t
}
a := 1
b := 2
Swap(&a, &b)
FileAppend a "," b, "*"
```

```json
["2,1"]
```

A built-in that writes back uses the same marker:

```ahk
; a built-in with by-reference outputs
SplitPath "C:\dir\file.txt", &name, &dir, &ext
FileAppend name "|" dir "|" ext, "*"
```

```json
["file.txt|C:\\dir|txt"]
```

A parameter that is not written that way holds the reference as a value of
its own, and reading it fails. See [References](#references).

**v1:** a by-reference parameter is written `ByRef name`, and the caller
passes the variable. The commands of the v1 write into a variable of the
caller through the **name** of the variable, not through a reference.

### Closures and lambdas

A lambda is an expression: `(a, b) => a + b` is a function with two
parameters and one expression as its body. A lambda may also be a block.

```ahk
; a lambda as a value
Twice := (x) => x * 2
FileAppend Twice(21), "*"
```

```json
["42"]
```

A function value is called through `Call`, or with the name of the variable
and parentheses:

```ahk
; a function value called through Call, and called by its name
Add := (a, b) => a + b
FileAppend Add.Call(2, 3) . " / " . Add(2, 3), "*"
```

```json
["5 / 5"]
```

**v1:** the name of a function is not a value. `x := StrLen` leaves `x`
blank, and the name of a command is not a value either.

### Where a definition goes

**v2:** functions are declared at the top level of the script, or as methods
of a class. The definition of a function does not have to come before its
call: the reader of the script finds it wherever it is.

```ahk
; the definition may come after the call
Outer() {
  return Inner()
}
Inner() {
  return 7
}
FileAppend Outer(), "*"
```

```json
["7"]
```

A definition inside another function is accepted too, and the inner name is
visible inside the outer body:

```ahk
; a definition inside a definition
F() {
  Inner() {
    return 1
  }
  return Inner()
}
FileAppend F(), "*"
```

```json
["1"]
```

**v1:** a function may be defined at the top level, and the reader of the
script finds it wherever it is, before or after the call. A v1 definition is
told apart from a call by the rest of the line -- see the section on
functions above.

## Classes and objects

### A class definition

A class is a name, an optional base, and a body of members.

```ahk
; a class with a constructor and a method
class Point {
  __New(x, y) {
    this.x := x
    this.y := y
  }
  Show() {
    return this.x "," this.y
  }
}
p := Point(3, 4)
FileAppend p.Show(), "*"
```

```json
["3,4"]
```

`__New` is the constructor, and it runs when the class is called. `this` is
the instance.

### The members

| Member | Written as |
|---|---|
| a method | `Name(a) { ... }` |
| a static method | `static Name(a) { ... }` |
| a property | `name { get => ..., set => ... }` |
| a static property | `static name := value` |
| an instance property | `name := value` |
| a nested class | `class Name { ... }` inside the body |

```ahk
; a property with a getter and a setter, and a static
class Counter {
  static Count := 0
  value {
    get => this._value
    set => this._value := value
  }
  static Bump() {
    Counter.Count += 1
  }
}
c := Counter()
c.value := 41
Counter.Bump()
FileAppend c.value " " Counter.Count, "*"
```

```json
["41 1"]
```

The body of a getter or a setter may be an expression after `=>` or a block
of statements between braces.

**v1:** there is no static method. `static Say() { ... }` is refused with
"Invalid class variable declaration." with the line under it. The v1 has the
`__Init` constructor, and `new C()` makes the instance.

**v1:** the body of the getter is a block of statements only, and an
expression after `=>` is refused with "Not a valid property getter/setter.".

**v2:** a duplicate name in the class body is refused, and the refusal names
the path of the member: "C.Prototype.Foo".
**v1:** the duplicate names the member that was written: "Duplicate
declaration." with "x".

### this, base and super

`this` is the object the method was called on. `super` reaches the member of
the base class.

```ahk
; super reaches the method of the base
class Base {
  Name() {
    return "base"
  }
}
class Child extends Base {
  Name() {
    return "child<" . super.Name() . ">"
  }
}
FileAppend Child().Name(), "*"
```

```json
["child<base>"]
```

`super` takes no parameters of its own, and `super.Name()` is the method of
the base. `super.Prop` and `super[key]` are the property and the item of the
base.

**v1:** the word is `base`, and `base.Method()` calls the method of the base
of the class where the method was defined.

```ahk
; v1: base is the word, and the braces go where the v1 puts them
class Base {
  Say() {
    return "base"
  }
}
class Child extends Base {
  Say() {
    return "child:" . base.Say()
  }
}
c := new Child()
FileAppend, % c.Say(), *
```

```json
["child:base"]
```

**v2:** the base of an object is the prototype of its class, and `super` is
the way to reach it.
**v1:** the base is the base of the class where the method was defined, and
the word `base` reaches it.

### The prototype and __Class

Every class has a `Prototype`, and an instance of the class has it in its
chain. The name of the class of a value is `__Class`.

```ahk
; the class of an instance, of a plain object, and of a class
class Thing {
}
t := Thing()
o := {a: 1}
FileAppend t.__Class . " " . o.__Class . " " . Thing.__Class, "*"
```

```json
["Thing Object Class"]
```

**v2:** every value carries `__Class`: `Integer`, `Float`, `String`, `Array`,
`Map`, `VarRef`, `Func`, the name of the class of the script. Writing it on
a value that is not an object is refused with "Property is read-only."; on an
object it is written like any other property and shadows the real class.

**v1:** only an object has `__Class`, and a plain object of the v1 has no
class, so its `__Class` is empty.

### new

**v2:** there is no `new`. The class is called: `Point(3, 4)` makes an
instance.
**v1:** `new` makes the instance: `p := new Point(3, 4)`. The word is not
reserved, so `new := 5` is a variable of the script.

**v1:** the class of the v1 is a variable: `new` reads the variable, and a
class name that the script assigned something else makes nothing.

**v2:** the name of a class may not be an output variable: `Thing := 5` is
refused with "This Class cannot be used as an output variable."

### Plain objects

An object literal `{a: 1, b: 2}` makes a plain object with the properties
named.

```ahk
; an object literal, read and written
o := {a: 1, b: 2}
o.c := 3
FileAppend o.a "," o.b "," o.c, "*"
```

```json
["1,2,3"]
```

```ahk
; the base object has HasProp, HasOwnProp and the rest
o := {a: 1}
FileAppend o.HasProp("a") "," o.HasProp("z"), "*"
```

```json
["1,0"]
```

**v2:** the item `o["a"]` of a plain object is not its property: the v2 asks
the `__Item` of the object, and a plain object has none. `o["a"]` fails with
"This value of type \"Object\" has no property named \"__Item\".".
**v1:** the item of an object is the property of that name. `o["b"]` reads
the property `b`, `o["B"]` reads it too, and `o["b"] := 5` writes it.

**v2:** a member that is not there is refused, and the refusal names the type
and the member: "This value of type \"Integer\" has no property named
\"nothing\".".
**v1:** a member that is not there answers an empty text, and so does the
member of a value that is not an object.

**v2:** a property name in an object literal must be a name, and a computed
name is written `{%x%: 1}`.
**v1:** the name of a property may be any text, and a computed name is
refused with "Quote marks are required around this key.".

## The class types of the runtime

The runtime carries classes the script cannot extend. They are classes:
their instance is an object, their methods are methods, and their `__Class`
is their name.

| Class | What it is |
|---|---|
| `Object` | the root of the chain of the classes |
| `Array` | an array, `[1, 2, 3]` |
| `Map` | a map, `Map("a", 1)` |
| `Buffer` | a block of bytes |
| `File` | an open file |
| `Func` | a function, a method or a lambda |
| `Gui`, `Menu`, `Hotkey` | the desktop objects |
| `Enumerator` | the walker of a loop |
| `ClipboardAll` | the state of the clipboard |
| `Error` and the error classes | the failures |

The members of each are in `docs/ahk-v2-reference.md`, under "The methods of
the class types".

### Arrays

An array is `[1, 2, 3]`. Positions count from one. `Length` is the number of
places, `Push` adds at the end, `InsertAt` and `RemoveAt` insert and remove.

```ahk
; an array counts from one
a := [1, 2, 3]
a.Push(4)
FileAppend a[1] "," a[4] "," a.Length, "*"
```

```json
["1,4,4"]
```

**v2:** an index that is not there is refused with "Invalid index.", and so
is a write to one.

```ahk
; v2: an array does not grow with the write of a missing place
a := [1, 2]
a[5] := 7
```

```json
{"ok":false,"error":"Invalid index.","error_line":3}
```

**v1:** the array of the v1 is an object, and `a.Length()` is a **method**,
not a property: `a.Length` is an empty text. The v1 has `Insert` and
`Remove`, where the v2 has `InsertAt` and `RemoveAt`. A place that is not
there reads as the empty text, and writing it makes the array grow:

```ahk
; v1: the write of a missing place grows the array
a := [1, 2]
a[5] := 7
FileAppend, % a[5] "|" a.Length(), *
```

```json
["7|5"]
```

```ahk
; v1: the length is a method, and the names of the insert and the remove differ
a := ["x", "y"]
a.Insert("z")
FileAppend, % a.Length() "|" a[3], *
```

```json
["3|z"]
```

### Maps

A map is `Map(key, value, ...)`. A key that is not there is refused with
"Item has no value.".

```ahk
; a map, written and read
m := Map("x", 1)
m["y"] := 2
FileAppend m["x"] "," m["y"] "," m.Count, "*"
```

```json
["1,2,2"]
```

There is no map class in the v1. An object of the v1 serves the purpose, and
its item is its property.

## References

A reference is a value of its own kind, and a v2 names it a `VarRef`. It
points at a variable by its name and by its scope. It is not a text and not
a number; it is an object, so `IsObject` of a reference is one.

A reference is written `&name`, and a built-in that wants a variable takes
one:

```ahk
; IsSetRef wants a reference, not the value
x := 1
a := IsSetRef(&x)
b := IsSetRef(&nothere)
FileAppend a "," b, "*"
```

```json
["1,0"]
```

`IsSet` is carried by this runtime: it asks whether the name it is given
has a value.

The reference of a built-in variable, of a function and of a class is
refused. The reference is a value, so `IsObject(&x)` is 1, and an operation
that wants a text or a number says that it got a `VarRef`.

**v1:** a reference is not a value. A command that wants a variable takes
the **name** of the variable as a position, and the v1 function that wants
one says so in its table.

## Errors

### try, catch and finally

`try` runs a body, `catch` handles a failure, and `finally` runs either way.

```ahk
; try and catch
try {
  x := 1 / 0
}
catch as e {
  FileAppend e.__Class ": " e.Message, "*"
}
```

```json
["ZeroDivisionError: Divide by zero."]
```

**v2:** the catch is followed by `as name`, or by a class and `as name`, or
by a class alone, or by nothing. The catch runs when the failure is of the
class named, or of a class derived from it:

```ahk
; the catch of a class catches its derived classes too
try
  throw TypeError("t")
catch Error as e
  FileAppend "base caught: " e.__Class, "*"
```

```json
["base caught: TypeError"]
```

```ahk
; finally runs whether the body succeeded or not
try
  FileAppend "body", "*"
finally
  FileAppend " finally", "*"
```

```json
["body"," finally"]
```

A bare `catch e`, with a name and no `as`, is **not** accepted by this
runtime: it is refused with "the class of a catch is not a class of this
runtime: e". Write `catch as e`. See [Known gaps](#known-gaps).

**v1:** the `try` and the `catch` of the v1 are the v1's:

```ahk
; v1: try and catch, with the braces on their own lines
try
{
  FileAppend, body, *
}
catch
{
  FileAppend, caught, *
}
```

```json
["body"]
```

**v1:** `catch as e` is not a form of the v1, and it is refused with
"Unexpected \"Catch\"". A catch names nothing and the caught value is not
carried into the body.

### throw

`throw` raises a value. The value is usually an error object, and it may be
any value.

```ahk
; an error carries a message, a What and an Extra
try
  throw Error("m", "w", "x")
catch Error as e
  FileAppend e.Message "|" e.What "|" e.Extra, "*"
```

```json
["m|w|x"]
```

### The error classes

The errors of v2 are a tree of classes, and every one of them is made the
way `Error` is: `ClassName(Message, What, Extra)`. These are the classes
this runtime carries, with the base each one derives from:

| Class | Base |
|---|---|
| `Error` | none: it is the root |
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

```ahk
; a class is derived from its base, and a catch of the base catches it
e := PropertyError()
f := IndexError()
FileAppend (e is MemberError) . " " . (f is ValueError) . " " . (e is Error), "*"
```

```json
["1 1 1"]
```

Note that `TimeoutError` derives from `Error` here and not from
`TargetError`. In the reference dialect `TimeoutError` is a kind of
`TargetError`, so a script that catches `TargetError` and expects to catch a
`TimeoutError` too will not catch it; catch `TimeoutError` or `Error`.

A message that is not given is the name of the class. The properties of an
error are `Message`, `What`, `Extra`, `File`, `Line` and `Stack`; the stack
is not carried by this runtime.

**v1:** there is no tree of error classes. A failure of the run is a failure
of the script, and many operations that fail in v2 are a blank value in v1
instead:

| v2 | v1 |
|---|---|
| an unset variable fails | an unset variable is blank |
| a division by zero fails | it is blank |
| an index that is not there fails | it is blank |
| a member that is not there fails | it is blank |
| a bad number in an operation fails | the operation is blank |
| a whole number is exact | the whole number is clamped |

A failure that the v1 does report stops the script, the way it does in v2.

### OnError

`OnError(Callback)` registers a function that handles a failure the script
did not catch. The callback takes the failure and the mode of the handler.

**v1:** `OnError` takes the **name** of the function, which is the form the
v1 scripts write -- `OnError("Handler")`. The object form works in both
dialects, and a v1 script that writes it as a command, `OnError, Handler`, is
a line that holds no action.

```ahk
; the handler of the v1 is named, and it is registered by name
OnError("Handler")
x := 1 / 0
FileAppend, still here, *

Handler(err, mode) {
    return 0
}
```

```json
["still here"]
```

### The report of a failure

The runner reports a failure the script did not catch in the `error` and
`error_line` of the report, and the process exits 1. A parse failure exits 2
and the report is a reduced payload.

```ahk
; a failure that nothing catches
x := [1][5]
```

```json
{"ok":false,"error":"Invalid index.","error_line":2}
```

## Directives

A directive is a name that begins with a hash at the start of a line, and
the rest of the line is its argument. A directive that the dialect does not
know is not a directive at all: the line holds no action and is refused.

```ahk
; two directives, and a statement
#Requires AutoHotkey v2.0
#Warn
FileAppend "ok", "*"
```

```json
["ok"]
```

```json
{"ok":false,"error":"This line does not contain a recognized action.\n     Specifically: #NotADirective","error_line":1}
```

**v2:** the nineteen directives are `#ClipboardTimeout`, `#DllLoad`,
`#ErrorStdOut`, `#HotIf`, `#HotIfTimeout`, `#Hotstring`, `#Include`,
`#IncludeAgain`, `#InputLevel`, `#MaxThreads`, `#MaxThreadsBuffer`,
`#MaxThreadsPerHotkey`, `#NoTrayIcon`, `#Requires`, `#SingleInstance`,
`#SuspendExempt`, `#UseHook`, `#Warn` and `#WinActivateForce`.

**v1:** the v1 carries the v2's directives and fifteen of its own:
`#CommentFlag`, `#EscapeChar`, `#HotkeyInterval`, `#HotkeyModifierTimeout`,
`#If`, `#IfTimeout`, `#IfWinActive`, `#InstallKeybdHook`,
`#InstallMouseHook`, `#KeyHistory`, `#MaxHotkeysPerInterval`, `#MaxMem`,
`#MenuMaskKey`, `#NoEnv` and `#Persistent`. The full list is in
`docs/ahk-v1-reference.md`, under "The directives".

**v1:** `#NoEnv` turns off the environment lookup of the `%name%` form.
**v1:** `#Persistent` keeps the script running to serve hotkeys.

**v2:** the `#HotIf` directive is carried and is refused with "the hotkeys
are not carried by this runtime." on this machine, because the hotkeys need
a Linux device.

**The `#Warn` directive** asks for warnings, and the warnings go into the
`warnings` of the report. The types and the modes are the dialect's:

| | v2 | v1 |
|---|---|---|
| Types | `VarUnset`, `LocalSameAsGlobal`, `Unreachable` | `UseUnsetLocal`, `UseUnsetGlobal`, `UseEnv`, `LocalSameAsGlobal`, `ClassOverwrite`, `Unreachable` |
| Modes | `MsgBox`, `StdOut`, `Off` | `MsgBox`, `StdOut`, `OutputDebug`, `Off` |
| Default | on | off |

## Hotkeys and hotstrings

**v2:** a hotkey is registered with a call, and its action is a function of
the script. A hotstring the same.

```ahk
; a hotkey and a hotstring of v2
Hotkey "^a", MyAction
Hotstring ":btw:", Expand

MyAction(ThisHotkey) {
  FileAppend "fired", "*"
}

Expand(ThisHotstring) {
  Send "by the way"
}
```

**v1:** a hotkey is a line of its own, and the action is a label or the
statement that the line carries. A hotstring is a statement:
`::btw::by the way`, or a body of lines that ends in `return`.

```ahk
; a hotkey of v1, with its label
^a::
  FileAppend, fired, *
return
```

```ahk
; v1: a hotstring is a statement of the script
::btw::by the way
```

```json
{"ok":false,"error":"the hotkeys of this box watch a Linux device; this is not Linux"}
```

**v1:** the section of the script that runs ends at the first hotkey or the
first hotstring. The v1 runs the code that comes before it, and each hotkey
is registered when the section reaches it.

**v2:** there is no such section. The script runs to the end, and a hotkey
registered with `Hotkey` lives as long as the script.

On a machine that is not Linux the hotkeys cannot be registered, and both
dialects say so:

```json
{"ok":false,"error":"the hotkeys are not carried by this runtime.","error_line":1}
```

```json
{"ok":false,"error":"the hotkeys of this box watch a Linux device; this is not Linux"}
```

The keys are written with the usual modifiers: `^` ctrl, `+` shift, `!` alt,
`#` super. `Send "^a"` sends the chord, `Send "+{Tab}"` sends shift and tab,
and `Send "{Raw}text"` sends the text as it is.

## Cheat sheets

### v2 to v1

| v2 | v1 |
|---|---|
| `x := 5` | `x = 5` (a text) or `x := 5` |
| `FileAppend "hi", "*"` | `FileAppend, hi, *` |
| `FileAppend %x%, "*"` | `FileAppend, %x%, *` |
| `Abs 5` | `Abs(5)` in an expression; a function is not a command |
| `StringLen, n` -- no such name | `StringLen, n, text` |
| `StrLen(text)` | `StrLen(text)`; there is no command |
| `if (x > 1)` with the body on the next line | the same |
| `if x = 1` compares the number | `if x = 1` compares the **text** |
| `Loop 3` | `Loop, 3` or `Loop 3` |
| `while` / `Loop` with a `break` | `While` / `Loop` with `Until` |
| `for v in a` walks the values | `for k in a` walks the keys |
| `a.Length` | `a.Length()` |
| `a.Push(v)` | `a.Insert(v)` |
| `a.RemoveAt(i)` | `a.Remove(i)` |
| `o["a"]` needs an `__Item` | `o["a"]` is the property `a` |
| `o.nothing` fails | `o.nothing` is blank |
| `Map("a", 1)` | an object, `o := {a: 1}` |
| `Point(1, 2)` | `new Point(1, 2)` |
| `super.Method()` | `base.Method()` |
| `static M() { ... }` | not allowed |
| `get => expr` | `get { return expr }` |
| `&x` | `ByRef` on the parameter, the name at the call |
| `catch as e` | `catch` |
| `try` / `catch` / `finally` | `try` / `catch` |
| `#HotIf` | `#If` |
| `Hotkey "^a", Fn` | `^a::` and a label |
| `Hotstring(":btw:", ...)` | `::btw::by the way` |
| `"a" 1` needs the space | `"a" 1` does not |
| `` `s `` is a space | `` `s `` is the letter `s` |
| `0x10` is 16 | `0x10` is the text `0x10` |

### v1 to v2

| v1 | v2 |
|---|---|
| `x = 5` | `x := 5` |
| `FileAppend, %x%, *` | `FileAppend x, "*"` |
| `FileAppend, % x + 1, *` | `FileAppend x + 1, "*"` |
| `StringLen, n, text` | `n := StrLen(text)` |
| `MouseGetPos, x, y` | `MouseGetPos &x, &y` |
| `SplitPath, p, n, d, e` | `SplitPath p, &n, &d, &e` |
| `Abs := 5` | refused: a function may not be an output variable |
| `goto Label` | `goto Label` (unchanged) |
| `Gosub Label` | not carried: a function, called by its name |
| `Loop, Read, file` | `Loop Read file` |
| `Loop, Parse, var, "`,"` | `for part in StrSplit(var, ",")` |
| `Until cond` | `if` and a `break` inside the loop |
| `a := []` then `a[5] := 7` | `a.Push(v)` for each place |
| `o := Object()` | `o := {}` |
| `o.b := 1` | `o.b := 1` (unchanged) |
| `new C()` | `C()` |
| `this.__Init()` | `__New` |
| `C.Say()` where `Say` is a method | refused: a class has no method of its own |
| `base.M()` | `super.M()` |
| `Clipboard` | `A_Clipboard` |
| `ErrorLevel` | a return value, or a `try`/`catch` |
| `IfInString, x, t` | `InStr(x, t)` |
| `SetFormat, Float, 0.2` | `Format("{:.2f}", x)` |
| `StringCaseSense, On` | `==` where the case matters, `=` where it does not |
| `Send, ^a` | `Send "^a"` |
| `^a::` and a label | `Hotkey "^a", Fn` |
| `::btw::text` | `Hotstring(":btw:", "text")` |
| `#NoEnv` | nothing: v2 never looks at the environment |
| `#Persistent` | nothing: v2 keeps a hotkey alive |

## Known gaps

These are places where the behaviour of this runtime differs from the
reference binary of the dialect it claims to speak, or where a name that the
language has is not carried. They are recorded here because they surprise a
script that was written for the reference dialect, and because a guide that
is quiet about them would be wrong.

- **`is` answers for a value that is not an object too.** The v2 binary
  answers 0 for `3 is Object` and 1 for `3 is Integer`, and this runtime does
  the same: the classes of the kinds of a value are `Integer`, `Float`,
  `String`, `Number` and `Primitive`, and the chain of them goes from the
  `Integer` and the `Float` to the `Number`, from the `Number` and the
  `String` to the `Primitive`, and from the `Primitive` to the `Any`. The
  class of an object is the class of it, and `b is Beta` answers 1.
- **`IsSet(x)` asks about the value, not the name.** The reference binary
  carries the name `IsSet` and this tree carries it too, but the two ask
  different questions: the v2 asks whether the **value** it is given is a
  variable that has one, and the v1 asks about the **name** -- `IsSet(x)`
  asks whether `x` has a value. `IsSetRef(&x)` asks the same question of
  the v2 with a reference, and answers 1 for a variable that has a value
  and 0 for one that has not.
- **`??=` is not an operator here.** `q ??= 5` is refused with a syntax
  error on the `??`. Write `q := q ?? 5`, or guard with `IsSetRef`.
- **A bare `catch e` is not accepted.** The v2 binary accepts `catch e` and
  binds the error to `e`. This runtime wants `catch as e` (or a class and
  `as e`), and refuses the bare form with "the class of a catch is not a
  class of this runtime: e". The same refusal comes from the v1, where the
  reference dialect does accept `catch e`.
- **`A_AhkVersion` answers `2.0.19` in both dialects.** A v1 script reads
  `A_AhkVersion` and gets `2.0.19`, the version of the v2 the tree speaks.
- **`TimeoutError` does not derive from `TargetError`.** The reference
  dialect makes `TimeoutError` a kind of `TargetError`. This runtime derives
  it from `Error` directly, so `catch TargetError` does not catch a
  `TimeoutError` made by `TimeoutError(...)`. The other classes of the tree
  derive as the dialect says; the table is in the section on errors.
- **`A_ScriptName` is `*`.** The script of the runner arrives on the
  standard input and has no file, so the name of the script is the name of
  the standard input.
- **`Gosub` is not carried.** A v1 line that begins with `Gosub` is refused
  as a line that holds no action, because the name is not a command of the
  v1 table here. A function called by its name does the job.
- **`break N` and `continue N` are not carried.** `continue 2` is refused
  with "Unexpected \"2\"".
- **`until` is a v1 word only.** A v2 `Loop` or `while` that carries an
  `until` clause is refused with "Unexpected \"Until\"", although the v2
  documentation has the clause.
- **`in` and `contains` are reserved and do nothing.** The lexer reserves
  them for a future of v2, and an expression that uses them is refused with
  "Unexpected reserved word.".
- **There is no case-sense setting in v2.** The v1 sets the case of the
  comparison with `StringCaseSense`. The v2 has no such name: the operator
  itself says which comparison it is, `=` without the case and `==` with it.
- **A text and a text do not compare with `<` or `>`.** The reference v2
  compares two texts. This runtime wants two numbers, and refuses with
  "Expected a Number but got a String.". `StrCompare` is the way to compare
  two texts.
- **`StrCompare` does not take the case option.** `StrCompare(a, b,
  "CaseSense")` is refused with "Parameter #3 of StrCompare is invalid.".
- **The hotkeys need a Linux desktop.** On any other machine a hotkey or a
  hotstring fails the script with the reason, in both dialects, so the
  examples of the hotkey section cannot be run here.
- **The `%name%` of a v1 expression reads nothing.** In a **command
  position** the `%name%` substitution works as documented, and the
  substitution or the forced expression is the form a v1 script uses in
  practice. Inside an expression of `:=` the same form is accepted and
  leaves an empty text, even when the variable has a value: `a = 5` then
  `y := %a%` leaves `y` blank. Write `y := a` instead.