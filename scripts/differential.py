#!/usr/bin/env python3
"""Three-way differential harness: real AutoHotkey v2.0.19 vs Keysharp vs Rust `ahk`.

This is the committed, cross-platform port of the audit's PowerShell harness
(`egm-review/keysharp-audit/differential/harness.ps1`).  It reads the case
scripts from a directory (`scripts/differential/cases/*.ahk` by default), runs
each case in up to three engines, normalizes only what is engine plumbing, and
writes `results.tsv` + `summary.txt` (+ an optional markdown report).

Why three engines
-----------------
Rust `ahk` claims AutoHotkey **v2.0.19 exactly**.  Real AutoHotkey v2.0.19 is
the authority for that dialect; Keysharp is a second implementation of the same
language family and the project this tree measures itself against.  A case
whose normalized answer is identical in all three is a case where the tree's
observable behaviour is the dialect's.

Engines (configured by environment variable or flag)
----------------------------------------------------
  engine    environment variable   default
  real      AHK_REAL               <temp>/ahk-v2-measure/AutoHotkey64.exe
  keysharp  AHK_KEYSHARP           ../Keysharp/bin/Debug/net10.0-windows/Keysharp.exe
  rust      AHK_RUST               target/release/ahk.exe

The real AutoHotkey binary exists only on Windows.  On another platform pass
`--engines rust`, or `--engines rust,keysharp` when Keysharp is available, and
the harness compares the engines it has (the verdicts reduce to MATCH/DIFFERS;
see "Verdict rules" below).  `--allow-missing` turns a missing enabled engine
into a warning instead of an error.

Output mechanism
----------------
`FileAppend(Text, "*")` is the mechanism, and it is unambiguous in all three
engines: real AutoHotkey and Keysharp write it to the process stdout, and the
Rust runner collects it in its JSON report on stdout, under `output` (an array
of the exact appended strings).  The harness therefore compares, per case:

  real / Keysharp : normalized process stdout
  Rust            : concatenation of report.output (normalized)

When a case fails before the script can print (a load-time error in real AHK, a
parse error in Rust), the answer falls back to the process stderr (real,
Keysharp) or `report.error` (Rust), so the wording of a failure is compared as
data and not lost.

Safety invariants (do not remove)
---------------------------------
* Every case script starts with `#ErrorStdOut` and `#Warn All, StdOut`, keeps an
  `OnError` backstop that suppresses the error dialog, and wraps every probe in
  `try`/`catch as e`.  Real AHK shows a modal dialog for an uncaught runtime
  error and for a warning, and a dialog is a hang nothing can dismiss -- which
  is why every engine is also bounded by a timeout and killed on expiry.
* Every process is bounded by a timeout (`--timeout-real` 15 s,
  `--timeout-keysharp` 90 s, `--timeout-rust` 25 s) and the process tree is
  killed on expiry.  A timeout is recorded and classified `ERROR`; it is never
  ignored.
* No case may open a dialog, so the case set has no `MsgBox`/`InputBox`/
  `FileSelect`/`Gui.Show`, no hotkeys, no timers, no clipboard, no network, no
  screen or pointer access, and no `Sleep`.  `scripts/README.md` says how to add
  a case that keeps the invariant.
* Keysharp's `--validate` flag exists for syntax-only checks; pass `--validate`
  to add it to every Keysharp invocation.  A full differential run does not use
  it: a syntax check cannot compare values.

Normalization (only what is engine plumbing, never a value)
-----------------------------------------------------------
1. `\\r\\n` -> `\\n`; the trailing newline of an answer is trimmed for comparison
   (the raw stream is kept under `raw/`).
2. Diagnostic warning blocks are removed from stdout of every engine:
   `... (N) : ==> Warning: ...` plus a following `     Specifically: ...` line.
   Warning wording and location are engine diagnostics, not script output.  The
   Rust runner emits a warning for the `OnError` backstop name and for every
   `catch as e` variable in every script; real AHK and Keysharp warn only for
   names such as `NoSuchFunc`, and the wording differs.
3. Error-location prefixes are removed from stderr answers:
   `<script> (12) : ==> ` (real AHK and Keysharp runtime/load errors) and
   `<script> 12:3: ` (Keysharp compile errors).  The script path is replaced by
   `<script>`, the per-engine working directory by `<cwd>`, and the repository
   root by `<root>`.
4. Nothing else: no version strings, addresses or paths are rewritten inside a
   value.  The channel a failure came out of (`STDERR:` vs `ERROR:`) is not part
   of the comparison; the text is.

Verdict rules
-------------
  MATCH          every enabled engine's normalized answer is identical
  KEYS-DIFFERS   real == Rust, Keysharp differs
  RUST-DIFFERS   real == Keysharp, Rust differs
  BOTH-DIFFER    no two engines agree
  ERROR          a timeout, a crash exit, or an engine that did not answer
  DIFFERS        only two engines were enabled and they disagree

A baseline can be passed with `--baseline old/results.tsv`: the run then reports
which cases were FIXED (divergent before, MATCH now) and which REGRESSED (MATCH
before, divergent now), and `--check` makes a regression exit non-zero.  That is
the CI gate the plan asks for: "no new divergence vs the previous baseline".

Usage
-----
    python scripts/differential.py                     # everything
    python scripts/differential.py --only er0          # cases whose id contains
    python scripts/differential.py --report report.md  # render the markdown
    python scripts/differential.py --engines rust      # one engine (any platform)
    python scripts/differential.py --baseline old.tsv --check

The harness writes into `--out` (default `scripts/differential/`): `results.tsv`,
`summary.txt`, `raw/`, the report if asked, and `_tmp/` unless `--keep-tmp`.

The harness is stdlib only: no third-party packages, no shelling out except to
the configured engine binaries and (on Windows) `taskkill` for the process tree.
"""

