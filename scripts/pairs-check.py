#!/usr/bin/env python3
"""The pairs of the upstream converter: the same program in the v1 and in the v2.

`../AHK-v2-script-converter` holds 852 pairs of files: a `.ah1` is a program
of the v1 and the `.ah2` beside it is the same program in the v2. The tree
speaks both dialects, so it can ask both of them the same question: does the
script load?

The tree is not the judge of itself, so the harness asks the two official
interpreters as well, and a refusal of the tree is a gap only when the
official one loads the script:

- the **v2** is the official v2.0.32, through the `check` of the MCP of the
  patched vanilla. The `check` parses and never runs, so nothing of the
  program is reached.
- the **v1** is the official v1.1.37.02, with `/ErrorStdOut`. The file that
  is measured carries an `ExitApp` in its first line: the load happens
  before the first line runs, and the `ExitApp` stops the program before
  any of its own lines runs. The line shifts the numbers of the lines of
  the error by one, and nothing else.

    python scripts/pairs-check.py
    python scripts/pairs-check.py --out docs/corpus/pairs.json
"""
import argparse
import collections
import json
import pathlib
import subprocess
import sys
import tempfile
import threading
import queue

REPO = pathlib.Path(__file__).resolve().parent.parent

UPSTREAM = pathlib.Path(r"C:\Users\danalec\Documents\src\AHK-v2-script-converter")
FOLDER = UPSTREAM / "tests" / "Test_Folder"

TREE = REPO / "target" / "release" / "ahk.exe"
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import reference_binaries  # noqa: E402

V2_REFERENCE = reference_binaries.the_v2()
V1_REFERENCE = reference_binaries.the_v1()

OUT = REPO / "docs" / "corpus" / "pairs.json"


class McpError(Exception):
    """The MCP server did not answer."""


class Mcp:
    """The MCP server of one interpreter: the `check` of it parses and answers.

    The framing is the JSON-RPC 2.0 of one JSON object per line. A reader
    thread takes the lines of the answer off the pipe and puts them on a
    queue, so a server that hangs cannot block the worker forever.
    """

    def __init__(self, argv, label, timeout=120.0):
        self.argv = [str(part) for part in argv]
        self.label = label
        self.timeout = timeout
        self.process = None
        self.answers = None
        self.next_id = 0
        self.start()

    def start(self):
        self.answers = queue.Queue()
        self.process = subprocess.Popen(
            self.argv,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            bufsize=0,
        )
        threading.Thread(target=self._read, daemon=True).start()

    def _read(self):
        try:
            for raw in self.process.stdout:
                self.answers.put(raw)
        except Exception:
            pass
        self.answers.put(None)

    def stop(self):
        if self.process is None:
            return
        try:
            self.process.stdin.close()
        except Exception:
            pass
        try:
            self.process.terminate()
            self.process.wait(timeout=5)
        except Exception:
            try:
                self.process.kill()
            except Exception:
                pass
        self.process = None

    def call(self, tool, arguments):
        self.next_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.next_id,
            "method": "tools/call",
            "params": {"name": tool, "arguments": arguments},
        }
        payload = (json.dumps(request) + "\n").encode("utf-8")
        try:
            self.process.stdin.write(payload)
            self.process.stdin.flush()
        except (OSError, ValueError) as error:
            raise McpError("%s: the pipe is closed (%s)" % (self.label, error))
        try:
            line = self.answers.get(timeout=self.timeout)
        except queue.Empty:
            raise McpError("%s: no answer in %gs" % (self.label, self.timeout))
        if line is None:
            raise McpError("%s: the server closed the pipe" % self.label)
        try:
            message = json.loads(line.decode("utf-8", "replace"))
        except ValueError as error:
            raise McpError("%s: the answer is not JSON (%s)" % (self.label, error))
        if message.get("error") is not None:
            raise McpError("%s: %s" % (self.label, message["error"]))
        content = (message.get("result") or {}).get("content") or []
        text = ""
        for part in content:
            if part.get("type") == "text":
                text = part.get("text", "")
        try:
            return json.loads(text)
        except ValueError as error:
            raise McpError("%s: the answer is not JSON (%s)" % (self.label, error))

    def check(self, source, dialect):
        """The verdict of the interpreter, and the refusal when it refuses."""
        last = None
        for _ in (1, 2):
            try:
                answer = self.call("check", {"source": source, "dialect": dialect})
                if answer.get("ok"):
                    return True, None
                return False, answer.get("error")
            except McpError as error:
                last = error
                self.stop()
                self.start()
        return False, "the server did not answer: %s" % last


def first_line(error):
    if not error:
        return ""
    return error.splitlines()[0].strip()


