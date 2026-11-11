#!/usr/bin/python3
"""Port the family of the small commands from the branch.

The tenth family of the port (`docs/two-lines.md` §5.2): five names that
needed no desktop of their own.

- `Edit` is refused with a reason of its own: the script of this runtime
  is read from the standard input, so it has no file for an editor to
  open. The row stays a static refusal, the way the branch keeps it.
- `BlockInput` answers the empty text for "Off" -- and refuses turning
  the input OFF for the whole machine, a privilege this compositor does
  not give a client.
- `RunAs` answers the empty text and remembers nothing: there is no user
  to run as on this side of the dialect.
- `Shutdown` refuses to log off, reboot or power down -- a script runner
  does not answer that -- but it validates the number first, the way the
  binary does.
- `FileGetVersion` reads the version resource out of the bytes of the
  file, wherever the file is: a program of Windows may be read on any
  platform. It brings the `version` module of the branch with it, and an
  image with no resource section throws "(1812) The specified image file
  did not contain a resource section.".

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-small-family.py
"""

import pathlib
import shutil
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

LIB = ROOT / "crates" / "ahk" / "src" / "lib.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_VERSION = BRANCH / "crates" / "ahk" / "src" / "version.rs"
PORT_VERSION = ROOT / "crates" / "ahk" / "src" / "version.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

ROWS = [
    ("BlockInput", "block_input"),
    ("Edit", "edit"),
    ("FileGetVersion", "file_get_version"),
    ("RunAs", "run_as"),
    ("Shutdown", "shutdown"),
]

HANDLERS = ["edit", "block_input", "run_as", "shutdown", "file_get_version"]

CASES = ["the_version_of_a_program_is_read_out_of_the_file_of_it"]
HELPERS = ["a_program_with_a_version"]

BEHAVIOR = ROOT / "crates" / "ahk" / "src" / "behavior.rs"

NEW_BEHAVIOR_TEST = '''#[test]
    fn a_refused_name_answers_with_its_reason() {
        // `WinActivate` was the example until the family of the windows was
        // carried on 2026-11-11, and `BlockInput` until the family of the
        // small commands was carried the same day: the name that is still
        // refused with the blanket reason is `ComObjGet`, a name that is
        // shaped for Windows through and through.
        let answer = the_answer("ComObjGet");
        assert!(answer.starts_with("refusal: "), "{answer}");
        // The reason is the one the surface prints.
        assert_eq!(
            answer,
            "refusal: this is a name of Windows, and this desktop is not Windows."
        );
    }'''


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
    # 1. The version module comes over whole, bytes for bytes, and the
    # language crate names it. The module of the branch leans on the std
    # prelude for `format!`, `String` and `Vec`; the language crate builds
    # without std too, so the imports of alloc go in after the copy.
    if not PORT_VERSION.exists():
        shutil.copyfile(BRANCH_VERSION, PORT_VERSION)
    version = read_lines(PORT_VERSION)
    if not any("use alloc::format;" in line for line in version):
        for index, line in enumerate(version):
            if line.strip().startswith("/// The type of the resource"):
                version = version[:index] + [
                    "use alloc::format;",
                    "use alloc::string::{String, ToString};",
                    "use alloc::vec::Vec;",
                    "",
                ] + version[index:]
                break
        else:
            raise SystemExit("the anchor of the version module is not there")
    write_lines(PORT_VERSION, version)
    lib = read_lines(LIB)
    if not any("pub mod version;" in line for line in lib):
        for index, line in enumerate(lib):
            if line.strip() == "pub mod variables;":
                lib = lib[: index + 1] + ["pub mod version;"] + lib[index + 1 :]
                break
        else:
            raise SystemExit("the anchor of the modules is not there")
    write_lines(LIB, lib)

    # 2. The table: the five handlers and the five rows.
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

    # 3. The case of FileGetVersion and the helper that makes the program
    # of Windows it reads. The fake has to read the bytes of the file the
    # test put in its map, the way the fake of the branch does: the fake of
    # this tree answered a fixed text, and the version of the program was
    # never in it.
    tests = read_lines(TESTS)
    start = find_fn(tests, "fn read_the_bytes_of_a_file(")
    if start is not None:
        first, last = block_of(tests, start)
        if not any("self.files.get(path)" in line for line in tests[first:last]):
            tests = tests[:first] + extract(BRANCH_TESTS, "fn read_the_bytes_of_a_file(") + tests[last:]
    for name in HELPERS + CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    # 4. The example of the blanket refusal in behavior.rs moves: the name
    # it used, `BlockInput`, is carried now, and the example becomes a name
    # that is shaped for Windows through and through.
    behavior = read_lines(BEHAVIOR)
    if any('the_answer("BlockInput")' in line for line in behavior):
        start = find_fn(behavior, "fn a_refused_name_answers_with_its_reason(")
        if start is None:
            raise SystemExit("the behavior case of the refusal is not there")
        first, last = block_of(behavior, start)
        block = NEW_BEHAVIOR_TEST.split("\n")
        block[0] = "    " + block[0]
        behavior = behavior[:first] + block + behavior[last:]
    write_lines(BEHAVIOR, behavior)

    # 5. The case of the runner: four refusals become four values. `Edit`
    # stays a refusal -- the branch keeps it one, with the reason of its
    # own at the run.
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
            runner[index] = '    assert_eq!(report.number("values"), 779);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 54);'
            break
    # And the example of a refused name in the same case moves with the
    # family: `BlockInput` answers now, and `ComObjGet` is still refused.
    for index in range(start, len(runner)):
        if 'field_text(entry, "name") == "BlockInput"' in runner[index]:
            runner[index] = runner[index].replace('"BlockInput"', '"ComObjGet"')
            break
    write_lines(RUNNER, runner)

    print("the family of the small commands is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