from __future__ import annotations

import argparse
import datetime as _datetime
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
DEFAULT_CASES = HERE / "differential" / "cases"
DEFAULT_OUT = HERE / "differential"

ENGINES = ("real", "keysharp", "rust")
ENGINE_ENV = {"real": "AHK_REAL", "keysharp": "AHK_KEYSHARP", "rust": "AHK_RUST"}
DEFAULT_TIMEOUTS = {"real": 15, "keysharp": 90, "rust": 25}

# The verdicts of the PowerShell harness, plus DIFFERS for a two-engine run.
VERDICTS = ("MATCH", "KEYS-DIFFERS", "RUST-DIFFERS", "BOTH-DIFFER", "ERROR", "DIFFERS")

# A case id is a family prefix and a number: ex01_precedence.
FAMILIES = {
    "ex": "expr",
    "nu": "numbers",
    "st": "strings",
    "va": "vars",
    "fu": "funcs",
    "cl": "classes",
    "ob": "objects",
    "co": "control",
    "fi": "fileio",
    "mi": "misc",
    "er": "errors",
    "di": "directives",
    "hk": "hotkeys",
    "hs": "hotstrings",
    "su": "suspend",
}

FAMILY_COVERAGE = {
    "expr": "precedence, implicit concat, comparison, logical/ternary, bitwise, compound assignment, ??",
    "numbers": "integer division/modulo, **, Float rendering, hex, Round, math funcs, types",
    "strings": "SubStr, InStr, StrReplace, Trim, SplitPath, case conversion, StrSplit, RegEx, Format",
    "vars": "global/local/static, by-ref swap, unset/IsSet, ++/--",
    "funcs": "defaults, variadics, recursion, nested closures, Func/BoundFunc, fat arrows, callbacks",
    "classes": "__New, extends/super, static, property get/set, __Get, __Call, __Item",
    "objects": "Object literal/DefineProp, Map, Array, Buffer, Clone, Map key types",
    "control": "Loop N/break/continue, While, Loop Until, Loop Parse, Switch, try/catch/finally, for-in, truthiness",
    "fileio": "FileAppend/FileRead, FileOpen read/write/seek/AtEOF/Pos/Length, IniRead/Write, Loop Read/Files, FileExist",
    "misc": "Sort, FormatTime, DateAdd/DateDiff, VarSetStrCapacity, StrPtr, Ord/Chr, VerCompare, A_AhkVersion",
    "errors": "divide by zero, bad index, missing Map key, unset variable, Chr(-1), type errors, wrong arity, unknown function, StrGet",
    "directives": "the directives a script may carry",
    "hotkeys": "hotkey and hotstring definitions: one-line and block bodies, return, modifier chords, the doubled colon of a hotstring, the literal empty brace the binary refuses",
    "hotstrings": "the options of a hotstring (`*`, `?`), a quoted character and a semicolon in the trigger and the replacement, and the multi-line continuation replacement",
    "suspend": "Suspend/Resume and A_IsSuspended, and #SuspendExempt with a hotkey",
}

WARNING_BLOCK = re.compile(
    r"(?m)^[^\n]*\(\d+\) : ==> Warning:[^\n]*(?:\n[ \t]+Specifically:[^\n]*)?\n?"
)
RUST_WARNING_ELEMENT = re.compile(r"^\(\d+\) : ==> Warning:")
REAL_ERROR_PREFIX = re.compile(r"(?m)^<script> \(\d+\) : ==> ")
KEYS_ERROR_PREFIX = re.compile(r"(?m)^<script> \d+:\d+: ")


class RunResult:
    """What one engine did on one case."""

    def __init__(self, argv, timed_out, exit_code, stdout, stderr, elapsed_ms):
        self.argv = argv
        self.timed_out = timed_out
        self.exit_code = exit_code
        self.stdout = stdout
        self.stderr = stderr
        self.elapsed_ms = elapsed_ms


