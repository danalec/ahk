# The refusals of the Rust tree, audited

Audit of what the Rust implementation of AutoHotkey v2.0.19 refuses, and of
whether the refusal is true. Read-only: no source file was changed.

- Worktree: `audit-ahk-x11-parity` (a worktree of the 3-crate workspace).
- Binary: `target/release/ahk.exe`, built from this tree.
- Date of the measurement: 2026-11-09.
- Host of the measurement: Windows. The `--surface` command reads the tables
  and asks the tree, so its answer is host-independent; the runtime probes
  that reach the desktop are marked where the desktop answered instead.
- The companion file is [`rust-refusals.tsv`](rust-refusals.tsv): one row per
  refused name, with the exact reason, a class, a doability verdict and a note.
  The companion finding sheet is [`t2-findings.md`](t2-findings.md).

## 1. The corrected headline numbers

`.\target\release\ahk.exe --surface` last line, verbatim:

```
the surface of v2 has 833 entries: 643 answer with a value, 190 are refused, 0 are pending.
```

| Fact | Reality | How it was measured |
|---|---|---|
| Entries of the surface | **833** | last line of `--surface`; 834 lines of output incl. the summary |
| Distinct full names | **785** | 833 entries − 48 names that appear twice (property + setter pairs) |
| Answer with a value | **643** | last line; recount by kind agrees |
| Refused | **190** | last line; `Where-Object { $_ -match ' refusal: ' }` returns 190 |
| Pending | **0** | last line; `crates/ahk/reference/ahk-v2.0.19.pending.tsv` is empty |
| Commands | **242** (86 value / 156 refused) | group the output by kind and state |
| Functions | **103** (78 value / 25 refused) | same |
| Variables | **131** (122 value / 9 refused) | same |
| Methods | **320** (all value) | same |
| Class names | **37** (all value) | same |
| Rows in the built-in table | **346** | 346 `Builtin {` rows in `crates/ahk/src/builtins.rs` (grep count) |
| Rows with a real handler | **165** | 346 − 181 `handler: not_carried` |
| Rows that refuse | **181** | 181 `handler: not_carried` |

The 181 refused commands and functions of the surface and the 181
`not_carried` rows are the same set — which is the proof in §5 that no
refusal is stale.

## 2. The census by exact reason string

Sixteen distinct reason strings cover the 190 refusals. Grouping matters:
**169 of the 190 share one sentence.**

| # | Reason (exact) | Count | Kind | Who owns it in the code |
|---|---|---|---|---|
| 1 | `this is a name of Windows, and this desktop is not Windows.` | **169** | command ×147, function ×20, variable ×2 | `builtins.rs:3781` (variant 0) + `variables.rs:90` for `ProgramFiles`, `WinDir` |
| 2 | `the registry is a thing of Windows, and this runtime does not carry it.` | **5** | function ×5 | `builtins.rs:3823` (variant 23) |
| 3 | `this runtime has no tray icon.` | **3** | variable ×3 | `variables.rs:142` |
| 4 | `this runtime reads no text of a window.` | **1** | command | `settings.rs:147-149` (variant 21) |
| 5 | `this desktop does not know the scale of the output.` | **1** | variable | `variables.rs:137` |
| 6 | `this runtime has no window of its own: it is a script that runs and ends.` | **1** | variable | `variables.rs:131` |
| 7 | `this runtime is not AutoHotkey, and it has no path of an executable.` | **1** | variable | `variables.rs:126` |
| 8 | `the shape of the cursor of the machine is not read by this runtime.` | **1** | variable | `variables.rs` (own const) |
| 9 | `this runtime cannot stop itself: it would never end.` | **1** | command | `builtins.rs:3804` (variant 13) |
| 10 | `this runtime cannot wait for a key to continue: the script ends when it ends.` | **1** | command | `builtins.rs:3801` (variant 10) |
| 11 | `this runtime is a script that ends when it ends, and it cannot be kept alive.` | **1** | command | `builtins.rs:3802` (variant 11) |
| 12 | `this runtime runs one script once, and it cannot run itself again.` | **1** | command | `builtins.rs:3803` (variant 12) |
| 13 | `this runtime runs one script in one thread, and it cannot make it critical.` | **1** | command | `builtins.rs:3799` (variant 8) |
| 14 | `this runtime runs one script in one thread, and it cannot start another.` | **1** | command | `builtins.rs:3800` (variant 9) |
| 15 | `the lines are listed in a window of v2, and this runtime has no window to show them in.` | **1** | command | `builtins.rs:3806` (variant 14) |
| 16 | `the variables are listed in a window of v2, and this runtime has no window to show them in.` | **1** | command | `builtins.rs:3809` (variant 15) |

