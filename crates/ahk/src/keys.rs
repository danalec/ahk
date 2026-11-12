//! Key codes and key specifications.
//!
//! A port of `src/ahk/keys.py` from the archived Python implementation,
//! with the key names of AutoHotkey v2 instead of the X11 ones.
//!
//! The codes here are what a uinput device needs to emit an event at all.
//! They are the Linux input event codes, and they were confirmed against
//! the box by pressing keys and reading the screen: 14 is Backspace, 22 is
//! `u`, 30 is `a`, 31 is `s`, 32 is `d`, 46 is `c`.
//!
//! The names are the ones of the v2 key list of v2.0.19: `Esc`, `Enter`,
//! `Backspace`, `CtrlRight`, `LWin`, `Numpad0`, `Volume_Up`, and the
//! `vkXX` and `scXXX` spellings. The X11 spellings that were here
//! before (`Return`, `Del`, `Esc`) are kept beside them, because the
//! scripts this lab runs are written in both.
//!
//! A name that the box has no key for is refused, and the refusal says
//! which name it was. The box has no browser keys, no joystick and no
//! mouse wheel, and a name with no key here is not answered with a
//! guess.
//!
//! `ydotool` is not used, and the reason is worth keeping: measured on
//! 2026-10-25 (ydotool 1.0.4), `ydotool key "Return"` is a silent no-op
//! -- ydotool 1.x takes numeric keycodes and reads a name as zero -- and
//! `ydotool key 28` presses the key without ever releasing it, so the
//! compositor's auto-repeat types the key forever. The daemon behind
//! `ydotool type` is not on the box any more, so the text goes through
//! the keyboard of our own.

use alloc::format;
use alloc::string::{String, ToString};
#[cfg(test)]
use alloc::vec;
use alloc::vec::Vec;

/// The Linux input event codes, the numbering ydotool 1.x expects.
pub const EVDEV: &[(&str, u16)] = &[
    ("ESC", 1),
    ("1", 2),
    ("2", 3),
    ("3", 4),
    ("4", 5),
    ("5", 6),
    ("6", 7),
    ("7", 8),
    ("8", 9),
    ("9", 10),
    ("0", 11),
    ("MINUS", 12),
    ("EQUAL", 13),
    ("BACKSPACE", 14),
    ("TAB", 15),
    ("Q", 16),
    ("W", 17),
    ("E", 18),
    ("R", 19),
    ("T", 20),
    ("Y", 21),
    ("U", 22),
    ("I", 23),
    ("O", 24),
    ("P", 25),
    ("LEFTBRACE", 26),
    ("RIGHTBRACE", 27),
    ("ENTER", 28),
    ("LEFTCTRL", 29),
    ("A", 30),
    ("S", 31),
    ("D", 32),
    ("F", 33),
    ("G", 34),
    ("H", 35),
    ("J", 36),
    ("K", 37),
    ("L", 38),
    ("SEMICOLON", 39),
    ("APOSTROPHE", 40),
    ("GRAVE", 41),
    ("LEFTSHIFT", 42),
    ("BACKSLASH", 43),
    ("Z", 44),
    ("X", 45),
    ("C", 46),
    ("V", 47),
    ("B", 48),
    ("N", 49),
    ("M", 50),
    ("COMMA", 51),
    ("DOT", 52),
    ("SLASH", 53),
    ("SPACE", 57),
    ("CAPSLOCK", 58),
    ("F1", 59),
    ("F2", 60),
    ("F3", 61),
    ("F4", 62),
    ("F5", 63),
    ("F6", 64),
    ("F7", 65),
    ("F8", 66),
    ("F9", 67),
    ("F10", 68),
    ("NUMLOCK", 69),
    ("SCROLLLOCK", 70),
    ("KP7", 71),
    ("KP8", 72),
    ("KP9", 73),
    ("KPMINUS", 74),
    ("KP4", 75),
    ("KP5", 76),
    ("KP6", 77),
    ("KPPLUS", 78),
    ("KP1", 79),
    ("KP2", 80),
    ("KP3", 81),
    ("KP0", 82),
    ("KPDOT", 83),
    ("KPENTER", 96),
    ("RIGHTCTRL", 97),
    ("KPSLASH", 98),
    ("SYSRQ", 99),
    ("RIGHTALT", 100),
    ("HOME", 102),
    ("UP", 103),
    ("PAGEUP", 104),
    ("LEFT", 105),
    ("RIGHT", 106),
    ("END", 107),
    ("DOWN", 108),
    ("PAGEDOWN", 109),
    ("INSERT", 110),
    ("DELETE", 111),
    ("MUTE", 113),
    ("VOLUMEDOWN", 114),
    ("VOLUMEUP", 115),
    ("POWER", 116),
    ("PAUSE", 119),
    ("LEFTALT", 56),
    ("LEFTMETA", 125),
    ("RIGHTMETA", 126),
    // The keys of the v2 list that were missing here. Measured on the binary
    // of v2.0.19 on 2026-11-09: every name below is one it accepts as a hotkey
    // (`NAME::MsgBox "x"` is validated, and `F25::` is refused), and the
    // numbers are the Linux input event codes of the box.
    //
    // The mouse buttons are EV_KEY codes of the pointer's device: BTN_LEFT is
    // 0x110 and the rest follow. They are keys like any other to this table,
    // and which device reports them is the desktop's business.
    ("BTN_LEFT", 0x110),
    ("BTN_RIGHT", 0x111),
    ("BTN_MIDDLE", 0x112),
    ("BTN_SIDE", 0x113),
    ("BTN_EXTRA", 0x114),
    ("RIGHTSHIFT", 54),
    ("KPASTERISK", 55),
    // The right shift was the other missing modifier. The table above names
    // the left one, and v2 knows the two apart.
    ("F11", 87),
    ("F12", 88),
    ("F13", 183),
    ("F14", 184),
    ("F15", 185),
    ("F16", 186),
    ("F17", 187),
    ("F18", 188),
    ("F19", 189),
    ("F20", 190),
    ("F21", 191),
    ("F22", 192),
    ("F23", 193),
    ("F24", 194),
    ("COMPOSE", 127),
    ("STOP", 128),
    ("HELP", 138),
    ("SLEEP", 142),
    ("MAIL", 155),
    ("BOOKMARKS", 156),
    ("COMPUTER", 157),
    ("BACK", 158),
    ("FORWARD", 159),
    ("NEXTSONG", 163),
    ("PLAYPAUSE", 164),
    ("PREVIOUSSONG", 165),
    ("STOPCD", 166),
    ("HOMEPAGE", 172),
    ("REFRESH", 173),
    ("SEARCH", 217),
    ("MEDIA", 226),
    ("CALC", 148),
];

