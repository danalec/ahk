#!/usr/bin/python3
"""Port the window of the script, and the menu of it, from the branch.

The first family of the port (`docs/two-lines.md` §5.2): the window itself.
The branch `audit-ahk-x11-parity` carries a window of the script -- a GTK3
window drawn by the helper of the box -- and a menu that hangs on it. The
tree of the owner has the classes of the windows, and every name of them
answers with the reason it cannot work here.

The script reads the branch and writes the tree of the owner. It refuses to
write anything when an anchor is not where it says it is, and it can be run
twice: the copies of a method that a second run leaves are dropped.

    python egm-review/port-gui-class.py
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

CLASSES = ROOT / "crates" / "ahk" / "src" / "classes" / "mod.rs"
RUNTIME = ROOT / "crates" / "ahk" / "src" / "runtime.rs"
BRANCH_CLASSES = BRANCH / "crates" / "ahk" / "src" / "classes" / "mod.rs"
BRANCH_RUNTIME = BRANCH / "crates" / "ahk" / "src" / "runtime.rs"

# The helpers of the classes that the window needs.
THE_HELPERS = [
    "the_events_of_a_control",
    "the_control_of_the_gui",
    "the_control_is_a_tab",
    "the_control_has_an_integer_value",
    "the_control_is_a_combo",
    "the_flag_of",
    "remember_the_control",
    "the_number_of_a_control",
    "the_title_of_the_gui",
]

# The hooks of the runtime that the window needs.
THE_HOOKS = ["remember_a_gui", "the_guis"]

# The dispatchers of the classes that carry the arms of the window.
THE_DISPATCHERS = ["property", "set_property"]

# The arm that the owner of the tree has, and the one of the branch, for
# each dispatcher of the classes.
THE_ARMS = [
    (
        [
            "        \"gui\" | \"menu\" | \"gui.control\" | \"gui.listview\" | \"gui.menu\" | \"gui.statusbar\" => {",
            "            Some(gui::construct(class, arguments, rt, line))",
            "        }",
        ],
        [
            "        \"menu\" | \"menubar\" => {",
            "            // Measured on the binary of v2.0.19 on 2026-11-10: `Type` of a",
            "            // MenuBar answers \"MenuBar\", so the spelling the script used is",
            "            // the class of the object.",
            "            let spelling = class.to_string();",
            "            let made = match menu::construct(arguments, rt, line) {",
            "                Ok(made) => made,",
            "                Err(failure) => return Some(Err(failure)),",
            "            };",
            "            if let Value::Object(object) = &made {",
            "                object.borrow_mut().class = spelling;",
            "            }",
            "            Some(Ok(made))",
            "        }",
            "        \"gui\" | \"gui.control\" | \"gui.listview\" | \"gui.treeview\" | \"gui.statusbar\" => {",
            "            Some(gui::construct(class, arguments, rt, line))",
            "        }",
        ],
    ),
    (
        [
            "        \"gui\" | \"menu\" | \"gui.control\" | \"gui.listview\" | \"gui.menu\" | \"gui.statusbar\" => {",
            "            gui::call(class, method, this, arguments, rt, line)",
            "        }",
        ],
        [
            "        \"menu\" | \"menubar\" => menu::call(method, this, arguments, rt, line),",
            "        \"gui\" | \"gui.control\" | \"gui.listview\" | \"gui.treeview\" | \"gui.statusbar\" => {",
            "            gui::call(class, method, this, arguments, rt, line)",
            "        }",
        ],
    ),
    (
        [
            "        \"gui\" | \"menu\" | \"gui.control\" | \"gui.listview\" | \"gui.menu\" | \"gui.statusbar\" => {",
            "            gui::property(class, name, this, rt, line)",
            "        }",
        ],
        [
            "        \"menu\" | \"menubar\" => menu::property(name, this, rt, line),",
            "        \"gui\" | \"gui.control\" | \"gui.listview\" | \"gui.treeview\" | \"gui.statusbar\" => {",
            "            gui::property(class, name, this, rt, line)",
            "        }",
        ],
    ),
    (
        [
            "        \"gui\" | \"menu\" | \"gui.control\" | \"gui.listview\" | \"gui.menu\" | \"gui.statusbar\" => {",
            "            gui::set_property(class, name, this, value, rt, line)",
            "        }",
        ],
        [
            "        \"menu\" | \"menubar\" => menu::set_property(name, this, &value, rt, line),",
            "        \"gui\" | \"gui.control\" | \"gui.listview\" | \"gui.treeview\" | \"gui.statusbar\" => {",
            "            gui::set_property(class, name, this, value, rt, line)",
            "        }",
        ],
    ),
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


def the_arm_of(lines, arm):
    """The place of an arm of a dispatcher, and the line after it."""
    for index in range(len(lines) - len(arm) + 1):
        if all(
            lines[index + offset].rstrip("\r") == arm[offset].rstrip("\r")
            for offset in range(len(arm))
        ):
            return index
    return None


def main():
    classes = read_lines(CLASSES)
    runtime = read_lines(RUNTIME)
    branch_classes = read_lines(BRANCH_CLASSES)
    branch_runtime = read_lines(BRANCH_RUNTIME)

    dropped = repair(classes, THE_HELPERS) + repair(runtime, THE_HOOKS)
    if dropped:
        print("dropped %d copies of a method" % dropped)

    # 1. The class of the window and the class of the menu.
    shutil.copyfile(
        BRANCH / "crates" / "ahk" / "src" / "classes" / "gui.rs",
        ROOT / "crates" / "ahk" / "src" / "classes" / "gui.rs",
    )
    shutil.copyfile(
        BRANCH / "crates" / "ahk" / "src" / "classes" / "menu.rs",
        ROOT / "crates" / "ahk" / "src" / "classes" / "menu.rs",
    )
    # The name of the item by default of a menu is the tray's, and the tray
    # is not ported yet: the helper would be a function that nothing calls.
    menu = read_lines(ROOT / "crates" / "ahk" / "src" / "classes" / "menu.rs")
    found = a_block(menu, "the_default_of")
    if found is not None:
        del menu[found[0]:found[1]]
        write_lines(ROOT / "crates" / "ahk" / "src" / "classes" / "menu.rs", menu)
    if not has_marker(classes, "pub(crate) mod menu;"):
        at = the_line_of(classes, r"^pub\(crate\) mod gui;$", "the module of the window")
        classes = insert(classes, at + 1, ["pub(crate) mod menu;"])

    # 2. The arms of the window: they go at the head of the dispatcher of
    # the classes, where the branch has them.
    if not has_marker(classes, "// A window of the script is remembered"):
        first = the_line_of(
            branch_classes,
            r"// A window of the script is remembered",
            "the arms of the window",
        )
        last = the_line_of(
            branch_classes,
            r"^    let answer = call_once\(class, method, this, arguments, rt, line\);$",
            "the end of the arms of the window",
        )
        arms = branch_classes[first:last]
        opened = the_line_of(
            classes,
            r"^pub\(crate\) fn call\($",
            "the dispatcher of the classes",
        )
        at = opened
        while not classes[at].strip().startswith(") -> Option<Result<Value, RuntimeFailure>> {"):
            at += 1
        classes = insert(classes, at + 1, [""] + arms)

    # 3. The helpers of the classes.
    if not has_marker(classes, "// ---- the_events_of_a_control"):
        block = []
        for name in THE_HELPERS:
            found = a_block(branch_classes, name)
            if found is None:
                raise SystemExit("the helper %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_classes[found[0]:found[1]])
            block.append("")
        classes.extend([""])
        classes.extend(block)

    # 3b. The arms of the properties: they go at the head of the two
    # dispatchers of the classes, where the branch has them.
    for name in THE_DISPATCHERS:
        if has_marker(classes, "// ---- the arms of %s" % name):
            continue
        opened = the_line_of(
            branch_classes,
            r"^pub\(crate\) fn %s\($" % name,
            "the dispatcher %s" % name,
        )
        at = opened
        while not branch_classes[at].strip().startswith(") -> Option<Result<"):
            at += 1
        first = at + 1
        last = first
        while "match class.to_lowercase().as_str() {" not in branch_classes[last]:
            last += 1
        arms = branch_classes[first:last]
        opened = the_line_of(
            classes,
            r"^pub\(crate\) fn %s\($" % name,
            "the dispatcher %s" % name,
        )
        at = opened
        while not classes[at].strip().startswith(") -> Option<Result<"):
            at += 1
        classes = insert(classes, at + 1, ["// ---- the arms of %s" % name] + arms)

    # 3c. The dispatch of the classes: the menu is a class of its own, and
    # the window is another one.
    if not has_marker(classes, "// ---- the dispatch of the menu"):
        for number, (old, new) in enumerate(THE_ARMS):
            at = the_arm_of(classes, old)
            if at is None:
                raise SystemExit("the arm of the dispatch is not where it should be")
            if number == 0:
                new = ["// ---- the dispatch of the menu"] + new
            classes[at:at + len(old)] = new

    # 4. The hooks of the runtime.
    if not has_marker(runtime, "pub fn remember_a_gui("):
        block = []
        for name in THE_HOOKS:
            found = a_block(branch_runtime, name)
            if found is None:
                raise SystemExit("the hook %s is not in the branch" % name)
            block.extend(branch_runtime[found[0]:found[1]])
            block.append("")
        opened = the_line_of(runtime, r"^impl<'a> Runtime<'a> \{$", "the runtime")
        closed = the_closing_brace(runtime, opened)
        runtime = insert(runtime, closed, block)

    # 5. The fields of the runtime: the windows of the script and the
    # identity of the objects of it.
    if not has_marker(runtime, "guis: Vec<Value>,"):
        opened = the_line_of(runtime, r"^pub struct Runtime<'a> \{$", "the runtime")
        closed = the_closing_brace(runtime, opened)
        block = []
        for name in ["guis", "pointers"]:
            at = the_line_of(
                branch_runtime,
                r"^\s+%s: " % name,
                "the field %s" % name,
            )
            first = at
            while first > 0 and branch_runtime[first - 1].strip().startswith("///"):
                first -= 1
            block.extend(branch_runtime[first:at + 1])
        runtime = insert(runtime, closed, block)
    if not has_marker(runtime, "            guis: Vec::new(),"):
        # Every constructor of the runtime makes them.
        at = 0
        while True:
            found = None
            for index in range(at, len(runtime)):
                if re.match(r"^\s+exit_callbacks: Vec::new\(\),\s*$", runtime[index].rstrip("\r")):
                    found = index
                    break
            if found is None:
                break
            runtime = insert(runtime, found + 1, [
                "            guis: Vec::new(),",
                "            pointers: Vec::new(),",
            ])
            at = found + 3

    # 6. The identity of an object: the handle a script hands back.
    if not has_marker(runtime, "pub fn the_pointer_of("):
        found = a_block(branch_runtime, "the_pointer_of")
        if found is None:
            raise SystemExit("the identity of an object is not in the branch")
        opened = the_line_of(runtime, r"^impl<'a> Runtime<'a> \{$", "the runtime")
        closed = the_closing_brace(runtime, opened)
        runtime = insert(runtime, closed, [""] + branch_runtime[found[0]:found[1]])

    write_lines(CLASSES, classes)
    write_lines(RUNTIME, runtime)
    print("the class of the window and the class of the menu are ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())

