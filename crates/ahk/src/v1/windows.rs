//! The window family of the v1, against the desktop of the machine.
//!
//! The v1 writes the answer of a command into the variable of the caller,
//! and the v2 answers with the value. The command of the v1 is therefore
//! the command of the v2 with the values of the caller and the answer
//! written into the first parameter.
//!
//! The failures of the v1 are not the failures of the v2. Measured on the
//! binary of v1.1.37.02:
//!
//! - the command that does not find its window is not a failure of the
//!   script: the variable of the caller is empty, the `ErrorLevel` is one
//!   when the command says so, and the script goes on;
//! - the `WinGetTitle` and the `WinGetClass` of a window that is not there
//!   leave the `ErrorLevel` at zero, and the `WinGetText` leaves it at one;
//! - the `WinGetPos` of a window that is not there leaves the four
//!   variables of the caller empty, and the `ErrorLevel` at zero.
//!
//! The names of the v1 that the v2 does not have are here: the `WinGet`
//! and the `WinSet` are families of sub-commands, and the `WinGetActive`
//! pair answers about the window that has the focus.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::builtins::{self, BuiltinHandler};
use crate::runtime::{Runtime, RuntimeError, Value};

use super::write_the_answer;

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
                "the command {name} did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `WinGetText, OutputVar, WinTitle, ...`: the text of a window.
///
/// The `ErrorLevel` is one when the window is not there: measured on the
/// binary of v1.1.37.02.
pub(crate) fn win_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::win_get_text(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "WinGetText", values, line, answer, true)
}

/// `WinGetClass, OutputVar, WinTitle, ...`: the class of a window.
///
/// The `ErrorLevel` stays at zero when the window is not there: measured
/// on the binary of v1.1.37.02.
pub(crate) fn win_get_class(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::win_get_class(rt, values.get(1..).unwrap_or(&[]), line, 0);
    the_answer_that_goes_on(rt, "WinGetClass", values, line, answer, false)
}

