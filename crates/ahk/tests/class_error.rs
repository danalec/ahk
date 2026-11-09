//! The `Error` class, and the classes that derive from it.
//!
//! The errors of v2 are a tree of classes: a property that is not there
//! is an `UnsetError`, a `MemberError` and a `PropertyError` at the
//! same time. The cases here are the ones that were measured on the
//! binary of v2.0.19 on 2026-10-29.

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
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_source(source)
        .run(&statements)
}

#[test]
fn an_error_carries_its_message() {
    let report = run("e := TypeError(\"the message\")\nMsgBox e.Message");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the message"]);
}

#[test]
fn an_error_of_a_class_is_an_error() {
    let report = run("e := TypeError(\"the message\")\nMsgBox e is Error");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn an_error_of_a_class_is_of_its_own_class() {
    let report = run("e := TypeError(\"the message\")\nMsgBox e is TypeError");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_plain_error_is_not_of_another_class() {
    let report = run("e := Error(\"the message\")\nMsgBox e is TypeError");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_property_error_is_a_member_error_and_an_unset_error() {
    let report = run(
        "e := PropertyError(\"the message\")\nMsgBox e is MemberError\nMsgBox e is UnsetError\nMsgBox e is Error",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "1", "1"]);
}

#[test]
fn an_index_error_is_a_value_error() {
    let report = run("e := IndexError(\"the message\")\nMsgBox e is ValueError");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn an_error_with_no_message_is_named_after_its_class() {
    let report = run("e := TypeError()\nMsgBox e.Message");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["TypeError"]);
}

#[test]
fn the_type_of_an_error_is_its_class() {
    let report = run("e := TypeError(\"the message\")\nMsgBox e.Nothing");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"TypeError\" has no property named \"Nothing\".")
    );
}

#[test]
fn an_error_carries_what_and_extra() {
    let report = run(
        "e := TypeError(\"the message\", \"the what\", \"the extra\")\nMsgBox e.What . \" \" . e.Extra",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the what the extra"]);
}

#[test]
fn every_class_of_the_errors_is_carried() {
    // The list is the one of the documentation of v2.
    let classes = [
        "MemoryError",
        "OSError",
        "TargetError",
        "TimeoutError",
        "TypeError",
        "UnsetError",
        "MemberError",
        "PropertyError",
        "MethodError",
        "UnsetItemError",
        "ValueError",
        "IndexError",
        "ZeroDivisionError",
    ];
    for class in classes {
        let report = run(&format!(
            "e := {class}(\"the message\")\nMsgBox e.Message . \" \" . (e is Error)"
        ));
        assert!(report.ok, "{class}: {:?}", report.error);
        assert_eq!(report.output, vec!["the message 1"], "{class}");
    }
}

#[test]
fn a_division_by_zero_is_a_zero_division_error() {
    // Measured on the binary of v2.0.19 on 2026-10-29.
    let report = run(
        "try {\n    d := 0\n    x := 1 / d\n} catch Any as e {\n    MsgBox e is ZeroDivisionError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_property_that_is_not_there_is_a_property_error() {
    let report = run(
        "try {\n    x := 1\n    y := x.Nothing\n} catch Any as e {\n    MsgBox e is PropertyError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_method_that_is_not_there_is_a_method_error() {
    let report = run(
        "try {\n    x := 1\n    x.Nothing()\n} catch Any as e {\n    MsgBox e is MethodError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn an_item_that_has_no_value_is_an_unset_item_error() {
    let report = run(
        "try {\n    m := Map()\n    y := m[\"nope\"]\n} catch Any as e {\n    MsgBox e is UnsetItemError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn an_index_that_is_out_of_range_is_an_index_error() {
    let report = run(
        "try {\n    a := [1]\n    y := a[9]\n} catch Any as e {\n    MsgBox e is IndexError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_value_of_the_wrong_type_is_a_type_error() {
    let report = run("try {\n    x := 1 + \"a\"\n} catch Any as e {\n    MsgBox e is TypeError\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_value_that_is_not_valid_is_a_value_error() {
    let report = run(
        "try {\n    m := Map()\n    m.CaseSense := \"x\"\n} catch Any as e {\n    MsgBox e is ValueError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_arity_of_a_call_is_a_plain_error() {
    let report =
        run("try {\n    f := Abs\n    f.Call()\n} catch Any as e {\n    MsgBox e is Error\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_stack_of_an_error_names_the_frames_of_the_call() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the stack is a
    // line for every frame of the call that failed, and the last line
    // names the section that starts the script. The script of the runner
    // has no file of its own, and the file of every frame is empty.
    let report = run("e := Error(\"the message\")\nMsgBox e.Stack");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![" (1) : [] e := Error(\"the message\")\n> Auto-execute"]
    );
}

#[test]
fn the_own_properties_of_an_error_are_the_ones_that_v2_spells() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the own
    // properties of an error are "Extra", "File", "Line", "Message",
    // "Stack" and "What".
    let report = run(
        "e := Error(\"m\", \"w\", \"x\")\ns := \"\"\nfor p in ObjOwnProps(e)\n    s .= \"[\" p \"]\"\nMsgBox s",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["[Extra][File][Line][Message][Stack][What]"]
    );
}

#[test]
fn the_number_of_the_arguments_of_the_constructor_of_an_error_is_checked() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the errors are made
    // the way the root of them is, and the call goes through the base
    // object.
    let report = run(
        "try {\n    x := Error(\"a\", \"b\", \"c\", \"d\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too many parameters passed to function. | Object.Call | Error.Prototype.__New"]
    );
}

#[test]
fn the_line_of_an_error_is_the_line_where_it_was_made() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the line of an error
    // is the line where it was made, and not the line where it is read.
    let report = run("e := Error(\"m\")\n\n\nMsgBox e.Line");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_line_of_an_error_that_was_thrown_is_the_line_of_the_throw() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the error that is
    // thrown carries the line where it was thrown.
    let report = run("try {\n    throw Error(\"m\")\n} catch as e {\n    MsgBox e.Line\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn an_error_has_no_prototype_of_its_own() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the prototype is a
    // property of the class and not of an instance of it.
    let report = run(
        "try {\n    e := Error(\"m\")\n    x := e.Prototype\n} catch Any as e {\n    MsgBox e.Message\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["This value of type \"Error\" has no property named \"Prototype\"."]
    );
}

#[test]
fn the_prototype_of_the_error_is_the_prototype_of_the_error() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the class of the
    // prototype is the class that it is the prototype of, and the base of
    // it is the prototype of the class that it extends.
    let report = run("MsgBox Error.Prototype.__Class \" | \" Error.Prototype.Base.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Error | Object"]);
}

#[test]
fn the_base_of_the_error_is_the_class_that_it_extends() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the base of a class
    // is the class that it extends.
    let report = run("MsgBox Error.Base.__Class \" | \" Error.HasBase(Object)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Class | 1"]);
}

#[test]
fn the_properties_of_an_error_are_its_own() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the message of an
    // error is a property of it, and it is an own one.
    let report =
        run("e := Error(\"m\")\nMsgBox e.HasProp(\"Message\") \" | \" e.HasOwnProp(\"Message\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 | 1"]);
}

#[test]
fn the_properties_of_an_error_are_written() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the message, the
    // what and the extra of an error are written the way the class says.
    let report = run(
        "e := Error(\"m\")\ne.Message := \"w\"\ne.What := \"n\"\ne.Extra := \"x\"\nMsgBox e.Message \" | \" e.What \" | \" e.Extra",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["w | n | x"]);
}
