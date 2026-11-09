# The AHK_X11 audit, and what this tree is missing

> An audit of **AHK_X11** — the AutoHotkey **v1.0.24** reimplementation for
> Linux/X11, in Crystal — against this tree, the AutoHotkey **v2.0.19** in
> Rust. It answers three questions:
>
> 1. **What does AHK_X11 really do?** (not what it claims: what the source does)
> 2. **What does this tree really do?** (measured by running it, not by reading the notes)
> 3. **What does this tree miss** that AHK_X11 proves is possible on Linux?
>
> The evidence is in `audit/`: the per-name feature inventory, the refusal
> census, the empirical probe reports, and 50 runnable probe scripts.
>
> **Audited:** AHK_X11 `master` (shard `1.0.7`) and this tree at commit `f47cbd6`.
> **Date of measurement:** 2026-11-09.
>
> **Status.** This document is the record of the tree at `f47cbd6`, before any
> of the work it recommends. The numbers in §3 are that state. The tree has
> since changed — the round log of what landed, and what is next, is
> [`audit/PLAN.md`](../audit/PLAN.md). After seven rounds the surface is
> **833 entries, 663 values, 170 refusals**: the `Process*` family, `TrayTip`,
> the four read-only `WinGet*`, the geometry, the maximized/minimized state and
> the four `WinWait*` no longer refuse.

---

## 0. Method, and the limits of this audit

Three kinds of claim are made below, and each is marked:

| Mark | Meaning |
|---|---|
| **MEASURED** | observed by running the release binary of this tree on the audit host |
| **SOURCE-READ** | read from the Crystal source of AHK_X11; the project was **not** executed |
| **DOC** | what a document of either project says, checked against 1 or 2 |

### The two limits, stated plainly

**The AHK_X11 was never executed.** There is no Crystal compiler on the audit
host, and none on the Linux box. The project pins `crystal: 1.5.1` in
`shard.yml`, the only Crystal available is 1.22.2 (an AUR/repo package), and its
seven dependencies are git branches of personal forks (`phil294/x11-cr`,
`phil294/xtt-cr`, `phil294/x_do.cr`, `phil294/gtk3.cr`, `phil294/atspi.cr`,
`phil294/notify.cr`, `tasker`). A 2023-era Crystal codebase does not build on a
1.22 compiler, and installing it on the box needs a sudo password. **So every
AHK_X11 claim here is a source read, not a run.** Two findings are additionally
marked `[code-evident]` where the defect is clear from the code but was not
observed.

**The tree was measured on Windows, not on the box.** That matters: many
refusals are platform-derived, and many of the box primitives shell out to Linux
tools. On Windows those tools are absent, so a few families appear to refuse for
the wrong reason. Section 4 separates the two classes of failure explicitly.
Where a claim is about the Linux box, it is sourced from `AGENTS.md`'s own
measurements or from tool presence checked live on the box (see §2.4).

---

## 1. Executive summary

Ten findings, in descending order of how much they should change the plan.

1. **The tree's headline number is stale in five documents, and wrong in a sixth.**
   The binary reports **833 entries, 643 with a value, 190 refused, 0 pending**
   (**MEASURED**). Five documents say **828 / 638**: `docs/v2-capability-report.md:5`,
   `docs/ahk-gaps-report.md:19-20`, `docs/ahk-v1-dialect.md:1426`,
   `docs/dialects.md:73`, `docs/runner-and-report.md:310`. The comparison document
   says something else again — **725 / 506 / 219** (`docs/ahk-x11-comparison.md:772`).
   **No document of the project states the real surface count.**

2. **115 of the 190 refusals are implementable on Linux and are refused anyway.**
   **169** refusals share one sentence — *"this is a name of Windows, and this
   desktop is not Windows."* Judged name by name: **60 are genuinely
   Windows-only, 15 are absent by the runtime's own design, and 115 are
   implementable** (31 trivially). (**MEASURED** + **SOURCE-READ**)

3. **The refusal taxonomy conflates two different things**, and that is the root
   cause of #2. *"Not a name of Windows"* and *"not possible in this Wayland
   session"* are one bucket, so 115 names that Linux can serve are filed under a
   platform that is not the obstacle. The bucket needs splitting before the list
   can be worked from.

