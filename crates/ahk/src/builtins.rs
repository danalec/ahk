//! The built-ins the language carries.
//!
//! AutoHotkey v2 keeps its built-ins in a flat table: a name, a handler,
//! an arity, and a byte that picks the flavour of a handler that several
//! names share. This module is that table in the same shape, so that the
//! language can grow to the whole of v2 without the dispatch becoming a
//! match with four hundred arms.
//!
//! The `kind` of a row is the record of the binary: it keeps its names
//! in two tables, and the kind says which one a name came from. It is
//! not a rule of the parser. v2 has no kind of command -- the
//! documentation says that the parentheses of a call may be left out
//! when the answer is not needed and the name is at the start of the
//! line, and that every function may be written that way -- so the
//! parser decides by the syntax alone.
//!
//! The arity is in the row too, and it is the arity of the binary, so
//! the runtime can answer "Too few parameters passed to function."
//! without every handler checking for itself. A handler that cannot
//! honour a parameter answers with the reason.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::dll::{Convention, NativeArg, NativeCall, NativeType, NativeValue};
use crate::runtime::{
    expected_type, unassigned, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, WindowAnswer,
};
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU32, Ordering as AtomicOrder};

/// What kind of built-in a name is.
///
/// The two kinds are the two tables of the binary. The parser does not
/// use them: v2 lets every function be written without parentheses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinKind {
    /// A name of the function table of the binary.
    Function,
    /// A name of the command table of the binary.
    Command,
}

/// What a built-in does with its arguments.
///
/// The variant is the byte that picks the flavour of a handler that
/// several names share. `Ceil` and `Floor` are one handler with two
/// variants, the way they are in the binary.
pub type BuiltinHandler =
    for<'a> fn(&mut Runtime<'a>, &[Value], u32, u8) -> Result<Value, RuntimeError>;

/// One built-in: the name, the arity, and the handler.
pub struct Builtin {
    /// The name, as AutoHotkey v2 spells it.
    pub name: &'static str,
    /// Whether it is a function or a command.
    pub kind: BuiltinKind,
    /// The fewest parameters it takes.
    pub min: u8,
    /// The most it takes. 255 means any number.
    pub max: u8,
    /// The flavour of the handler.
    pub variant: u8,
    /// What it does.
    pub handler: BuiltinHandler,
}

