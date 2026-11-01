//! The text functions of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! `SubStr` of the v1 counts the places the way the v1 counts them -- the
//! place that is zero is the last character of the text, and a place that is
//! negative counts from the end of it and takes one character more than the v2
//! takes -- and the `InStr` of the v1 reads the sense of the case as the
//! number that the text holds: "1" is the case that is sensitive and "on" is
//! not.

use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, Value};

/// `SubStr(String, StartingPos, Length)`: the part of the text.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the place
/// that is zero is the last character of the text, and a place that is
/// negative counts from the end of it: "-1" is the last two characters and
/// "-2" the last three. A place that is beyond the text is the empty text,
/// and the number of the characters that is negative leaves that many out at
/// the end of it.
pub(crate) fn sub_str(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text: Vec<char> = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default()
        .chars()
        .collect();
    let start = values.get(1).and_then(|value| value.to_number()).unwrap_or(0.0) as i64;
    let from = the_place_of_the_start(start, text.len());
    let answer = match values.get(2).and_then(|value| value.to_number()) {
        Some(length) => {
            let length = length as i64;
            if length < 0 {
                // A negative number of the characters leaves that many out at
                // the end of the text.
                let to = text.len().saturating_sub((-length) as usize).max(from);
                text[from.min(to)..to].iter().collect()
            } else {
                let to = (from + length as usize).min(text.len());
                text[from..to].iter().collect()
            }
        }
        None => text[from..].iter().collect(),
    };
    Ok(Value::Text(answer))
}

/// The place of the first character, counting from one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the place
/// that is zero is the last character of the text, and a place that is
/// negative counts from the end of it and takes one character more than the v2
/// takes.
fn the_place_of_the_start(start: i64, length: usize) -> usize {
    if start < 0 {
        (length as i64 + start - 1).max(0) as usize
    } else if start == 0 {
        length.saturating_sub(1)
    } else {
        ((start - 1) as usize).min(length)
    }
}

/// `InStr(Haystack, Needle, CaseSensitive, StartingPos, Occurrence)`: where
/// the needle is in the haystack.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the sense
/// of the case is the number that the text holds -- "1" is the case that is
/// sensitive and "on" is not -- and the needle that is empty is at the first
/// character of the haystack.
pub(crate) fn in_str(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let mut values = values.to_vec();
    // The sense of the case of the v1 is a number: the text that holds one
    // that is not zero is the case that is sensitive.
    if let Some(value) = values.get(2) {
        let number = crate::v1::the_number_at_the_start_of_a_text(&rt.the_text_of(value))
            .and_then(|number| number.to_number())
            .unwrap_or(0.0);
        values[2] = Value::Int(if number == 0.0 { 0 } else { 1 });
    }
    // The needle that is empty is at the first character of the haystack.
    if values.get(1).map(|value| value.to_text()).unwrap_or_default().is_empty() {
        return Ok(Value::Int(1));
    }
    crate::builtins::in_str(rt, &values, line, variant)
}
