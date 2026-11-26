//! The commands of the v1 that the tree now carries.
//!
//! The v1 writes the answer of a command into the variable of the caller,
//! and the v2 answers with the value. The command of the v1 is therefore
//! the command of the v2 with the values of the caller and the answer
//! written into the first parameter.
//!
//! The failures of the v1 are not the failures of the v2. Measured on the
//! binary of v1.1.37.02, the command that does not find
//! what it asks for is not a failure of the script: the variable of the
//! caller is empty, the `ErrorLevel` says whether the command did the
//! thing, and the script goes on.
//!
//! The `ErrorLevel` of each command is the one the binary gives, and it is
//! not the same for all of them:
//!
//! | the command | the `ErrorLevel` when the thing is not there |
//! | --- | --- |
//! | the `WinGetTitle`, the `WinGetClass`, the `WinGetPos` | zero |
//! | the `WinGetText`, the `ControlGetText`, the `ControlGetFocus` | one |
//! | the `RegRead`, the `ControlGet` | one |
//!
//! The names of the v1 that the v2 does not have are here too: the
//! `RegRead` of the v1 is a command that writes into a variable and the
//! `RegRead` of the v2 is a function that answers with the value, and the
//! `SysGet` of the v1 is a command with a sub-command where the `SysGet`
//! of the v2 is a command with a number.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::builtins::{self, BuiltinHandler};
use crate::runtime::{Runtime, RuntimeError, Value};

pub(crate) use super::write_the_answer;

