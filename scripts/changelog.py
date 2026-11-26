"""The change log of a release, out of the decompiled documentation.

    python scripts/_changelog.py <the chm> [<the version>]

The change log of the release is the summary of what the release
changed, written by the people of the release.
"""

import pathlib
import re
import sys
from html import unescape


def text_of(path):
    raw = path.read_text(encoding="utf-8", errors="replace")
    raw = re.sub(r"<script.*?</script>", " ", raw, flags=re.S | re.I)
    raw = re.sub(r"<style.*?</style>", " ", raw, flags=re.S | re.I)
    raw = re.sub(r"<br\s*/?>", "\n", raw, flags=re.I)
    raw = re.sub(r"</(p|div|li|h[1-6]|tr)>", "\n", raw, flags=re.I)
    raw = re.sub(r"<[^>]+>", "", raw)
    raw = unescape(raw)
    lines = [" ".join(line.split()) for line in raw.splitlines()]
    return "\n".join(line for line in lines if line)


def main():
    chm = pathlib.Path(sys.argv[1])
    version = sys.argv[2] if len(sys.argv) > 2 else None
    page = chm / "docs" / "ChangeLog.htm"
    text = text_of(page)
    if version is None:
        print(text[:6000])
        return
    # The section of the release: from its heading to the next one.
    match = re.search(r"^.*%s.*$" % re.escape(version), text, re.M)
    if not match:
        print("the version %s is not in the change log" % version)
        print(text[:4000])
        return
    start = match.start()
    rest = text[start:]
    following = re.search(r"\n\s*(v?2\.0\.\d+|v?1\.1\.\d+\.\d+)", rest[10:])
    end = 10 + following.start() if following else len(rest)
    print(rest[:end])


if __name__ == "__main__":
    main()
