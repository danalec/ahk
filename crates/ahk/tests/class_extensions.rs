//! The members the KeySharp extension adds to the `Array`, against a fake
//! desktop.
//!
//! The rules are the ones the documentation of KeySharp writes
//! (`Keysharp/docs/reference.md`, the members of the classes of AutoHotkey),
//! and this file pins the ones the tree carries: the searches compare the
//! actual underlying values, the callbacks take the place they are given,
//! and a start of zero throws.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect::{self, Dialect};
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing.
#[derive(Debug, Default)]
struct Fake;

impl Clipboard for Fake {}
impl Desktop for Fake {}
impl Dialogs for Fake {}
impl Files for Fake {}
impl Hotkeys for Fake {}
impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}
impl Machine for Fake {}
impl Native for Fake {}
impl Network for Fake {}
impl Pictures for Fake {}
impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}
impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
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
impl Sound for Fake {}
impl Timers for Fake {}
impl Windows for Fake {}

/// A script in the dialect of KeySharp, and the report of it.
fn run_ks(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = ahk::parse_with(source, dialect::ks()).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::ks())
        .run(&statements)
}

/// The text an output line of the report holds.
fn line(report: &Report, index: usize) -> String {
    report.output[index].clone()
}

#[test]
fn a_map_answers_the_array_of_the_answers_of_the_callback() {
    // The example of the documentation of KeySharp: `[10, 20, 30]` mapped
    // with `(x, i) => x * i` answers `[10, 40, 90]`.
    let report = run_ks(
        "arr := [10, 20, 30]\nm := arr.Map((x, i) => x * i)\nMsgBox m.ToString()\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[10, 40, 90]");
}

#[test]
fn a_contains_and_an_indexof_compare_the_underlying_values() {
    // The documentation says the comparisons compare the actual underlying
    // values, so "1" is not 1 -- a search of an array is not the `=` of an
    // `if`.
    let report = run_ks(
        concat!(
            "a := [1, \"1\", 2]\n",
            "MsgBox a.Contains(1)\n",
            "MsgBox a.IndexOf(\"1\")\n",
            "MsgBox a.IndexOf(9)\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "2");
    assert_eq!(line(&report, 2), "0");
}

#[test]
fn a_negative_start_walks_from_the_end_toward_the_beginning() {
    let report = run_ks("MsgBox [10, 20, 10].IndexOf(10, -1)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "3");
}

#[test]
fn a_start_of_zero_throws() {
    let report = run_ks(
        concat!(
            "a := [10, 20]\n",
            "try {\n",
            "    a.IndexOf(10, 0)\n",
            "} catch as e {\n",
            "    MsgBox e.Message\n",
            "}\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        line(&report, 0),
        "Parameter #2 of Array.Prototype.IndexOf is invalid."
    );
}

#[test]
fn a_remove_takes_the_first_match_and_answers_whether_it_did() {
    let report = run_ks(
        concat!(
            "a := [10, 20, 10]\n",
            "r := a.Remove(10)\n",
            "MsgBox r\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "[20, 10]");
}

#[test]
fn a_remove_without_a_value_takes_the_first_place_that_has_none() {
    let report = run_ks(
        concat!(
            "a := [1, 2, 3]\n",
            "a.Delete(2)\n",
            "r := a.Remove()\n",
            "MsgBox r\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    // The place that went is the hole itself: the array keeps the first and
    // the third, and there is no place without a value any more.
    assert_eq!(line(&report, 1), "[1, 3]");
}

#[test]
fn a_sort_keeps_the_order_of_the_equal_ones() {
    // The documentation says only the sign of the answer counts, and equal
    // elements keep their order.
    let report = run_ks(
        concat!(
            "a := [\"b\", \"a\", \"c\"]\n",
            "a.Sort((one, two) => 0)\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[\"b\", \"a\", \"c\"]");
}

#[test]
fn a_tostring_quotes_the_texts_and_names_the_absent_places() {
    let report = run_ks(
        concat!(
            "a := [1, \"two\"]\n",
            "a.Delete(1)\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[unset, \"two\"]");
}

#[test]
fn a_join_puts_the_separator_between_the_elements() {
    let report = run_ks("MsgBox [10, 20, 30].Join(\"-\")\nMsgBox [10, 20].Join()\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "10-20-30");
    // The separator the documentation names as the default is the comma.
    assert_eq!(line(&report, 1), "10,20");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_members() {
    // The surface of the v2.0.19 stays pure: the same script that runs in
    // the dialect of KeySharp is refused in the v2, and the refusal is the
    // one of a member that is not there.
    let mut desktop = Fake;
    let statements = ahk::parse("x := [10, 20]\nx.Contains(10)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert!(report
        .error
        .as_deref()
        .is_some_and(|error| error.contains("has no method named \"Contains\"")));
}
