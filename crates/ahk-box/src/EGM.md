# ahk-box src

## Overview

`src` is the whole of `ahk-box`: four modules and the module root that names them.

| File | What it holds |
| --- | --- |
| `lib.rs` | The public surface of the crate, and nothing else. It declares the four modules and re-exports the names the runner needs. |
| `keyboard.rs` | A uinput keyboard of our own, for the keys `ydotool type` cannot express. |
| `pointer.rs` | An absolute uinput pointer, for driving a Wayland session exactly. |
| `native.rs` | The call into a library of the machine: the loader, the trampoline and the marshalling of the arguments. |
| `desktop.rs` | The lab box as the runtime sees it: the `ahk::desktop::Desktop` implementation that ties the keyboard, the pointer, the screen and the shell together. |

The crate does no parsing and no interpreting: a script is already parsed by the language crate `ahk`, and this crate only answers what the runtime asks of the desktop. It depends on `ahk` for the `Desktop` trait, for the key names and for the keymap of the US layout, and on `libc`, `png` and `thiserror` for the rest. `serde` and `serde_json` are declared in the manifest and used by nothing here: the report is written by the runner, which is where the serialisation lives. Nothing here depends on the runner, and the runner depends on this crate in one direction only.

## Interface

### `lib.rs` — the public surface

```rust
pub mod desktop;
pub mod keyboard;
pub mod native;
pub mod pointer;

pub use desktop::{BoxError, LabDesktop};
pub use keyboard::{ENUMERATION_WAIT, KEY_GAP, KEY_HOLD, Keyboard, KeyboardError};
pub use native::Libraries;
pub use pointer::{PRESS_HOLD, PRESS_SETTLE, Pointer, PointerError};
```

Everything a caller needs is reachable from the crate root, except the three names of `pointer` and the one of `desktop` that are not re-exported: `ahk_box::pointer::DEVICE_NAME`, `ahk_box::pointer::POSITION_ATTEMPTS`, `ahk_box::pointer::POSITION_RETRY` and `ahk_box::desktop::modifier_names()`.

### `keyboard.rs` — the keyboard of our own

| Item | Signature | Notes |
| --- | --- | --- |
| `Keyboard` | struct | The device. It is created on `open` and destroyed when the object is dropped, because closing the file destroys it. |
| `Keyboard::open` | `(codes: Vec<u16>) -> Result<Self, KeyboardError>` | Creates the device and announces it to the compositor. Off Linux: `NotLinux`. |
| `Keyboard::down` | `(&mut self, code: u16) -> Result<(), KeyboardError>` | Writes the scan code first, then the key event. |
| `Keyboard::up` | `(&mut self, code: u16) -> Result<(), KeyboardError>` | Releases the key. |
| `Keyboard::tap` | `(&mut self, code: u16) -> Result<(), KeyboardError>` | Press and release once: `KEY_HOLD` between them, `KEY_GAP` after. |
| `Keyboard::tap_with_modifiers` | `(&mut self, code: u16, modifiers: &[u16]) -> Result<(), KeyboardError>` | The modifiers go down first and up last, the way a keyboard delivers them. |
| `Keyboard::sync` | `(&mut self) -> Result<(), KeyboardError>` | The `SYN_REPORT` that closes a frame. |
| `KeyboardError` | enum | `NotLinux`, `CannotOpen(String)`, `Ioctl(u64, String)`, `Setup(String)`. |
| `KEY_HOLD` | `Duration` | Twenty milliseconds. How long a key stays down. |
| `KEY_GAP` | `Duration` | Twenty milliseconds. The gap between one key and the next. |
| `ENUMERATION_WAIT` | `Duration` | Four seconds. How long the compositor needs to notice a freshly created device. |

The keyboard announces no list of its own: the codes it declares are the ones it is opened with, and `desktop.rs` is what passes them — the 112 of the box's physical keyboard. The device is announced as `BUS_VIRTUAL` with the ydotool vendor and product ids and the name `ahk keyboard`.

### `pointer.rs` — the absolute pointer

