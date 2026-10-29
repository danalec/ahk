//! The lab box as a script target.
//!
//! A port of `src/ahk/box.py` from the archived Python implementation.
//!
//! Three primitives are not obvious, and all three were measured on the
//! lab box (CachyOS, COSMIC Wayland) on 2026-10-25:
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
//! ## The focus check
//!
//! The box is a Wayland session, and the compositor does not expose a
//! window list to D-Bus or to X11: the COSMIC services answer
//! `Introspect` with nothing, and the X server that runs beside it (the
//! rootless Xwayland of gamescope) reports an empty client list. So the
//! window-level questions cannot be answered for a native Wayland window,
//! and this module says so instead of answering `false`.
//!
//! What *is* answerable, and is what the focus check uses, is the screen
//! itself: the colour of a pixel, the place of a colour, and the position
//! of the pointer. Those three are read from the frame the compositor
//! draws, so they work for any window, Wayland or not.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ahk::keys::{MODIFIERS, resolve_name};
use ahk::desktop::{Processes, WindowAnswer};
use ahk::dll::{NativeCall, NativeResult};

use crate::keyboard::{ENUMERATION_WAIT, Keyboard, KeyboardError};
use crate::native::Libraries;
use crate::pointer::{PRESS_SETTLE, Pointer, PointerError};

/// A primitive could not be carried out.
#[derive(Debug, thiserror::Error)]
pub enum BoxError {
    #[error("{0}")]
    Failed(String),
    #[error("the keyboard: {0}")]
    Keyboard(#[from] KeyboardError),
    #[error("the pointer: {0}")]
    Pointer(#[from] PointerError),
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
            libraries: Libraries::new(),
            #[cfg(target_os = "linux")]
            raven: None,
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
            self.notes.push("opened a uinput keyboard for the keys".to_string());
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
        let size = image_size(&path).ok_or_else(|| BoxError::Failed("the capture has no size".into()))?;
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
            self.notes.push("no uid on this platform; the session environment is unknown".to_string());
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
                self.notes.push(format!("WAYLAND_DISPLAY={name} (XDG_RUNTIME_DIR={runtime_dir})"));
                found = true;
                break;
            }
        }
        if !found {
            self.notes.push(format!("no wayland socket in {runtime_dir}"));
        }
    }

    /// The call of a function of a library of Windows.
    ///
    /// The raven is the Windows of this box: it is a program that
    /// emulates one, and it runs the helper that does the call. It
    /// starts with the first call and lives until the desktop is
    /// dropped, because a program of its own is not cheap to start.
    #[cfg(target_os = "linux")]
    fn windows(&mut self, call: &mut NativeCall) -> Result<NativeResult, String> {
        if self.raven.is_none() {
            let paths = crate::raven::Paths::from_environment().ok_or_else(|| {
                "this box has no raven: the libraries of Windows cannot be called.".to_string()
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
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
        70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 89, 90, 91, 92,
        93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 108, 109, 119, 120, 121, 122, 123, 124, 125,
        126, 127, 130,
    ]
}

/// The uid of this process, when the platform has one.
fn uid() -> Option<u32> {
    #[cfg(unix)]
    {
        Some(unsafe { libc::getuid() })
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

impl ahk::desktop::Clipboard for LabDesktop {}

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
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, String> {
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
                other => ahk::keys::character_stroke(other).ok_or_else(|| {
                    format!("the keyboard of this box has no key for {other:?}")
                })?,
            };
            let modifiers: Vec<u16> = if shift { vec![42] } else { Vec::new() };
            self.keyboard()
                .map_err(|error| error.to_string())?
                .tap_with_modifiers(code, &modifiers)
                .map_err(|error| error.to_string())?;
            typed += 1;
        }
        Ok(format!("typed {typed} characters through the uinput keyboard"))
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
            self.keyboard().map_err(|error| error.to_string())?.tap(code).map_err(|error| error.to_string())?;
            return Ok(format!("pressed {name} (code {code}) through the uinput keyboard"));
        }
        self.keyboard()
            .map_err(|error| error.to_string())?
            .tap_with_modifiers(code, &modifier_codes)
            .map_err(|error| error.to_string())?;
        let names = modifiers.join("+");
        Ok(format!("pressed {names}+{name} (code {code}) through the uinput keyboard"))
    }
    fn key_down(&mut self, name: &str) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        self.keyboard().map_err(|error| error.to_string())?.down(code).map_err(|error| error.to_string())?;
        Ok(format!("{name} is down (code {code})"))
    }
    fn key_up(&mut self, name: &str) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        self.keyboard().map_err(|error| error.to_string())?.up(code).map_err(|error| error.to_string())?;
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
        let times = if count == 1 { String::new() } else { format!(" {count} times") };
        Ok(format!(
            "clicked ({x}, {y}){times} through the absolute pointer (screen {width}x{height})",
            width = size.0,
            height = size.1
        ))
    }
    fn mouse_down(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        if let (Some(x), Some(y)) = (x, y) {
            self.pointer().map_err(|error| error.to_string())?.move_to(x, y).map_err(|error| error.to_string())?;
            std::thread::sleep(PRESS_SETTLE);
        }
        self.pointer().map_err(|error| error.to_string())?.button_down(button).map_err(|error| error.to_string())?;
        Ok(format!("the {button} button is down"))
    }
    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        if let (Some(x), Some(y)) = (x, y) {
            self.pointer().map_err(|error| error.to_string())?.move_to(x, y).map_err(|error| error.to_string())?;
            std::thread::sleep(PRESS_SETTLE);
        }
        self.pointer().map_err(|error| error.to_string())?.button_up(button).map_err(|error| error.to_string())?;
        Ok(format!("the {button} button is up"))
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.pointer().map_err(|error| error.to_string())?.move_to(x, y).map_err(|error| error.to_string())?;
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
                let last = self.pointer().map(|pointer| pointer.last_position()).unwrap_or((0, 0));
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
        let command = format!("grim -g {} {}", shell_quote(&geometry), shell_quote(&path.to_string_lossy()));
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() || !path.exists() {
            return Err(format!("grim failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(path.to_string_lossy().to_string())
    }
    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        LabDesktop::screen_size(self).map_err(|error| error.to_string())
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
        let path = self.capture(left, top, width, height, &format!("ahk-search-{}.png", millis()))?;
        let found = search_pixel(&path, target);
        let _ = std::fs::remove_file(&path);
        Ok(found.map(|(x, y)| (x + left, y + top)))
    }
}

