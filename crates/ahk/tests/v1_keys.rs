//! The keys of the containers of the v1, against a fake desktop.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02: the array of the v1 is
//! a table of keys, and the item of a key that is not there is not a hole.
//! The `a[12345] := "x"` is the item of the key 12345, the array does not
//! grow to hold the places that are not there, and the enumeration of it
//! walks the items that are there.
//!
//! The tree kept the row of the places and grew it with holes, and the
//! write of a key that is the handle of a window cost the time of the
//! handle. The cases below are about the representation, and not about the
//! time: the row of the places of an array of the v1 is empty when the
//! script writes a key that is not there.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that does nothing: the keys of a container never touch the
/// machine.
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
impl Com for Fake {}

/// Run a script of the v1 and answer with the report.
fn run(script: &str) -> Report {
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::v1())
        .run(&statements)
}

/// Run a script of the v2 and answer with the report.
fn run_v2(script: &str) -> Report {
    let statements = ahk::parse_with(script, dialect::v2()).expect("the script of the v2 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::v2())
        .run(&statements)
}

#[test]
fn the_array_of_the_v1_keeps_the_item_under_its_key() {
    // The row of the places of the array stays empty: the write of a key
    // that is not there does not leave a hole, and the item is under the
    // key.
    let report = run("a := []\na[12345] := \"x\"");
    assert!(report.ok, "{:?}", report.error);
    let Some(Value::Object(array)) = report.variables.get("a") else {
        panic!("the array is not a variable of the run");
    };
    let array = array.borrow();
    let Some(Value::Object(state)) = array.value_of("array state") else {
        panic!("the array has no state");
    };
    let state = state.borrow();
    let Some(Value::List(places)) = state.value_of("items") else {
        panic!("the state of the array has no row of places");
    };
    assert!(
        places.is_empty(),
        "the array grew with holes: {:?}",
        places.len()
    );
    let keys: Vec<&String> = state.properties.keys().collect();
    assert!(
        keys.iter().any(|name| name.starts_with("key:")),
        "the item is not under a key: {keys:?}"
    );
}

#[test]
fn the_count_and_the_length_of_the_array_of_the_v1_are_the_keys_it_has() {
    // The count is the number of the items, and the length is the highest
    // key: the places that are not there are not items.
    let report = run(
        "a := []\na[12345] := \"x\"\ns := a.Count() . \" \" . a.Length() . \" \" . a.MaxIndex() . \" \" . a.HasKey(12345)\nMsgBox %s%",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 12345 12345 1"]);
}

#[test]
fn the_enumeration_of_the_array_of_the_v1_walks_the_items_that_are_there() {
    // The place that has no value of its own is not a turn of the
    // enumeration: the array of one item under the key 12345 has one turn.
    let report = run(
        "a := []\na[12345] := \"x\"\nn := 0\nfor k, v in a\n    n := n + 1\ns := n . \" \" . a[12345]\nMsgBox %s%",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 x"]);
}

#[test]
fn the_array_of_the_v1_keeps_the_places_that_are_there() {
    // The row of the places is still the row of the places: the item of the
    // fifth place goes under the key, and the fourth place has no value.
    let report = run(
        "a := [1, 2, 3]\na[5] := \"five\"\ns := a.Count() . \" \" . a.Length() . \" \" . a[4] . \" \" . a[5]\nMsgBox %s%",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4 5  five"]);
}

#[test]
fn the_keys_of_the_array_of_the_v1_are_written_fast() {
    // The write of ten thousand keys that are handles of windows costs the
    // time of the ten thousand writes: the tree grew the row of the places
    // with holes, and the write of a key cost the time of the key.
    let report = run(
        "a := []\nLoop, 10000\n    a[100000 + A_Index] := A_Index\ns := a.Count() . \" \" . a.Length()\nMsgBox %s%",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["10000 110000"]);
}

#[test]
fn the_array_of_the_v2_is_a_row_of_places() {
    // The v2 does not grow with the write of an item that is not there,
    // and the enumeration of it walks the places that have no value.
    let report = run_v2("a := [1, 2, 3]\na[5] := \"five\"");
    assert!(!report.ok, "the array of the v2 grew with the write");
    let report = run_v2("a := [1, , 3]\nn := 0\nfor k, v in a\n    n := n + 1\nMsgBox n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}
