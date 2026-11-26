//! The native calls of the v1, against the machine of the runner.
//!
//! A script of the v1 asks for the size of the client area of its
//! window with a pointer of thirty-two bits -- `DllCall("GetClientRect",
//! "uint", hwnd, "uint", &rc)` -- and the address of the buffer of the
//! variable is above the four gigabytes. The pointer has to go out whole,
//! or the callee writes into another place, the size of the canvas is
//! zero, and the viewer opens the dialogue of the GDI+ that could not be
//! made.
//!
//! The facts of the v1 that this case carries were measured on the binary
//! of AutoHotkey v1.1.37.02, and the probe is the one of the measurement:
//! the
//! `RtlFillMemory` of the ntdll fills the buffer of the variable, and the
//! `NumGet` reads the bytes that it left there.

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

/// The pointer of a native call reaches the callee whole.
///
/// The fill of the memory says whether the callee wrote into the buffer
/// of the variable: the bytes of the buffer are the answer. The address
/// of the buffer is above the four gigabytes, and the type of the
/// argument is of thirty-two bits.
#[test]
fn the_pointer_of_a_native_call_reaches_the_callee_whole() {
    let report = run(
        "VarSetCapacity(buf, 16, 0)\n\
         DllCall(\"ntdll\\RtlFillMemory\", \"uint\", &buf, \"uint64\", 16, \"uint\", 65)\n\
         n := NumGet(buf, 0, \"int\")\n",
    );
    assert_eq!(the_number(&report, "n"), 0x4141_4141);
}

/// The pointer of a native call of sixty-four bits reaches the callee too.
///
/// The v1 of a script of the v1 asks for the size of the client area
/// with a pointer of thirty-two bits, and the answer is read with the
/// `NumGet` of the buffer of the variable.
#[test]
fn the_pointer_of_a_native_call_of_sixty_four_bits_reaches_the_callee() {
    let report = run(
        "VarSetCapacity(buf, 16, 0)\n\
         DllCall(\"ntdll\\RtlFillMemory\", \"uint64\", &buf, \"uint64\", 16, \"uint\", 66)\n\
         n := NumGet(buf, 0, \"int\")\n",
    );
    assert_eq!(the_number(&report, "n"), 0x4242_4242);
}

/// The write of the machine in the address of a variable comes back into
/// the variable.
///
/// The `&var` of the v1 is the address of the memory of the variable, and
/// not the address of a copy of it: the callee writes into the variable
/// itself, and the variable of the script holds the bytes that the callee
/// left there.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the `ack := "AAAA"`
/// of a function whose `DllCall("ntdll\RtlFillMemory", "uint", &ack, ...)`
/// writes four bytes of 65 holds a text whose first character is not the
/// `A` of the text that it had -- the length of the variable does not
/// change, and the bytes of it do.
#[test]
fn the_write_of_the_machine_in_the_address_of_a_variable_comes_back_into_it() {
    let report = run(
        "theProbe() {\n\
         global out\n\
         ack := \"AAAA\"\n\
         DllCall(\"ntdll\\RtlFillMemory\", \"uint\", &ack, \"uint64\", 4, \"uint\", 65)\n\
         out := (SubStr(ack, 1, 1) == \"A\")\n\
         }\n\
         theProbe()\n",
    );
    assert_eq!(the_number(&report, "out"), 0);
}
