//! The built-in variables of AutoHotkey v2.
//!
//! v2 has one hundred and thirty-one of them, and this runtime carries
//! every one of them: the name is answered even when the answer is that
//! the runtime cannot say. A name that is not one of them is not a
//! built-in variable at all, and the runtime says that the variable has
//! not been assigned a value.
//!
//! The names come in five shapes.
//!
//! - The ones the runtime knows without asking anybody: the version of
//!   the dialect, the size of a pointer, the clock, the end of a line.
//! - The ones the runtime keeps as its own state: the coordinate modes,
//!   the delays, the send mode, the mode of the matching of the titles.
//!   They are the settings of v2, and the module of the settings keeps
//!   them.
//! - The ones that want the date and the time. The clock of the runtime
//!   counts the milliseconds of a run from an arbitrary origin, and it
//!   does not know the date.
//! - The ones that want the machine, the session or the hotkey system.
//!   The machine and the session are the desktop's: the user, the
//!   computer, the shell, the folders of the machine, the version of the
//!   operating system, the language, the elevation and the idle times are
//!   the ones of the machine that runs the script, and the names that the
//!   tree does not carry refuse with the reason of the name. The hotkeys
//!   are the ones of the hotkey system of the runtime.
//! - The one that lives outside the script: the clipboard. The desktop
//!   holds it, and the runtime asks it. Its name is `A_Clipboard`, like
//!   the name of every other one of them.
//!
//! A script writes some of the names, and the value that was written is
//! the value the name keeps. The settings are written where they live,
//! the names of the table of the writes keep the value in the table of
//! the script, and the names that the dialect does not let a script write
//! are refused.
//!
//! The name arrives as the script spells it, `A_Now`; the table below
//! carries the name without the `A_` prefix that v2 gives every one of
//! them. A name the script writes without the prefix is not a built-in
//! variable of the dialect at all: it is a variable of the script, and
//! the runtime says that it has no value.

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

/// The reason the names of Windows are not answered.
///
/// The two names are about the folders of Windows, and the desktop of the
/// tree is not the one of Windows.
const NOT_WINDOWS: &str = "this is a name of Windows, and this desktop is not Windows.";

/// The reason the short names of the files are not answered.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
/// `A_LoopFileShortName` is the name that Windows gives to a file that has a
/// short one, and this desktop has no such names.
pub(crate) const NO_SHORT_NAMES_OF_THE_FILES: &str =
    "the short names of the files are a thing of Windows, and this desktop is not Windows.";

/// The reason a folder of the machine is not there.
///
/// The folders are the ones the machine has, and the ones it does not have
/// are not answered.
const NO_SUCH_FOLDER: &str = "the machine has no such folder.";

/// The reason the desktop of everybody is not there.
///
/// The desktop of the session is the one of the user, and there is no
/// other one.
const NO_DESKTOP_COMMON: &str =
    "the machine has no desktop for everybody: the desktop of the session is the one of the user.";

/// The reason the programs of the user are not there.
///
/// The start menu of the user is the folder of the applications.
const NO_PROGRAMS: &str =
    "the machine has no programs folder: the start menu of the user is the folder of the applications.";

/// The reason the programs of everybody are not there.
///
/// The start menu is the folder of the applications.
const NO_PROGRAMS_COMMON: &str =
    "the machine has no programs folder for everybody: the start menu is the folder of the applications.";

/// The reason the path of the executable is not answered.
///
/// This runtime is not AutoHotkey, and it has no executable of its own.
const NO_AHK_PATH: &str = "this runtime is not AutoHotkey, and it has no path of an executable.";

/// The reason the window of the script is not answered.
///
/// The runtime is a script that runs and ends, and it has no window.
const NO_SCRIPT_HWND: &str = "this runtime has no window of its own: it is a script that runs and ends.";

/// The reason the scale of the output is not answered.
///
/// The desktop does not know the scale of the screen it shows.
const NO_SCREEN_DPI: &str = "this desktop does not know the scale of the output.";

/// The reason the tray icon is not answered.
///
/// The runtime has no tray icon.
const NO_TRAY_ICON: &str = "this runtime has no tray icon.";

