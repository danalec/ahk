#!/usr/bin/python3
"""Write the note of the family of the focused edit into the notes of the tree.

The note is a section of `AGENTS.md`, and the text of a file of the tree is
edited by a script under `audit/` and not by a command of the shell. The
script refuses to write the section twice.

    python audit/append-edit-note.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
NOTES = ROOT / "AGENTS.md"

THE_NOTE = """
## 319. The edit that has the focus is read of the window of the script

**The `Edit*` names take the handle of a control.** Measured on the binary
of v2.0.19 by the branch: `EditGetCurrentLine` and its kin want one
argument at least, and that argument is the handle of the control --
`e.Hwnd` of a `Gui.Edit` is a number, and it is the number that the
helper of the window knows the control by. A call without it fails with
"Too few parameters passed to function." and the name of the name.

**And the answer comes through the helper of the window**, not through the
bus of the accessibility: the operations are `cursor` (the place of the
cursor in the text), `value` (the text itself), `lines` (the number of
the lines), `selection` (the text that is selected) and `caret` (the
place of the caret of the focused control).

Measured on the box on 2026-11-11, with this tree built there, and a
window of the script with one `Edit` that holds "hello from the port":

- `EditGetCurrentLine(1)` answers **1**,
- `EditGetCurrentCol(1)` answers **20** -- the cursor is at the end of the
  nineteen characters of the text,
- `EditGetLineCount(1)` answers **1**,
- `EditGetSelectedText(1)` answers **"hello from the port"**, and
- `CaretGetPos()` answers **1**.

**And the surface moves with the family: 747 answers with a value and 86
are refused become 754 and 79.** The command `Edit` -- the one that opens
a file in an editor -- stays refused in this tree and in the branch, and
for the same reason: the script of this runtime is read from the standard
input, so it has no file for an editor to open.
"""


def main():
    notes = read_lines(NOTES)
    if any("## 319." in line for line in notes):
        print("the note of the edit is already there")
        return 0
    notes.extend(THE_NOTE.split("\n"))
    write_lines(NOTES, notes)
    print("the note of the edit is written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
