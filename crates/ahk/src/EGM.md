# ahk/src

The source of the language crate: the public surface and the modules
behind it. The crate is portable and does no I/O of its own -- a script is
parsed here and executed against a `Desktop`, which is whatever implements
it. The real one lives on the lab box (the `ahk-box` crate); the tests use a
fake that records the calls.

## Overview

The files, in the order the data moves through them:

| File | What it holds |
| --- | --- |
| `lib.rs` | the public surface: the modules, and the re-exports of the symbols below |
| `lexer.rs` | the tokens: `tokenize`, `Token`, `TokenKind`, `SyntaxError` |
| `ast.rs` | the tree: `Node` and the operators that hang on it |
| `parser.rs` | the grammar: `parse`, `Parser` |
| `runtime.rs` | the interpreter: `Runtime`, `Report`, `Step`, `Clock` |
| `builtins.rs` | the built-in table: every name of v2 the dialect carries |
| `classes/` | the class types: `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey` |
| `desktop.rs` | the capability traits the runtime is handed, and the rule of the matching of the titles |
| `dll.rs` | the marshalling of a call into a library of the machine |
| `variables.rs` | the built-in variables of v2 |
| `keys.rs` | the key table: the evdev codes, the aliases and the key specs |
| `regex.rs` | the regular expressions of `RegExMatch` and `RegExReplace` |
| `date.rs` | the date and the time: the arithmetic, and the format of v2 |
| `files.rs` | the files of the machine, in the dialect of v2 |
| `settings.rs` | the settings of the script, and the state that keeps them |
| `hotkeys.rs` | the hotkeys of the script: the registry and the watcher |
| `surface.rs` | the state of every name of a dialect: the value it answers, or the reason it does not |
| `dialect.rs` | the dialects that the runtime speaks: the trait, the v2 and the v1 |
| `v1/` | the table of the v1: the names of the legacy dialect, the shape of their parameters and the handlers of the v1 |

The dependency between them runs one way: `parser.rs` reads `ast.rs` and
`lexer.rs`, and `runtime.rs` reads `ast.rs`, `builtins.rs`, `classes/`,
`variables.rs`, `regex.rs`, `date.rs`, `files.rs`, `settings.rs` and
`hotkeys.rs`. `dll.rs` depends on nothing inside the
crate: it is the marshalling of a native call, and it is pure. `ast.rs`, `lexer.rs`, `keys.rs` and `regex.rs`
depend on
nothing inside the crate. The key table is the odd one out: the runtime
hands the desktop the *name* of a key, and turning that name into a code is
the desktop's work.

## Interface

### The crate surface (`lib.rs`)

`lib.rs` is the whole public surface. It opens the modules and
re-exports the symbols that a caller is expected to use:

```rust
pub mod ast;
pub mod builtins;
pub mod classes;
pub mod date;
pub mod desktop;
pub mod dialect;
pub mod dll;
pub mod files;
pub mod hotkeys;
pub mod keys;
pub mod lexer;
pub mod parser;
pub mod regex;
pub mod runtime;
pub mod settings;
pub mod surface;
pub mod v1;
pub mod variables;

pub use ast::{AssignOp, BinaryOp, LiteralValue, Node, Parameter, UnaryOp};
pub use desktop::{Desktop, WindowAnswer};
pub use keys::{KeyStroke, UnknownKey, parse_spec, resolve_name};
pub use lexer::{SyntaxError, Token, TokenKind, tokenize};
pub use parser::{Parser, parse, parse_with, precedence};
pub use runtime::{Clock, Report, Runtime, RuntimeError, Step, Timeout, Value};
#[cfg(feature = "std")]
pub use runtime::SystemClock;
```

The crate is `#![no_std]` when the `std` feature is off, and it pulls in
`alloc` for the collections.

### The lexer (`lexer.rs`)

- `tokenize(source: &str) -> Result<Vec<Token>, SyntaxError>` -- the source
  into its tokens.
- `Token { kind: TokenKind, value: String, line: u32, column: u32 }` -- one
  token, with the place it came from.
- `TokenKind` -- `Name`, `Keyword`, `Number`, `String`, `Op`, `Newline`,
  `Eof`.
- The number of v2 is a decimal, a hex -- `0x` and the digits of the base of
  sixteen -- or a decimal with an exponent, like `1e3`. A number has one
  point, and the point of `1.5.2` is the member operator. A number may begin
  with a point: `.5` is the number 0.5, and the point that follows a value is
  the point of a member. A name that begins with a number is refused at the
  load, with the message of v2.
- `KEYWORDS: &[&str]` -- the words the language gives a meaning to. The
  word `until` is one of them: the binary of v2.0.19 reserves it, and a
  script may not use it as the name of a variable. The `elseif`, the
  `default` and the `extends` are not among them: the binary of v2.0.19
  takes them as names, and the dialect tells the label of the default case
  of a switch and the base of a class by what comes with them, and not by
  the word itself.
- `the_refusal_of_a_word(word: &str) -> Option<TheRefusal>` -- the reason of
  the refusal of a word. The reason belongs to the word and not to the place
  where the word is refused: the words that begin a statement are
  `Variable`, the words that are operators and the two that are the values
  of the dialect are `Syntax`, the words that are reserved for a future of
  v2 are `Future`, the `ExitApp` is `Func` and the `Class` is `Class`.
- The script may begin with the byte order mark, and the mark is not a
  character of it. Only the first one is taken. The mark is also a letter of
  a name, the way it is in the v2, and the name that begins with a number
  and a mark is refused with the name in it -- the mark is written as a
  question mark, because it cannot be written.
- The hash is a directive only at the start of a line. Anywhere else it is
  an illegal character in an expression, and the parser names the text of
  it. `THE_DIRECTIVES` carries the names of the v2, and a name that is not
  one of them is not a directive: the line holds no action, and the lexer
  refuses it at the load with the line as it was written.
- The semicolon is a comment when it is at the start of a line or when a
  space comes before it. The one that is glued to the token that comes
  before it is a character of the script, and the parser refuses it as an
  illegal character.
- The name of the v1 may not carry a quote. The lexer does not refuse it: it
  marks the name with the symbol that comes with the word -- the `a"b"x` is
  the symbol `a"b"x` -- and the parser refuses it where the name is a
  variable (`the_name_may_not_carry_a_quote`).
- The percent sign is an operator, and it builds a name: the name is the
  text that is written and the values of the expressions, joined without a
  space between them. `%x%` is the variable whose name is the value of `x`,
  and `MyArray%i%` is the variable `MyArray1`. The space between two
  pieces is what tells that the name is over.
- `SyntaxError { message, line, column }` -- the error the lexer and the
  parser raise.
- The continuation section of v2 is read here, and not by the parser: a line
  that begins with a `(` is a section, and the lines between the `(` and the
  `)` are merged with the line above them. The merge is a merge of the
  source: the tokens of the section carry the line of the line they merged
  with, and the newlines of the section are spaces in an expression and
  linefeeds in a text.

### The AST (`ast.rs`)

- `Node` -- one node of the tree. The variants are `Literal`, `Var`,
  `Unary`, `Binary`, `Ternary`, `Assign`, `Index`, `AssignIndex`,
  `Call`, `Block`, `If`, `Switch`, `While`, `Loop`, `For`, `Directive`, `Break`,
  `Continue`, `Return`, `ExitApp`, `FunctionDef`, `ClassDef`, `MethodCall`,
  `Property`, `AssignProperty`, `ArrayLiteral`, `ObjectLiteral`, `Lambda`,
  `Label`, `Goto`, `DynamicName` and `AssignDynamic`. Every variant carries
  its line.

  The statements that v2 spells as commands -- `Send`, `Click`, `Sleep`,
  `Run`, `MsgBox` -- are not variants of the tree: they are names of the
  built-in table, and the runtime calls the table.

  `AssignIndex` is the write of an item -- `obj[key] := value` -- and it is
  a node of its own, not an assignment to a property. v2.0.19 writes the
  items of a `Map` and of an `Array`, and it does not write the item of a
  class of the script: the meta-functions answer for the properties that
  are not there, and `x[y]` asks only the `__Item`, which reads and never
  writes.
- `ArrayLiteral { items, line }` -- the literal of an array: `[a, b]` is
  `Array(a, b)`. It is a primary, so the suffixes of a value follow it
  the way they follow a variable: `[1, 2].Length` is the length of the
  array. The item of the list may be absent: the hole of `[1, , 3]` is a
  place of the array that has no value of its own, and the item of it is
  `None`.
- `Group { items, line }` -- the list of the values between parentheses:
  the value of it is the last of them, and every item runs. The
  parenthesis that is spaced is not a group.
- `Empty { line }` -- the parameter that a command does not give a value
  to: it keeps its place, and it has no value. A parameter that has a
  default takes it, and one that has none is a missing parameter.
- `Legacy { items, line }` -- the list of v1 that may follow the
  expression of a statement: the comma ends the expression, and the rest
  of the line is this list. It is evaluated, and its value is thrown away.
- `ObjectLiteral { properties, line }` -- the literal of an object:
  `{a: 1}` is an object of the base class with the property `a`. The name of
  a property is a name of the script, and the names of the properties do not
  know their case. The name of a property may also be a whole number: `{5: 1}`
  is an object with the property `5`, and a number with a point is not a
  name. The name of a property may be built by the script as well:
  `{%name%: 1}` is the property whose name is the value of `name`. A comma
  at the end of the list of an array, of an object
  and of the arguments of a call is allowed.
- `Lambda { parameters, body, line }` -- the literal of a function:
  `(a, b) => a + b`. The body is an expression and not a block of
  statements, and the function closes over the scope where it was written.
- `For { variables, source, body, line }` -- the loop that walks what an
  object enumerates. A name the script left out is an empty string in
  the list, so `for , value in map` is two names and the first one is
  nobody's.
- `Label { name, line }` -- a label of the script: a name and a colon. It
  names a line, and it does nothing by itself.
- `DynamicName { pieces, line }` -- the name that the script builds:
  `%x%`, `MyArray%i%`. The pieces are the text that is written
  (`DynamicPiece::Text`) and the values of the expressions
  (`DynamicPiece::Value`), and they are joined without a space between them.
  `AssignDynamic` is the write of it.
- `MemberName` -- the name of a member: the text that is written
  (`MemberName::Text`) or the pieces of a name that the script builds
  (`MemberName::Dynamic`).
- `Goto { name, line }` -- the jump to a label. The name is the text of
  the parameter as the script wrote it, and not its value: the quotes of a
  text are part of it.
- `Node::line(&self) -> u32` -- the line of the node, which is the line the
  report blames.
- `LiteralValue` -- `Integer(i64)`, `Float(f64)`, `Text(String)`,
  `Bool(bool)`. The shape of the literal is the shape of the value: the
  whole number of v2 is a whole number, and a number with a point or
  with an exponent is a float. The boolean keeps its own kind so that the
  parser can tell `true` from `1`; v2 has no boolean type, and the
  runtime folds it into the integer.
- `UnaryOp` -- `Not`, `NotWord`, `Negate`, `BitwiseNot`, `BitwiseAnd`,
  `BitwiseOr`, `BitwiseXor`, `Plus`, `Address`, `PreIncrement`,
  `PostIncrement`, `PreDecrement` and `PostDecrement`. `Address` is the
  `&` of the reference operator.
- `BinaryOp` -- the forty operators of v2: `Add`, `Subtract`, `Multiply`,
  `Divide`, `IntegerDivide`, `Power`, `BitwiseAnd`, `BitwiseOr`,
  `BitwiseXor`, `ShiftLeft`, `ShiftRight`, `ShiftRightUnsigned`, `Concat`,
  `Equal`, `NotEqual`, `StrictEqual`, `StrictNotEqual`, `Less`, `Greater`,
  `LessEqual`, `GreaterEqual`, `RegexMatch`, `And`, `Or`, `Is` and
  `Coalesce`, and the five that the loop kinds use: `Count`, `Files`,
  `Reg`, `Read` and `Parse`.
- `AssignOp` -- `Set` (`=` and `:=`), `Add`, `Subtract`, `Multiply`,
  `Divide`, `IntegerDivide`, `Concat`, `BitwiseAnd`, `BitwiseOr`,
  `BitwiseXor`, `ShiftLeft`, `ShiftRight` and `ShiftRightUnsigned`.
- `LoopKind` -- `Count`, `Files`, `Reg`, `Read` and `Parse`: the kinds of
  `Loop`.
- `PropertyDef` -- the definition of a property: its name, the value it
  starts with, whether it belongs to the class, and its line.
