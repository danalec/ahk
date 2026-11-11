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

pub(crate) use crate::date::{date_add, date_diff, format_time};
pub(crate) use crate::dialogs::{dir_select, file_select, input_box};
use crate::dll::{Convention, NativeArg, NativeCall, NativeFailureKind, NativeType, NativeValue};
pub(crate) use crate::files::{
    dir_copy, dir_move, download, file_append, file_copy, file_encoding, file_get_attrib,
    file_get_size, file_get_time, file_install, file_move, file_open, file_read, file_set_attrib,
    file_set_time, ini_delete, ini_read, ini_write,
};
pub(crate) use crate::hotkeys::{
    get_key_state, hotif, hotif_window, hotkey, hotstring, install_hook, key_history, list_hotkeys,
};
pub(crate) use crate::pictures::{image_search, load_picture};
use crate::runtime::{
    Runtime, RuntimeError, RuntimeFailure, TypeName, Value, WindowAnswer, expected_type,
};
pub(crate) use crate::settings::{
    coord_mode, get_key_name, get_key_sc, get_key_vk, send_event, send_input, send_level,
    send_play, send_text, set_control_delay, set_default_mouse_speed, set_detect_hidden_text,
    set_detect_hidden_windows, set_key_delay, set_mouse_delay, set_reg_view,
    set_store_caps_lock_mode, set_title_match_mode, set_win_delay,
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

/// The shape of one parameter of a built-in.
///
/// The shape is what the dialect says about the parameter: the value of an
/// expression, the name of a variable of the script, or the name of a
/// variable that the built-in writes into. The v2 writes a reference with
/// the `&`, and the v1 writes the name of a variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parameter {
    /// The parameter is an expression, and the value of it reaches the
    /// handler.
    Value,
    /// The parameter is a reference that the caller passes, written with
    /// the `&` of the v2.
    Reference,
    /// The parameter is a reference the caller passes, and v2 declares it as
    /// a value of the kind `VarRef`: a value of another kind is refused with
    /// "requires a VarRef, but received ...", and the name of the built-in is
    /// in the `What` of the failure.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `MonitorGet(1, 9)` is
    /// "Parameter #2 of MonitorGet requires a VarRef, but received an
    /// Integer." with "MonitorGet" in the `What`, while the output variable of
    /// `RegExMatch` answers "a variable reference" with the empty `What`.
    VarRef,
    /// The parameter is the name of a variable of the script, and the value
    /// of that variable reaches the handler. The commands of the v1 read
    /// their input this way.
    Variable,
    /// The parameter is the name of a variable of the script, and the
    /// handler writes into it. The commands of the v1 write their output
    /// this way.
    Output,
    /// The parameter is the name of a thing of the machine -- a variable of
    /// the environment -- and the text of it reaches the handler. The name
    /// may not be blank, and the commands of the v1 that name a thing of
    /// the machine are checked at the load.
    Name,
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
    /// The shape of each parameter.
    pub parameters: &'static [Parameter],
    /// What it does.
    pub handler: BuiltinHandler,
}

impl Builtin {
    /// Whether the parameter at this place is written back.
    ///
    /// The reference of the v2 is written back by the caller that passes
    /// it, and the output of the v1 is written back by the built-in that
    /// writes into the variable of the script.
    pub fn writes_back(&self, index: usize) -> bool {
        matches!(
            self.parameters.get(index),
            Some(Parameter::Reference) | Some(Parameter::Output)
        )
    }
}

/// The built-ins the language carries.
///
/// The arities are AutoHotkey v2's, read out of the table in the binary:
/// the function entries state them, and the command entries keep theirs
/// in a packing that was not decoded, so the command rows carry the
/// arity the documentation gives.
pub const BUILTINS: &[Builtin] = &[
    // -- the numbers -------------------------------------------------------------
    Builtin {
        name: "Abs",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: abs,
    },
    Builtin {
        name: "ACos",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: inverse_trigonometry,
    },
    Builtin {
        name: "ASin",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: inverse_trigonometry,
    },
    Builtin {
        name: "ATan",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: inverse_trigonometry,
    },
    Builtin {
        name: "Ceil",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: round_to,
    },
    Builtin {
        name: "Cos",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: trigonometry,
    },
    Builtin {
        name: "Exp",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: exponential,
    },
    Builtin {
        name: "Floor",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: round_to,
    },
    Builtin {
        name: "Ln",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: logarithm,
    },
    Builtin {
        name: "Log",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: logarithm,
    },
    Builtin {
        name: "Max",
        kind: BuiltinKind::Function,
        min: 1,
        max: 255,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: extreme,
    },
    Builtin {
        name: "Min",
        kind: BuiltinKind::Function,
        min: 1,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: extreme,
    },
    Builtin {
        name: "Mod",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: modulo,
    },
    Builtin {
        name: "NumGet",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: num_get,
    },
    Builtin {
        name: "NumPut",
        kind: BuiltinKind::Function,
        min: 3,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: num_put,
    },
    Builtin {
        name: "Random",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: random,
    },
    Builtin {
        name: "Round",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: round_to_digits,
    },
    Builtin {
        name: "Sin",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: trigonometry,
    },
    Builtin {
        name: "Sqrt",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: square_root,
    },
    Builtin {
        name: "Tan",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: trigonometry,
    },
    // -- the text ----------------------------------------------------------------
    Builtin {
        name: "Chr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: chr,
    },
    Builtin {
        name: "Format",
        kind: BuiltinKind::Function,
        min: 1,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: format,
    },
    Builtin {
        name: "InStr",
        kind: BuiltinKind::Function,
        min: 2,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: in_str,
    },
    Builtin {
        name: "LTrim",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: trim,
    },
    Builtin {
        name: "Ord",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: ord,
    },
    Builtin {
        name: "RTrim",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: trim,
    },
    Builtin {
        name: "Sort",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sort,
    },
    Builtin {
        name: "StrCompare",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_compare,
    },
    Builtin {
        name: "StrLen",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_len,
    },
    Builtin {
        name: "StrLower",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: change_case,
    },
    Builtin {
        name: "StrReplace",
        kind: BuiltinKind::Function,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Reference,
            Parameter::Value,
        ],
        handler: str_replace,
    },
    Builtin {
        name: "StrUpper",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: change_case,
    },
    Builtin {
        name: "SubStr",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sub_str,
    },
    Builtin {
        name: "Trim",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: trim,
    },
    // -- the predicates --------------------------------------------------------------
    // The one name of the predicates that asks for a variable itself: the
    // argument is the name of a variable of the script, and the answer is
    // whether that variable has a value.
    Builtin {
        name: "IsSet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Reference],
        handler: is_set,
    },
    Builtin {
        name: "IsAlnum",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 7,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsAlpha",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 8,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsDigit",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsFloat",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 3,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsInteger",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsLower",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 10,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsNumber",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsObject",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: is_object,
    },
    Builtin {
        name: "IsSpace",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 11,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsTime",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsUpper",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 9,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "IsXDigit",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 6,
        parameters: &[Parameter::Value],
        handler: is_value,
    },
    Builtin {
        name: "Type",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: type_of,
    },
    // -- the patterns ----------------------------------------------------------------
    Builtin {
        name: "RegExMatch",
        kind: BuiltinKind::Function,
        min: 2,
        max: 4,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::Value,
            Parameter::Reference,
            Parameter::Value,
        ],
        handler: regex_match,
    },
    Builtin {
        name: "RegExReplace",
        kind: BuiltinKind::Function,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Reference,
            Parameter::Value,
            Parameter::Value,
        ],
        handler: regex_replace,
    },
    // -- the screen --------------------------------------------------------------
    Builtin {
        name: "Click",
        kind: BuiltinKind::Function,
        min: 0,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: click,
    },
    Builtin {
        name: "MouseGetPos",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Value,
        ],
        handler: mouse_get_pos,
    },
    Builtin {
        name: "MouseMove",
        kind: BuiltinKind::Command,
        min: 2,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: mouse_move,
    },
    Builtin {
        name: "PixelGetColor",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: pixel_get_color,
    },
    Builtin {
        name: "PixelSearch",
        kind: BuiltinKind::Command,
        min: 7,
        max: 8,
        variant: 0,
        parameters: &[
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
        ],
        handler: pixel_search,
    },
    Builtin {
        name: "Screenshot",
        kind: BuiltinKind::Command,
        min: 5,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: screenshot,
    },
    // -- the windows -------------------------------------------------------------
    Builtin {
        name: "WinActive",
        kind: BuiltinKind::Function,
        min: 0,
        max: 4,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: window_active,
    },
    Builtin {
        name: "WinExist",
        kind: BuiltinKind::Function,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: window_exist,
    },
    Builtin {
        name: "WinGetTitle",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_title,
    },
    // -- the rest ----------------------------------------------------------------
    Builtin {
        name: "MsgBox",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: msg_box,
    },
    Builtin {
        name: "Run",
        kind: BuiltinKind::Command,
        min: 1,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: run,
    },
    Builtin {
        name: "Send",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: send,
    },
    Builtin {
        name: "Sleep",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sleep,
    },
    // -- the names this runtime answers -------------------------------------------
    Builtin {
        name: "ClipWait",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: clip_wait,
    },
    Builtin {
        name: "DirCreate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: dir_create,
    },
    Builtin {
        name: "DirDelete",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: dir_delete,
    },
    Builtin {
        name: "DirExist",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: dir_exist,
    },
    Builtin {
        name: "EnvGet",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: env_get,
    },
    Builtin {
        name: "EnvSet",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: env_set,
    },
    Builtin {
        name: "Exit",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: exit_script,
    },
    Builtin {
        name: "ExitApp",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: exit_script,
    },
    Builtin {
        name: "FileAppend",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_append,
    },
    Builtin {
        name: "FileDelete",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_delete,
    },
    Builtin {
        name: "FileExist",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_exist,
    },
    Builtin {
        name: "FileRead",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_read,
    },
    Builtin {
        name: "GetMethod",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: get_method,
    },
    Builtin {
        name: "HasBase",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: has_base,
    },
    Builtin {
        name: "HasMethod",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: has_method,
    },
    Builtin {
        name: "HasProp",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: has_prop,
    },
    Builtin {
        name: "IsSetRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Reference],
        handler: is_set_ref,
    },
    Builtin {
        name: "KeyWait",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: key_wait,
    },
    Builtin {
        name: "RunWait",
        kind: BuiltinKind::Function,
        min: 1,
        max: 4,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Reference,
        ],
        handler: run,
    },
    Builtin {
        name: "SendMode",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: send_mode,
    },
    Builtin {
        name: "SetKeyDelay",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_key_delay,
    },
    Builtin {
        name: "SetTimer",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_timer,
    },
    Builtin {
        name: "SplitPath",
        kind: BuiltinKind::Function,
        min: 1,
        max: 6,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
        ],
        handler: split_path,
    },
    Builtin {
        name: "StrSplit",
        kind: BuiltinKind::Command,
        min: 1,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_split,
    },
    Builtin {
        name: "VarSetStrCapacity",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Reference, Parameter::Value],
        handler: var_set_str_capacity,
    },
    // -- the names of Windows ------------------------------------------------
    Builtin {
        name: "BlockInput",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "CallbackCreate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "CallbackFree",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "CaretGetPos",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Reference, Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComCall",
        kind: BuiltinKind::Function,
        min: 2,
        max: 255,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjActive",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjConnect",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjFlags",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjFromPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjGet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjQuery",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjType",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjValue",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlAddItem",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_add_item,
    },
    Builtin {
        name: "ControlChooseIndex",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_choose_index,
    },
    Builtin {
        name: "ControlChooseString",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_choose_string,
    },
    Builtin {
        name: "ControlClick",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_act,
    },
    Builtin {
        name: "ControlDeleteItem",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_delete_item,
    },
    Builtin {
        name: "ControlFindItem",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_find_item,
    },
    Builtin {
        name: "ControlFocus",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: control_act,
    },
    Builtin {
        name: "ControlGetChecked",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_checked,
    },
    Builtin {
        name: "ControlGetChoice",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_choice,
    },
    Builtin {
        name: "ControlGetClassNN",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_classnn,
    },
    Builtin {
        name: "ControlGetEnabled",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: control_flag,
    },
    Builtin {
        name: "ControlGetExStyle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlGetFocus",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_focus,
    },
    Builtin {
        name: "ControlGetHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_hwnd,
    },
    Builtin {
        name: "ControlGetIndex",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_index,
    },
    Builtin {
        name: "ControlGetItems",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_items,
    },
    Builtin {
        name: "ControlGetPos",
        kind: BuiltinKind::Command,
        min: 0,
        max: 9,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_pos,
    },
    Builtin {
        name: "ControlGetStyle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlGetText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_get_text,
    },
    Builtin {
        name: "ControlGetVisible",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_flag,
    },
    Builtin {
        name: "ControlHide",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_visibility,
    },
    Builtin {
        name: "ControlHideDropDown",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_hide_drop_down,
    },
    Builtin {
        name: "ControlMove",
        kind: BuiltinKind::Command,
        min: 0,
        max: 9,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_move,
    },
    Builtin {
        name: "ControlSend",
        kind: BuiltinKind::Command,
        min: 1,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_send,
    },
    Builtin {
        name: "ControlSendText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_send_text,
    },
    Builtin {
        name: "ControlSetChecked",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_set_checked,
    },
    Builtin {
        name: "ControlSetEnabled",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_set_enabled,
    },
    Builtin {
        name: "ControlSetExStyle",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlSetStyle",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlSetText",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_set_text,
    },
    Builtin {
        name: "ControlShow",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_show,
    },
    Builtin {
        name: "ControlShowDropDown",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_show_drop_down,
    },
    Builtin {
        name: "DetectHiddenText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_detect_hidden_text,
    },
    Builtin {
        name: "DetectHiddenWindows",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_detect_hidden_windows,
    },
    Builtin {
        name: "DllCall",
        kind: BuiltinKind::Function,
        min: 1,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: dll_call,
    },
    Builtin {
        name: "DriveEject",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetCapacity",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetFilesystem",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 3,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetList",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetSerial",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetSpaceFree",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetStatus",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 6,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveGetStatusCD",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetType",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 7,
        parameters: &[Parameter::Value],
        handler: drive_get,
    },
    Builtin {
        name: "DriveLock",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveRetract",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveSetLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveUnlock",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "Edit",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditGetCurrentCol",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditGetCurrentLine",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditGetLine",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditGetLineCount",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditGetSelectedText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "EditPaste",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "FileCreateShortcut",
        kind: BuiltinKind::Command,
        min: 2,
        max: 9,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_create_shortcut,
    },
    Builtin {
        name: "FileGetShortcut",
        kind: BuiltinKind::Command,
        min: 1,
        max: 8,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_get_shortcut,
    },
    Builtin {
        name: "FileGetVersion",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "FileRecycle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_recycle,
    },
    Builtin {
        name: "FileRecycleEmpty",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_recycle_empty,
    },
    Builtin {
        name: "GroupActivate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "GroupAdd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "GroupClose",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "GroupDeactivate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "GuiCtrlFromHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "GuiFromHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "IL_Add",
        kind: BuiltinKind::Command,
        min: 2,
        max: 4,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "IL_Create",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "IL_Destroy",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ListViewGetContent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 6,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "MenuFromHandle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "MenuSelect",
        kind: BuiltinKind::Command,
        min: 0,
        max: 11,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "MonitorGet",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::VarRef,
            Parameter::VarRef,
            Parameter::VarRef,
            Parameter::VarRef,
        ],
        handler: monitor_get,
    },
    Builtin {
        name: "MonitorGetCount",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: monitor_get_count,
    },
    Builtin {
        name: "MonitorGetName",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: monitor_get_name,
    },
    Builtin {
        name: "MonitorGetPrimary",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: monitor_get_primary,
    },
    Builtin {
        name: "MonitorGetWorkArea",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[
            Parameter::Value,
            Parameter::VarRef,
            Parameter::VarRef,
            Parameter::VarRef,
            Parameter::VarRef,
        ],
        handler: monitor_get_work_area,
    },
    Builtin {
        name: "ObjAddRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_add_ref,
    },
    Builtin {
        name: "ObjBindMethod",
        kind: BuiltinKind::Function,
        min: 1,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_bind_method,
    },
    // -- the object model ---------------------------------------------------------
    Builtin {
        name: "ObjFromPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ObjFromPtrAddRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ObjGetBase",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_get_base,
    },
    Builtin {
        name: "ObjGetCapacity",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_get_capacity,
    },
    Builtin {
        name: "ObjHasOwnProp",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_has_own_prop,
    },
    Builtin {
        name: "ObjOwnPropCount",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_own_prop_count,
    },
    Builtin {
        name: "ObjOwnProps",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_own_props,
    },
    Builtin {
        name: "ObjPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ObjPtrAddRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ObjRelease",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ObjSetBase",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_set_base,
    },
    Builtin {
        name: "ObjSetCapacity",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_set_capacity,
    },
    Builtin {
        name: "OnClipboardChange",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "OnMessage",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "OutputDebug",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: output_debug,
    },
    Builtin {
        name: "PostMessage",
        kind: BuiltinKind::Command,
        min: 1,
        max: 8,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ProcessClose",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_close,
    },
    Builtin {
        name: "ProcessExist",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_exist,
    },
    Builtin {
        name: "ProcessGetName",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_get_name,
    },
    Builtin {
        name: "ProcessGetParent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_get_parent,
    },
    Builtin {
        name: "ProcessGetPath",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_get_path,
    },
    Builtin {
        name: "ProcessSetPriority",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_set_priority,
    },
    Builtin {
        name: "ProcessWait",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_wait,
    },
    Builtin {
        name: "ProcessWaitClose",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: process_wait_close,
    },
    Builtin {
        name: "RegCreateKey",
        kind: BuiltinKind::Function,
        min: 0,
        max: 1,
        variant: 123,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "RegDelete",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 123,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "RegDeleteKey",
        kind: BuiltinKind::Function,
        min: 0,
        max: 1,
        variant: 123,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "RegRead",
        kind: BuiltinKind::Function,
        min: 0,
        max: 3,
        variant: 123,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "RegWrite",
        kind: BuiltinKind::Function,
        min: 0,
        max: 4,
        variant: 123,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "RunAs",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SendMessage",
        kind: BuiltinKind::Command,
        min: 1,
        max: 9,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SetCapsLockState",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: set_lock_state,
    },
    Builtin {
        name: "SetControlDelay",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_control_delay,
    },
    Builtin {
        name: "SetNumLockState",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: set_lock_state,
    },
    Builtin {
        name: "SetRegView",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_reg_view,
    },
    Builtin {
        name: "SetScrollLockState",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 3,
        parameters: &[Parameter::Value],
        handler: set_lock_state,
    },
    Builtin {
        name: "SetTitleMatchMode",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_title_match_mode,
    },
    Builtin {
        name: "SetWinDelay",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_win_delay,
    },
    Builtin {
        name: "Shutdown",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SoundBeep",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_beep,
    },
    Builtin {
        name: "SoundGetInterface",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SoundGetMute",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_get_mute,
    },
    Builtin {
        name: "SoundGetName",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SoundGetVolume",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_get_volume,
    },
    Builtin {
        name: "SoundPlay",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_play,
    },
    Builtin {
        name: "SoundSetMute",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_set_mute,
    },
    Builtin {
        name: "SoundSetVolume",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_set_volume,
    },
    Builtin {
        name: "StatusBarGetText",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "StatusBarWait",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "StrGet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_get,
    },
    Builtin {
        name: "StrPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_ptr,
    },
    Builtin {
        name: "StrPut",
        kind: BuiltinKind::Function,
        min: 1,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_put,
    },
    Builtin {
        name: "SysGet",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sys_get,
    },
    Builtin {
        name: "SysGetIPAddresses",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sys_get_ip_addresses,
    },
    Builtin {
        name: "ToolTip",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: tooltip,
    },
    Builtin {
        name: "TraySetIcon",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "TrayTip",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "WinActivate",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_activate,
    },
    Builtin {
        name: "WinActivateBottom",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_activate_bottom,
    },
    Builtin {
        name: "WinClose",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_close,
    },
    Builtin {
        name: "WinGetClass",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: window_query,
    },
    Builtin {
        name: "WinGetClientPos",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 0,
        parameters: &[
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
        ],
        handler: win_get_client_pos,
    },
    Builtin {
        name: "WinGetControls",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_controls,
    },
    Builtin {
        name: "WinGetControlsHwnd",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_controls_hwnd,
    },
    Builtin {
        name: "WinGetCount",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: window_query,
    },
    Builtin {
        name: "WinGetExStyle",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_style,
    },
    Builtin {
        name: "WinGetID",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 3,
        parameters: &[Parameter::Value],
        handler: window_query,
    },
    Builtin {
        name: "WinGetIDLast",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: window_query,
    },
    Builtin {
        name: "WinGetList",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: window_query,
    },
    Builtin {
        name: "WinGetMinMax",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_min_max,
    },
    Builtin {
        name: "WinGetPID",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_pid,
    },
    Builtin {
        name: "WinGetPos",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 0,
        parameters: &[
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
        ],
        handler: win_get_pos,
    },
    Builtin {
        name: "WinGetProcessName",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_process,
    },
    Builtin {
        name: "WinGetProcessPath",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: win_get_process,
    },
    Builtin {
        name: "WinGetStyle",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_style,
    },
    Builtin {
        name: "WinGetText",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_text,
    },
    Builtin {
        name: "WinGetTransColor",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_trans_color,
    },
    Builtin {
        name: "WinGetTransparent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_transparent,
    },
    Builtin {
        name: "WinHide",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_hide,
    },
    Builtin {
        name: "WinKill",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_kill,
    },
    Builtin {
        name: "WinMaximize",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_maximize,
    },
    Builtin {
        name: "WinMinimize",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_minimize,
    },
    Builtin {
        name: "WinMinimizeAll",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_minimize_all,
    },
    Builtin {
        name: "WinMinimizeAllUndo",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_minimize_all_undo,
    },
    Builtin {
        name: "WinMove",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_move,
    },
    Builtin {
        name: "WinMoveBottom",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_move_bottom,
    },
    Builtin {
        name: "WinMoveTop",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_move_top,
    },
    Builtin {
        name: "WinRedraw",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_redraw,
    },
    Builtin {
        name: "WinRestore",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_restore,
    },
    Builtin {
        name: "WinSetAlwaysOnTop",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_always_on_top,
    },
    Builtin {
        name: "WinSetEnabled",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_enabled,
    },
    Builtin {
        name: "WinSetExStyle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_style,
    },
    Builtin {
        name: "WinSetRegion",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_region,
    },
    Builtin {
        name: "WinSetStyle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_style,
    },
    Builtin {
        name: "WinSetTitle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_title,
    },
    Builtin {
        name: "WinSetTransColor",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: win_trans_color,
    },
    Builtin {
        name: "WinSetTransparent",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_transparent,
    },
    Builtin {
        name: "WinShow",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_show,
    },
    Builtin {
        name: "WinWait",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_wait,
    },
    Builtin {
        name: "WinWaitActive",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_wait_active,
    },
    Builtin {
        name: "WinWaitClose",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_wait_close,
    },
    Builtin {
        name: "WinWaitNotActive",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_wait_not_active,
    },
    // -- the hotkeys --------------------------------------------------------
    Builtin {
        name: "GetKeyState",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: get_key_state,
    },
    Builtin {
        name: "HotIf",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: hotif,
    },
    Builtin {
        name: "HotIfWinActive",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: hotif_window,
    },
    Builtin {
        name: "HotIfWinExist",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: hotif_window,
    },
    Builtin {
        name: "HotIfWinNotActive",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: hotif_window,
    },
    Builtin {
        name: "HotIfWinNotExist",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 3,
        parameters: &[Parameter::Value],
        handler: hotif_window,
    },
    Builtin {
        name: "Hotkey",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: hotkey,
    },
    Builtin {
        name: "Hotstring",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: hotstring,
    },
    Builtin {
        name: "InstallKeybdHook",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: install_hook,
    },
    Builtin {
        name: "InstallMouseHook",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: install_hook,
    },
    Builtin {
        name: "KeyHistory",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: key_history,
    },
    Builtin {
        name: "ListHotkeys",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: list_hotkeys,
    },
    // -- the names of the script that this runtime cannot answer -----------------
    Builtin {
        name: "CoordMode",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: coord_mode,
    },
    Builtin {
        name: "Critical",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 108,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DateAdd",
        kind: BuiltinKind::Command,
        min: 3,
        max: 3,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: date_add,
    },
    Builtin {
        name: "DateDiff",
        kind: BuiltinKind::Command,
        min: 3,
        max: 3,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: date_diff,
    },
    Builtin {
        name: "DirCopy",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: dir_copy,
    },
    Builtin {
        name: "DirMove",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: dir_move,
    },
    Builtin {
        name: "DirSelect",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: dir_select,
    },
    Builtin {
        name: "Download",
        kind: BuiltinKind::Command,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: download,
    },
    Builtin {
        name: "FileCopy",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 6,
        parameters: &[Parameter::Value],
        handler: file_copy,
    },
    Builtin {
        name: "FileEncoding",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_encoding,
    },
    Builtin {
        name: "FileGetAttrib",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_get_attrib,
    },
    Builtin {
        name: "FileGetSize",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_get_size,
    },
    Builtin {
        name: "FileGetTime",
        kind: BuiltinKind::Command,
        min: 0,
        max: 2,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_get_time,
    },
    Builtin {
        name: "FileInstall",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_install,
    },
    Builtin {
        name: "FileMove",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 6,
        parameters: &[Parameter::Value],
        handler: file_move,
    },
    Builtin {
        name: "FileOpen",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_open,
    },
    Builtin {
        name: "FileSelect",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_select,
    },
    Builtin {
        name: "FileSetAttrib",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_set_attrib,
    },
    Builtin {
        name: "FileSetTime",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: file_set_time,
    },
    Builtin {
        name: "FormatTime",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 2,
        parameters: &[Parameter::Value],
        handler: format_time,
    },
    Builtin {
        name: "GetKeyName",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: get_key_name,
    },
    Builtin {
        name: "GetKeySC",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: get_key_sc,
    },
    Builtin {
        name: "GetKeyVK",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: get_key_vk,
    },
    Builtin {
        name: "ImageSearch",
        kind: BuiltinKind::Command,
        min: 7,
        max: 7,
        variant: 0,
        parameters: &[
            Parameter::Reference,
            Parameter::Reference,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
            Parameter::Value,
        ],
        handler: image_search,
    },
    Builtin {
        name: "IniDelete",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 6,
        parameters: &[Parameter::Value],
        handler: ini_delete,
    },
    Builtin {
        name: "IniRead",
        kind: BuiltinKind::Command,
        min: 1,
        max: 4,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: ini_read,
    },
    Builtin {
        name: "IniWrite",
        kind: BuiltinKind::Command,
        min: 3,
        max: 4,
        variant: 4,
        parameters: &[Parameter::Value],
        handler: ini_write,
    },
    Builtin {
        name: "InputBox",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: input_box,
    },
    Builtin {
        name: "IsLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: is_label,
    },
    Builtin {
        name: "ListLines",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 114,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ListVars",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 115,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "LoadPicture",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value, Parameter::Value, Parameter::Reference],
        handler: load_picture,
    },
    Builtin {
        name: "MouseClick",
        kind: BuiltinKind::Command,
        min: 0,
        max: 7,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: mouse_click,
    },
    Builtin {
        name: "MouseClickDrag",
        kind: BuiltinKind::Command,
        min: 0,
        max: 7,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: mouse_click_drag,
    },
    Builtin {
        name: "OnError",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: on_error,
    },
    Builtin {
        name: "OnExit",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: on_exit,
    },
    Builtin {
        name: "Pause",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: pause,
    },
    Builtin {
        name: "Persistent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 111,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "Reload",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 112,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SendEvent",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: send_event,
    },
    Builtin {
        name: "SendInput",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: send_input,
    },
    Builtin {
        name: "SendLevel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: send_level,
    },
    Builtin {
        name: "SendPlay",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: send_play,
    },
    Builtin {
        name: "SendText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: send_text,
    },
    Builtin {
        name: "SetDefaultMouseSpeed",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: set_default_mouse_speed,
    },
    Builtin {
        name: "SetMouseDelay",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: set_mouse_delay,
    },
    Builtin {
        name: "SetStoreCapsLockMode",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 5,
        parameters: &[Parameter::Value],
        handler: set_store_caps_lock_mode,
    },
    Builtin {
        name: "SetWorkingDir",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_working_dir,
    },
    Builtin {
        name: "StrTitle",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: str_title,
    },
    Builtin {
        name: "Suspend",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: suspend,
    },
    Builtin {
        name: "Thread",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 109,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "VerCompare",
        kind: BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: ver_compare,
    },
];

/// The parameters of a command that may not be left out.
///
/// The table of the binary says how many parameters a command declares,
/// and the documentation says which of them may be left out. There are
/// commands that declare fewer than they need: the `FileGetAttrib` takes
/// the name of the file, and it takes it whether the table says so or
/// not. Measured on the binary of v2.0.19 on 2026-10-31: the refusal of
/// such a command is not the one of the count -- "Too few parameters
/// passed to function." -- but the one of the parameter that is missing:
/// "Parameter #1 of FileGetAttrib must not be omitted in this case."
///
/// The numbers are the places of the parameters, counted from one.
const THE_REQUIRED: &[(&str, &[u8])] = &[
    ("ControlGetPos", &[5]),
    ("FileGetAttrib", &[1]),
    ("FileGetSize", &[1]),
    ("FileGetTime", &[1]),
    ("FileGetVersion", &[1]),
    ("FileSetTime", &[2]),
    ("ListViewGetContent", &[2]),
    ("MenuSelect", &[3]),
    ("MouseClickDrag", &[4, 5]),
];

/// The parameters of a command that may not be left out, in order.
///
/// The first one that the call does not carry is the one that the
/// refusal names.
pub fn required_parameters(name: &str) -> &'static [u8] {
    THE_REQUIRED
        .iter()
        .find(|(command, _)| *command == name)
        .map(|(_, numbers)| *numbers)
        .unwrap_or(&[])
}

/// The parameters of a built-in that take a reference.
///
/// The flag of the table of v2 is the place of the first parameter that
/// is written back, and the parameters after it are the ones that follow
/// it. A reference in any other place is a value where a variable is
/// wanted, and the built-in says so.
pub fn reference_parameters(name: &str) -> Vec<usize> {
    find(name)
        .map(|builtin| {
            builtin
                .parameters
                .iter()
                .enumerate()
                .filter(|(_, shape)| **shape == Parameter::Reference)
                .map(|(index, _)| index)
                .collect()
        })
        .unwrap_or_default()
}

/// The shape of the parameters of a built-in.
///
/// The shape is the column of the table: the value of an expression, the
/// reference that the caller passes, the name of a variable of the script,
/// or the name of the variable that the built-in writes into.
pub fn parameters_of(name: &str) -> &'static [Parameter] {
    find(name).map(|builtin| builtin.parameters).unwrap_or(&[])
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
    BUILTINS
        .iter()
        .find(|builtin| builtin.name.to_lowercase() == wanted)
}

/// The value that a built-in complains about, when its message does
/// not name the parameter it is about.
///
/// v2 carries the value in the `Extra` of the failure, and most of the
/// messages name the parameter it is about. The message of the options
/// of an input box that v2 does not know does not: the value there is
/// the option itself, and the built-in says which one it is.
pub fn the_extra_of(name: &str, message: &str, values: &[Value]) -> Option<String> {
    // The failure of a mode of the window titles carries the value that was
    // not a mode in the `Extra`: measured on the binary of v2.0.19 on
    // 2026-11-09, `SetTitleMatchMode("Bogus")` carries "Bogus",
    // `SetTitleMatchMode(4)` carries "4" and `SetTitleMatchMode(0)` carries
    // "0".
    if name == "SetTitleMatchMode" && message == "Parameter #1 of SetTitleMatchMode is invalid." {
        return values.first().map(|value| value.to_text());
    }
    // The failure of a window query carries the window it was asked about in
    // the `Extra`, and a window that was named by its id leaves it empty.
    //
    // Measured on the binary of v2.0.19 on 2026-11-09: `WinGetID("zzz")` is
    // "Target window not found." with "zzz" in the `Extra`, and `WinGetID(0)`
    // carries nothing.
    if matches!(
        name,
        "WinGetID" | "WinGetIDLast" | "WinGetClass" | "WinGetTitle"
    ) && message == "Target window not found."
    {
        return match values.first() {
            Some(Value::Int(_)) | None => None,
            Some(value) => Some(value.to_text()),
        };
    }
    // The base of an object is written with the `ObjSetBase`, and the value
    // that is not an object goes in the `Extra` of the refusal.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31.
    if name == "ObjSetBase" && message.starts_with("Expected an Object but got") {
        return values.get(1).map(crate::runtime::the_extra_of_the_value);
    }
    // The button of a `MouseClick` is the first parameter, and the value
    // that the refusal carries has the space at the end of it cut.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31: "Left " is refused
    // with the "Left" in the `Extra`, and "  nonsense  " with the
    // " nonsense" in it.
    if name == "MouseClick" && message == "Parameter #1 of MouseClick is invalid." {
        return values
            .first()
            .map(|value| value.to_text().trim_end().to_string());
    }
    // The failure of a copy or of a move of a file carries the number of
    // the files that failed.
    //
    // Measured on the binary of v2.0.19 on 2026-11-01: the copy of a file
    // that is not there fails with "Failed", and the `Extra` of the failure
    // is 1.
    if matches!(name, "FileCopy" | "FileMove") && message == "Failed" {
        return Some("1".to_string());
    }
    // The target of a `StrPut` is a buffer, and a buffer that cannot hold
    // the text is refused with the kind of the value in the `Extra`: the
    // name of the class, and not the text of the object.
    //
    // Measured on the binary of v2.0.19 on 2026-11-09.
    if name == "StrPut"
        && message == "Parameter #2 of StrPut is invalid."
        && matches!(values.get(1), Some(Value::Object(_)))
    {
        return Some("Buffer".to_string());
    }
    if message != "Invalid option." {
        return None;
    }
    match name {
        "InputBox" => {
            let options = values
                .get(2)
                .map(|value| value.to_text())
                .unwrap_or_default();
            crate::dialogs::the_option_that_failed(&options)
        }
        // The options of a file are in the second parameter of the read and
        // in the third one of the write.
        "FileRead" => {
            let options = values
                .get(1)
                .map(|value| value.to_text())
                .unwrap_or_default();
            crate::files::the_option_that_failed(&options, true)
        }
        "FileAppend" => {
            let options = values
                .get(2)
                .map(|value| value.to_text())
                .unwrap_or_default();
            crate::files::the_option_that_failed(&options, false)
        }
        _ => None,
    }
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
pub(crate) fn dll_call(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    dll_call_with_the_kind(rt, values, line).map_err(|(_, error)| error)
}

/// The call of a library, with the kind of the failure.
///
/// The kind says what failed: the call itself, the library, or the function
/// in the library. The v1 turns it into the code that it writes in the
/// `ErrorLevel`, and the v2 does not use it: the wrapper above drops it.
pub(crate) fn dll_call_with_the_kind(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<Value, (NativeFailureKind, RuntimeError)> {
    let Some(first) = values.first() else {
        return Err((
            NativeFailureKind::Invalid,
            RuntimeError {
                message: "DllCall wants the library and the function.".into(),
                line,
            },
        ));
    };
    let Value::Text(target) = first else {
        return Err((
            NativeFailureKind::Invalid,
            RuntimeError {
                message: expected_type(TypeName::String, first.type_name()),
                line,
            },
        ));
    };
    let (library, function) = crate::dll::split_target(target);
    let rest = &values[1..];
    let (returns, convention, rest) = if rest.len() % 2 == 1 {
        let last = rest.last().expect("there is one");
        let Value::Text(text) = last else {
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: expected_type(TypeName::String, last.type_name()),
                    line,
                },
            ));
        };
        let (convention, name) = split_convention(text);
        let Some((kind, _)) = crate::dll::parse_type(name) else {
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: "Invalid return type.".into(),
                    line,
                },
            ));
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
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: expected_type(TypeName::String, rest[index].type_name()),
                    line,
                },
            ));
        };
        let Some((kind, address)) = crate::dll::parse_type(text) else {
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: "Invalid arg type.".into(),
                    line,
                },
            ));
        };
        let (argument, reference) = marshal(rt, kind, address, &rest[index + 1], line)
            .map_err(|error| (NativeFailureKind::Invalid, error))?;
        arguments.push(argument);
        references.push(reference);
        index += 2;
    }
    let mut call = NativeCall {
        library,
        function,
        convention,
        returns,
        arguments,
    };
    let answer = rt.desktop().call_native(&mut call).map_err(|failure| {
        (
            failure.kind,
            RuntimeError {
                message: failure.reason,
                line,
            },
        )
    })?;
    // The arguments that are the address of a variable come back into
    // the variable.
    for (argument, reference) in call.arguments.iter().zip(references) {
        // Only the argument that is the address of a variable comes back.
        if !argument.address {
            continue;
        }
        let Some(reference) = reference else { continue };
        let NativeValue::Bytes(bytes) = &argument.value else {
            continue;
        };
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
                return Err(RuntimeError {
                    message: rt.the_unassigned(&reference.name),
                    line,
                });
            };
            (current, Some(value.clone()))
        }
        other => (other.clone(), None),
    };
    // A text goes out as its bytes.
    if kind.is_text() {
        let Value::Text(text) = &value else {
            return Err(RuntimeError {
                message: expected_type(TypeName::String, value.type_name()),
                line,
            });
        };
        let bytes = crate::dll::encode(text, kind);
        return Ok((
            NativeArg {
                kind,
                address,
                value: NativeValue::Bytes(bytes),
            },
            reference,
        ));
    }
    // A pointer may be an object, and then it is the address of the
    // object that goes.
    if matches!(kind, NativeType::Ptr | NativeType::UPtr) {
        if let Value::Object(_) = &value {
            if let Some(place) = rt.property_of(&value, "ptr", line) {
                let place = place.to_number_strict().map_err(|got| RuntimeError {
                    message: expected_type(TypeName::Integer, got),
                    line,
                })?;
                return Ok((
                    NativeArg {
                        kind,
                        address,
                        value: NativeValue::Integer(place as i64),
                    },
                    reference,
                ));
            }
        }
    }
    let number = native_number(rt, &value, line)?;
    let value = if kind.is_float() {
        NativeValue::Float(number)
    } else {
        NativeValue::Integer(number as i64)
    };
    Ok((
        NativeArg {
            kind,
            address,
            value,
        },
        reference,
    ))
}