def family_of(case_id):
    """The family of a case, from its id prefix."""
    return FAMILIES.get(case_id[:2], "misc")


def default_engines():
    """The three engine binaries, from the environment or from their defaults."""
    temp = Path(tempfile.gettempdir())
    return {
        "real": os.environ.get("AHK_REAL", str(temp / "ahk-v2-measure" / "AutoHotkey64.exe")),
        "keysharp": os.environ.get(
            "AHK_KEYSHARP",
            str(REPO.parent / "Keysharp" / "bin" / "Debug" / "net10.0-windows" / "Keysharp.exe"),
        ),
        "rust": os.environ.get("AHK_RUST", str(REPO / "target" / "release" / "ahk.exe")),
    }


def kill_tree(process):
    """Kill an engine process and the children it started."""
    if os.name == "nt":
        try:
            subprocess.run(
                ["taskkill", "/F", "/T", "/PID", str(process.pid)],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
        except OSError:
            pass
    else:
        import signal

        try:
            os.killpg(os.getpgid(process.pid), signal.SIGKILL)
        except OSError:
            pass
    try:
        process.kill()
    except OSError:
        pass


def run_process(argv, cwd, timeout, stdin_text=None, extra_env=None):
    """Run one engine, bounded by `timeout`, and never leave it running."""
    environment = os.environ.copy()
    if extra_env:
        environment.update(extra_env)
    stdin = subprocess.PIPE if stdin_text is not None else subprocess.DEVNULL
    started = time.monotonic()
    try:
        process = subprocess.Popen(
            argv,
            cwd=str(cwd),
            stdin=stdin,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
            creationflags=getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0) if os.name == "nt" else 0,
            start_new_session=os.name != "nt",
        )
    except OSError as error:
        return RunResult(argv, False, None, "", f"cannot start {argv[0]}: {error}", 0)
    timed_out = False
    payload = stdin_text.encode("utf-8") if stdin_text is not None else None
    try:
        out, err = process.communicate(payload, timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        kill_tree(process)
        try:
            out, err = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            out, err = b"", b""
    elapsed_ms = int((time.monotonic() - started) * 1000)
    return RunResult(
        argv,
        timed_out,
        process.returncode,
        out.decode("utf-8", "replace"),
        err.decode("utf-8", "replace"),
        elapsed_ms,
    )


def strip_warnings(text):
    """Remove the diagnostic warning blocks of every engine."""
    return WARNING_BLOCK.sub("", text)


def normalize_paths(text, script, cwd, root):
    """Replace the paths an engine puts in its diagnostics with stable tags."""
    if not text:
        return text
    for needle, tag in ((str(script), "<script>"), (str(cwd), "<cwd>"), (str(root), "<root>")):
        if needle:
            text = text.replace(needle, tag)
    return text


def process_answer(engine, run, script, cwd, root, note):
    """The normalized answer of real AutoHotkey or Keysharp."""
    if run.timed_out:
        note.append(f"{engine}-timeout")
        return "TIMEOUT(killed)"
    if run.exit_code is not None and run.exit_code not in (0, 1, 2):
        note.append(f"{engine}-crash-exit={run.exit_code}")
        return f"CRASH(exit={run.exit_code})"
    out = run.stdout
    normalized = out.replace("\r\n", "\n")
    if normalized != out:
        note.append("crlf-normalized")
    stripped = strip_warnings(normalized)
    if stripped != normalized:
        note.append("warn-stripped")
    stripped = normalize_paths(stripped, script, cwd, root).rstrip("\r\n")
    err = normalize_paths(run.stderr, script, cwd, root).rstrip("\r\n")
    cleaned = REAL_ERROR_PREFIX.sub("", err)
    cleaned = KEYS_ERROR_PREFIX.sub("", cleaned)
    if cleaned != err:
        note.append(f"{engine}-err-loc-normalized")
        err = cleaned
    if stripped:
        answer = stripped
        if err:
            answer += "\nSTDERR:" + err
        return answer
    if err:
        note.append(f"{engine}-answer=stderr")
        return "STDERR:" + err
    note.append(f"{engine}-empty-exit={run.exit_code}")
    return ""


def rust_answer(run, script, cwd, root, note):
    """The normalized answer of the Rust runner, out of its JSON report."""
    if run.timed_out:
        note.append("rust-timeout")
        return "TIMEOUT(killed)"
    if run.exit_code is not None and run.exit_code not in (0, 1, 2):
        note.append(f"rust-crash-exit={run.exit_code}")
        return f"CRASH(exit={run.exit_code})"
    raw = run.stdout.strip()
    if not raw:
        err = normalize_paths(run.stderr, script, cwd, root).rstrip("\r\n").strip()
        if err:
            note.append("rust-no-json")
            return "STDERR:" + err
        note.append("rust-no-report")
        return "NO-REPORT"
    try:
        report = json.loads(raw)
    except json.JSONDecodeError:
        note.append("rust-bad-json")
        return "BAD-JSON:" + raw[:300]
    parts = []
    dropped = 0
    for element in report.get("output") or []:
        text = str(element)
        if RUST_WARNING_ELEMENT.match(text):
            dropped += 1
            continue
        parts.append(text)
    if dropped:
        note.append(f"rust-warn-elements-dropped={dropped}")
    answer = strip_warnings("".join(parts))
    answer = normalize_paths(answer, script, cwd, root).rstrip("\r\n")
    if answer:
        return answer
    error = report.get("error")
    if error:
        note.append("rust-answer=report.error")
        return "ERROR:" + normalize_paths(str(error), script, cwd, root).strip()
    note.append("rust-empty")
    return ""


def compare_key(answer):
    """Drop the channel a failure came out of: only the text is compared."""
    return re.sub(r"^(STDERR:|ERROR:)", "", answer)


def tsv_escape(text):
    """One field of results.tsv: no tabs, no newlines, bounded length."""
    if text is None:
        return ""
    text = str(text).replace("\t", " ").replace("\r", "").replace("\n", "\\n")
    if len(text) > 2000:
        text = text[:2000] + "...<truncated>"
    return text


def read_baseline(path):
    """The verdict of every case of a previous results.tsv."""
    verdicts = {}
    table = Path(path)
    if not table.exists():
        return verdicts
    lines = table.read_text(encoding="utf-8", errors="replace").splitlines()
    if not lines:
        return verdicts
    for line in lines[1:]:
        fields = line.split("\t")
        if len(fields) >= 3:
            verdicts[fields[0]] = fields[2]
    return verdicts


def render_report(args, rows, engines, baseline, deltas):
    """Render the markdown report of a run."""
    total = len(rows)
    counts = {verdict: sum(1 for row in rows if row["verdict"] == verdict) for verdict in VERDICTS}
    divergent = total - counts["MATCH"]
    date = _datetime.date.today().isoformat()
    lines = []
    add = lines.append

    add("# Three-way differential report: real AutoHotkey v2.0.19 vs Keysharp vs Rust `ahk`")
    add("")
    add(f"Date: {date}.  Case set: {total} deterministic v2 scripts, family-classified, "
        f"run in: {', '.join(engines)}; raw evidence in `raw/`.")
    add("")
    add(f"**Verdict: {counts['MATCH']}/{total} cases MATCH "
        f"({round(100.0 * counts['MATCH'] / total, 1) if total else 0}%).  "
        f"{counts['KEYS-DIFFERS']} KEYS-DIFFERS, {counts['RUST-DIFFERS']} RUST-DIFFERS, "
        f"{counts['BOTH-DIFFER']} BOTH-DIFFER, {counts['ERROR']} ERROR"
        + (f", {counts['DIFFERS']} DIFFERS" if counts["DIFFERS"] else "")
        + ".**")
    add("")
    add("Generated by `scripts/differential.py --report`; every disagreement is listed with the "
        "exact answer each engine produced, and the raw streams are in `raw/`.")
    add("")

    if baseline:
        fixed = deltas["fixed"]
        regressed = deltas["regressed"]
        changed = deltas["changed"]
        add("## Against the baseline")
        add("")
        add(f"Baseline: `{args.baseline}` — {len(baseline)} cases.")
        add("")
        add(f"- **FIXED** (divergent before, MATCH now): {len(fixed)}"
            + (": " + ", ".join(f"`{case}`" for case in fixed) if fixed else "."))
        add(f"- **REGRESSED** (MATCH before, divergent now): {len(regressed)}"
            + (": " + ", ".join(f"`{case}`" for case in regressed) if regressed else "."))
        add(f"- Other verdict changes: {len(changed)}"
            + (": " + ", ".join(f"`{case}`" for case in changed) if changed else "."))
        add("")
        if fixed:
            add("A fixed case is the improvement the last work package bought; a regressed one is "
                "a defect the next work package has to own.")
            add("")
    else:
        add("## Against the audit baseline")
        add("")
        add("The audit measured 67/93 MATCH (72%) with 8 KEYS-DIFFERS, 14 RUST-DIFFERS and "
            "4 BOTH-DIFFER.  The two Float-rendering divergences (`nu03_float_render`, "
            "`ex02_implicit_concat`) match when the Rust binary prints v2's 17 significant digits; "
            "the run below is the new baseline and `--baseline` compares against it.")
        add("")

    add("## Engines and method")
    add("")
    add("| Engine | Binary | Enabled |")
    add("| --- | --- | --- |")
    for engine in ENGINES:
        add(f"| {engine} | `{engines[engine]}` | {'yes' if engine in args.engines else 'no'} |")
    add("")
    add("`FileAppend(Text, \"*\")` is the mechanism: real AutoHotkey and Keysharp write it to "
        "process stdout, and the Rust runner collects it in its JSON report on stdout under "
        "`output`.  The harness compares real = normalized stdout, Keysharp = normalized stdout, "
        "Rust = concatenated `report.output` (normalized), with the failure path falling back to "
        "stderr/`report.error` so failure wording is compared as data.")
    add("")
    add("### Safety")
    add("")
    add("Every script starts with `#ErrorStdOut` and `#Warn All, StdOut`, registers an `OnError` "
        "backstop that suppresses the error dialog, and wraps every probe in `try`/`catch as e`.  "
        f"Every process is bounded by a timeout (real {args.timeout_real} s, Keysharp "
        f"{args.timeout_keysharp} s, Rust {args.timeout_rust} s) and the process tree is killed on "
        "expiry; a timeout is recorded and classified ERROR.  No case timed out or crashed in this "
        "run unless the ERROR column says otherwise.")
    add("")
    add("### Normalization (only what is engine plumbing, never a value)")
    add("")
    add("1. `\\r\\n` -> `\\n`; the trailing newline of an answer is trimmed for comparison.")
    add("2. Diagnostic warning blocks are removed from stdout: `... (N) : ==> Warning: ...` plus a "
        "following `     Specifically: ...` line.  The Rust runner emits such a warning for the "
        "`OnError` backstop name and for every `catch as e` variable in every script.")
    add("3. Error-location prefixes are removed from stderr answers: `<script> (12) : ==> ` and "
        "Keysharp's compile form `<script> 12:3: `.  The script path becomes `<script>`, the "
        "per-engine working directory `<cwd>`, the repository root `<root>`.")
    add("4. Nothing else: the channel a failure came out of (`STDERR:` vs `ERROR:`) is dropped and "
        "the text is compared.")
    add("")
    add("### Verdict rules")
    add("")
    add("| Verdict | Rule |")
    add("| --- | --- |")
    add("| MATCH | every enabled engine's answer is identical |")
    add("| KEYS-DIFFERS | real == Rust, Keysharp differs |")
    add("| RUST-DIFFERS | real == Keysharp, Rust differs |")
    add("| BOTH-DIFFER | no two engines agree |")
    add("| ERROR | a timeout, a crash exit, or an engine that did not answer |")
    add("| DIFFERS | two engines were enabled and they disagree |")
    add("")

    sections = (
        ("RUST-DIFFERS", "Rust `ahk` divergences (real AutoHotkey == Keysharp)",
         ("Case", "Real AutoHotkey / Keysharp (agree)", "Rust `ahk`")),
        ("KEYS-DIFFERS", "Keysharp divergences (real AutoHotkey == Rust `ahk`)",
         ("Case", "Real AutoHotkey / Rust `ahk` (agree)", "Keysharp")),
        ("BOTH-DIFFER", "Cases where no two engines agree",
         ("Case", "Real AutoHotkey", "Keysharp", "Rust `ahk`")),
        ("DIFFERS", "Cases where the enabled engines disagree",
         ("Case", "Engine A", "Engine B")),
        ("ERROR", "Cases where an engine did not answer",
         ("Case", "Engine", "Raw failure")),
    )
    for verdict, title, header in sections:
        subset = [row for row in rows if row["verdict"] == verdict]
        if not subset:
            continue
        add(f"## {title}")
        add("")
        add("| " + " | ".join(header) + " |")
        add("| " + " | ".join("---" for _ in header) + " |")
        for row in subset:
            cells = [f"`{row['script']}`"]
            if verdict == "RUST-DIFFERS":
                cells += [_md(row["real"]), _md(row["rust"])]
            elif verdict == "KEYS-DIFFERS":
                cells += [_md(row["real"]), _md(row["keysharp"])]
            elif verdict == "BOTH-DIFFER":
                cells += [_md(row["real"]), _md(row["keysharp"]), _md(row["rust"])]
            elif verdict == "ERROR":
                cells += ["-", _md(row["normalized_note"])]
            else:
                cells += [_md(row["real"]), _md(row["keysharp"])]
            add("| " + " | ".join(cells) + " |")
        add("")

    add("## Coverage by family")
    add("")
    add("| Family | Cases | MATCH | KEYS-DIFFERS | RUST-DIFFERS | BOTH-DIFFER | ERROR | DIFFERS | What it covers |")
    add("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |")
    families = []
    for row in rows:
        if row["family"] not in families:
            families.append(row["family"])
    for family in families:
        subset = [row for row in rows if row["family"] == family]
        cells = [str(sum(1 for row in subset if row["verdict"] == verdict)) for verdict in VERDICTS]
        add("| " + family + f" | {len(subset)} | " + " | ".join(cells) + " | "
            + FAMILY_COVERAGE.get(family, "") + " |")
    add("")

    add("## Full case table")
    add("")
    add("The exact answers and normalization notes of every case are in `results.tsv` and in "
        "`raw/<case>.<engine>.answer.txt`.")
    add("")
    add("| # | Script | Family | Verdict | Normalization note |")
    add("| ---: | --- | --- | --- | --- |")
    for index, row in enumerate(rows, start=1):
        note = row["normalized_note"] or "-"
        add(f"| {index} | `{row['script']}.ahk` | {row['family']} | {row['verdict']} | "
            + _md(note) + " |")
    add("")

    add("## Caveats and limitations")
    add("")
    add("1. **Keysharp targets v2.1-alpha.37, not v2.0.19.** A handful of its divergences are "
        "expected; `A_AhkVersion` confirms the target (`mi08_version`).")
    add("2. **The Rust tree claims v2.0.19 exactly**, so its divergences are not excused by a "
        "version target.  A refusal with an honest message is still a script-visible "
        "incompatibility: the case exists to make it countable.")
    add("3. **No case may open a dialog** (see the safety invariant above), which bounds the "
        "exercised surface to the language, the string/number/date built-ins, the object model and "
        "file I/O.")
    add("4. **Warnings are diagnostics, not output**, and are stripped in every engine; the Rust "
        "runner's warning for `OnError`/`catch` variables is recorded here rather than compared.")
    add("5. **Error-path comparisons are about message text, not location:** class, message and "
        "`Extra` are compared; the file/line prefix is normalized away.")
    add("6. **One machine, one run per case.** The scripts are deterministic (fixed date-time "
        "stamps, sorted listings, no current time, no random), so a repeat run reproduces "
        "`results.tsv`.")
    add("7. **Timeout and kill are reported, never ignored:** a hang is classified ERROR with "
        "`TIMEOUT(killed)`, a crash exit with `CRASH(exit=..)`.")
    add("")

    add("## Reproduction")
    add("")
    add("```sh")
    add("python scripts/differential.py --report scripts/differential/report.md")
    add("python scripts/differential.py --only er0")
    add("python scripts/differential.py --engines rust   # on a machine without AHK/Keysharp")
    add("```")
    add("")
    add("`--only <substring>` runs a subset; `--keep-tmp` keeps the per-engine working directories "
        "for inspection; `--baseline <results.tsv>` reports the fixed/regressed cases since a "
        "previous run.")
    add("")

    add("## Files")
    add("")
    add("| Path | What it is |")
    add("| --- | --- |")
    add("| `scripts/differential.py` | the harness: runs the cases in the enabled engines, "
        "normalizes, classifies, writes `results.tsv` + `summary.txt` (+ this report) |")
    add(f"| `scripts/differential/cases/` | the {total} case scripts (shared by all engines; every "
        "one starts with `#ErrorStdOut` + `#Warn All, StdOut` and catches everything) |")
    add("| `scripts/differential/results.tsv` | one row per case: script, family, verdict, "
        "real_out, keysharp_out, rust_out, normalized_note (`\\n` is a literal newline) |")
    add("| `scripts/differential/raw/` | per case and engine: raw stdout/stderr, the Rust JSON "
        "report, and the normalized answer |")
    add("| `scripts/differential/summary.txt` | verdict totals and the per-family matrix |")
    add("")
    return "\n".join(lines) + "\n"


def _md(text):
    """One markdown table cell."""
    if text is None:
        return ""
    return str(text).replace("|", "\\|")


def summary_text(rows):
    """The verdict totals and the per-family matrix, as the audit's summary.txt."""
    counts = {verdict: sum(1 for row in rows if row["verdict"] == verdict) for verdict in VERDICTS}
    lines = [f"cases: {len(rows)}", "", "verdict\tcount"]
    for verdict in VERDICTS:
        if verdict == "DIFFERS" and not counts[verdict]:
            continue
        lines.append(f"{verdict}\t{counts[verdict]}")
    lines.append("")
    lines.append("family\t" + "\t".join(VERDICTS[:-1]) + "\ttotal")
    families = []
    for row in rows:
        if row["family"] not in families:
            families.append(row["family"])
    for family in families:
        subset = [row for row in rows if row["family"] == family]
        cells = [str(sum(1 for row in subset if row["verdict"] == verdict))
                 for verdict in VERDICTS[:-1]]
        lines.append(family + "\t" + "\t".join(cells) + f"\t{len(subset)}")
    return "\n".join(lines) + "\n"


def main(argv=None):
    parser = argparse.ArgumentParser(
        prog="differential.py",
        description="Three-way differential harness: real AutoHotkey v2.0.19 vs Keysharp vs "
                    "Rust ahk.",
        epilog="See scripts/README.md for the case format and the safety invariants.",
    )
    parser.add_argument("--cases", default=None,
                        help=f"case directory (default: {DEFAULT_CASES})")
    parser.add_argument("--out", default=None,
                        help=f"output directory (default: {DEFAULT_OUT})")
    parser.add_argument("--only", default="",
                        help="run only the cases whose id contains this substring")
    parser.add_argument("--engines", default="real,keysharp,rust",
                        help="comma-separated engines to run (default: real,keysharp,rust)")
    parser.add_argument("--allow-missing", action="store_true",
                        help="skip a missing engine instead of failing")
    parser.add_argument("--timeout-real", type=float, default=DEFAULT_TIMEOUTS["real"])
    parser.add_argument("--timeout-keysharp", type=float, default=DEFAULT_TIMEOUTS["keysharp"])
    parser.add_argument("--timeout-rust", type=float, default=DEFAULT_TIMEOUTS["rust"])
    parser.add_argument("--validate", action="store_true",
                        help="pass --validate to Keysharp (syntax-only check, not a differential)")
    parser.add_argument("--report", default=None, metavar="PATH",
                        help="write the markdown report to PATH")
    parser.add_argument("--baseline", default=None, metavar="RESULTS_TSV",
                        help="compare the verdicts against a previous results.tsv")
    parser.add_argument("--check", action="store_true",
                        help="exit non-zero when a case regressed against --baseline")
    parser.add_argument("--keep-tmp", action="store_true",
                        help="keep the per-engine working directories")
    parser.add_argument("--list", action="store_true",
                        help="list the cases and exit")
    args = parser.parse_args(argv)

    args.engines = [name.strip().lower() for name in args.engines.split(",") if name.strip()]
    unknown = [name for name in args.engines if name not in ENGINES]
    if unknown or not args.engines:
        parser.error(f"unknown engine(s): {', '.join(unknown) or '(none)'}; "
                     f"choose from {', '.join(ENGINES)}")

    cases_dir = Path(args.cases) if args.cases else DEFAULT_CASES
    if args.out:
        out_dir = Path(args.out)
    elif args.only:
        # A subset must not overwrite the full run's results.tsv, which is the
        # baseline the next full run is compared against.
        out_dir = DEFAULT_OUT / "_subset"
    else:
        out_dir = DEFAULT_OUT
    root = REPO
    cases = sorted(cases_dir.glob("*.ahk"))
    if args.only:
        cases = [case for case in cases if args.only in case.stem]
    if not cases:
        parser.error(f"no cases in {cases_dir}" + (f" matching {args.only!r}" if args.only else ""))
    if args.list:
        for case in cases:
            print(f"{case.stem}\t{family_of(case.stem)}")
        return 0

    engines = default_engines()
    enabled = []
    for engine in args.engines:
        binary = engines[engine]
        if not Path(binary).exists():
            if args.allow_missing:
                print(f"warning: {engine} engine not found at {binary}; skipped", file=sys.stderr)
                continue
            print(f"error: the {engine} engine is not at {binary}.\n"
                  f"       Set {ENGINE_ENV[engine]} to its path, or pass `--engines` with the "
                  f"engines this machine has (e.g. `--engines rust`), or `--allow-missing`.",
                  file=sys.stderr)
            return 2
        enabled.append(engine)
    if not enabled:
        print("error: no engine is enabled", file=sys.stderr)
        return 2

    out_dir.mkdir(parents=True, exist_ok=True)
    raw_dir = out_dir / "raw"
    raw_dir.mkdir(exist_ok=True)
    tmp_root = out_dir / "_tmp"
    if tmp_root.exists():
        shutil.rmtree(tmp_root, ignore_errors=True)
    tmp_root.mkdir(parents=True, exist_ok=True)

    timeouts = {
        "real": args.timeout_real,
        "keysharp": args.timeout_keysharp,
        "rust": args.timeout_rust,
    }

    baseline = read_baseline(args.baseline) if args.baseline else {}
    rows = []
    total = len(cases)
    started_at = time.monotonic()

    for index, case in enumerate(cases, start=1):
        case_id = case.stem
        family = family_of(case_id)
        print(f"[{index}/{total}] {case_id} ({family})", flush=True)
        case_tmp = tmp_root / case_id
        answers = {}
        notes = []
        runs = {}
        for engine in enabled:
            work = case_tmp / engine
            work.mkdir(parents=True, exist_ok=True)
            environment = {"AUDIT_TMP": str(work)}
            if engine == "real":
                run = run_process([engines["real"], "/ErrorStdOut", str(case)],
                                  work, timeouts["real"], extra_env=environment)
            elif engine == "keysharp":
                command = [engines["keysharp"]]
                if args.validate:
                    command.append("--validate")
                command.append(str(case))
                run = run_process(command, work, timeouts["keysharp"], extra_env=environment)
            else:
                run = run_process([engines["rust"], "--dialect=v2", "--timeout=15"],
                                  work, timeouts["rust"],
                                  stdin_text=case.read_text(encoding="utf-8", errors="replace"),
                                  extra_env=environment)
            runs[engine] = run
            (raw_dir / f"{case_id}.{engine}.out.txt").write_text(
                run.stdout, encoding="utf-8", errors="replace")
            (raw_dir / f"{case_id}.{engine}.err.txt").write_text(
                run.stderr, encoding="utf-8", errors="replace")
            if engine == "rust":
                (raw_dir / f"{case_id}.rust.json").write_text(
                    run.stdout, encoding="utf-8", errors="replace")

        for engine in enabled:
            if engine == "rust":
                answers[engine] = rust_answer(runs[engine], case, case_tmp / engine, root, notes)
            else:
                answers[engine] = process_answer(
                    engine, runs[engine], case, case_tmp / engine, root, notes)
            (raw_dir / f"{case_id}.{engine}.answer.txt").write_text(
                answers[engine], encoding="utf-8", errors="replace")

        hard_error = any(
            runs[engine].timed_out
            or (runs[engine].exit_code is not None and runs[engine].exit_code not in (0, 1, 2))
            for engine in enabled
        )
        if hard_error:
            verdict = "ERROR"
        elif len(enabled) == 1:
            verdict = "MATCH"
        else:
            keys = [compare_key(answers[engine]) for engine in enabled]
            if len(set(keys)) == 1:
                verdict = "MATCH"
            elif set(enabled) == {"real", "keysharp", "rust"}:
                if keys[0] == keys[2]:
                    verdict = "KEYS-DIFFERS"
                elif keys[0] == keys[1]:
                    verdict = "RUST-DIFFERS"
                else:
                    verdict = "BOTH-DIFFER"
            else:
                verdict = "DIFFERS"

        note = "; ".join(dict.fromkeys(notes))
        rows.append({
            "script": case_id,
            "family": family,
            "verdict": verdict,
            "real": answers.get("real", ""),
            "keysharp": answers.get("keysharp", ""),
            "rust": answers.get("rust", ""),
            "normalized_note": note,
        })
        print(f"      -> {verdict}" + (f"  ({note})" if note else ""), flush=True)

    elapsed = time.monotonic() - started_at

    header = ["script", "family", "verdict", "ahk_out", "keysharp_out", "rust_out",
              "normalized_note"]
    table = ["\t".join(header)]
    for row in rows:
        table.append("\t".join([
            tsv_escape(row["script"]),
            tsv_escape(row["family"]),
            row["verdict"],
            tsv_escape(row["real"]),
            tsv_escape(row["keysharp"]),
            tsv_escape(row["rust"]),
            tsv_escape(row["normalized_note"]),
        ]))
    (out_dir / "results.tsv").write_text("\n".join(table) + "\n", encoding="utf-8")
    (out_dir / "summary.txt").write_text(summary_text(rows), encoding="utf-8")

    deltas = {"fixed": [], "regressed": [], "changed": []}
    if baseline:
        for row in rows:
            before = baseline.get(row["script"])
            if before is None:
                continue
            if before != "MATCH" and row["verdict"] == "MATCH":
                deltas["fixed"].append(row["script"])
            elif before == "MATCH" and row["verdict"] != "MATCH":
                deltas["regressed"].append(row["script"])
            elif before != row["verdict"]:
                deltas["changed"].append(row["script"])

    if args.report:
        report_path = Path(args.report)
        if not report_path.is_absolute():
            report_path = Path.cwd() / report_path
        report_path.write_text(render_report(args, rows, engines, baseline, deltas),
                               encoding="utf-8")
        print(f"wrote {report_path}")

    if not args.keep_tmp:
        shutil.rmtree(tmp_root, ignore_errors=True)

    counts = {verdict: sum(1 for row in rows if row["verdict"] == verdict) for verdict in VERDICTS}
    print()
    print(f"cases: {total}  engines: {', '.join(enabled)}  elapsed: {elapsed:.1f}s")
    print("verdict\tcount")
    for verdict in VERDICTS:
        if verdict == "DIFFERS" and not counts[verdict]:
            continue
        print(f"{verdict}\t{counts[verdict]}")
    if baseline:
        print(f"fixed: {len(deltas['fixed'])}  regressed: {len(deltas['regressed'])}  "
              f"changed: {len(deltas['changed'])}")
    print(f"wrote {out_dir / 'results.tsv'}")
    print(f"wrote {out_dir / 'summary.txt'}")

    if counts["ERROR"]:
        print("error: at least one case did not answer; see the ERROR rows", file=sys.stderr)
        return 1
    if args.check and baseline and deltas["regressed"]:
        print(f"error: {len(deltas['regressed'])} case(s) regressed against the baseline: "
              + ", ".join(deltas["regressed"]), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())