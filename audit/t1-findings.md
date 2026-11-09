# T1 — AHK_X11: ten verified facts, and ten things Wayland makes hard

Scope: `C:\Users\danalec\Documents\src\AHK_X11` (GPL-2.0, read-only). Companion documents: `ahk-x11-feature-inventory.tsv` (260 rows, evidence per row), `ahk-x11-subsystems.md`, `ahk-x11-claims-vs-reality.md`.

**Verification caveat, applies to every line below:** there is **no Crystal compiler** on this machine or on the Linux box, so the project was never built or run. "Verified" here means *verified by reading the source and cross-checking the project's claims* — not observed at runtime. Two findings are marked **[code-evident]** because they are defects inferred from the code path, not from a failing run.

---

## 1. Top 10 verified facts an engineering lead must know

**1. It is a real interpreter, not a wrapper — and the command name *is* the class name.**
`Build::Parser` maps each line's first word to a `Cmd::Base` subclass by name (`src/build/parser.cr:10-15, 61-81`), where the name is derived from the class with a macro (`src/cmd/base.cr:4-6`). There are **166 command source files, 162 of which declare a `Cmd::Base` subclass**; the other four are helpers. Adding a command = adding a class with `min_args`/`max_args`/`run`. Consequence: the dispatch table is compile-time and complete — you can enumerate the full surface by listing subclasses.

**2. The syntax is legacy AHK v1.0.24 *only* — there is no expression parser.**
Assignments are commands (`x = 1` → `Cmd::Math::SetEnv`, `x += 1` → `EnvAdd`, `x := …` → `EnvMath`, which shells out to **awk**, `src/cmd/math/env-math.cr:15`). There are no functions, no objects, no `% v` expressions, no arrays. The README publishes a migration table for this (`README.md:298-394`). Any port that assumes v1.1 expression syntax will not parse.

**3. Everything graphical funnels through exactly three native stacks plus raw Xlib.**
libxdo (`display.x_do`: windows, keyboard, mouse), GTK3 (`display.gtk`: GUI, clipboard, dialogs, notifications), AT-SPI over D-Bus (`display.at_spi`: controls, window text), and `x11-cr` directly for grabs and `_NET_*` messages (`src/run/display.cr:12-29`; `src/run/display/x11.cr:1-2`). There is no abstraction over these; commands call `display.x_do.*` etc. directly from ~60 call sites.

**4. The display-adapter abstraction does not isolate X11.**
`Run::DisplayAdapter` has only six members — init, run, grab/ungrab hotkey, block/unblock input (`src/run/display/display-adapter.cr:1-11`) — and **one** implementation, `Run::X11` (`src/run/display.cr:22`). `Run::Gtk` and `Run::AtSpi` are *not* adapters, and commands reach through the concrete X11 adapter with `.as(Run::X11)` and `display.adapter.display.*` (e.g. `src/run/thread.cr:246`, `src/cmd/x11/window/win-set.cr:9`). A non-X11 port cannot be dropped in behind this interface; it is a 60-site refactor.

**5. Hotkeys are X11 passive grabs + the XTEST Record extension, not a keyboard hook.**
Modifier variants (CapsLock/NumLock/ScrollLock/AltGr) are enumerated and each is `XGrabKey`/`XGrabButton`'d (`src/run/display/hotkey.cr:24-34, 60-78`; `src/run/display/x11.cr:382-408`), and the grab set is **ungrab-and-regrab'd every time the active window changes** (`x11.cr:296-311`) unless `#XGrabRoot` is set (`x11.cr:364-372`). Key events arrive via `Xtst::RecordExtension` (`x11.cr:174-184, 325-341`). This is why the README says hotkeys do not work on Wayland (`README.md:13`).

**6. Threads are cooperative pseudo-threads with one current thread, and blocking commands keep running in the background.**
"Threads never really run in parallel: there's always one current thread" (`src/run/thread.cr:40-43`). The clock fiber services the highest-priority thread one command at a time (`src/run/runner.cr:161-190`); a `Sleep` or a modal dialog finishes in its own fiber while another thread takes the lead. Per-thread state = `last_found_window`, `msgbox_response`, five `CoordMode`s, key/mouse delays, `ErrorLevel` (`src/run/thread.cr:13-26, 75-77`). **There is no `OnExit` and no exit-handler dispatch** — only a Crystal `at_exit` that kills SoundPlay and runs GC (`src/run/runner.cr:113-119`).

