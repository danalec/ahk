# The `ks`: the extensions of KeySharp

The `ks` is not a dialect of its own: it is the **v2** with two things
beside it -- the mechanisms of the v2.1, and the extensions of
[KeySharp](https://github.com/Firoc/KeySharp).

```sh
echo 'Msg "hello from ks"' | ahk --dialect=ks
```

| Flag | What it carries |
|---|---|
| `--dialect=v2` | the v2 -- the default |
| `--dialect=v1` | the v1 |
| `--dialect=v2.1` | the v2 with the mechanisms of the v2.1 |
| `--dialect=ks` | the v2 with the mechanisms of the v2.1 **and** the extensions of KeySharp |
| `--dialect=ks20` | the extensions of KeySharp without the mechanisms of the v2.1 |

The two switches are apart from each other: `ks20` is the KeySharp on
the v2 as it is, and `ks` is the KeySharp on the v2.1.

## The surface

The surface of the `ks` is the surface of the v2 (833 names) with **130
more**:

| | names |
|---|---|
| classes | 16 |
| functions | 30 |
| methods | 43 |
| variables | 41 |

```sh
ahk --surface --dialect=ks
ahk --surface --dialect=ks --json
```

The whole surface of the `ks` is 969 names: 953 answer with a value, 16
are refused with the reason the machine cannot give one, and none is
pending.

## The table of the surface

`crates/ahk/reference/ahk-ks.surface.tsv` is the contract of the
extensions, the way the table of the v2 is the contract of the v2. It
carries the **138 names of KeySharp that need neither Windows, nor .NET,
nor a GUI** -- the denominator of the audit -- with the kind of each one
and the class a member belongs to.

The names are written by the logic of the audit itself, out of the frozen
files of `egm-review/keysharp-audit/surface/`, minus the ones that need
Windows, or .NET, or a GUI, and minus the ones that are names of the v2
(those are parity, and the table of the v2 has them).

The table is read when the tests are compiled, and the parity test asks
the tree about every name of it: a name the tree stops carrying fails
the build of a test and not a run of a script. The question is asked of
the tree and not of one dialect -- three of the names (`A_Args`,
`EnvUpdate` and `IsSet`) are names of the v1, and the tree carries them
in the v1, which is the dialect that has them.

## What the extensions are, by family

- **The process of a script.** `RunScript` starts a script and answers a
  `ScriptProcess`: `HasExited`, `ExitCode`, `ExitTime`, `Kill`, `StdOut`,
  `StdErr`, `StdIn`. The state of a process is the machine's: `/proc` on
  Linux, `OpenProcess` on Windows. The moment of the exit is the moment
  of the wait, written the way KeySharp writes a moment.
- **The pointers.** `ObjCreateWithDataPtr`, `ObjGetDataPtr`,
  `ObjGetDataSize`, `ObjSetDataPtr`, and the `Struct` class with its
  `At` -- a view of the memory of the machine at a place, of a type that
  is asked for.
- **The typed numbers.** `Int8`, `UInt8`, `Int16`, `UInt16`, `Int32`,
  `UInt32`, `Int64`, `IntPtr`, `Float32`, `Float64`.
- **The machine.** `Mail` sends a message through an SMTP server;
  `GetKeyboardLayout` answers the keymap of the session; `GetKeyInfo`
  answers the state of a key; `ClipCursor` confines the pointer;
  `WinFromPoint` answers the window at a place; `ShowDebug` and
  `OutputDebugLine` write to the log of the machine.
- **The arrays and the maps.** `Join`, `IndexOf`, `FindIndex`, `Filter`,
  `Contains`, `Remove`, `StartsWith`, `EndsWith`, `ToString` -- the
  methods that take a collection and answer another one.
- **The threads and the hotkeys.** `A_RealThread` and the `Thread` class
  (`Id`, `IsActive`, `Kind`, `Priority`, `AllowTimers`, `Paused`), the
  `Hotkey` class with its `BeforeHotkeys` and `Enabled`, and the
  settings of the hotstrings (`A_DefaultHotstringEndChars` and the
  rest).
- **The errors of the mode.** `ArgumentError` and `UnsupportedError`,
  beside the tree of the errors of the v2.

## The machine behind them

A name of the `ks` is carried when the tree answers it, and the answer
comes from the machine:

| Name | What answers it |
|---|---|
| `Mail` | the SMTP server, through `curl` |
| `GetKeyboardLayout` | the keymap of the X server, read with `setxkbmap` |
| `ClipCursor` | the clamp of the coordinates the pointer sends |
| `WinFromPoint` | the X server, with `XTranslateCoordinates` |
| `ScriptProcess.HasExited` | `/proc/<pid>/stat`, or `OpenProcess` on Windows |
| `ScriptProcess.ExitTime` | the moment of the wait, in the local time |

The confinement of the pointer is the clamp of the coordinates and not a
grab, the way KeySharp clamps the events of the mouse in its hook: a
grab would take the pointer events of the whole session to the client
that grabbed it, and the clamp cannot do that.

## The audit

The audit of the KeySharp surface is under `egm-review/keysharp-audit/`
-- the plan, the scorecard and the frozen files of the surface of
KeySharp. The scorecard answers one question:

```
python egm-review/keysharp-audit/score.py
```

```
The carried: 138 of 138 = 100.0 % of the platform-neutral Keysharp surface surpassed.
```

The denominator is the 138 names of the table of this tree; the count is
what the tree answers with a value, in either dialect. A name the tree
carries only as a refusal is not carried: it is a name the tree knows
and does not do.

## What is not carried

The names that need a platform are not carried, and the tree says so
instead of pretending: the surface of the `ks` carries the 16 names of
the v2 that are shaped for Windows, and it refuses each one with the
reason.

The gaps that are left are the ones of the machine, and they are the
same ones the v2 has: the state of a process that is not ours is known
but its code is not, the moment of the exit of a process that is not
ours is not kept by the kernel, the confinement of the pointer is the
clamp of the pointer of the tree and not of the mouse of the user, and
the mail needs a server that answers.