/// `WinGetPos, OutX, OutY, OutWidth, OutHeight, WinTitle, ...`: the place of
/// a window.
///
/// The four variables of the caller are empty when the window is not
/// there, and the `ErrorLevel` stays at zero: measured on the binary of
/// v1.1.37.02.
pub(crate) fn win_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v2 writes into the four references of the caller, and the v1
    // gives it the four names of the variables of the caller as the four
    // references: the shape of the two is the same.
    let answer = builtins::win_get_pos(rt, values, line, 0);
    match answer {
        Ok(value) => Ok(value),
        Err(failure) => {
            // The four variables of the caller are empty, and not the
            // numbers they had before.
            for index in 0..4 {
                if values.get(index).is_some() {
                    write_the_answer(rt, "WinGetPos", values, index, Value::Text("".into()), line)?;
                }
            }
            rt.note(format!(
                "the command WinGetPos did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// `WinGetActiveTitle, OutputVar`: the title of the window that has the
/// focus.
///
/// Measured on the binary of v1.1.37.02: the answer is the
/// title of the window that has the focus, and the `ErrorLevel` stays at
/// zero.
pub(crate) fn win_get_active_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = rt
        .desktop()
        .active_window_title()
        .map(|title| title.unwrap_or_default())
        .map(Value::text)
        .map_err(|reason| builtins::desktop_failure(reason, line));
    the_answer_that_goes_on(rt, "WinGetActiveTitle", values, line, answer, false)
}

/// `WinGetActiveStats, OutTitle, OutWidth, OutHeight, OutX, OutY`: the
/// window that has the focus.
///
/// Measured on the binary of v1.1.37.02: the title is the
/// title of the window that has the focus, the width and the height are
/// the size of it, and the two last numbers are where its upper-left
/// corner is.
pub(crate) fn win_get_active_stats(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let title = rt
        .desktop()
        .active_window_title()
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    let Some(title) = title else {
        // The window that has the focus is not there: the five variables
        // of the caller stay as they were.
        return Ok(Value::Text("".into()));
    };
    let geometry = rt
        .desktop()
        .window_geometry(&title, crate::settings::title_match_mode())
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    let Some(geometry) = geometry else {
        return Ok(Value::Text("".into()));
    };
    // The order of the v1 is the title, the width, the height, the x and
    // the y -- and the `WinGetPos` of the v2 is the x, the y, the width
    // and the height.
    let places = [
        Value::Text(title.into()),
        Value::Int(geometry.width),
        Value::Int(geometry.height),
        Value::Int(geometry.x),
        Value::Int(geometry.y),
    ];
    for (index, place) in places.iter().enumerate() {
        if values.get(index).is_some() {
            write_the_answer(rt, "WinGetActiveStats", values, index, place.clone(), line)?;
        }
    }
    Ok(Value::Text("".into()))
}

/// `WinExist(WinTitle, ...)`: the identifier of the window that is there.
///
/// The v1 answers the identifier of the window as a hexadecimal number
/// with the "0x" in front of it, and it answers the empty text -- the
/// "0x0" -- when there is no window. Measured on the binary of
/// v1.1.37.02: the `WinExist` of a window that is not
/// there answers "0x0", and the answer is the text of the identifier and
/// not a number.
///
/// The title that is left out is the last window that a script found, and
/// a window that is found becomes the last found window: measured on the
/// binary of v1.1.37.02, the `WinGet, out, IDLast` that
/// follows a `WinExist` answers the window that it found.
pub(crate) fn win_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let title = values.first().map(|value| value.to_text()).unwrap_or_default();
    // The title that is left out is the last window that a script found.
    if title.is_empty() {
        return Ok(match rt.the_last_window_of_the_v1() {
            Some(id) => Value::Text(the_identifier_of_the_v1(id).into()),
            None => Value::Text("0x0".into()),
        });
    }
    // The title `A` alone is the window that has the focus: the v1 and the
    // v2 ask for it that way. Measured on the binary of v1.1.37.02: the
    // `WinExist("A")` answers the handle of that window (`0x2080564`).
    if crate::desktop::criteria::the_title_is_the_active_window(&title) {
        return Ok(match the_window_with_the_focus(rt, line)? {
            Some(window) => {
                // The window that is found is the last found window.
                rt.remember_the_last_window_of_the_v1(&window.id.to_string());
                Value::Text(the_identifier_of_the_v1(&window.id.to_string()).into())
            }
            None => Value::Text("0x0".into()),
        });
    }
    let mode = crate::settings::title_match_mode();
    let windows = rt
        .desktop()
        .matching_windows(&title, mode)
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    match windows.first() {
        Some(window) => {
            // The window that is found is the last found window.
            rt.remember_the_last_window_of_the_v1(&window.id);
            Ok(Value::Text(the_identifier_of_the_v1(&window.id).into()))
        }
        // The window that is not there leaves the last found window
        // where it was: measured on the binary of v1.1.37.02.
        None => Ok(Value::Text("0x0".into())),
    }
}
/// `WinGet, OutputVar, SubCommand, WinTitle, ...`: a question about the
/// windows of the machine.
///
/// The sub-command names the question, and the v2 has a name for each one
/// of them: the `ID` is the `WinGetID`, the `Count` is the `WinGetCount`,
/// the `List` is the `WinGetList`, and so on.
pub(crate) fn win_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let sub_command = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    // The target of the window of the v1 begins in the third place: the
    // title, the text of the window and the two exclusions. Measured on
    // the binary of v1.1.37.02: the `WinGetTitle` of a
    // window by the text of one text element of it finds the window, and
    // the `WinGetTitle` with the exclusion of its title finds no window
    // at all.
    let wanted = crate::desktop::TheWindowWanted::of(values, 2);
    // The `List` of the v1 is a pseudo-array: the variable of the caller
    // has the number of the windows, and the variables whose name begins
    // with it have the identifiers of them. It is the one sub-command
    // that writes more than one variable.
    if sub_command.eq_ignore_ascii_case("list") {
        return the_list_of_the_windows(rt, values, &wanted, line);
    }
    let answer = the_answer_of_the_sub_command(rt, &sub_command, &wanted, line);
    the_answer_that_goes_on(rt, "WinGet", values, line, answer, false)
}

/// `WinGet, OutputVar, List, WinTitle, ...`: the identifiers of the
/// windows that match.
///
/// The v1 writes the number of the windows in the variable of the caller
/// and the identifier of each one of them in the variables whose name
/// begins with the name of that variable: measured on the binary of
/// v1.1.37.02, the `WinGet, out, List` of two windows
/// leaves `out` at 2, `out1` at the identifier of the first one and
/// `out2` at the identifier of the second one.
fn the_list_of_the_windows(
    rt: &mut Runtime<'_>,
    values: &[Value],
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<Value, RuntimeError> {
    let windows = the_windows_that_answer(rt, wanted, line)?;
    // The number of the windows goes into the variable of the caller.
    let count = windows.len() as i64;
    write_the_answer(rt, "WinGet", values, 0, Value::Int(count), line)?;
    // The identifier of each one of them goes into the variable whose
    // name begins with the name of the variable of the caller.
    let Some(name) = the_name_of_the_output(rt, values) else {
        return Ok(Value::Int(count));
    };
    for (index, window) in windows.iter().enumerate() {
        let identifier = the_identifier_of_the_v1(&window.id.to_string());
        rt.assign_the_variable_of_the_pseudo_array(
            &format!("{name}{}", index + 1),
            Value::Text(identifier.into()),
        );
    }
    Ok(Value::Int(count))
}

/// As janelas que respondem ao alvo, na listagem do desktop.
///
/// A listagem e a do desktop, e o filtro e o do runtime: e o mesmo
/// para os dois dialetos.
fn the_windows_that_answer(
    rt: &mut Runtime<'_>,
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<Vec<crate::desktop::WindowInfo>, RuntimeError> {
    let windows = rt
        .desktop()
        .windows()
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(builtins::the_windows_of_the_listing(rt, &windows, wanted))
}


/// The identifier of the window that the target names.
///
/// The title that is left out is the last window that a script found, and
/// the window that is not there is no window at all: the v1 answers the
/// empty text when there is none.
fn the_id_of_the_target(
    rt: &mut Runtime<'_>,
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<Option<String>, RuntimeError> {
    if wanted.title.is_empty() {
        return Ok(rt.the_last_window_of_the_v1().map(|id| id.to_string()));
    }
    let windows = the_windows_that_answer(rt, wanted, line)?;
    Ok(windows.first().map(|window| window.id.to_string()))
}

/// The name of the variable of the caller, if the caller gave one.
///
/// The name is the one of the reference of the first parameter, and it is
/// the beginning of the names of the variables of a pseudo-array.
fn the_name_of_the_output(rt: &mut Runtime<'_>, values: &[Value]) -> Option<String> {
    let Some(Value::Reference(reference)) = values.first() else {
        return None;
    };
    let name = reference.name.clone();
    if name.is_empty() {
        None
    } else {
        let _ = rt;
        Some(name)
    }
}

/// The identifier of a window, as the v1 writes it.
///
/// The v1 writes the identifier of a window as a hexadecimal number with
/// the "0x" in front of it: measured on the binary of v1.1.37.02 the `WinGet, out, ID` of a window of the machine answered
/// "0x6530f1e".
fn the_identifier_of_the_v1(id: &str) -> String {
    match id.parse::<u64>() {
        Ok(number) => format!("0x{number:x}"),
        // The identifier of a window of a compositor is a token, and there
        // is no hexadecimal number in it.
        Err(_) => id.to_string(),
    }
}

/// The answer of the sub-command of the `WinGet`.
///
/// The name of the sub-command is the name of the question, and the v2
/// has a built-in for each one of them.
///
/// The title that is left out is the last window that a script found: the
/// v1 keeps one, and it is empty at the start of a script. Measured on the
/// binary of v1.1.37.02: the `WinGet, out, ID` of a script
/// that has found no window answers the empty text.
///
/// The title that is given is matched against the windows of the machine,
/// and the `IDLast` of it is the last of them: measured on the binary of
/// v1.1.37.02, the `WinGet, out, IDLast` of a title with
/// many matches answers the identifier of the bottommost one, and the
/// `ID` of the same title answers the one of the topmost.
fn the_answer_of_the_sub_command(
    rt: &mut Runtime<'_>,
    sub_command: &str,
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<Value, RuntimeError> {
    let mode = crate::settings::title_match_mode();
    let title = &wanted.title;
    // The title that is left out is the last window that a script found.
    // The `Count` and the `List` are the two sub-commands that do not use
    // it: they ask about every window of the machine.
    let uses_the_last_window = !title.is_empty();
    match sub_command.to_ascii_lowercase().as_str() {
        "id" => {
            if uses_the_last_window {
                let windows = the_windows_that_answer(rt, wanted, line)?;
                return match windows.first() {
                    Some(window) => Ok(Value::Text(the_identifier_of_the_v1(&window.id.to_string()).into())),
                    None => Err(RuntimeError {
                        message: builtins::TARGET_WINDOW.to_string(),
                        line,
                    }),
                };
            }
            match rt.the_last_window_of_the_v1() {
                Some(id) => Ok(Value::Text(the_identifier_of_the_v1(id).into())),
                None => Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                }),
            }
        }
        "idlast" => {
            if uses_the_last_window {
                let windows = the_windows_that_answer(rt, wanted, line)?;
                return match windows.last() {
                    Some(window) => Ok(Value::Text(the_identifier_of_the_v1(&window.id.to_string()).into())),
                    None => Err(RuntimeError {
                        message: builtins::TARGET_WINDOW.to_string(),
                        line,
                    }),
                };
            }
            match rt.the_last_window_of_the_v1() {
                Some(id) => Ok(Value::Text(the_identifier_of_the_v1(id).into())),
                None => Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                }),
            }
        }
        "count" => {
            let windows = the_windows_that_answer(rt, wanted, line)?;
            Ok(Value::Int(windows.len() as i64))
        }
        "minmax" => {
            let state = rt
                .desktop()
                .window_min_max(title, mode)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            match state {
                Some(state) => Ok(Value::Int(state)),
                None => Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                }),
            }
        }
        "pid" => {
            let pid = rt
                .desktop()
                .window_pid(title)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            match pid {
                Some(pid) => Ok(Value::Int(pid)),
                None => Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                }),
            }
        }
        "processname" | "processpath" => {
            let pid = rt
                .desktop()
                .window_pid(title)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            let Some(pid) = pid else {
                return Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                });
            };
            let name = if sub_command.eq_ignore_ascii_case("processname") {
                rt.desktop().process_name(Some(&pid.to_string()))
            } else {
                rt.desktop().process_path(Some(&pid.to_string()))
            };
            match name
                .map_err(|reason| builtins::desktop_failure(reason, line))?
            {
                Some(name) => Ok(Value::Text(name.into())),
                None => Err(RuntimeError {
                    message: builtins::TARGET_WINDOW.to_string(),
                    line,
                }),
            }
        }
        "transparent" => rt
            .desktop()
            .window_trans_color(title)
            .map(Value::text)
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "transcolor" => rt
            .desktop()
            .window_trans_color(title)
            .map(Value::text)
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "style" => rt
            .desktop()
            .window_style(title, false)
            .map(Value::Int)
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        "exstyle" => rt
            .desktop()
            .window_style(title, true)
            .map(Value::Int)
            .map_err(|reason| builtins::desktop_failure(reason, line)),
        // The controls of a window are the ones of the window that the
        // target names, and not the ones of the window of the script:
        // measured on the binary of v1.1.37.02, the `WinGet, out,
        // ControlList, <title>` of a window that is not there answers the
        // empty text, and the sub-command `ControlListHwnd` answers the
        // identifiers of the controls, and not their names.
        "controllist" | "controllisthwnd" => {
            let handles = sub_command.eq_ignore_ascii_case("controllisthwnd");
            let Some(id) = the_id_of_the_target(rt, wanted, line)? else {
                return Ok(Value::Text("".into()));
            };
            let list = rt
                .desktop()
                .window_controls(&id, handles)
                .map_err(|reason| builtins::desktop_failure(reason, line))?;
            Ok(Value::Text(list.unwrap_or_default().into()))
        }
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the WinGet is not known."),
            line,
        }),
    }
}