4. **`--surface` reports names, not capabilities — and overstates the tree by
   about 148 names.** 148 method entries of the `Gui` family (`Gui` 59,
   `Gui.Control` 29, `Gui.Menu` 19, `ListView` 15, `TreeView` 15, `StatusBar` 11)
   print `value`. They do not work: `g := Gui()` constructs an object, and
   `g.Show()` fails with *"Gui cannot show the window: a window is a Windows
   window, and this desktop is a Wayland session without one."* (**MEASURED**)
   The real count of working surface entries is nearer **495** than 643.

5. **Two functional defects break ordinary v2 scripts.** `#Requires AutoHotkey
   v2.0` — a line at the top of almost every real v2 script — is **refused by the
   v2 interpreter**: *"This script requires AutoHotkey v2.0. This interpreter is
   v2."*, exit 1. The interpreter reports its own version as `v2`, so `v2.0`,
   `v2.0.19` and `>=2.0 <2.1` all fail. The reference `AutoHotkey64.exe` 2.0.19
   loads the same line. (**MEASURED**) Second: `ImageSearch` fails with an
   **empty message** — no reason is given at all.

6. **Platform error messages are hard-coded to Linux and emitted on Windows.**
   The shell-outs (`grim`, `paplay`, `pactl`, `zenity`, `wl-copy`, `xclip`) are
   **not** `cfg`-gated, unlike the uinput paths (11 `cfg(target_os)` gates in
   `ahk-box/src/desktop.rs`). So on Windows: `Screenshot` answers
   `sh: line 0: grim: not found`; `Gui.Show()` claims *"this desktop is a Wayland
   session"*; `SoundPlay` answers `paplay: not found`. The tree is supposed to
   refuse with a reason, not to leak a shell error. (**MEASURED**)

7. **AHK_X11's own parity claim is arithmetically wrong, and it also
   under-sells itself.** The README's four lists contain **127 + 15 + 11 + 70 =
   223** names, not the advertised **226** — every per-list count is right and
   only the denominator is wrong. Conversely, **six commands it advertises as
   missing are implemented** (`BlockInput`, `DetectHiddenWindows`, `WinSet`,
   `#MaxThreadsPerHotkey` filed as TO DO; `Run`, `RunWait` in no list at all),
   and **13 of its 127 "DONE" commands are only partial**. (**SOURCE-READ**)

8. **The largest gap is the GUI, and it is the one the surface hides.** Beyond
   the 148 invisible `Gui` methods, `ToolTip`, `TrayTip`, `Control*` (48 names)
   and `Win*` (52 names) are refused. The AHK_X11 reaches all of these through
   **GTK3 and AT-SPI** — and AT-SPI is **D-Bus, which Wayland keeps**. This is
   the single most valuable thing AHK_X11 has to teach, and the audit confirms
   the route is open.

9. **The windows gap is narrower than the refusal list implies.** The tree
   already binds `ext_foreign_toplevel_list_v1` and `zcosmic_toplevel_info_v1`,
   which carry a stable identifier, a geometry rectangle and
   an activated/fullscreen state. That makes ~16 `Win*`/`Group*` names
   implementable **now**; only placement-type verbs (`WinMove`, `WinHide`,
   `WinSetTitle`) are genuinely impossible in Wayland. (**SOURCE-READ**)

10. **Every capability that the box actually needs is present on the box.**
    Checked live: `wl-copy`, `wl-paste`, `grim`, `zenity`, `curl`, `notify-send`,
    `ffplay`, `paplay`, `aplay`, `amixer`, `pactl`, `xprop` are all installed.
    So **`TrayTip` refusing as "Windows" is gratuitous** — `notify-send` is
    right there. `xclip`, `xsel`, `xdotool` and `xdotool`-adjacent tools are not
    installed. (**MEASURED**, live on the box)

---

## 2. Part 1 — What AHK_X11 really does

Full detail: [`audit/ahk-x11-feature-inventory.tsv`](../audit/ahk-x11-feature-inventory.tsv)
(260 rows, every row with `file:line` evidence),
[`audit/ahk-x11-subsystems.md`](../audit/ahk-x11-subsystems.md) (15 subsystems),
[`audit/ahk-x11-claims-vs-reality.md`](../audit/ahk-x11-claims-vs-reality.md).

### 2.1 The technical sheet

