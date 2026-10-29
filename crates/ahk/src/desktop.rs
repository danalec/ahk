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

use crate::dll::{NativeCall, NativeResult};
use alloc::vec::Vec;
use alloc::string::{String, ToString};

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
    fn mouse_down(&mut self, _x: Option<i64>, _y: Option<i64>, _button: &str) -> Result<String, String> {
        Err("this desktop cannot hold a mouse button down".to_string())
    }

    /// Let a mouse button come up.
    fn mouse_up(&mut self, _x: Option<i64>, _y: Option<i64>, _button: &str) -> Result<String, String> {
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
    fn capture(&mut self, x: i64, y: i64, width: i64, height: i64, filename: &str) -> Result<String, String>;

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
}

/// The windows of the desktop.
pub trait Windows {
    /// Whether the window with this title is the active one.
    fn window_active(&mut self, _title: &str) -> Result<WindowAnswer, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }

    /// Whether a window with this title exists.
    fn window_exists(&mut self, _title: &str) -> Result<WindowAnswer, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// The title of the active window.
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }
}

/// The processes of the desktop.
pub trait Processes {
    /// Run a command; return its exit code, stdout and stderr.
    fn run(&mut self, command: &str, window: &str) -> Result<(i32, String, String), String>;

    /// The value of an environment variable.
    fn env_get(&mut self, _name: &str) -> Result<String, String> {
        Err("this desktop cannot read the environment".to_string())
    }

    /// Set an environment variable; answer what it was set to.
    fn env_set(&mut self, _name: &str, _value: &str) -> Result<String, String> {
        Err("this desktop cannot write the environment".to_string())
    }
}

/// The files of the desktop.
pub trait Files {
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
    /// gives the directories instead of the files.
    fn list_files(&mut self, _folder: &str, _pattern: &str, _mode: &str) -> Result<Vec<String>, String> {
        Err("this desktop cannot list the files of a directory".to_string())
    }

    /// The directory the script runs in.
    fn working_directory(&mut self) -> Result<String, String> {
        Err("this desktop cannot tell where the script runs".to_string())
    }

    /// The lines of a file.
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Err("this desktop cannot read a file".to_string())
    }

    /// Write a file, replacing what was in it; return what was written.
    fn write_file(&mut self, _path: &str, _text: &str) -> Result<String, String> {
        Err("this desktop cannot write a file".to_string())
    }

    /// Add to the end of a file; return what was added.
    fn append_file(&mut self, _path: &str, _text: &str) -> Result<String, String> {
        Err("this desktop cannot write a file".to_string())
    }

    /// Whether a file is there.
    fn file_exists(&mut self, _path: &str) -> Result<bool, String> {
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

    /// Whether a directory is there.
    fn dir_exists(&mut self, _path: &str) -> Result<bool, String> {
        Err("this desktop cannot look for a directory".to_string())
    }

    /// Take a directory away.
    fn dir_delete(&mut self, _path: &str) -> Result<String, String> {
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
    fn set_file_attributes(&mut self, _path: &str, _attributes: &[String]) -> Result<String, String> {
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
    fn ini_read(&mut self, _path: &str, _section: &str, _key: &str, _default: &str) -> Result<String, String> {
        Err("this desktop cannot read an ini file".to_string())
    }

    /// Write a key of an ini file.
    fn ini_write(&mut self, _path: &str, _section: &str, _key: &str, _value: &str) -> Result<String, String> {
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
    fn call_native(&mut self, _call: &mut NativeCall) -> Result<NativeResult, String> {
        Err("this desktop cannot make a native call.".to_string())
    }
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
{
}
