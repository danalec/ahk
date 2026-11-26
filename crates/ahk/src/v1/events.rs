//! The events of the v1: the `OnExit`, the `OnError` and the
//! `OnClipboardChange`.
//!
//! Measured on the binary of v1.1.37.02: the three of them are one
//! function of the v1 -- the callback and the mode -- and the name of the
//! function tells them apart. The `OnExit` is also a command of the v1, and
//! the command names a label: the function wants the callback, and the two
//! are two things.

use alloc::format;
use alloc::string::{String, ToString};

use crate::runtime::{Runtime, RuntimeError, Value};

/// The event that the function of the v1 asks the callback for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TheEvent {
    /// The end of the script: the `OnExit`.
    Exit,
    /// A failure that was not caught: the `OnError`.
    Error,
    /// The change of the clipboard: the `OnClipboardChange`.
    Clipboard,
}

impl TheEvent {
    /// The fewest parameters that the function of the callback may take.
    ///
    /// Measured on the binary of v1.1.37.02: the function of the `OnExit`
    /// is called with the reason of the exit and the code, and the function
    /// of the `OnError` and of the `OnClipboardChange` is called with one
    /// value. A function that takes more than that is not a callback of the
    /// event.
    fn the_fewest_parameters(self) -> usize {
        match self {
            TheEvent::Exit => 2,
            TheEvent::Error | TheEvent::Clipboard => 1,
        }
    }
}

/// The callback of one of the three functions of the v1.
pub(crate) struct TheCallback {
    /// The name of the function of the script that the callback runs.
    ///
    /// It is `None` when the callback is an object that does not name a
    /// function of the script: the tree keeps the callbacks by the name of
    /// the function they run, and an object that is not a function has no
    /// name to keep.
    pub name: Option<String>,
    /// The mode of the call: the 1 adds the callback, the 0 takes it away,
    /// and the -1 adds it once.
    pub mode: i32,
}

/// The callback of the `OnExit`, the `OnError` or the `OnClipboardChange`
/// of the v1, and the mode of the call.
///
/// Measured on the binary of v1.1.37.02: the first parameter wants the
/// callback, and it wants it in one of two ways: a text with the name of a
/// function of the script, or an object. A value that is neither is
/// refused with "Parameter #1 invalid." -- the refusal of the v1 does not
/// name the function, and it comes before the refusal of the mode.
///
/// Measured on the binary of v1.1.37.02: the name in a text has to be the
/// name of a function and not the name of a label: the `OnExit("Bye")` of a
/// script that has the label `Bye` is refused with "Parameter #1 invalid.".
/// The function has to take at most the parameters of the event: the
/// `OnError("Handler")` of a function that takes three parameters is
/// refused the same way.
///
/// Measured on the binary of v1.1.37.02: the second parameter is the mode.
/// The 1 adds the callback, the 0 takes it away, and the -1 adds it once. A
/// mode that is not one of the three is refused with "Parameter #2
/// invalid.". A mode that is not a number is the 0, and a mode that has a
/// point is cut to a whole number: the `OnExit({}, 1.5)` adds the callback
/// and the `OnExit({}, "x")` takes it away.
pub(crate) fn the_callback_and_the_mode(
    rt: &Runtime<'_>,
    values: &[Value],
    line: u32,
    event: TheEvent,
) -> Result<TheCallback, RuntimeError> {
    let name = match values.first() {
        // An object is the callback itself, and the v1 does not ask it for
        // anything else. The object of a function of the script is the one
        // exception: the v1 asks it for the parameters it takes.
        Some(Value::Object(object)) => {
            let object = object.borrow();
            let name = match object.value_of("name") {
                Some(Value::Text(text)) => Some(text.into_string()),
                _ => None,
            };
            let fewest = match object.value_of("minparams") {
                Some(Value::Int(fewest)) => fewest as usize,
                _ => 0,
            };
            if fewest > event.the_fewest_parameters() {
                return Err(RuntimeError {
                    message: "Parameter #1 invalid.".to_string(),
                    line,
                });
            }
            name
        }
        // A text is the name of a function of the script.
        Some(Value::Text(text)) => {
            let name = text.to_string();
            let fewest = rt.the_fewest_parameters_of_the_function(&name);
            match fewest {
                Some(fewest) if fewest <= event.the_fewest_parameters() => Some(name),
                _ => {
                    return Err(RuntimeError {
                        message: "Parameter #1 invalid.".to_string(),
                        line,
                    })
                }
            }
        }
        // A value that is neither an object nor a text is not a callback.
        _ => {
            return Err(RuntimeError {
                message: "Parameter #1 invalid.".to_string(),
                line,
            })
        }
    };
    // The second parameter is the mode.
    let mode = match values.get(1) {
        Some(value) => value.to_number().unwrap_or(0.0) as i32,
        None => 1,
    };
    if !(-1..=1).contains(&mode) {
        return Err(RuntimeError {
            message: "Parameter #2 invalid.".to_string(),
            line,
        });
    }
    Ok(TheCallback { name, mode })
}

/// `OnExit(Callback, Mode)`: the function that runs when the script ends.
///
/// Measured on the binary of v1.1.37.02: the answer of the call is the
/// empty text, and the callback is the one of the rule of the module.
///
/// The callback that is an object and not the name of a function is
/// accepted, the way the v1 accepts it, and the tree keeps it nowhere: the
/// callbacks of the tree are the functions of the script, and the object of
/// a callback is not one of them.
pub(crate) fn on_exit(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let callback = the_callback_and_the_mode(rt, values, line, TheEvent::Exit)?;
    if let Some(name) = &callback.name {
        match callback.mode {
            1 => rt.set_exit_callback(name, line),
            -1 => {
                rt.set_exit_callback(name, line);
                rt.order_exit_callback(name, true);
            }
            _ => rt.remove_exit_callback(name),
        }
    }
    Ok(Value::Text("".into()))
}

/// `OnError(Callback, Mode)`: the function that runs when a failure is not
/// caught.
///
/// Measured on the binary of v1.1.37.02: the answer of the call is the
/// empty text, and the callback is the one of the rule of the module.
pub(crate) fn on_error(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let callback = the_callback_and_the_mode(rt, values, line, TheEvent::Error)?;
    if let Some(name) = &callback.name {
        match callback.mode {
            1 => rt.set_error_callback(name, line),
            -1 => {
                rt.set_error_callback(name, line);
                rt.order_error_callback(name, true);
            }
            _ => rt.remove_error_callback(name),
        }
    }
    Ok(Value::Text("".into()))
}

/// `OnClipboardChange(Callback, Mode)`: the function that runs when the
/// clipboard changes.
///
/// Measured on the binary of v1.1.37.02: the answer of the call is the
/// empty text, and the callback is the one of the rule of the module.
///
/// The tree does not watch the clipboard, and the call says so in a note.
pub(crate) fn on_clipboard_change(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The callback is checked the way the `OnExit` checks it, and the tree
    // does not watch the clipboard: the call says so in a note.
    the_callback_and_the_mode(rt, values, line, TheEvent::Clipboard)?;
    rt.note(format!(
        "the clipboard is not watched by this runtime, and the callback will not run (line {line})"
    ));
    Ok(Value::Text("".into()))
}
