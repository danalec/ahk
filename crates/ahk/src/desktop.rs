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
                let pattern =
                    if ignore_case { alloc::format!("(?i){pattern}") } else { pattern.to_string() };
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
    fn attributes_of_the_pattern(&mut self, _pattern: &str, _mode: &str) -> Result<Vec<String>, String> {
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
    fn load_picture(&mut self, _filename: &str, _options: &PictureOptions) -> Result<Option<u32>, String> {
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
        Err("the download is made by the desktop, and this desktop does not reach the network.".to_string())
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
    fn dir_select(&mut self, _starting: &str, _options: i64, _prompt: &str) -> Result<String, String> {
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
