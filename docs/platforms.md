# The platforms

What runs where, what each platform gives and what it refuses, and the
floor of each. The measurements of this page were taken on the Windows
host of the tree and on the XP guest of the lab, and they are the ones
the claims below rest on.

The other document of the pair is
[`desktop-backend.md`](desktop-backend.md): it says what each capability
*answers*, this one says where the tree *runs* and what the floor of a
build is.

- [The three crates](#the-three-crates)
- [The map](#the-map)
- [The language crate is portable](#the-language-crate-is-portable)
- [Linux](#linux)
  - [Wayland](#wayland)
  - [X11](#x11)
  - [What Linux asks of the machine](#what-linux-asks-of-the-machine)
- [FreeBSD](#freebsd)
- [macOS](#macos)
- [Windows](#windows)
  - [What Windows answers](#what-windows-answers)
  - [The floor of the toolchain](#the-floor-of-the-toolchain)
  - [Windows XP](#windows-xp)
- [The targets](#the-targets)
- [The gate](#the-gate)

---

## The three crates

| Crate | `std` | What it is | Where it runs |
|---|---|---|---|
| `ahk` | optional -- it is `no_std` without the default feature | the language: lexer, AST, parser, runtime, the built-in table, the class library | anywhere Rust builds: a microcontroller, a WASM host, a 32-bit Windows, a Linux box, a FreeBSD box |
| `ahk-box` | required | the desktop backend: the uinput keyboard, the absolute pointer, `grim` captures, the Wayland toplevel enumeration, an X11 path, the Windows desktop of `user32`/`gdi32`/`advapi32`, the files, the native calls | Linux; FreeBSD for the X11 path; Windows for the `windows` module; on every other host it compiles and refuses, with the reason |
| `ahk-cli` | required | the runner: a script on stdin, a report on stdout | wherever `std` runs |

The dependency runs one way: `ahk-cli` -> `ahk-box` -> `ahk`. The
language crate knows nothing about the box: it is handed a `Desktop`
implementation, which is why the language can be tested against a fake
that records the calls.

---

## The map

The fifteen capabilities of a desktop, and what each platform answers.
The cell says *how*, and the long form of the refusals is in
[`desktop-backend.md`](desktop-backend.md).

| Capability | Linux + Wayland | Linux + X11 | FreeBSD | macOS | Windows |
|---|---|---|---|---|---|
| Keyboard | a uinput device of its own | XTEST through Xlib | refuses | refuses | `SendInput` of user32 |
| Pointer | an absolute uinput device | XTEST, honours the focus | refuses | refuses | `SendInput` of user32 |
| Screen | `grim` | `XGetImage` through Xlib | the X11 way, if Xlib is there | refuses | `BitBlt` and `GetDIBits` |
| Windows (of the session) | `ext_foreign_toplevel_list_v1` + `zcosmic_toplevel_info_v1` | the Xlib toplevel list | the Xlib toplevel list | refuses | `EnumWindows` and the criteria of user32 |
| Clipboard | `wl-paste` / `wl-copy` | `xclip` or `xsel` | `xclip` or `xsel` | refuses | the clipboard of user32 |
| Sound | `pactl` / `paplay` | `pactl` / `paplay` | refuses: there is no `pactl` | refuses: there is no `pactl` | the endpoint volume of WASAPI (Vista); `PlaySoundW` for the play; on XP the mixer of winmm answers |
| Dialogs | `zenity` | `zenity` | refuses: there is no `zenity` | refuses: there is no `zenity` | the `IFileDialog` of the shell (Vista); on XP the common dialogs of comdlg32 answer |
| Hotkeys | a watched `/dev/input/eventN` | the keys of the X server | refuses | refuses | `RegisterHotKey` |
| Processes | `sh -c` | `sh -c` | `sh -c` | `sh -c` | the processes of kernel32 |
| Files | the filesystem | the filesystem | the filesystem | the filesystem | the filesystem -- works |
| Network | `curl` | `curl` | `curl` | `curl` | `curl` -- works (Windows 10 1803 and later) |
| Native calls (`DllCall`) | `dlopen` / `dlsym` | `dlopen` / `dlsym` | refuses: there is no loader | refuses: there is no loader | `LoadLibraryW` / `GetProcAddress` -- works |
| Machine | the Linux machine | the Linux machine | the environment | the Unix machine | the environment (`USERNAME`, `COMPUTERNAME`) |
| Pictures | `png` + `grim` | `png` + `XGetImage` | `png` | `png` | `png` for a file, the capture for the screen |
| Timers | modelled, not real | modelled | modelled | modelled | modelled |
| GUI and menus | the GTK helper of the window | the GTK helper of the window | refuses | refuses | the windows and menus of user32 |
| Object model | D-Bus (`DBusObject`, through `busctl`) | D-Bus (`DBusObject`, through `busctl`) | refuses | refuses | COM (`ComObject`) |
| Messaging / events | the events of the helper | X11 ClientMessage (`XSendEvent`) + the event queue | the X11 way, if Xlib is there | refuses | `SendMessage` / `PostMessage` / `OnMessage` |
| Memory of the process | `/proc/self/mem` | `/proc/self/mem` | `/proc/self/mem` | refuses | `ReadProcessMemory` |

The refusals are the point of the design: a question the machine cannot
answer fails with the reason, and it never answers `false` and never
answers zero. So a script on a host that cannot do the thing says so,
and a client reads the reason out of `error` of the report.

Two lines of the table are worth reading twice, because they are gaps
and not refusals by design:

- **`DllCall` refuses on macOS and FreeBSD** even though both have
  `dlopen`. The loader of the crate has a Windows branch
  (`LoadLibraryW`) and a Linux branch (`dlopen`) and nothing else
  (`crates/ahk-box/src/native.rs:343-445`); and the trampoline that
  makes the call exists for `x86_64` on those two systems only
  (`native.rs:315-331`). Everywhere else the answer is *"this desktop
  cannot make a native call on this machine"* (`native.rs:508-512`).
- **The registry is the registry of the machine on Windows** -- the
  hives of `advapi32` (`crates/ahk-box/src/windows/machine.rs`). On
  every other platform it is a file store: a directory under
  `$XDG_CONFIG_HOME` (`crates/ahk-box/src/registry.rs:66-85`).

---

## The language crate is portable

The language crate does no I/O of its own, and it needs no operating
system at all. Turn the `std` feature off and the crate is
`#![no_std]` (`crates/ahk/src/lib.rs:1`): the whole of its `std` surface
is a handful of lines, each behind `#[cfg(feature = "std")]` -- the clock
(`SystemTime::now`, `thread::sleep`), the timezone (`env::var("TZ")` and
the TZif file), and the `std::error::Error` impls. The rest is `core`
and `alloc`.

The clock is the one thing the host has to supply: the `Clock` trait
(`crates/ahk/src/runtime.rs:1113-1129`) is where the time and the pause
come from, and `Runtime::with_clock` is the entry point of a host with
no `std`.

The targets that build today, measured:

```sh
cargo build -p ahk --no-default-features --target thumbv7em-none-eabihf
cargo build -p ahk --no-default-features --target x86_64-unknown-none
cargo build -p ahk --no-default-features --target i686-pc-windows-gnu
cargo build -p ahk --no-default-features --target i686-pc-windows-msvc
```

The last two are the interesting ones: a 32-bit Windows, where the
operating system of the machine may be much older than the Windows 10
the toolchain asks for -- see [Windows XP](#windows-xp).

---

## Linux

Linux is the platform the tree was written for. The backend drives the
machine with devices of its own and with the tools of the session, and
both paths are measured in `docs/desktop-backend.md`.

### Wayland

The default path (`LabDesktop`).

- The keyboard and the pointer are **uinput devices of its own**: the
  crate opens `/dev/uinput`, which is group-writable by `input`
  (`crates/ahk-box/src/keyboard.rs:24,125,311`,
  `crates/ahk-box/src/pointer.rs:17,188,539`). The user has to be in
  the `input` group, and the answer without it says so.
- The screen is read with `grim`, and the size of the screen is read
  from a capture (`crates/ahk-box/src/desktop.rs:402-424`).
- The windows of the session are asked of the compositor, with the
  protocols `ext_foreign_toplevel_list_v1` and
  `zcosmic_toplevel_info_v1` (`crates/ahk-box/src/wayland/linux.rs:27-34`).
  A compositor that publishes neither is answered with the reason.
- The clipboard is `wl-paste` and `wl-copy`
  (`crates/ahk-box/src/desktop.rs:758-780`).

### X11

The second path, additive: nothing in `desktop.rs` changes when it is
chosen. The caller selects it with `AHK_DESKTOP=x11` and asks
`ahk_box::x11_available()` first
(`crates/ahk-box/src/lib.rs:64-73`, `crates/ahk-cli/src/main.rs:144-152`).

- No X11 library is linked at build time: `libX11.so.6` and
  `libXtst.so.6` are `dlopen`ed at runtime and the symbols are looked
  up by name (`crates/ahk-box/src/x11.rs:16,48-53`). A machine without
  them is answered with the reason, and the runner falls back to the
  uinput path.
- The keyboard and the pointer go through XTEST; the screen is read
  with `XGetImage`; the clipboard is `xclip` or `xsel`
  (`x11.rs:2229`).

### What Linux asks of the machine

The backend shells out to the tools of the session rather than linking
them. The list, with where each is called:

| Tool | For |
|---|---|
| `grim` | the capture of the screen |
| `wl-paste`, `wl-copy` | the clipboard of a Wayland session |
| `xclip`, `xsel` | the clipboard of an X11 session |
| `pactl`, `paplay` | the volume, the mute, the sound |
| `zenity` | the input box, the file and folder pickers |
| `notify-send` | the notification of the tray |
| `df`, `lsblk`, `udisksctl`, `eject`, `e2label`, `mlabel`, `ntfslabel` | the drives, their labels, their ejection |
| `busctl` | the accessibility bus (AT-SPI) |
| `python3` | the GUI helper, with a GTK toolkit |
| `curl` | the download of a file |
| `sh` | every command that is a shell line |

The GUI of the tree is a Python program with a toolkit
(`crates/ahk-box/src/gui.rs:83`): a window of the tree is a window of
`python3`, and a host without Python has no window.

---

## FreeBSD

FreeBSD is the other host the crate names, and it is the X11 backend and
nothing else: it has no `/dev/uinput`, so the keyboard, the pointer and
the hotkeys refuse (`docs/desktop-backend.md:950-951`). The X11 module
is compiled for it (`crates/ahk-box/src/lib.rs:35-36`), so the parts
that need Xlib work there when Xlib is there. `DllCall` refuses: the
loader of the crate has no Unix branch that is not Linux.

The tree builds for it (`x86_64-unknown-freebsd` is one of the targets
of the lab box); the release workflow builds no asset for it.

---

## macOS

macOS is a host the tree compiles for and the release workflow builds
assets for, and a host where the desktop is almost empty:

- the files, the network (`curl`) and the machine (`getuid`,
  `gethostname` -- the Unix branch of `crates/ahk-box/src/sys.rs:46-157`)
  work;
- the keyboard, the pointer, the screen, the windows, the clipboard,
  the sound and the hotkeys refuse -- they are the Linux devices of the
  box, and there is no macOS backend;
- `DllCall` refuses: the loader has no macOS branch, even though
  `dlopen` is there.

The assets of the release workflow are `x86_64-macos` (built on
`macos-15-intel`) and `aarch64-macos` (built on `macos-15`).

---

## Windows

Windows is a host the tree compiles for, the release workflow builds an
asset for, and has a backend for: `ahk-box::windows`, the
`WindowsDesktop` (`crates/ahk-box/src/windows/`), behind
`#[cfg(windows)]`. The runner picks it by the target alone, with no
switch (`crates/ahk-cli/src/main.rs:220-223`). The design of it is in
[`windows-backend.md`](windows-backend.md); the behaviour is measured
against AutoHotkey v2.0.31.

### What Windows answers

- The **keyboard and the pointer** are `SendInput` of user32.
- The **screen** is `BitBlt` and `GetDIBits`; the **pictures** are the
  `png` of the tree over that capture.
- The **windows of the session** are `EnumWindows` and the criteria of
  user32, with the controls, the styles and the toplevel enumeration.
- The **clipboard** is the clipboard of user32.
- The **hotkeys** are `RegisterHotKey`, on a thread of their own with
  a window of the machine; the wait of the script is an event of
  kernel32 (`CreateEventW`), not the `Condvar` of the `std`, which is
  of Vista.
- The **sound**: the volume and the mute are the endpoint volume of
  WASAPI (`MMDeviceEnumerator` + `IAudioEndpointVolume`, Vista); the
  play is `PlaySoundW` of winmm, which is of Windows 2000. On XP the
  class of Vista is not there (measured on the XP guest of the lab),
  and the mixer API of winmm
  answers in its place -- the way of the v1 (`windows/mixer.rs`).
- The **dialogs**: the pickers are the `IFileDialog` of the shell
  (Vista). On XP the class is not there (measured on the XP guest of
  the lab), and the common
  dialogs of comdlg32 answer in their place (`GetOpenFileNameW`,
  `GetSaveFileNameW`, and `SHBrowseForFolderW` for the folders --
  `windows/picker.rs`).
- The **processes** are the processes of kernel32; the **registry**
  is the registry of `advapi32`; the **trash** is `SHFileOperationW`.
- The **files** work, and the creation time of a file can be set
  (`crates/ahk-box/src/files.rs:881-885`).
- The **native calls** work: `LoadLibraryW` / `GetProcAddress`, and the
  trampoline for `x86_64` (`crates/ahk-box/src/native.rs:315-384`).
- The **network** works, through `curl` -- which is `curl.exe` on
  Windows 10 1803 and later, and nothing before it.
- The **machine** is read from the environment (`USERNAME`,
  `COMPUTERNAME`).
- The **GUI** is the common controls v6 of `comctl32`
  (`InitCommonControlsEx`), loaded at runtime.

The calls newer than XP are asked by name at runtime
(`windows/sys.rs:the_symbol_of`), with the fallback of the older API
or the refusal with the reason -- the rule of
[`windows-legacy.md`](windows-legacy.md). That is what lets one body of
code serve the modern Windows and the XP floor.

### The floor of the toolchain

`ahk-box` and `ahk-cli` use the standard library, so their floor is the
floor of the toolchain, not of this tree -- unless the toolchain is
emended, which is what the XP build does.

**Windows 10 is the minimum of the shipped `std`.** Rust 1.78 raised
the requirement of the tier-1 Windows targets to Windows 10:

> Rust 1.78 has increased its minimum requirement to Windows 10 for the
> following targets: `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`,
> `x86_64-pc-windows-gnu`, `i686-pc-windows-gnu`,
> `x86_64-pc-windows-gnullvm`, `i686-pc-windows-gnullvm`.

-- the compatibility notes of [Rust 1.78.0](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0/).
Before that the floor was Windows 7. The reason was the target tier
policy, not an API of `std`: the old targets could no longer be tested
in CI and were not supported by their vendor
([the announcement](https://blog.rust-lang.org/2024/02/26/Windows-7/)).

The Windows asset of the release workflow is `x86_64-pc-windows-msvc`,
built with the stable toolchain, so that runner runs on Windows 10 and
later. Its floor is in fact Windows 10 version 1703: the newest call
the backend imports by name is `SetProcessDpiAwarenessContext`
(`windows/screen.rs`).

**The tier-3 `win7` targets are the way below that floor.**
`x86_64-win7-windows-msvc`, `i686-win7-windows-msvc`,
`x86_64-win7-windows-gnu` and `i686-win7-windows-gnu` exist since Rust
1.76. They are tier 3, so the standard library is not shipped for them:
it is built with `-Z build-std` on a nightly toolchain. The `std` of
those targets carries the Windows 7 baseline, which is still Vista+
APIs -- fine for Windows 7, not for XP. The tree uses the GNU flavour
of them, with the `std` emended for the era of XP
([Windows XP](#windows-xp) below). A Windows 7 machine is above that
floor by construction; it is not measured, because the lab has no
Windows 7 machine.

### Windows XP

The short answer: **the runner of this tree runs on Windows XP SP3, in
both dialects.** The way is the tier-3 target
`i686-win7-windows-gnu` with a **patched standard library**
(`scripts/xp/patch_std.py` emends the `std` for the era of XP), built
with `-Z build-std=std,core,alloc,panic_abort` on a nightly. The
binary of 2,68 MB runs the v1 and the v2 on the XP guest of the lab,
and the report comes back through the floppy
(`scripts/xp/README.md`, "O ahk.exe no XP, nos dois dialectos").

The floor is measured, not quoted: the probe of `scripts/xp/floor`
asked the XP machine for every symbol the binaries import -- **352
asked, 352 present, none missing** (`scripts/xp/floor-xp-sp3.txt`).
The gate (`scripts/xp/gate.py`) reads that measurement and fails on a
symbol the machine does not have, and on a symbol nobody asked about.

What is proven, and what is not:

| The machine | The state |
|---|---|
| Windows XP SP3, 32 bits | **measured**: the `hello`, the `ahk.exe` in v1 and in v2 run on the guest of the lab (5.1 build 2600) |
| Windows XP x64 / Server 2003 x64 | **measured**: the binary of `x86_64-win7-windows-gnu` runs the v1 and the v2 on the XP x64 SP2 guest of the lab (5.2 build 3790); the floor of the x64 is 345 symbols asked, 345 present, none missing (`scripts/xp/floor-xp-x64.txt`) |
| Vista, 7, 8, 8.1 | **not measured**: no machine of the lab. By construction the import table is XP-clean and the newer calls resolve at runtime, so the floor is below them -- but the floor of this tree is a measurement, not a quotation |
| Windows 10 / 11 | **measured on the host** (build 26300): the same binaries run the v1 and the v2, and the report is the same |

The two COM classes of Vista are measured too: `CLSID_FileOpenDialog`
(the picker) and `CLSID_MMDeviceEnumerator` (the volume of WASAPI) are
**not registered** on XP -- the answer is `0x80040154`. The script is not
refused there: the common dialogs of comdlg32 and the mixer of winmm
answer in their place, the way of the v1. `CLSID_ShellLink` is registered.
The play of the sound (`PlaySoundW`) is of Windows 2000 and stays.

The measurements and the rule of the three floors are in
[windows-legacy.md](windows-legacy.md); the tools of the lab are in
[`scripts/xp/README.md`](../scripts/xp/README.md).

#### Why the `std` is patched

The loader of Windows resolves the imports of a PE before the first
instruction of the program runs. A symbol that XP's `kernel32.dll` does
not have is not a runtime error: it is a refusal to start. The import
table of the modern MSVC build carries:

| Import | Where it comes from | First Windows |
|---|---|---|
| `api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`, `WakeByAddressSingle`, `WakeByAddressAll`) | `std`, the thread parking | Windows 8 (Windows 7 with the platform update) |
| `GetSystemTimePreciseAsFileTime` | `std`, the clock | Windows 8 |
| `IsThreadAFiber` | `std` | Windows 8 |
| `SetThreadStackGuarantee` | `std`, the stack overflow guard | Vista SP2 |
| `GetFileInformationByHandleEx` | `std`, the file metadata | Vista |
| `GetFinalPathNameByHandleW` | `std`, `fs::canonicalize` | Vista |
| `CompareStringOrdinal` | `std` | Vista |
| `SwitchToThread`, `CancelIo` | `std` | Vista |
| `CreateWaitableTimerExW`, `WaitForSingleObjectEx` | `std` | Windows 7 |
| `api-ms-win-crt-*.dll` and `VCRUNTIME140.dll` | the Universal CRT | Windows 8 api sets; the UCRT does not exist on XP at all |

XP has none of the api sets, no Universal CRT, no SRWLOCK and no FLS.
A binary that imports them stops at the loader, before `main`, with
*"the procedure entry point ... could not be located"*.

Pinning an older toolchain does not fix it. Rust 1.77 -- the last
version before the 1.78 floor -- still imports `AcquireSRWLockExclusive`,
`TryAcquireSRWLockExclusive`, `ReleaseSRWLockExclusive`,
`InitializeSListHead`, `SetThreadStackGuarantee` and, with a static
CRT, `InitializeCriticalSectionEx` and the `Fls*` family: all of them
are Vista+ APIs, and all of them are static imports. Rust deleted the
delayed binding that made XP possible in **1.51.0**
([PR #81250](https://github.com/rust-lang/rust/pull/81250): *"The
minimum supported Windows version is now Windows 7. Windows XP and
Windows Vista are no longer supported"*), and the XP tier-3 target
became `no_std`-only in 1.61.0
([PR #94499](https://github.com/rust-lang/rust/pull/94499)). An
XP-capable `std` existed from 1.2 to 1.50, and it never came back --
so the tree emends the `std` of today instead of pinning an old one.

#### A binary without `std` can run on XP

The toolchain itself is not the obstacle: a `no_std` Rust program -- a
global allocator over `HeapAlloc`, a `#[panic_handler]`, and three
calls to `kernel32` -- built for `i686-pc-windows-gnu` with the MinGW
of the `msvcrt` flavour, imports:

```
KERNEL32.dll:  DeleteCriticalSection, EnterCriticalSection, GetLastError,
                GetProcessHeap, GetStdHandle, GetTickCount, HeapAlloc,
                HeapFree, InitializeCriticalSection, LeaveCriticalSection,
                SetUnhandledExceptionFilter, Sleep, TlsGetValue,
                VirtualProtect, VirtualQuery, WriteFile
msvcrt.dll:    malloc, free, memcpy, memset, exit, _initterm, ...
```

Every one of those is on Windows XP SP3. There is no `api-ms-win-*`
import, no Universal CRT, no Vista+ symbol; the PE subsystem version
is 4.0, which the loader of XP accepts. The probes of the lab
(`scripts/xp/probe`, `scripts/xp/floor`) are binaries of this kind,
and they run on the guest. The decision record of the Rust compiler
team when it dropped XP said it in as many words
([compiler-team#378](https://github.com/rust-lang/compiler-team/issues/378)):
*"Removing official XP support would not stop anyone from compiling to XP
(minus the std) so long as LLVM and their linker still support XP
targets."*

The MinGW matters here: Rust's `*-windows-gnu` target links `msvcrt`,
the CRT that Windows itself has shipped since NT, and `msvcrt.dll` is on
XP. The other CRT -- the Universal one -- would break the binary with
its `api-ms-win-crt-*` imports. The toolchain of the XP build therefore
has to be the `msvcrt` flavour of MinGW, not the `ucrt` flavour
(llvm-mingw defaults to it).

Two smaller floors of the same kind: the `i686` target enables SSE2 by
default, and the minimum CPU of Windows XP is a Pentium 233, which has
no SSE2 -- `-C target-feature=-sse2` buys the older machines, and any
real XP machine of the last twenty years is a Pentium III or later. And
a 32-bit XP is the target that matters: the 64-bit XP is a rarity, and
the `x86_64` Windows targets are the ones that carry the tier-1
instruction set of Rust 1.78.

#### How the XP build is made

The pieces:

1. **The Windows backend** (`ahk-box::windows`), measured against
   v2.0.31 -- the same body the XP floor uses.
2. **The calls newer than XP** (seven of them: `SHGetKnownFolderPath`,
   `SHCreateItemFromParsingName`, `GetDpiForMonitor`, `GetDpiForSystem`,
   `SetProcessDpiAwarenessContext`, `GetUserDefaultLocaleName`,
   `QueryFullProcessImageNameW`) are questions to the name of the
   function at runtime (`the_symbol_of`), or calls of the era of XP.
   The wait of the hotkeys is the `CreateEventW` of kernel32, not the
   `Condvar` of the `std`, which is of Vista.
3. **The `std` is emended, not dropped**: `scripts/xp/patch_std.py`
   rewrites the Vista+ calls of the `std` (the FLS family, the SRW
   locks and the mutex, the `Condvar`, `GetFileInformationByHandleEx`,
   the trio of `ProcThreadAttributeList`, `CreateSymbolicLinkW`, ...)
   over the APIs of Windows 2000. The `-Z build-std` recompiles it;
   the build folder of the target has to be erased for the emend to
   enter the binary, because the fingerprint of the cargo does not
   change when the `std` changes.
4. **The fallbacks of the era of XP**: the picker of Vista
   (`IFileDialog`) and the volume of WASAPI (`MMDeviceEnumerator`)
   are not registered on XP, so the common dialogs of comdlg32
   (`windows/picker.rs`) and the mixer API of winmm (`windows/mixer.rs`)
   answer in their place -- the way of the v1.
5. **The gate and the floor**: `scripts/xp/gate.py` over the measured
   `scripts/xp/floor-xp-sp3.txt` and `scripts/xp/floor-xp-x64.txt`,
   and the probe that asks the two machines.

The build of the laboratory and of the release is one script, which
pins the nightly, emends the `std`, builds and gates:

```sh
# the thirty-two bits
powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/release.ps1
# the sixty-four bits
powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/release.ps1 \
  -Target x86_64-win7-windows-gnu
```

The script downloads the MinGW of the `msvcrt` flavour when the machine
has none (the winlibs of thirty-two bits, the `w64devkit` of sixty-four
with the empty stub of `libgcc_eh`), and `-Mingw` names a toolchain that
is already there. The nightly is pinned (`nightly-2026-11-13`): the
emends anchor to the text of the `std`, and a floating nightly breaks
them.

#### What the XP runner of this tree does not give

- **The proof of the middle Windows.** Vista, 7, 8 and 8.1 are above
  the XP floor by construction -- the import table is XP-clean and the
  newer calls resolve at runtime -- but the lab has no machine of them,
  and the floor of this tree is a measurement, not a quotation.
- **The CPU below the Pentium III.** The build keeps SSE2 (the default
  of the `i686` target), so a machine whose CPU has no SSE2 -- the
  Pentium 233 that is the minimum of XP -- does not run it. Any real XP
  machine of the last twenty years is a Pentium III or later.

The picker and the volume, which the first version of this page listed
as refusals of XP, are carried now: the common dialogs of comdlg32 and
the mixer API of winmm answer on the XP machines, measured in both
dialects. The release workflow builds the XP binary as an asset
(`i686-windows-xp`, `x86_64-windows-xp`), and the CI gates it
([windows-legacy.md](windows-legacy.md), "The release").

If the goal is only "a machine that still runs Windows XP must run AHK
scripts", the cheaper answer is still the interpreter that already
ships: the AutoHotkey v1.1.37.02 binary, whose import table is XP-clean
as well. The case for this tree on XP is the case for the tree: one
interpreter, two dialects, the surface documented in thirteen languages,
and a backend that is a real desktop.

---

## The targets

| Target | `ahk` (no_std) | `ahk-box` + `ahk-cli` | In the release workflow |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | yes | yes | yes, glibc 2.31 and later |
| `aarch64-unknown-linux-gnu` | yes | yes | yes, glibc 2.31 and later |
| `x86_64-pc-windows-msvc` | yes | yes | yes, Windows 10 and later |
| `i686-pc-windows-msvc` | yes | builds, needs a Windows SDK | no |
| `i686-pc-windows-gnu` | yes | builds, needs the MinGW | no |
| `i686-win7-windows-gnu` | yes | builds: nightly, `-Z build-std`, the `std` patched for XP; the gate passes | yes -- `i686-windows-xp` |
| `x86_64-win7-windows-gnu` | yes | builds: the same, with the stub of `libgcc_eh`; the gate passes | yes -- `x86_64-windows-xp` |
| `x86_64-apple-darwin` | yes | yes | yes |
| `aarch64-apple-darwin` | yes | yes | yes |
| `x86_64-unknown-freebsd` | yes | yes | no |
| `thumbv7em-none-eabihf` | yes | no | no |
| `x86_64-unknown-none` | yes | no | no |

The Linux assets are built inside `rust:1.98-slim-bullseye` and not on
the runner of the workflow, because glibc 2.31 is the floor the release
notes promise and the image of the runner (Ubuntu 24.04, glibc 2.39)
would make a binary that does not run on the older systems
(`.github/workflows/release.yml:44-61`).

---

## The gate

The gate of the tree builds on Ubuntu 24.04, on Windows 2022 and on
macOS 15, and builds the language crate for the two bare-metal targets
beside them (`thumbv7em-none-eabihf` and `x86_64-unknown-none`). It
runs `clippy`, the suite, and the parity of the surface of the three
dialects (`.github/workflows/ci.yml`).

The gate of the XP floor is the other one: `scripts/xp/gate.py` reads
the import table of a PE and fails when the binary asks for a symbol
the XP machine does not have -- or for a symbol nobody asked the
machine about. The authority is the measurement
(`scripts/xp/floor-xp-sp3.txt` and `scripts/xp/floor-xp-x64.txt`), not
a list of quotations: a floor that was not measured is a floor that is
guessed. The CI runs it over the two XP builds (the `windows-xp` job
of `.github/workflows/ci.yml`), through `scripts/xp/release.ps1`.

The release workflow builds seven assets: `x86_64-windows`,
`x86_64-linux-gnu`, `aarch64-linux-gnu`, `x86_64-macos`,
`aarch64-macos`, and the two of the XP floor, `i686-windows-xp` and
`x86_64-windows-xp`. The project is not released yet; the workflow
builds the assets and a tag publishes them.
