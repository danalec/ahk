# AHK_X11 — the project's own numbers, checked against the source

Target: `C:\Users\danalec\Documents\src\AHK_X11` (GPL-2.0, read-only). Sources compared: `README.md` (parity block at `README.md:137-191`), `docs/list of commands.txt` (214 lines), `docs/index.html` (1.7 MB manual with an explicit legend at `docs/index.html:29-34`), `tests.ahk` (838 lines, `N_TESTS = 70`), and the Crystal tree under `src/`.

**Verification limitation (applies to this whole document):** there is no Crystal compiler on this machine and none on the Linux box, so the project was never built or run. Everything below is a source/claim cross-check. Where a defect is inferred from code rather than observed, it is marked **[code-evident, not runtime-verified]**.

---

## 1. The four lists: what the README claims vs what is actually in it

The README block is:

```
DONE      56% (127/226)
NEW        7% (15/226)
REMOVED    5% (11/226)
TO DO     31% (70/226)
```

Counted item by item, delimiting on commas and treating each written entry as one item:

| List | README line range | Items as written | Claimed | Match? |
|---|---|---|---|---|
| DONE | `README.md:139-156` | **127** | 127 | ✅ |
| NEW | `README.md:159-161` | **15** | 15 | ✅ |
| REMOVED | `README.md:164-170` | **11** | 11 | ✅ |
| TO DO | `README.md:173-184` | **70** | 70 | ✅ |
| **Sum** | | **223** | 224 | **❌ off by 3** |

So the per-list counts are all correct, and **the denominator is wrong**. 127 + 15 + 11 + 70 = **223, not 226**. The percentages are, however, computed consistently against 226 (127/226 = 56.19 → 56 %, 15/226 = 6.64 → 7 %, 11/226 = 4.87 → 5 %, 70/226 = 30.97 → 31 %), so the error is a single stale constant, not a miscount of the items.

### Where 226 does not come from

The obvious "true universe" candidate is `docs/list of commands.txt` (the generated list). It has **214 lines, of which line 1 is a comment, therefore 213 command entries**.

An item-by-item reconciliation closes exactly:

* **21** README items have no line in the generated list:
  * **11** NEW names: `Echo`, `ahk_x11_print_vars`, `Click`, `Eval`, `ahk_x11_track_performance_start`, `ahk_x11_track_performance_stop`, `#DefineCommand`, `DllCall`, `VarSetCapacity`, `NumGet`, `NumPut` (confirmed absent by searching the list);
  * `AutoTrim` and `Loop (registry)` (REMOVED, absent);
  * **7** TO-DO names absent from the generated list: `Progress`, `SetCapslockState`, `SetNumlockState`, `SetScrollLockState`, `SplashImage`, `SplashTextOn`, `SplashTextOff` (search returns no match for `Progress|Splash|CapslockState|NumlockState|ScrollLockState|AutoTrim`);
  * `{ ... }` (the brace block), which is a DONE item and not a command.
* **202** README items therefore do appear in the generated list (223 − 21).
* The generated list has **5 entries that no README list classifies at all**: `Run`, `Var++`, `Var--`, `Var += Value`, `Var -= Value`.
* It has **6 extra signature lines that duplicate a name already counted**: `MsgBox` ×2, `WinMove` ×2, `WinSetTitle` ×2, `SetBatchLines` ×2, `SetTitleMatchMode` ×2, and `if var is type` / `if var is not type` (2 lines for the single README item "If var is [not] type").

Check: 202 + 5 + 6 = **213** ✅ — the generated list is fully explained.

**Conclusion.** The distinct-name universe is **223** (the four lists) and grows to **228** if the five list-only names are added. **226 equals neither**: it is +3 over the lists and −2 over the expanded union. No file in the repository yields 226. The honest figures are:

| List | Items | % of 223 | % of 228 |
|---|---|---|---|
| DONE | 127 | 57.0 % | 55.7 % |
| NEW | 15 | 6.7 % | 6.6 % |
| REMOVED | 11 | 4.9 % | 4.8 % |
| TO DO | 70 | 31.4 % | 30.7 % |

None of these move by more than one percentage point, so the *headline* is not materially misleading — but the README's prose **is**: line 109 says *"Currently about 80 % of work of getting there is done"*, while its own table reports 56 % DONE (63 % if NEW is counted as done). That 80 % figure has no basis in the table, the list, or the source.