/// The reason the directory where the run started is not answered.
///
/// The runtime does not keep it.
const NO_INITIAL_WORKING_DIR: &str = "this runtime does not keep the directory where the run started.";

/// The reason a built-in variable cannot be written.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the dialect refuses
/// the name when the script writes it, and it names the name as the script
/// spelled it: "This built-in variable cannot be used as an output
/// variable.\n     Specifically: A_AhkVersion".
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the compound operator
/// says that the variable cannot be assigned a value, and the plain one
/// says that it cannot be an output variable.
pub(crate) fn read_only(name: &str, op: AssignOp) -> String {
    match op {
        AssignOp::Set => {
            format!("This built-in variable cannot be used as an output variable.\n     Specifically: {name}")
        }
        _ => format!("This built-in variable cannot be assigned a value.\n     Specifically: {name}"),
    }
}

/// The built-in variables that the dialect lets a script write.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the surface of v2
/// carries a hundred and thirty-one built-in variables, and the v2 refuses
/// ninety-four of them when a script writes them. These are the thirty-
/// seven that it lets a script write. The names are the ones of the table
/// of the surface, without the prefix `A_` that the dialect gives them.
const WRITABLE: &[&str] = &[
    // The two names that live outside the script: the clipboard and the
    // directory where the run is.
    "clipboard", "workingdir",
    // The names of the table of the writes: the name of the script, the
    // tip of the icon, the information of an event, the last error of the
    // operating system, and the three of the hotkeys.
    "eventinfo", "hotkeyinterval", "hotkeymodifiertimeout", "iconhidden", "icontip",
    "lasterror", "maxhotkeysperinterval", "scriptname",
    // The settings of the run: the mode of the coordinates, the delays,
    // the mode of the matching of the titles. The shape of the cursor is
    // not one of them: the v2 refuses to write it as well.
    "allowmainwindow", "controldelay", "coordmodecaret", "coordmodemenu", "coordmodemouse",
    "coordmodepixel", "coordmodetooltip", "defaultmousespeed", "detecthiddentext",
    "detecthiddenwindows", "fileencoding", "keydelay", "keydelayplay", "keyduration",
    "keydurationplay", "listlines", "menumaskkey", "mousedelay", "mousedelayplay", "regview",
    "sendlevel", "sendmode", "storecapslockmode", "titlematchmode", "titlematchmodespeed",
    "windelay",
    // The index of the loop is a name that a script may write.
    "index",
];

/// Whether the name is a built-in variable that the dialect does not let a
/// script write.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the refusal is the one
/// of the load, and it is the one of every scope -- a local variable may
/// not carry the name either. The body of a class is the one place where
/// the name is a property and not a variable.
pub(crate) fn is_read_only(name: &str) -> bool {
    let Some(spelled) = spelled(name) else { return false };
    is_built_in(name) && !WRITABLE.contains(&spelled.as_str())
}

