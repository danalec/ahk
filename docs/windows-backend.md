# The Windows backend

What it would take to give this tree the desktop that AutoHotkey has on
Windows: the plan, the seam, the phases, and the decisions that are not
mine to take.

**The target is the modern Windows, and Windows 11 is the machine the
plan is written for.** The calls that the plan asks for are the ones of
today: per-monitor DPI awareness v2, the shell of the modern pickers,
the endpoint volume API, `SendInput`. What is left of the old Windows
is only what is still the right tool -- the window messages, the common
controls v6, the registry, the shell -- because that is what AutoHotkey
v2.0.31 itself is made of, and the behaviour of the tree is measured
against it.

The two floors that the toolchain does not give for free -- Windows XP
and Windows 7 -- have their own document, with the measurements and the
crate split they force: [`windows-legacy.md`](windows-legacy.md).

The measurements of this page were taken, on the Windows
host of the tree (rustc 1.99.0), against the source of AutoHotkey
**v2.0.31**, fetched from
`https://codeload.github.com/AutoHotkey/AutoHotkey/tar.gz/refs/tags/v2.0.31`
(tag `v2.0.31`, object `55c7ac269a3315cc8e27be74b9461163dddef82c`) and
unpacked in `../refs/AutoHotkey-2.0.31`. The source is the reference of
every "what does v2 do" question in this page; the citations are
`file:line` of that source, and of this tree.