- `Parameter { name, default, variadic, by_ref }` -- one parameter of a
  function or of a method: its name, the value it takes when the call does
  not give it one, whether it takes the rest of the arguments, and whether
  it is written by reference -- `Name(a, &b)`. The parameter that takes the
  rest of the arguments may have no name at all -- `f(*)`.

The precedence of the operators is the table of `ahk::precedence`, which
follows the documentation of v2. The kinds of the tokens of the binary
are not a precedence: they are what the lexer reads.

Two values side by side are one value: that is the implicit
concatenation of v2, and it binds like the dot. The parser makes it a
`Concat` as well, so the two ways of writing it are the same node.
- `Branch { condition: Node, tail: Vec<Node>, body: Vec<Node> }` -- one branch
  of an `If`. The `tail` holds the items that come after the comma of the
  line of the condition of the v1, and they are evaluated whether the
  condition is true or false.
- `Case { values: Vec<Node>, body: Vec<Node> }` -- one `Case` of a
  `Switch`; an empty list of values is the `Default`. A case may name many
  values, and it matches when the subject of the switch is one of them.

### The parser (`parser.rs`)

- `parse(source: &str) -> Result<Vec<Node>, SyntaxError>` -- the whole
  script, parsed.
- `Parser::new(source: &str, tokens: Vec<Token>) -> Parser` -- a parser over
  tokens the caller already has, and the source they came from. The source is
  what keeps the spaces of the script: the text of the legacy dialect is the
  text of the script.
- `Parser::parse_script(&mut self) -> Result<Vec<Node>, SyntaxError>` -- the
  statements of the script.

A name at the start of a line, without parentheses, is a call statement:
the rest of the line is the list of the parameters. Every function may be
written that way -- a built-in and a function of the script alike -- and
the answer of the call is thrown away. A name that is neither is not
refused at the load: it is a variable of the script, and the value of it
is what the call asks for. A value that cannot be called is refused with
the kind of it, the way v2 refuses it.

The argument of a command is a value of the dialect, and not a word of the
language. The brace that opens one opens the literal of an object, and not a
key: `Send {Enter}` is refused at the load with "Missing \"propertyname:\" in
object literal.", and the key of v2 is a text -- `Send "{Enter}"`. The button
of a click is a value too: `Click 10, 20, right` reads the variable `right`.

The answer of a call is a value, and the members of it are read the way
the members of any other value are: `StrLen("abc").__Class` is "Integer",
and `Map().Count` is 0. A statement that reads the member of a call leaves
the value behind, and the member of it may be written: `Map()["a"] := 1`
writes the entry of a map that nothing keeps. The call itself is not a value
that may be written, and the refusal of it is at the load, and the text of it
is named: "Invalid assignment.\n     Specifically: := 1".

The words that name something are separated by a newline and five spaces, and
the text that is named is the text of the source. Measured on the binary of
v2.0.19 on 2026-10-31: an operator that may not begin a value is a "Syntax
error." and the rest of the line, and the operator that wanted the operand
names the failure -- a "Missing operand." and the operator and the rest of
the line. The text is not there when the operator is the last item of the
arguments of a call or of the items of an array. The dot that does not reach
into a value is a "Syntax error." and the rest of the line, from the dot.

A statement that begins with an operator is a line that holds no action, and
the text that is named is the line itself: "This line does not contain a
recognized action.". The operators that want a word of their own are named
apart -- the equal wants the operator of an assignment, the assignment wants
a name on its left, the comma separates the parameters of a call, the
question asks a question that is not there, and the double colon is the
separator of a hotkey. The dot is the member of a value that is not there,
and the v2 names the call that it wanted to make: `.x` is named `.x()`.

The body of a block begins on the next line, and the brace that closes it
is at the start of its line. The word that `parse_block` takes is the
construct the block belongs to, and it says in its own words that the
brace is not where it belongs.

The body of a definition may be an expression, and the fat arrow is what
says so: `f(a) => a + 1` is a function that answers the expression of it.
The body of the definition is a `Return` of that expression, and the arrow
may be on the line of the parameters or on the one after it. A definition
that has no body is refused at the load: "Invalid function declaration." and
the text of the definition.

The `break` and the `continue` need a loop around them, and the parser
counts the loops that it is inside: the body of a loop is read with
`the_body_of_a_loop`, and the body of a function with `the_body_of_a_scope`,
which starts the count again. A `break` or a `continue` that has no loop
around it is refused at the load: "Break/Continue must be enclosed by a
Loop."

The body of a block may also be one statement, and the braces may be
absent: `if (1)` and then `x := 1` on the next line is an `if` with a body.
The body is one statement, and the `if`, the `while`, the `loop`, the `for`
and the `catch` want it on the next line, while the `try`, the `else` and
the `finally` take it on the line of the construct as well.

The v2 reads the body of a construct before the condition that it carries,
and the refusal of a condition that is empty comes after the refusal of
the body. The subject of a switch and the values of a case are read the
same way.

Every construct needs a body, and the one that has none says so in its own
words: "Unexpected \"Catch\"", "Unexpected \"Else\"" and "Unexpected
\"Finally\"". The `else` of a `try` needs a `catch` before it, and it
names itself when there is none.

The content of a block may begin on the line of the brace that opens it,
and the construct refuses it when the brace is not at the start of its
line. The `try`, the `else` and the `finally` take it there always. The
block that is on the line of the construct says that the brace is missing,
and the block that is empty and that is on one line is not a block at all:
the brace that closes it is a statement that holds no action.

The block that is not closed names the construct that opened it when it is
empty, and it names nothing when it has content.

The parenthesis of a `goto` that is spaced is a part of the text of the
name of the label: `Goto (done)` looks for the label "(done)", and the
parenthesis that is not spaced is the parenthesis of the dialect.

The loop of v2 is one statement with five shapes, and the word that follows
the `Loop` says which one it is: a name of the kind -- `Files`, `Reg`,
`Read` or `Parse` -- when the line goes on after it, and the count of the
loop when the line ends there. The name of the kind is part of the
statement, and the command checks its parameters when the script loads:
the parameter that is not there is named, and so is the head of the
statement. The count of the loop is not a name of a kind: a comma after it
says that the type of the loop is not a valid one.

The bracket with a space opens the array and the bracket without one is
the item of the value: `f [1, 2]` is a call whose argument is the array,
and `f[1, 2]` is the item of the value `f`. The space is the reason, the
same way it is for the dot. In an expression the space does not matter:
`arr [2]` is the second item of `arr`.

The parameters of a function and of a method are read apart from the
arguments of a call: the brace that follows the list is what tells a
definition from a call. A parameter is a name, it may carry a default --
`Name(a, b := 1)` -- it may be the one that takes the rest of the
arguments -- `Name(a, rest*)` -- and it may be written by reference --
`Name(a, &b)`. The marker is in the definition, and the caller passes the
reference: `Name(&x)`. A parameter written that way is an alias of the
variable of the caller.

The arrow function of v2 is `(a, b) => a + b`: the body is an expression and
not a block of statements, and the parenthesis may be left out when there is
one parameter -- `a => a + 1`. The token that follows the closing parenthesis
is what tells a list of parameters from the parenthesis of an expression.

The assignment of v2 is an expression: `(x := 1)` is a value, and the value of
it is the value that was written. The equal sign of v1 is not an assignment:
`x = 1` does not load, and the parser says "Syntax error. Did you mean to use
\":=\"?".