/// The names of the wheel of the pointer.
///
/// The wheel is **not** a key: the kernel reports it as a relative axis
/// (`REL_WHEEL` 0x08 and `REL_HWHEEL` 0x06), and those two numbers are EV_KEY
/// codes already -- 0x08 is the `7` of the keyboard and 0x06 is the `5` -- so
/// they cannot be in the table above without making `Send "{WheelUp}"` press a
/// digit. The names therefore answer codes above `KEY_MAX` (0x2ff), which no
/// key of a Linux machine has: the name resolves and a hotkey of it parses,
/// and the watcher of the box -- which watches `EV_KEY` only -- never fires
/// it. Measured on the binary of v2.0.19 on 2026-11-09: `WheelUp::`,
/// `WheelDown::`, `WheelLeft::` and `WheelRight::` are all accepted.
pub const WHEEL_NAMES: &[(&str, u16)] = &[
    ("wheelup", 0xfff1),
    ("wheeldown", 0xfff2),
    ("wheelleft", 0xfff3),
    ("wheelright", 0xfff4),
];

/// The names people actually write, mapped onto the codes above.
///
/// The spelling is the one of the v2 key list: `Esc`, `Enter`,
/// `Backspace`, `CtrlRight`, `LWin`, `Numpad0`, `Volume_Up`. The X11
/// spellings that were here before (`Return`, `Del`, `Esc`) are kept
/// beside them, because both dialects turn up in the scripts this lab
/// runs.
///
/// Every name here points at a key of the table above, and a name that
/// does not is refused by `resolve_name` rather than pressed as
/// something else. The v2 names that are missing for that reason are
/// the browser keys, the media keys but the three volumes, the launch
/// keys, the joystick, the mouse buttons, the wheel, `Help`, `Sleep`
/// and `CtrlBreak` -- the box has no key for them -- and `AppsKey`,
/// `RShift`, `NumpadMult` and `F11` upwards, which the table above
/// has no code for.
pub const KEY_ALIASES: &[(&str, &str)] = &[
    // The general keys of the v2 list.
    ("esc", "ESC"),
    ("escape", "ESC"),
    ("space", "SPACE"),
    ("spacebar", "SPACE"),
    ("tab", "TAB"),
    ("enter", "ENTER"),
    ("return", "ENTER"),
    ("backspace", "BACKSPACE"),
    ("bksp", "BACKSPACE"),
    ("bs", "BACKSPACE"),
    ("capslock", "CAPSLOCK"),
    ("scrolllock", "SCROLLLOCK"),
    ("numlock", "NUMLOCK"),
    ("printscreen", "SYSRQ"),
    ("printscr", "SYSRQ"),
    ("prtsc", "SYSRQ"),
    ("pause", "PAUSE"),
    // The cursor keys.
    ("del", "DELETE"),
    ("delete", "DELETE"),
    ("ins", "INSERT"),
    ("insert", "INSERT"),
    ("home", "HOME"),
    ("end", "END"),
    ("pgup", "PAGEUP"),
    ("pageup", "PAGEUP"),
    ("pgdn", "PAGEDOWN"),
    ("pagedown", "PAGEDOWN"),
    ("up", "UP"),
    ("down", "DOWN"),
    ("left", "LEFT"),
    ("right", "RIGHT"),
    // The numpad. The second name of each key is the one it has when
    // the num lock is off.
    ("numpad0", "KP0"),
    ("numpad1", "KP1"),
    ("numpad2", "KP2"),
    ("numpad3", "KP3"),
    ("numpad4", "KP4"),
    ("numpad5", "KP5"),
    ("numpad6", "KP6"),
    ("numpad7", "KP7"),
    ("numpad8", "KP8"),
    ("numpad9", "KP9"),
    ("numpadins", "KP0"),
    ("numpadend", "KP1"),
    ("numpaddown", "KP2"),
    ("numpadpgdn", "KP3"),
    ("numpadleft", "KP4"),
    ("numpadclear", "KP5"),
    ("numpadright", "KP6"),
    ("numpadhome", "KP7"),
    ("numpadup", "KP8"),
    ("numpadpgup", "KP9"),
    ("numpaddot", "KPDOT"),
    ("numpaddel", "KPDOT"),
    ("numpaddecimal", "KPDOT"),
    ("numpaddiv", "KPSLASH"),
    ("numpaddivide", "KPSLASH"),
    ("numpadadd", "KPPLUS"),
    ("numpadsub", "KPMINUS"),
    ("numpadsubtract", "KPMINUS"),
    ("numpadenter", "KPENTER"),
    // The multimedia keys the box has. The browser keys and the media
    // keys have no code here.
    ("volume_mute", "MUTE"),
    ("volume_down", "VOLUMEDOWN"),
    ("volume_up", "VOLUMEUP"),
    // The modifiers. The right shift is not here: the table above has
    // no code for it, and the left one is not the same key.
    ("ctrl", "LEFTCTRL"),
    ("control", "LEFTCTRL"),
    ("lctrl", "LEFTCTRL"),
    ("lcontrol", "LEFTCTRL"),
    ("ctrlleft", "LEFTCTRL"),
    ("rctrl", "RIGHTCTRL"),
    ("rcontrol", "RIGHTCTRL"),
    ("ctrlright", "RIGHTCTRL"),
    ("shift", "LEFTSHIFT"),
    ("lshift", "LEFTSHIFT"),
    ("shiftleft", "LEFTSHIFT"),
    ("alt", "LEFTALT"),
    ("lalt", "LEFTALT"),
    ("altleft", "LEFTALT"),
    ("ralt", "RIGHTALT"),
    ("altright", "RIGHTALT"),
    ("lwin", "LEFTMETA"),
    ("winleft", "LEFTMETA"),
    ("win", "LEFTMETA"),
    ("rwin", "RIGHTMETA"),
    ("winright", "RIGHTMETA"),
    ("super", "LEFTMETA"),
    ("meta", "LEFTMETA"),
    ("cmd", "LEFTMETA"),
    // The mouse buttons of the v2 list, and the wheel.
    ("lbutton", "BTN_LEFT"),
    ("rbutton", "BTN_RIGHT"),
    ("mbutton", "BTN_MIDDLE"),
    ("xbutton1", "BTN_SIDE"),
    ("xbutton2", "BTN_EXTRA"),
    ("numpadmult", "KPASTERISK"),
    ("numpadmultiply", "KPASTERISK"),
    // The other names of the v2 list that the table lacked. Measured on the
    // binary of v2.0.19 on 2026-11-09.
    ("appskey", "COMPOSE"),
    ("apps", "COMPOSE"),
    ("menu", "COMPOSE"),
    ("sleep", "SLEEP"),
    ("help", "HELP"),
    // The Break of a PC keyboard is the Pause key with Ctrl held, and the
    // code is the Pause key: the Ctrl is not part of the code.
    ("ctrlbreak", "PAUSE"),
    ("rshift", "RIGHTSHIFT"),
    ("shiftright", "RIGHTSHIFT"),
    ("launch_mail", "MAIL"),
    ("launch_media", "MEDIA"),
    ("launch_app1", "COMPUTER"),
    ("launch_app2", "CALC"),
    // The media keys: the v2 spelling is the one with the underscores, and
    // `MediaPlay` without them is not a name of v2 -- measured on the binary
    // of v2.0.19 on 2026-11-09, it answers "Invalid key name.".
    ("media_play_pause", "PLAYPAUSE"),
    ("media_stop", "STOPCD"),
    ("media_next", "NEXTSONG"),
    ("media_prev", "PREVIOUSSONG"),
    ("browser_back", "BACK"),
    ("browser_forward", "FORWARD"),
    ("browser_refresh", "REFRESH"),
    ("browser_stop", "STOP"),
    ("browser_search", "SEARCH"),
    ("browser_favorites", "BOOKMARKS"),
    ("browser_home", "HOMEPAGE"),
];

