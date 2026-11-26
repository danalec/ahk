# The documents of this tree

The guides, the references and the records behind the code. Start here when
you do not know which file you want: every document of the folder is listed
with what it holds and who it is for.

- [Three kinds of document](#three-kinds-of-document)
- [The index](#the-index)
- [Reading paths](#reading-paths)
  - [You are new, and you want to run a script](#you-are-new-and-you-want-to-run-a-script)
  - [You want to look up a name](#you-want-to-look-up-a-name)
  - [You have a v1 script to port](#you-have-a-v1-script-to-port)
  - [You are contributing to the tree](#you-are-contributing-to-the-tree)
- [The document next door](#the-document-next-door)

---

## Three kinds of document

| The kind | What it is | The documents |
|---|---|---|
| a **user guide** | written for someone who runs scripts or writes them: plain sentences, examples, the behaviour of the language | `language-guide.md`, `ahk-v2-reference.md`, `ahk-v1-reference.md`, `ahk-v2.1-dialect.md`, `ahk-ks-dialect.md`, `runner-and-report.md`, `desktop-backend.md`, `errors.md`, `dialects.md` |
| a **user guide** | the subtree of the surface: every name of the v1, of the v2 and of KeySharp, with the state this tree answers for each, and the crosswalk between the three. It is written in thirteen languages -- English, Español, Português (Brasil), Français, Italiano, Polski, Türkçe, Deutsch, Русский, 日本語, 中文（简体）, 繁體中文 and 한국어 -- and every page carries the switcher of the languages | `surface/README.md`, `surface/index.md`, `surface/crosswalk.md`, `surface/v2.md`, `surface/v1.md`, `surface/keysharp.md`, `surface/families/*.md` |
| a **record** | the design or the measurement of one piece of work, kept because it says *why* the tree is the way it is. The numbers and the decisions are the ones of the version that carries them | `ahk-v1-dialect.md`, `v2-capability-report.md`, `wayland-toplevel-enumeration.md`, `ahk-x11-comparison.md`, `ahk-gaps-report.md`, `file-family-v2.md`, `platforms.md`, `simd.md`, `performance.md`, `windows-backend.md`, `windows-legacy.md`, `window-style-mapping.md` |
| an **index** | this page | `README.md` |

A guide is read to use the tree. A record is read before changing it: the
record is where the measurement behind a rule lives, and a rule without its
measurement is an opinion.

---

## The index

| The document | One line | For whom |
|---|---|---|
| [language-guide.md](language-guide.md) | the language itself, from the lexical structure to the classes and the statements, with the behaviour of each construct | a new user: someone learning to write a script |
| [errors.md](errors.md) | the error model: a load error and a run error, the fourteen classes of v2, the object a catch holds, `try` / `catch` / `throw` and `OnError`, the guard rails, and the leniency of v1 | anyone whose script failed, or who writes a `catch` |
| [dialects.md](dialects.md) | the gate between the two dialects, a full v2-against-v1 reference, and how to port a script either way | someone who has a v1 script, or who moves between the dialects |
| [ahk-ks-dialect.md](ahk-ks-dialect.md) | the extensions of KeySharp -- the mode `ks` of the v2: the families of the names, the table of their surface, the machine behind them, and the audit that named them | someone writing a script against the extensions, or a contributor keeping the table of them honest |
| [ahk-v2.1-dialect.md](ahk-v2.1-dialect.md) | the v2.1 gate: the mechanisms the tree carries, what the official v2.1 (alpha.39) adds that it does not carry yet, and the forms that are KeySharp's rather than upstream's -- each with its source and its measurement | someone writing a script behind `--dialect=v2.1`, or a contributor closing the v2.1 gaps |
| [ahk-h-dialect.md](ahk-h-dialect.md) | the AutoHotkey_H forks: what the v1-H and the v2-H releases add beside the plain dialects, measured out of their CHMs and their export tables, and the plan (H0 to H8) that carries them behind `--dialect=h1` and `--dialect=h2` | a contributor picking up the H phases, or someone asking what the forks have that this tree does not |
| [surface/README.md](surface/README.md) | the index of the subtree: what the surface is, the thirteen languages it is written in, the families, the rules, and how to regenerate it. It is the CHM of the tree, in Markdown | someone who wants to know where the names live and what the tree says about them |
| [surface/index.md](surface/index.md) | the alphabetical index of every name of the three dialects, with its family and its state | someone who has a name in mind and wants the page that carries it |
| [surface/crosswalk.md](surface/crosswalk.md) | the crosswalk between the v1, the v2 and KeySharp: the names they share and the names only one of them has | someone moving between the dialects |
| [surface/v2.md](surface/v2.md) | the summary of the v2: the numbers of the runner -- the names, the value, the refused and the pending -- the numbers by family and by kind, and the names that are refused or pending | someone looking up a name of the v2: is this here, and does it do the thing? |
| [surface/v1.md](surface/v1.md) | the same summary for the v1 | someone looking up a name of the v1 |
| [surface/keysharp.md](surface/keysharp.md) | the same summary for KeySharp | someone looking up a name of KeySharp |
| [surface/families/](surface/families/) | one page per family: the ficha of every name of it -- the kind, the class, the parameters and the state -- of the three dialects, in one table | someone who wants the detail of a name, and the family that carries it |
| [ahk-v2-reference.md](ahk-v2-reference.md) | every name of AutoHotkey v2.0.32 -- the commands, the functions, the built-in variables, the class types and their methods -- with the state the tree answers for each | someone looking up a name: is this here, and does it do the thing? |
| [ahk-v1-reference.md](ahk-v1-reference.md) | the same catalogue for v1.1.37.02: the commands, the functions, the built-in variables and the directives | someone porting a v1 script, or checking what the v1 carries |
| [runner-and-report.md](runner-and-report.md) | the command `ahk`, its flags, the standard input and output, and the whole shape of the JSON report: the steps, the `error` and `error_line`, the notes, the `exit_code`, the `variables`, the includes, the pid file and the guard rails | anyone who reads a report -- a client of the runner, a test -- and anyone changing what it prints |
| [desktop-backend.md](desktop-backend.md) | how a script touches the machine: the fifteen capability traits, the uinput keyboard, the absolute pointer, the screen, the windows, the clipboard, the dialogs and the native calls, with what each one answers on a Linux box and on another host | anyone whose script types, clicks or reads the screen, and a contributor to the code that answers the machine |
| [platforms.md](platforms.md) | what runs where: the three crates, the map of the fifteen capabilities against the five hosts, the floor of each build, and the measurements behind each claim | anyone wondering which machine a build runs on, and a contributor adding a target |
| [simd.md](simd.md) | which functions a SIMD build speeds up (the vectorized text search, the encodings, the memory moves, the pixel search) and which it does not (the interpreter, the scalar math, the `Map`), and the flags of the CI/AUR/Nix SIMD builds | anyone choosing between the portable and the SIMD package, and a contributor wondering where a gain is real |
| [performance.md](performance.md) | the speed of the tree against the official binaries: the instruments (the suite of the workloads and the focused one of five million turns), the cost of a path -- a statement, a literal, a call, a property -- and the three levers that would bring the tree next to the official | a contributor to the runtime: read it before changing the path of a call, of a property or of a literal |
| [mcp.md](mcp.md) | the MCP server (`ahk --mcp`): a JSON-RPC stdio endpoint with `check`/`run`/`surface` tools to read the parse/run errors of a script, for debugging AHK development | anyone wiring the tree into an editor or agent |
| [reference/](reference/index.md) | the per-name reference, one page per name of the three dialects, CHM-shaped (description, syntax, parameters, return, errors, remarks, example, related), original to this tree and more complete than the CHM (its own behaviour + cross-platform notes); generated by `scripts/gen-reference-docs.py` | anyone who wants the full help of a name, and a contributor authoring the prose |
| [windows-backend.md](windows-backend.md) | the plan of the Windows backend: the source of AutoHotkey v2.0.32 as the reference, the seam and the seven places the trait is narrower than v2, the map of every capability against the Win32 calls it needs and the floor of each, the design in Rust (the DPI of Windows 11, the manifest, the threads), the phases of the work, and the decisions that are not the writer's | anyone who wants this tree to drive a Windows 11 machine, and a contributor picking the next piece |
| [windows-legacy.md](windows-legacy.md) | the two floors that the toolchain does not give for free: Windows XP, whose reference is the last v1 (AutoHotkey 1.1.37.02, measured XP-clean), and Windows 7, whose reference is the v2; the rule that lets the three floors live in one body of code, the crate split that the `no_std` of XP forces, the phases of each, and the gate of the import table | anyone who wants this tree to run on an old machine, and a contributor deciding what to do about the floors |
| [v2-capability-report.md](v2-capability-report.md) | the surface of the v2 counted by area -- the parser and the run, the files, the input, the windows, the screen, the native calls, the registry and the COM objects -- generated from the tree | someone asking how much of the v2 is here before writing a script against an area |
| [ahk-gaps-report.md](ahk-gaps-report.md) | what is missing in the v1 and in the v2, with the reason each hole is a hole, and the order in which they should be closed | a contributor choosing what to work on next |
| [ahk-v1-dialect.md](ahk-v1-dialect.md) | the plan that put the v1 beside the v2: the gate, the phases F0 to F5, the measurements on the binary of v1.1.37.02, the risks and what stays out | a contributor touching the parser or the built-in table: this is where the dialect is woven in |
| [wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md) | how a Wayland compositor lists its windows -- the `ext_foreign_toplevel_list_v1` and `zcosmic_toplevel_info_v1` protocols -- with the Rust that does it, compiled and run on the box | a contributor to the windows: read it before touching `WinExist` or `WinActive` |
| [window-style-mapping.md](window-style-mapping.md) | the window style as a number, the ten names that read and write it (`WinGetStyle` and its family), the text that says what to do with the style that is there, the colour a window lets through, and the mapping of the X11 attributes onto the bits of Windows -- a choice of this tree, with the reason it is one | a portable script that reads a style on two machines, and a contributor to `window_style.rs` or to the X11 layer |
| [ahk-x11-comparison.md](ahk-x11-comparison.md) | the AHK_X11 (the AHK v1.0.24 for X11, in Crystal) against this tree: what it does, what the tree does, and what is worth porting. Its code is GPL-2.0 and this tree is Apache 2.0: the behaviour may be ported, the code may not | a contributor to the hotkeys, the clipboard, the sound or the windows |
| [file-family-v2.md](file-family-v2.md) | the measurement of the file family of v2 against the binary of v2.0.19: the overwrite flag of the `FileInstall`, the wildcards and the recursion of the `FileSetTime` and of the `FileSetAttrib`, and the answer of the `DirDelete` | a contributor to the files: read it before changing `FileSetTime`, `FileSetAttrib`, `FileInstall` or `DirDelete` |
| [v2.0.31-changes.md](v2.0.31-changes.md) | the move of the tree to the dialect of the v2.0.31: what the release changed, what the two binaries answer, and what the tree carries | a contributor to the dialect: read it before changing the version, the tables or the behaviour of a name |
| [v2.0.32-changes.md](v2.0.32-changes.md) | the move of the tree to the dialect of the v2.0.32: the eight fixes of the release, the one of them that the tree did not carry, and the measurement that says the surface did not move | a contributor to the dialect: read it after the v2.0.31, before changing the version or the tables |
| [v0.6.3-changes.md](v0.6.3-changes.md) | the release 0.6.3: what it carries (the native call of the v1, the name of a hotkey of the v2, the v1 that a real program asked for), the three measurements that say it, and the gaps that are left -- six of the v2, sixty-eight of the v1 and forty-five of the other kind | a reader of the release notes, and a contributor choosing what to work on next |
| [v0.6.4-changes.md](v0.6.4-changes.md) | the release 0.6.4: the name of the v2, the name that the script builds, the hotkey of the v2 and a v1 that the tree carries -- and the gaps that are left: zero of the v2 and forty-three of the v1 | a reader of the release notes, and a contributor choosing what to work on next |
| [v0.6.5-changes.md](v0.6.5-changes.md) | the release 0.6.5: the refusals of the v2 that the tree now shares, the v1 whole (sixty-eight gaps closed), and the gaps that are left -- none of the kind that the tree refuses a script that the official loads | a reader of the release notes, and a contributor choosing what to work on next |
| [v0.6.6-changes.md](v0.6.6-changes.md) | the release 0.6.6: the buffer of the v2 that is the memory of the machine (a thousand reads of a megabyte go from 8 062 ms to 0 ms), the enumeration of a collection that is live (the entries that the loop adds are seen), and the paths that every script walks made faster -- and the gaps that are left | a reader of the release notes, and a contributor choosing what to work on next |
| [v0.6.7-changes.md](v0.6.7-changes.md) | the release 0.6.7: the loop that is compiled and the call of the script in it (the `NumPut` of a loop goes from 141 ms to 47 ms, the official takes 140), the call that names no window asking about the window that the script found last, the text of a window of another program being the text of its child windows, and the sweep of the v1 finding zero difference -- and the gaps that are left | a reader of the release notes, and a contributor choosing what to work on next |
| [corpus/v2-corpus-gaps.md](corpus/v2-corpus-gaps.md) | the 1745 real-world v2 scripts of `../AHKv2_Finetune` against the tree: what the corpus is, what the two of them answer, and the 0 gaps of the tree -- fifteen bugs that the corpus found and the tree now carries | a contributor to the dialect: the gaps that the 114 cases of the harness do not reach |
| [v1-menu.md](v1-menu.md) | the menu of the v1 measured against the machine: the popup that the machine refuses as the bar of a window, the numbers of the items, the `RemoveMenu` that keeps the menus of the script alive, and the bar that is the menu of the script | a contributor to the menus: read it before touching `menu.rs` or the window procedure |
| [v1-differences.md](v1-differences.md) | the differences of the v1 that a script of the v1 found first: the timers of a script that waits, the catch without a class, the native call of a built-in variable, the callback of a method and the icon of the window | a contributor to the runtime: read it before touching the timers, the parser or the builtins |

The numbers of `v2-capability-report.md` and `ahk-gaps-report.md` are checked
`../scripts/README.md` describes the instruments of the tree:
`check-surface-docs.py` (the numbers of the documents against the runner),
`differential.py` (the same script in the real binary, in Keysharp and in this
runner) and the corpus triage under `../egm-review/keysharp-audit/`.

against the runner by `../scripts/check-surface-docs.py`
(`python scripts/check-surface-docs.py`, and `--fix` writes the numbers of the
tree into the documents): it exits non-zero when the two disagree, and it is
meant to run in CI, which this tree does not have yet.

---

## Reading paths

### You are new, and you want to run a script

1. **[language-guide.md](language-guide.md)** -- the language, in order. It
   is written to be read from the top, and the code blocks are scripts of the
   default dialect.
2. **[runner-and-report.md](runner-and-report.md)** -- how to run what you
   wrote, and how to read what the binary answers.
3. **[errors.md](errors.md)** -- when the run fails: what the message means,
   and how a `try` catches it.
4. **[ahk-v2-reference.md](ahk-v2-reference.md)** -- when you want a name and are
   not sure it exists: the list of every name of the dialect.
5. **[desktop-backend.md](desktop-backend.md)** -- when the script has to
   type, click or read the screen: what the machine can do, and what it
   cannot.

### You want to look up a name

The subtree is written in thirteen languages -- English, Español, Português (Brasil), Français, Italiano, Polski, Türkçe, Deutsch, Русский, 日本語, 中文（简体）, 繁體中文 and 한국어
-- and every page carries the switcher of the languages at the top.

1. **[surface/README.md](surface/README.md)** -- the surface of the tree:
   the thirteen languages, the families, the rules, and how the subtree is
   regenerated.
2. **[surface/index.md](surface/index.md)** -- the alphabetical index of
   every name of the v1, of the v2 and of KeySharp, with its family and
   its state.
3. **[surface/v2.md](surface/v2.md)**, **[surface/v1.md](surface/v1.md)**,
   **[surface/keysharp.md](surface/keysharp.md)** -- the summary of the
   dialect you are in: the numbers, the names that are refused or
   pending, and the page of the family that carries the name.
4. **[surface/families/](surface/families/)** -- the ficha of the name:
   the kind, the class, the parameters and the state the tree answers.
5. **[surface/crosswalk.md](surface/crosswalk.md)** -- when the name is not
   in your dialect: what it is called in the others.

### You have a v1 script to port

1. **[dialects.md](dialects.md)** -- the v2-against-v1 reference, the
   translation table and the porting checklist, in each direction.
2. **[ahk-v1-reference.md](ahk-v1-reference.md)** -- the names your script uses, and
   what the v1 carries.
3. **[errors.md](errors.md)** -- the section on the leniency of the v1: the
   failures that a v1 script never saw, and that the v2 raises.
4. **[ahk-v2-reference.md](ahk-v2-reference.md)** -- the v2 name that replaces each
   v1 one.
5. **[ahk-v1-dialect.md](ahk-v1-dialect.md)** -- when the port needs to know
   *why* a rule is the way it is: the measurement behind it.

### You are contributing to the tree

1. **[ahk-gaps-report.md](ahk-gaps-report.md)** -- what is missing, and the
   order the plan gives the holes.
2. **[ahk-v2.1-dialect.md](ahk-v2.1-dialect.md)** -- before the v2.1
   mechanisms: what the official documentation says, what the tree
   carries, and the forms that are KeySharp's rather than upstream's.
3. **[ahk-h-dialect.md](ahk-h-dialect.md)** -- before the H phases:
   what the two AutoHotkey_H releases add, the guardrails (clean-room,
   the pure base dialects, `no_std`), and the decisions that wait.
4. **[ahk-v1-dialect.md](ahk-v1-dialect.md)** -- before the parser or the
   built-in table: the gate, the phases, and what the dialect is.
5. **[desktop-backend.md](desktop-backend.md)** -- before the machine: the
   fifteen capability traits and what a host has to answer.
6. **[wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md)** --
   before the windows, and as the model of how a protocol is spoken here.
7. **[runner-and-report.md](runner-and-report.md)** -- before changing what
   the runner prints: the report is a contract with every client of it.
8. **[v2-capability-report.md](v2-capability-report.md)** -- before claiming
   that an area is done: the numbers of the area are generated from the tree.
9. **[windows-backend.md](windows-backend.md)** -- before the Windows
   desktop: the reference, the seam, the phases, and the decisions that
   are waiting.
10. **[windows-legacy.md](windows-legacy.md)** -- before the older
   floors: what the v1 of AutoHotkey is for Windows XP, what the
   Windows 7 build needs, and how the three floors share one body of
   code.

---

## The document next door

- **[../README.md](../README.md)** -- the project overview: what this tree is,
  what it runs, and who it is for.
