//! The trace of a run: one JSON object per line, in a file.
//!
//! `AHK_TRACE_JSONL` names the file of the trace. The runtime writes one
//! line per event -- a native call, a message of a window, a timer that
//! ran, a global that changed -- and it flushes every line, so that a run
//! that hangs still leaves a trace that can be read.
//!
//! The trace is off when the variable is not set: no file is made, no
//! line is written, and the run costs what it costs without it. The check
//! is one read of a flag, and the events ask for it before they gather
//! anything.
//!
//! The shape of the lines is a contract with the v1 that is measured
//! beside this tree: the same four kinds with the same fields, so that the
//! two traces are compared with a diff.
//!
//! ```text
//! {"kind":"dllcall","fn":"user32\\GetWindowRect","args":[1234],"result":null}
//! {"kind":"gui_message","hwnd":5678,"msg":71,"wparam":0,"lparam":0}
//! {"kind":"timer","name":"mytimer","period":-50}
//! {"kind":"var","name":"g","value":"42"}
//! ```
//!
//! The arguments of a native call are the ones the script gave: the
//! numbers as numbers, the texts as texts, and the name of the variable
//! when the script gave a reference to one.
//!
//! The globals are traced only when `AHK_TRACE_VARS` is set as well: the
//! trace of them costs a comparison of the value that changed, and a
//! change of a global is rare.

use core::fmt::Write as _;

use crate::json::Object;
use crate::runtime::Value;

/// Whether the trace of the run is asked for.
///
/// The variable is read once, and the answer does not change while the
/// run goes on: the check of every event is one read of a flag.
#[cfg(feature = "std")]
pub(crate) fn is_asked() -> bool {
    static THE_ANSWER: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *THE_ANSWER.get_or_init(|| std::env::var_os("AHK_TRACE_JSONL").is_some())
}

/// Whether the trace of the run is asked for.
///
/// The crate runs on a target that has no files, and a trace that has
/// nowhere to go is no trace.
#[cfg(not(feature = "std"))]
pub(crate) fn is_asked() -> bool {
    false
}

/// Whether the trace of the globals is asked for.
///
/// The globals are traced when the file of the trace is asked for and
/// when the variables are asked for as well: the trace of them costs a
/// comparison of the value that changed, and a change of a global is
/// rare.
#[cfg(feature = "std")]
pub(crate) fn the_vars_are_asked() -> bool {
    static THE_ANSWER: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    is_asked() && *THE_ANSWER.get_or_init(|| std::env::var_os("AHK_TRACE_VARS").is_some())
}

/// Whether the trace of the globals is asked for.
#[cfg(not(feature = "std"))]
pub(crate) fn the_vars_are_asked() -> bool {
    false
}

/// The trace of a native call.
///
/// The target is the one the script wrote, the arguments are the ones it
/// gave -- the numbers as numbers and the texts as texts -- and the
/// answer is the number the call returned: a text is not a number, and a
/// call that failed has no answer at all.
pub(crate) fn dll_call(target: &str, args: &[Value], result: Option<&Value>) {
    if !is_asked() {
        return;
    }
    let mut object = Object::new(96);
    object.text("kind", "dllcall");
    object.text("fn", target);
    object.field("args", |out| {
        out.push('[');
        for (index, argument) in args.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            match argument {
                Value::Int(number) => {
                    let _ = write!(out, "{number}");
                }
                Value::Float(number) if number.is_finite() => {
                    let _ = write!(out, "{number}");
                }
                Value::Text(text) => crate::json::write_text(out, text),
                // A reference is the variable that the script named: the
                // name of it is the name the script wrote.
                Value::Reference(reference) => {
                    crate::json::write_text(out, &reference.name);
                }
                other => crate::json::write_text(out, &other.to_text()),
            }
        }
        out.push(']');
    });
    object.field("result", |out| match result {
        Some(Value::Int(number)) => {
            let _ = write!(out, "{number}");
        }
        Some(Value::Float(number)) if number.is_finite() => {
            let _ = write!(out, "{number}");
        }
        // A text is not a number, and a call that failed has no answer.
        _ => out.push_str("null"),
    });
    write(&object.finish());
}

