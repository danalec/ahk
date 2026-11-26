"""The check of the corpus: the tree and the reference, side by side.

    python scripts/corpus-check.py

The corpus is the test set of the v1 -> v2 converter of AutoHotkey: 1745
real-world scripts of the v2. This script asks the tree and the reference
the same question about every one of them -- *does this script load?* --
and puts the two answers side by side.

The question is asked with the tool `check` of the MCP server of each of
them (`ahk --mcp` and `AutoHotkey64.exe --mcp`). `check` parses and never
runs, so the answer has no effect on the machine: the corpus carries a
`Shutdown(8+1)` and this script never lets it run.

The two servers are driven over stdio, with the JSON-RPC 2.0 of one JSON
object per line, and they run in parallel: a pool of workers, one
connection to each server per worker, so the 1745 checks take minutes
instead of an hour.

The answer of each of them is written to
`docs/corpus/AHv2_Finetune.check.json`, one row per script, and the
summary at the end of the run says how many scripts the two of them load
together, what the reference refuses and why, and which scripts the
reference loads and the tree does not -- those are the gaps of the tree.

The instrument of the measurement of the corpus is this script; the
instrument of the shape of the corpus is `scripts/corpus-manifest.py`.
"""

import argparse
import collections
import json
import pathlib
import queue
import subprocess
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

REPO = pathlib.Path(__file__).resolve().parent.parent

CORPUS_DEFAULT = pathlib.Path(
    r"C:\Users\danalec\Documents\src\AHKv2_Finetune\data\raw_scripts\AHK_v2_Examples"
)
TREE_DEFAULT = REPO / "target" / "release" / "ahk.exe"
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import reference_binaries  # noqa: E402

REFERENCE_DEFAULT = reference_binaries.the_v2()
OUT_DEFAULT = REPO / "docs" / "corpus" / "AHv2_Finetune.check.json"


class McpError(RuntimeError):
    """The server did not answer."""


