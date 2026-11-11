#!/usr/bin/python3
"""Port the reasons of the remaining refusals from the branch.

The branch's pass of 2026-11-10 over the refusals gave each group of them
a reason of its own -- the slots 24 to 33 of `refusal_message` -- instead
of the blanket claim that the name is "a name of Windows", which was
false about most of them. The port carries the blanket table only up to
the slot 23.

This script copies the missing slots and moves the four rows of the
control styles -- `ControlGetStyle`, `ControlSetStyle`,
`ControlGetExStyle` and `ControlSetExStyle` -- to the variant 127 of the
branch. The surface does not move: the four names stay refusals in both
trees. What moves is the reason the run gives.

The script is idempotent.

    python audit/port-refusal-reasons.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"

ROWS = [
    "ControlGetExStyle",
    "ControlGetStyle",
    "ControlSetExStyle",
    "ControlSetStyle",
]

FALLBACK_COMMENT = "// No name of the surface has this variant"


def main():
    builtins = read_lines(BUILTINS)
    branch = read_lines(BRANCH_BUILTINS)

    # 1. The slots 24 to 33 of the refusal table.
    if not any("24 => {" in line for line in builtins):
        start = None
        end = None
        for index, line in enumerate(branch):
            if line.strip() == "24 => {":
                start = index
            if start is not None and FALLBACK_COMMENT in line:
                end = index
                break
        if start is None or end is None:
            raise SystemExit("the slots of the branch are not delimited")
        for index, line in enumerate(builtins):
            if FALLBACK_COMMENT in line:
                builtins = builtins[:index] + branch[start:end] + builtins[index:]
                break
        else:
            raise SystemExit("the fallback comment is not in the port")

    # 2. The four rows of the control styles, to the variant of the branch.
    for name in ROWS:
        start = None
        for index, line in enumerate(builtins):
            if f'name: "{name}",' in line:
                start = index
                break
        if start is None:
            raise SystemExit(f"the row of {name} is not there")
        branch_row = None
        bstart = None
        for index, line in enumerate(branch):
            if f'name: "{name}",' in line:
                bstart = index
                break
        for index in range(bstart, bstart + 9):
            if branch[index].strip().startswith("variant:"):
                branch_row = branch[index].strip()
        for index in range(start, start + 9):
            stripped = builtins[index].strip()
            if stripped.startswith("variant:") and stripped != branch_row:
                indent = builtins[index][: len(builtins[index]) - len(builtins[index].lstrip())]
                builtins[index] = f"{indent}{branch_row}"
    write_lines(BUILTINS, builtins)

    print("the reasons of the remaining refusals are ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
