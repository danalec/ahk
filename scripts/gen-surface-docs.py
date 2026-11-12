#!/usr/bin/env python3
"""Write the surface documents: every name of the v1, the v2 and KeySharp.

    python scripts/gen-surface-docs.py            # write docs/surface/
    python scripts/gen-surface-docs.py --check    # fail when the tree moved
    python scripts/gen-surface-docs.py --lang es  # write one language only

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

The documents are written in five languages -- English, Spanish,
Portuguese of Brazil, Japanese and Chinese. The English ones are the
reference and live at the root of the subtree; a translation lives in a
directory of its own (`es/`, `pt-BR/`, `ja/`, `zh/`) with the same pages,
and every page carries the switcher of the languages.

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
import os
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import surface_areas  # noqa: E402  (the path is set above)
import surface_i18n  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
REFERENCE = ROOT / "crates" / "ahk" / "reference"
DEFAULT_OUT = ROOT / "docs" / "surface"

DIALECTS = (
    ("v2", "AutoHotkey v2.0.19", "ahk-v2.0.19.surface.tsv", "v2.md"),
    ("v1", "AutoHotkey v1.1.37.02", "ahk-v1.1.37.02.surface.tsv", "v1.md"),
    ("ks", "KeySharp", "ahk-ks.surface.tsv", "keysharp.md"),
)

# Where the run is writing, and the pages it wrote: the pages of an
# earlier run that this one did not write are removed.
CURRENT = {"out": None, "written": set()}


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
    """The state of every name of a dialect, and the numbers of the runner.

    The answer of the runner is the state of the tree: a value, a refusal
    with its reason, or a name to do. A tree without a runner has no
    state, and the documents say so instead of guessing.

    The numbers are the ones of the runner itself (`total`, `values`,
    `refusals`, `pending`), so the documents carry the same count that
    `ahk --surface` prints.
    """
    empty = {"total": 0, "values": 0, "refusals": 0, "pending": 0}
    if binary is None:
        return {}, empty
    out = subprocess.run(
        [str(binary), "--surface", "--json", "--dialect=" + dialect],
        capture_output=True, text=True, check=False,
    )
    if out.returncode != 0:
        return {}, empty
    try:
        payload = json.loads(out.stdout)
    except ValueError:
        return {}, empty
    states = {}
    for entry in payload.get("entries", []):
        key = (entry.get("name", ""), entry.get("class", "") or "")
        states[key] = entry
    summary = {
        "total": payload.get("total", 0),
        "values": payload.get("values", 0),
        "refusals": payload.get("refusals", 0),
        "pending": payload.get("pending", 0),
    }
    return states, summary


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
    def label(self):
        """The name as a script writes it."""
        if self.klass:
            return "%s.%s" % (self.klass, self.name)
        return self.name

    def state_text(self, lang):
        """What the tree answers for the name, in a language."""
        t = surface_i18n.t
        if self.state == "":
            return t(lang, "state_not_measured")
        if self.state == "value":
            return t(lang, "state_value")
        if self.state == "pending":
            return t(lang, "state_pending")
        if self.reason:
            return t(lang, "state_refused") % surface_i18n.reason(lang, self.reason)
        return self.state


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
    """The family of a name."""
    return surface_areas.area_of(name.name, name.dialect, name.klass)


# --- the paths of the languages -----------------------------------------


def lang_dir(out, lang):
    """The directory of a language: the root of the subtree for English."""
    return out if lang == "en" else out / lang


def link_to(out, from_lang, to_lang, rest):
    """The relative link from a page of one language to a page of another."""
    source = lang_dir(out, from_lang)
    target = lang_dir(out, to_lang) / rest
    return os.path.relpath(target.as_posix(), source.as_posix()).replace("\\", "/")


def switcher(out, lang):
    """The row of the languages, as the head of every page carries it."""
    parts = []
    for other in surface_i18n.LANGS:
        parts.append(
            "[%s](%s)"
            % (surface_i18n.LANG_NAMES[other], link_to(out, lang, other, "index.md"))
        )
    return "| %s |" % " | ".join(parts)


# --- the pages ----------------------------------------------------------


def the_header(out, lang, title, intro):
    t = surface_i18n.t
    return [
        "# %s" % title,
        "",
        switcher(out, lang),
        "",
        "%s: %s" % (t(lang, "languages"), surface_i18n.LANG_NAMES[lang]),
        "",
        intro,
        "",
    ]


def write_page(path, lines):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")
    if CURRENT["out"] is not None:
        CURRENT["written"].add(path.relative_to(CURRENT["out"]).as_posix())


def page_index(out, lang, names_by_dialect, summaries):
    """The index of every name of the three dialects, alphabetical."""
    t = surface_i18n.t
    lines = the_header(out, lang, t(lang, "index_title"), t(lang, "index_intro"))
    lines += [
        "| %s | %s | %s | %s |" % (t(lang, "head_dialect"), t(lang, "head_names"),
                                   t(lang, "head_refused"), t(lang, "head_pending")),
        "|---|---|---|---|",
    ]
    for dialect, title, _, _ in DIALECTS:
        names = names_by_dialect[dialect]
        summary = summaries.get(dialect, {})
        refused = summary.get("refusals", len([n for n in names if n.state == "refusal"]))
        pending = summary.get("pending", len([n for n in names if n.state == "pending"]))
        total = summary.get("total", len(names))
        lines.append(
            "| %s | %d | %d | %d |" % (title, total, refused, pending)
        )
    lines.append("")

    for dialect, title, _, _ in DIALECTS:
        names = names_by_dialect[dialect]
        lines += ["## %s" % title, ""]
        lines += [
            "| %s | %s | %s | %s | %s |"
            % (t(lang, "head_name"), t(lang, "head_kind"), t(lang, "head_class"),
               t(lang, "head_family"), t(lang, "head_state")),
            "|---|---|---|---|---|",
        ]
        for name in sorted(names, key=lambda n: (n.name.lower(), n.klass)):
            area = area_of(name) or "unclassified"
            page = "families/%s.md" % area
            lines.append(
                "| [`%s`](%s) | %s | %s | %s | %s |"
                % (
                    name.label,
                    page,
                    name.kind,
                    name.klass or "--",
                    surface_i18n.family_title(lang, area),
                    name.state_text(lang),
                )
            )
        lines.append("")
    write_page(lang_dir(out, lang) / "index.md", lines)


def page_crosswalk(out, lang, names_by_dialect):
    """The v1 <-> v2 <-> KeySharp crosswalk."""
    t = surface_i18n.t
    lines = the_header(out, lang, t(lang, "crosswalk_title"), t(lang, "crosswalk_intro"))
    v2 = {n.name for n in names_by_dialect["v2"]}
    v1 = {n.name for n in names_by_dialect["v1"]}
    ks = {n.name for n in names_by_dialect["ks"]}

    shared = sorted(v1 & v2)
    only_v1 = sorted(v1 - v2)
    only_ks = sorted(ks - v2)

    lines += [
        "| %s | %s |" % (t(lang, "the_three"), t(lang, "head_names")),
        "|---|---|",
        "| v1 and v2 | %d |" % len(shared),
        "| v1 only | %d |" % len(only_v1),
        "| KeySharp only | %d |" % len(only_ks),
        "",
        "## %s" % t(lang, "section_in_both"),
        "",
        "| %s | %s |" % (t(lang, "head_name"), t(lang, "head_family")),
        "|---|---|",
    ]
    for name in shared:
        area = surface_areas.area_of(name, "v1") or "unclassified"
        lines.append(
            "| `%s` | %s |" % (name, surface_i18n.family_title(lang, area))
        )
    lines += ["", "## %s" % t(lang, "section_only_v1"), "",
              "| %s | %s | %s |" % (t(lang, "head_name"), t(lang, "head_kind"),
                                    t(lang, "head_family")),
              "|---|---|---|"]
    for entry in names_by_dialect["v1"]:
        if entry.name in only_v1:
            area = area_of(entry) or "unclassified"
            lines.append(
                "| `%s` | %s | %s |"
                % (entry.name, entry.kind, surface_i18n.family_title(lang, area))
            )
    lines += ["", "## %s" % t(lang, "section_only_ks"), "",
              "| %s | %s | %s |" % (t(lang, "head_name"), t(lang, "head_kind"),
                                    t(lang, "head_family")),
              "|---|---|---|"]
    for entry in names_by_dialect["ks"]:
        if entry.name in only_ks:
            area = area_of(entry) or "unclassified"
            lines.append(
                "| `%s` | %s | %s |"
                % (entry.name, entry.kind, surface_i18n.family_title(lang, area))
            )
    write_page(lang_dir(out, lang) / "crosswalk.md", lines)


def page_families(out, lang, names_by_dialect):
    """One page per family: the ficha of every name of it."""
    t = surface_i18n.t
    families = surface_areas.families_of()
    buckets = {}
    for dialect, _, _, _ in DIALECTS:
        for name in names_by_dialect[dialect]:
            area = area_of(name) or "unclassified"
            buckets.setdefault(area, []).append(name)

    for key, title in families:
        lines = the_header(
            out, lang,
            surface_i18n.family_title(lang, key),
            t(lang, "family_intro") % surface_i18n.family_title(lang, key),
        )
        names = sorted(buckets.get(key, []), key=lambda n: (n.name.lower(), n.klass))
        lines += [
            "| %s | %s | %s | %s | %s | %s |"
            % (t(lang, "head_name"), t(lang, "head_dialect"), t(lang, "head_kind"),
               t(lang, "head_class"), t(lang, "head_parameters"), t(lang, "head_state")),
            "|---|---|---|---|---|---|",
        ]
        for name in names:
            lines.append(
                "| `%s` | %s | %s | %s | %s | %s |"
                % (
                    name.label,
                    DIALECT_TITLE[name.dialect],
                    name.kind,
                    name.klass or "--",
                    name.parameters or "--",
                    name.state_text(lang),
                )
            )
        lines.append("")
        write_page(lang_dir(out, lang) / "families" / ("%s.md" % key), lines)

    leftovers = buckets.get("unclassified", [])
    if leftovers:
        lines = the_header(
            out, lang,
            t(lang, "unclassified_title"),
            t(lang, "unclassified_intro"),
        )
        lines += ["| %s | %s | %s |" % (t(lang, "head_name"), t(lang, "head_dialect"),
                                       t(lang, "head_kind")),
                  "|---|---|---|"]
        for name in sorted(leftovers, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s | %s |" % (name.label, name.dialect, name.kind))
        write_page(lang_dir(out, lang) / "families" / "unclassified.md", lines)


def page_dialect(out, lang, dialect, title, names_by_dialect, summaries):
    """The summary of one dialect."""
    t = surface_i18n.t
    names = names_by_dialect[dialect]
    summary = summaries.get(dialect, {})
    lines = the_header(
        out, lang,
        t(lang, "dialect_title") % title,
        t(lang, "dialect_intro") % title,
    )
    lines += [
        "| %s | %s | %s | %s |"
        % (t(lang, "head_names"), t(lang, "head_value"), t(lang, "head_refused"),
           t(lang, "head_pending")),
        "|---|---|---|---|",
        "| %d | %d | %d | %d |"
        % (summary.get("total", len(names)), summary.get("values", 0),
           summary.get("refusals", 0), summary.get("pending", 0)),
        "",
    ]
    lines += [
        "| %s | %s | %s | %s | %s |"
        % (t(lang, "head_family"), t(lang, "head_names"), t(lang, "head_value"),
           t(lang, "head_refused"), t(lang, "head_pending")),
        "|---|---|---|---|---|",
    ]
    by_family = {}
    for name in names:
        by_family.setdefault(area_of(name) or "unclassified", []).append(name)
    for key, _ in surface_areas.families_of():
        entries = by_family.get(key, [])
        value = len([n for n in entries if n.state == "value"])
        refused = len([n for n in entries if n.state == "refusal"])
        pending = len([n for n in entries if n.state == "pending"])
        lines.append(
            "| [%s](families/%s.md) | %d | %d | %d | %d |"
            % (surface_i18n.family_title(lang, key), key, len(entries),
               value, refused, pending)
        )
    lines.append("")

    lines += ["## %s" % t(lang, "section_by_kind"), "",
              "| %s | %s |" % (t(lang, "head_kind"), t(lang, "head_names")),
              "|---|---|"]
    by_kind = {}
    for name in names:
        by_kind[name.kind] = by_kind.get(name.kind, 0) + 1
    for kind in sorted(by_kind):
        lines.append("| %s | %d |" % (kind, by_kind[kind]))
    lines.append("")

    refused = [n for n in names if n.state == "refusal"]
    if refused:
        lines += ["## %s" % t(lang, "section_refused"), "",
                  "| %s | %s |" % (t(lang, "head_name"), t(lang, "head_reason")),
                  "|---|---|"]
        for name in sorted(refused, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s |" % (name.label, surface_i18n.reason(lang, name.reason)))
        lines.append("")

    pending = [n for n in names if n.state == "pending"]
    if pending:
        lines += ["## %s" % t(lang, "section_pending"), "",
                  "| %s | %s |" % (t(lang, "head_name"), t(lang, "head_kind")),
                  "|---|---|"]
        for name in sorted(pending, key=lambda n: (n.name.lower(), n.klass)):
            lines.append("| `%s` | %s |" % (name.label, name.kind))
        lines.append("")

    write_page(lang_dir(out, lang) / DIALECT_FILE[dialect], lines)


DIALECT_FILE = {dialect: file for dialect, _, _, file in DIALECTS}
DIALECT_TITLE = {dialect: title for dialect, title, _, _ in DIALECTS}


# --- the tool -----------------------------------------------------------


def load_names(binary):
    """The names of the three dialects, and the numbers of the runner."""
    names_by_dialect = {}
    summaries = {}
    for dialect, _, table, _ in DIALECTS:
        states, summary = the_states(binary, dialect)
        names_by_dialect[dialect] = load(dialect, table, states)
        summaries[dialect] = summary
    return names_by_dialect, summaries


def generate(out, langs=None):
    """Write the subtree into `out`; returns the names it wrote."""
    binary = the_binary(None)
    names_by_dialect, summaries = load_names(binary)
    langs = langs or surface_i18n.LANGS
    CURRENT["out"] = out
    CURRENT["written"] = set()
    written = []
    for lang in langs:
        page_index(out, lang, names_by_dialect, summaries)
        page_crosswalk(out, lang, names_by_dialect)
        page_families(out, lang, names_by_dialect)
        for dialect, title, _, _ in DIALECTS:
            page_dialect(out, lang, dialect, title, names_by_dialect, summaries)
    clean(out, CURRENT["written"])
    return names_by_dialect


def clean(out, written):
    """Remove the pages of an earlier run that this one did not write.

    The page of the root that is written by hand -- `README.md` -- is
    never removed.
    """
    if not out.exists():
        return
    for path in sorted(out.rglob("*.md")):
        relative = path.relative_to(out).as_posix()
        if relative == "README.md" or relative in written:
            continue
        path.unlink()


def report(names_by_dialect, summaries=None):
    """What the run found, on the stdout."""
    binary = the_binary(None)
    print("runner: %s" % (binary if binary else "none (no state was measured)"))
    for dialect, title, _, _ in DIALECTS:
        names = names_by_dialect[dialect]
        unclassified = [n for n in names if area_of(n) is None]
        value = len([n for n in names if n.state == "value"])
        refused = len([n for n in names if n.state == "refusal"])
        pending = len([n for n in names if n.state == "pending"])
        print(
            "%-3s %-24s names=%d value=%d refused=%d pending=%d unclassified=%d"
            % (dialect, title, len(names), value, refused, pending, len(unclassified))
        )
        for name in unclassified:
            print("     unclassified: %s" % name.label)


def check(out):
    """Fail when the subtree is not what the generator writes."""
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        generate(tmp)
        wanted = sorted(p.relative_to(tmp).as_posix() for p in tmp.rglob("*.md"))
        have = sorted(
            p.relative_to(out).as_posix()
            for p in out.rglob("*.md")
            if p.name != "README.md"
        )
        problems = []
        for name in wanted:
            if name not in have:
                problems.append("missing: %s" % name)
                continue
            a = (tmp / name).read_text(encoding="utf-8")
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


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", default=str(DEFAULT_OUT),
                        help="where the subtree is written (default: docs/surface)")
    parser.add_argument("--check", action="store_true",
                        help="do not write; fail when the tree moved")
    parser.add_argument("--lang", action="append",
                        help="write only this language (repeatable)")
    args = parser.parse_args()
    out = Path(args.out)
    if not out.is_absolute():
        out = ROOT / out

    if args.check:
        return check(out)

    names_by_dialect = generate(out, args.lang)
    report(names_by_dialect)
    print("written in %s" % out)
    return 0


if __name__ == "__main__":
    sys.exit(main())

