"""The target of the window family of the v2, measured on the binary.

    python scripts/measure-window-targets-2.0.32.py [--out FILE]

**The call that names no window asks about the window that the script found
last.** That is the rule of the v2, and of the v1 as well, and the tree did
not follow it: it kept no window at all, and the names of the family either
answered with the window that was in the foreground of the machine or
refused with "Target window not found.".

The source of the v2.0.32 says it twice, and the source of the v1.1.37.02
says it once:

- `window.cpp:490-494` of the v2.0.32: "User passed no params, so use the
  window most recently found by WinExist()."
- `script2.cpp:10911-10923` of the v1.1.37.02: "Use the \"last found\"
  window."

The instrument asks the official v2.0.32 and the tree the same questions
about the nineteen names of the family, in the two states that matter: the
call that names no window, and the call that follows a `WinExist` that
found a window. It asks the official for a window of its own -- the first
one of the listing that is not the window in the foreground -- and the two
engines are asked about that one.

The binary is the build of the v2.0.32 that answers the errors as text, and
not the copy that is installed in the machine: that one is the v2.1-alpha,
and its answers are not the ones of the v2.0.32. The variable `AHK_V2` says
another binary, and `AHK_V2_FLAGS` says the switches of it.

The report is a table with the name, the answer of the binary and the answer
of the tree, and it is a report and not a change: the tree is changed by
hand, with the measurement in it.
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
import reference_binaries  # noqa: E402

BINARY = reference_binaries.the_v2()
FLAGS = reference_binaries.the_v2_flags(BINARY)
TREE = ROOT / "target" / "release" / "ahk.exe"

# The names of the family, in the order of the table of the reference
# (`crates/ahk/reference/ahk-v2.0.32.surface.tsv`).
FAMILY = [
    "WinGetClass",
    "WinGetClientPos",
    "WinGetControls",
    "WinGetControlsHwnd",
    "WinGetCount",
    "WinGetExStyle",
    "WinGetID",
    "WinGetIDLast",
    "WinGetList",
    "WinGetMinMax",
    "WinGetPID",
    "WinGetPos",
    "WinGetProcessName",
    "WinGetProcessPath",
    "WinGetStyle",
    "WinGetText",
    "WinGetTitle",
    "WinGetTransColor",
    "WinGetTransparent",
]


# The names that answer an array: the length of it is what a text can hold,
# and the items of it are the names of the controls of the window.
ARRAYS = {"WinGetControls", "WinGetControlsHwnd", "WinGetList"}


def the_script(before, expression):
    """A script that answers with the value it got, or with the exception."""
    return (
        "try {\n"
        + (before + "\n" if before else "")
        + '    FileAppend("VALUE: " %s, "*")\n' % expression
        + "} catch as e {\n"
        '    FileAppend("THREW " e.Message, "*")\n'
        "}\n"
    )


def the_answer_of_the_official(script, work):
    path = work / "probe.ahk"
    path.write_text(script, encoding="utf-8")
    try:
        p = subprocess.run(
            [str(BINARY), "/ErrorStdOut"] + FLAGS + [str(path)],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            timeout=20,
            cwd=str(work),
        )
    except subprocess.TimeoutExpired:
        return "waits (a dialog)"
    return (p.stdout or "").strip() or (p.stderr or "").strip() or "(nothing)"


def the_answer_of_the_tree(script, work):
    try:
        p = subprocess.run(
            [str(TREE), "--dialect=v2"],
            input=script,
            capture_output=True,
            text=True,
            timeout=30,
            cwd=str(work),
        )
    except subprocess.TimeoutExpired:
        return "waits"
    try:
        report = json.loads(p.stdout) if p.stdout else {}
    except json.JSONDecodeError:
        return (p.stdout or p.stderr or "").strip()[:200]
    parts = [str(line).strip() for line in (report.get("output") or [])]
    if report.get("error"):
        parts.append("ERROR " + str(report["error"]))
    return " | ".join(parts) if parts else "(nothing)"


def a_window_of_the_machine(work):
    """The title of a window that is not the one in the foreground.

    The window in the foreground is the one that the "A" names, and the
    family asks about another one: the window that the script found has to
    be a window that is not in the front, or the two rules would look the
    same.
    """
    script = (
        "ids := WinGetList()\n"
        'foreground := WinExist("A")\n'
        "for id in ids {\n"
        "    if id = foreground {\n"
        "        continue\n"
        "    }\n"
        "    try {\n"
        "        title := WinGetTitle(id)\n"
        "    } catch {\n"
        "        continue\n"
        "    }\n"
        "    if title != \"\" {\n"
        '        FileAppend(title, "*")\n'
        "        break\n"
        "    }\n"
        "}\n"
    )
    answer = the_answer_of_the_official(script, work)
    return answer.strip() or None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, help="the file of the report")
    args = parser.parse_args()

    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        work = Path(folder)
        target = a_window_of_the_machine(work)
        if not target:
            print("the machine has no window that is not in the foreground.")
            return 1
        title = target.replace('"', '``')
        print("the window that the script finds: [%s]" % target)
        print()

        rows = []
        for name in FAMILY:
            expression = "%s()" % name
            if name in ARRAYS:
                expression = "%s().Length" % name
            alone = the_script("", expression)
            found = the_script('WinExist("%s")' % title, expression)
            official_alone = the_answer_of_the_official(alone, work)
            official_found = the_answer_of_the_official(found, work)
            tree_alone = the_answer_of_the_tree(alone, work)
            tree_found = the_answer_of_the_tree(found, work)
            rows.append(
                (
                    name,
                    official_alone,
                    tree_alone,
                    official_found,
                    tree_found,
                )
            )

    different = [
        row
        for row in rows
        if row[1] != row[2] or row[3] != row[4]
    ]
    print("the names of the family: %d" % len(rows))
    print("the names that the binary and the tree do not agree on: %d" % len(different))
    for name, official_alone, tree_alone, official_found, tree_found in different:
        print("--- %s ---" % name)
        print("  no target   the binary: %s" % official_alone[:110])
        print("  no target   the tree   : %s" % tree_alone[:110])
        print("  with a found the binary: %s" % official_found[:110])
        print("  with a found the tree   : %s" % tree_found[:110])

    if args.out:
        with args.out.open("w", encoding="utf-8", newline="\n") as handle:
            handle.write(
                "name\tthe binary without a target\tthe tree without a target"
                "\tthe binary with a window found\tthe tree with a window found\n"
            )
            for row in rows:
                handle.write("\t".join(row) + "\n")
        print("written: %s" % args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
