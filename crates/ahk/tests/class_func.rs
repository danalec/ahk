//! The `Func` class, against a fake desktop.
//!
//! A function never touches the machine: the desktop here is a fake that
//! answers nothing, because nothing is asked of it.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}

impl Files for Fake {}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
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
        Ok(String::new())
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Windows for Fake {}

impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

impl Native for Fake {}

/// A script, and the report of it.
fn run(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

#[test]
fn a_function_has_no_address() {
    // Measured on the binary of v2.0.19 on 2026-10-30: a function has no
    // `Ptr` and no `Size`, and the tree does not carry them either.
    let report = run("f := Abs\nx := f.Ptr");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Func\" has no property named \"Ptr\".")
    );
}

#[test]
fn a_function_has_no_address_to_call() {
    let report = run("f := Abs\nx := f.Ptr()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Func\" has no method named \"Ptr\".")
    );
}

#[test]
fn a_function_is_not_made_again() {
    let report = run("f := Abs\nx := f.__New()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Func\" has no method named \"__New\".")
    );
}

#[test]
fn a_function_is_not_made() {
    // Measured on the binary of v2.0.19 on 2026-10-30: v2 does not make
    // a function, and the call goes through the base object.
    let report = run(
        "try {\n    x := Func()\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid base. | Object.Call | "]);
}

#[test]
fn the_name_of_a_function_may_not_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // function is a property that may not be written, and the write of it is
    // refused with "Property is read-only.", the name of the property is in
    // the Extra, and the What is empty.
    let report = run(
        "try {\n    f := (a, b) => a\n    f.Name := \"x\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Property is read-only. |  | Name"]);
}
/// The `Bind` of a function binds it to an object.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
/// `t.Method.Bind(t)` is a value of the class `BoundFunc`, and the call of it
/// calls the method with the object.
#[test]
fn the_bind_of_a_method_calls_it_with_the_object() {
    let report = run(
        "class Thing {\n    __New() {\n        this.v := 42\n    }\n    Method(x) {\n        return this.v + x\n    }\n}\nt := Thing()\nf := t.Method.Bind(t)\nMsgBox f(8)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["50"]);
}

/// The `Bind` of a built-in is made too, and the call of it fails: the object
/// is the first argument of the call of the function that is bound, and the
/// built-in takes one parameter.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
/// `Abs.Bind(5)` is a value of the class `BoundFunc`, and the call of it with
/// an argument fails.
#[test]
fn the_bind_of_a_builtin_is_made_and_its_call_fails() {
    let report = run("f := Abs.Bind(5)\nMsgBox f(3)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.")
    );
}
