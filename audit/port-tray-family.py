#!/usr/bin/python3
"""Port the family of the tray and the watcher of the clipboard.

The fifteenth and last family of the port (`docs/two-lines.md` §5.2): six
names -- `TraySetIcon`, `TrayTip`, `OnClipboardChange` and the three
variables `A_IconFile`, `A_IconNumber` and `A_TrayMenu`.

- `TraySetIcon` loads the icon before it sets it -- an icon that cannot
  be loaded is "Can't load icon." and the icon of the script stays as it
  was -- and keeps the canonical path of the file and the number of the
  icon in the runtime, which is what `A_IconFile` and `A_IconNumber`
  answer. The tray of the session shows the icon when the session has
  one: on the box a helper of its own (`ahk-tray.py`) registers a
  StatusNotifierItem with the watcher of the compositor, and a desktop
  with no tray is a NOTE in the report, not a failure.
- `A_TrayMenu` answers a Menu object -- the same one for the life of the
  script -- whose Default is "&Open" before any item is added.
- `TrayTip` is a notification of the session: the box answers with
  `notify-send`, and the name answers the empty text.
- `OnClipboardChange` registers a function and the runtime calls it with
  1 when the script itself writes `A_Clipboard` -- the only change this
  runtime can see, since it has no watcher of another program's
  clipboard. A second argument of zero removes the function.

The script is idempotent: it refuses to insert what is already there, and
it only replaces the rows whose variant is the marker of a name that is
not carried.

    python audit/port-tray-family.py
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
VARIABLES = ROOT / "crates" / "ahk" / "src" / "variables.rs"
DIALOGS = ROOT / "crates" / "ahk" / "src" / "dialogs.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
BOX_FILES = ROOT / "crates" / "ahk-box" / "src" / "files.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
VAR_TESTS = ROOT / "crates" / "ahk" / "tests" / "variables.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_DESKTOP = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_RUNTIME = BRANCH / "crates" / "ahk" / "src" / "runtime.rs"
BRANCH_VARIABLES = BRANCH / "crates" / "ahk" / "src" / "variables.rs"
BRANCH_DIALOGS = BRANCH / "crates" / "ahk" / "src" / "dialogs.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"
BRANCH_BOX_FILES = BRANCH / "crates" / "ahk-box" / "src" / "files.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"
BRANCH_VAR_TESTS = BRANCH / "crates" / "ahk" / "tests" / "variables.rs"
BRANCH_TRAY_HELPER = BRANCH / "crates" / "ahk-box" / "helper" / "ahk-tray.py"
PORT_TRAY_HELPER = ROOT / "crates" / "ahk-box" / "helper" / "ahk-tray.py"

ROWS = [
    ("OnClipboardChange", "on_clipboard_change"),
    ("TraySetIcon", "tray_set_icon"),
    ("TrayTip", "tray_tip"),
]

RUNTIME_METHODS = [
    "the_callback_with_one_argument",
    "call_a_function_for_a_built_in",
    "the_tray_menu",
    "set_the_icon_of_the_script",
    "the_icon_of_the_script",
    "set_the_clipboard_watcher",
    "the_clipboard_watcher",
]

FIELDS = [
    ("clipboard_watcher: Option<String>,", "message_handlers: BTreeMap<i64, String>,"),
    ("icon_of_the_script: Option<(String, i64)>,", "clipboard_watcher: Option<String>,"),
    ("tray_menu: Option<Value>,", "icon_of_the_script: Option<(String, i64)>,"),
]

INITS = ["clipboard_watcher: None,", "icon_of_the_script: None,", "tray_menu: None,"]

BOX_FREE = [
    "the_events_of_the_window",
    "the_answers_of_the_tray",
    "the_tray_took",
    "the_helper_of_the_tray",
]

CASES = [
    "the_icon_of_the_script_is_the_one_that_was_put_in_it",
    "the_tray_tip_tells_the_user_something",
    "the_tray_tip_answers_nothing_the_script_reads",
    "a_clipboard_change_calls_the_function_with_one",
    "a_clipboard_change_that_is_removed_is_not_called",
    "a_clipboard_change_wants_a_function_not_a_text",
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


def field_block(lines, wanted):
    for index, line in enumerate(lines):
        if wanted in line:
            first = index
            while first > 0 and (
                lines[first - 1].lstrip().startswith("///")
                or lines[first - 1].lstrip().startswith("#[")
            ):
                first -= 1
            last = index
            # A field may span lines, like the tuple of the tray.
            depth = lines[index].count("(") - lines[index].count(")")
            while depth > 0:
                last += 1
                depth += lines[last].count("(") - lines[last].count(")")
            return lines[first : last + 1]
    raise SystemExit(f"the branch has no field {wanted}")


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
    return lines[:first] + lines[last:], True


def inherent_insert(box, block):
    """Put a block inside the `impl LabDesktop` that ends above the comment
    of the keyboard codes."""
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
    return box[:close] + [""] + block + box[close:]


def main():
    # 1. The helper of the tray comes over whole.
    if not PORT_TRAY_HELPER.exists():
        shutil.copyfile(BRANCH_TRAY_HELPER, PORT_TRAY_HELPER)

    # 2. The trait of the language grows the three methods, with the
    # defaults that refuse.
    desktop = read_lines(DESKTOP)
    for name, trait in (
        ("canonical_path", "pub trait Files"),
        ("tray_set_icon", "pub trait Machine"),
        ("notify", "pub trait Dialogs"),
    ):
        if any(f"fn {name}(" in line for line in desktop):
            continue
        desktop = insert_after(
            desktop, trait, [""] + extract(BRANCH_DESKTOP, f"fn {name}(")
        )
    write_lines(DESKTOP, desktop)

    # 3. The runtime: the three fields, their inits and the seven methods.
    runtime = read_lines(RUNTIME)
    branch_runtime = read_lines(BRANCH_RUNTIME)
    for wanted, anchor in FIELDS:
        if not any(wanted in line for line in runtime):
            runtime = insert_after(
                runtime, anchor, field_block(branch_runtime, wanted)
            )
    out = []
    for line in runtime:
        out.append(line)
        if line.strip() == "message_handlers: BTreeMap::new(),":
            indent = line[: len(line) - len(line.lstrip())]
            for init in INITS:
                if not any(init in held for held in runtime):
                    out.append(f"{indent}{init}")
    runtime = out
    for name in RUNTIME_METHODS:
        if any(f"fn {name}(" in line for line in runtime):
            continue
        runtime = insert_before(
            runtime,
            "    /// Find the functions and the classes the script defines",
            extract(BRANCH_RUNTIME, f"fn {name}(") + [""],
        )
    # The value of a function carries the flag of the rest and counts the
    # parameters before it, the way the branch measured on the binary on
    # 2026-11-11 -- without it the fat arrow of `(*) => 0` is not the
    # callback a menu takes. The `PARAMETERS` property of this tree stays:
    # it is the record of the by-reference parameters, which the branch
    # does not have.
    start = find_fn(runtime, "fn func_value(")
    if start is not None:
        first, last = block_of(runtime, start)
        block = runtime[first:last]
        joined = "\n".join(block)
        if '"maxparams",\n        Value::Int(function.parameters.len() as i64),' in joined:
            old = [
                '    put(',
                '        &mut properties,',
                '        "maxparams",',
                '        Value::Int(function.parameters.len() as i64),',
                '    );',
                '    put(&mut properties, "isbuiltin", Value::Int(0));',
                '    put(&mut properties, "isvariadic", Value::Int(0));',
            ]
            new = [
                '    // Measured on the binary of v2.0.19 on 2026-11-11: the most of a',
                '    // function with a rest are the parameters that come BEFORE it -- a',
                '    // function of (a, b, c, Rest*) has three of them and not four -- and',
                '    // the flag of the rest is one.',
                '    let variadic = function.parameters.iter().any(|parameter| parameter.variadic);',
                '    let most = function',
                '        .parameters',
                '        .iter()',
                '        .filter(|parameter| !parameter.variadic)',
                '        .count();',
                '    put(',
                '        &mut properties,',
                '        "maxparams",',
                '        Value::Int(most as i64),',
                '    );',
                '    put(&mut properties, "isbuiltin", Value::Int(0));',
                '    put(&mut properties, "isvariadic", Value::Int(if variadic { 1 } else { 0 }));',
            ]
            at = None
            for index in range(len(block) - len(old) + 1):
                if all(block[index + offset].rstrip("\r") == old[offset] for offset in range(len(old))):
                    at = index
                    break
            if at is None:
                raise SystemExit("the func_value of the port is not where it should be")
            block = block[:at] + [line + "\n" for line in new] + block[at + len(old):]
            runtime = runtime[:first] + block + runtime[last:]
    write_lines(RUNTIME, runtime)

    # 4. The dialogs module carries the tray tip.
    dialogs = read_lines(DIALOGS)
    if not any("fn tray_tip(" in line for line in dialogs):
        dialogs = dialogs + [""] + extract(BRANCH_DIALOGS, "fn tray_tip(")
    write_lines(DIALOGS, dialogs)

    # 5. The table: the import of the tray tip, the two handlers and the
    # three rows.
    builtins = read_lines(BUILTINS)
    builtins = [
        line.replace(
            "pub(crate) use crate::dialogs::{dir_select, file_select, input_box};",
            "pub(crate) use crate::dialogs::{dir_select, file_select, input_box, tray_tip};",
        )
        for line in builtins
    ]
    for name in ("tray_set_icon", "on_clipboard_change"):
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

    # 6. The variables: the three arms of the tray, the watcher in the
    # write of the clipboard, and the refusal that goes away.
    variables = read_lines(VARIABLES)
    if not any('"traymenu" => Ok(' in line for line in variables):
        arm = extract_region(
            BRANCH_VARIABLES,
            '"iconfile" => Ok(Value::Text(',
            '"traymenu" => Ok(rt.the_tray_menu(0)),',
        )
        variables = insert_after(
            variables, '"iconhidden" => Ok(Value::Int(1)),', arm
        )
    variables, _ = remove_fn(variables, "fn put_on_the_clipboard(")
    variables = insert_before(
        variables,
        "/// Whether the name is a built-in variable of v2.",
        extract(BRANCH_VARIABLES, "fn put_on_the_clipboard(") + [""],
    )
    variables = [
        line
        for line in variables
        if '"iconfile" | "iconnumber" | "traymenu" => NO_TRAY_ICON,' not in line
    ]
    if not any("NO_TRAY_ICON" in line and "const NO_TRAY_ICON" not in line for line in variables):
        out = []
        for line in variables:
            if "const NO_TRAY_ICON" in line:
                while out and out[-1].lstrip().startswith("///"):
                    out.pop()
                continue
            out.append(line)
        variables = out
    write_lines(VARIABLES, variables)

    # 7. The box: the field of the tray, the helper calls, the free
    # functions and the two trait impls.
    box = read_lines(BOX)
    branch_box = read_lines(BRANCH_BOX)
    if not any("tray: Option<(" in line for line in box):
        box = insert_after(box, "raven: Option<crate::raven::Guests>,", field_block(branch_box, "tray: Option<("))
    if not any("tray: None," in line for line in box):
        box = insert_after(box, "raven: None,", field_block(branch_box, "tray: None,"))
    if not any("fn tray_set_icon(" in line for line in box):
        block = extract(BRANCH_BOX, "fn tray_set_icon(") + [""]
        block = block + extract(BRANCH_BOX, "fn the_answer_of_the_tray(") + [""]
        box = inherent_insert(box, block)
    for name in BOX_FREE:
        if any(f"fn {name}(" in line for line in box):
            continue
        box = box + [""] + extract(BRANCH_BOX, f"fn {name}(")
    if sum(1 for line in box if "fn tray_set_icon(" in line) < 2:
        box = insert_after(
            box,
            "impl ahk::desktop::Machine for LabDesktop",
            [""]
            + extract(
                BRANCH_BOX,
                "fn tray_set_icon(",
                after_anchor="impl ahk::desktop::Machine for LabDesktop",
            ),
        )
    if not any("fn notify(" in line for line in box):
        box = insert_after(
            box,
            "impl ahk::desktop::Dialogs for LabDesktop",
            [""] + extract(BRANCH_BOX, "fn notify(", after_anchor="impl ahk::desktop::Dialogs for LabDesktop"),
        )
    write_lines(BOX, box)

    # 8. The files of the box: the canonical form of a path.
    box_files = read_lines(BOX_FILES)
    if not any("fn canonical_path(" in line for line in box_files):
        box_files = insert_after(
            box_files,
            "impl ahk::desktop::Files for LabDesktop",
            [""] + extract(BRANCH_BOX_FILES, "fn canonical_path("),
        )
    if not any("pub fn canonical_path(" in line for line in box_files):
        box_files = box_files + [""] + extract(BRANCH_BOX_FILES, "pub fn canonical_path(")
    write_lines(BOX_FILES, box_files)

    # 9. The tests: the fake notifies, and the seven cases come over.
    tests = read_lines(TESTS)
    if not any("fn notify(" in line for line in tests):
        tests = insert_after(
            tests, "impl Dialogs for Fake", [""] + extract(BRANCH_TESTS, "fn notify(")
        )
    for name in CASES:
        if any(f"fn {name}(" in line for line in tests):
            continue
        tests = tests + [""] + extract(BRANCH_TESTS, f"fn {name}(")
    write_lines(TESTS, tests)

    var_tests = read_lines(VAR_TESTS)
    # The coherence case of the refusals loses `A_IconFile` too: the tray
    # answers it now, and the loop is the one of the branch.
    start = find_fn(
        var_tests, "fn the_reason_of_a_name_is_the_same_in_the_read_and_in_the_refusal("
    )
    if start is not None:
        first, last = block_of(var_tests, start)
        block = [
            line
            for line in var_tests[first:last]
            if '"A_IconFile",' not in line
            and "`A_IconFile` stays here" not in line
            and "answers it, and the loop of the branch lost the name there." not in line
        ]
        var_tests = var_tests[:first] + block + var_tests[last:]
    # The case of the tray variables reads values, and the module of the
    # port did not name the type.
    var_tests = [
        line.replace(
            "use ahk::runtime::{Report, Runtime};",
            "use ahk::runtime::{Report, Runtime, Value};",
        )
        for line in var_tests
    ]
    if not any("fn the_tray_variables_answer" in line for line in var_tests):
        var_tests = var_tests + [""] + extract(
            BRANCH_VAR_TESTS, "fn the_tray_variables_answer"
        )
    write_lines(VAR_TESTS, var_tests)

    # 10. The case of the runner: six refusals become six values, and the
    # port passes the branch -- `Pause` and `Suspend` answer here.
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
            runner[index] = '    assert_eq!(report.number("values"), 817);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 16);'
            break
    # The tray variables of the v1 answer too, and the count of the v1
    # becomes the one the branch pins.
    for index, line in enumerate(runner):
        if 'report.number("values"), 306' in line:
            runner[index] = '    assert_eq!(report.number("values"), 308);'
            break
    write_lines(RUNNER, runner)

    print("the family of the tray and the watcher of the clipboard is ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