| Item | Signature | Notes |
| --- | --- | --- |
| `Pointer` | struct | The device. It holds the file of the device, so it cannot be in a closed state. |
| `Pointer::open` | `(width: i64, height: i64) -> Result<Self, PointerError>` | The absolute axes are told the range they cover, so the screen size is needed first. Off Linux: `NotLinux`. |
| `Pointer::size` | `(&self) -> (i64, i64)` | The size of the screen this pointer reports. |
| `Pointer::last_position` | `(&self) -> (i64, i64)` | Where the runtime last put the pointer. |
| `Pointer::move_to` | `(&mut self, x: i64, y: i64) -> Result<(), PointerError>` | Clamped to the screen. |
| `Pointer::button_down` | `(&mut self, button: &str) -> Result<(), PointerError>` | "right" and "middle" are the other buttons; anything else is the left one. |
| `Pointer::button_up` | `(&mut self, button: &str) -> Result<(), PointerError>` | Releases the button. |
| `Pointer::click` | `(&mut self, x: i64, y: i64, button: &str) -> Result<(), PointerError>` | Move, settle, press, hold, release — in that order. |
| `Pointer::position` | `(&mut self) -> Result<(i64, i64), PointerError>` | Where the kernel says the pointer is, read back from the event node. |
| `PointerError` | enum | `NotLinux`, `CannotOpen(String)`, `Ioctl(u64, String)`, `Setup(String)`, `Closed`. |
| `PRESS_SETTLE` | `Duration` | Two hundred milliseconds between the motion and the press. |
| `PRESS_HOLD` | `Duration` | Forty milliseconds. How long the button stays down. |
| `DEVICE_NAME` | `&str` | "ahk absolute pointer". The event node is found by this name. |
| `POSITION_ATTEMPTS` | `usize` | Twenty tries. |
| `POSITION_RETRY` | `Duration` | A hundred milliseconds between the tries. |
| `clamp_to_screen` | private fn | The place a click at (x, y) really lands on. |
| `button_code` | private fn | The code of a mouse button. |

`PointerError::Closed` is declared but never built: the pointer holds the file of the device, so the archive's "a closed pointer refuses to move" has no state to happen in here.

The numbers of the kernel ABI live in a private `abi` module, and they are the ones that fail silently — a struct laid out in the wrong order, or an ioctl number built with `_IO` instead of `_IOW`, gives `EINVAL` from the kernel and nothing else. They are pinned by the cases at the bottom of the file, against the C definitions of `/usr/include/linux/uinput.h`:

| Constant | Value | What it is |
| --- | --- | --- |
| `UI_DEV_CREATE` | `_IO('U', 1)` | The direction and the size are both zero. |
| `UI_DEV_SETUP` | `_IOW('U', 3, 92)` | The size of the struct it carries is part of the request number. |
| `UI_ABS_SETUP` | `_IOW('U', 4, 28)` | The same, for the absolute axes. |
| `UI_SET_EVBIT` | `_IOW('U', 100, 4)` | The bit operations take an int. |
| `UI_SET_KEYBIT` | `_IOW('U', 101, 4)` | |
| `UI_SET_ABSBIT` | `_IOW('U', 103, 4)` | |
| `UI_SET_PROPBIT` | `_IOW('U', 110, 4)` | |
| `UINPUT_SETUP_SIZE` | 92 | `4*2 + 80 + 4`. |
| `UINPUT_ABS_SETUP_SIZE` | 28 | `2*2 + 6*4`. |
| `INPUT_EVENT_SIZE` | 24 | `2*8 + 2*2 + 4`. |
| `EVIOCGABS(abs)` | `_IOR('E', 0x40+abs, 24)` | Reads the current value of an absolute axis. |
| `BTN_LEFT` / `BTN_RIGHT` / `BTN_MIDDLE` | `0x110` / `0x111` / `0x112` | |
| `ABS_X` / `ABS_Y` | `0` / `1` | |
| `INPUT_PROP_DIRECT` | `1` | A capability *index*, not a bit mask: the kernel shifts the index into the device's bitmap, so it shows up as `B: PROP=2`. |

### `native.rs` — the call into a library of the machine

| Item | Signature | Notes |
| --- | --- | --- |
| `Libraries` | struct | The libraries that are loaded. A library is loaded once and kept. |
| `Libraries::new` | `() -> Self` | A desktop that has not loaded anything yet. |
| `Libraries::loaded` | `(&self) -> usize` | How many libraries are loaded. |
| `Libraries::symbol` | `(&mut self, library: &str, function: &str) -> Result<u64, String>` | The address of the function. The two failures are the ones v2.0.19 gives: "Failed to load DLL." and "Call to nonexistent function." |
| `call` | `(&mut Libraries, &mut NativeCall) -> Result<NativeResult, String>` | The call itself: the library is loaded, the function is found, the arguments go in the registers the machine wants, and the answer comes back. |
| `trampoline` | private fn | The assembly: one for Windows and one for Linux, and a stub for a machine that has neither. |
| `loader` | private mod | `LoadLibraryW` and `GetProcAddress` on Windows, `dlopen` and `dlsym` on Linux, and a stub elsewhere. |

The handle of a library is a number and not a pointer: a pointer is not
`Send`, and the desktop may have to cross a thread. An empty library is
the process itself.

