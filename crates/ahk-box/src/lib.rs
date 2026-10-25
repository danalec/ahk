//! The lab box as a script target.
//!
//! `ahk-box` is the part of the runtime that touches the machine: the
//! keyboard of our own, the screen capture, the commands. It runs **on the
//! lab box**, because that is where the desktop is.

pub mod desktop;
pub mod keyboard;

pub use desktop::{BoxError, LabDesktop};
pub use keyboard::{ENUMERATION_WAIT, KEY_GAP, KEY_HOLD, Keyboard, KeyboardError};
