#!/usr/bin/python3
"""Port the family of the focused edit from the branch.

The fourth family of the port (`docs/two-lines.md` §5.2): the edit that
has the focus. The `Edit*` names of v2 read the caret, the line and the
selection of the focused edit of the window of the script, and the tree of
the owner refused every one of them with the blanket reason of the names
of Windows.

The tree of the owner carries the capability already -- the methods of the
`Screen` trait that answer these questions are there, and the helper of the
window has the operations of them -- so the port is the seven handlers and
the rows of the table that answer to them.

    python audit/port-edit-family.py
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk\.agents\worktrees\audit-ahk-x11-parity"
)

BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

# The handlers of the family.
THE_HANDLERS = [
    "caret_get_pos",
    "edit_get_line_count",
    "edit_get_line",
    "edit_get_current_line",
    "edit_get_current_col",
    "edit_get_selected_text",
    "edit_paste",
    # The command `Edit` opens a file in an editor, and this runtime reads
    # its script from the standard input.
    "edit",
]

# The names of the family in the table.
THE_NAMES = ["Edit", "CaretGetPos"]

# The cases of the family.
THE_CASES = ["caret_get_pos_answers_the_place_of_the_caret_and_zero_without_one"]


def a_block(lines, name, kind="fn"):
    start = None
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?%s\s+%s\b" % (kind, re.escape(name)), line):
            start = index
            break
    if start is None:
        return None
    opened = start
    while opened < len(lines) and "{" not in lines[opened]:
        opened += 1
    if opened >= len(lines):
        return None
    depth = 0
    begun = False
    end = None
    for index in range(opened, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            end = index + 1
            break
    if end is None:
        return None
    first = start
    while first > 0 and (
        lines[first - 1].strip().startswith("///")
        or lines[first - 1].strip().startswith("#[")
    ):
        first -= 1
    return (first, end)


def has_marker(lines, marker):
    return any(marker in line for line in lines)


def the_rows(lines, prefixes):
    """The rows of the built-in table whose name starts with a prefix."""
    rows = {}
    current = None
    for index, line in enumerate(lines):
        match = re.match(r"\s*name: \"([A-Za-z0-9_]+)\",\s*$", line)
        if match:
            current = match.group(1)
            rows[current] = index
            continue
        if current is None:
            continue
        if re.match(r"\s*\},?\s*$", line):
            current = None
    return {
        name: index
        for name, index in rows.items()
        if any(name.startswith(prefix) for prefix in prefixes)
    }


def main():
    builtins = read_lines(BUILTINS)
    tests = read_lines(TESTS)
    branch_builtins = read_lines(BRANCH_BUILTINS)
    branch_tests = read_lines(BRANCH_TESTS)

    # 1. The seven handlers of the family.
    if not has_marker(builtins, "fn edit_get_line_count("):
        block = []
        for name in THE_HANDLERS:
            found = a_block(branch_builtins, name)
            if found is None:
                raise SystemExit("the handler %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        at = None
        for index, line in enumerate(builtins):
            if re.match(r"^#\[cfg\(test\)\]$", line.rstrip("\r")):
                at = index
                break
        if at is None:
            builtins = builtins + [""] + block
        else:
            builtins = builtins[:at] + block + builtins[at:]

    # 2. The rows of the table: the handler of the family is the one of the
    # branch, and the variant with it -- but only for the names that the
    # tree of the owner refused.
    branch_rows = the_rows(branch_builtins, THE_NAMES)
    here_rows = the_rows(builtins, THE_NAMES)
    places = []
    for name, at in branch_rows.items():
        if name not in here_rows:
            raise SystemExit("the row of %s is not in the tree of the owner" % name)
        start = here_rows[name]
        end = start
        while not re.match(r"^\s*\},?\s*$", builtins[end]):
            end += 1
        if "handler: not_carried" not in "\n".join(builtins[start:end]):
            continue
        bstart = branch_rows[name]
        bend = bstart
        while not re.match(r"^\s*\},?\s*$", branch_builtins[bend]):
            bend += 1
        wanted = branch_builtins[bstart + 1:bend]
        if wanted != builtins[start + 1:end]:
            places.append((start, end, wanted))
    for start, end, wanted in sorted(places, key=lambda place: place[0], reverse=True):
        builtins[start + 1:end] = wanted

    # 3. The cases of the family.
    for name in THE_CASES:
        wanted = a_block(branch_tests, name)
        if wanted is None:
            raise SystemExit("the case %s is not in the branch" % name)
        found = a_block(tests, name)
        if found is None:
            tests = tests + [""] + branch_tests[wanted[0]:wanted[1]]
        else:
            tests[found[0]:found[1]] = branch_tests[wanted[0]:wanted[1]]

    # 4. The case of the runner reads the numbers of the surface, and they
    # move with the family.
    runner = read_lines(RUNNER)
    start = None
    for index, line in enumerate(runner):
        if 'report.number("total"), 833' in line:
            start = index
            break
    if start is None:
        raise SystemExit("the case of the runner is not where it should be")
    for index in range(start, len(runner)):
        if 'report.number("values")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("values"), 754);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 79);'
            break
    write_lines(RUNNER, runner)

    write_lines(BUILTINS, builtins)
    write_lines(TESTS, tests)
    print("the family of the focused edit is ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())