/// The answer of a command of the v1 that goes into the variable of the
/// caller, and the failure of it is a note and not a failure of the script.
///
/// The `ErrorLevel` is one when the command did not do the thing, and the
/// variable of the caller is empty.
fn the_answer_that_goes_on(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    line: u32,
    answer: Result<Value, RuntimeError>,
    error_level: bool,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => {
            write_the_answer(rt, name, values, 0, value.clone(), line)?;
            if error_level {
                rt.set_the_error_level(Value::Int(0));
            }
            Ok(value)
        }
        Err(failure) => {
            write_the_answer(rt, name, values, 0, Value::Text("".into()), line)?;
            if error_level {
                rt.set_the_error_level(Value::Int(1));
            }
            rt.note(format!(
                "the command {name} did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// The function of the v1 that calls the function of the v2 with the
/// values of the caller.
///
/// The failure of the v2 is the failure of the v1: a function of the v1
/// that cannot answer throws.
fn the_function_of_the_v2(
    rt: &mut Runtime<'_>,
    handler: BuiltinHandler,
    values: &[Value],
    line: u32,
) -> Result<Value, RuntimeError> {
    handler(rt, values, line, 0)
}

// -- the registry ----------------------------------------------------------------

/// `RegRead, OutputVar, KeyName, ValueName`: the value of the registry.
///
/// The `ErrorLevel` is one when the value is not there: measured on the
/// binary of v1.1.37.02.
///
/// Measured on the binary of v1.1.37.02: the `RegRead` of a
/// key that is not there is not something that the v1 says: the text of
/// the caller stays empty, the `ErrorLevel` is one, and the script goes
/// on.
pub(crate) fn reg_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::reg_read(rt, values.get(1..).unwrap_or(&[]), line, 0);
    match answer {
        Ok(value) => {
            write_the_answer(rt, "RegRead", values, 0, value.clone(), line)?;
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(_) => {
            // The v1 says nothing: the value that is not there is not a
            // failure of the script.
            write_the_answer(rt, "RegRead", values, 0, Value::Text("".into()), line)?;
            rt.set_the_error_level(Value::Int(1));
            Ok(Value::Text("".into()))
        }
    }
}

/// `RegWrite, ValueType, KeyName, ValueName, Value`: write into the
/// registry.
///
/// The order of the parameters of the v1 is the value, the type, the key
/// and the name, and the order of the v2 is the type, the key, the name
/// and the value.
pub(crate) fn reg_write(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `RegWrite, ValueType, KeyName, ValueName, Value`.
    // The v2: `RegWrite(Value, ValueType, KeyName, ValueName)`.
    let mut reordered: Vec<Value> = Vec::with_capacity(values.len());
    reordered.push(values.get(4).cloned().unwrap_or(Value::Int(0)));
    reordered.push(values.first().cloned().unwrap_or(Value::Int(0)));
    reordered.push(values.get(1).cloned().unwrap_or(Value::Int(0)));
    reordered.push(values.get(2).cloned().unwrap_or(Value::Int(0)));
    let answer = builtins::reg_write(rt, &reordered, line, 0);
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command RegWrite did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `RegDelete, KeyName, ValueName`: remove a value of the registry, or a
/// key of it.
///
/// The name of the value is not needed to remove the key: the v1 removes
/// the key itself when the name is left out (source `script.cpp:15953-15966`
/// of v1.1.37.02, `RegDelete`: the name of the value is the second
/// parameter, and the key is removed when it is empty).
///
/// The `ErrorLevel` is one when the thing is not there: measured on the
/// binary of v1.1.37.02.
pub(crate) fn reg_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default()
        .trim()
        .to_string();
    let answer = if name.is_empty() {
        // The key itself, and not a value of it.
        let key = builtins::the_key_of(values, 0, "RegDelete", line)?;
        rt.desktop()
            .registry_delete_key(&key)
            .map_err(|reason| builtins::desktop_failure(reason, line))
            .map(|_| Value::Text("".into()))
    } else {
        builtins::reg_delete(rt, values, line, 0)
    };
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command RegDelete did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

// -- the screen ------------------------------------------------------------------

/// `SysGet, OutputVar, N`: a number of the screen.
///
/// The `SysGet` of the v1 has a sub-command, and the `SysGet` of the v2
/// has a number: the sub-command of the v1 is the question about the
/// monitors of the screen, and the v2 has a name for each one of them.
pub(crate) fn sys_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SysGet, OutputVar, SubCommand, Value`.
    // The sub-command is the second parameter, and the number is the
    // third.
    let sub_command = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let number = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let answer = the_answer_of_the_sub_command(rt, values, &sub_command, &number, line);
    the_answer_that_goes_on(rt, "SysGet", values, line, answer, false)
}

/// Write a value into a variable of the script, by its name.
///
/// The name of the variable of the caller is a name of the script, and the
/// scope of it is the one of the script that is running.
fn write_the_variable(rt: &mut Runtime<'_>, name: &str, value: Value) {
    rt.write_reference(
        &Value::Reference(crate::runtime::Reference {
            name: name.to_string(),
            scope: crate::runtime::Scope::Global,
            alias: true,
        }),
        value,
    );
}

/// The name of the variable of the caller that holds one of the four places
/// of a monitor: the name of the caller and the "Left", the "Top", the
/// "Right" or the "Bottom".
fn the_name_of_the_monitor_variable(values: &[Value], index: usize) -> String {
    let base = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let suffix = match index {
        0 => "Left",
        1 => "Top",
        2 => "Right",
        _ => "Bottom",
    };
    format!("{base}{suffix}")
}

/// The answer of the sub-command of the `SysGet` of the v1.
fn the_answer_of_the_sub_command(
    rt: &mut Runtime<'_>,
    values: &[Value],
    sub_command: &str,
    number: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    match sub_command.to_ascii_lowercase().as_str() {
        "" => {
            // The v1 without a sub-command is the metric of the screen.
            let metric = number.trim().parse::<i64>().unwrap_or(0);
            builtins::sys_get(rt, &[Value::Int(metric)], line, 0)
        }
        // The number of a metric of the screen is a sub-command of the v1:
        // the `SysGet, out, 0` is the width of the screen. Measured on the
        // binary of v1.1.37.02: the answer was "3840", and
        // the number of the metric is in the place of the sub-command.
        _ if sub_command.trim().parse::<i64>().is_ok() => {
            let metric = sub_command.trim().parse::<i64>().unwrap_or(0);
            builtins::sys_get(rt, &[Value::Int(metric)], line, 0)
        }
        "monitorcount" => builtins::monitor_get_count(rt, &[], line, 0),
        // The number of the primary monitor of the v1 is one-based: it is
        // always one on a machine with one monitor -- measured on the
        // binary of v1.1.37.02. The number that the v2
        // answers counts from one too, so it is the number itself.
        "monitorprimary" => builtins::monitor_get_primary(rt, &[], line, 0),
        "monitor" | "monitorworkarea" => {
            // The v1 writes the four places into the four variables whose
            // names begin with the name of the caller: the "Mon2" of the
            // "SysGet, Mon2, Monitor, 2" is the "Mon2Left", the "Mon2Top",
            // the "Mon2Right" and the "Mon2Bottom".
            let index = number.trim().parse::<usize>().unwrap_or(1);
            let monitors = rt
                .desktop()
                .monitors()
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            let Some(monitor) = monitors.get(index.saturating_sub(1)) else {
                // The monitor that is not there leaves the four variables
                // of the caller blank.
                return Ok(Value::Text("".into()));
            };
            let places = [
                Value::Int(monitor.left),
                Value::Int(monitor.top),
                Value::Int(monitor.right),
                Value::Int(monitor.bottom),
            ];
            for (index, place) in places.iter().enumerate() {
                let name = the_name_of_the_monitor_variable(values, index);
                write_the_variable(rt, &name, place.clone());
            }
            Ok(Value::Text("".into()))
        }
        "monitorname" => builtins::monitor_get_name(rt, &[Value::Text(number.to_string().into())], line, 0),
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the SysGet is not known."),
            line,
        }),
    }
}

// -- the controls ----------------------------------------------------------------

/// `ControlGetText, OutputVar, Control, WinTitle, ...`: the text of a
/// control of a window.
///
/// The `ErrorLevel` is one when the control is not there: measured on the
/// binary of v1.1.37.02.
pub(crate) fn control_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_get_text(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "ControlGetText", values, line, answer, true)
}

/// `ControlGetFocus, OutputVar, WinTitle, ...`: the control that has the
/// focus.
///
/// The `ErrorLevel` is one when there is no such control: measured on the
/// binary of v1.1.37.02.
pub(crate) fn control_get_focus(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_get_focus(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "ControlGetFocus", values, line, answer, true)
}

/// `ControlGetPos, OutX, OutY, OutWidth, OutHeight, Control, ...`: the place
/// of a control of a window.
///
/// The `ErrorLevel` stays at zero when the control is not there, and the
/// four variables of the caller are empty: measured on the binary of
/// v1.1.37.02.
pub(crate) fn control_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v2 writes into the four references of the caller, and the v1
    // gives it the four names of the variables of the caller as the four
    // references: the shape of the two is the same.
    let answer = builtins::control_get_pos(rt, values, line, 0);
    match answer {
        Ok(value) => Ok(value),
        Err(failure) => {
            for index in 0..4 {
                if values.get(index).is_some() {
                    write_the_answer(
                        rt,
                        "ControlGetPos",
                        values,
                        index,
                        Value::Text("".into()),
                        line,
                    )?;
                }
            }
            rt.note(format!(
                "the command ControlGetPos did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `ControlClick, Control, WinTitle, ...`: click a control of a window.
pub(crate) fn control_click(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_act(rt, values, line, 0);
    the_answer_of_an_act(rt, "ControlClick", answer)
}

/// `ControlFocus, Control, WinTitle, ...`: give the focus to a control of
/// a window.
pub(crate) fn control_focus(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_act(rt, values, line, 1);
    the_answer_of_an_act(rt, "ControlFocus", answer)
}

/// `ControlMove, Control, X, Y, Width, Height, ...`: move a control of a
/// window.
pub(crate) fn control_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_move(rt, values, line, 0);
    the_answer_of_an_act(rt, "ControlMove", answer)
}

/// `ControlSend, Control, Keys, WinTitle, ...`: type into a control of a
/// window.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 writes the
/// control before the keys, and the built-in of the v2 takes the text
/// first: the two are swapped here.
pub(crate) fn control_send(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_send(rt, &the_text_first(values), line, 0);
    the_answer_of_an_act(rt, "ControlSend", answer)
}

/// `ControlSendRaw, Control, Text, WinTitle, ...`: write text into a control
/// of a window.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 writes the text
/// as it is, without the keys of it being simulated: the `{Enter}` of the
/// text is not an enter keystroke.
pub(crate) fn control_send_raw(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_send_text(rt, &the_text_first(values), line, 0);
    the_answer_of_an_act(rt, "ControlSendRaw", answer)
}

/// The values of a command of the v1 with the first two of them swapped.
///
/// The v1 writes the control before the text that goes into it, and the
/// built-ins of the v2 take the text first.
fn the_text_first(values: &[Value]) -> Vec<Value> {
    let mut ordered = values.to_vec();
    if ordered.len() > 1 {
        ordered.swap(0, 1);
    }
    ordered
}

/// `ControlSetText, Control, NewText, WinTitle, ...`: write the text of a
/// control of a window.
pub(crate) fn control_set_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::control_set_text(rt, values, line, 0);
    the_answer_of_an_act(rt, "ControlSetText", answer)
}

/// The answer of an act on a control of a window.
///
/// The `ErrorLevel` is one when the act did not happen.
pub(crate) fn the_answer_of_an_act(
    rt: &mut Runtime<'_>,
    name: &str,
    answer: Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command {name} did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `ControlGet, OutputVar, SubCommand, ...`: a question about a control of
/// a window.
///
/// The `ErrorLevel` is one when the control is not there: measured on the
/// binary of v1.1.37.02.
pub(crate) fn control_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let sub_command = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    // The v1 has a parameter between the sub-command and the control: the
    // `ControlGet, OutputVar, SubCommand, Value, Control, WinTitle, ...`
    // (source `script.cpp:15107-15108` of v1.1.37.02,
    // `ControlGet(ARG2, ARG3, ARG4, ARG5, ARG6, ARG7, ARG8)`). The value
    // is the options of the `List`, and it has to be blank for the other
    // sub-commands (`script.cpp:6948-6950`, "Parameter #3 must be blank in
    // this case"). So the control is the fourth parameter and the window
    // is the fifth.
    //
    // Measured on the binary of v1.1.37.02: the
    // `ControlGet, Style, , , ahk_id %hButton%` of a button of a GUI of
    // the script answers the style of the button, and the
    // `ControlGetText, , ahk_id %hButton%` answers the text of it.
    let control = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let title = values.get(4).map(|value| value.to_text()).unwrap_or_default();
    let answer = the_answer_of_the_control(rt, &sub_command, &control, &title, line);
    the_answer_that_goes_on(rt, "ControlGet", values, line, answer, true)
}

/// The answer of the flag of a control: one for the flag that is set, zero
/// for the one that is not, and the failure of the v2 for the control that
/// is not there.
fn the_answer_of_the_flag(
    flag: crate::desktop::ControlFlag,
    line: u32,
) -> Result<Value, RuntimeError> {
    match flag {
        crate::desktop::ControlFlag::Yes => Ok(Value::Int(1)),
        crate::desktop::ControlFlag::No => Ok(Value::Int(0)),
        crate::desktop::ControlFlag::NoSuchWindow => Err(RuntimeError {
            message: builtins::TARGET_WINDOW.to_string(),
            line,
        }),
        crate::desktop::ControlFlag::NoSuchControl => Err(RuntimeError {
            message: builtins::TARGET_CONTROL.to_string(),
            line,
        }),
    }
}

/// The answer of the sub-command of the `ControlGet` of the v1.
fn the_answer_of_the_control(
    rt: &mut Runtime<'_>,
    sub_command: &str,
    control: &str,
    title: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    match sub_command.to_ascii_lowercase().as_str() {
        "enabled" => {
            let flag = rt
                .desktop()
                .control_enabled(control, title)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            the_answer_of_the_flag(flag, line)
        }
        "visible" => {
            let flag = rt
                .desktop()
                .control_visible(control, title)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            the_answer_of_the_flag(flag, line)
        }
        // The style of a control is a text of eight hexadecimal digits
        // with the `0x` in front: the CHM of v1.1.37.02 says "Style:
        // Retrieves an 8-digit hexadecimal number representing the style
        // of the control", and the binary answers "0x50012000" for a
        // button of a GUI of the script (measured on the binary of
        // v1.1.37.02).
        "style" => rt
            .desktop()
            .control_style(control, title, false)
            .map(|style| Value::Text((format!("0x{:08X}", style as u32)).into()))
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "exstyle" => rt
            .desktop()
            .control_style(control, title, true)
            .map(|style| Value::Text((format!("0x{:08X}", style as u32)).into()))
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "hwnd" => rt
            .desktop()
            .gui_number_named(control)
            .map(Value::Int)
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "list" => {
            let joined = rt
                .desktop()
                .gui_controls()
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            Ok(Value::Text(joined.into()))
        }
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the ControlGet is not known."),
            line,
        }),
    }
}

// -- the groups ------------------------------------------------------------------

/// `GroupAdd, GroupName, WinTitle, ...`: put a window in a group.
///
/// The v1: the window that is not there is not something that the v1 says:
/// the `ErrorLevel` is one and the script goes on.
///
/// Measured on the binary of v1.1.37.02: the `GroupAdd` of
/// a window that does not exist leaves the `ErrorLevel` at one and says
/// nothing.
pub(crate) fn group_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::group_add(rt, values, line, 0);
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) if failure.message == builtins::TARGET_WINDOW => {
            rt.set_the_error_level(Value::Int(1));
            Ok(Value::Text("".into()))
        }
        Err(failure) => {
            rt.note(format!(
                "the command GroupAdd did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `GroupActivate, GroupName, Mode`: activate the windows of a group.
pub(crate) fn group_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::group_activate(rt, values, line, 0);
    the_answer_of_an_act(rt, "GroupActivate", answer)
}

/// `GroupClose, GroupName, Mode`: close the windows of a group.
pub(crate) fn group_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::group_close(rt, values, line, 0);
    the_answer_of_an_act(rt, "GroupClose", answer)
}

/// `GroupDeactivate, GroupName, Mode`: take the focus from the windows of
/// a group.
pub(crate) fn group_deactivate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::group_close(rt, values, line, 0);
    the_answer_of_an_act(rt, "GroupDeactivate", answer)
}

// -- the files -------------------------------------------------------------------

/// `FileCreateShortcut, Target, LinkFile, ...`: make a shortcut.
pub(crate) fn file_create_shortcut(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::file_create_shortcut(rt, values, line, 0);
    the_answer_of_an_act(rt, "FileCreateShortcut", answer)
}

/// `FileGetShortcut, LinkFile, OutTarget, ...`: the pieces of a shortcut.
///
/// The `ErrorLevel` is one when the shortcut is not there: measured on
/// the binary of v1.1.37.02.
pub(crate) fn file_get_shortcut(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::file_get_shortcut(rt, values, line, 0);
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command FileGetShortcut did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `FileGetVersion, OutputVar, Filename`: the version of a program.
pub(crate) fn file_get_version(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::file_get_version(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "FileGetVersion", values, line, answer, false)
}

/// `FileRecycle, FilePattern`: send a file to the trash.
pub(crate) fn file_recycle(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::file_recycle(rt, values, line, 0);
    the_answer_of_an_act(rt, "FileRecycle", answer)
}

/// `FileRecycleEmpty, DriveLetter`: empty the trash.
pub(crate) fn file_recycle_empty(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::file_recycle_empty(rt, values, line, 0);
    the_answer_of_an_act(rt, "FileRecycleEmpty", answer)
}

// -- the sound -------------------------------------------------------------------

/// `SoundGet, OutputVar, ComponentType, ...`: a number of the sound.
pub(crate) fn sound_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SoundGet, OutputVar, ComponentType, ControlType, DeviceNumber`.
    // The component is the second parameter, the control is the third and
    // the device is the fourth.
    let component = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let control = values
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let device = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let answer = rt
        .desktop()
        .sound_control(&component, &control, &device)
        .map(|answer| match answer {
            crate::desktop::SoundControl::Volume(volume) => Value::Float(volume),
            crate::desktop::SoundControl::Switch(on) => {
                Value::Text((if on { "On" } else { "Off" }).into())
            }
        })
        .map_err(|failure| RuntimeError {
            message: failure.the_words_of_the_v1().to_string(),
            line,
        });
    // The `ErrorLevel` of the v1 is the phrase of the failure, and not
    // a number: measured on the binary of v1.1.37.02.
    match answer {
        Ok(value) => {
            write_the_answer(rt, "SoundGet", values, 0, value.clone(), line)?;
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            write_the_answer(rt, "SoundGet", values, 0, Value::Text("".into()), line)?;
            rt.set_the_error_level(Value::Text(failure.message.clone().into()));
            rt.note(format!(
                "the command SoundGet did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `SoundSet, NewSetting, ComponentType, ...`: a number of the sound.
pub(crate) fn sound_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SoundSet, NewSetting, ComponentType, ControlType, DeviceNumber`.
    // The setting is the first parameter, the component is the second, the
    // control is the third and the device is the fourth.
    let setting = values.first().map(|value| value.to_text()).unwrap_or_default();
    let component = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let control = values
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let device = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    // The setting is a number of percent, and the mute of the v1 is the
    // word "On" or the number that is not zero.
    //
    // Measured on the binary of v1.1.37.02: o v1 le o
    // ajuste com o `ATOF`, e por isso a palavra "On" e o zero -- o
    // `SoundSet, On, Master, Mute` deixa o mute desligado, e o
    // `SoundSet, 1, Master, Mute` liga o mute. E o `+` e o `-` na
    // frente do ajuste fazem o ajuste relatifo: o `+` liga o mute que
    // estava desligado, e o mute ligado vira desligado.
    let number = crate::v1::the_number_at_the_start_of_a_text(&setting)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    let answer = rt
        .desktop()
        .sound_control_is_set(&component, &control, number, &device)
        .map_err(|failure| RuntimeError {
            message: failure.the_words_of_the_v1().to_string(),
            line,
        });
    match answer {
        Ok(()) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(Value::Text("".into()))
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Text(failure.message.clone().into()));
            rt.note(format!(
                "the command SoundSet did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `SoundGetWaveVolume, OutputVar, DeviceNumber`: the volume of the wave
/// of a device.
pub(crate) fn sound_get_wave_volume(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SoundGetWaveVolume, OutputVar, DeviceNumber`.
    //
    // On Windows Vista and later the v1 says that the command is the
    // same as the `SoundGet, OutputVar, Wave, Volume`: measured on the
    // binary of v1.1.37.02, the answer of the wave of a
    // machine that has no wave is the empty text and the `ErrorLevel` is
    // "Component Doesn't Support This Control Type".
    let device = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let answer = rt
        .desktop()
        .sound_control("wave", "volume", &device)
        .map(|answer| match answer {
            crate::desktop::SoundControl::Volume(volume) => Value::Float(volume),
            crate::desktop::SoundControl::Switch(on) => {
                Value::Text((if on { "On" } else { "Off" }).into())
            }
        })
        .map_err(|failure| RuntimeError {
            message: failure.the_words_of_the_v1().to_string(),
            line,
        });
    match answer {
        Ok(value) => {
            write_the_answer(
                rt,
                "SoundGetWaveVolume",
                values,
                0,
                value.clone(),
                line,
            )?;
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            write_the_answer(
                rt,
                "SoundGetWaveVolume",
                values,
                0,
                Value::Text("".into()),
                line,
            )?;
            rt.set_the_error_level(Value::Text(failure.message.clone().into()));
            rt.note(format!(
                "the command SoundGetWaveVolume did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `SoundSetWaveVolume, Percent, DeviceNumber`: the volume of the wave of
/// a device.
pub(crate) fn sound_set_wave_volume(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SoundSetWaveVolume, Percent, DeviceNumber`.
    //
    // On Windows Vista and later the v1 says that the command is the
    // same as the `SoundSet, Percent, Wave, Volume`: measured on the
    // binary of v1.1.37.02, the `ErrorLevel` of a machine
    // that has no wave is "Can't Open Specified Mixer".
    let percent = values.first().map(|value| value.to_text()).unwrap_or_default();
    let device = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let number = crate::v1::the_number_at_the_start_of_a_text(&percent)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    let answer = rt
        .desktop()
        .sound_control_is_set("wave", "volume", number, &device)
        .map_err(|failure| RuntimeError {
            message: failure.the_words_of_the_v1().to_string(),
            line,
        });
    match answer {
        Ok(()) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(Value::Text("".into()))
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Text(failure.message.clone().into()));
            rt.note(format!(
                "the command SoundSetWaveVolume did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

// -- the drives -----------------------------------------------------------------

/// `DriveGet, OutputVar, SubCommand, Value`: a number of a drive.
pub(crate) fn drive_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `DriveGet, OutputVar, SubCommand, Value`.
    let sub_command = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let drive = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        "label" => builtins::drive_get_label(rt, &[Value::Text(drive.into())], line, 0),
        "serial" => builtins::drive_get_serial(rt, &[Value::Text(drive.into())], line, 0),
        "type" => builtins::drive_get_type(rt, &[Value::Text(drive.into())], line, 0),
        "capacity" => builtins::drive_get_capacity(rt, &[Value::Text(drive.into())], line, 0),
        "filesystem" => builtins::drive_get_filesystem(rt, &[Value::Text(drive.into())], line, 0),
        "list" => {
            let list = rt
                .desktop()
                .drive_list()
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            Ok(Value::Text(list.join("\n").into()))
        }
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the DriveGet is not known."),
            line,
        }),
    };
    the_answer_that_goes_on(rt, "DriveGet", values, line, answer, false)
}

/// `DriveSpaceFree, OutputVar, Path`: the space of a drive that is free.
pub(crate) fn drive_space_free(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::drive_get_space_free(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "DriveSpaceFree", values, line, answer, false)
}

// -- the processes ---------------------------------------------------------------

/// `Process, SubCommand, PIDOrName, Value`: a question about a process of
/// the machine.
pub(crate) fn process(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Process, SubCommand, PIDOrName, Value`.
    let sub_command = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let which = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        "exist" => rt
            .desktop()
            .process_exist(Some(&which))
            .map(|pid| {
                if pid == 0 {
                    Value::Int(0)
                } else {
                    Value::Int(pid as i64)
                }
            })
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "close" => rt
            .desktop()
            .process_close(&which)
            .map(|pid| Value::Int(pid as i64))
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "priority" => {
            let level = values.get(2).map(|value| value.to_text()).unwrap_or_default();
            rt.desktop()
                .process_set_priority(&level, Some(&which))
                .map(|pid| match pid {
                    Some(pid) => Value::Int(pid as i64),
                    None => Value::Int(0),
                })
                .map_err(|reason| builtins::desktop_failure(reason, line))
        }
        "wait" => {
            let timeout = values.get(2).map(|value| value.to_text()).unwrap_or_default();
            let timeout = timeout.trim().parse::<f64>().ok();
            rt.desktop()
                .process_wait(&which, timeout)
                .map(|pid| Value::Int(pid as i64))
                .map_err(|reason| builtins::desktop_failure(reason, line))
        }
        "waitclose" => {
            let timeout = values.get(2).map(|value| value.to_text()).unwrap_or_default();
            let timeout = timeout.trim().parse::<f64>().ok();
            rt.desktop()
                .process_wait_close(&which, timeout)
                .map(|pid| Value::Int(pid as i64))
                .map_err(|reason| builtins::desktop_failure(reason, line))
        }
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the Process is not known."),
            line,
        }),
    };
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command Process did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

// -- the COM --------------------------------------------------------------------

/// `ComObjActive(CLSID)`: the COM object of a class.
///
/// Measured on the binary of v1.1.37.02: the `ComObjActive` of a class
/// that the machine does not have answers the empty text, and it does not
/// throw.
pub(crate) fn com_obj_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match builtins::com_obj_active(rt, values, line, 0) {
        Ok(value) => Ok(value),
        Err(_) => Ok(Value::Text("".into())),
    }
}

/// `ComObjConnect(ComObject, PrefixOrSink)`: the events of a COM object.
pub(crate) fn com_obj_connect(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_connect, values, line)
}

/// `ComObjFlags(ComObject, NewFlags, Mask)`: the flags of a COM object.
pub(crate) fn com_obj_flags(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_flags, values, line)
}

/// `ComObjGet(Name)`: the COM object of a name.
pub(crate) fn com_obj_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_get, values, line)
}

/// `ComObjQuery(ComObject, SID, IID)`: the interface of a COM object.
pub(crate) fn com_obj_query(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_query, values, line)
}

/// `ComObjType(ComObject, InfoType)`: what is known of a COM object.
pub(crate) fn com_obj_type(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_type, values, line)
}

/// `ComObjValue(ComObject)`: the value of a COM object.
pub(crate) fn com_obj_value(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_function_of_the_v2(rt, builtins::com_obj_value, values, line)
}

// -- the text --------------------------------------------------------------------

/// `StrGet(Source, Length, Encoding)`: the text at an address.
///
/// The v1 reads the memory of the script and the memory of the machine,
/// and the encoding of the machine is the `UTF-16` of the build of
/// Unicode: the handler of the v1 is its own.
pub(crate) fn str_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    super::memory::str_get(rt, values, line, _variant)
}

/// `StrPut(String, Target, Length, Encoding)`: the writing of a text in a
/// memory.
///
/// The v1 writes in the memory of the script and in the memory of the
/// machine, and the handler of the v1 is its own.
pub(crate) fn str_put(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    super::memory::str_put(rt, values, line, _variant)
}

// -- the script ------------------------------------------------------------------

/// `BlockInput, OnOff`: the input of the machine, blocked.
pub(crate) fn block_input(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::block_input(rt, values, line, 0);
    the_answer_of_an_act(rt, "BlockInput", answer)
}

/// `Edit`: open the script in an editor.
pub(crate) fn edit(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::edit(rt, values, line, 0);
    the_answer_of_an_act(rt, "Edit", answer)
}

/// `URLDownloadToFile, Url, File`: fetch a file of the network.
///
/// Measured on the binary of v1.1.37.02: the failure of the download is
/// the `ErrorLevel` and not the end of the script: the
/// `URLDownloadToFile, a, b` of a url that is not one goes on, and the
/// `ErrorLevel` is one.
pub(crate) fn url_download_to_file(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::download(rt, values, line, 0);
    the_answer_of_an_act(rt, "URLDownloadToFile", answer)
}

/// `KeyHistory`: the keys that were pressed.
/// `KeyWait, key`: the v1 checks the name of the key before it waits.
///
/// Measured on the binary of v1.1.37.02: a name that is not a key is
/// "Parameter #1 invalid." with the name under it. The runtime that
/// cannot wait refuses only after the name is known to be a key.
pub(crate) fn key_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values.first().map(|v| v.to_text()).unwrap_or_default();
    if crate::keys::resolve_name(&name).is_err() {
        return Err(RuntimeError {
            message: format!("Parameter #1 invalid.\n     Specifically: {name}"),
            line,
        });
    }
    builtins::key_wait(rt, values, line, variant)
}

pub(crate) fn key_history(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::hotkeys::key_history(rt, values, line, 0);
    the_answer_of_an_act(rt, "KeyHistory", answer)
}

/// `ListHotkeys`: the hotkeys of the script.
pub(crate) fn list_hotkeys(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::hotkeys::list_hotkeys(rt, values, line, 0);
    the_answer_of_an_act(rt, "ListHotkeys", answer)
}

/// `ListLines, OnOff`: the lines of the script, listed.
pub(crate) fn list_lines(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::list_lines(rt, values, line, 0);
    the_answer_of_an_act(rt, "ListLines", answer)
}

/// `ListVars`: the variables of the script, listed.
pub(crate) fn list_vars(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::list_vars(rt, values, line, 0);
    the_answer_of_an_act(rt, "ListVars", answer)
}

/// `OnExit [, Label, AddRemove]`: the subroutine that runs when the
/// script ends. The v1 names a label or a function; the `-1` takes the
/// registration away.
pub(crate) fn on_exit_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let label = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let label = label.trim().to_string();
    if label.is_empty() {
        return Ok(Value::Text("".into()));
    }
    let add_remove = values
        .get(1)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i64;
    if add_remove == -1 {
        rt.remove_exit_callback(&label);
    } else {
        rt.set_exit_callback(&label, line);
    }
    Ok(Value::Text("".into()))
}

/// `OnMessage(MsgNumber, Callback, MaxThreads)`: a handler for a message.
///
/// The v1 keeps two kinds of handler for a message: the **legacy** one,
/// which the script registers by name, and the ones that are objects. The
/// legacy one is at most one for each number of message, and the name it
/// keeps is the name of the function of the script -- not the spelling that
/// the script wrote.
///
/// Measured on the binary of v1.1.37.02:
///
/// - `OnMessage(0x321)` answers the name of the legacy monitor of `0x321`,
///   and the empty text when there is none.
/// - `OnMessage(0x321, "MyHandler")` answers the name of the monitor that
///   was there before, and the name of the new one when there was none.
///   The name it answers is the name of the function: `OnMessage(0x321,
///   "myhandler")` answers `MyHandler`.
/// - `OnMessage(0x321, "")` takes the monitor away and answers the name it
///   had.
/// - `OnMessage(0x321, "MyHandler", 0)` takes the monitor away as well, and
///   answers the name it had.
/// - A callback that is not an object and not the name of a function of the
///   script answers the empty text, and the failure is silent: the v1 does
///   not raise an error for a name.
/// - A function that needs more than four parameters is refused: the empty
///   text, and no monitor.
/// - A callback that is an object answers the empty text, and the object is
///   registered as a monitor of its own.
pub(crate) fn on_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The number of the message is read the strict way: the v1 keeps the
    // spelling of a hexadecimal constant -- `OnMessage(0x324)` holds the
    // text "0x324" -- and the number of it is the hexadecimal one.
    let number = values
        .first()
        .map(|value| value.to_number_strict().unwrap_or(0.0) as i64)
        .unwrap_or(0);
    // The query: the name of the legacy monitor of the message.
    let Some(callback) = values.get(1) else {
        return Ok(Value::Text(rt.the_legacy_message_handler(number).unwrap_or_default().into()));
    };
    match callback {
        Value::Text(name) => {
            let max_threads = values
                .get(2)
                .map(|value| value.to_number_strict().unwrap_or(1.0) as i64)
                .unwrap_or(1);
            // The empty name takes the monitor away, and so does a
            // `MaxThreads` of zero: the v1 answers the name that the
            // monitor had, and the empty text when there was none.
            if name.is_empty() || max_threads == 0 {
                let previous = rt
                    .forget_a_legacy_message_handler(number)
                    .unwrap_or_default();
                return Ok(Value::Text(previous.into()));
            }
            // The name of a function of the script: the v1 refuses the
            // others, and the answer of the refusal is the empty text.
            let Some((canonical, required)) = rt.the_function_named(name) else {
                return Ok(Value::Text("".into()));
            };
            // The callback of a message takes four parameters: the v1
            // refuses a function that needs more than that.
            if required > 4 {
                return Ok(Value::Text("".into()));
            }
            let previous = rt
                .the_legacy_message_handler(number)
                .map(str::to_string)
                .unwrap_or_else(|| canonical.to_string());
            rt.remember_a_legacy_message_handler(number, canonical.to_string());
            // A message monitor keeps the script alive after its statements
            // end, the way the original AutoHotkey does.
            rt.the_script_asked_to_stay(true);
            // The window of the script watches the message: when it arrives,
            // the desktop queues it and the runtime calls the handler
            // between the statements.
            let _ = rt.desktop().watch_a_message(number);
            Ok(Value::Text(previous.into()))
        }
        Value::Object(_) => {
            // The object is a monitor of its own, and the answer of the v1
            // is the empty text.
            let answer = builtins::on_message(rt, values, line, 0);
            the_answer_of_an_act(rt, "OnMessage", answer)
        }
        // A callback that is neither an object nor a name is a failure of
        // the v1, and the failure is silent.
        _ => Ok(Value::Text("".into())),
    }
}

