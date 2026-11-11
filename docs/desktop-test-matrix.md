# The desktops to test the tree on

The tree of the AHK is the same language on every machine, and what
changes from one machine to another is the desktop underneath it: the
compositor that lists the windows, the bus of the accessibility that
carries the controls of them, the toolkit that draws the window of the
script, and the X server of the sessions that are not Wayland.

This file says **what the tree asks of the desktop**, **which desktop
answers what**, and **how one VM can carry several of them** -- so that a
change can be proved against more than one compositor instead of against
the one that happens to be on the box.

## 0. What the tree asks of the desktop

| the family of the port | what it asks | where it is asked |
|---|---|---|
| the windows of the machine (`Win*`) | `ext_foreign_toplevel_list_v1` (version 1) -- the portable listing of the toplevels | `crates/ahk-box/src/wayland/linux.rs` |
| the same | `zcosmic_toplevel_info_v1` (version 2 or 3) -- the state of a window: the focus, the geometry, the maximized and the minimized | `crates/ahk-box/src/wayland/linux.rs` |
| the same | `wl_output` (version 4 or less) -- the outputs, their modes and their scale | `crates/ahk-box/src/wayland/linux.rs` |
| the same | `ext_workspace_manager_v1` -- the workspaces | `crates/ahk-box/src/wayland/linux.rs` |
| the controls of another window (`Control*`) | the bus of the accessibility: `org.a11y.Bus`, and the `org.a11y.atspi` objects of the apps | `crates/ahk-box/src/atspi.rs` (through `busctl`) |
| the window of the script (`Gui`) | **GTK 3** -- the helper is a program of GTK 3, and it is the window the script draws | `crates/ahk-box/helper/ahk-gui.py` |
| the windows of an X session | `libX11`: `XQueryTree` and `XGetWindowProperty` | `crates/ahk-box/src/x11.rs` |
| the pointer and the keyboard | the devices of uinput -- the tree makes its own | `crates/ahk-box/src/keyboard.rs`, `pointer.rs` |

**And the tree answers with the reason when the desktop cannot answer.**
That is the rule of the tree: a question that cannot be answered fails
with the reason, and it does not answer `false` and it does not answer a
zero. So a compositor that does not publish the listing is not a failure
of the port -- it is a case of the port, and the reason is already
written:

- "the compositor does not publish ext_foreign_toplevel_list_v1, so its
  windows cannot be listed through Wayland."
- "the compositor does not publish zcosmic_toplevel_info_v1, so it can
  list its windows but cannot say which one has the focus."
- "the compositor did not say where this window is, so its place is not
  known."

## 1. The list of the desktops

The order is the order of what each one proves.

### 1.1 COSMIC -- the reference

**What it proves:** the whole of the window family. It is the only
desktop that publishes `zcosmic_toplevel_info_v1`, so it is the only one
that answers *which window has the focus*, *where a window is* and *in
what state it is*. The box is this desktop, and every measurement of the
windows in `AGENTS.md` is of it.

**Arch:** the session is in the AUR -- `cosmic-session` (with
`cosmic-comp` and `cosmic-workspace`), and the tree needs
`wayland-protocols` with the XML of COSMIC, which the tree already
carries in `crates/ahk-box/protocols/`.

**Without it:** the window family answers the reason of the compositor,
and the *window of the script* is the only window the tree can draw --
which is what the GTK helper does in any session.

### 1.2 A wlroots compositor -- Sway, Hyprland, niri, labwc, Wayfire

**What it proves:** the listing of the windows without the extension of
COSMIC. The wlroots family publishes
`wlr_foreign_toplevel_management_unstable_v1`, and the *portable*
listing `ext_foreign_toplevel_list_v1` is a different interface -- so
this is the case that proves the **reason** of the tree when the listing
is not there, and the case that proves the XWayland path beside it (the
`Xwayland` of the session is a second X server, and the tree asks it
too).

**Arch:** `sway` ✔, `hyprland` ✔, `niri` ✔, `labwc` ✔, `wayfire` ✔ --
all in the official repository. The tree needs `xwayland` for the X
server of the session.

