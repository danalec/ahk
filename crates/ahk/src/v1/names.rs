//! The names of the v1 that the tree answers now.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! answer of each one of them, and the way the argument of the ones that
//! ask for a name arrives.
//!
//! The handler of a name of the v1 that the v2 shares is the handler of the
//! v2, and the handler of the names that are here is one of their own.

use alloc::format;
use alloc::string::{String, ToString};

use crate::runtime::{Runtime, RuntimeError, Value};

/// The place of the argument of this function that asks for the name of a
/// variable.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IsSet` asks
/// for the name of the variable and not for its value -- `IsSet(y)` asks
/// whether the variable `y` has a value, and not whether the value of `y`
/// is the name of a variable -- and the `IsByRef` asks for the name as
/// well. The `IsFunc` asks for the value.
pub(crate) fn the_place_of_the_argument_that_wants_the_name(name: &str) -> Option<usize> {
    match name.to_lowercase().as_str() {
        "isset" | "isbyref" => Some(0),
        _ => None,
    }
}

/// The place of the argument of this function that asks for the variable of
/// the caller and not for its value.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the
/// `VarSetCapacity` takes the variable itself and not its value --
/// `VarSetCapacity(y, 100)` empties `y`, and a target that is not a
/// variable is answered with zero -- and the variable is the first
/// argument. The `StrReplace` takes the variable that receives the number
/// of the replacements, and that variable is the fourth argument.
pub(crate) fn the_place_of_the_argument_that_wants_the_variable(name: &str) -> Option<usize> {
    match name.to_lowercase().as_str() {
        "varsetcapacity" => Some(0),
        "strreplace" => Some(3),
        _ => None,
    }
}

/// `SetFormat, Integer, H`: the format of the numbers of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the number type is
/// "Integer", "IntegerFast", "Float" or "FloatFast", the format of the
/// whole numbers is "D", "H" or "h", and the format of the floats is
/// "<width>.<decimals>" with an optional "e" for the scientific notation.
/// The value that is not a format is refused with "Parameter #2 invalid."
/// and the value in the `Extra`.
pub(crate) fn set_format(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = |index: usize| {
        values
            .get(index)
            .map(|value| value.to_text())
            .unwrap_or_default()
            .trim()
            .to_string()
    };
    let number_type = text(0);
    let format = text(1);
    match super::parameters::the_kind_of_a_number(&number_type) {
        Some(super::parameters::TheKindOfANumber::Integer) => {
            let format = match super::parameters::the_format_of_the_whole_numbers(&format) {
                Some(format) => format,
                None => {
                    return Err(RuntimeError {
                        message: format!("Parameter #2 invalid.\n     Specifically: {format}"),
                        line,
                    })
                }
            };
            rt.set_integer_format(format);
        }
        Some(super::parameters::TheKindOfANumber::Float) => {
            let format = match super::parameters::the_format_of_the_floats(&format) {
                Some(format) => format,
                None => {
                    return Err(RuntimeError {
                        message: format!("Parameter #2 invalid.\n     Specifically: {format}"),
                        line,
                    })
                }
            };
            rt.set_float_format(format);
        }
        None => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {number_type}"),
                line,
            })
        }
    }
    Ok(Value::Text(String::new()))
}

/// `AutoTrim, On`: the trim of the text of a legacy assignment.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command turns
/// the trim on and off, and the value that it does not know is refused with
/// "Parameter #1 invalid." and the value in the `Extra`.
pub(crate) fn auto_trim(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = the_one_parameter_of_a_command(values);
    let trimmed = text.trim();
    match super::parameters::the_switch_of_the_v1(trimmed) {
        Some(trim) => rt.set_auto_trim(trim),
        None => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {trimmed}"),
                line,
            })
        }
    }
    Ok(Value::Text(String::new()))
}

