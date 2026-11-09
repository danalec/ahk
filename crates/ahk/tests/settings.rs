//! The settings of the script, as a script sees them.
//!
//! The settings are the state of the runtime, and the state is one for the
//! whole crate: the cases of this file are the only ones that touch it, and
//! each one puts back what it changed.
//!
//! The desktop is a fake: this is about what the language says, not about
//! what the machine does.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that answers nothing: the settings are the language's.
#[derive(Debug)]
struct Fake;

impl Desktop for Fake {}
impl Clipboard for Fake {}
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
        Ok(String::new())
    }
}
impl Sound for Fake {}
impl Timers for Fake {}
impl Windows for Fake {}

/// The report of a script that ran.
fn run(script: &str) -> Report {
    let statements = parse(script).expect("parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

#[test]
fn the_delay_of_the_controls_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay takes the
    // number of the value, and a text that holds no number is zero. The
    // compound operator reads the delay that is there before it writes it,
    // and the command of the delay answers the value that is there too.
    let report = run(
        "A_ControlDelay := 50\na := A_ControlDelay\nb := SetControlDelay(100)\nA_ControlDelay := \"x\"\nc := A_ControlDelay\nA_ControlDelay := \"20x\"\nd := A_ControlDelay\nA_ControlDelay += 5\ne := A_ControlDelay\nA_ControlDelay := 20\nMsgBox \"%a% %b% %c% %d% %e%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["50 50 0 20 25"]);
}

#[test]
fn the_mode_of_the_coordinates_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode takes the
    // name of it, and the name is the one of v2.
    let report = run(
        "A_CoordModeMouse := \"Screen\"\na := A_CoordModeMouse\nA_CoordModeMouse := \"screen\"\nb := A_CoordModeMouse\nA_CoordModeMouse := \"Client\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Screen Screen"]);
}

#[test]
fn the_view_of_the_registry_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the view takes the
    // number of it or its name, and the answer is always a text.
    let report = run(
        "A_RegView := 32\na := A_RegView\nA_RegView := \"Default\"\nb := A_RegView\nA_RegView := \"Default\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["32 Default"]);
}

#[test]
fn the_mode_of_the_send_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the method of the
    // send takes its name, and the name is the one of v2.
    let report = run(
        "A_SendMode := \"event\"\na := A_SendMode\nA_SendMode := \"InputThenPlay\"\nb := A_SendMode\nA_SendMode := \"Input\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Event InputThenPlay"]);
}

#[test]
fn the_level_of_the_send_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the level is a
    // number, and a text that holds one is that number.
    let report = run(
        "A_SendLevel := \"2\"\na := A_SendLevel\nA_SendLevel := 3\nb := A_SendLevel\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 3"]);
}

#[test]
fn the_encoding_of_the_files_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the encoding takes
    // its name, and the name is kept the way the tree spells it.
    let report = run(
        "A_FileEncoding := \"utf-8\"\na := A_FileEncoding\nA_FileEncoding := \"UTF-8-RAW\"\nb := A_FileEncoding\nA_FileEncoding := \"UTF-8\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["UTF-8 UTF-8-RAW"]);
}

#[test]
fn the_key_of_the_menu_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the key takes the
    // virtual key of it, and the scan code is left at zero.
    let report = run(
        "A_MenuMaskKey := \"A\"\na := A_MenuMaskKey\nA_MenuMaskKey := \"Esc\"\nb := A_MenuMaskKey\nA_MenuMaskKey := \"Esc\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["vk41sc000 vk1Bsc000"]);
}

#[test]
fn the_mode_of_the_matching_of_the_titles_is_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode takes the
    // name of it, and the speed takes its own name.
    let report = run(
        "A_TitleMatchMode := \"RegEx\"\na := A_TitleMatchMode\nA_TitleMatchModeSpeed := \"slow\"\nb := A_TitleMatchModeSpeed\nA_TitleMatchMode := 2\nA_TitleMatchModeSpeed := \"Fast\"\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["RegEx Slow"]);
}

#[test]
fn the_flags_are_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a flag takes the
    // truth of the value, and a text that is not empty and not "0" is true.
    let report = run(
        "A_DetectHiddenWindows := \"x\"\na := A_DetectHiddenWindows\nA_DetectHiddenWindows := \"0\"\nb := A_DetectHiddenWindows\nA_ListLines := 0\nc := A_ListLines\nA_AllowMainWindow := 0\nd := A_AllowMainWindow\nA_StoreCapsLockMode := 0\ne := A_StoreCapsLockMode\nA_DetectHiddenWindows := 0\nA_ListLines := 1\nA_AllowMainWindow := 1\nA_StoreCapsLockMode := 1\nMsgBox \"%a% %b% %c% %d% %e%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0 0 0 0"]);
}

#[test]
fn the_delays_are_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delays are
    // numbers, and a negative one is a number that the delay of the
    // windowing functions takes.
    let report = run(
        "A_WinDelay := -1\na := A_WinDelay\nA_MouseDelay := 10\nb := A_MouseDelay\nA_KeyDelay := 5\nc := A_KeyDelay\nA_DefaultMouseSpeed := 5\nd := A_DefaultMouseSpeed\nA_WinDelay := 100\nA_MouseDelay := 10\nA_KeyDelay := 10\nA_DefaultMouseSpeed := 10\nMsgBox \"%a% %b% %c% %d%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["-1 10 5 5"]);
}

#[test]
fn a_setting_that_has_a_fixed_set_of_values_refuses_the_others() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is not
    // one of the set is a value that is not valid, and the failure is of
    // the class `ValueError`.
    let report = run(
        "try {\n    A_CoordModeMouse := 5\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid value. | ValueError"]);
}

#[test]
fn a_level_of_the_send_that_is_not_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the level wants a
    // number, and the kind of the value that arrived is named.
    let report = run(
        "try {\n    A_SendLevel := \"x\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Expected a Number but got a String. | TypeError"]
    );
}
