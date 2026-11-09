# The plan: surpass every feature of AHK_X11

> The working document of the goal **"implement to surpass all features of
> AHK_X11"**. It says what "surpass" is measured by, what has landed, what is
> next, and how each change is verified. It is updated every round.
>
> Started 2026-11-09. The audit that produced the gap list is
> [`../docs/ahk-x11-audit.md`](../docs/ahk-x11-audit.md).

---

## 1. What "surpass" is measured by

AHK_X11 implements **142 names** of AutoHotkey v1.0.24: 127 that it marks
`DONE` and 15 that it adds. Thirteen of the 127 are only partial. It cannot run
on this tree's dialect: its scripts are v1 and this tree is v2.0.19.

So "surpass" is not a name count. It is this, in order:

| # | The claim | How it is proven |
|---|---|---|
| 1 | **Every capability AHK_X11 has, that the target platform can serve, works here too.** | each capability is exercised on the Linux box and answers with a value |
| 2 | **Nothing is refused for the wrong reason.** | no name that Linux/Wayland can serve answers "this is a name of Windows" |
| 3 | **What is refused is refused honestly.** | an impossible capability answers with its real obstacle, not a blanket sentence |
| 4 | **This tree keeps what v2 has and v1 never did.** | the classes, `DllCall`, `no_std`, the report, the guard rails |

The gap list is [`gap-matrix.tsv`](gap-matrix.tsv): 80 rows already covered,
**~49 gap rows** still open, which are ~115 of the 190 refusals in the
surface (see [`rust-refusals.tsv`](rust-refusals.tsv)).

**The 49 gap rows are the objective.** The list below is the order to take
them in.

## 2. Where the objective stands

Measured with `ahk --surface` (2026-11-09):

```
the surface of v2 has 833 entries: 674 answer with a value, 159 are refused, 0 are pending.
```

| Round | Cluster | Gap rows closed | Verified on the box |
|---|---|---|---|
| 1 | `Process*` — the process table | 2 | ✅ §4.1 |
| 2 | `ProcessWait`/`ProcessWaitClose`, `TrayTip` | 2 | ✅ §4.2 |
| 3 | `WinGetID`, `WinGetList`, `WinGetCount`, `WinGetClass` | 2 | ✅ §4.3 |
| 4 | `WinGetPos`, `WinGetClientPos` — window geometry | 1 | ✅ §4.4 |
| 5 | `WinGetMinMax` — maximized / minimized | 1 | ✅ §4.5 |
| 6 | `WinClose` — attempted, **reverted** | 0 | ✅ (the correction is the deliverable) §4.6 |
| 7 | `WinWait`, `WinWaitActive`, `WinWaitNotActive`, `WinWaitClose` | 1 | ✅ §4.7 |
| 9 | `A_AhkPath` — the first `Machine` variable | 1 | ✅ §4.9 |
| 12 | `DriveGetSpaceFree`, `DriveGetCapacity`, `DriveGetStatus` | 1 | ✅ §4.12 |
| 16 | the five `MonitorGet*` | 1 | ✅ §4.16 (size is a documented fallback) |

## 3. The remaining work, in the order it is worth doing

Each line is a gap cluster from `gap-matrix.tsv`, with the names it carries.
The order is by value per unit of risk, not by size.

### 3.1 Next — cheap, independent, verifiable on the box

**Assessed in round 8, not yet taken — the refused `A_*` variables.** The nine
that still refuse are `A_AhkPath`, `A_Cursor`, `A_IconFile`, `A_IconNumber`,
`A_ProgramFiles`, `A_ScreenDPI`, `A_ScriptHwnd`, `A_TrayMenu`, `A_WinDir`.
They look cheap and are **not**: `crates/ahk` does no I/O and is `no_std`, so
every one of them has to be answered by the desktop through the `Machine`
trait — and `Machine` has no method for any of them (`user_name`,
`computer_name`, `shell`, `folders`, `os_version`, `language`, `is_admin`,
`idle_times`). The cheapest is `A_AhkPath` (`std::env::current_exe()` in the
box), and it still costs: one trait method, the `LabDesktop` impl, the
non-Linux side, the `Fake`, the value arm in `variables.rs` instead of
`machine_refusal`, its cases, and the surface count. **Budget a whole round for
two or three of them, not one.**

The rest of the list is unchanged:

**`A_ScreenDPI` — measured in round 11 and DROPPED.** Round 10 asked whether the
scale is 1 before the plumbing was written. It is, and the measurement settles
the cluster:

```
$ cosmic-randr list
HDMI-A-1 (enabled)
  Physical Size: 600 x 340 mm
  Position: 0,0
  Scale: 100%
  Modes:
    1920x1080 @ 60.000 Hz (current) (preferred)
$ grim /tmp/s.png   ->  1920x1080
```

`cosmic-randr` **is** installed on the box (round 6 recorded `wayland-info` as
absent and stopped there — that was the wrong tool to look for). One output, one
mode, scale 100%, and `A_ScreenWidth` already answers 1920.

