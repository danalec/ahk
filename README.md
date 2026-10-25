# yoko-rust

The yoko lab, in Rust: the AutoHotkey engine, the lab MCP, and the MCP
that speaks AHK.

```
crates/ahk/           the language: a lexer, an AST, a parser, an interpreter
crates/ahk-box/       the box primitives -- runs on the lab box
crates/ahk-cli/       the runner: a script on stdin, a report on stdout
crates/yoko-mcp/      the lab MCP: discover, capture_screen, input_type, input_probe
crates/yoko-ahk-mcp/  the AHK MCP: ahk_run, ahk_check, ahk_keys, ahk_probe
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

The language is AutoHotkey's, cut down to what a script can do to a
Linux desktop: `Send`, `Click`, `Sleep`, `Run`, `PixelGet`,
`Screenshot`, `Msg`, `If`/`Else`, `While`, `Loop`, `Break`, `Continue`,
the assignment operators, `%name%` interpolation, and the expressions
in between.

## The three input paths are not equal

Everything here was measured on the lab box (CachyOS, COSMIC Wayland,
ydotool 1.0.4) on 2026-10-25. The difference is not a detail:

| path | what happens |
|---|---|
| **text** — `ydotool type` | **exact.** Every character arrives once, in order. A newline arrives as a Return. |
| **a key** — the uinput keyboard in `ahk-box` | **exact.** An explicit keydown and keyup, so nothing repeats. |
| **a modifier** — ctrl, alt, shift | **not delivered.** A modifier never reaches the focused surface on this box, whatever device it comes from. The report says so instead of pretending. |

`ydotool key` is **not used**, and the reason is worth writing down: on
ydotool 1.x a key *name* is read as zero and does nothing at all (the
command exits 0 and the screen does not change), and a numeric *code*
presses the key without ever releasing it, so the compositor's
auto-repeat types it forever. One `ydotool key 22` produced about a
hundred and thirty `u`s and left the key held; the modifier suffix
(`30:1` for ctrl+a) is not understood at all.

Two more measured details, both of which cost time to find:

- A client **coalesces** a burst of key events. A hundred backspaces
  that arrive back to back are counted as a few. The keyboard leaves a
  gap between the release and the next press, and with that gap every
  one of them counts.
- The uinput device needs **four seconds** before the compositor adopts
  it, and the pointer needs five.

## The report

```json
{
  "ok": true,
  "steps": [
    {"line": 1, "op": "Msg", "detail": "hello from the box", "ok": true, "elapsed_ms": 0},
    {"line": 3, "op": "Send", "detail": "typed 8 characters", "ok": true, "elapsed_ms": 304},
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

`yoko-mcp`:

- **`discover`** — what is really on the box: binaries, pids, ports, capture and input tools.
- **`capture_screen`** — a frame of the COSMIC Wayland session, via `grim`.
- **`input_type`** — text, through `ydotool type`, which is exact. With `submit`, a Return is appended.
- **`input_probe`** — whether the box can take input at all.

`yoko-ahk-mcp`:

- **`ahk_run`** — run a script on the box, in memory. The report says what every statement did.
- **`ahk_check`** — parse a script without running it; a syntax error comes back with the line and the column.
- **`ahk_keys`** — the key table, and which of them this box can deliver.
- **`ahk_probe`** — measure the box's input and capture paths right now.

## Build and test

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
```

The tests do not need the box: the desktop is a fake that records the
calls, so they are about what the language means, not about what the
desktop does.

## On the box

The runtime has to be there:

```sh
ssh 192.168.0.40 'cd ~/yoko-rust && cargo build --release -p ahk-cli'
```

`ahk_probe` says whether it is.

## End-to-end verified on 2026-10-25

Against the real `192.168.0.40` box, the Rust engine ran:

```ahk
Msg "the runtime is alive"
Run "echo the box ran the script"
PixelGet 100, 100
```

in 47 ms, and `PixelGet` read `#12101b` -- the same colour the Python
`yoko-ahk-mcp` reads, and the one the Python `yoko-mcp` README records
for that pixel of the Orion wallpaper: three implementations, one
answer. A second script typed eight characters in 304 ms and pressed
Backspace twelve times through the uinput keyboard, each in 40 ms,
without a single repeated character.
