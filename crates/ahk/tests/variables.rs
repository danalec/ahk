//! The built-in variables of the runtime, as a script sees them.
//!
//! The desktop is a fake: this is about what the language says, not about
//! what the machine does. The fake answers the machine, so the names that
//! the desktop carries are answered with the value of the fake, and the
//! names that the tree does not carry refuse with the reason of the name.

use ahk::desktop::{
    Clipboard, Desktop, Files, Folders, Hotkeys, Idle, Keyboard, Machine, Native, Pointer,
    Processes, Screen, Sound, Timers, Windows,
};
use ahk::runtime::{Report, Runtime};
use ahk::parse;
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
        app_data: Some("/home/danalec/.local/share".to_string()),
        app_data_common: Some("/usr/share".to_string()),
        desktop: Some("/home/danalec/Desktop".to_string()),
        desktop_common: None,
        start_menu: Some("/home/danalec/.local/share/applications".to_string()),
        start_menu_common: Some("/usr/share/applications".to_string()),
        programs: Some("/home/danalec/.local/share/applications".to_string()),
        programs_common: Some("/usr/share/applications".to_string()),
        startup: Some("/home/danalec/.config/autostart".to_string()),
        startup_common: Some("/etc/xdg/autostart".to_string()),
        my_documents: Some("/home/danalec/Documents".to_string()),
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
impl Sound for Fake {}

impl Machine for Fake {
    fn user_name(&mut self) -> Result<String, String> {
        Ok("danalec".to_string())
    }
    fn computer_name(&mut self) -> Result<String, String> {
        Ok("pod158c".to_string())
    }
    fn shell(&mut self) -> Result<String, String> {
        Ok("/usr/bin/zsh".to_string())
    }
    fn folders(&mut self) -> Result<Folders, String> {
        Ok(self.folders.clone())
    }
    fn os_version(&mut self) -> Result<String, String> {
        Ok("7.3.0-5-cachyos".to_string())
    }
    fn language(&mut self) -> Result<String, String> {
        Ok("en_US.UTF-8".to_string())
    }
    fn is_admin(&mut self) -> Result<bool, String> {
        Ok(false)
    }
    fn idle_times(&mut self) -> Result<Idle, String> {
        Ok(Idle { any: 1000, physical: 2000, keyboard: 3000, mouse: 4000 })
    }
}

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

/// The report of a script that ran, and the clipboard it left behind.
fn run_with_clipboard(script: &str, clipboard: &str) -> (Report, String) {
    let statements = parse(script).expect("parses");
    let mut desktop = Fake { clipboard: clipboard.to_string(), ..Fake::default() };
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
    assert!(report.output[0].parse::<u64>().is_ok(), "{:?}", report.output);
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
    assert_eq!(report.output, vec!["danalec, pod158c, /usr/bin/zsh, en_US.UTF-8"]);
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
    let report = run("MsgBox A_TimeIdle \", \" A_TimeIdlePhysical \", \" A_TimeIdleKeyboard \", \" A_TimeIdleMouse");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1000, 2000, 3000, 4000"]);
}

#[test]
fn a_folder_of_the_machine_answers_with_the_folder() {
    let report = run("MsgBox A_Desktop \", \" A_Temp \", \" A_MyDocuments");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/home/danalec/Desktop, /tmp, /home/danalec/Documents"]);
}

#[test]
fn a_folder_that_the_machine_does_not_have_refuses_with_the_reason_of_the_name() {
    // The machine of the test has no desktop for everybody, and the reason
    // is the one of the name.
    let report = run("MsgBox A_DesktopCommon");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the machine has no desktop for everybody: the desktop of the session is the one of the user.")
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
fn a_setting_that_the_runtime_does_not_keep_refuses_with_the_reason_of_the_settings() {
    let report = run("MsgBox A_ControlDelay");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the runtime keeps no such state yet."));
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn each_setting_that_the_runtime_does_not_keep_refuses_with_its_own_reason() {
    let report = run("MsgBox A_Cursor");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this runtime does not change the cursor of the pointer.")
    );

    let report = run("MsgBox A_RegView");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the registry is not carried by this runtime.")
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
        "A_ControlDelay",
        "A_Cursor",
        "A_RegView",
        "A_AhkPath",
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
    assert!(!report.variables.contains_key("A_Clipboard"), "{:?}", report.variables);
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
fn the_write_of_the_name_of_the_script_refuses_with_the_reason() {
    // The name of the script is the title of the messages, and the box has
    // no window to show a message in.
    let report = run("A_ScriptName := \"x\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this runtime has no title for the messages: the box has no window to show them in.")
    );
    assert_eq!(report.error_line, Some(1));
}
