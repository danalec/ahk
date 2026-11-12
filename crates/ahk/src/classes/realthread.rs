//! The `RealThread` class of the KeySharp dialect.
//!
//! KeySharp runs a script on real threads of the platform, and
//! `A_RealThread` is the one the script runs on. The tree runs the script
//! on one real thread and no other: the collection of the threads is that
//! one, the worker members are refused with the reason -- the way
//! KeySharp itself refuses `Task` and `Terminated` on a thread that is
//! not a worker -- and `Exit` stops the run the way `ExitApp` does.

use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The names of the members of a `RealThread`.
pub(crate) fn names() -> Vec<&'static str> {
    vec![
        "id",
        "isalive",
        "threads",
        "main",
        "task",
        "terminated",
        "exit",
        "post",
        "send",
    ]
}

/// The object of the one real thread of the run: the thread the script
/// runs on, which is the main one.
pub(crate) fn the_main_thread() -> Value {
    crate::classes::object("realthread")
}

/// A member of a `RealThread`, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    _this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !_class.eq_ignore_ascii_case("realthread") {
        return None;
    }
    match name.to_lowercase().as_str() {
        // The thread is alive while the script runs on it: the question
        // is asked from the thread itself.
        "isalive" => Some(Ok(Value::Int(1))),
        // The identity of the thread is the machine's, and a machine
        // that cannot say is refused with the reason.
        "id" => match rt.desktop().the_current_thread_id() {
            Ok(id) => Some(Ok(Value::Int(id))),
            Err(reason) => Some(Err(RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            }))),
        },
        // The tree runs the script on one real thread: the collection of
        // the threads is that one, the main one.
        "threads" => {
            let items = vec![the_main_thread()];
            Some(crate::classes::array::construct(&items, rt, line))
        }
        // The main thread is the one the script runs on.
        "main" => Some(Ok(the_main_thread())),
        // The thread of the tree is not a worker: the two members of a
        // worker are refused, the way KeySharp refuses them on its main
        // thread.
        "task" | "terminated" => Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "the only real thread of this run is the main one, and it is not a worker: it has no task of its own".to_string(),
            line,
        }))),
        _ => None,
    }
}

/// A method of a `RealThread`.
pub(crate) fn call(
    _class: &str,
    method: &str,
    _this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !_class.eq_ignore_ascii_case("realthread") {
        return None;
    }
    match method.to_lowercase().as_str() {
        // The exit of the main thread is the end of the run, the way
        // ExitApp ends it: the code is kept and the script stops at the
        // end of the statement.
        "exit" => {
            let code = arguments.first().and_then(|value| value.to_number()).unwrap_or(0.0) as i32;
            rt.exit_with(code);
            Some(Ok(Value::Int(code as i64)))
        }
        // The tree has no queue of a real thread to post to: the script
        // runs on the only thread there is, and a callback posted to it
        // would be a callback the script itself has to run.
        "post" | "send" => Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "this tree runs the script on one real thread and has no queue to post to: Post and Send are not carried".to_string(),
            line,
        }))),
        _ => None,
    }
}
