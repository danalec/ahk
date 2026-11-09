# AHK_X11 — subsystem audit (source-read verification)

Target: `C:\Users\danalec\Documents\src\AHK_X11` — AutoHotkey v1.0.24 re-implementation for Linux/X11 in Crystal, by Philip Waritschlager, GPL-2.0 (read-only for us: behaviour may be described, code must not be copied into an Apache-2.0 tree).

## Verification method — read this first

**No Crystal compiler exists on this machine or on the Linux box, so the project was never built or executed.** Every statement below comes from (a) reading the Crystal source, and (b) cross-checking the project's own claims (README, `docs/index.html` legend, `docs/list of commands.txt`, `tests.ahk`) against that source.

What that buys:

* "implemented" = a `Cmd::Base` subclass with a real `run(thread, args)` body exists, is reachable through the parser, and calls the libraries it claims to call. Exact line references are given.
* "works" is **not** claimed. A code path can be present and still be wrong or unreachable; two such cases are called out explicitly (`FileGetSize`, `If var contains`).
* Nothing here was observed at runtime. There is no X server, no GTK, no AT-SPI and no libxdo on the machine, so all mechanism statements are about *which API the code calls*, not about whether that call succeeds on a particular desktop.

Where the source itself is the only evidence of a defect, the finding is marked **[code-evident, not runtime-verified]**.

---

## 0. Shape of the whole program

The program is a classic three-stage scripting engine, all in memory:

1. **Parse** — `Build::Parser` (`src/build/parser.cr:28-256`) turns each script line into an instance of a `Cmd::Base` subclass. The command name is *derived from the class name*: `Cmd::File::FileCopy` → `"filecopy"` (`src/cmd/base.cr:4-6`), and the class table is built by the macro `{{ @type.all_subclasses }}` (`src/cmd/base.cr:37-39`, used at `src/build/parser.cr:10-15`). There are **166 `.cr` files under `src/cmd/`, 162 of which declare a `Cmd::Base` subclass** (the other four are helpers: `base.cr`, `no-run.cr`, `file/file-util.cr`, `x11/window/win-util.cr`).
2. **Link** — `Build::Linker` (`src/build/linker.cr:151-247`) eats `if`/`else`/`loop`/`{`/`}` and rewrites them into a **linked list** with `next`, `je`, `jne` pointers (`src/cmd/base.cr:33-35`). After linking there is no nesting state left: "all if/else/loop commands are eaten up and replaced with direct cmd go tos" (`src/build/linker.cr:148-150`). `Break`/`Continue` targets are resolved at build time (`src/cmd/control-flow/break.cr:4-5`).
3. **Run** — `Run::Runner` owns all global state (user variables as `Bytes`, labels, timers, display), `Run::Thread` owns the per-thread stack and settings, and one "clock" fiber (`src/run/runner.cr:161-190`) runs exactly one command at a time.

There is **no expression evaluator**. This is the legacy AHK v1 "command" syntax only: `x = 1` (`Cmd::Math::SetEnv`), `x += 1` (`Cmd::Math::EnvAdd`), `x := <awk formula>` (`Cmd::Math::EnvMath`), `++x`/`x--` (parser rewrites them, `src/build/parser.cr:223-230`). The README says so plainly and gives a migration table (`README.md:298-394`).

A consequence that matters for every later section: **the parser is the gate.** An unknown first word is not ignored — it aborts the whole script with `Syntax Error in line N: Command 'x' not found` (`src/build/parser.cr:253`, wrapped at `:37`). So every unimplemented command (and every removed directive) is a *hard parse failure*, not a silent no-op. This is a behavioural difference from Windows AHK, where an unknown command is also a load error but the *known* ones like `#UseHook` load fine.

---

## 1. Hotkeys (`src/run/display/hotkey.cr`, `hotkeys.cr`, `x11.cr`)

**Mechanism.** Hotkeys are X11 **passive grabs**, not a keyboard hook:

* `Run::Hotkey < Run::KeyCombination` parses the key string, then enumerates **modifier variants** (`src/run/display/hotkey.cr:60-78`). X11 treats `A` and `NumLock+A` as different grabs, so the class pre-computes all combinations of `LockMask, ControlMask, ShiftMask, Mod1(alt), Mod4(super), Mod5(altgr), Mod2(numlock)` (`:24-34`) and grabs each variant. Numpad keys are special-cased because their keycode depends on NumLock (`:45-58`).
* `~` sets `no_grab` (`:68`) — the hotkey still *observes* the key but does not swallow it. `*` adds wildcard modifier variants (`:71-77`).
* Grabbing is done by `Run::X11#grab_hotkey` with `XGrabKey`, or `XGrabButton` when the keysym is a mouse button (< 10) (`src/run/display/x11.cr:382-394`).
* **Grab target is re-attached to the active window.** By default the grabs live on the currently active window and are ungrab/regrab'd whenever `_NET_ACTIVE_WINDOW` changes (`src/run/display/x11.cr:296-311`). `#XGrabRoot` switches to grabbing the root window (`:364-372`, parser at `src/build/parser.cr:115-116`) — the README documents this as a workaround for KDE (`README.md:257-259`).
* Key events arrive from the **XTST Record extension**, not from the normal event loop: `Xtst::RecordExtension` with a range `KeyPress..ButtonRelease` (`src/run/display/x11.cr:174-184`, consumed at `:325-341`). The normal event loop is still needed "or otherwise the hotkeys aren't even grabbed" (`:277-291`).
* `Run::Hotkeys` keeps its own list, matches `keysym + up/down + exact modifier state` and fires `hotkey.trigger` (`src/run/display/hotkeys.cr:65-91`).

