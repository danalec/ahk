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
impl Files for Fake {
    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        // The cases that read a file write one that is there.
        let text = std::fs::read_to_string(path)
            .map_err(|reason| format!("{reason}"))?;
        Ok(text.lines().map(|line| line.to_string()).collect())
    }
}
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

/// The built-in variables of the v1 are read-only, and the arguments of the
/// script are the only one that a script may write.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the write of
/// `A_AutoTrim` is refused with "Not allowed as an output variable.", and
/// the write of `A_Args` is kept.
#[test]
fn the_built_in_variables_of_the_v1_are_read_only() {
    let report = run("A_AutoTrim := \"Off\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Not allowed as an output variable.\n     Specifically: A_AutoTrim")
    );

    let report = run("A_Args := \"written\"\nx := A_Args");
    assert!(report.ok);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("written".into())));
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
    assert_eq!(report.variables.get("y"), Some(&Value::Int(5)));
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

    // The value that the command does not know is refused.
    let report = run("AutoTrim, Maybe");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 invalid.\n     Specifically: Maybe")
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
    assert_eq!(report.variables.get("x"), Some(&Value::Int(255)));
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
/// with "Parameter #2 invalid." and the value in the `Extra`.
#[test]
fn the_format_that_the_set_format_does_not_know_is_refused() {
    let report = run("SetFormat, Integer, X");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #2 invalid.\n     Specifically: X")
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