/// The value as a number, the way a native call wants it.
fn native_number(rt: &Runtime<'_>, value: &Value, line: u32) -> Result<f64, RuntimeError> {
    match rt.number(value, line) {
        Ok(number) => Ok(number),
        Err(RuntimeFailure::Error(error)) => Err(error),
        Err(RuntimeFailure::Detailed(error, _)) => Err(error),
        Err(RuntimeFailure::Fatal(error)) => Err(error),
        Err(RuntimeFailure::Timeout(_)) => Err(RuntimeError {
            message: "the script ran out of time.".into(),
            line,
        }),
        Err(RuntimeFailure::Flow(_)) => Err(RuntimeError {
            message: "a number was expected.".into(),
            line,
        }),
        Err(RuntimeFailure::Thrown(_)) => Err(RuntimeError {
            message: "a number was expected.".into(),
            line,
        }),
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

/// The variant that says the name is refused.
///
/// The variants below it belong to the handlers that answer a value, and
/// the ones from it up are the refusals: the reason of a refusal is
/// told from the variant of the row.
pub const REFUSAL: u8 = 100;

/// The reason the tree does not answer a name, when it does not.
///
/// The refusals are the names whose handler is the refusal, and the
/// variant of the row says which reason it is.
pub fn refusal_of(name: &str) -> Option<&'static str> {
    find(name).and_then(refusal_of_the)
}

/// The reason the tree does not answer a built-in, when it does not.
///
/// The refusal is told from the variant of the row, and the row may be
/// the one of any dialect.
pub fn refusal_of_the(builtin: &Builtin) -> Option<&'static str> {
    if builtin.variant < REFUSAL {
        return None;
    }
    Some(refusal_message(builtin.variant - REFUSAL))
}

/// The reason of a refusal.
///
/// The variant is the one of the row, without the offset of the
/// refusals.
fn refusal_message(variant: u8) -> &'static str {
    match variant {
        0 => "this is a name of Windows, and this desktop is not Windows.",
        1 => "this runtime has no hotkeys yet.",
        2 => {
            "the clock of this runtime counts from an arbitrary origin, and it does not know the date."
        }
        3 => "this runtime is one script that ends when it ends, and it cannot do that.",
        4 => "this desktop cannot serve that with the files it has.",
        5 => {
            "the pointer and the keyboard of this runtime are the ones of the box, and they have no setting of that kind."
        }
        // The reason of a method: a name of a class that the tree does
        // not carry. The names of v2 that were in this slot were not
        // methods at all -- the tables that were read out of the binary
        // had them in a table of methods -- and the reason moved with
        // them.
        6 => "a method of this runtime is not bound to an object.",
        // The shape of the runtime: each name of the group says what it
        // wants, and the group of them is gone.
        8 => "this runtime runs one script in one thread, and it cannot make it critical.",
        9 => "this runtime runs one script in one thread, and it cannot start another.",
        10 => "this runtime cannot wait for a key to continue: the script ends when it ends.",
        11 => "this runtime is a script that ends when it ends, and it cannot be kept alive.",
        12 => "this runtime runs one script once, and it cannot run itself again.",
        13 => "this runtime cannot stop itself: it would never end.",
        14 => {
            "the lines are listed in a window of v2, and this runtime has no window to show them in."
        }
        15 => {
            "the variables are listed in a window of v2, and this runtime has no window to show them in."
        }
        16 => {
            "the question of the user is asked in a window of v2, and this runtime has no window to ask in."
        }
        17 => "the download is made by the desktop, and this desktop does not reach the network.",
        18 => {
            "the picture is read by the desktop, and this desktop does not read the images of the machine."
        }
        19 => {
            "the search of a picture is made by the desktop, and this desktop does not read the images of the machine."
        }
        20 => "the handler of an error is not carried: this runtime ends the script when it fails.",
        21 => crate::settings::NO_TEXT_OF_A_WINDOW,
        23 => "the registry is a thing of Windows, and this runtime does not carry it.",
        // No name of the surface has this variant: the `ObjBindMethod` was
        // the last one, and it is carried now. The arm is the fallback of a
        // variant that the table does not have.
        _ => crate::dialect::NOT_CARRIED_YET,
    }
}

/// A name that this runtime carries, and cannot answer.
///
/// The variant says why: the name is shaped for Windows, it is a
/// hotkey, or it is a name of the script that this runtime does not
/// have. The script knows which name it called, so the reason names the
/// group and not the name.
pub(crate) fn not_carried(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let message = refusal_message(variant.saturating_sub(REFUSAL));
    Err(RuntimeError {
        message: message.to_string(),
        line,
    })
}

// -- the environment ---------------------------------------------------------------

/// `EnvGet`: the value of an environment variable.
pub(crate) fn env_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    // variable that the environment does not have is an empty text, and it is
    // not a failure. Measured on the binary of AutoHotkey v1.1.37.02 on
    // 2026-11-05: the `EnvGet` of a name that is not there leaves the output
    // empty, and the `ErrorLevel` is zero.
    let value = rt
        .desktop()
        .env_get(&name)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?
        .unwrap_or_default();
    Ok(Value::Text(value))
}

/// `EnvSet`: set an environment variable, and answer its value.
pub(crate) fn env_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    let value = text_of(values, 1);
    rt.desktop()
        .env_set(&name, &value)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Text(value))
}

// -- the files of the disk ----------------------------------------------------------

/// `FileExist`: the attributes of the first entry that matches.
///
/// v2 answers with the attributes of the file and not with a number: a
/// file that is there is `A`, a folder is `D`, and a file that is not
/// there is the empty text.
///
/// Measured on the binary of v2.0.19 on 2026-10-28 and on 2026-10-31:
/// the pattern may carry the wildcards `*` and `?`, and the answer is
/// the attributes of the first entry that matches, a folder among them.
pub(crate) fn file_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let pattern = text_of(values, 0);
    let attributes = rt
        .desktop()
        .attributes_of_the_pattern(&pattern, "")
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Text(attributes.concat()))
}

/// `DirExist`: the attributes of the first folder that matches.
///
/// The answer of v2 is `D` when it is there, and the empty text when it
/// is not. A file that matches the pattern is not a folder, and the
/// answer is the empty text.
pub(crate) fn dir_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let pattern = text_of(values, 0);
    let attributes = rt
        .desktop()
        .attributes_of_the_pattern(&pattern, "D")
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Text(attributes.concat()))
}

/// `FileDelete`: take a file away, or the files that a pattern names.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the name may be a pattern
/// whose last part holds a `*`, which is any run of characters, and a `?`,
/// which is one character; a pattern that names no file is not a failure.
pub(crate) fn file_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = crate::files::the_name_of_a_file(values, 0, 1, "FileDelete", line)?;
    let files = if crate::files::is_a_pattern(&path) {
        crate::files::the_files_of_the_pattern(rt, &path, line)?
    } else {
        vec![path]
    };
    for file in files {
        rt.desktop()
            .delete_file(&file)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
    }
    // Measured on the binary of v2.0.19 on 2026-11-01: the answer of the
    // command is an empty text.
    Ok(Value::Text(String::new()))
}

/// `DirCreate`: make a directory.
pub(crate) fn dir_create(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    rt.desktop()
        .create_dir(&path)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    // Measured on the binary of v2.0.19 on 2026-11-01: the answer of the
    // command is an empty text.
    Ok(Value::Text(String::new()))
}

/// `DirDelete`: take a directory away.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the second parameter
/// is the recursion, and a folder that is not empty stays without it.
pub(crate) fn dir_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let recurse = crate::files::flag_of(values, 1, "DirDelete", line)?;
    rt.desktop()
        .dir_delete(&path, recurse)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Text(path))
}

/// `FileCreateShortcut(LinkFile, Target, WorkingDir, Args, Description, Icon,
/// IconNumber, Run)`: make a shortcut to a program.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `FileCreateShortcut`
/// answers the empty text and makes the file, and it makes it even for a
/// target that is not there.
pub(crate) fn file_create_shortcut(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    let shortcut = text_of(values, 1);
    let working_directory = text_of(values, 2);
    let arguments = text_of(values, 3);
    let description = text_of(values, 4);
    let icon = text_of(values, 5);
    rt.desktop()
        .create_shortcut(
            &target,
            &shortcut,
            &working_directory,
            &arguments,
            &description,
            &icon,
        )
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

/// `FileGetShortcut(LinkFile, &OutTarget, &OutDir, &OutArgs, &OutDesc,
/// &OutIcon, &OutIconNum, &OutRun)`: read a shortcut back.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: all eight fields of a
/// shortcut `FileCreateShortcut` made come back -- target, working directory,
/// arguments, description, icon, icon number and run state. This tree reads
/// the `.desktop` file `FileCreateShortcut` wrote; the icon NUMBER and the run
/// state have no counterpart there, so those two answers are the empty text.
pub(crate) fn file_get_shortcut(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let shortcut = text_of(values, 0);
    let (target, dir, args, desc, icon) = rt
        .desktop()
        .read_shortcut(&shortcut)
        .map_err(|reason| desktop_failure(reason, line))?;
    let places: [(&str, Value); 7] = [
        ("target", Value::Text(target)),
        ("dir", Value::Text(dir)),
        ("args", Value::Text(args)),
        ("desc", Value::Text(desc)),
        ("icon", Value::Text(icon)),
        // A `.desktop` file has no icon index and no run state: the honest
        // answer is the empty text, not a number invented for the shape.
        ("iconnum", Value::Text(String::new())),
        ("run", Value::Text(String::new())),
    ];
    for (index, (name, value)) in places.iter().enumerate() {
        let at = index + 1;
        let Some(slot) = values.get(at) else {
            continue;
        };
        if !matches!(slot, Value::Reference(_)) {
            return Err(RuntimeError {
                message: format!(
                    "the {} of FileGetShortcut is written back, so parameter #{} wants a variable",
                    name, at + 1
                ),
                line,
            });
        }
        rt.write_reference(slot, value.clone());
    }
    Ok(Value::Text(String::new()))
}

// -- the pieces of a text ----------------------------------------------------------

/// `StrSplit`: the pieces of a text, cut at the delimiters.
///
/// v2 cuts the text where a delimiter is, and a delimiter is a literal: a
/// text is one delimiter and an array of texts is a set of them, and the
/// first one of them that is there wins. The characters to omit are cut
/// from the beginning and the end of every piece, and the number of the
/// pieces says how many times the text is cut: the rest of the text is
/// the last piece.
///
/// Measured on the binary of v2.0.19 on 2026-10-31.
pub(crate) fn str_split(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    let delimiters = the_delimiters(rt, values.get(1), line)?;
    let omit = text_of(values, 2);
    // The number of the pieces is not there: there is no limit.
    let max = match values.get(3) {
        None => -1.0,
        Some(value) => value.to_number().unwrap_or(-1.0),
    };
    let pieces = the_pieces(&text, &delimiters, &omit, max);
    let values: Vec<Value> = pieces.into_iter().map(Value::Text).collect();
    // The answer of v2 is an array.
    match crate::classes::construct("array", &values, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(values)),
    }
}

/// The delimiters of a `StrSplit`: the literals the text is cut at.
///
/// v2 takes a text or an array of texts, and refuses anything else with
/// "Parameter #2 of StrSplit is invalid.". An array that is empty is
/// refused as well: the delimiters that are blank are a text that is
/// empty, and not an array with nothing in it.
fn the_delimiters(
    rt: &Runtime<'_>,
    value: Option<&Value>,
    line: u32,
) -> Result<Vec<String>, RuntimeError> {
    let invalid = || RuntimeError {
        message: "Parameter #2 of StrSplit is invalid.".into(),
        line,
    };
    match value {
        None => Ok(Vec::new()),
        Some(Value::List(items)) => {
            if items.is_empty() {
                return Err(invalid());
            }
            Ok(items.iter().map(|item| item.to_text()).collect())
        }
        Some(value) => {
            // The array of the delimiters is a class of the runtime.
            if rt.class_of(value).as_deref() == Some("Array") {
                let items = crate::classes::array::the_items(value);
                if items.is_empty() {
                    return Err(invalid());
                }
                return items
                    .iter()
                    .map(|item| match item {
                        Value::Text(text) => Ok(text.clone()),
                        other => Err(RuntimeError {
                            message: crate::runtime::requires(
                                2,
                                "StrSplit",
                                TypeName::String,
                                other,
                            ),
                            line,
                        }),
                    })
                    .collect::<Result<Vec<String>, RuntimeError>>();
            }
            // The other objects are not delimiters.
            if matches!(value, Value::Object(_)) {
                return Err(invalid());
            }
            // A text that is empty is a blank delimiter: every character of
            // the text is a piece of its own.
            let text = value.to_text();
            if text.is_empty() {
                return Ok(Vec::new());
            }
            Ok(vec![text])
        }
    }
}

/// The pieces of a text, cut at the delimiters.
///
/// The text is cut where a delimiter is, and the delimiters are tried in
/// the order they were given: the first one of them that is there wins.
/// The characters to omit are cut from the beginning and the end of every
/// piece. When the delimiters are blank every character of the text is a
/// piece of its own, and the characters to omit are not pieces at all.
fn the_pieces(text: &str, delimiters: &[String], omit: &str, max: f64) -> Vec<String> {
    // The delimiters are blank: every character of the text is a piece.
    if delimiters.is_empty() {
        return text
            .chars()
            .filter(|character| !omit.contains(*character))
            .map(|character| character.to_string())
            .collect();
    }
    // The number of the pieces is zero: there is no piece at all.
    if max == 0.0 {
        return Vec::new();
    }
    // Measured on the binary of v2.0.19 on 2026-10-31: a text that is empty
    // has no piece at all, and the piece that is empty is not a piece.
    if text.is_empty() {
        return Vec::new();
    }
    // The number of the pieces says how many times the text is cut: the
    // rest of the text is the last piece.
    let cuts = if max > 0.0 { max as usize - 1 } else { 0 };
    let mut pieces: Vec<String> = Vec::new();
    let mut rest = text;
    let mut made = 0;
    while let Some((at, length)) = the_first_delimiter(rest, delimiters) {
        if max > 0.0 && made >= cuts {
            pieces.push(trim_the_omit(rest, omit));
            return pieces;
        }
        pieces.push(trim_the_omit(&rest[..at], omit));
        rest = &rest[at + length..];
        made += 1;
    }
    pieces.push(trim_the_omit(rest, omit));
    pieces
}

/// Where the text is cut: the place of the first delimiter that is there,
/// and how long it is.
///
/// The delimiters are tried in the order they were given, and the first
/// one of them that is there wins.
fn the_first_delimiter(text: &str, delimiters: &[String]) -> Option<(usize, usize)> {
    let mut found: Option<(usize, usize)> = None;
    for delimiter in delimiters {
        if delimiter.is_empty() {
            continue;
        }
        let Some(at) = text.find(delimiter.as_str()) else {
            continue;
        };
        match found {
            // The one that is there first wins, and the order of the
            // delimiters decides between the ones that are at the same
            // place: the first of them is the one that is kept.
            Some((first, _)) if first <= at => continue,
            _ => found = Some((at, delimiter.len())),
        }
    }
    found
}

/// The text without the characters to omit at its beginning and its end.
fn trim_the_omit(text: &str, omit: &str) -> String {
    if omit.is_empty() {
        return text.to_string();
    }
    let mut start = 0;
    while start < text.len() {
        let Some(character) = text[start..].chars().next() else {
            break;
        };
        if !omit.contains(character) {
            break;
        }
        start += character.len_utf8();
    }
    let mut end = text.len();
    while end > start {
        let Some(character) = text[..end].chars().next_back() else {
            break;
        };
        if !omit.contains(character) {
            break;
        }
        end -= character.len_utf8();
    }
    text[start..end].to_string()
}

/// `SplitPath`: the pieces of a path.
///
/// v2 writes the pieces into the variables of the caller, and it
/// answers with nothing. The pieces are the name of the file, the
/// directory, the extension, the name without the extension and the
/// drive.
pub(crate) fn split_path(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_argument("SplitPath", values, 0, line)?;
    let pieces = split_the_path(&path);
    for (index, piece) in pieces.into_iter().enumerate() {
        let Some(value) = values.get(index + 1) else {
            continue;
        };
        // The pieces are written into the variables of the caller, and
        // those are given by reference.
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 2,
                    "SplitPath",
                    "a variable reference",
                    value,
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
fn text_argument(
    name: &str,
    values: &[Value],
    index: usize,
    line: u32,
) -> Result<String, RuntimeError> {
    match values.get(index) {
        None => Ok(String::new()),
        // Measured on the binary of v2.0.19 on 2026-11-10: the absent value is
        // no argument, and the text of no argument is the empty text.
        Some(Value::Unset) => Ok(String::new()),
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

/// `ObjBindMethod`: the function that calls the method of an object.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the value is
/// of the class `BoundFunc`, and the call of it calls the method of the
/// object with the object as its `this`. The name of the method is not
/// checked when the value is made: the `ObjBindMethod(t, "Nope")` is made,
/// and the call of it fails.
///
/// The v1 makes the binding too, and it refuses the call of the name that
/// holds it: the v1 does not know that the name is a function.
pub(crate) fn obj_bind_method(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let object = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    if !matches!(object, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjBindMethod", TypeName::Object, &object),
            line,
        });
    }
    let method = text_of(values, 1);
    Ok(rt.bound_function(object, Value::Text(method)))
}

/// `HasProp`: whether an object has a property.
pub(crate) fn has_prop(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is
    // not an object has no property, and the answer is zero and not a
    // refusal. The one property that every value has is its class.
    if !matches!(target, Value::Object(_)) {
        return Ok(Value::Int(if name.eq_ignore_ascii_case("__class") {
            1
        } else {
            0
        }));
    }
    // A class is not an instance of itself: the question is about the
    // class, and the properties of the instances are not its own.
    if rt.class_name_of(&target).is_some() {
        return rt.base_object(&target, "hasprop", &[Value::Text(name)], line);
    }
    rt.own_method(&target, "hasprop", &[Value::Text(name)], line)
}

/// `HasMethod`: whether an object has a method.
pub(crate) fn has_method(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method, and the tree asks the method whether it
    // could take them.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    // Measured on the binary of v2.0.19 on 2026-10-31: the question with
    // no name asks whether the value is a class, and the name is not there
    // to be asked about.
    let mut arguments = Vec::new();
    if values.len() > 1 {
        arguments.push(Value::Text(name));
    }
    if let Some(count) = count {
        arguments.push(Value::Int(count as i64));
    }
    // A value that is not an object has no method, and the answer is zero.
    if !matches!(target, Value::Object(_)) {
        return Ok(Value::Int(0));
    }
    if rt.class_name_of(&target).is_some() {
        return rt.base_object(&target, "hasmethod", &arguments, line);
    }
    rt.own_method(&target, "hasmethod", &arguments, line)
}

/// `GetMethod`: the method of an object, as a value.
pub(crate) fn get_method(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    let mut arguments = vec![Value::Text(name)];
    if let Some(count) = count {
        arguments.push(Value::Int(count as i64));
    }
    rt.own_method(&target, "getmethod", &arguments, line)
}

/// `HasBase`: whether the base of an object is another object.
pub(crate) fn has_base(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let base = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    // Measured on the binary of v2.0.19 on 2026-10-31: the base is an
    // object, and the refusal of a base that is not one comes before the
    // question of the object itself.
    if !matches!(base, Value::Object(_)) {
        return Err(RuntimeError {
            message: format!(
                "Expected an object but got {}.",
                crate::runtime::the_kind_of(&base)
            ),
            line,
        });
    }
    // A value that is not an object has no base, and the answer is zero.
    if !matches!(target, Value::Object(_)) {
        return Ok(Value::Int(0));
    }
    rt.own_method(&target, "hasbase", &[base], line)
}

/// `IsSetRef`: whether a variable holds a value.
///
/// v2 takes the variable by reference and answers one when the variable
/// is set and zero when it is not. A variable that was never given a
/// value is not set, and the reference to it is still a reference.
pub(crate) fn is_set_ref(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "IsSetRef"),
            line,
        });
    };
    let Value::Reference(reference) = value else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(
                1,
                "IsSetRef",
                "a variable reference",
                value,
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
pub(crate) fn send_mode(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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

/// `KeyWait`: wait for a key to be up, or to go down.
///
/// The runtime does not keep the keys that are down: the box does, and
/// the runtime does not ask it.
pub(crate) fn key_wait(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "this runtime does not keep the keys that are down, so it cannot wait for one."
            .into(),
        line,
    })
}

// -- the timers ---------------------------------------------------------------------

/// `SetTimer`: a function of the script that runs between the statements.
///
/// The runtime is one thread, so the timer is not a thread of its own:
/// it runs between the statements of the script, and its budget is the
/// budget of the script.
pub(crate) fn set_timer(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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

// -- the sound ---------------------------------------------------------------------

/// `SoundBeep`: a tone of a frequency, for a number of milliseconds.
///
/// The frequency of v2 is between 37 and 32767, and it is 523 when the
/// script does not say. The duration is in milliseconds, and it is 150
/// when the script does not say.
pub(crate) fn sound_beep(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let frequency = match values.first() {
        Some(value) => value.to_number().unwrap_or(523.0),
        None => 523.0,
    };
    if !(37.0..=32767.0).contains(&frequency) {
        return Err(RuntimeError {
            message: "the frequency of a tone is between 37 and 32767.".into(),
            line,
        });
    }
    let duration = match values.get(1) {
        Some(value) => value.to_number().unwrap_or(150.0),
        None => 150.0,
    };
    let duration = if duration > 0.0 { duration as u64 } else { 150 };
    rt.desktop()
        .sound_beep(frequency, duration)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// `SoundPlay`: a sound file.
///
/// The other parameters of v2 -- the volume, the speed, the pitch, the
/// wait and the options -- are not carried, and the desktop of the box
/// plays the file as it is.
pub(crate) fn sound_play(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the volume, the speed and the pitch of a sound are not carried yet.".into(),
            line,
        });
    }
    let path = text_of(values, 0);
    if path.is_empty() {
        return Err(RuntimeError {
            message: "the sound file is missing.".into(),
            line,
        });
    }
    rt.desktop()
        .sound_play(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// `SoundGetVolume`: the volume of a device, in percent.
///
/// The component of v2 is a part of a device -- the master volume, the
/// microphone -- and the box has one volume per device.
pub(crate) fn sound_get_volume(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message:
                "the component of a sound is not carried yet: the box has one volume per device."
                    .into(),
            line,
        });
    }
    component_of(values, 0, line)?;
    let device = device_of(values, 1);
    let volume = rt
        .desktop()
        .sound_volume(&device)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Float(volume))
}

/// `SoundSetVolume`: the volume of a device, in percent.
///
/// The setting of v2 is a percentage between -100 and 100, and a sign
/// in front of it adjusts the volume instead of setting it.
pub(crate) fn sound_set_volume(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 3 {
        return Err(RuntimeError {
            message:
                "the component of a sound is not carried yet: the box has one volume per device."
                    .into(),
            line,
        });
    }
    let setting = text_of(values, 0);
    let (adjust, number) = match setting.strip_prefix('+') {
        Some(rest) => (true, rest.to_string()),
        None => match setting.strip_prefix('-') {
            Some(rest) => (true, format!("-{rest}")),
            None => (false, setting.clone()),
        },
    };
    let number: f64 = number.trim().parse().map_err(|_| RuntimeError {
        message: format!(
            "the volume of a device is a percentage between -100 and 100, and {setting:?} is not one."
        ),
        line,
    })?;
    if !(-100.0..=100.0).contains(&number) {
        return Err(RuntimeError {
            message: "the volume of a device is a percentage between -100 and 100.".into(),
            line,
        });
    }
    component_of(values, 1, line)?;
    let device = device_of(values, 2);
    if adjust {
        let current = rt
            .desktop()
            .sound_volume(&device)
            .map_err(|e| desktop_failure(e, line))?;
        let wanted = (current + number).clamp(0.0, 100.0);
        rt.desktop()
            .sound_set_volume(&device, wanted)
            .map_err(|e| desktop_failure(e, line))?;
    } else {
        rt.desktop()
            .sound_set_volume(&device, number)
            .map_err(|e| desktop_failure(e, line))?;
    }
    Ok(Value::Int(1))
}

/// `SoundGetMute`: whether a device is muted.
pub(crate) fn sound_get_mute(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message:
                "the component of a sound is not carried yet: the box has one volume per device."
                    .into(),
            line,
        });
    }
    component_of(values, 0, line)?;
    let device = device_of(values, 1);
    let muted = rt
        .desktop()
        .sound_muted(&device)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(muted as i64))
}

/// `SoundSetMute`: mute a device, or unmute it.
///
/// The setting of v2 is 1 to mute, 0 to unmute, and -1 to turn the
/// mute of the device around.
pub(crate) fn sound_set_mute(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 3 {
        return Err(RuntimeError {
            message:
                "the component of a sound is not carried yet: the box has one volume per device."
                    .into(),
            line,
        });
    }
    let setting = number_of(values, 0);
    if !(-1.0..=1.0).contains(&setting) {
        return Err(RuntimeError {
            message: "the mute of a device is 1, 0 or -1.".into(),
            line,
        });
    }
    component_of(values, 1, line)?;
    let device = device_of(values, 2);
    let muted = if setting < 0.0 {
        let current = rt
            .desktop()
            .sound_muted(&device)
            .map_err(|e| desktop_failure(e, line))?;
        !current
    } else {
        setting != 0.0
    };
    rt.desktop()
        .sound_set_muted(&device, muted)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// The component of the sound that the script asks about.
///
/// v2 names a part of a device -- the master volume, the microphone --
/// and the box has one volume per device: the master one. A component
/// that is not the master fails with the reason.
fn component_of(values: &[Value], index: usize, line: u32) -> Result<String, RuntimeError> {
    let component = match values.get(index) {
        Some(value) => value.to_text(),
        None => return Ok(String::new()),
    };
    if component.is_empty() || component.eq_ignore_ascii_case("master") {
        return Ok(String::new());
    }
    Err(RuntimeError {
        message: "the box has one volume per device: the master volume is the only one it has."
            .into(),
        line,
    })
}

/// The device that the script asks about.
///
/// v2 names a device by its name or by its number. The device of the
/// system is the empty name, and a number is written with a `#` in
/// front of it, so that the desktop can tell the two apart.
fn device_of(values: &[Value], index: usize) -> String {
    match values.get(index) {
        Some(Value::Int(number)) => format!("#{number}"),
        Some(value) => value.to_text(),
        None => String::new(),
    }
}

// -- the suspend and the pause ----------------------------------------------------

/// The number of `Suspend` and of `Pause`: on, off, or the toggle of the
/// omitted one.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the parameter is a Number,
/// and a text is the TypeError "Parameter #1 of Suspend requires a Number, but
/// received a String." with the text in the `Extra` -- "On", "Off" and
/// "Toggle" are not taken. The number one turns the state on, the zero turns
/// it off, and the minus one toggles, the way the omitted parameter toggles.
/// Any other number is "Parameter #1 of <name> is invalid." with the number in
/// the `Extra`: `Suspend 2` answers it.
fn the_on_or_off_of_a_command(
    values: &[Value],
    name: &str,
    line: u32,
) -> Result<Option<bool>, RuntimeError> {
    let Some(value) = values.first() else {
        return Ok(None);
    };
    match token_to_int64(value) {
        1 => Ok(Some(true)),
        0 => Ok(Some(false)),
        -1 => Ok(None),
        _ => Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        }),
    }
}

/// `Suspend`: the hotkeys of the script off, or on.
///
/// The answer is the empty text, and the command is observable through
/// `A_IsSuspended` and through the enabled flag of the hotkey table: the
/// runner has no keyboard, so no hotkey fires here, and a test reads the
/// table instead.
pub(crate) fn suspend(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match the_on_or_off_of_a_command(values, "Suspend", line)? {
        Some(suspended) => rt.set_suspended(suspended),
        None => {
            let now = rt.is_suspended();
            rt.set_suspended(!now);
        }
    }
    Ok(Value::Text(String::new()))
}

/// `Pause`: the script paused, or not.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the shapes of the
/// parameter are the ones of `Suspend`, and the answer is the empty text. A
/// `Pause` with no parameter pauses the *current thread* of the binary, which
/// waits for a key and hangs a script that has no keyboard -- the tree keeps a
/// state instead, and says so in `A_IsPaused`.
pub(crate) fn pause(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match the_on_or_off_of_a_command(values, "Pause", line)? {
        Some(paused) => rt.set_paused(paused),
        None => {
            let now = rt.is_paused();
            rt.set_paused(!now);
        }
    }
    Ok(Value::Text(String::new()))
}

// -- the script ---------------------------------------------------------------------

/// `Exit` and `ExitApp`: stop the script.
///
/// The code is kept and the script stops at the end of the statement
/// it was asked in, the way the statement of the same name stops it.
pub(crate) fn exit_script(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let code = number_of(values, 0) as i32;
    rt.exit_with(code);
    Ok(Value::Int(code as i64))
}

/// `OnExit`: the function the script asks to run when it ends.
///
/// The runtime is one script that ends when it ends, and the end is the
/// moment the runtime knows: the function runs then, whatever the reason
/// the script ended -- the end of the last statement, the code it asked to
/// stop with, or the error that stopped it.
///
/// The callback is a function object, and its prototype has to take the two
/// parameters of v2: the reason of the exit and the code.
pub(crate) fn on_exit(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let callback = rt.the_callback(values.first(), "OnExit", line)?;
    // The order of the callbacks: 1 calls it after the ones that are
    // registered, -1 before them, and 0 does not call it at all.
    match the_order(values.get(1), "OnExit", line)? {
        1 => rt.set_exit_callback(&callback.name, line),
        -1 => {
            rt.set_exit_callback(&callback.name, line);
            rt.order_exit_callback(&callback.name, true);
        }
        0 => rt.remove_exit_callback(&callback.name),
        _ => {
            return Err(RuntimeError {
                message: format!(
                    "{}\n     Specifically: {}",
                    crate::runtime::invalid_parameter(2, "OnExit"),
                    values
                        .get(1)
                        .map(|value| value.to_text())
                        .unwrap_or_default()
                ),
                line,
            });
        }
    }
    Ok(Value::Int(1))
}

/// `OnError`: the function the script asks to run when a failure is not
/// caught.
///
/// The callback is a function object, and its prototype has to take the two
/// parameters of v2: the value that was thrown and the mode of the
/// failure.
pub(crate) fn on_error(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let callback = rt.the_callback(values.first(), "OnError", line)?;
    match the_order(values.get(1), "OnError", line)? {
        1 => rt.set_error_callback(&callback.name, line),
        -1 => {
            rt.set_error_callback(&callback.name, line);
            rt.order_error_callback(&callback.name, true);
        }
        0 => rt.remove_error_callback(&callback.name),
        _ => {
            return Err(RuntimeError {
                message: format!(
                    "{}\n     Specifically: {}",
                    crate::runtime::invalid_parameter(2, "OnError"),
                    values
                        .get(1)
                        .map(|value| value.to_text())
                        .unwrap_or_default()
                ),
                line,
            });
        }
    }
    Ok(Value::Int(1))
}

/// The order of the callbacks: 1, -1 or 0.
///
/// The order is a number of v2, and it is cut to a whole number: 1.5 is
/// 1. The order that is not there is 1.
fn the_order(value: Option<&Value>, name: &str, line: u32) -> Result<i32, RuntimeError> {
    let Some(value) = value else { return Ok(1) };
    let number = value.to_number_strict().map_err(|_| RuntimeError {
        message: crate::runtime::wants_a_reference(2, name, "a Number", value),
        line,
    })?;
    Ok(number as i32)
}

