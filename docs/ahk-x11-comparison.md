# AHK_X11 × AHK — what the AHK_X11 already does, what our tree already does, and what is missing

> A document of comparison and of suggestions.
>
> - **AHK_X11** — <https://phil294.github.io/AHK_X11/> and
>   <https://github.com/phil294/AHK_X11>. A reimplementation of AutoHotkey for
>   Linux, written in Crystal, by Philip Waritschlager, **GPL-2.0**.
> - **AHK (this repository)** — AutoHotkey **v2.0.19** in Rust, in three
>   crates (`ahk`, `ahk-box`, `ahk-cli`), **Apache 2.0**.
>
> The sources of every claim are at the end of the document. What is
> **measured** on the box (192.168.0.40) is marked with the date; what is
> **read** from foreign code is marked as such; what is **inferred** is marked
> as an inference.

---

## 0. How to read this document

The AHK_X11 and our tree are not the same thing, and the comparison only makes
sense after saying **what each one is**:

| | AHK_X11 | AHK (ours) |
|---|---|---|
| Goal | Bring AutoHotkey **v1.0.24** (2004) to Linux/X11 | Bring AutoHotkey **v2.0.19** (2024) to Linux/Wayland, with portability |
| Dialect | **Legacy v1**: `x = 1`, `MsgBox, texto`, `%var%`, pseudo-arrays | **v2.0.19**: `x := 1`, `MsgBox texto`, classes, objects, methods, `&` |
| Language | Crystal 1.5.1 (single binary) | Rust (3 crates, `no_std` core) |
| Surface | 226 v1 commands — 127 done (56%), 15 new, 11 removed, 70 to do | 725 entries of the surface of v2.0.19 — **93 built-ins with a real implementation**, 253 with a motivated refusal |
| Windows | Full X11 (xdotool fork): activation, moving, resizing, minimizing, closing | Wayland: **motivated refusal** — the compositor does not expose a list of windows |
| Injection | XTEST (through `x_do`) | **uinput** (our own keyboard and pointer) + `grim` for the screen |
| GUI | GTK3 (windows, `MsgBox`, `InputBox`, `Menu`, `ToolTip`, `TrayTip`) | `Gui`/`Menu`/`Hotkey` **refused** with the reason |
| Classes | None (v1 has none) | `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey`, `Object` |
| Concurrency | Cooperative, interruptible threads (`src/run/thread.cr`) | One script, one thread; guard rails instead of interruption |
| License | GPL-2.0 | Apache 2.0 |

**The conclusion in one line**: the AHK_X11 is *old in the dialect and complete
on the screen*; we are *new in the dialect and incomplete on the screen*. The
suggestions of section 5 exploit exactly that division.

---

## 1. The AHK_X11 on one page

### 1.1 Technical sheet

- **Repository**: <https://github.com/phil294/AHK_X11> — branch `master`,
  Crystal 1.5.1, `shard.yml` at version **1.0.7**. The documentation describes
  the dialect of AutoHotkey **v1.0.24**, which is what it implements.
- **Authorship**: Philip Waritschlager, GPL-2.0. The original AutoHotkey
  documentation is by Chris Mallett (AutoIt).
- **Dependencies** (all Crystal, no external runtime for the core): `x11`
  (fork `phil294/x11-cr`), `xtst` (the XTEST extension), `x_do` (fork of
  xdotool), `gtk3`, `atspi` (accessibility), `notify` (notifications),
  `tasker` (command line).
- **Packaging**: single binary; `ahk_x11.AppImage`; `shard` publishes the AUR
  package (`ahk_x11`, `ahk_x11-bin`, `ahk_x11-git`); graphical installer;
  `.desktop` files and `mime.xml` to associate `.ahk`; compiler to a standalone
  executable (`--compile`).
- **Execution modes**: `ahk_x11 script.ahk`, `ahk_x11 < script.ahk` (stdin),
  `--repl` (interactive console), `--compile`, `--windowspy`, shebang
  (`#!/caminho/ahk_x11`), and `DISPLAY=` to run *headless* (no GUI).
- **Hard requirement**: **X11**. Wayland is not supported (issue #2 of the
  project). Ubuntu 22.04+ and Wayland by default are the hard case.

### 1.2 Official numbers (the README of the project)

```
DONE      56% (127/226)
NEW       7%  (15/226)
REMOVED   5%  (11/226)
TO DO     31% (70/226)
```

The README also says: *"Currently about 80% of work of getting there is done"*,
with the declared goal of 80% of functional parity.

### 1.3 What it is, and what it is not

It is a **reimplementation of AHK v1.0.24**, complete enough to run real
scripts, but without the modern inventions of AHK:

- **No expressions** — only the legacy syntax (`x = %y%`, `MsgBox, texto`).
- **No classes nor objects** — `class MyClass {}` does not exist; only
  variables.
- **No real arrays** — pseudo-arrays with `StringSplit` + `StringTrimLeft`,
  exactly as the original documentation did.
- **No functions** — `Gosub` and labels; `#DefineCommand` is the improvised
  substitute.
- **`#NoEnv` is the default** — the environment is not injected into the
  variables.
- **All the arguments are evaluated at runtime**, even the static ones.

That is, in itself, the **biggest lesson** the AHK_X11 has for us: it proves
that the hard part of an AHK for Linux is not the language, it is the
**screen**. And it proves the **cost** of being stuck to a dead dialect: half
of the effort spent on v1 compatibility is effort that v2 solves another way.

---

## 2. The implementations of the AHK_X11, by area

This is the complete list of what the project does, read from the code and the
documentation. Each line says **the mechanism** (how it does it), because that
is what matters when one decides what to port.

### 2.1 The runtime

- Single Crystal binary; execution by script, by stdin, by REPL, or compiled
  to a standalone executable.
- Own parser (`src/build/parser.cr`) and *linker* (`src/build/linker.cr`) that
  embeds the script in a new executable.
- `src/run/runner.cr` (~15 KB) coordinates: **display**, **threads**, the
  **settings** of the runner (the "last found window", `detect_hidden_windows`,
  the pid of the sound).
