//! The run that stays alive: the `Persistent` of the v2, the resident
//! run of the runner, and the run with no budget of time.
//!
//! The cases are measured against a fake desktop that watches the
//! keyboard: the run that stays alive waits for the events of the
//! machine, and the fake is the machine.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::parse_with;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that watches the keyboard and waits for the events.
#[derive(Debug, Default)]
struct Fake {
    /// Whether the desktop knows how to wait for an event.
    ///
    /// A desktop that does not know refuses the wait, and the runtime
    /// waits with its own clock.
    can_wait: bool,
}

/// Run a script and stay alive, and answer with its report.
///
/// The run is a resident one: it goes on with the events of the machine
/// after the statements of the script end.
fn run_and_stay(source: &str, desktop: &mut Fake) -> Report {
    let statements = parse_with(source, ahk::dialect::v2()).expect("parses");
    let mut runtime = Runtime::new(desktop, Duration::from_secs(10))
        .with_resident(true)
        .with_dialect(ahk::dialect::v2());
    runtime.run_and_stay(&statements)
}

/// Run a script in the ks dialect, and answer with its report.
fn run_in_the_ks(source: &str) -> Report {
    let statements = parse_with(source, ahk::dialect::ks()).expect("parses");
    let mut desktop = Fake::default();
    let runtime =
        Runtime::new(&mut desktop, Duration::from_secs(10)).with_dialect(ahk::dialect::ks());
    runtime.run(&statements)
}

impl Desktop for Fake {}
impl Hotkeys for Fake {
    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn wait_for_event(&mut self, _timeout: u64) -> Result<bool, String> {
        if !self.can_wait {
            return Err("this desktop cannot wait for an event".to_string());
        }
        // The wait of the machine is short: the case does not want to
        // sleep.
        std::thread::sleep(Duration::from_millis(2));
        Ok(false)
    }
}
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
}
impl Windows for Fake {}
impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}
impl Files for Fake {}
impl Clipboard for Fake {}
impl Timers for Fake {}
impl Native for Fake {}
impl Com for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}
impl Network for Fake {}
impl Pictures for Fake {}

/// The `Persistent` of the v2 answers the value that the script had.
#[test]
fn the_persistent_answers_the_value_that_the_script_had() {
    let mut desktop = Fake::default();
    let statements = parse_with(
        "a := Persistent()\nb := Persistent()\nc := Persistent(false)\nd := Persistent()",
        ahk::dialect::v2(),
    )
    .expect("parses");
    let runtime =
        Runtime::new(&mut desktop, Duration::from_secs(10)).with_dialect(ahk::dialect::v2());
    let report = runtime.run(&statements);
    // The first call answers the zero, and the second the one: the
    // answer is the old value, the way v2 answers it.
    assert_eq!(
        report.variables.get("a"),
        Some(&ahk::runtime::Value::Int(0))
    );
    assert_eq!(
        report.variables.get("b"),
        Some(&ahk::runtime::Value::Int(1))
    );
    assert_eq!(
        report.variables.get("c"),
        Some(&ahk::runtime::Value::Int(1))
    );
    assert_eq!(
        report.variables.get("d"),
        Some(&ahk::runtime::Value::Int(0))
    );
}

/// The `Persistent` of the ks dialect is a variable, and it reads the
/// state of the run.
#[test]
fn the_persistent_variable_of_the_ks_dialect_reads_the_state_of_the_run() {
    let report = run_in_the_ks("a := A_Persistent");
    assert_eq!(
        report.variables.get("a"),
        Some(&ahk::runtime::Value::Int(0))
    );
}

/// The script that asked to live goes on after its statements: its
/// timers run.
#[test]
fn the_script_that_asked_to_live_goes_on_with_the_events_of_the_machine() {
    let mut desktop = Fake { can_wait: true };
    let report = run_and_stay(
        "count := 0\nPersistent\nSetTimer(\"tick\", 10)\ntick() {\n    global count\n    count += 1\n    if count >= 3\n        ExitApp\n}",
        &mut desktop,
    );
    assert!(report.ok, "the run failed: {:?}", report.error);
    // The timer ran three times: the run went on after the statements.
    assert_eq!(
        report.variables.get("count"),
        Some(&ahk::runtime::Value::Int(3))
    );
}

/// The script that did not ask ends with its statements.
#[test]
fn the_script_that_did_not_ask_ends_with_its_statements() {
    let mut desktop = Fake { can_wait: true };
    let statements = parse_with(
        "count := 0\nSetTimer(\"tick\", 10)\ntick() {\n    global count\n    count += 1\n}",
        ahk::dialect::v2(),
    )
    .expect("parses");
    let runtime =
        Runtime::new(&mut desktop, Duration::from_secs(10)).with_dialect(ahk::dialect::v2());
    let report = runtime.run(&statements);
    assert!(report.ok, "the run failed: {:?}", report.error);
    // The timer did not run: the run ended with the statements of the
    // script.
    assert_eq!(
        report.variables.get("count"),
        Some(&ahk::runtime::Value::Int(0))
    );
}

/// The wait that the machine does not have is said once, and not once
/// per turn of the run.
#[test]
fn the_desktop_that_cannot_wait_is_said_once() {
    let mut desktop = Fake { can_wait: false };
    let report = run_and_stay(
        "count := 0\nPersistent\nSetTimer(\"tick\", 10)\ntick() {\n    global count\n    count += 1\n    if count >= 3\n        ExitApp\n}",
        &mut desktop,
    );
    assert!(report.ok, "the run failed: {:?}", report.error);
    let refusals = report
        .notes
        .iter()
        .filter(|note| note.contains("cannot wait for an event"))
        .count();
    assert_eq!(refusals, 1, "the notes are {:?}", report.notes);
}

/// The run with no budget of time does not stop with the time.
#[test]
fn the_run_with_no_budget_does_not_stop_with_the_time() {
    let statements = parse_with(
        "count := 0\nwhile count < 500 {\n    count += 1\n}",
        ahk::dialect::v2(),
    )
    .expect("parses");
    let mut desktop = Fake::default();
    // With the budget of time, the run stops in the middle of the way.
    let runtime =
        Runtime::new(&mut desktop, Duration::from_millis(0)).with_dialect(ahk::dialect::v2());
    let report = runtime.run(&statements);
    assert!(!report.ok, "the run with a budget of nothing did not fail");
    // Without the budget, the run goes to the end.
    let mut desktop = Fake::default();
    let runtime = Runtime::new(&mut desktop, Duration::from_millis(0))
        .with_no_budget(true)
        .with_dialect(ahk::dialect::v2());
    let report = runtime.run(&statements);
    assert!(
        report.ok,
        "the run without a budget failed: {:?}",
        report.error
    );
    assert_eq!(
        report.variables.get("count"),
        Some(&ahk::runtime::Value::Int(500))
    );
}
