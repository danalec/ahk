# The v2.1: the mechanisms beside the v2.0.31

The `--dialect=v2.1` is the v2.0.31 with the mechanisms of the AutoHotkey
v2.1 beside it. The names do not move -- the surface is the 843 of the v2 --
and only the grammar and the runtime rules that the v2.1 adds are gated.
This document is the guide to that gate, and the record of what the
official v2.1 documentation says beside what the tree carries.

The official v2.1 is an **alpha**: the release this was read from is
**2.1-alpha.39**. The documentation was read out of the `alpha` branch of
[AutoHotkey/AutoHotkeyDocs](https://github.com/AutoHotkey/AutoHotkeyDocs/tree/alpha),
and the source of the interpreter out of the `alpha` branch of
[AutoHotkey/AutoHotkey](https://github.com/AutoHotkey/AutoHotkey/tree/alpha).
The tree's answers were measured on the runner the same day, with the
instrument beside this document (`scripts/probe-v2.1.py`).

One thing the reader must know up front: **the v2.1 the tree speaks is not
only the upstream's.** Some of the gated mechanisms come from the test
suite of KeySharp, which states that it speaks the v2.1-alpha.37, and the
upstream parser does not carry them. The two are told apart below, in
[The mechanisms the tree carries](#the-mechanisms-the-tree-carries) and
[Where the tree's v2.1 is KeySharp's](#where-the-trees-v21-is-keysharps-not-upstreams),
each with its source.

- [The gate](#the-gate)
- [Where the official documentation lives](#where-the-official-documentation-lives)
- [The boundary: what the v2.0 already has](#the-boundary-what-the-v20-already-has)
- [The mechanisms the tree carries](#the-mechanisms-the-tree-carries)
- [What the official v2.1 adds and the tree does not carry yet](#what-the-official-v21-adds-and-the-tree-does-not-carry-yet)
- [Where the tree's v2.1 is KeySharp's, not upstream's](#where-the-trees-v21-is-keysharps-not-upstreams)
- [The measurements](#the-measurements)
- [Where this rests](#where-this-rests)

---

## The gate

| Command | What it carries |
|---|---|
| `ahk` / `ahk --dialect=v2` | the v2.0.31 -- the default |
| `ahk --dialect=v2.1` | the v2.0.31 and the mechanisms of the v2.1 |
| `ahk --dialect=ks` | the mechanisms of the v2.1 and the extensions of KeySharp |
| `ahk --dialect=ks20` | the extensions of KeySharp without the mechanisms of the v2.1 |

The two switches are apart from each other: `extended` gates the
mechanisms, `keysharp` gates the names, and `ks20` is the proof -- the
names of KeySharp are there, and `z ??= 5` is refused with a syntax error
exactly as in the v2. The dialect is one struct, `V2`, with the two flags
(`crates/ahk/src/dialect.rs`).

**The `#Requires` line checks the gate; it does not select it.** The
version the comparison reads is the dialect's own: 2.0.32 for the v2,
2.1.0 for the v2.1 and the ks, 1.1.37.02 for the v1. Measured:

| The line | v2 | v2.1 / ks |
|---|---|---|
| `#Requires AutoHotkey v2.1` | refused: `This script requires AutoHotkey v2.1.` | met |
| `#Requires AutoHotkey v2.0.31` | met | met (2.1.0 is later, same major) |
| `#Requires v2` | met | met -- every v2-family dialect answers to the name `v2` |
| `#Requires v2.1` | refused | refused -- the dialect names are names, not sub-versions |

The forms and the operators of the line are in the language guide,
[`#Requires`: the check, not the gate](language-guide.md#requires-the-check-not-the-gate).
The official v2.1 gives the line a second job -- the **compatibility mode**
-- which the tree does not carry yet; see below.

**What the gate does not change.** The surface is the table of the v2.0.31
either way (`ahk --surface --dialect=v2.1 --json` answers the same 843
entries, and the `dialect` field of the report names them all `v2`).
`A_AhkVersion` answers `2.0.31` in every dialect: it is the constant of
`variables.rs`, not the version of the dialect -- the version of the
dialect is what `#Requires` reads. The runner, the report, the guard rails
and the desktop are the same.

## Where the official documentation lives

The v2.1 documentation is not on the website beside the v2's -- it is the
`alpha` branch of the documentation repository. The pages this document
rests on:

| The page | What it holds |
|---|---|
| `docs/v2.1-changes.htm` | the overview: high impact, compatibility mode, low impact, backward-compatible additions |
| `docs/ChangeLog.htm` | the alpha releases, alpha.1 to alpha.39, with the version tags of each change |
| `docs/Language.htm` | the unset expression: what may be unset, and where |
| `docs/Variables.htm` | the operator table: `var?`, `??`, `??=`, `!~=`, `&` |
| `docs/Functions.htm` | the unset parameter, the mandatory after the optional, the function definition expression, the default return |
| `docs/Objects.htm`, `docs/lib/Class.htm` | the parameterized property, the `Class.Call` of the dynamic class |
| `docs/Structs.htm` | the typed property, the struct class, the numeric types, the pointer and the array |
| `docs/Modules.htm`, `docs/lib/_Module.htm`, `docs/lib/_Import.htm` | the module, the import, the search path |
| `docs/lib/_Requires.htm` | the compatibility mode |
| `docs/lib/IsSet.htm`, `docs/lib/Type.htm`, `docs/lib/Props.htm`, `docs/lib/PropRef.htm` | the names the v2.1 changes or adds |
| `docs/lib/Until.htm`, `docs/lib/While.htm` | the `Until` of a `Loop` and a `For` -- not of a `while` |
| `source/defines.h`, `source/script.cpp`, `source/script_expression.cpp` (the interpreter repo) | the parser and the evaluator, where the documentation is quiet |

## The boundary: what the v2.0 already has

These are not v2.1 additions, and the tree carries them in the default
dialect -- measured on the v2 the same day:

| The form | The v2 answers |
|---|---|
| the `unset` keyword, `x := unset` | the variable loses its value (`IsSet(x)` is 0) |
| `var?` -- the maybe of a variable | `F(x?)` passes the value or omits the parameter |
| `x ?? 5` -- the or-maybe of a variable | 5 when `x` has no value |
| `f(a?)`, `f(a := unset)` -- the unset parameter | `IsSet(a)` is 0 when the caller omits it |
| `[unset]` -- the unset element of an array literal | `a.Length` is 1 |
| `o.Prop[args]` -- the parameterized property | the get and the set with parameters |
| `Loop { } Until cond`, `for ... Until cond` | the condition is read after each turn, even a turn `Continue` cut short |
| `IsSet(var)`, `IsSetRef(&var)` | the question of the variable |

The v2.1 delta is what comes next: the maybe of a **property or a call**,
the **chain**, the `??` of **any expression**, the `??=`, the **return**
of unset, and the rest.

## The mechanisms the tree carries

Each one is refused in the v2.0.31 and read behind the gate. The examples
are the measured answers of the runner (`scripts/probe-v2.1.py`).

### The optional chain: `?.`, `?[`, `?(`

The `?` glued to the access asks whether the left side has a value; when
it has none, the whole chain is the absent value, and the steps after it
are not read. Official: `var?` extended to a property, a method or a call
in alpha.2, the chain in alpha.2, the short-circuit of most operators in
alpha.29, the chain on the left of an assignment in alpha.29, the
short-circuit from a property-name substitution in alpha.34.

```ahk
x := unset
MsgBox (x?.a is unset)      ; 1 -- the v2 refuses the ?. at the load
MsgBox (x?[1] is unset)     ; 1
F(a?) {
    return a
}
MsgBox (F()?.z is unset)    ; 1 -- the call answers the absent value
MsgBox ((x?) is unset)      ; 1 -- the postfix ? on a value
```

The rules the tree keeps, measured:

- **Only the access the mark follows is lenient.** `h.Prop?.Inner` is the
  absent value when `Prop` returns unset; `h.Missing.Inner?.Value` is the
  `PropertyError` of the access *before* the mark.
- **The chain does not cross a parenthesis.** `(h?.Prop)?.Inner` is an
  `UnsetError`: the group has no value, and the access after it is not the
  one the mark tolerates. A new chain may start inside, as
  `(w ?? x?.y)?.z`.
- **The absent value propagates through the operators, and the other
  operand is not read.** `x?.v + Bump()` with `x` unset answers the absent
  value and does not call `Bump()`; the unary and the ternary are the same.
- **A write through the chain is skipped when the left has no value**, the
  value expression included; when it has one, the write lands:
  `o?.a.b := 7` writes 7.
- **The `%name?%`** marks a built name as one that may be absent:
  `o.%n?%` with `n` unset reads nothing and evaluates no parameter.
- **A final `?` omits the parameter**: `F((x?.y)?)` passes nothing when
  the chain is absent.

The gap beside it: the **implicit maybe of the `??`** -- upstream reads
`x.y ?? z` as `(x.y?) ?? z`, permitting the property to be absent -- is not
carried: `o.missing ?? "d"` is the `PropertyError` of the read. The chain
left and the call left of the `??` work.

### The or-maybe of an expression: `??`

The v2.0.31 takes only a variable (or a built name) on the left of the
`??`; the v2.1 takes any unset expression (alpha.2). Measured: `F() ?? "none"`
with `F` returning unset answers `none`; `(1 + 1) ?? 9` answers 2. The
official rule of the left side: if it is a property, a method or a call
that is not part of a chain, the effect is the same as if the maybe was
used -- the tree carries the call and the chain, not the bare property
(above).

### The maybe-assign: `??=`

`x ??= y` is the shorthand of `x := x ?? y` (alpha.2): `y` is evaluated
and written only when `x` has no value. The lexer is where the difference
lives -- the v2.0.31 reads the two signs `??` and `=` and refuses the line
at the load. Measured:

```ahk
z := unset
z ??= 5        ; z is 5
z ??= 7        ; z stays 5
a := [unset]
a[1] ??= 9     ; the item is 9
```

The targets carried: the variable and the array/map item. The gap:
`o.a ??= 5` on a **missing property** is the `PropertyError` of the read
-- upstream initializes the property; the tree does not. On a property
that is there, `o.a ??= 5` keeps the value.

### The absent value returned: `return unset`

A function may return the absent value (alpha.2), and the caller asks
about it with `IsSet` or handles it with `??`. Measured: `F() { return unset }`
and `F() is unset` is 1 behind the gate; the v2 refuses the `return unset`
at the load. The object literal keeps the absent property,
`{prop: unset}` -- `ObjOwnPropCount` is 1 behind the gate and 0 in the v2
-- which is the KeySharp reading, not the upstream's (below).

What is **not** carried: the default return of the v2.1 mode. Upstream, a
function that reaches its closing brace returns unset in v2.1 mode
(alpha.28/29); the tree's `F() { }` returns `""` in every dialect, and the
arrow tail call `G() => F()` does not propagate the blank-unset.

### The `IsSet` of an expression

Upstream alpha.22 lets `IsSet` take any unset expression, and alpha.37
makes it imply a final `?` when none is used. Measured behind the gate:

```ahk
o := {}
MsgBox IsSet(o.missing)    ; 0 -- the v2 throws the PropertyError
x := unset
MsgBox IsSet(x?.a)         ; 0
MsgBox IsSet(F())          ; the call is asked, not a variable
```

The holder is still read: `IsSet(h.Missing.Inner)` is the `PropertyError`
of `h.Missing`, because the expression that holds the member is evaluated.

### The forms of the KeySharp corpus

These are carried behind the gate, and they are **KeySharp's v2.1, not
the upstream's** -- the evidence is in
[the section below](#where-the-trees-v21-is-keysharps-not-upstreams):

- **The `Until` of a `while`.** `while true { x++ } Until (A_Index == 5)`
  leaves `x` at six; the body runs before the condition is read on every
  turn. The v2.0.31 refuses the word with `Unexpected "Until"`.
- **`x is unset`.** The `is` answers 1 when the left has no value, and the
  left is not read as a value of its own. The v2 refuses it at the load.
- **`x = unset`, `x != unset`.** The comparison asks whether the other
  side has a value; a name that has none is the answer, not a failure.
- **`{prop: unset}` keeps the property** (above).

## What the official v2.1 adds and the tree does not carry yet

The official additions the gate does not read, with the alpha that added
each and the refusal the runner gives today. The list is the diff of the
`v2.1-changes.htm` page against the tree; the names the KeySharp
extensions already carry are marked **ks**.

### The syntax and the core

| The addition | Alpha | The tree answers |
|---|---|---|
| `a !~= b` -- the not-RegExMatch | 21 | `Missing ")"` at the load |
| the mandatory parameter to the right of the optional, `F(a?, b)` | 18 | `Parameter default required.` |
| the function definition expression -- `SetTimer(tick() { ... }, 1000)`, the anonymous `() { ... }`, the immediate call | 3 | `Missing ")"` / `Syntax error.` |
| the load-time validation of `unset`, `?` and `??` | 2+ | partial -- the chain and the `??` are checked, the rest is not |
| the length limit of a parameter default string removed | -- | not measured |

### The compatibility mode and the default return

The `#Requires` of the official v2.1 (alpha.29, refined in alpha.34) sets
a **mode** per module and per function: a requirement that no 2.0.x can
meet turns the v2.1 mode on, and the mode decides the default return and
the absent-item behavior. The tree's `#Requires` checks the version and
stops there -- there is no mode, and the v2.1 dialect keeps the v2.0
semantics of the return and the item:

| The behavior of the v2.1 mode | Alpha | The tree answers |
|---|---|---|
| the function that reaches its brace returns unset (when any `return expr` is present) | 28/29 | `""` -- `F() { } is unset` is 0 |
| the blank-unset: `""` in v2.0 mode, unset in v2.1 mode, of the functions with no documented return | 29 | `""` everywhere |
| the arrow tail call propagates the blank-unset | 29 | does not |
| `Array.__Item`/`Get`, `Map.__Item`/`Delete`/`Get`, `Gui.__Item`, `Object.GetMethod`/`GetOwnPropDesc` return unset instead of throwing | 29 | still throws: `Map.Get` of an absent key is `Item has no value.`, `Array.Get` of an absent index is `Invalid index.` |
| `Type(unset)` answers `"unset"`; `Type()` with no parameter | 22 | `Type(x?)` answers `String`; `Type()` is `Too few parameters passed to function.` |

### The objects

| The addition | Alpha | The tree answers |
|---|---|---|
| `Props(Value)` and `Any.Prototype.Props()` -- enumerate own and inherited properties | 18 | `This global variable has not been assigned a value.` / `no method named "Props"` |
| the virtual reference: `&x.y` calls `x.__Ref('y')`; `PropRef`; `VarRef`/`ComValueRef.__Value`; the `&` of an assignable built-in | 10 | `"&" requires a variable.`; the `PropRef` class exists in the ks, but `%r%` of one is `Expected a String or VarRef but got a PropRef.` |
| the `DefineProp` function (the method is carried) | 22 | `This global variable has not been assigned a value.` |
| `Class.Call(Name, BaseClass, Args*)` -- the dynamic class, the prototype based on a native type | 3 | `Too many parameters passed to function.` |
| `Enumerable: false` hides a property from `Props` and the debugger | 34 | not carried (the `Props` is not) |

### The structs

The typed property is the largest piece of the official v2.1, and it is
**in progress** in the tree: the parsing landed in `be870d6f` (the `struct`
word is soft -- `struct := 5` still assigns in the v2 -- the fields are
`name : Type` with an optional `:= initializer`, several per line, and the
class defaults to extending `Struct`), and the runtime follows. Measured
on the build of the day: `struct POINT { x : Int32, y : Int32 }` parses
behind the gate, and `POINT()` is `Unknown class.` -- the `Struct` base is
not there yet.

| The piece | Alpha | The state |
|---|---|---|
| the `struct` definition, the typed field | 22 | parsed, gated; the runtime pending |
| the numeric struct classes `Int8`..`Int64`, `UInt8`..`UInt32`, `IntPtr`, `Float32`, `Float64` | 23 | carried as **ks** names (the typed numbers of KeySharp), not as the struct classes of the v2.1 |
| `Struct.Ptr` -- the pointer class; the boxed pointer of `StructClass.At(addr)` | 22 | pending (the ks has `ObjCreateWithDataPtr` and the `Struct.At` of the memory view) |
| `Struct.Array`, `Int32[10]` -- the structured array | 23 | pending |
| `#StructPack`, the `pack` of `DefineProp` | 19 | `This line does not contain a recognized action.` |
| the union block | 34 | pending |
| the type specifier expression | 30 | pending |
| the open structure (the layout closes on the first instance) | 38 | pending |
| `__Value` -- the virtual reference of the boxed value | 23 | pending |
| `ObjCreateWithDataPtr`; `ObjGetDataPtr` returns unset | 36/38 | the ks carries the names; the v2.1 does not |
| `Struct.Prototype.__New(fieldValues*)` | 37 | pending |

The corpus of the upstream struct tests is under `scripts/v21-corpus/`.

### The modules

| The addition | Alpha | The tree answers |
|---|---|---|
| `#Module`, `#Import`, the module namespace and body | 11 | `This line does not contain a recognized action.` |
| the implicit `AHK` module; the built-in reachable as `AHK.MsgBox` | 11 | not carried |
| the `Module` class; the alias; the wildcard and the `as` | 11+ | not carried |
| the execution on the first reference | 21 | not carried |
| the search path (`ModuleName`, `ModuleName\__Init.ahk`, `ModuleName.ahk`) | 35 | not carried |
| `&MyMod.MyVar` (`Module.__Ref`) | 33 | not carried |
| `#Include <A_B>` checks `A_B` before `A_B.ahk`, never `A.ahk` | 35 | not measured |

### The names

The v2.1 dialect reads the table of the v2.0.31, so a name the table does
not have is a name the gate does not have. The official v2.1 adds:

| The name | Kind | The state |
|---|---|---|
| `ATan2` | function | **ks** only -- the v2.1 answers `This global variable has not been assigned a value.` |
| `WinGetAlwaysOnTop`, `WinGetEnabled` | function | **ks** only, the same refusal |
| `A_HotIf`, `A_KeybdHookInstalled`, `A_MouseHookInstalled` | variable | **ks** only |
| `Throw` as a function | function | a statement of the tree; the function form is not carried |
| `Error.Show` | method | not in the table of the v2.0.31 |
| `Gui.OnMessage`, `Gui.SetCue` (Edit, ComboBox), `Gui.FontHandle` | method/property | not in the table |
| `Menu.Show` parameter `Wait` (#3), `Menu.Add` option `RTL` | parameter/option | not in the table |
| `HotIf` returns the previous `A_HotIf`; `Menu.ToggleCheck`/`ToggleEnable` return the new state | return | not measured |
| `RegExReplace` callback replacement; `CallbackCreate` type array; `DllCall` `CDecl` omission; `Edit` `Filename`; `FileCreateShortcut` `ShortcutKey` modifiers; `ImageSearch`/`PixelSearch` X/Y omission; `TraySetIcon` updates the main window | parameter | not measured -- the table of the v2.0.31 is the contract |

### The rest of the official page

- **High impact:** the modeless menus (the tray menu and `Menu.Show` no
  longer stall the thread), and the **DPI awareness default of
  per-monitor v2** with the automatic `DPIResize`. Both are the desktop's
  matter, not the language crate's; the Windows backend is the place they
  land ([windows-backend.md](windows-backend.md)).
- **Low impact:** `MouseGetPos` throws `OSError` when the position cannot
  be determined; `RegExReplace` no longer truncates at the first null;
  `ListVars` shows the unset vars; `#Warn` with no parameter turns on
  `VarUnset` and `Unreachable` and off `LocalSameAsGlobal` for the module;
  the UTF-8 decoding warning; a `WinTitle` with two `ahk_id` values yields
  no match; the ASCII-only case insensitivity of the `Tab` and the `Menu`;
  the class property setter accepts an unset `value`; `ahk_class` is
  case-insensitive outside the RegEx mode. The setter one is a defect of
  the tree today (below); the rest are not measured.
- **The keyboard:** the dead-key handling, the `InputHook` `H` option,
  `{Blind>^}` and the left/right modifiers of the remap, the wheel remap,
  the hotstring that avoids retyping the identical leading part. The
  desktop's matter.
- **The debugger and the optimizations:** the DBGp improvements, the
  `Pause`/`RunWait`/`ProcessWaitClose` that sleep instead of polling. Not
  the language crate's matter.

## Where the tree's v2.1 is KeySharp's, not upstream's

The tree measured these on the test suite of KeySharp (the frozen corpus
under `egm-review/keysharp-audit/corpus/scripts/`) and gated them with the
v2.1. The upstream documentation and the upstream source say otherwise;
the evidence is beside each. They are carried because the `ks` dialect
needs them and the gate is shared -- a script that wants the exact
upstream v2.1 must know that these four forms are not it.

| The form | The tree | The upstream | The evidence |
|---|---|---|---|
| `while ... Until` | carried behind the gate | **refused** -- the `Until` attaches to a `Loop` or a `For`, not a `while` | `source/defines.h`: `ACT_IS_LOOP_EXCLUDING_WHILE(ActionType)` is `ActionType >= ACT_LOOP && ActionType <= ACT_FOR`; `source/script.cpp`: `case ACT_UNTIL: expected = ACT_IS_LOOP_EXCLUDING_WHILE(parent_act);`; `Until.htm`: "Applies a condition to the continuation of a Loop or For-loop" |
| `x is unset` | carried behind the gate | **refused** -- the right of the `is` must be a class; an unset right operand aborts | `source/script_expression.cpp`: the right `SYM_MISSING` is permitted only by `??`, `,`, `?` and `:=`; the `SYM_IS` evaluation wants an object with a prototype, else the `Class` type mismatch |
| `x = unset`, `x != unset` | carried behind the gate | **refused** -- the same gate of the unset right operand | the same `SYM_MISSING` check |
| `{prop: unset}` keeps the property | kept behind the gate (count 1); the v2 drops it (count 0) | **ignored** -- the unset property of a literal is not defined | `Language.htm`, the unset expression contexts: "On the right-hand side of : in an object literal, where unset properties are ignored" |

And three seams of the tree itself, measured in **both** dialects, that a
script of the official v2.1 would not expect:

- **`o.a := unset` keeps the property** (the count stays 1, and the read
  throws `UnsetError`), and **`m["k"] := unset` keeps the key** (`Has` is
  1). Upstream deletes the value property and the map element
  (`Language.htm`: "unsetting a map element or value property deletes it
  from the object"). The tree keeps the place with the absent value in it.
- **The setter of a class property refuses the unset write.**
  `c.p := unset` with a `set` accessor is `This Class cannot be used as an
  output variable.` -- a defect; upstream lets the setter's `value`
  parameter be unset (the low-impact list of the changes page).
- **The `VarUnset` warning names the `unset` keyword.** `x is unset`
  behind the gate answers right and warns "This global variable appears
  to never be assigned a value. Specifically: unset" -- the warning reads
  the keyword as a name.

## The measurements

The instrument is `scripts/probe-v2.1.py`:

```sh
python scripts/probe-v2.1.py --ahk target/debug/ahk.exe
```

Each case is a script of its own, because a load error of one case must
not hide the answers of the others; the runner reads it on stdin and the
line beside each case is the output of the run or the first line of the
refusal. The answers quoted in this document were measured on the debug build of the day (built 10:03, before the
struct-parsing commit `be870d6f` of 10:06 -- the struct row above cites
the commit, not the build). Two cautions of the day:

- The **release build of 08:42 was stale**: it hung on `x := 5` then
  `MsgBox (x is unset)`, which the debug build answers 0. Rebuild before
  measuring with the release.
- The debug build is slow under load; a case that answers TIMEOUT is a
  case to re-run with a bigger budget, not a refusal of the language. The
  `Loop ... Until` cases answer 32 with `--timeout=6`.

The summary of the gate (the full table is the instrument's output):

| The case | v2 | v2.1 |
|---|---|---|
| `z := unset`, `z ??= 5` | `Syntax error.` | 5 |
| `x?.a`, `x?[1]`, `F()?.z`, `(x?)` | `Syntax error.` | the absent value |
| `o.%n?%` with `n` unset | `Missing ending "%"` | the absent value |
| `F() ?? "none"`, `F` returning unset | `Syntax error.` | `none` |
| `o.missing ?? "d"` | `Syntax error.` | `PropertyError` (the implicit maybe is not carried) |
| `o.a ??= 5`, `a` missing | `Syntax error.` | `PropertyError` (the property is not initialized) |
| `return unset` | `Syntax error.` | the absent value |
| `IsSet(o.missing)` | `PropertyError` | 0 |
| `x is unset`, `x = unset` | `Syntax error.` / `UnsetError` | 1 |
| `while true { x++ } Until (A_Index == 5)` | `Unexpected "Until"` | 6 |
| `Loop { x *= 2 } Until x > y` | 32 | 32 (the v2.0 already has it) |
| `{prop: unset}` count | 0 | 1 |
| `F() { }` returns | `""` | `""` (the v2.1 mode is not carried) |
| `Type(x?)` | `String` | `String` (upstream: `"unset"`) |
| `!~=`, `F(a?, b)`, the function definition expression | refused | refused (not carried) |
| `#Module`, `#Import`, `#StructPack` | -- | `This line does not contain a recognized action.` |
| `struct POINT { x : Int32 }` then `POINT()` | -- | parses; `Unknown class.` (the runtime pending) |
| `ATan2`, `WinGetEnabled`, `A_HotIf` | not a name | not a name (the ks carries them) |
| `A_AhkVersion` | `2.0.31` | `2.0.31` |

## Where this rests

| What | Where |
|---|---|
| the gate: the `V2` struct, the `extended` and `keysharp` flags, the trait methods of the mechanisms | `crates/ahk/src/dialect.rs` (`the_chain_may_be_skipped`, `the_lexer_knows_the_null_assignment`, `the_absent_value_may_be_the_class_of_the_is`, `an_expression_may_be_the_left_of_the_maybe`, `a_while_may_carry_an_until`) |
| the `??=` token | `crates/ahk/src/lexer.rs` |
| the chain, the `%name?%`, the struct definition | `crates/ahk/src/parser.rs`, `ast.rs` |
| the absent value: the propagation, the skipped write, the `IsSet` of an expression, the comparison with unset | `crates/ahk/src/runtime.rs` |
| the version of the dialect (the `#Requires` reads it) | `dialect.rs` (`version`); the constant of `A_AhkVersion` is `variables.rs` (`AHK_VERSION`) |
| the cases of the mechanisms | `crates/ahk/tests/language.rs` (the tests that run in `ahk::dialect::v2_1()`) |
| the KeySharp corpus the forms were measured on | `egm-review/keysharp-audit/corpus/scripts/` (`op-null-assign.ahk`, `assign-unset.ahk`, `flow-until.ahk`, `class-ownprops.ahk`) |
| the names of the extensions (the ks) | `crates/ahk/src/extensions.rs`, `crates/ahk/reference/ahk-ks.surface.tsv` |
| the instrument of the measurements | `scripts/probe-v2.1.py` |
| the upstream struct corpus | `scripts/v21-corpus/` |
| the guide of the language, the gate and the `#Requires` | [language-guide.md](language-guide.md) |
| the extensions of KeySharp | [ahk-ks-dialect.md](ahk-ks-dialect.md) |
| the gate between the v1 and the v2 | [dialects.md](dialects.md) |
