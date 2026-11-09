//! The math functions of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! functions of the v1 take the text that holds a number, and a text that
//! does not hold one is the number zero for most of them -- and the empty
//! text for the `Abs`, the `Max` and the `Min`. The number that is outside
//! of the domain of the function is the empty text as well.
//!
//! The four functions that want a number that is a number -- the `Abs`, the
//! `Max`, the `Min` and the `Mod` -- and the other thirteen, that want the
//! number that the text starts with, are told apart in the module of the
//! numbers of the v1.

use alloc::format;
use alloc::string::String;

use crate::runtime::{Runtime, RuntimeError, Value};

/// The number of the argument, the way the v1 reads it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the text
/// of the argument is the number that it starts with -- "65abc" is 65 -- and
/// a text that does not start with a number is nothing at all.
fn the_number(rt: &mut Runtime<'_>, values: &[Value], at: usize) -> Option<Value> {
    let value = values.get(at)?;
    match value {
        Value::Int(_) | Value::Float(_) => Some(value.clone()),
        other => crate::v1::the_number_at_the_start_of_a_text(&rt.the_text_of(other)),
    }
}

/// The number of the argument, and zero when the text does not hold one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a text that
/// does not hold a number is the number zero for the functions that compute
/// with it, and the v1 does not fail.
fn the_number_or_zero(rt: &mut Runtime<'_>, values: &[Value], at: usize) -> f64 {
    the_number(rt, values, at)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0)
}

/// The number of the argument, when the text is a number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `Abs`,
/// the `Max`, the `Min` and the `Mod` want a text that is a number, and the
/// answer of a text that is not a number is the empty text.
fn the_whole_number(rt: &mut Runtime<'_>, values: &[Value], at: usize) -> Option<Value> {
    let value = values.get(at)?;
    match value {
        Value::Int(_) | Value::Float(_) => Some(value.clone()),
        other => crate::v1::the_number_of_a_text(&rt.the_text_of(other)),
    }
}

/// `Abs(x)`: the absolute value of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the answer
/// keeps the kind of the number -- a whole number stays a whole number --
/// and a text that is not a number is the empty text.
pub(crate) fn abs(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_whole_number(rt, values, 0) else {
        return Ok(Value::Text(String::new()));
    };
    Ok(match number {
        Value::Int(number) => Value::Int(number.abs()),
        other => Value::Float(other.to_number().unwrap_or(0.0).abs()),
    })
}

/// `Ceil(x)`: the whole number that is not less than the number.
pub(crate) fn ceil(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(
        crate::runtime::ceil(the_number_or_zero(rt, values, 0)) as i64,
    ))
}

/// `Floor(x)`: the whole number that is not greater than the number.
pub(crate) fn floor(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(
        crate::runtime::floor(the_number_or_zero(rt, values, 0)) as i64,
    ))
}

/// `Round(x, n)`: the number rounded, and the digits when there are some.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a half is
/// rounded away from the zero, the digits are the ones of the second
/// argument, and the answer is a text with that many digits. The answer is
/// a whole number when there are no digits.
pub(crate) fn round(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if values.get(1).is_none() {
        return Ok(Value::Int(crate::runtime::round(number) as i64));
    }
    let digits = the_number_or_zero(rt, values, 1) as i32;
    let factor = crate::runtime::powf(10.0, digits as f64);
    let rounded = crate::runtime::round(number * factor) / factor;
    if digits <= 0 {
        return Ok(Value::Int(rounded as i64));
    }
    Ok(Value::Text(format!("{:.*}", digits as usize, rounded)))
}

/// `Exp(x)`: the exponential of the number.
pub(crate) fn exp(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::exp(the_number_or_zero(
        rt, values, 0,
    ))))
}

/// `Ln(x)`: the natural logarithm of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a number
/// that is negative is the empty text, and the logarithm of the zero is the
/// infinity of the C library.
pub(crate) fn ln(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if number < 0.0 {
        return Ok(Value::Text(String::new()));
    }
    Ok(Value::Float(crate::runtime::ln(number)))
}

