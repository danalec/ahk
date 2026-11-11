#!/usr/bin/python3
"""Read and write the text of the tree without touching its line endings.

`pathlib.Path.read_text()` translates the line endings: a lone `\\r` inside
a line becomes a line break, and a file that has one (a comment with a
stray `\\r` in it, say) is broken by the round trip. The port reads and
writes the text of the tree, and it must not change a byte it did not
mean to change.
"""

import pathlib


def read_lines(path):
    """The lines of a file, with the line endings as they are."""
    with open(path, "r", encoding="utf-8", newline="") as handle:
        return handle.read().split("\n")


def write_lines(path, lines):
    """The lines of a file, written back with the line endings they had."""
    with open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write("\n".join(lines))


def read_text(path):
    """The text of a file, with the line endings as they are."""
    with open(path, "r", encoding="utf-8", newline="") as handle:
        return handle.read()


def write_text(path, text):
    """The text of a file, written back with the line endings it had."""
    with open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write(text)


def main():
    root = pathlib.Path(__file__).resolve().parent.parent
    for name in ["crates/ahk/src/classes/mod.rs", "crates/ahk/src/runtime.rs"]:
        path = root / name
        lines = read_lines(path)
        write_lines(path, lines)
        print("the line endings of %s are the ones it had" % name)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

