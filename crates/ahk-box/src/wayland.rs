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
    /// Whether the window is maximized.
    pub maximized: bool,
    /// Whether the window is minimized.
    pub minimized: bool,
    /// Where the window is and how big it is, when the compositor says it.
    ///
    /// `None` is the compositor that did not say it, and it is not the same
    /// as a window at the origin with no size: the geometry of a window that
    /// the compositor has not described is not known, and a script must be
    /// told that, not given a zero.
    pub geometry: Option<Geometry>,
}

/// The geometry of a window, in the coordinates of the screen.
///
/// The x and the y are the upper-left corner and not the center, and the
/// width and the height are of the window the user sees.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Geometry {
    /// The left of the window.
    pub x: i32,
    /// The top of the window.
    pub y: i32,
    /// How wide the window is.
    pub width: i32,
    /// How tall the window is.
    pub height: i32,
}
