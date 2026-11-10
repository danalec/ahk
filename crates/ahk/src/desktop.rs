//! The machine the script drives.
//!
//! The runtime does no I/O of its own: everything that touches the
//! machine goes through one of the traits here. The traits are the
//! capabilities a machine may have, so a desktop with a keyboard and no
//! pointer says so about the pointer alone, and the script that asks
//! for the pointer is told why it cannot have it.
//!
//! [`Desktop`] is the sum of the capabilities. A desktop implements the
//! ones it has, and the sum, and the sum has no methods of its own.
//!
//! Every method that a capability does not have to answer says so:
//! "this desktop cannot ...". A question the machine cannot answer
//! fails with the reason, and never answers `false`.

use crate::dll::{NativeCall, NativeFailure, NativeResult};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// The answer to a question about a window.
#[derive(Debug, Clone, PartialEq)]
pub enum WindowAnswer {
    /// The window is there and is the active one.
    Active,
    /// The window exists and is not the active one.
    NotActive,
    /// The window is not there.
    ///
    /// This is an answer and not a failure: `WinExist` says that a window
    /// that is not there is not there, and `WinActive` says that a window
    /// that is not there is not the active one.
    Absent,
    /// The desktop cannot tell; the reason says why.
    Unknown(String),
}

/// The keyboard of the desktop.
pub trait Keyboard {
    /// Type text exactly, and say how it was typed.
    fn type_text(&mut self, text: &str) -> Result<String, String>;

    /// Press one key, and say how it was pressed.
    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String>;

    /// Put a key down, and leave it down.
    fn key_down(&mut self, _name: &str) -> Result<String, String> {
        Err("this desktop cannot hold a key down".to_string())
    }

    /// Let a key come up.
    fn key_up(&mut self, _name: &str) -> Result<String, String> {
        Err("this desktop cannot hold a key down".to_string())
    }

    /// Whether a key is down right now.
    fn key_is_down(&mut self, _name: &str) -> Result<bool, String> {
        Err("this desktop cannot tell which keys are down".to_string())
    }

    /// The keys that are down right now.
    fn keys_down(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot tell which keys are down".to_string())
    }
}

/// The pointer of the desktop.
pub trait Pointer {
    /// Click at a screen position.
    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String>;

    /// Put a mouse button down, and leave it down.
    fn mouse_down(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _button: &str,
    ) -> Result<String, String> {
        Err("this desktop cannot hold a mouse button down".to_string())
    }

    /// Let a mouse button come up.
    fn mouse_up(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _button: &str,
    ) -> Result<String, String> {
        Err("this desktop cannot hold a mouse button down".to_string())
    }

    /// Move the pointer to a screen position.
    ///
    /// A desktop that cannot move the pointer says so; the default is an
    /// error, because a script that believes it moved the pointer and did
    /// not is a script that types into the wrong window.
    fn move_pointer(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this desktop cannot move the pointer".to_string())
    }

    /// Where the pointer is, in screen pixels.
    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        Err("this desktop cannot tell where the pointer is".to_string())
    }

    /// The speed of the pointer, from 0 to 100.
    ///
    /// The number is the number of steps of a movement, and the speed of
    /// v2 is 0 to 100 with a default of 2: the 1 to 20 of the mouse of
    /// Windows is not the setting of v2.
    fn set_mouse_speed(&mut self, _speed: i64) -> Result<String, String> {
        Err("this desktop cannot change the speed of the pointer".to_string())
    }

    /// The delay between two movements of the pointer, in milliseconds.
    fn set_mouse_delay(&mut self, _delay: i64) -> Result<String, String> {
        Err("this desktop cannot change the delay of the pointer".to_string())
    }
}

/// The screen of the desktop.
pub trait Screen {
    /// Capture a region of the screen into a file; return the path.
    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String>;

    /// The colour of one pixel, as `#rrggbb`.
    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String>;

    /// The size of the screen, in pixels.
    ///
    /// The width comes first and the height second, the way the two
    /// variables of v2 ask for them.
    /// The `ClassNN` of every control of the script's window, one per line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetControls` answers the
    /// three of a window with a label, an edit and a button, as `Static1`, `Edit1` and
    /// `Button1`.
    fn gui_controls(&mut self) -> Result<String, String> {
        Err("this desktop cannot list the controls of the window of the script".to_string())
    }

    /// The text of every control of the script's window, each followed by a line break.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetText` of a window with a
    /// label and an edit answers the two texts, one after the other, each with a break.
    fn gui_all_text(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the text of the window of the script".to_string())
    }

    /// The opacity of the window of the script, or set it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `WinSetTransparent` counts
    /// from nought to 255, and `WinGetTransparent` answers the number that was set.
    fn gui_opacity(&mut self, _setting: Option<i64>) -> Result<i64, String> {
        Err("this desktop cannot read the opacity of the window of the script".to_string())
    }

    /// Put a text into a control of the script at its cursor.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `EditPaste` takes the text
    /// **first** and the handle second, as the whole family does.
    fn gui_paste(&mut self, _number: u64, _text: &str) -> Result<(), String> {
        Err("this desktop cannot put a text into a control of the script".to_string())
    }

    /// Whether a check box of the script is ticked, and ticking it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the `Value` of a check box
    /// is a **number**, nought or one, and its `Text` is its label.
    fn gui_checked(&mut self, _number: u64, _setting: Option<bool>) -> Result<i64, String> {
        Err("this desktop cannot read the tick of a check box of the script".to_string())
    }

    /// The scale factor of the output, as the session reports it.
    ///
    /// `A_ScreenDPI` is 96 times this, the number v2 gives with the scale of the
    /// session applied.
    fn screen_scale(&mut self) -> Result<i64, String> {
        Err("this desktop does not know the scale of the output".to_string())
    }

    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        Err("this desktop cannot tell how big the screen is".to_string())
    }

    /// Show a small window of text, and answer the number that names it. An empty
    /// text takes it away. Measured on the binary of v2.0.19 on 2026-11-10:
    /// showing one answers an Integer and `ToolTip()` clears it.
    fn tooltip_show(&mut self, _text: &str, _x: i64, _y: i64) -> Result<i64, String> {
        Err("this desktop cannot show a tooltip".to_string())
    }

    /// Move and size the window of the script. Measured on the binary of v2.0.19
    /// on 2026-11-10: `WinMove` of the window the script made works and answers
    /// nothing. A Wayland compositor places windows itself, so the call is the
    /// one the toolkit offers and need not be honoured on the screen.
    fn window_move(&mut self, _x: i64, _y: i64, _width: i64, _height: i64) -> Result<(), String> {
        Err("this desktop cannot move the window of the script".to_string())
    }

    /// The monitors of the desktop, in the order it lists them.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `MonitorGetCount`
    /// answers a whole number, `MonitorGetName(1)` answers the name of the
    /// first output, and a monitor that is not there is refused with
    /// "Parameter #1 of MonitorGetName is invalid.".
    fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        Err("this desktop cannot list the monitors".to_string())
    }

    /// The first pixel of a colour inside a region, if there is one.
    fn pixel_search(
        &mut self,
        _x1: i64,
        _y1: i64,
        _x2: i64,
        _y2: i64,
        _colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        Err("this desktop cannot search the screen for a colour".to_string())
    }
}

/// The way the title of a window is matched with the text that is wanted.
///
/// Measured on the binary of v2.0.19 on 2026-10-31 and read in the
/// documentation of v2.0.19: the mode 1 wants the title of the window to
/// start with the text, the mode 2 wants it to have the text in it, the
/// mode 3 wants it to be the text, and the pattern wants the text to be a
/// regular expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleMatch {
    /// The title of the window starts with the text that is wanted.
    Starts,
    /// The title of the window has the text that is wanted in it.
    Contains,
    /// The title of the window is the text that is wanted.
    Exact,
    /// The text that is wanted is a pattern of a regular expression.
    Pattern,
}