/// `OutputDebug, Text`: a line in the log of the debugger.
pub(crate) fn output_debug(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::output_debug(rt, values, line, 0);
    the_answer_of_an_act(rt, "OutputDebug", answer)
}

/// `Pause, OnOffToggle`: the script, paused.
pub(crate) fn pause(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::pause(rt, values, line, 0);
    the_answer_of_an_act(rt, "Pause", answer)
}

/// `PostMessage, Msg, wParam, lParam, Control, WinTitle`: a message to a
/// window.
///
/// The v1 writes the control before the title, and the built-in reads
/// the title in the fourth place: the parameters are put in the order
/// of the built-in. A control that is a number is the handle of the
/// target itself; a control that is a name is left out for now, and the
/// message goes to the window.
pub(crate) fn post_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let ordered = the_order_of_a_message(values);
    let answer = builtins::post_message(rt, &ordered, line, 0);
    the_answer_of_an_act(rt, "PostMessage", answer)
}

/// `SendMessage, Msg, wParam, lParam, Control, WinTitle, ...`: a message
/// to a window, and the answer of it.
///
/// Measured on the binary of v1.1.37.02: the `ErrorLevel` carries the
/// answer of the message, and the text "FAIL" when the target is not
/// there.
pub(crate) fn send_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let ordered = the_order_of_a_message(values);
    match builtins::send_message(rt, &ordered, line, 0) {
        Ok(answer) => {
            rt.set_the_error_level(answer.clone());
            Ok(answer)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Text("FAIL".into()));
            rt.note(format!(
                "the command SendMessage did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// The parameters of a message command in the order the built-in reads
/// them: the message, its two parameters, and the target. The control of
/// the v1 is the fourth parameter and the title the fifth; a control that
/// is a number is the handle of the target.
fn the_order_of_a_message(values: &[Value]) -> Vec<Value> {
    let mut ordered: Vec<Value> = values.iter().take(3).cloned().collect();
    let control = values.get(3);
    let target_is_the_control = matches!(control, Some(Value::Int(_) | Value::Float(_)));
    if target_is_the_control {
        ordered.push(control.expect("the control is there").clone());
    } else if let Some(title) = values.get(4) {
        ordered.push(title.clone());
    }
    ordered
}

/// `Reload`: the script, run again.
pub(crate) fn reload(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::reload(rt, values, line, 0);
    the_answer_of_an_act(rt, "Reload", answer)
}

/// `RunAs, User, Password, Domain`: the script, run as another user.
pub(crate) fn run_as(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::run_as(rt, values, line, 0);
    the_answer_of_an_act(rt, "RunAs", answer)
}

/// `Shutdown, Flag`: the machine, shut down.
pub(crate) fn shutdown(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::shutdown(rt, values, line, 0);
    the_answer_of_an_act(rt, "Shutdown", answer)
}

/// `Suspend, OnOffToggle`: the script, suspended.
pub(crate) fn suspend(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::suspend(rt, values, line, 0);
    the_answer_of_an_act(rt, "Suspend", answer)
}

/// `Thread, SubCommand, Value1, Value2`: the thread of the script.
pub(crate) fn thread(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Thread, SubCommand, Value1, Value2`.
    // The v2: `Thread(Priority)`.
    let sub_command = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        // The three settings of the thread are the ones of the v2: the
        // priority, the interrupt (the milliseconds and the lines) and
        // the NoTimers. The sub-command and its values go as they are.
        "priority" | "interrupt" | "notimers" => {
            builtins::thread_command(rt, values, line, 0)
        }
        _ => Err(RuntimeError {
            message: "Parameter #1 invalid.".to_string(),
            line,
        }),
    };
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command Thread did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `ToolTip, Text, X, Y, WhichToolTip`: the tip of the mouse.
pub(crate) fn tool_tip(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::tooltip(rt, values, line, 0);
    the_answer_of_an_act(rt, "ToolTip", answer)
}

/// `TrayTip, Title, Text, Timeout, Options`: the tip of the tray.
pub(crate) fn tray_tip(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = crate::dialogs::tray_tip(rt, values, line, 0);
    the_answer_of_an_act(rt, "TrayTip", answer)
}

/// `StatusBarGetText, OutputVar, Part#, ...`: the text of a status bar.
pub(crate) fn status_bar_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::status_bar_get_text(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "StatusBarGetText", values, line, answer, false)
}

/// `StatusBarWait, BarText, Timeout, Part#, ...`: wait for a status bar.
pub(crate) fn status_bar_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::status_bar_wait(rt, values, line, 0);
    the_answer_of_an_act(rt, "StatusBarWait", answer)
}

/// `SetCapsLockState, On`: the state of the caps lock of the keyboard.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 takes the word
/// "On" and the word "Off" -- and the numbers one and zero -- and the word
/// that is not one of them is refused with "Parameter #1 invalid." and the
/// word under it.
pub(crate) fn set_caps_lock_state_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_the_state_of_a_lock_of_the_v1(rt, values, line, "CapsLock")
}

/// `SetNumLockState, On`: the state of the num lock of the keyboard.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 takes the word
/// "On" and the word "Off" -- and the numbers one and zero -- and the word
/// that is not one of them is refused with "Parameter #1 invalid." and the
/// word under it.
pub(crate) fn set_num_lock_state_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_the_state_of_a_lock_of_the_v1(rt, values, line, "NumLock")
}

