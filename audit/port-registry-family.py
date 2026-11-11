#!/usr/bin/python3
"""Port the registry of the machine and the log trio from the branch.

The thirteenth family of the port (`docs/two-lines.md` §5.2): eight names.

- The five `Reg*` names: Windows has a registry and this platform has
  none, so the branch carries the API over a store of its own -- the
  hives are folders and the keys are files under the config folder of the
  user, in the `registry` module of the box. The module comes over whole,
  the trait grows the five methods (with the defaults that refuse), the
  box delegates to the module, and the handlers are the branch's.
- `ListLines` answers the setting that was there and writes the new one;
  `ListVars` puts a line for each variable of the script in the output of
  the report -- the window of v2 does not exist here, and the output is
  the one channel a script has to show things; `Persistent` answers zero,
  because this runtime is one script that ends when it ends.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-registry-family.py
"""

import pathlib
import shutil
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = ROOT.parent / "audit-ahk-x11-parity"

DESKTOP = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
RUNTIME = ROOT / "crates" / "ahk" / "src" / "runtime.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BOX_LIB = ROOT / "crates" / "ahk-box" / "src" / "lib.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_REGISTRY = BRANCH / "crates" / "ahk-box" / "src" / "registry.rs"
PORT_REGISTRY = ROOT / "crates" / "ahk-box" / "src" / "registry.rs"
BRANCH_DESKTOP = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_RUNTIME = BRANCH / "crates" / "ahk" / "src" / "runtime.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

ROWS = [
    ("ListLines", "list_lines"),
    ("ListVars", "list_vars"),
    ("Persistent", "persistent"),
    ("RegCreateKey", "reg_create_key"),
    ("RegDelete", "reg_delete"),
    ("RegDeleteKey", "reg_delete_key"),
    ("RegRead", "reg_read"),
    ("RegWrite", "reg_write"),
]

TRAIT_METHODS = [
    "registry_value",
    "registry_write",
    "registry_delete",
    "registry_delete_key",
    "registry_create_key",
]

HANDLERS = [
    "the_key_of",
    "the_value_name_of",
    "reg_create_key",
    "reg_write",
    "reg_read",
    "reg_delete",
    "reg_delete_key",
    "list_lines",
    "list_vars",
    "persistent",
]

CASES = [
    "the_registry_of_the_machine_answers_the_shapes_the_binary_carries",
    "a_view_of_the_registry_that_is_not_one_is_refused",
    "the_log_of_the_lines_answers_the_setting_that_was_there",
]

THE_LIST_CASE = '''
#[test]
fn the_variables_are_listed_in_the_output_and_the_script_may_stay_alive() {
    // The window of v2 that lists the variables does not exist here: each
    // variable goes as a line into the output of the report, the way a
    // `MsgBox` goes, and the answer is the empty text. `Persistent`
    // answers zero and changes nothing: this runtime is one script that
    // ends when it ends. A case of this tree -- the branch carries the two
    // names but pins neither in a case.
    let report = run(concat!(
        "answer := Persistent()",
        "\\nshown := ListVars()",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "answer"), Value::Int(0));
    assert_eq!(value_of(&report, "shown"), Value::Text(String::new()));
    assert!(
        report.output.iter().any(|line| line.starts_with("answer[")),
        "{:?}",
        report.output
    );
}
'''


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


def const_block(lines, start):
    first = start
    while first > 0 and lines[first - 1].lstrip().startswith("///"):
        first -= 1
    for index in range(start, len(lines)):
        if lines[index].strip() == "];":
            return lines[first : index + 1]
    raise SystemExit("the const does not end")


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


