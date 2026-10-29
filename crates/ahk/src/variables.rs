//! The built-in variables of AutoHotkey v2.
//!
//! v2 has one hundred and thirty-one of them, and this runtime carries
//! every one of them: the name is answered even when the answer is that
//! the runtime cannot say. A name that is not one of them is not a
//! built-in variable at all, and the runtime says that the variable has
//! not been assigned a value.
//!
//! The names come in four shapes.
//!
//! - The ones the runtime knows without asking anybody: the version of
//!   the dialect, the size of a pointer, the clock, the end of a line.
//! - The ones the runtime keeps as its own state: the coordinate modes,
//!   the delays, the send mode. It keeps none of them yet, so they are
//!   answered with the reason.
//! - The ones that want the date and the time. The clock of the runtime
//!   counts the milliseconds of a run from an arbitrary origin, and it
//!   does not know the date.
//! - The ones that want the machine, the session or the hotkey system.
//!   This runtime carries none of them.
//! - The one that lives outside the script: the clipboard. The desktop
//!   holds it, and the runtime asks it. It is the only built-in variable
//!   of v2 that a script writes and does not keep, and its name has no
//!   `A_` prefix.
//!
//! The name arrives as the script spells it, `A_Now`; the table below
//! carries the name without the `A_` prefix that v2 gives every one of
//! them.

use alloc::format;
use alloc::string::ToString;

use alloc::string::String;

use crate::ast::AssignOp;
use crate::runtime::{Runtime, RuntimeError, Value};

/// The built-in variables the runtime writes where it runs: the loop
/// variables, the line of the statement and the function that runs.
///
/// They are not answered here, because the runtime holds them while it
/// runs. The tree carries them all the same.
pub const THE_RUNTIME_OWNS: &[&str] = &[
    "index", "linefile", "linenumber", "loopfield", "loopfileattrib", "loopfiledir", "loopfileext",
    "loopfilefullpath", "loopfilename", "loopfilepath", "loopfileshortname", "loopfileshortpath",
    "loopfilesize", "loopfilesizekb", "loopfilesizemb", "loopfiletimeaccessed", "loopfiletimecreated",
    "loopfiletimemodified", "loopreadline", "loopregkey", "loopregname", "loopregtimemodified",
    "loopregtype", "thisfunc",
];

/// The version of the dialect this runtime carries.
const AHK_VERSION: &str = "2.0.19";

/// The reason the machine and the session are not answered.
///
/// The desktop has questions for the screen, but none for the user, for
/// the folders or for the idle time, and the runtime carries nothing of
/// its own about them.
const NO_MACHINE: &str = "the machine and the session are not carried by this runtime.";

/// The reason a setting of the runtime is not answered.
///
/// The runtime keeps none of them yet: the coordinate modes, the delays
/// and the send mode are not in it.
const NO_STATE: &str = "the runtime keeps no such state yet.";

/// The built-in variable of this name, if there is one.
///
/// `None` means the name is not a built-in variable of the runtime, and
/// the runtime says that the variable has not been assigned a value.
/// `Some(Err(..))` means the name is one of them and this runtime cannot
/// answer it: the error carries the reason.
///
/// The error carries no line. The caller knows the line of the statement
/// that asked, and puts it there.
pub(crate) fn lookup(name: &str, rt: &mut Runtime) -> Option<Result<Value, RuntimeError>> {
    let name = spelled(name);
    match kind_of(&name)? {
        Kind::Known => Some(Ok(known_value(&name, rt))),
        Kind::State => Some(match crate::settings::value_of(&name) {
            Some(Ok(value)) => Ok(value),
            Some(Err(why)) => Err(reason(&why)),
            None => Err(reason(NO_STATE)),
        }),
        Kind::Date => Some(crate::date::variable_value(&name, rt)),
        Kind::Machine => Some(Err(reason(NO_MACHINE))),
        // The clipboard is the desktop's, and the desktop is asked.
        Kind::Clipboard => Some(match rt.desktop().clipboard_read() {
            Ok(text) => Ok(Value::Text(text)),
            Err(message) => Err(reason(&message)),
        }),
        Kind::Hotkey => crate::hotkeys::variable(&name, rt),
        Kind::Screen => Some(match rt.desktop().screen_size() {
            Ok((width, height)) => {
                Ok(Value::Int(if name == "screenwidth" { width } else { height }))
            }
            Err(message) => Err(reason(&message)),
        }),
    }
}