**Options supported.** `Hotkey, KeyName [, Label, Options]` (`src/cmd/control-flow/hotkey.cr`): `On`/`Off` toggling, `B` (buffer) and `T` (max threads) are *parsed but unused* (`:21-29` — both carry `# TODO: not used`), `P` sets the thread priority (`:27`, `src/run/runner.cr:130-144`). Dynamic registration goes through `Hotkeys#add_or_update` (`src/run/display/hotkeys.cr:38-63`), which reuses the object and can auto-exempt `Suspend` hotkeys (`:50`).

**Key remapping.** A hotkey whose action is a single word that is *not* a command becomes a remap: the parser rewrites `a::b` into `*a::` `Send {blind}{b down}` plus `*a up::` `Send {blind}{b up}` (`src/build/parser.cr:172-195`). That is why remaps work with `{blind}` and why the README's mouse-remap caveat exists (`README.md:241-256`).

**BlockInput.** `Cmd::X11::Keyboard::BlockInput` calls `DisplayAdapter#block_input`, implemented as `XGrabKeyboard` + `XGrabPointer` (`src/cmd/x11/block-input.cr:2-12`; `src/run/display/x11.cr:433-440`). Note: README lists BlockInput as TO DO, but it is implemented.

**Pause / Suspend / Reload.**

* `Suspend` (`src/cmd/misc/suspend.cr:1-11`) → `Runner#suspend` (`src/run/runner.cr:379-390`) → `Display#suspend` → `Hotkeys#suspend`, which ungrabs every hotkey except those whose command *is* `Suspend` (`src/run/display/hotkeys.cr:103-116`, exemption set at `:50`). The tray icon switches to `input-keyboard` (`src/run/display/gtk.cr:330-339`).
* `Pause` (`src/cmd/misc/pause.cr:1-11`) → `Runner#pause_thread`. It reaches into `@threads[@threads.size - 2]` — i.e. "the thread that is not me" — and sets `Thread#paused` (`src/run/runner.cr:391-404`); the clock fiber then skips paused threads and shows a loading icon (`:164-170`, `src/run/display/gtk.cr:340-353`). This is **not** the AHK per-thread pause model; `Pause, , 1` (thread priority) does not exist.
* `Reload` (`src/cmd/misc/reload.cr:1-5`) → `Runner#reload`: spawns a new process with the same `ARGV` and `exit`s (`src/run/runner.cr:196-200`).

**Completeness.** Core hotkeys, wildcards, `~`, up/down, mouse-button hotkeys, dynamic on/off, remaps, suspension and priority are present. Missing: `ListHotkeys`, `KeyHistory`, `#InstallKeybdHook`/`#UseHook` (parse errors), a poll/evdev fallback, per-hotkey `#IfWinActive` context (`#If` is not parsed at all), and any rate limiting (`#HotkeyInterval`, `#MaxHotkeysPerInterval` are not parsed).

**Limitations visible in code.** Grab failures are logged and swallowed (`src/run/display/x11.cr:247-249`: "Grabbing failed… You can probably ignore this error"); a hotkey that fires while the same key is physically down is special-cased with an extra synthetic key-up (`src/run/display/hotkeys.cr:77-88`, `src/cmd/x11/keyboard/send.cr:60-73`); `max_threads` overflow skips the press with a warning on stderr (`src/run/display/hotkey.cr:83-87`).

---

## 2. Hotstrings (`src/run/display/hotstring.cr`, `hotstrings.cr`)

**Word recognition engine.** `Run::Hotstrings` keeps a **fixed 30-char keysym buffer** (`StaticArray(Char, 30)`, `src/run/display/hotstrings.cr:4,27`) fed by the same XTST Record key listener as hotkeys (`:16-19`). On each *normal* keypress (no Ctrl/Alt/Super/AltGr, checked via the event state mask, `:48`) it:

* BackSpace → decrement the buffer (`:50-51`);
* an **end character** (default set: `- ( ) [ ] { } : ; ' " / \ , . ? ! \n space tab \r`, `src/run/runner.cr:21`) → reset the buffer and, if the previous keypress had produced a candidate match, fire it with `A_EndChar` set (`:52-57`);
* anything else → append to the buffer and ask every hotstring whether its keysyms match (`:58-70`). A match with `*` fires immediately (empty `A_EndChar`), otherwise it becomes the "candidate" that waits for an end char.

