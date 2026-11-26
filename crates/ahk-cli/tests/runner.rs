//! The runner: a script on stdin, a report on stdout.
//!
//! These are the cases of the archived Python runner, ported, and the two
//! the archive does not have: the exit codes of the process, and the pid
//! file that makes a runaway script stoppable from outside.
//!
//! The runner is what runs on the lab box. Here the box is a fake: the
//! `Msg` statement is the only one that needs nothing.
//!
//! The script may come from a file as well: the path of the file is the
//! one argument that is not a flag.
//!
//! The `--surface` flag is the other way round: the report is the table
//! of the names of v2, and there is no script.

mod json_doc;

use json_doc::{Json, field_text};

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

/// The runner, as it was built.
const RUNNER: &str = env!("CARGO_BIN_EXE_ahk");

/// The surface of v2.0.19, as the table of it.
const SURFACE: &str = include_str!("../../ahk/reference/ahk-v2.0.32.surface.tsv");

/// Run a script and answer with the exit code and the report.
fn run(source: &str) -> (i32, Json) {
    let mut child = Command::new(RUNNER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let closer = the_dialogs_are_closed(child.id());
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(source.as_bytes())
        .expect("the script goes in");
    let output = child.wait_with_output().expect("the runner finishes");
    closer();
    let text = String::from_utf8(output.stdout).expect("the report is utf-8");
    let report = Json::parse(text.trim());
    (output.status.code().unwrap_or(-1), report)
}

/// Close the message boxes of a child of the tests, while it runs.
///
/// The `MsgBox` of the runner is the real box of the machine: it waits
/// for a person, and a test has no person. A thread of the test watches
/// the windows of the child (and only the ones of the child: the tests
/// run side by side, and the box of one is not the box of another) and
/// closes the boxes of the class of the dialog, as a click on the close
/// button would. The answer of the box is the one of the close, and the
/// report carries the text that was said.
#[cfg(windows)]
fn the_dialogs_are_closed(pid: u32) -> impl FnOnce() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    std::thread::spawn(move || {
        use windows_sys::Win32::Foundation::{HWND, LPARAM};
        use windows_sys::core::BOOL;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            EnumWindows, GetClassNameW, GetWindowThreadProcessId, PostMessageW, WM_CLOSE,
        };
        unsafe extern "system" fn visit(window: HWND, lparam: LPARAM) -> BOOL {
            let pid = unsafe { *(lparam as *const u32) };
            let mut owner = 0u32;
            unsafe { GetWindowThreadProcessId(window, &mut owner) };
            if owner == pid {
                let mut class = [0u16; 16];
                let read = unsafe { GetClassNameW(window, class.as_mut_ptr(), class.len() as i32) };
                let name = String::from_utf16_lossy(&class[..read as usize]);
                if name == "#32770" {
                    unsafe { PostMessageW(window, WM_CLOSE, 0, 0) };
                }
            }
            1
        }
        while !flag.load(Ordering::Relaxed) {
            unsafe { EnumWindows(Some(visit), &pid as *const u32 as LPARAM) };
            std::thread::sleep(Duration::from_millis(50));
        }
    });
    move || stop.store(true, Ordering::Relaxed)
}

/// On a machine that is not Windows the box of the lab writes to the
/// report and waits for no one: there is nothing to close.
#[cfg(not(windows))]
fn the_dialogs_are_closed(_pid: u32) -> impl FnOnce() {
    || {}
}

/// A script names the dialect it needs, and it runs in it.
///
/// Measured on the binaries of both dialects: the `#Requires` directive names
/// the version of the interpreter that the script needs, and the version
/// names the dialect -- the `v1.1` is the v1 and the `v2.0` is the v2. The
/// `FileAppend` of the v1 is a command and the one of the v2 is a call, so
/// the script below only runs in the dialect it asks for.
///
/// The quotes of the text of the `FileAppend` of the v1 stay in it: measured
/// on the binary of the v1.1.37.02, the command writes the text as it was
/// written.
#[test]
fn a_script_that_names_its_dialect_runs_in_it() {
    let (code, report) = run("#Requires AutoHotkey v1.1\nFileAppend, \"the v1 ran\", *\n");
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("\"the v1 ran\"")
    );
}