impl TitleMatch {
    /// Whether the title of a window matches the text that is wanted.
    ///
    /// The case matters, as it matters in v2: "the window titles and text
    /// are case-sensitive". The only exception is the case-insensitive
    /// option of the pattern, which v2 spells with an `i)` at the front of
    /// the title.
    ///
    /// The pattern is the one of the engine of the tree, and a pattern that
    /// the engine does not carry is a failure with the reason of the engine.
    pub fn matches(self, title: &str, wanted: &str) -> Result<bool, String> {
        match self {
            TitleMatch::Starts => Ok(title.starts_with(wanted)),
            TitleMatch::Contains => Ok(title.contains(wanted)),
            TitleMatch::Exact => Ok(title == wanted),
            TitleMatch::Pattern => {
                // v2 spells the case-insensitive option of a title with an
                // `i)` at the front of it, and the engine of the tree spells
                // it with `(?i)`.
                let (pattern, ignore_case) = match wanted.strip_prefix("i)") {
                    Some(rest) => (rest, true),
                    None => (wanted, false),
                };
                let pattern = if ignore_case {
                    alloc::format!("(?i){pattern}")
                } else {
                    pattern.to_string()
                };
                let regex = crate::regex::Regex::new(&pattern)?;
                Ok(regex.find(title)?.is_some())
            }
        }
    }
}

/// A window of the desktop, as the listing of the windows describes it.
///
/// The two things that v2 reads of a window are the ones here: the identifier,
/// which is what `WinGetID` answers and what the entries of `WinGetList` are,
/// and the class, which is what `WinGetClass` answers.
///
/// The identifier of a Wayland window is a token that the compositor gives it,
/// and not a number, so it is text here. Measured on the binary of v2.0.19 on
/// 2026-11-09: the id of a window of Windows is a whole number, and a script
/// that does arithmetic on it would not have the same thing on Wayland.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowRef {
    /// The identifier the compositor gives the window, or the empty text when
    /// it did not give one.
    pub id: String,
    /// The class of the window: the application id of it on Wayland, and the
    /// class of the window on X11.
    pub class: String,
}

/// Where a window is and how big it is.
///
/// The x and the y are the upper-left corner of the window, and not its
/// center; the width and the height are of the window that the user sees.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowGeometry {
    /// The left of the window.
    pub x: i64,
    /// The top of the window.
    pub y: i64,
    /// How wide the window is.
    pub width: i64,
    /// How tall the window is.
    pub height: i64,
}

/// What a command of the `WinWait` family waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitFor {
    /// A window that matches exists.
    Present,
    /// A window that matches exists and has the focus.
    Active,
    /// No window that matches has the focus.
    NotActive,
    /// No window that matches exists.
    Close,
}

/// The space of the drive that a path is on, in megabytes.
///
/// v2 answers the two numbers of a drive in megabytes, measured on the binary
/// of v2.0.19 on 2026-11-09: `DriveGetSpaceFree("C:\")` answered `836675` and
/// `DriveGetCapacity("C:\")` answered `3905969`, both whole numbers.
///
/// A drive of this tree is the filesystem that a path is on, because a Linux
/// box has mount points and not the letters of a drive.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DriveSpace {
    /// The space that is free, in megabytes.
    pub free: i64,
    /// The capacity of the drive, in megabytes.
    pub capacity: i64,
    /// The filesystem of the drive, as the machine names it: "ext4", "zfs".
    pub filesystem: String,
    /// The kind of the drive, in the word of v2: "Fixed", "Network",
    /// "Removable", "CDROM", "RAMDisk" or "Unknown".
    pub kind: String,
}

/// A monitor: an output of the desktop, and the rectangle of it.
///
/// The rectangle is in the space of the desktop, which is the space the windows
/// are placed in. It is what `MonitorGet` answers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Monitor {
    /// The name of the output, when the desktop gives one.
    pub name: String,
    /// The left of the monitor.
    pub left: i64,
    /// The top of the monitor.
    pub top: i64,
    /// The right of the monitor.
    pub right: i64,
    /// The bottom of the monitor.
    pub bottom: i64,
    /// The scale of the monitor: 1 is the ordinary one.
    pub scale: i64,
}

/// What became of a question about a control of a window.
///
/// The two failures are different facts and v2 tells them apart. Measured on the
/// binary of v2.0.19 on 2026-11-09: `ControlGetText("x", "zzz-no-such-window-zzz")`
/// fails with **"Target window not found."** and the title in the `Extra`, and
/// `ControlGetText("x", "A")` -- a window that is there -- fails with **"Target
/// control not found."**. Neither of them answers an empty text, which is what
/// this tree answered before the measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlAnswer {
    /// The control is there, and this is its text.
    Found(String),
    /// No window of the session has this title.
    NoSuchWindow,
    /// The window is there and the control is not in it.
    NoSuchControl,
}

/// What became of a question about a property of a control.
///
/// Measured on the binary of v2.0.19 on 2026-11-09 with a window of its own
/// making: `ControlGetVisible` and `ControlGetEnabled` answer **1** for a control
/// that is there and visible or enabled, so the answer is a number and not a
/// truth of the host; and the two failures are the two of `ControlGetText`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlag {
    /// The control is there and the flag is set.
    Yes,
    /// The control is there and the flag is not set.
    No,
    /// No window of the session has this title.
    NoSuchWindow,
    /// The window is there and the control is not in it.
    NoSuchControl,
}

/// What became of a question that writes to a control.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `ControlSetText` answers the
/// **empty text** when it worked, and the two failures are the two of
/// `ControlGetText`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlEdit {
    /// The text was written.
    Done,
    /// No window of the session has this title.
    NoSuchWindow,
    /// The window is there and the control is not in it.
    NoSuchControl,
}