/// What a parameter of a built-in wants.
///
/// The dialect has two ways of saying that a value is not the one it
/// wanted. A parameter that declares its kind says "Parameter #1 of Abs
/// requires a Number, but received a String.", and a check that the body
/// of the function makes says "Expected an Object but got a String.".
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Wants {
    /// A parameter that declares its kind: the dialect converts the value
    /// to it, and the message of the failure names it.
    Declared(TypeName),
    /// A check that the body of the function makes: the kind of the value
    /// is the kind itself.
    Checked(TypeName),
    /// A parameter that wants a reference. The wording is the one of the
    /// dialect: "a variable reference", "a VarRef".
    Reference(&'static str),
    /// A value of any kind.
    Value,
}

/// The type the parameters of a built-in declare.
///
/// The tables were measured on the binary of v2.0.19 on 2026-10-31:
/// every built-in of the tree was called with an object where the
/// parameter goes, and the refusal of the binary named the parameter and
/// the kind it wanted. A built-in that is not in them takes a value of
/// any kind, and the guard on a reference says so.
///
/// The number of the parameter is the one of the call: the first is the
/// first, and the two of `StrReplace` are the text and the needle.
/// The parameters that want a text.
const THE_TEXTS: &[(&str, &[u8])] = &[
    ("CoordMode", &[0, 1]),
    ("DateAdd", &[0, 2]),
    ("DateDiff", &[0, 1, 2]),
    ("DirCopy", &[0, 1]),
    ("DirCreate", &[0]),
    ("DirDelete", &[0]),
    ("DirExist", &[0]),
    ("DirMove", &[0, 1]),
    ("DirSelect", &[0, 2]),
    ("Download", &[0, 1]),

    ("EnvGet", &[0]),
    ("EnvSet", &[0, 1]),
    ("FileAppend", &[1, 2]),
    ("FileCopy", &[0, 1]),
    ("FileDelete", &[0]),
    ("FileEncoding", &[0]),
    ("FileExist", &[0]),
    ("FileGetAttrib", &[0]),
    ("FileGetSize", &[0, 1]),
    ("FileGetTime", &[0, 1]),
    ("FileInstall", &[0, 1]),
    ("FileMove", &[0, 1]),
    ("FileRead", &[0, 1]),
    ("FileSelect", &[0, 1, 2, 3]),
    ("FileSetAttrib", &[0, 1]),
    ("FileSetTime", &[0, 1, 2]),
    ("Format", &[0]),
    ("FormatTime", &[0, 1]),
    ("GetKeyName", &[0]),
    ("GetKeySC", &[0]),
    ("GetKeyState", &[0, 1]),
    ("GetKeyVK", &[0]),
    ("HotIfWinActive", &[0]),
    ("HotIfWinExist", &[0]),
    ("HotIfWinNotActive", &[0]),
    ("HotIfWinNotExist", &[0]),
    ("Hotkey", &[0, 2]),
    ("Hotstring", &[0, 2]),
    ("InStr", &[0, 1]),
    ("IniDelete", &[0, 1, 2]),
    ("IniRead", &[0, 1, 2, 3]),
    ("IniWrite", &[0, 1, 2, 3]),
    ("InputBox", &[0, 1, 2, 3]),
    ("IsAlnum", &[0, 1]),
    ("IsAlpha", &[0, 1]),
    ("IsDigit", &[0]),
    ("IsLabel", &[0]),
    ("IsLower", &[0, 1]),
    ("IsSpace", &[0]),
    ("IsTime", &[0]),
    ("IsUpper", &[0, 1]),
    ("IsXDigit", &[0]),
    ("KeyWait", &[0, 1]),
    ("LTrim", &[0, 1]),
    ("LoadPicture", &[0, 1]),
    ("MouseClick", &[0]),
    ("MouseClickDrag", &[0]),
    ("MouseMove", &[3]),
    ("MsgBox", &[0, 1, 2]),
    ("Ord", &[0]),
    ("RTrim", &[0, 1]),
    ("RegExMatch", &[0, 1]),
    ("RegExReplace", &[0, 1]),
    ("Run", &[0, 1, 2]),
    ("RunWait", &[0, 1, 2]),
    ("Send", &[0]),
    ("SendEvent", &[0]),
    ("SendInput", &[0]),
    ("SendMode", &[0]),
    ("SendPlay", &[0]),
    ("SendText", &[0]),
    ("SetKeyDelay", &[2]),
    ("SetMouseDelay", &[1]),
    ("SetWorkingDir", &[0]),
    ("Sort", &[0, 1]),
    ("SoundGetMute", &[0, 1]),
    ("SoundGetVolume", &[0, 1]),
    ("SoundPlay", &[0]),
    ("SoundSetMute", &[1, 2]),
    ("SoundSetVolume", &[1, 2]),
    ("SplitPath", &[0]),
    ("StrCompare", &[0, 1]),
    ("StrLen", &[0]),
    ("StrLower", &[0]),
    ("StrReplace", &[0, 1]),
    ("StrSplit", &[0, 2]),
    ("StrTitle", &[0]),
    ("StrUpper", &[0]),
    ("SubStr", &[0]),
    ("Trim", &[0, 1]),
    ("VerCompare", &[0, 1]),
    ("WinGetTitle", &[1, 2, 3]),
];

/// The parameters that want a number.
const THE_NUMBERS: &[(&str, &[u8])] = &[
    ("ACos", &[0]),
    ("ASin", &[0]),
    ("ATan", &[0]),
    ("Abs", &[0]),
    ("Ceil", &[0]),
    ("Chr", &[0]),

    ("ClipWait", &[0, 1]),
    ("Cos", &[0]),
    ("DateAdd", &[1]),
    ("DirSelect", &[1]),
    ("Exit", &[0]),
    ("ExitApp", &[0]),
    ("Exp", &[0]),
    ("FileCopy", &[2]),
    ("FileInstall", &[2]),
    ("FileMove", &[2]),
    ("Floor", &[0]),
    ("InStr", &[3, 4]),
    ("KeyHistory", &[0]),
    ("Ln", &[0]),
    ("Log", &[0]),
    ("MouseClick", &[1, 2, 3, 4]),
    ("MouseClickDrag", &[1, 2, 3, 4]),
    ("MouseMove", &[0, 1, 2]),
    ("PixelGetColor", &[0, 1]),
    ("Random", &[0, 1]),
    ("Round", &[0, 1]),
    ("SendLevel", &[0]),
    ("SetDefaultMouseSpeed", &[0]),
    ("SetKeyDelay", &[0, 1]),
    ("SetMouseDelay", &[0]),
    ("Sin", &[0]),
    ("Sleep", &[0]),
    ("SoundBeep", &[0, 1]),
    ("Sqrt", &[0]),
    ("StrSplit", &[3]),
    ("SubStr", &[1, 2]),
    ("Tan", &[0]),
];

/// The kinds of the parameters of the names the tree does not carry.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: every name that the
/// tree does not carry was called with a reference where the parameter
/// goes, and the refusal of the binary named the kind it wanted. The
/// dialect converts the arguments from the left, so a refusal about a
/// later parameter was read and the probe was asked again. A parameter
/// that wants a reference takes one, and it is not in this table: the
/// table of the references is the one of the binary.
const THE_REFUSED: &[(&str, usize, Wants)] = &[
    ("BlockInput", 0, Wants::Declared(TypeName::String)),
    ("CallbackFree", 0, Wants::Declared(TypeName::Number)),
    ("CaretGetPos", 0, Wants::Reference("a variable reference")),
    ("CaretGetPos", 1, Wants::Reference("a variable reference")),
    ("ComCall", 0, Wants::Declared(TypeName::Integer)),
    ("ComObjFlags", 0, Wants::Declared(TypeName::ComValue)),
    ("ComObjFromPtr", 0, Wants::Declared(TypeName::Number)),
    ("ComObjValue", 0, Wants::Declared(TypeName::ComValue)),
    ("ControlAddItem", 0, Wants::Declared(TypeName::String)),
    ("ControlAddItem", 3, Wants::Declared(TypeName::String)),
    ("ControlAddItem", 4, Wants::Declared(TypeName::String)),
    ("ControlAddItem", 5, Wants::Declared(TypeName::String)),
    ("ControlChooseIndex", 0, Wants::Declared(TypeName::Number)),
    ("ControlChooseIndex", 3, Wants::Declared(TypeName::String)),
    ("ControlChooseIndex", 4, Wants::Declared(TypeName::String)),
    ("ControlChooseIndex", 5, Wants::Declared(TypeName::String)),
    ("ControlChooseString", 0, Wants::Declared(TypeName::String)),
    ("ControlChooseString", 3, Wants::Declared(TypeName::String)),
    ("ControlChooseString", 4, Wants::Declared(TypeName::String)),
    ("ControlChooseString", 5, Wants::Declared(TypeName::String)),
    ("ControlClick", 2, Wants::Declared(TypeName::String)),
    ("ControlClick", 3, Wants::Declared(TypeName::String)),
    ("ControlClick", 4, Wants::Declared(TypeName::Number)),
    ("ControlClick", 5, Wants::Declared(TypeName::String)),
    ("ControlClick", 6, Wants::Declared(TypeName::String)),
    ("ControlClick", 7, Wants::Declared(TypeName::String)),
    ("ControlDeleteItem", 0, Wants::Declared(TypeName::Number)),
    ("ControlDeleteItem", 3, Wants::Declared(TypeName::String)),
    ("ControlDeleteItem", 4, Wants::Declared(TypeName::String)),
    ("ControlDeleteItem", 5, Wants::Declared(TypeName::String)),
    ("ControlFindItem", 0, Wants::Declared(TypeName::String)),
    ("ControlFindItem", 3, Wants::Declared(TypeName::String)),
    ("ControlFindItem", 4, Wants::Declared(TypeName::String)),
    ("ControlFindItem", 5, Wants::Declared(TypeName::String)),
    ("ControlFocus", 2, Wants::Declared(TypeName::String)),
    ("ControlFocus", 3, Wants::Declared(TypeName::String)),
    ("ControlFocus", 4, Wants::Declared(TypeName::String)),
    ("ControlGetChecked", 2, Wants::Declared(TypeName::String)),
    ("ControlGetChecked", 3, Wants::Declared(TypeName::String)),
    ("ControlGetChecked", 4, Wants::Declared(TypeName::String)),
    ("ControlGetChoice", 2, Wants::Declared(TypeName::String)),
    ("ControlGetChoice", 3, Wants::Declared(TypeName::String)),
    ("ControlGetChoice", 4, Wants::Declared(TypeName::String)),
    ("ControlGetClassNN", 2, Wants::Declared(TypeName::String)),
    ("ControlGetClassNN", 3, Wants::Declared(TypeName::String)),
    ("ControlGetClassNN", 4, Wants::Declared(TypeName::String)),
    ("ControlGetEnabled", 2, Wants::Declared(TypeName::String)),
    ("ControlGetEnabled", 3, Wants::Declared(TypeName::String)),
    ("ControlGetEnabled", 4, Wants::Declared(TypeName::String)),
    ("ControlGetExStyle", 2, Wants::Declared(TypeName::String)),
    ("ControlGetExStyle", 3, Wants::Declared(TypeName::String)),
    ("ControlGetExStyle", 4, Wants::Declared(TypeName::String)),
    ("ControlGetFocus", 1, Wants::Declared(TypeName::String)),
    ("ControlGetFocus", 2, Wants::Declared(TypeName::String)),
    ("ControlGetFocus", 3, Wants::Declared(TypeName::String)),
    ("ControlGetHwnd", 2, Wants::Declared(TypeName::String)),
    ("ControlGetHwnd", 3, Wants::Declared(TypeName::String)),
    ("ControlGetHwnd", 4, Wants::Declared(TypeName::String)),
    ("ControlGetIndex", 2, Wants::Declared(TypeName::String)),
    ("ControlGetIndex", 3, Wants::Declared(TypeName::String)),
    ("ControlGetIndex", 4, Wants::Declared(TypeName::String)),
    ("ControlGetItems", 2, Wants::Declared(TypeName::String)),
    ("ControlGetItems", 3, Wants::Declared(TypeName::String)),
    ("ControlGetItems", 4, Wants::Declared(TypeName::String)),
    ("ControlGetPos", 0, Wants::Reference("a VarRef")),
    ("ControlGetPos", 1, Wants::Reference("a VarRef")),
    ("ControlGetPos", 2, Wants::Reference("a VarRef")),
    ("ControlGetPos", 3, Wants::Reference("a VarRef")),
    ("ControlGetPos", 6, Wants::Declared(TypeName::String)),
    ("ControlGetPos", 7, Wants::Declared(TypeName::String)),
    ("ControlGetPos", 8, Wants::Declared(TypeName::String)),
    ("ControlGetStyle", 2, Wants::Declared(TypeName::String)),
    ("ControlGetStyle", 3, Wants::Declared(TypeName::String)),
    ("ControlGetStyle", 4, Wants::Declared(TypeName::String)),
    ("ControlGetText", 2, Wants::Declared(TypeName::String)),
    ("ControlGetText", 3, Wants::Declared(TypeName::String)),
    ("ControlGetText", 4, Wants::Declared(TypeName::String)),
    ("ControlGetVisible", 2, Wants::Declared(TypeName::String)),
    ("ControlGetVisible", 3, Wants::Declared(TypeName::String)),
    ("ControlGetVisible", 4, Wants::Declared(TypeName::String)),
    ("ControlHide", 2, Wants::Declared(TypeName::String)),
    ("ControlHide", 3, Wants::Declared(TypeName::String)),
    ("ControlHide", 4, Wants::Declared(TypeName::String)),
    ("ControlHideDropDown", 2, Wants::Declared(TypeName::String)),
    ("ControlHideDropDown", 3, Wants::Declared(TypeName::String)),
    ("ControlHideDropDown", 4, Wants::Declared(TypeName::String)),
    ("ControlMove", 0, Wants::Declared(TypeName::Number)),
    ("ControlMove", 1, Wants::Declared(TypeName::Number)),
    ("ControlMove", 2, Wants::Declared(TypeName::Number)),
    ("ControlMove", 3, Wants::Declared(TypeName::Number)),
    ("ControlMove", 6, Wants::Declared(TypeName::String)),
    ("ControlMove", 7, Wants::Declared(TypeName::String)),
    ("ControlMove", 8, Wants::Declared(TypeName::String)),
    ("ControlSend", 0, Wants::Declared(TypeName::String)),
    ("ControlSend", 3, Wants::Declared(TypeName::String)),
    ("ControlSend", 4, Wants::Declared(TypeName::String)),
    ("ControlSend", 5, Wants::Declared(TypeName::String)),
    ("ControlSendText", 0, Wants::Declared(TypeName::String)),
    ("ControlSendText", 3, Wants::Declared(TypeName::String)),
    ("ControlSendText", 4, Wants::Declared(TypeName::String)),
    ("ControlSendText", 5, Wants::Declared(TypeName::String)),
    ("ControlSetChecked", 0, Wants::Declared(TypeName::Number)),
    ("ControlSetChecked", 3, Wants::Declared(TypeName::String)),
    ("ControlSetChecked", 4, Wants::Declared(TypeName::String)),
    ("ControlSetChecked", 5, Wants::Declared(TypeName::String)),
    ("ControlSetEnabled", 0, Wants::Declared(TypeName::Number)),
    ("ControlSetEnabled", 3, Wants::Declared(TypeName::String)),
    ("ControlSetEnabled", 4, Wants::Declared(TypeName::String)),
    ("ControlSetEnabled", 5, Wants::Declared(TypeName::String)),
    ("ControlSetExStyle", 0, Wants::Declared(TypeName::String)),
    ("ControlSetExStyle", 3, Wants::Declared(TypeName::String)),
    ("ControlSetExStyle", 4, Wants::Declared(TypeName::String)),
    ("ControlSetExStyle", 5, Wants::Declared(TypeName::String)),
    ("ControlSetStyle", 0, Wants::Declared(TypeName::String)),
    ("ControlSetStyle", 3, Wants::Declared(TypeName::String)),
    ("ControlSetStyle", 4, Wants::Declared(TypeName::String)),
    ("ControlSetStyle", 5, Wants::Declared(TypeName::String)),
    ("ControlSetText", 0, Wants::Declared(TypeName::String)),
    ("ControlSetText", 3, Wants::Declared(TypeName::String)),
    ("ControlSetText", 4, Wants::Declared(TypeName::String)),
    ("ControlSetText", 5, Wants::Declared(TypeName::String)),
    ("ControlShow", 2, Wants::Declared(TypeName::String)),
    ("ControlShow", 3, Wants::Declared(TypeName::String)),
    ("ControlShow", 4, Wants::Declared(TypeName::String)),
    ("ControlShowDropDown", 2, Wants::Declared(TypeName::String)),
    ("ControlShowDropDown", 3, Wants::Declared(TypeName::String)),
    ("ControlShowDropDown", 4, Wants::Declared(TypeName::String)),
    ("Critical", 0, Wants::Declared(TypeName::String)),
    ("DriveEject", 0, Wants::Declared(TypeName::String)),
    ("DriveGetCapacity", 0, Wants::Declared(TypeName::String)),
    ("DriveGetFilesystem", 0, Wants::Declared(TypeName::String)),
    ("DriveGetLabel", 0, Wants::Declared(TypeName::String)),
    ("DriveGetList", 0, Wants::Declared(TypeName::String)),
    ("DriveGetSerial", 0, Wants::Declared(TypeName::String)),
    ("DriveGetSpaceFree", 0, Wants::Declared(TypeName::String)),
    ("DriveGetStatus", 0, Wants::Declared(TypeName::String)),
    ("DriveGetStatusCD", 0, Wants::Declared(TypeName::String)),
    ("DriveGetType", 0, Wants::Declared(TypeName::String)),
    ("DriveLock", 0, Wants::Declared(TypeName::String)),
    ("DriveRetract", 0, Wants::Declared(TypeName::String)),
    ("DriveSetLabel", 0, Wants::Declared(TypeName::String)),
    ("DriveSetLabel", 1, Wants::Declared(TypeName::String)),
    ("DriveUnlock", 0, Wants::Declared(TypeName::String)),
    ("EditGetCurrentCol", 2, Wants::Declared(TypeName::String)),
    ("EditGetCurrentCol", 3, Wants::Declared(TypeName::String)),
    ("EditGetCurrentCol", 4, Wants::Declared(TypeName::String)),
    ("EditGetCurrentLine", 2, Wants::Declared(TypeName::String)),
    ("EditGetCurrentLine", 3, Wants::Declared(TypeName::String)),
    ("EditGetCurrentLine", 4, Wants::Declared(TypeName::String)),
    ("EditGetLine", 0, Wants::Declared(TypeName::Number)),
    ("EditGetLine", 3, Wants::Declared(TypeName::String)),
    ("EditGetLine", 4, Wants::Declared(TypeName::String)),
    ("EditGetLine", 5, Wants::Declared(TypeName::String)),
    ("EditGetLineCount", 2, Wants::Declared(TypeName::String)),
    ("EditGetLineCount", 3, Wants::Declared(TypeName::String)),
    ("EditGetLineCount", 4, Wants::Declared(TypeName::String)),
    ("EditGetSelectedText", 2, Wants::Declared(TypeName::String)),
    ("EditGetSelectedText", 3, Wants::Declared(TypeName::String)),
    ("EditGetSelectedText", 4, Wants::Declared(TypeName::String)),
    ("EditPaste", 0, Wants::Declared(TypeName::String)),
    ("EditPaste", 3, Wants::Declared(TypeName::String)),
    ("EditPaste", 4, Wants::Declared(TypeName::String)),
    ("EditPaste", 5, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 0, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 1, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 2, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 3, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 4, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 5, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 6, Wants::Declared(TypeName::String)),
    ("FileCreateShortcut", 7, Wants::Declared(TypeName::Number)),
    ("FileCreateShortcut", 8, Wants::Declared(TypeName::Number)),
    ("FileGetShortcut", 0, Wants::Declared(TypeName::String)),
    ("FileGetShortcut", 1, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 2, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 3, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 4, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 5, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 6, Wants::Reference("a VarRef")),
    ("FileGetShortcut", 7, Wants::Reference("a VarRef")),
    ("FileGetVersion", 0, Wants::Declared(TypeName::String)),
    ("FileRecycle", 0, Wants::Declared(TypeName::String)),
    ("FileRecycleEmpty", 0, Wants::Declared(TypeName::String)),
    ("GroupActivate", 0, Wants::Declared(TypeName::String)),
    ("GroupActivate", 1, Wants::Declared(TypeName::String)),
    ("GroupAdd", 0, Wants::Declared(TypeName::String)),
    ("GroupAdd", 1, Wants::Declared(TypeName::String)),
    ("GroupAdd", 2, Wants::Declared(TypeName::String)),
    ("GroupAdd", 3, Wants::Declared(TypeName::String)),
    ("GroupAdd", 4, Wants::Declared(TypeName::String)),
    ("GroupClose", 0, Wants::Declared(TypeName::String)),
    ("GroupClose", 1, Wants::Declared(TypeName::String)),
    ("GroupDeactivate", 0, Wants::Declared(TypeName::String)),
    ("GroupDeactivate", 1, Wants::Declared(TypeName::String)),
    ("GuiCtrlFromHwnd", 0, Wants::Declared(TypeName::Number)),
    ("GuiFromHwnd", 0, Wants::Declared(TypeName::Number)),
    ("IL_Add", 0, Wants::Declared(TypeName::Number)),
    ("IL_Add", 1, Wants::Declared(TypeName::String)),
    ("IL_Add", 2, Wants::Declared(TypeName::Number)),
    ("IL_Create", 0, Wants::Declared(TypeName::Number)),
    ("IL_Create", 1, Wants::Declared(TypeName::Number)),
    ("IL_Destroy", 0, Wants::Declared(TypeName::Number)),
    ("ListLines", 0, Wants::Declared(TypeName::Number)),
    ("ListViewGetContent", 0, Wants::Declared(TypeName::String)),
    ("ListViewGetContent", 3, Wants::Declared(TypeName::String)),
    ("ListViewGetContent", 4, Wants::Declared(TypeName::String)),
    ("ListViewGetContent", 5, Wants::Declared(TypeName::String)),
    ("MenuFromHandle", 0, Wants::Declared(TypeName::Number)),
    ("MenuSelect", 1, Wants::Declared(TypeName::String)),
    ("MenuSelect", 2, Wants::Declared(TypeName::String)),
    ("MenuSelect", 3, Wants::Declared(TypeName::String)),
    ("MenuSelect", 4, Wants::Declared(TypeName::String)),
    ("MenuSelect", 5, Wants::Declared(TypeName::String)),
    ("MenuSelect", 6, Wants::Declared(TypeName::String)),
    ("MenuSelect", 7, Wants::Declared(TypeName::String)),
    ("MenuSelect", 8, Wants::Declared(TypeName::String)),
    ("MenuSelect", 9, Wants::Declared(TypeName::String)),
    ("MenuSelect", 10, Wants::Declared(TypeName::String)),
    ("MonitorGet", 0, Wants::Declared(TypeName::Number)),
    // The four that follow are values of the kind VarRef in the table of the
    // binary, and the kind is named by the match at the end of this function:
    // a `Wants::Reference` here would say the same words but would put the
    // empty text in the `What`, and the binary puts the name of the built-in
    // there (measured on 2026-11-09).
    ("MonitorGetName", 0, Wants::Declared(TypeName::Number)),
    ("MonitorGetWorkArea", 0, Wants::Declared(TypeName::Number)),
    ("OnMessage", 0, Wants::Declared(TypeName::Number)),
    ("OutputDebug", 0, Wants::Declared(TypeName::String)),
    ("Pause", 0, Wants::Declared(TypeName::Number)),
    ("PostMessage", 0, Wants::Declared(TypeName::Number)),
    ("PostMessage", 5, Wants::Declared(TypeName::String)),
    ("PostMessage", 6, Wants::Declared(TypeName::String)),
    ("PostMessage", 7, Wants::Declared(TypeName::String)),
    ("ProcessClose", 0, Wants::Declared(TypeName::String)),
    ("ProcessExist", 0, Wants::Declared(TypeName::String)),
    ("ProcessGetName", 0, Wants::Declared(TypeName::String)),
    ("ProcessGetParent", 0, Wants::Declared(TypeName::String)),
    ("ProcessGetPath", 0, Wants::Declared(TypeName::String)),
    ("ProcessSetPriority", 0, Wants::Declared(TypeName::String)),
    ("ProcessSetPriority", 1, Wants::Declared(TypeName::String)),
    ("ProcessWait", 0, Wants::Declared(TypeName::String)),
    ("ProcessWait", 1, Wants::Declared(TypeName::Number)),
    ("ProcessWaitClose", 0, Wants::Declared(TypeName::String)),
    ("ProcessWaitClose", 1, Wants::Declared(TypeName::Number)),
    ("RunAs", 0, Wants::Declared(TypeName::String)),
    ("RunAs", 1, Wants::Declared(TypeName::String)),
    ("RunAs", 2, Wants::Declared(TypeName::String)),
    ("SendMessage", 0, Wants::Declared(TypeName::Number)),
    ("SendMessage", 5, Wants::Declared(TypeName::String)),
    ("SendMessage", 6, Wants::Declared(TypeName::String)),
    ("SendMessage", 7, Wants::Declared(TypeName::String)),
    ("SendMessage", 8, Wants::Declared(TypeName::Number)),
    ("SetCapsLockState", 0, Wants::Declared(TypeName::String)),
    ("SetNumLockState", 0, Wants::Declared(TypeName::String)),
    ("SetScrollLockState", 0, Wants::Declared(TypeName::String)),
    ("Shutdown", 0, Wants::Declared(TypeName::Number)),
    ("SoundGetName", 0, Wants::Declared(TypeName::String)),
    ("SoundGetName", 1, Wants::Declared(TypeName::String)),
    ("StatusBarGetText", 0, Wants::Declared(TypeName::Number)),
    ("StatusBarGetText", 2, Wants::Declared(TypeName::String)),
    ("StatusBarGetText", 3, Wants::Declared(TypeName::String)),
    ("StatusBarGetText", 4, Wants::Declared(TypeName::String)),
    ("StatusBarWait", 0, Wants::Declared(TypeName::String)),
    ("StatusBarWait", 1, Wants::Declared(TypeName::Number)),
    ("StatusBarWait", 2, Wants::Declared(TypeName::Number)),
    ("StatusBarWait", 4, Wants::Declared(TypeName::String)),
    ("StatusBarWait", 5, Wants::Declared(TypeName::Number)),
    ("StatusBarWait", 6, Wants::Declared(TypeName::String)),
    ("StatusBarWait", 7, Wants::Declared(TypeName::String)),
    ("StrPtr", 0, Wants::Checked(TypeName::String)),
    ("StrPut", 0, Wants::Declared(TypeName::String)),
    ("Suspend", 0, Wants::Declared(TypeName::Number)),
    ("SysGet", 0, Wants::Declared(TypeName::Number)),
    ("Thread", 0, Wants::Declared(TypeName::String)),
    ("Thread", 1, Wants::Declared(TypeName::Number)),
    ("Thread", 2, Wants::Declared(TypeName::Number)),
    ("ToolTip", 0, Wants::Declared(TypeName::String)),
    ("ToolTip", 1, Wants::Declared(TypeName::Number)),
    ("ToolTip", 2, Wants::Declared(TypeName::Number)),
    ("ToolTip", 3, Wants::Declared(TypeName::Number)),
    ("TraySetIcon", 0, Wants::Declared(TypeName::String)),
    ("TraySetIcon", 1, Wants::Declared(TypeName::Number)),
    ("TrayTip", 0, Wants::Declared(TypeName::String)),
    ("TrayTip", 1, Wants::Declared(TypeName::String)),
    ("TrayTip", 2, Wants::Declared(TypeName::String)),
    ("WinActivate", 1, Wants::Declared(TypeName::String)),
    ("WinActivate", 2, Wants::Declared(TypeName::String)),
    ("WinActivate", 3, Wants::Declared(TypeName::String)),
    ("WinActivateBottom", 1, Wants::Declared(TypeName::String)),
    ("WinActivateBottom", 2, Wants::Declared(TypeName::String)),
    ("WinActivateBottom", 3, Wants::Declared(TypeName::String)),
    ("WinClose", 1, Wants::Declared(TypeName::String)),
    ("WinClose", 2, Wants::Declared(TypeName::Number)),
    ("WinClose", 3, Wants::Declared(TypeName::String)),
    ("WinClose", 4, Wants::Declared(TypeName::String)),
    ("WinGetClass", 1, Wants::Declared(TypeName::String)),
    ("WinGetClass", 2, Wants::Declared(TypeName::String)),
    ("WinGetClass", 3, Wants::Declared(TypeName::String)),
    ("WinGetClientPos", 0, Wants::Reference("a VarRef")),
    ("WinGetClientPos", 1, Wants::Reference("a VarRef")),
    ("WinGetClientPos", 2, Wants::Reference("a VarRef")),
    ("WinGetClientPos", 3, Wants::Reference("a VarRef")),
    ("WinGetClientPos", 5, Wants::Declared(TypeName::String)),
    ("WinGetClientPos", 6, Wants::Declared(TypeName::String)),
    ("WinGetClientPos", 7, Wants::Declared(TypeName::String)),
    ("WinGetControls", 1, Wants::Declared(TypeName::String)),
    ("WinGetControls", 2, Wants::Declared(TypeName::String)),
    ("WinGetControls", 3, Wants::Declared(TypeName::String)),
    ("WinGetControlsHwnd", 1, Wants::Declared(TypeName::String)),
    ("WinGetControlsHwnd", 2, Wants::Declared(TypeName::String)),
    ("WinGetControlsHwnd", 3, Wants::Declared(TypeName::String)),
    ("WinGetCount", 1, Wants::Declared(TypeName::String)),
    ("WinGetCount", 2, Wants::Declared(TypeName::String)),
    ("WinGetCount", 3, Wants::Declared(TypeName::String)),
    ("WinGetExStyle", 1, Wants::Declared(TypeName::String)),
    ("WinGetExStyle", 2, Wants::Declared(TypeName::String)),
    ("WinGetExStyle", 3, Wants::Declared(TypeName::String)),
    ("WinGetID", 1, Wants::Declared(TypeName::String)),
    ("WinGetID", 2, Wants::Declared(TypeName::String)),
    ("WinGetID", 3, Wants::Declared(TypeName::String)),
    ("WinGetIDLast", 1, Wants::Declared(TypeName::String)),
    ("WinGetIDLast", 2, Wants::Declared(TypeName::String)),
    ("WinGetIDLast", 3, Wants::Declared(TypeName::String)),
    ("WinGetList", 1, Wants::Declared(TypeName::String)),
    ("WinGetList", 2, Wants::Declared(TypeName::String)),
    ("WinGetList", 3, Wants::Declared(TypeName::String)),
    ("WinGetMinMax", 1, Wants::Declared(TypeName::String)),
    ("WinGetMinMax", 2, Wants::Declared(TypeName::String)),
    ("WinGetMinMax", 3, Wants::Declared(TypeName::String)),
    ("WinGetPID", 1, Wants::Declared(TypeName::String)),
    ("WinGetPID", 2, Wants::Declared(TypeName::String)),
    ("WinGetPID", 3, Wants::Declared(TypeName::String)),
    ("WinGetPos", 0, Wants::Reference("a VarRef")),
    ("WinGetPos", 1, Wants::Reference("a VarRef")),
    ("WinGetPos", 2, Wants::Reference("a VarRef")),
    ("WinGetPos", 3, Wants::Reference("a VarRef")),
    ("WinGetPos", 5, Wants::Declared(TypeName::String)),
    ("WinGetPos", 6, Wants::Declared(TypeName::String)),
    ("WinGetPos", 7, Wants::Declared(TypeName::String)),
    ("WinGetProcessName", 1, Wants::Declared(TypeName::String)),
    ("WinGetProcessName", 2, Wants::Declared(TypeName::String)),
    ("WinGetProcessName", 3, Wants::Declared(TypeName::String)),
    ("WinGetProcessPath", 1, Wants::Declared(TypeName::String)),
    ("WinGetProcessPath", 2, Wants::Declared(TypeName::String)),
    ("WinGetProcessPath", 3, Wants::Declared(TypeName::String)),
    ("WinGetStyle", 1, Wants::Declared(TypeName::String)),
    ("WinGetStyle", 2, Wants::Declared(TypeName::String)),
    ("WinGetStyle", 3, Wants::Declared(TypeName::String)),
    ("WinGetText", 1, Wants::Declared(TypeName::String)),
    ("WinGetText", 2, Wants::Declared(TypeName::String)),
    ("WinGetText", 3, Wants::Declared(TypeName::String)),
    ("WinGetTransColor", 1, Wants::Declared(TypeName::String)),
    ("WinGetTransColor", 2, Wants::Declared(TypeName::String)),
    ("WinGetTransColor", 3, Wants::Declared(TypeName::String)),
    ("WinGetTransparent", 1, Wants::Declared(TypeName::String)),
    ("WinGetTransparent", 2, Wants::Declared(TypeName::String)),
    ("WinGetTransparent", 3, Wants::Declared(TypeName::String)),
    ("WinHide", 1, Wants::Declared(TypeName::String)),
    ("WinHide", 2, Wants::Declared(TypeName::String)),
    ("WinHide", 3, Wants::Declared(TypeName::String)),
    ("WinKill", 1, Wants::Declared(TypeName::String)),
    ("WinKill", 2, Wants::Declared(TypeName::Number)),
    ("WinKill", 3, Wants::Declared(TypeName::String)),
    ("WinKill", 4, Wants::Declared(TypeName::String)),
    ("WinMaximize", 1, Wants::Declared(TypeName::String)),
    ("WinMaximize", 2, Wants::Declared(TypeName::String)),
    ("WinMaximize", 3, Wants::Declared(TypeName::String)),
    ("WinMinimize", 1, Wants::Declared(TypeName::String)),
    ("WinMinimize", 2, Wants::Declared(TypeName::String)),
    ("WinMinimize", 3, Wants::Declared(TypeName::String)),
    ("WinMove", 0, Wants::Declared(TypeName::Number)),
    ("WinMove", 1, Wants::Declared(TypeName::Number)),
    ("WinMove", 2, Wants::Declared(TypeName::Number)),
    ("WinMove", 3, Wants::Declared(TypeName::Number)),
    ("WinMove", 5, Wants::Declared(TypeName::String)),
    ("WinMove", 6, Wants::Declared(TypeName::String)),
    ("WinMove", 7, Wants::Declared(TypeName::String)),
    ("WinMoveBottom", 1, Wants::Declared(TypeName::String)),
    ("WinMoveBottom", 2, Wants::Declared(TypeName::String)),
    ("WinMoveBottom", 3, Wants::Declared(TypeName::String)),
    ("WinMoveTop", 1, Wants::Declared(TypeName::String)),
    ("WinMoveTop", 2, Wants::Declared(TypeName::String)),
    ("WinMoveTop", 3, Wants::Declared(TypeName::String)),
    ("WinRedraw", 1, Wants::Declared(TypeName::String)),
    ("WinRedraw", 2, Wants::Declared(TypeName::String)),
    ("WinRedraw", 3, Wants::Declared(TypeName::String)),
    ("WinRestore", 1, Wants::Declared(TypeName::String)),
    ("WinRestore", 2, Wants::Declared(TypeName::String)),
    ("WinRestore", 3, Wants::Declared(TypeName::String)),
    ("WinSetAlwaysOnTop", 0, Wants::Declared(TypeName::Number)),
    ("WinSetAlwaysOnTop", 2, Wants::Declared(TypeName::String)),
    ("WinSetAlwaysOnTop", 3, Wants::Declared(TypeName::String)),
    ("WinSetAlwaysOnTop", 4, Wants::Declared(TypeName::String)),
    ("WinSetEnabled", 0, Wants::Declared(TypeName::Number)),
    ("WinSetEnabled", 2, Wants::Declared(TypeName::String)),
    ("WinSetEnabled", 3, Wants::Declared(TypeName::String)),
    ("WinSetEnabled", 4, Wants::Declared(TypeName::String)),
    ("WinSetExStyle", 0, Wants::Declared(TypeName::String)),
    ("WinSetExStyle", 2, Wants::Declared(TypeName::String)),
    ("WinSetExStyle", 3, Wants::Declared(TypeName::String)),
    ("WinSetExStyle", 4, Wants::Declared(TypeName::String)),
    ("WinSetRegion", 0, Wants::Declared(TypeName::String)),
    ("WinSetRegion", 2, Wants::Declared(TypeName::String)),
    ("WinSetRegion", 3, Wants::Declared(TypeName::String)),
    ("WinSetRegion", 4, Wants::Declared(TypeName::String)),
    ("WinSetStyle", 0, Wants::Declared(TypeName::String)),
    ("WinSetStyle", 2, Wants::Declared(TypeName::String)),
    ("WinSetStyle", 3, Wants::Declared(TypeName::String)),
    ("WinSetStyle", 4, Wants::Declared(TypeName::String)),
    ("WinSetTitle", 0, Wants::Declared(TypeName::String)),
    ("WinSetTitle", 2, Wants::Declared(TypeName::String)),
    ("WinSetTitle", 3, Wants::Declared(TypeName::String)),
    ("WinSetTitle", 4, Wants::Declared(TypeName::String)),
    ("WinSetTransColor", 2, Wants::Declared(TypeName::String)),
    ("WinSetTransColor", 3, Wants::Declared(TypeName::String)),
    ("WinSetTransColor", 4, Wants::Declared(TypeName::String)),
    ("WinSetTransparent", 0, Wants::Declared(TypeName::String)),
    ("WinSetTransparent", 2, Wants::Declared(TypeName::String)),
    ("WinSetTransparent", 3, Wants::Declared(TypeName::String)),
    ("WinSetTransparent", 4, Wants::Declared(TypeName::String)),
    ("WinShow", 1, Wants::Declared(TypeName::String)),
    ("WinShow", 2, Wants::Declared(TypeName::String)),
    ("WinShow", 3, Wants::Declared(TypeName::String)),
    ("WinWait", 1, Wants::Declared(TypeName::String)),
    ("WinWait", 2, Wants::Declared(TypeName::Number)),
    ("WinWait", 3, Wants::Declared(TypeName::String)),
    ("WinWait", 4, Wants::Declared(TypeName::String)),
    ("WinWaitActive", 1, Wants::Declared(TypeName::String)),
    ("WinWaitActive", 2, Wants::Declared(TypeName::Number)),
    ("WinWaitActive", 3, Wants::Declared(TypeName::String)),
    ("WinWaitActive", 4, Wants::Declared(TypeName::String)),
    ("WinWaitClose", 1, Wants::Declared(TypeName::String)),
    ("WinWaitClose", 2, Wants::Declared(TypeName::Number)),
    ("WinWaitClose", 3, Wants::Declared(TypeName::String)),
    ("WinWaitClose", 4, Wants::Declared(TypeName::String)),
    ("WinWaitNotActive", 1, Wants::Declared(TypeName::String)),
    ("WinWaitNotActive", 2, Wants::Declared(TypeName::Number)),
    ("WinWaitNotActive", 3, Wants::Declared(TypeName::String)),
    ("WinWaitNotActive", 4, Wants::Declared(TypeName::String)),
    // The parameters that the caller passes a variable reference to.
    // Measured on the binary of v2.0.19 on 2026-11-09: a value that is not
    // a reference is refused with "requires a variable reference, but
    // received a ...", and the failure carries no name of the built-in in
    // its `What` -- the caller's check makes it, and not the built-in.
    ("RegExMatch", 2, Wants::Reference("a variable reference")),
    ("RegExReplace", 3, Wants::Reference("a variable reference")),
    ("VarSetStrCapacity", 0, Wants::Reference("a variable reference")),
];

pub fn wants(name: &str, index: usize) -> Wants {
    let place = index as u8;
    if THE_TEXTS
        .iter()
        .any(|(built_in, indexes)| *built_in == name && indexes.contains(&place))
    {
        return Wants::Declared(TypeName::String);
    }
    if THE_NUMBERS
        .iter()
        .any(|(built_in, indexes)| *built_in == name && indexes.contains(&place))
    {
        return Wants::Declared(TypeName::Number);
    }
    if let Some((_, _, kind)) = THE_REFUSED
        .iter()
        .find(|(built_in, place, _)| *built_in == name && *place == index)
    {
        return *kind;
    }
    match (name, place) {
        // Every parameter of the two is a number, and there is no telling
        // how many of them there are.
        ("Min" | "Max", _) => Wants::Declared(TypeName::Number),
        ("ObjOwnProps" | "ObjOwnPropCount" | "ObjHasOwnProp" | "ObjSetBase", 0) => {
            Wants::Checked(TypeName::Object)
        }
        // Measured on the binary of v2.0.19 on 2026-11-09: the last four
        // parameters of the two monitor commands are values of the kind
        // `VarRef`, and v2 names the kind in the failure of another one with
        // "requires a VarRef, but received ..." -- the wording of a declared
        // kind, and the name of the built-in in the `What`.
        ("MonitorGet" | "MonitorGetWorkArea", 1..=4) => Wants::Declared(TypeName::VarRef),
        _ => Wants::Value,
    }
}

/// The built-ins that refuse the parameter themselves.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the `Mod` says
/// "Invalid parameter(s)." for anything that is not a number -- a text,
/// an object and a reference alike -- and it does not name the type that
/// was wrong. The tree says it in its handler, and the guard above lets
/// it speak.
pub fn refuses_itself(name: &str) -> bool {
    matches!(name, "Mod")
}

/// The handlers of the object model.
///
/// The names of v2 reach the object model from here: the base of an
/// object, the properties it owns, and the room it has for them.
pub(crate) fn obj_get_base(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    rt.base_of(&target).ok_or_else(|| RuntimeError {
        message: crate::runtime::expected_type(TypeName::Object, target.type_name()),
        line,
    })
}

pub(crate) fn obj_set_base(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let base = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    if !matches!(base, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Object, base.type_name()),
            line,
        });
    }
    // The answer is the base the object had, and an object of a script
    // has none until the script gives it one.
    let old = rt
        .stored_base_of(&target)
        .unwrap_or(Value::Text(String::new()));
    rt.set_base_of(&target, base, line)?;
    Ok(old)
}

