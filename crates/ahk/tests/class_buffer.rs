//! The `Buffer` class, against a fake desktop.
//!
//! A buffer is the one class that does not touch the machine: the
//! desktop is here because `Desktop` is the sum of the capabilities,
//! and the runtime drives one.
//!
//! v2.0.19 gives a buffer a size, an address and a constructor, and
//! nothing else. The reads and the writes of bytes belong to the file,
//! which reads into a buffer and writes out of one. Measured on the
//! binary of v2.0.19 on 2026-10-27, with `HasProp` and `HasMethod`.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::runtime::{Report, Runtime, Value};
use ahk::parse;
use std::time::Duration;

/// A desktop that answers nothing: a buffer does not ask it anything.
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

/// Run a script and answer the report of it.
fn run(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

#[test]
fn a_buffer_is_as_big_as_it_was_asked_for() {
    let report = run("b := Buffer(4)\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_buffer_has_an_address() {
    let report = run("b := Buffer(4)\naddress := b.Ptr");
    assert!(report.ok, "{:?}", report.error);
    match report.variables.get("address") {
        Some(Value::Int(number)) => assert!(*number > 0, "the address is {number}"),
        other => panic!("the address is {other:?}"),
    }
}

#[test]
fn a_buffer_may_be_made_again_by_its_constructor() {
    let report = run("b := Buffer(2)\nb.__New(3)\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_buffer_of_a_negative_size_fails() {
    // Measured on the binary of v2.0.19 on 2026-10-30: `Buffer(-1)`
    // answers "Invalid value.", of the class `ValueError`, and the
    // constructor is named in the `What`.
    let report = run(
        "try {\n    b := Buffer(-1)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid value. | Buffer.Prototype.__New | "]);
}

#[test]
fn a_buffer_may_be_made_of_another_size() {
    let report = run("b := Buffer(2)\nb.Size := 4\nMsgBox \"%b.Size%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_buffer_has_no_position() {
    // v2.0.19 does not give a buffer one: the position was never there.
    let report = run("b := Buffer(4)\nposition := b.Pos");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Buffer\" has no property named \"Pos\".")
    );
}

#[test]
fn a_buffer_does_not_read_bytes() {
    // The reads and the writes belong to the file, which reads into a
    // buffer and writes out of one.
    let report = run("b := Buffer(4)\nb.ReadUChar()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Buffer\" has no method named \"ReadUChar\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_buffer_has_no_item() {
    let report = run("b := Buffer(2)\nbyte := b[1]");
    assert!(!report.ok);
    // The buffer of v2 has no item either: the name of the item is the
    // name of the property that carries it.
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Buffer\" has no property named \"__Item\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn the_fill_of_a_buffer_is_the_byte_of_the_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a number that is
    // not a byte is not refused, and the byte of it is the fill.
    let report = run(
        "b := Buffer(3, 0x414243)\nMsgBox b.Size \" \" NumGet(b, 0, \"UChar\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 67"]);
}

#[test]
fn a_buffer_has_the_methods_of_the_base_object() {
    let report = run(
        "b := Buffer(4)\nmethod := b.HasMethod(\"ReadUChar\")\nprop := b.HasProp(\"Size\")\nbase := b.HasBase(Object.Prototype)\nMsgBox \"%method% %prop% %base%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // The buffer has no method of its own but the constructor, so the
    // read is not one of them.
    assert_eq!(report.output, vec!["0 1 1"]);
}

#[test]
fn the_inside_of_a_buffer_is_not_a_property_of_it() {
    let report = run(
        "b := Buffer(4)\ncount := ObjOwnPropCount(b)\ninside := b.HasOwnProp(\"buffer state\")\nMsgBox \"%count% %inside%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0 0"]);
}

#[test]
fn a_buffer_may_have_a_property_of_its_own() {
    let report = run(
        "b := Buffer(4)\nb.DefineProp(\"x\", {Value: 1})\nvalue := b.x\ngone := b.DeleteProp(\"x\")\nMsgBox \"%value% %gone%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1"]);
}

// -- the numbers --------------------------------------------------------------

