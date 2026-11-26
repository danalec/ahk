"""Compare the documentation of two AutoHotkey v2 releases.

    python scripts/_diff_chm.py <the chm of the old> <the chm of the new>

The instrument: the CHM is decompiled with `hh.exe -decompile`, and the
names come out of the index and of the pages of the library. The delta
between two releases is what the tree has to carry.
"""

import pathlib
import re
import sys
from html import unescape


def index_entries(chm):
    """The entries of the index of the CHM: the name and the page."""
    index = (chm / "Index.hhk").read_text(encoding="utf-8", errors="replace")
    entries = re.findall(
        r'<param name="Name" value="([^"]*)"><param name="Local" value="([^"]*)">',
        index,
    )
    return entries


def lib_names(chm):
    """The names of the pages of the library: one page, one name."""
    return sorted(page.stem for page in (chm / "docs" / "lib").glob("*.htm"))


def variables_of(chm):
    """The built-in variables of a page of the documentation."""
    text = (chm / "docs" / "Variables.htm").read_text(encoding="utf-8", errors="replace")
    return sorted(set(re.findall(r"\bA_[A-Za-z][A-Za-z0-9_]*\b", text)))


def syntaxes_of(text):
    """The syntax lines of a page, in the order of the page."""
    lines = []
    for block in re.findall(r'<pre class="Syntax">(.*?)</pre>', text, re.S):
        clean = unescape(re.sub(r"<[^>]+>", "", block))
        for line in clean.splitlines():
            line = " ".join(line.split())
            if line:
                lines.append(line)
    return lines


def page_text(chm, name):
    page = chm / "docs" / "lib" / (name + ".htm")
    if not page.exists():
        return ""
    return page.read_text(encoding="utf-8", errors="replace")


def main():
    old = pathlib.Path(sys.argv[1])
    new = pathlib.Path(sys.argv[2])

    old_lib = set(lib_names(old))
    new_lib = set(lib_names(new))
    print("=== the pages of the library")
    print("only in the old: %s" % sorted(old_lib - new_lib))
    print("only in the new: %s" % sorted(new_lib - old_lib))

    old_vars = set(variables_of(old))
    new_vars = set(variables_of(new))
    print("=== the built-in variables")
    print("only in the old: %s" % sorted(old_vars - new_vars))
    print("only in the new: %s" % sorted(new_vars - old_vars))

    old_index = {name for name, _ in index_entries(old)}
    new_index = {name for name, _ in index_entries(new)}
    print("=== the entries of the index")
    print("only in the old: %s" % sorted(old_index - new_index)[:40])
    print("only in the new: %s" % sorted(new_index - old_index)[:40])

    print("=== the syntax lines that changed")
    for name in sorted(old_lib & new_lib):
        before = syntaxes_of(page_text(old, name))
        after = syntaxes_of(page_text(new, name))
        if before != after:
            print("--- %s" % name)
            for line in before:
                if line not in after:
                    print("    old: %s" % line)
            for line in after:
                if line not in before:
                    print("    new: %s" % line)


if __name__ == "__main__":
    main()
