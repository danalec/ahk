//! The names of the v1 that the tree answers now.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02: the
//! answer of each one of them, and the way the argument of the ones that
//! ask for a name arrives.
//!
//! The handler of a name of the v1 that the v2 shares is the handler of the
//! v2, and the handler of the names that are here is one of their own.

use alloc::format;
use alloc::string::{String, ToString};

use crate::ast::BinaryOp;
use crate::runtime::{Runtime, RuntimeError, Value};

/// The letters that the v1 does not accept in the name of a variable.
///
/// Measured on the binary of AutoHotkey v1.1.37.02
/// (the probe of the measurement): the name that the script builds may not have a space,
/// a tab, or one of the characters that are special in an expression. The
/// `#`, the `$` and the `@` are legal in the name of a variable, even
/// though they are special somewhere else.
pub(crate) fn the_illegal_letters_of_a_name() -> &'static [char] {
    &[
        ' ', '\t', '!', '"', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '[', '\\', ']', '^', '`', '{', '|', '}', '~',
    ]
}

/// The place of the argument of this function that asks for the name of a
/// variable.
///
/// Measured on the binary of v1.1.37.02: the `IsSet` asks
/// for the name of the variable and not for its value -- `IsSet(y)` asks
/// whether the variable `y` has a value, and not whether the value of `y`
/// is the name of a variable -- and the `IsByRef` asks for the name as
/// well. The `IsFunc` asks for the value.
pub(crate) fn the_place_of_the_argument_that_wants_the_name(name: &str) -> Option<usize> {
    // The name is read without a copy of it: the question is asked for
    // every call of a built-in, and the copy of the name of it is a block
    // of the heap in every one of those calls.
    if name.eq_ignore_ascii_case("isset") || name.eq_ignore_ascii_case("isbyref") {
        Some(0)
    } else {
        None
    }
}

/// The place of the argument of this function that asks for the variable of
/// the caller and not for its value.
///
/// Measured on the binary of v1.1.37.02: the
/// `VarSetCapacity` takes the variable itself and not its value --
/// `VarSetCapacity(y, 100)` empties `y`, and a target that is not a
/// variable is answered with zero -- and the variable is the first
/// argument. The `StrReplace` takes the variable that receives the number
/// of the replacements, and that variable is the fourth argument.
/// Whether every argument of this function that is a naked variable keeps
/// the variable of the caller.
///
/// Measured on the binary of v1.1.37.02: the `DllCall` of
/// the v1 writes the answer of the machine into the variable that the
/// parameter names. The source of the v1 says it at the line 13619 of
/// `script2.cpp`: "Output parameters are copied back only if its
/// counterpart parameter is a naked variable."
pub(crate) fn every_argument_that_is_a_variable_keeps_it(name: &str) -> bool {
    name.eq_ignore_ascii_case("DllCall")
}

/// The place of the argument of this function that asks for the variable of
/// the caller and not for its value.
pub(crate) fn the_place_of_the_argument_that_wants_the_variable(name: &str) -> Option<usize> {
    // The name is read without a copy of it: the question is asked for
    // every call of a built-in, and the copy of the name of it is a block
    // of the heap in every one of those calls.
    if name.eq_ignore_ascii_case("varsetcapacity") {
        Some(0)
    } else if name.eq_ignore_ascii_case("strreplace") {
        Some(3)
    } else if name.eq_ignore_ascii_case("numput") {
        // The target of a `NumPut`/`NumGet` is a variable or an address:
        // a bare variable goes as the reference of its memory, and an
        // expression goes as the number that is the address.
        Some(1)
    } else if name.eq_ignore_ascii_case("numget") {
        Some(0)
    } else if name.eq_ignore_ascii_case("loadpicture") {
        // Read in the CHM of the v1.1.37.02 (`LoadPicture.htm`): the third
        // parameter is `OutImageType`, the name of the variable that the
        // command writes into.
        Some(2)
    } else if name.eq_ignore_ascii_case("regexmatch") {
        // `RegExMatch(Haystack, Needle, OutputVar, StartingPos)` and
        // `RegExReplace(Haystack, Needle, Repl, OutputVarCount, Limit, Start)`.
        Some(2)
    } else if name.eq_ignore_ascii_case("regexreplace") {
        Some(3)
    } else if name.eq_ignore_ascii_case("lv_gettext") || name.eq_ignore_ascii_case("tv_gettext") {
        // `LV_GetText(OutputVar, RowNumber, ColumnNumber)` and
        // `TV_GetText(OutputVar, ItemID)`.
        Some(0)
    } else {
        None
    }
}

