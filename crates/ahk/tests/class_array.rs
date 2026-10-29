//! The `Array` class, against a fake desktop.
//!
//! An array never touches the machine: the desktop here is a fake that
//! answers nothing, because nothing is asked of it.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Machine, Native, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}

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
fn an_array_is_made_empty() {
    let report = run("a := Array()\nMsgBox a.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn an_array_is_made_with_its_values() {
    let report = run("a := Array(\"x\", \"y\")\nMsgBox a.Length . \" \" . a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 x"]);
}

#[test]
fn a_value_goes_at_the_end() {
    let report = run("a := Array()\na.Push(\"x\")\nMsgBox a.Length . \" \" . a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 x"]);
}

#[test]
fn the_push_answers_the_array() {
    let report = run("a := Array()\nMsgBox a.Push(\"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["array"]);
}

#[test]
fn the_last_value_can_be_taken_away() {
    let report = run("a := Array(\"x\", \"y\")\nMsgBox a.Pop() . \" \" . a.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["y 1"]);
}

#[test]
fn taking_away_the_last_value_of_an_empty_array_is_an_error() {
    let report = run("a := Array()\na.Pop()");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid index."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_value_can_be_inserted_in_the_middle() {
    let report = run("a := Array(\"x\", \"y\")\na.InsertAt(2, \"z\")\nMsgBox a[1] . \" \" . a[2] . \" \" . a[3]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x z y"]);
}

#[test]
fn a_value_can_be_inserted_at_the_end() {
    let report = run("a := Array(\"x\")\na.InsertAt(0, \"z\")\nMsgBox a.Length . \" \" . a[1] . \" \" . a[2]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 x z"]);
}

#[test]
fn a_value_can_be_taken_away_from_the_middle() {
    let report = run("a := Array(\"x\", \"y\", \"z\")\nMsgBox a.RemoveAt(2) . \" \" . a[2] . \" \" . a.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["y z 2"]);
}

#[test]
fn a_range_of_values_can_be_taken_away() {
    let report = run("a := Array(\"x\", \"y\", \"z\")\nMsgBox a.RemoveAt(1, 2) . \" \" . a[1] . \" \" . a.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 z 1"]);
}

#[test]
fn a_value_can_be_deleted_and_the_place_stays() {
    let report = run("a := Array(\"x\", \"y\")\nMsgBox a.Delete(1) . \" \" . a.Length . \" \" . a.Has(1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x 2 0"]);
}

#[test]
fn a_place_without_a_value_is_an_error() {
    let report = run("a := Array(\"x\", \"y\")\na.Delete(1)\nMsgBox a[1]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Key not found."));
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn the_default_answers_for_a_place_without_a_value() {
    let report = run("a := Array(\"x\", \"y\")\na.Delete(1)\nMsgBox a.Get(1, \"none\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["none"]);
}

#[test]
fn a_place_that_is_not_there_is_an_error() {
    let report = run("a := Array(\"x\")\nMsgBox a[2]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid index."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_place_that_is_not_a_number_is_an_error() {
    let report = run("a := Array(\"x\")\nMsgBox a[\"1\"]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected an Integer but got a String."));
}

#[test]
fn an_array_says_whether_it_has_a_value_at_a_place() {
    let report = run("a := Array(\"x\")\nMsgBox a.Has(1) . \" \" . a.Has(2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn a_place_counts_from_the_end_when_it_is_negative() {
    let report = run("a := Array(\"x\", \"y\")\nMsgBox a[-1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["y"]);
}

#[test]
fn a_clone_is_a_copy_of_the_array() {
    let report = run("a := Array(\"x\")\nc := a.Clone()\nc.Push(\"y\")\nMsgBox a.Length . \" \" . c.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 2"]);
}

#[test]
fn the_length_can_be_set() {
    let report = run("a := Array(\"x\")\na.Length := 3\nMsgBox a.Length . \" \" . a.Has(2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 0"]);
}

#[test]
fn a_shorter_length_takes_the_places_at_the_end_away() {
    let report = run("a := Array(\"x\", \"y\", \"z\")\na.Length := 1\nMsgBox a.Length . \" \" . a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 x"]);
}

#[test]
fn the_capacity_can_be_set() {
    let report = run("a := Array(\"x\")\na.Capacity := 10\nMsgBox a.Capacity");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["10"]);
}

#[test]
fn a_capacity_below_the_length_takes_the_places_at_the_end_away() {
    let report = run("a := Array(\"x\", \"y\", \"z\")\na.Capacity := 1\nMsgBox a.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_enumeration_is_in_the_order_of_the_values() {
    let report = run("a := Array(\"x\", \"y\")\nMsgBox a.__Enum()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x,y"]);
}

#[test]
fn the_own_properties_of_an_array_are_the_ones_the_script_gave_it() {
    let report = run("a := Array()\na.DefineProp(\"x\", 7)\nMsgBox a.OwnProps() . \" \" . a.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x 7"]);
}

#[test]
fn an_array_knows_the_methods_it_has() {
    let report = run("a := Array()\nMsgBox a.HasMethod(\"Push\") . \" \" . a.HasMethod(\"Nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}