The refusals of the parser are the words of v2: a name that is not there is
named, and the line of the script says where it was -- "Missing parameter
name.\n     Specifically: f(a, b,) {". A bracket that is not closed is named
the same way, and a token that is not wanted is named too: "Unexpected
\")\"\n     Specifically: x := 1 )".

The text that a refusal names is the text of the source, and the newline of
the line is not a part of it: the block that the error printer of v2 writes
ends with a newline of its own, and the message is the block without it.
The rest of the line is named when a value came too close -- "Missing space
or operator before this.\n     Specifically: \"b\"\"c\"" -- and the
statement is named when a bracket is not there -- "Missing \")\"\n
     Specifically: FileAppend \"[\" ( abc". The parenthesis that closes a
command-style call is carried by the text of the failure of the expression:
"Missing space or operator before this.\n     Specifically: 2)".

A line that is a label is a label before it is anything else: a name and a
colon, and nothing else may be written on the line of it. The name is of
letters, of digits and of the underscore, and it may begin with a digit.
A line that ends with a colon and is not a label holds no action: "This
line does not contain a recognized action.\n     Specifically: a-b:".

The parameter of a `Goto` is not an expression: it is the name of a label
and it is read as the text of it. The `Goto` needs one parameter and it
takes one, and the load refuses the one that has none and the one that has
more, in the words of the dialect. A label that is written twice in the
same scope is refused at the load too: "Duplicate label.\n     Specifically:
done".

The load refuses the goto whose label is not in the scope, and it names
the label: "Label not found in current scope.\n     Specifically:
nowhere". The scope of a label is the scope it is written in, and the name
of a label does not know its case. The check is at the end of the scope,
because the goto may name a label that comes after it.

An operator that wants a number may not follow a literal text, and the
refusal is at the load: "Unexpected operator following literal string.\n
Specifically: + 0". The text that is named is the operator and the rest of
the expression that holds it -- the one of the statement -- and the parser
waits for that expression to name it. A group is not a literal text, and
the concatenation of two literal texts is one.

The text of v2 is between double quotes or between single quotes, and the
escape of the dialect is the backtick in the two of them. The token keeps
the quote that was written, because the source of the script is rebuilt
from the tokens and the rebuild says the truth: `Goto 'done'` looks for a
label that is called `'done'`, with the quotes. A text does not cross a
line, and a text that is not closed before the end of it is refused with
the quote that is not closed and the line that holds it.

Two values side by side are one value, and the space between them is what
makes it so. Without the space, v2 refuses and names the value that came
too close: "Missing space or operator before this.\n     Specifically:
"b"".

The reserved word that is not wanted says what it is. Measured on the binary
of v2.0.19 on 2026-10-31: the words that may not be the name of a variable
say "The following reserved word must not be used as a variable name:" and the
word; the words that are reserved for a future of v2 -- `in` and `contains` --
say "Unexpected reserved word."; and the words that are not operators -- `not`
and `super` -- say "Syntax error.". The text that is named is the rest of the
line, from the word. The words that begin a construct -- `elseif`, `default`,
`exitapp`, `class` and `extends` -- are refused by v2 with a dialog, and the
message of the dialog cannot be read.

v2 has no cast. The word `as` is a reserved word, and `x := 1 as Integer` is
refused with "Unexpected reserved word." and the rest of the line. The tree
read it as a cast of its own, and a cast that the dialect does not have.

A parenthesis holds a list of the values, and the value of the list is the
last of them. Measured on the binary of v2.0.19 on 2026-10-31: `x := ("a",
"b")` is "b", and every item of the list runs. The list of nothing is a
"Syntax error.", and the parenthesis that closes it is named; a comma at
the end of the list is a "Syntax error." too.

The parenthesis that is spaced is not the list of the arguments of a call.
A name at the start of a line that is followed by a space and a parenthesis
is a command of v2, and the parenthesis is one parameter of it: `MyFunc
("a"), "b"` calls `MyFunc` with two parameters, and `MyFunc ("a", "b")`
calls it with one, the "b". A member at the start of a line is the same
command, and the member is the callee. In an expression the parenthesis
that is spaced is the value that follows the name or the member, and the
concatenation of v2 takes the two: `x := MyFunc ("a")` says "Expected a
String but got a Func." at the run. The commas at the end of the parameters
of a command add nothing: `One "a",` calls `One` with one parameter.

The parameter that a command does not give a value to keeps its place, and
it has no value. Measured on the binary of v2.0.19 on 2026-10-31: `One
"a",, "b"` calls `One` with three parameters, a parameter that has a
default takes it, and one that has none is a missing parameter -- the call
is refused with "Missing a required parameter." and the name of the
parameter goes in the `Extra`. The number of the arguments is checked
before the parameters are bound, so `Two "a",, "b"` is refused with "Too
many parameters passed to function.". The value that is not there in the
list of a group is a "Syntax error.", and the comma that follows it is
named. The value that is not there in the list of the arguments of a call
keeps its place, and the one that is the last is dropped: the commas at
the end of the list add nothing.

The statement of v2 is an expression, and the comma is what ends it.
Measured on the binary of v2.0.19 on 2026-10-31: the expression of a
statement goes on -- the operators and the concatenation of v2 are a part
of it, and the value of it is thrown away -- so `f() + 1` is the sum and
`f() "a"` is the concatenation, and it wants a String ("Expected a String
but got a Func."). The comma ends the expression, and the rest of the
line is the list of v1: it is evaluated, and its value is thrown away, so
`f(), g()` runs the two and `x := 1, 2` is "1". The name of a command of
v2 is followed by a space or a parenthesis, and the comma that is there
instead of the space is refused with "Function calls require a space or
\"(\". Use comma only between parameters." and the line is named with it;
the space before the comma makes it a command, and `x , 2` is one. The
number that is at the start of a statement is the name of a command, and
`1, 2` is refused with the same message; the number that carries a point
is not a name, and the line holds no action. The member that has
parentheses, and the item that is read, are expressions too, and the
comma ends them; the write of a member is a statement like any other, and
`x := 1, 2` writes the property and evaluates the list.

The member that has no parentheses is a command of v2, and it carries its
arguments. Measured on the binary of v2.0.19 on 2026-10-31: the rest of
the line is the list of the arguments of the method, the space is the
concatenation of v2 and the comma is what separates them (`t.Method 7 8`
is one argument and `t.Method 7, 8` is two), and the comma that comes
right after the member with no space is not one of them -- the line holds
no action. The parameter that the command does not give a value to keeps
its place, and the method is refused with "Missing a required parameter.".
The line that ends with a comma continues on the next one, and the list of
v1 goes on with it.

### The runtime (`runtime.rs`)

The interpreter and everything it reports:

| Item | What it is |
| --- | --- |
| `Value` | a value a script can hold: `Int(i64)`, `Float(f64)`, `Text(String)`, `List(Vec<Value>)`, `Object(Rc<RefCell<Object>>)`, `Reference(Reference)` -- and no boolean, because v2 has no boolean type |
| `Scope` | where a variable lives: `Global`, or `Local(depth)`, the place of the scope in the stack |
| `Reference { name, scope, alias }` | a reference to a variable: its name, its scope, and whether it is an alias of the variable |
| `TypeName` | the kind of a value: `String`, `Integer`, `Float`, `Object`, `VarRef` -- the names v2 uses in its diagnostics. `Number` and `Class` are there too: no value carries them, and they are what a parameter wants |
| `Value::truthy`, `Value::to_text`, `Value::to_number` | the three readings of a value |
| `Value::from_number(f64) -> Value` | a number as a whole number when it is one |
| `Runtime<'a>` | the run: it holds the desktop, the clock, the variables and the steps |
| `Runtime::new(desktop, timeout)` | the constructor for a hosted platform, behind the `std` feature |
| `Runtime::with_clock(desktop, clock, timeout)` | the constructor for a target with no `std` |
| `Runtime::run(self, statements: &[Node]) -> Report` | the run, and the report of it |
| `Runtime::note(&mut self, note: impl Into<String>)` | a note for the report |
| `Runtime::MAX_STEPS` | 20 000: the most statements one script may execute |
| `Runtime::MAX_ITERATIONS` | 1 000: the most times a loop body may run |
| `Runtime::MAX_DEPTH` | 1 000: the most nested calls a script may make |
| `Runtime::MAX_STACK` | 512 KB: how much stack a run may use, when the host does not say |
| `Runtime::with_stack_budget(bytes)` | the host says how much stack the run may use |
| `Report` | the whole run: `ok`, `steps`, `output`, `notes`, `error`, `error_line`, `exit_code`, `elapsed_ms`, `variables` |
| `Step` | one statement: `line`, `op`, `detail`, `ok`, `elapsed_ms` |
| `RuntimeError { message, line }` | a statement failed |
| `Timeout { seconds }` | the script ran for longer than the caller allowed |
| `WindowAnswer` | `Active`, `NotActive`, `Unknown(String)` -- and the third case is the point |
| `op_name(&Node) -> &'static str` | the name of a statement, for the report |

The built-ins that have a sense of the case want a boolean: a zero or a one.
A number that is another one, a float and a text are refused, and the name
of the built-in says which parameter it is.

A class of the script is initialised when it is first used: the statement
that defines it and the first reference to it. The variables that belong to
the class are assigned to the class then, and the `__New` that is static is
the one that runs. The variables of an instance are assigned when the
instance is made.

A `Goto` is a jump of the run: the run goes on at the label, and it goes out
of the block when the block does not hold the label. The scope of a label is
the scope it is written in -- the scope of the script, or the scope of a
function or of a method -- and the jump of one scope is not the jump of
another: "Label not found in current scope.\n     Specifically: later" is the
failure of a label that is elsewhere, and "A Goto must not jump into a
block that doesn't enclose it." is the failure of a label of a block that
does not enclose the jump. The failure is of its own kind -- `RuntimeFailure
::Fatal` -- and a catch does not take it.

The scope of the runtime is shared: a function that is written inside another
one closes over the scope of the one around it, and the write of the closure
is seen by the scope it closes over. A name that the scope around the call
already has is written there -- v2 assumes it is local unless the scope around
it has the name -- and the name of a function that is written inside another
one is local to the function that has it. The functions that are written
inside a function are collected when the function around them is called, and
not when the statement that defines them runs: a function that is called
before it is defined finds it. The name of a closure of v2 is empty, and the
runtime keeps every one of them under a name of its own, that a script cannot
reach.

The call of a method binds its parameters the way the call of a function
binds its own: the parameter that the command does not give a value to keeps
its place, and it has no value -- a parameter that has a default takes it,
and one that has none is a missing parameter, and the call is refused with
"Missing a required parameter." and the name of the parameter goes in the
`Extra`.

The number of the arguments of a call is checked before the arguments are
evaluated: the refusal of the arity comes before the failure of an
argument, the way v2 has it -- v2 refuses the script when it loads, and the
tree refuses it when it runs, with the same words. A method is the same
way, and the failure of the arity of a method carries the name of the
method in the `Extra`.

The traits the runtime is handed:

| Trait | Methods |
| --- | --- |
| `Desktop` | the sum of the fifteen capability traits below: it has no method of its own, and there is no blanket implementation |
| `Keyboard` | `type_text`, `press_key`; `key_down`, `key_up`, `key_is_down` and `keys_down` have a default that answers with the reason it cannot |
| `Pointer` | `click`; `mouse_down`, `mouse_up`, `move_pointer`, `mouse_position`, `set_mouse_speed` and `set_mouse_delay` have a default that answers with the reason it cannot |
| `Screen` | `capture`, `pixel`; `pixel_search` and `screen_size` have a default that answers with the reason it cannot |
| `Windows` | `window_active`, `window_exists` -- the mode of the matching of the titles goes with the question; `active_window_title` has a default that answers with the reason it cannot |
| `Processes` | `run`; `env_get`, `env_set` have a default that answers with the reason it cannot |
| `Files` | `open_file`, `read_file_at`, `write_file_at`, `file_size`, `close_file`; `list_files`, `working_directory`, `initial_working_directory`, `set_working_directory`, `read_file`, `write_file`, `append_file`, `file_exists`, `delete_file`, `create_dir`, `dir_exists` and `dir_delete` have a default that answers with the reason it cannot |
| `Files` (the files of v2) | `copy_file`, `move_file`, `copy_dir`, `move_dir`, `file_attributes`, `set_file_attributes`, `file_times`, `set_file_times`, `ini_read`, `ini_write` and `ini_delete` have a default that answers with the reason it cannot |
| `Clipboard` | `clipboard_read`, `clipboard_write` have a default that answers with the reason it cannot |
| `Timers` | `takes_over_timers` has a default that answers with the reason it cannot |
| `Hotkeys` | `register_hotkey`, `unregister_hotkey`, `hotkeys`, `fired_hotkey` have a default that answers with the reason it cannot |
| `Sound` | `sound_volume`, `sound_set_volume`, `sound_muted`, `sound_set_muted`, `sound_play` and `sound_beep` have a default that answers with the reason it cannot |
| `Machine` | `user_name`, `computer_name`, `shell`, `folders`, `os_version`, `language`, `is_admin` and `idle_times` have a default that answers with the reason it cannot |
| `Dialogs` | `input_box`, `file_select` and `dir_select` have a default that answers with the reason it cannot: the question of the user is the machine's |
| `Network` | `download` has a default that answers with the reason it cannot: the network is the machine's |
| `Pictures` | `load_picture` and `image_search` have a default that answers with the reason it cannot: the images of the machine are the machine's |
| `Clock` | `now_ms() -> u64` (the milliseconds since an arbitrary origin, never backwards), `sleep_ms(u64)` (waits at least that long) and `wall_clock() -> Option<WallClock>` (the date of the machine, and nothing when the host does not know it) |

Every implementor writes the fifteen of them: there is no blanket
implementation, so a desktop that grows a capability grows with it.

`SystemClock` is the clock of a hosted platform -- the monotonic clock and the
system sleep. It exists only with the `std` feature.

The built-ins the language carries are the ones of v2.0.19, and the table
of them is in `builtins.rs`: 243 commands and 103 functions. The names are
v2's names -- `PixelGetColor`, `MouseGetPos`, `WinGetTitle`, `WinActive`,
`WinExist`, `Abs`, `Sqrt`, `Mod`, `Floor`, `Ceil`, `Round`, `Min`, `Max`,
`StrLen`, `SubStr`, `StrReplace`, `InStr`, `Trim`, `StrUpper`, `StrLower`,
`Format`, `MsgBox`, `Send` -- and there are no aliases. The one name that is
not a name of v2 is `Screenshot`: the dialect writes a picture of the screen
to a file, and v2 has no word for it.

A name that is shaped for Windows is in the table too, and the handler
answers it with the reason: the tree carries the name, and a script that
asks for one is told the truth about it.

The arity in a row is the arity of the binary, so a call that asks for
more than the tree can do is taken by the table and refused by the
handler, with the reason. A parameter is never ignored: a call that
answers with a wrong value and says nothing is the worst outcome.

A reference is a value of its own kind, and the kind is "VarRef". Where
the parameter of a built-in takes any value, the reference goes through
and the built-in answers about it: the `Type` of a reference is "VarRef",
and a reference is an object, like an array, a map and a class. Where
the parameter wants a text, a number or an object, the reference does
not fit and the built-in says so. Measured on the binary of v2.0.19 on
2026-10-31.

The operator `is` asks whether a value is of a class, and the value that is
not a class is refused with the value in the `Extra`: `1 is 2` says "Expected
a Class but got an Integer." and the "2" is the `Extra`, and the `What` is
empty. Measured on the binary of v2.0.19 on 2026-10-31.

The value of a parenthesized list is the last of its items, and every item
of it runs. Measured on the binary of v2.0.19 on 2026-10-31: `x := ("a",
"b")` is "b", and the list of three items runs all of them.

The parameter that a command does not give a value to has no value, and
the call that takes it knows it: a parameter that has a default takes it,
and one that has none is a missing parameter. Measured on the binary of
v2.0.19 on 2026-10-31: `One "a",, "b"` calls `One` with three parameters,
and the second one has no value at all.

The built-in variables of v2 are in `variables.rs`: the ones the runtime
knows, and the ones the runtime writes where it runs.

#### The error handling

The runtime has two kinds of failure: `RuntimeFailure::Error` is an error
of the runtime, and `RuntimeFailure::Thrown` is a value that the script
threw. A catch names the class it takes -- `error` (the default) takes the
errors of the runtime, `any` takes everything, and a class of the script
takes the objects of that class -- and the value it binds is an object of
the class `error` when it is an error of the runtime.

#### The declarations of v2

`global` and `local` are declarations, and the tree carries them.

| Item | What it is |
| --- | --- |
| `Node::Declare { scope, names, line }` | a declaration: the scope it gives to the names, and the names with the value they start with |
| `DeclareKind` | `Global` or `Local` |
| `Declared` | a name of a declaration, the value it starts with, and the text of the declaration |
| `Declarations` | the declarations of a function or of a method: the names that live in the scope of the script, the names that live in the scope of the function, and whether the function assumes the scope of the script |
| `Runtime::lives_in_the_script` | where a name lives: the frame of the function that runs says |

The names of a declaration are read when the script is read, so a
declaration inside a block takes effect for the whole function; only the
value waits for the statement that carries it. The parser reads them
(`the_declarations_of_a_body`) and refuses what v2 refuses.

#### The handlers of the runtime

`OnExit` and `OnError` register a function of the script, and the runtime
calls it the way v2.0.19 calls it.

| Item | What it is |
| --- | --- |
| `Callback { name, line }` | a function of the script that the runtime calls: the name the script wrote, and the line of the statement that registered it |
| `Runtime::the_callback(value, name, line)` | the callback of a value, and the three refusals of v2: a value that is not an object, a value that cannot be called, and a prototype that does not take the two parameters |
| `Runtime::set_exit_callback`, `set_error_callback` | the registration; v2 keeps one entry for a callback |
| `Runtime::remove_exit_callback`, `remove_error_callback` | the removal: the order 0 does not call the callback |
| `Runtime::order_exit_callback`, `order_error_callback` | the order: the callback of the order -1 runs before the ones that are registered |
| `Runtime::ask_the_error_callbacks(failure)` | the functions of the error, and what they said: `TheError::Goes`, `TheError::Hidden` and `TheError::Continues` |

The callback of the exit takes the reason and the code, and the callback of
the error takes the value that was thrown and the mode. The functions of the
error are called where the failure happened, before the stack unwinds, and
not while the body of a `try` runs.

### The key table (`keys.rs`)

- `EVDEV: &[(&str, u16)]` -- the key names and their evdev codes.
- `KEY_ALIASES: &[(&str, &str)]` -- the names that are spelled more than one
  way.
- `MODIFIERS: &[(&str, &str)]` -- the modifiers and the key that carries
  them.
- `resolve_name(name: &str) -> Result<u16, UnknownKey>` -- the code of a key
  name.
- `character_stroke(character: char) -> Option<(u16, bool)>` -- the key a
  character is typed with, and whether it needs the shift.
- `parse_spec(spec: &str) -> Result<Vec<KeyStroke>, UnknownKey>` -- a key
  specification into the strokes it means: `Return`, `ctrl+c`,
  `ctrl+shift+t`, and `ctrl+c ctrl+c` for two strokes.
- `KeyStroke { code: u16, modifiers: Vec<String> }` -- one press, and
  `KeyStroke::describe()` for the report that says what was pressed.
- `UnknownKey(pub String)` -- the key the table does not have.

### The built-in table (`builtins.rs`)

- `Builtin { name, kind, min, max, variant, parameters, handler }` -- one name of the
  table: the name as v2 spells it, which of the two tables of the binary
  it came from, the fewest and the most parameters, the byte that picks
  the flavour of a shared handler, the shape of each parameter, and the handler. The arity of the
  commands was measured on the binary of v2.0.19 on 2026-10-31: the call
  that asks for too few is refused at the load, and the call that asks for
  too many is refused as well, so the two refusals say where the bounds
  are.
- `BuiltinKind` -- `Command` or `Function`: the two tables of the binary.
  The parser does not use them. v2 has no kind of command: the
  parentheses of a call may be left out when the answer is not needed
  and the name is at the start of the line, and every function may be
  written that way.
- `BUILTINS` -- the table itself: 243 commands and 103 functions.
- `find(name) -> Option<&'static Builtin>` -- the built-in of that name,
  whatever the case of the name is.
- `find_command(name)` -- the same, for a command.
- `names()` -- the names of the table, as the language spells them.
- `reference_parameters(name) -> &'static [usize]` -- the places of the
  parameters that take a reference. The flag of the table of v2 is the
  place of the first parameter that is written back, and this is that
  column turned into the places of the parameters.
- `required_parameters(name) -> &'static [u8]` -- the places of the
  parameters that a command may not leave out, though its table says it
  may. The nine commands that have one were measured on the binary of
  v2.0.19 on 2026-10-31, and the refusal of such a command is the one of
  the parameter that is missing: "Parameter #1 of FileGetAttrib must not
  be omitted in this case."
- `Wants` -- what a parameter wants, and the shape of the refusal of the
  dialect: `Declared(TypeName)` for a parameter that declares its kind,
  `Checked(TypeName)` for a check that the body of the function makes,
  `Reference(&str)` for a parameter that wants a reference, and `Value`
  for a built-in that takes a value of any kind.
- `wants(name, index) -> Wants` -- what that parameter wants. The tables
  of it -- `THE_TEXTS`, `THE_NUMBERS` and `THE_REFUSED` -- were measured
  on the binary of v2.0.19 on 2026-10-31: every built-in of the tree
  was called with an object where the parameter goes, and the refusal of
  the binary named the parameter and the kind it wanted. Every name that
  the tree does not carry was asked the same way, and the kinds of their
  442 parameters are in `THE_REFUSED`. The runtime asks the table before
  the handler runs, and a value that does not fit is refused with the
  reason: the kind of the parameter comes before the reason of the
  machine, and it comes before the parameter that may not be left out.
- `refuses_itself(name) -> bool` -- the built-ins that refuse the
  parameter in their own handler: the `Mod` says "Invalid parameter(s)."
  for anything that is not a number, and it does not name the type.
- `refusal_of(name) -> Option<&'static str>` -- the reason the name refuses
  with, when it refuses.

A name that the tree carries and cannot answer has the refusal as its
handler, and the variant of the row is the reason: the reason is written
once, and the read of the name and the refusal of the name say the same
thing. The variant is the reason of the name and not the reason of a group
of names: the names of the shape of the runtime each say what they want.

The `Click` and the `MouseClick` are two functions of v2 and not one. The
components of a `Click` may be spread across the parameters and in one text
-- `Click 100, 200, "R D"` is the same as `Click "100 200 R D"` -- and the
handler of it reads the words of the parameters. The button of a
`MouseClick` is the first parameter, and it is a text that is not spread:
measured on the binary of v2.0.19 on 2026-10-31, `MouseClick "R D"` and
`MouseClick 10, 20` are refused with "Parameter #1 of MouseClick is
invalid." and the value in the `Extra`, with the space at the end of it
cut. The wheel and the fourth and the fifth button are the ones that the
desktop of the tree does not carry, and the refusal says so.

### The class types (`classes/`)

- `names(class) -> Vec<&'static str>` -- the names a class type answers:
  its methods and its properties.
- `carries(class, name) -> bool` -- whether a class type answers the name.
  The case of the name does not matter.

The rest of the module is what the runtime asks of a class: whether a
name is a class type, the class type as a value, and the construction,
the calls, the properties and the item of an object of that class.

The classes are `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`,
`Hotkey` and `Enumerator`. A script cannot extend them: their state lives in
the property table of the object under a reserved key, and only the module
that owns the class touches it. The module is asked before the base
object, so a class that answers a name itself is taken at its word.

The table of the surface carries the twenty-nine class types of the dialect,
and a script reads every one of them as the class. The tree keeps two
notions apart: `is_type` is the class of an object, and
`is_a_name_of_a_class` is a name that a script reads as a class. The
`Hotkey` is the reason: the name is a function of v2, and the class of a
hotkey is the class of the object that the function makes.

`CONSTRUCTORS` carries the arity of every constructor and the two names that
go in the `What` and in the `Extra` of the refusal of a call that has the
wrong number of arguments. The number of the arguments is checked before the
object is made, and before the reason of the machine: the dialect checks it
first. The `Any` is not a class that can be made -- the call of it goes
through the `Call` method of the class, and a class has none -- and the COM
objects are a thing of Windows, and this runtime does not carry them.

The `Enumerator` is the enumeration of v2: the `__Enum` of a value and the
`OwnProps` of an object hand one out, and its `Call` writes the turn into
the variables of the caller. The key and the value under it are the turn,
and the one of the two that it gives alone is the one that the producer
names: the value of an array, the key of a map, the name of a property. A
turn that has no value of its own leaves the variable of the caller without
one.

The item of an object of a class type is asked of the module that owns
the class, and the write of one goes the same way. Only a `Map` and an
`Array` have a way to write an item: the others answer that they have
none, and the statement fails with that reason.

`READ_ONLY` carries the properties of the class types that may not be
written: the count of a map, the pointer of a buffer, the name, the two
numbers and the two flags of a function and of an enumerator, and the end
and the handle of a file. The write of one of them is refused at the run
with "Property is read-only.", the name of the property is in the `Extra`,
and the `What` is empty. The class value is not an instance of it, and
`Map.Count := 1` is the property of the class value itself.

The encoding of a `File` is the encoding of the script when the script does
not say which one it wants, and the code page of the machine is the default of
the script: `CP1252`, the code page of the machine where the dialect was
measured. The byte order mark of the encoding is written when the file is
opened to be written, and the bytes of the file decide the encoding of it when
it is opened: a file that begins with the mark of UTF-8 is read with UTF-8,
whatever the script asked for.

The options of the `Gui` are the dialect's, and they are read here: the
words of the text, each one of them with a sign and with the value of the
option, and the ones that v2 does not know are refused with "Invalid
option." and the name of the word. The owner of a window and the parent of
it are windows, and this desktop has none, so the construction is refused
with the reason of the machine.

### The clipboard of the class types (`classes/clipboard.rs`)

The `ClipboardAll` is the class of the dialect that carries everything that
is on the clipboard. The tree carries no binary clipboard data: the
clipboard of the desktop is a text. The call of the class with data fails
with the reason of the machine, and the checks of the dialect come first,
because the number of the arguments and the kind of them are the dialect's.

### The desktop (`desktop.rs`)

The fifteen capability traits, and the `Desktop` that is their sum. Every
implementor writes the fifteen of them: there is no blanket implementation,
so a desktop that grows a capability grows with it. The tenth is `Native`,
the call into a library of the machine, the eleventh is `Sound`, the
volume of a device and the playing of a sound, the twelfth is `Machine`,
the machine itself: the user, the computer, the folders and the session,
the thirteenth is `Dialogs`, the questions of the user, the fourteenth is
`Network`, the way out to the network of the machine, and the fifteenth is
`Pictures`, the images of the machine. Every method that a
capability does not have has a default that answers with the reason it
cannot.

The `Windows` trait carries the mode of the matching of the titles with the
question, because the mode is a setting of the script and the rule that goes
with it is the dialect's: `TitleMatch` says what the mode 1, the mode 2, the
mode 3 and the pattern want, and the desktop gives the titles of the machine.

### The pictures (`pictures.rs`)

The two names of v2 that read the images of the machine: `LoadPicture`
loads a picture of a file and answers the number that identifies it, and
`ImageSearch` looks for a picture on the screen. The options of each one
are read here, because they are the dialect's, and the image is the
machine's: the runtime reads the options and hands them to the desktop,
and the desktop reads the image.

The `LoadPicture` answers zero when the picture could not be loaded: v2
does not fail when the file is not there. The `ImageSearch` answers one
when the picture is on the screen and zero when it is not, and the place
of the picture is written in the first two variables of the caller.

### The dialogs (`dialogs.rs`)

The three questions the script asks the user: `InputBox`, `FileSelect`
and `DirSelect`. The options of each one are read here, because they are
the dialect's, and the window is the machine's: the runtime reads the
options and hands them to the desktop, and the desktop asks the user.

The `InputBox` answers an object of the class `Object`, and the object
carries the text of the user in its `Value` and the way the dialog ended
in its `Result`: "OK", "Cancel" or "Timeout". The `FileSelect` answers
the path of what the user chose, and it is an array of the paths when the
user may choose more than one. The `DirSelect` answers the path of the
folder.

The refusals are the ones of v2, measured on the binary of v2.0.19 on
2026-10-29: an option of an input box that v2 does not know is refused
with "Invalid option.", of the class `ValueError`, and the option that
was not known is in the `Extra` of the failure; an option of a file
selection that v2 does not know is refused with "Parameter #1 of
FileSelect is invalid."; the filter of a file selection is not there when
the user chooses a folder, and it is refused with "Parameter #4 of
FileSelect is invalid."; and the options of a `DirSelect` are a number,
and a text that is not one is refused with "Parameter #2 of DirSelect
requires a Number, but received a String."

### The date, the files, the settings and the hotkeys

Four modules that answer what the runtime used to answer with a reason.

- `date.rs` -- the date and the time. The arithmetic is pure and lives
  here: `date_add`, `date_diff`, `format_time` and the number of fourteen
  digits that is the date of v2. The date of the machine comes from the
  host, through the `Clock` trait: the host reads the zone (`/etc/localtime`
  or the `TZ`) and says whether it knows it, and the module asks the clock
  for the moment. A host that does not know the date has no answer, and
  the names of the date say so. The units of v2 are `d`, `h`, `m` and `s`,
  and the unit comes last: `DateAdd(date, amount, unit)`.
- `files.rs` -- the files of the machine, in the dialect of v2: the copy,
  the move, the attributes, the times, the directories and the ini files.
  The failure of a command is the failure of v2: the code of the system in
  front of what it says for the failure of the machine, and "Failed" for a
  copy, a move or the delete of a folder that does not happen.
  The names that *set* something take the value first
  (`FileSetTime "20000101000000", file, "M"`), and the ones that *read* it
  take the file first. `FileExist` answers with the attributes of the file
  and not with a number, and the pattern of it may carry the wildcards:
  the answer is the attributes of the first entry that matches, and
  `DirExist` takes a folder among them. The encoding of the files is one
  state, and it lives in `settings.rs`.
  The read of a file is the text of it, and the options of the read and of
  the write are the options of v2: the encoding of the file, `RAW`, `m<n>`
  and a linefeed. The bytes of the file are the machine's, and the text of
  them is the dialect's: the module asks the desktop for the bytes and
  reads them in the encoding that the script asked for.
- `settings.rs` -- the settings of the script: the mode of the
  coordinates, the delays, the method of the send, the speed of the
  pointer, the mode of the caps lock and the encoding of the files. The
  state is in the module and not in the runtime: the runtime has no field
  for it, and the crate runs without the standard library, so there is no
  mutex of the standard library either. The modes of the coordinates that
  need windows answer with the reason. The encoding of the files is the
  name that the script wrote: `CP0` is the code page of the machine, and
  the tree carries one code page -- `CP1252` -- and refuses another one
  with the reason of its own.
  The settings that the tree keeps are the settings of v2: the mode of the
  matching of the titles, the speed of it, the hidden windows, the hidden
  text, the delay of the windowing functions, the delay of the controls,
  the view of the registry and the five modes of the coordinates. The
  commands of them answer the value that was there before, and the one
  that the tree cannot honour refuses with the reason of its own name. A
  script that writes the name of a setting changes it: `A_ControlDelay :=
  50` makes the delay 50, and the command of the delay answers 50 after
  it.
- `hotkeys.rs` -- the hotkeys of the script. The runtime holds the
  registry and the desktop holds the watcher: the script says which
  function a hotkey runs, and the machine says which key was pressed. The
  runtime asks between the statements of the script, and only at the top
  level.

### The marshalling of a native call (`dll.rs`)

The shapes of a call into a library of the machine, and the marshalling of
the values. It is pure, so it belongs to this crate; the call itself is
I/O, and belongs to the desktop.

- `NativeType` -- `Int`, `Int64`, `Short`, `Char`, `UChar`, `UShort`, `UInt`,
  `UInt64`, `Float`, `Double`, `Ptr`, `UPtr`, `Str`, `WStr` and `AStr`. The
  methods are `width()`, `is_float()`, `is_text()`, `is_integer()` and
  `character_width()`.
- `Convention` -- `Standard` and `Cdecl`. The two conventions of x64 are
  the same one, so the value does not change the shape of the call.
- `NativeValue` -- `Integer(i64)`, `Float(f64)` and `Bytes(Vec<u8>)`.
- `NativeArg { kind, address, value }` -- one argument. `address` is the
  `*` that v2 writes at the end of a type: the value of the variable of the
  caller comes back from the call, and the desktop is the one that writes
  it into the argument.
- `NativeCall { library, function, convention, returns, arguments }` -- the
  call, ready to be made.
- `NativeResult { kind, value }` -- the answer.
- `NativeFailureKind` -- `Invalid`, `NoLibrary` and `NoFunction`: what failed
  in a call into a library.
- `NativeFailure { kind, reason }` -- the failure of a call. The reason is
  the one of the machine, in the words of the v2, and the kind is what the
  v1 turns into the code that it writes in the `ErrorLevel`.
- `parse_type(&str) -> Option<(NativeType, bool)>` -- the type out of the
  string the script writes, and whether it is the address of a value.
- `split_target(&str) -> (String, String)` -- the library and the function
  out of `"kernel32\GetLastError"`.
- `encode` and `decode` -- the bytes of a text: UTF-16 for `Str` and
  `WStr`, UTF-8 for `AStr`. The text ends at the first nul.
- `encode_number` and `decode_number` -- the bytes of a number, in the
  width its type asks for.

### The built-in variables (`variables.rs`)

- `THE_RUNTIME_OWNS` -- the built-in variables the runtime writes where
  it runs: the loop variables, the line of the statement and the function
  that runs.
- `carries(name) -> bool` -- whether the tree carries the built-in
  variable of that name.
- `refusal_of(name) -> Option<&'static str>` -- the reason the name
  refuses with, when the tree refuses it.
- `WRITABLE` -- the built-in variables that the dialect lets a script
  write: thirty-seven of the hundred and thirty-one of the surface.
- `is_read_only(name) -> bool` -- whether the dialect does not let a script
  write the name.

The name of a built-in variable has two spellings, and the two meet here.
The dialect gives every one of them the prefix `A_`, and the script writes
it: the read, the write and `is_built_in` want the name the script writes,
and a name without the prefix is a variable of the script and not a
built-in variable of the dialect. The table of the surface stores the
names without the prefix, and `carries` and `refusal_of` take the name of
the table.

The variables the runtime knows -- the version, the size of a pointer,
the clock -- are answered by the module. The ones it cannot answer fail
the statement with the reason.

A name that the dialect does not let a script write is refused at the load,
and the refusal is the one of every scope. The table of the names that a
script may write is the measurement of the binary of v2.0.19, and it is not
the table of the kinds: the shape of the cursor is a setting of the run and
the v2 refuses to write it all the same, and the index of the loop is a name
that the runtime owns and a script may write it.

The v1 has its own table of the names that a script may write, and its own
reason: the write of a built-in variable of the v1 is refused with "Not
allowed as an output variable.\n     Specifically: A_AhkPath", and the
concat is the one that it accepts -- the built-in keeps the value that it
had.

The names of the machine are the desktop's: the user, the computer, the
shell, the folders, the version of the operating system, the language, the
rights of the script and the idle times are asked of the desktop, and the
reason of a question the desktop cannot answer is the one of the desktop.
The facts of the script are the ones of the run: the directory where the
run started is asked of the desktop when the runtime is made, and the name
and the path of a script that comes from the standard input are an
asterisk, the way v2 has them.
The names that the tree does not carry refuse with the reason of the name,
and that reason is written once: the read of the name and the refusal of
the name say the same thing.
- `the_kind_of_the_write(name) -> Option<Written>` -- what the dialect does
  with the value that is written to a built-in variable: it keeps any
  value, it takes the number of it, it takes the truth of it, or it wants
  a number and refuses a text that holds none.
- `the_written_value(kind, value, line) -> Result<Value, RuntimeError>` --
  the value that the name keeps, the way the dialect keeps it.

The write of a built-in variable is the write of the dialect: the settings
are written where they live, the names that v2 lets a script write keep the
value that was written, and the names that it does not let a script write
are refused with "This built-in variable cannot be used as an output
variable.\n     Specifically: A_AhkVersion".

The refusal of the write is a rule of the dialect (`dialect.rs`), and the
parser asks it: the refusal of the write of a built-in variable, and the
refusal of the increment of one. The v2 answers with its own two reasons --
the plain assignment says that the variable cannot be an output variable and
the compound operator says that it cannot be assigned a value -- and the v1
answers with one reason for every operator.

The write that goes nowhere is a rule of the dialect too, and the runtime
asks it before it writes anything: the concat on a built-in variable of the
v1 is accepted, and the built-in keeps the value that it had. The settings
of the run are written where they live, and the settings of the v2 are
built-in variables of the v1.

The increment is a rule of the dialect as well. The space that separates the
operator of the increment from the name is the rule of v2 -- the `x ++` is a
call of the `x` -- and the v1 does not have it: the `x ++` is the increment
of the `x`. The increment of a built-in variable of the v1 answers the empty
text and changes nothing, and the refusal of it at the load is the one of the
line that is the increment alone.

The parameter of a command is a rule of the dialect too. The v1 checks the
value of the parameter when the script loads -- the parser asks the dialect
after it has checked the number of the parameters -- and the complaint of
the v1 is "Parameter #N invalid." with the text that the script wrote. The
rules are in `v1/parameters.rs`: the command that reads one parameter takes
the whole rest of the line, and the command that reads more than one takes
the text of each place. The v2 answers nothing to the hook, and the value of
the parameter is checked by the handler of the command when it runs.

The defaults of the settings are a rule of the dialect too. The state of
the settings lives in a static, and the runtime writes the defaults of its
dialect when it is made: the v1 begins with the mode of the coordinates in
the "Window", with the method of the sending in the "Event" and with the
mode of the matching of the titles in 1, and the v2 with the "Client", the
"Input" and the 2. The names that the v1 spells in its own way -- the flags
that are the "On" and the "Off" -- are answered by the module of the
variables of the v1, and they come before the ones of the v2.

The `Run` of the v1 is not the `Run` of the v2. The v1 starts the program
and goes on, and the v2 waits for it: the `Run` of the v1 asks the machine
for a program that is started without being waited for, and the machine
answers with the pid of it. The pid goes into the fourth parameter of the
command -- the variable of the caller -- in the `Run` and in the `RunWait`.
The machine of the runtime has two capabilities for it: `launch`, which does
not wait, and `run_and_wait`, which does. The box starts the program
through the shell of the machine, the way its `run` does.

The failure of a native call is a rule of the dialect too. The machine says
what failed -- the call itself, the library, or the function in the library
-- and the v1 turns that into the code that it writes in the `ErrorLevel`:
the minus two, the minus three and the minus four. The v2 does not use the
code: it keeps the reason of the machine.

The files of the v1 are a rule of the dialect too. The source of the
`FileCopy`, of the `FileMove` and of the `FileDelete` may be a pattern, and
the command does the thing to every file that matches it. The `ErrorLevel`
is the number of the files that failed, and the v1 goes on: the `ErrorLevel`
is the only report of it. The v2 refuses the wildcards, and its
`FileExist` answers with the attributes of the first entry that matches the
pattern.

### The regular expressions (`regex.rs`)

- `Regex` -- a compiled pattern.
- `Match` -- a match of a pattern in a text: `span()` says where it is,
  counting from zero, and `text(chars)` says what it holds.
- `find(text, pattern) -> Result<Option<usize>, String>` -- where the
  pattern is in the text.
- `MAX_STEPS` -- 200 000: the most steps the machine of the pattern
  takes, so that a pattern that cannot stop stops.

The machine is the Pike VM of the language, and it runs without the
standard library like the rest of the crate.

The pattern of v2 is exact, and the option `(?i)` at its front is what
takes the case away. That is the one option the engine carries: a pattern
that asks for another one is refused with the reason.

## Invariants

- **The crate does no I/O of its own.** Nothing here opens a file, starts a
  process or writes to disk. The script is compiled in memory and executed
  statement by statement; the source is gone when the turn is over.
- **The desktop is an interface, not a dependency.** The runtime holds a
  `&mut dyn Desktop` and calls into it. The real one lives in `ahk-box`; the
  tests use a fake that records the calls, and that is the only reason the
  dialect is testable without a box.
- **The clock is the host's, when the host has no `std`.** `now_ms` never
  goes backwards and `sleep_ms` waits at least as long as it is asked to. The
  same clock measures the timeout, the steps and the pause of `Sleep`.
- **The guard rails are checked before every statement.** The step count
  goes over `MAX_STEPS` first, and the timeout second, so the message names
  the statement that would have been the 20 001st. A counted `Loop` has no
  iteration guard of its own: it is bounded by the step count and by the
  timeout. A `Loop` without a count is bounded by the step count too, and it
  counts each of its turns as a statement, because the body of it may be
  empty and the step count is the bound that still moves.
- **The fields of the loop that parses are the characters of the text.**
  The delimiters that are not there make every character a field of its own,
  and the spaces are fields too; the characters that are not wanted at the
  ends of a field are taken away from it.
- **The variables of the loops are there before the first loop runs.** The
  index is zero and the others are empty, and the line of the file is
  `A_LoopReadLine` -- `A_LoopField` is the field of the loop that parses.
- **The index of the turn is the number of it,** and every loop counts:
  the counted one, the one without a count, the `while` and the `for`. The
  loop saves the value of the index when it starts and gives it back when
  it ends, and a loop that is inside another one does not disturb the index
  of the outer one. The index of a counted loop is the value of `A_Index`,
  and the body may write it: the loop goes on from the value that was
  written.
- **A variable that has no value is a text that is empty.** The capacity of
  a text is the length of it: the tree keeps no capacity of its own, and
  the strings of the tree have no room to spare.
- **An object is not written as a text.** v2 reads the property "Ptr" of
  the value that it writes, and an object has none. A class is a class of
  its own kind ("Class"), and a prototype is of its own too ("Prototype").
- **The failure of a built-in names the built-in.** The `What` of the failure
  is the name of it, and the `Extra` is the value of the parameter that
  failed -- "empty string" when the value is a text that is empty. A
  parameter that may not be left out is refused the same way, and the
  `Extra` is empty.
- **The commands of the files answer with an empty text,** and not with the
  name of what they did.
- **The name of a file that is empty is refused by the command that acts on
  it.** The message is "Parameter #1 of FileDelete is invalid.", the name of
  the built-in goes in the `What`, and the value that failed goes in the
  `Extra` as "empty string". The commands that open a file or ask about one
  let the machine answer.
- **The name of the file of a `FileAppend` may be left out only inside the
  body of a `Loop Read` that has a file of its own,** and the text goes to
  that file. Everywhere else the refusal is "Parameter #2 of FileAppend must
  not be omitted in this case.". The runtime keeps the file of the loop that
  runs, and the tree asks the machine to append to it.
- **The stack of an error names every frame of the call that failed.** The
  frame is the file, the line, the name of the frame and the source of the
  line, and the section that starts the script is the last frame and has no
  name. The script of the runner has no file of its own, and the file of
  every frame is empty. The host gives the source to the runtime with
  `Runtime::with_source`, and the runtime keeps the line of every frame as
  the statements run.
- **The error carries the line where it was made.** The line is a property of
  the error and not of the statement that reads it, and the file of it is the
  file of the script: the script of the runner has no file, and the file of
  an error and the file of every frame of its stack are empty.
- **`Sleep` is capped at 60 000 ms**, in the statement and in the builtin.
- **The timeout knows no line.** It stops the script between two statements,
  so `error_line` stays null and the message is "the script ran for more than
  Ns". Every other failure carries the line of the statement that failed.
- **A question that cannot be answered fails.** `WinActive` and `WinExist`
  turn `WindowAnswer::Unknown` into an error that says why; they do not
  answer `false`. `WinGetTitle` is the exception: with no active window it
  answers an empty string, because there is no window to ask about.
- **A statement that fails is reported once.** It stops the script and lands
  in `error` and `error_line`; it is not a step, and `ok` is true on every
  step that exists.
- **The functions are collected before the first statement runs**, so a
  script may call a function it defines further down.
- **Only `abs`, `min`, `max` and `clamp` are called on an `f64` directly.**
  They are in `core`. `sqrt`, `floor`, `ceil`, `round` and `fract` go
  through the helper in `runtime.rs`, which is `libm` under `no_std`, so a
  target with no `std` answers with the same numbers.
- **The marshalling of a native call is pure.** `dll.rs` opens nothing and
  calls nothing: it turns the values of the script into the shapes of the
  machine and back. The call itself is the tenth capability of the desktop,
  and the crate never reaches for it on its own.
- **The tree is not in rustfmt's shape, and the formatter is not part of the
  loop.** The code is written compactly, with a struct variant and a line of
  logic on one line, and `cargo fmt --all` would rewrite it into something
  else. Match the code next door.
- **The state of the settings lives in the module.** The mode of the
  coordinates, the delays, the method of the send, the speed of the pointer
  and the encoding of the files are in the statics of `settings.rs`: the
  runtime has no field for them, and the crate runs without the standard
  library, so there is no mutex of the standard library either. The
  encoding of the files has one truth: `files.rs` asks `settings.rs` for it.
- **A setting that the tree keeps is the setting of v2.** The mode of the
  matching of the titles, the speed of it, the hidden windows, the hidden
  text, the delay of the windowing functions, the delay of the controls,
  the view of the registry and the five modes of the coordinates are
  answered with the value of v2, and their commands answer the value that
  was there before. A setting that changes nothing here is kept all the
  same: the tree has no controls of a window and no registry. The one
  setting the tree does not keep refuses with the reason of its own name.
- **The mask of the menu takes the scan code, and the speed of the matching
  takes a number as the mode.** The mask is the text of the virtual key and
  of the scan code as it is -- "vk41sc01D" -- or the scan code alone, and
  the name of a key is a mask whose scan code is zero. The speed of the
  matching sends the value to the mode of the matching when the mode takes
  it, and to the speed when the speed takes it: a number is the mode, and a
  name is the speed.
- **The mode of the matching of the titles is handed to the desktop.** The
  rule of the mode is the dialect's, and it lives in the crate of the
  language: the desktop gives the titles of the machine, and `TitleMatch`
  says which one of them matches. The pattern of the mode is the one of the
  engine of the tree, and the `i)` that v2 spells at the front of a title
  is the `(?i)` of the engine.
- **The date comes from the host.** The crate does not read the clock of
  the machine: it asks the `Clock` trait for it, and a host that does not
  know the date has the names of the date answer with the reason.
- **The hotkeys run between the instructions of the script, at the top
  level.** A hotkey runs with the scope of the script and not with the one
  of a function.
- **The kind of a value is the name of its class.** An object is an "Object",
  a "Map", a "Func" or the name of the class of the script, and the
  messages of the refusals name it: "Expected a String but got an Array.".
  The operators refuse a value that they cannot work with: the
  concatenation wants a text or a number, and the maths wants a number.
- **A catch without a class takes every error.** The errors of the runtime
  are a tree of classes, and every one of them is an `Error`: a catch that
  names no class takes the whole tree, and a catch that names one takes
  that class and the ones under it.
- **The delimiters of the split of a text are literals.** The `StrSplit` cuts
  on the text that is given and not on a set of characters, and the
  delimiters of an array are tried in the order they were given.
- **A text that is empty has no piece.** The `StrSplit` of an empty text
  answers an array with no piece in it, and the piece that is empty is not a
  piece.
- **A property is written with a descriptor.** The `DefineProp` takes an
  object with a `Value`, a `Get`, a `Set` or a `Call` in it, and the
  descriptor lives under the reserved key of the runtime.
- **The concat of a variable that has no value is the concat of an empty
  text.** The other compound operators want a value that is there, and a
  variable that has no value is an error for them.
- **The compound operators read the property before they write it.** A
  property that is not there is an error that names it, and the setter of the
  object is not called.
- **The name of a class is a name of the script.** It may not be the output
  variable of an assignment, and the parser refuses the script when it reads
  it. The classes of the runtime are names of the script too.
- **A name may be declared only once.** The parser refuses the second
  declaration of a name, and the message names the kind of the one that is
  already there.
- **A local that is declared is the local one.** A local that has no value
  until something writes it has no value at all, and the global of the
  script is not read in its place.
- **The base of an object is another object.** The second parameter of the
  `HasBase` is an object, and the question is whether the base of the object
  is it; the chain of the bases is walked, and every chain ends at the
  prototype of the base object.
- **The read of a file is the text of it.** The bytes of the file are the
  machine's and the text of them is the dialect's: the module asks the
  desktop for the bytes and reads them in the encoding that the script
  asked for.
- **The enumerator of a loop is something that can be called.** A function,
  an enumerator of the runtime, or an object with a `Call` method. A list
  is not one, and a value that is not an object is a mismatch of the type.
- **The options of a file are the options of v2.** The encoding of the
  file, `RAW`, `m<n>` and a linefeed; an option that v2 does not know is
  refused with "Invalid option." and the option goes in the `Extra`.
- **The methods of the base object are inherited.** Every object has them,
  and the question with no name asks whether the value is a class.
- **A class that this runtime does not carry says why.** The members of
  the `InputHook` and of the `ClipboardAll` are refused with the reason
  of the runtime -- "this runtime does not carry that class yet." -- and
  a class value keeps the words of the dialect.
- **The base of an object is the prototype of its class, and the base of
  a class is the class that it extends.** The copy of a class is a class
  of its own, and the call of a class value makes an object of the
  class.
- **A class that the runtime makes is a class of the dialect.** The
  `BoundFunc`, the `Closure` and the `RegExMatchInfo` are classes, and a
  script reads them as one, even though the documentation does not name
  them.
- **The members of the class types are in the table of the surface.** The
  `InputHook` and the `Gui.TreeView` are in it, and a name of a class
  that the tree does not carry is refused with the reason of the runtime
  and not with the words of the dialect.
- **The static methods of a class are handed over as values.** Every
  class has the methods of the base object as static ones, and the call
  of the class is one of them.
- **The table of the surface carries the members that the binary keeps
  outside its tables.** The properties of the error object are in it, and
  the tree answers every one of them.
- **A class value has no name, and its base is written the way v2 writes
  it.** The name of a class is kept under a key of its own, the base of a
  class is the class that it extends, and the write of it is the write of
  the `ObjSetBase`. A class value is not an instance of its class: the
  static methods are its own and the others are not.
- **Two objects are the same object and nothing else.** v2 compares them
  by their identity, and an object is never equal to a value that is not
  an object.
- **The members of a class value are in the table of the surface.** The
  class of a class value is "Class", and it has the base, the prototype,
  the call of the class and the initialiser of it. The four methods that
  every object has are in the table as well.
- **The own properties of a class value are the members of it.** A class
  of the runtime owns the call of it and the prototype, and a class of the
  script owns the initialiser of it, the prototype and the static methods
  that it declares. The methods of the base object are inherited, and not
  own.
- **A name may be declared only once in a class, and the failure names
  the second one.** The tables of a class are four -- the methods of the
  instance, the methods of the class, the properties of the instance and
  the properties of the class -- and a method and a property are two names
  and not one. A static `__Init` is a duplicate of the initialiser that
  every class has, and the `__Init` of an instance and the properties of
  the class cannot both be there: only the properties that come before the
  `__Init` are the ones that cannot, and the ones that come after it are
  assigned after the body of it.
- **A load error that names a thing writes it on the second line.** The
  five spaces before the `Specifically:` are the ones of v2.
- **The names of the own properties are the ones that the script wrote.**
  The table of the properties is keyed by the name in lower case, and the
  name is kept beside the value: the first write is the one that fixes the
  case of it, and the names are in the order of v2.
- **The own properties of a function are none.** The members of a function
  live on the prototype of the class of it, and the function owns none of
  them.
- **The base of an object of a class type of the runtime cannot be
  written.** The prototype of the class is there, and the refusal says
  "Invalid base.".
- **The `GetMethod` of the prototype fails with the words of the
  prototype.** The prototype of a class is a property of it, and the method
  that is asked for is the call of it.
- **The prototype of a class owns the members of the class.** The class of
  the value and the methods and the properties of the class are its own
  properties, and the property of the class cannot be read on it: the
  prototype is not an instance of the class.
- **A method is a value.** The read of a method hands a function over, and
  the `GetMethod` of a value hands the same one over; the object is one of
  the arguments of the call.
- **A static method belongs to the class and not to its instances.** The
  prototype of the class does not carry it either, and the class value has it
  as one of its own properties.
- **The methods of a class are the properties of an instance.** The `HasProp`
  of a method answers 1, and so do the initialiser and the base; a static
  method and a static variable are not members of an instance.
- **A function may not contain a class.** The parser refuses it at the load,
  and it names the line of the class.
- **A class may be declared inside a class.** Its name is the name of the
  class that holds it and the name of it, and it is an own property of the
  class value of the class that holds it.
- **A property of a class may have a getter and a setter.** The read of it is
  the value of the getter, the write of it calls the setter, and the property
  is own of the prototype of the class.
- **The item of an object of a class of the script is the property `__Item`
  of it.** A method of that name is not the item: the item is the method
  itself.
- **A line that ends with an operator continues on the next one, and a line
  that begins with an operator continues the one before it.** The group, the
  array, the object and the arguments of a call may be written in more than
  one line.
- **The operand that is not there is refused with the words of v2.** The
  operator that wants it names the failure, and a brace that closes a block
  is a token that cannot be there: the statement it is in is named with it.
- **The name of a property of a class may be a number, and the base of a
  class is a class.** The base that is not there is refused at the load, and
  the base may be declared after the class that extends it.
- **The own properties of an error are the ones that v2 spells, and the class
  value of a class of the runtime owns the call of it and the prototype.**
  The table that the tree keeps is its own, and the name of the class is
  not a property of the class value.

- **The increment writes to a variable, a property and an item, and the
  left side of an assignment is a variable, a property or an item.** The
  increment may be a statement of its own, and the value that cannot be
  written to is refused at the load with the operator that writes it.

- **The `static` is the third kind of declaration of v2.** The name lives
  in the storage of the function, it keeps its value between the calls, and
  the value of the declaration runs the first time that declaration is
  reached.
- **The class of v2 is initialised when it is first referenced, and the
  `__New` of an instance is the constructor of it.** The statement that
  defines the class is a use, and the `__Init` of v1 runs before the
  `__New`.

- **The number of v2 may begin with a point, and the point that follows a
  value is the point of a member.** The number `.5` is 0.5, and the name of
  a property of the literal of an object may be a whole number.

- **A statement that begins with an operator is a line that holds no
  action.** The text that is named is the line itself, and the dot is the
  member of a value that is not there.

- **The percent signs of v2 build a name, and the name may be read and
  written.** The name is the text that is written and the values of the
  expressions, and the pieces are joined without a space between them. The
  name of a variable is a name and not a path of names, and the name of a
  member may be built too: `m.%x%` is the member whose name is the value of
  `x`. A name that the script builds does not create a variable, and a
  function that writes one says that it wants a global declaration.

- **The bracket that is not closed is the failure.** The operand that is
  not there inside it does not change the message, and a brace that closes
  a block is named before the bracket. The text that is named is the
  statement and the line of the failure.

- **The argument of a command is a value of the dialect, and not a word of
  the language.** The brace that opens one opens the literal of an object,
  and not a key, and the button of a click is a value: `Click 10, 20,
  right` reads the variable `right`. The name of a property of the literal
  of an object may be built by the script.

- **The button of a `MouseClick` is the first parameter, and it is
  checked.** The words of it are not spread, and a button that is not one
  of the words of v2 is refused with "Parameter #1 of MouseClick is
  invalid." and the value in the `Extra`, with the space at the end of it
  cut.

- **The reserved word that is not wanted says what it is.** The words that
  may not be the name of a variable say so, the words that are reserved for
  a future of v2 say "Unexpected reserved word.", and the words that are
  not operators say "Syntax error.". The tree has no cast: the word `as`
  is a reserved word.

- **The operator `is` carries the value that failed in the `Extra`.** The
  right side of it has to be a class, and the value that arrived goes in
  the `Extra` of the failure, and the `What` is empty.

- **A parenthesis holds a list, and the value of the list is the last of
  its items.** Every item of it runs, the list of nothing is a "Syntax
  error.", and the parenthesis that is spaced is not the list of the
  arguments of a call: it is one parameter of a command.

- **The parameter that a command does not give a value to keeps its
  place.** It has no value: a parameter that has a default takes it, and
  one that has none is a missing parameter, and the call is refused with
  "Missing a required parameter.". The one that is the last of the list is
  dropped: the commas at the end of the list add nothing.

- **The statement of v2 is an expression, and the comma ends it.** The
  operators and the concatenation of v2 are a part of it, and the value of
  it is thrown away. The rest of the line after the comma is the list of
  v1: it is evaluated, and its value is thrown away too. The name of a
  command of v2 is followed by a space or a parenthesis, and the comma
  that is there instead of the space is refused with the words of v2.

- **The member that has no parentheses is a command of v2, and it carries
  its arguments.** The space is the concatenation of v2 and the comma is
  what separates the arguments, and the comma that comes right after the
  member with no space is not one of them. The line that ends with a comma
  continues on the next one.

- **The number of the arguments comes before the values.** The refusal of
  the arity comes before the failure of an argument, and a method is the
  same way.

- **The brace that follows a value opens the literal of an object.** The
  literal may be written in more than one line, and the properties of it
  are separated by commas: the newline is not a separator. The brace that
  follows a value is refused with "Unexpected \"{\"" when the literal is
  closed, and with "Missing \"}\"" when it is not, and the statement is
  named with it.

- **The character that is illegal is illegal where it is glued.** The hash
  and the semicolon are a comment and a directive only when they are
  apart; glued to a token they are characters of the script, and the
  refusal names the text that holds them. The one that is inside an
  expression is an illegal character in an expression, and the one that
  begins a statement is a line that holds no action.

- **The brace that opens a line opens a block.** The statements between
  the braces run, the content may be on the line of the brace that opens
  it, and the brace that closes it is alone in its line -- unless the
  block is empty. The brace that follows a value opens the literal of an
  object, and the place of the brace is what tells the two apart.

- **The literal of an object is closed before it is read.** The v2 looks
  for the brace that closes the literal before it reads the properties of
  it: the literal that is not closed is the failure, and it swallows the
  rest of the script. The failures of the literal are named with the line
  of the brace that opens it.

- **The point of a number is in the number when a name does not come
  after it.** `1.ToString` is the number 1 and the member that follows
  it, and `1.` is the number 1.0. The or-maybe wants a variable on its
  left, and the v2 refuses the script at the load when it is another
  value. The three-way comparison is not a token of the v2: the `===` is
  the `==` and the `=`.

- **The literal of an array may have a place that has no value of its
  own.** The hole is a comma that follows a comma, and the hole is a
  place of the array: it counts in the length, and the read of it says
  "Item has no value.". The hole that is the last one is dropped, the
  way the parameter that is not there is dropped at the end of a call.
  The enumeration of a hole is a turn of the enumeration too, and the
  turn has no value of its own: the variable of the loop is left without
  one, and the script that reads it is told that the variable has no
  value.

- **The `break` and the `continue` need a loop around them.** The script
  is refused at the load when there is none: "Break/Continue must be
  enclosed by a Loop.". The body of a function is a scope of its own, and
  the `break` of a function that is written inside a loop is refused too.
  The question that has no colon says "A \"?\" is missing its \":\"", and
  the colon that is left over after an expression says "A \":\" is missing
  its \"?\"".

- **The body of a definition may be an expression.** The fat arrow is what
  says so: `f(a) => a + 1` is a function that answers the expression of
  it, and the body of the definition is a `Return` of it. The parameters
  are the ones of a function, and the method of a class may be written the
  same way. The definition that has no body is refused at the load:
  "Invalid function declaration.".

- **A case of a switch may name many values.** `case 1, 2:` matches when
  the subject of the switch is one of them, and the comma at the end of the
  list adds nothing. The `Default` is the case with no values.

- **The word that may not be the name of a variable says so.** The word
  `until` is reserved: "The following reserved word must not be used as a
  variable name:" and the word. The words that are not operators -- `not`,
  `and`, `or` and `is` -- are a "Syntax error." when they are the name of a
  variable, and the words that are reserved for a future of v2 -- `as`,
  `in` and `contains` -- say "Unexpected reserved word.". The text that is
  named is the word alone, and the word alone at the start of a statement
  is a call: the text of it carries the parentheses. The bare `until` wants
  a condition, and the binary crashes when it has one: the tree refuses it
  with "Unexpected \"Until\"".

- **The number of the parameters of a command of the v1 is the number of the
  binary.** The complaint of the v1 has two forms: the one that names the
  command ("\"X\" requires at least N parameter(s).") and the one that names
  the parameter that has to be there ("Parameter #N required"). The rule is a
  hook of the dialect, and the commands that are not in the table of the v1
  answer with the complaint of the parser.
- **A call that is a statement of the v1 is a call,** and the name that is
  not a function is refused at the load: "Call to nonexistent function." and
  the text of the call.

- **The arithmetic of the v1 is on a variable of the script.** The `EnvAdd`,
  the `EnvSub`, the `EnvMult` and the `EnvDiv` write into the variable that
  their first parameter names, and the number that is not there is a zero.
  The two whole numbers give a whole number, and the division of them is
  rounded towards the zero.

- **The environment of the v1 writes the `ErrorLevel`, and the name of the
  variable may not be blank.** The `EnvSet` answers with one when the
  machine refused the name and with zero otherwise, and the v1 goes on: the
  reason of the failure is a note and not the end of the script.

- **The last parameter of a command of the v1 takes the rest of the line.**
  The text that is left of the line goes into the last parameter that the
  command reads, and it is the source of the line: the `AutoTrim, On,extra`
  names "On,extra". The command that has a body is the exception: the text
  that comes after its parameters is the action when it begins with the name
  of a command, and the last parameter takes the rest of the line when it
  does not.

- **The name of a variable of the script is checked when the script loads.**
  The name may not carry a character that is not a letter, a digit, the
  underscore, the "#", the "$" or the "@", and the complaint of the v1 names
  the name and not the parameter. The percent sign opens the name of
  another variable, and the one that is not closed is refused.

- **The value of the parameter of a command of the v1 is checked when the
  script loads.** The rules of the v1 are in `v1/parameters.rs`: the button
  of the `IfMsgBox`, the letter of the `StringMid`, the mode of a group, the
  way of the move of a directory, and the command that takes no parameter.

- **The name of a variable of the v1 may carry the "#", the "$" and the
  "@".** The lexer asks the dialect before it reads the name
  (`accepts_a_hash_a_dollar_and_an_at_in_a_name`), and the v2 does not read
  them.

- **The sub-command of a command of the v1 is checked when the script
  loads.** The sub-command that the v1 does not know is refused with
  "Parameter #N invalid.", and the one that it knows has rules of its own:
  the parameter that it does not take has to be blank, the parameter that
  it needs may not be blank, and the parameter that it does not take and
  the ones after it have to be left out. The rules are in
  `v1/sub_commands.rs`, and the parameter that may not be blank is a hook
  of the dialect (`the_parameter_may_not_be_blank`).

- **The label that a command of the v1 names is checked when the script
  loads.** The label that is not there is refused with "Target label does
  not exist.", and it names no text. The `GroupAdd` names it in its fourth
  parameter, the `Hotkey` in its second and the `SetTimer` in its first,
  and the label may be written anywhere in the script. The rules are in
  `v1/labels.rs`, the label of a command is a hook of the dialect
  (`the_label_of_the_command`), and the check is at the end of the parse,
  because the names of the labels of the script are known only then.

- **The goto of the v1 is checked when the script loads.** The label that
  the goto names and that is not there is refused with "Target label does
  not exist.", and the label is named after it. The jump that may not be
  made is refused too: the label of a block that does not enclose the jump
  is "A Goto/Gosub must not jump into a block that doesn't enclose it.",
  and the jump that leaves a function is "Cannot jump from inside a
  function to outside.". The block of a label is the chain of the blocks
  that enclose it. The rules are in `v1/labels.rs`, the goto is a hook of
  the dialect (`the_goto_is_refused_with_the_labels`), and the complaint of
  the jump that the runtime refuses is the one of the dialect too
  (`the_refusal_of_a_jump_into_a_block`).

- **The `GetKeyState` of the v1 is a command and a function.** The call of an
  expression is the function and the command statement is the command: the
  command writes the letter D or U into the variable of the caller, and the
  function answers one or zero. The command has no maximum -- the v1 ignores
  the parameters that it does not need -- and the name of the key is checked
  when the script loads: "Parameter #2 invalid." The names of the keys are in
  `v1/keys.rs`, and the two rows of the table are the two forms of the name.
- **The hotkey of the v1 is a line of its own.** The two colons that separate
  the name from the action are what says so, and the line of a command is not
  the line of a hotkey. The name is a set of prefixes, a set of modifiers and
  the name of a key, and the name that is not a key is refused when the script
  loads: "Invalid hotkey." The hotkey that is declared twice is refused too:
  "Duplicate hotkey." The action of the hotkey is the body that comes after
  the line, or the command that the line carries, and the body is not a
  block: the `return` of it ends the thread of the hotkey and not the script.
  The names are in `v1/hotkeys.rs`, the line of the hotkey is the node
  `Node::HotkeyDefinition`, and the hotkeys are registered with the desktop
  when the script loads.

- **The `if` of the v1 is an expression, and the text that follows it is not
  run.** The condition of the `if` is the expression of the line -- the
  concatenation of the v1 is a part of it -- and the text that the expression
  does not reach is the tail: the v1 parses it and does not run it. The body
  of the `if` is the line that comes next, or the block, and the tail that
  ends with an operator takes the line that comes with it. The `else` of the
  v1 takes no parameter, and the text that comes right after it is the body
  of it; the `else` that has no `if` before it is refused at the load:
  "ELSE with no matching IF". The tail is in `parser.rs`
  (`the_tail_of_the_if_of_the_v1`), and the dialect says that the `if` may
  carry one (`the_if_may_carry_a_tail`).
- **The legacy `if` of the v1 compares a variable with the text of the rest
  of the line.** The text is not an expression -- the `if x = 1 + 1` is false
  -- and it goes through the legacy substitution. The `=` and the `!=` of it
  ignore the case, the `==` of the v2 is not an operator of it, and it has no
  One True Brace: the brace of the line of the condition is a part of the
  text, and the brace that closes the block is refused with "Unexpected
  \"}\"". The comparison is in `runtime.rs`
  (`the_comparison_of_a_legacy_if`), the condition is in `parser.rs`
  (`the_condition_of_a_legacy_if`), the node is `Node::LegacyComparison`, and
  the dialect says that the `if` may be the legacy one
  (`the_if_may_be_a_legacy_comparison`).

- **The legacy `if` of the v1 has the forms of the `between`, of the `in` and
  of the `contains`.** The texts of the forms are the texts of the script and
  not expressions, and they go through the legacy substitution. The `in`
  compares as text and never as a number, the list is cut at the commas with
  the spaces around them inside the item, and two commas in a row are one
  comma in the item. The `between` takes the bounds inclusively and cuts the
  text at the first word `and`. The three forms and the `=` of the legacy
  `if` follow `StringCaseSense`. The match is in `runtime.rs`
  (`the_match_of_a_legacy_if`), the condition is in `parser.rs`
  (`the_form_of_a_legacy_if`), and the node is `Node::LegacyMatch`.
- **The tail of the `if` of the v1 leaves the `if` without a body.** When the
  tail ends with an operator it takes the line that comes with it, and the
  statement that comes after the tail is a statement of its own: it runs
  whether the condition is true or false. The `else` that comes right after
  the tail is the other side of the `if`, and the body of it runs in the two
  sides.
- **The truth of the v1 is the number of the text.** The text of a condition
  of the v1 is a number when the syntax of the v1 accepts it, and the
  condition is false when that number is zero: the "00" is false. The `!` and
  the `&&` and the `||` of the v1 see the text that was written between
  quotes as a string, and a string is true when it is not empty, and they
  answer one and zero and not the operand. The truth of the run is in
  `runtime.rs` (`the_value_is_true` and `the_operand_is_true`), and the
  dialect says which of the rules hold (`the_text_that_is_a_number_is_false`,
  `the_text_that_was_written_is_a_string` and
  `the_operators_answer_a_number`).
- **The legacy `if` of the v1 takes a number as the name of a variable.**
  The names of the v1 may begin with a number, and the `if 1 = 1` compares
  the variable "1" with the text "1", and the variable "1" has no value.
  The number is a name when the text of it has no point, and the `if 1.0 =
  1.0` is the form of the expression. The variable that is named "0" holds
  the text "0" when it has no value, and the `if 0 = 0` is true because of
  it. The parser takes the number as the name (`the_condition_of_a_legacy_if`),
  the rule of the name is in `crates/ahk/src/v1/parameters.rs`
  (`is_the_name_of_a_variable`), and the value of the variable that is named
  "0" is in `runtime.rs` (`the_value_of_an_unset_variable`).
- **The `if` and the `while` of the v1 may come inside the line of the
  condition of an `if`.** The construct that comes inside the line has no
  body of its own: its condition is a part of the condition that holds it,
  and the line that comes after is the body of the `if` that holds it. The
  value of it is the value of its condition (`Node::LegacyControl`), the
  parser reads it in `parse_the_control_of_a_condition`, and the dialect
  says whether the control flow may be a value
  (`the_control_flow_may_be_a_value`).
- **The number that opens a statement of the v1 is the name of a variable.**
  The `0 := 9` writes the variable "0", and the `0 = 9` writes it too, and
  the `0++` is the increment of it, and the `++0` is the increment of it as
  well. The number is a name only where the statement may write: the `x := 0
  + 3` is the sum of two numbers. The parser turns the number into the name
  of a variable (`the_number_becomes_the_name_of_a_variable`), the lexer
  keeps the `++` that comes after a number as one operator
  (`the_increment_after_a_number_is_an_operator`), and the increment of a
  literal is the empty text (`the_increment_of_a_literal_is_blank`).
- **The assignment of the v1 that has no value writes the empty text.** The
  `y :=` writes the empty text, and the line that comes is a statement of its
  own: the `y :=` with a `3 + 4` under it is a line that holds no action. The
  dialect says it (`the_assignment_may_have_no_value`) and the parser asks it
  when the value is not there (`the_empty_value_of_an_assignment`). The `:=`
  at the end of the tail is an operator of the v1
  (`the_assignment_continues_the_tail`).
- **The line that comes with the tail of the `if` of the v1 is an action.**
  The `if (0) x := 1, FileAppend, A, *` with a `B` under it is a "This line
  does not contain a recognized action. Specifically: B". The parser reads the
  line as a statement to know it, and puts the state of the parser back where
  it was (`the_line_of_the_tail_is_an_action`).
- **The construct of the v1 may have no body.** The `if (1)` that is the last
  line of the script is a script that loads, and the `while (0)` that is the
  last line of it is a script that loads too, and the `else` that has no body
  is a script that loads as well. The dialect says it
  (`the_body_may_be_missing`) and the parser asks it when the body is not
  there (`the_block_needs_a_body`).
- **The message of the bracket of the v1 is the name of it alone.** The `x :=
  (1` is a "Missing ")" and the line of the script is not under it, and the
  bracket that closes nothing is an "Unexpected ")" with nothing under it
  either. The text that is not closed is a "Missing close-quote" that names
  the text that was read -- the `"abc` -- and not the line of the script.
  The dialect says which is which (`the_bracket_is_named_alone` and
  `the_text_that_is_not_closed_is_named_alone`), the parser names the
  bracket (`the_message_of_a_bracket` and
  `the_message_of_an_unexpected_token`), and the lexer names the text.