/// The name of a built-in variable, as the table below carries it.
///
/// The prefix is v2's; the table carries the name without it. The
/// clipboard is the one variable of v2 that has no prefix at all.
fn spelled(name: &str) -> String {
    let lower = name.to_lowercase();
    lower.strip_prefix("a_").unwrap_or(lower.as_str()).to_string()
}

/// The written value of a built-in variable that is not a variable of the
/// script.
///
/// `None` means the name is an ordinary variable of the script, and the
/// runtime keeps it in its own table. `Some` is the value that was
/// written where the variable lives: the clipboard is the one built-in
/// variable of v2 that a script writes and does not keep.
pub(crate) fn write(
    name: &str,
    op: AssignOp,
    value: &Value,
    line: u32,
    rt: &mut Runtime,
) -> Option<Result<Value, RuntimeError>> {
    if spelled(name) != "clipboard" {
        return None;
    }
    Some(write_to_the_clipboard(op, value, line, rt))
}

/// Write a value on the clipboard.
fn write_to_the_clipboard(
    op: AssignOp,
    value: &Value,
    line: u32,
    rt: &mut Runtime,
) -> Result<Value, RuntimeError> {
    match op {
        // `Clipboard := "x"` puts the text on the clipboard.
        AssignOp::Set => put_on_the_clipboard(rt, &value.to_text(), line),
        // `Clipboard .= "x"` adds to what is there.
        AssignOp::Concat => {
            let current = read_the_clipboard(rt, line)?;
            put_on_the_clipboard(rt, &format!("{current}{}", value.to_text()), line)
        }
        // The clipboard holds a text, and the other operators are the
        // ones of the numbers.
        _ => {
            let current = read_the_clipboard(rt, line)?;
            Err(RuntimeError { message: crate::runtime::expected_number(&Value::Text(current)), line })
        }
    }
}

/// The text on the clipboard, or the reason the desktop cannot say.
fn read_the_clipboard(rt: &mut Runtime, line: u32) -> Result<String, RuntimeError> {
    rt.desktop().clipboard_read().map_err(|message| RuntimeError { message, line })
}

/// Put a text on the clipboard.
fn put_on_the_clipboard(rt: &mut Runtime, text: &str, line: u32) -> Result<Value, RuntimeError> {
    rt.desktop()
        .clipboard_write(text)
        .map_err(|message| RuntimeError { message, line })?;
    Ok(Value::Text(text.to_string()))
}

/// Whether the name is a built-in variable of v2.
///
/// The name is the one the script writes: `A_ScreenWidth`. The table
/// below carries the name without the prefix, and a variable of the
/// script that is called `index` is not the built-in `A_Index`.
pub fn is_built_in(name: &str) -> bool {
    let spelled = name.to_lowercase();
    let Some(rest) = spelled.strip_prefix("a_") else {
        // The clipboard is the one built-in variable of v2 that has no
        // prefix: the name it has is `Clipboard`.
        return spelled == "clipboard";
    };
    kind_of(rest).is_some() || THE_RUNTIME_OWNS.contains(&rest)
}

/// The reason the tree does not answer a built-in variable, when it does
/// not.
///
/// `None` means the tree answers the name: the value may be the value of
/// the machine, and the reason of a failure belongs to the question and
/// not to the name.
pub fn refusal_of(name: &str) -> Option<&'static str> {
    let name = spelled(name);
    match kind_of(&name)? {
        // The hotkeys answer with a value of their own, and the date
        // answers when the clock of the host knows it.
        Kind::Known | Kind::Date | Kind::Hotkey | Kind::Screen | Kind::Clipboard => None,
        Kind::State => Some(NO_STATE),
        Kind::Machine => Some(NO_MACHINE),
    }
}