- `src/run/display/` has **adapters** (`display-adapter.cr`) with an abstract
  interface of 6 methods: `run`, `grab_hotkey`, `ungrab_hotkey`, `block_input`,
  `unblock_input`. The implementations are `x11.cr` (18 KB) and `gtk.cr`
  (17 KB).
- `src/cmd/` has one command per file, ~120 files, grouped by family
  (`control-flow`, `file`, `gtk`, `math`, `misc`, `string`, `unix`, `x11`).

### 2.2 Hotkeys and hotstrings

- **Hotkeys** (`src/run/display/hotkey.cr`, 5 KB): `XGrabKey` straight on the X
  server, with the modifiers and the options of AHK v1 (`t` — transparent,
  `c` — *context-sensitive*, `p` — *physical*, `i` — *ignore all modifiers*,
  `w` — *wildcard*).
- **Key remapping** — `F1::LButton` works; a mouse button triggered by another
  button cannot be sent again (a limitation of X11, documented).
- **Hotstrings** (`src/run/display/hotstring.cr`, 2 KB): a word recognition
  engine, with expansions `::btw::By the way`, `:R:` (literal), `:C:` etc.
- **`BlockInput`** (`src/cmd/x11/block-input.cr`): blocks the input of the
  user.
- **`Pause`**, **`Suspend`**, **`Reload`**: the script can be paused,
  suspended or reloaded.
- **Captured keys**: `src/run/display/pressed-keys.cr`.

### 2.3 Sending keys

- **`Send`** (`src/cmd/x11/keyboard/send.cr`, 4,8 KB) and **`SendRaw`**: inject
  through **XTEST**, with the key parser of AHK (`{Ctrl}`, `{F1}`, `{Blind}`…).
- **`Send` is limited by the layout**: `§` does not come out; the note of the
  README is to use `SendRaw`.
- **`Input`** (`src/cmd/x11/keyboard/input.cr`, 3 KB): reads what the user
  types, with `EndKeys` and `MatchList`.
- **`KeyWait`**, **`GetKeyState`**, **`SetKeyDelay`**.

### 2.4 Mouse

- **`MouseMove`, `MouseClick`, `MouseClickDrag`, `MouseGetPos`,
  `SetMouseDelay`** (`src/cmd/x11/mouse/`), through XTEST.
- **`ControlClick`** (`src/cmd/x11/mouse/control-click.cr`): a click on a
  control, through position + AT-SPI.
- **`Click`** is a *new* command (v2-like) that the project adds.

### 2.5 Windows

This is the most complete subsystem of the AHK_X11, and it is worth reading
`src/cmd/x11/window/win-util.cr` (4,5 KB) with attention:

- **Window search** through `x_do` (fork of xdotool): by name (regex), by
  class (`ahk_class`), by `ahk_id`, by the current desktop, and only the
  visible ones when `DetectHiddenWindows` is off.
- **`WinText` / `ExcludeText`** are resolved through **AT-SPI**, not through
  X11.
- **Commands**: `WinActivate`, `WinActivateBottom` (to do), `WinClose`,
  `WinGet` (`id`, `pid`, `processname`, `controllist`, `count`, `list`),
  `WinGetPos`, `WinGetText`, `WinGetTitle`, `WinGetClass`, `WinHide`,
  `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`/`Undo`,
  `WinMove`, `WinRestore`, `WinSet`, `WinSetTitle`, `WinShow`, `WinWait`,
  `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`.
- **`IfWinExist` / `IfWinNotExist` / `IfWinActive` / `IfWinNotActive`**.
- **The "last found window"** — a v1 concept that v2 abandoned.

### 2.6 Controls — the accessibility layer

The file `src/run/display/at-spi.cr` has **20 KB** and is the most substantial
one outside the X11 core. It implements:

- **`ControlGetText`**, **`ControlSetText`**, **`ControlGetPos`**
  (`src/cmd/x11/control/`), **`ControlSend` / `ControlSendRaw`**.
- Reading the text of a whole window, for the `WinGetText` and for the
  `WinText`.
- **`ControlClick`** (position + AT-SPI).

**This is the most valuable piece of the AHK_X11 for us** — see section 5.4.
The AT-SPI is a **D-Bus** protocol, not an X11 protocol: it works on Wayland.

### 2.7 Screen

- **`PixelGetColor`** (`src/cmd/misc/pixel-get-color.cr`) and
  **`PixelSearch`** (`src/cmd/misc/pixel-search.cr`): read the image of the
  screen through `XGetImage` and look for the color.
- **`CoordMode`**: `Screen` or `Relative` for mouse, pixel and tooltip.
- **`Screenshot`** does not exist (it is a v2 name — we have it, and it is the
  only invention of our dialect; see AGENTS.md §14).

### 2.8 GUI and dialogs (GTK3)

- **`Gui`** complete (`src/cmd/gtk/gui/`, 7,8 KB in `gui-add.cr` alone):
  `Add` with ~30 control types (`Text`, `Edit`, `Button`, `ComboBox`,
  `ListView`, `TreeView`, `Tab`, `Picture`, `Progress`, `Slider`, `DateTime`,
  `MonthCal`, `GroupBox`, `Link`, `Hotkey`, `DDL`, `UpDown`…), `Show`, `Hide`,
  `Submit`, `Cancel`, `Destroy`, `Color`, `Option`, `Scroll`.
- **`GuiControl`**: `Add`, `Show`, `Hide`, `Enable`, `Disable`, `Focus`,
  `Choose`, `ChooseString`, `ChooseIndex`.
- **Dialogs**: **`MsgBox`** (with the buttons and the icons of AHK v1),
  **`InputBox`**, **`FileSelectFile`**, **`FileSelectFolder`**,
  **`IfMsgBox`**.
- **`Menu`** (`src/cmd/gtk/menu.cr`): **only the tray menu** — the command
  refuses any name that is not `TRAY`. Sub-commands: `Add` (which also creates
  the icon), `Icon`, `NoIcon`, `Tip`.
- **`ToolTip`** (`src/cmd/gtk/tooltip.cr`) and **`TrayTip`** (libnotify).

### 2.9 Files

- `FileAppend`, `FileCopy`, `FileCreateDir`, `FileDelete`, `FileGetSize`,
  `FileRead`, `FileReadLine`, `FileRecycle` (puts it in the trash),
  `FileRecycleEmpty`, `FileSetAttrib`, `FileSetTime`, `FileGetAttrib`.
