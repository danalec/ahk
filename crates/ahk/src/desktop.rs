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

    /// Whether a lock key is on: "CapsLock", "NumLock" or "ScrollLock".
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: a lock key is a key
    /// whose state is a switch, and not a key that is held: `SetCapsLockState`
    /// turns the switch, and the state of it is what `GetKeyState` answers for
    /// the mode "T".
    fn lock_is_on(&mut self, _lock: &str) -> Result<bool, String> {
        Err("this desktop cannot read the state of the lock keys".to_string())
    }

    /// Turn a lock key on or off.
    ///
    /// The desktop that can read the state of a lock key but not change it
    /// says so, and the state of the key is not touched.
    fn set_lock(&mut self, _lock: &str, _on: bool) -> Result<(), String> {
        Err("this desktop cannot change the state of the lock keys".to_string())
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
    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        Err("this desktop cannot tell how big the screen is".to_string())
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

    /// The monitors of the desktop.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: a monitor is a
    /// rectangle of the screen -- the right and the bottom are one past the
    /// last column and row, the way the rectangles of the system are -- and
    /// the monitors are counted from one, the way `MonitorGet` numbers them.
    /// A desktop that has one screen answers the one monitor of it; a
    /// desktop that cannot say says why.
    fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        Err("this desktop cannot say what monitors it has".to_string())
    }

    /// The place of the primary monitor, counted from one, among the
    /// monitors of the desktop.
    fn primary_monitor(&mut self) -> Result<usize, String> {
        Err("this desktop cannot say which monitor is the primary one".to_string())
    }

    /// The work area of a monitor: left, top, right and bottom.
    ///
    /// The work area is the part of the monitor that the panels and the
    /// taskbars leave. `None` is a monitor that is not there.
    fn monitor_work_area(&mut self, _monitor: usize) -> Result<Option<(i64, i64, i64, i64)>, String> {
        Err("this desktop cannot say the work area of its monitors".to_string())
    }
}

/// A monitor of the desktop: a rectangle of the screen, and the name of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the rectangle of
/// `MonitorGet` is left, top, right and bottom, and the right and the bottom
/// are one past the last column and the last row -- the rectangle of the
/// system, and not the inclusive corners a person counts. The name is the
/// name the system gives the monitor: "\\.\DISPLAY1" on the binary, and the
/// name of the output of a compositor on a Wayland session.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Monitor {
    /// The name of the monitor.
    ///
    /// The empty text is a monitor whose name the desktop cannot say.
    pub name: String,
    /// The left edge, in pixels.
    pub left: i64,
    /// The top edge, in pixels.
    pub top: i64,
    /// The right edge: one past the last column.
    pub right: i64,
    /// The bottom edge: one past the last row.
    pub bottom: i64,
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

    /// The windows of the session, in the order the desktop lists them.
    ///
    /// The id of a window is the identifier of this runtime, and not a handle
    /// of Windows: a script hands it back to this runtime, and a number that
    /// no window has is a window that is not there -- it is not a failure.
    /// The class of a window is the name the desktop gives it: the
    /// application id on Wayland, and the class of the window on X11.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetList` answers
    /// the ids of the windows as an array, `WinGetCount` their number, and
    /// the class of a window is the class name the system gives it.
    fn windows(&mut self) -> Result<Vec<WindowInfo>, String> {
        Err("this desktop cannot list the windows".to_string())
    }
    /// Whether a window is above the others, when the machine knows.
    ///
    /// `Ok(None)` is the answer of a machine that cannot say: it is not
    /// `false`, and the caller refuses with the reason. A question that
    /// cannot be answered does not answer `false` in this tree.
    fn always_on_top(&mut self, _window: &WindowInfo) -> Result<Option<bool>, String> {
        Err("this desktop cannot say whether a window is above the others".to_string())
    }

    /// Put a window above the others, or take it from there.
    ///
    /// The answer is a failure with the reason when the machine cannot do
    /// it: a change that did not happen is never reported as a change that
    /// did.
    fn set_always_on_top(&mut self, _window: &WindowInfo, _on: bool) -> Result<(), String> {
        Err("this desktop cannot put a window above the others".to_string())
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

    /// Whether the control of a window is enabled, when the machine can say.
    ///
    /// This is the state of a widget, and it needs the accessibility tree
    /// (AT-SPI). A machine without it cannot say.
    fn control_enabled(&mut self, _window: &WindowInfo) -> Result<Option<bool>, String> {
        Err("this desktop cannot say whether a control is enabled".to_string())
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
}

/// A window of the session, as the window family of v2 reads it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a window has an
/// identifier, a title and a class name, and the question about the active
/// window is answered by the same listing.
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
}

/// The processes of the desktop.
/// The state of a process.

///

/// The code and the moment are there when the process has exited; while it

/// runs, they are not.

#[derive(Debug, Clone)]

pub struct ProcessState {

    /// Whether the process has exited.

    pub has_exited: bool,

    /// The code it exited with, when it has exited.

    pub exit_code: Option<i32>,

    /// The moment it exited, as the text of the date and the time.

