# T3 — what the v2 surface actually does at runtime

Empirical audit of the release runner against the 17 feature families that
`AHK_X11` implements. Every claim below is a report read out of the binary,
not a documentation claim.

## 1. What was run, and where

| Item | Value |
|---|---|
| Worktree | `.agents/worktrees/audit-ahk-x11-parity`, commit `f47cbd6` |
| Binary | `target\release\ahk.exe`, SHA-256 `D6427075DA7025CC9D7565BFD21914D2B9F1093783054150D4B03863F20A8DD8` |
| Host | **Windows 11 Pro N, build 26300, 64-bit** (not the Linux box) |
| Dialect | v2 (`--dialect=v2`, the default) |
| Probes | 50 probe scripts in `audit/empirical/probes/` (plus `lib-probe.ahk`, the file `include-ok.ahk` includes), each with its `.json` report beside it |
| Runner flags | `--timeout=5` everywhere; a hard 25 s kill on top (no probe was killed) |
| Name-level rows | 192 in `audit/empirical-results.tsv` |

**The host matters, and it is on every row.** A refusal that names `grim`,
`xclip`, `pactl`, `zenity`, `uinput` or `HOSTNAME` is a **Linux-shaped
refusal given on Windows**, and it is marked platform-dependent in the TSV.
The comparison doc's §3.3 is a measurement *of the Linux box*; nothing here
refutes it, and nothing here confirms it either. What this audit establishes
is what the runtime does on the host it was run on, and how the desktop
primitives behave when the host is not Linux.

## 2. Headline numbers

| Verdict | Rows |
|---|---|
| `value` — answered with a usable value | 78 |
| `refusal` — refused with a reason | 99 |
| `silent-wrong` — answered, but wrongly or without a cause | 11 |
| `not-probed` — the name does not exist in v2 | 4 |

By family:

| State | Families |
|---|---|
| **Really works** | 4 INI (3 value), 17 Output (4 value) — every probed name answers with a real value |
| **Works with a hole** | 3 Files (16 value / 5 refusal / 3 silent-wrong), 10 Time (6/1), 11 Binary (4/3), 13 Native (4/3), 14 Regex (9/2), 9 Processes (4/8), 15 A_* (14/10/2), 16 Directives (5/5/3), 12 Screen (1 value only) |
| **Constructs, then refuses** | 7 GUI/dialogs (3 value / 11 refusal) — `Gui()` and `Menu()` objects are made; every operation on them refuses; `MsgBox` works |
| **Refuses as a family** | 1 Clipboard (1 value / 5 refusal), 2 Sound (7), 5 Windows (21), 6 Controls (8), 8 Hotkeys/hotstrings (2 value / 7 refusal) |
| **Silently accepts** | 8 Hotkeys — `HotIf()` and `HotIfWinActive()` return `""` and say nothing (the `#HotIf` directive refuses) |
| **Extra, for §3.3** | keyboard/pointer (2 value / 1 refusal / 1 silent-wrong) |

**The worst finding is `ImageSearch`: it fails with an empty message.**
`class Error`, `What "ImageSearch"`, `Extra ""`, `Message ""` — length 0. A
program cannot distinguish it from a bug in its own code. See §6.

## 3. The report schema actually observed

The report of a run is one line of JSON with ten keys, in alphabetical order,
exactly as `docs/runner-and-report.md` describes — `elapsed_ms`, `error`,
`error_line`, `exit_code`, `notes`, `ok`, `output`, `steps`, `variables`,
`warnings`. The shape, with the step array elided:

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":null,
 "notes":["no uid on this platform; the session environment is unknown"],
 "ok":true,"output":["FileExist=A;"],"steps":[...],"variables":{"a_linenumber":14},
 "warnings":[]}
```

Three shapes were confirmed by observation:

- **A run** has all ten keys.
- **A failure of the load** (`#Include` of a missing file) has the reduced
  payload the doc describes — `ok`, `error`, `error_line`, `column`,
  `steps`, `output`, `notes` — with `error_line` 0 and `column` 0, and the
  process exits 2.
