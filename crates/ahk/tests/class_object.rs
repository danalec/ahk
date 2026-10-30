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
fn the_static_methods_of_a_class_are_handed_over_as_values() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every class has
    // the methods of the base object as static ones, and the call of the
    // class is one of them.
    assert_eq!(the_answer_of("c := Map\nf := c.GetMethod(\"HasProp\")\nMsgBox f.Name"), "HasProp");
    assert_eq!(
        the_answer_of("c := Map\nf := c.GetMethod(\"OwnProps\")\nMsgBox f.Name"),
        "Object.Prototype.OwnProps"
    );
    assert_eq!(
        the_answer_of("c := Map\nf := c.GetMethod(\"Call\")\nMsgBox f.Name"),
        "Map.Call"
    );
    // The object is one of the arguments of the call.
    assert_eq!(
        the_answer_of("c := Map\nf := c.GetMethod(\"HasProp\")\nMsgBox f.MinParams \" \" f.MaxParams"),
        "2 2"
    );
    assert_eq!(the_answer_of("c := Map\nf := c.GetMethod(\"HasProp\")\nMsgBox f(Map(), \"x\")"), "0");
    assert_eq!(
        the_answer_of("c := Map\nf := c.GetMethod(\"Call\")\nMsgBox f(c).__Class"),
        "Map"
    );
    assert_eq!(
        the_answer_of("c := Map\nf := c.GetMethod(\"Clone\")\nMsgBox f(Object()).__Class"),
        "Object"
    );
}

#[test]
fn the_method_a_class_does_not_have_is_refused_with_the_words_of_the_dialect() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    assert_eq!(
        the_failure_of("c := Map\nf := c.GetMethod(\"Get\")"),
        "This value of type \"Class\" has no method named \"Get\". | GetMethod |"
    );
}

#[test]
fn the_copy_of_the_base_object_refuses_a_class_that_has_a_copy_of_its_own() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the type of the
    // argument is not the one that is wanted.
    assert_eq!(
        the_failure_of("c := Map\nf := c.GetMethod(\"Clone\")\nf(Map())"),
        "Type mismatch. | Object.Prototype.Clone |"
    );
}

#[test]
fn a_class_value_has_no_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class value has
    // no property named `Name`, and the read of it is refused with the
    // words of the dialect.
    assert_eq!(
        the_failure_of("c := Map\nx := c.Name"),
        "This value of type \"Class\" has no property named \"Name\". |  |"
    );
    assert_eq!(
        the_failure_of("class C {\n}\nx := C.Name"),
        "This value of type \"Class\" has no property named \"Name\". |  |"
    );
}

#[test]
fn a_name_that_a_script_writes_is_a_property_of_the_class_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the write makes an
    // own property of the class value, and the class keeps its identity.
    assert_eq!(the_answer_of("c := Map\nc.Name := \"x\"\nMsgBox c.Name"), "x");
    assert_eq!(
        the_answer_of("c := Map\nc.Name := \"x\"\nMsgBox c.HasOwnProp(\"Name\")"),
        "1"
    );
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"Name\")"), "0");
    assert_eq!(the_answer_of("c := Map\nc.Name := \"x\"\nMsgBox c.__Class"), "Class");
    assert_eq!(the_answer_of("c := Map\nc.Name := \"x\"\nMsgBox c.Base.__Class"), "Class");
    assert_eq!(the_answer_of("c := Map\nc.Name := \"x\"\nMsgBox c.Prototype.__Class"), "Map");
}

#[test]
fn a_prototype_has_no_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    // class has no name either, and the write of one is a property of the
    // prototype.
    assert_eq!(
        the_failure_of("c := Map\nx := c.Prototype.Name"),
        "This value of type \"Prototype\" has no property named \"Name\". |  |"
    );
    assert_eq!(
        the_answer_of("c := Map\nc.Prototype.Name := \"x\"\nMsgBox c.Prototype.Name"),
        "x"
    );
    assert_eq!(
        the_answer_of("c := Map\nc.Prototype.Name := \"x\"\nMsgBox c.Prototype.__Class"),
        "Map"
    );
}

#[test]
fn the_base_of_a_class_is_written_the_way_objsetbase_writes_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value has to be
    // an object, and the base of the class becomes it.
    assert_eq!(
        the_failure_of("c := Map\nc.Base := 1"),
        "Expected an Object but got an Integer. | ObjSetBase | 1"
    );
    assert_eq!(
        the_failure_of("class C {\n}\nC.Base := 1"),
        "Expected an Object but got an Integer. | ObjSetBase | 1"
    );
    assert_eq!(
        the_answer_of("c := Map\nc.Base := Object()\nMsgBox c.Base.__Class"),
        "Object"
    );
    assert_eq!(
        the_answer_of("c := Map\nc.Base := Object()\nMsgBox c.HasBase(Object)"),
        "0"
    );
}

#[test]
fn the_prototype_of_a_class_is_written_by_a_script() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    // class is a property that a script may write.
    assert_eq!(the_answer_of("c := Map\nc.Prototype := 1\nMsgBox c.Prototype"), "1");
    assert_eq!(
        the_answer_of("class C {\n}\nC.Prototype := 1\nMsgBox C.Prototype"),
        "1"
    );
}

#[test]
fn the_static_methods_of_a_class_are_read_only() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static methods
    // of a class are read-only, and the name that is written is in the
    // `Extra` of the failure.
    assert_eq!(
        the_failure_of("c := Map\nc.HasProp := 1"),
        "Property is read-only. |  | HasProp"
    );
}

