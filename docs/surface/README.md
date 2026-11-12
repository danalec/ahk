# The surface of the tree

Every name of the three dialects, in one place, with what **this tree**
answers for each of them today.

This is the help of the tree -- the thing the official AutoHotkey CHM is
for the official binary. It covers the v1.1.37.02, the v2.0.19 and the
extensions of KeySharp, and it answers two questions for every name: what
the name is, and whether the tree carries it.

The pages of this subtree are **generated**. The tables of the dialects
are the contract of the tree
(`crates/ahk/reference/*.surface.tsv`) and the state of every name is the
answer of the runner (`ahk --surface --json --dialect=v2|v1|ks`). The
generator writes them, and the check that CI is meant to run compares them
with the tree.

## The five languages

| Language | Where it lives |
|---|---|
| English | the root of this subtree -- `index.md`, `crosswalk.md`, `v2.md`, `v1.md`, `keysharp.md`, `families/` |
| Español | `es/` |
| Português (Brasil) | `pt-BR/` |
| 日本語 | `ja/` |
| 中文 | `zh/` |

Every page carries the switcher of the languages at the top, so a reader
of one language can walk to the same page in another one.

What is translated is the prose of the pages and the words that describe a
state -- the kind of a name, the state of it, the reason of a refusal, the
title of a family. What is **not** translated is the surface itself: a
script writes `FileAppend` in every language of the world, and the class
`Gui.Control` is `Gui.Control` in the Japanese page too.

## The pages

| The page | What it holds |
|---|---|
| [index.md](index.md) | every name of the three dialects, alphabetical, with its family and its state |
| [crosswalk.md](crosswalk.md) | the v1 against the v2 against KeySharp: the names they share and the names only one of them has |
| [v2.md](v2.md) | the surface of the v2: the numbers by family and by kind, and the names that are refused or pending |
| [v1.md](v1.md) | the same for the v1 |
| [keysharp.md](keysharp.md) | the same for the extensions of KeySharp |
| [families/](families/) | one page per family: the **ficha** of every name of it -- kind, class, parameters, state -- of the three dialects, in one table |

## The families

A family is the area a script thinks in. The families are the areas of
the capability report
(`crates/ahk/reference/ahk-v2.0.19.areas.py`), so the two documents
cannot disagree about where a name lives:

| The family | What it holds |
|---|---|
| script | the parser and the execution of the script: the control flow, the timers, the directives |
| files | the files, the directories and the INI |
| input | the hotkeys, the hotstrings, the keyboard and the pointer |
| gui | the windows of the GUI and the automation of the windows |
| screen | the capture of the screen and the pixels |
| native | the calls to the native code: `DllCall`, `NumGet`, `NumPut` |
| registry | the registry of Windows |
| com | the automation COM, the `ComObject` |
| core | the core of the language: text, mathematics and patterns |
| objects | the objects of the language: `Array`, `Map`, `Func` |
| time | the dates and the times |
| machine | the machine, the processes and the environment |
| clipboard | the clipboard |
| sound | the sound |
| network | the network and the mail |

## The rules

* A name is a **command**, a **function**, a **variable**, a **method**,
  a **class** or a **directive**. A method of a class is written
  `Class.Method`.
* The **state** of a name is the answer of the runner:
  * *value* -- the tree carries the name and does the thing;
  * *refused* -- the tree carries the name and refuses it **with the
    reason**. A question the desktop cannot answer fails; it never
    answers `false`;
  * *pending* -- the tree does not carry the name yet.
* The **parameters** of a name are the ones the reference table measured:
  the fewest and the most. The v1 table carries the shape of each one as
  well (`Value`, `Variable`, `Output`).
* A name of the v1 that the v2 spells another way is a line of the
  v1-only table of the crosswalk, with the v2 name that replaced it.

## How the pages are made

```sh
python scripts/gen-surface-docs.py            # write docs/surface/
python scripts/gen-surface-docs.py --check    # fail when the tree moved
python scripts/gen-surface-docs.py --lang es  # write one language only
python scripts/check-surface-docs.py          # the check, with the rest
```

The generator is `scripts/gen-surface-docs.py`; the families come from
`scripts/surface_areas.py` and the translations from
`scripts/surface_i18n.py`. Both are Python 3 standard library only.

A name that no family claims lands on the page
`families/unclassified.md`, and the generator says so on the stdout: a
name in the wrong family is visible instead of silent.
