#!/usr/bin/env python3
"""Ask the tree and the official v2.0.32 the same question about a script.

The tree is not the judge of itself: a refusal of the tree is a gap only
when the official one loads the script. This harness puts the two answers
side by side, and it is the instrument of the measurements of the gaps of
the v2.

The question is a **parse**, and not a run: the `check` of the MCP server
of each side answers whether the script loads, and nothing of the script is
ever executed. A file of cases holds several of them, separated by a line
that is only `---`.

    python scripts/v2-probes/probe.py scripts/v2-probes/probe-hotkey.ahk

The tree is built with `cargo build --release` first, and the binary of the
official v2.0.32 is the headless build at the place below (the path may be
given as the second argument of the script).
"""
import json
import pathlib
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent.parent

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


def the_cases(path):
    """The cases of a file: they are separated by a line that is only `---`."""
    text = path.read_bytes().decode("utf-8", "replace")
    cases = []
    for chunk in text.split("\n---\n"):
        if chunk.strip():
            cases.append(chunk.strip("\n"))
    return cases


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    tree = Server(TREE)
    official = Server(VANILLA)
    try:
        for path in sys.argv[1:]:
            print("=== %s ===" % path)
            for number, case in enumerate(the_cases(pathlib.Path(path)), 1):
                a = tree.check(case)
                b = official.check(case)
                verdict = "SAME" if a.get("ok") == b.get("ok") else "DIFF"
                print("--- case %d: %s" % (number, verdict))
                for line in case.splitlines():
                    print("    | %s" % line)
                print("    tree   : %s" % (a.get("error") or "OK").replace("\n", " | "))
                print("    v2.0.32: %s" % (b.get("error") or "OK").replace("\n", " | "))
    finally:
        tree.close()
        official.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
