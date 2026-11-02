//! The legacy dialect, against a fake desktop.
//!
//! The v1 has a grammar of its own and a table of its own, and these
//! cases are about what the dialect means: the command whose parameters
//! are texts, the equal sign that assigns, and the command that writes
//! into a variable of the script.
//!
//! The cases were measured on the binary of AutoHotkey v1.1.37.02 on
//! 2026-11-01, and the probes are in `egm-review/probe_v1_semantics.py`,
//! in `egm-review/probe_v1_semantics2.py` and in
//! `egm-review/probe_v1_semantics3.py`.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer,
    Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::dll::{NativeCall, NativeFailure, NativeResult, NativeValue};
use ahk::runtime::{Report, Runtime, Value};
use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Duration;

/// The turn of the tests: the settings of the script live in a static, and
/// the runtime writes the defaults of its dialect when it is made. Two cases
/// that run a script cannot run at the same time, or the one that reads a
/// setting reads the one that the other wrote.
static THE_TURN: Mutex<()> = Mutex::new(());

/// A desktop that does nothing: the cases of the dialect do not need the
/// machine, and the methods that a capability does not have answer that
/// they do not have it.
struct Fake {
    /// What is on the clipboard of the fake.
    clipboard: String,
    /// What the machine was asked, in the order of the questions.
    calls: RefCell<Vec<String>>,
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}
impl Clipboard for Fake {
    fn clipboard_read(&mut self) -> Result<String, String> {
        // The cases of the `ClipWait` put nothing on the clipboard, and the
        // one that waits for the data puts something there.
        Ok(self.clipboard.clone())
    }
}
impl Files for Fake {
    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        // The cases that read a file write one that is there.
        let text = std::fs::read_to_string(path)
            .map_err(|reason| format!("{reason}"))?;
        Ok(text.lines().map(|line| line.to_string()).collect())
    }

    fn append_file(&mut self, _path: &str, _bytes: &[u8], _mark: &[u8]) -> Result<String, String> {
        Ok(String::new())
    }

    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        self.calls.borrow_mut().push(format!("delete_file {path}"));
        // The name that is not there is the one that the cases use for the
        // command that fails.
        if path.contains("nothere") {
            return Err("the file is not there".into());
        }
        Ok(String::new())
    }

    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        self.calls.borrow_mut().push(format!("copy_file {from} {to} {overwrite}"));
        if from.contains("nothere") {
            return Err("the file is not there".into());
        }
        Ok(String::new())
    }

    fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        self.calls.borrow_mut().push(format!("move_file {from} {to} {overwrite}"));
        if from.contains("nothere") {
            return Err("the file is not there".into());
        }
        Ok(String::new())
    }

    fn list_files(&mut self, _folder: &str, pattern: &str, _mode: &str) -> Result<Vec<String>, String> {
        self.calls.borrow_mut().push(format!("list_files {pattern}"));
        // The pattern with the wildcard is the one of the cases that want
        // many files, and the one with the "bin" is the one whose file cannot
        // be copied. The pattern that is not there matches nothing.
        if pattern.contains("nothere") {
            return Ok(Vec::new());
        }
        if pattern.contains("*") {
            if pattern.contains("bin") {
                return Ok(vec!["C:\\work\\nothere.bin".to_string()]);
            }
            return Ok(vec!["C:\\work\\a.txt".to_string(), "C:\\work\\b.txt".to_string()]);
        }
        Ok(vec![format!("C:\\work\\{pattern}")])
    }

    fn working_directory(&mut self) -> Result<String, String> {
        Ok("C:\\work".to_string())
    }
}
impl Hotkeys for Fake {}
impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        // The names that are not there are the ones that the cases use for
        // the two failures of the machine: the library that is not there and
        // the function that is not in it. The call that works answers with
        // seven.
        if call.library.contains("nolibrary") {
            return Err(NativeFailure::no_library("Failed to load DLL."));
        }
        if call.function.contains("nofunction") {
            return Err(NativeFailure::no_function("Call to nonexistent function."));
        }
        Ok(NativeResult { kind: call.returns, value: NativeValue::Integer(7) })
    }
}
impl Timers for Fake {}
impl Windows for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Err("this fake has no pointer".into())
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        // The name that is not there is the one that the cases use for the
        // command that fails, and the program that runs answers with seven.
        if command.contains("nothere") {
            return Err("this fake has no processes".into());
        }
        Ok((7, String::new(), String::new()))
    }

    fn launch(&mut self, command: &str, _working_dir: &str) -> Result<u32, String> {
        // The `Run` of the v1 starts the program and goes on, and the pid of
        // the process is what the fake answers with.
        if command.contains("nothere") {
            return Err("this fake has no processes".into());
        }
        Ok(4242)
    }

    fn run_and_wait(
        &mut self,
        command: &str,
        _working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        if command.contains("nothere") {
            return Err("this fake has no processes".into());
        }
        Ok((4242, 7, String::new(), String::new()))
    }
}

impl Screen for Fake {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Err("this fake has no screen".into())
    }

    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this fake has no screen".into())
    }
}

/// Run a script of the v1 and answer with the report.
fn run(script: &str) -> Report {
    run_with_the_clipboard(script, "")
}

/// Run a script of the v1 with something on the clipboard, and answer with
/// the report.
fn run_with_the_clipboard(script: &str, clipboard: &str) -> Report {
    // The settings of the script live in a static: the case holds the turn
    // while it runs, so that no other case writes them under its feet.
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake { clipboard: clipboard.to_string(), calls: RefCell::new(Vec::new()) };
    Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements)
}

/// Run a script of the v1 and answer with the report and with what the
/// machine was asked.
///
/// The cases that want to know what the machine was asked -- the ones of the
/// patterns of the files -- need the questions of the fake.
fn run_and_the_calls(script: &str) -> (Report, Vec<String>) {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake { clipboard: String::new(), calls: RefCell::new(Vec::new()) };
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    let calls = desktop.calls.into_inner();
    (report, calls)
}

/// The refusal of the load of a script of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the value of a
/// parameter of a command of the v1 is checked when the script loads, and
/// the script does not run at all.
fn the_refusal_of_the_load(script: &str) -> String {
    let error = ahk::parse_with(script, dialect::v1()).expect_err("the script of the v1 is refused");
    error.message
}

/// The command of the v1 writes the text that it was given.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `FileAppend,
/// hello, *` writes `hello`.
#[test]
fn the_command_of_the_v1_writes_the_text_that_it_was_given() {
    let report = run("FileAppend, hello, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["hello".to_string()]);
}

/// The quotes of the text of a command are part of the text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `FileAppend,
/// "hello", *` writes `"hello"`.
#[test]
fn the_quotes_of_the_text_of_a_command_are_part_of_the_text() {
    let report = run("FileAppend, \"hello\", *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["\"hello\"".to_string()]);
}

/// The equal sign of the v1 assigns.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `x = 5` leaves `x`
/// at 5.
#[test]
fn the_equal_sign_of_the_v1_assigns() {
    let report = run("x = 5");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(5)));
}

/// The substitution of a parameter is the variable of the script.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `FileAppend, %x%,
/// *` writes the value of `x`.
#[test]
fn the_substitution_of_a_parameter_is_the_variable_of_the_script() {
    let report = run("x := 5\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);
}

/// The second parameter of `StringLen` is the name of a variable.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `StringLen, n,
/// hello` is five when `hello` is a variable of the script, and zero when
/// it is not.
#[test]
fn the_second_parameter_of_the_string_length_is_the_name_of_a_variable() {
    let report = run("hello := \"world\"\nStringLen, n, hello");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(5)));

    let report = run("StringLen, n, hello");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(0)));
}

/// The name of a command that the dialect does not have is a line that
/// holds no action.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: the line is
/// refused at the load, and it names the line.
#[test]
fn the_name_of_a_command_that_the_dialect_does_not_have_is_refused_at_the_load() {
    let error = ahk::parse_with("NoSuchCommand, 1", dialect::v1())
        .expect_err("the line holds no action");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: NoSuchCommand, 1"
    );
}

/// The division by zero of the v1 is a blank result.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `1/0` is blank,
/// and it is not a failure.
#[test]
fn the_division_by_zero_of_the_v1_is_a_blank_result() {
    let report = run("x := 1/0");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

/// The sum of a text and a number is a concatenation.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `"5" + 1` is
/// `51`, and the sum of two numbers is a number.
#[test]
fn the_sum_of_a_text_and_a_number_is_a_concatenation() {
    let report = run("x := \"5\" + 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("51".into())));

    let report = run("x := 5 + 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(6)));
}

/// A variable of the v1 that has no value is blank.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: the variable is
/// blank, and it is not a failure.
#[test]
fn a_variable_of_the_v1_that_has_no_value_is_blank() {
    let report = run("x := nosuchvariable");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

/// The substitution of a parameter is the variable of the script, and the
/// environment is what is left.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: the variable of
/// the script comes first, and the directive `#NoEnv` turns the lookup of
/// the environment off.
#[test]
fn the_substitution_of_a_parameter_is_the_variable_of_the_script_and_then_the_environment() {
    let report = run("x := 5\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);

    // The environment is what is left when the script has no variable of
    // that name.
    let report = run("FileAppend, %AHK_A_VARIABLE_THAT_IS_NOT_THERE%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec![String::new()]);

    // The directive turns the lookup of the environment off, and the
    // variable of the script is still there.
    let report = run("#NoEnv\nx := 5\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);
}

/// The count of the loop of the v1 comes after a comma.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: `Loop, 3` is a
/// loop of three turns.
#[test]
fn the_count_of_the_loop_of_the_v1_comes_after_a_comma() {
    let report = run("Loop, 3\n    FileAppend, [turn], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[turn]".to_string(), "[turn]".to_string(), "[turn]".to_string()]);
}

/// The body of a construct of the v1 is one statement.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-01: the `if` takes the
/// statement that comes after it, and not a block.
#[test]
fn the_body_of_a_construct_of_the_v1_is_one_statement() {
    let report = run("if (1)\n    FileAppend, yes, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["yes".to_string()]);

    let report = run("if (0)\n    FileAppend, no, *\nelse\n    FileAppend, else, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["else".to_string()]);
}

/// The built-in variables of the v1 answer with the value that the binary
/// answers.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `A_AutoTrim` is
/// `On`, `A_FormatFloat` is `0.6`, `A_DefaultGui` is `1` and `A_IsUnicode`
/// is `1`.
#[test]
fn the_built_in_variables_of_the_v1_answer_with_the_value_that_the_binary_answers() {
    let report = run("x := A_AutoTrim");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));

    let report = run("x := A_FormatFloat");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0.6".into())));

    let report = run("x := A_FormatInteger");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("D".into())));

    let report = run("x := A_DefaultGui");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("x := A_IsUnicode");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));
}

/// The two variables of the v1 that are about the machine are refused with
/// the reason.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `A_OSType` is
/// `WIN32_NT`, and the `A_IPAddress1` is the address of the machine, and
/// this runtime answers neither.
#[test]
fn the_variables_of_the_v1_that_are_about_the_machine_are_refused_with_the_reason() {
    let report = run("x := A_OSType");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("this name is about Windows, and this machine is not."));

    let report = run("x := A_IPAddress1");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this desktop does not know the address of the network of the machine.")
    );
}

/// The write of a built-in variable of the v1 is refused at the load.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write of
/// `A_AhkPath` is refused with "Not allowed as an output variable.", and the
/// refusal is at the load: the script does not run. The reason is the same
/// for every operator.
#[test]
fn the_write_of_a_built_in_variable_of_the_v1_is_refused_at_the_load() {
    for script in [
        "A_AhkPath := 5",
        "A_AhkPath += 5",
        "A_AhkPath -= 5",
        "A_AhkPath *= 5",
        "A_AhkPath /= 5",
    ] {
        let error = ahk::parse_with(script, dialect::v1()).expect_err("the write is refused");
        assert_eq!(
            error.message,
            "Not allowed as an output variable.\n     Specifically: A_AhkPath"
        );
    }
}

/// The refusal names the variable as the script spelled it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write of
/// `a_ahkpath` is refused with the name that the script wrote.
#[test]
fn the_refusal_of_the_v1_names_the_variable_as_the_script_spelled_it() {
    let error =
        ahk::parse_with("a_ahkpath := 5", dialect::v1()).expect_err("the write is refused");
    assert_eq!(
        error.message,
        "Not allowed as an output variable.\n     Specifically: a_ahkpath"
    );
}

/// Every built-in variable of the v1 is read-only.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the names that the v2
/// lets a script write are refused as well -- the `A_Index` and the
/// `A_ListLines` -- and so are the names that the v1 has and the v2 does not
/// -- the `A_AutoTrim` -- and the ones that the v1 spells without the prefix
/// -- the `ComSpec`.
#[test]
fn every_built_in_variable_of_the_v1_is_read_only() {
    for name in [
        "A_Index",
        "A_ListLines",
        "A_WorkingDir",
        "A_ScriptName",
        "A_EventInfo",
        "A_LastError",
        "A_IconTip",
        "A_RegView",
        "A_CoordModePixel",
        "A_AutoTrim",
        "A_Gui",
        "A_OSType",
        "A_IsUnicode",
        "A_FormatInteger",
        "A_ThisLabel",
        "A_IPAddress1",
        "ComSpec",
        "ProgramFiles",
    ] {
        let script = format!("{name} := 5");
        let error = ahk::parse_with(&script, dialect::v1()).expect_err("the write is refused");
        assert_eq!(
            error.message,
            format!("Not allowed as an output variable.\n     Specifically: {name}")
        );
    }
}

/// The clipboard, the last error of the run and the arguments of the script
/// are the built-in variables that the v1 lets a script write.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write of the
/// `ErrorLevel` and the write of the `A_Args` are kept.
#[test]
fn the_names_that_the_v1_lets_a_script_write_are_kept() {
    let report = run("A_Args := \"written\"\nx := A_Args");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("written".into())));

    let report = run("ErrorLevel := 5\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));
}

/// The concat on a built-in variable of the v1 is accepted, and the built-in
/// keeps the value that it had.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write
/// `A_AutoTrim .= "y"` is not a failure, and the `A_AutoTrim` is still `On`.
#[test]
fn the_concat_on_a_built_in_variable_of_the_v1_is_accepted() {
    let report = run("A_AutoTrim .= \"y\"\nx := A_AutoTrim");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));
}

/// The concat on a setting of the v1 is accepted, and the setting keeps the
/// value that it had.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write
/// `A_TitleMatchMode .= "y"` is not a failure, and the setting is the mode
/// that it was.
#[test]
fn the_concat_on_a_setting_of_the_v1_is_accepted() {
    let report = run("x := A_TitleMatchMode\nA_TitleMatchMode .= \"y\"\ny := A_TitleMatchMode");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), report.variables.get("y"));
}

