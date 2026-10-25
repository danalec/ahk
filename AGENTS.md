# AGENTS.md — working notes for yoko-rust

This repository is the yoko lab in Rust: the AutoHotkey engine, the box
primitives, and the two MCP servers. It is a port of two Python
repositories (`yoko-mcp` and `yoko-ahk-mcp`) that live next door, and
the two have to agree: the Python is the reference, the Rust is the
implementation that is being built.

The rules below are the ones that are easy to break by accident. Each one
says what the rule is and why it is there.

## 1. The three trees, and what may be changed in them

| tree | whose it is | what you may do |
|---|---|---|
| `crates/ahk` | the language: lexer, AST, parser, runtime | freely |
| `crates/ahk-box` | the box primitives -- runs on the lab box | freely |
| `crates/yoko-mcp`, `crates/yoko-ahk-mcp` | the MCP servers | freely |

**The Python is the reference.** When the language grows, it grows in
both places, with the same tests. A feature that exists in only one of
them is a bug in the port, not a difference.

## 2. The code that touches the box only compiles on the box

Everything that opens `/dev/uinput`, writes ioctls, or reads an event
node is behind `#[cfg(target_os = "linux")]`.

```sh
cargo build --workspace        # on Windows: the box code is not compiled
ssh 192.168.0.40 'cd ~/yoko-rust && cargo build --release -p ahk-cli'
```

**A green Windows build says nothing about the box code.** It was green
while the box build failed on a type error for two days. The box build
is the check that matters.

## 3. The uinput devices have to announce themselves like real ones

This is the rule that cost the most time, and it is not obvious.

**The keyboard announces the 112 codes of the box's own keyboard.** The
list was read out of `/proc/bus/input/devices` on 2026-10-25 (the
`AT Translated Set 2 keyboard`). With a sparse list, or with codes a
real keyboard does not have, libinput builds a keymap it cannot use: the
key events arrive at the compositor and are reported with the key code
`-1`, which a client cannot use. The events are in the event node the
whole time -- `cat /dev/input/eventN` shows them -- and they stop there.

The scan codes go with the key events (`EV_MSC`/`MSC_SCAN` before the
key), and the device announces the MSC bits.

**The pointer announces `INPUT_PROP_DIRECT` and absolute axes**, like the
device Sunshine creates on that box. ydotool's device is a *relative*
mouse with a full keyboard key bitmap and no `INPUT_PROP_POINTER`, so
libinput classifies it as a keyboard and ignores the relative axes: the
click arrives and moves nothing.

## 4. Timing on the box is not a detail

- **A new device needs four seconds before the compositor adopts it.**
  A click that arrives before the enumeration lands nowhere -- and the
  report says the click happened, because the keyboard did accept it.
  The keyboard and the pointer both wait.
- **The pointer needs 200 ms between the motion and the press.** The
  compositor applies a press that arrives in the same frame as the
  motion at the *previous* position. Measured: with a 20 ms gap the
  click landed hundreds of pixels away from where it was aimed.
- **A client coalesces a burst of key events.** A hundred backspaces
  that arrive back to back are counted as a few. The keyboard leaves
  20 ms between the release and the next press, and with that gap
  every one of them counts.
- **The kernel does not repeat an axis value that did not move.** A
  click at the position the pointer is already at produces a press with
  no motion. The pointer of the Python lab moves between two points for
  exactly this reason.

## 5. The text goes through our own keyboard

`ydotool type` was exact while `ydotoald` was running on that box. It
is not any more: `/usr/bin/ydotoald` was removed from the disk of that
box, the process that was still running has died, and `ydotool type`
without a daemon writes into a socket nobody is listening on and reports
nothing.

**`ydotool key` is never used.** ydotool 1.x reads a key *name* as zero
and does nothing at all (the command exits 0 and the screen does not
change), and a numeric *code* presses the key without ever releasing
it, so the compositor's auto-repeat types it forever. One
`ydotool key 22` produced about a hundred and thirty `u`s and left the
key held. The modifier suffix (`30:1` for ctrl+a) is not understood at
all.

## 6. The modifiers arrive -- and the note that said they do not was wrong

The first note said "a modifier never reaches the client on this box".
That was measured with the **ydotool** device, whose `key 30:1` ignores
the suffix: the chord was never sent.