/// The windows of the desktop.
pub trait Windows {
    /// Whether the window with this title is the active one.
    ///
    /// The mode is the way the title is matched with the text that is
    /// wanted: it is the setting of the script, and the runtime hands it
    /// with the question.
    fn window_active(&mut self, _title: &str, _mode: TitleMatch) -> Result<WindowAnswer, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }

    /// Whether a window with this title exists.
    ///
    /// The mode is the way the title is matched with the text that is
    /// wanted: it is the setting of the script, and the runtime hands it
    /// with the question.
    fn window_exists(&mut self, _title: &str, _mode: TitleMatch) -> Result<WindowAnswer, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// The title of the active window.
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }

    /// The windows whose title matches, in the order the desktop lists them.
    ///
    /// The empty title matches every window. Measured on the binary of
    /// v2.0.19 on 2026-11-09: `WinGetList()` and `WinGetList("")` both
    /// answered every window of the desktop, and `WinGetCount()` with no
    /// argument answered the number of them -- so an empty title is not "no
    /// window", and it is not "the last found window" either, because this
    /// runtime keeps none.
    fn matching_windows(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// Where the window is and how big it is.
    ///
    /// `None` is a window that is not there. A window that **is** there but
    /// whose geometry the compositor did not describe is a failure with the
    /// reason, and not a geometry of zeros: a script must be told that the
    /// place is not known, and not be given a window at the origin.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetPos` writes
    /// its four answers into four variables given by reference, and it
    /// answers the empty text. A window that is not there is a failure --
    /// "Target window not found." -- and a call with no window at all is the
    /// same failure, because there is no last found window to fall back to.
    fn window_geometry(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
    ) -> Result<Option<WindowGeometry>, String> {
        Err("this desktop cannot tell where a window is".to_string())
    }

    /// Whether a window is maximized, minimized, or neither.
    ///
    /// The answer is the number of v2: `1` for maximized, `-1` for minimized
    /// and `0` for neither. `None` is a window that is not there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09, on a window of its
    /// own: `0` before, `1` after `WinMaximize`, `0` after `WinRestore`, and
    /// `-1` after `WinMinimize`.
    fn window_min_max(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
    ) -> Result<Option<i64>, String> {
        Err("this desktop cannot tell whether a window is maximized".to_string())
    }

    /// Wait until a condition about a window holds.
    ///
    /// `Some` is the condition met, and the empty text is the answer of the
    /// conditions that are about the absence of a window -- `NotActive` and
    /// `Close` -- because there is no window to identify. `None` is the
    /// timeout.
    ///
    /// The empty title is no window at all: v2 falls back to the last found
    /// window, and this runtime keeps none, so the conditions that talk about
    /// the absence of a window hold at once and the ones that talk about its
    /// presence never do. Measured on the binary of v2.0.19 on 2026-11-09:
    /// `WinWaitNotActive("no-such")` and `WinWaitClose("no-such")` answered
    /// `1` at once, and `WinWait`/`WinWaitActive` answered `0` when the
    /// timeout ran out.
    fn window_wait(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
        _what: WaitFor,
        _timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        Err("this desktop cannot wait for a window".to_string())
    }

    /// The text of a named control of the window with this title.
    ///
    /// This is the **accessibility** layer: AT-SPI is a D-Bus protocol, and
    /// D-Bus is what a Wayland session keeps, so it is the one way to reach the
    /// controls of a window on the box. The AHK_X11 reaches the same layer
    /// through `at-spi.cr`, and it is the most valuable piece it has.
    ///
    /// A control is named, and the name is what a script writes: the text of a
    /// label, the caption of a button. The window is found by its title, which
    /// is the `Name` of a window in the tree.
    ///
    /// It is a method here and not a capability trait of its own, because the
    /// controls are the windows' and every fake desktop of the suite already
    /// answers `Windows`. A trait of its own is the rule of this tree, and it
    /// would have cost an empty `impl` in eighteen test files for no gain the
    /// suite can see.
    ///
    /// `None` is a window or a control that is not there, and not a failure: a
    /// script asks about a control that may be absent.
    fn control_text(&mut self, _control: &str, _title: &str) -> Result<ControlAnswer, String> {
        Err("this desktop cannot read the controls of a window".to_string())
    }

    /// Whether a named control of the window with this title is visible.
    fn control_visible(&mut self, _control: &str, _title: &str) -> Result<ControlFlag, String> {
        Err("this desktop cannot read the controls of a window".to_string())
    }

    /// Whether a named control of the window with this title is enabled.
    fn control_enabled(&mut self, _control: &str, _title: &str) -> Result<ControlFlag, String> {
        Err("this desktop cannot read the controls of a window".to_string())
    }

    /// Write a text into a named control of the window with this title.
    fn control_set_text(
        &mut self,
        _control: &str,
        _title: &str,
        _text: &str,
    ) -> Result<ControlEdit, String> {
        Err("this desktop cannot write to the controls of a window".to_string())
    }

    /// Click a named control of the window with this title.
    fn control_click(&mut self, _control: &str, _title: &str) -> Result<ControlEdit, String> {
        Err("this desktop cannot click the controls of a window".to_string())
    }

    /// Put the focus on a named control of the window with this title.
    fn control_focus(&mut self, _control: &str, _title: &str) -> Result<ControlEdit, String> {
        Err("this desktop cannot focus the controls of a window".to_string())
    }

    /// Type a text into a named control of the window with this title.
    ///
    /// The way in is two capabilities together: the focus goes on the control
    /// through the accessibility layer, and the keys go through the keyboard.
    fn control_send_text(
        &mut self,
        _control: &str,
        _title: &str,
        _text: &str,
    ) -> Result<ControlEdit, String> {
        Err("this desktop cannot type into the controls of a window".to_string())
    }

    /// The process of the window with this title.
    ///
    /// `None` is a window that is not there. Measured on the binary of v2.0.19 on
    /// 2026-11-09: `WinGetPID` answers the number of the process, and a title that
    /// no window has is "Target window not found.".
    fn window_pid(&mut self, _title: &str) -> Result<Option<i64>, String> {
        Err("this desktop cannot say the process of a window".to_string())
    }

    /// Whether a window of the script is open.
    ///
    /// It is what makes the lazy open composable: `gui_open` refuses a second
    /// window, so a member that needs one has to ask first whether there is one
    /// already. Without it, the second `Show` of a script would be refused by
    /// its own desktop.
    fn gui_is_open(&mut self) -> bool {
        false
    }

    /// Open a window of the script's own, and answer the handle of it.
    ///
    /// This is the `Gui` of v2. The window is **not shown**: measured on the
    /// binary of v2.0.19 on 2026-11-09, `Gui()` makes it and `Show()` puts it on
    /// the screen, and a script may add to it in between.
    fn gui_open(&mut self, _title: &str) -> Result<u64, String> {
        Err("this desktop cannot make a window of the script".to_string())
    }

    /// Add a control to a window of the script.
    ///
    /// The answer is the number that names the control and its `ClassNN`, which
    /// is the file name of the kind and the count of that kind -- `Edit1` for the
    /// first edit, whatever else was added before it. Measured on the binary of
    /// v2.0.19 on 2026-11-09.
    fn gui_add(&mut self, _handle: u64, _kind: &str, _text: &str) -> Result<(u64, String), String> {
        Err("this desktop cannot add a control to a window of the script".to_string())
    }

    /// The items of a list-shaped control of the script's window, one per line.
    /// Measured on the binary of v2.0.19 on 2026-11-10: `ControlGetItems`
    /// answers an array of them.
    fn gui_combo_items(&mut self, _number: u64) -> Result<String, String> {
        Err("this desktop cannot read the items of a list-shaped control".to_string())
    }

    /// Append an item to a list-shaped control. Measured on the same day:
    /// `ControlAddItem("gamma", ...)` makes the array one longer.
    fn gui_combo_add(&mut self, _number: u64, _text: &str) -> Result<(), String> {
        Err("this desktop cannot add an item to a list-shaped control".to_string())
    }

    /// Choose an item by its 1-based index or by its text. Returns the index.
    /// Measured on the same day: `ControlChooseIndex(2, ...)` leaves the
    /// control's `Value` at 2.
    fn gui_combo_select(&mut self, _number: u64, _wanted: &str, _by_text: bool) -> Result<i64, String> {
        Err("this desktop cannot choose an item of a list-shaped control".to_string())
    }

    /// The chosen item as (index, text). Measured on the same day: nothing
    /// chosen is (0, ""), and `ControlGetChoice` reads the text alone.
    fn gui_combo_choice(&mut self, _number: u64) -> Result<(i64, String), String> {
        Err("this desktop cannot read the choice of a list-shaped control".to_string())
    }

    /// Remove an item by its 1-based index. Measured on the binary of v2.0.19
    /// on 2026-11-10: `ControlDeleteItem(1, ...)` leaves the array one shorter.
    fn gui_combo_delete(&mut self, _number: u64, _index: &str) -> Result<(), String> {
        Err("this desktop cannot remove an item of a list-shaped control".to_string())
    }

    /// The 1-based index of the first item that is this text, or zero.
    fn gui_combo_find(&mut self, _number: u64, _text: &str) -> Result<i64, String> {
        Err("this desktop cannot find an item of a list-shaped control".to_string())
    }

    /// Open or close the pop-up of a list-shaped control. Measured on the
    /// binary of v2.0.19 on 2026-11-10: `ControlShowDropDown` and
    /// `ControlHideDropDown` both answer the empty text.
    fn gui_combo_dropdown(&mut self, _number: u64, _show: bool) -> Result<(), String> {
        Err("this desktop cannot open the pop-up of a list-shaped control".to_string())
    }

    /// The text of the status bar of the script's window. Measured on the
    /// binary of v2.0.19 on 2026-11-10: `StatusBarGetText(1, WinTitle)`
    /// answers it.
    fn gui_status_text(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the text of a status bar".to_string())
    }

    // -- the list view ------------------------------------------------------------
    //
    // Measured on the binary of v2.0.19 on 2026-11-10: a ListView is
    // `SysListView321`, `lv.Add("", "a", "b")` answers a 1-based row number,
    // `lv.GetCount()` counts them, `lv.GetText(row, col)` reads a cell,
    // `lv.Modify(row, ...)` changes and selects one, `lv.GetNext()` gives the
    // next row, and `ListViewGetContent` answers the cells separated by tabs
    // and the rows by line breaks.

    /// Append a row of cells; answers its 1-based number.
    fn gui_list_add(&mut self, _number: u64, _cells: &[String]) -> Result<i64, String> {
        Err("this desktop cannot add a row to a list view".to_string())
    }

    /// The number of rows.
    fn gui_list_count(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot count the rows of a list view".to_string())
    }

    /// The text of one cell, by 1-based row and column.
    fn gui_list_text(&mut self, _number: u64, _row: i64, _col: i64) -> Result<String, String> {
        Err("this desktop cannot read a cell of a list view".to_string())
    }

    /// Every row, cells by tabs and rows by line breaks; `selected` limits it
    /// to the row that is chosen.
    fn gui_list_content(&mut self, _number: u64, _selected: bool) -> Result<String, String> {
        Err("this desktop cannot read the content of a list view".to_string())
    }

    /// Change the cells of a row and select it; answers 1.
    fn gui_list_modify(&mut self, _number: u64, _row: i64, _cells: &[String]) -> Result<i64, String> {
        Err("this desktop cannot change a row of a list view".to_string())
    }

    /// The row after this one, or zero when there is none.
    fn gui_list_next(&mut self, _number: u64, _start: i64) -> Result<i64, String> {
        Err("this desktop cannot read the next row of a list view".to_string())
    }

    // -- the tree view ------------------------------------------------------------
    //
    // Measured on the binary of v2.0.19 on 2026-11-10: a TreeView is
    // `SysTreeView321`, `tv.Add(text, parent)` answers an item id, and the
    // Get* family walks the tree by those ids. This runtime numbers its items
    // sequentially from one, which is an identity a script can hold and compare.

    /// Add an item under a parent (0 = a root); answers its id.
    fn gui_tree_add(&mut self, _number: u64, _text: &str, _parent: i64, _options: &str) -> Result<i64, String> {
        Err("this desktop cannot add an item to a tree view".to_string())
    }

    /// The number of items.
    fn gui_tree_count(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot count the items of a tree view".to_string())
    }

    /// The text of an item.
    fn gui_tree_text(&mut self, _number: u64, _item: i64) -> Result<String, String> {
        Err("this desktop cannot read the text of an item of a tree view".to_string())
    }

    /// A related item's id: "parent", "child", "next" or "prev".
    fn gui_tree_relation(&mut self, _number: u64, _item: i64, _relation: &str) -> Result<i64, String> {
        Err("this desktop cannot walk a tree view".to_string())
    }

    /// The id of the selected item, or zero.
    fn gui_tree_selection(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot read the selection of a tree view".to_string())
    }

    /// Select or expand an item; answers its id.
    fn gui_tree_modify(&mut self, _number: u64, _item: i64, _options: &str) -> Result<i64, String> {
        Err("this desktop cannot change an item of a tree view".to_string())
    }

    /// Remove an item.
    fn gui_tree_delete(&mut self, _number: u64, _item: i64) -> Result<i64, String> {
        Err("this desktop cannot delete an item of a tree view".to_string())
    }

    /// A second, independent text slot on a control.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: on an Edit or a Text
    /// control `Value` and `Text` are the SAME text, but a Hotkey keeps them
    /// apart -- writing `hk.Text` does not change `hk.Value` and the other way
    /// round. A Link is the other shape: its `Value` is always empty and
    /// writing it is refused with "Invalid usage.", while `Text` is its
    /// caption. This slot is the second string for those two kinds.
    fn gui_second_value(&mut self, _number: u64, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop has no second text slot for a control".to_string())
    }

    /// The place and size of a control of the script's window, relative to its
    /// client area: (x, y, width, height).
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `ControlGetPos` with
    /// four VarRef outputs answers the four numbers. The reference does NOT
    /// hang on this name -- the earlier note was a call with plain strings.
    fn gui_control_pos(&mut self, _number: u64) -> Result<(i64, i64, i64, i64), String> {
        Err("this desktop cannot say where a control of the window is".to_string())
    }

    /// Show a menu bar on the window of the script.
    ///
    /// The JSON is an array of `[name, callback]` entries. Measured on the
    /// binary of v2.0.19 on 2026-11-10: a `MenuBar` attached with
    /// `Gui.Menu :=` shows at the top of the window.
    fn gui_menu_bar(&mut self, _json: &str) -> Result<(), String> {
        Err("this desktop cannot show a menu bar on the window of the script".to_string())
    }

    /// Put a window of the script on the screen.
    fn gui_show(&mut self, _handle: u64) -> Result<(), String> {
        Err("this desktop cannot show a window of the script".to_string())
    }

    /// Take a window of the script away.
    fn gui_destroy(&mut self, _handle: u64) -> Result<(), String> {
        Err("this desktop cannot take away a window of the script".to_string())
    }

    /// Read the text of a control of a window of the script, or write it.
    ///
    /// The number is the one `gui_add` answered, which is the `Hwnd` of the
    /// control. Measured on the binary of v2.0.19 on 2026-11-09: `Value` of a
    /// control is read and written as text.
    fn gui_value(&mut self, _number: u64, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop cannot read the text of a control of the script".to_string())
    }

    /// Read a flag of a control of a window of the script, or write it.
    ///
    /// The flag is `"enabled"` or `"visible"`. Measured on the binary of v2.0.19
    /// on 2026-11-09: both are integers of nought and one, read and written.
    fn gui_flag(
        &mut self,
        _number: u64,
        _name: &str,
        _setting: Option<bool>,
    ) -> Result<i64, String> {
        Err("this desktop cannot read a flag of a control of the script".to_string())
    }

    /// Set a state of a control of the script's window, named by its `ClassNN`.
    ///
    /// The states are `"visible"` and `"enabled"`.
    fn gui_control_state(&mut self, _classnn: &str, _what: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot set a state of a control of the script".to_string())
    }

    /// Show or hide a control of the script's window, named by its `ClassNN`.
    fn gui_visibility(&mut self, _classnn: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot show or hide a control of the script".to_string())
    }

    /// The number that names a control of the script's window, from its `ClassNN`.
    fn gui_number_named(&mut self, _classnn: &str) -> Result<i64, String> {
        Err("this desktop cannot find a control of the script by its name".to_string())
    }

    /// Which control of the script's window has the focus, as its `ClassNN`.
    fn gui_focused(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the focus of a control of the script".to_string())
    }

    /// The text a control of the script has selected.
    fn gui_selection(&mut self, _number: u64) -> Result<String, String> {
        Err("this desktop cannot read the selection of a control of the script".to_string())
    }

    /// Where the cursor of a control of the script is, as a place in its text.
    fn gui_cursor(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot read the cursor of a control of the script".to_string())
    }

    /// How many lines the text of a control of the script holds.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `EditGetLineCount` of a
    /// control added with three lines answers 3.
    fn gui_line_count(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot count the lines of a control of the script".to_string())
    }

    /// Apply one option to a window of the script, with the sign the script wrote.
    ///
    /// The options a program controls under this compositor are `AlwaysOnTop`,
    /// `Resize`, `Caption` and `Disabled`; the rest are refused with the reason.
    fn gui_option(&mut self, _name: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot set an option of the window of the script".to_string())
    }

    /// Take a window of the script off the screen.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `Hide` answers the empty
    /// text and takes no parameters.
    fn gui_hide(&mut self) -> Result<(), String> {
        Err("this desktop cannot hide the window of the script".to_string())
    }

    /// Put the focus on a control of a window of the script.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `Focus` answers the empty
    /// text and takes no parameters.
    fn gui_focus(&mut self, _number: u64) -> Result<(), String> {
        Err("this desktop cannot put the focus on a control of the script".to_string())
    }

    /// Set the size of a control of a window of the script.
    ///
    /// The **size** and not the place: measured on 2026-11-09, the window this
    /// tree makes lays its controls out in order, and GTK gives a child of a
    /// layout no place of its own.
    fn gui_size(&mut self, _number: u64, _width: i64, _height: i64) -> Result<(), String> {
        Err("this desktop cannot size a control of the script".to_string())
    }

    /// Read the events of the windows of the script that have happened.
    ///
    /// The answer is the events **since the last time it was asked**, one name
    /// each, and an empty answer is the common case and must cost almost
    /// nothing. Measured on the binary of v2.0.19 on 2026-11-09: the events a
    /// window of a script has are `close`, `escape`, and a click on a control.
    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    /// Read the title of a window of the script, or write it.
    fn gui_title(&mut self, _handle: u64, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop cannot say the title of a window of the script".to_string())
    }
}

