# The plan: surpass every feature of AHK_X11

> The working document of the goal **"implement to surpass all features of
> AHK_X11"**. It says what "surpass" is measured by, what has landed, what is
> next, and how each change is verified. It is updated every round.
>
> Started 2026-11-09. The audit that produced the gap list is
> [`../docs/ahk-x11-audit.md`](../docs/ahk-x11-audit.md).

---

## 0. Resume here

**State at the end of round 42**: the working tree is clean, both suites are green
(28 on the box), `clippy` is clean, and `ahk --surface` answers **684 values,
149 refusals** from the 643/190 the audit started at.

**What is next, and it is written down in order**: wire `Gui()`, `Add`, `Show`
and `Destroy` to the helper of `crates/ahk-box/helper/ahk-gui.py` -- the first
round in that area where a name of the surface moves. Then `Title`, `Value` and
`GetPos`. Round 39's shape table is the contract; round 41's answers are the
protocol.

**The five rules that produced this state**, and the cost of each lesson:

1. **Measure the binary before writing anything.** Rounds 21, 22 and 23 each
   found a defect claim in the audit that was simply wrong (#4, #6, #9) -- three
   of ten -- and every one of them would have caused a wrong "fix". The defect
   table now carries the binary's answer beside each row, the way the code does.
2. **A refusal is a result.** Three rounds in a row (35, 36) ended in one:
   `ControlGetPos`, the checked pair and `WinGetText` each looked like a
   straightforward call and each failed on a fact only the measurement could
   give. The alternative was a command that answers `Minimize Maximize Close`
   for the text of a window.
3. **A toggle is not a set.** `ControlSetChecked` was measured against a desktop
   that publishes no `CHECKED` state; a click would have worn the name of a set
   and done something else.
4. **Distrust a test that asserts the defect.** Three tests had to be moved
   because the behaviour they described became true (`#Requires` sub-versions,
   the `DateAdd` unit, `StrPut` as a "not carried" example).
5. **The box is the only opinion that counts for a desktop behaviour**, and the
   build there is two steps: copy the sources, *then* build. `audit/atspi-dump.sh`
   and `audit/gtk3-probe.py` are the instruments, and the accessibility bridge is
   **off by default** -- `org.a11y.Status.IsEnabled` must be set for a round and
   put back afterwards.

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

### Round 19 — 2026-11-09: `#Requires` learns to compare versions

The audit's highest-severity defect, and the one most likely to stop a real v2
script at its first line. `the_requires` compared the argument with the *word*
`v2`, so the version the mainline binary writes -- `v2.0`, `v2.0.19` -- never
matched, and a script that carried a normal `#Requires` line refused to load.

**Measured on `AutoHotkey64.exe` 2.0.19**, each form as its own file:

| line | the binary |
|---|---|
| `#Requires AutoHotkey v2.0` | runs |
| `#Requires AutoHotkey v2.0.19` | runs |
| `#Requires AutoHotkey v2.0.20` | **refused** — later than the running version |
| `#Requires AutoHotkey >=2.0 <2.1` | runs |
| `#Requires AutoHotkey v9` | refused |
| `#Requires AutoHotkey v1` | refused |
| `#Requires v2.0` | **refused** — the first word is the interpreter's name |

The tree had a `VerCompare` and a `compare_versions` whose own doc said "the
rule is the one of `#Requires`" -- and `#Requires` was not using them. It does
now: a requirement is a version or a range of constraints, a bare version is
**the least one that runs**, and it has to be the dialect of this run, so `v1`
under a v2 runtime is still refused.

The comparison is **dialect-aware**: the v1 dialect has its own major and its
own version (`1.1.37.02`), and a hard-coded `major == 2` broke four v1 cases --
which is how the omission was caught.

The bare form stays the tree's own: `#Requires v2` runs, and `#Requires v2.0`
does not, which is the same answer the binary gives and for the same reason.

**Verified: all seven forms now answer exactly what the reference answers.**
Both suites green, `clippy` clean.

One test had to be **rewritten rather than repaired**: `a_sub_version_is_not_a_dialect`
asserted that `#Requires AutoHotkey v2.0` is refused. That was the bug, written
down as a requirement. It is now three cases: the sub-version is met, a later
version is refused, and a range is met.

### Round 20 — 2026-11-09: two defects, and two tests that asserted them

**Defect 2 — `ImageSearch` failed with an empty message.** The box discarded the
decoder's reason:

```rust
read(name).map_err(|_| String::new())?
```

A file that existed but was not a picture therefore failed with
`Message ""` — indistinguishable from a bug in the caller. Measured on the
binary of v2.0.19 on 2026-11-09: a corrupt picture and a missing one **both**
answer *"Parameter #7 of ImageSearch is invalid."*, so the tree now answers that
for both, which is also the message it already gave for a missing file.

**Defect 3 — `DateAdd`/`DateDiff` refused the documented units.** The tree
matched the argument as a whole word, `"d" | "D"`, and so refused `"Days"`. v2
reads the unit by its **first letter**. Measured, all eight:

| `DateAdd(ts, 11, …)` | the binary | the tree now |
|---|---|---|
| `"Seconds"` / `"s"` | `…000011` | the same |
| `"Minutes"` / `"m"` | `…001100` | the same |
| `"Hours"` / `"h"` | `…110000` | the same |
| `"Days"` / `"d"` | `20260112000000` | the same |
| `"Months"` | `…001100` — read as **minutes** | the same |
| `"Years"`, `"Weeks"` | refused | refused |

**Two tests asserted the defects.** `a_unit_that_v2_does_not_have_fails_with_the_reason_of_v2`
used `"days"` and expected it to be **refused** — it was the defect, pinned. It
now uses `"years"`, and a new case pins the first-letter rule. This is the
second time in two rounds that a test was the bug's alibi (round 19 had
`a_sub_version_is_not_a_dialect`); worth watching for.

Also checked: no other place in the tree discards a reason with
`map_err(|_| String::new())`.

### Round 21 — 2026-11-09: a defect that was not one

**No code changed, and that is the result.** The audit listed defect 4 as
"errors lose their cause": `FileCopy`, `FileMove` and `DirDelete` answer
`"Failed"` where `FileDelete` answers `(2) The system cannot find the file
specified.`. It read like the same class as round 20's `ImageSearch` bug.

It is not. Measured on the binary of v2.0.19 on 2026-11-09:

| call | the binary | this tree |
|---|---|---|
| `FileCopy` of a missing file | `Failed`, `What "FileCopy"`, `Extra "1"` | the same |
| `FileMove` of a missing file | `Failed`, `Extra "1"` | the same |
| `DirDelete` of a missing folder | `Failed`, `Extra ""` | the same |
| `DirDelete` of a **non-empty** folder | `Failed`, `Extra ""` | the same |
| `FileDelete` of a missing file | `(2) The system cannot find the file specified.` | the same |

**v2 itself is inconsistent, and the tree is faithful to the inconsistency.**
The tree had already recorded this on 2026-11-01 -- the handler says
"Measured on the binary of v2.0.19: the copy of a file that is not there fails
with `Failed`, and the `Extra` is 1" -- and `tests/builtins.rs` pins
`"Failed | FileCopy | 1"` with the measurement beside it. So a future round
that "improved" the message would fail a test that exists for exactly that
reason.

**The lesson is about the audit, not the tree.** A defect in that table has to
carry the binary's answer beside it, the way every behaviour in this tree does.
This row carried only the tree's answer, so it read as a deviation when it was
fidelity. Rows 1, 2, 3 and 7 of the same table were all measured before they
were acted on; this one was not, and it was wrong.

