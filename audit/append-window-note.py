#!/usr/bin/python3
"""Write the note of the port of the window into the notes of the tree.

The note is a section of `AGENTS.md`, and the text of a file of the tree is
edited by a script under `egm-review/` and not by a command of the shell.
The script refuses to write the section twice.

    python egm-review/append-window-note.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
NOTES = ROOT / "AGENTS.md"

THE_NOTE = """
## 316. The window of the script is drawn by the helper of the box

**The window is the desktop's, and the desktop of the box has one.** The
helper `crates/ahk-box/helper/ahk-gui.py` draws a GTK3 window when it is
told to over a pipe, and the transport
(`crates/ahk-box/src/gui.rs`) is the pipe. The runtime asks for it
through the `Windows` trait -- the fifty methods of it that the window
needs -- and the class of the window (`crates/ahk/src/classes/gui.rs`)
answers the names of v2 with them.

**And the menu that hangs on it is a class of its own** --
`crates/ahk/src/classes/menu.rs` -- with the `MenuBar` spelling the
dialect uses: `Type` of a `MenuBar` answers "MenuBar", and the menu is
put on the window with `g.Menu := mb`.

**The events are answered when the script waits.** The helper writes what
happened to a file the runtime polls, and the drain reads it: the handler
that the window kept is called **by its value**, because a closure has no
name in v2 and only the value of it can be called. `Sleep` is the one
statement of this runtime that waits, so it is the one place an event can
be answered without a loop of its own.

Measured on the box on 2026-11-11, with this tree built there:

- The window opens: `Gui()`, `Add("Text")`, `Add("Button")`, `Title :=`,
  `Show()` and `Destroy()` all answer, and the report carries the
  controls with their `ClassNN` (`Static1`, `Button1`) and their numbers.
- The event reaches the handler: the event written into the file the
  runtime polls is read by the `Sleep` that follows, and the handler of
  the control is called with the control as its argument -- the output
  is "[pressed [Gui.Button]]".

**And the port of it is a script and not a copy by hand.** The two trees
have drifted -- the branch has the window, and the tree of the owner has
the KeySharp dialect -- and a copy would have taken the drift with it.
`egm-review/port-gui-window.py`, `egm-review/port-gui-class.py` and
`egm-review/port-gui-names.py` read the branch and write the tree of the
owner; they refuse to write anything when an anchor is not where it says
it is, and they can be run twice.
"""


def main():
    notes = read_lines(NOTES)
    if any("## 316." in line for line in notes):
        print("the note of the window is already there")
        return 0
    notes.extend(THE_NOTE.split("\n"))
    write_lines(NOTES, notes)
    print("the note of the window is written")
    return 0


if __name__ == "__main__":
    sys.exit(main())