/// `SetBatchLines, 20`: the number of the lines that the script runs at once.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter is a
/// whole number, and the value that is not one is refused with "Parameter #1
/// invalid." and the value in the message. The hex is a whole number as well
/// -- the "0x10" is sixteen -- and the number that is written is the one that
/// the name of the variable says: the "007" is seven. The zero is the default
/// of the dialect, and it is ten.
pub(crate) fn set_batch_lines(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = the_one_parameter_of_a_command(values);
    let trimmed = text.trim();
    let number = match trimmed.parse::<i64>() {
        Ok(number) => number,
        // The hex of the v1 is a whole number as well.
        Err(_) if super::numbers::the_text_is_a_hex(trimmed) => {
            super::numbers::the_whole_number_of_a_hex(trimmed)
        }
        Err(_) => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {trimmed}"),
                line,
            })
        }
    };
    // The zero is the default of the dialect, and it is ten.
    let number = if number == 0 { 10 } else { number };
    rt.set_batch_lines(&number.to_string());
    Ok(Value::Text(String::new()))
}

/// The text of the parameter of a command of the v1 that takes one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command takes the
/// parameter that it wants, and the text that is left of the line goes into it,
/// with a comma and a space between: the "AutoTrim, On, extra" is the
/// "On, extra", and it is refused because it is not "On".
pub(crate) fn the_one_parameter_of_a_command(values: &[Value]) -> String {
    let mut text = String::new();
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            text.push_str(", ");
        }
        text.push_str(&value.to_text());
    }
    text
}

/// `StringCaseSense, On`: the sense of the case of the comparisons.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command takes
/// "On", "Off" or "Locale", and the numbers one and zero are the two
/// first. The value that it does not know is refused with "Parameter #1
/// invalid." and the value in the `Extra`.
pub(crate) fn string_case_sense(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = the_one_parameter_of_a_command(values);
    let trimmed = text.trim();
    let sense = match super::parameters::the_case_sense_of_the_v1(trimmed) {
        Some(sense) => sense,
        None => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {trimmed}"),
                line,
            })
        }
    };
    rt.set_case_sense(sense);
    Ok(Value::Text(String::new()))
}

/// `CoordMode, Mouse, Screen`: the way the coordinates are read.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// check the target -- the "abc" changes nothing -- and the mode that it
/// does not know sets every mode to the "Event". The v2 refuses the target
/// and the mode that it does not know.
pub(crate) fn coord_mode_of_the_v1(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values.first().map(|value| value.to_text()).unwrap_or_default();
    let relative = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    crate::settings::set_the_coord_mode_of_the_v1(&target, &relative);
    Ok(Value::Text(String::new()))
}

/// `SetControlDelay, 20`: the pause after a change of a control.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// check the parameter -- the "abc" is zero and the "20abc" is zero -- and
/// the number that does not fit in the machine wraps around it. The v2
/// refuses a parameter that is not a number and a delay that is out of the
/// range of the machine.
pub(crate) fn set_control_delay_of_the_v1(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values.first().map(|value| value.to_text()).unwrap_or_default();
    let delay = crate::v1::the_number_of_a_text(&text)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i32;
    let previous = crate::settings::set_the_control_delay_of_the_v1(delay);
    Ok(Value::Int(previous as i64))
}

/// `SetWinDelay, 20`: the pause after a windowing function.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// check the parameter -- the "abc" is zero -- and the v2 refuses a
/// parameter that is not a number.
pub(crate) fn set_win_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values.first().map(|value| value.to_text()).unwrap_or_default();
    let delay = crate::v1::the_number_of_a_text(&text)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    crate::settings::set_win_delay(rt, &[Value::Float(delay)], line, 0)
}

/// `SetMouseDelay, 20`: the pause after a movement of the mouse.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// check the mode -- the "abc" is not the "Play" -- and the v2 refuses it.
pub(crate) fn set_mouse_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = values.first().and_then(|value| value.to_number()).unwrap_or(0.0);
    let mode = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let mode = if mode.trim().eq_ignore_ascii_case("play") {
        "Play"
    } else {
        ""
    };
    crate::settings::set_mouse_delay(
        rt,
        &[Value::Float(delay), Value::Text(mode.to_string())],
        line,
        0,
    )
}