/// The built-ins the language carries.
///
/// The arities are AutoHotkey v2's, read out of the table in the binary:
/// the function entries state them, and the command entries keep theirs
/// in a packing that was not decoded, so the command rows carry the
/// arity the documentation gives.
pub const BUILTINS: &[Builtin] = &[
    // -- the numbers -------------------------------------------------------------
    Builtin { name: "Abs", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: abs },
    Builtin { name: "ACos", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: inverse_trigonometry },
    Builtin { name: "ASin", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: inverse_trigonometry },
    Builtin { name: "ATan", kind: BuiltinKind::Function, min: 1, max: 1, variant: 2, handler: inverse_trigonometry },
    Builtin { name: "Ceil", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: round_to },
    Builtin { name: "Cos", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: trigonometry },
    Builtin { name: "Exp", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: exponential },
    Builtin { name: "Floor", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: round_to },
    Builtin { name: "Ln", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: logarithm },
    Builtin { name: "Log", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: logarithm },
    Builtin { name: "Max", kind: BuiltinKind::Function, min: 1, max: 255, variant: 1, handler: extreme },
    Builtin { name: "Min", kind: BuiltinKind::Function, min: 1, max: 255, variant: 0, handler: extreme },
    Builtin { name: "Mod", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: modulo },
    Builtin { name: "Random", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: random },
    Builtin { name: "Round", kind: BuiltinKind::Function, min: 1, max: 2, variant: 0, handler: round_to_digits },
    Builtin { name: "Sin", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: trigonometry },
    Builtin { name: "Sqrt", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: square_root },
    Builtin { name: "Tan", kind: BuiltinKind::Function, min: 1, max: 1, variant: 2, handler: trigonometry },
    // -- the text ----------------------------------------------------------------
    Builtin { name: "Chr", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: chr },
    Builtin { name: "Format", kind: BuiltinKind::Function, min: 1, max: 255, variant: 0, handler: format },
    Builtin { name: "InStr", kind: BuiltinKind::Function, min: 2, max: 5, variant: 0, handler: in_str },
    Builtin { name: "LTrim", kind: BuiltinKind::Function, min: 1, max: 2, variant: 1, handler: trim },
    Builtin { name: "Ord", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: ord },
    Builtin { name: "RTrim", kind: BuiltinKind::Function, min: 1, max: 2, variant: 2, handler: trim },
    Builtin { name: "Sort", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: sort },
    Builtin { name: "StrCompare", kind: BuiltinKind::Function, min: 2, max: 3, variant: 0, handler: str_compare },
    Builtin { name: "StrLen", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: str_len },
    Builtin { name: "StrLower", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: change_case },
    Builtin { name: "StrReplace", kind: BuiltinKind::Function, min: 2, max: 6, variant: 0, handler: str_replace },
    Builtin { name: "StrUpper", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: change_case },
    Builtin { name: "SubStr", kind: BuiltinKind::Function, min: 2, max: 3, variant: 0, handler: sub_str },
    Builtin { name: "Trim", kind: BuiltinKind::Function, min: 1, max: 2, variant: 0, handler: trim },
// -- the predicates --------------------------------------------------------------
    Builtin { name: "IsAlnum", kind: BuiltinKind::Function, min: 1, max: 2, variant: 7, handler: is_value },
    Builtin { name: "IsAlpha", kind: BuiltinKind::Function, min: 1, max: 2, variant: 8, handler: is_value },
    Builtin { name: "IsDigit", kind: BuiltinKind::Function, min: 1, max: 1, variant: 5, handler: is_value },
    Builtin { name: "IsFloat", kind: BuiltinKind::Function, min: 1, max: 1, variant: 3, handler: is_value },
    Builtin { name: "IsInteger", kind: BuiltinKind::Function, min: 1, max: 1, variant: 2, handler: is_value },
    Builtin { name: "IsLower", kind: BuiltinKind::Function, min: 1, max: 2, variant: 10, handler: is_value },
    Builtin { name: "IsNumber", kind: BuiltinKind::Function, min: 1, max: 1, variant: 1, handler: is_value },
    Builtin { name: "IsObject", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: is_object },
    Builtin { name: "IsSpace", kind: BuiltinKind::Function, min: 1, max: 1, variant: 11, handler: is_value },
    Builtin { name: "IsTime", kind: BuiltinKind::Function, min: 1, max: 1, variant: 4, handler: is_value },
    Builtin { name: "IsUpper", kind: BuiltinKind::Function, min: 1, max: 2, variant: 9, handler: is_value },
    Builtin { name: "IsXDigit", kind: BuiltinKind::Function, min: 1, max: 1, variant: 6, handler: is_value },
    Builtin { name: "Type", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: type_of },
// -- the patterns ----------------------------------------------------------------
    Builtin { name: "RegExMatch", kind: BuiltinKind::Function, min: 2, max: 4, variant: 0, handler: regex_match },
    Builtin { name: "RegExReplace", kind: BuiltinKind::Function, min: 2, max: 6, variant: 0, handler: regex_replace },
    // -- the screen --------------------------------------------------------------
    Builtin { name: "Click", kind: BuiltinKind::Function, min: 0, max: 6, variant: 0, handler: click },
    Builtin { name: "MouseGetPos", kind: BuiltinKind::Command, min: 0, max: 5, variant: 0, handler: mouse_get_pos },
    Builtin { name: "MouseMove", kind: BuiltinKind::Command, min: 2, max: 4, variant: 0, handler: mouse_move },
    Builtin { name: "PixelGetColor", kind: BuiltinKind::Command, min: 0, max: 2, variant: 0, handler: pixel_get_color },
    Builtin { name: "PixelSearch", kind: BuiltinKind::Command, min: 4, max: 6, variant: 0, handler: pixel_search },
    Builtin { name: "Screenshot", kind: BuiltinKind::Command, min: 5, max: 5, variant: 0, handler: screenshot },
    // -- the windows -------------------------------------------------------------
    Builtin { name: "WinActive", kind: BuiltinKind::Function, min: 0, max: 4, variant: 1, handler: window_active },
    Builtin { name: "WinExist", kind: BuiltinKind::Function, min: 0, max: 4, variant: 0, handler: window_exist },
    Builtin { name: "WinGetTitle", kind: BuiltinKind::Command, min: 0, max: 4, variant: 0, handler: win_get_title },
    // -- the rest ----------------------------------------------------------------
    Builtin { name: "MsgBox", kind: BuiltinKind::Command, min: 1, max: 4, variant: 0, handler: msg_box },
    Builtin { name: "Run", kind: BuiltinKind::Command, min: 1, max: 4, variant: 0, handler: run },
    Builtin { name: "Send", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: send },
    Builtin { name: "Sleep", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: sleep },
    // -- the names this runtime answers -------------------------------------------
    Builtin { name: "DirCreate", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: dir_create },
    Builtin { name: "DirDelete", kind: BuiltinKind::Command, min: 1, max: 2, variant: 0, handler: dir_delete },
    Builtin { name: "DirExist", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: dir_exist },
    Builtin { name: "EnvGet", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: env_get },
    Builtin { name: "EnvSet", kind: BuiltinKind::Command, min: 2, max: 2, variant: 0, handler: env_set },
    Builtin { name: "Exit", kind: BuiltinKind::Command, min: 0, max: 1, variant: 0, handler: exit_script },
    Builtin { name: "ExitApp", kind: BuiltinKind::Command, min: 0, max: 1, variant: 0, handler: exit_script },
    Builtin { name: "FileAppend", kind: BuiltinKind::Command, min: 1, max: 3, variant: 0, handler: file_append },
    Builtin { name: "FileDelete", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: file_delete },
    Builtin { name: "FileExist", kind: BuiltinKind::Command, min: 1, max: 1, variant: 0, handler: file_exist },
    Builtin { name: "FileRead", kind: BuiltinKind::Command, min: 1, max: 2, variant: 0, handler: file_read },
    Builtin { name: "GetMethod", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: get_method },
    Builtin { name: "HasBase", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: has_base },
    Builtin { name: "HasMethod", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: has_method },
    Builtin { name: "HasProp", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: has_prop },
    Builtin { name: "IsSetRef", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: is_set_ref },
    Builtin { name: "KeyWait", kind: BuiltinKind::Command, min: 1, max: 2, variant: 0, handler: key_wait },
    Builtin { name: "RunWait", kind: BuiltinKind::Function, min: 1, max: 4, variant: 0, handler: run },
    Builtin { name: "SendMode", kind: BuiltinKind::Command, min: 1, max: 2, variant: 0, handler: send_mode },
    Builtin { name: "SetKeyDelay", kind: BuiltinKind::Command, min: 0, max: 2, variant: 0, handler: set_key_delay },
    Builtin { name: "SetTimer", kind: BuiltinKind::Command, min: 1, max: 3, variant: 0, handler: set_timer },
    Builtin { name: "SplitPath", kind: BuiltinKind::Function, min: 1, max: 6, variant: 0, handler: split_path },
    Builtin { name: "StrSplit", kind: BuiltinKind::Command, min: 1, max: 4, variant: 0, handler: str_split },
    Builtin { name: "VarSetStrCapacity", kind: BuiltinKind::Function, min: 1, max: 2, variant: 0, handler: var_set_str_capacity },
    // -- the names of Windows ------------------------------------------------
    Builtin { name: "BlockInput", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "CallbackCreate", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "CallbackFree", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "CaretGetPos", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "ClipWait", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ComCall", kind: BuiltinKind::Function, min: 2, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ComObjActive", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ComObjConnect", kind: BuiltinKind::Function, min: 1, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "ComObjFlags", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "ComObjFromPtr", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ComObjGet", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ComObjQuery", kind: BuiltinKind::Function, min: 2, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "ComObjType", kind: BuiltinKind::Function, min: 1, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "ComObjValue", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ControlAddItem", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlChooseIndex", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlChooseString", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlClick", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlDeleteItem", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlFindItem", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlFocus", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetChecked", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetChoice", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetClassNN", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetEnabled", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetExStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetFocus", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetHwnd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetIndex", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetItems", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetPos", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlGetVisible", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlHide", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlHideDropDown", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlMove", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSend", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSendText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSetChecked", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSetEnabled", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSetExStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSetStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlSetText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlShow", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ControlShowDropDown", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DetectHiddenText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DetectHiddenWindows", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DllCall", kind: BuiltinKind::Function, min: 1, max: 255, variant: 0, handler: dll_call },
    Builtin { name: "DriveEject", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetCapacity", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetFilesystem", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetLabel", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetList", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetSerial", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetSpaceFree", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetStatus", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetStatusCD", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveGetType", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveLock", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveRetract", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveSetLabel", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "DriveUnlock", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "Edit", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditGetCurrentCol", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditGetCurrentLine", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditGetLine", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditGetLineCount", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditGetSelectedText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "EditPaste", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "FileCreateShortcut", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "FileGetShortcut", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "FileGetVersion", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "FileRecycle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "FileRecycleEmpty", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GroupActivate", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GroupAdd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GroupClose", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GroupDeactivate", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GuiCtrlFromHwnd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "GuiFromHwnd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "IL_Add", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "IL_Create", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "IL_Destroy", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ListViewGetContent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MenuFromHandle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MenuSelect", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MonitorGet", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MonitorGetCount", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MonitorGetName", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MonitorGetPrimary", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "MonitorGetWorkArea", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "NumGet", kind: BuiltinKind::Function, min: 2, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "NumPut", kind: BuiltinKind::Function, min: 3, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ObjAddRef", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: obj_add_ref },
    Builtin { name: "ObjBindMethod", kind: BuiltinKind::Function, min: 1, max: 255, variant: 6, handler: not_carried },
    // -- the object model ---------------------------------------------------------
    Builtin { name: "ObjFromPtr", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ObjFromPtrAddRef", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ObjGetBase", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: obj_get_base },
    Builtin { name: "ObjGetCapacity", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: obj_get_capacity },
    Builtin { name: "ObjHasOwnProp", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: obj_has_own_prop },
    Builtin { name: "ObjOwnPropCount", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: obj_own_prop_count },
    Builtin { name: "ObjOwnProps", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: obj_own_props },
    Builtin { name: "ObjPtr", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ObjPtrAddRef", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ObjRelease", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "ObjSetBase", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: obj_set_base },
    Builtin { name: "ObjSetCapacity", kind: BuiltinKind::Function, min: 2, max: 2, variant: 0, handler: obj_set_capacity },
    Builtin { name: "OnClipboardChange", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "OnMessage", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "OutputDebug", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "PostMessage", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessClose", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessExist", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessGetName", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessGetParent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessGetPath", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessSetPriority", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessWait", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ProcessWaitClose", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "RegCreateKey", kind: BuiltinKind::Function, min: 0, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "RegDelete", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "RegDeleteKey", kind: BuiltinKind::Function, min: 0, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "RegRead", kind: BuiltinKind::Function, min: 0, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "RegWrite", kind: BuiltinKind::Function, min: 0, max: 4, variant: 0, handler: not_carried },
    Builtin { name: "RunAs", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SendMessage", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetCapsLockState", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetControlDelay", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetNumLockState", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetRegView", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetScrollLockState", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetTitleMatchMode", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SetWinDelay", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "Shutdown", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SoundBeep", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SoundGetInterface", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "SoundGetMute", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "SoundGetName", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "SoundGetVolume", kind: BuiltinKind::Function, min: 0, max: 2, variant: 0, handler: not_carried },
    Builtin { name: "SoundPlay", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SoundSetMute", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "SoundSetVolume", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "StatusBarGetText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "StatusBarWait", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "StrGet", kind: BuiltinKind::Function, min: 1, max: 3, variant: 0, handler: not_carried },
    Builtin { name: "StrPtr", kind: BuiltinKind::Function, min: 1, max: 1, variant: 0, handler: not_carried },
    Builtin { name: "StrPut", kind: BuiltinKind::Function, min: 1, max: 4, variant: 0, handler: not_carried },
    Builtin { name: "SysGet", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "SysGetIPAddresses", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "ToolTip", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "TraySetIcon", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "TrayTip", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinActivate", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinActivateBottom", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinClose", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetClass", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetClientPos", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetControls", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetControlsHwnd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetCount", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetExStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetID", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetIDLast", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetList", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetMinMax", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetPID", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetPos", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetProcessName", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetProcessPath", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetTransColor", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinGetTransparent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinHide", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinKill", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMaximize", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMinimize", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMinimizeAll", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMinimizeAllUndo", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMove", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMoveBottom", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinMoveTop", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinRedraw", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinRestore", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetAlwaysOnTop", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetEnabled", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetExStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetRegion", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetStyle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetTitle", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetTransColor", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinSetTransparent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinShow", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinWait", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinWaitActive", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinWaitClose", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    Builtin { name: "WinWaitNotActive", kind: BuiltinKind::Command, min: 0, max: 255, variant: 0, handler: not_carried },
    // -- the hotkeys --------------------------------------------------------
    Builtin { name: "GetKeyState", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "HotIf", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "HotIfWinActive", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "HotIfWinExist", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "HotIfWinNotActive", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "HotIfWinNotExist", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "Hotkey", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "Hotstring", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "InstallKeybdHook", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "InstallMouseHook", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "KeyHistory", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    Builtin { name: "ListHotkeys", kind: BuiltinKind::Command, min: 0, max: 255, variant: 1, handler: not_carried },
    // -- the names of the script that this runtime cannot answer -----------------
    Builtin { name: "CoordMode", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "Critical", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "DateAdd", kind: BuiltinKind::Command, min: 0, max: 255, variant: 2, handler: not_carried },
    Builtin { name: "DateDiff", kind: BuiltinKind::Command, min: 0, max: 255, variant: 2, handler: not_carried },
    Builtin { name: "DirCopy", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "DirMove", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "DirSelect", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "Download", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "FileCopy", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "FileEncoding", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileGetAttrib", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileGetSize", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileGetTime", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileInstall", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileMove", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "FileOpen", kind: BuiltinKind::Function, min: 2, max: 3, variant: 4, handler: not_carried },
    Builtin { name: "FileSelect", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileSetAttrib", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FileSetTime", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "FormatTime", kind: BuiltinKind::Function, min: 0, max: 2, variant: 2, handler: not_carried },
    Builtin { name: "GetKeyName", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "GetKeySC", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "GetKeyVK", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "ImageSearch", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "IniDelete", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "IniRead", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "IniWrite", kind: BuiltinKind::Command, min: 0, max: 255, variant: 4, handler: not_carried },
    Builtin { name: "InputBox", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "IsLabel", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "ListLines", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "ListVars", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "LoadPicture", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "MouseClick", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "MouseClickDrag", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "OnError", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "OnExit", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "Pause", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "Persistent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "Reload", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "SendEvent", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SendInput", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SendLevel", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SendPlay", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SendText", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SetDefaultMouseSpeed", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SetMouseDelay", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SetStoreCapsLockMode", kind: BuiltinKind::Command, min: 0, max: 255, variant: 5, handler: not_carried },
    Builtin { name: "SetWorkingDir", kind: BuiltinKind::Command, min: 0, max: 255, variant: 6, handler: not_carried },
    Builtin { name: "StrTitle", kind: BuiltinKind::Function, min: 1, max: 1, variant: 6, handler: not_carried },
    Builtin { name: "Suspend", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "Thread", kind: BuiltinKind::Command, min: 0, max: 255, variant: 3, handler: not_carried },
    Builtin { name: "VerCompare", kind: BuiltinKind::Function, min: 2, max: 2, variant: 6, handler: not_carried },
];

/// The parameters of a built-in that take a reference.
///
/// The flag of the table of v2 is the place of the first parameter that
/// is written back, and the parameters after it are the ones that follow
/// it. A reference in any other place is a value where a variable is
/// wanted, and the built-in says so.
pub fn reference_parameters(name: &str) -> &'static [usize] {
    match name {
        "CaretGetPos" => &[0],
        "IsSetRef" => &[0],
        "MouseGetPos" => &[0, 1, 2, 3],
        "RegExMatch" => &[2],
        "RegExReplace" => &[3],
        "RunWait" => &[3],
        "SplitPath" => &[1, 2, 3, 4, 5],
        "StrReplace" => &[4],
        "VarSetStrCapacity" => &[0],
        _ => &[],
    }
}

/// Whether the built-in says itself which parameters take a reference.
///
/// The `DllCall` is the one that does: the place of the reference is
/// said by the type of the argument, and the type is a parameter of the
/// call. The table of v2 cannot hold that, because the table is fixed
/// and the call is not.
pub fn decides_references(name: &str) -> bool {
    matches!(name, "DllCall")
}

/// The built-in with this name, if the language has one.
///
/// The name is matched the way AutoHotkey matches it: without regard to
/// the case.
pub fn find(name: &str) -> Option<&'static Builtin> {
    let wanted = name.to_lowercase();
    BUILTINS.iter().find(|builtin| builtin.name.to_lowercase() == wanted)
}

/// The built-in with this name, if the language has one, as a command.
///
/// A command may be written as a statement; a function may not.
pub fn find_command(name: &str) -> Option<&'static Builtin> {
    find(name).filter(|builtin| builtin.kind == BuiltinKind::Command)
}

/// The names of the built-ins, as the language spells them.
pub fn names() -> impl Iterator<Item = &'static str> {
    BUILTINS.iter().map(|builtin| builtin.name)
}

// -- the names that are carried, and that this runtime cannot answer ------------


/// `DllCall`: a call into a library of the machine.
///
/// v2 writes the library and the function in one string, the type of
/// every argument after it, and the type of the answer at the end. The
/// types and the values alternate, and the last parameter may be the
/// type of the answer alone: the count of the parameters tells the two
/// apart.
fn dll_call(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let Some(first) = values.first() else {
        return Err(RuntimeError { message: "DllCall wants the library and the function.".into(), line });
    };
    let Value::Text(target) = first else {
        return Err(RuntimeError { message: expected_type(TypeName::String, first.type_name()), line });
    };
    let (library, function) = crate::dll::split_target(target);
    let rest = &values[1..];
    let (returns, convention, rest) = if rest.len() % 2 == 1 {
        let last = rest.last().expect("there is one");
        let Value::Text(text) = last else {
            return Err(RuntimeError { message: expected_type(TypeName::String, last.type_name()), line });
        };
        let (convention, name) = split_convention(text);
        let Some((kind, _)) = crate::dll::parse_type(name) else {
            return Err(RuntimeError { message: "Invalid return type.".into(), line });
        };
        (kind, convention, &rest[..rest.len() - 1])
    } else {
        (NativeType::Int, Convention::Standard, rest)
    };
    let mut arguments = Vec::new();
    let mut references = Vec::new();
    let mut index = 0;
    while index < rest.len() {
        let Value::Text(text) = &rest[index] else {
            return Err(RuntimeError { message: expected_type(TypeName::String, rest[index].type_name()), line });
        };
        let Some((kind, address)) = crate::dll::parse_type(text) else {
            return Err(RuntimeError { message: "Invalid arg type.".into(), line });
        };
        let (argument, reference) = marshal(rt, kind, address, &rest[index + 1], line)?;
        arguments.push(argument);
        references.push(reference);
        index += 2;
    }
    let mut call = NativeCall { library, function, convention, returns, arguments };
    let answer = rt
        .desktop()
        .call_native(&mut call)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    // The arguments that are the address of a variable come back into
    // the variable.
    for (argument, reference) in call.arguments.iter().zip(references) {
        // Only the argument that is the address of a variable comes back.
        if !argument.address {
            continue;
        }
        let Some(reference) = reference else { continue };
        let NativeValue::Bytes(bytes) = &argument.value else { continue };
        let written = if argument.kind.is_text() {
            Value::Text(crate::dll::decode(bytes, argument.kind))
        } else if argument.kind.is_float() {
            Value::Float(crate::dll::decode_number(bytes, argument.kind))
        } else {
            Value::Int(crate::dll::decode_number(bytes, argument.kind) as i64)
        };
        rt.write_reference(&reference, written);
    }
    Ok(match answer.value {
        NativeValue::Integer(number) => Value::Int(number),
        NativeValue::Float(number) => Value::Float(number),
        NativeValue::Bytes(bytes) => Value::Text(crate::dll::decode(&bytes, answer.kind)),
    })
}

/// The argument of a native call, out of the value the script wrote.
///
/// A value behind a reference is the value of the variable, and the
/// call writes into the variable: the reference comes back with the
/// argument.
fn marshal(
    rt: &mut Runtime<'_>,
    kind: NativeType,
    address: bool,
    value: &Value,
    line: u32,
) -> Result<(NativeArg, Option<Value>), RuntimeError> {
    let (value, reference) = match value {
        Value::Reference(reference) => {
            let Some(current) = rt.read_reference(reference) else {
                return Err(RuntimeError { message: unassigned(&reference.name), line });
            };
            (current, Some(value.clone()))
        }
        other => (other.clone(), None),
    };
    // A text goes out as its bytes.
    if kind.is_text() {
        let Value::Text(text) = &value else {
            return Err(RuntimeError { message: expected_type(TypeName::String, value.type_name()), line });
        };
        let bytes = crate::dll::encode(text, kind);
        return Ok((NativeArg { kind, address, value: NativeValue::Bytes(bytes) }, reference));
    }
    // A pointer may be an object, and then it is the address of the
    // object that goes.
    if matches!(kind, NativeType::Ptr | NativeType::UPtr) {
        if let Value::Object(_) = &value {
            if let Some(place) = rt.property_of(&value, "ptr", line) {
                let place = place
                    .to_number_strict()
                    .map_err(|got| RuntimeError { message: expected_type(TypeName::Integer, got), line })?;
                return Ok((NativeArg { kind, address, value: NativeValue::Integer(place as i64) }, reference));
            }
        }
    }
    let number = native_number(rt, &value, line)?;
    let value = if kind.is_float() {
        NativeValue::Float(number)
    } else {
        NativeValue::Integer(number as i64)
    };
    Ok((NativeArg { kind, address, value }, reference))
}

/// The value as a number, the way a native call wants it.
fn native_number(rt: &Runtime<'_>, value: &Value, line: u32) -> Result<f64, RuntimeError> {
    match rt.number(value, line) {
        Ok(number) => Ok(number),
        Err(RuntimeFailure::Error(error)) => Err(error),
        Err(RuntimeFailure::Timeout(_)) => {
            Err(RuntimeError { message: "the script ran out of time.".into(), line })
        }
        Err(RuntimeFailure::Flow(_)) => {
            Err(RuntimeError { message: "a number was expected.".into(), line })
        }
    }
}

/// The convention and the type of the answer, read out of the last
/// parameter.
///
/// v2 writes the convention and the type of the answer in one string:
/// "CDecl Int", or "CDecl" alone, or the type alone.
fn split_convention(text: &str) -> (Convention, &str) {
    let trimmed = text.trim();
    let (head, tail) = match trimmed.split_once(char::is_whitespace) {
        Some((head, tail)) => (head, tail.trim()),
        None => (trimmed, ""),
    };
    if head.eq_ignore_ascii_case("cdecl") {
        return (Convention::Cdecl, tail);
    }
    if head.eq_ignore_ascii_case("stdcall") {
        return (Convention::Standard, tail);
    }
    (Convention::Standard, trimmed)
}

/// A name that this runtime carries, and cannot answer.
///
/// The variant says why: the name is shaped for Windows, it is a
/// hotkey, or it is a name of the script that this runtime does not
/// have. The script knows which name it called, so the reason names the
/// group and not the name.
fn not_carried(_rt: &mut Runtime<'_>, _values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let message = match variant {
        0 => "this is a name of Windows, and this desktop is not Windows.",
        1 => "this runtime has no hotkeys yet.",
        2 => "the clock of this runtime counts from an arbitrary origin, and it does not know the date.",
        3 => "this runtime is one script that ends when it ends, and it cannot do that.",
        4 => "this desktop cannot serve that with the files it has.",
        5 => "the pointer and the keyboard of this runtime are the ones of the box, and they have no setting of that kind.",
        6 => "a method of this runtime is not bound to an object.",
        _ => "this runtime does not carry that name yet.",
    };
    Err(RuntimeError { message: message.to_string(), line })
}

// -- the environment ---------------------------------------------------------------

/// `EnvGet`: the value of an environment variable.
fn env_get(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    let value = rt.desktop().env_get(&name).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(value))
}

