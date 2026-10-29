//! The windows of a Wayland compositor.
//!
//! The compositor of the box is COSMIC, and it answers the questions
//! about its windows through two protocols:
//!
//! * `ext-foreign-toplevel-list-v1` -- the portable one. It lists the
//!   toplevels and says what each of them is: the title, the application
//!   id and an identifier that is stable while the window lives.
//! * `zcosmic-toplevel-info-v1` -- the one of COSMIC. It *extends* the
//!   listing: for a handle of the listing the compositor gives a handle
//!   of its own, and that one carries the state of the window -- and the
//!   state says whether the window is the active one.
//!
//! The listing is enough to answer whether a window exists. The active
//! window needs the second one, which belongs to the compositor of the
//! box: a compositor that does not publish it can list its windows but
//! cannot say which one has the focus.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::Client;

#[cfg(target_os = "linux")]
mod protocols;

#[cfg(not(target_os = "linux"))]
mod other;
#[cfg(not(target_os = "linux"))]
pub use other::Client;

/// A window of the compositor.
#[derive(Debug, Clone, Default)]
pub struct Window {
    /// The identifier of the window, when the compositor gives one. It is
    /// the same for the same window across processes, and it is valid
    /// while the window is mapped.
    pub identifier: Option<String>,
    /// The title of the window.
    pub title: String,
    /// The application id of the window: the name of the program that
    /// opened it.
    pub app_id: String,
    /// Whether the window is the active one.
    pub active: bool,
}