/// `WinWait, WinTitle, WinText, Timeout, ExcludeTitle, ExcludeText`: wait
/// for a window.
///
/// The order of the parameters of the v1 is the title, the text of the
/// window, the timeout and the two exclusions, and the order of the v2 is
/// the title, the text of the window, the timeout and the two exclusions
/// as well -- the two dialects put the timeout in the third place.
/// Measured on the binary of v1.1.37.02: the third
/// parameter is the timeout, and the two exclusions are read and not
/// refused.
///
/// The `ErrorLevel` is one when the wait ran out and zero when it found
/// a window, and a window that is found becomes the last found window:
/// measured on the binary of v1.1.37.02.
pub(crate) fn win_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::win_wait(rt, &the_values_of_a_wait(values), line, 0);
    the_wait_that_goes_on(rt, answer, true)
}

/// `WinWaitActive, WinTitle, ...`: wait for a window to have the focus.
pub(crate) fn win_wait_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let answer = builtins::win_wait_active(rt, &the_values_of_a_wait(values), line, 0);
    the_wait_that_goes_on(rt, answer, true)
}

/// `WinWaitClose, WinTitle, ...`: wait for a window to be gone.
///
/// The wait for the close does not change the last found window: the
/// window it waits for is gone. The window that the title names becomes
/// the last found window, as it does in the v1: the source of it
/// (`script2.cpp:2277-2283`) asks the `WinExist` for it, and the
/// `WinExist` puts the window that it finds there.
pub(crate) fn win_wait_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let values = the_values_of_a_wait(values);
    the_window_of_the_wait_is_remembered(rt, &values, line);
    let answer = builtins::win_wait_close(rt, &values, line, 0);
    the_wait_that_goes_on(rt, answer, false)
}

