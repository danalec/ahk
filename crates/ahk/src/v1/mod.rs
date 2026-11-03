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
use alloc::vec;
use alloc::vec::Vec;

use crate::builtins::BuiltinHandler;
use crate::runtime::{Runtime, RuntimeError, Value};

mod chars;
mod dates;
mod files;
mod hotkeys;
mod keys;
mod kinds;
mod labels;
mod loops;
mod math;
mod names;
mod numbers;
mod parameters;
mod strings;
mod sub_commands;
mod table;
mod variables;

pub(crate) use names::the_place_of_the_argument_that_wants_the_name;
pub(crate) use names::the_place_of_the_argument_that_wants_the_variable;
pub(crate) use names::the_command_has_a_body;
pub(crate) use names::the_command_is_the_else;
pub(crate) use names::{else_of_the_v1, if_in_string, if_msg_box, if_not_in_string};
pub use table::BUILTINS;
pub use variables::{carries as carries_variable, may_not_be_declared, refusal_of as refusal_of_variable};

pub(crate) use chars::chr;

pub(crate) use dates::{the_date_of_the_v1, the_text_is_a_number_of_days};

pub(crate) use kinds::{keeps_its_text, wants_a_number};

pub(crate) use labels::the_label_of_the_command;
pub(crate) use labels::the_refusal_of_a_goto;

pub(crate) use loops::the_flag_asks_for_the_recursion;
pub(crate) use loops::the_mode_of_the_loop_of_the_files;
pub(crate) use loops::the_refusal_of_the_name_of_the_parse_loop;
pub(crate) use loops::the_refusal_of_the_parameters_of_the_loop;
pub(crate) use loops::the_refusal_of_the_registry_of_the_loop;
pub(crate) use loops::{THE_FILES, THE_FILES_AND_THE_FOLDERS, THE_FOLDERS};

pub(crate) use hotkeys::is_the_name_of_a_command;
pub(crate) use hotkeys::the_context_of_the_v1_is_refused_in_a_hotkey;
pub(crate) use hotkeys::the_name_of_a_hotkey;
pub(crate) use hotkeys::TheName;

pub(crate) use math::{
    abs, acos, asin, atan, ceil, cos, exp, floor, ln, log, max, min, modulo, round, sin, sqrt, tan,
};

pub(crate) use numbers::{
    the_count_of_the_loop, the_number_at_the_start_of_a_text, the_number_is_a_number, the_number_of_a_text,
    the_text_is_a_float, the_text_is_a_hex, the_text_is_the_count_of_a_loop, the_text_of_the_number_that_fits,
    the_whole_number_at_the_start_of_a_text, the_whole_number_of_a_hex,
};

pub(crate) use parameters::the_refusal_of_a_parameter;
pub(crate) use parameters::is_the_name_of_a_variable;
pub(crate) use parameters::is_the_name_of_a_variable_that_may_carry_a_point;
pub(crate) use parameters::the_refusal_of_the_name_of_a_variable;
pub(crate) use parameters::the_refusal_of_the_percent_sign_of_a_parameter;
pub(crate) use parameters::the_refusal_of_the_arity;
pub(crate) use sub_commands::the_refusal_of_a_sub_command;
pub(crate) use sub_commands::the_parameter_may_not_be_blank;

pub(crate) use strings::{in_str, sub_str};

pub(crate) use names::{
    auto_trim, coord_mode_of_the_v1, file_encoding_of_the_v1, file_read_line, func, input, is_by_ref,
    is_func, is_set, object, set_batch_lines, set_control_delay_of_the_v1,
    set_detect_hidden_text_of_the_v1, set_detect_hidden_windows_of_the_v1, set_format,
    set_key_delay_of_the_v1, set_mouse_delay_of_the_v1, set_store_caps_lock_mode_of_the_v1,
    set_win_delay_of_the_v1, string_case_sense, transform, var_set_capacity,
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

/// The write of the built-in variable of this name, when the name is one.
///
/// `None` means the name is not a built-in variable of the v1, or that the
/// v1 lets a script write it: the runtime keeps the value in the table of
/// the script.
pub(crate) fn write_variable(
    name: &str,
    op: crate::ast::AssignOp,
    value: &Value,
    line: u32,
    rt: &mut Runtime,
) -> Option<Result<Value, RuntimeError>> {
    variables::write(name, op, value, line, rt)
}

/// Whether the v1 does not let a script write the built-in variable of this
/// name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the write of
/// every built-in variable of the v1 is refused at the load, and the only
/// ones that a script may write are the clipboard, the last error of the run
/// and the arguments of the script.
pub(crate) fn is_read_only(name: &str) -> bool {
    variables::is_read_only(name)
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

/// `EnvSet, name, value`: the value of an environment variable.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is one when there was a problem and zero otherwise, and the v1
/// goes on: the name of the variable of the environment may not carry the
/// equal sign, the machine refuses it, and the reason of it is a note.
pub(crate) fn env_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|value| value.to_text()).unwrap_or_default();
    let value = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    match rt.desktop().env_set(&name, &value) {
        Ok(answer) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(Value::Text(answer))
        }
        Err(reason) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!("the variable of the environment was not set: {reason}"));
            Ok(Value::Text(value))
        }
    }
}