The members of group 1 (169) are exact: `Control*` (32), `Edit*` (7), `Drive*`
(14), `Process*` (8), `Win*` (44), `Monitor*` (5), `IL_*` (3), `Group*` (4), the
COM/pointer/string natives (`ComCall`, 8 `ComObj*`, 5 `Obj*`, `StrGet`/`StrPtr`/
`StrPut`, `CallbackCreate`/`CallbackFree` = 19), `SoundGetInterface`/
`SoundGetName`, `BlockInput`, the three lock-key commands, `RunAs`, `Shutdown`,
`SysGet`/`SysGetIPAddresses`, `TraySetIcon`/`TrayTip`, `ToolTip`, `OutputDebug`,
`CaretGetPos`, `OnMessage`/`PostMessage`/`SendMessage`, `OnClipboardChange`, the
five Windows-format/recycle `File*` names, `MenuFromHandle`/`MenuSelect`,
`StatusBarGetText`/`StatusBarWait`, `ListViewGetContent`, `GuiCtrlFromHwnd`/
`GuiFromHwnd` and `ProgramFiles`/`WinDir`. 32+7+14+8+44+5+3+4+19+2+1+3+1+1+2+2+1+1+1+3+1+5+2+2+1+2+2 = 169;
the TSV lists them name by name with the same sentence in every row.

A note on the shape of the refusal table: the reason is picked by the
**variant byte** of the row (`builtins.rs:3754`, `REFUSAL = 100`;
`builtins.rs:3768-3773`), not by the handler. `crates/ahk/src/surface.rs:273-276`
is the only place that turns a row into a `value`/`refusal` state, and it asks
`builtins::refusal_of_the` — so the surface cannot disagree with the runtime
about *whether* a name refuses.

## 3. The census by area

Area is the one in the `note` column of the TSV (`[area]` prefix).

| Area | Refused | Value | Class mix | Doability |
|---|---|---|---|---|
| windows | **52** | 3 (`WinExist`, `WinActive`, `WinGetTitle`) | 25 windows-only / 27 missing | 9 yes, 10 likely, 33 no |
| controls | **48** | 0 | 11 windows-only / 37 missing | 37 likely, 11 no |
| machine | **40** | many (`A_ScreenWidth`, `Run`, `Sleep` …) | 2 windows-only / 10 subsystem / 28 missing | 10 yes, 16 likely, 14 no |
| registry/COM | **14** | 0 | 14 windows-only | 14 no |
| native | **10** | `DllCall`, `NumGet`, `NumPut`, `ObjAddRef`, … | 5 subsystem / 5 missing | 5 likely, 5 no |
| processes | **9** | `Run`, `RunWait` | 9 missing | 8 yes, 1 likely |
| files | **5** | 26 of the 31 `File*`/`Dir*`/`Ini*` names | 3 windows-only / 2 missing | 2 likely, 3 no |
| hotkeys | **4** | `Hotkey`, `Hotstring`, `HotIf*`, `GetKey*` … | 1 windows-only / 3 missing | 3 yes, 1 no |
| GUI | **3** | **148 method rows of the `Gui` family** | 2 windows-only / 1 missing | 1 likely, 2 no |
| screen | **2** | `PixelGetColor`, `PixelSearch`, `A_ScreenWidth` | 1 windows-only / 1 missing | 1 likely, 1 no |
| sound | **2** | `SoundPlay`, `SoundBeep`, `SoundGetVolume`, `SoundSetMute` … | 1 windows-only / 1 missing | 1 yes, 1 no |
| clipboard | **1** | `Clipboard`, `ClipWait`, `ClipboardAll` | 1 missing | 1 likely |
| timers | **0** | `SetTimer`, `Sleep` | — | — |
| dialogs | **0** | `MsgBox`, `InputBox`, `FileSelect`, `DirSelect` | — | — |
| network | **0** | `Download` | — | — |
| pictures | **0** | `LoadPicture`, `ImageSearch` | — | — |