class McpClient:
    """One MCP server over stdio.

    The framing is the JSON-RPC 2.0 of one JSON object per line. A reader
    thread takes the lines of the answer off the pipe and puts them on a
    queue, so a server that hangs cannot block the worker forever: the
    wait has a time out.
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
        if "error" in message and message["error"] is not None:
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

    def call_with_retry(self, tool, arguments, attempts=2):
        """Ask again when the server died, with a fresh connection."""
        last = None
        for attempt in range(attempts):
            try:
                return self.call(tool, arguments)
            except McpError as error:
                last = error
                self.stop()
                time.sleep(0.3)
                self.start()
        raise last


def first_line(error):
    if not error:
        return ""
    return error.splitlines()[0].strip()


def reference_reason(answer):
    """Why the reference refuses, when it refuses.

    `requires` and `library` are not gaps of the tree: the script asks
    for a version the reference does not carry, or for a library that is
    not there. `parse` is a parse error, the same kind the tree can have.
    """
    if answer.get("ok"):
        return None
    head = first_line(answer.get("error"))
    if "requires AutoHotkey" in head:
        return "requires"
    if "Script library not found" in head:
        return "library"
    return "parse"


def tree_class(answer):
    """The class of the refusal of the tree.

    The classes are the shapes of the error of the tree, taken from the
    first line of it.
    """
    if answer.get("ok"):
        return None
    head = first_line(answer.get("error"))
    if not head:
        return "empty"
    for needle, name in TREE_CLASSES:
        if needle in head:
            return name
    return "other"


# The shapes of the refusal of the tree, from the first line of the error.
TREE_CLASSES = [
    ("This global variable has not been assigned a value", "unassigned variable"),
    ("This variable has not been assigned a value", "unassigned variable"),
    ("Missing space or operator", "missing space or operator"),
    ("Unexpected token", "unexpected token"),
    ("Invalid expression", "invalid expression"),
    ("Function calls not allowed", "function call not allowed"),
    ("Invalid function name", "invalid function name"),
    ("not a function", "not a function"),
    ("not a method", "not a method"),
    ("Cannot assign", "cannot assign"),
    ("must be assigned", "must be assigned"),
    ("Duplicate declaration", "duplicate declaration"),
    ("already declared", "already declared"),
    ("Invalid", "invalid"),
    ("not found", "not found"),
]


def read_source(path):
    return path.read_bytes().decode("utf-8", "replace")


def check_row(tree, reference, path, dialect):
    source = read_source(path)
    tree_answer = tree.call_with_retry("check", {"source": source, "dialect": dialect})
    reference_answer = reference.call_with_retry("check", {"source": source})
    row = {
        "file": path.name,
        "tree_ok": bool(tree_answer.get("ok")),
        "tree_error": tree_answer.get("error"),
        "tree_line": tree_answer.get("line"),
        "tree_column": tree_answer.get("column"),
        "tree_class": tree_class(tree_answer),
        "reference_ok": bool(reference_answer.get("ok")),
        "reference_error": reference_answer.get("error"),
        "reference_line": reference_answer.get("line"),
        "reference_column": reference_answer.get("column"),
        "reference_reason": reference_reason(reference_answer),
        "agree": bool(tree_answer.get("ok")) == bool(reference_answer.get("ok")),
    }
    if not row["tree_ok"]:
        # The tree carries a v2.1 dialect of its own; a script the tree
        # refuses at the v2 and accepts at the v2.1 is not a gap of the
        # tree, it is a script of the v2.1.
        v21 = tree.call_with_retry("check", {"source": source, "dialect": "v2.1"})
        row["tree_v21_ok"] = bool(v21.get("ok"))
        row["tree_v21_error"] = v21.get("error")
    else:
        row["tree_v21_ok"] = None
        row["tree_v21_error"] = None
    return row


def run(pool_size, paths, dialect, tree_argv, reference_argv, timeout):
    work = queue.Queue()
    for path in paths:
        work.put(path)
    rows = []
    done = collections.Counter()
    lock = threading.Lock()
    started = time.monotonic()

    def worker():
        tree = McpClient(tree_argv, "tree", timeout)
        reference = McpClient(reference_argv, "reference", timeout)
        try:
            while True:
                try:
                    path = work.get_nowait()
                except queue.Empty:
                    return
                try:
                    row = check_row(tree, reference, path, dialect)
                except McpError as error:
                    row = {
                        "file": path.name,
                        "tree_ok": False,
                        "tree_error": "the server failed: %s" % error,
                        "tree_line": None,
                        "tree_column": None,
                        "tree_class": "server",
                        "reference_ok": False,
                        "reference_error": "the server failed: %s" % error,
                        "reference_line": None,
                        "reference_column": None,
                        "reference_reason": "server",
                        "agree": False,
                        "tree_v21_ok": None,
                        "tree_v21_error": None,
                    }
                with lock:
                    rows.append(row)
                    done["n"] += 1
                    if done["n"] % 100 == 0:
                        print("  %d/%d  (%.0fs)" % (done["n"], len(paths),
                                                   time.monotonic() - started))
        finally:
            tree.stop()
            reference.stop()

    with ThreadPoolExecutor(max_workers=pool_size) as pool:
        list(pool.map(lambda _: worker(), range(pool_size)))
    rows.sort(key=lambda row: row["file"])
    return rows


def summarise(rows, tree_argv, reference_argv, dialect, elapsed):
    total = len(rows)
    agree = sum(1 for row in rows if row["agree"])
    both_load = sum(1 for row in rows if row["tree_ok"] and row["reference_ok"])
    both_refuse = sum(1 for row in rows if not row["tree_ok"] and not row["reference_ok"])
    tree_only = sum(1 for row in rows if row["tree_ok"] and not row["reference_ok"])
    reference_only = sum(1 for row in rows if not row["tree_ok"] and row["reference_ok"])

    print()
    print("=" * 78)
    print("The corpus against the tree and the reference")
    print("=" * 78)
    print("tree:      %s" % " ".join(tree_argv))
    print("reference: %s" % " ".join(reference_argv))
    print("dialect of the tree: %s" % dialect)
    print("scripts: %d   (%.0fs)" % (total, elapsed))
    print()
    print("the two verdicts agree: %d/%d  (%.1f%%)" % (agree, total, 100.0 * agree / total))
    print("  both load:              %d" % both_load)
    print("  both refuse:            %d" % both_refuse)
    print("  the tree loads, the reference refuses:  %d" % tree_only)
    print("  the reference loads, the tree refuses:  %d" % reference_only)
    print()

    print("the refusals of the reference, by shape")
    reasons = collections.Counter(row["reference_reason"] for row in rows
                                  if row["reference_reason"])
    for reason, count in reasons.most_common():
        note = ""
        if reason == "requires":
            note = "  (not a gap of the tree: the script asks for a version the reference does not carry)"
        elif reason == "library":
            note = "  (not a gap of the tree: the library of the script is not there)"
        elif reason == "server":
            note = "  (the server failed)"
        print("  %-10s %5d%s" % (reason, count, note))
    print()
    heads = collections.Counter(first_line(row["reference_error"]) for row in rows
                                if row["reference_reason"] == "parse")
    print("  the parse refusals of the reference, the ten commonest shapes:")
    for head, count in heads.most_common(10):
        print("    %5d  %s" % (count, head[:96]))
    print()

    gaps = [row for row in rows if row["reference_ok"] and not row["tree_ok"]]
    print("the gaps of the tree: the reference loads and the tree refuses: %d" % len(gaps))
    if gaps:
        by_class = collections.Counter(row["tree_class"] for row in gaps)
        for name, count in by_class.most_common():
            print("  %-28s %5d" % (name, count))
        print()
        print("  one example of each class:")
        seen = set()
        for row in gaps:
            if row["tree_class"] in seen:
                continue
            seen.add(row["tree_class"])
            print("    %s (%s)" % (row["file"], row["tree_class"]))
            print("      tree: %s (line %s)" % (first_line(row["tree_error"]), row["tree_line"]))
            if row["tree_v21_ok"] is not None:
                print("      tree at the v2.1: %s"
                      % ("loads" if row["tree_v21_ok"]
                         else first_line(row["tree_v21_error"])))
        carried = sum(1 for row in gaps if row["tree_v21_ok"])
        print()
        print("  the v2.1 dialect of the tree carries %d of the %d gaps" % (carried, len(gaps)))
        print()

    print("the refusals of the tree, by shape (the whole corpus)")
    classes = collections.Counter(row["tree_class"] for row in rows if not row["tree_ok"])
    for name, count in classes.most_common():
        print("  %-28s %5d" % (name, count))
    print()
    print("the files that the tree refuses and the reference loads:")
    for row in gaps[:40]:
        print("  %-50s %s" % (row["file"], first_line(row["tree_error"])[:80]))
    if len(gaps) > 40:
        print("  ... and %d more" % (len(gaps) - 40))
    print()
    print("=" * 78)


def the_name_inside_the_tree(path):
    """The path of the tree as it is written inside the tree."""
    try:
        return str(pathlib.Path(path).relative_to(REPO))
    except ValueError:
        return pathlib.Path(path).name


def the_name_of_the_reference(path):
    """The name of the reference: the machine keeps it in a folder of its own."""
    return pathlib.Path(path).name


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--corpus", type=pathlib.Path, default=CORPUS_DEFAULT,
                        help="the folder of the corpus (read-only)")
    parser.add_argument("--tree", type=pathlib.Path, default=TREE_DEFAULT,
                        help="the binary of the tree")
    parser.add_argument("--reference", type=pathlib.Path, default=REFERENCE_DEFAULT,
                        help="the binary of the reference")
    parser.add_argument("--out", type=pathlib.Path, default=OUT_DEFAULT,
                        help="the file of the answers to write")
    parser.add_argument("--dialect", default="v2",
                        help="the dialect of the tree to ask (v2, v2.1, v1, ks, ks20)")
    parser.add_argument("--workers", type=int, default=8,
                        help="how many workers, one connection to each server each")
    parser.add_argument("--timeout", type=float, default=120.0,
                        help="how long to wait for an answer of a server")
    parser.add_argument("--limit", type=int, default=0,
                        help="check only the first N scripts (0 is all of them)")
    args = parser.parse_args(argv)

    corpus = args.corpus
    if not corpus.is_dir():
        print("the corpus is not there: %s" % corpus, file=sys.stderr)
        return 2
    for binary in (args.tree, args.reference):
        if not binary.is_file():
            print("the binary is not there: %s" % binary, file=sys.stderr)
            return 2

    paths = sorted(p for p in corpus.iterdir() if p.is_file() and p.suffix.lower() == ".ahk")
    if args.limit:
        paths = paths[:args.limit]

    tree_argv = [str(args.tree), "--mcp"]
    reference_argv = [str(args.reference), "--mcp"]

    print("checking %d scripts: %d workers, the tree at the %s"
          % (len(paths), args.workers, args.dialect))
    started = time.monotonic()
    rows = run(args.workers, paths, args.dialect, tree_argv, reference_argv, args.timeout)
    elapsed = time.monotonic() - started

    args.out.parent.mkdir(parents=True, exist_ok=True)
    document = {
        "corpus": str(corpus),
        # The tree is named by the path that is inside the tree, and the
        # reference by its name: the report is the same in every machine,
        # and two runs of it write the same file.
        "tree": [the_name_inside_the_tree(tree_argv[0])] + tree_argv[1:],
        "reference": [the_name_of_the_reference(reference_argv[0])] + reference_argv[1:],
        "dialect": args.dialect,
        "scripts": len(rows),
        "rows": rows,
    }
    with args.out.open("w", encoding="utf-8", newline="\n") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")

    summarise(rows, tree_argv, reference_argv, args.dialect, elapsed)
    print("written: %s" % args.out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
