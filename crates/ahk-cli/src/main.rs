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
use std::time::Duration;

use ahk::runtime::{Report, Runtime};
use ahk::parse;
use ahk_box::LabDesktop;

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

/// The report, for the callers that want it typed.
#[allow(dead_code)]
fn typed(report: &Report) -> &Report {
    report
}
