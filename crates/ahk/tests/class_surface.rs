//! Every name of every class answers.
//!
//! The surface table says what v2.0.19 has for each class. A script that
//! uses one of those names must be answered: the class does the thing, or
//! it says why it cannot. What it may not say is that the value has no
//! such method, or no such property -- that would mean the name is
//! lost.
//!
//! A name of the surface is a method or a property, and the table does
//! not say which: the binary lists a property the same way it lists a
//! method, and a property with a setter is in the table twice. So the
//! probe asks both ways, the call first, because a method is a method
//! and a property is not one.
//!
//! The parts of a window -- a control, a list view, a status bar -- are
//! not things a script can reach: the only way to one is a window, and a
//! window is a Windows window. The module that owns them checks their
//! tables against the surface instead.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::runtime::{Report, Runtime};
use ahk::parse;
use std::time::Duration;

/// The surface of v2.0.19, read out of the binary.
const SURFACE: &str = include_str!("../reference/ahk-v2.0.19.surface.tsv");

/// The classes a script can reach, and the line that makes one.
///
/// The class of the registry and the class of the surface are the same
/// name, in the spelling of the surface.
const REACHABLE: &[(&str, &str)] = &[
    ("Array", "x := Array()"),
    ("Buffer", "x := Buffer(4)"),
    ("File", "x := FileOpen(\"probe.txt\", \"w\")"),
    ("Func", "x := Abs"),
    ("Gui", "x := Gui()"),
    ("Gui.Menu", "x := Menu()"),
    ("Hotkey", "x := Hotkey(\"^!a\", \"TheCallback\")"),
    ("Map", "x := Map()"),
];

/// A desktop that answers nothing, and keeps its files in memory.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}

impl Files for Fake {
    fn open_file(&mut self, _path: &str, _mode: &str) -> Result<i64, String> {
        Ok(1)
    }

    fn read_file_at(&mut self, _handle: i64, _pos: i64, _count: i64) -> Result<Vec<u8>, String> {
        Ok(Vec::new())
    }

    fn write_file_at(&mut self, _handle: i64, _pos: i64, bytes: &[u8]) -> Result<usize, String> {
        Ok(bytes.len())
    }

    fn file_size(&mut self, _handle: i64) -> Result<i64, String> {
        Ok(0)
    }

    fn close_file(&mut self, _handle: i64) -> Result<(), String> {
        Ok(())
    }
}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Windows for Fake {}

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

impl Native for Fake {}

/// The report of a script.
fn run(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

/// The names of the surface that belong to a class.
fn names_of(class: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for line in SURFACE.lines() {
        let mut fields = line.trim_end().split('\t');
        let kind = fields.next().unwrap_or("");
        let owner = fields.next().unwrap_or("");
        let name = fields.next().unwrap_or("");
        if kind == "method" && owner == class && !names.iter().any(|entry| entry == name) {
            names.push(name.to_string());
        }
    }
    names
}

/// Every name of every class answers.
#[test]
fn every_name_of_every_class_answers() {
    let mut lost: Vec<String> = Vec::new();
    for (class, make) in REACHABLE {
        for name in names_of(class) {
            let called = run(&format!("{make}\nx.{name}()"));
            let error = called.error.unwrap_or_default();
            if !error.contains("has no method named") {
                // The class answered the call, one way or another.
                continue;
            }
            // It is a property and not a method: the read has to answer.
            let read = run(&format!("{make}\ny := x.{name}"));
            let error = read.error.unwrap_or_default();
            if error.contains("has no property") {
                lost.push(format!("{class}.{name}: {error}"));
            }
        }
    }
    assert!(lost.is_empty(), "these names of v2 are lost: {lost:?}");
}

/// The class answers the names of the surface that are its own, and the
/// names of another class are not its own.
#[test]
fn a_class_does_not_answer_the_names_of_another() {
    // `Focus` is a name of a control and not of a window, and the two
    // tables have names in common besides.
    let report = run("x := Gui()\nx.Focus()");
    let error = report.error.unwrap_or_default();
    assert!(error.contains("has no method named"), "{error}");
}