/// `EnvSet`: set an environment variable, and answer its value.
fn env_set(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    let value = text_of(values, 1);
    rt.desktop().env_set(&name, &value).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(value))
}

// -- the files of the disk ----------------------------------------------------------

/// `FileExist`: whether a file is there.
fn file_exist(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let there = rt.desktop().file_exists(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(there as i64))
}

/// `DirExist`: whether a directory is there.
fn dir_exist(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let there = rt.desktop().dir_exists(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(there as i64))
}

/// `FileRead`: the lines of a file.
fn file_read(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The options of v2 are the encoding, the raw reading and the size
    // of the part to read, and the desktop of the box reads the whole
    // file as text.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the options of the read are not carried yet.".into(),
            line,
        });
    }
    let path = text_of(values, 0);
    let lines = rt.desktop().read_file(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::List(lines.into_iter().map(Value::Text).collect()))
}

/// `FileAppend`: add to the end of a file.
fn file_append(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The options of v2 are the encoding and the way the file is
    // opened, and the desktop of the box appends text.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the options of the write are not carried yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let path = text_of(values, 1);
    rt.desktop().append_file(&path, &text).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(text))
}

/// `FileDelete`: take a file away.
fn file_delete(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    rt.desktop().delete_file(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(path))
}

/// `DirCreate`: make a directory.
fn dir_create(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    rt.desktop().create_dir(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(path))
}

/// `DirDelete`: take a directory away.
fn dir_delete(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The recursive delete of v2 is not carried: the desktop of the box
    // takes a folder away and says whether it could.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the recursive delete is not carried yet.".into(),
            line,
        });
    }
    let path = text_of(values, 0);
    rt.desktop().dir_delete(&path).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(path))
}

// -- the pieces of a text ----------------------------------------------------------

/// `StrSplit`: the pieces of a text, cut at the delimiters.
fn str_split(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The characters to omit and the number of pieces are not carried.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the characters to omit and the number of pieces are not carried yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let delimiters = text_of(values, 1);
    // v2: when the delimiters are blank, every character of the text is
    // a piece of its own.
    let pieces = if delimiters.is_empty() {
        text.chars().map(|character| character.to_string()).collect()
    } else {
        crate::runtime::split_on_any(&text, &delimiters)
    };
    Ok(Value::List(pieces.into_iter().map(Value::Text).collect()))
}

/// `SplitPath`: the pieces of a path.
///
/// v2 writes the pieces into the variables of the caller, and it
/// answers with nothing. The pieces are the name of the file, the
/// directory, the extension, the name without the extension and the
/// drive.
fn split_path(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_argument("SplitPath", values, 0, line)?;
    let pieces = split_the_path(&path);
    for (index, piece) in pieces.into_iter().enumerate() {
        let Some(value) = values.get(index + 1) else { continue };
        // The pieces are written into the variables of the caller, and
        // those are given by reference.
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 2,
                    "SplitPath",
                    "a variable reference",
                    value.type_name(),
                    &value.to_text(),
                    true,
                ),
                line,
            });
        }
        rt.write_reference(value, Value::Text(piece));
    }
    Ok(Value::Text(String::new()))
}

/// The pieces of a path, the way v2 cuts it.
///
/// The name of the file, the directory, the extension, the name
/// without the extension and the drive. A path of a file is cut at the
/// backslash and a URL at the slash, and v2 keeps that difference: the
/// backslash is the separator of a file and the slash is the one of a
/// URL.
fn split_the_path(path: &str) -> [String; 5] {
    // A URL: the drive is the scheme and the host.
    if let Some(at) = path.find("://") {
        let scheme = &path[..at];
        let rest = &path[at + 3..];
        let host_end = rest.find('/').unwrap_or(rest.len());
        let drive = format!("{scheme}://{}", &rest[..host_end]);
        let tail = &rest[host_end..];
        let separator = tail.rfind('/');
        let name = match separator {
            Some(at) => tail[at + 1..].to_string(),
            None => tail.to_string(),
        };
        let directory = match separator {
            Some(at) => format!("{drive}{}", &tail[..at]),
            None => drive.clone(),
        };
        let (stem, extension) = the_extension(&name);
        return [name, directory, extension, stem, drive];
    }
    // A file: the drive is the letters and the colon, or the server of
    // a share. The drive is cut off the path first: the name of v2 does
    // not carry it, and a path with a drive and no backslash keeps the
    // slash in the name.
    let (drive, rest) = if let Some(rest) = path.strip_prefix("\\\\") {
        let end = rest.find('\\').unwrap_or(rest.len());
        (format!("\\\\{}", &rest[..end]), &rest[end..])
    } else if path.len() >= 2 && path.as_bytes()[1] == b':' {
        (path[..2].to_string(), &path[2..])
    } else {
        (String::new(), path)
    };
    let separator = rest.rfind('\\');
    let name = match separator {
        Some(at) => rest[at + 1..].to_string(),
        None => rest.to_string(),
    };
    // Without a backslash the name is the whole path after the drive,
    // and the directory is the drive alone.
    let directory = match separator {
        Some(at) => format!("{drive}{}", &rest[..at]),
        None => drive.clone(),
    };
    let (stem, extension) = the_extension(&name);
    [name, directory, extension, stem, drive]
}