Matching is either case-sensitive or a `downcase` comparison (`src/run/display/hotstring.cr:32-39`). Firing does **automatic backspacing** of `abbrev.size (+1 for the end char)` keystrokes through `x_do.keys "BackSpace"` and then starts a normal thread with the hotstring's label (`:40-59`).

**Option prefixes.** `:R:`/`:C:` are handled by one options parser: `*` immediate, `b`/`B0` backspacing, `c`/`C`/`C1` case sensitivity and "conform case", `k` delay, `o` omit ending char, `p` priority, `r` "raw" = emit `SendRaw` instead of `Send` (`src/run/display/hotstring.cr:17-31`; `:R:` → `auto_send_raw`, applied at `src/build/parser.cr:167-170`). Hotstring lines are rewritten by the parser into a `Label` plus an optional `Send`/`SendRaw` + `Return` (`src/build/parser.cr:154-171`), and `#Hotstring EndChars` resets the end-char set (`:87-93`).

**Completeness / limits.** `?` (inside-a-word) is **not** parsed — only `*bckopr` are read. `C1` "conform case" is accepted but explicitly not implemented (`@conform_case = true # TODO not implemented`, `src/run/display/hotstring.cr:12`). The buffer is 30 chars (`hotstrings.cr:59`). Test coverage acknowledges a real gap: `;; TODO: hotstring with _ in it doesn't work` (`tests.ahk:826`) and case detection is skipped with xdotool input (`tests.ahk:629-632`).

---

## 3. Threads (`src/run/thread.cr`, `src/run/runner.cr`)

The v1 "thread" model is reproduced as **cooperative pseudo-threads**, and the code says so: "AHK threads are no real threads but pseudo-threads and pretty much like crystal fibers, except they're not cooperative at all; they take each other's place (prioritized) and continue until their individual end. Threads never really run in parallel: There's always one 'current thread'" (`src/run/thread.cr:40-43`).

* **Queue and priority.** `Runner#add_thread` inserts the new thread into `@threads` by descending priority (`src/run/runner.cr:130-144`); the clock fiber always services `@threads.last` (`:161-190`).
* **Interruption.** A thread runs exactly one command per scheduling step, then yields at the `select` in the clock (`:171-185`). So a long command (a `Sleep`, a blocking dialog) keeps running in its own fiber while another thread takes the lead — the comment at `:153-159` states that explicitly.
* **Per-thread settings.** `ThreadSettings` holds `last_found_window`, `msgbox_response`, the five `CoordMode` values, `detect_hidden_windows`, `key_delay`, `key_press_duration`, `mouse_delay` (`src/run/thread.cr:13-26`). Each thread gets a copy; the auto-execute thread's final settings become the defaults for later threads (`src/run/runner.cr:180-182`).
* **Per-thread built-ins.** `ErrorLevel` (`:75-77`, set by any command with `sets_error_level`, `:176-181`) plus computed `A_Index`, `A_DetectHiddenWindows`, `A_KeyDelay`, `A_MouseDelay`, `A_LineNumber`, `A_ThisLabel`, `A_Gui`, `A_GuiControl` (`:229-240`).
* **Stack.** `gosub` pushes, `return` pops, `goto` replaces the top (`:195-215`); the loop stack is separate (`:91`).
* **Exit / OnExit.** `Thread#exit` clears the stack and records the code (`:211-215`); `Runner#exit_app` calls `::exit` (`:192-194`). **There is no `OnExit` command and no exit-handler dispatch anywhere** — the only hook is a Crystal `at_exit` that kills the SoundPlay PID and runs `GC.collect` (`src/run/runner.cr:113-119`, `src/run/display/x11.cr:5`). A grep for `OnExit|on_exit|exit_handler` finds nothing else.
* **Timers.** `Run::Timer` wraps `Tasker.every`; if the previous timer thread has not finished, the tick is skipped (`src/run/timer.cr:16-26`).

**Completeness.** Faithful in the important respects (one current thread, priority, per-thread state, background sleeps, per-hotkey thread caps). Missing entirely: `Thread` command (`Priority`, `Interrupt`), `#MaxThreads`/`#MaxThreadsBuffer`, `SetBatchLines` (there is no line-count yield at all), `OnExit`, and true input-buffer semantics.

---

## 4. Windows (`src/cmd/x11/window/**`, `win-util.cr`)

**Search.** All window commands funnel through `Cmd::X11::Window::Util.match_all` (`src/cmd/x11/window/win-util.cr:6-63`), which implements the AHK window-spec in this order:

* no arguments → the **last found window** (`:10`), or `"A"` → the active window (`:11-13`);
* `ahk_id <n>` → constructs an `XDo::Window` directly and rejects it if it has no name (`:15-19`, "avoids segfaults");
* otherwise a **libxdo search** (`:28-42`) with `require_all`, and — unless `DetectHiddenWindows` is on — `only_visible` + `desktop(current_desktop)`. `ahk_class` → `window_class`, anything else → `window_name` with `.*` when empty.
* `WinText` / `ExcludeText` are implemented by walking the window's **AT-SPI** text (`get_all_texts`, `:48-57`) — so any command that matches by text requires the accessibility bus, exactly as the README warns (`README.md:261-263`).