**7. `DllCall` is real FFI via the Crystal runtime, and it explains the whole variable model.**
`Crystal::Loader` (dlopen) + `Crystal::FFI::CallInterface` (libffi) with a hand-written 12-type list (`src/cmd/misc/dll-call.cr:10-69`; `src/util/ahk-bin-var.cr:3-19`). All user variables are `Bytes` holding a serialized string (`src/run/runner.cr:44-48`), so typed arguments are re-serialized into fresh writable buffers and results serialized back to text (`dll-call.cr:20-75`). `VarSetCapacity`/`NumGet`/`NumPut` exist precisely to make that usable. The loader also needs a compiler-ish environment (`src/hacks.cr:64-78`).

**8. "Compilation" is append-to-self, and portability is entirely an AppImage property.**
`Compiler#compile` copies the running binary, appends the script text, the trailer `"AHK_SOUR"` and the original binary size as an `Int64`; startup seeks to `-16`, checks the trailer, and reads the script back (`src/compiler.cr:13-34`; `src/ahk_x11.cr:69-71`). The code warns that outside an AppImage the produced binary "will not be very portable" (`compiler.cr:14-16`). There is no bytecode, no minification, no obfuscation.

**9. The README's parity numbers are wrong at the denominator, and two DONE items are broken.**
The four lists contain **127 + 15 + 11 + 70 = 223** items, not 226; the generated list has 213 entries; the union of distinct names is 223–228. `226` is reproducible from nothing (full accounting in `ahk-x11-claims-vs-reality.md` §1). Independently: **`FileGetSize`** declares `sets_error_level` but returns `Bytes` on success, so `Thread` raises *"Result should be String for ErrorLevel command??"* (`src/cmd/file/file-get-size.cr:20` + `src/run/thread.cr:176-178`; its test is disabled at `tests.ahk:318`) — **[code-evident]**; and **`if var contains`** is inverted (`src/cmd/control-flow/if-contains.cr:9`) — **[code-evident]**.

**10. The test suite is honest about its count and shallow in its coverage.**
`N_TESTS = 70` (`tests.ahk:6`) is exact: 51 `gosub assert` sites, 4 of them inside helpers invoked 23 times → 70 at runtime. It covers the GTK/Gui happy path (windows, controls, send, hotkeys ×8, hotstrings ×5, clipboard, pixels, Input, remap). It asserts **nothing** for dialogs, loops, flow control, directives, most string/math/file commands, sound, `DllCall`, window verbs beyond the tested set, or any error path.

**Bonus fact (licensing).** GPL-2.0. Behaviour may be studied and re-implemented; code may not be copied into an Apache-2.0 tree (`LICENSE`; `README.md:446-448`).

---

## 2. Top 10 things AHK_X11 does that are hard to reproduce on Wayland

Each item names the X11 mechanism AHK_X11 relies on, then the platform reason it does not carry over. The X11 side is cited in AHK_X11; the Wayland side is a platform statement, not something this repository demonstrates.

**1. Global hotkey grabbing.**
AHK_X11: `XGrabKey`/`XGrabButton` per modifier variant, re-grabbed on active-window change (`src/run/display/x11.cr:382-408, 296-311`).
Wayland: no client may grab keys globally. The only sanctioned paths are a compositor-specific IPC/protocol (e.g. a global-shortcuts portal), or reading the evdev/`libinput` stream and synthesising the effect yourself — with the side effect that the application also receives the key unless you intercept at a virtual device.

**2. Raw key-event capture for hotstrings, `Input`, `KeyWait`, `GetKeyState`.**
AHK_X11: the XTEST **Record** extension streams every key/button event to the process (`src/run/display/x11.cr:174-184, 325-341`), feeding the 30-char hotstring buffer (`src/run/display/hotstrings.cr:16-19`) and the pressed-key list (`src/run/display/pressed-keys.cr:9-27`).
Wayland: XRecord does not exist; a client cannot observe other clients' input. Equivalents are (a) a uinput device you own *plus* an evdev reader of the physical keyboard, or (b) a compositor/portal hook. Either way the hotstring engine's "keysym stream" assumption has to be rebuilt from `libinput`/evdev key codes and the active keymap.

**3. `BlockInput`.**
AHK_X11: `XGrabKeyboard` + `XGrabPointer` (`src/cmd/x11/block-input.cr:7-10`; `src/run/display/x11.cr:433-440`).
Wayland: there is no global keyboard/pointer grab; a client can only lock its own surfaces. Blocking user input needs a compositor facility (a "lock input" extension) or an `EVIOCGRAB`-style exclusive grab of the physical device — a privilege/root-level operation.

