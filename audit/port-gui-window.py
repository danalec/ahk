#!/usr/bin/python3
"""Port the window of the script from the branch into the tree of the owner.

Two lines: the branch `audit-ahk-x11-parity` carries a window of the
script -- a GTK3 window drawn by the helper of the box -- and the tree of
the owner does not. The port is family by family (see `docs/two-lines.md`),
and this is the first family: the window itself.

The script reads the branch and writes the tree of the owner. It refuses to
write anything when an anchor is not where it says it is, and it can be run
twice: the copies of a method that a second run leaves are dropped.

    python egm-review/port-gui-window.py
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

TRAIT = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
LIB = ROOT / "crates" / "ahk-box" / "src" / "lib.rs"

BRANCH_TRAIT = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"

# The methods of the window. The five that read the controls of ANOTHER
# window belong to the accessibility bus, which is the next family.
THE_WINDOW = [
    "gui_is_open", "gui_open", "gui_add",
    "gui_combo_items", "gui_combo_add", "gui_combo_select", "gui_combo_choice",
    "gui_combo_delete", "gui_combo_find", "gui_combo_dropdown", "gui_status_text",
    "gui_list_add", "gui_list_count", "gui_list_text", "gui_list_content",
    "gui_list_modify", "gui_list_next",
    "gui_tree_add", "gui_tree_count", "gui_tree_text", "gui_tree_relation",
    "gui_tree_selection", "gui_tree_modify", "gui_tree_delete",
    "gui_second_value", "gui_control_pos", "gui_menu_bar", "gui_caret",
    "gui_show", "gui_destroy", "gui_value", "gui_flag", "gui_control_state",
    "gui_visibility", "gui_number_named", "gui_focused", "gui_selection",
    "gui_cursor", "gui_line_count", "gui_option", "gui_hide", "gui_focus",
    "gui_size", "gui_events", "gui_title", "gui_checked",
]

# The helpers of the machine that the window uses.
THE_HELPERS = [
    "the_helper_of_the_window",
    "the_events_of_the_window",
    "quoted",
    "the_number_in",
    "the_text_in",
    "box_tools_are_near",
]


def a_block(lines, name):
    """The block of a function or a method, with the comment above it."""
    start = None
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?fn\s+%s\b" % re.escape(name), line):
            start = index
            break
    if start is None:
        return None
    depth = 0
    begun = False
    end = None
    for index in range(start, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            end = index + 1
            break
    if end is None:
        return None
    first = start
    while first > 0 and lines[first - 1].strip().startswith("///"):
        first -= 1
    return (first, end)


def the_line_of(lines, pattern, what):
    for index, line in enumerate(lines):
        # The line keeps the ending it had, and the ending is not part of
        # the name of the line.
        if re.search(pattern, line.rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


def the_closing_brace(lines, opened_at):
    depth = 0
    begun = False
    for index in range(opened_at, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            return index
    return None


def insert(lines, at, block):
    return lines[:at] + block + lines[at:]


def has_marker(lines, marker):
    return any(marker in line for line in lines)


def the_duplicates(lines, name):
    blocks = []
    index = 0
    while index < len(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?fn\s+%s\b" % re.escape(name), lines[index]):
            depth = 0
            begun = False
            end = None
            for at in range(index, len(lines)):
                depth += lines[at].count("{") - lines[at].count("}")
                if "{" in lines[at]:
                    begun = True
                if begun and depth <= 0:
                    end = at + 1
                    break
            if end is None:
                break
            first = index
            while first > 0 and lines[first - 1].strip().startswith("///"):
                first -= 1
            blocks.append((first, end))
            index = end
            continue
        index += 1
    return blocks[1:]


def repair(lines, names):
    dropped = 0
    for name in names:
        for start, end in reversed(the_duplicates(lines, name)):
            del lines[start:end]
            dropped += 1
    return dropped


def main():
    trait = read_lines(TRAIT)
    box = read_lines(BOX)
    branch_trait = read_lines(BRANCH_TRAIT)
    branch_box = read_lines(BRANCH_BOX)

    dropped = repair(trait, THE_WINDOW) + repair(box, THE_WINDOW + THE_HELPERS)
    if dropped:
        print("dropped %d copies of a method" % dropped)

    # 1. The trait: the methods of the window go into the `Windows` trait.
    if not has_marker(trait, "// ---- gui_open"):
        opened = the_line_of(trait, r"^pub trait Windows \{", "the trait of the windows")
        closed = the_closing_brace(trait, opened)
        block = []
        for name in THE_WINDOW:
            found = a_block(branch_trait, name)
            if found is None:
                raise SystemExit("the method %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_trait[found[0]:found[1]])
            block.append("")
        trait = insert(trait, closed, block)

    # 2. The box: the methods that answer them go into the same trait.
    if not has_marker(box, "// ---- gui_open"):
        opened = the_line_of(
            box, r"^impl ahk::desktop::Windows for LabDesktop \{", "the box"
        )
        closed = the_closing_brace(box, opened)
        block = []
        for name in THE_WINDOW:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the method %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        box = insert(box, closed, block)

    # 3. The box: the helpers of the machine. They go before the cases of
    # the crate, so that the file ends with them.
    if not has_marker(box, "// ---- the_helper_of_the_window"):
        block = []
        for name in THE_HELPERS:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the helper %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(box, r"^#\[cfg\(test\)\]$", "the cases of the crate")
        box = insert(box, at, block + [""])

    # 4. The box: the window of the script is a field of the desktop.
    if not has_marker(box, "gui: Option<crate::gui::TheWindow>,"):
        at = the_line_of(box, r"^    pointer: Option<Pointer>,$", "the field of the pointer")
        box = insert(box, at + 1, [
            "    /// The window of the script, when the session has one.",
            "    ///",
            "    /// It is made by the helper of the window and lives as long as",
            "    /// the desktop does.",
            "    gui: Option<crate::gui::TheWindow>,",
        ])
    if not has_marker(box, "            gui: None,"):
        at = the_line_of(box, r"^            pointer: None,$", "the pointer of the desktop")
        box = insert(box, at + 1, ["            gui: None,"])

    # 5. The box: the helper that talks to the window is a method of the
    # desktop, and it lives with the other methods of it.
    if not has_marker(box, "fn the_gui_says("):
        opened = the_line_of(box, r"^impl LabDesktop \{$", "the desktop")
        closed = the_closing_brace(box, opened)
        found = a_block(branch_box, "the_gui_says")
        box = insert(box, closed, [""] + branch_box[found[0]:found[1]])

    write_lines(TRAIT, trait)
    write_lines(BOX, box)

    # 6. The module of the transport.
    lib = read_lines(LIB)
    if not has_marker(lib, "pub mod gui;"):
        at = the_line_of(lib, r"^pub mod desktop;$", "the module of the desktop")
        lib = insert(lib, at + 1, ["pub mod gui;"])
        write_lines(LIB, lib)

    print("the trait and the box are ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())