/// `SetScrollLockState, On`: the state of the scroll lock of the keyboard.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 takes the word
/// "On" and the word "Off" -- and the numbers one and zero -- and the word
/// that is not one of them is refused with "Parameter #1 invalid." and the
/// word under it.
pub(crate) fn set_scroll_lock_state_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_the_state_of_a_lock_of_the_v1(rt, values, line, "ScrollLock")
}

/// The state of a lock key of the keyboard, as the command of the v1 asks.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the machine that cannot
/// change the state of the key does not fail the script: the `ErrorLevel` is
/// one and the script goes on.
fn set_the_state_of_a_lock_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    lock: &str,
) -> Result<Value, RuntimeError> {
    let word = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    // Measured on the binary of v1.1.37.02 (source/script2.cpp:10869,
    // `SetToggleState`): the v1 does nothing when the state is not
    // written -- the lock is not touched and the command goes on -- and
    // the words "AlwaysOn" and "AlwaysOff" turn the lock on and off.
    let on = match word.trim().to_lowercase().as_str() {
        "" => {
            return super::the_error_level_of_a_command_that_goes_on(rt, Ok(Value::Int(0)));
        }
        "on" | "1" | "alwayson" => true,
        "off" | "0" | "alwaysoff" => false,
        _ => {
            return Err(RuntimeError {
                message: format!("Parameter #1 invalid.\n     Specifically: {word}"),
                line,
            })
        }
    };
    let answer = rt.desktop().set_lock(lock, on).map_err(|reason| RuntimeError {
        message: reason,
        line,
    }).map(|_| Value::Int(0));
    super::the_error_level_of_a_command_that_goes_on(rt, answer)
}