/// `SetFormat, Integer, H`: the format of the numbers of the v1.
///
/// Measured on the binary of v1.1.37.02: the number type is
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
                    });
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
                    });
                }
            };
            rt.set_float_format(format);
        }
        None => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {number_type}"),
                line,
            });
        }
    }
    Ok(Value::Text("".into()))
}

/// `AutoTrim, On`: the trim of the text of a legacy assignment.
///
/// Measured on the binary of v1.1.37.02: the command turns
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
            });
        }
    }
    Ok(Value::Text("".into()))
}

/// `SetBatchLines, 20`: the number of the lines that the script runs at once.
///
/// Measured on the binary of v1.1.37.02: the parameter is a
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
            });
        }
    };
    // The zero is the default of the dialect, and it is ten.
    let number = if number == 0 { 10 } else { number };
    rt.set_batch_lines(&number.to_string());
    Ok(Value::Text("".into()))
}

/// The text of the parameter of a command of the v1 that takes one.
///
/// Measured on the binary of v1.1.37.02: the command takes the
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
/// Measured on the binary of v1.1.37.02: the command takes
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
            });
        }
    };
    rt.set_case_sense(sense);
    Ok(Value::Text("".into()))
}

/// `DetectHiddenText, On`: the text of the hidden windows is read.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1
/// takes the word "On" and the word "Off" -- and the numbers one and zero --
/// and the v2 takes a value that is true when it is not empty.
pub(crate) fn set_detect_hidden_text_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = super::parameters::the_switch_of_the_v1(
        &values
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default(),
    )
    .unwrap_or(false);
    crate::settings::set_detect_hidden_text(rt, &[Value::Int(on as i64)], line, 0)
}

/// `DetectHiddenWindows, On`: the windows that are hidden are found.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1
/// takes the word "On" and the word "Off" -- and the numbers one and zero --
/// and the v2 takes a value that is true when it is not empty.
pub(crate) fn set_detect_hidden_windows_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = super::parameters::the_switch_of_the_v1(
        &values
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default(),
    )
    .unwrap_or(false);
    crate::settings::set_detect_hidden_windows(rt, &[Value::Int(on as i64)], line, 0)
}

/// `SetStoreCapsLockMode, On`: whether the state of the caps lock is put back.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1
/// takes the word "On" and the word "Off" -- and the numbers one and zero --
/// and the v2 takes a value that is true when it is not empty.
pub(crate) fn set_store_caps_lock_mode_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = super::parameters::the_switch_of_the_v1(
        &values
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default(),
    )
    .unwrap_or(false);
    crate::settings::set_store_caps_lock_mode(rt, &[Value::Int(on as i64)], line, 0)
}

/// `CoordMode, Mouse, Screen`: the way the coordinates are read.
///
/// Measured on the binary of v1.1.37.02: the v1 does not
/// check the target -- the "abc" changes nothing -- and the mode that it
/// does not know sets every mode to the "Event". The v2 refuses the target
/// and the mode that it does not know.
pub(crate) fn coord_mode_of_the_v1(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let relative = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    crate::settings::set_the_coord_mode_of_the_v1(&target, &relative);
    Ok(Value::Text("".into()))
}