The audit is corrected: the row now says what the binary does.

### Round 22 — 2026-11-09: the other half of a defect that was not one

**No code changed.** Defect 6 said two things. Measured on the binary of
v2.0.19 on 2026-11-09, one of them is not true:

| script | the binary | this tree |
|---|---|---|
| `A_ThisFunc` at the top level | `""` | `[]` — the same |
| `A_ThisFunc` inside `F1()` | `F1` | `[F]` — the same |
| `A_Args` | an **Array**, length 0 | *"This global variable has not been assigned a value."* |

So `A_ThisFunc` was never broken; the audit's claim that it answered
"Variable not found." does not reproduce at the top level or inside a function.
**`A_Args` is a real gap**, and it has two blockers that are worth writing down
rather than rediscovering:

1. **The name is not in the reference table.** `A_Args` does not appear in
   `crates/ahk/reference/ahk-v2.0.19.surface.tsv`, although the binary answers
   it. That is an extraction miss of the same kind the audit found for the nine
   class types -- and it means `--surface` cannot even count the name, so
   "carrying it" would not move the metric that measures the port.
2. **The read path cannot build an Array.** `the_value_of_a_variable_of_the_runtime`
   takes `&self`, and an Array of v2 is made through `classes::construct`, which
   needs `&mut Runtime` and a line. A `Value::List` would be the wrong answer
   again -- that is the round-3 lesson, where a list answered `value` and then
   had no `Length`.

The way in is to seed the empty Array into the globals where the runtime is
built, which has `&mut self` and a line, rather than at the read.

**That is two rounds in a row where a defect row was wrong**, and both were
wrong in the same way: the row described the tree and not the binary. The
audit is corrected.

### Round 23 — 2026-11-09: the batch, and one defect that was real

The plan said to verify the four remaining defect claims against the binary
together, because rounds 21 and 22 had each found a wrong one. Result:

| claim | verdict |
|---|---|
| 5 — `--surface` counts 148 broken `Gui` methods as `value` | **real**, and it is a design question about the metric, not a bug |
| 8 — `ObjAddRef` is `value` while `ObjRelease` refuses | **real, and fixed** |
| 9 — `Screenshot` carries "the Windows refusal variant" | **wrong** — `variant: 0` is the carried one |
| 10 — `MouseGetPos` answers `0,0` | **host artifact** — real on the box, wrong off it |

**Defect 9 was the third mischaracterisation.** The audit called `variant: 0`
"the Windows refusal"; the Windows refusal is `variant: 100`, and `0` is what
`Run`, `MsgBox` and every other carried handler use. `Screenshot` was modelled
correctly all along.

**Defect 8 was real and is fixed.** Measured on the binary of v2.0.19 on
2026-11-09: `ObjAddRef(1)` and `ObjRelease(1)` both answer
`Parameter #1 of … is invalid.`. They are symmetric in v2 and were not in this
tree -- `ObjRelease` refused with *"this is a name of Windows"*, which is not
the obstacle for either of them. One handler now serves both, chosen by the
`variant` byte, which is what that byte is for.

```
ObjAddRef(1)     Parameter #1 of ObjAddRef is invalid.
ObjRelease(1)    Parameter #1 of ObjRelease is invalid.
```

The surface moved **674 → 675 values, 159 → 158 refusals**.

**Three of the ten defects the audit reported were wrong** (4, 6, 9), and all
three were wrong the same way: the row described the tree and never the binary.
The audit's own rule -- every behaviour carries the measurement that justifies
it -- was applied to the tree and not to the defect table. It is now applied to
both.

### Round 24 — 2026-11-09: the largest gap is real, and measured

The plan said to re-verify the audit's doability judgement, because rounds 21-23
found three defect claims that were wrong. The biggest claim in the gap list is
that the `Control*` family is implementable through **AT-SPI**. It is, and this
round proves it on the box rather than reasoning about it.

```
$ busctl --user list | grep a11y
org.a11y.Bus                 1767 at-spi-bus-laun …
org.freedesktop.a11y.Manager 1358 cosmic-comp    …

$ dbus-send --bus=unix:path=/run/user/1000/at-spi/bus_0 --print-reply \
    --dest=org.a11y.atspi.Registry /org/a11y/atspi/accessible/root \
    org.a11y.atspi.Accessible.GetChildren
   array [ struct { ":1.1" "/org/a11y/atspi/accessible/root" }
           struct { ":1.4" "/org/a11y/atspi/accessible/root" } ]
```

The accessibility bus is running, **the compositor itself owns the manager**,
and the registry enumerates applications with accessible roots. So the audit's
`missing-linux-impl` verdict on that family is **confirmed by measurement**.

**The size of what is behind it**, from `--surface`:

| family | refused | what it needs |
|---|---|---|
| `Control*` | **32** | AT-SPI |
| `Edit*` | **7** | AT-SPI (the text of a control) |
| `Win*` | 35 | mostly compositor requests that do not exist (`activate`, `close`, `move`), plus `WinGetText` through AT-SPI |
| `Gui`/`List`/`Tree`/`Status` | 7 | a window toolkit |
| `Com*` | 9 | Windows COM |
| `Reg*` | 5 | the Windows registry |

**The way in, in the order it should be taken.** The tree has no D-Bus
dependency and declares its foreign calls by hand; it *does* shell out to
`grim`, `paplay`, `zenity`, `df` and `notify-send`, and `dbus-send` is installed
on the box. So the cheapest first slice is a shell-out:

1. `org.a11y.Bus.GetAddress` gives the accessibility bus, and `dbus-send`
   connects to it. No new dependency.
2. Walk `org.a11y.atspi.Accessible`: `GetChildren` down from the registry root,
   reading `Name`, `Role` and the `Text` interface.
3. **The hard part is the correlation**: a Wayland toplevel is a title and an
   application id, and an AT-SPI application is a bus name. Matching one to the
   other is what `ControlGetText(Control, WinTitle)` actually needs, and it is
   where the AHK_X11 spent its effort.
4. Then `ControlGetText`, `ControlGetPos`, `ControlClick` follow.

**This is not a one-round cluster.** It is the first thing in this plan that is
bigger than a round, and it is worth modelling as a family the way `Process*`
and `Drive*` were modelled: a trait, a box implementation, and the built-ins
that ride it.

### Round 25 — 2026-11-09: the parsing problem of AT-SPI solved, and one name

**`busctl --json=short` is the way into the accessibility bus.** Round 24 proved
AT-SPI is live on the box; the open question was how a tree with no D-Bus
dependency reads it. Measured:

```
$ busctl --address=unix:path=/run/user/1000/at-spi/bus_0 --json=short call \
    org.a11y.atspi.Registry /org/a11y/atspi/accessible/root \
    org.a11y.atspi.Accessible GetChildren
{"type":"a(so)","data":[[[":1.1","/org/a11y/atspi/accessible/root"], …]]}

$ busctl … get-property :1.1 /org/a11y/atspi/accessible/root \
    org.a11y.atspi.Accessible Name
{"type":"s","data":"xdg-desktop-portal-gtk"}
```

That is JSON that a few string operations read -- no D-Bus crate, and the shape
the tree already uses for `grim`, `zenity` and `df`. The AT-SPI family is now
**unblocked on paper**; it is still a multi-file arc (a trait, the walk, the
toplevel-to-application correlation) and starting it with too little left to
finish would have produced a broken tree, so it starts next round.