/// `SetKeyDelay, 20`: the pause between the keys.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// check the mode -- the "abc" is not the "Play" -- and the v2 refuses it.
pub(crate) fn set_key_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = values.first().and_then(|value| value.to_number()).unwrap_or(0.0);
    let mode = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let mode = if mode.trim().eq_ignore_ascii_case("play") {
        "Play"
    } else {
        ""
    };
    crate::settings::set_key_delay(
        rt,
        &[Value::Float(delay), Value::Text(mode.to_string())],
        line,
        0,
    )
}

/// `FileEncoding, UTF-8`: the encoding the files are opened with.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the text that is
/// empty changes nothing, and the v2 refuses it.
pub(crate) fn file_encoding_of_the_v1(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Value::Text(crate::settings::file_encoding().to_string()));
    }
    crate::settings::set_file_encoding(&name)
        .map(|encoding| Value::Text(encoding.to_string()))
        .map_err(|message| RuntimeError { message, line })
}

/// The text of a value of a command, without the quotes that surround it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the numbers of the
/// `Transform` are the numbers of the text without its quotes: `Ceil, "1.2"`
/// is 2, and `Abs, "-5"` is "5". The `Asc` and the `Deref` see the text as
/// it is.
fn the_text_without_the_quotes(text: &str) -> &str {    let text = text.trim();
    if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        return &text[1..text.len() - 1];
    }
    text
}

/// Whether the command of this name has a body.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IfInString`,
/// the `IfNotInString` and the `IfMsgBox` have an action: the command that
/// is on the same line, the block, or the statement of the next line.
pub(crate) fn the_command_has_a_body(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "ifinstring" | "ifnotinstring" | "ifmsgbox"
    )
}

/// Whether the command of this name is the `else` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` of the v1
/// is a command of its own, and it takes no parameter.
pub(crate) fn the_command_is_the_else(name: &str) -> bool {
    name.to_lowercase() == "else"
}

/// The condition of the `IfInString` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command holds
/// when the text of the needle is in the variable, and the sense of the
/// case says whether the letters of them are the same. The needle that is
/// not there is a failure.
pub(crate) fn if_in_string(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_search(rt, values, line, "IfInString")
}

/// The condition of the `IfNotInString` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command is the
/// other side of the `IfInString`.
pub(crate) fn if_not_in_string(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_search(rt, values, line, "IfNotInString")
}

/// The condition of a search of the v1 that has a body.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command holds
/// when the text of the needle is in the variable, and the sense of the
/// case says whether the letters of them are the same.
///
/// The needle that is empty is a refusal of the load, and not of the run:
/// the number of the parameters is checked there, and the needle that is
/// written as nothing does not count. The needle that becomes empty at the
/// run -- the substitution of a name that has no value -- is a text that is
/// in every text, and the command holds.
fn the_condition_of_the_search(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    name: &str,
) -> Result<Value, RuntimeError> {
    let haystack = values.first().map(|value| value.to_text()).unwrap_or_default();
    let needle = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let holds = crate::runtime::the_text_contains(&haystack, &needle, rt.case_sense().the_method());
    Ok(Value::Int(if name == "IfNotInString" { !holds } else { holds } as i64))
}

/// The condition of the `IfMsgBox` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command holds
/// when the person pressed the button that is named. The message box of
/// this runtime has no buttons, and the button that was pressed is not
/// there: the condition is false.
pub(crate) fn if_msg_box(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(0))
}

/// The `else` of the v1 that has no `if`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` without
/// an `if` is refused at the load with "ELSE with no matching IF".
pub(crate) fn else_of_the_v1(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "ELSE with no matching IF".into(),
        line,
    })
}

