//! The hook of the messages of the v1: the callback that the script gives
//! to the machine.
//!
//! The v1 makes an address out of a function of the script
//! (`RegisterCallback`), and the machine calls that address: the Quick
//! A script of the v1 installs a `WH_CALLWNDPROC` hook with it, and the hook
//! is called for every message of every window of the thread of the
//! script.
//!
//! The cases are about the three things that the hook of a script of the v1
//! Viewer needs:
//!
//! - the machine calls the address, and the function of the script runs;
//! - the answer of the function comes back to the machine, the
//!   `CallNextHookEx` of a script of the v1 is the answer of the
//!   callback of the script;
//! - the messages of the machine are not pumped again while the callback
//!   runs: the callback of the machine does not re-enter the runtime that
//!   is running it.
//!
//! The cases were measured on the binary of AutoHotkey v1.1.37.02 and the probe is in
//! the probe of the measurement.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::dll::{NativeCall, NativeFailure, NativeResult, NativeValue};
use ahk::runtime::{Report, Runtime};
use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Duration;

/// The turn of the tests: the settings of the script live in a static, and
/// the runtime writes the defaults of its dialect when it is made. Two
/// cases that run a script cannot run at the same time.
static THE_TURN: Mutex<()> = Mutex::new(());

/// A desktop that calls the callback of the script the way the machine
/// calls it.
///
/// The machine calls the address that the script gave it, and the answer
/// of the function of the script comes back to the machine.
struct Fake {
    /// The address of the callback that the script gave to the machine.
    hook: RefCell<Option<usize>>,
    /// The answers that the callback of the script gave to the machine.
    answers: RefCell<Vec<usize>>,
    /// How many times the machine pumped the messages of the thread.
    pumps: RefCell<u32>,
    /// How deep the pump of the messages went.
    ///
    /// The pump of the machine calls the callback of the script, and the
    /// run of the callback is inside the pump: the depth says whether
    /// the runtime pumped the messages again while the callback ran.
    depth: RefCell<u32>,
    /// The deepest the pump of the messages went.
    deepest: RefCell<u32>,
    /// Whether the pump of the messages calls the hook of the script.
    ///
    /// The machine calls the hook of the script for every message of the
    /// thread, and the messages are pumped by the runtime of the tree.
    hook_in_the_pump: RefCell<bool>,
}

impl Fake {
    /// A desktop that has not been asked anything yet.
    fn new() -> Self {
        Fake {
            hook: RefCell::new(None),
            answers: RefCell::new(Vec::new()),
            pumps: RefCell::new(0),
            depth: RefCell::new(0),
            deepest: RefCell::new(0),
            hook_in_the_pump: RefCell::new(true),
        }
    }

    /// Call the address of the callback the way the machine calls it, and
    /// answer with what the function of the script answered.
    fn the_hook_is_called(&self) -> Option<usize> {
        let address = (*self.hook.borrow())?;
        // The machine calls the address with the registers of the call:
        // the tree makes the trampoline of the place of the pool, and the
        // place knows the function of the script. The trampoline itself
        // is asked for, and not the number of it: a number has no
        // provenance, and the call through it cannot be checked by Miri.
        // The registers carry numbers that are easy to tell apart: the
        // count of the parameters of the callback says how many of them
        // the function of the script gets.
        let trampoline = ahk::callback::the_trampoline_of(address)?;
        let answer = trampoline(11, 22, 33, 44);
        self.answers.borrow_mut().push(answer);
        Some(answer)
    }
}

impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        // The script gives the address of the callback to the machine:
        // `SetWindowsHookEx` takes it as its second parameter.
        if call.function.contains("SetWindowsHookEx") {
            let address = match call.arguments.get(1).map(|argument| &argument.value) {
                Some(NativeValue::Integer(address)) => *address as usize,
                _ => 0,
            };
            *self.hook.borrow_mut() = Some(address);
            return Ok(NativeResult {
                kind: call.returns,
                value: NativeValue::Integer(1),
            });
        }
        // The machine calls the callback: the answer of the function of
        // the script is the answer of the call.
        if call.function.contains("CallTheHook") {
            let answer = self.the_hook_is_called().unwrap_or(0) as i64;
            return Ok(NativeResult {
                kind: call.returns,
                value: NativeValue::Integer(answer),
            });
        }
        Ok(NativeResult {
            kind: call.returns,
            value: NativeValue::Integer(0),
        })
    }
}

