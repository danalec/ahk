"""The trait of the desktop, as a spec.

    python scripts/dump-desktop-traits.py [trait ...]

The `Desktop` trait of the `ahk` crate is the sum of fourteen capability
traits, and the signatures of their methods are the contract that a
backend has to meet. The dump is the spec of the Windows backend.
"""

import pathlib
import re
import sys

SOURCE = pathlib.Path("crates/ahk/src/desktop/mod.rs")


def the_body_of(text, start):
    depth = 1
    index = start
    while index < len(text) and depth:
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
        index += 1
    return text[start:index]


def main():
    text = SOURCE.read_text(encoding="utf-8")
    wanted = sys.argv[1:] or None
    for match in re.finditer(r"pub trait (\w+)(?::[^{]+)?\s*\{", text):
        name = match.group(1)
        if wanted and name not in wanted:
            continue
        body = the_body_of(text, match.end())
        print("=== %s" % name)
        for method in re.finditer(r"(?m)^    fn ([a-z_0-9]+)\(", body):
            # the parameters, balanced
            index = body.index("(", method.start())
            depth = 0
            while index < len(body):
                if body[index] == "(":
                    depth += 1
                elif body[index] == ")":
                    depth -= 1
                    if depth == 0:
                        break
                index += 1
            parameters = " ".join(body[body.index("(", method.start()):index + 1].split())
            # the return type, up to the brace or the semicolon
            tail = body[index + 1:index + 200]
            stop = len(tail)
            for mark in ("{", ";"):
                where = tail.find(mark)
                if where != -1:
                    stop = min(stop, where)
            returns = " ".join(tail[:stop].split())
            print(
                "    fn %s%s%s;"
                % (
                    method.group(1),
                    parameters,
                    (" -> " + returns) if returns else "",
                )
            )
        print()


if __name__ == "__main__":
    main()
