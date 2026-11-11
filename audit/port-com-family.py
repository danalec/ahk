#!/usr/bin/python3
"""Port the family of the COM objects and the pointers of objects.

The eleventh family of the port (`docs/two-lines.md` §5.2): fourteen
names.

- The nine `Com*` names: this platform has no component model, and every
  one of them answers the wording the binary of v2.0.19 was measured to
  throw -- "(0x80070057) The parameter is incorrect." for `ComObjGet`,
  "(0x800401F3) Invalid class string" for `ComObjActive`, the empty text
  for `ComObjType`, and the reasons of their own for the rest. Three of
  them share the `com_object_argument!` macro of the branch.
- The five `Obj*` names: `ObjPtr` and `ObjPtrAddRef` mint the identity
  number of a value -- the same table the handles family reads -- and
  `ObjFromPtr` and `ObjFromPtrAddRef` give the object back. `ObjRelease`
  refuses with the invalid parameter: this runtime keeps no count of
  references to release.

The port also carries the case of its own for the round trip of the
pointers, which the branch measures in the binary but does not pin in a
case.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-com-family.py
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
BEHAVIOR = ROOT / "crates" / "ahk" / "src" / "behavior.rs"

NEW_BEHAVIOR_TEST = '''#[test]
    fn a_refused_name_answers_with_its_reason() {
        // `WinActivate` was the example until the family of the windows was
        // carried on 2026-11-11, `BlockInput` until the family of the small
        // commands, and `ComObjGet` until the family of the COM objects --
        // all on the same day. The name that stays refused with the blanket
        // reason is `Edit`, and it stays one in the branch as well: the
        // script of this runtime is read from the standard input, so it has
        // no file for an editor to open.
        let answer = the_answer("Edit");
        assert!(answer.starts_with("refusal: "), "{answer}");
        // The reason is the one the surface prints.
        assert_eq!(
            answer,
            "refusal: this is a name of Windows, and this desktop is not Windows."
        );
    }'''

BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

ROWS = [
    ("ComCall", "com_call"),
    ("ComObjActive", "com_obj_active"),
    ("ComObjConnect", "com_obj_connect"),
    ("ComObjFlags", "com_obj_flags"),
    ("ComObjFromPtr", "com_obj_from_ptr"),
    ("ComObjGet", "com_obj_get"),
    ("ComObjQuery", "com_obj_query"),
    ("ComObjType", "com_obj_type"),
    ("ComObjValue", "com_obj_value"),
    ("ObjFromPtr", "obj_from_ptr"),
    ("ObjFromPtrAddRef", "obj_from_ptr_add_ref"),
    ("ObjPtr", "obj_ptr"),
    ("ObjPtrAddRef", "obj_ptr_add_ref"),
    ("ObjRelease", "obj_add_ref"),
]

FUNCTIONS = [
    "com_call",
    "com_obj_from_ptr",
    "obj_ptr",
    "obj_ptr_add_ref",
    "obj_from_ptr",
    "obj_from_ptr_add_ref",
    "obj_add_ref",
    "the_kind_of",
]

CASES = [
    "the_com_of_the_machine_answers_the_shapes_the_binary_answers",
    "the_com_group_answers_the_wordings_the_binary_throws",
]

THE_POINTER_CASE = '''
#[test]
fn the_pointers_of_the_objects_round_trip_through_the_table_of_identities() {
    // The number `ObjPtr` mints is the identity the handles family reads,
    // and `ObjFromPtr` gives the object back. `ObjPtrAddRef` wants an
    // object, and `ObjRelease` refuses every call with the invalid
    // parameter: this runtime keeps no count of references. A case of this
    // tree -- the branch measures the wordings in the binary but pins the
    // round trip in no case.
    let report = run(concat!(
        "m := Map(\\"a\\", 1)",
        "\\np := ObjPtr(m)",
        "\\nback := ObjFromPtr(p)",
        "\\nsame := back = m",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "same"), Value::Int(1));
    let report = run("ObjPtrAddRef(\\"x\\")");
    assert_eq!(failure_of(&report), "Expected an object but got a String.");
    let report = run("ObjRelease(1)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ObjRelease is invalid."
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


def the_com_group(branch):
    """The whole COM region: the four functions, the macro and its calls."""
    start = None
    end = None
    for index, line in enumerate(branch):
        if "// -- the COM group" in line:
            start = index
        elif start is not None and "// -- the log of the lines" in line:
            end = index
            break
    if start is None or end is None:
        raise SystemExit("the COM group of the branch is not delimited")
    return branch[start:end]


def main():
    builtins = read_lines(BUILTINS)
    branch = read_lines(BRANCH_BUILTINS)
    if not any("fn com_obj_get(" in line for line in builtins):
        builtins = builtins + [""] + the_com_group(branch)
    for name in FUNCTIONS:
        if any(f"fn {name}(" in line for line in builtins):
            continue
        builtins = builtins + [""] + extract(BRANCH_BUILTINS, f"fn {name}(")
    # The port carried an `obj_add_ref` of its own, of the master, which
    # always named "ObjAddRef": the version of the branch names itself by
    # the variant of the row, and the row of `ObjRelease` carries the one.
    # Without the swap, `ObjRelease(1)` answers as `ObjAddRef`.
    start = find_fn(builtins, "fn obj_add_ref(")
    if start is not None:
        first, last = block_of(builtins, start)
        if not any("ObjRelease" in line for line in builtins[first:last]):
            builtins = (
                builtins[:first]
                + extract(BRANCH_BUILTINS, "fn obj_add_ref(")
                + builtins[last:]
            )
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
    # The case of the master that asked `ObjFromPtr` for the blanket reason
    # dies with the refusal: the branch does not carry it, and the family
    # answers with a reason of its own now.
    tests, _ = remove_fn(tests, "fn the_names_of_com_fail_with_the_reason(")
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    if not any(
        "fn the_pointers_of_the_objects_round_trip" in line for line in tests
    ):
        tests = tests + THE_POINTER_CASE.split("\n")
    write_lines(TESTS, tests)

    # The examples of the blanket refusal move with the family: `ComObjGet`
    # answers now, and the name that stays refused is `Edit` -- in this
    # tree and in the branch alike.
    behavior = read_lines(BEHAVIOR)
    if any('the_answer("ComObjGet")' in line for line in behavior):
        bstart = find_fn(behavior, "fn a_refused_name_answers_with_its_reason(")
        if bstart is None:
            raise SystemExit("the behavior case of the refusal is not there")
        first, last = block_of(behavior, bstart)
        block = NEW_BEHAVIOR_TEST.split("\n")
        block[0] = "    " + block[0]
        behavior = behavior[:first] + block + behavior[last:]
    write_lines(BEHAVIOR, behavior)

    runner = read_lines(RUNNER)
    for index, line in enumerate(runner):
        if '"ComObjGet"' in line:
            runner[index] = line.replace('"ComObjGet"', '"Edit"').replace(
                "`ComObjGet` is still refused.",
                "`ComObjGet` until the family of the\n    // COM objects; `Edit` stays refused.",
            )
    start = None
    for index, line in enumerate(runner):
        if 'report.number("total"), 833' in line:
            start = index
            break
    if start is None:
        raise SystemExit("the case of the runner is not where it should be")
    for index in range(start, len(runner)):
        if 'report.number("values")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("values"), 793);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 40);'
            break
    write_lines(RUNNER, runner)

    print("the family of the COM objects and the pointers is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