pub(crate) fn obj_own_props(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    rt.own_method(&target, "ownprops", &[], line)
}

pub(crate) fn obj_own_prop_count(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let names = rt.own_method(&target, "ownprops", &[], line)?;
    // The own properties of v2 are an enumerator, and the count of them is
    // the number of the turns it has left.
    Ok(Value::Int(crate::classes::enumerator::the_count(&names)))
}

pub(crate) fn obj_has_own_prop(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    let name = values.get(1).cloned().unwrap_or(Value::Text(String::new()));
    rt.own_method(&target, "hasownprop", &[name], line)
}

pub(crate) fn obj_get_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
    if !matches!(target, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::expected_type(TypeName::Object, target.type_name()),
            line,
        });
    }
    // The room an object has is the capacity of its properties, and an
    // object without any has none.
    Ok(rt
        .property_of(&target, "capacity", line)
        .unwrap_or(Value::Int(0)))
}

pub(crate) fn obj_set_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
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
pub(crate) fn obj_add_ref(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: crate::runtime::invalid_parameter(1, "ObjAddRef"),
        line,
    })
}

// -- the helpers ----------------------------------------------------------------

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values
        .get(index)
        .map(|value| value.to_text())
        .unwrap_or_default()
}

/// The number of the argument at this place, or zero.
fn number_of(values: &[Value], index: usize) -> f64 {
    values
        .get(index)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0)
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

/// Whether a value is a number, the way a parameter that wants one is
/// checked.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a value is a number when
/// it is one, and a text is one when it holds a number and nothing else. The
/// spaces and tabs at the two ends are allowed, the `0x` of a hex number is
/// allowed, a fraction and an exponent are allowed, and a letter after the
/// number is not.
pub(crate) fn is_numeric_value(value: &Value) -> bool {
    match value {
        Value::Int(_) | Value::Float(_) => true,
        Value::Text(text) => is_numeric_text(text),
        _ => false,
    }
}

/// Whether the text holds a number and nothing else.
///
/// The rule is the one of `IsNumeric` of the binary: the leading spaces and
/// tabs are skipped, the sign is allowed, a `0x` prefix makes it hex, a
/// fraction and an exponent are allowed for a decimal number, and only spaces
/// and tabs may follow it.
fn is_numeric_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() && (bytes[at] == b' ' || bytes[at] == b'\t') {
        at += 1;
    }
    if at >= bytes.len() {
        return false;
    }
    if bytes[at] == b'-' || bytes[at] == b'+' {
        at += 1;
    }
    let hex = is_hex_at(bytes, at);
    if hex {
        at += 2;
    }
    let mut point = false;
    let mut exponent = false;
    let mut digit = false;
    while at < bytes.len() {
        let character = bytes[at];
        if character == b' ' || character == b'\t' {
            // The number ends here when nothing but spaces or tabs follows.
            let mut next = at;
            while next < bytes.len() && (bytes[next] == b' ' || bytes[next] == b'\t') {
                next += 1;
            }
            if next < bytes.len() {
                return false;
            }
            break;
        }
        if character == b'.' {
            if point || hex {
                return false;
            }
            point = true;
            at += 1;
            continue;
        }
        let is_digit = if hex {
            character.is_ascii_hexdigit()
        } else {
            character.is_ascii_digit()
        };
        if !is_digit {
            // Only a decimal number may carry an exponent, and the exponent
            // has to hold at least one digit.
            if hex || (character != b'e' && character != b'E') || !digit || exponent {
                return false;
            }
            let mut next = at + 1;
            if next < bytes.len() && (bytes[next] == b'+' || bytes[next] == b'-') {
                next += 1;
            }
            if next >= bytes.len() || !bytes[next].is_ascii_digit() {
                return false;
            }
            exponent = true;
            point = true;
            at = next + 1;
            continue;
        }
        digit = true;
        at += 1;
    }
    digit
}

/// Whether a `0x` prefix of a hex number starts at this byte.
fn is_hex_at(bytes: &[u8], at: usize) -> bool {
    at + 2 < bytes.len()
        && bytes[at] == b'0'
        && (bytes[at + 1] == b'x' || bytes[at + 1] == b'X')
        && bytes[at + 2].is_ascii_hexdigit()
}

/// The value of one hex digit.
fn hex_digit(character: u8) -> u64 {
    match character {
        b'0'..=b'9' => (character - b'0') as u64,
        b'a'..=b'f' => (character - b'a') as u64 + 10,
        _ => (character - b'A') as u64 + 10,
    }
}

/// The whole number a value holds, the way the conversion of the binary
/// reads it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a float is cut toward
/// zero, and a number that a 64-bit integer cannot hold answers the smallest
/// one -- the cast of the machine, and not a saturation. A text is read by
/// [`text_to_int64`].
pub(crate) fn token_to_int64(value: &Value) -> i64 {
    match value {
        Value::Int(number) => *number,
        Value::Float(number) => double_to_int64(*number),
        Value::Text(text) => text_to_int64(text),
        _ => 0,
    }
}

/// The whole number of a float, the way the cast of the binary reads it.
///
/// The cast of the machine answers the smallest 64-bit integer for a number
/// that is out of its range, and for a number that is not one at all.
fn double_to_int64(number: f64) -> i64 {
    if !(-9223372036854775808.0..9223372036854775808.0).contains(&number) {
        return i64::MIN;
    }
    number as i64
}

/// The number a value holds, the way the conversion of the binary reads it.
pub(crate) fn token_to_double(value: &Value) -> f64 {
    match value {
        Value::Int(number) => *number as f64,
        Value::Float(number) => *number,
        Value::Text(text) => text_to_double(text),
        _ => 0.0,
    }
}

/// The whole number at the front of a text.
///
/// The rule is the one of `nstrtoi64` of the binary, measured on 2026-11-09:
/// a decimal number with a fraction and an exponent, or a hex number with the
/// `0x` prefix; the leading spaces and tabs are skipped; a leading zero does
/// not make it octal; and the overflow wraps the way the arithmetic of a
/// script wraps. `"1e3"` is 1000, `"1.9"` is 1, `"010"` is 10 and `"0x1F"`
/// is 31.
pub(crate) fn text_to_int64(text: &str) -> i64 {
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() && (bytes[at] == b' ' || bytes[at] == b'\t') {
        at += 1;
    }
    let negative = at < bytes.len() && bytes[at] == b'-';
    if negative || (at < bytes.len() && bytes[at] == b'+') {
        at += 1;
    }
    let mut value: u64;
    if is_hex_at(bytes, at) {
        at += 2;
        value = 0;
        while at < bytes.len() && bytes[at].is_ascii_hexdigit() {
            value = value.wrapping_mul(16).wrapping_add(hex_digit(bytes[at]));
            at += 1;
        }
    } else {
        value = 0;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            value = value
                .wrapping_mul(10)
                .wrapping_add((bytes[at] - b'0') as u64);
            at += 1;
        }
        let end_integer = at;
        if at < bytes.len() && bytes[at] == b'.' {
            at += 1;
            while at < bytes.len() && bytes[at].is_ascii_digit() {
                at += 1;
            }
        }
        let end_fraction = at;
        if at < bytes.len() && (bytes[at] == b'e' || bytes[at] == b'E') {
            at += 1;
            let exponent_negative = at < bytes.len() && bytes[at] == b'-';
            if exponent_negative || (at < bytes.len() && bytes[at] == b'+') {
                at += 1;
            }
            let digits_start = at;
            let mut exponent: u64 = 0;
            while at < bytes.len() && bytes[at].is_ascii_digit() {
                exponent = exponent
                    .wrapping_mul(10)
                    .wrapping_add((bytes[at] - b'0') as u64);
                at += 1;
            }
            if at > digits_start {
                // The digits of the fraction that the exponent carries into
                // the whole number are applied first.
                if !exponent_negative && end_integer < end_fraction && bytes[end_integer] == b'.' {
                    let mut next = end_integer + 1;
                    while next < end_fraction && exponent > 0 {
                        value = value
                            .wrapping_mul(10)
                            .wrapping_add((bytes[next] - b'0') as u64);
                        next += 1;
                        exponent -= 1;
                    }
                }
                if exponent >= if exponent_negative { 20 } else { 64 } {
                    value = 0;
                } else if exponent != 0 {
                    let mut multiplier: u64 = 1;
                    let mut base: u64 = 10;
                    let mut left = exponent;
                    while left > 1 {
                        if left & 1 == 1 {
                            multiplier = multiplier.wrapping_mul(base);
                        }
                        left >>= 1;
                        base = base.wrapping_mul(base);
                    }
                    multiplier = multiplier.wrapping_mul(base);
                    if exponent_negative {
                        value /= multiplier;
                    } else {
                        value = value.wrapping_mul(multiplier);
                    }
                }
            }
        }
    }
    if negative {
        -(value as i64)
    } else {
        value as i64
    }
}

/// The number at the front of a text.
///
/// The rule is the one of `ATOF` of the binary, measured on 2026-11-09: a hex
/// number is read as a whole number, and a decimal number is the number at
/// the front of the text -- `"1e3"` is 1000 and `"5abc"` is 5.
pub(crate) fn text_to_double(text: &str) -> f64 {
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() && (bytes[at] == b' ' || bytes[at] == b'\t') {
        at += 1;
    }
    let mut probe = at;
    if probe < bytes.len() && (bytes[probe] == b'-' || bytes[probe] == b'+') {
        probe += 1;
    }
    if is_hex_at(bytes, probe) {
        return text_to_int64(text) as f64;
    }
    let start = at;
    if at < bytes.len() && (bytes[at] == b'-' || bytes[at] == b'+') {
        at += 1;
    }
    let mut digit = false;
    while at < bytes.len() && bytes[at].is_ascii_digit() {
        at += 1;
        digit = true;
    }
    if at < bytes.len() && bytes[at] == b'.' {
        at += 1;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            at += 1;
            digit = true;
        }
    }
    if digit && at < bytes.len() && (bytes[at] == b'e' || bytes[at] == b'E') {
        let mut next = at + 1;
        if next < bytes.len() && (bytes[next] == b'+' || bytes[next] == b'-') {
            next += 1;
        }
        if next < bytes.len() && bytes[next].is_ascii_digit() {
            while next < bytes.len() && bytes[next].is_ascii_digit() {
                next += 1;
            }
            at = next;
        }
    }
    if !digit {
        return 0.0;
    }
    text[start..at].parse::<f64>().unwrap_or(0.0)
}

/// Whether the number a text holds is a whole number.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `Number("0x10")` is the
/// integer 16 and `Number("1.5")` is the float 1.5, so the shape the number
/// was written with is the shape of the answer.
pub(crate) fn text_holds_an_integer(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() && (bytes[at] == b' ' || bytes[at] == b'\t') {
        at += 1;
    }
    if at < bytes.len() && (bytes[at] == b'-' || bytes[at] == b'+') {
        at += 1;
    }
    if is_hex_at(bytes, at) {
        return true;
    }
    !bytes[at..]
        .iter()
        .any(|character| *character == b'.' || *character == b'e' || *character == b'E')
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

/// The sense of the case, the way v2 reads it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the parameter takes a
/// whole number -- 1 is case-sensitive and 0 is not -- or a text. The text
/// "On" or "1" is case-sensitive, "Off" or "0" is case-insensitive,
/// "Locale" looks at the letters through the locale, and "Logical" is the
/// logical order, which reads the run of digits of a name as a number
/// ("A2" comes before "A10"). A float is not a value the parameter takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaseSense {
    Sensitive,
    Insensitive,
    Locale,
    Logical,
}

/// The sense of the case of one parameter.
fn case_sense(
    values: &[Value],
    index: usize,
    name: &str,
    line: u32,
) -> Result<CaseSense, RuntimeError> {
    let invalid = || RuntimeError {
        message: crate::runtime::invalid_parameter(index + 1, name),
        line,
    };
    match values.get(index) {
        None => Ok(CaseSense::Insensitive),
        Some(Value::Unset) => Ok(CaseSense::Insensitive),
        Some(Value::Int(1)) => Ok(CaseSense::Sensitive),
        Some(Value::Int(0)) => Ok(CaseSense::Insensitive),
        Some(Value::Text(text)) => case_sense_of_a_text(text).ok_or_else(invalid),
        Some(_) => Err(invalid()),
    }
}

/// The sense of the case that a text names, when it names one.
fn case_sense_of_a_text(text: &str) -> Option<CaseSense> {
    if text.eq_ignore_ascii_case("On") || text == "1" {
        Some(CaseSense::Sensitive)
    } else if text.eq_ignore_ascii_case("Off") || text == "0" {
        Some(CaseSense::Insensitive)
    } else if text.eq_ignore_ascii_case("Locale") {
        Some(CaseSense::Locale)
    } else if text.eq_ignore_ascii_case("Logical") {
        Some(CaseSense::Logical)
    } else {
        None
    }
}

/// How two texts stand to each other, in this sense of the case.
fn compare_texts(left: &str, right: &str, sense: CaseSense) -> Ordering {
    match sense {
        CaseSense::Sensitive => left.cmp(right),
        CaseSense::Logical => compare_logically(left, right),
        // The locale of the machine is not the tree's to look at, and the
        // letters of the two texts are compared without the case, which is
        // what the locale does for the letters a script sorts.
        CaseSense::Insensitive | CaseSense::Locale => left.to_lowercase().cmp(&right.to_lowercase()),
    }
}

/// How two texts stand to each other in the logical order.
///
/// The run of digits of a name is a number there: "A2" comes before "A10".
/// The letters themselves are compared without the case.
fn compare_logically(left: &str, right: &str) -> Ordering {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let mut at = 0usize;
    while at < left.len() && at < right.len() {
        let a = left[at];
        let b = right[at];
        if a.is_ascii_digit() && b.is_ascii_digit() {
            // The two runs of digits are the numbers they write, and the
            // shorter run is the smaller one when the numbers are equal.
            let mut end_a = at;
            while end_a < left.len() && left[end_a].is_ascii_digit() {
                end_a += 1;
            }
            let mut end_b = at;
            while end_b < right.len() && right[end_b].is_ascii_digit() {
                end_b += 1;
            }
            let number_a: &str = &left[at..end_a].iter().collect::<String>();
            let number_b: &str = &right[at..end_b].iter().collect::<String>();
            let trimmed_a = number_a.trim_start_matches('0');
            let trimmed_b = number_b.trim_start_matches('0');
            let order = trimmed_a
                .len()
                .cmp(&trimmed_b.len())
                .then_with(|| trimmed_a.cmp(trimmed_b))
                .then_with(|| number_a.len().cmp(&number_b.len()));
            if order != Ordering::Equal {
                return order;
            }
            at = end_a.min(end_b);
            continue;
        }
        let folded_a = a.to_lowercase().next().unwrap_or(a);
        let folded_b = b.to_lowercase().next().unwrap_or(b);
        if folded_a != folded_b {
            return folded_a.cmp(&folded_b);
        }
        at += 1;
    }
    left.len().cmp(&right.len())
}

/// Whether two letters are the same, with or without the case.
fn same_letter(left: char, right: char, sense: CaseSense) -> bool {
    match sense {
        CaseSense::Sensitive => left == right,
        // The locale is read as the case-insensitive compare of the tree.
        CaseSense::Insensitive | CaseSense::Locale | CaseSense::Logical => {
            left.to_lowercase().eq(right.to_lowercase())
        }
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
            .all(|(left, right)| same_letter(*left, *right, CaseSense::Insensitive));
        if same {
            return haystack.char_indices().nth(at).map(|(byte, _)| byte);
        }
    }
    None
}

/// Where the needle is in the text, in this sense of the case.
fn find_in_sense(text: &str, needle: &str, sense: CaseSense) -> Option<usize> {
    match sense {
        CaseSense::Sensitive => text.find(needle),
        // The locale is read as the case-insensitive compare of the tree.
        CaseSense::Insensitive | CaseSense::Locale | CaseSense::Logical => {
            find_ignoring_case(text, needle)
        }
    }
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
pub(crate) fn abs(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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
pub(crate) fn round_to(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = if variant == 1 { "Ceil" } else { "Floor" };
    let number = number_argument(values, 0, line, name)?;
    let rounded = if variant == 1 {
        crate::runtime::ceil(number)
    } else {
        crate::runtime::floor(number)
    };
    Ok(Value::from_number(rounded))
}

/// `Max` and `Min`: the largest and the smallest of the numbers.
///
/// They share one handler, and the variant says which of the two it is.
pub(crate) fn extreme(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
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
pub(crate) fn modulo(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let dividend = values.first().cloned().unwrap_or(Value::Int(0));
    let divisor = values.get(1).cloned().unwrap_or(Value::Int(0));
    // v2 does not name the type that was wrong here: it says that the
    // parameters are invalid.
    let invalid = || RuntimeError {
        message: "Invalid parameter(s).".into(),
        line,
    };
    let left = number_value(&dividend).ok_or_else(invalid)?;
    let right = number_value(&divisor).ok_or_else(invalid)?;
    if right.to_number() == Some(0.0) {
        return Err(RuntimeError {
            message: "Divide by zero.".into(),
            line,
        });
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

/// `Round`: the number to the number of decimals the caller asks for.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: with a positive number of
/// decimals the answer is a text of exactly that many digits after the point,
/// zeros kept -- `Round(2.567, 2)` is "2.57", `Round(2, 2)` is "2.00" and
/// `Round(1.005, 2)` is "1.00" -- and with the number omitted, zero or
/// negative it is a whole number. The number is scaled by the power of ten
/// and rounded away from zero on the half, and the two parameters are
/// numbers: a text that does not hold one is refused.
pub(crate) fn round_to_digits(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let decimals = match values.get(1) {
        None => 0i32,
        Some(value) => {
            if !is_numeric_value(value) {
                return Err(RuntimeError {
                    message: crate::runtime::requires(2, "Round", TypeName::Number, value),
                    line,
                });
            }
            token_to_int64(value) as i32
        }
    };
    let number = match values.first() {
        None => 0.0,
        Some(value) => {
            if !is_numeric_value(value) {
                return Err(RuntimeError {
                    message: crate::runtime::requires(1, "Round", TypeName::Number, value),
                    line,
                });
            }
            token_to_double(value)
        }
    };
    let multiplier = crate::runtime::powi(10.0, decimals);
    let rounded = if number >= 0.0 {
        crate::runtime::floor(number * multiplier + 0.5)
    } else {
        crate::runtime::ceil(number * multiplier - 0.5)
    } / multiplier;
    if decimals > 0 {
        // The text is a text of the given number of decimals, and the shape
        // of a text that large is not one a script writes: the answer is
        // refused rather than allocated, and the reason says the parameter.
        if decimals as usize > 100_000 {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(2, "Round"),
                line,
            });
        }
        let body = if rounded.is_nan() {
            if rounded.is_sign_negative() {
                "-nan(ind)".to_string()
            } else {
                "nan(ind)".to_string()
            }
        } else if rounded.is_infinite() {
            if rounded < 0.0 {
                "-inf".to_string()
            } else {
                "inf".to_string()
            }
        } else {
            format!("{:.*}", decimals as usize, rounded)
        };
        return Ok(Value::Text(body));
    }
    // The whole number the rounded value is, with the nudge of the binary
    // that keeps a value such as 1039.9999999999999 out of the answer.
    let nudged = rounded + if rounded > 0.0 { 0.2 } else { -0.2 };
    Ok(Value::Int(double_to_int64(nudged)))
}

/// `Sqrt`: the square root.
pub(crate) fn square_root(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "Sqrt")?;
    if number < 0.0 {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "Sqrt"),
            line,
        });
    }
    // AutoHotkey v2 always answers a float here, even for a square.
    Ok(Value::Float(crate::runtime::sqrt(number)))
}

/// `Cos`, `Sin` and `Tan`: the trigonometry of a number.
///
/// They share one handler, and the variant says which of the three it is.
/// AutoHotkey v2 answers a float here, whatever went in.
pub(crate) fn trigonometry(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
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
pub(crate) fn inverse_trigonometry(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = match variant {
        0 => "ASin",
        1 => "ACos",
        _ => "ATan",
    };
    let number = number_argument(values, 0, line, name)?;
    if variant != 2 && !(-1.0..=1.0).contains(&number) {
        // The sine and the cosine are not there outside the unit range,
        // and v2 says that the parameter is invalid.
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
    }
    let result = match variant {
        0 => crate::runtime::asin(number),
        1 => crate::runtime::acos(number),
        _ => crate::runtime::atan(number),
    };
    Ok(Value::Float(result))
}

/// `Exp`: e raised to the number.
pub(crate) fn exponential(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Float(crate::runtime::exp(number_argument(
        values, 0, line, "Exp",
    )?)))
}

/// `Ln` and `Log`: the logarithm of the number.
///
/// `Ln` is the natural one and `Log` the one of base ten.
pub(crate) fn logarithm(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = if variant == 1 { "Log" } else { "Ln" };
    let number = number_argument(values, 0, line, name)?;
    if number < 0.0 {
        // The logarithm of a negative number is not there, and v2 says
        // that the parameter is invalid.
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
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
pub(crate) fn random(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The seed a call of RandomSeed left is mixed into the entropy of the
    // machine, so that a seeded run of the same script asks the same numbers
    // of the machine.
    let now = rt.now_ms() ^ rt.the_seed_of_the_random();
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
            let whole = values[0].type_name() != TypeName::Float
                && values[1].type_name() != TypeName::Float;
            if whole {
                let span = (high - low) as u64;
                let offset = if span == 0 {
                    0
                } else {
                    next_random(now) as u64 % (span + 1)
                };
                Ok(Value::Int((low as i64).saturating_add(offset as i64)))
            } else {
                Ok(Value::Float(low + unit_random(now) * (high - low)))
            }
        }
    }
}

/// `NumGet`: the number the bytes of a buffer hold.
///
/// v2 reads the memory of an address as well, and this runtime does
/// not: the source of a number is a buffer, and the bytes of a buffer
/// live in the runtime.
pub(crate) fn num_get(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let source = values.first().cloned().unwrap_or(Value::Int(0));
    if !matches!(source, Value::Object(_)) {
        // The memory of an address is not read here: the source of a
        // number is a buffer.
        let message = match source {
            Value::Int(_) => "this runtime does not read the memory of an address: the source of a number is a buffer.".to_string(),
            other => expected_type(TypeName::Object, other.type_name()),
        };
        return Err(RuntimeError { message, line });
    }
    // The offset is the second parameter when the type is the third.
    let (offset, type_name) = if values.len() > 2 {
        (integer_argument(values, 1, line)?, text_of(values, 2))
    } else {
        (0, text_of(values, 1))
    };
    let kind = crate::classes::buffer::number_type(&type_name).ok_or_else(|| RuntimeError {
        message: "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.".into(),
        line,
    })?;
    match crate::classes::buffer::read_number(&source, offset, kind, line) {
        Ok(number) => Ok(number),
        Err(RuntimeFailure::Error(error)) => Err(error),
        Err(RuntimeFailure::Detailed(error, _)) => Err(error),
        Err(_) => Err(RuntimeError {
            message: "the buffer did not answer.".into(),
            line,
        }),
    }
}

/// `NumPut`: a number, into the bytes of a buffer.
///
/// The types and the numbers come in pairs, and the target and the
/// offset come after them: v2 writes the target last.
pub(crate) fn num_put(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The target is the last object of the arguments: the pairs come
    // before it, and the offset comes after it.
    let Some(index) = values
        .iter()
        .rposition(|value| matches!(value, Value::Object(_)))
    else {
        let message = match values.last() {
            Some(Value::Int(_)) => "this runtime does not write to the memory of an address: the target of a number is a buffer.".to_string(),
            Some(other) => expected_type(TypeName::Object, other.type_name()),
            None => expected_type(TypeName::Object, TypeName::String),
        };
        return Err(RuntimeError { message, line });
    };
    let pairs = &values[..index];
    if pairs.len() % 2 != 0 {
        return Err(RuntimeError {
            message: "the types and the numbers of a write come in pairs.".into(),
            line,
        });
    }
    if index + 2 < values.len() {
        return Err(RuntimeError {
            message: "a write takes the target and the offset after the pairs.".into(),
            line,
        });
    }
    let offset = match values.get(index + 1) {
        Some(_) => integer_argument(values, index + 1, line)?,
        None => 0,
    };
    let mut numbers = Vec::with_capacity(pairs.len() / 2);
    let mut at = 0;
    while at < pairs.len() {
        let kind = crate::classes::buffer::number_type(&text_of(pairs, at)).ok_or_else(|| RuntimeError {
            message: "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.".into(),
            line,
        })?;
        let number = pairs[at + 1].clone();
        let number = number_value(&number).ok_or_else(|| RuntimeError {
            message: crate::runtime::requires(at + 2, "NumPut", TypeName::Number, &number),
            line,
        })?;
        numbers.push((kind, number));
        at += 2;
    }
    let target = &values[index];
    match crate::classes::buffer::write_numbers(target, offset, &numbers, line) {
        Ok(address) => Ok(Value::Int(address)),
        Err(RuntimeFailure::Error(error)) => Err(error),
        Err(RuntimeFailure::Detailed(error, _)) => Err(error),
        Err(_) => Err(RuntimeError {
            message: "the buffer did not answer.".into(),
            line,
        }),
    }
}

// -- the text -------------------------------------------------------------------

/// `Chr`: the character with that code point.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the parameter wants
/// a number and not a whole one, and the point that is not whole is the
/// one before it.
pub(crate) fn chr(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let code = number_argument(values, 0, line, "Chr")? as i64;
    // Measured on the binary of v2.0.19 on 2026-11-10: a surrogate code unit
    // is a text of its own -- `StrLen(Chr(0xD83D))` is one -- and a value
    // outside Unicode is an invalid parameter.
    if !(0..=0x10FFFF).contains(&code) {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "Chr"),
            line,
        });
    }
    // A surrogate is one code unit of its own, and every other code is the
    // character of it -- the pair of units of a character above the plane of
    // BMP is written by the text of it.
    let text = if (0xD800..=0xDFFF).contains(&code) {
        crate::text16::the_text_of(&[code as u16])
    } else {
        let Some(character) = char::from_u32(code as u32) else {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(1, "Chr"),
                line,
            });
        };
        character.to_string()
    };
    Ok(Value::Text(text))
}

/// `Format`: a pattern with the values in it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the pattern is the one of
/// the C library, with the placeholders of the dialect. A pattern that is one
/// argument is answered as the text of it, and the arguments a placeholder
/// names are read through their references, the way the binary reads them.
pub(crate) fn format(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(first) = values.first() else {
        return Ok(Value::Text(String::new()));
    };
    if matches!(first, Value::Object(_) | Value::List(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "Format", TypeName::String, first),
            line,
        });
    }
    // A reference reads through, the way the binary reads the variable it
    // points at.
    let mut arguments = Vec::with_capacity(values.len());
    for value in values {
        arguments.push(match value {
            Value::Reference(reference) => rt
                .read_reference(reference)
                .unwrap_or_else(|| Value::Text(String::new())),
            other => other.clone(),
        });
    }
    let pattern = match arguments.first() {
        Some(value @ (Value::Object(_) | Value::List(_))) => {
            return Err(RuntimeError {
                message: crate::runtime::requires(1, "Format", TypeName::String, value),
                line,
            })
        }
        Some(value) => value.to_text(),
        None => String::new(),
    };
    Ok(Value::Text(format_pattern(&pattern, &arguments, line)?))
}

/// `InStr`: where the needle is in the haystack.
///
/// The position counts from one, and it is zero when the needle is not
/// there at all. The third parameter is the sense of the case, the fourth is
/// the place where the search begins and the fifth is the occurrence the
/// caller wants.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, and read out of the
/// `BIF_InStr` of the source:
///
/// - a negative place is a place counted from the end -- `len + 1 + place`
///   -- and the search runs from the right, so `InStr("hello", "l", "On",
///   -2)` is 4; when the occurrence is not given with it, the search from
///   the right is the default;
/// - a negative occurrence is the search from the right, and the fourth
///   parameter then limits the haystack to the place it names;
/// - the matches of a search from the left do not overlap: after a match the
///   search goes on after it, so `InStr("aaaa", "aa", "On", 1, 2)` is 3 and
///   not 2;
/// - the place zero and the occurrence zero are refused, and a place or an
///   occurrence that is not a number is refused with the kind it got.
pub(crate) fn in_str(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let haystack: Vec<char> = text_of(values, 0).chars().collect();
    let needle: Vec<char> = text_of(values, 1).chars().collect();
    if needle.is_empty() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "InStr"),
            line,
        });
    }
    let sense = case_sense(values, 2, "InStr", line)?;
    // The logical order is not a sense of the case that a search can use,
    // and the binary refuses it here.
    if sense == CaseSense::Logical {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(3, "InStr"),
            line,
        });
    }
    let start_given = values.get(3).is_some();
    let mut offset: i64 = 1;
    if let Some(value) = values.get(3) {
        if !is_numeric_value(value) {
            return Err(RuntimeError {
                message: crate::runtime::requires(4, "InStr", TypeName::Number, value),
                line,
            });
        }
        offset = token_to_int64(value);
        if offset == 0 {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(4, "InStr"),
                line,
            });
        }
    }
    let mut occurrence: i64 = 1;
    if let Some(value) = values.get(4) {
        if !is_numeric_value(value) {
            return Err(RuntimeError {
                message: crate::runtime::requires(5, "InStr", TypeName::Number, value),
                line,
            });
        }
        // The binary reads the occurrence through an `int`.
        occurrence = token_to_int64(value) as i32 as i64;
        if occurrence == 0 {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(5, "InStr"),
                line,
            });
        }
    }
    let length = haystack.len() as i64;
    if offset < 0 {
        if values.get(4).is_none() {
            // A place from the right with no occurrence is the search from
            // the right.
            occurrence = -1;
        }
        // A place at the very front of a 64-bit integer would run over the
        // edge of it: the sum is taken to the edge, and the place is the
        // first character after that.
        offset = offset.saturating_add(length + 1);
    }
    if occurrence > 0 {
        offset = offset.saturating_sub(1);
    }
    if offset < 0 {
        offset = 0;
    } else if offset > length {
        offset = length;
    }
    if occurrence < 0 {
        // The search from the right, over the part of the haystack that the
        // fourth parameter leaves.
        let end = if start_given { offset as usize } else { haystack.len() };
        let wanted = (-occurrence) as u64;
        let mut found = 0u64;
        if needle.len() <= end {
            for at in (0..=(end - needle.len())).rev() {
                if haystack[at..at + needle.len()]
                    .iter()
                    .zip(needle.iter())
                    .all(|(left, right)| same_letter(*left, *right, sense))
                {
                    found += 1;
                    if found == wanted {
                        return Ok(Value::Int(at as i64 + 1));
                    }
                }
            }
        }
        return Ok(Value::Int(0));
    }
    // The search from the left: after a match it goes on after it.
    let mut at = offset as usize;
    let mut count: i64 = 1;
    while at + needle.len() <= haystack.len() {
        if haystack[at..at + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(left, right)| same_letter(*left, *right, sense))
        {
            if count == occurrence {
                return Ok(Value::Int(at as i64 + 1));
            }
            count += 1;
            at += needle.len();
        } else {
            at += 1;
        }
    }
    Ok(Value::Int(0))
}

/// `Ord`: the code point of the first character.
///
/// An empty text has no character, and the code point of it is zero.
pub(crate) fn ord(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    // Measured on the binary of v2.0.19 on 2026-11-10: the code of the first
    // character -- `Ord(Chr(0x1F600))` is 0x1F600 -- and the code unit of a
    // lone surrogate -- `Ord(SubStr(Chr(0x1F600), 2))` is 0xDE00. A text of no
    // character has the code zero.
    let code = match text.chars().next() {
        None => 0,
        Some(character) => match crate::text16::the_surrogate_of_a_character(character) {
            Some(unit) => unit as i64,
            None => character as u32 as i64,
        },
    };
    Ok(Value::Int(code))
}

/// The options of a `Sort`, read out of the second parameter.
struct SortOptions {
    /// The character that ends an item: a linefeed unless `Dx` says another.
    delimiter: char,
    /// How the letters are compared.
    sense: CaseSense,
    /// `N`: each item is read as a number.
    numeric: bool,
    /// `R`: the order is reversed.
    reverse: bool,
    /// `Pn`: the compare begins this many characters into each item.
    column: usize,
    /// `Z`: the blank item after a trailing delimiter is an item of its own.
    blank_trailing: bool,
    /// `\`: the name after the last backslash is what is compared.
    naked_filename: bool,
    /// `Random`: the items are shuffled.
    random: bool,
    /// `U`: the duplicates that end up next to each other are removed.
    unique: bool,
}

/// The options of a `Sort`, read out of its second parameter.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, and read out of the
/// `BIF_Sort` of the source: the letters are read without the case, a letter
/// the binary does not know is ignored, and the `Random` of `R` wins over
/// the reverse order.
fn sort_options(text: &str) -> SortOptions {
    let chars: Vec<char> = text.chars().collect();
    let mut options = SortOptions {
        delimiter: '\n',
        sense: CaseSense::Insensitive,
        numeric: false,
        reverse: false,
        column: 0,
        blank_trailing: false,
        naked_filename: false,
        random: false,
        unique: false,
    };
    let mut at = 0usize;
    while at < chars.len() {
        match chars[at].to_ascii_uppercase() {
            'C' => {
                if chars.get(at + 1).map(|c| c.to_ascii_uppercase()) == Some('L') {
                    let rest: String = chars[at + 2..].iter().take(6).collect();
                    if rest.eq_ignore_ascii_case("ogical") {
                        options.sense = CaseSense::Logical;
                        at += 8;
                    } else {
                        let rest: String = chars[at + 2..].iter().take(5).collect();
                        if rest.eq_ignore_ascii_case("ocale") {
                            at += 7;
                        } else {
                            at += 2;
                        }
                        options.sense = CaseSense::Locale;
                    }
                } else {
                    let rest: String = chars[at + 1..].iter().take(3).collect();
                    if rest.eq_ignore_ascii_case("Off") {
                        options.sense = CaseSense::Insensitive;
                        at += 4;
                    } else if chars.get(at + 1) == Some(&'0') {
                        options.sense = CaseSense::Insensitive;
                        at += 1;
                    } else {
                        let rest: String = chars[at + 1..].iter().take(2).collect();
                        if rest.eq_ignore_ascii_case("On") {
                            at += 3;
                        } else {
                            at += 1;
                        }
                        options.sense = CaseSense::Sensitive;
                    }
                }
            }
            'D' => {
                // The character after the `D` is the delimiter itself.
                if at + 1 < chars.len() {
                    options.delimiter = chars[at + 1];
                    at += 2;
                } else {
                    at += 1;
                }
            }
            'N' => {
                options.numeric = true;
                at += 1;
            }
            'P' => {
                // The number after the `P` is the column, and a number that
                // is not one starts at the first character.
                let mut next = at + 1;
                while next < chars.len() && chars[next].is_ascii_whitespace() {
                    next += 1;
                }
                let negative = next < chars.len() && chars[next] == '-';
                if negative || (next < chars.len() && chars[next] == '+') {
                    next += 1;
                }
                let mut value: i64 = 0;
                while next < chars.len() && chars[next].is_ascii_digit() {
                    value = value
                        .saturating_mul(10)
                        .saturating_add((chars[next] as u8 - b'0') as i64);
                    next += 1;
                }
                if negative {
                    value = -value;
                }
                options.column = if value < 1 { 1 } else { value as usize } - 1;
                at += 1;
            }
            'R' => {
                let rest: String = chars[at..].iter().take(6).collect();
                if rest.eq_ignore_ascii_case("Random") {
                    options.random = true;
                    at += 6;
                } else {
                    options.reverse = true;
                    at += 1;
                }
            }
            'U' => {
                options.unique = true;
                at += 1;
            }
            'Z' => {
                options.blank_trailing = true;
                at += 1;
            }
            '\\' => {
                options.naked_filename = true;
                at += 1;
            }
            _ => at += 1,
        }
    }
    options
}

