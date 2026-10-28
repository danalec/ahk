//! The `Hotkey` class, against a fake desktop.
//!
//! A hotkey is a key the script watches for, and this runtime has no
//! hotkeys yet: the box has a keyboard of its own but nothing that
//! watches it. So what a script gets here is the object and the reason:
//! the object is made, and every name of it says why it cannot work.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers,
    Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that records what it was asked to do.
///
/// The class of the hotkeys asks it nothing: this runtime has no hotkeys
/// yet. The fake is here because `Desktop` is the sum of the nine
/// capabilities, and the runtime wants all of them.
#[derive(Debug, Default)]
struct Fake;

/// Run a script, and give the report back.
fn run(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

/// The class of the object a variable holds.
fn class_of(report: &Report, name: &str) -> String {
    let Some(Value::Object(object)) = report.variables.get(name) else {
        panic!("{name} holds no object");
    };
    object.borrow().class.clone()
}

/// The message the report carries.
fn error_of(report: &Report) -> String {
    report.error.clone().unwrap_or_default()
}

impl Desktop for Fake {}

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
    fn capture(&mut self, _x: i64, _y: i64, _width: i64, _height: i64, _filename: &str) -> Result<String, String> {
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

impl Windows for Fake {}

impl Native for Fake {}

#[test]
fn a_hotkey_is_made() {
    let report = run("hk := Hotkey(\"^!a\", \"TheCallback\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "hk"), "hotkey");
}

#[test]
fn a_hotkey_that_starts_says_why_it_cannot() {
    let report = run("hk := Hotkey(\"^!a\", \"TheCallback\")\nhk.Start()");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Hotkey cannot start watching for the key"), "{error}");
    assert!(error.contains("this runtime has no hotkeys yet"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_hotkey_says_why_it_cannot() {
    let report = run("hk := Hotkey(\"^!a\", \"TheCallback\")\ntimeout := hk.Timeout");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Hotkey cannot read or set the timeout of the hotkey"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_method() {
    let report = run("hk := Hotkey(\"^!a\", \"TheCallback\")\nhk.Nonsense()");
    assert!(!report.ok);
    assert_eq!(error_of(&report), "\"Nonsense\" is not a method of this value.");
    assert_eq!(report.error_line, Some(2));
}
