//! The legacy dialect: the names of AutoHotkey v1.1.37.02.
//!
//! The table of the v1 is the table of the v1, measured on the binary of
//! v1.1.37.02 on 2026-10-28 and kept in
//! `crates/ahk/reference/ahk-v1.1.37.02.surface.tsv`. The names are the
//! ones that the v1 spells, and the shape of each parameter is the one
//! that the v1 has: the value of an expression, the name of a variable of
//! the script, or the name of the variable that the command writes into.
//!
//! A name that the v1 has and the v2 does not is here; a name that the two
//! share is a row of the v2, and the table of the v1 points at it.

use alloc::format;
use alloc::string::{String, ToString};

use crate::builtins::BuiltinHandler;
use crate::runtime::{Runtime, RuntimeError, Value};

mod names;
mod table;
mod variables;

pub(crate) use names::wants_the_name_of_the_argument;
pub use table::BUILTINS;
pub use variables::{carries as carries_variable, refusal_of as refusal_of_variable};

pub(crate) use names::{
    auto_trim, file_read_line, func, input, is_by_ref, is_func, is_set, object, set_format,
};

/// The built-in variables of the v1 that the v2 does not carry.
///
/// The name of a variable of the v1 has the prefix that the dialect gives
/// it: `A_Clipboard` and not `Clipboard`.
///
/// The value of each one of them, and the reason the two of them that the
/// machine knows are refused, are in the module of the variables.
pub const VARIABLES: &[&str] = &[
    "A_Args",
    "A_AutoTrim",
    "A_BatchLines",
    "A_CaretX",
    "A_CaretY",
    "A_DefaultGui",
    "A_DefaultListView",
    "A_DefaultTreeView",
    "A_ExitReason",
    "A_FormatFloat",
    "A_FormatInteger",
    "A_Gui",
    "A_GuiControl",
    "A_GuiControlEvent",
    "A_GuiEvent",
    "A_GuiHeight",
    "A_GuiWidth",
    "A_GuiX",
    "A_GuiY",
    "A_IPAddress1",
    "A_IsUnicode",
    "A_NumBatchLines",
    "A_OSType",
    "A_StringCaseSense",
    "A_ThisLabel",
    "A_ThisMenu",
    "A_ThisMenuItem",
    "A_ThisMenuItemPos",
];

/// The directives of the v1 that the v2 does not know.
///
/// The name of a directive is the one of the dialect without the hash,
/// and the case of it does not matter.
pub const DIRECTIVES: &[&str] = &[
    "commentflag",
    "escapechar",
    "hotkeyinterval",
    "hotkeymodifiertimeout",
    "if",
    "iftimeout",
    "ifwinactive",
    "installkeybdhook",
    "installmousehook",
    "keyhistory",
    "maxhotkeysperinterval",
    "maxmem",
    "menumaskkey",
    "noenv",
    "persistent",
];

/// Whether the v1 knows the directive of this name.
pub fn carries_directive(name: &str) -> bool {
    let wanted = name.to_lowercase();
    DIRECTIVES.iter().any(|carried| *carried == wanted)
}

/// The value of the built-in variable of this name, when the name is one.
///
/// `None` means the name is not a built-in variable of the v1: the name is a
/// variable of the script.
pub(crate) fn lookup_variable(
    name: &str,
    rt: &mut Runtime,
) -> Option<Result<Value, RuntimeError>> {
    variables::lookup(name, rt)
}

/// The number of the text of a legacy assignment of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the text of the
/// legacy assignment is a whole number when it is one in base ten. "5" and
/// "007" and "+5" and "-5" are numbers, and "5.5" and "0x10" and "1e3" and
/// "5 5" are texts.
pub(crate) fn the_number_of_a_legacy_text(text: &str) -> Option<Value> {
    let mut body = text;
    if let Some(rest) = body.strip_prefix('+') {
        body = rest;
    } else if let Some(rest) = body.strip_prefix('-') {
        body = rest;
    }
    if body.is_empty() || !body.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }
    text.parse::<i64>().ok().map(Value::Int)
}

/// The number of a text in the v1, when the text is one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "5" is 5, and "5.5"
/// is 5.5, and " 5 " is 5, and "0x10" is 16. A text that is not a number
/// ("abc", "5abc", "1e3" and the empty text) is not a number at all, and the
/// operation that wants one is blank.
pub(crate) fn the_number_of_a_text(text: &str) -> Option<Value> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    // The hex of the v1: 0x10 is 16.
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        return i64::from_str_radix(hex, 16).ok().map(Value::Int);
    }
    let body = text.strip_prefix('+').or_else(|| text.strip_prefix('-')).unwrap_or(text);
    if body.is_empty() {
        return None;
    }
    // A number of base ten: the sign, the digits and the point.
    let mut whole = true;
    let mut digits = 0usize;
    for character in body.chars() {
        match character {
            '0'..='9' => digits += 1,
            '.' if whole => whole = false,
            _ => return None,
        }
    }
    if digits == 0 {
        return None;
    }
    if whole {
        // The whole number: the sign is a part of it.
        return text.parse::<i64>().ok().map(Value::Int);
    }
    // The number with a point: the sign is a part of it.
    let number: f64 = text.parse().ok()?;
    Some(Value::Float(number))
}