- `IniRead`, `IniWrite`, `IniDelete`.
- `SplitPath`, `SetWorkingDir`, `DriveSpaceFree`.
- `Loop Files` (with `R` recursive), `Loop Read`, `Loop Parse`.

### 2.10 Sound

- **`SoundPlay`** (`src/cmd/misc/sound-play.cr`): a ladder of players, in order
  of preference — `ffplay` → `cvlc` → `paplay` → `mpg123` → `aplay`. Kills the
  previous one before playing.
- **`SoundGet` / `SoundSet`** (`src/cmd/misc/sound-get.cr`): the volume comes
  from `amixer` (ALSA), not from PulseAudio.
- **`SoundGetWaveVolume` / `SoundSetWaveVolume`**: to do.

### 2.11 Processes and environment

- **`Run` / `RunWait`** (`src/cmd/misc/run.cr`, 6,8 KB), **`RunAs`**,
  **`Shutdown`**, **`URLDownloadToFile`**, **`Edit`**.
- **`EnvGet`** (a *new* command in the AHK_X11), **`EnvSet`**, `EnvAdd`,
  `EnvSub`, `EnvMult`, `EnvDiv`.

### 2.12 `DllCall` — the bridge to the system libraries

`src/cmd/misc/dll-call.cr` (3,6 KB) uses the **FFI of Crystal**:
`Crystal::Loader` (which is `dlopen`/`dlsym`) and `Crystal::FFI::CallInterface`.
Types: `Int`, `Int64`, `Short`, `Char`, `Float`, `Double`, `Str`, `Ptr`, with
the `P`/`*` suffix for an address. The `Str` is read back as text.

Limitations declared in the code: the variables of the AHK_X11 are **text
only**, and every non-`Ptr` value is serialized and deserialized — "a lot of
conversion going on which isn't optimal but necessary due to ahkx11's
string-only vars".

### 2.13 Binary shadowing

Three commands that the AHK_X11 adds to manipulate memory:

- **`NumGet`** (`src/cmd/misc/num-get.cr`): reads a number from a buffer.
- **`NumPut`** (`src/cmd/misc/num-put.cr`): writes a number into a buffer.
- **`VarSetCapacity`** (`src/cmd/misc/var-set-capacity.cr`): sizes the buffer
  of a variable.
- The base is `src/util/ahk-bin-var.cr`, which maps between the text of the
  variable and the bytes.

### 2.14 Threads — the v1 model

`src/run/thread.cr` (9,2 KB) implements the **cooperative** threads of AHK v1:

- A new thread **interrupts** the current one; the interrupted ones stay
  asleep and are resumed in reverse order.
- Each thread has its own settings (ErrorLevel, SetKeyDelay).
- **Priorities** (`Thread`, `SetTimer` with a priority): a thread of lesser
  priority does not interrupt the current one.
- `OnExit` always runs, whatever the priority.

### 2.15 Packaging and tools

- **Compiler** (`src/build/`): script → standalone executable, with its own
  *parser* and *linker*; the generated binary has hotkeys and GUIs.
- **Window Spy** (`src/window-spy.ahk`): shows the window and the control
  under the mouse.
- **Graphical installer** (`src/installer.ahk`, 5,8 KB) +
  `ahk_x11-compiler.desktop`, `ahk_x11-windowspy.desktop`, `ahk_x11.desktop`,
  `ahk_x11-mime.xml`.
- **REPL**: after the auto-run, it accepts one-line commands; each line runs
  in its own thread.
- **`Echo`** (`src/cmd/unix/echo.cr`): prints on stdout — the "Unix mode".
- **`Eval`** (`src/cmd/misc/eval.cr`): evaluates code at runtime.

### 2.16 The commands that the AHK_X11 adds (15)

`Echo`, `Eval`, `#DefineCommand`, `DllCall`, `NumGet`, `NumPut`,
`VarSetCapacity`, `FileRead`, `RegExGetPos`, `RegExReplace`, `EnvGet`,
`Click`, `ahk_x11_print_vars`, `ahk_x11_track_performance_start`,
`ahk_x11_track_performance_stop`.

### 2.17 What the AHK_X11 removes (11)

`EnvUpdate`, `PostMessage`, `RegDelete`, `RegRead`, `RegWrite`, `SendMessage`,
`#InstallKeybdHook`, `#InstallMouseHook`, `#UseHook`, `Loop (registry)` —
*"those that simply make no sense in Linux"*.

`AutoTrim` was set aside for a reason that is worth copying: *"It's always Off.
It would not differentiate between %a_space% and %some_var%. It's possible but
needs significant work."*

### 2.18 What is left to do (70)

The families: `Drive*`, `FileCopyDir`/`FileMove`/`FileMoveDir`/`FileRemoveDir`,
`FileGetShortcut`/`FileCreateShortcut`/`FileGetVersion`/`FileInstall`,
`FileGetTime`/`FileGetAttrib`, `FormatTime`, `SetFormat`, `StringCaseSense`,
`Process`, `SysGet`, `Thread` (the command, not the model), `SetBatchLines`,
`OnExit`, `Progress`/`SplashImage`/`SplashTextOn`, `GuiControlGet`,
`GroupAdd`/`GroupActivate`/`GroupClose`/`GroupDeactivate`,
`StatusBarGetText`/`StatusBarWait`, `WinMenuSelectItem`,
`Control`/`ControlFocus`/`ControlGet`/`ControlGetFocus`/`ControlMove`,
`DetectHiddenText`, `SetTitleMatchMode`, `SetWinDelay`, `SetControlDelay`,
`SetDefaultMouseSpeed`, `SetNumScrollCapsLockState`/`SetStoreCapslockMode`,
`KeyHistory`, `ListHotkeys`/`ListLines`/`ListVars`, `SoundGetWaveVolume`/
`SoundSetWaveVolume`, `FormatTime`, and the directives `#CommentFlag`,
`#ErrorStdOut`, `#EscapeChar`, `#HotkeyInterval`, `#HotkeyModifierTimeout`,
`#MaxHotkeysPerInterval`, `#MaxMem`, `#MaxThreads`, `#MaxThreadsBuffer`,
`#MaxThreadsPerHotkey`, `#WinActivateForce`.

