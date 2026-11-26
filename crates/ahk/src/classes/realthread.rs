//! The `RealThread` class of the KeySharp dialect.
//!
//! KeySharp runs a script on real threads of the platform, and
//! `A_RealThread` is the one the script runs on. The `Create` of it starts
//! a worker thread that runs a function of the script on a real thread of
//! the machine: the worker parses the source again on its own side and
//! runs the function there, and the object of the worker reads its state
//! -- alive, terminated, the task, the answer. The main thread is not a
//! worker, so its `Task` and `Terminated` are refused the way KeySharp
//! refuses them, and its `Exit` stops the run the way `ExitApp` does.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::desktop::ThreadValue;
use crate::runtime::{Callback, Runtime, RuntimeError, RuntimeFailure, Value};

/// The name of the function a `Post` or a `Send` calls.
///
/// The callback is a function of the script, and it may take any number
/// of parameters -- the two-parameter rule of the error callbacks is not
/// asked of it. `None` means the value is not a function.
fn the_name_of_the_callback(value: Option<&Value>) -> Option<String> {
    let value = value?;
    if let Value::Object(object) = value {
        let object = object.borrow();
        if object.class == "func" {
            let name = object.value_of("name").map(|name| name.to_text())?;
            if !name.is_empty() {
                return Some(name);
            }
        }
        return None;
    }
    None
}

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
        "create",
        "result",
        "exitcode",
        "wait",
        "error",
    ]
}

/// The object of the one real thread of the run: the thread the script
/// runs on, which is the main one.
pub(crate) fn the_main_thread() -> Value {
    crate::classes::object("realthread")
}

/// The key the number of the worker lives under in the object.
const WORKER_ID: &str = "\u{1}realthread worker";
/// The key the name of the task of the worker lives under.
const WORKER_TASK: &str = "\u{1}realthread task";

/// The object of a worker thread: it carries the number that names the
/// worker and the function it runs.
pub(crate) fn the_worker_thread(id: u64, task: &str) -> Value {
    crate::classes::object_with(
        "realthread",
        vec![
            (WORKER_ID, Value::Int(id as i64)),
            (WORKER_TASK, Value::Text(task.to_string().into())),
        ],
    )
}

/// The number of the worker of a RealThread object, when it is a worker.
/// The main thread carries none.
fn the_worker_id(this: &Value) -> Option<u64> {
    if let Value::Object(object) = this {
        if let Some(Value::Int(id)) = object.borrow().value_of(WORKER_ID) {
            return Some(id as u64);
        }
    }
    None
}



/// A member of a `RealThread`, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !_class.eq_ignore_ascii_case("realthread") {
        return None;
    }
    // A worker thread reads its state out of the table of the workers.
    if let Some(id) = the_worker_id(this) {
        return Some(the_worker_property(id, name, this, rt, line));
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
            let mut items = vec![the_main_thread()];
            if let Ok(workers) = rt.desktop().worker_thread_list() {
                for (id, task) in workers {
                    items.push(the_worker_thread(id, &task));
                }
            }
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

/// The property of a worker thread: its state, read live out of the
/// table of the workers.
fn the_worker_property(
    id: u64,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let lowered = name.to_lowercase();
    // The task is the name the worker was created with.
    if lowered == "task" {
        let task = match this {
            Value::Object(object) => object
                .borrow()
                .value_of(WORKER_TASK)
                .unwrap_or(Value::Text("".into())),
            _ => Value::Text("".into()),
        };
        return Ok(task);
    }
    let state = rt.desktop().worker_thread_state(id).map_err(|reason| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    })?;
    let answer = match lowered.as_str() {
        "isalive" => Value::Int(i64::from(state.alive)),
        "terminated" => Value::Int(i64::from(state.terminated)),
        "exitcode" => Value::Int(state.exit_code),
        "id" => Value::Int(id as i64),
        "result" => match state.result {
            Some(answer) => answer.to_value(),
            None => Value::Unset,
        },
        "error" => match state.error {
            Some(error) => Value::Text(error.into()),
            None => Value::Text("".into()),
        },
        _ => {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!(
                    "This value of type \"RealThread\" has no property named {name:?}."
                ),
                line,
            }))
        }
    };
    Ok(answer)
}

