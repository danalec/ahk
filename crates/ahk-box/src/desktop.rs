//! The lab box as a script target.
//!
//! A port of `src/yoko_ahk_mcp/box.py` from the Python `yoko-ahk-mcp`.
//!
//! Three primitives are not obvious, and all three were measured on the
//! lab box (CachyOS, COSMIC Wayland, ydotool 1.0.4) on 2026-10-25:
//!
//! * **Text** goes through `ydotool type`. It is exact: every character
//!   arrives once, in order.
//! * **Keys** go through a uinput keyboard of our own. `ydotool key`
//!   cannot be used: ydotool 1.x reads a key *name* as zero and does
//!   nothing, and a numeric code presses the key without ever releasing
//!   it, so the compositor's auto-repeat types the key forever.
//! * **Modifiers** are the one thing our device does not deliver to the
//!   client: on this box a modifier key never reaches the focused
//!   surface, whatever device it comes from. A key with a modifier is
//!   therefore reported as *not delivered* rather than quietly doing
//!   nothing.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ahk::keys::{MODIFIERS, resolve_name};
use ahk::runtime::Desktop;

use crate::keyboard::{ENUMERATION_WAIT, Keyboard, KeyboardError};

/// A primitive could not be carried out.
#[derive(Debug, thiserror::Error)]
pub enum BoxError {
    #[error("{0}")]
    Failed(String),
    #[error("the keyboard: {0}")]
    Keyboard(#[from] KeyboardError),
}

/// The lab box, as the runtime sees it.
pub struct LabDesktop {
    keyboard: Option<Keyboard>,
    notes: Vec<String>,
}

impl Default for LabDesktop {
    fn default() -> Self {
        Self::new()
    }
}

impl LabDesktop {
    /// A desktop that has not yet opened anything.
    pub fn new() -> Self {
        Self { keyboard: None, notes: Vec::new() }
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
}

/// The codes the device announces.
///
/// A keyboard node that claims every code in the world is what makes
/// libinput treat a *pointer* as a keyboard, so the list is the keys this
/// lab uses and nothing else.
fn announced_codes() -> Vec<u16> {
    let mut codes: Vec<u16> = vec![
        1, 14, 15, 28, 57, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 29, 42, 56, 125, 97, 100, 126,
        58, 87, 88, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
    ];
    // The letter and digit rows, in the evdev numbering.
    codes.extend((16u16..=53).filter(|code| {
        matches!(code, 16..=25 | 30..=38 | 39..=41 | 43..=53)
    }));
    codes.sort_unstable();
    codes.dedup();
    codes
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

impl Desktop for LabDesktop {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        if text.is_empty() {
            return Ok(String::new());
        }
        let command = format!("ydotool type -- {}", shell_quote(text));
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!("ydotool type failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(format!("typed {} characters", text.chars().count()))
    }

    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        let code = resolve_name(name).map_err(|error| error.to_string())?;
        if !modifiers.is_empty() {
            // A modifier does not reach the client on this box. Say so
            // instead of pretending the chord happened.
            let note = format!(
                "{}+{name}: the modifier does not reach the client on this box",
                modifiers.join("+")
            );
            self.notes.push(note.clone());
            return Ok(format!("{note} NOT delivered (modifier unsupported here)"));
        }
        self.keyboard().map_err(|error| error.to_string())?.tap(code).map_err(|error| error.to_string())?;
        Ok(format!("pressed {name} (code {code})"))
    }

    fn click(&mut self, x: i64, y: i64, button: &str) -> Result<String, String> {
        // The absolute pointer is the Python project's path; here the click
        // goes through ydotool, which moves the pointer relatively and does
        // not reach a client surface either.
        let code = match button {
            "right" => "0x02",
            "middle" => "0x04",
            _ => "0x01",
        };
        let command =
            format!("ydotool mousemove --absolute -- {x} {y} && ydotool click {code}");
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!("the click failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(format!("clicked ({x}, {y}) through ydotool (the position is approximate)"))
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

    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String> {
        let name = if filename.is_empty() {
            format!("yoko-ahk-{}.png", millis())
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

    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        let path = self.capture(x, y, 1, 1, &format!("yoko-ahk-pixel-{}.png", millis()))?;
        let colour = read_pixel(&path)?;
        let _ = std::fs::remove_file(&path);
        Ok(colour)
    }
}

/// The milliseconds since the epoch, for the scratch file names.
fn millis() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
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

/// The modifier names, for the report.
pub fn modifier_names() -> Vec<String> {
    MODIFIERS.iter().map(|(name, _)| (*name).to_string()).collect()
}
