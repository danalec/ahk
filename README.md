# yoko-rust

The yoko lab, in Rust: the AutoHotkey engine, the lab MCP, and the MCP
that speaks AHK.

```
crates/ahk/           the language: a lexer, an AST, a parser, an interpreter
crates/ahk-box/       the box primitives -- runs on the lab box
crates/ahk-cli/       the runner: a script on stdin, a report on stdout
crates/yoko-mcp/      the lab MCP: the box, the screen, Sunshine, Moonlight
crates/yoko-ahk-mcp/  the AHK MCP: ahk_run, ahk_stop, ahk_check, ahk_keys, ahk_probe
```

The layering is the point: the AHK MCP is built on the other two. The
language and the runtime are a library, the box primitives are a library
that runs on the box, and the MCP is the surface an agent talks to.

## The AHK engine

The parser is portable and does no I/O of its own. The runtime runs
**on the box**, because that is where the desktop is:

```
MCP                          box
────                          ────
parse the script  ──source──▶  parse it again
                ◀──report──   execute it, statement by statement
```

The script is handed over in memory and never touches the disk.

```ahk
; The tree on the box
Msg "hello from the box"

; Type text, exactly
Send "invoice-{Enter}"

; Read the screen
colour := PixelGet(100, 100)
If colour = "#12101b" {
    Msg "the wallpaper is there"
}

; Drive it
Click 400, 300
Run "echo done"
```

## The language

AutoHotkey's, cut down to what a script can do to a Linux desktop.

**Statements.** `Send`, `Click`, `MouseMove`, `Sleep`, `Run`, `PixelGet`,
`Screenshot`, `Msg`, `If`/`ElseIf`/`Else`, `Switch`/`Case`/`Default`,
`While`, `Loop`, `Break`, `Continue`, `Return`, `ExitApp`, and the
assignment operators (`=`, `:=`, `+=`, `-=`, `.=`).

**Functions.** Your own, with a block and a `Return`:

```ahk
Add(a, b) {
    return a + b
}
total := Add(2, 3)
```

**The builtins.** `Abs`, `Sqrt`, `Mod`, `Floor`, `Ceil`, `Round`, `Min`,
`Max`, `StrLen`, `SubStr`, `StrReplace`, `InStr`, `Trim`, `Upper`,
`Lower`, `Number`, `Format`, `Sleep`, and the screen ones below.

The positions count from one, the way AutoHotkey counts them: `SubStr("hello", 2)`
is `ello`, and `InStr("hello", "ll")` is `3`.

## The focus check

A script that types into the wrong window is worse than a script that
fails, so the language carries the primitives that answer "is the right
thing in front of us?".

| call | what it answers |
|---|---|
| `MouseGetPos()` | where the pointer is. A list: `pos[1]`, `pos[2]`. |
| `PixelGet(x, y)` | the colour of one pixel, as `#rrggbb`. |
| `PixelSearch(x1, y1, x2, y2, colour)` | where that colour is. A list, or an empty one. |
| `WinActive(title)` | is that window the active one? |
| `WinExist(title)` | is that window there? |
| `WinGetTitle()` | the title of the active window. |

The first three are read from the screen itself, so they work for any
window, Wayland or not:

```ahk
MouseMove 960, 540
pos := MouseGetPos()
colour := PixelGet(pos[1], pos[2])
If colour = "#12101b" {
    Msg "the right window is under the pointer"
}
```

The last three are the window-level questions, and they are answered by
whatever the desktop can actually ask. On the lab box the session is
Wayland and the compositor exposes **no window list** -- the COSMIC
services answer `Introspect` with nothing, and the X server beside it
(the rootless Xwayland of gamescope) reports an empty client list. So
`WinActive` and `WinExist` **fail loudly** with the reason instead of
answering `false`:

> `WinActive cannot answer: the X server sees the active X11 window as
> "Steam", but this session is Wayland: a native Wayland window can hold
> the focus without appearing in X11. Use PixelSearch or MouseGetPos with
> PixelGet to check the focus.`

A script that cannot answer the question fails; it does not quietly take
the wrong branch.

## The input paths

Everything here was measured on the lab box (CachyOS, COSMIC Wayland)
on 2026-10-25. The difference is not a detail.

**Text and keys go through a uinput keyboard of our own.** The text used
to go through `ydotool type`, which was exact -- but `/usr/bin/ydotoald`
was removed from the disk of that box, the process that was still
running has died, and `ydotool type` without a daemon writes into a
socket nobody is listening on and reports nothing. The keyboard of our
own is there, and it delivers: every character arrives once, in order,
capitals included (the shift key works; only ctrl and alt do not).

**`ydotool key` is not used**, and the reason is worth writing down: on
ydotool 1.x a key *name* is read as zero and does nothing at all (the
command exits 0 and the screen does not change), and a numeric *code*
presses the key without ever releasing it, so the compositor's
auto-repeat types it forever. One `ydotool key 22` produced about a
hundred and thirty `u`s and left the key held; the modifier suffix
(`30:1` for ctrl+a) is not understood at all.

**The mouse goes through an absolute pointer of our own**, because
ydotool's device is a *relative* mouse with a full keyboard key bitmap
and no `INPUT_PROP_POINTER`: libinput classifies that as a keyboard, the
relative axes are ignored, and a click lands nowhere. The pointer of our
own announces `INPUT_PROP_DIRECT` and absolute axes, like the one
Sunshine creates, and a click lands exactly where it was aimed.

### What the devices have to look like

Three things had to be right, and each of them cost a measurement.

