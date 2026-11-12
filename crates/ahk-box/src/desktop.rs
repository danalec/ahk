//! The lab box as a script target.
//!
//! A port of `src/ahk/box.py` from the archived Python implementation.
//!
//! Three primitives are not obvious, and all three were measured on the
//! lab box (COSMIC Wayland):
//!
//! * **Text** goes through the keyboard of our own, one key at a time,
//!   and the layout is the US one. A character the layout does not have
//!   fails the statement -- "the keyboard of this box has no key for
//!   it" -- rather than being typed as something else.
//! * **Keys** go through that same keyboard. `ydotool key` cannot be
//!   used: ydotool 1.x reads a key *name* as zero and does nothing, and
//!   a numeric code presses the key without ever releasing it, so the
//!   compositor's auto-repeat types the key forever.
//! * **Modifiers** do reach the client, as long as they go down before
//!   the key and come up after it: measured on 2026-10-25, ctrl+a
//!   selected the text of a GTK field and the next key replaced it. (The
//!   note that "a modifier never reaches the client" was measured with
//!   the `ydotool` device, whose `key 30:1` is ignored -- a property of
//!   that tool, not of the box.)
//!
//! ## The windows of the compositor
//!
//! The box is a Wayland session, and the compositor is the one that knows
//! the windows: `ext_foreign_toplevel_list_v1` lists them, and
//! `zcosmic_toplevel_info_v1` says which one has the focus. The client is
//! in `wayland.rs`, and the X11 path is kept for a session that is X11.
//!
//! The path that is not there any more is the one through D-Bus and X11:
//! the COSMIC services answer `Introspect` with nothing, and the X server
//! that runs beside it (the rootless Xwayland of gamescope) reports an
//! empty client list.
//!
//! What is answerable either way, and is what the focus check uses, is the
//! screen itself: the colour of a pixel, the place of a colour, and the
//! position of the pointer. Those three are read from the frame the
//! compositor draws, so they work for any window, Wayland or not.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ahk::desktop::{
    FileAnswer, FileOptions, Folders, Idle, InputAnswer, InputOptions, Processes, WindowAnswer,
};
use ahk::dll::{NativeCall, NativeFailure, NativeResult};
use ahk::dll::{Convention, NativeArg, NativeType, NativeValue};
use ahk::keys::{MODIFIERS, resolve_name};

use crate::keyboard::{ENUMERATION_WAIT, Keyboard, KeyboardError};
use crate::native::Libraries;
use crate::pointer::{PRESS_SETTLE, Pointer, PointerError};

/// A primitive could not be carried out.
#[derive(Debug)]
pub enum BoxError {
    Failed(String),
    Keyboard(KeyboardError),
    Pointer(PointerError),
}

impl std::fmt::Display for BoxError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxError::Failed(reason) => write!(formatter, "{reason}"),
            BoxError::Keyboard(reason) => write!(formatter, "the keyboard: {reason}"),
            BoxError::Pointer(reason) => write!(formatter, "the pointer: {reason}"),
        }
    }
}

impl std::error::Error for BoxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BoxError::Failed(_) => None,
            BoxError::Keyboard(reason) => Some(reason),
            BoxError::Pointer(reason) => Some(reason),
        }
    }
}

impl From<KeyboardError> for BoxError {
    fn from(reason: KeyboardError) -> BoxError {
        BoxError::Keyboard(reason)
    }
}

impl From<PointerError> for BoxError {
    fn from(reason: PointerError) -> BoxError {
        BoxError::Pointer(reason)
    }
}

/// The lab box, as the runtime sees it.
pub struct LabDesktop {
    keyboard: Option<Keyboard>,
    pointer: Option<Pointer>,
    /// The window of the script, when the session has one.
    ///
    /// It is made by the helper of the window and lives as long as
    /// the desktop does.
    gui: Option<crate::gui::TheWindow>,
    screen: Option<(i64, i64)>,
    notes: Vec<String>,
    /// The files that are open. A handle is the place of the file in
    /// this list, and a closed file is a hole in it.
    pub(crate) files: Vec<Option<std::fs::File>>,
    /// The directory where the run started.
    ///
    /// It is read when the desktop is made, and it is kept: a script that
    /// changes the working directory does not change this one.
    pub(crate) initial_working_dir: Option<String>,
    /// The pictures that are loaded. The number that identifies one is
    /// the place of the picture in this list.
    pub(crate) pictures: Vec<crate::pictures::Picture>,
    /// The libraries that are loaded. A library is loaded once and
    /// kept: a script that calls the same library twice does not load
    /// it twice.
    libraries: Libraries,
    /// The raven, when a library of Windows has been asked for. The
    /// raven is a process of its own, and it is started with the first
    /// call and lives until the desktop is dropped.
    ///
    /// The raven has two guests: one of the sixty-four bits and one of
    /// the thirty-two, and the second is only started when the library
    /// is not in the folder of the first.
    ///
    /// It is only here on the box: on Windows the machine is the
    /// Windows, and there is no raven to ask.
    #[cfg(target_os = "linux")]
    raven: Option<crate::raven::Guests>,
    /// The program of the tray of the session, and the pipes of it.
    ///
    /// It is started with the first icon of the script and lives as long as
    /// the desktop does. The answers of the helper are read on a thread of
    /// their own, so that the wait for one of them is bounded: a helper that
    /// stops answering is a failure with a reason, and not a hang.
    #[cfg(target_os = "linux")]
    tray: Option<(
        std::process::Child,
        std::process::ChildStdin,
        std::sync::mpsc::Receiver<String>,
    )>,
    /// The client of the compositor, when the session is Wayland. It is
    /// opened with the first question about the windows and kept: the
    /// connection to the compositor is worth keeping.
    #[cfg(target_os = "linux")]
    wayland: Option<crate::wayland::Client>,
    /// The program that draws tooltips, which is NOT the window of a `Gui`.
    ///
    /// `ToolTip` of v2 works with no window of the script at all, so the helper
    /// that draws it is a process of its own. Keeping it apart from `gui` is what
    /// lets a script show a tooltip and then make a `Gui` without being told
    /// that a window is already there.
    tooltip: Option<crate::gui::TheWindow>,
}

impl Default for LabDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl LabDesktop {
    // ---- the_x_ids_in
    /// The ids of a window list of the X server.
    ///
    /// Measured on the VM of the matrix on 2026-11-11: `xprop -root
    /// _NET_CLIENT_LIST` answers `_NET_CLIENT_LIST(WINDOW): window id #
    /// 0xa0000c, 0x80000c, 0xc0000c`, and `_NET_ACTIVE_WINDOW` answers one
    /// id -- or `window id # 0x0` when no window of the server has the
    /// focus, and the zero is not a window. The ids are the ones of the X
    /// server, and they are the ids the tree answers for the windows of an
    /// X11 session: they are stable while the window lives, which a place
    /// in the stacking order is not.
    fn the_x_ids_in(text: &str) -> Vec<i64> {
        let Some((_, after)) = text.split_once('#') else {
            return Vec::new();
        };
        after
            .split(',')
            .filter_map(|id| {
                let id = id.trim().trim_start_matches("0x");
                i64::from_str_radix(id, 16).ok().filter(|id| *id != 0)
            })
            .collect()
    }

    // ---- x11_windows
    /// The windows of an X11 session: the client list of the X server, the
    /// title of each window and the one that has the focus.
    fn x11_windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        let Some(display) = x_display() else {
            return Err(no_window_list_reason());
        };
        let listing = self.run(
            &format!("DISPLAY={display} xprop -root _NET_CLIENT_LIST 2>/dev/null || true"),
            "",
        )?;
        let ids = Self::the_x_ids_in(&listing.1);
        let active = self.run(
            &format!("DISPLAY={display} xprop -root _NET_ACTIVE_WINDOW 2>/dev/null || true"),
            "",
        )?;
        let active_id = Self::the_x_ids_in(&active.1).first().copied().unwrap_or(0);
        let mut windows = Vec::new();
        for id in ids {
            let output = self.run(
                &format!("DISPLAY={display} xprop -id {id} _NET_WM_NAME WM_NAME 2>/dev/null || true"),
                "",
            )?;
            let title = output
                .1
                .lines()
                .filter_map(|line| line.split_once(" = "))
                .map(|(_, value)| value.trim().trim_matches('"').to_string())
                .find(|value| !value.is_empty())
                .unwrap_or_default();
            windows.push(ahk::desktop::WindowInfo {
                id,
                title,
                class: String::new(),
                active: id == active_id,
            });
        }
        Ok(windows)
    }

    // ---- gui_title_is

    /// The long name of the device that the mixer chose for a kind.
    ///
    /// The mixer publishes the name of every device and a description that
    /// is longer; the description is the one that v2 would answer for the
    /// device of Windows.
    fn the_description_of(&mut self, kind: &str, which: &str) -> Result<String, String> {
        let chosen = self.pactl(&[which])?.trim().to_string();
        if chosen.is_empty() {
            return Ok(String::new());
        }
        let listing = self.pactl(&["list", kind])?;
        // The mixer lists the name of a device and its description, and the
        // description comes after the name: the answer is the description of
        // the device whose name is the one that was chosen.
        let mut matched = false;
        for line in listing.lines() {
            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            match key.trim() {
                "Name" => matched = value.trim() == chosen,
                "Description" if matched => {
                    return Ok(value.trim().to_string());
                }
                _ => {}
            }
        }
        Ok(chosen)
    }
    /// Whether the title names the window the script made.
    fn gui_title_is(&mut self, title: &str) -> bool {
        if title.is_empty() {
            return true;
        }
        let command = "{\"op\": \"title\"}".to_string();
        match self.the_gui_says(&command) {
            Ok(said) => the_text_in(&said, "\"title\": ")
                .map(|t| t.eq_ignore_ascii_case(title))
                .unwrap_or(false),
            Err(_) => false,
        }
    }

    // ---- own_control_text
    /// The text of a control of the script's own window.
    fn own_control_text(&mut self, control: &str) -> Result<ahk::desktop::ControlAnswer, String> {
        let number = match self.own_control_number(control) {
            Ok(number) => number,
            Err(_) => return Ok(ahk::desktop::ControlAnswer::NoSuchControl),
        };
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"value\", \"id\": {}}}",
            number
        ))?;
        Ok(ahk::desktop::ControlAnswer::Found(
            the_text_in(&said, "\"value\": ").unwrap_or_default(),
        ))
    }

    // ---- own_control_send_text
    /// Put text into a control of the script's own window, at its cursor.
    fn own_control_send_text(
        &mut self,
        control: &str,
        text: &str,
    ) -> Result<ahk::desktop::ControlEdit, String> {
        let number = match self.own_control_number(control) {
            Ok(number) => number,
            Err(_) => return Ok(ahk::desktop::ControlEdit::NoSuchControl),
        };
        let command = format!(
            "{{\"op\": \"paste\", \"id\": {}, \"text\": {}}}",
            number,
            quoted(text)
        );
        match self.the_gui_says(&command) {
            Ok(_) => Ok(ahk::desktop::ControlEdit::Done),
            Err(reason) if reason.contains("is not an edit") => {
                Ok(ahk::desktop::ControlEdit::NoSuchControl)
            }
            Err(reason) => Err(reason),
        }
    }

    // ---- own_control_number
    /// The number of a control of the script's own window, from its `ClassNN`.
    fn own_control_number(&mut self, control: &str) -> Result<i64, String> {
        let command = format!(
            "{{\"op\": \"resolve\", \"classnn\": {}}}",
            quoted(control)
        );
        let said = self.the_gui_says(&command)?;
        the_number_in(&said, "\"value\": ")
            .ok_or_else(|| format!("the window of the script has no control named {control}"))
    }

    /// A desktop that has not yet opened anything.
    pub fn new() -> Self {
        Self {
            tooltip: None,
            keyboard: None,
            pointer: None,
            gui: None,
            screen: None,
            notes: Vec::new(),
            files: Vec::new(),
            pictures: Vec::new(),
            initial_working_dir: std::env::current_dir()
                .ok()
                .map(|path| path.to_string_lossy().to_string()),
            libraries: Libraries::new(),
            #[cfg(target_os = "linux")]
            raven: None,
            #[cfg(target_os = "linux")]
            tray: None,
            #[cfg(target_os = "linux")]
            wayland: None,
        }
    }

    /// The notes so far: the places where the box was not what the script
    /// assumed.
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// The uinput keyboard, opened on first use.
    ///
    /// The device is created once and kept open: closing the fd destroys
    /// it, and the compositor needs a moment to enumerate a new one.
    fn keyboard(&mut self) -> Result<&mut Keyboard, BoxError> {
        if self.keyboard.is_none() {
            let codes = announced_codes();
            let keyboard = Keyboard::open(codes)?;
            std::thread::sleep(ENUMERATION_WAIT);
            self.notes
                .push("opened a uinput keyboard for the keys".to_string());
            self.keyboard = Some(keyboard);
        }
        Ok(self.keyboard.as_mut().expect("just opened"))
    }

    /// The absolute pointer, opened on first use.
    ///
    /// The screen size is needed before the device is created, because the
    /// absolute axes are told the range they cover.
    ///
    /// The wait after the creation is the same one the keyboard has: the
    /// compositor enumerates a new device through udev, and a click that
    /// arrives before the enumeration is a click that lands nowhere.
    /// Measured on the lab box: the click is delivered once the device is
    /// in the seat, and lost before that.
    fn pointer(&mut self) -> Result<&mut Pointer, BoxError> {
        if self.pointer.is_none() {
            let (width, height) = self.screen_size()?;
            let pointer = Pointer::open(width, height)?;
            std::thread::sleep(ENUMERATION_WAIT);
            self.notes.push(format!(
                "opened an absolute pointer for the mouse ({width}x{height})"
            ));
            self.pointer = Some(pointer);
        }
        Ok(self.pointer.as_mut().expect("just opened"))
    }

    /// The size of the screen, in pixels.
    ///
    /// It is read from a full-screen capture: the compositor draws the
    /// frame, and the frame knows how big it is. It is measured once and
    /// kept.
    pub fn screen_size(&mut self) -> Result<(i64, i64), BoxError> {
        if let Some(size) = self.screen {
            return Ok(size);
        }
        let path = PathBuf::from(format!("/tmp/ahk-size-{}.png", millis()));
        let command = format!("grim {}", shell_quote(&path.to_string_lossy()));
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| BoxError::Failed(error.to_string()))?;
        if !output.status.success() {
            return Err(BoxError::Failed(format!(
                "grim failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let size =
            image_size(&path).ok_or_else(|| BoxError::Failed("the capture has no size".into()))?;
        let _ = std::fs::remove_file(&path);
        self.screen = Some(size);
        Ok(size)
    }

    /// Pin the graphical session's environment, and say what it found.
    ///
    /// Over SSH the display variables are empty, and `grim` refuses to run
    /// without them ("failed to create display"). The Wayland socket is
    /// listed in the runtime directory of the user, and the directory is
    /// `/run/user/<uid>`, so both can be found without asking anybody.
    pub fn ensure_session_env(&mut self) {
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            return;
        }
        let Some(uid) = uid() else {
            self.notes
                .push("no uid on this platform; the session environment is unknown".to_string());
            return;
        };
        let runtime_dir = format!("/run/user/{uid}");
        unsafe { std::env::set_var("XDG_RUNTIME_DIR", &runtime_dir) };
        let Ok(entries) = std::fs::read_dir(&runtime_dir) else {
            self.notes.push(format!("cannot list {runtime_dir}"));
            return;
        };
        let mut found = false;
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("wayland-") {
                unsafe { std::env::set_var("WAYLAND_DISPLAY", &name) };
                self.notes.push(format!(
                    "WAYLAND_DISPLAY={name} (XDG_RUNTIME_DIR={runtime_dir})"
                ));
                found = true;
                break;
            }
        }
        if !found {
            self.notes
                .push(format!("no wayland socket in {runtime_dir}"));
        }
    }

    /// The call of a function of a library of Windows.
    ///
    /// The raven is the Windows of this box: it is a program that
    /// emulates one, and it runs the helper that does the call. It
    /// starts with the first call and lives until the desktop is
    /// dropped, because a program of its own is not cheap to start.
    #[cfg(target_os = "linux")]
    fn windows(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        if self.raven.is_none() {
            let paths = crate::raven::Paths::from_environment().ok_or_else(|| {
                NativeFailure::invalid(
                    "this box has no raven: the libraries of Windows cannot be called.",
                )
            })?;
            self.raven = Some(crate::raven::Guests::new(paths));
        }
        let guests = self.raven.as_mut().expect("just made");
        let answer = guests.call(call);
        // The notes of the guests: the raven that was started, and the
        // one of the thirty-two bits.
        for note in guests.take_notes() {
            self.notes.push(note);
        }
        answer
    }

    /// Send one command to the program that draws the windows of the script, and
    /// refuse when it says it did not do the thing.
    fn the_gui_says(&mut self, command: &str) -> Result<String, String> {
        let window = self
            .gui
            .as_mut()
            .ok_or("there is no window of the script")?;
        let answer = window.send(command)?;
        if !answer.contains("\"ok\": true") {
            return Err(format!("the window of the script refused: {answer}"));
        }
        Ok(answer)
    }

    /// A path of the Windows of the raven.
    ///
    /// The raven has a file system of its own, and the paths of it are
    /// the ones that its Windows says -- "C:\Windows" for the directory of
    /// the system, and "C:\Program Files" for the programs of the user.
    /// The block of the path is given to the callee and comes back with
    /// what the callee wrote in it.
    #[cfg(target_os = "linux")]
    fn a_path_of_the_windows(
        &mut self,
        library: &str,
        function: &str,
        arguments: Vec<NativeArg>,
        block: usize,
    ) -> Result<String, String> {
        let mut call = NativeCall {
            library: library.to_string(),
            function: function.to_string(),
            convention: Convention::Standard,
            returns: NativeType::Int,
            arguments,
        };
        self.windows(&mut call)
            .map_err(|failure| failure.reason)?;
        // The block of the argument is the path that came back.
        let text = match &call.arguments[block].value {
            NativeValue::Bytes(bytes) => {
                let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
                String::from_utf8_lossy(&bytes[..end]).into_owned()
            }
            _ => String::new(),
        };
        if text.is_empty() {
            return Err(format!("the raven said no path for {function}."));
        }
        Ok(text)
    }

    /// The directory of the Windows of the raven.
    #[cfg(target_os = "linux")]
    fn windows_directory(&mut self) -> Result<String, String> {
        self.a_path_of_the_windows(
            "kernel32",
            "GetWindowsDirectoryA",
            vec![
                NativeArg {
                    kind: NativeType::AStr,
                    address: true,
                    value: NativeValue::Bytes(vec![0u8; 260]),
                },
                NativeArg {
                    kind: NativeType::Int,
                    address: false,
                    value: NativeValue::Integer(260),
                },
            ],
            0,
        )
    }

    /// The directory of the programs of the user, in the Windows of the raven.    ///
    /// The shell of the Windows of the raven does not answer with the path
    /// of the folder of the programs (measured on the box on 2026-11-11:
    /// `SHGetFolderPathA` leaves the block empty), and the environment of
    /// the emulated system does: the program of the environment answers
    /// with the path of the folder of the programs.
    #[cfg(target_os = "linux")]
    fn program_files(&mut self) -> Result<String, String> {
        self.a_path_of_the_windows(
            "kernel32",
            "GetEnvironmentVariableA",
            vec![
                NativeArg {
                    kind: NativeType::AStr,
                    address: false,
                    value: NativeValue::Bytes(b"ProgramFiles\0".to_vec()),
                },
                NativeArg {
                    kind: NativeType::AStr,
                    address: true,
                    value: NativeValue::Bytes(vec![0u8; 260]),
                },
                NativeArg {
                    kind: NativeType::Int,
                    address: false,
                    value: NativeValue::Integer(260),
                },
            ],
            1,
        )
    }

    /// The icon of the script, in the tray of the session.
    ///
    /// The compositor of the box carries a watcher of the
    /// StatusNotifierItem, and the helper of the tray registers an item in
    /// it. The item lives as long as the helper does, and the helper lives
    /// as long as the desktop does.
    ///
    /// The answer of the helper is read and understood: the answer of the
    /// start is the answer of an item that the watcher holds, and a helper
    /// that did not register is a failure with the reason of the helper.
    #[cfg(target_os = "linux")]
    fn tray_set_icon(&mut self, name: &str) -> Result<(), String> {
        if self.tray.is_none() {
            let program = the_helper_of_the_tray()?;
            let mut child = Command::new("python3")
                .arg(&program)
                .env("AHK_EVENTS_FILE", the_events_of_the_window())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|error| format!("the tray of the session did not start: {error}"))?;
            let input = child.stdin.take().ok_or("the tray takes no input")?;
            let output = child.stdout.take().ok_or("the tray has no output")?;
            self.tray = Some((child, input, the_answers_of_the_tray(output)));
            let answer = self.the_answer_of_the_tray()?;
            the_tray_took(&answer)?;
        }
        let command = format!("{{\"op\": \"icon\", \"name\": {}}}\n", quoted(name));
        {
            let Some((_, input, _)) = self.tray.as_mut() else {
                return Err("the tray of the session is not there".to_string());
            };
            input
                .write_all(command.as_bytes())
                .map_err(|error| format!("the tray of the session did not take the icon: {error}"))?;
            input
                .flush()
                .map_err(|error| format!("the tray of the session did not take the icon: {error}"))?;
        }
        let answer = self.the_answer_of_the_tray()?;
        the_tray_took(&answer)
    }

    /// The answer of the helper of the tray, read with a bound.
    ///
    /// The wait is bounded: the helper of the tray is a program of the
    /// machine, and a program that stops answering is a failure with a
    /// reason and not a hang of the runtime.
    #[cfg(target_os = "linux")]
    fn the_answer_of_the_tray(&mut self) -> Result<String, String> {
        let Some((_, _, answers)) = self.tray.as_mut() else {
            return Err("the tray of the session is not there".to_string());
        };
        let answer = answers
            .recv_timeout(std::time::Duration::from_secs(5))
            .map_err(|_| "the tray of the session stopped answering".to_string())?;
        if answer.is_empty() {
            return Err("the tray of the session ended before it answered".to_string());
        }
        Ok(answer)
    }

}