/// The virtual key codes of the PC keyboard, and the key each one names.
///
/// These are the numbers behind the `vkXX` spelling of the v2 list, and
/// they are a numbering of their own: `vk41` is the A key and `vk20`
/// the space bar, and neither is the evdev code the box wants.
const VIRTUAL_KEYS: &[(u32, &str)] = &[
    (0x008, "BACKSPACE"),
    (0x009, "TAB"),
    (0x00d, "ENTER"),
    (0x013, "PAUSE"),
    (0x014, "CAPSLOCK"),
    (0x01b, "ESC"),
    (0x020, "SPACE"),
    (0x021, "PAGEUP"),
    (0x022, "PAGEDOWN"),
    (0x023, "END"),
    (0x024, "HOME"),
    (0x025, "LEFT"),
    (0x026, "UP"),
    (0x027, "RIGHT"),
    (0x028, "DOWN"),
    (0x02c, "SYSRQ"),
    (0x02d, "INSERT"),
    (0x02e, "DELETE"),
    (0x060, "KP0"),
    (0x061, "KP1"),
    (0x062, "KP2"),
    (0x063, "KP3"),
    (0x064, "KP4"),
    (0x065, "KP5"),
    (0x066, "KP6"),
    (0x067, "KP7"),
    (0x068, "KP8"),
    (0x069, "KP9"),
    (0x06b, "KPPLUS"),
    (0x06c, "KPDOT"),
    (0x06d, "KPMINUS"),
    (0x06e, "KPDOT"),
    (0x06f, "KPSLASH"),
    (0x070, "F1"),
    (0x071, "F2"),
    (0x072, "F3"),
    (0x073, "F4"),
    (0x074, "F5"),
    (0x075, "F6"),
    (0x076, "F7"),
    (0x077, "F8"),
    (0x078, "F9"),
    (0x079, "F10"),
    (0x090, "NUMLOCK"),
    (0x091, "SCROLLLOCK"),
    (0x0a0, "LEFTSHIFT"),
    (0x0a2, "LEFTCTRL"),
    (0x0a3, "RIGHTCTRL"),
    (0x0a4, "LEFTALT"),
    (0x0a5, "RIGHTALT"),
    (0x0ad, "MUTE"),
    (0x0ae, "VOLUMEDOWN"),
    (0x0af, "VOLUMEUP"),
    (0x0ba, "SEMICOLON"),
    (0x0bb, "EQUAL"),
    (0x0bc, "COMMA"),
    (0x0bd, "MINUS"),
    (0x0be, "DOT"),
    (0x0bf, "SLASH"),
    (0x0c0, "GRAVE"),
    (0x0db, "LEFTBRACE"),
    (0x0dc, "BACKSLASH"),
    (0x0dd, "RIGHTBRACE"),
    (0x0de, "APOSTROPHE"),
    (0x0e2, "BACKSLASH"),
];

