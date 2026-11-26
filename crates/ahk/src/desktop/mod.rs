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

pub mod criteria;

use crate::dll::{NativeArg, NativeCall, NativeFailure, NativeResult, NativeType};
use crate::runtime::Value;
use alloc::string::{String, ToString};
use alloc::vec;
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

    /// The state of a lock light of the keyboard.
    ///
    /// The three locks of a keyboard are CapsLock, NumLock and ScrollLock.
    /// `None` is a name that is not a lock of a keyboard.
    ///
    /// Measured on the binary of v2.0.19: `SetCapsLockState
    /// ("On")` makes `GetKeyState("CapsLock", "T")` answer 1, and "Off"
    /// makes it answer 0 -- the state a program sets is the state it reads
    /// back.
    fn lock_light(&mut self, _name: &str) -> Result<Option<bool>, String> {
        Err("this desktop cannot tell the state of a lock light".to_string())
    }

    /// Set a lock light of the keyboard, and say whether it is on now.
    fn set_lock_light(&mut self, _name: &str, _on: bool) -> Result<bool, String> {
        Err("this desktop cannot set a lock light".to_string())
    }

    /// Turn a lock key on or off.
    ///
    /// The desktop that can read the state of a lock key but not change it
    /// says so, and the state of the key is not touched.
    fn set_lock(&mut self, _lock: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot change the state of the lock keys".to_string())
    }

    /// Whether a lock key is on: "CapsLock", "NumLock" or "ScrollLock".
    ///
    /// Measured on the binary of v2.0.19: a lock key is a key
    /// whose state is a switch, and not a key that is held: `SetCapsLockState`
    /// turns the switch, and the state of it is what `GetKeyState` answers for
    /// the mode "T".
    fn lock_is_on(&mut self, _lock: &str) -> Result<bool, String> {
        Err("this desktop cannot read the state of the lock keys".to_string())
    }

    /// The name of the keyboard layout of the machine.
    ///
    /// The name is the one the platform gives it -- the KLID of Windows,
    /// the keymap of the session -- and the empty text when the platform
    /// has no name to give, the way KeySharp answers it. A machine that
    /// cannot say at all is a failure with the reason.
    fn keyboard_layout_name(&mut self) -> Result<String, String> {
        Err("this desktop cannot say the keyboard layout".to_string())
    }

    /// The virtual key and the scan code of a key in a layout.
    ///
    /// The `GetKeyInfo(KeyName, Layout)` of KeySharp resolves the key in
    /// the layout that is named. The tree answers the key of its own
    /// table, and this adjusts it for the layout: the virtual key of a
    /// character and the scan code of the key. A desktop that cannot
    /// resolve a key in a layout says so.
    fn key_in_layout(&mut self, _vk: u32, _name: &str, _layout: &str) -> Result<(u32, u32), String> {
        Err("this desktop cannot resolve a key in a keyboard layout".to_string())
    }
}

/// The pointer of the desktop.
pub trait Pointer {
    /// The name of the shape of the cursor of the machine.
    ///
    /// Measured on the binary of v2.0.19: `A_Cursor` answers
    /// the name of the shape -- "IBeam" for the shape of a field of text.
    fn cursor_shape(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell the shape of the cursor".to_string())
    }

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

    /// Turn the wheel of the mouse.
    ///
    /// The direction is the name of one of the four wheels -- the top,
    /// the bottom, the left and the right -- and the amount is how many
    /// teeth it turns.
    ///
    /// Measured on the binary of v2.0.31: the
    /// `MouseClick "WheelUp"` answers and the wheel turns.
    fn mouse_wheel(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _direction: &str,
        _amount: u32,
    ) -> Result<String, String> {
        Err("this desktop cannot turn the wheel of the mouse".to_string())
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

    /// Confine the pointer to a rectangle of the screen, or release it.
    ///
    /// The rectangle is `(left, top, right, bottom)`, the far edges
    /// exclusive, and `None` releases the confinement. A desktop that
    /// does not own the pointer cannot confine it, and says so.
    fn clip_cursor(&mut self, _rect: Option<(i64, i64, i64, i64)>) -> Result<(), String> {
        Err("this desktop cannot confine the pointer".to_string())
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
    /// Measured on the binary of v2.0.19: `WinGetControls` answers the
    /// three of a window with a label, an edit and a button, as `Static1`, `Edit1` and
    /// `Button1`.
    fn gui_controls(&mut self) -> Result<String, String> {
        Err("this desktop cannot list the controls of the window of the script".to_string())
    }

    /// The text of every control of the script's window, each followed by a line break.
    ///
    /// Measured on the binary of v2.0.19: `WinGetText` of a window with a
    /// label and an edit answers the two texts, one after the other, each with a break.
    fn gui_all_text(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the text of the window of the script".to_string())
    }

    /// The opacity of the window of the script, or set it.
    ///
    /// Measured on the binary of v2.0.19: `WinSetTransparent` counts
    /// from nought to 255, and `WinGetTransparent` answers the number that was set.
    fn gui_opacity(&mut self, _setting: Option<i64>) -> Result<i64, String> {
        Err("this desktop cannot read the opacity of the window of the script".to_string())
    }

    /// Put a text into a control of the script at its cursor.
    ///
    /// Measured on the binary of v2.0.19: `EditPaste` takes the text
    /// **first** and the handle second, as the whole family does.
    fn gui_paste(&mut self, _number: u64, _text: &str) -> Result<(), String> {
        Err("this desktop cannot put a text into a control of the script".to_string())
    }

    /// Whether a check box of the script is ticked, and ticking it.
    ///
    /// Measured on the binary of v2.0.19: the `Value` of a check box
    /// is a **number**, nought or one, and its `Text` is its label.
    fn gui_checked(&mut self, _number: u64, _setting: Option<bool>) -> Result<i64, String> {
        Err("this desktop cannot read the tick of a check box of the script".to_string())
    }

    /// The dots per inch of the output, as the session reports it.
    ///
    /// This is the number `A_ScreenDPI` answers, the way v2 gives it: the 96
    /// of the ordinary session with the scale of the session applied -- 120
    /// at one hundred and twenty-five per cent, 144 at one hundred and fifty,
    /// 192 at two hundred. It is not the scale as a factor and not the scale
    /// as a percentage: the two would have to be divided before the answer,
    /// and the tree asks the desktop for the number the script reads.
    fn screen_dpi(&mut self) -> Result<i64, String> {
        Err("this desktop does not know the scale of the output".to_string())
    }

    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        Err("this desktop cannot tell how big the screen is".to_string())
    }

    /// Show a small window of text, and answer the number that names it. An empty
    /// text takes it away. Measured on the binary of v2.0.19:
    /// showing one answers an Integer and `ToolTip()` clears it.
    fn tooltip_show(&mut self, _text: &str, _x: i64, _y: i64) -> Result<i64, String> {
        Err("this desktop cannot show a tooltip".to_string())
    }

    /// Move and size the window of the script. Measured on the binary of v2.0.19
    ///: `WinMove` of the window the script made works and answers
    /// nothing. A Wayland compositor places windows itself, so the call is the
    /// one the toolkit offers and need not be honoured on the screen.
    fn window_move(&mut self, _x: i64, _y: i64, _width: i64, _height: i64) -> Result<(), String> {
        Err("this desktop cannot move the window of the script".to_string())
    }

    /// The monitors of the desktop, in the order it lists them.
    ///
    /// Measured on the binary of v2.0.19: `MonitorGetCount`
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

    /// The work area of a monitor: left, top, right and bottom.
    ///
    /// The work area is the part of the monitor that the panels and the
    /// taskbars leave. `None` is a monitor that is not there.
    fn monitor_work_area(
        &mut self,
        _monitor: usize,
    ) -> Result<Option<(i64, i64, i64, i64)>, String> {
        Err("this desktop cannot say the work area of its monitors".to_string())
    }

    /// The place of the primary monitor, counted from one, among the
    /// monitors of the desktop.
    fn primary_monitor(&mut self) -> Result<usize, String> {
        Err("this desktop cannot say which monitor is the primary one".to_string())
    }

    /// A metric of the system (`SysGet` of a number that is not about the
    /// monitors): the window manager of the machine answers it, and a
    /// desktop that has no window manager of its own has none.
    fn system_metric(&mut self, _metric: i32) -> Option<i64> {
        None
    }
}

/// The way the title of a window is matched with the text that is wanted.
///
/// Measured on the binary of v2.0.19 and read in the
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

/// The shape of the region of a window.
///
/// The `None` is the region of the window itself: the window takes the
/// shape that it has.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowRegionKind {
    /// The window has no region of its own.
    #[default]
    None,
    /// The region is a rectangle.
    Rectangle,
    /// The region is a rectangle with rounded corners.
    RoundedRectangle,
    /// The region is an ellipse.
    Ellipse,
    /// The region is a polygon.
    Polygon,
}

/// The region of a window: the shape that the window takes on the screen.
///
/// The region of the v1 is a text of points and of letters, and the
/// runtime reads that text and gives the shape to the machine.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WindowRegion {
    /// The shape of the region.
    pub kind: WindowRegionKind,
    /// The points of the polygon, when the region is one.
    pub points: Vec<(i64, i64)>,
    /// The left of the shape.
    pub x: i64,
    /// The top of the shape.
    pub y: i64,
    /// The width of the shape.
    pub width: i64,
    /// The height of the shape.
    pub height: i64,
    /// The width of the corner of the rounded rectangle.
    pub radius_x: i64,
    /// The height of the corner of the rounded rectangle.
    pub radius_y: i64,
    /// Whether the polygon is filled by the winding rule, and not by the
    /// rule of the alternate areas.
    pub winding: bool,
}

/// A window of the desktop, as the listing of the windows describes it.
///
/// The two things that v2 reads of a window are the ones here: the identifier,
/// which is what `WinGetID` answers and what the entries of `WinGetList` are,
/// and the class, which is what `WinGetClass` answers.
///
/// The identifier of a Wayland window is a token that the compositor gives it,
/// and not a number, so it is text here. Measured on the binary of v2.0.19 the id of a window of Windows is a whole number, and a script
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
/// of v2.0.19: `DriveGetSpaceFree("C:\")` answered `836675` and
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
    /// The scale of the monitor: the dots per inch of it, 96 for the ordinary
    /// one, the same number `screen_dpi` answers for the screen.
    pub scale: i64,
}