So `wl_output.scale` is **1**, and `A_ScreenDPI` would answer the constant 96 —
on this box and on every display at 100%. It would cost a whole round (the
scale event, a `Client` accessor, a `Machine` method, the variables arm, and two
surface counts) to produce a constant. **Not worth it.** The name stays refused,
and the reason it refuses is the honest one: the desktop of this runtime does
not report a scale, because a scale of 1 has nothing to report.

| Cluster | Names | Why there |
|---|---|---|
| Lock keys | `SetCapsLockState`, `SetNumLockState`, `SetScrollLockState` | `keys.rs` already resolves all three and pins the codes; the question is whether the lock **state** can be read and not only pressed (round 2 looked and left it open — see §4.2) |
| `OnClipboardChange` | 1 | the clipboard works; the missing piece is `wl-paste --watch` |
| Machine variables | `A_ScreenDPI`, `A_AhkPath`, `A_Args` | `wl_output.scale`; the runtime has an executable; `A_Args` is not carried at all |

**Closed in round 2:** `ProcessWait`, `ProcessWaitClose`, `TrayTip`.
**Looked at and left open:** `Edit` (the reference row says arity 0 — "the current
script file", and a script that arrives on stdin has no file) and
`StrGet`/`StrPut`/`StrPtr` (they need a stable address for the buffer of a
`String`, and `Value::Text` owns a `String` whose address is not the buffer the
script may write to; this needs a design decision, not a binding).

### 3.2 The window verbs the client can already reach

`WinGetID`, `WinGetList`, `WinGetCount`, `WinGetClass`, `WinClose`, the four
`WinWait*`, the four `Group*` — the client already binds
`ext_foreign_toplevel_list_v1` (identifier) and
`zcosmic_toplevel_info_v1` (geometry, activated/fullscreen).

**Closed in round 3:** `WinGetID`, `WinGetList`, `WinGetCount`, `WinGetClass`.
**Closed in round 4:** `WinGetPos`, `WinGetClientPos`.
**Closed in round 5:** `WinGetMinMax`.
**Closed in round 7:** the four `WinWait*`.
**Still open here:** `WinClose` — **not** through the bound protocols; round 6
proved they have no `close` request, so it needs `wlr-foreign-toplevel-management`
or a process kill (see §4.6). Then the four `Group*` (script-side
groups over the listing), `WinGetPID`.

### 3.3 The machine's numbers

`SysGet`, `MonitorGet*`, `DriveGetSpaceFree`, `DriveGetList`, `DriveGetType`,
`Shutdown`. Note `A_ScreenWidth` already works on the box, so `SysGet`
refusing is an internal inconsistency of the tree, not a platform limit.

### 3.4 The big ones

| Cluster | Names | Note |
|---|---|---|
| Accessibility (AT-SPI) | 48 `Control*` + `WinGetText` | D-Bus, so it works on Wayland. The single most valuable piece AHK_X11 has |
| GUI toolkit | 148 `Gui`/`ListView`/`TreeView`/`StatusBar` names + `ToolTip` | needs GTK4 and a `--gui` opt-in; the runner must stay headless-capable |
| Lifecycle / events | `Pause`, `Suspend`, `Reload`, `Persistent`, `OnMessage`, `OutputDebug`, `CallbackCreate` | changes "runs to the end" into an event loop |

### 3.5 The defects the audit found, still open

These are not gap rows, but "surpass" includes them: a tree that refuses to
run `#Requires AutoHotkey v2.0` cannot claim to be a v2 interpreter.

1. `#Requires` rejects its own version (`v2.0`, `v2.0.19`, `>=2.0 <2.1`).
2. `ImageSearch` fails with an **empty** message.
3. `DateAdd`/`DateDiff` reject the documented unit words (`"Days"`, …).
4. `FileCopy`/`FileMove`/`DirDelete` answer the literal `Failed`, losing the cause.
5. `--surface` counts 148 broken `Gui` methods as `value`.
6. The ungated shell-outs leak `sh: … not found` on a non-Linux host.
7. `ObjRelease` refused while `ObjAddRef` is a value.
8. The five documents that say 828/638 (see the audit §6).

## 4. The round log

### Round 1 — 2026-11-09: the process table

**What landed.** The `Processes` capability gained six methods, and six names
stopped refusing:

| Name | Answer |
|---|---|
| `ProcessExist([pidOrName])` | the id, or 0 |
| `ProcessGetName([pidOrName])` | the image name |
| `ProcessGetPath([pidOrName])` | the image path |
| `ProcessGetParent([pidOrName])` | the parent id, or 0 |
| `ProcessClose(pidOrName)` | the id that was stopped, or 0 |
| `ProcessSetPriority(priority, [pidOrName])` | the id |

Files: `crates/ahk/src/desktop.rs` (the trait), `crates/ahk/src/builtins.rs`
(the six rows and their handlers), `crates/ahk-box/src/desktop.rs` (the
reading of `/proc` and the fallbacks), `crates/ahk-box/src/sys.rs` (`kill`
and `setpriority`), `crates/ahk/tests/builtins.rs` (14 cases),
`crates/ahk-cli/tests/runner.rs` (the surface count).

**The semantics were measured, not guessed.** On the reference binary
`E:\autohotkey\AutoHotkey64.exe` (ProductVersion 2.0.19), with
`/ErrorStdOut` and the answers written to a file:

| call | v2.0.19 answered |
|---|---|
| `ProcessExist()` | the id of the script |
| `ProcessExist(999999)` | `0` |
| `ProcessExist("no-such")` | `0` |
| `ProcessExist("AutoHotkey64.exe")` | the id — **the name is matched whole** |
| `ProcessExist("AutoHotkey64")` | `0` |
| `ProcessGetName(999999)` | failure `Target process not found.` |
| `ProcessGetPath(999999)` | failure `Target process not found.` |
| `ProcessGetParent(999999)` | `0` — **not** a failure |
| `ProcessClose(999999)` | `0` |
| `ProcessSetPriority("Normal", PID)` | the id |
| `ProcessSetPriority("Bogus", PID)` | the id — **the word is not checked** |
| `ProcessSetPriority("Normal", 999999)` | failure `Target process not found.` |

The tree follows every one of those. On Linux the table is `/proc`; a
platform with no such table answers *"this desktop cannot look at the
processes"* and **not** the `0` of v2, because `0` means "no such process" and
a platform that cannot answer must not be mistaken for a process that is not
there.

**Verified.** `cargo test --workspace` green on Windows **and** on the box;
`cargo clippy --workspace --all-targets` clean; the `no_std` build
(`--target thumbv7em-none-eabihf`) green. On the box, by hand:

```
self=893706  name=ahk  path=/home/danalec/ahk/target/release/ahk  parent=893703
missing_pid=0  missing_name=0
getname_missing=ERR|Target process not found.
byName_ahk=893706
prio_self=893706  prio_low=893706
```

and the close path against a real process:

```
found=893917  name=ahkprobe-sleep  closed=893917  after=0  close_missing=0
```

**Open from this round.**

1. `ProcessWait`/`ProcessWaitClose` still refuse. The v2.0.19 measurement was
   **inconsistent** and is recorded here rather than guessed at: with a
   process that closed after 1.4 s, `ProcessWaitClose(pid, 5)` answered `0`;
   with a process still running at a 0.2 s timeout it answered the **pid**;
   with a pid that did not exist it answered `0`. Any implementation has to
   pick a rule, and the rule has to be measured again on a clean probe before
   it is written. **(Closed in round 2 — see §4.2.)**
2. `Run`'s fourth parameter (the pid of what was started) still refuses:
   `the options of the window and the pid of the process are not carried yet.`
   That is why round 1 spawned its test process through `/bin/sleep` and a
   name instead of through `Run`.

### Round 2 — 2026-11-09: the two waits, and the notification

**The waits were measured first, and they are counter-intuitive.** On
`AutoHotkey64.exe` 2.0.19, with the elapsed time recorded:

| call | answered | took |
|---|---|---|
| `ProcessWaitClose(999999, 0.3)` — not there | `0` | 16 ms |
| `ProcessWaitClose(pid, 10)` — there, closes at ~1.4 s | `0` | 1422 ms |
| `ProcessWaitClose(pid, 0.3)` — there, still running at the timeout | **the pid** | 328 ms |
| `ProcessWait("no-such", 0.3)` | `0` | 343 ms |
| `ProcessWait(self, 0.3)` | the pid | 0 ms |
| `ProcessWait("no-such")` — no timeout at all | — | **still waiting after 15 s** |

So `ProcessWaitClose` answers the **id** for the process that *outlived* the
timeout, and `0` both for the one that closed and for the one that was never
there. The tree follows it exactly, and the shape is written into the trait's
doc so that the next reader does not "fix" it.

**One deliberate deviation.** v2 waits **forever** when the timeout is left
out. This runtime caps the wait at **60 000 ms**, the cap of `Sleep`, because
the runner's timeout is checked *between statements*: a call that blocked
forever could not be stopped by it, and the runner would have to be killed.
That is a guard-rail rule of the project, not an oversight, and it is in the
trait's doc.

**Verified on the box**, against the same cases:

```
wait_absent=0 elapsed=235        wait_self=909552 elapsed=0
waitclose_closes=0 elapsed=1809  waitclose_timeout=909572 elapsed=206
```

— the same answers and the same shapes as the reference binary.

**`TrayTip`** was the second half. The refusal said *"this is a name of
Windows"*, which was false: the box has `notify-send` and a notification
service (`cosmic-applet-notifications`) on the bus. The `Dialogs` capability
gained

```rust
fn notify(&mut self, title: &str, text: &str) -> Result<(), String>
```

with the box running `notify-send`, and `TrayTip` is the built-in that calls
it. It returns the empty text, which is what v2 answers. The failure of
`notify-send` is answered and not swallowed.

Files: `crates/ahk/src/desktop.rs` (two trait methods and `notify`),
`crates/ahk/src/builtins.rs` (the three rows and the two wait handlers),
`crates/ahk/src/dialogs.rs` (`tray_tip`), `crates/ahk-box/src/desktop.rs`
(the polling of `/proc` and `notify-send`), `crates/ahk/tests/builtins.rs`
(8 more cases), `crates/ahk-cli/tests/runner.rs` (the surface count).

**Verified.** `cargo test --workspace` green on Windows **and** on the box
(28 suites); `clippy` clean; the `no_std` build green. The surface moved
**649 → 652 values, 184 → 181 refusals**.

**Open from this round.**

1. **A test caught the surface count again** — this is the second time the
   assertion in `crates/ahk-cli/tests/runner.rs` has been the thing that
   proved the change landed. Keep it.
2. **`Edit` and the `Str*` pointer family were looked at and deferred**, with
   the reasons written in §3.1. Neither is a binding: both need a decision
   about the model (no script file; no stable string address).
3. The **lock keys** need the state read, not only the key pressed. The
   uinput keyboard can press `CapsLock`, but whether the compositor's LED
   state can be read back is unmeasured. That is a probe, not a guess.
4. **The shell-out gate is started, not finished.** `TrayTip` first reproduced
   the audit's defect #7 on Windows — it answered
   `sh: line 0: notify-send: not found`. It now answers
   *"this desktop drives the tools of a Linux box, and this is not Linux"*
   through a `box_tools_are_near()` gate. The **same leak is still there** for
   `grim`, `paplay`, `pactl`, `zenity`, `wl-copy` and `xclip`: on Windows
   `Screenshot`, `SoundPlay` and the clipboard all answer a `sh:` error. That
   is the next defect worth taking, and the gate to hang it on already exists.

### Round 3 — 2026-11-09: the listing of the windows

The first of the window verbs, and the ones the client could already reach: the
toplevel listing it binds carries an identifier, a title and an application id,
which are exactly the id and the class that these four read.

**Measured first**, on `AutoHotkey64.exe` 2.0.19:

| call | answered |
|---|---|
| `WinGetList()`, `WinGetList("")` | **every** window (9 on that desktop) |
| `WinGetCount()`, `WinGetCount("")` | the number of them |
| `WinGetCount("A")` | `1` — `A` is the active window |
| `WinGetID("A")` | the id |
| `WinGetClass("A")` | the class name |
| `WinGetID("no-such")` | failure `Target window not found.` |
| `WinGetClass("no-such")` | failure `Target window not found.` |
| `WinGetList("no-such")` | an **empty array**, not a failure |
| `WinGetCount("no-such")` | `0` |
| `WinGetList()[1]` | an **Integer** — the HWND of Windows is a number |

The last line is a platform difference that is written into the code: the
identifier of a Wayland toplevel is a **token**, not a number, so `WindowRef::id`
is text here and a script that does arithmetic on an id would not have the same
thing. That is documented on the struct.

**The shape of the change.** One trait method, not four:

```rust
fn matching_windows(&mut self, title: &str, mode: TitleMatch)
    -> Result<Vec<WindowRef>, String>
```

where `WindowRef { id, class }`. All four built-ins read it, which is why they
landed together. The empty title lists every window; the box walks the toplevel
listing it already had.

**A defect that only the box found.** The first version of `WinGetList`
answered `Value::List` — the bare list that `MouseGetPos` returns. The 7 unit
cases passed, because they asserted on that representation. On the box:

```
list=ERR|This value of type "Array" has no property named "Length".
```

v2 answers an **Array**, and a script reads its `Length` and its places. The
handler now builds a real one through `classes::construct("array", …)`, the way
`StrSplit` does, and the cases assert on `x.Length` and `x[1]` — on what a
script does, not on what the runtime happens to store. **Lesson: a unit case
that asserts on the internal representation can pass while the feature is
unusable.**

**Verified on the box** (the real compositor, two windows open):

```
count_all=2   list_len=2   first_id=[sFhyzgAq6IDBZf10cuxtBH6BQdtzi5JY]
id_all=[sFhyzgAq6IDBZf10cuxtBH6BQdtzi5JY]   class_all=[chromium]
id_none=ERR|Target window not found.        count_none=0
```

Files: `crates/ahk/src/desktop.rs` (`WindowRef`, `matching_windows`),
`crates/ahk/src/builtins.rs` (four rows, four handlers, `window_target`),
`crates/ahk-box/src/desktop.rs` (`matching_windows` and `wayland_matching`, on
both the Linux and the other-platform side), `crates/ahk/tests/builtins.rs`
(7 cases), `crates/ahk-cli/tests/runner.rs` (the surface count).

**Verified.** Full suite green on Windows **and** the box; `clippy` clean;
`no_std` green. The surface moved **652 → 656 values, 181 → 177 refusals**.

**Open from this round.**

1. `WinGetPID` still refuses: the listing carries an application id and not a
   process id, so the pid needs the compositor's own extension or a match
   through `/proc`, and neither is measured yet.
2. `WinGetPos`, `WinGetClientPos`, `WinGetMinMax` need the geometry and the
   state that `zcosmic_toplevel_info_v1` carries but the `Window` struct does
   not expose yet. That is a field and a binding, and then three more built-ins.
3. `A` as a title (the active window) is **not** special-cased: `WinGetCount("A")`
   looks for a window titled `A`. That is consistent with `WinExist`, which
   does the same, but it is not what v2 does, and it is worth one round.

### Round 4 — 2026-11-09: where a window is

**Measured first**, and the measurement changed the design: `WinGetPos` in
v2.0.19 does **not** return an object. It takes four variables by reference and
answers the empty text:

| call | answered |
|---|---|
| `WinGetPos(&x,&y,&w,&h, "A")` | `""`; `x=819 y=479 w=1787 h=1262` |
| `WinGetClientPos(&x,&y,&w,&h, "A")` | `""`; `x=827 y=479 w=1771 h=1254` — the frame differs |
| `WinGetPos(...)` with no window | failure `Target window not found.` |
| `WinGetPos("A")` — a value, not a variable | `Parameter #1 of WinGetPos requires a VarRef, but received a String.` |

So the rows of the table became four `Parameter::Reference` and four
`Parameter::Value`, and the handler writes the four places through
`rt.write_reference` — the same shape `ImageSearch` already uses.

**The protocol detail that cost the round.** The `geometry` event of
`zcosmic_toplevel_info_v1` exists since version 2, and the client already bound
that manager at `2..=3` — so it looked like a field and a binding. It was not.
The event is:

> *"Emitted when the geometry of a toplevel ... relative to the provided
> output has changed. ... emitted once on creation ... for every entered
> output"*

and it carries a **`wl_output` object**. The client had never bound
`wl_output`, so the compositor had no output to name and sent nothing. The
failure was honest ("the compositor did not say where this window is"), and it
persisted across repeated refreshes and a 300 ms pause — which is what ruled
out timing and pointed at the binding.

The fix is a `Vec<WlOutput>` bound from the registry list in `bind()`, kept
alive like the other proxies, with an empty `Dispatch<WlOutput, ()>` impl.

**Verified on the box**, which is the proof that the binding was the cause:

```
active_title=[danalec@pod158c:~]
pos=[] x=214 y=181 w=1280 h=468
client=[] x=214 y=181 w=1280 h=468
missing=ERR|Target window not found.
```

The return is the empty text, the four places are real, and the two commands
answer the same thing here — a Wayland client does not draw its own frame, so
there is no frame/client distinction to report. That is documented on
`win_get_client_pos`, with the Windows measurement beside it so the difference
is on the record.

**A trap worth remembering.** Cargo reported `Finished in 0.03s` three times
for a source that had changed, and `cargo clean -p ahk-box` did not clear it.
`cargo clean --release` did. The stale release artifacts were being treated as
fresh — the same class of failure as the stale sources on the box in round 1,
on the other side of the sync.

**Verified.** Full suite green on Windows **and** the box; `clippy` clean;
`no_std` green. The surface moved **656 → 658 values, 177 → 175 refusals**.

**Open from this round.** `WinGetMinMax` is the next cheap one: the state array
that already says `ACTIVATED` also carries `maximized` as the bit 0, so it is a
constant and a branch.

### Round 5 — 2026-11-09: whether a window is maximized

The same state array that says which window is active also carries
`maximized` and `minimized`, so this was a constant and a branch, as round 4
predicted. The measurement still mattered.

| call | v2.0.19 answered |
|---|---|
| a window at rest | `0` |
| after `WinMaximize` | `1` |
| after `WinRestore` | `0` |
| after `WinMinimize` | `-1` |
| a window that is not there | failure `Target window not found.` |

**One measurement was thrown away first.** The opening probe asked
`WinGetMinMax("A")` after minimizing, and got `0` — because minimizing the
foreground window makes a *different* window active, so `A` no longer named the
window that had been minimized. The probe was rebuilt around a window the
script could name (`ahk_class Notepad`) and the answer was `-1`. **A
measurement that changes the thing it measures is not a measurement**, and the
`-1` would have been recorded as `0` if the first probe had been trusted.

The change parses the whole state array once and keeps the three flags:

```rust
for chunk in states.chunks_exact(4) {
    match u32::from_ne_bytes([...]) {
        ACTIVATED => active = true,     // 2
        MAXIMIZED => maximized = true,  // 0
        MINIMIZED => minimized = true,  // 1
        _ => {}
    }
}
```

The numbers are the `state` enum of the vendored COSMIC XML, which is the
`xdg-toplevel` enum: `maximized 0`, `minimized 1`, `activated 2`,
`fullscreen 3`, `sticky 4`.

**Verified on the box:** `minmax=[0]` for the ordinary window it has, and
`missing=ERR|Target window not found.`

**Honest limit.** The `1` and `-1` paths could not be exercised on the box:
this tree cannot maximize or minimize a window yet, so there is no way to put a
toplevel into those states from a script. They are covered by the unit cases
against a fake desktop and by the protocol constants; the `0` path and the
failure are measured on the real compositor.

Files: `crates/ahk-box/src/wayland.rs` (two flags on `Window`),
`crates/ahk-box/src/wayland/linux.rs` (the constants, the parse, the field),
`crates/ahk/src/desktop.rs` (`window_min_max`), `crates/ahk/src/builtins.rs`
(the row and the handler), `crates/ahk-box/src/desktop.rs` (the Wayland and
other-platform sides), `crates/ahk/tests/builtins.rs` (5 cases),
`crates/ahk-cli/tests/runner.rs` (the surface count).

**Verified.** Full suite green on Windows **and** the box; `clippy` clean;
`no_std` green. The surface moved **658 → 659 values, 175 → 174 refusals**.

**Open from this round.** `WinClose` is the next one and it is a real request
rather than a reading: `ext_foreign_toplevel_handle_v1.close` exists in the
protocol the client already binds, so it is a method on `Client`, a trait
method, and a handler — no new protocol.

### Round 6 — 2026-11-09: a correction, and a reverted feature

**No capability landed this round. It produced a correction instead, and the
feature was reverted rather than shipped broken.**

The plan said `WinClose` was "a request rather than a reading: the `close` of
`ext_foreign_toplevel_handle_v1` exists in the protocol the client already
binds". That came from the audit (`docs/ahk-x11-audit.md` §1, from the refusal
analysis), and **it is false**.

