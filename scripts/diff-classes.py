"""The classes of the dialect, out of the two documentations.

    python scripts/diff_classes.py

The classes of the dialect are not in the pages of the library: they are
in the page of the objects (`ObjList.htm`), and the nested classes of the
GUI are in the page of the controls (`GuiControl.htm`). The two releases
are compared class by class.
"""

import pathlib
import re
import sys
from html import unescape

PAGES = ("ObjList.htm", "GuiControl.htm", "Objects.htm")


def classes_of(chm):
    """The names of the classes of the documentation, with their page."""
    found = {}
    root = chm / "docs"
    for name in PAGES:
        page = root / name
        if not page.exists():
            continue
        text = page.read_text(encoding="utf-8", errors="replace")
        for match in re.finditer(r'<a[^>]+href="([^"]+)"[^>]*>([^<]+)</a>', text):
            label = unescape(match.group(2)).strip()
            if re.match(r"^[A-Z][A-Za-z0-9]*(\.[A-Za-z0-9]+)*$", label):
                found.setdefault(label, name)
    return found


def main():
    old = pathlib.Path(sys.argv[1])
    new = pathlib.Path(sys.argv[2])
    before = classes_of(old)
    after = classes_of(new)
    print("=== the classes of the old (%d) and of the new (%d)" % (len(before), len(after)))
    for label in sorted(set(after) - set(before)):
        print("  only in the new: %-28s (%s)" % (label, after[label]))
    for label in sorted(set(before) - set(after)):
        print("  only in the old: %-28s (%s)" % (label, before[label]))


if __name__ == "__main__":
    main()