/// The name of a file without its extension, and the extension.
///
/// A dot that opens the name is not one: the name of a file that
/// begins with one has no extension.
fn the_extension(name: &str) -> (String, String) {
    match name.rfind('.') {
        Some(at) if at > 0 => (name[..at].to_string(), name[at + 1..].to_string()),
        _ => (name.to_string(), String::new()),
    }
}

// -- the objects -------------------------------------------------------------------

/// The text of an argument, when it is a text.
///
/// A reference is not a text, and the built-in that wants one says
/// which parameter got it.
fn text_argument(name: &str, values: &[Value], index: usize, line: u32) -> Result<String, RuntimeError> {
    match values.get(index) {
        None => Ok(String::new()),
        Some(Value::Reference(_)) => Err(RuntimeError {
            message: format!(
                "Parameter #{} of {} requires a String, but received a VarRef.",
                index + 1,
                name
            ),
            line,
        }),
        Some(value) => Ok(value.to_text()),
    }
}

/// `HasProp`: whether an object has a property.
fn has_prop(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    rt.base_object(&target, "hasprop", &[Value::Text(name)], line)
}

/// `HasMethod`: whether an object has a method.
fn has_method(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method, and the tree asks the method whether it
    // could take them.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    let mut arguments = vec![Value::Text(name)];
    if let Some(count) = count {
        arguments.push(Value::Int(count as i64));
    }
    rt.base_object(&target, "hasmethod", &arguments, line)
}

/// `GetMethod`: the method of an object, as a value.
fn get_method(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    let mut arguments = vec![Value::Text(name)];
    if let Some(count) = count {
        arguments.push(Value::Int(count as i64));
    }
    rt.base_object(&target, "getmethod", &arguments, line)
}

/// `HasBase`: whether an object is of a class, or extends it.
fn has_base(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    rt.base_object(&target, "hasbase", &[Value::Text(name)], line)
}

/// `IsSetRef`: whether a variable holds a value.
///
/// v2 takes the variable by reference and answers one when the variable
/// is set and zero when it is not. A variable that was never given a
/// value is not set, and the reference to it is still a reference.
fn is_set_ref(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, "IsSetRef"), line });
    };
    let Value::Reference(reference) = value else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(
                1,
                "IsSetRef",
                "a variable reference",
                value.type_name(),
                &value.to_text(),
                true,
            ),
            line,
        });
    };
    Ok(Value::Int(rt.read_reference(reference).is_some() as i64))
}

// -- the keys ----------------------------------------------------------------------

/// `SendMode`: how the keys are sent.
///
/// The keys of this runtime go through the keyboard of the box, one at
/// a time, and that is the mode of the box. The other modes of v2 are
/// the ones of Windows.
fn send_mode(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The method of v2 is the way the keys are sent, and there is one
    // way here.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the method of the sending is not carried yet.".into(),
            line,
        });
    }
    let mode = text_of(values, 0).to_lowercase();
    match mode.as_str() {
        "input" => {
            rt.note("the keys go through the keyboard of the box, one at a time");
            Ok(Value::Int(1))
        }
        _ => Err(RuntimeError {
            message: "the keys of this runtime go through the keyboard of the box, and that is the only mode there is.".into(),
            line,
        }),
    }
}

/// `SetKeyDelay`: the pause between the keys.
///
/// The gap between the keys is the one of the box: it was measured on
/// 2026-10-25, and it is the gap that makes every key of a burst count.
fn set_key_delay(_rt: &mut Runtime<'_>, _values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the gap between the keys is the one of the box; it was measured, and it is the gap that makes every key of a burst count.".into(),
        line,
    })
}

/// `KeyWait`: wait for a key to be up, or to go down.
///
/// The runtime does not keep the keys that are down: the box does, and
/// the runtime does not ask it.
fn key_wait(_rt: &mut Runtime<'_>, _values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "this runtime does not keep the keys that are down, so it cannot wait for one.".into(),
        line,
    })
}

// -- the timers ---------------------------------------------------------------------

/// `SetTimer`: a function of the script that runs between the statements.
///
/// The runtime is one thread, so the timer is not a thread of its own:
/// it runs between the statements of the script, and its budget is the
/// budget of the script.
fn set_timer(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The priority of v2 is the order of the timers, and the runtime
    // has one thread and one order.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the priority of the timer is not carried yet.".into(),
            line,
        });
    }
    let name = text_of(values, 0);
    let period = number_of(values, 1);
    let period = if period > 0.0 { period as u64 } else { 250 };
    rt.add_timer(&name, period);
    Ok(Value::Int(1))
}

// -- the script ---------------------------------------------------------------------

/// `Exit` and `ExitApp`: stop the script.
///
/// The code is kept and the script stops at the end of the statement
/// it was asked in, the way the statement of the same name stops it.
fn exit_script(rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let code = number_of(values, 0) as i32;
    rt.exit_with(code);
    Ok(Value::Int(code as i64))
}

/// What a parameter of a built-in wants.
///
/// v2 declares the type of a parameter when the built-in has one, and
/// the message of the failure names it: "Parameter #1 of Abs requires a
/// Number, but received a String.".
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Wants {
    /// A number: an integer, a float, or a text that holds one.
    Number,
    /// A text.
    String,
    /// An object.
    Object,
    /// A value of any kind.
    Value,
}

/// The type the parameters of a built-in declare.
///
/// The table holds the built-ins whose types were measured on the binary
/// of v2.0.19: the maths, the two texts and the objects. A built-in
/// that is not in it takes a value of any kind, and the guard on a
/// reference says so.
pub fn wants(name: &str, index: usize) -> Wants {
    match (name, index) {
        (
            "Abs" | "ACos" | "ASin" | "ATan" | "Ceil" | "Cos" | "Exp" | "Floor" | "Ln" | "Log"
            | "Sin" | "Sqrt" | "Tan",
            0,
        ) => Wants::Number,
        ("Min" | "Max", _) => Wants::Number,
        ("Mod", 0 | 1) => Wants::Number,
        ("Round", 0 | 1) => Wants::Number,
        ("StrLen" | "SubStr", 0) => Wants::String,
        ("ObjOwnProps" | "ObjOwnPropCount" | "ObjHasOwnProp" | "ObjSetBase", 0) => Wants::Object,
        _ => Wants::Value,
    }
}

/// The handlers of the object model.
///
/// The names of v2 reach the object model from here: the base of an
/// object, the properties it owns, and the room it has for them.
fn obj_get_base(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    rt.base_of(&target).ok_or_else(|| RuntimeError {
        message: crate::runtime::expected_type(TypeName::Object, target.type_name()),
        line,
    })
}

fn obj_set_base(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let base = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    if !matches!(base, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Object, base.type_name()),
            line,
        });
    }
    // The answer is the base the object had, and an object of a script
    // has none until the script gives it one.
    let old = rt.stored_base_of(&target).unwrap_or(Value::Text(String::new()));
    rt.set_base_of(&target, base, line)?;
    Ok(old)
}

fn obj_own_props(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    rt.own_method(&target, "ownprops", &[], line)
}

fn obj_own_prop_count(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let names = rt.own_method(&target, "ownprops", &[], line)?;
    Ok(Value::Int(match names {
        Value::List(items) => items.len() as i64,
        _ => 0,
    }))
}

fn obj_has_own_prop(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let name = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    rt.own_method(&target, "hasownprop", &[name], line)
}

fn obj_get_capacity(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    if !matches!(target, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Object, target.type_name()),
            line,
        });
    }
    // The room an object has is the capacity of its properties, and an
    // object without any has none.
    Ok(rt.property_of(&target, "capacity", line).unwrap_or(Value::Int(0)))
}

fn obj_set_capacity(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let target = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let capacity = values.get(1).cloned().unwrap_or(Value::Int(0));
    if !matches!(target, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Object, target.type_name()),
            line,
        });
    }
    rt.write_property_of(&target, "capacity", capacity, line)?;
    Ok(Value::Int(1))
}

/// `ObjAddRef`: the reference count of an object, one more.
///
/// v2 counts the references of a COM object, and an object of a script
/// has no such count: the built-in says that the parameter is invalid.
fn obj_add_ref(_rt: &mut Runtime<'_>, _values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: crate::runtime::invalid_parameter(1, "ObjAddRef"),
        line,
    })
}

// -- the helpers ----------------------------------------------------------------

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values.get(index).map(|value| value.to_text()).unwrap_or_default()
}

/// The number of the argument at this place, or zero.
fn number_of(values: &[Value], index: usize) -> f64 {
    values.get(index).and_then(|value| value.to_number()).unwrap_or(0.0)
}

/// The number a value holds, and the kind it has.
///
/// A text that holds a number is a number: a whole one is an integer and
/// one with a point is a float, the way v2 reads them.
fn number_value(value: &Value) -> Option<Value> {
    match value {
        Value::Int(_) | Value::Float(_) => Some(value.clone()),
        Value::Text(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return None;
            }
            let number: f64 = trimmed.parse().ok()?;
            if trimmed.contains('.') || trimmed.contains('e') || trimmed.contains('E') {
                Some(Value::Float(number))
            } else {
                Some(Value::Int(number as i64))
            }
        }
        _ => None,
    }
}

/// The number of the argument at this place, as the maths wants it.
///
/// A string that holds a number is a number; a string that holds anything
/// else is not one, and the runtime says so the way v2 says it:
/// "Parameter #1 of Abs requires a Number, but received a String."
fn number_argument(
    values: &[Value],
    index: usize,
    line: u32,
    name: &str,
) -> Result<f64, RuntimeError> {
    let value = values.get(index).cloned().unwrap_or(Value::Int(0));
    value.to_number_strict().map_err(|_| RuntimeError {
        message: crate::runtime::requires(index + 1, name, TypeName::Number, &value),
        line,
    })
}

/// The numbers of the arguments, as the maths wants them.
///
/// Every one of them has to be a number, and the message names the one
/// that was not.
fn numbers_argument(values: &[Value], line: u32, name: &str) -> Result<Vec<f64>, RuntimeError> {
    let mut numbers = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        numbers.push(value.to_number_strict().map_err(|_| RuntimeError {
            message: crate::runtime::requires(index + 1, name, TypeName::Number, value),
            line,
        })?);
    }
    Ok(numbers)
}

/// The whole number of the argument at this place.
fn integer_argument(values: &[Value], index: usize, line: u32) -> Result<i64, RuntimeError> {
    match values.get(index) {
        Some(Value::Int(number)) => Ok(*number),
        Some(other) => Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Integer, other.type_name()),
            line,
        }),
        None => Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Integer, TypeName::String),
            line,
        }),
    }
}

/// Whether the case sense says that the letters are to be taken as they
/// are.
///
/// The sense is on when it says "On", and the number one is the same
/// thing, the way the booleans of AutoHotkey are ones and zeros.
fn case_sense(values: &[Value], index: usize) -> bool {
    match values.get(index) {
        Some(Value::Text(text)) => text.eq_ignore_ascii_case("on"),
        Some(Value::Int(number)) => *number != 0,
        _ => false,
    }
}

/// How two texts stand to each other.
fn compare_texts(left: &str, right: &str, case_sensitive: bool) -> Ordering {
    if case_sensitive {
        left.cmp(right)
    } else {
        left.to_lowercase().cmp(&right.to_lowercase())
    }
}