Totals: **190 refused**, of which **60 windows-only**, **15 subsystem-absent**,
**115 missing-linux-impl**; **31 yes**, **74 likely**, **85 no** doable on
Linux/Wayland.

### The verdict, area by area

**windows — mostly gratuitous.** 3 window names work (`WinExist`,
`WinActive`, `WinGetTitle`) through `ext-foreign-toplevel-list-v1` +
`zcosmic-toplevel-info-v1` (`crates/ahk-box/src/wayland/linux.rs:122,142`).
The protocols the tree already speaks carry more than it uses: the listing has
a stable `identifier` per window and a `close` request, and the COSMIC handle
reports a `geometry` rectangle (`protocols/cosmic-toplevel-info-unstable-v1.xml:229-243`)
and an `activated`/`fullscreen` state (`:215-217`). That makes `WinGetID`,
`WinGetList`, `WinGetCount`, `WinGetClass` (app_id), `WinClose`, `WinGetPos`,
`WinGetClientPos`, `WinGetMinMax`, `WinWait`, `WinWaitActive`, `WinWaitClose`,
`WinWaitNotActive` and the four `Group*` names implementable now. The rest is
genuinely blocked: no protocol exposes z-order, a pid, a title write, a move,
a show/hide, or a Win32 style word. `WinGetText` is a separate sentence
(`settings.rs:147-149`) and is doable only through AT-SPI, which the tree does
not speak.

**controls — gratuitous, and it is the biggest mechanical gap.** All 48 refuse
with the Windows sentence, but 37 of them are answerable through AT-SPI
(the accessibility bus): text, value, state, selection, position, focus, click.
That is exactly how AHK_X11 does it on X11 (the doc itself cites
`src/run/display/at-spi.cr`). The 11 that are honestly Windows-only are the
ones that name an HWND or a Win32 style word: `ControlGetHwnd`,
`ControlGetClassNN`, `ControlGet/SetStyle`, `ControlGet/SetExStyle`,
`GuiFromHwnd`, `GuiCtrlFromHwnd`, and the three `IL_*` image-list calls.

**machine — a mixed bag.** Doable now: `A_AhkPath` (the runtime *does* have an
executable — `std::env::current_exe`), `SysGetIPAddresses` (`getifaddrs`, the
same kind of C call `sys.rs` already declares), `OutputDebug` (a write to
stderr / to the report), the `Drive*` capacity/filesystem/space/type/list
family (`statvfs`, `/proc/mounts`), `MonitorGetCount`, and `Suspend` (a flag
the hotkey dispatcher already present in `crates/ahk-box/src/hotkeys.rs` would
consult). Likely: label/serial/eject/lock (`udisks2`, `blkid`), `Shutdown`
(`systemctl`), `TrayTip` (`org.freedesktop.Notifications`), `SysGet`,
`ListLines`/`ListVars` (they need a *format*, not a window — the runner already
has an output channel). Genuinely absent: the tray (`IconFile`, `IconNumber`,
`TrayMenu`, `TraySetIcon`), the runtime's own window (`ScriptHwnd`), threads
(`Thread`, `Critical`), a persistent script (`Persistent`, `Pause`, `Reload`),
and the two Windows folder names (`ProgramFiles`, `WinDir`).

**hotkeys — 3 cheap wins.** The uinput keyboard already resolves and presses
`CapsLock`, `NumLock` and `ScrollLock` (`crates/ahk/src/keys.rs:171,770-772`),
so `SetCapsLockState`, `SetNumLockState` and `SetScrollLockState` are a loop
over `press_key`. `BlockInput` is not: on Wayland a client cannot block the
user's input (AHK_X11 has `XGrabKeyboard`, which Wayland deliberately removes).

**processes — the cheapest cluster.** All 9 are Linux-doable with `/proc`,
`kill(2)` and `renice(2)`; the only missing piece is that the `Processes` trait
(`crates/ahk/src/desktop.rs:237-282`) has no method for any of them — it only
has `run`, `launch`, `run_and_wait`, `env_get`, `env_set`. `RunAs` is "likely":
`sudo`/`pkexec` carry it, with a policy this runtime does not have.

**files — 2 likely wins.** `FileRecycle`/`FileRecycleEmpty` map to the XDG trash
specification (`gio trash`, `trash-cli`); `crates/ahk-box/src/files.rs` has no
trash code at all (no match for "trash"). The three Windows-format names are
honest refusals: `.lnk` (`FileCreateShortcut`, `FileGetShortcut`) and the PE
version resource (`FileGetVersion`).

