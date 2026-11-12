//! The characters of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `Chr`
//! of the v1 takes the whole number that the text of its argument starts
//! with -- "65abc" is 65 -- and a text that does not start with a number is
//! the empty text. The code that is zero, that is negative or that is not a
//! character of the Unicode is the empty text as well.

use alloc::string::{String, ToString};

use crate::runtime::{Runtime, RuntimeError, Value};

/// `Chr(n)`: the character of the code.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the code
/// is the number that the argument holds, and the text of the argument is
/// read as the whole number that it starts with. A code that is zero, that
/// is negative or that is not a character is the empty text.
pub(crate) fn chr(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(code) = the_code_of(rt, values) else {
        return Ok(Value::Text(String::new()));
    };
    if code <= 0 {
        return Ok(Value::Text(String::new()));
    }
    match u32::try_from(code).ok().and_then(char::from_u32) {
        Some(character) => Ok(Value::Text(character.to_string())),
        None => Ok(Value::Text(String::new())),
    }
}

/// The code that the argument of the `Chr` holds.
fn the_code_of(rt: &mut Runtime<'_>, values: &[Value]) -> Option<i64> {
    match values.first() {
        Some(Value::Int(number)) => Some(*number),
        Some(Value::Float(number)) => Some(*number as i64),
        Some(other) => crate::v1::the_whole_number_at_the_start_of_a_text(&rt.the_text_of(other)),
        None => None,
    }
}
