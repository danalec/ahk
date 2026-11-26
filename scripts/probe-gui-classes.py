"""Read the table of the nested classes of the GUI out of the binary.

    python scripts/probe-gui-classes.py [binary]

The classes of the controls of a window are nested classes of the class
`Gui`, and a script reads them as values: `Type(Gui.Control)` is "Class"
on the binaries of v2.0.19 and of v2.0.31.

The table of each one of them is read out of the binary: the prototype of
the class owns the members of it, and the base of the class is the class
that it derives from. The instrument of the measurement of the tree is
this script, and the tables that it reads are in
`crates/ahk/reference/ahk-v2.0.32.surface.tsv`.

The binary is the one of the v2.0.31 by default, and the one of the
v2.0.19 when the path of it is given.
"""

import os
import subprocess
import sys
import tempfile
from pathlib import Path

CLASSES = [
    "Gui.Control", "Gui.List", "Gui.ListBox", "Gui.ListView", "Gui.TreeView",
    "Gui.StatusBar", "Gui.Button", "Gui.CheckBox", "Gui.ComboBox",
    "Gui.Custom", "Gui.DDL", "Gui.DateTime", "Gui.Edit", "Gui.GroupBox",
    "Gui.Hotkey", "Gui.Link", "Gui.MonthCal", "Gui.Pic", "Gui.Progress",
    "Gui.Radio", "Gui.Slider", "Gui.Tab", "Gui.Text", "Gui.UpDown",
    "Gui.ActiveX",
]

NAMES = [
    "Add", "Choose", "Delete", "SetFormat", "UseTab", "ClassNN", "Enabled",
    "Focus", "Focused", "GetPos", "Gui", "Hwnd", "Move", "Name", "OnCommand",
    "OnEvent", "OnNotify", "Opt", "Redraw", "SetFont", "Text", "Type", "Value",
    "Visible", "Get", "GetChild", "GetCount", "GetNext", "GetParent",
    "GetPrev", "GetSelection", "Modify", "SetIcon", "SetImageList",
    "SetParts", "SetText", "Insert", "InsertCol", "DeleteCol", "ModifyCol",
    "GetText",
]


def ask(binary, body, work):
    """Run the script in the binary and give back what it answered."""
    script = work / "case.ahk"
    script.write_bytes(body.replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(binary), "/ErrorStdOut", str(script)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    try:
        out, err = process.communicate(timeout=30)
    except subprocess.TimeoutExpired:
        process.kill()
        process.communicate()
        return "TIMEOUT (a dialog)"
    text = ((out or "") + (err or "")).strip()
    if "==>" in text:
        text = text.split("==>")[-1]
    return text.replace("\r\n", "\n").strip()


def main():
    binary = Path(sys.argv[1]) if len(sys.argv) > 1 else (
        Path(os.environ["TEMP"]) / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe")
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        for name in CLASSES:
            body = [
                "#ErrorStdOut",
                "c := %s" % name,
                "out := \"\"",
                "try {",
                "    out := \"base=\" ObjGetBase(c.Prototype).__Class \" own=\"",
                "    for p in c.Prototype.OwnProps()",
                "        out .= p \",\"",
                "} catch as e {",
                "    out := \"!! \" e.Message",
                "}",
                'FileAppend out "`n", "*"',
            ]
            print(name, "::", ask(binary, "\n".join(body) + "\n", work))


if __name__ == "__main__":
    main()
