//! The classes of the windows, against a fake desktop.
//!
//! A window is a Windows window, and this desktop is a Wayland session
//! without one. So what a script gets here is the object and the reason:
//! the object is made, and every name of it says why it cannot work.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that records what it was asked to do.
///
/// The classes of the windows ask it nothing: a window is a Windows
/// window, and this desktop is a Wayland session without one. The fake is
/// here because `Desktop` is the sum of the twelve capabilities, and the
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
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a class
    // is a name of the script, and the variable may not have it: the case
    // of the name does not count.
    let report = run("window := Gui()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "window"), "gui");
}

#[test]
fn a_gui_that_shows_itself_says_why_it_cannot() {
    let report = run("window := Gui()\nwindow.Show()");
    assert!(!report.ok);
    assert_eq!(
        error_of(&report),
        "Gui cannot show the window: a window is a Windows window, and this desktop is a Wayland session without one."
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_menu_is_made() {
    let report = run("bar := Menu()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(class_of(&report, "bar"), "menu");
}

#[test]
fn a_menu_that_adds_an_item_says_why_it_cannot() {
    let report = run("bar := Menu()\nbar.Add(\"File\")");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Menu cannot add an item to the menu bar"), "{error}");
    assert!(error.contains("Wayland session without one"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_says_why_it_cannot() {
    let report = run("window := Gui()\ntitle := window.Title");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Gui cannot read or set the title of the window"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_that_is_written_says_why_it_cannot() {
    let report = run("window := Gui()\nwindow.Title := \"the window\"");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(error.contains("Gui cannot read or set the title of the window"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_method() {
    let report = run("window := Gui()\nwindow.Focus()");
    assert!(!report.ok);
    assert_eq!(
        error_of(&report),
        "This value of type \"Gui\" has no method named \"Focus\"."
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_name_that_is_not_in_the_table_is_not_a_property() {
    let report = run("window := Gui()\nvalue := window.Nonsense");
    assert!(!report.ok);
    assert_eq!(
        error_of(&report),
        "This value of type \"Gui\" has no property named \"Nonsense\"."
    );
}

#[test]
fn the_events_of_a_gui_are_an_object() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the third argument
    // of the constructor is the object that receives the events, and v2
    // checks it before the options.
    let report = run(
        "try {\n    x := Gui(0, \"the title\", 5)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #4 of Gui.Prototype.__New requires an Object, but received an Integer. | Gui.Prototype.__New | 5"]
    );
}

#[test]
fn the_options_of_a_gui_are_read() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the options are the
    // words of the text, and the name of one is written in full and it does
    // not know the case.
    for options in [
        "AlwaysOnTop", "Border", "Caption", "Disabled", "DPIScale", "E",
        "LastFound", "MaximizeBox", "MinimizeBox", "MinSize", "MaxSize",
        "OwnDialogs", "Resize", "SysMenu", "Theme", "ToolWindow", "+Resize",
        "-Caption", "E0", "MinSize100x200", "0", "12", "0x10", "caption",
        "+Resize -Caption", "+Resize  -Caption",
    ] {
        let report = run(&format!("window := Gui(\"{options}\")"));
        assert!(report.ok, "{options}: {:?}", report.error);
    }
}

#[test]
fn an_option_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the word that is
    // refused is the word without its sign.
    for (options, name) in [
        ("Nonsense", "Nonsense"),
        ("+Nonsense", "Nonsense"),
        ("-Nonsense", "Nonsense"),
        ("W", "W"),
        ("Always", "Always"),
        ("Ex", "Ex"),
        ("12x34", "12x34"),
    ] {
        let report = run(&format!(
            "try {{\n    window := Gui(\"{options}\")\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}"
        ));
        assert!(report.ok, "{options}: {:?}", report.error);
        assert_eq!(
            report.output,
            vec![format!("Invalid option. | Gui.Prototype.__New | {name}")],
            "{options}"
        );
    }
}

#[test]
fn the_owner_of_a_window_says_why_it_cannot() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the owner of a
    // window and the parent of it are windows, and this desktop has none.
    for option in ["Owner", "Parent"] {
        let report = run(&format!("window := Gui(\"{option}\")"));
        assert!(!report.ok, "{option}");
        let error = error_of(&report);
        assert!(
            error.contains(&format!("Gui cannot take the {} of the window", option.to_lowercase())),
            "{error}"
        );
        assert!(error.contains("Wayland session without one"), "{error}");
    }
}

#[test]
fn the_options_of_a_gui_are_a_text() {
    let report = run(
        "try {\n    x := Gui(1.5)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Invalid option. | Gui.Prototype.__New | 1.5"]
    );
}