The "last found window" is per-thread (`thread.settings.last_found_window`), and every `match` also issues a throw-away `x_do.focused_window sane: false` because WM commands otherwise "fail unless there is some other, arbitrary x11 request being sent after" (`win-util.cr:65-77`).

**Win\* verbs present** (all `.cr` under `src/cmd/x11/window/`): `WinActivate`, `WinActivate`-by-text, `WinClose`, `WinGetClass`, `WinGetPos`, `WinGetText`, `WinGetTitle`, `WinGet` (`id/pid/processname/controllist`, plus `Count`/`List`), `WinHide`, `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`, `WinMinimizeAllUndo`, `WinMove`, `WinRestore`, `WinSet`, `WinSetTitle`, `WinShow`, `WinWait`, `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`.

**Mechanisms.** Mostly libxdo window verbs: `activate!/focus!/raise!` (`win-activate.cr:11-13`), `quit!` (`win-close.cr:10`), `kill!` (`win-kill.cr:10`), `move/resize` (`win-move.cr:16-17`), `unmap!/map!` (`win-hide.cr:9`, `win-show.cr:9`), `minimize!` (`win-minimize.cr:8`). Maximize/restore go through EWMH `_NET_WM_STATE` (`win-maximize.cr:8-9`, `win-restore.cr:8-9`) — so they need an EWMH window manager. `WinMinimizeAll` sends the `_NET_SHOWING_DESKTOP` client message to the root window (`src/run/display/x11.cr:442-453`). `WinSetTitle` writes `WM_NAME` only (`win-set-title.cr:17`) — not `_NET_WM_NAME`. `WinSet` supports `AlwaysOnTop`, `Bottom` (raw `lower_window`), `Transparent` (the `_NET_WM_WINDOW_OPACITY` atom) and `TransColor` (GTK-only, and only for a Gui whose `Gui, Color` matches, `win-set.cr:33-48`). Waits poll with exponential backoff (`src/util/exponential-back-off.cr`, e.g. `win-wait.cr:13-19`).

**Gaps.** There is no `SetTitleMatchMode` — the match mode is whatever libxdo's `window_name` does, which the source itself flags: "seems to be partial match but only at *start* of string" (`win-util.cr:40`) and "TODO: is this regex? how to make partial matches like ahk?" (`:37`). `WinActivateBottom`, `WinGetActiveTitle/Stats`, `WinMenuSelectItem` are absent; `WinClose`/`WinKill` accept and then **delete** the `SecondsToWait` parameter (`win-close.cr:8`, `win-kill.cr:8`); `WinRestore` does not un-minimize.

---

## 5. Controls / accessibility (`src/run/display/at-spi.cr`, `src/cmd/x11/control/**`, `control-click.cr`)

**What AT-SPI gives this project.** `Run::AtSpi` (471 lines) implements a hand-rolled accessibility client over the D-Bus AT-SPI bus:

* **Window lookup**: an app is found by PID, then its top-level frames are matched **by title**, falling back to **approximate size** when the title is missing or ambiguous (`src/run/display/at-spi.cr:25-77`). The comment admits the root cause: "There is no match by window XID" (`:41`, linking an at-spi2-core issue). Results are cached per thread by X window id (`:28-29, :72-74`; cache declared in `src/run/thread.cr:29-32`).
* **ClassNN**: since accessibility elements have no stable id, the project *forges* one — `role_childindex_childindex…` (`a b` + path → `a_b_0_2`), built by walking `index_in_parent` (`at-spi.cr:248-262, 416-440`). Lookup parses the same format back and walks children (`:87-111`).
* **Text read**: `AbstractText#text(0, -1)` with a fallback to the accessible *name*, stripping U+FFFC and whitespace (`:307-317`); `WinGetText` joins all descendant texts with `\n` (`src/cmd/x11/window/win-get-text.cr:10-13`), capped at 1000 children per node (`at-spi.cr:330-338`).
* **Text write**: `EditableText#text_contents=` (`:318-326`) — used by `ControlSetText`.
* **Geometry**: screen extents minus the window origin (`:444-469`), because window-relative extents are "faulty … most prominently Firefox/Thunderbird" (`:445-456`).
* **Click**: no pointer is moved. It searches a prioritized action-name list (`click, press, push, activate, …`, `:357-359`), special-cases XFCE/Thunar with an `[edit, activate]` pair (`:365-372`), walks up when the action is `clickAncestor` (`:379-386`), and falls back to **Selection** for tab-like widgets (`:344-356`).
* **Robustness**: every request is wrapped in `Display#at_spi`, which retries 5 times 600 ms apart and then gives up returning `nil` (`src/run/display.cr:125-145`).

