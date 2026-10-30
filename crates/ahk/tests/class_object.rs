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
fn every_class_has_an_initialiser_and_it_is_a_member_of_the_class_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the initialiser of a
    // class is a member of the class value, and the name of it is empty for
    // a class of the runtime and "C.__Init" for a class of the script.
    assert_eq!(the_answer_of("x := Map\ny := x.__Init\nMsgBox y.Name"), "");
    assert_eq!(the_answer_of("x := Map\ny := x.__Init\nMsgBox y.MinParams"), "1");
    assert_eq!(the_answer_of("x := Map\ny := x.__Init\nMsgBox y.MaxParams"), "1");
    assert_eq!(the_answer_of("x := Map\ny := x.__Init\nMsgBox y.IsBuiltin"), "1");
    assert_eq!(
        the_answer_of("class C {\n}\ny := C.__Init\nMsgBox y.Name"),
        "C.__Init"
    );
    assert_eq!(
        the_answer_of("class C {\n}\ny := C.__Init\nMsgBox y.IsBuiltin"),
        "0"
    );
}

#[test]
fn the_own_properties_of_a_class_value_are_the_ones_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class value of a
    // class of the runtime owns the call of the class and the prototype, and
    // the class value of a class of the script owns the initialiser of the
    // class, the prototype and the static methods that the class declares.
    assert_eq!(
        the_answer_of("c := Map\ns := \"\"\nfor p in c.OwnProps()\n    s .= p\nMsgBox s"),
        "CallPrototype"
    );
    assert_eq!(
        the_answer_of(
            "class C {\n    static Say() {\n    }\n}\ns := \"\"\nfor p in C.OwnProps()\n    s .= p\nMsgBox s"
        ),
        "__InitPrototypeSay"
    );
    assert_eq!(
        the_answer_of("class C {\n}\ns := \"\"\nfor p in C.OwnProps()\n    s .= p\nMsgBox s"),
        "__InitPrototype"
    );
    // The methods of the instances are not the ones of the class.
    assert_eq!(
        the_answer_of(
            "class C {\n    Say() {\n    }\n}\ns := \"\"\nfor p in C.OwnProps()\n    s .= p\nMsgBox s"
        ),
        "__InitPrototype"
    );
}

#[test]
fn the_methods_of_the_base_object_are_inherited_and_not_own() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the call of a class
    // of the script is not own, and the initialiser of a class of the
    // runtime is not own either.
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"Call\")"), "1");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"__Init\")"), "0");
    assert_eq!(the_answer_of("c := Map\nMsgBox c.HasOwnProp(\"Prototype\")"), "1");
    assert_eq!(the_answer_of("class C {\n}\nMsgBox C.HasOwnProp(\"Call\")"), "0");
    assert_eq!(the_answer_of("class C {\n}\nMsgBox C.HasOwnProp(\"__Init\")"), "1");
    assert_eq!(
        the_answer_of("class C {\n    Say() {\n    }\n}\nMsgBox C.HasOwnProp(\"Say\")"),
        "0"
    );
    assert_eq!(
        the_answer_of(
            "class C {\n    static Say() {\n    }\n}\nMsgBox C.HasOwnProp(\"Say\")"
        ),
        "1"
    );
}

#[test]
fn the_properties_of_a_class_are_assigned_after_the_body_of_its_init() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the properties of a
    // class that has an `__Init` of its own are assigned after the body of
    // it, and not before it -- the body may write them, and the assignment
    // of the class is the one that is left.
    assert_eq!(
        the_answer_of(
            "class C {\n    __Init() {\n        this.value := 2\n    }\n    value := 1\n}\nobj := C()\nMsgBox obj.value"
        ),
        "1"
    );
    // The properties that come before the `__Init` are a duplicate of it,
    // and the ones that come after it are taken.
    assert_eq!(
        the_answer_of(
            "class C {\n    __Init() {\n    }\n    value := 1\n}\nobj := C()\nMsgBox obj.value"
        ),
        "1"
    );
    // The properties of the base are not assigned when the derived has an
    // `__Init` of its own.
    assert_eq!(
        the_answer_of(
            "class D {\n    dvalue := 1\n}\nclass C extends D {\n    __Init() {\n    }\n    cvalue := 2\n}\nobj := C()\nMsgBox obj.cvalue"
        ),
        "2"
    );
}

#[test]
fn the_names_of_the_own_properties_are_the_ones_that_the_script_wrote() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of an own
    // property is the name as the script wrote it.
    assert_eq!(
        the_answer_of("o := Object()\no.X := 1\ns := \"\"\nfor p in o.OwnProps()\n    s .= p\nMsgBox s"),
        "X"
    );
    assert_eq!(
        the_answer_of("o := {X: 1}\ns := \"\"\nfor p in o.OwnProps()\n    s .= p\nMsgBox s"),
        "X"
    );
    assert_eq!(
        the_answer_of("m := Map()\nm.X := 1\ns := \"\"\nfor p in m.OwnProps()\n    s .= p\nMsgBox s"),
        "X"
    );
    assert_eq!(
        the_answer_of(
            "class C {\n}\nobj := C()\nobj.X := 1\ns := \"\"\nfor p in obj.OwnProps()\n    s .= p\nMsgBox s"
        ),
        "X"
    );
}

