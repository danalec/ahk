//! The settings of the pointer and of the keyboard.
//!
//! v2 keeps them in the thread that runs: the coordinate modes, the
//! delays of the keys and of the mouse, the method of the send, the
//! speed of the pointer, the mode of the caps lock, the encoding of the
//! files and the mask of the menu.
//!
//! The runtime has no field for them and the tree runs one script in one
//! thread, so the state lives here, in the module that answers it. The
//! numbers are in atomics: the crate runs without the standard library,
//! and there is no `Mutex` of it to keep a number behind.
//!
//! The numbers of v2 were measured on the binary of v2.0.19 on
//! 2026-10-28: the speed of the pointer is from 0 to 100 and its default
//! is 2, the delay of the mouse is 10 and the one of the keys is 10, the
//! method of the send is `Input`, and the mask of the menu is the Escape
//! key spelled `vk11sc01D`.

use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicI32, AtomicU8, Ordering};
#[cfg(test)]
use core::sync::atomic::AtomicBool;

use crate::keys;
use crate::runtime::{self, Runtime, RuntimeError, TypeName, Value};

/// The coordinate modes, the way v2 names them.
const CLIENT: u8 = 0;
const SCREEN: u8 = 1;
const WINDOW: u8 = 2;
const MODES: [&str; 3] = ["Client", "Screen", "Window"];

/// The targets of `CoordMode`, the way v2 names them.
const MOUSE: u8 = 0;
const PIXEL: u8 = 1;
const CARET: u8 = 2;
const MENU: u8 = 3;
const TOOLTIP: u8 = 4;

/// The methods of the send, the way v2 names them.
const INPUT: u8 = 0;
const EVENT: u8 = 1;
const PLAY: u8 = 2;
const INPUT_THEN_PLAY: u8 = 3;
const SEND_MODES: [&str; 4] = ["Input", "Event", "Play", "InputThenPlay"];

/// The encodings of the files that the tree carries.
///
/// The default of v2 is `CP0`, the code page of the machine, and v2
/// resolves it to the code page of the machine: on the machine of the
/// measurement it was `CP1252`. The box has no code pages, so the tree
/// carries the default and the two that name a byte order, with the
/// `-RAW` forms: the suffix says that the file has no byte order mark.
///
/// Measured on the binary of v2.0.19 on 2026-10-28: the variable keeps
/// the suffix (`FileEncoding "UTF-8-RAW"` leaves `A_FileEncoding` at
/// `UTF-8-RAW`), `FileEncoding "CP0"` is carried, and `FileEncoding
/// "ANSI"`, `FileEncoding "OEM"` and `FileEncoding "UTF8"` are refused.
const ENCODINGS: [&str; 5] = ["CP0", "UTF-8", "UTF-8-RAW", "UTF-16", "UTF-16-RAW"];

/// The key that closes a menu.
///
/// The default of v2 is the Escape key, and v2 spells it with the two
/// numbers of it: `vk11sc01D`. A script changes it by writing the
/// built-in variable, and the tree does not write built-in variables.
const MENU_MASK_KEY: &str = "vk11sc01D";

/// The reason the mode of the caret is not answered.
///
/// The caret is a part of a window, and the coordinates of a caret are
/// the coordinates of its window.
const NO_CARET: &str =
    "the caret belongs to a window, and this runtime has no windows: the mode of the caret is not answered.";

/// The reason the mode of the menu is not answered.
///
/// The menu is a window of its own, and the coordinates of a menu are
/// the coordinates of that window.
const NO_MENU: &str =
    "the menu is a window of its own, and this runtime has no windows: the mode of the menu is not answered.";

/// The reason the mode of the tooltip is not answered.
///
/// The tooltip is a window of its own, and the coordinates of a tooltip
/// are the coordinates of that window.
const NO_TOOLTIP: &str = "the tooltip is a window of its own, and this runtime has no windows: the mode of the tooltip is \
                           not answered.";

/// The reason the play method is not carried.
///
/// The play method of v2 does not send keystrokes: it posts the
/// messages to the active window. There is no window here to post them
/// to.
const NO_PLAY: &str = "the play method of v2 posts the messages to the active window, and there is no window here to post \
                       them to.";

/// The state of the settings.
///
/// The runtime has no field for them and the tree runs one thread, so
/// they live here, in the module that answers them.
struct State {
    coord_mouse: AtomicU8,
    coord_pixel: AtomicU8,
    key_delay: AtomicI32,
    key_delay_play: AtomicI32,
    key_duration: AtomicI32,
    key_duration_play: AtomicI32,
    mouse_delay: AtomicI32,
    mouse_delay_play: AtomicI32,
    send_mode: AtomicU8,
    send_level: AtomicI32,
    mouse_speed: AtomicI32,
    caps_lock: AtomicI32,
    encoding: AtomicU8,
}

/// The turn of the tests: the state is one for the whole crate.
///
/// The settings live in a static, so two tests that touch them cannot
/// run at the same time. The guard is here and not in the tests of this
/// module because the files and the variables read the same state.
#[cfg(test)]
pub(crate) struct Turn;

#[cfg(test)]
static THE_TURN: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
impl Turn {
    /// Take the turn, and wait for it while another test has it.
    pub(crate) fn take() -> Self {
        while THE_TURN.swap(true, Ordering::Relaxed) {
            core::hint::spin_loop();
        }
        Turn
    }
}

#[cfg(test)]
impl Drop for Turn {
    fn drop(&mut self) {
        THE_TURN.store(false, Ordering::Relaxed);
    }
}