**Which commands use it**: `ControlGetPos` (`src/cmd/x11/control/control-get-pos.cr:17-22`), `ControlGetText` (`control-get-text.cr:13-18`), `ControlSetText` (`control-set-text.cr:13-20`), `ControlClick` (`src/cmd/x11/mouse/control-click.cr:16-25`), `WinGetText`, `WinGet CONTROLLIST` (`win-get.cr:30-40`), `MouseGetPos`'s control output (`mouse-get-pos.cr:7-10`), and the `WinText` filter (`win-util.cr:48-57`).

**Gaps / honesty notes.** No `ControlFocus`, `ControlMove`, `Control`, `ControlGet`, `ControlGetFocus`, `GuiControlGet`; no control enumeration by class name in the AHK sense (CONTROLLIST deliberately returns only *interactive* elements, `win-get.cr:32-35`); the README's accessibility section is the project's own admission that this whole family is environment-dependent (`README.md:261-263`), and the performance notes say `WinGetText` can take 0–3 s (`README.md:408`).

---

## 6. GUI (`src/cmd/gtk/**`, `src/run/display/gtk.cr`)

**Architecture.** GTK3 runs its own `Gtk.main` on a dedicated worker thread (`src/run/display/gtk.cr:14-17`, started in `src/run/display.cr:41-44`). Every widget operation must be marshalled onto that thread; `Gtk#act` does it with `GLib.idle_add` plus a `Channel`, wrapping an `@act_mutex` and disabling the GC around the call (`gtk.cr:19-50`). The README/`build/README.md` call gi-crystal + GTK3 the GUI dependency (`README.md:440`, `build/README.md:15`).

**Gui subcommands** exist as separate classes, produced by the parser rewriting `Gui, 2:Add, …` into `GuiAdd, 2, …` (`src/build/parser.cr:196-212`): `GuiAdd`, `GuiShow`, `GuiSubmit`, `GuiCancel`, `GuiHide`, `GuiDestroy`, `GuiColor`, `GuiScroll` (AHK_X11-only), `GuiOption`, `GuiControl`. Window objects live in `Gtk#guis : Hash(String, GuiInfo)` with a `Gtk::Fixed` layout container, an optional scroll area, remembered last x/y, padding and per-variable control table (`gtk.cr:366-435`).

**Control types in `gui-add.cr`** — the list is 8 entries, not 30:

| `ControlType` | GTK widget | Notes |
|---|---|---|
| `Edit` (and default Multi-line when `R>1`) | `Gtk::Entry`, or `Gtk::ScrolledWindow`+`Gtk::TextView` | `gui-add.cr:28-43` |
| `Button` | `Gtk::Button` | also fires a `button<text>` label, `:44-50` |
| `Checkbox` | `Gtk::CheckButton` (`Checked` option) | `:51-54` |
| `DropDownList`/`DDL` | `Gtk::ComboBoxText` with `|` items and `||` selection, `Choose` | `:55-65` |
| `ListBox` | `Gtk::ListBox` (`Multi`, `Choose`) | `:66-85` |
| `Picture`/`Pic` | `Gtk::EventBox`+`Gtk::Image`, `icon:` paths, scaling | `:86-116` |
| `Scroll` | `Gtk::ScrolledWindow` + nested `Gtk::Fixed` | `:117-123` |
| anything else | `Gtk::Label` (so `Text` and unknown types) | `:124-130` |

There is **no** Radio, Slider, Progress, Tab, ComboBox, GroupBox, StatusBar, MonthCal, UpDown, Hotkey, DateTime, TreeView, ListView, or Picture-button. Options are parsed loosely by `parse_word_options` (`src/util/ahk-string.cr:188-220`): `v`, `g`, `w`, `h`, `x`, `y`, `xp`, `xm`, `xs`, `yp`, `ym`, `ys`, `section`, `checked`, `choose`, `multi`, `altsubmit` are used; `X+n`/`wp+n` are explicitly not possible (`gui-add.cr:145-157`).

**`GuiControl`** is the weakest of the family: only the *empty* sub-command is implemented, and it dispatches on the widget class to set text/state (`src/cmd/gtk/gui/gui-control.cr:14-66`); any other widget raises "GuiControl not yet supported for element of type …" (`:64`). Text/Move/Focus/Disable/Hide/Choose do not exist, and there is no `GuiControlGet`.

**`Menu`** is tray-only: `raise … "MenuName has to be TRAY" if args[0].downcase != "tray"` (`src/cmd/gtk/menu.cr:6`), with `Add`/`Icon`/`NoIcon`/`Tip` on a `Gtk::StatusIcon` (`:7-53`). The README lists `Menu` as DONE without mentioning this restriction.

**`ToolTip`** = one borderless always-on-top `Gtk::Window` per id, positioned at x/y or at the mouse (`src/cmd/gtk/tooltip.cr:1-42`; windows created in `gtk.cr:436-464`); updating its label can randomly fail and is only logged (`tooltip.cr:16-23`). **`TrayTip`** = libnotify (`src/cmd/gtk/tray-tip.cr:1-29`, `Notify.init` at `gtk.cr:322`). **Dialogs**: `MsgBox` (buttons/icons/always-on-top/timeout, `gtk.cr:136-195`), `InputBox` (`gtk.cr:197-252`), `FileSelectFile`/`FileSelectFolder` (`Gtk::FileChooserDialog`).

