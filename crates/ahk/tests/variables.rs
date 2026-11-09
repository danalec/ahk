//! The built-in variables of the runtime, as a script sees them.
//!
//! The desktop is a fake: this is about what the language says, not about
//! what the machine does. The fake answers the machine, so the names that
//! the desktop carries are answered with the value of the fake, and the
//! names that the tree does not carry refuse with the reason of the name.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Folders, Hotkeys, Idle, Keyboard, Machine, Native, Network,
    Pictures, Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::parse;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that holds a clipboard of its own and answers the machine.
#[derive(Debug)]
struct Fake {
    /// The text on the clipboard.
    clipboard: String,
    /// The folders of the machine.
    folders: Folders,
    /// The working directory of the run.
    working_dir: String,
}

/// The folders of the machine of the test.
///
/// They are the ones of the box, measured on 2026-10-29: the folders of
/// the user and the ones of everybody, and no desktop for everybody.
fn the_folders() -> Folders {
    Folders {
        app_data: Some("/home/<user>/.local/share".to_string()),
        app_data_common: Some("/usr/share".to_string()),
        desktop: Some("/home/<user>/Desktop".to_string()),
        desktop_common: None,
        start_menu: Some("/home/<user>/.local/share/applications".to_string()),
        start_menu_common: Some("/usr/share/applications".to_string()),
        programs: Some("/home/<user>/.local/share/applications".to_string()),
        programs_common: Some("/usr/share/applications".to_string()),
        startup: Some("/home/<user>/.config/autostart".to_string()),
        startup_common: Some("/etc/xdg/autostart".to_string()),
        my_documents: Some("/home/<user>/Documents".to_string()),
        temp: Some("/tmp".to_string()),
    }
}

impl Default for Fake {
    fn default() -> Self {
        Fake {
            clipboard: String::new(),
            folders: the_folders(),
            working_dir: "/work".to_string(),
        }
    }
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}

impl Machine for Fake {
    fn user_name(&mut self) -> Result<String, String> {
        Ok("<user>".to_string())
    }
    fn computer_name(&mut self) -> Result<String, String> {
        Ok("<host>".to_string())
    }
    fn shell(&mut self) -> Result<String, String> {
        Ok("/usr/bin/zsh".to_string())
    }
    fn folders(&mut self) -> Result<Folders, String> {
        Ok(self.folders.clone())
    }
    fn os_version(&mut self) -> Result<String, String> {
        Ok("<kernel>".to_string())
    }
    fn language(&mut self) -> Result<String, String> {
        Ok("en_US.UTF-8".to_string())
    }
    fn is_admin(&mut self) -> Result<bool, String> {
        Ok(false)
    }
    fn idle_times(&mut self) -> Result<Idle, String> {
        Ok(Idle {
            any: 1000,
            physical: 2000,
            keyboard: 3000,
            mouse: 4000,
        })
    }
}

impl Dialogs for Fake {}

impl Files for Fake {
    fn working_directory(&mut self) -> Result<String, String> {
        Ok(self.working_dir.clone())
    }

    fn initial_working_directory(&mut self) -> Result<String, String> {
        Ok("/work".to_string())
    }

    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        self.working_dir = directory.to_string();
        Ok(format!("the working directory is now {directory}"))
    }
}

impl Clipboard for Fake {
    fn clipboard_read(&mut self) -> Result<String, String> {
        Ok(self.clipboard.clone())
    }
    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        self.clipboard = text.to_string();
        Ok(text.to_string())
    }
}

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
    let mut desktop = Fake::default();
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

/// The line of the statement and the name of the function that runs.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
/// `A_LineNumber` is the line of the statement that runs, and the
/// `A_ThisFunc` is the name of the function that runs. The tree refused both
/// with "This global variable has not been assigned a value.".
#[test]
fn the_line_of_the_statement_and_the_name_of_the_function_that_runs() {
    let report = run("x := 1\nFileAppend A_LineNumber, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2".to_string()]);

    let report = run("f() {\n  FileAppend A_ThisFunc, \"*\"\n}\nf()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["f".to_string()]);

    let report = run("f() {\n  FileAppend A_LineNumber, \"*\"\n}\nf()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2".to_string()]);

    // Outside a function the name is empty.
    let report = run("FileAppend A_ThisFunc, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["".to_string()]);
}