The argument that is the address of a variable gets a block of the width
of its type, and the bytes the callee left in that block come back into
the argument: the argument is the way back to the variable of the script.
The answer of a text is the bytes read at the address the callee
returned, and the language decodes them.

### `desktop.rs` — the box as the runtime sees it

| Item | Signature | Notes |
| --- | --- | --- |
| `LabDesktop` | struct | Implements `ahk::runtime::Desktop`. |
| `LabDesktop::new` | `() -> Self` | A desktop that has not yet opened anything. |
| `LabDesktop::notes` | `(&self) -> &[String]` | The places where the box was not what the script assumed. |
| `LabDesktop::screen_size` | `(&mut self) -> Result<(i64, i64), BoxError>` | Read from a full-screen capture; measured once and kept. |
| `LabDesktop::ensure_session_env` | `(&mut self)` | Pins the graphical session's environment, and says what it found. |
| `BoxError` | enum | `Failed(String)`, `Keyboard(KeyboardError)`, `Pointer(PointerError)`. |
| `modifier_names` | `() -> Vec<String>` | The modifier names, for the report. |
| `announced_codes` | private fn | The 112 codes the keyboard announces. |
| `parse_colour` | private fn | `#rrggbb`, `#rgb` or `rrggbb`. |
| `read_pixel` / `search_pixel` | private fn | The colour of a pixel, and the place of a colour, in a PNG. |
| `x_display` / `wayland_session` | private fn | The X server to ask, and whether the session is Wayland. |

The `Desktop` of the crate is the sum of the ten capability traits of
`ahk::desktop`, and the box writes the ten of them: `Keyboard`
(`type_text`, `press_key`, `mouse_down`, `mouse_up`, `key_down`, `key_up`),
`Pointer` (`click`, `move_pointer`, `mouse_position`), `Screen` (`capture`,
`pixel`, `pixel_search`, `screen_size`), `Windows` (`window_active`,
`window_exists`, `active_window_title`), `Processes` (`run`), `Files` (the
file handles, and the whole-file writes), `Clipboard`, `Timers` and
`Hotkeys` and `Native` (the call into a library of the machine, which
`native.rs` answers). The keyboard and the pointer are opened on first use and kept open for the lifetime of the object: the compositor needs a moment to enumerate a new device, and it takes that moment again for every one it opens.

## Invariants

- **The uinput syscalls are behind `#[cfg(target_os = "linux")]`, per function.** On another target they are not compiled at all, and the device answers `NotLinux` — "the keyboard is a Linux uinput device; this is not Linux" — instead of pretending. The event writing itself is compiled everywhere; it simply cannot reach a device that is not there. `desktop.rs` is not behind that gate at all: it compiles on Windows too, and fails there when it is used, so a green build on another platform says nothing about the uinput code.
- **The keyboard announces exactly the 112 codes of the box's physical keyboard.** The list is the one of the `AT Translated Set 2 keyboard`, read out of `/proc/bus/input/devices` on 2026-10-25. A device that announces codes a real keyboard does not have makes libinput build a keymap it cannot use: the key events arrive and are reported with the key code `-1`, which a client cannot use. The events are in the event node the whole time, and they stop there.
- **The scan codes ride with the key events.** `down` writes `EV_MSC`/`MSC_SCAN` before the key event, the way a keyboard sends it, and the device declares the `MSC_SCAN` bit in its capabilities. It declares the relative axes too, for the same reason the working `ydotool` device does: libinput builds the keymap of a device from what the device says it can do.
- **Every frame is closed.** Each event is followed by `SYN_REPORT`, because without it the event sits in the compositor's buffer and is never delivered: the keyboard is alive, the device is enumerated, and nothing the client sends arrives.
- **A key is never held long enough to repeat.** The auto-repeat delay is half a second, the key is held for 20 ms, and the gap between two keys is 20 ms — long enough for the compositor to see the press, short enough that nothing repeats, and spaced enough that a client does not coalesce a burst of key events into a handful instead of a hundred.
- **The modifiers go down before the key and come up after it.** That order is what makes the chord reach the client: measured on the lab box on 2026-10-25, `ctrl+a` selected the text of a GTK field and the next key replaced it. The note that "a modifier never reaches the client" was measured with the `ydotool` device, whose `key 30:1` is ignored — a property of that tool, not of the box.
- **The pointer announces `INPUT_PROP_DIRECT` and absolute axes**, like the device Sunshine creates on that box, and it announces no tool bits: a tablet's tip press does not activate anything, not the compositor's decorations and not a client.
- **The keyboard and the pointer are two devices.** libinput picks one role per device, and the role with the richer key bitmap wins, so one node cannot be both. The pointer is also mandatory: there is no `ydotool` fallback anywhere in this tree, and a pointer that cannot be created fails the statement instead of quietly clicking somewhere else.
- **The pointer waits 200 ms between the motion and the press.** The compositor coalesces events into frames, and a press delivered in the same frame as the motion is applied at the pointer's *previous* position. The button stays down for 40 ms.
- **The coordinates are clamped to the screen**, so a stale or silly coordinate cannot wrap around to the far corner.
- **The pointer's position is read from the kernel, not remembered.** The read happens on the event node of the device, found by the name under `/sys/class/input/event*/device/name`, because the uinput file only answers the `UI_*` ioctls. The node is created with the mode of a device only its owner may touch, and `udev` opens it up to the `input` group a moment later, so the read waits for that: twenty tries, a hundred milliseconds apart. A failure here is a race, not a wrong answer, and the caller is told rather than given a number from somewhere else. When the kernel does not answer, the desktop falls back to the position the runtime put the pointer at and says so in a note.
- **The text is typed one key at a time through the keyboard of our own, and the layout is the US one.** A character the layout does not have fails the statement with "the keyboard of this box has no key for it"; it is not typed as something else. The shift key is what makes the capitals work, for the same reason the chord does.
- **The call into a library of the machine is made by the desktop, and the
  machine's convention is followed on each platform.** The two conventions
  of x64 do not agree: on Windows the register of an argument is the one
  of its place, and on Linux it is the one of its class. The two
  trampolines were proved before they entered the tree, in a prototype of
  their own: thirteen cases on Windows and fourteen on the box.
