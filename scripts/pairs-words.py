#!/usr/bin/env python3
"""Do the tree and the official refuse with the same words?

The gate of the pairs (`scripts/pairs-check.py`) asks the tree and the two
official interpreters the same question: does the script load? A refusal of
the tree is a gap only when the official one loads the script, and the
words of the refusal are not a part of that count.

This instrument asks the question that the gate does not: when both refuse,
do they refuse with the same words? It reads the report of the gate
(`docs/corpus/pairs.json`) and says how many pairs refuse the same and
which ones do not.

    python scripts/pairs-words.py

WHY: the gate counts the refusals, and a tree that refuses every script
with the same words passes it. The words are the contract that a script
sees, and they are the thing that the pairs of the upstream measure: the
same program in the v1 and in the v2, refused the same way by the tree and
by the official.

The prefix of the official -- `<cwd> (2) : ==>` -- is the place of the
script and the line of it, and the report of the tree carries the line on
its own: it is cut before the comparison, because it is not a part of the
refusal.
"""

import argparse
import collections
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REPORT = REPO / "docs" / "corpus" / "pairs.json"

# The prefix of the official: the folder of the script, the line of it, and
# the arrow of the message.
PREFIX = re.compile(r"^<cwd> \(\d+\) : ==>\s*")


def the_text_of_the_refusal(text):
    """The refusal without the place of the script and without the BOM."""
    text = PREFIX.sub("", str(text)).replace("\ufeff", "")
    return " ".join(text.split())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--report", type=pathlib.Path, default=REPORT)
    parser.add_argument("--examples", type=int, default=6,
                        help="how many differences to print")
    args = parser.parse_args()

    if not args.report.exists():
        print("the report of the pairs is not there: %s" % args.report,
              file=sys.stderr)
        print("run `python scripts/pairs-check.py` first", file=sys.stderr)
        return 2

    rows = json.loads(args.report.read_text(encoding="utf-8"))
    the_same = 0
    the_different = []
    shapes = collections.Counter()

    for row in rows:
        for dialect in ("v1", "v2"):
            reference = row.get(dialect + "_ref_error")
            tree = row.get(dialect + "_tree_error")
            if not (reference and tree):
                continue
            reference_text = the_text_of_the_refusal(reference)
            tree_text = the_text_of_the_refusal(tree)
            if reference_text == tree_text:
                the_same += 1
                continue
            shapes[(dialect, reference_text.split(".")[0], tree_text.split(".")[0])] += 1
            the_different.append((row.get("name"), dialect, reference_text, tree_text))

    print("the pairs: %d" % len(rows))
    print("  both refuse with the same words: %d" % the_same)
    print("  both refuse with different words: %d" % len(the_different))
    if not the_different:
        print("the tree and the official refuse every pair the same way")
        return 0

    print()
    for (dialect, reference_kind, tree_kind), count in shapes.most_common():
        print("%3d  %s  the official: %s" % (count, dialect, reference_kind[:60]))
        print("     the tree    : %s" % tree_kind[:60])
    print()
    for name, dialect, reference_text, tree_text in the_different[: args.examples]:
        print("### %s  %s" % (dialect, name))
        print("  the official: %s" % reference_text[:200])
        print("  the tree    : %s" % tree_text[:200])
    return 1


if __name__ == "__main__":
    sys.exit(main())