/// The codes the device announces.
///
/// The list is the one the real keyboard of this box announces -- the
/// `AT Translated Set 2 keyboard`, read out of
/// `/proc/bus/input/devices` on 2026-10-25: 112 codes, from `KEY_ESC` to
/// `KEY_MENU`.
///
/// The list matters more than it looks. A device that announces codes a
/// real keyboard does not have -- the whole range up to 255 was tried --
/// makes libinput build a keymap it cannot use: the key events arrive,
/// libinput reports them with the key code `-1`, and a client that gets
/// a key with no code cannot use it. The keys arrive at the compositor
/// and stop there. With the real keyboard's list, the same events carry
/// a key code.
fn announced_codes() -> Vec<u16> {
    vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 70, 71, 72,
        73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 108, 109, 119, 120, 121, 122, 123, 124, 125, 126, 127, 130,
    ]
}

/// The uid of this process, when the platform has one.
fn uid() -> Option<u32> {
    #[cfg(unix)]
    {
        Some(crate::sys::getuid())
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// Quote a value for the shell.
fn shell_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '$' => out.push_str("\\$"),
            '`' => out.push_str("\\`"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// The pause between two readings of the table of the processes.
///
/// The table is read twenty times a second: a change is seen quickly, and the
/// loop costs the machine almost nothing while it waits.
fn pause_before_the_next_look() {
    std::thread::sleep(Duration::from_millis(20));
}

impl ahk::desktop::Desktop for LabDesktop {}

/// The command that reads the clipboard of an X11 session.
///
/// `xclip` is the one that is usually there, and `xsel` is the other.
/// Neither of them is on the box, and the reason is said when the
/// clipboard of an X11 session is asked.
const X11_CLIPBOARD_READ: &str = "if command -v xclip > /dev/null 2>&1; then xclip -selection clipboard -out; elif command -v xsel > /dev/null 2>&1; then xsel --clipboard --output; else echo 'neither xclip nor xsel is installed on this machine' >&2; exit 127; fi";

/// The command that puts a text on the clipboard of an X11 session.
const X11_CLIPBOARD_WRITE: &str = "if command -v xclip > /dev/null 2>&1; then xclip -selection clipboard; elif command -v xsel > /dev/null 2>&1; then xsel --clipboard --input; else echo 'neither xclip nor xsel is installed on this machine' >&2; exit 127; fi";

impl ahk::desktop::Clipboard for LabDesktop {
    fn clipboard_read(&mut self) -> Result<String, String> {
        // The session of the box is Wayland, and the compositor holds the
        // clipboard.
        if wayland_session() {
            return self.wayland_clipboard_read();
        }
        self.x11_clipboard_read()
    }

    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        if wayland_session() {
            return self.wayland_clipboard_write(text);
        }
        self.x11_clipboard_write(text)
    }
}

impl LabDesktop {
    /// The text on the clipboard of a Wayland session.
    ///
    /// `wl-paste` answers the text, and it says "Nothing is copied" when
    /// the clipboard is empty. The type is given, or the tool waits for
    /// ever for a type to arrive.
    fn wayland_clipboard_read(&mut self) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("wl-paste --no-newline --type text/plain")
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        // The empty clipboard is a value, and not a failure.
        if stderr.contains("Nothing is copied") {
            return Ok(String::new());
        }
        Err(format!("wl-paste failed: {stderr}"))
    }

    /// Put a text on the clipboard of a Wayland session.
    fn wayland_clipboard_write(&mut self, text: &str) -> Result<String, String> {
        self.copy_to_the_clipboard("wl-copy --type text/plain", text)
    }

    /// The text on the clipboard of an X11 session.
    ///
    /// An X11 selection that no application owns is an empty clipboard.
    fn x11_clipboard_read(&mut self) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(X11_CLIPBOARD_READ)
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("not available") {
            return Ok(String::new());
        }
        Err(format!(
            "the clipboard of an X11 session cannot be read: {stderr}"
        ))
    }

    /// Put a text on the clipboard of an X11 session.
    fn x11_clipboard_write(&mut self, text: &str) -> Result<String, String> {
        self.copy_to_the_clipboard(X11_CLIPBOARD_WRITE, text)
    }

    /// Put a text on the clipboard with the tool of the session.
    ///
    /// The tool serves the clipboard in the background, and the process it
    /// leaves behind holds the channel of the caller: the output is
    /// detached from it, or the caller waits for the end of the output for
    /// ever. The text goes in on the input, and the tool stops when the
    /// input ends.
    fn copy_to_the_clipboard(&mut self, command: &str, text: &str) -> Result<String, String> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| error.to_string())?;
        let mut input = child
            .stdin
            .take()
            .ok_or("the tool of the clipboard takes no input")?;
        // A tool that dies while the text goes in says why on its output,
        // and the status of it is the reason.
        let _ = input.write_all(text.as_bytes());
        drop(input);
        let status = child.wait().map_err(|error| error.to_string())?;
        if !status.success() {
            let mut stderr = String::new();
            if let Some(mut pipe) = child.stderr.take() {
                let _ = pipe.read_to_string(&mut stderr);
            }
            return Err(format!(
                "the text did not go on the clipboard: {}",
                stderr.trim()
            ));
        }
        Ok(format!("put {text:?} on the clipboard"))
    }
}

impl ahk::desktop::Timers for LabDesktop {}

impl ahk::desktop::Hotkeys for LabDesktop {
    fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
        crate::hotkeys::register(name)
    }
    fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {
        crate::hotkeys::unregister(name)
    }
    fn hotkeys(&mut self) -> Result<Vec<String>, String> {
        crate::hotkeys::list()
    }
    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
        crate::hotkeys::fired()
    }
}

impl ahk::desktop::Native for LabDesktop {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        // A library of this machine is called here, and a library of
        // Windows is called by the raven: the box has no Windows of
        // its own, and the raven is the Windows of the box.
        //
        // On Windows the machine is the Windows, and there is no raven
        // to ask: every library is called here.
        #[cfg(target_os = "linux")]
        if crate::raven::is_windows(&call.library) {
            return self.windows(call);
        }
        crate::native::call(&mut self.libraries, call)
    }
}

impl ahk::desktop::Keyboard for LabDesktop {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        if text.is_empty() {
            return Ok(String::new());
        }
        // The text goes through the keyboard of our own, one key at a
        // time. It used to go through `ydotool type`, which was measured
        // to be exact -- but the daemon behind it is not on this box any
        // more (`/usr/bin/ydotoald` is gone from the disk and the process
        // that was still running has died), and `ydotool type` without a
        // daemon writes into a socket nobody is listening on and reports
        // nothing. Our own keyboard is here, and it works.
        let mut typed = 0usize;
        for character in text.chars() {
            let (code, shift) = match character {
                '\n' => (28, false),
                '\t' => (15, false),
                other => ahk::keys::character_stroke(other)
                    .ok_or_else(|| format!("the keyboard of this box has no key for {other:?}"))?,
            };
            let modifiers: Vec<u16> = if shift { vec![42] } else { Vec::new() };
            self.keyboard()
                .map_err(|error| error.to_string())?
                .tap_with_modifiers(code, &modifiers)
                .map_err(|error| error.to_string())?;
            typed += 1;
        }
        Ok(format!(
            "typed {typed} characters through the uinput keyboard"
        ))
    }
    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        // The modifiers go down first and up last, around the key.
        //
        // They do arrive: measured on the lab box on 2026-10-25, with a
        // GTK client and a keyboard of our own, ctrl+a selected the text
        // of the field and the next key replaced it. (The note that
        // "a modifier never reaches the client" was measured with the
        // `ydotool` device, whose `key 30:1` is ignored -- a property
        // of that tool, not of the box.)
        let modifier_codes: Vec<u16> = modifiers
            .iter()
            .map(|modifier| resolve_name(modifier).map_err(|error| error.to_string()))
            .collect::<Result<Vec<u16>, String>>()?;
        if modifier_codes.is_empty() {
            self.keyboard()
                .map_err(|error| error.to_string())?
                .tap(code)
                .map_err(|error| error.to_string())?;
            return Ok(format!(
                "pressed {name} (code {code}) through the uinput keyboard"
            ));
        }
        self.keyboard()
            .map_err(|error| error.to_string())?
            .tap_with_modifiers(code, &modifier_codes)
            .map_err(|error| error.to_string())?;
        let names = modifiers.join("+");
        Ok(format!(
            "pressed {names}+{name} (code {code}) through the uinput keyboard"
        ))
    }
    fn key_down(&mut self, name: &str) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        self.keyboard()
            .map_err(|error| error.to_string())?
            .down(code)
            .map_err(|error| error.to_string())?;
        Ok(format!("{name} is down (code {code})"))
    }
    fn key_up(&mut self, name: &str) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        self.keyboard()
            .map_err(|error| error.to_string())?
            .up(code)
            .map_err(|error| error.to_string())?;
        Ok(format!("{name} is up (code {code})"))
    }
    fn key_is_down(&mut self, name: &str) -> Result<bool, String> {
        crate::hotkeys::key_is_down(name)
    }
    fn keys_down(&mut self) -> Result<Vec<String>, String> {
        crate::hotkeys::keys_down()
    }
    fn lock_is_on(&mut self, lock: &str) -> Result<bool, String> {
        crate::locks::is_on(lock)
    }
    fn set_lock(&mut self, lock: &str, on: bool) -> Result<(), String> {
        // The state of a lock key is the LED of the keyboard, and the key is
        // what turns it: measured on the box on 2026-11-09, the compositor
        // writes the LED when the key goes through it. The key is sent only
        // when the state is not the one that is wanted -- a keyboard that
        // sends the key it already is in would turn the state away from the
        // one that was asked for.
        let now = crate::locks::is_on(lock)?;
        if now == on {
            return Ok(());
        }
        self.press_key(lock, &[])?;
        // The compositor answers the key at the pace of its own frame, and
        // the LED is what says that it did: the wait is bounded, and a state
        // that did not arrive is the reason and not a silence.
        for _ in 0..50 {
            std::thread::sleep(std::time::Duration::from_millis(20));
            if crate::locks::is_on(lock)? == on {
                return Ok(());
            }
        }
        Err(format!(
            "the compositor did not change the state of the {lock}: the key was sent, and the LED \
             of the key is still the one it was."
        ))
    }
}

