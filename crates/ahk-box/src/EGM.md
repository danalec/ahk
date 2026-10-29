# ahk-box src

## Overview

`src` is the whole of `ahk-box`: seven modules and the module root that names them.

| File | What it holds |
| --- | --- |
| `lib.rs` | The public surface of the crate, and nothing else. It declares the five modules and re-exports the names the runner needs. |
| `keyboard.rs` | A uinput keyboard of our own, for the keys `ydotool type` cannot express. |
| `pointer.rs` | An absolute uinput pointer, for driving a Wayland session exactly. |
| `native.rs` | The call into a library of the machine: the loader, the trampoline and the marshalling of the arguments. |
| `raven.rs` | The call into a library of Windows: the raven, the helper that runs inside it, and the protocol between them. |
| `files.rs` | The files of the machine: the copy, the move, the attributes, the times, the directories and the ini files. |
| `hotkeys.rs` | The watcher of the keyboard: the device that is read, and the hotkeys and the hotstrings that are registered. |
| `desktop.rs` | The lab box as the runtime sees it: the `ahk::desktop::Desktop` implementation that ties the keyboard, the pointer, the screen and the shell together. |

The crate does no parsing and no interpreting: a script is already parsed by the language crate `ahk`, and this crate only answers what the runtime asks of the desktop. It depends on `ahk` for the `Desktop` trait, for the key names and for the keymap of the US layout, and on `libc`, `png` and `thiserror` for the rest. `serde` and `serde_json` are declared in the manifest and used by nothing here: the report is written by the runner, which is where the serialisation lives. Nothing here depends on the runner, and the runner depends on this crate in one direction only.

## Interface

### `lib.rs` — the public surface

```rust
pub mod desktop;
pub mod files;
pub mod hotkeys;
pub mod keyboard;
pub mod native;
pub mod pointer;
pub mod raven;

pub use desktop::{BoxError, LabDesktop};
pub use keyboard::{ENUMERATION_WAIT, KEY_GAP, KEY_HOLD, Keyboard, KeyboardError};
pub use native::Libraries;
pub use pointer::{PRESS_HOLD, PRESS_SETTLE, Pointer, PointerError};
```

Everything a caller needs is reachable from the crate root, except the three names of `pointer` and the one of `desktop` that are not re-exported: `ahk_box::pointer::DEVICE_NAME`, `ahk_box::pointer::POSITION_ATTEMPTS`, `ahk_box::pointer::POSITION_RETRY` and `ahk_box::desktop::modifier_names()`.
The names of the raven are reached through its module: `ahk_box::raven::Raven` and `ahk_box::raven::Paths`.

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

### `raven.rs` — the Windows of the box

The box has no Windows of its own, and the raven is one: it is a program
that emulates a Windows process on Linux, and it runs the helper that
does the call. The helper is a Windows program of our own, and it lives
in `crates/ahk-box/helper/`. The raven is not a part of this crate: it
is a program of the box, built from the upstream, and the environment
names it.

| Item | Signature | Notes |
| --- | --- | --- |
| `Raven` | struct | The raven: the process, and the pipes to it. It is started once and kept. |
| `Raven::start` | `(&Path, &Path) -> Result<Raven, String>` | Starts the raven with the guest as its program, and hands it the pipes of the host. |
| `Raven::call` | `(&mut self, &mut NativeCall) -> Result<NativeResult, String>` | One call: the request goes down the pipe, the answer comes up it. |
| `Guests` | struct | The two guests of the raven: the one of the sixty-four bits and the one of the thirty-two. |
| `Guests::new` | `(Paths) -> Guests` | The guests of a box, with the paths of the programs. |
| `Guests::call` | `(&mut self, &mut NativeCall) -> Result<NativeResult, String>` | The call of the sixty-four bits, and the call of the thirty-two bits when the first does not have the library. |
| `Guests::take_notes` | `(&mut self) -> Vec<String>` | The notes of the guests: the ravens that were started. |
| `Paths` | struct | The three programs: the raven, the guest of the sixty-four bits and the guest of the thirty-two. |
| `Paths::from_environment` | `() -> Option<Paths>` | `AHK_RAVEN` is the raven, `AHK_RAVEN_HELPER` is the guest of the sixty-four bits and `AHK_RAVEN_HELPER32` is the guest of the thirty-two. `None` when the box has not the raven and the guest of the sixty-four bits. |
| `is_windows` | `(&str) -> bool` | Whether the name of a library is a library of Windows. |