#[test]
fn the_properties_of_a_class_do_not_answer_a_class_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class value is not
    // an instance of its class, and the properties of the class do not
    // answer it.
    assert_eq!(
        the_failure_of("c := Map\nx := c.Count"),
        "This value of type \"Class\" has no property named \"Count\". |  |"
    );
    assert_eq!(
        the_failure_of("c := Map\nx := c.Clear"),
        "This value of type \"Class\" has no property named \"Clear\". |  |"
    );
}

#[test]
fn the_base_of_a_class_of_the_script_is_the_class_that_it_extends() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the base of a class
    // of the script is the class that it extends, and the chain of the
    // bases of a class is the chain of the classes it extends.
    assert_eq!(
        the_answer_of("class D {\n}\nclass C extends D {\n}\nx := (C.Base == D)\nMsgBox x"),
        "1"
    );
    assert_eq!(the_answer_of("class C {\n}\nMsgBox C.Base.__Class"), "Class");
    assert_eq!(the_answer_of("class D {\n}\nclass C extends D {\n}\nMsgBox C.HasBase(D)"), "1");
    assert_eq!(the_answer_of("class C {\n}\nMsgBox C.HasBase(Object)"), "1");
    assert_eq!(
        the_answer_of("class D {\n}\nclass C extends D {\n}\nC.Base := Object()\nMsgBox C.HasBase(D)"),
        "0"
    );
}

#[test]
fn two_objects_are_the_same_object_and_nothing_else() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the objects are
    // compared by their identity, and an object is never equal to a value
    // that is not an object.
    assert_eq!(the_answer_of("m1 := Map()\nm2 := Map()\nMsgBox m1 == m2"), "0");
    assert_eq!(the_answer_of("m1 := Map()\nMsgBox m1 == m1"), "1");
    assert_eq!(the_answer_of("m := Map()\nMsgBox m == Map"), "0");
    assert_eq!(the_answer_of("c := Map\nMsgBox c == Map"), "1");
    assert_eq!(the_answer_of("m := Map()\nMsgBox m == \"map\""), "0");
    assert_eq!(the_answer_of("m1 := Map()\nm2 := Map()\nMsgBox m1 != m2"), "1");
}

#[test]
fn the_static_methods_of_a_class_value_are_handed_over_as_values() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static methods
    // of a class are its own properties, and v2 hands them over as
    // functions.
    assert_eq!(the_answer_of("c := Map\nx := c.Call\nMsgBox x.Name"), "Map.Call");
    assert_eq!(the_answer_of("c := Map\nx := c.HasProp\nMsgBox x.Name"), "HasProp");
    assert_eq!(
        the_answer_of("class C {\n    static Say() {\n    }\n}\nx := C.Say\nMsgBox x.Name"),
        "C.Say"
    );
    // The methods of the instances are not the methods of the class.
    assert_eq!(
        the_failure_of("c := Map\nx := c.Clear"),
        "This value of type \"Class\" has no property named \"Clear\". |  |"
    );
}

#[test]
fn a_class_value_has_the_static_methods_and_not_the_others() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the methods of the
    // instances of a class do not answer a class value, and the static
    // methods of a class of the script are its own.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasMethod(\"Clear\")"), "0");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasMethod(\"HasProp\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasMethod(\"Call\")"), "1");
    assert_eq!(
        the_answer_of("class C {\n    static Say() {\n    }\n}\nMsgBox C.HasMethod(\"Say\")"),
        "1"
    );
    assert_eq!(
        the_answer_of("class C {\n    Say() {\n    }\n}\nMsgBox C.HasMethod(\"Say\")"),
        "0"
    );
    // The static methods are properties of the class as well.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasProp(\"Call\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasProp(\"HasProp\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasProp(\"Base\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasProp(\"Clear\")"), "0");
}

#[test]
fn the_own_properties_of_a_class_value_are_the_call_and_the_prototype() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the call of a class
    // is its own property, and the methods of the base object are
    // inherited and not own.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"Call\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"HasProp\")"), "0");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"Prototype\")"), "1");
    assert_eq!(
        the_answer_of("class C {\n    static Say() {\n    }\n}\nMsgBox C.HasOwnProp(\"Say\")"),
        "1"
    );
}

#[test]
fn the_write_of_a_static_method_of_a_class_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static methods
    // of a class are read-only.
    assert_eq!(
        the_failure_of("class C {\n    static Say() {\n    }\n}\nC.Say := 1"),
        "Property is read-only. |  | Say"
    );
    // The methods of the instances are not the methods of the class, and a
    // script may write a property with that name.
    assert_eq!(
        the_answer_of("class C {\n    Say() {\n    }\n}\nC.Say := 1\nMsgBox C.Say"),
        "1"
    );
}

#[test]
fn the_methods_of_the_instances_do_not_answer_a_class_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class value is
    // not an instance of its class.
    assert_eq!(
        the_failure_of("c := Map\nc.Get(\"a\")"),
        "This value of type \"Class\" has no method named \"Get\". |  |"
    );
    assert_eq!(
        the_failure_of("c := Map\nc.Clear()"),
        "This value of type \"Class\" has no method named \"Clear\". |  |"
    );
    assert_eq!(
        the_failure_of("class C {\n    Say() {\n    }\n}\nC.Say()"),
        "This value of type \"Class\" has no method named \"Say\". |  |"
    );
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
