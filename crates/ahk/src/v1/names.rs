//! The names of the v1 that the tree answers now.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! answer of each one of them, and the way the argument of the ones that
//! ask for a name arrives.
//!
//! The handler of a name of the v1 that the v2 shares is the handler of the
//! v2, and the handler of the names that are here is one of their own.

use alloc::format;
use alloc::string::String;

use crate::runtime::{FloatFormat, IntegerFormat, Runtime, RuntimeError, Value};

/// Whether the function of this name asks for the name of a variable.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IsSet` asks
/// for the name of the variable and not for its value -- `IsSet(y)` asks
/// whether the variable `y` has a value, and not whether the value of `y`
/// is the name of a variable -- and the `IsByRef` asks for the name as
/// well. The `IsFunc` asks for the value.
pub(crate) fn wants_the_name_of_the_argument(name: &str) -> bool {
    matches!(name.to_lowercase().as_str(), "isset" | "isbyref")
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
    match number_type.to_lowercase().as_str() {
        "integer" | "integerfast" => {
            let format = match format.as_str() {
                "D" | "d" => IntegerFormat::Decimal,
                "H" | "HEX" | "hex" => IntegerFormat::Hexadecimal,
                "h" => IntegerFormat::HexadecimalLower,
                _ => {
                    return Err(RuntimeError {
                        message: format!("Parameter #2 invalid.\n     Specifically: {format}"),
                        line,
                    })
                }
            };
            rt.set_integer_format(format);
        }
        "float" | "floatfast" => {
            let format = match the_float_format_of_the_v1(&format) {
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
        _ => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {number_type}"),
                line,
            })
        }
    }
    Ok(Value::Text(String::new()))
}

/// The format of the floats of the v1, when the text is one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the format is
/// "<width>.<decimals>", and the width may be in zero ("06.2") or in minus
/// ("-6.2") for the padding, and the "e" at the end asks for the scientific
/// notation.
fn the_float_format_of_the_v1(text: &str) -> Option<FloatFormat> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let (body, scientific) = match text.strip_suffix('e') {
        Some(body) => (body, true),
        None => match text.strip_suffix('E') {
            Some(body) => (body, true),
            None => (text, false),
        },
    };
    let (width, decimals) = body.split_once('.')?;
    let (left, width) = match width.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, width),
    };
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the width "0" is
    // the width zero, and the zeroes that pad the text are a zero in front
    // of a width that has more digits ("06.2").
    let (zeroes, width) = match width.strip_prefix('0') {
        Some(rest) if !rest.is_empty() => (true, rest),
        _ => (false, width),
    };
    let width: u32 = width.parse().ok()?;
    let decimals: u32 = decimals.parse().ok()?;
    Some(FloatFormat {
        width,
        zeroes,
        left,
        decimals,
        scientific,
        upper: text.ends_with('E'),
    })
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
    let text = values.first().map(|value| value.to_text()).unwrap_or_default();
    let trimmed = text.trim();
    match trimmed.to_lowercase().as_str() {
        "on" => rt.set_auto_trim(true),
        "off" => rt.set_auto_trim(false),
        _ => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {trimmed}"),
                line,
            })
        }
    }
    Ok(Value::Text(String::new()))
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
pub(crate) fn is_func(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    let answer = if rt.has_a_function(&name) {
        1
    } else if rt.has_a_builtin(&name) {
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

/// `Func(name)`: the function of the script of that name.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer is a
/// value that carries the name of the function and the number of its
/// parameters, and the call of it is the call of the function.
pub(crate) fn func(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    match rt.the_function_of_the_script(&name) {
        Some(value) => Ok(value),
        None => Err(RuntimeError {
            message: format!("there is no function of the script named {name}."),
            line,
        }),
    }
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