/// The increment of a built-in variable of the v1 is refused at the load.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the increment of the
/// `A_AhkPath` is refused with the reason of the write.
#[test]
fn the_increment_of_a_built_in_variable_of_the_v1_is_refused_at_the_load() {
    for script in ["A_AhkPath++", "A_AhkPath--", "A_Index++"] {
        let error = ahk::parse_with(script, dialect::v1()).expect_err("the increment is refused");
        assert_eq!(
            error.message,
            format!(
                "Not allowed as an output variable.\n     Specifically: {}",
                script.trim_end_matches(['+', '-'])
            )
        );
    }
}

/// The space does not separate the operator of the increment from the name.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `x ++` is the
/// increment of the `x`, and the `x --` is the decrement of it. The v2 is the
/// other way round: the space tells the operator of the increment apart from
/// the call.
#[test]
fn the_space_does_not_separate_the_increment_of_the_v1() {
    let report = run("x := 5\nx ++");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(6)));

    let report = run("x := 5\nx --");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(4)));
}

/// The rest of the line after the increment is evaluated, and its value is
/// thrown away.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `x ++ 5` is the
/// increment of the `x`, and the `5` is the list of the v1.
#[test]
fn the_rest_of_the_line_after_the_increment_of_the_v1_is_thrown_away() {
    let report = run("x := 5\nx ++ 5");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(6)));

    let report = run("x := 5\nx -- 5");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(4)));
}

/// The increment of a built-in variable of the v1 is only refused when the
/// line is the increment alone.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `A_AhkPath++ 5`
/// is accepted, and the increment of a built-in variable answers the empty
/// text and changes nothing.
#[test]
fn the_increment_of_a_built_in_variable_of_the_v1_is_refused_only_alone() {
    let report = run("A_AhkPath++ 5");
    assert!(report.ok);

    let report = run("y := A_AutoTrim++\nx := A_AutoTrim");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));
}

/// The `SetBatchLines` of the v1 turns the number of the lines that the script
/// runs at once.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the number is "10ms"
/// when the script begins, and the command turns it into the number that was
/// written -- the "007" is seven, and the hex is a whole number as well.
#[test]
fn the_set_batch_lines_of_the_v1_turns_the_number_of_the_lines() {
    let report = run("x := A_NumBatchLines");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("10ms".into())));

    let report = run("SetBatchLines, 20\nx := A_NumBatchLines");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("20".into())));

    let report = run("SetBatchLines, 007\nx := A_NumBatchLines");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("7".into())));

    let report = run("SetBatchLines, 0x10\nx := A_NumBatchLines");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("16".into())));
}

/// The zero of the batch lines of the v1 is the default, and it is ten.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `SetBatchLines, 0`
/// leaves the number at ten.
#[test]
fn the_zero_of_the_batch_lines_of_the_v1_is_ten() {
    let report = run("SetBatchLines, 0\nx := A_NumBatchLines");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("10".into())));
}

/// The value of the batch lines of the v1 that is not a whole number is
/// refused.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is
/// "Parameter #1 invalid." and the value that failed is in the message. The
/// refusal is at the load: the script does not run.
#[test]
fn the_value_of_the_batch_lines_that_is_not_a_number_is_refused() {
    for value in ["off", "abc", "1+1", "20.5"] {
        assert_eq!(
            the_refusal_of_the_load(&format!("SetBatchLines, {value}")),
            format!("Parameter #1 invalid.\n     Specifically: {value}")
        );
    }
}

/// The command of the v1 that takes one parameter takes the text that is left
/// of the line with it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `AutoTrim, On,
/// extra` is the "On, extra", and it is refused because it is not "On". The
/// refusal is at the load.
#[test]
fn the_extra_text_of_the_line_goes_into_the_parameter_of_the_v1() {
    assert_eq!(
        the_refusal_of_the_load("AutoTrim, On, extra"),
        "Parameter #1 invalid.\n     Specifically: On, extra"
    );
}

/// The directive of the v1 that the runtime already is is accepted, and the
/// one that asks for a thing the runtime does not have is refused with the
/// reason.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `#Persistent`
/// is the case that the runner already is, and the `#MaxMem` asks for a
/// limit of memory that this runtime does not keep.
#[test]
fn the_directive_of_the_v1_is_answered_with_the_reason_of_the_thing_it_asks_for() {
    let report = run("#Persistent\nFileAppend, ok, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["ok".to_string()]);

    let report = run("#MaxMem\nFileAppend, ok, *");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the memory limits are not carried by this runtime."));
}

/// The `IsSet` of the v1 asks for the name of the variable and not for its
/// value.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `IsSet(y)` asks
/// whether the variable `y` has a value, and not whether the value of `y`
/// is the name of a variable, and a built-in variable has a value.
#[test]
fn the_is_set_of_the_v1_asks_for_the_name_of_the_variable() {
    let report = run("x := 5\nn := IsSet(x)");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(1)));

    let report = run("n := IsSet(NoSuchVariable)");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(0)));

    let report = run("n := IsSet(A_AutoTrim)");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(1)));

    // The name of the variable and not its value.
    let report = run("y := \"nosuchname\"\nn := IsSet(y)");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(1)));
}

/// The `IsFunc` of the v1 asks for the value, and answers 1 for a function
/// of the script, 2 for a built-in and 0 for anything else.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02.
#[test]
fn the_is_func_of_the_v1_answers_with_the_kind_of_the_name() {
    let report = run("f() {\n    return 1\n}\nn := IsFunc(\"f\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(1)));

    let report = run("n := IsFunc(\"StrLen\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(2)));

    // A built-in of the v1 that the v2 does not have is a built-in too.
    let report = run("n := IsFunc(\"IsSet\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(2)));

    let report = run("x := 5\nn := IsFunc(\"x\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(0)));
}

/// The object of the v1 has no class.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `__Class` of the
/// object of the v1 is empty, and the literal `{}` is the same object.
#[test]
fn the_object_of_the_v1_has_no_class() {
    let report = run("o := Object()\nx := o.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    let report = run("o := {}\nx := o.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    // A property that is written on it is a property of it.
    let report = run("o := Object()\no.x := 5\ny := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("5".into())));
}

/// The `Func` of the v1 hands the function of the script over as a value.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the value carries
/// the name of the function and the number of its parameters, and the call
/// of it is the call of the function.
#[test]
fn the_func_of_the_v1_hands_the_function_over_as_a_value() {
    let report = run("f(a, b) {\n    return a + b\n}\nfn := Func(\"f\")\nn := fn.Name\nlo := fn.MinParams\nhi := fn.MaxParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Text("f".into())));
    assert_eq!(report.variables.get("lo"), Some(&Value::Int(2)));
    assert_eq!(report.variables.get("hi"), Some(&Value::Int(2)));

    let report = run("f(a) {\n    return a * 2\n}\nfn := Func(\"f\")\nr := fn.Call(3)");
    assert!(report.ok);
    assert_eq!(report.variables.get("r"), Some(&Value::Int(6)));
}

/// The `FileReadLine` of the v1 reads the n-th line of a file.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command wants
/// three parameters, and the answer is the line itself.
#[test]
fn the_file_read_line_of_the_v1_reads_the_line_of_the_file() {
    let path = std::env::temp_dir().join("ahk-v1-file-read-line.txt");
    std::fs::write(&path, "one\ntwo\n").unwrap();

    let script = format!(
        "FileReadLine, text, {}, 1\nx := text",
        path.to_string_lossy()
    );
    let report = run(&script);
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("one".into())));

    let script = format!(
        "FileReadLine, text, {}, 2\nx := text",
        path.to_string_lossy()
    );
    let report = run(&script);
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("two".into())));

    // A line that the file does not have is empty.
    let script = format!(
        "FileReadLine, text, {}, 9\nx := text",
        path.to_string_lossy()
    );
    let report = run(&script);
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));
}

/// The right side of the legacy assignment of the v1 is a text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x = 5 + 3` leaves
/// `x` at "5 + 3", `x = "abc"` leaves it at `"abc"` with the quotes that
/// were written, and `x = (5+3)` leaves it at "(5+3)".
#[test]
fn the_right_side_of_the_legacy_assignment_of_the_v1_is_a_text() {
    let report = run("x = 5 + 3");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5 + 3".into())));

    let report = run("x = \"abc\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("\"abc\"".into())));

    let report = run("x = (5+3)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("(5+3)".into())));
}

/// The text of the legacy assignment of the v1 is a whole number when it is
/// one in base ten.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x = 5` leaves `x`
/// at the number 5, and `x = 5.5` leaves it at the text "5.5", and `x =
/// 0x10` leaves it at the text "0x10".
#[test]
fn the_text_of_the_legacy_assignment_of_the_v1_is_a_whole_number() {
    let report = run("x = 5");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(5)));

    let report = run("x = 5.5");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5.5".into())));

    let report = run("x = 0x10");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0x10".into())));
}

/// The text of the legacy assignment of the v1 is trimmed, and the `AutoTrim`
/// trims the text that comes from a substitution.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x =    hello`
/// leaves `x` at "hello", and `x = %z%` leaves `x` at the value of `z`
/// without the spaces that come before it and the spaces that come after it
/// when the `AutoTrim` is On.
#[test]
fn the_text_of_the_legacy_assignment_of_the_v1_is_trimmed() {
    let report = run("x =    hello");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("hello".into())));

    let report = run("z := \"  pad  \"\nx = %z%");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("pad".into())));

    // The `AutoTrim` off leaves the spaces of the value that comes from the
    // substitution.
    let report = run("AutoTrim, Off\nz := \"  pad  \"\nx = %z%");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("  pad  ".into())));
}

/// The `AutoTrim` of the v1 turns the trim of the legacy assignments on and
/// off, and the `A_AutoTrim` says which one is in force.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `A_AutoTrim` is
/// "On" when the script begins, and it is "Off" after the command.
#[test]
fn the_auto_trim_of_the_v1_turns_the_trim_on_and_off() {
    let report = run("x := A_AutoTrim");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));

    let report = run("AutoTrim, Off\nx := A_AutoTrim");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Off".into())));

    // The value that the command does not know is refused, and the refusal
    // is at the load.
    assert_eq!(
        the_refusal_of_the_load("AutoTrim, Maybe"),
        "Parameter #1 invalid.\n     Specifically: Maybe"
    );
}

/// The parameters of a command of the v1 are not trimmed by the `AutoTrim`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the value that comes
/// from a substitution keeps its spaces in the parameter of a command.
#[test]
fn the_parameters_of_a_command_of_the_v1_are_not_trimmed_by_the_auto_trim() {
    let report = run("z := \"  pad  \"\nFileAppend, [%z%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[  pad  ]".to_string()]);
}

/// The text of a command of the v1 keeps the spaces of the script.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `FileAppend, [  pad
/// ], *` writes `[  pad  ]`.
#[test]
fn the_text_of_a_command_of_the_v1_keeps_the_spaces_of_the_script() {
    let report = run("FileAppend, [  pad  ], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[  pad  ]".to_string()]);
}

/// The substitution of a parameter of the v1 finds the built-in variables.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `%A_AutoTrim%` is
/// "On".
#[test]
fn the_substitution_of_a_parameter_of_the_v1_finds_the_built_in_variables() {
    let report = run("FileAppend, %A_AutoTrim%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["On".to_string()]);
}

/// The equal sign of the v1 does not write a member of an object.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the line holds no
/// action, and it is refused at the load.
#[test]
fn the_equal_sign_of_the_v1_does_not_write_a_member_of_an_object() {
    let error = ahk::parse_with("o := Object()\no.a = 5", dialect::v1())
        .expect_err("the line holds no action");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: o.a = 5"
    );
}

/// The format of the numbers of the v1 is the one of the `SetFormat`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the whole number is
/// in the base of the format, and the float is rounded to the decimals of it.
#[test]
fn the_format_of_the_numbers_of_the_v1_is_the_one_of_the_set_format() {
    // The whole number in base sixteen, with the prefix.
    let report = run("SetFormat, Integer, H\nx := 255");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0xFF".into())));
    let report = run("SetFormat, Integer, H\nx := 255\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0xFF".to_string()]);

    // The float is rounded to the decimals of the format.
    let report = run("x := 1/3\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0.333333".to_string()]);
    let report = run("SetFormat, Float, 0.2\nx := 1/3\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0.33".to_string()]);

    // The float that is written keeps the text that was written.
    let report = run("x := 1.5\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.5".to_string()]);
}

/// The formats of the v1 say which one is in force.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `A_FormatFloat`
/// is "0.6" and the `A_FormatInteger` is "D" when the script begins.
#[test]
fn the_formats_of_the_v1_say_which_one_is_in_force() {
    let report = run("x := A_FormatFloat\ny := A_FormatInteger");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0.6".into())));
    assert_eq!(report.variables.get("y"), Some(&Value::Text("D".into())));

    let report = run("SetFormat, Float, 0.2\nSetFormat, Integer, H\nx := A_FormatFloat\ny := A_FormatInteger");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0.2".into())));
    assert_eq!(report.variables.get("y"), Some(&Value::Text("H".into())));
}

/// The format that the `SetFormat` does not know is refused.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the value is refused
/// with "Parameter #2 invalid." and the value in the `Extra`. The refusal is
/// at the load.
#[test]
fn the_format_that_the_set_format_does_not_know_is_refused() {
    assert_eq!(
        the_refusal_of_the_load("SetFormat, Integer, X"),
        "Parameter #2 invalid.\n     Specifically: X"
    );
}

/// The parameter of a command of the v1 is a text, and a number may be
/// followed by a name there.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `FileAppend, 2px, *`
/// writes `2px`.
#[test]
fn the_parameter_of_a_command_of_the_v1_may_be_a_number_and_a_name() {
    let report = run("FileAppend, 2px, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["2px".to_string()]);
}

/// The value of a variable of the v1 is a number when it can be one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x + 1` is 6 when
/// `x` holds the text "5", and "0x10" is 16, and " 5 " is 5.
#[test]
fn the_value_of_a_variable_of_the_v1_is_a_number_when_it_can_be_one() {
    let report = run("x := \"5\"\ny := x + 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(6)));

    let report = run("x := \"0x10\"\ny := x + 0");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(16)));

    let report = run("x := \" 5 \"\ny := x + 0");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(5)));

    // A text that is not a number is not a number at all: "1e3" is not one.
    let report = run("x := \"1e3\"\ny := x + 0");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// A value that is not a number makes the operation of the v1 blank.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x - 1` is blank
