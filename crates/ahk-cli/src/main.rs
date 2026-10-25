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

use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use ahk::parse;
use ahk::runtime::Runtime;
use ahk_box::LabDesktop;

/// Where the runtime says it is running.
///
/// A script that types forever has to be stoppable from outside, and the
/// only thing that can stop it is the process. The pid file is how the
/// `ahk_stop` tool finds it -- and how a human finds it, with `cat`.
fn pid_file() -> PathBuf {
    let base = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cache"));
    base.join("yoko-ahk").join("run.pid")
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

fn main() {
    let mut source = String::new();
    if let Err(error) = std::io::stdin().read_to_string(&mut source) {
        eprintln!("cannot read the script: {error}");
        std::process::exit(2);
    }
    let timeout = std::env::args()
        .skip(1)
        .find_map(|argument| argument.strip_prefix("--timeout=").and_then(|value| value.parse::<f64>().ok()))
        .unwrap_or(60.0);

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
            std::process::exit(2);
        }
    };

    // From here on, the run can be stopped from outside: the pid file says
    // which process to kill.
    write_pid_file();
    let _guard = PidGuard;

    let mut desktop = LabDesktop::new();
    desktop.ensure_session_env();
    let report = Runtime::new(&mut desktop, Duration::from_secs_f64(timeout)).run(&statements);
    let mut payload = serde_json::to_value(&report).expect("the report serialises");
    if let Some(map) = payload.as_object_mut() {
        map.insert("notes".to_string(), serde_json::json!(desktop.notes()));
    }
    println!("{payload}");
    std::process::exit(if report.ok { 0 } else { 1 });
}

/// Removes the pid file when the run ends, however it ends.
struct PidGuard;

impl Drop for PidGuard {
    fn drop(&mut self) {
        clear_pid_file();
    }
}