/// The state of the settings, with the defaults of v2.
static STATE: State = State {
    coord_mouse: AtomicU8::new(CLIENT),
    coord_pixel: AtomicU8::new(CLIENT),
    key_delay: AtomicI32::new(10),
    key_delay_play: AtomicI32::new(-1),
    key_duration: AtomicI32::new(-1),
    key_duration_play: AtomicI32::new(-1),
    mouse_delay: AtomicI32::new(10),
    mouse_delay_play: AtomicI32::new(-1),
    send_mode: AtomicU8::new(INPUT),
    send_level: AtomicI32::new(0),
    mouse_speed: AtomicI32::new(2),
    caps_lock: AtomicI32::new(1),
    encoding: AtomicU8::new(0),
};

/// The value of the built-in variable of the settings of this name.
///
/// The name is the one of the table of the variables: in lower case and
/// without the `A_` prefix. `None` means the name is not one of the
/// settings of this module. `Some(Err(reason))` means the name is one
/// of them and the tree cannot answer it: the reason says why.
pub fn value_of(name: &str) -> Option<Result<Value, String>> {
    let value = match name {
        "coordmodemouse" => mode_text(STATE.coord_mouse.load(Ordering::Relaxed)),
        "coordmodepixel" => mode_text(STATE.coord_pixel.load(Ordering::Relaxed)),
        "coordmodecaret" => return Some(Err(NO_CARET.to_string())),
        "coordmodemenu" => return Some(Err(NO_MENU.to_string())),
        "coordmodetooltip" => return Some(Err(NO_TOOLTIP.to_string())),
        "defaultmousespeed" => Value::Int(STATE.mouse_speed.load(Ordering::Relaxed) as i64),
        "fileencoding" => Value::Text(encoding_text()),
        "keydelay" => Value::Int(STATE.key_delay.load(Ordering::Relaxed) as i64),
        "keydelayplay" => Value::Int(STATE.key_delay_play.load(Ordering::Relaxed) as i64),
        "keyduration" => Value::Int(STATE.key_duration.load(Ordering::Relaxed) as i64),
        "keydurationplay" => Value::Int(STATE.key_duration_play.load(Ordering::Relaxed) as i64),
        "listlines" => Value::Int(1),
        "menumaskkey" => Value::Text(MENU_MASK_KEY.to_string()),
        "mousedelay" => Value::Int(STATE.mouse_delay.load(Ordering::Relaxed) as i64),
        "mousedelayplay" => Value::Int(STATE.mouse_delay_play.load(Ordering::Relaxed) as i64),
        "sendlevel" => Value::Int(STATE.send_level.load(Ordering::Relaxed) as i64),
        "sendmode" => Value::Text(send_mode_text()),
        "storecapslockmode" => Value::Int(STATE.caps_lock.load(Ordering::Relaxed) as i64),
        "allowmainwindow" => Value::Int(1),
        _ => return None,
    };
    Some(Ok(value))
}

/// The name of a coordinate mode.
fn mode_text(mode: u8) -> Value {
    Value::Text(MODES[mode as usize].to_string())
}

/// The name of the method of the send.
fn send_mode_text() -> String {
    SEND_MODES[STATE.send_mode.load(Ordering::Relaxed) as usize].to_string()
}

/// The name of the encoding of the files, as `A_FileEncoding` spells it.
fn encoding_text() -> String {
    file_encoding().to_string()
}

/// The name of the encoding of the files, as `A_FileEncoding` spells it.
///
/// The state is in this module: the files of the machine ask it here for
/// the encoding they are opened with, and there is one truth about it.
pub fn file_encoding() -> &'static str {
    ENCODINGS[STATE.encoding.load(Ordering::Relaxed) as usize]
}

/// The encoding the bytes of the files are written in.
///
/// The default of v2 is the code page of the machine, and the box has no
/// code page: the tree writes UTF-8, the encoding that carries every
/// character it can write.
pub fn file_bytes_encoding() -> &'static str {
    if file_encoding().starts_with("UTF-16") { "UTF-16" } else { "UTF-8" }
}

/// `CoordMode`: the way the coordinates are read.
///
/// The mouse and the pixels are the two that a box without windows can
/// have. The caret, the menu and the tooltip belong to a window, and
/// there is no window here to be relative to.
///
/// The second parameter says what the coordinates are relative to, and
/// it is the screen when it is not said.
pub fn coord_mode(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    let relative = text_of(values, 1);
    let relative = if relative.is_empty() { "Screen".to_string() } else { relative };
    // The spelling of the two parameters is checked first: v2 says which
    // parameter is invalid before it says that the mode is not carried.
    let target_kind = match target.to_lowercase().as_str() {
        "mouse" => MOUSE,
        "pixel" => PIXEL,
        "caret" => CARET,
        "menu" => MENU,
        "tooltip" => TOOLTIP,
        _ => {
            return Err(RuntimeError {
                message: runtime::invalid_parameter(1, "CoordMode"),
                line,
            })
        }
    };
    let mode = match relative.to_lowercase().as_str() {
        "screen" => SCREEN,
        "window" => WINDOW,
        "client" => CLIENT,
        _ => {
            return Err(RuntimeError {
                message: runtime::invalid_parameter(2, "CoordMode"),
                line,
            })
        }
    };
    match target_kind {
        MOUSE => {
            let previous = STATE.coord_mouse.swap(mode, Ordering::Relaxed);
            Ok(mode_text(previous))
        }
        PIXEL => {
            let previous = STATE.coord_pixel.swap(mode, Ordering::Relaxed);
            Ok(mode_text(previous))
        }
        CARET => Err(RuntimeError {
            message: NO_CARET.to_string(),
            line,
        }),
        MENU => Err(RuntimeError {
            message: NO_MENU.to_string(),
            line,
        }),
        _ => Err(RuntimeError {
            message: NO_TOOLTIP.to_string(),
            line,
        }),
    }
}