/// when `x` holds the text "abc", and it is not a failure.
#[test]
fn a_value_that_is_not_a_number_makes_the_operation_of_the_v1_blank() {
    let report = run("x := \"abc\"\ny := x - 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("x := \"abc\"\ny := x * 2");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// The sum of a number and of a text that is written is blank in the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `5 + "1"` is blank,
/// and the sum of two texts that are written is a concatenation.
#[test]
fn the_sum_of_a_number_and_of_a_text_that_is_written_is_blank_in_the_v1() {
    let report = run("x := 5 + \"1\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    let report = run("x := \"5\" + \"1\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("51".into())));
}

/// The comparison of the v1 is a number when both sides are numbers, and a
/// text otherwise.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "abc" > 1 is true,
/// and "abc" < 1 is false.
#[test]
fn the_comparison_of_the_v1_is_a_text_when_a_side_is_not_a_number() {
    let report = run("x := \"abc\" > 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("x := \"abc\" < 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The text that is written is a string: "10" < "3" is true.
    let report = run("x := \"10\" < 3");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    // The value of a variable is a number: 10 < 3 is false.
    let report = run("z := \"10\"\nx := z < 3");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
}

/// The sense of the case of the v1 is Off when the script begins, and the
/// command turns it into On or into Locale.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `A_StringCaseSense`
/// is "Off" when the script begins, and it is "On", "Off" or "Locale"
/// after the command. The numbers one and zero are the two first.
#[test]
fn the_sense_of_the_case_of_the_v1_is_off_when_the_script_begins() {
    let report = run("x := A_StringCaseSense");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Off".into())));

    let report = run("StringCaseSense, On\nx := A_StringCaseSense");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));

    let report = run("StringCaseSense, Locale\nx := A_StringCaseSense");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Locale".into())));

    let report = run("StringCaseSense, 1\nx := A_StringCaseSense");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("On".into())));

    let report = run("StringCaseSense, 0\nx := A_StringCaseSense");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Off".into())));
}

/// The value that the `StringCaseSense` does not know is refused.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is
/// "Parameter #1 invalid." and the value is in the `Extra`. The refusal is
/// at the load.
#[test]
fn the_value_that_the_string_case_sense_does_not_know_is_refused() {
    assert_eq!(
        the_refusal_of_the_load("StringCaseSense, Maybe"),
        "Parameter #1 invalid.\n     Specifically: Maybe"
    );
}

/// The comparison of two texts follows the sense of the case.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "ABC" < "abc" is
/// false when the sense is Off, and it is true when the sense is On.
#[test]
fn the_comparison_of_two_texts_follows_the_sense_of_the_case() {
    let report = run("x := (\"ABC\" < \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("StringCaseSense, On\nx := (\"ABC\" < \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("StringCaseSense, On\nx := (\"ABC\" > \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
}

/// The equal sign of the v1 is always without regard to the case, and the
/// two equals are always with it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "ABC" = "abc" is true
/// with the sense Off and with the sense On, and "ABC" == "abc" is false
/// with the two of them.
#[test]
fn the_equal_sign_of_the_v1_is_always_without_regard_to_the_case() {
    for sense in ["Off", "On", "Locale"] {
        let report = run(&format!("StringCaseSense, {sense}\nx := (\"ABC\" = \"abc\")"));
        assert!(report.ok);
        assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

        let report = run(&format!("StringCaseSense, {sense}\nx := (\"ABC\" == \"abc\")"));
        assert!(report.ok);
        assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
    }
}

/// The not equal sign of the v1 follows the sense of the case.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "ABC" != "abc" is
/// false when the sense is Off, and it is true when the sense is On.
#[test]
fn the_not_equal_sign_of_the_v1_follows_the_sense_of_the_case() {
    let report = run("x := (\"ABC\" != \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("StringCaseSense, On\nx := (\"ABC\" != \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));
}

/// The sense of the locale folds the letters of the Unicode, and the sense
/// that is Off folds only the letters of the English.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: "ä" = "Ä" is false
/// when the sense is Off, and it is true when the sense is On or when it is
/// the sense of the locale.
#[test]
fn the_sense_of_the_locale_folds_the_letters_of_the_unicode() {
    let report = run("x := (\"ä\" = \"Ä\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("StringCaseSense, On\nx := (\"ä\" = \"Ä\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("StringCaseSense, Locale\nx := (\"ä\" = \"Ä\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));
}

/// The `Transform` of the v1 answers the math of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the answer goes
/// into the variable of the caller, and it is the number that the
/// sub-command asks for: the `Mod` of 7 and 3 is 1, and the `Sqrt` of 9 is
/// "3.000000".
#[test]
fn the_transform_of_the_v1_answers_the_math_of_the_v1() {
    let report = run("Transform, out, Mod, 7, 3");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(1)));

    let report = run("Transform, out, Sqrt, 9");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Float(3.0)));

    let report = run("Transform, out, Ceil, 1.2");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(2)));

    let report = run("Transform, out, Pow, 2, 10");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(1024)));

    // The bits of the v1: the `BitNot` is of thirty-two bits and the rest
    // is of sixty-four.
    let report = run("Transform, out, BitNot, 0");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(4294967295)));

    let report = run("Transform, out, BitShiftLeft, 1, 40");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(1099511627776)));
}

/// The `Asc` of the `Transform` of the v1 sees the text as it is, and the
/// numbers see it without the quotes.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `Asc, "a"` is 34,
/// and `Ceil, "1.2"` is 2.
#[test]
fn the_asc_of_the_transform_of_the_v1_sees_the_text_as_it_is() {
    let report = run("Transform, out, Asc, \"a\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(34)));

    let report = run("Transform, out, Ceil, \"1.2\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(2)));
}

/// The absolute value of the `Transform` of the v1 is the text of the
/// number, and the sign of the minus is not a part of it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `Abs, "-5.50"` is
/// "5.50", and `Abs, "  -5 "` is "5 ".
#[test]
fn the_absolute_value_of_the_transform_of_the_v1_is_the_text_of_the_number() {
    let report = run("Transform, out, Abs, \"-5.50\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("5.50".into())));

    let report = run("Transform, out, Abs, \"  -5 \"");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("5 ".into())));
}

/// The sub-command that the `Transform` of the v1 does not know is refused.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is
/// "Parameter #2 invalid." and the sub-command is in the `Extra`. The refusal
/// is at the load.
#[test]
fn the_sub_command_that_the_transform_of_the_v1_does_not_know_is_refused() {
    assert_eq!(
        the_refusal_of_the_load("Transform, out, Hex, 255"),
        "Parameter #2 invalid.\n     Specifically: Hex"
    );
}

/// The `VarSetCapacity` of the v1 empties the variable of the caller.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the function answers
/// the number of bytes that were asked for, and the variable is emptied. A
/// target that is not a variable is answered with zero.
#[test]
fn the_var_set_capacity_of_the_v1_empties_the_variable_of_the_caller() {
    let report = run("y := \"hello\"\nx := VarSetCapacity(y, 100)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(100)));
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // A target that is not a variable is answered with zero.
    let report = run("x := VarSetCapacity(\"hello\", 100)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The capacity that a variable holds is the capacity that is asked for.
    let report = run("size := 100\ny := \"hello\"\nx := VarSetCapacity(y, size)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(100)));
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// The command of the v1 that has a body runs its action when the condition
/// holds.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IfInString` holds
/// when the text of the needle is in the variable, and its action is the
/// statement of the next line, the block that comes after it, or the command
/// that is on the same line.
#[test]
fn the_command_of_the_v1_that_has_a_body_runs_its_action_when_the_condition_holds() {
    // The action is the statement of the next line.
    let report = run("x := \"ABC\"\nIfInString, x, b\nFileAppend, [yes], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);

    // The action is the block that comes after it.
    let report = run("x := \"ABC\"\nIfInString, x, b\n{\nFileAppend, [yes], *\n}");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);

    // The action is the command that is on the same line.
    let report = run("x := \"ABC\"\nIfInString, x, b, FileAppend, [yes], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);

    // The action does not run when the text of the needle is not there.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *");
    assert!(report.ok);
    assert!(report.output.is_empty());
}

/// The `IfNotInString` of the v1 is the other side of the `IfInString`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the action runs when
/// the text of the needle is not in the variable.
#[test]
fn the_if_not_in_string_of_the_v1_is_the_other_side() {
    let report = run("x := \"ABC\"\nIfNotInString, x, z\nFileAppend, [yes], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);

    let report = run("x := \"ABC\"\nIfNotInString, x, b\nFileAppend, [yes], *");
    assert!(report.ok);
    assert!(report.output.is_empty());
}

/// The `IfMsgBox` of the v1 holds when the person pressed the button that is
/// named, and the message box of this runtime has no buttons.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the action does not
/// run, and the script goes on.
#[test]
fn the_if_msg_box_of_the_v1_does_not_hold() {
    let report = run("IfMsgBox, OK\nFileAppend, [yes], *\nFileAppend, [after], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[after]".to_string()]);
}

/// The `else` of the v1 is the action of the other side of the `if` that
/// comes before it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` follows the
/// action of the `if`, and it may be a block, the statement of the next line
/// or the command that is on the same line.
#[test]
fn the_else_of_the_v1_is_the_action_of_the_other_side() {
    // The else is a block.
    let report = run("x := \"ABC\"\nIfInString, x, z\n{\nFileAppend, [yes], *\n}\nelse\n{\nFileAppend, [no], *\n}");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[no]".to_string()]);

    // The else is the statement of the next line.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *\nelse\nFileAppend, [no], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[no]".to_string()]);

    // The else is the command that is on the same line.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *\nelse FileAppend, [no], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[no]".to_string()]);

    // The action of the `if` runs when the condition holds.
    let report = run("x := \"ABC\"\nIfInString, x, b\nFileAppend, [yes], *\nelse\nFileAppend, [no], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);
}

/// The action of the `else` of the v1 may be on the same line.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the action is the
/// command that is on the same line, and it may be an assignment too. The
/// `else` that is at the end of the script has no action.
#[test]
fn the_action_of_the_else_of_the_v1_may_be_on_the_same_line() {
    // The action is the command that is on the same line.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *\nelse FileAppend, [no], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[no]".to_string()]);

    // The action is the assignment that is on the same line.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *\nelse y = 5");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(5)));

    // The `else` that is at the end of the script has no action.
    let report = run("x := \"ABC\"\nIfInString, x, z\nFileAppend, [yes], *\nelse");
    assert!(report.ok);
    assert!(report.output.is_empty());
}

/// The `else` of the v1 that has no `if` before it is refused at the load.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is "ELSE
/// with no matching IF", and the `else` that carries a parameter names the
/// parameter of it.
#[test]
fn the_else_of_the_v1_that_has_no_if_before_it_is_refused_at_the_load() {
    let error = ahk::parse_with("else", dialect::v1()).expect_err("the else has no if");
    assert_eq!(error.message, "ELSE with no matching IF");

    let error = ahk::parse_with("else, something", dialect::v1())
        .expect_err("the else takes no parameter");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: something"
    );
}

/// The number of the parameters of a command of the v1 is checked at the load.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is the one
/// of the v1, and it names the command and the parameters that were written.
#[test]
fn the_number_of_the_parameters_of_a_command_of_the_v1_is_checked_at_the_load() {
    let error = ahk::parse_with("IfInString, x", dialect::v1())
        .expect_err("the command wants two parameters");
    assert_eq!(
        error.message,
        "\"IfInString\" requires at least 2 parameters.\n     Specifically: IfInString, x"
    );

    let error = ahk::parse_with("IfMsgBox", dialect::v1())
        .expect_err("the command wants one parameter");
    assert_eq!(
        error.message,
        "\"IfMsgBox\" requires at least 1 parameter.\n     Specifically: IfMsgBox"
    );

    // The comma at the end of the line does not open a parameter.
    let error = ahk::parse_with("Send,", dialect::v1())
        .expect_err("the command wants one parameter");
    assert_eq!(
        error.message,
        "\"Send\" requires at least 1 parameter.\n     Specifically: Send,"
    );
}

/// The parameter of a command of the v1 that wants the name of a variable may
/// not be blank.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is the one
/// of the v1, and it names the parameter that is blank.
#[test]
fn the_parameter_of_a_command_of_the_v1_that_wants_a_name_may_not_be_blank() {
    let error = ahk::parse_with("IfInString, , b", dialect::v1())
        .expect_err("the variable of the search has no name");
    assert_eq!(
        error.message,
        "\"IfInString\" requires that parameter #1 be non-blank.\n     Specifically: IfInString,"
    );
}

/// The needle of the `IfInString` of the v1 that is written as nothing is a
/// refusal of the load, and the one that becomes empty at the run is a text
/// that is in every text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the needle that is
/// written as nothing does not count, and the load refuses the command. The
/// needle that is the substitution of a name that has no value is empty at the
/// run, and the command holds.
#[test]
fn the_needle_of_the_search_of_the_v1_that_is_empty_is_a_refusal_of_the_load() {
    let error = ahk::parse_with("x := \"ABC\"\nIfInString, x,", dialect::v1())
        .expect_err("the needle is not there");
    assert_eq!(
        error.message,
        "\"IfInString\" requires at least 2 parameters.\n     Specifically: IfInString, x"
    );

    // The needle that becomes empty at the run is in every text.
    let report = run("x := \"ABC\"\nIfInString, x, %nothing%\nFileAppend, [yes], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[yes]".to_string()]);
}

/// The place of the `StringGetPos` of the v1 counts from zero.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the place of the text
/// is the number of the characters that come before it, and it is minus one
/// when the text is not there. The occurrence is the one that the caller
/// wants: "L2" is the second from the left and "R1" is the first from the
/// right. The offset is the number of characters that the search skips over.
#[test]
fn the_place_of_the_string_get_pos_of_the_v1_counts_from_zero() {
    // The place of the text.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, nan");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(2)));

    // The text that is not there.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, zzz");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(-1)));

    // The second occurrence from the left.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, a, L2");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(3)));

    // The first occurrence from the right.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, a, R1");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(5)));

    // The offset skips the characters of the left.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, a, , 2");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(3)));

    // The sense of the case says whether the letters are the same.
    let report = run("StringCaseSense, On\ntext := \"banana\"\nStringGetPos, pos, text, NAN");
    assert!(report.ok);
    assert_eq!(report.variables.get("pos"), Some(&Value::Int(-1)));
}

/// The occurrence of the `StringGetPos` of the v1 that is not one is refused.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal is the
/// one of the v1, and it names the occurrence that was written. The refusal is
/// at the load.
#[test]
fn the_occurrence_of_the_string_get_pos_of_the_v1_that_is_not_one_is_refused() {
    assert_eq!(
        the_refusal_of_the_load("text := \"banana\"\nStringGetPos, pos, text, a, 0"),
        "Parameter #4 invalid.\n     Specifically: 0"
    );
}

/// The `StringReplace` of the v1 replaces the first occurrence of the text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the first occurrence
/// is the one that is replaced, and the parameter that says "all" replaces
/// every one of them. The sense of the case says whether the letters of the
/// text and of the search are the same.
#[test]
fn the_string_replace_of_the_v1_replaces_the_first_occurrence_of_the_text() {
    // The first occurrence.
    let report = run("out := \"banana\"\nStringReplace, out, out, a, b");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("bbnana".to_string())));

    // Every occurrence.
    let report = run("out := \"banana\"\nStringReplace, out, out, a, b, All");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("bbnbnb".to_string())));

    // The text that is put in the place may be nothing.
    let report = run("out := \"banana\"\nStringReplace, out, out, a");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("bnana".to_string())));

    // The sense of the case says whether the letters are the same.
    let report = run("StringCaseSense, On\nout := \"banana\"\nStringReplace, out, out, A, b");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("banana".to_string())));

    let report = run("out := \"banana\"\nStringReplace, out, out, A, b");
    assert!(report.ok);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("bbnana".to_string())));
}