/// The key of an item: the part of it that the options compare.
fn sort_key(item: &str, options: &SortOptions) -> String {
    if options.column == 0 {
        return item.to_string();
    }
    item.chars().skip(options.column).collect()
}

/// The name of an item after the last backslash.
fn naked_filename(item: &str) -> &str {
    match item.rfind('\\') {
        Some(at) => &item[at + 1..],
        None => item,
    }
}

/// `Sort`: the items of a text in order.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, and read out of the
/// `BIF_Sort` of the source:
///
/// - the second parameter is the options, read letter by letter: `C`, `C1`
///   and `COn` take the case into the sort, `C0`, `COff` and the default
///   leave it out, `CL`/`CLocale` look at the letters through the locale and
///   `CLogical` reads a run of digits as a number; `Dx` sets the delimiter to
///   the character after it; `N` reads each item as a number; `Pn` starts the
///   compare at the nth character; `R` reverses the order; `Random` shuffles;
///   `U` removes the adjacent duplicates; `Z` counts the blank item after a
///   trailing delimiter; `\` compares the name after the last backslash. A
///   letter the binary does not know is ignored.
/// - the third parameter is a callback, called with the two items and the
///   offset of the second from the first in the original text; the sign of
///   the number it answers orders the two, and zero leaves them in place.
/// - the first parameter is a text; an array is refused with the reason, and
///   so is a value that is not a function when the third one is given.
pub(crate) fn sort(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let contents = sort_text(values, 0, line)?;
    let options = sort_options(&sort_text(values, 1, line)?);
    let callback = match values.get(2) {
        None => None,
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => Some(value.clone()),
        Some(_) => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(3, "Sort"),
                line,
            })
        }
    };
    if contents.is_empty() {
        return Ok(Value::Text(String::new()));
    }
    // A trailing delimiter is the terminator of the last item unless `Z`
    // says that the blank item after it is an item of its own.
    let mut working = contents.clone();
    let mut terminate_last = false;
    let mut trailing_crlf = false;
    if !options.blank_trailing && working.ends_with(options.delimiter) {
        terminate_last = true;
    } else if options.delimiter == '\n' {
        // The lines of a CRLF list are sorted as if the delimiter were the
        // linefeed alone, and the carriage return stays at the end of the
        // item: measured on 2026-11-09, "B`r`nA" sorts to "A`r`nB".
        let chars: Vec<char> = working.chars().collect();
        if let Some(at) = chars.iter().position(|c| *c == '\n') {
            if at > 0 && chars[at - 1] == '\r' {
                trailing_crlf = true;
                terminate_last = true;
                working.push_str("\r\n");
            }
        }
    }
    let mut items: Vec<String> = working
        .split(options.delimiter)
        .map(|item| item.to_string())
        .collect();
    if terminate_last {
        // The blank item that the last delimiter makes is not an item.
        items.pop();
    }
    if items.len() == 1 {
        return Ok(Value::Text(contents));
    }
    // Where each item begins in the text being sorted: the callback is told
    // the distance between the two items it compares.
    let mut positions: Vec<usize> = Vec::with_capacity(items.len());
    let mut position = 0usize;
    for item in &items {
        positions.push(position);
        position += item.chars().count() + 1;
    }
    let keys: Vec<String> = items
        .iter()
        .map(|item| sort_key(item, &options))
        .collect();
    let mut order: Vec<usize> = (0..items.len()).collect();
    let mut failure: Option<RuntimeError> = None;
    if let Some(callback) = &callback {
        // A function is called as a value, and any other object is asked for
        // its `Call` method: measured on the binary of v2.0.19 on 2026-11-09,
        // an array as the third parameter of a `Sort` answers "This value of
        // type "Array" has no method named "Call".".
        let is_a_function = match callback {
            Value::Object(object) => {
                let class = object.borrow().class.to_lowercase();
                class == "func" || class == "boundfunc" || class == "closure"
            }
            _ => false,
        };
        let no_method = |value: &Value| RuntimeError {
            message: format!(
                "This value of type \"{}\" has no method named \"Call\".",
                value.the_kind()
            ),
            line,
        };
        // The callback is called with the two items and the distance
        // between them, and a zero leaves them as they are.
        order.sort_by(|&left, &right| {
            if failure.is_some() {
                return Ordering::Equal;
            }
            let offset = positions[right] as i64 - positions[left] as i64;
            let arguments = [
                Value::Text(items[left].clone()),
                Value::Text(items[right].clone()),
                Value::Int(offset),
            ];
            let called = if is_a_function {
                match crate::classes::func::call_the_value(callback, &arguments, rt, line) {
                    Some(Ok(value)) => Ok(value),
                    Some(Err(error)) => Err(the_failure_in_one(error, line)),
                    None => Err(no_method(callback)),
                }
            } else {
                match rt.call_a_method_of_an_object(callback, "Call", &arguments, line) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(the_failure_in_one(error, line)),
                }
            };
            match called {
                Ok(value) => match token_to_int64(&value) {
                    number if number < 0 => Ordering::Less,
                    number if number > 0 => Ordering::Greater,
                    _ => Ordering::Equal,
                },
                Err(error) => {
                    failure = Some(error);
                    Ordering::Equal
                }
            }
        });
    } else if options.random {
        // A shuffle: the items are put in a random order, and the options
        // of `N`, `C` and `CL` are still what the duplicates are read by.
        // The seed a call of RandomSeed left is mixed into the entropy of the
    // machine, so that a seeded run of the same script asks the same numbers
    // of the machine.
    let now = rt.now_ms() ^ rt.the_seed_of_the_random();
        for at in (1..order.len()).rev() {
            let pick = (unit_random(now) * (at as f64 + 1.0)) as usize;
            order.swap(at, pick.min(at));
        }
    } else {
        order.sort_by(|&left, &right| {
            let order = if options.naked_filename {
                compare_texts(
                    naked_filename(&items[left]),
                    naked_filename(&items[right]),
                    options.sense,
                )
            } else if options.numeric {
                let a = text_to_double(&keys[left]);
                let b = text_to_double(&keys[right]);
                // A tie keeps the two in the order they came in.
                a.partial_cmp(&b).unwrap_or(Ordering::Equal)
            } else {
                compare_texts(&keys[left], &keys[right], options.sense)
            };
            // A value that compares equal keeps the place it had, and the
            // reverse order turns that place around too.
            let order = if order == Ordering::Equal {
                left.cmp(&right)
            } else {
                order
            };
            if options.reverse {
                order.reverse()
            } else {
                order
            }
        });
    }
    if let Some(failure) = failure {
        return Err(failure);
    }
    // The duplicates that ended up next to each other are removed, and the
    // rule of a duplicate is the rule of the sort that made them adjacent.
    let mut kept: Vec<usize> = Vec::with_capacity(order.len());
    for &index in &order {
        if options.unique {
            if let Some(&previous) = kept.last() {
                if sort_duplicates(&items[previous], &items[index], &options) {
                    continue;
                }
            }
        }
        kept.push(index);
    }
    let mut out = String::with_capacity(contents.len());
    for (at, &index) in kept.iter().enumerate() {
        out.push_str(&items[index]);
        if at + 1 < kept.len() || terminate_last {
            out.push(options.delimiter);
        }
    }
    if trailing_crlf && out.ends_with("\r\n") {
        out.truncate(out.len() - 2);
    }
    Ok(Value::Text(out))
}

/// The text of one parameter of a `Sort`.
fn sort_text(values: &[Value], index: usize, line: u32) -> Result<String, RuntimeError> {
    match values.get(index) {
        None => Ok(String::new()),
        // Measured on the binary of v2.0.19 on 2026-11-10: the absent value is
        // no argument, and the text of no argument is the empty text.
        Some(Value::Unset) => Ok(String::new()),
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => {
            Err(RuntimeError {
                message: crate::runtime::requires(index + 1, "Sort", TypeName::String, value),
                line,
            })
        }
        Some(value) => Ok(value.to_text()),
    }
}

/// Whether two items of a sort are a duplicate of each other.
///
/// The rule is the one of the binary: a numeric sort compares the numbers,
/// the logical order compares the names in that order, and the other sorts
/// compare the two items with their sense of the case.
fn sort_duplicates(left: &str, right: &str, options: &SortOptions) -> bool {
    if options.numeric && options.column == 0 {
        text_to_double(left) == text_to_double(right)
    } else if options.sense == CaseSense::Logical {
        compare_logically(left, right) == Ordering::Equal
    } else {
        compare_texts(left, right, options.sense) == Ordering::Equal
    }
}

/// The failure of a call, as the failure of the built-in that made it.
///
/// The details of the failure are not carried here: a built-in answers with
/// the message and the line, and the runtime names the built-in in the
/// `What`.
fn the_failure_in_one(error: RuntimeFailure, line: u32) -> RuntimeError {
    match error {
        RuntimeFailure::Error(error)
        | RuntimeFailure::Detailed(error, _)
        | RuntimeFailure::Fatal(error) => error,
        RuntimeFailure::Thrown(value) => RuntimeError {
            message: value.to_text(),
            line,
        },
        _ => RuntimeError {
            message: "the sort stopped the run.".to_string(),
            line,
        },
    }
}

/// `StrCompare`: how two texts stand to each other.
///
/// The answer is -1, 0 or 1, and the case sense says whether the letters
/// are taken as they are.
pub(crate) fn str_compare(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let left = text_of(values, 0);
    let right = text_of(values, 1);
    let order = compare_texts(&left, &right, case_sense(values, 2, "StrCompare", line)?);
    Ok(Value::Int(match order {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }))
}

/// `StrLen`: how many characters the text has.
pub(crate) fn str_len(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // Measured on the binary of v2.0.19 on 2026-11-10: the length is counted in
    // the code units of UTF-16, so `StrLen(Chr(0x1F600))` is two.
    Ok(Value::Int(
        crate::text16::the_number_of_the_code_units(&text_of(values, 0)) as i64,
    ))
}

/// `StrLower` and `StrUpper`: the text in the other case.
///
/// They share one handler, and the variant says which of the two it is.
pub(crate) fn change_case(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    Ok(Value::Text(if variant == 1 {
        text.to_uppercase()
    } else {
        text.to_lowercase()
    }))
}

/// `StrReplace`: the text with one part of it swapped for another.
///
/// v2 writes the number of the swaps into the variable of the caller, and
/// the limit says how many of them are swapped: none of them means all of
/// them. The case sense says whether the letters are taken as they are,
/// and it is off when the caller does not say.
pub(crate) fn str_replace(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let haystack = text_argument("StrReplace", values, 0, line)?;
    let needle = text_argument("StrReplace", values, 1, line)?;
    let replacement = text_argument("StrReplace", values, 2, line)?;
    // Measured on the binary of v2.0.19 on 2026-11-10: the fourth parameter of
    // the call is the sense of the case, the fifth is the variable that takes
    // the number of the swaps, and the sixth is the limit. An argument that is
    // the absent value is no argument: the case is the one of the dialect, the
    // number of the swaps is not written, and the limit is all of them.
    let sense = match values.get(3) {
        Some(Value::Unset) | None => CaseSense::Insensitive,
        Some(_) => case_sense(values, 3, "StrReplace", line)?,
    };
    // The logical order is not a sense of the case that a replacement can
    // use, and the binary refuses it here.
    if sense == CaseSense::Logical {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(4, "StrReplace"),
            line,
        });
    }
    let limit = match values.get(5) {
        Some(Value::Unset) | None => 0,
        Some(_) => number_of(values, 5) as i64,
    };
    let mut out = String::with_capacity(haystack.len());
    let mut rest = haystack.as_str();
    let mut replaced = 0i64;
    // Measured on the binary of v2.0.19 on 2026-10-31: a needle that is
    // empty changes nothing, and the text comes back as it is.
    while !needle.is_empty() {
        let found = find_in_sense(rest, &needle, sense);
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
        if matches!(value, Value::Unset) {
            return Ok(Value::Text(out));
        }
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    5,
                    "StrReplace",
                    "a variable reference",
                    value,
                ),
                line,
            });
        }
        rt.write_reference(value, Value::Int(replaced));
    }
    Ok(Value::Text(out))
}

/// The encodings the string functions of v2 name.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `UTF-8`, `UTF-16` and
/// `CP0` are the names it takes, the case does not matter, and the two
/// names that are spelled without the hyphen -- `UTF8`, `UTF16` -- are not
/// names. The encoding of a call that does not name one is `UTF-16`, the
/// text of the machine.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TextEncoding {
    Utf8,
    Utf16,
    /// The code page of the machine. The tree is UTF-8 everywhere, and the
    /// code page of the box is not asked: the letters of ASCII are the
    /// same bytes in both, and the others are the ones the tree can hold.
    Cp0,
}

/// The encoding a string function was given.
fn the_encoding_of(name: Option<&Value>) -> Result<TextEncoding, String> {
    match name {
        None => Ok(TextEncoding::Utf16),
        Some(Value::Text(text)) => match text.to_ascii_lowercase().as_str() {
            "utf-8" => Ok(TextEncoding::Utf8),
            "utf-16" => Ok(TextEncoding::Utf16),
            "cp0" => Ok(TextEncoding::Cp0),
            _ => Err("Invalid Encoding.".to_string()),
        },
        Some(_) => Err("Invalid Encoding.".to_string()),
    }
}

/// The bytes of a text, in an encoding.
fn the_bytes_of(text: &str, encoding: TextEncoding) -> Vec<u8> {
    match encoding {
        TextEncoding::Utf8 | TextEncoding::Cp0 => text.as_bytes().to_vec(),
        TextEncoding::Utf16 => {
            let mut bytes: Vec<u8> = Vec::with_capacity(text.len() * 2);
            for unit in text.encode_utf16() {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
            bytes
        }
    }
}

/// The terminator of an encoding: the bytes that end a text in it.
fn the_terminator_of(encoding: TextEncoding) -> Vec<u8> {
    match encoding {
        TextEncoding::Utf8 | TextEncoding::Cp0 => vec![0],
        TextEncoding::Utf16 => vec![0, 0],
    }
}

/// The text of some bytes, in an encoding, up to a terminator or a length.
///
/// The length is the number of the units of the encoding: the bytes of
/// `UTF-8` and the sixteen-bit units of `UTF-16`. Measured on the binary of
/// v2.0.19 on 2026-11-09: `StrGet(buf, 2, "UTF-8")` answers the first two
/// characters of a text of ASCII.
fn the_text_of(bytes: &[u8], encoding: TextEncoding, length: Option<usize>) -> String {
    match encoding {
        TextEncoding::Utf8 | TextEncoding::Cp0 => {
            let end = bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len());
            let end = length.map(|length| end.min(length)).unwrap_or(end);
            String::from_utf8_lossy(&bytes[..end]).into_owned()
        }
        TextEncoding::Utf16 => {
            let mut units: Vec<u16> = Vec::new();
            let mut at = 0;
            while at + 1 < bytes.len() {
                let unit = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
                if unit == 0 {
                    break;
                }
                units.push(unit);
                at += 2;
                if let Some(length) = length {
                    if units.len() >= length {
                        break;
                    }
                }
            }
            String::from_utf16_lossy(&units)
        }
    }
}

/// `StrPtr`: the address of the text of a string.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the parameter is a
/// text, and nothing else -- a number and an object are refused with the
/// kind they are -- and the answer is the address of the buffer of the
/// text, which is a number above zero and the same number for the same
/// variable read twice. The tree keeps the addresses of the texts that a
/// script asks for; the same text answers the same address.
pub(crate) fn str_ptr(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = match values.first() {
        Some(Value::Text(text)) => text.clone(),
        Some(value) => {
            return Err(RuntimeError {
                message: crate::runtime::expected_type(TypeName::String, value.type_name()),
                line,
            })
        }
        None => String::new(),
    };
    Ok(Value::Int(rt.the_address_of_a_text(&text)))
}

/// `StrGet`: the text at an address, or the text of a buffer.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the source is a buffer
/// or an address, and a text there is refused with "Parameter #1 invalid.";
/// the length and the encoding are the second and the third parameters,
/// and a second parameter that is a text is the encoding, with the length
/// left to the terminator. The encoding of a name the binary does not know
/// is refused with "Invalid Encoding.".
pub(crate) fn str_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(source) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "StrGet"),
            line,
        });
    };
    let bytes: Vec<u8> = match source {
        // The bytes of a buffer are the bytes of the buffer.
        Value::Object(_) => {
            crate::classes::buffer::raw_bytes(source, line).map_err(|failure| RuntimeError { message: the_message_of(failure), line })?
        }
        // The address of a text of the script: the text is a UTF-16 text,
        // and the reading of it with another encoding reads the bytes of
        // it, the way the binary reads the buffer of the variable.
        Value::Int(address) => match rt.the_text_at_an_address(*address) {
            Some(text) => the_bytes_of(&text, TextEncoding::Utf16),
            None => {
                return Err(RuntimeError {
                    message: crate::runtime::invalid_parameter(1, "StrGet"),
                    line,
                })
            }
        },
        Value::Text(_) => {
            return Err(RuntimeError {
                message: "Parameter #1 invalid.".to_string(),
                line,
            })
        }
        _ => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(1, "StrGet"),
                line,
            })
        }
    };
    // The second parameter is the length when it is a number, and the
    // encoding when it is a text; the third one is the encoding.
    let (length, encoding_name) = match values.get(1) {
        Some(value) if is_numeric_value(value) => (
            Some(token_to_int64(value).max(0) as usize),
            values.get(2),
        ),
        Some(value) => (None, Some(value)),
        None => (None, None),
    };
    let encoding = the_encoding_of(encoding_name).map_err(|message| RuntimeError { message, line })?;
    Ok(Value::Text(the_text_of(&bytes, encoding, length)))
}

/// `StrPut`: write a text into a buffer, or say how many bytes it needs.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the parameters are the
/// text, the target, the encoding and the length; a second parameter that
/// is a text is the encoding, with the target left out -- the order of the
/// v1 -- and a target that is not a buffer is refused with "Parameter #2 of
/// StrPut is invalid.". Without a target the answer is the number of the
/// bytes the text needs, terminator included. The third parameter may be a
/// number: it is the size the caller says the target has, in characters,
/// and a text that does not fit in it is refused with "Invalid Length.".
pub(crate) fn str_put(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let _ = &rt;
    let text = match values.first() {
        Some(Value::Text(text)) => text.clone(),
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => {
            return Err(RuntimeError {
                message: crate::runtime::requires(1, "StrPut", TypeName::String, value),
                line,
            })
        }
        Some(value) => value.to_text(),
        None => String::new(),
    };
    // The parameters, in the two orders that the binary takes.
    let (target, encoding_name, length_name) = match values.get(1) {
        None => (None, values.get(2), values.get(3)),
        Some(value @ Value::Object(_)) => (Some(value), values.get(2), values.get(3)),
        Some(value @ Value::Text(_)) => (None, Some(value), values.get(2)),
        Some(_) => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(2, "StrPut"),
                line,
            })
        }
    };
    // A length with no target is not a use of v2.
    if target.is_none() && length_name.is_some() {
        return Err(RuntimeError {
            message: "Invalid parameter(s).".to_string(),
            line,
        });
    }
    // The size the caller says the target has, when the third parameter is
    // a number instead of the name of an encoding.
    let mut capacity: Option<i64> = None;
    let encoding = match encoding_name {
        Some(value) if is_numeric_value(value) => {
            capacity = Some(token_to_int64(value));
            TextEncoding::Utf16
        }
        other => the_encoding_of(other).map_err(|message| RuntimeError { message, line })?,
    };
    let characters = text.chars().count() as i64;
    let mut bytes = the_bytes_of(&text, encoding);
    let mut terminator = true;
    if let Some(capacity) = capacity {
        if capacity < 0 || capacity < characters {
            return Err(RuntimeError {
                message: "Invalid Length.".to_string(),
                line,
            });
        }
        // A text that fits exactly fills the buffer it was told about, and
        // there is no room for the terminator: that is the answer of the
        // binary, which counts the bytes it wrote.
        if capacity == characters {
            terminator = false;
        }
    }
    if terminator {
        bytes.extend_from_slice(&the_terminator_of(encoding));
    }
    let Some(target) = target else {
        return Ok(Value::Int(bytes.len() as i64));
    };
    // The target is a buffer, and it has to hold the text.
    let size = match crate::classes::buffer::size_of(target, line) {
        Ok(size) => size,
        Err(_) => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(2, "StrPut"),
                line,
            })
        }
    };
    if bytes.len() as i64 > size {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "StrPut"),
            line,
        });
    }
    match crate::classes::buffer::fill(target, &bytes, line) {
        Ok(_) => Ok(Value::Int(bytes.len() as i64)),
        Err(_) => Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "StrPut"),
            line,
        }),
    }
}

/// The message of a failure, for a handler that has no room for the whole
/// of it.
fn the_message_of(failure: crate::runtime::RuntimeFailure) -> String {
    match failure {
        crate::runtime::RuntimeFailure::Error(error) => error.message,
        crate::runtime::RuntimeFailure::Detailed(error, _) => error.message,
        _ => "the runtime cannot answer that.".to_string(),
    }
}

/// `VarSetStrCapacity`: the room of the string of a variable.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, with the source of the
/// binary read beside the measurements:
///
/// - with a capacity the variable is emptied -- the buffer of it is made
///   again, and the text in it is the text of nothing -- and the answer is
///   the capacity of the variable, which is the one asked for when it is
///   greater than the one the variable had;
/// - with zero the buffer is freed and the text is emptied, and the answer
///   is the capacity the variable had: a text of the tree has no buffer to
///   give back, and the capacity of it stays the one of the size class;
/// - with nothing the variable is not touched, and the answer is the
///   capacity it has; a variable that holds a number or an object is
///   refused with the kind it holds, because a capacity is a capacity of a
///   text;
/// - with minus one the answer is the length of the text, and not the
///   capacity of it;
/// - a capacity that is negative and not minus one is an invalid
///   parameter, and so is one that can not be a number of characters.
pub(crate) fn var_set_str_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "VarSetStrCapacity"),
            line,
        });
    };
    let Value::Reference(reference) = value else {
        return Err(RuntimeError {
            message: crate::runtime::wants_a_reference(
                1,
                "VarSetStrCapacity",
                "a variable reference",
                value,
            ),
            line,
        });
    };
    let Some(capacity) = values.get(1) else {
        // The capacity that the variable has, and the refusal of a value
        // that is not a text at all.
        return match rt.read_reference(reference) {
            Some(Value::Text(_)) | None => Ok(Value::Int(rt.string_capacity(reference))),
            Some(other) => Err(RuntimeError {
                message: crate::runtime::expected_type(TypeName::String, other.type_name()),
                line,
            }),
        };
    };
    // The number is read the way the binary reads it: a value of any kind
    // is a number, and the text of it is parsed.
    let requested = token_to_int64(capacity);
    if requested == -1 {
        // The one capacity that answers the length of the text.
        let length = rt
            .read_reference(reference)
            .map(|value| value.to_text().chars().count() as i64)
            .unwrap_or(0);
        return Ok(Value::Int(length));
    }
    if !(0..=i64::MAX / 2).contains(&requested) {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "VarSetStrCapacity"),
            line,
        });
    }
    let current = rt.string_capacity(reference);
    let asked_for = rt.keeps_a_string_capacity(reference);
    // The variable is emptied by the call, whether the capacity grows or
    // not: the buffer that holds it is made again.
    rt.write_reference(value, Value::Text(String::new()));
    let capacity = if requested > 0 {
        rt.set_string_capacity(reference, requested.max(current));
        requested.max(current)
    } else if asked_for {
        // The free of the binary gives a buffer that was asked for back,
        // and leaves the simple allocation of a text alone.
        rt.forget_a_string_capacity(reference);
        0
    } else {
        current
    };
    Ok(Value::Int(capacity))
}

/// `SubStr`: the part of the text between two positions.
///
/// AutoHotkey counts positions from one, and a negative position counts
/// from the end: -1 is the last character.
pub(crate) fn sub_str(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // Measured on the binary of v2.0.19 on 2026-11-10: the text is cut in the
    // code units of UTF-16, so the second unit of a surrogate pair is a text of
    // its own -- `Ord(SubStr(Chr(0x1F600), 2))` is 0xDE00.
    let text = crate::text16::the_code_units_of(&text_of(values, 0));
    let start = number_of(values, 1) as i64;
    let from = start_index(start, text.len()).min(text.len());
    let sliced: Vec<u16> = match values.get(2).and_then(|value| value.to_number()) {
        Some(length) => {
            let length = length as i64;
            if length < 0 {
                // A negative length leaves that many units out at the end of
                // the text.
                let to = text.len().saturating_sub((-length) as usize).max(from);
                text[from..to].to_vec()
            } else {
                let to = (from + length as usize).min(text.len());
                text[from..to].to_vec()
            }
        }
        None => text[from..].to_vec(),
    };
    Ok(Value::Text(crate::text16::the_text_of(&sliced)))
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
pub(crate) fn trim(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
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
pub(crate) fn is_value(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    // The second parameter of v2 is the code page the text is read
    // with, and the text of this tree is always the text it holds.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the code page of the text is not carried yet.".into(),
            line,
        });
    }
    let value = values
        .first()
        .cloned()
        .unwrap_or(Value::Text(String::new()));
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
pub(crate) fn is_object(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Int(0));
    // Measured on the binary of v2.0.19 on 2026-10-31: everything that is
    // not a number and not a text is an object -- an array, a map, a class
    // and a reference too.
    let answer = !matches!(value, Value::Int(_) | Value::Float(_) | Value::Text(_));
    Ok(Value::Int(if answer { 1 } else { 0 }))
}

/// `Type`: the name of the type of the value.
pub(crate) fn type_of(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Int(0));
    let name = match value.type_name() {
        TypeName::Integer => "Integer",
        TypeName::Float => "Float",
        TypeName::String => "String",
        TypeName::Object => "Object",
        TypeName::VarRef => "VarRef",
        TypeName::Number => "Number",
        TypeName::Class => "Class",
        TypeName::ComValue => "ComValue",
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

/// `IsSet`: whether the variable of the caller has a value.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a variable that has no
/// value is zero and not a failure, a name that was never written is zero,
/// and an argument that is not a variable is refused with "IsSet requires a
/// variable.".
pub(crate) fn is_set(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match values.first() {
        // The runtime hands the argument over as a reference only when the
        // script wrote a variable: a reference of its own -- `&x` -- is a
        // value, and v2 refuses it.
        Some(Value::Reference(reference)) if reference.alias => {
            // Measured on the test suite of KeySharp (the corpus script
            // `op-null-assign.ahk`) and on the notes of v2.1-alpha: a variable
            // that holds the absent value, or the result of an access that the
            // mark of the v2.1 passed over, does not have a value, and `IsSet`
            // is zero for both of them.
            let set = match rt.read_reference(reference) {
                Some(Value::Unset) | Some(Value::Skipped) => false,
                Some(_) => true,
                None => false,
            };
            Ok(Value::Int(if set { 1 } else { 0 }))
        }
        _ => Err(RuntimeError {
            message: "IsSet requires a variable.".to_string(),
            line,
        }),
    }
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
        return !digits.is_empty()
            && digits
                .chars()
                .all(|character| character.is_ascii_hexdigit());
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
    let rest = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .unwrap_or(text);
    rest.chars().all(|character| character.is_ascii_hexdigit())
}

/// Whether the text holds nothing but the letters.
fn is_alpha(text: &str) -> bool {
    text.chars()
        .all(|character| character.is_ascii_alphabetic())
}

/// Whether the text holds nothing but the letters and the digits.
fn is_alnum(text: &str) -> bool {
    text.chars()
        .all(|character| character.is_ascii_alphanumeric())
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
    text.chars()
        .all(|character| matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}'))
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
/// there at all. The third parameter is the variable that the match is
/// written into, and the fourth is the place where the search begins.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, with the source of it
/// read beside the measurements:
///
/// - the match is an object of the class `RegExMatchInfo`, and a text that
///   does not match leaves the variable with the empty text in it -- set,
///   and not unset;
/// - the place counts from one, and a place that is zero or negative is a
///   place from the end of the text: `len + place`, and zero when that is
///   before the first character, so the place zero is the end of the text;
/// - a value that is not a variable reference is refused with "Parameter
///   #3 of RegExMatch requires a variable reference, but received a ...".
pub(crate) fn regex_match(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The output variable, when the caller gives one.
    let output = match values.get(2) {
        None => None,
        Some(value @ Value::Reference(_)) => Some(value.clone()),
        Some(value) => {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    3,
                    "RegExMatch",
                    "a variable reference",
                    value,
                ),
                line,
            })
        }
    };
    let text = regex_text(values, 0, "RegExMatch", line)?;
    let pattern = regex_text(values, 1, "RegExMatch", line)?;
    // The place where the search begins, the way `BIF_RegEx` reads it.
    let mut starting = 0usize;
    if let Some(value) = values.get(3) {
        if !is_numeric_value(value) {
            return Err(RuntimeError {
                message: crate::runtime::requires(4, "RegExMatch", TypeName::Number, value),
                line,
            });
        }
        let length = text.chars().count() as i64;
        let mut place = token_to_int64(value) as i32 as i64;
        if place <= 0 {
            place += length;
            if place < 0 {
                place = 0;
            }
        } else if place > length {
            place = length;
        } else {
            place -= 1;
        }
        starting = place as usize;
    }
    let regex = crate::regex::Regex::new(&pattern).map_err(|reason| RuntimeError {
        message: reason,
        line,
    })?;
    let found = regex
        .find_at(&text, starting)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    let Some(found) = found else {
        // The text that does not match leaves the variable with the empty
        // text in it: the variable is set, and it holds nothing.
        if let Some(output) = &output {
            rt.write_reference(output, Value::Text(String::new()));
        }
        return Ok(Value::Int(0));
    };
    let place = found.span().0 as i64 + 1;
    if let Some(output) = &output {
        rt.write_reference(
            output,
            crate::classes::regexmatchinfo::of(&text, &regex, &found),
        );
    }
    Ok(Value::Int(place))
}

/// The text of one pattern parameter of a `RegExMatch` or a `RegExReplace`.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: an object is refused
/// with the kind it got, and a value that is not an object is written as
/// its text.
fn regex_text(
    values: &[Value],
    index: usize,
    name: &str,
    line: u32,
) -> Result<String, RuntimeError> {
    match values.get(index) {
        None => Ok(String::new()),
        // Measured on the binary of v2.0.19 on 2026-11-10: the absent value is
        // no argument, and the text of no argument is the empty text.
        Some(Value::Unset) => Ok(String::new()),
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => Err(RuntimeError {
            message: crate::runtime::requires(index + 1, name, TypeName::String, value),
            line,
        }),
        Some(value) => Ok(value.to_text()),
    }
}

/// `RegExReplace`: the text with the matches of the pattern replaced.
///
/// The replacement may name the groups of the pattern with `$1` or `\1`.
/// v2 writes the number of the swaps into the variable of the caller, and
/// the case of the letters is the one of the options of the pattern: the
/// pattern begins with `(?i)` when the case is not to be looked at.
pub(crate) fn regex_replace(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = regex_text(values, 0, "RegExReplace", line)?;
    let pattern = regex_text(values, 1, "RegExReplace", line)?;
    let replacement = text_argument("RegExReplace", values, 2, line)?;
    let limit = number_of(values, 4) as i64;
    // The place where the search starts counts from one, the way
    // AutoHotkey counts the places.
    let starting = (number_of(values, 5) as i64).max(1) as usize - 1;
    let regex = crate::regex::Regex::new(&pattern).map_err(|reason| RuntimeError {
        message: reason,
        line,
    })?;
    let characters: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut at = starting.min(characters.len());
    out.extend(characters[..at].iter());
    let mut replaced = 0i64;
    while at <= characters.len() {
        let found = regex.find_at(&text, at).map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
        let Some(found) = found else {
            break;
        };
        let (start, end) = found.span();
        out.extend(characters[at..start].iter());
        out.push_str(&expand_replacement(&replacement, &regex, &found, &characters));
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
                    value,
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
fn expand_replacement(
    replacement: &str,
    regex: &crate::regex::Regex,
    found: &crate::regex::Match,
    characters: &[char],
) -> String {
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
                let count = tail
                    .chars()
                    .take_while(|character| character.is_ascii_digit())
                    .count();
                (&tail[..count], &tail[count..])
            }
        };
        match name.parse::<usize>() {
            Ok(group) => {
                out.push_str(&found.group(group, characters).unwrap_or_default());
                rest = after;
            }
            // A group that is named: `${year}`. Measured on the binary of
            // v2.0.19 on 2026-11-09: the braced name is read, and the name
            // that is not a group is left as the text it was written as.
            Err(_) => match regex.group_number(name) {
                Some(group) => {
                    out.push_str(&found.group(group, characters).unwrap_or_default());
                    rest = after;
                }
                None => {
                    out.push(marker);
                    rest = tail;
                }
            },
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
pub(crate) fn click(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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
            let next_is_a_number = words
                .get(at + 1)
                .map(|next| next.parse::<f64>().is_ok())
                .unwrap_or(false);
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
            // The fourth and the fifth button are the ones that the
            // desktop of the tree does not carry.
            "x1" => {
                return Err(RuntimeError {
                    message: "the fourth button is not carried yet.".into(),
                    line,
                });
            }
            "x2" => {
                return Err(RuntimeError {
                    message: "the fifth button is not carried yet.".into(),
                    line,
                });
            }
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
            other
                if other.starts_with("wheel")
                    || other == "wu"
                    || other == "wd"
                    || other == "wl"
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
            let (pointer_x, pointer_y) = rt
                .desktop()
                .mouse_position()
                .map_err(|e| desktop_failure(e, line))?;
            (pointer_x, pointer_y)
        }
    };
    let detail = rt
        .desktop()
        .click(x, y, &button, count)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail))
}

/// `MouseClick`: the button goes first, and the parameters are the ones of
/// the function of v2.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the button of a
/// `MouseClick` is the first parameter, and it is a text that is not
/// spread -- `MouseClick "R D"` is refused, and so is `MouseClick 10, 20`,
/// with the "10" in the `Extra`. A button that is not one of the words of
/// v2 is refused with "Parameter #1 of MouseClick is invalid." and the
/// value in the `Extra`, with the space at the end of it cut. A button
/// that is not there is the left one, and the space alone is not a button
/// either.
///
/// The wheel and the fourth and the fifth button are the ones that the
/// desktop of the tree does not carry, and the refusal says so.
pub(crate) fn mouse_click(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let spelled = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    // A button that is not there is the left one.
    let button = if spelled.is_empty() {
        "left".to_string()
    } else {
        spelled
    };
    let name = match button.to_lowercase().as_str() {
        "left" | "l" => "left",
        "right" | "r" => "right",
        "middle" | "m" => "middle",
        "x1" => {
            return Err(RuntimeError {
                message: "the fourth button is not carried yet.".into(),
                line,
            });
        }
        "x2" => {
            return Err(RuntimeError {
                message: "the fifth button is not carried yet.".into(),
                line,
            });
        }
        "wheelup" | "wu" | "wheeldown" | "wd" | "wheelleft" | "wl" | "wheelright" | "wr" => {
            return Err(RuntimeError {
                message: "the wheel is not carried yet.".into(),
                line,
            });
        }
        _ => {
            return Err(RuntimeError {
                message: "Parameter #1 of MouseClick is invalid.".into(),
                line,
            });
        }
    };
    // The place is the second and the third parameters, and the number of
    // the clicks is the fourth. The speed is the fifth, and the tree does
    // not carry it.
    let x = values
        .get(1)
        .and_then(|value| value.to_number())
        .map(|number| number as i64);
    let y = values
        .get(2)
        .and_then(|value| value.to_number())
        .map(|number| number as i64);
    let count = values
        .get(3)
        .and_then(|value| value.to_number())
        .unwrap_or(1.0)
        .max(1.0) as u32;
    // The pressing and the releasing of the button are the sixth parameter,
    // and the place relative to the pointer is the seventh.
    if values
        .get(5)
        .map(|value| !value.to_text().trim().is_empty())
        .unwrap_or(false)
    {
        return Err(RuntimeError {
            message: "the pressing of the button is not carried yet.".into(),
            line,
        });
    }
    if values
        .get(6)
        .map(|value| !value.to_text().trim().is_empty())
        .unwrap_or(false)
    {
        return Err(RuntimeError {
            message: "the place relative to the pointer is not carried yet.".into(),
            line,
        });
    }
    // A click with no place is a click where the pointer is.
    let (x, y) = match (x, y) {
        (Some(x), Some(y)) => (x, y),
        _ => {
            let (pointer_x, pointer_y) = rt
                .desktop()
                .mouse_position()
                .map_err(|e| desktop_failure(e, line))?;
            (pointer_x, pointer_y)
        }
    };
    let detail = rt
        .desktop()
        .click(x, y, name, count)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail))
}

