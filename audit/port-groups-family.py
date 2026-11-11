#!/usr/bin/python3
"""Port the family of the window groups from the branch.

The seventh family of the port (`docs/two-lines.md` §5.2): the four
`Group*` names. A group is a set of window titles that outlives the
statement that added it, and the state lives in the runtime -- the
desktop is asked only when a group is activated, for the windows that
carry its titles.

The port carries:

- the field of the runtime, its two inits and the three methods that keep
  the set (`add_to_group`, `the_titles_of_a_group`, `close_a_group`);
- the three handlers (`GroupDeactivate` shares the one of `GroupClose`,
  the way the branch does) and the four rows of the table;
- the three cases of the family.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose handler is `not_carried`.

    python audit/port-groups-family.py
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
BRANCH_TESTS = BRANCH / "crates/ahk/tests/builtins.rs".replace("/", str(pathlib.PurePosixPath("/")))

ROWS = [
    ("GroupActivate", "group_activate"),
    ("GroupAdd", "group_add"),
    ("GroupClose", "group_close"),
    ("GroupDeactivate", "group_close"),
]

HANDLERS = ["group_add", "group_activate", "group_close"]
METHODS = ["add_to_group", "the_titles_of_a_group", "close_a_group"]
CASES = [
    "a_group_added_in_one_statement_is_read_in_a_later_one",
    "a_group_that_was_not_added_answers_zero",
    "closing_a_group_takes_it_away",
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


def insert_before(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[:index] + block + lines[index:]
    raise SystemExit(f"the anchor is not there: {anchor}")


def main():
    # 1. The state of the groups lives in the runtime: the field, the two
    # inits of the constructors and the three methods that keep the set.
    runtime = read_lines(RUNTIME)
    if not any("groups: Vec<(String, Vec<String>)>," in line for line in runtime):
        branch = read_lines(BRANCH_RUNTIME)
        start = None
        for index, line in enumerate(branch):
            if "groups: Vec<(String, Vec<String>)>," in line:
                start = index
                break
        first = start
        while first > 0 and branch[first - 1].lstrip().startswith("///"):
            first -= 1
        runtime = insert_before(
            runtime, "    last_window: Option<String>,", branch[first : start + 1]
        )
        # The field is initialised in the two constructors, right after the
        # line of the last window.
        out = []
        for line in runtime:
            out.append(line)
            if line.strip() == "last_window: None," and not any(
                "groups: Vec::new()," in held for held in out[-3:]
            ):
                indent = line[: len(line) - len(line.lstrip())]
                out.append(f"{indent}groups: Vec::new(),")
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

    # 2. The table of the language: the three handlers and the four rows.
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

    # 3. The cases of the family. The fake of this tree names its windows
    # the way the window family measured them -- "Steam", "101" -- and not
    # the way the fake of the branch names them -- "first", "0x1" -- so the
    # case of the group is adapted to the fake it runs against.
    tests = read_lines(TESTS)
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        block = extract(BRANCH_TESTS, f"fn {name}(")
        block = [
            line.replace('"first"', '"Steam"').replace('"0x1"', '"101"')
            for line in block
        ]
        tests = tests + [""] + block
    write_lines(TESTS, tests)

    # 4. The case of the runner reads the numbers of the surface, and they
    # move with the family: four refusals become four values.
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
            runner[index] = '    assert_eq!(report.number("values"), 767);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 66);'
            break
    write_lines(RUNNER, runner)

    print("the family of the groups is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