/// `EnvUpdate`: the environment of the script again.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 takes the
/// environment of the process again, and the tree shares the environment of
/// the process with the script: the command has nothing to do.
pub(crate) fn env_update(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(String::new()))
}

/// `FileGetAttrib, name, file`: the attributes of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the file that is not
/// there is not a failure of the script: the `ErrorLevel` is one, the script
/// goes on, and the variable of the caller is empty.
pub(crate) fn file_get_attrib(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_of_a_file_of_the_loop(
        rt,
        "FileGetAttrib",
        values,
        line,
        crate::builtins::file_get_attrib,
        4,
    )
}

/// `FileGetSize, name, file, unit`: the size of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the file that is not
/// there is not a failure of the script: the `ErrorLevel` is one, the script
/// goes on, and the variable of the caller is empty.
pub(crate) fn file_get_size(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_of_a_file_of_the_loop(
        rt,
        "FileGetSize",
        values,
        line,
        crate::builtins::file_get_size,
        4,
    )
}

/// `FileGetTime, name, file, unit`: the time of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the file that is not
/// there is not a failure of the script: the `ErrorLevel` is one, the script
/// goes on, and the variable of the caller is empty.
pub(crate) fn file_get_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_of_a_file_of_the_loop(
        rt,
        "FileGetTime",
        values,
        line,
        crate::builtins::file_get_time,
        4,
    )
}

/// `FileRead, name, file`: the text of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is zero when the load was successful and one when it was not.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the file that is not
/// there is not a failure of the script: the `ErrorLevel` is one, the script
/// goes on, and the variable of the caller is empty.
pub(crate) fn file_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_answer_of_a_file_command(rt, "FileRead", values, line, crate::builtins::file_read, 0)
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

/// `GetKeyState, name, key, mode`: the state of a key.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the v1 has the
/// command and the function, and they are two things. The command writes
/// the letter D or U into the variable of the caller, and the function
/// answers one or zero. The variant of the row says which of the two is
/// the one that runs: the command is the first row of the table and the
/// function is the second one.
///
/// The name of the key is checked when the script loads, and the function
/// answers an empty text when the name is not a key: the v1 does not fail
/// for a name that it does not know.
pub(crate) fn get_key_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let is_the_command = variant == 0;
    // The name of the key is the second parameter of the command and the
    // first one of the function.
    let name = if is_the_command { values.get(1) } else { values.first() };
    let answer = match (is_the_command, the_state_of_a_key(rt, name, line)?) {
        // The command writes the letter D or U.
        (true, Some(true)) => Value::Text("D".to_string()),
        (true, Some(false)) => Value::Text("U".to_string()),
        // The function answers one or zero.
        (false, Some(true)) => Value::Int(1),
        (false, Some(false)) => Value::Int(0),
        // The state could not be determined: the v1 answers an empty text.
        (_, None) => Value::Text(String::new()),
    };
    if is_the_command {
        write_the_answer(rt, "GetKeyState", values, 0, answer.clone(), line)?;
    }
    Ok(answer)
}