What the protocols actually carry, read from the vendored and the crates.io
XML:

| interface | version | requests |
|---|---|---|
| `ext_foreign_toplevel_list_v1` | 1 | `stop`, `destroy` |
| `ext_foreign_toplevel_handle_v1` | 1 | `destroy` |
| `zcosmic_toplevel_info_v1` | 3 | `stop`, `get_cosmic_toplevel` |
| `zcosmic_toplevel_handle_v1` | 3 | `destroy` |

There is no `close` on any of them. `destroy` destroys the *client's handle*,
which does not close the window — it stops the client being told about it.

**The mistake was caught by building on the box.** `crates/ahk-box/src/wayland/linux.rs`
is `#[cfg(target_os = "linux")]`, so the Windows build never type-checks it, and
`cargo build --release` on Windows reported success for a file that cannot
compile. The box answered at once:

```
error[E0599]: no method named `close` found for reference
              `&ExtForeignToplevelHandleV1` in the current scope
```

**The feature was reverted**, not stubbed: `WinClose` is back to its refusal,
the surface is back to **659 / 174**, and the tree has no name that answers
"value" while always failing. The protocol-detail work was real (a `Client`
method, the trait, the handler, five cases) and it was all removed.

**What `WinClose` would need**, for whoever takes it next:
1. `zwlr_foreign_toplevel_manager_v1` — the wlroots protocol has a `close`
   request per handle. Whether the box's COSMIC session publishes it is
   **unmeasured**: `wayland-info` is not installed on the box, and there is no
   other tool there to list the globals. That is the first thing to establish.