/// `MouseClickDrag`: a click that drags.
///
/// The button goes down at the first place, the pointer moves to the
/// second one, and the button comes up there.
pub(crate) fn mouse_click_drag(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let words = words_of(values);
    let mut at = 0usize;
    let mut button = "left".to_string();
    if let Some(word) = words.first() {
        if let Some(name) = button_of(word) {
            button = name.to_string();
            at = 1;
        } else if word.parse::<f64>().is_err() {
            // The button comes first, and the tree carries the three
            // buttons of a mouse and not the fourth, the fifth or the
            // wheel.
            return Err(RuntimeError {
                message: format!(
                    "{word} is not a button of a mouse: the tree carries the left, the right and the middle one."
                ),
                line,
            });
        }
    }
    let mut numbers: Vec<i64> = Vec::new();
    for word in &words[at..] {
        match word.parse::<f64>() {
            Ok(number) => numbers.push(number as i64),
            Err(_) => {
                // The tree carries the two places of the drag and not the
                // speed or the relative mode.
                return Err(RuntimeError {
                    message: format!(
                        "{word} is not a part of a drag: the tree carries the two places and not the speed or the relative mode."
                    ),
                    line,
                });
            }
        }
    }
    if numbers.len() < 4 {
        return Err(RuntimeError {
            message: "a drag wants the two places: the one where the button goes down and the one where it comes up."
                .into(),
            line,
        });
    }
    let (x1, y1, x2, y2) = (numbers[0], numbers[1], numbers[2], numbers[3]);
    rt.desktop()
        .mouse_down(Some(x1), Some(y1), &button)
        .map_err(|e| desktop_failure(e, line))?;
    rt.desktop()
        .move_pointer(x2, y2)
        .map_err(|e| desktop_failure(e, line))?;
    rt.desktop()
        .mouse_up(None, None, &button)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// `SetWorkingDir`: the working directory of the script.
///
/// The name of the directory is a subfolder of the working directory when
/// it is not an absolute path, the way v2 says it.
pub(crate) fn set_working_dir(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let directory = text_of(values, 0);
    if directory.is_empty() {
        return Err(RuntimeError {
            message: "the name of the directory is missing.".into(),
            line,
        });
    }
    rt.desktop()
        .set_working_directory(&directory)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// `StrTitle`: a string in title case.
///
/// The rule is the one of the title case: the first letter of each word
/// is uppercased and the rest of the word is lowercased. A word is a run
/// of letters and of digits, so `GONE with the WIND` becomes `Gone With
/// The Wind`.
pub(crate) fn str_title(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(title_case(&text_of(values, 0))))
}

/// The text with the first letter of each word in the upper case and the
/// rest of it in the lower case.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: a word begins at the
/// start of the text and after the whitespace, and at nothing else --
/// `hello-world` becomes `Hello-world` and `ab-cd` becomes `Ab-cd` -- and the
/// first *letter* of the word is the one in the upper case: the digits in
/// front of it keep the word waiting, so `123abc` becomes `123Abc`, and a
/// word that has begun keeps the letters after its digits in the lower case,
/// so `abc123def` becomes `Abc123def`.
fn title_case(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut at_a_word = true;
    for character in text.chars() {
        if character.is_alphabetic() {
            if at_a_word {
                out.extend(character.to_uppercase());
                at_a_word = false;
            } else {
                out.extend(character.to_lowercase());
            }
        } else {
            out.push(character);
            // The whitespace begins a word, and the other characters leave the
            // word that has begun as it is and the one that has not begun
            // waiting for its first letter.
            if character.is_whitespace() {
                at_a_word = true;
            }
        }
    }
    out
}

/// `VerCompare`: two version strings, compared.
///
/// The rule is the one of `#Requires`: the numbers of the two versions
/// are compared one by one, and the component that is missing is a zero.
/// A version with a pre-release suffix is older than the same version
/// without it.
///
/// The second version may carry an operator -- `<`, `<=`, `>`, `>=` or
/// `=` -- and then the answer is whether the relation holds.
pub(crate) fn ver_compare(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = text_of(values, 0);
    let second = text_of(values, 1);
    if let Some((operator, wanted)) = split_operator(&second) {
        let order = compare_versions(&first, wanted.trim());
        let answer = match operator {
            "<" => order < 0,
            "<=" => order <= 0,
            ">" => order > 0,
            ">=" => order >= 0,
            _ => order == 0,
        };
        return Ok(Value::Int(answer as i64));
    }
    Ok(Value::Int(compare_versions(&first, second.trim()) as i64))
}

/// `IsLabel`: whether a label of the script is there.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the label is asked in
/// the scope that runs: the scope of the script, or the scope of the
/// function or of the method. The answer is one when the label is there
/// and zero when it is not, and the name of a label does not know its
/// case. A name that is empty is a name that is not there.
pub(crate) fn is_label(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    Ok(Value::Int(if rt.has_label(&name) { 1 } else { 0 }))
}

/// The words of the arguments, one by one.
///
/// v2 writes the parts of a command in the parameters and the parameters
/// may hold more than one of them.
fn words_of(values: &[Value]) -> Vec<String> {
    let mut words = Vec::new();
    for value in values {
        for word in value.to_text().split_whitespace() {
            words.push(word.to_string());
        }
    }
    words
}

/// The button that a word names, when it names one.
fn button_of(word: &str) -> Option<&'static str> {
    match word.to_lowercase().as_str() {
        "left" | "l" => Some("left"),
        "right" | "r" => Some("right"),
        "middle" | "m" => Some("middle"),
        _ => None,
    }
}

/// The operator of a comparison, when the second version carries one.
fn split_operator(version: &str) -> Option<(&'static str, &str)> {
    let trimmed = version.trim_start();
    for operator in ["<=", ">=", "<", ">", "="] {
        if let Some(rest) = trimmed.strip_prefix(operator) {
            return Some((operator, rest));
        }
    }
    None
}

/// The order of two version strings.
///
/// The numbers are compared one by one, and the component that is missing
/// is a zero. A version with a pre-release suffix is older than the same
/// version without one.
///
/// The known limitation of v2 is carried: a component that is bigger
/// than 2147483647 is the same as 2147483647.
fn compare_versions(first: &str, second: &str) -> i32 {
    let (first_numbers, first_pre) = split_version(first);
    let (second_numbers, second_pre) = split_version(second);
    let order = compare_numbers(&first_numbers, &second_numbers);
    if order != 0 {
        return order;
    }
    match (first_pre, second_pre) {
        (None, None) => 0,
        (Some(_), None) => -1,
        (None, Some(_)) => 1,
        (Some(first_pre), Some(second_pre)) => compare_pre(first_pre, second_pre),
    }
}

/// The numbers of a version, and its pre-release suffix.
fn split_version(version: &str) -> (Vec<u64>, Option<&str>) {
    let trimmed = version.trim();
    let (numbers, pre) = match trimmed.split_once('-') {
        Some((numbers, pre)) => (numbers, Some(pre)),
        None => (trimmed, None),
    };
    let numbers = numbers
        .split('.')
        .map(|part| part.trim().parse::<u64>().unwrap_or(0).min(2147483647))
        .collect();
    (numbers, pre)
}

/// The order of the numbers of two versions.
fn compare_numbers(first: &[u64], second: &[u64]) -> i32 {
    let count = first.len().max(second.len());
    for index in 0..count {
        let left = first.get(index).copied().unwrap_or(0);
        let right = second.get(index).copied().unwrap_or(0);
        if left != right {
            return if left < right { -1 } else { 1 };
        }
    }
    0
}

/// The order of two pre-release suffixes.
///
/// A suffix of numbers is compared as a number, and a suffix of text is
/// compared as text.
fn compare_pre(first: &str, second: &str) -> i32 {
    match (first.parse::<u64>(), second.parse::<u64>()) {
        (Ok(left), Ok(right)) => {
            if left == right {
                0
            } else if left < right {
                -1
            } else {
                1
            }
        }
        (Ok(_), Err(_)) => -1,
        (Err(_), Ok(_)) => 1,
        (Err(_), Err(_)) => match first.cmp(second) {
            core::cmp::Ordering::Less => -1,
            core::cmp::Ordering::Equal => 0,
            core::cmp::Ordering::Greater => 1,
        },
    }
}

/// `MouseGetPos`: where the pointer is.
///
/// v2 writes the place of the pointer into the variables of the caller,
/// and it answers with nothing. The window and the control are empty: the
/// box has no window to ask. The flag of v2 says how the control is to be
/// found, and there is no control to find.
pub(crate) fn mouse_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (x, y) = rt
        .desktop()
        .mouse_position()
        .map_err(|e| desktop_failure(e, line))?;
    let places = [
        Value::Int(x),
        Value::Int(y),
        Value::Text(String::new()),
        Value::Text(String::new()),
    ];
    for (index, place) in places.into_iter().enumerate() {
        let Some(value) = values.get(index) else {
            continue;
        };
        // The place is written into the variables of the caller, and
        // those are given by reference.
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 1,
                    "MouseGetPos",
                    "a VarRef",
                    value,
                ),
                line,
            });
        }
        rt.write_reference(value, place);
    }
    Ok(Value::Text(String::new()))
}

/// `MouseMove`: the pointer to a place on the screen.
pub(crate) fn mouse_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The relative place of the move is not carried: the box moves the
    // pointer to the place, and not from where it is.
    if values.len() > 3 {
        return Err(RuntimeError {
            message: "the relative place of the move is not carried yet.".into(),
            line,
        });
    }
    // The speed of the move is not carried either: the box moves the pointer
    // at its own speed. The speed of zero is the speed of nothing, and the
    // pointer goes to the place at once.
    if let Some(speed) = values.get(2) {
        if speed.to_number().unwrap_or(0.0) != 0.0 {
            return Err(RuntimeError {
                message: "the speed of the move is not carried yet.".into(),
                line,
            });
        }
    }
    let x = number_of(values, 0) as i64;
    let y = number_of(values, 1) as i64;
    let detail = rt
        .desktop()
        .move_pointer(x, y)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail))
}

/// `PixelGetColor`: the colour of one pixel.
pub(crate) fn pixel_get_color(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (x, y) = if values.len() >= 2 {
        (number_of(values, 0) as i64, number_of(values, 1) as i64)
    } else {
        let (pointer_x, pointer_y) = rt
            .desktop()
            .mouse_position()
            .map_err(|e| desktop_failure(e, line))?;
        (pointer_x, pointer_y)
    };
    let colour = rt
        .desktop()
        .pixel(x, y)
        .map_err(|e| desktop_failure(e, line))?;
    rt.set_global("pixel", Value::Text(colour.clone()));
    Ok(Value::Text(colour))
}

/// `PixelSearch`: where a colour is on the screen.
///
/// The place is written in the two variables of the caller and the answer
/// is one when the colour is there and zero when it is not, the way v2
/// does it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the command takes
/// seven parameters and the first two are references -- `PixelSearch
/// &OutputVarX, &OutputVarY, X1, Y1, X2, Y2, ColorID` -- and the
/// documentation of the v2.0.19 says the two variables are made blank
/// when the colour is not there.
pub(crate) fn pixel_search(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The place of the colour is written in the first two parameters, and
    // they are given by reference.
    for index in 0..2 {
        let Some(value) = values.get(index) else {
            continue;
        };
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 1,
                    "PixelSearch",
                    "a VarRef",
                    value,
                ),
                line,
            });
        }
    }
    let x1 = number_of(values, 2) as i64;
    let y1 = number_of(values, 3) as i64;
    let x2 = number_of(values, 4) as i64;
    let y2 = number_of(values, 5) as i64;
    let colour = text_of(values, 6);
    let found = rt
        .desktop()
        .pixel_search(x1, y1, x2, y2, &colour)
        .map_err(|e| desktop_failure(e, line))?;
    match found {
        Some((x, y)) => {
            rt.write_reference(&values[0], Value::Int(x));
            rt.write_reference(&values[1], Value::Int(y));
            Ok(Value::Int(1))
        }
        None => {
            // The place of v2 is blank when the colour is not there.
            rt.write_reference(&values[0], Value::Text(String::new()));
            rt.write_reference(&values[1], Value::Text(String::new()));
            Ok(Value::Int(0))
        }
    }
}

/// `Screenshot`: a picture of a part of the screen.
///
/// AutoHotkey v2 has no such built-in; this one is ours, and it is the
/// only name in the language that v2 would not have.
pub(crate) fn screenshot(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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
/// The windows of the session.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the window family of v2
/// asks the system about its windows, and a window of this runtime is named
/// by the id it is given here -- an identifier of this runtime, and not a
/// handle of Windows. A number that no window has is a window that is not
/// there.
fn the_windows_of(
    rt: &mut Runtime<'_>,
    line: u32,
) -> Result<Vec<crate::desktop::WindowInfo>, RuntimeError> {
    rt.desktop()
        .windows()
        .map_err(|e| desktop_failure(e, line))
}

/// Whether a title matches the text that is wanted, the way v2 matches it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a pattern of the `RegEx`
/// mode that does not compile is not a failure of the question about a
/// window -- the question answers that no window matches -- while a direct
/// `RegExMatch` of the same pattern is a failure. The error of the pattern is
/// therefore dropped here, and only here.
fn a_title_matches(
    mode: crate::desktop::TitleMatch,
    title: &str,
    wanted: &str,
) -> bool {
    mode.matches(title, wanted).unwrap_or(false)
}

/// The window a call of the family names, inside a listing of the windows.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a number is the id of a
/// window of this runtime, and a text is a title matched the way the script
/// matches it -- the mode of the settings. The empty text and the omitted one
/// ask about the active window, which is the way this tree has always asked.
fn the_window_in(
    windows: &[crate::desktop::WindowInfo],
    value: Option<&Value>,
) -> Result<Option<crate::desktop::WindowInfo>, RuntimeError> {
    match value {
        Some(Value::Int(id)) => Ok(windows.iter().find(|window| window.id == *id).cloned()),
        Some(value) if !value.to_text().is_empty() => {
            let wanted = value.to_text();
            let mode = crate::settings::title_match_mode();
            for window in windows {
                if a_title_matches(mode, &window.title, &wanted) {
                    return Ok(Some(window.clone()));
                }
            }
            Ok(None)
        }
        _ => Ok(windows.iter().find(|window| window.active).cloned()),
    }
}

/// The windows of a call that match the window it names.
///
/// The first parameter of the family is the window to ask about -- a number
/// is the id of one, a text is a title, and an omitted one is every window.
/// The other parameters of v2 are the text of the window and the two
/// exclusions, and this desktop has none of them.
fn the_matching_windows(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<Vec<crate::desktop::WindowInfo>, RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".to_string(),
            line,
        });
    }
    let windows = the_windows_of(rt, line)?;
    let Some(value) = values.first() else {
        return Ok(windows);
    };
    if let Value::Int(id) = value {
        return Ok(windows.into_iter().filter(|window| window.id == *id).collect());
    }
    let wanted = value.to_text();
    if wanted.is_empty() {
        return Ok(windows);
    }
    let mode = crate::settings::title_match_mode();
    let mut matched = Vec::new();
    for window in windows {
        if a_title_matches(mode, &window.title, &wanted) {
            matched.push(window);
        }
    }
    Ok(matched)
}

/// The failure of a window query that found nothing.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetID`, `WinGetIDLast`
/// and `WinGetClass` of a window that is not there answer "Target window not
/// found.", and the `Extra` of the failure is the title that was asked about.
fn no_window(line: u32) -> RuntimeError {
    RuntimeError {
        message: "Target window not found.".to_string(),
        line,
    }
}

/// `WinGetList`, `WinGetCount`, `WinGetID`, `WinGetIDLast` and `WinGetClass`:
/// what the session knows about its windows.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetList` answers an
/// array of the ids -- an empty one when nothing matches -- `WinGetCount`
/// answers the number of the windows, and the other three answer the id, or
/// the class, of the first (or the last) of them, and fail with "Target
/// window not found." when there is none.
pub(crate) fn window_query(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let windows = the_matching_windows(rt, values, line)?;
    match variant {
        1 => {
            let ids: Vec<Value> = windows
                .iter()
                .map(|window| Value::Int(window.id))
                .collect();
            crate::classes::array::construct(&ids, rt, line).map_err(|failure| RuntimeError {
                message: the_message_of(failure),
                line,
            })
        }
        2 => Ok(Value::Int(windows.len() as i64)),
        5 => match windows.into_iter().next() {
            Some(window) => Ok(Value::Text(window.class)),
            None => Err(no_window(line)),
        },
        3 => match windows.into_iter().next() {
            Some(window) => Ok(Value::Int(window.id)),
            None => Err(no_window(line)),
        },
        _ => match windows.into_iter().last() {
            Some(window) => Ok(Value::Int(window.id)),
            None => Err(no_window(line)),
        },
    }
}

pub(crate) fn window_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and the desktop of the box has none of them.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    // The listing names the window the way v2 answers it: measured on the
    // binary of v2.0.19 on 2026-11-09, `WinActive("A")` is the id of the
    // active window, and not one.
    match rt.desktop().windows() {
        Ok(windows) => {
            let window = the_window_in(&windows, values.first())?;
            return Ok(Value::Int(
                match window {
                    Some(window) if window.active => window.id,
                    _ => 0,
                },
            ));
        }
        Err(reason) => {
            // A window of the listing that is not there is the answer of the
            // desktop itself, and a window that is named by its id cannot be
            // looked up without the listing.
            if matches!(values.first(), Some(Value::Int(_))) {
                return Err(RuntimeError {
                    message: format!("WinActive cannot look a window up by its id: {reason}"),
                    line,
                });
            }
        }
    }
    let title = text_of(values, 0);
    let mode = crate::settings::title_match_mode();
    match rt
        .desktop()
        .window_active(&title, mode)
        .map_err(|e| desktop_failure(e, line))?
    {
        WindowAnswer::Active => Ok(Value::Int(1)),
        // A window that is not there is not the active one, and a window
        // that is there but is not in front is not the active one either.
        WindowAnswer::NotActive | WindowAnswer::Absent => Ok(Value::Int(0)),
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
pub(crate) fn window_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and the desktop of the box has none of them.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    // The answer of v2 is the id of the window that is there, and not one:
    // measured on the binary of v2.0.19 on 2026-11-09, `WinExist("A")` is the
    // id of the active window.
    match rt.desktop().windows() {
        Ok(windows) => {
            let window = the_window_in(&windows, values.first())?;
            return Ok(Value::Int(window.map(|window| window.id).unwrap_or(0)));
        }
        Err(reason) => {
            if matches!(values.first(), Some(Value::Int(_))) {
                return Err(RuntimeError {
                    message: format!("WinExist cannot look a window up by its id: {reason}"),
                    line,
                });
            }
        }
    }
    let title = text_of(values, 0);
    let mode = crate::settings::title_match_mode();
    match rt
        .desktop()
        .window_exists(&title, mode)
        .map_err(|e| desktop_failure(e, line))?
    {
        WindowAnswer::Active | WindowAnswer::NotActive => Ok(Value::Int(1)),
        // A window that is not there is not there: the answer is false,
        // and not a failure.
        WindowAnswer::Absent => Ok(Value::Int(0)),
        WindowAnswer::Unknown(reason) => Err(RuntimeError {
            message: format!("WinExist cannot answer: {reason}"),
            line,
        }),
    }
}

/// `WinGetTitle`: the title of the active window.
pub(crate) fn win_get_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and the desktop of the box has none of them.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    // Measured on the binary of v2.0.19 on 2026-11-09: the title of a window
    // that is not there is "Target window not found.", while the empty text
    // and the omitted one ask about the active window, which the tree has
    // always answered with the empty text when there is none.
    let asked = match values.first() {
        None => false,
        Some(value) => match value {
            Value::Int(_) => true,
            other => !other.to_text().is_empty(),
        },
    };
    if let Ok(windows) = rt.desktop().windows() {
        let window = the_window_in(&windows, values.first())?;
        let title = match window {
            Some(window) => window.title,
            None if asked => return Err(no_window(line)),
            None => String::new(),
        };
        // The pause of v2 follows a windowing function, and a delay of -1 is
        // no pause at all.
        let delay = crate::settings::the_win_delay();
        if delay > 0 {
            rt.sleep_ms(delay as u64);
        }
        return Ok(Value::Text(title));
    }
    if asked && matches!(values.first(), Some(Value::Int(_))) {
        let reason = rt.desktop().windows().err().unwrap_or_default();
        return Err(RuntimeError {
            message: format!("WinGetTitle cannot look a window up by its id: {reason}"),
            line,
        });
    }
    let title = rt
        .desktop()
        .active_window_title()
        .map_err(|e| desktop_failure(e, line))?;
    // The pause of v2 follows a windowing function, and a delay of -1 is no
    // pause at all.
    let delay = crate::settings::the_win_delay();
    if delay > 0 {
        rt.sleep_ms(delay as u64);
    }
    Ok(Value::Text(title.unwrap_or_default()))
}

// -- the processes ----------------------------------------------------------------

/// The process a call names: the text of it, and nothing when the call names
/// nothing.
///
/// The text is the pid or the name of a process, and the desktop reads it the
/// way the binary reads it: a text that is a number is a pid, and the rest is
/// a name. Nothing means the process of the script itself.
fn the_process_of(values: &[Value], index: usize) -> Option<String> {
    values.get(index).map(|value| value.to_text())
}

/// The timeout of a wait, in seconds, and nothing when the call gives none.
///
/// The timeout of the binary is a number of seconds and it may be a text that
/// holds one; a call that gives none waits forever.
fn the_timeout_of(
    values: &[Value],
    index: usize,
    name: &str,
    line: u32,
) -> Result<Option<f64>, RuntimeError> {
    if values.get(index).is_none() {
        return Ok(None);
    }
    Ok(Some(number_argument(values, index, line, name)?))
}

/// The failure of a process that is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `ProcessGetName` and
/// `ProcessGetPath` of a pid that is not there answer "Target process not
/// found.", of the class `TargetError`, and the name of the built-in is in
/// the `What`.
fn no_process(line: u32) -> RuntimeError {
    RuntimeError {
        message: "Target process not found.".to_string(),
        line,
    }
}

/// `ProcessExist`: whether a process is there, and the pid of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the pid of
/// the process, and zero when there is none; a call with no parameter asks
/// about the process of the script itself, and a pid that is not there is
/// zero and not a failure.
pub(crate) fn process_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let which = the_process_of(values, 0);
    let pid = rt
        .desktop()
        .process_exist(which.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(pid as i64))
}

/// `ProcessGetName`: the name of the executable of a process.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the name is the file name
/// of the executable -- "AutoHotkey64.exe", and not the path -- and a process
/// that is not there is the failure "Target process not found.".
pub(crate) fn process_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let which = the_process_of(values, 0);
    let answer = rt
        .desktop()
        .process_name(which.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(name) => Ok(Value::Text(name)),
        None => Err(no_process(line)),
    }
}

/// `ProcessGetPath`: the full path of the executable of a process.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the path is the whole
/// path of the executable, and a process that is not there is the failure
/// "Target process not found.".
pub(crate) fn process_get_path(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let which = the_process_of(values, 0);
    let answer = rt
        .desktop()
        .process_path(which.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(path) => Ok(Value::Text(path)),
        None => Err(no_process(line)),
    }
}

/// `ProcessGetParent`: the pid of the parent of a process.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the pid of
/// the parent, and zero for a process that is not there -- the parent of a
/// process that is not there is not a failure.
pub(crate) fn process_get_parent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let which = the_process_of(values, 0);
    let pid = rt
        .desktop()
        .process_parent(which.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(pid as i64))
}

/// `ProcessClose`: close a process, and answer the pid of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the pid
/// that was closed, and zero when the process was not there or could not be
/// closed -- the binary answers zero for a process it could not open, too.
pub(crate) fn process_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(which) = the_process_of(values, 0) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ProcessClose"),
            line,
        });
    };
    let pid = rt
        .desktop()
        .process_close(&which)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(pid as i64))
}

/// `ProcessWait`: wait for a process to be there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the pid of
/// the process when it appears, and zero at the timeout; a call with no
/// timeout waits forever.
pub(crate) fn process_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(which) = the_process_of(values, 0) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ProcessWait"),
            line,
        });
    };
    let timeout = the_timeout_of(values, 1, "ProcessWait", line)?;
    let pid = rt
        .desktop()
        .process_wait(&which, timeout)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(pid as i64))
}

/// `ProcessWaitClose`: wait for a process that is there to close.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is zero when
/// the process is gone -- a process that never existed is gone -- and the pid
/// of the process that is still there at the timeout.
pub(crate) fn process_wait_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(which) = the_process_of(values, 0) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ProcessWaitClose"),
            line,
        });
    };
    let timeout = the_timeout_of(values, 1, "ProcessWaitClose", line)?;
    let pid = rt
        .desktop()
        .process_wait_close(&which, timeout)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(pid as i64))
}

/// `ProcessSetPriority`: the priority of a process, and the pid of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the binary reads only the
/// first letter of the priority word -- "Bogus" is the below-normal class,
/// and a number is an invalid parameter -- the call answers the pid of the
/// process, and a process that is not there is the failure "Target process
/// not found.". A call with no process named is the process of the script
/// itself, and an empty text there is the script itself too.
pub(crate) fn process_set_priority(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let priority = values.first().map(|value| value.to_text()).unwrap_or_default();
    let letter = priority.chars().next().map(|letter| letter.to_ascii_uppercase());
    if !matches!(letter, Some('L' | 'B' | 'N' | 'A' | 'H' | 'R')) {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ProcessSetPriority"),
            line,
        });
    }
    let which = the_process_of(values, 1);
    let answer = rt
        .desktop()
        .process_set_priority(&priority, which.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(pid) => Ok(Value::Int(pid as i64)),
        None => Err(no_process(line)),
    }
}

// -- the screen and the machine --------------------------------------------------

/// The monitor a call of the monitor family names, and the monitors of the
/// desktop.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: an omitted number and the
/// number zero ask about the primary monitor; a number that is not a monitor
/// of the desktop is "Parameter #1 of <name> is invalid." with the number in
/// the `Extra`; a text that holds a number is a number, and the part of it
/// after the point is cut.
fn the_monitor_of(
    rt: &mut Runtime<'_>,
    values: &[Value],
    name: &str,
    line: u32,
) -> Result<(usize, Vec<crate::desktop::Monitor>), RuntimeError> {
    let wanted = match values.first() {
        None => 0,
        Some(value) => token_to_int64(value) as i32 as i64,
    };
    let monitors = rt.desktop().monitors().map_err(|e| desktop_failure(e, line))?;
    if wanted == 0 {
        let primary = rt
            .desktop()
            .primary_monitor()
            .map_err(|e| desktop_failure(e, line))?;
        if primary == 0 || primary > monitors.len() {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(1, name),
                line,
            });
        }
        return Ok((primary, monitors));
    }
    if wanted < 0 || wanted as usize > monitors.len() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
    }
    Ok((wanted as usize, monitors))
}

/// The monitor at a place, counted from one, when it is there.
fn the_monitor_at(
    monitors: &[crate::desktop::Monitor],
    index: usize,
) -> Option<crate::desktop::Monitor> {
    monitors.get(index.checked_sub(1)?).cloned()
}

/// `MonitorGet`: where a monitor is, and the number of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the rectangle of the
/// monitor goes into the variables that were given, the right and the bottom
/// are one past the last column and the last row, a call may give any number
/// of them -- the first one only, and the four -- and the answer is one when
/// the monitor is there.
pub(crate) fn monitor_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (index, monitors) = the_monitor_of(rt, values, "MonitorGet", line)?;
    let Some(monitor) = the_monitor_at(&monitors, index) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "MonitorGet"),
            line,
        });
    };
    for (place, number) in [
        (1, monitor.left),
        (2, monitor.top),
        (3, monitor.right),
        (4, monitor.bottom),
    ] {
        if let Some(value) = values.get(place) {
            rt.write_reference(value, Value::Int(number));
        }
    }
    Ok(Value::Int(1))
}

/// `MonitorGetCount`: how many monitors the desktop has.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the number
/// of the monitors, and a desktop with one screen answers one.
pub(crate) fn monitor_get_count(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let monitors = rt.desktop().monitors().map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(monitors.len() as i64))
}

/// `MonitorGetName`: the name of a monitor.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the name of the monitor
/// of the binary is "\\.\DISPLAY1", the name the system gives it; the name of
/// an output of a Wayland compositor is the name of the output. A desktop
/// that does not say the name of its monitors is refused with the reason.
pub(crate) fn monitor_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (index, monitors) = the_monitor_of(rt, values, "MonitorGetName", line)?;
    let Some(monitor) = the_monitor_at(&monitors, index) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "MonitorGetName"),
            line,
        });
    };
    if monitor.name.is_empty() {
        return Err(RuntimeError {
            message: "this desktop does not say the name of its monitors.".to_string(),
            line,
        });
    }
    Ok(Value::Text(monitor.name))
}

/// `MonitorGetPrimary`: the number of the primary monitor.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer counts from
/// one, and it is the number that a call of `MonitorGet` with no monitor
/// answers.
pub(crate) fn monitor_get_primary(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let index = rt
        .desktop()
        .primary_monitor()
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(index as i64))
}

/// `MonitorGetWorkArea`: the part of a monitor that the panels leave.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the shape is the one of
/// `MonitorGet`, and the work area of the monitor of the binary was the
/// screen less the bar of the taskbar -- 0, 0, 3840, 2112 for a screen of
/// 3840 by 2160. A desktop that does not say the work area of its monitors
/// is refused with the reason.
pub(crate) fn monitor_get_work_area(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (index, monitors) = the_monitor_of(rt, values, "MonitorGetWorkArea", line)?;
    if the_monitor_at(&monitors, index).is_none() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "MonitorGetWorkArea"),
            line,
        });
    }
    let Some((left, top, right, bottom)) = rt
        .desktop()
        .monitor_work_area(index)
        .map_err(|e| desktop_failure(e, line))?
    else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "MonitorGetWorkArea"),
            line,
        });
    };
    for (place, number) in [(1, left), (2, top), (3, right), (4, bottom)] {
        if let Some(value) = values.get(place) {
            rt.write_reference(value, Value::Int(number));
        }
    }
    Ok(Value::Int(1))
}

/// `SysGet`: a metric of the system.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, with the source of it
/// read beside the measurements: the number of `SysGet` is the number of a
/// metric of `GetSystemMetrics`, and the binary answers it as it is. The
/// numbers that are about the screen are the ones this desktop can answer --
/// 0 and 1 are the width and the height of the primary monitor, 16 the width
/// of a full screen, 76 to 79 the virtual screen, 80 the number of monitors
/// and 81 the one metric of the format of the displays. The other metrics are
/// the ones of the window manager of Windows, and the number of one of them
/// is refused with the reason; a number that is not a metric at all is zero,
/// which is what the binary answers for one.
pub(crate) fn sys_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "SysGet"),
            line,
        });
    };
    // The number is the one of `GetSystemMetrics`, read the way the binary
    // reads it: the part of a float after the point is cut.
    let metric = token_to_int64(value) as i32 as i64;
    let monitors = rt.desktop().monitors().map_err(|e| desktop_failure(e, line))?;
    if monitors.is_empty() && (0..=102).contains(&metric) && metric != 81 {
        return Err(RuntimeError {
            message: format!(
                "the compositor published no monitor, so the metric {metric} of SysGet is not \
                 known."
            ),
            line,
        });
    }
    let answer = match metric {
        0 | 16 => {
            let index = rt
                .desktop()
                .primary_monitor()
                .map_err(|e| desktop_failure(e, line))?;
            match the_monitor_at(&monitors, index) {
                Some(monitor) => monitor.width(),
                None => 0,
            }
        }
        1 => {
            let index = rt
                .desktop()
                .primary_monitor()
                .map_err(|e| desktop_failure(e, line))?;
            match the_monitor_at(&monitors, index) {
                Some(monitor) => monitor.height(),
                None => 0,
            }
        }
        // The virtual screen: the box around all of the monitors.
        76 => monitors.iter().map(|monitor| monitor.left).min().unwrap_or(0),
        77 => monitors.iter().map(|monitor| monitor.top).min().unwrap_or(0),
        78 => {
            let left = monitors.iter().map(|m| m.left).min().unwrap_or(0);
            let right = monitors.iter().map(|m| m.right).max().unwrap_or(0);
            right - left
        }
        79 => {
            let top = monitors.iter().map(|m| m.top).min().unwrap_or(0);
            let bottom = monitors.iter().map(|m| m.bottom).max().unwrap_or(0);
            bottom - top
        }
        80 => monitors.len() as i64,
        // The one metric of the format: every output of a compositor is the
        // same format as far as a script can see, and the binary answers 1
        // for the one screen of its machine.
        81 => 1,
        other if (0..=102).contains(&other) => {
            return Err(RuntimeError {
                message: format!(
                    "SysGet({other}) is a metric of the window manager of Windows, and this \
                     desktop does not have one."
                ),
                line,
            })
        }
        // A number that is not a metric of the system is zero, which is what
        // the binary answers for one: measured on it on 2026-11-09,
        // `SysGet(-1)` and `SysGet(1000)` are zero.
        _ => 0,
    };
    Ok(Value::Int(answer))
}

/// `SysGetIPAddresses`: the addresses of the machine.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is an array of
/// texts, one for each address of the machine, and the address of the
/// loopback is not one of them.
pub(crate) fn sys_get_ip_addresses(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let addresses = rt
        .desktop()
        .ip_addresses()
        .map_err(|e| desktop_failure(e, line))?;
    // The answer of v2 is an `Array`, and not a list of the runtime: a script
    // asks it for its `Length` and its items the way it asks an array.
    let items: Vec<Value> = addresses.into_iter().map(Value::Text).collect();
    crate::classes::array::construct(&items, rt, line).map_err(|failure| RuntimeError {
        message: the_message_of(failure),
        line,
    })
}

// -- the keyboard, the debugger, the trash and the drives -------------------------

/// `SetCapsLockState`, `SetNumLockState` and `SetScrollLockState`: a lock key
/// on, off, or left as it is.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, with `ConvertOnOffAlways`
/// read beside it:
///
/// - "On", and the text "1", turn the state on; "Off", and the text "0", turn
///   it off, without regard to the case of the word;
/// - "AlwaysOn" and "AlwaysOff" turn it on and off too, and they ask a
///   keyboard hook of Windows to keep it so: the tree carries no hook, and the
///   state is the one that was asked for and not a state forced from then on;
/// - nothing, the empty text, and any word that is not one of those leaves the
///   state alone, and it is not a failure: `SetCapsLockState("Bogus")` answers
///   the empty text and the state does not change;
/// - "-1" is not one of the words, and it does not toggle anything.
///
/// The answer of every form is the empty text.
pub(crate) fn set_lock_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let lock = match variant {
        1 => "CapsLock",
        2 => "NumLock",
        _ => "ScrollLock",
    };
    let wanted = match values.first() {
        None => None,
        Some(value) => {
            let text = value.to_text();
            let lowered = text.to_ascii_lowercase();
            match lowered.as_str() {
                "on" | "alwayson" => Some(true),
                "off" | "alwaysoff" => Some(false),
                // The numbers are the words "1" and "0", and they are read as
                // the texts they are: measured on the binary of v2.0.19 on
                // 2026-11-09, `SetCapsLockState(1)` turns the state on.
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            }
        }
    };
    if let Some(on) = wanted {
        rt.desktop()
            .set_lock(lock, on)
            .map_err(|e| desktop_failure(e, line))?;
    }
    Ok(Value::Text(String::new()))
}

/// `OutputDebug`: a text for the debugger of the machine.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the parameter is a text,
/// and the answer is the empty text. A machine whose debugger is the standard
/// error of the process writes the text there, and the report of the runner on
/// the standard output is not touched.
pub(crate) fn output_debug(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    rt.desktop()
        .output_debug(&text)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

/// `FileRecycle`: a file, a folder, or the files that a pattern matches.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the empty
/// text, a plain path that is not there is the failure "Failed", a pattern
/// that matches nothing is not a failure, and an empty path is "Parameter #1
/// of FileRecycle is invalid." with the empty text in the `Extra`.
pub(crate) fn file_recycle(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(value) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "FileRecycle"),
            line,
        });
    };
    let pattern = value.to_text();
    if pattern.is_empty() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "FileRecycle"),
            line,
        });
    }
    rt.desktop()
        .recycle(&pattern)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

/// `FileRecycleEmpty`: empty the trash of the machine.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the empty
/// text, and the drive that is named is the trash that is emptied.
pub(crate) fn file_recycle_empty(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let drive = values
        .first()
        .map(|value| value.to_text())
        .filter(|text| !text.is_empty());
    rt.desktop()
        .empty_trash(drive.as_deref())
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

/// The name of the reading of a drive that a variant asks for.
fn the_drive_reading(variant: u8) -> &'static str {
    match variant {
        1 => "DriveGetCapacity",
        2 => "DriveGetFilesystem",
        3 => "DriveGetLabel",
        4 => "DriveGetSerial",
        5 => "DriveGetSpaceFree",
        6 => "DriveGetStatus",
        _ => "DriveGetType",
    }
}