impl Windows for Fake {
    /// The events of the windows of the script: the machine pumps the
    /// messages of the thread of the script here, and the hook of the
    /// script is called for every message of the thread.
    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        *self.pumps.borrow_mut() += 1;
        {
            let mut depth = self.depth.borrow_mut();
            *depth += 1;
            let mut deepest = self.deepest.borrow_mut();
            if *depth > *deepest {
                *deepest = *depth;
            }
        }
        // The machine dispatches the messages of the thread, and the hook
        // of the script is called for each of them.
        if *self.hook_in_the_pump.borrow() {
            self.the_hook_is_called();
        }
        *self.depth.borrow_mut() -= 1;
        Ok(Vec::new())
    }
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Sound for Fake {}
impl Dialogs for Fake {}
impl Network for Fake {}
impl Clipboard for Fake {}
impl Files for Fake {}
impl Hotkeys for Fake {}
impl Com for Fake {}
impl Timers for Fake {}
impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Err("this fake has no pointer".into())
    }
}

impl Processes for Fake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Err("this fake has no processes".into())
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
        Err("this fake has no screen".into())
    }

    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this fake has no screen".into())
    }
}

impl Machine for Fake {}

/// Run a script of the v1 and answer with the report and with the desktop
/// that answered it.
fn run_and_the_desktop(script: &str) -> (Report, Fake) {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake::new();
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    (report, desktop)
}

/// Run a script of the v1 whose pump of the messages does not call the
/// hook of the script: the case is about the call of the machine itself.
fn run_without_the_hook_in_the_pump(script: &str) -> (Report, Fake) {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake::new();
    *desktop.hook_in_the_pump.borrow_mut() = false;
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    (report, desktop)
}

/// The hook of the messages of the v1 reaches the script, and the answer
/// of the script comes back to the machine.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `RegisterCallback` of the v1 makes a native callback, and the callback
/// of the script runs in the thread of the script; the number that the
/// function answers is the answer that the caller of the address gets.
#[test]
fn the_hook_of_the_v1_answers_the_machine_with_the_answer_of_the_script() {
    let (report, desktop) = run_without_the_hook_in_the_pump(
        "cb := RegisterCallback(\"theHook\", \"F\")\n\
         DllCall(\"SetWindowsHookEx\", \"Int\", 4, \"UPtr\", cb)\n\
         answer := DllCall(\"CallTheHook\", \"Int\", 7)\n\
         theHook(nCode, wParam, lParam) {\n\
         \x20 global count\n\
         \x20 count += 1\n\
         \x20 Return 42\n\
         }\n",
    );
    assert!(report.ok, "{:?}", report.error);
    // The answer of the function of the script is the answer of the
    // machine: the `CallNextHookEx` of a script of the v1 comes back
    // as the answer of the callback of the script.
    assert_eq!(desktop.answers.borrow().as_slice(), &[42]);
    assert_eq!(
        report.variables.get("answer").map(|value| value.to_text()),
        Some("42".to_string())
    );
    // The function of the script ran once, for the call of the machine.
    assert_eq!(
        report.variables.get("count").map(|value| value.to_text()),
        Some("1".to_string())
    );
}

/// The hook of the messages of the v1 reaches the script when the machine
/// pumps the messages of the thread of the script.
///
/// The hook of a script of the v1 is a `WH_CALLWNDPROC`: the machine
/// calls it for every message of every window of the thread, and the
/// messages are pumped by the runtime of the tree, not by a native call
/// of the script. Measured on the binary of AutoHotkey v1.1.37.02 the callback of the script runs for every message of the
/// thread, and the run of the script goes on after the callback.
#[test]
fn the_hook_of_the_v1_reaches_the_script_when_the_machine_pumps_the_messages() {
    let (report, desktop) = run_and_the_desktop(
        "cb := RegisterCallback(\"theHook\", \"F\")\n\
         DllCall(\"SetWindowsHookEx\", \"Int\", 4, \"UPtr\", cb)\n\
         first := 1\n\
         second := 2\n\
         theHook(nCode, wParam, lParam) {\n\
         \x20 global count\n\
         \x20 count += 1\n\
         \x20 Return 42\n\
         }\n",
    );
    assert!(report.ok, "{:?}", report.error);
    // The machine pumped the messages of the thread, and the hook of the
    // script ran for each of them.
    assert!(
        *desktop.pumps.borrow() > 0,
        "the machine pumped the messages of the thread"
    );
    let answers = desktop.answers.borrow();
    assert!(
        !answers.is_empty(),
        "the machine pumped the messages of the thread"
    );
    // The answer of the callback is the answer of the function of the
    // script: the machine called the hook, and the script answered.
    assert!(
        answers.iter().all(|answer| *answer == 42),
        "the callback of the script ran for the messages of the machine: {answers:?}"
    );
    let count = report
        .variables
        .get("count")
        .map(|value| value.to_text())
        .unwrap_or_default();
    assert_ne!(count, "0", "the function of the script ran for the hook");
}