**Clipboard** is part of this subsystem too: the `Clipboard` variable is intercepted in `Runner#set_user_var` (`src/run/runner.cr:268-270`) and read in `get_global_built_in_computed_var` (`:305`). *Unsetting* is emulated with an image, then a magic text, then an empty text, each verified by polling — with three distinct failure exceptions (`gtk.cr:58-97`).

---

## 7. Screen (`PixelGetColor`, `PixelSearch`, `CoordMode`)

**Mechanism — correction to a common assumption.** The pixels do **not** come from a direct `XGetImage` call in this tree. They come from **GDK**:

* `PixelGetColor`: `Gdk.pixbuf_get_from_window(Gdk.default_root_window, x, y, 1, 1)`, then the first three bytes of the pixel buffer are read, reversed unless the `RGB` flag was given, and hex-formatted (`src/cmd/misc/pixel-get-color.cr:14-30`). It runs inside `gtk.act` because GDK must be used from the GTK thread.
* `PixelSearch`: one `Gdk.pixbuf_get_from_window` of the whole region, then a per-pixel loop honouring `rowstride`/`n_channels` and a per-channel `|delta| <= variation` test (`src/cmd/misc/pixel-search.cr:25-55`). It returns 2 for an unexpected pixbuf layout (`:28`).

GDK itself uses `XGetImage` on an X11 backend, so the pixels ultimately come from the X server — but the call site, and therefore the Wayland story, is GDK, not Xlib.

`CoordMode` sets four/five per-thread enums (`src/cmd/misc/coord-mode.cr:1-22`, `src/run/thread.cr:16-20`); `Screen` is the default and `RELATIVE` means **relative to the active window** via `x_do.active_window.location` (`win-util.cr:82-89`).

---

## 8. Sound (`src/cmd/misc/sound-*.cr`)

* **Playback ladder**: `SoundPlay` tries `ffplay` → `cvlc` → `paplay` → `mpg123` → `aplay`, first one found on `PATH` (`src/cmd/misc/sound-play.cr:11-24`), kills the previous SoundPlay PID before starting (`:7-9`), and honours the `wait` argument by waiting for the exit code (`:26-29`). The README's "player ladder" is literally this array.
* **Volume source**: `SoundGet`/`SoundSet` shell out to **amixer** and parse its text output: `amixer get Master` + `grep -c '[on]'` for mute state, or `awk -F"[][]" '/Left:/ {print $2}'` for the level (`src/cmd/misc/sound-get.cr:10-16`); `amixer sset Master [N% | N%+ | N%- | mute | unmute | toggle]` (`src/cmd/misc/sound-set.cr:11-30`).
* **Gaps**: `SoundGetWaveVolume`/`SoundSetWaveVolume` are TODOs with no class; the `ComponentType`/`ControlType`/`DeviceNumber` parameters are ignored (only "Master"); there is no PulseAudio/PipeWire API — the project depends on ALSA tooling being installed.

---

## 9. Files & INI (`src/cmd/file/**`)

A thin layer over Crystal's `File`/`Dir`/`Path`, plus a globbing helper:

* `Cmd::File::Util.match` (`src/cmd/file/file-util.cr:7-26`) globs `pattern` (adding `**` when Recurse=1) and filters folders by `OperateOnFolders` (0=files only, 2=dirs only). This is used by `FileCopy`, `FileSetAttrib`, `FileSetTime` and the file `Loop`.
* Implemented: `FileAppend`, `FileCopy`, `FileCreateDir`, `FileDelete`, `FileGetSize`, `FileRead`, `FileReadLine`, `FileRecycle` (via `gio trash`), `FileRecycleEmpty` (`rm -rf …/Trash/*`), `FileSetAttrib` (chmod), `FileSetTime` (`File.touch`), `SetWorkingDir`, `SplitPath`, `DriveSpaceFree` (`df -Pm`+awk), and `IniRead`/`IniWrite`/`IniDelete` via the `ini` shard.
* Gaps, all visible in code: `FileSetAttrib` only implements `X` and `N` (R/A/S/H/O/T are parsed and dropped, `file-set-attrib.cr:12-27`); `FileSetTime` only implements `M` (`file-set-time.cr:19-21`); `SplitPath` never sets the 5th output (`split-path.cr:7-10`); INI writes rebuild the whole file and therefore lose comments and ordering (`ini-write.cr:19`); there is no `FileMove`, `FileCopyDir`, `FileRemoveDir`, `FileGetTime`, `FileGetAttrib`, `FileGetVersion`, `FileGetShortcut`, `FileCreateShortcut`, `FileInstall`.
* **[code-evident, not runtime-verified]** `FileGetSize` declares `sets_error_level; true` but its success path ends on `set_user_var` (returning `Bytes`), while `Thread` raises `"Result should be String for ErrorLevel command??"` for any non-`String` result (`src/run/thread.cr:176-178`). Its test is disabled (`tests.ahk:318`). Only the failure path returns a `String`.

