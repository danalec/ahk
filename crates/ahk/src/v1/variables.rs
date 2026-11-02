//! The built-in variables of the v1 that the v2 does not carry.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! value of each one of them, and the fact that a script may write only
//! one of them.
//!
//! The name of a built-in variable of the v1 has the prefix that the
//! dialect gives to every one of them: `A_Clipboard` and not `Clipboard`.
//! The table of the v2 carries the name without the prefix, and the two
//! conventions meet in the dialect, and nowhere else.
//!
//! The `ErrorLevel` is the one built-in variable of the v1 that has no
//! prefix: it is the last error of the run, and the commands that fail
//! write it.

use alloc::format;
use alloc::string::{String, ToString};

use crate::ast::AssignOp;
use crate::runtime::{Runtime, RuntimeError, Value};

/// The value of a built-in variable of the v1.
#[derive(Clone, Copy)]
enum TheValue {
    /// A text that the dialect answers.
    Text(&'static str),
    /// A whole number.
    Number(i64),
    /// The value is the state of the runtime: the `A_AutoTrim` of the v1
    /// says whether the text of a legacy assignment is trimmed.
    TheTrim,
    /// The value is the state of the runtime: the `A_FormatFloat` and the
    /// `A_FormatInteger` of the v1 say the format of the numbers.
    TheFormats,
    /// The value is the state of the runtime: the `A_StringCaseSense` of the
    /// v1 says the sense of the case of the comparisons.
    TheCaseSense,
    /// The value is the state of the runtime: the `A_NumBatchLines` of the v1
    /// says the number of the lines that the script runs at once.
    TheBatchLines,
    /// The value is a flag of the settings: the v1 says the "On" and the
    /// "Off", and the v2 says the number one and zero.
    TheFlag(&'static str),
    /// The value is the encoding of the files: the v1 says the empty text
    /// for the code page of the machine.
    TheEncoding,
    /// The name is carried and the runtime refuses it: the reason is here.
    Refused(&'static str),
}

/// The reason the type of the operating system is not answered.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the value
/// of the name is `WIN32_NT`, and it is a type of Windows.
const NOT_A_WINDOWS: &str = "this name is about Windows, and this machine is not.";

/// The reason the address of the network is not answered.
///
/// The value of the name in the v1 is the address of the first adapter of
/// the machine, and the desktop of the tree does not know it.
const NO_ADDRESS: &str = "this desktop does not know the address of the network of the machine.";

/// The built-in variables of the v1 that the v2 does not carry, with the
/// value of each one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02. Every
/// one of them is read-only, and the only one that a script may write is
/// the `A_Args`: the write of the others is refused with "Not allowed as an
/// output variable.".
const THE_VARIABLES: &[(&str, TheValue)] = &[
    // The arguments of the script: the runner reads the script on its
    // standard input, and a script that writes the name has its own.
    ("A_Args", TheValue::Text("")),
    // Measured: "On". The command `AutoTrim` turns the trim off, and the
    // value of the name follows it.
    ("A_AutoTrim", TheValue::TheTrim),
    // Measured: "10ms".
    ("A_BatchLines", TheValue::Text("10ms")),
    // The caret of the control that has the focus: the tree has none.
    ("A_CaretX", TheValue::Text("")),
    ("A_CaretY", TheValue::Text("")),
    // Measured: "1".
    ("A_DefaultGui", TheValue::Number(1)),
    // The list view and the tree view of the default: the tree has none.
    ("A_DefaultListView", TheValue::Text("")),
    ("A_DefaultTreeView", TheValue::Text("")),
    // Measured: the "A_DetectHiddenText" is the "On" and the
    // "A_DetectHiddenWindows" is the "Off". The v2 answers the number one
    // and zero, and the v1 answers the word.
    ("A_DetectHiddenText", TheValue::TheFlag("detecthiddentext")),
    ("A_DetectHiddenWindows", TheValue::TheFlag("detecthiddenwindows")),
    // The reason of the exit: the script has not exited yet.
    ("A_ExitReason", TheValue::Text("")),
    // Measured: the empty text for the code page of the machine, and the
    // name of the encoding for the rest -- the "UTF-8" after the command
    // that sets it.
    ("A_FileEncoding", TheValue::TheEncoding),
    // Measured: "0.6". The command `SetFormat` turns the format of the
    // floats, and the name of it follows it.
    ("A_FormatFloat", TheValue::TheFormats),
    // Measured: "D". The command `SetFormat` turns the format of the whole
    // numbers, and the name of it follows it.
    ("A_FormatInteger", TheValue::TheFormats),
    // The window and its controls: the tree has no GUI.
    ("A_Gui", TheValue::Text("")),
    ("A_GuiControl", TheValue::Text("")),
    ("A_GuiControlEvent", TheValue::Text("")),
    ("A_GuiEvent", TheValue::Text("")),
    ("A_GuiHeight", TheValue::Text("")),
    ("A_GuiWidth", TheValue::Text("")),
    ("A_GuiX", TheValue::Text("")),
    ("A_GuiY", TheValue::Text("")),
    // The address of the first adapter of the machine: the desktop of the
    // tree does not know it.
    ("A_IPAddress1", TheValue::Refused(NO_ADDRESS)),
    // Measured: "1".
    ("A_IsUnicode", TheValue::Number(1)),
    // Measured: the empty text. The v1 has no key that closes a menu, and
    // the v2 answers the numbers of the key that closes it.
    ("A_MenuMaskKey", TheValue::Text("")),
    // Measured: "10ms".
    ("A_NumBatchLines", TheValue::TheBatchLines),
    // The type of the operating system: it is a type of Windows.
    ("A_OSType", TheValue::Refused(NOT_A_WINDOWS)),
    // Measured: "Off". The command `StringCaseSense` turns the sense of
    // the case on and off, and the name of it follows it.
    ("A_StringCaseSense", TheValue::TheCaseSense),
    // Measured: the "On". The command `SetStoreCapsLockMode` turns it off,
    // and the name of it follows it.
    ("A_StoreCapsLockMode", TheValue::TheFlag("storecapslockmode")),
    // The last error of the run: the name has no prefix, and it is the only
    // one of the v1 that has not. Measured: "0", and a script may write it.
    ("ErrorLevel", TheValue::Number(0)),
    // The label of the thread that runs: the script is at its top.
    ("A_ThisLabel", TheValue::Text("")),
    // The menu of the thread that runs: the tree has none.
    ("A_ThisMenu", TheValue::Text("")),
    ("A_ThisMenuItem", TheValue::Text("")),
    ("A_ThisMenuItemPos", TheValue::Text("")),
];

/// The reason the name is not written: the v1 refuses it at the load.
const NOT_AN_OUTPUT: &str = "Not allowed as an output variable.";

/// The built-in variables that the v1 lets a script write.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// clipboard and the last error of the run are the two built-in variables of
/// the v1 that a script may write, and the arguments of the script are a
/// variable of the script. Every other one of them is refused, and the ones
/// that the v2 lets a script write -- the `A_Index`, the `A_ListLines` and
/// the rest -- are refused as well.
const THE_WRITABLE: &[&str] = &["a_args", "a_clipboard", "clipboard", "errorlevel"];

/// Whether the v1 does not let a script write the built-in variable of this
/// name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the write
/// of every built-in variable of the v1 is refused at the load -- "Not
/// allowed as an output variable.\n     Specifically: A_AhkPath" -- and the
/// only ones that a script may write are the clipboard, the last error of the
/// run and the arguments of the script.
pub fn is_read_only(name: &str) -> bool {
    let wanted = name.to_lowercase();
    if THE_WRITABLE.contains(&wanted.as_str()) {
        return false;
    }
    // The `ComSpec` and the `ProgramFiles` are the built-in variables that the
    // v1 spells without the prefix, and they are read-only.
    if wanted == "comspec" || wanted == "programfiles" {
        return true;
    }
    // Every other built-in variable of the v1 has the prefix.
    let Some(stem) = wanted.strip_prefix("a_") else {
        return false;
    };
    // The names that the v1 does not have as variables are not refused: they
    // are the directives of the v1 and the `A_TrayMenu`.
    if NOT_A_VARIABLE_OF_THE_V1.contains(&stem) {
        return false;
    }
    // The built-in variables that the two dialects share, and the ones that
    // are the v1 alone.
    crate::variables::carries(stem) || carries(&wanted)
}

/// The built-in variables of the v1 that the v1 spells without the prefix.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// `Clipboard` is the variable of the clipboard and the `A_Clipboard` is
/// its alias -- the documentation of the v1 says so -- the `ComSpec` and
/// the `ProgramFiles` are two names of the same thing, and the
/// `ErrorLevel` has no prefix at all.
///
/// Every other built-in variable of the v1 has the prefix, and a name
/// without it is a variable of the script: the v1 reads it through the
/// environment when the environment has it -- the `Temp` of the machine is
/// the environment variable and not a built-in variable of the v1 -- and the
/// environment has no `Cursor` of its own.
const THE_WITHOUT_THE_PREFIX: &[&str] =
    &["clipboard", "comspec", "errorlevel", "programfiles"];

/// The names that the tree carries and the v1 does not have as a variable.
///
/// Five of them are the directives of the v1 -- the `#AllowMainWindow`,
/// the `#HotkeyInterval`, the `#HotkeyModifierTimeout`, the
/// `#MaxHotkeysPerInterval` and the `#MenuMaskKey` -- and the `A_TrayMenu`
/// is a variable of the v2 that the v1 does not carry. Measured on the
/// binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1 accepts the
/// `global` of every one of them.
const NOT_A_VARIABLE_OF_THE_V1: &[&str] = &[
    "allowmainwindow",
    "hotkeyinterval",
    "hotkeymodifiertimeout",
    "maxhotkeysperinterval",
    "menumaskkey",
    "traymenu",
];

/// Whether a script may declare the variable of this name as global.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// refuses the `global` of every built-in variable of the dialect, and the
/// refusal names the variable. The `A_Args` is the one that is not
/// refused: the interpreter creates it at the load, and it is a variable
/// of the script.
pub fn may_not_be_declared(name: &str) -> bool {
    let wanted = name.to_lowercase();
    if THE_WITHOUT_THE_PREFIX.contains(&wanted.as_str()) {
        return true;
    }
    // Every other built-in variable of the v1 has the prefix.
    let Some(stem) = wanted.strip_prefix("a_") else {
        return false;
    };
    // The `A_Args` is a variable of the script, and not a built-in one.
    if stem == "args" {
        return false;
    }
    // The names that the v1 does not have as variables are not refused.
    if NOT_A_VARIABLE_OF_THE_V1.contains(&stem) {
        return false;
    }
    crate::variables::carries(stem) || carries(&wanted)
}

/// Whether the v1 carries the built-in variable of this name.
///
/// The name is the one that the dialect spells, with the prefix that it
/// gives to every built-in variable.
pub fn carries(name: &str) -> bool {
    let wanted = name.to_lowercase();
    THE_VARIABLES.iter().any(|(carried, _)| carried.to_lowercase() == wanted)
}

/// The reason the v1 does not answer the built-in variable of this name.
///
/// `None` means the tree answers the name.
pub fn refusal_of(name: &str) -> Option<&'static str> {
    let wanted = name.to_lowercase();
    for (carried, value) in THE_VARIABLES {
        if carried.to_lowercase() != wanted {
            continue;
        }
        return match value {
            TheValue::Refused(reason) => Some(*reason),
            _ => None,
        };
    }
    None
}

/// The value of the built-in variable of this name.
///
/// `None` means the name is not one of them: the name is a variable of the
/// script.
pub fn lookup(name: &str, rt: &mut Runtime) -> Option<Result<Value, RuntimeError>> {
    let wanted = name.to_lowercase();
    for (carried, value) in THE_VARIABLES {
        if carried.to_lowercase() != wanted {
            continue;
        }
        return Some(match value {
            TheValue::Text(text) => Ok(Value::Text((*text).to_string())),
            TheValue::Number(number) => Ok(Value::Int(*number)),
            // The trim of the legacy assignments is a state of the run, and
            // the name of it says which one is in force.
            TheValue::TheTrim => Ok(Value::Text(
                if rt.auto_trim() { "On" } else { "Off" }.to_string(),
            )),
            // The format of the numbers is a state of the run too.
            TheValue::TheFormats => Ok(Value::Text(
                if wanted.starts_with("a_formatfloat") {
                    rt.float_format().name()
                } else {
                    rt.integer_format().name().to_string()
                },
            )),
            // The sense of the case is a state of the run as well.
            TheValue::TheCaseSense => Ok(Value::Text(rt.case_sense().name().to_string())),
            // The number of the lines that the script runs at once is a state
            // of the run too.
            TheValue::TheBatchLines => Ok(Value::Text(rt.batch_lines().to_string())),
            // The flag of the v1 is the "On" and the "Off", and the v2 says
            // the number one and zero.
            TheValue::TheFlag(name) => {
                let on = match crate::settings::value_of(name) {
                    Some(Ok(Value::Int(number))) => number != 0,
                    _ => false,
                };
                Ok(Value::Text(if on { "On" } else { "Off" }.to_string()))
            }
            // The encoding of the files: the code page of the machine is the
            // empty text for the v1.
            TheValue::TheEncoding => {
                let encoding = crate::settings::file_encoding();
                if encoding == "CP0" {
                    Ok(Value::Text(String::new()))
                } else {
                    Ok(Value::Text(encoding.to_string()))
                }
            }
            TheValue::Refused(reason) => {
                Err(RuntimeError { message: (*reason).to_string(), line: 0 })
            }
        });
    }
    None
}

/// The write of the built-in variable of this name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// arguments of the script and the last error of the run are the variables of
/// the script, and every other built-in variable of the v1 is read-only. The
/// refusal is at the load, and this is the backstop of it: the name that the
/// script builds is not known at the load.
///
/// The concat is the one that the v1 accepts, and the built-in variable keeps
/// the value that it had: the rule of it is in the dialect, and the runtime
/// asks it before it writes anything.
pub fn write(
    name: &str,
    _op: AssignOp,
    _value: &Value,
    line: u32,
    _rt: &mut Runtime,
) -> Option<Result<Value, RuntimeError>> {
    let wanted = name.to_lowercase();
    // The arguments of the script are the ones the script writes, and the
    // runtime keeps them in the table of the script.
    if wanted == "a_args" {
        return None;
    }
    // The last error of the run is the global variable of the script, and the
    // runtime keeps it in the table of the script.
    if wanted == "errorlevel" {
        return None;
    }
    if !is_read_only(&wanted) {
        return None;
    }
    // The message of the v1 names the variable that was written, and it is
    // the same for every write of it.
    let message = format!("{NOT_AN_OUTPUT}\n     Specifically: {name}");
    Some(Err(RuntimeError { message, line }))
}