### 1.3 GNOME (Mutter)

**What it proves:** the listing of the windows in the desktop that most
of the world uses, and the bus of the accessibility with the apps of
GTK. Mutter publishes the foreign-toplevel interfaces; **whether it
publishes the portable listing `ext_foreign_toplevel_list_v1` or only
the old `wlr` one is to be measured on the VM** -- and either way the
answer of the tree is written down.

**Arch:** `gnome-shell` ✔ (official) + `gnome-control-center` ✔.

### 1.4 KDE Plasma (KWin)

**What it proves:** the third family of the compositor (KWin has its own
way of saying the state of a window) and the bus of the accessibility
with the apps of Qt, which expose AT-SPI through a bridge of their own
(`qt5-wayland` / `qt6-wayland` + the accessibility bridge of Qt).

**Arch:** `plasma-desktop` ✔ (official).

### 1.5 The X11 sessions -- i3, openbox, Xfwm

**What it proves:** the X path of the tree, which the Wayland session
cannot: the tree asks the X server for the tree of the windows
(`XQueryTree`) and for the name of each of them
(`XGetWindowProperty`). On the box this path answers "unknown", because
the X server beside the Wayland session is the rootless Xwayland of
gamescope and it reports an empty list of clients -- so a VM with a real
X session is the only way to prove that path.

**Arch:** `i3-wm` ✔, `openbox` ✔, `xfwm4` ✔, `xorg-server` ✔.

### 1.6 The services that are not a desktop

They are needed by every session, and they are what the *control* family
and the *window of the script* need:

| the service | the package of Arch | what it is for |
|---|---|---|
| the bus of the accessibility | `at-spi2-core` ✔ (`at-spi-bus-launcher` is in it) | the `Control*` family reads the controls of the apps of the session through it |
| the toolkit of the helper | `gtk3` ✔ + `python-gobject` ✔ | the window of the script is drawn by a program of GTK 3 |
| a bridge of AT-SPI for o Chromium | `chromium` ✔ with `--force-renderer-accessibility` | the Chromium does not expose its controls unless it is told to |
| o cliente de X11 | `libx11` ✔ | the X path of the tree |
| as ferramentas do X | `xorg-xprop` ✔, `xorg-xwininfo` ✔ | the client list, the active window and the state are `xprop`; the place and the size are `xwininfo`. Without them the X11 family answers "Target window not found." |
| o mixer da sessão | `pipewire` + `pipewire-pulse` (which puts `pactl` in the path) | the family of the sound devices reads the names of the devices through `pactl`, the way the volume and the mute already do. Without a mixer the names fail with the reason of the machine |
| o compositor do COSMIC | `cosmic-comp` (AUR) | a referência |
| o utilitário do bus | `systemd` ✔ (`busctl` ✔) | o `atspi.rs` fala com o bus por `busctl` |

## 2. How to switch the configuration in one VM

The tree asks **the compositor of the session** -- so the switch is a
switch of the session, not a switch of a window manager inside it.

1. **Several sessions in one image.** Install the desktops and let the
   display manager offer them: `gdm` ✔ (GNOME), `sddm` ✔ (KDE), or
   `lightdm` ✔. The session is chosen at the login, and the switch takes
   a reboot of the session, not of the machine.
2. **Without a display manager.** Start the compositor from a TTY --
   `systemd-run --user --scope sway` ✔, or `Ctrl+Alt+F2` and
   `sway`/`Hyprland`/`cosmic-comp` by hand. This is the fastest way to
   go through the five desktops of a test matrix in an afternoon.
3. **The environment is the same.** `WAYLAND_DISPLAY` ✔ and
   `XDG_RUNTIME_DIR` ✔ are what the tree reads, and the runner of the box
   sets them itself (`ensure_session_env` ✔) -- so a test that is run
   over ssh with the compositor of the session running works without any
   more.