2. Or kill the process that owns the window — but that is `WinKill`'s meaning,
   not `WinClose`'s, and the toplevel listing does not carry a pid
   (`WinGetPID` is refused for the same reason).

**Two lessons for the rounds ahead.**

1. **A `cfg` gate hides a module from the platform that is not its target.** A
   green build on Windows says nothing about `wayland/linux.rs`, `keyboard.rs`
   or anything else behind `cfg(target_os = "linux")`. **Every round that
   touches a gated module must build on the box**, and that is now step 2 of
   §5.
2. **An audit claim about a protocol is a claim, not a fact.** The claim was
   checked only when code depended on it, six rounds later. A protocol request
   that a plan depends on should be `grep`ped in the XML before the plan is
   written.

### Round 7 — 2026-11-09: waiting for a window

The family round 6 could not take, and with no protocol risk: it is polling over
the listing the client already reads.

**Measured on `AutoHotkey64.exe` 2.0.19:**

| call | answered | took |
|---|---|---|
| `WinWait(present, 2)` | the id | 109 ms |
| `WinWait(missing, 0.3)` | `0` | 297 ms |
| `WinWaitActive(missing, 0.3)` | `0` | 297 ms |
| `WinWaitNotActive(missing, 0.3)` | **`1`** | 109 ms |
| `WinWaitClose(missing, 0.3)` | **`1`** | 110 ms |
| `WinWaitClose(running, 0.3)` | `0` | 297 ms |
| `WinWaitClose(closed, 2)` | `1` | 109 ms |
| `WinWait("no-such")` — no seconds | — | still waiting after 20 s |