/// `Transform, out, SubCommand, Value1, Value2`: the math of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command takes
/// the name of the variable of the answer, the sub-command and the two
/// values, and the answer goes into the variable. The sub-command that it
/// does not know is refused with "Parameter #2 invalid.".
pub(crate) fn transform(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let sub = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let first = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let second = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let answer = the_answer_of_the_transform(rt, &sub, &first, &second, line)?;
    crate::v1::write_the_answer(rt, "Transform", values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// The answer of one sub-command of the `Transform`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: every sub-command
/// has its own answer, and some of them have their own failure.
fn the_answer_of_the_transform(
    rt: &mut Runtime<'_>,
    sub: &str,
    first: &str,
    second: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    // The number of a value of the command: a text that is not a number is
    // zero, and the v1 does not fail.
    let number = |text: &str| {
        crate::v1::the_number_of_a_text(text)
            .and_then(|value| value.to_number())
            .unwrap_or(0.0)
    };
    // The value of the v1 as a number: a whole one or a float.
    let value = |text: &str| crate::v1::the_number_of_a_text(text);
    // The numbers of the command are the numbers of the text without the
    // quotes that surround it.
    fn plain(text: &str) -> &str {
        the_text_without_the_quotes(text)
    }
    let invalid = |message: String| Err(RuntimeError { message, line });
    match sub.trim().to_lowercase().as_str() {
        // The code of the first letter of the text, and an empty text is an
        // empty answer. The text of this one is the text as it is: the
        // quotes are a part of it.
        "asc" => Ok(match first.chars().next() {
            Some(character) => Value::Int(character as i64),
            None => Value::Text(String::new()),
        }),
        // The letter of a code: the codes of the v1 are the ones of the
        // ANSI, and the code zero is an empty letter.
        "chr" => {
            let code = number(plain(first));
            if !(0.0..=255.0).contains(&code) {
                return invalid(format!(
                    "Parameter #3 invalid.\n     Specifically: {}",
                    plain(first)
                ));
            }
            let code = code as u32;
            Ok(match char::from_u32(code) {
                Some(character) if code != 0 => Value::Text(character.to_string()),
                _ => Value::Text(String::new()),
            })
        }
        // The remainder of a division, and the divisor of zero is a failure.
        "mod" => {
            let divisor = number(plain(second));
            if divisor == 0.0 {
                return invalid(format!("Divide by zero\n     Specifically: {}", plain(second)));
            }
            let left = number(plain(first));
            Ok(Value::Int((left % divisor) as i64))
        }
        "exp" => Ok(Value::Float(crate::runtime::exp(number(plain(first))))),
        // The square root, and the number of the root that is not one.
        "sqrt" => {
            let root = number(plain(first));
            if root < 0.0 {
                return invalid(format!(
                    "Parameter #3 must be a positive integer in this case.\n     Specifically: {}",
                    plain(first)
                ));
            }
            Ok(Value::Float(crate::runtime::sqrt(root)))
        }
        // The logarithm, and the number of it that is not one.
        "log" | "ln" => {
            let logarithm = number(plain(first));
            if logarithm < 0.0 {
                return invalid(format!(
                    "Parameter #3 must be a positive integer in this case.\n     Specifically: {}",
                    plain(first)
                ));
            }
            let answer = if sub.trim().eq_ignore_ascii_case("log") {
                crate::runtime::log10(logarithm)
            } else {
                crate::runtime::ln(logarithm)
            };
            Ok(Value::Float(answer))
        }
        // The number rounded to the digits of the second value: without
        // them the answer is a whole number.
        "round" => {
            let digits = number(plain(second));
            let rounded = number(plain(first));
            let factor = crate::runtime::powf(10.0, digits);
            let answer = crate::runtime::round(rounded * factor) / factor;
            if digits <= 0.0 {
                return Ok(Value::Int(answer as i64));
            }
            Ok(Value::Float(answer))
        }
        "ceil" => Ok(Value::Int(crate::runtime::ceil(number(plain(first))) as i64)),
        "floor" => Ok(Value::Int(crate::runtime::floor(number(plain(first))) as i64)),
        // The absolute value of the v1 is the text of the number without the
        // sign of the minus: "5.50" stays "5.50". A text that does not
        // begin with a number is the text as it is.
        "abs" => {
            let text = plain(first);
            if text.is_empty() {
                return invalid("Parameter #3 must be a number in this case.".to_string());
            }
            let text = text.trim_start();
            match text.strip_prefix('-') {
                Some(rest) => {
                    if value(rest).is_none() {
                        return Ok(Value::Text(String::new()));
                    }
                    Ok(Value::Text(rest.to_string()))
                }
                None => Ok(Value::Text(text.to_string())),
            }
        }
        "sin" => Ok(Value::Float(crate::runtime::sin(number(plain(first))))),
        "cos" => Ok(Value::Float(crate::runtime::cos(number(plain(first))))),
        "tan" => Ok(Value::Float(crate::runtime::tan(number(plain(first))))),
        // The arc of the trigonometry: a number that is out of the arc is
        // not a number, and the v1 answers nothing.
        "asin" | "acos" | "atan" => {
            let arc = number(plain(first));
            let answer = match sub.trim().to_lowercase().as_str() {
                "asin" => crate::runtime::asin(arc),
                "acos" => crate::runtime::acos(arc),
                _ => crate::runtime::atan(arc),
            };
            if answer.is_nan() {
                return Ok(Value::Text(String::new()));
            }
            Ok(Value::Float(answer))
        }
        // The power, and the base that is negative is a failure.
        "pow" => {
            let base = number(plain(first));
            if base < 0.0 {
                return invalid(format!(
                    "Parameter #3 must be a positive integer in this case.\n     Specifically: {}",
                    plain(first)
                ));
            }
            // The v1 answers with a whole number when the two are whole.
            if let (Some(Value::Int(left)), Some(Value::Int(right))) =
                (value(plain(first)), value(plain(second)))
            {
                if right >= 0 {
                    if let Some(answer) = left.checked_pow(right as u32) {
                        return Ok(Value::Int(answer));
                    }
                }
            }
            Ok(Value::Float(crate::runtime::powf(base, number(plain(second)))))
        }
        // The bits of the v1: the `BitNot` is of thirty-two bits and the
        // rest is of sixty-four.
        "bitnot" => {
            let bits = number(plain(first)) as u32;
            Ok(Value::Int((!bits) as i64))
        }
        "bitand" => Ok(Value::Int(number(plain(first)) as i64 & number(plain(second)) as i64)),
        "bitor" => Ok(Value::Int(number(plain(first)) as i64 | number(plain(second)) as i64)),
        "bitxor" => Ok(Value::Int(number(plain(first)) as i64 ^ number(plain(second)) as i64)),
        "bitshiftleft" => Ok(Value::Int(
            (number(plain(first)) as i64) << (number(plain(second)) as i64 & 63),
        )),
        "bitshiftright" => Ok(Value::Int(
            (number(plain(first)) as i64) >> (number(plain(second)) as i64 & 63),
        )),
        // The references that are inside the text are expanded.
        "deref" => {
            let text = rt
                .the_substitution_of_the_v1(first, line)
                .map_err(|failure| RuntimeError {
                    message: crate::runtime::the_message_of(&failure),
                    line,
                })?;
            Ok(Value::Text(text))
        }
        // The HTML of the v1 is a table of the names of the characters that
        // are not of the English, and the tree does not carry it.
        "html" => invalid(
            "this runtime does not carry the sub-command of the v1.\n     Specifically: HTML".to_string(),
        ),
        // The sub-command that the v1 does not know, and the one of the
        // clipboard that a Unicode build does not have.
        _ => invalid(format!("Parameter #2 invalid.\n     Specifically: {sub}")),
    }
}

/// `VarSetCapacity(TargetVar, RequestedCapacity, FillByte)`: the capacity
/// of a variable.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the function takes
/// the variable of the caller and empties it, and it answers the number of
/// bytes that were asked for. A target that is not a variable is answered
/// with zero.
pub(crate) fn var_set_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The target is the variable of the caller, and a text that is not a
    // variable is answered with zero.
    let Some(Value::Reference(_)) = values.first() else {
        return Ok(Value::Int(0));
    };
    let capacity = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    if capacity.trim().is_empty() {
        // The capacity that the variable has is not the one of the tree:
        // the tree keeps no capacity, and it says so.
        return Err(RuntimeError {
            message: "this runtime does not keep the capacity of a variable.".into(),
            line,
        });
    }
    let capacity = crate::v1::the_number_of_a_text(&capacity)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i64;
    // The variable is emptied. The byte of the fill is not written: the
    // tree has no memory of its own to fill.
    crate::v1::write_the_answer(
        rt,
        "VarSetCapacity",
        values,
        0,
        Value::Text(String::new()),
        line,
    )?;
    Ok(Value::Int(capacity))
}

/// `IsSet(name)`: whether the variable of that name has a value.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer is 1 for
/// a variable that has a value and 0 for one that has none, and a built-in
/// variable of the v1 has a value.
pub(crate) fn is_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    let answer = if rt.has_a_value(&name) { 1 } else { 0 };
    Ok(Value::Int(answer))
}

