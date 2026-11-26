"""The three open questions of the tree, measured on the binaries.

    python scripts/measure-open-items.py

The warning of the v1 that the tree gives once and the reference gives
twice, the built-in variable `A_CoordMode` of the v2, and the shape of
the cursor of the machine.
"""

import json
import os
import subprocess
import tempfile
from pathlib import Path

V1_CASES = [
    (
        "the variable that is read is the value of an assignment",
        'y := z\nExitApp\n',
    ),
    (
        "the variable that is read is alone",
        'y := z\n',
    ),
]

V2_CASES = [
    (
        "the type of A_CoordMode",
        'FileAppend "type=" Type(A_CoordMode), "*"\n',
    ),
    (
        "the modes of A_CoordMode",
        'FileAppend "mouse=" A_CoordMode.Mouse " pixel=" A_CoordMode.Pixel, "*"\n',
    ),
    (
        "the cursor of the machine",
        'FileAppend "cursor=" A_Cursor, "*"\n',
    ),
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
    text = ((out or "") + (err or "")).strip()
    if "==>" in text:
        text = text.split("==>")[-1]
    return " ".join(text.split())


def main():
    temp = Path(os.environ["TEMP"])
    v1 = temp / "ahk_init" / "v1" / "bin" / "AutoHotkeyU64.exe"
    v2 = temp / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe"
    tree = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("target/release/ahk")
    with tempfile.TemporaryDirectory() as folder:
        work = Path(folder)
        print("=== the v1: the warning of the variable that is never written")
        for label, body in V1_CASES:
            print("--- %s" % label)
            print("    the v1.1.37.02: %s" % run(v1, body, work)[:300])
        print("=== the v2: A_CoordMode and the cursor")
        for label, body in V2_CASES:
            print("--- %s" % label)
            print("    the v2.0.31: %s" % run(v2, body, work)[:300])


if __name__ == "__main__":
    import sys
    main()