/// The parameter of the `StringReplace` of the v1 that says "all" says it
/// with a letter.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter says
/// "all" when it has the letter "a" -- in any case -- or the digit "1" in
/// it, and when it begins with "useerrorlevel" -- in any case. The parameter
/// that says nothing replaces the first occurrence and no other.
#[test]
fn the_parameter_of_the_string_replace_of_the_v1_that_says_all_is_a_letter() {
    for (parameter, answer) in [
        ("All", "bbnbnb"),
        ("A", "bbnbnb"),
        ("1", "bbnbnb"),
        ("UseErrorLevel", "bbnbnb"),
        ("0", "bbnana"),
        ("2", "bbnana"),
        ("yes", "bbnana"),
    ] {
        let report = run(&format!(
            "out := \"banana\"\nStringReplace, out, out, a, b, {parameter}"
        ));
        assert!(report.ok, "{parameter}");
        assert_eq!(
            report.variables.get("out"),
            Some(&Value::Text(answer.to_string())),
            "{parameter}"
        );
    }
}

/// The `StrReplace` of the v1 writes the number of the replacements into the
/// variable of the caller.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the fourth parameter
/// is the variable that receives the number of the replacements, and it is a
/// variable and not a name of one -- a text, a number or an expression has
/// nowhere to go and the number is written nowhere. The fifth parameter is
/// the number of the replacements that the caller allows, and the limit that
/// is zero replaces none of them. The variable of a function is the one of
/// the function, and the word `global` is what makes it the one of the
/// script.
#[test]
fn the_str_replace_of_the_v1_writes_the_number_of_the_replacements() {
    // Every occurrence of the text is replaced when the limit is not there.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("XX".to_string())));

    // The number of the replacements goes into the variable of the caller.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\", count)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("XX".to_string())));
    assert_eq!(report.variables.get("count"), Some(&Value::Int(2)));

    // The needle that is not there replaces nothing.
    let report = run("x := StrReplace(\"abcabc\", \"zzz\", \"X\", count)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("abcabc".to_string())));
    assert_eq!(report.variables.get("count"), Some(&Value::Int(0)));

    // The limit says how many of them are replaced.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\", count, 1)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Xabc".to_string())));
    assert_eq!(report.variables.get("count"), Some(&Value::Int(1)));

    // The limit that is zero replaces none of them.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\", count, 0)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("abcabc".to_string())));
    assert_eq!(report.variables.get("count"), Some(&Value::Int(0)));

    // The fourth parameter that is not a variable has nowhere to go.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\", 1 + 1)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("XX".to_string())));

    // The sense of the case says whether the letters are the same.
    let report = run("StringCaseSense, On\nx := StrReplace(\"ABCabc\", \"abc\", \"X\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("ABCX".to_string())));

    let report = run("x := StrReplace(\"ABCabc\", \"abc\", \"X\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("XX".to_string())));

    // The variable of a function is the one of the function.
    let report = run("f() {\n    x := StrReplace(\"abcabc\", \"abc\", \"X\", count)\n}\nf()");
    assert!(report.ok);
    assert_eq!(report.variables.get("count"), None);

    // The word `global` is what makes it the one of the script.
    let report = run(
        "f() {\n    global count\n    x := StrReplace(\"abcabc\", \"abc\", \"X\", count)\n}\nf()",
    );
    assert!(report.ok);
    assert_eq!(report.variables.get("count"), Some(&Value::Int(2)));
}

/// The refusal of a command of the v1 names the first parameter of it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the text that the
/// refusal names is the text of the command, from the name of it to the end
/// of the first parameter -- and the first one and no other, whatever the
/// refusal is about. The command that has no parameter names the name and the
/// comma that comes after it, and the first parameter that is blank adds
/// nothing to the text.
#[test]
fn the_refusal_of_a_command_of_the_v1_names_the_first_parameter() {
    // The command that has too few parameters.
    let error = ahk::parse_with("StringReplace, out, a", dialect::v1())
        .expect_err("the command wants three parameters");
    assert_eq!(
        error.message,
        "\"StringReplace\" requires at least 3 parameters.\n     Specifically: StringReplace, out"
    );

    // The parameter that is blank.
    let error = ahk::parse_with("StringReplace, out, , b", dialect::v1())
        .expect_err("the text of the command is blank");
    assert_eq!(
        error.message,
        "\"StringReplace\" requires that parameter #2 be non-blank.\n     Specifically: StringReplace, out"
    );

    // The first parameter that is blank adds nothing to the text.
    let error = ahk::parse_with("StringReplace, , a", dialect::v1())
        .expect_err("the command wants three parameters");
    assert_eq!(
        error.message,
        "\"StringReplace\" requires at least 3 parameters.\n     Specifically: StringReplace,"
    );

    // The command that has no parameter names the comma that comes after it.
    let error = ahk::parse_with("IfMsgBox", dialect::v1())
        .expect_err("the command wants one parameter");
    assert_eq!(
        error.message,
        "\"IfMsgBox\" requires at least 1 parameter.\n     Specifically: IfMsgBox"
    );
}

/// The call of a function of the v1 is refused at the load.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the number of the
/// arguments that is wrong is refused at the load, and the refusal names the
/// call as it was written -- the spaces of the script are in it. The name
/// that is not a function is refused at the load too: a command of the v1 is
/// not a function, and a name that the script does not define is not one
/// either.
#[test]
fn the_call_of_a_function_of_the_v1_is_refused_at_the_load() {
    // The built-in that has too few arguments.
    let error = ahk::parse_with("x := InStr(\"ABCabc\")", dialect::v1())
        .expect_err("the function wants two arguments");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: InStr(\"ABCabc\")"
    );

    // The built-in that has too many arguments.
    let error = ahk::parse_with("x := InStr(\"a\", \"b\", 1, 1, 1, 1)", dialect::v1())
        .expect_err("the function wants five arguments");
    assert_eq!(
        error.message,
        "Too many parameters passed to function.\n     Specifically: InStr(\"a\", \"b\", 1, 1, 1, 1)"
    );

    // The spaces of the script are in the text that is named.
    let error = ahk::parse_with("x := InStr( \"ABCabc\" )", dialect::v1())
        .expect_err("the function wants two arguments");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: InStr( \"ABCabc\" )"
    );

    // The name that is not there.
    let error = ahk::parse_with("x := Nonsense(1)", dialect::v1())
        .expect_err("the name is not a function");
    assert_eq!(
        error.message,
        "Call to nonexistent function.\n     Specifically: Nonsense(1)"
    );

    // The command of the v1 is not a function.
    let error = ahk::parse_with("x := FileAppend(\"hello\", \"*\")", dialect::v1())
        .expect_err("the command is not a function");
    assert_eq!(
        error.message,
        "Call to nonexistent function.\n     Specifically: FileAppend(\"hello\", \"*\")"
    );

    // The function of the script with too few arguments.
    let error = ahk::parse_with("f(a, b) {\n}\nx := f(1)", dialect::v1())
        .expect_err("the function wants two arguments");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: f(1)"
    );

    // The function of the script with too many arguments.
    let error = ahk::parse_with("f(a) {\n}\nx := f(1, 2)", dialect::v1())
        .expect_err("the function wants one argument");
    assert_eq!(
        error.message,
        "Too many parameters passed to function.\n     Specifically: f(1, 2)"
    );

    // The parameter that has a default may be left out.
    let error = ahk::parse_with("f(a, b := 1) {\n}\nx := f()", dialect::v1())
        .expect_err("the function wants one argument");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: f()"
    );

    // The name of a function of the script may be defined after the call.
    let error = ahk::parse_with("x := f(1)\nf(a, b) {\n}", dialect::v1())
        .expect_err("the function wants two arguments");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: f(1)"
    );
}

/// A function of the v1 is not a command.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the command that is
/// written with the name of a function is a line that holds no action, and
/// the load refuses it with the text of the line.
#[test]
fn a_function_of_the_v1_is_not_a_command() {    let error = ahk::parse_with("StrLen \"abc\"", dialect::v1())
        .expect_err("the function is not a command");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: StrLen \"abc\""
    );

    let error = ahk::parse_with("InStr \"abc\"", dialect::v1())
        .expect_err("the function is not a command");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: InStr \"abc\""
    );

    // The function of the script is not a command either.
    let error = ahk::parse_with("f(a) {\n}\nf \"abc\"", dialect::v1())
        .expect_err("the function is not a command");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: f \"abc\""
    );
}

/// The `ErrorLevel` of the v1 is the global variable of the script.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: it is zero when the
/// script begins, and a script may write it -- with any value, and with the
/// operators of the v1. The `StringGetPos` writes zero when it finds the
/// text and one when it does not, and the `StringReplace` writes the number
/// of the replacements when the parameter says "useerrorlevel" and zero when
/// it does not. The write of a function is a local one, and the machine
/// writes the global.
#[test]
fn the_error_level_of_the_v1_is_the_global_variable_of_the_script() {
    // The value that the name has when nothing wrote it.
    let report = run("x := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The name has a value, and the v1 says so.
    let report = run("x := IsSet(ErrorLevel)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    // A script may write it.
    let report = run("ErrorLevel := 5\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));

    // The operators of it are the ones of the v1.
    let report = run("ErrorLevel := 5\nErrorLevel += 3\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(8)));

    let report = run("ErrorLevel := 5\nErrorLevel -= 3\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(2)));

    // The value that is written is kept as it is.
    let report = run("ErrorLevel := \"abc\"\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("abc".to_string())));

    // The write of a function is a local one, and the global keeps the value
    // that the machine wrote.
    let report = run(
        "f() {\n    ErrorLevel := 5\n    return ErrorLevel\n}\nx := f()\ny := ErrorLevel",
    );
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));

    // The `StringGetPos` writes it.
    let report = run("text := \"banana\"\nStringGetPos, pos, text, nan\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("text := \"banana\"\nStringGetPos, pos, text, zzz\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    // The `StringReplace` writes the number of the replacements when the
    // parameter says "useerrorlevel", and zero when it does not.
    let report = run(
        "text := \"banana\"\nStringReplace, out, text, a, b, UseErrorLevel\nx := ErrorLevel",
    );
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(3)));

    let report = run("text := \"banana\"\nStringReplace, out, text, a, b, All\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The `StrReplace` does not write it.
    let report = run("x := StrReplace(\"abcabc\", \"abc\", \"X\")\ny := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));
}

/// The file commands of the v1 write the `ErrorLevel`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `FileAppend` is
/// one when there was a problem and zero otherwise, and the `FileCopy`, the
/// `FileDelete`, the `FileMove`, the `FileRead` and the `FileSetTime` are
/// the number of the files that failed.
#[test]
fn the_file_commands_of_the_v1_write_the_error_level() {
    // The command that works writes zero.
    let report = run("ErrorLevel := 5\nFileAppend, hello, *\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("ErrorLevel := 5\nFileCopy, a, b\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("ErrorLevel := 5\nFileDelete, a\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("ErrorLevel := 5\nFileMove, a, b\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The file that is not there is not a failure of the v1: the command
    // goes on and the `ErrorLevel` is zero.
    let report = run("ErrorLevel := 5\nFileDelete, nothere.txt\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
}

/// The source of the file commands of the v1 may be a pattern.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `FileCopy`, the
/// `FileMove` and the `FileDelete` take a pattern, and they do the thing to
/// every file that matches it -- the copy and the move into the folder of
/// the destination, with the name of the file. The `ErrorLevel` is the
/// number of the files that failed, and the v1 goes on.
#[test]
fn the_file_commands_of_the_v1_take_a_pattern() {
    // The copy of a pattern puts every file that matches it into the folder
    // of the destination.
    let (report, calls) = run_and_the_calls("FileCopy, *.txt, folder");
    assert!(report.ok);
    assert_eq!(
        calls,
        vec![
            "list_files *.txt",
            "copy_file C:\\work\\a.txt folder/a.txt false",
            "copy_file C:\\work\\b.txt folder/b.txt false",
        ]
    );
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(0)));

    // The move of a pattern does the same, and the folder of the
    // destination may end with the separator already.
    let (report, calls) = run_and_the_calls("FileMove, *.txt, folder\\");
    assert!(report.ok);
    assert_eq!(
        calls,
        vec![
            "list_files *.txt",
            "move_file C:\\work\\a.txt folder\\a.txt false",
            "move_file C:\\work\\b.txt folder\\b.txt false",
        ]
    );

    // The delete of a pattern takes every file that matches it away.
    let (report, calls) = run_and_the_calls("FileDelete, *.txt");
    assert!(report.ok);
    assert_eq!(
        calls,
        vec![
            "list_files *.txt",
            "delete_file C:\\work\\a.txt",
            "delete_file C:\\work\\b.txt",
        ]
    );

    // The file that cannot be copied is counted, and the v1 goes on.
    let (report, calls) = run_and_the_calls("FileCopy, *.bin, folder\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(
        calls,
        vec!["list_files *.bin", "copy_file C:\\work\\nothere.bin folder/nothere.bin false"]
    );
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(1)));

    // The pattern that matches nothing is not a failure.
    let (report, calls) = run_and_the_calls("FileDelete, nothere*.txt\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(calls, vec!["list_files nothere*.txt"]);
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(0)));
}

/// The name of a function is not a value of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `x := StrLen` is a
/// blank text, and so is the name of a function of the script and the name
/// of a command. The `Func` is the way that the v1 has a function as a
/// value.
#[test]
fn the_name_of_a_function_is_not_a_value_of_the_v1() {
    let report = run("x := StrLen");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    let report = run("f() {\n    return 1\n}\nx := f");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    // A command of the v1 is not a value either.
    let report = run("x := MsgBox");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    // The name that is followed by a text is a concatenation, and the name
    // contributes nothing to it.
    let report = run("x := StrLen \"abc\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("abc".into())));
}

/// The `IsFunc` of the v1 takes a value and not a name.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the value of the name
/// of a function is a blank text, and so `IsFunc(StrLen)` is zero. A command
/// of the v1 is not a function, and a reference to a function is a function.
#[test]
fn the_is_func_of_the_v1_takes_a_value() {
    let report = run("n := IsFunc(StrLen)");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(0)));

    // A command of the v1 is not a function.
    let report = run("n := IsFunc(\"MsgBox\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(0)));

    // The name that is a command and a function at the same time is a
    // function.
    let report = run("n := IsFunc(\"GetKeyState\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(2)));

    // A reference to a function is a function.
    let report = run("n := IsFunc(Func(\"StrLen\"))");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(2)));

    let report = run("f() {\n    return 1\n}\nn := IsFunc(Func(\"f\"))");
    assert!(report.ok);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(1)));
}

