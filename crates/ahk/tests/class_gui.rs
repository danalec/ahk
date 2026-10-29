//! The classes of the windows, against a fake desktop.
//!
//! A window is a Windows window, and this desktop is a Wayland session
//! without one. So what a script gets here is the object and the reason:
//! the object is made, and every name of it says why it cannot work.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen,
    Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that records what it was asked to do.
///
/// The classes of the windows ask it nothing: a window is a Windows
/// window, and this desktop is a Wayland session without one. The fake is
/// here because `Desktop` is the sum of the eleven capabilities, and the
/// runtime wants all of them.
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
impl Sound for Fake {}

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
fn a_gui_is_made() {
    let report = run("gui := Gui()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "gui"), "gui");
}

#[test]
fn a_gui_that_shows_itself_says_why_it_cannot() {
    let report = run("gui := Gui()\ngui.Show()");
    assert!(!report.ok);
    assert_eq!(
        error_of(&report),
        "Gui cannot show the window: a window is a Windows window, and this desktop is a Wayland session without one."
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_menu_is_made() {
    let report = run("menu := Menu()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "menu"), "menu");
}

#[test]
fn a_menu_that_adds_an_item_says_why_it_cannot() {
    let report = run("menu := Menu()\nmenu.Add(\"File\")");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Menu cannot add an item to the menu bar"), "{error}");
    assert!(error.contains("Wayland session without one"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_says_why_it_cannot() {
    let report = run("gui := Gui()\ntitle := gui.Title");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Gui cannot read or set the title of the window"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_that_is_written_says_why_it_cannot() {
    let report = run("gui := Gui()\ngui.Title := \"the window\"");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Gui cannot read or set the title of the window"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_method() {
    let report = run("gui := Gui()\ngui.Focus()");
    assert!(!report.ok);
    assert_eq!(error_of(&report), "\"Focus\" is not a method of this value.");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_property() {
    let report = run("gui := Gui()\nvalue := gui.Nonsense");
    assert!(!report.ok);
    assert_eq!(error_of(&report), "This object has no property \"nonsense\".");
}
