# ahk-box

## Overview

`ahk-box` is the part of the runtime that touches the machine. At this level there is no source file of its own: the whole crate is in `src/`, which is its single component.

The dependency runs one way. The language crate `ahk` does no I/O at all and runs a script against whatever implements `ahk::desktop::Desktop` -- the sum of the eleven capability traits; this crate is that implementation for the lab box, and the runner `ahk-cli` is what builds it. Nothing here reaches back into the language's internals, and nothing here depends on the runner.

Everything that talks to the machine is Linux. The keyboard, the pointer, the screen reads and the windows of the compositor are the four primitives, and all four were measured on the lab box (CachyOS, COSMIC Wayland): the three first on 2026-10-25, and the windows on 2026-10-29.

## Subdirectories & Boundaries

- `src`: [the component package contract](./src/EGM.md) — the crate's whole public surface: the uinput keyboard, the absolute pointer, and the `LabDesktop` that ties them to the screen and the shell.
- `protocols`: the XML of the Wayland protocols the box speaks. They are vendored, not generated from a crate: `cosmic-toplevel-info-unstable-v1.xml` and `cosmic-workspace-unstable-v1.xml` come from `pop-os/cosmic-protocols` and carry its MIT notice. The crate itself is GPL-3.0 and is not a dependency of this tree — the behaviour is ported and the code is not copied. `wayland-scanner` turns the XML into the bindings, at build time.

## Interface

The names a caller of the crate can name. All of them are re-exported by `src/lib.rs`, except the last paragraph.

| Symbol | Module | What it is |
| --- | --- | --- |
| `LabDesktop` | `desktop` | The `ahk::runtime::Desktop` implementation for the lab box. |
| `BoxError` | `desktop` | The error of the desktop: `Failed`, `Keyboard`, `Pointer`. |
| `Keyboard` | `keyboard` | The uinput keyboard of our own. |
| `KeyboardError` | `keyboard` | `NotLinux`, `CannotOpen`, `Ioctl`, `Setup`. |
| `Pointer` | `pointer` | The absolute uinput pointer. |
| `PointerError` | `pointer` | `NotLinux`, `CannotOpen`, `Ioctl`, `Setup`, `Closed`. |
| `ENUMERATION_WAIT` | `keyboard` | Four seconds. |
| `KEY_GAP` | `keyboard` | Twenty milliseconds. |
| `KEY_HOLD` | `keyboard` | Twenty milliseconds. |
| `PRESS_SETTLE` | `pointer` | Two hundred milliseconds. |
| `PRESS_HOLD` | `pointer` | Forty milliseconds. |

Reachable, but not re-exported at the crate root: `ahk_box::pointer::DEVICE_NAME`, `ahk_box::pointer::POSITION_ATTEMPTS`, `ahk_box::pointer::POSITION_RETRY` and `ahk_box::desktop::modifier_names()`.

## Invariants

