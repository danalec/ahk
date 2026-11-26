//! The `MinIndex()`, the `MaxIndex()` and the static of a class of the v1.
//!
//! The `MinIndex()` and the `MaxIndex()` of the v1 are the lowest and the
//! highest index of an array that has a value, and the static of a class is
//! a member of the instance of it.
//!
//! Every case of this file was measured on the binary of AutoHotkey
//! v1.1.37.02, and the probe of each one is written in the
//! comment of it. The probe went to
//! the probe of the measurement and the answer
//! of it to the file next to it.
//!
//! The cases run against a fake desktop: the arrays and the classes are the
//! business of the runtime, and not of the machine.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing: the cases of the scope do not need the
/// machine, and the methods that a capability does not have answer that
/// they do not have it.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}
impl Clipboard for Fake {}
impl Timers for Fake {}
impl Hotkeys for Fake {}
impl Native for Fake {}
impl Com for Fake {}
impl Files for Fake {}
impl Windows for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        Ok(format!("typed {text}"))
    }

    fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(format!("pressed {name}"))
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok("clicked".into())
    }

    fn mouse_down(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _button: &str,
    ) -> Result<String, String> {
        Ok(format!("{_button} is down"))
    }

    fn mouse_up(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        _button: &str,
    ) -> Result<String, String> {
        Ok(format!("{_button} is up"))
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
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, format!("ran {command}"), String::new()))
    }
}

/// Run a script of the v1 and answer with the report.
fn run(script: &str) -> Report {
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::v1())
        .run(&statements)
}

// -- the indexes of an array ---------------------------------------------------------

#[test]
fn the_min_and_the_max_index_are_the_indexes_that_have_a_value() {
    // The probe:
    //
    // ```ahk
    // a := [1, 2, 3]
    // r := a.MinIndex() "-" a.MaxIndex()
    // ```
    //
    // The answer of the v1.1.37.02: "1-3".
    let report = run(
        "a := [1, 2, 3]\n\
         r := a.MinIndex() \"-\" a.MaxIndex()\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1-3"]);
}

#[test]
fn the_min_and_the_max_index_of_an_array_that_has_no_value_are_blank() {
    // The probe:
    //
    // ```ahk
    // b := []
    // r := "[" b.MinIndex() "][" b.MaxIndex() "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[][]". An array that has
    // no value at all answers the empty text for the two of them.
    let report = run(
        "b := []\n\
         r := \"[\" b.MinIndex() \"][\" b.MaxIndex() \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[][]"]);
}

#[test]
fn the_min_and_the_max_index_of_an_object_that_is_not_an_array_are_blank() {
    // The probe:
    //
    // ```ahk
    // d := {b: 1}
    // r := "[" d.MinIndex() "][" d.MaxIndex() "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[][]". An object that is
    // not an array has no index at all.
    let report = run(
        "d := {b: 1}\n\
         r := \"[\" d.MinIndex() \"][\" d.MaxIndex() \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[][]"]);
}

#[test]
fn the_max_index_of_an_array_with_holes_is_the_highest_index_that_has_a_value() {
    // The probe:
    //
    // ```ahk
    // h := [1, 2, 3]
    // h[5] := 4
    // r := h.MinIndex() "-" h.MaxIndex()
    // ```
    //
    // The answer of the v1.1.37.02: "1-5". The place that is
    // not there is not a value, and the highest index that has a value is
    // the fifth one.
    let report = run(
        "h := [1, 2, 3]\n\
         h[5] := 4\n\
         r := h.MinIndex() \"-\" h.MaxIndex()\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1-5"]);
}

#[test]
fn the_min_index_of_an_array_that_has_only_its_third_one_is_three() {
    // The probe:
    //
    // ```ahk
    // t := []
    // t[3] := 9
    // r := t.MinIndex() "-" t.MaxIndex()
    // ```
    //
    // The answer of the v1.1.37.02: "3-3". The places that
    // are not there are not values, and the lowest index that has a value is
    // the third one.
    let report = run(
        "t := []\n\
         t[3] := 9\n\
         r := t.MinIndex() \"-\" t.MaxIndex()\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3-3"]);
}

// -- the static of a class ----------------------------------------------------------

#[test]
fn the_static_of_a_class_is_read_by_this_in_an_instance() {
    // The probe:
    //
    // ```ahk
    // class Probe {
    //     Static MaxOptions := 10
    //     Create(HWND, Options*) {
    //         return "min=" Options.MinIndex() " max=" Options.MaxIndex() " thisMax=" This.MaxOptions
    //     }
    // }
    // o := new Probe
    // r := o.Create(0, [1])
    // ```
    //
    // The answer of the v1.1.37.02: "inst: min=1 max=1
    // thisMax=10". The static of the class is a member of the instance of
    // it, and the `This` of a method reads it.
    let report = run(
        "class Probe {\n\
         \x20   Static MaxOptions := 10\n\
         \x20   Create(HWND, Options*) {\n\
         \x20       return \"min=\" Options.MinIndex() \" max=\" Options.MaxIndex() \" thisMax=\" This.MaxOptions\n\
         \x20   }\n\
         }\n\
         o := new Probe\n\
         r := o.Create(0, [1])\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["min=1 max=1 thisMax=10"]);
}

#[test]
fn the_indexes_of_the_options_of_a_method_are_the_ones_of_the_arguments() {
    // The probe:
    //
    // ```ahk
    // class Probe {
    //     Static MaxOptions := 10
    //     Create(HWND, Options*) {
    //         return "min=" Options.MinIndex() " max=" Options.MaxIndex() " thisMax=" This.MaxOptions
    //     }
    // }
    // r := Probe.Create(0, [1], [2], [3])
    // ```
    //
    // The answer of the v1.1.37.02: "min=1 max=3
    // thisMax=10". The variadic parameter is an array, and its indexes are
    // the ones of the arguments that the call gave.
    let report = run(
        "class Probe {\n\
         \x20   Static MaxOptions := 10\n\
         \x20   Create(HWND, Options*) {\n\
         \x20       return \"min=\" Options.MinIndex() \" max=\" Options.MaxIndex() \" thisMax=\" This.MaxOptions\n\
         \x20   }\n\
         }\n\
         r := Probe.Create(0, [1], [2], [3])\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["min=1 max=3 thisMax=10"]);
}