- **The items that come after the comma of the line of the condition of the
  `if` of the v1 are evaluated.** The `if (0) x := 1, y := 3` writes the
  variable "y", and it writes it whether the condition is true or false. The
  value of the items does not count for the truth of the condition: the `if
  (0) x := 0, y := 3` does not run the body of the `if`. The command of the
  v1 is not run by them: the `if (0) x := 1, FileAppend, A1, done` does not
  write the file, because the name of the command is a variable when it is
  read as an expression, and the variable has no value. The parser reads the
  items in `the_items_of_the_tail_of_the_v1` and keeps the state of the
  parser while it reads them, because a text that is not an expression is
  the parameter of a command. The runtime runs the items in the `If`
  (`Node::If`).

- **The values of the v1 may be side by side.** The v1 does not ask for the
  space that tells two values apart: the `x := "a"5` is the text `a5`, and
  the `x := 5 3` is the text `53`. The dialect says it
  (`the_space_separates_the_values`) and the parser asks it when it joins two
  values. The quote of the v1 escapes itself: the `x := "a""b"` is the text
  `a"b`. The dialect says that too (`the_quote_escapes_the_quote`) and the
  lexer asks it when it reads a text.
- **The name of the v1 may not carry a quote.** The `x := a"b"` is refused at
  the load with "The following variable name contains an illegal character:"
  and the name of it. The refusal is where the name is a variable: the target
  of an assignment, the value that is read and the member. The name of a
  command of the v1 is a text, and the `FileAppend, a"b", *` runs; the name of
  a function is not a variable, and the `a"b"(1)` is a "Call to nonexistent
  function.". The dialect says it (`the_name_may_not_carry_a_quote`), the
  lexer marks the symbol (`the_end_of_a_symbol_that_carries_a_quote`) and the
  parser refuses it where the name is a variable
  (`the_name_that_carries_a_quote`).

