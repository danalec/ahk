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
use alloc::string::ToString;

use crate::runtime::Value;

/// Whether the number of the v1 is a number and not a pure number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// whole number of base ten that is written without the zeros in front of
/// it is a number -- the `255` is the number 255, and the format of the
/// integers writes it -- and the number with the zeros in front of it,
/// the hex and the float are pure numbers: they keep the text that was
/// written.
///
/// The sign of the minus is a number of its own: the v1 computes it, and
/// the format writes it -- the `-00123` is the number -123.
pub(crate) fn the_number_is_a_number(text: &str) -> bool {
    let negative = text.starts_with('-');
    let body = text.strip_prefix('-').unwrap_or(text);
    // The float is a pure number: it keeps the text that was written.
    if the_text_is_a_float(body) {
        return false;
    }
    // The sign of the minus makes the number a number.
    if negative {
        return true;
    }
    // The zeros in front of the number make it a pure number: they are a
    // part of the text that was written.
    !(body.len() > 1 && body.starts_with('0'))
}

/// The text of the number of the v1, when the number does not fit.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// whole number of base ten that does not fit in the machine is the one
/// that fits -- the "100000000000000000000" is "9223372036854775807", and
/// the "-100000000000000000000" is "-9223372036854775808". The hex and the
/// float keep the text that was written: the "0xFFFFFFFFFFFFFFFF" is
/// "0xFFFFFFFFFFFFFFFF", and the "1.5e400" is "1.5e400".
pub(crate) fn the_text_of_the_number_that_fits(text: &str) -> String {
    if the_text_is_a_hex(text) || the_text_is_a_float(text) {
        return text.to_string();
    }
    match text.parse::<i64>() {
        Ok(_) => text.to_string(),
        Err(_) => {
            // The number does not fit in the machine, and the v1 keeps the
            // one that fits: the largest, or the smallest when the sign is
            // the minus.
            let number = if text.starts_with('-') { i64::MIN } else { i64::MAX };
            number.to_string()
        }
    }
}

/// Whether the text is the hex of a number.
pub(crate) fn the_text_is_a_hex(text: &str) -> bool {
    text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).is_some()
}

/// Whether the text is a number with a point or with an exponent.
pub(crate) fn the_text_is_a_float(text: &str) -> bool {
    !the_text_is_a_hex(text) && (text.contains('.') || text.contains('e') || text.contains('E'))
}

/// The count of the `Loop` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the count
/// of the `Loop` of the v1 is a number and not an expression, and the loop is
/// skipped when the text is not a number: the `loop n` is skipped, and so are
/// the `loop 1+1` and the `loop 2.7`. The hex is a number: the `loop 0x3` is
/// three turns.
///
/// The text is trimmed, and the sign of the plus is a part of it: the `loop
/// +3` is three turns.
pub(crate) fn the_count_of_the_loop(text: &str) -> i64 {
    let text = text.trim();
    if let Ok(number) = text.parse::<i64>() {
        return number;
    }
    if the_text_is_a_hex(text) {
        return the_whole_number_of_a_hex(text);
    }
    0
}

/// The whole number that the hex holds.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the hex
/// that does not fit in the machine is the largest whole number that
/// fits: the `0xFFFFFFFFFFFFFFFF` is 9223372036854775807, and not -1.
pub(crate) fn the_whole_number_of_a_hex(text: &str) -> i64 {
    let digits = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).unwrap_or(text);
    match u64::from_str_radix(digits, 16) {
        Ok(number) if number <= i64::MAX as u64 => number as i64,
        _ => i64::MAX,
    }
}

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
        let number = the_whole_number_of_a_hex(&digits);
        return Some(if negative { number.wrapping_neg() } else { number });
    }
    // The whole number of base ten: the digits, and nothing else.
    let digits: String = body.chars().take_while(|character| character.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let number: i64 = digits.parse().unwrap_or(i64::MAX);
    Some(if negative { number.wrapping_neg() } else { number })
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
        let number = the_whole_number_of_a_hex(&digits);
        return Some(Value::Int(if negative { number.wrapping_neg() } else { number }));
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
    // The whole number that does not fit in the machine is the one that
    // fits: the largest, or the smallest when the sign is the minus.
    //
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the
    // "100000000000000000000" is 9223372036854775807, and the
    // "-100000000000000000000" is -9223372036854775808.
    match text.parse::<i64>() {
        Ok(number) => Some(Value::Int(number)),
        Err(_) => Some(Value::Int(if negative { i64::MIN } else { i64::MAX })),
    }
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
