//! The Wayland client, on a platform that has no Wayland.
//!
//! The protocols of the compositor are Linux ones, and the crate of the
//! client is Unix-only. The client therefore says so, instead of
//! pretending that the windows are not there.

use ahk::desktop::Monitor;

use super::Window;

/// The client of the compositor.
pub struct Client;

impl Client {
    /// Open the connection to the compositor.
    pub fn open() -> Result<Self, String> {
        Err(not_linux())
    }

    /// The monitors the compositor has.
    pub fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        Err(not_linux())
    }

    /// The windows the compositor has.
    pub fn windows(&mut self) -> Result<Vec<Window>, String> {
        Err(not_linux())
    }

    /// The window that has the focus.
    pub fn active(&mut self) -> Result<Option<Window>, String> {
        Err(not_linux())
    }
}

/// The reason this platform cannot answer.
fn not_linux() -> String {
    "the Wayland protocols of this crate are for Linux, and this is not Linux".to_string()
}