- **The words of the v1 may be the name of a variable.** The `while := 5`
  writes the variable "while", and the `x := while` reads it. The words that
  keep their meaning in the v1 are the operators (`and`, `or` and `not`), the
  values (`true` and `false`) and the `if`, that is a command of the v1. The
  dialect says it (`the_words_may_be_the_name_of_a_variable`), the lexer
  makes a name of the words that the v1 does not use as operators, and the
  parser reads the word as a variable where the v1 does.

- **The count of the `Loop` of the v1 is a number and not an expression.** The
  count is the text that the script wrote, and the loop is skipped when the
  text is not a number: the `loop n` is skipped, and so are the `loop 1+1`
  and the `loop 2.7`. The hex is a number: the `loop 0x3` is three turns.
  The `%` and a space force the expression, and the `%name%` is the value of
  the variable of the script. The dialect says it
  (`the_count_of_the_loop_is_a_number`), the parser reads the text of the
  count, and the runtime substitutes it and asks the number of it
  (`v1::numbers::the_count_of_the_loop`).

- **The illegal character of the v1 names the symbol.** The v1 says "The
  leftmost character above is illegal in an expression." and names the symbol
  that begins at the character -- the `;c` of the `x := 1;c`. The v2 says
  "Illegal character in expression." and names the rest of the expression. The
  dialect says it (`the_illegal_character_names_the_symbol` and
  `the_illegal_character_says_the_leftmost`), and the parser names the symbol
  of the line where the character is (`the_symbol_that_begins_at`).