#[test]
fn the_first_write_of_a_property_is_the_one_that_fixes_the_case_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the names of a class
    // are the names in lower case, and the first write is the one that fixes
    // the case of the name.
    assert_eq!(
        the_answer_of("o := Object()\no.X := 1\no.x := 2\ns := \"\"\nfor p in o.OwnProps()\n    s .= p\nMsgBox s"),
        "X"
    );
    assert_eq!(
        the_answer_of("o := Object()\no.x := 1\no.X := 2\ns := \"\"\nfor p in o.OwnProps()\n    s .= p\nMsgBox s"),
        "x"
    );
    // The value of the property is the one that was written last.
    assert_eq!(the_answer_of("o := Object()\no.X := 1\no.x := 2\nMsgBox o.X"), "2");
}

#[test]
fn the_names_of_the_own_properties_are_in_the_order_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the names are sorted
    // and the case of them does not count.
    assert_eq!(
        the_answer_of(
            "o := Object()\no.Zebra := 1\no.apple := 2\no.Mango := 3\ns := \"\"\nfor p in o.OwnProps()\n    s .= p . \" \"\nMsgBox s"
        ),
        "apple Mango Zebra "
    );
}

#[test]
fn the_get_method_of_the_prototype_fails_with_the_words_of_the_prototype() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    // class is a property of it, and the `GetMethod` of it fails with the
    // words of the prototype.
    assert_eq!(
        the_failure_of("c := Map\nc.GetMethod(\"Prototype\")"),
        "This value of type \"Prototype\" has no method named \"Call\". | GetMethod |"
    );
    assert_eq!(
        the_failure_of("class C {\n}\nC.GetMethod(\"Prototype\")"),
        "This value of type \"Prototype\" has no method named \"Call\". | GetMethod |"
    );
}

#[test]
fn the_own_properties_of_a_prototype_are_the_members_of_the_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    // class owns the class of the value and the members that the class
    // declares -- the methods of it and the properties of it.
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Map.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class__Enum__Item__NewCapacityCaseSenseClearCloneCountDeleteGetHasSet"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Array.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class__Enum__Item__NewCapacityCloneDeleteGetHasInsertAtLengthPopPushRemoveAt"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Buffer.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class__NewPtrSize"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Error.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class__New"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Object.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__ClassCloneDefinePropDeletePropGetOwnPropDescHasOwnPropOwnProps"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Abs.Base.OwnProps()\n    s .= p\nMsgBox s"),
        "__ClassBindCallIsBuiltInIsByRefIsOptionalIsVariadicMaxParamsMinParamsName"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in Enumerator.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class"
    );
    assert_eq!(
        the_answer_of("s := \"\"\nfor p in ClipboardAll.Prototype.OwnProps()\n    s .= p\nMsgBox s"),
        "__Class__New"
    );
}

#[test]
fn the_own_properties_of_the_prototype_of_a_class_of_the_script_are_its_methods() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    // class of the script owns the methods that the class declares itself,
    // and the ones that it inherits are not its own.
    assert_eq!(
        the_answer_of(
            "class C {\n    __Init() {\n    }\n    Say() {\n    }\n    static Shout() {\n    }\n}\ns := \"\"\nfor p in C.Prototype.OwnProps()\n    s .= p\nMsgBox s"
        ),
        "__Class__InitSay"
    );
    assert_eq!(
        the_answer_of(
            "class C {\n    Say() {\n    }\n}\nclass D extends C {\n    Other() {\n    }\n}\ns := \"\"\nfor p in D.Prototype.OwnProps()\n    s .= p\nMsgBox s"
        ),
        "__ClassOther"
    );
}

#[test]
fn the_prototype_answers_the_questions_about_the_class_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    assert_eq!(the_answer_of("MsgBox Map.Prototype.__Class"), "Map");
    assert_eq!(the_answer_of("MsgBox ObjOwnPropCount(Map.Prototype)"), "13");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasOwnProp(\"__Class\")"), "1");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasOwnProp(\"Count\")"), "1");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasOwnProp(\"Nope\")"), "0");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasProp(\"Count\")"), "1");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasMethod(\"Get\")"), "1");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.HasMethod(\"Count\")"), "0");
    assert_eq!(the_answer_of("MsgBox Map.Prototype.Get.Name"), "Map.Prototype.Get");
    assert_eq!(
        the_answer_of("MsgBox Map.Prototype.GetMethod(\"Get\").Name"),
        "Map.Prototype.Get"
    );
    assert_eq!(
        the_answer_of("MsgBox Object.Prototype.GetMethod(\"Clone\").Name"),
        "Object.Prototype.Clone"
    );
}