/// Whether the tree carries the built-in variable of this name.
///
/// The name is carried when this module answers it, or when the runtime
/// writes it where it runs.
pub fn carries(name: &str) -> bool {
    let name = spelled(name);
    kind_of(&name).is_some() || THE_RUNTIME_OWNS.contains(&name.as_str())
}

/// The kind of a built-in variable.
///
/// `None` means the name is not a built-in variable of the runtime.
fn kind_of(name: &str) -> Option<Kind> {
    match name {
        // -- the ones the runtime knows ------------------------------------------
        "ahkversion" | "is64bitos" | "ptrsize" | "iscompiled" | "iscritical" | "ispaused"
        | "issuspended" | "tickcount" | "endchar" | "space" | "tab" => Some(Kind::Known),
        // -- the settings the runtime keeps ---------------------------------------
        "allowmainwindow" | "coordmodecaret" | "coordmodemenu" | "coordmodemouse" | "coordmodepixel"
        | "coordmodetooltip" | "defaultmousespeed" | "fileencoding" | "keydelay" | "keydelayplay"
        | "keyduration" | "keydurationplay" | "listlines" | "menumaskkey" | "mousedelay"
        | "mousedelayplay" | "sendlevel" | "sendmode" | "storecapslockmode" => Some(Kind::State),
        // -- the date and the time ------------------------------------------------
        "dd" | "ddd" | "dddd" | "hour" | "mday" | "min" | "mm" | "mmm" | "mmmm" | "mon" | "msec"
        | "now" | "nowutc" | "sec" | "wday" | "yday" | "year" | "yweek" | "yyyy" => Some(Kind::Date),
        // -- the machine and the session -------------------------------------------
        "ahkpath" | "appdata" | "appdatacommon" | "computername" | "comspec"
        | "controldelay" | "cursor" | "desktop" | "desktopcommon" | "detecthiddentext"
        | "detecthiddenwindows" | "eventinfo" | "iconfile" | "iconhidden" | "iconnumber" | "icontip"
        | "initialworkingdir" | "isadmin" | "language" | "lasterror" | "mydocuments" | "osversion"
        | "programfiles" | "programs" | "programscommon" | "regview" | "screendpi"
        | "scriptdir" | "scriptfullpath" | "scripthwnd" | "scriptname" | "startmenu"
        | "startmenucommon" | "startup" | "startupcommon" | "temp" | "timeidle" | "timeidlekeyboard"
        | "timeidlemouse" | "timeidlephysical" | "titlematchmode" | "titlematchmodespeed" | "traymenu"
        | "username" | "windelay" | "windir" | "workingdir" => Some(Kind::Machine),
        // -- the clipboard -----------------------------------------------------------
        "clipboard" => Some(Kind::Clipboard),
        // -- the screen ---------------------------------------------------------------
        "screenheight" | "screenwidth" => Some(Kind::Screen),
        // -- the hotkeys ------------------------------------------------------------
        "hotkeyinterval" | "hotkeymodifiertimeout" | "maxhotkeysperinterval" | "priorhotkey"
        | "priorkey" | "thishotkey" | "timesincepriorhotkey" | "timesincethishotkey" => {
            Some(Kind::Hotkey)
        }
        _ => None,
    }
}

/// What a built-in variable is.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// The runtime knows it without asking anybody.
    Known,
    /// A setting the runtime keeps as its own state.
    State,
    /// A date or a time.
    Date,
    /// The machine, the session or the user.
    Machine,
    /// The clipboard of the desktop.
    Clipboard,
    /// The hotkey system.
    Hotkey,
    /// The screen of the desktop.
    Screen,
}

