# Security policy

## What this project is

An AutoHotkey interpreter written in Rust, with a backend that drives a
desktop: it types, it clicks, it reads the screen and it runs files a script
names. **A script is code, and this runtime runs it.** The security model of
the project starts there.

## What a script can do

A script has the power the runtime gives it:

- it types into whatever window has the focus, and it clicks wherever the
  pointer goes;
- it reads the screen, and it writes and deletes files with the rights of
  the user who started it;
- it runs programs (`Run`, `RunWait`), and it calls into a library of the
  machine (`DllCall`);
- on Linux it opens `/dev/uinput` and the compositor's socket, and on Windows
  it calls the libraries of the system.

The runtime is **not** a sandbox, and it does not claim to be one. Do not run
a script you have not read, and do not run a script from a stranger.

## The parts that are written not to be exploitable

- **The language crate does no I/O.** `ahk` is a lexer, a parser and an
  interpreter; everything it can do to the machine goes through the `Desktop`
  trait, and a host that wants fewer rights implements fewer capabilities.
- **The parser is a recursive descent over a checked token stream**, with a
  depth bound (`MAX_DEPTH`) and a statement bound (`MAX_STEPS`), so a script
  that recurses or loops forever stops with a report instead of taking the
  process down.
- **The two tables of the dialect are data.** A name the tree does not carry
  is refused with the reason; it is not invented at the run.

## Reporting a vulnerability

A defect that lets a script do something the model above does not give it --
memory unsafety in the runtime, a way to escape the bounds of a buffer, a
`DllCall` that reaches past what it was asked for, a parse that crashes where
it should report -- is a vulnerability, and it should be reported privately.

- **Use GitHub's private advisory form:**
  https://github.com/danalec/ahk/security/advisories/new
- Do not open a public issue for it, and do not send it in a pull request.
- If you prefer not to use GitHub, write to the address on the owner's
  GitHub profile (`https://github.com/danalec`) and say `ahk security` in the
  subject.

Please include the version or the commit, the operating system, and the
smallest script that shows the problem.

## What to expect

- An answer that says the report was read, and whether it is taken as a
  vulnerability under the model above.
- A fix and a note in the changelog, and credit in the advisory if you want
  it.

## Supported versions

The project is pre-release: the `master` branch is the only version, and a
fix lands there. There is no backport branch yet, and the note says so
rather than pretending a version is maintained.