/// `SetMouseDelay`: the pause after a movement of the mouse.
///
/// The delay of the `Play` method is the one of `SendPlay`, and the
/// tree does not post the messages of that method: the delay is carried
/// and reported, and the pause between two movements is the one of the
/// pointer of the box.
pub fn set_mouse_delay(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = number_argument(values, 0, "SetMouseDelay", line)? as i64;
    let mode = text_of(values, 1);
    let play = match mode.to_lowercase().as_str() {
        "" => false,
        "play" => true,
        _ => {
            return Err(RuntimeError {
                message: runtime::invalid_parameter(2, "SetMouseDelay"),
                line,
            })
        }
    };
    if play {
        let previous = STATE.mouse_delay_play.swap(delay as i32, Ordering::Relaxed);
        return Ok(Value::Int(previous as i64));
    }
    let previous = STATE.mouse_delay.swap(delay as i32, Ordering::Relaxed);
    // The pointer of the box pauses between two movements.
    rt.desktop()
        .set_mouse_delay(delay)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(previous as i64))
}

/// `SetDefaultMouseSpeed`: the speed of the pointer.
///
/// The speed of v2 is from 0 to 100, and 0 moves the pointer at once.
/// The pointer of the box moves in steps, and the speed is the number of
/// them.
pub fn set_default_mouse_speed(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let speed = number_argument(values, 0, "SetDefaultMouseSpeed", line)?;
    if !(0.0..=100.0).contains(&speed) {
        return Err(RuntimeError {
            message: runtime::invalid_parameter(1, "SetDefaultMouseSpeed"),
            line,
        });
    }
    let speed = speed as i64;
    let previous = STATE.mouse_speed.swap(speed as i32, Ordering::Relaxed);
    rt.desktop()
        .set_mouse_speed(speed)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(previous as i64))
}

/// `SetStoreCapsLockMode`: whether the state of the caps lock is put back.
///
/// v2 has no boolean of its own: a number is true when it is not zero,
/// and a text is true when it is not empty.
pub fn set_store_caps_lock_mode(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let setting = values.first().map(|value| value.truthy()).unwrap_or(false);
    let previous = STATE.caps_lock.swap(setting as i32, Ordering::Relaxed);
    Ok(Value::Int(previous as i64))
}

/// `GetKeyName`: the name of the key.
///
/// The name is the one v2 gives the key. A name the tree does not know
/// is the empty string, the way v2 answers a name it does not know.
pub fn get_key_name(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(key_name(&text_of(values, 0))))
}

/// `GetKeySC`: the code of the Linux input event of the key.
///
/// The code of the event is the one the box writes to its device. It is
/// not the scan code of the PC keyboard: the two numberings part company
/// on the extended keys.
pub fn get_key_sc(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(key_event(&text_of(values, 0)) as i64))
}

/// `GetKeyVK`: the virtual key code of Windows of the key.
pub fn get_key_vk(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(key_code(&text_of(values, 0)) as i64))
}

/// `SendText`: the text, typed as it stands.
///
/// The braces of v2 are not read here: the text goes to the keyboard as
/// it is written, one character at a time.
pub fn send_text(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let text = rt.interpolate(&text_of(values, 0), line)?;
    rt.desktop()
        .type_text(&text)
        .map(Value::Text)
        .map_err(|reason| RuntimeError { message: reason, line })
}

/// `SendEvent`: the keys, sent the way the box sends them.
///
/// The event method of v2 is the method of Windows, and the tree has one
/// keyboard: the keys go through it, the way `Send` sends them.
pub fn send_event(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    send_keys(rt, values, line)
}

/// `SendInput`: the keys, sent the way the box sends them.
///
/// The input method of v2 is the method of the keyboard of the box: the
/// keys go through it, the way `Send` sends them.
pub fn send_input(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    send_keys(rt, values, line)
}

/// `SendPlay`: the messages of a window.
///
/// The play method of v2 does not send keystrokes: it posts the
/// messages to the active window, and there is no window here.
pub fn send_play(_rt: &mut Runtime<'_>, _values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: NO_PLAY.to_string(),
        line,
    })
}

/// `SendLevel`: the level of the events the script sends.
///
/// The level tells the hotkeys which of the artificial events they are
/// to ignore. The tree has no hotkeys yet, so the level is carried and
/// reported, and nothing reads it yet.
pub fn send_level(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let level = number_argument(values, 0, "SendLevel", line)?;
    if !(0.0..=100.0).contains(&level) {
        return Err(RuntimeError {
            message: runtime::invalid_parameter(1, "SendLevel"),
            line,
        });
    }
    let previous = STATE.send_level.swap(level as i32, Ordering::Relaxed);
    Ok(Value::Int(previous as i64))
}

/// The keys of a spec, sent the way the tree sends them.
///
/// The tree sends the keys with one `Send`, so the names of v2 that
/// send keys -- `SendEvent`, `SendInput` and `Send` -- send them the
/// same way.
fn send_keys(rt: &mut Runtime<'_>, values: &[Value], line: u32) -> Result<Value, RuntimeError> {
    let send = crate::builtins::find("send").expect("the tree carries Send");
    (send.handler)(rt, values, line, send.variant)
}

/// The method of the send, when the tree carries it.
///
/// The two methods that deliver the keys as events are carried: the
/// keyboard of the box is the input method of v2. The two that fall back
/// to the play method are not: they post the messages to a window.
pub fn set_send_mode(mode: &str) -> Result<&'static str, String> {
    let wanted = mode.trim().to_lowercase();
    let found = match wanted.as_str() {
        "input" => INPUT,
        "event" => EVENT,
        "play" => PLAY,
        "inputthenplay" => INPUT_THEN_PLAY,
        _ => return Err(runtime::invalid_parameter(1, "SendMode")),
    };
    // The two methods that fall back to the play method post the
    // messages to a window, and there is no window here.
    if found == PLAY || found == INPUT_THEN_PLAY {
        return Err(NO_PLAY.to_string());
    }
    STATE.send_mode.store(found, Ordering::Relaxed);
    Ok(SEND_MODES[found as usize])
}

