//! The classes of the windows, against a fake desktop.
//!
//! A window is a Windows window, and this desktop is a Wayland session
//! without one. So what a script gets here is the object and the reason:
//! the object is made, and every name of it says why it cannot work.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
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

impl Windows for Fake {}

impl Native for Fake {}
impl Com for Fake {}

#[test]
fn a_gui_is_made() {
    // Measured on the binary of v2.0.19: the name of a class
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
    assert!(
        error.contains("Menu cannot add an item to the menu bar"),
        "{error}"
    );
    assert!(error.contains("Wayland session without one"), "{error}");
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_says_why_it_cannot() {
    let report = run("window := Gui()\ntitle := window.Title");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(
        error.contains("Gui cannot read or set the title of the window"),
        "{error}"
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_property_of_a_gui_that_is_written_says_why_it_cannot() {
    let report = run("window := Gui()\nwindow.Title := \"the window\"");
    assert!(!report.ok);
    let error = error_of(&report);
    assert!(
        error.contains("Gui cannot read or set the title of the window"),
        "{error}"
    );
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
    // Measured on the binary of v2.0.19: the third argument
    // of the constructor is the object that receives the events, and v2
    // checks it before the options.
    let report = run(
        "try {\n    x := Gui(0, \"the title\", 5)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "Parameter #4 of Gui.Prototype.__New requires an Object, but received an Integer. | Gui.Prototype.__New | 5"
        ]
    );
}

#[test]
fn the_options_of_a_gui_are_read() {
    // Measured on the binary of v2.0.19: the options are the
    // words of the text, and the name of one is written in full and it does
    // not know the case.
    for options in [
        "AlwaysOnTop",
        "Border",
        "Caption",
        "Disabled",
        "DPIScale",
        "E",
        "LastFound",
        "MaximizeBox",
        "MinimizeBox",
        "MinSize",
        "MaxSize",
        "OwnDialogs",
        "Resize",
        "SysMenu",
        "Theme",
        "ToolWindow",
        "+Resize",
        "-Caption",
        "E0",
        "MinSize100x200",
        "0",
        "12",
        "0x10",
        "caption",
        "+Resize -Caption",
        "+Resize  -Caption",
    ] {
        let report = run(&format!("window := Gui(\"{options}\")"));
        assert!(report.ok, "{options}: {:?}", report.error);
    }
}

#[test]
fn an_option_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19: the word that is
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
    // Measured on the binary of v2.0.31: a bare `Owner` carries no owner and
    // the window is made; an owner or a parent that names a window of the
    // machine is resolved, and the one that is not a window is "Invalid or
    // nonexistent owner or parent window."
    let report = run("window := Gui(\"Owner\")");
    assert!(report.ok, "{:?}", report.error);
    for option in ["Parent", "Owner123"] {
        let report = run(&format!("window := Gui(\"{option}\")"));
        assert!(!report.ok, "{option}");
        let error = error_of(&report);
        assert!(
            error.contains("Invalid or nonexistent owner or parent window."),
            "{error}"
        );
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

// -- the classes of the controls of a window ---------------------------------

/// The classes of the controls of a window.
///
/// Measured on the binary of v2.0.31: the class of a
/// control is a nested class of the class `Gui`, and a script reads each
/// one of them as a class: `Type(Gui.Control)` answers "Class" on the
/// binary of the v2.0.19 and on the binary of the v2.0.31, alike.
///
/// The classes are the twenty-five of them, and the `Gui.Menu` is not one
/// of them: the binaries refuse the name too.
const THE_CLASSES_OF_THE_CONTROLS: [&str; 25] = [
    "Gui.ActiveX",
    "Gui.Button",
    "Gui.CheckBox",
    "Gui.ComboBox",
    "Gui.Control",
    "Gui.Custom",
    "Gui.DDL",
    "Gui.DateTime",
    "Gui.Edit",
    "Gui.GroupBox",
    "Gui.Hotkey",
    "Gui.Link",
    "Gui.List",
    "Gui.ListBox",
    "Gui.ListView",
    "Gui.MonthCal",
    "Gui.Pic",
    "Gui.Progress",
    "Gui.Radio",
    "Gui.Slider",
    "Gui.StatusBar",
    "Gui.Tab",
    "Gui.Text",
    "Gui.TreeView",
    "Gui.UpDown",
];

#[test]
fn the_class_of_a_control_is_a_class() {
    for name in THE_CLASSES_OF_THE_CONTROLS {
        let report = run(&format!("MsgBox Type({name})"));
        assert!(report.ok, "{name}: {:?}", report.error);
        assert_eq!(report.output, vec!["Class"], "{name}");
    }
}

#[test]
fn the_class_of_a_control_is_a_member_of_the_class_of_the_window() {
    for name in THE_CLASSES_OF_THE_CONTROLS {
        let member = name.strip_prefix("Gui.").unwrap_or(name);
        let report = run(&format!("MsgBox Gui.HasProp(\"{member}\")"));
        assert!(report.ok, "{name}: {:?}", report.error);
        assert_eq!(report.output, vec!["1"], "{name}");
    }
}

#[test]
fn the_class_value_of_the_window_owns_the_classes_of_the_controls() {
    let report = run("for p in Gui.OwnProps()\n    MsgBox p");
    assert!(report.ok, "{:?}", report.error);
    for name in THE_CLASSES_OF_THE_CONTROLS {
        let member = name.strip_prefix("Gui.").unwrap_or(name);
        assert!(
            report.output.iter().any(|line| line == member),
            "{member} is not an own property of the class of the window"
        );
    }
}

#[test]
fn the_prototype_of_a_class_of_a_control_owns_its_members() {
    // Measured on the binary of v2.0.31: the class of a
    // control owns nineteen names, and the `Add`, the `Choose` and the
    // `Delete` are not among them -- they are the names of the class of a
    // list of items, the `Gui.List`.
    let report = run("for p in Gui.Control.Prototype.OwnProps()\n    MsgBox p");
    assert!(report.ok, "{:?}", report.error);
    for name in [
        "ClassNN",
        "Enabled",
        "Focus",
        "Focused",
        "GetPos",
        "Gui",
        "Hwnd",
        "Move",
        "Name",
        "OnCommand",
        "OnEvent",
        "OnNotify",
        "Opt",
        "Redraw",
        "SetFont",
        "Text",
        "Type",
        "Value",
        "Visible",
    ] {
        assert!(
            report.output.iter().any(|line| line == name),
            "{name} is not an own property of the prototype"
        );
    }
    for name in ["Add", "Choose", "Delete"] {
        assert!(
            !report.output.iter().any(|line| line == name),
            "{name} is not a member of the class of a control"
        );
    }
}

#[test]
fn a_class_of_a_control_derives_from_its_base() {
    // Measured on the binary of v2.0.31: the `Gui.ListBox`,
    // the `Gui.ComboBox`, the `Gui.DDL` and the `Gui.Tab` derive from the
    // `Gui.List`, and the `Gui.List` and the rest derive from the
    // `Gui.Control`.
    for (name, base) in [
        ("Gui.Control", "Object"),
        ("Gui.List", "Gui.Control"),
        ("Gui.ListBox", "Gui.List"),
        ("Gui.ComboBox", "Gui.List"),
        ("Gui.DDL", "Gui.List"),
        ("Gui.Tab", "Gui.List"),
        ("Gui.ListView", "Gui.Control"),
        ("Gui.TreeView", "Gui.Control"),
        ("Gui.StatusBar", "Gui.Control"),
        ("Gui.DateTime", "Gui.Control"),
        ("Gui.Button", "Gui.Control"),
    ] {
        let report = run(&format!(
            "MsgBox ObjGetBase({name}.Prototype).__Class"
        ));
        assert!(report.ok, "{name}: {:?}", report.error);
        assert_eq!(report.output, vec![base], "{name}");
    }
}

#[test]
fn a_class_of_a_control_inherits_the_members_of_its_base() {
    // Measured on the binary of v2.0.31: the prototype of a
    // class owns the members of it and the ones of its base.
    let report = run(
        "MsgBox Gui.ListBox.Prototype.HasMethod(\"Focus\") \" \" \
         Gui.ListBox.Prototype.HasProp(\"ClassNN\") \" \" \
         Gui.List.Prototype.HasMethod(\"Choose\") \" \" \
         Gui.Control.Prototype.HasMethod(\"Add\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1 1 0"]);
}

#[test]
fn the_call_of_a_class_of_a_control_is_refused() {
    // Measured on the binary of v2.0.31: the call of a
    // class that v2 does not make is refused with "Invalid base.", the way
    // the call of the `File` and of the `Func` is refused.
    let report = run(
        "try {\n    Gui.Control()\n} catch Any as e {\n    MsgBox e.Message \" | \" Type(e)\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid base. | ValueError"]);
}

#[test]
fn a_class_of_a_control_is_not_a_class_that_a_script_makes() {
    // Measured on the binary of v2.0.31: the classes of the
    // controls of a window are classes of the runtime, and a script may
    // name one of them as the base of a class of its own.
    let report = run("class A extends Gui.Control {\n}\nMsgBox Type(A)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Class"]);
}
