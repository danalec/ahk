//! The COM of the v1, against the machine of the runner.
//!
//! The `ComObjCreate` of the v1 answers the number of the object when the
//! script asks for an interface, and the wrapper of it when it does not.
//! A script of the v1 asks for the interface of the file dialog of the
//! machine -- `ComObjCreate("{DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7}",
//! "{42f85136-db7e-439c-85f1-e4075d135fc8}")` -- and reads the table of its
//! methods with the number: `NumGet(pDlg+0, "UPtr")`.
//!
//! The facts of the v1 that this case carries were measured on the binary of
//! AutoHotkey v1.1.37.02, and the probe is the one of the measurement.

#![cfg(target_os = "windows")]

mod json_doc;

use json_doc::Json;
use std::io::Write;
use std::process::{Command, Stdio};

/// The runner, as it was built.
const RUNNER: &str = env!("CARGO_BIN_EXE_ahk");

/// Run a script of the v1 and answer with the report.
fn run(source: &str) -> Json {
    let mut child = Command::new(RUNNER)
        .arg("--dialect=v1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner starts");
    child
        .stdin
        .take()
        .expect("the runner reads a script")
        .write_all(source.as_bytes())
        .expect("the script goes in");
    let output = child.wait_with_output().expect("the runner finishes");
    let text = String::from_utf8(output.stdout).expect("the report is utf-8");
    Json::parse(text.trim())
}

/// The whole number of a variable of the script.
fn the_number(report: &Json, name: &str) -> i64 {
    report
        .get("variables")
        .get(name)
        .and_then(|value| value.as_i64())
        .unwrap_or_else(|| panic!("the report has no number {name:?}"))
}

/// The text of a variable of the script.
///
/// The number of the v1 is a "pure number": it keeps the text that was
/// written, and the report writes that text. Measured on the binary of
/// v1.1.37.02: the `x := 1 ? 2 : 3` leaves the text "2" in
/// the variable, and not the number 2.
fn the_text(report: &Json, name: &str) -> String {
    report
        .get("variables")
        .get(name)
        .map(|value| match value {
            ahk_cli::json::Read::Text(text) => text.clone(),
            other => format!("{other:?}"),
        })
        .unwrap_or_else(|| panic!("the report has no value {name:?}"))
}

/// The answer of the `ComObjCreate` with the interface of the file dialog is
/// the number of the object.
///
/// Measured on the binary of v1.1.37.02: the answer is a
/// number -- 10322152 on that run -- and the table of the methods of the
/// object is at the address of it. The script of a script of the v1
/// reads the table with `NumGet(pDlg+0, "UPtr")` and calls the methods of
/// it.
#[test]
fn the_com_obj_create_with_an_interface_answers_the_number_of_the_object() {
    let report = run(
        "pDlg := ComObjCreate(\"{DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7}\", \
         \"{42f85136-db7e-439c-85f1-e4075d135fc8}\")\n\
         vt := NumGet(pDlg+0, \"UPtr\")\n",
    );
    assert!(report.boolean("ok"), "{}", report.text("error"));
    // The number of the object is above zero, and the table of its methods
    // is at the address of it.
    let pointer = the_number(&report, "pdlg");
    assert!(pointer > 0, "the pointer of the object is {pointer}");
    let table = the_number(&report, "vt");
    assert!(table > 0, "the table of the methods is {table}");
}

/// The answer of the `ComObjCreate` without an interface is the wrapper of
/// the object.
///
/// Measured on the binary of v1.1.37.02: the answer of the
/// `ComObjCreate("Shell.Application")` is an object, and not a number.
#[test]
fn the_com_obj_create_without_an_interface_answers_the_wrapper_of_the_object() {
    let report = run(
        "pShell := ComObjCreate(\"Shell.Application\")\n\
         x := IsObject(pShell) ? 1 : 0\n",
    );
    assert!(report.boolean("ok"), "{}", report.text("error"));
    assert_eq!(
        the_text(&report, "x"),
        "1",
        "the answer of the ComObjCreate is the wrapper of the object"
    );
}

/// The class of the file dialog does not serve the `IDispatch`, and the
/// `ComObjCreate` without an interface is refused.
///
/// Measured on the binary of v1.1.37.02
/// (the probe of the measurement): the `ComObjCreate("{DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7}")`
/// without an interface is refused -- `no` -- and the one with the interface
/// of it works -- `yes`. The `Shell.Application` works in both ways.
#[test]
fn the_class_of_the_file_dialog_is_refused_without_the_interface() {
    let report = run(
        "Try a := ComObjCreate(\"{DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7}\")\n\
         x := a ? 1 : 0\n\
         Try b := ComObjCreate(\"{DC1C5A9C-E88A-4dde-A5A1-60F82A20AEF7}\", \
         \"{42f85136-db7e-439c-85f1-e4075d135fc8}\")\n\
         y := b ? 1 : 0\n",
    );
    assert!(report.boolean("ok"), "{}", report.text("error"));
    assert_eq!(
        the_text(&report, "x"),
        "0",
        "the class of the file dialog does not serve the IDispatch"
    );
    assert_eq!(
        the_text(&report, "y"),
        "1",
        "the one with the interface of it works"
    );
}