impl ahk::desktop::Windows for LabDesktop {
    fn window_active(&mut self, title: &str) -> Result<WindowAnswer, String> {
        let Some(active) = self.active_window_title()? else {
            return Ok(WindowAnswer::Unknown(
                "no X11 window is active, and a native Wayland window that holds the focus is \
                 invisible to X11. Use PixelSearch or MouseGetPos with PixelGet to check the \
                 focus."
                    .to_string(),
            ));
        };
        if title_matches(&active, title) {
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
    fn window_exists(&mut self, title: &str) -> Result<WindowAnswer, String> {
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
        let display = x_display();
        let Some(display) = display else {
            return Err(
                "there is no X server on this box to ask: the session is Wayland, and the \
                 compositor exposes no window list to D-Bus."
                    .to_string(),
            );
        };
        let command = format!("DISPLAY={display} xprop -root _NET_ACTIVE_WINDOW 2>/dev/null || true");
        let output = self.run(&command, "")?;
        let text = output.1.trim().to_string();
        if text.is_empty() {
            return Ok(None);
        }
        let Some(id) = text.rsplit('#').next().map(str::trim).filter(|id| !id.is_empty()) else {
            return Ok(None);
        };
        if id == "0x0" {
            return Ok(None);
        }
        let command = format!("DISPLAY={display} xprop -id {id} _NET_WM_NAME WM_NAME 2>/dev/null || true");
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

impl ahk::desktop::Processes for LabDesktop {
    fn env_get(&mut self, name: &str) -> Result<String, String> {
        Ok(std::env::var(name).unwrap_or_default())
    }

    fn env_set(&mut self, name: &str, value: &str) -> Result<String, String> {
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
}


/// The files of a folder that match a pattern.
///
/// The mode is v2's: "R" walks into the subfolders as well, and "D"
/// gives the directories instead of the files.
pub(crate) fn walk_the_folder(folder: &Path, pattern: &str, mode: &str, files: &mut Vec<String>) -> Result<(), String> {
    let entries = std::fs::read_dir(folder)
        .map_err(|error| format!("the folder {folder:?} cannot be read: {error}"))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("the folder {folder:?} cannot be read: {error}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let is_file = entry.path().is_file();
        // The directories are not files: a loop over the files of a
        // folder does not walk into them, unless the mode asks for the
        // directories instead.
        let wanted = if mode.contains('D') { !is_file } else { is_file };
        if wanted && name_matches(pattern, &name) {
            files.push(entry.path().to_string_lossy().to_string());
        }
        if mode.contains('R') && !is_file {
            walk_the_folder(&entry.path(), pattern, mode, files)?;
        }
    }
    Ok(())
}

/// The X display to ask, when there is one.
///
/// The session is Wayland, and the X server beside it is the rootless
/// Xwayland of gamescope, whose socket is `/tmp/.X11-unix/X0`.
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

/// Whether a title matches the one asked for.
///
/// AutoHotkey matches on the window title, and a script that wants an
/// exact one says so; here the match is the same way: the wanted text has
/// to be inside the title, and the case does not matter.
fn title_matches(title: &str, wanted: &str) -> bool {
    if wanted.is_empty() {
        return false;
    }
    title.to_lowercase().contains(&wanted.to_lowercase())
}

/// Parse a colour: `#rrggbb`, `#rgb`, or `rrggbb`.
fn parse_colour(colour: &str) -> Result<[u8; 3], String> {
    let text = colour.trim().trim_start_matches('#').trim_start_matches("0x");
    let parse = |part: &str| -> Result<u8, String> { u8::from_str_radix(part, 16).map_err(|error| error.to_string()) };
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
            Ok([expand(&text[0..1])?, expand(&text[1..2])?, expand(&text[2..3])?])
        }
        _ => Err(format!("{colour:?} is not a colour: write it as #rrggbb")),
    }
}

/// The milliseconds since the epoch, for the scratch file names.
fn millis() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
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
    let file = std::fs::File::open(Path::new(path)).map_err(|error| error.to_string())?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
    let mut buffer = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).map_err(|error| error.to_string())?;
    if info.width == 0 || info.height == 0 {
        return Err("the capture is empty".to_string());
    }
    let bytes = &buffer[..info.buffer_size()];
    let (red, green, blue) = match info.color_type {
        png::ColorType::Rgb => (bytes[0], bytes[1], bytes[2]),
        png::ColorType::Rgba => (bytes[0], bytes[1], bytes[2]),
        png::ColorType::Grayscale => (bytes[0], bytes[0], bytes[0]),
        png::ColorType::GrayscaleAlpha => (bytes[0], bytes[0], bytes[0]),
        png::ColorType::Indexed => {
            let palette = reader.info().palette.as_ref().ok_or("no palette")?;
            let index = bytes[0] as usize * 3;
            (palette[index], palette[index + 1], palette[index + 2])
        }
    };
    Ok(format!("#{red:02x}{green:02x}{blue:02x}"))
}

/// Find a colour in an image; the position is relative to the image.
fn search_pixel(path: &str, target: [u8; 3]) -> Option<(i64, i64)> {
    let file = std::fs::File::open(Path::new(path)).ok()?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().ok()?;
    let mut buffer = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).ok()?;
    let width = info.width as usize;
    let height = info.height as usize;
    let bytes = &buffer[..info.buffer_size()];
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Indexed => 1,
    };
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * channels;
            if offset + channels > bytes.len() {
                return None;
            }
            let (red, green, blue) = match info.color_type {
                png::ColorType::Rgb => (bytes[offset], bytes[offset + 1], bytes[offset + 2]),
                png::ColorType::Rgba => (bytes[offset], bytes[offset + 1], bytes[offset + 2]),
                png::ColorType::Grayscale => (bytes[offset], bytes[offset], bytes[offset]),
                png::ColorType::GrayscaleAlpha => (bytes[offset], bytes[offset], bytes[offset]),
                png::ColorType::Indexed => {
                    let palette = reader.info().palette.as_ref()?;
                    let index = bytes[offset] as usize * 3;
                    (palette[index], palette[index + 1], palette[index + 2])
                }
            };
            if (red, green, blue) == (target[0], target[1], target[2]) {
                return Some((x as i64, y as i64));
            }
        }
    }
    None
}

/// The modifier names, for the report.
pub fn modifier_names() -> Vec<String> {
    MODIFIERS.iter().map(|(name, _)| (*name).to_string()).collect()
}

/// Whether a name matches a pattern with the stars and the question
/// marks of a shell glob.
///
/// The pattern of a `Loop Files` is a glob: `*` is any number of
/// characters and `?` is one. The matcher is the same in every
/// language, so it is written out here.
pub(crate) fn name_matches(pattern: &str, name: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let name: Vec<char> = name.chars().collect();
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
