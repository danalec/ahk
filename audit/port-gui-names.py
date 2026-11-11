#!/usr/bin/python3
"""Carry the name of the menu bar of the window into the tree of the owner.

The branch has a `MenuBar` class: the menu that hangs on the top of a
window is a class of its own, and `Type` of it answers "MenuBar". The tree
of the owner knows `Menu` and not `MenuBar`, and the port of the window
needs both.

    python egm-review/port-gui-names.py
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
CLASSES = ROOT / "crates" / "ahk" / "src" / "classes" / "mod.rs"


def the_line_of(lines, pattern, what, after=0):
    for index in range(after, len(lines)):
        if re.search(pattern, lines[index].rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


def main():
    classes = read_lines(CLASSES)

    # 1. The types: the menu bar is a class of the values.
    at = the_line_of(classes, r"^pub\(crate\) const TYPES: \[&str; \d+\] = \[$", "the types")
    end = the_line_of(classes, r"^\];$", "the end of the types", at)
    if not any(
        line.strip().strip(",").strip('"') == "menubar" for line in classes[at:end]
    ):
        where = the_line_of(classes, r"^\s*\"menu\",$", "the type of the menu", at)
        classes.insert(where + 1, '    "menubar",')
        end += 1
        # The table has one more name now.
        count = len([
            line for line in classes[at:end]
            if line.strip().startswith('"') and line.strip().endswith('",')
        ])
        classes[at] = re.sub(r"\[&str; \d+\]", "[&str; %d]" % count, classes[at].rstrip("\r"))

    # 2. The spellings: the dialect spells it with two capitals.
    at = the_line_of(
        classes,
        r"^\s*\(\"inputhook\", \"InputHook\"\),$",
        "the spelling of the hook",
    )
    if not any("menubar" in line and "MenuBar" in line for line in classes):
        classes.insert(at + 1, '    ("menubar", "MenuBar"),')
        at = the_line_of(
            classes,
            r"^\s*const THE_SPELLINGS: \[\(&str, &str\); \d+\] = \[$",
            "the spellings",
        )
        end = the_line_of(classes, r"^\];$", "the end of the spellings", at)
        count = len([line for line in classes[at:end] if line.strip().startswith("(")])
        classes[at] = re.sub(
            r"\); \d+\]", "); %d]" % count, classes[at].rstrip("\r")
        )

    # 3. The names of the window: the menu bar is one of them.
    at = the_line_of(classes, r"^\s*\"gui\",$", "the names of the window")
    end = the_line_of(classes, r"^\s*\],$", "the end of the names of the window", at)
    if not any(
        line.strip().strip(",").strip('"') == "MenuBar" for line in classes[at:end]
    ):
        where = the_line_of(classes, r"^\s*\"Maximize\",$", "the name of the window", at)
        classes.insert(where + 1, '            "MenuBar",')

    write_lines(CLASSES, classes)
    print("the name of the menu bar is carried")
    return 0


if __name__ == "__main__":
    sys.exit(main())