/// `SetKeyDelay`: the pause between the keys.
///
/// The gap between the keys of the box is the one that was measured on
/// the lab box on 2026-10-25, and it is the gap that makes every key of
/// a burst count. The delay of v2 is carried and reported, and the gap
/// of the keyboard is the one that is used.
pub fn set_key_delay(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = number_argument(values, 0, "SetKeyDelay", line)? as i64;
    let mode = text_of(values, 1);
    let play = match mode.to_lowercase().as_str() {
        "" => false,
        "play" => true,
        _ => {
            return Err(RuntimeError {
                message: runtime::invalid_parameter(2, "SetKeyDelay"),
                line,
            })
        }
    };
    Ok(Value::Int(key_delay_of(delay, play)))
}

/// The delay of the keys, and the one that was there before.
fn key_delay_of(delay: i64, play: bool) -> i64 {
    let delay = delay as i32;
    if play {
        STATE.key_delay_play.swap(delay, Ordering::Relaxed) as i64
    } else {
        STATE.key_delay.swap(delay, Ordering::Relaxed) as i64
    }
}

/// `FileEncoding`: the encoding the files are opened with.
///
/// The name has to be one of the encodings of the tree: a file read
/// with an encoding the tree does not know is a file read as something
/// else. The box has no code pages, so a name of a code page -- `CP1252`,
/// the one v2 accepts and resolves -- is refused with the message v2
/// gives for a name it does not know.
///
/// The name is carried the way v2 carries it: `utf-8` becomes `UTF-8`,
/// and the suffix `-RAW` is kept.
pub fn set_file_encoding(name: &str) -> Result<&'static str, String> {
    let wanted = name.trim().to_uppercase();
    let found = ENCODINGS
        .iter()
        .position(|encoding| *encoding == wanted.as_str())
        .ok_or_else(|| runtime::invalid_parameter(1, "FileEncoding"))?;
    STATE.encoding.store(found as u8, Ordering::Relaxed);
    Ok(ENCODINGS[found])
}

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values.get(index).map(|value| value.to_text()).unwrap_or_default()
}

/// The number of the argument at this place, as the maths wants it.
///
/// A string that holds a number is a number; a string that holds
/// anything else is not one, and the message names the parameter and
/// the kind that arrived, the way v2 names them.
fn number_argument(
    values: &[Value],
    index: usize,
    name: &str,
    line: u32,
) -> Result<f64, RuntimeError> {
    let value = values.get(index).cloned().unwrap_or(Value::Int(0));
    value.to_number_strict().map_err(|_| RuntimeError {
        message: runtime::requires(index + 1, name, TypeName::Number, &value),
        line,
    })
}

// -- the keys -------------------------------------------------------------------

/// One key of the tree.
///
/// The name is the one v2 gives the key, the event is the name of the
/// Linux input event, and the code is the virtual key code of Windows.
struct Key {
    /// The name v2 gives the key.
    name: String,
    /// The code of the Linux input event.
    event: u16,
    /// The virtual key code of Windows.
    code: u32,
}

/// The keys the tree carries, by the name v2 gives them.
const KEYS: &[(&str, &str, u32)] = &[
    ("Escape", "ESC", 0x1b),
    ("Space", "SPACE", 0x20),
    ("Tab", "TAB", 0x09),
    ("Enter", "ENTER", 0x0d),
    ("Backspace", "BACKSPACE", 0x08),
    ("CapsLock", "CAPSLOCK", 0x14),
    ("Numlock", "NUMLOCK", 0x90),
    ("ScrollLock", "SCROLLLOCK", 0x91),
    ("PrintScreen", "SYSRQ", 0x2c),
    ("Pause", "PAUSE", 0x13),
    ("Delete", "DELETE", 0x2e),
    ("Insert", "INSERT", 0x2d),
    ("Home", "HOME", 0x24),
    ("End", "END", 0x23),
    ("PgUp", "PAGEUP", 0x21),
    ("PgDn", "PAGEDOWN", 0x22),
    ("Up", "UP", 0x26),
    ("Down", "DOWN", 0x28),
    ("Left", "LEFT", 0x25),
    ("Right", "RIGHT", 0x27),
    ("Control", "LEFTCTRL", 0x11),
    ("Shift", "LEFTSHIFT", 0x10),
    ("Alt", "LEFTALT", 0x12),
    ("LWin", "LEFTMETA", 0x5b),
    ("RWin", "RIGHTMETA", 0x5c),
    ("RControl", "RIGHTCTRL", 0xa3),
    ("LShift", "LEFTSHIFT", 0xa0),
    ("LAlt", "LEFTALT", 0xa4),
    ("RAlt", "RIGHTALT", 0xa5),
    ("Numpad0", "KP0", 0x60),
    ("Numpad1", "KP1", 0x61),
    ("Numpad2", "KP2", 0x62),
    ("Numpad3", "KP3", 0x63),
    ("Numpad4", "KP4", 0x64),
    ("Numpad5", "KP5", 0x65),
    ("Numpad6", "KP6", 0x66),
    ("Numpad7", "KP7", 0x67),
    ("Numpad8", "KP8", 0x68),
    ("Numpad9", "KP9", 0x69),
    ("NumpadEnter", "KPENTER", 0x0d),
    ("NumpadAdd", "KPPLUS", 0x6b),
    ("NumpadSub", "KPMINUS", 0x6d),
    ("NumpadDiv", "KPSLASH", 0x6f),
    ("NumpadDot", "KPDOT", 0x6e),
    ("NumpadClear", "KP5", 0x0c),
    ("F1", "F1", 0x70),
    ("F2", "F2", 0x71),
    ("F3", "F3", 0x72),
    ("F4", "F4", 0x73),
    ("F5", "F5", 0x74),
    ("F6", "F6", 0x75),
    ("F7", "F7", 0x76),
    ("F8", "F8", 0x77),
    ("F9", "F9", 0x78),
    ("F10", "F10", 0x79),
    ("Volume_Mute", "MUTE", 0xad),
    ("Volume_Down", "VOLUMEDOWN", 0xae),
    ("Volume_Up", "VOLUMEUP", 0xaf),
    ("-", "MINUS", 0xbd),
    ("=", "EQUAL", 0xbb),
    ("[", "LEFTBRACE", 0xdb),
    ("]", "RIGHTBRACE", 0xdd),
    (";", "SEMICOLON", 0xba),
    ("'", "APOSTROPHE", 0xde),
    ("`", "GRAVE", 0xc0),
    ("\\", "BACKSLASH", 0xdc),
    (",", "COMMA", 0xbc),
    (".", "DOT", 0xbe),
    ("/", "SLASH", 0xbf),
];