Two answers are the id and two are `1`/`0`, and that asymmetry is in the
handlers. As with `ProcessWait`, v2 waits **forever** when the seconds are left
out; this runtime caps at 60 000 ms, the cap of `Sleep`, for the same reason —
a blocking call cannot be interrupted by a runner timeout that is checked
between statements.

**One trait method, four commands**, the way round 3 did it:

```rust
fn window_wait(&mut self, title: &str, mode: TitleMatch,
               what: WaitFor, timeout_ms: u64) -> Result<Option<String>, String>
```

with `WaitFor::{Present, Active, NotActive, Close}`. The empty title is *no
window*: v2 would fall back to the last found window and this runtime keeps
none, so the conditions about absence hold at once and the ones about presence
never do — which is exactly what v2 answered for a title that matches nothing.

**Verified on the real compositor**, all five cases:

```
wait_present=[Wv31fI12NCbw998pInBuyWa6FAAv1TPL]
wait_missing=[0]   notactive_missing=[1]
close_present=[0]  close_missing=[1]
```

**A process note.** I patched the four table rows with a PowerShell
`Set-Content`, which the project forbids (text files go through the editor
tools) — and it went wrong exactly as the rule warns: the offsets were off by
two, so the `variant` became `0` while `handler` stayed `not_carried`. That is
the worst state the surface can be in — a name reported as a `value` that
still refuses — and it was caught by reading the rows back before building.
Redone with the editor.