4. **The bus of the accessibility is per session.** `at-spi-bus-launcher`
   starts with the session; a session that does not have it has no
   `org.a11y.Bus`, and the tree answers "the bus of the accessibility is
   not running in this session" -- the case to prove.
5. **The helper is GTK 3 in any session.** The window of the script does
   not need a compositor that is any particular one: it is a GTK program
   with a pipe. It needs the *session* to have a Wayland or an X display
   though, and the `Gtk.WindowType.POPUP` of the helper is GTK 3 only.

## 3. The checklist of a VM

The same list for every desktop of the matrix:

- [ ] the session starts, and the compositor publishes what it publishes
- [ ] `ahk --surface` answers the same names in every session (the
      surface is the language, not the desktop)
- [ ] `WinGetCount()` and `WinGetList()` answer the windows of the
      session -- or the reason of the compositor
- [ ] `WinActive("...")` answers the window that has the focus -- or the
      reason of the compositor
- [ ] `WinGetPos` answers the place of a window -- or the reason of the
      compositor
- [ ] `ControlGetText("...", "...")` reads a control of an app of the
      session -- or the reason of the bus
- [ ] a `Gui` opens, takes its controls, and the event of a click reaches
      the handler of the script
- [ ] the pointer and the keyboard go through the devices of uinput, and
      the compositor adopts them (the four seconds of the note of the
      tree)
- [ ] the emergency stop works: `pkill -f release/ahk` ✔

## 4. What is measured and what is not

- **Measured (2026-10-29 and later, the box, COSMIC):** the listing of
  the windows, the titles, the focus, the ids as whole numbers, the wait
  of the family with its timeout, and the reason of the compositor when
  the geometry is not said.
- **Measured (the box):** the X path answers "unknown", because the X
  server of the session is the rootless Xwayland of gamescope and it
  reports an empty list of clients.
- **Measured on the VM `AhkDesktopTest` (2026-11-11):** sway and i3.
  The measurements are in section 5.
- **Not measured yet:** GNOME, KDE, and the wlroots family beyond sway.

## 5. The measurement of the VM `AhkDesktopTest` (2026-11-11)

The VM is an Arch Linux (kernel 7.2.15) under QEMU with a **virtio-gpu**
(the DRM device is `/dev/dri/card1`, the connector `Virtual-1`, and
`WLR_RENDERER=pixman` drives it in software), 8 GB of RAM, 6 vCPUs, a
bridged network, and no passthrough: the GPU of the host is the one the
COSMIC session is using. The desktop is chosen by `/etc/ahk-session` and
the session starts from the autologin of `tty1`: `sway`, or `startx i3`.

**The suite is green on the VM**: `cargo test --workspace` passes every
case, and the surface answers the same 754 names as on the workstation --
the language does not depend on the compositor.

### 5.1 sway 1.12 (wlroots) -- the item 2 of the list

The compositor of the VM **does publish the portable listing**
(`ext_foreign_toplevel_list_v1`): the tree lists its windows with no
adapter at all. That is the answer the matrix was written to find out.

| the question | the answer of the VM |
|---|---|
| `WinGetCount()` | **3** (three `foot` terminals tiled) |
| the titles of the list | `tester@archlinux:~` three times |
| `WinGetTitle("tester@archlinux")` | the title: the match by substring works |
| `WinExist("zzz-nothing-zzz")` | 0 |
| `WinWait("zzz-nothing-zzz", "", 1)` | 0, in 1012 ms |
| `WinGetPos(...)` | **"the compositor did not say where this window is, so its place is not known"** -- sway does not publish the geometry of a window through the extension of COSMIC |
| `WinActive("tester")` | **the reason** of the compositor (see 5.4): the listing is not the question of the focus |
| a `Gui` | opens, and its control is read: `ControlGetText("Edit1", ...)` answers "hello from sway" |
| `EditGetCurrentCol(1)` | 16 |

**The bus of the accessibility works in sway**: `at-spi-bus-launcher`
starts with the session, and the `Control*` family reads the controls of
the window of the script through it.

### 5.2 i3 4.25 (X11, Xorg 21.1) -- the item 5 of the list