/// The scan codes of the PC keyboard, and the key each one names.
///
/// These are the numbers behind the `scXXX` spelling of the v2 list.
/// The typing block shares its numbers with the evdev codes, and the
/// two numberings part company on the extended keys: the arrows and
/// the navigation block take the numbers of the keypad, which is why
/// `SC047` is `Home` here and not the `7` of the keypad.
const SCAN_KEYS: &[(u32, &str)] = &[
    (0x001, "ESC"),
    (0x002, "1"),
    (0x003, "2"),
    (0x004, "3"),
    (0x005, "4"),
    (0x006, "5"),
    (0x007, "6"),
    (0x008, "7"),
    (0x009, "8"),
    (0x00a, "9"),
    (0x00b, "0"),
    (0x00c, "MINUS"),
    (0x00d, "EQUAL"),
    (0x00e, "BACKSPACE"),
    (0x00f, "TAB"),
    (0x010, "Q"),
    (0x011, "W"),
    (0x012, "E"),
    (0x013, "R"),
    (0x014, "T"),
    (0x015, "Y"),
    (0x016, "U"),
    (0x017, "I"),
    (0x018, "O"),
    (0x019, "P"),
    (0x01a, "LEFTBRACE"),
    (0x01b, "RIGHTBRACE"),
    (0x01c, "ENTER"),
    (0x01d, "LEFTCTRL"),
    (0x01e, "A"),
    (0x01f, "S"),
    (0x020, "D"),
    (0x021, "F"),
    (0x022, "G"),
    (0x023, "H"),
    (0x024, "J"),
    (0x025, "K"),
    (0x026, "L"),
    (0x027, "SEMICOLON"),
    (0x028, "APOSTROPHE"),
    (0x029, "GRAVE"),
    (0x02a, "LEFTSHIFT"),
    (0x02b, "BACKSLASH"),
    (0x02c, "Z"),
    (0x02d, "X"),
    (0x02e, "C"),
    (0x02f, "V"),
    (0x030, "B"),
    (0x031, "N"),
    (0x032, "M"),
    (0x033, "COMMA"),
    (0x034, "DOT"),
    (0x035, "SLASH"),
    (0x038, "LEFTALT"),
    (0x039, "SPACE"),
    (0x03a, "CAPSLOCK"),
    (0x03b, "F1"),
    (0x03c, "F2"),
    (0x03d, "F3"),
    (0x03e, "F4"),
    (0x03f, "F5"),
    (0x040, "F6"),
    (0x041, "F7"),
    (0x042, "F8"),
    (0x043, "F9"),
    (0x044, "F10"),
    (0x045, "NUMLOCK"),
    (0x046, "SCROLLLOCK"),
    (0x047, "HOME"),
    (0x048, "UP"),
    (0x049, "PAGEUP"),
    (0x04b, "LEFT"),
    (0x04d, "RIGHT"),
    (0x04f, "END"),
    (0x050, "DOWN"),
    (0x051, "PAGEDOWN"),
    (0x052, "INSERT"),
    (0x053, "DELETE"),
    (0x05b, "LEFTMETA"),
    (0x05c, "RIGHTMETA"),
];

/// The modifier names, and the code that identifies each one.
///
/// The four names the runtime uses are `ctrl`, `shift`, `alt` and
/// `super`; the v2 spellings of the same keys are here beside them, so
/// that a script may write `LWin` or `RCtrl` as well.
pub const MODIFIERS: &[(&str, &str)] = &[
    ("ctrl", "LEFTCTRL"),
    ("control", "LEFTCTRL"),
    ("lctrl", "LEFTCTRL"),
    ("lcontrol", "LEFTCTRL"),
    ("rctrl", "RIGHTCTRL"),
    ("rcontrol", "RIGHTCTRL"),
    ("shift", "LEFTSHIFT"),
    ("lshift", "LEFTSHIFT"),
    // The right shift is a modifier of its own, and v2 knows it: measured on
    // the binary of v2.0.19 on 2026-11-09, `RShift::` is accepted.
    ("rshift", "RIGHTSHIFT"),
    ("alt", "LEFTALT"),
    ("lalt", "LEFTALT"),
    ("ralt", "RIGHTALT"),
    ("lwin", "LEFTMETA"),
    ("win", "LEFTMETA"),
    ("rwin", "RIGHTMETA"),
    ("super", "LEFTMETA"),
    ("meta", "LEFTMETA"),
    ("cmd", "LEFTMETA"),
];

/// A key name that could not be understood.
#[derive(Debug)]
pub struct UnknownKey(pub String);

