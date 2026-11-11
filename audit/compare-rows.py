#!/usr/bin/python3
"""Compare the rows of the built-in table of two trees, name by name.

The port of a family is a change of the handler of a row -- and the shape
of the row may have moved too. This prints the rows of the names of a
family in both trees, so that the difference is seen before it is written.

    python audit/compare-rows.py Control Edit
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines  # noqa: E402

BRANCH = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk\.agents\worktrees\audit-ahk-x11-parity"
)
HERE = pathlib.Path(__file__).resolve().parent.parent


def the_rows(lines, prefixes):
    """The rows of the table whose name starts with one of the prefixes."""
    rows = {}
    current = None
    for line in lines:
        match = re.match(r"\s*name: \"([A-Za-z0-9_]+)\",\s*$", line)
        if match:
            current = match.group(1)
            rows[current] = []
            continue
        if current is None:
            continue
        if re.match(r"\s*\},?\s*$", line):
            current = None
            continue
        rows[current].append(line.strip())
    return {
        name: body
        for name, body in rows.items()
        if any(name.startswith(prefix) for prefix in prefixes)
    }


def main():
    prefixes = sys.argv[1:] or ["Control"]
    branch = the_rows(read_lines(BRANCH / "crates/ahk/src/builtins.rs"), prefixes)
    here = the_rows(read_lines(HERE / "crates/ahk/src/builtins.rs"), prefixes)
    for name in sorted(branch):
        if name not in here:
            print("%s: NOT IN THE TREE OF THE OWNER" % name)
            continue
        if branch[name] != here[name]:
            print("%s:" % name)
            print("  branch: %s" % " ".join(branch[name]))
            print("  here:   %s" % " ".join(here[name]))
    print("the names: %d in the branch, %d in the tree of the owner" % (len(branch), len(here)))


if __name__ == "__main__":
    main()
