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
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ahk::desktop::{
    FileAnswer, FileOptions, Folders, Idle, InputAnswer, InputOptions, Processes, WindowAnswer,
};
use ahk::dll::{NativeCall, NativeFailure, NativeResult};
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
    /// The client of the compositor, when the session is Wayland. It is
    /// opened with the first question about the windows and kept: the
    /// connection to the compositor is worth keeping.
    #[cfg(target_os = "linux")]
    wayland: Option<crate::wayland::Client>,
}

impl Default for LabDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl LabDesktop {
    /// A desktop that has not yet opened anything.
    pub fn new() -> Self {
        Self {
            keyboard: None,
            pointer: None,
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

        // The X server of this box is the rootless Xwayland of gamescope,
        // and it reports an empty client list: there is no way to ask it
        // which windows exist.
        let listing = self.run("xprop -root _NET_CLIENT_LIST 2>/dev/null || true", "")?;
        let windows = listing.1.trim();
        if windows.is_empty() || windows.ends_with("window id #") {
            return Ok(WindowAnswer::Unknown(
                "the X server on this box reports an empty client list (it is the rootless \
                 Xwayland of gamescope), and the compositor exposes no window list to D-Bus: a \
                 native Wayland window cannot be found this way. Use PixelSearch to check what \
                 is on the screen."
                    .to_string(),
            ));
        }
        if title.is_empty() {
            return Ok(WindowAnswer::NotActive);
        }
        Ok(WindowAnswer::Unknown(format!(
            "the X server lists {windows:?}, which this runtime does not parse yet"
        )))
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
}

impl ahk::desktop::Sound for LabDesktop {
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

impl ahk::desktop::Machine for LabDesktop {
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
        if title.is_empty() {
            return Ok(empty_title());
        }
        let active = self.wayland()?.active()?;
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
}

/// The windows of a Wayland session, on a platform that has no Wayland.
///
/// The protocols of the compositor are Linux ones, and the client of the
/// compositor is a Unix program. The questions therefore fail with the
/// reason, and the box says that it is not Linux.
#[cfg(not(target_os = "linux"))]
impl LabDesktop {
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
/// runtime does not keep one.
#[cfg(target_os = "linux")]
fn empty_title() -> WindowAnswer {
    WindowAnswer::Unknown(
        "the title is empty, and v2 would use the last found window: this runtime does not keep \
         one."
            .to_string(),
    )
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
