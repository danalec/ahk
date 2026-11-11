#!/usr/bin/python3
"""Append the case of the compositor that lists its windows and does not
say the focus. The case is the measurement of the VM of the matrix, on
2026-11-11: sway answers the listing and nothing about the focus, and the
tree answered 0 and the empty text where its rule says the reason.

    python audit/append-focus-case.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"

THE_CASE = """
#[test]
fn a_compositor_that_lists_its_windows_without_the_focus_answers_the_reason() {
    // Measured on the VM of the matrix on 2026-11-11: sway publishes the
    // portable listing of the toplevels and no extension that says which
    // window has the focus. `WinActive` answered 0 there and `WinGetTitle`
    // the empty text -- two wrong values with no word about them, which is
    // the worst outcome of the tree. The listing is not the question of the
    // focus: the question goes to the desktop, and the desktop answers with
    // the reason when it cannot say.
    let mut desktop = Fake {
        no_focus: true,
        ..Default::default()
    };
    let report = run_on("WinActive(\\"Steam\\")", &mut desktop);
    assert_eq!(
        failure_of(&report),
        "WinActive cannot answer: the compositor does not say which window has the focus"
    );
    let mut desktop = Fake {
        no_focus: true,
        ..Default::default()
    };
    let report = run_on("WinGetTitle()", &mut desktop);
    assert_eq!(
        failure_of(&report),
        "the compositor does not say which window has the focus"
    );
}
"""


def main():
    tests = read_lines(TESTS)
    if any("a_compositor_that_lists_its_windows_without_the_focus" in line for line in tests):
        print("the case is already there")
        return 0
    tests.extend(THE_CASE.split("\n"))
    write_lines(TESTS, tests)
    print("the case of the focus is appended")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