/// The reason a name of the machine is not answered.
///
/// The table of the names and the table of the questions are the same one,
/// and a name that is in neither of them is a name the tree does not
/// carry.
const NO_SUCH_MACHINE_NAME: &str = "this runtime does not carry this name of the machine.";

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
    let name = spelled(name)?;
    match kind_of(&name)? {
        Kind::Known => Some(Ok(known_value(&name, rt))),
        Kind::State => Some(match crate::settings::value_of(&name) {
            Some(Ok(value)) => Ok(value),
            Some(Err(why)) => Err(reason(why)),
            None => Err(reason(crate::settings::NO_STATE)),
        }),
        Kind::Date => Some(crate::date::variable_value(&name, rt)),
        // The machine is the desktop's, and the desktop is asked.
        Kind::Machine => Some(machine_value(&name, rt)),
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
/// The dialect gives every built-in variable the prefix `A_`, and the
/// table carries the name without it. `None` means the name the script
/// wrote is not the name of a built-in variable of the dialect: it is a
/// variable of the script, and the runtime says that it has no value.
///
/// The name that comes back is in the lower case, the way the table of the
/// surface carries it.
pub(crate) fn spelled(name: &str) -> Option<String> {
    let lower = name.to_lowercase();
    lower.strip_prefix("a_").map(|rest| rest.to_string())
}

/// The name of a built-in variable, as the table below carries it.
///
/// The table of the surface stores the names without the prefix, and the
/// surface and the tests ask with those names. A name that carries the
/// prefix is accepted as well: the two conventions meet here, and
/// nowhere else.
fn in_the_table(name: &str) -> String {
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
    if is_read_only(name) {
        return Some(Err(reason(&read_only(name, op))));
    }
    match spelled(name).as_deref() {
        Some("clipboard") => Some(write_to_the_clipboard(op, value, line, rt)),
        // The working directory of the run is the one of the desktop, and
        // the script changes it by writing the name.
        Some("workingdir") => Some(write_the_working_directory(op, value, line, rt)),
        // A name that is not a built-in variable of the dialect is a
        // variable of the script.
        _ => None,
    }
}

/// What the dialect does with the value that is written to a built-in
/// variable.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a name that takes any
/// value keeps it, a name that is a number takes the number of the value,
/// a name that is a flag takes the truth of it, and a name that wants a
/// number refuses a text that holds no number.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Written {
    /// The value is kept as it was written.
    Any,
    /// The value is a number: a text holds the number it carries.
    Number,
    /// The value is a number that does not go below zero: the last error of
    /// the operating system is a number of thirty-two bits.
    Unsigned,
    /// The value is a flag: it is one when the value is true.
    Flag,
    /// The value is a number, and a text that holds no number is refused.
    /// The two bounds are the ones of the name.
    Strict { lowest: i64, highest: i64 },
}

/// The kind of a built-in variable that the dialect lets a script write.
///
/// `None` means the name is not one of them: the dialect refuses the write
/// of the name, and the tree says so.
pub(crate) fn the_kind_of_the_write(name: &str) -> Option<Written> {
    let name = spelled(name)?;
    Some(match name.as_str() {
        // The name of the script, the tip of the icon and the timeout of
        // the change of a hotkey take any value, and they keep it.
        "scriptname" | "icontip" | "hotkeymodifiertimeout" => Written::Any,
        // The information of an event is a number, and the last error of
        // the operating system is a number of thirty-two bits.
        "eventinfo" => Written::Number,
        "lasterror" => Written::Unsigned,
        // The icon of the script is hidden or it is not.
        "iconhidden" => Written::Flag,
        // The interval of the hotkeys and the number of the hotkeys are
        // numbers, and a text that holds no number is refused.
        "hotkeyinterval" => Written::Strict { lowest: 0, highest: 2147483647 },
        "maxhotkeysperinterval" => Written::Strict { lowest: 1, highest: 2147483647 },
        _ => return None,
    })
}

/// The value that a built-in variable keeps, the way the dialect keeps it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a text that holds a
/// number is that number, a text that holds no number is refused with the
/// kind of the value that arrived, and a number that is out of the range of
/// the name is a value that is not valid.
pub(crate) fn the_written_value(
    kind: Written,
    value: &Value,
    line: u32,
) -> Result<Value, RuntimeError> {
    Ok(match kind {
        Written::Any => value.clone(),
        // The number of a value: a text holds the number it carries, and a
        // float is a whole number.
        Written::Number => Value::Int(value.to_number_lenient() as i64),
        // The last error is a number of thirty-two bits, and it wraps
        // around: -1 is 4294967295.
        Written::Unsigned => Value::Int(value.to_number_lenient() as i64 as u32 as i64),
        // The flag is the truth of the value.
        Written::Flag => Value::Int(value.truthy() as i64),
        Written::Strict { lowest, highest } => {
            // A text that holds no number is refused, and the kind of the
            // value that arrived is named.
            let number = value.to_number_strict().map_err(|_| RuntimeError {
                message: crate::runtime::expected_number(value),
                line,
            })? as i64;
            if number < lowest || number > highest {
                return Err(RuntimeError {
                    message: crate::settings::INVALID_VALUE.to_string(),
                    line,
                });
            }
            Value::Int(number)
        }
    })
}

