//! The `StrGet` and the `StrPut` of the v1.
//!
//! The v1 reads and writes the memory of the script and the memory of the
//! machine: the target of a `StrPut` is an address -- `&buf` is the address
//! of the memory of the variable -- and the answer of a `StrPut` is the
//! number of the units that were written, the terminator included.
//!
//! Every case of this file was measured on the binary of AutoHotkey
//! v1.1.37.02, and the probe of each one is written in the
//! comment of it. The probes went to
//! the probe of the measurement and
//! the probe of the measurement, and the answers
//! of them to the files next to them.
//!
//! The cases run against a fake desktop: the memory of the script is the
//! business of the runtime, and the memory of the machine is not asked here.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing: the cases of the text do not need the
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
    fn click(&mut self, _x: i64, _y: i64, button: &str, _count: u32) -> Result<String, String> {
        Ok(format!("{button} is down"))
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

// -- the size that a text needs -----------------------------------------------------

#[test]
fn the_size_of_a_text_in_the_utf16_is_the_number_of_the_units() {
    // The probe:
    //
    // ```ahk
    // s := StrPut("hello", "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: 6 -- the five units of the
    // text and the terminator. The answer is the number of the units of the
    // encoding, and not the number of the bytes.
    let report = run(
        "s := StrPut(\"hello\", \"UTF-16\")\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["6"]);
}

#[test]
fn the_size_of_a_text_in_the_utf8_is_the_number_of_the_bytes() {
    // The probe:
    //
    // ```ahk
    // s := StrPut("hello", "UTF-8")
    // ```
    //
    // The answer of the v1.1.37.02: 6. The unit of the `UTF-8`
    // is the byte, and the answer counts the bytes and the terminator.
    let report = run(
        "s := StrPut(\"hello\", \"UTF-8\")\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["6"]);
}

#[test]
fn the_size_of_a_text_without_an_encoding_is_the_size_in_the_encoding_of_the_machine() {
    // The probe:
    //
    // ```ahk
    // s := StrPut("hello")
    // ```
    //
    // The answer of the v1.1.37.02: 6. The encoding of the
    // machine is the `UTF-16` of the build of Unicode, and the answer counts
    // the units of it.
    let report = run(
        "s := StrPut(\"hello\")\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["6"]);
}

#[test]
fn the_size_of_a_text_with_a_name_that_is_not_an_encoding_is_empty() {
    // The probe:
    //
    // ```ahk
    // s := StrPut("hello", "not a variable")
    // ```
    //
    // The answer of the v1.1.37.02: the empty text. The v1
    // does not fail here: it answers the empty text when the parameters are
    // not ones that it takes.
    let report = run(
        "s := StrPut(\"hello\", \"not a variable\")\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![""]);
}

// -- the writing of a text -----------------------------------------------------------

#[test]
fn the_writing_of_a_text_answers_the_number_of_the_units_that_were_written() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // n := StrPut("hello world", &buf, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: 12 -- the eleven units of
    // the text and the terminator.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         n := StrPut(\"hello world\", &buf, \"UTF-16\")\n\
         FileAppend, %n%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["12"]);
}

#[test]
fn the_writing_of_a_text_at_an_offset_of_a_variable_writes_there() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(st, 16, 0)
    // n := StrPut("XY", &st+4, "UTF-16")
    // t := StrGet(&st+4, "UTF-16")
    // w := StrGet(&st, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: 3, "XY" and the empty
    // text. The address of a variable with an offset is a place inside the
    // memory of it, and the whole memory starts with the zeros that the
    // `VarSetCapacity` left there.
    let report = run(
        "VarSetCapacity(st, 16, 0)\n\
         n := StrPut(\"XY\", &st+4, \"UTF-16\")\n\
         t := StrGet(&st+4, \"UTF-16\")\n\
         w := StrGet(&st, \"UTF-16\")\n\
         r := n \"|\" t \"|[\" w \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3|XY|[]"]);
}

#[test]
fn a_text_that_fills_the_length_that_was_asked_for_is_written_without_the_terminator() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // n := StrPut("hello", &buf, 5, "UTF-16")
    // t := StrGet(&buf, 5, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: 5 and "hello". The text is
    // exactly the length that was asked for, and there is no room for the
    // terminator.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         n := StrPut(\"hello\", &buf, 5, \"UTF-16\")\n\
         t := StrGet(&buf, 5, \"UTF-16\")\n\
         r := n \"|\" t\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5|hello"]);
}