Also note what the item counts hide: the README counts a **command family as one item**. `Gui` is one DONE item but is backed by **10 `Gui*` command classes** (`GuiAdd`, `GuiShow`, `GuiSubmit`, `GuiCancel`, `GuiHide`, `GuiDestroy`, `GuiColor`, `GuiScroll`, `GuiOption`, `GuiControl`); `IfWinExist/IfWinNotExist`, `If var [not] between`, `IfInString/IfNotInString`, `IfGreater/IfGreaterOrEqual`, `IfLess/IfLessOrEqual`, `IfWinActive/IfWinNotActive`, `IfExist/IfNotExist` and `If var [not] in/contains MatchList` each stand for 2–4 classes. Conversely the source contains **162 `Cmd::Base` subclasses**, i.e. far more dispatchable names than the 142 "DONE+NEW" the README claims — because `Run`, `RunWait`, the 10 Gui subcommands, `GuiScroll`, `GuiOption` and the individual `If*` classes are not counted as separate items.

---

## 2. DONE names that are not working, or clearly incomplete

Of the 127 DONE items, **114 have a direct, non-stub implementation** and **13 are incomplete** (the `partial` rows of `ahk-x11-feature-inventory.tsv`). No DONE item has *no* class at all.

### 2.1 Incomplete (13)

| DONE name | What is missing | Evidence |
|---|---|---|
| `Menu` | `MenuName` must be `TRAY`; no GUI menu bars, submenus, `Delete`, `Default` | `src/cmd/gtk/menu.cr:6` |
| `GuiControl` | Only the *empty* sub-command exists; other widget types raise; no `Text/Move/Focus/Disable/Hide/Choose/Enable` | `src/cmd/gtk/gui/gui-control.cr:14-66` |
| `SplitPath` | The 5th output (`OutDrive`) is never assigned (4 `set_user_var` calls only) | `src/cmd/file/split-path.cr:7-10` |
| `FileSetAttrib` | Only `X` (executable) and `N` honoured; `R/A/S/H/O/T` parsed and dropped | `src/cmd/file/file-set-attrib.cr:12-27` |
| `FileSetTime` | Only `WhichTime M` (mtime); `C`/`A` ignored | `src/cmd/file/file-set-time.cr:19-21` |
| `SoundGet` | Master channel only; `ComponentType`/`DeviceNumber` ignored; parses `amixer` text | `src/cmd/misc/sound-get.cr:8-16` |
| `SoundSet` | Same: Master only, amixer text | `src/cmd/misc/sound-set.cr:8-30` |
| `Pause` | Pauses "the thread that is not me" by index — not the AHK per-thread model; `Pause, , 1` unsupported | `src/run/runner.cr:391-404` |
| `GetKeyState` | Only `D`/`U`; no `T` (toggle) mode; state from the recorded event stream | `src/cmd/x11/keyboard/get-key-state.cr:5-14` |
| `StringMid` | The `L` (by-reference output) parameter is not implemented | `src/cmd/string/string-mid.cr:6-17` |
| `If var [not] in/contains MatchList` | `contains`/`not contains` are **inverted** — they test whether a list element contains the variable, not whether the variable contains a list element **[code-evident]** | `src/cmd/control-flow/if-contains.cr:9`; `if-not-contains.cr:9` |
| `FileGetSize` | Declares `sets_error_level` but its success path returns `Bytes`, which makes `Thread` raise *"Result should be String for ErrorLevel command??"* — so only the failure path is sane **[code-evident]** | `src/cmd/file/file-get-size.cr:20` + `src/run/thread.cr:176-178` |
| `Edit` | Just launches the desktop's default text editor on the script; no editor choice, no line | `src/cmd/gtk/edit.cr:1-5`; `src/run/display/gtk.cr:355-364` |

All 13 are DONE items; none of the 15 NEW items is partial (all 15 are fully implemented, though `DllCall` carries the string-variable design limit).

### 2.2 Implemented, but behaviourally narrower than the name implies (not counted as partial)

These work as written but are not the v1 semantics a porter would assume:

* `Send` / `SendRaw`: cannot send a key that is physically down; `RButton::LButton` remap does not work; `{blind}` is a workaround; sending the hotkey's own key needs a synthetic key-up (`src/cmd/x11/keyboard/send.cr:60-73`, `README.md:241-256`).
* `CoordMode` relative = relative to the **active** window, not the last-found window (`src/cmd/x11/window/win-util.cr:82-89`).
* `WinClose`/`WinKill`: the `SecondsToWait` argument is accepted and then **deleted** (`win-close.cr:8`, `win-kill.cr:8`).
* `WinRestore` removes the maximized state but does not un-minimize (`win-restore.cr:8-9`).
* `WinSetTitle` writes `WM_NAME` only, not `_NET_WM_NAME` (`win-set-title.cr:17`).
* Window matching has **no `SetTitleMatchMode`**: title matching is libxdo's leading-substring regex, and the code says so (`win-util.cr:37-40`).
* `StringReplace` / `RegExReplace` / `StringGetPos` / `RegExGetPos`: `ErrorLevel` is 0/1 (found/changed) instead of AHK's not-found/count contract (`string-replace.cr:19`, `reg-ex-replace.cr:19`, `string-get-pos.cr:17`, `reg-ex-get-pos.cr:17`).
* `RegEx*` and all string searches are **always case-insensitive** (hard-coded `Regex::Options::IGNORE_CASE` / `downcase`), since `StringCaseSense` does not exist.
* `Input` with the `V` (visible) option does not display anything — it only skips the keyboard grab (`src/cmd/x11/keyboard/input.cr:46-48,103-105`).
* `Transform`: `Unicode` and `HTML` raise "Unimplemented" (`src/cmd/math/transform.cr:15,25`).
* `Sort` leaves a debug `puts "sort iter"` in the comparator (`src/cmd/string/sort.cr:38`).
* `PixelGetColor`/`PixelSearch` go through **GDK**, not Xlib (`src/cmd/misc/pixel-get-color.cr:15`, `pixel-search.cr:27`).
* `FileGetSize`, `WinGetText`, `Control*` all depend on environment features (AT-SPI accessibility) that the README itself warns about (`README.md:261-263`).

### 2.3 The one genuinely dangerous class of "DONE but silent"

`To DO`-listed and `REMOVED`-listed names are **hard parse errors**, because the parser aborts on an unknown first word (`src/build/parser.cr:253`). This is a defensible design, but a user porting a Windows script gets *"Syntax Error in line N: Command 'x' not found"* (wrapped at `parser.cr:37`) rather than a targeted "not implemented" message — for 66 TO-DO commands, 11 REMOVED names, and every one of the ~20 unlisted directives (`#IfWinActive`, `#InstallKeybdHook`, `#CommentFlag`, …).

---

## 3. TO-DO names that exist in the source anyway

Four of the 70 TO-DO names are **fully implemented** and are only mis-filed:

| TO-DO name | Implementation | Evidence |
|---|---|---|
| `BlockInput` | `XGrabKeyboard` + `XGrabPointer` | `src/cmd/x11/block-input.cr:2-12`; `src/run/display/x11.cr:433-440` |
| `DetectHiddenWindows` | Thread setting that drops the `only_visible`/`desktop` filters from the window search; `A_DetectHiddenWindows` is computed | `src/cmd/misc/detect-hidden-windows.cr:2-7`; `src/cmd/x11/window/win-util.cr:30-34`; `src/run/thread.cr:232` |
| `WinSet` | `AlwaysOnTop`, `Bottom`, `Transparent`, `TransColor` | `src/cmd/x11/window/win-set.cr:3-50` |
| `#MaxThreadsPerHotkey` | Parser directive (max 20) enforced by `Hotkey#trigger` | `src/build/parser.cr:110-112`; `src/run/display/hotkey.cr:83-87` |

The project's **own HTML documentation agrees with the source, not the README**: in `docs/index.html` these four are *not* tagged `tbd` ("Not yet supported"), whereas genuinely missing names are (e.g. `OnExit` at `docs/index.html:236/547/2510`). See `docs/index.html:29-34` for the legend (`tbd` = red/strikethrough = TODO; `x11` = green/italic = AHK_X11-only; `rm` = grey/strikethrough = removed).

Two more "not in the lists at all, but implemented" items are worth flagging because they are *entire commands*: **`Run`** and **`RunWait`** (`src/cmd/misc/run.cr:78-170`, `src/cmd/misc/run-wait.cr:4`). `Run` *is* in `docs/list of commands.txt:137`, so the generated list knows about it — the README's four lists simply never mention it. A parity table that omits `Run` cannot be read as a complete inventory.

---

## 4. What `tests.ahk` actually covers

`tests.ahk` is a genuine self-test: it declares `N_TESTS = 70` (`tests.ahk:6`), runs `GoSub, run_tests`, and fails if `tests_run != N_TESTS` (`:12-16`). Assertions are collected by two subroutines, `assert` (value equals) and `assert_false` (value differs) (`:20-81`), with a separate `timeout` helper for async expectations (`:89-109`).

