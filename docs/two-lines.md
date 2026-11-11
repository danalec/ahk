# The two lines of the tree: `master` and `audit-ahk-x11-parity`

A record of the state of the two branches on 2026-11-10, measured with `git`
and with the suite of each. It is here so that whoever picks this up does not
have to re-derive the shape of the divergence, and so that the merge is not
attempted without knowing what it costs.

Read it before touching either branch. The short of it: **the two branches
are not two versions of one line -- they are two lines, and the branch is
not a superset of `master` nor the other way round.**

---

## 1. What is there

| | `master` | `audit-ahk-x11-parity` |
|---|---|---|
| Where | the tree of the owner | `.agents/worktrees/audit-ahk-x11-parity` |
| Worktree | the root | the one above |
| Head on 2026-11-10 | `1b9b351` | `91c23c2` |
| Commits over the merge base | 2 | 238 |
| Surface of v2 | 833 entries: **681 answer, 152 refused** | 833 entries: **759 answer, 74 refused** |
| Suite | green | green |
| Pushed | 41 commits over `forgejo/master` | **nothing** -- the branch is local only |

Both lines are clean as of this writing. There is no merge in progress in
either, and no stash.

---

## 2. The merge base and what it means

The merge base of the two branches is `f5e8d54`, which is the tip of
`master` before the two commits of 2026-11-10 (`d78f7f8`, `1b9b351`). The
branch is 238 commits over it and `master` is 2.

