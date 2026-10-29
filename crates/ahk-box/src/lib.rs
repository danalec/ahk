//! The lab box as a script target.
//!
//! `ahk-box` is the part of the runtime that touches the machine: the
//! keyboard of our own, the absolute pointer, the screen capture, the
//! commands. It runs **on the lab box**, because that is where the desktop
//! is.

pub mod desktop;
pub mod files;
pub mod hotkeys;
pub mod keyboard;
pub mod machine;
pub mod native;
pub mod pointer;
pub mod raven;
pub mod wayland;

pub use desktop::{BoxError, LabDesktop};
pub use keyboard::{ENUMERATION_WAIT, KEY_GAP, KEY_HOLD, Keyboard, KeyboardError};
pub use native::Libraries;
pub use pointer::{PRESS_HOLD, PRESS_SETTLE, Pointer, PointerError};