/// The trace of a message of a window.
///
/// The message is the one that the runtime dispatches to the handler of
/// the script, and the handle is the one of the window that received it.
pub(crate) fn gui_message(hwnd: i64, msg: i64, wparam: i64, lparam: i64) {
    if !is_asked() {
        return;
    }
    let mut object = Object::new(96);
    object.text("kind", "gui_message");
    object.number("hwnd", hwnd);
    object.number("msg", msg);
    object.number("wparam", wparam);
    object.number("lparam", lparam);
    write(&object.finish());
}

/// The trace of a timer that ran.
///
/// The name is the name of the function of the script that the timer
/// runs, and the period is the milliseconds between two runs: a negative
/// period is a timer that runs once.
pub(crate) fn timer(name: &str, period: i64) {
    if !is_asked() {
        return;
    }
    let mut object = Object::new(64);
    object.text("kind", "timer");
    object.text("name", name);
    object.number("period", period);
    write(&object.finish());
}

/// The trace of a global that changed.
///
/// The name is the name of the variable in the table of the globals --
/// the lower case of it, because that is the key the runtime keeps it
/// under -- and the value is the text of the value it holds.
pub(crate) fn variable(name: &str, value: &str) {
    if !the_vars_are_asked() {
        return;
    }
    let mut object = Object::new(64);
    object.text("kind", "var");
    object.text("name", name);
    object.text("value", value);
    write(&object.finish());
}

/// Write a line of the trace, and flush it.
///
/// The line is written whole and flushed, so that a run which hangs
/// still leaves a trace that can be read.
#[cfg(feature = "std")]
fn write(line: &str) {
    writer::write(line);
}

/// Write a line of the trace.
///
/// The crate runs on a target that has no files, and a trace that has
/// nowhere to go is no trace.
#[cfg(not(feature = "std"))]
fn write(_line: &str) {}

/// The file of the trace.
///
/// The file is opened by the first thread that writes a line, and it is
/// truncated then. The other threads open it to append to it, and the
/// lock keeps two lines from crossing each other.
#[cfg(feature = "std")]
mod writer {
    use std::cell::RefCell;
    use std::fs::{File, OpenOptions};
    use std::io::Write as _;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Mutex, OnceLock};

    // The file of each thread: a thread opens its own and writes in it.
    thread_local! {
        static THE_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    }

    /// The lock that keeps two threads from writing at the same time.
    static THE_LOCK: Mutex<()> = Mutex::new(());

    /// Whether the file was truncated already.
    static THE_FILE_WAS_MADE: AtomicBool = AtomicBool::new(false);

    /// Whether the failure to open the file was said already.
    static THE_FAILURE_WAS_SAID: AtomicBool = AtomicBool::new(false);

    /// The path of the file of the trace.
    fn the_path() -> Option<&'static PathBuf> {
        static THE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
        THE_PATH
            .get_or_init(|| std::env::var_os("AHK_TRACE_JSONL").map(PathBuf::from))
            .as_ref()
    }

    /// Write a line in the file of the trace, and flush it.
    pub(super) fn write(line: &str) {
        let Some(path) = the_path() else {
            return;
        };
        let Ok(_lock) = THE_LOCK.lock() else {
            return;
        };
        THE_FILE.with(|held| {
            let mut held = held.borrow_mut();
            if held.is_none() {
                let Some(file) = the_file_is_opened(path) else {
                    return;
                };
                *held = Some(file);
            }
            let Some(file) = held.as_mut() else {
                return;
            };
            let _ = file.write_all(line.as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.flush();
        });
    }

    /// The file of the trace, opened: the first one truncates it, and the
    /// others append to it.
    fn the_file_is_opened(path: &Path) -> Option<File> {
        let mut options = OpenOptions::new();
        options.write(true).create(true);
        if THE_FILE_WAS_MADE.swap(true, Ordering::SeqCst) {
            options.append(true);
        } else {
            options.truncate(true);
        }
        match options.open(path) {
            Ok(file) => Some(file),
            Err(error) => {
                if !THE_FAILURE_WAS_SAID.swap(true, Ordering::SeqCst) {
                    eprintln!("the trace of the run could not be written: {error}");
                }
                None
            }
        }
    }
}
