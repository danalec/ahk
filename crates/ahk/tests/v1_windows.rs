//! The windows of the v1: the list, the count, and the last window.
//!
//! The v1 has a state of its own for the last window that a script
//! found, and a shape of its own for the list of the windows: the
//! number goes into the variable of the caller and the identifiers go
//! into the variables whose names begin with the name of it.
//!
//! The cases were measured on the binary of AutoHotkey v1.1.37.02 and the probes are in
//! `egm-review/measure_the_count_and_the_list.py` and in
//! `egm-review/measure_the_last_window_of_the_v1.py`.

use ahk::desktop::{Desktop, TitleMatch, WindowInfo, WindowRef, Windows};
use ahk::dialect;
use ahk::runtime::{Report, Runtime, Value};
use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Duration;

/// The turn of the tests: the settings of the script live in a static.
static THE_TURN: Mutex<()> = Mutex::new(());

/// A desktop that has a few windows and nothing else.
#[derive(Default)]
struct TheWindows {
    /// The windows of the desktop, in the order that they are listed.
    windows: Vec<WindowInfo>,
    /// What the machine was asked, in the order of the questions.
    calls: RefCell<Vec<String>>,
}

impl Desktop for TheWindows {}
impl ahk::desktop::Clipboard for TheWindows {}
impl ahk::desktop::Com for TheWindows {}
impl ahk::desktop::Dialogs for TheWindows {}
impl ahk::desktop::Files for TheWindows {}
impl ahk::desktop::Hotkeys for TheWindows {}
impl ahk::desktop::Keyboard for TheWindows {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }
}
impl ahk::desktop::Machine for TheWindows {}
impl ahk::desktop::Native for TheWindows {}
impl ahk::desktop::Network for TheWindows {}
impl ahk::desktop::Pictures for TheWindows {}
impl ahk::desktop::Pointer for TheWindows {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Err("this fake has no pointer".into())
    }
}
impl ahk::desktop::Processes for TheWindows {
    fn env_get(&mut self, _name: &str) -> Result<Option<String>, String> {
        Ok(None)
    }
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Err("this fake has no process".into())
    }
}
impl ahk::desktop::Screen for TheWindows {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Err("this fake has no screen".into())
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this fake has no screen".into())
    }
}
impl ahk::desktop::Sound for TheWindows {}
impl ahk::desktop::Timers for TheWindows {}

impl Windows for TheWindows {
    fn windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        Ok(self.windows.clone())
    }

    fn matching_windows(
        &mut self,
        title: &str,
        mode: TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        self.calls.borrow_mut().push(format!("matching {title}"));
        let mut found = Vec::new();
        for window in &self.windows {
            let matches = if title.is_empty() {
                true
            } else {
                mode.matches(&window.title, title).unwrap_or(false)
            };
            if matches {
                found.push(WindowRef {
                    id: window.id.to_string(),
                    class: window.class.clone(),
                });
            }
        }
        Ok(found)
    }

    fn window_controls(&mut self, id: &str, handles: bool) -> Result<Option<String>, String> {
        self.calls
            .borrow_mut()
            .push(format!("controls {id} {handles}"));
        // The fake has two controls in every window: a button and an edit.
        if !self
            .windows
            .iter()
            .any(|window| window.id.to_string() == id)
        {
            return Ok(None);
        }
        Ok(Some(if handles {
            "0x11\n0x22".to_string()
        } else {
            "Button1\nEdit1".to_string()
        }))
    }

    fn window_exists(&mut self, title: &str, mode: TitleMatch) -> Result<ahk::desktop::WindowAnswer, String> {
        self.calls.borrow_mut().push(format!("exists {title}"));
        for window in &self.windows {
            if title.is_empty() || mode.matches(&window.title, title).unwrap_or(false) {
                return Ok(ahk::desktop::WindowAnswer::NotActive);
            }
        }
        Ok(ahk::desktop::WindowAnswer::Absent)
    }

    fn window_wait(
        &mut self,
        title: &str,
        mode: TitleMatch,
        what: ahk::desktop::WaitFor,
        timeout_ms: u64,
        excludes: &[String],
    ) -> Result<Option<String>, String> {
        self.calls.borrow_mut().push(format!(
            "wait {title} {what:?} {timeout_ms} {}",
            excludes.join(",")
        ));
        let there = if title.is_empty() && excludes.is_empty() {
            // The empty title is no window at all: the runtime resolves the
            // window that the script found last before it asks the desktop,
            // and a window with no title cannot be asked for by one.
            false
        } else {
            self.windows.iter().any(|window| {
                (title.is_empty() || mode.matches(&window.title, title).unwrap_or(false))
                    && !excludes.iter().any(|excluded| {
                        mode.matches(&window.title, excluded).unwrap_or(false)
                    })
            })
        };
        Ok(match what {
            ahk::desktop::WaitFor::Present | ahk::desktop::WaitFor::Active => {
                there.then(|| "0x421812".to_string())
            }
            ahk::desktop::WaitFor::NotActive | ahk::desktop::WaitFor::Close => {
                (!there).then(String::new)
            }
        })
    }
}