/// What became of a question about a control of a window.
///
/// The two failures are different facts and v2 tells them apart. Measured on the
/// binary of v2.0.19: `ControlGetText("x", "zzz-no-such-window-zzz")`
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
/// Measured on the binary of v2.0.19 with a window of its own
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
/// Measured on the binary of v2.0.19: `ControlSetText` answers the
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
    /// v2.0.19: `WinGetList()` and `WinGetList("")` both
    /// answered every window of the desktop, and `WinGetCount()` with no
    /// argument answered the number of them -- so an empty title is not "no
    /// window", and it is not "the last found window" either: the runtime
    /// resolves the last found window before it asks the desktop, and the
    /// listing asks about every window.
    fn matching_windows(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// The `ClassNN` of every control of a window of the machine, or the
    /// identifiers of them.
    ///
    /// The v1 asks about the window that is named: the `WinGet` with the
    /// sub-command `ControlList` is the list of the controls of that
    /// window, and not the list of the controls of the window of the
    /// script.
    ///
    /// The identifier of the window is the one that the desktop names it
    /// with, and the list is separated by a line break. The number of
    /// each class is the number of the controls of that class: the first
    /// button is `Button1` and the second is `Button2`.
    ///
    /// Measured on the binary of v1.1.37.02: the `WinGet, out,
    /// ControlList, <title>` of a window that is not there answers the
    /// empty text, and the sub-command `ControlListHwnd` answers the
    /// identifiers of the controls, and not their names.
    fn window_controls(&mut self, _id: &str, _handles: bool) -> Result<Option<String>, String> {
        Err("this desktop cannot list the controls of a window".to_string())
    }

    /// The texts of the controls of a window, and the empty text when the
    /// window has none.
    ///
    /// The texts are separated by a line break, and they are the texts of
    /// the child windows of the machine. Measured on the binary of the
    /// v2.0.32: the `WinGetText` of a window of another program answers the
    /// texts of its child windows (source `lib/win.cpp:2075` of it: the
    /// `EnumChildWindows` of the target window).
    ///
    /// The window of the script is asked with `gui_all_text`, and a window of
    /// another program is asked by its identifier: the accessibility tree of
    /// another program carries its title bar with its content, and the
    /// desktop that cannot separate them refuses with that reason.
    fn window_text(&mut self, _id: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot read the text of a window of another program".to_string())
    }

    /// The name of the executable of the window, and the empty text when the
    /// desktop does not say.
    ///
    /// The `ahk_exe` criterion compares the name of the file of the
    /// executable, without the folder: `python.exe`, and not the whole
    /// path. The name is read only when the criterion asks for it: it is
    /// a call that opens the process of each window, and a list that
    /// nobody asks for does not pay for it.
    fn window_exe(&mut self, _id: i64) -> String {
        String::new()
    }

    /// Where the window is and how big it is.
    ///
    /// `None` is a window that is not there. A window that **is** there but
    /// whose geometry the compositor did not describe is a failure with the
    /// reason, and not a geometry of zeros: a script must be told that the
    /// place is not known, and not be given a window at the origin.
    ///
    /// Measured on the binary of v2.0.19: `WinGetPos` writes
    /// its four answers into four variables given by reference, and it
    /// answers the empty text. A window that is not there is a failure --
    /// "Target window not found." -- and a call with no window at all is the
    /// same failure, because the runtime resolved the window that the script
    /// found last and there was none.
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
    /// Measured on the binary of v2.0.19, on a window of its
    /// own: `0` before, `1` after `WinMaximize`, `0` after `WinRestore`, and
    /// `-1` after `WinMinimize`.
    fn window_min_max(&mut self, _title: &str, _mode: TitleMatch) -> Result<Option<i64>, String> {
        Err("this desktop cannot tell whether a window is maximized".to_string())
    }

    /// Wait until a condition about a window holds.
    ///
    /// `Some` is the condition met, and the empty text is the answer of the
    /// conditions that are about the absence of a window -- `NotActive` and
    /// `Close` -- because there is no window to identify. `None` is the
    /// timeout.
    ///
    /// The empty title is no window at all: the v1 and the v2 fall back to
    /// the last found window, and the runtime resolves that window before
    /// it asks the desktop, so the desktop sees the title of it -- or the
    /// empty title when there is no window found, and the empty title is
    /// the title of no window at all: the conditions that talk about the
    /// absence of a window hold at once and the ones that talk about its
    /// presence never do.
    /// Measured on the binary of v2.0.19: `WinWaitNotActive("no-such")`
    /// and `WinWaitClose("no-such")` answered `1` at once, and
    /// `WinWait`/`WinWaitActive` answered `0` when the timeout ran out.
    ///
    /// The title that is empty with an exclusion is not the same thing: the
    /// title that is empty is the title of any window, and the exclusion
    /// takes that window out of the answer.
    ///
    /// Measured on the binary of v1.1.37.02: the `WinWaitClose` with no
    /// title at all answers at once and leaves the `ErrorLevel` at zero.
    /// Measured on the binary of v2.0.32: the `WinWaitClose()` with no
    /// title at all answers `1` at once, and the `WinWait()` with no title
    /// at all waits for a window that never comes.
    ///
    /// The exclusions are the titles of the windows that do not answer the
    /// wait, and they are measured against the title of the window with the
    /// mode of the script. The exclusion by the text of the window needs the
    /// text of the controls, and the desktop does not have it.
    fn window_wait(
        &mut self,
        _title: &str,
        _mode: TitleMatch,
        _what: WaitFor,
        _timeout_ms: u64,
        _excludes: &[String],
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

    /// The style of a control of a window, or its extended style.
    ///
    /// A control is a window of its own, and its style is the number of
    /// the bits of Windows the way the style of a window is.
    fn control_style(
        &mut self,
        _control: &str,
        _title: &str,
        _extended: bool,
    ) -> Result<i64, String> {
        Err("this desktop cannot read the style of a control".to_string())
    }

    /// Write the style of a control of a window, or its extended style.
    fn control_set_style(
        &mut self,
        _control: &str,
        _title: &str,
        _style: i64,
        _extended: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot write the style of a control".to_string())
    }

    /// The handles of the controls of a window given its handle: the
    /// `WinGetControlsHwnd` of a window of another program, whose controls
    /// the machine enumerates.
    fn control_hwnds_of(&mut self, _window: i64) -> Result<Vec<i64>, String> {
        Err("this desktop cannot list the controls of a window".to_string())
    }

    /// The class and number that name a control given its handle.
    fn control_classnn_of(&mut self, _control: i64) -> Result<String, String> {
        Err("this desktop cannot name a control by its handle".to_string())
    }

    /// The style of a control given its handle, or its extended style.
    fn control_style_of_hwnd(&mut self, _control: i64, _extended: bool) -> Result<i64, String> {
        Err("this desktop cannot read the style of a control".to_string())
    }

    /// Write the style of a control given its handle.
    fn control_set_style_of_hwnd(
        &mut self,
        _control: i64,
        _style: i64,
        _extended: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot write the style of a control".to_string())
    }

    /// The process of the window with this title.
    ///
    /// `None` is a window that is not there. Measured on the binary of v2.0.19 `WinGetPID` answers the number of the process, and a title that
    /// no window has is "Target window not found.".
    fn window_pid(&mut self, _title: &str) -> Result<Option<i64>, String> {
        Err("this desktop cannot say the process of a window".to_string())
    }

    /// The style of a window, or its extended style.
    ///
    /// The style of a window is the number of the bits of Windows: the
    /// `WS_*` of the machine, and the extended style is the `WS_EX_*` of
    /// it. The two are the same question with the same answer, and the
    /// flag tells which one is asked.
    ///
    /// Measured on the binary of v2.0.31: the style of the
    /// window of the script is `80674816` (`0x04CF0000`, with the
    /// `WS_CLIPSIBLINGS` the machine adds) and its extended style is `256`
    /// (`WS_EX_WINDOWEDGE`).
    ///
    /// A window that is not there is a failure, and not a zero: the
    /// question was about a window, and there is none.
    fn window_style(&mut self, _title: &str, _extended: bool) -> Result<i64, String> {
        Err("this desktop cannot read the style of a window".to_string())
    }

    /// Write the style of a window, or its extended style.
    ///
    /// The style of a window is the number of the bits of Windows, and the
    /// machine is the one that keeps them: the frame of the window is
    /// drawn again by the machine, and not by the script.
    fn window_set_style(
        &mut self,
        _title: &str,
        _style: i64,
        _extended: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot write the style of a window".to_string())
    }

    /// The colour a window lets through, or the empty text.
    ///
    /// The colour is the one of a **layered** window: the window that
    /// lets the pixels of the screen through, and the colour that is not
    /// drawn. A window that is not layered has no such colour, and the
    /// answer is the empty text -- measured on the binary of v2.0.31 where the window of the script answered the empty text.
    fn window_trans_color(&mut self, _title: &str) -> Result<String, String> {
        Err("this desktop cannot read the transparent colour of a window".to_string())
    }

    /// Write the colour a window lets through.
    ///
    /// The window becomes a layered one, and the machine draws it again.
    fn window_set_trans_color(&mut self, _title: &str, _color: &str) -> Result<(), String> {
        Err("this desktop cannot write the transparent colour of a window".to_string())
    }

    /// The handle of the window of the script itself.
    ///
    /// The window of the script is the hidden one that owns the messages
    /// of the session: the tray of the script and the hotkeys of it are
    /// its own. The number is the handle of the machine, and the desktop
    /// that has no window of its own says so.
    fn script_window(&mut self) -> Result<i64, String> {
        Err("this desktop has no window of the script".to_string())
    }

    /// Whether this desktop makes the windows of the script.
    ///
    /// The `Gui` of the language asks this before it delegates: a desktop
    /// that makes windows carries the family, and one that does not keeps
    /// the reason of the family. The Windows backend makes them; the box
    /// of a Wayland session does not.
    fn carries_the_gui(&mut self) -> bool {
        false
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
    /// binary of v2.0.19, `Gui()` makes it and `Show()` puts it on
    /// the screen, and a script may add to it in between.
    fn gui_open(&mut self, _title: &str) -> Result<u64, String> {
        Err("this desktop cannot make a window of the script".to_string())
    }

    /// Add a control to a window of the script.
    ///
    /// The answer is the number that names the control and its `ClassNN`, which
    /// is the file name of the kind and the count of that kind -- `Edit1` for the
    /// first edit, whatever else was added before it. Measured on the binary of
    /// v2.0.19.
    fn gui_add(&mut self, _handle: u64, _kind: &str, _text: &str) -> Result<(u64, String), String> {
        Err("this desktop cannot add a control to a window of the script".to_string())
    }

    /// The items of a list-shaped control of the script's window, one per line.
    /// Measured on the binary of v2.0.19: `ControlGetItems`
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
    fn gui_combo_select(
        &mut self,
        _number: u64,
        _wanted: &str,
        _by_text: bool,
    ) -> Result<i64, String> {
        Err("this desktop cannot choose an item of a list-shaped control".to_string())
    }

    /// The chosen item as (index, text). Measured on the same day: nothing
    /// chosen is (0, ""), and `ControlGetChoice` reads the text alone.
    fn gui_combo_choice(&mut self, _number: u64) -> Result<(i64, String), String> {
        Err("this desktop cannot read the choice of a list-shaped control".to_string())
    }

    /// Remove an item by its 1-based index. Measured on the binary of v2.0.19
    ///: `ControlDeleteItem(1, ...)` leaves the array one shorter.
    fn gui_combo_delete(&mut self, _number: u64, _index: &str) -> Result<(), String> {
        Err("this desktop cannot remove an item of a list-shaped control".to_string())
    }

    /// The 1-based index of the first item that is this text, or zero.
    fn gui_combo_find(&mut self, _number: u64, _text: &str) -> Result<i64, String> {
        Err("this desktop cannot find an item of a list-shaped control".to_string())
    }

    /// Open or close the pop-up of a list-shaped control. Measured on the
    /// binary of v2.0.19: `ControlShowDropDown` and
    /// `ControlHideDropDown` both answer the empty text.
    fn gui_combo_dropdown(&mut self, _number: u64, _show: bool) -> Result<(), String> {
        Err("this desktop cannot open the pop-up of a list-shaped control".to_string())
    }

    /// The text of the status bar of the script's window. Measured on the
    /// binary of v2.0.19: `StatusBarGetText(1, WinTitle)`
    /// answers it.
    fn gui_status_text(&mut self) -> Result<String, String> {
        Err("this desktop cannot read the text of a status bar".to_string())
    }

    // -- the list view ------------------------------------------------------------
    //
    // Measured on the binary of v2.0.19: a ListView is
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
    fn gui_list_modify(
        &mut self,
        _number: u64,
        _row: i64,
        _cells: &[String],
    ) -> Result<i64, String> {
        Err("this desktop cannot change a row of a list view".to_string())
    }

    /// The row after this one, or zero when there is none.
    fn gui_list_next(&mut self, _number: u64, _start: i64) -> Result<i64, String> {
        Err("this desktop cannot read the next row of a list view".to_string())
    }

    // -- the tree view ------------------------------------------------------------
    //
    // Measured on the binary of v2.0.19: a TreeView is
    // `SysTreeView321`, `tv.Add(text, parent)` answers an item id, and the
    // Get* family walks the tree by those ids. This runtime numbers its items
    // sequentially from one, which is an identity a script can hold and compare.

    /// Add an item under a parent (0 = a root); answers its id.
    fn gui_tree_add(
        &mut self,
        _number: u64,
        _text: &str,
        _parent: i64,
        _options: &str,
    ) -> Result<i64, String> {
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
    fn gui_tree_relation(
        &mut self,
        _number: u64,
        _item: i64,
        _relation: &str,
    ) -> Result<i64, String> {
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

    // -- the windows of the v1 ----------------------------------------------------
    //
    // The v1 names its windows: the `Gui, Name: Add` puts a control in the
    // window that the name names, and the `GuiControl, Name:` names the
    // window of the control too. The window of the script that has no name
    // is the one of the number one.

    /// The windows of the v1 that the script made, by their name.
    fn v1_gui_windows(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot name the windows of the script".to_string())
    }

    /// The window of the v1 that a name names, made when it is not there.
    ///
    /// The answer is the name of the window that is there, and the name
    /// that the script gave is the one that is answered.
    fn v1_gui_open(&mut self, _name: &str, _title: &str) -> Result<String, String> {
        Err("this desktop cannot make a window of the script".to_string())
    }

    /// The window of the v1 that a name names, when it is there.
    fn v1_gui_is_open(&mut self, _name: &str) -> Result<bool, String> {
        Err("this desktop cannot find the window of the script".to_string())
    }

    /// The window of the v1 that a name names, and the number of it.
    ///
    /// The number of a window of the v1 is the one that the script uses to
    /// name it when it has no name of its own.
    fn v1_gui_number_of(&mut self, _name: &str) -> Result<i64, String> {
        Err("this desktop cannot number the windows of the script".to_string())
    }

    /// A control is added to the window of the v1 that a name names.
    ///
    /// The answer is the number of the control and the class of it, the way
    /// the v2 answers it.
    ///
    /// The style is the one that the options of the v1 ask for: the bits
    /// that they add and the bits that they take away.
    fn v1_gui_add(
        &mut self,
        _window: &str,
        _kind: &str,
        _variable: &str,
        _text: &str,
        _style: TheStyleOfTheV1,
    ) -> Result<(u64, String), String> {
        Err("this desktop cannot add a control to a window of the script".to_string())
    }

    /// The caption of a control of the window of the v1: the text that
    /// the machine shows in it.
    fn v1_gui_caption(&mut self, _window: &str, _number: u64) -> Result<String, String> {
        Err("this desktop cannot read the caption of a control of a window of the script".to_string())
    }

    /// The place and the size of a control of the window of the v1.
    fn v1_gui_place(
        &mut self,
        _window: &str,
        _number: u64,
        _place: ThePlaceOfTheV1,
    ) -> Result<(), String> {
        Err("this desktop cannot place a control of a window of the script".to_string())
    }

    /// The place and the size of the window of the v1 that a name names.
    ///
    /// The centering is asked for with the "Center" and the "xCenter" and
    /// the "yCenter" of the `Show`.
    ///
    /// WHY the allowance: the shape of the `Show` of the v1 is the shape of
    /// the call, and the call says the place, the size and the centering of
    /// the window one by one. The same reason is in the crate of the box.
    #[allow(clippy::too_many_arguments)]
    fn v1_gui_window_place(
        &mut self,
        _window: &str,
        _x: Option<i64>,
        _y: Option<i64>,
        _width: Option<i64>,
        _height: Option<i64>,
        _centered_x: bool,
        _centered_y: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot place a window of the script".to_string())
    }

    /// The number of the control of the window of the v1 that a name names.
    ///
    /// The name of a control of the v1 is the name of the variable that it
    /// is bound to, or the `ClassNN` of it.
    fn v1_gui_number_named(&mut self, _window: &str, _name: &str) -> Result<i64, String> {
        Err("this desktop cannot find a control of a window of the script".to_string())
    }

    /// The handle of the machine of a control of the window of the v1.
    fn v1_gui_handle(&mut self, _window: &str, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot find the handle of a control of a window".to_string())
    }

    /// The handle of the machine of the window of the v1.
    fn v1_gui_window_handle(&mut self, _window: &str) -> Result<i64, String> {
        Err("this desktop cannot find the handle of a window of the script".to_string())
    }

    /// The text of a control of the window of the v1 is read or written.
    fn v1_gui_value(
        &mut self,
        _window: &str,
        _number: u64,
        _setting: Option<&str>,
    ) -> Result<String, String> {
        Err("this desktop cannot read the text of a control of a window".to_string())
    }

    /// The state of a control of the window of the v1: the "enabled" of it
    /// and the "visible" of it.
    fn v1_gui_flag(
        &mut self,
        _window: &str,
        _number: u64,
        _what: &str,
        _setting: Option<bool>,
    ) -> Result<i64, String> {
        Err("this desktop cannot read the state of a control of a window".to_string())
    }

    /// A control of the window of the v1 is shown or hidden.
    fn v1_gui_visible(&mut self, _window: &str, _number: u64, _on: bool) -> Result<(), String> {
        Err("this desktop cannot show a control of a window of the script".to_string())
    }

    /// A control of the window of the v1 has the focus.
    fn v1_gui_focus(&mut self, _window: &str, _number: u64) -> Result<(), String> {
        Err("this desktop cannot give the focus to a control of a window".to_string())
    }

    /// The control of the window of the v1 that has the focus.
    fn v1_gui_focused(&mut self, _window: &str) -> Result<String, String> {
        Err("this desktop cannot find the control of a window that has the focus".to_string())
    }

    /// The place and the size of a control of the window of the v1.
    fn v1_gui_control_pos(
        &mut self,
        _window: &str,
        _number: u64,
    ) -> Result<(i64, i64, i64, i64), String> {
        Err("this desktop cannot place a control of a window of the script".to_string())
    }

    /// The window of the v1 that a name names is shown.
    fn v1_gui_show(&mut self, _window: &str, _title: &str) -> Result<(), String> {
        Err("this desktop cannot show a window of the script".to_string())
    }

    /// The window of the v1 that a name names is shown without being
    /// activated.
    ///
    /// The "NoActivate" and the "NA" of the `Show` of the v1 show the
    /// window without making it the one of the thread: measured on the
    /// binary of v1.1.37.02, the `Gui, PVother: Show,
    /// NoActivate AutoSize, The other` and the `Gui, PVthird: Show, NA`
    /// are both accepted.
    fn v1_gui_show_without_activating(
        &mut self,
        _window: &str,
        _title: &str,
    ) -> Result<(), String> {
        Err("this desktop cannot show a window of the script without waking it".to_string())
    }

    /// The window of the v1 that a name names is made as big as the
    /// controls that it holds.
    ///
    /// The "AutoSize" of the `Show` of the v1 resizes the window to
    /// accommodate the controls that are visible in it. Measured on the
    /// binary of v1.1.37.02: the option is accepted.
    fn v1_gui_auto_size(&mut self, _window: &str) -> Result<(), String> {
        Err("this desktop cannot size a window of the script to its controls".to_string())
    }

    /// The window of the v1 that a name names is hidden.
    fn v1_gui_hide(&mut self, _window: &str) -> Result<(), String> {
        Err("this desktop cannot hide a window of the script".to_string())
    }

    /// The window of the v1 that a name names is closed.
    fn v1_gui_destroy(&mut self, _window: &str) -> Result<(), String> {
        Err("this desktop cannot close a window of the script".to_string())
    }

    /// The title of the window of the v1 that a name names is read or written.
    fn v1_gui_title(&mut self, _window: &str, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop cannot name the window of the script".to_string())
    }

    /// The option of the window of the v1 that a name names is applied.
    ///
    /// The `value` is the one of the options that take a value: the
    /// "minsize", the "maxsize" and the "delimiter". The options that
    /// take no value have it empty.
    fn v1_gui_option(
        &mut self,
        _window: &str,
        _name: &str,
        _value: Option<&str>,
        _on: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot apply an option to a window of the script".to_string())
    }

    /// The style without a name of the window of the v1 is applied.
    ///
    /// The style is the one that the script wrote with a sign and a
    /// number: `+0x...` for the ordinary style and `+E0x...` for the
    /// extended one.
    fn v1_gui_style(
        &mut self,
        _window: &str,
        _extended: bool,
        _style: u32,
        _on: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot apply a style to a window of the script".to_string())
    }

    /// The window of the v1 that a name names is owned by another window.
    ///
    /// The `owner` is the name of the other window, or the handle of it,
    /// and the `parent` says that the window becomes a child of it. The
    /// `owner` of `None` takes the owner away from the window.
    fn v1_gui_owner(
        &mut self,
        _window: &str,
        _owner: Option<&str>,
        _parent: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot give an owner to a window of the script".to_string())
    }

    /// The class of a control of the window of the v1: the "ClassNN" of it.
    fn v1_gui_classnn(&mut self, _window: &str, _number: u64) -> Result<String, String> {
        Err("this desktop cannot name a control of a window of the script".to_string())
    }

    /// The name of the variable of a control of the window of the v1.
    ///
    /// The name is the one that the `v` of the options of the `Gui, Add`
    /// gave to the control, and it is empty when the control has none.
    fn v1_gui_control_name(&mut self, _window: &str, _number: u64) -> Result<String, String> {
        Err("this desktop cannot name the variable of a control of a window".to_string())
    }

    /// An item of a control of the window of the v1 is chosen.
    ///
    /// The item is the number of it, or its text when the `by_text` says
    /// so. The answer is the number of the item that was chosen.
    fn v1_gui_choose(
        &mut self,
        _window: &str,
        _number: u64,
        _value: &str,
        _by_text: bool,
    ) -> Result<i64, String> {
        Err("this desktop cannot choose an item of a control of a window".to_string())
    }

    /// The tab of the window of the v1 is chosen.
    fn v1_gui_tab(&mut self, _window: &str, _tab: i64) -> Result<(), String> {
        Err("this desktop cannot choose a tab of a window of the script".to_string())
    }

    /// The name of the window of the v1 that a handle names.
    ///
    /// The v1 names a window by its handle as well as by its name, and
    /// the name of the window with no name is empty.
    fn v1_gui_name_of_handle(&mut self, _handle: i64) -> Result<String, String> {
        Err("this desktop cannot find the window of a handle".to_string())
    }

    /// The background of the window of the v1 that a name names is written.
    fn v1_gui_color(&mut self, _window: &str, _color: &str) -> Result<(), String> {
        Err("this desktop cannot write the colour of a window of the script".to_string())
    }

    /// The margin of the window of the v1 that a name names is written.
    fn v1_gui_margin(&mut self, _window: &str, _x: i64, _y: i64) -> Result<(), String> {
        Err("this desktop cannot write the margin of a window of the script".to_string())
    }

    /// The font of the window of the v1 that a name names is written.
    fn v1_gui_font(
        &mut self,
        _window: &str,
        _options: &str,
        _name: &str,
    ) -> Result<(), String> {
        Err("this desktop cannot write the font of a window of the script".to_string())
    }

    /// The bar of the menu of the window of the v1 that a name names is written.
    ///
    /// The `menu` is the number of the menu of the machine, and it is
    /// zero when the machine cannot make a menu: the bar is then built
    /// from the `json`, the items of the menu of the script.
    fn v1_gui_menu_bar(
        &mut self,
        _window: &str,
        _name: &str,
        _json: &str,
        _menu: u64,
    ) -> Result<(), String> {
        Err("this desktop cannot write the menu of a window of the script".to_string())
    }

    /// The bar of the menu of the windows that use the menu of the script
    /// that a name names is written again: the menu of the script has
    /// changed, and the bar of the window shows it.
    fn v1_gui_menu_bar_is_updated(
        &mut self,
        _name: &str,
        _json: &str,
        _menu: u64,
    ) -> Result<(), String> {
        Ok(())
    }

    /// The contents of the controls of the window of the v1 that a name names
    /// are read, and the answer is the text of every control that has a
    /// variable of its own.
    fn v1_gui_submit(&mut self, _window: &str) -> Result<Vec<(String, String)>, String> {
        Err("this desktop cannot read the controls of a window of the script".to_string())
    }

    // -- the list view and the tree view of the v1 --------------------------------
    //
    // The `LV_*` and the `TV_*` of the v1: the rows and the columns of the
    // list view, the items of the tree view, and the lists of images.

    /// The window of the v1 that the `LV_*` and the `TV_*` write to.
    ///
    /// The v1 names the list view and the tree view with the `Gui, Name:
    /// ListView` and the `Gui, Name: TreeView`, and the commands that
    /// write to them work on the window that carries them.
    fn v1_gui_use_window(&mut self, _window: &str) -> Result<(), String> {
        Ok(())
    }

    /// `LV_Add`: a row at the end; the answer is its number, from one.
    fn v1_lv_add(&mut self, _number: u64, _options: &str, _cells: &[String]) -> Result<i64, String> {
        Err("this desktop cannot add a row to a list view".to_string())
    }

    /// `LV_Delete`: a row, or them all when the row is zero.
    fn v1_lv_delete(&mut self, _number: u64, _row: i64) -> Result<bool, String> {
        Err("this desktop cannot delete a row of a list view".to_string())
    }

    /// `LV_GetCount`: the rows, the chosen ones or the columns.
    fn v1_lv_count(&mut self, _number: u64, _mode: &str) -> Result<i64, String> {
        Err("this desktop cannot count the rows of a list view".to_string())
    }

    /// `LV_GetNext`: the next row that is chosen, checked or focused.
    fn v1_lv_next(&mut self, _number: u64, _start: i64, _mode: &str) -> Result<i64, String> {
        Err("this desktop cannot read the next row of a list view".to_string())
    }

    /// `LV_GetText`: the text of one cell.
    fn v1_lv_text(&mut self, _number: u64, _row: i64, _col: i64) -> Result<String, String> {
        Err("this desktop cannot read a cell of a list view".to_string())
    }

    /// `LV_Modify`: the options and the texts of a row, or of them all.
    fn v1_lv_modify(
        &mut self,
        _number: u64,
        _row: i64,
        _options: &str,
        _cells: &[String],
    ) -> Result<bool, String> {
        Err("this desktop cannot change a row of a list view".to_string())
    }

    /// `LV_ModifyCol`: a column, made when it is not there.
    fn v1_lv_modifycol(
        &mut self,
        _number: u64,
        _col: i64,
        _options: &str,
        _title: Option<&str>,
    ) -> Result<bool, String> {
        Err("this desktop cannot change a column of a list view".to_string())
    }

    /// `LV_SetImageList`: the list of images of the control.
    fn v1_lv_setimagelist(&mut self, _number: u64, _list: i64, _large: bool) -> Result<i64, String> {
        Err("this desktop cannot set the image list of a list view".to_string())
    }

    /// The first row that is seen.
    fn v1_lv_firstvisible(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot read the first visible row of a list view".to_string())
    }

    /// `IL_Create`: a list of images; the answer is its handle.
    fn v1_il_create(&mut self, _initial: i32, _grow: i32, _large: bool) -> Result<i64, String> {
        Err("this desktop cannot make a list of images".to_string())
    }

    /// `IL_Add`: an image in a list; the answer is its place, from one.
    fn v1_il_add(&mut self, _list: i64, _source: &str, _icon_number: i64) -> Result<i64, String> {
        Err("this desktop cannot add an image to a list".to_string())
    }

    /// `IL_Destroy`: a list of images given back.
    fn v1_il_destroy(&mut self, _list: i64) -> Result<bool, String> {
        Err("this desktop cannot destroy a list of images".to_string())
    }

    /// `TV_Add`: an item under a parent; the answer is its handle.
    fn v1_tv_add(
        &mut self,
        _number: u64,
        _text: &str,
        _parent: i64,
        _options: &str,
    ) -> Result<i64, String> {
        Err("this desktop cannot add an item to a tree view".to_string())
    }

    /// `TV_Delete`: an item, or them all when the item is zero.
    fn v1_tv_delete(&mut self, _number: u64, _item: i64) -> Result<bool, String> {
        Err("this desktop cannot delete an item of a tree view".to_string())
    }

    /// `TV_GetCount`: the number of items.
    fn v1_tv_count(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot count the items of a tree view".to_string())
    }

    /// `TV_GetText`: the text of an item.
    fn v1_tv_text(&mut self, _number: u64, _item: i64) -> Result<String, String> {
        Err("this desktop cannot read the text of an item of a tree view".to_string())
    }

    /// `TV_Get`: the item is expanded, checked or bold.
    fn v1_tv_get(&mut self, _number: u64, _item: i64, _attribute: &str) -> Result<bool, String> {
        Err("this desktop cannot read the state of an item of a tree view".to_string())
    }

    /// `TV_GetChild`: the first child of a parent.
    fn v1_tv_child(&mut self, _number: u64, _parent: i64) -> Result<i64, String> {
        Err("this desktop cannot walk a tree view".to_string())
    }

    /// `TV_GetParent`: the parent of an item.
    fn v1_tv_parent(&mut self, _number: u64, _item: i64) -> Result<i64, String> {
        Err("this desktop cannot walk a tree view".to_string())
    }

    /// `TV_GetNext`: the next item, checked or not.
    fn v1_tv_next(&mut self, _number: u64, _item: i64, _mode: &str) -> Result<i64, String> {
        Err("this desktop cannot walk a tree view".to_string())
    }

    /// `TV_GetSelection`: the item that is chosen.
    fn v1_tv_selection(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop cannot read the selection of a tree view".to_string())
    }

    /// `TV_Modify`: the options and the text of an item.
    fn v1_tv_modify(
        &mut self,
        _number: u64,
        _item: i64,
        _options: &str,
        _text: Option<&str>,
    ) -> Result<bool, String> {
        Err("this desktop cannot change an item of a tree view".to_string())
    }

    /// A second, independent text slot on a control.
    ///
    /// Measured on the binary of v2.0.19: on an Edit or a Text
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
    /// Measured on the binary of v2.0.19: `ControlGetPos` with
    /// four VarRef outputs answers the four numbers. The reference does NOT
    /// hang on this name -- the earlier note was a call with plain strings.
    fn gui_control_pos(&mut self, _number: u64) -> Result<(i64, i64, i64, i64), String> {
        Err("this desktop cannot say where a control of the window is".to_string())
    }

    /// Show a menu bar on the window of the script.
    ///
    /// The JSON is an array of `[name, checked, enabled, callback]` entries,
    /// and the callback is left out when the item has none. Measured on the
    /// binary of v2.0.19: a `MenuBar` attached with
    /// `Gui.Menu :=` shows at the top of the window.
    fn gui_menu_bar(&mut self, _json: &str) -> Result<(), String> {
        Err("this desktop cannot show a menu bar on the window of the script".to_string())
    }

    /// Make a menu of the script, and answer the number that names it.
    fn menu_create(&mut self) -> Result<u64, String> {
        Err("this desktop cannot make a menu".to_string())
    }

    /// Add an item to a menu. The callback is the name of the function
    /// the item runs when it is chosen.
    fn menu_add(&mut self, _menu: u64, _name: &str, _callback: &str) -> Result<(), String> {
        Err("this desktop cannot add an item to a menu".to_string())
    }

    /// Add an item to a menu, or update the item of that name when the
    /// menu has one.
    ///
    /// The v1: "If MenuItemName does not yet exist, it will be added to
    /// the menu. Otherwise, MenuItemName is updated with the newly
    /// specified LabelOrSubmenu" (the CHM of the v1.1.37.02,
    /// `Menu.htm`).
    ///
    /// Measured on the binary of the v1.1.37.02: the
    /// `Menu, PVbar, Add, &File, :PVfile` of a script of the v1 is
    /// written twice -- once when the bar is built and once when the
    /// attachments are repaired -- and the bar of the window has five
    /// items, and not ten.
    fn menu_update(&mut self, _menu: u64, _name: &str, _callback: &str) -> Result<(), String> {
        Err("this desktop cannot add an item to a menu".to_string())
    }

    /// The handle of the machine of a menu.
    fn menu_handle(&mut self, _menu: u64) -> Result<i64, String> {
        Err("this desktop cannot give the handle of a menu".to_string())
    }

    /// Delete the item of a name from a menu.
    fn menu_delete(&mut self, _menu: u64, _name: &str) -> Result<(), String> {
        Err("this desktop cannot delete an item of a menu".to_string())
    }

    /// Delete the menu of the machine itself, and the numbers of its
    /// items.
    ///
    /// The v1: "If MenuItemName is omitted, the entire MenuName menu
    /// will be deleted as will any menu items in other menus that use
    /// MenuName as a submenu" (the CHM of the v1.1.37.02, `Menu.htm`).
    ///
    /// The numbers of the items of the machine are unique and they stay
    /// under 65536: the number of a command is the low word of it. A
    /// menu that the script deletes gives its numbers back, or the
    /// numbers of the machine grow for ever.
    fn menu_delete_the_menu(&mut self, _menu: u64) -> Result<(), String> {
        Err("this desktop cannot delete a menu".to_string())
    }

    /// Delete every item of a menu.
    ///
    /// The v1: "Deletes all custom menu items from the menu. ... Unlike
    /// a menu entirely deleted by the Delete sub-command, an empty menu
    /// still exists and thus any other menus that use it as a submenu
    /// will still work" (the CHM of the v1.1.37.02, `Menu.htm`).
    fn menu_delete_all(&mut self, _menu: u64) -> Result<(), String> {
        Err("this desktop cannot empty a menu".to_string())
    }

    /// Insert an item in a menu, above the last one.
    ///
    /// The v1: the `Insert` puts the item before the item that is named,
    /// and the item that is named is the last one when the script does
    /// not name it.
    fn menu_insert(&mut self, _menu: u64, _name: &str, _callback: &str) -> Result<(), String> {
        Err("this desktop cannot insert an item into a menu".to_string())
    }

    /// The link between an item of a menu and the menu that it opens.
    ///
    /// The v1: "To have MenuItemName become a submenu -- which is a menu
    /// item that opens a new menu when selected -- specify for
    /// LabelOrSubmenu a colon followed by the MenuName of an existing
    /// custom menu" (the CHM of the v1.1.37.02, `Menu.htm`).
    fn menu_set_submenu(
        &mut self,
        _menu: u64,
        _item: &str,
        _submenu: u64,
    ) -> Result<(), String> {
        Err("this desktop cannot attach a menu to an item of a menu".to_string())
    }

    /// The icon of an item of a menu.
    ///
    /// The v1: "Menu, MenuName, Icon, MenuItemName, FileName, IconNumber,
    /// IconWidth" (the CHM of the v1.1.37.02, `Menu.htm`).
    fn menu_set_icon(
        &mut self,
        _menu: u64,
        _item: &str,
        _icon: &str,
        _number: i64,
    ) -> Result<(), String> {
        Err("this desktop cannot set the icon of an item of a menu".to_string())
    }

    /// Rename the item of a name in a menu.
    fn menu_rename(&mut self, _menu: u64, _name: &str, _new: &str) -> Result<(), String> {
        Err("this desktop cannot rename an item of a menu".to_string())
    }

    /// Enable or disable the item of a name in a menu.
    fn menu_enable(&mut self, _menu: u64, _name: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot enable an item of a menu".to_string())
    }

    /// Check or uncheck the item of a name in a menu.
    fn menu_check(&mut self, _menu: u64, _name: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot check an item of a menu".to_string())
    }

    /// The number of items of a menu.
    fn menu_count(&mut self, _menu: u64) -> Result<i64, String> {
        Err("this desktop cannot count the items of a menu".to_string())
    }

    /// Whether the item of a name is enabled.
    fn menu_is_enabled(&mut self, _menu: u64, _name: &str) -> Result<bool, String> {
        Err("this desktop cannot say whether an item of a menu is enabled".to_string())
    }

    /// Whether the item of a name is checked.
    fn menu_is_checked(&mut self, _menu: u64, _name: &str) -> Result<bool, String> {
        Err("this desktop cannot say whether an item of a menu is checked".to_string())
    }

    /// Show a menu as a popup, and answer the callback of the item chosen.
    fn menu_show(&mut self, _menu: u64) -> Result<Option<String>, String> {
        Err("this desktop cannot show a menu".to_string())
    }

    /// Show a menu as a popup at a place, and answer the callback of the
    /// item that was chosen.
    ///
    /// The v1: "Menu, MenuName, Show, X, Y -- Displays the specified
    /// menu. ... If both X and Y are omitted, the menu is displayed at
    /// the current position of the mouse cursor" (the CHM of the
    /// v1.1.37.02, `Menu.htm`).
    fn menu_show_at(&mut self, _menu: u64, _x: i32, _y: i32) -> Result<Option<String>, String> {
        Err("this desktop cannot show a menu".to_string())
    }

    /// Attach a menu to the window of the script as its menu bar.
    fn menu_attach(&mut self, _menu: u64) -> Result<(), String> {
        Err("this desktop cannot attach a menu to the window of the script".to_string())
    }

    /// Where the caret of the text is, in the client coordinates of the
    /// window of the script.
    ///
    /// `None` is no caret: no window, or nothing focused that holds one.
    /// Measured on the binary of v2.0.19: with no window
    /// `CaretGetPos` answers 0 and the variables of the caller are left
    /// untouched; with a focused Edit it answers 1 and the place of the
    /// caret in the client area.
    fn gui_caret(&mut self) -> Result<Option<(i64, i64)>, String> {
        Err("this desktop cannot find the caret of the window".to_string())
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
    /// control. Measured on the binary of v2.0.19: `Value` of a
    /// control is read and written as text.
    fn gui_value(&mut self, _number: u64, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop cannot read the text of a control of the script".to_string())
    }

    /// Read a flag of a control of a window of the script, or write it.
    ///
    /// The flag is `"enabled"` or `"visible"`. Measured on the binary of v2.0.19
    ///: both are integers of nought and one, read and written.
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

    /// The name of a control of the script's window, the way the v2 names it.
    ///
    /// The name is the kind of the control and the number of it among the
    /// ones of that kind: "Edit1" is the first edit of the window. It is
    /// the way back of `gui_number_named`.
    fn gui_classnn(&mut self, _number: u64) -> Result<String, String> {
        Err("this desktop cannot name a control of the script".to_string())
    }

    /// The handle of the machine of a control of the script's window.
    ///
    /// The number that names a control of the tree is not the handle of the
    /// machine: a script that asks for the handle wants the number that the
    /// machine of it knows.
    fn gui_handle(&mut self, _number: u64) -> Result<i64, String> {
        Err("this desktop has no handle for a control of the script".to_string())
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
    /// Measured on the binary of v2.0.19: `EditGetLineCount` of a
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
    /// Measured on the binary of v2.0.19: `Hide` answers the empty
    /// text and takes no parameters.
    fn gui_hide(&mut self) -> Result<(), String> {
        Err("this desktop cannot hide the window of the script".to_string())
    }

    /// Put the focus on a control of a window of the script.
    ///
    /// Measured on the binary of v2.0.19: `Focus` answers the empty
    /// text and takes no parameters.
    fn gui_focus(&mut self, _number: u64) -> Result<(), String> {
        Err("this desktop cannot put the focus on a control of the script".to_string())
    }

    /// Set the size of a control of a window of the script.
    ///
    /// The **size** and not the place: measured, the window this
    /// tree makes lays its controls out in order, and GTK gives a child of a
    /// layout no place of its own.
    fn gui_size(&mut self, _number: u64, _width: i64, _height: i64) -> Result<(), String> {
        Err("this desktop cannot size a control of the script".to_string())
    }

    /// Read the events of the windows of the script that have happened.
    ///
    /// The answer is the events **since the last time it was asked**, one name
    /// each, and an empty answer is the common case and must cost almost
    /// nothing. Measured on the binary of v2.0.19: the events a
    /// window of a script has are `close`, `escape`, and a click on a control.
    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    /// Watch a message of the window of the script.
    ///
    /// The `OnMessage` registers a handler for the number, and the
    /// desktop queues the message when it arrives at the window.
    fn watch_a_message(&mut self, _number: i64) -> Result<(), String> {
        Err("this desktop cannot watch a message of the window of the script".to_string())
    }

    /// Post a message to a window of the machine.
    fn post_a_message(
        &mut self,
        _window: i64,
        _message: i64,
        _wparam: i64,
        _lparam: i64,
    ) -> Result<(), String> {
        Err("this desktop cannot post a message to a window".to_string())
    }

    /// Send a message to a window of the machine, and answer what it said.
    fn send_a_message(
        &mut self,
        _window: i64,
        _message: i64,
        _wparam: i64,
        _lparam: i64,
    ) -> Result<i64, String> {
        Err("this desktop cannot send a message to a window".to_string())
    }

    /// Read the title of a window of the script, or write it.
    fn gui_title(&mut self, _handle: u64, _setting: Option<&str>) -> Result<String, String> {
        Err("this desktop cannot say the title of a window of the script".to_string())
    }

    /// The place and the size of the window of the script.
    ///
    /// Read in the CHM of the v1.1.37.02 (`Gui.htm`): the options of the
    /// `Gui, Show` are `w`, `h`, `x` and `y`, and the ones that are not
    /// there keep the place they had.
    fn gui_place(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _width: Option<i64>,
        _height: Option<i64>,
    ) -> Result<(), String> {
        Err("this desktop cannot place the window of the script".to_string())
    }

    /// The capabilities of the windows of this desktop.
    ///
    /// The names are the ones of `WinGetCapability`: `AlwaysOnTopGet`,
    /// `AlwaysOnTopSet`, `WindowEnabledGet`, `ControlEnabledGet`,
    /// `WindowList`, `WindowActivate`, `WindowMinimize` and `WindowClose`.
    /// Only a capability the machine can really answer is here.
    fn window_capabilities(&mut self) -> Vec<String> {
        Vec::new()
    }

    /// Whether a window is enabled, when the machine knows.
    ///
    /// This is the state of the window itself. On a session of X11 it is
    /// the state of the window of the machine; on a native Wayland session
    /// it is mapped and not minimized, because the compositor publishes no
    /// more than that.
    fn window_enabled(&mut self, _window: &WindowInfo) -> Result<Option<bool>, String> {
        Err("this desktop cannot say whether a window is enabled".to_string())
    }

    /// Put a window above the others, or take it from there.
    ///
    /// The answer is a failure with the reason when the machine cannot do
    /// it: a change that did not happen is never reported as a change that
    /// did.
    fn set_always_on_top(&mut self, _window: &WindowInfo, _on: bool) -> Result<(), String> {
        Err("this desktop cannot put a window above the others".to_string())
    }

    /// Enable a window of the machine, or disable it.
    ///
    /// The window of the v1 that is disabled stops answering to the mouse
    /// and to the keyboard, and the one that is enabled answers again.
    fn set_window_enabled(&mut self, _window: &WindowInfo, _on: bool) -> Result<(), String> {
        Err("this desktop cannot enable or disable a window".to_string())
    }

    /// Move a window of the machine, and change its size.
    ///
    /// The place and the size are the ones of the machine: the pixels.
    fn move_window(
        &mut self,
        _window: &WindowInfo,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
    ) -> Result<(), String> {
        Err("this desktop cannot move a window".to_string())
    }

    /// The transparency of a window of the machine: the alpha of it.
    ///
    /// The alpha is a number from 0 to 255, and the window becomes
    /// layered before the alpha is written.
    fn set_window_alpha(&mut self, _window: &WindowInfo, _alpha: i64) -> Result<(), String> {
        Err("this desktop cannot change the transparency of a window".to_string())
    }

    /// Bring a window of the machine to the front, and give it the focus.
    fn activate_window(&mut self, _window: &WindowInfo) -> Result<(), String> {
        Err("this desktop cannot bring a window to the front".to_string())
    }

    /// Give a window of the machine another title.
    ///
    /// The v1: the `WinSetTitle` renames the window that its criteria
    /// name, and it says nothing when the window is not there.
    fn set_window_title(&mut self, _window: &WindowInfo, _title: &str) -> Result<(), String> {
        Err("this desktop cannot give a window another title".to_string())
    }

    /// Give a region to a window of the machine: the shape that it takes
    /// on the screen.
    ///
    /// The region of the `None` is the region of the window itself: the
    /// window takes the shape that it has.
    fn set_window_region(
        &mut self,
        _window: &WindowInfo,
        _region: &WindowRegion,
    ) -> Result<(), String> {
        Err("this desktop cannot give a region to a window".to_string())
    }

    /// Whether a window is above the others, when the machine knows.
    ///
    /// `Ok(None)` is the answer of a machine that cannot say: it is not
    /// `false`, and the caller refuses with the reason. A question that
    /// cannot be answered does not answer `false` in this tree.
    fn always_on_top(&mut self, _window: &WindowInfo) -> Result<Option<bool>, String> {
        Err("this desktop cannot say whether a window is above the others".to_string())
    }

    /// The windows of the session, in the order the desktop lists them.
    ///
    /// The id of a window is the identifier of this runtime, and not a handle
    /// of Windows: a script hands it back to this runtime, and a number that
    /// no window has is a window that is not there -- it is not a failure.
    /// The class of a window is the name the desktop gives it: the
    /// application id on Wayland, and the class of the window on X11.
    ///
    /// Measured on the binary of v2.0.19: `WinGetList` answers
    /// the ids of the windows as an array, `WinGetCount` their number, and
    /// the class of a window is the class name the system gives it.
    fn windows(&mut self) -> Result<Vec<WindowInfo>, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// The window of a handle, even when it is a child of another window.
    ///
    /// The listing of the windows has only the windows of the top, and the
    /// criterion `ahk_id` names a window by the handle of it: the v1 (source
    /// `window.cpp:586-606` of v1.1.37.02) takes the handle as it is, and a
    /// window that is a child of another one is taken too (`window.cpp:626`,
    /// "Child windows (via ahk_id) are always detectible").
    ///
    /// The answer is `None` when the handle is not a window of the machine.
    fn window_by_id(&mut self, _id: i64) -> Option<WindowInfo> {
        None
    }

    /// The window at a point of the screen.
    ///
    /// The answer is `None` when no window is at the point, and a failure
    /// with the reason when the machine cannot say: zero means "no
    /// window", and a desktop that does not publish what is under a point
    /// does not answer zero for it.
    fn the_window_at(&mut self, _x: i64, _y: i64) -> Result<Option<i64>, String> {
        Err("this desktop cannot say which window is at a point".to_string())
    }

    /// The window that is under the pointer, and the control under it.
    ///
    /// The v1 asks the machine for the two of them: the `MouseGetPos` of
    /// the v1 answers the window of the top level that is under the
    /// pointer, and the control that is under it.  The answer is the
    /// pair of the two handles, and zero for the one that is not there.
    /// The mode is the one of the command: the bit 1 asks for the window
    /// of the machine alone, without the search of the control.
    ///
    /// Measured on the binary of v1.1.37.02: the `MouseGetPos, , , w` of
    /// the v1 answers the handle of the window of the top level that is
    /// under the pointer, and the tree answered the empty text: a script
    /// of the v1 compares that handle with the windows of it, and the
    /// empty text made it leave before it opened its menu.
    fn the_window_under_the_pointer(&mut self, _mode: i64) -> Result<(i64, i64), String> {
        // A desktop that does not publish what is under the pointer
        // answers that there is no window under it: the v1 answers the
        // empty text when there is no window.
        Ok((0, 0))
    }
}

/// The processes of the desktop.
pub trait Processes {
    /// Run a command; return its exit code, stdout and stderr.
    fn run(&mut self, command: &str, window: &str) -> Result<(i32, String, String), String>;

    /// Run a command and answer the pid of the process too.
    ///
    /// The `Run` with the variable that takes the pid uses this. A
    /// desktop that cannot say the pid answers zero.
    fn run_with_pid(
        &mut self,
        command: &str,
        window: &str,
    ) -> Result<(i32, String, String, u32), String> {
        let (code, stdout, stderr) = self.run(command, window)?;
        Ok((code, stdout, stderr, 0))
    }

    /// Launch a program without waiting for it; return the pid of it.
    ///
    /// Measured on the binary of v1.1.37.02: the `Run` of the
    /// v1 does not wait for the program, and it writes the pid of it into a
    /// variable of the script. The desktop that cannot launch a program says
    /// so, and not `false`.
    fn launch(&mut self, _command: &str, _working_dir: &str) -> Result<u32, String> {
        Err("this desktop cannot launch a program".to_string())
    }

    /// Launch a program and wait for it; return the pid of it, the code and
    /// what it wrote.
    ///
    /// Measured on the binary of v1.1.37.02: the `RunWait` of
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
    /// Measured on the binary of AutoHotkey v1.1.37.02: the
    /// warning of the environment is given when the variable is there, and the
    /// name that the environment does not have is a name that has no value.
    fn env_get(&mut self, _name: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot read the environment".to_string())
    }

    /// Set an environment variable; answer what it was set to.
    fn env_set(&mut self, _name: &str, _value: &str) -> Result<String, String> {
        Err("this desktop cannot write the environment".to_string())
    }

    // -- the processes -----------------------------------------------------------
    //
    // The process family of v2.0.19 asks about a process by the pid of it or
    // by the name of it, and `None` means the process of the script itself.
    //
    // Measured on the binary of v2.0.19: the name of a call is
    // the text of it, and a text that is a number is a pid -- a pid that is
    // not there answers zero, and it is not a failure. The methods below are
    // the ones of the capability: a desktop that has no processes says so
    // with the reason, and never answers zero for a question it cannot ask.
    //
    // The shape of each answer is the one of the binary of v2.0.19, measured
    // on the same day; the Linux reading behind them is in `ahk-box`, over
    // `/proc`.

    /// The pid of a process, and zero when there is no such process.
    fn process_exist(&mut self, _which: Option<&str>) -> Result<u32, String> {
        Err("this desktop cannot read the processes of the machine".to_string())
    }

    /// The name of the executable of a process.
    ///
    /// The name is the file name of the executable, and not the whole path
    /// of it. `None` is a process that is not there, which the caller says
    /// with the words of the dialect: "Target process not found.", of the
    /// class `TargetError`.
    fn process_name(&mut self, _which: Option<&str>) -> Result<Option<String>, String> {
        Err("this desktop cannot read the processes of the machine".to_string())
    }

    /// The full path of the executable of a process.
    ///
    /// `None` is a process that is not there, which the caller says with the
    /// words of the dialect.
    fn process_path(&mut self, _which: Option<&str>) -> Result<Option<String>, String> {
        Err("this desktop cannot read the processes of the machine".to_string())
    }

    /// The pid of the parent of a process, and zero when there is no such
    /// process.
    fn process_parent(&mut self, _which: Option<&str>) -> Result<u32, String> {
        Err("this desktop cannot read the processes of the machine".to_string())
    }

    /// Close a process: answer the pid that was closed, and zero when the
    /// process was not there or could not be closed.
    fn process_close(&mut self, _which: &str) -> Result<u32, String> {
        Err("this desktop cannot close a process of the machine".to_string())
    }

    /// Wait for a process to exist: answer the pid of it, and zero at the
    /// timeout.
    ///
    /// A timeout of `None` waits forever, which is what the omitted timeout
    /// of the binary does.
    fn process_wait(&mut self, _which: &str, _timeout: Option<f64>) -> Result<u32, String> {
        Err("this desktop cannot wait for a process of the machine".to_string())
    }

    /// Wait for a process that is there to close.
    ///
    /// Measured on the binary of v2.0.19: the answer is zero
    /// when the process is not there any more -- a process that never existed
    /// included -- and the pid of the process that is still there at the
    /// timeout.
    fn process_wait_close(&mut self, _which: &str, _timeout: Option<f64>) -> Result<u32, String> {
        Err("this desktop cannot wait for a process of the machine".to_string())
    }

    /// Set the priority of a process: answer the pid of it.
    ///
    /// The priority is the word of v2 -- "Low", "BelowNormal", "Normal",
    /// "AboveNormal", "High" or "Realtime", and the binary reads the first
    /// letter of it -- and `None` is a process that is not there, which the
    /// caller says with the words of the dialect.
    fn process_set_priority(
        &mut self,
        _priority: &str,
        _which: Option<&str>,
    ) -> Result<Option<u32>, String> {
        Err("this desktop cannot set the priority of a process".to_string())
    }

    /// Kill a process.
    ///
    /// The answer is a failure with the reason when the machine cannot do
    /// it: a process that is not killed is never reported as killed.
    fn kill_a_process(&mut self, _pid: u32) -> Result<(), String> {
        Err("this desktop cannot kill a process".to_string())
    }

    fn the_state_of_a_process(&mut self, _pid: u32) -> Result<Option<ProcessState>, String> {
        Err("this desktop cannot say the state of a process".to_string())
    }

    /// Start a program with a text on its standard input and wait for
    /// it; return the pid, the code of the exit and what it wrote.
    fn run_with_stdin(
        &mut self,
        _program: &str,
        _arguments: &[String],
        _standard_input: &str,
        _working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        Err("this desktop cannot launch a program".to_string())
    }

    /// Start a program with a text on its standard input, and do not
    /// wait for it; return the pid of it.
    ///
    /// The runner of the tree reads its script on the standard input, so
    /// a program that is started this way receives what the script gives
    /// it there. A desktop that cannot start a program says so.
    fn launch_with_stdin(
        &mut self,
        _program: &str,
        _arguments: &[String],
        _standard_input: &str,
        _working_dir: &str,
    ) -> Result<u32, String> {
        Err("this desktop cannot launch a program".to_string())
    }
}

/// The files of the desktop.
pub trait Files {
    /// Create a shortcut to a program, the way this platform makes one.
    ///
    /// Measured on the binary of v2.0.19: `FileCreateShortcut` answers the
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
    /// Measured on the binary of v2.0.19: `FileRecycleEmpty` answers the
    /// empty text.
    fn empty_the_trash(&mut self) -> Result<(), String> {
        Err("this desktop cannot empty the trash".to_string())
    }

    /// Put one file in the trash of the machine.
    ///
    /// Measured on the binary of v2.0.19: `FileRecycle(path)`
    /// answers the empty text and the file is gone. An earlier note said the
    /// reference answers "Failed" for a file that exists -- that was a path
    /// that was not there, not the name.
    fn recycle_a_file(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot put a file in the trash".to_string())
    }

    /// Read a shortcut back: (target, dir, args, description, icon).
    ///
    /// Measured on the binary of v2.0.19: `FileGetShortcut` of a
    /// shortcut answers all eight of its fields. The icon NUMBER and the run
    /// state have no counterpart in a `.desktop` file, so those two answers
    /// are the empty text here.
    fn read_shortcut(
        &mut self,
        _shortcut: &str,
    ) -> Result<(String, String, String, String, String), String> {
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

    /// The path of a file, in the form the machine writes it.
    ///
    /// Measured on the binary of v2.0.19: `TraySetIcon` puts
    /// the icon of the script, and `A_IconFile` answers the path of it in
    /// the canonical form of the machine -- "C:\WINDOWS\System32\SHELL32.dll"
    /// for the shell32.dll that the script wrote in another case.
    fn canonical_path(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot write the canonical form of a path".to_string())
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

    /// Open a file with the program the machine opens it with.
    ///
    /// The shell of the machine knows the program of a kind of file: the
    /// editor of a script, the player of a sound, the reader of a text.
    /// This is the `Edit` of v2.0.31, which opens the script of the run
    /// in the editor of the machine.
    fn open_with_the_default_program(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot open a file with the program of the machine".to_string())
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

    /// Empty the trash of the machine.
    ///
    /// The answer is nothing. A `drive` that is given names the trash to
    /// empty, and the deskop that has one trash for the whole machine
    /// empties it for any drive.
    fn empty_trash(&mut self, _drive: Option<&str>) -> Result<(), String> {
        Err("this desktop has no trash to empty".to_string())
    }

    /// Recycle a file or a folder, or the files that a pattern matches.
    ///
    /// Measured on the binary of v2.0.19: a file that is there
    /// is recycled and the answer is nothing; a plain path that is not there
    /// is the failure "Failed"; and a pattern that matches nothing is not a
    /// failure. A desktop with no trash says so.
    fn recycle(&mut self, _pattern: &str) -> Result<(), String> {
        Err("this desktop has no trash to recycle a file into".to_string())
    }

    /// Make an empty temporary file, and answer its full path.
    ///
    /// Measured on the documentation of KeySharp:
    /// `FileCreateTemp()` makes the file and answers where it is.
    fn create_temp_file(&mut self) -> Result<String, String> {
        Err("this desktop cannot make a temporary file".to_string())
    }

    /// The directory the script is in.
    ///
    /// The name of the directory is a subfolder of the working directory
    /// when it is not an absolute path.
    /// Create a shortcut to a program, the way this platform makes one.
    ///
    /// Measured on the binary of v2.0.19: `FileCreateShortcut`
    /// answers the empty text and makes the file, and it makes it even for a
    /// target that is not there.
    /// The full path to a file, the way the platform writes one.
    ///
    /// Measured on the documentation of KeySharp:
    /// `FileFullPath(Filename)` answers the full path to the file.
    fn full_path(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot say the full path of a file".to_string())
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

    /// Everything that is on the clipboard, as the bytes of every format.
    ///
    /// The `ClipboardAll` takes the whole clipboard, and this is the
    /// capture of it. A desktop that cannot read the binary clipboard
    /// says so.
    fn clipboard_read_all(&mut self) -> Result<Vec<u8>, String> {
        Err("this desktop cannot read the binary clipboard".to_string())
    }

    /// Put the bytes of every format back on the clipboard.
    fn clipboard_write_all(&mut self, _data: &[u8]) -> Result<(), String> {
        Err("this desktop cannot write the binary clipboard".to_string())
    }

    /// The bytes of the memory of the process at an address.
    ///
    /// The `ClipboardAll(address, size)` reads the bytes of the data of
    /// the clipboard out of the memory. The read is safe: an address that
    /// is not readable is a reason, and not a crash of the process. A
    /// desktop that cannot read the memory says so.
    fn read_memory(&mut self, _address: i64, _size: i64) -> Result<Vec<u8>, String> {
        Err("this desktop cannot read the memory of the process".to_string())
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

    /// Whether a menu of the script is on the screen.
    ///
    /// The v1 does not run the timed subroutines while a menu is
    /// visible: the menu would be sluggish, and the script would change
    /// the menu while the user is in it.
    ///
    /// The v1: "Checking g_MenuIsVisible here prevents timed
    /// subroutines from running while the tray menu or main menu is in
    /// use" (`script2.cpp:5529-5535` of v1.1.37.02).
    fn the_menu_is_visible(&mut self) -> bool {
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

    /// Wait for an event of the desktop, at most `timeout` milliseconds.
    ///
    /// The desktop that watches the keyboard knows when a key was pressed
    /// without the runtime asking it every few milliseconds: this is the
    /// wait, and it blocks until something arrives or the timeout is over.
    /// The answer says whether something arrived.
    ///
    /// The default refuses, and the reason is the one the tree gives for a
    /// desktop that cannot: a runtime that has no wait of this kind waits
    /// with its own clock and says so once.
    fn wait_for_event(&mut self, _timeout: u64) -> Result<bool, String> {
        Err("this desktop cannot wait for an event".to_string())
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

    /// A call to an address of the machine: `DllCall` of a number (the
    /// callback of `CallbackCreate`, say) calls the place itself, and not
    /// a library that carries the function.
    fn call_native_at(
        &mut self,
        _address: u64,
        _call: &mut NativeCall,
    ) -> Result<NativeResult, NativeFailure> {
        Err(NativeFailure::invalid(
            "this desktop cannot make a native call.",
        ))
    }

    /// Preload a library (`#DllLoad`): a call that names only the function
    /// may find it among the loaded libraries.
    fn load_library(&mut self, _name: &str) -> Result<(), String> {
        Err("this desktop cannot load a library.".to_string())
    }

    /// `CallbackCreate`: a callable address that runs the function when the
    /// machine calls it. A desktop with no native callbacks refuses.
    fn create_callback(
        &mut self,
        _func: &crate::runtime::Value,
        _param_count: usize,
    ) -> Result<u64, String> {
        Err("this desktop cannot create a native callback.".to_string())
    }

    /// `CallbackFree`: release a callback address.
    fn free_callback(&mut self, _address: u64) {}

    /// The bytes of the memory of the machine at an address.
    ///
    /// The `StrGet` of the v1 reads the memory of the machine: the text of
    /// a structure that a call of the machine filled, the buffer that a
    /// call allocated. The desktop knows the memory of its machine, and
    /// the default refuses.
    ///
    /// The answer holds the bytes that were read, which may be less than
    /// the ones that were asked for: the read stops at the end of the
    /// memory that can be read.
    fn the_bytes_of_the_memory(&mut self, _address: u64, _length: usize) -> Option<Vec<u8>> {
        None
    }

    /// Write bytes into the memory of the machine at an address.
    ///
    /// The answer is whether the bytes were written: a desktop that cannot
    /// write into the memory of its machine says that it did not.
    fn the_memory_is_written(&mut self, _address: u64, _bytes: &[u8]) -> bool {
        false
    }
}

/// The answer of a member of an object of the COM.
#[derive(Clone, Debug, PartialEq)]
pub enum ComAnswer {
    /// Nothing: the member had no value.
    Empty,
    /// A number.
    Number(f64),
    /// A text.
    Text(String),
    /// Another object: the pointer and its kind.
    Object { pointer: u64, kind: i64 },
}

/// What the script is doing with the member.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberWhat {
    /// Reading a property.
    Get,
    /// Writing a property.
    Set,
    /// Calling a method.
    Call,
}

/// An object of the COM of the machine.
///
/// The object is the pointer that the machine gave and the kind of the
/// value of it -- the `VARTYPE` of v2 -- and the tree carries both as
/// numbers, because the crate does no I/O of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComObject {
    /// The pointer of the object.
    pub pointer: u64,
    /// The kind of the value: the `VARTYPE` of v2.
    pub kind: i64,
}

/// One item of a key of the registry, as the `Loop, Reg` of the v1 reads
/// it: the `A_LoopRegName`, the `A_LoopRegType`, the `A_LoopRegKey`, the
/// `A_LoopRegSubKey` and the `A_LoopRegTimeModified`.
#[derive(Clone, Debug, Default)]
pub struct TheRegEntry {
    /// The name of the value or of the subkey.
    pub name: String,
    /// "KEY" for a subkey, and the `REG_*` kind for a value.
    pub kind: String,
    /// The root of the key, spelled whole: "HKEY_CURRENT_USER".
    pub root: String,
    /// The key that is enumerated, without the root of it.
    pub subkey: String,
    /// The time the subkey was written last, `YYYYMMDDHH24MISS`; a value
    /// has none.
    pub modified: String,
}

/// A value that goes into the COM of the machine.
#[derive(Clone, Debug, PartialEq)]
pub enum ComValue {
    /// A number.
    Number(f64),
    /// A text.
    Text(String),
    /// The pointer of an object.
    Pointer(u64),
}

/// An event of an object of the COM.
///
/// The machine gives the event when the object fires it, and the name of
/// it is the name of the function of the script that receives it -- the
/// prefix of the connection and the name of the member.
#[derive(Clone, Debug, PartialEq)]
pub struct ComEvent {
    /// The name of the event: the prefix and the name of the member.
    pub name: String,
    /// The pointer of the object that fired the event.
    pub object: u64,
    /// The kind of the value of the object: the `VARTYPE` of v2.
    pub kind: i64,
    /// The arguments of the event.
    pub arguments: Vec<ComValue>,
}

/// The COM of the machine.
///
/// The COM is a thing of Windows: a desktop that does not have it says
/// what it does not have, and the refusal carries the reason.
///
/// The shape of it is the one of the v2.0.31, read in its source
/// (`source/script_com.cpp` of the reference tree): the class of the
/// object is the name of a class of the registry -- a `ProgID` or the
/// text of a `CLSID` -- and the interface of it is the text of an `IID`,
/// and the one of the default is the `IDispatch`.
pub trait Com {
    /// The object of a class of the machine.
    ///
    /// The class is the text of a `ProgID` or of a `CLSID`, and the
    /// interface is the text of an `IID` -- the `IDispatch` when the
    /// script does not say.
    fn com_object_of_the_class(
        &mut self,
        _class: &str,
        _interface: Option<&str>,
    ) -> Result<ComObject, String> {
        Err(
            "the COM objects are a thing of Windows, and this runtime does not carry them."
                .to_string(),
        )
    }

    /// The object that a program of the machine has running.
    fn com_object_of_the_running(&mut self, _class: &str) -> Result<ComObject, String> {
        Err("(0x800401F3) Invalid class string".to_string())
    }

    /// The object that the machine gives for a name.
    ///
    /// The name is a moniker of the machine: the text of a `CLSID`, or
    /// the name of a file, or the name of an object that is running.
    fn com_object_of_the_name(&mut self, _name: &str) -> Result<ComObject, String> {
        Err("(0x80070057) The parameter is incorrect.".to_string())
    }

    /// Another interface of the same object.
    fn com_object_is_queried(
        &mut self,
        _object: &ComObject,
        _interface: &str,
    ) -> Result<ComObject, String> {
        Err(
            "this platform has no COM: there is no object model for an interface to be asked of"
                .to_string(),
        )
    }

    /// The name or the number of the class or of the interface of an
    /// object.
    ///
    /// The `what` is "Name" or "IID" -- and "Class" and "CLSID" are the
    /// names of the information of the class.
    fn com_type_of_the_object(
        &mut self,
        _object: &ComObject,
        _what: &str,
    ) -> Result<String, String> {
        Err(
            "this platform has no COM: there is no object model for a class to be asked of"
                .to_string(),
        )
    }

    /// A value of the COM: the kind of it and the number of it.
    ///
    /// The kind is the `VARTYPE` of v2, and the number is the value of
    /// it: the pointer of an object, the bits of a number, or the
    /// address of a text that the machine made.
    fn com_value_is_made(&mut self, _kind: i64, _value: ComValue) -> Result<ComObject, String> {
        Err(
            "the COM objects are a thing of Windows, and this runtime does not carry them."
                .to_string(),
        )
    }

    /// The count of a COM pointer, one more: the `ObjAddRef` of the v1.
    fn com_pointer_is_counted(&mut self, _pointer: u64) -> Result<u64, String> {
        Err(
            "the COM objects are a thing of Windows, and this runtime does not carry them."
                .to_string(),
        )
    }

    /// The count of a COM pointer, one less: the `ObjRelease` of the v1.
    fn com_pointer_is_released(&mut self, _pointer: u64) -> Result<u64, String> {
        Err(
            "the COM objects are a thing of Windows, and this runtime does not carry them."
                .to_string(),
        )
    }

    /// The write of the default member of the object: the `obj := value`
    /// of the v1, the `DISPID_VALUE` of the COM.
    fn com_default_is_written(&mut self, _object: &ComObject, _value: ComValue) -> Result<(), String> {
        Err(
            "the COM objects are a thing of Windows, and this runtime does not carry them."
                .to_string(),
        )
    }

    /// The call of a method of the object, by its place in the table.
    ///
    /// The call of the COM is a call of the machine: the method is the
    /// place of it in the table of the object, and the object is the
    /// first argument of the call.
    fn com_method_is_called(
        &mut self,
        _object: &ComObject,
        _index: usize,
        _arguments: &mut [NativeArg],
        _returns: NativeType,
        _failure_is_an_error: bool,
    ) -> Result<NativeResult, String> {
        Err(
            "this platform has no COM: there is no object model behind the call of a method"
                .to_string(),
        )
    }

    /// The script is connected to the events of an object.
    ///
    /// The prefix is what the names of the events carry, and the script
    /// takes each event in the function whose name is the prefix and the
    /// name of the member.
    fn com_object_is_connected(
        &mut self,
        _object: &ComObject,
        _prefix: &str,
    ) -> Result<(), String> {
        Err(
            "the events of a COM object are a sink that this runtime does not carry: the script cannot be called back by the machine."
                .to_string(),
        )
    }

    /// The events of the COM that the machine gave, and the wait for them.
    ///
    /// The wait is the time the tree gives the machine for the events to
    /// arrive: the machine calls the script by the queue of the thread of
    /// the apartment, and the queue moves only when the thread looks at it.
    fn com_events_are_taken(&mut self, _wait_ms: u64) -> Vec<ComEvent> {
        Vec::new()
    }

    /// A member of an object of the COM, got, set or called.
    ///
    /// The member is named by the script, and the machine turns the name
    /// into its number and answers with the value of the `VARIANT`.
    fn com_member(
        &mut self,
        _object: &ComObject,
        _what: MemberWhat,
        _name: &str,
        _arguments: &[ComValue],
    ) -> Result<ComAnswer, String> {
        Err(
            "this platform has no COM: there is no object model for a member to be asked of"
                .to_string(),
        )
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
    /// Measured on the binary of v2.0.19: `SysGetIPAddresses`
    /// answers an **array** of the addresses as text, in the order the machine
    /// lists them.
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the addresses of its interfaces.".to_string())
    }

    /// Send a mail message through the network of the machine.
    ///
    /// A desktop that cannot send a mail says so with the reason: a
    /// message that was not sent is never reported as sent.
    fn send_mail(&mut self, _message: &MailMessage) -> Result<(), String> {
        Err("this desktop cannot send mail".to_string())
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
/// muted, and the playing of a sound.
///
/// The device is named by the script the way the v2.0.31 names it, read
/// in its source (`source/lib/sound.cpp` of the reference tree): the
/// empty name is the device of the system, a number is the place of the
/// device in the list of every device of the machine, and a name is the
/// first device whose name begins with it -- the comparison of the v2
/// is a prefix and it does not mind the case.
pub trait Sound {
    /// The names of the sound devices of the machine: the one that plays
    /// and the one that records.
    ///
    /// Measured on the binary of v2.0.19: `SoundGetName` with
    /// no component answers the name of the device that plays -- "Speakers"
    /// on the machine of that day -- and with the component one the name
    /// of the one that records -- "Microphone".
    fn sound_device_names(&mut self) -> Result<(String, String), String> {
        Err("this desktop cannot tell the names of the sound devices".to_string())
    }

    /// The name of the sound device of this name, if the machine has one.
    ///
    /// Measured on the binary of v2.0.19: `SoundGetName` with
    /// a device that the machine does not have throws "Component not
    /// found.".
    fn sound_device_named(&mut self, _device: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot tell the names of the sound devices".to_string())
    }
    /// The volume of a device, as a number between 0 and 100.
    ///
    /// The component is the name of the part of the sound whose volume is
    /// wanted, and the empty text is the master of the device.
    ///
    /// Measured on the binary of v2.0.31: the
    /// `SoundGetVolume("Microphone")` answers the volume of the microphone
    /// and o `SoundGetVolume("")` responde o master do device.
    fn sound_volume(&mut self, _device: &str, _component: &str) -> Result<f64, String> {
        Err("this desktop cannot say the volume of a device".to_string())
    }

    /// Set the volume of a device, as a number between 0 and 100.
    fn sound_set_volume(
        &mut self,
        _device: &str,
        _component: &str,
        _volume: f64,
    ) -> Result<(), String> {
        Err("this desktop cannot set the volume of a device".to_string())
    }

    /// Whether a device is muted.
    fn sound_muted(&mut self, _device: &str, _component: &str) -> Result<bool, String> {
        Err("this desktop cannot say whether a device is muted".to_string())
    }

    /// Mute a device, or unmute it.
    fn sound_set_muted(
        &mut self,
        _device: &str,
        _component: &str,
        _muted: bool,
    ) -> Result<(), String> {
        Err("this desktop cannot mute a device".to_string())
    }

    /// The pointer of an interface of a device of the sound.
    ///
    /// The interface is the text of an `IID`, and the answer is the
    /// address of it -- zero when the machine does not have the
    /// interface of that device.
    ///
    /// Measured on the binary of v2.0.19: `SoundGetInterface`
    /// of a GUID that the machine does not carry the interface of answers
    /// zero, and not a failure. A desktop that has no device of the sound
    /// answers zero too.
    fn sound_interface_of_the_device(
        &mut self,
        _device: &str,
        _interface: &str,
    ) -> Result<u64, String> {
        Ok(0)
    }

    /// Play a sound file.
    fn sound_play(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot play a sound".to_string())
    }

    /// The value of a control of a component of a device of the sound.
    ///
    /// The v1 names a part of the sound in two words: the component --
    /// the "Master", the "Microphone", the "Wave" -- and the control --
    /// the "Volume", the "Mute". The answer is the value of the control,
    /// and the failure says which of the two the machine does not have.
    ///
    /// Measured on the binary of v1.1.37.02: the master
    /// and the microphone of the machine of that day have a volume, and
    /// the other components do not exist; the master has a mute, and the
    /// other controls do not exist either.
    fn sound_control(
        &mut self,
        _component: &str,
        _control: &str,
        _device: &str,
    ) -> Result<SoundControl, SoundFailure> {
        Err(SoundFailure::NoControl)
    }

    /// Write the value of a control of a component of a device.
    fn sound_control_is_set(
        &mut self,
        _component: &str,
        _control: &str,
        _value: f64,
        _device: &str,
    ) -> Result<(), SoundFailure> {
        Err(SoundFailure::NoControl)
    }

    /// Play a tone of a frequency, for a number of milliseconds.
    fn sound_beep(&mut self, _frequency: f64, _duration_ms: u64) -> Result<String, String> {
        Err("this desktop cannot play a tone".to_string())
    }
}

/// The value of a control of the sound of the machine.
#[derive(Debug, Clone, PartialEq)]
pub enum SoundControl {
    /// A number between zero and a hundred: the volume of a device.
    Volume(f64),
    /// Whether a control is on or off: the mute of a device.
    Switch(bool),
}

/// The reason why a control of the sound of the machine is not there.
///
/// The words of the v1 are the ones of the reasons: measured on the
/// binary of v1.1.37.02, the `ErrorLevel` of a `SoundGet`
/// that does not find the component is "Mixer Doesn't Support This
/// Component Type", and the one that does not find the control is
/// "Component Doesn't Support This Control Type".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoundFailure {
    /// The machine has no such component: "Mixer Doesn't Support This
    /// Component Type".
    NoComponent,
    /// The component has no such control: "Component Doesn't Support This
    /// Control Type".
    NoControl,
    /// The mixer of the device cannot be opened: "Can't Open Specified
    /// Mixer".
    NoMixer,
    /// The setting cannot be read: "Can't Get Current Setting".
    NoSetting,
    /// The setting cannot be written: "Can't Change Setting".
    ///
    /// The phrase is the one of the `SoundSet`, and not the one of the
    /// `SoundGet`: the binary of v1.1.37.02 answers one phrase in each
    /// case.
    NoChange,
}

impl SoundFailure {
    /// The words of the v2 for this reason.
    ///
    /// Measured on the binary of v2.0.31: o
    /// `SoundGetVolume("zzz")` recusa com "Component not found", o
    /// `SoundGetVolume("1")` de um device que nao tem o componente
    /// recusa com "Component doesn't support this control type", e o
    /// `SoundGetVolume("", "zzzNotADevice")` recusa com "Device not
    /// found".
    pub fn the_words_of_the_v2(&self) -> &'static str {
        match self {
            SoundFailure::NoComponent => "Component not found",
            SoundFailure::NoControl => "Component doesn't support this control type",
            SoundFailure::NoMixer => "Device not found",
            SoundFailure::NoSetting => "Cannot get current setting",
            SoundFailure::NoChange => "Cannot change setting",
        }
    }

    /// The words of the v1 for this reason.
    ///
    /// Measured on the binary of v1.1.37.02: the phrases
    /// are the ones of the `ErrorLevel` of the v1.
    pub fn the_words_of_the_v1(&self) -> &'static str {
        match self {
            SoundFailure::NoComponent => "Mixer Doesn't Support This Component Type",
            SoundFailure::NoControl => "Component Doesn't Support This Control Type",
            SoundFailure::NoMixer => "Can't Open Specified Mixer",
            SoundFailure::NoSetting => "Can't Get Current Setting",
            SoundFailure::NoChange => "Can't Change Setting",
        }
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
/// A value that crosses a thread: the arguments and the answer of a worker.
///
/// The values of the script hold references that do not cross a thread;
/// the worker takes the number or the text of a value, and makes a value
/// of it on its own side.
#[derive(Clone, Debug, PartialEq)]
pub enum ThreadValue {
    Int(i64),
    Float(f64),
    Text(String),
    /// A list of values that cross: the items are numbers, text, or lists
    /// of them. An object of the script holds references that do not cross.
    Array(Vec<ThreadValue>),
}

impl ThreadValue {
    /// The value of the script of a value that crossed a thread.
    pub fn to_value(&self) -> crate::runtime::Value {
        match self {
            ThreadValue::Int(number) => crate::runtime::Value::Int(*number),
            ThreadValue::Float(number) => crate::runtime::Value::Float(*number),
            ThreadValue::Text(text) => crate::runtime::Value::Text(text.as_str().into()),
            ThreadValue::Array(items) => crate::classes::array::of_items(
                items.iter().map(|item| item.to_value()).collect(),
            ),
        }
    }

    /// The value that crosses a thread of a value of the script: the
    /// numbers, the text and the lists of them. An object becomes the text
    /// of it, for its references do not cross.
    pub fn of_value(value: &crate::runtime::Value) -> ThreadValue {
        match value {
            crate::runtime::Value::Int(number) => ThreadValue::Int(*number),
            crate::runtime::Value::Float(number) => ThreadValue::Float(*number),
            crate::runtime::Value::Text(text) => ThreadValue::Text(text.clone().into()),
            crate::runtime::Value::List(items) => {
                ThreadValue::Array(items.iter().map(ThreadValue::of_value).collect())
            }
            crate::runtime::Value::Object(object)
                if object.borrow().class.eq_ignore_ascii_case("array") =>
            {
                ThreadValue::Array(
                    crate::classes::array::the_items(value)
                        .iter()
                        .map(ThreadValue::of_value)
                        .collect(),
                )
            }
            other => match other.to_number() {
                Some(number) if crate::runtime::fract(number) == 0.0 => ThreadValue::Int(number as i64),
                Some(number) => ThreadValue::Float(number),
                None => ThreadValue::Text(other.to_text()),
            },
        }
    }
}

/// The table of the globals a run shares with its workers, live.
///
/// With the std it is a table the threads share behind a lock; without the
/// std there are no worker threads, and the type is a placeholder.
#[cfg(feature = "std")]
pub type SharedGlobals =
    std::sync::Arc<std::sync::Mutex<std::collections::BTreeMap<String, crate::runtime::Value>>>;
#[cfg(not(feature = "std"))]
pub type SharedGlobals = ();

/// The state of a worker thread.
#[derive(Clone, Debug, Default)]
pub struct WorkerState {
    /// Whether the worker is still running.
    pub alive: bool,
    /// Whether the worker has finished.
    pub terminated: bool,
    /// The answer of the function, when it has finished.
    pub result: Option<ThreadValue>,
    /// The code the worker ended with.
    pub exit_code: i64,
    /// The name of the function the worker runs.
    pub task: String,
    /// The failure of the function, when it did not answer.
    pub error: Option<String>,
}

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

    /// The code of the last error of the machine.
    ///
    /// It is the code that the `OSError` of the v2 carries when the script
    /// does not name it: measured on the binary of v2.0.32, the `OSError()`
    /// that follows a native call that failed answers "(87) The parameter
    /// is incorrect.".
    fn last_error(&mut self) -> Result<i64, String> {
        Err("this desktop cannot say the last error of the machine".to_string())
    }

    /// The error of the machine is told, and it is the one that the next
    /// native call of the script sees.
    ///
    /// Measured on the binary of v2.0.32: the `DllCall` of a script that
    /// called nothing sees the error of the script, and not the one that
    /// the calls of the interpreter left behind.
    fn set_last_error(&mut self, _code: i64) -> Result<(), String> {
        Err("this desktop cannot set the last error of the machine".to_string())
    }

    /// The text that the machine says about the code of an error.
    ///
    /// Measured on the binary of v2.0.32: the `OSError(87)` answers "(87)
    /// The parameter is incorrect.".
    fn the_text_of_an_error(&mut self, _code: i64) -> Result<String, String> {
        Err("this desktop cannot say what the code of an error means".to_string())
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

    /// The value of the registry of the machine, and its kind.
    ///
    /// `None` is a value that is not there; a key that is not there is a
    /// failure with the reason of the machine. Measured on the binary of
    /// v2.0.19: `RegRead` of a value that is not there
    /// throws "(2) The system cannot find the file specified.".
    fn registry_value(
        &mut self,
        _key: &str,
        _name: &str,
    ) -> Result<Option<(String, String)>, String> {
        Err("this desktop cannot read the registry of the machine".to_string())
    }

    /// Write a value into the registry of the machine.
    ///
    /// The key is made when it is not there, with its ancestors: measured
    /// on the binary of v2.0.19.
    fn registry_write(
        &mut self,
        _key: &str,
        _name: &str,
        _kind: &str,
        _value: &str,
    ) -> Result<(), String> {
        Err("this desktop cannot write the registry of the machine".to_string())
    }

    /// Remove a value of the registry of the machine.
    fn registry_delete(&mut self, _key: &str, _name: &str) -> Result<(), String> {
        Err("this desktop cannot remove a value of the registry".to_string())
    }

    /// Remove a key of the registry of the machine.
    fn registry_delete_key(&mut self, _key: &str) -> Result<(), String> {
        Err("this desktop cannot remove a key of the registry".to_string())
    }

    /// Make a key of the registry of the machine.
    fn registry_create_key(&mut self, _key: &str) -> Result<(), String> {
        Err("this desktop cannot make a key of the registry".to_string())
    }

    /// The items of a key of the registry, for the `Loop, Reg` of the v1:
    /// the values and the subkeys, and the subkeys of the subkeys when the
    /// mode asks for the recursion.
    ///
    /// The flag says that a key that cannot be opened is not a failure:
    /// the key gives no items and the enumeration goes on. The v1 asks
    /// for it, and the v2 does not.
    fn registry_enumerate(
        &mut self,
        _key: &str,
        _mode: &str,
        _quiet: bool,
    ) -> Result<Vec<TheRegEntry>, String> {
        Err("this desktop cannot enumerate the registry of the machine".to_string())
    }

    /// The directory of the Windows of the machine.
    ///
    /// Measured on the binary of v2.0.19: `A_WinDir` answers
    /// "C:\WINDOWS" on a machine of Windows.
    fn windows_directory(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell the directory of the Windows of the machine".to_string())
    }

    /// The directory of the programs of the user, in the Windows of the machine.
    ///
    /// Measured on the binary of v2.0.19: `A_ProgramFiles`
    /// answers "C:\Program Files" on a machine of Windows.
    fn program_files(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell the directory of the programs of the machine".to_string())
    }

    /// Put the icon of the script in the tray of the session.
    ///
    /// Measured on the binary of v2.0.19: `TraySetIcon` puts
    /// the icon of the script in the tray, and the built-in variables carry
    /// it afterwards.
    fn tray_set_icon(&mut self, _name: &str) -> Result<(), String> {
        Err("this desktop has no tray to put the icon of the script in".to_string())
    }

    /// The icon of a file of the machine, and the answer of whether the
    /// file has one.
    ///
    /// The `number` is the place of the icon in the file: the first one is
    /// the number one. A file that has no icon is a failure with the
    /// reason.
    fn icon_of_a_file(&mut self, _path: &str, _number: i64) -> Result<(), String> {
        Err("this desktop cannot read the icons of the files of the machine".to_string())
    }

    /// Show the persistent icon of the script in the tray of the session.
    ///
    /// A run that stays alive -- a hotkey, a Gui, a message monitor -- puts
    /// the icon of the script in the tray, the way the original AutoHotkey
    /// does, so that the script can be closed from it.
    fn tray_of_the_script_is_shown(&mut self) -> Result<(), String> {
        Err("this desktop has no tray to put the icon of the script in".to_string())
    }

    /// Whether the Exit of the tray of the script was chosen.
    fn tray_exit_was_asked(&mut self) -> bool {
        false
    }

    /// Take the icon of the script out of the tray of the session.
    fn tray_of_the_script_is_removed(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// The label of the drive that a path is on.
    ///
    /// `None` is a drive that is not there. Measured on the binary of
    /// v2.0.19: `DriveGetLabel` of a drive that is not there
    /// throws "(3) The system cannot find the path specified.".
    fn drive_label(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot tell the label of a drive".to_string())
    }

    /// The serial of the drive that a path is on, as the machine writes it.
    ///
    /// Measured on the binary of the same day: `DriveGetSerial` of a drive
    /// that is not there throws the same "(3) ..." as the label, and of a
    /// drive that is there answers a whole number -- the volume serial of
    /// Windows. The serial of a Linux drive is the string of the hardware,
    /// so the shape is the machine's.
    fn drive_serial(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot tell the serial of a drive".to_string())
    }

    /// The drives of the machine, one name per line.
    ///
    /// Measured on the binary of the same day: `DriveGetList` answers the
    /// drive letters run together -- a letter is one character, so no
    /// separator is needed. A drive of this platform is a device name, which
    /// is more than one character, so the names ride apart on newlines.
    fn drive_list(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the drives".to_string())
    }

    /// The status of the drive a path is on, when that drive is a CD or DVD
    /// reader. `None` is a drive that is not a reader.
    ///
    /// Measured on the binary of the same day: `DriveGetStatusCD` of a drive
    /// that is not a reader throws "Failed".
    fn drive_status_cd(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot tell the status of a CD drive".to_string())
    }

    /// Eject the drive of a path.
    fn drive_eject(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot eject a drive".to_string())
    }

    /// Lock the drive of a path against removal.
    fn drive_lock(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot lock a drive".to_string())
    }

    /// Unlock the drive of a path.
    fn drive_unlock(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot unlock a drive".to_string())
    }

    /// Spin the drive of a path down.
    fn drive_retract(&mut self, _path: &str) -> Result<(), String> {
        Err("this desktop cannot spin a drive down".to_string())
    }

    /// Write a new label on the drive of a path.
    fn drive_set_label(&mut self, _path: &str, _label: &str) -> Result<(), String> {
        Err("this desktop cannot write the label of a drive".to_string())
    }

    fn idle_times(&mut self) -> Result<Idle, String> {
        Err("this desktop cannot say how long the session has been idle".to_string())
    }

    /// The state of the drive that a path is on.
    ///
    /// `None` is a path that is not a drive of this machine: the binary
    /// answers "Invalid" for it.
    fn drive_status(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot say what state a drive is in".to_string())
    }

    /// The kind of the drive that a path is on.
    ///
    /// `None` is a path that is not a drive of this machine: the binary
    /// answers the empty text for it.
    fn drive_type(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot say what kind a drive is".to_string())
    }

    /// The drive that a path is on, when the path is one.
    ///
    /// Measured on the binary of v2.0.19: `DriveGetType` of a
    /// drive that is not there answers the empty text, and `DriveGetStatus`
    /// of one answers "Invalid"; the other readings of a drive that is not
    /// there are the failure of the machine.
    fn drive(&mut self, _path: &str) -> Result<Option<Drive>, String> {
        Err("this desktop cannot say what a drive is".to_string())
    }

    /// Write a text to the debugger of the machine.
    ///
    /// Measured on the binary of v2.0.19: `OutputDebug` writes
    /// the text to the debugger and answers nothing. A machine whose debugger
    /// is the standard error of the process writes it there.
    fn output_debug(&mut self, _text: &str) -> Result<(), String> {
        Err("this desktop has no debugger to write to".to_string())
    }

    /// The identity of the thread the run is on.
    ///
    /// The identity of a thread is the machine's, and a machine that
    /// cannot say is a failure with the reason: the number of a thread
    /// is not guessed.
    fn the_current_thread_id(&mut self) -> Result<i64, String> {
        Err("this desktop cannot say the thread it runs on".to_string())
    }

    /// Create a worker thread that runs a function of the script.
    ///
    /// The worker runs on a real thread of the machine: it parses the
    /// source again on its own side and runs the function there. The
    /// answer is the number that names the worker. The dialect is the name
    /// of it -- "v1", "v2", "ks" -- and the arguments are the values that
    /// cross a thread.
    fn create_worker_thread(
        &mut self,
        _source: &str,
        _dialect: &str,
        _function: &str,
        _args: Vec<ThreadValue>,
        _globals: Vec<(String, ThreadValue)>,
        _shared: Option<SharedGlobals>,
    ) -> Result<u64, String> {
        Err("this desktop cannot create a worker thread".to_string())
    }

    /// The state of a worker thread: alive, terminated, and the answer.
    fn worker_thread_state(&mut self, _id: u64) -> Result<WorkerState, String> {
        Err("this desktop cannot say the state of a worker thread".to_string())
    }

    /// The workers that are alive: the number, the task, of each.
    fn worker_thread_list(&mut self) -> Result<Vec<(u64, String)>, String> {
        Err("this desktop cannot list the worker threads".to_string())
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

    /// Show a message box and answer the name of the button that was
    /// pressed (OK, Cancel, Yes, No, ...). The options are the number of
    /// v2, whose buttons and icon are the ones of the machine's message
    /// box. A desktop with no window answers OK without showing anything.
    fn msg_box(&mut self, _text: &str, _title: &str, _options: i64) -> Result<String, String> {
        Ok("OK".to_string())
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

/// A place of the options of the `Gui, Add` of the v1.
///
/// The v1 has many ways to say where a control goes: the place of the
/// client area of the window, the place of the control that was added
/// before it, the margin of the window and the section of the window. The
/// tree reads the way that the script wrote and asks the machine for the
/// place.
///
/// Read in the CHM of the v1.1.37.02 (`Gui.htm`) and in the source of the
/// v1 (`script_gui.cpp:5731-5835`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheCoordinate {
    /// `x10`: ten pixels of the client area of the window.
    Absolute(i64),
    /// `x+10`: the right edge of the control before it, ten pixels more.
    AfterThePrevious(i64),
    /// `x+m`: the right edge of the control before it, the margin more.
    AfterThePreviousWithTheMargin,
    /// `xm+10`: the margin of the window, ten pixels more.
    FromTheMargin(i64),
    /// `xp+10`: the left edge of the control before it, ten pixels more.
    FromThePrevious(i64),
    /// `xs+10`: the section of the window, ten pixels more.
    FromTheSection(i64),
}

/// The size of a control of the options of the `Gui, Add` of the v1.
///
/// Read in the source of the v1 (`script_gui.cpp:5717-5728`): the `wp` and
/// the `hp` are the size of the control that was added before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheSize {
    /// `w100`: a hundred pixels.
    Absolute(i64),
    /// `wp`: the size of the control before it.
    FromThePrevious(i64),
}

/// The place and the size that the options of the `Gui, Add` of the v1 ask
/// for.
///
/// The place and the size that are not there are the ones that the control
/// takes by itself: the place of the control before it, and the size that
/// the kind of it asks for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ThePlaceOfTheV1 {
    /// The `x` of the options: where the control goes from the left.
    pub x: Option<TheCoordinate>,
    /// The `y` of the options: where the control goes from the top.
    pub y: Option<TheCoordinate>,
    /// The `w` of the options: how wide the control is.
    pub width: Option<TheSize>,
    /// The `h` of the options: how high the control is.
    pub height: Option<TheSize>,
    /// The `r` of the options: how many rows tall the control is (`r1`, `r17`).
    pub rows: Option<i64>,
    /// Whether the options ask for a new section.
    ///
    /// The v1 keeps the place of a section, and the `xs` and the `ys` of
    /// the options of a control are the place of it.
    pub section: bool,
}

/// The bits of the style of a control that the options do ask for.
///
/// The v1: "Pure numbers are assumed to be style additions or removals"
/// (source `script_gui.cpp:5564-5572` of v1.1.37.02), and the style of
/// the control that is made is `style_add & ~style_remove`.
///
/// Measured on the binary of v1.1.37.02: the
/// `Gui, Test: Add, Button, x10 y10 w100 h30 +0x8000, Hello` gives a
/// button the style `0x5001A000` -- the `WS_CHILD`, the `WS_VISIBLE`, the
/// `WS_TABSTOP`, the `BS_FLAT` (0x8000) and the `BS_MULTILINE` (0x2000).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheStyleOfTheV1 {
    /// The bits that the options add.
    pub add: u32,
    /// The bits that the options take away.
    pub remove: u32,
    /// The state that the "Checked" of the options asks for: zero is
    /// unchecked, one is checked and two is the gray one.
    ///
    /// The v1 reads the "Checked" of the options of the `Gui, Add` (source
    /// `script_gui.cpp:4632-4662` of v1.1.37.02), and the word may have a
    /// number after it: the "Checked1" asks for the box to be checked and
    /// the "Checked0" asks for it to be unchecked.
    ///
    /// Measured on the binary of v1.1.37.02: the
    /// `Gui, Add, Checkbox, vCHK Checked, Check` makes a check box that is
    /// checked, and the `GuiControlGet, out, , CHK` of it answers "1".
    pub checked: Option<i64>,
    /// The number of the icon that the options of the control ask for.
    ///
    /// The v1: the "Icon" option of the `Gui, Add` names the icon of a
    /// file of icons -- an .ico, an .exe or a .dll -- that a picture
    /// control shows (source `script_gui.cpp:4632-4662` of v1.1.37.02).
    ///
    /// Measured on the binary of v1.1.37.02: the picture
    /// of the options "Icon1" is a control of the style `0x50000003` --
    /// the `SS_ICON` --, and the picture of a file that is not there is
    /// of the style `0x50000000`.
    pub icon: Option<u32>,
    /// The g-label callback of the control (`gMyLabel` in `Gui, Add`).
    pub glabel: String,
    /// The initial 1-based selected item/tab (`ChooseN` in `Gui, Add`).
    pub choose: Option<i64>,
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
    + Com
    + Sound
    + Machine
    + Dialogs
    + Network
    + Pictures
{
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WindowInfo {
    /// The identifier of the window, as this runtime names it.
    pub id: i64,
    /// The title of the window.
    pub title: String,
    /// The class of the window.
    pub class: String,
    /// Whether the window is the active one.
    pub active: bool,
    /// The name of the executable of the window, and the empty text when
    /// the desktop does not say.
    ///
    /// The name is the one of the file, without the folder of it: the
    /// `ahk_exe` criterion of the machine compares the name and not the
    /// path. Measured on the binaries of v1.1.37.02 and of v2.0.31 `ahk_exe python.exe` names the window whose
    /// executable is o `python.exe`, and `ahk_exe python` names none.
    pub exe: String,
    /// The process of the window, and zero when the desktop does not say.
    ///
    /// The `ahk_pid` criterion of the machine compares the number of the
    /// process with the number that the script gives.
    pub pid: i64,
    /// The texts of the text elements of the window, and the empty text
    /// when the desktop does not say.
    ///
    /// The text of a window is the text of the controls that have text,
    /// and the `WinText` of the v1 and of the v2 match one of them.
    /// Measured on the binary of v1.1.37.02: the
    /// `WinGetTitle` of a window with a control that has a text finds
    /// the window by the text of the control.
    ///
    /// The texts are separated by a line break.
    pub text: String,
}

impl WindowInfo {
    /// Whether one of the text elements of the window has the text that
    /// is wanted in it.
    ///
    /// The `WinText` is a piece of one text element of the window, and
    /// not of all of them: measured on the binary of v1.1.37.02 the text that goes across two elements does not find
    /// the window.
    pub fn has_the_text(&self, wanted: &str) -> bool {
        if wanted.is_empty() {
            return true;
        }
        self.text.split('\n').any(|element| element.contains(wanted))
    }
}

/// A janela que um script da familia das janelas procura.
///
/// O titulo, o texto de um elemento de texto da janela e as duas
/// exclusoes: e o que os comandos `WinGetTitle`, `WinActive`,
/// `WinExist` e `WinGetList` tem em comum, e o que o `WinGet` do v1
/// tem nas suas cinco posicoes.
///
/// Measured on the binaries of v1.1.37.02 and of v2.0.31 the exclusion by the title is a title and not a set of
/// criteria, and the exclusion by the text is a text of one text element
/// of the window.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheWindowWanted {
    /// Whether the title was written by the script.
    ///
    /// O titulo que nao esta escrito e a janela ativa -- e o titulo que
    /// esta escrito e o texto vazio nao acha janela nenhuma: medido no
    /// binario do v2.0.31, o `WinGetTitle("")` recusa com
    /// "Target window not found." e o `WinGetTitle()` da o titulo da
    /// janela ativa.
    pub asked: bool,
    /// The title of the window, with the criteria of the machine.
    pub title: String,
    /// The text of a text element of the window.
    pub text: String,
    /// The titles of the windows that do not answer.
    pub exclude_titles: Vec<String>,
    /// The texts of the windows that do not answer.
    pub exclude_texts: Vec<String>,
}

/// The title that the value names.
///
/// A number is the identifier of the window and not its title: measured
/// on the binary of v2.0.19, the `WinGetTitle(102)` is
/// the window that has the identifier 102.
fn the_title_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::Int(_)) => String::new(),
        Some(value) => value.to_text(),
        None => String::new(),
    }
}

impl TheWindowWanted {
    /// The target of the first parameter of a command of the family.
    ///
    /// The text and the first exclusion go in the places where the
    /// machine puts them, and the exclusions that come after are the two.
    pub fn of(values: &[Value], title_at: usize) -> TheWindowWanted {
        let wanted = TheWindowWanted {
            // Um numero e o identificador da janela e nao o titulo
            // dela: o `WinGetTitle(102)` e a janela que tem o
            // identificador 102.
            asked: matches!(
                values.get(title_at),
                Some(value) if !matches!(value, Value::Int(_))
            ),
            title: the_title_of(values.get(title_at)),
            text: values
                .get(title_at + 1)
                .map(|value| value.to_text())
                .unwrap_or_default(),
            exclude_titles: values
                .get(title_at + 2)
                .map(|value| vec![value.to_text()])
                .unwrap_or_default(),
            exclude_texts: values
                .get(title_at + 3)
                .map(|value| vec![value.to_text()])
                .unwrap_or_default(),
        };
        wanted.without_the_empty()
    }

    /// The target without the exclusions that are not written.
    pub fn without_the_empty(mut self) -> TheWindowWanted {
        self.exclude_titles.retain(|title| !title.is_empty());
        self.exclude_texts.retain(|text| !text.is_empty());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ProcessState {
    /// Whether the process has exited.
    pub has_exited: bool,
    /// The code it exited with, when it has exited.
    pub exit_code: Option<i32>,
    /// The moment it exited, as the text of the date and the time.
    pub exit_time: Option<String>,
}

/// The network of the desktop.
///
/// A download is a file that comes from the network, and the network is
/// the machine's: a desktop that does not reach it says so.
/// One mail message, validated by the dialect.
///
/// The shape is the one the `Mail` of KeySharp accepts: the recipients,
/// the subject, the body, the sender and the optional copies, replies,
/// attachments and headers of it, beside the server and the port to
/// send it through. The validation of all of it is the dialect's, and
/// the sending is the machine's.
pub struct MailMessage {
    /// The recipients of the message.
    pub to: Vec<String>,
    /// The copies.
    pub cc: Vec<String>,
    /// The blind copies.
    pub bcc: Vec<String>,
    /// The subject of the message.
    pub subject: String,
    /// The body of the message.
    pub body: String,
    /// The sender.
    pub from: String,
    /// The address the replies go to, when it is not the sender.
    pub reply_to: Option<String>,
    /// The paths of the attachments.
    pub attachments: Vec<String>,
    /// The headers beside the ones the message carries.
    pub headers: Vec<(String, String)>,
    /// The SMTP server.
    pub host: String,
    /// The port of the server, 25 when the host named none.
    pub port: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Drive {
    /// The kind of the drive: "Fixed", "Removable", "Network", "CDROM",
    /// "RAMDisk" or "Unknown".
    pub drive_type: String,
    /// The name of the format of the volume.
    pub filesystem: String,
    /// The label of the volume, and the empty text when it has none.
    pub label: String,
    /// The number that identifies the volume.
    pub serial: i64,
    /// The free space, in megabytes.
    pub free_megabytes: i64,
    /// The capacity, in megabytes.
    pub capacity_megabytes: i64,
    /// The state of the drive: "Ready" and the like.
    pub status: String,
}

impl Monitor {
    /// The number of the columns of the monitor.
    pub fn width(&self) -> i64 {
        self.right - self.left
    }

    /// The number of the rows of the monitor.
    pub fn height(&self) -> i64 {
        self.bottom - self.top
    }
}
