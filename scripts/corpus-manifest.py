"""The manifest of the AHK v2 corpus of the finetune.

    python scripts/corpus-manifest.py

The corpus is the test set of the v1 -> v2 converter of AutoHotkey:
1745 real-world scripts of the v2, flat in one folder. It is read-only and
it is never executed here -- the manifest is built from the bytes of the
files alone.

The manifest is one JSON object per script (JSON Lines, UTF-8, one object
per line) and it is written to `docs/corpus/AHv2_Finetune.manifest.jsonl`.
It says, per script: the size, the number of lines, the digest of the
normalised content, the version the `#Requires` names, whether the script
includes a library, how many lines are executable, whether the first
statement of the script was swallowed into a trailing comment of the
`#SingleInstance` line, the balance of the braces, the risky calls the
script carries, the group of the duplicates and the category the name of
the file puts it in.

The normalisation of the content, before the digest: the bytes are read as
UTF-8 (a byte that is not UTF-8 becomes U+FFFD), a BOM is dropped, the line
endings become "\\n", the end of every line is stripped of its blanks and
the blank lines at the end of the file are dropped. Two files with the same
normalised content are the same script, whatever their line endings and
their trailing blanks are.

The instrument of the measurement of the corpus is this script; the
instrument of the measurement of the tree against the corpus is
`scripts/corpus-check.py`.
"""

import argparse
import collections
import hashlib
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent

CORPUS_DEFAULT = pathlib.Path(
    r"C:\Users\danalec\Documents\src\AHKv2_Finetune\data\raw_scripts\AHK_v2_Examples"
)
OUT_DEFAULT = REPO / "docs" / "corpus" / "AHv2_Finetune.manifest.jsonl"

# The calls that touch the machine. A script of this list is never run by
# the tools of this repository: the corpus carries a `Shutdown(8+1)`.
RISKY_CALLS = [
    "FileInstall",
    "DirDelete",
    "FileDelete",
    "DirMove",
    "FileMove",
    "URLDownloadToFile",
    "Download",
    "Run",
    "RunAs",
    "RunWait",
    "Shutdown",
    "RestartComputer",
    "RegDelete",
    "RegWrite",
    "IniWrite",
    "Send",
    "SendInput",
    "SendText",
    "SendPlay",
    "ControlSend",
    "ControlSendText",
    "ControlClick",
    "MouseClick",
    "MouseClickDrag",
    "BlockInput",
    "WinActivate",
    "WinClose",
    "WinKill",
    "SetWorkingDir",
    "EnvSet",
]

# The directives of the v2 (and of the v2.1, which the corpus asks for).
# A line that starts with one of these words and a `#` is a directive; a
# line that starts with a `#` and any other word is a hotkey label
# (`#Up::`), and a label is code.
DIRECTIVES = {name.lower() for name in (
    "AllowSameLineComments",
    "ClipboardTimeout",
    "DllLoad",
    "ErrorStdOut",
    "ExitApp",
    "HotIf",
    "HotIfTimeout",
    "HotInterval",
    "HotString",
    "Include",
    "InputLevel",
    "MaxThreads",
    "MaxThreadsBuffer",
    "MaxThreadsPerHotkey",
    "Module",
    "NoEnv",
    "Persistent",
    "Requires",
    "SingleInstance",
    "Suspend",
    "UseHook",
    "Warn",
    "WinActivateForce",
)}

# The markers that say that a comment carries code and not prose.
CODE_MARKERS = (":=", "(", ")", "{", "}", '"', "`", ";", "#", ",", "&", "==", "=>", ".=")
CODE_WORDS = ("Persistent", "ExitApp")

DIRECTIVE_WORD = re.compile(r"^#([A-Za-z_][A-Za-z_0-9]*)")
REQUIRES = re.compile(r"^#Requires\s+AutoHotkey\s+(\S+)", re.IGNORECASE)


def strip_comments(text):
    """Blank out the comments of the text, keeping the lines of it.

    A `;` that is not inside a string opens a comment that ends with the
    line. A `/*` opens a comment that ends with the `*/`. A string opens
    with a `"` and ends with the next `"` that is not doubled; a string
    that is not closed by the end of its line goes on with the next line,
    the way the v2 carries a string over a line break.
    """
    out = []
    index = 0
    size = len(text)
    in_string = False
    in_block = False
    while index < size:
        char = text[index]
        if in_block:
            if char == "*" and index + 1 < size and text[index + 1] == "/":
                in_block = False
                out.append("  ")
                index += 2
                continue
            out.append("\n" if char == "\n" else " ")
            index += 1
            continue
        if in_string:
            if char == '"':
                if index + 1 < size and text[index + 1] == '"':
                    out.append('""')
                    index += 2
                    continue
                in_string = False
                out.append('"')
                index += 1
                continue
            out.append(char)
            index += 1
            continue
        if char == '"':
            in_string = True
            out.append(char)
            index += 1
            continue
        if char == ";":
            while index < size and text[index] != "\n":
                out.append(" ")
                index += 1
            continue
        if char == "\ufeff":
            # A byte order mark in the middle of a file: the corpus carries
            # a few of them, and they hide the directive that follows.
            out.append(" ")
            index += 1
            continue
        if char == "/" and index + 1 < size and text[index + 1] == "*":
            in_block = True
            out.append("  ")
            index += 2
            continue
        out.append(char)
        index += 1
    return "".join(out)


def normalise(text):
    """The content of a file, with the line endings and the blanks of the
    end of the lines made the same, and with the BOM dropped."""
    if text.startswith("\ufeff"):
        text = text[1:]
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and not lines[-1]:
        lines.pop()
    return "\n".join(lines)