**4. `Send` / `SendRaw` / `ControlSend` — synthetic input into other applications.**
AHK_X11: libxdo, i.e. **XTEST** (`src/cmd/x11/keyboard/send.cr:74`; `send-raw.cr:25`; `control-send.cr:12`).
Wayland: XTEST reaches only Xwayland clients that opt into it and never native Wayland clients. Injection must go through a virtual-keyboard/uinput device (which is exactly the path our own tree documents in `AGENTS.md` §3–§5) or the compositor's virtual-keyboard protocol; both change the timing model (device adoption, key coalescing, held-key state) that `SetKeyDelay` and the `{blind}`/self-key workarounds in `send.cr:60-73` are calibrated against.

**5. Window enumeration, title/class, geometry and activation.**
AHK_X11: libxdo search over the X server's window tree plus EWMH `_NET_ACTIVE_WINDOW` (`src/cmd/x11/window/win-util.cr:28-42`; `src/run/display/x11.cr:187-204`), with `WinMove`/`WinRestore` mutating EWMH state (`win-move.cr:16-17`, `win-restore.cr:8-9`).
Wayland: no global window list and no client-controlled window geometry. The portable answer is a compositor protocol that publishes toplevels (title, app id, stable handle) — `ext_foreign_toplevel_list_v1` — plus a compositor extension for focus state; activation/move/restore generally require compositor cooperation or are simply not allowed.

**6. `WinSetTitle`, `WinHide`, `WinSet` transparency/always-on-top.**
AHK_X11: writes `WM_NAME` (`win-set-title.cr:17`), `unmap!`/`map!` (`win-hide.cr:9`, `win-show.cr:9`), EWMH state and `_NET_WM_WINDOW_OPACITY` (`win-set.cr:18-32`).
Wayland: window properties belong to the client that owns the surface; another process cannot rename, hide or restyle it. There is no `_NET_WM_*` equivalent; opacity/keep-above must be offered by the compositor protocol for that surface.

**7. `WinMinimizeAll` / `WinMinimizeAllUndo`.**
AHK_X11: `_NET_SHOWING_DESKTOP` ClientMessage to the root window (`src/cmd/x11/window/win-minimize-all.cr:7`; `src/run/display/x11.cr:442-453`).
Wayland: no root window and no global "show desktop" message; only a compositor-specific action (D-Bus/portal) can do it.

**8. `PixelGetColor` / `PixelSearch`.**
AHK_X11: **GDK** capture of the root window, `Gdk.pixbuf_get_from_window` (`src/cmd/misc/pixel-get-color.cr:15`; `src/cmd/misc/pixel-search.cr:27`) — not a direct XGetImage, but the same "read the screen" model.
Wayland: clients cannot read the framebuffer. The replacement is a screencopy protocol (e.g. `wlr-screencopy`/`ext-image-copy-capture`) or the portal screenshot API (`grim`, `xdg-desktop-portal`), both asynchronous, permission-gated and rate-limited — the synchronous 1-pixel read becomes a round trip.

**9. Absolute pointer position and warping (`MouseGetPos`, `MouseMove`, `MouseClick`, `MouseClickDrag`).**
AHK_X11: libxdo `mouse_location` + `move_mouse`/`mouse_down`/`mouse_up` (`src/cmd/x11/mouse/mouse-get-pos.cr:6`; `mouse-move.cr:21-31`; `mouse-click.cr:39-47`).
Wayland: a client cannot query the global pointer position or warp it; native Wayland clients see pointer events only inside their own surfaces. An absolute pointer must be a virtual device you own (uinput, absolute axes) and the position must be tracked yourself — and the "pointer already there" case still needs a motion event.

**10. Control-level automation through AT-SPI, keyed by X11 window id.**
AHK_X11: finds an app's accessible tree by **PID**, then a top-level frame **by title, falling back to size**, and caches it by **X window id** (`src/run/display/at-spi.cr:25-77`; cache keyed at `:28-29, 72-74`; thread cache in `src/run/thread.cr:29-32`), forging ClassNNs from `role_childindex…` (`at-spi.cr:416-440`).
Wayland: AT-SPI itself works over D-Bus and is compositor-independent — this is the *one* subsystem that ports well — but the **X11 window id** that ties an accessible frame to a window is gone, so the correlation key and the "found by XID" shortcuts must be re-derived from toplevel handles or app ids, and `WinText`/`Control*`/`MouseGetPos`'s control output lose their anchor.

**Cross-cutting note for planning.** Items 1–4 are *input* and 5–9 are *observation/control of other clients*; Wayland closes both, for the same architectural reason (clients own their surfaces and their input). An AHK_X11-compatible engine on Wayland therefore needs a privileged helper on the uinput/evdev side **plus** compositor-protocol support for toplevels and screencopy — i.e. two different capabilities, not one. Item 10 is the only piece that can be reused as-is.