| | |
|---|---|
| Dialect | AutoHotkey **v1.0.24** (2004), legacy syntax only |
| Language | Crystal, pinned `crystal: 1.5.1`; `shard.yml` version `1.0.7` |
| License | **GPL-2.0** — the code may be read, never copied into this Apache-2.0 tree |
| Size | **166** `.cr` files under `src/cmd`, **162** declaring a `Cmd::Base` subclass |
| Command list | `docs/list of commands.txt`: **213** entries (214 lines − 1 comment) |
| Tests | `tests.ahk`, **70** assertions — the arithmetic is honest (`51` `gosub assert` sites + `4` helpers invoked `23` times) |
| Display | X11 only. Wayland is **not** supported (it is the project's own issue #2) |
| Inject | `XTEST` through `x_do` (a fork of xdotool) |
| Windows | `x_do`/`X11` for the verbs, **AT-SPI** for text and controls |
| GUI | GTK3 (`Gui`, dialogs, `Menu` tray, `ToolTip`, `TrayTip`) |

### 2.2 The README's numbers, checked

```
DONE      56% (127/226)      -> really 127 items, and all 127 exist
NEW        7% (15/226)       -> really 15
REMOVED    5% (11/226)       -> really 11  (10 platform + AutoTrim)
TO DO     31% (70/226)       -> really 70
                                ----------
                                223, not 226
```

Each **per-list** count is correct; the **denominator** is wrong. The percentages
were computed against 226, so they are internally consistent (56/7/5/31 ≈
127/15/11/70 over 226) — the error is upstream of the arithmetic and was never
caught because nothing recomputes the total. **No file in the repository yields
226.** A reconciliation of

`list of commands.txt` ↔ README closes exactly: 202 README names appear in the
list; 21 README items do not (11 NEW + `AutoTrim` + `Loop (registry)` + 7 TO-DO
names + `{ ... }`); the list adds 5 unclassified names (`Run`, `Var++`, `Var--`,
`Var+=`, `Var-=`) and 6 duplicate signature lines.

**The real universe is 223** (228 if the 5 list-only names are counted).

### 2.3 What the README gets wrong in both directions

**It under-sells itself by six commands.** Implemented but filed as TO DO:

| Name | Evidence |
|---|---|
| `BlockInput` | `src/cmd/x11/block-input.cr` |
| `DetectHiddenWindows` | `src/cmd/misc/detect-hidden-windows.cr` |
| `WinSet` | `src/cmd/x11/window/win-set.cr` (2.3 KB) |
| `#MaxThreadsPerHotkey` | handled in the thread/hotkey code |
| `Run`, `RunWait` | `src/cmd/misc/run.cr` (6.9 KB) — these appear in **no** README list at all |

**Thirteen of the 127 "DONE" are only partial.** The important ones:

- `Menu` is **tray-only** and refuses every other menu name (`menu.cr:6`).
- `GuiControl` implements **one** sub-command and is write-only.
- `SplitPath` silently **drops `OutDrive`**.
- `FileSetTime` supports only the `M` (modification) kind.
- `FileSetAttrib` handles only `X`/`N`.
- `Sound*` is **Master-only**, via `amixer`.
- `FileGetSize` declares `sets_error_level` but writes the **size** into
  `ErrorLevel` instead of the v1 `0`/`1` status (`file-get-size.cr:5,20`).

> **Correction to an earlier draft of this audit.** A first pass claimed
> `FileGetSize` *raises* on success. It does not: `set_user_var`
> (`runner.cr:275-277`) returns the assigned `String`, so the
> `raise "Result should be String for ErrorLevel command??"`
> (`thread.cr:177`) is not reached on the normal path. The real defect is the
> semantic one above — the size landing in `ErrorLevel`. The claim that its test
> is "disabled" is also weak: the line is one entry in a long block of names the
> suite simply does not test, not a test switched off for a known failure.

### 2.4 Two mechanisms the existing comparison document gets wrong

| The comparison document says | The source says |
|---|---|
| `PixelGetColor`/`PixelSearch` read the screen "through `XGetImage`" (`:192-193`, `:438`, `:543`) | **GDK**: `Gdk.pixbuf_get_from_window(Gdk.default_root_window, …)` (`pixel-get-color.cr:15`) |
| The display-adapter "abstract interface of 6 methods" (`:116-118`) | True as a list, but **the abstraction does not isolate X11** — one implementation, and ~60 call sites reach through the concrete `Run::X11` |

The mechanism difference matters because GDK goes through the same session as
the compositor: it is the *portable* part of the design, and it is why a
`PixelGetColor` port is cheaper than "rewrite `XGetImage` in Rust" implies.

### 2.5 What AHK_X11 proves is possible on Linux

This is the part of the audit that matters for the plan. Each of these works on
Linux in AHK_X11, through a mechanism that also exists on the box.

| Capability | AHK_X11's mechanism | Does the box have it? |
|---|---|---|
| Window search and verbs | `x_do`/X11 + AT-SPI for text | partially (`xprop` yes, `xdotool` no) |
| Control text, geometry, click | **AT-SPI over D-Bus** | **yes** — D-Bus is how Wayland works |
| GUI and dialogs | **GTK3** | **yes** — `zenity` already installed |
| Clipboard + change events | GTK clipboard | **yes** — `wl-copy`/`wl-paste` installed |
| Sound play/volume | `ffplay`/`cvlc`/`paplay`/`mpg123`/`aplay`, `amixer` | **yes** — all four players + `amixer`/`pactl` |
| Screen pixels | GDK pixbuf | **yes** — `grim` installed |
| Hotkeys | `XGrabKey` | different route (uinput), but the tree already drives uinput |
| Desktop notifications | libnotify | **yes** — `notify-send` installed |
| Process management | (not implemented — TO DO) | `/proc`, `kill` |

---

## 3. Part 2 — What this tree really does

Full detail: [`audit/rust-refusals.tsv`](../audit/rust-refusals.tsv) (190 rows),
[`audit/rust-refusal-analysis.md`](../audit/rust-refusal-analysis.md),
[`audit/empirical-results.tsv`](../audit/empirical-results.tsv) (192 rows),
[`audit/empirical/probes/`](../audit/empirical/probes/) (50 runnable probes).

### 3.1 The measured state, today

```
$ ahk --surface
the surface of v2 has 833 entries: 643 answer with a value, 190 are refused, 0 are pending.
```

| Quantity | Value |
|---|---|
| Surface entries | **833** |
| Answer with a value | **643** (overstated: see §3.2) |
| Refused | **190** |
| Pending | **0** |
| Built-in table rows | **346** |
| Rows with `handler: not_carried` | **181** |
| Rows with a real handler | **165** |
| Non-v2 names carried | **1** — `Screenshot` (`builtins.rs:665`) |
| Capability traits summed by `Desktop` | **15** (`desktop.rs:920-936`) |

### 3.2 The surface overstates the tree by ~148 names

`--surface` derives a name's state from the table, and a **method** of a class
counts as `value` because the class row exists. The `Gui` family is the case
that breaks it: 148 method rows are `value`, and the whole family refuses when
used. (**MEASURED**)

```ahk
g := Gui()        ; ok — constructs
g.Show()          ; error: "Gui cannot show the window: a window is a Windows
                  ;         window, and this desktop is a Wayland session without one."
```

The same shape recurs at the variable level: `A_ThisFunc` is listed as a
`value` variable and answers *"Variable not found."* at run time.

**Therefore: the working surface is ≈495, not 643, and `--surface` cannot be
used as a coverage metric until a method's state reflects its class's refusal.**

### 3.3 The refusal census

190 refusals, **16 distinct reason strings**:

| Count | Reason |
|---|---|
| **169** | `this is a name of Windows, and this desktop is not Windows.` |
| 5 | the registry is a thing of Windows |
| 3 | this runtime has no tray icon |
| 13 | thirteen singletons (cursor shape, output scale, `ListLines`, `ListVars`, one script/one thread, …) |

Judged name by name against what Linux and Wayland actually publish:

| Verdict | Count | Meaning |
|---|---|---|
| **`missing-linux-impl`** | **115** | the platform can do it; the tree has no implementation |
| `windows-only` | 60 | a genuine Windows API (COM, Win32 messages, registry) |
| `subsystem-absent` | 15 | absent by the runtime's own design (one script that ends) |
| | | |
| *of which* doable | **31 yes / 74 likely / 85 no** | |

By area: windows **52**, controls **48**, machine **40**, registry/COM **14**,
native **10**, processes **9**, files **5**, hotkeys **4**, GUI **3**, screen
**2**, sound **2**, clipboard **1**. Timers, dialogs, network and pictures have
**zero** refusals.

### 3.4 The empirical result, by family

192 name-level probes, 50 scripts, each wrapped in `try/catch` so a refusal
records its exact text:

| Verdict | Count |
|---|---|
| value | 78 |
| refusal | 99 |
| **silent-wrong** | **11** |
| not-probed (not a v2 name) | 4 |

**Works end-to-end:** INI (4 names), output routing (17 — `FileAppend "*"`,
`MsgBox` and `FileOpen("*","w")` all land in the report's `output`).

**Works with holes:** Files, Time, Binary (`Buffer` + `NumPut`/`NumGet` verified
working), Native (`DllCall` really called `kernel32`/`user32`), Regex/strings,
Processes (`Run`/`RunWait`/`EnvGet`/`EnvSet`), `A_*` variables, Directives,
Screen (`LoadPicture` only).

**Refuses as a family:** Clipboard, Sound, Windows (21 refusals), Controls,
Hotkeys/hotstrings — **on Windows**. See §4 for which of these are real.

---

## 4. Part 3 — What the tree misses

This is the answer to the question that was asked, ordered by cost per unit of
value. Every item is `missing-linux-impl` unless marked.

### 4.1 Cheap wins — a few hours each

| # | What | Names | Why it is cheap |
|---|---|---|---|
| 1 | **Process management** | `ProcessExist`, `ProcessGetName`, `ProcessGetParent`, `ProcessGetPath`, `ProcessClose`, `ProcessSetPriority`, `ProcessWait`, `ProcessWaitClose` (8) | `kill(pid,0)`, `/proc/<pid>/comm|stat|exe`, `renice(2)`. The `Processes` trait already has `run`/`run_and_wait`; it lacks the four accessors — **six of the eight landed in round 1** (`audit/PLAN.md` §4) |
| 2 | **Lock keys** | `SetCapsLockState`, `SetNumLockState`, `SetScrollLockState` (3) | `keys.rs` already resolves all three and pins codes 58/69/70; it is a `press_key` each |
| 3 | **`OnClipboardChange`** | 1 | the clipboard works (`wl-copy`/`wl-paste`); the missing piece is `wl-paste --watch` |
| 4 | **`Str*` pointer family** | `StrGet`, `StrPut`, `StrPtr` (3) | `NumGet`/`NumPut` over `Buffer` are verified working; the encoder is already in `native.rs` |
| 5 | **`A_ScreenDPI`, `A_AhkPath`** | 2 | `wl_output.scale`; the runtime *has* an executable |
| 6 | **`SoundGetName`** | 1 | the sinks are already read for `SoundGetVolume` |
| 7 | **`Edit`** | 1 | `$EDITOR` — it is literally "open this file" |
| 8 | **`TrayTip`** | 1 | **`notify-send` is installed on the box** |

That is **20 names** for very little code, and it fixes the most obviously
wrong refusals (a process question answered with "this is Windows").

### 4.2 Medium — a protocol the tree already speaks

| # | What | Names | Route |
|---|---|---|---|
| 9 | **Window verbs from the bound protocols** | ~16: `WinGetID`, `WinGetList`, `WinGetCount`, `WinGetClass`, the four `WinWait*`, the four `Group*` | the client already binds `ext_foreign_toplevel_list_v1` (identifier) and `zcosmic_toplevel_info_v1` (geometry, activated/maximized/minimized). **`WinClose` is not one of them**: round 6 found that neither protocol has a `close` request (see `audit/PLAN.md` §4.6) |
| 10 | **Machine metrics** | `SysGet`, `MonitorGet*` (6), `A_ScreenDPI` | `wl_output`; note `A_ScreenWidth` already works, so `SysGet` refusing is inconsistent with the tree's own state |
| 11 | **Drive/info** | `DriveGetSpaceFree`, `DriveGetList`, `DriveGetType` | `statvfs(3)` |
| 12 | **`Shutdown`** | 1 | `systemctl poweroff` |

### 4.3 Expensive — the real gaps

| # | What | Names | Why it is the real work |
|---|---|---|---|
| 13 | **A GUI toolkit** | **148** `Gui`/`ListView`/`TreeView`/`StatusBar` methods + `ToolTip` | needs GTK4 (or an equivalent), and the runner must stay headless-capable. AHK_X11's `gui-add.cr` (8 KB, ~30 control types) is the behavioural reference — **not** copyable (GPL-2.0) |
| 14 | **Accessibility (AT-SPI)** | 48 `Control*` + `WinGetText` | D-Bus, so it works on Wayland. AHK_X11's `at-spi.cr` is 21 KB — the single most valuable piece it has |
| 15 | **Script lifecycle / events** | `Pause`, `Suspend`, `Reload`, `Persistent`, `OnExit` (partly), `OnMessage`, `OutputDebug`, `CallbackCreate` | changes the execution model from "runs to the end" to an event loop. The right v2 shape is events + callbacks, not the v1 thread model |
| 16 | **Genuinely impossible** | ~85 names | Win32 messages, COM, registry, Win32 menu bars, image lists |

### 4.4 What is *not* a gap

A large part of the AHK_X11's surface has a **direct v2 equivalent that already
works** — the gap is a renaming, not a capability. Full mapping:
[`audit/gap-matrix.tsv`](../audit/gap-matrix.tsv). The categories:

- **Renamed, works:** `StringLen`→`StrLen`, `StringLeft/Right/Mid`→`SubStr`,
  `StringTrim*`→`LTrim`/`RTrim`, `StringGetPos`→`InStr`, `StringReplace`→`StrReplace`,
  `StringUpper/Lower`→`StrUpper`/`StrLower`, `MouseClick`→`Click`,
  `FileSelectFile`→`FileSelect`, `FileSelectFolder`→`DirSelect`, `SendRaw`→`SendText`,
  `PixelGetColor`→`PixelGetColor`, `SoundGet`→`SoundGetVolume`, `IniRead`→`IniRead`,
  `VarSetCapacity`→`VarSetStrCapacity`, `Input`→`InputHook`, `URLDownloadToFile`→`Download`.
- **Dropped by v2, not a gap:** `Gosub`, `Goto`, `IfMsgBox`, `Transform`,
  `SetFormat`, `StringSplit`, `EnvAdd`, `FileReadLine`, `RegExGetPos`, `#DefineCommand`,
  `Eval`, `Thread` (the model), the expressionless v1 syntax.
- **Already ahead:** `DllCall` (typed, arbitrary conventions, writes back, plus the
  raven), the full `Map`/`Array`/`Buffer`/`File` classes, `no_std` core,
  the JSON report, the guard rails, `ImageSearch`/`LoadPicture`.

---

## 5. Part 4 — Defects found in this tree

Ordered by severity. All **MEASURED**.

| # | Defect | Reproduce | Impact |
|---|---|---|---|
| 1 | **`#Requires` rejects its own version** | `#Requires AutoHotkey v2.0` + `MsgBox "ok"` → error *"This script requires AutoHotkey v2.0. This interpreter is v2."*, exit 1 | **High** — it broke the first line of most real v2 scripts. **Fixed in round 19**: the directive now compares versions, so `AutoHotkey v2.0`, `v2.0.19` and `>=2.0 <2.1` run and `v2.0.20`, `v9`, `v1` and a bare `v2.0` do not — the same seven answers the reference binary gives |
| 2 | **`ImageSearch` fails with an empty message** | any failing `ImageSearch` → `Error`, `What "ImageSearch"`, `Extra ""`, `Message ""` | **High** — indistinguishable from a caller bug. **Fixed in round 20**: a picture that cannot be loaded is refused with *"Parameter #7 of ImageSearch is invalid."*, which is what the reference answers for a corrupt file and a missing one alike |
| 3 | **`DateAdd`/`DateDiff` reject documented units** | `DateAdd(ts, 11, "Days")` → *"Parameter #3 of DateAdd is invalid."* | **High** — only `s m h d` accepted. **Fixed in round 20**: the unit is read by its first letter, so `Seconds`, `Minutes`, `Hours` and `Days` work, `Months` is read as minutes, and `Years`/`Weeks` are refused — exactly the reference's answers |
| 4 | ~~**Errors lose their cause**~~ **— NOT A DEFECT; corrected in round 21** | `FileCopy` of a missing file → `What "FileCopy"`, `Extra "1"`, `Message "Failed"`; `DirDelete` of a non-empty folder → `Message "Failed"` | **The tree matches v2.** Measured on the binary of v2.0.19 on 2026-11-09: v2 itself answers `Failed` for `FileCopy`, `FileMove` and `DirDelete` — with `Extra "1"` for the two copies — and carries the system message only for `FileDelete`. The tree is exact, including `DirDelete` of a non-empty folder, and `tests/builtins.rs` pins `"Failed \| FileCopy \| 1"` with the measurement beside it. The audit mistook v2's own inconsistency for a defect of this tree; "fixing" it would be a divergence |
| 5 | **`--surface` counts methods as working** | 148 `Gui` method rows are `value`; `g.Show()` refuses | **Medium** — corrupts the project's own metric |
| 6 | **`A_ThisFunc` and `A_Args` mismatch** — **half withdrawn in round 22** | the audit said `A_ThisFunc` answers *"Variable not found."* though the surface lists it as `value`, and that `A_Args` answers the same where v2 gives an Array | **`A_ThisFunc` is correct**: measured on the binary of v2.0.19 on 2026-11-09 it is `""` at the top level and the function's name inside one, and the tree answers `""` and `[F]` in exactly those two places. **`A_Args` is a real gap**: the tree has no value for it. Two blockers, both recorded: the name is **absent from `crates/ahk/reference/ahk-v2.0.19.surface.tsv`** — the extraction missed it, as it missed the nine class types — and the read path, `the_value_of_a_variable_of_the_runtime`, takes `&self`, so it cannot build an Array there |
| 7 | **Ungated shell-outs leak platform errors** | on Windows: `Screenshot` → `sh: line 0: grim: not found`; `SoundPlay` → `paplay: not found`; `Gui.Show()` → *"this desktop is a Wayland session"* on Windows | **Medium** — the uinput paths refuse cleanly (`cfg(target_os)`, 11 gates); the shell-outs did not. **Fixed**: round 17 gated `grim`, `pactl`, `paplay`, `zenity`, the clipboard and `notify-send`; round 18 removed the platform claim from the `Gui` family, which now says *"a window is made by a toolkit, and this runtime has no toolkit that makes one"* on every system |
| 8 | ~~**`ObjAddRef`/`ObjRelease` inconsistent**~~ **— fixed in round 23** | `ObjAddRef` was `value` and `ObjRelease` was refused as *"a name of Windows"* | **Low** — measured on the binary of v2.0.19 on 2026-11-09: `ObjAddRef(1)` and `ObjRelease(1)` both answer `Parameter #1 of … is invalid.`, so the two are symmetric in v2. One handler now serves both, chosen by the table's `variant` byte, and the tree answers what the binary answers for each |
| 9 | ~~**`Screenshot` carries the refusal variant**~~ **— NOT A DEFECT; corrected in round 23** | the audit said `variant: 0` is "the Windows refusal" | **Wrong.** `variant: 0` is the *carried* variant -- `Run`, `MsgBox` and every real handler use it; the Windows refusal is `variant: 100`. `Screenshot` is modelled correctly. It is still the one name the dialect has that v2 does not, and `--surface` still does not print it because the reference table has no row for it |
| 10 | **`MouseGetPos` answers `0,0`** — **host artifact, not a tree defect** | on a host with no pointer device, returns `0,0` and discloses it only in a note | **Low, and only off the box.** On the box the pointer is the absolute uinput device and the position is real; the `0,0` was measured on Windows, where this runtime drives no pointer. The note discloses it. It is still the one place where a wrong value is answered instead of a failure, and that is against the project's own rule -- but the honest fix is a platform question, not a bug in the box |

---

## 6. Part 5 — Documentation drift

The audit found that **seven documents** state numbers that the code contradicts.
This is the errata list; nothing has been rewritten yet.

| Document | Says | Really | Note |
|---|---|---|---|
| `docs/v2-capability-report.md:5` | 828 entries / 638 values | **833 / 643** | |
| `docs/ahk-gaps-report.md:19-20` | 828 / 405 names | **833 / 643** | the "405 names" column is a different metric; verify separately |
| `docs/ahk-v1-dialect.md:1426` | 828 / 638 / 190 | **833 / 643 / 190** | refusal count is right |
| `docs/dialects.md:73` | `total: 828, values: 638` | **833 / 643** | |
| `docs/runner-and-report.md:310,340,343` | 828 / 638, and `entries` cut to 4 of 828 | **833 / 643** | |
| `docs/ahk-x11-comparison.md:772` | 725 entries / 506 value / 219 refused | **833 / 643 / 190** | badly stale |
| `docs/ahk-x11-comparison.md:355-356` | 93 real handlers / 253 `not_carried` | **165 real / 181 `not_carried`** | |
| `docs/ahk-x11-comparison.md:343-346` | `Desktop` is "ten" capabilities | **fifteen** | `AGENTS.md` is right |
| `docs/ahk-x11-comparison.md:404-405` | report keys list omits `warnings` | the payload has **10 keys**, `warnings` included | `docs/runner-and-report.md` has it right |
| `docs/ahk-x11-comparison.md:358-366` | five `not_carried` variants, 0-4 | the surface uses **11** of 22 numbered reasons; **only variant 0 is live** | variants 1-4 have no user: hotkeys, dates, files all landed |
| `docs/ahk-x11-comparison.md:192-193, 438, 543` | pixels read via `XGetImage` | **GDK pixbuf** | |
| `docs/ahk-x11-comparison.md:28` | AHK_X11: 226 v1 commands, 127 done | **223** items; 127 done is right | |
| `docs/runner-and-report.md:48` | "the keys come out in alphabetical order" | struct order (`ok` first) | |

Two structural corrections for the comparison document: its §3.2 "what is still a
sketch" table lists `hotkeys.rs`, `files.rs`, `date.rs`, `settings.rs` as sketches
— **all four are implemented now**, and §5.1's "the six items are done" is the
accurate statement.

---

## 7. Recommendations

1. **Fix the two high-severity bugs first** (§5 #1, #2). A v2 interpreter that
   rejects `#Requires AutoHotkey v2.0` cannot run the corpus it targets, and an
   empty error message is worse than a refusal.
2. **Split the refusal taxonomy before writing any code** (§3.3). Replace the one
   blanket sentence with at least: `windows-api`, `wayland-forbidden`,
   `not-implemented`, `by-design`. That turns 169 unknowns into 115 tickets.
   This is a one-file change to `builtins.rs` and it unblocks the whole plan.
3. **Make `--surface` report capability, not presence** (§3.2), or publish a
   second number. A metric that counts 148 broken `Gui` methods as working will
   be trusted for a year and then be wrong at the worst moment.
4. **Take the cheap 20** (§4.1). They are the refusals most likely to be
   reported as bugs, and each is a few hours. `Process*` and `TrayTip` are the
   most embarrassing.
5. **Take AT-SPI seriously** (§4.3 #14). It is D-Bus, it works on Wayland, and
   AHK_X11's `at-spi.cr` is 21 KB of proven behaviour to reimplement. It is the
   only route to `Control*` and `WinGetText`, and it is what makes an
   automation tool usable on a Wayland desktop.
6. **The GUI is a product decision, not a task** (§4.3 #13). 148 method names
   ride on it. Decide (`--gui` with GTK4) before estimating anything else.
7. **Do not chase the v1 thread model** (§4.3 #15). AHK_X11's `thread.cr` is a
   good reference for *behaviour*, but v2 replaced threads with events and
   callbacks; porting the model would be a step backwards.
8. **Correct the seven documents** (§6) in one commit, and add a test that
   recomputes the surface total so the number cannot drift again. The AHK_X11's
   `/226` is exactly the failure mode: a denominator nobody recomputes.
9. **Gate the shell-outs** (§5 #7) so the tree refuses with a reason on a
   non-Linux host instead of leaking `sh: ... not found`.

---

## 8. Evidence index

| File | What it is |
|---|---|
| [`audit/ahk-x11-feature-inventory.tsv`](../audit/ahk-x11-feature-inventory.tsv) | 260 rows: every AHK_X11 name vs the source, with `file:line` |
| [`audit/ahk-x11-subsystems.md`](../audit/ahk-x11-subsystems.md) | the 15 subsystems, how each really works |
| [`audit/ahk-x11-claims-vs-reality.md`](../audit/ahk-x11-claims-vs-reality.md) | the 226 accounting, the 13 partials, the test coverage |
| [`audit/rust-refusals.tsv`](../audit/rust-refusals.tsv) | 190 refused names, reason, class, doability, note |
| [`audit/rust-refusal-analysis.md`](../audit/rust-refusal-analysis.md) | census by reason and area, per-area verdicts |
| [`audit/empirical-results.tsv`](../audit/empirical-results.tsv) | 192 probes: verdict + exact observed output |
| [`audit/empirical/probes/`](../audit/empirical/probes/) | 50 runnable `.ahk` probes + 51 JSON reports |
| [`audit/empirical/run-probes.ps1`](../audit/empirical/run-probes.ps1) | the probe harness |
| [`audit/gap-matrix.tsv`](../audit/gap-matrix.tsv) | v1 name → v2 name → tree state → gap class |
| [`audit/t1-findings.md`](../audit/t1-findings.md), [`t2-findings.md`](../audit/t2-findings.md), [`t3-findings.md`](../audit/t3-findings.md) | the three working summaries |

### What this audit did not do

- It did not execute AHK_X11 (§0), so its "what works" is source-evidence, not
  runtime evidence. The two `[code-evident]` items are marked.
- It did not measure on the Linux box. The box claims are either quoted from
  `AGENTS.md`'s dated measurements or are tool-presence checks run live during
  this audit (§2.5).
- It did not drive real input. `Send` was only called with an empty string and no
  real click was issued, because both would have typed and clicked into the
  foreground window of the audit host.
- It did not establish whether the 5-entry surface drift (828→833) reflects code
  that landed without a docs update, or a docs update that was reverted. That is
  worth a `git log` on `surface.rs` and `builtins.rs` before the errata commit.