/// The state of the key of this name.
///
/// The answer is `None` when the name is not a key of the v1, and it is
/// the state of the key when it is one.
fn the_state_of_a_key(
    rt: &mut Runtime<'_>,
    name: Option<&Value>,
    line: u32,
) -> Result<Option<bool>, RuntimeError> {
    let name = name.map(|value| value.to_text()).unwrap_or_default();
    let name = name.trim();
    if !keys::is_the_name_of_a_key(name) {
        return Ok(None);
    }
    match rt.desktop().key_is_down(name) {
        Ok(down) => Ok(Some(down)),
        Err(reason) => Err(RuntimeError { message: reason, line }),
    }
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

/// `StringGetPos, name, text, search, occurrence, offset`: the place of a text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the place counts from
/// zero, and it is minus one when the text is not there. The occurrence is
/// the one that the caller wants: "L1" is the first from the left and "R1" is
/// the first from the right, and the number that is alone is the first from
/// the right. The offset is the number of characters that the search skips
/// over, and it is on the side that the occurrence names.
pub(crate) fn string_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let haystack = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let needle = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let occurrence = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let offset = values.get(4).and_then(|value| value.to_number()).unwrap_or(0.0);
    let (from_the_right, number) = the_occurrence_of_the_v1(&occurrence, line)?;
    let method = rt.case_sense().the_method();
    let offset = if offset < 0.0 { 0usize } else { offset as usize };
    let characters: Vec<char> = haystack.chars().collect();
    let needle_characters: Vec<char> = needle.chars().collect();
    // The offset is on the side that the occurrence names: the left one
    // skips the characters of the beginning, and the right one skips the
    // characters of the end.
    let (text, from) = if from_the_right {
        let end = characters.len().saturating_sub(offset);
        (characters[..end].to_vec(), 0usize)
    } else {
        let from = offset.min(characters.len());
        (characters[from..].to_vec(), from)
    };
    let places = the_places_of_the_needle(&text, &needle_characters, method);
    // The place is the one of the occurrence that the caller wants, and it
    // is always relative to the first character of the text.
    let place = if number == 0 {
        None
    } else if from_the_right {
        places.iter().rev().nth(number - 1).copied()
    } else {
        places.get(number - 1).copied()
    };
    let answer = match place {
        Some(place) => Value::Int((place + from) as i64),
        None => Value::Int(-1),
    };
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel`
    // of the v1 is zero when the text is there and one when it is not.
    rt.set_the_error_level(Value::Int(if place.is_some() { 0 } else { 1 }));
    write_the_answer(rt, "StringGetPos", values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// The occurrence of the `StringGetPos` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the occurrence is the
/// letter "L" or "R" and the number of the one that the caller wants, and
/// the number that is alone is the first from the right. The number that is
/// not a number is no occurrence at all, and the place is minus one.
fn the_occurrence_of_the_v1(text: &str, line: u32) -> Result<(bool, usize), RuntimeError> {
    let wanted = text.trim();
    if !parameters::is_an_occurrence(wanted) {
        return Err(RuntimeError {
            message: format!("Parameter #4 invalid.\n     Specifically: {wanted}"),
            line,
        });
    }
    let Some(first) = wanted.chars().next() else {
        // The occurrence that is not there is the first from the left.
        return Ok((false, 1));
    };
    let from_the_right = match first {
        'L' | 'l' => false,
        'R' | 'r' => true,
        // The number that is alone is the first from the right.
        _ => return Ok((true, 1)),
    };
    let number = wanted[1..].trim().parse::<usize>().unwrap_or(0);
    Ok((from_the_right, number))
}

/// The places where the needle is in the text, with the method of the case.
fn the_places_of_the_needle(
    text: &[char],
    needle: &[char],
    method: crate::runtime::CaseMethod,
) -> Vec<usize> {
    let mut places = Vec::new();
    if needle.len() > text.len() {
        return places;
    }
    let wanted: String = needle.iter().collect();
    for at in 0..=(text.len() - needle.len()) {
        let found: String = text[at..at + needle.len()].iter().collect();
        if crate::runtime::the_same_text(&found, &wanted, method) {
            places.push(at);
        }
    }
    places
}

/// `StringReplace, name, text, search, replace, all`: a text with another.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the first occurrence
/// of the text is the one that is replaced, and the parameter that says
/// "all" replaces every one of them. The sense of the case says whether the
/// letters of the text and of the search are the same.
pub(crate) fn string_replace(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let haystack = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let needle = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let replacement = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let every = values.get(4).map(|value| value.to_text()).unwrap_or_default();
    let method = rt.case_sense().the_method();
    let limit = if the_replace_all_of_the_v1(&every) { usize::MAX } else { 1 };
    let (answer, count) =
        the_text_with_the_needle_in_it(&haystack, &needle, &replacement, limit, method);
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel`
    // of the v1 is the number of the replacements when the parameter says
    // "useerrorlevel", and zero when it does not.
    rt.set_the_error_level(Value::Int(if the_use_error_level_of_the_v1(&every) {
        count as i64
    } else {
        0
    }));
    write_the_answer(rt, "StringReplace", values, 0, Value::Text(answer.clone()), line)?;
    Ok(Value::Text(answer))
}

/// Whether the `ReplaceAll` of the `StringReplace` of the v1 replaces every
/// occurrence of the text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter says
/// "all" when it has the letter "a" -- in any case -- or the digit "1" in
/// it, and when it has the word "useerrorlevel" in it -- in any case, and
/// wherever it is in the parameter. The parameter that says nothing replaces
/// the first occurrence and no other.
fn the_replace_all_of_the_v1(text: &str) -> bool {
    let wanted = text.trim();
    if the_use_error_level_of_the_v1(wanted) {
        return true;
    }
    wanted.contains('a') || wanted.contains('A') || wanted.contains('1')
}

/// Whether the parameter of the `StringReplace` of the v1 says that the
/// number of the replacements goes into the `ErrorLevel`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter says
/// it when it has the word "useerrorlevel" in it, in any case, and wherever
/// it is in the parameter.
fn the_use_error_level_of_the_v1(text: &str) -> bool {
    text.trim().to_lowercase().contains("useerrorlevel")
}

/// The text with the needle in it replaced by the other text, and the number
/// of the replacements.
///
/// The limit is the number of the replacements that the caller allows, and the
/// part of the text that is at the right of the last one is left as it is.
fn the_text_with_the_needle_in_it(
    haystack: &str,
    needle: &str,
    replacement: &str,
    limit: usize,
    method: crate::runtime::CaseMethod,
) -> (String, usize) {
    let characters: Vec<char> = haystack.chars().collect();
    let needle_characters: Vec<char> = needle.chars().collect();
    let places = the_places_of_the_needle(&characters, &needle_characters, method);
    let mut answer = String::new();
    let mut at = 0usize;
    let mut count = 0usize;
    for place in places {
        if place < at || count >= limit {
            break;
        }
        answer.extend(characters[at..place].iter());
        answer.push_str(replacement);
        at = place + needle_characters.len();
        count += 1;
    }
    answer.extend(characters[at.min(characters.len())..].iter());
    (answer, count)
}