---

## 10. Processes / environment (`src/cmd/misc/run.cr`, `run-as`, `shutdown`, `env-*`)

* `Run`/`RunWait` (`run-wait.cr` is `Run` with `@wait=true`): the target is first tried through `Gio.app_info_launch_default_for_uri` (handles URLs and files by MIME), then as a shell command `Process.new(cmd, shell: true)`, then `xdg-open` (`src/cmd/misc/run.cr:120-143`). Non-waiting runs are prefixed with `setsid` because nothing else kept the child alive (`:48-53`). `OutputVarPID`/`OutputVarStdout`/`OutputVarStderr` and `UseErrorLevel` are supported (`:141-169`); `Run open|edit|explore|print` prefixes are recognised, with `edit` going through `gtk-launch <default text editor>` and `print` degraded to `lp` (`:87-121`).
* `RunAs` only stores credentials (`src/cmd/misc/run-as.cr:1-9`); the actual switch happens inside `Run` via `su --login USER -c …<<< password` plus temporary `xhost si:localuser` grants (`run.cr:27-44, 42, 74`).
* `EnvGet`/`EnvSet` are direct `ENV[]` access (`env-get.cr`, `env-set.cr`); `#NoEnv` is a no-op and `AutoTrim` does not exist, so environment variables are never auto-substituted.
* `Shutdown` shells out to desktop-specific chains (`gnome-session-quit || mate-session-save … || loginctl terminate-session`, `halt --poweroff`, `sudo -S reboot --force || pkexec … || gksu …`) (`src/cmd/misc/shutdown.cr:8-16`); codes 3 and 7 have no branch.

---

## 11. `DllCall` (`src/cmd/misc/dll-call.cr`)

**Mechanism.** It uses **Crystal's own runtime FFI**, not `libffi` directly:

* `Crystal::Loader.new(Crystal::Loader.default_search_paths)` (`:11`), `load_library?` + `find_symbol?` (`:51-61`);
* `Crystal::FFI::CallInterface.new(return_type, param_types)` and `.call(...)` (`:59-69`), i.e. libffi underneath Crystal's compiler library;
* the type list is a hand-written mapping of 12 names — `ptr/uptr/str`, `int64`, `int`, `short`, `char`, `uint64`, `uint`, `ushort`, `uchar`, `float`, `double` (`src/util/ahk-bin-var.cr:3-19`). Anything else raises `"Invalid binary var type"`.

**The string-only-variable limitation, stated precisely.** Every user variable is a `Bytes` slice holding a serialized string (`src/run/runner.cr:44-48`). For a `DllCall` parameter the code *reinterprets* that slice as the declared type: it computes the C size, allocates a fresh writable buffer if the slice is read-only or too small, copies the converted value in, and — if the buffer moved — writes the new slice back to the variable (`dll-call.cr:20-40`). On return it serializes each parameter and the return value back to strings (`:71-75`). Consequences: integers/doubles round-trip through text; the returned value of type `ptr` is a raw address stored as bytes; `str`/`*`/`p` suffixes mean "pass a pointer" (`:86-93`); return codes `-2/-3/-4` mean missing variable, missing library, missing symbol (`:24, :48, :61`). It also **maintains its own `Crystal::Loader` monkey-patch** (`src/hacks.cr:64-78`) so that library names resolve and so the loader's search path works under musl.

Practical note: because `Crystal::Loader` uses the local compiler toolchain's linker for search paths (`hacks.cr:73-78`), `DllCall` assumes a compiler-ish environment unless the library path is absolute.

---

## 12. Binary / memory (`NumGet`, `NumPut`, `VarSetCapacity`, `src/util/ahk-bin-var.cr`)

`VarSetCapacity` allocates `Bytes.new(capacity, fill_byte)` and stores it as the variable, reporting the granted capacity (`src/cmd/misc/var-set-capacity.cr:3-17`); `NumGet` reads a typed value at an offset (`num-get.cr:4-30`); `NumPut` writes one, growing the buffer to 8 bytes when the variable is missing or read-only (`num-put.cr:4-32`). `Util::AhkBinVar` is the shared type serializer (`str_to_type`/`type_to_str`, `src/util/ahk-bin-var.cr:21-54`). Types available are the same 12 as `DllCall`. This is what makes the AHK pattern `VarSetCapacity(buf, 260)` + `DllCall(..., "str", buf)` usable at all — at the price of all data being strings in the general case.

---

## 13. Build / compile, REPL, `--windowspy`, installer, packaging