/// `SetControlDelay, 20`: the pause after a change of a control.
///
/// Measured on the binary of v1.1.37.02: the v1 does not
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
    let text = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let delay = crate::v1::the_number_of_a_text(&text)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i32;
    let previous = crate::settings::set_the_control_delay_of_the_v1(delay);
    Ok(Value::Int(previous as i64))
}

/// `SetWinDelay, 20`: the pause after a windowing function.
///
/// Measured on the binary of v1.1.37.02: the v1 does not
/// check the parameter -- the "abc" is zero -- and the v2 refuses a
/// parameter that is not a number.
pub(crate) fn set_win_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let delay = crate::v1::the_number_of_a_text(&text)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    crate::settings::set_win_delay(rt, &[Value::Float(delay)], line, 0)
}

/// `SetMouseDelay, 20`: the pause after a movement of the mouse.
///
/// Measured on the binary of v1.1.37.02: the v1 does not
/// check the mode -- the "abc" is not the "Play" -- and the v2 refuses it.
pub(crate) fn set_mouse_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = values
        .first()
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    let mode = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let mode = if mode.trim().eq_ignore_ascii_case("play") {
        "Play"
    } else {
        ""
    };
    crate::settings::set_mouse_delay(
        rt,
        &[Value::Float(delay), Value::Text(mode.to_string().into())],
        line,
        0,
    )
}

/// `SetKeyDelay, 20`: the pause between the keys.
///
/// Measured on the binary of v1.1.37.02: the v1 does not
/// check the mode -- the "abc" is not the "Play" -- and the v2 refuses it.
pub(crate) fn set_key_delay_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let delay = values
        .first()
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    let mode = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let mode = if mode.trim().eq_ignore_ascii_case("play") {
        "Play"
    } else {
        ""
    };
    crate::settings::set_key_delay(
        rt,
        &[Value::Float(delay), Value::Text(mode.to_string().into())],
        line,
        0,
    )
}

/// `FileEncoding, UTF-8`: the encoding the files are opened with.
///
/// Measured on the binary of v1.1.37.02: the text that is
/// empty changes nothing, and the v2 refuses it.
pub(crate) fn file_encoding_of_the_v1(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Value::Text((crate::settings::file_encoding().to_string()).into()));
    }
    crate::settings::set_file_encoding(&name)
        .map(|encoding| Value::Text(encoding.to_string().into()))
        .map_err(|message| RuntimeError { message, line })
}

/// The text of a value of a command, without the quotes that surround it.
///
/// Measured on the binary of v1.1.37.02: the numbers of the
/// `Transform` are the numbers of the text without its quotes: `Ceil, "1.2"`
/// is 2, and `Abs, "-5"` is "5". The `Asc` and the `Deref` see the text as
/// it is.
fn the_text_without_the_quotes(text: &str) -> &str {
    let text = text.trim();
    if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        return &text[1..text.len() - 1];
    }
    text
}

/// Whether the command of this name has a body.
///
/// Measured on the binary of v1.1.37.02: the `IfInString`,
/// the `IfNotInString` and the `IfMsgBox` have an action: the command that
/// is on the same line, the block, or the statement of the next line.
pub(crate) fn the_command_has_a_body(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "ifinstring"
            | "ifnotinstring"
            | "ifmsgbox"
            | "ifexist"
            | "ifnotexist"
            | "ifequal"
            | "ifnotequal"
            | "ifless"
            | "iflessorequal"
            | "ifgreater"
            | "ifgreaterorequal"
            | "ifwinactive"
            | "ifwinnotactive"
            | "ifwinexist"
            | "ifwinnotexist"
    )
}

/// Whether the command of this name is the `else` of the v1.
///
/// Measured on the binary of v1.1.37.02: the `else` of the v1
/// is a command of its own, and it takes no parameter.
pub(crate) fn the_command_is_the_else(name: &str) -> bool {
    name.to_lowercase() == "else"
}