- **The single quote is not a text in the v1.** The single quote begins a
  symbol, and the symbol that begins with it is refused where an expression
  wants an operand -- the `x := 'abc'` is refused with "The leftmost
  character above is illegal in an expression. Specifically: 'abc'". Where the
  symbol is a name, the refusal is the one of the name: the `'abc' := 5` is
  refused with "The following variable name contains an illegal character:".
  The dialect says it (`the_single_quote_is_not_a_text`), the lexer reads the
  symbol, and the parser refuses it where an expression wants an operand.

- **The `for` of the v1 walks the keys of an array.** The `for i in [10,20]`
  gives 1 and 2 to the `i`, and the key goes to the first name that is not
  empty: the `for , v in [10,20]` gives it to the `v`. The source that is
  not an array is no turn, and it does not fail: the `for i in 5`. The `in`
  of the v1 is the name of a variable, and the parser of the v2 asks for a
  word. The dialect says it (`the_for_is_the_one_of_the_v1`).

- **The escape of the v1 is the backtick, and the name may carry it.** The
  escape is expanded before the name is looked at: the `a`c` is the name
  "ac", and the `x := a`b` is the name "a<backspace>", which is not a legal
  name of a variable. The set of the escapes is the one of the v1: the `a is
  a bell, and the `s is the letter itself. The escape does not protect the
  quote of a text, and the percent sign that it carries is a literal: it does
  not open the name of a variable. The backtick that the escape carries is an
  illegal character in an expression, and the symbol that the complaint names
  begins at it. The dialect says it (`the_escape_of_the_dialect`,
  `the_escape_may_be_in_a_name` and `the_escape_protects_the_quote_of_a_text`).