/// `DriveGetCapacity`, `DriveGetFilesystem`, `DriveGetLabel`, `DriveGetSerial`,
/// `DriveGetSpaceFree`, `DriveGetStatus` and `DriveGetType`: the readings of a
/// drive.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the free space and the
/// capacity are counted in megabytes, the serial is a number, the filesystem
/// and the label are texts, a drive that is not there is the empty text for
/// the type and "Invalid" for the state, and the other readings of a drive
/// that is not there are the failure of the machine.
///
/// A machine has no drive letters: a drive is the path a volume is mounted at,
/// and a text that names a drive the way Windows does is refused with the
/// reason.
pub(crate) fn drive_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = the_drive_reading(variant);
    let Some(value) = values.first() else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
    };
    let path = value.to_text();
    match variant {
        6 => Ok(Value::Text(
            match rt
                .desktop()
                .drive_status(&path)
                .map_err(|e| desktop_failure(e, line))?
            {
                Some(status) => status,
                None => "Invalid".to_string(),
            },
        )),
        7 => Ok(Value::Text(
            rt.desktop()
                .drive_type(&path)
                .map_err(|e| desktop_failure(e, line))?
                .unwrap_or_default(),
        )),
        _ => {
            let Some(drive) = rt
                .desktop()
                .drive(&path)
                .map_err(|e| desktop_failure(e, line))?
            else {
                return Err(RuntimeError {
                    message: format!("the drive of {path} is not there."),
                    line,
                });
            };
            Ok(match variant {
                1 => Value::Int(drive.capacity_megabytes),
                2 => Value::Text(drive.filesystem),
                3 => Value::Text(drive.label),
                4 => Value::Int(drive.serial),
                5 => Value::Int(drive.free_megabytes),
                _ => Value::Int(0),
            })
        }
    }
}

// -- the rest -------------------------------------------------------------------

/// `MsgBox`: a message for the person watching.
///
/// The box has no window to show it in, so the message goes into the
/// report instead. That is the box's answer, not a quiet `false`.
pub(crate) fn msg_box(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The box has no buttons, no title and no answer to give back.
    if values.len() > 1 {
        return Err(RuntimeError {
            message:
                "the options, the title and the variable that takes the answer are not carried yet."
                    .into(),
            line,
        });
    }
    let text = rt.interpolate(&text_of(values, 0), line)?;
    rt.push_output(text.clone());
    Ok(Value::Text(text))
}

/// `Run`: a command, and what it answered.
pub(crate) fn run(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The third parameter of v2 is the options of the window, and the
    // fourth is the variable that takes the pid of the process. The
    // desktop of the box has no options, and the reference operator is
    // not carried, so a call that asks for them fails with the reason.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the options of the window and the pid of the process are not carried yet."
                .into(),
            line,
        });
    }
    let command = text_of(values, 0);
    let working_dir = text_of(values, 1);
    let (code, stdout, stderr) = rt
        .desktop()
        .run(&command, &working_dir)
        .map_err(|e| desktop_failure(e, line))?;
    if !stdout.trim().is_empty() {
        rt.push_output(stdout.trim_end().to_string());
    }
    if !stderr.trim().is_empty() {
        rt.push_output(stderr.trim_end().to_string());
    }
    Ok(Value::Int(code as i64))
}

/// `Send`: text and keys, the way AutoHotkey writes them.
pub(crate) fn send(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
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
pub(crate) fn sleep(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let millis = number_of(values, 0).clamp(0.0, 60_000.0) as u64;
    rt.sleep_ms(millis);
    // An event that happened while the script slept is answered when it
    // wakes. `Sleep` is the one statement of this runtime that waits, so it
    // is the one place an event can be answered without a loop of its own.
    rt.the_events_are_answered(line)?;
    Ok(Value::Int(millis as i64))
}

// -- the clipboard ----------------------------------------------------------------

/// The gap between the reads of the clipboard while a wait goes on.
///
/// The read is a process of its own on the box, and a wait is not a
/// quarter of a second long.
const CLIP_POLL_MS: u64 = 50;

/// `ClipWait`: wait until the clipboard has data.
///
/// The timeout is in seconds and may be a fraction of one. A wait with no
/// timeout is for the time the script has left: the timeout of the run is
/// checked between the statements, and this wait is inside one.
pub(crate) fn clip_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The second parameter waits for the data of any kind, and the
    // clipboard of this runtime carries text only.
    if number_of(values, 1) != 0.0 {
        return Err(RuntimeError {
            message: "the clipboard of this runtime carries text only, and it cannot wait for data of any kind.".to_string(),
            line,
        });
    }
    let limit = if values.is_empty() {
        None
    } else {
        Some(number_of(values, 0) * 1000.0)
    };
    let started = rt.now_ms();
    loop {
        let text = rt
            .desktop()
            .clipboard_read()
            .map_err(|message| desktop_failure(message, line))?;
        if !text.is_empty() {
            return Ok(Value::Int(1));
        }
        let waited = rt.now_ms().saturating_sub(started) as f64;
        let left = match limit {
            Some(limit) => {
                if waited >= limit {
                    return Ok(Value::Int(0));
                }
                limit - waited
            }
            None => {
                let left = rt.remaining_ms();
                if left == 0 {
                    return Ok(Value::Int(0));
                }
                left as f64
            }
        };
        rt.sleep_ms((left as u64).clamp(1, CLIP_POLL_MS));
    }
}

// -- the pattern of `Format` -----------------------------------------------------

/// The largest width or precision a pattern may ask for.
///
/// The binary builds the spec in a buffer of `12 + MAX_INTEGER_LENGTH*2`
/// characters and refuses a spec that does not fit, so a width longer than a
/// few dozen digits is not one it takes. The same kind of limit keeps a
/// pattern from asking for a text of no end here.
const FORMAT_LIMIT: usize = 100_000;

/// The flags of a C specifier: `-`, `+`, the space, `#` and `0`.
#[derive(Clone, Copy, Default)]
struct FormatFlags {
    minus: bool,
    plus: bool,
    space: bool,
    hash: bool,
    zero: bool,
}

/// What a specifier converts its value to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FormatKind {
    /// `d`, `i`, `o`, `u`, `x` and `X`: a whole number.
    Int,
    /// `e`, `E`, `f`, `g`, `G`, `a` and `A`: a float.
    Float,
    /// `c` and `C`: one character.
    Char,
    /// `p`: an address, written as sixteen hex digits.
    Pointer,
    /// The default and `s`: the text of the value, with an optional custom
    /// case -- `U`, `L` or `T`.
    Str(Option<char>),
}

/// One C specifier, read out of a placeholder.
struct FormatSpec {
    flags: FormatFlags,
    width: usize,
    precision: Option<usize>,
    kind: FormatKind,
    /// The type character of the specifier, or `'s'` for the default.
    letter: char,
}

/// One argument of a placeholder, converted to what the specifier wants.
enum FormatValue {
    Int(i64),
    Float(f64),
    Text(String),
}

/// The number of a specifier, or the whole number a specifier takes.
///
/// The binary refuses a spec that asks for a width or a precision that does
/// not fit its own buffer, and a number too large for the limit of the tree
/// is refused the same way: the placeholder stays literal text.
fn parse_spec_number(chars: &[char]) -> Option<usize> {
    let mut value = 0usize;
    for character in chars {
        if !character.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(character.to_digit(10)? as usize)?;
        if value > FORMAT_LIMIT {
            return None;
        }
    }
    Some(value)
}

/// Read the spec of a placeholder, after the `:` of it.
///
/// The grammar is the one of the binary: the flags `-+0 #` in any order, a
/// width, a point with a precision, and one type character. A spec that does
/// not have that shape makes the whole placeholder literal text, which is
/// what v2 does -- `{:<6d}`, `{:b}` and `{:.}` are left as they were written.
fn parse_format_spec(chars: &[char], mut at: usize) -> Option<(FormatSpec, usize)> {
    let start = at;
    let mut spec = FormatSpec {
        flags: FormatFlags::default(),
        width: 0,
        precision: None,
        kind: FormatKind::Str(None),
        letter: 's',
    };
    while at < chars.len() {
        match chars[at] {
            '-' => spec.flags.minus = true,
            '+' => spec.flags.plus = true,
            ' ' => spec.flags.space = true,
            '#' => spec.flags.hash = true,
            '0' => spec.flags.zero = true,
            _ => break,
        }
        at += 1;
    }
    let width_start = at;
    while at < chars.len() && chars[at].is_ascii_digit() {
        at += 1;
    }
    if at > width_start {
        spec.width = parse_spec_number(&chars[width_start..at])?;
    }
    if at < chars.len() && chars[at] == '.' {
        at += 1;
        let precision_start = at;
        while at < chars.len() && chars[at].is_ascii_digit() {
            at += 1;
        }
        spec.precision = Some(parse_spec_number(&chars[precision_start..at])?);
    }
    if at - start + 4 >= 52 {
        return None;
    }
    let letter = *chars.get(at)?;
    match letter {
        'd' | 'i' | 'o' | 'u' | 'x' | 'X' => {
            spec.kind = FormatKind::Int;
            spec.letter = letter;
            at += 1;
        }
        'e' | 'E' | 'f' | 'g' | 'G' | 'a' | 'A' => {
            spec.kind = FormatKind::Float;
            spec.letter = letter;
            at += 1;
        }
        'c' | 'C' => {
            spec.kind = FormatKind::Char;
            spec.letter = letter;
            at += 1;
        }
        'p' => {
            spec.kind = FormatKind::Pointer;
            spec.letter = letter;
            at += 1;
        }
        _ => {
            if "ULlTt".contains(letter) {
                spec.kind = FormatKind::Str(Some(letter.to_ascii_uppercase()));
                at += 1;
            }
            if chars.get(at) == Some(&'s') {
                at += 1;
            }
            spec.letter = 's';
        }
    }
    Some((spec, at))
}

/// The value a placeholder formats, or the failure of a parameter that is
/// not the kind its specifier wants.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a specifier that takes a
/// text takes a number and a text, and an object is refused with the kind it
/// got; a specifier that takes a number takes a number and a text that holds
/// one, and anything else is refused in the same way. The number of the
/// parameter is the number of the call, the format string included.
fn format_argument(
    spec: &FormatSpec,
    value: &Value,
    parameter: usize,
    line: u32,
) -> Result<FormatValue, RuntimeError> {
    // Measured on the binary of v2.0.19 on 2026-11-10: an argument that is the
    // absent value -- the hole between two commas, or the keyword `unset` -- is
    // the empty text in a format, whatever the specifier wants.
    if matches!(value, Value::Unset) {
        return Ok(FormatValue::Text(String::new()));
    }
    match spec.kind {
        FormatKind::Str(_) => match value {
            Value::Int(number) => Ok(FormatValue::Text(number.to_string())),
            Value::Float(number) => Ok(FormatValue::Text(crate::float_text::the_text_of(*number))),
            Value::Text(text) => Ok(FormatValue::Text(text.clone())),
            other => Err(RuntimeError {
                message: crate::runtime::requires(parameter, "Format", TypeName::String, other),
                line,
            }),
        },
        FormatKind::Float => {
            if !is_numeric_value(value) {
                return Err(RuntimeError {
                    message: crate::runtime::requires(parameter, "Format", TypeName::Number, value),
                    line,
                });
            }
            Ok(FormatValue::Float(token_to_double(value)))
        }
        _ => {
            if !is_numeric_value(value) {
                return Err(RuntimeError {
                    message: crate::runtime::requires(parameter, "Format", TypeName::Number, value),
                    line,
                });
            }
            Ok(FormatValue::Int(token_to_int64(value)))
        }
    }
}

/// Put the arguments into a pattern.
///
/// The pattern is the one of the binary, measured on 2026-11-09: `{1}` is the
/// first argument and a placeholder with no number takes the next one; `{{`
/// and `}}` are one brace of their own; and a placeholder that names an
/// argument that is not there, or whose spec is not one the binary reads, is
/// left exactly as it was written.
fn format_pattern(
    pattern: &str,
    values: &[Value],
    line: u32,
) -> Result<String, RuntimeError> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::with_capacity(pattern.len() + 8);
    let mut lit = 0usize;
    let mut position = 0usize;
    let mut last_parameter = 0usize;
    loop {
        // The literal text before the next placeholder.
        let mut at = position;
        while at < chars.len() && chars[at] != '{' {
            at += 1;
        }
        if lit < at {
            out.extend(chars[lit..at].iter());
        }
        lit = at;
        if at >= chars.len() {
            break;
        }
        let mut cursor = at + 1;
        // `{{` and `}}` are a brace of their own; the other brace of the
        // pair is the one that is written.
        if cursor + 1 < chars.len()
            && (chars[cursor] == '{' || chars[cursor] == '}')
            && chars[cursor + 1] == '}'
        {
            out.push(chars[cursor]);
            cursor += 2;
            lit = cursor;
            position = cursor;
            continue;
        }
        // The number of the argument the placeholder names, or the one after
        // the last one that a placeholder named.
        let digits_start = cursor;
        while cursor < chars.len() && chars[cursor].is_ascii_digit() {
            cursor += 1;
        }
        let parameter = if cursor > digits_start {
            match parse_spec_number(&chars[digits_start..cursor]) {
                Some(number) => number,
                None => usize::MAX,
            }
        } else {
            last_parameter + 1
        };
        if parameter >= values.len() || parameter < 1 {
            // The placeholder is not one: it stays as it was written.
            position = cursor;
            continue;
        }
        // The spec, when the placeholder has one.
        let parsed = if cursor < chars.len() && chars[cursor] == ':' {
            parse_format_spec(&chars, cursor + 1)
        } else {
            Some((
                FormatSpec {
                    flags: FormatFlags::default(),
                    width: 0,
                    precision: None,
                    kind: FormatKind::Str(None),
                    letter: 's',
                },
                cursor,
            ))
        };
        let Some((spec, next)) = parsed else {
            position = cursor;
            continue;
        };
        // The kind of the value is checked before the closing brace, the way
        // the binary checks it: a parameter of the wrong kind is a failure,
        // and only a spec of the wrong shape is literal text.
        let argument = format_argument(&spec, &values[parameter], parameter + 1, line)?;
        if next >= chars.len() || chars[next] != '}' {
            position = next;
            continue;
        }
        out.push_str(&format_spec(&spec, &argument));
        last_parameter = parameter;
        lit = next + 1;
        position = next + 1;
    }
    Ok(out)
}

/// The text of one value in its specifier.
fn format_spec(spec: &FormatSpec, value: &FormatValue) -> String {
    match spec.kind {
        FormatKind::Str(custom) => {
            let mut text = match value {
                FormatValue::Text(text) => text.clone(),
                _ => String::new(),
            };
            if let Some(precision) = spec.precision {
                text = text.chars().take(precision).collect();
            }
            // The custom case is applied to the whole answer, padding and
            // all; the padding of a text is spaces or zeros, which the
            // cases leave alone.
            let fill = if spec.flags.zero && !spec.flags.minus {
                '0'
            } else {
                ' '
            };
            let text = pad_text(&text, spec.width, fill, spec.flags.minus);
            match custom {
                Some('U') => text.to_uppercase(),
                Some('L') => text.to_lowercase(),
                Some('T') => title_case(&text),
                _ => text,
            }
        }
        FormatKind::Int => format_integer(spec, match value {
            FormatValue::Int(number) => *number,
            _ => 0,
        }),
        FormatKind::Char => {
            let number = match value {
                FormatValue::Int(number) => *number,
                _ => 0,
            };
            let text = character_text(spec.letter, number);
            pad_text(&text, spec.width, ' ', spec.flags.minus)
        }
        FormatKind::Pointer => {
            let number = match value {
                FormatValue::Int(number) => *number,
                _ => 0,
            };
            let text = format!("{:016X}", number as u64);
            pad_text(&text, spec.width, ' ', spec.flags.minus)
        }
        FormatKind::Float => format_float(spec, match value {
            FormatValue::Float(number) => *number,
            _ => 0.0,
        }),
    }
}

/// The text of a value in a width, with the padding on the side it asks for.
fn pad_text(text: &str, width: usize, fill: char, on_the_right: bool) -> String {
    let length = text.chars().count();
    if length >= width {
        return text.to_string();
    }
    let mut out = String::with_capacity(width);
    if on_the_right {
        out.push_str(text);
        for _ in length..width {
            out.push(fill);
        }
    } else {
        for _ in length..width {
            out.push(fill);
        }
        out.push_str(text);
    }
    out
}

/// The text of a whole number in its specifier.
///
/// The flags and the width are the ones of the C library: the sign and the
/// `0x` of a hex number come before the zeros of the padding, a precision
/// makes the digits at least that many and takes the `0` flag away, and a
/// precision of zero with the number zero writes no digit at all.
fn format_integer(spec: &FormatSpec, number: i64) -> String {
    let unsigned = number as u64;
    let mut sign = String::new();
    let mut prefix = String::new();
    let mut digits: String;
    match spec.letter {
        'd' | 'i' => {
            if number < 0 {
                sign.push('-');
            } else if spec.flags.plus {
                sign.push('+');
            } else if spec.flags.space {
                sign.push(' ');
            }
            digits = number.unsigned_abs().to_string();
        }
        'u' => digits = unsigned.to_string(),
        'o' => {
            digits = format!("{unsigned:o}");
            if spec.flags.hash && !digits.starts_with('0') {
                prefix.push('0');
            }
        }
        'x' | 'X' => {
            digits = if spec.letter == 'X' {
                format!("{unsigned:X}")
            } else {
                format!("{unsigned:x}")
            };
            if spec.flags.hash && number != 0 {
                prefix.push_str(if spec.letter == 'X' { "0X" } else { "0x" });
            }
        }
        _ => digits = number.to_string(),
    }
    if let Some(precision) = spec.precision {
        if precision == 0 && number == 0 {
            digits.clear();
        } else if digits.chars().count() < precision {
            digits = format!(
                "{}{digits}",
                "0".repeat(precision - digits.chars().count())
            );
        }
    }
    let body = format!("{sign}{prefix}{digits}");
    let length = body.chars().count();
    if length >= spec.width {
        return body;
    }
    let count = spec.width - length;
    if spec.flags.minus {
        format!("{body}{}", " ".repeat(count))
    } else if spec.flags.zero && spec.precision.is_none() {
        format!("{sign}{prefix}{}{digits}", "0".repeat(count))
    } else {
        format!("{}{body}", " ".repeat(count))
    }
}

/// One character from the number a `%c` or `%C` got.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `%c` writes the low
/// sixteen bits of the number and `%C` the low eight. The binary writes the
/// UTF-16 unit as it is, and a surrogate is not a character the tree can
/// hold: it is written with the replacement character.
fn character_text(letter: char, number: i64) -> String {
    let code = if letter == 'C' {
        (number as u64 & 0xff) as u32
    } else {
        (number as u64 & 0xffff) as u32
    };
    match char::from_u32(code) {
        Some(character) => character.to_string(),
        None => "\u{fffd}".to_string(),
    }
}

/// The text of a float in its specifier.
///
/// The types are the ones of the C library of the binary: `f` is the plain
/// form, `e` the exponent form, `g` the shorter of the two, and `a` the
/// hexadecimal form. The two values that are not numbers are written the way
/// the library writes them -- "inf", "-inf", "nan(ind)" and "-nan(ind)".
fn format_float(spec: &FormatSpec, number: f64) -> String {
    if number.is_nan() {
        let text = if number.is_sign_negative() {
            "-nan(ind)"
        } else {
            "nan(ind)"
        };
        return pad_text(text, spec.width, ' ', spec.flags.minus);
    }
    if number.is_infinite() {
        let text = if number < 0.0 { "-inf" } else { "inf" };
        return pad_text(text, spec.width, ' ', spec.flags.minus);
    }
    let negative = number.is_sign_negative();
    let magnitude = number.abs();
    let mut sign = String::new();
    if negative {
        sign.push('-');
    } else if spec.flags.plus {
        sign.push('+');
    } else if spec.flags.space {
        sign.push(' ');
    }
    let letter = spec.letter.to_ascii_lowercase();
    let uppercase = spec.letter.is_ascii_uppercase();
    let body = match letter {
        'e' => exponent_text(magnitude, spec, uppercase),
        'g' => general_text(magnitude, spec, uppercase),
        'a' => hex_float_text(magnitude, spec, uppercase),
        _ => fixed_text(magnitude, spec),
    };
    let text = format!("{sign}{body}");
    let length = text.chars().count();
    if length >= spec.width {
        return text;
    }
    let count = spec.width - length;
    if spec.flags.minus {
        format!("{text}{}", " ".repeat(count))
    } else if spec.flags.zero {
        // The zeros of the padding go after the sign.
        format!("{sign}{}{body}", "0".repeat(count))
    } else {
        format!("{}{text}", " ".repeat(count))
    }
}

/// The plain form of a float: `%f`.
fn fixed_text(magnitude: f64, spec: &FormatSpec) -> String {
    let precision = spec.precision.unwrap_or(6);
    let mut text = format!("{magnitude:.precision$}");
    if spec.flags.hash && precision == 0 {
        text.push('.');
    }
    text
}

/// The exponent form of a float: `%e` and `%E`.
fn exponent_text(magnitude: f64, spec: &FormatSpec, uppercase: bool) -> String {
    let precision = spec.precision.unwrap_or(6);
    let text = format!("{magnitude:.precision$e}");
    let (mantissa, exponent) = text
        .split_once('e')
        .expect("the exponential form of a number has an exponent");
    let exponent: i32 = exponent.parse().expect("the exponent is a number");
    let mut mantissa = mantissa.to_string();
    if spec.flags.hash && precision == 0 {
        mantissa.push('.');
    }
    format!(
        "{mantissa}{}{}{:02}",
        if uppercase { 'E' } else { 'e' },
        if exponent < 0 { '-' } else { '+' },
        exponent.abs()
    )
}

/// The shorter of the two forms of a float: `%g` and `%G`.
///
/// The number is written with the precision as the number of the significant
/// digits; the exponent form is used when the decimal exponent is less than
/// -4 or not less than the precision, and the zeros at the end of the
/// fraction are dropped -- unless the `#` flag asks for them.
fn general_text(magnitude: f64, spec: &FormatSpec, uppercase: bool) -> String {
    let precision = spec.precision.unwrap_or(6);
    let precision = if precision == 0 { 1 } else { precision };
    if magnitude == 0.0 {
        return if spec.flags.hash {
            format!("{:.*}", precision - 1, 0.0)
        } else {
            "0".to_string()
        };
    }
    let scientific = format!("{:.*e}", precision - 1, magnitude);
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("the exponential form of a number has an exponent");
    let exponent: i32 = exponent.parse().expect("the exponent is a number");
    if exponent < -4 || exponent >= precision as i32 {
        let mut mantissa = mantissa.to_string();
        if !spec.flags.hash {
            strip_trailing_zeros(&mut mantissa);
        }
        return format!(
            "{mantissa}{}{}{:02}",
            if uppercase { 'E' } else { 'e' },
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        );
    }
    let decimals = (precision as i32 - 1 - exponent).max(0) as usize;
    let mut text = format!("{magnitude:.decimals$}");
    if !spec.flags.hash {
        strip_trailing_zeros(&mut text);
    }
    text
}

/// Drop the zeros at the end of the fraction of a text, and the point that
/// is left with nothing after it.
fn strip_trailing_zeros(text: &mut String) {
    if !text.contains('.') {
        return;
    }
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
}

/// The hexadecimal form of a float: `%a` and `%A`.
///
/// The leading digit, the thirteen hex digits of the mantissa and the
/// exponent are the ones of the machine, written the way the library of the
/// binary writes them: `0x1.8000000000000p+0`.
fn hex_float_text(magnitude: f64, spec: &FormatSpec, uppercase: bool) -> String {
    let bits = magnitude.to_bits();
    let exponent_field = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & 0x000f_ffff_ffff_ffff;
    let (mut leading, exponent) = if magnitude == 0.0 {
        // The zero of the library is written with the exponent zero, and
        // not with the one of the sentence of the subnormal numbers.
        (0u64, 0i64)
    } else if exponent_field == 0 {
        (0u64, -1022i64)
    } else {
        (1u64, exponent_field - 1023)
    };
    let asked = spec.precision.unwrap_or(13);
    let precision = asked.min(13);
    let shift = 52 - 4 * precision;
    let mut kept = if shift > 0 {
        (fraction + (1u64 << (shift - 1))) >> shift
    } else {
        fraction
    };
    if precision < 13 && kept == (1u64 << (4 * precision)) {
        leading += 1;
        kept = 0;
    }
    let mut digits = if precision == 0 {
        String::new()
    } else {
        format!("{kept:0width$x}", width = precision)
    };
    if asked > 13 {
        digits.push_str(&"0".repeat(asked - 13));
    }
    let mut text = format!("0x{leading}");
    if !digits.is_empty() {
        text.push('.');
        text.push_str(&digits);
    }
    text.push(if uppercase { 'P' } else { 'p' });
    text.push(if exponent < 0 { '-' } else { '+' });
    text.push_str(&format!("{}", exponent.abs()));
    if uppercase {
        text = text.to_uppercase();
    }
    text
}

// -- the parts of a `Send` --------------------------------------------------------

/// One part of what a `Send` types.
enum SendPart {
    /// Text, typed as it stands.
    Text(String),
    /// A key, with the modifiers held while it is pressed.
    Key {
        name: String,
        modifiers: Vec<String>,
    },
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
                if name.chars().count() == 1
                    && modifier_name(name.chars().next().unwrap()).is_some()
                {
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
                                parts.push(SendPart::Key {
                                    name: key.clone(),
                                    modifiers: modifiers.clone(),
                                });
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

// -- the functions the ks dialect adds --------------------------------------------
//
// The table of them is in `extensions.rs`, and the dialect of KeySharp is
// the one that carries it: the v2.0.19 does not see any of these names.

/// `ATan2(Y, X)`: the arc whose tangent is the quotient, in the quadrant
/// the signs name.
pub(crate) fn ks_atan2(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let y = number_argument(values, 0, line, "ATan2")?;
    let x = number_argument(values, 1, line, "ATan2")?;
    Ok(Value::Float(crate::runtime::atan2(y, x)))
}

/// `Sinh`, `Cosh` and `Tanh`: the hyperbolic sine, cosine and tangent.
pub(crate) fn ks_hyperbolic(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = match variant {
        0 => "Sinh",
        1 => "Cosh",
        _ => "Tanh",
    };
    let number = number_argument(values, 0, line, name)?;
    let result = match variant {
        0 => crate::runtime::sinh(number),
        1 => crate::runtime::cosh(number),
        _ => crate::runtime::tanh(number),
    };
    Ok(Value::Float(result))
}

/// `RandomSeed(Seed)`: reinitialize the random number generator with the
/// seed the script named, and answer nothing.
pub(crate) fn ks_random_seed(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let seed = number_argument(values, 0, line, "RandomSeed")?;
    rt.seed_the_random(seed as i64);
    Ok(Value::Text(String::new()))
}

/// `ReplaceLineEndings(Str, EndOfLine?)`: make every line ending of the
/// text the one the caller names.
///
/// The endings are the seven the documentation of KeySharp lists: the two
/// of Windows, the two of the classic Mac and the Unix, the form feed, the
/// next line of Unicode, and the two separators of Unicode.
pub(crate) fn ks_replace_line_endings(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let _ = line;
    let text = text_of(values, 0);
    let ending = if values.len() > 1 {
        text_of(values, 1)
    } else {
        "\n".to_string()
    };
    let mut out = String::new();
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        let is_ending = match character {
            '\r' => {
                // The two of Windows go together, and the pair is one
                // ending and not two.
                if characters.peek() == Some(&'\n') {
                    characters.next();
                }
                true
            }
            '\n' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}' => true,
            _ => false,
        };
        if is_ending {
            out.push_str(&ending);
        } else {
            out.push(character);
        }
    }
    Ok(Value::Text(out))
}

/// `FileFullPath(Filename)`: the full path to the file.
pub(crate) fn ks_file_full_path(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    rt.desktop()
        .full_path(&path)
        .map(Value::Text)
        .map_err(|reason| desktop_failure(reason, line))
}

/// `FileCreateTemp()`: make an empty temporary file, and answer where it
/// is.
pub(crate) fn ks_file_create_temp(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    rt.desktop()
        .create_temp_file()
        .map(Value::Text)
        .map_err(|reason| desktop_failure(reason, line))
}

/// `OutputDebugLine(Text)`: the same as `OutputDebug`, with the line break
/// at the end.
pub(crate) fn ks_output_debug_line(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    rt.desktop()
        .output_debug(&(text + "\n"))
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

/// `Collect()`: ask for the memory of the unreachable objects to be
/// reclaimed.
///
/// The objects of this runtime are reclaimed the moment the last reference
/// to them goes away, so there is nothing to ask for: the answer is the
/// empty text, the way an answer of no value is.
pub(crate) fn ks_collect(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(String::new()))
}

/// The row of a built-in of the dialect: the table of the v2.0.19 first,
/// and the rows the dialect adds after it.
pub(crate) fn the_builtin_in_the_dialect(
    dialect: &'static dyn crate::dialect::Dialect,
    wanted: &str,
) -> Option<&'static Builtin> {
    dialect
        .builtins()
        .iter()
        .find(|builtin| builtin.name.to_lowercase() == wanted)
        .or_else(|| dialect.the_builtin_of_the_extension(wanted))
}

/// The row of a built-in FUNCTION of the dialect: the table of the v2.0.19
/// first, and the rows the dialect adds after it.
pub(crate) fn the_function_in_the_dialect(
    dialect: &'static dyn crate::dialect::Dialect,
    wanted: &str,
) -> Option<&'static Builtin> {
    dialect
        .builtins()
        .iter()
        .find(|builtin| {
            builtin.kind == BuiltinKind::Function && builtin.name.to_lowercase() == wanted
        })
        .or_else(|| {
            dialect
                .the_builtin_of_the_extension(wanted)
                .filter(|builtin| builtin.kind == BuiltinKind::Function)
        })
}

/// `Throw(Value)`: throw the value, the way the `throw` statement does.
///
/// The documentation of KeySharp: the value is thrown, and the catch
/// that takes it holds it. The value waits for the caller of the
/// built-in, and the built-in fails the statement with it.
pub(crate) fn ks_throw(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Unset);
    rt.throw_the_value(value);
    Err(RuntimeError {
        message: "the value was thrown".to_string(),
        line,
    })
}

/// `Props(Value)`: the properties of a value, as an iterator.
///
/// The documentation of KeySharp: the iterator gives the name of a
/// property, and the value of a property that is a descriptor is the
/// value of the property itself. A value that is not an object has no
/// properties of its own, and the tree refuses it with the kind of it,
/// the way KeySharp refuses it.
pub(crate) fn ks_props(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = values.first().cloned().unwrap_or(Value::Unset);
    let Some(pairs) = rt.the_properties_of_a_value(&value) else {
        let kind = rt.the_type_of(&value);
        return Err(RuntimeError {
            message: format!(
                "Parameter #1 of Props requires an Object, but received a {kind}."
            ),
            line,
        });
    };
    Ok(crate::classes::enumerator::make(
        pairs,
        crate::classes::enumerator::Alone::TheKey,
    ))
}

/// `IsComponentAvailable(Capability)`: whether the unit of the dialect is
/// there.
///
/// The documentation of KeySharp: the unit is the `parser` or the
/// `compiler` of the first party, and the aliases `parsing` and
/// `compilation` are accepted. This tree has the parser -- it reads the
/// scripts -- and it has no compiler: it runs what it reads, and it never
/// turns a script into another language. A name that is not one of them
/// is refused, the way KeySharp refuses it.
pub(crate) fn ks_is_component_available(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0).to_lowercase();
    let answer = match name.as_str() {
        "parser" | "parsing" => 1,
        "compiler" | "compilation" => 0,
        _ => {
            return Err(RuntimeError {
                message: format!("Unknown capability \"{name}\"."),
                line,
            })
        }
    };
    Ok(Value::Int(answer))
}

/// `ValidateScript(Code)`: check a script without running it.
///
/// The documentation of KeySharp: the code is the path of a script file
/// when a file of that name is there, and the source of a script
/// otherwise; the answer is an object with `IsValid`, `Errors` and
/// `Warnings`, and the errors carry the file, the line and the column.
///
/// The tree has a parser, and it reads the script with the parser of the
/// dialect that runs: the script is checked, and it is not run.
pub(crate) fn ks_validate_script(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let code = text_of(values, 0);
    // A file of that name, when the machine has one, and the source
    // otherwise.
    let (source, file) = match rt.desktop().read_file(&code) {
        Ok(lines) => (lines.join("\n"), Some(code.clone())),
        Err(_) => (code.clone(), None),
    };
    let parsed = crate::parser::parse_with(&source, rt.dialect());
    let mut errors: Vec<Value> = Vec::new();
    let valid = match parsed {
        Ok(_) => 1,
        Err(failure) => {
            let place = match &file {
                Some(file) => format!("{file}:{}:{}", failure.line, failure.column),
                None => format!("{}:{}", failure.line, failure.column),
            };
            errors.push(Value::Text(format!("{place}: {}", failure.message)));
            0
        }
    };
    let object = crate::classes::object("object");
    let Value::Object(object) = object else {
        return Err(RuntimeError {
            message: "this runtime cannot make an object".to_string(),
            line: _line,
        });
    };
    object.borrow_mut().put("IsValid", Value::Int(valid));
    object.borrow_mut().put(
        "Errors",
        crate::classes::array::construct(&errors, rt, _line).map_err(|failure| RuntimeError {
            message: crate::runtime::the_message_of(&failure),
            line: _line,
        })?,
    );
    object
        .borrow_mut()
        .put("Warnings", crate::classes::array::construct(&[], rt, _line).map_err(
            |failure| RuntimeError {
                message: crate::runtime::the_message_of(&failure),
                line: _line,
            },
        )?);
    Ok(Value::Object(object))
}

/// `RequestCapabilities(Capabilities...)`: what the machine can do.
///
/// The documentation of KeySharp: the answer is an object with a property
/// for every capability -- `Granted`, `Denied`, `NotApplicable` or
/// `Unsupported` -- and an `IsGranted` that says whether every capability
/// that was asked for is granted.
///
/// The tree has no store of permissions: what it answers is what it can
/// do. The capability of the keyboard and of the pointer is the one of
/// the input, and the capability of the screen is the one of the capture:
/// both are there, and both are granted. The names that the tree does
/// not know are unsupported.
pub(crate) fn ks_request_capabilities(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let mut asked: Vec<String> = Vec::new();
    for value in values {
        for part in value
            .to_text()
            .split(|character: char| character == ',' || character.is_whitespace())
        {
            if !part.is_empty() {
                asked.push(part.to_lowercase());
            }
        }
    }
    if asked.is_empty() {
        asked.push("inputmonitoring".to_string());
        asked.push("screencapture".to_string());
    }
    let mut granted = true;
    let object = crate::classes::object("object");
    let Value::Object(object) = object else {
        return Err(RuntimeError {
            message: "this runtime cannot make an object".to_string(),
            line,
        });
    };
    for name in asked {
        let answer = match name.as_str() {
            "inputmonitoring" | "screencapture" => "Granted",
            _ => "Unsupported",
        };
        if answer != "Granted" {
            granted = false;
        }
        object
            .borrow_mut()
            .put(&name, Value::Text(answer.to_string()));
    }
    object
        .borrow_mut()
        .put("IsGranted", Value::Int(if granted { 1 } else { 0 }));
    let _ = rt;
    Ok(Value::Object(object))
}


// ---- control_get_text
/// `ControlGetText`: the text of a control of a window.
///
/// The control is found by its name and the window by its title, through the
/// accessibility layer, because that is the only description of the controls of
/// a window that a Wayland session keeps.
///
/// **A control that the application does not publish answers the empty text,
/// and that is not a failure.** Measured on the box on 2026-11-09 with a dump of
/// the whole tree: `zenity` publishes its dialog -- the `Name` of the window is
/// the title of the window, and it is a `dialog` -- and publishes no name and no
/// text for anything inside it. Any client of AT-SPI sees the same nothing. The
/// walk therefore answers what the tree holds, which for that program is the
/// empty text.
pub(crate) fn control_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The parameters after the control and the window are the text of the
    // window and the two exclusions, and they are not carried yet.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let control = text_of(values, 0);
    let title = text_of(values, 1);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window of a control is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    match rt
        .desktop()
        .control_text(&control, &title)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlAnswer::Found(text) => Ok(Value::Text(text)),
        crate::desktop::ControlAnswer::NoSuchWindow => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
        crate::desktop::ControlAnswer::NoSuchControl => Err(RuntimeError {
            message: TARGET_CONTROL.into(),
            line,
        }),
    }
}