/// The report of a script that ran, and the clipboard it left behind.
fn run_with_clipboard(script: &str, clipboard: &str) -> (Report, String) {
    let statements = parse(script).expect("parses");
    let mut desktop = Fake {
        clipboard: clipboard.to_string(),
        ..Fake::default()
    };
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    (report, desktop.clipboard)
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
    assert!(
        report.output[0].parse::<u64>().is_ok(),
        "{:?}",
        report.output
    );
}

#[test]
fn a_script_that_asks_for_the_date_is_told_the_date() {
    let report = run("MsgBox A_Now");
    assert!(report.ok, "{:?}", report.error);
    // The date of v2 is a number of fourteen digits.
    assert_eq!(report.output[0].len(), 14);
}

#[test]
fn a_name_of_the_machine_answers_with_the_value_of_the_desktop() {
    // The machine is the desktop's, and the desktop is asked.
    let report = run("MsgBox A_UserName \", \" A_ComputerName \", \" A_ComSpec \", \" A_Language");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["<user>, <host>, /usr/bin/zsh, en_US.UTF-8"]
    );
}

#[test]
fn a_script_that_asks_whether_it_is_elevated_is_told_a_number() {
    // The elevation of v2 is a number: one or zero.
    let report = run("MsgBox A_IsAdmin");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn an_idle_time_answers_with_the_milliseconds_of_the_desktop() {
    let report = run(
        "MsgBox A_TimeIdle \", \" A_TimeIdlePhysical \", \" A_TimeIdleKeyboard \", \" A_TimeIdleMouse",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1000, 2000, 3000, 4000"]);
}

#[test]
fn a_folder_of_the_machine_answers_with_the_folder() {
    let report = run("MsgBox A_Desktop \", \" A_Temp \", \" A_MyDocuments");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["/home/<user>/Desktop, /tmp, /home/<user>/Documents"]
    );
}

#[test]
fn a_folder_that_the_machine_does_not_have_refuses_with_the_reason_of_the_name() {
    // The machine of the test has no desktop for everybody, and the reason
    // is the one of the name.
    let report = run("MsgBox A_DesktopCommon");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "the machine has no desktop for everybody: the desktop of the session is the one of the user."
        )
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_working_directory_answers_with_the_directory_of_the_desktop() {
    let report = run("MsgBox A_WorkingDir");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/work"]);
}

#[test]
fn a_name_of_windows_refuses_with_the_reason_of_the_contract() {
    let report = run("MsgBox A_WinDir");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this is a name of Windows, and this desktop is not Windows.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_settings_that_the_runtime_keeps_answer_with_the_value_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode of the
    // matching of the titles is 2, the speed of it is "Fast", the hidden
    // windows are not seen, the hidden text is, and the delay of the
    // windowing functions is 100.
    let report = run(
        "MsgBox \"%A_TitleMatchMode% %A_TitleMatchModeSpeed% %A_DetectHiddenWindows% %A_DetectHiddenText% %A_WinDelay%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 Fast 0 1 100"]);
}

#[test]
fn the_settings_that_the_tree_keeps_answer_with_the_value_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay of the
    // controls is 20, the view of the registry is "Default", and the modes
    // of the caret, of the menu and of the tooltip are "Client".
    let report = run(
        "MsgBox \"%A_ControlDelay% %A_RegView% %A_CoordModeCaret% %A_CoordModeMenu% %A_CoordModeToolTip%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["20 Default Client Client Client"]);
}

#[test]
fn the_shape_of_the_cursor_is_not_read_by_this_runtime() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the shape of the
    // cursor belongs to the machine, and it changes with what the pointer
    // is over.
    let report = run("MsgBox A_Cursor");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the shape of the cursor of the machine is not read by this runtime.")
    );
}

