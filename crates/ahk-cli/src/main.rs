//! The runner: a script on stdin, a report on stdout.
//!
//! This is the binary that runs on the lab box. The MCP hands it a script
//! over stdin -- in memory, never a file -- and reads the report back.
//!
//! ```sh
//! echo 'Msg "hello"' | ahk
//! ```
//!
//! The exit code is 0 when the script finished, 1 when it failed, and 2
//! when it did not parse; the report is on stdout either way.
//!
//! The `--surface` flag is the other way round: it prints the table of
//! the names of v2 with the state of each one, and it does not read a
//! script.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ahk::parse;
use ahk::runtime::Runtime;
use ahk::surface;
use ahk_box::LabDesktop;

/// Where the runtime says it is running.
///
/// A script that types forever has to be stoppable from outside, and the
/// only thing that can stop it is the process. The pid file is how the
/// `ahk_stop` tool finds it -- and how a human finds it, with `cat`.
///
/// `XDG_CACHE_HOME` is the first choice, then the cache directory of the
/// home directory. A host with neither -- a Windows one that nobody
/// configured -- gets the temporary directory, and never the working
/// directory: the file has to be where the stop looks for it, and the
/// working directory changes from one call to the next.
fn pid_file() -> PathBuf {
    let base = match std::env::var_os("XDG_CACHE_HOME") {
        Some(base) => PathBuf::from(base),
        None => match std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            Some(home) => PathBuf::from(home).join(".cache"),
            None => std::env::temp_dir(),
        },
    };
    base.join("ahk").join("run.pid")
}

fn write_pid_file() {
    let path = pid_file();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, format!("{}\n", std::process::id()));
}

fn clear_pid_file() {
    let _ = std::fs::remove_file(pid_file());
}

/// How much stack the run of a script gets.
///
/// The frames of the tree are heavier than the ones of v2, and a script
/// that recurses needs a stack that the recursion rail fits in.
const STACK_BYTES: usize = 128 * 1024 * 1024;
/// How much of that stack the run may use.
const STACK_BUDGET: usize = 64 * 1024 * 1024;

fn main() -> std::process::ExitCode {
    // The flag is answered before the script is read: the command is
    // about the tree, and it must not wait for a script that is not
    // coming.
    if std::env::args().skip(1).any(|argument| argument == "--surface") {
        print_surface();
        return std::process::ExitCode::from(0);
    }

    let mut source = String::new();
    if let Err(error) = std::io::stdin().read_to_string(&mut source) {
        eprintln!("cannot read the script: {error}");
        return std::process::ExitCode::from(2);
    }
    let timeout = std::env::args()
        .skip(1)
        .find_map(|argument| argument.strip_prefix("--timeout=").and_then(|value| value.parse::<f64>().ok()))
        .unwrap_or(60.0);

    // The includes are resolved here and not in the language: the
    // language crate does no I/O, and the runner is the one that has a
    // disk to read from.
    let source = match resolve_includes(&source, Path::new("."), &mut Vec::new()) {
        Ok(source) => source,
        Err(error) => {
            let report = serde_json::json!({
                "ok": false,
                "error": error,
                "error_line": 0,
                "column": 0,
                "steps": [],
                "output": [],
                "notes": [],
            });
            println!("{report}");
            return std::process::ExitCode::from(2);
        }
    };

    let statements = match parse(&source) {
        Ok(statements) => statements,
        Err(error) => {
            let report = serde_json::json!({
                "ok": false,
                "error": error.message,
                "error_line": error.line,
                "column": error.column,
                "steps": [],
                "output": [],
                "notes": [],
            });
            println!("{report}");
            return std::process::ExitCode::from(2);
        }
    };

    // From here on, the run can be stopped from outside: the pid file says
    // which process to kill.
    write_pid_file();
    let _guard = PidGuard;

    // The script runs on a thread with a stack of its own: the tree's
    // frames are heavier than the ones of v2, and the recursion rail has
    // to fit in the stack of the run. The report is written down there,
    // because a value of the runtime is not `Send` and cannot travel.
    let handle = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(move || {
            let mut desktop = LabDesktop::new();
            desktop.ensure_session_env();
            let report = Runtime::new(&mut desktop, Duration::from_secs_f64(timeout))
                .with_stack_budget(STACK_BUDGET)
                .with_source(&source)
                .run(&statements);
            let mut payload = serde_json::to_value(&report).expect("the report serialises");
            if let Some(map) = payload.as_object_mut() {
                // The notes of the runtime come first, and the notes of the
                // box are added to them: a host that wrote a note of its own
                // keeps it.
                let mut notes = report.notes.clone();
                notes.extend(desktop.notes().iter().cloned());
                map.insert("notes".to_string(), serde_json::json!(notes));
            }
            (payload.to_string(), report.ok)
        })
        .expect("the thread of the run starts");
    let (payload, ok) = match handle.join() {
        Ok(pair) => pair,
        Err(_) => {
            eprintln!("the run of the script died");
            return std::process::ExitCode::from(1);
        }
    };
    println!("{payload}");
    // The exit code goes out through the return, and not through
    // `process::exit`: that one leaves without running the destructors, and
    // the guard that removes the pid file is a destructor.
    std::process::ExitCode::from(if ok { 0 } else { 1 })
}

/// Print the surface of v2 and where every name of it stands here.
///
/// The report is a line for every name of the surface and a summary at
/// the end: the number is the point of the command, and it is what makes
/// "what is missing" a measurement and not an opinion.
fn print_surface() {
    let entries = surface::entries();
    let mut out = String::new();
    for entry in &entries {
        out.push_str(&entry.line());
        out.push('\n');
    }
    out.push_str(&surface::summary(&entries).text());
    out.push('\n');
    print!("{out}");
}

/// The source with the includes put in place of the lines that ask for
/// them.
///
/// A `#Include "path"` line is replaced by the text of the file, and the
/// path is taken from the directory of the file that asks for it. An
/// include that is already in the tree is not put in twice: the second
/// time it is read it is skipped, the way `#IncludeAgain` says.
fn resolve_includes(
    source: &str,
    base: &Path,
    seen: &mut Vec<PathBuf>,
) -> Result<String, String> {
    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim();
        let Some(argument) = trimmed
            .strip_prefix("#Include")
            .or_else(|| trimmed.strip_prefix("#include"))
        else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        let argument = argument.trim();
        // The path is in quotes, or it is the rest of the line.
        let path = argument
            .strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .unwrap_or(argument)
            .trim();
        if path.is_empty() {
            return Err("an include needs a path.".to_string());
        }
        let path = base.join(path);
        let path = match path.canonicalize() {
            Ok(path) => path,
            Err(_) => return Err(format!("the include \"{path:?}\" cannot be found.")),
        };
        if seen.contains(&path) {
            // The file is already in the tree: it is not put in twice.
            out.push('\n');
            continue;
        }
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("the include \"{path:?}\" cannot be read: {error}"))?;
        let directory = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        seen.push(path);
        out.push_str(&resolve_includes(&text, &directory, seen)?);
        out.push('\n');
    }
    Ok(out)
}

/// Removes the pid file when the run ends, however it ends.
struct PidGuard;

impl Drop for PidGuard {
    fn drop(&mut self) {
        clear_pid_file();
    }
}
