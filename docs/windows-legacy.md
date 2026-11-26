# Windows XP and Windows 7

The two floors that the toolchain does not give for free, and the way
the three Windows floors -- the modern one of
[`windows-backend.md`](windows-backend.md), Windows 7 and Windows XP --
live in the tree.

The Windows desktop is a module of `ahk-box` (`ahk-box::windows`), and
the crate keeps the standard library. The XP floor is reached without
splitting the crate and without dropping the `std`: the tier-3 target
`i686-win7-windows-gnu` carries a standard library **emended for the
era of XP** (`scripts/xp/patch_std.py`), and the calls of the backend
newer than XP are resolved at runtime. The runner runs on Windows XP
SP3 in both dialects, measured on the machine of the lab; the floor is
423 symbols asked, 423 present, none missing.

- [The three floors](#the-three-floors)
- [The shape of the tree](#the-shape-of-the-tree)
- [The reference of each floor](#the-reference-of-each-floor)
  - [Windows XP is the v1](#windows-xp-is-the-v1)
  - [Windows 7 is the v2](#windows-7-is-the-v2)
- [The measurements](#the-measurements)
  - [The last AHK v1 is XP-clean](#the-last-ahk-v1-is-xp-clean)
  - [The modern build is not](#the-modern-build-is-not)
  - [A binary without `std` can be](#a-binary-without-std-can-be)
  - [The toolchain of the floor](#the-toolchain-of-the-floor)
  - [The machine of the lab](#the-machine-of-the-lab)
  - [The floor of the two builds](#the-floor-of-the-two-builds)
- [The rule that lets the three live together](#the-rule-that-lets-the-three-live-together)
- [The XP backend](#the-xp-backend)
- [The Windows 7 backend](#the-windows-7-backend)
- [The middle Windows: Vista, 8, 8.1 and the rest](#the-middle-windows-vista-8-81-and-the-rest)
- [The gate](#the-gate)
- [The release](#the-release)
- [The decisions](#the-decisions)

---

## The three floors

| Floor | The binary | The toolchain | The CRT | The reference |
|---|---|---|---|---|
| the modern Windows (Windows 11 first) | `x86_64-pc-windows-msvc` | stable, as it is | the Universal CRT | AutoHotkey **v2.0.31** |
| Windows 7 | `x86_64-win7-windows-msvc` (tier 3) | **nightly** with `-Z build-std`, because the tier-3 target has no shipped standard library | the Universal CRT downlevel (KB2999226) or a static CRT | AutoHotkey **v2.0.31** (its floor is Vista) |
| Windows XP (SP3) | `i686-win7-windows-gnu` (and `x86_64-win7-windows-gnu`), with the standard library **emended** by `scripts/xp/patch_std.py` | **nightly**, with `-Z build-std=std,core,alloc,panic_abort` | `msvcrt.dll`, the one Windows has since NT | AutoHotkey **v1.1.37.02**, the last v1 -- and the tree runs the v2 there too, measured |

Three floors, three different reasons to be careful: the modern one is
about the behaviour, the Windows 7 one is about the *standard library*,
and the XP one is about the standard library **and** the calls of the
backend.

The measurements of this page were taken on the Windows host of the
tree (rustc 1.99.0), against:

- the source of AutoHotkey **v2.0.31** (`../refs/AutoHotkey-2.0.31`,
  the tag `v2.0.31`);
- the binary of the last **v1**, AutoHotkey **1.1.37.02**
  (`../refs/ahk-v1.1.37.02`, the release `v1.1.37.02` of the
  AutoHotkey repository);
- the runner of this tree (`target/debug/ahk.exe` and
  `target/release/ahk.exe`, rustc 1.99.0), read with
  `llvm-objdump -p`;
- and the XP SP3 guest of the lab, through the probes of
  `scripts/xp/`.

---

## The shape of the tree

**The Windows desktop is a module of `ahk-box`, not a crate of its
own.** The tree calls it `ahk-box::windows`, it lives in
`crates/ahk-box/src/windows/`, and it is behind the `windows`
configuration (`crates/ahk-box/src/lib.rs:35-36`):

```rust
#[cfg(windows)]
pub mod windows;
```

The module carries the machine (`machine.rs`), the processes
(`processes.rs`), the screen (`screen.rs`), the keyboard and the
pointer (`keyboard.rs`, `pointer.rs`), the windows and their controls
(`desktop.rs`, `controls.rs`, `toplevel.rs`), the clipboard
(`clipboard.rs`), the files (`files.rs`), the COM objects and the
apartment (`com.rs`), the sound (`audio.rs`, `sound.rs`), the dialogs
and the pickers (`dialogs.rs`), the GUI (`gui.rs`), the network
(`network.rs`), the pictures (`pictures.rs`), the trash (`trash.rs`)
and the state of the session (`state.rs`).

This is the shape that [`windows-backend.md`](windows-backend.md)
chose, and the choice was made on purpose: a new crate would be a
second publish and a second docs.rs page, for a backend that adds no
dependency of its own to the workspace beyond `windows-sys`, which is
declared for Windows alone (`crates/ahk-box/Cargo.toml:30-58`).

**The runner picks it by the target, not by a switch.** On a Windows
host the desktop of the run is the Windows one, with no environment
variable and no probe (`crates/ahk-cli/src/main.rs:220-223`):

```rust
#[cfg(windows)]
let mut windows_desktop = ahk_box::windows::WindowsDesktop::new();
#[cfg(windows)]
let desktop: &mut dyn ahk::desktop::Desktop = &mut windows_desktop;
```

The `AHK_DESKTOP` variable and the `x11_available()` probe belong to
the Linux box, where there are three desktops to choose from; on
Windows there is one, and the refusal of the Linux one is not reachable
at all.

**The consequences for the legacy floors are three:**

1. **The crate of the desktops requires the standard library** --
   `ahk-box` spawns processes, reads pipes on threads of their own,
   and uses the filesystem and the environment throughout. The XP
   build does not drop the `std`: it **emends** it for the era of XP
   (`scripts/xp/patch_std.py`) and rebuilds it with `-Z build-std`.
   The binary of it runs on the XP machine of the lab. The crate
   keeps the `std`; the `std` is what changes.
2. **The Windows API layer is inside `ahk-box`, and it stays there.**
   The calls of the Windows are declared by `windows-sys`, inside the
   crate of the desktops. The patched `std` makes a split of the crate
   unnecessary: the XP binary links the same crate, with the same
   layer, and the runner, the file layer and the desktop come along as
   they are.
3. **The imports of the modern build are static, and they are
   modern.** The MSVC build asks for the calls of the machine by name,
   at link time, and its floor is decided by the **newest** call it
   imports (Windows 10 1703). The XP build is the other way: the seven
   calls newer than XP are asked to the DLL at runtime
   (`windows/sys.rs:the_symbol_of`), or are calls of the era of XP, and
   its floor is the measured floor of XP SP3.

---

## The reference of each floor

### Windows XP is the v1

AutoHotkey v2 **never ran on Windows XP**: the source of v2.0.31
defines `_WIN32_WINNT 0x0600` (`source/stdafx.h:32`), which is Vista,
and the v2 needs the Vista APIs it uses. The last AutoHotkey that runs
on XP is therefore a **v1**, and the last v1 is **1.1.37.02**.

So the reference of the XP floor is the **v1 dialect**, measured
against the binary of the v1.1.37.02 -- and the tree already carries
that measurement: `crates/ahk/reference/ahk-v1.1.37.02.surface.tsv`
(the table of the names of the v1, 18.9 KB), the pending one beside it,
and the probe that made them (`ahk-v1.1.37.02.probe.py`). The tree
speaks the v1 already (`--dialect=v1`, `crates/ahk/src/v1/`).

**The XP backend is not a subset of the v2 work, it is the same tree
with the other dialect.** The modern backend of `windows-backend.md`
is measured against the v2; the XP one is measured against the v1; and
the two share the language crate, the desktop traits and every line of
the Windows API layer. The dialects of the tree are its own, and both
run on the XP guest, measured -- the floor of the machine does not
choose between them.

What the v1 has and the v2 does not, and that the XP floor inherits:
the sound of the volume through the **mixer API** (the v1.1.37.02
binary imports `mixerOpen`, `mixerGetLineInfoW`, `mixerSetControlDetails`
-- the endpoint volume API of the v2 does not exist on XP), the
controls of the common controls **v5** (the v1 binary imports
`InitCommonControlsEx` and carries no manifest), and the absence of
DPI awareness (the v1 binary imports no DPI call at all: the Windows of
2008 scaled the whole screen for the process).

### Windows 7 is the v2

The v2.0.31 runs on Windows 7: its floor is Vista. The Windows 7
backend is the modern backend with a different standard library, and it
is measured against the same v2.0.31.

---

## The measurements

### The last AHK v1 is XP-clean

Measured on `AutoHotkeyU32.exe` of the v1.1.37.02 (the last v1), by
reading the import table of the PE:

```
PE32  subsystem 2 (GUI)  subsystem version 5.0
WSOCK32.dll  WINMM.dll  VERSION.dll  COMCTL32.dll  PSAPI.DLL
WININET.dll  KERNEL32.dll  USER32.dll  GDI32.dll  COMDLG32.dll
ADVAPI32.dll  SHELL32.dll  ole32.dll  OLEAUT32.dll
```

Fourteen DLLs, all of them of the classic name -- **no `api-ms-win-*`
api set, no `msvcrt.dll`, no `VCRUNTIME140.dll`**: the CRT is linked
inside the binary. And the symbols are of the XP era: the kernel32
imports are `InitializeCriticalSection` and **not**
`InitializeCriticalSectionEx`, `GetVersionExW` and **not**
`RtlGetVersion`, and there is no `GetTickCount64`, no
`GetSystemTimePreciseAsFileTime`, no SRWLOCK and no FLS.

The 64-bit binary of the same release is the same story with the
subsystem version 5.2 (`PE32+`, the same fourteen DLLs, no api set):
it runs on the 64-bit XP.

That is the shape of the target: **a binary of the classic DLLs, of the
XP-era symbols, of a subsystem version that XP accepts, and of a CRT
that XP has.**

### The modern build is not

Measured on `target/debug/ahk.exe` (rustc 1.99.0,
`x86_64-pc-windows-msvc`), the import table carries:

| Import | Comes from | First Windows |
|---|---|---|
| `api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`, `WakeByAddressSingle`, `WakeByAddressAll`) | `std`, the thread parking | Windows 8 |
| `api-ms-win-shcore-scaling-l1-1-1.dll` (`GetDpiForMonitor`) | `windows-sys`, the DPI of the monitor | Windows 8.1 |
| `api-ms-win-crt-*.dll` and `VCRUNTIME140.dll` | the Universal CRT | Windows 8 api sets; the UCRT does not exist on XP |
| `GetSystemTimePreciseAsFileTime` | `std`, the clock | Windows 8 |
| `IsThreadAFiber` | `std` | Windows 8 |
| `SetThreadStackGuarantee` | `std`, the stack overflow guard | Vista SP2 |
| `GetFileInformationByHandleEx` | `std`, the file metadata | Vista |
| `GetFinalPathNameByHandleW` | `std`, `fs::canonicalize` | Vista |
| `CompareStringOrdinal` | `std` | Vista |
| `SwitchToThread`, `CancelIo` | `std` | Vista |
| `CreateWaitableTimerExW`, `WaitForSingleObjectEx` | `std` | Windows 7 |
| `ntdll.dll` (`RtlGetVersion`, `NtReadFile`, `NtWriteFile`, `RtlCaptureContext`) | `std` and the backend | NT4 |

The loader of Windows resolves the imports **before** the first
instruction of the program, so a symbol that the `kernel32.dll` of XP
does not have is not a runtime error: it is a refusal to start. This
binary stops at the loader, with *"the procedure entry point ... could
not be located"*.

Pinning an older toolchain does not fix it: the `std` of Rust 1.77 --
the last version before the 1.78 floor -- still imports
`AcquireSRWLockExclusive`, `InitializeSListHead`,
`SetThreadStackGuarantee` and, with a static CRT, `InitializeCriticalSectionEx`
and the `Fls*` family. Rust deleted the delayed binding that made XP
possible in **1.51.0** ([PR #81250](https://github.com/rust-lang/rust/pull/81250):
*"The minimum supported Windows version is now Windows 7. Windows XP and
Windows Vista are no longer supported"*), and the XP tier-3 target
became `no_std`-only in 1.61.0
([PR #94499](https://github.com/rust-lang/rust/pull/94499)). The
`std` of XP existed from 1.2 to 1.50, and it never came back. The way
of the tree is therefore the emend of the `std`, not the pin of an
old toolchain.

### A binary without `std` can be

Measured on a probe written for the question -- a global allocator over
`HeapAlloc`, a `#[panic_handler]`, and three calls to `kernel32` --
built with rustc 1.99.0 for `i686-pc-windows-gnu`, with the MinGW of
w64devkit:

```
PE32  subsystem 3 (console)  subsystem version 4.0
KERNEL32.dll:  DeleteCriticalSection, EnterCriticalSection, GetLastError,
                GetProcessHeap, GetStdHandle, GetTickCount, HeapAlloc,
                HeapFree, InitializeCriticalSection, LeaveCriticalSection,
                SetUnhandledExceptionFilter, Sleep, TlsGetValue,
                VirtualProtect, VirtualQuery, WriteFile
msvcrt.dll:    malloc, free, memcpy, memset, exit, _initterm, ...
```

Every one of those is on Windows XP SP3. There is no `api-ms-win-*`
import, no Universal CRT, no symbol of Vista or later. **A binary of
Rust 1.99 runs on XP, as long as the standard library does not come
into it.** The probes of the lab (`scripts/xp/probe`, `scripts/xp/floor`)
are binaries of this kind; the runner takes the other way, and emends
the `std` instead of dropping it.

### The toolchain of the floor

The build of the floor asks three things of the machine, and the three
are measured:

1. **The link of the target asks for `-lgcc_eh`, and `w64devkit` does
   not ship it.** The link line of `i686-pc-windows-gnu` carries
   `-lgcc_eh -l:libpthread.a -lmsvcrt -lmingwex -lmingw32 -lgcc`, and
   the `lib32` of `w64devkit` has the import libraries (`libkernel32.a`,
   `libuser32.a`, `libmsvcrt.a`, `libpthread.a`) and `libgcc.a`, and
   **no unwinder**. The link stops with `cannot find -lgcc_eh`. The
   answer is the MinGW of thirty-two bits with the `msvcrt` runtime --
   the winlibs build `winlibs-i686-posix-dwarf-gcc-16.2.0-mingw-w64msvcrt-14.0.0-r3`,
   in `C:\Users\<user>\tools\mingw32` -- and it is the one the floor
   wants anyway, because its CRT is the `msvcrt.dll` of Windows.
2. **The `core` that ships with the target asks for a personality
   routine.** The `core` of the sysroot is built with the unwinding on,
   and its `.eh_frame` refers to `rust_eh_personality`; a binary
   without `std` has no personality routine, and the link fails with
   `undefined reference to 'rust_eh_personality'` as soon as the
   program touches the parts of `core` that carry it (the strings of
   UTF-8, for one). `-Z build-std=core,panic_abort` builds `core`
   again with the aborting panic, and the reference is gone. So the
   toolchain of the floor is a **nightly** with `rust-src`, and not a
   stable one. The nightly is not pinned yet; the build of the floor
   takes the name of it in `-Toolchain`, so a dated one can be named
   when the build has to be the same tomorrow.
3. **No compiler of the tree runs on XP.** The driver of the compiler
   imports `api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`,
   Windows 8), `GetSystemTimePreciseAsFileTime` (Windows 8),
   `InitializeCriticalSectionEx` (Vista), `IsThreadAFiber` (Windows 8)
   and `CreateWaitableTimerExW` (Windows 7): the loader of XP refuses
   it before its first instruction. And the tree needs `rustc` 1.85 at
   least (the edition of 2024), so no compiler of the tree can live in
   the guest. The toolchain is on the machine of the tree, and the
   guest only ever runs what the tree builds.

The setup of the machine of the tree is therefore:

```sh
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup target add i686-pc-windows-gnu --toolchain nightly
```

and the script that does the build of the probes and reads the gate is
`scripts/xp/build.ps1`; the page of the tools is
[`scripts/xp/README.md`](../scripts/xp/README.md).

### The machine of the lab

The XP machine of the lab is a guest of KVM on the Unraid box
(`WindowsXP`), with the answer file of `unattend.flp`: Windows XP
Professional SP3, an empty password for the administrator, the account
`User`, 1024x768x32.

**Its network is isolated, and the isolation is measured from the
inside.** The domain has one adapter, an `e1000`, and it is on a
libvirt network of its own (`xp-isolated`, `virbr-xp`,
192.168.250.0/24) with `<forward mode="none"/>`: libvirt's own rule
rejects everything that leaves the bridge
(`LIBVIRT_FWO -i virbr-xp -j REJECT`), so the guest reaches neither
the LAN nor the internet, with or without a default route. A hook of
the network adds the other half: the guest reaches the host only on
the address of the bridge (192.168.250.1), and not on the LAN
address of the host, nor on its other services
(`LIBVIRT_INP ! -d 192.168.250.1 -i virbr-xp -j DROP`). The guest,
tried to reach the internet on its own after its network came up, and
answered "Cannot find server": the floor of the isolation, measured.

**The guest is a stripped XP, and the answer file is why.** The file
says `InstallDefaultComponents=No`, so the installation left out the
network components, and with them WMI (`wmic` is not recognized),
`netcfg.exe` and the TCP/IP client. The components were put back by
hand from the CD of the installation (`nettcpip.in_` and
`netmscli.in_`), and the stack is up. What is missing is the *driver*
of the adapter: the `e1000` of QEMU is the Intel 82540EM, and the
driver of it is in the CD (`nete1000.in_`, with the files of the
drivers in `driver.cab`); the step that is left is the wizard of the
hardware ("Have Disk", the folder of the INF and of the cab).

**The answer file of the box has two traps:**

1. **Its lines end with CRLF, and the setup of XP ignores a file whose
   lines end with LF alone** -- the whole file, not a section of it.
   The setup of a file with LF alone asks for the partition, asks for
   the key of the product, and applies none of the sections (the
   resolution, the account, the network).
2. **The key of the product is asked for, and the setup refuses the
   two keys that are published**: `MRX3F-47B9T-2487J-783G8-XV738` (the
   key of the file) and `VK7JG-NPHTM-C97JM-9MPGT-3V66T` both answer
   "The Product Key you entered is not valid." The installation that
   the box has was made with the key of the file, so the way around
   the question is the answer file, and not the keyboard.

So the reinstallation of the machine, with the whole of the network
and the driver of the adapter, is a question that the file has to
answer -- and the file has to keep its CRLF.

The channel of the lab, the floppy and the screen, and the tools of
it, are in [`scripts/xp/README.md`](../scripts/xp/README.md).

Measured on that machine, with the probe of `scripts/xp/probe` -- Rust
without the standard library, for `i686-pc-windows-gnu`, linked by the
MinGW of thirty-two bits with the `msvcrt` runtime, built by the
nightly with `-Z build-std`:

```
C:\Documents and Settings\User>c:\probe.exe
ahk-xp-probe: alive
windows: 5.1 build 2600 sp3
tick: 2357171
pid: 1516
rust: no_std, i686-pc-windows-gnu, panic=abort
```

The table of the imports of that binary is `kernel32.dll` (51 symbols)
and `msvcrt.dll` (43 symbols), the subsystem version is 4.0, and there
is no api set in it. **The floor is measured on the machine it is a
promise about.**

### The floor of the two builds

The tree has two Windows builds, and the floor of each is decided by
its import table.

**The modern MSVC build** asks for the calls of the backend by name at
link time, through `windows-sys`. Measured on `target/release/ahk.exe`
and on `target/debug/ahk.exe` (`llvm-objdump -p`), the calls of the
backend that are newer than the floor of the `std` are:

| The call | Where | First Windows |
|---|---|---|
| `SetProcessDpiAwarenessContext` | `crates/ahk-box/src/windows/screen.rs:45` | **Windows 10, version 1703** |
| `GetDpiForSystem` | `crates/ahk-box/src/windows/screen.rs:218` | Windows 10, version 1607 |
| `GetDpiForMonitor` | `crates/ahk-box/src/windows/screen.rs:282` | Windows 8.1 |
| `SHGetKnownFolderPath` | `crates/ahk-box/src/windows/machine.rs` | Vista |
| `GetAdaptersAddresses` | `crates/ahk-box/src/windows/network.rs` | Vista |
| `InitCommonControlsEx` | `crates/ahk-box/src/windows/gui.rs` | XP |

**The floor of the modern MSVC binary is Windows 10 version 1703**,
and it is not the `std` that decides it: it is the call of the DPI
awareness that the screen of the backend asks for. The `std` of Rust
1.78 would have made the floor Windows 8; the backend moved it to
Windows 10 1703. This is the build the release workflow makes.

**The XP build** (`i686-win7-windows-gnu`, the `std` emended) imports
none of those calls by name: the seven newer than XP are asked to the
DLL at runtime (`the_symbol_of`), with the fallback of the older API
or the refusal with the reason. Its floor is the measured floor of XP
SP3 -- 423 symbols asked, 423 present, none missing -- and the binary
runs the v1 and the v2 on the machine of the lab.

---

## The rule that lets the three live together

The floor of a binary is **the import table**, and nothing else. So a
binary whose import table is XP-clean can still behave like a Windows
11 program: it resolves the newer calls **at runtime** and uses them
when they are there.

That is not an invention: it is what AutoHotkey itself does.

| The call | How AHK gets it | Where |
|---|---|---|
| `RtlGetVersion` | `GetProcAddress(GetModuleHandle("ntdll.dll"), "RtlGetVersion")`, with `GetVersionExW` as the fallback | `os_version.cpp:91-101` |
| `IsHungAppWindow` | `GetProcAddress`, with a `SendMessageTimeout` fallback | `window.cpp:1163-1215` |
| `SendInput` | resolved dynamically, with the `keybd_event` path as the fallback | `keyboard_mouse.cpp:2791`, `:183-221` |
| GDI+ | `LoadLibrary("gdiplus")`, with `OleLoadPicture` as the last resort | `util.cpp:2335-2419` |
| `SetProcessDpiAwarenessContext` | resolved dynamically, with `SetProcessDPIAware` as the fallback | -- |

So the rule of the three backends is short:

1. **A call of the floor is imported statically.** The floor of the XP
   build is the XP era: `kernel32`, `user32`, `gdi32`, `advapi32`,
   `shell32`, `comdlg32`, `winmm`, `psapi`, `version`, `wsock32`, and
   `msvcrt`.
2. **A call newer than the floor is resolved at runtime**, and the
   desktop uses it when it is there. When it is not, the answer is
   the refusal of the tree -- with the reason -- and the fallback of
   the older API when there is one.
3. **The floor of the build decides which of the two is used first**,
   and the behaviour of the modern machine is the modern one.

**The rule is implemented.** The loader is
`windows/sys.rs:the_symbol_of` (`GetModuleHandleW` / `LoadLibraryW` +
`GetProcAddress`, with the name ended in zero), and the seven calls
newer than XP go through it: `SHGetKnownFolderPath`,
`SHCreateItemFromParsingName`, `GetDpiForMonitor`, `GetDpiForSystem`,
`SetProcessDpiAwarenessContext`, `GetUserDefaultLocaleName` (which
became `GetLocaleInfoW`) and `QueryFullProcessImageNameW` (which became
`GetModuleFileNameExW` of psapi). The gate that enforces the rule is
the one of [The gate](#the-gate), over the measured floor.

The table of the calls of the desktop, and what happens at each floor:

| The call | First Windows | At the modern floor | At the XP floor |
|---|---|---|---|
| `RtlGetVersion` | NT4 | used | resolved; the fallback is `GetVersionExW` |
| `SetProcessDpiAwarenessContext(PER_MONITOR_AWARE_V2)` | Win10 1703 | used | resolved; the fallback is `SetProcessDPIAware` (the model of AHK) |
| `GetDpiForWindow`, `GetSystemMetricsForDpi`, `AdjustWindowRectExForDpi` | Win10 1607 | used | not there; the DPI of the process is 96 |
| `IFileDialog` | Vista | used | not registered on XP; the picker refuses -- the fallback (`GetOpenFileNameW` of `comdlg32`) is not written |
| `MMDeviceEnumerator` (the volume of the sound) | Vista | used | not registered on XP; the volume refuses -- the fallback (the **mixer API** of the v1, `mixerOpen`, `mixerGetLineInfoW`) is not written |
| `SHGetKnownFolderPath` | Vista | used | resolved; the fallback is the classic folder of XP |
| `GetTickCount64` | Vista | used | resolved; the fallback is `GetTickCount` (which is what AHK uses) |
| `DwmGetWindowAttribute(DWMWA_CLOAKED)` | Vista | used | not there; the cloaked windows do not exist |
| `SetWinEventHook` | Vista | used, to be told | XP has it (measured on the XP guest of the lab) |
| `AddClipboardFormatListener` | Vista | used | resolved; the fallback is the chain of `SetClipboardViewer` |
| `GetFileInformationByHandleEx` | Vista | used | resolved; the fallback is `GetFileTime` and `GetFileSizeEx` |
| `QueryFullProcessImageNameW` | Vista | used | resolved; the fallback is `GetModuleFileNameExW` of `psapi` |
| `PROCESS_QUERY_LIMITED_INFORMATION` | Vista | used | resolved; the fallback is `PROCESS_QUERY_INFORMATION` |
| `RegDeleteKeyEx` | XP SP2 | used | not there on XP RTM; the fallback is `RegDeleteKey` |
| `SendInput` | Win2000 | used | used (XP has it), with the cap of the older system in mind |
| `keybd_event` | Win95 | the fallback | used |

---

## The XP backend

What it is: **the runner of the tree, in both dialects, on the tier-3
target `i686-win7-windows-gnu` with the standard library emended for
the era of XP, over the XP-era Win32 calls, with the modern calls
resolved at runtime when they are there.** The binary of 2,57 MB
(`-C strip=symbols -C opt-level=z`) runs the v1 and the v2 on the
Windows XP SP3 guest of the lab, and the report comes back through the
floppy with `"a_linefile":"c:\t.ahk"` -- the path of the disk of the
guest, the proof that it runs there and not on the host.

The pieces:

1. **The calls resolved at runtime** -- `the_symbol_of`, and the seven
   calls of the backend that are newer than XP (the rule above).
2. **The `std` emended, not dropped** -- `scripts/xp/patch_std.py`
   rewrites the Vista+ calls of the `std` over the APIs of Windows
   2000: the FLS family becomes TLS, the SRW locks become a spin lock
   of atomics, the `Condvar` becomes an event of auto-reset, the
   `GetFileInformationByHandleEx` becomes `GetFileInformationByHandle`
   and `NtQueryDirectoryFile`, the trio of `ProcThreadAttributeList`
   and `CreateSymbolicLinkW` become `Unsupported`. The `-Z build-std`
   recompiles it; the build folder of the target has to be erased for
   the emend to enter the binary, because the fingerprint of the cargo
   does not change when the `std` changes.
3. **The gate over the measured floor** -- `scripts/xp/gate.py` reads
   `scripts/xp/floor-xp-sp3.txt` and fails on the symbol the machine
   does not have, and on the symbol nobody asked about.
4. **The fallbacks of the era of XP** -- the picker of Vista
   (`IFileDialog`) and the volume of WASAPI (`MMDeviceEnumerator`)
   are not registered on XP, so the common dialogs of comdlg32
   (`windows/picker.rs`) and the mixer API of winmm (`windows/mixer.rs`)
   answer in their place, the way of the v1. Measured on the two
   guests in both dialects.
5. **The machine of the lab** -- the XP SP3 guest of the Unraid box
   and the XP x64 SP2 guest, the floppy as the channel in, the floppy
   and the screen as the channel out (`scripts/xp/xpctl.py`,
   `scripts/xp/mkiso.py`).

What is left:

- **The middle Windows**: Vista, 7, 8 and 8.1 are above the floor by
  construction -- the XP build serves them, see
  [the middle Windows](#the-middle-windows-vista-8-81-and-the-rest) --
  but the lab has no machine of them, and the floor is a measurement.

### The notebook on XP, measured

The behaviour notebook was run on the XP SP3 guest itself, not only on
the modern host: 1051 cases of the v1 and the 113 cases of the v2
differential, each in its own process, driven by
`scripts/xp/driver.bat` (`scripts/xp/xpctl.py` carries the corpus and
reads the results).

- **v1, XP-tree against host-tree**: **1051 of 1051 identical**.
- **v1, XP-tree against the real v1.1.37.02 on XP**: **1051 of 1051
  exact.** All 26 original diffs are closed: ten were environmental (the
  XP `read_dir` fix) and sixteen were closed in the tree (key names
  `LButton`/`F13`-`F24`/`vkXX`/`scXX`/`JoyN`, `KeyWait` parameter
  validation, `FileEncoding, CP1250`, the v1 error wordings, the
  unclosed quote in an `if` condition, the v1 refusal to write a member,
  and the v1 error precedence where an earlier nonexistent call wins
  over a later missing bracket but a later line that holds no action is
  reported at once).
- **v2, XP-tree against host-tree**: **113 of 113 identical**. The v2
  reference (2.0.31) does not run on XP, so the host tree is the
  authority there.

The last seven v1 and one v2 diffs were not behaviour: the std's
`read_dir` opens the folder with `FindFirstFileExW(FindExInfoBasic)`,
which XP rejects with OS error 87, so any relative-path file loop died
on the guest. `scripts/xp/patch_std.py` now emends it to
`FindExInfoStandard` (Windows 2000-era), and with that the notebook is
100% on XP with no curation.

### The channel, and the FTP question

The lab guest is air-gapped by the unattend (`InstallDefaultComponents
=No`), so the first channel was the floppy. The floppy works but is
fragile for bulk: FAT12 corrupts under thousands of files, and QEMU
caches guest writes until the media is ejected. The robust pattern is
to pack results into a few large files, or better, to move off the
floppy entirely.

The NIC was switched from `e1000` (no in-box XP driver) to `pcnet`,
which XP drives in-box; TCP/IP was installed and a static address
assigned (`192.168.250.2` on `xp-isolated`). **FTP now works** and is
the bulk channel: a `pyftpdlib` server on the NAS, the guest's built-in
`ftp.exe` as client (`scripts/xp/ftp.md`, `scripts/xp/ftp-measure.py`).
One caveat: `mget`/`mput` do not enumerate against this server, so the
scripts use one `get`/`put` per file. The floppy remains for the small
control batches; the ISO (`scripts/xp/mkiso.py`) and a partitioned
writable data disk are the offline alternatives.

What it buys: a machine that still runs Windows XP -- an old
industrial machine, a point of sale of 2009, a museum -- runs the v1
dialect of this tree, with the same names the tree documents in
thirteen languages. The v2 runs there too, measured.

---

## The Windows 7 backend

Much cheaper than the XP one, because the standard library of Windows 7
**exists**: Rust ships the tier-3 targets `x86_64-win7-windows-msvc`,
`i686-win7-windows-msvc`, `x86_64-win7-windows-gnu` and
`i686-win7-windows-gnu` since 1.76, and their standard library is
built with the Windows 7 baseline. The local source of the standard
library shows the two shims that make it so:

- `library/std/src/sys/pal/windows/c.rs:146-152` -- under
  `#[cfg(target_vendor = "win7")]`, `GetSystemTimePreciseAsFileTime`
  becomes `GetSystemTimeAsFileTime`;
- `library/std/src/sys/pal/windows/c.rs:186-195` -- the same cfg makes
  `WaitOnAddress` and `WakeByAddressSingle` *optional*, loaded through
  the delayed binding of `compat.rs`, with `NtCreateKeyedEvent` as the
  fallback of the parking of a thread.

So the Windows 7 backend is:

1. the same body of code, built for the tier-3 target with
   `-Z build-std` on a **nightly** toolchain (tier 3 has no shipped
   standard library, so it has to be built);
2. **the calls of the backend resolved at runtime** -- done for the XP
   floor, and it serves this one too: the DPI call
   (`SetProcessDpiAwarenessContext`, Windows 10 1703) and the DPI of
   the monitor (`GetDpiForMonitor`, Windows 8.1) are the two that
   matter, and the fallbacks are `SetProcessDPIAware` and the DPI of
   96;
3. a linker that does not leave the `api-ms-win-crt-*` imports to the
   machine -- they resolve on Windows 7 only with the Universal CRT
   downlevel (KB2999226). The AHK v1 binary of the measurement imports
   **no** CRT at all: the CRT is inside it. That is the safest answer,
   and the tree has the machinery for it (`-C target-feature=+crt-static`
   with the `msvcrt` flavour of the MinGW, or the static CRT of the
   MSVC toolset). The `msvcrt` of the GNU flavour needs no downlevel
   update at all;
4. the same gate of the import table, with the floor of Windows 7.

The `win7` target of the GNU flavour is already in use -- it is the
target of the XP build -- and the binary of the XP lab is XP-clean, so
a Windows 7 machine is above its floor by construction. What is left
is the asset and the choice of the CRT. It is not measured: the lab
has no Windows 7 machine, and the floor of this tree is a measurement.

---

## The middle Windows: Vista, 8, 8.1 and the rest

The three floors are not three machines: a floor is an import table, and
a binary runs on **every machine at or above its floor**. The XP build
resolves the newer calls at runtime, so it runs on XP *and on everything
after it* -- Vista, 7, 8, 8.1, 10 and 11. The middle Windows are therefore
covered by construction, even though the lab has no guest of them; what
changes from one to the next is which of the runtime-resolved calls the
machine actually has.

| Windows | NT kernel | modern MSVC (floor Win10 1703) | `win7` build (floor 7) | XP build (floor XP) |
|---|---|---|---|---|
| 2000 | 5.0 | no | no | no |
| XP (SP3) | 5.1 | no | no | **yes** (measured on the XP guest of the lab) |
| 2003 / XP x64 | 5.2 | no | no | **yes** (x64 measured) |
| Vista | 6.0 | no | no | **yes** |
| 7 | 6.1 | no | **yes** | **yes** |
| 8 | 6.2 | no | **yes** | **yes** |
| 8.1 | 6.3 | no | **yes** | **yes** |
| 10 (1703+) | 10.0 | **yes** | **yes** | **yes** |
| 11 | 10.0 (build ≥ 22000) | **yes** | **yes** | **yes** |

The `win7` standard library has the Windows 7 baseline, so it does not
promise Vista; the XP build, whose import table is XP-clean and whose
newer calls are runtime-resolved, is the one that reaches Vista.

**The ladder of the calls.** Each rung is the first Windows that has the
call; below it the XP build uses the fallback (or refuses with the
reason), above it the call is used. This is the same table as
[the rule](#the-rule-that-lets-the-three-live-together), read as a
ladder of machines:

| First Windows | The calls that arrive | What the machine below does |
|---|---|---|
| Vista (6.0) | `IFileDialog`, `MMDeviceEnumerator` (WASAPI), `SHGetKnownFolderPath`, `GetTickCount64`, `DwmGetWindowAttribute`, `GetFileInformationByHandleEx`, `QueryFullProcessImageNameW` | XP: the comdlg32 picker and the winmm mixer answer in place of the COM ones; the classic folders, `GetTickCount`, `GetFileTime`/`GetFileSizeEx`, `GetModuleFileNameExW` answer in place of the rest |
| 7 (6.1) | `CreateWaitableTimerExW`, `WaitForSingleObjectEx` (the `std` of the modern build) | the `win7` `std` shims them; the XP `std` is emended |
| 8 (6.2) | `WaitOnAddress`/`WakeByAddress*`, `GetSystemTimePreciseAsFileTime`, `IsThreadAFiber` | the `win7` `std` loads the parking delayed with `NtCreateKeyedEvent`; the clock falls back to `GetSystemTimeAsFileTime` |
| 8.1 (6.3) | `GetDpiForMonitor` | the DPI of the monitor is 96 below it |
| 10 1607 | `GetDpiForSystem`, `GetDpiForWindow`, `GetSystemMetricsForDpi` | the DPI of the process is 96 below it |
| 10 1703 | `SetProcessDpiAwarenessContext` | the fallback is `SetProcessDPIAware` (the model of AHK) |

So a script on Vista, 8 or 8.1 gets the behaviour of the calls that
machine has, and the honest fallback or refusal for the ones it does
not -- the same rule as XP, one rung up. `A_OSVersion` reads the machine
through `RtlGetVersion` (the manifest shim does not touch it) and answers
`major.minor.build`, so it names every one of these versions, measured on
the modern host (`10.0.…`) and on the XP guest (`5.1.2600`).

What is left for the middle Windows is a **measurement**, not code: the
lab has no Vista/8/8.1 guest, so their rungs are read from the import
tables and the API documentation, not from a probe on the machine. The
XP build is the asset that serves them; a guest of each would turn the
rungs into measured floors, the way XP and XP x64 already are.

---

## The gate

The promise of each floor is a property of the built binary, and the
check is a script: **`scripts/xp/gate.py`**. It reads the import table
of a PE by itself (the DOS header, the PE header, the sections and the
data directories) and it fails, naming the reason, when:

- the machine of the binary is neither the thirty-two bits one nor the
  sixty-four bits one;
- the subsystem version is newer than the one of XP (5.1);
- a DLL of the api sets (`api-ms-win-*`, `ext-ms-*`) is in the table;
- a DLL outside the allowlist is in the table;
- a symbol is in the table that the machine said it does not have;
- a symbol is in the table that nobody asked the machine about.

**The authority is the measurement, not a list of quotations.** The
probe of the floor (`scripts/xp/floor`) asks the XP machine of the lab
for every symbol of the table and for the classes of COM that the tree
names, and writes the answer in `scripts/xp/floor-xp-sp3.txt` (the
PRESENT and the MISSING lines). The gate reads that file and is an
oracle of **three** answers: measured present, measured missing, never
asked about. It fails on the third too -- a floor that was not measured
is a floor that is guessed.

The table is the **union** of the import tables of every binary that
goes to the machine -- the `ahk.exe` of the patched `std`, the minimal
`hello` of the lab, and the two probes (`scripts/xp/floor/gen_table.py`
writes it). The union matters: the `hello` imports more than the
`ahk.exe`, because the `--gc-sections` of the tree cuts from the
`ahk.exe` the sections it never references. The answer of the machine:

```
machine: windows 5.1 build 2600 sp3
symbols: 352   missing: 0
com CLSID_FileOpenDialog: NOT registered, the answer is 0x80040154
com CLSID_MMDeviceEnumerator: NOT registered, the answer is 0x80040154
com CLSID_ShellLink: registered
```

The gate passes with exit 0 on all four binaries of the lab. The
classes say what the tree has to do without COM on XP: the folder
dialog (`IFileDialog`) and the sound (WASAPI) are **Vista**, the shell
link is **XP**.

**The machine corrects the documents.** Seven symbols that the tables
of the documents put after XP, XP has: `GetAdaptersAddresses`,
`SetWinEventHook`, `InitializeCriticalSectionEx`, `CancelIo`,
`SwitchToThread`, `WaitForSingleObjectEx` and `GetFileTime`. The gate
follows the machine, and not the quotations.

The allowlist of the DLLs is the measurement of the last AHK v1
(1.1.37.02): the fourteen classic DLLs of its table, plus `msvcrt.dll`,
plus the three the tree asks and XP SP3 has (measured on the XP guest
of the lab): `iphlpapi.dll`,
`userenv.dll`, `ws2_32.dll`.

The sixty-four bits have their own machine and their own floor: the
guest `WindowsXP64` of the lab (Windows XP Professional x64 Edition
with SP2, 5.2 build 3790) answers the probe built for
`x86_64-pc-windows-gnu`, and the answer is in
`scripts/xp/floor-xp-x64.txt` -- 345 symbols asked, 345 present, none
missing, including the eight that only exist as such on a
sixty-four-bit Windows. The gate reads the two floors together.

The gate is in CI beside the build of each asset (the `windows-xp` job
of `.github/workflows/ci.yml`, through `scripts/xp/release.ps1`), and
the check of the XP asset is the one that has teeth: a single new call
added without thinking about the floor breaks the XP build, and the
build says so instead of the machine of the user. The CI of the tree
already
builds the workspace for the 32-bit Windows (`i686-pc-windows-msvc`,
`.github/workflows/ci.yml:141-163`), and the build of the Windows
backend is part of it -- so the body of the code is compiled and
clippyed for the machine of thirty-two bits, which is the machine of
XP.

The build of the floor asks three things of the tree, and the tree
answers them: the packing of the hotkeys is the width of the machine
(`windows/hotkeys.rs`, the `lparam` carries the key and the modifiers
in the halves of the word), the flags of the manifest go to the linker
of Microsoft only (`crates/ahk-box/build.rs`, `crates/ahk-cli/build.rs`
-- the linker of the GNU target refuses them), and the linker has the
`libgcc_eh` the target asks for (the MinGW of the floor, or the empty
stub of it for the sixty-four bits). The details are in
[`scripts/xp/README.md`](../scripts/xp/README.md).

---

## The release

The project is not released yet. The release workflow
(`.github/workflows/release.yml`) builds seven assets --
`x86_64-windows`, `x86_64-linux-gnu`, `aarch64-linux-gnu`,
`x86_64-macos`, `aarch64-macos`, `i686-windows-xp` and
`x86_64-windows-xp` -- with the naming of the tree
(`ahk-v<version>-<asset>.<zip>`). The Windows family has three floors:

| The asset | The target | The dialect | The floor | The state |
|---|---|---|---|---|
| `x86_64-windows` | `x86_64-pc-windows-msvc` | v2 (and v1) | Windows 10 and later | in the workflow |
| `x86_64-windows-7` | `x86_64-win7-windows-msvc`, built with `-Z build-std` | v2 (and v1) | Windows 7 SP1 | planned |
| `i686-windows-xp` | `i686-win7-windows-gnu`, the `std` patched, built by `scripts/xp/release.ps1` on a pinned nightly | v1 and v2 (both measured on XP) | Windows XP SP3 | in the workflow (builds, gates, smoke-tests) |
| `x86_64-windows-xp` | `x86_64-win7-windows-gnu`, the same, with the stub of `libgcc_eh` | v1 and v2 (both measured on XP x64) | Windows XP x64 SP2 | in the workflow (builds, gates, smoke-tests) |

The XP assets are built and gated by `scripts/xp/release.ps1`, which
pins the nightly, emends the `std`, downloads the MinGW of the `msvcrt`
flavour when the machine has none, and runs `gate.py` over the measured
floor. The smoke test of the runner answers on the runner too (the XP
binary runs on the modern Windows): the surface of both dialects and a
script that writes to the standard output. The Linux asset has its own
smoke test beside.

---

## The decisions

1. **The dialect of the XP backend: both.** The reference of the floor
   is the v1, because the v2 of AutoHotkey never ran on XP and the last
   AHK that runs on XP is the last v1 (1.1.37.02). The dialects of this
   tree are its own: the same binary runs the v1 and the v2 on the XP
   guest, and the report is the same. The floor of the machine does not
   choose between them.
2. **The shape of the crates: the Windows API layer is inside
   `ahk-box`, and it stays there.** The tree took the module
   (`ahk-box::windows`, `#[cfg(windows)]`, `windows-sys`): one publish,
   no duplicate of the parts that `ahk-box` already has. The patched
   `std` makes a split of the crate unnecessary -- the XP binary links
   the same crate, with the same `std` emended.
3. **The calls newer than the floor are resolved at runtime.** This is
   the rule that AHK itself follows, and it is what the tree does:
   `the_symbol_of` asks the DLL for the name, and the seven calls
   newer than XP go through it. The floor of the XP build is the
   measured floor of XP SP3; the floor of the modern MSVC build is
   Windows 10 1703, and that is the build the release workflow makes.
4. **The Windows 7 build with a pinned nightly.** `-Z build-std` needs
   a nightly toolchain, and a pinned one is a promise to update it.
   The alternative -- to skip Windows 7 and to keep the modern floor --
   is honest and cheaper, and the v1.1.37.02 binary of the measurement
   shows that a machine of 2011 is still a real audience.
5. **The XP build with a nightly and with the MinGW of the `msvcrt`
   flavour.** The XP build needs `-Z build-std=std,core,alloc,panic_abort`
   and the emends of `scripts/xp/patch_std.py`; the build folder of the
   target has to be erased for the emend to enter the binary. The
   thirty-two bits uses the MinGW of thirty-two bits with the `msvcrt`
   runtime (`w64devkit` has no `libgcc_eh.a`); the sixty-four bits uses
   the `w64devkit` with the empty stub of `libgcc_eh` (the linker of
   the `strawberry/c` links a binary that rebents at startup). The
   promise to update the nightly is the same promise of the Windows 7
   build, and the toolchain of the C is a promise to keep the `msvcrt`
   flavour of the MinGW.

And one thing that is mine to say: the XP floor is a **promise about
the import table**, and it is worth exactly as much as the gate that
checks it.}