/// `WinWaitNotActive, WinTitle, ...`: wait for no window to have the focus.
///
/// The command is of the v1.1.37.02: the source of it
/// (`globaldata.cpp:443`) carries it next to the `WinWaitClose`.
pub(crate) fn win_wait_not_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let values = the_values_of_a_wait(values);
    the_window_of_the_wait_is_remembered(rt, &values, line);
    let answer = builtins::win_wait_not_active(rt, &values, line, 0);
    the_wait_that_goes_on(rt, answer, false)
}

/// The window that the wait looks for becomes the last found window.
///
/// The wait that talks about the absence of a window does not answer with
/// a window, and the v1 puts the window that it looks for in the last
/// found window: the source of it (`script2.cpp:2277-2297`) asks the
/// `WinExist` for it, and the `WinExist` puts the window that it finds
/// there.
///
/// The title that is not written is the last found window itself, and the
/// `WinExist` without a title does not change it.
fn the_window_of_the_wait_is_remembered(rt: &mut Runtime<'_>, values: &[Value], line: u32) {
    let title = values.first().map(|value| value.to_text()).unwrap_or_default();
    if title.is_empty() {
        return;
    }
    if let Ok(Some(window)) = the_window_named(rt, &title, line) {
        rt.remember_the_last_window_of_the_v1(&window.id.to_string());
    }
}

/// The answer of a wait of the v1.
///
/// The `ErrorLevel` is one when the wait ran out and zero when it found a
/// window, and the window that it found is the last found window of the
/// v1: measured on the binary of v1.1.37.02.
///
/// The wait that talks about the absence of a window answers a number and
/// not a window, and that number is not the last found window: the
/// `WinWaitClose` and the `WinWaitNotActive` leave the last found window
/// where it was. Measured on the binary of v1.1.37.02: the source of it
/// (`script2.cpp:2277-2297`) asks the `WinExist` for the window, and the
/// `WinExist` without a title does not change the last found window.
fn the_wait_that_goes_on(
    rt: &mut Runtime<'_>,
    answer: Result<Value, RuntimeError>,
    finds_a_window: bool,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => {
            let found = value.to_text();
            // The wait that found a window remembers it, and the wait that
            // found nothing does not take the last found window away:
            // measured on the binary of v1.1.37.02 a wait that runs out
            // leaves the window that an earlier wait found where it was.
            if finds_a_window && !found.is_empty() && found != "0" {
                rt.remember_the_last_window_of_the_v1(&found);
            }
            if found.is_empty() || found == "0" {
                rt.set_the_error_level(Value::Int(1));
            } else {
                rt.set_the_error_level(Value::Int(0));
            }
            Ok(value)
        }
        Err(failure) => {
            rt.set_the_error_level(Value::Int(1));
            rt.note(format!(
                "the command of the wait did not do the thing: {}",
                failure.message
            ));
            Ok(Value::Int(0))
        }
    }
}

