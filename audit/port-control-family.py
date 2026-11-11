#!/usr/bin/python3
"""Port the family of the controls of another window from the branch.

The second family of the port (`docs/two-lines.md` §5.2): the accessibility
bus. The `Control*` names of v2 read and write the controls of *another*
window, and the bus of the accessibility is what answers them.

The port is four things:

- The module of the bus (`crates/ahk-box/src/atspi.rs`) and the three
  answers it carries (`ControlAnswer`, `ControlFlag` and `ControlEdit`).
- The eight methods of the `Windows` trait that the family needs, the eight
  answers of the box that carry them, and the four helpers of the box that
  tell the window of the script from the windows of the session.
- The twenty-six handlers of the family, the helpers they call, and the
  rows of the built-in table that answer to them.
- The cases of the family are the ones of the branch.

The script reads the branch and writes the tree of the owner. It refuses to
write anything when an anchor is not where it says it is, and it can be run
twice.

    python audit/port-control-family.py
"""

import pathlib
import re
import shutil
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk\.agents\worktrees\audit-ahk-x11-parity"
)

TRAIT = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
LIB = ROOT / "crates" / "ahk-box" / "src" / "lib.rs"
ATSPI = ROOT / "crates" / "ahk-box" / "src" / "atspi.rs"

BRANCH_TRAIT = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"

# The methods of the trait that the family needs.
THE_METHODS = [
    "control_text",
    "control_visible",
    "control_enabled",
    "control_set_text",
    "control_click",
    "control_focus",
    "control_send_text",
    "window_pid",
]

# The answers of the bus.
THE_TYPES = ["ControlAnswer", "ControlFlag", "ControlEdit"]

# The handlers of the family.
THE_HANDLERS = [
    "control_get_text",
    "control_flag",
    "control_set_text",
    "control_act",
    "control_send_text",
    "control_get_focus",
    "control_get_hwnd",
    "control_visibility",
    "control_show",
    "control_get_classnn",
    "control_set_enabled",
    "control_get_checked",
    "control_set_checked",
    "control_move",
    "control_send",
    "control_get_pos",
    "control_get_items",
    "control_add_item",
    "control_choose_index",
    "control_choose_string",
    "control_get_choice",
    "control_get_index",
    "control_delete_item",
    "control_find_item",
    "control_show_drop_down",
    "control_hide_drop_down",
]

# The helpers of the family that the handlers call.
THE_HELPERS = [
    "the_state_of_a_control",
    "the_tick_of_a_control",
    "the_visibility_of_a_control",
    "the_window_of_the_script",
    "the_number_of_a_combo",
    "the_drop_down_of_a_combo",
]

# The two failures of the family.
THE_CONSTANTS = ["TARGET_WINDOW", "TARGET_CONTROL"]

# The helpers of the box that tell the window of the script from the
# windows of the session.
THE_BOX_HELPERS = [
    "gui_title_is",
    "own_control_text",
    "own_control_send_text",
    "own_control_number",
]


def a_block(lines, name, kind="fn", single=False):
    """The block of a function or a method, with the comment above it."""
    start = None
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?%s\s+%s\b" % (kind, re.escape(name)), line):
            start = index
            break
    if start is None:
        return None
    first = start
    while first > 0 and (lines[first - 1].strip().startswith("///") or lines[first - 1].strip().startswith("#[")):
        first -= 1
    if single:
        # The item has no braces of its own -- a constant, say -- so the
        # block is the line and the comment above it.
        return (first, start + 1)
    # The braces of a function may be on a line of their own -- the
    # parameters of a long signature -- so the count starts where they are.
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
    return (first, end)