/// The processes of the desktop.
pub trait Processes {
    /// Run a command; return its exit code, stdout and stderr.
    fn run(&mut self, command: &str, window: &str) -> Result<(i32, String, String), String>;

    /// Launch a program without waiting for it; return the pid of it.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `Run` of the
    /// v1 does not wait for the program, and it writes the pid of it into a
    /// variable of the script. The desktop that cannot launch a program says
    /// so, and not `false`.
    fn launch(&mut self, _command: &str, _working_dir: &str) -> Result<u32, String> {
        Err("this desktop cannot launch a program".to_string())
    }

    /// Launch a program and wait for it; return the pid of it, the code and
    /// what it wrote.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `RunWait` of
    /// the v1 waits for the program and writes the pid of it into a variable
    /// of the script. The desktop that cannot launch a program says so, and
    /// not `false`.
    fn run_and_wait(
        &mut self,
        _command: &str,
        _working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        Err("this desktop cannot launch a program".to_string())
    }

    /// The value of an environment variable.
    ///
    /// The value is `None` when the environment does not have the variable,
    /// and the value of a variable that is there may be the empty text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// warning of the environment is given when the variable is there, and the
    /// name that the environment does not have is a name that has no value.
    fn env_get(&mut self, _name: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot read the environment".to_string())
    }