/// Change the working directory of the run.
///
/// The directory is the one of the desktop, and the assignment answers the
/// value that was written.
fn write_the_working_directory(
    op: AssignOp,
    value: &Value,
    line: u32,
    rt: &mut Runtime,
) -> Result<Value, RuntimeError> {
    match op {
        AssignOp::Set => {}
        _ => {
            return Err(RuntimeError {
                message: "the working directory is set with the assignment, and the other operators do not make a path of it.".into(),
                line,
            })
        }
    }
    let path = value.to_text();
    rt.desktop()
        .set_working_directory(&path)
        .map_err(|why| reason(&why))?;
    Ok(Value::Text(path))
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
///
/// The dialect gives every built-in variable the prefix, and a name
/// without it is not one of them.
pub fn is_built_in(name: &str) -> bool {
    let Some(rest) = spelled(name) else { return false };
    kind_of(&rest).is_some() || THE_RUNTIME_OWNS.contains(&rest.as_str())
}

/// The reason the tree does not answer a built-in variable, when it does
/// not.
///
/// `None` means the tree answers the name: the value may be the value of
/// the machine, and the reason of a failure belongs to the question and
/// not to the name.
pub fn refusal_of(name: &str) -> Option<&'static str> {
    let name = in_the_table(name);
    match kind_of(&name)? {
        // The hotkeys answer with a value of their own, and the date
        // answers when the clock of the host knows it.
        Kind::Known | Kind::Date | Kind::Hotkey | Kind::Screen | Kind::Clipboard => None,
        // The settings are kept by the module of the settings, and that
        // module says which of them it cannot answer.
        Kind::State => match crate::settings::value_of(&name) {
            Some(Ok(_)) => None,
            Some(Err(why)) => Some(why),
            None => Some(crate::settings::NO_STATE),
        },
        // The names that the desktop answers have no reason of their own:
        // the desktop says what the machine cannot answer.
        Kind::Machine => machine_refusal(&name),
    }
}