/// `StrReplace(Haystack, Needle, ReplaceText, OutputVarCount, Limit)`: a text
/// with another.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the needle is not
/// with the case when the sense of the case is Off, and every occurrence of
/// it is replaced when the limit is not there. The fourth parameter is the
/// variable that receives the number of the replacements -- a bare name
/// receives it and a text, a number or an expression does not -- and the
/// fifth is the number of the replacements that the caller allows. The limit
/// that is zero replaces none of them.
pub(crate) fn str_replace(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let haystack = values.first().map(|value| value.to_text()).unwrap_or_default();
    let needle = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let replacement = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let limit = values.get(4).and_then(|value| value.to_number()).unwrap_or(-1.0);
    let method = rt.case_sense().the_method();
    let limit = if limit < 0.0 { usize::MAX } else { limit as usize };
    let (answer, count) = the_text_with_the_needle_in_it(
        &haystack,
        &needle,
        &replacement,
        limit,
        method,
    );
    // The number of the replacements goes into the variable of the caller,
    // and a fourth argument that is not a variable has nowhere to go.
    if let Some(Value::Reference(_)) = values.get(3) {
        rt.write_reference(&values[3], Value::Int(count as i64));
    }
    Ok(Value::Text(answer))
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

// -- the files of the machine, and the ErrorLevel ---------------------------------

/// `FileAppend, text, file`: add to the end of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is one when there was a problem and zero otherwise.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the file that cannot
/// be written is not a failure of the script: the `ErrorLevel` is one and the
/// script goes on.
pub(crate) fn file_append(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::file_append(rt, values, line, variant);
    the_error_level_of_a_command_that_goes_on(rt, answer)
}

/// `FileCopyDir, source, destination`: take a copy of a folder.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the folder
/// that is not there is not a failure of the script: the `ErrorLevel` is one
/// and the script goes on.
pub(crate) fn file_copy_dir(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::dir_copy(rt, values, line, variant);
    the_error_level_of_a_command_that_goes_on(rt, answer)
}

/// `FileMoveDir, source, destination, way`: move a folder.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the folder
/// that is not there is not a failure of the script: the `ErrorLevel` is one
/// and the script goes on.
pub(crate) fn file_move_dir(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::dir_move(rt, values, line, variant);
    the_error_level_of_a_command_that_goes_on(rt, answer)
}

/// `FileRemoveDir, folder`: take a folder away.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the folder
/// that is not there is not a failure of the script: the `ErrorLevel` is one
/// and the script goes on.
pub(crate) fn file_remove_dir(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::builtins::dir_delete(rt, values, line, variant);
    the_error_level_of_a_command_that_goes_on(rt, answer)
}

/// `FileCopy, source, destination`: take a copy of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the source of the
/// v1 may be a pattern, and the command takes a copy of every file that
/// matches it -- into the folder of the destination, with the name of the
/// file. The `ErrorLevel` is the number of the files that could not be
/// copied, and the v1 goes on: the `ErrorLevel` is the only report of it.
pub(crate) fn file_copy(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let from = crate::files::the_name_of_a_file(values, 0, 1, "FileCopy", line)?;
    let to = crate::files::the_name_of_a_file(values, 1, 2, "FileCopy", line)?;
    let overwrite = crate::files::flag_of(values, 2, "FileCopy", line)?;
    let files = files::the_files_of_the_pattern(rt, &from, line)?;
    let mut failed = 0;
    for file in files {
        let destination = if files::is_a_pattern(&from) {
            files::the_path_in_the_folder(&to, files::the_name_of_the_file(&file))
        } else {
            to.clone()
        };
        if let Err(reason) = rt.desktop().copy_file(&file, &destination, overwrite) {
            failed += 1;
            rt.note(format!("the file {file} was not copied: {reason}"));
        }
    }
    rt.set_the_error_level(Value::Int(failed));
    Ok(Value::Text(String::new()))
}

/// `FileDelete, file`: delete a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the file of the v1
/// may be a pattern, and the command deletes every file that matches it. The
/// `ErrorLevel` is the number of the files that failed to be deleted, and it
/// is zero when the pattern matches nothing.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the name that is not
/// a pattern is the name of one file, and the file that is not there is a
/// failure of the command: the `ErrorLevel` is one.
pub(crate) fn file_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = crate::files::the_name_of_a_file(values, 0, 1, "FileDelete", line)?;
    let files = if files::is_a_pattern(&path) {
        files::the_files_of_the_pattern(rt, &path, line)?
    } else {
        // The name that is not a pattern is the name of one file.
        vec![path.clone()]
    };
    let mut failed = 0;
    for file in files {
        if let Err(reason) = rt.desktop().delete_file(&file) {
            failed += 1;
            rt.note(format!("the file {file} was not deleted: {reason}"));
        }
    }
    rt.set_the_error_level(Value::Int(failed));
    Ok(Value::Text(String::new()))
}