/// The `Func` of the v1 is the reference to the function.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the reference of a
/// built-in carries the name of it and the number of its parameters, and the
/// most parameters of a built-in that takes any number of them is ten
/// thousand. The name of a command is not a function, and the name of
/// nothing is the number zero.
#[test]
fn the_func_of_the_v1_is_the_reference_to_the_function() {
    let report = run("fn := Func(\"StrLen\")\nx := fn.Name");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("StrLen".into())));

    let report = run("fn := Func(\"StrLen\")\nx := fn.IsBuiltIn");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("fn := Func(\"StrLen\")\nx := fn.MinParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("fn := Func(\"StrLen\")\nx := fn.MaxParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    // The built-in that takes any number of arguments.
    let report = run("fn := Func(\"Format\")\nx := fn.MaxParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(10000)));

    // The function of the script.
    let report = run("f(a, b) {\n}\nfn := Func(\"f\")\nx := fn.Name");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("f".into())));

    let report = run("f(a, b) {\n}\nfn := Func(\"f\")\nx := fn.IsBuiltIn");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("f(a, b) {\n}\nfn := Func(\"f\")\nx := fn.MinParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(2)));

    // The call of the reference.
    let report = run("fn := Func(\"StrLen\")\nx := fn.Call(\"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(3)));

    // The name of a command is not a function.
    let report = run("x := Func(\"MsgBox\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The name of nothing is the number zero.
    let report = run("x := Func(\"nothing\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The name that is a command and a function at the same time is a
    // function.
    let report = run("fn := Func(\"GetKeyState\")\nx := fn.IsBuiltIn");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    let report = run("fn := Func(\"GetKeyState\")\nx := fn.MinParams");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));
}

/// The `ClipWait` of the v1 writes the `ErrorLevel`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` is
/// one when the wait expires and zero when the clipboard has data -- the
/// other way round from the answer of the v2.
#[test]
fn the_clip_wait_of_the_v1_writes_the_error_level() {
    // The clipboard has nothing: the wait expires.
    let report = run("ErrorLevel := 5\nClipWait, 0.05\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(1)));

    // The clipboard has data: the wait is over at once.
    let report = run_with_the_clipboard(
        "ErrorLevel := 5\nClipWait, 0.05\nx := ErrorLevel",
        "the data of the clipboard",
    );
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
}

/// The `Run` of the v1 writes the `ErrorLevel` when the options say it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` is
/// not touched, unless the options say "useerrorlevel": then it is zero when
/// the program starts, and the word "ERROR" when it does not. The option
/// says it when the word is in it, in any case, and wherever it is in it.
#[test]
fn the_run_of_the_v1_writes_the_error_level() {
    // The program starts: the `ErrorLevel` is zero.
    let report = run("ErrorLevel := 5\nRun, program.exe, , UseErrorLevel\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // The word of the option is in the options in any case.
    let report = run("ErrorLevel := 5\nRun, program.exe, , xUseErrorLevel\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    // Without the option the `ErrorLevel` is not touched.
    let report = run("ErrorLevel := 5\nRun, program.exe\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));

    // The program does not start: the `ErrorLevel` is the word that says it,
    // and the tree fails the script where the v1 goes on.
    let report = run("ErrorLevel := 5\nRun, nothere.exe, , UseErrorLevel");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("this fake has no processes"));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Text("ERROR".into())));
}

/// The `RunWait` of the v1 writes the code of the program in the
/// `ErrorLevel`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `RunWait` is a
/// command of the v1, and the code of the program that ran is the
/// `ErrorLevel`, and the word "ERROR" is the one of a program that did not
/// start.
#[test]
fn the_run_wait_of_the_v1_writes_the_error_level() {
    let report = run("ErrorLevel := 5\nRunWait, program.exe\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(7)));

    let report = run("ErrorLevel := 5\nRunWait, nothere.exe, , UseErrorLevel");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("this fake has no processes"));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Text("ERROR".into())));
}

/// The `Run` of the v1 does not wait for the program, and the pid of the
/// process goes into the variable of the caller.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `Run` starts the
/// program and goes on -- the script ends while the program runs -- and the
/// pid of the process goes into the fourth parameter of the command. The
/// program that does not start is the same failure of the `RunWait`.
#[test]
fn the_run_of_the_v1_writes_the_pid() {
    let report = run("Run, program.exe, , , pid\nx := pid");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(4242)));

    // The `ErrorLevel` is not touched: the `Run` does not wait, and it has no
    // code to write.
    let report = run("ErrorLevel := 5\nRun, program.exe\nx := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));

    // The program that does not start fails the script where the v1 goes on.
    let report = run("Run, nothere.exe, , , pid");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("this fake has no processes"));
}

/// The `RunWait` of the v1 waits for the program, and the pid of the process
/// goes into the variable of the caller.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `RunWait` waits
/// for the program and writes the pid of it into the fourth parameter of the
/// command.
#[test]
fn the_run_wait_of_the_v1_writes_the_pid() {
    let report = run("RunWait, program.exe, , , pid\nx := pid");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(4242)));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(7)));
}

/// The `DllCall` of the v1 writes the code of the failure in the
/// `ErrorLevel`.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel` is
/// zero when the call works, and the code of the failure when it does not --
/// the minus two when the call itself is not valid, the minus three when the
/// library is not there and the minus four when the function is not in it.
/// The v1 goes on when the call fails, and the tree fails the script with
/// the reason of the machine.
#[test]
fn the_dll_call_of_the_v1_writes_the_code_of_the_failure() {
    // The call that works leaves the `ErrorLevel` at zero.
    let report = run("x := DllCall(\"kernel32\\Sleep\", \"Int\", 1)\ny := ErrorLevel");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(7)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));

    // The library that is not there is the minus three.
    let report = run("x := DllCall(\"nolibrary\\Sleep\", \"Int\", 1)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Failed to load DLL."));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(-3)));

    // The function that is not in the library is the minus four.
    let report = run("x := DllCall(\"kernel32\\nofunction\", \"Int\", 1)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Call to nonexistent function."));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(-4)));

    // The call that is not valid is the minus two.
    let report = run("x := DllCall(\"kernel32\\Sleep\", \"NoSuchType\", 1)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid arg type."));
    assert_eq!(report.variables.get("errorlevel"), Some(&Value::Int(-2)));
}

/// A member that a value does not have is a blank text of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
/// refuse a member that is not there, and the answer is a blank text -- the
/// property of an object that does not have it, and the property of a value
/// that is not an object.
#[test]
fn a_member_that_is_not_there_is_a_blank_text_of_the_v1() {
    let report = run("o := {}\ny := o.nothing");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));

    let report = run("o := {}\ny := o.nothing()");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));

    // A value that is not an object.
    let report = run("x := 5\ny := x.nothing");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));

    let report = run("x := 5\ny := x.nothing()");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));

    // The item of an object.
    let report = run("o := {}\ny := o[\"a\"]");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("".into())));

    // The property that is there is still there.
    let report = run("o := {a: 1}\ny := o.a");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("1".into())));
}

/// The text of an object of the v1 is the empty text.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: `Object()` is a
/// blank text, and the concatenation of an object and of "a" is "a".
#[test]
fn the_text_of_an_object_of_the_v1_is_the_empty_text() {
    let report = run("x := Object() . \"a\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("a".into())));

    let report = run("x := \"a\" . Object()");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("a".into())));

    // The object of the `Func` is an object too.
    let report = run("x := Func(\"StrLen\") . \"a\"");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("a".into())));

    // The sum of an object and of a number is blank.
    let report = run("x := Object() + 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("".into())));

    // A built-in that wants a text takes the empty text of the object.
    let report = run("x := StrLen(Object())");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));

    let report = run("x := InStr(Object(), \"a\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
}

/// The `global` of a built-in variable is refused by the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses it at
/// the load -- "Built-in variables must not be declared.\n     Specifically:
/// ErrorLevel" -- and the v2 allows it: the global names the built-in
/// variable of the dialect and not a variable of the script.
#[test]
fn the_global_of_a_built_in_variable_is_refused_by_the_v1() {
    // Every built-in variable of the v1 is refused, and the refusal names
    // it. Three of the variables of the v1 have a name that the v1 spells
    // without the prefix -- the `Clipboard`, the `ComSpec` and the
    // `ProgramFiles` -- and the `ErrorLevel` has no prefix at all.
    for name in [
        "ErrorLevel",
        "A_Clipboard",
        "Clipboard",
        "A_ComSpec",
        "ComSpec",
        "A_ProgramFiles",
        "ProgramFiles",
        "A_Cursor",
        "A_ScriptDir",
    ] {
        let error = ahk::parse_with(&format!("global {name}\nx := 1"), dialect::v1())
            .expect_err("the built-in variable may not be declared");
        assert_eq!(
            error.message,
            format!("Built-in variables must not be declared.\n     Specifically: {name}")
        );
    }

    // A name that the v1 spells without the prefix and that is not one of
    // those four is a variable of the script: the v1 reads it through the
    // environment, and the environment of the machine has no `Cursor` of
    // its own.
    for name in ["Cursor", "Temp", "Desktop", "AhkPath"] {
        let report = run(&format!("global {name}\nx := 1"));
        assert!(report.ok, "the v1 accepts the global of {name}");
    }

    // The `A_Args` is a variable of the script: the interpreter creates it
    // at the load, and the v1 does not refuse it.
    let report = run("global A_Args\nx := 1");
    assert!(report.ok);

    // The names that the v1 does not have as variables are not refused
    // either: the five of them are directives of the v1 and the
    // `A_TrayMenu` is a variable of the v2.
    for name in [
        "A_AllowMainWindow",
        "A_HotkeyInterval",
        "A_HotkeyModifierTimeout",
        "A_MaxHotkeysPerInterval",
        "A_MenuMaskKey",
        "A_TrayMenu",
    ] {
        let report = run(&format!("global {name}\nx := 1"));
        assert!(report.ok, "the v1 accepts the global of {name}");
    }

    // The global of a variable of the script is fine.
    let report = run("global x\nx := 1");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1".into())));

    // The global of a built-in function is fine too: the name is not a
    // variable.
    let report = run("global StrLen\nx := 1");
    assert!(report.ok);
}

/// The `new` of the v1 makes an instance of a class of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `new` makes an instance, the `__Init` of the class runs before the
/// `__New`, and the arguments of the `new` are the ones of the `__New`.
#[test]
fn the_new_of_the_v1_makes_an_instance() {
    // The `__Init` of the script runs, and the `this` of it is the instance.
    let report = run("class C {\n    __Init() {\n        this.x := 1\n    }\n}\no := new C\nx := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1".into())));

    // The `__New` of the script runs too.
    let report = run("class C {\n    __New() {\n        this.x := 2\n    }\n}\no := new C\nx := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("2".into())));

    // The arguments of the `new` are the ones of the `__New`.
    let report = run("class C {\n    __New(a) {\n        this.x := a\n    }\n}\no := new C(5)\nx := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));

    // The `__Init` takes no arguments: the `new` gives its arguments to the
    // `__New` and to nothing else.
    let report = run("class C {\n    __Init(a) {\n        this.x := a\n    }\n}\no := new C(5)\nx := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // The instance variables of the class are assigned when the instance is
    // made.
    let report = run("class C {\n    x := 5\n}\no := new C\nx := o.x");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));

    // The new of a class that is not there is the empty text, and the v1
    // does not fail.
    let report = run("o := new Nothing\nx := o");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // The word `new` is not reserved: it is a variable of the script when no
    // name follows it.
    let report = run("new := 5\nx := new");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("5".into())));
}

