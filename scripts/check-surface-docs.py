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

The second half of the check is the generated subtree of the surface:

    docs/surface/README.md            (hand-written, never required here)
    docs/surface/index.md             (the index of every name)
    docs/surface/crosswalk.md         (the v1 <-> v2 <-> KeySharp crosswalk)
    docs/surface/v2.md                (the catalogue of the v2)
    docs/surface/v1.md                (the catalogue of the v1)
    docs/surface/keysharp.md          (the catalogue of the KeySharp dialect)
    docs/surface/families/<f>.md      (one page per family)

Four things are proved about it:

1. Every name of `crates/ahk/reference/ahk-v2.0.19.surface.tsv`,
   `ahk-v1.1.37.02.surface.tsv` and `ahk-ks.surface.tsv` is carried by the
   subtree -- the page of the family of it, the catalogue of the dialect or
   the index. A name that no file of the subtree carries is a failure, and
   the message names the dialect, the name and the file it has to be in.

2. Every name of the reference tables has its page of a family: the ficha of
   a name is in exactly one page of `docs/surface/families/`.

3. The counts the subtree prints are the counts of the tree: the number of
   the names of every dialect, of every family, and the numbers of the
   names that answer with a value, that are refused and that are pending.
   The names come from the reference tables, and the state of every name
   comes from the runner: `ahk --surface --json --dialect=v2|v1|ks`.

4. The generated files are the ones the generator writes. When
   `scripts/gen-surface-docs.py` is there and takes a `--check`, it is run
   and its verdict is reported; the generator is never re-implemented here.

The counts are read out of the generated files in this order: the marker of
the generator

    <!-- surface-counts: dialect=v2 total=833 values=685 refusals=148 pending=0 -->

