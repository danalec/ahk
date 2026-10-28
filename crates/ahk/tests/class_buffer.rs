//! The `Buffer` class, against a fake desktop.
//!
//! A buffer is the one class that does not touch the machine: the
//! desktop is here because `Desktop` is the sum of the capabilities,
//! and the runtime drives one.
//!
//! v2.0.19 gives a buffer a size, an address and a constructor, and
//! nothing else. The reads and the writes of bytes belong to the file,
//! which reads into a buffer and writes out of one. Measured on the
//! binary of v2.0.19 on 2026-10-27, with `HasProp` and `HasMethod`.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Pointer, Processes, Screen, Timers, Windows,
};
use ahk::runtime::{Report, Runtime, Value};
use ahk::parse;
use std::time::Duration;

/// A desktop that answers nothing: a buffer does not ask it anything.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}

impl Files for Fake {}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Windows for Fake {}

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

impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

/// Run a script and answer the report of it.
fn run(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

#[test]
fn a_buffer_is_as_big_as_it_was_asked_for() {
    let report = run("b := Buffer(4)\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_buffer_has_an_address() {
    let report = run("b := Buffer(4)\naddress := b.Ptr");
    assert!(report.ok, "{:?}", report.error);
    match report.variables.get("address") {
        Some(Value::Int(number)) => assert!(*number > 0, "the address is {number}"),
        other => panic!("the address is {other:?}"),
    }
}

#[test]
fn a_buffer_may_be_made_again_by_its_constructor() {
    let report = run("b := Buffer(2)\nb.__New(3)\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_buffer_of_a_negative_size_fails() {
    let report = run("b := Buffer(-1)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the size of a buffer cannot be negative."));
}

#[test]
fn a_buffer_may_be_made_of_another_size() {
    let report = run("b := Buffer(2)\nb.Size := 4\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_buffer_has_no_position() {
    // v2.0.19 does not give a buffer one: the position was never there.
    let report = run("b := Buffer(4)\nposition := b.Pos");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This object has no property \"pos\".")
    );
}

#[test]
fn a_buffer_does_not_read_bytes() {
    // The reads and the writes belong to the file, which reads into a
    // buffer and writes out of one.
    let report = run("b := Buffer(4)\nb.ReadUChar()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("\"ReadUChar\" is not a method of this value.")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_buffer_has_no_item() {
    let report = run("b := Buffer(2)\nbyte := b[1]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid index."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_buffer_has_the_methods_of_the_base_object() {
    let report = run(
        "b := Buffer(4)\nmethod := b.HasMethod(\"ReadUChar\")\nprop := b.HasProp(\"Size\")\nbase := b.HasBase(\"Object\")\nMsgBox \"%method% %prop% %base%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // The buffer has no method of its own but the constructor, so the
    // read is not one of them.
    assert_eq!(report.output, vec!["0 1 1"]);
}

#[test]
fn the_inside_of_a_buffer_is_not_a_property_of_it() {
    let report = run(
        "b := Buffer(4)\nprops := b.OwnProps()\ninside := b.HasOwnProp(\"buffer state\")\nMsgBox \"%props% %inside%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![" 0"]);
}

#[test]
fn a_buffer_may_have_a_property_of_its_own() {
    let report = run(
        "b := Buffer(4)\nb.DefineProp(\"x\", 1)\nvalue := b.x\ngone := b.DeleteProp(\"x\")\nMsgBox \"%value% %gone%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1"]);
}