/// The `new` of the v1 takes the path of the class.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name
/// of the class may be followed by the name of the class that it holds: the
/// `new C.D` is an instance of D, and the `new o.D` is an instance of the
/// class that the instance holds, and the `new this.D` is an instance of the
/// class that the instance of the method holds.
#[test]
fn the_new_of_the_v1_takes_the_path_of_the_class() {
    // The `new C.D` is an instance of the class that C holds.
    let report = run(
        "class C {\n    class D {\n    }\n}\no := new C.D\nx := o.__Class\nFileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["C.D".to_string()]);

    // The `new o.D` is an instance of the class that the instance holds.
    let report = run(
        "class C {\n    class D {\n    }\n}\no := new C\np := new o.D\nx := p.__Class\nFileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["C.D".to_string()]);

    // The `new this.D` is an instance of the class that the instance of the
    // method holds.
    let report = run(
        "class C {\n    class D {\n    }\n    M() {\n        return new this.D\n    }\n}\no := new C\np := o.M()\nx := p.__Class\nFileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["C.D".to_string()]);

    // The path may be longer than one name.
    let report = run(
        "class C {\n    class D {\n        class E {\n        }\n    }\n}\no := new C.D.E\nx := o.__Class\nFileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["C.D.E".to_string()]);

    // The methods and the properties of the class that is held are there.
    let report = run(
        "class C {\n    class D {\n        M() {\n            return 7\n        }\n        x := 5\n    }\n}\no := new C.D\ny := o.M()\nz := o.x\nFileAppend, %y% %z%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["7 5".to_string()]);

    // The class that is not there is the empty text, and the v1 does not
    // fail.
    let report = run("class C {\n}\no := new C.D\nx := o");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // The name that is not there is the empty text too.
    let report = run("o := new Nothing.D\nx := o");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // The class without a name inside a class is read, and the class that
    // holds it does not own it.
    let report = run("class C {\n    class {\n    }\n}\no := new C\nx := o");
    assert!(report.ok);
    assert!(report.variables.contains_key("x"));

    // The class that a class holds is a member of the instance of it too.
    let report = run(
        "class C {\n    class D {\n    }\n}\no := new C\nx := o.D\ny := x.__Class\nFileAppend, %y%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["C.D".to_string()]);

    // The class of the v1 has no `Call`, and the call of it makes nothing.
    let report = run("class C {\n}\no := C.Call()\nx := o");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // The call of the class that a class holds makes nothing too.
    let report = run("class C {\n    class D {\n    }\n}\no := C.D()\nx := o");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

/// The class of an instance of the v1 is the name of the class.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `__Class` of the v1 is a property of an object, and a value that is not
/// an object has none.
#[test]
fn the_class_of_the_instance_of_the_v1_is_the_name_of_the_class() {
    let report = run("class C {\n}\no := new C\nx := o.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("C".into())));

    // The class of a class that extends another is the name of it.
    let report = run("class C {\n}\nclass D extends C {\n}\no := new D\nx := o.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("D".into())));

    // The class of the class is the name of it too.
    let report = run("class C {\n}\nx := C.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("C".into())));

    // A value that is not an object has no class: the answer is the empty
    // text.
    let report = run("x := 5\ny := x.__Class");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// The method of the class object of the v1 runs, and the `this` of it is
/// the class object.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02. The v2
/// refuses it: "This value of type \"Class\" has no method named \"Say\".".
#[test]
fn the_method_of_the_class_object_of_the_v1_runs() {
    let report = run("class C {\n    Say() {\n        return \"hi\"\n    }\n}\nx := C.Say()");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("hi".into())));

    // The `this` of the method is the class object.
    let report = run("class C {\n    Say() {\n        return this.__Class\n    }\n}\nx := C.Say()");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("C".into())));
}

/// The call of a method of the v1 does not check the number of the
/// arguments.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// method runs only when the call gives a value to every parameter that has
/// no default, and the answer of a call that does not run is the empty text.
#[test]
fn the_call_of_a_method_of_the_v1_does_not_check_the_arity() {
    // The method does not run when a parameter has no value.
    let report = run("class C {\n    Say(a) {\n        return \"hi\"\n    }\n}\no := new C\nx := o.Say()");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));

    // A parameter that has a default is fine.
    let report = run("class C {\n    Say(a = 5) {\n        return \"hi\"\n    }\n}\no := new C\nx := o.Say()");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("hi".into())));

    // The arguments that the method does not want are ignored.
    let report = run("class C {\n    Say() {\n        return \"hi\"\n    }\n}\no := new C\nx := o.Say(1, 2)");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("hi".into())));
}

/// The `base` of the v1 is the base of the class where the method was
/// defined.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02.
#[test]
fn the_base_of_the_v1_is_the_base_of_the_class() {
    let report = run(
        "class C {\n    Say() {\n        return \"hi\"\n    }\n}\nclass D extends C {\n    Say() {\n        return base.Say()\n    }\n}\no := new D\nx := o.Say()",
    );
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("hi".into())));
}

