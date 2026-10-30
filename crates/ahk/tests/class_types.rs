//! The class types of v2: the names that a script reads as a class.
//!
//! Measured on the binary of v2.0.19 on 2026-10-31: a class type of the
//! dialect is a value, and its `__Class` is "Class". The name `Hotkey` is
//! the one that is not: it is a function of the dialect, and the class of
//! a hotkey is the class of the object that the function makes.

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

/// The class of the value of a name.
fn the_class_of(name: &str) -> String {
    let report = run(&format!("x := {name}\nMsgBox x.__Class"));
    assert!(report.ok, "{:?}", report.error);
    report.output.first().cloned().unwrap_or_default()
}

#[test]
fn a_class_type_of_the_dialect_is_a_class() {
    for name in [
        "Any",
        "Array",
        "BoundFunc",
        "Buffer",
        "Class",
        "ClipboardAll",
        "Closure",
        "ComObject",
        "ComValue",
        "Enumerator",
        "Error",
        "File",
        "Func",
        "Gui",
        "InputHook",
        "Map",
        "Menu",
        "Object",
        "RegExMatchInfo",
    ] {
        assert_eq!(the_class_of(name), "Class", "the class of {name}");
    }
}

#[test]
fn the_class_of_the_errors_is_a_class() {
    for name in [
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
    ] {
        assert_eq!(the_class_of(name), "Class", "the class of {name}");
    }
}

#[test]
fn the_name_of_a_hotkey_is_a_function() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `x := Hotkey` is a
    // function of the dialect, and the class of a hotkey is the class of
    // the object that the function makes.
    assert_eq!(the_class_of("Hotkey"), "Func");
}

#[test]
fn a_class_type_is_spelled_the_way_v2_spells_it() {
    for (name, spelled) in [
        ("BoundFunc", "BoundFunc"),
        ("ClipboardAll", "ClipboardAll"),
        ("Closure", "Closure"),
        ("ComObject", "ComObject"),
        ("ComValue", "ComValue"),
        ("InputHook", "InputHook"),
        ("Map", "Map"),
        ("RegExMatchInfo", "RegExMatchInfo"),
        ("TypeError", "TypeError"),
    ] {
        // Measured on the binary of v2.0.19 on 2026-10-31: a class value
        // has no name of its own, and the spelling of the class is seen
        // through its prototype.
        let report = run(&format!("x := {name}\nMsgBox x.Prototype.__Class"));
        assert!(report.ok, "{:?}", report.error);
        assert_eq!(report.output.first().cloned().unwrap_or_default(), spelled);
    }
}

/// The failure of a call: the message, the `What` and the `Extra`.
///
/// The three of them are the three of v2, and the script reads them
/// through a catch: the failure of the dialect is not a value.
fn the_failure_of(source: &str) -> String {
    let report = run(&format!(
        "try {{\n    {source}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}\n"
    ));
    assert!(report.ok, "{:?}", report.error);
    report.output.first().cloned().unwrap_or_default().trim().to_string()
}

#[test]
fn the_call_of_a_class_that_v2_does_not_make_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `BoundFunc`,
    // the `Closure` and the `RegExMatchInfo` are classes of the dialect
    // that the runtime makes, and not a script: the call of them is
    // refused with "Invalid base.", the way the call of the `File` is.
    for name in ["BoundFunc", "Closure", "RegExMatchInfo"] {
        assert_eq!(
            the_failure_of(&format!("{name}()")),
            "Invalid base. | Object.Call |",
            "the call of {name}"
        );
    }
}

#[test]
fn the_calls_of_the_classes_are_the_calls_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    for (call, answer) in [
        // The `Any` is the type of any value, and it is not a class that
        // can be made: the call of it goes through the `Call` method of
        // the class, and a class has none.
        ("Any()", "This value of type \"Class\" has no method named \"Call\". |  |"),
        ("Any(1)", "This value of type \"Class\" has no method named \"Call\". |  |"),
        // The number of the arguments of the constructors, and the names
        // that go in the `What` and in the `Extra` of the refusal.
        ("ComObject()", "Too few parameters passed to function. |  | ComObject.Call"),
        ("ComObject(1, 2, 3)", "Too many parameters passed to function. |  | ComObject.Call"),
        ("ComValue()", "Too few parameters passed to function. |  | ComValue.Call"),
        ("ComValue(1)", "Too few parameters passed to function. |  | ComValue.Call"),
        (
            "ComValue(\"x\", 1)",
            "Parameter #1 of ComValue.Call requires a Number, but received a String. | ComValue.Call | x",
        ),
        (
            "ComValue(1, 2, \"x\")",
            "Parameter #3 of ComValue.Call requires a Number, but received a String. | ComValue.Call | x",
        ),
        ("Class(1)", "Too many parameters passed to function. | Object.Call |"),
        ("Object(1)", "Too many parameters passed to function. | Object.Call |"),
        ("Menu(1)", "Too many parameters passed to function. | Menu.Call |"),
        (
            "ClipboardAll(\"x\")",
            "Parameter #1 of ClipboardAll.Prototype.__New requires a Number, but received a String. | ClipboardAll.Prototype.__New | x",
        ),
        (
            "ClipboardAll(0)",
            "Parameter #1 of ClipboardAll.Prototype.__New is invalid. | ClipboardAll.Prototype.__New | 0",
        ),
        (
            "ClipboardAll(-1)",
            "Parameter #2 of ClipboardAll.Prototype.__New must not be omitted in this case. | ClipboardAll.Prototype.__New |",
        ),
        (
            "ClipboardAll(1, 2, 3)",
            "Too many parameters passed to function. | ClipboardAll.Call | ClipboardAll.Prototype.__New",
        ),
        (
            "InputHook(1, 2, 3, 4)",
            "Too many parameters passed to function. | InputHook.Call | InputHook.Prototype.__New",
        ),
        (
            "Buffer(1, 2, 3)",
            "Too many parameters passed to function. | Buffer.Call | Buffer.Prototype.__New",
        ),
        (
            "Error(\"a\", \"b\", \"c\", \"d\")",
            "Too many parameters passed to function. | Object.Call | Error.Prototype.__New",
        ),
        // The classes that v2 does not make: the call goes through the
        // base object, and the base of them has no such thing.
        ("File()", "Invalid base. | Object.Call |"),
        ("Func()", "Invalid base. | Object.Call |"),
        ("Enumerator()", "Invalid base. | Object.Call |"),
    ] {
        assert_eq!(the_failure_of(call), answer, "the call of {call}");
    }
}