/// The modifiers, by the names a script writes for them.
///
/// The tree takes the generic name and the name of the left key for the
/// same key -- the box has one control, on the left -- and v2 has a name
/// for each of them. The name the script wrote is the name it gets.
const MODIFIERS: &[(&str, &str, &str, u32)] = &[
    ("ctrl", "Control", "LEFTCTRL", 0x11),
    ("control", "Control", "LEFTCTRL", 0x11),
    ("lctrl", "LControl", "LEFTCTRL", 0xa2),
    ("lcontrol", "LControl", "LEFTCTRL", 0xa2),
    ("ctrlleft", "LControl", "LEFTCTRL", 0xa2),
    ("rctrl", "RControl", "RIGHTCTRL", 0xa3),
    ("rcontrol", "RControl", "RIGHTCTRL", 0xa3),
    ("ctrlright", "RControl", "RIGHTCTRL", 0xa3),
    ("shift", "Shift", "LEFTSHIFT", 0x10),
    ("lshift", "LShift", "LEFTSHIFT", 0xa0),
    ("shiftleft", "LShift", "LEFTSHIFT", 0xa0),
    ("alt", "Alt", "LEFTALT", 0x12),
    ("lalt", "LAlt", "LEFTALT", 0xa4),
    ("altleft", "LAlt", "LEFTALT", 0xa4),
    ("ralt", "RAlt", "RIGHTALT", 0xa5),
    ("altright", "RAlt", "RIGHTALT", 0xa5),
    ("lwin", "LWin", "LEFTMETA", 0x5b),
    ("winleft", "LWin", "LEFTMETA", 0x5b),
    ("rwin", "RWin", "RIGHTMETA", 0x5c),
    ("winright", "RWin", "RIGHTMETA", 0x5c),
];

/// The name v2 gives the key of this name.
///
/// A name the tree does not know is the empty string, the way v2 answers
/// a name it does not know.
fn key_name(name: &str) -> String {
    match find_key(name) {
        Some(key) => key.name,
        None => String::new(),
    }
}

/// The code of the Linux input event of the key of this name.
///
/// A name the tree does not know is the zero, the way v2 answers it.
fn key_event(name: &str) -> u16 {
    match find_key(name) {
        Some(key) => key.event,
        None => 0,
    }
}

/// The virtual key code of Windows of the key of this name.
///
/// A name the tree does not know is the zero, the way v2 answers it.
fn key_code(name: &str) -> u32 {
    match find_key(name) {
        Some(key) => key.code,
        None => 0,
    }
}

/// The key of this name, when the tree knows it.
///
/// The name is the one the script writes: the name of v2, the name of a
/// character, or one of the aliases of the tree (`Esc`, `Return`,
/// `CtrlLeft`...).
fn find_key(name: &str) -> Option<Key> {
    if name.is_empty() {
        return None;
    }
    // The name is not trimmed: the space is a key of its own.
    let wanted = name.to_lowercase();
    // The modifiers: the name the script wrote is the name it gets.
    if let Some((_, answer, event, code)) = MODIFIERS.iter().find(|(spelling, _, _, _)| *spelling == wanted) {
        return Some(Key {
            name: answer.to_string(),
            event: event_of(event),
            code: *code,
        });
    }
    // The name of v2.
    if let Some((answer, event, code)) = KEYS.iter().find(|(key, _, _)| key.eq_ignore_ascii_case(&wanted)) {
        return Some(Key {
            name: answer.to_string(),
            event: event_of(event),
            code: *code,
        });
    }
    // A letter or a digit is named after itself.
    if let Some(key) = character_key(&wanted) {
        return Some(key);
    }
    // The aliases of the tree: they name an event, and the event names
    // the key.
    let event = keys::resolve_name(&wanted).ok().or_else(|| {
        // A mark of the layout is typed with a key of its own: the `!`
        // is the `1` key with the shift held.
        let mut chars = wanted.chars();
        let first = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        keys::character_stroke(first).map(|(code, _)| code)
    })?;
    key_of_event(event)
}