/// The math functions of the v1 take the text that holds a number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: a text
/// that does not hold a number is the number zero for most of the functions
/// -- and the empty text for the `Abs`, the `Max`, the `Min` and the `Mod`.
/// The answer keeps the kind of the number, and the number that is outside
/// of the domain of the function is the empty text.
#[test]
fn the_math_functions_of_the_v1_take_the_text_that_holds_a_number() {
    // The answer keeps the kind of the number.
    let report = run("y := Abs(5)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(5)));

    let report = run("y := Abs(-2.5)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Float(2.5)));
    assert_eq!(report.output, vec!["2.500000".to_string()]);

    // A text that does not hold a number is the empty text for the `Abs`.
    let report = run("y := Abs(\"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // It is the number zero for the others.
    let report = run("y := Ceil(\"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));

    let report = run("y := Exp(\"abc\")\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.000000".to_string()]);

    // The whole numbers go up and down.
    let report = run("y := Ceil(2.5)");
    assert_eq!(report.variables.get("y"), Some(&Value::Int(3)));

    let report = run("y := Floor(2.5)");
    assert_eq!(report.variables.get("y"), Some(&Value::Int(2)));

    // A half is rounded away from the zero.
    let report = run("y := Round(5.5)");
    assert_eq!(report.variables.get("y"), Some(&Value::Int(6)));

    // The digits of the second argument are the decimals of the answer.
    let report = run("y := Round(1.234, 2)");
    assert_eq!(report.variables.get("y"), Some(&Value::Text("1.23".into())));

    // The number that is outside of the domain is the empty text.
    let report = run("y := Ln(-2.5)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("y := Sqrt(-2.5)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("y := ASin(5)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // The logarithm of the zero is the infinity of the C library.
    let report = run("y := Ln(0)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-1.#INF00".to_string()]);

    // The remainder of the division keeps the kind of the numbers.
    let report = run("y := Mod(5, 2)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(1)));

    let report = run("y := Mod(5.5, 2)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.500000".to_string()]);

    // The divisor of zero is the empty text.
    let report = run("y := Mod(5, 0)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // The biggest and the smallest take any number of them.
    let report = run("y := Max(1, 9, 4)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(9)));

    let report = run("y := Min(1, 9, 4)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(1)));

    // A text that does not hold a number is the empty text for them too.
    let report = run("y := Max(5, \"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // The number of the arguments is checked at the load.
    let error = ahk::parse_with("y := Abs()", dialect::v1())
        .expect_err("the function wants a number");
    assert_eq!(
        error.message,
        "Too few parameters passed to function.\n     Specifically: Abs()"
    );
}

/// Every variable of the surface of the v1 is a variable that a script may
/// not declare as global.
///
/// The table of the surface is the list of the names of the v1, read out of
/// the binary, and every one of them is refused. The `A_Args` is the one
/// that is not: the interpreter creates it at the load, and it is a
/// variable of the script.
#[test]
fn every_variable_of_the_surface_of_the_v1_may_not_be_declared_global() {
    let surface = include_str!("../reference/ahk-v1.1.37.02.surface.tsv");
    let mut count = 0;
    for line in surface.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') || line.starts_with("kind") {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields[0] != "variable" {
            continue;
        }
        let name = fields[2];
        if name == "A_Args" {
            continue;
        }
        count += 1;
        let error = ahk::parse_with(&format!("global {name}\nx := 1"), dialect::v1())
            .expect_err("the built-in variable may not be declared");
        assert_eq!(
            error.message,
            format!("Built-in variables must not be declared.\n     Specifically: {name}"),
            "the v1 refuses the global of {name}"
        );
    }
    // The surface of the v1 carries 138 variables, and the `A_Args` is the
    // one that is not refused.
    assert_eq!(count, 137);
}

/// The math functions of the v1 read the number that the text starts with.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `Ceil`,
/// the `Floor`, the `Round`, the `Exp`, the `Ln`, the `Log`, the `Sqrt`, the
/// `Sin`, the `Cos`, the `Tan`, the `ASin`, the `ACos` and the `ATan` want
/// the number that the text of the argument starts with -- "65abc" is 65 --
/// and the exponent does not need the point of the number: "1e3" is 1000.
#[test]
fn the_math_functions_of_the_v1_read_the_number_that_the_text_starts_with() {
    // The number that the text starts with is the number.
    let report = run("y := Ceil(\"65abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(65)));

    let report = run("y := Floor(\"007abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(7)));

    // The exponent of the number is part of it.
    let report = run("y := Ceil(\"1e3\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(1000)));

    let report = run("y := Log(\"1.5e2\")\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["2.176091".to_string()]);

    // The text that does not start with a number is the number zero.
    let report = run("y := Log(\"abc65\")\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-1.#INF00".to_string()]);

    // The hex of the v1 is a number.
    let report = run("y := Ceil(\"0x41\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Int(65)));
}

/// The `Abs`, the `Max`, the `Min` and the `Mod` of the v1 want a text that
/// is a number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the text
/// of the argument has to be the number, and "65abc" is not a number at
/// all. The exponent needs the point of the number: "1.5e2" is 150 and
/// "1e2" is not a number.
#[test]
fn the_abs_and_the_max_of_the_v1_want_a_text_that_is_a_number() {
    // A text that is not a number is the empty text.
    let report = run("y := Abs(\"65abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("y := Max(\"65abc\", 1)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("y := Mod(\"65abc\", 2)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // The exponent needs the point.
    let report = run("y := Abs(\"1.5e2\")\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["150.000000".to_string()]);

    let report = run("y := Abs(\"1e2\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// The `Chr` of the v1 takes the whole number that the text starts with.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the code is
/// the whole number that the text of the argument starts with, and it stops
/// at the point: "1e3" is one. A code that is zero, that is negative or
/// that is not a character is the empty text.
#[test]
fn the_chr_of_the_v1_takes_the_whole_number_that_the_text_starts_with() {
    // The text that holds a number is the number.
    let report = run("y := Chr(65)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("A".into())));

    let report = run("y := Chr(\"65\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("A".into())));

    // The number that the text starts with is the number.
    let report = run("y := Chr(\"65abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("A".into())));

    // The whole number stops at the point.
    let report = run("y := Chr(\"1e3\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("\u{1}".into())));

    // The hex of the v1 is a number.
    let report = run("y := Chr(\"0x41\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("A".into())));

    // A text that does not start with a number is the empty text.
    let report = run("y := Chr(\"abc\")");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    // The zero and the number that is negative are the empty text too.
    let report = run("y := Chr(0)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));

    let report = run("y := Chr(-1)");
    assert!(report.ok);
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

/// The text of a float of the v1 has seventeen digits.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// writes the number with seventeen digits and rounds that number to the
/// decimals of the format, and a half goes away from the zero.
#[test]
fn the_text_of_a_float_of_the_v1_has_seventeen_digits() {
    // The number of the v1 is a number of seventeen digits, and not the
    // number that the double holds.
    let report = run("y := Exp(65)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(
        report.output,
        vec!["16948892444103338000000000000.000000".to_string()]
    );

    // A half goes away from the zero, and not to the even number.
    let report = run("y := 0.125+0\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0.125000".to_string()]);

    let report = run("SetFormat, Float, 0.2\ny := 0.125+0\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0.13".to_string()]);

    // The exponent of the scientific notation has three digits.
    let report = run("SetFormat, Float, 0.6e\ny := Exp(30)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.068647e+013".to_string()]);
}

/// The whole number of the v1 that does not fit is the largest one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// keeps the whole numbers in sixty-four bits, and the number that is
/// written and does not fit is the largest of them.
#[test]
fn the_whole_number_of_the_v1_that_does_not_fit_is_the_largest_one() {
    let report = run("y := 100000000000000000000");
    assert!(report.ok);
    assert_eq!(
        report.variables.get("y"),
        Some(&Value::Text("9223372036854775807".into()))
    );
}

/// The body of a class of the v1 has one set of names.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses a name that is declared twice in the body of a class, and it does
/// not care what the member is: a method and a variable with the same name
/// are a duplicate, and so are a static member and one that is not. The
/// refusal names the member that was written.
#[test]
fn the_body_of_a_class_of_the_v1_has_one_set_of_names() {
    // A method and a variable with the same name.
    let error = ahk::parse_with(
        "class C\n{\n    x := 5\n    x() {\n    }\n}",
        dialect::v1(),
    )
    .expect_err("the name is declared twice");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: x"
    );

    // A variable and a method with the same name.
    let error = ahk::parse_with(
        "class C\n{\n    x() {\n    }\n    x := 5\n}",
        dialect::v1(),
    )
    .expect_err("the name is declared twice");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: x"
    );

    // A static member and one that is not.
    let error = ahk::parse_with(
        "class C\n{\n    static x := 5\n    x() {\n    }\n}",
        dialect::v1(),
    )
    .expect_err("the name is declared twice");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: x"
    );

    // The case of the name does not count, and the name is the one that was
    // written.
    let error = ahk::parse_with(
        "class C\n{\n    x := 5\n    X() {\n    }\n}",
        dialect::v1(),
    )
    .expect_err("the name is declared twice");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: X"
    );

    // The `__Init` that has a property before it is refused, and the one
    // that has it after it is taken.
    let error = ahk::parse_with(
        "class C\n{\n    x := 5\n    __Init() {\n    }\n}",
        dialect::v1(),
    )
    .expect_err("the init has a property");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: __Init"
    );

    ahk::parse_with(
        "class C\n{\n    __Init() {\n    }\n    x := 5\n}",
        dialect::v1(),
    )
    .expect("the property that comes after the init is taken");
}

/// A class of the v1 that is declared twice is refused with its own words.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses it with "Duplicate class definition.\n     Specifically: C", and
/// the name is the one that was written.
#[test]
fn a_class_of_the_v1_that_is_declared_twice_is_refused_with_its_own_words() {
    let error = ahk::parse_with("class C\n{\n}\nclass C\n{\n}", dialect::v1())
        .expect_err("the class is declared twice");
    assert_eq!(
        error.message,
        "Duplicate class definition.\n     Specifically: C"
    );

    // The case of the name does not count.
    let error = ahk::parse_with("class C\n{\n}\nclass c\n{\n}", dialect::v1())
        .expect_err("the class is declared twice");
    assert_eq!(
        error.message,
        "Duplicate class definition.\n     Specifically: c"
    );

    // A class inside a class is refused the same way.
    let error = ahk::parse_with(
        "class C\n{\n    class D\n    {\n    }\n    class D\n    {\n    }\n}",
        dialect::v1(),
    )
    .expect_err("the class is declared twice");
    assert_eq!(
        error.message,
        "Duplicate class definition.\n     Specifically: D"
    );
}

/// The parameter of the v1 that wants a number takes the number that the
/// text of it starts with.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// does not check the kind of the arguments -- the built-in of the v1 takes
/// the text that holds a number, and it does not fail -- and the number is
/// the one that the text starts with: `SubStr("abcdef", "2abc")` is "bcdef".
#[test]
fn the_parameter_of_the_v1_takes_the_number_that_the_text_starts_with() {
    let report = run("x := SubStr(\"abcdef\", \"2abc\")\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["bcdef".to_string()]);
}

/// The hex of the v1 is a number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
/// `SubStr("abcdef", "0x2")` is "bcdef".
#[test]
fn the_hex_of_the_v1_is_a_number() {
    let report = run("x := SubStr(\"abcdef\", \"0x2\")\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["bcdef".to_string()]);
}

/// The place that is zero is the last character of the text.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
/// `SubStr("abcdef", 0)` is "f".
#[test]
fn the_place_that_is_zero_is_the_last_character_of_the_text() {
    let report = run("x := SubStr(\"abcdef\", 0)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["f".to_string()]);
}

/// A place that is negative counts from the end of the text.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
/// `SubStr("abcdef", -1)` is "ef" and `SubStr("abcdef", -2)` is "def": the v1
/// takes one character more than the v2 takes.
#[test]
fn a_place_that_is_negative_counts_from_the_end_of_the_text() {
    let report = run("x := SubStr(\"abcdef\", -1)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["ef".to_string()]);

    let report = run("x := SubStr(\"abcdef\", -2)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["def".to_string()]);
}

/// The sense of the case of the v1 is the number that the text holds.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// reads the sense of the case as a number: "1" is the case that is sensitive
/// and "Off" is not, and the v2 refuses the text of it.
#[test]
fn the_sense_of_the_case_of_the_v1_is_the_number_that_the_text_holds() {
    let report = run("x := InStr(\"ABCdef\", \"cd\", \"1\")\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0".to_string()]);

    let report = run("x := InStr(\"ABCdef\", \"cd\", \"Off\")\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["3".to_string()]);
}

/// The needle that is empty is at the first character of the haystack.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
/// `InStr("abcdef", "")` is 1, and the v2 refuses it.
#[test]
fn the_needle_that_is_empty_is_at_the_first_character_of_the_haystack() {
    let report = run("x := InStr(\"abcdef\", \"\")\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);
}

/// The parameter of a command of the v1 that wants a number is an expression.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `Exit, 1+1` leaves the code at 2, and the documentation of the v1 says so:
/// "Numeric parameters accept a literal number or an expression".
#[test]
fn the_parameter_of_a_command_of_the_v1_that_wants_a_number_is_an_expression() {
    let report = run("Exit, 1+1");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(2));

    // The call of a function is an expression too.
    let report = run("Exit, StrLen(\"abcd\")");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(4));
}

/// The parameter of a command of the v1 reads the variable of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `Random, x, n, n` leaves `x` at the value of `n`, and the `Exit, n` leaves
/// the code at it too.
#[test]
fn the_parameter_of_a_command_of_the_v1_reads_the_variable_of_the_script() {
    let report = run("n := 5\nRandom, x, n, n");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(5)));

    let report = run("n := 5\nExit, n");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(5));
}

/// The parameter of a command of the v1 that is not an expression is zero.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `Exit, 100abc` leaves the code at zero, and the `Exit, 1+` as well.
#[test]
fn the_parameter_of_a_command_of_the_v1_that_is_not_an_expression_is_zero() {
    let report = run("Exit, 1+");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(0));

    let report = run("Exit, 100abc");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(0));
}

/// The parameter of a command of the v1 that is a text is the number that the
/// text holds.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `Exit, "3"` leaves the code at 3, and the `Exit, 0x10` leaves it at 16.
#[test]
fn the_parameter_of_a_command_of_the_v1_that_is_a_text_is_the_number_that_it_holds() {
    let report = run("Exit, \"3\"");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(3));

    let report = run("Exit, 0x10");
    assert!(report.ok);
    assert_eq!(report.exit_code, Some(16));
}

/// The class of the v1 has no static method.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses it with "Invalid class variable declaration.\n     Specifically:
/// __Init() {", and the text is the member, from the name to the brace that
/// opens the body.
#[test]
fn the_class_of_the_v1_has_no_static_method() {
    let error = ahk::parse_with("class C\n{\n    static __Init() {\n    }\n}", dialect::v1())
        .expect_err("the static method is refused");
    assert_eq!(
        error.message,
        "Invalid class variable declaration.\n     Specifically: __Init() {"
    );

    let error =
        ahk::parse_with("class C\n{\n    static Say() {\n        return 1\n    }\n}", dialect::v1())
            .expect_err("the static method is refused");
    assert_eq!(
        error.message,
        "Invalid class variable declaration.\n     Specifically: Say() {"
    );
}

/// The class of the v1 has no property with a getter.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the getter
/// that is on the same line as the name is refused with "Not a valid method,
/// class or property definition.\n     Specifically: x { get => 1 }", and the
/// getter that is in a block with "Not a valid property
/// getter/setter.\n     Specifically: get => 1".
#[test]
fn the_class_of_the_v1_has_no_property_with_a_getter() {
    let error = ahk::parse_with("class C\n{\n    x { get => 1 }\n}", dialect::v1())
        .expect_err("the getter is refused");
    assert_eq!(
        error.message,
        "Not a valid method, class or property definition.\n     Specifically: x { get => 1 }"
    );

    let error = ahk::parse_with("class C\n{\n    x {\n        get => 1\n    }\n}", dialect::v1())
        .expect_err("the getter is refused");
    assert_eq!(
        error.message,
        "Not a valid property getter/setter.\n     Specifically: get => 1"
    );
}

/// The name of a class of the v1 is the name of a variable.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// takes the name of a class as the name of a variable: `C := 5` is accepted,
/// and the value of the name is the one that was written. The v2 refuses it
/// with "This Class cannot be used as an output variable.".
#[test]
fn the_name_of_a_class_of_the_v1_is_the_name_of_a_variable() {
    let report = run("class C\n{\n}\nC := 5\nFileAppend, %C%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);

    // The name of a class of the machine is a variable as well.
    let report = run("Abs := 5\nFileAppend, %Abs%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);
}

/// The call of a built-in of the v1 goes to the built-in.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name
/// of a built-in is the name of a variable, and the call still goes to the
/// built-in: `Abs := 5` and then `Abs(3)` answers 3.
#[test]
fn the_call_of_a_built_in_of_the_v1_goes_to_the_built_in() {
    let report = run("Abs := 5\nx := Abs(3)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["3".to_string()]);
}

/// A class and a function of the v1 may carry the same name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// takes the two, in either order: the call goes to the function and the
/// `new` goes to the class. The v2 refuses the second one with "This
/// function declaration conflicts with an existing Class.".
#[test]
fn a_class_and_a_function_of_the_v1_may_carry_the_same_name() {
    let report = run(
        "class C\n{\n    Say() {\n        return \"said\"\n    }\n}\nC() {\n    return 1\n}\n\
         x := C()\no := new C\ny := o.Say()\nFileAppend, %x% %y%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["1 said".to_string()]);

    // The function may come first.
    let report = run("C() {\n    return 1\n}\nclass C\n{\n}\nx := C()\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);
}

/// The function that is declared twice has the words of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses it with "Duplicate function definition.\n     Specifically: C()",
/// and the name is written the way the script spelled it and with the
/// parenthesis of the call.
#[test]
fn the_function_that_is_declared_twice_has_the_words_of_the_v1() {
    let error = ahk::parse_with("C() {\n    return 1\n}\nC() {\n    return 2\n}", dialect::v1())
        .expect_err("the function is refused");
    assert_eq!(error.message, "Duplicate function definition.\n     Specifically: C()");

    let error = ahk::parse_with("Foo() {\n    return 1\n}\nfoo() {\n    return 2\n}", dialect::v1())
        .expect_err("the function is refused");
    assert_eq!(error.message, "Duplicate function definition.\n     Specifically: foo()");
}

/// The `new` of the v1 reads the variable of the name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the class
/// of the v1 is a variable: `new v` makes an instance of the class that `v`
/// holds, and a name that holds another value makes nothing, and the v1 does
/// not fail.
#[test]
fn the_new_of_the_v1_reads_the_variable_of_the_name() {
    let report = run(
        "class C\n{\n    Say() {\n        return \"said\"\n    }\n}\nv := C\no := new v\n\
         x := o.Say()\nFileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["said".to_string()]);

    // The name that holds another value makes nothing, and the v1 does not
    // fail.
    let report = run("class C\n{\n}\nC := 5\no := new C\nFileAppend, [%o%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[]".to_string()]);
}

/// The `new` of the v1 is a statement of its own.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: `new C`
/// makes an instance of the class and throws it away, and the v1 does not
/// fail when the name holds something else.
#[test]
fn the_new_of_the_v1_is_a_statement_of_its_own() {
    let report = run("class C\n{\n}\nnew C\nFileAppend, ok, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["ok".to_string()]);

    let report = run("new Nope\nFileAppend, ok, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["ok".to_string()]);
}

/// The name of a class of the v1 may be a word of the language.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// takes the name of a class as it is written: the `class if` is a class, and
/// the `new if` makes an instance of it. The v2 refuses the name with "The
/// following reserved word must not be used as a class name.".
#[test]
fn the_name_of_a_class_of_the_v1_may_be_a_word_of_the_language() {
    let report = run(
        "class if\n{\n    Say() {\n        return \"said\"\n    }\n}\no := new if\nx := o.Say()\n\
         FileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["said".to_string()]);
}

/// The brace that opens a class of the v1 names the class.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// names the text that comes after the word "class": the `class C { }` is a
/// "Missing \"{\"\n     Specifically: C { }", and the `class C` is a
/// "Missing \"{\"\n     Specifically: C". The v2 names the whole of the line.
#[test]
fn the_brace_that_opens_a_class_of_the_v1_names_the_class() {
    let error = ahk::parse_with("class C { }", dialect::v1())
        .expect_err("the brace of the class is refused");
    assert_eq!(error.message, "Missing \"{\"\n     Specifically: C { }");

    let error = ahk::parse_with("class C", dialect::v1())
        .expect_err("the brace of the class is refused");
    assert_eq!(error.message, "Missing \"{\"\n     Specifically: C");
}

/// The class of the v1 that is not closed names the class.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses it with "Missing \"}\"\n     Specifically: C", and the name of the
/// class is in it even when the file ends inside a method of it.
#[test]
fn the_class_of_the_v1_that_is_not_closed_names_the_class() {
    let error = ahk::parse_with("class C\n{\n", dialect::v1())
        .expect_err("the brace that closes the class is refused");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: C");

    let error = ahk::parse_with("class C\n{\n    Say() {\n", dialect::v1())
        .expect_err("the brace that closes the class is refused");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: C");
}

/// The word "class" alone holds no action of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses the line with "This line does not contain a recognized action.\n
/// Specifically: class", and the word is a variable when it is assigned.
#[test]
fn the_word_class_alone_holds_no_action_of_the_v1() {
    let error = ahk::parse_with("class", dialect::v1())
        .expect_err("the word alone is refused");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: class"
    );

    // The word is a variable when it is assigned.
    let report = run("class := 5\nFileAppend, %class%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);
}

/// The base of a class of the v1 is a class of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1 has
/// no class types, and a base that is not a class of the script is refused
/// with "Unknown class.\n     Specifically: Object". The v2 takes the base of
/// a class type of the runtime.
#[test]
fn the_base_of_a_class_of_the_v1_is_a_class_of_the_script() {
    let error = ahk::parse_with("class C extends Object\n{\n}", dialect::v1())
        .expect_err("the base is refused");
    assert_eq!(error.message, "Unknown class.\n     Specifically: Object");

    // The base that is a class of the script is taken.
    let report = run("class A\n{\n}\nclass B extends A\n{\n}\nFileAppend, ok, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["ok".to_string()]);
}

/// The function of the v1 does not see the scope of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: every
/// function of the v1 has its own scope: the name of the script is not
/// visible in the function, and only the `global` declaration makes it
/// visible. The v2 reads the scope of the script when the name is only read.
#[test]
fn the_function_of_the_v1_does_not_see_the_scope_of_the_script() {
    // The name of the script is not visible in the function.
    let report = run("x := 1\nf() {\n    return x\n}\nr := f()\nFileAppend, [%r%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[]".to_string()]);

    // The name that the function declares global is visible.
    let report = run("x := 1\nf() {\n    global x\n    return x\n}\nr := f()\nFileAppend, [%r%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[1]".to_string()]);

    // The write of the function is its own, and the scope of the script is
    // not touched.
    let report = run("x := 1\nf() {\n    x := 2\n}\nf()\nFileAppend, [%x%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[1]".to_string()]);
}

/// The function of the v1 does not share the scopes around the call.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name of
/// an outer call is not visible in the function that it calls, and the write
/// of it is in the scope of the function that runs.
#[test]
fn the_function_of_the_v1_does_not_share_the_scopes_around_the_call() {
    let report = run(
        "x := 5\ng() {\n    return x\n}\nf() {\n    local x := 1\n    return g()\n}\nr := f()\n\
         FileAppend, [%r%], *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["[]".to_string()]);
}

/// The name of the v1 may begin with a number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// reads a name that begins with a number: the "1C" is a name, the "class 1C"
/// is a class, the "1F()" is a function and the "f(1a)" is a function with a
/// parameter that is named "1a". The v2 refuses the script at the load.
#[test]
fn the_name_of_the_v1_may_begin_with_a_number() {
    // The class whose name begins with a number.
    let report = run(
        "class 1C\n{\n    Say() {\n        return \"said\"\n    }\n}\no := new 1C\nx := o.Say()\n\
         FileAppend, %x%, *",
    );
    assert!(report.ok);
    assert_eq!(report.output, vec!["said".to_string()]);

    // The function whose name begins with a number.
    let report = run("1F() {\n    return 1\n}\nx := 1F()\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);

    // The variable whose name begins with a number.
    let report = run("1C := 5\nFileAppend, %1C%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["5".to_string()]);

    // The name that is not assigned is the empty text.
    let report = run("x := 1C\nFileAppend, [%x%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[]".to_string()]);

    // The parameter whose name begins with a number.
    let report = run("f(1a) {\n    return 1\n}\nx := f(1)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);
}

/// The exponent of the v1 needs a decimal point.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// scientific notation of the v1 needs a decimal point: the "1.0e4" is the
/// number 10000, and the "1e2" is a name. The v2 reads the "1e2" as the
/// number 100.
#[test]
fn the_exponent_of_the_v1_needs_a_point() {
    // The exponent without a point is a name, and the name that is not
    // assigned is the empty text.
    let report = run("x := 1e2\nFileAppend, [%x%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[]".to_string()]);

    // The name that carries the exponent may be assigned.
    let report = run("1e2 := 5\nx := 1e2\nFileAppend, [%x%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[5]".to_string()]);

    // The exponent with a point is a number.
    let report = run("x := 1.5e2\ny := x + 0\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["150.000000".to_string()]);
}

/// The number of the v1 keeps the text that was written.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// number of the v1 is a "pure number": it keeps the text that was
/// written -- the "0x10" is "0x10", and the "00123" is "00123", and the
/// "1.50" is "1.50" -- and that text is what the value answers when it is
/// read as a text. The value of it is the number that the text holds, and
/// the operations that want a number ask the text for it.
#[test]
fn the_number_of_the_v1_keeps_the_text_that_was_written() {
    // The hex keeps the text.
    let report = run("x := 0x10");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0x10".into())));

    // The zeros in front of the number keep the text.
    let report = run("x := 00123");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("00123".into())));

    // The float keeps the text, and so does the exponent.
    let report = run("x := 1.50");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1.50".into())));
    let report = run("x := 1.5e2");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1.5e2".into())));

    // The text of it is the text that was written: the length of the "0x10"
    // is four, and the concatenation of it is the text.
    let report = run("x := 0x10\ny := StrLen(x)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["4".to_string()]);
    let report = run("x := 0x10\nFileAppend, [%x%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[0x10]".to_string()]);

    // The arithmetic asks the text for the number: the "0x10" is the
    // number 16.
    let report = run("x := 0x10\ny := x + 1\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["17".to_string()]);

    // The comparison is a number when both sides are numbers, and a text
    // otherwise: the "0x10" is the number 16, and the "0x10" that is
    // written between quotes is a text that is not a number.
    let report = run("x := 0x10\ny := (x = 16)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);
    let report = run("x := 0x10\ny := (x = \"16\")\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0".to_string()]);

    // The number of the v1 that is passed to a function keeps the text.
    let report = run("y := StrLen(0x10)\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["4".to_string()]);
}

/// The number of the v1 that does not fit is the one that fits.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// whole number of base ten that does not fit in the machine is the
/// largest one that fits -- the "100000000000000000000" is
/// "9223372036854775807", and the "-100000000000000000000" is
/// "-9223372036854775808". The hex and the float keep the text that was
/// written, and the value of the hex that does not fit is the largest
/// whole number that fits.
#[test]
fn the_number_of_the_v1_that_does_not_fit_is_the_one_that_fits() {
    let report = run("x := 100000000000000000000");
    assert!(report.ok);
    assert_eq!(
        report.variables.get("x"),
        Some(&Value::Text("9223372036854775807".into()))
    );
    let report = run("x := -100000000000000000000");
    assert!(report.ok);
    assert_eq!(
        report.variables.get("x"),
        Some(&Value::Text("-9223372036854775808".into()))
    );

    // The hex keeps the text, and its value is the largest one that fits.
    let report = run("x := 0xFFFFFFFFFFFFFFFF");
    assert!(report.ok);
    assert_eq!(
        report.variables.get("x"),
        Some(&Value::Text("0xFFFFFFFFFFFFFFFF".into()))
    );
    let report = run("x := 0xFFFFFFFFFFFFFFFF\ny := x + 0\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["9223372036854775807".to_string()]);

    // The float that is too big keeps the text, and its value is the
    // infinity of the machine.
    let report = run("x := 1.5e400");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1.5e400".into())));
}

/// The sign of the number of the v1 is part of the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// plus of the v1 changes nothing -- the "+0x10" is "0x10" -- and the
/// minus gives the text of the number that it negates -- the "-00123" is
/// "-123", and the "-0x10" is "-16" -- while the float keeps the text that
/// was written, with the sign in front of it: the "-1.50" is "-1.50". The
/// sign of a number that is in a group is an operation of its own: the
/// "-(1.50)" is the number that the group holds, negated.
#[test]
fn the_sign_of_the_number_of_the_v1_is_part_of_the_number() {
    let report = run("x := +0x10");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0x10".into())));
    let report = run("x := +00123");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("00123".into())));
    let report = run("x := -00123");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("-123".into())));
    let report = run("x := -0x10");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("-16".into())));
    let report = run("x := -1.50");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("-1.50".into())));

    // The group keeps the sign out of the number.
    let report = run("x := -(1.50)\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-1.500000".to_string()]);

    // The plus of a variable changes nothing either.
    let report = run("x := 00123\ny := +x\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["00123".to_string()]);
}

/// The whole number of the v1 wraps the way the machine does.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the sum
/// of the 9223372036854775807 and the one is -9223372036854775808, and
/// the product of it and the two is -2.
#[test]
fn the_whole_number_of_the_v1_wraps_the_way_the_machine_does() {
    let report = run("x := 9223372036854775807\ny := x + 1\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-9223372036854775808".to_string()]);
    let report = run("x := 9223372036854775807\ny := x * 2\nFileAppend, %y%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-2".to_string()]);
    let report = run("x := 9223372036854775807\nx++\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-9223372036854775808".to_string()]);
}

/// The operators that write a variable of the v1 read the number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// values are numbers when they can be one -- the "5" is 5, and the
/// "0x10" is 16 -- and the text that is not a number is the number zero:
/// the `x += 1` of the "abc" is 1. The whole numbers stay whole numbers,
/// and the division of two whole numbers is a whole number: the `x /= 2`
/// of the 5 is 2.
#[test]
fn the_operators_that_write_a_variable_of_the_v1_read_the_number() {
    let report = run("x := 5\nx += 1\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["6".to_string()]);
    let report = run("x := 0x10\nx += 1\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["17".to_string()]);
    let report = run("x := \"abc\"\nx += 1\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1".to_string()]);
    let report = run("x := 5\nx /= 2\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["2".to_string()]);
    let report = run("x := 5.0\nx /= 2\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["2.500000".to_string()]);
}

/// The format of the v1 writes the number and not the pure number.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// whole number that is written without the zeros in front of it is a
/// number -- the `255` is the number 255, and the format of the integers
/// writes it -- and the number with the zeros in front of it, the hex and
/// the float are pure numbers: they keep the text that was written. The
/// sign of the minus is a number of its own: the v1 computes it, and the
/// format writes it.
#[test]
fn the_format_of_the_v1_writes_the_number_and_not_the_pure_number() {
    let report = run("SetFormat, Integer, H\nx := 255\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0xFF".to_string()]);

    // The sign of the minus is in front of the prefix.
    let report = run("SetFormat, Integer, H\nx := -255\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["-0xFF".to_string()]);

    // The pure numbers keep the text that was written.
    let report = run("SetFormat, Integer, H\nx := 00123\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["00123".to_string()]);
    let report = run("SetFormat, Integer, D\nx := 0x10\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["0x10".to_string()]);
    let report = run("SetFormat, Float, 0.3\nx := 1.50\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.50".to_string()]);

    // The number that is computed follows the format.
    let report = run("SetFormat, Float, 0.3\nx := 1.50 + 0\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["1.500".to_string()]);
}

/// The parameter that the v1 does not want is refused at the load.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// checks the parameter of its commands when the script loads, and the
/// complaint of the v1 is "Parameter #N invalid." with the text that the
/// script wrote. The text is the text of the parameter, and not its value.
#[test]
fn the_parameter_that_the_v1_does_not_want_is_refused_at_the_load() {
    let cases = [
        ("SetTitleMatchMode, abc", 1, "abc"),
        ("SetTitleMatchMode, 1+1", 1, "1+1"),
        ("SetStoreCapsLockMode, abc", 1, "abc"),
        ("SendMode, abc", 1, "abc"),
        ("DetectHiddenText, abc", 1, "abc"),
        ("DetectHiddenWindows, abc", 1, "abc"),
        ("FileEncoding, abc", 1, "abc"),
        ("SetRegView, abc", 1, "abc"),
        ("AutoTrim, abc", 1, "abc"),
        ("StringCaseSense, abc", 1, "abc"),
        ("SetBatchLines, abc", 1, "abc"),
        ("SetFormat, abc, H", 1, "abc"),
        ("SetFormat, Integer, abc", 2, "abc"),
        ("Transform, out, Hex, 255", 2, "Hex"),
        ("StringGetPos, pos, text, a, 0", 4, "0"),
        ("FileGetTime, out, file, D", 3, "D"),
        ("FileGetSize, out, file, G", 3, "G"),
        ("FileSetAttrib, abc, file", 1, "abc"),
    ];
    for (script, number, text) in cases {
        assert_eq!(
            the_refusal_of_the_load(script),
            format!("Parameter #{number} invalid.\n     Specifically: {text}"),
            "the script is {script}"
        );
    }
}

/// The command that the v1 does not check takes what it is given.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `SetControlDelay`, the `SetWinDelay`, the `SetMouseDelay`, the
/// `SetKeyDelay` and the `CoordMode` do not refuse the value that they do
/// not know: the "abc" is zero, and the mode that is not the "Play" is the
/// mode of the input.
#[test]
fn the_command_that_the_v1_does_not_check_takes_what_it_is_given() {
    for script in [
        "SetControlDelay, abc",
        "SetControlDelay, -5",
        "SetControlDelay, 20abc",
        "SetWinDelay, abc",
        "SetKeyDelay, 10, abc",
        "SetKeyDelay, 10, Play",
        "CoordMode, abc",
        "CoordMode, Mouse, abc",
        "CoordMode, Mouse, Screen",
        "FileEncoding, ",
    ] {
        let report = run(script);
        assert!(report.ok, "the script is {script}: {:?}", report.error);
    }

    // The delay and the speed of the pointer are the machine's: the fake
    // has no pointer, and the refusal is the one of the machine and not the
    // one of the parameter.
    for script in ["SetMouseDelay, 10, abc", "SetDefaultMouseSpeed, abc"] {
        let report = run(script);
        assert!(!report.ok, "the script is {script}");
        assert!(
            report
                .error
                .as_deref()
                .unwrap_or("")
                .starts_with("this desktop cannot change"),
            "the script is {script}: {:?}",
            report.error
        );
    }
}

/// The `CoordMode` of the v1 with a mode that it does not know sets every
/// mode to the "Event".
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the mode
/// that the v1 does not know is the "Event", and it is the mode of every
/// target: the mouse, the pixel, the caret, the menu and the tooltip. The
/// target that the v1 does not know changes nothing.
#[test]
fn the_coord_mode_of_the_v1_that_is_not_one_sets_every_mode_to_the_event() {
    let report = run("CoordMode, Mouse, abc\nx := A_CoordModePixel\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["Event".to_string()]);

    // The target that the v1 does not know changes nothing.
    let report = run("CoordMode, Mouse, Screen\nCoordMode, abc\nx := A_CoordModeMouse\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["Screen".to_string()]);
}

/// The settings of the v1 begin with the values of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the mode
/// of the coordinates is the "Window", the method of the sending is the
/// "Event", the mode of the matching of the titles is the 1, the encoding
/// of the files is the empty text and the key that closes a menu is the
/// empty text. The flags are the "On" and the "Off", and not the number one
/// and zero that the v2 answers.
#[test]
fn the_settings_of_the_v1_begin_with_the_values_of_the_v1() {
    let report = run("x := A_CoordModeMouse\ny := A_SendMode\nz := A_TitleMatchMode\nFileAppend, %x% %y% %z%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["Window Event 1".to_string()]);

    let report = run("x := A_FileEncoding\ny := A_MenuMaskKey\nFileAppend, [%x%][%y%], *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["[][]".to_string()]);

    let report = run("x := A_StoreCapsLockMode\ny := A_DetectHiddenWindows\nz := A_DetectHiddenText\nFileAppend, %x% %y% %z%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["On Off On".to_string()]);
}

/// The flag of the v1 follows the command that turns it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the flag
/// is the "On" and the "Off", and the command that sets it is the one that
/// the v1 has.
#[test]
fn the_flag_of_the_v1_follows_the_command_that_turns_it() {
    let report = run("SetStoreCapsLockMode, Off\nDetectHiddenWindows, On\nDetectHiddenText, Off\nx := A_StoreCapsLockMode\ny := A_DetectHiddenWindows\nz := A_DetectHiddenText\nFileAppend, %x% %y% %z%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["Off On Off".to_string()]);
}

/// The encoding of the files of the v1 is the name of the encoding.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the code
/// page of the machine is the empty text, and the name of the encoding is
/// the name after the command that sets it.
#[test]
fn the_encoding_of_the_files_of_the_v1_is_the_name_of_the_encoding() {
    let report = run("FileEncoding, UTF-8\nx := A_FileEncoding\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["UTF-8".to_string()]);

    let report = run("FileEncoding, UTF-8\nFileEncoding, CP0\nx := A_FileEncoding\nFileAppend, %x%, *");
    assert!(report.ok);
    assert_eq!(report.output, vec!["".to_string()]);
}



/// The number of the parameters of a command of the v1 is the number of the
/// binary.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// command of the `GetKeyState` wants two parameters -- the key and the mode
/// -- and the complaint of the v1 names them. The `EnvSub` wants two, and the
/// complaint that names the command comes first. The `ControlSend` and the
/// `GuiControl` want two, and the `WinMenuSelectItem` wants three, and the
/// complaint of the v1 names the parameter that is missing: "Parameter #N
/// required". The name of the `SetStoreCapsLockMode` is the one that the
/// binary writes, and it is not the name of the surface: "SetStoreCapslockMode".
#[test]
fn the_number_of_the_parameters_of_a_command_of_the_v1_is_the_number_of_the_binary() {
    let cases = [
        (
            "GetKeyState",
            "\"GetKeyState\" requires at least 2 parameters.\n     Specifically: GetKeyState",
        ),
        (
            "GetKeyState, a",
            "\"GetKeyState\" requires at least 2 parameters.\n     Specifically: GetKeyState, a",
        ),
        (
            "EnvSub",
            "\"EnvSub\" requires at least 1 parameter.\n     Specifically: EnvSub",
        ),
        ("EnvSub, a", "Parameter #2 required"),
        ("ControlSend", "Parameter #2 required"),
        ("ControlSend, a", "Parameter #2 required"),
        ("GuiControl", "Parameter #2 required"),
        ("GuiControl, a", "Parameter #2 required"),
        ("WinMenuSelectItem", "Parameter #3 required"),
        ("WinMenuSelectItem, a", "Parameter #3 required"),
        ("WinMenuSelectItem, a, b", "Parameter #3 required"),
        (
            "SetStoreCapsLockMode",
            "\"SetStoreCapslockMode\" requires at least 1 parameter.\n     Specifically: SetStoreCapsLockMode",
        ),
    ];
    for (script, refusal) in cases {
        assert_eq!(
            the_refusal_of_the_load(script),
            refusal,
            "the script is {script}"
        );
    }
}

/// The call of a name that is not a function is refused at the load, and a
/// call that is a statement is a call.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `MyFunc()` is a statement, and it is refused at the load with "Call to
/// nonexistent function." and the text of the call is named. The call of a
/// built-in is a call too, and the answer of it is thrown away.
#[test]
fn the_call_of_a_name_that_is_not_a_function_is_refused_at_the_load() {
    let cases = [
        ("MyFunc()", "Call to nonexistent function.\n     Specifically: MyFunc()"),
        (
            "MyFunc(1, 2)",
            "Call to nonexistent function.\n     Specifically: MyFunc(1, 2)",
        ),
        (
            "x := MyFunc()",
            "Call to nonexistent function.\n     Specifically: MyFunc()",
        ),
    ];
    for (script, refusal) in cases {
        assert_eq!(
            the_refusal_of_the_load(script),
            refusal,
            "the script is {script}"
        );
    }

    // The call of a built-in is a call, and the answer of it is thrown away.
    let report = run("StrLen(\"abc\")");
    assert!(report.ok, "{:?}", report.error);
}
