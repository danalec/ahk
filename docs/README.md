# The documents of this tree

The guides, the references and the records behind the code. Start here when
you do not know which file you want: every document of the folder is listed
with what it holds and who it is for.

- [Three kinds of document](#three-kinds-of-document)
- [The index](#the-index)
- [Reading paths](#reading-paths)
  - [You are new, and you want to run a script](#you-are-new-and-you-want-to-run-a-script)
  - [You have a v1 script to port](#you-have-a-v1-script-to-port)
  - [You are contributing to the tree](#you-are-contributing-to-the-tree)
- [The two documents next door](#the-two-documents-next-door)

---

## Three kinds of document

| The kind | What it is | The documents |
|---|---|---|
| a **user guide** | written for someone who runs scripts or writes them: plain sentences, examples, the behaviour of the language | `language-guide.md`, `ahk-v2-reference.md`, `ahk-v1-reference.md`, `runner-and-report.md`, `desktop-backend.md`, `errors.md`, `dialects.md` |
| a **record** | the design or the measurement of one piece of work, kept because it says *why* the tree is the way it is. The numbers and the decisions are the ones of the day it was written | `ahk-v1-dialect.md`, `v2-capability-report.md`, `wayland-toplevel-enumeration.md`, `ahk-x11-comparison.md`, `ahk-gaps-report.md` |
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
| [ahk-v2-reference.md](ahk-v2-reference.md) | every name of AutoHotkey v2.0.19 -- the commands, the functions, the built-in variables, the class types and their methods -- with the state the tree answers for each | someone looking up a name: is this here, and does it do the thing? |
| [ahk-v1-reference.md](ahk-v1-reference.md) | the same catalogue for v1.1.37.02: the commands, the functions, the built-in variables and the directives | someone porting a v1 script, or checking what the v1 carries |
| [runner-and-report.md](runner-and-report.md) | the command `ahk`, its flags, the standard input and output, and the whole shape of the JSON report: the steps, the `error` and `error_line`, the notes, the `exit_code`, the `variables`, the includes, the pid file and the guard rails | anyone who reads a report -- a client of the runner, a test -- and anyone changing what it prints |
| [desktop-backend.md](desktop-backend.md) | how a script touches the machine: the fifteen capability traits, the uinput keyboard, the absolute pointer, the screen, the windows, the clipboard, the dialogs and the native calls, with what each one answers on a Linux box and on another host | anyone whose script types, clicks or reads the screen, and a contributor to the code that answers the machine |
| [v2-capability-report.md](v2-capability-report.md) | the surface of the v2 counted by area -- the parser and the run, the files, the input, the windows, the screen, the native calls, the registry and the COM objects -- generated from the tree | someone asking how much of the v2 is here before writing a script against an area |
| [ahk-gaps-report.md](ahk-gaps-report.md) | what is missing in the v1 and in the v2, with the reason each hole is a hole, and the order in which they should be closed | a contributor choosing what to work on next |
| [ahk-v1-dialect.md](ahk-v1-dialect.md) | the plan that put the v1 beside the v2: the gate, the phases F0 to F5, the measurements on the binary of v1.1.37.02, the risks and what stays out | a contributor touching the parser or the built-in table: this is where the dialect is woven in |
| [wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md) | how a Wayland compositor lists its windows -- the `ext_foreign_toplevel_list_v1` and `zcosmic_toplevel_info_v1` protocols -- with the Rust that does it, compiled and run on the box | a contributor to the windows: read it before touching `WinExist` or `WinActive` |
| [ahk-x11-comparison.md](ahk-x11-comparison.md) | the AHK_X11 (the AHK v1.0.24 for X11, in Crystal) against this tree: what it does, what the tree does, and what is worth porting. Its code is GPL-2.0 and this tree is Apache 2.0: the behaviour may be ported, the code may not | a contributor to the hotkeys, the clipboard, the sound or the windows |

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

1. **[desktop-backend.md](desktop-backend.md)** -- the map of the machine: the
   fifteen capability traits and what a host has to answer. Start here, and
   follow the document that matches what you are about to touch.
2. **[ahk-gaps-report.md](ahk-gaps-report.md)** -- what is missing, and the
   order the plan gives the holes.
3. **[ahk-v1-dialect.md](ahk-v1-dialect.md)** -- before the parser or the
   built-in table: the gate, the phases, and what the dialect is.
4. **[desktop-backend.md](desktop-backend.md)** -- before the machine: the
   fifteen capability traits and what a host has to answer.
5. **[wayland-toplevel-enumeration.md](wayland-toplevel-enumeration.md)** --
   before the windows, and as the model of how a protocol is spoken here.
6. **[runner-and-report.md](runner-and-report.md)** -- before changing what
   the runner prints: the report is a contract with every client of it.
7. **[v2-capability-report.md](v2-capability-report.md)** -- before claiming
   that an area is done: the numbers of the area are generated from the tree.

---

## The document next door

- **[../README.md](../README.md)** -- the project overview: what this tree is,
  what it runs, and who it is for.