#[test]
fn a_text_that_does_not_fit_in_the_length_that_was_asked_for_is_not_written() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // n := StrPut("hello", &buf, 4, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: 0. The text is longer than
    // the room that was given, and nothing is written.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         n := StrPut(\"hello\", &buf, 4, \"UTF-16\")\n\
         FileAppend, %n%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_length_of_zero_is_refused_and_the_answer_is_the_empty_text() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // n := StrPut("hello", &buf, 0, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: the empty text. The
    // `StrPut(str, address, 0)` is not supported by the v1.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         n := StrPut(\"hello\", &buf, 0, \"UTF-16\")\n\
         r := \"[\" n \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

// -- the reading of a text -----------------------------------------------------------

#[test]
fn the_reading_of_a_text_gives_the_text_that_was_written() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // StrPut("hello world", &buf, "UTF-16")
    // t := StrGet(&buf, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: "hello world".
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         StrPut(\"hello world\", &buf, \"UTF-16\")\n\
         t := StrGet(&buf, \"UTF-16\")\n\
         FileAppend, %t%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello world"]);
}

#[test]
fn the_reading_without_an_encoding_reads_the_encoding_of_the_machine() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // StrPut("hello world", &buf, "UTF-16")
    // t := StrGet(&buf)
    // ```
    //
    // The answer of the v1.1.37.02: "hello world". The
    // encoding of the machine is the `UTF-16` of the build of Unicode, and
    // the text that was written in the `UTF-16` is read back as it is.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         StrPut(\"hello world\", &buf, \"UTF-16\")\n\
         t := StrGet(&buf)\n\
         FileAppend, %t%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello world"]);
}

#[test]
fn the_reading_with_a_length_reads_that_many_units() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // StrPut("hello world", &buf, "UTF-16")
    // t := StrGet(&buf, 3, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: "hel".
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         StrPut(\"hello world\", &buf, \"UTF-16\")\n\
         t := StrGet(&buf, 3, \"UTF-16\")\n\
         FileAppend, %t%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hel"]);
}

#[test]
fn the_reading_with_a_length_of_zero_is_the_empty_text() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // StrPut("hello", &buf, "UTF-16")
    // t := StrGet(&buf, 0, "UTF-16")
    // ```
    //
    // The answer of the v1.1.37.02: the empty text.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         StrPut(\"hello\", &buf, \"UTF-16\")\n\
         t := StrGet(&buf, 0, \"UTF-16\")\n\
         r := \"[\" t \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_reading_with_a_name_that_is_not_an_encoding_is_the_empty_text() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // StrPut("hello", &buf, "UTF-16")
    // t := StrGet(&buf, "XXXX")
    // ```
    //
    // The answer of the v1.1.37.02: the empty text. The v1
    // does not fail here: it answers the empty text when the parameters are
    // not ones that it takes.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         StrPut(\"hello\", &buf, \"UTF-16\")\n\
         t := StrGet(&buf, \"XXXX\")\n\
         r := \"[\" t \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_reading_of_a_text_that_is_not_an_address_is_the_empty_text() {
    // The probe:
    //
    // ```ahk
    // t := StrGet("hello")
    // ```
    //
    // The answer of the v1.1.37.02: the empty text. The source
    // of the `StrGet` of the v1 is the address of the text.
    let report = run(
        "t := StrGet(\"hello\")\n\
         r := \"[\" t \"]\"\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn a_text_that_is_written_in_the_utf8_is_read_back_in_the_utf8() {
    // The probe:
    //
    // ```ahk
    // VarSetCapacity(buf, 100, 0)
    // n := StrPut("hi", &buf, "UTF-8")
    // t := StrGet(&buf, "UTF-8")
    // ```
    //
    // The answer of the v1.1.37.02: 3 and "hi". The unit of
    // the `UTF-8` is the byte, and the answer of the writing counts the two
    // bytes of the text and the terminator.
    let report = run(
        "VarSetCapacity(buf, 100, 0)\n\
         n := StrPut(\"hi\", &buf, \"UTF-8\")\n\
         t := StrGet(&buf, \"UTF-8\")\n\
         r := n \"|\" t\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3|hi"]);
}