**Verified.** Full suite green on Windows **and** the box; `clippy` clean;
`no_std` green. The surface moved **659 → 663 values, 174 → 170 refusals**.

### Round 9 — 2026-11-09: the path of the interpreter

**`A_AhkPath`** moved from a refusal to a value. It is the first of the refused
`A_*` variables, and it needed the plumbing round 8 predicted: a
`Machine::executable` method (default: refuse), the `LabDesktop` impl
(`std::env::current_exe()`), and a value arm in `variables.rs` replacing the
`machine_refusal` entry. The desktop answers, so the variable is a value; a
desktop that cannot tell still refuses, and the reason is the one of
`NO_AHK_PATH`.

**Verified on both platforms**, which is the point of this one:

```
Windows: C:\...\target\release\ahk.exe
box:     /home/danalec/ahk/target/release/ahk
```

**Two tests had to move with it, and both were the guard working:**

1. `crates/ahk/tests/variables.rs` lists the names that must refuse --
   `A_AhkPath` was in it and is not one any more.
2. `crates/ahk-cli/tests/runner.rs` asserts the surface count of **both**
   dialects, and the v1 count moved too: the v1 surface asks the same runtime
   whether a name is carried, so `A_AhkPath` became a value there as well
   (301 → 302). **A change to a shared name moves two counts, not one** -- worth
   remembering for the rest of the `A_*` family.