/// The key of an event of the Linux input, when the tree knows it.
fn key_of_event(event: u16) -> Option<Key> {
    if let Some((_, answer, _, code)) = MODIFIERS.iter().find(|(_, _, name, _)| event_of(name) == event) {
        return Some(Key {
            name: answer.to_string(),
            event,
            code: *code,
        });
    }
    if let Some((answer, _, code)) = KEYS.iter().find(|(_, name, _)| event_of(name) == event) {
        return Some(Key {
            name: answer.to_string(),
            event,
            code: *code,
        });
    }
    // The letters and the digits are named after themselves.
    let name = keys::EVDEV.iter().find(|(_, code)| *code == event).map(|(name, _)| *name)?;
    let mut chars = name.chars();
    let first = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    if first.is_ascii_alphabetic() {
        let capital = first.to_ascii_uppercase();
        return Some(Key {
            name: first.to_lowercase().to_string(),
            event,
            code: capital as u32,
        });
    }
    if first.is_ascii_digit() {
        return Some(Key {
            name: first.to_string(),
            event,
            code: first as u32,
        });
    }
    None
}

/// The key of a letter or of a digit.
///
/// The letters and the digits are named after themselves: the name of the
/// `a` key is `a`, and the code of the letter is the code of its
/// capital.
fn character_key(character: &str) -> Option<Key> {
    let mut chars = character.chars();
    let first = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    if first.is_ascii_alphabetic() {
        let capital = first.to_ascii_uppercase();
        let event = event_of(&capital.to_string());
        return Some(Key {
            name: first.to_lowercase().to_string(),
            event,
            code: capital as u32,
        });
    }
    if first.is_ascii_digit() {
        let event = event_of(&first.to_string());
        return Some(Key {
            name: first.to_string(),
            event,
            code: first as u32,
        });
    }
    // The space is a key of its own, and v2 names it.
    if first == ' ' {
        return Some(Key {
            name: "Space".to_string(),
            event: event_of("SPACE"),
            code: 0x20,
        });
    }
    None
}