class V1Reference:
    """The official v1.1.37.02: the load of a script, and its refusal.

    The file that is measured carries an `ExitApp`, so the program stops
    before any of its own lines runs. The load of the script happens before
    the first line runs, so a refusal of the load is still seen.

    The `ExitApp` goes after the directives that are at the top of the
    script, because a directive of the v1 (`#Delimiter`, `#Requires`, ...)
    belongs to the head of the file.
    """

    def __init__(self, exe, timeout=15.0):
        self.exe = pathlib.Path(exe)
        self.timeout = timeout

    @staticmethod
    def _with_the_exit(source):
        # Preserve any leading UTF-8 BOM at byte 0 so v1.1.37.02 reads the
        # file as UTF-8 instead of seeing BOM bytes on line 2 as ANSI garbage,
        # and ensure a newline separates leading directives/comments from ExitApp.
        bom = "\ufeff" if source.startswith("\ufeff") else ""
        body = source[len(bom) :]
        lines = body.splitlines(True)
        at = 0
        while at < len(lines) and (
            not lines[at].strip()
            or lines[at].lstrip().startswith(";")
            or lines[at].lstrip().startswith("#")
        ):
            at += 1
        head = "".join(lines[:at])
        if head and not head.endswith(("\n", "\r")):
            head += "\r\n"
        return bom + head + "ExitApp\r\n" + "".join(lines[at:])

    def check(self, source):
        if not self.exe.exists():
            return False, "the reference is not there: %s" % self.exe
        with tempfile.TemporaryDirectory(prefix="ahk-v1-probe-") as folder:
            path = pathlib.Path(folder) / "probe.ahk"
            path.write_bytes(self._with_the_exit(source).encode("utf-8", "replace"))
            try:
                done = subprocess.run(
                    [str(self.exe), "/ErrorStdOut", str(path)],
                    stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    timeout=self.timeout,
                )
            except subprocess.TimeoutExpired:
                # The official v1 shows a dialog of the machine for a
                # warning, and `/ErrorStdOut` does not take that away. The
                # verdict of such a script is not known.
                return None, "the reference showed a dialog of the machine"
            out = done.stdout.decode("utf-8", "replace")
            err = done.stderr.decode("utf-8", "replace")
            text = (out + "\n" + err).strip()
            # The folder of the case is a temporary one, and its name is
            # different every time: the report says `<cwd>` for it, so that
            # two runs of the check write the same file.
            text = text.replace(str(path), "<cwd>")
            if done.returncode == 0:
                return True, None
            # The refusal of the load: the message of the official one.
            return False, text or "the reference refused with the code %d" % done.returncode


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--upstream", type=pathlib.Path, default=FOLDER)
    parser.add_argument("--out", type=pathlib.Path, default=OUT)
    parser.add_argument("--tree", type=pathlib.Path, default=TREE)
    parser.add_argument("--v1", type=pathlib.Path, default=V1_REFERENCE)
    parser.add_argument("--v2", type=pathlib.Path, default=V2_REFERENCE)
    parser.add_argument("--limit", type=int, default=0)
    args = parser.parse_args()

    if not args.tree.exists():
        print("the tree is not built: %s" % args.tree, file=sys.stderr)
        return 2

    pairs = sorted(p for p in args.upstream.rglob("*.ah1"))
    if args.limit:
        pairs = pairs[: args.limit]
    print("the pairs: %d" % len(pairs))

    tree = Mcp([str(args.tree), "--mcp"], "the tree")
    v2 = Mcp([str(args.v2), "--mcp"], "the v2.0.32") if args.v2.exists() else None
    if v2 is None:
        print("the v2.0.32 reference is not there: %s" % args.v2, file=sys.stderr)
    v1 = V1Reference(args.v1)

    rows = []
    for i, v1_path in enumerate(pairs):
        v2_path = v1_path.with_suffix(".ah2")
        if not v2_path.exists():
            continue
        source_v1 = v1_path.read_bytes().decode("utf-8", "replace")
        source_v2 = v2_path.read_bytes().decode("utf-8", "replace")

        v1_tree_ok, v1_tree_error = tree.check(source_v1, "v1")
        v2_tree_ok, v2_tree_error = tree.check(source_v2, "v2")
        v1_ref_ok, v1_ref_error = v1.check(source_v1)
        if v2 is None:
            v2_ref_ok, v2_ref_error = None, "the v2.0.32 reference is not there"
        else:
            v2_ref_ok, v2_ref_error = v2.check(source_v2, "v2")

        rows.append({
            "name": str(v1_path.relative_to(args.upstream)),
            "v1_tree_ok": v1_tree_ok,
            "v1_ref_ok": v1_ref_ok,
            "v1_tree_error": v1_tree_error,
            "v1_ref_error": v1_ref_error,
            "v2_tree_ok": v2_tree_ok,
            "v2_ref_ok": v2_ref_ok,
            "v2_tree_error": v2_tree_error,
            "v2_ref_error": v2_ref_error,
        })
        if (i + 1) % 50 == 0:
            print("  %d/%d" % (i + 1, len(pairs)))
            sys.stdout.flush()

    tree.stop()
    if v2 is not None:
        v2.stop()

    def count(predicate):
        return sum(1 for row in rows if predicate(row))

    print()
    for dialect, title in (("v1", "the official v1.1.37.02"), ("v2", "the official v2.0.32")):
        print("the %s: the tree and %s" % (dialect, title))
        print("  both load:                        %4d" % count(
            lambda r: r[dialect + "_tree_ok"] and r[dialect + "_ref_ok"]))
        print("  the tree refuses, the %s loads:   %4d" % (dialect, count(
            lambda r: not r[dialect + "_tree_ok"] and r[dialect + "_ref_ok"])))
        print("  the tree loads, the %s refuses:   %4d" % (dialect, count(
            lambda r: r[dialect + "_tree_ok"] and r[dialect + "_ref_ok"] is False)))
        print("  both refuse:                      %4d" % count(
            lambda r: not r[dialect + "_tree_ok"] and r[dialect + "_ref_ok"] is False))
        print("  the reference showed a dialog:    %4d" % count(
            lambda r: r[dialect + "_ref_ok"] is None))
        print()

    for dialect in ("v1", "v2"):
        gaps = [r for r in rows if not r[dialect + "_tree_ok"] and r[dialect + "_ref_ok"]]
        if not gaps:
            continue
        print("the gaps of the tree in the %s, by shape:" % dialect)
        shapes = collections.Counter(
            first_line(r[dialect + "_tree_error"])[:80] for r in gaps
        )
        for shape, count_ in shapes.most_common(20):
            print("  %4d  %s" % (count_, shape))
        print()

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(rows, indent=1), encoding="utf-8")
    print()
    print("written: %s" % args.out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
