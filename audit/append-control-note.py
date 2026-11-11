#!/usr/bin/python3
"""Write the note of the family of the controls into the notes of the tree.

The note is a section of `AGENTS.md`, and the text of a file of the tree is
edited by a script under `audit/` and not by a command of the shell. The
script refuses to write the section twice.

    python audit/append-control-note.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
NOTES = ROOT / "AGENTS.md"

THE_NOTE = """
## 317. The controls of another window are read through the bus of the accessibility

**The `Control*` names of v2 read the controls of another window.** The
tree of the owner refused every one of them with the blanket reason of the
names of Windows, and the branch carries a bus of the accessibility
(`crates/ahk-box/src/atspi.rs`) that answers them: the tree of the
accessibility is asked over D-Bus, and the `Name` of a node of it is the
**title** of the window -- which is the thing that makes
`ControlGetText(Control, WinTitle)` possible without correlating a Wayland
toplevel to a bus name.

**The window of the script is not a window of the session.** A control of
*our own* window is read through the helper of the window, not through the
bus, and the four helpers of the box that tell the two apart are
`gui_title_is`, `own_control_text`, `own_control_send_text` and
`own_control_number`.

Measured on the box on 2026-11-11, with this tree built there:

- `ControlGetText("Edit1", "the-probe-of-the-controls")` answers "hello
  from the port" -- the text of a control of a window, read through the bus
  of the accessibility.
- The surface moves with the family: **712 answers with a value and 121
  are refused**, where the tree of the owner had 684 and 149.

**And the port is a script** (`audit/port-control-family.py` and
`audit/port-control-cases.py`), like the window itself: the rows of the
built-in table are in another order in the two trees, and a copy that took
the order of the branch would have taken the names of the other rows with
it.
"""


def main():
    notes = read_lines(NOTES)
    if any("## 317." in line for line in notes):
        print("the note of the controls is already there")
        return 0
    notes.extend(THE_NOTE.split("\n"))
    write_lines(NOTES, notes)
    print("the note of the controls is written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