impl core::fmt::Display for UnknownKey {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "unknown key name: {:?}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownKey {}

/// The evdev code for a key name.
///
/// Accepts the aliases above (`Enter`, `Del`, `ctrl`...), the bare evdev
/// names (`LEFTCTRL`, `F7`), the virtual key and scan code spellings
/// (`vk41`, `sc01e`), and a single character (`a`, `7`, `/`).
pub fn resolve_name(name: &str) -> Result<u16, UnknownKey> {
    let key = name.trim();
    if key.is_empty() {
        return Err(UnknownKey(name.to_string()));
    }
    let lowered = key.to_lowercase();
    if let Some((_, target)) = KEY_ALIASES.iter().find(|(alias, _)| *alias == lowered) {
        return code_of(target).ok_or_else(|| UnknownKey(name.to_string()));
    }
    // The wheel is not a key of the keyboard, and it is answered from its own
    // table: the codes above `KEY_MAX` say "the box has no such key".
    if let Some((_, code)) = WHEEL_NAMES.iter().find(|(alias, _)| *alias == lowered) {
        return Ok(*code);
    }
    if let Some(code) = hex_code(&lowered, "vk") {
        if let Some(target) = virtual_key_name(code) {
            return code_of(target).ok_or_else(|| UnknownKey(name.to_string()));
        }
    }
    if let Some(code) = hex_code(&lowered, "sc") {
        if let Some(target) = scan_key_name(code) {
            return code_of(target).ok_or_else(|| UnknownKey(name.to_string()));
        }
    }
    let upper = key.to_uppercase();
    if let Some((_, code)) = EVDEV.iter().find(|(name_, _)| *name_ == upper) {
        return Ok(*code);
    }
    if key.chars().count() == 1 {
        return character_code(key).ok_or_else(|| UnknownKey(name.to_string()));
    }
    Err(UnknownKey(name.to_string()))
}

/// The evdev code of a name that is already one of ours.
fn code_of(name: &str) -> Option<u16> {
    EVDEV
        .iter()
        .find(|(name_, _)| *name_ == name)
        .map(|(_, code)| *code)
}

/// The number of a `vkXX` or `scXXX` name, read as hexadecimal.
fn hex_code(name: &str, prefix: &str) -> Option<u32> {
    let rest = name.strip_prefix(prefix)?;
    if rest.is_empty() || rest.len() > 4 {
        return None;
    }
    u32::from_str_radix(rest, 16).ok()
}

/// The name of the key with a virtual key code.
///
/// The letters and the digits have the virtual key code of the character
/// itself, and the name of a letter in the table above is the letter in
/// upper case.
fn virtual_key_name(code: u32) -> Option<&'static str> {
    if let Some((_, name)) = VIRTUAL_KEYS.iter().find(|(key, _)| *key == code) {
        return Some(*name);
    }
    let character = char::from_u32(code)?;
    if !character.is_ascii_alphanumeric() {
        return None;
    }
    let upper = character.to_ascii_uppercase();
    EVDEV
        .iter()
        .find(|(name, _)| name.starts_with(upper))
        .map(|(name, _)| *name)
}

/// The name of the key with a scan code.
fn scan_key_name(code: u32) -> Option<&'static str> {
    SCAN_KEYS
        .iter()
        .find(|(key, _)| *key == code)
        .map(|(_, name)| *name)
}

/// The evdev code for a single printable character.
fn character_code(character: &str) -> Option<u16> {
    let name = match character {
        " " => "SPACE",
        "-" => "MINUS",
        "=" => "EQUAL",
        "[" => "LEFTBRACE",
        "]" => "RIGHTBRACE",
        ";" => "SEMICOLON",
        "'" => "APOSTROPHE",
        "`" => "GRAVE",
        "\\" => "BACKSLASH",
        "," => "COMMA",
        "." => "DOT",
        "/" => "SLASH",
        other => {
            if other.chars().all(|c| c.is_ascii_digit()) {
                other
            } else if other.chars().all(|c| c.is_ascii_alphabetic()) {
                return other.to_uppercase().chars().next().and_then(|c| {
                    EVDEV
                        .iter()
                        .find(|(name_, _)| *name_ == c.to_string())
                        .map(|(_, code)| *code)
                });
            } else {
                return None;
            }
        }
    };
    EVDEV
        .iter()
        .find(|(name_, _)| *name_ == name)
        .map(|(_, code)| *code)
}

/// The key that types a character, and whether it needs the shift key.
///
/// The layout is the US one, the layout of the keyboard of the lab box.
/// A character that the layout does not have -- an accented letter, a
/// currency sign -- has no key here, and the caller is told so instead of
/// typing something else.
pub fn character_stroke(character: char) -> Option<(u16, bool)> {
    let shifted = |name: &str| {
        EVDEV
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, code)| (*code, true))
    };
    let plain = |name: &str| {
        EVDEV
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, code)| (*code, false))
    };
    if character.is_ascii_alphabetic() {
        let name = character.to_ascii_uppercase().to_string();
        let code = EVDEV
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, code)| *code)?;
        return Some((code, character.is_ascii_uppercase()));
    }
    if character.is_ascii_digit() {
        let name = character.to_string();
        return EVDEV
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, code)| (*code, false));
    }
    match character {
        ' ' => plain("SPACE"),
        '-' => plain("MINUS"),
        '_' => shifted("MINUS"),
        '=' => plain("EQUAL"),
        '+' => shifted("EQUAL"),
        '[' => plain("LEFTBRACE"),
        '{' => shifted("LEFTBRACE"),
        ']' => plain("RIGHTBRACE"),
        '}' => shifted("RIGHTBRACE"),
        ';' => plain("SEMICOLON"),
        ':' => shifted("SEMICOLON"),
        '\'' => plain("APOSTROPHE"),
        '"' => shifted("APOSTROPHE"),
        '`' => plain("GRAVE"),
        '~' => shifted("GRAVE"),
        '\\' => plain("BACKSLASH"),
        '|' => shifted("BACKSLASH"),
        ',' => plain("COMMA"),
        '<' => shifted("COMMA"),
        '.' => plain("DOT"),
        '>' => shifted("DOT"),
        '/' => plain("SLASH"),
        '?' => shifted("SLASH"),
        '!' => shifted("1"),
        '@' => shifted("2"),
        '#' => shifted("3"),
        '$' => shifted("4"),
        '%' => shifted("5"),
        '^' => shifted("6"),
        '&' => shifted("7"),
        '*' => shifted("8"),
        '(' => shifted("9"),
        ')' => shifted("0"),
        _ => None,
    }
}

