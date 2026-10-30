//! The base object, and the class value that is the class of the dialect.
//!
//! Measured on the binary of v2.0.19 on 2026-10-31: the base of an object
//! is the prototype of its class, the base of a class is the class that it
//! extends, the copy of a class is a class of its own, and the call of a
//! class value makes an object of the class.

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

/// The value that a script leaves behind.
fn the_answer_of(source: &str) -> String {
    let report = run(source);
    assert!(report.ok, "{:?}", report.error);
    report.output.first().cloned().unwrap_or_default()
}

/// The failure of a script: the message, the `What` and the `Extra`.
fn the_failure_of(source: &str) -> String {
    let report = run(&format!(
        "try {{\n    {source}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}\n"
    ));
    assert!(report.ok, "{:?}", report.error);
    report.output.first().cloned().unwrap_or_default().trim().to_string()
}

#[test]
fn the_base_of_an_object_is_the_prototype_of_its_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.Base.__Class"), "Object");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.Prototype.Base.__Class"), "Object");
}

#[test]
fn the_base_of_a_class_is_the_class_that_it_extends() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the base of the
    // `Map` is the `Object`, and the base of the `Object` is the `Any`.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.Base.__Class"), "Class");
    assert_eq!(the_answer_of("c := Object\nMsgBox c.Base.__Class"), "Class");
}

#[test]
fn the_copy_of_an_object_carries_the_state_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    assert_eq!(the_answer_of("o := Object()\no.x := 1\nMsgBox o.Clone().x"), "1");
}

#[test]
fn the_copy_of_a_class_is_a_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the copy of a
    // class is a class, and not an object of it.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.Clone().__Class"), "Class");
}

#[test]
fn the_call_of_a_class_value_makes_an_object() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `c.Call()` makes
    // an object of the class, the way `c()` does.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.Call().__Class"), "Map");
    assert_eq!(the_answer_of("c := Error\nMsgBox c.Call().__Class"), "Error");
}

#[test]
fn the_failure_of_a_method_of_the_base_object_names_the_method() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the failure of a
    // method of the base object carries the name of the method in the
    // `What`.
    assert_eq!(
        the_failure_of("o := Object()\no.GetOwnPropDesc(\"x\")"),
        "This value of type \"Object\" has no property named \"x\". | Object.Prototype.GetOwnPropDesc |"
    );
    assert_eq!(
        the_failure_of("o := Object()\no.DefineProp(\"x\", 1)"),
        "Parameter #2 of Object.Prototype.DefineProp is invalid. | Object.Prototype.DefineProp | 1"
    );
}
