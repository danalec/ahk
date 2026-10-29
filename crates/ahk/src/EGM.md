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
| `desktop.rs` | the capability traits the runtime is handed |
| `dll.rs` | the marshalling of a call into a library of the machine |
| `variables.rs` | the built-in variables of v2 |
| `keys.rs` | the key table: the evdev codes, the aliases and the key specs |
| `regex.rs` | the regular expressions of `RegExMatch` and `RegExReplace` |
| `date.rs` | the date and the time: the arithmetic, and the format of v2 |
| `files.rs` | the files of the machine, in the dialect of v2 |
| `settings.rs` | the settings of the script, and the state that keeps them |
| `hotkeys.rs` | the hotkeys of the script: the registry and the watcher |
| `surface.rs` | the state of every name of v2: the value it answers, or the reason it does not |

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
pub mod dll;
pub mod files;
pub mod hotkeys;
pub mod keys;
pub mod lexer;
pub mod parser;
pub mod regex;
pub mod runtime;
pub mod settings;
pub mod variables;

pub use ast::{AssignOp, BinaryOp, LiteralValue, Node, UnaryOp};
pub use desktop::{Desktop, WindowAnswer};
pub use keys::{KeyStroke, UnknownKey, parse_spec, resolve_name};
pub use lexer::{SyntaxError, Token, TokenKind, tokenize};
pub use parser::{Parser, parse, precedence};
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
  point, and the point of `1.5.2` is the member operator. A name that begins
  with a number is refused at the load, with the message of v2.
- `KEYWORDS: &[&str]` -- the words the language gives a meaning to.
- `SyntaxError { message, line, column }` -- the error the lexer and the
  parser raise.

### The AST (`ast.rs`)

- `Node` -- one node of the tree. The variants are `Literal`, `Var`,
  `Unary`, `Binary`, `Ternary`, `Cast`, `Assign`, `Index`, `AssignIndex`,
  `Call`, `If`, `Switch`, `While`, `Loop`, `For`, `Directive`, `Break`,
  `Continue`, `Return`, `ExitApp`, `FunctionDef`, `ClassDef`, `MethodCall`,
  `Property`, `AssignProperty`, `ArrayLiteral`, `ObjectLiteral` and
  `Lambda`. Every variant carries its line.

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
  array.
- `ObjectLiteral { properties, line }` -- the literal of an object:
  `{a: 1}` is an object of the base class with the property `a`. The name of
  a property is a name of the script, and the names of the properties do not
  know their case. A comma at the end of the list of an array, of an object
  and of the arguments of a call is allowed.
- `Lambda { parameters, body, line }` -- the literal of a function:
  `(a, b) => a + b`. The body is an expression and not a block of
  statements, and the function closes over the scope where it was written.
- `For { variables, source, body, line }` -- the loop that walks what an
  object enumerates. A name the script left out is an empty string in
  the list, so `for , value in map` is two names and the first one is
  nobody's.
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
  it is written by reference -- `Name(a, &b)`.

The precedence of the operators is the table of `ahk::precedence`, which
follows the documentation of v2. The kinds of the tokens of the binary
are not a precedence: they are what the lexer reads.

Two values side by side are one value: that is the implicit
concatenation of v2, and it binds like the dot. The parser makes it a
`Concat` as well, so the two ways of writing it are the same node.
- `Branch { condition: Node, body: Vec<Node> }` -- one branch of an `If`.
- `Case { value: Option<Node>, body: Vec<Node> }` -- one `Case` of a
  `Switch`; a `None` value is the `Default`.

### The parser (`parser.rs`)

- `parse(source: &str) -> Result<Vec<Node>, SyntaxError>` -- the whole
  script, parsed.
- `Parser::new(tokens: Vec<Token>) -> Parser` -- a parser over tokens the
  caller already has.
- `Parser::parse_script(&mut self) -> Result<Vec<Node>, SyntaxError>` -- the
  statements of the script.

A name at the start of a line, without parentheses, is a call statement:
the rest of the line is the list of the parameters. Every function may be
written that way -- a built-in and a function of the script alike -- and
the answer of the call is thrown away. The names that are neither are
checked at the end of the parse, because the script may define a
function further down.

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

The traits the runtime is handed:

| Trait | Methods |
| --- | --- |
| `Desktop` | the sum of the fifteen capability traits below: it has no method of its own, and there is no blanket implementation |
| `Keyboard` | `type_text`, `press_key`; `key_down`, `key_up`, `key_is_down` and `keys_down` have a default that answers with the reason it cannot |
| `Pointer` | `click`; `mouse_down`, `mouse_up`, `move_pointer`, `mouse_position`, `set_mouse_speed` and `set_mouse_delay` have a default that answers with the reason it cannot |
| `Screen` | `capture`, `pixel`; `pixel_search` and `screen_size` have a default that answers with the reason it cannot |
| `Windows` | `window_active`, `window_exists`; `active_window_title` has a default that answers with the reason it cannot |
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

- `Builtin { name, kind, min, max, variant, handler }` -- one name of the
  table: the name as v2 spells it, which of the two tables of the binary
  it came from, the fewest and the most parameters, the byte that picks
  the flavour of a shared handler, and the handler.
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
- `Wants` -- the type a parameter declares: `Number`, `String`, `Object`, or
  `Value` for a built-in that takes a value of any kind.
- `wants(name, index) -> Wants` -- the type of that parameter. The table holds the built-ins whose types were measured on the
  binary of v2.0.19, and the guard on a reference asks it, so that the
  message names the type the built-in wanted.
- `refusal_of(name) -> Option<&'static str>` -- the reason the name refuses
  with, when it refuses.

A name that the tree carries and cannot answer has the refusal as its
handler, and the variant of the row is the reason: the reason is written
once, and the read of the name and the refusal of the name say the same
thing. The variant is the reason of the name and not the reason of a group
of names: the names of the shape of the runtime each say what they want.

### The class types (`classes/`)

- `names(class) -> Vec<&'static str>` -- the names a class type answers:
  its methods and its properties.
- `carries(class, name) -> bool` -- whether a class type answers the name.
  The case of the name does not matter.

The rest of the module is what the runtime asks of a class: whether a
name is a class type, the class type as a value, and the construction,
the calls, the properties and the item of an object of that class.

The classes are `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`
and `Hotkey`. A script cannot extend them: their state lives in the
property table of the object under a reserved key, and only the module
that owns the class touches it. The module is asked before the base
object, so a class that answers a name itself is taken at its word.

The item of an object of a class type is asked of the module that owns
the class, and the write of one goes the same way. Only a `Map` and an
`Array` have a way to write an item: the others answer that they have
none, and the statement fails with that reason.

The encoding of a `File` is the encoding of the script when the script does
not say which one it wants, and the code page of the machine is the default of
the script: `CP1252`, the code page of the machine where the dialect was
measured. The byte order mark of the encoding is written when the file is
opened to be written, and the bytes of the file decide the encoding of it when
it is opened: a file that begins with the mark of UTF-8 is read with UTF-8,
whatever the script asked for.

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
  The names that *set* something take the value first
  (`FileSetTime "20000101000000", file, "M"`), and the ones that *read* it
  take the file first. `FileExist` answers with the attributes of the file
  and not with a number. The encoding of the files is one state, and it
  lives in `settings.rs`.
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
  timeout.
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