With a keyboard of our own the chord is sent -- the modifier goes down,
the key goes down, the key comes up, the modifier comes up -- and a GTK
client acts on it: measured on 2026-10-25, `ctrl+a` selected the text of
a field and the next key replaced it.

**The shift key is what makes capitals work** in the text path, and it
works for the same reason.

## 7. The focus check, and the window questions it cannot answer

`MouseGetPos`, `PixelGet` and `PixelSearch` are read from the screen, so
they work for any window.

`WinActive` and `WinExist` need an API of windows, and the box has
none: the session is Wayland, the COSMIC services answer `Introspect`
with nothing, and the X server beside it (the rootless Xwayland of
gamescope) reports an empty client list. **The answer is therefore
"unknown", and the runtime says so out loud** -- a script that cannot
answer the question fails; it does not quietly take the wrong branch.

## 8. The emergency stop

A script that types forever has to be stoppable from outside, and the
only thing that can stop it is the process.

The runtime writes its pid to `~/.cache/yoko-ahk/run.pid` and removes
it when it ends. The `ahk_stop` tool reads that file, sends `TERM`,
then `KILL`, then sweeps up whatever is left.

From a shell, when the MCP itself is stuck and cannot answer:

```sh
ssh 192.168.0.40 'pkill -f release/ahk'
```

## 9. The traps of the transport

- **The environment probe's arguments are in the order the format asks
  for them.** With them the wrong way round, the environment is *set* and
  every Wayland tool on the box fails to connect, and the failure says
  nothing about the cause.
- **`~` is not expanded inside quotes.** A quoted `~` is a file named
  `~`, and the shell says "no such file or directory". The path of the
  runtime is passed unquoted for that reason.
- **The geometry belongs to the command, not in front of it.** The
  environment is put in front of the command, and `env ... -g ...` would
  ask `env` to run a program called `-g`.
- **`base64 -w0` does not exist on that box** (its `base64` is the
  BusyBox one). The output has newlines in it: `base64 | tr -d '\n'`.
- **`grim -o` is the output *format*, not the file.** `grim -o
  /tmp/x.png` answers "unknown output". The file is the argument:
  `grim /tmp/x.png`. And `grim -` writes the PNG to stdout.
- **`id -u` and `ls` are BusyBox on that box**: no `pgrep -o`, and
  `base64` without the GNU options.

## 10. The report never lies

A script that drives a desktop has to be able to say what it did, not
just that it finished.

- Every statement produces a step: the line, what it did, whether it
  worked, how long it took.
- The `notes` are the places where the box was not what the script
  assumed -- the environment that was found, the device that had to be
  waited for.
- A question that cannot be answered fails, with the reason. It does
  not answer `false`.

## 11. The commands

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets

# the box, where the box code is compiled and run
ssh 192.168.0.40 'rm -rf ~/yoko-rust/crates && cd ~/yoko-rust'
scp -r crates danalec@192.168.0.40:~/yoko-rust/
ssh 192.168.0.40 'cd ~/yoko-rust && cargo build --release -p ahk-cli'

# a script, on the box
printf %s <base64> | base64 -d | ~/yoko-rust/target/release/ahk --timeout=60
```

## 12. What is verified, and what is not

Verified on the box on 2026-10-25, with the real `192.168.0.40`:

- `PixelGet 100, 100` reads `#12101b` -- the same colour the Python
  `yoko-ahk-mcp` reads, and the one the Python `yoko-mcp` README records
  for that pixel of the Orion wallpaper: three implementations, one
  answer.
- `Send "Teste-Yoko 42!"` types the whole string, capitals and
  punctuation included.
- Seven backspaces delete the seven characters before the caret.
- `ctrl+a` selects the text of a GTK field, and the next key replaces
  it.
- A click at `(518, 305)` selects the "Documents" entry of the Files
  sidebar -- the entry at exactly that point.
- The focus check answers `pointer at 960,540 - colour #1b1b1b`.
- The emergency stop kills a script that is typing forever.

Not verified, and known to be open:

- The `ui_diff` and the `ui_wait_*` tools are exercised against the
  box, but the *thresholds* were not tuned against a screen that is
  known to be busy.
- `moonlight_stream` is not in the port: the Rust server drives the
  Moonlight client for version, list and quit, and not for the stream
  itself.
- The compositor on the box is COSMIC; nothing here was measured on
  GNOME or on X11.