/// A script that names nothing keeps the dialect of the tree.
///
/// The `FileAppend` of the v2 is a call, and the one of the v1 is a command:
/// the script below is refused in the v1 and runs in the v2.
#[test]
fn a_script_that_names_no_dialect_runs_in_the_v2() {
    let (code, report) = run("FileAppend(\"the v2 ran\", \"*\")\n");
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("the v2 ran")
    );
}

/// The version of the v2.1 names the v2.1, and the mechanisms of it are
/// carried.
///
/// The `??=` is a mechanism of the v2.1, and the v2.0 refuses it at the load
/// with "Syntax error.".
#[test]
fn a_script_that_names_the_v2_1_runs_in_it() {
    let (code, report) = run("#Requires AutoHotkey v2.1-alpha.16\nz := unset\nz ??= 5\n");
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
}

#[test]
fn a_script_answers_with_a_report() {
    let (code, report) = run("MsgBox \"hello from the box\"\n");
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("hello from the box")
    );
}

#[test]
fn a_script_that_does_not_parse_answers_with_the_line() {
    let (code, report) = run("MsgBox \"unterminated\n");
    assert_eq!(code, 2);
    assert!(!report.boolean("ok"));
    assert_eq!(report.number("error_line"), 1);
    assert!(report.get("column").is_number());
}

#[test]
fn a_failing_script_answers_with_a_non_zero_exit() {
    let (code, report) = run("x := 1 / 0\n");
    assert_eq!(code, 1);
    assert!(!report.boolean("ok"));
    assert_eq!(report.text("error"), "Divide by zero.");
    assert_eq!(report.number("error_line"), 1);
}

#[test]
fn the_report_carries_the_fields_the_client_reads() {
    let (_code, report) = run("MsgBox \"hello\"\n");
    let fields: Vec<&str> = report.names();
    for field in [
        "ok",
        "steps",
        "output",
        "notes",
        "error",
        "error_line",
        "exit_code",
        "elapsed_ms",
        "variables",
    ] {
        assert!(fields.contains(&field), "the report has no {field}");
    }
}

#[test]
fn the_exit_app_code_is_the_code_of_the_process() {
    // Measured on the binary of v1.1.37.02: the `ExitApp, 3` leaves the
    // code three, and the process ends with it. Measured on the binary of
    // v2.0.32: the `ExitApp(3)` does the same.
    let (code, report) = run("ExitApp 3\n");
    assert_eq!(code, 3);
    assert_eq!(report.number("exit_code"), 3);
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
    assert!(
        !pid_file.exists(),
        "the pid file is removed when the script ends"
    );
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
    let closer = the_dialogs_are_closed(child.id());
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(b"MsgBox \"x\"\n")
        .expect("the script goes in");
    let output = child.wait_with_output().expect("the runner finishes");
    closer();
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
fn surface_json(dialect: &str) -> (i32, Json) {
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
    let report = Json::parse(text.trim());
    (status.code().unwrap_or(-1), report)
}

#[test]
fn the_surface_flag_answers_with_json() {
    let (code, report) = surface_json("v2");
    assert_eq!(code, 0);
    assert_eq!(report.text("dialect"), "v2");
    assert_eq!(report.number("total"), 843);
    assert_eq!(report.number("values"), 843);
    assert_eq!(report.number("refusals"), 0);
    assert_eq!(report.number("pending"), 0);

    let entries = report.array("entries");
    assert_eq!(entries.len(), 843);
    // A name that the tree answers is a value, and the surface of the v2
    // has no refusal left: the last names of the block of fifteen -- the
    // styles of a window, `Edit`, `Reload`, `Thread`, `A_Cursor` and
    // `A_ScriptHwnd` -- were carried, each one answered by
    // the desktop of the machine or by the runtime of the tree.
    let abs = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "Abs")
        .expect("the name is in the report");
    assert_eq!(field_text(abs, "kind"), "function");
    assert_eq!(field_text(abs, "state"), "value");
    // The name that refuses is one of the two the runtime cannot do yet:
    // `Reload` is the script that runs once, and `Thread` is the thread the
    // runtime does not start. The style names were carried,
    // when the backends answered the style of a window: the X11 maps its
    // attributes onto the bits, and Windows reads them. The last two names
    // of the block of fifteen -- `Reload` and `Thread` -- were carried the
    // same day: the runner starts the script again, and the command writes
    // the state of the thread. The surface has no refusal left, so the
    // check is that the two names answer.
    let windows = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "ControlGetStyle")
        .expect("the name is in the report");
    assert_eq!(field_text(windows, "state"), "value");
    let reload = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "Reload")
        .expect("the name is in the report");
    assert_eq!(field_text(reload, "state"), "value");
    let thread = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "Thread")
        .expect("the name is in the report");
    assert_eq!(field_text(thread, "state"), "value");
}