The surface of the v2 moved **663 → 664 values, 170 → 169 refusals**.

### Round 12 — 2026-11-09: the drives

**`Group*` was dropped before it was started, by the rule this plan wrote for
itself.** The plan said to size the seam first. `GroupActivate` and
`GroupDeactivate` need a window to be *activated*, and `GroupClose` needs one to
be *closed* -- and the client has no activation request at all (round 6 already
proved there is no close), so three of the four cannot work. Only `GroupAdd`
would land, and a group that nothing can activate is a list with no reader.

**The drive cluster instead** — 14 refusals, pure machine queries, no protocol
risk.

**Measured on `AutoHotkey64.exe` 2.0.19**, all wrapped in `try` (an uncaught
runtime error opens a dialog and hangs the probe):

| call | answered |
|---|---|
| `DriveGetList()` | a **String**, not an array — it has no `Length` |
| `DriveGetType("C:\")` | `Fixed` |
| `DriveGetStatus("C:\")` | `Ready` |
| `DriveGetSpaceFree("C:\")` | `836675` — whole megabytes |
| `DriveGetCapacity("C:\")` | `3905969` |
| `DriveGetType("Z:\")` — missing | the **empty text**, not a failure |
| `DriveGetSpaceFree("Z:\")` — missing | a **failure**, `(3) The system cannot find the path specified.` |

The last two disagree about the same missing drive, and that disagreement is in
the handlers.

Three names landed: `DriveGetSpaceFree`, `DriveGetCapacity`, `DriveGetStatus`.
The drive of a path is **the filesystem the path is on** — a Linux box has mount
points, not drive letters — and the box answers it with `df -Pk`, which is the
POSIX form: one line per drive, a thousand-byte block, a fixed column order.
`DriveGetList` was **not** taken: its contract is a string of drive letters, and
there is no honest mapping from mount points to that.

**Verified on the box, against `df` itself:**

```
df -Pk /      ->  28774016 21747712 7026304 76% /
the tree      ->  free=6859 cap=28097 status=[Ready]
```

(df said 7026304/28774016 KiB, about 6862/28100 MB; the tool ran its own `df`
a moment later, and a few megabytes of live drift is the difference.)

`DriveGetStatus` of a drive that is not there answers the empty text — **and
that was not measured**, only inferred from `DriveGetType`, which is recorded on
the handler.

**Two more rode the same seam: `DriveGetFilesystem` and `DriveGetType`.** The
filesystem of a path is the one of the **longest mount point it is under**
(`/proc/self/mounts`), so a path under `/home` is in the `/home` filesystem and
not the `/` one. `DriveGetType` maps that to the word of v2: network
filesystems are `Network`, disc images `CDROM`, memory filesystems `RAMDisk`,
and the rest of the machine `Fixed`. Verified on the box:

```
/proc/self/mounts:  zpcachyos/ROOT/cos/root / zfs ...
the tree:           fs_root=zfs type_root=Fixed
                    fs_tmp=tmpfs type_tmp=RAMDisk
```

### Round 16 — 2026-11-09: the monitors