/// One key press: the key itself, plus the modifiers held while it is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyStroke {
    pub code: u16,
    pub modifiers: Vec<String>,
}

impl KeyStroke {
    /// A human-readable spelling, for the report that says what was pressed.
    pub fn describe(&self) -> String {
        if self.modifiers.is_empty() {
            self.code.to_string()
        } else {
            format!("{}+{}", self.modifiers.join("+"), self.code)
        }
    }
}

/// Parse a key specification into the strokes it means.
///
/// Accepts the shapes people write: `Enter`, `ctrl+c`, `ctrl+shift+t`,
/// and `ctrl+c ctrl+c` for two strokes.
pub fn parse_spec(spec: &str) -> Result<Vec<KeyStroke>, UnknownKey> {
    let mut strokes = Vec::new();
    for chunk in spec.split_whitespace() {
        if chunk.is_empty() {
            continue;
        }
        let parts: Vec<&str> = chunk.split('+').collect();
        let mut modifiers = Vec::new();
        let mut key_part = parts[parts.len() - 1].to_string();
        for modifier in &parts[..parts.len() - 1] {
            let name = modifier.to_lowercase();
            if name == "plus" {
                key_part = "+".to_string();
                continue;
            }
            if !MODIFIERS.iter().any(|(alias, _)| *alias == name) {
                return Err(UnknownKey(modifier.to_string()));
            }
            modifiers.push(name);
        }
        strokes.push(KeyStroke {
            code: resolve_name(&key_part)?,
            modifiers,
        });
    }
    if strokes.is_empty() {
        return Err(UnknownKey(spec.to_string()));
    }
    Ok(strokes)
}