/// The values of a command of the `WinWait` family of the v1, in the
/// order of the v2.
///
/// The v1 is the title, the text of the window, the timeout and the two
/// exclusions, and the v2 is the title, the text of the window, the
/// timeout and the two exclusions as well: the timeout is in the third
/// place of the two dialects. Measured on the binary of v1.1.37.02 the timeout of the v1 is the third parameter, and the two
/// exclusions are read and not refused.
///
/// The timeout that the script did not write is the timeout that does
/// not end, and the zero is the meio segundo: measured on the binary of
/// v1.1.37.02, the `WinWait, titulo` without a timeout
/// waits forever, and the `WinWait, titulo, , 0` waits the meio
/// segundo. This is why the timeout that is missing is a value that is
/// not a number, and not the zero.
fn the_values_of_a_wait(values: &[Value]) -> Vec<Value> {
    let mut answer: Vec<Value> = values.iter().take(5).cloned().collect();
    while answer.len() < 3 {
        answer.push(Value::Text("".into()));
    }
    answer
}

/// The command of the v1 that acts on a window, and whose failure is not
/// a failure of the script.
///
/// The v1: the window that is not there is not something that the v1
/// says: the command is silent and the script goes on.
///
/// Measured on the binary of v1.1.37.02: the `WinHide` and
/// the `WinShow` of a window that is not there say nothing.
fn the_act_on_a_window(
    rt: &mut Runtime<'_>,
    _name: &str,
    values: &[Value],
    line: u32,
    handler: BuiltinHandler,
) -> Result<Value, RuntimeError> {
    let answer = handler(rt, values, line, 0);
    match answer {
        Ok(value) => {
            rt.set_the_error_level(Value::Int(0));
            Ok(value)
        }
        Err(_) => {
            // The v1 says nothing: the window that is not there is not a
            // failure of the script.
            Ok(Value::Text("".into()))
        }
    }
}

/// The window of the machine that has the focus, when it has one.
///
/// The title `A` alone is that window: the v1 and the v2 ask for it that
/// way, and the rule is the macro of the v1 (source `window.h:30-31` of
/// v1.1.37.02).
fn the_window_with_the_focus(
    rt: &mut Runtime<'_>,
    line: u32,
) -> Result<Option<crate::desktop::WindowInfo>, RuntimeError> {
    let all = rt
        .desktop()
        .windows()
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(all.into_iter().find(|window| window.active))
}

/// The window that the text of the v1 names.
///
/// The text of the v1 is a title or a criterion: the `ahk_id 0x...` names
/// the window by the number of it, and the `ahk_class` and the `ahk_exe`
/// name it by the class and by the program of it.
///
/// The answer is `None` when no window of the machine has that name, and
/// the v1 says nothing about it.
fn the_window_named(
    rt: &mut Runtime<'_>,
    text: &str,
    line: u32,
) -> Result<Option<crate::desktop::WindowInfo>, RuntimeError> {
    // The title `A` alone is the window that has the focus.
    if crate::desktop::criteria::the_title_is_the_active_window(text) {
        return the_window_with_the_focus(rt, line);
    }
    let mode = crate::settings::title_match_mode();
    let found = rt
        .desktop()
        .matching_windows(text, mode)
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    let Some(first) = found.first() else {
        return Ok(None);
    };
    let all = rt
        .desktop()
        .windows()
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(all
        .into_iter()
        .find(|window| window.id.to_string() == first.id))
}

/// `WinActivate, WinTitle, ...`: give the focus to a window.
///
/// The v1: the window that is not there is not a failure that the v1 says:
/// the command is silent and the script goes on.
pub(crate) fn win_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let title = values.first().map(|value| value.to_text()).unwrap_or_default();
    let Some(window) = the_window_named(rt, &title, line)? else {
        return Ok(Value::Text("".into()));
    };
    rt.desktop()
        .activate_window(&window)
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `WinActivateBottom, WinTitle, ...`: give the focus to the bottom of the
/// windows that match.
pub(crate) fn win_activate_bottom(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(
        rt,
        "WinActivateBottom",
        values,
        line,
        builtins::win_activate_bottom,
    )
}

/// `WinHide, WinTitle, ...`: take a window off the screen.
pub(crate) fn win_hide(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinHide", values, line, builtins::win_hide)
}

/// `WinShow, WinTitle, ...`: put a window back on the screen.
pub(crate) fn win_show(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinShow", values, line, builtins::win_show)
}

/// `WinMinimize, WinTitle, ...`: minimize a window.
pub(crate) fn win_minimize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinMinimize", values, line, builtins::win_minimize)
}

/// `WinMaximize, WinTitle, ...`: maximize a window.
pub(crate) fn win_maximize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinMaximize", values, line, builtins::win_maximize)
}

/// `WinRestore, WinTitle, ...`: put a window back to its size.
pub(crate) fn win_restore(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinRestore", values, line, builtins::win_restore)
}

