//! The `ScriptProcess` class of the KeySharp dialect.
//!
//! KeySharp makes a process of the script an object: the members of it
//! say whether the process has exited, the code it exited with, the
//! moment it exited, and the streams of it, and `Kill` kills it at once.
//!
//! The state of the process is asked of the machine, and the machine that
//! cannot say is refused with the reason: the tree does not answer a
//! state it does not know.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The key the pid of the process is under.
const THE_PID: &str = "\u{1}scriptprocess pid";

/// The names of the members of a `ScriptProcess`.
pub(crate) fn names() -> Vec<&'static str> {
    vec![
        "hasexited",
        "exitcode",
        "exittime",
        "kill",
        "stdout",
        "stderr",
        "stdin",
    ]
}

/// The object a `ScriptProcess` makes: the process of that pid.
pub(crate) fn construct(
    _class: &str,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Some(first) = arguments.first() else {
        return Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "Too few parameters passed to function.".to_string(),
            line,
        })));
    };
    let pid = match first {
        Value::Int(number) => *number as u32,
        other => {
            return Some(Err(RuntimeFailure::Error(RuntimeError {
                message: crate::runtime::requires(
                    1,
                    "ScriptProcess",
                    crate::runtime::TypeName::Number,
                    other,
                ),
                line,
            })));
        }
    };
    let Value::Object(object) = crate::classes::object("scriptprocess") else {
        return None;
    };
    object.borrow_mut().put(THE_PID, Value::Int(pid as i64));
    Some(Ok(Value::Object(object)))
}

/// The pid of the process of an object, and the diagnosis when it is not
/// there.
fn the_pid(this: &Value, line: u32) -> Result<u32, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(RuntimeFailure::Error(RuntimeError {
            message: "the value is not an object.".to_string(),
            line,
        }));
    };
    let pid = match object.borrow().value_of(THE_PID) {
        Some(Value::Int(pid)) => Some(pid as u32),
        _ => None,
    };
    match pid {
        Some(pid) => Ok(pid),
        None => {
            // The diagnosis: what the object is and what it carries.
            let object = object.borrow();
            Err(RuntimeFailure::Error(RuntimeError {
                message: format!(
                    "the object of the class {} carries {} properties and no pid",
                    object.class,
                    object.properties.len()
                ),
                line,
            }))
        }
    }
}

/// A member of a `ScriptProcess`, as a property read.
///
/// The facts are read live: the state of the process moves while the script
/// runs, the way it moves in KeySharp.
pub(crate) fn property(
    _class: &str,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let lowered = name.to_lowercase();
    match lowered.as_str() {
        "hasexited" | "exitcode" | "exittime" => {}
        _ => return None,
    }
    let pid = match the_pid(this, line) {
        Ok(pid) => pid,
        Err(failure) => return Some(Err(failure)),
    };
    let state = match rt.desktop().the_state_of_a_process(pid) {
        Ok(state) => state,
        Err(reason) => {
            return Some(Err(RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })));
        }
    };
    let Some(state) = state else {
        // The machine does not know the process, and it does not know
        // whether it has exited: the tree says so and does not answer zero.
        return Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "the desktop of this run does not know the state of that process.".to_string(),
            line,
        })));
    };
    let answer = match lowered.as_str() {
        "hasexited" => Value::Int(if state.has_exited { 1 } else { 0 }),
        // The code of a process that has not exited is unset, the way the
        // v2.1 mode of KeySharp reads it.
        "exitcode" => match state.exit_code {
            Some(code) => Value::Int(code as i64),
            None => Value::Unset,
        },
        // The moment of the exit is unset while the process runs.
        "exittime" => match state.exit_time {
            Some(moment) => Value::Text(moment),
            None => Value::Unset,
        },
        _ => Value::Unset,
    };
    Some(Ok(answer))
}

/// A member of a `ScriptProcess`, as a call.
///
/// The only method is `Kill`, and it kills the process at once.
pub(crate) fn call(
    _class: &str,
    method: &str,
    this: &Value,
    _arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !method.eq_ignore_ascii_case("kill") {
        return None;
    }
    let pid = match the_pid(this, line) {
        Ok(pid) => pid,
        Err(failure) => return Some(Err(failure)),
    };
    if let Err(reason) = rt.desktop().kill_a_process(pid) {
        return Some(Err(RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })));
    }
    // The answer of a method that answers nothing is the empty text.
    Some(Ok(Value::Text(String::new())))
}