/// `Log(x)`: the logarithm of the number in base ten.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a number
/// that is negative is the empty text, and the logarithm of the zero is the
/// infinity of the C library.
pub(crate) fn log(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if number < 0.0 {
        return Ok(Value::Text(String::new()));
    }
    Ok(Value::Float(crate::runtime::log10(number)))
}

/// `Sqrt(x)`: the square root of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a number
/// that is negative is the empty text.
pub(crate) fn sqrt(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if number < 0.0 {
        return Ok(Value::Text(String::new()));
    }
    Ok(Value::Float(crate::runtime::sqrt(number)))
}

/// `Mod(x, y)`: the remainder of the division of the first by the second.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// divisor of zero is the empty text, and so is a text that is not a
/// number. The remainder has the sign of the dividend, and it is a whole
/// number when the two numbers are whole numbers.
pub(crate) fn modulo(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (Some(dividend), Some(divisor)) = (
        the_whole_number(rt, values, 0),
        the_whole_number(rt, values, 1),
    ) else {
        return Ok(Value::Text(String::new()));
    };
    let (dividend_number, divisor_number) = (
        dividend.to_number().unwrap_or(0.0),
        divisor.to_number().unwrap_or(0.0),
    );
    if divisor_number == 0.0 {
        return Ok(Value::Text(String::new()));
    }
    Ok(match (dividend, divisor) {
        (Value::Int(dividend), Value::Int(divisor)) => Value::Int(dividend % divisor),
        _ => Value::Float(dividend_number % divisor_number),
    })
}

/// `Sin(x)`: the sine of the number.
pub(crate) fn sin(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::sin(the_number_or_zero(
        rt, values, 0,
    ))))
}

/// `Cos(x)`: the cosine of the number.
pub(crate) fn cos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::cos(the_number_or_zero(
        rt, values, 0,
    ))))
}

/// `Tan(x)`: the tangent of the number.
pub(crate) fn tan(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::tan(the_number_or_zero(
        rt, values, 0,
    ))))
}

/// `ASin(x)`: the arc sine of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a number
/// that is outside of the domain is the empty text.
pub(crate) fn asin(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if !(-1.0..=1.0).contains(&number) {
        return Ok(Value::Text(String::new()));
    }
    Ok(Value::Float(crate::runtime::asin(number)))
}

/// `ACos(x)`: the arc cosine of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a number
/// that is outside of the domain is the empty text.
pub(crate) fn acos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_or_zero(rt, values, 0);
    if !(-1.0..=1.0).contains(&number) {
        return Ok(Value::Text(String::new()));
    }
    Ok(Value::Float(crate::runtime::acos(number)))
}

/// `ATan(x)`: the arc tangent of the number.
pub(crate) fn atan(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::atan(the_number_or_zero(
        rt, values, 0,
    ))))
}

/// `Max(a, b, ...)`: the biggest of the numbers.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a text
/// that is not a number is the empty text, and the answer keeps the
/// kind of the number.
pub(crate) fn max(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_biggest_or_the_smallest(rt, values, true)
}

/// `Min(a, b, ...)`: the smallest of the numbers.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a text
/// that is not a number is the empty text, and the answer keeps the
/// kind of the number.
pub(crate) fn min(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_biggest_or_the_smallest(rt, values, false)
}

/// The biggest of the numbers, or the smallest of them.
///
/// The answer keeps the kind of the number that won, and a text that is not
/// a number is the empty text of the answer.
fn the_biggest_or_the_smallest(
    rt: &mut Runtime<'_>,
    values: &[Value],
    the_biggest: bool,
) -> Result<Value, RuntimeError> {
    let mut answer: Option<Value> = None;
    for at in 0..values.len() {
        let Some(number) = the_whole_number(rt, values, at) else {
            return Ok(Value::Text(String::new()));
        };
        let number_value = number.to_number().unwrap_or(0.0);
        answer = Some(match answer {
            None => number,
            Some(previous) => {
                let previous_value = previous.to_number().unwrap_or(0.0);
                if (the_biggest && number_value > previous_value)
                    || (!the_biggest && number_value < previous_value)
                {
                    number
                } else {
                    previous
                }
            }
        });
    }
    Ok(answer.unwrap_or(Value::Text(String::new())))
}