- **The uinput syscalls are behind `#[cfg(target_os = "linux")]`, per function.** On another target they are not compiled at all, and the device answers `NotLinux` — "the keyboard is a Linux uinput device; this is not Linux". `desktop.rs` is not behind that gate: it compiles on Windows too, and fails there when it is used. A green build on another platform says nothing about the uinput code; the numbers are pinned by the cases in `src/pointer.rs`, and the device itself is checked on the box.
- **The keyboard announces exactly the 112 codes of the box's physical keyboard**, the `AT Translated Set 2 keyboard` read out of `/proc/bus/input/devices` on 2026-10-25. A device that announces codes a real keyboard does not have makes libinput build a keymap it cannot use: the key events arrive and are reported with the key code `-1`, which a client cannot use.
- **The scan codes ride with the key events.** The device emits `EV_MSC`/`MSC_SCAN` before each key event and declares the `MSC_SCAN` bit in its capabilities.
- **The pointer announces `INPUT_PROP_DIRECT` and absolute axes**, like the device Sunshine creates on that box. ydotool's device is a relative mouse with a full keyboard key bitmap and no `INPUT_PROP_POINTER`, so libinput classifies it as a keyboard and ignores the relative axes: the click arrives and moves nothing.
- **The keyboard and the pointer are two devices.** libinput picks one role per device, and the role with the richer key bitmap wins, so one node cannot be both.
- **A new device waits four seconds** (`ENUMERATION_WAIT`) before the compositor adopts it, and the keyboard and the pointer both wait. A click that arrives before the adoption lands nowhere — and the report still says the click happened, because the device accepted the event even though the compositor ignored it.
- **The pointer waits 200 ms between the motion and the press** (`PRESS_SETTLE`). A press delivered in the same frame as the motion is applied at the position the pointer had before the motion; measured on 2026-10-25, a 20 ms gap put the click hundreds of pixels away from where it was aimed.
- **The pointer is mandatory.** There is no `ydotool` fallback anywhere in this tree: the archive catches a pointer that cannot be created, notes it and falls back to `ydotool mousemove --absolute`, and the Rust propagates the error and the click fails.
- **A key is held for 20 ms and a button for 40 ms** — long enough for the compositor to see the press, short enough that the auto-repeat never starts — **and the gap between two keys is 20 ms**, because a client coalesces a burst of key events that arrive back to back into a handful instead of a hundred.
- **The text is typed one key at a time through the keyboard of our own, and the layout is the US one.** A character the layout does not have fails the statement with "the keyboard of this box has no key for it"; it is not typed as something else.
- **A question the box cannot answer fails; it does not answer `false`.** The windows of a Wayland session come from the compositor: `ext_foreign_toplevel_list_v1` lists them and `zcosmic_toplevel_info_v1` says which one has the focus. A compositor that publishes neither fails the question with the reason. A window that is *not there* is not a failure: `WinExist` answers `false` and `WinActive` answers `false`. `WinGetTitle` with no window in front answers an empty string, because there is no window to ask about.
- **The title of a window is matched the way v2 matches it.** The default mode of v2 is the mode 2: the title of the window has to *contain* the text that was asked for, and the case matters.

## Side Effects

- **Opens `/dev/uinput`** and creates two devices on it, the keyboard and the pointer. The user has to be in the `input` group; no privilege beyond that is asked for.
- **Writes input events** to the uinput file, and **reads the event node** of the pointer (`/dev/input/eventN`) through `EVIOCGABS` to learn where the pointer really is, rather than trusting what the process believes it did.
- **Runs the environment probe**: reads `/run/user/<uid>`, and sets `XDG_RUNTIME_DIR` and `WAYLAND_DISPLAY` when `WAYLAND_DISPLAY` is not set yet. Over SSH the display variables are empty and `grim` refuses to run without them.
- **Calls out to `grim`** for the screen captures, writes PNG files (the temporary ones under `/tmp` and in the working directory), reads the pixel data back and removes the temporary files afterwards.
- **Connects to the compositor**: the socket of the Wayland display, opened when the script asks the first question about the windows, and kept for the rest of the run.
- **Calls out to `xprop`** for the window questions of an X11 session, and runs the script's own commands through `sh -c`.
- **Sleeps**: four seconds after each device creation, 200 ms between a motion and a press, 40 ms per button, 20 ms per key and 20 ms between keys, 60 ms between the two clicks of a double click, and five milliseconds between the attempts to hear the state of a window.
- **Collects notes** — the places where the box was not what the script assumed — and the runner adds them to the report after the runtime's own notes.
- **Writes the files the script asks for.** `FileAppend` and the `File` class write through the desktop, and the path is the one the script wrote: a relative path is in the working directory of the run.

## Verification

- Run: `cargo test --workspace`
- The numbers of the uinput ABI are pinned by the cases inside `src/pointer.rs`, so a mistake in them is caught on the workstation and not on the box. The device itself is checked on the box, by hand.
