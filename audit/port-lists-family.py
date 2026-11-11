#!/usr/bin/python3
"""Port the family of the image lists, the list controls and the status bars.

The twelfth family of the port (`docs/two-lines.md` §5.2): seven names.

- `IL_Create`, `IL_Add` and `IL_Destroy` keep lists of images in the
  runtime: the addresses are minted from `0x3000_0000`, apart from the
  ones of the objects and of the texts, and `IL_Add` loads the picture
  through the desktop before it puts it in a list.
- `ListViewGetContent` reads the content of a list control of the
  script's own window through the helper -- all of it, or the selected
  rows alone.
- `StatusBarGetText` and `StatusBarWait` read and watch the text of the
  status bar of the script's own window through the helper.
- `MenuSelect` chooses an item of the menu of the script's own window,
  and refuses what it cannot reach with the wording of v2: "Non-existent
  or unsupported menu.".

Every desktop method the family calls is in the tree already -- the
controls family carried `gui_list_content`, `gui_status_text` and
`gui_number_named` -- so the port is the runtime state of the image
lists, the seven handlers, the seven rows and the five cases.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-lists-family.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

RUNTIME = ROOT / "crates" / "ahk" / "src" / "runtime.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_RUNTIME = BRANCH / "crates" / "ahk" / "src" / "runtime.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

ROWS = [
    ("IL_Add", "il_add"),
    ("IL_Create", "il_create"),
    ("IL_Destroy", "il_destroy"),
    ("ListViewGetContent", "list_view_get_content"),
    ("MenuSelect", "menu_select"),
    ("StatusBarGetText", "status_bar_get_text"),
    ("StatusBarWait", "status_bar_wait"),
]

HANDLERS = [
    "il_create",
    "il_add",
    "il_destroy",
    "list_view_get_content",
    "menu_select",
    "status_bar_get_text",
    "status_bar_wait",
]

METHODS = [
    "make_an_image_list",
    "put_an_image_in_a_list",
    "take_an_image_list",
]

CASES = [
    "a_list_of_images_holds_the_images_that_were_put_in_it",
    "menu_select_fails_on_a_window_that_is_not_there_and_on_a_menu_it_cannot_reach",
    "the_pop_up_and_the_status_bar_answer",
    "status_bar_wait_of_a_part_that_is_not_one_is_refused",
    "the_kind_of_a_value_comes_before_the_parameter_that_may_not_be_left_out",
]

FIELDS = [
    "image_lists: Vec<(i64, Vec<i64>)>,",
    "image_list_addresses: i64,",
]


def block_of(lines, start):
    first = start
    while first > 0 and (
        lines[first - 1].lstrip().startswith("///")
        or lines[first - 1].lstrip().startswith("#[")
    ):
        first -= 1
    depth = 0
    seen = False
    last = start
    for index in range(start, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            seen = True
        if seen and depth == 0:
            last = index
            break
    return first, last + 1


def doc_block_of(lines, start):
    """The docs above a line and the line itself -- for a field or a const."""
    first = start
    while first > 0 and lines[first - 1].lstrip().startswith("///"):
        first -= 1
    return lines[first : start + 1]


def find_fn(lines, signature):
    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith(("fn ", "pub fn ", "pub(crate) fn ")) and signature in stripped:
            return index
    return None


def extract(source, signature):
    lines = read_lines(source)
    start = find_fn(lines, signature)
    if start is None:
        raise SystemExit(f"the branch has no {signature}")
    first, last = block_of(lines, start)
    return lines[first:last]


def remove_fn(lines, signature):
    start = find_fn(lines, signature)
    if start is None:
        return lines, False
    first, last = block_of(lines, start)
    return lines[:first] + lines[last:], True


def insert_before(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[:index] + block + lines[index:]
    raise SystemExit(f"the anchor is not there: {anchor}")


def main():
    # 1. The runtime: the base of the addresses, the two fields of the
    # lists of images, their inits and the three methods.
    runtime = read_lines(RUNTIME)
    branch_runtime = read_lines(BRANCH_RUNTIME)
    if not any("IMAGE_LIST_ADDRESS_BASE" in line for line in runtime):
        start = None
        for index, line in enumerate(branch_runtime):
            if line.startswith("const IMAGE_LIST_ADDRESS_BASE"):
                start = index
                break
        runtime = insert_before(
            runtime,
            "/// The variables of the loops, the ones that the runtime writes",
            doc_block_of(branch_runtime, start) + [""],
        )
    if not any("image_lists: Vec<(i64, Vec<i64>)>," in line for line in runtime):
        block = []
        for field in FIELDS:
            start = None
            for index, line in enumerate(branch_runtime):
                if field in line:
                    start = index
                    break
            block = block + doc_block_of(branch_runtime, start)
        runtime = insert_before(
            runtime, "    last_window: Option<String>,", block
        )
        out = []
        for line in runtime:
            out.append(line)
            if line.strip() == "groups: Vec::new()," and not any(
                "image_lists: Vec::new()," in held for held in out[-3:]
            ):
                indent = line[: len(line) - len(line.lstrip())]
                out.append(f"{indent}image_lists: Vec::new(),")
                out.append(f"{indent}image_list_addresses: 0,")
        runtime = out
    for name in METHODS:
        if any(f"fn {name}(" in line for line in runtime):
            continue
        runtime = insert_before(
            runtime,
            "    /// Remember the window a `Win*` call just found.",
            extract(BRANCH_RUNTIME, f"fn {name}(") + [""],
        )
    write_lines(RUNTIME, runtime)

    # 2. The table: the seven handlers and the seven rows.
    builtins = read_lines(BUILTINS)
    for name in HANDLERS:
        if any(f"fn {name}(" in line for line in builtins):
            continue
        builtins = builtins + [""] + extract(BRANCH_BUILTINS, f"fn {name}(")
    branch = read_lines(BRANCH_BUILTINS)
    for name, handler in ROWS:
        start = None
        for index, line in enumerate(builtins):
            if f'name: "{name}",' in line:
                start = index
                break
        if start is None:
            raise SystemExit(f"the row of {name} is not there")
        branch_row = {}
        bstart = None
        for index, line in enumerate(branch):
            if f'name: "{name}",' in line:
                bstart = index
                break
        for index in range(bstart, bstart + 9):
            stripped = branch[index].strip()
            if stripped.startswith("handler:"):
                branch_row["handler"] = stripped
            if stripped.startswith("variant:"):
                branch_row["variant"] = stripped
        for index in range(start, start + 9):
            stripped = builtins[index].strip()
            if stripped.startswith("handler:") and stripped != branch_row["handler"]:
                indent = builtins[index][: len(builtins[index]) - len(builtins[index].lstrip())]
                builtins[index] = f"{indent}{branch_row['handler']}"
            if stripped.startswith("variant:") and stripped != branch_row["variant"]:
                indent = builtins[index][: len(builtins[index]) - len(builtins[index].lstrip())]
                builtins[index] = f"{indent}{branch_row['variant']}"
    write_lines(BUILTINS, builtins)

    # 3. The five cases of the family. The case of the master that asked
    # `StatusBarGetText` for the blanket reason dies with the refusal -- the
    # branch does not carry it. The fake of this tree answers every picture
    # with a number, and the branch answers "no-such" with none, which is
    # what `IL_Add` of a picture that is not there needs; and the fake of
    # this tree names its windows "Steam" and not "first", so the case of
    # `MenuSelect` is adapted to the fake it runs against.
    tests = read_lines(TESTS)
    tests, _ = remove_fn(
        tests,
        "fn a_name_that_the_tree_does_not_carry_says_the_reason_of_the_machine_when_the_kind_fits(",
    )
    # The case of MenuSelect may already be there, of an earlier run, with
    # the window name of the branch's fake: it goes out and the adapted one
    # comes in.
    tests, _ = remove_fn(
        tests,
        "fn menu_select_fails_on_a_window_that_is_not_there_and_on_a_menu_it_cannot_reach(",
    )
    start = find_fn(tests, "fn load_picture(")
    if start is not None:
        first, last = block_of(tests, start)
        if not any('"no-such"' in line for line in tests[first:last]):
            tests = (
                tests[:first]
                + extract(BRANCH_TESTS, "fn load_picture(")
                + tests[last:]
            )
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        block = extract(BRANCH_TESTS, f"fn {name}(")
        if name.startswith("menu_select_fails"):
            block = [line.replace('\\"first\\"', '\\"Steam\\"') for line in block]
        tests = tests + [""] + block
    write_lines(TESTS, tests)

    # 4. The case of the runner: seven refusals become seven values.
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
            runner[index] = '    assert_eq!(report.number("values"), 800);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 33);'
            break
    write_lines(RUNNER, runner)

    print("the family of the image lists, the list controls and the status bars is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