- **The notes** on this non-Linux host are always
  `no uid on this platform; the session environment is unknown`, and a
  runtime note is appended after it when there is one (`MouseGetPos`).

The process exit codes observed are the documented ones: 0 for a run, 1 for
a failed run (`#Requires`, `Hotkey`), 2 for a load failure (`#Include`).

**Documentation drift in the surface count**: the binary prints

```
the surface of v2 has 833 entries: 643 answer with a value, 190 are refused, 0 are pending.
```

while `docs/runner-and-report.md` and `docs/v2-capability-report.md` say 828
entries and 638 values.

## 4. What really works, with the values observed

- **INI** — `IniWrite`, `IniRead` (with a default for a missing key),
  `IniDelete`: real reads and writes of a real file.
- **Output** — `FileAppend "x", "*"` and `MsgBox` both land in `output`, in
  call order, and `FileOpen("*", "w")` opens the console as a file
  (`Write` returned 18 and the text appeared). `FileAppend` returns `""`.
- **Files, the `std::fs` core** — `FileCopy`, `FileMove`, `FileDelete`,
  `FileRead`, `FileAppend` (the `"UTF-8"` encoding argument is accepted),
  `FileGetSize` (9), `FileGetTime` (`20261109194238`), `FileSetTime` (real
  write-back), `FileGetAttrib` (`A`, `RA`), `FileSetAttrib` (real
  write-back), `DirCreate`, `DirDelete` (empty folder), `DirExist` (`D`),
  `DirCopy`, `DirMove`, `FileExist` (`A`/`D`). All verified against the
  disk, not just against the return value.
- **Time** — `FormatTime` (`2026-11-09 19:44:36`), `A_Now`, `A_Year`,
  `A_TickCount` (moved 5 ms across `Sleep 5`), `DateAdd`/`DateDiff` **for
  the single-letter units only**.
- **Binary/memory** — `Buffer(16, 0)`, `NumPut("Int", 7, b, 0)`,
  `NumGet(b, 0, "Int")` → 7, `VarSetStrCapacity(&s, 64)` → 64.
- **Native** — `DllCall` really calls: `GetTickCount` 10645265,
  `GetCurrentProcessId` 24792, `GetSystemMetrics(0)` 3840. `ObjGetBase`,
  `ObjSetBase`, `ObjOwnPropCount` work.
- **Regex/strings** — `RegExMatch` (position 13), `RegExReplace` (with
  `$2 $1` back references), `InStr` (7; 3 case-sensitive), `StrSplit`
  (Array of 3), `Sort` (plain), `Format` (`a-3.14`), `StrReplace`, `SubStr`,
  `StrLen`.
- **Processes/machine** — `Run` starts a process, `RunWait` returns the real
  exit code (7), `EnvGet`/`EnvSet` read and write the run's environment.
- **A_\* variables** — `A_AhkVersion` 2.0.19, `A_Now`, `A_Year`,
  `A_TickCount`, `A_ScriptDir`, `A_ScriptName` (`*`), `A_WorkingDir`,
  `A_Temp`, `A_Space`, `A_Tab`, `A_LineNumber`, `A_PtrSize` 8,
  `A_KeyDelay` 10, `A_CoordModeMouse` (`Client`).
- **Directives** — `#Requires AutoHotkey v2`, `#SingleInstance Force`,
  `#NoTrayIcon`, `#Warn` load. `#Include` of a file that exists works.
- **Dialogs** — `InputBox`, `FileSelect` and `DirSelect` fail *fast* with
  `zenity is not on this machine, and the box has no other dialog`; they do
  not block, which is the good news about them.

## 5. What refuses with a reason — the exact strings

The 99 refusals fall into a small number of sentences. These are verbatim.

**A. Windows-shaped (39 rows) — the single most common refusal, including
on this Windows host.**

