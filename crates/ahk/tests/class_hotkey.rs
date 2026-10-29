//! The `Hotkey` class, against a fake desktop.
//!
//! A hotkey is a key the script watches for, and the runtime carries
//! them: the object is made, the desktop is told to watch the key, and
//! the options of the object are read back.
//!
//! The action of a hotkey is a function of the script, and the binary of
//! v2.0.19 refuses a text there: the callback has to be the object of
//! the function.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Machine, Native, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    /// The hotkeys that were registered.
    hotkeys: Vec<String>,
}

/// Run a script, and give the report and the desktop back.
fn run(source: &str) -> (Report, Fake) {
    let mut desktop = Fake::default();
    let statements = parse(source).expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    (report, desktop)
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
impl Sound for Fake {}
impl Machine for Fake {}

impl Files for Fake {}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {
    fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
        self.hotkeys.push(name.to_string());
        Ok(format!("{name} is watched"))
    }

    fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {
        self.hotkeys.retain(|held| held != name);
        Ok(format!("{name} is not watched"))
    }

    fn hotkeys(&mut self) -> Result<Vec<String>, String> {
        Ok(self.hotkeys.clone())
    }

    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
        Ok(None)
    }
}

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
fn a_hotkey_is_made_and_the_desktop_is_told_to_watch_the_key() {
    let (report, desktop) = run("Callback() {
    return 0
}\nhk := Hotkey(\"^!a\", Callback)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "hk"), "hotkey");
    assert_eq!(desktop.hotkeys, vec!["^!a".to_string()]);
}

#[test]
fn an_action_that_is_not_a_function_is_refused_the_way_v2_refuses_it() {
    let (report, _) = run("hk := Hotkey(\"^!a\", \"TheCallback\")");
    assert!(!report.ok);
    assert_eq!(error_of(&report), "Parameter #2 of Hotkey is invalid.");
}

#[test]
fn a_hotkey_that_starts_is_watched() {
    let (report, desktop) = run("Callback() {
    return 0
}\nhk := Hotkey(\"^!a\", Callback)\nhk.Start()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.hotkeys, vec!["^!a".to_string()]);
}

#[test]
fn a_property_of_a_hotkey_is_read_back() {
    let (report, _) = run("Callback() {
    return 0
}\nhk := Hotkey(\"^!a\", Callback)\ntimeout := hk.Timeout");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("timeout"), Some(&Value::Int(0)));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_method() {
    let (report, _) = run("Callback() {
    return 0
}\nhk := Hotkey(\"^!a\", Callback)\nhk.Nonsense()");
    assert!(!report.ok);
    assert_eq!(error_of(&report), "\"Nonsense\" is not a method of this value.");
    // The function of the script takes three lines, and the call is the
    // fifth.
    assert_eq!(report.error_line, Some(5));
}
