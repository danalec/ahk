//! Key codes and key specifications.
//!
//! A port of `src/yoko_mcp/keycodes.py` from the Python `yoko-mcp`.
//!
//! There are three ways to press a key on the lab box, and they are not
//! equally trustworthy. Measured on 2026-10-25 (ydotool 1.0.4):
//!
//! 1. `ydotool type -- "TEXT"` is **exact**: every character arrives once,
//!    in order, and nothing is left held down.
//! 2. `ydotool key "Return"` is **a silent no-op**: ydotool 1.x takes
//!    numeric keycodes and reads a name as zero.
//! 3. `ydotool key 28` presses the key, but the keyup is late enough that
//!    the compositor's auto-repeat fires, and the key stays held.
//!
//! So the codes here are what a uinput device needs to emit an event at
//! all, and what turns a key *name* into the numeric code ydotool 1.x
//! would want. They are the Linux input event codes, which is the
//! numbering ydotool 1.x uses; they were confirmed against the box by
//! pressing keys and reading the screen: 14 is Backspace, 22 is `u`, 30 is
//! `a`, 31 is `s`, 32 is `d`, 46 is `c`.

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
];

/// The names people actually write, mapped onto the codes above.
///
/// The spelling is the X11/ydotool one (`BackSpace`, `Return`, `Esc`),
/// plus the AutoHotkey one (`Enter`, `Del`), because both dialects turn up
/// in the scripts this lab runs.
pub const KEY_ALIASES: &[(&str, &str)] = &[
    ("esc", "ESC"),
    ("escape", "ESC"),
    ("space", "SPACE"),
    ("spacebar", "SPACE"),
    ("return", "ENTER"),
    ("enter", "ENTER"),
    ("backspace", "BACKSPACE"),
    ("bksp", "BACKSPACE"),
    ("bs", "BACKSPACE"),
    ("tab", "TAB"),
    ("del", "DELETE"),
    ("delete", "DELETE"),
    ("ins", "INSERT"),
    ("insert", "INSERT"),
    ("home", "HOME"),
    ("end", "END"),
    ("pageup", "PAGEUP"),
    ("pgup", "PAGEUP"),
    ("pagedown", "PAGEDOWN"),
    ("pgdn", "PAGEDOWN"),
    ("up", "UP"),
    ("down", "DOWN"),
    ("left", "LEFT"),
    ("right", "RIGHT"),
    ("ctrl", "LEFTCTRL"),
    ("control", "LEFTCTRL"),
    ("lctrl", "LEFTCTRL"),
    ("rctrl", "RIGHTCTRL"),
    ("shift", "LEFTSHIFT"),
    ("lshift", "LEFTSHIFT"),
    ("rshift", "LEFTSHIFT"),
    ("alt", "LEFTALT"),
    ("lalt", "LEFTALT"),
    ("ralt", "RIGHTALT"),
    ("super", "LEFTMETA"),
    ("win", "LEFTMETA"),
    ("meta", "LEFTMETA"),
    ("cmd", "LEFTMETA"),
];

/// The modifier names, and the code that identifies each one.
pub const MODIFIERS: &[(&str, &str)] = &[
    ("ctrl", "LEFTCTRL"),
    ("control", "LEFTCTRL"),
    ("shift", "LEFTSHIFT"),
    ("alt", "LEFTALT"),
    ("super", "LEFTMETA"),
    ("meta", "LEFTMETA"),
    ("win", "LEFTMETA"),
];

/// A key name that could not be understood.
#[derive(Debug, thiserror::Error)]
#[error("unknown key name: {0:?}")]
pub struct UnknownKey(pub String);

/// The evdev code for a key name.
///
/// Accepts the aliases above (`Return`, `Del`, `ctrl`...), the bare evdev
/// names (`LEFTCTRL`, `F7`), and a single character (`a`, `7`, `/`).
pub fn resolve_name(name: &str) -> Result<u16, UnknownKey> {
    let key = name.trim();
    if key.is_empty() {
        return Err(UnknownKey(name.to_string()));
    }
    let lowered = key.to_lowercase();
    if let Some((_, code)) = KEY_ALIASES.iter().find(|(alias, _)| *alias == lowered) {
        return Ok(code_of(code));
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
fn code_of(name: &str) -> u16 {
    EVDEV.iter().find(|(name_, _)| *name_ == name).map(|(_, code)| *code).unwrap_or(0)
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
                    EVDEV.iter().find(|(name_, _)| *name_ == c.to_string()).map(|(_, code)| *code)
                });
            } else {
                return None;
            }
        }
    };
    EVDEV.iter().find(|(name_, _)| *name_ == name).map(|(_, code)| *code)
}

/// The key that types a character, and whether it needs the shift key.
///
/// The layout is the US one, the layout of the keyboard of the lab box.
/// A character that the layout does not have -- an accented letter, a
/// currency sign -- has no key here, and the caller is told so instead of
/// typing something else.
pub fn character_stroke(character: char) -> Option<(u16, bool)> {
    let shifted = |name: &str| EVDEV.iter().find(|(key, _)| *key == name).map(|(_, code)| (*code, true));
    let plain = |name: &str| EVDEV.iter().find(|(key, _)| *key == name).map(|(_, code)| (*code, false));
    if character.is_ascii_alphabetic() {
        let name = character.to_ascii_uppercase().to_string();
        let code = EVDEV.iter().find(|(key, _)| *key == name).map(|(_, code)| *code)?;
        return Some((code, character.is_ascii_uppercase()));
    }
    if character.is_ascii_digit() {
        let name = character.to_string();
        return EVDEV.iter().find(|(key, _)| *key == name).map(|(_, code)| (*code, false));
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
/// Accepts the shapes people write: `Return`, `ctrl+c`, `ctrl+shift+t`,
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
        strokes.push(KeyStroke { code: resolve_name(&key_part)?, modifiers });
    }
    if strokes.is_empty() {
        return Err(UnknownKey(spec.to_string()));
    }
    Ok(strokes)
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
}
