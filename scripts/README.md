# `scripts/` — the cross-platform verification tools

The tools that make the project's claims measurable instead of asserted, and
the tool that writes the surface documents. All are Python 3 standard-library
only: no packages to install.

| Script | What it does |
| --- | --- |
| `differential.py` | Runs the deterministic case set in the engines you choose — by default **real AutoHotkey v2.0.31 vs this tree's Rust `ahk`** (the tree's fidelity); Keysharp is an optional third opinion (`--engines real,keysharp,rust`). Normalizes only engine plumbing and classifies every case MATCH / RUST-DIFFERS / KEYS-DIFFERS / BOTH-DIFFER / ERROR / DIFFERS. `--gaps` re-runs only the cases where the tree diverged from real in the previous run. This is the port of the audit's PowerShell harness, kept in the tree so the comparison is reproducible and can gate CI. |
| `differential/cases/` | The 113 case scripts, one per behaviour family (`ex`, `nu`, `st`, `va`, `fu`, `cl`, `ob`, `co`, `fi`, `mi`, `er`, `di`, `hk`, `hs`, `su`, `wi`, `th`). |
| `corpus-manifest.py` | Writes the manifest of the corpus of `../AHKv2_Finetune`: one row per script with its hash, the version it asks for, the number of its lines that carry an action, whether its first statement was swallowed into a comment, whether its braces balance, the side effects it may have and the group of the duplicates it belongs to. Nothing of the corpus is executed: the manifest reads the bytes. |
| `corpus-check.py` | Puts the answer of the tree and the answer of the reference side by side for every script of the corpus, through the two MCP servers, and says which of the refusals are the gaps of the tree and which are the `#Requires` of a script that asks for a version the reference does not carry. The report is `docs/corpus/v2-corpus-gaps.md`. |
| `corpus-shrink.py` | Cuts a script of the corpus down to the smallest one that still shows the same gap: the tree refuses it with the same error and the official v2.0.32 loads it. The script that is measured is never executed -- the question is a parse. |
| `pairs-check.py` | Asks the tree the same question about the 851 pairs of the upstream converter (`../AHK-v2-script-converter`) that the two official interpreters are asked: does the script load? The tree is not the judge of itself, so a refusal of the tree is a gap only when the official one loads the script. Nothing is executed -- the programs of the pairs open boxes of the machine, and a parse never reaches them. The `.ah2` goes through the `check` of the MCP of the headless v2.0.32, and the `.ah1` through the official v1.1.37.02 with an `ExitApp` in it, so that the load is seen and nothing of the program runs. |
| `pairs-words.py` | Reads the report of `pairs-check.py` and asks the question that it does not: when the tree and the official both refuse a pair, do they refuse with the same words? The gate counts the refusals, and a tree that refuses every script with the same words passes it; the words are the contract that a script sees. It exits non-zero when a pair refuses differently, and it names the pairs that do. |
| `check-surface-docs.py` | The drift check between `ahk --surface` and the numbers the documents print (owned by the documentation work package; run it after a release build). |
| `v2-probes/probe.py` | Puts the answer of the tree and the answer of the official v2.0.32 side by side for the cases of a file of `v2-probes/`: the tree is not the judge of itself, and a refusal of the tree is a gap only when the official one loads the script. The question is a parse, and a parse never runs anything. The cases of a file are separated by a line that is only `---`. |
| `probe-gui-classes.py` | Reads the table of the nested classes of the `Gui` out of the binary: the classes of the controls of a window are nested classes of the class `Gui`, and the binary carries twenty-five of them as values. The instrument of the measurement of `crates/ahk/reference/ahk-v2.0.32.surface.tsv`. |
| `compare-gui-classes.py` | Runs the same questions about the classes of the controls of a window in the binary of v2.0.31 and in the tree, and puts the two answers side by side. |
| `gui-control-classes.py` | Reads the classes of the controls of a window of a script from the machine: a control of the v1 is a window of the machine, and the class of it is the class of that window (the class of the `Link` is `SysLink`). The instrument of the measurement of the classes of the controls of the v1. |
| `gen-surface-docs.py` | Writes `docs/surface/` — every name of the v1, of the v2 and of KeySharp, with the state this tree answers for it, in thirteen languages (English, Spanish, Portuguese of Brazil, French, Italian, Polish, Turkish, German, Russian, Japanese, Chinese, Traditional Chinese, Korean). `--check` fails when the subtree is out of date. |
| `sweep-v1-loads.py` | Asks the binary of v1.1.37.02 and the tree the same line of the v1 — every command of the table of the reference with zero to five parameters — and puts the two complaints side by side. The names and the shape of the parameters come from the table that was measured on the binary, and not from the surface of the tree, so that a name that the tree does not carry is a difference and not a silence. The complaint of the v1 has two kinds, the load and the run, and the kind is part of the answer. The commands that touch the machine are left out, and every line runs in a folder of its own. The instrument of the measurement of the loads of the v1. |
| `measure-window-targets-2.0.32.py` | Asks the official v2.0.32 and the tree the same question about the nineteen names of the `WinGet*` family: the call that names no window, and the call that follows a `WinExist` that found a window. The binary is the build that answers the errors as text, and not the copy that is installed in the machine — that one is the v2.1-alpha. The instrument of the measurement of the target of the family. |
| `reference_binaries.py` | Where the reference binaries are: the official v2.0.32 and the official v1.1.37.02 are not in the tree, and the temporary folder of the machine is cleaned. The variables `AHK_V2` and `AHK_V1` name them, and the places that the machine keeps them are the fallback. The copy that is installed in `C:\Program Files\AutoHotkey\v2` is the v2.1-alpha, and a measurement against it is a measurement against another dialect. |
| `perf-compare.py` | Runs every workload of `perf/` under the tree and under the official v2.0.32, and prints the median of five runs of each: the tree is faster, or slower, and by how much. The instrument of the measurement of the performance — the numbers of `docs/performance.md` come from it. |
| `perf/` | The workloads of the performance, one file per work. The workload takes the time of the machine with `A_TickCount` and writes it to `perf-out.txt`, in the folder of the script, where the harness reads it. |
| `perf-fine.py` | The fine clock of the performance: it reads the `elapsed_ms` of the report of the tree, whose clock is of a millisecond. The clock of the machine counts in 15,6 milliseconds, and a gain of ten nanoseconds is under a tick -- this is the instrument of the changes that small. |
| `surface_areas.py` | The families of the surface: which area a name belongs to. The areas of the v2 come from `crates/ahk/reference/ahk-v2.0.32.areas.py`; the names of the v1 and of KeySharp that no v2 area names are placed by hand, and the rules place the rest. |
| `surface_i18n.py` | The thirteen languages of the surface documents: the prose, the titles of the families and the reason of a refusal. |
| `surface_behavior.py` | The behavior of the names: what the tree does with them, what it answers, what it refuses with, and an example. It names the names that carry one and loads the text of `behaviour/`. |
| `v1-probes.sh` | Runs every probe of `v1-probes/` under the binary of AutoHotkey v1.1.37.02 and under the tree, and puts the two answers side by side. The instrument of the measurements of the menu of the v1, of the timers of a script that waits, of the catch of the v1, of the native call of a built-in variable and of the icon of the window of the script. |
| `v1-probes.ps1` | The twin of `v1-probes.sh` for the machines of Windows that have no bash: it runs the same probes and says the same thing, and it says `ERROR` when a probe fails under one of the two and `UNSTABLE` when the pointer of the machine did not stay where the probe put it. |
| `tree-windows.ps1` | Walks the windows of a process of Windows -- the windows of a script of the v1 among them -- with their class, their styles, their place and their parent, and it draws the window of the script into a picture with `PrintWindow`. The instrument of the proof that the windows that a script of the v1 paints into are the children of the window of the script. |
| `behaviour/` | The behavior itself, one file per chunk of names (`chunk1.py` … `chunk7.py`), each with the entry of every name in English, Chinese and Portuguese of Brazil. |

