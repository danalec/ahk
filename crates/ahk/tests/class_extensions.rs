//! The members the KeySharp extension adds to the `Array`, against a fake
//! desktop.
//!
//! The rules are the ones the documentation of KeySharp writes
//! (`Keysharp/docs/reference.md`, the members of the classes of AutoHotkey),
//! and this file pins the ones the tree carries: the searches compare the
//! actual underlying values, the callbacks take the place they are given,
//! and a start of zero throws.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing.
#[derive(Debug, Default)]
struct Fake;

impl Clipboard for Fake {}
impl Desktop for Fake {}
impl Dialogs for Fake {}

impl Hotkeys for Fake {
    fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
        // The Fake watches the keyboard: a hotkey of the script is
        // registered, and the condition of it is kept.
        Ok(name.to_string())
    }
}
impl Keyboard for Fake {
    fn keyboard_layout_name(&mut self) -> Result<String, String> {
        Ok("00000409:Test".to_string())
    }
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}
impl Machine for Fake {
    fn executable(&mut self) -> Result<String, String> {
        Ok("/usr/bin/ahk".to_string())
    }
    fn the_current_thread_id(&mut self) -> Result<i64, String> {
        Ok(4242)
    }
}
impl Native for Fake {}
impl Network for Fake {
    fn send_mail(&mut self, _message: &ahk::desktop::MailMessage) -> Result<(), String> {
        Ok(())
    }
}
impl Pictures for Fake {}
impl Files for Fake {
    fn full_path(&mut self, path: &str) -> Result<String, String> {
        Ok(format!("/work/{path}"))
    }
    fn create_temp_file(&mut self) -> Result<String, String> {
        Ok("/tmp/ahk-test.tmp".to_string())
    }
}
impl Pointer for Fake {
    fn clip_cursor(&mut self, _rect: Option<(i64, i64, i64, i64)>) -> Result<(), String> {
        Ok(())
    }
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}
impl Processes for Fake {
    fn launch_with_stdin(
        &mut self,
        _program: &str,
        _arguments: &[String],
        _standard_input: &str,
        _working_dir: &str,
    ) -> Result<u32, String> {
        Ok(1234)
    }
    fn run_with_stdin(
        &mut self,
        _program: &str,
        _arguments: &[String],
        _standard_input: &str,
        _working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        Ok((1234, 0, String::new(), String::new()))
    }
    fn the_state_of_a_process(
        &mut self,
        pid: u32,
    ) -> Result<Option<ahk::desktop::ProcessState>, String> {
        // The process of the pid 1234 is the one that runs, and the one of
        // the pid 1 is the one that has exited.
        Ok(Some(ahk::desktop::ProcessState {
            has_exited: pid != 1234,
            exit_code: if pid == 1234 { None } else { Some(3) },
            exit_time: if pid == 1234 {
                None
            } else {
                Some("20261111120000".to_string())
            },
        }))
    }
    fn kill_a_process(&mut self, _pid: u32) -> Result<(), String> {
        Ok(())
    }
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
        Ok("#000000".into())
    }
}
impl Sound for Fake {}
impl Timers for Fake {}
impl Windows for Fake {
    fn the_window_at(&mut self, x: i64, y: i64) -> Result<Option<i64>, String> {
        // The window of the point (5, 5) is the plain one, and the one of
        // (9, 9) is the window that is above the others; anywhere else
        // there is no window.
        match (x, y) {
            (5, 5) => Ok(Some(2)),
            (9, 9) => Ok(Some(1)),
            _ => Ok(None),
        }
    }
    fn windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        Ok(vec![
            ahk::desktop::WindowInfo {
                id: 1,
                title: "Above".to_string(),
                class: "Window".to_string(),
                active: false,
            },
            ahk::desktop::WindowInfo {
                id: 2,
                title: "Plain".to_string(),
                class: "Window".to_string(),
                active: false,
            },
            ahk::desktop::WindowInfo {
                id: 3,
                title: "Unknown".to_string(),
                class: "Window".to_string(),
                active: false,
            },
        ])
    }
    fn always_on_top(&mut self, window: &ahk::desktop::WindowInfo) -> Result<Option<bool>, String> {
        match window.id {
            1 => Ok(Some(true)),
            2 => Ok(Some(false)),
            _ => Ok(None),
        }
    }
    fn set_always_on_top(
        &mut self,
        _window: &ahk::desktop::WindowInfo,
        _on: bool,
    ) -> Result<(), String> {
        Ok(())
    }
    fn window_enabled(&mut self, window: &ahk::desktop::WindowInfo) -> Result<Option<bool>, String> {
        match window.id {
            3 => Ok(None),
            _ => Ok(Some(true)),
        }
    }
    fn control_enabled(
        &mut self,
        _window: &ahk::desktop::WindowInfo,
    ) -> Result<Option<bool>, String> {
        Ok(Some(false))
    }
    fn window_capabilities(&mut self) -> Vec<String> {
        vec![
            "AlwaysOnTopGet".to_string(),
            "AlwaysOnTopSet".to_string(),
            "WindowEnabledGet".to_string(),
            "ControlEnabledGet".to_string(),
        ]
    }
}