The protocol between the two is one line per call, tab separated, and
every value in it is hexadecimal: the library, the function, the type of
the answer, the number of the arguments, and then the type and the
value of every argument. The answer is `ok` and the value and the
blocks that came back, or `error` and the reason. The type is the name
the tree spells it with, and a star at the end says that the argument
is the address of a value.

The reasons are the messages of v2.0.19 — "Failed to load DLL.",
"Call to nonexistent function.", "Invalid arg type." and "Invalid
return type." — and the box does not answer with a reason of its own
where v2 has one.

### `files.rs` — the files of the machine

The operations of v2 that are not the file handles: the copy, the move,
the attributes, the times, the directories and the ini files.

| Item | Signature | Notes |
| --- | --- | --- |
| `impl Files for LabDesktop` | — | The `Files` capability of the runtime, as the box answers it. The file handles stay in `desktop.rs`; the two cannot be in the same file, because Rust has one `impl` of a trait for a type. |
| `copy_file` / `move_file` | `(from, to, overwrite) -> Result<String, String>` | The copy writes the bytes; the move renames when it can. |
| `copy_dir` / `move_dir` | `(from, to, overwrite) -> Result<String, String>` | The copy takes the *contents* of the folder: the folder that is there is the one that holds the files. |
| `file_attributes` / `set_file_attributes` | `(path) / (path, attributes)` | The letters of v2: `R` for the read-only, `A` for the archive, `D` for the folder. The box has no bit of its own for the archive. |
| `file_times` / `set_file_times` | `(path) / (path, times)` | The three times of a file: the change, the access and the change of the metadata. |
| `ini_read` / `ini_write` / `ini_delete` | `(file, section, key)` and so on | An ini of Windows: `[section]`, `key=value`, and `;` for a comment. |
| `file_exists` / `delete_file` / `create_dir` / `dir_exists` | `(path)` | The four that answer whether something is there, and that take it away or make it. |
| `letters_of` | private fn | The attributes of a file, in the order of v2. |

The wildcards and the recursion are not carried: `FileCopy "a*.txt", "b"`
and the folders that hold the folders are not in the tree, and the names
that would ask for them say so.

### `hotkeys.rs` — the watcher of the keyboard

The box watches the keyboard that *reads*: the keyboard of the box writes
(the uinput), and the hotkeys need the events of the physical one.

| Item | Signature | Notes |
| --- | --- | --- |
| `Watcher` | struct | The watcher: the device, the keys that are down, the hotkeys and the hotstrings that are registered, and the text that was typed. |
| `Watcher::open` | `() -> Result<Watcher, String>` | The watcher without a device: the device is opened when the first events are wanted. |
| `register` / `unregister` | `(name)` | The hotkeys and the hotstrings that the script registered. |
| `fired` | `() -> Result<Option<String>, String>` | The hotkey that fired, and the reason when the device cannot be read. |
| `key_is_down` / `keys_down` | `(code)` | The keys that are down right now. |
| `device_path` | private fn | The device the events are read from: the path of the variable, or the name of the device. |
| `device_named` | private fn | The event node of a device, by its name in `/proc/bus/input/devices`. |
| `DEVICE_VARIABLE` | `&str` | `AHK_HOTKEY_DEVICE`: the device to watch, by path or by name. |