The Keysharp corpus importer and triage table are not here: they live in
`egm-review/keysharp-audit/corpus/` because the corpus is BSD-3-Clause and must
stay outside the Apache-2.0 tree. See `egm-review/keysharp-audit/corpus/README.md`.

The instrument of the icons of the tree is not here either: it lives beside the
resources it writes, in [`../resources/icons-from-the-svg.py`](../resources/icons-from-the-svg.py),
because the drawing it reads and the resources it writes are both there.

`docs/README.md` should link this page from the documentation index.

## Engines

The engine binaries are configured by environment variable or flag; a missing
engine is an error unless it is not in `--engines` (or `--allow-missing` is
passed). **The default is `real,rust`** — the tree claims real AutoHotkey
v2.0.31 exactly, so that pair is the fidelity gate. Keysharp (which targets
v2.1-alpha) is optional and not the default; a case where real == rust and
Keysharp differs (KEYS-DIFFERS) is a Keysharp divergence, not a gap of the tree.

| Engine | Environment variable | Default |
| --- | --- | --- |
| Real AutoHotkey v2.0.31 | `AHK_REAL` | `<temp>/ahk-v2-measure-2.0.31/AutoHotkey64.exe` |
| Keysharp | `AHK_KEYSHARP` | `../Keysharp/bin/Debug/net10.0-windows/Keysharp.exe` |
| Rust `ahk` | `AHK_RUST` | `target/release/ahk.exe` |

The real AutoHotkey binary exists only on Windows. On any other platform run
`--engines rust` (or `--engines rust,keysharp` where Keysharp is present); the
verdicts reduce to MATCH/DIFFERS and the harness still runs.

## Usage

```sh
# the full run (real vs rust), with the markdown report
python scripts/differential.py --report scripts/differential/report.md

# only the cases where the tree diverged from real in the previous run
python scripts/differential.py --gaps

# only the cases whose id contains "er0"
python scripts/differential.py --only er0

# the optional three-way comparison with Keysharp
python scripts/differential.py --engines real,keysharp,rust

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

- Real AutoHotkey v2.0.31 is an oracle, never a dependency: it is used to settle
  what the dialect means when the tree and Keysharp disagree.
- Keysharp targets v2.1-alpha.37, not v2.0.31; its divergences are expected where
  it documents a v2.1 difference, and the report says which are documented.
- The case set is deliberately deterministic (fixed date-time stamps, sorted
  listings, no current time, no random), so a repeat run reproduces
  `results.tsv`.