/// The condition of the `IfInString` of the v1.
///
/// Measured on the binary of v1.1.37.02: the command holds
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
/// Measured on the binary of v1.1.37.02: the command is the
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
/// Measured on the binary of v1.1.37.02: the command holds
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
    let haystack = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let needle = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let holds = crate::runtime::the_text_contains(&haystack, &needle, rt.case_sense().the_method());
    Ok(Value::Int(if name == "IfNotInString" {
        !holds
    } else {
        holds
    } as i64))
}

/// The condition of the `IfMsgBox` of the v1.
///
/// Measured on the binary of v1.1.37.02: the command holds
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

/// `IfExist, FilePattern`: the file or the folder is there.
pub(crate) fn if_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_existence(rt, values, line, false)
}

/// `IfNotExist, FilePattern`: the other side of the `IfExist`.
pub(crate) fn if_not_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_existence(rt, values, line, true)
}

/// The condition of the existence of a file: the attributes of the pattern
/// are not empty when the file or the folder is there.
fn the_condition_of_existence(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    negated: bool,
) -> Result<Value, RuntimeError> {
    let pattern = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let attributes = rt
        .desktop()
        .attributes_of_the_pattern(&pattern, "")
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    let holds = !attributes.concat().is_empty();
    Ok(Value::Int((holds ^ negated) as i64))
}

/// `IfEqual, Var, Value`: the comparison of the legacy `if`.
pub(crate) fn if_equal(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::Equal)
}

/// `IfNotEqual, Var, Value`: the other side of the `IfEqual`.
pub(crate) fn if_not_equal(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::NotEqual)
}

/// `IfLess, Var, Value`: the order of the legacy `if`.
pub(crate) fn if_less(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::Less)
}

/// `IfLessOrEqual, Var, Value`: the order that holds the equal.
pub(crate) fn if_less_or_equal(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::LessEqual)
}

/// `IfGreater, Var, Value`: the order of the legacy `if`.
pub(crate) fn if_greater(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::Greater)
}

/// `IfGreaterOrEqual, Var, Value`: the order that holds the equal.
pub(crate) fn if_greater_or_equal(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_comparison(rt, values, line, BinaryOp::GreaterEqual)
}

/// The comparison of the commands of the legacy `if`: the numbers when
/// both sides are numbers, and the texts otherwise, with the sense of the
/// case of the script.
fn the_condition_of_the_comparison(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    op: BinaryOp,
) -> Result<Value, RuntimeError> {
    let left = values
        .first()
        .cloned()
        .unwrap_or(Value::Text("".into()));
    let right = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let left_number = match &left {
        Value::Int(number) => Some(Value::Int(*number)),
        Value::Float(number) => Some(Value::Float(*number)),
        Value::Text(text) => crate::v1::the_number_of_a_text(text),
        _ => None,
    };
    let right_number = crate::v1::the_number_of_a_text(&right);
    let answer = match op {
        BinaryOp::Equal | BinaryOp::NotEqual => {
            let method = rt.case_sense().the_method();
            let same = match (left_number, right_number) {
                (Some(one), Some(other)) => crate::runtime::the_same_number(&one, &other),
                _ => crate::runtime::the_same_text(&left.to_text(), &right, method),
            };
            if op == BinaryOp::Equal {
                same
            } else {
                !same
            }
        }
        _ => {
            let numbers = match (left_number, right_number) {
                (Some(one), Some(other)) => Some((
                    one.to_number().unwrap_or(0.0),
                    other.to_number().unwrap_or(0.0),
                )),
                _ => None,
            };
            rt.the_order_of_the_v1(numbers, &left.to_text(), &right, &op)
        }
    };
    Ok(Value::Int(answer as i64))
}

/// `IfWinExist, WinTitle, ...`: the window is there.
pub(crate) fn if_win_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_window(rt, values, line, false, false)
}

