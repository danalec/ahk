#!/usr/bin/python3
"""Port the cases of the family of the controls of another window.

The cases of the branch are the record of what the family does, and the
tree of the owner needs them: a family that is ported without its cases is
a family that nothing holds.

    python audit/port-control-cases.py
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

TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

# The answers of the fake of the suite.
THE_FAKE = [
    "control_text",
    "control_visible",
    "control_enabled",
    "control_set_text",
    "control_click",
    "control_focus",
    "control_send_text",
]

# The helpers of the fake: they are not methods of a trait.
THE_FAKE_HELPERS = ["the_edit", "the_flag"]

# The functions of the suite that the cases of the family call.
THE_FAKE_FUNCTIONS = ["a_fake_with_a_combo"]

# The answers of the fake for the window of the script: the cases of the
# family ask them.
THE_FAKE_WINDOW = [
    "gui_caret",
    "gui_combo_add",
    "gui_combo_choice",
    "gui_combo_delete",
    "gui_combo_dropdown",
    "gui_combo_find",
    "gui_combo_items",
    "gui_combo_select",
    "gui_control_pos",
    "gui_list_add",
    "gui_list_content",
    "gui_list_count",
    "gui_list_modify",
    "gui_list_next",
    "gui_list_text",
    "gui_number_named",
    "gui_option",
    "gui_second_value",
    "gui_status_text",
    "gui_title",
    "gui_tree_add",
    "gui_tree_count",
    "gui_tree_delete",
    "gui_tree_modify",
    "gui_tree_relation",
    "gui_tree_selection",
    "gui_tree_text",
    "gui_value",
]

# The cases of the family.
THE_CASES = [
    "control_get_items_answers_an_array_of_the_items",
    "control_add_item_makes_the_array_one_longer",
    "control_choose_index_and_string_set_the_choice",
    "control_find_item_and_delete_item",
    "a_control_get_pos_writes_the_four_places_of_a_control",
]


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
    while first > 0 and (lines[first - 1].strip().startswith("///") or lines[first - 1].strip().startswith("#[")):
        first -= 1
    return (first, end)


def the_line_of(lines, pattern, what):
    for index, line in enumerate(lines):
        if re.search(pattern, line.rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


def has_marker(lines, marker):
    return any(marker in line for line in lines)


def main():
    tests = read_lines(TESTS)
    branch = read_lines(BRANCH_TESTS)

    # 1. The answers of the fake: they are methods of the `Windows` trait.
    if not has_marker(tests, "fn control_text("):
        block = []
        for name in THE_FAKE:
            found = a_block(branch, name)
            if found is None:
                raise SystemExit("the answer of the fake %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch[found[0]:found[1]])
            block.append("")
        at = the_line_of(tests, r"^impl Windows for Fake \{$", "the fake")
        at += 1
        tests = tests[:at] + block + tests[at:]

    # 1b. The answers of the fake for the window of the script.
    for name in THE_FAKE_WINDOW:
        if a_block(tests, name) is not None:
            continue
        found = a_block(branch, name)
        if found is None:
            raise SystemExit("the answer of the fake %s is not in the branch" % name)
        at = the_line_of(tests, r"^impl Windows for Fake \{$", "the fake")
        at += 1
        tests = tests[:at] + [
            "    // ---- %s" % name,
        ] + branch[found[0]:found[1]] + [""] + tests[at:]

    # 1c. The helpers of the fake: they live with the fake itself.
    if not has_marker(tests, "fn the_edit("):
        block = []
        for name in THE_FAKE_HELPERS:
            found = a_block(branch, name)
            if found is None:
                raise SystemExit("the helper of the fake %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch[found[0]:found[1]])
            block.append("")
        at = the_line_of(tests, r"^impl Windows for Fake \{$", "the fake")
        tests = tests[:at] + ["impl Fake {"] + block + ["}"] + [""] + tests[at:]

    # 1c. The three answers of the bus are named by the cases.
    if not has_marker(tests, "ControlAnswer, ControlEdit, ControlFlag"):
        at = the_line_of(tests, r"^    Clipboard, Desktop, Dialogs, ", "the imports")
        tests[at] = tests[at].replace(
            "Clipboard, Desktop, Dialogs, ",
            "Clipboard, ControlAnswer, ControlEdit, ControlFlag, Desktop, Dialogs, ",
        )

    # 1d. The fields of the fake that the family needs.
    at = the_line_of(tests, r"^struct Fake \{$", "the fake")
    end = at
    while not re.match(r"^\}$", tests[end].rstrip("\r")):
        end += 1
    bat = the_line_of(branch, r"^struct Fake \{$", "the fake of the branch")
    bend = bat
    while not re.match(r"^\}$", branch[bend].rstrip("\r")):
        bend += 1
    # The fields of the branch that the tree of the owner does not have.
    fields = []
    for index in range(bat + 1, bend):
        line = branch[index]
        match = re.match(r"\s+([a-z_]+):", line)
        if not match:
            continue
        name = match.group(1)
        if any(re.match(r"\s*%s:" % name, held) for held in tests):
            continue
        # The field carries its comment with it.
        first = index
        while first > 0 and branch[first - 1].strip().startswith("///"):
            first -= 1
        fields.extend(branch[first:index + 1])
    if fields:
        tests = tests[:end] + fields + tests[end:]

    # 1e. The case that said the blanket reason of the family: the family is
    # carried now, and a control without a window fails with the reason of
    # the family.
    old = [
        "#[test]",
        "fn a_name_of_windows_fails_with_the_reason() {",
        "    let report = run(\"ControlGetText(\\\"x\\\")\");",
        "    assert_eq!(",
        "        failure_of(&report),",
        "        \"this is a name of Windows, and this desktop is not Windows.\"",
        "    );",
        "}",
    ]
    new = [
        "#[test]",
        "fn a_name_of_the_control_family_fails_with_the_reason_of_the_family() {",
        "    // The blanket reason -- \"this is a name of Windows, and this desktop",
        "    // is not Windows\" -- was the one of a family that was not carried at",
        "    // all. The family of the controls of *another* window is carried since",
        "    // 2026-11-11, and a control without a window fails with the reason of",
        "    // the family: the window is found by its title, and there is none.",
        "    let report = run(\"ControlGetText(\\\"x\\\")\");",
        "    assert_eq!(",
        "        failure_of(&report),",
        "        \"the window of a control is found by its title, and an empty title is not carried yet.\"",
        "    );",
        "}",
    ]
    for index in range(len(tests) - len(old) + 1):
        if [line.rstrip("\r") for line in tests[index:index + len(old)]] == old:
            tests[index:index + len(old)] = new
            break

    # 2. The cases of the family.
    if not has_marker(tests, "fn control_get_items_answers_an_array_of_the_items("):
        block = []
        for name in THE_CASES:
            found = a_block(branch, name)
            if found is None:
                raise SystemExit("the case %s is not in the branch" % name)
            block.extend(branch[found[0]:found[1]])
            block.append("")
        for name in THE_FAKE_FUNCTIONS:
            found = a_block(branch, name)
            if found is None:
                raise SystemExit("the function %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch[found[0]:found[1]])
            block.append("")
        tests = tests + [""] + block

    write_lines(TESTS, tests)
    print("the cases of the family of the controls are ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())