#[test]
fn a_name_that_the_runtime_does_not_carry_refuses_with_the_reason_of_the_contract() {
    let report = run("MsgBox A_ScriptHwnd");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this runtime has no window of its own: it is a script that runs and ends.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_reason_of_a_name_is_the_same_in_the_read_and_in_the_refusal() {
    for name in [
        "A_WinDir",
        "A_ProgramFiles",
        "A_Cursor",
        "A_ScriptHwnd",
        "A_IconFile",
    ] {
        let refusal = match ahk::variables::refusal_of(name) {
            Some(refusal) => refusal,
            None => panic!("{name} answers with a value and has no reason"),
        };
        let report = run(&format!("MsgBox {name}"));
        assert!(!report.ok, "{name} answered with a value");
        assert_eq!(report.error.as_deref(), Some(refusal), "{name}");
    }
}

#[test]
fn a_script_that_asks_for_the_clipboard_is_told_the_text_of_the_desktop() {
    let (report, _) = run_with_clipboard("MsgBox A_Clipboard", "the text of the clipboard");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the text of the clipboard"]);
}

#[test]
fn a_clipboard_that_is_empty_is_an_empty_text() {
    let (report, _) = run_with_clipboard("MsgBox A_Clipboard", "");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![""]);
}

#[test]
fn a_script_that_writes_the_clipboard_puts_the_text_on_it() {
    let (report, clipboard) = run_with_clipboard("A_Clipboard := \"the new text\"", "");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(clipboard, "the new text");
}

#[test]
fn a_script_that_appends_to_the_clipboard_adds_to_what_is_there() {
    let (report, clipboard) = run_with_clipboard("A_Clipboard .= \" and more\"", "the first part");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(clipboard, "the first part and more");
}

#[test]
fn the_clipboard_is_not_a_variable_of_the_script() {
    let (report, _) = run_with_clipboard("A_Clipboard := \"the new text\"", "");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        !report.variables.contains_key("A_Clipboard"),
        "{:?}",
        report.variables
    );
}

#[test]
fn the_name_without_the_prefix_is_a_variable_of_the_script() {
    // The dialect gives every built-in variable the prefix `A_`, and the
    // binary of v2.0.19 warns about the name without it: `Clipboard` is a
    // global that was never assigned, and not the clipboard of the
    // session.
    let report = run("MsgBox Clipboard");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_name_of_the_elevation_without_the_prefix_is_a_variable_of_the_script() {
    // Measured on the binary of v2.0.19 on 2026-10-29: `x := A_IsAdmin`
    // answers 0, and `x := IsAdmin` answers the warning of a global that
    // was never assigned.
    let report = run("MsgBox IsAdmin");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
}

#[test]
fn a_variable_of_the_script_named_like_the_clipboard_is_kept_by_the_script() {
    // The name without the prefix is a variable of the script, and the
    // script keeps it: the clipboard of the desktop is not touched.
    let (report, clipboard) = run_with_clipboard(
        "Clipboard := \"the text of the script\"\nMsgBox Clipboard",
        "the text of the clipboard",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the text of the script"]);
    assert_eq!(clipboard, "the text of the clipboard");
}

#[test]
fn the_directory_where_the_run_started_is_the_one_of_the_run() {
    // The runtime asks the desktop when it is made, and it keeps it.
    let report = run("MsgBox A_InitialWorkingDir \", \" A_ScriptDir");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/work, /work"]);
}

#[test]
fn a_script_that_comes_from_the_standard_input_has_no_file_of_its_own() {
    // v2 says what the names of a script that comes from there are: the
    // name and the path are an asterisk.
    let report = run("MsgBox A_ScriptName \", \" A_ScriptFullPath");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["*, *"]);
}

#[test]
fn the_script_writes_the_working_directory() {
    // The directory is the one of the desktop, and the script changes it.
    let report = run("A_WorkingDir := \"/tmp\"\nMsgBox A_WorkingDir \", \" A_InitialWorkingDir");
    assert!(report.ok, "{:?}", report.error);
    // The working directory moved, and the one where the run started did
    // not.
    assert_eq!(report.output, vec!["/tmp, /work"]);
}

#[test]
fn the_script_writes_the_name_of_the_script() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect lets a
    // script write the name of the script, and the value that was written
    // is the value the name keeps.
    let report = run("A_ScriptName := \"x\"\nMsgBox A_ScriptName");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x"]);
}

#[test]
fn the_script_writes_the_information_of_an_event() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the information of
    // an event is a number, and a text holds the number it carries.
    let report = run(
        "A_EventInfo := \"20x\"\na := A_EventInfo\nA_EventInfo := 7\nb := A_EventInfo\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["20 7"]);
}