/// `FileMove, source, destination`: move a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the source of the v1
/// may be a pattern, and the command moves every file that matches it -- into
/// the folder of the destination, with the name of the file. The `ErrorLevel`
/// is the number of the files that could not be moved, and the v1 goes on.
pub(crate) fn file_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let from = crate::files::the_name_of_a_file(values, 0, 1, "FileMove", line)?;
    let to = crate::files::the_name_of_a_file(values, 1, 2, "FileMove", line)?;
    let overwrite = crate::files::flag_of(values, 2, "FileMove", line)?;
    let files = files::the_files_of_the_pattern(rt, &from, line)?;
    let mut failed = 0;
    for file in files {
        let destination = if files::is_a_pattern(&from) {
            files::the_path_in_the_folder(&to, files::the_name_of_the_file(&file))
        } else {
            to.clone()
        };
        if let Err(reason) = rt.desktop().move_file(&file, &destination, overwrite) {
            failed += 1;
            rt.note(format!("the file {file} was not moved: {reason}"));
        }
    }
    rt.set_the_error_level(Value::Int(failed));
    Ok(Value::Text(String::new()))
}

/// `FileSetTime, file, ...`: change the time of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is the number of the files that failed to be changed.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the file
/// that is not there is not a failure of the script: the `ErrorLevel` is one
/// and the script goes on.
pub(crate) fn file_set_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    // The name of the file may be left out, and the file of the loop that
    // runs is the one that is changed.
    let Some(values) = the_values_of_the_file_of_the_loop(rt, values) else {
        rt.set_the_error_level(Value::Int(1));
        rt.note("the command FileSetTime did not do the thing: there is no file of a loop.");
        return Ok(Value::Text(String::new()));
    };
    let answer = crate::files::file_set_time(rt, &values, line, variant);
    the_error_level_of_a_command_that_goes_on(rt, answer)
}

/// The `ErrorLevel` of a command of the v1 that the machine could not do.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-05: the command that
/// cannot be done is not a failure of the script: the `ErrorLevel` is one and
/// the script goes on. The `ErrorLevel` is zero when the command did the
/// thing.
fn the_error_level_of_a_command_that_goes_on(
    rt: &mut Runtime<'_>,
    answer: Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!("the command did not do the thing: {}", failure.message));
            Ok(Value::Text(String::new()))
        }
    }
}

/// The answer of a command of the files of the v1 that writes into a variable.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
/// command that cannot be done is not a failure of the script: the
/// `ErrorLevel` is one and the script goes on, and the variable of the caller
/// is empty.
fn the_answer_of_a_file_command(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    line: u32,
    handler: BuiltinHandler,
    flavour: u8,
) -> Result<Value, RuntimeError> {
    let answer = handler(rt, values.get(1..).unwrap_or(&[]), line, flavour);
    let answer = the_error_level_of_a_command_that_goes_on(rt, answer)?;
    write_the_answer(rt, name, values, 0, answer.clone(), line)?;
    Ok(answer)
}

/// The values of a command of the files of the v1 whose name of the file may
/// be left out.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the name of
/// the file may be left out, and the file of the loop that runs is the one
/// that is used. The answer is nothing when there is no file at all.
fn the_values_of_the_file_of_the_loop(
    rt: &mut Runtime<'_>,
    values: &[Value],
) -> Option<Vec<Value>> {
    // The name of the file is the second parameter of the command of the v1.
    let file = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    if !file.is_empty() {
        return Some(values.to_vec());
    }
    let current = rt.the_file_of_the_loop()?;
    let mut values = values.to_vec();
    if values.len() < 2 {
        values.push(Value::Text(current));
    } else {
        values[1] = Value::Text(current);
    }
    Some(values)
}

/// The answer of a command of the files of the v1 whose name of the file may
/// be left out.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the name of
/// the file may be left out, and the file of the loop that runs is the one
/// that is used. There is no file at all: the `ErrorLevel` is one, the script
/// goes on, and the variable of the caller is empty.
fn the_answer_of_a_file_of_the_loop(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    line: u32,
    handler: BuiltinHandler,
    flavour: u8,
) -> Result<Value, RuntimeError> {
    let Some(values) = the_values_of_the_file_of_the_loop(rt, values) else {
        rt.set_the_error_level(Value::Int(1));
        rt.note(format!("the command {name} did not do the thing: there is no file of a loop."));
        write_the_answer(rt, name, values, 0, Value::Text(String::new()), line)?;
        return Ok(Value::Text(String::new()));
    };
    the_answer_of_a_file_command(rt, name, &values, line, handler, flavour)
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

/// `Run, target, dir, options, pid`: the program that runs.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the shape of the v1
/// is the target, the working directory, the options and the variable that
/// takes the pid of the process. The `ErrorLevel` is not touched, unless the
/// options say "useerrorlevel": then it is zero when the program starts, and
/// the word "ERROR" when it does not.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `Run` does not
/// wait for the program: the script that starts a program that takes four
/// seconds ends at once, and the `RunWait` waits for it.
pub(crate) fn run_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    run_or_wait_of_the_v1(rt, values, line, false)
}