/// A method of a `RealThread`.
pub(crate) fn call(
    _class: &str,
    method: &str,
    this: &Value,
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
        // The `Post` puts the callback in the queue of the thread, and the
        // runtime runs it between the statements of the script. The `Send`
        // runs it at once and answers what it answered. The tree runs the
        // script on one thread, so the two run on that one.
        "post" => {
            let name = match the_name_of_the_callback(arguments.first()) {
                Some(name) => name,
                None => {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: crate::runtime::wants_a_reference(
                            1,
                            "Post",
                            "a Func",
                            arguments.first().unwrap_or(&Value::Int(0)),
                        ),
                        line,
                    })))
                }
            };
            rt.post_callback(Callback { name, line });
            Some(Ok(Value::Int(1)))
        }
        "send" => {
            let name = match the_name_of_the_callback(arguments.first()) {
                Some(name) => name,
                None => {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: crate::runtime::wants_a_reference(
                            1,
                            "Send",
                            "a Func",
                            arguments.first().unwrap_or(&Value::Int(0)),
                        ),
                        line,
                    })))
                }
            };
            Some(rt.call_named(&name, &[], line, crate::runtime::Route::Written))
        }
        // The `Create` starts a worker thread that runs a function of the
        // script on a real thread of the machine, and answers the object
        // of the worker.
        "create" => {
            let name = match the_name_of_the_callback(arguments.first()) {
                Some(name) => name,
                None => {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: crate::runtime::wants_a_reference(
                            1,
                            "Create",
                            "a Func",
                            arguments.first().unwrap_or(&Value::Int(0)),
                        ),
                        line,
                    })))
                }
            };
            let args: Vec<ThreadValue> =
                arguments.iter().skip(1).map(ThreadValue::of_value).collect();
            let source = rt.the_source();
            let dialect = rt.the_dialect_name();
            let globals = rt.the_globals_as_thread_values();
            #[cfg(feature = "std")]
            let shared = Some(rt.share_the_globals());
            #[cfg(not(feature = "std"))]
            let shared = None;
            let created =
                rt.desktop()
                    .create_worker_thread(&source, dialect, &name, args, globals, shared);
            Some(match created {
                Ok(id) => Ok(the_worker_thread(id, &name)),
                Err(reason) => Err(RuntimeFailure::Error(RuntimeError {
                    message: reason,
                    line,
                })),
            })
        }
        // The `Wait` blocks until the worker finishes, or the timeout
        // passes: it answers one when the worker terminated, zero when
        // the timeout passed first.
        "wait" => {
            #[cfg_attr(not(feature = "std"), allow(unused_variables))]
            let Some(id) = the_worker_id(this) else {
                return Some(Err(RuntimeFailure::Error(RuntimeError {
                    message: "the main thread cannot be waited for: it is the one that runs"
                        .to_string(),
                    line,
                })));
            };
            let timeout_ms = arguments.first().and_then(|v| v.to_number()).unwrap_or(-1.0);
            #[cfg(feature = "std")]
            {
                let start = std::time::Instant::now();
                loop {
                    let state = match rt.desktop().worker_thread_state(id) {
                        Ok(state) => state,
                        Err(reason) => {
                            return Some(Err(RuntimeFailure::Error(RuntimeError {
                                message: reason,
                                line,
                            })))
                        }
                    };
                    if state.terminated {
                        return Some(Ok(Value::Int(1)));
                    }
                    if timeout_ms >= 0.0
                        && start.elapsed().as_millis() as f64 >= timeout_ms
                    {
                        return Some(Ok(Value::Int(0)));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
            #[cfg(not(feature = "std"))]
            {
                let _ = timeout_ms;
                Some(Err(RuntimeFailure::Error(RuntimeError {
                    message: "the wait of a worker needs the std of the tree".to_string(),
                    line,
                })))
            }
        }
        _ => None,
    }
}