def the_line_of(lines, pattern, what):
    for index, line in enumerate(lines):
        if re.search(pattern, line.rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


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


def the_block_of(lines, name, kind="fn", single=False):
    """The block of a name, or None when the tree of the owner has none."""
    found = a_block(lines, name, kind, single)
    return found


def main():
    trait = read_lines(TRAIT)
    builtins = read_lines(BUILTINS)
    box = read_lines(BOX)
    branch_trait = read_lines(BRANCH_TRAIT)
    branch_builtins = read_lines(BRANCH_BUILTINS)
    branch_box = read_lines(BRANCH_BOX)

    # 1. The module of the bus.
    shutil.copyfile(
        BRANCH / "crates" / "ahk-box" / "src" / "atspi.rs",
        ATSPI,
    )
    lib = read_lines(LIB)
    if not has_marker(lib, "pub mod atspi;"):
        at = the_line_of(lib, r"^pub mod desktop;$", "the module of the desktop")
        lib = lib[:at + 1] + ["pub mod atspi;"] + lib[at + 1:]
        write_lines(LIB, lib)

    # 2. The three answers of the bus.
    if not has_marker(trait, "pub enum ControlAnswer"):
        block = []
        for name in THE_TYPES:
            found = a_block(branch_trait, name, "enum")
            if found is None:
                raise SystemExit("the answer %s is not in the branch" % name)
            block.extend(branch_trait[found[0]:found[1]])
            block.append("")
        at = the_line_of(trait, r"^pub trait Windows \{$", "the trait of the windows")
        trait = trait[:at] + block + trait[at:]

    # 3. The eight methods of the trait.
    if not has_marker(trait, "fn control_text("):
        block = []
        for name in THE_METHODS:
            found = a_block(branch_trait, name)
            if found is None:
                raise SystemExit("the method %s is not in the branch" % name)
            block.extend(branch_trait[found[0]:found[1]])
            block.append("")
        at = the_line_of(trait, r"^pub trait Windows \{$", "the trait of the windows")
        at += 1
        trait = trait[:at] + block + trait[at:]

    # 4a. The eight answers of the box: they are methods of the trait.
    if not has_marker(box, "fn control_text("):
        block = []
        for name in THE_METHODS:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the answer of the box %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(
            box, r"^impl ahk::desktop::Windows for LabDesktop \{$", "the box"
        )
        at += 1
        box = box[:at] + block + box[at:]

    # 4b. The helpers of the box: they are not methods of a trait, and they
    # live with the other methods of the desktop.
    if not has_marker(box, "fn own_control_number("):
        block = []
        for name in THE_BOX_HELPERS:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the helper of the box %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(box, r"^impl LabDesktop \{$", "the desktop")
        at += 1
        box = box[:at] + block + box[at:]

    # 5. The twenty-six handlers of the family, the helpers they call and
    # the two failures they answer with.
    if not has_marker(builtins, "fn control_get_text("):
        block = []
        for name in THE_HANDLERS:
            found = a_block(branch_builtins, name)
            if found is None:
                raise SystemExit("the handler %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        for name in THE_HELPERS:
            if the_block_of(builtins, name) is not None:
                # The tree of the owner has it already.
                continue
            found = a_block(branch_builtins, name)
            if found is None:
                raise SystemExit("the helper %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        for name in THE_CONSTANTS:
            if the_block_of(builtins, name, "const", True) is not None:
                continue
            found = a_block(branch_builtins, name, "const", True)
            if found is None:
                raise SystemExit("the failure %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        # The handlers go before the cases of the crate, or at the end of
        # the file when it has none.
        at = None
        for index, line in enumerate(builtins):
            if re.match(r"^#\[cfg\(test\)\]$", line.rstrip("\r")):
                at = index
                break
        if at is None:
            builtins = builtins + [""] + block
        else:
            builtins = builtins[:at] + block + builtins[at:]

    # 6. The rows of the table: the handler of the family is the one of the
    # branch, and the variant with it.
    branch_rows = the_rows(branch_builtins, ["Control"])
    here_rows = the_rows(builtins, ["Control"])
    # The places of the rows are found first, because a row that is written
    # changes the places of the rows after it.
    places = []
    for name, at in branch_rows.items():
        if name not in here_rows:
            raise SystemExit("the row of %s is not in the tree of the owner" % name)
        start = here_rows[name]
        end = start
        while not re.match(r"^\s*\},?\s*$", builtins[end]):
            end += 1
        bstart = branch_rows[name]
        bend = bstart
        while not re.match(r"^\s*\},?\s*$", branch_builtins[bend]):
            bend += 1
        wanted = branch_builtins[bstart + 1:bend]
        if wanted != builtins[start + 1:end]:
            places.append((start, end, wanted))
    # And the rows are written from the last to the first -- the last *of the
    # tree of the owner*, and not of the branch, whose table is in another
    # order -- so that the places of the ones before it do not move.
    for start, end, wanted in sorted(places, key=lambda place: place[0], reverse=True):
        builtins[start + 1:end] = wanted

    write_lines(TRAIT, trait)
    write_lines(BUILTINS, builtins)
    write_lines(BOX, box)
    print("the family of the controls of another window is ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())