**sound — 1 cheap win.** `SoundGetName` is missing only the name-by-number
lookup: the tree already lists the sinks through `pactl`
(`crates/ahk-box/src/desktop.rs:1375`). `SoundGetInterface` is a COM interface
and is honestly Windows-only.

**clipboard — 1 gratuitous refusal.** `OnClipboardChange` is refused with the
Windows sentence while the clipboard capability itself is implemented
(`wl-paste`/`wl-copy`, `crates/ahk-box/src/desktop.rs:367-446`). `wl-paste
--watch` provides exactly the change events the name wants.

**screen — 1 likely win, 1 honest denial.** `A_ScreenDPI` has no answer today,
but `wl_output.scale` is core Wayland; the client binds no `wl_output` yet
(`wayland/linux.rs` binds only the two toplevel interfaces). `A_Cursor` is an
honest denial: a Wayland client is never told the cursor image.

**native — half gratuitous.** `StrGet`, `StrPut`, `StrPtr` are cheap: the
string marshalling of `DllCall` already exists (`dll.rs` encode/decode,
`native.rs:731` `NativeType::AStr`), and `NumGet`/`NumPut` already read and
write a `Buffer` (verified: `NumPut("Int",42,b)` then `NumGet(b,"Int")` answers
42). `CallbackCreate`/`CallbackFree` are "likely" — the tree already builds
call trampolines. The five `Obj*` pointer names are honestly impossible: this
runtime's values have no address to hand out.

**registry/COM — honest.** 14 of 14 are Windows subsystems with no Linux
equivalent (COM vtable calls, `QueryInterface`, HKLM). Refusing is right.

**GUI — the worst gap in the surface, and it is invisible there.** Not one
`Gui` name is refused: all 59 `Gui.*`, 29 `Gui.Control.*`, 19 `Gui.Menu.*`,
15 `Gui.ListView.*`, 15 `Gui.TreeView.*` and 11 `Gui.StatusBar.*` method rows
are reported as `value`. The real answer is in `crates/ahk/src/classes/gui.rs:215-258`:
the object is made and every window/menu/control call fails with "a window is
a Windows window, and this desktop is a Wayland session without one".
Verified on `Show`, `Add`, `Title` (read and set), `MarginX` (set), `Destroy`,
`Opt` and `SetFont` — every one refuses with that sentence, each naming the
member (`"Gui cannot show the window: …"`, `"Gui cannot read or set the title
of the window: …"`). A Wayland client *can* make windows (`xdg-shell`,
`layer-shell`), so this is a missing implementation, not an impossible one;
`ToolTip` is the same story.

**timers, dialogs, network, pictures — no refusals at all.** They are carried:
`SetTimer`/`Sleep` by the runtime's own clock, the three dialogs through
`zenity` (`Dialogs`), `Download` through `curl`
(`crates/ahk-box/src/network.rs:16`), the pictures through the `png` crate and
`grim`. This is what makes several entries of
`docs/ahk-x11-comparison.md` (§4.1: "Sound ❌ refusal (variant 0)",
"GUI/dialogs ❌ refusal (variant 0)", "INI ❌ refusal (variant 4)") stale.

## 4. The capability traits: fifteen, not ten

`AGENTS.md` is right and `docs/ahk-x11-comparison.md` is wrong.

`crates/ahk/src/desktop.rs:920-936`:

```rust
pub trait Desktop:
    Keyboard + Pointer + Screen + Windows + Processes + Files + Clipboard
    + Timers + Hotkeys + Native + Sound + Machine + Dialogs + Network
    + Pictures
{}
```

**Fifteen** capability traits. The comparison doc (`:343-346`) says "the sum of
**ten** capabilities" and lists `Keyboard`, `Pointer`, `Screen`, `Windows`,
`Processes`, `Files`, `Clipboard`, `Timers`, `Hotkeys`, `Native` — it is missing
the five that were added when the sound, the machine, the dialogs, the network
and the pictures were implemented: **`Sound`, `Machine`, `Dialogs`, `Network`,
`Pictures`**. `AGENTS.md` ("the sum of fifteen capability traits") is accurate.