- **A question the box cannot answer fails; it does not answer `false`.** The session is Wayland, the COSMIC services answer `Introspect` with an empty list, and the rootless Xwayland of gamescope reports an empty client list, so `WinActive` and `WinExist` answer "unknown" and say out loud how to check the focus instead. `WinGetTitle` is the exception: with no active X11 window it answers an empty string, because there is no X11 window to ask about.
- **The screen is the focus check.** `PixelGet` and `PixelSearch` are read from the frame the compositor draws, so they work whatever window is in front, Wayland or not.

## Side Effects

- **Opens `/dev/uinput`** twice over the life of a run, once per device, with `O_NONBLOCK`. The user has to be in the `input` group; no privilege beyond that is asked for.
- **Creates two uinput devices** and writes their events: `EV_KEY`, `EV_REL`, `EV_MSC`, `EV_ABS` and `EV_SYN`, as 24-byte `input_event` structs. The timeval's second field is written in microseconds, because nanoseconds there are a thousand times too large and the compositor would see an event from the far future.
- **Reads the event node of the pointer** under `/dev/input/event*`, through `EVIOCGABS` on the two axes, and lists `/sys/class/input` to find it.
- **Sleeps**: four seconds after each device creation, 200 ms between a motion and a press, 40 ms per button, 20 ms per key and 20 ms between keys, 60 ms between the two clicks of a double click.
- **Runs the environment probe.** `ensure_session_env` reads `/run/user/<uid>`, sets `XDG_RUNTIME_DIR` to it, and sets `WAYLAND_DISPLAY` to the first `wayland-*` socket it finds — but only when `WAYLAND_DISPLAY` is not set already. Over SSH the display variables are empty and `grim` refuses to run without them.
- **Calls out to `grim`** for the screen captures: `grim <path>` for the size of the screen, and `grim -g <x,y> <w>x<h> <path>` for a region. It writes PNG files, reads the pixel data back through the `png` crate, and removes the temporary ones afterwards; the file of a `capture` stays where it was written, for the caller to read.
- **Calls out to `xprop`** for the window questions: `_NET_ACTIVE_WINDOW` and `_NET_WM_NAME`/`WM_NAME` for the title, `_NET_CLIENT_LIST` for the list of windows.
- **Runs the script's own commands** through `sh -c`, and returns the exit code, the standard output and the standard error.
- **Loads the libraries of the machine** that the script asks for, and calls into them. The library stays loaded for the life of the desktop, and it is the callee that owns the memory it writes into.
- **Collects notes** — the device that had to be waited for, the environment that was found, the position the runtime put the pointer at when the kernel would not say — and the runner adds them to the report.

## Verification

- Run: `cargo test --workspace`
- The cases inside `pointer.rs` pin the numbers of the uinput ABI against the C definitions, so a mistake in them is caught on the workstation and not on the box. The cases inside `native.rs` make real calls, against the libraries of the machine: `kernel32` on Windows and `libc.so.6` on Linux. They are the ones that say whether the shape of the call is right, and they run on both platforms. The device itself is checked on the box, by hand: a script that asks for the pointer and for a pixel touches the screen and nothing else.