#[test]
fn the_window_state_answers_what_the_machine_says() {
    // The machine that knows says one or zero, and the machine that cannot
    // say is refused with the reason.
    let report = run_ks(concat!(
        "MsgBox WinGetAlwaysOnTop(\"Above\")\n",
        "MsgBox WinGetAlwaysOnTop(\"Plain\")\n",
        "MsgBox WinGetEnabled(\"Above\")\n",
        "MsgBox WinGetCapability(\"AlwaysOnTopGet\")\n",
        "MsgBox WinGetCapability(\"WindowList\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "0");
    assert_eq!(line(&report, 2), "1");
    assert_eq!(line(&report, 3), "1");
    // The capability that the desktop does not have is zero, and not a
    // failure.
    assert_eq!(line(&report, 4), "0");
}

#[test]
fn the_machine_that_cannot_say_is_refused_with_the_reason() {
    // The tree does not answer zero for a state it does not know.
    let report = run_ks("MsgBox WinGetAlwaysOnTop(\"Unknown\")\n");
    assert!(!report.ok);
    let error = report.error.unwrap();
    assert!(
        error.contains("cannot say whether the window is above the others"),
        "{}",
        error
    );
}

#[test]
fn the_setter_of_the_state_takes_the_numbers_of_the_v2() {
    // `1` enables, `0` disables and `-1` toggles. The shape of the v2.0.19
    // says that the first parameter is a number, and the words Toggle, On
    // and Off are refused by it before the handler is called.
    let report = run_ks(concat!(
        "WinSetAlwaysOnTop(1, \"Plain\")\n",
        "WinSetAlwaysOnTop(0, \"Plain\")\n",
        "WinSetAlwaysOnTop(-1, \"Plain\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_state_that_is_not_a_number_is_refused_by_the_shape_of_the_v2() {
    // The shape is the one the binary of the v2.0.19 was measured with, and
    // it refuses the words before the handler of the command is called.
    let report = run_ks("WinSetAlwaysOnTop(\"Toggle\", \"Plain\")\n");
    assert!(!report.ok);
    let error = report.error.unwrap();
    assert!(error.contains("requires a Number"), "{}", error);
}

#[test]
fn the_capability_that_the_tree_does_not_know_is_refused() {
    let report = run_ks("MsgBox WinGetCapability(\"Nonsense\")\n");
    assert!(!report.ok);
    let error = report.error.unwrap();
    assert!(
        error.contains("is not one of the capabilities of the tree"),
        "{}",
        error
    );
}

#[test]
fn the_names_of_the_window_state_are_not_in_the_v2() {
    // The surface of the v2.0.19 stays pure: the names are behind the gate
    // of the ks dialect.
    let mut desktop = Fake;
    let statements = ahk::parse("MsgBox WinGetCapability(\"AlwaysOnTopGet\")\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

/// A script in the dialect of KeySharp, and the report of it.
fn run_ks(source: &str) -> Report {
    let mut desktop = Fake;
    let statements = ahk::parse_with(source, dialect::ks()).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::ks())
        .run(&statements)
}

/// The text an output line of the report holds.
fn line(report: &Report, index: usize) -> String {
    report.output[index].clone()
}

#[test]
fn a_map_answers_the_array_of_the_answers_of_the_callback() {
    // The example of the documentation of KeySharp: `[10, 20, 30]` mapped
    // with `(x, i) => x * i` answers `[10, 40, 90]`.
    let report = run_ks(
        "arr := [10, 20, 30]\nm := arr.Map((x, i) => x * i)\nMsgBox m.ToString()\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[10, 40, 90]");
}

#[test]
fn a_contains_and_an_indexof_compare_the_underlying_values() {
    // The documentation says the comparisons compare the actual underlying
    // values, so "1" is not 1 -- a search of an array is not the `=` of an
    // `if`.
    let report = run_ks(
        concat!(
            "a := [1, \"1\", 2]\n",
            "MsgBox a.Contains(1)\n",
            "MsgBox a.IndexOf(\"1\")\n",
            "MsgBox a.IndexOf(9)\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "2");
    assert_eq!(line(&report, 2), "0");
}

#[test]
fn a_negative_start_walks_from_the_end_toward_the_beginning() {
    let report = run_ks("MsgBox [10, 20, 10].IndexOf(10, -1)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "3");
}

#[test]
fn a_start_of_zero_throws() {
    let report = run_ks(
        concat!(
            "a := [10, 20]\n",
            "try {\n",
            "    a.IndexOf(10, 0)\n",
            "} catch as e {\n",
            "    MsgBox e.Message\n",
            "}\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        line(&report, 0),
        "Parameter #2 of Array.Prototype.IndexOf is invalid."
    );
}

#[test]
fn a_remove_takes_the_first_match_and_answers_whether_it_did() {
    let report = run_ks(
        concat!(
            "a := [10, 20, 10]\n",
            "r := a.Remove(10)\n",
            "MsgBox r\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "[20, 10]");
}

#[test]
fn a_remove_without_a_value_takes_the_first_place_that_has_none() {
    let report = run_ks(
        concat!(
            "a := [1, 2, 3]\n",
            "a.Delete(2)\n",
            "r := a.Remove()\n",
            "MsgBox r\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    // The place that went is the hole itself: the array keeps the first and
    // the third, and there is no place without a value any more.
    assert_eq!(line(&report, 1), "[1, 3]");
}

#[test]
fn a_sort_keeps_the_order_of_the_equal_ones() {
    // The documentation says only the sign of the answer counts, and equal
    // elements keep their order.
    let report = run_ks(
        concat!(
            "a := [\"b\", \"a\", \"c\"]\n",
            "a.Sort((one, two) => 0)\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[\"b\", \"a\", \"c\"]");
}

#[test]
fn a_tostring_quotes_the_texts_and_names_the_absent_places() {
    let report = run_ks(
        concat!(
            "a := [1, \"two\"]\n",
            "a.Delete(1)\n",
            "MsgBox a.ToString()\n",
        ),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[unset, \"two\"]");
}

#[test]
fn a_join_puts_the_separator_between_the_elements() {
    let report = run_ks("MsgBox [10, 20, 30].Join(\"-\")\nMsgBox [10, 20].Join()\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "10-20-30");
    // The separator the documentation names as the default is the comma.
    assert_eq!(line(&report, 1), "10,20");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_members() {
    // The surface of the v2.0.19 stays pure: the same script that runs in
    // the dialect of KeySharp is refused in the v2, and the refusal is the
    // one of a member that is not there.
    let mut desktop = Fake;
    let statements = ahk::parse("x := [10, 20]\nx.Contains(10)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert!(report
        .error
        .as_deref()
        .is_some_and(|error| error.contains("has no method named \"Contains\"")));
}

#[test]
fn a_map_tostring_writes_the_pairs_the_way_the_documentation_does() {
    // The documentation of KeySharp writes `[key: value, ...]`, and says
    // the keys and the values are formatted the way `Array.ToString()`
    // formats its elements.
    let report = run_ks(concat!(
        "m := Map(\"a\", 1, \"b\", \"x\")\n",
        "MsgBox m.ToString()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "[\"a\": 1, \"b\": \"x\"]");
}

#[test]
fn the_metadata_of_a_function_names_its_kind_and_its_parameters() {
    // The documentation of KeySharp says `IsClosure`, `IsMethod` and
    // `Params` expose the metadata of the function.
    let report = run_ks(concat!(
        "F(a, b) {\n",
        "    return a + b\n",
        "}\n",
        "f := F\n",
        "MsgBox f.IsClosure()\n",
        "MsgBox f.IsMethod()\n",
        "MsgBox f.Params().ToString()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "0");
    assert_eq!(line(&report, 2), "[\"a\", \"b\"]");
}

#[test]
fn a_closure_is_known_as_one() {
    let report = run_ks(concat!(
        "g := (x) => x + 1\n",
        "MsgBox g.IsClosure()\n",
        "MsgBox g.Params().ToString()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "[\"x\"]");
}

#[test]
fn a_flush_answers_the_empty_text_the_writes_do() {
    // The writes of a file of this runtime reach the disk as the
    // statements write them, so the flush has nothing to do: the answer is
    // the empty text, and it is a value and not a refusal.
    let report = run_ks(concat!(
        "out := FileOpen(\"*\", \"w\")\n",
        "MsgBox out.Flush()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "");
}

#[test]
fn the_two_clocks_of_the_ks_write_the_milliseconds() {
    // The documentation of KeySharp writes them as
    // "YYYYMMDDHH24MISS.ff" -- the number of v2 with the two digits of the
    // fraction after it.
    let report = run_ks("MsgBox A_NowMs\nMsgBox A_NowUTCMs\n");
    assert!(report.ok, "{:?}", report.error);
    // The number of v2 is fourteen digits, and the fraction is a dot and
    // two of them.
    assert_eq!(report.output[0].len(), 14 + 3);
    assert!(report.output[0].contains('.'));
    assert_eq!(report.output[1].len(), 14 + 3);
}

#[test]
fn the_separator_and_the_newline_are_the_ones_the_platform_writes() {
    // The documentation says the separator is `\` on Windows and `/`
    // elsewhere.
    let report = run_ks("MsgBox A_DirSeparator\nMsgBox A_NewLine\n");
    assert!(report.ok, "{:?}", report.error);
    let expected = if cfg!(windows) { "\\" } else { "/" };
    assert_eq!(line(&report, 0), expected);
    assert_eq!(line(&report, 1), "\n");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_accessors() {
    // The surface of the v2.0.19 stays pure: `A_NowMs` is not a variable
    // of the v2, and the tree says so.
    let mut desktop = Fake;
    let statements = ahk::parse("MsgBox A_NowMs\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_arcs_and_the_hyperbolics_of_the_ks_answer_the_numbers_of_the_documentation() {
    // Measured on the binary of KeySharp on 2026-11-09: `ATan2` of the
    // probe answers the float 0.4636..., and `Sinh(0)` is zero.
    let report = run_ks(concat!(
        "MsgBox Round(ATan2(0.5, 1), 4)\n",
        "MsgBox Sinh(0)\n",
        "MsgBox Cosh(0)\n",
        "MsgBox Tanh(0)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0.4636");
    assert_eq!(line(&report, 1), "0.0");
    assert_eq!(line(&report, 2), "1.0");
    assert_eq!(line(&report, 3), "0.0");
}

#[test]
fn a_replace_of_the_line_endings_takes_the_seven_of_them() {
    // The documentation lists the seven: the two of Windows, the two of
    // the classic Mac and the Unix, the form feed, the next line of
    // Unicode and the two separators of Unicode.
    let report = run_ks(concat!(
        "text := \"a`r`nb`nc`rd`ne\u{000C}f\u{0085}g\u{2028}h\u{2029}i\"\n",        "MsgBox ReplaceLineEndings(text, \"-\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "a-b-c-d-e-f-g-h-i");
}

#[test]
fn a_seed_of_the_random_is_answered_with_nothing() {
    let report = run_ks("r := RandomSeed(42)\nMsgBox r\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "");
}

#[test]
fn the_paths_of_the_files_are_answered_by_the_desktop() {
    let report = run_ks(concat!(
        "MsgBox FileFullPath(\"x.ahk\")\n",
        "MsgBox FileCreateTemp()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "/work/x.ahk");
    assert_eq!(line(&report, 1), "/tmp/ahk-test.tmp");
}

#[test]
fn a_collect_and_an_output_debug_line_answer_the_empty_text() {
    let report = run_ks("MsgBox Collect()\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_functions() {
    // The surface of the v2.0.19 stays pure: `ATan2` is not a name of the
    // v2, and the tree says so.
    let mut desktop = Fake;
    let statements = ahk::parse("x := ATan2(0.5, 1)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_defaults_of_the_hotstrings_are_the_ones_keysharp_starts_with() {
    // The defaults were read out of the HotstringManager of KeySharp on
    // 2026-11-11: the case counts for nothing, the case conforms, the
    // backspacing happens, and the ending characters are the punctuation
    // of AHK.
    let report = run_ks(concat!(
        "MsgBox A_DefaultHotstringCaseSensitive\n",
        "MsgBox A_DefaultHotstringConformToCase\n",
        "MsgBox A_DefaultHotstringDoBackspace\n",
        "MsgBox A_DefaultHotstringEndChars\n",
        "MsgBox A_DefaultHotstringSendMode\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "1");
    assert_eq!(line(&report, 3), "-()[]{}:;'\"/\\,.?!\r\n \t");
    assert_eq!(line(&report, 4), "Input");
}

#[test]
fn the_settings_of_the_hotkeys_are_the_ones_keysharp_starts_with() {
    // The defaults were read out of the AccessorData of KeySharp on
    // 2026-11-11: the clipboard waits a second, the criterion of a #HotIf
    // has a quarter of a second, and ten threads run at once.
    let report = run_ks(concat!(
        "MsgBox A_ClipboardTimeout\n",
        "MsgBox A_HotIfTimeout\n",
        "MsgBox A_MaxThreads\n",
        "MsgBox A_MaxThreadsBuffer\n",
        "MsgBox A_MaxThreadsPerHotkey\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1000");
    assert_eq!(line(&report, 1), "250");
    assert_eq!(line(&report, 2), "10");
    assert_eq!(line(&report, 3), "0");
    assert_eq!(line(&report, 4), "1");
}

#[test]
fn the_machine_and_the_build_are_named_the_way_keysharp_names_them() {
    // The architecture and the system are in the upper-case spelling the
    // documentation names, and the theme is the one KeySharp starts with
    // on this platform.
    let report = run_ks(concat!(
        "MsgBox A_OSArch\n",
        "MsgBox A_OSType\n",
        "MsgBox A_GuiTheme\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    let expected_arch = if cfg!(target_arch = "x86_64") {
        "X64"
    } else if cfg!(target_arch = "aarch64") {
        "ARM64"
    } else {
        "UNKNOWN"
    };
    assert_eq!(line(&report, 0), expected_arch);
    let expected_os = if cfg!(windows) {
        "WINDOWS"
    } else if cfg!(target_os = "macos") {
        "OSX"
    } else {
        "LINUX"
    };
    assert_eq!(line(&report, 1), expected_os);
    let expected_theme = if cfg!(windows) { "System" } else { "Classic" };
    assert_eq!(line(&report, 2), expected_theme);
}

#[test]
fn the_core_of_the_dialect_is_the_binary_that_runs_the_script() {
    // `A_KsCorePath` is the path to the Keysharp.Core.dll of KeySharp; in
    // this tree it is the binary of the runner, the same one `A_AhkPath`
    // answers.
    let report = run_ks("MsgBox A_KsCorePath\n");
    assert!(report.ok, "{:?}", report.error);
        assert_eq!(line(&report, 0), "/usr/bin/ahk");
}

#[test]
fn the_flags_of_the_script_read_the_defaults() {
    let report = run_ks(concat!(
        "MsgBox A_WinActivateForce\n",
        "MsgBox A_NoTrayIcon\n",
        "MsgBox A_Persistent\n",
        "MsgBox A_PeekFrequency\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "0");
    assert_eq!(line(&report, 3), "0");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_accessor_names() {
    let mut desktop = Fake;
    let statements = ahk::parse("x := A_ClipboardTimeout\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_thread_of_the_singleton_names_the_facts_of_the_one_thread() {
    // The documentation of KeySharp: the main thread is launched
    // automatically, it is active, it is interruptible, and the thread
    // underneath the oldest one is the empty text.
    let report = run_ks(concat!(
        "t := Thread()\n",
        "MsgBox t.Id\n",
        "MsgBox t.Index\n",
        "MsgBox t.IsActive\n",
        "MsgBox t.Kind\n",
        "MsgBox t.IsInterruptible\n",
        "MsgBox t.Underlying\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "1");
    assert_eq!(line(&report, 3), "Auto");
    assert_eq!(line(&report, 4), "1");
    assert_eq!(line(&report, 5), "");
}

#[test]
fn the_exit_of_the_thread_stops_the_script_with_the_code() {
    // The documentation of KeySharp: `Exit` on the current thread exits
    // immediately, and the code of it is the exit code of the process.
    let report = run_ks(concat!(
        "t := Thread()\n",
        "t.Exit(3)\n",
        "MsgBox \"never\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output.is_empty());
}

#[test]
fn the_code_of_a_thread_that_has_not_exited_is_unset() {
    // The v2.1 mode reads the exit code of a thread that runs as unset.
    let report = run_ks(concat!(
        "t := Thread()\n",
        "MsgBox IsSet(t.ExitCode)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_class() {
    // The surface of the v2.0.19 stays pure: `Thread` is a command of the
    // v2 and not a class, and the tree refuses the script that makes one.
    let mut desktop = Fake;
    let statements = ahk::parse("t := Thread()\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_two_booleans_answer_the_numbers_of_the_dialect() {
    let report = run_ks(concat!(
        "MsgBox True\n",
        "MsgBox False\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "0");
}

#[test]
fn the_thread_of_the_script_is_the_object_of_the_class() {
    // The documentation of KeySharp: `A_Thread` is the current
    // pseudo-thread as an object, and there is exactly one object per
    // pseudo-thread, so the test for "am I in this one" is the identity.
    let report = run_ks(concat!(
        "MsgBox Type(A_Thread)\n",
        "MsgBox A_Thread.Kind\n",
        "MsgBox (A_Thread == Thread())\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "Thread");
    assert_eq!(line(&report, 1), "Auto");
    assert_eq!(line(&report, 2), "1");
}

#[test]
fn the_hooks_of_the_machine_are_not_installed() {
    // This runtime has no hook of the keyboard and none of the mouse, and
    // the answer of "is it installed" is the truth of it.
    let report = run_ks(concat!(
        "MsgBox A_KeybdHookInstalled\n",
        "MsgBox A_MouseHookInstalled\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "0");
}

#[test]
fn the_value_of_the_last_loop_of_the_registry_is_unset() {
    // The dialect of v2 has no `Loop Reg`, so the loop never ran.
    let report = run_ks("MsgBox IsSet(A_LoopRegValue)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
}

#[test]
fn the_props_of_an_object_are_the_properties_of_it() {
    let report = run_ks(concat!(
        "class Thing {\n",
        "    x := 1\n",
        "}\n",
        "obj := Thing()\n",
        "names := \"\"\n",
        "for name, value in Props(obj) {\n",
        "    names := names name \",\"\n",
        "}\n",
        "MsgBox names\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "x,");
}

#[test]
fn the_props_of_a_value_that_is_not_an_object_refuses_with_the_kind() {
    let report = run_ks("MsgBox Props(42)\n");
    assert!(!report.ok);
    let message = report.error.clone().unwrap_or_default();
    assert!(message.contains("Parameter #1 of Props requires an Object"), "{message}");
}

#[test]
fn the_throw_of_a_value_is_taken_by_the_catch() {
    // The documentation of KeySharp: the value is thrown, and the catch
    // that takes it holds it.
    let report = run_ks(concat!(
        "caught := \"no\"\n",
        "try {\n",
        "    Throw(Error(\"boom\"))\n",
        "} catch {\n",
        "    caught := \"yes\"\n",
        "}\n",
        "MsgBox caught\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "yes");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_names() {
    // The surface of the v2.0.19 stays pure: `Props` and `A_Thread` are
    // not names of the v2, and the tree refuses the script that uses them.
    let mut desktop = Fake;
    let statements = ahk::parse("p := Props(1)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_typed_numbers_cut_the_number_to_the_width_of_the_field() {
    // The truncation is the one of the C# of KeySharp: the number is cast
    // to the width of the field with `unchecked`, and it wraps the way a
    // cast wraps -- 300 in a byte is 44, and 70000 in a word is 4464.
    let report = run_ks(concat!(
        "MsgBox Int8(300).__Value\n",
        "MsgBox UInt8(300).__Value\n",
        "MsgBox Int8(-1).__Value\n",
        "MsgBox Int16(70000).__Value\n",
        "MsgBox Int32(5).__Value\n",
        "MsgBox Float64(1.5).__Value\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "44");
    assert_eq!(line(&report, 1), "44");
    assert_eq!(line(&report, 2), "-1");
    assert_eq!(line(&report, 3), "4464");
    assert_eq!(line(&report, 4), "5");
    assert_eq!(line(&report, 5), "1.5");
}

#[test]
fn the_size_of_a_typed_number_is_the_width_of_its_field() {
    let report = run_ks(concat!(
        "MsgBox Int32.Size\n",
        "MsgBox UInt8.Size\n",
        "MsgBox Float64.Size\n",
        "MsgBox Struct.Size\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "4");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "8");
    assert_eq!(line(&report, 3), "0");
}

#[test]
fn the_struct_of_no_field_has_no_value_of_its_own() {
    // The message of KeySharp: the base of the typed numbers defines no
    // value of its own.
    let report = run_ks("MsgBox Struct().__Value\n");
    assert!(!report.ok);
    let message = report.error.clone().unwrap_or_default();
    assert!(message.contains("This struct does not define __Value."), "{message}");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_typed_numbers() {
    // The surface of the v2.0.19 stays pure: `Int32` is not a class of
    // the v2, and the tree refuses the script that makes one.
    let mut desktop = Fake;
    let statements = ahk::parse("x := Int32(5)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_argument_error_carries_the_message_of_keysharp() {
    // The source of KeySharp on 2026-11-11: the `ArgumentError` is the
    // error of a call that left out a parameter, and it carries that
    // message when the caller gives it none.
    let report = run_ks(concat!(
        "e := ArgumentError()\n",
        "MsgBox Type(e)\n",
        "MsgBox e.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "ArgumentError");
    assert_eq!(line(&report, 1), "A required parameter was omitted or unset.");
}

#[test]
fn the_unsupported_error_is_the_error_of_what_the_platform_cannot_do() {
    let report = run_ks(concat!(
        "u := UnsupportedError(\"no x11\")\n",
        "MsgBox Type(u)\n",
        "MsgBox u.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "UnsupportedError");
    assert_eq!(line(&report, 1), "no x11");
}

#[test]
fn the_property_reference_carries_the_target_and_the_arguments() {
    let report = run_ks(concat!(
        "class Thing {\n",
        "    x := 1\n",
        "}\n",
        "obj := Thing()\n",
        "p := PropRef(obj, \"x\")\n",
        "MsgBox Type(p)\n",
        "MsgBox Type(p.Target)\n",
        "MsgBox p.Args.Length\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "PropRef");
    assert_eq!(line(&report, 1), "Thing");
    assert_eq!(line(&report, 2), "0");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_classes() {
    // The surface of the v2.0.19 stays pure: `ArgumentError` and
    // `PropRef` are not names of the v2.
    let mut desktop = Fake;
    let statements = ahk::parse("e := ArgumentError()\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}
#[test]
fn the_input_hook_of_the_ks_carries_its_settings() {
    // The defaults are the ones of the source of KeySharp on 2026-11-11:
    // the buffer is 1023 characters long, the mouse is visible when it
    // moves, and the flags are off. The tree has no hook, and the object
    // is the settings of one.
    let report = run_ks(concat!(
        "ih := InputHook()\n",
        "MsgBox ih.BeforeHotkeys\n",
        "MsgBox ih.BufferLengthMax\n",
        "MsgBox ih.VisibleMouseMove\n",
        "ih.BeforeHotkeys := 1\n",
        "MsgBox ih.BeforeHotkeys\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "1023");
    assert_eq!(line(&report, 2), "1");
    assert_eq!(line(&report, 3), "1");
}
#[test]
fn the_tree_has_the_parser_and_not_the_compiler() {
    // The documentation of KeySharp: the unit is the parser or the
    // compiler, and the aliases are accepted. The tree reads the scripts
    // and never turns them into another language.
    let report = run_ks(concat!(
        "MsgBox IsComponentAvailable(\"parser\")\n",
        "MsgBox IsComponentAvailable(\"parsing\")\n",
        "MsgBox IsComponentAvailable(\"compiler\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "0");
}

#[test]
fn a_script_is_validated_without_being_run() {
    let report = run_ks(concat!(
        "good := ValidateScript(\"x := 1\")\n",
        "MsgBox good.IsValid\n",
        "MsgBox good.Errors.Length\n",
        "bad := ValidateScript(\"x := )\")\n",
        "MsgBox bad.IsValid\n",
        "MsgBox bad.Errors.Length\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "0");
    assert_eq!(line(&report, 2), "0");
    assert_eq!(line(&report, 3), "1");
}

#[test]
fn the_capabilities_of_the_machine_are_answered() {
    // The tree has no store of permissions: what it answers is what it
    // can do. The input and the capture are there, and a name it does not
    // know is unsupported.
    let report = run_ks(concat!(
        "c := RequestCapabilities(\"InputMonitoring\")\n",
        "MsgBox c.IsGranted\n",
        "MsgBox c.InputMonitoring\n",
        "d := RequestCapabilities(\"Nonsense\")\n",
        "MsgBox d.Nonsense\n",
        "MsgBox d.IsGranted\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "Granted");
    assert_eq!(line(&report, 2), "Unsupported");
    assert_eq!(line(&report, 3), "0");
}
#[test]
fn the_two_members_of_the_string_are_the_ones_of_the_ks() {
    // The comparison is the ordinal one -- the case matters -- and the
    // answer is one when the text begins or ends with the other.
    let report = run_ks(concat!(
        "t := \"hello world\"\n",
        "MsgBox t.StartsWith(\"hello\")\n",
        "MsgBox t.EndsWith(\"world\")\n",
        "MsgBox t.StartsWith(\"x\")\n",
        "MsgBox t.StartsWith(\"Hello\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "0");
    assert_eq!(line(&report, 3), "0");
}

#[test]
fn the_dialect_of_the_v2_does_not_have_the_members_of_the_string() {
    // The surface of the v2.0.19 stays pure: a text has no member named
    // `StartsWith` there.
    let mut desktop = Fake;
    let statements = ahk::parse("t := \"a\"\nMsgBox t.StartsWith(\"a\")\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}
#[test]
fn the_compilation_of_a_script_is_the_parse_of_it() {
    // This tree has no compiler of its own: the script is compiled into
    // what the tree runs, and that is the parse. The answer is the object
    // of IsValid, Errors and Warnings.
    let report = run_ks(concat!(
        "good := CompileScript(\"x := 1\")\n",
        "MsgBox good.IsValid\n",
        "MsgBox good.Errors.Length\n",
        "bad := CompileScript(\"x := )\")\n",
        "MsgBox bad.IsValid\n",
        "MsgBox bad.Errors.Length\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "0");
    assert_eq!(line(&report, 2), "0");
    assert_eq!(line(&report, 3), "1");
}

#[test]
fn the_members_of_the_process_answer_what_the_machine_says() {
    // The state of a process is asked of the machine, and the machine that
    // cannot say is refused with the reason.
    let report = run_ks(concat!(
        "p := ScriptProcess(1)\n",
        "MsgBox p.HasExited\n",
        "MsgBox p.ExitCode\n",
        "MsgBox p.ExitTime\n",
        "p.Kill()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "3");
    assert_eq!(line(&report, 2), "20261111120000");
}

#[test]
fn the_code_of_a_process_that_runs_is_unset() {
    // The v2.1 mode of KeySharp reads the code of a process that has not
    // exited as the value of unset.
    let report = run_ks(concat!(
        "q := ScriptProcess(1234)\n",
        "MsgBox q.HasExited\n",
        "MsgBox IsSet(q.ExitCode)\n",
        "MsgBox IsSet(q.ExitTime)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "0");
    assert_eq!(line(&report, 2), "0");
}

#[test]
fn the_process_that_the_machine_does_not_know_is_refused_with_the_reason() {
    // The tree does not answer zero for a state it does not know.
    let mut desktop = Fake;
    let statements = ahk::parse("MsgBox ScriptProcess(7).HasExited\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_timers_of_the_script_are_objects() {
    // The timers the script set are an array of the objects of them, and
    // each object says the function it runs and the period.
    let report = run_ks(concat!(
        "SetTimer(() => 0, 100)\n",
        "MsgBox A_Timers.Length\n",
        "MsgBox A_Timers[1].Period\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "1");
    assert_eq!(line(&report, 1), "100");
}

#[test]
fn the_condition_of_the_script_is_the_text_of_it() {
    // The tree keeps the condition of a hotkey in one of the five forms it
    // understands, and A_HotIf is the text of it.
    let report = run_ks(concat!(
        "#HotIf WinActive(\"a window\")\n",
        "F1::MsgBox \"x\"\n",
        "MsgBox A_HotIf\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "WinActive(\"a window\")");
}

#[test]
fn the_script_that_set_no_condition_has_no_hotif() {
    // The answer is the empty text when the script set no condition.
    let report = run_ks("MsgBox A_HotIf\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "");
}

#[test]
fn the_info_of_a_key_is_what_the_tree_knows_of_it() {
    // The resolution is the one of GetKeyName, GetKeyVK and GetKeySC: the
    // four builtins agree, and the name is the name v2 gives the key --
    // "Esc" is the alias and "Escape" is the name.
    let report = run_ks(concat!(
        "k := GetKeyInfo(\"Enter\")\n",
        "MsgBox k.VK\n",
        "MsgBox k.SC\n",
        "MsgBox k.Name\n",
        "MsgBox k.Name = GetKeyName(\"Enter\")\n",
        "MsgBox GetKeyInfo(\"Esc\").Name\n",
        "MsgBox GetKeyInfo(\"NoSuchKey\")\n",
        "MsgBox GetKeyInfo(\"^\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "13");
    assert_eq!(line(&report, 1), "28");
    assert_eq!(line(&report, 2), "Enter");
    assert_eq!(line(&report, 3), "1");
    assert_eq!(line(&report, 4), "Escape");
    assert_eq!(line(&report, 5), "0");
    assert_eq!(line(&report, 6), "0");
}

#[test]
fn the_modifiers_of_a_key_are_the_symbols_it_carries() {
    // The mask is the one of KeySharp: the alt is one, the control two,
    // the shift four and the windows key eight. The prefix is built in the
    // order of KeySharp -- the control, the shift, the alt, the windows
    // key -- and a side appears in it when the symbol carried one.
    let report = run_ks(concat!(
        "k := GetKeyInfo(\"^a\")\n",
        "MsgBox k.VK\n",
        "MsgBox k.Modifiers\n",
        "MsgBox k.Prefix\n",
        "j := GetKeyInfo(\"<+!f\")\n",
        "MsgBox j.VK\n",
        "MsgBox j.Modifiers\n",
        "MsgBox j.Prefix\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "65");
    assert_eq!(line(&report, 1), "2");
    assert_eq!(line(&report, 2), "^");
    assert_eq!(line(&report, 3), "70");
    assert_eq!(line(&report, 4), "5");
    assert_eq!(line(&report, 5), "<+!");
}

#[test]
fn the_property_reference_keeps_its_own_name() {
    // The chain of the extension classes asks every module, and before the
    // guard the PropRef answered the "Name" of every class of the chain
    // with the unset value: the name of a PropRef is the name of the
    // property it points at, and the name of a KeyInfo is the name of the
    // key. Each module answers only for its own class.
    let report = run_ks(concat!(
        "class Thing {\n",
        "    x := 1\n",
        "}\n",
        "p := PropRef(Thing(), \"x\")\n",
        "MsgBox p.Name\n",
        "MsgBox GetKeyInfo(\"Enter\").Name\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "x");
    assert_eq!(line(&report, 1), "Enter");
}

#[test]
fn the_layout_of_the_key_info_is_refused_with_the_reason() {
    // The tree has one keyboard layout: a layout the script names is a
    // parameter the tree cannot honour, and it says so instead of
    // answering as if it had one.
    let report = run_ks("MsgBox GetKeyInfo(\"Enter\", \"00000409\")\n");
    assert!(!report.ok);
}

#[test]
fn the_keyboard_layout_is_the_name_the_machine_gives() {
    // The name is the one the platform gives, and a machine that cannot
    // say at all is refused with the reason, and not answered with a
    // guess.
    let report = run_ks("MsgBox GetKeyboardLayout()\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "00000409:Test");
}

#[test]
fn the_cursor_is_confined_and_released() {
    // No argument releases the confinement, four arguments confine it,
    // and any other count is refused with the message of KeySharp. The
    // corners may come in any order, and a value that is not a number is
    // refused with the reason.
    let report = run_ks(concat!(
        "ClipCursor()\n",
        "ClipCursor(10, 10, 100, 100)\n",
        "ClipCursor(100, 100, 10, 10)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    let report = run_ks("ClipCursor(1, 2, 3)\n");
    assert!(!report.ok);
    let report = run_ks("ClipCursor(\"x\", 2, 3, 4)\n");
    assert!(!report.ok);
}

#[test]
fn the_window_at_a_point_is_the_one_the_machine_names() {
    // The answer is the identity of the window, and zero when no window
    // is at the point. A value that is not a number is refused with the
    // reason: zero means "no window", and not "the argument was wrong".
    let report = run_ks(concat!(
        "MsgBox WinFromPoint(5, 5)\n",
        "MsgBox WinFromPoint(9, 9)\n",
        "MsgBox WinFromPoint(500, 500)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "2");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "0");
    let report = run_ks("MsgBox WinFromPoint(\"x\", 5)\n");
    assert!(!report.ok);
}

#[test]
fn the_real_thread_is_the_thread_the_script_runs_on() {
    // The tree runs the script on one real thread: the collection of the
    // threads is that one, it is alive while the script runs, and its
    // identity is the one the machine gives.
    let report = run_ks(concat!(
        "t := A_RealThread\n",
        "MsgBox Type(t)\n",
        "MsgBox t.IsAlive\n",
        "MsgBox t.Id\n",
        "MsgBox t.Threads.Length\n",
        "MsgBox RealThread.Main.IsAlive\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "RealThread");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "4242");
    assert_eq!(line(&report, 3), "1");
    assert_eq!(line(&report, 4), "1");
}

#[test]
fn the_worker_members_of_the_real_thread_are_refused_with_the_reason() {
    // The thread of the tree is not a worker: Task and Terminated are
    // refused the way KeySharp refuses them on its main thread, and Post
    // and Send are refused because the tree has no queue to post to.
    let report = run_ks("MsgBox A_RealThread.Task\n");
    assert!(!report.ok);
    let report = run_ks("MsgBox A_RealThread.Post(() => 0)\n");
    assert!(!report.ok);
}

#[test]
fn the_exit_of_the_real_thread_stops_the_run() {
    // The exit of the main thread is the end of the run, the way ExitApp
    // ends it: the code is kept and the statements after it do not run.
    let report = run_ks(concat!(
        "MsgBox \"before\"\n",
        "A_RealThread.Exit(3)\n",
        "MsgBox \"after\"\n",
    ));
    assert_eq!(line(&report, 0), "before");
    assert_eq!(report.exit_code, Some(3));
}

#[test]
fn the_empty_reference_points_at_no_variable() {
    // VarRef.Empty is the reference that points at no variable: the type
    // of it is VarRef, it is an object, and a built-in that writes into
    // it writes nowhere -- the run goes on and nothing is created.
    let report = run_ks(concat!(
        "e := VarRef.Empty\n",
        "MsgBox Type(e)\n",
        "MsgBox IsObject(e)\n",
        "SplitPath(\"C:/dir/file.txt\", e)\n",
        "MsgBox \"done\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "VarRef");
    assert_eq!(line(&report, 1), "1");
    assert_eq!(line(&report, 2), "done");
}

#[test]
fn the_view_of_a_struct_carries_the_address_it_borrows() {
    // The view of the empty struct at an address carries the address and
    // nothing else: it has no field to read, and the tree never reads the
    // memory it points at.
    let report = run_ks(concat!(
        "v := Struct.At(0x10000)\n",
        "MsgBox v.Ptr\n",
        "MsgBox ObjGetDataPtr(v)\n",
        "MsgBox ObjGetDataSize(v)\n",
        "ObjSetDataPtr(v, 0x20000)\n",
        "MsgBox ObjGetDataPtr(v)\n",
        "w := ObjCreateWithDataPtr(Struct, 0x30000)\n",
        "MsgBox ObjGetDataPtr(w)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "65536");
    assert_eq!(line(&report, 1), "65536");
    assert_eq!(line(&report, 2), "0");
    assert_eq!(line(&report, 3), "131072");
    assert_eq!(line(&report, 4), "196608");
}

#[test]
fn the_data_pointer_of_an_object_that_borrows_none_is_refused() {
    // The answers of KeySharp itself: "Operation failed." for an object
    // with no external data, "Invalid class." for a class that is not a
    // struct, and the parameter message for an address below 65536 that
    // is not the null one. The typed number -- whose field would live in
    // the memory -- is refused with the reason.
    let report = run_ks("MsgBox ObjGetDataPtr(Map())\n");
    assert!(!report.ok);
    let report = run_ks("MsgBox ObjCreateWithDataPtr(Map, 0x10000)\n");
    assert!(!report.ok);
    let report = run_ks("MsgBox Int32.At(0x10000)\n");
    assert!(!report.ok);
    let report = run_ks("MsgBox Int32.At(5)\n");
    assert!(!report.ok);
    let report = run_ks("MsgBox ObjGetDataPtr(42)\n");
    assert!(!report.ok);
    let report = run_ks(concat!(
        "MsgBox ObjGetDataSize(Map())\n",
        "MsgBox ObjGetDataSize(Int32(5))\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "0");
    assert_eq!(line(&report, 1), "4");
}

#[test]
fn the_debug_tab_of_no_window_is_the_empty_text() {
    // The ShowDebug of KeySharp does nothing when the script has no main
    // window -- the call of it is the null-conditional in its source --
    // and the runner of this tree has no window at all: the answer is the
    // empty text and the run goes on.
    let report = run_ks("ShowDebug()\nMsgBox \"done\"\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "done");
}

#[test]
fn the_v2_has_no_view_of_a_struct() {
    // The surface of the v2.0.19 stays pure: the view of a struct at an
    // address is a name of the ks dialect alone.
    let mut desktop = Fake;
    let statements = ahk::parse("v := Struct.At(0x10000)\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}

#[test]
fn the_mail_is_validated_before_the_machine_is_asked() {
    // The shapes are the dialect's, and the messages are the ones of
    // KeySharp: the empty list of the recipients, the host that is
    // missing, the port that is not one and the key that is not known
    // are refused before the network is asked. The Fake sends the
    // message that passes.
    let report = run_ks("Mail([], \"s\", \"b\", Map(\"host\", \"smtp\", \"from\", \"a@b\"))\n");
    assert!(!report.ok);
    let report = run_ks("Mail(\"a@b\", \"s\", \"b\")\n");
    assert!(!report.ok);
    let report = run_ks(
        "Mail(\"a@b\", \"s\", \"b\", Map(\"host\", \"smtp.example:99999\", \"from\", \"a@b\"))\n",
    );
    assert!(!report.ok);
    let report = run_ks(
        "Mail(\"a@b\", \"s\", \"b\", Map(\"host\", \"smtp.example\", \"from\", \"a@b\", \"nope\", 1))\n",
    );
    assert!(!report.ok);
    let report = run_ks(
        "Mail([\"a@b\", \"c@d\"], \"s\", \"b\", Map(\"host\", \"smtp.example:587\", \"from\", \"a@b\", \"cc\", \"e@f\"))\n",
    );
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_script_of_the_run_script_is_checked_before_it_is_launched() {
    // The compile of the tree is the parser: a script that does not
    // parse is never launched, and one that parses comes back as the
    // ScriptProcess of its pid. The name of the generated program of
    // KeySharp has nothing to name here, and the switches of a compile
    // the tree does not do are refused with the reason.
    let report = run_ks("p := RunScript(\"MsgBox 1\")\nMsgBox Type(p)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "ScriptProcess");
    let report = run_ks("RunScript(\"a := := 5\")\n");
    assert!(!report.ok);
    let report = run_ks("RunScript(\"ExitApp\", , , \"named\")\n");
    assert!(!report.ok);
    let report = run_ks("RunScript(\"ExitApp\", , , , , \"--define:X\")\n");
    assert!(!report.ok);
}

#[test]
fn the_callback_of_a_waited_run_script_is_called_with_the_process() {
    // The run is waited for, so the process is over when the callback
    // is called with it, the way KeySharp calls it after the exit.
    let report = run_ks(concat!(
        "Done(p) {\n",
        "    MsgBox Type(p)\n",
        "}\n",
        "q := RunScript(\"ExitApp\", false, Done)\n",
        "MsgBox Type(q)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "ScriptProcess");
    assert_eq!(line(&report, 1), "ScriptProcess");
}

#[test]
fn the_run_script_of_a_process_it_does_not_wait_for_says_so() {
    // The tree does not watch a process it does not wait for: the
    // callback beside the asynchronous run is refused with the reason,
    // and the run without one comes back with the process of the pid.
    let report = run_ks(concat!(
        "Done(p) {\n",
        "}\n",
        "RunScript(\"ExitApp\", true, Done)\n",
    ));
    assert!(!report.ok);
    let report = run_ks("p := RunScript(\"ExitApp\", true)\nMsgBox Type(p)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(line(&report, 0), "ScriptProcess");
}

#[test]
fn the_v2_has_no_mail_and_no_run_script() {
    // The surface of the v2.0.19 stays pure: the two names are of the
    // ks dialect alone.
    let mut desktop = Fake;
    let statements = ahk::parse("Mail(\"a@b\", \"s\", \"b\")\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    let mut desktop = Fake;
    let statements = ahk::parse("RunScript(\"ExitApp\")\n").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
}