/// Whether the tree carries the built-in variable of this name.
///
/// The name is carried when this module answers it, or when the runtime
/// writes it where it runs.
pub fn carries(name: &str) -> bool {
    let name = in_the_table(name);
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
        "allowmainwindow" | "controldelay" | "coordmodecaret" | "coordmodemenu" | "coordmodemouse"
        | "coordmodepixel" | "coordmodetooltip" | "cursor" | "defaultmousespeed" | "detecthiddentext"
        | "detecthiddenwindows" | "fileencoding" | "keydelay" | "keydelayplay" | "keyduration"
        | "keydurationplay" | "listlines" | "menumaskkey" | "mousedelay" | "mousedelayplay"
        | "regview" | "sendlevel" | "sendmode" | "storecapslockmode" | "titlematchmode"
        | "titlematchmodespeed" | "windelay" => Some(Kind::State),
        // -- the date and the time ------------------------------------------------
        "dd" | "ddd" | "dddd" | "hour" | "mday" | "min" | "mm" | "mmm" | "mmmm" | "mon" | "msec"
        | "now" | "nowutc" | "sec" | "wday" | "yday" | "year" | "yweek" | "yyyy" => Some(Kind::Date),
        // -- the machine and the session -------------------------------------------
        "ahkpath" | "appdata" | "appdatacommon" | "computername" | "comspec" | "desktop"
        | "desktopcommon" | "eventinfo" | "iconfile" | "iconhidden" | "iconnumber" | "icontip"
        | "initialworkingdir" | "isadmin" | "language" | "lasterror" | "mydocuments" | "osversion"
        | "programfiles" | "programs" | "programscommon" | "screendpi" | "scriptdir"
        | "scriptfullpath" | "scripthwnd" | "scriptname" | "startmenu" | "startmenucommon"
        | "startup" | "startupcommon" | "temp" | "timeidle" | "timeidlekeyboard" | "timeidlemouse"
        | "timeidlephysical" | "traymenu" | "username" | "windir" | "workingdir" => Some(Kind::Machine),
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

/// The value of a built-in variable of the machine.
///
/// The desktop is asked: the user, the computer, the shell, the folders,
/// the version of the operating system, the language, the elevation and
/// the idle times are the ones of the machine that runs the script, and
/// the reason of a question the desktop cannot answer is the one of the
/// desktop. The names that the tree refuses are refused with the reason
/// of the name, and that reason is written once.
fn machine_value(name: &str, rt: &mut Runtime) -> Result<Value, RuntimeError> {
    if let Some(why) = machine_refusal(name) {
        return Err(reason(why));
    }
    match name {
        "username" => text(rt.desktop().user_name()),
        "computername" => text(rt.desktop().computer_name()),
        "comspec" => text(rt.desktop().shell()),
        "osversion" => text(rt.desktop().os_version()),
        "language" => text(rt.desktop().language()),
        "isadmin" => rt.desktop().is_admin().map(|yes| Value::Int(yes as i64)).map_err(|why| reason(&why)),
        // The directory where the run started is the one of the run, and
        // the script does not change it.
        "initialworkingdir" | "scriptdir" => match rt.initial_working_dir() {
            Some(directory) => Ok(Value::Text(directory.to_string())),
            None => Err(reason(NO_INITIAL_WORKING_DIR)),
        },
        // The script of this runtime comes on the standard input, and v2
        // says what the names of a script that comes from there are: the
        // directory of the script is the directory where the run started,
        // and the name and the path are an asterisk.
        "scriptname" | "scriptfullpath" | "icontip" => Ok(Value::Text("*".to_string())),
        "appdata" | "appdatacommon" | "desktop" | "desktopcommon" | "startmenu" | "startmenucommon"
        | "programs" | "programscommon" | "startup" | "startupcommon" | "mydocuments" | "temp" => {
            folder(rt, name)
        }
        "timeidle" | "timeidlephysical" | "timeidlekeyboard" | "timeidlemouse" => idle(rt, name),
        // The working directory is the one of the files, and the tree uses
        // it everywhere else.
        "workingdir" => text(rt.desktop().working_directory()),
        // The runtime has no events that carry information, so the
        // information of an event is zero.
        "eventinfo" => Ok(Value::Int(0)),
        // The tree does not call the operating system, so the last error
        // of it is zero.
        "lasterror" => Ok(Value::Int(0)),
        // There is no tray icon, so the icon of the script is not hidden.
        "iconhidden" => Ok(Value::Int(1)),
        _ => Err(reason(NO_SUCH_MACHINE_NAME)),
    }
}

/// The reason a name of the machine refuses with.
///
/// The names that the desktop answers have no reason here: the desktop
/// says what the machine cannot answer, and that reason is not the one of
/// the name.
fn machine_refusal(name: &str) -> Option<&'static str> {
    Some(match name {
        "windir" | "programfiles" => NOT_WINDOWS,
        "ahkpath" => NO_AHK_PATH,
        "scripthwnd" => NO_SCRIPT_HWND,
        "screendpi" => NO_SCREEN_DPI,
        "iconfile" | "iconnumber" | "traymenu" => NO_TRAY_ICON,
        _ => return None,
    })
}

/// The text of the answer of the desktop, or the reason it cannot answer.
fn text(answer: Result<String, String>) -> Result<Value, RuntimeError> {
    answer.map(Value::Text).map_err(|why| reason(&why))
}

/// The folder of the machine that a name asks for.
///
/// A folder that the machine does not have is a reason, and the reason is
/// the one of the name: the desktop of everybody and the programs are the
/// two folders that the machine answers in their own way.
fn folder(rt: &mut Runtime, name: &str) -> Result<Value, RuntimeError> {
    let folders = rt.desktop().folders().map_err(|why| reason(&why))?;
    let path = match name {
        "appdata" => folders.app_data.clone(),
        "appdatacommon" => folders.app_data_common.clone(),
        "desktop" => folders.desktop.clone(),
        "desktopcommon" => folders.desktop_common.clone(),
        "startmenu" => folders.start_menu.clone(),
        "startmenucommon" => folders.start_menu_common.clone(),
        "programs" => folders.programs.clone(),
        "programscommon" => folders.programs_common.clone(),
        "startup" => folders.startup.clone(),
        "startupcommon" => folders.startup_common.clone(),
        "mydocuments" => folders.my_documents.clone(),
        "temp" => folders.temp.clone(),
        _ => None,
    };
    match path {
        Some(path) => Ok(Value::Text(path)),
        None => Err(reason(no_folder(name))),
    }
}

