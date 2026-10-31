//! The `Enumerator` class, against a fake desktop.
//!
//! An enumerator never touches the machine: the desktop here is a fake that
//! answers nothing, because nothing is asked of it.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, Windows,
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
fn the_enumeration_of_an_array_is_an_enumerator() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class of the
    // enumeration of v2 is `Enumerator`.
    let report = run("a := Array(1)\nMsgBox Type(a.__Enum())");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Enumerator"]);
}

#[test]
fn the_enumeration_of_a_map_is_an_enumerator() {
    let report = run("m := Map()\nMsgBox Type(m.__Enum())");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Enumerator"]);
}

#[test]
fn the_own_properties_are_an_enumerator() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the own properties
    // of v2 are an enumerator too.
    let report = run("o := {a: 1}\nMsgBox Type(o.OwnProps()) . \" \" . Type(ObjOwnProps(o))");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Enumerator Enumerator"]);
}

#[test]
fn the_call_of_an_enumerator_hands_out_the_next_value() {
    let report = run(
        "a := Array(\"x\", \"y\")\ne := a.__Enum()\n\
         first := \"\"\nok := e.Call(&first)\n\
         second := \"\"\ne.Call(&second)\n\
         third := \"\"\nlast := e.Call(&third)\n\
         MsgBox first . second . ok . last",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["xy10"]);
}

#[test]
fn the_call_of_an_enumerator_hands_out_the_key_and_the_value() {
    let report = run(
        "m := Map()\nm[\"k\"] := 7\ne := m.__Enum()\n\
         key := \"\"\nvalue := \"\"\ne.Call(&key, &value)\n\
         MsgBox key . \"/\" . value",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["k/7"]);
}

#[test]
fn the_call_of_an_enumerator_with_one_variable_gives_the_key_of_a_map() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the enumerator of
    // a map gives the key, and the enumerator of an array gives the value.
    let report = run(
        "m := Map()\nm[\"k\"] := 7\ne := m.__Enum()\n\
         key := \"\"\ne.Call(&key)\n\
         a := Array(\"v\")\ne2 := a.__Enum()\n\
         value := \"\"\ne2.Call(&value)\n\
         MsgBox key . \"/\" . value",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["k/v"]);
}

#[test]
fn the_own_properties_give_the_name_and_the_value() {
    let report = run(
        "o := {a: 1}\ne := o.OwnProps()\n\
         name := \"\"\nvalue := \"\"\ne.Call(&name, &value)\n\
         MsgBox name . \"=\" . value",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a=1"]);
}

#[test]
fn the_enumerator_carries_the_values_of_a_function() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the enumerator is
    // a function of its own kind, and it carries the values of a function.
    let report = run(
        "e := Array(1).__Enum()\n\
         MsgBox \"[\" e.Name \"]\" . \" \" . e.MinParams . \" \" . e.MaxParams . \" \" . e.IsBuiltin . \" \" . e.IsVariadic",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[] 0 0 1 0"]);
}

#[test]
fn the_enumerator_has_no_own_property() {
    let report = run("e := Array(1).__Enum()\nMsgBox ObjOwnPropCount(e) . \" \" . e.HasOwnProp(\"Call\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0 0"]);
}

#[test]
fn the_enumerator_is_not_made_by_a_call() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the enumerator is
    // made out of the `__Enum` of a value, and not by a call.
    let report = run("MsgBox Enumerator()");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid base."));
}

#[test]
fn the_enumerator_is_not_indexed() {
    let report = run("MsgBox Array(1).__Enum()[1]");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Enumerator\" has no property named \"__Item\".")
    );
}

#[test]
fn the_enumerator_has_no_enumeration_of_its_own() {
    let report = run("MsgBox Array(1).__Enum().__Enum()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Enumerator\" has no method named \"__Enum\".")
    );
}

#[test]
fn a_loop_walks_an_enumerator() {
    let report = run(
        "e := Array(1, 2).__Enum()\ns := \"\"\n\
         for v in e\n    s .= v \";\"\n\
         MsgBox s",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1;2;"]);
}

#[test]
fn the_name_of_an_enumerator_may_not_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of an
    // enumerator is a property that may not be written, and the write of it
    // is refused with "Property is read-only.", the name of the property is
    // in the Extra, and the What is empty.
    let report = run(
        "try {\n    e := [1, 2].__Enum()\n    e.Name := \"x\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Property is read-only. |  | Name"]);
}
