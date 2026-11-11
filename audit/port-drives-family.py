#!/usr/bin/python3
"""Port the family of the drives from the branch.

The fifth family of the port (`docs/two-lines.md` §5.2): the drives of the
machine. The branch measures them on the box on 2026-11-10 and answers
them with the tools of Linux -- `df` for the space, `lsblk` for the list
and the serial, `eject` and `udisksctl` for the hardware words -- where
this tree refuses seven of the fourteen names and answers the other seven
with one generic handler that the branch does not have.

The port carries:

- the `DriveSpace` struct and the ten methods of the `Machine` trait that
  replace the two generic ones of this tree (`drive_type`, `drive_status`);
- the fourteen handlers of the table and the two wordings of the machine
  (`DRIVE_PATH`, `DRIVE_FILE`), and the rows point at them;
- the ten implementations of the box and the three helpers that read the
  machine (`the_filesystem_of`, `the_device_of`, `the_kind_of`);
- the drive methods of the fake and the nine cases of the family.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose handler is `drive_get` or `not_carried`.

    python audit/port-drives-family.py
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

# The fourteen names and the handler of each one in the branch.
ROWS = [
    ("DriveEject", "drive_eject"),
    ("DriveGetCapacity", "drive_get_capacity"),
    ("DriveGetFilesystem", "drive_get_filesystem"),
    ("DriveGetLabel", "drive_get_label"),
    ("DriveGetList", "drive_get_list"),
    ("DriveGetSerial", "drive_get_serial"),
    ("DriveGetSpaceFree", "drive_get_space_free"),
    ("DriveGetStatus", "drive_get_status"),
    ("DriveGetStatusCD", "drive_get_status_cd"),
    ("DriveGetType", "drive_get_type"),
    ("DriveLock", "drive_lock"),
    ("DriveRetract", "drive_retract"),
    ("DriveSetLabel", "drive_set_label"),
    ("DriveUnlock", "drive_unlock"),
]

# The methods of the trait and of the box and of the fake, by name.
DRIVE_METHODS = [
    "drive_space",
    "drive_label",
    "drive_serial",
    "drive_list",
    "drive_status_cd",
    "drive_eject",
    "drive_lock",
    "drive_unlock",
    "drive_retract",
    "drive_set_label",
]

HANDLERS = [
    "drive_get_label",
    "drive_get_serial",
    "drive_get_list",
    "drive_get_status_cd",
    "the_drive_for_the_operation",
    "the_drive_of",
    "drive_eject",
    "drive_lock",
    "drive_unlock",
    "drive_retract",
    "drive_set_label",
    "drive_get_space_free",
    "drive_get_capacity",
    "drive_get_status",
    "drive_get_filesystem",
    "drive_get_type",
]

HELPERS = ["the_filesystem_of", "the_device_of", "the_kind_of"]

CASES = [
    "the_drive_family_answers_the_label_the_serial_the_list_and_the_hardware_words",
    "drive_get_space_free_answers_the_free_megabytes",
    "drive_get_capacity_answers_the_capacity_in_megabytes",
    "drive_get_space_free_of_a_drive_that_is_not_there_fails",
    "drive_get_status_answers_ready_for_a_drive_that_is_there",
    "drive_get_status_of_a_drive_that_is_not_there_is_empty",
    "drive_get_filesystem_answers_the_filesystem_of_the_machine",
    "drive_get_type_answers_the_kind_in_the_word_of_v2",
    "drive_get_type_of_a_drive_that_is_not_there_is_empty",
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


def insert_at(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[:index] + block + lines[index:]
    raise SystemExit(f"the anchor is not there: {anchor}")


def main():
    # 1. The trait of the language: the two generic methods of this tree
    # give their place to the ten of the branch, and the struct of the
    # space of a drive comes with them.
    desktop = read_lines(DESKTOP)
    desktop, _ = remove_fn(desktop, "fn drive_type(")
    desktop, _ = remove_fn(desktop, "fn drive_status(")
    if not any("pub struct DriveSpace" in line for line in desktop):
        struct = []
        branch = read_lines(BRANCH_DESKTOP)
        start = None
        for index, line in enumerate(branch):
            if "pub struct DriveSpace" in line:
                start = index
                break
        first, last = block_of(branch, start)
        # The docs of the struct start above the derive.
        while first > 0 and branch[first - 1].lstrip().startswith("///"):
            first -= 1
        struct = branch[first:last]
        desktop = insert_at(desktop, "pub trait Machine", struct + [""])
    for name in DRIVE_METHODS:
        if any(f"fn {name}(" in line for line in desktop):
            continue
        at = None
        for index, line in enumerate(desktop):
            if "pub trait Machine" in line:
                at = index + 1
                break
        desktop = (
            desktop[:at] + [""] + extract(BRANCH_DESKTOP, f"fn {name}(") + desktop[at:]
        )
    write_lines(DESKTOP, desktop)

    # 2. The table of the language: the generic handler goes, the fourteen
    # handlers and the two wordings come, and the rows point at them.
    builtins = read_lines(BUILTINS)
    builtins, removed = remove_fn(builtins, "pub(crate) fn drive_get(")
    # The helper of the generic handler dies with it: nothing else reads a
    # drive the old way.
    builtins, _ = remove_fn(builtins, "fn the_drive_reading(")
    if not any("const DRIVE_PATH" in line for line in builtins):
        branch = read_lines(BRANCH_BUILTINS)
        constants = []
        for index, line in enumerate(branch):
            if "const DRIVE_PATH" in line or "const DRIVE_FILE" in line:
                first = index
                while first > 0 and branch[first - 1].lstrip().startswith("///"):
                    first -= 1
                constants += branch[first : index + 1] + [""]
        builtins = builtins + [""] + constants
    for name in HANDLERS:
        if any(f"fn {name}(" in line for line in builtins):
            continue
        builtins = builtins + [""] + extract(BRANCH_BUILTINS, f"fn {name}(")
    for name, handler in ROWS:
        start = None
        for index, line in enumerate(builtins):
            if f'name: "{name}",' in line:
                start = index
                break
        if start is None:
            raise SystemExit(f"the row of {name} is not there")
        # The row of the branch carries the variant and the handler of the
        # family: a variant of 100 is the marker of a name that is not
        # carried, and the surface reads it, so the handler alone does not
        # move the surface.
        branch_row = {}
        branch = read_lines(BRANCH_BUILTINS)
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

    # 3. The box: the two generic implementations go, the ten of the branch
    # and the three helpers that read the machine come.
    box = read_lines(BOX)
    box, _ = remove_fn(box, "fn drive_type(")
    box, _ = remove_fn(box, "fn drive_status(")
    for name in HELPERS:
        if any(f"fn {name}(" in line for line in box):
            continue
        box = insert_at(
            box, "impl ahk::desktop::Machine for LabDesktop", extract(BRANCH_BOX, f"fn {name}(") + [""]
        )
    for name in DRIVE_METHODS:
        if any(f"fn {name}(" in line for line in box):
            continue
        at = None
        for index, line in enumerate(box):
            if "impl ahk::desktop::Machine for LabDesktop" in line:
                at = index + 1
                break
        box = box[:at] + extract(BRANCH_BOX, f"fn {name}(") + box[at:]
    write_lines(BOX, box)

    # 3b. The X11 desktop of this tree implemented the two generic methods
    # of the old trait; the trait of the branch has none of them, and the
    # X11 desktop of the branch implements none of the drive methods -- the
    # ones of the box answer for every session.
    x11 = ROOT / "crates" / "ahk-box" / "src" / "x11.rs"
    x11_lines = read_lines(x11)
    x11_lines, _ = remove_fn(x11_lines, "fn drive_type(")
    x11_lines, _ = remove_fn(x11_lines, "fn drive_status(")
    # The drives of an X11 session are the drives of the machine: the X11
    # desktop of this tree keeps answering the family, by asking a desktop
    # of the lab that is made only to answer it. The branch has no X11
    # desktop with drive methods, and the master had two of them -- this is
    # the hand merge of the two shapes.
    signatures = {
        "drive_space": "fn drive_space(&mut self, path: &str) -> Result<Option<ahk::desktop::DriveSpace>, String> {\n        crate::desktop::LabDesktop::new().drive_space(path)\n    }",
        "drive_label": "fn drive_label(&mut self, path: &str) -> Result<Option<String>, String> {\n        crate::desktop::LabDesktop::new().drive_label(path)\n    }",
        "drive_serial": "fn drive_serial(&mut self, path: &str) -> Result<Option<String>, String> {\n        crate::desktop::LabDesktop::new().drive_serial(path)\n    }",
        "drive_list": "fn drive_list(&mut self) -> Result<Vec<String>, String> {\n        crate::desktop::LabDesktop::new().drive_list()\n    }",
        "drive_status_cd": "fn drive_status_cd(&mut self, path: &str) -> Result<Option<String>, String> {\n        crate::desktop::LabDesktop::new().drive_status_cd(path)\n    }",
        "drive_eject": "fn drive_eject(&mut self, path: &str) -> Result<(), String> {\n        crate::desktop::LabDesktop::new().drive_eject(path)\n    }",
        "drive_lock": "fn drive_lock(&mut self, path: &str) -> Result<(), String> {\n        crate::desktop::LabDesktop::new().drive_lock(path)\n    }",
        "drive_unlock": "fn drive_unlock(&mut self, path: &str) -> Result<(), String> {\n        crate::desktop::LabDesktop::new().drive_unlock(path)\n    }",
        "drive_retract": "fn drive_retract(&mut self, path: &str) -> Result<(), String> {\n        crate::desktop::LabDesktop::new().drive_retract(path)\n    }",
        "drive_set_label": "fn drive_set_label(&mut self, path: &str, label: &str) -> Result<(), String> {\n        crate::desktop::LabDesktop::new().drive_set_label(path, label)\n    }",
    }
    at = None
    for index, line in enumerate(x11_lines):
        if "impl Machine for X11Desktop" in line:
            at = index + 1
            break
    if at is None:
        raise SystemExit("the Machine impl of the X11 desktop is not there")
    added = []
    for name in DRIVE_METHODS:
        if any(f"fn {name}(" in line for line in x11_lines):
            continue
        added += ["    " + line for line in signatures[name].split("\n")]
    if added:
        x11_lines = x11_lines[:at] + added + x11_lines[at:]
    write_lines(x11, x11_lines)

    # 4. The fake and the cases of the suite. The case of the master that
    # read the two generic methods of the old trait goes with them: the
    # branch replaced it with the nine cases of the family.
    tests = read_lines(TESTS)
    tests, _ = remove_fn(tests, "fn the_readings_of_a_drive_are_asked_of_the_desktop(")
    tests, _ = remove_fn(tests, "fn drive_type(")
    tests, _ = remove_fn(tests, "fn drive_status(")
    if "DriveSpace" not in "\n".join(tests[:25]):
        for index, line in enumerate(tests):
            if line.strip().startswith("PictureOptions, Pictures,"):
                tests[index] = line.replace("Timers,", "DriveSpace, Timers,")
                break
    for name in DRIVE_METHODS:
        if any(f"fn {name}(" in line for line in tests):
            continue
        at = None
        for index, line in enumerate(tests):
            if "impl Machine for Fake" in line:
                at = index + 1
                break
        tests = tests[:at] + extract(BRANCH_TESTS, f"fn {name}(") + tests[at:]
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests += [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    # 5. The case of the runner reads the numbers of the surface, and they
    # move with the family: seven refusals become seven values.
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
            runner[index] = '    assert_eq!(report.number("values"), 761);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 72);'
            break
    write_lines(RUNNER, runner)

    print("the family of the drives is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