/// How two values stand to each other when a list is sorted.
///
/// The numbers go by their value and the text by its letters; a number
/// and a text are compared as the text they are.
fn compare_values(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Int(left), Value::Int(right)) => left.cmp(right),
        (Value::Float(left), Value::Float(right)) => left.partial_cmp(right).unwrap_or(Ordering::Equal),
        (Value::Int(left), Value::Float(right)) => {
            (*left as f64).partial_cmp(right).unwrap_or(Ordering::Equal)
        }
        (Value::Float(left), Value::Int(right)) => {
            left.partial_cmp(&(*right as f64)).unwrap_or(Ordering::Equal)
        }
        _ => compare_texts(&left.to_text(), &right.to_text(), false),
    }
}

/// Whether two letters are the same, with or without the case.
fn same_letter(left: char, right: char, case_sensitive: bool) -> bool {
    if case_sensitive {
        left == right
    } else {
        left.eq_ignore_ascii_case(&right)
    }
}

/// Where the needle is in the haystack, without regard to the case.
///
/// The place is the one of a byte, and the text that is not there is
/// answered with nothing.
fn find_ignoring_case(haystack: &str, needle: &str) -> Option<usize> {
    let hay: Vec<char> = haystack.chars().collect();
    let pin: Vec<char> = needle.chars().collect();
    if pin.is_empty() || pin.len() > hay.len() {
        return None;
    }
    for at in 0..=(hay.len() - pin.len()) {
        let same = hay[at..at + pin.len()]
            .iter()
            .zip(pin.iter())
            .all(|(left, right)| same_letter(*left, *right, false));
        if same {
            return haystack.char_indices().nth(at).map(|(byte, _)| byte);
        }
    }
    None
}

/// An error the desktop reported, with the line of the statement.
fn desktop_failure(message: String, line: u32) -> RuntimeError {
    RuntimeError { message, line }
}

// -- the numbers ----------------------------------------------------------------

/// `Abs`: the distance from zero.
///
/// An integer stays an integer, which is what AutoHotkey v2 does: the
/// handler keeps the whole number and only the rest becomes a float.
fn abs(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Int(0));
    // The number is checked first, so that the message of v2 comes
    // before the answer.
    number_argument(values, 0, line, "Abs")?;
    Ok(match number_value(&value) {
        Some(Value::Int(number)) => Value::Int(number.abs()),
        Some(Value::Float(number)) => Value::Float(number.abs()),
        _ => Value::Int(0),
    })
}

/// `Ceil` and `Floor`: the whole number above or below.
///
/// They share one handler, and the variant says which of the two it is.
fn round_to(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let name = if variant == 1 { "Ceil" } else { "Floor" };
    let number = number_argument(values, 0, line, name)?;
    let rounded = if variant == 1 { crate::runtime::ceil(number) } else { crate::runtime::floor(number) };
    Ok(Value::from_number(rounded))
}

/// `Max` and `Min`: the largest and the smallest of the numbers.
///
/// They share one handler, and the variant says which of the two it is.
fn extreme(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let name = if variant == 1 { "Max" } else { "Min" };
    let numbers = numbers_argument(values, line, name)?;
    let mut result = numbers.first().copied().unwrap_or(0.0);
    for number in numbers.iter().skip(1) {
        if (variant == 1 && *number > result) || (variant == 0 && *number < result) {
            result = *number;
        }
    }
    Ok(Value::from_number(result))
}

/// `Mod`: the remainder of a division.
///
/// The sign of the remainder is the sign of the dividend, and the answer
/// is a whole number when both of the arguments are.
fn modulo(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let dividend = values.first().cloned().unwrap_or(Value::Int(0));
    let divisor = values.get(1).cloned().unwrap_or(Value::Int(0));
    // v2 does not name the type that was wrong here: it says that the
    // parameters are invalid.
    let invalid = || RuntimeError { message: "Invalid parameter(s).".into(), line };
    let left = number_value(&dividend).ok_or_else(invalid)?;
    let right = number_value(&divisor).ok_or_else(invalid)?;
    if right.to_number() == Some(0.0) {
        return Err(RuntimeError { message: "Divide by zero.".into(), line });
    }
    Ok(match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left.wrapping_rem(right)),
        (left, right) => {
            let left = left.to_number().unwrap_or(0.0);
            let right = right.to_number().unwrap_or(0.0);
            Value::Float(left % right)
        }
    })
}

/// `Round`: the number, with the digits the caller asks for.
fn round_to_digits(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "Round")?;
    let digits = number_argument(values, 1, line, "Round")? as i32;
    let factor = crate::runtime::powi(10.0, digits);
    Ok(Value::from_number(crate::runtime::round(number * factor) / factor))
}

/// `Sqrt`: the square root.
fn square_root(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "Sqrt")?;
    if number < 0.0 {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, "Sqrt"), line });
    }
    // AutoHotkey v2 always answers a float here, even for a square.
    Ok(Value::Float(crate::runtime::sqrt(number)))
}

/// `Cos`, `Sin` and `Tan`: the trigonometry of a number.
///
/// They share one handler, and the variant says which of the three it is.
/// AutoHotkey v2 answers a float here, whatever went in.
fn trigonometry(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let name = match variant {
        0 => "Sin",
        1 => "Cos",
        _ => "Tan",
    };
    let number = number_argument(values, 0, line, name)?;
    let result = match variant {
        0 => crate::runtime::sin(number),
        1 => crate::runtime::cos(number),
        _ => crate::runtime::tan(number),
    };
    Ok(Value::Float(result))
}

/// `ASin`, `ACos` and `ATan`: the angle whose sine, cosine or tangent is
/// the number.
fn inverse_trigonometry(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let name = match variant {
        0 => "ASin",
        1 => "ACos",
        _ => "ATan",
    };
    let number = number_argument(values, 0, line, name)?;
    if variant != 2 && !(-1.0..=1.0).contains(&number) {
        // The sine and the cosine are not there outside the unit range,
        // and v2 says that the parameter is invalid.
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, name), line });
    }
    let result = match variant {
        0 => crate::runtime::asin(number),
        1 => crate::runtime::acos(number),
        _ => crate::runtime::atan(number),
    };
    Ok(Value::Float(result))
}

/// `Exp`: e raised to the number.
fn exponential(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::exp(number_argument(values, 0, line, "Exp")?)))
}

/// `Ln` and `Log`: the logarithm of the number.
///
/// `Ln` is the natural one and `Log` the one of base ten.
fn logarithm(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let name = if variant == 1 { "Log" } else { "Ln" };
    let number = number_argument(values, 0, line, name)?;
    if number < 0.0 {
        // The logarithm of a negative number is not there, and v2 says
        // that the parameter is invalid.
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, name), line });
    }
    let result = if variant == 0 {
        crate::runtime::ln(number)
    } else {
        crate::runtime::log10(number)
    };
    Ok(Value::Float(result))
}

/// The state of the generator of the random numbers.
///
/// The crate builds for a target that has no `std`, so there is no random
/// crate and no clock of its own: the generator is a small xorshift, and
/// the clock the host gave us is mixed into the seed, so that two runs do
/// not answer the same.
static RANDOM_STATE: AtomicU32 = AtomicU32::new(0);

/// The next number of the generator, as a whole number.
fn next_random(now_ms: u64) -> u32 {
    let mut state = RANDOM_STATE.load(AtomicOrder::Relaxed);
    if state == 0 {
        // The first call seeds the generator.
        state = (now_ms as u32) ^ 0x9E37_79B9 | 1;
    }
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    RANDOM_STATE.store(state, AtomicOrder::Relaxed);
    state
}

/// The next number of the generator, between zero and one.
fn unit_random(now_ms: u64) -> f64 {
    next_random(now_ms) as f64 / (u32::MAX as f64 + 1.0)
}

/// `Random`: a number between the two the caller names.
///
/// Without arguments it is a number between zero and one; with one it is
/// a whole number below it; with two it is a whole number between them,
/// both ends counted.
fn random(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let now = rt.now_ms();
    match values.len() {
        0 => Ok(Value::Float(unit_random(now))),
        1 => {
            let bound = number_argument(values, 0, line, "Random")?;
            if bound <= 0.0 {
                return Ok(Value::Int(0));
            }
            if values[0].type_name() == TypeName::Float {
                return Ok(Value::Float(unit_random(now) * bound));
            }
            Ok(Value::Int(next_random(now) as i64 % bound as i64))
        }
        _ => {
            let mut low = number_argument(values, 0, line, "Random")?;
            let mut high = number_argument(values, 1, line, "Random")?;
            if low > high {
                core::mem::swap(&mut low, &mut high);
            }
            let whole = values[0].type_name() != TypeName::Float && values[1].type_name() != TypeName::Float;
            if whole {
                let span = (high - low) as u64;
                let offset = if span == 0 { 0 } else { next_random(now) as u64 % (span + 1) };
                Ok(Value::Int((low as i64).saturating_add(offset as i64)))
            } else {
                Ok(Value::Float(low + unit_random(now) * (high - low)))
            }
        }
    }
}

// -- the text -------------------------------------------------------------------

/// `Chr`: the character with that code point.
fn chr(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let code = integer_argument(values, 0, line)?;
    let Some(character) = char::from_u32(code as u32) else {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, "Chr"), line });
    };
    Ok(Value::Text(character.to_string()))
}

/// `Format`: a pattern with the values in it.
fn format(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Ok(Value::Text(format_pattern(&text_of(values, 0), &values[1.min(values.len())..])))
}

/// `InStr`: where the needle is in the haystack.
///
/// The position counts from one, and it is zero when the needle is not
/// there at all. The search may be limited to a part of the haystack, and
/// the case sense says whether the letters are taken as they are.
fn in_str(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let haystack: Vec<char> = text_of(values, 0).chars().collect();
    let needle: Vec<char> = text_of(values, 1).chars().collect();
    if needle.is_empty() {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(2, "InStr"), line });
    }
    let start = match values.get(2).and_then(|value| value.to_number()) {
        Some(start) => (start.max(1.0) as usize - 1).min(haystack.len()),
        None => 0,
    };
    let end = match values.get(3).and_then(|value| value.to_number()) {
        Some(end) => (end.max(0.0) as usize).min(haystack.len()),
        None => haystack.len(),
    };
    let case_sensitive = case_sense(values, 4);
    if end < start || end - start < needle.len() {
        return Ok(Value::Int(0));
    }
    for at in start..=(end - needle.len()) {
        let same = haystack[at..at + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(left, right)| same_letter(*left, *right, case_sensitive));
        if same {
            return Ok(Value::Int(at as i64 + 1));
        }
    }
    Ok(Value::Int(0))
}

/// `Ord`: the code point of the first character.
///
/// An empty text has no character, and the code point of it is zero.
fn ord(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    let code = text.chars().next().map(|character| character as i64).unwrap_or(0);
    Ok(Value::Int(code))
}