#[test]
fn the_own_properties_of_a_function_are_none() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the members of a
    // function -- the name, the number of the parameters and the rest --
    // live on the prototype of the class of it, and the function owns none
    // of them.
    let report = run("f := Abs\ns := \"\"\nfor p in f.OwnProps()\n    s .= p\nMsgBox s");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output.first().cloned().unwrap_or_default(), "");
    let report = run("f := Abs\nMsgBox f.HasOwnProp(\"Name\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output.first().cloned().unwrap_or_default(), "0");
    // The name is read all the same.
    let report = run("f := Abs\nMsgBox f.Name");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output.first().cloned().unwrap_or_default(), "Abs");
    // A method of a class of the script is a function as well.
    let report = run(
        "class C {\n    static Say() {\n    }\n}\nf := C.Say\ns := \"\"\nfor p in f.OwnProps()\n    s .= p\nMsgBox s",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output.first().cloned().unwrap_or_default(), "");
}

#[test]
fn the_call_of_a_class_value_makes_an_object() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class is a value,
    // and the call of it makes an object of that class -- `c := Map; c()`
    // makes a map, the way `Map()` does.
    for (name, class) in [
        ("Array", "Array"),
        ("Buffer", "Buffer"),
        ("Class", "Class"),
        ("ClipboardAll", "ClipboardAll"),
        ("Error", "Error"),
        ("Gui", "Gui"),
        ("InputHook", "InputHook"),
        ("Map", "Map"),
        ("Menu", "Menu"),
        ("Object", "Object"),
    ] {
        let report = run(&format!("c := {name}\nx := c()\nMsgBox x.__Class"));
        assert!(report.ok, "{:?}", report.error);
        assert_eq!(report.output.first().cloned().unwrap_or_default(), class, "the call of {name}");
    }
}

#[test]
fn a_class_that_the_dialect_does_not_make_says_so_when_it_is_called() {
    // The `Any` is not a class that can be made, and the `File`, the
    // `Func` and the `Enumerator` are not made at all.
    assert_eq!(
        the_failure_of("c := Any\nc()"),
        "This value of type \"Class\" has no method named \"Call\". |  |"
    );
    for name in ["File", "Func", "Enumerator"] {
        assert_eq!(the_failure_of(&format!("c := {name}\nc()")), "Invalid base. | Object.Call |", "the call of {name}");
    }
}

#[test]
fn a_class_of_windows_that_this_runtime_does_not_carry_says_so() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `ComObject`
    // and the `ComValue` are classes of the dialect, and the binary makes
    // the COM object of them. This runtime has no COM, and it says so.
    for call in ["ComObject(\"Shell.Application\")", "ComValue(1, 2)"] {
        assert_eq!(
            the_failure_of(call),
            "the COM objects are a thing of Windows, and this runtime does not carry them. |  |",
            "the call of {call}"
        );
    }
}

#[test]
fn a_class_that_this_runtime_does_not_carry_says_why() {
    // The `InputHook` is a keyboard hook, and this runtime has none. The
    // name is there, and the tree does not carry it: the refusal says so,
    // and it does not say that the name is not there.
    for (source, answer) in [
        (
            "ih := InputHook()\nih.Visible",
            "this runtime does not carry that class yet. |  |",
        ),
        (
            "ih := InputHook()\nih.KeyOpt(\"a\")",
            "this runtime does not carry that class yet. |  |",
        ),
        (
            "ca := ClipboardAll()\nca.Size",
            "this runtime does not carry that class yet. |  |",
        ),
    ] {
        assert_eq!(the_failure_of(source), answer, "the member of {source}");
    }
}

#[test]
fn a_class_value_keeps_the_words_of_the_dialect() {
    // A class value is a class of the dialect, and the refusal of a member
    // that is not there keeps the words of the dialect -- the class is
    // there, and the tree carries it.
    for source in ["a := Any\nx := a.Nope", "o := Object\nx := o.Nope"] {
        assert_eq!(
            the_failure_of(source),
            "This value of type \"Class\" has no property named \"Nope\". |  |",
            "the member of {source}"
        );
    }
}

#[test]
fn the_clipboard_all_is_made_with_nothing() {
    // The tree carries no binary clipboard data, and the call of the
    // class with no arguments makes the object.
    let report = run("x := ClipboardAll()\nMsgBox x.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output.first().cloned().unwrap_or_default(), "ClipboardAll");
}