The virtual devices are skipped when the device is found by itself: a
hotkey that fires on what the script writes is a hotkey in a loop. The
variable is the way around that, and it is how the hotkeys are proved on
the box without a person at the keyboard.

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
(`type_text`, `press_key`, `mouse_down`, `mouse_up`, `key_down`, `key_up`,
`key_is_down`, `keys_down`), `Pointer` (`click`, `move_pointer`,
`mouse_position`, `set_mouse_speed`, `set_mouse_delay`), `Screen` (`capture`,
`pixel`, `pixel_search`, `screen_size`), `Windows` (`window_active`,
`window_exists`, `active_window_title`), `Processes` (`run`), `Files` (the
file handles, the whole-file writes, and the operations of `files.rs`),
`Clipboard`, `Timers`, `Hotkeys` (`register_hotkey`, `unregister_hotkey`,
`hotkeys`, `fired_hotkey`) and `Native` (the call into a library of the
machine, which `native.rs` answers). The keyboard and the pointer are opened on first use and kept open for the lifetime of the object: the compositor needs a moment to enumerate a new device, and it takes that moment again for every one it opens.

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
- **The hotkeys watch the keyboard that reads.** The keyboard of the box writes; the watcher reads the event node of the physical keyboard. The virtual devices are skipped when the device is found by itself, because a hotkey that fires on what the script writes is a hotkey in a loop. The variable `AHK_HOTKEY_DEVICE` names the device to read, by path or by name, and it is how the hotkeys are proved on the box without a person at the keyboard.
- **The device of the watcher is opened when the first events are wanted.** The keyboard of the box is made when the script types for the first time, and a watcher that wanted a device that is not there yet would say that the hotkeys cannot be watched. The device that cannot be read is a failure with the reason, and not a watcher that never fires.

- **A library of this machine is called here, and a library of Windows is
  called by the raven.** The name of the library decides: a slash or a
  `.so` in it is a library of this machine, and every other name is a
  library of Windows. The box has no Windows of its own, and the raven
  is the Windows of the box. The rule is a rule of the box: on Windows
  the machine is the Windows, and every library is called here.
- **The raven is named by the environment.** `AHK_RAVEN` is its program,
  `AHK_RAVEN_HELPER` is the guest of the sixty-four bits and
  `AHK_RAVEN_HELPER32` is the guest of the thirty-two bits. A box that
  has not the first two says so — "this box has no raven: the libraries
  of Windows cannot be called." — and it does not answer with a wrong
  answer. The guest of the thirty-two bits is not needed: a box that
  has not it calls the libraries of the sixty-four bits and no more.
- **There are two guests, and the sixty-four bits is asked first.** The
  guest of the sixty-four bits does not have a library of thirty-two
  bits — the raven skips a library of the other width — and it answers
  "Failed to load DLL." The box then asks the guest of the thirty-two
  bits. A function that is not in the library is not looked for in the
  other guest: the library was there and the function was not.
- **The raven is a process of its own, and it is started once.** It
  starts with the first library of Windows that a script asks for, and
  it lives until the desktop is dropped: a program of its own is not
  cheap to start. Its standard input and its standard output are the
  pipes of the host, and its standard error goes nowhere — the reason of
  a failure comes back in the protocol. It stops with the runner, even
  with a runner that is killed: the pipe of its input ends with the
  runner, and the guest reads the end of it and stops.
- **The call is the one of the Microsoft x64.** The guest is a real
  Linux x86-64 process under ptrace, and the ABI of it is genuine: it
  is the ABI that the tree already uses on Windows. A call that takes a
  variable number of arguments cannot be made, because the callee saves
  the float registers in a space whose size only the callee knows. The
  call of the thirty-two bits is the one of the Microsoft x86, and it
  has the limit of the others: the number of the float registers would
  have to ride in AL, and the helper does not write it there.
