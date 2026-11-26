//! The `OnMessage` of the v1: the monitor that the script registers by
//! name.
//!
//! The v1 keeps two kinds of handler for a message: the **legacy** one,
//! which the script registers by the name of a function of the script, and
//! the ones that are objects. The legacy one is at most one for each
//! number of message, and the name that the v1 keeps is the name of the
//! function -- not the spelling that the script wrote.
//!
//! The cases are about the four things that a script of the v1 needs
//! of it:
//!
//! - the registration answers the name of the monitor that was there
//!   before, and the name of the new one when there was none;
//! - the query answers the name of the monitor, and the empty text when
//!   there is none;
//! - the empty name takes the monitor away, and so does a `MaxThreads` of
//!   zero;
//! - a callback that the v1 refuses answers the empty text, and the
//!   failure is silent: the v1 does not raise an error for a name.
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

/// A desktop that hands the messages of the windows of the script to the
/// runtime, the way the machine does.
struct Fake {
    /// The messages that the machine has for the script, and that arrive
    /// when the script watches their number.
    ///
    /// The machine does not deliver a message before the script watches
    /// it: the monitor of the script is what gives the message its
    /// handler.
    pending: RefCell<Vec<String>>,
    /// The messages that arrived at the windows of the script.
    messages: RefCell<Vec<String>>,
    /// The numbers that the script watches with `OnMessage`.
    watched: RefCell<Vec<i64>>,
}

impl Fake {
    /// A desktop that has not been asked anything yet.
    fn new() -> Self {
        Fake {
            pending: RefCell::new(Vec::new()),
            messages: RefCell::new(Vec::new()),
            watched: RefCell::new(Vec::new()),
        }
    }
}

impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        Ok(NativeResult {
            kind: call.returns,
            value: NativeValue::Integer(0),
        })
    }
}