/// `IfWinNotExist, WinTitle, ...`: the window is not there.
pub(crate) fn if_win_not_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_window(rt, values, line, false, true)
}

/// `IfWinActive, WinTitle, ...`: the window is there and has the focus.
pub(crate) fn if_win_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_window(rt, values, line, true, false)
}

/// `IfWinNotActive, WinTitle, ...`: the window is not the one with the focus.
pub(crate) fn if_win_not_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_condition_of_the_window(rt, values, line, true, true)
}

/// The condition of a window of the legacy `if`.
fn the_condition_of_the_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    active: bool,
    negated: bool,
) -> Result<Value, RuntimeError> {
    let title = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let mode = crate::settings::title_match_mode();
    let holds = if active {
        match rt
            .desktop()
            .window_active(&title, mode)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })? {
            crate::desktop::WindowAnswer::Active => true,
            crate::desktop::WindowAnswer::NotActive | crate::desktop::WindowAnswer::Absent => false,
            crate::desktop::WindowAnswer::Unknown(reason) => {
                return Err(RuntimeError {
                    message: reason,
                    line,
                });
            }
        }
    } else {
        let windows = rt
            .desktop()
            .matching_windows(&title, mode)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
        if let Some(window) = windows.first() {
            rt.remember_the_last_window_of_the_v1(&window.id);
            true
        } else {
            false
        }
    };
    Ok(Value::Int((holds ^ negated) as i64))
}

/// The types of the `if x is Type` of the v1, and the check of each.
///
/// Measured on the binary of v1.1.37.02: the empty text is digit, xdigit,
/// alpha, upper, lower, alnum and space; the number trims the spaces and
/// takes the sign and the hex, but not the exponent; the timestamp is the
/// `YYYYMMDDHH24MISS` of the machine, from the year 1601 on, cut at
/// fourteen digits and read in pairs.
pub(crate) fn the_type_of_the_is_is_known(word: &str) -> bool {
    matches!(
        word.trim().to_ascii_lowercase().as_str(),
        "integer"
            | "float"
            | "number"
            | "digit"
            | "xdigit"
            | "alpha"
            | "upper"
            | "lower"
            | "alnum"
            | "space"
            | "time"
            | "date"
    )
}

/// Whether the text is of the type the `is` of the v1 names.
pub(crate) fn the_value_is_of_the_type(text: &str, word: &str) -> bool {
    match word.trim().to_ascii_lowercase().as_str() {
        "integer" => the_text_is_an_integer_of_the_is(text),
        "float" => the_text_is_a_float_of_the_is(text),
        "number" => {
            the_text_is_an_integer_of_the_is(text) || the_text_is_a_float_of_the_is(text)
        }
        "digit" => text.chars().all(|character| character.is_ascii_digit()),
        "xdigit" => {
            // Measured on the binary of v1.1.37.02: the `0x` prefix is a
            // part of the hex: the `0x10` is xdigit, and the `0x` alone is
            // too; the sign and the spaces are not.
            let rest = text
                .strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))
                .unwrap_or(text);
            rest.chars().all(|character| character.is_ascii_hexdigit())
        }
        "alpha" => text.chars().all(|character| character.is_alphabetic()),
        "upper" => text.chars().all(|character| character.is_uppercase()),
        "lower" => text.chars().all(|character| character.is_lowercase()),
        "alnum" => text.chars().all(|character| character.is_alphanumeric()),
        "space" => text.chars().all(|character| character.is_whitespace()),
        "time" | "date" => the_text_is_a_timestamp_of_the_is(text),
        _ => false,
    }
}