    /// Set an environment variable; answer what it was set to.
    fn env_set(&mut self, _name: &str, _value: &str) -> Result<String, String> {
        Err("this desktop cannot write the environment".to_string())
    }

    /// The id of a process, or zero when there is no such process.
    ///
    /// The target is a pid or the name of the image. The empty text names the
    /// process of the script itself, which is what `ProcessExist()` with no
    /// argument means.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `ProcessExist()` with
    /// no argument answers the id of the script's own process; a pid that is
    /// not there answers zero; a name that is not there answers zero. A name
    /// is matched whole, the extension included:
    /// `ProcessExist("AutoHotkey64.exe")` answered the id, and
    /// `ProcessExist("AutoHotkey64")` answered zero.
    fn process_exist(&mut self, _pid_or_name: &str) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// The name of a process, or nothing when there is no such process.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the name of a process
    /// that is not there is a failure -- "Target process not found." -- and
    /// not an empty text. The name is the name of the image, extension
    /// included, and it may be given as the target as well.
    fn process_name(&mut self, _pid_or_name: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// The full path of the image of a process, or nothing when there is no
    /// such process.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the path of a process
    /// that is not there is a failure, the same one the name gives.
    fn process_path(&mut self, _pid_or_name: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// The id of the process that started a process, or zero.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the parent of a
    /// process that is not there answers zero, and it is not a failure --
    /// unlike the name and the path.
    fn process_parent(&mut self, _pid_or_name: &str) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// Stop a process, and answer the id of the one that was stopped.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: a process that is not
    /// there answers zero, and it is not a failure.
    fn process_close(&mut self, _pid_or_name: &str) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// Set the priority of a process, and answer the id of the one that was
    /// set.
    ///
    /// The priority is the word of v2 -- "Low", "BelowNormal", "Normal",
    /// "AboveNormal", "High", "Realtime" -- or a number. Measured on the
    /// binary of v2.0.19 on 2026-11-09: a process that is not there is a
    /// failure -- "Target process not found." -- and a priority that v2 does
    /// not know is *not* a failure: `ProcessSetPriority("Bogus", PID)`
    /// answered the id, so the word is not checked here either.
    fn process_set_priority(
        &mut self,
        _priority: &str,
        _pid_or_name: &str,
    ) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// Wait for a process to be there, and answer its id, or zero if the
    /// timeout came first.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: a process that is
    /// there answers its id at once, and a target that never appears answers
    /// the zero when the timeout runs out.
    ///
    /// The binary of v2.0.19 waits **forever** when the timeout is left out:
    /// `ProcessWait("a-name-that-never-appears")` with no second argument was
    /// still waiting after fifteen seconds. This runtime does not wait
    /// forever -- a script must be stoppable, and the runner's own timeout is
    /// checked between two statements, so a wait that never returns would not
    /// be stopped by it. The cap is the one of `Sleep`, 60 000 ms.
    fn process_wait(&mut self, _pid_or_name: &str, _timeout_ms: u64) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }

    /// Wait for a process to close, and answer the id of the one that is
    /// still there, or zero when it closed.
    ///
    /// The answer is the one measured on the binary of v2.0.19 on 2026-11-09,
    /// and it is not the one a reader expects:
    ///
    /// | the target | the answer | the time taken |
    /// |---|---|---|
    /// | not there | `0` | at once |
    /// | there, and closes inside the timeout | `0` | until it closed |
    /// | there, and still there when the timeout runs out | **its id** | the timeout |
    ///
    /// So the process that closed and the process that was never there answer
    /// the same thing, and the id is the answer for the one that is still
    /// running. The same cap on the timeout as `process_wait` applies.
    fn process_wait_close(&mut self, _pid_or_name: &str, _timeout_ms: u64) -> Result<u32, String> {
        Err("this desktop cannot look at the processes".to_string())
    }
}

/// The files of the desktop.
pub trait Files {
    /// Create a shortcut to a program, the way this platform makes one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `FileCreateShortcut` answers the
    /// empty text and makes the file, and it makes it even for a target that is not
    /// there.
    fn create_shortcut(
        &mut self,
        _target: &str,
        _shortcut: &str,
        _working_directory: &str,
        _arguments: &str,
        _description: &str,
        _icon: &str,
    ) -> Result<(), String> {
        Err("this desktop cannot create a shortcut".to_string())
    }