The same paragraph also says "every implementor writes the ten", which the code
contradicts twice over: the capability methods have **defaults that refuse**
(`desktop.rs:46-58`, `96-117`, …), and the X11 implementor relies on them —
`crates/ahk-box/src/x11.rs:1389-1398` is

```rust
impl Files for X11Desktop {}
impl Clipboard for X11Desktop {}
impl Timers for X11Desktop {}
impl Hotkeys for X11Desktop {}
impl Native for X11Desktop {}
impl Sound for X11Desktop {}
impl Machine for X11Desktop {}
impl Dialogs for X11Desktop {}
impl Network for X11Desktop {}
impl Pictures for X11Desktop {}
```

so the X11 desktop writes five capabilities (Keyboard, Pointer, Screen, Windows,
Processes) and inherits ten refusing defaults. The Linux `LabDesktop` does write
all fifteen, but in three files: 13 impls in `ahk-box/src/desktop.rs`, `Files` in
`ahk-box/src/files.rs:40`, `Pictures` in `ahk-box/src/pictures.rs:189`.

## 5. Stale refusals: none — and the proof

A stale refusal would be a name the surface calls refused while the code
implements its handler. There are **zero**, and it can be shown by counting
rather than by sampling:

- Every refused command/function state comes from a row whose `variant >= 100`
  (`builtins.rs:3768-3773`), and every such row's handler is `not_carried`
  (`builtins.rs:3837-3848` refuses with the sentence of the variant).
- `builtins.rs` has **181** `handler: not_carried` rows (grep count) and exactly
  **346** `Builtin {` rows (grep count).
- The surface refuses **181** commands and functions (156 commands + 25
  functions) and 9 variables = 190.
- 181 refused built-ins = 181 `not_carried` rows. If a row had a real handler
  but a refusal variant, the two counts would differ by one. They do not, and
  the one row of the table that is not a v2 name (`Screenshot`,
  `builtins.rs:665`; `crates/ahk/tests/surface.rs:33-38` names it as the one
  dialect name v2 does not have) carries the real `screenshot` handler, so it is
  not among the 181.

The 9 refused variables are the same story: `variables::lookup` returns the
reason for exactly those names (`variables.rs:90-142`), and the surface asks
`dialect.refusal_of_variable` (`surface.rs:286-289`).

**What does exist is the inverse drift, and it is worth stating:** the surface
*under*-reports the GUI gap (§3), and two small inconsistencies sit next to the
refusals:

1. `ObjAddRef` is reported as `value` while `ObjRelease` is refused as Windows;
   `ObjAddRef()` on an object fails at runtime with
   "Parameter #1 of ObjAddRef is invalid." So the pair is neither both carried
   nor both refused.
2. `Screenshot` is in the built-in table but is **not** a v2 name — it is the
   one dialect name v2 does not have (`crates/ahk/tests/surface.rs:33-38`
   asserts exactly that). It does not distort the v2 count because the surface
   table is the list of names; it does mean the table carries 346 rows for 345
   v2 commands and functions.

## 6. The counts of `docs/ahk-x11-comparison.md`, verified one by one

Every number below was checked against `--surface` or against the source.

