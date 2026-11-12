# `scripts/` — the cross-platform verification tools

The tools that make the project's claims measurable instead of asserted, and
the tool that writes the surface documents. All are Python 3 standard-library
only: no packages to install.

| Script | What it does |
| --- | --- |
| `differential.py` | Runs the deterministic case set in up to three engines — real AutoHotkey v2.0.19, Keysharp and this tree's Rust `ahk` — normalizes only engine plumbing, and classifies every case MATCH / RUST-DIFFERS / KEYS-DIFFERS / BOTH-DIFFER / ERROR. This is the port of the audit's PowerShell harness (`egm-review/keysharp-audit/differential/harness.ps1`), kept in the tree so the comparison is reproducible and can gate CI. |
| `differential/cases/` | The 93 case scripts, one per behaviour family (`ex`, `nu`, `st`, `va`, `fu`, `cl`, `ob`, `co`, `fi`, `mi`, `er`). |
| `check-surface-docs.py` | The drift check between `ahk --surface` and the numbers the documents print (owned by the documentation work package; run it after a release build). |
| `gen-surface-docs.py` | Writes `docs/surface/` — every name of the v1, of the v2 and of KeySharp, with the state this tree answers for it, in ten languages (English, Spanish, Portuguese of Brazil, French, German, Russian, Japanese, Chinese, Traditional Chinese, Korean). `--check` fails when the subtree is out of date. |
| `surface_areas.py` | The families of the surface: which area a name belongs to. The areas of the v2 come from `crates/ahk/reference/ahk-v2.0.19.areas.py`; the names of the v1 and of KeySharp that no v2 area names are placed by hand, and the rules place the rest. |
| `surface_i18n.py` | The ten languages of the surface documents: the prose, the titles of the families and the reason of a refusal. |

The Keysharp corpus importer and triage table are not here: they live in
`egm-review/keysharp-audit/corpus/` because the corpus is BSD-3-Clause and must
stay outside the Apache-2.0 tree. See `egm-review/keysharp-audit/corpus/README.md`.

`docs/README.md` should link this page from the documentation index.

## Engines

The engine binaries are configured by environment variable or flag; a missing
engine is an error unless it is not in `--engines` (or `--allow-missing` is
passed).

| Engine | Environment variable | Default |
| --- | --- | --- |
| Real AutoHotkey v2.0.19 | `AHK_REAL` | `<temp>/ahk-v2-measure/AutoHotkey64.exe` |
| Keysharp | `AHK_KEYSHARP` | `../Keysharp/bin/Debug/net10.0-windows/Keysharp.exe` |
| Rust `ahk` | `AHK_RUST` | `target/release/ahk.exe` |

The real AutoHotkey binary exists only on Windows. On any other platform run
`--engines rust` (or `--engines rust,keysharp` where Keysharp is present); the
verdicts reduce to MATCH/DIFFERS and the harness still runs.

## Usage

```sh
# the full three-engine run, with the markdown report
python scripts/differential.py --report scripts/differential/report.md

# only the cases whose id contains "er0"
python scripts/differential.py --only er0

# one engine, on a machine without AutoHotkey or Keysharp
python scripts/differential.py --engines rust

# a syntax-only pass through Keysharp's validator
python scripts/differential.py --engines keysharp --validate

# compare against the previous baseline (the CI gate: no new divergence)
python scripts/differential.py --baseline scripts/differential/results.tsv --check
```

Outputs, written into `--out` (default `scripts/differential/`):

- `results.tsv` — one row per case: `script`, `family`, `verdict`, the three
  normalized answers, and the normalization note (`\n` is a literal newline).
- `summary.txt` — verdict totals and the per-family matrix.
- `raw/` — per case and engine: raw stdout, raw stderr, the Rust JSON report,
  and the normalized answer.
- the markdown report, when `--report` is given.
- `_tmp/` — the per-engine working directories, removed unless `--keep-tmp`.

A subset run (`--only <text>`) writes into `scripts/differential/_subset/`
instead, so exploring a case never overwrites the full run's `results.tsv`,
which is the baseline the next run is compared against.

The harness exits non-zero when a case is `ERROR`, and with `--baseline --
--check` when a case regressed from `MATCH` to a different verdict.

## Normalization

Only engine plumbing is rewritten; a value is never touched.

1. `\r\n` → `\n`; the trailing newline is trimmed for comparison.
2. Diagnostic warning blocks (`... (N) : ==> Warning: ...` plus a
   `     Specifically: ...` line) are removed from stdout: warning wording and
   location are engine diagnostics, not script output.
3. Error-location prefixes are removed from stderr answers (`<script> (12) :
   ==>` and Keysharp's compile form `<script> 12:3:`), and the script path, the
   per-engine working directory and the repository root are tagged
   `<script>`/`<cwd>`/`<root>`.
4. Nothing else. The channel a failure came out of (`STDERR:` vs `ERROR:`) is
   dropped; the text is compared.

## Adding a case

A case is a small AHK v2 script whose observable answer is what it prints with
`FileAppend(text, "*")` (the console of the tree is the report's `output`).
Every case must keep the safety invariant: a script that raises an uncaught
error in real AutoHotkey opens a modal dialog, and a dialog is a hang nothing
can dismiss. So every case starts with the harness header:

```ahk
#ErrorStdOut
#Warn All, StdOut
OnError(__backstop)
__backstop(e, mode) {
    FileAppend("UNCAUGHT|" Type(e) "|" e.Message "`n", "*")
    return true
}
Emit(s) {
    FileAppend(s "`n", "*")
}
```

and wraps each probe in `try { ... } catch as e { Emit("ERR " Type(e) ": " e.Message) }`.
No `MsgBox`/`InputBox`/`FileSelect`/`Gui.Show`, no hotkeys, no timers, no
clipboard, no network, no screen/pointer access, no `Sleep`. File I/O cases call
`SetWorkingDir(EnvGet("AUDIT_TMP"))` first, because each engine gets its own
fresh absolute directory that way.

Name the file `<family><nn>_<topic>.ahk`; the id prefix selects the family
(`ex`/`nu`/`st`/`va`/`fu`/`cl`/`ob`/`co`/`fi`/`mi`/`er`), the family is reported
in `results.tsv`, and `scripts/differential/cases/` is the case set the CI runs.

## Notes

- Real AutoHotkey v2.0.19 is an oracle, never a dependency: it is used to settle
  what the dialect means when the tree and Keysharp disagree.
- Keysharp targets v2.1-alpha.37, not v2.0.19; its divergences are expected where
  it documents a v2.1 difference, and the report says which are documented.
- The case set is deliberately deterministic (fixed date-time stamps, sorted
  listings, no current time, no random), so a repeat run reproduces
  `results.tsv`.