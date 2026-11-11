#!/usr/bin/python3
"""Port the family of the sound devices from the branch.

The sixth family of the port (`docs/two-lines.md` §5.2): the two names
that ask the mixer of the session which device plays and which one
records. The branch measures them on the binary on 2026-11-11 and reads
the mixer of Linux with `pactl`, which the box already speaks for the
volume and the mute of the family that came before.

The port carries:

- the two methods of the `Sound` trait and the helper of the box that
  reads the description of the device the mixer chose;
- the two handlers and the reader of the identifier of an interface;
- the two rows of the table, the two methods of the fake and the case of
  the family.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose handler is `not_carried`.

    python audit/port-sound-family.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

DESKTOP = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_DESKTOP = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

ROWS = [
    ("SoundGetName", "sound_get_name"),
    ("SoundGetInterface", "sound_get_interface"),
]

TRAIT_METHODS = ["sound_device_names", "sound_device_named"]
BOX_METHODS = ["sound_device_names", "sound_device_named"]
HANDLERS = [
    "sound_get_name",
    "sound_get_interface",
    "is_the_identifier_of_an_interface",
]
FAKE_METHODS = ["sound_device_names", "sound_device_named"]
CASES = [
    "the_names_of_the_sound_that_are_windows_fail_with_the_reason",
    "the_name_of_a_sound_device_is_the_name_the_machine_gives_it",
]


def block_of(lines, start):
    """The span of a function: the docs and attributes above it, and the
    braces from its body to the end."""
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
        if stripped.startswith(("fn ", "pub(crate) fn ", "pub fn ")) and signature in stripped:
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


def insert_after(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[: index + 1] + block + lines[index + 1 :]
    raise SystemExit(f"the anchor is not there: {anchor}")


def main():
    # 1. The trait of the language grows the two questions of the mixer.
    desktop = read_lines(DESKTOP)
    for name in TRAIT_METHODS:
        if any(f"fn {name}(" in line for line in desktop):
            continue
        desktop = insert_after(
            desktop, "pub trait Sound", [""] + extract(BRANCH_DESKTOP, f"fn {name}(")
        )
    write_lines(DESKTOP, desktop)

    # 2. The table of the language: the two handlers, the reader of the
    # identifier of an interface, and the rows that point at them.
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

    # 3. The box: the two implementations and the helper that reads the
    # description of the device the mixer chose.
    box = read_lines(BOX)
    if not any("fn the_description_of(" in line for line in box):
        box = insert_after(
            box,
            "    // ---- gui_title_is",
            [""] + extract(BRANCH_BOX, "fn the_description_of("),
        )
    for name in BOX_METHODS:
        if any(f"fn {name}(" in line for line in box):
            continue
        box = insert_after(
            box,
            "impl ahk::desktop::Sound for LabDesktop",
            [""] + extract(BRANCH_BOX, f"fn {name}("),
        )
    write_lines(BOX, box)

    # 4. The fake and the case of the suite. The case of the master that
    # asked for the blanket refusal dies with the refusal: the branch keeps
    # its name and gives it the body of the family.
    tests = read_lines(TESTS)
    tests, _ = remove_fn(
        tests, "fn the_names_of_the_sound_that_are_windows_fail_with_the_reason("
    )
    for name in FAKE_METHODS:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = insert_after(
            tests, "impl Sound for Fake", [""] + extract(BRANCH_TESTS, f"fn {name}(")
        )
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    # 5. The case of the runner reads the numbers of the surface, and they
    # move with the family: two refusals become two values.
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
            runner[index] = '    assert_eq!(report.number("values"), 763);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 70);'
            break
    write_lines(RUNNER, runner)

    print("the family of the sound devices is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