- **The `Until` of the v1 is the condition of the loop, and the `if` of the
  v1 has no name of its own.** The `Until` is the statement that comes right
  after the body of a loop, and it ends the loop when it is true: the
  condition is read after every turn of the body. The `Until` that no loop
  takes is refused when the script loads, and the refusal waits for the end
  of the load. The `Until` that is the name of a variable is not the
  condition. The words that want a parameter name themselves in the
  complaint that they make, and the `if` of the v1 is a command that has no
  name of its own: the `if` alone is refused with '"" requires at least 1
  parameter. Specifically: if'. The dialect says it
  (`the_name_of_the_word_that_wants_a_parameter` and
  `the_refusal_of_the_until_that_has_no_loop`).

- **The refusal of a call of the v1 names the rest of the line.** The text
  that the refusal names is the rest of the line, from the name of the call,
  and not only the call: the `x := abc(c) + 1` is refused with "Call to
  nonexistent function.\n     Specifically: abc(c) + 1". The complaint of the
  number of the arguments that is wrong names the rest of the line too. The
  spaces that the script wrote are in it.

- **The definition of a function of the v1 needs the brace at the end of the
  line.** The definition is the line `f(a) {` and the line `f(a)` whose brace
  is on the line that comes next, and the rest of the line after the
  parenthesis that closes the list decides: when there is anything else
  there, the line is a call. The v2 decides with the token that comes after
  the parenthesis: the brace and the fat arrow make a definition of it. The
  dialect says it (`a_line_is_a_definition`).