    /// Empty the trash of the machine.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `FileRecycleEmpty` answers the
    /// empty text.
    fn empty_the_trash(&mut self) -> Result<(), String> {
        Err("this desktop cannot empty the trash".to_string())
    }

    /// Put one file in the trash of the machine.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `FileRecycle(path)`
    /// answers the empty text and the file is gone. An earlier note said the
    /// reference answers "Failed" for a file that exists -- that was a path
    /// that was not there, not the name.
    fn recycle_a_file(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot put a file in the trash".to_string())
    }

    /// Read a shortcut back: (target, dir, args, description, icon).
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `FileGetShortcut` of a
    /// shortcut answers all eight of its fields. The icon NUMBER and the run
    /// state have no counterpart in a `.desktop` file, so those two answers
    /// are the empty text here.
    fn read_shortcut(&mut self, _shortcut: &str) -> Result<(String, String, String, String, String), String> {
        Err("this desktop cannot read a shortcut".to_string())
    }

    /// The directory the script is in.
    ///
    /// The name of the directory is a subfolder of the working directory
    /// when it is not an absolute path.
    fn set_working_directory(&mut self, _directory: &str) -> Result<String, String> {
        Err("this desktop cannot change the working directory".to_string())
    }

    /// The files of a directory that match a pattern.
    ///
    /// The mode is v2's: "R" walks into the subfolders as well, and "D"
    /// gives the directories instead of the files. The "A" gives the files
    /// and the directories together, and it is the mode of the v1 that has
    /// the number one.
    fn list_files(
        &mut self,
        _folder: &str,
        _pattern: &str,
        _mode: &str,
    ) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the files of a directory".to_string())
    }

    /// The directory where the run started.
    ///
    /// It is the directory the run began in, and not the one that the run
    /// is in now: a script that changes the working directory does not
    /// change this one.
    fn initial_working_directory(&mut self) -> Result<String, String> {
        Err("this desktop cannot say where the run started".to_string())
    }

    /// The directory the script runs in.
    fn working_directory(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell where the script runs".to_string())
    }

    /// The lines of a file.
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Err("this desktop cannot read a file".to_string())
    }

    /// The bytes of a file.
    ///
    /// The read of a file is the bytes of it: the dialect reads them in the
    /// encoding that it asks for, and the machine keeps them as they are.
    fn read_the_bytes_of_a_file(&mut self, _path: &str) -> Result<Vec<u8>, String> {
        Err("this desktop cannot read a file".to_string())
    }

    /// Write a file, replacing what was in it; return what was written.
    fn write_file(&mut self, _path: &str, _text: &str) -> Result<String, String> {
        Err("this desktop cannot write a file".to_string())
    }

    /// Add to the end of a file; return what was added.
    ///
    /// The bytes are the ones the dialect wrote, and the mark is the mark of
    /// the encoding of the file: the machine writes it when the file is
    /// made, and not when the file is already there.
    fn append_file(&mut self, _path: &str, _bytes: &[u8], _mark: &[u8]) -> Result<String, String> {
        Err("this desktop cannot write a file".to_string())
    }

    /// The attributes of the first entry that matches the pattern.
    ///
    /// The pattern is v2's: the last part of it may carry the wildcards
    /// `*` and `?`, and the answer is the attributes of the first entry
    /// that matches, a folder among them. The empty list is the answer
    /// when there is none. A folder is asked for with the mode "D", and
    /// any entry with the empty mode.
    fn attributes_of_the_pattern(
        &mut self,
        _pattern: &str,
        _mode: &str,
    ) -> Result<Vec<String>, String> {
        Err("this desktop cannot look for a file".to_string())
    }

    /// Take a file away.
    fn delete_file(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot delete a file".to_string())
    }

    /// Make a directory.
    fn create_dir(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot make a directory".to_string())
    }

    /// Take a directory away.
    ///
    /// The flag asks for the folder with the contents of it: v2 has the
    /// option, and without it a folder that is not empty stays.
    fn dir_delete(&mut self, _path: &str, _recurse: bool) -> Result<String, String> {
        Err("this desktop cannot delete a directory".to_string())
    }

    /// Open a file and answer the handle of it.
    ///
    /// The mode is the way AutoHotkey spells it: `r`, `w`, `a` or `rw`.
    fn open_file(&mut self, _path: &str, _mode: &str) -> Result<i64, String> {
        Err("this desktop cannot open a file".to_string())
    }

    /// The bytes of an open file, at a place in it.
    fn read_file_at(&mut self, _handle: i64, _pos: i64, _count: i64) -> Result<Vec<u8>, String> {
        Err("this desktop cannot read a file".to_string())
    }

    /// Write bytes to an open file, at a place in it; answer how many
    /// were written.
    fn write_file_at(&mut self, _handle: i64, _pos: i64, _bytes: &[u8]) -> Result<usize, String> {
        Err("this desktop cannot write a file".to_string())
    }

    /// How big an open file is.
    fn file_size(&mut self, _handle: i64) -> Result<i64, String> {
        Err("this desktop cannot tell how big a file is".to_string())
    }

    /// How big a file is, in the bytes of it.
    ///
    /// The answer is the size of the file and not the size of a folder: the
    /// machine gives the size of a folder as zero.
    fn the_size_of_a_file(&mut self, _path: &str) -> Result<i64, String> {
        Err("this desktop cannot tell how big a file is".to_string())
    }

    /// Close an open file.
    fn close_file(&mut self, _handle: i64) -> Result<(), String> {
        Err("this desktop cannot close a file".to_string())
    }

    /// Take a copy of a file.
    fn copy_file(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
        Err("this desktop cannot copy a file".to_string())
    }

    /// Move a file.
    fn move_file(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
        Err("this desktop cannot move a file".to_string())
    }

    /// Take a copy of a folder and of everything in it.
    fn copy_dir(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
        Err("this desktop cannot copy a folder".to_string())
    }

    /// Move a folder.
    fn move_dir(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
        Err("this desktop cannot move a folder".to_string())
    }

    /// The attributes of a file.
    fn file_attributes(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Err("this desktop cannot read the attributes of a file".to_string())
    }

    /// Change the attributes of a file.
    fn set_file_attributes(
        &mut self,
        _path: &str,
        _attributes: &[String],
    ) -> Result<String, String> {
        Err("this desktop cannot change the attributes of a file".to_string())
    }

    /// The times of a file: when it was made, when it was changed, when it
    /// was read.
    fn file_times(&mut self, _path: &str) -> Result<(i64, i64, i64), String> {
        Err("this desktop cannot read the times of a file".to_string())
    }

    /// Change the times of a file.
    ///
    /// A time of nothing is a time that does not change.
    fn set_file_times(
        &mut self,
        _path: &str,
        _created: Option<i64>,
        _modified: Option<i64>,
        _accessed: Option<i64>,
    ) -> Result<String, String> {
        Err("this desktop cannot change the times of a file".to_string())
    }

    /// The value of a key of an ini file.
    fn ini_read(
        &mut self,
        _path: &str,
        _section: &str,
        _key: &str,
        _default: &str,
    ) -> Result<String, String> {
        Err("this desktop cannot read an ini file".to_string())
    }

    /// Write a key of an ini file.
    fn ini_write(
        &mut self,
        _path: &str,
        _section: &str,
        _key: &str,
        _value: &str,
    ) -> Result<String, String> {
        Err("this desktop cannot write an ini file".to_string())
    }

    /// Take a key of an ini file away.
    fn ini_delete(&mut self, _path: &str, _section: &str, _key: &str) -> Result<String, String> {
        Err("this desktop cannot write an ini file".to_string())
    }
}