/// The callback of the machine does not pump the messages of the machine
/// while it runs.
///
/// The callback of the machine re-enters the runtime that is running it:
/// the run of the callback is inside the run that called it, and the pump
/// of the messages of the outer run is the pump of the callback. A second
/// pump would call the hook again, and the callback would run again, and
/// the tree would never come back from the messages of the machine.
#[test]
fn the_callback_of_the_machine_does_not_pump_the_messages_again() {
    let (report, desktop) = run_and_the_desktop(
        "cb := RegisterCallback(\"theHook\", \"F\")\n\
         DllCall(\"SetWindowsHookEx\", \"Int\", 4, \"UPtr\", cb)\n\
         first := 1\n\
         theHook(nCode, wParam, lParam) {\n\
         \x20 global count\n\
         \x20 count += 1\n\
         \x20 Return 0\n\
         }\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(
        *desktop.deepest.borrow() <= 1,
        "the runtime pumped the messages of the machine inside the callback"
    );
}

/// The callback of the machine takes the parameters that the function of
/// the script requires, and not the ones that it declares as optional.
///
/// The `RegisterCallback` without a count takes the count of the
/// function of the script, and the v1 counts the parameters that are
/// required. The tree counted all of them, and a callback got one
/// parameter too many.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the hook of a
/// `WH_GETMESSAGE` whose function declares three parameters and one
/// optional gets three parameters from the machine, and the fourth is
/// empty.
#[test]
fn the_callback_of_the_machine_takes_the_parameters_that_are_required() {
    let (report, _desktop) = run_and_the_desktop(
        "cb := RegisterCallback(\"theHook\", \"F\")\n\
         DllCall(\"SetWindowsHookEx\", \"Int\", 4, \"UPtr\", cb)\n\
         first := 1\n\
         theHook(a, b, c, d:=\"none\") {\n\
         \x20 global seen\n\
         \x20 seen := a \":\" b \":\" c \":\" d\n\
         \x20 Return 42\n\
         }\n",
    );
    assert!(report.ok, "{:?}", report.error);
    // The machine gave the callback four numbers, and the function of the
    // script declares three parameters and one optional: the callback
    // takes the three, and the optional one keeps its own value.
    assert_eq!(
        report
            .variables
            .get("seen")
            .map(|value| value.to_text())
            .unwrap_or_default(),
        "11:22:33:none",
        "the callback of the machine took the wrong parameters: {:?}",
        report.variables
    );
}

/// The hook of the script re-enters the machine, and the runtime keeps its
/// state.
///
/// The function of the script that the machine calls calls the machine
/// again, and the machine calls the hook again: the runtime is entered
/// again while it is running the first call. The runtime is lent to the
/// machine for the time of the call, and the loan is the borrow that the
/// caller made -- a pointer to the runtime would be a second borrow of
/// the same place at the same time, and the state of the script would be
/// in an unknown state when the call comes back.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1 runs the hook
/// of the script on the thread of the script, and the call of the script
/// into the machine is nested in the call of the machine: the nest goes
/// as deep as the script asks for.
#[test]
fn the_hook_of_the_script_re_enters_the_machine_and_the_runtime_keeps_its_state() {
    let (report, desktop) = run_and_the_desktop(
        "cb := RegisterCallback(\"theHook\", \"F\")\n\
         DllCall(\"SetWindowsHookEx\", \"Int\", 4, \"UPtr\", cb)\n\
         before := 1\n\
         theHook(nCode, wParam, lParam) {\n\
         \x20 global depth, deepest\n\
         \x20 depth += 1\n\
         \x20 if (depth > deepest)\n\
         \x20   deepest := depth\n\
         \x20 if (depth < 3)\n\
         \x20   DllCall(\"CallTheHook\", \"Int\", 7)\n\
         \x20 depth -= 1\n\
         \x20 Return 42\n\
         }\n",
    );
    assert!(report.ok, "{:?}", report.error);
    // The hook of the script called the machine again, and the machine
    // called the hook again: the runtime was entered again while it was
    // running the first call.
    let deepest = report
        .variables
        .get("deepest")
        .map(|value| value.to_text())
        .unwrap_or_default();
    assert_eq!(
        deepest, "3",
        "the runtime did not enter the machine again: the variables of the run: {:?}",
        report.variables
    );
    // The answer of the function of the script came back to the machine on
    // every call, the nested ones included.
    let answers = desktop.answers.borrow();
    assert!(
        !answers.is_empty(),
        "the machine called the hook of the script"
    );
    assert!(
        answers.iter().all(|answer| *answer == 42),
        "the answer of the function of the script came back to the machine: {answers:?}"
    );
    // The state of the script is whole: the variable that the run wrote
    // before the hook is still there, and the run went on.
    assert_eq!(
        report
            .variables
            .get("before")
            .map(|value| value.to_text())
            .unwrap_or_default(),
        "1",
        "the state of the script did not survive the call of the machine: {:?}",
        report.variables
    );
}
