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
use alloc::vec::Vec;

use crate::builtins::BuiltinHandler;
use crate::runtime::{Runtime, RuntimeError, Value};

mod chars;
mod kinds;
mod math;
mod names;
mod numbers;
mod strings;
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

pub(crate) use kinds::wants_a_number;

pub(crate) use math::{
    abs, acos, asin, atan, ceil, cos, exp, floor, ln, log, max, min, modulo, round, sin, sqrt, tan,
};

pub(crate) use numbers::{
    the_number_at_the_start_of_a_text, the_number_is_a_number, the_number_of_a_text,
    the_text_is_a_float, the_text_is_a_hex, the_text_of_the_number_that_fits,
    the_whole_number_at_the_start_of_a_text, the_whole_number_of_a_hex,
};

pub(crate) use strings::{in_str, sub_str};

pub(crate) use names::{
    auto_trim, file_read_line, func, input, is_by_ref, is_func, is_set, object, set_format,
    string_case_sense, transform, var_set_capacity,
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
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is zero when the load was successful and one when it was not.
pub(crate) fn file_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = the_answer_goes_into_the_first_output(
        rt,
        "FileRead",
        values,
        line,
        crate::builtins::file_read,
        0,
    );
    the_error_level_of_a_file_command(rt, answer)
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
    let Some(first) = wanted.chars().next() else {
        // The occurrence that is not there is the first from the left.
        return Ok((false, 1));
    };
    let from_the_right = match first {
        'L' | 'l' => false,
        'R' | 'r' => true,
        // The number that is alone is the first from the right.
        '1' => return Ok((true, 1)),
        _ => {
            return Err(RuntimeError {
                message: format!("Parameter #4 invalid.\n     Specifically: {wanted}"),
                line,
            })
        }
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
pub(crate) fn file_append(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::file_append(rt, values, line, variant);
    the_error_level_of_a_file_command(rt, answer)
}

/// `FileCopy, source, destination`: take a copy of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is the number of the files that could not be copied, and the tree
/// refuses the wildcards, so it is one when the command fails and zero when
/// it does not.
pub(crate) fn file_copy(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::file_copy(rt, values, line, variant);
    the_error_level_of_a_file_command(rt, answer)
}

/// `FileDelete, file`: delete a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is the number of the files that failed to be deleted.
pub(crate) fn file_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::builtins::file_delete(rt, values, line, variant);
    the_error_level_of_a_file_command(rt, answer)
}

/// `FileMove, source, destination`: move a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is the number of the files that could not be moved.
pub(crate) fn file_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::file_move(rt, values, line, variant);
    the_error_level_of_a_file_command(rt, answer)
}

/// `FileSetTime, file, ...`: change the time of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` of
/// the v1 is the number of the files that failed to be changed.
pub(crate) fn file_set_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::files::file_set_time(rt, values, line, variant);
    the_error_level_of_a_file_command(rt, answer)
}

/// The `ErrorLevel` of a file command of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` is
/// the number of the files that the command could not do, and the tree
/// refuses the wildcards, so the number is one when the command fails and
/// zero when it does not.
fn the_error_level_of_a_file_command(
    rt: &mut Runtime<'_>,
    answer: Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    rt.set_the_error_level(Value::Int(if answer.is_ok() { 0 } else { 1 }));
    answer
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
/// the `Run` writes nothing unless the options say "useerrorlevel".
fn run_or_wait_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    waits: bool,
) -> Result<Value, RuntimeError> {
    let target = values.first().map(|value| value.to_text()).unwrap_or_default();
    let dir = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let options = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    // The pid of the process: the machine of this runtime does not give it.
    if values.len() > 3 {
        return Err(RuntimeError {
            message: "the pid of the process is not carried yet.".into(),
            line,
        });
    }
    let says = the_options_say_use_error_level(&options);
    let (code, stdout, stderr) = match rt.desktop().run(&target, &dir) {
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
    if waits {
        // The code of the program is the `ErrorLevel` of the `RunWait`.
        rt.set_the_error_level(Value::Int(code as i64));
    } else if says {
        rt.set_the_error_level(Value::Int(0));
    }
    Ok(Value::Int(code as i64))
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
/// the v1 is zero when the call works. The code of the failure is not
/// carried: the machine of this runtime answers with a reason and not with
/// the code of the v1.
pub(crate) fn dll_call_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::builtins::dll_call(rt, values, line, variant)?;
    rt.set_the_error_level(Value::Int(0));
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