```
this is a name of Windows, and this desktop is not Windows.
```

Names: `WinActivate`, `WinMove`, `WinClose`, `WinShow`, `WinHide`,
`WinMinimize`, `WinMaximize`, `WinSetTitle`, `WinWait`, `WinWaitActive`,
`WinWaitClose`, `WinSetAlwaysOnTop`, `WinGetPos`, `WinGetList`,
`WinGetClass`, `WinGetPID`, `ControlGetText`, `ControlSetText`,
`ControlGetPos`, `ControlClick`, `ControlSend`, `ControlFocus`,
`ControlGetFocus`, `ControlMove`, `ToolTip`, `TrayTip`, `OnClipboardChange`,
`ComCall`, `StrPtr`, `SysGet`, `ProcessExist`, `ProcessGetName`,
`ProcessSetPriority`, `ProcessClose`, `ProcessWait`, `ProcessWaitClose`,
`FileGetVersion`, `FileRecycle`, `FileCreateShortcut`.

**B. Linux-shaped, given on Windows (platform-dependent).**

```
the windows of the desktop are a thing of the Linux box: this host has neither the compositor of a Wayland session nor an X server to ask.
WinExist cannot answer: the windows of the desktop are a thing of the Linux box: this host has neither the compositor of a Wayland session nor an X server to ask.
the hotkeys of this box watch a Linux device; this is not Linux
the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine
the mixer refused: sh: line 0: pactl: not found
the mixer could not play "/tmp/ahk-tone-1794253358032.wav": sh: line 0: paplay: not found
the mixer could not play "C:\\ahk-no-such-file-zzz.wav": sh: line 0: paplay: not found
grim failed: sh: line 0: grim: not found
zenity is not on this machine, and the box has no other dialog
the session does not carry the name of the computer: the environment has no HOSTNAME.
the session does not carry the name of the user: the environment has no USER.
the session does not carry the language: the environment has no LC_ALL, LANG or LANGUAGE, and /etc/locale.conf is not there.
the version of the operating system is the release of the kernel, and this is not Linux.
the rights of an administrator are the effective user id, and this is not Linux.
```

**C. The runtime's own reasons — honest and specific.**

```
this runtime does not read the memory of an address: the source of a number is a buffer.
this runtime does not write to the memory of an address: the target of a number is a buffer.
this runtime reads no text of a window.
the window to ask about is not carried yet: only the active one is.
this runtime does not carry that class yet.
this runtime cannot stop itself: it would never end.
this runtime cannot wait for a key to continue: the script ends when it ends.
this runtime is a script that ends when it ends, and it cannot be kept alive.
this runtime runs one script once, and it cannot run itself again.
this runtime has no window of its own: it is a script that runs and ends.
this runtime does not keep the keys that are down, so it cannot wait for one.
the matches of the pattern are not carried into a variable yet.
the options of the sort are not carried yet.
the options of the window and the pid of the process are not carried yet.
the COM objects are a thing of Windows, and this runtime does not carry them.
Gui cannot show the window: a window is a Windows window, and this desktop is a Wayland session without one.
Gui cannot add a control to the window: a window is a Windows window, and this desktop is a Wayland session without one.
Gui cannot destroy the window: a window is a Windows window, and this desktop is a Wayland session without one.
Menu cannot add an item to the menu bar: a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one.
Menu cannot show the menu bar: a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one.
Menu cannot delete an item of the menu bar: a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one.
Parameter #1 of Hotstring is invalid.
Parameter #3 of DateAdd is invalid.
Parameter #3 of DateDiff is invalid.
This script requires AutoHotkey v2.0. This interpreter is v2.
the hotkeys are not carried by this runtime.
the include "".\\ahk-probe-no-such-include.ahk"" cannot be found.
Variable not found.
this file is not a PNG
Failed to load DLL.
```