/// The modifier symbols at the front of a key name.
///
/// KeySharp's `TextToVKandSC` reads the modifier symbols of AHK at the
/// front of the name -- `^` for the control, `+` for the shift, `!` for
/// the alt and `#` for the windows key, each with an optional `<` or `>`
/// side in front of it -- and answers with the rest of the name and the
/// modifiers the symbols carry. The mask is the one of KeySharp: the alt
/// is one, the control two, the shift four and the windows key eight. The
/// prefix is built in the order of KeySharp -- the control, the shift,
/// the alt, the windows key -- and a side appears in it when the symbol
/// carried one; a symbol with no side is both sides.
pub(crate) fn the_modifier_prefix_of(text: &str) -> (String, u32, &str) {
    let bytes = text.as_bytes();
    let mut index = 0;
    // The sides of each modifier: the left and the right.
    let mut ctrl = (false, false);
    let mut shift = (false, false);
    let mut alt = (false, false);
    let mut win = (false, false);
    loop {
        let mut peek = index;
        let mut side = b'\0';
        if peek < bytes.len() && (bytes[peek] == b'<' || bytes[peek] == b'>') {
            side = bytes[peek];
            peek += 1;
        }
        if peek >= bytes.len() {
            break;
        }
        let slot = match bytes[peek] {
            b'^' => &mut ctrl,
            b'+' => &mut shift,
            b'!' => &mut alt,
            b'#' => &mut win,
            _ => break,
        };
        match side {
            b'<' => slot.0 = true,
            b'>' => slot.1 = true,
            _ => {
                // A symbol with no side is the two sides: the hotkey fires
                // with either of them.
                slot.0 = true;
                slot.1 = true;
            }
        }
        index = peek + 1;
    }
    let mut mask = 0;
    let mut prefix = String::new();
    for (slot, bit, symbol) in [
        (ctrl, 2u32, '^'),
        (shift, 4u32, '+'),
        (alt, 1u32, '!'),
        (win, 8u32, '#'),
    ] {
        if !slot.0 && !slot.1 {
            continue;
        }
        mask |= bit;
        if slot.0 && !slot.1 {
            prefix.push('<');
        } else if slot.1 && !slot.0 {
            prefix.push('>');
        }
        prefix.push(symbol);
    }
    (prefix, mask, &text[index..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_map_to_the_codes_the_box_uses() {
        assert_eq!(resolve_name("BackSpace").unwrap(), 14);
        assert_eq!(resolve_name("Return").unwrap(), 28);
        assert_eq!(resolve_name("a").unwrap(), 30);
        assert_eq!(resolve_name("u").unwrap(), 22);
        assert_eq!(resolve_name("c").unwrap(), 46);
    }

    #[test]
    fn a_specification_carries_its_modifiers() {
        let strokes = parse_spec("ctrl+c").unwrap();
        assert_eq!(strokes.len(), 1);
        assert_eq!(strokes[0].code, 46);
        assert_eq!(strokes[0].modifiers, vec!["ctrl"]);
    }

    #[test]
    fn a_sequence_is_several_strokes() {
        let strokes = parse_spec("ctrl+c ctrl+c").unwrap();
        assert_eq!(strokes.len(), 2);
    }

    #[test]
    fn an_unknown_name_is_refused() {
        assert!(parse_spec("NoSuchKey").is_err());
    }

    #[test]
    fn a_letter_is_its_key() {
        assert_eq!(character_stroke('a'), Some((30, false)));
        assert_eq!(character_stroke('u'), Some((22, false)));
    }

    #[test]
    fn a_capital_needs_the_shift_key() {
        assert_eq!(character_stroke('A'), Some((30, true)));
        assert_eq!(character_stroke('Z'), Some((44, true)));
    }

    #[test]
    fn a_digit_is_its_key() {
        assert_eq!(character_stroke('1'), Some((2, false)));
        assert_eq!(character_stroke('0'), Some((11, false)));
    }

    #[test]
    fn the_shifted_punctuation_is_the_same_key_with_shift() {
        assert_eq!(character_stroke('!'), Some((2, true)));
        assert_eq!(character_stroke('?'), Some((53, true)));
        assert_eq!(character_stroke(' '), Some((57, false)));
    }

    #[test]
    fn a_character_the_layout_does_not_have_has_no_key() {
        assert_eq!(character_stroke('ç'), None);
        assert_eq!(character_stroke('€'), None);
    }

    #[test]
    fn the_general_keys_are_named_the_way_v2_names_them() {
        assert_eq!(resolve_name("Esc").unwrap(), 1);
        assert_eq!(resolve_name("Escape").unwrap(), 1);
        assert_eq!(resolve_name("Space").unwrap(), 57);
        assert_eq!(resolve_name("Tab").unwrap(), 15);
        assert_eq!(resolve_name("Enter").unwrap(), 28);
        assert_eq!(resolve_name("Backspace").unwrap(), 14);
        assert_eq!(resolve_name("BS").unwrap(), 14);
        assert_eq!(resolve_name("CapsLock").unwrap(), 58);
        assert_eq!(resolve_name("ScrollLock").unwrap(), 70);
        assert_eq!(resolve_name("NumLock").unwrap(), 69);
        assert_eq!(resolve_name("PrintScreen").unwrap(), 99);
        assert_eq!(resolve_name("PrintScr").unwrap(), 99);
        assert_eq!(resolve_name("Pause").unwrap(), 119);
    }

    #[test]
    fn the_cursor_keys_are_named_the_way_v2_names_them() {
        assert_eq!(resolve_name("Del").unwrap(), 111);
        assert_eq!(resolve_name("Delete").unwrap(), 111);
        assert_eq!(resolve_name("Ins").unwrap(), 110);
        assert_eq!(resolve_name("Insert").unwrap(), 110);
        assert_eq!(resolve_name("Home").unwrap(), 102);
        assert_eq!(resolve_name("End").unwrap(), 107);
        assert_eq!(resolve_name("PgUp").unwrap(), 104);
        assert_eq!(resolve_name("PageUp").unwrap(), 104);
        assert_eq!(resolve_name("PgDn").unwrap(), 109);
        assert_eq!(resolve_name("PageDown").unwrap(), 109);
        assert_eq!(resolve_name("Up").unwrap(), 103);
        assert_eq!(resolve_name("Down").unwrap(), 108);
        assert_eq!(resolve_name("Left").unwrap(), 105);
        assert_eq!(resolve_name("Right").unwrap(), 106);
    }

    #[test]
    fn the_numpad_is_named_the_way_v2_names_it() {
        assert_eq!(resolve_name("Numpad0").unwrap(), 82);
        assert_eq!(resolve_name("Numpad9").unwrap(), 73);
        assert_eq!(resolve_name("NumpadHome").unwrap(), 71);
        assert_eq!(resolve_name("NumpadDel").unwrap(), 83);
        assert_eq!(resolve_name("NumpadDecimal").unwrap(), 83);
        assert_eq!(resolve_name("NumpadAdd").unwrap(), 78);
        assert_eq!(resolve_name("NumpadSubtract").unwrap(), 74);
        assert_eq!(resolve_name("NumpadDivide").unwrap(), 98);
        assert_eq!(resolve_name("NumpadEnter").unwrap(), 96);
    }

    #[test]
    fn the_modifiers_are_named_the_way_v2_names_them() {
        assert_eq!(resolve_name("Ctrl").unwrap(), 29);
        assert_eq!(resolve_name("CtrlLeft").unwrap(), 29);
        assert_eq!(resolve_name("LCtrl").unwrap(), 29);
        assert_eq!(resolve_name("CtrlRight").unwrap(), 97);
        assert_eq!(resolve_name("RCtrl").unwrap(), 97);
        assert_eq!(resolve_name("Shift").unwrap(), 42);
        assert_eq!(resolve_name("ShiftLeft").unwrap(), 42);
        assert_eq!(resolve_name("Alt").unwrap(), 56);
        assert_eq!(resolve_name("AltLeft").unwrap(), 56);
        assert_eq!(resolve_name("AltRight").unwrap(), 100);
        assert_eq!(resolve_name("LWin").unwrap(), 125);
        assert_eq!(resolve_name("WinLeft").unwrap(), 125);
        assert_eq!(resolve_name("RWin").unwrap(), 126);
        assert_eq!(resolve_name("WinRight").unwrap(), 126);
    }

    #[test]
    fn the_volumes_are_the_keys_the_box_has() {
        assert_eq!(resolve_name("Volume_Mute").unwrap(), 113);
        assert_eq!(resolve_name("Volume_Down").unwrap(), 114);
        assert_eq!(resolve_name("Volume_Up").unwrap(), 115);
    }

    #[test]
    fn a_virtual_key_is_the_key_it_names() {
        assert_eq!(resolve_name("vk41").unwrap(), 30);
        assert_eq!(resolve_name("VK41").unwrap(), 30);
        assert_eq!(resolve_name("vk20").unwrap(), 57);
        assert_eq!(resolve_name("vk0d").unwrap(), 28);
        assert_eq!(resolve_name("vk70").unwrap(), 59);
        assert_eq!(resolve_name("vk60").unwrap(), 82);
    }

    #[test]
    fn a_scan_code_is_the_key_it_names() {
        assert_eq!(resolve_name("sc01e").unwrap(), 30);
        assert_eq!(resolve_name("SC01E").unwrap(), 30);
        assert_eq!(resolve_name("sc039").unwrap(), 57);
        assert_eq!(resolve_name("sc048").unwrap(), 103);
    }

    #[test]
    fn a_name_the_box_has_no_key_for_is_refused() {
        // The joystick of v2 has no device on the box, and the names the
        // binary itself refuses are refused here too: `F25::` and
        // `MediaPlay::` answer "Invalid key name." on the binary of v2.0.19
        // (measured on 2026-11-09), and a name that is not a key at all is
        // not answered with a guess.
        assert!(resolve_name("Joy1").is_err());
        assert!(resolve_name("F25").is_err());
        assert!(resolve_name("MediaPlay").is_err());
        assert!(resolve_name("MediaPlayPause").is_err());
        assert!(resolve_name("zzz").is_err());
        assert!(resolve_name("Nonsense").is_err());
        assert!(resolve_name("NumpadClean").is_err());
        // `Mute` is not a name of v2 -- `Volume_Mute` is -- and the tree
        // answers it anyway: the bare epdev names of this table are part of
        // its own surface, and `MUTE` is one of them.
        assert_eq!(resolve_name("Mute").unwrap(), 113);
    }

    #[test]
    fn the_keys_the_binary_accepts_are_carried() {
        // Measured on the binary of v2.0.19 on 2026-11-09: every name here is
        // accepted as a hotkey, and the code is the Linux input event code of
        // the key on the box.
        assert_eq!(resolve_name("LButton").unwrap(), 0x110);
        assert_eq!(resolve_name("RButton").unwrap(), 0x111);
        assert_eq!(resolve_name("MButton").unwrap(), 0x112);
        assert_eq!(resolve_name("XButton1").unwrap(), 0x113);
        assert_eq!(resolve_name("XButton2").unwrap(), 0x114);
        assert_eq!(resolve_name("F11").unwrap(), 87);
        assert_eq!(resolve_name("F12").unwrap(), 88);
        assert_eq!(resolve_name("F13").unwrap(), 183);
        assert_eq!(resolve_name("F24").unwrap(), 194);
        assert_eq!(resolve_name("NumpadMult").unwrap(), 55);
        assert_eq!(resolve_name("AppsKey").unwrap(), 127);
        assert_eq!(resolve_name("Sleep").unwrap(), 142);
        assert_eq!(resolve_name("Help").unwrap(), 138);
        assert_eq!(resolve_name("CtrlBreak").unwrap(), 119);
        assert_eq!(resolve_name("RShift").unwrap(), 54);
        assert_eq!(resolve_name("Media_Play_Pause").unwrap(), 164);
        assert_eq!(resolve_name("Media_Stop").unwrap(), 166);
        assert_eq!(resolve_name("Media_Next").unwrap(), 163);
        assert_eq!(resolve_name("Media_Prev").unwrap(), 165);
        assert_eq!(resolve_name("Browser_Back").unwrap(), 158);
        assert_eq!(resolve_name("Browser_Forward").unwrap(), 159);
        assert_eq!(resolve_name("Browser_Refresh").unwrap(), 173);
        assert_eq!(resolve_name("Browser_Stop").unwrap(), 128);
        assert_eq!(resolve_name("Browser_Search").unwrap(), 217);
        assert_eq!(resolve_name("Browser_Favorites").unwrap(), 156);
        assert_eq!(resolve_name("Browser_Home").unwrap(), 172);
        assert_eq!(resolve_name("Launch_Mail").unwrap(), 155);
        assert_eq!(resolve_name("Launch_Media").unwrap(), 226);
        assert_eq!(resolve_name("Launch_App1").unwrap(), 157);
        assert_eq!(resolve_name("Launch_App2").unwrap(), 148);
    }

    #[test]
    fn the_wheel_is_not_a_key_and_has_a_code_of_its_own() {
        // The wheel is a relative axis of the pointer, and the two numbers of
        // it -- REL_WHEEL 0x08 and REL_HWHEEL 0x06 -- are keys of the
        // keyboard already (the `7` and the `5`). The names answer codes
        // above KEY_MAX (0x2ff), which no key has, so a hotkey of the wheel
        // parses and the watcher of the box, which watches EV_KEY, never
        // fires it.
        for name in ["WheelUp", "WheelDown", "WheelLeft", "WheelRight"] {
            let code = resolve_name(name).expect("the name of the wheel is carried");
            assert!(
                code > 0x2ff,
                "{name} is {code}, and it must be above KEY_MAX"
            );
            assert_ne!(code, 8, "{name} must not be the `7` key");
            assert_ne!(code, 6, "{name} must not be the `5` key");
        }
        let mut codes: Vec<u16> = WHEEL_NAMES.iter().map(|(_, code)| *code).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), WHEEL_NAMES.len(), "two names share a code");
    }

    #[test]
    fn every_alias_points_at_a_key_of_the_table() {
        for (alias, target) in KEY_ALIASES {
            assert!(code_of(target).is_some(), "{alias} points at no key");
        }
    }

    #[test]
    fn every_modifier_is_a_key_the_table_knows() {
        for (name, _) in MODIFIERS {
            assert!(resolve_name(name).is_ok(), "{name} is no key");
        }
    }

    #[test]
    fn a_modifier_of_the_v2_spelling_is_carried_by_the_stroke() {
        let strokes = parse_spec("LWin+a").unwrap();
        assert_eq!(strokes[0].code, 30);
        assert_eq!(strokes[0].modifiers, vec!["lwin"]);
    }
}