/// The write of the built-in variable of this name, when the name is one.
///
/// `None` means the name is not a built-in variable of the v1, or that the
/// v1 lets a script write it: the runtime keeps the value in the table of
/// the script.
pub(crate) fn write_variable(
    name: &str,
    op: crate::ast::AssignOp,
    line: u32,
) -> Option<Result<Value, RuntimeError>> {
    variables::write(name, op, line)
}

// -- the answer that goes into the variable of the caller ---------------------

/// The hotstring of the v1: `::btw::by the way`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: the line is a
/// hotstring, and the text that comes after the second pair of colons is
/// the action of it.
pub(crate) fn hotstring_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values.first().map(|value| value.to_text()).unwrap_or_default();
    let invalid = RuntimeError {
        message: "Parameter #1 of Hotstring is invalid.".to_string(),
        line,
    };
    // The trigger of the hotstring is between the two pairs of colons, and
    // the action is what comes after them.
    let Some(rest) = text.strip_prefix("::") else { return Err(invalid) };
    let Some((trigger, action)) = rest.split_once("::") else { return Err(invalid) };
    // The v2 writes the trigger between two colons.
    let definition = format!(":{trigger}:");
    crate::builtins::hotstring(
        rt,
        &[Value::Text(definition), Value::Text(action.to_string())],
        line,
        1,
    )
}

/// The answer of a command of the v1 goes into the variable of the caller.
///
/// The command of the v1 writes its answer into a variable of the script,
/// and the function of the v2 answers with the value: the function of the
/// v2 answers, and the answer goes into the first parameter of the command,
/// which is the name of that variable.
fn the_answer_goes_into_the_first_output(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    line: u32,
    handler: BuiltinHandler,
    flavour: u8,
) -> Result<Value, RuntimeError> {
    let answer = handler(rt, values.get(1..).unwrap_or(&[]), line, flavour)?;
    write_the_answer(rt, name, values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// Write the answer of a command of the v1 into the variable of the caller.
pub(crate) fn write_the_answer(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    at: usize,
    answer: Value,
    line: u32,
) -> Result<(), RuntimeError> {
    let Some(value) = values.get(at) else { return Ok(()) };
    if !matches!(value, Value::Reference(_)) {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(at + 1, name, "a VarRef", value),
            line,
        });
    }
    rt.write_reference(value, answer);
    Ok(())
}

/// `EnvGet, name, value`: the value of an environment variable.
pub(crate) fn env_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "EnvGet", values, line, crate::builtins::env_get, 0)
}

/// `FileGetAttrib, name, file`: the attributes of a file.
pub(crate) fn file_get_attrib(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FileGetAttrib",
        values,
        line,
        crate::builtins::file_get_attrib,
        4,
    )
}

/// `FileGetSize, name, file, unit`: the size of a file.
pub(crate) fn file_get_size(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FileGetSize",
        values,
        line,
        crate::builtins::file_get_size,
        4,
    )
}

/// `FileGetTime, name, file, unit`: the time of a file.
pub(crate) fn file_get_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FileGetTime",
        values,
        line,
        crate::builtins::file_get_time,
        4,
    )
}

/// `FileRead, name, file`: the text of a file.
pub(crate) fn file_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "FileRead", values, line, crate::builtins::file_read, 0)
}

/// `FormatTime, name, format, time`: the time of the machine as a text.
pub(crate) fn format_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FormatTime",
        values,
        line,
        crate::builtins::format_time,
        2,
    )
}

/// `GetKeyState, name, key, physical`: the state of a key.
pub(crate) fn get_key_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "GetKeyState",
        values,
        line,
        crate::builtins::get_key_state,
        1,
    )
}

/// `IniRead, name, file, section, key, default`: the value of a key of a file.
pub(crate) fn ini_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "IniRead", values, line, crate::builtins::ini_read, 4)
}

/// `PixelGetColor, name, x, y`: the colour of a pixel of the screen.
pub(crate) fn pixel_get_color(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "PixelGetColor",
        values,
        line,
        crate::builtins::pixel_get_color,
        0,
    )
}

/// `Random, name, min, max`: a number between the two.
pub(crate) fn random(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "Random", values, line, crate::builtins::random, 0)
}