#[test]
fn the_script_writes_the_tip_of_the_icon() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the tip of the icon
    // takes any value, and it keeps it.
    let report = run("A_IconTip := \"x\"\nMsgBox A_IconTip");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x"]);
}

#[test]
fn the_script_writes_the_hidden_of_the_icon() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the icon is hidden
    // or it is not, and the flag takes the truth of the value.
    let report = run(
        "A_IconHidden := \"x\"\na := A_IconHidden\nA_IconHidden := false\nb := A_IconHidden\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn the_script_writes_the_last_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the last error of
    // the operating system is a number of thirty-two bits, and it wraps
    // around.
    let report = run(
        "A_LastError := -1\na := A_LastError\nA_LastError := \"20x\"\nb := A_LastError\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4294967295 20"]);
}

#[test]
fn the_script_writes_the_interval_of_the_hotkeys() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the interval is a
    // number, and a text that holds one is that number.
    let report = run(
        "A_HotkeyInterval := \"2\"\na := A_HotkeyInterval\nA_HotkeyInterval := 7\nb := A_HotkeyInterval\nMsgBox \"%a% %b%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 7"]);
}

#[test]
fn a_text_that_holds_no_number_is_refused_by_the_interval_of_the_hotkeys() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the interval wants a
    // number, and the kind of the value that arrived is named.
    let report = run(
        "try {\n    A_HotkeyInterval := \"text\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Expected a Number but got a String. | TypeError"]
    );
}

#[test]
fn the_script_writes_the_number_of_the_hotkeys() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
    // hotkeys is a number, and the smallest one that it takes is one.
    let report = run("A_MaxHotkeysPerInterval := 7\na := A_MaxHotkeysPerInterval\nMsgBox \"%a%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn a_number_of_hotkeys_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a number that is out
    // of the range of the name is a value that is not valid.
    let report = run(
        "try {\n    A_MaxHotkeysPerInterval := 0\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid value. | ValueError"]);
}

#[test]
fn the_timeout_of_the_change_of_a_hotkey_is_fifty() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the timeout of the
    // change of a hotkey is fifty until a script writes another one. The
    // name is `A_HotkeyModifierTimeout`: the tree had measured a name that
    // does not exist.
    let report = run("MsgBox A_HotkeyModifierTimeout");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["50"]);
}

#[test]
fn the_timeout_of_the_change_of_a_hotkey_keeps_the_value_that_was_written() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a script that writes
    // the timeout of the change of a hotkey gives it the value it wrote.
    let report = run("A_HotkeyModifierTimeout := 5\nMsgBox A_HotkeyModifierTimeout");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5"]);
}