// ---- control_flag
/// `ControlGetVisible` and `ControlGetEnabled`: a flag of a control of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09 against a window of its own
/// making: both answer the integer **1** for a control that is there and visible
/// or enabled, and the two failures are the two of `ControlGetText`. The variant
/// is the name -- this one is `ControlGetVisible` and the other is
/// `ControlGetEnabled`.
pub(crate) fn control_flag(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    // The parameters after the control and the window are the text of the
    // window and the two exclusions, and they are not carried yet.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let control = text_of(values, 0);
    let title = text_of(values, 1);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window of a control is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    let answer = if variant == 0 {
        rt.desktop().control_visible(&control, &title)
    } else {
        rt.desktop().control_enabled(&control, &title)
    }
    .map_err(|e| desktop_failure(e, line))?;
    match answer {
        crate::desktop::ControlFlag::Yes => Ok(Value::Int(1)),
        crate::desktop::ControlFlag::No => Ok(Value::Int(0)),
        crate::desktop::ControlFlag::NoSuchWindow => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
        crate::desktop::ControlFlag::NoSuchControl => Err(RuntimeError {
            message: TARGET_CONTROL.into(),
            line,
        }),
    }
}

// ---- control_set_text
/// `ControlSetText`: write a text into a control of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the **empty
/// text**, and the text of the control is the new one afterwards. The
/// parameters are the *new text first*, then the control, then the title --
/// which is the order of v2 and not the order of `ControlGetText`.
pub(crate) fn control_set_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The parameters after the three are the text of the window and the two
    // exclusions, and they are not carried yet.
    if values.len() > 3 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let control = text_of(values, 1);
    let title = text_of(values, 2);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window of a control is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    match rt
        .desktop()
        .control_set_text(&control, &title, &text)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlEdit::Done => Ok(Value::Text(String::new())),
        crate::desktop::ControlEdit::NoSuchWindow => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
        crate::desktop::ControlEdit::NoSuchControl => Err(RuntimeError {
            message: TARGET_CONTROL.into(),
            line,
        }),
    }
}

// ---- control_act
/// `ControlClick` and `ControlFocus`: an act on a named control of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: both answer the **empty
/// text**, and the two failures are the two of `ControlGetText`. The variant is
/// the name -- this one is `ControlClick` and the other is `ControlFocus`.
pub(crate) fn control_act(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    // The parameters after the control and the window are the button, the
    // count, the options, the text of the window and the two exclusions, and
    // they are not carried yet.
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the parameters after the control and the window are not carried yet.".into(),
            line,
        });
    }
    let control = text_of(values, 0);
    let title = text_of(values, 1);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window of a control is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    let answer = if variant == 0 {
        rt.desktop().control_click(&control, &title)
    } else {
        rt.desktop().control_focus(&control, &title)
    }
    .map_err(|e| desktop_failure(e, line))?;
    match answer {
        crate::desktop::ControlEdit::Done => Ok(Value::Text(String::new())),
        crate::desktop::ControlEdit::NoSuchWindow => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
        crate::desktop::ControlEdit::NoSuchControl => Err(RuntimeError {
            message: TARGET_CONTROL.into(),
            line,
        }),
    }
}

// ---- control_send_text
/// `ControlSendText`: type a text into a control of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the **empty
/// text**, and the control holds the text afterwards. The parameters are the
/// text first, then the control, then the title.
pub(crate) fn control_send_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 3 {
        return Err(RuntimeError {
            message: "the parameters after the text, the control and the window are not carried yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let control = text_of(values, 1);
    let title = text_of(values, 2);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window of a control is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    match rt
        .desktop()
        .control_send_text(&control, &title, &text)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlEdit::Done => Ok(Value::Text(String::new())),
        crate::desktop::ControlEdit::NoSuchWindow => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
        crate::desktop::ControlEdit::NoSuchControl => Err(RuntimeError {
            message: TARGET_CONTROL.into(),
            line,
        }),
    }
}

// ---- control_get_focus
/// `ControlGetFocus(Title?)`: the `ClassNN` of the control that has the focus.
///
/// Answered for a window of the script's own, which the helper made and therefore
/// knows the focus of; the focus of another program's window is not read, and says
/// so rather than answering a control that is not the one the script meant.
pub(crate) fn control_get_focus(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "the focus of a window of another program is not read by this runtime, and this script has no window of its own".to_string(),
            line,
        });
    }
    rt.desktop()
        .gui_focused()
        .map(Value::Text)
        .map_err(|reason| desktop_failure(reason, line))
}

// ---- control_get_hwnd
/// `ControlGetHwnd(Control)`: the handle of a control of the script's window.
///
/// The control is named by its `ClassNN`, and the answer is the number that names
/// it -- the same number an event about it carries.
pub(crate) fn control_get_hwnd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, 0);
    rt.desktop()
        .gui_number_named(&classnn)
        .map(Value::Int)
        .map_err(|reason| desktop_failure(reason, line))
}

// ---- control_visibility
pub(crate) fn control_visibility(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_a_control(rt, values, line, false)
}

// ---- control_show
pub(crate) fn control_show(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_a_control(rt, values, line, true)
}

// ---- control_get_classnn
/// `ControlGetClassNN(Control)`: the class and number that name a control.
///
/// Answered for a window of the script's own. The name is what v2 gives a control
/// of a window it made, and the lookup that already exists is what checks that the
/// control is really there.
pub(crate) fn control_get_classnn(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, 0);
    rt.desktop()
        .gui_number_named(&classnn)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(classnn))
}

// ---- control_set_enabled
pub(crate) fn control_set_enabled(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    the_state_of_a_control(rt, values, line, "enabled", on)
}

// ---- control_get_checked
pub(crate) fn control_get_checked(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_tick_of_a_control(rt, values, line, 0, None)
}

// ---- control_set_checked
pub(crate) fn control_set_checked(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    the_tick_of_a_control(rt, values, line, 1, Some(on))
}

// ---- control_move
/// `ControlMove(X, Y, W, H, Control, WinTitle)`: moves a control of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-11: the shape is
/// `ControlMove X, Y, W, H, Control, WinTitle` -- the **control is the
/// fifth parameter**, and a call that leaves it out is refused with
/// "Parameter #5 of ControlMove must not be omitted in this case."; the
/// first parameter is a number, and a text throws "Parameter #1 of
/// ControlMove requires a Number, but received a String."; and a control
/// or a window that is not there throws "Target window not found.".
///
/// **A call that names a control that IS there hangs the binary.** Measured
/// on 2026-11-11: `ControlMove("10", "20", "100", "30", "Edit1", "title")`
/// on a window of the script never came back, and the run was killed after
/// ten seconds. There is no answer of the binary to match, and this runtime
/// does not wait forever either -- so the call that names a control that is
/// there answers the reason of this tree.
pub(crate) fn control_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The control is the fifth parameter, and the binary wants it.
    if values.len() < 5 {
        return Err(RuntimeError {
            message: "Parameter #5 of ControlMove must not be omitted in this case."
                .to_string(),
            line,
        });
    }
    // The first parameter is the place, and it is a number: a text that
    // holds no number is refused, and a text that holds one is taken as it
    // is -- the binary answers "Target window not found." for a call whose
    // place is the text "10".
    number_argument(values, 0, line, "ControlMove")?;
    // The target: the window of the script by its title, and the control of
    // it by its ClassNN.
    let title = text_of(values, 5);
    the_window_of_the_script(rt, &title, line)?;
    let classnn = text_of(values, 4);
    match rt.desktop().gui_number_named(&classnn) {
        Ok(_) => Err(RuntimeError {
            message: "the binary of v2.0.19 hangs on this call -- measured on 2026-11-11 -- and this runtime does not wait forever: the window of this tree lays its controls out in order, so the place of a control cannot be set.".to_string(),
            line,
        }),
        // A control that is not there is the failure of the target.
        Err(_) => Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        }),
    }
}

// ---- control_send
/// `ControlSend(Keys, Control, WinTitle)`: send plain text into a control of the
/// script's own window.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: `ControlSend "abc", "Edit1"`
/// leaves `abc` in the edit, and `ControlSend "a{Enter}b"` leaves `ab` in a
/// single-line edit and `a` + a line break + `b` in a multi-line one. The tree
/// carries the text path and expands the keys that have a text effect -- Enter,
/// Tab and Backspace -- into that text. Keys that move a caret, press a modifier
/// or act on the window are refused with a reason rather than faked.
pub(crate) fn control_send(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let raw = rt.interpolate(&text_of(values, 0), line)?;
    if !raw.contains('{') && !raw.contains('}') {
        return control_send_text(rt, values, line, 0);
    }
    let mut text = String::new();
    for part in split_send(&raw) {
        match part {
            SendPart::Text(t) => text.push_str(&t),
            SendPart::Key { name, modifiers } => {
                if !modifiers.is_empty() {
                    return Err(RuntimeError {
                        message: format!(
                            "the {name} key with a modifier has no text to put into a control"
                        ),
                        line,
                    });
                }
                match name.to_ascii_lowercase().as_str() {
                    "enter" => text.push('\n'),
                    "tab" => text.push('\t'),
                    "backspace" | "bs" => {
                        text.pop();
                    }
                    other => {
                        return Err(RuntimeError {
                            message: format!(
                                "the {other} key has no text effect on a control, and this tree will not claim it did"
                            ),
                            line,
                        })
                    }
                }
            }
            SendPart::Down(name) | SendPart::Up(name) => {
                return Err(RuntimeError {
                    message: format!("holding the {name} key down has no text effect on a control"),
                    line,
                })
            }
        }
    }
    let mut sent = values.to_vec();
    if sent.is_empty() {
        sent.push(Value::Text(text));
    } else {
        sent[0] = Value::Text(text);
    }
    control_send_text(rt, &sent, line, 0)
}

// ---- control_get_pos
/// `ControlGetPos(&X, &Y, &W, &H, Control, WinTitle)`: the place and size of a
/// control of the script's own window, relative to its client area.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: with four VarRef outputs it
/// answers the four numbers (e.g. `[10,6,31,21]` for an Edit), and the answer
/// of the command is the empty text. **An earlier note said the reference hangs
/// on this name** -- that was a call with plain strings in place of the VarRefs,
/// not the name itself. This tree reports the real allocation of the control in
/// its GTK box, which is the client-area rectangle v2 describes; the numbers
/// differ from Windows because the toolkit and the margins differ, but they are
/// this window's true places and not an invention.
pub(crate) fn control_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    for index in 0..4 {
        let Some(value) = values.get(index) else {
            continue;
        };
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 1,
                    "ControlGetPos",
                    "a VarRef",
                    value,
                ),
                line,
            });
        }
    }
    let number = the_number_of_a_combo(rt, values, 4, line)?;
    let (x, y, width, height) = rt
        .desktop()
        .gui_control_pos(number)
        .map_err(|e| desktop_failure(e, line))?;
    let places = [x, y, width, height];
    for (index, place) in places.iter().enumerate() {
        if values.get(index).is_some() {
            rt.write_reference(&values[index], Value::Int(*place));
        }
    }
    Ok(Value::Text(String::new()))
}

// ---- control_get_items
/// `ControlGetItems(Control, WinTitle)`: the items of a list-shaped control.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: it answers an Array of the
/// texts, in order, and `ControlAddItem` makes it one longer.
pub(crate) fn control_get_items(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 0, line)?;
    let joined = rt
        .desktop()
        .gui_combo_items(number)
        .map_err(|e| desktop_failure(e, line))?;
    let answers: Vec<Value> = if joined.is_empty() {
        Vec::new()
    } else {
        joined
            .split('\n')
            .map(|item| Value::Text(item.to_string()))
            .collect()
    };
    match crate::classes::construct("array", &answers, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(answers)),
    }
}

// ---- control_add_item
/// `ControlAddItem(Text, Control, WinTitle)`: append an item. Measured on the
/// same day: the answer is nothing the script reads (the empty text).
pub(crate) fn control_add_item(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let text = text_of(values, 0);
    rt.desktop()
        .gui_combo_add(number, &text)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- control_choose_index
/// `ControlChooseIndex(N, Control, WinTitle)`: choose by a 1-based index.
/// Measured on the same day: the control's `Value` becomes that index.
pub(crate) fn control_choose_index(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let index = number_of(values, 0) as i64;
    rt.desktop()
        .gui_combo_select(number, &index.to_string(), false)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- control_choose_string
/// `ControlChooseString(Text, Control, WinTitle)`: choose by the item's text.
pub(crate) fn control_choose_string(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let text = text_of(values, 0);
    rt.desktop()
        .gui_combo_select(number, &text, true)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- control_get_choice
/// `ControlGetChoice(Control, WinTitle)`: the text of the chosen item, or the
/// empty text when nothing is chosen.
pub(crate) fn control_get_choice(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 0, line)?;
    let (_index, text) = rt
        .desktop()
        .gui_combo_choice(number)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(text))
}

// ---- control_get_index
/// `ControlGetIndex(Control, WinTitle)`: the 1-based index of the chosen item.
pub(crate) fn control_get_index(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 0, line)?;
    let (index, _text) = rt
        .desktop()
        .gui_combo_choice(number)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(index))
}

// ---- control_delete_item
/// `ControlDeleteItem(N, Control, WinTitle)`: remove an item by its 1-based
/// index. Measured on the same day: the array becomes one shorter.
pub(crate) fn control_delete_item(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let index = number_of(values, 0) as i64;
    rt.desktop()
        .gui_combo_delete(number, &index.to_string())
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- control_find_item
/// `ControlFindItem(Text, Control, WinTitle)`: the 1-based index of the first
/// item that is this text, or zero. Measured on the same day.
pub(crate) fn control_find_item(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let text = text_of(values, 0);
    rt.desktop()
        .gui_combo_find(number, &text)
        .map(Value::Int)
        .map_err(|e| desktop_failure(e, line))
}

// ---- control_show_drop_down
pub(crate) fn control_show_drop_down(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_drop_down_of_a_combo(rt, values, line, true)
}

// ---- control_hide_drop_down
pub(crate) fn control_hide_drop_down(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_drop_down_of_a_combo(rt, values, line, false)
}

// ---- the_state_of_a_control
/// `ControlSetEnabled(Value, Control)`: set the sensitivity of a control of the
/// script's window.
///
/// Round 35 refused this because the accessibility layer has no call to change
/// another program's control state -- true, and not this case: the control is the
/// helper's own, and `set_sensitive` is one call.
fn the_state_of_a_control(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    what: &str,
    on: bool,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, 1);
    rt.desktop()
        .gui_control_state(&classnn, what, on)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- the_tick_of_a_control
/// `ControlGetChecked(Control)` and `ControlSetChecked(Value, Control)`: whether a
/// check box of the script's window is ticked.
///
/// Round 35 refused this because the accessibility layer reports no `CHECKED` state
/// and a click toggles, which is true of another program's window and not of this
/// one: the check box is the helper's own and its state is one call.
fn the_tick_of_a_control(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    at: usize,
    setting: Option<bool>,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, at);
    let number = rt
        .desktop()
        .gui_number_named(&classnn)
        .map_err(|reason| desktop_failure(reason, line))? as u64;
    let checked = rt
        .desktop()
        .gui_checked(number, setting)
        .map_err(|reason| desktop_failure(reason, line))?;
    match setting {
        Some(_) => Ok(Value::Text(String::new())),
        None => Ok(Value::Int(checked)),
    }
}

// ---- the_visibility_of_a_control
/// `ControlHide(Control)` and `ControlShow(Control)`: hide or show a control of
/// the script's window, named by its `ClassNN`.
fn the_visibility_of_a_control(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    on: bool,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, 0);
    rt.desktop()
        .gui_visibility(&classnn, on)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- the_window_of_the_script
/// The window of the script, if the title the script named is that window.
///
/// The default match of v2 is that the title **contains** the text and the case
/// matters, and a window that does not match is not there: *"Target window not
/// found."* Shared by the names that act on the script's own window.
fn the_window_of_the_script(
    rt: &mut Runtime<'_>,
    target: &str,
    line: u32,
) -> Result<(), RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    let current = rt
        .desktop()
        .gui_title(0, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    if !target.is_empty() && !current.contains(target) {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    Ok(())
}

// ---- the_number_of_a_combo
/// The number of a list-shaped control of the script's own window, from its
/// `ClassNN`. The `Control*` family names a control by its class and number;
/// the helper knows each one by the number it gave it when it was added.
fn the_number_of_a_combo(
    rt: &mut Runtime<'_>,
    values: &[Value],
    control_index: usize,
    line: u32,
) -> Result<u64, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "this name asks about a window of another program, and this script has no window of its own".to_string(),
            line,
        });
    }
    let classnn = text_of(values, control_index);
    let number = rt
        .desktop()
        .gui_number_named(&classnn)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(number as u64)
}

// ---- the_drop_down_of_a_combo
/// `ControlShowDropDown` and `ControlHideDropDown`: open or close the pop-up of
/// a list-shaped control.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: both answer the empty text.
/// They have one handler each because the variant field of a row is the count's
/// answer/refusal flag and cannot carry which one this is.
fn the_drop_down_of_a_combo(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    show: bool,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 0, line)?;
    rt.desktop()
        .gui_combo_dropdown(number, show)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- TARGET_WINDOW
/// The failure v2 gives when a window is looked for and is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetID` and
/// `WinGetClass` fail with it, while `WinGetList` answers an empty array and
/// `WinGetCount` answers the zero.
const TARGET_WINDOW: &str = "Target window not found.";

// ---- TARGET_CONTROL
/// The failure of a question about a control, when the window is there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `ControlGetText("x", "A")`
/// -- a window that is there -- fails with this, and not with an empty text.
const TARGET_CONTROL: &str = "Target control not found.";


// ---- win_activate
pub(crate) fn win_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "activate")
}

// ---- win_activate_bottom
/// `WinActivateBottom`: activate the window at the bottom of the ones that
/// match.
///
/// Measured on the binary of v2.0.19 on 2026-11-10, with two windows of the
/// script matching one title: the BOTTOM one -- the first the script made --
/// is the one that ends up active, and the call answers the empty text. This
/// runtime makes one window of the script, so the bottom one that matches is
/// the one there, and the activation is the same call `WinActivate` makes. A
/// target that does not match the window of the script is "Target window not
/// found.", the wording `WinActivate` answers for the same thing.
pub(crate) fn win_activate_bottom(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "activate")
}

// ---- win_close
pub(crate) fn win_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_close_of_the_window(rt, values, line)
}

// ---- win_get_client_pos
/// `WinGetClientPos`: the same place, of the part of the window that the
/// program draws in.
///
/// A Wayland client does not draw its own frame -- the compositor does -- so
/// the geometry that the compositor reports is the window the user sees, and
/// it is the answer of both commands here. On Windows the two differ by the
/// frame: measured on the binary of v2.0.19 on 2026-11-09, the frame was
/// 819,479 1787x1262 and the client area 827,479 1771x1254.
pub(crate) fn win_get_client_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    window_geometry_into(rt, values, line, "WinGetClientPos")
}

// ---- win_get_controls
/// `WinGetControls(WinTitle)`: the `ClassNN` of every control of the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a window with a label, an edit and a
/// button answers three items, `Static1`, `Edit1` and `Button1`, and a window that is not
/// there answers *"Target window not found."*
pub(crate) fn win_get_controls(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    let joined = rt
        .desktop()
        .gui_controls()
        .map_err(|reason| desktop_failure(reason, line))?;
    let answers: Vec<Value> = if joined.is_empty() {
        Vec::new()
    } else {
        joined
            .split('\n')
            .map(|name| Value::Text(name.to_string()))
            .collect()
    };
    // The answer of v2 is an array, and this is how `StrSplit` makes one.
    match crate::classes::construct("array", &answers, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(answers)),
    }
}

// ---- win_get_controls_hwnd
/// `WinGetControlsHwnd(WinTitle)`: the handle of every control of the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers as many Integers as the
/// window has controls. No channel of its own is needed: the names come from
/// `WinGetControls` and the number of a name from the lookup of round 164.
pub(crate) fn win_get_controls_hwnd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    let joined = rt
        .desktop()
        .gui_controls()
        .map_err(|reason| desktop_failure(reason, line))?;
    let mut answers: Vec<Value> = Vec::new();
    if !joined.is_empty() {
        for name in joined.split('\n') {
            let number = rt
                .desktop()
                .gui_number_named(name)
                .map_err(|reason| desktop_failure(reason, line))?;
            answers.push(Value::Int(number));
        }
    }
    match crate::classes::construct("array", &answers, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(answers)),
    }
}

// ---- win_get_min_max
/// `WinGetMinMax`: whether the window is maximized, minimized, or neither.
///
/// The answer is the number of v2: `1`, `-1` or `0`. A window that is not
/// there is the failure of v2, measured on the binary of v2.0.19 on
/// 2026-11-09.
pub(crate) fn win_get_min_max(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (title, mode) = window_target(values, line)?;
    // The no-window call of v2 falls back to the last found window, and this
    // runtime keeps none: it is the same failure v2 gives with nothing to
    // fall back to.
    if title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        });
    }
    match rt
        .desktop()
        .window_min_max(&title, mode)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(answer) => Ok(Value::Int(answer)),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

// ---- win_get_pid
/// `WinGetPID`: the process of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the **Integer** of
/// the process, and a title that no window has is "Target window not found.".
///
/// The chain on this desktop is the accessibility bus: the registry names the
/// applications, a window is a child of one and its `Name` is its title, and the
/// bus answers the `PID` of a name -- measured on the box on 2026-11-09.
pub(crate) fn win_get_pid(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let title = text_of(values, 0);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    match rt
        .desktop()
        .window_pid(&title)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(pid) => Ok(Value::Int(pid)),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        }),
    }
}

// ---- win_get_pos
/// `WinGetPos`: where the window is and how big it is.
pub(crate) fn win_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    window_geometry_into(rt, values, line, "WinGetPos")
}

// ---- win_get_process
/// `WinGetProcessName` and `WinGetProcessPath`: the program of a window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the name is the file name of
/// the program (`AutoHotkey64.exe`) and the path is the whole of it, and a title
/// that no window has is "Target window not found.".
///
/// They ride the chain of `WinGetPID`: the window's title names an application on
/// the accessibility bus, the bus answers its process, and the process table
/// answers the name and the path -- which the `Processes` capability already
/// reads.
pub(crate) fn win_get_process(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    let title = text_of(values, 0);
    if title.is_empty() {
        return Err(RuntimeError {
            message: "the window is found by its title, and an empty title is not carried yet.".into(),
            line,
        });
    }
    let pid = match rt
        .desktop()
        .window_pid(&title)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(pid) => pid,
        None => {
            return Err(RuntimeError {
                message: TARGET_WINDOW.into(),
                line,
            })
        }
    };
    let number = pid.to_string();
    let answer = if variant == 0 {
        rt.desktop().process_name(Some(&number))
    } else {
        rt.desktop().process_path(Some(&number))
    }
    .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(text) => Ok(Value::Text(text)),
        None => Err(RuntimeError {
            message: "Target process not found.".into(),
            line,
        }),
    }
}

// ---- win_get_text
/// `WinGetText(WinTitle)`: the text of every control of the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a window with a label and an edit
/// answers the two texts, one after the other, **each followed by a line break**, and a
/// window that is not there answers *"Target window not found."*
///
/// **The script's own window and not another program's.** This name was measured and
/// refused in an earlier round for a foreign window, and that reason still holds: the
/// accessibility tree of another program carries its title bar with its content and no
/// rule separates them. Its own window has only the controls the script made.
pub(crate) fn win_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_all_text()
        .map(Value::Text)
        .map_err(|reason| desktop_failure(reason, line))
}

// ---- win_get_transparent
pub(crate) fn win_get_transparent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_opacity(None)
        .map(Value::Int)
        .map_err(|reason| desktop_failure(reason, line))
}

// ---- win_hide
pub(crate) fn win_hide(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_the_window(rt, values, line, false)
}

// ---- win_kill
pub(crate) fn win_kill(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_close_of_the_window(rt, values, line)
}

// ---- win_maximize
pub(crate) fn win_maximize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "maximize")
}

// ---- win_minimize
pub(crate) fn win_minimize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "minimize")
}

// ---- win_minimize_all
/// `WinMinimizeAll` and `WinMinimizeAllUndo`: minimize every window of the
/// session and undo it.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: both answer the empty
/// text. The effect reaches every window of the session, and the windows
/// this runtime can act on are the ones of the script: the wayland client
/// reads the listing of the compositor and carries no action that moves the
/// windows of other programs. The shape of the call is the contract here --
/// the same shape the binary answers -- and the effect is carried for the
/// window of the script alone.
pub(crate) fn win_minimize_all(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Ok(Value::Text(String::new()));
    }
    rt.desktop()
        .gui_option("minimize", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_minimize_all_undo
/// `WinMinimizeAllUndo`: put the minimized windows back.
pub(crate) fn win_minimize_all_undo(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Ok(Value::Text(String::new()));
    }
    rt.desktop()
        .gui_option("restore", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_move
/// `WinMove(X, Y, Width, Height, WinTitle)`: move the script's own window.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: moving the window the script
/// made works and answers nothing. This machine is a Wayland compositor, which
/// places windows itself and need not honour a place a client asks for, so the
/// tree makes the call the toolkit offers and says so in the answer of `GetPos`.
pub(crate) fn win_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let x = number_of(values, 0) as i64;
    let y = number_of(values, 1) as i64;
    let width = number_of(values, 2) as i64;
    let height = number_of(values, 3) as i64;
    let title = text_of(values, 4);
    the_window_of_the_script(rt, &title, line)?;
    rt.desktop()
        .window_move(x, y, width, height)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_move_bottom
/// `WinMoveBottom(WinTitle)`: send the window to the bottom of the order.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: it answers the empty
/// text, and a window that is not there answers "Target window not found.".
/// An earlier note said GTK removed the call that lowers a window -- that is
/// true of GTK 4, and the helper draws with GTK 3, which carries it. On
/// Wayland the compositor decides the order and may refuse the request; the
/// request is the part a program owns, and the call is made either way.
pub(crate) fn win_move_bottom(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "movebottom")
}

// ---- win_move_top
/// `WinMoveTop(WinTitle)`: bring the script's window to the front.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text, and a
/// window that is not there answers *"Target window not found."* It is the `activate`
/// word of the option channel: presenting a window raises it.
pub(crate) fn win_move_top(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "activate")
}

// ---- win_redraw
/// `WinRedraw(WinTitle)`: ask the script's window to draw itself again.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text, and a
/// window that is not there answers *"Target window not found."* It is the `redraw`
/// word of the option channel.
pub(crate) fn win_redraw(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "redraw")
}

// ---- win_restore
pub(crate) fn win_restore(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "restore")
}

// ---- win_set_always_on_top
/// `WinSetAlwaysOnTop(Value, WinTitle)`: keep a window of the script above the rest.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text, the
/// first parameter must be a **Number**, and a window that is not there answers
/// *"Target window not found."*
///
/// The script's own window again: what a compositor does with another program's
/// window is not a program's to set, and its own is the `Opt("+AlwaysOnTop")` this
/// tree already answers (round 132).
pub(crate) fn win_set_always_on_top(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    let target = text_of(values, 1);
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    let current = rt
        .desktop()
        .gui_title(0, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    if !target.is_empty() && !current.contains(&target) {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    rt.desktop()
        .gui_option("alwaysontop", on)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_set_enabled
/// `WinSetEnabled(Value, WinTitle)`: the sensitivity of the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text, and a
/// window that is not there answers *"Target window not found."* It goes through the
/// `disabled` word of the option channel, which is the same state the other way up.
pub(crate) fn win_set_enabled(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    let target = text_of(values, 1);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_option("disabled", !on)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_set_region
/// `WinSetRegion`: the shape of a window, cut out of it.
///
/// Measured on the binary of v2.0.19 on 2026-11-11: every target throws
/// "Target window not found." -- the window of the script included -- because
/// a region is a shape of Windows that this platform's windows do not have.
pub(crate) fn win_set_region(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "Target window not found.".to_string(),
        line,
    })
}

// ---- win_set_title
/// `WinSetTitle(NewTitle, WinTitle)`: rename a window of the script.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text after
/// renaming, and a window that is not there answers *"Target window not found."*
///
/// **The script's own window and not another program's**: renaming someone else's
/// window is not something a Wayland client can do, and the title of its own window
/// is the `Title` this tree already answers (round 71).
pub(crate) fn win_set_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let new_title = text_of(values, 0);
    let target = text_of(values, 1);
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    let current = rt
        .desktop()
        .gui_title(0, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    // The default match mode of v2 is that the title CONTAINS the text, and the
    // case matters; a window that does not match is not there.
    if !target.is_empty() && !current.contains(&target) {
        return Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        });
    }
    rt.desktop()
        .gui_title(0, Some(&new_title))
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_set_transparent
/// `WinSetTransparent(Value, WinTitle)` and `WinGetTransparent(WinTitle)`: the
/// opacity of the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the set answers the empty text,
/// the get answers the number that was set, `"Off"` means opaque, and a window that is
/// not there answers *"Target window not found."*
pub(crate) fn win_set_transparent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 1);
    the_window_of_the_script(rt, &target, line)?;
    let written = text_of(values, 0);
    // "Off" is what v2 means by opaque.
    let setting = if written.eq_ignore_ascii_case("off") {
        255
    } else {
        number_of(values, 0) as i64
    };
    rt.desktop()
        .gui_opacity(Some(setting))
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- win_show
pub(crate) fn win_show(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_the_window(rt, values, line, true)
}

// ---- win_style
/// `WinGetStyle`, `WinSetStyle`, `WinGetExStyle` and `WinSetExStyle`: **refused, with a
/// reason of their own.**
///
/// A window style is a number of Windows that says how a window is decorated and what
/// it may do -- a thick frame, a system menu, a title bar. Under this compositor **the
/// decoration belongs to the compositor and the client has no style number at all**:
/// there is nothing to read and nothing to set, and the words of v2's style bits have
/// no counterpart here.
pub(crate) fn win_style(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "a window style is a number of Windows that says how a window is decorated, and under this compositor the decoration belongs to the compositor and a client has no style number at all".to_string(),
        line,
    })
}

// ---- win_trans_color
/// `WinGetTransColor` and `WinSetTransColor`: **refused, with a reason of their own.**
///
/// A transparency COLOUR is a layered window of Windows: the colour is made invisible
/// while the rest of the window stays opaque. What this tree answers is an **opacity**
/// applied to the whole window, which is `WinSetTransparent` and `WinGetTransparent`
/// and is a different thing. Answering the colour would be answering a different
/// question than the one the script asked.
pub(crate) fn win_trans_color(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "a transparency colour is a layered window of Windows, where one colour is made invisible; this tree sets an opacity for the whole window, which is WinSetTransparent and WinGetTransparent".to_string(),
        line,
    })
}

// ---- win_wait
/// `WinWait`: wait for a window that matches to exist.
///
/// The answer of v2 is the identifier of the window, or the zero when the
/// timeout ran out -- measured on the binary of v2.0.19 on 2026-11-09. The
/// identifier of a Wayland window is a token, so it is text here.
pub(crate) fn win_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match window_wait_into(rt, values, line, crate::desktop::WaitFor::Present)? {
        Some(id) => Ok(Value::Text(id)),
        None => Ok(Value::Int(0)),
    }
}

// ---- win_wait_active
/// `WinWaitActive`: wait for a window that matches to have the focus.
pub(crate) fn win_wait_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match window_wait_into(rt, values, line, crate::desktop::WaitFor::Active)? {
        Some(id) => Ok(Value::Text(id)),
        None => Ok(Value::Int(0)),
    }
}

// ---- win_wait_close
/// `WinWaitClose`: wait for a matching window to be gone.
pub(crate) fn win_wait_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let met = window_wait_into(rt, values, line, crate::desktop::WaitFor::Close)?.is_some();
    Ok(Value::Int(if met { 1 } else { 0 }))
}

// ---- win_wait_not_active
/// `WinWaitNotActive`: wait for no matching window to have the focus.
///
/// v2 answers `1` and `0` for this one, and not the identifier -- measured on
/// the binary of v2.0.19 on 2026-11-09.
pub(crate) fn win_wait_not_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let met = window_wait_into(rt, values, line, crate::desktop::WaitFor::NotActive)?.is_some();
    Ok(Value::Int(if met { 1 } else { 0 }))
}

// ---- tooltip
/// `ToolTip(Text, X, Y)`: show a small window of text, and answer its number.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: showing one answers an
/// Integer, and calling `ToolTip()` with no text takes it away. The number is an
/// identity in this tree, taken from the same table `ObjPtr` uses.
pub(crate) fn tooltip(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    if text.is_empty() {
        rt.desktop()
            .tooltip_show("", 0, 0)
            .map_err(|e| desktop_failure(e, line))?;
        return Ok(Value::Text(String::new()));
    }
    let x = number_of(values, 1) as i64;
    let y = number_of(values, 2) as i64;
    let number = rt
        .desktop()
        .tooltip_show(&text, x, y)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(number))
}

// ---- window_target
/// The window that a `WinGet*` command asks about.
///
/// The parameters after the first are the text of the window and the two
/// exclusions, and the desktop of the box has none of them.
fn window_target(
    values: &[Value],
    line: u32,
) -> Result<(String, crate::desktop::TitleMatch), RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "the text of the window and the exclusions are not carried yet.".into(),
            line,
        });
    }
    Ok((text_of(values, 0), crate::settings::title_match_mode()))
}

// ---- window_geometry_into
/// The four places of a `WinGetPos`, written into the references of the call.
fn window_geometry_into(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
) -> Result<Value, RuntimeError> {
    // The four places of v2 are given by reference, and a value where a
    // variable is wanted is refused the way v2 refuses it.
    for index in 0..4 {
        let Some(value) = values.get(index) else {
            continue;
        };
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(index + 1, name, "a VarRef", value),
                line,
            });
        }
    }
    let title = text_of(values, 4);
    // v2 falls back to the last found window when the window is left out, and
    // this runtime keeps none: a call with no window is the failure that v2
    // gives when it has nothing to fall back to, measured on the binary of
    // v2.0.19 on 2026-11-09.
    if title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        });
    }
    let mode = crate::settings::title_match_mode();
    let geometry = rt
        .desktop()
        .window_geometry(&title, mode)
        .map_err(|e| desktop_failure(e, line))?;
    let Some(geometry) = geometry else {
        return Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        });
    };
    let places = [geometry.x, geometry.y, geometry.width, geometry.height];
    for (index, place) in places.iter().enumerate() {
        if values.get(index).is_some() {
            rt.write_reference(&values[index], Value::Int(*place));
        }
    }
    // The answer of v2 is the empty text.
    Ok(Value::Text(String::new()))
}

// ---- the_state_of_the_window
/// `WinActivate`, `WinMinimize`, `WinMaximize` and `WinRestore` of the script's
/// window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: all four answer the empty text,
/// and a window that is not there answers *"Target window not found."*
///
/// They go through the option channel of round 132, which already asks the helper to
/// act on the window, so none of them needs a method of its own.
fn the_state_of_the_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    state: &str,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_option(state, true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- the_close_of_the_window
/// `WinClose(WinTitle)` and `WinKill(WinTitle)`: close the script's window.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: both answer the empty text, and a
/// window that is not there answers *"Target window not found."*
///
/// **The script's own window and not another program's**: round 31 reverted a
/// `WinClose` because the compositor's toplevel listing publishes no close request,
/// and that is still true of a foreign window. Its own is the `close` word of the
/// option channel -- the helper made the window and can close it.
fn the_close_of_the_window(rt: &mut Runtime<'_>, values: &[Value], line: u32) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_option("close", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- the_visibility_of_the_window
/// `WinHide(WinTitle)` and `WinShow(WinTitle)`: take the script's window off the
/// screen and put it back.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: both answer the empty text, and
/// a window that is not there answers *"Target window not found."*
fn the_visibility_of_the_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    shown: bool,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    let answer = if shown {
        rt.desktop().gui_show(0)
    } else {
        rt.desktop().gui_hide()
    };
    answer.map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

// ---- window_wait_into
/// Wait for the condition of a `WinWait` family command.
fn window_wait_into(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    what: crate::desktop::WaitFor,
) -> Result<Option<String>, RuntimeError> {
    // The parameters after the seconds are the two exclusions, and they are
    // not carried yet.
    if values.len() > 3 {
        return Err(RuntimeError {
            message: "the two exclusions are not carried yet.".into(),
            line,
        });
    }
    let title = text_of(values, 0);
    let mode = crate::settings::title_match_mode();
    let timeout = window_wait_timeout(values);
    rt.desktop()
        .window_wait(&title, mode, what, timeout)
        .map_err(|e| desktop_failure(e, line))
}

// ---- window_wait_timeout
/// The seconds of a wait, in milliseconds.
///
/// The third parameter of v2 is the seconds. A call without it, or with a
/// number that is not positive, waits the cap.
fn window_wait_timeout(values: &[Value]) -> u64 {
    let seconds = number_of(values, 2);
    if seconds <= 0.0 {
        return MAX_WINDOW_WAIT_MS;
    }
    (seconds * 1000.0).min(MAX_WINDOW_WAIT_MS as f64) as u64
}

// ---- MAX_WINDOW_WAIT_MS
/// The longest a wait for a window may block.
///
/// It is the cap of `Sleep`, and it is there for the same reason: the runner's
/// timeout is checked between two statements, so a call that blocked forever
/// could not be stopped by it. The binary of v2.0.19 waits for a long time when
/// the seconds are left out of a `WinWait`, measured on 2026-11-09; this
/// runtime bounds it.
const MAX_WINDOW_WAIT_MS: u64 = 60_000;