#[test]
fn the_property_of_a_prototype_is_refused_with_the_kind_of_the_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype is not
    // an instance of the class, and the property of the class cannot be read
    // on it.
    assert_eq!(
        the_failure_of("MsgBox Map.Prototype.Count"),
        "Expected a Map but got a Prototype. | Map.Prototype.Count.Get |"
    );
    assert_eq!(
        the_failure_of("MsgBox File.Prototype.Pos"),
        "Expected a File but got a Prototype. | File.Prototype.Pos.Get |"
    );
    assert_eq!(
        the_failure_of("MsgBox Abs.Base.Name"),
        "Expected a Func but got a Prototype. | Func.Prototype.Name.Get |"
    );
}

#[test]
fn the_call_of_a_method_of_a_prototype_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the method of a class
    // cannot be called on the prototype of it, and the copy of a prototype is
    // refused as well.
    assert_eq!(
        the_failure_of("Map.Prototype.Get(\"a\")"),
        "Expected a Map but got a Prototype. | Map.Prototype.Get |"
    );
    assert_eq!(
        the_failure_of("Object.Prototype.Clone()"),
        "Type mismatch. | Object.Prototype.Clone |"
    );
}

#[test]
fn the_members_of_a_prototype_are_read_only() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the members of the
    // prototype of a class are read-only, and a name that the prototype does
    // not own is written.
    assert_eq!(
        the_failure_of("Map.Prototype.Count := 5"),
        "Property is read-only. |  | Count"
    );
    assert_eq!(
        the_failure_of("Map.Prototype.Get := 5"),
        "Property is read-only. |  | Get"
    );
    assert_eq!(the_answer_of("Map.Prototype.Nope := 5\nMsgBox \"ok\""), "ok");
    // The base of a prototype is written the way the base of an object is.
    assert_eq!(the_answer_of("Map.Prototype.Base := Object()\nMsgBox \"ok\""), "ok");
}

#[test]
fn the_methods_are_values() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a method is handed
    // over as a function, and the name of it is the name of the member.
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.Clone.Name"), "Object.Prototype.Clone");
    assert_eq!(
        the_answer_of("o := Object()\nMsgBox o.OwnProps.Name"),
        "Object.Prototype.OwnProps"
    );
    // The four that the script asks about every day keep the name they
    // have.
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.HasProp.Name"), "HasProp");
    assert_eq!(the_answer_of("m := Map()\nMsgBox m.Get.Name"), "Map.Prototype.Get");
    assert_eq!(the_answer_of("f := Abs\nMsgBox f.Call.Name"), "Func.Prototype.Call");
}

#[test]
fn the_get_method_of_a_value_hands_the_method_over() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    assert_eq!(
        the_answer_of("o := Object()\nMsgBox o.GetMethod(\"Clone\").Name"),
        "Object.Prototype.Clone"
    );
    assert_eq!(
        the_answer_of("m := Map()\nMsgBox m.GetMethod(\"Get\").Name"),
        "Map.Prototype.Get"
    );
    assert_eq!(
        the_answer_of("a := [1]\nMsgBox a.GetMethod(\"Push\").Name"),
        "Array.Prototype.Push"
    );
    assert_eq!(
        the_answer_of("f := Abs\nMsgBox f.GetMethod(\"Call\").Name"),
        "Func.Prototype.Call"
    );
    assert_eq!(
        the_answer_of("o := Error(\"x\")\nMsgBox o.GetMethod(\"Clone\").Name"),
        "Object.Prototype.Clone"
    );
    assert_eq!(
        the_answer_of(
            "class Thing {\n    Say() {\n        return 1\n    }\n}\nobj := Thing()\nMsgBox obj.GetMethod(\"Say\").Name"
        ),
        "Thing.Prototype.Say"
    );
    // The method that is not there is refused with the words of the value.
    assert_eq!(
        the_failure_of("m := Map()\nm.GetMethod(\"Nope\")"),
        "This value of type \"Map\" has no method named \"Nope\". | GetMethod |"
    );
}

#[test]
fn the_number_of_the_arguments_of_a_method_is_the_one_of_the_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the object is one of
    // the arguments of the call.
    assert_eq!(the_answer_of("m := Map()\nMsgBox m.Get.MinParams"), "2");
    assert_eq!(the_answer_of("m := Map()\nMsgBox m.Get.MaxParams"), "3");
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.Clone.MinParams"), "1");
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.Clone.MaxParams"), "1");
    assert_eq!(the_answer_of("o := Object()\nMsgBox o.DefineProp.MaxParams"), "3");
}

#[test]
fn the_prototype_of_an_any_has_no_method_at_all() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of an
    // `Any`, of a `ComObject` and of a `ComValue` is not a prototype: it has
    // no method at all.
    assert_eq!(
        the_failure_of("Any.Prototype.OwnProps()"),
        "This value of type \"Prototype\" has no method named \"OwnProps\". |  |"
    );
    assert_eq!(
        the_failure_of("ComValue.Prototype.OwnProps()"),
        "This value of type \"Prototype\" has no method named \"OwnProps\". |  |"
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
