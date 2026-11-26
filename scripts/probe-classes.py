"""The classes of the dialect, on the binary of the release and on the tree.

    python scripts/_probe_classes.py

The classes of the GUI are nested classes of the class `Gui`, and the
tree carries five of them. The question is what the binaries answer for
the others.
"""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

CLASSES = [
    "Gui.Control", "Gui.List", "Gui.ListBox", "Gui.ListView", "Gui.TreeView",
    "Gui.Menu", "Gui.StatusBar", "Gui.Button", "Gui.CheckBox", "Gui.ComboBox",
    "Gui.Custom", "Gui.DDL", "Gui.DateTime", "Gui.Edit", "Gui.GroupBox",
    "Gui.Hotkey", "Gui.Link", "Gui.MonthCal", "Gui.Pic", "Gui.Progress",
    "Gui.Radio", "Gui.Slider", "Gui.Tab", "Gui.Text", "Gui.UpDown",
    "Gui.ActiveX",
]

BODY = """try {
    FileAppend "type=" Type(%s), "*"
} catch as e {
    FileAppend "caught: " e.Message, "*"
}
"""


def ask_the_binary(binary, body, work):
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


def ask_the_tree(tree, body):
    process = subprocess.run(
        [str(tree)], input=body, capture_output=True, text=True, timeout=30,
    )
    try:
        report = json.loads(process.stdout.strip())
    except ValueError:
        return process.stdout.strip().replace("\n", " ")[:120]
    if report.get("error"):
        return "error: %s" % report["error"].replace("\n", " ")[:90]
    return "output: %s" % ", ".join(report.get("output") or [])


def main():
    temp = Path(os.environ["TEMP"])
    old = temp / "ahk-v2-measure" / "AutoHotkey64.exe"
    new = temp / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe"
    tree = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("target/debug/ahk.exe")
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        for name in CLASSES:
            body = BODY % name
            a = ask_the_binary(old, body, work)
            b = ask_the_binary(new, body, work)
            t = ask_the_tree(tree, body)
            print("%-16s 2.0.19: %-28s 2.0.31: %-28s tree: %s"
                  % (name, a[:28], b[:28], t[:60]))


if __name__ == "__main__":
    main()