impl ahk::desktop::Pointer for LabDesktop {
    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
        // The absolute pointer is the path that lands where it is aimed:
        // ydotool's device is relative, so its click lands wherever the
        // pointer happened to be.
        let size = self.pointer().map_err(|error| error.to_string())?.size();
        let count = count.max(1);
        for index in 0..count {
            self.pointer()
                .map_err(|error| error.to_string())?
                .click(x, y, button)
                .map_err(|error| error.to_string())?;
            if index + 1 < count {
                // The gap between the two clicks of a double click.
                std::thread::sleep(Duration::from_millis(60));
            }
        }
        let times = if count == 1 {
            String::new()
        } else {
            format!(" {count} times")
        };
        Ok(format!(
            "clicked ({x}, {y}){times} through the absolute pointer (screen {width}x{height})",
            width = size.0,
            height = size.1
        ))
    }
    fn mouse_down(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        if let (Some(x), Some(y)) = (x, y) {
            self.pointer()
                .map_err(|error| error.to_string())?
                .move_to(x, y)
                .map_err(|error| error.to_string())?;
            std::thread::sleep(PRESS_SETTLE);
        }
        self.pointer()
            .map_err(|error| error.to_string())?
            .button_down(button)
            .map_err(|error| error.to_string())?;
        Ok(format!("the {button} button is down"))
    }
    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        if let (Some(x), Some(y)) = (x, y) {
            self.pointer()
                .map_err(|error| error.to_string())?
                .move_to(x, y)
                .map_err(|error| error.to_string())?;
            std::thread::sleep(PRESS_SETTLE);
        }
        self.pointer()
            .map_err(|error| error.to_string())?
            .button_up(button)
            .map_err(|error| error.to_string())?;
        Ok(format!("the {button} button is up"))
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.pointer()
            .map_err(|error| error.to_string())?
            .move_to(x, y)
            .map_err(|error| error.to_string())?;
        Ok(format!("moved the pointer to ({x}, {y})"))
    }
    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        // The pointer this runtime drives is an absolute device, so its
        // position can be read back from the kernel. The physical mouse on
        // this box is a relative device, so *its* position cannot be read
        // at all: the number below is the position of the pointer this
        // tool controls, which is the pointer the compositor is showing.
        let read = match self.pointer() {
            Ok(pointer) => pointer.position().map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        match read {
            Ok(position) => Ok(position),
            Err(error) => {
                // The kernel would not answer: the event node of the
                // device had not been opened up to the group yet. The
                // position the runtime put the pointer at is still the
                // truth about what it did, and the report says which of
                // the two this is.
                let last = self
                    .pointer()
                    .map(|pointer| pointer.last_position())
                    .unwrap_or((0, 0));
                self.notes.push(format!(
                    "the kernel did not say where the pointer is ({error}); this is the position \
                     the runtime put it at"
                ));
                Ok(last)
            }
        }
    }
    fn set_mouse_speed(&mut self, speed: i64) -> Result<String, String> {
        // The speed of v2 is the number of steps of a movement, and the
        // pointer of the box moves in steps.
        crate::pointer::set_speed(speed);
        Ok(format!("the speed of the pointer is {speed}"))
    }
    fn set_mouse_delay(&mut self, delay: i64) -> Result<String, String> {
        crate::pointer::set_delay(delay);
        Ok(format!("the delay of the pointer is {delay}ms"))
    }
}

impl ahk::desktop::Screen for LabDesktop {
    // ---- screen_scale
    fn screen_scale(&mut self) -> Result<i64, String> {
        // The scale is a question for the toolkit, which is the helper's, and it
        // needs no window: the session's scale is the monitor's.
        if self.gui.is_none() {
            let helper = the_helper_of_the_window()?;
            self.gui = Some(crate::gui::TheWindow::open(&helper)?);
        }
        let window = self.gui.as_mut().ok_or("the helper of the window did not start")?;
        let answer = window.send("{\"op\": \"scale\"}")?;
        the_number_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the session gave no scale: {answer}"))
    }

    // ---- tooltip_show
    fn tooltip_show(&mut self, text: &str, x: i64, y: i64) -> Result<i64, String> {
        let command = format!(
            "{{\"op\": \"tooltip\", \"text\": {}, \"x\": {}, \"y\": {}}}",
            quoted(text),
            x,
            y
        );
        // A tooltip needs no window of the script, so it has a helper of its own
        // and is started the first time one is shown. Measured on the binary of
        // v2.0.19 on 2026-11-10: showing one answers a number even with no Gui.
        if self.tooltip.is_none() {
            let helper = the_helper_of_the_window()?;
            self.tooltip = Some(crate::gui::TheWindow::open(&helper)?);
        }
        let window = self
            .tooltip
            .as_mut()
            .ok_or("the helper of the tooltip did not start")?;
        let said = window.send(&command)?;
        if !said.contains("\"ok\": true") {
            return Err(format!("the tooltip was not shown: {said}"));
        }
        the_number_in(&said, "\"value\": ").ok_or_else(|| "the tooltip gave no number".to_string())
    }