/// The clipboard of the desktop.
pub trait Clipboard {
    /// What is on the clipboard.
    fn clipboard_read(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the clipboard".to_string())
    }

    /// Put something on the clipboard.
    fn clipboard_write(&mut self, _text: &str) -> Result<String, String> {
        Err("this desktop cannot write the clipboard".to_string())
    }
}

/// The timers of the desktop.
///
/// The runtime runs the timer of a script between the statements of the
/// script, so the budget of a timer is the budget of the script. This is
/// asked only of a desktop that wants the timers of its own.
pub trait Timers {
    /// Whether the desktop takes the timers of the script over.
    fn takes_over_timers(&mut self) -> bool {
        false
    }
}

/// The hotkeys of the desktop.
pub trait Hotkeys {
    /// Register a hotkey, spelled the way the script spells it.
    fn register_hotkey(&mut self, _name: &str) -> Result<String, String> {
        Err("this desktop cannot register a hotkey".to_string())
    }

    /// Take a hotkey away.
    fn unregister_hotkey(&mut self, _name: &str) -> Result<String, String> {
        Err("this desktop cannot register a hotkey".to_string())
    }

    /// The hotkeys that are registered.
    fn hotkeys(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the hotkeys".to_string())
    }

    /// The hotkey that was pressed since the last question.
    ///
    /// The desktop watches the keyboard and says which of the hotkeys it
    /// registered was pressed; the runtime owns the function that the
    /// hotkey runs, and asks this between the statements of the script.
    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
        Err("this desktop cannot watch the keyboard".to_string())
    }
}

/// The native calls of the desktop.
///
/// A call into a library of the machine is not something the language
/// can do: the crate does no I/O of its own, and the desktop is the way
/// out. The call carries the blocks the callee may write into, and the
/// answer comes back with them.
pub trait Native {
    /// The call of a function of a library.
    fn call_native(&mut self, _call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        Err(NativeFailure::invalid(
            "this desktop cannot make a native call.",
        ))
    }
}

/// The pictures of the desktop.
///
/// A picture is an image of the machine, and the machine is the one
/// that reads it: a desktop that does not read the images says so.
pub trait Pictures {
    /// Load a picture of a file, and answer the number that identifies
    /// it.
    ///
    /// `None` is the answer when the picture is not there: v2 answers
    /// zero in that case, and not a failure.
    fn load_picture(
        &mut self,
        _filename: &str,
        _options: &PictureOptions,
    ) -> Result<Option<u32>, String> {
        Err("the picture is read by the desktop, and this desktop does not read the images of the machine.".to_string())
    }

    /// Look for a picture on the screen.
    fn image_search(
        &mut self,
        _x1: i64,
        _y1: i64,
        _x2: i64,
        _y2: i64,
        _image: &Image,
        _options: &ImageOptions,
    ) -> Result<ImageSearch, String> {
        Err("the search of a picture is made by the desktop, and this desktop does not read the images of the machine.".to_string())
    }
}

/// The network of the desktop.
///
/// A download is a file that comes from the network, and the network is
/// the machine's: a desktop that does not reach it says so.
pub trait Network {
    /// Fetch a file of the network and write it on the disk.
    ///
    /// The answer is the path of the file that was written.
    fn download(&mut self, _url: &str, _file: &str) -> Result<String, String> {
        Err(
            "the download is made by the desktop, and this desktop does not reach the network."
                .to_string(),
        )
    }

    /// The IPv4 addresses of the interfaces of the machine.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `SysGetIPAddresses`
    /// answers an **array** of the addresses as text, in the order the machine
    /// lists them.
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the addresses of its interfaces.".to_string())
    }
}

/// The options of a picture.
///
/// The options are the ones of v2: the size the picture is loaded at,
/// and the icon of a file that has many. The runtime reads them, and
/// the desktop keeps the ones that are about the machine.
#[derive(Clone, Debug, Default)]
pub struct PictureOptions {
    /// The width of the picture, in pixels.
    pub width: Option<i64>,
    /// The height of the picture, in pixels.
    pub height: Option<i64>,
    /// The icon of a file that has many.
    pub icon: Option<i64>,
}

/// The options of a search of the screen for a picture.
#[derive(Clone, Debug, Default)]
pub struct ImageOptions {
    /// How much the colours of the screen may differ from the picture.
    pub variation: i64,
    /// The size the picture is looked for at.
    pub width: Option<i64>,
    /// The size the picture is looked for at.
    pub height: Option<i64>,
    /// The colour of the picture that matches any colour of the screen.
    pub transparent: Option<String>,
}

/// The picture of a search.
///
/// v2 writes the picture as the name of a file, and it writes the
/// handle of a picture that was loaded before in the spelling of
/// Windows: "HBITMAP:*123". The runtime reads both.
#[derive(Clone, Debug)]
pub enum Image {
    /// A picture of a file.
    File(String),
    /// A picture that was loaded before.
    Loaded(u32),
}

/// The answer of a search of the screen for a picture.
#[derive(Clone, Debug)]
pub enum ImageSearch {
    /// The picture is not on the screen.
    NotFound,
    /// The picture is on the screen, at this place.
    Found(i64, i64),
    /// The picture could not be loaded.
    NotLoadable,
}

/// The sound of the machine.
///
/// The names are the ones of v2: the volume of a device, whether it is
/// muted, and the playing of a sound. The device is named by the script
/// with the name of the device, or with `#` and its number -- `#1` is
/// the first device -- and the empty name is the device of the system.
pub trait Sound {
    /// The volume of a device, as a number between 0 and 100.
    fn sound_volume(&mut self, _device: &str) -> Result<f64, String> {
        Err("this desktop cannot say the volume of a device".to_string())
    }

    /// Set the volume of a device, as a number between 0 and 100.
    fn sound_set_volume(&mut self, _device: &str, _volume: f64) -> Result<(), String> {
        Err("this desktop cannot set the volume of a device".to_string())
    }

    /// Whether a device is muted.
    fn sound_muted(&mut self, _device: &str) -> Result<bool, String> {
        Err("this desktop cannot say whether a device is muted".to_string())
    }

    /// Mute a device, or unmute it.
    fn sound_set_muted(&mut self, _device: &str, _muted: bool) -> Result<(), String> {
        Err("this desktop cannot mute a device".to_string())
    }

    /// Play a sound file.
    fn sound_play(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot play a sound".to_string())
    }

    /// Play a tone of a frequency, for a number of milliseconds.
    fn sound_beep(&mut self, _frequency: f64, _duration_ms: u64) -> Result<String, String> {
        Err("this desktop cannot play a tone".to_string())
    }
}

/// The machine, the session and the user.
///
/// The names are the ones of v2: the name of the user, the name of the
/// computer, the shell of the session, the folders of the machine, the
/// version of the operating system, the language of the session, whether
/// the script is elevated, and how long the session has been idle.
///
/// The folders are the ones the machine has. A folder that the machine
/// does not have is not in the answer: the name that asks for it says
/// so.
pub trait Machine {
    /// The name of the user that launched the session.
    fn user_name(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the name of the user".to_string())
    }

    /// The name of the computer, as the network sees it.
    fn computer_name(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the name of the computer".to_string())
    }

    /// The program that runs a command of the session.
    ///
    /// v2 calls it `A_ComSpec`, and on the machine of the box it is the
    /// shell of the session.
    fn shell(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the shell of the session".to_string())
    }

