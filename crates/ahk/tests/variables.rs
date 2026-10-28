//! The built-in variables of the runtime, as a script sees them.
//!
//! The desktop is a fake: this is about what the language says, not about
//! what the machine does.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers,
    Windows,
};
use ahk::runtime::{Report, Runtime};
use ahk::parse;
use std::time::Duration;

/// A desktop that answers nothing.
#[derive(Debug)]
struct Fake;

impl Desktop for Fake {}

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
        Ok(String::new())
    }
    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        Ok((1920, 1080))
    }
}

impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

impl Native for Fake {}

/// The report of a script that ran.
fn run(script: &str) -> Report {
    let statements = parse(script).expect("parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

#[test]
fn a_script_that_asks_for_the_version_is_told_the_version_of_the_dialect() {
    let report = run("MsgBox A_AhkVersion");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2.0.19"]);
}

#[test]
fn a_script_that_asks_for_the_size_of_the_screen_is_told_it() {
    let report = run("MsgBox A_ScreenWidth \", \" A_ScreenHeight");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1920, 1080"]);
}

#[test]
fn a_script_that_asks_for_the_space_is_told_a_space() {
    let report = run("MsgBox A_Space");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![" "]);
}

#[test]
fn a_script_that_asks_for_the_tick_count_is_told_a_number_of_milliseconds() {
    let report = run("MsgBox A_TickCount");
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output[0].parse::<u64>().is_ok(), "{:?}", report.output);
}

#[test]
fn a_script_that_asks_for_the_date_fails_with_the_reason_of_the_clock() {
    let report = run("MsgBox A_Now");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the clock of this runtime counts from an arbitrary origin and does not know the date.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_script_that_asks_for_the_user_name_fails_with_the_reason_of_the_machine() {
    let report = run("MsgBox A_UserName");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the machine and the session are not carried by this runtime.")
    );
    assert_eq!(report.error_line, Some(1));
}