- **The stack of the call is kept in `RBX`.** `R11` is volatile in the
  Windows x64 ABI: the callee takes it, and the stack that the
  trampoline had prepared is lost — the guest then reads outside its
  memory, and the raven says "native read exceeds readable guest
  memory". `RBX` is a register the callee gives back. The trampoline of
  `native.rs` says this already, and the helper paid for it again on
  2026-10-28.

## Side Effects

- **Opens `/dev/uinput`** twice over the life of a run, once per device, with `O_NONBLOCK`. The user has to be in the `input` group; no privilege beyond that is asked for.
- **Creates two uinput devices** and writes their events: `EV_KEY`, `EV_REL`, `EV_MSC`, `EV_ABS` and `EV_SYN`, as 24-byte `input_event` structs. The timeval's second field is written in microseconds, because nanoseconds there are a thousand times too large and the compositor would see an event from the far future.
- **Reads the event node of the pointer** under `/dev/input/event*`, through `EVIOCGABS` on the two axes, and lists `/sys/class/input` to find it.
- **Reads the event node of the keyboard** for the hotkeys, and lists `/proc/bus/input/devices` to find it. The node is opened with `O_NONBLOCK`: the watcher must not wait for an event that may never come.
- **Sleeps**: four seconds after each device creation, 200 ms between a motion and a press, 40 ms per button, 20 ms per key and 20 ms between keys, 60 ms between the two clicks of a double click.
- **Runs the environment probe.** `ensure_session_env` reads `/run/user/<uid>`, sets `XDG_RUNTIME_DIR` to it, and sets `WAYLAND_DISPLAY` to the first `wayland-*` socket it finds — but only when `WAYLAND_DISPLAY` is not set already. Over SSH the display variables are empty and `grim` refuses to run without them.
- **Calls out to `grim`** for the screen captures: `grim <path>` for the size of the screen, and `grim -g <x,y> <w>x<h> <path>` for a region. It writes PNG files, reads the pixel data back through the `png` crate, and removes the temporary ones afterwards; the file of a `capture` stays where it was written, for the caller to read.
- **Calls out to `xprop`** for the window questions: `_NET_ACTIVE_WINDOW` and `_NET_WM_NAME`/`WM_NAME` for the title, `_NET_CLIENT_LIST` for the list of windows.
- **Runs the script's own commands** through `sh -c`, and returns the exit code, the standard output and the standard error.
- **Loads the libraries of the machine** that the script asks for, and calls into them. The library stays loaded for the life of the desktop, and it is the callee that owns the memory it writes into.
- **Collects notes** — the device that had to be waited for, the environment that was found, the position the runtime put the pointer at when the kernel would not say — and the runner adds them to the report.

- **Starts the raven**, a process of its own with the helper as its
  program, and talks to it over a pair of pipes. Its standard error goes
  nowhere, and the timeout of the guest is a day. The guest of the
  thirty-two bits is a second process of its own, and it starts only
  when the library is not in the folder of the first.

## Verification

- Run: `cargo test --workspace`
- The cases inside `pointer.rs` pin the numbers of the uinput ABI against the C definitions, so a mistake in them is caught on the workstation and not on the box. The cases inside `native.rs` make real calls, against the libraries of the machine: `kernel32` on Windows and `libc.so.6` on Linux. They are the ones that say whether the shape of the call is right, and they run on both platforms. The device itself is checked on the box, by hand: a script that asks for the pointer and for a pixel touches the screen and nothing else.

- The raven needs the box and the two programs of the environment, so it
  is not in the suite: what it answers is checked on the box, by hand.
  Measured there on 2026-10-28, the kernel32 of the raven answered
  `GetTickCount`, `GetCurrentProcessId`, `lstrlenA`,
  `GetSystemTimeAsFileTime`, `GetLastError`, `GetProcAddress`,
  `LoadLibraryW`, `GetModuleHandleW`, `GetStdHandle`, `ReadFile` and
  `WriteFile`. `GetProcessHeap` is the exception: it is dispatched only
  when the component `raven-rtl.dll` is built, and that component needs
  a mingw cross compiler.
