//! The runner: a script on stdin, a report on stdout.
//!
//! These are the cases of the archived Python runner, ported, and the two
//! the archive does not have: the exit codes of the process, and the pid
//! file that makes a runaway script stoppable from outside.
//!
//! The runner is what runs on the lab box. Here the box is a fake: the
//! `Msg` statement is the only one that needs nothing.
//!
//! The `--surface` flag is the other way round: the report is the table
//! of the names of v2, and there is no script.

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

/// The runner, as it was built.
const RUNNER: &str = env!("CARGO_BIN_EXE_ahk");

/// The surface of v2.0.19, as the table of it.
const SURFACE: &str = include_str!("../../ahk/reference/ahk-v2.0.19.surface.tsv");

/// Run a script and answer with the exit code and the report.
fn run(source: &str) -> (i32, serde_json::Value) {
    let mut child = Command::new(RUNNER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(source.as_bytes())
        .expect("the script goes in");
    let output = child.wait_with_output().expect("the runner finishes");
    let text = String::from_utf8(output.stdout).expect("the report is utf-8");
    let report: serde_json::Value = serde_json::from_str(text.trim()).expect("the report is json");
    (output.status.code().unwrap_or(-1), report)
}

#[test]
fn a_script_answers_with_a_report() {
    let (code, report) = run("MsgBox \"hello from the box\"\n");
    assert_eq!(code, 0);
    assert_eq!(report["ok"], true);
    assert_eq!(report["output"][0], "hello from the box");
}

#[test]
fn a_script_that_does_not_parse_answers_with_the_line() {
    let (code, report) = run("MsgBox \"unterminated\n");
    assert_eq!(code, 2);
    assert_eq!(report["ok"], false);
    assert_eq!(report["error_line"], 1);
    assert!(report["column"].is_number());
}

#[test]
fn a_failing_script_answers_with_a_non_zero_exit() {
    let (code, report) = run("x := 1 / 0\n");
    assert_eq!(code, 1);
    assert_eq!(report["ok"], false);
    assert_eq!(report["error"], "Divide by zero.");
    assert_eq!(report["error_line"], 1);
}

#[test]
fn the_report_carries_the_fields_the_client_reads() {
    let (_code, report) = run("MsgBox \"hello\"\n");
    let fields: Vec<&str> = report.as_object().expect("the report is an object").keys().map(String::as_str).collect();
    for field in ["ok", "steps", "output", "notes", "error", "error_line", "exit_code", "elapsed_ms", "variables"] {
        assert!(fields.contains(&field), "the report has no {field}");
    }
}

#[test]
fn the_exit_app_code_is_in_the_report_and_not_in_the_exit_code() {
    let (code, report) = run("ExitApp 3\n");
    assert_eq!(code, 0);
    assert_eq!(report["exit_code"], 3);
}

#[test]
fn the_pid_file_is_written_while_the_script_runs_and_removed_after() {
    // The file is how a script that types forever is stopped from outside,
    // so it has to be there while the script runs and gone when it ends.
    let cache = std::env::temp_dir().join(format!("ahk-runner-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&cache);
    std::fs::create_dir_all(&cache).expect("the cache directory");
    let pid_file = cache.join("ahk").join("run.pid");

    let mut child = Command::new(RUNNER)
        .env("XDG_CACHE_HOME", &cache)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(b"Sleep 2000\n")
        .expect("the script goes in");

    let mut written = None;
    for _ in 0..100 {
        if let Ok(text) = std::fs::read_to_string(&pid_file) {
            written = Some(text);
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let text = written.expect("the pid file is written while the script runs");
    assert_eq!(text.trim(), child.id().to_string());

    let output = child.wait_with_output().expect("the runner finishes");
    assert!(output.status.success());
    assert!(!pid_file.exists(), "the pid file is removed when the script ends");
    let _ = std::fs::remove_dir_all(&cache);
}

#[test]
fn the_pid_file_does_not_land_in_the_working_directory() {
    // A host with no cache directory of its own must not have the runner
    // write the file into the directory the caller happened to be in.
    let cache = std::env::temp_dir().join(format!("ahk-runner-nohome-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&cache);
    std::fs::create_dir_all(&cache).expect("the cache directory");
    // a run of an older build may have left one here; the case is about
    // what this run does, so it starts from a clean directory
    let _ = std::fs::remove_dir_all(".cache");

    let mut child = Command::new(RUNNER)
        .env_remove("XDG_CACHE_HOME")
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .env("TMPDIR", &cache)
        .env("TMP", &cache)
        .env("TEMP", &cache)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(b"MsgBox \"x\"\n")
        .expect("the script goes in");
    let output = child.wait_with_output().expect("the runner finishes");
    assert!(output.status.success());

    assert!(
        !std::path::Path::new(".cache").exists(),
        "the pid file landed in the working directory"
    );
    let _ = std::fs::remove_dir_all(&cache);
}

/// Ask the runner for the surface of v2, with no script on stdin.
///
/// The stdin of the runner is a pipe that is never written to and never
/// closed: a runner that waits for a script waits for ever, and the case
/// is that the flag answers before the read.
fn surface() -> (i32, String) {
    let mut child = Command::new(RUNNER)
        .arg("--surface")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let _script = child.stdin.take().expect("the runner reads a script");
    let stdout = child.stdout.take().expect("the runner writes the surface");
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let mut text = String::new();
        let mut stdout = stdout;
        let _ = stdout.read_to_string(&mut text);
        let _ = sender.send(text);
    });
    let text = match receiver.recv_timeout(Duration::from_secs(30)) {
        Ok(text) => text,
        Err(_) => {
            let _ = child.kill();
            panic!("the runner did not answer the flag without a script on stdin");
        }
    };
    let status = child.wait().expect("the runner finishes");
    (status.code().unwrap_or(-1), text)
}

/// Ask the runner for the surface of the dialect, as JSON.
///
/// The JSON is for the CI, and it carries the numbers of the summary and
/// the state of every name.
fn surface_json(dialect: &str) -> (i32, serde_json::Value) {
    let mut child = Command::new(RUNNER)
        .arg("--surface")
        .arg("--json")
        .arg(format!("--dialect={dialect}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let _script = child.stdin.take().expect("the runner reads a script");
    let stdout = child.stdout.take().expect("the runner writes the surface");
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let mut text = String::new();
        let mut stdout = stdout;
        let _ = stdout.read_to_string(&mut text);
        let _ = sender.send(text);
    });
    let text = match receiver.recv_timeout(Duration::from_secs(30)) {
        Ok(text) => text,
        Err(_) => {
            let _ = child.kill();
            panic!("the runner did not answer the flag without a script on stdin");
        }
    };
    let status = child.wait().expect("the runner finishes");
    let report: serde_json::Value =
        serde_json::from_str(text.trim()).expect("the surface is json");
    (status.code().unwrap_or(-1), report)
}

#[test]
fn the_surface_flag_answers_with_json() {
    let (code, report) = surface_json("v2");
    assert_eq!(code, 0);
    assert_eq!(report["dialect"], "v2");
    assert_eq!(report["total"], 828);
    assert_eq!(report["values"], 637);
    assert_eq!(report["refusals"], 191);
    assert_eq!(report["pending"], 0);

    let entries = report["entries"].as_array().expect("the entries are a list");
    assert_eq!(entries.len(), 828);
    // A name that the tree answers is a value, and a name that the tree
    // refuses carries the reason of the refusal.
    let abs = entries
        .iter()
        .find(|entry| entry["name"] == "Abs")
        .expect("the name is in the report");
    assert_eq!(abs["kind"], "function");
    assert_eq!(abs["state"], "value");
    let windows = entries
        .iter()
        .find(|entry| entry["name"] == "ControlSendText")
        .expect("the name is in the report");
    assert_eq!(windows["state"], "refusal");
    assert!(windows["reason"].is_string());
}

#[test]
fn the_surface_flag_answers_with_the_json_of_the_v1() {
    let (code, report) = surface_json("v1");
    assert_eq!(code, 0);
    assert_eq!(report["dialect"], "v1");
    assert_eq!(report["total"], 405);
    // The names that the v1 has and the v2 does not are carried, and the one
    // that is still to do is pending.
    assert_eq!(report["values"], 296);
    assert_eq!(report["pending"], 1);
    let entries = report["entries"].as_array().expect("the entries are a list");
    let is_set = entries
        .iter()
        .find(|entry| entry["name"] == "IsSet")
        .expect("the name is in the report");
    assert_eq!(is_set["state"], "value");
    // The `AutoTrim` is carried: the command turns the trim of the text of a
    // legacy assignment on and off.
    let auto_trim = entries
        .iter()
        .find(|entry| entry["name"] == "AutoTrim")
        .expect("the name is in the report");
    assert_eq!(auto_trim["state"], "value");
    // The `SetFormat` is carried: the command turns the format of the
    // numbers of the v1.
    let set_format = entries
        .iter()
        .find(|entry| entry["name"] == "SetFormat")
        .expect("the name is in the report");
    assert_eq!(set_format["state"], "value");
    // The `StringCaseSense` is carried: the command turns the sense of the
    // case of the comparisons on and off.
    let string_case_sense = entries
        .iter()
        .find(|entry| entry["name"] == "StringCaseSense")
        .expect("the name is in the report");
    assert_eq!(string_case_sense["state"], "value");
    // The `Transform` is carried: the command answers the math of the v1.
    let transform = entries
        .iter()
        .find(|entry| entry["name"] == "Transform")
        .expect("the name is in the report");
    assert_eq!(transform["state"], "value");
    // The `VarSetCapacity` is carried: the function empties the variable of
    // the caller.
    let var_set_capacity = entries
        .iter()
        .find(|entry| entry["name"] == "VarSetCapacity")
        .expect("the name is in the report");
    assert_eq!(var_set_capacity["state"], "value");
    // The `IfInString` is carried: the command of the v1 that has a body runs
    // its action when the text of the needle is in the variable.
    let if_in_string = entries
        .iter()
        .find(|entry| entry["name"] == "IfInString")
        .expect("the name is in the report");
    assert_eq!(if_in_string["state"], "value");
    // The `IfNotInString` is carried: it is the other side of the
    // `IfInString`.
    let if_not_in_string = entries
        .iter()
        .find(|entry| entry["name"] == "IfNotInString")
        .expect("the name is in the report");
    assert_eq!(if_not_in_string["state"], "value");
    // The `IfMsgBox` is carried: the command holds when the person pressed
    // the button that is named.
    let if_msg_box = entries
        .iter()
        .find(|entry| entry["name"] == "IfMsgBox")
        .expect("the name is in the report");
    assert_eq!(if_msg_box["state"], "value");
    // The `Else` is carried: it is the action of the other side of the `if`
    // that comes before it.
    let else_of_the_v1 = entries
        .iter()
        .find(|entry| entry["name"] == "Else")
        .expect("the name is in the report");
    assert_eq!(else_of_the_v1["state"], "value");
    // A name that is still to do is pending, and it carries the reason.
    let set_batch_lines = entries
        .iter()
        .find(|entry| entry["name"] == "SetBatchLines")
        .expect("the name is in the report");
    assert_eq!(set_batch_lines["state"], "pending");
    assert!(set_batch_lines["reason"].is_string());
}

/// The names of the surface: the rows of the table, without the comments
/// and without the line of column names.
fn names_of_the_surface() -> usize {
    SURFACE
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#') && !line.starts_with("kind\t"))
        .count()
}

#[test]
fn the_surface_flag_answers_without_a_script() {
    let (code, text) = surface();
    assert_eq!(code, 0);
    assert!(
        !text.trim_start().starts_with('{'),
        "the flag answered with a report of a script and not with the surface"
    );
    assert!(text.contains("Abs function value"), "the surface is not in the report");
}

#[test]
fn the_surface_has_a_line_for_every_name_of_v2() {
    let (_code, text) = surface();
    let lines: Vec<&str> = text.lines().collect();
    let names = names_of_the_surface();
    // The last line is the summary; the ones before it are the names.
    assert_eq!(
        lines.len(),
        names + 1,
        "the surface has {names} names and the report has {} lines",
        lines.len()
    );
    assert!(
        lines[lines.len() - 1].contains("the surface of v2 has"),
        "the report has no summary: {:?}",
        lines[lines.len() - 1]
    );
}

#[test]
fn a_name_that_the_tree_carries_is_reported_as_a_value() {
    let (_code, text) = surface();
    assert!(
        text.lines().any(|line| line == "Abs function value"),
        "the name that the tree answers is not reported as a value"
    );
    // A name that the tree refuses is reported with the reason of the
    // refusal, and the reason is the one of the tree.
    let line = text
        .lines()
        .find(|line| line.starts_with("ControlSendText command "))
        .expect("the name is in the report");
    match ahk::builtins::refusal_of("ControlSendText") {
        Some(reason) => assert_eq!(line, format!("ControlSendText command refusal: {reason}")),
        None => assert_eq!(line, "ControlSendText command value"),
    }
}