/// `WinMove, WinTitle, WinText, X, Y, Width, Height, ...`: move a window.
///
/// The v1: the title of the window comes first, and the place and the
/// size come after it. The CHM of the v1.1.37.02 says of the command:
/// "WinMove , WinTitle, WinText, X, Y, Width, Height, ExcludeTitle,
/// ExcludeText".
///
/// The value that is left out, and the word "Default", keep the place or
/// the size that the window has.
///
/// The window that is not there is not a failure that the v1 says: the
/// command is silent and the script goes on.
pub(crate) fn win_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let title = values.first().map(|value| value.to_text()).unwrap_or_default();
    let x = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let y = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let width = values.get(4).map(|value| value.to_text()).unwrap_or_default();
    let height = values.get(5).map(|value| value.to_text()).unwrap_or_default();
    let Some(window) = the_window_named(rt, &title, line)? else {
        return Ok(Value::Text("".into()));
    };
    // The place and the size that the window has now: the value that is
    // left out keeps them.
    let mode = crate::settings::title_match_mode();
    let geometry = rt
        .desktop()
        .window_geometry(&title, mode)
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    let (current_x, current_y, current_width, current_height) = match geometry {
        Some(geometry) => (geometry.x, geometry.y, geometry.width, geometry.height),
        None => (0, 0, 0, 0),
    };
    let wanted_x = the_place_of_the_v1(&x, current_x);
    let wanted_y = the_place_of_the_v1(&y, current_y);
    let wanted_width = the_place_of_the_v1(&width, current_width);
    let wanted_height = the_place_of_the_v1(&height, current_height);
    rt.desktop()
        .move_window(&window, wanted_x, wanted_y, wanted_width, wanted_height)
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// The place or the size that the text of the v1 asks for.
///
/// The text that is left out, and the word "Default", ask for the place
/// or the size that the window has.
fn the_place_of_the_v1(text: &str, current: i64) -> i64 {
    let text = text.trim();
    if text.is_empty() || text.eq_ignore_ascii_case("default") {
        return current;
    }
    text.parse::<i64>().unwrap_or(current)
}

/// `WinSetTitle, NewTitle` or `WinSetTitle, WinTitle, WinText, NewTitle, ...`.
///
/// The v1: the source of the binary of v1.1.37.02 (`script.cpp`) reads
/// the command as `mArgc > 1 ? WinSetTitle(FIVE_ARGS) : WinSetTitle("",
/// "", ARG1)`. With more than one parameter, the first is the title that
/// names the window and the third is the new title; with one parameter,
/// the new title is the first and the window is the one that was found
/// last.
///
/// The window that is not there is not a failure: the source of the v1
/// (`script2.cpp`) says that the command "always return OK, even if the
/// target window doesn't exist".
///
/// A script of the v1 renames its window with the criteria of the
/// window: `WinSetTitle, ahk_id %PVhwnd%,, % verType thisMsg " | v1"`.
pub(crate) fn win_set_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (criteria, new_title) = if values.len() > 1 {
        (
            values.first().map(|value| value.to_text()).unwrap_or_default(),
            values.get(2).map(|value| value.to_text()).unwrap_or_default(),
        )
    } else {
        (
            String::new(),
            values.first().map(|value| value.to_text()).unwrap_or_default(),
        )
    };
    // The window: the one that the criteria of the script name, or the
    // one that was found last.
    let window = if criteria.trim().is_empty() {
        the_last_window(rt, line)?
    } else {
        the_window_named(rt, &criteria, line)?
    };
    let Some(window) = window else {
        return Ok(Value::Text("".into()));
    };
    // The v1 says nothing when the window is not there.
    let _ = rt.desktop().set_window_title(&window, new_title.trim());
    Ok(Value::Text("".into()))
}

/// The window that the v1 found last, when there is one.
fn the_last_window(
    rt: &mut Runtime<'_>,
    line: u32,
) -> Result<Option<crate::desktop::WindowInfo>, RuntimeError> {
    let Some(id) = rt.the_last_window_of_the_v1().map(|id| id.to_string()) else {
        return Ok(None);
    };
    let all = rt
        .desktop()
        .windows()
        .map_err(|reason| builtins::desktop_failure(reason, line))?;
    Ok(all
        .into_iter()
        .find(|window| window.id.to_string() == id))
}

/// `WinClose, WinTitle, ...`: close a window.
pub(crate) fn win_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinClose", values, line, builtins::win_close)
}

/// `WinKill, WinTitle, ...`: close a window without asking it.
pub(crate) fn win_kill(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(rt, "WinKill", values, line, builtins::win_kill)
}

/// `WinMinimizeAll`: minimize every window of the session.
pub(crate) fn win_minimize_all(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(
        rt,
        "WinMinimizeAll",
        values,
        line,
        builtins::win_minimize_all,
    )
}

/// `WinMinimizeAllUndo`: put the minimized windows back.
pub(crate) fn win_minimize_all_undo(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_act_on_a_window(
        rt,
        "WinMinimizeAllUndo",
        values,
        line,
        builtins::win_minimize_all_undo,
    )
}