def is_directive(line):
    match = DIRECTIVE_WORD.match(line.lstrip())
    if not match:
        return False
    return match.group(1).lower() in DIRECTIVES


def directive_word(line):
    match = DIRECTIVE_WORD.match(line.lstrip())
    return match.group(1).lower() if match else None


def looks_like_code(comment):
    """True when a comment carries a fragment of code and not prose."""
    for marker in CODE_MARKERS:
        if marker in comment:
            return True
    for word in CODE_WORDS:
        if re.search(r"\b%s\b" % word, comment):
            return True
    return False


def swallowed_first_statement(raw_lines):
    """True when the first statement of the script was swallowed into a
    trailing comment of the `#SingleInstance` line.

    The corpus writes the head of the script as a block of comments and
    then the directive, and the statement that came after the directive in
    the original script was appended to the same line, behind the `;`. The
    statement is therefore a comment and never runs.
    """
    for raw in raw_lines:
        if "#SingleInstance" not in raw:
            continue
        where = raw.find(";")
        if where < 0:
            return False
        comment = raw[where + 1:].strip()
        if not comment:
            return False
        return looks_like_code(comment)
    return False


def risky_calls(code_text):
    """The risky calls the code of the script carries."""
    found = []
    for name in RISKY_CALLS:
        called = re.search(r"(?<![\w.$])%s\s*\(" % name, code_text)
        commanded = re.search(r"(?m)^[ \t]*%s[ \t]+" % name, code_text)
        if called or commanded:
            found.append(name)
    return found


def read_script(path):
    raw = path.read_bytes()
    text = raw.decode("utf-8", "replace")
    if text.startswith("\ufeff"):
        text = text[1:]
    return raw, text


def row_for(path):
    raw, text = read_script(path)
    raw_lines = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    code = strip_comments(text)
    code_lines = code.replace("\r\n", "\n").replace("\r", "\n").split("\n")

    exec_lines = 0
    requires = None
    has_include = False
    for line in code_lines:
        stripped = line.strip()
        if not stripped:
            continue
        if is_directive(stripped):
            word = directive_word(stripped)
            if word == "include":
                has_include = True
            elif word == "requires" and requires is None:
                match = REQUIRES.match(stripped)
                if match:
                    requires = match.group(1)
            continue
        exec_lines += 1

    normalised = normalise(text)
    digest = hashlib.sha256(normalised.encode("utf-8")).hexdigest()
    stem = path.stem
    category = stem.split("_")[0].upper()

    return {
        "file": path.name,
        "bytes": len(raw),
        "lines": len(raw_lines) - 1 if raw_lines and raw_lines[-1] == "" else len(raw_lines),
        "sha256": digest,
        "requires": requires,
        "has_include": has_include,
        "exec_lines": exec_lines,
        "swallowed_first_statement": swallowed_first_statement(raw_lines),
        "brace_delta": text.count("{") - text.count("}"),
        "side_effects": risky_calls(code),
        "dup_group": None,
        "category": category,
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--corpus", type=pathlib.Path, default=CORPUS_DEFAULT,
                        help="the folder of the corpus (read-only)")
    parser.add_argument("--out", type=pathlib.Path, default=OUT_DEFAULT,
                        help="the manifest to write")
    args = parser.parse_args(argv)

    corpus = args.corpus
    if not corpus.is_dir():
        print("the corpus is not there: %s" % corpus, file=sys.stderr)
        return 2

    paths = sorted(p for p in corpus.iterdir() if p.is_file() and p.suffix.lower() == ".ahk")
    rows = [row_for(path) for path in paths]

    # The group of the duplicates: the members of a group share the digest
    # of their normalised content, and the group is named by the prefix of
    # that digest -- the digest of the first file of the group, which is
    # the digest of the content of all of them.
    members = collections.defaultdict(list)
    for row in rows:
        members[row["sha256"]].append(row)
    for digest, group in members.items():
        if len(group) > 1:
            for row in group:
                row["dup_group"] = digest[:12]

    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.open("w", encoding="utf-8", newline="\n") as handle:
        for row in rows:
            handle.write(json.dumps(row, ensure_ascii=False, sort_keys=False) + "\n")

    summarise(rows, args.out)
    return 0


def summarise(rows, out):
    categories = collections.Counter(row["category"] for row in rows)
    groups = collections.Counter(row["dup_group"] for row in rows if row["dup_group"])
    requires = collections.Counter(row["requires"] for row in rows)
    print("manifest: %s" % out)
    print("scripts: %d" % len(rows))
    print("categories: %d" % len(categories))
    print("with a #Requires: %d" % sum(1 for row in rows if row["requires"]))
    print("  the versions named:")
    for version, count in requires.most_common():
        print("    %-24s %d" % (version, count))
    print("with a #Include: %d" % sum(1 for row in rows if row["has_include"]))
    print("with no executable line: %d" % sum(1 for row in rows if row["exec_lines"] == 0))
    print("with the first statement swallowed: %d"
          % sum(1 for row in rows if row["swallowed_first_statement"]))
    print("with an unbalanced brace: %d" % sum(1 for row in rows if row["brace_delta"] != 0))
    print("in a group of duplicates: %d" % sum(groups.values()))
    for group, count in groups.most_common():
        print("    %-14s %d" % (group, count))
    print("with a risky call: %d" % sum(1 for row in rows if row["side_effects"]))
    print("the categories:")
    for name, count in categories.most_common():
        print("    %-24s %d" % (name, count))


if __name__ == "__main__":
    sys.exit(main())