#[test]
fn a_number_goes_into_a_buffer_and_comes_back() {
    let report = run(
        "b := Buffer(8)\nNumPut(\"Int\", 42, b)\nvalue := NumGet(b, \"Int\")\nMsgBox \"%value%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["42"]);
}

#[test]
fn every_type_of_a_number_is_a_number() {
    let report = run(
        "b := Buffer(64)\n\
         NumPut(\"UInt\", 4294967295, b, 0)\n\
         NumPut(\"Int\", -2, b, 4)\n\
         NumPut(\"Int64\", -3, b, 8)\n\
         NumPut(\"Short\", -4, b, 16)\n\
         NumPut(\"UShort\", 65535, b, 18)\n\
         NumPut(\"Char\", -5, b, 20)\n\
         NumPut(\"UChar\", 255, b, 21)\n\
         NumPut(\"Float\", 1.5, b, 22)\n\
         NumPut(\"Double\", 2.25, b, 26)\n\
         u := NumGet(b, 0, \"UInt\")\n\
         i := NumGet(b, 4, \"Int\")\n\
         i64 := NumGet(b, 8, \"Int64\")\n\
         s := NumGet(b, 16, \"Short\")\n\
         us := NumGet(b, 18, \"UShort\")\n\
         c := NumGet(b, 20, \"Char\")\n\
         uc := NumGet(b, 21, \"UChar\")\n\
         f := NumGet(b, 22, \"Float\")\n\
         d := NumGet(b, 26, \"Double\")\n\
         MsgBox \"%u% %i% %i64% %s% %us% %c% %uc% %f% %d%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4294967295 -2 -3 -4 65535 -5 255 1.5 2.25"]);
}

#[test]
fn a_pointer_is_a_number_of_the_size_of_a_pointer() {
    let report = run(
        "b := Buffer(16)\nNumPut(\"Ptr\", 1234, b, 0)\nNumPut(\"UPtr\", 4321, b, 8)\n\
         first := NumGet(b, 0, \"Ptr\")\nsecond := NumGet(b, 8, \"UPtr\")\nMsgBox \"%first% %second%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1234 4321"]);
}

#[test]
fn a_number_is_at_the_offset_it_is_written_at() {
    let report = run(
        "b := Buffer(8)\nNumPut(\"Int\", 42, b, 4)\nvalue := NumGet(b, 4, \"Int\")\n\
         empty := NumGet(b, 0, \"Int\")\nMsgBox \"%value% %empty%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["42 0"]);
}

#[test]
fn a_write_takes_several_numbers() {
    let report = run(
        "b := Buffer(8)\nNumPut(\"Int\", 1, \"Short\", 2, b)\n\
         first := NumGet(b, 0, \"Int\")\nsecond := NumGet(b, 4, \"Short\")\nMsgBox \"%first% %second%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 2"]);
}

#[test]
fn a_write_answers_the_address_to_the_right_of_the_last_number() {
    let report = run("b := Buffer(8)\naddress := NumPut(\"Int\", 42, b)\nright := address - b.Ptr");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("right"), Some(&Value::Int(4)));
}

#[test]
fn a_number_that_does_not_fit_in_the_buffer_fails() {
    let report = run("b := Buffer(2)\nNumPut(\"Int\", 42, b)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the bytes do not fit in the buffer."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_number_that_is_not_in_the_buffer_fails() {
    let report = run("b := Buffer(2)\nNumGet(b, 0, \"Int\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the bytes do not fit in the buffer."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_number_is_not_read_from_the_memory_of_an_address() {
    let report = run("NumGet(1234, \"Int\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this runtime does not read the memory of an address: the source of a number is a buffer.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_number_is_not_written_to_the_memory_of_an_address() {
    let report = run("NumPut(\"Int\", 42, 1234)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this runtime does not write to the memory of an address: the target of a number is a buffer.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_number_is_not_read_from_a_value_that_is_not_a_buffer() {
    let report = run("a := Array()\nNumGet(a, \"Int\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("this value is not a buffer."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_type_that_is_not_a_type_of_a_number_fails() {
    let report = run("b := Buffer(4)\nNumGet(b, \"Word\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_number_that_is_not_a_number_fails() {
    let report = run("b := Buffer(4)\nNumPut(\"Int\", \"word\", b)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #2 of NumPut requires a Number, but received a String.")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn num_get_wants_a_source_and_a_type() {
    let report = run("b := Buffer(4)\nNumGet(b)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too few parameters passed to function.\n     Specifically: NumGet")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn num_put_wants_a_type_a_number_and_a_target() {
    let report = run("NumPut(\"Int\", 42)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too few parameters passed to function.\n     Specifically: NumPut")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_number_of_the_arguments_of_the_constructor_of_a_buffer_is_checked() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the call is in the
    // `What` of the error, and the constructor is in the `Extra`.
    let report = run(
        "try {\n    x := Buffer(1, 2, 3)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too many parameters passed to function. | Buffer.Call | Buffer.Prototype.__New"]
    );
}

#[test]
fn the_size_of_a_buffer_is_a_number() {
    let report = run(
        "try {\n    x := Buffer(\"x\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of Buffer.Prototype.__New requires a Number, but received a String. | Buffer.Prototype.__New | x"]
    );
}
