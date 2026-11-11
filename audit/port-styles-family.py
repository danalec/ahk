#!/usr/bin/python3
"""Port the family of the window styles from the branch.

The eighth family of the port (`docs/two-lines.md` §5.2): the six names
that read and write the style numbers of a window of Windows. The branch
refuses them with a reason of their own -- under every desktop of this
tree the decoration belongs to the compositor, and a client has no style
number at all -- where this tree refuses them with the blanket reason of
the names of Windows.

The port carries the two handlers (`win_style` for the four style names,
`win_trans_color` for the two transparency names), the six rows of the
table, and one case of its own that pins the two reasons: the branch has
no case for the family, and a refusal with a reason of its own is exactly
the kind of answer a case keeps honest.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-styles-family.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"

ROWS = [
    ("WinGetExStyle", "win_style"),
    ("WinGetStyle", "win_style"),
    ("WinGetTransColor", "win_trans_color"),
    ("WinSetExStyle", "win_style"),
    ("WinSetStyle", "win_style"),
    ("WinSetTransColor", "win_trans_color"),
]

HANDLERS = ["win_style", "win_trans_color"]

THE_CASE = """
#[test]
fn the_styles_of_a_window_are_refused_with_the_reason_of_the_compositor() {
    // The decoration of a window belongs to the compositor under every
    // desktop of this tree, and the style number of Windows has no
    // counterpart here: the branch refused them with these reasons of its
    // own on 2026-11-10, and a refusal with a reason of its own is exactly
    // the answer a case keeps honest.
    let report = run("WinGetStyle(\\"A\\")");
    assert_eq!(
        failure_of(&report),
        "a window style is a number of Windows that says how a window is decorated, and under this compositor the decoration belongs to the compositor and a client has no style number at all"
    );
    let report = run("WinGetTransColor(\\"A\\")");
    assert_eq!(
        failure_of(&report),
        "a transparency colour is a layered window of Windows, where one colour is made invisible; this tree sets an opacity for the whole window, which is WinSetTransparent and WinGetTransparent"
    );
}
"""


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


def main():
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

    tests = read_lines(TESTS)
    if not any("fn the_styles_of_a_window_are_refused_with_the_reason" in line for line in tests):
        tests = tests + THE_CASE.split("\n")
    write_lines(TESTS, tests)

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

    print("the family of the styles is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

