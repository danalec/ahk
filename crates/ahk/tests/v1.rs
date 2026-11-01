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
/// "Parameter #1 invalid." and the value is in the `Extra`.
#[test]
fn the_value_that_the_string_case_sense_does_not_know_is_refused() {
    let report = run("StringCaseSense, Maybe");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 invalid.\n     Specifically: Maybe")
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
/// "Parameter #2 invalid." and the sub-command is in the `Extra`.
#[test]
fn the_sub_command_that_the_transform_of_the_v1_does_not_know_is_refused() {
    let report = run("Transform, out, Hex, 255");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #2 invalid.\n     Specifically: Hex")
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
/// one of the v1, and it names the occurrence that was written.
#[test]
fn the_occurrence_of_the_string_get_pos_of_the_v1_that_is_not_one_is_refused() {
    let report = run("text := \"banana\"\nStringGetPos, pos, text, a, 0");
    assert!(!report.ok);
    assert_eq!(
        report.error,
        Some("Parameter #4 invalid.\n     Specifically: 0".to_string())
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