/// The reason a folder of the machine is not there.
///
/// The reason is the one of the name: the desktop of everybody and the
/// programs are the two folders that the machine answers in their own way.
fn no_folder(name: &str) -> &'static str {
    match name {
        "desktopcommon" => NO_DESKTOP_COMMON,
        "programs" => NO_PROGRAMS,
        "programscommon" => NO_PROGRAMS_COMMON,
        _ => NO_SUCH_FOLDER,
    }
}

/// The idle time of the session that a name asks for.
///
/// The times are the milliseconds since the last input, and the desktop
/// says when it cannot tell.
fn idle(rt: &mut Runtime, name: &str) -> Result<Value, RuntimeError> {
    let idle = rt.desktop().idle_times().map_err(|why| reason(&why))?;
    let millis = match name {
        "timeidle" => idle.any,
        "timeidlephysical" => idle.physical,
        "timeidlekeyboard" => idle.keyboard,
        "timeidlemouse" => idle.mouse,
        _ => 0,
    };
    Ok(Value::Int(millis as i64))
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
        Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer,
        Processes, Screen, Sound, Timers, Windows,
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
    impl Pictures for Fake {}
    impl Network for Fake {}
    impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}

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
    fn a_script_that_asks_for_the_machine_is_told_what_the_desktop_cannot_answer() {
        // The machine is the desktop's, and the desktop of this test
        // answers nothing: the reason is the one of the desktop.
        assert_eq!(reason("a_computername"), "this desktop cannot say the name of the computer");
        assert_eq!(reason("a_username"), "this desktop cannot say the name of the user");
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
        assert_eq!(value("a_maxhotkeysperinterval"), Value::Int(70));
        // Measured on the binary of v2.0.19 on 2026-11-01: the timeout of
        // the change of a hotkey is fifty until a script writes another one.
        assert_eq!(value("a_hotkeymodifiertimeout"), Value::Int(50));
        // Until a hotkey fires, the two names are empty.
        assert_eq!(value("a_thishotkey"), Value::Text(String::new()));
        assert_eq!(value("a_priorkey"), Value::Text(String::new()));
    }

    #[test]
    fn a_script_that_asks_for_a_setting_is_answered_with_the_setting() {
        // The settings of the script are answered: the module of the
        // settings keeps them, and the one that is a question about the
        // machine answers with the reason.
        let _turn = crate::settings::Turn::take();
        assert_eq!(value("a_coordmodemouse"), Value::Text("Client".to_string()));
        assert_eq!(value("a_sendmode"), Value::Text("Input".to_string()));
        assert_eq!(value("a_keydelay"), Value::Int(10));
        // The modes of the caret, of the menu and of the tooltip are kept:
        // the tree has no windows, and the mode is a setting of the script.
        assert_eq!(value("a_coordmodecaret"), Value::Text("Client".to_string()));
        assert_eq!(value("a_controldelay"), Value::Int(20));
        assert_eq!(value("a_regview"), Value::Text("Default".to_string()));
        assert!(reason("a_cursor").contains("shape of the cursor"));
    }

    #[test]
    fn a_name_without_the_prefix_is_not_a_built_in_variable() {
        // The dialect gives every built-in variable the prefix `A_`, and
        // the binary of v2.0.19 answers the prefixed name only: `IsAdmin`
        // is a global that was never assigned, and not the elevation of
        // the script.
        assert!(answer("isadmin").is_none());
        assert!(answer("clipboard").is_none());
        assert!(answer("now").is_none());
        assert!(!is_built_in("isadmin"));
        assert!(!is_built_in("clipboard"));
    }

    #[test]
    fn the_table_carries_the_names_without_the_prefix() {
        // The table of the surface stores the names without the prefix,
        // and the two conventions meet in `carries` and `refusal_of`.
        assert!(carries("clipboard"));
        assert!(carries("Clipboard"));
        assert!(carries("A_Clipboard"));
        assert!(refusal_of("A_ScriptHwnd").is_some());
        assert!(refusal_of("ScriptHwnd").is_some());
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
            // The table stores the name without the prefix, and the
            // script writes it with it.
            .map(|name| format!("a_{name}"))
            .filter(|name| answer(name).is_none())
            .collect();
        assert!(lost.is_empty(), "these variables of v2 are not answered: {lost:?}");
    }
}
