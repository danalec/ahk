#!/usr/bin/python3
"""Pull the arms of the window out of the branch's class dispatcher.

The branch carries the methods of a `Gui` inside the dispatcher of the
classes (`crates/ahk/src/classes/mod.rs`), and the tree of the owner has
the same dispatcher without them. The script writes the block where the
editor can read it.
"""

import pathlib
import re

BRANCH = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk\.agents\worktrees\audit-ahk-x11-parity"
)
OUT = pathlib.Path(r"C:\Temp\gui-arms.txt")

MOD = BRANCH / "crates/ahk/src/classes/mod.rs"
RUNTIME = BRANCH / "crates/ahk/src/runtime.rs"


def main():
    lines = MOD.read_text(encoding="utf-8").split("\n")
    # The arms of the window: from the line that remembers the window to
    # the line before the menu arm.
    start = None
    end = None
    for index, line in enumerate(lines):
        if "rt.remember_a_gui(this)" in line:
            if start is None:
                start = index
        if start is not None and '"gui" | "gui.control"' in line and end is None:
            end = index
    text = [
        "=== the arms of the window: lines %d-%d of the branch ===" % (start, end),
    ]
    text.extend(lines[start:end])
    OUT.write_text("\n".join(text), encoding="utf-8", newline="\n")
    print("written", OUT)

    # The hooks of the runtime.
    runtime = RUNTIME.read_text(encoding="utf-8").split("\n")
    text = []
    for name in ["remember_a_gui", "the_events_are_answered", "the_guis"]:
        for index, line in enumerate(runtime):
            if re.match(r"\s*pub fn %s\b" % name, line):
                depth = 0
                begun = False
                for at in range(index, len(runtime)):
                    depth += runtime[at].count("{") - runtime[at].count("}")
                    if "{" in runtime[at]:
                        begun = True
                    if begun and depth <= 0:
                        first = index
                        while first > 0 and runtime[first - 1].strip().startswith("///"):
                            first -= 1
                        text.append("=== %s: lines %d-%d ===" % (name, first, at + 1))
                        text.extend(runtime[first:at + 1])
                        text.append("")
                        break
                break
    (OUT.parent / "gui-hooks.txt").write_text("\n".join(text), encoding="utf-8", newline="\n")
    print("written", OUT.parent / "gui-hooks.txt")


if __name__ == "__main__":
    main()