/// `Sort`: the values in order.
///
/// The list is sorted where it stands. The numbers go by their value and
/// the text by its letters.
fn sort(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        // The options of AutoHotkey are not carried yet.
        return Err(RuntimeError { message: "the options of the sort are not carried yet.".into(), line });
    }
    match values.first() {
        Some(Value::List(items)) => {
            let mut sorted = items.clone();
            sorted.sort_by(compare_values);
            Ok(Value::List(sorted))
        }
        Some(Value::Text(text)) => {
            // A text is a list of lines, and the lines are what is sorted.
            let mut lines: Vec<&str> = text.split('\n').collect();
            lines.sort_by(|left, right| compare_texts(left, right, false));
            Ok(Value::Text(lines.join("\n")))
        }
        _ => Err(RuntimeError { message: crate::runtime::invalid_parameter(1, "Sort"), line }),
    }
}

/// `StrCompare`: how two texts stand to each other.
///
/// The answer is -1, 0 or 1, and the case sense says whether the letters
/// are taken as they are.
fn str_compare(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let left = text_of(values, 0);
    let right = text_of(values, 1);
    let order = compare_texts(&left, &right, case_sense(values, 2));
    Ok(Value::Int(match order {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }))
}

/// `StrLen`: how many characters the text has.
fn str_len(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    Ok(Value::Int(text_of(values, 0).chars().count() as i64))
}

/// `StrLower` and `StrUpper`: the text in the other case.
///
/// They share one handler, and the variant says which of the two it is.
fn change_case(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    Ok(Value::Text(if variant == 1 { text.to_uppercase() } else { text.to_lowercase() }))
}

/// `StrReplace`: the text with one part of it swapped for another.
///
/// v2 writes the number of the swaps into the variable of the caller, and
/// the limit says how many of them are swapped: none of them means all of
/// them. The case sense says whether the letters are taken as they are,
/// and it is off when the caller does not say.
fn str_replace(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let haystack = text_argument("StrReplace", values, 0, line)?;
    let needle = text_argument("StrReplace", values, 1, line)?;
    let replacement = text_argument("StrReplace", values, 2, line)?;
    let case_sensitive = case_sense(values, 3);
    let limit = number_of(values, 5) as i64;
    if needle.is_empty() {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(2, "StrReplace"), line });
    }
    let mut out = String::with_capacity(haystack.len());
    let mut rest = haystack.as_str();
    let mut replaced = 0i64;
    loop {
        let found = if case_sensitive {
            rest.find(needle.as_str())
        } else {
            find_ignoring_case(rest, &needle)
        };
        let Some(at) = found else {
            break;
        };
        out.push_str(&rest[..at]);
        out.push_str(&replacement);
        rest = &rest[at + needle.len()..];
        replaced += 1;
        if limit > 0 && replaced >= limit {
            break;
        }
    }
    out.push_str(rest);
    // The number of the swaps goes into the variable of the caller.
    if let Some(value) = values.get(4) {
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    5,
                    "StrReplace",
                    "a variable reference",
                    value.type_name(),
                    &value.to_text(),
                    true,
                ),
                line,
            });
        }
        rt.write_reference(value, Value::Int(replaced));
    }
    Ok(Value::Text(out))
}

/// `VarSetStrCapacity`: the capacity of a string.
///
/// v2 empties the string when it is given a capacity, and it answers the
/// capacity. Without a capacity it answers the length of the string and
/// leaves the string alone. The tree keeps no capacity of its own: it
/// answers the capacity it is given.
fn var_set_str_capacity(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError { message: crate::runtime::invalid_parameter(1, "VarSetStrCapacity"), line });
    };
    let Value::Reference(reference) = value else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(
                1,
                "VarSetStrCapacity",
                "a variable reference",
                value.type_name(),
                &value.to_text(),
                true,
            ),
            line,
        });
    };
    let Some(current) = rt.read_reference(reference) else {
        return Err(RuntimeError { message: crate::runtime::unassigned(&reference.name), line });
    };
    let Some(capacity) = values.get(1) else {
        return Ok(Value::Int(current.to_text().chars().count() as i64));
    };
    let capacity = match capacity.to_number_strict() {
        Ok(number) => number as i64,
        Err(kind) => {
            return Err(RuntimeError { message: crate::runtime::expected_type(TypeName::Integer, kind), line });
        }
    };
    rt.write_reference(value, Value::Text(String::new()));
    Ok(Value::Int(capacity))
}

/// `SubStr`: the part of the text between two positions.
///
/// AutoHotkey counts positions from one, and a negative position counts
/// from the end: -1 is the last character.
fn sub_str(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let text: Vec<char> = text_of(values, 0).chars().collect();
    let start = number_of(values, 1) as i64;
    let from = start_index(start, text.len()).min(text.len());
    let sliced: String = match values.get(2).and_then(|value| value.to_number()) {
        Some(length) => {
            let length = length as i64;
            if length < 0 {
                // A negative length leaves that many characters out at the
                // end of the text.
                let to = text.len().saturating_sub((-length) as usize).max(from);
                text[from..to].iter().collect()
            } else {
                let to = (from + length as usize).min(text.len());
                text[from..to].iter().collect()
            }
        }
        None => text[from..].iter().collect(),
    };
    Ok(Value::Text(sliced))
}

/// The place a position points at, counting from one.
///
/// A negative position counts from the end, and a position that is out of
/// the text is the end of it.
fn start_index(start: i64, length: usize) -> usize {
    if start < 0 {
        (length as i64 + start).max(0) as usize
    } else if start == 0 {
        length
    } else {
        (start - 1) as usize
    }
}

/// `Trim`, `LTrim` and `RTrim`: the text without the characters at the end.
///
/// They share one handler, and the variant says which end: `Trim` takes
/// both of them, `LTrim` the left one and `RTrim` the right one. The
/// characters are the space and the tab, unless the caller names others.
fn trim(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, variant: u8) -> Result<Value, RuntimeError> {
    let characters: Vec<char> = text_of(values, 0).chars().collect();
    let set: Vec<char> = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_else(|| " \t".to_string())
        .chars()
        .collect();
    let mut start = 0usize;
    let mut end = characters.len();
    if variant != 2 {
        while start < end && set.contains(&characters[start]) {
            start += 1;
        }
    }
    if variant != 1 {
        while end > start && set.contains(&characters[end - 1]) {
            end -= 1;
        }
    }
    Ok(Value::Text(characters[start..end].iter().collect()))
}

// -- the predicates -------------------------------------------------------------

/// The `Is` functions: what the value is.
///
/// They share one handler, and the variant says which question is being
/// asked; the numbers are the ones the binary uses. The second argument
/// of the ones that take one is the mode of AutoHotkey, and the letters
/// of this tree are the Latin ones in either mode.
fn is_value(_rt: &mut Runtime<'_>, values: &[Value], line: u32, variant: u8) -> Result<Value, RuntimeError> {
    // The second parameter of v2 is the code page the text is read
    // with, and the text of this tree is always the text it holds.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the code page of the text is not carried yet.".into(),
            line,
        });
    }
    let value = values.first().cloned().unwrap_or(Value::Text(String::new()));
    let answer = match variant {
        1 => is_number(&value),
        2 => is_integer(&value),
        3 => is_float(&value),
        4 => is_time(&value),
        5 => is_digits(&text_of_value(&value, line)?),
        6 => is_hex_digits(&text_of_value(&value, line)?),
        7 => is_alnum(&text_of_value(&value, line)?),
        8 => is_alpha(&text_of_value(&value, line)?),
        9 => is_upper(&text_of_value(&value, line)?),
        10 => is_lower(&text_of_value(&value, line)?),
        _ => is_space(&text_of_value(&value, line)?),
    };
    Ok(Value::Int(if answer { 1 } else { 0 }))
}

/// `IsObject`: whether the value is an object.
///
/// The objects of this tree are the lists and the class instances, and
/// both of them are objects the way an array is one in AutoHotkey.
fn is_object(_rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Int(0));
    let answer = matches!(value, Value::List(_) | Value::Object(_));
    Ok(Value::Int(if answer { 1 } else { 0 }))
}

/// `Type`: the name of the type of the value.
fn type_of(rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Int(0));
    let name = match value.type_name() {
        TypeName::Integer => "Integer",
        TypeName::Float => "Float",
        TypeName::String => "String",
        TypeName::Object => "Object",
        TypeName::VarRef => "VarRef",
        TypeName::Number => "Number",
        TypeName::Class => "Class",
    };
    // A class is a class: v2 names it so.
    if rt.class_name_of(&value).is_some() {
        return Ok(Value::Text("Class".into()));
    }
    // An object is named after the class it is an instance of.
    if let Value::Object(_) = &value {
        if let Some(class) = rt.class_of(&value) {
            return Ok(Value::Text(class));
        }
    }
    Ok(Value::Text(name.into()))
}

/// The text of the value, when it is text.
///
/// The questions that are about the letters of a value are not about
/// numbers, and AutoHotkey says so when it is given one.
fn text_of_value(value: &Value, line: u32) -> Result<String, RuntimeError> {
    match value {
        Value::Text(text) => Ok(text.clone()),
        other => Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::String, other.type_name()),
            line,
        }),
    }
}

/// Whether the value is a number: an integer, a float, or a text that
/// holds one.
fn is_number(value: &Value) -> bool {
    is_integer(value) || is_float(value)
}

/// Whether the value is a whole number.
fn is_integer(value: &Value) -> bool {
    match value {
        Value::Int(_) => true,
        Value::Float(_) => false,
        Value::Text(text) => is_integer_text(text),
        _ => false,
    }
}

/// Whether the value is a number with a point in it.
fn is_float(value: &Value) -> bool {
    match value {
        Value::Float(_) => true,
        Value::Int(_) => false,
        Value::Text(text) => is_float_text(text),
        _ => false,
    }
}

/// Whether the text is a whole number written out.
///
/// A hexadecimal number is a whole number too, and AutoHotkey counts one
/// as an integer.
fn is_integer_text(text: &str) -> bool {
    let text = text.trim_matches(&[' ', '\t'][..]);
    let rest = text.strip_prefix(&['+', '-'][..]).unwrap_or(text);
    if rest.is_empty() {
        return false;
    }
    if let Some(digits) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        return !digits.is_empty() && digits.chars().all(|character| character.is_ascii_hexdigit());
    }
    rest.chars().all(|character| character.is_ascii_digit())
}

/// Whether the text is a number with a point in it.
fn is_float_text(text: &str) -> bool {
    let text = text.trim_matches(&[' ', '\t'][..]);
    let rest = text.strip_prefix(&['+', '-'][..]).unwrap_or(text);
    let (mantissa, exponent) = match rest.find(['e', 'E']) {
        Some(at) => (&rest[..at], Some(&rest[at + 1..])),
        None => (rest, None),
    };
    if let Some(exponent) = exponent {
        let exponent = exponent.strip_prefix(&['+', '-'][..]).unwrap_or(exponent);
        if exponent.is_empty() || !exponent.chars().all(|character| character.is_ascii_digit()) {
            return false;
        }
    }
    let Some(point) = mantissa.find('.') else {
        return false;
    };
    let (whole, fraction) = mantissa.split_at(point);
    let fraction = &fraction[1..];
    if whole.is_empty() && fraction.is_empty() {
        return false;
    }
    whole.chars().all(|character| character.is_ascii_digit())
        && fraction.chars().all(|character| character.is_ascii_digit())
}

/// Whether the text holds nothing but the digits.
fn is_digits(text: &str) -> bool {
    text.chars().all(|character| character.is_ascii_digit())
}

