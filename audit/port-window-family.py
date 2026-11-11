#!/usr/bin/python3
"""Port the family of the windows of the machine from the branch.

The third family of the port (`docs/two-lines.md` §5.2): the windows. The
`Win*` names of v2 move, show and close the windows of the session, and
the tree of the owner refused every one of them with the blanket reason of
the names of Windows.

The port is:

- The three answers the family needs: `WindowRef`, `WindowGeometry` and
  `WaitFor`.
- The seven methods of the traits that the family needs -- four of the
  `Windows` trait and three of the `Screen` one -- and the seven answers of
  the box that carry them.
- The forty-one handlers of the family, the helpers they call, and the
  rows of the built-in table that answer to them.
- The cases of the family are the ones of the branch.

The script reads the branch and writes the tree of the owner. It refuses to
write anything when an anchor is not where it says it is, and it can be run
twice.

    python audit/port-window-family.py
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent
BRANCH = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk\.agents\worktrees\audit-ahk-x11-parity"
)

TRAIT = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
BUILTINS = ROOT / "crates" / "ahk" / "src" / "builtins.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
WAYLAND = ROOT / "crates" / "ahk-box" / "src" / "wayland.rs"
RUNTIME = ROOT / "crates" / "ahk" / "src" / "runtime.rs"
BEHAVIOUR = ROOT / "crates" / "ahk" / "src" / "behavior.rs"
RUNNER = ROOT / "crates" / "ahk-cli" / "tests" / "runner.rs"

BRANCH_TRAIT = BRANCH / "crates" / "ahk" / "src" / "desktop.rs"
BRANCH_BUILTINS = BRANCH / "crates" / "ahk" / "src" / "builtins.rs"
BRANCH_BOX = BRANCH / "crates" / "ahk-box" / "src" / "desktop.rs"
BRANCH_WAYLAND = BRANCH / "crates" / "ahk-box" / "src" / "wayland.rs"
BRANCH_RUNTIME = BRANCH / "crates" / "ahk" / "src" / "runtime.rs"

# The answers the family needs.
THE_TYPES = [
    ("WindowRef", "struct"),
    ("WindowGeometry", "struct"),
    ("WaitFor", "enum"),
]

# The methods of the `Windows` trait that the family needs.
THE_WINDOWS = [
    "matching_windows",
    "window_geometry",
    "window_min_max",
    "window_wait",
]

# The methods of the `Screen` trait that the family needs.
THE_SCREEN = [
    "screen_scale",
    "tooltip_show",
    "window_move",
    "gui_controls",
    "gui_all_text",
    "gui_opacity",
    "gui_paste",
]

# The hooks of the runtime that the family needs: the window that was asked
# for last, so that `WinGetID` and `WinGetText` with no title answer about
# it.
THE_HOOKS = ["remember_the_last_window", "the_last_window"]

# The handlers of the family. The names that the tree of the owner answers
# already -- `WinGetID`, `WinGetList`, `WinGetCount` and `WinGetClass`,
# which one handler of the tree answers with the variant of the row -- are
# not in the list: the port does not take a name away from the tree of the
# owner to give it back in another shape.
THE_HANDLERS = [
    "win_activate",
    "win_activate_bottom",
    "win_close",
    "win_get_client_pos",
    "win_get_controls",
    "win_get_controls_hwnd",
    "win_get_min_max",
    "win_get_pid",
    "win_get_pos",
    "win_get_process",
    "win_get_text",
    "win_get_transparent",
    "win_hide",
    "win_kill",
    "win_maximize",
    "win_minimize",
    "win_minimize_all",
    "win_minimize_all_undo",
    "win_move",
    "win_move_bottom",
    "win_move_top",
    "win_redraw",
    "win_restore",
    "win_set_always_on_top",
    "win_set_enabled",
    "win_set_region",
    "win_set_title",
    "win_set_transparent",
    "win_show",
    "win_style",
    "win_trans_color",
    "win_wait",
    "win_wait_active",
    "win_wait_close",
    "win_wait_not_active",
    "tooltip",
]

# The helpers of the family that the handlers call.
THE_HELPERS = [
    "window_target",
    "window_geometry_into",
    "the_state_of_the_window",
    "the_close_of_the_window",
    "the_visibility_of_the_window",
    "window_wait_into",
    "window_wait_timeout",
]

# The bound of the wait of the family.
THE_CONSTANTS = ["MAX_WINDOW_WAIT_MS"]

# The four questions that the compositor of the box answers, and the
# helpers they call.
THE_BOX_WAYLAND = [
    "wayland_matching",
    "wayland_geometry",
    "wayland_min_max",
    "wayland_wait",
    "wayland_monitors",
]

# The helpers that only the block of Linux has.
THE_BOX_LINUX = ["the_window_condition"]

# The functions of the box that are not methods of a trait.
THE_BOX_FUNCTIONS = ["pause_before_the_next_look"]

# The shape of two calls is not the same in the two trees: the tree of the
# owner asks for the process by `Option<&str>` -- the nothing is the
# process of the script itself -- and the branch asks for a `&str`.
THE_ADAPTATIONS = [
    (".process_name(&number)", ".process_name(Some(&number))"),
    (".process_path(&number)", ".process_path(Some(&number))"),
]

# The names of the family in the table.
THE_NAMES = ["Win", "ToolTip"]


def a_block(lines, name, kind="fn", single=False, skip=0):
    """The block of a function or a method, with the comment above it."""
    start = None
    seen = 0
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?%s\s+%s\b" % (kind, re.escape(name)), line):
            if seen == skip:
                start = index
                break
            seen += 1
    if start is None:
        return None
    first = start
    while first > 0 and (
        lines[first - 1].strip().startswith("///")
        or lines[first - 1].strip().startswith("#[")
    ):
        first -= 1
    if single:
        return (first, start + 1)
    opened = start
    while opened < len(lines) and "{" not in lines[opened]:
        opened += 1
    if opened >= len(lines):
        return None
    depth = 0
    begun = False
    end = None
    for index in range(opened, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            end = index + 1
            break
    if end is None:
        return None
    return (first, end)


def the_line_of(lines, pattern, what):
    for index, line in enumerate(lines):
        if re.search(pattern, line.rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


def the_closing_brace(lines, opened_at):
    """The line of the brace that closes the block opened at a line."""
    depth = 0
    begun = False
    for index in range(opened_at, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            return index
    return None


def has_marker(lines, marker):
    return any(marker in line for line in lines)


def the_rows(lines, prefixes):
    """The rows of the built-in table whose name starts with a prefix."""
    rows = {}
    current = None
    for index, line in enumerate(lines):
        match = re.match(r"\s*name: \"([A-Za-z0-9_]+)\",\s*$", line)
        if match:
            current = match.group(1)
            rows[current] = index
            continue
        if current is None:
            continue
        if re.match(r"\s*\},?\s*$", line):
            current = None
    return {
        name: index
        for name, index in rows.items()
        if any(name.startswith(prefix) for prefix in prefixes)
    }


def main():
    trait = read_lines(TRAIT)
    builtins = read_lines(BUILTINS)
    box = read_lines(BOX)
    branch_trait = read_lines(BRANCH_TRAIT)
    branch_builtins = read_lines(BRANCH_BUILTINS)
    branch_box = read_lines(BRANCH_BOX)

    # 1. The three answers the family needs.
    for name, kind in THE_TYPES:
        if a_block(trait, name, kind) is not None:
            # The tree of the owner has it already.
            continue
        found = a_block(branch_trait, name, kind)
        if found is None:
            raise SystemExit("the answer %s is not in the branch" % name)
        at = the_line_of(trait, r"^pub trait Windows \{$", "the trait of the windows")
        trait = trait[:at] + branch_trait[found[0]:found[1]] + [""] + trait[at:]

    # 2. The seven methods of the traits.
    for name in THE_WINDOWS:
        if has_marker(trait, "fn %s(" % name):
            continue
        found = a_block(branch_trait, name)
        if found is None:
            raise SystemExit("the method %s is not in the branch" % name)
        at = the_line_of(trait, r"^pub trait Windows \{$", "the trait of the windows")
        trait = trait[:at + 1] + branch_trait[found[0]:found[1]] + [""] + trait[at + 1:]
    for name in THE_SCREEN:
        if has_marker(trait, "fn %s(" % name):
            continue
        found = a_block(branch_trait, name)
        if found is None:
            raise SystemExit("the method %s is not in the branch" % name)
        at = the_line_of(trait, r"^pub trait Screen \{$", "the trait of the screen")
        trait = trait[:at + 1] + branch_trait[found[0]:found[1]] + [""] + trait[at + 1:]

    # 3. The seven answers of the box.
    if not has_marker(box, "fn matching_windows("):
        block = []
        for name in THE_WINDOWS:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the answer of the box %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(
            box, r"^impl ahk::desktop::Windows for LabDesktop \{$", "the box"
        )
        box = box[:at + 1] + block + box[at + 1:]
    if not has_marker(box, "fn screen_scale("):
        block = []
        for name in THE_SCREEN:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the answer of the box %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(
            box, r"^impl ahk::desktop::Screen for LabDesktop \{$", "the box"
        )
        box = box[:at + 1] + block + box[at + 1:]

    # 3b. The tooltip has a helper of its own, and the box keeps it.
    if not has_marker(box, "tooltip: Option<crate::gui::TheWindow>,"):
        at = None
        for index, line in enumerate(branch_box):
            if line.strip().startswith("tooltip: Option<crate::gui::TheWindow>,"):
                at = index
                break
        if at is None:
            raise SystemExit("the field of the tooltip is not in the branch")
        first = at
        while first > 0 and branch_box[first - 1].strip().startswith("///"):
            first -= 1
        opened = the_line_of(box, r"^pub struct LabDesktop \{$", "the desktop")
        closed = the_closing_brace(box, opened)
        box = box[:closed] + branch_box[first:at + 1] + box[closed:]
        # And the constructor makes it.
        at = None
        for index, line in enumerate(branch_box):
            if line.strip().startswith("tooltip: None,"):
                at = index
                break
        if at is None:
            raise SystemExit("the tooltip of the constructor is not in the branch")
        opened = the_line_of(box, r"^\s+Self \{$", "the constructor")
        box = box[:opened + 1] + branch_box[at:at + 1] + box[opened + 1:]

    # 3b. The four questions that the compositor of the box answers. They
    # live in the block of the box that is compiled on Linux, and in the one
    # that is compiled elsewhere: the box answers the same way on every
    # machine, and the answer of a machine that is not Wayland is "unknown".
    if not has_marker(box, "fn wayland_matching("):
        for skip, cfg in ((0, '#[cfg(target_os = "linux")]'), (1, '#[cfg(not(target_os = "linux"))]')):
            block = []
            for name in THE_BOX_WAYLAND + (THE_BOX_LINUX if skip == 0 else []):
                found = a_block(branch_box, name, skip=skip)
                if found is None:
                    raise SystemExit(
                        "the question of the box %s is not in the branch" % name
                    )
                block.append("    // ---- %s" % name)
                block.extend(branch_box[found[0]:found[1]])
                block.append("")
            at = None
            for index, line in enumerate(box):
                if line.rstrip("\r").strip() == "impl LabDesktop {" and index > 0:
                    if box[index - 1].rstrip("\r").strip() == cfg:
                        at = index
                        break
            if at is None:
                raise SystemExit("the block of the box %s is not where it should be" % cfg)
            box = box[:at + 1] + block + box[at + 1:]


    # 3d. The helpers of the box that the questions call, and the import
    # that the wait of the family needs.
    if not has_marker(box, "fn pause_before_the_next_look("):
        block = []
        for name in THE_BOX_FUNCTIONS:
            found = a_block(branch_box, name)
            if found is None:
                raise SystemExit("the function of the box %s is not in the branch" % name)
            block.extend(branch_box[found[0]:found[1]])
            block.append("")
        at = the_line_of(box, r"^impl ahk::desktop::Desktop for LabDesktop", "the box")
        box = box[:at] + block + box[at:]
    if "use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};" not in box:
        for index, line in enumerate(box):
            if line.rstrip("\r").startswith("use std::time::{"):
                box[index] = "use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};"
                break

    # 4. The forty-one handlers of the family, and the helpers they call.
    if not has_marker(builtins, "fn win_get_pos("):
        block = []
        for name in THE_HANDLERS:
            found = a_block(branch_builtins, name)
            if found is None:
                raise SystemExit("the handler %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        for name in THE_HELPERS:
            if a_block(builtins, name) is not None:
                # The tree of the owner has it already.
                continue
            found = a_block(branch_builtins, name)
            if found is None:
                raise SystemExit("the helper %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        for name in THE_CONSTANTS:
            if a_block(builtins, name, "const", True) is not None:
                continue
            found = a_block(branch_builtins, name, "const", True)
            if found is None:
                raise SystemExit("the bound %s is not in the branch" % name)
            block.append("// ---- %s" % name)
            block.extend(branch_builtins[found[0]:found[1]])
            block.append("")
        at = None
        for index, line in enumerate(builtins):
            if re.match(r"^#\[cfg\(test\)\]$", line.rstrip("\r")):
                at = index
                break
        if at is None:
            builtins = builtins + [""] + block
        else:
            builtins = builtins[:at] + block + builtins[at:]

    # 5. The rows of the table: the handler of the family is the one of the
    # branch, and the variant with it.
    branch_rows = the_rows(branch_builtins, THE_NAMES)
    here_rows = the_rows(builtins, THE_NAMES)
    places = []
    for name, at in branch_rows.items():
        if name not in here_rows:
            raise SystemExit("the row of %s is not in the tree of the owner" % name)
        start = here_rows[name]
        end = start
        while not re.match(r"^\s*\},?\s*$", builtins[end]):
            end += 1
        # The row that the tree of the owner answers is not taken away from
        # it: the port gives a name a handler only when the tree of the owner
        # refused it.
        if "handler: not_carried" not in "\n".join(builtins[start:end]):
            continue
        bstart = branch_rows[name]
        bend = bstart
        while not re.match(r"^\s*\},?\s*$", branch_builtins[bend]):
            bend += 1
        wanted = branch_builtins[bstart + 1:bend]
        if wanted != builtins[start + 1:end]:
            places.append((start, end, wanted))
    for start, end, wanted in sorted(places, key=lambda place: place[0], reverse=True):
        builtins[start + 1:end] = wanted

    # 5b. The two calls whose shape is not the same in the two trees.
    for index, line in enumerate(builtins):
        for old, new in THE_ADAPTATIONS:
            if old in line:
                builtins[index] = line.replace(old, new)

    # 5b2. The id of a window: the tree of the owner answers the ids of
    # the windows it lists as whole numbers, and the branch answers the
    # identifier of the compositor, which is a text. The two are reconciled
    # by the fake of the suite, which answers the ids of the tree of the
    # owner -- so nothing is changed here.

    # 5c. The case of the behaviour of the module: the name it refused is
    # carried now, and the name that is still refused is another.
    behaviour = read_lines(BEHAVIOUR)
    old = [
        "    fn a_refused_name_answers_with_its_reason() {",
        "        let answer = the_answer(\"WinActivate\");",
    ]
    new = [
        "    fn a_refused_name_answers_with_its_reason() {",
        "        // `WinActivate` was the example until the family of the windows was",
        "        // carried on 2026-11-11: it answers with a value now, and the name",
        "        // that is still refused with the blanket reason is `BlockInput`.",
        "        let answer = the_answer(\"BlockInput\");",
    ]
    for index in range(len(behaviour) - len(old) + 1):
        if [line.rstrip("\r") for line in behaviour[index:index + len(old)]] == old:
            behaviour[index:index + len(old)] = new
            write_lines(BEHAVIOUR, behaviour)
            break

    # 5d. The case of the runner reads the numbers of the surface, and they
    # move with the family. The numbers of the v1 are the ones of the other
    # case, and they do not move.
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
            runner[index] = '    assert_eq!(report.number("values"), 747);'
            break
    for index in range(start, len(runner)):
        if 'report.number("refusals")' in runner[index]:
            runner[index] = '    assert_eq!(report.number("refusals"), 86);'
            break
    write_lines(RUNNER, runner)

    write_lines(TRAIT, trait)
    write_lines(BUILTINS, builtins)
    write_lines(BOX, box)

    # 6. The hooks of the runtime.
    runtime = read_lines(RUNTIME)
    branch_runtime = read_lines(BRANCH_RUNTIME)
    if not has_marker(runtime, "pub fn remember_the_last_window("):
        block = []
        for name in THE_HOOKS:
            found = a_block(branch_runtime, name)
            if found is None:
                raise SystemExit("the hook %s is not in the branch" % name)
            block.extend(branch_runtime[found[0]:found[1]])
            block.append("")
        opened = the_line_of(runtime, r"^impl<'a> Runtime<'a> \{$", "the runtime")
        closed = the_closing_brace(runtime, opened)
        runtime = runtime[:closed] + block + runtime[closed:]
        # The field of the window that was asked for last.
        at = None
        for index, line in enumerate(branch_runtime):
            if re.match(r"\s+last_window: Option<String>,", line):
                at = index
                break
        if at is None:
            raise SystemExit("the field of the last window is not in the branch")
        first = at
        while first > 0 and branch_runtime[first - 1].strip().startswith("///"):
            first -= 1
        opened = the_line_of(runtime, r"^pub struct Runtime<'a> \{$", "the runtime")
        closed = the_closing_brace(runtime, opened)
        runtime = runtime[:closed] + branch_runtime[first:at + 1] + runtime[closed:]
        # And the two constructors make it.
        at = 0
        while True:
            found = None
            for index in range(at, len(runtime)):
                if re.match(r"^\s+exit_callbacks: Vec::new\(\),\s*$", runtime[index].rstrip("\r")):
                    found = index
                    break
            if found is None:
                break
            runtime.insert(found + 1, "            last_window: None,")
            at = found + 3
        write_lines(RUNTIME, runtime)

    print("the family of the windows of the machine is ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())