/// `IsFunc(name)`: whether the name is a function.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer is 1 for
/// a function of the script, 2 for a built-in and 0 for a name that is
/// neither.
///
/// The `IsFunc` of the v1 takes a value and not a name: `IsFunc("StrLen")`
/// asks about the function of that name, and `IsFunc(StrLen)` asks about the
/// value of the name -- which is a blank text, because the name of a
/// function is not a value -- and it is zero. A command of the v1 is not a
/// function: `IsFunc("MsgBox")` is zero.
///
/// A reference to a function is a function: `IsFunc(Func("StrLen"))` is 2
/// and `IsFunc(Func("f"))` is 1.
pub(crate) fn is_func(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = values.first();
    if let Some(Value::Object(object)) = value {
        let object = object.borrow();
        if object.class == "func" {
            let builtin = matches!(object.value_of("isbuiltin"), Some(Value::Int(1)));
            return Ok(Value::Int(if builtin { 2 } else { 1 }));
        }
    }
    let name = value.map(|value| value.to_text()).unwrap_or_default();
    let answer = if rt.has_a_function(&name) {
        1
    } else if rt.has_a_builtin_function(&name) {
        2
    } else {
        0
    };
    Ok(Value::Int(answer))
}

/// `IsByRef(name)`: whether the variable of that name is a reference.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer is 1 for
/// a variable that is a reference and 0 for one that is not.
pub(crate) fn is_by_ref(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    let answer = if rt.is_a_reference(&name) { 1 } else { 0 };
    Ok(Value::Int(answer))
}