    /// The folders of the machine.
    fn folders(&mut self) -> Result<Folders, String> {
        Err("this desktop cannot say the folders of the machine".to_string())
    }

    /// The version of the operating system.
    fn os_version(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the version of the operating system".to_string())
    }

    /// The language of the session.
    fn language(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the language of the session".to_string())
    }

    /// Whether the script runs with the rights of an administrator.
    fn is_admin(&mut self) -> Result<bool, String> {
        Err("this desktop cannot say whether the script is elevated".to_string())
    }

    /// How long the session has been idle, in milliseconds.
    /// The path of the executable that is running the script.
    ///
    /// It is what `A_AhkPath` answers. A desktop that cannot tell says so, and
    /// the variable is then a refusal and not an empty text: a script that
    /// reads the path of the interpreter must be told that it is not known.
    fn executable(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell the path of the interpreter".to_string())
    }

    /// The space of the drive that a path is on.
    ///
    /// `None` is a path whose drive is not there. A path that is there and a
    /// drive whose space cannot be read is a failure with the reason.
    fn drive_space(&mut self, _path: &str) -> Result<Option<DriveSpace>, String> {
        Err("this desktop cannot tell the space of a drive".to_string())
    }

    fn idle_times(&mut self) -> Result<Idle, String> {
        Err("this desktop cannot say how long the session has been idle".to_string())
    }
}

/// The dialogs of the desktop.
///
/// A dialog asks the user a question and waits for the answer. The
/// answer of an input box is the text the user typed and the way the
/// dialog was closed; the answer of a file selection is the path, or
/// the paths, of what the user chose.
///
/// The options of a dialog are read by the runtime before the question
/// is asked: an option that v2 does not know is refused there, and the
/// desktop is never asked about it.
pub trait Dialogs {
    /// Ask the user for a text.
    ///
    /// The answer carries the text the user typed and the way the
    /// dialog was closed: "OK", "Cancel" or "Timeout".
    fn input_box(
        &mut self,
        _prompt: &str,
        _title: &str,
        _options: &InputOptions,
        _default: &str,
    ) -> Result<InputAnswer, String> {
        Err("this desktop cannot ask the user for a text".to_string())
    }

    /// Ask the user for a file, or for a folder.
    ///
    /// The answer is one path, or many of them when the user may choose
    /// more than one.
    fn file_select(
        &mut self,
        _options: &FileOptions,
        _root: &str,
        _title: &str,
        _filter: &str,
    ) -> Result<FileAnswer, String> {
        Err("this desktop cannot ask the user for a file".to_string())
    }

    /// Ask the user for a folder.
    ///
    /// The options are the number of v2: 1 is the button that makes a
    /// new folder, 2 is the field where the name is written, and 4 is
    /// the old style of the dialog.
    fn dir_select(
        &mut self,
        _starting: &str,
        _options: i64,
        _prompt: &str,
    ) -> Result<String, String> {
        Err("this desktop cannot ask the user for a folder".to_string())
    }

    /// Tell the user something, without asking anything back.
    ///
    /// The title is the first line of it and the text is the body. A desktop
    /// that has no way to tell the user anything says so, and not `false`.
    ///
    /// The `TrayTip` of v2 is the name of this: on Windows it is a balloon in
    /// the tray, and on a Linux desktop it is a notification of the session,
    /// which is the same thing said to the same person. The refusal of the
    /// tree used to call it a name of Windows, which was false: the box has
    /// `notify-send` on it.
    fn notify(&mut self, _title: &str, _text: &str) -> Result<(), String> {
        Err("this desktop cannot show a notification".to_string())
    }
}

/// The answer of an input box.
#[derive(Clone, Debug, Default)]
pub struct InputAnswer {
    /// The text the user typed.
    pub value: String,
    /// The way the dialog was closed: "OK", "Cancel" or "Timeout".
    pub result: String,
}

/// The answer of a file selection.
#[derive(Clone, Debug)]
pub enum FileAnswer {
    /// One file, or one folder.
    One(String),
    /// Many files, or many folders.
    Many(Vec<String>),
}

/// The options of an input box.
///
/// The options are the ones of v2: where the window is, how big it is,
/// how long it waits, and whether the text is hidden. The runtime
/// reads them, and gives the desktop the ones that are about the
/// machine.
#[derive(Clone, Debug, Default)]
pub struct InputOptions {
    /// The seconds the window waits for the user before it closes itself.
    pub timeout: Option<f64>,
    /// Whether the text the user types is hidden.
    pub password: bool,
    /// The width of the window, in pixels.
    pub width: Option<i64>,
    /// The height of the window, in pixels.
    pub height: Option<i64>,
    /// Where the window is, in pixels, from the left of the screen.
    pub x: Option<i64>,
    /// Where the window is, in pixels, from the top of the screen.
    pub y: Option<i64>,
}

/// The options of a file selection.
#[derive(Clone, Debug, Default)]
pub struct FileOptions {
    /// The number of the flags of v2, when the options were a number.
    ///
    /// The flags are the ones of the documentation: 1 is a file that
    /// must be there, 2 is a path that must be there, 8 asks to make a
    /// new file, 16 asks to overwrite one and 32 keeps the shortcuts as
    /// they are.
    pub flags: i64,
    /// Whether the user may choose a folder instead of a file.
    pub directory: bool,
    /// Whether the user may choose more than one file.
    pub multiple: bool,
    /// Whether the dialog is a save dialog.
    pub save: bool,
}

/// The folders of the machine, as the names of v2 ask for them.
///
/// A folder that is not there is a folder the machine does not have.
#[derive(Clone, Debug, Default)]
pub struct Folders {
    /// `A_AppData`: the application data of the user.
    pub app_data: Option<String>,
    /// `A_AppDataCommon`: the application data of everybody.
    pub app_data_common: Option<String>,
    /// `A_Desktop`: the desktop of the user.
    pub desktop: Option<String>,
    /// `A_DesktopCommon`: the desktop of everybody.
    pub desktop_common: Option<String>,
    /// `A_StartMenu`: the start menu of the user.
    pub start_menu: Option<String>,
    /// `A_StartMenuCommon`: the start menu of everybody.
    pub start_menu_common: Option<String>,
    /// `A_Programs`: the programs of the user.
    pub programs: Option<String>,
    /// `A_ProgramsCommon`: the programs of everybody.
    pub programs_common: Option<String>,
    /// `A_Startup`: the startup folder of the user.
    pub startup: Option<String>,
    /// `A_StartupCommon`: the startup folder of everybody.
    pub startup_common: Option<String>,
    /// `A_MyDocuments`: the documents of the user.
    pub my_documents: Option<String>,
    /// `A_Temp`: the folder of the temporary files.
    pub temp: Option<String>,
}

/// The idle times of the session, in milliseconds.
#[derive(Clone, Copy, Debug, Default)]
pub struct Idle {
    /// `A_TimeIdle`: since the last input of any kind.
    pub any: u64,
    /// `A_TimeIdlePhysical`: since the last input of the user.
    pub physical: u64,
    /// `A_TimeIdleKeyboard`: since the last key of the user.
    pub keyboard: u64,
    /// `A_TimeIdleMouse`: since the last move of the mouse.
    pub mouse: u64,
}

/// The sum of the capabilities: what the runtime drives.
///
/// The trait has no methods of its own. It is the sum, so that the
/// runtime can hold one thing and ask it for everything.
pub trait Desktop:
    Keyboard
    + Pointer
    + Screen
    + Windows
    + Processes
    + Files
    + Clipboard
    + Timers
    + Hotkeys
    + Native
    + Sound
    + Machine
    + Dialogs
    + Network
    + Pictures
{
}