/// Whether the text holds nothing but the hexadecimal digits.
///
/// A prefix of `0x` is tolerated, the way AutoHotkey tolerates it.
fn is_hex_digits(text: &str) -> bool {
    let rest = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")).unwrap_or(text);
    rest.chars().all(|character| character.is_ascii_hexdigit())
}

/// Whether the text holds nothing but the letters.
fn is_alpha(text: &str) -> bool {
    text.chars().all(|character| character.is_ascii_alphabetic())
}

/// Whether the text holds nothing but the letters and the digits.
fn is_alnum(text: &str) -> bool {
    text.chars().all(|character| character.is_ascii_alphanumeric())
}

/// Whether the text holds nothing but the capital letters.
fn is_upper(text: &str) -> bool {
    text.chars().all(|character| character.is_ascii_uppercase())
}

/// Whether the text holds nothing but the small letters.
fn is_lower(text: &str) -> bool {
    text.chars().all(|character| character.is_ascii_lowercase())
}

/// Whether the text holds nothing but the white space.
///
/// The white space of AutoHotkey is the space, the tab, the linefeed, the
/// return, the vertical tab and the formfeed.
fn is_space(text: &str) -> bool {
    text.chars().all(|character| matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}'))
}

/// Whether the value is a time.
///
/// The rule is the one of AutoHotkey: the value is written as
/// `YYYYMMDDHH24MISS`, so it is a time when it holds an even number of
/// digits, from four to fourteen, and the year is one the system knows.
fn is_time(value: &Value) -> bool {
    let text = value.to_text();
    if !text.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    let digits = text.len();
    if !(4..=14).contains(&digits) || digits % 2 != 0 {
        return false;
    }
    let year: u32 = text[..4].parse().unwrap_or(0);
    (1601..=9999).contains(&year)
}

// -- the patterns ---------------------------------------------------------------

/// `RegExMatch`: where the pattern is in the text.
///
/// The position counts from one, and it is zero when the pattern is not
/// there at all. v2 writes the matches of the pattern into a variable of
/// the caller, and this tree does not carry that yet.
fn regex_match(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the matches of the pattern are not carried into a variable yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let pattern = text_of(values, 1);
    let regex = crate::regex::Regex::new(&pattern).map_err(|reason| RuntimeError { message: reason, line })?;
    let found = regex.find(&text).map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(found.map(|found| found.span().0 as i64 + 1).unwrap_or(0)))
}

/// `RegExReplace`: the text with the matches of the pattern replaced.
///
/// The replacement may name the groups of the pattern with `$1` or `\1`.
/// v2 writes the number of the swaps into the variable of the caller, and
/// the case of the letters is the one of the options of the pattern: the
/// pattern begins with `(?i)` when the case is not to be looked at.
fn regex_replace(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let text = text_argument("RegExReplace", values, 0, line)?;
    let pattern = text_argument("RegExReplace", values, 1, line)?;
    let replacement = text_argument("RegExReplace", values, 2, line)?;
    let limit = number_of(values, 4) as i64;
    // The place where the search starts counts from one, the way
    // AutoHotkey counts the places.
    let starting = (number_of(values, 5) as i64).max(1) as usize - 1;
    let regex = crate::regex::Regex::new(&pattern).map_err(|reason| RuntimeError { message: reason, line })?;
    let characters: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut at = starting.min(characters.len());
    out.extend(characters[..at].iter());
    let mut replaced = 0i64;
    while at <= characters.len() {
        let found = regex.find_at(&text, at).map_err(|reason| RuntimeError { message: reason, line })?;
        let Some(found) = found else {
            break;
        };
        let (start, end) = found.span();
        out.extend(characters[at..start].iter());
        out.push_str(&expand_replacement(&replacement, &found, &characters));
        replaced += 1;
        if end == start {
            // A match of no characters: the search has to move on, or it
            // would never end.
            if end < characters.len() {
                out.push(characters[end]);
            }
            at = end + 1;
        } else {
            at = end;
        }
        if limit > 0 && replaced >= limit {
            break;
        }
    }
    if at <= characters.len() {
        out.extend(characters[at..].iter());
    }
    // The number of the swaps goes into the variable of the caller.
    if let Some(value) = values.get(3) {
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    4,
                    "RegExReplace",
                    "a variable reference",
                    value.type_name(),
                    &value.to_text(),
                    true,
                ),
                line,
            });
        }
        rt.write_reference(value, Value::Int(replaced));
    }
    Ok(Value::Text(out))
}

/// Put the groups of the match into the replacement.
///
/// A group is named with `$1` or `\1`, and `$$` is a dollar of its own.
fn expand_replacement(replacement: &str, found: &crate::regex::Match, characters: &[char]) -> String {
    let mut out = String::new();
    let mut rest = replacement;
    while let Some(at) = rest.find(['$', '\\']) {
        out.push_str(&rest[..at]);
        let marker = rest[at..].chars().next().unwrap_or('$');
        let tail = &rest[at + marker.len_utf8()..];
        if marker == '$' && tail.starts_with('$') {
            out.push('$');
            rest = &tail[1..];
            continue;
        }
        // The group may be named in braces: `${1}`.
        let (name, after) = match tail.strip_prefix('{') {
            Some(braced) => match braced.find('}') {
                Some(close) => (&braced[..close], &braced[close + 1..]),
                None => ("", tail),
            },
            None => {
                let count = tail.chars().take_while(|character| character.is_ascii_digit()).count();
                (&tail[..count], &tail[count..])
            }
        };
        match name.parse::<usize>() {
            Ok(group) => {
                out.push_str(&found.group(group, characters).unwrap_or_default());
                rest = after;
            }
            Err(_) => {
                out.push(marker);
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

// -- the screen -----------------------------------------------------------------

/// `Click`: a click at a place on the screen.
/// `Click`: the components of a click.
///
/// The v2 documentation says that the components may be spread across
/// the parameters, and that they are the place, the button, the number
/// of clicks, the pressing or the releasing of the button, and the way
/// the place is read. The tree carries the first three, and it says so
/// when a call asks for one of the others.
///
/// The place is two numbers, so a number that is not followed by another
/// one is the number of clicks: `Click 2` is a double click where the
/// pointer is, and not a click at (2, 0).
fn click(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The components are the words of the parameters, and a parameter
    // may hold more than one of them.
    let mut words: Vec<String> = Vec::new();
    for value in values {
        for word in value.to_text().split_whitespace() {
            words.push(word.to_string());
        }
    }
    let mut place: Option<(i64, i64)> = None;
    let mut button = "left".to_string();
    let mut count = 1u32;
    let mut at = 0usize;
    while at < words.len() {
        let word = words[at].clone();
        if let Ok(number) = word.parse::<f64>() {
            // The place is two numbers, and it comes before the number
            // of clicks.
            let next_is_a_number =
                words.get(at + 1).map(|next| next.parse::<f64>().is_ok()).unwrap_or(false);
            if place.is_none() && next_is_a_number {
                let y = words[at + 1].parse::<f64>().unwrap_or(0.0);
                place = Some((number as i64, y as i64));
                at += 2;
                continue;
            }
            count = number.max(1.0) as u32;
            at += 1;
            continue;
        }
        match word.to_lowercase().as_str() {
            "left" | "l" => button = "left".to_string(),
            "right" | "r" => button = "right".to_string(),
            "middle" | "m" => button = "middle".to_string(),
            "down" | "d" | "up" | "u" => {
                return Err(RuntimeError {
                    message: "the pressing of the button is not carried yet.".into(),
                    line,
                });
            }
            "rel" | "relative" => {
                return Err(RuntimeError {
                    message: "the place relative to the pointer is not carried yet.".into(),
                    line,
                });
            }
            other if other.starts_with("wheel") || other == "wu" || other == "wd" || other == "wl"
                || other == "wr" =>
            {
                return Err(RuntimeError {
                    message: "the wheel is not carried yet.".into(),
                    line,
                });
            }
            other => {
                return Err(RuntimeError {
                    message: format!("{other} is not a part of a click."),
                    line,
                });
            }
        }
        at += 1;
    }
    // A click with no place is a click where the pointer is.
    let (x, y) = match place {
        Some(place) => place,
        None => {
            let (pointer_x, pointer_y) =
                rt.desktop().mouse_position().map_err(|e| desktop_failure(e, line))?;
            (pointer_x, pointer_y)
        }
    };
    let detail = rt.desktop().click(x, y, &button, count).map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail))
}

/// `MouseGetPos`: where the pointer is.
///
/// v2 writes the place of the pointer into the variables of the caller,
/// and it answers with nothing. The window and the control are empty: the
/// box has no window to ask. The flag of v2 says how the control is to be
/// found, and there is no control to find.
fn mouse_get_pos(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let (x, y) = rt.desktop().mouse_position().map_err(|e| desktop_failure(e, line))?;
    let places = [
        Value::Int(x),
        Value::Int(y),
        Value::Text(String::new()),
        Value::Text(String::new()),
    ];
    for (index, place) in places.into_iter().enumerate() {
        let Some(value) = values.get(index) else { continue };
        // The place is written into the variables of the caller, and
        // those are given by reference.
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 1,
                    "MouseGetPos",
                    "a VarRef",
                    value.type_name(),
                    &value.to_text(),
                    false,
                ),
                line,
            });
        }
        rt.write_reference(value, place);
    }
    Ok(Value::Text(String::new()))
}

/// `MouseMove`: the pointer to a place on the screen.
fn mouse_move(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The speed and the relative place of v2 are not carried: the box
    // moves the pointer at its own speed.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the speed and the relative place are not carried yet.".into(),
            line,
        });
    }
    let x = number_of(values, 0) as i64;
    let y = number_of(values, 1) as i64;
    let detail = rt.desktop().move_pointer(x, y).map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail))
}

/// `PixelGetColor`: the colour of one pixel.
fn pixel_get_color(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let (x, y) = if values.len() >= 2 {
        (number_of(values, 0) as i64, number_of(values, 1) as i64)
    } else {
        let (pointer_x, pointer_y) =
            rt.desktop().mouse_position().map_err(|e| desktop_failure(e, line))?;
        (pointer_x, pointer_y)
    };
    let colour = rt.desktop().pixel(x, y).map_err(|e| desktop_failure(e, line))?;
    rt.set_global("pixel", Value::Text(colour.clone()));
    Ok(Value::Text(colour))
}

/// `PixelSearch`: where a colour is on the screen.
fn pixel_search(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let x1 = number_of(values, 0) as i64;
    let y1 = number_of(values, 1) as i64;
    let x2 = number_of(values, 2) as i64;
    let y2 = number_of(values, 3) as i64;
    let colour = text_of(values, 4);
    let found = rt
        .desktop()
        .pixel_search(x1, y1, x2, y2, &colour)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(match found {
        Some((x, y)) => Value::List(vec![Value::Int(x), Value::Int(y)]),
        None => Value::List(Vec::new()),
    })
}

/// `Screenshot`: a picture of a part of the screen.
///
/// AutoHotkey v2 has no such built-in; this one is ours, and it is the
/// only name in the language that v2 would not have.
fn screenshot(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let x = number_of(values, 0) as i64;
    let y = number_of(values, 1) as i64;
    let width = number_of(values, 2) as i64;
    let height = number_of(values, 3) as i64;
    let name = text_of(values, 4);
    let path = rt
        .desktop()
        .capture(x, y, width, height, &name)
        .map_err(|e| desktop_failure(e, line))?;
    rt.set_global("screenshot", Value::Text(path.clone()));
    Ok(Value::Text(path))
}