---

## 3. Our tree, by the same list

### 3.1 The architecture

```
ahk-cli  (runner)  →  ahk-box (primitivas do Linux)  →  ahk (linguagem)
```

- **`ahk`** — lexer, AST, parser, runtime. **It does no I/O.** It compiles
  without `std`
  (`cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf`).
  The language is **v2.0.19**, read from the binary: 725 entries of surface
  (644 distinct names), 11 class tables, operator table.
- **`ahk-box`** — the box: uinput keyboard, uinput pointer, `grim`, `xprop`,
  `dlopen`, the *raven*, files.
- **`ahk-cli`** — the runner: script on stdin, JSON report on stdout, pid file
  for the emergency stop.

The runtime is handed a `Desktop`, which is the sum of **ten** capabilities:
`Keyboard`, `Pointer`, `Screen`, `Windows`, `Processes`, `Files`, `Clipboard`,
`Timers`, `Hotkeys`, `Native`. There is no default implementation: every
implementor writes the ten.

### 3.2 What answers with a value, and what answers with the reason

The table of built-ins (`crates/ahk/src/builtins.rs`) has **346 entries**. The
measurement of today:

| State | How many | What it is |
|---|---|---|
| A serious handler | **93** | The command works |
| `not_carried` | **253** | The name exists and answers **with the reason** |

The five reasons of `not_carried`:

| Variant | Reason | Typical names |
|---|---|---|
| 0 | *"this is a name of Windows, and this desktop is not Windows."* | `WinMove`, `ControlClick`, `ToolTip`, `SoundPlay`, `MenuSelect` |
| 1 | *"this runtime has no hotkeys yet."* | `Hotkey`, `Hotstring`, `HotIf`, `HotIfWinActive` |
| 2 | *"the clock of this runtime counts from an arbitrary origin, and it does not know the date."* | `FormatTime`, `DateAdd`, `DateDiff` |
| 3 | *"this runtime is one script that ends when it ends, and it cannot do that."* | `Persistent`, `Reload`, `Suspend`, `Pause`, `OnExit` |
| 4 | *"this desktop cannot serve that with the files it has."* | `FileCopy`, `FileMove`, `FileGetSize`, `IniRead`, `IniWrite`, `DirCopy` |

The 93 with a real implementation, by family: mathematics (`Abs`, `Ceil`,
`Floor`, `Round`, `Max`, `Min`, `Mod`, `Sqrt`, `Exp`, `Ln`, `Log`, `Sin`,
`Cos`, `Tan`, `ASin`, `ACos`, `ATan`, `Random`), text (`Chr`, `Ord`, `Format`,
`StrLen`, `StrUpper`, `StrLower`, `SubStr`, `StrReplace`, `InStr`, `Trim`,
`LTrim`, `RTrim`, `StrCompare`, `StrSplit`, `Sort`, `RegExMatch`,
`RegExReplace`), objects (`Type`, `IsObject`, `HasProp`, `HasMethod`,
`GetMethod`, `HasBase`, `IsSetRef`, `ObjGetBase`, `ObjSetBase`,
`ObjOwnProps`, `ObjOwnPropCount`, `ObjHasOwnProp`, `ObjGetCapacity`,
`ObjSetCapacity`, `ObjAddRef`), files (`FileAppend`, `FileDelete`,
`FileExist`, `FileRead`, `DirCreate`, `DirDelete`, `DirExist`), screen and
mouse (`PixelGetColor`, `PixelSearch`, `Screenshot`, `Click`, `MouseMove`,
`MouseGetPos`), windows (`WinActive`, `WinExist`, `WinGetTitle`), and the rest
(`MsgBox`, `Send`, `KeyWait`, `SendMode`, `SetKeyDelay`, `SetTimer`, `Sleep`,
`Run`, `RunWait`, `Exit`, `ExitApp`, `EnvGet`, `EnvSet`, `SplitPath`,
`VarSetStrCapacity`, `DllCall`).

The **131 variables** of v2 are all carried: the 40 that the runtime knows
(`A_AhkVersion`, `A_Index`, `A_ScreenWidth`, `A_ScreenHeight`, `A_TickCount`,
`A_PtrSize`, `A_Space`, `A_Tab`, `A_EndChar`, `A_ThisFunc`, …) and the others
with the reason.

### 3.3 What the box does on Linux, measured

| Primitive | Mechanism | Measurement (2026-10-25 / 27 / 28) |
|---|---|---|
| Text and keys | our own **uinput** keyboard, US layout | `Send "Teste-Ahk 42!"` writes the whole string, capitals and punctuation included |
| Modifiers | they go down before the key, and up after | `ctrl+a` selects the text of a GTK field and the backspace deletes it |
| Pointer | absolute **uinput**, `INPUT_PROP_DIRECT` | a click at `(830, 316)` goes through the pointer |
| Screen | `grim` | `PixelGet 100, 100` → `#12101b` |
| Windows | `xprop` (the Xwayland of gamescope) | `WinExist`/`WinActive` → *"unknown"*, with the reason |
| Native calls | `dlopen`/`dlsym` + trampoline in `asm!` | 13 green cases on Windows, 14 green on the box |
| Windows DLLs | **raven** (an 18 KB PE, freestanding) | the box gets to call Windows DLLs |
| Files | `std::fs` | `FileAppend("hello", "ahk_probe.txt")` writes in the working directory |

Safety limits: `MAX_STEPS` 20 000, `MAX_ITERATIONS` 1 000, `MAX_DEPTH` 1 000,
`MAX_STACK` 512 KB, `Sleep` ≤ 60 000 ms, timeout of the runner 60 s. JSON
report: `ok`, `steps`, `output`, `notes`, `error`, `error_line`, `exit_code`,
`elapsed_ms`, `variables`.

### 3.4 What is still a sketch

In the working tree of today:

| File | State |
|---|---|
| `crates/ahk/src/hotkeys.rs` | sketch — `run_fired` returns `Ok(())` |
| `crates/ahk-box/src/hotkeys.rs` | sketch |
| `crates/ahk/src/files.rs` | sketch (only the header) |
| `crates/ahk-box/src/files.rs` | sketch |
| `crates/ahk/src/date.rs` | `DateTime` and `WallClock` defined; `system_wall_clock()` returns `None` |
| `crates/ahk/src/settings.rs` | sketch |

