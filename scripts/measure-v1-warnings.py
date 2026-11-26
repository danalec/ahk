"""The number of the warnings of the v1, measured on the binary.

    python scripts/measure-v1-warnings.py

The v1 warns once for the read of a variable that has no value, and twice
when the read is the value of an assignment, the value of an object or the
value of a comparison: the v1 materializes the empty content of the
variable once for each of those.
"""

import json
import os
import subprocess
import tempfile
from pathlib import Path

CASES = [
    ("a plain read", 'nope\n'),
    ("the value of an assignment", 'x := nope\n'),
    ("the value of an object", 'x := {a: nope}\n'),
    ("the value of a comparison", 'if (nope = 1)\n    x := 1\n'),
    ("the value of a call", 'x := f()\nf() {\n    return nope\n}\n'),
    ("the value of an argument", 'x := f(nope)\nf() {\n    return 1\n}\n'),
    ("the value of a sum", 'x := nope + 1\n'),
    ("the value of a condition", 'if (nope)\n    x := 1\n'),
    ("the value of a concatenation", 'x := nope "b"\n'),
    ("the right value of a comparison", 'if (1 = nope)\n    x := 1\n'),
    ("the two values of a comparison", 'if (nope = nada)\n    x := 1\n'),
    ("the value of an assignment in an object", 'x := {a: nope, b: nada}\n'),
]


def run(binary, body, work, timeout=10):
    script = work / "case.ahk"
    script.write_bytes(body.replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(binary), "/ErrorStdOut", str(script)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    try:
        out, err = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        process.kill()
        process.communicate()
        return "TIMEOUT (a dialog)"
    return ((out or "") + (err or "")).strip()


def main():
    temp = Path(os.environ["TEMP"])
    v1 = temp / "ahk_init" / "v1" / "bin" / "AutoHotkeyU64.exe"
    tree = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("target/release/ahk")
    local_cases = [
        ("a local that is read alone", 'f() {\n    nope\n}\nf()\n'),
        ("a local that is the value of an assignment", 'f() {\n    x := nope\n}\nf()\n'),
        ("a local that is an argument", 'f() {\n    x := g(nope)\n}\ng() {\n    return 1\n}\nf()\n'),
    ]
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        for label, body in CASES:
            script = "#Warn UseUnsetGlobal, StdOut\n" + body
            reference = run(v1, script, work)
            warnings = reference.count("Warning:")
            process = subprocess.run(
                [str(tree), "--dialect=v1"], input=script, capture_output=True,
                text=True, timeout=30,
            )
            try:
                report = json.loads(process.stdout.strip())
                tree_warnings = len(report.get("warnings") or [])
            except ValueError:
                tree_warnings = "?"
            print("--- %s" % label)
            print("    the v1.1.37.02: %s warning(s)" % warnings)
            print("    the tree: %s warning(s)" % tree_warnings)
        print("=== the local variable")
        for label, body in local_cases:
            script = "#Warn UseUnsetLocal, StdOut\n" + body
            reference = run(v1, script, work)
            warnings = reference.count("Warning:")
            process = subprocess.run(
                [str(tree), "--dialect=v1"], input=script, capture_output=True,
                text=True, timeout=30,
            )
            try:
                report = json.loads(process.stdout.strip())
                tree_warnings = len(report.get("warnings") or [])
            except ValueError:
                tree_warnings = "?"
            print("--- %s" % label)
            print("    the v1.1.37.02: %s warning(s)" % warnings)
            print("    the tree: %s warning(s)" % tree_warnings)


if __name__ == "__main__":
    import sys
    main()
