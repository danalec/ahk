//! The name of a hotkey of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the v1
//! writes a hotkey in a line of its own, and the name of it is a set of
//! prefixes, a set of modifiers and the name of a key. The name that is not
//! one of those is refused with "Invalid hotkey.".

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::keys::is_the_name_of_a_key;

/// Whether the name is the name of a command of the v1.
///
/// The line of a command is not the line of a hotkey: the `if (1) :: {` is
/// the `if` of the v1, and the v1 says so.
pub(crate) fn is_the_name_of_a_command(name: &str) -> bool {
    super::table::BUILTINS.iter().any(|builtin| {
        builtin.kind == crate::builtins::BuiltinKind::Command
            && builtin.name.eq_ignore_ascii_case(name)
    })
}

/// Whether the command is a context of the v1 that a hotkey may not carry.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
/// contexts of the v1 are refused in the action of a hotkey that is written
/// in one line, and the refusal is "Invalid single-line hotkey/hotstring.".
pub(crate) fn the_context_of_the_v1_is_refused_in_a_hotkey(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "ifwinactive"
            | "ifwinnotactive"
            | "ifwinexist"
            | "ifwinnotexist"
            | "ifexist"
            | "ifnotexist"
            | "ifequal"
            | "ifnotequal"
            | "ifgreater"
            | "ifless"
            | "ifbetween"
            | "ifnotbetween"
            | "ifin"
            | "ifnotin"
            | "ifinstring"
            | "ifnotinstring"
            | "ifis"
            | "ifisnot"
    )
}

/// The name of a hotkey of the v1.
pub(crate) struct TheName {
    /// The name as the script wrote it.
    pub spelling: String,
    /// The name without the prefixes that do not make it another hotkey.
    ///
    /// The `$` and the `~` do not: the hotkey with them and the hotkey
    /// without them are the same one, and the v1 refuses the second one
    /// with "Duplicate hotkey.". The `*` does: the hotkey with the
    /// wildcard and the hotkey without it are two hotkeys.
    pub identity: String,
}

/// Whether the name is the name of a hotkey of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the name
/// is a set of prefixes -- the `$` of the hook, the `~` of the tilde and the
/// `*` of the wildcard -- a set of modifiers -- the `^` of the control, the
/// `!` of the alt, the `+` of the shift and the `#` of the win, with the `<`
/// and the `>` naming the side of the key -- and the name of a key. The word
/// "up" may follow the name of the key, and the hotkey fires when the key
/// comes up.
///
/// The name of a key is one of the keys of the list of the documentation, a
/// single character, or the spelling of a virtual key or of a scan code. The
/// name may not have a space in it: the `Browser Back` is not a key, and the
/// `Browser_Back` is.
///
/// The combination of two keys -- the `RCtrl & RShift` -- is a hotkey of its
/// own: the two keys are the ones of it, and a modifier is not one of them.
pub(crate) fn the_name_of_a_hotkey(name: &str) -> Option<TheName> {
    let mut rest = name;    // The prefixes. The order of them does not matter, and the `$` and the
    // `~` do not make the hotkey another one.
    let mut wildcard = false;
    let mut tilde = false;
    let mut hook = false;
    while let Some(first) = rest.chars().next() {
        match first {
            '*' if !wildcard => {
                wildcard = true;
                rest = &rest[1..];
            }
            '~' if !tilde => {
                tilde = true;
                rest = &rest[1..];
            }
            '$' if !hook => {
                hook = true;
                rest = &rest[1..];
            }
            _ => break,
        }
    }
    // The identity of the hotkey: the wildcard is one of them, and the order
    // of the modifiers is not.
    let mut identity = String::new();
    if wildcard {
        identity.push('*');
    }
    // The word "up" may follow the name of the key, and the hotkey fires
    // when the key comes up.
    let (key, up) = the_name_without_the_up(rest);
    // The combination of two keys is a hotkey of its own, and the modifiers
    // are not one of its keys.
    if key.contains('&') {
        let mut parts = key.split('&').map(|part| part.trim());
        let first = parts.next()?;
        let second = parts.next()?;
        // Three keys are not a hotkey.
        if parts.next().is_some() {
            return None;
        }
        if !is_a_key(first) || !is_a_key(second) {
            return None;
        }
        identity.push_str(&first.to_lowercase());
        identity.push('&');
        identity.push_str(&second.to_lowercase());
        if up {
            identity.push('u');
        }
        return Some(TheName { spelling: name.to_string(), identity });
    }
    // The modifiers, with the side of the key that the `<` and the `>` name.
    let mut modifiers: Vec<String> = Vec::new();
    let mut left = false;
    let mut rest = key;
    while let Some(first) = rest.chars().next() {
        match first {
            '<' => {
                left = true;
                rest = &rest[1..];
            }
            '^' | '!' | '+' | '#' => {
                rest = &rest[1..];
                let right = rest.starts_with('>');
                if right {
                    rest = &rest[1..];
                }
                let mut modifier = String::new();
                if left {
                    modifier.push('l');
                }
                modifier.push(match first {
                    '^' => 'c',
                    '!' => 'a',
                    '+' => 's',
                    _ => 'w',
                });
                if right {
                    modifier.push('r');
                }
                modifiers.push(modifier);
                left = false;
            }
            _ => break,
        }
    }
    // A `<` that names no modifier is not a name.
    if left {
        return None;
    }
    modifiers.sort();
    for modifier in &modifiers {
        identity.push_str(modifier);
    }
    // The key of the hotkey.
    if !is_a_key(rest) {
        return None;
    }
    identity.push_str(&rest.to_lowercase());
    // The hotkey that fires when the key comes up is another hotkey.
    if up {
        identity.push('u');
    }
    Some(TheName { spelling: name.to_string(), identity })
}

/// The name of the key without the word "up", and whether the word was there.
///
/// The word is the last one of the name, and a space comes before it. The
/// name that has no word keeps its spaces: the `x ` of a hotkey that is
/// written with a space before the two colons is not a key.
fn the_name_without_the_up(name: &str) -> (&str, bool) {
    let trimmed = name.trim_end();
    if trimmed.len() < 3 {
        return (name, false);
    }
    let rest = &trimmed[trimmed.len() - 2..];
    if !rest.eq_ignore_ascii_case("up") {
        return (name, false);
    }
    let at = trimmed.len() - 2;
    // A space comes before the word.
    if at == 0 || !trimmed[..at].ends_with(char::is_whitespace) {
        return (name, false);
    }
    (trimmed[..at].trim_end(), true)
}

/// Whether the text is the name of a key.
///
/// The name of a key may not have a space at its sides: the `x ` of a
/// hotkey that is written with a space before the two colons is not a key.
fn is_a_key(text: &str) -> bool {
    if text.starts_with(char::is_whitespace) || text.ends_with(char::is_whitespace) {
        return false;
    }
    is_the_name_of_a_key(text)
}

#[cfg(test)]
mod tests {
    use super::the_name_of_a_hotkey;

    #[test]
    fn the_names_of_the_hotkeys() {
        assert!(the_name_of_a_hotkey("^!a").is_some());
        assert!(the_name_of_a_hotkey("a & b").is_some());
        assert!(the_name_of_a_hotkey("RCtrl & RShift").is_some());
        assert!(the_name_of_a_hotkey("a & b & c").is_none());
        assert!(the_name_of_a_hotkey("^!a b").is_none());
    }
}