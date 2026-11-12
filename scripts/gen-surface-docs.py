#!/usr/bin/env python3
"""Write the surface documents: every name of the v1, the v2 and KeySharp.

    python scripts/gen-surface-docs.py            # write docs/surface/
    python scripts/gen-surface-docs.py --check    # fail when the tree moved
    python scripts/gen-surface-docs.py --out DIR  # write somewhere else

The tree of the AutoHotkey dialects has three surfaces: the v1.1.37.02,
the v2.0.19 and the extensions of KeySharp. The tables of them are the
contract of the tree -- `crates/ahk/reference/*.surface.tsv` -- and the
state of every name in this tree is the answer of the runner:
`ahk --surface --json --dialect=v2|v1|ks`.

The documents this tool writes are the CHM of the tree: every name of the
three dialects, in one place, with the kind of it, the class of it, the
parameters of it, and what this tree answers for it today.

    docs/surface/index.md          the index of every name, alphabetical
    docs/surface/crosswalk.md      the v1 <-> v2 <-> KeySharp crosswalk
    docs/surface/families/<f>.md   one page per family: the ficha of
                                   every name of the family, of the three
                                   dialects, in one table
    docs/surface/v2.md             the summary of one dialect: the numbers
    docs/surface/v1.md             by family and by kind, and the names
    docs/surface/keysharp.md       that are refused or pending

`docs/surface/README.md` is written by hand and is not touched here.

The families are the areas of the surface -- the files, the input, the
windows, the screen, the native calls, the registry, the COM objects, the
core of the language. They come from
`crates/ahk/reference/ahk-v2.0.19.areas.py`, the file that writes the
capability report, so the two cannot disagree about an area.

Python 3 standard library only, no dependencies.
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import surface_areas  # noqa: E402  (the path is set above)

ROOT = Path(__file__).resolve().parent.parent
REFERENCE = ROOT / "crates" / "ahk" / "reference"
DEFAULT_OUT = ROOT / "docs" / "surface"

DIALECTS = (
    ("v2", "AutoHotkey v2.0.19", "ahk-v2.0.19.surface.tsv"),
    ("v1", "AutoHotkey v1.1.37.02", "ahk-v1.1.37.02.surface.tsv"),
    ("ks", "KeySharp", "ahk-ks.surface.tsv"),
)


# --- the tables ---------------------------------------------------------


def read_surface(path):
    """The rows of a surface table: the comments are not rows."""
    header = None
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.split("\t")
        if header is None:
            header = fields
            continue
        row = dict(zip(header, fields))
        row["_fields"] = fields
        rows.append(row)
    return rows


def the_binary(asked):
    """The runner to ask: the one that was given, or the build of the tree."""
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


def the_states(binary, dialect):
    """The state of every name of a dialect, as the runner answers it.

    The answer of the runner is the state of the tree: a value, a refusal
    with its reason, or a name to do. A tree without a runner has no
    state, and the documents say so instead of guessing.
    """
    if binary is None:
        return {}
    out = subprocess.run(
        [str(binary), "--surface", "--json", "--dialect=" + dialect],
        capture_output=True, text=True, check=False,
    )
    if out.returncode != 0:
        return {}
    try:
        payload = json.loads(out.stdout)
    except ValueError:
        return {}
    states = {}
    for entry in payload.get("entries", []):
        key = (entry.get("name", ""), entry.get("class", "") or "")
        states[key] = entry
    return states


# --- the shape of a name ------------------------------------------------


class Name(object):
    """One name of one dialect, with everything the documents say of it."""

    def __init__(self, dialect, row):
        self.dialect = dialect
        self.kind = row.get("kind", "")
        self.name = row.get("name", "")
        self.klass = row.get("class", "") or ""
        self.min = row.get("min", "")
        self.max = row.get("max", "")
        self.shape = row.get("shape", "")
        self.variant = row.get("variant", "")
        self.flag = row.get("flag", "")
        self.state = ""
        self.reason = ""

    @property
    def key(self):
        return (self.name, self.klass)

    @property
    def parameters(self):
        """The parameters of the name, as the documents print them."""
        if self.min == "" and self.max == "":
            return ""
        if self.max == "255" or self.max == "":
            return "up to %s" % (self.min or "any")
        if self.min == self.max:
            return self.min
        return "%s to %s" % (self.min, self.max)

    @property
    def state_text(self):
        if self.state == "":
            return "not measured"
        if self.state == "value":
            return "value"
        if self.state == "pending":
            return "pending"
        if self.reason:
            return "refused: %s" % self.reason
        return self.state

    @property
    def label(self):
        """The name as a script writes it."""
        if self.klass:
            return "%s.%s" % (self.klass, self.name)
        return self.name


def load(dialect, table, states):
    """The names of a dialect, with the state of the tree on each."""
    names = []
    seen = set()
    for row in read_surface(REFERENCE / table):
        name = Name(dialect, row)
        entry = states.get(name.key)
        if entry is None:
            # a name of a class that the runner answers without a class
            entry = states.get((name.name, ""))
        if entry is not None:
            name.state = entry.get("state", "")
            name.reason = entry.get("reason", "") or ""
        if name.key in seen:
            continue
        seen.add(name.key)
        names.append(name)
    return names


def area_of(name):
    """The family of a name, and whether a rule placed it."""
    area = surface_areas.area_of(name.name, name.dialect, name.klass)
    placed = area is not None and name.name not in surface_areas.V2_NAME_AREA
    return area, placed


# --- the pages ----------------------------------------------------------


def slug(text):
    """The anchor of a heading, the way GitHub makes one."""
    out = []
    for char in text.lower():
        if char.isalnum() or char in "-_":
            out.append(char)
        elif char in " .":
            out.append("-")
    return "".join(out)


def the_header(title, intro):
    return ["# %s" % title, "", intro, ""]


def write_page(path, lines):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")


def page_index(names_by_dialect, out):
    """The index of every name of the three dialects, alphabetical."""
    lines = the_header(
        "The index of every name",
        "Every name of the three dialects, alphabetical, with the family it "
        "belongs to and what this tree answers for it. The **ficha** of a "
        "name -- its kind, its class, its parameters -- is in the page of "
        "its family; this page is the way in.",
    )
    lines += [
        "| Dialect | Names | Refused | Pending |",
        "|---|---|---|---|",
    ]
    for dialect, title, _ in DIALECTS:
        names = names_by_dialect[dialect]
        refused = len([n for n in names if n.state == "refusal"])
        pending = len([n for n in names if n.state == "pending"])
        lines.append(
            "| %s | %d | %d | %d |"
            % (title, len(names), refused, pending)
        )
    lines.append("")

    for dialect, title, _ in DIALECTS:
        names = names_by_dialect[dialect]
        lines += ["## %s" % title, ""]
        lines += [
            "| Name | Kind | Class | Family | State |",
            "|---|---|---|---|---|",
        ]
        for name in sorted(names, key=lambda n: (n.name.lower(), n.klass)):
            area, _ = area_of(name)
            family = surface_areas.AREA_TITLES.get(area, "unclassified")
            link = "families/%s.md" % (area or "unclassified")
            lines.append(
                "| [`%s`](%s) | %s | %s | %s | %s |"
                % (
                    name.label,
                    link,
                    name.kind,
                    name.klass or "--",
                    family,
                    name.state_text,
                )
            )
        lines.append("")
    write_page(out / "index.md", lines)


def page_crosswalk(names_by_dialect, out):
    """The v1 <-> v2 <-> KeySharp crosswalk."""
    lines = the_header(
        "The crosswalk of the three dialects",
        "The same name in the three dialects, and the name that only one of "
        "them has. A name of the v1 that the v2 spells another way is a "
        "line of the v1-only table, with the v2 name that replaced it.",
    )
    v2 = {n.name for n in names_by_dialect["v2"]}
    v1 = {n.name for n in names_by_dialect["v1"]}
    ks = {n.name for n in names_by_dialect["ks"]}

    shared = sorted(v1 & v2)
    only_v1 = sorted(v1 - v2)
    only_ks = sorted(ks - v2)

    lines += [
        "| The three | Names |",
        "|---|---|",
        "| in the v1 and in the v2 | %d |" % len(shared),
        "| only in the v1 | %d |" % len(only_v1),
        "| only in the KeySharp | %d |" % len(only_ks),
        "",
        "## In the v1 and in the v2",
        "",
        "| Name | Family |",
        "|---|---|",
    ]
    for name in shared:
        area = surface_areas.area_of(name, "v1") or ""
        lines.append(
            "| `%s` | %s |" % (name, surface_areas.AREA_TITLES.get(area, "unclassified"))
        )
    lines += ["", "## Only in the v1", "", "| Name | Kind | Family |", "|---|---|---|"]
    for entry in names_by_dialect["v1"]:
        if entry.name in only_v1:
            area, _ = area_of(entry)
            lines.append(
                "| `%s` | %s | %s |"
                % (entry.name, entry.kind, surface_areas.AREA_TITLES.get(area, "unclassified"))
            )
    lines += ["", "## Only in the KeySharp", "", "| Name | Kind | Family |", "|---|---|---|"]
    for entry in names_by_dialect["ks"]:
        if entry.name in only_ks:
            area, _ = area_of(entry)
            lines.append(
                "| `%s` | %s | %s |"
                % (entry.name, entry.kind, surface_areas.AREA_TITLES.get(area, "unclassified"))
            )
    write_page(out / "crosswalk.md", lines)


def page_families(names_by_dialect, out):
    """One page per family: the ficha of every name of it."""
    families = surface_areas.families_of()
    buckets = {}
    for dialect, _, _ in DIALECTS:
        for name in names_by_dialect[dialect]:
            area, _ = area_of(name)
            buckets.setdefault(area or "unclassified", []).append(name)

    for key, title in families:
        lines = the_header(
            title,
            "The names of the family **%s** in the three dialects, with what "
            "this tree answers for each of them. The family is the area a "
            "script thinks in; the names are the whole of it." % title,
        )
        names = sorted(
            buckets.get(key, []), key=lambda n: (n.name.lower(), n.klass)
        )
        lines += [
            "| Name | Dialect | Kind | Class | Parameters | State |",
            "|---|---|---|---|---|---|",
        ]
        for name in names:
            lines.append(
                "| `%s` | %s | %s | %s | %s | %s |"
                % (
                    name.label,
                    name.dialect,
                    name.kind,
                    name.klass or "--",
                    name.parameters or "--",
                    name.state_text,
                )
            )
        lines.append("")
        write_page(out / "families" / ("%s.md" % key), lines)

    # the names that no family claims
    leftovers = buckets.get("unclassified", [])
    if leftovers:
        lines = the_header(
            "Unclassified",
            "The names that no family claims. A name lands here when no "
            "table and no rule of `scripts/surface_areas.py` places it.",
        )
        lines += ["| Name | Dialect | Kind |", "|---|---|---|"]
        for name in sorted(leftovers, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s | %s |" % (name.label, name.dialect, name.kind))
        write_page(out / "families" / "unclassified.md", lines)


def page_dialect(dialect, title, names_by_dialect, out):
    """The summary of one dialect."""
    names = names_by_dialect[dialect]
    lines = the_header(
        "The surface of %s" % title,
        "The names of %s, counted by family and by kind, and the names "
        "that this tree refuses or does not carry. The ficha of every name "
        "is in the page of its family." % title,
    )
    lines += ["| Family | Names | Value | Refused | Pending |", "|---|---|---|---|---|"]
    by_family = {}
    for name in names:
        area, _ = area_of(name)
        by_family.setdefault(area or "unclassified", []).append(name)
    for key, family_title in surface_areas.families_of():
        entries = by_family.get(key, [])
        value = len([n for n in entries if n.state == "value"])
        refused = len([n for n in entries if n.state == "refusal"])
        pending = len([n for n in entries if n.state == "pending"])
        lines.append(
            "| [%s](families/%s.md) | %d | %d | %d | %d |"
            % (family_title, key, len(entries), value, refused, pending)
        )
    lines.append("")

    lines += ["## By kind", "", "| Kind | Names |", "|---|---|"]
    by_kind = {}
    for name in names:
        by_kind[name.kind] = by_kind.get(name.kind, 0) + 1
    for kind in sorted(by_kind):
        lines.append("| %s | %d |" % (kind, by_kind[kind]))
    lines.append("")

    refused = [n for n in names if n.state == "refusal"]
    if refused:
        lines += ["## Refused", "", "| Name | Reason |", "|---|---|"]
        for name in sorted(refused, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s |" % (name.label, name.reason))
        lines.append("")

    pending = [n for n in names if n.state == "pending"]
    if pending:
        lines += ["## Pending", "", "| Name | Kind |", "|---|---|"]
        for name in sorted(pending, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s |" % (name.label, name.kind))
        lines.append("")

    write_page(out / ("%s.md" % dialect), lines)


# --- the tool -----------------------------------------------------------


def generate(out):
    """Write the whole subtree into `out`; returns the files it wrote."""
    binary = the_binary(None)
    names_by_dialect = {}
    for dialect, _, table in DIALECTS:
        states = the_states(binary, dialect)
        names_by_dialect[dialect] = load(dialect, table, states)

    out.mkdir(parents=True, exist_ok=True)
    page_index(names_by_dialect, out)
    page_crosswalk(names_by_dialect, out)
    page_families(names_by_dialect, out)
    for dialect, title, _ in DIALECTS:
        page_dialect(dialect, title, names_by_dialect, out)

    return names_by_dialect


def report(names_by_dialect):
    """What the run found, on the stdout."""
    binary = the_binary(None)
    print("runner: %s" % (binary if binary else "none (no state was measured)"))
    for dialect, title, _ in DIALECTS:
        names = names_by_dialect[dialect]
        unclassified = [n for n in names if area_of(n)[0] is None]
        value = len([n for n in names if n.state == "value"])
        refused = len([n for n in names if n.state == "refusal"])
        pending = len([n for n in names if n.state == "pending"])
        print(
            "%-3s %-24s names=%d value=%d refused=%d pending=%d unclassified=%d"
            % (dialect, title, len(names), value, refused, pending, len(unclassified))
        )
        for name in unclassified:
            print("     unclassified: %s" % name.label)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", default=str(DEFAULT_OUT),
                        help="where the subtree is written (default: docs/surface)")
    parser.add_argument("--check", action="store_true",
                        help="do not write; fail when the tree moved")
    args = parser.parse_args()
    out = Path(args.out)
    if not out.is_absolute():
        out = ROOT / out

    if args.check:
        with tempfile.TemporaryDirectory() as tmp:
            names_by_dialect = generate(Path(tmp))
            wanted = sorted(
                p.relative_to(tmp) for p in Path(tmp).rglob("*.md")
            )
            have = sorted(
                p.relative_to(out) for p in out.rglob("*.md")
                if p.name != "README.md"
            )
            problems = []
            for name in wanted:
                if name not in have:
                    problems.append("missing: %s" % name)
                    continue
                a = (Path(tmp) / name).read_text(encoding="utf-8")
                b = (out / name).read_text(encoding="utf-8")
                if a != b:
                    problems.append("stale: %s" % name)
            for name in have:
                if name not in wanted:
                    problems.append("unknown: %s" % name)
            if problems:
                for problem in problems:
                    print(problem)
                print("the surface documents are out of date")
                return 1
        print("the surface documents are in sync")
        return 0

    names_by_dialect = generate(out)
    report(names_by_dialect)
    print("written in %s" % out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