the table of the catalogue ("| Family | Names | Value | Refused | Pending |"),
the table of the index ("| Dialect | Names | Refused | Pending |"), and the
prose of the page ("the surface of the v2 has 833 entries: 685 answer with a
value, 148 are refused, 0 are pending").

The names of a dialect are counted as the tree counts them: a name with a
class is one name ("Map.Clone"), and a name that the tree carries twice
(the getter and the setter of a property) is one name.
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

# The dialects of the surface of the tree, the third one being the dialect
# of KeySharp. The runner is asked for each of them.
SURFACE_DIALECTS = ("v2", "v1", "ks")

# The table of the reference of every dialect: the names of it.
REFERENCE = {
    "v2": "crates/ahk/reference/ahk-v2.0.19.surface.tsv",
    "v1": "crates/ahk/reference/ahk-v1.1.37.02.surface.tsv",
    "ks": "crates/ahk/reference/ahk-ks.surface.tsv",
}

# The catalogue of every dialect of the subtree. The first name that is
# there is the one that is read.
CATALOGUE = {
    "v2": ("v2.md",),
    "v1": ("v1.md",),
    "ks": ("keysharp.md", "ks.md"),
}

# The files of the generated subtree that the generator writes. The
# `README.md` is hand-written and is not required here.
GENERATED_FILES = ("index.md", "crosswalk.md")

# The generator of the subtree, and the flag that makes it compare its own
# output with the files that are there.
GENERATOR = "scripts/gen-surface-docs.py"

# The marker the generator writes its counts with.
MARKER = re.compile(r"surface-counts:\s*(?P<body>[^>]*?)\s*-->")
PAIR = re.compile(r"([a-z_]+)\s*=\s*(\d+)")

# The four numbers of a dialect, and the words that carry them.
LABELS = ("total", "values", "refusals", "pending")
LABEL_WORDS = {
    "total": r"total|entries|names",
    "values": r"values?|answer(?:s|ed)? with a value",
    "refusals": r"refus\w+",
    "pending": r"pending",
}


def the_binary(asked):
    """The runner to ask: the one that was given, or the release build.

    None is returned when there is no runner in the tree and none was asked
    for: the numbers of the tree are then not there to be compared with, and
    the check says so instead of dying.
    """
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
    return None


def the_surface_of(binary, dialect):
    """The surface of one dialect, as the runner answers it."""
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
    return surface


def read_surface(binary, dialect):
    """The summary of one dialect, and the distinct names of it."""
    surface = the_surface_of(binary, dialect)
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


# ------------------------------------------------------------ the subtree

def read_reference(dialect):
    """The names of the reference table of one dialect.

    Every row is a name of the surface of the binary, with the class of it
    when it is a method of one. The name of a method is the name of it with
    the class of it in front, as `Map.Clone`. A name that the table names
    twice is one name.
    """
    path = ROOT / REFERENCE[dialect]
    if not path.exists():
        return None
    names = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        columns = line.split("\t")
        if columns[0] == "kind":
            continue  # the head of the table
        name = columns[2].strip()
        if not name:
            continue
        klass = columns[1].strip()
        names.append((klass + "." + name) if klass else name)
    seen = set()
    unique = []
    for name in names:
        if name not in seen:
            seen.add(name)
            unique.append(name)
    return unique


def the_states_of(binary, dialect):
    """The state of every name of a dialect, as the runner answers it.

    A name that the tree carries twice (the getter and the setter of a
    property) is one name, and it is a name that answers with a value when
    one of its rows answers with a value.
    """
    surface = the_surface_of(binary, dialect)
    states = {}
    for entry in surface["entries"]:
        name = entry["name"]
        if entry.get("class"):
            name = entry["class"] + "." + name
        state = entry.get("state", "pending")
        old = states.get(name)
        if old is None or (old != "value" and state == "value"):
            states[name] = state
    return states


def the_subtree(where):
    """The generated subtree: the directory of it and the files in it."""
    directory = ROOT / where
    if not directory.is_dir():
        return None, {}
    files = {}
    for path in sorted(directory.glob("*.md")):
        files[path.name] = path
    families = {}
    family_directory = directory / "families"
    if family_directory.is_dir():
        for path in sorted(family_directory.glob("*.md")):
            families[path.stem] = path
    return directory, {"files": files, "families": families}


def the_catalogue(files, dialect):
    """The file of the catalogue of one dialect, when it is there."""
    for name in CATALOGUE[dialect]:
        if name in files:
            return files[name]
    return None


def mentions(text, name):
    """Whether the text carries the name, whole.

    A method may be written with the class of it in front (`Map.Clone`) or
    under the heading of the class (`Clone`); both are the name of it.
    """
    plain = name.rsplit(".", 1)[-1]
    for candidate in (name, plain):
        if re.search(r"(?<![A-Za-z0-9_])" + re.escape(candidate) + r"(?![A-Za-z0-9_])", text):
            return True
    return False


def is_a_separator(cells):
    """Whether the cells of a row are the dashes of a table."""
    return bool(cells) and all(re.match(r"^:?-+:?$", cell) for cell in cells)


def the_tables(text):
    """The tables of a text: a head of cells and the rows under it."""
    tables = []
    current = None
    for number, line in enumerate(text.splitlines(), 1):
        if not line.startswith("|"):
            current = None
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if is_a_separator(cells):
            continue
        if current is None:
            current = {"head": cells, "rows": [], "line": number}
            tables.append(current)
        else:
            current["rows"].append((number, cells))
    return tables


def the_table(text, wanted):
    """The first table of a text whose head carries all the words given."""
    for table in the_tables(text):
        head = [cell.lower() for cell in table["head"]]
        if all(word in head for word in wanted):
            return table
    return None


def the_column(table, word):
    """The number of the column of a table that carries a word."""
    for index, cell in enumerate(table["head"]):
        if cell.strip().lower() == word:
            return index
    return None


def the_number(cells, index):
    """The number of a cell, when the cell carries one."""
    if index is None or index >= len(cells):
        return None
    found = re.search(r"\d+", cells[index])
    return int(found.group(0)) if found else None


def the_name_of_a_cell(cell):
    """The name of a cell of a table: the text in the backticks of it."""
    found = re.search(r"`([^`]+)`", cell)
    if found:
        return found.group(1)
    link = re.search(r"\[([^\]]+)\]", cell)
    if link:
        return link.group(1).strip("` ")
    return cell.strip().strip("` ")


def the_dialect_of_a_label(label):
    """The dialect a label of the index names."""
    lowered = label.lower()
    if "keysharp" in lowered:
        return "ks"
    if "v2" in lowered or "2.0.19" in lowered:
        return "v2"
    if "v1" in lowered or "1.1.37" in lowered:
        return "v1"
    return None


def the_expected(dialect, names, states_of):
    """The counts of the tree for the names of one dialect.

    The names come from the reference table and the state of every name
    comes from the runner. A name of the KeySharp table that the dialect
    of KeySharp does not carry is a name of the v1 -- the table of KeySharp
    says so -- and it is a name of the KeySharp surface that the tree
    carries, but not one that answers with a value of its own.
    """
    counts = {"total": 0, "values": 0, "refusals": 0, "pending": 0}
    unknown = []
    own = states_of.get(dialect, {})
    for name in names:
        state = own.get(name)
        if state is None and dialect == "ks":
            state = states_of.get("v1", {}).get(name)
            if state is not None:
                # The name is carried by the v1; it answers there.
                counts["total"] += 1
                continue
        if state is None:
            unknown.append(name)
            continue
        counts["total"] += 1
        if state == "value":
            counts["values"] += 1
        elif state == "refusal":
            counts["refusals"] += 1
        else:
            counts["pending"] += 1
    return counts, unknown


def the_units(surfaces, expected):
    """The numbers of the tree, in the two units a document may use.

    The runner counts the rows of its surface: a name that it carries
    twice (the getter and the setter of a property) is two rows. The
    reference tables count the names: it is one name. A document may print
    either of the two, and the check takes the one that agrees.
    """
    units = {}
    for dialect in SURFACE_DIALECTS:
        if dialect not in expected:
            continue
        runner = {}
        for label in LABELS:
            runner[label] = surfaces.get(dialect, {}).get(label)
        units[dialect] = {"the rows of the runner": runner,
                          "the names of the reference": expected[dialect]}
    return units


def the_difference(found, units):
    """The unit the numbers of a document agree with, and the differences."""
    for name, unit in units.items():
        if all(found[label] == unit[label] for label in found if label in unit):
            return name, []
    problems = []
    for label, got in found.items():
        alternatives = {name: unit[label] for name, unit in units.items()
                        if label in unit}
        if alternatives and all(value != got for value in alternatives.values()):
            problems.append((label, got, alternatives))
    return None, problems


def the_counts_of_a_line(line):
    """The four numbers of a line, when the line carries them."""
    counts = {}
    for label in LABELS:
        words = LABEL_WORDS[label]
        forward = re.search(r"(?:" + words + r")[^\d]{0,40}?(\d+)", line, re.I)
        if forward:
            counts[label] = int(forward.group(1))
            continue
        backward = re.search(r"(\d+)[^\d]{0,20}?(?:" + words + r")", line, re.I)
        if backward:
            counts[label] = int(backward.group(1))
    return counts if len(counts) == len(LABELS) else None


def the_order_of_a_head(head):
    """The order of the labels of a table head, when it carries the four."""
    found = []
    for label in LABELS:
        found_at = None
        for index, cell in enumerate(head):
            if re.search(LABEL_WORDS[label], cell, re.I):
                found_at = index
                break
        if found_at is None:
            return None
        found.append((found_at, label))
    found.sort()
    return [label for _, label in found]


def the_count_blocks(text):
    """The count blocks of a text.

    Every block is a line number, the dialect it names (None when it names
    none) and the four numbers it carries.
    """
    blocks = []
    lines = text.splitlines()
    for number, line in enumerate(lines, 1):
        marker = MARKER.search(line)
        if marker:
            counts = {}
            dialect = None
            for key, value in PAIR.findall(marker.group("body")):
                if key == "dialect":
                    dialect = value
                elif key in LABELS:
                    counts[key] = int(value)
            if len(counts) == len(LABELS):
                blocks.append((number, dialect, counts))
            continue
        counts = the_counts_of_a_line(line)
        if counts:
            blocks.append((number, None, counts))
            continue
        # A table of the counts: the head carries the labels and the row
        # under it carries the numbers.
        order = the_order_of_a_head([cell for cell in line.strip().strip("|").split("|")])
        if order:
            for ahead in range(number + 1, min(number + 3, len(lines)) + 1):
                numbers = re.findall(r"\d+", lines[ahead - 1])
                if len(numbers) >= len(order):
                    blocks.append(
                        (ahead, None, {label: int(numbers[index])
                                       for index, label in enumerate(order)})
                    )
                    break
    return blocks


def check_the_names(subtree, where, failures):
    """Every name of the reference tables is carried by the subtree."""
    files = subtree["files"]
    families = subtree["families"]
    pages = {}
    for name, path in files.items():
        pages[where + "/" + name] = path.read_text(encoding="utf-8")
    for family, path in families.items():
        pages[where + "/families/" + family + ".md"] = path.read_text(encoding="utf-8")
    family_text = [text for path, text in pages.items() if "/families/" in path]
    for dialect in SURFACE_DIALECTS:
        names = read_reference(dialect)
        if names is None:
            print("MISSING %s: the reference table of the %s is not there"
                  % (REFERENCE[dialect], dialect))
            failures += 1
            continue
        catalogue = the_catalogue(files, dialect)
        if catalogue is None:
            print("MISSING %s: the catalogue of the %s is not there "
                  "(one of %s)"
                  % (where, dialect, ", ".join(CATALOGUE[dialect])))
            failures += 1
            continue
        missing = [name for name in names
                   if not any(mentions(text, name) for text in pages.values())]
        if missing:
            failures += 1
            print("MISSING NAMES of the %s: %d of the %d names of %s are not "
                  "carried by %s"
                  % (dialect, len(missing), len(names), REFERENCE[dialect], where))
            for name in missing:
                print("  the %s does not carry `%s`; it has to be in %s/%s and "
                      "in the page of its family under %s/families/"
                      % (dialect, name, where, CATALOGUE[dialect][0], where))
        without = [name for name in names
                   if not any(mentions(text, name) for text in family_text)]
        if without:
            failures += 1
            print("MISSING FICHAS of the %s: %d of the %d names have no page of "
                  "a family under %s/families/"
                  % (dialect, len(without), len(names), where))
            for name in without:
                print("  the %s does not have a ficha for `%s`" % (dialect, name))
        if not missing and not without:
            print("the %s: the %d names of %s are carried by %s/"
                  % (dialect, len(names), REFERENCE[dialect], where))
    return failures


def check_the_counts(subtree, where, units, failures):
    """The counts the subtree prints are the counts of the tree."""
    files = subtree["files"]
    for dialect in SURFACE_DIALECTS:
        if dialect not in units:
            continue
        catalogue = the_catalogue(files, dialect)
        if catalogue is None:
            continue  # the name check has already said that it is missing
        text = catalogue.read_text(encoding="utf-8")
        table = the_table(text, ["family", "names"])
        if table is None:
            blocks = the_count_blocks(text)
            if not blocks:
                failures += 1
                print("DRIFT %s/%s: the page does not print the counts of the %s"
                      % (where, CATALOGUE[dialect][0], dialect))
                continue
            number, _, counts = blocks[0]
            found = {label: counts[label] for label in LABELS}
            unit, problems = the_difference(found, units[dialect])
            if unit is None:
                failures += 1
                print("DRIFT %s/%s line %d: the counts of the %s"
                      % (where, CATALOGUE[dialect][0], number, dialect))
                for label, got, alternatives in problems:
                    print("  the document says %s=%d; the tree says %s"
                          % (label, got, the_alternatives(alternatives)))
            continue
        # The table of the families: the numbers of the page are the sum
        # of the numbers of the rows of it.
        total = {"total": 0, "values": 0, "refusals": 0, "pending": 0}
        for number, cells in table["rows"]:
            for label, column in (("total", 1), ("values", 2), ("refusals", 3),
                                  ("pending", 4)):
                number_in_the_row = the_number(cells, column)
                if number_in_the_row is not None:
                    total[label] += number_in_the_row
        unit, problems = the_difference(total, units[dialect])
        if unit is None:
            failures += 1
            print("DRIFT %s/%s: the table of the families does not add up to "
                  "the surface of the %s" % (where, CATALOGUE[dialect][0], dialect))
            for label, got, alternatives in problems:
                print("  the document says %s=%d; the tree says %s"
                      % (label, got, the_alternatives(alternatives)))
            for number, cells in table["rows"]:
                if the_number(cells, 1) is None:
                    continue
                print("    the row of line %d: %s"
                      % (number, " | ".join(cells)))
        else:
            print("the %s: %s/%s carries the counts of the tree (%d names, "
                  "%d values, %d refusals, %d pending, counted as %s)"
                  % (dialect, where, CATALOGUE[dialect][0], total["total"],
                     total["values"], total["refusals"], total["pending"], unit))
    # The index of every name carries the numbers of the dialects too.
    index = files.get("index.md")
    if index is not None:
        text = index.read_text(encoding="utf-8")
        table = the_table(text, ["dialect", "names"])
        if table is None:
            print("NOTE %s/index.md: the page does not print the numbers of the "
                  "dialects; they are not compared" % where)
        else:
            for number, cells in table["rows"]:
                dialect = the_dialect_of_a_label(cells[0]) if cells else None
                if dialect is None or dialect not in units:
                    continue
                found = {}
                for label, column in (("total", 1), ("refusals", 2), ("pending", 3)):
                    got = the_number(cells, column)
                    if got is not None:
                        found[label] = got
                if not found:
                    continue
                unit, problems = the_difference(found, units[dialect])
                if unit is None:
                    failures += 1
                    print("DRIFT %s/index.md line %d: the numbers of the %s"
                          % (where, number, dialect))
                    for label, got, alternatives in problems:
                        print("  the document says %s=%d; the tree says %s"
                              % (label, got, the_alternatives(alternatives)))
                else:
                    print("the %s: %s/index.md carries the counts of the tree "
                          "(counted as %s)" % (dialect, where, unit))
    return failures


def the_alternatives(alternatives):
    """The numbers of the tree of a label, in the two units."""
    return " or ".join("%d (%s)" % (value, name)
                       for name, value in sorted(alternatives.items()))


def the_state_of(text):
    """The state of a cell: `value`, `refusal` or `pending`.

    The page writes the reason of a refusal beside the word of it, as in
    "refused: this is a name of Windows", and the word of it is the state.
    """
    lowered = text.strip().lower()
    if lowered.startswith("value") or lowered == "value":
        return "value"
    if lowered.startswith("refus"):
        return "refusal"
    if lowered.startswith("pend"):
        return "pending"
    return ""


def check_the_families(subtree, where, failures):
    """The pages of the families carry what the catalogue says of them."""
    files = subtree["files"]
    families = subtree["families"]
    for family, path in families.items():
        text = path.read_text(encoding="utf-8")
        table = the_table(text, ["name", "dialect"])
        if table is None:
            print("NOTE %s/families/%s.md: the page does not carry a table of "
                  "names; nothing is compared" % (where, family))
            continue
        name_column = the_column(table, "name")
        dialect_column = the_column(table, "dialect")
        state_column = the_column(table, "state")
        # A name that the tree carries twice (the getter and the setter of
        # a property) is one name, and it answers with a value when one of
        # its rows does.
        carried = {}
        for number, cells in table["rows"]:
            if name_column is None or name_column >= len(cells):
                continue
            dialect = the_dialect_of_a_label(cells[dialect_column]) \
                if dialect_column is not None and dialect_column < len(cells) else None
            if dialect is None:
                continue
            name = the_name_of_a_cell(cells[name_column])
            state = the_state_of(cells[state_column]) \
                if state_column is not None and state_column < len(cells) else ""
            old = carried.get((dialect, name))
            if old is None or (old != "value" and state == "value"):
                carried[(dialect, name)] = state
        counted = {}
        for (dialect, name), state in carried.items():
            bucket = counted.setdefault(dialect, {"total": 0, "values": 0,
                                                   "refusals": 0, "pending": 0})
            bucket["total"] += 1
            if state == "value":
                bucket["values"] += 1
            elif state == "refusal":
                bucket["refusals"] += 1
            elif state == "pending":
                bucket["pending"] += 1
        # The catalogue of the dialect has to carry a row of this family.
        for dialect in SURFACE_DIALECTS:
            catalogue = the_catalogue(files, dialect)
            if catalogue is None:
                continue
            catalogue_text = catalogue.read_text(encoding="utf-8")
            table_of_families = the_table(catalogue_text, ["family", "names"])
            if table_of_families is None:
                continue
            linked = False
            for number, cells in table_of_families["rows"]:
                if family + ".md" in cells[0]:
                    linked = True
                    printed = {"total": the_number(cells, 1), "values": the_number(cells, 2),
                               "refusals": the_number(cells, 3), "pending": the_number(cells, 4)}
                    got = counted.get(dialect)
                    if got is None:
                        print("NOTE %s/%s: the page of the family %s has no name "
                              "of the %s" % (where, CATALOGUE[dialect][0], family, dialect))
                        continue
                    wrong = [(label, printed[label], got[label]) for label in LABELS
                             if printed[label] is not None and printed[label] != got[label]]
                    if wrong:
                        failures += 1
                        print("DRIFT %s/%s line %d: the row of the family %s"
                              % (where, CATALOGUE[dialect][0], number, family))
                        for label, said, counted_value in wrong:
                            print("  the document says %s=%d; the page of the "
                                  "family says %s=%d"
                                  % (label, said, label, counted_value))
            if not linked:
                failures += 1
                print("DRIFT %s/%s: the family %s has no row in the table of the "
                      "families" % (where, CATALOGUE[dialect][0], family))
    return failures


def check_the_links(subtree, where, failures):
    """Every family page the subtree links to is there."""
    for name, path in subtree["files"].items():
        text = path.read_text(encoding="utf-8")
        for target in set(re.findall(r"\(families/([^)]+\.md)\)", text)):
            if not (ROOT / where / "families" / target).exists():
                failures += 1
                print("DRIFT %s/%s: the page links to families/%s, and that "
                      "page is not there" % (where, name, target))
    return failures


def check_the_generator(failures):
    """The generated files are the ones the generator writes."""
    path = ROOT / GENERATOR
    if not path.exists():
        print("NOTE %s is not there: the sync of the subtree is not checked"
              % GENERATOR)
        return failures
    source = path.read_text(encoding="utf-8", errors="replace")
    if "--check" not in source:
        print("NOTE %s does not take a --check: the sync of the subtree is not "
              "checked" % GENERATOR)
        return failures
    result = subprocess.run(
        [sys.executable, str(path), "--check"],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        timeout=600,
    )
    output = (result.stdout or "").strip().splitlines()
    for line in output[:20]:
        print("  the generator: %s" % line)
    if len(output) > 20:
        print("  the generator: ... and %d more lines" % (len(output) - 20))
    if result.returncode != 0:
        failures += 1
        print("DRIFT %s --check: the generator says the subtree is not the one "
              "it writes (exit %d)" % (GENERATOR, result.returncode))
        if (result.stderr or "").strip():
            for line in result.stderr.strip().splitlines()[-5:]:
                print("  the generator: %s" % line)
    else:
        print("the generator: the subtree is the one it writes")
    return failures


def main():
    parser = argparse.ArgumentParser(
        description="check that the documents carry the numbers of the tree"
    )
    parser.add_argument("--binary", default=None, metavar="PATH",
                        help="the runner to ask (default: target/release/ahk)")
    parser.add_argument("--fix", action="store_true",
                        help="write the numbers of the runner into the documents")
    parser.add_argument("--docs-dir", default="docs/surface", metavar="PATH",
                        help="the generated subtree (default: docs/surface)")
    parser.add_argument("--skip-families", action="store_true",
                        help="do not compare the pages of the families")
    arguments = parser.parse_args()

    binary = the_binary(arguments.binary)
    surfaces = {}
    states = {}
    if binary is None:
        print("no runner under target/release or target/debug: the numbers of "
              "the tree are not there to be compared with (build it with "
              "`cargo build --release -p ahk-cli`)")
    else:
        print("the runner: %s" % binary)
        for dialect in DIALECTS:
            surfaces[dialect] = read_surface(binary, dialect)
        for dialect in SURFACE_DIALECTS:
            if dialect not in surfaces:
                try:
                    surfaces[dialect] = read_surface(binary, dialect)
                except SystemExit as error:
                    print("the runner does not answer for the %s: %s" % (dialect, error))
        for dialect in SURFACE_DIALECTS:
            counts = surfaces.get(dialect)
            if not counts:
                continue
            print("the %s: %d entries, %d values, %d refusals, %d pending, %d distinct names"
                  % (dialect, counts["total"], counts["values"], counts["refusals"],
                     counts["pending"], counts["distinct"]))

    failures = 0
    changed = set()
    if binary is None:
        print("the checks of the documents are skipped: they need the runner")
    else:
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

    # The generated subtree of the surface.
    directory, subtree = the_subtree(arguments.docs_dir)
    if directory is None:
        print()
        print("MISSING %s: the generated subtree of the surface is not there "
              "(the generator is %s)" % (arguments.docs_dir, GENERATOR))
        return 1
    print("the subtree: %s" % arguments.docs_dir)
    for name in GENERATED_FILES:
        if name not in subtree["files"]:
            failures += 1
            print("MISSING %s/%s: the generated file is not there"
                  % (arguments.docs_dir, name))
    if not subtree["families"]:
        failures += 1
        print("MISSING %s/families: the pages of the families are not there"
              % arguments.docs_dir)

    failures = check_the_names(subtree, arguments.docs_dir, failures)

    expected = {}
    if binary is None:
        print("the counts of the subtree are not compared: the runner is not there")
    else:
        states_of = {}
        for dialect in SURFACE_DIALECTS:
            try:
                states_of[dialect] = the_states_of(binary, dialect)
            except SystemExit as error:
                print("the runner does not answer for the %s: %s" % (dialect, error))
        for dialect in SURFACE_DIALECTS:
            names = read_reference(dialect)
            if names is None:
                continue
            wanted, unknown = the_expected(dialect, names, states_of)
            expected[dialect] = wanted
            if unknown:
                failures += 1
                print("DRIFT the %s: the tree does not carry %d of the names of "
                      "%s" % (dialect, len(unknown), REFERENCE[dialect]))
                for name in unknown:
                    print("  the tree does not carry `%s` of the %s"
                          % (name, dialect))
        units = the_units(surfaces, expected)
        failures = check_the_counts(subtree, arguments.docs_dir, units, failures)
        if not arguments.skip_families:
            failures = check_the_families(subtree, arguments.docs_dir, failures)
    failures = check_the_links(subtree, arguments.docs_dir, failures)
    failures = check_the_generator(failures)

    if failures:
        print()
        print("%d checks failed: the documents do not carry the numbers of the "
              "tree, or the generated subtree is not the one the tree has."
              % failures)
        print("Run `python scripts/check-surface-docs.py --fix` to write the "
              "numbers of the runner into the documents, or update the prose by hand.")
        return 1
    print("the subtree is the one the tree has, and the documents carry the "
          "numbers of the tree"
          if binary is not None else
          "the subtree is the one the tree has (the numbers of the documents "
          "were not compared: there is no runner)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
