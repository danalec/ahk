#!/usr/bin/python3
"""Write the note of the family of the windows into the notes of the tree.

The note is a section of `AGENTS.md`, and the text of a file of the tree is
edited by a script under `audit/` and not by a command of the shell. The
script refuses to write the section twice.

    python audit/append-window-note.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
NOTES = ROOT / "AGENTS.md"

THE_NOTE = """
## 318. The windows of the machine are asked of the compositor, and the place of one is not always known

**The `Win*` names of v2 ask the compositor of the session.** The tree of
the owner carried a few of them already -- the listing, the title, the
focus -- and refused the rest with the blanket reason of the names of
Windows; the family came with the port of 2026-11-11.

**The two clients of the compositor diverged, and the merge is by hand.**
The client of the tree of the owner learned to name the windows of the
session with a number (`windows_with_ids`), and the client of the branch
learned where a window is (`geometry`, the event of the version 2 of
`zcosmic_toplevel_info_v1`), in what state it is (maximized, minimized)
and how many pixels of an output are in one point of it (`scale`). The
merge is `audit/port-wayland-merge.py`: it writes the two texts that the
two trees disagree about, and nothing is copied.

Measured on the box on 2026-11-11, with this tree built there:

- `WinGetCount()` answers **3** and `WinGetList()` answers the ids **1 2
  3** -- the three windows of the session.
- Their titles are "Gmail - Chromium", "Unlock Keyring" and "Steam".
- `WinWait("zzz-nothing-zzz", "", 1)` answers **0** in **1012 ms**: the
  wait of the family stops at the second that was asked for. The third
  parameter is the timeout, and a call without it waits the cap.
- `WinGetPos(&x, &y, &w, &h, "Steam")` fails with **"the compositor did
  not say where this window is, so its place is not known"**. The geometry
  is an event of the compositor, and this compositor did not send it for
  that window. The answer is the reason and not a zero: a window at the
  origin with no size is not the same as a window whose place is unknown.

**And the surface moves with the family: 712 answers with a value and 121
are refused become 747 and 86.**
"""


def main():
    notes = read_lines(NOTES)
    if any("## 318." in line for line in notes):
        print("the note of the windows is already there")
        return 0
    notes.extend(THE_NOTE.split("\n"))
    write_lines(NOTES, notes)
    print("the note of the windows is written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