// -- the windows ----------------------------------------------------------------

/// `WinActive`: whether the window with this title is the active one.
fn window_active(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and the desktop of the box has none of them.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let title = text_of(values, 0);
    match rt.desktop().window_active(&title).map_err(|e| desktop_failure(e, line))? {
        WindowAnswer::Active => Ok(Value::Int(1)),
        WindowAnswer::NotActive => Ok(Value::Int(0)),
        WindowAnswer::Unknown(reason) => Err(RuntimeError {
            message: format!("WinActive cannot answer: {reason}"),
            line,
        }),
    }
}

/// `WinExist`: whether a window with this title exists.
///
/// It shares its handler with `WinActive`, and the variant says which of
/// the two it is: a window that exists is the answer, whether it is the
/// active one or not.
fn window_exist(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and the desktop of the box has none of them.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let title = text_of(values, 0);
    match rt.desktop().window_exists(&title).map_err(|e| desktop_failure(e, line))? {
        WindowAnswer::Active | WindowAnswer::NotActive => Ok(Value::Int(1)),
        WindowAnswer::Unknown(reason) => Err(RuntimeError {
            message: format!("WinExist cannot answer: {reason}"),
            line,
        }),
    }
}

/// `WinGetTitle`: the title of the active window.
fn win_get_title(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // v2 asks for a window and for the text of it, and the box has only
    // the window that is in front.
    if !values.is_empty() {
        return Err(RuntimeError {
            message: "the window to ask about is not carried yet: only the active one is.".into(),
            line,
        });
    }
    let title = rt.desktop().active_window_title().map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(title.unwrap_or_default()))
}

// -- the rest -------------------------------------------------------------------

/// `MsgBox`: a message for the person watching.
///
/// The box has no window to show it in, so the message goes into the
/// report instead. That is the box's answer, not a quiet `false`.
fn msg_box(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The box has no buttons, no title and no answer to give back.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the options, the title and the variable that takes the answer are not carried yet.".into(),
            line,
        });
    }
    let text = rt.interpolate(&text_of(values, 0), line)?;
    rt.push_output(text.clone());
    Ok(Value::Text(text))
}

/// `Run`: a command, and what it answered.
fn run(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    // The third parameter of v2 is the options of the window, and the
    // fourth is the variable that takes the pid of the process. The
    // desktop of the box has no options, and the reference operator is
    // not carried, so a call that asks for them fails with the reason.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the options of the window and the pid of the process are not carried yet.".into(),
            line,
        });
    }
    let command = text_of(values, 0);
    let working_dir = text_of(values, 1);
    let (code, stdout, stderr) =
        rt.desktop().run(&command, &working_dir).map_err(|e| desktop_failure(e, line))?;
    if !stdout.trim().is_empty() {
        rt.push_output(stdout.trim_end().to_string());
    }
    if !stderr.trim().is_empty() {
        rt.push_output(stderr.trim_end().to_string());
    }
    Ok(Value::Int(code as i64))
}

/// `Send`: text and keys, the way AutoHotkey writes them.
fn send(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let text = rt.interpolate(&text_of(values, 0), line)?;
    let mut parts = Vec::new();
    for part in split_send(&text) {
        let said = match part {
            SendPart::Text(text) => rt.desktop().type_text(&text),
            SendPart::Key { name, modifiers } => rt.desktop().press_key(&name, &modifiers),
            SendPart::Down(name) => match mouse_button(&name) {
                Some(button) => rt.desktop().mouse_down(None, None, button),
                None => rt.desktop().key_down(&name),
            },
            SendPart::Up(name) => match mouse_button(&name) {
                Some(button) => rt.desktop().mouse_up(None, None, button),
                None => rt.desktop().key_up(&name),
            },
        };
        parts.push(said.map_err(|e| desktop_failure(e, line))?);
    }
    Ok(Value::Text(parts.concat()))
}

/// `Sleep`: the pause, in milliseconds.
fn sleep(rt: &mut Runtime<'_>, values: &[Value], _line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let millis = number_of(values, 0).clamp(0.0, 60_000.0) as u64;
    rt.sleep_ms(millis);
    Ok(Value::Int(millis as i64))
}

// -- the pattern of `Format` -----------------------------------------------------

/// Put the arguments into a pattern.
///
/// The pattern is AutoHotkey's: `{1}` is the first argument, `{2}` the
/// second, and `{{` and `}}` are braces of their own. A placeholder that
/// names no argument is left as it stands.
fn format_pattern(pattern: &str, arguments: &[Value]) -> String {
    let mut out = String::with_capacity(pattern.len());
    let chars: Vec<char> = pattern.chars().collect();
    let mut index = 0usize;
    let mut next = 0usize;
    while index < chars.len() {
        match chars[index] {
            '{' if chars.get(index + 1) == Some(&'{') => {
                out.push('{');
                index += 2;
                continue;
            }
            '}' if chars.get(index + 1) == Some(&'}') => {
                out.push('}');
                index += 2;
                continue;
            }
            '{' => {}
            other => {
                out.push(other);
                index += 1;
                continue;
            }
        }
        let Some(offset) = chars[index + 1..].iter().position(|c| *c == '}') else {
            out.push(chars[index]);
            index += 1;
            continue;
        };
        let close = index + 1 + offset;
        let inside: String = chars[index + 1..close].iter().collect();
        let (position, spec) = match inside.split_once(':') {
            Some((position, spec)) => (position.trim().to_string(), spec.trim().to_string()),
            None => (inside.trim().to_string(), String::new()),
        };
        // A placeholder with no number takes the next argument.
        let argument = if position.is_empty() {
            let argument = arguments.get(next);
            next += 1;
            argument
        } else {
            position
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
                .and_then(|number| arguments.get(number))
        };
        match argument {
            Some(value) => out.push_str(&format_value(value, &spec)),
            None => {
                out.push('{');
                out.push_str(&inside);
                out.push('}');
            }
        }
        index = close + 1;
    }
    out
}

/// The value as the pattern asks for it.
///
/// The default is the way AutoHotkey writes a value: an integer as an
/// integer, a float with its point, a text as it stands. A pattern that
/// asks for a number of decimals gets them, and one that asks for an
/// integer gets one.
fn format_value(value: &Value, spec: &str) -> String {
    let mut decimals: Option<usize> = None;
    let mut integer = false;
    for character in spec.chars() {
        match character {
            '.' => decimals = Some(0),
            '0'..='9' => {
                if let Some(count) = decimals {
                    decimals = Some(count * 10 + character.to_digit(10).unwrap_or(0) as usize);
                }
            }
            'd' | 'i' | 'u' => integer = true,
            _ => {}
        }
    }
    if integer {
        let number = crate::runtime::round(value.to_number().unwrap_or(0.0));
        return Value::Int(number as i64).to_text();
    }
    match decimals {
        Some(decimals) => {
            let number = value.to_number().unwrap_or(0.0);
            format!("{number:.decimals$}")
        }
        None => value.to_text(),
    }
}

// -- the parts of a `Send` --------------------------------------------------------

/// One part of what a `Send` types.
enum SendPart {
    /// Text, typed as it stands.
    Text(String),
    /// A key, with the modifiers held while it is pressed.
    Key { name: String, modifiers: Vec<String> },
    /// A key that goes down and stays down.
    Down(String),
    /// A key that comes up.
    Up(String),
}

/// The mouse button a key name means, if it means one.
///
/// AutoHotkey writes a mouse button as a key: `{LButton down}` is the
/// button going down. The box has a pointer of its own, so the button
/// goes there.
fn mouse_button(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "lbutton" | "left" => Some("left"),
        "rbutton" | "right" => Some("right"),
        "mbutton" | "middle" => Some("middle"),
        _ => None,
    }
}

/// Split what a `Send` says into the text and the keys.
///
/// The string of a `Send` carries the AutoHotkey conventions:
///
/// * `{Name}` is a key, and `{Name down}` or `{Name up}` is that key
///   going down or coming up, on its own.
/// * `^`, `+`, `!` and `#` before a key are the modifiers: ctrl, shift,
///   alt and the super key. They apply to the key that comes next, and
///   to nothing else.
/// * A modifier on its own is a literal character, and so is one in
///   braces: `{^}` is a caret.
///
/// The text of a `Send` that is not a key is text, and it is typed as it
/// stands.
fn split_send(text: &str) -> Vec<SendPart> {
    let mut parts = Vec::new();
    let mut buffer = String::new();
    let mut modifiers: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    // The raw mode: everything after it is text, the braces and the
    // modifiers among them.
    let mut raw = false;

    let modifier_name = |character: char| match character {
        '^' => Some("ctrl"),
        '+' => Some("shift"),
        '!' => Some("alt"),
        '#' => Some("super"),
        _ => None,
    };

    while index < chars.len() {
        if raw {
            buffer.push(chars[index]);
            index += 1;
            continue;
        }
        // A modifier before the key it belongs to.
        if let Some(name) = modifier_name(chars[index]) {
            let next = chars.get(index + 1);
            let is_literal = match next {
                None => true,
                Some(next) => next.is_whitespace(),
            };
            if !is_literal {
                modifiers.push(name.to_string());
                index += 1;
                continue;
            }
        }
        // A character with modifiers waiting for it is a key.
        if !modifiers.is_empty() {
            parts.push(SendPart::Key {
                name: chars[index].to_string(),
                modifiers: core::mem::take(&mut modifiers),
            });
            index += 1;
            continue;
        }
        if chars[index] == '{' {
            if let Some(offset) = chars[index + 1..].iter().position(|c| *c == '}') {
                let close = index + 1 + offset;
                let name: String = chars[index + 1..close].iter().collect();
                if !buffer.is_empty() {
                    parts.push(SendPart::Text(core::mem::take(&mut buffer)));
                }
                // `{^}` is a literal character, not a modifier.
                if name.chars().count() == 1 && modifier_name(name.chars().next().unwrap()).is_some() {
                    parts.push(SendPart::Text(name));
                } else if name.eq_ignore_ascii_case("raw") {
                    // The raw mode: the rest of the text is text.
                    raw = true;
                } else {
                    let (key, state, times) = match name.rsplit_once(' ') {
                        Some((key, "down")) => (key.to_string(), Some(true), 1),
                        Some((key, "up")) => (key.to_string(), Some(false), 1),
                        Some((key, times))
                            if !times.is_empty() && times.chars().all(|c| c.is_ascii_digit()) =>
                        {
                            (key.to_string(), None, times.parse::<usize>().unwrap_or(1))
                        }
                        _ => (name.clone(), None, 1),
                    };
                    match state {
                        Some(true) => parts.push(SendPart::Down(key)),
                        Some(false) => parts.push(SendPart::Up(key)),
                        None => {
                            let modifiers = core::mem::take(&mut modifiers);
                            for _ in 0..times {
                                parts.push(SendPart::Key { name: key.clone(), modifiers: modifiers.clone() });
                            }
                        }
                    }
                }
                index = close + 1;
                continue;
            }
        }
        buffer.push(chars[index]);
        index += 1;
    }
    if !buffer.is_empty() {
        parts.push(SendPart::Text(buffer));
    }
    parts
}