/// `WinGetTitle, name, title, ...`: the title of a window.
pub(crate) fn win_get_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "WinGetTitle",
        values,
        line,
        crate::builtins::win_get_title,
        0,
    )
}

/// `StringLen, name, text`: the length of the text of a variable.
pub(crate) fn string_len(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "StringLen", values, line, crate::builtins::str_len, 0)
}

/// `StringMid, name, text, start, length`: a part of the text of a variable.
pub(crate) fn string_mid(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "StringMid", values, line, crate::builtins::sub_str, 0)
}

/// `StringGetPos, name, text, search, offset`: the place of a text.
pub(crate) fn string_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(rt, "StringGetPos", values, line, crate::builtins::in_str, 0)
}

/// `StringReplace, name, text, search, replace, case`: a text with another.
pub(crate) fn string_replace(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "StringReplace",
        values,
        line,
        crate::builtins::str_replace,
        0,
    )
}

/// `StringSplit, name, text, delimiters, chars`: the parts of a text.
pub(crate) fn string_split(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "StringSplit",
        values,
        line,
        crate::builtins::str_split,
        0,
    )
}

/// `StringTrimLeft, name, text, chars`: a text without the spaces of its left.
pub(crate) fn string_trim_left(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "StringTrimLeft",
        values,
        line,
        crate::builtins::trim,
        0,
    )
}

/// `StringLower, name, text, case`: a text with the letters in lower case.
pub(crate) fn string_lower(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "StringLower",
        values,
        line,
        crate::builtins::change_case,
        0,
    )
}

/// `FileSelectFile, name, ...`: the file that the user chose.
pub(crate) fn file_select_file(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FileSelectFile",
        values,
        line,
        crate::builtins::file_select,
        0,
    )
}

/// `FileSelectFolder, name, ...`: the folder that the user chose.
pub(crate) fn file_select_folder(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_goes_into_the_first_output(
        rt,
        "FileSelectFolder",
        values,
        line,
        crate::builtins::dir_select,
        0,
    )
}

/// `InputBox, name, ...`: the text that the user wrote.
pub(crate) fn input_box_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v2 answers with an object that carries the text of the user in
    // its `Value`, and the v1 writes that text into the variable of the
    // caller.
    let answer = crate::builtins::input_box(rt, values.get(1..).unwrap_or(&[]), line, 0)?;
    let text = match rt.read_property(&answer, "Value", line) {
        Ok(value) => value,
        Err(failure) => {
            return Err(RuntimeError { message: crate::runtime::the_message_of(&failure), line });
        }
    };
    write_the_answer(rt, "InputBox", values, 0, text, line)?;
    Ok(answer)
}

/// `Run, program, dir, use, name`: the program that runs, and its pid.
pub(crate) fn run_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The pid of the v1 goes into the last parameter of the command, and
    // the parameters of the program are the ones before it.
    let last = values.len().saturating_sub(1);
    let answer = crate::builtins::run(rt, &values[..last], line, 0)?;
    write_the_answer(rt, "Run", values, last, answer.clone(), line)?;
    Ok(answer)
}

/// `StringLeft, name, text, count`: the left of a text.
pub(crate) fn string_left(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The left of a text is the part of it that begins at its first
    // letter, and the count says how many letters it has.
    let text = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    let count = values.get(2).cloned().unwrap_or(Value::Text(String::new()));
    let part = crate::builtins::sub_str(rt, &[text, Value::Int(1), count], line, 0)?;
    write_the_answer(rt, "StringLeft", values, 0, part.clone(), line)?;
    Ok(part)
}

// -- the environment of the machine ---------------------------------------------

/// The arithmetic of the environment of the machine.
///
/// The v1 adds to the number of an environment variable, and the number is
/// a whole one: the v1 rounds it.
fn env_arithmetic(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    operation: fn(i64, i64) -> i64,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    let current = rt.desktop().env_get(&name).map_err(|reason| RuntimeError { message: reason, line })?;
    let current = current.trim().parse::<i64>().unwrap_or(0);
    let other = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let other = other.trim().parse::<i64>().unwrap_or(0);
    let answer = operation(current, other);
    let text = format!("{answer}");
    rt.desktop().env_set(&name, &text).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(text))
}

/// `EnvAdd, name, value`: add to the number of an environment variable.
pub(crate) fn env_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, |left, right| left + right)
}

/// `EnvSub, name, value`: take from the number of an environment variable.
pub(crate) fn env_sub(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, |left, right| left - right)
}

/// `EnvMult, name, value`: multiply the number of an environment variable.
pub(crate) fn env_mult(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, |left, right| left * right)
}

/// `EnvDiv, name, value`: divide the number of an environment variable.
pub(crate) fn env_div(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, |left, right| {
        if right == 0 { 0 } else { left / right }
    })
}
