#!/usr/bin/python3
"""Move the helpers of the window before the cases of the crate.

The port put the helpers of the machine at the end of the file of the box,
and the file ends with its cases -- so the helpers were after the cases,
and the compiler says so. This moves them back.

    python egm-review/move-helpers.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"


def main():
    box = read_lines(BOX)
    first = None
    for index, line in enumerate(box):
        if line.strip() == "// ---- the_helper_of_the_window":
            first = index
            break
    if first is None:
        print("the helpers are not there")
        return 0
    # The helpers go up to the end of the file: the block is the lines
    # between the first marker and the end.
    helpers = box[first:]
    box = box[:first]
    # The cases of the crate are the last block of the file.
    at = None
    for index, line in enumerate(box):
        if line.strip() == "#[cfg(test)]":
            at = index
            break
    if at is None:
        raise SystemExit("the cases of the crate are not where they should be")
    if first < at:
        # The helpers are already before the cases: nothing to move.
        print("the helpers of the window are already before the cases")
        return 0
    # The helpers go before the cases, and the blank line that separated
    # them goes with them.
    while helpers and helpers[-1].strip() == "":
        helpers.pop()
    box = box[:at] + helpers + [""] + box[at:]
    write_lines(BOX, box)
    print("the helpers of the window are before the cases")
    return 0


if __name__ == "__main__":
    sys.exit(main())

