# The desktop backend

The language does no I/O of its own. Everything a script does to the
machine -- type a key, move the pointer, read a pixel, look at a window,
read the clipboard, call a library -- goes through one trait, and the
crate that implements it is `ahk-box`. That implementation is what this
document is about.

The short version, and the one to read first: **the backend is a Linux
desktop.** Its keyboard is a uinput device, its pointer is a uinput
device, its screen is read with `grim`, its dialogs are `zenity`, its
sound is `pactl`. On a host that is not Linux the crate still compiles and
every one of those actions answers a reason -- `NotLinux` when the
primitive is a kernel device, or the failure of the program the primitive
shells out to (`grim failed: sh: line 0: grim: not found`) when it is
not. It does not answer a wrong value and it does not answer `false`.
That refusal is honest and it is worth knowing, because it is what a
script gets on a Windows workstation.

Three capabilities work on any host, because they are the program's and
not the desktop's: the files, the network (`curl`) and the native calls
(the library of the host itself). Everything else needs the box.

- [The fifteen capabilities](#the-fifteen-capabilities)
- [The keyboard](#the-keyboard)
- [The pointer](#the-pointer)
- [The screen](#the-screen)
- [The windows](#the-windows)
- [The clipboard](#the-clipboard)
- [The sound](#the-sound)
- [The dialogs](#the-dialogs)
- [The files](#the-files)
- [The network](#the-network)
- [The device processes](#the-device-processes)
- [The hotkeys](#the-hotkeys)
- [The native calls](#the-native-calls)
- [The raven, and the libraries of Windows](#the-raven-and-the-libraries-of-windows)
- [The machine variables](#the-machine-variables)
- [The environment variables](#the-environment-variables)
- [The platforms, side by side](#the-platforms-side-by-side)
- [Driving a real desktop](#driving-a-real-desktop)

## The fifteen capabilities

`ahk::desktop::Desktop` is the sum of fifteen traits and has no method of
its own. A desktop writes the ones it has, and every method a capability
does not have has a default that says so -- the defaults all read
"this desktop cannot ...".

| Trait | What it is for | The box implements it |
| --- | --- | --- |
| `Keyboard` | Type text, press a key, hold a key, read which keys are down. | Yes, with uinput. |
| `Pointer` | Click, move, hold and release a button, read the position, set the speed and the delay. | Yes, with an absolute uinput pointer. |
| `Screen` | Capture a region, read a pixel, search for a colour, read the size of the screen. | Yes, with `grim`. |
| `Windows` | Whether a window is active, whether one exists, the title of the active one. | Yes, through the compositor. |
| `Processes` | Run a command, launch a program, read and write an environment variable. | Yes, through `sh -c`. |
| `Files` | Every file, folder and ini operation, and the working directory. | Yes, on the filesystem. |
| `Clipboard` | Read the text of the clipboard, write it. | Yes, with `wl-paste` / `wl-copy` or `xclip` / `xsel`. |
| `Timers` | Whether the desktop takes the timers over. | Yes -- and it does not take them over. |
| `Hotkeys` | Register a hotkey, take one away, list them, ask which one fired. | Yes, by watching the keyboard. |
| `Native` | Call a function of a library. | Yes, with a trampoline. |
| `Sound` | Read and set the volume, read and set the mute, play a file, play a tone. | Yes, with `pactl` and `paplay`. |
| `Machine` | The user, the computer, the shell, the folders, the version, the language, the elevation, the idle times. | Yes, from the Linux machine. |
| `Dialogs` | An input box, a file selection, a folder selection. | Yes, with `zenity`. |
| `Network` | Download a file. | Yes, with `curl`. |
| `Pictures` | Load a picture, look for a picture on the screen. | Yes, with the `png` crate and `grim`. |

The design decision behind the split is in the trait itself: a desktop
with a keyboard and no pointer says so about the pointer alone, and the
script that asks for the pointer is told why it cannot have it. There is
no blanket implementation, so a new implementor writes all fifteen and
the compiler makes sure none of them was forgotten.

The rule that follows from it: **a question the machine cannot answer
fails with the reason.** It does not answer `false`, and it does not
answer zero. A script that cannot tell whether the right window is in
front has to stop, because a script that types into the wrong window is
worse than a script that fails.

## The keyboard

Two things go through the keyboard of the box: the text a script types,
and the keys it presses. Both go through a uinput device the crate
creates itself.

### Why a device of our own

`ydotool` was the obvious tool and it is broken in ways that are worse
than "not implemented":

- `ydotool type` needs the `ydotoald` daemon. On the box that daemon is
  gone, and `ydotool type` without it writes into a socket nobody is
  listening on and reports nothing -- a silent failure.
- `ydotool key "Return"` is a **silent no-op** on ydotool 1.x, which
  takes numeric keycodes and reads a name as zero. The command exits 0 and
  the screen does not change.
- `ydotool key 28` presses the key but releases it late enough that the
  compositor's auto-repeat fires: one call for the letter `u` produced
  about a hundred and thirty `u`s and left the key held.
- `ydotool key 30:1` -- ctrl+a -- is not understood at all. The suffix is
  ignored and the bare key repeats.

So the crate makes its own device: one uinput node that reports key
events, with an explicit keydown and an explicit keyup a few milliseconds
apart. The compositor's auto-repeat only fires while a key is *held*, and
this device never holds one.

### What the device declares, and why it matters

An event travels: the kernel reads the device, libinput turns it into a
description for the compositor, and the compositor hands the key to the
client. A device declares what it can do, and libinput builds its keymap
from that declaration. A wrong declaration is accepted by the kernel and
stops at the compositor.

- **The device declares the same 112 key codes as the physical keyboard
  of the box** -- from `KEY_ESC` to `KEY_MENU`, read out of
  `/proc/bus/input/devices`. A device that declares codes a real keyboard
  does not have makes libinput build a keymap it cannot use: the key
  events arrive and are reported with the key code `-1`, which a client
  cannot use.
- **The scan codes ride with the key events.** The device emits
  `EV_MSC`/`MSC_SCAN` before each key event and declares the `MSC_SCAN`
  bit in its capabilities.
- **The device announces the relative axes and the timestamp of the
  device it replaces.** They are announced for the same reason: libinput
  builds the keymap of a device from what the device says it can do.

The device is named `ahk keyboard` and declares the ydotool vendor and
product so that libinput classifies it the way it classifies the device
that used to work.

### The text path

`Send` types one key at a time, in the US layout. A character the layout
does not have fails the statement with the message the code gives --
`the keyboard of this box has no key for <character>` -- rather than being
typed as something else. A capital is the key with shift held down
around it, and that is why capitals work.

A newline and a tab have a key of their own, and the text path takes them.

### The chord

A key with a modifier is sent the way a keyboard sends it: the modifier
goes down, the key goes down, the key comes up, the modifier comes up.

Measured on the box: `ctrl+a` selected the text of a GTK field and the
next key replaced it, and the same chord selected the text of a field of
a Steam store window and a backspace deleted it. Modifiers do reach the
client. The note that once said otherwise was measured with the `ydotool`
device, whose suffix is ignored -- it was a property of that tool and not
of the box.

### The timings

| Constant | Value | Why |
| --- | --- | --- |
| `KEY_HOLD` | 20 ms | Long enough for the compositor to see the press, short enough that the auto-repeat never starts. |
| `KEY_GAP` | 20 ms | Between the release and the next press. A client coalesces a burst; without the gap a burst of backspaces is counted as a handful. |
| `ENUMERATION_WAIT` | 4 s | How long the compositor needs to notice the device. A key pressed before that lands on nothing. |

The enumeration wait is the one that surprises: the first `Send` of a run
takes four seconds, because the device was just created.

### On a host that is not Linux

The keyboard is a Linux uinput device, and on a host without one every
key action answers:

```
the keyboard: the keyboard is a Linux uinput device; this is not Linux
```

```pwsh
'Send "abc"' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":"the keyboard: the keyboard is a Linux uinput device; this is not Linux","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

The `the keyboard: ` prefix is the wrapper of the desktop around the
error of the device, so the reason names which primitive refused.

## The pointer

The pointer is a *second* uinput device, and it is absolute: it reports a
position in screen pixels, not a movement. It has to be a device of its
own, because libinput picks one role per device and the role with the
richer key bitmap wins -- one node cannot be both the keyboard and the
pointer.

ydotool's device is a *relative* mouse with a full keyboard key bitmap,
so libinput classifies it as a keyboard: the relative axes are ignored
and a click arrives and moves nothing. The absolute pointer announces
`INPUT_PROP_DIRECT` and absolute axes over the size of the screen, which
is how the device Sunshine creates on that box announces itself.

### Two timings that are not "a click with a sleep in it"

- **`PRESS_SETTLE` is 200 ms**, between the motion and the press. The
  compositor coalesces events into frames, and a motion and a press in
  the same batch arrive together -- the press is then applied at the
  pointer's *previous* position. Measured on the box with a 20 ms gap,
  the click landed hundreds of pixels away from where it was aimed.
- **`PRESS_HOLD` is 40 ms**: long enough for the compositor to see the
  press, short enough that the auto-repeat never starts.

The device announces itself as a plain pointer and not as a tablet:
adding the tool bits makes libinput classify the node as a tablet, and a
tablet's tip press does not activate anything -- not the compositor's own
decorations, not a client.

### The operations

| Operation | What the box does |
| --- | --- |
| `Click(x, y, button, count)` | Moves, settles, presses and releases, `count` times with 60 ms between the clicks. |
| `MouseMove(x, y)` | Moves the pointer and nothing else. |
| `MouseClickDrag` | Put a button down at one place, move, let it up. |
| `MouseGetPos` | Reads the position of the pointer this runtime drives, out of the kernel. |
| `SetMouseDelay` | Keeps the delay between two movements. |

`MouseGetPos` is the one to be careful with. It reads the position of the
pointer *this runtime drives*, and that is the only pointer on the box
whose place can be read at all: the physical mouse is a relative device,
so its position is not something the kernel can answer. When the kernel
will not answer -- the event node had not been opened to the group yet --
the desktop answers the position the runtime last put the pointer at, and
says so in a note:

```
the kernel did not say where the pointer is (<reason>); this is the position the runtime put it at
```

### On a host that is not Linux

The pointer is not there, and the first thing every pointer action does
is ask the screen how big it is, because the absolute axes have to be
told the range they cover. On a host without `grim` that is where the
action stops:

```pwsh
'Click 100, 200' | target\debug\ahk.exe
```

```json
{"elapsed_ms":64,"error":"grim failed: sh: line 0: grim: not found\n","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

The reason names the screen and not the pointer. That is the order of the
chain -- the pointer cannot exist without the size of the screen -- and it
is worth knowing that the failure of a click on such a host is reported
as a failure of `grim`.

## The screen

Everything the screen capability does goes through `grim`, which writes a
PNG, and the `png` crate, which reads it.

| Name | What the box does |
| --- | --- |
| `PixelGetColor(x, y)` | Captures a 1x1 region and reads the one pixel: `#rrggbb`. |
| `PixelSearch(...)` | Captures the region once and walks it for the first pixel of the colour. |
| `ImageSearch(...)` | Captures the region once and looks for the picture in it, with the variation and the transparent colour. |
| `Screenshot` | Captures a region into a file. |
| `LoadPicture` | Reads the file with the `png` crate. |
| `A_ScreenWidth`, `A_ScreenHeight` | Captures the whole screen once and reads the size out of the PNG header. |
| `SysGet` | The size of the screen, or the reason. |

A colour is `#rrggbb`, `#rgb` or `rrggbb`. A colour written as a number --
`0xFF0000` or the decimal `16711680` -- is not one, and the statement fails
with:

```
"16711680" is not a colour: write it as #rrggbb
```

The size of the screen is measured once and kept for the run. The scratch
files of a capture live under `/tmp` and are removed after they are read.

The two readings that make up the *focus check* are `PixelGetColor` and
`PixelSearch`: they are read from the frame the compositor draws, so they
work whatever window is in front, Wayland or not. That is why a script
that wants to know whether the right window is up should read a pixel of
it and not ask `WinActive`.

### On a host that is not Linux

`grim` is not there, and every one of the names above fails with the same
reason:

```pwsh
'x := A_ScreenWidth' | target\debug\ahk.exe
```

```json
{"elapsed_ms":61,"error":"grim failed: sh: line 0: grim: not found\n","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

`LoadPicture` is the exception in one respect: it reads the file with the
`png` crate and never touches the screen, so a missing file is not a
failure. v2 answers zero for a picture that could not be loaded, and so
does the box:

```json
{"ok":true,"output":[],"steps":[{"detail":"r = 0","elapsed_ms":0,"line":1,"ok":true,"op":"Assign"}],...}
```

## The windows

The box is a Wayland session, and the compositor is the one that knows
the windows. On that session the questions are answered by the
compositor, through two protocols; on an X11 session they are answered by
Xlib and XTEST.

### The Wayland path

The client speaks two protocols, both vendored as XML in
`crates/ahk-box/protocols/` and bound at build time by `wayland-scanner`:

- `ext_foreign_toplevel_list_v1` -- the portable one. It lists the
  toplevels and says what each of them is: the title, the application id,
  and an identifier that is stable while the window lives. This is enough
  to answer whether a window exists.
- `zcosmic_toplevel_info_v1` -- the one of COSMIC, which *extends* the
  listing. It gives a handle of its own for a handle of the listing, and
  that one carries the state of the window -- and the state says whether
  the window has the focus. This is what answers which window is active.

The correlation between the two is by the handle, so it is exact and not a
guess by title. The state arrives a frame late: the client waits up to
250 ms, five milliseconds at a time, and a window whose state did not
arrive leaves the question about the focus open -- the answer is the
reason, not a wrong window.

The full guide, with the Rust that does it, is
[the Wayland toplevel enumeration](wayland-toplevel-enumeration.md).

### The matching of a title

The mode of v2 is the default one, 2: the title of the window has to
*contain* the text that was asked for, and the case matters --
"the window titles and text are case-sensitive". `SetTitleMatchMode` is
not carried, so the mode 2 is the one that is used.

The other three modes are the ones the trait carries -- starts with,
exactly, and a regular expression -- and a pattern that the engine of the
tree does not carry is a failure with the reason of the engine.

### A window that is not there is not a failure

The desktop answers `Active`, `NotActive`, `Absent` or `Unknown`:

- `Absent` is an answer and not a failure. `WinExist` answers 0 and
  `WinActive` answers 0.
- `Unknown` carries the reason. It is what a question the desktop cannot
  answer looks like, and the statement fails with it.

An empty title is `Unknown` too: v2 would use the last found window, and
this runtime does not keep one.

### On a host that is not Linux

The Wayland path is Linux-only, and on a host without a Wayland session
the questions fall to the X11 path, which asks `xprop`:

```pwsh
'WinExist("Untitled")' | target\debug\ahk.exe
```

```json
{"elapsed_ms":58,"error":"WinExist cannot answer: the X server on this box reports an empty client list (it is the rootless Xwayland of gamescope), and the compositor exposes no window list to D-Bus: a native Wayland window cannot be found this way. Use PixelSearch to check what is on the screen.","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

`WinActive` and `WinGetTitle` stop one step earlier, because the X11 path
has no display to ask at all:

```json
{"elapsed_ms":0,"error":"there is no X server on this box to ask: the session is Wayland, and the compositor exposes no window list to D-Bus.","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

The reason is a Linux reason on a Windows host, because the X11 fallback
is the Linux one and it does not know that the host is Windows.
That is one of the places where the report is honest about the backend
and not about the host.

The one thing that does work on any host is `PixelSearch`, and the reason
above says so: it is the check that works whatever the desktop is.

## The clipboard

The clipboard is the session's, and the tool that reads it depends on the
session.

| Session | Read | Write |
| --- | --- | --- |
| Wayland | `wl-paste --no-newline --type text/plain` | `wl-copy --type text/plain` |
| X11 | `xclip -selection clipboard -out`, or `xsel --clipboard --output` | `xclip -selection clipboard`, or `xsel --clipboard --input` |

The session is Wayland when `WAYLAND_DISPLAY` is set. The type is given to
`wl-paste` because without it the tool waits for ever for a type to
arrive.

Two things the box says out loud:

- **An empty clipboard is a value and not a failure.** `wl-paste` answers
  `Nothing is copied`, and the desktop turns that into the empty text.
- **The tool serves the clipboard in the background**, so the write runs
  with its output detached; otherwise the caller waits for the end of a
  channel that the tool keeps open.

`A_Clipboard` and `ClipWait` are the two names. `ClipWait` waits by
asking, and the clock of the runtime measures the wait.

On an X11 session with neither `xclip` nor `xsel` -- and the box has
neither -- the answer is the reason and not an empty text:

```pwsh
'x := A_Clipboard' | target\debug\ahk.exe
```

```json
{"elapsed_ms":72,"error":"the clipboard of an X11 session cannot be read: neither xclip nor xsel is installed on this machine","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

That is what a Windows host gets: the session has no `WAYLAND_DISPLAY`, so
it is the X11 path, and the X11 path is a Linux path.

## The sound

The mixer of the session is PulseAudio on PipeWire, and `pactl` is the
source. Six names answer with a value: `SoundBeep`, `SoundPlay`,
`SoundGetVolume`, `SoundSetVolume`, `SoundGetMute` and `SoundSetMute`.
`SoundGetName` and `SoundGetInterface` are names of Windows (DirectSound)
and answer with the reason.

- **The device.** The empty name is the default sink of the session. A
  name is the sink of that name. `#1` is the first sink of the mixer, and
  `#N` is the Nth.
- **The volume is a percentage between 0 and 100**, the way v2 says it,
  and not a number between 0 and 1. The value is clamped to that range
  before it is set. A sign in front of the setting adjusts the volume
  instead of setting it.
- **The component of v2 is a part of a device** -- the master volume, the
  microphone -- and the box has one volume per device. A component that is
  not the master fails with the reason.
- **The tone is a file.** The box has no speaker of its own: `SoundBeep`
  writes a sine of 44.1 kHz mono with a fade at each end into a wave file
  under `/tmp`, plays it with `paplay`, and removes it. The script waits
  for the sound to finish, the way v2 waits.

On a host without the mixer the reason is the shell's:

```pwsh
'SoundSetVolume 50' | target\debug\ahk.exe
```

```json
{"elapsed_ms":59,"error":"the mixer refused: sh: line 0: pactl: not found","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

and `SoundBeep` fails at the player, after it has written its tone:

```
the mixer could not play "/tmp/ahk-tone-<millis>.wav": sh: line 0: paplay: not found
```

The name of the tone file carries the milliseconds since the epoch, so the
reason is different on every run. The file is removed even when the
player failed.

## The dialogs

The three dialogs of v2 are `InputBox`, `FileSelect` and `DirSelect`. The
box asks the user through `zenity`, which opens the dialog on the display
of the session.

The exit code of `zenity` says how the dialog ended, and the words are
the ones of v2:

| Exit code | `InputBox` `Result` |
| --- | --- |
| 0 | `OK` |
| 5 | `Timeout` |
| 1, or anything else in 0..=5 | `Cancel` |

An exit code outside 0..=5 is a failure of the dialog, and the reason is
what `zenity` wrote on its error output.

- **The options are read by the runtime** and not by the desktop: an
  option of an input box that v2 does not know is refused with
  `Invalid option.`, so the user is never shown a dialog that the runtime
  could not have understood. The ones that reach the desktop are the
  timeout, the password flag, and the position and size.
- **The filter of a file selection is translated.** v2 writes
  `Documents (*.txt; *.doc)`; zenity wants `Documents | *.txt *.doc`.
- **A file selection that may choose more than one file** is asked with
  `--multiple --separator=\n`, and the answer is an array of the paths.
- **A folder selection** is `--file-selection --directory`, with `--save`
  when the options ask for the new-folder button or the name field.

Measured on the box: zenity 4.2.2 answered the entry with the text of the
user, and it exited with 5 when the timeout of the options went by.

On a host without `zenity` the reason is its own, and it says what the
box would have needed:

```pwsh
'r := InputBox("Name?", "Title")' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":"zenity is not on this machine, and the box has no other dialog","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

## The files

The file capability is the one that works on every host, because it is
the filesystem and not the desktop. It is the only capability that a
script can rely on when it runs somewhere that is not the box.

| Group | Names |
| --- | --- |
| Reading and writing | `FileRead`, `FileAppend`, `FileOpen`, the `File` object, `FileDelete`, `FileCopy`, `FileMove` |
| Folders | `DirCreate`, `DirDelete`, `DirCopy`, `DirMove`, `Loop Files`, `Loop Read`, `Loop Parse` |
| Questions | `FileExist`, `DirExist`, `FileGetSize`, `FileGetAttrib`, `FileSetAttrib`, `FileGetTime`, `FileSetTime` |
| Ini files | `IniRead`, `IniWrite`, `IniDelete` |
| The working directory | `A_WorkingDir`, `A_InitialWorkingDir`, `SetWorkingDir` |
| The star | `FileAppend "text", "*"`, which writes to the report's `output` |

Three things are of the machine and not of the dialect:

- **An attribute the filesystem does not have is refused with the
  reason.** The read-only bit is the only one with a bit of its own on
  Linux; the others are the ordinary state of a file.
- **The times of a file are milliseconds since the Unix epoch**, so that
  the language can write them the way v2 does.
- **An ini is an ini of Windows**: `[section]`, `key=value`, and `;` for a
  comment. The files v2 writes are UTF-16 with a byte order mark and the
  ones this box writes are UTF-8; both are read.

`FileExist` answers with the attributes and not with a number -- `A` for a
file, `D` for a folder, the empty text when it is not there -- and it
works on any host:

```pwsh
'x := FileExist("C:\Windows\win.ini")' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":[],"steps":[{"detail":"x = A","elapsed_ms":0,"line":1,"ok":true,"op":"Assign"}],"variables":{"a_linenumber":1,"x":"A"},"warnings":[]}
```

The encoding of a file is another matter: the box has no code pages, so a
name of a code page -- `CP1252` -- is refused with the message v2 gives
for a name it does not know, and the bytes of a file are UTF-16 when the
encoding says so and UTF-8 otherwise.

## The network

`Download(url, file)` fetches a file and writes it on the disk, and the
box does it with `curl`. The answer is the empty text, because that is
what v2 answers.

`curl` is a program of the machine and not of the desktop, so this is the
second capability that works on a host that is not the box -- the first
being the files. Measured on this workstation:

```pwsh
'Download("https://example.com/x", "dl-probe.html")' | target\debug\ahk.exe
```

```json
{"elapsed_ms":66,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":[],"steps":[{"detail":"","elapsed_ms":66,"line":1,"ok":true,"op":"Download"}],"variables":{"a_linenumber":1},"warnings":[]}
```

The file was on the disk afterwards. The `detail` of the step is the empty
text: the answer of the name is nothing.

On a machine without `curl` the reason is its own:

```
curl is not on this machine, and the box has no other way to the network
```

Three things are v2's and not the tool's:

- **A file that is not there on the network is not a failure.** The
  server sends its page of the error, and that page is what is written.
- **A url that cannot be reached at all fails** with the kind of failure
  v2 gives, and the message carries the code of the machine.

## The device processes

The `Processes` capability is how a script starts a program and how it
sees the environment of the session.

| Name | What the box does |
| --- | --- |
| `Run` | Starts the command through `sh -c`, detached: standard input, output and error are `/dev/null`, and the script does not wait. |
| `RunWait` | Starts it through `sh -c` and waits for it, and answers the code and what it wrote. |
| `EnvGet` | Reads the environment of the runner. |
| `EnvSet` | Writes the environment of the runner. |
| `ProcessExist`, `ProcessClose` | The names of Windows that the runtime does not carry. |

`EnvSet` refuses a name that is empty and a name that carries an equal
sign, with the reason of the machine. The environment it writes is the
environment of the runner: the script runs on one thread, so nothing else
is reading it while the change happens.

`ProcessExist("notepad.exe")` fails with the general reason of a name of
Windows:

```json
{"elapsed_ms":0,"error":"this is a name of Windows, and this desktop is not Windows.","error_line":1,...}
```

## The hotkeys

The keyboard of the box *writes*; the hotkeys *read*. The device that is
read is the physical keyboard of the machine: `/dev/input/eventN` is
opened, the events are read, and the state of the keys is kept.

Three things follow:

- **The device of the box is skipped.** A hotkey that fires on the events
  of the keyboard of the box is a hotkey that fires on what the script
  types, and that is a loop. The virtual devices -- the one of the box
  among them -- are recognised by their `Sysfs` being under
  `/devices/virtual/` and their `Phys` being empty.
- **The device may be named.** `AHK_HOTKEY_DEVICE` says which one to
  read, and it may hold the path of the device or its name. A name is
  looked for among the devices of the machine, the virtual ones included.
  That is how the hotkeys are measured on the box, where there is nobody
  at the keyboard.
- **The runtime owns the registry and the machine owns the watcher.** The
  script says which function a hotkey runs; the machine says which key
  was pressed. The runtime asks between the statements, and only at the
  top level.

A key that is down and a key that is up are answered from the same state:
`GetKeyState` reads what the watcher saw.

On a host that is not Linux the hotkeys cannot be watched at all, and a
script that registers one fails before its first statement:

```
the hotkeys of this box watch a Linux device; this is not Linux
```

That is the one failure whose `error_line` is 0 and which still has steps
before it: the registration happens when the script loads.

## The native calls

`DllCall` calls a function of a library of the machine. It is split into
three pieces, and the split is the rule of the tree:

- **The marshalling is pure, and it is in the language crate**
  (`ahk::dll`): the names of the types, their sizes, the type of the
  answer, the convention, and the bytes of a value in the encoding its
  type asks for.
- **The call is I/O, and it is the `Native` capability**: the runtime
  hands the desktop a `NativeCall` and the desktop answers with a
  `NativeResult`.
- **The box answers it** (`ahk-box::native`): `LoadLibraryW` and
  `GetProcAddress` on Windows, `dlopen` and `dlsym` on Linux, and a
  trampoline that puts the arguments where the machine wants them.

The trampoline is inline assembly. There is no `libffi`: the shape of the
call is known when it is made -- how many arguments there are, and which
of them are floats -- so the assembly fills the registers and the answer
comes back in one of them.

### The two conventions of x64

- **On Windows the register of an argument is the one of its place.** The
  first four arguments go in RCX, RDX, R8 and R9, and the float of a place
  goes in the XMM of that place.
- **On Linux the register of an argument is the one of its class.** The
  integers go in RDI, RSI, RDX, RCX, R8 and R9, and the floats go in XMM0
  to XMM7 by their own count.

A function that takes a variable number of arguments is carried on Linux
and not on Windows: on Linux the number of the float registers rides in
AL, and the tree writes it there; on Windows the callee saves the float
registers in a room whose size only the callee knows, and the trampoline
cannot know that the callee is variadic.

### The types

`Str`, `WStr`, `AStr`, `Int`, `Int64`, `Short`, `Char`, `Float`, `Double`,
`Ptr` and `UPtr`, with a `U` in front of a whole number and a `*` or a `P`
at the end for the address of the value. The return type is `Int` when it
is left out, and the last parameter may be `CDecl` to say the convention.

The types are UTF-8 and UTF-16 in this tree: `AStr` is the text of the
machine and not the code page of Windows.

The `*` passes the address of a copy. The values of the tree have no
stable address, so the desktop allocates the block, passes its address,
and reads the bytes the callee left there back into the variable of the
caller. Measured on the binary of v2.0.19: `DllCall("kernel32\
QueryPerformanceCounter", "Int64*", &counter)` leaves the count in
`counter`, and so does the tree.

The answer on this workstation, with a real library of the machine:

```pwsh
'DllCall("kernel32.dll\GetTickCount", "UInt")' | target\debug\ahk.exe
```

```json
{"elapsed_ms":0,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":[],"steps":[{"detail":"61662390","elapsed_ms":0,"line":1,"ok":true,"op":"DllCall"}],"variables":{"a_linenumber":1},"warnings":[]}
```

That is the one capability that answers a value on a host that is not the
box, because the library is the one of the host.

## The raven, and the libraries of Windows

On the box the library is a `.so`, and the trampoline calls it. When the
library is one of Windows, the box has no Windows of its own -- and the
**raven** answers: a program that emulates a Windows process on Linux,
and inside it a guest program, the **helper**, which loads the library
and calls the function.

- **The routing is a rule of the box.** A name that carries a slash, or
  `.so`, is a library of this machine and the direct path answers it.
  Every other name is a library of Windows and the raven answers it. The
  backslash of a path of Windows is what sends a call there. On Windows
  the machine is the Windows, and every library is called directly: the
  raven is not there.
- **Two guests.** One of the sixty-four bits and one of the thirty-two.
  The guest of the sixty-four bits is asked first; the guest of the
  thirty-two is asked when the first does not have the library. A library
  of thirty-two bits is only called by a guest of thirty-two bits.
- **The protocol** is one line per call, tab separated, every value in
  hexadecimal: the library, the function, the type of the answer, the
  number of the arguments, and then the type and the value of every
  argument. The answer is `ok` and the value and the blocks that came
  back, or `error` and the reason.
- **The reasons are the ones of v2.0.19**: `Failed to load DLL.`,
  `Call to nonexistent function.`, `Invalid arg type.` and
  `Invalid return type.`.
- **The raven is a long-lived process.** It starts with the first library
  of Windows a script asks for and lives until the desktop is dropped. Its
  standard error goes nowhere -- the reason comes back in the protocol,
  and a pipe nobody reads fills up and stops the raven. The pipe of its
  standard input is held by the runner, so the kernel closes it when the
  runner dies and the raven stops with it; the emergency stop does not
  have to know about the raven.

The raven is named by `AHK_RAVEN` and `AHK_RAVEN_HELPER`, and a box that
does not have the two of them says so:

```
this box has no raven: the libraries of Windows cannot be called.
```

The helper is `crates/ahk-box/helper/helper.c`, built for a guest with no
C runtime, and it is only imported from five functions of the `kernel32`.
The kernel32 the raven serves is its own and not the one of Microsoft, so
a function the raven does not serve is `Call to nonexistent function.`
even when Windows has it.

## The machine variables

One hundred and thirty-one built-in variables of v2 are carried, and the
names that ask the machine are answered by the `Machine` capability.
Twenty-four of them answer with a value on the box; the rest say why they
cannot.

| Name | Where the answer comes from on the box |
| --- | --- |
| `A_UserName` | `USER` |
| `A_ComputerName` | The kernel's hostname, and `HOSTNAME` when the kernel will not answer |
| `A_ComSpec` | `SHELL` |
| `A_OSVersion` | `/proc/sys/kernel/osrelease` |
| `A_Language` | `LC_ALL`, then `LANG`, then `LANGUAGE`, then `/etc/locale.conf` |
| `A_IsAdmin` | The effective user id is zero |
| `A_AppData` | `XDG_DATA_HOME`, or `~/.local/share` |
| `A_AppDataCommon` | `/usr/share` |
| `A_Desktop` | `XDG_DESKTOP_DIR` of `user-dirs.dirs`, or `~/Desktop` |
| `A_StartMenu` | The folder of the applications of the user |
| `A_StartMenuCommon` | `/usr/share/applications` |
| `A_Startup` | `~/.config/autostart` |
| `A_StartupCommon` | `/etc/xdg/autostart` |
| `A_MyDocuments` | `XDG_DOCUMENTS_DIR`, or `~/Documents` |
| `A_Temp` | `TMPDIR`, or `/tmp` |

The names that have no answer on the box, each with its reason:

| Name | The reason |
| --- | --- |
| `A_DesktopCommon` | `the machine has no desktop for everybody: the desktop of the session is the one of the user.` |
| `A_Programs`, `A_ProgramsCommon` | `the machine has no programs folder: the start menu of the user is the folder of the applications.` |
| `A_WinDir`, `A_ProgramFiles` | `this is a name of Windows, and this desktop is not Windows.` |
| `A_TimeIdle`, `A_TimeIdlePhysical`, `A_TimeIdleKeyboard`, `A_TimeIdleMouse` | `the XDG portal of this box does not publish the Idle interface, and the session has no idle monitor.` |
| `A_ScreenDPI` | `this desktop does not know the scale of the output.` |
| `A_AhkPath` | `this runtime is not AutoHotkey, and it has no path of an executable.` |
| `A_ScriptHwnd` | `this runtime has no window of its own: it is a script that runs and ends.` |
| Any folder the machine does not have | `the machine has no such folder.` |

The names that the runtime knows without asking anybody answer on every
host: `A_AhkVersion` is `2.0.19`, `A_PtrSize`, `A_Is64bitOS`,
`A_EndChar`, `A_Space` and `A_Tab`.

`A_TickCount` is the clock of the run: the milliseconds since the run
began, from an arbitrary origin, never going backwards. It is not the
uptime of the machine, and it starts at zero.

```pwsh
@'
a := A_TickCount
Sleep 300
b := A_TickCount
MsgBox a " " b
'@ | target\debug\ahk.exe
```

```json
{"elapsed_ms":300,"error":null,"error_line":null,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":true,"output":["0 300"],"steps":[{"detail":"a = 0","elapsed_ms":0,"line":1,"ok":true,"op":"Assign"},{"detail":"300","elapsed_ms":300,"line":2,"ok":true,"op":"Sleep"},{"detail":"b = 300","elapsed_ms":0,"line":3,"ok":true,"op":"Assign"},{"detail":"0 300","elapsed_ms":0,"line":4,"ok":true,"op":"MsgBox"}],"variables":{"a":0,"a_linenumber":4,"b":300},"warnings":[]}
```

### The machine on a host that is not the box

Every name above is answered from the Linux machine, and a host that is
not Linux has none of those places to read. The answer is the reason of
the name, and the reason names the file or the variable that was not
there:

```pwsh
foreach ($v in 'A_UserName','A_ComputerName','A_IsAdmin','A_OSVersion','A_Language','A_Temp','A_ComSpec','A_AppData') {
  "x := $v" | target\debug\ahk.exe
}
```

| Name | The answer on a Windows host |
| --- | --- |
| `A_UserName` | `the session does not carry the name of the user: the environment has no USER.` |
| `A_ComputerName` | `the session does not carry the name of the computer: the environment has no HOSTNAME.` |
| `A_IsAdmin` | `the rights of an administrator are the effective user id, and this is not Linux.` |
| `A_OSVersion` | `the version of the operating system is the release of the kernel, and this is not Linux.` |
| `A_Language` | `the session does not carry the language: the environment has no LC_ALL, LANG or LANGUAGE, and /etc/locale.conf is not there.` |
| `A_ComSpec` | `the session does not carry the shell: the environment has no SHELL.` |
| `A_Temp` | `/tmp` -- the fallback of the folder, and a folder that does not exist on that host |
| `A_AppData` | `the machine has no such folder.` |

`A_Temp` is worth pointing at: the folder capability asks the machine
whether the folder exists, and `/tmp` is what `TMPDIR` falls back to
before that question. On a host with no `/tmp` the answer is still
`/tmp`, because none of the fallbacks is checked. That is a divergence
worth knowing and the only one of the table that answers a value instead
of a reason.

The full report of one of them:

```json
{"elapsed_ms":0,"error":"the session does not carry the name of the user: the environment has no USER.","error_line":1,"exit_code":null,"notes":["no uid on this platform; the session environment is unknown"],"ok":false,"output":[],"steps":[],"variables":{"a_linenumber":1},"warnings":[]}
```

The dates are the exception in the other direction: `A_Now`,
`A_NowUTC`, `A_WDay` and the rest come from the clock of the host, which
is the standard library, so they answer on every host.

## The environment variables

The backend and the runner read these variables. Every one of them is a
setting of the machine and not of the script: a script reads them with
`EnvGet`, and a host sets them before the run.

| Variable | Who reads it | What it does |
| --- | --- | --- |
| `YOKO_DESKTOP` | The runner | `x11` selects the X11 backend when Xlib and XTEST resolve. Anything else, or nothing, is the uinput/Wayland backend. |
| `YOKO_DISPLAY` | The X11 backend | The display to open. Falls back to `DISPLAY`, then `:0`. |
| `DISPLAY` | The X11 backend, the X11 window path, the folder of the user | The display, and `/tmp/.X11-unix/X0..X7` when there is none. |
| `WAYLAND_DISPLAY` | The desktop | Whether the session is Wayland. It picks the Wayland path for the windows and the clipboard over the X11 one. |
| `XDG_RUNTIME_DIR` | The desktop | Where the Wayland socket is looked for, when `WAYLAND_DISPLAY` is not set. `/run/user/<uid>` is what it is set to. |
| `HOME` | The folders, the pid file | The home of the user. |
| `USERPROFILE` | The pid file | The home, after `HOME`, on a host that has one. |
| `XDG_DATA_HOME` | The folders | `A_AppData` and the start menu of the user. |
| `XDG_CONFIG_HOME` | The folders | The startup folder and `user-dirs.dirs`. |
| `XDG_CACHE_HOME` | The runner | Where the pid file goes. |
| `TMPDIR` | The folders | `A_Temp`. |
| `USER`, `HOSTNAME`, `SHELL`, `LC_ALL`, `LANG`, `LANGUAGE` | The machine | The user, the computer, the shell and the language. |
| `TZ` | The clock of the host | The zone of the date. |
| `AHK_HOTKEY_DEVICE` | The hotkey watcher | The keyboard to read for the hotkeys: a path, or a name of a device. |
| `AHK_RAVEN` | The native calls | The program of the raven. |
| `AHK_RAVEN_HELPER` | The native calls | The guest of the sixty-four bits. Both this and `AHK_RAVEN` are needed for the raven to exist. |
| `AHK_RAVEN_HELPER32` | The native calls | The guest of the thirty-two bits. Optional: without it a library of thirty-two bits answers `Failed to load DLL.`. |

The four that change what a run does, and that are worth setting
deliberately:

- **`YOKO_DESKTOP=x11`** is the only way to pick the X11 backend, and it
  picks it only when Xlib and XTEST resolve on the machine. A host
  without them ignores the variable and stays on the uinput backend
  instead of failing at the first statement.
- **`YOKO_DISPLAY`** sets the display of the X11 backend without
  disturbing the session's own `DISPLAY`. That is why it exists beside
  `DISPLAY`.
- **`AHK_HOTKEY_DEVICE`** is what makes a hotkey measurable on a box
  where nobody is at the keyboard: point it at the keyboard the box
  writes, and a script that types with `Send` fires its own hotkey.
- **`AHK_RAVEN`, `AHK_RAVEN_HELPER` and `AHK_RAVEN_HELPER32`** are the
  three paths of the raven. A box that sets only some of them gets the
  reason, and not a wrong answer.

## The platforms, side by side

The names of the dialect are the same on every platform. The desktop is
the one thing that differs, and it differs a lot.

| Area | Linux + Wayland (the box) | Linux + X11 | Windows (this workstation) |
| --- | --- | --- | --- |
| Keyboard | uinput device, 4 s to appear | XTEST through Xlib, or uinput | `the keyboard is a Linux uinput device; this is not Linux` |
| Pointer | Absolute uinput device | XTEST, honours the server's focus | `grim failed: ...` -- the size of the screen comes first |
| Screen | `grim` | `XGetImage` through Xlib | `grim failed: sh: line 0: grim: not found` |
| Windows | `ext_foreign_toplevel_list_v1` + `zcosmic_toplevel_info_v1` | Xlib toplevels | The X11 path, which asks `xprop` and finds nothing |
| Clipboard | `wl-paste` / `wl-copy` | `xclip` or `xsel`, or the reason | The X11 path, which has neither |
| Sound | `pactl` / `paplay` | The same commands | `pactl: not found` |
| Dialogs | `zenity` | `zenity` | `zenity is not on this machine` |
| Files | The filesystem | The filesystem | The filesystem -- works |
| Network | `curl` | `curl` | `curl` -- works |
| Hotkeys | A watched `/dev/input/eventN` | The X11 keys of the server | `the hotkeys of this box watch a Linux device; this is not Linux` |
| Native calls | `dlopen` / `dlsym` | `dlopen` / `dlsym` | `LoadLibraryW` / `GetProcAddress` -- works |
| Libraries of Windows | The raven | The raven | Called directly; there is no raven |
| Machine | The Linux machine | The Linux machine | The reasons of the table above |
| Pictures | `png` + `grim` | `png` + `XGetImage` | `png` for a file, `grim` for the screen |

The backend is three implementations of the same fifteen traits:

- `LabDesktop`, in `ahk-box::desktop` and its sibling modules, is the
  uinput/Wayland one. It is the one the runner makes by default, and it
  compiles everywhere and works on Linux.
- `X11Desktop`, in `ahk-box::x11`, is the X11 one. It is compile-gated to
  Linux and FreeBSD, and it is selected by `YOKO_DESKTOP=x11` when the
  Xlib and XTEST symbols resolve. It is additive: nothing in
  `desktop.rs` changes its behaviour when it is chosen.
- There is no Windows backend. On Windows the runner still builds
  `LabDesktop`, and `LabDesktop` fails when it is used. The names are the
  names of v2 on every platform; the desktop of this tree is the Linux
  one.

FreeBSD is the other host the crate names: it has no `/dev/uinput`, so it
is the X11 backend and nothing else.

### What "fails" means, precisely

A capability that the host does not have does not answer `false` and does
not answer zero. It answers `Err` with a sentence, and the runtime turns
that into the `error` of the report. A script can catch it with `try`,
and a client reads it out of `error`.

The one thing that is not a failure on any host is a question whose
answer is genuinely "no": a window that is not there is `Absent` and
`WinExist` answers 0, and a file that is not there is the empty text. The
difference between "no" and "cannot tell" is the whole point of the
`WindowAnswer` enum, and it is the reason the focus check on a Wayland
session is a pixel and not a window title.

## Driving a real desktop

A script of this runtime does what a person would do:

- it types keys into whatever has the focus;
- it moves the real pointer and clicks real things;
- it can launch programs, and they stay launched after the script ends;
- it can write the clipboard, and the clipboard is shared with everything;
- it can set the volume of the machine;
- it can ask the user a question and wait for the answer.

Three habits make that safe.

**Read the script before you run it.** There is no sandbox and no
permission prompt: `Send "rm -rf"` typed into a terminal is a person
typing it. A script that came from somewhere else is a script to read
first.

**Keep the emergency stop in mind.** The run is stoppable from outside
through the pid file, and the pid file is described in
[the runner and the report](runner-and-report.md#the-pid-file-and-the-emergency-stop).
The `--timeout` flag and the step rail are the bounds inside the runtime,
and neither of them stops a statement that is already slow.

**Check the focus, do not assume it.** A script that types into the wrong
window is worse than a script that fails. The three readings that work
whatever the desktop is are `MouseGetPos`, `PixelGetColor` and
`PixelSearch`, and they are read from the frame the compositor draws.
`WinActive` and `WinExist` are the window-level questions, and on a
compositor that does not publish the focus the answer is a reason and not
a guess.

The first use of the keyboard and of the pointer costs four seconds each,
because the compositor has to adopt the device. A script that clicks once
and reads a pixel is a script that takes nine seconds, and that is not a
regression: it is the price of the device being a real device.

---

The names, with the state of each one, are `docs/v2-reference.md` and
`docs/v1-reference.md`, and `docs/v2-capability-report.md` groups them by
area. The report of a run, the flags and the guard rails are
[the runner and the report](runner-and-report.md). The windows of a
Wayland compositor are
[the Wayland toplevel enumeration](wayland-toplevel-enumeration.md).