/// The integer of the `is`: the spaces around are not a part of it, the
/// sign is, the hex is, and the exponent is not.
fn the_text_is_an_integer_of_the_is(text: &str) -> bool {
    let trimmed = text.trim();
    let unsigned = trimmed
        .strip_prefix('+')
        .or_else(|| trimmed.strip_prefix('-'))
        .unwrap_or(trimmed);
    if let Some(hex) = unsigned
        .strip_prefix("0x")
        .or_else(|| unsigned.strip_prefix("0X"))
    {
        return !hex.is_empty() && hex.chars().all(|character| character.is_ascii_hexdigit());
    }
    !unsigned.is_empty() && unsigned.chars().all(|character| character.is_ascii_digit())
}

/// The float of the `is`: one point, digits around it, and no exponent.
fn the_text_is_a_float_of_the_is(text: &str) -> bool {
    let trimmed = text.trim();
    let unsigned = trimmed
        .strip_prefix('+')
        .or_else(|| trimmed.strip_prefix('-'))
        .unwrap_or(trimmed);
    if !unsigned.contains('.') {
        return false;
    }
    let mut points = 0;
    let mut digits = 0;
    for character in unsigned.chars() {
        if character == '.' {
            points += 1;
            if points > 1 {
                return false;
            }
        } else if character.is_ascii_digit() {
            digits += 1;
        } else {
            return false;
        }
    }
    digits > 0
}

/// The timestamp of the `is`: the pairs of the digits, from the year on,
/// and the year is the one of the machine, 1601 or later.
fn the_text_is_a_timestamp_of_the_is(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    let length = trimmed.len().min(14);
    if length < 4 || length % 2 != 0 {
        return false;
    }
    let part = |from: usize, to: usize| -> u32 {
        trimmed[from..to].parse().unwrap_or(0)
    };
    let year = part(0, 4);
    if year < 1601 {
        return false;
    }
    if length >= 6 {
        let month = part(4, 6);
        if !(1..=12).contains(&month) {
            return false;
        }
        if length >= 8 {
            let day = part(6, 8);
            if day < 1 || day > the_days_of_the_month(year, month) {
                return false;
            }
            if length >= 10 && part(8, 10) > 23 {
                return false;
            }
            if length >= 12 && part(10, 12) > 59 {
                return false;
            }
            if length >= 14 && part(12, 14) > 59 {
                return false;
            }
        }
    }
    true
}

/// The days of the month, and the leap year of the Gregorian calendar.
fn the_days_of_the_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// The `else` of the v1 that has no `if`.
///
/// Measured on the binary of v1.1.37.02: the `else` without
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
/// Measured on the binary of v1.1.37.02: the command takes
/// the name of the variable of the answer, the sub-command and the two
/// values, and the answer goes into the variable. The sub-command that it
/// does not know is refused with "Parameter #2 invalid.".
pub(crate) fn transform(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let sub = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let first = values
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let second = values
        .get(3)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let answer = the_answer_of_the_transform(rt, &sub, &first, &second, line)?;
    crate::v1::write_the_answer(rt, "Transform", values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// The answer of one sub-command of the `Transform`.
///
/// Measured on the binary of v1.1.37.02: every sub-command
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
            None => Value::Text("".into()),
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
                Some(character) if code != 0 => Value::Text(character.to_string().into()),
                _ => Value::Text("".into()),
            })
        }
        // The remainder of a division, and the divisor of zero is a failure.
        "mod" => {
            let divisor = number(plain(second));
            if divisor == 0.0 {
                return invalid(format!(
                    "Divide by zero\n     Specifically: {}",
                    plain(second)
                ));
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
        "floor" => Ok(Value::Int(
            crate::runtime::floor(number(plain(first))) as i64
        )),
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
                        return Ok(Value::Text("".into()));
                    }
                    Ok(Value::Text(rest.to_string().into()))
                }
                None => Ok(Value::Text(text.to_string().into())),
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
                return Ok(Value::Text("".into()));
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
            Ok(Value::Float(crate::runtime::powf(
                base,
                number(plain(second)),
            )))
        }
        // The bits of the v1: the `BitNot` is of thirty-two bits and the
        // rest is of sixty-four.
        "bitnot" => {
            let bits = number(plain(first)) as u32;
            Ok(Value::Int((!bits) as i64))
        }
        "bitand" => Ok(Value::Int(
            number(plain(first)) as i64 & number(plain(second)) as i64,
        )),
        "bitor" => Ok(Value::Int(
            number(plain(first)) as i64 | number(plain(second)) as i64,
        )),
        "bitxor" => Ok(Value::Int(
            number(plain(first)) as i64 ^ number(plain(second)) as i64,
        )),
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
            Ok(Value::Text(text.into()))
        }
        // The HTML of the v1 is a table of the names of the characters that
        // are not of the English, and the tree does not carry it.
        "html" => invalid(
            "this runtime does not carry the sub-command of the v1.\n     Specifically: HTML"
                .to_string(),
        ),
        // The sub-command that the v1 does not know, and the one of the
        // clipboard that a Unicode build does not have.
        _ => invalid(format!("Parameter #2 invalid.\n     Specifically: {sub}")),
    }
}

