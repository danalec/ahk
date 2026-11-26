//! The name that the v1 builds and the scope that it resolves.
//!
//! The v1 has two names that the script builds: the double deref of an
//! expression -- the `%nm%` of an `out := %nm%` -- and the target of a
//! legacy write -- the `%n% := 3` of a command. The source of the v1
//! (`script.cpp:9254-9267`) says of the second kind: "v1: Each *dynamic*
//! variable reference may resolve to a global if one exists."
//!
//! Every case of this file was measured on the binary of AutoHotkey
//! v1.1.37.02, and the probe of each one is written in the
//! comment of it. The probe went to
//! the probe of the measurement and
//! the answer of it to a file next to it.
//!
//! The cases run against a fake desktop: the scope of a variable is the
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
        button: &str,
    ) -> Result<String, String> {
        Ok(format!("{button} is down"))
    }
    fn mouse_up(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        Ok(format!("{button} is up"))
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

// -- the scope of the name that the double deref carries -------------------------

#[test]
fn the_double_deref_resolves_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // plain := "plain-global"
    // f1(nm) {
    //     Global out
    //     out := %nm%
    // }
    // f1("plain")
    // ```
    //
    // The answer of the v1.1.37.02: "01-plain-global=plain-global".
    // The name that the double deref carries is resolved in the scope of the
    // script: a global that the top level wrote is the answer, and the
    // function does not need to declare it.
    let report = run(
        "plain := \"plain-global\"\n\
         f1(nm) {\n\
         \x20   Global out\n\
         \x20   out := %nm%\n\
         }\n\
         f1(\"plain\")\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["plain-global"]);
}

#[test]
fn the_double_deref_resolves_the_super_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // Global super := "super-global"
    // f2(nm) {
    //     Global out
    //     out := %nm%
    // }
    // f2("super")
    // ```
    //
    // The answer of the v1.1.37.02: "02-super-global=super-global".
    // The super-global of the script is a global of the script too.
    let report = run(
        "Global super := \"super-global\"\n\
         f2(nm) {\n\
         \x20   Global out\n\
         \x20   out := %nm%\n\
         }\n\
         f2(\"super\")\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["super-global"]);
}

#[test]
fn the_local_of_the_function_wins_over_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // plain := "plain-global"
    // f3(nm) {
    //     Global out
    //     plain := "the-local"
    //     out := %nm%
    // }
    // f3("plain")
    // ```
    //
    // The answer of the v1.1.37.02: "03-local-wins=the-local".
    // The local of the function that runs is the answer, and the global of
    // the script keeps its value.
    let report = run(
        "plain := \"plain-global\"\n\
         f3(nm) {\n\
         \x20   Global out\n\
         \x20   plain := \"the-local\"\n\
         \x20   out := %nm%\n\
         }\n\
         f3(\"plain\")\n\
         text := out \" \" plain\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the-local plain-global"]);
}

#[test]
fn the_parameter_of_the_function_wins_over_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // plain := "plain-global"
    // f4(nm, plain) {
    //     Global out
    //     out := %nm%
    // }
    // f4("plain", "the-parameter")
    // ```
    //
    // The answer of the v1.1.37.02: "04-parameter-wins=the-parameter".
    let report = run(
        "plain := \"plain-global\"\n\
         f4(nm, plain) {\n\
         \x20   Global out\n\
         \x20   out := %nm%\n\
         }\n\
         f4(\"plain\", \"the-parameter\")\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the-parameter"]);
}