Note the last four groups overlap: the **same capability** is refused with a
Windows-shaped sentence by one name and a Wayland-shaped sentence by another.
`WinActivate` says "this desktop is not Windows" while `WinActive` says
"the windows of the desktop are a thing of the Linux box" — on the same host,
in the same run. The messages are per-handler, not derived from one
platform check.

## 6. What silently misbehaves — the worst case

These are the rows marked `silent-wrong`: the call succeeded, and the answer
is wrong, unexplained, or contradicts the platform it is running on.

1. **`ImageSearch` fails with an empty message** (worst). `class Error`,
   `What "ImageSearch"`, `Extra ""`, `Message ""`, `StrLen(e.Message)` is 0.
   `audit/empirical/probes/imagesearch.ahk`. A program cannot tell this from
   its own bug, and the search for the picture is not even attempted (the
   screen reader is `grim`).
2. **`#Requires AutoHotkey v2.0` is refused by the v2 interpreter.**
   `This script requires AutoHotkey v2.0. This interpreter is v2.` — exit 1,
   `error_line` 1. The interpreter calls itself only `v2`, so **every**
   version with a minor component fails, including `v2.0.19` (the exact
   dialect) and the range form `>=2.0 <2.1`. The reference binary
   `E:\autohotkey\AutoHotkey64.exe` (AutoHotkey 2.0.19) loads the same line:
   `#Requires-v2.0=loaded`. The bare `v2` is the only form that works here.
3. **`DateAdd`/`DateDiff` accept only the single letters `s m h d`.** The
   documented v2 words are refused: `Parameter #3 of DateAdd is invalid.` for
   `"Seconds"`, `"Minutes"`, `"Hours"`, `"Days"`. The reference binary
   answers `20260112030405` for `DateAdd("20260102030405", 10, "Days")` and
   `-31` for the `DateDiff`. So a valid v2 program fails here, and the
   failure is a parameter complaint that points at the caller.
4. **`FileCopy`, `FileMove` and `DirDelete` throw `Failed`.** The message is
   the literal word `Failed`; `FileCopy` carries `Extra "1"` and nothing
   else. The same run's `FileDelete`, `FileRead`, `FileGetSize`,
   `FileGetTime` and `FileGetAttrib` all carry the real
   `(2) The system cannot find the file specified.` — so the cause is lost
   only in the copy/move/delete-folder handlers, and `DirDelete` of a
   non-empty folder leaves the folder in place while saying only `Failed`.
5. **`MouseGetPos` answers `0,0`.** With no `grim` the runtime answers the
   position it thinks it put the pointer at; the note says
   `the kernel did not say where the pointer is (grim failed: sh: line 0: grim: not found); this is the position the runtime put it at`,
   but the value is a coordinate that is not where the pointer is.
6. **`A_Args` is `Variable not found.`** v2 answers an `Array` when there are
   no arguments (confirmed on the reference binary: `A_Args=[Array]`). The
   name is not even in the surface table.
7. **`A_ThisFunc` is `Variable not found.`** while the surface lists
   `ThisFunc` as a variable with state `value`. The surface and the runtime
   disagree.
8. **`ComObject` is listed in the surface as a class with state `value`**,
   and calling it refuses: `the COM objects are a thing of Windows, and this
   runtime does not carry them.` Surface state is not a promise of a value.
9. **`ClipboardAll` is a class with state `value`, and `ClipboardAll().Size`
   refuses** with `this runtime does not carry that class yet.` The
   constructor answers; the members do not.
10. **`HotIf()` and `HotIfWinActive()` return `""` and accept without a
    word** — a hotkey condition the runtime cannot evaluate is silently
    accepted, while the `#HotIf` *directive* refuses with
    `the hotkeys are not carried by this runtime.`

(The 11th `silent-wrong` row is the third `#Requires` form, item 2 above:
`v2.0.19` and the range `>=2.0 <2.1` are the same failure as `v2.0`.)

Three further behaviours worth recording, not counted as `silent-wrong`:

- **A refused hotkey registration still fails the whole run** even when the
  error is caught: `audit/empirical/probes/hotkeys.ahk` ran three
  assignments (the catch wrote the message into `acc`), then the run ended
  with `ok:false`, `error_line:0`, `output:[]` — the closing `FileAppend`
  never ran. The report of `docs/runner-and-report.md` describes this for an
  uncaught registration; the same happens for a caught one.
- **`DirDelete` of a folder that is not empty** is the only file operation
  whose failure is both uninformative *and* leaves the state unchanged.
- **`Run` returns `0`** for a launch that really happened (`RunWait` of the
  same shell returns the real exit code 7). The reference binary answered
  `""` for the same command, so the pid is not carried by either; v2
  documents the pid, so the `0` is a placeholder and not a v2 value.

## 7. What contradicts the documentation

`docs/ahk-x11-comparison.md` §3.3 is a table of what the box does on Linux.
It is not refuted here — but three of its rows are contradicted **for the
host the tree was run on**, and the general wording of two other documents is
contradicted outright.

| §3.3 row | Doc says | Observed |
|---|---|---|
| Text and keys | uinput keyboard; `Send` writes the whole string | Not contradicted on Windows: `Send("")` returns `""`. Real text was deliberately not sent (it would type into the foreground window of this desktop), so the keyboard is *unmeasured* here |
| Pointer | absolute uinput; a click goes through | `MouseGetPos` answers `0,0` plus the note that the kernel did not say where the pointer is. The pointer position is not readable on this host |
| Screen | `grim`; `PixelGet 100,100` → `#12101b` | `PixelGetColor`, `PixelSearch`, `A_ScreenWidth`, `A_ScreenHeight` all fail with `grim failed: sh: line 0: grim: not found` **on a Windows host** |
| Windows | `xprop`; `WinExist`/`WinActive` → *"unknown"*, with the reason | On this host they **throw**, and the reason is the Linux-box sentence. The comparison doc's own §4.1 ("❌ *"unknown"*") and AGENTS.md §7 ("`WinExist` answers 0") describe an answer of `0`/unknown; observed is a thrown `Error` |
| Native calls | `dlopen`/`dlsym` + trampoline | **Confirmed**: `DllCall` made three real calls into `kernel32`/`user32` on this host |
| Windows DLLs | raven | Not probed (no Windows DLL was called; `DllCall` called the host's own libraries) |
| Files | `std::fs`; `FileAppend("hello", "ahk_probe.txt")` writes in the working directory | **Confirmed for the `std::fs` core** (16 names verified on disk). The row is too wide: `FileGetVersion`, `FileRecycle` and `FileCreateShortcut` refuse with `this is a name of Windows, and this desktop is not Windows.` **on a Windows host**, and three copy/move/delete handlers lose the OS error (`Failed`) |

Other doc claims contradicted by measurement:

- **`docs/runner-and-report.md` and `docs/v2-capability-report.md`: "the
  surface of v2 has 828 entries: 638 answer with a value, 190 are
  refused."** The same binary prints **833 / 643 / 190 / 0 pending**.
- **§3.4: "`Gui`, `Menu` and `Hotkey` are carried and refuse with the
  reason"** — observed: `Gui()` and `Menu()` **construct** successfully
  (`Type()` = `Gui`, `Menu`) and only their operations refuse; `Hotkey`
  refuses at registration whether caught or not. So the object model is
  carried further than the doc says, and the refusal is per method.
- **AGENTS.md §7: "a window that is not there is not a failure … `WinExist`
  answers 0."** On this host `WinExist` throws. That is consistent with the
  runtime's own rule ("a question the desktop cannot answer fails"), but the
  doc's sentence about `0` is not what this host answers.
- **The refusal reasons are not platform-derived.** On Windows the same run
  returns a Windows-shaped reason for one name and a Wayland-shaped reason
  for its neighbour (`WinActivate` vs `WinActive`; `ComCall` vs `ComObject`).
  The desktop is assumed to be Linux, except in `DllCall`, the files and the
  environment, which are the only three that reached the host.

## 8. The platform-dependent verdicts, and how to read them

Every row whose note says *platform-dependent* is a refusal of a Linux
primitive **while running on Windows**. On the Linux box those rows may well
be values; on this host they are hard failures, and the sentences name tools
the host does not have. They must not be read as "the tree cannot do this" —
they are "the tree's desktop is Linux, and this audit ran elsewhere".

The same questions were asked of the reference binary
`E:\autohotkey\AutoHotkey64.exe` (AutoHotkey **2.0.19**, verified by
`FileVersion` and by `A_AhkVersion`), in `audit/empirical/differential/`:

| Question | Tree | AutoHotkey 2.0.19 |
|---|---|---|
| `#Requires AutoHotkey v2.0` | **fails**, `This script requires AutoHotkey v2.0. This interpreter is v2.` | loads |
| `A_ScreenWidth` | refusal (`grim`) | `3840` |
| `A_OSVersion` | refusal ("not Linux") | `10.0.26300` |
| `A_ComputerName` | refusal (no `HOSTNAME`) | `RAPHAEL` |
| `A_UserName` | refusal (no `USER`) | `danalec` |
| `A_Args` | `Variable not found.` | `Array` |
| `DateAdd(…, 10, "Days")` | `Parameter #3 of DateAdd is invalid.` | `20260112030405` |
| `WinGetTitle()` | refusal (desktop) | throws `Target window not found.` (no foreground window in the harness) |
| `Run("cmd /c exit 0")` | `0` | `""` |

The reference binary **crashed** (0xC0000005) on the wider mixed probe
`differential/diff2.ahk` and `diff4.ahk` (ComObject / ImageSearch / DllCall /
WinGetList in one script), and one earlier attempt hung on a dialog; those
comparisons were not obtained, and the harness killed the process. Only the
minimal probe `diff3.ahk` and the `Run`-only probe `diff5` came back clean.
Nothing above depends on the crashed runs.

## 9. What was not probed, and why

- **`Send`/`SendText` with real text, `Click`, drag** — a script of this
  runtime drives the *real* desktop; typing or clicking would land in
  whatever window has the focus of the user's session. Only the empty string
  was sent (`Send("")` → `""`), which short-circuits before any key is
  written. The keyboard is therefore **unmeasured** on this host.
- **`SoundPlay` of a real sound** — the mixer is missing, so it would fail
  identically; and a real sound would play on the host.
- **Names that are not v2**: `FileCreateDir`, `FileReadLine`, `SoundGet`,
  `SoundSet`, `Screenshot`, `WinGetProcess` are **absent from the v2 surface
  table** (they are v1 names), so they cannot be probed as v2. The tree
  follows the table: a call to one reads as an unassigned variable
  (`This global variable has not been assigned a value.`), not as a
  function.
- **A Windows DLL through the raven** — not probed; `DllCall` was exercised
  against `kernel32`/`user32` only.
- **The Linux box itself** — not reachable as a build in this audit; every
  desktop verdict here is the Windows host's.

## 10. Files written

- `audit/empirical/probes/` — 50 `.ahk` probes and their 51 `.json` reports
  (plus `.err.txt` stderr captures, all empty), and `lib-probe.ahk`, the file
  `include-ok.ahk` includes, and `include-ok-cwd.json`, the same probe run
  with the probe folder as the working directory.
- `audit/empirical/surface.json` — the surface table of the binary, for the
  `value`/`refusal` state of a name.
- `audit/empirical/differential/diff3.ahk`, `diff5` (written to
  `%TEMP%\ahk-diff5.ahk`), `diff2.ahk`, `diff4.ahk` — the reference-binary
  probes.
- `audit/empirical/run-probes.ps1` — the harness that runs a probe with a
  hard timeout and captures its report.
- `audit/empirical-results.tsv` — 192 rows, one per name.
- `audit/t3-findings.md` — this file.