/// The code of the Linux input event of this name.
fn event_of(event: &str) -> u16 {
    keys::EVDEV
        .iter()
        .find(|(name, _)| *name == event)
        .map(|(_, code)| *code)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{
        Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers,
        Windows,
    };
    use crate::runtime::Clock;
    use core::time::Duration;

    /// A desktop that records what it was asked to do.
    #[derive(Debug, Default)]
    struct Fake {
        /// The text that was typed.
        typed: Vec<String>,
        /// The keys that were pressed.
        pressed: Vec<String>,
        /// The speed of the pointer, when it was set.
        speed: Option<i64>,
        /// The delay of the pointer, when it was set.
        delay: Option<i64>,
    }

    impl Desktop for Fake {}

    impl Files for Fake {}

    impl Clipboard for Fake {}

    impl Timers for Fake {}

    impl Hotkeys for Fake {}

    impl Windows for Fake {}

    impl Native for Fake {}

    impl Keyboard for Fake {
        fn type_text(&mut self, text: &str) -> Result<String, String> {
            self.typed.push(text.to_string());
            Ok(String::new())
        }
        fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
            self.pressed.push(name.to_string());
            Ok(String::new())
        }
    }

    impl Pointer for Fake {
        fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
            Ok(String::new())
        }
        fn set_mouse_speed(&mut self, speed: i64) -> Result<String, String> {
            self.speed = Some(speed);
            Ok(String::new())
        }
        fn set_mouse_delay(&mut self, delay: i64) -> Result<String, String> {
            self.delay = Some(delay);
            Ok(String::new())
        }
    }

    impl Screen for Fake {
        fn capture(
            &mut self,
            _x: i64,
            _y: i64,
            _width: i64,
            _height: i64,
            _filename: &str,
        ) -> Result<String, String> {
            Ok(String::new())
        }
        fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Processes for Fake {
        fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
            Ok((0, String::new(), String::new()))
        }
    }

    /// The clock of the tests: it counts from an arbitrary origin.
    #[derive(Debug)]
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            1234
        }
        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// Put the state back the way v2 starts it.
    fn reset() {
        STATE.coord_mouse.store(CLIENT, Ordering::Relaxed);
        STATE.coord_pixel.store(CLIENT, Ordering::Relaxed);
        STATE.key_delay.store(10, Ordering::Relaxed);
        STATE.key_delay_play.store(-1, Ordering::Relaxed);
        STATE.key_duration.store(-1, Ordering::Relaxed);
        STATE.key_duration_play.store(-1, Ordering::Relaxed);
        STATE.mouse_delay.store(10, Ordering::Relaxed);
        STATE.mouse_delay_play.store(-1, Ordering::Relaxed);
        STATE.send_mode.store(INPUT, Ordering::Relaxed);
        STATE.send_level.store(0, Ordering::Relaxed);
        STATE.mouse_speed.store(2, Ordering::Relaxed);
        STATE.caps_lock.store(1, Ordering::Relaxed);
        STATE.encoding.store(0, Ordering::Relaxed);
    }

    /// Run a body against a runtime that asks a fake desktop, and give
    /// the fake back.
    fn with_runtime<F: FnOnce(&mut Runtime<'_>)>(body: F) -> Fake {
        let mut desktop = Fake::default();
        let mut clock = TheClock;
        {
            let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
            body(&mut rt);
        }
        desktop
    }

    /// The value of the built-in variable of the settings of this name.
    fn value(name: &str) -> Value {
        match value_of(name) {
            Some(Ok(value)) => value,
            Some(Err(reason)) => panic!("{name} answered with a reason instead of a value: {reason}"),
            None => panic!("{name} is not a setting of this module"),
        }
    }

    #[test]
    fn the_defaults_are_the_ones_of_v2() {
        let _turn = Turn::take();
        reset();
        // Measured on the binary of v2.0.19 on 2026-10-28.
        assert_eq!(value("coordmodemouse"), Value::Text("Client".into()));
        assert_eq!(value("coordmodepixel"), Value::Text("Client".into()));
        assert_eq!(value("defaultmousespeed"), Value::Int(2));
        assert_eq!(value("fileencoding"), Value::Text("CP0".into()));
        assert_eq!(value("keydelay"), Value::Int(10));
        assert_eq!(value("keydelayplay"), Value::Int(-1));
        assert_eq!(value("keyduration"), Value::Int(-1));
        assert_eq!(value("keydurationplay"), Value::Int(-1));
        assert_eq!(value("listlines"), Value::Int(1));
        assert_eq!(value("menumaskkey"), Value::Text("vk11sc01D".into()));
        assert_eq!(value("mousedelay"), Value::Int(10));
        assert_eq!(value("mousedelayplay"), Value::Int(-1));
        assert_eq!(value("sendlevel"), Value::Int(0));
        assert_eq!(value("sendmode"), Value::Text("Input".into()));
        assert_eq!(value("storecapslockmode"), Value::Int(1));
        assert_eq!(value("allowmainwindow"), Value::Int(1));
    }

    #[test]
    fn the_encoding_of_the_files_is_the_one_v2_carries() {
        let _turn = Turn::take();
        reset();
        // Measured on the binary of v2.0.19 on 2026-10-28.
        assert_eq!(value("fileencoding"), Value::Text("CP0".into()));
        assert_eq!(set_file_encoding("utf-8"), Ok("UTF-8"));
        assert_eq!(value("fileencoding"), Value::Text("UTF-8".into()));
        // The suffix of the raw files is kept, the way v2 keeps it.
        assert_eq!(set_file_encoding("UTF-16-RAW"), Ok("UTF-16-RAW"));
        assert_eq!(value("fileencoding"), Value::Text("UTF-16-RAW".into()));
        // The bytes of a file are UTF-16 when the encoding says so.
        assert_eq!(file_bytes_encoding(), "UTF-16");
        assert_eq!(set_file_encoding("CP0"), Ok("CP0"));
        assert_eq!(file_bytes_encoding(), "UTF-8");
        // A code page is a name the tree does not carry: the box has none.
        assert_eq!(
            set_file_encoding("CP1252"),
            Err("Parameter #1 of FileEncoding is invalid.".to_string())
        );
    }

    #[test]
    fn the_mode_of_a_window_is_not_answered() {
        let _turn = Turn::take();
        reset();
        for name in ["coordmodecaret", "coordmodemenu", "coordmodetooltip"] {
            match value_of(name) {
                Some(Err(reason)) => assert!(reason.contains("no windows"), "{name} said {reason}"),
                other => panic!("{name} answered {other:?} instead of a reason"),
            }
        }
    }

    #[test]
    fn the_names_of_the_keys_are_the_ones_of_v2() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        assert_eq!(key_name("a"), "a");
        assert_eq!(key_name("A"), "a");
        assert_eq!(key_name("1"), "1");
        assert_eq!(key_name(" "), "Space");
        assert_eq!(key_name("Esc"), "Escape");
        assert_eq!(key_name("Enter"), "Enter");
        assert_eq!(key_name("Backspace"), "Backspace");
        assert_eq!(key_name("Ctrl"), "Control");
        assert_eq!(key_name("Shift"), "Shift");
        assert_eq!(key_name("Alt"), "Alt");
        assert_eq!(key_name("LWin"), "LWin");
        assert_eq!(key_name("RWin"), "RWin");
        assert_eq!(key_name("Numpad0"), "Numpad0");
        assert_eq!(key_name("NumpadEnter"), "NumpadEnter");
        assert_eq!(key_name("Volume_Up"), "Volume_Up");
        assert_eq!(key_name("NumLock"), "Numlock");
        assert_eq!(key_name("PgUp"), "PgUp");
        assert_eq!(key_name("PrintScreen"), "PrintScreen");
        assert_eq!(key_name("/"), "/");
        // A name the tree does not know is the empty string.
        assert_eq!(key_name("NoSuchKey"), "");
        assert_eq!(key_name(""), "");
    }

    #[test]
    fn the_codes_of_the_keys_are_the_ones_of_the_tree() {
        // The code of the event is the one the box writes, and the code
        // of Windows is the one of the PC keyboard.
        assert_eq!(key_event("a"), 30);
        assert_eq!(key_code("a"), 65);
        assert_eq!(key_event(" "), 57);
        assert_eq!(key_code(" "), 32);
        assert_eq!(key_event("Escape"), 1);
        assert_eq!(key_code("Escape"), 27);
        assert_eq!(key_event("Home"), 102);
        assert_eq!(key_code("Home"), 36);
        // A name the tree does not know is the zero.
        assert_eq!(key_event("NoSuchKey"), 0);
        assert_eq!(key_code("NoSuchKey"), 0);
    }

    /// The text of a parameter.
    fn text(value: &str) -> Value {
        Value::Text(value.to_string())
    }

    /// The message of a failure.
    fn message(result: Result<Value, RuntimeError>) -> String {
        match result {
            Ok(value) => panic!("the call answered {value:?} instead of failing"),
            Err(error) => error.message,
        }
    }

    #[test]
    fn the_coordinate_mode_answers_the_mode_it_replaces() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            let previous = coord_mode(rt, &[text("Mouse"), text("Screen")], 1, 0).expect("the mode changes");
            assert_eq!(previous, Value::Text("Client".into()));
        });
        assert_eq!(value("coordmodemouse"), Value::Text("Screen".into()));
        // The pixels are a mode of their own.
        assert_eq!(value("coordmodepixel"), Value::Text("Client".into()));
    }

    #[test]
    fn the_coordinate_mode_without_a_second_parameter_is_the_screen() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            coord_mode(rt, &[text("Pixel")], 1, 0).expect("the mode changes");
        });
        assert_eq!(value("coordmodepixel"), Value::Text("Screen".into()));
    }

    #[test]
    fn the_coordinate_mode_of_a_window_is_refused() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            let said = message(coord_mode(rt, &[text("Caret"), text("Screen")], 1, 0));
            assert!(said.contains("no windows"), "the call said {said}");
        });
    }

    #[test]
    fn a_coordinate_mode_that_is_not_a_name_is_refused() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            assert_eq!(
                message(coord_mode(rt, &[text("Mouse"), text("Bogus")], 1, 0)),
                "Parameter #2 of CoordMode is invalid."
            );
            assert_eq!(
                message(coord_mode(rt, &[text("Bogus"), text("Screen")], 1, 0)),
                "Parameter #1 of CoordMode is invalid."
            );
        });
    }

    #[test]
    fn the_delay_of_the_mouse_is_kept_and_answers_the_one_it_replaces() {
        let _turn = Turn::take();
        reset();
        let desktop = with_runtime(|rt| {
            let previous = set_mouse_delay(rt, &[Value::Int(50)], 1, 0).expect("the delay changes");
            assert_eq!(previous, Value::Int(10));
        });
        assert_eq!(value("mousedelay"), Value::Int(50));
        // The pointer of the box pauses between two movements.
        assert_eq!(desktop.delay, Some(50));
    }

    #[test]
    fn the_delay_of_the_play_method_is_kept_apart() {
        let _turn = Turn::take();
        reset();
        let desktop = with_runtime(|rt| {
            let previous = set_mouse_delay(rt, &[Value::Int(50), text("Play")], 1, 0).expect("the delay changes");
            assert_eq!(previous, Value::Int(-1));
        });
        assert_eq!(value("mousedelayplay"), Value::Int(50));
        assert_eq!(value("mousedelay"), Value::Int(10));
        // The pointer of the box is not the pointer of the play method.
        assert_eq!(desktop.delay, None);
    }

    #[test]
    fn a_delay_of_the_mouse_that_is_not_a_number_is_refused() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            assert_eq!(
                message(set_mouse_delay(rt, &[text("abc")], 1, 0)),
                "Parameter #1 of SetMouseDelay requires a Number, but received a String."
            );
            assert_eq!(
                message(set_mouse_delay(rt, &[Value::Int(50), text("Bogus")], 1, 0)),
                "Parameter #2 of SetMouseDelay is invalid."
            );
        });
    }

    #[test]
    fn the_speed_of_the_pointer_is_kept_and_answers_the_one_it_replaces() {
        let _turn = Turn::take();
        reset();
        let desktop = with_runtime(|rt| {
            let previous = set_default_mouse_speed(rt, &[Value::Int(20)], 1, 0).expect("the speed changes");
            assert_eq!(previous, Value::Int(2));
        });
        assert_eq!(value("defaultmousespeed"), Value::Int(20));
        // The pointer of the box moves in steps.
        assert_eq!(desktop.speed, Some(20));
    }

    #[test]
    fn a_speed_outside_the_range_of_v2_is_refused() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            assert_eq!(
                message(set_default_mouse_speed(rt, &[Value::Int(101)], 1, 0)),
                "Parameter #1 of SetDefaultMouseSpeed is invalid."
            );
            assert_eq!(
                message(set_default_mouse_speed(rt, &[Value::Int(-5)], 1, 0)),
                "Parameter #1 of SetDefaultMouseSpeed is invalid."
            );
        });
    }

    #[test]
    fn the_mode_of_the_caps_lock_is_a_boolean() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            let previous = set_store_caps_lock_mode(rt, &[Value::Int(0)], 1, 0).expect("the mode changes");
            assert_eq!(previous, Value::Int(1));
        });
        assert_eq!(value("storecapslockmode"), Value::Int(0));
        with_runtime(|rt| {
            let previous = set_store_caps_lock_mode(rt, &[Value::Int(1)], 1, 0).expect("the mode changes");
            assert_eq!(previous, Value::Int(0));
        });
        assert_eq!(value("storecapslockmode"), Value::Int(1));
    }

    #[test]
    fn the_level_of_the_send_is_kept() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            let previous = send_level(rt, &[Value::Int(3)], 1, 0).expect("the level changes");
            assert_eq!(previous, Value::Int(0));
        });
        assert_eq!(value("sendlevel"), Value::Int(3));
        with_runtime(|rt| {
            assert_eq!(
                message(send_level(rt, &[Value::Int(101)], 1, 0)),
                "Parameter #1 of SendLevel is invalid."
            );
        });
    }

    #[test]
    fn the_text_is_typed_as_it_stands() {
        let _turn = Turn::take();
        reset();
        let desktop = with_runtime(|rt| {
            send_text(rt, &[text("a{b}c")], 1, 0).expect("the text is typed");
        });
        // The braces of v2 are not read here.
        assert_eq!(desktop.typed, vec!["a{b}c".to_string()]);
    }

    #[test]
    fn the_keys_are_sent_the_way_the_tree_sends_them() {
        let _turn = Turn::take();
        reset();
        let desktop = with_runtime(|rt| {
            send_event(rt, &[text("{a}")], 1, 0).expect("the key is sent");
            send_input(rt, &[text("{b}")], 1, 0).expect("the key is sent");
        });
        assert_eq!(desktop.pressed, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn the_play_method_is_not_carried() {
        let _turn = Turn::take();
        reset();
        with_runtime(|rt| {
            let said = message(send_play(rt, &[text("abc")], 1, 0));
            assert!(said.contains("no window"), "the call said {said}");
        });
    }
}