* **Parse/link** — see §0. Parser errors carry the line number and the offending text (`src/build/parser.cr:37`); linker errors say "most likely a closing brace is missing somewhere" (`src/build/linker.cr:242`).
* **Compile** — `Compiler#compile` copies the running binary and appends: the script bytes, the 8-byte trailer `"AHK_SOUR"`, and the original binary size as an `Int64` (`src/compiler.cr:13-26`). At startup `Compiler#extract` seeks to `-16`, checks the trailer, reads the size and returns the script text (`:27-34`), which `src/ahk_x11.cr:69-71` treats as "compiled". This is a *self-extracting append*, not a real compiler — and portability depends entirely on running inside an AppImage (`compiler.cr:14-16`).
* **REPL** — after the auto-execute section ends (or immediately with `--repl`), `Runner#repl` reads one line at a time and `eval`s it through the builder into a new thread (`src/run/runner.cr:210-237`). Documented consequence: each line is its own thread, so `ErrorLevel` and variables from previous lines are not shared as they would be (`README.md:224-225`).
* **Window Spy** — `--windowspy` sets the script text to the embedded `src/window-spy.ahk` (`src/ahk_x11.cr:85-86`, `read_file("./src/window-spy.ahk")` at compile time). The spy is a normal AHK script that uses `WinGet`, `WinGetText`, `MouseGetPos`, `ControlGetPos/Text`, `PixelGetColor`, `CoordMode` and a Gui (`src/window-spy.ahk:1-97`) — so it is exactly as capable as those commands.
* **Installer** — with no script, no stdin and no flags, the embedded `src/installer.ahk` runs (`src/ahk_x11.cr:103-109`); the binary also supports `-v/--version`, `-h/--help`, `--repl`, `--compile`, and a hashbang line (`src/build/parser.cr:117`).
* **Packaging** — AppImage built on Ubuntu 20.04 with linuxdeploy; `make bin/ahk_x11.dev` for a local build; `release.sh` publishes; an Alpine Dockerfile exists for headless GUI use (`build/README.md:7-47`). `xdotool` is a **git submodule** (`.gitmodules:1-3`) and the tree expects a *patched* libxdo, because the stock one can segfault in `xdo_get_desktop_for_window` (the code says so at `src/cmd/x11/window/win-util.cr:26-27` and points at `build/README.md`). In the checkout I audited the `xdotool/` directory is **empty** (submodule not initialised).
* **Runtime knobs** — the binary forces `CRYSTAL_WORKERS=3` and `LC_ALL=en_US.UTF-8` at startup (`src/ahk_x11.cr:17-21`), and warns loudly when `XDG_SESSION_TYPE != "x11"` (`:57-59`).

---

## 14. Display-adapter abstraction (`src/run/display/display-adapter.cr`) and its implementations

The abstraction is **five methods**: `initialize(runner)`, `run(*, key_handler)`, `grab_hotkey`, `ungrab_hotkey`, `block_input`, `unblock_input` (`src/run/display/display-adapter.cr:1-11`). It is the *input and grabbing* seam only.

There is **exactly one implementation, `Run::X11`** (`src/run/display/x11.cr:137`), instantiated unconditionally at `src/run/display.cr:22`. `Run::Gtk` and `Run::AtSpi` are **not** adapters — they are separate classes held alongside the adapter (`src/run/display.cr:12-29`), and every caller reaches through them directly (`runner.display.gtk`, `runner.display.x_do`, `runner.display.at_spi`). So although the file is named like a portability layer, a non-X11 port would have to change every one of the ~60 call sites that say `display.x_do`, `display.adapter.as(Run::X11)`, or `display.adapter.display.*`. Additional X11-only members used by commands (`grab_keyboard`, `show_desktop`, `display`, `keysym_to_keycode`) are not in the abstract class at all.

That mismatch is one of the strongest structural findings of this audit: **the abstraction does not isolate X11.** See `t1-findings.md`.

---

## 15. Completeness summary (source-read)

| Subsystem | Completeness | Hard blockers to parity |
|---|---|---|
| Command dispatch / interpreter | High | legacy syntax only, no expressions/functions/objects |
| Control flow (if/loop/gosub) | High | `if var is type` absent; no `#If` contexts |
| Hotkeys | High on X11 | X11-only grabbing; no `#If`, no key history, no rate limits |
| Hotstrings | Medium-high | `?` absent, `C1` unimplemented, 30-char buffer |
| Threads | Medium | no `Thread` command, no `OnExit`, non-preemptive |
| Windows | Medium | match-mode fixed, no activation retry, no `WinActivateBottom` |
| Controls (AT-SPI) | Medium | no XID mapping, slow, environment-dependent |
| GUI | Medium-low | 8 control types, `GuiControl` write-only and one subcommand |
| Screen | Medium | GDK capture, naive search |
| Sound | Low-medium | amixer only, Master only, no wave volume |
| Files/INI | Medium | no move/remove-dir/attrib-get/time-get; INI rewrites |
| Processes/env | Medium-high | shell-based, su for RunAs |
| DllCall | Medium | string-only variables, needs `cc` |
| Binary vars | Medium | 12 types, offsets unchecked |
| Build/compile | High for what it is | append-to-binary, AppImage-dependent |
| Packaging | Medium | submodule-patched libxdo, no vendored build |
