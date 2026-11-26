"""Compare the tree and the binary of v2.0.31 on the classes of the GUI.

    python scripts/compare-gui-classes.py [binary] [tree]

Every case is a script that answers with the text of the thing it asks
about. The answer of the tree and the answer of the binary are printed
side by side, and the ones that are the same are marked.
"""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HEADER = """#ErrorStdOut
OnError(__backstop)
__backstop(e, mode) {
    FileAppend("UNCAUGHT|" Type(e) "|" e.Message "`n", "*")
    return true
}
Emit(s) {
    FileAppend(s "`n", "*")
}
"""

CASES = [
    ("the type of the class of a control", "Emit(Type(Gui.Control))"),
    ("the type of the class of a button", "Emit(Type(Gui.Button))"),
    ("the type of the class of a list view", "Emit(Type(Gui.ListView))"),
    ("the type of the class of a tab", "Emit(Type(Gui.Tab))"),
    ("the class of the class value", "Emit(Gui.Control.__Class)"),
    ("the class of the prototype", "Emit(Gui.Control.Prototype.__Class)"),
    ("the base of the prototype", "Emit(ObjGetBase(Gui.Control.Prototype).__Class)"),
    ("the base of the prototype of a list box",
     "Emit(ObjGetBase(Gui.ListBox.Prototype).__Class)"),
    ("the base of the prototype of a tab",
     "Emit(ObjGetBase(Gui.Tab.Prototype).__Class)"),
    ("the members of the prototype of a control", """
for p in Gui.Control.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a list", """
for p in Gui.List.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a list view", """
for p in Gui.ListView.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a tree view", """
for p in Gui.TreeView.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a status bar", """
for p in Gui.StatusBar.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a date time", """
for p in Gui.DateTime.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a tab", """
for p in Gui.Tab.Prototype.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a button", """
for p in Gui.Button.Prototype.OwnProps()
    Emit(p)
"""),
    ("the method of a control is there", "Emit(Gui.Control.Prototype.HasMethod(\"Focus\"))"),
    ("the method of a control is not there",
     "Emit(Gui.Control.Prototype.HasMethod(\"Add\"))"),
    ("the method of a list is there", "Emit(Gui.List.Prototype.HasMethod(\"Choose\"))"),
    ("the method of a list box is inherited",
     "Emit(Gui.ListBox.Prototype.HasMethod(\"Focus\"))"),
    ("the method of a tab is inherited", "Emit(Gui.Tab.Prototype.HasMethod(\"Focus\"))"),
    ("the method of a tab is its own", "Emit(Gui.Tab.Prototype.HasMethod(\"UseTab\"))"),
    ("the property of a control is there",
     "Emit(Gui.Control.Prototype.HasProp(\"ClassNN\"))"),
    ("the property of the prototype of a list box is inherited",
     "Emit(Gui.ListBox.Prototype.HasProp(\"ClassNN\"))"),
    ("the class holds the class of a control", "Emit(Gui.HasProp(\"Control\"))"),
    ("the class holds the class of a button", "Emit(Gui.HasProp(\"Button\"))"),
    ("the class holds the class of a list view", "Emit(Gui.HasProp(\"ListView\"))"),
    ("the class holds the class of a tree view", "Emit(Gui.HasProp(\"TreeView\"))"),
    ("the class holds the class of a status bar", "Emit(Gui.HasProp(\"StatusBar\"))"),
    ("the class holds the class of a tab", "Emit(Gui.HasProp(\"Tab\"))"),
    ("the class holds the class of a list", "Emit(Gui.HasProp(\"List\"))"),
    ("the class holds the class of a list box", "Emit(Gui.HasProp(\"ListBox\"))"),
    ("the class holds the class of a combo box", "Emit(Gui.HasProp(\"ComboBox\"))"),
    ("the class holds the class of a drop-down list", "Emit(Gui.HasProp(\"DDL\"))"),
    ("the class holds the class of a date time", "Emit(Gui.HasProp(\"DateTime\"))"),
    ("the class holds the class of an ActiveX", "Emit(Gui.HasProp(\"ActiveX\"))"),
    ("the class holds the class of an up-down", "Emit(Gui.HasProp(\"UpDown\"))"),
    ("the class does not hold a class of its own", "Emit(Gui.HasProp(\"Nope\"))"),
    ("the class of a control is a class", "Emit(Gui.Control is Class)"),
    ("the class of a control is not a map", "Emit(Gui.Control is Map)"),
    ("the call of the class of a control", "Gui.Control()"),
    ("the call of the class of a button", "Gui.Button()"),
    ("the own properties of the class of a window", """
for p in Gui.OwnProps()
    Emit(p)
"""),
    ("the own properties of the class of a control", """
for p in Gui.Control.OwnProps()
    Emit(p)
"""),
    ("the members of the prototype of a window", """
for p in Gui.Prototype.OwnProps()
    Emit(p)
"""),
    ("the method of the class value of a control",
     "Emit(Gui.Control.HasMethod(\"Focus\"))"),
    ("the property of the class value of a control",
     "Emit(Gui.Control.HasProp(\"Prototype\"))"),
    ("the base of the class of a control", "Emit(Gui.Control.Base.__Class)"),
    ("the base of the class of a list box", "Emit(Gui.ListBox.Base.__Class)"),
    ("the call of the class value of a control", "Gui.Control.Call()"),
    ("the method of the class of a control is not there",
     "Emit(Gui.Control.HasMethod(\"Add\"))"),
    ("the property of the prototype of a control is not there",
     "Emit(Gui.Control.Prototype.HasProp(\"Add\"))"),
    ("the property of the prototype of a list is inherited",
     "Emit(Gui.List.Prototype.HasProp(\"Focus\"))"),
    ("the class of a control is a class of the object",
     "Emit(Gui.Control is Object)"),
    ("the class of a control is a class of the any", "Emit(Gui.Control is Any)"),
]


def run_binary(binary, body, work):
    script = work / "case.ahk"
    script.write_bytes((HEADER + body + "\n").replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(binary), "/ErrorStdOut", str(script)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )
    try:
        out, err = process.communicate(timeout=20)
    except subprocess.TimeoutExpired:
        process.kill()
        process.communicate()
        return "TIMEOUT (a dialog)"
    text = ((out or "") + (err or "")).strip()
    if "==>" in text:
        text = text.split("==>")[-1]
    return text.replace("\r\n", "\n").strip()


def run_tree(tree, body):
    process = subprocess.run(
        [str(tree)], input=HEADER + body + "\n", capture_output=True, text=True,
        timeout=30,
    )
    try:
        report = json.loads(process.stdout)
    except ValueError:
        return "NOT JSON: " + (process.stdout + process.stderr).strip()[:200]
    parts = []
    output = [line.strip() for line in (report.get("output") or [])]
    output = [line for line in output if line]
    if output:
        parts.append(" | ".join(output))
    if report.get("error"):
        parts.append("error: " + str(report["error"]).replace("\n", " "))
    return "\n    ".join(parts) if parts else "(nothing)"


def main():
    binary = Path(sys.argv[1]) if len(sys.argv) > 1 else (
        Path(os.environ["TEMP"]) / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe")
    tree = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("target/release/ahk.exe")
    same = 0
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        for label, body in CASES:
            a = run_binary(binary, body, work)
            b = run_tree(tree, body)
            mark = "same" if " ".join(a.replace("|", " ").split()) == " ".join(
                b.replace("|", " ").split()
            ) else "DIFFERENT"
            if mark == "same":
                same += 1
            print("--- %s [%s]" % (label, mark))
            print("    the v2.0.31: %s" % a.replace("\n", " / "))
            print("    the tree:    %s" % b.replace("\n", " / "))
    print("=== %d of %d are the same" % (same, len(CASES)))


if __name__ == "__main__":
    main()