/// `RunWait, target, dir, options, pid`: the program that runs, and the wait.
///
/// The `RunWait` of the v1 is a command of its own, and the page of the
/// documentation carries it with the `Run`. Measured on the binary of
/// v1.1.37.02 on 2026-11-02: the shape is the one of the `Run`, and the
/// `ErrorLevel` is the code of the program that ran.
///
/// The pid of the program that ran is not carried: the desktop that waits
/// for a program does not give the pid of it, and the parameter is refused
/// with the reason.
pub(crate) fn run_wait_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    run_or_wait_of_the_v1(rt, values, line, true)
}

/// The program that runs, and the `ErrorLevel` of it.
///
/// The two commands of the v1 have the same shape, and the `ErrorLevel` of
/// them is not the same: the `RunWait` writes the code of the program, and
/// the `Run` writes nothing unless the options say "useerrorlevel". The pid
/// of the program goes into the variable of the caller in the two of them.
fn run_or_wait_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    waits: bool,
) -> Result<Value, RuntimeError> {
    let target = values.first().map(|value| value.to_text()).unwrap_or_default();
    let dir = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let options = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let says = the_options_say_use_error_level(&options);
    if waits {
        // The `RunWait` waits for the program, and the pid of it goes into
        // the variable of the caller.
        let (pid, code, stdout, stderr) = match rt.desktop().run_and_wait(&target, &dir) {
            Ok(answer) => answer,
            Err(reason) => {
                // The v1 goes on when the program does not start, and the
                // `ErrorLevel` is the word that says it.
                if says {
                    rt.set_the_error_level(Value::Text("ERROR".into()));
                }
                return Err(RuntimeError { message: reason, line });
            }
        };
        if !stdout.trim().is_empty() {
            rt.push_output(stdout.trim_end().to_string());
        }
        if !stderr.trim().is_empty() {
            rt.push_output(stderr.trim_end().to_string());
        }
        // The code of the program is the `ErrorLevel` of the `RunWait`.
        rt.set_the_error_level(Value::Int(code as i64));
        write_the_answer(rt, "RunWait", values, 3, Value::Int(pid as i64), line)?;
        return Ok(Value::Int(code as i64));
    }
    // The `Run` does not wait for the program: it starts it and goes on.
    let pid = match rt.desktop().launch(&target, &dir) {
        Ok(pid) => pid,
        Err(reason) => {
            if says {
                rt.set_the_error_level(Value::Text("ERROR".into()));
            }
            return Err(RuntimeError { message: reason, line });
        }
    };
    if says {
        rt.set_the_error_level(Value::Int(0));
    }
    // The pid of the process goes into the variable of the caller.
    write_the_answer(rt, "Run", values, 3, Value::Int(pid as i64), line)?;
    Ok(Value::Int(pid as i64))
}

/// Whether the options of the command say "useerrorlevel".
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the word is in the
/// options in any case, and wherever it is in them -- "xUseErrorLevel" says
/// it as well, and an option that is not known is ignored.
fn the_options_say_use_error_level(options: &str) -> bool {
    options.to_lowercase().contains("useerrorlevel")
}

/// `ClipWait, timeout, kind`: the wait for the clipboard.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is one when the wait expires and zero when the clipboard has data
/// -- the other way round from the answer of the v2.
pub(crate) fn clip_wait_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::builtins::clip_wait(rt, values, line, variant)?;
    let expired = matches!(answer, Value::Int(0));
    rt.set_the_error_level(Value::Int(if expired { 1 } else { 0 }));
    Ok(answer)
}

/// `DllCall("function", ...)`: the call into a library of the machine.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is zero when the call works, and the code of the failure when it
/// does not -- the minus three when the library is not there, the minus four
/// when the function is not in it, and the minus two when the call itself is
/// not valid. The v1 goes on when the call fails, and the tree fails the
/// script with the reason of the machine.
pub(crate) fn dll_call_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match crate::builtins::dll_call_with_the_kind(rt, values, line) {
        Ok(answer) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(answer)
        }
        Err((kind, error)) => {
            // The code of the failure is the `ErrorLevel` of the v1.
            rt.set_the_error_level(Value::Int(the_code_of_the_failure(kind)));
            Err(error)
        }
    }
}

