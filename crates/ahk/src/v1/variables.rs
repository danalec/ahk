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

use alloc::format;
use alloc::string::ToString;

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
    // The reason of the exit: the script has not exited yet.
    ("A_ExitReason", TheValue::Text("")),
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
    // Measured: "10ms".
    ("A_NumBatchLines", TheValue::Text("10ms")),
    // The type of the operating system: it is a type of Windows.
    ("A_OSType", TheValue::Refused(NOT_A_WINDOWS)),
    // Measured: "Off".
    ("A_StringCaseSense", TheValue::Text("Off")),
    // The label of the thread that runs: the script is at its top.
    ("A_ThisLabel", TheValue::Text("")),
    // The menu of the thread that runs: the tree has none.
    ("A_ThisMenu", TheValue::Text("")),
    ("A_ThisMenuItem", TheValue::Text("")),
    ("A_ThisMenuItemPos", TheValue::Text("")),
];

/// The reason the name is not written: the v1 refuses it at the load.
const NOT_AN_OUTPUT: &str = "Not allowed as an output variable.";

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
            TheValue::Refused(reason) => {
                Err(RuntimeError { message: (*reason).to_string(), line: 0 })
            }
        });
    }
    None
}

/// The write of the built-in variable of this name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: every
/// one of the names is read-only, and the only one that a script may write
/// is the `A_Args`. The refusal is the one of the v1, and it is a load
/// error there.
pub fn write(name: &str, _op: AssignOp, line: u32) -> Option<Result<Value, RuntimeError>> {
    let wanted = name.to_lowercase();
    for (carried, _) in THE_VARIABLES {
        if carried.to_lowercase() != wanted {
            continue;
        }
        // The arguments of the script are the ones the script writes, and
        // the runtime keeps them in the table of the script.
        if wanted == "a_args" {
            return None;
        }
        // The message of the v1 names the variable that was written, and it
        // is the same for every write of it.
        let message = format!("{NOT_AN_OUTPUT}\n     Specifically: {carried}");
        return Some(Err(RuntimeError { message, line }));
    }
    None
}
