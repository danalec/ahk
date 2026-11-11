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
- **Not measured yet:** every other desktop of this list. The reasons of
  the tree are written for them, and the VMs are what turns them into
  measurements.