- **The call of the v1 does not need the parenthesis that closes it.** The v1
  does not need the parenthesis that closes the call of a name that is not a
  function, and the call is refused at the load with "Call to nonexistent
  function.". The arguments of the call are the rest of the line, and the
  list that is not closed does not go on to the line that comes next. The
  call of a name that is a function needs the parenthesis, and the failure of
  it is "Missing ")"", and it waits for the end of the parse: the name of a
  function of the script may be defined after the call. The dialect says it
  (`the_closing_parenthesis_of_a_call_is_needed`).

- **The brace that follows a value is refused at once in the v1.** The v1
  refuses the brace at once, and it does not look for the brace that closes
  the literal of an object: the `x := 1 {` is refused with "Unexpected "{"",
  and not with "Missing "}"". The literal of an object is a literal of the v1
  too, when it begins the expression. The dialect says it
  (`the_brace_after_a_value_opens_a_literal`).

## Side Effects

- **None of its own.** The crate opens no file, starts no process, writes
  nothing to disk and touches no network. The `Report` is a value it hands
  back, not something it puts anywhere.
- **The calls it makes into the injected `Desktop`.** Typing, clicking,
  pressing keys, moving the pointer, reading a pixel, searching the screen
  for a colour, capturing a region, running a command, asking about the
  windows and calling into a library of the machine all go through the
  `Desktop` the runtime was handed. A desktop
  that cannot do one of them answers with the reason, and the runtime turns
  that reason into the `error` of the report.
- **The pause it asks of the injected `Clock`.** `Sleep` waits on the clock,
  and the runtime measures the elapsed time of every step and the timeout on
  that same clock.
- **The state it keeps while it runs.** The variables, the scopes, the
  functions, the steps, the output and the notes live in the `Runtime` and
  come out in the `Report`; nothing survives the run.
- **The two variables `MouseGetPos` writes as a side effect**: `mouse_x` and
  `mouse_y`, in the variables of the report.

## Verification

- Run: `cargo test --workspace`
- Without the standard library: `cargo build -p ahk --no-default-features
  --target thumbv7em-none-eabihf`