**One name landed instead: `OutputDebug`.** Measured on the binary of v2.0.19:
it answers the empty text. There is no debugger on this desktop, and the tree
has one channel for what a script says -- the `output` of the report, where a
`MsgBox` goes -- so the text goes there, and a script that logs with
`OutputDebug` shows up in the report its client reads.

**`Critical` was measured and deliberately left refused.** It answers the
previous setting (`0` then `16`), and it could be modelled as inert script
state -- but its whole meaning is thread interruption, and this runtime runs one
script in one thread. Its existing refusal, *"this runtime runs one script in one
thread, and it cannot make it critical"*, is true. A value that is stored and
has no effect would be the worse answer.

The surface moved **675 → 676 values, 158 → 157 refusals**.

### Round 26 — 2026-11-09: the accessibility client, and a feature withheld

The family started, and **the built-in was deliberately not shipped**. Both halves
are the result.

**What was built and is in the tree:**

- `crates/ahk-box/src/atspi.rs` -- the client of the accessibility bus, over
  `busctl`. `address()` reads `org.a11y.Bus.GetAddress`, `children()` and
  `name()` read the tree, `text()` asks a node for its text, and
  `control_text(title, control)` walks from the registry to a window by its
  **title** and then to a control by its name or its text. The walk is bounded
  in depth (8) and in nodes (200) so that a program with a tree that never ends
  cannot hold the script.
- `desktop.rs` -- `control_text` on the `Windows` trait, with a default that
  says the desktop cannot read the controls. It is a method there and not a
  capability trait of its own: a trait of its own is the rule of this tree, and
  it would have cost an empty `impl` in **eighteen** test files.
- The box implements it.

**Three bugs were found by measuring, and one is the reason the feature is
withheld:**

1. `busctl call` takes the interface and the method as **two** arguments. Passing
   `org.a11y.atspi.Accessible.GetChildren` as one made `busctl` answer
   `Too few arguments.` -- a message that is not in this tree and not in the
   binary, which is what made it hard to place.
2. `org.a11y.Bus.Address` is a **method** (`GetAddress`), not a property.
3. `Accessible.Role` is a **method** (`GetRoleName`), not a property, and
   `org.a11y.atspi.Text` is not on every node.

**Why `ControlGetText` is not shipped.** With the walk working and the dialog
registered -- the registry listed `:1.18` (zenity) with a window whose `Name` is
the title -- `ControlGetText("hello control", "ahk-ctrl-probe")` answered the
empty text, and the label of that dialog is the control that should have been
found. The walk reaches the application and does not reach the control, and the
cause was not found inside the round.

Shipping it would have been the worst outcome this project names: a name that
`--surface` reports as `value` and that answers a **wrong empty** and says
nothing. So the row stays `variant: 100` / `not_carried`, the surface stays
**676 / 157**, and the module and the trait method stay in the tree as the
groundwork the next round builds on.

**The next step is exact**: walk one zenity dialog by hand, dump the `Name` and
the `Text` of every node under the window, and find which call the walk gets
wrong. The buses are named by the registry and change between dialogs --
`:1.8` in one probe and `:1.18` minutes later -- so a probe that hard-codes one
is measuring nothing.

### Round 27 — 2026-11-09: the walk was right and the fixture was empty

Round 26 withheld `ControlGetText` because the walk reached the application and
not the control, and could not say why. The cause is found, and **it was not the
walk**.

**A dump of the whole tree** (`audit/atspi-dump.sh`, the walker that prints the
name, the role and the text of every node) against a live `zenity` dialog:

```
root                                name=[main]     role=[desktop frame]
  /org/a11y/atspi/accessible/root   name=[zenity]   role=[application]
    /org/gnome/Zenity/a11y/efef…    name=[ahk-ctrl-probe] role=[dialog]
      /org/gnome/Zenity/a11y/c92d…  name=[]         role=[generic]
        /org/gnome/Zenity/a11y/b08e… name=[]        role=[generic]
          …                          name=[]         role=[generic]
```

Two things, both measured:

1. **The title match works.** The `Name` of the dialog is the title of the
   window, so the correlation that round 24 called the hard part is there.
2. **`zenity` publishes no control.** Every node under the dialog has an empty
   `Name` and answers no `Text`. The label is on the screen and is not in the
   tree, and **no AT-SPI client can read it** -- not this tree, and not the
   AHK_X11 either. The empty answer was the tree's answer, and it was right.

So `ControlGetText` ships: it walks, it finds the window by title, and it
answers what the tree holds. Verified end to end on the box with the dialog
registered -- `ok: true`, the walk took 83 ms, the answer was the empty text.

**Three findings that a later round must not have to rediscover:**

- **The bridge is off by default.** `org.a11y.Status.IsEnabled` was `false` on
  the box, and applications publish nothing while it is. It is set with
  `busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status
  IsEnabled b true`. The tree does **not** set it: that is a session-wide switch
  of the user's desktop, and a runtime does not flip it to read a label. It is
  the caller's business, and it is recorded here. (The box was restored to
  `false` after the measurement.)
- **Registration takes about ten seconds.** A dialog launched and queried after
  three seconds is not in the registry at all; after ten it is.
- **`zenity` is a bad fixture** for this family: it publishes a skeleton. A
  program that publishes its controls is needed to test a control that has text.

The surface moved **676 → 677 values, 157 → 156 refusals**.

### Round 28 — 2026-11-09: two failures, not one empty text

Round 27 shipped `ControlGetText` answering the empty text for a control that is
not there. **Measured, that is wrong twice.** On the binary of v2.0.19:

| call | the binary |
|---|---|
| `ControlGetText("x", "zzz-no-such-window-zzz")` | **fails** *"Target window not found."*, `Extra` = the title |
| `ControlGetText("x", "A")` — a window that is there | **fails** *"Target control not found."* |
| `ControlGetFocus("zzz-no-such-window-zzz")` | *"Target window not found."* |
| `ControlGetChecked("x", "A")` | *"Target control not found."* |

Neither is an empty text, and the two are different facts. The walk now says
which: `ControlAnswer::Found`, `NoSuchWindow` or `NoSuchControl`, and the
handler turns the last two into the two failures of v2.

**Verified on the box**, with a zenity dialog registered:

```
missing_window=[Target window not found.]
no_such_control=[Target control not found.]
```

and the timings are the proof that the walk really walks: **15 ms** when no
window matches (it looks at each application in turn and gives up) and **185 ms**
when it finds the dialog by title and searches inside it. The error the script
catches carries `What: "ControlGetText"`, the way v2's does.

An empty title is refused rather than treated as a wildcard: v2 reads it as the
active window, and this tree has no active window to read.

### Round 29 — 2026-11-09: the family is implementable and **not verifiable here**

Round 28 ended with a claim that the rest of the control family is "cheap: each
is a handler rather than a build". This round measured that claim and it is
**half wrong**: the handlers are cheap, and **there is nothing on the box to
verify them against**.

**What v2 answers, measured** on the binary of v2.0.19:

| call | the binary |
|---|---|
| `ControlGetFocus("A")` — a window that is there, nothing focused | **`0`**, an Integer |
| `ControlGetFocus("zzz…")` | *"Target window not found."* |
| `ControlGetVisible("x", "zzz…")` | *"Target window not found."* |
| `ControlGetVisible("x", "A")` | *"Target control not found."* |
| `ControlGetEnabled("x", "A")` | *"Target control not found."* |

So the two failures are the ones round 28 already raises, and `ControlGetFocus`
answers the **zero** rather than failing when the window has no focused control.