- [Why](#why)
- [The reference](#the-reference)
- [Where the code lives](#where-the-code-lives)
- [The seam](#the-seam)
- [The seam that has to be cut](#the-seam-that-has-to-be-cut)
- [The map](#the-map)
  - [The machine](#the-machine)
  - [The processes](#the-processes)
  - [The screen and the pictures](#the-screen-and-the-pictures)
  - [The keyboard and the pointer](#the-keyboard-and-the-pointer)
  - [The windows](#the-windows)
  - [The hotkeys](#the-hotkeys)
  - [The clipboard](#the-clipboard)
  - [The files](#the-files)
  - [The registry](#the-registry)
  - [The COM objects](#the-com-objects)
  - [The sound](#the-sound)
  - [The drives](#the-drives)
  - [The GUI](#the-gui)
- [The design in Rust](#the-design-in-rust)
  - [The imports](#the-imports)
  - [The handles](#the-handles)
  - [The threads](#the-threads)
  - [The refusals](#the-refusals)
  - [The DPI](#the-dpi)
  - [The manifest](#the-manifest)
- [The phases](#the-phases)
- [The tests](#the-tests)
- [Windows 11, and the modern Windows](#windows-11-and-the-modern-windows)
- [The decisions](#the-decisions)

---

## Why

**The backend this page plans landed.** It is
`ahk-box::windows`, a module of the crate of the desktops, gated by
`#[cfg(windows)]` (`crates/ahk-box/src/lib.rs:35-36`), with about
8.900 lines in twenty-one files; the runner makes it on a Windows host
by the target alone (`crates/ahk-cli/src/main.rs:154-171`). The plan
below is the design it was built from, and it is kept as the record of
**why** the shape is the one it is -- the seam, the floors of every
call, and the decisions that were not the writer's to take. Where the
plan and the code disagree, the code is the truth and this page is
behind; the phases at the end are the plan, and the state of them is
the table of the phases that follows it.

The tree carries the whole surface of v2.0.31 -- 833 entries (785 distinct
names) in `crates/ahk/reference/ahk-v2.0.32.surface.tsv`, and the table of the
pending ones has no line of data -- and on Windows the desktop of the
tree used to answer almost everything with a refusal: the desktop was a
Linux desktop (`docs/desktop-backend.md:9-18`), and the runner on a
Windows host built `LabDesktop` and failed when it was used.

What worked on Windows then, exactly: the language, the files, the
native calls (`DllCall`, x86_64 only), the network through `curl`, and
the machine variables read from the environment. What refused: the
keyboard, the pointer, the screen, the windows, the clipboard, the
sound, the dialogs, the hotkeys, the drives, the processes (`sh -c`
does not exist), the registry (a file store, everywhere, Windows
included) and the GUI.

That was a large gap, and it was the gap that kept this tree from being
an AutoHotkey on the platform AutoHotkey is for. The plan below is the
way out of it, and it is the way out that was taken.

---

## The reference

AutoHotkey v2.0.31 is C++ and its own source is the best document
there is of what "the v2 on Windows" means. Three facts about it shape
the whole plan:

- **Its floor is Vista**: `source/stdafx.h:32` defines `_WIN32_WINNT
  0x0600`. It is a modern program that still speaks to the old
  messages, and the plan follows it there: the window messages, the
  common controls, the registry and the shell are still the right
  tools on Windows 11.
- **It carries a manifest** that a Rust binary has to carry too:
  `source/resources/AutoHotkey.exe.manifest` declares the side-by-side
  `Microsoft.Windows.Common-Controls` 6.0 (the common controls v6, the
  themed ones), the `supportedOS` GUIDs of Vista through Windows 10,
  `dpiAware` and `longPathAware`, and `requestedExecutionLevel
  asInvoker`. It is embedded by `source/resources/AutoHotkey.rc:35`.
- **It is not per-monitor aware**: its manifest says `<dpiAware>true`,
  which is the system-DPI model of the 2008. A Windows 11 backend
  should do better than the reference here, and [The DPI](#the-dpi)
  says how.

The tree's own measurements of the binary of v2.0.19 (in the comments of
the source and in `docs/`) are the measurements of the behaviour; the
source of 2.0.31 says where that behaviour comes from.

---

## Where the code lives

**Decision, taken and executed: a `windows` module inside `ahk-box`,
gated by `#[cfg(windows)]`.** It is `ahk-box::windows`
(`crates/ahk-box/src/windows/`), and the runner makes it on a Windows
host by the target alone.

The tree already answered the question of a second desktop once.
`X11Desktop` is a second implementation of the same fifteen traits
inside `ahk-box`, compiled only where Xlib can exist
(`crates/ahk-box/src/lib.rs:44-45`) and chosen at runtime by
`AHK_DESKTOP` plus an availability probe
(`crates/ahk-box/src/lib.rs:79-84`). `docs/desktop-backend.md` calls it
"the backend is three implementations of the same fifteen traits", and
the third one is the Windows.

The alternatives and why they lose:

| Option | Why not |
|---|---|
| a new crate `ahk-win` | a second publish and a second docs.rs page, for a backend that needs no new dependency of its own: `windows-sys` is declared for Windows alone, inside `ahk-box` (`crates/ahk-box/Cargo.toml:30-58`), and the crate of the desktops already holds the Windows-native parts that a new crate would have to duplicate or re-export: the `DllCall` loader and the trampoline (`native.rs:94-189`, `:343-384`), the file implementation (`files.rs`), the `sys` helpers (`sys.rs:171-188`) and the PNG codec (`picture/`). |
| `ahk-box::windows` (**chosen**) | the cost is honest and small: the crate's description says "the lab box", and on Windows it grows a second desktop. The Wayland dependency is already Linux-only (`crates/ahk-box/Cargo.toml:60-64`), the crate already compiles on macOS and Windows, and the CI matrix already has `windows-2022` (`.github/workflows/ci.yml:63`) -- the module is compiled and clippyed the day it lands. |
| a crate `ahk-native` for the shared Windows parts, and `ahk-win` for the desktop | the right shape **if** the backend grows past what a module should hold. The trigger to split: the Windows code passes ~8000 lines, or it needs its own release cadence, or a second consumer appears. **The first trigger has fired**: the module is at about 8.900 lines today. The split is: `ahk-native` (the FFI, the loader, the trampoline, the picture codec) and `ahk-win` (the desktop). It is not done, and the page of the legacy floors says what it would cost: [`windows-legacy.md`](windows-legacy.md). |

**The desktop is chosen by the target, not by a switch.** On a Windows
host the runner builds the Windows desktop and nothing else
(`crates/ahk-cli/src/main.rs:154-171`); there is no `AHK_DESKTOP=win32`,
and no `win32_available()`: the `LabDesktop` is not there to be picked
instead, because the Linux desktop does not exist on that host. The
`AHK_DESKTOP` variable and the `x11_available()` probe belong to the
Linux box, where there are three desktops to choose from.

---

## The seam

`ahk::desktop::Desktop` is the sum of fifteen traits and has no method
of its own (`crates/ahk/src/desktop/mod.rs:2692`): `Keyboard` (`:40`),
`Pointer` (`:115`), `Screen` (`:186`), `Windows` (`:513`), `Processes`
(`:1204`), `Files` (`:1375`), `Clipboard` (`:1703`), `Timers`
(`:1720`), `Hotkeys` (`:1728`), `Native` (`:1774`), `Pictures`
(`:1994`), `Network` (`:2026`), `Sound` (`:2118`), `Machine` (`:2280`),
`Dialogs` (`:2528`). Together they are 213 methods, and every one of
them has a default that refuses with a sentence ("this desktop cannot
..."). A Windows backend may therefore land **incrementally**: what is
not written yet keeps refusing, and the tree never answers `false`
where it cannot know.

The types a Windows implementation has to speak:

| Type | What it is | What it becomes on Windows |
|---|---|---|
| `WindowAnswer` (`desktop.rs:21-35`) | `Active` / `NotActive` / `Absent` / `Unknown(reason)` | unchanged -- it is exactly the "no vs cannot tell" rule (`docs/desktop-backend.md:953-965`) |
| `TitleMatch` (`desktop.rs:310-362`) | the four modes of v2, with `matches()` already pure | unchanged, and it already knows the `i)` regex prefix |
| `WindowRef { id: String, class }` (`desktop.rs:374-382`) | the id is a string because a Wayland token is not a number | the HWND as a decimal string; `WindowInfo.id` (`desktop.rs:2258-2268`) is the HWND as an `i64` |
| `WindowGeometry`, `WaitFor`, `Monitor`, `ControlAnswer`, `ControlFlag`, `ControlEdit` (`desktop.rs:388-503`) | the shapes of the answers | unchanged |
| `NativeCall` / `NativeResult` / `NativeFailure` / `NativeArg` / `NativeType` / `Convention` (`crates/ahk/src/dll.rs:15-220`) | the shape of a `DllCall` | unchanged -- and the Windows side of it **already works** |
| `Timers` (`desktop.rs:1594-1599`) | the desktop does not take the timers over | stays: the runtime runs the timers between the statements (`docs/desktop-backend.md:59`) |

What the tree has on Windows already, and does not need to be written
again:

- **`DllCall`**: the loader is `LoadLibraryW` / `GetModuleHandleW` /
  `GetProcAddress` declared by hand (`crates/ahk-box/src/native.rs:343-384`)
  and the trampoline is the x86_64 one (`native.rs:94-189`), with the
  failures of v2 spelled the same way ("Failed to load DLL.",
  "Call to nonexistent function." -- `native.rs:488`, `:494`).
- **The files**: the whole `Files` trait is portable `std` and works on
  Windows as it is (`crates/ahk-box/src/files.rs`).
- **The PNG codec**: `crates/ahk-box/src/picture/` is portable Rust, and
  the matching of `ImageSearch` is already split from the capture
  (`crates/ahk-box/src/pictures.rs:135-160`).

---

## The seam that has to be cut

Five places where the trait is narrower than v2, and the Windows
implementation cannot live inside the current shape. Each one touches
the fake desktops of the tests (18 files, `desktop.rs:613-617`), so
each one is a decision, not a detail.

| The gap | Where | What v2 does | What it costs to fix |
|---|---|---|---|
| the target of a window question is a **title**, and v2's target is a composite (`ahk_id`, `ahk_class`, `ahk_pid`, `ahk_exe`, plus the title) | `desktop.rs:512-673` | `WindowSearch::SetCriteria` parses the composite (`window.cpp:1349-1501`) and `IsMatch` compares title, class, pid, exe (`window.cpp:1518-1699`) | a `WindowCriteria` type in the trait, parsed from the `WinTitle` string by a pure function -- testable on any host, and it is the first thing to build |
| `DetectHiddenWindows` never reaches the desktop | `crates/ahk/src/builtins.rs:1410` carries it as a setting | `ScriptThreadSettings::DetectWindow` decides what a window is (`window.cpp:1229-1232`) | the setting travels with the question; the Linux side ignores it and says so |
| the state family (`WinClose`, `WinHide`, `WinMove`, `WinSetTitle`, ...) closes only **the script's own window** | `crates/ahk/src/builtins.rs:15210-15248` | `WinClose` posts `WM_CLOSE` to the found window (`window.cpp:379`) and escalates to `TerminateProcess` when it hangs (`script_autoit.cpp:1822-1839`) | a `window_action(&WindowInfo, action)` method, or the criteria widened to every member of the family |
| `Screen::pixel_search` has **no variation** | `desktop.rs:281-290` | the variation is a range per channel, with the `0x00FFFFFF` mask and the reversed rectangle (`lib/pixel.cpp:248-322`) | one parameter; the matching itself is portable and can stay in the dialect |
| `Processes::launch` / `run_and_wait` have **no window mode** | `desktop.rs:1098-1115` | "Min"/"Max"/"Hide" become `SW_MINIMIZE`/`SW_MAXIMIZE`/`SW_HIDE` (`script.h:1133-1142`) | one parameter, and the `CreateProcessW` path with `STARTF_USESHOWWINDOW` |
| `Hotkeys::fired_hotkey` is **pull-only**, and the keyboard state has one mode | `desktop.rs:1602-1626`, `:56-61` | the state is physical (`GetAsyncKeyState` or the hook), logical (`GetKeyState`) or toggle (the low bit), and they differ (`script2.cpp:3799-3843`) | `fired_events()` for the `InputHook` class, and a mode on the state methods |
| `Clipboard` is **text only** | `desktop.rs:1577-1587` | the clipboard holds formats; `ClipboardAll` captures them all (`clipboard.cpp:324-447`) | a binary channel, or the `ClipboardAll` class stays refused |

---

## The map

What v2.0.31 does on Windows, per area, and what this tree would have
to write. The last column says the floor of the call, so that a reader
knows what a Windows 11 machine has and what it does not.

### The machine

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| the version of the OS | `RtlGetVersion` resolved by `GetProcAddress(GetModuleHandle("ntdll.dll"))`, with `GetVersionExW` as the fallback (`os_version.cpp:91-101`) -- the reason is that `GetVersionEx` lies without a manifest | `RtlGetVersion`, `GetVersionExW` | NT4 | reads `/proc/sys/kernel/osrelease` and refuses (`ahk-box/src/machine.rs:96-108`) |
| the user, the computer | `GetUserName` / `GetComputerName` (`lib/vars.cpp:871-878`) | both | XP | the environment (`USERNAME`, `COMPUTERNAME`) |
| is the script an admin | **not** `CheckTokenMembership`: `OpenSCManager` + `LockServiceDatabase` (`script_autoit.cpp:149-170`) | advapi32 | XP | `geteuid() == 0` (`machine.rs:157-169`) |
| the idle time | the counters of the hook when it is installed, else `GetLastInputInfo` (`lib/vars.cpp:1441-1464`) | `GetLastInputInfo` | XP | refuses (`machine.rs:313-319`) |
| the folders | the CSIDL of the shell | `SHGetKnownFolderPath` | Vista | XDG (`machine.rs:176-281`) |
| is the OS 64 bits | `IsWow64Process` when the script is 32 bits (`util.cpp:1985-1997`) | kernel32 | XP SP2, x64 | the trait has no such method |

### The processes

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| `Run` | `CreateProcessW` with the raw command line, `STARTF_USESHOWWINDOW` and the mode of the option (`script.cpp:12353-12660`); `ShellExecuteExW` as the fallback for documents and verbs | `CreateProcessW`, `ShellExecuteExW` | NT4, XP | `sh -c` (`ahk-box/src/desktop.rs:2501-2508`) -- there is no `sh` on Windows |
| `RunWait` | poll `WaitForSingleObject(h, 0)` and then `GetExitCodeProcess`, as a **signed** int (`lib/wait.cpp:340-358`) | kernel32 | NT4 | `std::process` |
| the environment | `GetEnvironmentVariableW` / `SetEnvironmentVariableW`, process scope (`lib/env.cpp:28-78`) | kernel32 | XP | `std::env` (the same thing) |
| a process exists | the pid path with `OpenProcess`, the name path with `CreateToolhelp32Snapshot` (`script_autoit.cpp:1911-1981`) | `PROCESS_QUERY_LIMITED_INFORMATION` | Vista | `/proc` (`ahk-box/src/process.rs:78-79`) |
| the path of a process | `GetProcessImageFileName` + a `QueryDosDevice` loop to turn the device path into a drive letter (`lib/process.cpp:188-242`) | psapi | Win2000 | -- |

### The screen and the pictures

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| the colour of a pixel | `GetDC(NULL)` + `GetPixel`, or `CreateDC("DISPLAY")` in the "Alt" mode (`script_autoit.cpp:174-206`) | gdi32 | Win95 | `grim` and a PNG file (`ahk-box/src/desktop.rs:402-424`) |
| the search of a colour | one capture of the rectangle (`CreateCompatibleDC` + `BitBlt` + `GetDIBits`) and then the search in memory (`lib/pixel.cpp:137-362`) | gdi32 | Win95 | a PNG file and a match in Rust (`pictures.rs`) |
| the monitors | `EnumDisplayMonitors` + `GetMonitorInfo`, the primary by `MONITORINFOF_PRIMARY` (`lib/env.cpp:95-209`) | user32 | Win2000 | refuses |
| the size of the screen | `GetSystemMetrics(SM_CXSCREEN/SM_CYSCREEN)` -- the **primary** monitor (`lib/vars.cpp:1070-1073`) | user32 | Win95 | the size of the `grim` capture |
| a picture | `LoadImageW` for bmp/ico, then GDI+ loaded dynamically (`GdipCreateBitmapFromFile`), then `OleLoadPicture` (`util.cpp:2245-2419`) | `LoadImageW` (Win95), GDI+ (XP) | -- | the tree's own PNG decoder only (`ahk-box/src/picture/`) |

### The keyboard and the pointer

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| type a text | `SendInput` with `KEYEVENTF_UNICODE`, in one batch, with the hooks of AHK removed around the call (`keyboard_mouse.cpp:2605-2829`) | `SendInput` | **Win2000** | a uinput device of its own (`ahk-box/src/keyboard.rs:142-163`) |
| the state of a key | physical (`GetAsyncKeyState`, or the counters of the hook), logical (`GetKeyState`) or toggle (the low bit) (`script2.cpp:3799-3843`) | user32 | XP | the state of the watched device (`ahk-box/src/hotkeys.rs:488-507`) |
| the lock keys | read the low bit of `GetKeyState`, send a key up first if the key is held, then the toggle (`keyboard_mouse.cpp:3022-3073`) | user32 | XP | the LED of `/sys/class/leds` (`ahk-box/src/locks.rs:22-52`) |
| move and click | `SetCursorPos`, `mouse_event` / `SendInput` with `MOUSEEVENTF_ABSOLUTE` (`keyboard_mouse.cpp:2459-2555`) | user32 | Win95 / Win2000 | an absolute uinput pointer |

### The windows

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| the window in front | `GetForegroundWindow`, which may be `NULL` and that is an answer and not a failure (`window.cpp:430-469`, `:103-110`) | user32 | Win95 | the compositor (`wayland_*`) |
| the list of the windows | `EnumWindows` with the rule of visibility (`window.cpp:554-569`) | `EnumWindows` (Win95), `DwmGetWindowAttribute(DWMWA_CLOAKED)` (Vista) | -- | the compositor, or `xprop` |
| the place of a window | `GetWindowRect`, `GetClientRect` + `MapWindowPoints` (`lib/win.cpp:2152-2160`) | user32 | Win95 | `xwininfo` |
| wait for a window | a poll of the same predicate, with the pump of AHK between the polls (`lib/wait.cpp:162-290`) | -- | -- | a poll of the compositor |
| bring a window forward | `SetForegroundWindow` and, when Windows refuses (the foreground lock), the escalation of AHK: `AttachThreadInput`, a synthetic Alt, `BringWindowToTop` (`window.cpp:88-313`) | user32 | Win95 | -- |
| the controls of a window | `EnumChildWindows` and `WM_GETTEXT` (`window.cpp:573-670`) | user32 | Win95 | AT-SPI, over D-Bus (`ahk-box/src/atspi.rs`) |

### The hotkeys

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| register a hotkey | `RegisterHotKey` against the main window, and **fall back to the hook** when the registration fails, so that the hotkey overrides whoever has it (`hotkey.cpp:439-443`, `:1968`) | user32 | Win95 | a watched `/dev/input/eventN` (`ahk-box/src/hotkeys.rs:100-204`) |
| the hook | a **thread of its own** with a `GetMessage` pump, because a low-level hook only fires on the thread that installed it and only while that thread pumps (`hook.cpp:4033-4122`) | user32 | Win98 | -- |
| the decision of the hook | the hook never runs the script: it calls `CallNextHookEx` first and posts a message (`hook.cpp:2053-2061`); the callback of the hook must be fast or the OS removes it | user32 | Win98 | the watcher matches and pushes (`ahk-box/src/hotkeys.rs:230-312`) |
| the events AHK sent itself | every injected event carries a `dwExtraInfo` sentinel, and the hook skips those (`keyboard_mouse.cpp:97-131`, `hook.cpp:144-153`) | user32 | Win2000 | the watcher skips the virtual devices (`ahk-box/src/hotkeys.rs:16-24`) |

### The clipboard

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| read | `CF_UNICODETEXT` first, `CF_HDROP` (the files) after it, with the names joined by `\r\n` (`clipboard.cpp:23-140`) | user32 | Win95 | `wl-copy` / `xclip` (`ahk-box/src/desktop.rs:727-844`) |
| write | `GlobalAlloc` + `EmptyClipboard` + `SetClipboardData`; an **empty string leaves the clipboard truly empty** (`clipboard.cpp:144-257`, `:232-237`) | user32 | Win95 | the same commands |
| `ClipboardAll` | every format, skipping the "Link Source" family (`clipboard.cpp:324-447`) | user32 | Win95 | refused |

### The files

The trait is portable `std` and works on Windows today. What the Windows
backend has to add is the encoding that is Windows-shaped: `CP0` means
the code page of the machine (`GetACP`), and the tree hardcodes cp1252
(`crates/ahk/src/classes/file.rs:1031`); the BOM rules of v2 are the
BOM `FF FE` (UTF-16) and `EF BB BF` (UTF-8), and the rest is decoded
with `MultiByteToWideChar` (`lib/file.cpp:249-311`). The attributes,
the times (`GetFileTime` / `SetFileTime`) and the long paths
(`\\?\`, the manifest of AHK asks for `longPathAware`) are the same
work.

### The registry

| Question | v2 does | The calls | Floor | The tree today |
|---|---|---|---|---|
| read and write | `RegOpenKeyEx` + `RegQueryValueEx` (twice: the size and the data), `RegCreateKeyEx` + `RegSetValueEx`; the types are `REG_SZ`, `REG_EXPAND_SZ`, `REG_MULTI_SZ` (the `\n` of the script becomes a NUL), `REG_DWORD`, `REG_BINARY` (`script_registry.cpp:186-492`) | advapi32 | XP | a file under `$XDG_CONFIG_HOME` (`ahk-box/src/registry.rs:66-85`) |
| the 32/64 view | `SetRegView` sets `KEY_WOW64_32KEY` / `KEY_WOW64_64KEY`, and it is OR'd into every access (`script.h:988-998`) | advapi32 | Vista, x64 | -- |
| the error | `(N) message` from `FormatMessage` (`script_registry.cpp:84`, `:166`) | kernel32 | NT4 | two literals (`registry.rs:42-45`) |

**This is a behaviour change**: on Windows the tree would stop being a
file store and become the registry of the machine. It is the change the
owner has to confirm.

### The COM objects

The COM of the machine is carried now
(`crates/ahk-box/src/windows/com.rs`): the apartment of each thread, the
classes read from the registry of the machine, the late binding through
`IDispatch`, the events through `IConnectionPointContainer`, and the
shell of the machine -- the pickers of the files, the shortcuts, the
devices of the sound.

v2 does `OleInitialize` once (`script.cpp:368`), `CLSIDFromString` and
then `CLSIDFromProgID` (`script_com.cpp:36-42`), `CoCreateInstance`
(`:54`), and the late binding through `IDispatch::GetIDsOfNames` and
`IDispatch::Invoke` (`:1191`, `:1263`), with the events through
`IConnectionPointContainer` (`:1121-1131`). The error of v2 is the
HRESULT in hex and the message of the system
(`script_com.cpp:986-1001`).

**Measured on the machine of the tree**, three things
decided the shape of the module:

- **The apartment belongs to the thread.** A thread that makes an
  object of the COM without an apartment is refused by the machine with
  the number `0x800401F0`, and the refusal was the tree's: the
  apartment was taken once for the whole program, and the thread that
  made the object was another one. Each thread takes its own now
  (`com.rs:52-67`).
- **The class is read from the machine.** The class of the picker of the
  files is `HKEY_CLASSES_ROOT\CLSID\{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}`,
  and the machine makes it. The tree carried a byte swapped in that
  number -- the tenth byte was `0x1A` and not `0xA1` -- and the machine
  refused it with the number `0x80040154` for that reason alone. The
  number is read from the registry of the machine now
  (`com.rs:712-718`), and the walk of the classes is made once for the
  whole program (`com.rs:626-634`).
- **The table of the methods is the table of the interface.**
  `IFileDialog` is `ShObjIdl_core.h`: the three of `IUnknown`, the
  `Show` of `IModalWindow`, and then the methods of the interface in the
  order of the header. The tree wrote the title with the place of
  `GetFileName` (the sixteenth), read the choice of the user with the
  place of `SetFileNameLabel` (the nineteenth) and read the name of the
  item with the place of `BindToHandler` (the third). The right places
  are the seventeenth, the twentieth and the fifth
  (`com.rs:202-231`) -- and the picker was never shown, so the wrong
  places never answered anything.

**The pickers of the files are the COM of the machine.** `FileSelect`
and `DirSelect` are the modern shell picker of today -- `IFileDialog`,
with `FOS_PICKFOLDERS` for the folders -- and not the old
`SHBrowseForFolder` of v2 (`script_autoit.cpp:1050-1161`): the flags of
the picker come from the options of the script, the filter is the
`COMDLG_FILTERSPEC` of the SDK, and the answer is the path of the item
of the shell that the user chose (`dialogs.rs:296-398`). The user
closing the picker is not a failure: the machine says that with the
number `0x800704C7` (`ERROR_CANCELLED`), and the answer of the tree is
the empty path, which is the answer of v2
(`script2.cpp:1762-1776`).

### The sound

`SoundBeep` is `Beep` (`lib/sound.cpp:593-600`), and `SoundPlay` is
**`mciSendString`**, not `PlaySound` (`lib/sound.cpp:540-589`). The
volume, the mute and the name of the device are the endpoint API of
today -- `MMDeviceEnumerator`, `IAudioEndpointVolume`, `IPropertyStore`
(`lib/sound.cpp:31-536`), which is Vista and later and is what a
Windows 11 machine has.

### The drives

The list is the loop of `'A'..'Z'` with `GetDriveTypeW` (and **not**
`GetLogicalDriveStrings`: `lib/drive.cpp:206-238`), the space is
`GetDiskFreeSpaceExW`, the label and the serial are
`GetVolumeInformationW`, the ejection is `DeviceIoControl` with
`IOCTL_STORAGE_EJECT_MEDIA` on `\\.\X:`, and the lock is
`IOCTL_STORAGE_MEDIA_REMOVAL` (`lib/drive.cpp:94-202`).

### The GUI

The largest surface of the plan, and it is where the tree's Linux GUI
(a Python program with a toolkit, `ahk-box/src/gui.rs:83`) has to be
replaced by real windows.

| Piece | v2 does | The calls |
|---|---|---|
| the window | one class registered once (`RegisterClassExW`, `script_gui.cpp:2381`), `CreateWindowExW` (`:2386`), `SetWindowLongPtrW(GWLP_USERDATA)` to find the object again (`:2391`), and `DefDlgProcW` as the default proc | user32 |
| the controls | a `CreateWindowExW` per kind: `Button`, `ComboBox`, `ListBox`, `WC_LISTVIEW`, `WC_TREEVIEW`, `Edit`, `DATETIMEPICK_CLASS`, `MONTHCAL_CLASS`, `HOTKEY_CLASS`, `UPDOWN_CLASS`, `TRACKBAR_CLASS`, `PROGRESS_CLASS`, `WC_TABCONTROL`, and `AtlAxWin` for ActiveX (`script_gui.cpp:3774-4620`) | user32 and comctl32 |
| the value of a control | one funnel, `ControlGetContents`, with a message per kind: `WM_GETTEXT`, `BM_GETCHECK`, `CB_GETCURSEL`, `LVM_*`, `DTM_GETSYSTEMTIME`, `HKM_GETHOTKEY` (`script_gui.cpp:7866`) | user32 and comctl32 |
| the events | one window proc: `WM_COMMAND`, `WM_NOTIFY` (the `LVN_*`, `TVN_*`, `NM_DBLCLK` of each control), `WM_CONTEXTMENU`; the event is **posted**, never run inside the proc (`script_gui.cpp:8322-9297`) | user32 |
| the menus | `CreateMenuW` / `CreatePopupMenuW`, `InsertMenuItemW`, `TrackPopupMenuEx` (`script_menu.cpp:610-1220`) | user32 |
| the tray | `Shell_NotifyIconW` (`script.cpp:797`, `:837`) | shell32 |
| the dialogs | `MsgBox` is `MessageBoxW` (`window.cpp:896-976`); `InputBox` is AHK's own dialog resource (`lib/InputBox.cpp:98-99`); `FileSelect` is `IFileDialog`, which is the **modern shell picker** of today (`script2.cpp:1698-1769`); `DirSelect` is `SHBrowseForFolderW` (`script_autoit.cpp:1050-1161`) | user32, comdlg32, shell32 |
| the transparency | `SetLayeredWindowAttributes` with `LWA_ALPHA`, and the window gets the `WS_EX_LAYERED` before it (`lib/win.cpp:1891-1934`) | user32 |

The tree's `FileSelect` and `DirSelect` are the modern picker of the
shell for the two of them -- `IFileDialog`, with `FOS_PICKFOLDERS` for
the folders (`dialogs.rs:296-398`) -- and the two are the COM of the
machine, which is carried (`com.rs`).

The transparency of the window of the script is the alfa of a window
layered: the window gets the `WS_EX_LAYERED` and the machine is told
the alfa with the `SetLayeredWindowAttributes` (`styles.rs:201-219`).
The tip of the machine is a window of the tree, and it is a window
layered too: it is a popup of its own, it draws itself, and the
transparency of it is the same call (`tooltip.rs:115-210`).

The tree's `Windows` trait carries **fifty** `gui_*` methods
(`desktop.rs:683-1021`) plus the control family, and the Linux
implementation of them is a JSON pipe to a Python program with string
scraping (`ahk-box/src/desktop.rs:1735-1864`). The Windows
implementation would be real windows, and it would need the manifest
(the common controls v6 are a side-by-side assembly: without it every
control renders with the metrics of 1994).

---

## The design in Rust

### The imports

No new dependency is forced by the plan. The tree's convention is a
hand-written `unsafe extern "system"` block per area, and it is the
right one for the small surface: the loader of `DllCall` is three
functions (`crates/ahk-box/src/native.rs:348-352`), and
`crates/ahk-box/src/sys.rs:9-13` says why -- "a crate that carried the
whole of the C library for seven names would be a crate more to trust
for nothing".

The modern surface is a good deal larger than seven names: the DPI of
Windows 10 and 11, the endpoint volume, the shell picker, the COM
apartment, the storage IOCTLs. Two hundred declarations written by hand
is where the convention stops paying -- and the declarations of
`windows-sys` are exactly those declarations, in a crate that
Microsoft publishes and that brings no runtime code (it is
`no_std`-friendly and it is declarations, not a library). The decision
is in [The decisions](#the-decisions).

The layer would live in `crates/ahk-box/src/win/sys.rs`, split by the
DLL that owns the symbol, with `#[link(name = "kernel32")]` and its
siblings:

| DLL | What comes from it |
|---|---|
| `kernel32` | the processes, the files, the memory, the modules, the system information |
| `user32` | the windows, the messages, the input, the clipboard |
| `gdi32` | the capture, the pixels, the fonts, the icons |
| `advapi32` | the registry, the security, the service manager |
| `ole32`, `oleaut32` | the COM apartment and the variants |
| `shell32`, `shlwapi`, `comdlg32` | the shell, the folders, the file dialogs |

The structures (`OSVERSIONINFOW`, `STARTUPINFOW`, `PROCESS_INFORMATION`,
`KBDLLHOOKSTRUCT`, `BITMAPINFO`, `MONITORINFOEXW`, `LASTINPUTINFO`,
`PREVENT_MEDIA_REMOVAL`) are declared by hand with `repr(C)`, the way
the tree already declares its own `extern "C"` block in `sys.rs:171-188`.

### The handles

`HWND` is `*mut c_void`; the trait carries the handles as `u64`, which
is exactly the width of an HWND on both architectures, and the id of a
window is the HWND itself (`WindowInfo.id: i64`). The objects that the
tree has to keep alive -- the windows it created, the pictures it
loaded -- are a table inside the desktop (`Vec<GuiRecord>`,
`Vec<HBITMAP>`), the way the tree already keeps the loaded libraries
(`crates/ahk-box/src/native.rs:455-496`).

The `GWLP_USERDATA` trick of AHK (`script_gui.cpp:2391`) is the way to
find the Rust object from the window procedure, and it is safe in Rust
because the object lives in a `Box` that the table owns.

### The threads

The tree runs the script on one thread (the runner spawns it with a
128 MB stack, `crates/ahk-cli/src/main.rs:141-173`) and models one
pseudo-thread (`crates/ahk/src/classes/thread.rs`). A Windows backend
needs three kinds of thread, and AHK itself has two:

| Thread | Who owns it | Why |
|---|---|---|
| the interpreter | the runner's thread | every question of a foreign window is answered here: `EnumWindows`, `GetWindowText`, `GetWindowRect` need no pump |
| the hook | a thread of its own, with a `GetMessage` pump | **a low-level hook only fires on the thread that installed it, and only while that thread pumps** (`hook.cpp:4033-4122`). AHK installs the hooks there and asks the main thread for the changes (`PostThreadMessage`) |
| the UI (only in the GUI phase) | a thread of its own | the windows of the script are created and pumped there, and the events travel to the interpreter as the strings `gui_events()` already speaks (`crates/ahk/src/runtime.rs:15637-15680`) |

The hotkeys keep the contract the tree already has: the runtime owns
the registry and the machine owns the watcher, and the runtime asks
between the statements (`crates/ahk/src/hotkeys.rs:311`). The Windows
watcher would be the hook, and `fired_hotkey()` would drain the queue
that the hook fills -- the same shape as the drain of the Linux
watcher.

### The refusals

The rule of the tree is that a question the machine cannot answer fails
with the reason, and it never answers `false` and never answers zero
(`docs/desktop-backend.md:74-78`, `:953-965`). A Windows backend keeps
it in three ways:

- what is not written yet keeps the default of the trait, which is
  already a refusal with a sentence;
- a failure of the system is `Err` with the name of the primitive and
  the message of the system (`GetLastError` + `FormatMessage`), the way
  the tree formats it today (`ahk-box/src/locks.rs:56-63`);
- a genuine "no" is an answer and not a failure: a window that is not
  there is `Absent`, and a foreground window that is `NULL` is `Absent`
  too (`window.cpp:103-110`).

Two cases of the modern Windows deserve care:

- **`SetForegroundWindow` refuses legally** (the foreground lock), and
  the answer of the tree has to be `Unknown(reason)`, never `false`.
  The escalation of AHK (`AttachThreadInput`, the synthetic Alt,
  `window.cpp:88-313`) is behaviour-cloning that is partially
  unreliable by AHK's own comments.
- **The elevation (UIPI).** A script that is not elevated cannot send
  input to an elevated window, and cannot read its title. The answer
  is a refusal with the reason, never a silence.

**What is really left on Windows,.** These are the
refusals that the backend of Windows still answers, and there is
nothing else: every name of the surface that is not on this table is
carried.

| What refuses | Where | The reason it gives |
|---|---|---|
| the tree view (`TreeView`) | `windows/gui.rs:650-697` | the common controls of the machine do not answer the messages of them: the failure of the machine is the number 6 |
| the menu bar of the window of the script | `windows/desktop.rs:833` | the menu of the machine is made with the messages of it, and the tree does not speak them yet |
| the state of a control | `windows/desktop.rs:843` | the states of the machine are the ones of the windows of it |
| the events of the window of the script | `windows/desktop.rs:847` | the machine sends them to the window, and the tree does not read them yet |
| the drives (`eject`, `lock`, `unlock`, `retract`) | `windows/desktop.rs:169-186` | the drives of the machine are driven by the machine alone, and the tree does not drive them |
| the mail (`send_mail`) | `windows/desktop.rs:1277` | the mail is a thing of the network of the machine, and the tree does not carry it |

And one that is not a refusal: the timers stay with the runtime
(`Timers::takes_over_timers`), because the runtime is what runs them
between the statements of the script.

The refusals that are *not* of the machine are the ones of the seam:
the `Gui` class of the runtime answers that a window is a Windows
window and that a Wayland session has none
(`crates/ahk/src/classes/gui.rs:216-235`), and that is a refusal of
the language, not of the machine of Windows -- the machine of Windows
has the windows, and the backend of Windows makes them
(`windows/gui.rs`).

### The DPI

The tree has no DPI awareness today, and without it every coordinate is
a lie on a Windows 11 display of 125%, 150% or 175%: `GetSystemMetrics`,
`GetPixel` and `BitBlt` speak the scaled space of the process, and the
search of a colour finds nothing where the script thinks it should be.

The modern answer is **per-monitor awareness v2**:

- the process declares it with
  `SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)`
  (Windows 10 Creators Update and later), or with the manifest
  (`<dpiAwareness>PerMonitorV2</dpiAwareness>`), and the call is
  process-wide and irreversible, so it happens once, before any GDI
  call;
- the windows of the tree then get their DPI with `GetDpiForWindow`
  (Windows 10 1607) and the sizes with `GetSystemMetricsForDpi` and
  `AdjustWindowRectExForDpi` (also 1607), and the points are
  converted with `LogicalToPhysicalPointForPerMonitorDPI` (1607);
- `A_ScreenDPI` (`crates/ahk/src/variables.rs:278-283`, asked of
  `Screen::screen_dpi`, answered by
  `crates/ahk-box/src/windows/screen.rs`) is the number v2 gives: the
  dots per inch of the session, `GetDpiForSystem` put out whole -- 96 at
  one hundred per cent, 120 at one hundred and twenty-five, 192 at two
  hundred. The contract of the trait is the number of the script and not
  a factor and not a percentage, so that no backend can answer 9600 for a
  screen of 96 dots per inch, which is what a percentage did before;
- the scale of a monitor of `Monitor` is the same number: the effective
  DPI of the monitor from `GetDpiForMonitor`, 96 when the machine says
  nothing.

AHK itself is only system-DPI aware (`<dpiAware>true</dpiAware>` in its
manifest), which is the model of 2008; a Windows 11 backend should be
per-monitor v2 and should say so in the documents.

### The manifest

The manifest of AHK is the model
(`source/resources/AutoHotkey.exe.manifest`), with three changes for
Windows 11:

| The node | AHK | The backend |
|---|---|---|
| the common controls | `Microsoft.Windows.Common-Controls` 6.0 | the same -- without it every control renders with the metrics of 1994 |
| the DPI | `<dpiAware>true</dpiAware>` (the model of 2008) | `<dpiAwareness>PerMonitorV2</dpiAwareness>` in the `windowsSettings` of the 2016 |
| the long paths | `<ws2:longPathAware>true</ws2:longPathAware>` | the same |
| the system | the `supportedOS` GUIDs of Vista to Windows 10 | the same -- **Windows 11 answers to the GUID of Windows 10** (`{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}`), there is no GUID of its own |
| the privilege | `asInvoker` | the same |

The tree has no `build.rs` today, and the cheapest way to carry a
manifest without one is `.cargo/config.toml` with
`-C link-arg=/MANIFEST:EMBED` and `/MANIFESTINPUT:` per Windows
target.

---

## The phases

The plan as it was written, with the state of each phase beside it. The
state is the tree as it is: the module of the backend is
`crates/ahk-box/src/windows/`, and the column says where the phase
landed.

| # | What it delivers | Depends on | Days | The risk | The state |
|---|---|---|---|---|---|
| 0 | the plumbing: the `windows` module and its `sys.rs`, the selection in the runner, the docs | -- | 2-3 | low | **done** -- `ahk-box::windows`, gated by `#[cfg(windows)]`, and the runner makes it by the target |
| 1 | **the machine and the processes**: `RtlGetVersion`, the names, the admin test, the idle time, the folders (`SHGetKnownFolderPath`); `CreateProcessW` (with the modes), `ShellExecuteExW` as the fallback, the exit code, the environment, the process family | 0 | 5-7 | low | **done** -- `machine.rs`, `processes.rs` |
| 2 | **the screen core**: the per-monitor v2 declaration, one capture (`BitBlt` + `GetDIBits`), the pixel, the search with the variation, the monitors with their DPI; the pictures (`LoadImageW`, GDI+ loaded dynamically) reusing the matcher of the tree | 0 | 6-8 | the DPI decision, and it is process-wide | **done** -- `screen.rs`, `pictures.rs` |
| 3 | **the keyboard and the pointer**: `SendInput` with `KEYEVENTF_UNICODE` and the sentinels, the `keybd_event` fallback, the state of the keys (physical, logical, toggle), the lock keys, the pointer | 2 | 6-8 | the elevation (UIPI): a script that is not elevated cannot type into an elevated window, and the answer has to say so | **done** -- `keyboard.rs`, `pointer.rs` |
| 4 | **the windows**: the criteria parser (`ahk_id`, `ahk_class`, `ahk_pid`, `ahk_exe`), `EnumWindows`, the foreground, the geometry with the DPI of the window, the wait, the controls of a window, the state family | 0, 2 | 7-10 | **the riskiest piece**: the rules of the foreground and the escalation of AHK | **done** -- `desktop.rs`, `controls.rs`, `toplevel.rs` |
| 5 | **the clipboard and the dialogs**: `CF_UNICODETEXT` and `CF_HDROP`, the empty string against the empty clipboard, `MessageBoxW`, `InputBox` (a dialog built in memory), `FileSelect` with `IFileDialog` (the modern picker), `DirSelect`, the tray | 0 | 5-7 | low | **done** -- `clipboard.rs`, `dialogs.rs` |
| 6 | **the hotkeys**: `RegisterHotKey` with the fallback to the hook, the thread of the hook, the two procedures, the sentinels of the injected events, the `~` and the `$` and the custom combinations | 3, 4 | 7-10 | the hook is dropped by the OS if it is slow: the procedure has to be short and never to run the script | **done** -- `hotkeys.rs` |
| 7 | **the registry and the files**: the real registry (with the 32/64 view), the code page of the machine, the attributes, the times, the long paths; the refinements of `DllCall` (the suffix "A", the search of the standard modules, `A_LastError`) | 0 | 5-7 | the registry changes a documented behaviour | **done** -- `registry.rs`, `files.rs` |
| 8 | **the sound, the drives, the network and the COM**: `Beep` and `mciSendString`; the endpoint volume; the drives and the storage IOCTLs; `GetAdaptersAddresses`; the apartment of COM and the late binding | 0, 7 | 8-12 | the COM by hand (the vtable calls) | **done** -- `audio.rs`, `sound.rs`, `network.rs`, `com.rs` |
| 9 | **the GUI**: the class, the window, the fifty `gui_*` methods, the controls, the menus, the list and the tree views, the manifest | 4, 6 | 15-25 | the size, not the difficulty | **done** -- `gui.rs`; the windows of the machine are made, but the class `Gui` of the runtime still refuses (a refusal of the language, not of the machine -- [The refusals](#the-refusals)) |

The phases 0 to 8 are about **42 to 61 days** and they are the value: a
Windows desktop that types, clicks, reads the screen, answers the
questions about the windows, registers hotkeys, runs programs and reads
the registry -- the whole of what the Linux backend does, on the
platform v2 was written for. The phase 9 is worth it only if the owner
wants GUI scripts on Windows; until then the fifty methods keep
refusing, honestly.

**All ten phases are done.** The estimate of the plan was 42 to 61 days
for the phases 0 to 8 and 15 to 25 more for the GUI; the module is
about 8.900 lines and it is compiled and clippyed in CI for the machine
of thirty-two bits (`i686-pc-windows-msvc`,
`.github/workflows/ci.yml:141-163`), which is the proof that the backend
does not reach for anything that only the machine of sixty-four bits
has.

What the phases did **not** decide, and what is still open, is the
floor of the binary: the calls of the machine are asked for by name at
link time, and the newest of them is of Windows 10 1703. The legacy
floors -- Windows 7 and Windows XP -- need the calls newer than the
floor to be resolved at runtime, and the page that says so, with the
measurement of the import table, is [`windows-legacy.md`](windows-legacy.md).

---

## The tests

The tree has no Windows machine in CI, and its tests are the fake
desktops that record the calls (`crates/ahk/tests/builtins.rs:25`,
"A desktop that records what it was asked to do"). Three tiers:

1. **Pure, on any host**: the criteria parser of the window targets
   (the port of `SetCriteria`, `window.cpp:1349-1501`), the modes of
   the title match, the table of the virtual keys and the scan codes,
   the builder of the `INPUT` array of `SendInput`, the state machine
   of the lock keys, the mapping of the error of the system to a
   sentence, the range of the variation of the pixel search, the
   conversion between the logical and the physical points. These are
   the pieces where the behaviour of v2 is subtle, and they are the
   ones that must not be written twice.
2. **Against a recording desktop**: the same fake of the tree, with the
   Windows implementation behind an interface of free functions, so
   that the *translation* (the semantics of v2 into the calls) is
   testable on the Linux box, and the fake records the sequence of the
   calls (the `SendInput` array, the `EnumWindows` result).
3. **On a real Windows**: the integration tests behind
   `#[cfg(target_os = "windows")]`, ignored by default: type a text
   into a `notepad.exe` and read it back with `WM_GETTEXT`; register a
   hotkey and fire it with `SendInput` (the Windows twin of the
   `AHK_HOTKEY_DEVICE` trick of the box); the clipboard; the window of
   a program that the test itself started; the DPI of a monitor that
   the test scales. The CI of Windows has a desktop session; the
   CachyOS box cannot run these.

The release needs a smoke test on Windows too: today only the Linux
asset has one (`.github/workflows/release.yml:158-168`).

---

## Windows 11, and the modern Windows

The plan is written for Windows 11, and the modern Windows is what the
tree should aim at. What that means in practice:

**The floor.** The tree, with the standard library, runs on Windows 10
and later: the toolchain has required it since Rust 1.78 (May 2024),
and every Windows target of the toolchain says so. The binary of the
tree therefore runs on Windows 11 out of the box, and the manifest
declares the GUID of Windows 10 -- which is the GUID that Windows 11
answers to.

**The modern APIs the plan asks for** are the ones of Windows 10 and 11:
the per-monitor DPI of the 1607 and of the 1703, the shell picker of
`IFileDialog`, the endpoint volume of the sound, `GetTickCount64` where
a 64-bit tick is wanted, the `QueryFullProcessImageNameW` of the
processes (the psapi `GetProcessImageFileName` of v2 is the older
path), `SetWinEventHook` when the tree wants the windows to be told
rather than polled.

**What stays old on purpose.** The window messages, the common controls
v6, the registry, the GDI capture, `SHBrowseForFolder`, the MCI sound:
they are what AutoHotkey v2.0.31 itself is made of, they are what the
behaviour of the tree is measured against, and they are still the
supported way to write a program that behaves like every other program
of the desktop. The tree is not a Windows Store application and does
not need to be.

**The Windows 11 that is not Windows 10** is mostly the surface: the
rounded corners and the dark mode of the system (a window of the tree
follows the system by itself), the modern shell pickers (which the
plan uses), and the ARM64 machines -- Windows 11 on ARM is a first
class thing now, and it decides one question of the plan: the
`DllCall` trampoline of the tree is x86_64 only
(`crates/ahk-box/src/native.rs:315-331`). An `aarch64-pc-windows-msvc`
build needs a trampoline of its own, or an honest refusal; and the
x86_64 asset runs on an ARM64 Windows under the emulation of the
system, which is a fine answer for a first step.

**The two modern restrictions** that the tree has to live with, and that
the plan refuses to hide: the foreground lock (`SetForegroundWindow`
refuses, and the answer has to say so) and the elevation (a script
that is not elevated cannot drive an elevated window, and the answer
has to say that too).

---

## The decisions

Three of them are not mine.

1. **The GUI (the phase 9).** Fifty methods of the `Windows` trait, the
   common controls v6, the manifest, the menus, the list and the tree
   views: 15 to 25 days, and it is the piece that makes a Windows
   script look like a Windows script. Or the phases 0 to 8, and the
   GUI keeps refusing.
2. **The declarations.** The hand-written style of the tree, extended
   to two hundred names, or `windows-sys` -- the crate of Microsoft
   that is declarations and not a library, and that brings no runtime
   code with it. The first keeps the tree free of dependencies and
   makes every symbol auditable in the tree; the second writes the
   structures, the GUIDs and the COM interfaces once, and is what
   everybody else on Windows uses. The tree's own rule
   (`crates/ahk-box/src/sys.rs:9-13`) argues for the first; the size
   of the modern surface argues for the second.
3. **The registry.** Today it is a file store on every platform
   (`crates/ahk-box/src/registry.rs:66-85`), Windows included. The
   phase 7 makes it the real registry on Windows, and that is a change
   of documented behaviour.

And two questions that come with the target: the **architecture**
(the x86_64 asset of the release is the one that runs everywhere; the
ARM64 one needs a trampoline of its own, and the 32-bit one is a
different program), and the **floor of the assertion** (Windows 11 and
Windows 10, which is what the toolchain gives for free, or Windows 11
alone, which the tree should only promise once it can test it).

And one thing that is mine to say: the backend lives in `ahk-box` until
it outgrows it, and the tree keeps answering `false` to nothing.
