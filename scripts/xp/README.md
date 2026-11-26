# The XP floor

The tools of the floor of Windows XP: the build of a binary of
thirty-two bits that runs on Windows XP SP3, the gate that says whether
a binary does, and the machine of the lab that runs it.

The plan of the floor is in [`docs/windows-legacy.md`](../../docs/windows-legacy.md);
this page is the instrument of it, and the measurements it carries.

- [The machine of the lab](#the-machine-of-the-lab)
- [The toolchain](#the-toolchain)
- [The build](#the-build)
- [The gate](#the-gate)
- [The probe](#the-probe)
- [What was measured](#what-was-measured)

---

## The machine of the lab

The lab has two XP guests of KVM on the Unraid box: `WindowsXP`, the
thirty-two bits, and `WindowsXP64`, the sixty-four. The two are on the
same isolated network (`xp-isolated`), and the channel of the two is
the floppy and the screen (`scripts/xp/xpctl.py`, with `XP_VM` naming
the guest).

The XP machine of thirty-two bits is a guest with
the answer file of `unattend.flp`: Windows XP Professional SP3, an
empty password for the administrator, the account `User`, 1024x768x32.

**The network of the guest is isolated.** The domain has one adapter,
an `e1000`, and it is on a libvirt network of its own:

```
<network>
  <name>xp-isolated</name>
  <forward mode="none"/>
  <bridge name="virbr-xp" stp="on" delay="0"/>
  <ip address="192.168.250.1" netmask="255.255.255.0">
    <dhcp><range start="192.168.250.2" end="192.168.250.100"/></dhcp>
  </ip>
  <hook script="/mnt/user/system/xp-isolated-hook.sh" prefix="xp-isolated"/>
</network>
```

The isolation is the one of libvirt and the one of the hook:

| Rule | What it does |
|---|---|
| `LIBVIRT_FWO -i virbr-xp -j REJECT` | libvirt's own rule for a network with `forward mode='none'`: **nothing that leaves the bridge is forwarded**, so the guest reaches neither the LAN nor the internet, with or without a default route |
| `LIBVIRT_INP ! -d 192.168.250.1 -i virbr-xp -j DROP` | the rule of the hook: the guest reaches the host only on the address of the bridge (192.168.250.1), and not on the LAN address of the host, and not on its other services |

The hook is run by libvirt every time a port is added to the network,
so the second rule is there after a reboot of the box, with the
network itself.

**The channel of the lab is the floppy and the screen.** The guest has
no network to the outside (that is the point), so a binary of the tree
travels on the **floppy** (the IDE one of the guest, which the guest
reads as `A:`), and the answer of it comes back through the **screen**:
the keyboard goes in through the QEMU monitor (`sendkey`) and the
framebuffer comes out through the same monitor (`screendump`) or
through the VNC of the guest (`scripts/xp/rfb.py`).

```sh
# a binary of the tree on the floppy of the guest
python scripts/xp/xpctl.py put scripts/xp/probe/target/i686-pc-windows-gnu/release/ahk-xp-probe.exe

# a command line in the console of the guest, and the return
python scripts/xp/xpctl.py cmd 'copy a:\ahk-xp-probe.exe c:\probe.exe'
python scripts/xp/xpctl.py cmd 'c:\probe.exe'

# the screen of the guest
python scripts/xp/xpctl.py shot /tmp/xp.png
```

The environment of the tool: `XP_SSH` (the ssh of the box,
`root@192.168.0.10`), `XP_KEY` (the key, `~/.ssh/rgx_unraid`) and
`XP_VM` (the name of the guest, `WindowsXP`).

### What the machine of the lab is, and what it is not

The guest was installed with the answer file of the box, and the
answer file says `InstallDefaultComponents=No`: the installation left
out the network components, and with them WMI (`wmic` is not
recognized), `netcfg.exe` and the TCP/IP client. The machine is
therefore a *stripped* XP: it is good for the language, the desktop
and the v1, and poor for the network.

The components were put back by hand, from the CD of the installation
(`en_windows_xp_professional_with_service_pack_3_x86_cd_vl_x14-73974.iso`),
which carries them (`i386\nettcpip.in_`, `i386\netmscli.in_`):

```
expand d:\i386\nettcpip.in_ c:\drv\nettcpip.inf
expand d:\i386\netmscli.in_ c:\drv\netmscli.inf
```

and the "Networking Services" of the Windows Components Wizard is
checked again. The stack is up: the guest tries to reach the internet
on its own and gets "Cannot find server" -- the isolation, measured
from the inside.

**What is still missing is the driver of the adapter.** This one is
measured now, and the answer is not "the CD has no
driver":

- **The card**: QEMU's `e1000` is the Intel 82540EM, and it reports
  **8086:100E** (`info pci` of the monitor of the guest).
- **The driver of the CD**: `i386\nete1000.in_` expands to
  `c:\drv\nete1000.inf` (22,319 bytes) and its `DriverVer` is
  **07/01/2001** -- the driver of the year 2001, which is before the
  82540EM (the "E" variant).
- **Its hardware IDs**: `DEV_1000` (82540EM), `DEV_1001`,
  `DEV_10048`, `DEV_10018` -- and **no `DEV_100E`**. So the setupapi
  of XP finds no device in it and answers, every time, *"The specified
  location does not contain information about your hardware"* --
  with the compressed `.in_` of the CD, with the expanded `.inf` of
  `c:\drv`, and with the folder `C:\drv` as the location of the
  search.
- **The fix is one line in the INF**: in the section `[Intel]`, next
  to the generic model of the INF,

  ```
  %E1000GENERIC.DeviceDesc% = E1000.ndi, PCI\VEN_8086&DEV_100E
  ```

  It is written in `c:\drv\nete1000.inf` (notepad), and
  `findstr /i "100E" c:\drv\nete1000.inf` finds it. The driver of the
  files of the family (`w940nd.sys` and the rest) is already extracted
  in `c:\drv` (393 files, 46 MB, `expand c:\drv\driver.cab -F:*
  c:\drv\`).

**And the cards that were tried, and why they do not work either:**

- **virtio: no.** Windows XP has no virtio driver; the virtio
  drivers of Windows are of Vista and later, and the CD of the
  installation has none.
- **`pcnet`** (the AMD PCnet-PCI, 1022:2000): XP knows the card only
  as a bare "PCI Device" (it has no class driver for it), the wizard
  offers no model for it, and the INF of the CD for it
  (`NETAMD2.IN_`, the "AMD PCNET Family PCI Ethernet Adapter", with
  `pcntpci5.sys`) does not change that.
- **`ne2k_pci` and `rtl8139`**: the CD has **no** INF for the NE2000
  and **no** INF for the Realtek RTL8139 (the network INFs of the CD
  are `NETAMD*`, `NETE1000`, `NETTB155`, `NETTIGER` (3Com),
  `NETW840`, `NETW926`, `NETW940` (Winbond) and the Microsoft
  miniports).

So the e1000 is the card, and the INF is the fix. What is left of
the installation is the last step of the wizard: "Have Disk",
`C:\drv\nete1000.inf`, the model, Next, Finish -- and the fight with
the notification balloon of the "Found New Hardware" (it takes the
keyboard, and Escape on it closes the wizard with it).

### The answer file, and the key of the product

Two things about the answer file of the box (`winnt.sif`, on the
floppy of the guest), measured:

1. **The lines end with CRLF.** The setup of XP reads the file as it
   is, and a file with LF alone is ignored *entirely*: the setup asks
   for the partition, it asks for the key of the product, and the
   sections of the file (the resolution, the account, the network) are
   not applied. The file of the box has 39 CRLF and the one written
   with LF has 0, and the difference is the whole behaviour of the
   installation.
2. **The key of the product is asked for, and the setup refuses the
   two keys that are published.** The setup of this media answers
   "The Product Key you entered is not valid." to
   `MRX3F-47B9T-2487J-783G8-XV738` (the key of the file) and to
   `VK7JG-NPHTM-C97JM-9MPGT-3V66T`. The installation that the box
   has was made with the key of the file, so the way around the
   question is the answer file, not the keyboard.

So the reinstallation of the machine, with the whole of the network
and the driver of the adapter, is a question that the file has to
answer, and the file has to keep its CRLF.

---

## The toolchain

The floor of a binary of Rust is decided by three things, and the three
of them are needed. The table is the one of the **probe**, the binary
without the standard library; the `ahk.exe` of the tree takes the
other way -- the `win7` target with the `std` emended -- and it is in
[O andar de cima](#o-andar-de-cima-o-windows-xp-como-alvo-de-verdade):

| What | Which | Why |
|---|---|---|
| the target | `i686-pc-windows-gnu` | the CRT of the target is the `msvcrt.dll` of Windows, the one that exists since NT, and a binary of it has no api set |
| the standard library | **none** (the probe) | the `std` of Rust imports `api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`, Windows 8) and the Universal CRT, and the loader of XP refuses the binary before the first instruction. The `ahk.exe` emends the `std` instead of dropping it (`patch_std.py`) |
| the C toolchain | the MinGW of thirty-two bits with the `msvcrt` runtime | the link line of the target asks for `-lgcc_eh`, and `w64devkit` -- the MinGW of the machine -- ships no `libgcc_eh.a`: its `lib32` has the import libraries and `libgcc.a`, and no unwinder. The link fails with `cannot find -lgcc_eh` |

The compiler is a **nightly with the source of the standard library**,
and not a stable one, and the reason is measured: the `core` that
ships with the target is built with the unwinding on, and it asks for
`rust_eh_personality` in its `.eh_frame`. A binary without `std` has
no personality routine, and the link fails with `undefined reference to
rust_eh_personality` as soon as the program touches the parts of
`core` that carry it (the strings of UTF-8, for one).
`-Z build-std=core,panic_abort` builds `core` again with the aborting
panic, and the reference is gone.

The setup of the machine:

```sh
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
rustup target add i686-pc-windows-gnu --toolchain nightly
```

And the MinGW of thirty-two bits with the `msvcrt` runtime: the
[winlibs](https://github.com/brechtsanders/winlibs_mingw) build
`winlibs-i686-posix-dwarf-gcc-16.2.0-mingw-w64msvcrt-14.0.0-r3.7z`,
unpacked in `$HOME/tools/mingw32` (or anywhere, with `AHK_XP_MINGW`
pointing at it). The `posix` flavour is the one the target wants
(`-l:libpthread.a`), and the `msvcrt` flavour is the one XP wants.

```sh
curl -L -o winlibs-i686-msvcrt.7z \
  https://github.com/brechtsanders/winlibs_mingw/releases/download/16.2.0posix-23.1.3-14.0.0-msvcrt-r3/winlibs-i686-posix-dwarf-gcc-16.2.0-mingw-w64msvcrt-14.0.0-r3.7z
7z x -y winlibs-i686-msvcrt.7z -o"$HOME/tools/"
```

**No Rust runs on XP.** The driver of the compiler imports
`api-ms-win-core-synch-l1-2-0.dll` (`WaitOnAddress`, Windows 8),
`GetSystemTimePreciseAsFileTime` (Windows 8), `InitializeCriticalSectionEx`
(Vista), `IsThreadAFiber` (Windows 8) and `CreateWaitableTimerExW`
(Windows 7): the loader of XP refuses the program before its first
instruction. And the tree needs `rustc` 1.85 at least (the edition of
2024), so no compiler of the tree can live in the guest. The toolchain
is on the machine of the tree, and the guest only ever runs what it
builds.

---

## The build

```sh
powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/build.ps1
```

It builds `scripts/xp/probe/Cargo.toml` for `i686-pc-windows-gnu` with
the nightly and `-Z build-std=core,panic_abort`, and it runs the gate
over what it built. The `-Manifest` parameter is the manifest of the
program to build: the probe, or the floor probe
(`scripts/xp/floor/Cargo.toml`). The runner of the tree is not built
by this script: it is the `ahk-cli` with `--features xp`, built by the
command of [O andar de cima](#o-andar-de-cima-o-windows-xp-como-alvo-de-verdade).

---

## The gate

The floor of a binary is its import table, and nothing else. The gate
reads the table of a PE and fails, naming the reason, when:

- the machine of the binary is neither the thirty-two bits one nor the
  sixty-four bits one;
- the subsystem version is newer than the one of XP (5.1);
- a DLL of the api sets (`api-ms-win-*`, `ext-ms-*`) is in the table;
- a DLL outside the allowlist is in the table;
- a symbol is in the table that the machine said it does not have;
- a symbol is in the table that nobody asked the machine about -- the
  floor is an oracle of three answers (measured present, measured
  missing, never asked), and it fails on the third too.

The authority is the measured floor (`floor-xp-sp3.txt`), not a list
of quotations.

## The allowlist

The allowlist of the DLLs is the measurement of the last AutoHotkey v1
(1.1.37.02): the fourteen classic DLLs of its table, plus `msvcrt.dll`,
plus the three the tree asks and XP SP3 has (measured on the XP guest
of the lab): `iphlpapi.dll`,
`userenv.dll`, `ws2_32.dll`.

---

## The probe

`scripts/xp/probe` is the smallest program that proves the floor: a
binary of Rust without the standard library, for
`i686-pc-windows-gnu`, that asks the machine three questions and
writes the answers on the console.

---

## The floor, measured on the machine

`scripts/xp/floor` is the probe that asks the machine the whole
question. It carries the table of the imports of the binaries of the
tree and it asks the XP machine for each of them, one at a time, with
`GetModuleHandleA` and `GetProcAddress`. It asks the classes of COM as
well, with `CoCreateInstance`: a class is not a symbol, and the floor of
a class is the registry of the machine.

```sh
# the table: the UNION of every binary that goes to the machine
python scripts/xp/floor/gen_table.py \
    target/i686-win7-windows-gnu/release/ahk.exe \
    /tmp/helloxp/target/i686-win7-windows-gnu/release/hello.exe \
    scripts/xp/floor/target/i686-pc-windows-gnu/release/ahk-xp-floor.exe \
    scripts/xp/probe/target/i686-pc-windows-gnu/release/ahk-xp-probe.exe \
    > scripts/xp/floor/src/table.rs

# the probe, and the gate over it
powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/build.ps1 \
    -Manifest scripts/xp/floor/Cargo.toml

# the probe on the machine, and the answer back through the floppy
python scripts/xp/xpctl.py put scripts/xp/floor/target/i686-pc-windows-gnu/release/ahk-xp-floor.exe
python scripts/xp/xpctl.py console 'cmd' 'a:\ahk-xp-floor.exe > a:\floor.txt'
```

The probe of sixty-four bits is the same program for the other
architecture: the table is the imports of the `ahk.exe` of
`x86_64-win7-windows-gnu` (`table64.rs`, which the probe selects by
`target_arch`), the linker is the `w64devkit` with the empty stub of
`libgcc_eh`, and the machine is the guest `WindowsXP64`:

```sh
python scripts/xp/floor/gen_table.py \
    target/x86_64-win7-windows-gnu/release/ahk.exe \
    > scripts/xp/floor/src/table64.rs

powershell -NoProfile -ExecutionPolicy Bypass scripts/xp/build.ps1 \
    -Manifest scripts/xp/floor/Cargo.toml -Target x86_64-pc-windows-gnu \
    -Mingw "$HOME/scoop/apps/w64devkit/current" -NoGate

XP_VM=WindowsXP64 python scripts/xp/xpctl.py put scripts/xp/floor/target/x86_64-pc-windows-gnu/release/ahk-xp-floor.exe
XP_VM=WindowsXP64 python scripts/xp/xpctl.py console 'cmd' 'a:\ahk-xp-floor.exe > a:\floor64.txt'
```

**The union is the point.** A table taken from `ahk.exe` alone leaves
the gate unable to vouch for the minimal `hello` that is the reproducer
of the lab — and `hello` imports *more* than `ahk.exe` does (207 symbols
against 303, and 44 of them nobody else asks for), because the
`--gc-sections` of the tree cuts from `ahk.exe` the sections it never
references. The sockets of `ws2_32` and the `userenv` are among them.

**The answer of the machine** (the evidence is
`scripts/xp/floor-xp-sp3.txt`):

```
machine: windows 5.1 build 2600 sp3
symbols: 424   missing: 0
com CLSID_FileOpenDialog: NOT registered, the answer is 0x80040154
com CLSID_MMDeviceEnumerator: NOT registered, the answer is 0x80040154
com CLSID_ShellLink: registered
```

**424 asked, 424 present, none missing.** Nothing that the
thirty-two-bit binaries of this tree take to the machine is a thing the
machine lacks. The gate now passes with exit 0 on all four of them —
`ahk.exe`, `hello.exe` and the two probes.

**The measurement of the 25th of November of 2026.** The table grew with
the work of the GUI and of the walk of the parent: the tree takes
`user32.dll!GetAncestor` to the machine now, and the forty-one symbols
that the work had added were recorded **by hand**, with a note that said
that the probe was to be run again. The probe ran on the machine of the
lab, and the answer of the machine is the answer above: the forty-two are
present, and the `GetAncestor` with them. The oracle is measured, and not
guessed.

The channel of that measurement is the one of the lab: the probe travels
on the floppy of the guest, and the answer of it comes back on the floppy
too. The guest needs a floppy with room -- the one that was there was
full -- and the keyboard of the QEMU monitor, because the clicks of the
mouse do not land in the guest (the tablet of the USB is at the input 0
and the mouse of the PS/2 at the input 1, and the monitor drives the
second one).

**The answer of the machine of sixty-four bits** (the evidence is
`scripts/xp/floor-xp-x64.txt`):

```
machine: windows 5.2 build 3790 sp2
symbols: 302   missing: 0
com CLSID_FileOpenDialog: NOT registered, the answer is 0x80040154
com CLSID_MMDeviceEnumerator: NOT registered, the answer is 0x80040154
com CLSID_ShellLink: registered
```

**302 asked, 302 present, none missing.** The table is the imports of
the `ahk.exe` of `x86_64-win7-windows-gnu`, and the eight symbols that
only exist as such on a sixty-four-bit Windows are in the answer. The
gate reads the two floors together, and passes on the binary of
sixty-four bits as well.

The three classes say what the tree does without the COM of Vista:
the folder dialog (`IFileDialog`) and the sound (WASAPI) are **Vista**,
and the shell link is **XP**. On XP the tree answers the picker with
the common dialogs of comdlg32 (`windows/picker.rs`) and the volume
with the mixer API of winmm (`windows/mixer.rs`) -- the way of the v1,
measured on the two guests in both dialects.

**The first measurement, and the defect it had.** The first time, the
table was the 372 symbols of the tree built for `i686-pc-windows-gnu`
with the `std` as it ships, and the machine said that 29 of them were
missing. Those 29 are not in this table because they are no longer
imported: `patch_std.py` emends the `std` for the era of XP, and the
tree gave up the five of Vista that it called.

The first measurement also wrote **only the MISSING lines**, and that
was a defect of the probe rather than a choice. The gate is an oracle of
*three* answers — measured missing, measured present, never asked about
— and it fails on the third, because a floor that was not measured is a
floor that is guessed. With an empty set of the present ones, every
symbol of every binary came back "was not asked about the machine", and
the gate could vouch for nothing **even when the binary ran**. The probe
now writes both halves, and the floor file carries both.

**The sixty-four bits have their own machine.** The binary of
`x86_64-win7-windows-gnu` carries eight symbols the table of the
thirty-two bits cannot speak for, because on the thirty-two-bit
Windows they do not exist as such:

| the symbol of x64 | because |
| --- | --- |
| `user32.dll!GetWindowLongPtrW`, `SetWindowLongPtrW` | on 32 bits they are macros to `GetWindowLongW` |
| `kernel32.dll!RtlUnwindEx`, `__C_specific_handler` | the SEH of x64 |
| `ntdll.dll!RtlLookupFunctionEntry`, `RtlVirtualUnwind` | the unwinding of x64 |
| `msvcrt.dll!__initenv`, `__iob_func` | the CRT of x64 |

The documents put all eight in XP x64 and Server 2003 x64 — and the
machine of the lab confirms it: the guest `WindowsXP64` (Windows XP
Professional x64 Edition with SP2, 5.2 build 3790, installed from
`en_win_xp_pro_x64_with_sp2_vl_x13-41611.iso`) answers the probe built
for `x86_64-pc-windows-gnu`, and the eight are present. The probe of
thirty-two bits carries `table.rs`; the probe of sixty-four bits
carries `table64.rs` — the imports of the `ahk.exe` of each
architecture, written by `gen_table.py`. The answer of the second is
`scripts/xp/floor-xp-x64.txt`.

---

### The first measurement, as it was written

**The answer of the machine, the first time**:

```
symbols: 372   missing: 29
com CLSID_FileOpenDialog: NOT registered, the answer is 0x80040154
com CLSID_MMDeviceEnumerator: NOT registered, the answer is 0x80040154
com CLSID_ShellLink: registered
```

The twenty-nine are the symbols that XP SP3 does not have.

**And the other half of the answer is the interesting half.** The
symbols that the documents put after XP and that the machine *has* are
wrong in the documents, and the gate now follows the machine:

| Symbol | The documents said | The machine says |
|---|---|---|
| `GetAdaptersAddresses` | Vista | **XP has it** (iphlpapi.dll) |
| `SetWinEventHook` | Vista | **XP has it** (user32.dll) |
| `InitializeCriticalSectionEx` | Vista | **XP has it** |
| `CancelIo` | Vista | **XP has it** |
| `SwitchToThread` | Vista | **XP has it** |
| `WaitForSingleObjectEx` | Windows 7 | **XP has it** |
| `GetFileTime` | Windows NT | **XP has it** |

So the gate's table (`scripts/xp/gate.py`) is the measurement now, and
not a list of quotations: it failed on the twenty-nine that the machine
does not have, and it let the seven above through.

*Those twenty-nine are history now*: the second measurement does not see
them, because the patched `std` and the tree stopped importing them. They
stay recorded here, and in the header of `floor-xp-sp3.txt`, as the list
of what the emend of the `std` had to answer for.

**The symbols of the tree itself that XP does not have are five**, and
they are the whole of the era of the modern backend:

| Symbol | Brought by | The site in the tree |
|---|---|---|
| `SHGetKnownFolderPath` | Vista | `windows/machine.rs` |
| `SHCreateItemFromParsingName` | Vista | `windows/dialogs.rs` |
| `GetDpiForMonitor` | Windows 8.1 | `windows/screen.rs` |
| `GetDpiForSystem` | Windows 10 1607 | `windows/screen.rs` |
| `SetProcessDpiAwarenessContext` | Windows 10 1703 | `windows/screen.rs` |

The other twenty-four are the standard library of Rust (`std`): they
are the reason the XP build carries no `std` at all.

---

## The tree, on thirty-two bits

The tree builds for `i686-pc-windows-gnu` today, and it did not, for
three reasons, all of them measured:

1. **`windows/hotkeys.rs` had a sixty-four bits assumption.** The
   key and the modifiers of a hotkey travel in the `lparam` of the
   message of the box, the key in the high half and the modifiers in
   the low one; on a thirty-two bits machine the `lparam` has no high
   half, and the build stops with *"attempt to shift right by 32_i32,
   which would overflow"*. The packing is now the width of the machine
   (`packed`/`unpacked` in that file): sixty-four bits, the key in the
   high thirty-two; thirty-two bits, the key in the high sixteen and
   the modifiers in the low sixteen (five flags fit in sixteen bits ✓).
2. **The manifest of the tree was linked on every Windows target.**
   `/MANIFEST:EMBED` is a flag of the linker of Microsoft, and the
   linker of the target of the GNU (mingw) is the one of GNU: it
   refuses the flags, and the link fails with *"cannot find
   /MANIFEST:EMBED"*. The flags go to the MSVC target only now
   (`crates/ahk-box/build.rs`, `crates/ahk-cli/build.rs`).
3. **The linker of the machine was the wrong one.** The build of the
   tree asks for `-lgcc_eh`, and `w64devkit` (the MinGW of the machine)
   ships no `libgcc_eh.a`. The build of thirty-two bits uses the
   MinGW of the scripts of the floor.

```sh
export PATH="$HOME/tools/mingw32/bin:$PATH"
cargo build -p ahk-cli --target i686-pc-windows-gnu --release
python scripts/xp/gate.py target/i686-pc-windows-gnu/release/ahk.exe
```

The gate fails on that binary, and it is right to fail: the table of
the modern tree asks for the api sets and for the twenty-nine symbols
that XP does not have. That binary is the *input* of the floor probe,
not a candidate for XP.

---

## What was measured

On the XP machine of the lab (Windows XP Professional SP3, 5.1 build
2600), with the probe built by the nightly of the tree and linked by
the MinGW of thirty-two bits with the `msvcrt` runtime:

```
C:\Documents and Settings\User>c:\probe.exe
ahk-xp-probe: alive
windows: 5.1 build 2600 sp3
tick: 2357171
pid: 1516
rust: no_std, i686-pc-windows-gnu, panic=abort
```

And the table of the imports of that binary:

```
kernel32.dll   51 symbol(s)     (InitializeCriticalSection, GetVersionExW,
                                 GetTickCount, GetSystemTimeAsFileTime ...)
msvcrt.dll     43 symbol(s)     (malloc, free, memcpy, fprintf, exit ...)
```

No api set, no symbol newer than XP SP3, subsystem version 4.0. The
gate passes.

## O andar de cima: o Windows XP como alvo de verdade

O que está descrito acima é o *piso medido*: a máquina do laboratório
diz quais funções da API ela tem. O que vem abaixo é o *andar de
cima*: como o binário da árvore passa a ser um programa de Windows XP
de verdade, e o que ainda falta.

### A compiling com o alvo certo

O `std` do Rust é escrito para o Windows de hoje, e ele pede à máquina
funções que o Windows XP não tem. O Rust tem alvos oficiais para isso:

```text
i686-win7-windows-gnu      os trinta e dois bits
x86_64-win7-windows-gnu    os sessenta e quatro bits
```

Com `target_vendor = "win7"` o `std` deixa de importar o
`api-ms-win-core-synch-l1-2-0.dll` (o conjunto de API não existe no
XP) e passa a carregar o `WaitOnAddress` em tempo de execução. O
comando do binário do laboratório é:

```bash
cargo +nightly build \
  -Z build-std=std,core,alloc,panic_abort \
  -p ahk-cli --features xp \
  --target i686-win7-windows-gnu --release
```

### O `std` emendado para o piso

Dezoito funções do `std` são de Windows Vista ou de depois, e elas
entram na tabela de importações como `raw-dylib`: o carregador do XP
procura cada nome no kernel32, o kernel32 do XP não os tem, e o
binário não começa. O script `scripts/xp/patch_std.py` emenda o `std`
da ferramenta para a era do XP:

| a função de Vista | o que o `std` passa a usar |
| --- | --- |
| `FlsAlloc`, `FlsSetValue`, `FlsFree` | `TlsAlloc`, `TlsSetValue`, `TlsFree` |
| `IsThreadAFiber` | `false` (o XP não tem fibras) |
| `AcquireSRWLockExclusive` e irmãs | um *spin lock* de átomos |
| `CreateWaitableTimerExW` | nada: o `std` usa o `Sleep` |
| `SetThreadStackGuarantee` | nada |
| `GetFileInformationByHandleEx` | `GetFileInformationByHandle` |
| `GetFinalPathNameByHandleW` | nada: a árvore canonicaliza com o `GetFullPathNameW` |
| `SetFileInformationByHandle` | nada: o arquivo fica onde está |
| `CompareStringOrdinal` | a comparação dos números, com a dobra da caixa |

O `std` emendado é recompilado pelo `-Z build-std`, e a impressão
digital do cargo não muda quando o `std` muda: para que a emenda
entre no binário, a pasta `build` do alvo tem de ser apagada.

### A árvore sem as funções de Vista

Sete chamadas da própria árvore eram de Vista ou de depois, e elas
foram trocadas por chamadas que o XP tem, ou por perguntas ao nome da
função em tempo de execução (`super::sys::the_symbol_of`):

| a chamada | o que a árvore faz agora |
| --- | --- |
| `SHGetKnownFolderPath` | o nome é perguntado ao shell32; sem ele, a pasta clássica do XP |
| `SHCreateItemFromParsingName` | o nome é perguntado ao shell32; sem ele, o recusa |
| `GetDpiForMonitor` | o nome é perguntado ao shcore; sem ele, o 96 |
| `GetDpiForSystem` | o nome é perguntado ao user32; sem ele, o 96 |
| `SetProcessDpiAwarenessContext` | o nome é perguntado ao user32; sem ele, nada |
| `GetUserDefaultLocaleName` | `GetLocaleInfoW` |
| `QueryFullProcessImageNameW` | `GetModuleFileNameExW` do psapi |

O `Condvar` do `std` também era de Vista (o
`SleepConditionVariableSRW`), e a espera das hotkeys passou a usar o
`CreateEventW` do kernel32, que é de Windows 2000.

### O gate que pergunta à máquina

O gate (`scripts/xp/gate.py`) já não confia numa lista escrita à mão:
ele lê o piso medido (`scripts/xp/floor-xp-sp3.txt`) e falha quando o
binário pede uma função que a máquina **não tem** — e também quando
pede uma função sobre a qual **ninguém perguntou**. Um símbolo não
medido é um símbolo desconhecido, e o piso é medido de novo.

### O que está pronto e o que falta

Pronto e medido:

- o binário de **trinta e dois bits** e o de **sessenta e quatro bits**
  compilam, e a tabela de importações dos dois **passa o gate**;
- o binário roda o **v1** e o **v2** no host, e o relatório é o mesmo
  nos dois;
- o binário roda o **v1** e o **v2** nos dois guests do laboratório
  (o XP SP3 de trinta e dois bits e o XP x64 SP2), e o relatório volta
  pelo disquete;
- o **picker** e o **volume** respondem no XP: os common dialogs do
  comdlg32 (`windows/picker.rs`) e o mixer do winmm (`windows/mixer.rs`)
  entram no lugar do `IFileDialog` e do WASAPI, que são de Vista --
  medido nos dois guests, nos dois dialectos;
- a máquina do laboratório lê a imagem do laboratório (o script
  `scripts/xp/mkiso.py` monta o ISO9660) e o binário é copiado para o
  disco do XP;
- o **release** e o **CI**: `scripts/xp/release.ps1` constrói e passa o
  gate nos dois alvos com o nightly pinado, o workflow de release faz
  os assets `i686-windows-xp` e `x86_64-windows-xp`, e o job
  `windows-xp` do CI passa o gate a cada push;
- o **caderno de comportamento** medido no guest: 1051 casos v1 + 113
  v2, cada um no seu processo, por `scripts/xp/driver.bat`. v1
  XP-árvore contra host-árvore **1051/1051**; contra o v1.1.37.02 real
  no XP 1025/1051 (os 26 são lacunas árvore-vs-v1 que o host já
  carrega). v2 **113/113**. Os últimos 7+1 diffs eram o `read_dir` do
  std a abrir a pasta com `FindFirstFileExW(FindExInfoBasic)`, que o XP
  recusa (erro 87); o `patch_std.py` emenda para `FindExInfoStandard`
  (era do Windows 2000) e o caderno fica 100% sem curadoria.

### O canal, e a pergunta do FTP

O guest é air-gapped pelo unattend, por isso o primeiro canal foi o
disquete. O disquete funciona mas é frágil para volume: o FAT12
corrompe com milhares de ficheiros e o QEMU só grava as escritas do
guest quando a mídia é ejectada. O padrão robusto é empacotar os
resultados em poucos ficheiros grandes, ou sair do disquete.

O NIC passou de `e1000` (sem driver in-box no XP) para `pcnet`, que o
XP conduz in-box; o TCP/IP foi instalado e um endereço estático
atribuído (`192.168.250.2` na `xp-isolated`). **O FTP funciona** e é o
canal de volume: servidor `pyftpdlib` no NAS, `ftp.exe` do guest como
cliente (`scripts/xp/ftp.md`, `scripts/xp/ftp-measure.py`). Uma
ressalva: `mget`/`mput` não enumeram contra este servidor, por isso os
scripts usam um `get`/`put` por ficheiro. O disquete fica para os
lotes pequenos de controlo; o ISO (`scripts/xp/mkiso.py`) e um disco
de dados gravável com partições são as alternativas offline.

### A causa do abort antes do `main`, achada

O `hello` mínimo morria no XP com zero bytes de saída, e a leitura
errada era a de que uma emenda do `patch_std.py` abortava em tempo de
execução. Não é. **Não havia nada a bisseccionar**: o que mata o
binário é a tabela de importações, que o carregador resolve antes da
primeira instrução. O kernel32 do XP SP3 não tem os nomes, o processo
morre sem correr uma linha, e a saída fica vazia.

A medição é esta: o `hello` compilado de novo, com o `std` da
ferramenta como está hoje, importa estaticamente oito funções que o XP
não tem.

| o símbolo estático | onde o `std` ainda o chama | o piso |
| --- | --- | --- |
| `GetFileInformationByHandleEx` | `sys/io/is_terminal/windows.rs:77` | medido: XP não tem |
| `GetFileInformationByHandleEx` | `sys/fs/windows.rs:552,584,820,892` | medido: XP não tem |
| `CreateSymbolicLinkW` | `sys/fs/windows.rs:1445,1456` | medido: XP não tem |
| `InitializeProcThreadAttributeList` | `os/windows/process.rs:713,726` | medido: XP não tem |
| `UpdateProcThreadAttribute` | `os/windows/process.rs:740` | medido: XP não tem |
| `DeleteProcThreadAttributeList` | `os/windows/process.rs:556` | medido: XP não tem |
| `SleepConditionVariableSRW` | `sys/sync/condvar/windows7.rs:21,26` | nunca perguntado |
| `WakeConditionVariable` | `sys/sync/condvar/windows7.rs:42` | nunca perguntado |
| `WakeAllConditionVariable` | `sys/sync/condvar/windows7.rs:47` | nunca perguntado |

Três coisas saem daí.

**1. O `patch_std.py` nunca cobriu três arquivos.** O escopo dele é
`sys/fs/windows.rs`, `sys/pal/windows/api.rs`,
`sys/pal/windows/stack_overflow.rs`, `sys/pal/windows/time.rs`,
`sys/process/windows.rs` e `sys/thread_local/guard/windows.rs`. Ficaram
de fora:

- `sys/sync/condvar/windows7.rs` — o `Condvar` inteiro é de Vista
  (`SRWLock` + `SleepConditionVariableSRW`). A árvore contornou o
  `Condvar` nas hotkeys com o `CreateEventW`, mas a `std` continua a
  usá-lo dentro de si, e o `println!` do `hello` chega lá.
- `sys/io/is_terminal/windows.rs` — é o caminho do `stdout`/`stderr`:
  perguntar se o descritor é um terminal. É a primeira coisa que um
  `println!` toca, e é por isso que o `hello` morre e o `ahk` com
  `--features xp` não: o `--gc-sections` corta do `ahk` as secções que
  ele não referencia.
- `os/windows/process.rs` — o trio `ProcThreadAttributeList`. O script
  emendava `sys/process/windows.rs`, que é outro arquivo; este nunca
  entrou no escopo.

**2. O `gate.py` escondia a resposta.** Na linha que denuncia um
símbolo medido como ausente, o formato tinha dois `%s` e três
argumentos, e o `gate` caía com `TypeError: not all arguments converted`
no lugar de imprimir o nome. Corrigido, ele diz os oito. É por isso que
as corridas anteriores do `gate` sobre o `ahk.exe` pareciam limpas: o
erro só se manifesta quando há mesmo um símbolo ausente para anunciar.

**3. O piso tem um buraco.** `SleepConditionVariableSRW`,
`WakeConditionVariable` e `WakeAllConditionVariable` nunca foram
perguntados à máquina (e `AcquireSRWLockExclusive` e irmãs também não —
o `ahk.exe` de sessenta e quatro bits importa as três). São de Vista,
mas o `floor-xp-sp3.txt` é a autoridade do gate, e o que ele não mede
ele não cobra.

### As emendas que faltavam, escritas — e o `hello` que corre no XP

Bisseccionar não era o caminho; escrever as emendas que faltavam era.
Estão todas no `patch_std.py`, e o `hello` mínimo **corre agora no
Windows XP SP3**. A leitura do ecrã do convidado, com o binário a correr
directamente do disquete:

```
C:\Documents and Settings\user>a:\hello.exe
HELLO: o programa comecou
HELLO: o mundo
HELLO: o programa terminou
```

As emendas novas, símbolo por símbolo:

| o arquivo do `std` | o que era de Vista | o que entrou no lugar |
| --- | --- | --- |
| `sys/sync/condvar/windows7.rs` | `SleepConditionVariableSRW`, `WakeConditionVariable`, `WakeAllConditionVariable` | o arquivo inteiro, reescrito sobre o evento de reposição automática do Windows 2000 (`CreateEventW` + `WaitForSingleObject` + `SetEvent`), com uma contagem de esperas para o `notify_all` |
| `sys/io/is_terminal/windows.rs` | `GetFileInformationByHandleEx` (o truque do msys lê o nome do cano) | a função `msys_tty_on` fica com o corpo vazio — não basta tirar a chamada, porque o `raw-dylib` põe o nome na tabela enquanto houver referência |
| `os/windows/process.rs` | o trio `ProcThreadAttributeList` | o construtor recusa com `Unsupported`, e as três chamadas somem |
| `sys/fs/windows.rs` (`symlink_inner`) | `CreateSymbolicLinkW` | `Unsupported` — o XP tem o hard link e mais nada |
| `sys/fs/windows.rs` (`basic_info`) | `GetFileInformationByHandleEx` | `GetFileInformationByHandle`, de Windows 2000: os mesmos atributos e as mesmas três horas, com o `ChangeTime` a zero |
| `sys/fs/windows.rs` (`fill_dir_buff`) | `GetFileInformationByHandleEx` | `NtQueryDirectoryFile` do ntdll, classe 8 — o mesmo `FILE_ID_BOTH_DIR_INFO` que o `DirBuffIter` lê, e que existe desde o NT 3.1 |
| `sys/pal/windows/c/windows_sys.rs` | — | as ligações que faltavam: `SetEvent`, `ResetEvent`, `NtQueryDirectoryFile` |

Duas notas de método que custaram tempo e valem ficar escritas.

**Uma emenda só é idempotente se o texto velho deixar de caber no texto
novo.** As primeiras versões destas emendas acrescentavam uma linha
*antes* de um texto que continuava lá — e cada corrida do script
acrescentava outra. Duas delas duplicaram o `SetEvent` (erro de
compilação) e empilharam quatro `#[allow(dead_code)]`. A regra agora é
ancorar nas duas linhas, a de cima e a de baixo, ou substituir o
arquivo inteiro, como faz o `Condvar`.

**O `#[allow(dead_code)]` não tira um símbolo da tabela de
importações.** Cala o aviso, e só. Enquanto houver uma referência ao
`raw-dylib`, o nome entra. Foi preciso apagar o corpo da função.

O gate passa: nenhum símbolo de Vista. O que ele ainda aponta é só a
falta de cobertura do piso — `OpenProcessToken`, `CloseHandle`,
`AddVectoredExceptionHandler`, `SystemFunction036` e outros são de NT
3.1/2000 e nunca foram perguntados à máquina, e o `floor-xp-sp3.txt`
é a autoridade do gate. Medir o piso de novo é a conta que fica em
aberto, não uma falha do binário.

A cópia do `std` emendado continua a precisar de
`rm -rf target/<alvo>/release/build`, porque a impressão digital do
cargo não muda quando o `std` muda.

### A ferramenta de sessenta e quatro bits, e o binário que rebentava

O `ahk.exe` de sessenta e quatro bits ligava e passava no gate, e
rebentava com `SIGSEGV` antes de imprimir uma linha. O `gdb` punha a
queda em `std::rt::lang_start_internal+38` — o arranque do `std`, antes
do `main`, exactamente o mesmo sítio onde o XP matava o `hello`.

Não eram as emendas. A experiência que separa as coisas é construir o
`hello` para o alvo **normal** `x86_64-pc-windows-gnu`, com o `std`
pré-compilado e virgem, sem `-Z build-std`: rebenta igual. Um programa
C de sessenta e quatro bits, com a mesma ferramenta, corre. Logo o que
estava partido era o ligador, não o `std`.

| a ferramenta de 64 bits | `libgcc_eh.a` | o resultado |
| --- | --- | --- |
| `strawberry/c` (gcc 13.2.0) | tem, de verdade | **liga e o binário rebenta** em `lang_start_internal` |
| `w64devkit` | não tem | o link falha: `cannot find -lgcc_eh` |
| `w64devkit` + o stub vazio | o stub de 8 bytes | **corre** |

O comando que funciona é o do `w64devkit` com o stub, e o detalhe que
custa tempo é o formato do caminho: o ligador é do Windows e não
entende `/c/Users/...`, só `C:/Users/...`.

```bash
export PATH="/c/Users/<user>/scoop/apps/w64devkit/current/bin:$PATH"
rm -rf target/x86_64-win7-windows-gnu/release/build
RUSTFLAGS="-C link-arg=-LC:/Users/<user>/tools/gcc_eh_stub" \
cargo +nightly build -Z build-std=std,core,alloc,panic_abort \
  -p ahk-cli --features xp --target x86_64-win7-windows-gnu --release
```

O `-C lto=fat` não entra aqui: choca com o `-C embed-bitcode=no` que o
`-Z build-std` impõe.

Com isto, os dois binários da árvore ligam, correm no host e passam no
gate sem nenhum símbolo de Vista:

| o alvo | o tamanho | de Vista | o piso não cobre |
| --- | --- | --- | --- |
| `i686-win7-windows-gnu` | 2 679 296 | 0 | **0 — o gate passa, exit 0** |
| `x86_64-win7-windows-gnu` | 4 534 032 | 0 | **0 — o gate passa, exit 0** |

O tamanho de trinta e dois bits é o do binário com `-C strip=symbols
-C opt-level=z`, que é o que coube no canal do disquete; sem eles são
4 446 246 bytes.

Os oito que só existem como tais no Windows de sessenta e quatro bits
estão medidos na máquina de sessenta e quatro bits: as duas
`GetWindowLongPtrW`/`SetWindowLongPtrW`, o trio do *unwinding* de x64,
o `__C_specific_handler` do SEH e os dois do CRT — todos presentes no
XP x64 SP2, e o gate lê os dois pisos juntos.

O binário de sessenta e quatro bits corre no laboratório: o guest
`WindowsXP64` é o XP x64 SP2, e o `ahk.exe` de 4,53 MB (quatro metades
de disquete) corre nele o v1 e o v2, com o relatório a voltar pelo
disquete como o de trinta e dois bits.

### O canal do laboratório, como ele realmente é

O que ficou aprendido a operar a máquina, e que não estava escrito:

- **o disquete vinha com `<readonly/>` no XML do libvirt**, e era isso o
  *Write Protect Error* — não uma limitação do XP. Tirar a linha do
  `fda` (`virsh dumpxml`, filtrar, `virsh define`, e reiniciar o domínio
  para a definição entrar) abre a única via determinística do
  laboratório: o convidado escreve em `A:` e o resultado lê-se do
  ficheiro da imagem, com `mcopy`, sem OCR nenhum.
- **o OCR é um olho fraco.** O ecrã do convidado é de 640×480, a linha
  do relatório JSON enrola, e o `tesseract` devolve fragmentos. Serve
  para saber se algo apareceu; não serve para ler o resultado.
- **o `change-media` do `xpctl put` não chega.** O caminho do ficheiro
  é o mesmo e o QEMU não o reabre: o convidado continua a ver o
  conteúdo antigo. É preciso ejectar e inserir, de preferência para um
  nome novo (`cp xp-floppy.img xp-fl-<letra>.img`, depois
  `virsh change-media WindowsXP fda --eject --live` e
  `--insert --live`).
- **um binário maior que 1,44 MB vai em metades.** O `ahk.exe` de
  2,68 MB passou assim: `split -b 1300000`, uma imagem de disquete por
  metade (três, desta vez), `copy a:\ahk1.bin c:\p1` em cada uma, e no
  fim `copy /b c:\p1+c:\p2+c:\p3 c:\ahk.exe`. O `copy /b` **pergunta**
  se sobrescreve quando o destino já existe, e a pergunta fica à espera
  na consola: `del c:\ahk.exe` primeiro.
- **o `-C strip=symbols -C opt-level=z` encolhe o `ahk.exe` de 4,45 MB
  para 2,68 MB**, e é o que o faz caber em três metades. O
  `-C lto=fat` não serve aqui: choca com o `-C embed-bitcode=no` que o
  `-Z build-std` impõe.
- **o CD não é uma via fiável.** O `hda` é o CD de instalação do XP na
  cabeça do convidado, e a troca do ISO não chegou lá.
- **o `cmd` do `xpctl` escreve na consola que estiver aberta**, e as
  janelas velhas ficam por cima. O `console` (Win+R) abre uma nova, e é
  o caminho firme. Quando a máquina se enche de diálogos modais — e ela
  enche: o balão do *Windows Update*, o do *antivirus*, o do *Found New
  Hardware*, o da resolução baixa — só o `virsh destroy` +
  `virsh start` limpa.

### O `ahk.exe` no XP, nos dois dialectos

Com o binário de 2,68 MB rejunto em `c:\ahk.exe` e o script de prova em
`c:\t.ahk`, o relatório que voltou pelo disquete, lido da imagem com
`mcopy` e não do ecrã:

```json
{"ok":true,"steps":[{"line":1,"op":"Assign","detail":"v = 42","ok":true,"elapsed_ms":0},{"line":2,"op":"Assign","detail":"s = xp!","ok":true,"elapsed_ms":0}],"output":[],"warnings":[],"notes":[],"error":null,"error_line":null,"exit_code":null,"elapsed_ms":5,"variables":{"a_linefile":"c:\\t.ahk","a_linenumber":2,"s":"xp!","v":42}}
```

O `--dialect=v1` devolve o mesmo relatório, com `elapsed_ms:1`. O
`"a_linefile":"c:\\t.ahk"` é a prova de que foi mesmo no convidado, e
não no host: é o caminho do disco do XP.

Correm, portanto, no Windows XP SP3 de trinta e dois bits: o `hello`
mínimo, o `ahk.exe` em v2 e o `ahk.exe` em v1. O de sessenta e quatro
bits liga, corre no host e passa no gate, mas a VM do laboratório não o
pode provar.
