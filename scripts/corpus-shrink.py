#!/usr/bin/env python3
"""Shrink a corpus script to the smallest one that still shows the gap.

The corpus is the oracle of the parse, and a gap of it is a bug of the tree.
The bug is easier to see when the script that shows it is small: this
harness takes the script and cuts the lines away, one window at a time, and
keeps the cut when the gap is still there.

The gap is kept when the tree refuses the script with the same first line of
the error and the official v2.0.32 loads it.

    python scripts/corpus-shrink.py <file.ahk> [the first line of the error]

The script that is measured is never executed: the question is a parse, and
the tree and the reference both answer it through their MCP servers.
"""
import json
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent

TREE = REPO / "target" / "release" / "ahk.exe"
VANILLA = pathlib.Path(
    r"C:\Users\danalec\Documents\src\ahk-v2-headless\bin\AutoHotkey64.exe"
)


class Server:
    """The MCP server of one interpreter: the `check` of it parses and answers."""

    def __init__(self, exe):
        self.p = subprocess.Popen([str(exe), "--mcp"], stdin=subprocess.PIPE,
                                  stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                  bufsize=0)
        self.n = 0

    def check(self, source, dialect="v2"):
        self.n += 1
        req = {"jsonrpc": "2.0", "id": self.n, "method": "tools/call",
               "params": {"name": "check",
                          "arguments": {"source": source, "dialect": dialect}}}
        try:
            self.p.stdin.write((json.dumps(req) + "\n").encode("utf-8"))
            self.p.stdin.flush()
            line = self.p.stdout.readline()
            if not line:
                return {"ok": False, "error": "the server closed"}
            answer = json.loads(line.decode("utf-8", "replace"))
            return json.loads(answer["result"]["content"][0]["text"])
        except Exception as exc:
            return {"ok": False, "error": "the server did not answer: %s" % exc}

    def close(self):
        try:
            self.p.kill()
        except Exception:
            pass


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    path = pathlib.Path(sys.argv[1])
    want = sys.argv[2] if len(sys.argv) > 2 else None

    tree = Server(TREE)
    official = Server(VANILLA)

    def keeps(lines):
        source = "".join(lines)
        a = tree.check(source)
        if a.get("ok"):
            return False
        head = (a.get("error") or "").splitlines()[0].strip()
        if want and head != want:
            return False
        return official.check(source).get("ok")

    lines = path.read_bytes().decode("utf-8", "replace").splitlines(True)
    if not keeps(lines):
        print("the file does not show the gap as it is")
        tree.close()
        official.close()
        return 1

    # The windows, from the widest to the narrowest.
    for size in (256, 128, 64, 32, 16, 8, 4, 2, 1):
        at = 0
        while at < len(lines):
            if size >= len(lines):
                break
            candidate = lines[:at] + lines[at + size:]
            if candidate and keeps(candidate):
                lines = candidate
            else:
                at += 1
        print("after the window of %d: %d lines" % (size, len(lines)))

    print("=" * 78)
    print("the smallest script that shows the gap:")
    print("-" * 78)
    print("".join(lines))
    print("-" * 78)
    a = tree.check("".join(lines))
    b = official.check("".join(lines))
    print("tree   : %s" % (a.get("error") or "OK").replace("\n", " | "))
    print("v2.0.32: %s" % (b.get("error") or "OK").replace("\n", " | "))

    tree.close()
    official.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