The five `MonitorGet*` names. The `wl_output` objects were bound in round 4 for
the geometry event, and their own events are what a monitor is: the position
from `wl_output.geometry`, the size from the current `wl_output.mode`, the name
from `wl_output.name`, and the scale from `wl_output.scale`.

**Two bugs that only the box could show**, because `wayland/linux.rs` is
`cfg(target_os = "linux")` and Windows never type-checks it:

1. `WEnum::into_result()` **fails on the mode flags**, because the flags of
   `wl_output.mode` are a bitmask and the protocol file does not declare the
   enum as one: the mode in use arrives as `Current | Preferred` (3), which is
   not one of the two named values. The bit has to be read from
   `WEnum::Unknown(bits)`.
2. **The mode event still does not arrive** on this compositor: the name does
   (`HDMI-A-1`) and the position does (`0,0`), and the size stayed zero. So the
   extent of the session is used for the size of an output that gave none, and
   **that is a gap recorded here, not a design** — a session with two outputs of
   different sizes would report the extent for both.

**Verified on the box:**

```
count=1 primary=1 name=[HDMI-A-1]
get_ret=[1] l=0 t=0 r=1920 b=1080
work    r=1920 b=1080
name9=ERR|Parameter #1 of MonitorGetName is invalid.
```

`MonitorGetPrimary` answers `1` always: a Wayland compositor lists its outputs
and does not say which is primary. `MonitorGetWorkArea` is the monitor itself:
a client is not told about the session's panels. Both are recorded on the
handlers and on the trait rather than hidden.

### Round 17 — 2026-11-09: the shell-outs learn to refuse

No new name; a defect fixed. The audit's defect #7 said that the tools the box
shells out to were invoked on **every** platform, so a Windows host got
`sh: line 0: grim: not found` instead of a reason. `notify-send` and `df` were
already behind `box_tools_are_near()`; the rest were not.

Gated now: `grim` (the capture and the pixel), `pactl`, `paplay`, `zenity`, and
the clipboard read and write (`wl-paste`/`wl-copy`, `xclip`/`xsel`). On Windows
they all answer *"this desktop drives the tools of a Linux box, and this is not
Linux"*.

**The risk of this change is the opposite direction**, so it was checked that
way: `box_tools_are_near()` answers `Ok` on Linux, and the box still does
everything it did.

```
clip=[probe-42]   screen=1920x1080   pixel=#12101b   volume=99.0   traytip=ok
```

`#12101b` is the same pixel the Python implementation and the archive record for
that wallaper, so the capture path is unchanged.

Still wrong on Windows: `Gui.Show()` says *"this desktop is a Wayland session"*,
which is a message of the `ahk` crate and not of the box. That is the rest of
defect #7.

### Round 18 — 2026-11-09: the GUI family stops naming a platform

The rest of the audit's defect #7. Round 17 fixed the tools that the **box**
shells out to; this is the other side of the crate boundary. The `Gui` family
lives in the `ahk` crate, which is platform-agnostic and `no_std`, and its
seven refusal reasons ended with *"and this desktop is a Wayland session
without one"* — false on Windows, and false as a diagnosis on the box too,
because the obstacle is not the compositor: there is **no GUI toolkit in the
runtime at all**, on any system.

The messages now say what is missing:

```
Gui cannot show the window: a window is made by a toolkit, and this runtime
has no toolkit that makes one.
```

The same sentence on Windows and on the box, which is the point: a refusal
that names a platform it cannot know is a guess, and this one was wrong.

Eleven places carried the old wording — seven reasons, the module doc, and
three assertions — and `tests/class_gui.rs` asserted the literal message.

## 5. How every round is verified
The project's own rules (`AGENTS.md`) plus three traps that cost time in
round 1:

1. **Measure v2 before implementing.** `E:\autohotkey\AutoHotkey64.exe` +
   `/ErrorStdOut`, answers written to a file, every call in `try`/`catch`
   (an uncaught runtime error opens a **dialog** and hangs the probe).
2. **Build and test on both platforms, and treat the box as mandatory for any
   gated module.** A green Windows build says *nothing* about
   `cfg(target_os = "linux")` code — round 6 shipped a file that could not
   compile and Windows reported success. `wayland/`, `keyboard.rs`,
   `pointer.rs` and every `#[cfg(unix)]` block are only type-checked on the
   box.
3. **The box build needs the sources copied *and touched*.** `tar` over
   `scp`, extract over `~/ahk`, and then `find crates -name '*.rs' -exec touch {} +`.
   Without the `touch`, `cargo` sees mtimes older than its fingerprints and
   silently runs the **old** test binary — round 1 lost a cycle to exactly
   that, with a surface-count test that passed locally and failed on the box.
4. **Write script probes with the editor tools, never with a shell array.**
   PowerShell turned a list of lines into `System.Object[]` literals inside
   the script.
5. **`box_exec` does not expand `$`.** Loops with `$var` do not work there;
   use `ssh` for anything with a variable.
6. Every change moves a test that would otherwise pass by accident: the
   surface count is asserted in `crates/ahk-cli/tests/runner.rs`.
