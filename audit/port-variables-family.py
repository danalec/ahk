#!/usr/bin/python3
"""Port three of the built-in variables from the branch.

The fourteenth family of the port (`docs/two-lines.md` §5.2): `A_WinDir`,
`A_ProgramFiles` and `A_ScreenDPI`.

- `A_WinDir` and `A_ProgramFiles` are the machine's: the box has no
  Windows of its own, and the raven is one -- it emulates a Windows on
  the box, and the paths of its file system are the ones it says
  (`GetWindowsDirectoryA` and the environment of the emulated system).
  On a machine that IS a Windows, the environment of the machine says
  the same two paths.
- `A_ScreenDPI` is not a name of the machine at all: the branch
  reclassifies it as a question of the screen, and the answer is the 96
  of v2 with the scale factor the session reports applied.

The trait grows the two methods of the paths, with the defaults that
refuse; the box carries the raven call and the environment fallback; the
fake of the variables tests says the two paths of a Windows; and the
case of the master that pinned the refusal of `A_WinDir` becomes the
case of the branch that pins the answer.

The script is idempotent.

    python audit/port-variables-family.py
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

DESKTOP = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
VARIABLES = ROOT / "crates" / "ahk" / "src" / "variables.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "variables.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_DESKTOP = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "variables.rs"

CASES = [
    "a_name_of_windows_refuses_with_the_reason_of_the_contract",
    "the_windows_of_the_machine_are_the_ones_the_machine_says",
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


def find_fn(lines, signature, after=0):
    for index in range(after, len(lines)):
        stripped = lines[index].strip()
        if stripped.startswith(("fn ", "pub fn ", "pub(crate) fn ")) and signature in stripped:
            return index
    return None


def extract(source, signature, after_anchor=None):
    lines = read_lines(source)
    after = 0
    if after_anchor is not None:
        for index, line in enumerate(lines):
            if after_anchor in line:
                after = index
                break
        else:
            raise SystemExit(f"the anchor is not there: {after_anchor}")
    start = find_fn(lines, signature, after)
    if start is None:
        raise SystemExit(f"the branch has no {signature}")
    first, last = block_of(lines, start)
    return lines[first:last]


def extract_region(source, start_marker, end_marker):
    lines = read_lines(source)
    start = None
    for index, line in enumerate(lines):
        if start_marker in line:
            start = index
            break
    if start is None:
        raise SystemExit(f"the branch has no {start_marker}")
    first = start
    while first > 0 and lines[first - 1].lstrip().startswith(("///", "//")):
        first -= 1
    for index in range(start, len(lines)):
        if end_marker in lines[index]:
            return lines[first : index + 1]
    raise SystemExit(f"the region of {start_marker} does not end")


def insert_after(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[: index + 1] + block + lines[index + 1 :]
    raise SystemExit(f"the anchor is not there: {anchor}")


def insert_before(lines, anchor, block):
    for index, line in enumerate(lines):
        if anchor in line:
            return lines[:index] + block + lines[index:]
    raise SystemExit(f"the anchor is not there: {anchor}")


def remove_fn(lines, signature):
    start = find_fn(lines, signature)
    if start is None:
        return lines, False
    first, last = block_of(lines, start)
    while last < len(lines) and lines[last].strip() == "":
        last += 1
    return lines[:first] + lines[last:], True


def main():
    # 1. The trait of the language grows the two paths of the Windows of
    # the machine, with the defaults that refuse.
    desktop = read_lines(DESKTOP)
    for name in ("windows_directory", "program_files"):
        if any(f"fn {name}(" in line for line in desktop):
            continue
        desktop = insert_after(
            desktop, "pub trait Machine", [""] + extract(BRANCH_DESKTOP, f"fn {name}(")
        )
    write_lines(DESKTOP, desktop)

    # 2. The variables: the screen arm of the scale, the two arms of the
    # paths, the kind lists and the refusal that goes away.
    variables = read_lines(VARIABLES)
    branch_variables = read_lines(ROOT.parent / "audit-ahk-x11-parity" / "crates" / "ahk" / "src" / "variables.rs")
    if not any('name == "screendpi"' in line for line in variables):
        arm = extract_region(
            ROOT.parent / "audit-ahk-x11-parity" / "crates" / "ahk" / "src" / "variables.rs",
            'Kind::Screen if name == "screendpi"',
            "}),",
        )
        variables = insert_before(
            variables,
            "        Kind::Screen => Some(match rt.desktop().screen_size() {",
            arm,
        )
    if not any('"windir" => text(' in line for line in variables):
        arm = extract_region(
            ROOT.parent / "audit-ahk-x11-parity" / "crates" / "ahk" / "src" / "variables.rs",
            '"windir" => text(',
            '"programfiles" => text(',
        )
        variables = insert_before(
            variables,
            '        "appdata" | "appdatacommon" | "desktop" | "desktopcommon" | "startmenu"',
            arm,
        )
    variables = [
        line.replace('| "programscommon" | "screendpi"', '| "programscommon"')
        .replace(
            '"screenheight" | "screenwidth" => Some(Kind::Screen),',
            '"screenheight" | "screenwidth" | "screendpi" => Some(Kind::Screen),',
        )
        for line in variables
    ]
    variables = [
        line
        for line in variables
        if '"windir" | "programfiles" => NOT_WINDOWS,' not in line
    ]
    # The reason of the blanket is used by no name now.
    if not any("NOT_WINDOWS" in line and "const NOT_WINDOWS" not in line for line in variables):
        out = []
        skip_doc = False
        for index, line in enumerate(variables):
            if "const NOT_WINDOWS" in line:
                # Take the doc lines above the const away with it.
                while out and out[-1].lstrip().startswith("///"):
                    out.pop()
                skip_doc = True
                continue
            out.append(line)
        variables = out
    write_lines(VARIABLES, variables)

    # 3. The box: the raven call and its two paths, inherent and in the
    # trait impl. The inherent methods go INSIDE the `impl LabDesktop`
    # that ends just above the comment of the keyboard codes -- the
    # comment itself is a top-level one, so it cannot be the anchor.
    box = read_lines(BOX)
    if not any("use ahk::dll::{Convention, NativeArg, NativeType, NativeValue};" in line for line in box):
        box = insert_after(
            box,
            "use ahk::dll::{NativeCall, NativeFailure, NativeResult};",
            ["use ahk::dll::{Convention, NativeArg, NativeType, NativeValue};"],
        )
    if not any("fn a_path_of_the_windows(" in line for line in box):
        block = []
        for name in ("a_path_of_the_windows", "windows_directory", "program_files"):
            block = block + extract(BRANCH_BOX, f"fn {name}(") + [""]
        codes = None
        for index, line in enumerate(box):
            if "/// The codes the device announces." in line:
                codes = index
                break
        if codes is None:
            raise SystemExit("the comment of the keyboard codes is not there")
        close = codes - 1
        while box[close].strip() == "":
            close -= 1
        if box[close].strip() != "}":
            raise SystemExit("the impl does not close where the comment says")
        box = box[:close] + [""] + block + box[close:]
    if sum(1 for line in box if "fn windows_directory(" in line) < 2:
        block = extract(
            BRANCH_BOX,
            "fn windows_directory(",
            after_anchor="impl ahk::desktop::Machine for LabDesktop",
        )
        block = block + extract(
            BRANCH_BOX,
            "fn program_files(",
            after_anchor="impl ahk::desktop::Machine for LabDesktop",
        )
        box = insert_after(
            box, "impl ahk::desktop::Machine for LabDesktop", [""] + block
        )
    write_lines(BOX, box)

    # 4. The tests of the variables: the fake says the two paths of a
    # Windows, the case of the refusal of `A_WinDir` becomes the case of
    # the branch that pins the answer, and the case of the two paths
    # comes over.
    tests = read_lines(TESTS)
    for name in ("windows_directory", "program_files"):
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = insert_after(
            tests, "impl Machine for Fake {", [""] + extract(BRANCH_TESTS, f"fn {name}(")
        )
    tests, _ = remove_fn(tests, "fn a_name_of_windows_refuses_with_the_reason_of_the_contract(")
    # The coherence case of the refusals loses the two names that answer
    # now; `A_IconFile` stays while the tray is not carried.
    for index, line in enumerate(tests):
        if '"A_WinDir",' in line and '"A_ProgramFiles",' in tests[index + 1]:
            tests = (
                tests[:index]
                + [
                    line.replace('"A_WinDir",', '"A_Cursor",'),
                    tests[index + 1].replace('"A_ProgramFiles",', '"A_ScriptHwnd",'),
                    tests[index + 2].replace('"A_Cursor",', '// `A_IconFile` stays here while the tray is not carried: the branch'),
                    tests[index + 3].replace('"A_ScriptHwnd",', '// answers it, and the loop of the branch lost the name there.'),
                    tests[index + 4],
                ]
                + tests[index + 5 :]
            )
            break
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    # 5. The case of the runner: three refusals become three values.
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
            runner[index] = '    assert_eq!(report.number("values"), 811);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 22);'
            break
    # The surface of the v1 shares the variables of the machine: the three
    # names answer in both dialects, and the count of the v1 moves with the
    # one of the v2.
    for index, line in enumerate(runner):
        if 'report.number("values"), 302' in line:
            runner[index] = '    assert_eq!(report.number("values"), 306);'
            break
    write_lines(RUNNER, runner)

    print("the variables of the machine and of the screen are ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
