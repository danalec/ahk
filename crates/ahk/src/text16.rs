//! The code units of a text.
//!
//! AutoHotkey v2 holds its strings in UTF-16, and the tree holds them in UTF-8:
//! a string of the tree is a `String`, and one character of it is one char.
//! Measured on the binary of v2.0.19 on 2026-11-10: `StrLen(Chr(0xD83D))` is
//! one, `StrLen(Chr(0x1F600))` is two, `Ord(Chr(0xD83D))` is 0xD83D and
//! `Ord(SubStr(Chr(0x1F600), 2))` is 0xDE00 -- the strings of the dialect are
//! counted and cut in the code units of UTF-16, and a lone surrogate is a unit
//! of its own, which a `char` of Rust cannot be.
//!
//! The tree writes a lone surrogate of the dialect as one character of the
//! private-use area: the block U+E000 to U+E7FF stands for the 2048 surrogates
//! (U+E000 is U+D800), and every function that counts, cuts or reads a code
//! unit goes through this module. A text that the script writes with a
//! character of that block is read as a surrogate here -- the block is a
//! compromise, and it is recorded as one.
use alloc::string::String;
use alloc::vec::Vec;

/// The first character of the private-use area that stands for a surrogate.
const THE_FIRST_STAND_IN: u32 = 0xE000;
/// The first surrogate of the dialect.
const THE_FIRST_SURROGATE: u32 = 0xD800;
/// The last surrogate of the dialect.
const THE_LAST_SURROGATE: u32 = 0xDFFF;

/// The character that stands for a surrogate code unit.
fn the_stand_in_of(unit: u16) -> char {
    let code = THE_FIRST_STAND_IN + (unit as u32 - THE_FIRST_SURROGATE);
    char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER)
}

/// The surrogate that a character of the private-use block stands for, when it
/// is one of the stand-ins.
pub(crate) fn the_surrogate_of_a_character(character: char) -> Option<u16> {
    let code = character as u32;
    if !(THE_FIRST_STAND_IN..THE_FIRST_STAND_IN + 0x800).contains(&code) {
        return None;
    }
    Some((code - THE_FIRST_STAND_IN) as u16 + THE_FIRST_SURROGATE as u16)
}

/// The code units of a text, the way the dialect counts them.
pub(crate) fn the_code_units_of(text: &str) -> Vec<u16> {
    let mut units = Vec::with_capacity(text.len());
    for character in text.chars() {
        if let Some(unit) = the_surrogate_of_a_character(character) {
            units.push(unit);
            continue;
        }
        let code = character as u32;
        if code > 0xFFFF {
            let code = code - 0x10000;
            units.push(0xD800 + (code >> 10) as u16);
            units.push(0xDC00 + (code & 0x3FF) as u16);
        } else {
            units.push(code as u16);
        }
    }
    units
}

/// The number of the code units of a text.
pub(crate) fn the_number_of_the_code_units(text: &str) -> usize {
    text.chars()
        .map(|character| {
            if the_surrogate_of_a_character(character).is_some() || (character as u32) <= 0xFFFF {
                1
            } else {
                2
            }
        })
        .sum()
}


/// The text of a run of code units.
pub(crate) fn the_text_of(units: &[u16]) -> String {
    let mut text = String::with_capacity(units.len());
    let mut at = 0;
    while at < units.len() {
        let unit = units[at];
        if (0xD800..=0xDBFF).contains(&unit)
            && units
                .get(at + 1)
                .map(|next| (0xDC00..=0xDFFF).contains(next))
                .unwrap_or(false)
        {
            let high = (unit - 0xD800) as u32;
            let low = (units[at + 1] - 0xDC00) as u32;
            let code = 0x10000 + (high << 10) + low;
            text.push(char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER));
            at += 2;
            continue;
        }
        if (THE_FIRST_SURROGATE..=THE_LAST_SURROGATE).contains(&(unit as u32)) {
            text.push(the_stand_in_of(unit));
        } else {
            text.push(char::from_u32(unit as u32).unwrap_or(char::REPLACEMENT_CHARACTER));
        }
        at += 1;
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lone_surrogate_is_one_unit_of_its_own() {
        let text = the_text_of(&[0xD83D]);
        assert_eq!(the_number_of_the_code_units(&text), 1);
        assert_eq!(the_code_units_of(&text), vec![0xD83D]);
    }

    #[test]
    fn a_pair_is_two_units_and_one_character() {
        let text = the_text_of(&[0xD83D, 0xDE00]);
        assert_eq!(the_number_of_the_code_units(&text), 2);
        assert_eq!(the_code_units_of(&text), vec![0xD83D, 0xDE00]);
        assert_eq!(text.chars().count(), 1);
    }

    #[test]
    fn the_units_of_a_text_come_back_the_same() {
        for text in ["", "hello", "a😀b", "héllo"] {
            let units = the_code_units_of(text);
            assert_eq!(the_text_of(&units), text);
            assert_eq!(the_number_of_the_code_units(text), units.len());
        }
    }
}