**The branch merged `master` with the `ours` strategy** (`d45276e`, "Merge
master into the audit branch, keeping the audited tree"). That records
`master` as an ancestor without taking its content, so:

- the branch's *history* contains `master`, and
- the branch's *tree* does not.

The branch's own plan says why (`audit/PLAN.md`, section 0): the two
histories had diverged far enough that a content merge mixed incompatible
APIs -- the AST parameter shape, the runtime state, the desktop traits -- so
the merge keeps the audited tree and the one name `master` carried that the
branch did not (`IsSet`) was ported by hand in `0147eb7`.

That decision is what makes the branches look merged when they are not.

---

## 3. The divergence, measured

Comparing the two *trees* directly (`git diff master audit-ahk-x11-parity`),
not the histories:

```
411 files changed, 26098 insertions(+), 24851 deletions(-)
125 files only on master
234 files only on the branch
 52 files changed on both
```

### 3.1 What only `master` has

**117 of the 125 files are `scripts/`** -- the differential harness
(`scripts/differential.py`, its README and the 111 cases under
`scripts/differential/cases/`).

The other 8 are code that `master` has and the branch does not:

| File | What it is |
|---|---|
| `crates/ahk/src/float_text.rs` | the 17-significant-digit rule of v2 (`1/3` is `0.33333333333333331`) |
| `crates/ahk/src/behavior.rs` | the behavioural probes behind `--surface --behavior` |
| `crates/ahk/src/text16.rs` | counting and cutting a text in UTF-16 code units |
| `crates/ahk/src/classes/regexmatchinfo.rs` | the `RegExMatchInfo` class type |
| `crates/ahk-box/src/drives.rs` | the drives of the machine |
| `crates/ahk-box/src/locks.rs` | the lock keys of the machine |
| `crates/ahk-box/src/process.rs` | the processes of the machine |
| `crates/ahk-box/src/trash.rs` | the trash of the machine |

The branch has the *functionality* of some of these in another shape -- its
`crates/ahk-box/src/desktop.rs` carries the process family inline, with a
different signature (`fn process_exist(&mut self, _pid_or_name: &str)` against
`master`'s `fn process_exist(&mut self, _which: Option<&str>)`). The two are
not interchangeable.

### 3.2 What only the branch has

229 of the 234 files are under `audit/` -- the audit of AHK_X11: the plan
(`audit/PLAN.md`, 263 KB), the gap matrix, the empirical probes and their
results, the three findings of the subagents, and the scripts that drive the
measurements.

The other 5 are code:

| File | What it is |
|---|---|
| `crates/ahk-box/helper/ahk-gui.py` | the GTK helper (1506 lines) that draws the window and the controls |
| `crates/ahk-box/src/atspi.rs` | the accessibility bus (497 lines) that reaches the controls of another application |
| `crates/ahk-box/src/gui.rs` | the window of the runtime, over the helper |
| `crates/ahk/src/classes/menu.rs` | the `Menu` / `MenuBar` class type (504 lines) |
| `docs/ahk-x11-audit.md` | the audit of AHK_X11 |

### 3.3 The 86 names the branch answers and `master` refuses

The branch answers 86 names that `master` refuses, and `master` answers 8
that the branch refuses (`DriveGetLabel`, `DriveGetSerial`, `Pause`,
`SetCapsLockState`, `SetNumLockState`, `SetScrollLockState`, `StrPtr`,
`Suspend`).

The 86 are the GUI and control family: `ControlAddItem`, `ControlClick`,
`ControlGetText`, `Gui.AddButton`, `Gui.ListView.Get`, `Gui.TreeView.Add`,
`Menu.Add`, and the rest. **This is the real work of the branch**, and it is
work `master` does not have.

The 8 that `master` has and the branch does not are the ones that moved into
their own module in `ahk-box` (the drives, the trash, the processes) and the
suspend family.

---

## 4. What a merge would cost

**A merge of the branch into `master` is not a merge -- it is a replacement.**
Measured on 2026-11-10 on a throwaway worktree:

```
git merge audit-ahk-x11-parity
CONFLICT (content): Merge conflict in crates/ahk/tests/language.rs
conflicted files: 3   (AGENTS.md, crates/ahk/src/dialect.rs, crates/ahk/tests/language.rs)
```

Only three files conflict, and each one is a single hunk that both sides
added to. But resolving them is not enough: the merged tree **deletes all 125
files that only `master` has**, including `scripts/differential.py`, the 116
differential cases, `crates/ahk/src/float_text.rs` and the four `ahk-box`
modules. The result does not compile, because the branch's tree has a
different `V2` (`pub struct V2;` with no `extended` field, against `master`'s
`pub struct V2 { pub extended: bool }`) and a different `Dialect` trait (264
flags against 287).

The two lines are not two versions of one tree. They are two trees that
share a common ancestor and diverged in the *internal shape* -- the AST
parameter shape, the runtime state, the desktop traits -- as the branch's plan
says.

---

## 5. What to do

### 5.0 The port has started

The first family is ported: **`FileCreateShortcut` and `FileGetShortcut`**, the
shortcuts of the machine. The tree answered 681 of the 833 names on
2026-11-10 and answers **683** since 2026-11-11.

The port is done family by family, and each family is:

1. measured on the binary of v2.0.19 (the order of the parameters and the
   shape of the answer are read from the binary, not from the branch),
2. added to the capability of the tree (`crates/ahk/src/desktop.rs`) with
   the refusal of its own,
3. implemented in `crates/ahk-box` over the machine,
4. wired into the built-in table of the tree (`crates/ahk/src/builtins.rs`),
5. pinned by cases of its own (`crates/ahk/tests/builtins.rs`), and
6. **verified on the box**, where the machine is the one the answer is
   about.

The shortcut family is done and verified on the box: the `.desktop` file is
written with the mode of an executable, read back, and every one of the five
fields comes back.

The second family is done too: **`A_AhkPath`**, the path of the
interpreter. The tree answers 684 of the 833 names since 2026-11-11, and the
answer is verified on the box too.

Three options, in the order they cost.

### 5.1 Keep the lines apart (the honest default)

The branch carries 238 commits of GUI and control work and a 263 KB audit
that `master` does not have. `master` carries 41 unpushed commits of the
KeySharp audit, the differential harness and the v2.1 dialect work that the
branch does not have. Neither is disposable.

The cheapest safe thing is to leave them apart and **push the branch**, so
that the 238 commits are not local only:

```
git push -u forgejo audit-ahk-x11-parity
```

That is a push to a remote the owner names; it is not done here.

### 5.2 Bring the branch's work to `master` as a series of ports

The 86 names are the value. Porting them is not a merge: it is taking the
`atspi.rs`, `gui.rs`, `menu.rs` and the helper onto `master`'s tree and
resolving the trait differences by hand, one name family at a time, each with
its own cases. That is the work of several rounds and it is the only way that
keeps both lines.

The order that costs least:

1. `crates/ahk-box/src/gui.rs` and the helper -- the window, with no change
   to the trait shape beyond the methods it needs.
2. `crates/ahk-box/src/atspi.rs` -- the accessibility bus.
3. `crates/ahk/src/classes/menu.rs` -- the `Menu` / `MenuBar` class type.
4. The 86 control names, family by family.

**The first family is ported (2026-11-11), and the third came with it.**
The window puts a menu on itself (`g.Menu := mb`), and the menu is a
class of its own -- so the two could not be separated without the window
answering that a menu bar is not joined to it yet. The port is a script
and not a copy by hand (`audit/port-gui-window.py`,
`audit/port-gui-class.py` and `audit/port-gui-names.py`): the
two trees have drifted, and a copy would have taken the drift with it.
Measured on the box: the window opens, takes its controls, and an event
reaches the handler of a control.

The bus of the accessibility is ported too (2026-11-11): the `Control*`
names read the controls of *another* window, and
`crates/ahk-box/src/atspi.rs` is the bus that answers them. The port is a
script as well (`audit/port-control-family.py` and
`audit/port-control-cases.py`) -- the rows of the built-in table are in
another order in the two trees, and a copy that took the order of the
branch would have taken the names of the other rows with it. Measured on
the box: `ControlGetText("Edit1", "the-probe-of-the-controls")` answers
"hello from the port".

**What the port has moved so far: the surface goes from 684 answers with
a value and 149 refusals to 761 and 72.** The branch answers 815 and
refuses 18, so **54 names are still to come**: the COM objects (9), the
registry (5), the tray (3), the groups (4), the sound devices (2) and the
small ones (the lock lights, the cursor, the trash, the IPs, the notify).

**The fourth family is ported too (2026-11-11): the edit that has the
focus.** The `Edit*` names take the *handle* of a control and read it
through the helper of the window. Measured on the box: `EditGetLine(1)`
answers 1, `EditGetCurrentCol(1)` answers 20, `EditGetLineCount(1)`
answers 1, `EditGetSelectedText(1)` answers "hello from the port" and
`CaretGetPos()` answers 1.

**The fifth family is ported too (2026-11-11): the drives of the
machine.** The fourteen `Drive*` names answer through the tools of Linux
-- `df`, `lsblk`, `eject`, `udisksctl` and the label tool of the
filesystem. Measured on the box: `DriveGetList()` answers `zram0` and
`vda`, `DriveGetSpaceFree("/")` answers 3936, `DriveGetType("/")` answers
"Fixed", `DriveGetFilesystem("/")` answers "zfs" and `DriveGetStatus("/")`
answers "Ready"; the same eight answers on the VM of the matrix, in the
X11 session, with the numbers of that machine -- the X11 desktop answers
the family by asking a desktop of the lab, because the drives are of the
machine and not of the session.

**And the round writes `docs/desktop-test-matrix.md`** -- what the tree
asks of the desktop (the listing of the toplevels, the extension of
COSMIC, the bus of the accessibility, the GTK 3 of the helper, o X11 das
sessões X), which desktop answers what, and how one VM can carry several
of them.

**The VM of the matrix (`AhkDesktopTest`, in the Unraid host) then
corrected three answers of the tree on 2026-11-11** -- section 5.4 of the
matrix and section 320 of the notes: the question of the focus no longer
hides behind the listing (sway answers the reason of the compositor, and
not 0), the client list of the X server is parsed (the whole family of
the windows answers on X11), and the place and the state of an X11
window come from `xwininfo` and `xprop`. The surface does not move with
them: the names were already answered; what changed is the answer of a
real desktop, which only a second desktop could measure.

**The third family is ported too (2026-11-11): the windows of the
machine.** The `Win*` names ask the compositor of the session, and the
two clients of the compositor had diverged -- the one of the tree of the
owner learned to name the windows with a number, the one of the branch
learned where a window is and the scale of an output -- so the merge of
the two is by hand (`audit/port-wayland-merge.py`). Measured on the box:
`WinGetCount()` answers 3, `WinGetList()` answers the ids 1 2 3, and
`WinWait("zzz-nothing-zzz", "", 1)` answers 0 in 1012 ms.

**And the port does not take a name away from the tree of the owner to
give it back in another shape**: `WinGetID`, `WinGetList`, `WinGetCount`
and `WinGetClass` keep the handler that the tree had, with the ids as
whole numbers -- the measurement of the binary of v2.0.19 -- and the port
gives the names that the tree refused.

The branch's own `audit/PLAN.md` says the gap matrix is 140/140 covered and
that the tree answers 759 of 833 names; that is the number to beat.

### 5.3 Make `master` the branch's tree (the reverse)

This is what the `ours` merge already did once, and it costs the 41
unpushed commits of `master` and the whole differential harness. It is the
wrong direction: `master` is the line the owner works on, and the branch's
plan would have to be rewritten.

---

## 6. What was done on 2026-11-10, and what was left

The session that owned the branch stopped at 22:02 UTC (19:02 local) while
it was **resolving a content merge of `master` into the worktree**: the log
shows it reading `crates/ahk/src/desktop.rs` with `<<<<<<< HEAD` /
`=======` / `>>>>>>> master` markers in it and editing the markers away.

It left **four files uncommitted** -- `crates/ahk/src/classes/menu.rs`,
`crates/ahk/src/classes/mod.rs`, `crates/ahk/src/runtime.rs` and
`crates/ahk/tests/builtins.rs`, 105 insertions -- the callback of a menu
item: the bar is kept on the window under a reserved key and the callback is
called with `(ItemName, ItemPos, MyMenu)`, the shape documented on v2.0.19 on
2026-11-10.

That work was committed on the branch as `91c23c2` ("Call the callback of a
menu item when the helper sends the event"), so that it is not lost. The
suite of the branch is green with it.

The four files are in the tree of the branch and nowhere else. **They are the
one thing that was at risk of being lost**, and they are safe now.

---

## 7. The rules this record follows

- The tree of the owner is not reset. A branch that another session is
  working in is not touched except to commit what is already in its working
  tree.
- Every number here is measured, and the command that measured it is named.
- The two branches are named as they are, and the difference between "the
  history contains master" and "the tree has master's content" is the whole
  point of section 2.

