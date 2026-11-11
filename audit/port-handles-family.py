#!/usr/bin/python3
"""Port the family of the handles, the messages and the callbacks.

The ninth family of the port (`docs/two-lines.md` §5.2): eight names.

- `GuiFromHwnd` and `GuiCtrlFromHwnd` are the inverse of `Hwnd`: the
  number this runtime mints for an object is looked up in the identity
  table, and the control lookup also walks the controls the helper drew;
- `MenuFromHandle` is refused with a reason of its own: a menu handle is
  a Win32 HMENU, and this runtime has none -- the menus are objects the
  script holds and the helper draws;
- `OnMessage` registers the handler of a message number in the runtime,
  and the drain of the gui events answers a `message:<number>` event of
  the helper with it; `PostMessage` and `SendMessage` refuse every window
  number with "Target window not found.", the way the binary of v2.0.19
  was measured to do on 2026-11-11 -- the messages go to the windows of
  other programs, and this runtime has no window procedure;
- `CallbackCreate` mints the identity number of a value (the same table
  `ObjPtr` reads) and `CallbackFree` refuses: a callback of v2 is a
  native function pointer, which this runtime does not hand out.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-handles-family.py
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
    ("CallbackCreate", "callback_create"),
    ("CallbackFree", "callback_free"),
    ("GuiCtrlFromHwnd", "gui_ctrl_from_hwnd"),
    ("GuiFromHwnd", "gui_from_hwnd"),
    ("MenuFromHandle", "menu_from_handle"),
    ("OnMessage", "on_message"),
    ("PostMessage", "post_message"),
    ("SendMessage", "send_message"),
]

HANDLERS = [
    "the_gui_object_of_a_handle",
    "gui_from_hwnd",
    "gui_ctrl_from_hwnd",
    "menu_from_handle",
    "callback_create",
    "callback_free",
    "on_message",
    "post_message",
    "send_message",
]

RUNTIME_METHODS = [
    "the_object_of",
    "remember_a_message_handler",
    "the_message_handler",
]

CASES = [
    "the_messages_of_a_window_procedure_are_registered_and_the_rest_is_not_carried",
    "the_hwnd_of_a_window_and_of_a_control_round_trip_through_the_lookup",
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


def the_message_hunk(branch):
    """The `if name == "message"` arm of the event drain, with its comment."""
    start = None
    for index, line in enumerate(branch):
        if "// A message event carries the number of the message" in line:
            start = index
            break
    if start is None:
        raise SystemExit("the branch has no message arm in the event drain")
    for index in range(start, len(branch)):
        if branch[index].strip() == "}":
            return branch[start : index + 1]
    raise SystemExit("the message arm does not end")


def main():
    # 1. The runtime: the field of the message handlers, its two inits, the
    # three methods, and the arm of the event drain that answers a message.
    runtime = read_lines(RUNTIME)
    branch_runtime = read_lines(BRANCH_RUNTIME)
    if not any("message_handlers: BTreeMap<i64, String>," in line for line in runtime):
        start = None
        for index, line in enumerate(branch_runtime):
            if "message_handlers: BTreeMap<i64, String>," in line:
                start = index
                break
        first = start
        while first > 0 and branch_runtime[first - 1].lstrip().startswith("///"):
            first -= 1
        runtime = insert_before(
            runtime, "    last_window: Option<String>,", branch_runtime[first : start + 1]
        )
        out = []
        for line in runtime:
            out.append(line)
            if line.strip() == "pointers: Vec::new()," and not any(
                "message_handlers: BTreeMap::new()," in held for held in out[-3:]
            ):
                indent = line[: len(line) - len(line.lstrip())]
                out.append(f"{indent}message_handlers: BTreeMap::new(),")
        runtime = out
    for name in RUNTIME_METHODS:
        if any(f"fn {name}(" in line for line in runtime):
            continue
        runtime = insert_before(
            runtime,
            "    /// The windows of the script that have been remembered.",
            extract(BRANCH_RUNTIME, f"fn {name}(") + [""],
        )
    if not any('name == "message"' in line for line in runtime):
        runtime = insert_before(
            runtime,
            "            // A menu event carries the name of the item that was chosen: the",
            the_message_hunk(branch_runtime),
        )
    write_lines(RUNTIME, runtime)

    # 2. The table: the eight handlers, the helper, and the eight rows.
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

    # 3. The cases of the family.
    tests = read_lines(TESTS)
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    # 4. The case of the runner reads the numbers of the surface, and they
    # move with the family: eight refusals become eight values.
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
            runner[index] = '    assert_eq!(report.number("values"), 775);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 58);'
            break
    write_lines(RUNNER, runner)

    print("the family of the handles, the messages and the callbacks is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