### 4.1 The 70 is real, but it is not 70 test sites

There are **51 literal `gosub assert` call sites**. Four of them sit *inside* helper subroutines that are invoked repeatedly:

| Helper | Assert sites inside | Invocations in the file | Asserts produced |
|---|---|---|---|
| `test_send` | 1 | 4 (`:579, :582, :586, :596`) | 4 |
| `test_hotstring` | 1 | 5 (`:627, :636, :641, :646, :651`) | 5 |
| `test_hotkey_success` | 1 | 8 (`:700, :704, :708, :712, :716, :721, :725, :729`) | 8 |
| `test_hotkey_send` | 1 | 6 (`:734, :748, :751, :759, :763, :770`) | 6 |

So the runtime count is (51 − 4) + 23 = **70** ✅ — the constant is honest, and the suite is self-checking.

### 4.2 Areas actually exercised

| Area | What is asserted | Evidence (tests.ahk) |
|---|---|---|
| Harness + variable semantics | equality, whitespace trimming, `assert_false`, the timeout helper | `:115-143` |
| GUI construction | `Gui -caption`, `Add Picture/Button/Edit`, `+resize`, `Show` at x/y | `:146-157` |
| Window predicates | 6 `IfWinExist`/`IfWinNotExist` variants incl. WinText and ExcludeTitle, then `WinActivate` + `IfWinNotActive` | `:161-201` |
| `WinGetPos` / `WinMove` | position, then 4 readback assertions after a move | `:203-220` |
| `ControlClick` | clicks a GTK button by its forged ClassNN `push_button_0_1` | `:244-247` |
| `ControlGetPos` / `ControlGetText` / `ControlSetText` | 3 + 2 assertions on a Gui picture and edit | `:253-268` |
| `CoordMode` + `MouseGetPos` | mouse coords relative/screen | `:272-279` |
| Files | `FileAppend`→`FileCopy`→`FileDelete`→`FileRead` round-trip | `:299-310` |
| `GetKeyState` | `{a down}` → `D`, `{a up}` → `U` | `:334-343` |
| `GuiControl` + `Gui Submit` | set an edit's text and read it back | `:353-356` |
| `Hotkey` (dynamic) | register `a` → label, trigger via `xdotool`, then `Off` | `:360-369` |
| `Input` | basic, extended (Match + EndKey + Backspace), timeout | `:390-427` |
| `MouseClick` | clicks the Gui button by coordinates | `:441-445` |
| `MouseGetPos` control | control under the cursor is `push_button_0_1` | `:449-451` |
| `PixelGetColor` / `PixelSearch` | a known Gui color, RGB mode, CoordMode relative | `:458-467` |
| `WinGetClass/Text/Title` | including the AppImage-vs-not class name branch | `:524-538` |
| `Send` ×4 | `123`, `aBc`, `+d` (shift), and modifier restoration after `{lshift down}` | `:558-600` |
| Hotstrings ×5 | plain, case-insensitive non-trigger, `:R:`, `:O:`, `:*:` | `:604-651` |
| Hotkey with inline command | `^+!s::MouseClick, L` fired by xdotool | `:655-660` |
| Dynamic hotkeys ×8 | `f2`, `F2`, `+s`, `+S`, `*s`, `esc`, `xbutton2`, `lbUTton` | `:698-729` |
| Hotkey-triggered `Send` ×6 | incl. sending the hotkey's own key, `{blind}`, and clipboard-in-hotkey | `:731-770` |
| Remap | `bracketleft` remapped, `GetKeyState` on the target key | `:772-784` |
| Mouse-button send | `Send, {LButton}` clicks the Gui button | `:788-792` |
| Clipboard | paste via right-click menu, unset, and the unset race | `:794-815` |

### 4.3 Whole areas with **no** test

The file itself marks many names with `;;` (skipped) or `;` (commented signature list) — e.g. `;;BlockInput` `:238`, `;;FileGetSize` `:318`, `;;SoundGet` `:493`, `;;WinSet` `:549`. Beyond those markers, these areas have *no assertion at all*:

* **Every dialog**: `MsgBox`, `InputBox`, `FileSelectFile`, `FileSelectFolder`, `TrayTip`, `ToolTip` (only `MsgBox`/`ToolTip` appear as commented signatures, `:429, :453`).
* **All four Loop kinds** — the harness uses `Loop` without asserting it (`:91-104, :607-616`).
* **All flow control**: `GoSub`/`GoTo`/`Return`/`Break`/`Continue`/`Else`/blocks are used by the harness but never asserted; `SetTimer` only as a harness mechanism.
* **All directives** except that the test script relies on defaults: `#Persistent`, `#SingleInstance`, `#Include`, `#DefineCommand`, `#MaxThreadsPerHotkey`, `#NoTrayIcon`, `#XGrabRoot` — none tested (`:224-237` are all commented).
* **Pause / Suspend / Reload / ExitApp / Exit** (`:455-457, :490`).
* **All string commands except** the four the harness needs (`StringGetPos`, `StringMid`, `StringLeft`, `StringLen`): no test for `StringReplace`, `StringTrimLeft/Right`, `StringSplit`, `StringUpper/Lower`, `Sort`, `RegExGetPos`, `RegExReplace`, `Transform`.
* **All math**: `EnvAdd`/`EnvSub`/`EnvMult`/`EnvDiv` only as `+=` in the harness; no `Random`, no `Transform`, no `SetFormat` (absent anyway).
* **Most of the file family**: only append/copy/delete/read. No `FileGetSize`, `FileSetTime`, `FileSetAttrib`, `FileRecycle`, `FileRecycleEmpty`, `FileReadLine`, `SplitPath`, `DriveSpaceFree`, `FileCreateDir`, `SetWorkingDir`, `Ini*`, `Run`, `RunWait`, `URLDownloadToFile`.
* **Sound** (all four commands), `DllCall`, `NumGet`, `NumPut`, `VarSetCapacity`, `Eval`, `#DefineCommand`, `ahk_x11_*` helpers.
* **Window verbs beyond the tested set**: `WinHide`, `WinShow`, `WinMaximize`, `WinMinimize`, `WinRestore`, `WinKill`, `WinSet`, `WinSetTitle`, `WinWait*`, `WinGet id/pid/controllist`, `DetectHiddenWindows`, `BlockInput`, `WinMinimizeAll*`.
* **`Input`/`Send` edge cases**, hotstring `:C:` case sensitivity (explicitly skipped, `:629-632`), `?` option, 30-char buffer boundary.
* **Error paths**: no test of a parse error, an unknown command, a runtime exception, or `ErrorLevel` values other than the Input cases.
* **Threading**: no test of priority, of two hotkeys interrupting each other, of `#MaxThreadsPerHotkey`, or of `Pause`/`Suspend` behaviour.

Roughly: **~35 % of the DONE surface is touched, and only ~25 % is asserted**, and the coverage is concentrated exactly where a GTK Gui can be observed on screen (windows, controls, send, hotkeys, clipboard, pixels). Commands whose test would need a display, a WM, a sound card, a notification daemon or a second machine are essentially untested.

---

## 5. Overstatements in the README (checked against source)

| # | Claim | Reality | Evidence |
|---|---|---|---|
| 1 | `DONE 56% (127/226)` — four lists sum to 223 | denominator is 3 too large; nothing in the repo yields 226 | `README.md:138-184`; `docs/list of commands.txt` |
| 2 | *"Currently about 80 % of work of getting there is done"* (`README.md:109`) | its own table says 56 % DONE / 63 % DONE+NEW; source says 162 classes vs a 228-name universe | `README.md:109,138-184` |
| 3 | `Menu` is DONE | tray menu only; any non-TRAY name raises | `src/cmd/gtk/menu.cr:6` |
| 4 | `GuiControl` is DONE | one sub-command, writes only, many widget types raise | `src/cmd/gtk/gui/gui-control.cr:14-66` |
| 5 | `SplitPath` is DONE | 4 of 5 outputs; `OutDrive` missing | `src/cmd/file/split-path.cr:7-10` |
| 6 | `SoundGet`/`SoundSet` are DONE | Master channel only, amixer text parsing, no wave volume | `src/cmd/misc/sound-get.cr:8-16` |
| 7 | `Pause` is DONE | not per-thread; no priority argument | `src/run/runner.cr:391-404` |
| 8 | *"create stand-alone binaries with no dependencies, including full functionality like Hotkeys and GUIs"* (`README.md:111`) | true only for an AppImage build; the tree warns that otherwise "the resulting standalone program will not be very portable" | `src/compiler.cr:14-16` |
| 9 | *"windows are internally cached by their ID during the lifetime of the thread"* (`README.md:428`) | the thread cache is of **AT-SPI accessibles** (`top_level_accessible_by_window_id`, `accessible_by_class_nn_by_window_id`), not of window-search results | `src/run/thread.cr:29-32`; `src/run/display/at-spi.cr:28-29` |
| 10 | Performance table (`README.md:398-408`: `Send` 530 µs, `WinGetText` 0–3 s, …) | cannot be verified here at all — unverified claim, no benchmark harness in the repo beyond `ahk_x11_track_performance_*` | `README.md:396-423` |
| 11 | *"everything important is done"* / *"Generally speaking, everything important is done"* (`README.md:135`) | 66 TO-DO commands absent, no `Run` in the table, `GuiControl` write-only, `OnExit` absent | `README.md:135`; `src/build/parser.cr:253` |