#[test]
fn the_surface_flag_answers_with_the_json_of_the_v1() {
    let (code, report) = surface_json("v1");
    assert_eq!(code, 0);
    assert_eq!(report.text("dialect"), "v1");
    assert_eq!(report.number("total"), 474);
    // The names that the v1 has and the v2 does not are carried, and none of
    // them is pending: the surface of the v1 is complete.
    //
    // The count moved, when the names that were the example of
    // the blanket reason of the names of Windows were carried: the machine
    // says where its system is, and the raven is a Windows of its own for the
    // box. `A_WinDir`, `A_ProgramFiles`, `A_IconFile`, `A_IconNumber` and
    // `A_ScreenDPI` answer a value where they refused.
    // The count moved, twice: the names that the table of the
    // v1 refused were carried, and then the fifteen names that only the v1
    // has landed with it -- the `Gui`, the `Menu`, the `Progress`, the
    // `Control`, the `Drive`, the `ComObjCreate` and the rest -- and the
    // two variables that the machine does not have (`A_OSType` and
    // `A_IPAddress1`) are answered by the machine.
    // The count moved, when the names that the script of the
    // a script of the v1 uses were carried: the commands of the comparison
    // (`IfEqual`, `IfExist`, `IfGreater`, `IfGreaterOrEqual`, `IfLess`,
    // `IfLessOrEqual` and `IfNotEqual`), the functions of the COM
    // (`ComObject` and `Exception`) and the ones of the keys (`GetKeyName`,
    // `GetKeySC` and `GetKeyVK`).
    // The count moved, when the four commands that the table of the surface
    // of the v1 did not carry landed with it: the `ControlSendRaw`, the
    // `SetCapsLockState`, the `SetNumLockState` and the
    // `WinMinimizeAllUndo`.
    // The count moved, when the `SplashTextOff` landed with it: it is a
    // command of the v1 that the table of the surface did not carry.
    // The count moved, when the seven names that the table of the surface of
    // the v1 did not carry landed with it: the `ComObj`, the
    // `ComObjMissing`, the `ComObjParameter`, the `ObjRawGet`, the
    // `OnClipboardChange`, the `SB_SetParts` and the `SB_SetText`.
    // The count moved, when the `SendRaw` command landed in the v1 surface.
    // The count moved, when the `WinWaitNotActive` landed with it: it is a
    // command of the v1 that the table of the surface did not carry.
    assert_eq!(report.number("values"), 474);
    assert_eq!(report.number("refusals"), 0);
    assert_eq!(report.number("pending"), 0);
    let entries = report.array("entries");
    let is_set = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "IsSet")
        .expect("the name is in the report");
    assert_eq!(field_text(is_set, "state"), "value");
    // The `AutoTrim` is carried: the command turns the trim of the text of a
    // legacy assignment on and off.
    let auto_trim = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "AutoTrim")
        .expect("the name is in the report");
    assert_eq!(field_text(auto_trim, "state"), "value");
    // The `SetFormat` is carried: the command turns the format of the
    // numbers of the v1.
    let set_format = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "SetFormat")
        .expect("the name is in the report");
    assert_eq!(field_text(set_format, "state"), "value");
    // The `StringCaseSense` is carried: the command turns the sense of the
    // case of the comparisons on and off.
    let string_case_sense = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "StringCaseSense")
        .expect("the name is in the report");
    assert_eq!(field_text(string_case_sense, "state"), "value");
    // The `Transform` is carried: the command answers the math of the v1.
    let transform = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "Transform")
        .expect("the name is in the report");
    assert_eq!(field_text(transform, "state"), "value");
    // The `VarSetCapacity` is carried: the function empties the variable of
    // the caller.
    let var_set_capacity = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "VarSetCapacity")
        .expect("the name is in the report");
    assert_eq!(field_text(var_set_capacity, "state"), "value");
    // The `IfInString` is carried: the command of the v1 that has a body runs
    // its action when the text of the needle is in the variable.
    let if_in_string = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "IfInString")
        .expect("the name is in the report");
    assert_eq!(field_text(if_in_string, "state"), "value");
    // The `IfNotInString` is carried: it is the other side of the
    // `IfInString`.
    let if_not_in_string = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "IfNotInString")
        .expect("the name is in the report");
    assert_eq!(field_text(if_not_in_string, "state"), "value");
    // The `IfMsgBox` is carried: the command holds when the person pressed
    // the button that is named.
    let if_msg_box = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "IfMsgBox")
        .expect("the name is in the report");
    assert_eq!(field_text(if_msg_box, "state"), "value");
    // The `Else` is carried: it is the action of the other side of the `if`
    // that comes before it.
    let else_of_the_v1 = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "Else")
        .expect("the name is in the report");
    assert_eq!(field_text(else_of_the_v1, "state"), "value");
    // The `SetBatchLines` is carried: it is the command that turns the lines
    // of the script on and off, and the v1 wants a number and not a flag.
    let set_batch_lines = entries
        .iter()
        .find(|entry| field_text(entry, "name") == "SetBatchLines")
        .expect("the name is in the report");
    assert_eq!(field_text(set_batch_lines, "state"), "value");
    // No name of the v1 is pending.
    assert!(
        entries
            .iter()
            .all(|entry| field_text(entry, "state") != "pending"),
        "the surface of the v1 has a name that is pending"
    );
}