/// `IsSet(name)`: whether the variable of that name has a value.
///
/// Measured on the binary of v1.1.37.02: the answer is 1 for
/// a variable that has a value and 0 for one that has none, and a built-in
/// variable of the v1 has a value.
pub(crate) fn is_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let answer = if rt.has_a_value(&name) { 1 } else { 0 };
    Ok(Value::Int(answer))
}

/// `IsFunc(name)`: whether the name is a function.
///
/// Measured on the binary of v1.1.37.02: the answer is 1 for
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
/// Measured on the binary of v1.1.37.02: the answer is 1 for
/// a variable that is a reference and 0 for one that is not.
pub(crate) fn is_by_ref(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let answer = if rt.is_a_reference(&name) { 1 } else { 0 };
    Ok(Value::Int(answer))
}

/// `Object()`: an object of the v1.
///
/// Measured on the binary of v1.1.37.02: the object of the
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
/// Measured on the binary of v1.1.37.02: the answer is a
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
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
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
/// Measured on the binary of v1.1.37.02: the command wants
/// three parameters, and the answer is the line itself, without the line
/// ending, and it is empty when the file has no such line.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the file
/// that is not there is not a failure of the script: the `ErrorLevel` is one,
/// the script goes on, and the variable of the caller is empty.
pub(crate) fn file_read_line(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let number = values
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let number: usize = number.trim().parse().unwrap_or(0);
    let lines = match rt.desktop().read_file(&path) {
        Ok(lines) => lines,
        Err(reason) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!("the file {path} was not read: {reason}"));
            let answer = Value::Text("".into());
            crate::v1::write_the_answer(rt, "FileReadLine", values, 0, answer.clone(), line)?;
            return Ok(answer);
        }
    };
    rt.set_the_error_level(Value::Int(0));
    let answer = match number.checked_sub(1).and_then(|at| lines.get(at)) {
        Some(text) => Value::Text(text.clone().into()),
        None => Value::Text("".into()),
    };
    crate::v1::write_the_answer(rt, "FileReadLine", values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// `Input, name, options, title, default`: what the user writes.
///
/// Measured on the binary of v1.1.37.02: the command asks
/// the user with a dialogue, and the text of the user is the answer.
pub(crate) fn input(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let options = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let title = values
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let default = values
        .get(3)
        .map(|value| value.to_text())
        .unwrap_or_default();
    // The title of the v1 is the prompt of the dialogue, and the options
    // are the ones that the v2 reads.
    let options = crate::dialogs::the_options_of_the_input_box(&options, line)?;
    let answer = rt
        .desktop()
        .input_box(&title, &title, &options, &default)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    let answer = Value::Text(answer.value.into());
    crate::v1::write_the_answer(rt, "Input", values, 0, answer.clone(), line)?;
    Ok(answer)
}
