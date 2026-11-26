"""The methods that a backend has to write, and the ones it may leave.

    python scripts/dump-required.py

A method of a trait that has a body is a method that a backend may leave
alone: the refusal of the trait is the answer. A method without a body
is a method that the backend has to write.
"""

import pathlib
import re

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
    for match in re.finditer(r"pub trait (\w+)(?::[^{]+)?\s*\{", text):
        name = match.group(1)
        body = the_body_of(text, match.end())
        required = []
        for method in re.finditer(r"(?m)^    fn ([a-z_0-9]+)\(", body):
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
            tail = body[index + 1:index + 200]
            stop = len(tail)
            for mark in ("{", ";"):
                where = tail.find(mark)
                if where != -1:
                    stop = min(stop, where)
            if tail[stop:stop + 1] == ";":
                required.append(method.group(1))
        print("%-12s %s" % (name, ", ".join(required)))


if __name__ == "__main__":
    main()
