#!/usr/bin/python3
"""Port the cases of the family of the windows of the machine.

The fake of the suite answers the four questions of the compositor, so the
family of the windows has something to answer about, and the cases of the
branch are the record of what it does.

    python audit/port-window-cases.py
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

TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"
BRANCH_TESTS = BRANCH / "crates" / "ahk" / "tests" / "builtins.rs"

# The answers of the fake for the windows of the session.
THE_FAKE = [
    "matching_windows",
    "window_geometry",
    "window_min_max",
    "window_wait",
]

# The cases of the family.
THE_CASES = [
    "the_windows_of_the_session_are_asked_of_the_desktop",
    "a_window_that_is_not_there_is_the_target_error_of_v2",
]


# The answers of the fake for the windows of the session: the windows of
# the tree of the owner, so that the case of the suite -- which is about
# those windows -- asks about the same ones.
THE_FAKE_ANSWERS = """    /// The windows of the session are the ones the fake answers with its
    /// own list: the family of the windows asks the desktop for them, and
    /// the case of the suite is about the same windows.
    fn matching_windows(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        self.calls.push(("windows".into(), title.to_string()));
        Ok(Windows::windows(self)
            .map_err(|reason| reason)?
            .into_iter()
            .filter(|window| title.is_empty() || window.title.contains(title))
            .map(|window| WindowRef {
                id: window.id.to_string(),
                class: window.class,
            })
            .collect())
    }

    /// The fake does not know where a window is.
    fn window_geometry(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<WindowGeometry>, String> {
        Ok(None)
    }

    /// The fake does not know whether a window is maximized.
    fn window_min_max(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<i64>, String> {
        Ok(None)
    }

    /// The wait of the family asks the desktop, and the fake answers with
    /// the windows it has.
    fn window_wait(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
        what: WaitFor,
        _timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        self.calls.push(("wait".into(), title.to_string()));
        let windows = Windows::windows(self).map_err(|reason| reason)?;
        let present = windows
            .iter()
            .any(|window| title.is_empty() || window.title.contains(title));
        match what {
            WaitFor::Present => Ok(present.then(|| title.to_string())),
            WaitFor::Active => Ok(windows
                .iter()
                .find(|window| {
                    window.active && (title.is_empty() || window.title.contains(title))
                })
                .map(|window| window.title.clone())),
            WaitFor::NotActive => Ok((!present).then(|| title.to_string())),
            WaitFor::Close => Ok((!present).then(|| title.to_string())),
        }
    }
"""


def a_block(lines, name, kind="fn"):
    start = None
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?%s\s+%s\b" % (kind, re.escape(name)), line):
            start = index
            break
    if start is None:
        return None
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
    first = start
    while first > 0 and (
        lines[first - 1].strip().startswith("///")
        or lines[first - 1].strip().startswith("#[")
    ):
        first -= 1
    return (first, end)


def the_line_of(lines, pattern, what):
    for index, line in enumerate(lines):
        if re.search(pattern, line.rstrip("\r")):
            return index
    raise SystemExit("the anchor of %s is not where it should be" % what)


def has_marker(lines, marker):
    return any(marker in line for line in lines)


def main():
    tests = read_lines(TESTS)
    branch = read_lines(BRANCH_TESTS)

    # 1. The answers of the fake: they are methods of the `Windows` trait.
    if not has_marker(tests, "fn matching_windows("):
        block = []
        for name in THE_FAKE:
            found = a_block(branch, name)
            if found is None:
                raise SystemExit("the answer of the fake %s is not in the branch" % name)
            block.append("    // ---- %s" % name)
            block.extend(branch[found[0]:found[1]])
            block.append("")
        at = the_line_of(tests, r"^impl Windows for Fake \{$", "the fake")
        at += 1
        tests = tests[:at] + block + tests[at:]

    # 1b. The answers of the fake are the ones of the tree of the owner: the
    # case of the suite is about the windows that the fake already lists.
    # The four answers came from the branch together, and they are replaced
    # together.
    first = a_block(tests, "matching_windows")
    last = a_block(tests, "window_wait")
    if first is not None and last is not None:
        tests[first[0]:last[1]] = THE_FAKE_ANSWERS.split("\n")

    # 2. The three answers of the family are named by the fake.
    if not has_marker(tests, "WindowRef, WindowGeometry, WaitFor"):
        at = the_line_of(tests, r"^    Clipboard, ", "the imports")
        tests[at] = tests[at].replace(
            "Clipboard, ",
            "Clipboard, WindowRef, WindowGeometry, WaitFor, ",
        )

    # 3. The cases of the family: the ones that the tree of the owner has
    # already are the ones of the branch, and the tree of the owner keeps
    # the ones that are about something else.
    for name in THE_CASES:
        found = a_block(tests, name)
        if found is None:
            raise SystemExit("the case %s is not in the tree of the owner" % name)
        wanted = a_block(branch, name)
        if wanted is None:
            continue
        tests[found[0]:found[1]] = branch[wanted[0]:wanted[1]]

    write_lines(TESTS, tests)
    print("the cases of the family of the windows are ported")
    return 0


if __name__ == "__main__":
    sys.exit(main())
