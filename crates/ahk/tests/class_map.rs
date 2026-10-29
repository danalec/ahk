//! The `Map` class, against a fake desktop.
//!
//! A map never touches the machine: the desktop here is a fake that
//! answers nothing, because nothing is asked of it.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
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

#[test]
fn a_map_is_made_empty() {
    let report = run("m := Map()\nMsgBox m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_map_is_made_with_its_pairs() {
    let report = run("m := Map(\"a\", 1, \"b\", 2)\nMsgBox m.Count . \" \" . m.Get(\"b\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 2"]);
}

#[test]
fn a_map_with_an_odd_number_of_parameters_is_an_error() {
    let report = run("m := Map(\"a\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid number of parameters."));
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_value_goes_under_a_key() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Get(\"a\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_set_answers_the_map() {
    // This is what v2 does: the map itself comes back, so that the calls
    // can be chained.
    let report = run("m := Map()\nMsgBox m.Set(\"a\", 1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["map"]);
}

#[test]
fn the_item_of_a_key_is_the_value_under_it() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m[\"a\"]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_key_that_is_not_there_is_an_error() {
    let report = run("m := Map()\nMsgBox m.Get(\"nope\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Item has no value."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn the_item_of_a_key_that_is_not_there_is_an_error() {
    let report = run("m := Map()\nMsgBox m[\"nope\"]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Item has no value."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_method_of_a_map_is_a_value() {
    // Measured on the binary of v2.0.19 on 2026-10-29: a method is handed
    // over as a function whose first parameter is `this`.
    let report = run(
        "m := Map()\nf := m.Clear\nMsgBox f.Name . \" \" . f.MinParams . \" \" . f.MaxParams",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Map.Prototype.Clear 1 1"]);
}

#[test]
fn a_method_of_a_map_is_called_with_the_object() {
    let report = run("m := Map(\"a\", 1)\nf := m.Clear\nf.Call(m)\nMsgBox m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_method_of_a_map_called_without_the_object_fails() {
    let report = run("m := Map()\nf := m.Clear\nf.Call()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too few parameters passed to function.")
    );
}

#[test]
fn the_class_of_a_map_has_no_method_to_hand_over() {
    // A class is not an instance: v2 does not hand its methods over.
    let report = run("f := Map.Clear");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Class\" has no property named \"Clear\".")
    );
}

#[test]
fn a_map_says_whether_it_has_a_key() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Has(\"a\") . \" \" . m.Has(\"b\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn a_key_can_be_taken_away() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Delete(\"a\") . \" \" . m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn taking_away_a_key_that_is_not_there_is_an_error() {
    let report = run("m := Map()\nMsgBox m.Delete(\"nope\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Item has no value."));
}

#[test]
fn a_map_can_be_emptied() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nm.Clear()\nMsgBox m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_clone_is_a_copy_of_the_map() {
    let report = run(
        "m := Map()\nm.Set(\"a\", 1)\nc := m.Clone()\nc.Set(\"b\", 2)\n\
         MsgBox m.Count . \" \" . c.Count . \" \" . c.Get(\"a\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 2 1"]);
}

#[test]
fn the_keys_are_compared_with_their_case() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Has(\"A\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_keys_can_be_compared_without_their_case() {
    let report = run("m := Map()\nm.CaseSense := \"Off\"\nm.Set(\"a\", 1)\nMsgBox m.Get(\"A\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_case_sense_says_how_the_keys_are_compared() {
    let report = run("m := Map()\nMsgBox m.CaseSense");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["On"]);
}

#[test]
fn the_case_sense_of_a_map_that_has_keys_cannot_change() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nm.CaseSense := \"Off\"");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Map must be empty"));
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_string_key_is_never_an_integer_key() {
    let report = run("m := Map()\nm.Set(1, \"one\")\nMsgBox m.Has(\"1\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_enumeration_is_in_the_order_of_the_entries() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nm.Set(\"b\", 2)\nMsgBox m.__Enum()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a,1,b,2"]);
}

#[test]
fn the_default_answers_for_a_key_that_is_not_there() {
    let report = run("m := Map()\nm.Default := 0\nMsgBox m.Get(\"nope\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_capacity_holds_the_entries() {
    let report = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Capacity");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_capacity_can_be_set() {
    let report = run("m := Map()\nm.Capacity := 10\nMsgBox m.Capacity");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["10"]);
}

#[test]
fn the_own_properties_of_a_map_are_the_ones_the_script_gave_it() {
    let report = run("m := Map()\nm.DefineProp(\"x\", 7)\nMsgBox m.OwnProps() . \" \" . m.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x 7"]);
}

#[test]
fn a_map_knows_the_methods_it_has() {
    let report = run("m := Map()\nMsgBox m.HasMethod(\"Set\") . \" \" . m.HasMethod(\"Nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn a_map_knows_the_properties_it_has() {
    let report = run("m := Map()\nMsgBox m.HasProp(\"Count\") . \" \" . m.HasProp(\"Nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn the_failure_of_an_item_of_a_map_names_the_method_and_the_key() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the failure of a
    // member of a class type carries the name of it in the `What`, and
    // the value that failed in the `Extra`.
    let report = run(
        "try {\n    m := Map()\n    x := m[\"nope\"]\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Item has no value. | Map.Prototype.__Item.Get | nope"]
    );
}

#[test]
fn the_number_of_the_arguments_of_a_method_of_a_map_is_checked() {
    let report = run(
        "try {\n    m := Map()\n    x := m.Get()\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too few parameters passed to function. |  | Map.Prototype.Get"]
    );
}

#[test]
fn a_method_of_a_map_with_too_many_arguments_is_an_error() {
    let report = run("m := Map()\nm.Get(\"a\", \"b\", \"c\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.")
    );
}

#[test]
fn the_failure_of_a_method_of_the_base_object_names_the_method() {
    let report = run(
        "try {\n    m := Map()\n    m.HasProp()\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too few parameters passed to function. |  | HasProp"]
    );
}

#[test]
fn the_failure_of_the_casesense_of_a_map_carries_the_value() {
    let report = run(
        "try {\n    m := Map()\n    m.CaseSense := 5\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Invalid value. | Map.Prototype.CaseSense.Set | 5"]
    );
}