    // ---- window_move
    fn window_move(&mut self, x: i64, y: i64, width: i64, height: i64) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"window-move\", \"x\": {}, \"y\": {}, \"w\": {}, \"h\": {}}}",
            x, y, width, height
        );
        self.the_gui_says(&command).map(|_| ())
    }

    // ---- gui_controls
    fn gui_controls(&mut self) -> Result<String, String> {
        let answer = self.the_gui_says("{\"op\": \"controls\"}")?;
        Ok(the_text_in(&answer, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_all_text
    fn gui_all_text(&mut self) -> Result<String, String> {
        let answer = self.the_gui_says("{\"op\": \"all-text\"}")?;
        Ok(the_text_in(&answer, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_opacity
    fn gui_opacity(&mut self, setting: Option<i64>) -> Result<i64, String> {
        let command = match setting {
            Some(on) => format!("{{\"op\": \"opacity\", \"on\": {}}}", on),
            None => "{\"op\": \"opacity\"}".to_string(),
        };
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the window of the script gave no opacity: {answer}"))
    }

    // ---- gui_paste
    fn gui_paste(&mut self, number: u64, text: &str) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"paste\", \"id\": {}, \"text\": {}}}",
            number,
            quoted(text)
        );
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the text was not put into the control: {answer}"))
        }
    }

    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String> {
        let name = if filename.is_empty() {
            format!("ahk-{}.png", millis())
        } else {
            filename.to_string()
        };
        let path = PathBuf::from(&name);
        let geometry = format!("{x},{y} {width}x{height}");
        let command = format!(
            "grim -g {} {}",
            shell_quote(&geometry),
            shell_quote(&path.to_string_lossy())
        );
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() || !path.exists() {
            return Err(format!(
                "grim failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Ok(path.to_string_lossy().to_string())
    }
    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        LabDesktop::screen_size(self).map_err(|error| error.to_string())
    }
    fn monitors(&mut self) -> Result<Vec<ahk::desktop::Monitor>, String> {
        // The box is a Wayland session, and its compositor publishes one
        // output for each monitor. Measured on the box on 2026-11-09: the
        // compositor of COSMIC published the output HDMI-A-1 at 0,0 with the
        // mode 1920x1080.
        #[cfg(target_os = "linux")]
        {
            self.ensure_session_env();
            self.wayland()?.monitors()
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err("the monitors of this desktop are the outputs of a Wayland compositor, and this \
                 machine has none."
                .to_string())
        }
    }
    fn primary_monitor(&mut self) -> Result<usize, String> {
        // A Wayland compositor does not say which of its outputs is the
        // primary one. A desktop with one output has no choice, and the one
        // monitor of it is the primary one; a desktop with several says that
        // it does not know instead of guessing which one the panel is on.
        #[cfg(target_os = "linux")]
        {
            self.ensure_session_env();
            let monitors = self.wayland()?.monitors()?;
            match monitors.len() {
                0 => Err("the compositor published no monitor, so there is no primary one."
                    .to_string()),
                1 => Ok(1),
                count => Err(format!(
                    "the compositor published {count} monitors, and it does not say which one is \
                     the primary one."
                )),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err("the primary monitor of this desktop is one of the outputs of a Wayland \
                 compositor, and this machine has none."
                .to_string())
        }
    }
    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        let path = self.capture(x, y, 1, 1, &format!("ahk-pixel-{}.png", millis()))?;
        let colour = read_pixel(&path)?;
        let _ = std::fs::remove_file(&path);
        Ok(colour)
    }
    fn pixel_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        let target = parse_colour(colour)?;
        let (left, top) = (x1.min(x2), y1.min(y2));
        let (right, bottom) = (x1.max(x2), y1.max(y2));
        let width = right - left + 1;
        let height = bottom - top + 1;
        if width <= 0 || height <= 0 {
            return Ok(None);
        }
        let path = self.capture(
            left,
            top,
            width,
            height,
            &format!("ahk-search-{}.png", millis()),
        )?;
        let found = search_pixel(&path, target);
        let _ = std::fs::remove_file(&path);
        Ok(found.map(|(x, y)| (x + left, y + top)))
    }
}

impl ahk::desktop::Windows for LabDesktop {
    // ---- matching_windows
    fn matching_windows(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Vec<ahk::desktop::WindowRef>, String> {
        // The compositor of a Wayland session is the one that knows the
        // windows, and the listing of them is the protocol it publishes. The
        // X server beside it is the rootless Xwayland of gamescope, and it
        // reports an empty client list, so there is nothing to read there.
        if wayland_session() {
            return self.wayland_matching(title, mode);
        }
        // The client list of the X server answers the matching too, since
        // the list is parsed: a group that activates on an X11 session looks
        // its titles up in the same list the listing family reads. Measured
        // on the VM of the matrix on 2026-11-11, where `GroupActivate` of a
        // group with the title of an xterm answers the id of the xterm.
        let windows = self.x11_windows()?;
        let mut matched = Vec::new();
        for window in &windows {
            if title.is_empty() || title_matches(&window.title, title, mode)? {
                matched.push(ahk::desktop::WindowRef {
                    id: window.id.to_string(),
                    class: window.class.clone(),
                });
            }
        }
        Ok(matched)
    }

    // ---- window_geometry
    fn window_geometry(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<ahk::desktop::WindowGeometry>, String> {
        if wayland_session() {
            return self.wayland_geometry(title, mode);
        }
        // The place of a window of an X11 session is what `xwininfo` says of
        // it. Measured on the VM of the matrix on 2026-11-11: the absolute
        // upper-left corner and the size are the four numbers v2 answers,
        // and the window is the one of the client list -- the active one
        // when the question names nothing.
        let windows = self.x11_windows()?;
        let Some(id) = the_x_window_named(&windows, title, mode)? else {
            return Ok(None);
        };
        let Some(display) = x_display() else {
            return Err(no_window_list_reason());
        };
        let output = self.run(
            &format!("DISPLAY={display} xwininfo -id {id} 2>/dev/null || true"),
            "",
        )?;
        let mut x = None;
        let mut y = None;
        let mut width = None;
        let mut height = None;
        for line in output.1.lines() {
            let line = line.trim();
            let number = |prefix: &str| {
                line.strip_prefix(prefix)
                    .and_then(|rest| rest.trim().parse::<i64>().ok())
            };
            if let Some(value) = number("Absolute upper-left X:") {
                x = Some(value);
            }
            if let Some(value) = number("Absolute upper-left Y:") {
                y = Some(value);
            }
            if let Some(value) = number("Width:") {
                width = Some(value);
            }
            if let Some(value) = number("Height:") {
                height = Some(value);
            }
        }
        match (x, y, width, height) {
            (Some(x), Some(y), Some(width), Some(height)) => {
                Ok(Some(ahk::desktop::WindowGeometry {
                    x,
                    y,
                    width,
                    height,
                }))
            }
            // A window `xwininfo` does not know is a window whose place is
            // not known: the same answer a compositor gives when it does
            // not say the geometry.
            _ => Ok(None),
        }
    }

    // ---- window_min_max
    fn window_min_max(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<i64>, String> {
        if wayland_session() {
            return self.wayland_min_max(title, mode);
        }
        // The state of a window of an X11 session is what the properties of
        // the server say: `WM_STATE` says the iconic state, which is the
        // minimized one, and `_NET_WM_STATE` carries the two atoms of the
        // maximized state. Measured on the VM of the matrix on 2026-11-11.
        let windows = self.x11_windows()?;
        let Some(id) = the_x_window_named(&windows, title, mode)? else {
            return Ok(None);
        };
        let Some(display) = x_display() else {
            return Err(no_window_list_reason());
        };
        let output = self.run(
            &format!("DISPLAY={display} xprop -id {id} WM_STATE _NET_WM_STATE 2>/dev/null || true"),
            "",
        )?;
        let mut minimized = false;
        let mut maximized_vert = false;
        let mut maximized_horz = false;
        for line in output.1.lines() {
            let line = line.trim();
            if line == "Iconic State" {
                minimized = true;
            }
            if line.starts_with("_NET_WM_STATE(ATOM)") {
                maximized_vert = line.contains("_NET_WM_STATE_MAXIMIZED_VERT");
                maximized_horz = line.contains("_NET_WM_STATE_MAXIMIZED_HORZ");
            }
        }
        // A window that is both is answered as maximized, the way the
        // Wayland path answers it: the maximized window is the one the user
        // sees.
        if maximized_vert && maximized_horz {
            return Ok(Some(1));
        }
        if minimized {
            return Ok(Some(-1));
        }
        Ok(Some(0))
    }

    // ---- window_wait
    fn window_wait(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
        what: ahk::desktop::WaitFor,
        timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        if wayland_session() {
            return self.wayland_wait(title, mode, what, timeout_ms);
        }
        Err("this desktop waits for the windows of a Wayland session, and this session is not \
             Wayland"
            .to_string())
    }

    // ---- control_text
    fn control_text(
        &mut self,
        control: &str,
        title: &str,
    ) -> Result<ahk::desktop::ControlAnswer, String> {
        // The window the script made is known to the helper by the class and
        // number of each control. A foreign window needs the accessibility bus;
        // the script's own window does not, and asking the bus about it would
        // invent a second route to something already in hand (round 216).
        if self.gui_is_open() && self.gui_title_is(title) {
            return self.own_control_text(control);
        }
        // `busctl` is the tool of the accessibility bus, and the bus is a thing
        // of a Linux session.
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        crate::atspi::control_text(&address, title, control)
    }

    // ---- control_visible
    fn control_visible(
        &mut self,
        control: &str,
        title: &str,
    ) -> Result<ahk::desktop::ControlFlag, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        // Measured on the box from the header of the machine: the enum of the
        // states starts at zero, so `ATSPI_STATE_VISIBLE` is the bit 30.
        crate::atspi::control_flag(&address, title, control, 30)
    }

    // ---- control_enabled
    fn control_enabled(
        &mut self,
        control: &str,
        title: &str,
    ) -> Result<ahk::desktop::ControlFlag, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        // `ATSPI_STATE_ENABLED` is the bit 8.
        crate::atspi::control_flag(&address, title, control, 8)
    }

    // ---- control_set_text
    fn control_set_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ahk::desktop::ControlEdit, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        crate::atspi::control_set_text(&address, title, control, text)
    }

    // ---- control_click
    fn control_click(
        &mut self,
        control: &str,
        title: &str,
    ) -> Result<ahk::desktop::ControlEdit, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        crate::atspi::control_click(&address, title, control)
    }

    // ---- control_focus
    fn control_focus(
        &mut self,
        control: &str,
        title: &str,
    ) -> Result<ahk::desktop::ControlEdit, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        crate::atspi::control_focus(&address, title, control)
    }

    // ---- control_send_text
    fn control_send_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ahk::desktop::ControlEdit, String> {
        if self.gui_is_open() && self.gui_title_is(title) {
            return self.own_control_send_text(control, text);
        }
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        // The focus first, through the accessibility layer, and then the keys
        // through the keyboard of our own: the two capabilities together.
        match crate::atspi::control_focus(&address, title, control)? {
            ahk::desktop::ControlEdit::Done => {}
            other => return Ok(other),
        }
        ahk::desktop::Keyboard::type_text(self, text)?;
        Ok(ahk::desktop::ControlEdit::Done)
    }

    // ---- window_pid
    fn window_pid(&mut self, title: &str) -> Result<Option<i64>, String> {
        box_tools_are_near()?;
        let address = crate::atspi::address()?;
        crate::atspi::app_pid(&address, title)
    }

    fn window_active(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        // The session of the box is Wayland, and the compositor is the one
        // that knows the windows. The X server beside it is the rootless
        // Xwayland of gamescope, and it sees nothing.
        if wayland_session() {
            return self.wayland_active(title, mode);
        }

        let Some(active) = self.active_window_title()? else {
            return Ok(WindowAnswer::Unknown(
                "no X11 window is active, and a native Wayland window that holds the focus is \
                 invisible to X11. Use PixelSearch or MouseGetPos with PixelGet to check the \
                 focus."
                    .to_string(),
            ));
        };
        if title_matches(&active, title, mode)? {
            return Ok(WindowAnswer::Active);
        }
        if wayland_session() {
            Ok(WindowAnswer::Unknown(format!(
                "the X11 server sees the active X11 window as {active:?}, but this session is \
                 Wayland: a native Wayland window can hold the focus without appearing in X11. \
                 Use PixelSearch or MouseGetPos with PixelGet to check the focus."
            )))
        } else {
            Ok(WindowAnswer::NotActive)
        }
    }
    fn window_exists(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        // The session of the box is Wayland, and the compositor is the one
        // that knows the windows.
        if wayland_session() {
            return self.wayland_exists(title, mode);
        }

        // A host that is not Linux has neither the compositor nor an X
        // server to ask, and it says that instead of running the X tools the
        // box has.
        if !cfg!(target_os = "linux") && x_display().is_none() {
            return Ok(WindowAnswer::Unknown(no_window_list_reason()));
        }

        // The client list of the X server is the listing of an X11 session.
        // Measured on the VM of the matrix on 2026-11-11: i3 answers the ids
        // of the windows of the session in `_NET_CLIENT_LIST`, and
        // `_NET_ACTIVE_WINDOW` says which one has the focus -- so a window
        // that is there is an answer, and a window that is not there is
        // absent, the same two answers the compositor gives.
        let windows = self.x11_windows()?;
        if title.is_empty() {
            return Ok(empty_title());
        }
        for window in &windows {
            if title_matches(&window.title, title, mode)? {
                return Ok(if window.active {
                    WindowAnswer::Active
                } else {
                    WindowAnswer::NotActive
                });
            }
        }
        Ok(WindowAnswer::Absent)
    }
    fn windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        // The windows of the box are the ones of the compositor: the session
        // is Wayland, and the X server beside it is the rootless Xwayland of
        // gamescope, which reports an empty client list.
        if wayland_session() {
            return self.wayland_windows();
        }
        // An X11 session lists its windows through the client list of the X
        // server, and the list is parsed since 2026-11-11: before that the
        // tree read the list and answered that it does not parse it, which
        // the VM of the matrix showed to be a gap and not a design.
        self.x11_windows()
    }
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        // The session of the box is Wayland, and the compositor is the one
        // that knows which window has the focus.
        if wayland_session() {
            return self.wayland_active_title();
        }

        let display = x_display();
        let Some(display) = display else {
            return Err(no_window_list_reason());
        };
        let command =
            format!("DISPLAY={display} xprop -root _NET_ACTIVE_WINDOW 2>/dev/null || true");
        let output = self.run(&command, "")?;
        let text = output.1.trim().to_string();
        if text.is_empty() {
            return Ok(None);
        }
        let Some(id) = text
            .rsplit('#')
            .next()
            .map(str::trim)
            .filter(|id| !id.is_empty())
        else {
            return Ok(None);
        };
        if id == "0x0" {
            return Ok(None);
        }
        let command =
            format!("DISPLAY={display} xprop -id {id} _NET_WM_NAME WM_NAME 2>/dev/null || true");
        let output = self.run(&command, "")?;
        for line in output.1.lines() {
            if let Some((_, value)) = line.split_once(" = ") {
                let value = value.trim().trim_matches('"');
                if !value.is_empty() {
                    return Ok(Some(value.to_string()));
                }
            }
        }
        Ok(None)
    }
    // ---- gui_is_open
    fn gui_is_open(&mut self) -> bool {
        self.gui.is_some()
    }

    // ---- gui_open
    fn gui_open(&mut self, title: &str) -> Result<u64, String> {
        box_tools_are_near()?;
        // One window at a time. The helper replaces the window it holds when it
        // is told to make another, so a second one would silently take the place
        // of the first -- and a script that made two `Gui`s would find one
        // window with no word about it. It fails with the reason instead, which
        // is the rule of this tree; a window that was taken away frees the place
        // again, so `Destroy` and then a new `Gui` works.
        if self.gui.is_some() {
            return Err(
                "this desktop makes one window at a time, and a window of the script is \
                 already there."
                    .to_string(),
            );
        }
        let helper = the_helper_of_the_window()?;
        self.gui = Some(crate::gui::TheWindow::open(&helper)?);
        let window = self.gui.as_mut().ok_or("the helper of the window did not start")?;
        let answer = window.send(&format!(
            "{{\"op\": \"window\", \"title\": {}}}",
            quoted(title)
        ))?;
        if !answer.contains("\"ok\": true") {
            return Err(format!("the window of the script was not made: {answer}"));
        }
        // The events of that window go where `gui_events` will look for them.
        let watch = format!(
            "{{\"op\": \"watch\", \"path\": {}}}",
            quoted(&the_events_of_the_window().to_string_lossy())
        );
        let window = self
            .gui
            .as_mut()
            .ok_or("the helper of the window did not start")?;
        let _ = window.send(&watch);
        Ok(1)
    }

    // ---- gui_add
    fn gui_add(&mut self, _handle: u64, kind: &str, text: &str) -> Result<(u64, String), String> {
        let window = self.gui.as_mut().ok_or("there is no window of the script")?;
        let answer = window.send(&format!(
            "{{\"op\": \"add\", \"kind\": {}, \"text\": {}}}",
            quoted(kind),
            quoted(text)
        ))?;
        let id = the_number_in(&answer, "\"id\": ")
            .ok_or_else(|| format!("the control was not added: {answer}"))?;
        let classnn = the_text_in(&answer, "\"classnn\": ")
            .ok_or_else(|| format!("the control was not added: {answer}"))?;
        Ok((id as u64, classnn))
    }

    // ---- gui_combo_items
    fn gui_combo_items(&mut self, number: u64) -> Result<String, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-items\", \"id\": {}}}",
            number
        ))?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_combo_add
    fn gui_combo_add(&mut self, number: u64, text: &str) -> Result<(), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-add\", \"id\": {}, \"text\": {}}}",
            number,
            quoted(text)
        ))?;
        if said.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the item was not added: {said}"))
        }
    }

    // ---- gui_combo_select
    fn gui_combo_select(&mut self, number: u64, wanted: &str, by_text: bool) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-select\", \"id\": {}, \"wanted\": {}, \"by_text\": {}}}",
            number,
            quoted(wanted),
            by_text
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| {
            format!("the item was not chosen: {said}")
        })
    }

    // ---- gui_combo_choice
    fn gui_combo_choice(&mut self, number: u64) -> Result<(i64, String), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-choice\", \"id\": {}}}",
            number
        ))?;
        let index = the_number_in(&said, "\"index\": ").unwrap_or(0);
        let text = the_text_in(&said, "\"text\": ").unwrap_or_default();
        Ok((index, text))
    }

    // ---- gui_combo_delete
    fn gui_combo_delete(&mut self, number: u64, index: &str) -> Result<(), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-delete\", \"id\": {}, \"wanted\": {}}}",
            number,
            quoted(index)
        ))?;
        if said.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the item was not removed: {said}"))
        }
    }

    // ---- gui_combo_find
    fn gui_combo_find(&mut self, number: u64, text: &str) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-find\", \"id\": {}, \"text\": {}}}",
            number,
            quoted(text)
        ))?;
        the_number_in(&said, "\"value\": ")
            .ok_or_else(|| format!("the item was not found: {said}"))
    }

    // ---- gui_combo_dropdown
    fn gui_combo_dropdown(&mut self, number: u64, show: bool) -> Result<(), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"combo-dropdown\", \"id\": {}, \"show\": {}}}",
            number, show
        ))?;
        if said.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the pop-up was not changed: {said}"))
        }
    }

    // ---- gui_status_text
    fn gui_status_text(&mut self) -> Result<String, String> {
        let said = self.the_gui_says("{\"op\": \"status-text\"}")?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_list_add
    fn gui_list_add(&mut self, number: u64, cells: &[String]) -> Result<i64, String> {
        let list = cells
            .iter()
            .map(|cell| quoted(cell))
            .collect::<Vec<_>>()
            .join(", ");
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-add\", \"id\": {}, \"cells\": [{}]}}",
            number, list
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the row was not added: {said}"))
    }

    // ---- gui_list_count
    fn gui_list_count(&mut self, number: u64) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-count\", \"id\": {}}}",
            number
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the rows were not counted: {said}"))
    }

    // ---- gui_list_text
    fn gui_list_text(&mut self, number: u64, row: i64, col: i64) -> Result<String, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-text\", \"id\": {}, \"row\": {}, \"col\": {}}}",
            number, row, col
        ))?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_list_content
    fn gui_list_content(&mut self, number: u64, selected: bool) -> Result<String, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-content\", \"id\": {}, \"selected\": {}}}",
            number, selected
        ))?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_list_modify
    fn gui_list_modify(&mut self, number: u64, row: i64, cells: &[String]) -> Result<i64, String> {
        let list = cells
            .iter()
            .map(|cell| quoted(cell))
            .collect::<Vec<_>>()
            .join(", ");
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-modify\", \"id\": {}, \"row\": {}, \"cells\": [{}]}}",
            number, row, list
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the row was not changed: {said}"))
    }

    // ---- gui_list_next
    fn gui_list_next(&mut self, number: u64, start: i64) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"list-next\", \"id\": {}, \"start\": {}}}",
            number, start
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the next row was not read: {said}"))
    }

    // ---- gui_tree_add
    fn gui_tree_add(&mut self, number: u64, text: &str, parent: i64, options: &str) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-add\", \"id\": {}, \"text\": {}, \"parent\": {}, \"options\": {}}}",
            number,
            quoted(text),
            parent,
            quoted(options)
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the item was not added: {said}"))
    }

    // ---- gui_tree_count
    fn gui_tree_count(&mut self, number: u64) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-count\", \"id\": {}}}",
            number
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the items were not counted: {said}"))
    }

    // ---- gui_tree_text
    fn gui_tree_text(&mut self, number: u64, item: i64) -> Result<String, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-text\", \"id\": {}, \"item\": {}}}",
            number, item
        ))?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_tree_relation
    fn gui_tree_relation(&mut self, number: u64, item: i64, relation: &str) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-relation\", \"id\": {}, \"item\": {}, \"relation\": {}}}",
            number,
            item,
            quoted(relation)
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the tree was not walked: {said}"))
    }

    // ---- gui_tree_selection
    fn gui_tree_selection(&mut self, number: u64) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-selection\", \"id\": {}}}",
            number
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the selection was not read: {said}"))
    }

    // ---- gui_tree_modify
    fn gui_tree_modify(&mut self, number: u64, item: i64, options: &str) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-modify\", \"id\": {}, \"item\": {}, \"options\": {}}}",
            number,
            item,
            quoted(options)
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the item was not changed: {said}"))
    }

    // ---- gui_tree_delete
    fn gui_tree_delete(&mut self, number: u64, item: i64) -> Result<i64, String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"tree-delete\", \"id\": {}, \"item\": {}}}",
            number, item
        ))?;
        the_number_in(&said, "\"value\": ").ok_or_else(|| format!("the item was not deleted: {said}"))
    }

    // ---- gui_second_value
    fn gui_second_value(&mut self, number: u64, setting: Option<&str>) -> Result<String, String> {
        let command = match setting {
            Some(text) => format!(
                "{{\"op\": \"second-value\", \"id\": {}, \"text\": {}}}",
                number,
                quoted(text)
            ),
            None => format!("{{\"op\": \"second-value\", \"id\": {}}}", number),
        };
        let said = self.the_gui_says(&command)?;
        Ok(the_text_in(&said, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_control_pos
    fn gui_control_pos(&mut self, number: u64) -> Result<(i64, i64, i64, i64), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"control-pos\", \"id\": {}}}",
            number
        ))?;
        let get = |name: &str| {
            the_number_in(&said, name)
                .ok_or_else(|| format!("the place of the control was not read: {said}"))
        };
        Ok((get("\"x\": ")?, get("\"y\": ")?, get("\"w\": ")?, get("\"h\": ")?))
    }

    // ---- gui_menu_bar
    fn gui_menu_bar(&mut self, json: &str) -> Result<(), String> {
        let said = self.the_gui_says(&format!(
            "{{\"op\": \"menu-bar\", \"items\": {}}}",
            json
        ))?;
        if said.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the menu bar was not shown: {said}"))
        }
    }

    // ---- gui_caret
    fn gui_caret(&mut self) -> Result<Option<(i64, i64)>, String> {
        let said = self.the_gui_says("{\"op\": \"caret\"}")?;
        // "value": null is no caret; "value": [x, y] is the place of it.
        if said.contains("\"value\": null") {
            return Ok(None);
        }
        let get = |name: &str| {
            the_number_in(&said, name)
                .ok_or_else(|| format!("the place of the caret was not read: {said}"))
        };
        Ok(Some((get("\"x\": ")?, get("\"y\": ")?)))
    }

    // ---- gui_show
    fn gui_show(&mut self, _handle: u64) -> Result<(), String> {
        self.the_gui_says("{\"op\": \"show\"}").map(|_| ())
    }

    // ---- gui_destroy
    fn gui_destroy(&mut self, _handle: u64) -> Result<(), String> {
        self.the_gui_says("{\"op\": \"destroy\"}").map(|_| ())?;
        // The helper goes with the window: it has nothing left to hold, and
        // letting it go frees the one place a window of the script can have.
        self.gui = None;
        Ok(())
    }

    // ---- gui_value
    fn gui_value(&mut self, number: u64, setting: Option<&str>) -> Result<String, String> {
        let command = match setting {
            Some(text) => format!(
                "{{\"op\": \"value\", \"id\": {}, \"text\": {}}}",
                number,
                quoted(text)
            ),
            None => format!("{{\"op\": \"value\", \"id\": {}}}", number),
        };
        let answer = self.the_gui_says(&command)?;
        the_text_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the control of the script gave no value: {answer}"))
    }

    // ---- gui_flag
    fn gui_flag(
        &mut self,
        number: u64,
        name: &str,
        setting: Option<bool>,
    ) -> Result<i64, String> {
        let command = match setting {
            Some(on) => format!(
                "{{\"op\": \"flag\", \"id\": {}, \"name\": {}, \"on\": {}}}",
                number,
                quoted(name),
                if on { "true" } else { "false" }
            ),
            None => format!(
                "{{\"op\": \"flag\", \"id\": {}, \"name\": {}}}",
                number,
                quoted(name)
            ),
        };
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the control of the script gave no flag: {answer}"))
    }

    // ---- gui_control_state
    fn gui_control_state(&mut self, classnn: &str, what: &str, on: bool) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"state\", \"classnn\": {}, \"what\": {}, \"on\": {}}}",
            quoted(classnn),
            quoted(what),
            if on { "true" } else { "false" }
        );
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the state of the control was not set: {answer}"))
        }
    }

    // ---- gui_visibility
    fn gui_visibility(&mut self, classnn: &str, on: bool) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"visibility\", \"classnn\": {}, \"on\": {}}}",
            quoted(classnn),
            if on { "true" } else { "false" }
        );
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the control was not shown or hidden: {answer}"))
        }
    }

    // ---- gui_number_named
    fn gui_number_named(&mut self, classnn: &str) -> Result<i64, String> {
        let command = format!(
            "{{\"op\": \"resolve\", \"classnn\": {}}}",
            quoted(classnn)
        );
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ").ok_or_else(|| {
            format!("the window of the script has no control named {classnn}")
        })
    }

    // ---- gui_focused
    fn gui_focused(&mut self) -> Result<String, String> {
        let answer = self.the_gui_says("{\"op\": \"focused\"}")?;
        Ok(the_text_in(&answer, "\"value\": ").unwrap_or_default())
    }

    // ---- gui_selection
    fn gui_selection(&mut self, number: u64) -> Result<String, String> {
        let command = format!("{{\"op\": \"selection\", \"id\": {}}}", number);
        let answer = self.the_gui_says(&command)?;
        the_text_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the control of the script gave no selection: {answer}"))
    }

    // ---- gui_cursor
    fn gui_cursor(&mut self, number: u64) -> Result<i64, String> {
        let command = format!("{{\"op\": \"cursor\", \"id\": {}}}", number);
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the control of the script gave no cursor: {answer}"))
    }

    // ---- gui_line_count
    fn gui_line_count(&mut self, number: u64) -> Result<i64, String> {
        let command = format!("{{\"op\": \"lines\", \"id\": {}}}", number);
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ").ok_or_else(|| {
            format!("the control of the script gave no line count: {answer}")
        })
    }

    // ---- gui_option
    fn gui_option(&mut self, name: &str, on: bool) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"option\", \"name\": {}, \"on\": {}}}",
            quoted(name),
            if on { "true" } else { "false" }
        );
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!(
                "the option {name} of the window was not set: {answer}"
            ))
        }
    }

    // ---- gui_hide
    fn gui_hide(&mut self) -> Result<(), String> {
        let answer = self.the_gui_says("{\"op\": \"hide\"}")?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the window of the script was not hidden: {answer}"))
        }
    }

    // ---- gui_focus
    fn gui_focus(&mut self, number: u64) -> Result<(), String> {
        let command = format!("{{\"op\": \"focus\", \"id\": {}}}", number);
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the focus did not move to the control: {answer}"))
        }
    }

    // ---- gui_size
    fn gui_size(&mut self, number: u64, width: i64, height: i64) -> Result<(), String> {
        let command = format!(
            "{{\"op\": \"size\", \"id\": {}, \"w\": {}, \"h\": {}}}",
            number, width, height
        );
        let answer = self.the_gui_says(&command)?;
        if answer.contains("\"ok\": true") {
            Ok(())
        } else {
            Err(format!("the control of the script was not sized: {answer}"))
        }
    }

    // ---- gui_events
    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        // The file the helper was told to write to. Its name is derived from the
        // process, so that two runners on one machine do not read each other's
        // events, and nothing has to be kept: both ends compute it.
        let path = the_events_of_the_window();
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            // No file is no events, and that is the common case.
            Err(_) => return Ok(Vec::new()),
        };
        // Taken and cleared: what was read is not read again.
        let _ = std::fs::write(&path, "");
        Ok(text
            .lines()
            .map(str::to_string)
            .filter(|line| !line.is_empty())
            .collect())
    }

    // ---- gui_title
    fn gui_title(&mut self, _handle: u64, setting: Option<&str>) -> Result<String, String> {
        let command = match setting {
            Some(title) => format!("{{\"op\": \"title\", \"set\": {}}}", quoted(title)),
            None => "{\"op\": \"title\"}".to_string(),
        };
        let answer = self.the_gui_says(&command)?;
        the_text_in(&answer, "\"title\": ")
            .ok_or_else(|| format!("the window of the script gave no title: {answer}"))
    }

    // ---- gui_checked
    fn gui_checked(&mut self, number: u64, setting: Option<bool>) -> Result<i64, String> {
        let command = match setting {
            Some(on) => format!(
                "{{\"op\": \"checked\", \"id\": {}, \"on\": {}}}",
                number,
                if on { "true" } else { "false" }
            ),
            None => format!("{{\"op\": \"checked\", \"id\": {}}}", number),
        };
        let answer = self.the_gui_says(&command)?;
        the_number_in(&answer, "\"value\": ")
            .ok_or_else(|| format!("the check box of the script gave no tick: {answer}"))
    }

}

