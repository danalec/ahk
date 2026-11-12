//! The lab box as a script target.
//!
//! `ahk-box` is the part of the runtime that touches the machine: the
//! keyboard of our own, the absolute pointer, the screen capture, the
//! commands. It runs **on the lab box**, because that is where the desktop
//! is.

pub mod atspi;
pub mod desktop;
pub mod drives;
pub mod files;
pub mod gui;
pub mod hotkeys;
pub mod keyboard;
pub mod locks;
pub mod machine;
pub mod native;
pub mod network;
pub mod picture;
pub mod pictures;
pub mod pointer;
pub mod process;
pub mod raven;
pub mod registry;
pub mod sys;
pub mod trash;
pub mod wayland;

/// The X11 desktop, on the two hosts whose display servers speak X11.
///
/// WHY: Xlib and XTEST are runtime `dlopen`s, so the module only exists
/// where an X server can be — Linux (Xorg and XWayland) and FreeBSD (X11).
/// On every other system the crate keeps compiling with no X11 surface at
/// all, and [`x11_available`] there answers `false`.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub mod x11;

pub use desktop::{BoxError, LabDesktop};
pub use keyboard::{ENUMERATION_WAIT, KEY_GAP, KEY_HOLD, Keyboard, KeyboardError};
pub use native::Libraries;
pub use pointer::{PRESS_HOLD, PRESS_SETTLE, Pointer, PointerError};

/// Whether the Xlib/XTEST symbol table resolved on this machine.
///
/// WHY: the caller asks this before choosing the X11 desktop, so a machine
/// without the libraries falls back to the uinput/Wayland backend instead of
/// failing at the first script statement. `dlopen` runs once, in the symbol
/// table, and the answer is cached.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn x11_available() -> bool {
    x11::available()
}

/// Whether the X11 desktop could be used on a host that has no X11 module.
///
/// WHY: the question is honest on every platform, so a caller does not need
/// its own `cfg` to ask it. On Windows and macOS there is no X11 backend and
/// the answer is `false`.
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
pub fn x11_available() -> bool {
    false
}

/// Which desktop the caller wants: `YOKO_DESKTOP`, or `uinput` by default.
///
/// WHY: the X11 backend is additive. Nothing in `desktop.rs` changes its
/// behaviour when this returns `x11`; the rest of the yoko stack reads the
/// value and picks `ahk_box::x11::X11Desktop` instead of `LabDesktop`. The
/// default keeps every existing run on the uinput/Wayland path.
pub fn desktop_kind() -> String {
    match std::env::var("YOKO_DESKTOP") {
        Ok(kind) if !kind.is_empty() => kind,
        _ => "uinput".to_string(),
    }
}
