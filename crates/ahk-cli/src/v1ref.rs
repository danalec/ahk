// SPDX-License-Identifier: Apache-2.0
//! The reference of the v1: the binary of the AutoHotkey v1 and what it
//! says about a script.
//!
//! The tree answers a script one way and the v1 another; the difference
//! is the bug to find. The tools of this module run the same script under
//! the two and give the two answers side by side, so a client reads the
//! difference without a shell and without the window of the v1.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// The binary of the v1 that is the reference of the tree.
///
/// The path is read from `AHK_V1` when it is there, and this is the
/// place of the install of the v1.1.37.02 on a machine of Windows.
const THE_V1: &str = "C:\\Program Files\\AutoHotkey\\v1.1.37.02\\AutoHotkeyU64.exe";

/// The seconds the v1 may take before the answer is taken as hung.
const THE_SECONDS: u64 = 10;

/// The path of the binary of the v1.
pub fn the_path_of_the_v1() -> String {
    std::env::var("AHK_V1").unwrap_or_else(|_| THE_V1.to_string())
}

/// What the v1 said about a script.
#[derive(Debug, Default)]
pub struct TheAnswer {
    /// The text that the v1 wrote out.
    pub output: String,
    /// The code of the end of the v1, and `None` when it did not end.
    pub code: Option<i32>,
    /// Whether the v1 was still running when the time was over.
    pub hung: bool,
    /// The reason the v1 could not be run at all.
    pub failure: Option<String>,
}

impl TheAnswer {
    /// The lines of the answer, with the line of the script in them.
    ///
    /// The v1 writes the errors like this:
    ///
    /// ```text
    /// probe.ahk (12) : ==> Message
    /// ```
    ///
    /// The script is written with the directive of the v1 in front, and
    /// the line of the error is one more than the line of the script.
    pub fn the_errors(&self) -> Vec<String> {
        self.output
            .lines()
            .filter(|line| line.contains("==>"))
            .map(the_line_of_the_script)
            .collect()
    }
}

/// Take one line of the v1 and give it back with the line of the script.
///
/// The directive of the v1 is a line of its own in front of the script,
/// and the line of the error is one more than the line of the script.
fn the_line_of_the_script(line: &str) -> String {
    let Some(mark) = line.find(" : ==> ") else {
        return line.to_string();
    };
    let (head, tail) = line.split_at(mark);
    let Some(open) = head.rfind('(') else {
        return line.to_string();
    };
    let Some(close) = head[open..].find(')') else {
        return line.to_string();
    };
    let close = open + close;
    let digits = &head[open + 1..close];
    let Ok(number) = digits.trim().parse::<u64>() else {
        return line.to_string();
    };
    if number == 0 {
        return line.to_string();
    }
    format!("{}({}){}", &head[..open], number - 1, &head[close + 1..])
        + tail
}

/// The file of the script, and the directory of it.
fn the_file_of_the_script(source: &str) -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("ahk-mcp-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let file = dir.join("probe.ahk");
    // The errors of the run go to the output and not to a dialog of the
    // machine: the dialog waits for a mouse that nobody moves. The
    // directive is the one of the v1 (`Set +ErrorStdOut` is of the v2).
    std::fs::write(&file, format!("#ErrorStdOut\n{source}"))?;
    Ok(file)
}

/// Run the source under the binary of the v1 and answer with what it
/// said.
pub fn the_run_of_the_v1(source: &str) -> TheAnswer {
    let path = the_path_of_the_v1();
    if !Path::new(&path).exists() {
        return TheAnswer {
            failure: Some(format!(
                "the binary of the v1 is not there: {path} (the path comes from AHK_V1)"
            )),
            ..TheAnswer::default()
        };
    }
    let file = match the_file_of_the_script(source) {
        Ok(file) => file,
        Err(e) => {
            return TheAnswer {
                failure: Some(format!("the script was not written: {e}")),
                ..TheAnswer::default()
            }
        }
    };
    let mut child = match Command::new(&path)
        .arg("/ErrorStdOut")
        .arg(&file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            return TheAnswer {
                failure: Some(format!("the v1 did not start: {e}")),
                ..TheAnswer::default()
            }
        }
    };
    let deadline = Instant::now() + Duration::from_secs(THE_SECONDS);
    let mut code = None;
    let mut hung = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                code = status.code();
                break;
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    hung = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(e) => {
                return TheAnswer {
                    failure: Some(format!("the v1 was not waited for: {e}")),
                    ..TheAnswer::default()
                };
            }
        }
    }
    let mut output = String::new();
    let mut errors = String::new();
    if let Some(mut out) = child.stdout.take() {
        let _ = out.read_to_string(&mut output);
    }
    if let Some(mut err) = child.stderr.take() {
        let _ = err.read_to_string(&mut errors);
    }
    if !errors.is_empty() {
        output.push_str(&errors);
    }
    TheAnswer {
        output,
        code,
        hung,
        failure: None,
    }
}