A real X server, which the box does not have (its X server is the
rootless Xwayland of gamescope, and it reports an empty list of clients).
Since the fixes of 5.4 the client list of the server is parsed, so the
X11 path answers the whole family of the windows:

| the question | the answer of the VM |
|---|---|
| `WinGetTitle()` | **"tester@archlinux:~"** -- the active window, read through `_NET_ACTIVE_WINDOW` |
| `WinGetCount()` | **5** -- the three terminals of the session and the two of i3 |
| `WinGetList()` | the ids of the X server: `8388620`, `10485772`, ... with their titles |
| `WinExist("tester@archlinux")` | **8388620** -- the id of the first window that matches |
| `WinExist("zzz-nothing-zzz")` | 0 -- absent, and not a failure |
| `WinActive("tester@archlinux")` | **16777228** -- the id of the window that has the focus |
| `WinGetPos(...)` | **2,18,250,778** -- the corner and the size that `xwininfo` says |
| `WinGetMinMax(...)` | 0 -- neither maximized nor minimized |

**The X11 path needs two tools of the X server beside `libX11`**:
`xprop` (the client list, the active window, the state) and `xwininfo`
(the place and the size). Both are in the list of the services below; a
VM without `xwininfo` answers the geometry with "Target window not
found.", which is how the absence of the tool looks from the script.

### 5.3 What the VM says about the matrix itself

- The item 2 (wlroots) is **proven**: the portable listing is enough for
  the listing family, and the two things the listing does not carry --
  the focus and the geometry -- answer the reason of the compositor,
  both of them, since the fixes of 5.4.
- The item 5 (X11) is **proven**: a real X server answers the whole
  family of the windows through its client list, once the list is
  parsed. The X path was written against the empty list of the Xwayland
  of gamescope, and the VM is what showed both the gap and the fix.
- The window of the script and the bus of the accessibility do **not**
  depend on the compositor: they worked in the two sessions of the VM,
  and they work on the box.
- One hand merge more came out of the groups family (2026-11-11):
  `matching_windows` -- the question a `GroupActivate` makes -- was
  Wayland-only in the branch and in the master, and an X11 session failed
  it with the reason of the compositor. Since the client list is parsed,
  the matching reads the same list: `GroupActivate` of a group that
  carries the title of an xterm answers the id of the xterm, `8388620`.

### 5.4 The fixes the VM produced (2026-11-11)

Three behaviours the VM measured and the tree then changed; all three
are in `crates/ahk/src/builtins.rs` and `crates/ahk-box/src/desktop.rs`,
and the case of the first is in `crates/ahk/tests/builtins.rs`:

- **The listing is not the question of the focus.** `WinActive` and
  `WinGetTitle()` asked the listing first and answered 0 and the empty
  text when no window of the listing carried the focus -- which on sway
  is always, because the focus is not in the portable listing. The
  handlers now fall through to the question of the focus when the
  listing has no active window, and the desktop answers the reason of
  the compositor: "the compositor does not publish
  zcosmic_toplevel_info_v1, so it can list its windows but cannot say
  which one has the focus." Measured again on the VM after the fix.
- **The client list of the X server is parsed.** `_NET_CLIENT_LIST` was
  read and refused with "which this runtime does not parse yet". The
  parse is `the_x_ids_in` and `x11_windows` in the box: the ids of the
  server, the `_NET_WM_NAME` of each one and the `_NET_ACTIVE_WINDOW`
  for the focus. `WinGetList`, `WinGetCount`, `WinExist` and `WinActive`
  answer on X11 since then.
- **The place and the state of an X11 window are `xwininfo` and
  `xprop`.** `WinGetPos` reads the absolute corner and the size out of
  `xwininfo -id`, and `WinGetMinMax` reads `WM_STATE` (iconic is the
  minimized one) and the two maximized atoms of `_NET_WM_STATE`. A VM
  without `xwininfo` shows why the tool belongs to the list of the
  services: the geometry answers "Target window not found."