/// `Object()`: an object of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the object of the
/// v1 has no class, and its `__Class` is empty, and a property that is
/// written on it is a property of it.
pub(crate) fn object(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(crate::classes::object(""))
}

/// `Func(name)`: the reference to the function of that name.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer is a
/// value that carries the name of the function and the number of its
/// parameters, and the call of it is the call of the function. The name of a
/// built-in function is a function too, and the name of a command is not a
/// function at all -- `Func("MsgBox")` is zero, and so is the name of
/// nothing.
pub(crate) fn func(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    if let Some(value) = rt.the_function_of_the_script(&name) {
        return Ok(value);
    }
    if let Some(value) = rt.the_builtin_function(&name) {
        return Ok(value);
    }
    Ok(Value::Int(0))
}

/// `FileReadLine, name, file, n`: the n-th line of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command wants
/// three parameters, and the answer is the line itself, without the line
/// ending, and it is empty when the file has no such line.
pub(crate) fn file_read_line(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let number = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let number: usize = number.trim().parse().unwrap_or(0);
    let lines = rt
        .desktop()
        .read_file(&path)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    let answer = match number.checked_sub(1).and_then(|at| lines.get(at)) {
        Some(text) => Value::Text(text.clone()),
        None => Value::Text(String::new()),
    };
    crate::v1::write_the_answer(rt, "FileReadLine", values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// `Input, name, options, title, default`: what the user writes.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command asks
/// the user with a dialogue, and the text of the user is the answer.
pub(crate) fn input(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let options = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let title = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let default = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    // The title of the v1 is the prompt of the dialogue, and the options
    // are the ones that the v2 reads.
    let options = crate::dialogs::the_options_of_the_input_box(&options, line)?;
    let answer = rt
        .desktop()
        .input_box(&title, &title, &options, &default)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    let answer = Value::Text(answer.value);
    crate::v1::write_the_answer(rt, "Input", values, 0, answer.clone(), line)?;
    Ok(answer)
}