## 6. Understatements in the README (things that are better than claimed, or mis-filed)

| # | Claim | Reality | Evidence |
|---|---|---|---|
| 1 | `BlockInput` is TO DO | implemented (XGrabKeyboard/XGrabPointer) | `src/cmd/x11/block-input.cr:2-12`; `src/run/display/x11.cr:433-440` |
| 2 | `DetectHiddenWindows` is TO DO | implemented, plus `A_DetectHiddenWindows` | `src/cmd/misc/detect-hidden-windows.cr:2-7`; `src/run/thread.cr:232` |
| 3 | `WinSet` is TO DO | implemented (4 attributes) | `src/cmd/x11/window/win-set.cr:3-50` |
| 4 | `#MaxThreadsPerHotkey` is TO DO | implemented and enforced | `src/build/parser.cr:110-112`; `src/run/display/hotkey.cr:83-87` |
| 5 | `Run` / `RunWait` appear in **no** README list | fully implemented, with stdout/stderr/PID capture and `UseErrorLevel` | `src/cmd/misc/run.cr:78-170`; `src/cmd/misc/run-wait.cr:4` |
| 6 | `REMOVED` names are described as "make no sense in Linux" / "skipped" | they are not no-ops — using one aborts the whole script at parse time | `src/build/parser.cr:253` |
| 7 | *"no Gui sub command is included in the listing"* (`README.md:135`) | true and honest, but the effect is that the single DONE item `Gui` hides 10 classes and 8 widget types — the README does not say how few | `src/cmd/gtk/gui/gui-add.cr:27-130` |
| 8 | `#XGrabRoot`, `#NoEnv`, hashbang, `GuiScroll`, `GuiOption` | implemented but mentioned only in passing (or not at all) in the four lists | `src/build/parser.cr:115-118, 117`; `src/cmd/gtk/gui/gui-scroll.cr`; `gui-option.cr` |

## 7. README vs the project's own HTML docs

The two disagree in exactly the four places above. `docs/index.html` uses a documented legend (`:29-34`) and does *not* tag `BlockInput` (`:347, :429, :2079`), `DetectHiddenWindows` (`:722, :2143`), `WinSet` (`:795, :2854`) as `tbd`, i.e. the HTML docs are **more accurate than the README** on those four names; `OnExit` *is* tagged `tbd` in both (`:236, :547, :2510`) and is genuinely absent (`grep OnExit src` finds no command). The docs also contain unmarked-but-behaviourally relevant annotations that support this audit, e.g. the `x11` note that only `WS_EX_NOACTIVATE` is supported as an extended Gui style (`docs/index.html:6115`), the `x11` note that the tray icon changes to a keyboard/loading symbol rather than changing colour (`:5738, :6020`), and the honest "unsure" note about hidden files in file loops (`:4743`).

---

## 8. Bottom line

* **Real counts: 127 DONE, 15 NEW, 11 REMOVED, 70 TO DO = 223 items.** The `226` denominator is wrong by +3 and cannot be reproduced from any file; percentages move by ≤1 point, but the prose "80 %" is contradicted by the table.
* **DONE is 90 % solid**: 114 of 127 items are direct implementations; 13 are partial, of which two are outright broken as written (`FileGetSize` on success, `if var contains`) and one (`Menu`) is tray-only despite the listing.
* **TO DO understates the tree by 4 commands** plus the entirely unlisted `Run`/`RunWait`; the generated command list confirms those exist.
* **Tests are real but shallow**: 70 self-counted assertions over 23 helper invocations, covering the GTK/Gui-centred happy path; dialogs, loops, flow control, directives, most string/math/file commands, sound, DllCall, and all error paths are untested.
* **Nothing in this document was verified by execution** — no Crystal compiler was available. Everything is traceable to the cited lines.
