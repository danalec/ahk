//! The names of the keys of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the v1
//! checks the name of a key when the script loads, and the name is one of
//! the keys of the list of the documentation, or a single character, or the
//! spelling of a virtual key or of a scan code.
//!
//! The list is the one of the page "List of Keys" of the documentation of
//! the v1. The names that the page has and the binary refuses are not keys
//! of the v1: the `Win` and the `PrtSc` are not there, and the `Joy0` and
//! the `Joy33` are not buttons of a controller.

/// The keys of the v1, as the list of the documentation spells them.
const KEYS: &[&str] = &[
    // The mouse.
    "LButton",
    "RButton",
    "MButton",
    "XButton1",
    "XButton2",
    "WheelUp",
    "WheelDown",
    "WheelLeft",
    "WheelRight",
    // The general keys.
    "CapsLock",
    "Space",
    "Tab",
    "Enter",
    "Return",
    "Escape",
    "Esc",
    "Backspace",
    "BS",
    // The cursor control keys.
    "ScrollLock",
    "Delete",
    "Del",
    "Insert",
    "Ins",
    "Home",
    "End",
    "PgUp",
    "PgDn",
    "Up",
    "Down",
    "Left",
    "Right",
    // The numpad.
    "Numpad0",
    "Numpad1",
    "Numpad2",
    "Numpad3",
    "Numpad4",
    "Numpad5",
    "Numpad6",
    "Numpad7",
    "Numpad8",
    "Numpad9",
    "NumpadIns",
    "NumpadEnd",
    "NumpadDown",
    "NumpadPgDn",
    "NumpadLeft",
    "NumpadClear",
    "NumpadRight",
    "NumpadHome",
    "NumpadPgUp",
    "NumpadDot",
    "NumpadDel",
    "NumLock",
    "NumpadDiv",
    "NumpadMult",
    "NumpadAdd",
    "NumpadSub",
    "NumpadEnter",
    // The function keys.
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
    // The modifier keys. The `Win` is not one of them: the v1 has the `LWin`
    // and the `RWin`, and it has no name that is both.
    "LWin",
    "RWin",
    "Ctrl",
    "Alt",
    "Shift",
    "LCtrl",
    "RCtrl",
    "LControl",
    "RControl",
    "LShift",
    "RShift",
    "LAlt",
    "RAlt",
    // The multimedia keys.
    "Browser_Back",
    "Browser_Forward",
    "Browser_Refresh",
    "Browser_Stop",
    "Browser_Search",
    "Browser_Favorites",
    "Browser_Home",
    "Volume_Mute",
    "Volume_Down",
    "Volume_Up",
    "Media_Next",
    "Media_Prev",
    "Media_Stop",
    "Media_Play_Pause",
    "Launch_Mail",
    "Launch_Media",
    "Launch_App1",
    "Launch_App2",
    // The other keys.
    "AppsKey",
    "PrintScreen",
    "CtrlBreak",
    "Pause",
    "Break",
    "Help",
    "Sleep",
];

/// The controls of a controller of the v1, besides its buttons.
const CONTROLS: &[&str] = &[
    "JoyX",
    "JoyY",
    "JoyZ",
    "JoyR",
    "JoyU",
    "JoyV",
    "JoyPOV",
    "JoyName",
    "JoyButtons",
    "JoyAxes",
    "JoyInfo",
];

/// The scan codes that the v1 knows.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: every
/// code of the machine was asked, and the ones that the v1 refuses are the
/// ones that it has no key for.
const SCAN_CODES: &[(u8, u8)] = &[
    (0x01, 0x54),
    (0x56, 0x5F),
    (0x62, 0x6F),
    (0x71, 0x71),
    (0x73, 0x73),
    (0x76, 0x76),
    (0x7B, 0x7B),
    (0x7C, 0x7C),
    (0x7E, 0x7E),
];

/// Whether the name is the name of a key of the v1.
///
/// The name does not know its case, and the text that surrounds it is not
/// a part of it.
pub(crate) fn is_the_name_of_a_key(name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return false;
    }
    // Measured on the binary of v1.1.37.02 on 2026-11-03: any single
    // character is the name of a key.
    if name.chars().count() == 1 {
        return true;
    }
    if KEYS.iter().any(|key| key.eq_ignore_ascii_case(name)) {
        return true;
    }
    is_the_name_of_a_controller(name) || is_the_spelling_of_a_code(name)
}

/// Whether the name is the name of a button or of a control of a controller.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the buttons are the
/// `Joy1` up to the `Joy32`, the controls are the axes and the hat of the
/// stick, and the number of the controller may come in front of the name of
/// it -- the `2Joy1` is the first button of the second controller.
fn is_the_name_of_a_controller(name: &str) -> bool {
    let rest = match name.find(|character: char| !character.is_ascii_digit()) {
        Some(0) => name,
        Some(at) => {
            // The number of the controller is at most sixteen.
            let number = match name[..at].parse::<u32>() {
                Ok(number) => number,
                Err(_) => return false,
            };
            if !(1..=16).contains(&number) {
                return false;
            }
            &name[at..]
        }
        None => return false,
    };
    if CONTROLS.iter().any(|control| control.eq_ignore_ascii_case(rest)) {
        return true;
    }
    // The buttons: the `Joy1` up to the `Joy32`.
    if rest.len() < 4 || !rest[..3].eq_ignore_ascii_case("joy") {
        return false;
    }
    match rest[3..].parse::<u32>() {
        Ok(number) => (1..=32).contains(&number),
        Err(_) => false,
    }
}

/// Whether the name is the spelling of a virtual key or of a scan code.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
/// number is in hexadecimal, and the byte that it ends in is the code: the
/// `vkFF` is a key and the `vk100` is the key of the `vk00`, which is not a
/// key at all. Every virtual key of the machine is a key, and the scan code
/// is one of the codes that the machine knows.
fn is_the_spelling_of_a_code(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    let (rest, is_a_scan_code) = if let Some(rest) = lowered.strip_prefix("vk") {
        (rest, false)
    } else if let Some(rest) = lowered.strip_prefix("sc") {
        (rest, true)
    } else {
        return false;
    };
    if rest.is_empty() || !rest.chars().all(|digit| digit.is_ascii_hexdigit()) {
        return false;
    }
    // The byte that the number ends in is the code.
    let digits = rest.as_bytes();
    let last = digits[digits.len() - 1];
    let code = match digits.len() {
        1 => the_number_of_a_digit(last),
        _ => the_number_of_a_digit(digits[digits.len() - 2]) * 16 + the_number_of_a_digit(last),
    };
    if code == 0 {
        return false;
    }
    if is_a_scan_code {
        return SCAN_CODES
            .iter()
            .any(|(first, last)| code >= *first as u32 && code <= *last as u32);
    }
    true
}

/// The number of a hexadecimal digit.
fn the_number_of_a_digit(digit: u8) -> u32 {
    match digit {
        b'0'..=b'9' => (digit - b'0') as u32,
        b'a'..=b'f' => (digit - b'a') as u32 + 10,
        _ => 0,
    }
}