def main():
    # 1. The registry module of the box comes over whole, and the crate
    # names it.
    if not PORT_REGISTRY.exists():
        shutil.copyfile(BRANCH_REGISTRY, PORT_REGISTRY)
    box_lib = read_lines(BOX_LIB)
    if not any("pub mod registry;" in line for line in box_lib):
        box_lib = insert_after(box_lib, "pub mod raven;", ["pub mod registry;"])
    write_lines(BOX_LIB, box_lib)

    # 2. The trait of the language grows the five questions of the
    # registry, with the defaults that refuse.
    desktop = read_lines(DESKTOP)
    for name in TRAIT_METHODS:
        if any(f"fn {name}(" in line for line in desktop):
            continue
        desktop = insert_after(
            desktop, "pub trait Machine", [""] + extract(BRANCH_DESKTOP, f"fn {name}(")
        )
    write_lines(DESKTOP, desktop)

    # 3. The box delegates to the module.
    box = read_lines(BOX)
    for name in TRAIT_METHODS:
        if any(f"fn {name}(" in line for line in box):
            continue
        box = insert_after(
            box,
            "impl ahk::desktop::Machine for LabDesktop",
            [""] + extract(BRANCH_BOX, f"fn {name}("),
        )
    write_lines(BOX, box)

    # 4. The table: the kinds of a value, the two readers of the
    # parameters, the eight handlers and the eight rows.
    builtins = read_lines(BUILTINS)
    branch = read_lines(BRANCH_BUILTINS)
    if not any("const REGISTRY_KINDS" in line for line in builtins):
        start = None
        for index, line in enumerate(branch):
            if line.strip().startswith("const REGISTRY_KINDS"):
                start = index
                break
        builtins = builtins + [""] + const_block(branch, start)
    # The two wordings of the machine the handlers quote.
    for wanted in ("const NO_VALUE_NAME", "const NOTHING_THERE"):
        if not any(wanted in line for line in builtins):
            start = None
            for index, line in enumerate(branch):
                if line.strip().startswith(wanted):
                    start = index
                    break
            first = start
            while first > 0 and branch[first - 1].lstrip().startswith("///"):
                first -= 1
            builtins = builtins + [""] + branch[first : start + 1]
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

    # 5. The runtime lists the variables of the script for `ListVars`.
    runtime = read_lines(RUNTIME)
    if not any("fn variable_names(" in line for line in runtime):
        runtime = insert_before(
            runtime,
            "    /// Find the functions and the classes the script defines",
            extract(BRANCH_RUNTIME, "fn variable_names(") + [""],
        )
    write_lines(RUNTIME, runtime)

    # 6. The fake keeps a registry of its own, the way the fake of the
    # branch does, and the cases come over.
    tests = read_lines(TESTS)
    if not any("registry: std::collections::BTreeMap<String, Vec<(String, String, String)>>," in line for line in tests):
        tests = insert_after(
            tests,
            "    files: std::collections::BTreeMap<String, Vec<u8>>,",
            ["    registry: std::collections::BTreeMap<String, Vec<(String, String, String)>>,"],
        )
    for name in TRAIT_METHODS:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = insert_after(
            tests, "impl Machine for Fake", [""] + extract(BRANCH_TESTS, f"fn {name}(")
        )
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    if not any("fn the_variables_are_listed_in_the_output" in line for line in tests):
        tests = tests + THE_LIST_CASE.split("\n")
    # The case of the master that pinned the refusals of the shape of the
    # runtime loses the three names the family carries: `Persistent`,
    # `ListLines` and `ListVars` answer now.
    start = find_fn(
        tests, "fn each_name_of_the_shape_of_the_runtime_refuses_with_the_reason"
    )
    if start is not None:
        first, last = block_of(tests, start)
        block = tests[first:last]
        for gone in ('"Persistent",', '"ListLines",', '"ListVars",'):
            if not any(gone in line for line in block):
                continue
            index = next(i for i, line in enumerate(block) if gone in line)
            # The entry is the tuple: the "(" before the name and the "),"
            # that closes it.
            open_at = index - 1
            while block[open_at].strip() != "(":
                open_at -= 1
            close_at = index
            while block[close_at].strip() != "),":
                close_at += 1
            block = block[:open_at] + block[close_at + 1 :]
        block = [
            line.replace(
                "    // The fourteen names of the shape shared one reason, and that reason",
                "    // The names of the shape shared one reason, and that reason hid a",
            )
            .replace(
                "    // hid fourteen different things.",
                "    // different thing per name. `Persistent`, `ListLines` and `ListVars`\n"
                "    // left the list when the registry and the log trio were carried on\n"
                "    // 2026-11-11; `Pause` and `Suspend` are not here because this tree\n"
                "    // carries them with values, where the branch refuses them.",
            )
            for line in block
        ]
        tests = tests[:first] + block + tests[last:]
    write_lines(TESTS, tests)

    # 7. The case of the runner: eight refusals become eight values.
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
            runner[index] = '    assert_eq!(report.number("values"), 808);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 25);'
            break
    write_lines(RUNNER, runner)

    print("the registry of the machine and the log trio are ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
