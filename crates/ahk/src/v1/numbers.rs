//! The numbers of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
//! reads a text as a number in three ways, and the built-ins do not all use
//! the same one.
//!
//! - The `Abs`, the `Max`, the `Min` and the `Mod` want a text that *is* a
//!   number: "5.5" is 5.5, "0x10" is 16 and " 5 " is 5, and "5abc" is not
//!   a number at all.
//! - The other functions of the math want the number that the text *starts*
//!   with: "65abc" is 65 and "1e3" is 1000.
//! - The `Chr` wants the whole number that the text starts with: "1e3" is 1.
//!
//! The exponent of the v1 needs the point of the number: "1.5e2" is 150 and
//! "1e2" is not a number of the first way. The second way takes the
//! exponent either way, and the third one stops at the point.

use alloc::format;
use alloc::string::String;

use crate::runtime::Value;

/// The number of a text in the v1, when the text is one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: "5" is 5,
/// and "5.5" is 5.5, and " 5 " is 5, and "0x10" is 16, and "1.5e2" is 150.
/// A text that is not a number ("abc", "5abc", "1e2" and the empty text) is
/// not a number at all, and the operation that wants one is blank.
pub(crate) fn the_number_of_a_text(text: &str) -> Option<Value> {
    let number = the_number_that_the_text_holds(text, true)?;
    Some(number)
}

/// The number that the text of the v1 starts with.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// number is the one that the text starts with -- "65abc" is 65 and
/// "abc65" is nothing -- and the exponent does not need the point of the
/// number: "1e3" is 1000.
pub(crate) fn the_number_at_the_start_of_a_text(text: &str) -> Option<Value> {
    the_number_that_the_text_holds(text, false)
}

/// The whole number that the text of the v1 starts with.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `Chr`
/// wants the whole number that the text starts with, and it stops at the
/// point: "1e3" is 1 and "1.5e2" is 1. The hex is a whole number as well:
/// "0x41" is 65.
pub(crate) fn the_whole_number_at_the_start_of_a_text(text: &str) -> Option<i64> {
    let text = text.trim();
    let (negative, body) = the_sign_of(text);
    if body.is_empty() {
        return None;
    }
    // The hex of the v1: 0x10 is 16.
    if let Some(hex) = the_hex_of(body) {
        let digits: String = hex.chars().take_while(|character| character.is_ascii_hexdigit()).collect();
        if digits.is_empty() {
            return None;
        }
        let number = i64::from_str_radix(&digits, 16).ok()?;
        return Some(if negative { -number } else { number });
    }
    // The whole number of base ten: the digits, and nothing else.
    let digits: String = body.chars().take_while(|character| character.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let number: i64 = digits.parse().ok()?;
    Some(if negative { -number } else { number })
}

/// The number that the text holds.
///
/// `whole` asks for the text to *be* the number. Without it, the number is
/// the one that the text starts with, and the rest of the text is left.
fn the_number_that_the_text_holds(text: &str, whole: bool) -> Option<Value> {
    let text = text.trim();
    let (negative, body) = the_sign_of(text);
    if body.is_empty() {
        return None;
    }
    // The hex of the v1: 0x10 is 16.
    if let Some(hex) = the_hex_of(body) {
        let digits: String = hex.chars().take_while(|character| character.is_ascii_hexdigit()).collect();
        if digits.is_empty() || (whole && digits.len() != hex.len()) {
            return None;
        }
        let number = i64::from_str_radix(&digits, 16).ok()?;
        return Some(Value::Int(if negative { -number } else { number }));
    }
    // A number of base ten: the digits, the point and the exponent.
    let mut number = String::new();
    let mut digits = 0usize;
    let mut point = false;
    for character in body.chars() {
        match character {
            '0'..='9' => {
                number.push(character);
                digits += 1;
            }
            '.' if !point => {
                point = true;
                number.push(character);
            }
            _ => break,
        }
    }
    if digits == 0 {
        return None;
    }
    // The exponent: the 'e', the sign and the digits. The exponent of the v1
    // needs the point of the number, and the way that takes the number that
    // the text starts with takes the exponent either way.
    let rest = body.chars().skip(number.chars().count()).collect::<String>();
    let mut exponent = String::new();
    if rest.starts_with('e') || rest.starts_with('E') {
        let mut candidate = String::from("e");
        for character in rest.chars().skip(1) {
            match character {
                '+' | '-' if candidate.len() == 1 => candidate.push(character),
                '0'..='9' => candidate.push(character),
                _ => break,
            }
        }
        let has_digits = candidate.chars().skip(1).any(|character| character.is_ascii_digit());
        if has_digits && (!whole || point) {
            exponent = candidate;
        }
    }
    let rest = rest.chars().skip(exponent.chars().count()).collect::<String>();
    if whole && !rest.is_empty() {
        return None;
    }
    let text = format!("{}{}{}", if negative { "-" } else { "" }, number, exponent);
    if number.contains('.') || !exponent.is_empty() {
        // The number with a point or with an exponent is a float.
        return text.parse::<f64>().ok().map(Value::Float);
    }
    text.parse::<i64>().ok().map(Value::Int)
}

/// The sign of the text, and the text without it.
fn the_sign_of(text: &str) -> (bool, &str) {
    match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    }
}

/// The hex of the text, when it starts with one.
fn the_hex_of(body: &str) -> Option<&str> {
    body.strip_prefix("0x").or_else(|| body.strip_prefix("0X"))
}