impl Windows for Fake {
    /// The messages that arrived at the windows of the script.
    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        Ok(self.messages.borrow_mut().drain(..).collect())
    }

    /// The script watches a message: the machine remembers the number, and
    /// the procedure of the window of the script gives the message to the
    /// monitor when it arrives.
    fn watch_a_message(&mut self, number: i64) -> Result<(), String> {
        {
            let mut watched = self.watched.borrow_mut();
            if !watched.contains(&number) {
                watched.push(number);
            }
        }
        // The message that the machine had for that number arrives now.
        let mut pending = self.pending.borrow_mut();
        let mut arrived = self.messages.borrow_mut();
        pending.retain(|message| {
            let number_of_the_message = message
                .strip_prefix("message:")
                .and_then(|rest| rest.split(':').next())
                .and_then(|number| number.parse::<i64>().ok());
            if number_of_the_message == Some(number) {
                arrived.push(message.clone());
                false
            } else {
                true
            }
        });
        Ok(())
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

/// The text of a variable of the script, when the run wrote it.
fn the_text_of(report: &Report, name: &str) -> String {
    report
        .variables
        .get(name)
        .map(|value| value.to_text())
        .unwrap_or_default()
}

/// The legacy monitor of the v1 keeps the name of the function of the
/// script, and the registration answers the name of the monitor that was
/// there before.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// query of a message that has no monitor answers the empty text, the
/// first registration answers the name of the new monitor, and the
/// registration that changes it answers the name of the old one. The name
/// that the v1 keeps is the name of the function: `OnMessage(0x321,
/// "myhandler")` registers `MyHandler`.
#[test]
fn the_legacy_monitor_of_the_v1_keeps_the_name_of_the_function() {
    let (report, _desktop) = run_and_the_desktop(
        "MyHandler(wP, lP, m, hwnd) {\n\
         \x20 return 7\n\
         }\n\
         OtherHandler(wP, lP, m, hwnd) {\n\
         \x20 return 8\n\
         }\n\
         empty := OnMessage(0x321)\n\
         first := OnMessage(0x321, \"MyHandler\")\n\
         query := OnMessage(0x321)\n\
         update := OnMessage(0x321, \"OtherHandler\")\n\
         after := OnMessage(0x321)\n\
         lower := OnMessage(0x322, \"myhandler\")\n\
         lowerquery := OnMessage(0x322)\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(the_text_of(&report, "empty"), "");
    assert_eq!(the_text_of(&report, "first"), "MyHandler");
    assert_eq!(the_text_of(&report, "query"), "MyHandler");
    // The registration that changes the monitor answers the name of the
    // old one, and the query answers the name of the new one.
    assert_eq!(the_text_of(&report, "update"), "MyHandler");
    assert_eq!(the_text_of(&report, "after"), "OtherHandler");
    // The name that the v1 keeps is the name of the function, and not the
    // spelling that the script wrote.
    assert_eq!(the_text_of(&report, "lower"), "MyHandler");
    assert_eq!(the_text_of(&report, "lowerquery"), "MyHandler");
}

/// The empty name takes the monitor away, and the v1 answers the name that
/// it had.
///
/// Measured on the binary of AutoHotkey v1.1.37.02:
/// `OnMessage(0x321, "")` answers the name of the monitor that it took
/// away, and the empty text when there was none. A `MaxThreads` of zero
/// takes the monitor away as well, and answers the same name.
#[test]
fn the_empty_name_takes_the_legacy_monitor_away() {
    let (report, _desktop) = run_and_the_desktop(
        "MyHandler(wP, lP, m, hwnd) {\n\
         \x20 return 7\n\
         }\n\
         OnMessage(0x321, \"MyHandler\")\n\
         taken := OnMessage(0x321, \"\")\n\
         query := OnMessage(0x321)\n\
         OnMessage(0x322, \"MyHandler\")\n\
         zero := OnMessage(0x322, \"MyHandler\", 0)\n\
         afterZero := OnMessage(0x322)\n\
         none := OnMessage(0x323, \"\")\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(the_text_of(&report, "taken"), "MyHandler");
    assert_eq!(the_text_of(&report, "query"), "");
    // The `MaxThreads` of zero takes the monitor away, and answers the name
    // that it had.
    assert_eq!(the_text_of(&report, "zero"), "MyHandler");
    assert_eq!(the_text_of(&report, "afterZero"), "");
    // The empty name of a message that has no monitor answers the empty
    // text.
    assert_eq!(the_text_of(&report, "none"), "");
}

/// The callback that the v1 refuses answers the empty text, and the
/// failure is silent.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: a name
/// that is not a function of the script, a label, a function that needs
/// more than four parameters, and a number are all refused: the answer is
/// the empty text, the run does not fail, and no monitor is registered.
#[test]
fn the_callback_that_the_v1_refuses_answers_the_empty_text() {
    let (report, _desktop) = run_and_the_desktop(
        "MyHandler(wP, lP, m, hwnd) {\n\
         \x20 return 7\n\
         }\n\
         TooMany(a, b, c, d, e) {\n\
         \x20 return 9\n\
         }\n\
         unknown := OnMessage(0x321, \"NoSuchHandler\")\n\
         label := OnMessage(0x322, \"MyLabel\")\n\
         number := OnMessage(0x323, 5)\n\
         tooMany := OnMessage(0x324, \"TooMany\")\n\
         afterUnknown := OnMessage(0x321)\n\
         afterLabel := OnMessage(0x322)\n\
         afterNumber := OnMessage(0x323)\n\
         afterTooMany := OnMessage(0x324)\n\
         MyLabel:\n\
         \x20 return\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(the_text_of(&report, "unknown"), "");
    assert_eq!(the_text_of(&report, "label"), "");
    assert_eq!(the_text_of(&report, "number"), "");
    assert_eq!(the_text_of(&report, "tooMany"), "");
    // The refusal does not register anything.
    assert_eq!(the_text_of(&report, "afterUnknown"), "");
    assert_eq!(the_text_of(&report, "afterLabel"), "");
    assert_eq!(the_text_of(&report, "afterNumber"), "");
    assert_eq!(the_text_of(&report, "afterTooMany"), "");
}

/// The monitor of the v1 runs when the message arrives at a window of the
/// script, and it takes the four parameters of the reference.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// handler is called with (wParam, lParam, msg, hwnd), the shape that the
/// a script of the v1 expects.
#[test]
fn the_monitor_of_the_v1_runs_with_the_four_parameters_of_the_reference() {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(
        "MyHandler(wP, lP, m, hwnd) {\n\
         \x20 global seen\n\
         \x20 seen := wP \":\" lP \":\" m \":\" hwnd\n\
         \x20 return 7\n\
         }\n\
         OnMessage(0x4555, \"MyHandler\")\n\
         first := 1\n",
        dialect::v1(),
    )
    .expect("the script of the v1 parses");
    let mut desktop = Fake::new();
    // The message arrives at a window of the script: the number is the one
    // that the monitor watches, and the four parameters travel with it.
    desktop
        .pending
        .borrow_mut()
        .push("message:17749:111:222:95354916".to_string());
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        the_text_of(&report, "seen"),
        "111:222:17749:95354916",
        "the variables of the run: {:?}",
        report.variables
    );
    // The machine was asked to watch the number of the message.
    assert_eq!(desktop.watched.borrow().as_slice(), &[17749]);
}

/// The handler of a message that is an object takes only the parameters
/// that it declares.
///
/// The v1 keeps two kinds of handler for a message: the one registered
/// by name and the one that is an object, and the two take the
/// parameters that they declare.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the object handler
/// that takes none runs with no parameter.
#[test]
fn the_object_handler_of_a_message_takes_only_the_parameters_that_it_declares() {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(
        "Short() {\n\
         \x20 global seen\n\
         \x20 seen := \"short\"\n\
         \x20 return 7\n\
         }\n\
         OnMessage(0x4555, Func(\"Short\"))\n\
         first := 1\n",
        dialect::v1(),
    )
    .expect("the script of the v1 parses");
    let mut desktop = Fake::new();
    // The message arrives at a window of the script: the number is the one
    // that the object handler watches.
    desktop
        .pending
        .borrow_mut()
        .push("message:17749:111:222:95354916".to_string());
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    assert!(report.ok, "{:?}", report.error);
    // The handler that takes none ran: the v1 does not refuse it.
    assert_eq!(
        the_text_of(&report, "seen"),
        "short",
        "the object handler of the message did not run: {:?}",
        report.variables
    );
}

/// The callback that is an object answers the empty text, and the monitor
/// of the object is registered.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// answer of the registration of an object is the empty text, and the
/// query of the message answers the empty text as well: the query is about
/// the legacy monitor, and an object is not one.
#[test]
fn the_callback_that_is_an_object_answers_the_empty_text() {
    let (report, desktop) = run_and_the_desktop(
        "MyHandler(wP, lP, m, hwnd) {\n\
         \x20 return 7\n\
         }\n\
         f := Func(\"MyHandler\")\n\
         answer := OnMessage(0x321, f)\n\
         query := OnMessage(0x321)\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(the_text_of(&report, "answer"), "");
    assert_eq!(the_text_of(&report, "query"), "");
    assert_eq!(desktop.watched.borrow().as_slice(), &[801]);
}

/// The handler of a message takes only the parameters that it declares.
///
/// The v1 calls the handler of a message with the four parameters of the
/// message -- (wParam, lParam, msg, hwnd) -- and a handler that declares
/// fewer takes only the first ones. The tree gave the four of them to
/// every handler, and a handler that takes none was refused with "Too
/// many parameters passed to function." -- the script died with it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the handler that
/// takes none runs, and the one that takes one takes the wParam of the
/// message.
#[test]
fn the_handler_of_a_message_takes_only_the_parameters_that_it_declares() {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(
        "Short() {\n\
         \x20 global seen\n\
         \x20 seen := \"short\"\n\
         \x20 return 7\n\
         }\n\
         One(wP) {\n\
         \x20 global seenOne\n\
         \x20 seenOne := wP\n\
         \x20 return 7\n\
         }\n\
         OnMessage(0x4555, \"Short\")\n\
         OnMessage(0x4556, \"One\")\n\
         first := 1\n",
        dialect::v1(),
    )
    .expect("the script of the v1 parses");
    let mut desktop = Fake::new();
    // The messages arrive at a window of the script: the numbers are the
    // ones that the monitors watch.
    desktop
        .pending
        .borrow_mut()
        .push("message:17749:111:222:95354916".to_string());
    desktop
        .pending
        .borrow_mut()
        .push("message:17750:333:444:95354917".to_string());
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    assert!(report.ok, "{:?}", report.error);
    // The handler that takes none ran: the v1 does not refuse it.
    assert_eq!(
        the_text_of(&report, "seen"),
        "short",
        "the handler of the message did not run: {:?}",
        report.variables
    );
    // The handler that takes one took the wParam of the message.
    // The names of the variables of the run are in lower case.
    assert_eq!(
        the_text_of(&report, "seenone"),        "333",
        "the handler of the message took the wrong parameter: {:?}",
        report.variables
    );
    assert_eq!(
        desktop.watched.borrow().as_slice(),
        &[17749, 17750],
        "the machine was asked to watch the numbers of the messages"
    );
}