/// The code that the `ErrorLevel` of the v1 carries when a native call fails.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the code is the
/// minus three when the library is not there, the minus four when the
/// function is not in it, and the minus two when the call itself is not
/// valid.
fn the_code_of_the_failure(kind: crate::dll::NativeFailureKind) -> i64 {
    match kind {
        crate::dll::NativeFailureKind::Invalid => -2,
        crate::dll::NativeFailureKind::NoLibrary => -3,
        crate::dll::NativeFailureKind::NoFunction => -4,
    }
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

/// The arithmetic of the v1 on a variable of the script.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `EnvAdd`, the
/// `EnvSub`, the `EnvMult` and the `EnvDiv` of the v1 are the arithmetic of
/// the v1 on a variable of the script -- the `EnvAdd, x, 3` with `x` at five
/// leaves `x` at eight -- and not the arithmetic of a variable of the
/// environment. The number that is not there is a zero, and the command
/// writes the result into the variable.
///
/// The two whole numbers give a whole number, and the division of them is
/// rounded towards the zero: the `EnvDiv, x, 3` with `x` at five leaves `x`
/// at one, and the `EnvDiv, x, 2` with `x` at five and a half leaves `x` at
/// two and three quarters.
fn env_arithmetic(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
    operation: TheArithmetic,
) -> Result<Value, RuntimeError> {
    let Some(variable) = values.first() else {
        return Ok(Value::Text(String::new()));
    };
    let Value::Reference(reference) = variable else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(1, name, "a VarRef", variable),
            line,
        });
    };
    // The units of the time are the third parameter of the `EnvAdd` and of
    // the `EnvSub`, and they are the arithmetic of the date of the v1.
    if let Some(units) = values.get(2) {
        if !units.to_text().trim().is_empty() {
            return the_arithmetic_of_the_date(rt, values, line, name, operation, &units.to_text());
        }
    }
    let current = rt.read_reference(reference).map(|value| value.to_text()).unwrap_or_default();
    let current = crate::v1::the_number_at_the_start_of_a_text(&current);
    let other = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let other = crate::v1::the_number_at_the_start_of_a_text(&other);
    let left = the_number_of_the_v1(&current);
    let right = the_number_of_the_v1(&other);
    // The two whole numbers give a whole number, and the division of them is
    // rounded towards the zero. The number that is not there is a whole one.
    let whole = !matches!(current, Some(Value::Float(_))) && !matches!(other, Some(Value::Float(_)));
    let answer = match operation {
        TheArithmetic::Add => left + right,
        TheArithmetic::Subtract => left - right,
        TheArithmetic::Multiply => left * right,
        TheArithmetic::Divide => {
            if right == 0.0 {
                // The division by zero is refused when the script loads; the
                // variable is blank when it is not.
                rt.write_reference(variable, Value::Text(String::new()));
                return Ok(Value::Text(String::new()));
            }
            if whole {
                ((left as i64) / (right as i64)) as f64
            } else {
                left / right
            }
        }
    };
    let value = if whole {
        Value::Int(answer as i64)
    } else {
        Value::Float(answer)
    };
    rt.write_reference(variable, value.clone());
    Ok(value)
}

/// The arithmetic of the date of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the third
/// parameter of the `EnvAdd` and of the `EnvSub` is the unit of the time, and
/// the command is the arithmetic of the date of the machine. The `EnvAdd`
/// adds the amount to the date of the machine and answers with the date --
/// the `EnvAdd, out, 3, S` answers with the date of the machine three
/// seconds ahead -- and the `EnvSub` takes the unit from a date and answers
/// with the number of the units that are between the date and the date of the
/// machine: the `EnvSub, out, 20200101120000, S` answers with the number of
/// the seconds since the noon of the first of January of 2020.
///
/// The amount that is not a number leaves the variable empty, and the date
/// that is not a date answers with zero.
fn the_arithmetic_of_the_date(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
    operation: TheArithmetic,
    units: &str,
) -> Result<Value, RuntimeError> {
    let Some(variable) = values.first() else {
        return Ok(Value::Text(String::new()));
    };
    let Value::Reference(_) = variable else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(1, name, "a VarRef", variable),
            line,
        });
    };
    let Some(clock) = rt.wall_clock() else {
        return Err(RuntimeError {
            message: "the date of the machine is not known.".to_string(),
            line,
        });
    };
    let unit = the_number_of_the_unit_of_the_time(units);
    let text = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    match operation {
        TheArithmetic::Add => {
            // The amount that is not a number leaves the variable empty: the
            // "abc" is not a number, and the "5abc" is five.
            let Some(amount) = crate::v1::the_number_at_the_start_of_a_text(&text) else {
                return the_variable_is_empty(rt, variable);
            };
            let amount = the_number_of_the_v1(&Some(amount)) * unit as f64;
            // The time of the machine is a number of a hundred of nanoseconds
            // since the first of January of 1601, and the arithmetic of it
            // goes around: the amount of the biggest number of the machine
            // leaves the date of the machine where it was.
            let file_time = the_file_time_of_the_machine(&clock.local).wrapping_add(
                the_hundred_of_nanoseconds_of_the_amount(amount),
            );
            let millis = (file_time / 10_000) as i64 - THE_MILLIS_BETWEEN_1601_AND_1970;
            let date = crate::date::from_unix_millis(millis);
            // The date that the machine does not read leaves the variable
            // empty: the machine reads the years from 1601 to 30827.
            if !(THE_FIRST_YEAR..=THE_LAST_YEAR).contains(&date.year) {
                return the_variable_is_empty(rt, variable);
            }
            let answer = crate::date::the_number_text(&date);
            rt.write_reference(variable, Value::Text(answer.clone()));
            Ok(Value::Text(answer))
        }
        TheArithmetic::Subtract => {
            // The text that is empty is zero.
            if text.is_empty() {
                rt.write_reference(variable, Value::Int(0));
                return Ok(Value::Int(0));
            }
            // The date that is not a date leaves the variable empty: the
            // "abc" is not a date, and the "5" is not one either.
            let Some(number) = crate::v1::the_number_at_the_start_of_a_text(&text) else {
                return the_variable_is_empty(rt, variable);
            };
            // The date of the v1 is the whole number of the amount: the
            // "20200101120000.5" is the noon of the first of January of 2020.
            let whole = the_number_of_the_v1(&Some(number)) as i64;
            let Some(date) = crate::v1::the_date_of_the_v1(&whole.to_string()) else {
                return the_variable_is_empty(rt, variable);
            };
            let difference = crate::date::to_unix_millis(&clock.local) - crate::date::to_unix_millis(&date);
            let answer = difference / 1000 / unit;
            rt.write_reference(variable, Value::Int(answer));
            Ok(Value::Int(answer))
        }
        // The `EnvMult` and the `EnvDiv` have no arithmetic of the date.
        _ => Ok(Value::Text(String::new())),
    }
}

