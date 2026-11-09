#!/usr/bin/env python3
"""The drift check between the surface of the tree and the documents.

    python scripts/check-surface-docs.py [--binary PATH] [--fix]

The runner is the `ahk` of this tree. `ahk --surface --json --dialect=v2`
and `--dialect=v1` are the numbers of the tree, and the documents that carry
them are read here: `docs/v2-capability-report.md` (the generated one), the
summary of `docs/ahk-gaps-report.md`, the JSON samples of `docs/dialects.md`
and `docs/runner-and-report.md`, and the two sentences of
`docs/ahk-v1-dialect.md`.

The check exits 0 when every document carries the numbers of the runner, and
1 when one of them does not -- the message says which document, which line,
and what the two numbers are. `--fix` writes the numbers of the runner into
the documents and exits 0 when the tree and the documents agree afterwards.

There is no CI in this repository yet. This is the check that CI is meant to
run after the release build:

    cargo build --release -p ahk-cli
    python scripts/check-surface-docs.py

The binary defaults to `target/release/ahk` on Windows
(`target/release/ahk.exe`) and to `target/release/ahk` elsewhere, and the
script may be given one with `--binary`. Python 3 only, no dependencies.
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# The numbers of one dialect, as the runner answers them.
#   total, values, refusals, pending
DIALECTS = ("v2", "v1")


def the_binary(asked):
    """The runner to ask: the one that was given, or the release build."""
    if asked:
        path = Path(asked)
        if not path.is_absolute():
            path = ROOT / path
        if not path.exists():
            raise SystemExit("the runner %s is not there" % path)
        return path
    for name in ("release", "debug"):
        for suffix in ("", ".exe"):
            path = ROOT / "target" / name / ("ahk" + suffix)
            if path.exists():
                return path
    raise SystemExit(
        "no runner under target/release or target/debug: build it with "
        "`cargo build --release -p ahk-cli`"
    )


def read_surface(binary, dialect):
    """The summary of one dialect, and the distinct names of it."""
    result = subprocess.run(
        [str(binary), "--surface", "--json", "--dialect=" + dialect],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        timeout=120,
    )
    if result.returncode != 0:
        raise SystemExit(
            "the runner failed for the %s: %s"
            % (dialect, (result.stderr or result.stdout).strip()[:400])
        )
    try:
        surface = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise SystemExit("the runner printed no JSON for the %s: %s" % (dialect, error))
    counts = {
        "total": int(surface["total"]),
        "values": int(surface["values"]),
        "refusals": int(surface["refusals"]),
        "pending": int(surface["pending"]),
        # The distinct names are counted the way the text surface writes
        # them: a method carries the class of it, as in `Map.Clone`.
        "distinct": len(
            {
                entry["name"] if not entry.get("class") else entry["class"] + "." + entry["name"]
                for entry in surface["entries"]
            }
        ),
    }
    return counts


# Every check is a document, the sentence of it that carries numbers, and the
# numbers of the runner that the sentence has to carry -- named one by one in
# the order the groups of the pattern are written.
CHECKS = [
    {
        "document": "docs/v2-capability-report.md",
        "what": "the summary of the generated report",
        "pattern": re.compile(
            r"\*\*(\d+) entries\*\*\s*:\s*\*\*(\d+) answer with a value\*\*"
            r"\s*,\s*\*\*(\d+) are refused\*\*"
        ),
        "numbers": ("total", "values", "refusals"),
        "dialect": "v2",
    },
    {
        "document": "docs/v2-capability-report.md",
        "what": "the line of the table of the generated report",
        "pattern": re.compile(r"the table has \*\*(\d+) entries and (\d+) distinct names\*\*", re.I),
        "numbers": ("total", "distinct"),
        "dialect": "v2",
    },
    {
        "document": "docs/ahk-gaps-report.md",
        "what": "the row of the names of the surface",
        "pattern": re.compile(r"\|\s*The names of the surface\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|"),
        "numbers": ("total", "total"),
        "dialect": ("v2", "v1"),
    },
    {
        "document": "docs/ahk-gaps-report.md",
        "what": "the row of the names that answer with a value",
        "pattern": re.compile(r"\|\s*Answer with a value\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|"),
        "numbers": ("values", "values"),
        "dialect": ("v2", "v1"),
    },
    {
        "document": "docs/ahk-gaps-report.md",
        "what": "the row of the names that are refused with a reason",
        "pattern": re.compile(r"\|\s*Refused with a reason\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|"),
        "numbers": ("refusals", "refusals"),
        "dialect": ("v2", "v1"),
    },
    {
        "document": "docs/ahk-gaps-report.md",
        "what": "the row of the pending names",
        "pattern": re.compile(r"\|\s*Pending \(not carried yet\)\s*\|\s*(\d+)\s*\|\s*(\d+)\s*\|"),
        "numbers": ("pending", "pending"),
        "dialect": ("v2", "v1"),
    },
    {
        "document": "docs/dialects.md",
        "what": "the JSON sample of the summary",
        "pattern": re.compile(
            r'\{"dialect": "(v2|v1)", "total": (\d+), "values": (\d+), '
            r'"refusals": (\d+), "pending": (\d+)\}'
        ),
        "numbers": ("total", "values", "refusals", "pending"),
        "dialect": "from the first group",
    },
    {
        "document": "docs/runner-and-report.md",
        "what": "the line of the text surface",
        "pattern": re.compile(
            r"the surface of v2 has (\d+) entries: (\d+) answer with a value, "
            r"(\d+) are refused, (\d+) are pending\."
        ),
        "numbers": ("total", "values", "refusals", "pending"),
        "dialect": "v2",
    },
    {
        "document": "docs/runner-and-report.md",
        "what": "the JSON sample of the v2",
        "pattern": re.compile(
            r'\{"dialect":"v2","total":(\d+),"values":(\d+),"refusals":(\d+),'
            r'"pending":(\d+)'
        ),
        "numbers": ("total", "values", "refusals", "pending"),
        "dialect": "v2",
    },
    {
        "document": "docs/runner-and-report.md",
        "what": "the JSON sample of the v1",
        "pattern": re.compile(
            r'\{"dialect":"v1","total":(\d+),"values":(\d+),"refusals":(\d+),'
            r'"pending":(\d+)'
        ),
        "numbers": ("total", "values", "refusals", "pending"),
        "dialect": "v1",
    },
    {
        "document": "docs/runner-and-report.md",
        "what": "the number of the members of the entries array",
        "pattern": re.compile(r"four of its (\d+) members"),
        "numbers": ("total",),
        "dialect": "v2",
    },
    {
        "document": "docs/ahk-v1-dialect.md",
        "what": "the sentence of the surface of the v1",
        "pattern": re.compile(
            r"The surface of the v1 has \*\*(\d+)\*\* entries: "
            r"\*\*(\d+)\*\* answer with a value,\s*\*\*(\d+)\*\* are refused"
        ),
        "numbers": ("total", "values", "refusals"),
        "dialect": "v1",
    },
    {
        "document": "docs/ahk-v1-dialect.md",
        "what": "the sentence of the surface of the v2",
        "pattern": re.compile(
            r"\*\*(\d+)\*\* entries, \*\*(\d+)\*\* values, \*\*(\d+)\*\* refusals"
        ),
        "numbers": ("total", "values", "refusals"),
        "dialect": "v2",
    },
]


def the_expectation(check, surfaces):
    """What the check has to find, as the numbers of the runner."""
    wanted = check["dialect"]
    if wanted == "from the first group":
        # The dialect of the sample is the first group of the pattern; it is
        # read before this function is asked, so it cannot be here. The
        # dialects of the JSON sample are checked one at a time below.
        raise AssertionError("the dialect of the sample is read from the match")
    if isinstance(wanted, tuple):
        return [surfaces[dialect][name] for dialect, name in zip(wanted, check["numbers"])]
    return [surfaces[wanted][name] for name in check["numbers"]]


def check_one(check, text, surfaces):
    """The differences of one document sentence, as (found, wanted) pairs."""
    if check["dialect"] == "from the first group":
        # The JSON samples of the two dialects: each one is checked against
        # the dialect that names it.
        problems = []
        found = False
        for match in check["pattern"].finditer(text):
            found = True
            dialect = match.group(1)
            wanted = [surfaces[dialect][name] for name in check["numbers"]]
            got = [int(match.group(index)) for index in range(2, 2 + len(wanted))]
            if got != wanted:
                problems.append((got, wanted))
        if not found:
            problems.append((None, None))
        return problems, found
    matches = list(check["pattern"].finditer(text))
    wanted = the_expectation(check, surfaces)
    if not matches:
        return [(None, wanted)], False
    problems = []
    for match in matches:
        got = [int(match.group(index)) for index in range(1, 1 + len(wanted))]
        if got != wanted:
            problems.append((got, wanted))
    return problems, bool(matches)


def fix_one(check, text, surfaces):
    """The text with the numbers of the runner written into its sentence."""
    if check["dialect"] == "from the first group":
        def replace_from_group(match):
            dialect = match.group(1)
            wanted = [surfaces[dialect][name] for name in check["numbers"]]
            return put(match, range(2, 2 + len(wanted)), wanted)
        return check["pattern"].sub(replace_from_group, text)
    wanted = the_expectation(check, surfaces)

    def replace(match):
        return put(match, range(1, 1 + len(wanted)), wanted)

    return check["pattern"].sub(replace, text)


def put(match, groups, wanted):
    """The text of a match with the given groups replaced by the numbers."""
    text = match.group(0)
    spans = [match.span(index) for index in groups]
    for (start, end), number in sorted(zip(spans, wanted), reverse=True):
        text = text[: start - match.start()] + str(number) + text[end - match.start():]
    return text


def main():
    parser = argparse.ArgumentParser(
        description="check that the documents carry the numbers of the tree"
    )
    parser.add_argument("--binary", default=None, metavar="PATH",
                        help="the runner to ask (default: target/release/ahk)")
    parser.add_argument("--fix", action="store_true",
                        help="write the numbers of the runner into the documents")
    arguments = parser.parse_args()

    binary = the_binary(arguments.binary)
    surfaces = {dialect: read_surface(binary, dialect) for dialect in DIALECTS}

    print("the runner: %s" % binary)
    for dialect in DIALECTS:
        counts = surfaces[dialect]
        print("the %s: %d entries, %d values, %d refusals, %d pending, %d distinct names"
              % (dialect, counts["total"], counts["values"], counts["refusals"],
                 counts["pending"], counts["distinct"]))

    changed = set()
    failures = 0
    for check in CHECKS:
        path = ROOT / check["document"]
        if not path.exists():
            print("MISSING %s (%s)" % (check["document"], check["what"]))
            failures += 1
            continue
        text = path.read_text(encoding="utf-8")
        problems, found = check_one(check, text, surfaces)
        if not problems and found:
            continue
        failures += 1
        print("DRIFT %s: %s" % (check["document"], check["what"]))
        for got, wanted in problems:
            if got is None:
                print("  the line is not there; it has to carry %s" % (wanted,))
            else:
                print("  the document says %s; the tree says %s"
                      % (tuple(got), tuple(wanted)))
        if arguments.fix:
            path.write_text(fix_one(check, text, surfaces), encoding="utf-8", newline="")
            changed.add(check["document"])

    if arguments.fix and changed:
        # The fix is not trusted until the documents are read again.
        print("the numbers are written into: %s" % ", ".join(sorted(changed)))
        still = 0
        for check in CHECKS:
            text = (ROOT / check["document"]).read_text(encoding="utf-8")
            problems, found = check_one(check, text, surfaces)
            if problems or not found:
                still += 1
                print("STILL WRONG %s: %s" % (check["document"], check["what"]))
        if still:
            print("the fix did not settle the documents")
            return 1
        print("the documents and the tree agree")
        return 0

    if failures:
        print()
        print("%d of the %d checks failed: the documents do not carry the numbers "
              "of the tree." % (failures, len(CHECKS)))
        print("Run `python scripts/check-surface-docs.py --fix` to write the "
              "numbers of the runner into them, or update the prose by hand.")
        return 1
    print("the documents carry the numbers of the tree (%d checks)" % len(CHECKS))
    return 0


if __name__ == "__main__":
    sys.exit(main())