    pub exit_time: Option<String>,

}

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
    /// The state of a process, when the machine knows it.
    ///
    /// The answer is `None` when the machine does not know the process: it
    /// is not one of its children and it does not know it. The code and
    /// the moment are there when the process has exited, and they are not
    /// while it runs.

    /// Kill a process.
    ///
    /// The answer is a failure with the reason when the machine cannot do
    /// it: a process that is not killed is never reported as killed.
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
    // Measured on the binary of v2.0.19 on 2026-11-09: the name of a call is
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
    /// Measured on the binary of v2.0.19 on 2026-11-09: the answer is zero
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
    fn the_state_of_a_process(&mut self, _pid: u32) -> Result<Option<ProcessState>, String> {

        Err("this desktop cannot say the state of a process".to_string())

    }
    /// Kill a process.
    ///
    /// The answer is a failure with the reason when the machine cannot do
    /// it: a process that is not killed is never reported as killed.
    fn kill_a_process(&mut self, _pid: u32) -> Result<(), String> {
        Err("this desktop cannot kill a process".to_string())
    }

}

/// The files of the desktop.
pub trait Files {
    /// The directory the script is in.
    ///
    /// The name of the directory is a subfolder of the working directory
    /// when it is not an absolute path.
    /// Create a shortcut to a program, the way this platform makes one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `FileCreateShortcut`
    /// answers the empty text and makes the file, and it makes it even for a
    /// target that is not there.
    /// The full path to a file, the way the platform writes one.
    ///
    /// Measured on the documentation of KeySharp on 2026-11-11:
    /// `FileFullPath(Filename)` answers the full path to the file.
    fn full_path(&mut self, _path: &str) -> Result<String, String> {
        Err("this desktop cannot say the full path of a file".to_string())
    }

    /// Make an empty temporary file, and answer its full path.
    ///
    /// Measured on the documentation of KeySharp on 2026-11-11:
    /// `FileCreateTemp()` makes the file and answers where it is.
    fn create_temp_file(&mut self) -> Result<String, String> {
        Err("this desktop cannot make a temporary file".to_string())
    }
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

    /// Read a shortcut back: (target, dir, args, description, icon).
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `FileGetShortcut` of a
    /// shortcut answers all eight of its fields.
    fn read_shortcut(&mut self, _shortcut: &str) -> Result<(String, String, String, String, String), String> {
        Err("this desktop cannot read a shortcut".to_string())
    }

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

    /// Recycle a file or a folder, or the files that a pattern matches.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: a file that is there
    /// is recycled and the answer is nothing; a plain path that is not there
    /// is the failure "Failed"; and a pattern that matches nothing is not a
    /// failure. A desktop with no trash says so.
    fn recycle(&mut self, _pattern: &str) -> Result<(), String> {
        Err("this desktop has no trash to recycle a file into".to_string())
    }

    /// Empty the trash of the machine.
    ///
    /// The answer is nothing. A `drive` that is given names the trash to
    /// empty, and the deskop that has one trash for the whole machine
    /// empties it for any drive.
    fn empty_trash(&mut self, _drive: Option<&str>) -> Result<(), String> {
        Err("this desktop has no trash to empty".to_string())
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
    /// The path of the executable that is running the script.
    ///
    /// It is what `A_AhkPath` answers. A desktop that cannot tell says so,
    /// and the variable is then a refusal and not an empty text: a script that
    /// reads the path of the interpreter must be told that it is not known.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-11: `A_AhkPath` answers
    /// the path of the executable that runs the script.
    fn executable(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell the path of the interpreter".to_string())
    }

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
    fn idle_times(&mut self) -> Result<Idle, String> {
        Err("this desktop cannot say how long the session has been idle".to_string())
    }

    /// The IPv4 addresses of the machine.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `SysGetIPAddresses`
    /// answers an array of the addresses of the machine, one for each of
    /// them, and the address of the loopback is not one of them: the binary
    /// answered three addresses and none of them was 127.0.0.1.
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        Err("this desktop cannot say the addresses of the machine".to_string())
    }

    /// Write a text to the debugger of the machine.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `OutputDebug` writes
    /// the text to the debugger and answers nothing. A machine whose debugger
    /// is the standard error of the process writes it there.
    fn output_debug(&mut self, _text: &str) -> Result<(), String> {
        Err("this desktop has no debugger to write to".to_string())
    }

    /// The drive that a path is on, when the path is one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetType` of a
    /// drive that is not there answers the empty text, and `DriveGetStatus`
    /// of one answers "Invalid"; the other readings of a drive that is not
    /// there are the failure of the machine.
    fn drive(&mut self, _path: &str) -> Result<Option<Drive>, String> {
        Err("this desktop cannot say what a drive is".to_string())
    }

    /// The kind of the drive that a path is on.
    ///
    /// `None` is a path that is not a drive of this machine: the binary
    /// answers the empty text for it.
    fn drive_type(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot say what kind a drive is".to_string())
    }

    /// The state of the drive that a path is on.
    ///
    /// `None` is a path that is not a drive of this machine: the binary
    /// answers "Invalid" for it.
    fn drive_status(&mut self, _path: &str) -> Result<Option<String>, String> {
        Err("this desktop cannot say what state a drive is in".to_string())
    }
}

/// A drive of the machine, as the `DriveGet` family of v2 reads it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the filesystem is the
/// name of the format of the volume ("NTFS"), the label is the name of it,
/// the serial is a number, the free space and the capacity are counted in
/// megabytes, and the state of a drive that is there is "Ready".
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