/// The names of the surface: the rows of the table, without the comments
/// and without the line of column names.
fn names_of_the_surface() -> usize {
    SURFACE
        .lines()
        .filter(|line| {
            !line.trim().is_empty() && !line.starts_with('#') && !line.starts_with("kind\t")
        })
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
    assert!(
        text.contains("Abs function value"),
        "the surface is not in the report"
    );
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

// -- #Include and #IncludeAgain ----------------------------------------------

/// A directory of the test, made under the temporary directory.
fn a_test_directory(name: &str) -> std::path::PathBuf {
    let directory = std::env::temp_dir().join(format!("ahk-runner-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test directory is made");
    directory
}

#[test]
fn an_include_is_put_in_once() {
    let directory = a_test_directory("include-once");
    let included = directory.join("included.ahk");
    std::fs::write(&included, "MsgBox \"included\"\n").expect("the included file is written");
    let script = format!(
        "#Include \"{}\"\n#Include \"{}\"\nMsgBox \"main\"\n",
        included.display(),
        included.display()
    );
    let (code, report) = run(&script);
    assert_eq!(code, 0);
    let output = report.array("output");
    assert_eq!(output.len(), 2, "the file is put in once");
    assert_eq!(output[0].as_str(), Some("included"));
    assert_eq!(output[1].as_str(), Some("main"));
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn an_include_again_is_put_in_every_time() {
    let directory = a_test_directory("include-again");
    let included = directory.join("included.ahk");
    std::fs::write(&included, "MsgBox \"included\"\n").expect("the included file is written");
    let script = format!(
        "#IncludeAgain \"{}\"\n#IncludeAgain \"{}\"\nMsgBox \"main\"\n",
        included.display(),
        included.display()
    );
    let (code, report) = run(&script);
    assert_eq!(code, 0);
    let output = report.array("output");
    assert_eq!(
        output.len(),
        3,
        "the file is put in every time it is asked for"
    );
    assert_eq!(output[0].as_str(), Some("included"));
    assert_eq!(output[1].as_str(), Some("included"));
    assert_eq!(output[2].as_str(), Some("main"));
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn an_include_again_after_an_include_puts_the_file_in_again() {
    // Measured on the binary of v2.0.19: the `#IncludeAgain`
    // puts the file in even when the file is already in the tree.
    let directory = a_test_directory("include-again-after");
    let included = directory.join("included.ahk");
    std::fs::write(&included, "MsgBox \"included\"\n").expect("the included file is written");
    let script = format!(
        "#Include \"{path}\"\n#IncludeAgain \"{path}\"\nMsgBox \"main\"\n",
        path = included.display()
    );
    let (code, report) = run(&script);
    assert_eq!(code, 0);
    assert_eq!(report.array("output").len(), 3);
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn an_include_that_includes_itself_is_refused() {
    let directory = a_test_directory("include-itself");
    let file = directory.join("itself.ahk");
    std::fs::write(&file, format!("#IncludeAgain \"{}\"\n", file.display()))
        .expect("the file is written");
    let script = format!("#IncludeAgain \"{}\"\n", file.display());
    let (code, report) = run(&script);
    assert_eq!(code, 2);
    assert!(!report.boolean("ok"));
    assert!(
        report.text("error").contains("nest more than"),
        "{:?}",
        report.text("error")
    );
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn an_include_again_without_a_path_is_refused() {
    // Measured on the binary of the v2.0.32: the directive with no parameter
    // is refused with "Parameter #1 required" (`ERR_PARAM1_REQUIRED`).
    let (code, report) = run("#IncludeAgain\n");
    assert_eq!(code, 2);
    assert!(report.text("error").contains("Parameter #1 required"));
}

/// An `#Include` that is inside a comment is not an include.
///
/// Measured on the binary of the v2.0.32: the script that has an
/// `#Include` inside a `/* ... */` block loads, and the directive of the
/// block is of no account.
#[test]
fn an_include_inside_a_comment_is_not_an_include() {
    let (code, report) = run("/*\n#Include <adash>\n*/\nFileAppend(\"the comment is not an include\", \"*\")\n");
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("the comment is not an include")
    );
}

/// A library that is not there is refused with the words of the official.
///
/// Measured on the binary of the v2.0.32: the script that asks for a
/// library that is not there is refused with "Script library not found."
/// and the line of the directive after it.
#[test]
fn a_library_that_is_not_there_is_refused_with_the_words_of_the_official() {
    let (code, report) = run("FileAppend(\"x\", \"*\")\n\n#Include <adash>\n");
    assert_eq!(code, 2);
    assert!(!report.boolean("ok"));
    assert_eq!(
        report.text("error"),
        "Script library not found.\n     Specifically: #Include <adash>"
    );
    assert_eq!(report.number("error_line"), 3);
}

/// A file that is not there is refused with the words of the official.
///
/// Measured on the binary of the v2.0.32: the file that cannot be opened is
/// refused with the path as the script wrote it, and the line of the
/// directive is the line of the refusal.
#[test]
fn a_file_that_is_not_there_is_refused_with_the_words_of_the_official() {
    let (code, report) = run("FileAppend(\"x\", \"*\")\n#Include \"nope.ahk\"\n");
    assert_eq!(code, 2);
    assert!(!report.boolean("ok"));
    assert_eq!(
        report.text("error"),
        "#Include file \"nope.ahk\" cannot be opened."
    );
    assert_eq!(report.number("error_line"), 2);
}

#[test]
fn the_report_of_the_capability_says_the_numbers_of_the_tree() {
    let (_, report) = surface_json("v2");
    let document = include_str!("../../../docs/v2-capability-report.md");
    for (number, what) in [
        (report.number("total"), "entries"),
        (report.number("values"), "answer with a value"),
        (report.number("refusals"), "are refused"),
    ] {
        let wanted = format!("**{number} {what}**");
        assert!(
            document.contains(&wanted),
            "the report of the capability does not say {wanted}"
        );
    }
}

// -- A script that comes from a file ------------------------------------------

/// Run the script of a file and answer with the exit code and the report.
///
/// The standard input is closed and empty: a runner that reads the script
/// from there answers with a report of nothing, and the case is that the
/// file is the script.
fn run_file(path: &std::path::Path) -> (i32, Json) {
    let child = Command::new(RUNNER)
        .arg(path)
        .current_dir(std::env::temp_dir())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let closer = the_dialogs_are_closed(child.id());
    let output = child.wait_with_output().expect("the runner finishes");
    closer();
    let text = String::from_utf8(output.stdout).expect("the report is utf-8");
    (output.status.code().unwrap_or(-1), Json::parse(text.trim()))
}

#[test]
fn a_script_from_a_file_answers_with_a_report() {
    let directory = a_test_directory("script-from-a-file");
    let script = directory.join("main.ahk");
    std::fs::write(&script, "MsgBox \"hello from the file\"\n").expect("the file is written");
    let (code, report) = run_file(&script);
    assert_eq!(code, 0);
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("hello from the file")
    );
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn a_file_that_cannot_be_read_is_a_failure_of_the_load() {
    let directory = a_test_directory("file-that-cannot-be-read");
    let script = directory.join("there-is-no-such-file.ahk");
    let (code, report) = run_file(&script);
    assert_eq!(code, 2);
    assert!(!report.boolean("ok"));
    let error = report.text("error");
    assert!(
        error.contains("cannot be read"),
        "the reason is not the one of a file that cannot be read: {error}"
    );
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn the_includes_of_a_script_from_a_file_are_resolved_against_its_directory() {
    // The include names a file of the directory of the script, and the
    // runner runs in another directory: the include is found because the
    // base is the directory of the file of the script.
    let directory = a_test_directory("includes-of-a-file");
    let included = directory.join("included.ahk");
    std::fs::write(&included, "MsgBox \"included\"\n").expect("the included file is written");
    let script = directory.join("main.ahk");
    std::fs::write(&script, "#Include \"included.ahk\"\nMsgBox \"main\"\n")
        .expect("the file is written");
    let (code, report) = run_file(&script);
    assert_eq!(code, 0);
    let output = report.array("output");
    assert_eq!(output.len(), 2, "the include is not in the script");
    assert_eq!(output[0].as_str(), Some("included"));
    assert_eq!(output[1].as_str(), Some("main"));
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn a_relative_path_is_joined_with_the_working_directory() {
    let directory = a_test_directory("relative-path");
    let script = directory.join("main.ahk");
    std::fs::write(&script, "MsgBox \"hello from the file\"\n").expect("the file is written");
    let child = Command::new(RUNNER)
        .arg("main.ahk")
        .current_dir(&directory)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let closer = the_dialogs_are_closed(child.id());
    let output = child.wait_with_output().expect("the runner finishes");
    closer();
    assert_eq!(output.status.code().unwrap_or(-1), 0);
    let text = String::from_utf8(output.stdout).expect("the report is utf-8");
    let report = Json::parse(text.trim());
    assert!(report.boolean("ok"));
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("hello from the file")
    );
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn a_run_without_a_path_still_reads_the_standard_input() {
    // The behaviour of the runner that reads the standard input is kept
    // as it was: the path of the script is optional.
    let (code, report) = run("MsgBox \"hello from the pipe\"\n");
    assert_eq!(code, 0);
    assert_eq!(
        report.array("output")[0].as_str(),
        Some("hello from the pipe")
    );
}

#[test]
fn the_help_says_that_the_script_may_come_from_a_file() {
    let child = Command::new(RUNNER)
        .arg("--help")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    let output = child.wait_with_output().expect("the runner finishes");
    assert_eq!(output.status.code().unwrap_or(-1), 0);
    let text = String::from_utf8(output.stdout).expect("the help is utf-8");
    assert!(
        text.contains("file"),
        "the help does not say that the script may come from a file: {text}"
    );
}
