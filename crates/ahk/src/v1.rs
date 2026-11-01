//! The legacy dialect: the names of AutoHotkey v1.1.37.02.
//!
//! The table of the v1 is the table of the v1, measured on the binary of
//! v1.1.37.02 on 2026-10-28 and kept in
//! `crates/ahk/reference/ahk-v1.1.37.02.surface.tsv`. The names are the
//! ones that the v1 spells, and the shape of each parameter is the one
//! that the v1 has: the value of an expression, the name of a variable of
//! the script, or the name of the variable that the command writes into.
//!
//! A name that the v1 has and the v2 does not is here; a name that the two
//! share is the row of the v2, and the table of the v1 points at it.

use crate::builtins::Builtin;

/// The built-ins that the v1 carries.
///
/// The table is the one of the v1, and a name that the v2 has as well is
/// a row of its own: the shape of the parameters of the v1 is not the
/// shape of the v2, and the handler of the v2 takes the value where the
/// v1 takes the name of a variable.
pub const BUILTINS: &[Builtin] = &[];

/// The built-in variables of the v1 that the v2 does not carry.
///
/// The name of a variable of the v1 has the prefix that the dialect gives
/// it: `A_Clipboard` and not `Clipboard`.
pub const VARIABLES: &[&str] = &[
    "A_Args",
    "A_AutoTrim",
    "A_BatchLines",
    "A_CaretX",
    "A_CaretY",
    "A_DefaultGui",
    "A_DefaultListView",
    "A_DefaultTreeView",
    "A_ExitReason",
    "A_FormatFloat",
    "A_FormatInteger",
    "A_Gui",
    "A_GuiControl",
    "A_GuiControlEvent",
    "A_GuiEvent",
    "A_GuiHeight",
    "A_GuiWidth",
    "A_GuiX",
    "A_GuiY",
    "A_IPAddress1",
    "A_IsUnicode",
    "A_NumBatchLines",
    "A_OSType",
    "A_StringCaseSense",
    "A_ThisLabel",
    "A_ThisMenu",
    "A_ThisMenuItem",
    "A_ThisMenuItemPos",
];

/// Whether the v1 carries the built-in variable of this name.
pub fn carries_variable(name: &str) -> bool {
    let wanted = name.to_lowercase();
    VARIABLES.iter().any(|carried| carried.to_lowercase() == wanted)
}

/// The reason the v1 does not answer the built-in variable of this name.
pub fn refusal_of_variable(name: &str) -> Option<&'static str> {
    if carries_variable(name) {
        return Some(crate::dialect::NOT_CARRIED_YET);
    }
    None
}
