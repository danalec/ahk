//! The `Thread` class of the KeySharp dialect.
//!
//! KeySharp makes the current pseudo-thread an object: `A_Thread` is the
//! one of them, and `Thread()` makes the one of the main thread. This
//! runtime runs one script in one thread, so there is exactly one thread
//! to describe, and the facts of it are the ones of the run: the kind of
//! it is the auto-execute one, it is active, it is interruptible, and it
//! has nothing underneath it.
//!
//! The members are the ones the documentation of KeySharp writes and the
//! source of it answers (`KeysharpThread.cs`, read on 2026-11-11). The
//! elapsed time is read live at every access, the way KeySharp reads it.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use crate::runtime::{Runtime, RuntimeFailure, Value};

/// The members the class answers, in the lower case: the twelve properties
/// and the one method.
pub(crate) fn names() -> Vec<&'static str> {
    vec![
        "id",
        "index",
        "isactive",
        "kind",
        "elapsed",
        "priority",
        "allowtimers",
        "critical",
        "paused",
        "isinterruptible",
        "underlying",
        "exitcode",
        "exit",
    ]
}

/// The object the class makes: the current thread, of which this runtime
/// has exactly one.
pub(crate) fn construct(
    _arguments: &[Value],
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    Ok(crate::classes::object_with("thread", vec![]))
}

/// A member of the `Thread`, as a property read.
///
/// The facts are read live: the elapsed time moves while the script runs,
/// the way it moves in KeySharp.
pub(crate) fn property(
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let _ = (this, line);
    let lowered = name.to_lowercase();
    let answer = match lowered.as_str() {
        // The main thread is the first created, at the position zero of the
        // stack of the one real thread: the id names that position.
        "id" => Value::Int(0),
        // The one-based position in the stack of the real thread.
        "index" => Value::Int(1),
        // The one thread this runtime runs is the one that runs.
        "isactive" => Value::Int(1),
        // The main thread is launched automatically.
        "kind" => Value::Text("Auto".to_string()),
        "elapsed" => Value::Int(rt.elapsed_ms() as i64),
        "priority" => Value::Int(0),
        "allowtimers" => Value::Int(1),
        // This runtime cannot make a thread critical: the guard rail says
        // so, and the thread is therefore not critical.
        "critical" => Value::Int(0),
        "paused" => Value::Int(0),
        // Not critical, and the startup window of the interrupt is over
        // the moment the script runs: the thread can be interrupted.
        "isinterruptible" => Value::Int(1),
        // The thread underneath the oldest one: there is none, and the
        // answer of KeySharp for that is the empty text.
        "underlying" => Value::Text(String::new()),
        // The v2.1 mode reads the code of a thread that has not exited as
        // the value of unset.
        "exitcode" => Value::Unset,
        _ => return None,
    };
    Some(Ok(answer))
}

/// A member of the `Thread`, as a method call.
///
/// The only method is `Exit`, which stops the script the way `ExitApp`
/// does: the runtime marks the code and the script ends. The answer is the
/// id of the thread, the way KeySharp answers it.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let _ = (this, line);
    let lowered = method.to_lowercase();
    match lowered.as_str() {
        "exit" => {
            let code = arguments
                .first()
                .map(|value| value.to_text().trim().parse::<i32>())
                .transpose()
                .ok()
                .flatten()
                .unwrap_or(0);
            rt.exit_with(code);
            Some(Ok(Value::Int(0)))
        }
        // The properties answer as calls too: the call of one runs the
        // same reading, the way a call of a property of the v2 does.
        "id" | "index" | "isactive" | "kind" | "elapsed" | "priority" | "allowtimers"
        | "critical" | "paused" | "isinterruptible" | "underlying" | "exitcode" => {
            property(method, this, rt, line)
        }
        _ => None,
    }
}