#[test]
fn the_name_that_is_not_a_built_in_variable_has_no_value() {
    // Measured on the binary of v2.0.19 on 2026-11-01: `A_HotkeyModifyTimeout`
    // is not the name of a built-in variable of the dialect -- the name is
    // `A_HotkeyModifierTimeout` -- and a variable that has no value is the
    // failure that says so.
    let report = run("MsgBox A_HotkeyModifyTimeout");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn the_index_of_the_turn_is_the_number_of_it() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the index of the
    // turn is the number of it, and every loop counts: the counted one,
    // the one without a count and the while.
    let report = run(
        "Loop 3 {\n    MsgBox A_Index\n}\nn := 0\nwhile n < 2 {\n    MsgBox A_Index\n    n += 1\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "2", "3", "1", "2"]);
}

#[test]
fn the_index_that_the_body_writes_changes_the_course_of_the_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the index of a
    // counted loop is the value of `A_Index`, and the body may write it:
    // the loop goes on from the value that was written.
    let report = run("Loop 5 {\n    A_Index := 9\n    MsgBox A_Index\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["9"]);
}

#[test]
fn the_index_of_the_turn_goes_back_when_the_loop_ends() {
    // Measured on the binary of v2.0.19 on 2026-11-01: every loop saves the
    // value of the index when it starts and gives it back when it ends.
    let report = run("Loop 2 {\n}\nMsgBox A_Index");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_index_of_the_outer_loop_is_not_disturbed_by_the_inner_one() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a loop that is
    // inside another one does not disturb the index of the outer one.
    let report = run("Loop 2 {\n    Loop 2 {\n        MsgBox A_Index\n    }\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "2", "1", "2"]);
}

#[test]
fn the_index_of_the_turn_is_zero_outside_a_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the index of the
    // turn is zero before the first loop runs, and the compound operator
    // reads the value that is there.
    let report = run("A_Index += 1\nMsgBox A_Index");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_built_in_variable_that_the_dialect_does_not_let_a_script_write_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect refuses
    // the name when the script writes it, and it names the name as the
    // script spelled it. The refusal is at the load: the script does not
    // run at all.
    let error = parse("A_AhkVersion := \"x\"").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be used as an output variable.\n     Specifically: A_AhkVersion"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn the_compound_operator_says_that_the_variable_cannot_be_assigned_a_value() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the compound
    // operator has its own reason, and it says that the variable cannot
    // be assigned a value.
    let error = parse("A_ScriptDir += 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be assigned a value.\n     Specifically: A_ScriptDir"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn the_refusal_of_a_built_in_variable_is_the_one_of_every_scope() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the refusal is at
    // the load, and it is the one of every scope: a local variable may
    // not carry the name of a built-in variable that the dialect does not
    // let a script write.
    let error = parse("f() {\n    A_ScriptDir := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be used as an output variable.\n     Specifically: A_ScriptDir"
    );
    assert_eq!(error.line, 2);
}

#[test]
fn the_shape_of_the_cursor_may_not_be_written() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the shape of the
    // cursor is a setting of the run, and the v2 refuses to write it all
    // the same.
    let error = parse("A_Cursor := \"Arrow\"").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be used as an output variable.\n     Specifically: A_Cursor"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn the_names_that_the_runtime_owns_may_not_be_written() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the names of the
    // loop and of the line are names that the v2 refuses to write, and
    // the refusal is at the load.
    let error = parse("A_LoopField := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be used as an output variable.\n     Specifically: A_LoopField"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn the_index_of_the_loop_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the index of the
    // loop is the one name that the runtime owns and that a script may
    // write.
    let report = run("A_Index := 5\nMsgBox A_Index");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5"]);
}

#[test]
fn the_body_of_a_class_may_hold_the_name_of_a_built_in_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the body of a class
    // is the one place where the name is a property and not a variable,
    // and the dialect lets it there.
    let report = run("class C {\n    A_ScriptDir := 1\n}");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_parameter_may_not_carry_the_name_of_a_built_in_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the declaration of
    // a parameter conflicts with a built-in variable, and the name is
    // named as the script spelled it.
    let error = parse("f(A_ScriptDir) {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This parameter declaration conflicts with an existing built-in variable.\n     Specifically: A_ScriptDir"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn a_static_declaration_may_not_carry_the_name_of_a_built_in_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a static
    // declaration may not carry the name of a built-in variable, and the
    // declaration says that it conflicts with it.
    let error = parse("f() {\n    static A_ScriptName := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This static variable declaration conflicts with an existing built-in variable.\n     Specifically: A_ScriptName"
    );
    assert_eq!(error.line, 2);
}

#[test]
fn a_global_declaration_may_carry_the_name_of_a_built_in_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the global names
    // the built-in variable of the dialect and not a variable of the
    // script, and the dialect lets it.
    let report = run("global A_ScriptDir");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_variable_of_a_for_may_not_carry_the_name_of_a_read_only_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the variable of a
    // for is an output variable, and the name of a built-in variable that
    // the dialect does not let a script write is refused.
    let error = parse("for A_ScriptDir in [1] {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This built-in variable cannot be used as an output variable.\n     Specifically: A_ScriptDir"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn a_variable_that_has_no_value_is_an_unset_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a variable that has
    // no value is an `UnsetError`.
    let report =
        run("try {\n    x += 5\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["This global variable has not been assigned a value. | UnsetError"]
    );
}

#[test]
fn the_compound_operator_checks_the_value_that_is_written_first() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect converts
    // the value that is written before it reads the one that is there, and
    // the kind it names is the kind of the value that was written.
    let report = run(
        "try {\n    x += \"text\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Expected a Number but got a String. | TypeError"]
    );
}