impl ahk::desktop::Sound for LabDesktop {

    fn sound_device_named(&mut self, device: &str) -> Result<Option<String>, String> {
        for kind in ["sinks", "sources"] {
            let listing = self.pactl(&["list", kind])?;
            // The description of a device comes after its name.
            let mut matched = false;
            for line in listing.lines() {
                let Some((key, value)) = line.split_once(':') else {
                    continue;
                };
                match key.trim() {
                    "Name" => matched = value.trim() == device,
                    "Description" if matched => {
                        return Ok(Some(value.trim().to_string()));
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    fn sound_device_names(&mut self) -> Result<(String, String), String> {
        // The mixer of the session knows the device that plays and the one
        // that records, and the name it publishes for them is the long one
        // -- "Radeon High Definition Audio Controller Digital Stereo (HDMI)"
        // -- the way v2 names the device of Windows.
        let plays = self.the_description_of("sinks", "get-default-sink")?;
        let records = self.the_description_of("sources", "get-default-source")?;
        Ok((plays, records))
    }
    fn sound_volume(&mut self, device: &str) -> Result<f64, String> {
        let sink = self.sink(device)?;
        let output = self.pactl(&["get-sink-volume", &sink])?;
        // The mixer answers with a line like
        // "Volume: front-left: 64881 /  99% / -0.26 dB,  front-right: ...".
        let head = output.split('%').next().unwrap_or_default();
        let percent = head.rsplit('/').next().unwrap_or_default().trim();
        percent
            .parse()
            .map_err(|_| format!("the volume of {sink} is not a number: {:?}", output.trim()))
    }

    fn sound_set_volume(&mut self, device: &str, volume: f64) -> Result<(), String> {
        let sink = self.sink(device)?;
        let volume = volume.clamp(0.0, 100.0);
        self.pactl(&["set-sink-volume", &sink, &format!("{volume:.0}%")])?;
        Ok(())
    }

    fn sound_muted(&mut self, device: &str) -> Result<bool, String> {
        let sink = self.sink(device)?;
        let output = self.pactl(&["get-sink-mute", &sink])?;
        // The mixer answers with "Mute: yes" or with "Mute: no".
        Ok(output.split_whitespace().last() == Some("yes"))
    }

    fn sound_set_muted(&mut self, device: &str, muted: bool) -> Result<(), String> {
        let sink = self.sink(device)?;
        self.pactl(&["set-sink-mute", &sink, if muted { "yes" } else { "no" }])?;
        Ok(())
    }

    fn sound_play(&mut self, path: &str) -> Result<String, String> {
        let command = format!("paplay {}", shell_quote(path));
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "the mixer could not play {path:?}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(format!("played {path:?}"))
    }

    fn sound_beep(&mut self, frequency: f64, duration_ms: u64) -> Result<String, String> {
        // The tone of v2 comes out of the speaker of the machine. The box
        // has no speaker of its own: the tone is written as a sound file
        // and the mixer of the session plays it.
        let path = format!("/tmp/ahk-tone-{}.wav", millis());
        std::fs::write(&path, tone(frequency, duration_ms)).map_err(|error| error.to_string())?;
        let played = self.sound_play(&path);
        let _ = std::fs::remove_file(&path);
        played.map(|played| format!("{played}, a tone of {frequency} hertz"))
    }
}

impl ahk::desktop::Processes for LabDesktop {
    fn env_get(&mut self, name: &str) -> Result<Option<String>, String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // warning of the environment is given when the variable is there, and
        // the name that the environment does not have is a name that has no
        // value.
        Ok(std::env::var(name).ok())
    }

    fn env_set(&mut self, name: &str, value: &str) -> Result<String, String> {
        // The name of a variable of the environment may not be empty, and it
        // may not carry the equal sign: the machine refuses both of them,
        // and it refuses them with a panic when the name is given to it as
        // it is.
        if name.is_empty() {
            return Err("the name of a variable of the environment may not be empty".to_string());
        }
        if name.contains('=') {
            return Err(
                "the name of a variable of the environment may not carry the equal sign"
                    .to_string(),
            );
        }
        // The environment of the runner is its own: the box has one
        // session and one user, and a script that sets a variable of it
        // changes it for the rest of the run. The script runs on one
        // thread, so nothing else is reading the environment while it
        // happens.
        unsafe { std::env::set_var(name, value) };
        Ok(value.to_string())
    }

    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|error| error.to_string())?;
        Ok((
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    fn launch(&mut self, command: &str, working_dir: &str) -> Result<u32, String> {
        // The `Run` of the v1 does not wait for the program: it starts it and
        // goes on. The child is left to the machine: dropping the handle does
        // not kill it, and the script may end before the program does.
        let child = the_program(command, working_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| error.to_string())?;
        Ok(child.id())
    }

    fn run_and_wait(
        &mut self,
        command: &str,
        working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        let child = the_program(command, working_dir)
            .spawn()
            .map_err(|error| error.to_string())?;
        let pid = child.id();
        let output = child
            .wait_with_output()
            .map_err(|error| error.to_string())?;
        Ok((
            pid,
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    // -- the processes -----------------------------------------------------------
    //
    // The shape of each answer is the one of the binary of v2.0.19, measured
    // on 2026-11-09; the reading of `/proc` behind it is in `crate::process`.

    fn process_exist(&mut self, which: Option<&str>) -> Result<u32, String> {
        crate::process::exist(which)
    }

    fn process_name(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        crate::process::name(which)
    }

    fn process_path(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        crate::process::path(which)
    }

    fn process_parent(&mut self, which: Option<&str>) -> Result<u32, String> {
        crate::process::parent(which)
    }

    fn process_close(&mut self, which: &str) -> Result<u32, String> {
        crate::process::close(which)
    }

    fn process_wait(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        crate::process::wait(which, timeout)
    }

    fn process_wait_close(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        crate::process::wait_close(which, timeout)
    }

    fn process_set_priority(
        &mut self,
        priority: &str,
        which: Option<&str>,
    ) -> Result<Option<u32>, String> {
        crate::process::set_priority(priority, which)
    }
}

/// The program that the `Run` of the v1 starts.
///
/// The command goes through the shell of the machine, the way the `Run` of
/// the tree does: the target of the v1 is a program and its arguments, and
/// the shell is what separates them. The working directory is the one of the
/// second parameter of the command, and it is the working directory of the
/// script when the parameter is empty.
fn the_program(command: &str, working_dir: &str) -> Command {
    let mut program = Command::new("sh");
    program.arg("-c").arg(command);
    if !working_dir.is_empty() {
        program.current_dir(working_dir);
    }
    program
}

/// The filesystem of the longest mount point that a path is under.
///
/// The table of the mounts of the machine is `/proc/self/mounts`, and the drive
/// of a path is the longest mount point it is under: a path under `/home` is in
/// the filesystem of `/home` and not in the one of `/`.
fn the_filesystem_of(path: &str) -> Option<String> {
    let table = std::fs::read_to_string("/proc/self/mounts").ok()?;
    let mut best: Option<(usize, String)> = None;
    for line in table.lines() {
        let mut fields = line.split_whitespace();
        let (_device, mount, filesystem) = (fields.next()?, fields.next()?, fields.next()?);
        let under = path == mount
            || (mount == "/" && path.starts_with('/'))
            || (path.starts_with(mount) && path.as_bytes().get(mount.len()) == Some(&b'/'));
        if !under {
            continue;
        }
        let beaten = match &best {
            None => true,
            Some((length, _)) => mount.len() >= *length,
        };
        if beaten {
            best = Some((mount.len(), filesystem.to_string()));
        }
    }
    best.map(|(_, filesystem)| filesystem)
}

/// The device node that a path is on, from the first column of `df`.
///
/// A filesystem without a device behind it -- `overlay`, a `tmpfs` -- is not
/// a drive a script can hold, and the answer is none.
fn the_device_of(path: &str) -> Option<String> {
    let output = Command::new("df").arg("-P").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let source = text.lines().nth(1)?.split_whitespace().next()?.to_string();
    if source == "overlay" || source.starts_with("tmpfs") {
        return None;
    }
    Some(source)
}

/// The kind of a drive, in the word of v2, from the filesystem of it.
///
/// The words are the ones of v2. The mapping is the honest one for a Linux
/// box: a network filesystem is a `Network` drive, an image of a disc is a
/// `CDROM`, a memory filesystem is a `RAMDisk`, and everything else on the
/// machine is `Fixed`.
fn the_kind_of(filesystem: &str) -> String {
    match filesystem {
        "nfs" | "nfs4" | "cifs" | "smb3" | "sshfs" | "9p" | "afs" => "Network",
        "iso9660" | "udf" => "CDROM",
        "tmpfs" | "ramfs" | "devtmpfs" => "RAMDisk",
        _ => "Fixed",
    }
    .to_string()
}

impl ahk::desktop::Machine for LabDesktop {

    fn tray_set_icon(&mut self, _name: &str) -> Result<(), String> {
        #[cfg(target_os = "linux")]
        {
            self.tray_set_icon(_name)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err("the tray of the session is a thing of the session of the box.".to_string())
        }
    }

    fn windows_directory(&mut self) -> Result<String, String> {
        #[cfg(target_os = "linux")]
        {
            self.windows_directory()
        }
        #[cfg(not(target_os = "linux"))]
        {
            // The machine is a Windows of its own, and it says where its
            // system is.
            std::env::var("SystemRoot").map_err(|error| error.to_string())
        }
    }
    fn program_files(&mut self) -> Result<String, String> {
        #[cfg(target_os = "linux")]
        {
            self.program_files()
        }
        #[cfg(not(target_os = "linux"))]
        {
            // The machine is a Windows of its own, and it says where the
            // programs of the user are.
            std::env::var("ProgramFiles").map_err(|error| error.to_string())
        }
    }

    fn registry_create_key(&mut self, key: &str) -> Result<(), String> {
        crate::registry::create_key(key)
    }

    fn registry_delete_key(&mut self, key: &str) -> Result<(), String> {
        crate::registry::delete_key(key)
    }

    fn registry_delete(&mut self, key: &str, name: &str) -> Result<(), String> {
        crate::registry::delete(key, name)
    }

    fn registry_write(
        &mut self,
        key: &str,
        name: &str,
        kind: &str,
        value: &str,
    ) -> Result<(), String> {
        crate::registry::write(key, name, kind, value)
    }

    fn registry_value(&mut self, key: &str, name: &str) -> Result<Option<(String, String)>, String> {
        crate::registry::value_of(key, name)
    }
    fn drive_set_label(&mut self, path: &str, label: &str) -> Result<(), String> {
        box_tools_are_near()?;
        let device = match the_device_of(path) {
            Some(device) => device,
            None => return Err(format!("the drive of {path} is not there")),
        };
        // The tool that writes a label is the filesystem's: ext* carries
        // e2label, the FAT family carries mlabel, and ntfs carries ntfslabel.
        let filesystem = the_filesystem_of(path).unwrap_or_default();
        let mut command = match filesystem.as_str() {
            fs if fs.starts_with("ext") => {
                let mut command = Command::new("e2label");
                command.arg(&device).arg(label);
                command
            }
            "vfat" | "msdos" | "fat" => {
                let mut command = Command::new("mlabel");
                command.arg("-i").arg(&device).arg(format!("::{label}"));
                command
            }
            "ntfs" => {
                let mut command = Command::new("ntfslabel");
                command.arg(&device).arg(label);
                command
            }
            other => {
                return Err(format!(
                    "the filesystem {other} of {path} has no label tool on this box"
                ));
            }
        };
        let output = command.output().map_err(|error| error.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "the label of {path} was not written: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }
    fn drive_retract(&mut self, path: &str) -> Result<(), String> {
        box_tools_are_near()?;
        let device = the_device_of(path).unwrap_or_else(|| path.to_string());
        let output = Command::new("udisksctl")
            .args(["power-off", "-b", &device])
            .output();
        match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(format!(
                "the drive of {path} was not spun down: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            )),
            Err(error) => Err(error.to_string()),
        }
    }
    fn drive_unlock(&mut self, path: &str) -> Result<(), String> {
        box_tools_are_near()?;
        let device = the_device_of(path).unwrap_or_else(|| path.to_string());
        let output = Command::new("udisksctl")
            .args(["unlock", "-b", &device])
            .output();
        match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(format!(
                "the drive of {path} was not unlocked: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            )),
            Err(error) => Err(error.to_string()),
        }
    }
    fn drive_lock(&mut self, path: &str) -> Result<(), String> {
        box_tools_are_near()?;
        let device = the_device_of(path).unwrap_or_else(|| path.to_string());
        let output = Command::new("udisksctl")
            .args(["lock", "-b", &device])
            .output();
        match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(format!(
                "the drive of {path} was not locked: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            )),
            Err(error) => Err(error.to_string()),
        }
    }
    fn drive_eject(&mut self, path: &str) -> Result<(), String> {
        box_tools_are_near()?;
        let output = Command::new("eject").arg(path).output();
        match output {
            Ok(out) if out.status.success() => Ok(()),
            Ok(out) => Err(format!(
                "the drive of {path} was not ejected: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            )),
            Err(error) => Err(error.to_string()),
        }
    }
    fn drive_status_cd(&mut self, path: &str) -> Result<Option<String>, String> {
        let device = match the_device_of(path) {
            Some(device) => device,
            None => return Ok(None),
        };
        let output = Command::new("lsblk")
            .args(["-no", "TYPE", &device])
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Ok(None);
        }
        let kind = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if kind != "rom" {
            return Ok(None);
        }
        // A reader with no medium in it is open; one that cannot be asked is
        // not ready. Neither state is measured on the binary: this box has no
        // reader, so the answer for one that IS a reader is the honest one.
        Ok(Some("not ready".to_string()))
    }
    fn drive_list(&mut self) -> Result<Vec<String>, String> {
        box_tools_are_near()?;
        // `-d` asks about the drives alone and not their partitions, and `-e 7`
        // leaves the loop and ram devices out: the drives of the machine are
        // the ones a script can hold and unmount.
        let output = Command::new("lsblk")
            .args(["-dno", "NAME", "-e", "7"])
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err("the machine did not list its drives".to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::to_string)
            .filter(|name| !name.is_empty())
            .collect())
    }
    fn drive_serial(&mut self, path: &str) -> Result<Option<String>, String> {
        let device = match the_device_of(path) {
            Some(device) => device,
            // A present drive without a device behind it carries no serial
            // the box can read: the drive is there, the serial is empty.
            None => {
                return Ok(if std::path::Path::new(path).exists() {
                    Some(String::new())
                } else {
                    None
                })
            }
        };
        // The serial lives on the DISK, and a path usually names a partition
        // of it: walk up to the parent device when the partition says none.
        let mut device = device;
        for _ in 0..2 {
            let output = Command::new("lsblk")
                .args(["-no", "SERIAL", &device])
                .output()
                .map_err(|error| error.to_string())?;
            if output.status.success() {
                let serial = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !serial.is_empty() {
                    return Ok(Some(serial));
                }
            } else if std::path::Path::new(path).exists() {
                // A source that lsblk cannot read -- a zfs dataset -- is a
                // drive that is THERE, and one that carries no serial.
                return Ok(Some(String::new()));
            }
            let parent = Command::new("lsblk")
                .args(["-no", "PKNAME", &device])
                .output()
                .map_err(|error| error.to_string())?;
            if !parent.status.success() {
                break;
            }
            let name = String::from_utf8_lossy(&parent.stdout).trim().to_string();
            if name.is_empty() {
                break;
            }
            device = format!("/dev/{name}");
        }
        Ok(None)
    }
    fn drive_label(&mut self, path: &str) -> Result<Option<String>, String> {
        match the_device_of(path) {
            Some(device) => {
                let output = Command::new("lsblk")
                    .args(["-no", "LABEL", &device])
                    .output()
                    .map_err(|error| error.to_string())?;
                if !output.status.success() {
                    // A source that lsblk cannot read -- a zfs dataset is the
                    // root of this box -- is a drive that is THERE and that
                    // carries no label the box can ask for.
                    return Ok(if std::path::Path::new(path).exists() {
                        Some(String::new())
                    } else {
                        None
                    });
                }
                Ok(Some(
                    String::from_utf8_lossy(&output.stdout).trim().to_string(),
                ))
            }
            // A filesystem without a device behind it -- a zfs dataset is the
            // root of this box -- is a drive that is THERE, and one that
            // carries no label. Measured on the binary of v2.0.19 on
            // 2026-11-10: a present drive answers its label, empty when it
            // has none; only a drive that is not there throws.
            None => Ok(if std::path::Path::new(path).exists() {
                Some(String::new())
            } else {
                None
            }),
        }
    }
    fn drive_space(&mut self, path: &str) -> Result<Option<ahk::desktop::DriveSpace>, String> {
        box_tools_are_near()?;
        // The drive of a path is the filesystem that the path is on, and `df`
        // is the tool that says it. `-P` is the POSIX form, which is one line
        // per drive and a fixed order of the columns, and `-k` counts the
        // blocks in a thousand bytes, which is the unit the answer is in.
        let output = Command::new("df")
            .arg("-Pk")
            .arg(path)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            // A path whose drive cannot be read is a drive that is not there:
            // the command that asked decides what that means, because v2 does
            // not answer the same thing for every one of them.
            return Ok(None);
        }
        let text = String::from_utf8_lossy(&output.stdout);
        let fields: Vec<&str> = text
            .lines()
            .nth(1)
            .unwrap_or_default()
            .split_whitespace()
            .collect();
        if fields.len() < 4 {
            return Err(format!("the machine did not describe the drive of {path}"));
        }
        let blocks: i64 = fields[1].parse().unwrap_or(0);
        let available: i64 = fields[3].parse().unwrap_or(0);
        let filesystem = the_filesystem_of(path).unwrap_or_default();
        let kind = if filesystem.is_empty() {
            String::new()
        } else {
            the_kind_of(&filesystem)
        };
        Ok(Some(ahk::desktop::DriveSpace {
            free: available / 1024,
            capacity: blocks / 1024,
            filesystem,
            kind,
        }))
    }
    fn executable(&mut self) -> Result<String, String> {
        crate::machine::executable()
    }
    fn user_name(&mut self) -> Result<String, String> {
        crate::machine::user_name()
    }
    fn computer_name(&mut self) -> Result<String, String> {
        crate::machine::computer_name()
    }
    fn shell(&mut self) -> Result<String, String> {
        crate::machine::shell()
    }
    fn folders(&mut self) -> Result<Folders, String> {
        crate::machine::folders()
    }
    fn os_version(&mut self) -> Result<String, String> {
        crate::machine::os_version()
    }
    fn language(&mut self) -> Result<String, String> {
        crate::machine::language()
    }
    fn is_admin(&mut self) -> Result<bool, String> {
        crate::machine::is_admin()
    }
    fn idle_times(&mut self) -> Result<Idle, String> {
        crate::machine::idle_times()
    }
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        crate::network::ip_addresses()
    }
    fn output_debug(&mut self, text: &str) -> Result<(), String> {
        // The debugger of the machine is the standard error of the process:
        // the report of the runner is the standard output, and it is not
        // touched.
        eprintln!("{text}");
        Ok(())
    }
    fn drive(&mut self, path: &str) -> Result<Option<ahk::desktop::Drive>, String> {
        crate::drives::drive(path)
    }
}

/// The dialogs of the box.
///
/// The box has zenity, and zenity is the window of the session: it
/// opens the dialog on the display of the user and waits for the
/// answer. The exit code of zenity says how the dialog ended: 0 is
/// the OK button, 1 is the Cancel button and 5 is the timeout.
///
/// Measured on the box on 2026-10-29: zenity 4.2.2 answers the entry
/// with the text of the user, and it exits with 5 when the timeout of
/// the options goes by.
impl ahk::desktop::Network for LabDesktop {
    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        crate::network::download(url, file)
    }
}

impl ahk::desktop::Dialogs for LabDesktop {

    fn notify(&mut self, title: &str, text: &str) -> Result<(), String> {
        // The tool is `notify-send`, which is a tool of a Linux desktop. A
        // host that has none of them says so, and does not let the shell
        // answer "not found": the failure of the desktop is the failure the
        // script sees.
        box_tools_are_near()?;
        // A notification of the session is what a `TrayTip` is on a Linux
        // desktop: the same words, to the same person, in the corner of the
        // same screen. The timeout is left to the session, which knows how
        // long its own notifications stay.
        let mut args: Vec<String> = Vec::new();
        if !title.is_empty() {
            args.push(format!("--app-name={title}"));
        }
        if !text.is_empty() {
            args.push(text.to_string());
        }
        let command = format!(
            "notify-send {}",
            args.iter()
                .map(|argument| shell_quote(argument))
                .collect::<Vec<String>>()
                .join(" ")
        );
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            let reason = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(if reason.is_empty() {
                "the notification could not be shown: notify-send failed".to_string()
            } else {
                format!("the notification could not be shown: {reason}")
            });
        }
        Ok(())
    }
    fn input_box(
        &mut self,
        prompt: &str,
        title: &str,
        options: &InputOptions,
        default: &str,
    ) -> Result<InputAnswer, String> {
        let mut args: Vec<String> = vec!["--entry".to_string()];
        if !prompt.is_empty() {
            args.push(format!("--text={prompt}"));
        }
        if !title.is_empty() {
            args.push(format!("--title={title}"));
        }
        if !default.is_empty() {
            args.push(format!("--entry-text={default}"));
        }
        if options.password {
            args.push("--password".to_string());
        }
        if let Some(timeout) = options.timeout {
            args.push(format!("--timeout={timeout}"));
        }
        let (text, code) = zenity(&args)?;
        Ok(InputAnswer {
            value: match code {
                0 => text,
                _ => String::new(),
            },
            result: the_result_of_the_dialog(code).to_string(),
        })
    }

    fn file_select(
        &mut self,
        options: &FileOptions,
        root: &str,
        title: &str,
        filter: &str,
    ) -> Result<FileAnswer, String> {
        let mut args: Vec<String> = vec!["--file-selection".to_string()];
        if options.directory {
            args.push("--directory".to_string());
        }
        if options.multiple {
            args.push("--multiple".to_string());
            // The separator of zenity is a bar, and the paths of the
            // files are read one by one.
            args.push("--separator=\n".to_string());
        }
        if options.save {
            args.push("--save".to_string());
        }
        if !root.is_empty() {
            args.push(format!("--filename={root}"));
        }
        if !title.is_empty() {
            args.push(format!("--title={title}"));
        }
        if let Some(filter) = the_filter_of_zenity(filter) {
            args.push(format!("--file-filter={filter}"));
        }
        let (text, code) = zenity(&args)?;
        match code {
            0 => {
                if options.multiple {
                    // Zenity writes one path per line.
                    let paths = text.lines().map(|line| line.to_string()).collect();
                    Ok(FileAnswer::Many(paths))
                } else {
                    Ok(FileAnswer::One(text))
                }
            }
            _ => Ok(FileAnswer::One(String::new())),
        }
    }

    fn dir_select(&mut self, starting: &str, options: i64, prompt: &str) -> Result<String, String> {
        let mut args: Vec<String> = vec!["--file-selection".to_string(), "--directory".to_string()];
        // The options of v2: 1 is the button that makes a new folder and
        // 2 is the field where the name is written. The save mode of
        // zenity has both of them, and it is the only one that has: the
        // "--create" of the older versions of zenity is not there, and
        // it is not there with a folder either.
        if options & 3 != 0 {
            args.push("--save".to_string());
        }
        if !starting.is_empty() {
            args.push(format!("--filename={starting}"));
        }
        if !prompt.is_empty() {
            args.push(format!("--title={prompt}"));
        }
        let (text, code) = zenity(&args)?;
        match code {
            0 => Ok(text),
            _ => Ok(String::new()),
        }
    }
}

/// The dialog of zenity, and how it ended.
///
/// The text of the answer is what zenity wrote on its standard output,
/// and the code is what it exited with.
fn zenity(args: &[String]) -> Result<(String, i32), String> {
    let output = Command::new("zenity")
        .args(args)
        .output()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                "zenity is not on this machine, and the box has no other dialog".to_string()
            } else {
                format!("the dialog of the box did not open: {error}")
            }
        })?;
    let code = output.status.code().unwrap_or(-1);
    if !(0..=5).contains(&code) {
        // The dialog failed: zenity says why on its error output.
        let reason = String::from_utf8_lossy(&output.stderr);
        return Err(format!("the dialog of the box failed: {}", reason.trim()));
    }
    let text = String::from_utf8_lossy(&output.stdout)
        .trim_end_matches(['\r', '\n'])
        .to_string();
    Ok((text, code))
}

/// How a dialog of the box ended.
///
/// The words are the ones of v2: "OK", "Cancel" and "Timeout".
fn the_result_of_the_dialog(code: i32) -> &'static str {
    match code {
        0 => "OK",
        5 => "Timeout",
        _ => "Cancel",
    }
}

/// The filter of v2, in the spelling of zenity.
///
/// v2 writes the name of the filter and its patterns in one string,
/// between parentheses: "Documents (*.txt; *.doc)". Zenity wants the
/// name and the patterns apart: "Documents | *.txt *.doc".
fn the_filter_of_zenity(filter: &str) -> Option<String> {
    let filter = filter.trim();
    if filter.is_empty() {
        return None;
    }
    let open = filter.find('(');
    let close = filter.rfind(')');
    let (name, patterns) = match (open, close) {
        (Some(open), Some(close)) if close > open => (
            filter[..open].trim(),
            filter[open + 1..close]
                .split(';')
                .map(|pattern| pattern.trim())
                .filter(|pattern| !pattern.is_empty())
                .collect::<Vec<_>>(),
        ),
        _ => (filter, Vec::new()),
    };
    if patterns.is_empty() {
        return Some(format!("{name} | *"));
    }
    Some(format!("{name} | {}", patterns.join(" ")))
}

/// The files of a folder that match a pattern.
///
/// The mode is v2's: "R" walks into the subfolders as well, and "D"
/// gives the directories instead of the files. The "A" gives the files and
/// the directories together, and it is the mode of the v1 that has the
/// number one.
///
/// Measured on the binary of v2.0.19 on 2026-11-05 and on the binary of
/// AutoHotkey v1.1.37.02 on 2026-11-05: the files of a folder come first,
/// and the walk into the folders of it comes after them.
pub(crate) fn walk_the_folder(
    folder: &Path,
    pattern: &str,
    mode: &str,
    files: &mut Vec<String>,
) -> Result<(), String> {
    let mut entries: Vec<(String, bool, PathBuf)> = Vec::new();
    for entry in std::fs::read_dir(folder)
        .map_err(|error| format!("the folder {folder:?} cannot be read: {error}"))?
    {
        let entry =
            entry.map_err(|error| format!("the folder {folder:?} cannot be read: {error}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let is_file = path.is_file();
        entries.push((name, is_file, path));
    }
    // The order of a directory is not the order of its name, and a script
    // that loops over the files wants the same order every time.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31: the files of a folder
    // of NTFS come in the order of the name with the case taken away, and the
    // tree sorts them the same way.
    entries.sort_by(|one, other| {
        one.0
            .to_uppercase()
            .cmp(&other.0.to_uppercase())
            .then_with(|| one.0.cmp(&other.0))
    });
    // The mode is a set of letters: the "F" and the "V" give the files, the
    // "D" and the "K" give the folders, and the "A" gives the two of them --
    // it is the way that the tree asks for the two of them. The mode that
    // asks for neither of them gives the files.
    //
    // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the "FD"
    // gives the files and the folders, and the "K" gives the folders like the
    // "D".
    let wants_the_files = mode.contains('F')
        || mode.contains('V')
        || mode.contains('A')
        || !(mode.contains('D') || mode.contains('K'));
    let wants_the_folders = mode.contains('D') || mode.contains('K') || mode.contains('A');
    for (name, is_file, path) in &entries {
        // The directories are not files: a loop over the files of a
        // folder does not walk into them, unless the mode asks for the
        // directories instead, or for the two of them.
        let wanted = if *is_file {
            wants_the_files
        } else {
            wants_the_folders
        };
        if wanted && name_matches(pattern, name) {
            files.push(path.to_string_lossy().to_string());
        }
    }
    // The walk into the folders of the folder comes after the files of it.
    if mode.contains('R') {
        for (_, _is_file, path) in entries.iter().filter(|(_, is_file, _)| !*is_file) {
            walk_the_folder(path, pattern, mode, files)?;
        }
    }
    Ok(())
}

/// The X display to ask, when there is one.
///
/// The session is Wayland, and the X server beside it is the rootless
/// Xwayland of gamescope, whose socket is `/tmp/.X11-unix/X0`.
impl LabDesktop {
    /// The sound device that the script asks about.
    ///
    /// The device of the system is the default device of the mixer, a
    /// number is the device of that number, and a name is the device of
    /// that name.
    fn sink(&mut self, device: &str) -> Result<String, String> {
        if device.is_empty() {
            return Ok("@DEFAULT_SINK@".to_string());
        }
        let sinks = self.sinks()?;
        if let Some(number) = device.strip_prefix('#') {
            let index: usize = number
                .parse()
                .map_err(|_| format!("the sound device {device:?} is not a number"))?;
            return sinks.get(index.saturating_sub(1)).cloned().ok_or_else(|| {
                format!(
                    "the mixer has {} devices, and there is no #{index}",
                    sinks.len()
                )
            });
        }
        if sinks.iter().any(|sink| sink == device) {
            return Ok(device.to_string());
        }
        Err(format!(
            "the mixer has no sound device named {device:?}: it has {}",
            sinks.join(", ")
        ))
    }

    /// The sound devices of the mixer, in the order of the mixer.
    fn sinks(&mut self) -> Result<Vec<String>, String> {
        let output = self.pactl(&["list", "sinks", "short"])?;
        Ok(output
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('\t');
                let _index = parts.next()?;
                parts.next().map(|name| name.to_string())
            })
            .collect())
    }

    /// Ask the mixer of the session.
    fn pactl(&mut self, args: &[&str]) -> Result<String, String> {
        let command = format!("pactl {}", args.join(" "));
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "the mixer refused: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// A tone, as a sound file.
///
/// The tone is one sine, and it starts and ends with a little silence,
/// so that it does not click.
fn tone(frequency: f64, duration_ms: u64) -> Vec<u8> {
    const RATE: u32 = 44100;
    let samples = (RATE as u64 * duration_ms / 1000).max(1);
    let mut data = Vec::with_capacity(samples as usize * 2);
    for index in 0..samples {
        let time = index as f64 / RATE as f64;
        let position = index as f64 / samples as f64;
        let fade = (position * 20.0).min(1.0) * ((1.0 - position) * 20.0).min(1.0);
        let value = (2.0 * std::f64::consts::PI * frequency * time).sin() * fade * 0.3;
        data.extend_from_slice(&((value * i16::MAX as f64) as i16).to_le_bytes());
    }
    // The header of a wave file: the riff, the wave, the format and the
    // data, with their sizes.
    let mut out = Vec::with_capacity(data.len() + 44);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&RATE.to_le_bytes());
    out.extend_from_slice(&(RATE * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&data);
    out
}

fn x_display() -> Option<String> {
    if let Ok(display) = std::env::var("DISPLAY") {
        if !display.is_empty() {
            return Some(display);
        }
    }
    for number in 0..8 {
        if Path::new(&format!("/tmp/.X11-unix/X{number}")).exists() {
            return Some(format!(":{number}"));
        }
    }
    None
}

/// Whether the session is Wayland.
fn wayland_session() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
}

/// The reason the windows of the session cannot be listed.
///
/// The window list is a thing of the Linux box: the compositor of a Wayland
/// session publishes it, and X11 is the other path. A host that is not Linux
/// has neither, and it says so instead of describing a session it is not in.
fn no_window_list_reason() -> String {
    if cfg!(target_os = "linux") {
        return "there is no X server on this box to ask: the session is Wayland, and the \
                compositor exposes no window list to D-Bus."
            .to_string();
    }
    "the windows of the desktop are a thing of the Linux box: this host has neither the \
     compositor of a Wayland session nor an X server to ask."
        .to_string()
}

/// The client of the compositor, opened when it is first needed.
///
/// The connection is kept: the compositor is asked again with every
/// question, and the answers arrive on the same connection.
#[cfg(target_os = "linux")]
impl LabDesktop {
    // ---- wayland_matching
    /// The windows whose title matches, by Wayland.
    ///
    /// The listing carries the identifier of a window and its application id,
    /// and those are the id and the class that the `WinGet*` commands read.
    #[cfg(target_os = "linux")]
    fn wayland_matching(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Vec<ahk::desktop::WindowRef>, String> {
        let windows = self.wayland()?.windows()?;
        let mut matched = Vec::new();
        for window in &windows {
            // The empty title is every window: measured on the binary of
            // v2.0.19 on 2026-11-09, `WinGetList()` answered the whole list.
            if !title.is_empty() && !title_matches(&window.title, title, mode)? {
                continue;
            }
            matched.push(ahk::desktop::WindowRef {
                id: window.identifier.clone().unwrap_or_default(),
                class: window.app_id.clone(),
            });
        }
        Ok(matched)
    }

    // ---- wayland_geometry
    /// Where the window is and how big it is, by Wayland.
    ///
    /// The geometry is the event of the version 2 of the COSMIC protocol, and
    /// the client asks for that version or a later one. A window whose
    /// geometry the compositor did not send is a failure with the reason: a
    /// place that is not known is not a window at the origin.
    #[cfg(target_os = "linux")]
    fn wayland_geometry(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<ahk::desktop::WindowGeometry>, String> {
        if title.is_empty() {
            return Ok(None);
        }
        let windows = self.wayland()?.windows()?;
        for window in &windows {
            if !title_matches(&window.title, title, mode)? {
                continue;
            }
            let Some(geometry) = window.geometry else {
                return Err("the compositor did not say where this window is, so its place is \
                            not known"
                    .to_string());
            };
            return Ok(Some(ahk::desktop::WindowGeometry {
                x: i64::from(geometry.x),
                y: i64::from(geometry.y),
                width: i64::from(geometry.width),
                height: i64::from(geometry.height),
            }));
        }
        Ok(None)
    }

    // ---- wayland_min_max
    #[cfg(target_os = "linux")]
    fn wayland_min_max(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<i64>, String> {
        if title.is_empty() {
            return Ok(None);
        }
        let windows = self.wayland()?.windows()?;
        for window in &windows {
            if !title_matches(&window.title, title, mode)? {
                continue;
            }
            // A window that is both is answered as maximized: the maximized
            // window is the one the user sees, and the minimized state of it
            // is not what the screen shows.
            if window.maximized {
                return Ok(Some(1));
            }
            if window.minimized {
                return Ok(Some(-1));
            }
            return Ok(Some(0));
        }
        Ok(None)
    }

    // ---- wayland_wait
    /// Wait for a condition about a window, by Wayland.
    #[cfg(target_os = "linux")]
    fn wayland_wait(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
        what: ahk::desktop::WaitFor,
        timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            if let Some(answer) = self.the_window_condition(title, mode, what)? {
                return Ok(Some(answer));
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }
            pause_before_the_next_look();
        }
    }

    // ---- wayland_monitors
    /// Whether the window is maximized or minimized, by Wayland.    ///
    /// The two states are entries of the state array of the COSMIC protocol,
    /// the same array that says which window is the active one, so the answer
    /// is already in what the client refreshes.
    /// The monitors of the compositor, by Wayland.
    ///
    /// The compositor names its outputs and places them, and the name of one is
    /// the name of `MonitorGetName`. The **size** of one is what its mode says,
    /// and the mode event of this compositor is not arriving: measured on the
    /// box on 2026-11-09, `/` answered `HDMI-A-1` for the name and a rectangle
    /// of zeros for the bounds. So the extent of the session is used for the
    /// size of an output that did not give one, and the plan records that this
    /// is a gap and not a design.
    #[cfg(target_os = "linux")]
    fn wayland_monitors(&mut self) -> Result<Vec<ahk::desktop::Monitor>, String> {
        let reported = self.wayland()?.monitors()?;
        let (width, height) = LabDesktop::screen_size(self).map_err(|error| error.to_string())?;
        Ok(reported
            .into_iter()
            .map(|monitor| {
                let mut left = i64::from(monitor.left);
                let mut top = i64::from(monitor.top);
                let mut right = i64::from(monitor.right);
                let mut bottom = i64::from(monitor.bottom);
                if right <= left || bottom <= top {
                    left = 0;
                    top = 0;
                    right = width;
                    bottom = height;
                }
                ahk::desktop::Monitor {
                    name: monitor.name,
                    left,
                    top,
                    right,
                    bottom,
                    scale: i64::from(monitor.scale),
                }
            })
            .collect())
    }

    // ---- the_window_condition
    /// Whether the condition that a wait is about holds now.
    ///
    /// `Some` is the condition met, and the answer of the conditions about the
    /// absence of a window is the empty text, because there is no window to
    /// identify.
    #[cfg(target_os = "linux")]
    fn the_window_condition(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
        what: ahk::desktop::WaitFor,
    ) -> Result<Option<String>, String> {
        use ahk::desktop::WaitFor;
        // The empty title is no window at all: v2 falls back to the last found
        // window, and this runtime keeps none.
        if title.is_empty() {
            return Ok(match what {
                WaitFor::NotActive | WaitFor::Close => Some(String::new()),
                WaitFor::Present | WaitFor::Active => None,
            });
        }
        let windows = self.wayland()?.windows()?;
        let mut found = None;
        for window in &windows {
            if title_matches(&window.title, title, mode)? {
                found = Some(window.clone());
                break;
            }
        }
        Ok(match what {
            WaitFor::Present => found.map(|window| window.identifier.unwrap_or_default()),
            WaitFor::Active => found
                .filter(|window| window.active)
                .map(|window| window.identifier.unwrap_or_default()),
            WaitFor::NotActive => match found {
                Some(window) if window.active => None,
                _ => Some(String::new()),
            },
            WaitFor::Close => match found {
                Some(_) => None,
                None => Some(String::new()),
            },
        })
    }

    /// The client of the compositor.
    fn wayland(&mut self) -> Result<&mut crate::wayland::Client, String> {
        if self.wayland.is_none() {
            self.wayland = Some(crate::wayland::Client::open()?);
        }
        Ok(self.wayland.as_mut().expect("the client was just opened"))
    }

    /// The title of the window that has the focus, by Wayland.
    #[cfg(target_os = "linux")]
    fn wayland_active_title(&mut self) -> Result<Option<String>, String> {
        let active = self.wayland()?.active()?;
        Ok(active.map(|window| window.title))
    }

    /// Whether the window with this title is the active one, by Wayland.
    #[cfg(target_os = "linux")]
    fn wayland_active(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        // The focus is asked before the title is looked at: a compositor
        // that does not say which window has the focus says that, and not
        // that the title of the question is empty. Measured on the VM of
        // the matrix on 2026-11-11: `WinActive()` with no title on sway
        // answered the reason of the empty title, which is true but is
        // not the reason the question has no answer.
        let active = self.wayland()?.active()?;
        if title.is_empty() {
            return Ok(empty_title());
        }
        let Some(window) = active else {
            return Ok(WindowAnswer::Absent);
        };
        if title_matches(&window.title, title, mode)? {
            return Ok(WindowAnswer::Active);
        }
        Ok(WindowAnswer::NotActive)
    }

    /// Whether a window with this title exists, by Wayland.
    #[cfg(target_os = "linux")]
    fn wayland_exists(
        &mut self,
        title: &str,
        mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        if title.is_empty() {
            return Ok(empty_title());
        }
        let windows = self.wayland()?.windows()?;
        for window in &windows {
            if title_matches(&window.title, title, mode)? {
                return Ok(if window.active {
                    WindowAnswer::Active
                } else {
                    WindowAnswer::NotActive
                });
            }
        }
        Ok(WindowAnswer::Absent)
    }

    /// The windows of the session, with the ids this runtime gives them, by
    /// Wayland.
    #[cfg(target_os = "linux")]
    fn wayland_windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        let windows = self.wayland()?.windows_with_ids()?;
        Ok(windows
            .into_iter()
            .map(|(id, window)| ahk::desktop::WindowInfo {
                id,
                title: window.title,
                class: window.app_id,
                active: window.active,
            })
            .collect())
    }
}

/// The windows of a Wayland session, on a platform that has no Wayland.
///
/// The protocols of the compositor are Linux ones, and the client of the
/// compositor is a Unix program. The questions therefore fail with the
/// reason, and the box says that it is not Linux.
#[cfg(not(target_os = "linux"))]
impl LabDesktop {
    // ---- wayland_matching
    /// The windows whose title matches, on a platform that has no Wayland.
    fn wayland_matching(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Vec<ahk::desktop::WindowRef>, String> {
        Err(not_linux())
    }

    // ---- wayland_geometry
    /// Where the window is, on a platform that has no Wayland.
    fn wayland_geometry(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<ahk::desktop::WindowGeometry>, String> {
        Err(not_linux())
    }

    // ---- wayland_min_max
    /// Whether the window is maximized, on a platform that has no Wayland.
    fn wayland_min_max(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<i64>, String> {
        Err(not_linux())
    }

    // ---- wayland_wait
    /// Wait for a window, on a platform that has no Wayland.
    fn wayland_wait(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
        _what: ahk::desktop::WaitFor,
        _timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        Err(not_linux())
    }

    // ---- wayland_monitors
    /// The monitors, on a platform that has no Wayland.
    fn wayland_monitors(&mut self) -> Result<Vec<ahk::desktop::Monitor>, String> {
        Err(not_linux())
    }

    /// The title of the window that has the focus, by Wayland.
    fn wayland_active_title(&mut self) -> Result<Option<String>, String> {
        Err(not_linux())
    }

    /// Whether the window with this title is the active one, by Wayland.
    fn wayland_active(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        Err(not_linux())
    }

    /// Whether a window with this title exists, by Wayland.
    fn wayland_exists(
        &mut self,
        _title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<WindowAnswer, String> {
        Err(not_linux())
    }

    /// The windows of the session, with the ids this runtime gives them.
    fn wayland_windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        Err(not_linux())
    }
}

/// The reason this platform cannot answer.
#[cfg(not(target_os = "linux"))]
fn not_linux() -> String {
    "the windows of a Wayland session are asked of the compositor, and the protocols of this \
     crate are for Linux"
        .to_string()
}

/// The answer to a question about a window with no title.
///
/// v2 uses the last found window when the title is empty, and this
/// runtime does not keep one. The gate of Linux is not on it: the X11
/// path that reads the client list is compiled on every machine, and it
/// answers the same reason there.
fn empty_title() -> WindowAnswer {
    WindowAnswer::Unknown(
        "the title is empty, and v2 would use the last found window: this runtime does not keep \
         one."
            .to_string(),
    )
}

/// The window of an X11 session that a question names: the active one when
/// the question names nothing, and the first one whose title matches when
/// it names a title. The id it answers is the id of the X server, which the
/// client list carries.
fn the_x_window_named(
    windows: &[ahk::desktop::WindowInfo],
    title: &str,
    mode: ahk::desktop::TitleMatch,
) -> Result<Option<i64>, String> {
    for window in windows {
        let matches = if title.is_empty() {
            window.active
        } else {
            title_matches(&window.title, title, mode)?
        };
        if matches {
            return Ok(Some(window.id));
        }
    }
    Ok(None)
}

/// Whether a title matches the one asked for.
///
/// The mode is the setting of the script, and the rule is the one of the
/// dialect: the crate of the language says which title matches, and the box
/// gives the titles of the machine.
fn title_matches(
    title: &str,
    wanted: &str,
    mode: ahk::desktop::TitleMatch,
) -> Result<bool, String> {
    if wanted.is_empty() {
        return Ok(false);
    }
    mode.matches(title, wanted)
}

/// Parse a colour: `#rrggbb`, `#rgb`, or `rrggbb`.
fn parse_colour(colour: &str) -> Result<[u8; 3], String> {
    let text = colour
        .trim()
        .trim_start_matches('#')
        .trim_start_matches("0x");
    let parse = |part: &str| -> Result<u8, String> {
        u8::from_str_radix(part, 16).map_err(|error| error.to_string())
    };
    match text.len() {
        6 => {
            let red = parse(&text[0..2])?;
            let green = parse(&text[2..4])?;
            let blue = parse(&text[4..6])?;
            Ok([red, green, blue])
        }
        3 => {
            let expand = |part: &str| -> Result<u8, String> {
                let value = parse(part)?;
                Ok(value * 17)
            };
            Ok([
                expand(&text[0..1])?,
                expand(&text[1..2])?,
                expand(&text[2..3])?,
            ])
        }
        _ => Err(format!("{colour:?} is not a colour: write it as #rrggbb")),
    }
}

/// The milliseconds since the epoch, for the scratch file names.
fn millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

/// The size of an image, read from its header.
fn image_size(path: &Path) -> Option<(i64, i64)> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.len() < 24 || &bytes[1..4] != b"PNG" {
        return None;
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]) as i64;
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]) as i64;
    Some((width, height))
}

/// Read the colour of the first pixel of a PNG, as `#rrggbb`.
fn read_pixel(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(Path::new(path)).map_err(|error| error.to_string())?;
    let image = crate::picture::read(&bytes)?;
    let (red, green, blue) = image
        .colour_at(0, 0)
        .ok_or_else(|| "the capture is empty".to_string())?;
    Ok(format!("#{red:02x}{green:02x}{blue:02x}"))
}

/// Find a colour in an image; the position is relative to the image.
fn search_pixel(path: &str, target: [u8; 3]) -> Option<(i64, i64)> {
    let bytes = std::fs::read(Path::new(path)).ok()?;
    let image = crate::picture::read(&bytes).ok()?;
    let (want_red, want_green, want_blue) = (target[0], target[1], target[2]);
    for y in 0..image.height {
        for x in 0..image.width {
            let (red, green, blue) = image.colour_at(x, y)?;
            if red == want_red && green == want_green && blue == want_blue {
                return Some((x as i64, y as i64));
            }
        }
    }
    None
}

/// The modifier names, for the report.
pub fn modifier_names() -> Vec<String> {
    MODIFIERS
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect()
}

/// Whether a name matches a pattern with the stars and the question
/// marks of a shell glob.
///
/// The pattern of a `Loop Files` is a glob: `*` is any number of
/// characters and `?` is one. The matcher is the same in every
/// language, so it is written out here.
pub(crate) fn name_matches(pattern: &str, name: &str) -> bool {
    // Measured on the binary of v2.0.19 on 2026-10-31: the case of the
    // pattern and the case of the name do not count: the machine of v2 is
    // one that does not tell them apart.
    let pattern: Vec<char> = pattern
        .chars()
        .flat_map(|letter| letter.to_lowercase())
        .collect();
    let name: Vec<char> = name
        .chars()
        .flat_map(|letter| letter.to_lowercase())
        .collect();
    let mut table = vec![vec![false; name.len() + 1]; pattern.len() + 1];
    table[0][0] = true;
    for p in 1..=pattern.len() {
        if pattern[p - 1] == '*' {
            table[p][0] = table[p - 1][0];
        }
    }
    for p in 1..=pattern.len() {
        for n in 1..=name.len() {
            table[p][n] = match pattern[p - 1] {
                '*' => table[p - 1][n] || table[p][n - 1],
                '?' => table[p - 1][n - 1],
                letter => letter == name[n - 1] && table[p - 1][n - 1],
            };
        }
    }
    table[pattern.len()][name.len()]
}

// ---- the_helper_of_the_window
/// Where the program that draws the windows of the script is.
///
/// Two places, in the order that works for a binary that has been moved and for
/// one that has not:
///
/// 1. **beside the binary**, under `helper/` -- which is where a copy of the
///    runner carries it, and the convention the raven's helper follows;
/// 2. **under the crate** as it was compiled, which is right when the runner is
///    built where the sources are (the box builds its own), and is a build path
///    baked into the binary when it is not.
///
/// The first that exists wins. The answer when neither is there names both, so a
/// failure says where it looked rather than that something is missing.
fn the_helper_of_the_window() -> Result<std::path::PathBuf, String> {
    let mut tried: Vec<String> = Vec::new();
    if let Ok(program) = std::env::current_exe() {
        if let Some(folder) = program.parent() {
            let beside = folder.join("helper/ahk-gui.py");
            if beside.is_file() {
                return Ok(beside);
            }
            tried.push(beside.to_string_lossy().into_owned());
        }
    }
    let under = Path::new(env!("CARGO_MANIFEST_DIR")).join("helper/ahk-gui.py");
    if under.is_file() {
        return Ok(under);
    }
    tried.push(under.to_string_lossy().into_owned());
    Err(format!(
        "the program that draws the windows of the script is not where it was looked for: {}",
        tried.join(", ")
    ))
}

// ---- the_events_of_the_window
/// of letting the shell answer "not found": a failure that names a missing
/// program of another platform tells the script nothing it can act on.
/// Where the events of the windows of the script are written.
///
/// Derived from the process, so two runners on one machine do not read each
/// other's events, and nothing has to be kept: the helper and the reader both
/// compute the same name.
fn the_events_of_the_window() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("ahk-gui-events-{}", std::process::id()))
}

// ---- quoted
/// A text as the JSON string of it, with the characters that JSON needs.
fn quoted(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for letter in text.chars() {
        match letter {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(letter),
        }
    }
    out.push('"');
    out
}

// ---- the_number_in
/// The number after a name in an answer of the helper.
fn the_number_in(answer: &str, name: &str) -> Option<i64> {
    let rest = answer.split_once(name)?.1;
    let digits: String = rest
        .chars()
        .take_while(|letter| letter.is_ascii_digit() || *letter == '-')
        .collect();
    digits.parse().ok()
}

// ---- the_text_in
/// The text after a name in an answer of the helper.
fn the_text_in(answer: &str, name: &str) -> Option<String> {
    let rest = answer.split_once(name)?.1.trim_start();
    let rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut letters = rest.chars();
    while let Some(letter) = letters.next() {
        match letter {
            '"' => return Some(out),
            '\\' => {
                if let Some(next) = letters.next() {
                    out.push(match next {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        other => other,
                    });
                }
            }
            other => out.push(other),
        }
    }
    None
}

// ---- box_tools_are_near
fn box_tools_are_near() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{name_matches, title_matches};
    use ahk::desktop::TitleMatch;

    /// The mode of v2 is the default one, 2: the title of the window has
    /// to contain the text that was asked for.
    #[test]
    fn a_title_contains_the_text_that_was_asked_for() {
        let mode = TitleMatch::Contains;
        assert!(title_matches("<host>/Login - Chromium", "Login", mode).expect("it matches"));
        assert!(
            title_matches("<host>/Login - Chromium", "<host>/Login - Chromium", mode)
                .expect("it matches")
        );
    }

    /// The case matters, the way it matters in v2: the window titles and
    /// the text are case-sensitive.
    #[test]
    fn the_case_of_the_title_matters() {
        let mode = TitleMatch::Contains;
        assert!(title_matches("<host>/Login - Chromium", "Chromium", mode).expect("it matches"));
        assert!(
            !title_matches("<host>/Login - Chromium", "chromium", mode).expect("it does not match")
        );
    }

    /// The case of the pattern and the case of the name do not count: the
    /// machine of v2 is one that does not tell them apart.
    /// Measured on the binary of v2.0.19 on 2026-10-31.
    #[test]
    fn the_case_of_the_pattern_does_not_count() {
        assert!(name_matches("*.TXT", "b.txt"));
        assert!(name_matches("*.txt", "b.TXT"));
        assert!(name_matches("A*", "a.txt"));
    }

    /// A title that is not there is not there, and a text that is empty
    /// is not a text that every title contains.
    #[test]
    fn a_title_that_is_not_there_is_not_there() {
        let mode = TitleMatch::Contains;
        assert!(
            !title_matches("<host>/Login - Chromium", "Firefox", mode).expect("it does not match")
        );
        assert!(!title_matches("<host>/Login - Chromium", "", mode).expect("it does not match"));
    }
}



/// The answers of the helper of the tray, read on a thread of their own.
///
/// The thread owns the reader, so the wait for an answer is a receive with
/// a timeout instead of a read that cannot be given up on.
#[cfg(target_os = "linux")]
fn the_answers_of_the_tray(
    output: std::process::ChildStdout,
) -> std::sync::mpsc::Receiver<String> {
    use std::io::BufRead;
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut reader = std::io::BufReader::new(output);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if sender.send(line.trim_end().to_string()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    receiver
}

/// Whether the helper of the tray took what it was asked.
///
/// The answer of the helper is JSON: `{"ok": true}` when it took it, and
/// `{"ok": false, "error": "..."}` when it did not. The reason of the
/// helper is carried as it is: a tray of a session that has no watcher of
/// the StatusNotifierItem says so.
#[cfg(target_os = "linux")]
fn the_tray_took(answer: &str) -> Result<(), String> {
    if answer.contains("\"ok\": true") {
        return Ok(());
    }
    Err(format!("the tray of the session refused: {answer}"))
}

/// Where the program that puts the icon of the script in the tray is.
///
/// The two places of the helper of the window, in the same order: beside
/// the binary, and under the crate as it was compiled.
#[cfg(target_os = "linux")]
fn the_helper_of_the_tray() -> Result<std::path::PathBuf, String> {
    let mut tried: Vec<String> = Vec::new();
    if let Ok(program) = std::env::current_exe() {
        if let Some(folder) = program.parent() {
            let beside = folder.join("helper/ahk-tray.py");
            if beside.is_file() {
                return Ok(beside);
            }
            tried.push(beside.to_string_lossy().into_owned());
        }
    }
    let under = Path::new(env!("CARGO_MANIFEST_DIR")).join("helper/ahk-tray.py");
    if under.is_file() {
        return Ok(under);
    }
    tried.push(under.to_string_lossy().into_owned());
    Err(format!(
        "the program that puts the icon of the script in the tray is not where it was looked for: {}",
        tried.join(", ")
    ))
}