/// The variable of the script that the command leaves empty.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the amount
/// of the `EnvAdd` that is not a number leaves the variable empty, and the
/// answer of the command is the empty text.
fn the_variable_is_empty(rt: &mut Runtime<'_>, variable: &Value) -> Result<Value, RuntimeError> {
    rt.write_reference(variable, Value::Text(String::new()));
    Ok(Value::Text(String::new()))
}

/// The reach of the time of the machine.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the time of
/// the machine is a number of a hundred of nanoseconds since the first of
/// January of 1601, and the arithmetic of it goes around the largest number
/// of the machine. The machine reads the years from 1601 to 30827, and the
/// date that is not one of them leaves the variable empty: the `EnvAdd` of
/// a hundred thousand years is empty, and the `EnvAdd` of a hundred thousand
/// years before the machine is a date of the year 28793.
const THE_HUNDRED_OF_NANOSECONDS: u64 = 10_000_000;

/// The number of the milliseconds between the first of January of 1601 and the
/// first of January of 1970.
const THE_MILLIS_BETWEEN_1601_AND_1970: i64 = 11_644_473_600_000;

/// The first year that the machine reads.
const THE_FIRST_YEAR: i64 = 1601;

/// The last year that the machine reads.
const THE_LAST_YEAR: i64 = 30827;

/// The time of the machine, in the hundred of nanoseconds that it counts.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the time
/// of the machine is a number of a hundred of nanoseconds since the first of
/// January of 1601.
fn the_file_time_of_the_machine(local: &crate::date::DateTime) -> u64 {
    let millis = crate::date::to_unix_millis(local) + THE_MILLIS_BETWEEN_1601_AND_1970;
    (millis as u64) * 10_000
}

/// The amount of the seconds, in the hundred of nanoseconds that the time of
/// the machine counts.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// multiplication goes around the largest number of the machine, and so does
/// the amount of the seconds that does not fit in it.
fn the_hundred_of_nanoseconds_of_the_amount(amount: f64) -> u64 {
    let whole = amount as i64;
    let fraction = amount - whole as f64;
    (whole as u64)
        .wrapping_mul(THE_HUNDRED_OF_NANOSECONDS)
        .wrapping_add((fraction * THE_HUNDRED_OF_NANOSECONDS as f64) as u64)
}

/// The number of the seconds of the unit of the time.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the unit
/// is the first letter of the text -- the "Seconds" is the "S" -- and it is
/// the second, the minute, the hour or the day.
fn the_number_of_the_unit_of_the_time(units: &str) -> i64 {
    match units.trim().chars().next().map(|letter| letter.to_ascii_uppercase()) {
        Some('M') => 60,
        Some('H') => 3600,
        Some('D') => 86400,
        _ => 1,
    }
}

/// The arithmetic that a command of the v1 does.
#[derive(Clone, Copy)]
enum TheArithmetic {
    /// The sum.
    Add,
    /// The difference.
    Subtract,
    /// The product.
    Multiply,
    /// The quotient.
    Divide,
}

/// The number that the v1 reads in a value.
///
/// The number that is not there is a zero, and so is the text that does not
/// start with a number.
fn the_number_of_the_v1(value: &Option<Value>) -> f64 {
    match value {
        Some(Value::Int(number)) => *number as f64,
        Some(Value::Float(number)) => *number,
        _ => 0.0,
    }
}

/// `EnvAdd, variable, value`: add to the number of a variable of the script.
pub(crate) fn env_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, "EnvAdd", TheArithmetic::Add)
}

/// `EnvSub, variable, value`: take from the number of a variable of the script.
pub(crate) fn env_sub(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, "EnvSub", TheArithmetic::Subtract)
}

/// `EnvMult, variable, value`: multiply the number of a variable of the script.
pub(crate) fn env_mult(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, "EnvMult", TheArithmetic::Multiply)
}

/// `EnvDiv, variable, value`: divide the number of a variable of the script.
pub(crate) fn env_div(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    env_arithmetic(rt, values, line, "EnvDiv", TheArithmetic::Divide)
}