- **The keyboard must announce itself like a real keyboard.** The codes
  are the 112 the box's own `AT Translated Set 2 keyboard` announces,
  read out of `/proc/bus/input/devices`; the scan codes go with the
  key events; the bus is the virtual one. With a sparse list of codes,
  or with codes a real keyboard does not have, libinput builds a keymap
  it cannot use: the key events arrive at the compositor and are
  reported with the key code `-1`, which a client cannot use.
- **A new device needs four seconds before the compositor adopts it.**
  A click that arrives before the enumeration is a click that lands
  nowhere -- and the report says the click happened, because the
  keyboard did accept it.
- **The axes of the pointer must change.** The kernel does not repeat an
  axis value that did not move: a click at the position the pointer is
  already at produces a press with no motion, and the press lands
  where the pointer was.

Two more measured details, both of which cost time to find:

- A client **coalesces** a burst of key events. A hundred backspaces
  that arrive back to back are counted as a few. The keyboard leaves a
  gap between the release and the next press, and with that gap every
  one of them counts.
- The pointer needs a **200 ms pause** between the motion and the
  press. The compositor applies a press that arrives in the same frame
  as the motion at the *previous* position.

## The emergency stop

A script that types forever has to be stoppable from outside, and the
only thing that can stop it is the process.

The runtime writes its pid to `~/.cache/yoko-ahk/run.pid` and removes it
when it ends. The `ahk_stop` tool reads that file, sends `TERM`, then
`KILL`, then sweeps up whatever is left.

From a shell, when the MCP itself is stuck and cannot answer:

```sh
ssh 192.168.0.40 'pkill -f release/ahk'
```

Verified live: the pid file was written, the process was running, the
stop killed it, the file was gone, and the one-liner found nothing left.

## The report

```json
{
  "ok": true,
  "steps": [
    {"line": 1, "op": "Msg", "detail": "hello from the box", "ok": true, "elapsed_ms": 0},
    {"line": 3, "op": "Send", "detail": "typed 8 characters through the uinput keyboard", "ok": true, "elapsed_ms": 304},
    {"line": 4, "op": "PixelGet", "detail": "(100, 100) = #12101b", "ok": true, "elapsed_ms": 68}
  ],
  "output": ["hello from the box"],
  "notes": ["WAYLAND_DISPLAY=wayland-1 (XDG_RUNTIME_DIR=/run/user/1000)"],
  "error": null,
  "elapsed_ms": 380,
  "variables": {"colour": "#12101b"}
}
```

The `notes` are the places where the box was not what the script
assumed, and they are the point of the report: a script that drives a
desktop has to be able to say what it did, not just that it finished.

## The servers

`yoko-mcp` and `yoko-ahk-mcp` speak the MCP protocol over stdio. The
protocol is small enough to speak directly -- newline-delimited JSON-RPC
2.0, four methods -- so the crate carries its own loop instead of a
dependency, and the other server reuses it: the two answer an agent the
same way.

`yoko-mcp` (21 tools):

- **`discover`** -- what is really on the box: binaries, pids, ports, capture and input tools.
- **`capture_screen`** -- a frame of the COSMIC Wayland session, via `grim`.
- **`input_type`** -- text, through the AHK runtime on the box, which drives its own keyboard. With `submit`, a Return is appended.
- **`input_probe`** -- whether the box can take input at all.
- **`ui_probe`**, **`ui_capture_region`**, **`ui_capture_full`**, **`ui_get_pixel`**, **`ui_sample_region`**, **`ui_diff`**, **`ui_screensaver_active`**, **`ui_notifications`** -- what is on the screen.
- **`sunshine_get_config`**, **`sunshine_set_config`**, **`sunshine_applications`**, **`sunshine_clients_list`**, **`sunshine_unpair_client`**, **`sunshine_restart`** -- the Sunshine API of the box.
- **`moonlight_version`**, **`moonlight_list_apps`**, **`moonlight_quit`** -- the Moonlight client on this machine.

`yoko-ahk-mcp`:

- **`ahk_run`** -- run a script on the box, in memory. The report says what every statement did.
- **`ahk_stop`** -- stop the script that is running. The emergency stop.
- **`ahk_check`** -- parse a script without running it; a syntax error comes back with the line and the column.
- **`ahk_keys`** -- the key table, and which of them this box can deliver.
- **`ahk_probe`** -- measure the box's input and capture paths right now.

## Build and test

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
```

The tests do not need the box: the desktop is a fake that records the
calls, so they are about what the language means, not about what the
desktop does.

The code that touches the box -- the uinput devices, the ioctls -- is
behind `#[cfg(target_os = "linux")]`, so **a Windows build does not
compile it**. The box is where that code is checked:

```sh
ssh 192.168.0.40 'cd ~/yoko-rust && cargo build --release -p ahk-cli'
```

## On the box

The runtime has to be there:

```sh
ssh 192.168.0.40 'cd ~/yoko-rust && cargo build --release -p ahk-cli'
```

`ahk_probe` says whether it is.

## Verified on the box, 2026-10-25

Against the real `192.168.0.40`:

- `PixelGet 100, 100` read `#12101b` -- the same colour the Python
  `yoko-ahk-mcp` reads, and the one the Python `yoko-mcp` README records
  for that pixel of the Orion wallpaper: three implementations, one
  answer.
- `Send "Teste-Yoko 42!"` typed the whole string, capitals and
  punctuation included, into the focused field of the Files window.
- Seven backspaces deleted the seven characters before the caret.
- A click at `(518, 305)` selected the "Documents" entry of the Files
  sidebar -- the entry at exactly that point -- and the window listed
  its files.
- The focus check answered `pointer at 960,540 - colour #1b1b1b`, and
  `MouseGetPos()` read back from the kernel what the pointer had been
  put at.
- A script with a function, a `Switch`, a `Return` and a built-in ran
  and reported each step.
- The emergency stop killed a script that was typing forever.
