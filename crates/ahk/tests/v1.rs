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
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that does nothing: the cases of the dialect do not need the
/// machine, and the methods that a capability does not have answer that
/// they do not have it.
struct Fake;

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}
impl Clipboard for Fake {}
impl Files for Fake {}
impl Hotkeys for Fake {}
impl Native for Fake {}
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
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Err("this fake has no processes".into())
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
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements)
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