#[test]
fn the_name_that_the_double_deref_carries_and_that_is_not_there_is_blank() {
    // The probe:
    //
    // ```ahk
    // f5(nm) {
    //     Global out
    //     out := %nm%
    // }
    // f5("nothinghere")
    // ```
    //
    // The answer of the v1.1.37.02: "05-not-there=". The v1
    // creates the variable of that name and reads it, and the answer is the
    // empty text.
    let report = run(
        "f5(nm) {\n\
         \x20   Global out\n\
         \x20   out := %nm%\n\
         }\n\
         f5(\"nothinghere\")\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![""]);
}

#[test]
fn the_global_that_another_function_made_is_the_answer() {
    // The probe:
    //
    // ```ahk
    // f6() {
    //     global made
    //     made := "made-by-f6"
    // }
    // f6()
    // f7(nm) {
    //     Global out
    //     out := %nm%
    // }
    // f7("made")
    // ```
    //
    // The answer of the v1.1.37.02: "06-made-by-another=made-by-f6".
    let report = run(
        "f6() {\n\
         \x20   global made\n\
         \x20   made := \"made-by-f6\"\n\
         }\n\
         f6()\n\
         f7(nm) {\n\
         \x20   Global out\n\
         \x20   out := %nm%\n\
         }\n\
         f7(\"made\")\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["made-by-f6"]);
}

#[test]
fn the_double_deref_of_the_top_level_resolves_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // plain := "plain-global"
    // nm := "plain"
    // out := %nm%
    // ```
    //
    // The answer of the v1.1.37.02: "plain-global". The top
    // level of the script has no function around it, and the global of the
    // script is the answer there too.
    let report = run(
        "plain := \"plain-global\"\n\
         nm := \"plain\"\n\
         out := %nm%\n\
         FileAppend, %out%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["plain-global"]);
}

// -- the text that is between the percent signs ---------------------------------

#[test]
fn the_text_between_the_percent_signs_is_the_name_of_a_variable() {
    // The probe:
    //
    // ```ahk
    // num := "4"
    // pPen4 := "the-pPen4"
    // s := "[" pPen%num% "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[[the-pPen4]]". The text
    // that is between the percent signs is the name of a variable, and the
    // value of that variable is a part of the name: the "num" is the name of
    // a variable whose value is the "4", and the name that the pieces make
    // is the "pPen4".
    let report = run(
        "num := \"4\"\n\
         pPen4 := \"the-pPen4\"\n\
         s := \"[\" pPen%num% \"]\"\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[the-pPen4]"]);
}

#[test]
fn the_text_between_the_percent_signs_that_is_not_a_name_adds_nothing() {
    // The probe:
    //
    // ```ahk
    // pPen4 := "the-pPen4"
    // s := "[" pPen%4% "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[[]]". The "4" is not
    // the name of a variable, so it adds nothing to the name: the name that
    // the pieces make is the "pPen", and there is no variable of that name.
    //
    // The probe of the measurement is
    // the probe of the measurement.
    let report = run(
        "pPen4 := \"the-pPen4\"\n\
         s := \"[\" pPen%4% \"]\"\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_number_between_the_percent_signs_is_not_the_text_that_is_written() {
    // The probe:
    //
    // ```ahk
    // arr4 := "four"
    // s := "[" arr%4% "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[[]]". The name that the
    // pieces make is the "arr" and not the "arr4": the "4" is not the name of
    // a variable and it adds nothing to the name, even when the script has
    // the variable `arr4`.
    //
    // The probe of the measurement is
    // the probe of the measurement.
    let report = run(
        "arr4 := \"four\"\n\
         s := \"[\" arr%4% \"]\"\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_name_between_the_percent_signs_that_is_not_there_adds_nothing() {
    // The probe:
    //
    // ```ahk
    // s := "[" arr%not% "]"
    // ```
    //
    // The answer of the v1.1.37.02: "[[]]". The "not" is the
    // name of a variable that the script does not have, and the value of a
    // variable that is not there is the empty text.
    let report = run(
        "s := \"[\" arr%not% \"]\"\n\
         FileAppend, %s%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_name_that_the_script_builds_is_written_by_the_double_deref() {
    // The probe:
    //
    // ```ahk
    // i := 2
    // arr1 := "one"
    // arr2 := "two"
    // arr%i% := "written"
    // ```
    //
    // The answer of the v1.1.37.02: "written". The write of a
    // name that the script builds is the write of the variable that the name
    // is: the "i" is the name of a variable whose value is the "2", and the
    // variable that the name "arr2" is gets the new value.
    let report = run(
        "i := 2\n\
         arr1 := \"one\"\n\
         arr2 := \"two\"\n\
         arr%i% := \"written\"\n\
         FileAppend, %arr2%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["written"]);
}

// -- the name that the script builds and that is not legal ------------------------

#[test]
fn the_write_of_a_name_that_has_an_illegal_character_is_refused() {
    // The probe:
    //
    // ```ahk
    // name := "a b"
    // %name% := 1
    // ```
    //
    // The answer of the v1.1.37.02: the script stops with
    // "The following variable name contains an illegal character:" and the
    // name in quotes. The name of a variable may not have a space.
    let report = run("name := \"a b\"\n%name% := 1\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("The following variable name contains an illegal character:\n\"a b\"")
    );
}

#[test]
fn the_read_of_a_name_that_has_an_illegal_character_is_refused() {
    // The probe:
    //
    // ```ahk
    // name := "a b"
    // r := %name%
    // ```
    //
    // The answer of the v1.1.37.02: the script stops with
    // "The following variable name contains an illegal character:" and the
    // name in quotes.
    let report = run("name := \"a b\"\nr := %name%\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("The following variable name contains an illegal character:\n\"a b\"")
    );
}

#[test]
fn the_name_that_the_script_builds_may_have_the_letters_that_are_special_elsewhere() {
    // The probe:
    //
    // ```ahk
    // name := "a#b"
    // %name% := 1
    // r := %name%
    // ```
    //
    // The answer of the v1.1.37.02: "1". The `#`, the `$` and
    // the `@` are legal in the name of a variable, even though they are
    // special somewhere else. Measured one by one on the same day, with the
    // probe the probe of the measurement: the only letters that the v1 refuses are the
    // space, the tab, and the ones that are special in an expression.
    let report = run(
        "name := \"a#b\"\n\
         %name% := 1\n\
         r := %name%\n\
         FileAppend, %r%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

// -- the scope of the write of the name that the script builds ---------------------

#[test]
fn the_write_of_a_name_that_the_script_builds_goes_to_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // MixedCase := 1
    // f() {
    //     n := "MixedCase"
    //     %n% := 7
    // }
    // f()
    // FileAppend, %MixedCase%, *
    // ```
    //
    // The answer of the v1.1.37.02 (the probe of the measurement): "7".
    // The write of a name that the script builds is a write of the scope of
    // the script when the name is not a local of the function, and the name
    // of the script does not know the case of the letters.
    let report = run(
        "MixedCase := 1\n\
         f() {\n\
         n := \"MixedCase\"\n\
         %n% := 7\n\
         }\n\
         f()\n\
         FileAppend, %MixedCase%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn the_write_of_a_name_that_the_script_builds_goes_to_the_local_that_is_declared() {
    // The probe:
    //
    // ```ahk
    // d := 1
    // f() {
    //     local d
    //     n := "d"
    //     %n% := 9
    // }
    // f()
    // FileAppend, %d%, *
    // ```
    //
    // The answer of the v1.1.37.02 (the probe of the measurement): "1".
    // The name that the function declares local is the local one, and the
    // global of the script keeps its value.
    let report = run(
        "d := 1\n\
         f() {\n\
         local d\n\
         n := \"d\"\n\
         %n% := 9\n\
         }\n\
         f()\n\
         FileAppend, %d%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_write_of_a_name_that_the_script_builds_makes_the_global_that_is_not_there() {
    // The probe:
    //
    // ```ahk
    // f() {
    //     n := "b"
    //     %n% := 3
    // }
    // f()
    // FileAppend, %b%, *
    // ```
    //
    // The answer of the v1.1.37.02 (the probe of the measurement): "3".
    // The global of the script is made when the name that the script builds
    // is not there.
    let report = run(
        "f() {\n\
         n := \"b\"\n\
         %n% := 3\n\
         }\n\
         f()\n\
         FileAppend, %b%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}