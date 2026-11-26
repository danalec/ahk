# The AHK_H: what the two forks add, and the plan to carry them

> **Measured.** The two releases were read out of
> `../ahkdll-v1-release` and `../ahkdll-v2-release` (the release repos of
> HotKeyIt: `github.com/HotKeyIt/ahkdll-v1-release` at `99db51a`,
> `github.com/HotKeyIt/ahkdll-v2-release` at `3eb870c`). The feature lists
> come out of the CHMs of the releases — `AutoHotkey_H.chm`, pages
> `AHKH_Features.htm` and `AHKH_ChangeLog.htm` (445 pages), and
> `AutoHotkey_H v2.chm`, page `AutoHotkey.htm` with the 107 command pages
> under `docs/commands/` (117 pages) — and the export tables come out of
> the PE export directories of `x64w/AutoHotkey.dll` and
> `x64w/AutoHotkey.exe` of each release. The `HASH` file of each release
> pins the binaries. The state of the tree is the head of `master`
> (`8e8066ae`): **no name of either fork is carried today** — a search of
> `crates/` for the names below answers nothing.

The two releases are **AutoHotkey_H**, the fork of HotKeyIt:

| Release | Base | What it is |
|---|---|---|
| `ahkdll-v1-release` | AutoHotkey **v1.1.37.02** merged into AutoHotkey_H 1.1.33.10 | the v1 with the H additions; the same base the tree's v1 dialect already targets |
| `ahkdll-v2-release` | **AHK_H 2.0-beta** (the v2 line, beta base) | the v2 with the H additions; the base is *older* than the tree's v2.0.31 |

Both are **GPL-2.0** (`LICENSE` of each release; the v2 CHM says the
same). This tree is Apache-2.0. The rule is the one of
[ahk-x11-comparison.md](ahk-x11-comparison.md): **the behaviour may be
ported, the code may not.** Everything below is a clean-room port from
the documentation and from measurement on the binaries — no source of
the fork is read into this tree. MemoryModule inside the fork is MPL-2.0
(Joachim Bauch) and the 7-Zip module is LGPL: neither is copied or
embedded either.