And the classes: `Map`, `Array`, `Buffer` and `File` **work**; `Gui`, `Menu`
and `Hotkey` are carried and **refuse** with the reason
(*"a menu is the menu bar of a Windows window, and this desktop is a Wayland
session without one"*).

---

## 4. Comparison

### 4.1 Coverage by area

| Area | AHK_X11 | AHK (ours) | State on our side |
|---|---|---|---|
| **Hotkeys** | ✅ XGrabKey, v1 options, remapping, BlockInput | ❌ refusal (variant 1) | **the biggest gap** |
| **Hotstrings** | ✅ word engine | ❌ refusal (variant 1) | depends on the hotkeys |
| **Threads** | ✅ cooperative, interruption, priority | ❌ one thread | a big change of model |
| **Sending keys** | ✅ XTEST, `Send`/`SendRaw` | ✅ **uinput** (better for Wayland) | ✅ ahead |
| **Mouse** | ✅ XTEST | ✅ absolute **uinput** | ✅ ahead |
| **Screen** | ✅ `XGetImage` | ✅ **`grim`** (Wayland) | ✅ ahead |
| **Windows** | ✅ complete (xdotool fork) | ❌ *"unknown"* (Wayland without a list) | needs backends |
| **Controls (AT-SPI)** | ✅ text, position, click | ❌ refusal (variant 0) | **opportunity** |
| **GUI / dialogs** | ✅ complete GTK3 | ❌ refusal (variant 0) | needs a toolkit |
| **Files** | ✅ 15 commands | ⚠️ 12 of the 30 methods of the trait `Files` | mechanical work |
| **INI** | ✅ | ❌ refusal (variant 4) | mechanical work |
| **Sound** | ✅ ffplay/vlc/paplay/amixer | ❌ refusal (variant 0) | **new capability** |
| **Environment** | ✅ `EnvGet`/`EnvSet` | ✅ | ✅ the same |
| **Processes** | ✅ `Run`/`RunWait`/`RunAs`/`Shutdown` | ⚠️ real `Run`/`RunWait`, the rest refuses | easy |
| **`DllCall`** | ✅ the FFI of Crystal (v1) | ✅ **dlopen + trampoline, v2, and the raven** | ✅ far ahead |
| **Binary (`NumGet`/`NumPut`)** | ✅ | ❌ refusal (variant 0) | over the `Buffer` class |
| **Clipboard** | ✅ GTK + exponential back-off | ❌ refusal (variant 0) | **the cheapest of all** |
| **Compiler for .exe** | ✅ | ❌ (the runner is the binary) | low value for us |
| **REPL / Window Spy / installer** | ✅ | ❌ | optional |

### 4.2 The names that changed from v1 to v2

A script of the AHK_X11 does **not** run in our tree, and almost always
because of a name:

| v1 (AHK_X11) | v2 (ours) | Note |
|---|---|---|
| `MsgBox, texto` | `MsgBox texto` | the comma is no longer a separator |
| `PixelGet` | `PixelGetColor` | |
| `Upper` / `Lower` | `StrUpper` / `StrLower` | |
| `StringLen` | `StrLen` | |
| `StringMid` / `StringLeft` / `StringRight` | `SubStr` | three in one |
| `StringTrimLeft` / `StringTrimRight` | `LTrim` / `RTrim` | |
| `StringSplit` | `StrSplit` | |
| `StringReplace` | `StrReplace` | different signature (v2 has the *case sense* and writes the number of replacements) |
| `StringGetPos` | `InStr` | |
| `IfWinExist` | `WinExist` | |
| `IfWinActive` | `WinActive` | |
| `MouseClick` | `Click` | |
| `FileSelectFile` | `FileSelect` | |
| `FileSelectFolder` | `DirSelect` | |
| `VarSetCapacity` | `VarSetStrCapacity` | only the text one |
| `IfMsgBox` | — | does not exist in v2 |
| `Transform` | — | does not exist in v2 |
| `RegExGetPos` | — | does not exist in v2 |
| `EnvGet` | `EnvGet` | in v1 it was needed; in v2 as well |
| `Hotstring` | `Hotstring` | a command, not a directive |
| `Gui, Add` | `Gui.Add` | a class, not a command |
| `Menu` | `Menu` | a class, not a command |

### 4.3 What the AHK_X11 does that we refuse — and why

Three refusals of ours that **must not** be reverted:

1. **Windows.** We refuse because the session of the box is Wayland and the
   compositor does not expose a list of windows. The AHK_X11 manages because
   X11 exposes it. It is not a failure: it is the correct symptom in a Wayland
   session.
2. **Injection through XTEST.** The AHK_X11 injects through XTEST because it
   lives in X11. We use uinput, which is what works on Wayland — and what the
   box measured.
3. **`DllCall` in the v1 style.** The AHK_X11 serializes everything as text
   because its variables are text. Our `DllCall` has types, conventions, and
   writes back — it is the v2 version.

One refusal of ours that **should** be reviewed:

4. **Clipboard.** We refuse for want of the trait, not because of a real
   limitation of the platform. `wl-paste`/`wl-copy` solve it on Wayland,
   `xclip`/`xsel` on X11. There is no technical reason.

### 4.4 What we do that the AHK_X11 does not

- **The full dialect of v2.0.19**: expressions, classes, objects, methods,
  reference by `&`, `Prototype`/`Base`, the `is` operator, the `Obj*` family.
- **Nine registered class types** — which cover the eleven tables of the
  binary: `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu`, `Hotkey`,
  `Object`.
- **Precedence and semantics measured against the binary** (`-2 ** 2` is -4;
  `"a" 1 + 2` is `"a3"`).
- **`no_std` core**: the language compiles for a target without a standard
  library.
- **The JSON report**: every step, every note, the line of the error, the
  limits.
- **Guard rails** instead of threads: a script that gets lost is stopped with
  the line of the error.
- **The `DllCall`** that talks to `.so` **and** to Windows DLLs, on Linux,
  through a PE of its own.

### 4.5 The lessons of method

1. **The screen is the work; the language is what is already done.** The
   AHK_X11 spent years on X11/GTK/AT-SPI and is still a v1. We have the right
   language, and the capabilities are missing.
2. **Refusing with the reason is a product decision, not an engineering one.**
   The AHK_X11 refuses what it does not do (`EnvUpdate`, `RegRead`) by removing
   the command. We answer with the reason — more honest, and it is the rule of
   the project (AGENTS.md: *"A question that cannot be answered fails, with the
   reason. It does not answer false."*).
3. **Accessibility is the way to the windows on Wayland.** The AT-SPI is D-Bus,
   not X11. The AHK_X11 has that piece done and measured.
4. **Hotkeys without a window server are half the value.** The AHK_X11 has
   both; we have the input but not the hook.

---

## 5. Suggestions: how to support the AHK_X11

### 5.0 The rule: the **capabilities** are ported, not the mechanism

The mechanism of the AHK_X11 (XTEST, XGrabKey, `XGetImage`, GTK3, the
`dlopen` of Crystal) is the mechanism of **X11**, and our target is
**Wayland**. Porting the mechanism would be a step backwards.

What is ported is the **contract**:

- What each command does, with which arguments, and which error it gives.
- The **options** that the command accepts (`CoordMode`, `SendMode`,
  `SetKeyDelay`, `DetectHiddenWindows`, the hotkey options `t/c/p/i/w`).
- The **recipes** that the users write (§5.4).

And the **seam** where the capability sits already exists: the traits of
`Desktop` are the thread. `Hotkeys`, `Clipboard` and `Windows` already have the
right method and the right signature — the body is missing.

### 5.1 Wave 1 — what gives results already, with little risk

| # | Suggestion | Where | Effort | Impact |
|---|---|---|---|---|
| 1 | **Real clipboard** — `wl-paste`/`wl-copy` (Wayland), `xclip`/`xsel` (X11) | `crates/ahk-box/src/desktop.rs`, `impl Clipboard` | **S** | `Clipboard`, `ClipWait` (with a timeout), and the "copy, transform, paste" recipe |
| 2 | **Sound** — `amixer` for the volume, `ffplay`/`paplay`/`aplay` to play, exactly the ladder of the AHK_X11 | new trait `Sound` in `ahk::desktop`, `impl Sound for LabDesktop` | **S** | `SoundGet`, `SoundSet`, `SoundPlay`, `SoundBeep` come out of the refusal |
| 3 | **`NumGet` / `NumPut`** — over the `Buffer` class, which already exists and works | `builtins.rs`, `classes/buffer.rs` | **S** | Two names that v2 has, and that the `Buffer` already supports, come out of the refusal |
| 4 | **Finish the `Files`** — `copy_file`, `move_file`, `copy_dir`, `move_dir`, `file_attributes`, `set_file_attributes`, `file_times`, `set_file_times`, `ini_read`, `ini_write`, `ini_delete` | `crates/ahk-box/src/desktop.rs` + `crates/ahk/src/files.rs` | **M** | `FileCopy`, `FileMove`, `FileGetAttrib`, `FileSetAttrib`, `FileGetTime`, `FileSetTime`, `IniRead`, `IniWrite`, `IniDelete` come out of the refusal. `std::fs` covers almost everything |
| 5 | **`FormatTime` / `DateAdd` / `DateDiff`** — `system_wall_clock()` stopped returning `None` | `crates/ahk/src/date.rs` | **M** | The real date, and `A_Year`, `A_Month`, `A_Day`, `A_Hour`, `A_Min`, `A_Sec`, `A_MSec`, `A_Now`, `A_NowUTC` |
| 6 | **`ahk --surface`** — a command that prints the table of 725 names with the state of each one (real / refusal / reason) | `crates/ahk-cli/src/main.rs` | **S** | It makes the debt visible; the measurement of today is 93/253 and it is in one line of code |

Item 6 is the cheapest of all and the most useful: it turns "what is missing"
from an opinion into a number.

**The state of this wave (2026-10-29).** The six items are done: the clipboard
(1), the sound (2), the `NumGet` and the `NumPut` (3), the `Files` (4) and the
`FormatTime` with the `DateAdd` and the `DateDiff` (5) came out of the refusal
in the block of section 25 of the notes, and the `ahk --surface` (6) now
prints the state of each of the v2 names.

### 5.2 Wave 2 — hotkeys, threads, hotstrings

This is the wave that **changes the product**: without hotkeys, an AHK is a
command line program that fiddles with the mouse.

**The seam already exists.** `ahk::desktop::Hotkeys` already has
`register_hotkey`, `unregister_hotkey`, `hotkeys` and `fired_hotkey`, and the
documentation of the trait says what the runtime expects:

> *"the desktop watches the keyboard and says which of the hotkeys it
> registered was pressed; the runtime owns the function that the hotkey runs,
> and asks this between the statements of the script."*

The hook is drawn; **the body does not exist**.

| # | Suggestion | Detail |
|---|---|---|
| 7 | **Hotkey backend for X11** | `XRecord` (or `XGrabKey` as the AHK_X11) + `XTest`. It serves plain X11, and it is what allows testing without Wayland |
| 8 | **Hotkey backend for Wayland** | There is no global grab in Wayland. The way is **uinput** + an `evdev` watcher (what `ydotool`/`wtype` do), or a portal. The cost is the same that we already paid for the keyboard: it is a device |
| 9 | **Hotstrings** | `Hotstring` is a v2 command and it is in the refusal (variant 1). It depends on the hotkeys: it is a hook that rewrites the text when the word matches |
| 10 | **Hotkey options** | `t` (transparent), `c` (context-sensitive), `p` (physical), `i` (ignore modifiers), `w` (wildcard), `x` (custom), `h` (hook). They are the contract of v1 and v2 renamed them — but the **set** is what the users know |
| 11 | **`Hotkey` as a class** | The v2 `Hotkey` class has 35 methods (`Start`, `Stop`, `Wait`, `KeyOpt`, `OnKeyDown`, …). The AHK_X11 does not have this notion; we have the sketch. Making the real class is the way of v2 |
| 12 | **Threads: the model of the AHK_X11 is the reference** | `src/run/thread.cr` is a concrete guide: cooperative interruption, priorities, settings per thread. But our runtime is "runs to the end" and the JSON report assumes one thread |

About item 12, the recommendation is **not** to chase the v1 model. v2 does not
have threads either: v2 replaced them with **events and callbacks**
(`OnMessage`, `OnClipboardChange`, `OnExit`, timers). The way of v2 is:

1. An **event loop** in the runtime: between the statements, the runtime asks
   the desktop for a fired hotkey, an expired timer, or an event.
2. The script keeps being a script — the difference is that the runtime can
   *interrupt* the current statement to dispatch the event.
3. The JSON report gains a `thread`/`task` per step, so that the interrogation
   keeps being readable by machine.

This is background work, not a task. It is worth doing **after** the hotkeys,
and not before.

### 5.3 Wave 3 — windows, controls and GUI

**The refusal of windows is still correct on the box.** The Wayland compositor
of the box does not expose a list of windows to D-Bus, and the Xwayland of
gamescope reports an empty list of clients. The right answer is still
*"unknown"* with the reason — not `false`.

What the AHK_X11 teaches is that there are **three ways**, and we only have the
worst:

| Way | Mechanism | Works on Wayland? | Effort |
|---|---|---|---|
| **A. X11 backend** | `xprop` + `xdotool`/`x_do` | No (but it works on X11) | **S** — the box already speaks `xprop` |
| **B. Portal / compositor** | `xdg-desktop-portal`, the D-Bus of COSMIC | Yes, if the compositor exposes it | **M** — depends on the compositor |
| **C. AT-SPI** | D-Bus, the accessibility protocol | **Yes** | **M** — the AHK_X11 has 20 KB of reference |

**Way C is the most interesting.** The AT-SPI is the layer that the AHK_X11
uses for `ControlGetText`, `ControlSetText`, `ControlGetPos`, `ControlClick`
and for the `WinText` of `WinGetText`. And it is a **D-Bus** protocol, which
is exactly what Wayland preserves. With it, a Wayland session gains:

- `ControlGetText` / `ControlSetText` — read and write the text of a field.
- `ControlGetPos` — the geometry of a control.
- `ControlClick` — click a control by name, without coordinates.
- `WinGetText` — the text of a window, which we refuse today.

The `xdotool` (way A) solves the rest: activation, moving, resizing,
minimizing, closing, waiting. It is exactly the `win-util.cr` of the AHK_X11,
and its fork (`phil294/x_do.cr`) solves the bug of the `XGetWindowProperty`
that makes `xdotool` crash in some cases.

**Concrete suggestion:** a new trait, `Accessibility`, with `control_text`,
`set_control_text`, `control_position`, `control_click` and `window_text`,
implemented by AT-SPI, with a default implementation that refuses with the
reason. One more trait in the sum of `Desktop` — the rule of the project is
that "a new capability goes in its own trait, and the `Desktop` grows by
naming it".

**GUI (GTK).** The AHK_X11 does `Gui`, `GuiControl`, `MsgBox`, `InputBox`,
`FileSelect`, `DirSelect`, `Menu`, `ToolTip` and `TrayTip` with GTK3. For us:

- `gtk4-rs` is the choice (GTK4 is what runs on Wayland without XWayland).
- The current `MsgBox` writes in the `output` of the JSON report — **that must
  not change**: the runner has to keep being able to run *headless* and to be
  interrogable. The GUI has to be an **opt-in** (`--gui`), not the default.
- The v2 `Gui` class has 59 names in the surface, and `Gui.Control` 29 more —
  it is a big API, and it is worth a direct mapping of `gui-add.cr` of the
  AHK_X11 to our `Gui` table.

### 5.4 Wave 4 — the recipes, which are the product

The AHK_X11 brings **14 example scripts** in `docs/`:
`ContextSensitiveHelp`, `EasyWindowDrag`, `FavoriteFolders`, `JoystickMouse`,
`JoystickTest`, `KeyboardOnScreen`, `MinimizeToTrayMenu`, `MsgBoxButtonNames`,
`Numpad000`, `NumpadMouse`, `Seek`, `TooltipMouseMenu`, `VolumeOSD`,
`WindowShading`.

They are **v1** scripts, and our tree is **v2**: they do not run. But they are
the best corpus of requirements that exists, because each one exercises a real
combination:

| Script | What it requires |
|---|---|
| `VolumeOSD` | sound + `Progress` + `SetTimer` + `SoundGet`/`SoundSet` |
| `Seek` | `Gui` + `File` + `Loop Files` + `StringSplit` + `Run` |
| `KeyboardOnScreen` | hotkeys + drawing of keys + `Input` |
| `NumpadMouse` | hotkeys + mouse + `KeyWait` |
| `JoystickMouse` | reading of a device + mouse |
| `MinimizeToTrayMenu` | `Menu` + `TrayTip` + events |
| `WindowShading` | `WinGet`/`WinMove`/`WinWait` + state |
| `EasyWindowDrag` | mouse + windows in a row |
| `ContextSensitiveHelp` | `WinGetText` + `MouseGetPos` + `ToolTip` |
| `FavoriteFolders` | `IniRead`/`IniWrite` + `Run` + folders |

**Suggestion:** port each one to v2 and use them as **conformance cases** on
our side — in `crates/ahk/tests/language.rs` (what is language) and in the
verification scripts on the box (what is screen). The `AGENTS.md` already says
where each case should live; these are **end to end** cases, and therefore
they belong to the box, not to the crate of the language.

This also solves the problem of the "I do not know what is missing": each
script is a list of names that the tree still refuses.

### 5.5 Wave 5 — the packaging

| # | Suggestion | Note |
|---|---|---|
| 13 | **`ahk --compile script.ahk`** | Embed the script in the binary of the runner and write an executable. It is what the AHK_X11 promises ("stand-alone binaries with no dependencies") and what an end user wants to carry. Low cost: it is an `include_bytes!` with a small executor |
| 14 | **Window Spy** | The AHK_X11 has its own, in AHK. We have the pieces (pixel reading, position of the pointer) and the `Hotkey` class. A simple TTY that prints the window under the mouse and the pressed keys |
| 15 | **REPL** | The AHK_X11 has `--repl`. It is useful for who is not developing the runtime. Cheaper alternative: a `--check` that does the *parse* and says what fails, without executing |

### 5.6 What is **not** ported

| Not ported | Why |
|---|---|
| The v1 legacy syntax | Our dialect is v2.0.19, and that is the competitive advantage. **This is true of v2, and only of v2**: v1 becomes an option behind a gate, and the decision lives in `docs/ahk-v1-dialect.md` |
| The "output var" convention (`StringLen, out, in`) | In v2 the answer is the return value; the reference by `&` solves the case where it is needed to write |
| `#NoEnv` as the default | In v2 the environment is explicit; `EnvGet` is there |
| The Crystal compiler and the *linker* | Item 13 covers the essential with much less |
| `x_do` (fork of xdotool) as a dependency | Prefer the system `xdotool`, or implement the X11 backend over `x11` + `xcb` in Rust |
| `AutoTrim` | The AHK_X11 set it aside for the same reason: it is ambiguous |
| The `PostMessage`/`SendMessage`/`Reg*` | They are Windows APIs. In v2 they refuse with the reason, which is the right answer |

### 5.7 The note about the license — **important**

The AHK_X11 is **GPL-2.0**. This project is **Apache 2.0**.

- You **can** read its code, study it, and implement the *ideas*, the *recipes*
  and the *behaviour* independently.
- You **cannot** copy its source code into this repository: GPL-2.0 is copyleft
  and Apache 2.0 is compatible with GPL-3.0, but not with GPL-2.0. A copy of
  GPL-2.0 code in this tree would contaminate the license of the whole
  project.

This is not a detail: `win-util.cr`, `at-spi.cr`, `send.cr` and `gui-add.cr`
are ~50 KB of code that **can be read and rewritten**, but not copied. Every
suggestion above was written to be implemented **from scratch**, from the
*description of the behaviour* and from our own design.

---

## 6. A plan by phases

| Phase | Content | Verifiable result |
|---|---|---|
| **F0 — measure** | `ahk --surface`; the table of 725 names with the state | 93/253 today; the number becomes visible in every commit |
| **F1 — what is cheap** | Clipboard, Sound, `NumGet`/`NumPut`, the whole `Files`, `FormatTime`/`Date*` | ~25 names come out of the refusal; the `Clipboard` and the `Sound` get traits of their own |
| **F2 — the hooks** | Hotkey backend (X11 and Wayland), `Hotstring`, the real `Hotkey` class | `Hotkey`, `Hotstring`, `HotIf*` come out of the refusal; a script starts reacting to keys |
| **F3 — the screen** | X11 backend of windows (`xdotool`), trait `Accessibility` by AT-SPI | `WinExist`/`WinActive`/`WinGetText`/`Control*` get a real answer |
| **F4 — the windows** | `Gui`/`Menu`/`ToolTip`/`TrayTip` by GTK4, with `--gui` opt-in | the `Gui` class stops refusing |
| **F5 — the body** | Event loop, cooperative interruption, `OnExit`, `OnMessage` | the JSON report gains the task; the timers become real |
| **F6 — the recipes** | Port the 14 example scripts to v2 and use them as cases | the parity becomes demonstrated, not asserted |
| **F7 — the package** | `--compile`, Window Spy, `--check` | the deliverable is a file |

The phases F0 and F1 are the ones that give the most per hour. F2 is the one
that changes what the project **is**. F4 is the biggest and the most
dependent on the environment.

---

## 7. How to measure the parity

The tree already has the machinery: `crates/ahk/tests/surface.rs` reads the
table of the surface and asks the tree, name by name, if it carries it. The
rule is that the answer may be a value **or the reason**, and a name that the
tree does not carry has to be in the table of pendents.

**The command already exists.** `ahk --surface` prints the state of each v2
name, without a script, and a summary at the end:

```
name                 kind      state
Abs                  function  value
WinMove              command   refusal: this is a name of Windows, and this desktop is not Windows.
Hotkey               command   value
...
the surface of v2 has 725 entries: 479 answer with a value, 246 are refused, 0 are pending.
```

With that, "what is missing" stops being an opinion and becomes a number that
**cannot get worse without being seen**. The JSON for the CI does not exist
yet: it is the same information, in another format, and the command is the
entry point.

A second number deserves attention: **how many names of the surface of v2 are
really implemented** — today 479 of 725 entries. The goal is not 725: v2 has
names that are Windows and that will not happen here. The honest goal is:
*no name that the AHK_X11 does on Linux should be in the refusal for lack of
implementation*. The AHK_X11 does ~127 v1 commands in a comparable
environment; many of them have a direct equivalent in v2. That is the parity
rule we propose.

---

## 8. The sources

- **AHK_X11** — documentation: <https://phil294.github.io/AHK_X11/>;
  code: <https://github.com/phil294/AHK_X11> (branch `master`,
  `shard.yml` version 1.0.7). Read: `README.md`, `docs/list of commands.txt`,
  `src/ahk_x11.cr`,
  `src/run/display/{x11,gtk,at-spi,hotkey,hotstring,display-adapter}.cr`,
  `src/run/thread.cr`, `src/build/{parser,linker}.cr`,
  `src/cmd/**`, `src/util/ahk-bin-var.cr`.
- **AHK (this repository)** — `AGENTS.md` (the working notes, §12–§23),
  `crates/ahk/src/{builtins,desktop,lib}.rs`,
  `crates/ahk/src/classes/*.rs`, `crates/ahk/src/{hotkeys,files,date,settings}.rs`,
  `crates/ahk/src/variables.rs`, `crates/ahk/reference/ahk-v2.0.19.surface.tsv`,
  `crates/ahk-box/src/{desktop,keyboard,pointer,native,raven}.rs`,
  `crates/ahk-cli/src/main.rs`.
- **Measurements of the box** — 2026-10-25, 2026-10-27 and 2026-10-28, on
  192.168.0.40 (CachyOS, Wayland, COSMIC). The full record is in the
  `AGENTS.md` (§13).
- **Counts of this comparison** — 725 entries of surface (644 distinct names):
  242 commands, 103 functions, 131 variables, 249 methods; 346 built-ins in the
  table, 141 with a real handler and 205 with `not_carried`. Of the 725
  entries, 479 answer with a value and 246 refuse with the reason — the
  `ahk --surface` is what counts.