/// Run a script of the v1 with a desktop that has windows, and answer
/// with the report.
fn run_with(script: &str, windows: Vec<WindowInfo>) -> Report {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = TheWindows {
        windows,
        calls: RefCell::new(Vec::new()),
    };
    Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements)
}

/// A window of the fake.
fn a_window(id: i64, title: &str, class: &str) -> WindowInfo {
    WindowInfo {
        id,
        title: title.to_string(),
        class: class.to_string(),
        active: false,
        exe: "notepad.exe".to_string(),
        pid: 9001,
        text: String::new(),
    }
}

/// A window of the fake that has the focus.
fn a_window_with_the_focus(id: i64, title: &str, class: &str) -> WindowInfo {
    let mut window = a_window(id, title, class);
    window.active = true;
    window
}

/// The title that is the `A` alone is the window that has the focus.
///
/// Measured on the binary of v1.1.37.02: the `WinExist("A")` answers the
/// handle of the window that has the focus (`0x2080564`).
#[test]
fn the_title_that_is_the_a_alone_is_the_window_that_has_the_focus() {
    let report = run_with(
        "x := WinExist(\"A\")",
        vec![
            a_window(0x421812, "One", "AClass"),
            a_window_with_the_focus(0x421813, "Two", "BClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "x"), "0x421813");
}

/// The text of a variable of the report.
fn text_of(report: &Report, name: &str) -> String {
    match report.variables.get(name) {
        Some(Value::Text(text)) => text.to_string(),
        Some(other) => format!("{other:?}"),
        None => String::new(),
    }
}

/// The number of a variable of the report.
fn int_of(report: &Report, name: &str) -> i64 {
    match report.variables.get(name) {
        Some(Value::Int(number)) => *number,
        Some(other) => {
            let text = format!("{other:?}");
            text.parse()
                .unwrap_or_else(|_| panic!("{name} is not a number: {text}"))
        }
        None => 0,
    }
}

#[test]
fn the_count_of_the_windows_goes_into_the_variable_of_the_caller() {
    let report = run_with(
        "WinGet, count, Count",
        vec![
            a_window(1, "One", "AClass"),
            a_window(2, "Two", "BClass"),
            a_window(3, "Three", "CClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "count"), 3);
}

#[test]
fn the_list_of_the_windows_is_a_pseudo_array() {
    // Medido no binario do v1.1.37.02: `WinGet, list,
    // List` escreve o numero das janelas na variavel do chamador e os
    // identificadores nas variaveis `list1`, `list2` e assim por diante.
    let report = run_with(
        "WinGet, list, List",
        vec![
            a_window(0x421812, "One", "AClass"),
            a_window(0x421813, "Two", "BClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "list"), 2);
    // O identificador e em hexadecimal, como no v1.
    assert_eq!(text_of(&report, "list1"), "0x421812");
    assert_eq!(text_of(&report, "list2"), "0x421813");
}

#[test]
fn the_list_of_the_windows_writes_nothing_when_there_is_no_window() {
    let report = run_with("WinGet, list, List", Vec::new());
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "list"), 0);
}

#[test]
fn the_count_and_the_list_do_not_need_a_title() {
    let report = run_with(
        "WinGet, count, Count\nWinGet, list, List",
        vec![a_window(1, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(int_of(&report, "list"), 1);
}

#[test]
fn the_exist_of_a_window_that_is_there_is_the_identifier_of_it() {
    let report = run_with(
        "found := WinExist(\"One\")",
        vec![a_window(0x421812, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "found"), "0x421812");
}

#[test]
fn the_exist_of_a_window_that_is_not_there_is_zero() {
    let report = run_with("found := WinExist(\"Two\")", vec![a_window(1, "One", "AClass")]);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "found"), "0x0");
}

#[test]
fn the_last_window_is_the_one_that_the_script_found_last() {
    // Medido no binario do v1.1.37.02: `WinGet, id,
    // IDLast` sem titulo usa a janela que o script achou por ultimo.
    let report = run_with(
        "found := WinExist(\"One\")\nWinGet, id, IDLast",
        vec![
            a_window(0x421812, "One", "AClass"),
            a_window(0x421813, "Two", "BClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "0x421812");
}

#[test]
fn the_last_window_is_not_changed_by_a_window_that_is_not_there() {
    // Measured on the binary of v1.1.37.02: a `WinExist`
    // that finds no window does not change the window of the last
    // time.
    let report = run_with(
        "found := WinExist(\"One\")\nfound := WinExist(\"Missing\")\nWinGet, id, IDLast",
        vec![a_window(0x421812, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "0x421812");
}

#[test]
fn the_last_window_without_a_window_that_was_found_is_empty() {
    // Measured on the binary of v1.1.37.02: the `WinGet,
    // out, IDLast` of a script that found no window at all answers the
    // empty text, and not the `0x0` that the `WinExist` answers.
    let report = run_with("WinGet, id, IDLast", Vec::new());
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "");
}

#[test]
fn the_id_of_a_window_with_a_title_is_the_identifier_of_it() {
    let report = run_with(
        "WinGet, id, ID, Two",
        vec![
            a_window(0x421812, "One", "AClass"),
            a_window(0x421813, "Two", "BClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "0x421813");
}

#[test]
fn the_id_last_of_a_window_with_a_title_is_the_last_that_matches() {
    // Measured on the binary of v1.1.37.02: with a
    // title, the `IDLast` is the window that is at the bottom of the
    // list.
    let report = run_with(
        "WinGet, id, IDLast, Two",
        vec![
            a_window(0x421812, "Two", "AClass"),
            a_window(0x421813, "Two", "BClass"),
        ],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "0x421813");
}

/// The waits that the fake responde na hora.
fn a_desktop_that_answers_the_waits() -> TheWindows {
    TheWindows {
        windows: Vec::new(),
        calls: RefCell::new(Vec::new()),
    }
}

#[test]
fn the_wait_of_the_v1_that_finds_a_window_leaves_the_error_level_at_zero() {
    // Medido no binario do v1.1.37.02: o `ErrorLevel` do
    // `WinWait` e zero quando a janela e achada, e um quando o tempo
    // acaba.
    let report = run_with("WinWait, One", vec![a_window(0x421812, "One", "AClass")]);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "errorlevel"), 0);
}

#[test]
fn the_wait_not_active_of_the_v1_leaves_the_error_level_at_zero() {
    // Measured on the binary of v1.1.37.02: the `WinWaitNotActive` with
    // no title answers at once -- 146 ms for the whole script -- and
    // leaves the `ErrorLevel` at zero.
    let report = run_with("WinWaitNotActive", vec![]);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "errorlevel"), 0);
}

#[test]
fn the_wait_for_the_close_does_not_change_the_last_found_window() {
    // Measured on the binary of v1.1.37.02: the `WinWaitClose` leaves the
    // last found window where it was -- the source of it
    // (`script2.cpp:2277-2283`) asks the `WinExist` without a title, and
    // that one does not change it.
    let report = run_with(
        "x := WinExist(\"One\")\nWinWaitClose\nWinGet, id, IDLast",
        vec![a_window(0x421812, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "id"), "0x421812");
}

#[test]
fn the_wait_with_no_title_waits_for_the_window_that_was_found_last() {
    // Measured on the binary of v1.1.37.02: the `WinWaitClose` with no
    // title waits for the last found window to go away. The source of it
    // (`window.cpp:571-576`) says that the `WinExist` without a title
    // answers the last found window, and the wait asks the `WinExist`
    // for the window.
    let mut desktop = TheWindows {
        windows: vec![a_window(0x421812, "One", "AClass")],
        calls: RefCell::new(Vec::new()),
    };
    let statements = ahk::parse_with("x := WinExist(\"One\")\nWinWaitClose", dialect::v1())
        .expect("the script of the v1 parses");
    let runtime = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1());
    let report = runtime.run(&statements);
    assert!(report.ok, "{:?}", report.error);
    // The wait looks for the window that the `WinExist` found last.
    assert_eq!(
        desktop.calls.borrow().last().map(String::as_str),
        Some("wait One Close 60000 ")
    );
}

#[test]
fn the_zero_of_the_timeout_of_the_wait_of_the_v1_is_the_meio_second() {
    // Measured on the binary of v1.1.37.02: the
    // `WinWait, titulo, , 0` waits the meio segundo, and not forever.
    let mut desktop = a_desktop_that_answers_the_waits();
    let statements = ahk::parse_with("WinWait, One, , 0", dialect::v1())
        .expect("the script of the v1 parses");
    let runtime = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1());
    let report = runtime.run(&statements);
    assert!(report.ok, "{:?}", report.error);
    // O zero do v1 e o meio segundo.
    assert_eq!(
        desktop.calls.borrow().last().map(String::as_str),
        Some("wait One Present 500 ")
    );
}

#[test]
fn the_exclusion_of_the_wait_of_the_v1_takes_the_window_out() {
    // Measured on the binary of v1.1.37.02: the `WinWait`
    // with the exclusion of the title of the window does not find the
    // window.
    let report = run_with(
        "WinWait, One, , 1, One",
        vec![a_window(0x421812, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "errorlevel"), 1);
}

#[test]
fn the_controls_are_the_ones_of_the_window_that_is_named() {
    // Measured on the binary of v1.1.37.02: the `WinGet, out, ControlList,
    // <title>` answers the controls of the window that the title names, and
    // not the controls of the window of the script.
    let report = run_with(
        "WinGet, names, ControlList, Two",
        vec![a_window(1, "One", "AClass"), a_window(2, "Two", "BClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "names"), "Button1\nEdit1");

    // The identifiers of the controls, and not their names.
    let report = run_with(
        "WinGet, handles, ControlListHwnd, Two",
        vec![a_window(1, "One", "AClass"), a_window(2, "Two", "BClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "handles"), "0x11\n0x22");

    // The window that is not there: the v1 answers the empty text, and the
    // run goes on.
    let report = run_with(
        "WinGet, names, ControlList, NoSuchWindow",
        vec![a_window(1, "One", "AClass")],
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(text_of(&report, "names"), "");
}