- [The comparison](#the-comparison)
  - [The families](#the-families)
  - [The names both forks add](#the-names-both-forks-add)
  - [What only the v1-H adds](#what-only-the-v1-h-adds)
  - [What only the v2-H adds](#what-only-the-v2-h-adds)
  - [The export model](#the-export-model)
- [What the tree already has to build on](#what-the-tree-already-has-to-build-on)
- [The guardrails](#the-guardrails)
- [The gate](#the-gate)
- [The contract and the instruments](#the-contract-and-the-instruments)
- [The phases](#the-phases)
- [The decisions that are not the writer's](#the-decisions-that-are-not-the-writers)
- [What stays out](#what-stays-out)

---

## The comparison

### The families

| Family | v1-H | v2-H | The tree today |
|---|---|---|---|
| Multi-threading | `AhkThread`, `AhkExported`, dll-per-thread | `NewThread`, `ThreadObj`, in-process threads | worker machinery exists (KeySharp `RealThread`), no H name |
| Thread sharing | `Alias`, `GetVar`, `CriticalObject`, `CriticalSection`, `ObjShare` | the same | `VarRef` exists; none of the five |
| Exported control | `ahkdll`…`ahkExecuteLine` (18 exports) | `NewThread` + 12 thread-id exports | none |
| COM embedding | `AutoHotkey.Script` (4 GUIDs) | `AutoHotkey2.Application` (1 GUID) | COM *client* only |
| Structures | `Struct`, `sizeof`, Windows type table | the same | v2.1 struct *classes* exist — a different mechanism |
| Native calls | `DynaCall`, `#DllImport`, `WinApi`, `MCodeH`, `ComObjDll` | the same | `DllCall` marshalling + `Native` trait exist |
| MemoryModule | 8 functions | 7 functions | none |
| Resources | `ResGet`…`ResDllCreate`, resource LIB, compressed resources | the same | none |
| Compression | Zip family (14), `SevenZip` | Zip family (13), `SevenZip` | none |
| Encryption / persistence | `CryptAES`, `ObjDump`, `ObjLoad` | the same | none |
| Process from memory | `BinRun`, `DynaRun`, `CreateScript` | the same | `run_with_stdin` exists (the pipe of `DynaRun`) |
| Utility functions | ~30 | ~35 (adds `Cast`, `Swap`, `Between`, `StrBuf`, `JSON`, `GetProcessParameter`, `WinGetPIDList`) | none of them |
| Object features | — | `DefineDefault`, `UMap`/`UArray`/`UObject`, `Map(obj)`, `#MapCaseSense` | none |
| Syntax mechanisms | `Send {N}`, `ahk_parent`, `&"String"`, `#NoEnv` default | `Send {N}`, `ahk_parent`, `macro`, InputHook `A`, ComValue truthiness, hotstring end char | none |
| GUI | auto-position `AX`/`AW`…, `Scroll`/`VScroll`/`HScroll` | the same | GUI backend exists, options not carried |
| Directives | 4 | 12 | none of them |
| Built-in variables | 10 | 12 | none of them |
| v1 commands in the v2 | — | `Progress`, `SplashImage`, `SplashTextOn/Off`, `EnvUpdate` | carried in the v1 dialect only |

### The names both forks add

**Directives.** `#DllImport`, `#WindowClassMain`, `#WindowClassGui`.

**Threads and sharing.** `CriticalObject`, `CriticalSection`,
`ObjShare`, `Alias`, `GetVar`, `CreateScript`, `DynaRun`, `BinRun`.
With them the script calls `EnterCriticalSection`,
`LeaveCriticalSection`, `TryEnterCriticalSection` (WinApi names the
fork makes reachable as built-ins).

**Exported control, callable as functions of the script.** `addScript`,
`ahkExec`, `ahkassign`, `ahkgetvar`, `ahkFunction`, `ahkPostFunction`,
`ahkLabel`, `ahkPause`, `ahkFindFunc`, `ahkFindLabel`, `ahkExecuteLine`
(the v1 adds `addFile`, `ahkdll`, `ahktextdll`, `ahkReady`, `ahkReload`,
`ahkTerminate` to the same list; in the v2 those six live on the thread
object and the dll).

**Structures and native calls.** `Struct`, `sizeof`, `DynaCall`,
`WinApi`, `MCodeH`, `ComObjDll`, `FindFunc`, `FindLabel`.

**MemoryModule.** `MemoryLoadLibrary`, `MemoryGetProcAddress`,
`MemoryFreeLibrary`, `MemoryFindResource`, `MemorySizeOfResource`,
`MemoryLoadResource`, `MemoryLoadString` (the v1 adds
`ResourceLoadLibrary`).

**Resources.** `ResGet`, `ResExist`, `ResPut`, `ResPutFile`,
`ResDelete`, `ResDllCreate`.

**Compression.** `ZipCreateFile`, `ZipAddFile`, `ZipAddFolder`,
`ZipCloseFile`, `ZipCreateBuffer`, `ZipAddBuffer`, `ZipCloseBuffer`,
`UnZip`, `UnZipBuffer`, `ZipInfo`, `ZipOptions`, `ZipRawMemory`,
`UnZipRawMemory`, `SevenZip` (the v1 adds `ZipCompressionLevel()`; the
v2 carries the level as `A_ZipCompressionLevel`).

**Encryption and persistence.** `CryptAES`, `ObjDump`, `ObjLoad`.

**Utility.** `BinToHex`, `HexToBin`, `ToChar`, `ToUChar`, `ToShort`,
`ToUShort`, `ToInt`, `ToUInt`, `HIBYTE`, `LOBYTE`, `HIWORD`, `LOWORD`,
`MAKEWORD`, `MAKELONG`, `MAKEWPARAM`, `MAKELPARAM`, `MAKELRESULT`,
`MAKELANGID`, `MAKELCID`, `ObjByRef`, `FileReplace`, `IsBOM`,
`DirGetParent`, `ErrorMessage`, `GetEnv`.

**Syntax.** `Send 123{100}456` — a number in braces sleeps that many
milliseconds mid-send; `ahk_parent <id>` in a WinTitle — identifies a
window by its parent, in every `Win…` and `Control…` function.

**GUI.** Auto-position and auto-size of controls on resize: `AX`, `AY`,
`AW`, `AH`, `AXP`, `AYP`, `AXA`, `AYA`, `AWA`, `AHA`, `AXR`, `AYR`,
`AXR`, `AWR`, `AYR`, `AHR` — each with an optional decimal or fraction
(`ax1/2`), negative allowed, default 1, relative to the initial
`Gui Show` size, never shrinking below the initial control size; and
the `Scroll`, `VScroll`, `HScroll` window options.

**Variables.** `Null`/`NULL` (0, like `False`), `A_AhkDir`, `A_DllPath`,
`A_DllDir`, `A_IsDll`, `A_ModuleHandle`, `A_ScriptStruct`,
`A_GlobalStruct`.

### What only the v1-H adds

| Name | What it does |
|---|---|
| `AhkThread(ScriptOrFile, Params, Title, ScriptIsFile, DllToUse)` | a new script thread in a loaded `AutoHotkey.dll`; returns the control object; `0` prepares without starting |
| `AhkExported()` | the control object of the main exe, same methods |
| `ahkthread_free(obj)` | frees the thread's module (library function) |
| `ResourceLoadLibrary()` | loads a dll from a resource of the current exe |
| `ZipCompressionLevel()` | the zip level as a function |
| `StrPutVar()` | puts an encoded string into a variable (the v2 answers this with `StrBuf`) |
| `#WarnContinuableException` | warns on a continuable exception instead of dying |
| `#NoEnv` **default** | environment variables are not script variables unless `EnvGet`/`GetEnv` |
| `&"String"` | the address operator on a quoted literal: `StrGet(&"Text")` |
| `ListVars` | static variables shown separately from locals |
| `A_IsMini`, `A_MemoryModule` | the Mini dll flag; the MemoryModule handle |
| `AutoHotkeyMini.dll` | the lightweight build: no hotkeys, hooks, Gui, menus, InputHook; `A_IsMini` = 1 |
| `/Execute` (`/E`) | a compiled exe/dll runs an external script instead of the compiled one |
| `Lib.lnk` | auto-include folder the shortcut points at |
| Resource LIB + compressed resources | library functions in resources (`LIB\NAME.AHK`), zip-raw-compressed/encrypted resources decompressed on load |
| `AhkIsUnicode` export | 1 in the Unicode builds |

### What only the v2-H adds

| Name | What it does |
|---|---|
| `NewThread(Script, Params, Title, Wait, Library)` | a real thread **in the current process**, no dll; the control object with the `ahk…` methods; `Library` switches to a dll thread |
| `ThreadObj(Script, Params, Title)` | the same thread with property syntax: `thread.var`, `thread.var := v`, `.Call`, `.PostCall`, `.FuncPtr`, `.LabelPtr`, `.Exec`, `.AddScript`, `.ExitApp` |
| `JSON` class | `JSON.Parse(str)` → Map/Array; `JSON.Stringify(obj)` for any `__Enum` object |
| `Cast(Type, Value, NewType)` | reinterpret between `UInt/Int/Int64/Short/UShort/Char/UChar/Double/Float/Ptr/UPtr` |
| `Swap(Var1, Var2)` | swaps two normal variables (built-ins and aliases refused) |
| `Between(Var, Lo, Hi)` | inclusive numeric or alphabetic range test |
| `macro name(...)` | a function that runs in the **caller's scope**; parameters stay local; no globals/statics |
| `StrBuf(String, Encoding)` | a Buffer with the string encoded (default UTF-8) |
| `Object.DefineDefault([Value])` | the answer for a missing key/property instead of an error |
| `UMap()`, `UArray()`, `UObject()` | insertion-ordered (unsorted) collections |
| `Map(obj)` | a Map initialized from an Object/Map/Array |
| `#MapCaseSense On\|Off\|Locale` | the default case sensitivity of Map |
| `#DefineDefaultObjectValue` / `…ArrayValue` / `…MapValue` | the script-wide default value |
| `#TargetWindowError`, `#TargetControlError` | missing window/control does not throw (default Off) |
| `#UseStdLib` | auto-load a library function on an unset name: UserLib → resource LIB → WinApi |
| `#CriticalObjectSleepTime`, `#CriticalObjectTimeOut` | the spin sleep and the timeout of CriticalObject access |
| `GetProcessParameter(PID, 0\|1)` | the command line or the working directory of a process |
| `WinGetPIDList(WinTitle, …)` | Array of PIDs of matching windows; `[criteria, true]` includes hidden |
| `OnMessage(Msg, HWND, Fn, MaxThreads)` | per-HWND monitoring; omitting Fn with MaxThreads 0 deletes all for the pair |
| `Progress`, `SplashImage`, `SplashTextOn/Off`, `EnvUpdate` | the v1 commands, carried into the v2-H |
| `A_ArgsMap` | the parameters as a Map (`param1:param2`), beside `A_Args` the Array |
| `A_MainThreadID`, `A_ThreadID` | the thread ids |
| `A_ZipCompressionLevel` | the zip level, 0–9 |
| ComValue truthiness | `VT_BOOL`/`VT_EMPTY`/`VT_NULL` answer false in `if`/ternary (upstream: all true) |
| InputHook option `A` | append to the previous input / `.Input` text instead of discarding |
| Hotstring end char | the ending character is sent so other threads see it; hotstrings can chain |
| `#include <lib:url>` | dynamic include through a library function (e.g. `urldownloadtovar:http://…`) |
| `DoubleToString`/`StringToDouble` | the internal double conversion library (thqby) |

### The export model

Measured on the PE export directories of the `x64w` binaries
(the v1 exports every name in four casings — the canonical set is
listed):

| | v1-H dll | v1-H exe | v2-H dll | v2-H exe |
|---|---|---|---|---|
| start | `ahkdll`, `ahktextdll` | — | `NewThread` | `NewThread` |
| lifecycle | `ahkReady`, `ahkReload`, `ahkTerminate` | — | `ahkReady` | `ahkReady` |
| code | `addFile`, `addScript`, `ahkExec`, `ahkExecuteLine` | the same | `addScript`, `ahkExec`, `ahkExecuteLine` | the same |
| variables | `ahkassign`, `ahkgetvar` | the same | the same | the same |
| calls | `ahkFunction`, `ahkPostFunction`, `ahkLabel`, `ahkPause` | the same | the same | the same |
| lookup | `ahkFindFunc`, `ahkFindLabel` | the same | the same | the same |
| identity | `AhkIsUnicode` | — | `g_FirstThreadID`, `g_ThreadExitApp` (data) | the same |
| COM | `DllRegisterServer`, `DllUnregisterServer`, `DllGetClassObject`, `DllCanUnloadNow` | — | — | — |
| hooks | `MinHookEnable`, `MinHookDisable` | the same | the same | the same |

The **v1 model is one script per loaded dll instance**; the **v2 model
is one process, many threads, addressed by ThreadID** (the last
parameter of every export). The COM interfaces:

| Fork | ProgID | GUID |
|---|---|---|
| v1 | `AutoHotkey.Script` (last registered) | `{C00BCC8C-5A04-4392-870F-20AAE1B926B2}` |
| v1 | `AutoHotkey.Script.UNICODE` (32-bit) | `{C58DCD96-1D6F-4F85-B555-02B7F21F5CAF}` |
| v1 | `AutoHotkey.Script.ANSI` (32-bit) | `{974318D9-A5B2-4FE5-8AC4-33A0C9EBB8B5}` |
| v1 | `AutoHotkey.Script.X64` | `{38D00012-DC83-4E17-9BAD-D9DD97902580}` |
| v2 | `AutoHotkey2.Application` | `{1EB0A161-FE26-4C22-A15D-1B6C6F2677B9}` |

---

## What the tree already has to build on

| Seam | Where | What it gives the plan |
|---|---|---|
| the dialect trait + the extension mechanism | `crates/ahk/src/dialect.rs` (`the_extension_rows`, `the_builtin_of_the_extension`, `carries_the_variable_of_the_extension`, `carries_the_class_of_the_extension`), `extensions.rs` | the `ks` pattern: a gated table of names beside a pure base dialect — the H names ride the same mechanism |
| the worker threads | `desktop/mod.rs` (`create_worker_thread`, `worker_thread_state`, `worker_thread_list`, `ThreadValue`, `share_the_globals`), `classes/realthread.rs` | real OS threads that re-parse a source and run it; H5 extends them from *a function* to *a whole script* |
| the native marshalling | `dll.rs` (`NativeType`), `Native` trait (`call_native`, `call_native_at`, `load_library`, `create_callback`) | `DynaCall`, `#DllImport`, `WinApi`, `MCodeH` are bindings over this, not a second FFI |
| the buffer mirrors | `memory.rs` | stable `Ptr` blocks the machine reads and writes — the memory `Struct` lays out over |
| the v2.1 struct classes | `structs.rs` | the alignment rules and the numeric types; the H `Struct` is a *definition-string* mechanism and stays a module of its own beside it |
| the variable references | `classes/varref.rs`, the `&var` of the runtime | `Alias`, `GetVar`, `ObjByRef` are reference plumbing over this |
| the process pipes | `Processes::run_with_stdin`, `launch_with_stdin` | `DynaRun` is a launch with the script on the pipe |
| the surface contracts | `crates/ahk/reference/*.surface.tsv`, `surface.rs`, the parity test, `scripts/check-surface-docs.py` | the H contract is one more TSV and the same test |
| the differential harness | `scripts/differential.py` (engines `real`, `keysharp`, `rust`) | two more engines: `h1`, `h2`, pointed at the release binaries |
| the GUI backend | `gui_option`, `gui_control_pos`, `gui_size` and the resize path | the auto-position options are option parsing + a resize rule, not a new backend |

**The constraint that shapes the plan:** the language crate has **zero
dependencies** and builds without `std` (`thumbv7em-none-eabihf`). The
zip/AES/7z codecs and the PE loader cannot ride a crate from crates.io
without a decision (D3); the machine-shaped names answer with the
reason off-Windows, the way the rest of the tree does.

## The guardrails

1. **Clean-room.** GPL-2.0 behaviour is ported from the CHM and from
   measurement on the binaries; no code of the fork, of MemoryModule
   (MPL-2.0) or of 7-Zip (LGPL) enters the tree. The lib scripts of the
   releases (`lib/WinApiDef.ahk`, …) are *data to measure*, not code to
   copy: the WinApi signature table is re-derived and written original.
2. **The base dialects stay pure.** The surfaces of `v2`, `v1`, `v2.1`,
   `ks`, `ks20` do not move by one name. The H names live behind the H
   gates only (the guardrail 2 of the KeySharp audit, applied again).
3. **`no_std` keeps building.** Every phase ends with
   `cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`.
   A name that needs the machine refuses with the reason on a host that
   cannot answer it; it never answers `false`.
4. **No raw internal pointers.** `GetVar`, `FindFunc`, `FindLabel`,
   `A_ScriptStruct`, `A_GlobalStruct` answer opaque handles of this
   tree, not addresses of Rust structures (D2). A script of the fork
   that dereferences them with `Struct` is told the reason.
5. **The report stays a contract.** What a thread writes appears in the
   report the way D4 decides; no phase changes the shape silently.

## The gate

Two new dialects, beside the five that exist:

```sh
ahk --dialect=h1    # the v1.1.37.02 + the additions of the v1-H
ahk --dialect=h2    # the v2.0.31   + the additions of the v2-H
```

- `h1` is the `V1` dialect with the H flag; `h2` is the `V2` struct with
  the H flag — the same shape as `ks`/`ks20` (`dialect.rs`).
- `#Requires`: `h1` answers to the name `v1` and the version 1.1.37.02;
  `h2` to `v2` and 2.0.31. The H scripts carry no distinct requirement
  line (measured on the CHM examples); the gate is the flag, as with
  `ks`.
- **The base gap of the v2-H is recorded, not hidden:** the fork's v2
  is a 2.0-beta base, the tree's v2 is 2.0.31. The tree carries the H
  *additions* on top of its own 2.0.31; where the beta base answers
  differently, the differential harness names it and
  `egm-review/reports/` keeps the quirk (D5).
- The runner's help and the refusal of an unknown dialect name list
  `h1` and `h2` (`ahk-cli/src/main.rs`, `the_dialect`).

## The contract and the instruments

- **`crates/ahk/reference/ahk-h.surface.tsv`** — one row per name of
  this document: `name`, `kind` (command/function/variable/method/
  class/directive), `class`, `dialect` (`h1`, `h2`, `both`). Written by
  **`scripts/measure-ahk-h.py`**, which reads the extracted CHM pages
  and the export tables, so the contract is generated from the release
  and not typed by hand. The `HASH` files pin the releases it reads.
- **The parity test** asks the tree about every row, the way the ks
  table is asked: at H0 every name is *pending*; a phase moves its
  names to *carried* (a value, or the reason of the machine); a name
  that stops being answered fails the build of a test.
- **`scripts/probe-ahk-h.py`** — the per-phase probe, the shape of
  `probe-v2.1.py`: small scripts, the answer of the binary beside the
  answer of the tree, the date.
- **`scripts/differential.py`** gains the engines `h1` and `h2`
  (`AHK_H1` / `AHK_H2`, defaulting at the `x64w/AutoHotkey.exe` of the
  two release directories), cases under `scripts/differential/cases-h/`.
  The v1-H cases run on the v1 base the tree already matches; the v2-H
  cases carry the beta-base caveat of D5.
- **`scripts/check-surface-docs.py`** counts the H surface into the
  documents it checks.

---

## The phases

Each phase: the names it carries, the seam it touches, the measurement
that closes it. A phase lands only with its probe, its parity rows and
the `no_std` build green.

### H0 — the gate, the contract, the harness

No behaviour. The two dialect flags, the empty extension tables, the
generated `ahk-h.surface.tsv` with every name **pending**, the parity
test wired, the two differential engines wired, this document indexed
in `docs/README.md`.

*Exit:* `ahk --surface --dialect=h1 --json` and `…=h2` print the
contract; `cargo test --workspace` green with the pending rows; the
harness runs one trivial case (`MsgBox`/`FileAppend "*"`) in all three
engines and reports MATCH.

### H1 — the pure names (~45, both dialects)

The names that need no new mechanism:

- arithmetic/conversion: `Cast`, `Swap`, `Between`, `ToChar`, `ToUChar`,
  `ToShort`, `ToUShort`, `ToInt`, `ToUInt`, `HIBYTE`, `LOBYTE`,
  `HIWORD`, `LOWORD`, `MAKEWORD`, `MAKELONG`, `MAKEWPARAM`,
  `MAKELPARAM`, `MAKELRESULT`, `MAKELANGID`, `MAKELCID`;
- binary/text: `BinToHex`, `HexToBin`, `StrBuf` (h2) / `StrPutVar`
  (h1), `IsBOM`;
- files/env: `FileReplace`, `DirGetParent`, `GetEnv`, `ErrorMessage`;
- objects: `ObjByRef`, the `JSON` class (`Parse`/`Stringify`);
- variables: `Null`/`NULL`, `A_AhkDir`, `A_DllPath`, `A_DllDir`,
  `A_IsDll`;
- the small desktop additions: `WinGetPIDList`, `GetProcessParameter`,
  `OnMessage` with the HWND parameter (h2).

*Seams:* a new `crates/ahk/src/h/` module (the table + the pure
handlers), `builtins.rs` dispatch through
`the_builtin_of_the_extension`, `variables.rs` through
`carries_the_variable_of_the_extension`, `classes/` for `JSON`,
`desktop/mod.rs` + both backends for the three machine questions
(`env_all`, `process_command_line`, `process_working_dir`,
`window_pid_list`, the per-HWND message watch).

*Measurement:* the probe against both binaries — `Cast("float",1.4,"int")`,
`Swap` on an array and a buffer, `Between("blur","blue","red")`,
`JSON.Stringify({a:1,b:[2,3],c:Map(...)})`, `IsBOM(FileRead(...,"RAW"))`,
`BinToHex(StrPtr("123"),6)`. `ErrorMessage` and `GetProcessParameter`
are Windows-shaped: the reason off-Windows.

### H2 — the mechanisms of the objects, the syntax and the GUI

- **Objects (h2):** `DefineDefault` + `#DefineDefaultObjectValue` /
  `…ArrayValue` / `…MapValue`, `UMap`/`UArray`/`UObject`, `Map(obj)`,
  `#MapCaseSense`. The insertion-ordered collections are a flag on the
  existing Map/Array/Object storage, not a second collection.
- **Syntax:** the `macro` keyword (h2 — the caller's scope, parameters
  local, globals/statics refused); `Send {N}` mid-send sleep (both);
  `ahk_parent` in the window criteria (both — `desktop/criteria.rs`);
  `&"String"` (h1); InputHook option `A` (h2); the ComValue
  truthiness of `VT_BOOL`/`VT_EMPTY`/`VT_NULL` (h2); the hotstring
  end-character send (h2); `ListVars` statics shown apart (h1).
- **Directives:** `#TargetWindowError`, `#TargetControlError` (h2),
  `#WindowClassMain`, `#WindowClassGui` (both),
  `#WarnContinuableException` (h1), the `#NoEnv` default (h1).
- **GUI:** the auto-position/auto-size options (`AX`…`AHR`, decimals
  and fractions, the never-shrink rule, the previous-control rules)
  and `Scroll`/`VScroll`/`HScroll`.
- **The v1 commands in h2:** `Progress`, `SplashImage`,
  `SplashTextOn/Off`, `EnvUpdate` — the handlers of the v1 dialect,
  reachable from the h2 table.

*Seams:* `parser.rs` (macro, `&"literal"`, the directives),
`classes/map.rs`/`array.rs`/`mod.rs` (the default value, the order
flag, the case-sense default), `keys.rs` + the send path (the `{N}`
sleep), `desktop/criteria.rs` (`ahk_parent`), `classes/inputhook.rs`,
`classes/comvalue.rs`, `hotkeys.rs` (hotstrings), the GUI option
parser + the resize path of both backends, `v1/commands.rs` reused.

*Measurement:* the resize probe (a Gui shown `w200 h100`, a control
`ax1/2 aw0.5`, the window resized +100px → the control +50/+50, never
below the initial size), the macro scope probe, the `Send {N}` timing
probe, the `ahk_parent` probe with two windows of one class.

### H3 — `Struct`, `sizeof`, and the handles

- **`Struct(Definition, Memory, Init)`** — the definition-string
  grammar: the Windows type table (the `StructTypes` page of the CHM,
  re-derived), fields separated by `,` or `;`, the carried default
  type, arrays `T[n]`, pointers `*T`, nested unnamed `struct`/`union`,
  C comments, the `A_PtrSize`-sized types; the layout with alignment
  over the buffer mirrors; the methods `Clone`, `CountOf`, `Encoding`,
  `GetAddress`, `GetCapacity`, `IsPointer`, `Offset`, `SetCapacity`,
  `Size`; the evaluation order Structure → Array → Pointer → field.
- **`sizeof(Definition, offset)`** — the same layout engine, the
  padded total.
- **The handles:** `Alias` (a variable becomes a reference to another,
  across threads included), `GetVar` (the handle of a variable,
  `ResolveAlias`), `FindFunc`/`FindLabel` (opaque tokens),
  `A_ScriptStruct`/`A_GlobalStruct` (opaque, D2). `Struct` over a
  handle of this tree reads the fields the tree exposes; over a raw
  address of the fork's internals it refuses with the reason.

*Seams:* a new `crates/ahk/src/h/struct_def.rs` beside `structs.rs`
(the v2.1 classes stay as they are), `memory.rs` for the blocks,
`classes/varref.rs` + the runtime's variable table for `Alias`/
`GetVar`, the parser's function/label tables for the tokens.

*Measurement:* `POINT:="Int x,y"`, `pt:=Struct(POINT)`, `sizeof(pt)`
= 8; the union probe of the CHM; the nested/pointer/array probes;
`Alias(refvar,var)` both directions; the 1-byte-only union that used
to crash the fork (its changelog) answers a size, not a division by
zero.

### H4 — the native-call family

- **`DynaCall("dll\func", "definition", defaults…)`** — the callable
  object: the short-type mini-language (`i s a w h c f d t i6`, `u`
  prefix, `*`/`p`, `=` stdcall / `==` cdecl), the parameter-reorder
  array `["t=tuitt", 3]`, the defaults in the original order, the
  property-call syntax `Msg.NewText`, `base`/`__Class` reserved.
- **`#DllImport Name, dll\func, Type, Default, …, Cdecl RetType`** —
  the directive that binds a script function to a native call at load
  time; the hex machine-code form `[32-bit hex]:[64-bit hex]`.
- **`WinApi`** — the unknown-name resolution against the signature
  table (re-derived from `lib/WinApiDef.ahk` of the releases, written
  original): `IsWindowVisible(hwnd)` as if built-in.
- **`MCodeH(hex, definition, defaults…)`** — the DynaCall object over
  machine code: executable memory through a new `Native` method
  (`alloc_executable`), the bytes written once, freed with the object.
- **`ComObjDll(hModule, CLSID/ProgID, IID)`** — the unregistered COM
  dll: `DllGetClassObject` → `IClassFactory` → the object, through a
  new `Com` method; without IID the IDispatch wrapper, with IID the
  raw interface pointer.

*Seams:* `dll.rs` (the definition parser is shared by the three),
`parser.rs` (`#DllImport`), `builtins.rs` (the unknown-name hook of
WinApi), `desktop/mod.rs` `Native`/`Com` + both backends (the Windows
backend does the real work; the box refuses with the reason).

*Measurement:* `TrueSleep:=DynaCall("kernel32\Sleep","ui",100)`;
the `SendMessage` reorder probe of the CHM (suspend via `AHKCMD(65305)`);
`#DllImport RGB_TO_BGR` with the 32:64 hex of the CHM, called with
`0xAABBCC`; `MCodeH("8AE18AC5C3","h==h")`; `ComObjDll` over the v1
release dll with the `AutoHotkey.Script.UNICODE` GUID.

### H5 — the threads

The core of the fork, over the worker machinery that exists:

- **`NewThread(Script, Params, Title, Wait, Library)`** (h2),
  **`ThreadObj(Script, Params, Title)`** (h2),
  **`AhkThread(ScriptOrFile, Params, Title, ScriptIsFile, DllToUse)`**
  and **`AhkExported()`** (h1) — a worker that runs a **whole script**
  with its own globals, its own `A_Args`, its own title; `Wait` = the
  start handshake; `Library` (h2) and `DllToUse` (h1) refuse with the
  reason until H8 (there is no dll to load — the thread is in-process,
  which is what `NewThread` without `Library` does anyway).
- **The control object** — the methods `ahkFunction`,
  `ahkPostFunction`, `ahkExec`, `addScript`, `addFile` (h1),
  `ahkassign`, `ahkgetvar` (value and `.var` property syntax),
  `ahkLabel`, `ahkPause`, `ahkReady`, `ahkTerminate`, `ahkReload`
  (h1), `ahkdll`/`ahktextdll` (h1, restart the worker's script),
  `ahkExecuteLine`, `ahkFindFunc`, `ahkFindLabel`; the ThreadObj
  syntax `thread.var`, `.Call`, `.PostCall`, `.FuncPtr`, `.LabelPtr`,
  `.Exec`, `.AddScript`, `.ExitApp`. The calls cross as messages to
  the worker's queue — the `SendMessage`/`PostMessage` semantics of
  the fork become the wait/no-wait of the queue.
- **Sharing:** `CriticalSection` + `EnterCriticalSection` /
  `LeaveCriticalSection` / `TryEnterCriticalSection`;
  `CriticalObject(obj, lpCS)` with the `1`/`2` unwraps and the
  last-reference free; `#CriticalObjectSleepTime`,
  `#CriticalObjectTimeOut`; `ObjShare(obj)` ⇄ lresult — in this tree
  a marshalled proxy over the worker queue, not COM `LresultFromObject`
  (D6); `Alias`/`GetVar` across threads (H3).
- **Variables:** `A_ThreadID`, `A_MainThreadID`, `A_ArgsMap` (h2).
- **`CreateScript(Script, ScriptPath, Password)`** — the source spans
  of `Start:End` labels and `Func{}` out of the parser;
  **`DynaRun(Script, Name, Params, Exe)`** — a new runner process,
  the script on the pipe (`run_with_stdin`), object parameters
  through `ObjDump`/`ObjLoad` (H6) or refused until then.

*Seams:* `desktop/mod.rs` (the worker trait grows: start-a-script,
queue-a-call, read/write-a-variable, pause, terminate, the shared
global table), `classes/realthread.rs` as the model, a new
`classes/hthread.rs`, the runtime's callback queue, the parser's
source spans.

*Measurement:* the `CriticalObject` counter of the CHM — 4 threads ×
1 000 000 increments with manual lock = 4 000 000; the
`CriticalSection` turn-taking probe; `MyThread.ahkgetvar.variable`;
`ObjShare` calling a class method from the other thread; `DynaRun`
with `{a:"Hello"}` arriving in `A_Args`. The runner's report carries
the thread output per D4.

### H6 — the archives and the crypto

- **The Zip family** (both): `ZipCreateFile`, `ZipAddFile`,
  `ZipAddFolder`, `ZipCloseFile`, `ZipCreateBuffer`, `ZipAddBuffer`,
  `ZipCloseBuffer`, `UnZip`, `UnZipBuffer`, `ZipInfo`, `ZipOptions`,
  `ZipRawMemory`, `UnZipRawMemory`, `ZipCompressionLevel()` (h1) /
  `A_ZipCompressionLevel` (h2). The file forms go through the `Files`
  trait; the buffer forms are pure. The codec is D3: an in-tree
  deflate (store + fixed/dynamic Huffman) or a feature-gated
  `no_std`-compatible crate — the decision before the phase starts.
- **`CryptAES(addr, size, password, encrypt, 128|192|256)`** — AES
  with the fork's key derivation, the +16-byte buffer rule; the same
  codec decision.
- **`ObjDump(obj, password)` / `ObjLoad(addrOrPath, password)`** —
  the serialization of the Value graph: Object/Map/Array items and
  properties, Buffer bytes, the ComValue types the CHM lists, shared
  references (the >16-object case of the fork's changelog), Struct →
  unsorted Map, no `base`; compressed with `""`, encrypted with a
  password, wrong password refused (the crash the fork fixed).
- **`SevenZip()`** — the 7z module. First answer: the reason (the
  embedded module is LGPL and is not carried); then, if D3 allows, a
  feature-gated binding to a system 7z or an in-tree LZMA. The method
  surface of the CHM (Add/Extract/ExtractMem/List/Hash/…) is the
  contract either way.

*Seams:* a new `crates/ahk/src/h/zip.rs` + `crypt.rs` + `objdump.rs`
(pure, `alloc` only), the `Files` trait for the file forms, the
feature gate of `Cargo.toml` if D3 chooses a crate.

*Measurement:* round-trip probes — `ZipCreateBuffer`/`ZipAddBuffer`/
`ZipCloseBuffer` → `UnZipBuffer` byte-identical; `ZipRawMemory` with
a password → `UnZipRawMemory` with the wrong password refused;
`ObjDump` of the CHM's `{key:"value",1:"test",2:10}` → `ObjLoad`
equal; `CryptAES` encrypt/decrypt round-trip at 128/192/256.

### H7 — the memory module, the resources, the process from memory

The Windows-shaped module work; every name answers the reason on the
box until the Windows backend carries it:

- **MemoryModule:** `MemoryLoadLibrary`, `MemoryGetProcAddress` (name
  and ordinal), `MemoryFreeLibrary`, `MemoryFindResource`,
  `MemorySizeOfResource`, `MemoryLoadResource`, `MemoryLoadString`,
  `ResourceLoadLibrary` (h1). A PE loader in the Windows backend
  (map, relocate, resolve imports, TLS, exceptions registered/freed —
  the two x64 crashes of the fork's changelog are the test cases),
  written original from the PE format, not from MemoryModule's code.
- **Resources:** `ResGet`, `ResExist`, `ResPut`, `ResPutFile`,
  `ResDelete`, `ResDllCreate` — `FindResource`/`LoadResource` and
  `BeginUpdateResource`/`UpdateResource`/`EndUpdateResource`.
- **`BinRun(addrOrPath, params, scriptParams, hide, exeToUse)`** —
  the exe from memory; the heaviest name of the fork. Carried with
  the reason first; the implementation is a decision of its own
  (a suspended-process mapping on Windows) and may stay refused.
- **Variables:** `A_ModuleHandle`, `A_MemoryModule`, `A_IsMini` (h1).
- **The resource library:** functions auto-included from `LIB\NAME.AHK`
  resources (of the exe and, for a dll script, of the host exe),
  compressed/encrypted resources decompressed on load (H6 codecs),
  **`#UseStdLib`** (h2: UserLib → resource LIB → WinApi on an unset
  name), the dynamic **`#include <lib:…>`** (h2), **`Lib.lnk`** and
  **`/Execute`** (h1, the runner).

*Seams:* `ahk-box/src/windows/` (the loader, the resource API), the
`Native`/`Files` traits, the include resolution of the parser, the
runner's argument handling.

*Measurement:* `MemoryLoadLibrary(A_AhkDir "\AutoHotkey.dll")` →
`MemoryGetProcAddress(…,"ahkdll")` → a script started through it (the
CHM's own example, against the release dll); `ResPut`/`ResGet`
round-trip on a copied exe; `ResDllCreate` + `MemoryLoadString`.

### H8 — the embedding (optional, only with a go-ahead)

The fork's reason to exist is the dll/COM embedding; the tree's reason
is the interpreter. This phase is a **new crate** (an `ahk-dll` beside
`ahk-box`), not a change of the runner:

- the C ABI export surface of both forks (the tables above, the v1
  instance model and the v2 ThreadID model), `AhkIsUnicode` = 1;
- the COM servers: `AutoHotkey.Script` (+ `.UNICODE`/`.X64`; the ANSI
  one is out — the tree is Unicode) and `AutoHotkey2.Application`,
  with the GUIDs of the fork so existing clients bind;
- the Mini build (a feature subset: no hotkeys/hooks/Gui/menus/
  InputHook, `A_IsMini` = 1);
- the compiled-script resources (`/Execute`, the Ahk2Exe-compatible
  resource layout) — only if the packaging asks for it.

*This phase does not start without D1 answered.*

---

## The decisions that are not the writer's

| | The question | The proposal |
|---|---|---|
| **D1** | Does the tree want the embedding (H8) at all, or is the interpreter + the in-script thread surface the whole goal? | H0–H7 first; H8 only on an explicit go-ahead |
| **D2** | The pointer names (`GetVar`, `FindFunc`, `FindLabel`, `A_ScriptStruct`, `A_GlobalStruct`): opaque handles of this tree, or refused with the reason? | opaque handles — `Alias`/`ahkgetvar`/`Struct` consume them; raw dereference refuses |
| **D3** | The codecs (deflate, AES, LZMA): in-tree `alloc`-only implementations, feature-gated `no_std` crates, or refused-with-reason? | in-tree deflate + AES for H6 (they are small and the crate stays dependency-free); SevenZip refused until a binding is decided |
| **D4** | The report of a run with threads: whose steps, whose `output`, whose `variables`? | the main thread's report as today, plus a `threads` array (id, title, alive, output, error) — a contract change, written into `runner-and-report.md` before H5 lands |
| **D5** | The v2-H beta base against the tree's 2.0.31: carry the additions on 2.0.31 and record the beta quirks, or target the beta exactly? | additions on 2.0.31; every measured difference kept in `egm-review/reports/` |
| **D6** | `ObjShare`: COM `LresultFromObject`/`ObjectFromLresult` on Windows only, or a marshalled proxy over the worker queue on every host? | the proxy everywhere (portable, no COM dependency); the Windows COM path only if a script measures a difference |

## What stays out

- **The code of the forks** (GPL-2.0), of MemoryModule (MPL-2.0) and
  of 7-Zip (LGPL) — behaviour only, clean-room.
- **The ANSI builds** (`Win32a`): the tree is Unicode; `AhkIsUnicode`
  answers 1 and the `.ANSI` COM GUID is not registered.
- **The raw internals**: the `_AHKVar`/`_AHKFunc`/`_AHKLine` structure
  definitions of the CHM examples describe the fork's C++ memory; the
  tree's handles are its own (D2), and the examples that read
  `mCharContents` out of a `GetVar` pointer are answered with the
  reason, not with a crash.
- **The Win32a/Win32w floors of the releases**: the tree carries its
  own legacy floors ([windows-legacy.md](windows-legacy.md)); the H
  names ride them or refuse, they do not add a third floor.
- **The Ahk2Exe compiler pipeline** — until H8, and only if asked.