/// `WinSet, SubCommand, Value, WinTitle, ...`: a change to the state of a
/// window.
///
/// The v1: the title of the window is the third parameter and the value of
/// the change is the second one. The CHM of the v1.1.37.02 says of the
/// command: "WinSet , Setting, Value, WinTitle, WinText, ExcludeTitle,
/// ExcludeText".
///
/// The window that is not there is not a failure that the v1 says: the
/// command is silent and the script goes on.
///
/// Measured on the binary of v1.1.37.02: the `WinSet,
/// Enable, , ahk_id 999999999` says nothing, leaves the `ErrorLevel` at
/// zero, and the script goes on.
pub(crate) fn win_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let sub_command = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let value = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let title = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let lower = sub_command.to_ascii_lowercase();
    // The v1: only the "Style", the "ExStyle" and the "Region" leave the
    // `ErrorLevel` at one when they do not do their thing.
    let uses_the_error_level = matches!(lower.as_str(), "style" | "exstyle" | "region");
    match the_change_of_the_sub_command(rt, &lower, &value, &title, line) {
        TheChange::Done(value) => {
            if uses_the_error_level {
                rt.set_the_error_level(Value::Int(0));
            }
            Ok(value)
        }
        TheChange::NoWindow => {
            if uses_the_error_level {
                rt.set_the_error_level(Value::Int(1));
            }
            Ok(Value::Text("".into()))
        }
        TheChange::Refused(failure) => {
            rt.note(format!(
                "the command WinSet did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// What the `WinSet` of the v1 did.
enum TheChange {
    /// The change was made.
    Done(Value),
    /// The window that the command names is not there, and the v1 says
    /// nothing about it.
    NoWindow,
    /// The sub-command is not one that the v1 knows.
    Refused(RuntimeError),
}

/// The change that the sub-command of the `WinSet` names.
///
/// The v1: every sub-command of the `WinSet` works on the window that the
/// title names, and the window that is not there is not a failure that the
/// v1 says.
fn the_change_of_the_sub_command(
    rt: &mut Runtime<'_>,
    sub_command: &str,
    value: &str,
    title: &str,
    line: u32,
) -> TheChange {
    let window = match the_window_named(rt, title, line) {
        Ok(Some(window)) => window,
        Ok(None) => return TheChange::NoWindow,
        Err(failure) => return TheChange::Refused(failure),
    };
    match sub_command {
        "alwaysontop" | "topmost" => {
            // The v1: the value that is left out turns the state of the
            // window around.
            let on = match value.trim().to_ascii_lowercase().as_str() {
                "on" | "1" | "true" => true,
                "off" | "0" | "false" => false,
                _ => !rt
                    .desktop()
                    .always_on_top(&window)
                    .ok()
                    .flatten()
                    .unwrap_or(false),
            };
            match rt.desktop().set_always_on_top(&window, on) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        "style" | "exstyle" => {
            // The v1: the value that is left out is an error, and the
            // prefix of the value says what to do with the style that is
            // there: the "+" adds the bits, the "-" takes them away and
            // the "^" turns them around.
            let extended = sub_command == "exstyle";
            let Some(change) = crate::window_style::the_style_change_of_a_text(value.trim()) else {
                return TheChange::Refused(RuntimeError {
                    message: "Parameter #2 invalid.".to_string(),
                    line,
                });
            };
            let current = match rt.desktop().window_style(title, extended) {
                Ok(style) => style as u32,
                Err(reason) => return TheChange::Refused(builtins::desktop_failure(reason, line)),
            };
            let wanted = change.the_style_of(current) as i64;
            match rt.desktop().window_set_style(title, wanted, extended) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        "transparent" | "trans" => {
            // The v1: the value is the alpha of the window, and the word
            // "Off" takes the transparency away.
            let alpha = if value.trim().eq_ignore_ascii_case("off") {
                255
            } else {
                match value.trim().parse::<i64>() {
                    Ok(alpha) => alpha,
                    Err(_) => {
                        return TheChange::Refused(RuntimeError {
                            message: "Parameter #2 invalid.".to_string(),
                            line,
                        })
                    }
                }
            };
            match rt.desktop().set_window_alpha(&window, alpha) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        "transcolor" => {
            if value.trim().is_empty() {
                return TheChange::Refused(RuntimeError {
                    message: "Parameter #2 must not be blank.".to_string(),
                    line,
                });
            }
            match rt.desktop().window_set_trans_color(title, value.trim()) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        "enable" | "disable" => {
            let on = sub_command == "enable";
            match rt.desktop().set_window_enabled(&window, on) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        "region" => {
            // The v1: the value is the shape of the region, and the value
            // that is left out takes the region of the window away.
            let region = match the_region_of_the_v1(value) {
                Some(region) => region,
                None => {
                    return TheChange::Refused(RuntimeError {
                        message: "Parameter #2 invalid.".to_string(),
                        line,
                    })
                }
            };
            match rt.desktop().set_window_region(&window, &region) {
                Ok(()) => TheChange::Done(Value::Text("".into())),
                Err(reason) => TheChange::Refused(builtins::desktop_failure(reason, line)),
            }
        }
        _ => TheChange::Refused(RuntimeError {
            message: format!("the sub-command {sub_command} of the WinSet is not known."),
            line,
        }),
    }
}

/// The region of a window of the v1, out of the text that the script wrote.
///
/// The text is a list of points and of letters: the "X-Y" is a point, the
/// "E" asks for an ellipse, the "R<w>-<h>" asks for a rectangle with
/// rounded corners, the "W<width>" and the "H<height>" ask for the size,
/// and the "Wind" asks for the winding rule of the polygon.
///
/// The v1: the text that is left out takes the region of the window away.
///
/// Measured on the binary of v1.1.37.02: the
/// `WinSet, Region, 0-0 R6-6 w100 h50` of a window gives the window a
/// rectangle with rounded corners, and the `WinSet, Region, , ahk_id X`
/// takes the region away.
fn the_region_of_the_v1(text: &str) -> Option<crate::desktop::WindowRegion> {
    let text = text.trim();
    if text.is_empty() {
        return Some(crate::desktop::WindowRegion::default());
    }
    let mut region = crate::desktop::WindowRegion::default();
    let mut width: Option<i64> = None;
    let mut height: Option<i64> = None;
    let mut radius: Option<(i64, i64)> = None;
    let mut ellipse = false;
    let mut rest = text;
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }
        let first = rest.chars().next()?;
        if first.is_ascii_digit() || first == '-' || first == '+' {
            // The point: the "X-Y".
            let dash = rest[1..].find('-')? + 1;
            let x = rest[..dash].parse::<i64>().ok()?;
            let after = &rest[dash + 1..];
            let end = after
                .find(|character: char| !character.is_ascii_digit() && character != '-')
                .unwrap_or(after.len());
            let y = after[..end].parse::<i64>().ok()?;
            region.points.push((x, y));
            rest = &after[end..];
            continue;
        }
        let letter = first.to_ascii_uppercase();
        let after = &rest[1..];
        match letter {
            'E' => {
                ellipse = true;
                rest = after;
            }
            'R' => {
                // The corner: the "R<w>-<h>", and the thirty by thirty of
                // the v1 when the numbers are not there.
                let (w, h, tail) = the_two_numbers_of_the_corner(after)?;
                radius = Some((w, h));
                rest = tail;
            }
            'W' => {
                if after.len() >= 3 && after[..3].eq_ignore_ascii_case("ind") {
                    region.winding = true;
                    rest = &after[3..];
                    continue;
                }
                let (number, tail) = the_number_of_the_option(after)?;
                width = Some(number);
                rest = tail;
            }
            'H' => {
                let (number, tail) = the_number_of_the_option(after)?;
                height = Some(number);
                rest = tail;
            }
            _ => return None,
        }
    }
    if region.points.is_empty() {
        return None;
    }
    let (x, y) = region.points[0];
    region.x = x;
    region.y = y;
    // The v1: the width and the height are the size of the shape, and
    // the shape starts at the first point.
    if let (Some(width), Some(height)) = (width, height) {
        region.width = width;
        region.height = height;
    }
    if let Some((radius_x, radius_y)) = radius {
        region.radius_x = radius_x;
        region.radius_y = radius_y;
        region.kind = crate::desktop::WindowRegionKind::RoundedRectangle;
    } else if ellipse {
        region.kind = crate::desktop::WindowRegionKind::Ellipse;
    } else if width.is_some() && height.is_some() {
        region.kind = crate::desktop::WindowRegionKind::Rectangle;
    } else {
        region.kind = crate::desktop::WindowRegionKind::Polygon;
    }
    Some(region)
}

/// The two numbers of the corner of a region, and the text that is left.
fn the_two_numbers_of_the_corner(text: &str) -> Option<(i64, i64, &str)> {
    let (first, tail) = the_number_of_the_option(text)?;
    let tail = tail.trim_start();
    if !tail.starts_with('-') {
        return None;
    }
    let (second, tail) = the_number_of_the_option(tail)?;
    Some((first, second, tail))
}

/// The number of an option of the region, and the text that is left.
fn the_number_of_the_option(text: &str) -> Option<(i64, &str)> {
    let text = text.trim_start();
    let end = text
        .find(|character: char| !character.is_ascii_digit() && character != '-')
        .unwrap_or(text.len());
    let number = text[..end].parse::<i64>().ok()?;
    Some((number, &text[end..]))
}

/// `WinMenuSelectItem, WinTitle, ..., Item1, Item2, ...`: choose an item
/// of the menu of a window.
///
/// The menu of another program is not read by this runtime, and the menu
/// of the window of the script is: the refusal says so.
pub(crate) fn win_menu_select_item(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The menu of the window of the script is drawn by the helper, and the
    // items of it are the ones of the helper.
    //
    // Measured on the binary of v1.1.37.02: the failure of the command
    // is the `ErrorLevel` and not the end of the script: the
    // `WinMenuSelectItem` of a window that is not there goes on, and the
    // `ErrorLevel` is one.
    let answer = match rt.desktop().gui_menu_bar("{}") {
        Ok(()) => Err(RuntimeError {
            message: "the menu of another program is not read by this runtime, and the menu of the window of the script is not read either.".to_string(),
            line,
        }),
        Err(reason) => Err(builtins::desktop_failure(reason, line)),
    };
    super::commands::the_answer_of_an_act(rt, "WinMenuSelectItem", answer)
}