/// The value of a built-in variable the runtime knows.
fn known_value(name: &str, rt: &Runtime) -> Value {
    match name {
        "ahkversion" => Value::Text(AHK_VERSION.to_string()),
        "is64bitos" => Value::Int((core::mem::size_of::<usize>() == 8) as i64),
        "ptrsize" => Value::Int(core::mem::size_of::<usize>() as i64),
        // The runner runs a script: it is not a compiled program. And the
        // runtime is never critical, never paused and never suspended.
        "iscompiled" | "iscritical" | "ispaused" | "issuspended" => Value::Int(0),
        // The clock of the runtime: the milliseconds since an arbitrary
        // origin, and never backwards.
        "tickcount" => Value::Int(rt.now_ms() as i64),
        "endchar" => Value::Text("\r".to_string()),
        "space" => Value::Text(" ".to_string()),
        "tab" => Value::Text("\t".to_string()),
        _ => Value::Text(String::new()),
    }
}

/// The error of a built-in variable this runtime cannot answer.
///
/// The line is zero: the caller knows the line of the statement that asked,
/// and puts it there.
fn reason(message: &str) -> RuntimeError {
    RuntimeError { message: message.to_string(), line: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{
        Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen,
        Sound, Timers, Windows,
    };
    /// The reason the date and the time are not answered.
    ///
    /// The clock of the runtime counts the milliseconds of a run, and it
    /// counts them from an arbitrary origin: it does not know the date.
    use crate::date::NO_DATE;
    use crate::runtime::Clock;
    use alloc::string::String;
    use core::time::Duration;

    /// The surface of v2.0.19, read out of the binary.
    const SURFACE: &str = include_str!("../reference/ahk-v2.0.19.surface.tsv");

    /// A desktop that answers nothing: this module asks it no question.
    #[derive(Debug)]
    struct Fake;

    impl Desktop for Fake {}
    impl Sound for Fake {}

    impl Files for Fake {}

    impl Clipboard for Fake {}

    impl Timers for Fake {}

    impl Hotkeys for Fake {}

    impl Windows for Fake {}

    impl Native for Fake {}

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

    impl Processes for Fake {
        fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
            Ok((0, String::new(), String::new()))
        }
    }

    /// The clock of the test: it counts from an arbitrary origin, like the
    /// clock of a runtime does.
    #[derive(Debug)]
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            1234
        }
        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// The answer of the built-in variable of this name.
    fn answer(name: &str) -> Option<Result<Value, RuntimeError>> {
        let mut desktop = Fake;
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        lookup(name, &mut rt)
    }

    /// The value of the built-in variable of this name.
    fn value(name: &str) -> Value {
        match answer(name) {
            Some(Ok(value)) => value,
            Some(Err(_)) => panic!("{name} answered with a reason instead of a value"),
            None => panic!("{name} is not a built-in variable"),
        }
    }

    /// The reason the built-in variable of this name is answered with.
    fn reason(name: &str) -> String {
        match answer(name) {
            Some(Err(error)) => error.message,
            Some(Ok(value)) => panic!("{name} answered {value:?} instead of a reason"),
            None => panic!("{name} is not a built-in variable"),
        }
    }

    /// The variable names of the surface of v2.0.19, in lower case.
    fn surface_variables() -> Vec<String> {
        SURFACE
            .lines()
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.first() == Some(&"variable") {
                    fields.get(2).map(|name| name.to_lowercase())
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn the_surface_has_one_hundred_and_thirty_one_variables() {
        assert_eq!(surface_variables().len(), 131);
    }

    #[test]
    fn a_script_that_asks_for_the_version_is_told_the_version_of_the_dialect() {
        assert_eq!(value("a_ahkversion"), Value::Text("2.0.19".into()));
    }

    #[test]
    fn a_script_that_asks_for_the_size_of_a_pointer_is_told_the_size_of_a_pointer() {
        let size = core::mem::size_of::<usize>() as i64;
        assert_eq!(value("a_ptrsize"), Value::Int(size));
        assert_eq!(value("a_is64bitos"), Value::Int((size == 8) as i64));
    }

    #[test]
    fn a_script_that_asks_whether_the_program_is_compiled_is_told_it_is_not() {
        assert_eq!(value("a_iscompiled"), Value::Int(0));
    }

    #[test]
    fn a_script_that_asks_whether_the_runtime_is_critical_paused_or_suspended_is_told_it_is_not() {
        assert_eq!(value("a_iscritical"), Value::Int(0));
        assert_eq!(value("a_ispaused"), Value::Int(0));
        assert_eq!(value("a_issuspended"), Value::Int(0));
    }

    #[test]
    fn a_script_that_asks_for_the_tick_count_is_told_the_milliseconds_of_the_clock() {
        assert_eq!(value("a_tickcount"), Value::Int(1234));
    }

    #[test]
    fn a_script_that_asks_for_the_end_of_a_line_is_told_the_carriage_return() {
        assert_eq!(value("a_endchar"), Value::Text("\r".into()));
    }

    #[test]
    fn a_script_that_asks_for_the_space_and_the_tab_is_told_them() {
        assert_eq!(value("a_space"), Value::Text(" ".into()));
        assert_eq!(value("a_tab"), Value::Text("\t".into()));
    }

    #[test]
    fn the_name_of_a_built_in_variable_does_not_matter() {
        assert_eq!(value("A_AhkVersion"), value("a_ahkversion"));
        assert_eq!(value("a_AHKVERSION"), value("a_ahkversion"));
    }

    #[test]
    fn a_script_that_asks_for_the_date_is_told_the_clock_does_not_know_it() {
        assert_eq!(reason("a_now"), NO_DATE);
        assert_eq!(reason("a_nowutc"), NO_DATE);
        assert_eq!(reason("a_msec"), NO_DATE);
        assert_eq!(reason("a_year"), NO_DATE);
    }

    #[test]
    fn a_script_that_asks_for_the_machine_is_told_the_runtime_does_not_carry_it() {
        assert_eq!(reason("a_computername"), NO_MACHINE);
        assert_eq!(reason("a_username"), NO_MACHINE);
    }

    #[test]
    fn a_script_that_asks_for_the_size_of_the_screen_is_told_by_the_desktop() {
        // The screen is not the machine: it is the desktop's, and the tree
        // asks it.
        assert_eq!(reason("a_screenwidth"), "this desktop cannot tell how big the screen is");
        assert_eq!(reason("a_screenheight"), "this desktop cannot tell how big the screen is");
    }

    #[test]
    fn a_script_that_asks_for_a_hotkey_is_answered_by_the_hotkey_system() {
        // The hotkeys are carried: the numbers are the ones of v2,
        // measured on the binary on 2026-10-28.
        assert_eq!(value("a_hotkeyinterval"), Value::Int(2000));
        assert_eq!(value("a_hotkeymodifiertimeout"), Value::Int(50));
        assert_eq!(value("a_maxhotkeysperinterval"), Value::Int(70));
        // Until a hotkey fires, the two names are empty.
        assert_eq!(value("a_thishotkey"), Value::Text(String::new()));
        assert_eq!(value("a_priorkey"), Value::Text(String::new()));
    }

    #[test]
    fn a_script_that_asks_for_a_setting_is_answered_with_the_setting() {
        // The settings of the script are answered: the module of the
        // settings keeps them, and the three that are the coordinates of
        // a window answer with the reason.
        let _turn = crate::settings::Turn::take();
        assert_eq!(value("a_coordmodemouse"), Value::Text("Client".to_string()));
        assert_eq!(value("a_sendmode"), Value::Text("Input".to_string()));
        assert_eq!(value("a_keydelay"), Value::Int(10));
        assert!(reason("a_coordmodecaret").contains("window"));
    }

    #[test]
    fn the_variables_the_runtime_writes_where_it_runs_are_not_answered_here() {
        assert!(answer("a_index").is_none());
        assert!(answer("a_loopfilename").is_none());
        assert!(answer("a_thisfunc").is_none());
    }

    #[test]
    fn every_variable_of_the_surface_is_answered_or_is_the_runtime_s_own() {
        let lost: Vec<String> = surface_variables()
            .into_iter()
            .filter(|name| !THE_RUNTIME_OWNS.contains(&name.as_str()))
            .filter(|name| answer(name).is_none())
            .collect();
        assert!(lost.is_empty(), "these variables of v2 are not answered: {lost:?}");
    }
}