**Why none of it can be verified here.** Three programs were launched with the
bridge on and the whole tree was dumped node by node:

| program | what it publishes |
|---|---|
| `zenity --info` | the dialog, `Name` = the window title, role `dialog`; **every descendant unnamed, no `Text`** |
| `nautilus /tmp` | the window, `Name` = `tmp`, role `window`; **every descendant unnamed, no `Text`** |
| `chromium --force-renderer-accessibility` | **nothing**: it never registered on the bus |

The trees are real -- the roles are there and the windows are named -- and the
controls inside them are not. So `ControlGetText`, `ControlGetVisible`,
`ControlGetEnabled` and the rest have **no case that can pass on this box**, and
the project's rule is that an unverified behaviour is not shipped.

**What this changes.** The remaining 31 `Control*` names are not a matter of
writing handlers: they need a program that publishes its controls, and there is
none here. Two ways forward, both honest:

1. Find or install a program that publishes a real tree (`atspi` test tools,
   `accerciser`'s probe, a GTK3 application, or Chromium with a working launch),
   and then the family falls quickly -- the client, the walk and the two failures
   are done and verified.
2. Leave the family refused and spend the rounds on what the box *can* answer.

**Nothing was shipped this round**, and the plan's wording from round 28 is
corrected here rather than left standing.

### Round 30 — 2026-11-09: the fixture, and the first control read

Round 29 said the family was blocked on a program that publishes its controls.
**The fixture was written, and the read works.**

`audit/gtk3-probe.py` is a GTK3 window with a label and an entry. GTK3 publishes
what GTK4 on this box does not:

```
gtk3-probe.py                                          role=[application]
  /org/a11y/atspi/accessible/1 name=[ahk-gtk-probe]     role=[frame]
    …/accessible/10            name=[ahk-gtk-probe]     role=[label]
    …/accessible/12            name=[hello control]     role=[label]
    …/accessible/13            name=[]                  role=[text]
    …/accessible/7             name=[Minimize]          role=[button]
```

The window's `Name` is the title, the label's `Name` is its text, and the buttons
are named. That is a tree a control command can be measured against.

**One finding made it work.** `Text.GetText` on a `label` **fails** -- the GTK
side answers `impl_GetText: assertion 'ATK_IS_TEXT (user_data)' failed`, because
a label is not a text *interface*, it is named with its text. So a control whose
name matches and which has no `Text` answers **its name**, which for a label is
the text a script is asking for.

**Verified end to end on the box:**

```
ControlGetText("hello control", "ahk-gtk-probe")  ->  "hello control"   (95 ms)
```

with the two failures already verified in round 28. `ControlGetText` is now the
first name of the family that answers a real control on this desktop, and it is
the piece AHK_X11 has and this tree lacked.

The fixture is the way the rest of the family gets done: `ControlGetVisible`,
`ControlGetEnabled`, `ControlGetFocus` and the `ControlSet*` half all read the
same tree, and each now has something to be measured against.

### Round 31 — 2026-11-09: the two flags, and the bits read from the machine

`ControlGetVisible` and `ControlGetEnabled`, on the spine of round 30.

**Measured on the binary of v2.0.19**, against a window it makes itself
(`Gui` + `Add("Edit")`), because Notepad on this machine has no `Edit1`:

| call | the binary |
|---|---|
| `ControlGetText("Edit1", title)` | `"hello"` |
| `ControlGetVisible("Edit1", title)` | **`1`**, an Integer |
| `ControlGetEnabled("Edit1", title)` | **`1`**, an Integer |
| `ControlGetClassNN("Edit1", title)` | `"Edit1"`, a String |
| `ControlGetFocus(title)` | **`0`** |

So the two flags answer a **number** and not a truth of the host, and the two
failures are the ones already carried.

**The bits are not guessed.** They are read out of
`/usr/include/at-spi-2.0/atspi/atspi-constants.h` on the box, where the enum
starts at `ATSPI_STATE_INVALID = 0`: so `ATSPI_STATE_ENABLED` is **8** and
`ATSPI_STATE_VISIBLE` is **30**. The measurement agrees: the label `hello
control` answers the state `0x43020100`, which carries the 8 and the 30.

**Verified end to end on the box** against the GTK3 fixture:

```
vis=[1]
en=[1]
flag_missing=ERR|Target control not found.
flag_badwin=ERR|Target window not found.
```

`What` is `ControlGetVisible` on the failure, the way v2's is. The surface moved
**677 → 679 values, 156 → 154 refusals**.

`ControlGetClassNN` and `ControlGetFocus` were measured and **not** taken:
the first answers a `ClassNN`, which is a Windows idea that the accessibility
tree does not carry; the second answers the same, and it can be reached through
the `ATSPI_STATE_FOCUSED` bit (12) once there is an answer to give for the name
of what holds the focus.

### Round 32 — 2026-11-09: the write half, and a round trip

`ControlSetText`, verified by writing and then reading the same control back.

**Measured on the binary of v2.0.19**, against a window it makes itself:

| call | the binary |
|---|---|
| `ControlSetText("changed", "Edit1", title)` | **`""`**, and `ControlGetText` answers `changed` after it |
| `ControlSetEnabled(0, "Edit1", title)` | `""`, and `ControlGetEnabled` answers `0` after it |
| `ControlSetEnabled(1, "Edit1", title)` | `""` |

**Its parameters are not the parameters of `ControlGetText`**: the new text
comes **first**, then the control, then the title. Getting that wrong is what the
first attempt at the measurement did, and it answered *"Target control not
found."* for a control that was there -- the failure was the measurement's, not
the binary's.

**A real bug fell out of the round trip.** `Text.GetText` was being called with
the signature `uu`; the interface takes **`ii`**, two *signed* thirty-two bit
numbers. With `uu` the call answers nothing, so the text of a `Gtk.Entry` was
invisible -- `ControlGetText` could only ever read a label, by falling back to its
name. That is why the first attempt at the round trip could not find
`entry value` at all.

**Verified end to end on the box** against the GTK3 fixture:

```
set_ret=[]                        the empty text, as v2 answers
read_back=[written by ahk]        ControlGetText reads what was just written
missing_ctl=ERR|Target control not found.
missing_win=ERR|Target window not found.
```

The surface moved **679 → 680 values, 154 → 153 refusals**.

**`ControlSetEnabled` and `ControlSetVisible` were measured and are not taken.**
v2 answers the empty text for both, but AT-SPI has no call that disables or
hides a control of another program: the accessibility tree *describes* a
control, it does not offer a client a way to change its sensitivity or its
visibility. Answering the empty text without doing the thing would be the silent
wrong answer this plan keeps refusing.

### Round 33 — 2026-11-09: the actions, and a click proven by what it did

`ControlClick` and `ControlFocus`, on the walk of round 30.

**Measured on the binary of v2.0.19**, against a window it makes itself: both
answer the **empty text**, and the two failures are the ones already carried.

**The actions were read off the tree** before anything was written. `Action.GetActions`
answers a list of triples -- name, description, key binding -- and the dump now
prints them:

```
/accessible/7  name=[Minimize] role=[button] acts=[[["Click","Clicks the button",""]]]
/accessible/9  name=[Close]    role=[button] acts=[[["Click","Clicks the button",""]]]
/accessible/13 name=[]         role=[text]   acts=[[["Activate","Activates the entry",""]]]
```

So a click is `Action.DoAction` at the index of the action named `Click`, and a
focus is `Component.GrabFocus`. **The `ii` fix of round 32 shows here too**: the
label and the entry both answer their **text** now (`text=[hello control]`,
`text=[entry value]`), which they did not before it.

**Verified end to end on the box**, and the click is proven by **what it did**
rather than by what it returned:

```
focus=[]                              the empty text, as v2 answers
click=[]                              the empty text, as v2 answers
after=ERR|Target window not found.    the window is GONE
```

The last line is the evidence: the click was on `Close`, and the window was not
there afterwards. A return value cannot be faked past that.

The surface moved **680 → 682 values, 153 → 151 refusals**.

The four commands of the family now share one walk: `find_control` answers the
node or which of the two failures it was, and the text, the flags, the write, the
click and the focus all ride it.

### Round 34 — 2026-11-09: typing into a control, and the position refused

**`ControlSendText` ships, and it is the first command of this tree that uses two
capabilities at once**: the focus goes on the control through the accessibility
layer, and the keys go through the keyboard of our own.

Measured on the binary of v2.0.19: it answers the **empty text**, and the control
holds the text afterwards; the missing control is `Target control not found.`.

Verified end to end on the box against the GTK3 fixture:

```
send_ret=[]                    the empty text, as v2 answers
read_back=[typed by ahk]       ControlGetText reads what was typed
missing_ctl=ERR|Target control not found.
```

and the report says `opened a uinput keyboard for the keys`, which is the second
capability doing its work. The call took 4.8 s, and almost all of it is the four
seconds a new uinput device needs before the compositor adopts it (section 4).

**`ControlGetPos` is refused, and the measurement is the reason.** v2 writes the
coordinates **relative to the client area** of the window. AT-SPI offers three
coordinate spaces -- screen, window and parent -- and the dump now reads all
three. Measured on the box:

```
label  ext=[[110,28,124,20]]   win=[[110,28,124,20]]   par=[[110,28,124,20]]
button ext=[[240,24,28,28]]    win=[[240,24,28,28]]    par=[[240,24,28,28]]
```

**All three are the same number.** The compositor does not distinguish them, so
the client-area origin -- the thing v2 measures from -- is not obtainable: the
best this tree could do is subtract the window's own origin, and that lands on
the *window* origin, which differs from the client origin by the height of the
title bar. Writing that into the variables of a script would be wrong by exactly
the frame, silently. It stays refused.

The surface moved **682 → 683 values, 151 → 150 refusals**.

### Round 35 — 2026-11-09: the checked pair, and a toggle is not a set

`ControlGetChecked` and `ControlSetChecked` were the next pair, and **both are
refused**, for a reason that only a measurement could give.

Measured on the binary of v2.0.19 first:

| call | the binary |
|---|---|
| `ControlGetChecked("Button1", title)` on a fresh check box | **`0`**, an Integer |
| `ControlSetChecked(1, "Button1", title)` | **`""`**, and `ControlGetChecked` answers `1` after it |
| `ControlSetChecked(0, …)` | `""`, and it answers `0` after it |

Then the fixture grew a `Gtk.CheckButton`, and the tree was read:

```
/accessible/14 name=[tick me] role=[check box] state=[[1124075776,0]]
                acts=[[["Click","Clicks the button",""]]]
```

The role is right and the action is there. **The state is not**: `1124075776` is
`0x43000800`-ish, and the bits that matter are absent -- `ATSPI_STATE_CHECKED`
is **4** and `ATSPI_STATE_CHECKABLE` is **41** (the enum has been read from the
header), and neither is set, in either of the two words.

**And clicking it does not change the state.** The tree was dumped before and
after a `ControlClick` on the check box, and the state is the same number both
times:

```
before: state=[[1124075776,0]]
after:  state=[[1124075776,0]]
```

So this desktop offers no way to **read** whether a check box is checked, and
therefore no way to **set** it either: the only act available is a click, and a
click **toggles**. `ControlSetChecked(1, …)` implemented as a click would be
`ControlToggle`, which is a different command wearing the same name -- and the
silent-wrong shape this plan refuses at every turn.

The fixture keeps its check box (it is the record of the measurement), the dump
keeps the three coordinate spaces and the actions, and the surface is unchanged.

**This is the shape of what is left.** Of the 25 control names still refused, the
ones measured so far fall into two groups: AT-SPI has no call for the thing
(`ControlSetEnabled`, `ControlSetVisible`, `ControlGetPos`, `ControlMove`,
`ControlHide`, `ControlShow`), or v2 answers a Windows-shaped value that the tree
does not carry (`ControlGetClassNN`, `ControlGetHwnd`, `ControlGetStyle`,
`ControlGetExStyle`, `ControlGetFocus`, and now the checked pair). Neither group
is a matter of effort.

### Round 36 — 2026-11-09: the text of a window, and the title bar in the way

`WinGetText` was the next candidate, because the accessibility tree looks like it
already holds the answer. **It does, and the answer is not the one v2 gives.**

Measured on the binary of v2.0.19: a window with an `Edit` and a `Button` answers
a **String**, the text of the controls, a line break after each:

```
text=[hello<CRLF>Press me<CRLF>]   len=17
```

and a title that no window has is *"Target window not found."*, the same as the
empty title.

**The obstacle is the title bar.** The tree of a GTK window holds the frame's own
controls beside the client's -- the frame has two children, a `panel` that is the
title bar and a `filler` that is the client:

```
frame  /accessible/1  name=[ahk-gtk-probe]
  panel  /accessible/2   →  /accessible/7  name=[Minimize]
                            /accessible/8  name=[Maximize]
                            /accessible/9  name=[Close]
                            /accessible/10 name=[ahk-gtk-probe]  (the title)
  filler /accessible/3   →  /accessible/12 name=[hello control]
                            /accessible/13 (the entry) text=[entry value]
                            /accessible/14 name=[tick me]
```

A concatenation of everything under the window therefore starts with
`Minimize`, `Maximize`, `Close` and the title -- none of which is a control of
the window in the sense v2 means, and none of which a script scraping a dialog
would expect. The two are told apart only by the role `panel` on the one child of
the frame, which is a GTK layout and not a rule.

So it is refused, with the tree above as the reason. The alternative was a
command that answers "Minimize Maximize Close" for the text of a window, which
passes a naive test and fails the first real one.

**Three rounds running have ended in a refusal, and the shape of the floor is
now clear and recorded**: the `Control*` and `Win*` families that remain need
either a call AT-SPI does not have or a value that is Windows-shaped, and the
three that looked closest -- `ControlGetPos`, the checked pair, `WinGetText` --
each failed on a fact that only the measurement could give. That is the honest
state of this area, and it is worth more than three commands that would have
been wrong.

### Round 37 — 2026-11-09: the bytes of a text

Away from the windows for a round, and a name with no platform in it at all.

**`StrPut` ships.** Measured on the binary of v2.0.19 on 2026-11-09, all five
answers:

| call | the binary | this tree |
|---|---|---|
| `StrPut("hello")` | **12** | 12 |
| `StrPut("hello", "UTF-8")` | **6** | 6 |
| `StrPut("hello", "UTF-16")` | **12** | 12 |
| `StrPut("héllo", "UTF-8")` | **7** | 7 |
| `StrPut("hello", "Bogus")` | **"Invalid Encoding."** | the same |

Twelve is five characters of two bytes and the two of the terminator; six is five
and the one; seven is six bytes and the one, because the accent is two. **The
form that writes the text takes a target address**, and this runtime has no
pointers, so that form is refused with that reason instead of being answered
wrongly.

The surface moved **683 → 684 values, 150 → 149 refusals**.

`StrGet` and `StrPtr` stay refused: both hand over or take an address.

### Round 38 — 2026-11-09: the shortcut pair, half measured

The next candidate was the shortcut pair, because a shortcut is a file operation
and this tree already writes files.

**`FileCreateShortcut` measured cleanly** on the binary of v2.0.19:

| call | the binary |
|---|---|
| `FileCreateShortcut("C:\Windows\notepad.exe", <tmp>.lnk)` | **`""`** |
| the file afterwards | `FileExist` answers **`A`** |

**`FileGetShortcut` could not be measured at all.** The reference binary **hangs**
on it in this environment: a script that makes a shortcut and then reads it back
with four by-reference parameters produces **no report and no error**, and has to
be killed. It was tried twice, once with eight references and once with four, and
both timed out at 12 and 20 seconds. Twice is enough to say it is the binary and
not the probe.

So neither name ships this round:

- `FileCreateShortcut` has its return measured and its shape is known, but the
  body is a **writer**, and the write path through the `Files` capability was not
  established inside the round. Shipping a writer against a guess is how this
  plan has broken before.
- `FileGetShortcut` has no measurement of its answer **or** of its by-reference
  behaviour, and a name with neither is not one that can be written honestly.

**The plan for it, recorded so the next attempt is not a rediscovery.** A
shortcut on this desktop is a **`.desktop` file**, and that is the honest
mapping: `Type=Application`, `Exec` from the target and the arguments, `Path`
from the working directory, `Name` from the description. The reader is the same
file parsed back. The first step is to establish the write path through `Files`
(how `FileAppend` reaches the disk), and the second is a fixture that makes a
`.desktop` and reads it, so that the pair is verified on both sides rather than
only the writer.

### Round 39 — 2026-11-09: the shape of a Gui, measured

The measurement phase of the one remaining frontier, and **nothing was written**:
the contract is the deliverable of the round.

Measured on the binary of v2.0.19 with a window it makes itself:

| the call | the binary |
|---|---|
| `g := Gui("+AlwaysOnTop", "ahk-gui-probe")` | an object of class **`Gui`** |
| `ed := g.Add("Edit", "vField w200", "hello")` | an object of class **`Gui.Edit`** |
| `ed.Hwnd` | an **Integer** (4064076 in the run) |
| `ed.Value` | `hello` |
| `ed.ClassNN` | `Edit1` |
| `g.Show()` | **`""`** |
| `g.GetPos(&x, &y, &w, &h)` | writes the screen rectangle |
| `g.Title` | read **and** written |
| `g.Destroy()` | **`""`** |

**What that says about the build**, and why it is not a round-sized thing:

1. A `Gui` is an **object of the tree**, with a class of its own (`Gui`,
   `Gui.Edit`) and properties that are read and written -- `Title`, `Value`,
   `ClassNN`, `Hwnd`. The class machinery for that exists; what does not is
   anything behind it.
2. `Hwnd` is an **Integer**, and this desktop has no window handles: the windows
   it knows are Wayland toplevels with token identifiers and accessibility nodes
   with object paths. Something has to be named by an Integer before `Hwnd` can
   be answered, and inventing one is the first decision of the build.
3. `ClassNN` (`Edit1`) is the same Windows-shaped value the `Control*` family has
   already been refused for. It would have to be **made up** by the runtime --
   `Edit1`, `Edit2` in order of `Add` -- which is defensible *because the window
   would be ours*, unlike a foreign window's.
4. The window itself needs a **toolkit**. The box has GTK3 (`gi` is installed and
   the fixture publishes a proper tree), so a GTK3 window is the way in, and the
   question the next round must answer is **how the runtime reaches it**: a
   helper process driven over a pipe, a GTK3 window in the runner, or a binding.
   That is an architecture decision with a dependency consequence, and it is the
   one thing this round deliberately did not take.

So the frontier is scoped rather than started. It is **several rounds**: the
toolkit channel, the class of `Gui`, the controls of `Add`, `Show`/`Destroy`, and
the properties. Each is measurable against this table, which is why the table is
worth the round on its own.

### Round 40 — 2026-11-09: the channel of the toolkit, proven

The first slice of the GUI build, and the architecture it takes: **a helper
process**.

`crates/ahk-box/helper/ahk-gui.py` is a GTK3 program that makes a window when it
is told to over its standard input, and answers on its standard output:

```
in   {"op": "window", "title": "ahk-gui-helper-probe"}
out  {"ok": true}
```

**Why a helper and not a binding.** The tree already reaches the desktop by
running the tools of the box -- `grim`, `zenity`, `busctl`, `df` -- and a toolkit
is the same kind of thing. A binding would put GTK inside the runner and a
`Gtk.main` loop inside a runtime that deliberately has neither, and it would
commit the tree to GTK for good. The helper keeps the language crate free of
both, and it is reversible in one file.

**Two things the helper had to get right**, and both were found by writing it:

1. **The GTK loop owns the main thread.** A window made off it does not map.
   So the pipe is read on a thread of its own and every command is handed to the
   loop with `GLib.idle_add`.
2. **The window goes when the pipe does.** The reader ends its loop when the
   input closes, so a runner that dies does not leave a window on the screen.

**Verified on the box** (`audit/gui-helper-test.sh`), with the accessibility bus
on and the helper driven through a fifo:

```
=== the answer of the helper ===
{"ok": true}
=== the tree ===
frame name=[ahk-gui-helper-probe]       role=[frame]
  label name=[the label of the window]  text=[the label of the window]
  text  name=[]                         text=[the text of the entry]
```

The window carries the title it was asked for, and **its controls are in the
tree with their texts** -- so the same walk that the `Control*` family rides can
reach a window this runtime made, which is what `Gui` will need to answer
`Add`, `ClassNN` and `Value`.

**What is not done, in the order it comes**: the protocol has one command and no
window handle; the `Gui` class of the tree is not wired to the helper; `Add`,
`Show`, `Destroy`, `Title`, `Value` and `GetPos` are not carried. Each is
measurable against the shape table of round 39.

### Round 41 — 2026-11-09: the handle and the controls of the window

Step one of the build: **the number that names a control**, and the controls
themselves.

The helper takes a control now, and answers with the two names a script will
use:

```
in   {"op": "add", "kind": "Edit",   "text": "the field"}
out  {"ok": true, "id": 1, "classnn": "Edit1"}
in   {"op": "add", "kind": "Button", "text": "the button"}
out  {"ok": true, "id": 2, "classnn": "Button1"}
```

**The `Hwnd` question of round 39 is answered.** This desktop has no window
handles -- a window is a Wayland toplevel, and a control is a node of the
accessibility tree -- so the runtime numbers its **own** controls as it makes
them, and that number is the `Hwnd`. It is honest here in a way it could not be
for a foreign window: the window is the runtime's own, so the number names
something the runtime really created.

**`ClassNN` is made the way v2 makes it**: `Edit1` is the first edit and
`Button1` the first button, **whatever the order of the `Add` calls** -- the
count is per kind, not the global id. That distinction is in the code and in the
answers above, where the second control added has the id 2 and the class
`Button1`.

**Verified on the box**: the two answers above came back, and the window is in
the tree with the entry's text:

```
frame name=[ahk-gui-helper-probe] role=[frame]
  text  name=[] role=[text] text=[the field]
```

**What is still not done**: the `Gui` class of the tree is not wired to the
helper, so no name of the surface has moved. The next slice is that wiring --
`Gui()`, `Add`, `Show`, `Destroy` -- with the properties after it.

### Round 51 — 2026-11-09: the two listings, measured

Ladder item 3. Both were measured on the binary of v2.0.19, and the round ends
with the measurements banked rather than a shipped name.

| call | the binary |
|---|---|
| `ListVars()` | **`""`**, a String, and it does **not** block |
| `ListLines()` | **`1`**, an Integer |
| `ListVars(1)` | the script does not load: the name takes no argument |

**`ListVars` is clean and `ListLines` is not.** The first answers the empty text,
which is what a command that shows a window and answers nothing should answer,
and the tree's convention for "show me" is the report's `output`. The second
answers **1**, and what that one means is not established: it is not the line the
call was on (the call was on line three), and it is not the number of lines. A
name whose answer is unexplained is not one to write.

**`ListVars` still did not ship**, for a reason the measurement could not give:
the runtime has **no public way to read its own variables**. `Runtime` keeps them
in a field and exposes `note` and nothing else that a handler of the built-in
table can use, so the command needs a small accessor first -- `variable_names`
or the like -- and that is a change to the runtime's public surface rather than a
one-line handler.

So the next attempt on it is: add the accessor, then the handler that writes one
line per variable to the `output`, then a case with two variables. Small, but not
the one-line thing the ladder promised.

### Round 53 — 2026-11-09: the lock keys, and why the ladder stops here

Round 52's box verification was outstanding and is done: **28 suites, surface
689/144 on the box**, the same as the workstation.

Then ladder item 4 was measured on the binary of v2.0.19, and the measurement
makes it a different job than the ladder claimed:

| call | the binary |
|---|---|
| `SetCapsLockState("On")` | **`""`**, and `GetKeyState("CapsLock","T")` answers 1 after |
| `SetCapsLockState("Off")` | `""`, and it answers 0 after |
| `SetCapsLockState("AlwaysOn")` | `""` |
| `SetCapsLockState("Bogus")` | **`""`** -- no failure at all |

**Three obstacles, and the third is the one that decides it:**

1. The state has to be **read**, and this desktop reads it with `EVIOCGLED` on the
   uinput keyboard -- ioctl code behind `#[cfg(target_os = "linux")]`, which does
   **not type-check on the workstation** and has already produced a green Windows
   build over a file that did not compile on the box (section 4). It is a job
   that has to be done with the box build in the loop from the first edit.
2. **`"AlwaysOn"` cannot be honoured.** It means "keep it on across every press",
   which needs a keyboard hook that this runtime does not have. Answering `""` for
   it -- which is what v2 answers -- would be a lie about a state the tree cannot
   hold.
3. **v2 accepts any word silently**, so the tree cannot copy v2's tolerance
   without inventing a meaning for `"Bogus"`.

So the ladder ends here, and that is the useful result: item 4 is **not a quick
win**. It is an ioctl job with a semantic corner (`AlwaysOn`) that this runtime
cannot honour, and the honest version of it is narrower than the reference's --
`On` and `Off` only, with `AlwaysOn`/`AlwaysOff` refused and the difference
written down.

**The ladder, at its end**: items 1, 2 and 3 shipped five names
(`WinGetProcessName`, `WinGetProcessPath`, `Persistent`, `ListVars`, and the
`WinGetPID` that opened the chain). Item 4 is a scoped job for a round with room
for the box build loop, not a tail-end win.

### Round 54 — 2026-11-09: the next quick win, measured

Looking for a rung after the ioctl job, and `SysGetIPAddresses` is one. Measured
on the binary of v2.0.19 on 2026-11-09:

```
type=Array len=4
  [172.29.16.1]  [172.26.48.1]  [192.168.0.45]  [169.254.83.107]
```

**The shape is clean and it is the shape this tree can build**: an array of the
IPv4 addresses of the machine's interfaces, in the order v2 lists them. That is a
name with no Windows in it and nothing to infer.

It did not ship, and the reason is only the size of the change against the
budget of the round: it needs a `Network` capability method (which carries a
default, so no fake desktop moves), a shell-out of the box to `ip -o -4 addr
show` and its parse, the handler that builds the array, the row, and the case.
Six pieces across three files, and a half-applied set of them is worse than none.

**The next attempt, in order**: `Network::ip_addresses` answering the list or the
reason; the box reading `ip -o -4 addr show` and taking the `inet` fields; the
handler building the array the way `WinGetList` does; the case; then the box run
against the interfaces the machine actually has, which the measurement above
gives for the workstation and the box will give for itself.

### Round 56 — 2026-11-09: who owns the helper, settled

No code this round; the state is verified (clean, green, 690/143) and the **one
question that blocks the `Gui` wiring is answered**, so the next session starts
on the writing rather than on a design debate.

**Who owns the helper process.** The `Gui` class cannot own it: a class of the
tree is a value the script may copy, pass and overwrite, and a process is not a
value. The runner cannot own it either without the language crate learning about
processes, which is the one thing it deliberately does not do.

So **the desktop owns it**: `LabDesktop` holds an `Option<TheWindow>`, filled on
the first `gui_open` and used by every operation after it. That is the same
lifetime the rest of the box's hands have -- the pointer device, the keyboard
device -- and it is the thing that already knows how to start and stop a program.

**The close is `Drop`.** A helper that is dropped is a helper whose input pipe
closes, and the helper ends its own loop when its input closes (that is why it
was written that way in round 40). `impl Drop for LabDesktop` calling `close` is
therefore enough for every exit the runner has -- the normal end, an error, a
panic -- with no shutdown hook to remember to call.

**The two consequences to write into the code, not to discover later:**

1. **The helper is lazy.** A script that never makes a `Gui` never starts a
   process, so the cost is only paid by a script that uses it.
2. **A failing helper is a failure of the desktop, not of the script.** If
   `python3` is not on the machine, `gui_open` answers the reason and the `Gui`
   refuses with it -- the same rule as every other capability.

**What is left of the wiring is now mechanical**: the five `Desktop` methods over
the transport, the handle in the `Gui` class's reserved key, and the five members
of the surface (`Gui()`, `Add`, `Show`, `Destroy`, `Title`).

### Round 59 — 2026-11-09: verified on the box, and a limitation found by the sync

Rounds 57 and 58 touched `ahk-box` and had not been run there. They are now:
**the box builds, 28 suites pass, and the surface is the same 690/143** as the
workstation.

**The sync found a real limitation in the wiring, and it is written down here
rather than discovered by a user later.** `gui_open` finds the helper with
`env!("CARGO_MANIFEST_DIR")`, which is **the path of the source tree at the
moment of the build**, baked into the binary. That is right for this use -- the
runner is built where the sources are, and the box builds its own -- and it is
**wrong for a binary that is copied somewhere else**, where the helper is not
under the directory the crate was compiled in.

The three ways out, when it matters:

1. **ship the helper beside the binary** and look for it there first, then fall
   back to the manifest directory -- which is what `raven.rs` does with its own
   helper, and is the convention of the tree;
2. take the path from the environment, with the manifest directory as the
   default;
3. embed the helper as a string with `include_str!` and write it to a temporary
   file the first time it is needed -- no external file at all, at the cost of the
   helper being unreadable where it runs.

**None of them is needed for the runner as it is used today**, and none is done:
it is recorded because a build path in a binary is the kind of thing that works
for a year and then does not, and because the choice is the caller's.

### Round 62 — 2026-11-09: where the wiring has to go, found by reading it

No code. The class wiring was deferred for several rounds on the grounds that it
was "multi-file against an unfamiliar mechanism"; this round read the mechanism,
and the guess was right.

**What `crates/ahk/src/classes/gui.rs` actually is**: a table of the seven class
names of the family, the options of a `Gui`, the argument checks, and one
dispatch function --

```rust
fn refused(class: &str, name: &str, line: u32) -> Option<RuntimeFailure>
```

-- which answers the reason a member cannot work. **It is private**, and the
caller that reaches it is outside the module, so a wired member cannot be added
here alone: the `Some(failure)` answer has to become "handle it" somewhere the
runtime and the desktop are both in hand.

**So the wiring is exactly three places, and all three are now named:**

1. `classes/gui.rs` -- a member of `gui`/`gui.control` stops being a refusal and
   becomes a name the module says it handles;
2. the caller of `refused` -- which is where the runtime and its desktop are
   available, and therefore the only place `gui_open`/`gui_add`/`gui_show`/
   `gui_destroy`/`gui_title` can be reached;
3. `builtins.rs`/its table only if the five members are answered as built-ins
   rather than as members -- which they are not, they are methods of the class.

**What that means for a round that takes it**: read the caller first (one grep
for `refused(` outside the module), then the handle the class object keeps in its
reserved key, then the five members. The desktop side is done and verified; the
contract is round 39's shape table; nothing else is open.

### Round 63 — 2026-11-09: the seam is in the file after all

Round 62 said the dispatch was private with a caller outside the module. **That
was wrong**, and the correction matters because it makes the wiring smaller.

`classes/gui.rs` calls `refused` **only from inside itself**, in three entry
points:

```rust
pub(crate) fn call(class, method, _this: &Value, _arguments: &[Value],
                   _rt: &mut Runtime, line) -> Option<Result<Value, RuntimeFailure>>
pub(crate) fn property(class, name, _this: &Value, _rt, line)     -> Option<Result<Value, ...>>
pub(crate) fn set_property(class, name, _this, _value, _rt, line) -> Option<Result<(), ...>>
```

**Each already has `rt` and `_this`.** So the desktop's five methods are
reachable from exactly where they are needed -- no outside caller to find, no new
plumbing.

**What is actually missing is smaller than a caller**: the class has no **state**.
A `Gui` object needs to carry the handle from `gui_open`, and this module has no
helper for the reserved key of the object yet (the other class modules have one,
per the note in `AGENTS.md`). So the wiring is:

1. a state helper in this module -- the handle in the object's reserved key,
   written when the object is constructed and read by `call`/`property`;
2. `call("gui", "show"|"destroy", …)` and friends answering through
   `rt.desktop().gui_show(…)` instead of the refusal;
3. `property`/`set_property` for `Title`;
4. the construction calling `gui_open`.

**Four steps, all inside `classes/gui.rs` plus wherever the object is
constructed.** The desktop side is done and verified; the contract is round 39's
shape table.

### Round 64 — 2026-11-09: the construction is generic, so the window opens lazily

Round 63 said step 4 of the wiring was "the construction calling `gui_open`".
Reading it, there is no such place: a `Gui` is made by the **generic** machinery --

```rust
("gui", 0, 3, "Gui.Call", "Gui.Prototype.__New"),
```

a row of the constructor table, checked for its argument count and its types and
then made the way every other object is. **There is no gui-specific construction
function to hook**, and the member dispatch of round 63 does not see the
arguments of the construction, so it does not see the title either.

**The answer is to open the window lazily, on the first member that needs one.**
`Show`, `Destroy`, `Add` and `Title` all run through `call`/`property`, which
have the object in hand; the first of them that finds no handle in the object's
reserved key calls `gui_open` with the title the object carries (and the empty
text if it carries none, which is what a `Gui` with no title has), stores the
handle, and carries on. That is also **the lazy behaviour the desktop was built
with** -- a script that never touches a `Gui` never starts a process.

**The four steps become:**

1. a state helper in `classes/gui.rs` -- the handle in the object's reserved key;
2. the first `call`/`property` that needs a window opens it, with the title read
   from the object;
3. the members answer through `rt.desktop()` instead of the refusal;
4. `Title` as a property and a write, over `gui_title`.

Steps 1 to 4 are inside `classes/gui.rs`. **The construction registers nothing
and changes nothing**, which is smaller than round 63 said.

### Round 65 — 2026-11-09: the two consequences the lazy open brings

No code. Looking for the object's reserved key before writing the four steps
turned up **two divergences that the wiring has to answer**, and answering them
badly would be worse than not answering them yet.

**1. This desktop has one window at a time.** The helper replaces its window when
it is told to make another, and `LabDesktop` holds one `Option<TheWindow>`. A
script that makes two `Gui`s therefore gets one window, where v2 gets two. That
is a limitation of the channel as built -- the helper would need a map of windows
and an identifier in the protocol -- and it is **not** something to paper over in
the class. It is fine for the common case (a script makes one window) and it must
be **stated in the refusal-free documentation of the member**, not discovered.

**2. `Gui(options, title)` loses its title.** The construction is generic (round
64), so the title argument never reaches a place that can open the window; the
lazy open at the first member would open it with no title. v2's
`Gui("", "MyTitle")` gives a titled window before anything else runs, and a script
that reads `g.Title` back would get the empty text here.

**The three ways to answer 2**, in the order of how much they cost:

1. **hold the state properly** -- the object's reserved key carries both the
   handle and the title, and the construction is reached through the generic
   machinery to write the title it was given. It is the right answer and it needs
   the reserved-key helper this round went looking for and did not find;
2. **open on the first `Title` write and on the first member that needs a
   window**, accepting that a `Gui` that is never given a title has none -- which
   is what a `Gui` with no title has anyway;
3. **open eagerly at the first member with the empty title**, which is the
   cheapest and the one that silently loses the argument.

**So the next round is not "pure writing" after all**, and it is worth saying so:
the two divergences are design, and picking one under a spent budget is exactly
how this session has gone wrong before. The plan is now: settle 1 (a documented
single-window limitation, or a window map in the helper) and 2 (option 1 if the
reserved-key helper is found quickly), then write the four steps.

### Round 67 — 2026-11-09: the title, decided

Round 66 left one divergence open and set the condition for choosing: take the
proper-state answer if a reserved-key helper turns up in the first look, else the
documented second. **It did not turn up.** The shared helpers of
`classes/mod.rs` are the names, the arities, the prototypes, the read-only names
and the failures -- there is no "the state of this object" among them, and the
class modules reach their own state privately.

**So divergence 2 takes option 2, and it is decided rather than deferred:**

- the window opens **on the first `Title` write and on the first member that
  needs one** (`Show`, `Add`, `Destroy`);
- a `Gui` that is **never given a title has none**, and `g.Title` reads the empty
  text -- which is what a `Gui` with no title has in v2 as well, so the loss is
  only for a title given **at construction**: `Gui("", "MyTitle")` loses `MyTitle`
  where `g.Title := "MyTitle"` keeps it.

**The loss is narrow and it is stated**: one form of setting the title
(construction) is not carried, the other (the property) is. Writing that in the
member's documentation is the honest answer; inventing a state store to close it
would be the kind of change that needs a round of its own.

**What that leaves**: the four steps of round 64, with no decisions left inside
them --

1. on the first member that needs a window, `gui_open` with the title the object
   carries (empty when none);
2. the members answer through `rt.desktop()` instead of the refusal;
3. `Title` as a property and a write, over `gui_title`;
4. the second `Gui` is refused by the desktop, which round 66 already does.

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