| Doc claim | Where | Reality | Verdict |
|---|---|---|---|
| "725 entries of the surface" | :28, :336, :772, :781, :807 | **833** | wrong, stale |
| "644 distinct names" | :337, :807 | **785** | wrong, stale |
| "506 answer with a value" | :772, :781, :809 | **643** | wrong, stale |
| "219 are refused" | :772, :810 | **190** | wrong, stale |
| "93 built-ins with a real implementation" | :28, :355 | **165** | wrong, stale |
| "253 with a motivated refusal" | :28, :356 | **181** | wrong, stale |
| "346 entries" in the built-in table | :350, :808 | **346** | **right** |
| "141 with a real handler and 205 `not_carried`" | :809 | **165 / 181** | wrong, stale (and contradicts the doc's own 93/253) |
| "242 commands, 103 functions, 131 variables" | :808 | **242 / 103 / 131** | **right** |
| "249 methods" | :808 | **320**, plus **37** class rows the doc does not count | wrong, stale |
| "sum of ten capabilities" and the list | :343-346 | **fifteen** (see §4) | wrong, stale |
| "every implementor writes the ten" | :345-346 | traits have refusing defaults; `X11Desktop` writes five and inherits ten | wrong |
| "the five reasons of `not_carried`, variants 0-4" | :358-366 | **22** numbered reasons; **11** are used by the v2 surface; of the doc's five, **only variant 0 is used** (1, 2, 3, 4 are dead code) | wrong, stale |
| "`Persistent`, `Reload`, `Suspend`, `Pause`, `OnExit`" as variant 3 | :365 | `Persistent`/`Reload`/`Suspend`/`Pause` carry their **own** variants (11, 12, 13, 10) and `OnExit` is a value; variant 3's sentence has no user at all | wrong |
| "`FileCopy`, `FileMove`, `FileGetSize`, `IniRead`, `IniWrite`, `DirCopy` — variant 4" | :366 | all six are **values**; variant 4 has no user at all | wrong |
| "`FormatTime`, `DateAdd`, `DateDiff` — variant 2" | :364 | all three are **values** | wrong |
| "`Hotkey`, `Hotstring`, `HotIf`, `HotIfWinActive` — variant 1" | :363 | all four are **values** (hotkeys are implemented); variant 1 has no user | wrong |
| JSON report keys `ok, steps, output, notes, error, error_line, exit_code, elapsed_ms, variables` | :404-405 | the real report has **10** keys: `ok, steps, output, warnings, notes, error, error_line, exit_code, elapsed_ms, variables` — `warnings` is missing from the list | wrong |
| "Windows: `xprop` … `WinExist`/`WinActive` → *unknown*" | :397 | the Wayland path answers (verified on the box in `AGENTS.md` §7); the X11 path is what answers "unknown" | stale |
| "`crates/ahk/src/hotkeys.rs` — sketch, `run_fired` returns `Ok(())`" | :413 | `run_fired` is a full dispatcher (`hotkeys.rs:284-334`), and `ahk-box/src/hotkeys.rs` reads `/dev/input/eventN` | stale |
| "`crates/ahk/src/files.rs` — sketch (only the header)" | :415 | 1742 lines, and `ahk-box/src/files.rs:40` implements the whole `Files` trait | stale |
| "`date.rs`: `system_wall_clock()` returns `None`" | :417 | a wall clock is carried and `FormatTime`/`DateAdd`/`DateDiff` are values | stale |
| "The 131 variables of v2 are all carried" | :384 | 131 entries, of which **122 value / 9 refused** — carried, yes; answered, no | imprecise |

So of the doc's numbers, **four are right** (346 built-ins, 242 commands, 103
functions, 131 variables) and **everything else in the measurement sections is
stale**. The doc appears to have been written before the wave that added
hotkeys, files, dates, sound, dialogs, pictures, network and the Wayland window
listing — and it was not regenerated.

The sibling document is stale too, but by much less:
`docs/v2-capability-report.md:5` says "**828 entries** : **638 answer with a
value**, **190 are refused**" (real: 833 / 643 / 190), `:7` says "780 distinct
names" (real: 785), and `:119` lists 109 members under the Windows sentence
(real: 169). Its refused count — the one number that matters most — is still
right.

## 7. The variant table behind the reasons

`refusal_message` (`crates/ahk/src/builtins.rs:3779-3829`) has 22 numbered arms
(0–6, 8–21, 23; 7 and 22 do not exist) plus the fallback
`dialect::NOT_CARRIED_YET`.

- **11 are used** by the v2 surface: 0 (169 names), 8 `Critical`, 9 `Thread`,
  10 `Pause`, 11 `Persistent`, 12 `Reload`, 13 `Suspend`, 14 `ListLines`,
  15 `ListVars`, 21 `WinGetText`, 23 `Reg*`.
- **11 are dead**: 1 "no hotkeys yet", 2 "the clock … does not know the date",
  3 "one script that ends when it ends", 4 "cannot serve that with the files it
  has", 5 "no setting of that kind", 6 "a method is not bound to an object",
  16 "the question of the user is asked in a window", 17 "does not reach the
  network", 18/19 "does not read the images", 20 "the handler of an error is not
  carried". Each of these was true once and was left behind when the capability
  landed; the comments at `builtins.rs:3791-3798` admit the same for 6.

The doc's "five reasons, variants 0-4" is therefore a snapshot of a table that
has since grown and shrunk: 4 of its 5 entries are now unreachable code.

## 8. The report schema

The real output of an empty script, from
`'' | .\target\release\ahk.exe`:

```json
{"ok":true,"steps":[],"output":[],"warnings":[],"notes":["no uid on this platform; the session environment is unknown"],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":0,"variables":{}}
```

Ten keys: `ok`, `steps`, `output`, **`warnings`**, `notes`, `error`,
`error_line`, `exit_code`, `elapsed_ms`, `variables`. The doc's list
(`:404-405`) omits `warnings`. `docs/runner-and-report.md:119-127` has it right,
and so does the example at `:45`.

Two smaller drifts in the same area:

- `docs/runner-and-report.md:48` says the keys "come out in alphabetical order".
  The real stdout is in struct order (`ok` first, `elapsed_ms` near the end);
  the alphabetic examples in that file are re-serialised (they are the shape a
  `jq`-style sorter produces), not the bytes the runner writes.
- The parse-error payload is right: `'@' | ahk.exe` answers
  `{"ok":false,"error":"unexpected character '@'","error_line":1,"column":1,"steps":[],"output":[],"notes":[]}`
  — no `warnings`, no `elapsed_ms`, no `exit_code`, no `variables`, as
  `runner-and-report.md:97-98` says.
- `warnings` is a real, populated array — the type is
  `{"line": N, "message": "..."}` per `runner-and-report.md:122` — it is simply
  empty for scripts that raise none.

## 9. What this means for the port

Ranked by value per unit of work (the full evidence for each is in the TSV):

1. **`ProcessExist` / `ProcessGetName` / `ProcessGetParent` / `ProcessGetPath` /
   `ProcessClose` / `ProcessSetPriority` / `ProcessWait` / `ProcessWaitClose`**
   — 8 names, one new capability trait method set, `/proc` + `kill` + `renice`.
2. **`SetCapsLockState` / `SetNumLockState` / `SetScrollLockState`** — 3 names,
   a `press_key` each; the key codes are already there.
3. **`OnClipboardChange`** — 1 name, `wl-paste --watch`, over a clipboard that
   already works.
4. **`SoundGetName`** — 1 name, the sink list is already read.
5. **`StrGet` / `StrPut` / `StrPtr`** — 3 names, over the encoder of `DllCall`
   and the `Buffer` that `NumGet`/`NumPut` already use.
6. **`WinGetID` / `WinGetList` / `WinGetCount` / `WinGetClass` / `WinClose` /
   `WinWait*` / `Group*`** — 13 names, over the two protocols the client
   already binds; `WinGetPos`/`WinGetClientPos`/`WinGetMinMax` (3 more) need the
   COSMIC geometry/state events.
7. **`A_ScreenDPI`** — 1 name, `wl_output.scale`, one more binding.
8. **`A_AhkPath`, `SysGetIPAddresses`, `OutputDebug`, `ListLines`, `ListVars`,
   `Suspend`, `TrayTip`** — 7 names, all trivial or nearly so.
9. **`Drive*`** — 14 names, one module over `statvfs` + `/proc/mounts` +
   `udisks2`/`blkid`.
10. **`Gui`/`Gui.*`** — 148 method rows that the surface already counts as
    `value` and that all fail. This is the largest gap in the tree and the
    largest error in the headline number's *meaning*.

Impossible, and it should stay refused: the 14 registry/COM names, the COM
sound interface, the eight HWND/style control names, the three Windows-format
file names (`.lnk` ×2, PE version ×1), the Win32 window-style/message names,
`A_Cursor`, `BlockInput`, `ProgramFiles`, `WinDir`, the `Obj*` pointer family,
and the runtime-shape names
(`Thread`, `Critical`, `Persistent`, `Pause`, `Reload`, `ScriptHwnd`, the tray).

## 10. Caveats

- The runtime probes ran on the Windows host. A probe that needed the desktop
  (`A_ScreenWidth`, `Hotkey`) failed at the desktop layer, which is a property
  of the host and not of the refusal; those two are marked in §3 by what the
  code and `AGENTS.md` §7/§13 record for the box.
- The doability verdicts rest on what Linux and Wayland publish
  (AT-SPI, `wl_output`, `ext-foreign-toplevel-list`, `udisks2`, `/proc`,
  `statvfs`, XDG trash) and on what this tree already does. "likely" means the
  capability exists but the mechanism would have to be written and, for the
  control family, that COSMIC's own accessibility support would have to be
  checked on the box. It is a judgement, and it is marked as one.
