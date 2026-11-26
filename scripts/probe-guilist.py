"""The class `Gui.List` on the two binaries.

    python scripts/_probe_guilist.py

The measurement of the release mentioned a `Gui.List.Prototype.Add`, and
the table of the tree has no `Gui.List` in it: the question is whether the
class is there at all.
"""

import subprocess
import sys
import tempfile
import os
from pathlib import Path

CASES = [
    'FileAppend "type=" Type(Gui.List), "*"',
    'lb := Gui.List()\nFileAppend "made", "*"',
    'lb := Gui.List()\nlb.Add(["four"])\nFileAppend "added", "*"',
    'lb := Gui.List()\nlb.Add("four")',
    'FileAppend "type=" Type(Gui.ListBox), "*"',
]

BODY = """try {
    %s
} catch as e {
    FileAppend "caught: " e.Message, "*"
}
"""


def ask(binary, body, work):
    script = work / "case.ahk"
    script.write_bytes(body.replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(binary), "/ErrorStdOut", str(script)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    try:
        out, err = process.communicate(timeout=10)
    except subprocess.TimeoutExpired:
        process.kill()
        process.communicate()
        return "TIMEOUT (a dialog)"
    text = ((out or "") + (err or "")).strip()
    if "==>" in text:
        text = text.split("==>")[-1]
    return " ".join(text.split())


def main():
    temp = Path(os.environ["TEMP"])
    old = temp / "ahk-v2-measure" / "AutoHotkey64.exe"
    new = temp / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe"
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        for case in CASES:
            body = BODY % case
            print("=== %s" % case.replace("\n", " ; "))
            print("    2.0.19 : %s" % ask(old, body, work)[:180])
            print("    2.0.31 : %s" % ask(new, body, work)[:180])


if __name__ == "__main__":
    main()
