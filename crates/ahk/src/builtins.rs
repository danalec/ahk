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
pub(crate) use crate::dialogs::{dir_select, file_select, input_box, tray_tip};
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: block_input,
    },
    Builtin {
        name: "CallbackCreate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: callback_create,
    },
    Builtin {
        name: "CallbackFree",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 132,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "CaretGetPos",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 132,
        parameters: &[Parameter::Reference, Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComCall",
        kind: BuiltinKind::Function,
        min: 2,
        max: 255,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjActive",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjConnect",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjFlags",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjFromPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjGet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjQuery",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjType",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ComObjValue",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 124,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlAddItem",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlChooseIndex",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlChooseString",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlFindItem",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlGetItems",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlGetPos",
        kind: BuiltinKind::Command,
        min: 0,
        max: 9,
        variant: 100,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ControlMove",
        kind: BuiltinKind::Command,
        min: 0,
        max: 9,
        variant: 100,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetCapacity",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_capacity,
    },
    Builtin {
        name: "DriveGetFilesystem",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_filesystem,
    },
    Builtin {
        name: "DriveGetLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetList",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetSerial",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetSpaceFree",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_space_free,
    },
    Builtin {
        name: "DriveGetStatus",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_status,
    },
    Builtin {
        name: "DriveGetStatusCD",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveGetType",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_type,
    },
    Builtin {
        name: "DriveLock",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveRetract",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveSetLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 128,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "DriveUnlock",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 128,
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
        handler: edit,
    },
    Builtin {
        name: "EditGetCurrentCol",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_get_current_col,
    },
    Builtin {
        name: "EditGetCurrentLine",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_get_current_line,
    },
    Builtin {
        name: "EditGetLine",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_get_line,
    },
    Builtin {
        name: "EditGetLineCount",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_get_line_count,
    },
    Builtin {
        name: "EditGetSelectedText",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_get_selected_text,
    },
    Builtin {
        name: "EditPaste",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: edit_paste,
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
        variant: 132,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "FileGetVersion",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: file_get_version,
    },
    Builtin {
        name: "FileRecycle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 100,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: group_activate,
    },
    Builtin {
        name: "GroupAdd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: group_add,
    },
    Builtin {
        name: "GroupClose",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: group_close,
    },
    Builtin {
        name: "GroupDeactivate",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: group_close,
    },
    Builtin {
        name: "GuiCtrlFromHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 130,
        parameters: &[Parameter::Value],
        handler: from_hwnd,
    },
    Builtin {
        name: "GuiFromHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 130,
        parameters: &[Parameter::Value],
        handler: from_hwnd,
    },
    Builtin {
        name: "IL_Add",
        kind: BuiltinKind::Command,
        min: 2,
        max: 4,
        variant: 125,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "IL_Create",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 125,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "IL_Destroy",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 125,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "ListViewGetContent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 6,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "MenuFromHandle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 130,
        parameters: &[Parameter::Value],
        handler: from_hwnd,
    },
    Builtin {
        name: "MenuSelect",
        kind: BuiltinKind::Command,
        min: 0,
        max: 11,
        variant: 130,
        parameters: &[Parameter::Value],
        handler: from_hwnd,
    },
    Builtin {
        name: "MonitorGet",
        kind: BuiltinKind::Command,
        min: 0,
        max: 5,
        variant: 0,
        parameters: &[Parameter::Value],
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
        parameters: &[Parameter::Value],
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_from_ptr,
    },
    Builtin {
        name: "ObjFromPtrAddRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_from_ptr_add_ref,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_ptr,
    },
    Builtin {
        name: "ObjPtrAddRef",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: obj_ptr_add_ref,
    },
    Builtin {
        name: "ObjRelease",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[Parameter::Value],
        handler: obj_add_ref,
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
        handler: on_clipboard_change,
    },
    Builtin {
        name: "OnMessage",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: on_message,
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
        variant: 126,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: run_as,
    },
    Builtin {
        name: "SendMessage",
        kind: BuiltinKind::Command,
        min: 1,
        max: 9,
        variant: 126,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "SetCapsLockState",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 131,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 131,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 131,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        handler: shutdown,
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
        variant: 132,
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
        handler: sound_get_name,
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
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "StatusBarWait",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 127,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "StrGet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 132,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "StrPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 133,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 132,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "TrayTip",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: tray_tip,
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
        variant: 100,
        parameters: &[Parameter::Value],
        handler: all_windows,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_class,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_count,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_id,
    },
    Builtin {
        name: "WinGetIDLast",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_id_last,
    },
    Builtin {
        name: "WinGetList",
        kind: BuiltinKind::Command,
        min: 0,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_get_list,
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
        variant: 100,
        parameters: &[Parameter::Value],
        handler: all_windows,
    },
    Builtin {
        name: "WinMinimizeAllUndo",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 100,
        parameters: &[Parameter::Value],
        handler: all_windows,
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
        variant: 100,
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
        variant: 126,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: list_vars,
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
        variant: 110,
        parameters: &[Parameter::Value],
        handler: not_carried,
    },
    Builtin {
        name: "Persistent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: persistent,
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
        variant: 113,
        parameters: &[Parameter::Value],
        handler: not_carried,
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
        // The groups of the 2026-11-10 pass over the remaining refusals. Each
        // one says what the name is about, instead of the blanket claim that it
        // is "a name of Windows" -- which was false about most of them. The
        // numbers are 100 + the index, because `not_carried` subtracts REFUSAL.
        24 => {
            "COM objects belong to the component model of Windows, and this runtime has no COM."
        }
        25 => "image lists are the IL_* handles of a Win32 control, and this runtime makes none.",
        26 => {
            "these are messages of a Win32 window procedure, and this runtime has no such loop."
        }
        27 => {
            "these names act on a list-shaped control or a status bar, and this runtime makes no such control; the reference cannot be asked about them either."
        }
        28 => {
            "the label, the serial, the list and the locking of a drive are the drive tools of the machine, and this runtime does not carry those tools."
        }
        29 => {
            "a window group is a set a script names across statements, and this runtime has no such state between statements."
        }
        30 => {
            "these read a handle or the last window a call found; this runtime has neither a handle table nor a last-found window."
        }
        31 => {
            "the lock lights belong to the keyboard of the machine, and this runtime does not set them."
        }
        32 => {
            "the reference hangs or fails on this name, so there is no answer for this runtime to match."
        }
        33 => "a pointer to a string would be a value at a stable address, and the values of this runtime have none.",
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

/// `FileDelete`: take a file away.
pub(crate) fn file_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = crate::files::the_name_of_a_file(values, 0, 1, "FileDelete", line)?;
    // The wildcards of v2 are not carried: the name of a file that is not
    // there is not a file.
    crate::files::refuse_the_wildcards(&path, line)?;
    rt.desktop()
        .delete_file(&path)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
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
    ("MonitorGet", 1, Wants::Reference("a VarRef")),
    ("MonitorGet", 2, Wants::Reference("a VarRef")),
    ("MonitorGet", 3, Wants::Reference("a VarRef")),
    ("MonitorGet", 4, Wants::Reference("a VarRef")),
    ("MonitorGetName", 0, Wants::Declared(TypeName::Number)),
    ("MonitorGetWorkArea", 0, Wants::Declared(TypeName::Number)),
    ("MonitorGetWorkArea", 1, Wants::Reference("a VarRef")),
    ("MonitorGetWorkArea", 2, Wants::Reference("a VarRef")),
    ("MonitorGetWorkArea", 3, Wants::Reference("a VarRef")),
    ("MonitorGetWorkArea", 4, Wants::Reference("a VarRef")),
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

/// `ObjAddRef` and `ObjRelease`: the reference count of an object.
///
/// v2 counts the references of a COM object, and an object of a script has no
/// such count: the built-in says that the parameter is invalid. Measured on the
/// binary of v2.0.19 on 2026-11-09: `ObjAddRef(1)` and `ObjRelease(1)` answer
/// the same thing, "Parameter #1 of ObjAddRef is invalid." and "Parameter #1 of
/// ObjRelease is invalid.".
///
/// The variant is the name: this one is `ObjAddRef` and the other is
/// `ObjRelease`. They were inconsistent until round 23 -- `ObjAddRef` was
/// carried and `ObjRelease` refused as a name of Windows, which is not the
/// obstacle for either of them.
pub(crate) fn obj_add_ref(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let name = if variant == 0 {
        "ObjAddRef"
    } else {
        "ObjRelease"
    };
    Err(RuntimeError {
        message: crate::runtime::invalid_parameter(1, name),
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
/// Measured on the binary of v2.0.19 on 2026-10-31: the parameter wants a
/// boolean, and a boolean of AutoHotkey is a zero or a one. A number that
/// is another one, a float and a text are not values that the parameter can
/// take, and the name of the built-in says which one it is.
fn case_sense(values: &[Value], index: usize, name: &str, line: u32) -> Result<bool, RuntimeError> {
    match values.get(index) {
        Some(Value::Int(number)) if *number == 0 || *number == 1 => Ok(*number == 1),
        None => Ok(false),
        Some(_) => Err(RuntimeError {
            message: crate::runtime::invalid_parameter(index + 1, name),
            line,
        }),
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
        (Value::Float(left), Value::Float(right)) => {
            left.partial_cmp(right).unwrap_or(Ordering::Equal)
        }
        (Value::Int(left), Value::Float(right)) => {
            (*left as f64).partial_cmp(right).unwrap_or(Ordering::Equal)
        }
        (Value::Float(left), Value::Int(right)) => left
            .partial_cmp(&(*right as f64))
            .unwrap_or(Ordering::Equal),
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

/// `Round`: the number, with the digits the caller asks for.
pub(crate) fn round_to_digits(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "Round")?;
    let digits = number_argument(values, 1, line, "Round")? as i32;
    let factor = crate::runtime::powi(10.0, digits);
    Ok(Value::from_number(
        crate::runtime::round(number * factor) / factor,
    ))
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
    let Some(character) = char::from_u32(code as u32) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "Chr"),
            line,
        });
    };
    Ok(Value::Text(character.to_string()))
}

/// `Format`: a pattern with the values in it.
pub(crate) fn format(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(format_pattern(
        &text_of(values, 0),
        &values[1.min(values.len())..],
    )))
}

/// `InStr`: where the needle is in the haystack.
///
/// The position counts from one, and it is zero when the needle is not
/// there at all. The search may be limited to a part of the haystack, and
/// the case sense says whether the letters are taken as they are.
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
    // Measured on the binary of v2.0.19 on 2026-10-31: the sense of the case
    // is the third parameter, the place where the search begins is the
    // fourth, and the occurrence that the caller wants is the fifth. The end
    // of the search is not a parameter of v2.
    let case_sensitive = case_sense(values, 2, "InStr", line)?;
    let start = match values.get(3).and_then(|value| value.to_number()) {
        Some(start) => (start.max(1.0) as usize - 1).min(haystack.len()),
        None => 0,
    };
    let occurrence = match values.get(4) {
        None => 1usize,
        Some(value) => {
            let Some(number) = value.to_number() else {
                return Err(RuntimeError {
                    message: crate::runtime::requires(
                        5,
                        "InStr",
                        crate::runtime::TypeName::Number,
                        value,
                    ),
                    line,
                });
            };
            if number < 1.0 {
                return Err(RuntimeError {
                    message: crate::runtime::invalid_parameter(5, "InStr"),
                    line,
                });
            }
            number as usize
        }
    };
    let end = haystack.len();
    if end < start || end - start < needle.len() {
        return Ok(Value::Int(0));
    }
    let mut found = 0usize;
    for at in start..=(end - needle.len()) {
        let same = haystack[at..at + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(left, right)| same_letter(*left, *right, case_sensitive));
        if same {
            found += 1;
            if found == occurrence {
                return Ok(Value::Int(at as i64 + 1));
            }
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
    let code = text
        .chars()
        .next()
        .map(|character| character as i64)
        .unwrap_or(0);
    Ok(Value::Int(code))
}

/// `Sort`: the values in order.
///
/// The list is sorted where it stands. The numbers go by their value and
/// the text by its letters.
pub(crate) fn sort(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        // The options of AutoHotkey are not carried yet.
        return Err(RuntimeError {
            message: "the options of the sort are not carried yet.".into(),
            line,
        });
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
        _ => Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "Sort"),
            line,
        }),
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
    Ok(Value::Int(text_of(values, 0).chars().count() as i64))
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
    let case_sensitive = case_sense(values, 3, "StrReplace", line)?;
    let limit = number_of(values, 5) as i64;
    let mut out = String::with_capacity(haystack.len());
    let mut rest = haystack.as_str();
    let mut replaced = 0i64;
    // Measured on the binary of v2.0.19 on 2026-10-31: a needle that is
    // empty changes nothing, and the text comes back as it is.
    while !needle.is_empty() {
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
                    value,
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
    // A variable that has no value is a text that is empty, and the
    // capacity of it is nothing.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31: the v2 takes the
    // reference of a variable that has no value and answers the capacity
    // that it was given.
    let current = rt
        .read_reference(reference)
        .unwrap_or(Value::Text(String::new()));
    let Some(capacity) = values.get(1) else {
        return Ok(Value::Int(current.to_text().chars().count() as i64));
    };
    let capacity = match capacity.to_number_strict() {
        Ok(number) => number as i64,
        Err(kind) => {
            return Err(RuntimeError {
                message: crate::runtime::expected_type(TypeName::Integer, kind),
                line,
            });
        }
    };
    rt.write_reference(value, Value::Text(String::new()));
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
/// there at all. v2 writes the matches of the pattern into a variable of
/// the caller, and this tree does not carry that yet.
pub(crate) fn regex_match(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the matches of the pattern are not carried into a variable yet.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let pattern = text_of(values, 1);
    let regex = crate::regex::Regex::new(&pattern).map_err(|reason| RuntimeError {
        message: reason,
        line,
    })?;
    let found = regex.find(&text).map_err(|reason| RuntimeError {
        message: reason,
        line,
    })?;
    Ok(Value::Int(
        found.map(|found| found.span().0 as i64 + 1).unwrap_or(0),
    ))
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
    let text = text_argument("RegExReplace", values, 0, line)?;
    let pattern = text_argument("RegExReplace", values, 1, line)?;
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
    let text = text_of(values, 0);
    let mut out = String::with_capacity(text.len());
    let mut first = true;
    for character in text.chars() {
        if character.is_alphanumeric() {
            if first {
                out.extend(character.to_uppercase());
            } else {
                out.extend(character.to_lowercase());
            }
            first = false;
        } else {
            out.push(character);
            first = true;
        }
    }
    Ok(Value::Text(out))
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
pub(crate) fn compare_versions(first: &str, second: &str) -> i32 {
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
    // v2 asks for a window and for the text of it, and the box has only
    // the window that is in front.
    if !values.is_empty() {
        return Err(RuntimeError {
            message: "the window to ask about is not carried yet: only the active one is.".into(),
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

/// The failure v2 gives when a window is looked for and is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetID` and
/// `WinGetClass` fail with it, while `WinGetList` answers an empty array and
/// `WinGetCount` answers the zero.
const TARGET_WINDOW: &str = "Target window not found.";

/// The failure of a question about a control, when the window is there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `ControlGetText("x", "A")`
/// -- a window that is there -- fails with this, and not with an empty text.
const TARGET_CONTROL: &str = "Target control not found.";

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

/// `WinGetID`: the identifier of the first window that matches.
pub(crate) fn win_get_id(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (title, mode) = window_target(values, line)?;
    let windows = rt
        .desktop()
        .matching_windows(&title, mode)
        .map_err(|e| desktop_failure(e, line))?;
    match windows.first() {
        Some(window) => {
            rt.remember_the_last_window(&window.id);
            Ok(Value::Text(window.id.clone()))
        }
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

/// `WinGetIDLast()`: the identifier of the window a `Win*` call last found.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: after `WinGetID()` finds a
/// window, `WinGetIDLast()` answers that window's id, and with no `Win*` call
/// before it, it answers "Target window not found." (round 222). On this
/// platform a window id is the text the compositor gives, so that is what is
/// answered. `WinExist` cannot set it: its desktop call says only whether a
/// window is there, not which one, and the fake desktop of the language tests
/// carries that call alone.
pub(crate) fn win_get_id_last(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match rt.the_last_window() {
        Some(id) => Ok(Value::Text(id.to_string())),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

/// `WinGetClass`: the class of the first window that matches.
pub(crate) fn win_get_class(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (title, mode) = window_target(values, line)?;
    let windows = rt
        .desktop()
        .matching_windows(&title, mode)
        .map_err(|e| desktop_failure(e, line))?;
    match windows.first() {
        Some(window) => Ok(Value::Text(window.class.clone())),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

/// `WinGetList`: the identifiers of every window that matches.
///
/// A title that matches nothing is an empty array, and not a failure: measured
/// on the binary of v2.0.19 on 2026-11-09.
///
/// The answer is an array of v2 and not a bare list of values, because a
/// script reads the `Length` of it and the places of it, and a list carries
/// neither.
pub(crate) fn win_get_list(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (title, mode) = window_target(values, line)?;
    let windows = rt
        .desktop()
        .matching_windows(&title, mode)
        .map_err(|e| desktop_failure(e, line))?;
    let ids: Vec<Value> = windows
        .iter()
        .map(|window| Value::Text(window.id.clone()))
        .collect();
    match crate::classes::construct("array", &ids, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(ids)),
    }
}

/// `WinGetCount`: how many windows match.
pub(crate) fn win_get_count(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (title, mode) = window_target(values, line)?;
    let windows = rt
        .desktop()
        .matching_windows(&title, mode)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(windows.len() as i64))
}

/// `WinGetPos`: where the window is and how big it is.
pub(crate) fn win_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    window_geometry_into(rt, values, line, "WinGetPos")
}

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

/// `OutputDebug`: send a text to the debugger of the script.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the answer is the empty
/// text. There is no debugger to write to on this desktop, and this tree has one
/// channel for what a script says -- the `output` of the report, which is where
/// a `MsgBox` goes -- so the text goes there. A script that logs with
/// `OutputDebug` therefore shows up in the report, which is the thing a client
/// of the runner reads.
pub(crate) fn output_debug(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = rt.interpolate(&text_of(values, 0), line)?;
    rt.push_output(text);
    Ok(Value::Text(String::new()))
}

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

/// `StrPut`: how many bytes a text takes in an encoding, with its terminator.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `StrPut("hello")` is **12**
/// and `StrPut("hello", "UTF-16")` is 12 -- five characters of two bytes and the
/// two of the terminator -- and `StrPut("hello", "UTF-8")` is **6**, five and the
/// one of the terminator. `StrPut("héllo", "UTF-8")` is **7**, because the accent
/// is two bytes. An encoding that is not one of those is refused with "Invalid
/// Encoding.".
///
/// The form that **writes** the text takes a target address, and this runtime has
/// no pointers: that form is refused with that reason.
pub(crate) fn str_put(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the form of StrPut that writes the text takes an address, and this runtime has no pointers.".into(),
            line,
        });
    }
    let text = text_of(values, 0);
    let encoding = if values.len() > 1 {
        text_of(values, 1)
    } else {
        "CP0".to_string()
    };
    let bytes = match encoding.to_ascii_uppercase().as_str() {
        // The encoding of the machine is the UTF-16 of v2, and the terminator
        // is one unit of it.
        "CP0" | "UTF-16" => text.encode_utf16().count() * 2 + 2,
        "UTF-8" | "CP65001" => text.len() + 1,
        _ => {
            return Err(RuntimeError {
                message: "Invalid Encoding.".into(),
                line,
            })
        }
    };
    Ok(Value::Int(bytes as i64))
}

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
        rt.desktop().process_name(&number)
    } else {
        rt.desktop().process_path(&number)
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

/// `Persistent`: whether the script keeps running after its auto-execute ends.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the Integer **0**.
/// A script of this runtime is persistent by its nature -- the run is one script
/// from its first line to its last, and there is no message loop for it to stay
/// for -- so the answer is the zero v2 gives and the setting changes nothing.
pub(crate) fn persistent(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(0))
}

/// `ListVars`: show the variables of the script.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the **empty
/// text**, and it does not block. A window of v2 shows the variables; this
/// runtime has one channel for what a script shows -- the `output` of the
/// report, which is where a `MsgBox` goes -- so a line for each variable goes
/// there, and the answer is the empty text.
///
/// **The names are the keys the runtime keeps**, which are the names of the
/// script without their case. A script that writes `MyVar` is listed as `myvar`,
/// and that is a difference from v2 that is written down rather than hidden.
pub(crate) fn list_vars(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    for (name, value) in rt.variable_names() {
        rt.push_output(format!("{name}[{value}]"));
    }
    Ok(Value::Text(String::new()))
}

/// `SysGetIPAddresses`: the IPv4 addresses of the interfaces of the machine.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers an **array** of
/// the addresses as text, in the order the machine lists them.
pub(crate) fn sys_get_ip_addresses(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let addresses = rt.desktop().ip_addresses().map_err(|e| desktop_failure(e, line))?;
    let values: Vec<Value> = addresses.into_iter().map(Value::Text).collect();
    // The answer of v2 is an array.
    match crate::classes::construct("array", &values, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(values)),
    }
}

// -- the monitors ------------------------------------------------------------------
/// The monitor that a number names, or the failure of v2 for a number that
/// names none.
///
/// The monitors of v2 are numbered from one. Measured on the binary of
/// v2.0.19 on 2026-11-09: `MonitorGetName(9)` fails with "Parameter #1 of
/// MonitorGetName is invalid.".
fn the_monitor_at(
    rt: &mut Runtime<'_>,
    number: i64,
    line: u32,
    name: &str,
) -> Result<crate::desktop::Monitor, RuntimeError> {
    let monitors = rt
        .desktop()
        .monitors()
        .map_err(|e| desktop_failure(e, line))?;
    if number < 1 || number as usize > monitors.len() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
    }
    Ok(monitors[number as usize - 1].clone())
}

/// The number of the monitor that a call asks about, and one when it asks about
/// none.
fn the_monitor_number(values: &[Value]) -> i64 {
    match values.first() {
        None => 1,
        Some(value) => value.to_number().unwrap_or(1.0) as i64,
    }
}

/// `MonitorGetCount`: how many monitors the desktop has.
pub(crate) fn monitor_get_count(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let monitors = rt
        .desktop()
        .monitors()
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(monitors.len() as i64))
}

/// `MonitorGetName`: the name of a monitor.
pub(crate) fn monitor_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_monitor_number(values);
    let monitor = the_monitor_at(rt, number, line, "MonitorGetName")?;
    Ok(Value::Text(monitor.name))
}

/// `MonitorGetPrimary`: the number of the primary monitor.
///
/// A Wayland compositor lists its outputs and does not say which one is the
/// primary, so the first one is the answer. That is a difference of the
/// platform and it is recorded on the trait, not hidden here.
pub(crate) fn monitor_get_primary(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(1))
}

/// The four bounds of a monitor, written into the references of the call.
fn the_monitor_bounds_into(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
) -> Result<Value, RuntimeError> {
    // The bounds are the parameters after the number, and they are given by
    // reference: the second parameter of the call is the first of them.
    for index in 0..4 {
        let Some(value) = values.get(index + 1) else {
            continue;
        };
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(index + 2, name, "a VarRef", value),
                line,
            });
        }
    }
    let number = the_monitor_number(values);
    let monitor = the_monitor_at(rt, number, line, name)?;
    let places = [monitor.left, monitor.top, monitor.right, monitor.bottom];
    for (index, place) in places.iter().enumerate() {
        if values.get(index + 1).is_some() {
            rt.write_reference(&values[index + 1], Value::Int(*place));
        }
    }
    Ok(Value::Int(1))
}

/// `MonitorGet`: the bounds of a monitor.
pub(crate) fn monitor_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_monitor_bounds_into(rt, values, line, "MonitorGet")
}

/// `MonitorGetWorkArea`: the bounds of a monitor without the panels of the
/// session.
///
/// A Wayland client is not told about the panels, so the work area of a monitor
/// is the monitor itself here. That is a difference of the platform and it is
/// recorded on the trait, not hidden here.
pub(crate) fn monitor_get_work_area(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_monitor_bounds_into(rt, values, line, "MonitorGetWorkArea")
}

// -- the drives --------------------------------------------------------------------

/// The drive that a path is on, and the failure v2 gives when it is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetSpaceFree` of a
/// drive that is not there is a failure, while `DriveGetType` of one answers
/// the empty text -- the two commands disagree about the same question.
fn the_drive_of(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
) -> Result<crate::desktop::DriveSpace, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_space(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(space) => Ok(space),
        None => Err(RuntimeError {
            message: format!("{name}: the drive of {path} is not there."),
            line,
        }),
    }
}

/// `DriveGetSpaceFree`: the free space of a drive, in megabytes.
pub(crate) fn drive_get_space_free(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(
        the_drive_of(rt, values, line, "DriveGetSpaceFree")?.free,
    ))
}

/// `DriveGetCapacity`: the capacity of a drive, in megabytes.
pub(crate) fn drive_get_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(
        the_drive_of(rt, values, line, "DriveGetCapacity")?.capacity,
    ))
}

/// `DriveGetStatus`: whether the drive is ready.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a drive that is there is
/// "Ready". The answer for a drive that is not there was **not** measured; the
/// empty text is used, which is what `DriveGetType` answers for the same drive.
pub(crate) fn drive_get_status(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_space(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(_) => Ok(Value::Text("Ready".to_string())),
        None => Ok(Value::Text(String::new())),
    }
}

/// `DriveGetFilesystem`: the filesystem of a drive, as the machine names it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a drive that is not there
/// answers the empty text, the way `DriveGetType` does -- and not a failure,
/// the way `DriveGetSpaceFree` does.
pub(crate) fn drive_get_filesystem(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_space(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(space) => Ok(Value::Text(space.filesystem)),
        None => Ok(Value::Text(String::new())),
    }
}

/// `DriveGetType`: the kind of a drive, in the word of v2.
pub(crate) fn drive_get_type(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_space(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(space) => Ok(Value::Text(space.kind)),
        None => Ok(Value::Text(String::new())),
    }
}

// -- waiting for a window ----------------------------------------------------------

/// The longest a wait for a window may block.
///
/// It is the cap of `Sleep`, and it is there for the same reason: the runner's
/// timeout is checked between two statements, so a call that blocked forever
/// could not be stopped by it. The binary of v2.0.19 waits for a long time when
/// the seconds are left out of a `WinWait`, measured on 2026-11-09; this
/// runtime bounds it.
const MAX_WINDOW_WAIT_MS: u64 = 60_000;

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

// -- the processes -----------------------------------------------------------------

/// The target of a `Process*` call: the pid or the name the script wrote.
///
/// A call without the argument acts on the process of the script itself, and
/// the desktop names that process with the empty text.
fn process_target(values: &[Value]) -> String {
    if values.is_empty() {
        String::new()
    } else {
        text_of(values, 0)
    }
}

/// The failure v2 gives when a process is looked for and is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09.
const NO_TARGET: &str = "Target process not found.";

/// `ProcessExist`: the id of the process, or zero.
pub(crate) fn process_exist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let pid = rt
        .desktop()
        .process_exist(&target)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
}

/// `ProcessGetName`: the name of the image of a process.
pub(crate) fn process_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let name = rt
        .desktop()
        .process_name(&target)
        .map_err(|e| desktop_failure(e, line))?;
    match name {
        Some(name) => Ok(Value::Text(name)),
        None => Err(RuntimeError {
            message: NO_TARGET.to_string(),
            line,
        }),
    }
}

/// `ProcessGetPath`: the full path of the image of a process.
pub(crate) fn process_get_path(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let path = rt
        .desktop()
        .process_path(&target)
        .map_err(|e| desktop_failure(e, line))?;
    match path {
        Some(path) => Ok(Value::Text(path)),
        None => Err(RuntimeError {
            message: NO_TARGET.to_string(),
            line,
        }),
    }
}

/// `ProcessGetParent`: the id of the process that started a process.
pub(crate) fn process_get_parent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let pid = rt
        .desktop()
        .process_parent(&target)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
}

/// `ProcessClose`: stop a process, and answer the id of the one that was
/// stopped.
pub(crate) fn process_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let pid = rt
        .desktop()
        .process_close(&target)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
}

/// `ProcessSetPriority`: set the priority of a process, and answer its id.
pub(crate) fn process_set_priority(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let priority = text_of(values, 0);
    let target = if values.len() > 1 {
        text_of(values, 1)
    } else {
        String::new()
    };
    let pid = rt
        .desktop()
        .process_set_priority(&priority, &target)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
}

/// The longest a wait for a process may block.
///
/// The number is the cap of `Sleep`, and it is there for the same reason: the
/// runner's own timeout is checked between two statements, so a call that
/// blocks forever would not be stopped by it, and a script would have to be
/// killed. The binary of v2.0.19 waits forever when the timeout is left out
/// of a `ProcessWait`; this runtime does not.
const MAX_PROCESS_WAIT_MS: u64 = 60_000;

/// The timeout of a wait, in milliseconds.
///
/// The second parameter of v2 is seconds, and it may carry a point. A call
/// without it waits the cap, because the runtime does not wait forever.
fn process_wait_timeout(values: &[Value]) -> u64 {
    if values.len() < 2 {
        return MAX_PROCESS_WAIT_MS;
    }
    let seconds = number_of(values, 1);
    if seconds <= 0.0 {
        return 0;
    }
    (seconds * 1000.0).min(MAX_PROCESS_WAIT_MS as f64) as u64
}

/// `ProcessWait`: wait for a process to be there.
pub(crate) fn process_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let timeout = process_wait_timeout(values);
    let pid = rt
        .desktop()
        .process_wait(&target, timeout)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
}

/// `ProcessWaitClose`: wait for a process to close.
pub(crate) fn process_wait_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = process_target(values);
    let timeout = process_wait_timeout(values);
    let pid = rt
        .desktop()
        .process_wait_close(&target, timeout)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(i64::from(pid)))
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
/// `EditGetLineCount(Hwnd)`: how many lines the text of a control holds.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a control added with three
/// lines answers 3. The handle of the control is the number that names it.
pub(crate) fn edit_get_line_count(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as u64;
    rt.desktop()
        .gui_line_count(number)
        .map(Value::Int)
        .map_err(|reason| desktop_failure(reason, line))
}

/// `EditGetLine(N, Hwnd)`: the text of the Nth line of a control.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: **the number comes first and
/// the handle second**, and called the other way round the reference answers
/// "Target window not found.", which reads as though the handle were wrong while
/// the two parameters had traded places. The text is read through the channel of
/// `Value`, which already answers the whole text of a control.
pub(crate) fn edit_get_line(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let wanted = number_of(values, 0).max(1.0) as usize;
    let number = number_of(values, 1) as u64;
    let text = rt
        .desktop()
        .gui_value(number, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    let found = text.split('\n').nth(wanted - 1).unwrap_or("");
    Ok(Value::Text(found.to_string()))
}

/// `EditGetCurrentLine(Hwnd)`: which line the cursor of a control is on.
///
/// The place of the cursor is the helper's answer, and the line is the number of
/// breaks before it plus one -- the text of an entry carries its breaks, measured
/// in round 143.
pub(crate) fn edit_get_current_line(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as u64;
    let cursor = rt
        .desktop()
        .gui_cursor(number)
        .map_err(|reason| desktop_failure(reason, line))?;
    let text = rt
        .desktop()
        .gui_value(number, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    let before: usize = (cursor.max(0) as usize).min(text.chars().count());
    let breaks = text.chars().take(before).filter(|c| *c == '\n').count();
    Ok(Value::Int(breaks as i64 + 1))
}

/// `EditGetCurrentCol(Hwnd)`: which column of its line the cursor is at.
///
/// The place is the helper's answer and the text comes through the channel of
/// `Value`; the column is the distance from the last break before it, counted from
/// one, and the first line has no break to measure from.
pub(crate) fn edit_get_current_col(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as u64;
    let cursor = rt
        .desktop()
        .gui_cursor(number)
        .map_err(|reason| desktop_failure(reason, line))?;
    let text = rt
        .desktop()
        .gui_value(number, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    let before: usize = (cursor.max(0) as usize).min(text.chars().count());
    let seen: Vec<char> = text.chars().take(before).collect();
    let column = match seen.iter().rposition(|held| *held == '\n') {
        Some(at) => seen.len() - at,
        None => seen.len() + 1,
    };
    Ok(Value::Int(column as i64))
}

/// `EditGetSelectedText(Hwnd)`: the text a control of the script has selected.
pub(crate) fn edit_get_selected_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as u64;
    rt.desktop()
        .gui_selection(number)
        .map(Value::Text)
        .map_err(|reason| desktop_failure(reason, line))
}

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

pub(crate) fn control_visibility(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_a_control(rt, values, line, false)
}

pub(crate) fn control_show(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_a_control(rt, values, line, true)
}

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

pub(crate) fn control_set_enabled(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    the_state_of_a_control(rt, values, line, "enabled", on)
}

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

pub(crate) fn control_get_checked(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_tick_of_a_control(rt, values, line, 0, None)
}

pub(crate) fn control_set_checked(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let on = number_of(values, 0) != 0.0;
    the_tick_of_a_control(rt, values, line, 1, Some(on))
}

/// `EditPaste(Text, Hwnd)`: put a text into a control of the script at its cursor.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the **text comes first** and
/// the handle second, as the whole family does.
pub(crate) fn edit_paste(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    let number = number_of(values, 1) as u64;
    rt.desktop()
        .gui_paste(number, &text)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

/// `ControlMove(X, Y, W, H, Control)`: moves a control of a window.
///
/// **Refused, and with a reason narrower than the blanket one.** Measured on the
/// binary of v2.0.19 on 2026-11-09: every one of the four numbers is required -- a
/// call that leaves one out is refused as *"Parameter #N of ControlMove requires a
/// Number, but received an empty string"* -- so **there is no size-only form**, and
/// every call asks for a place. The window of this tree lays its controls out in
/// order, so a place cannot be set: unlike the `Move` of a control (round 122),
/// which has an honourable half, this name has none.
pub(crate) fn control_move(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the window of this tree lays its controls out in order, so the place of a control cannot be set, and this name has no form that asks for a size alone".to_string(),
        line,
    })
}

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

pub(crate) fn win_hide(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_the_window(rt, values, line, false)
}

pub(crate) fn win_show(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_visibility_of_the_window(rt, values, line, true)
}

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

pub(crate) fn win_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "activate")
}

pub(crate) fn win_minimize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "minimize")
}

pub(crate) fn win_maximize(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "maximize")
}

pub(crate) fn win_restore(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "restore")
}

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

/// `WinMoveBottom(WinTitle)`: **refused, with a reason of its own.**
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text. GTK 4
/// removed the call that lowers a window -- `present` raises, and there is no
/// counterpart -- and a compositor decides the order anyway, so this tree has no way
/// to send a window to the back.
pub(crate) fn win_move_bottom(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the toolkit of this desktop can raise a window and has no call that lowers one, and the order of the windows is the compositor's".to_string(),
        line,
    })
}

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

pub(crate) fn win_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_close_of_the_window(rt, values, line)
}

pub(crate) fn win_kill(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_close_of_the_window(rt, values, line)
}

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

/// `ObjPtr(Object)` and `ObjFromPtr(Ptr)`: the number that names an object, and the
/// object a number names.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `ObjPtr` answers an Integer and
/// `ObjFromPtr` of it answers the same object, so the pair round-trips. The number is
/// an **identity and not an address**: the same object always answers the same
/// number, and it is shaped like an address because that is what v2 answers.
pub(crate) fn obj_ptr(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value.clone(),
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            })
        }
    };
    Ok(Value::Int(rt.the_pointer_of(&value)))
}

pub(crate) fn obj_from_ptr(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as i64;
    match rt.the_object_of(number) {
        Some(value) => Ok(value),
        None => Err(RuntimeError {
            message: format!(
                "the number {number} does not name an object this script has asked about"
            ),
            line,
        }),
    }
}

/// `ObjPtrAddRef(Object)` and `ObjFromPtrAddRef(Ptr)`: the pair of `ObjPtr` that also
/// keeps a reference.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the pair round-trips, and a value
/// that is not an object is refused as *"Expected an object but got an Integer."*
///
/// The reference this tree adds is the one its `Rc` already holds: the number that
/// names the object is kept in the runtime's table, which is a holder, so the object
/// outlives what would otherwise have dropped it.
pub(crate) fn obj_ptr_add_ref(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value.clone(),
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            })
        }
    };
    if !matches!(value, Value::Object(_)) {
        let kind = the_kind_of(&value);
        // The reference says "an Integer", not "a Integer".
        let article = if kind.starts_with(['A', 'E', 'I', 'O', 'U']) {
            "an"
        } else {
            "a"
        };
        return Err(RuntimeError {
            message: format!("Expected an object but got {article} {kind}."),
            line,
        });
    }
    Ok(Value::Int(rt.the_pointer_of(&value)))
}

pub(crate) fn obj_from_ptr_add_ref(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_of(values, 0) as i64;
    match rt.the_object_of(number) {
        Some(value) => Ok(value),
        None => Err(RuntimeError {
            message: format!(
                "the number {number} does not name an object this script has asked about"
            ),
            line,
        }),
    }
}

/// The name of the kind of a value, for the message of a refusal.
fn the_kind_of(value: &Value) -> &'static str {
    match value {
        Value::Int(_) => "Integer",
        Value::Float(_) => "Float",
        Value::Text(_) => "String",
        Value::Object(_) => "Object",
        _ => "value that is not an object",
    }
}

/// `SoundGetName(Component, Device)`: **refused, with a reason of its own.**
///
/// The name of an audio device belongs to the system that runs it, and this runtime
/// answers the volume and the mute of the default device (`SoundGetMute` answers, and
/// `SoundGetVolume` with it) and not the names the system gives them. Answering a name
/// would be a name of this machine passed off as the one v2 would give on its own.
pub(crate) fn sound_get_name(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the name of an audio device belongs to the system that runs it, and this runtime answers the volume and the mute of the default device and not the names the system gives them".to_string(),
        line,
    })
}

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

/// `FileRecycleEmpty`: empty the trash of the machine.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text.
pub(crate) fn file_recycle_empty(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    rt.desktop()
        .empty_the_trash()
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(String::new()))
}

/// `FileCreateShortcut(Target, LinkFile, WorkingDir, Args, Description, IconFile, …)`:
/// make a shortcut.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: it answers the empty text and makes
/// the file, and it makes it even for a target that is not there. On this platform a
/// shortcut is a `.desktop` file.
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

/// `Edit()`: **refused, with a reason of its own.**
///
/// The name takes no parameters -- measured on the binary of v2.0.19 on 2026-11-09, its
/// arity is nought to nought -- because it opens the **file of the script itself**. The
/// script of this runtime is read from the standard input, so **there is no file**, and
/// the same fact refuses `A_ScriptFullPath`, `A_ScriptDir` and `A_ScriptName`.
pub(crate) fn edit(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the script of this runtime is read from the standard input, so it has no file for an editor to open".to_string(),
        line,
    })
}

// -- the gap names that a script can ask about ----------------------------------
//
// These are the names the audit of AHK_X11 left in a gap class. Every one of them
// is carried now: a value where the machine can honestly give one, and a reason
// of its own where it cannot. A reason of its own is not the blanket reason about
// Windows, which was false about names that are not shaped for Windows at all.

/// `SysGet(N)`: the system metric of this index, as this machine knows it.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: `SysGet(0)` answers 3840,
/// `SysGet(1)` 2160, `SysGet(80)` 1 and `SysGet(4)` 23. The first two are the
/// width and the height of the virtual screen, and 80 is the number of monitors.
/// The tree knows exactly those, so it answers them and refuses the rest by name.
pub(crate) fn sys_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let index = number_of(values, 0) as i64;
    let (width, height) = rt
        .desktop()
        .screen_size()
        .map_err(|e| desktop_failure(e, line))?;
    let answer = match index {
        0 => Some(width),
        1 => Some(height),
        80 => Some(rt.desktop().monitors().map(|m| m.len() as i64).unwrap_or(1)),
        _ => None,
    };
    match answer {
        Some(value) => Ok(Value::Int(value)),
        None => Err(RuntimeError {
            message: format!(
                "the system metric {index} is a measure of Windows that this machine does not keep"
            ),
            line,
        }),
    }
}

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

/// `RunAs(User, Password, Domain)`: set the user a later `Run` would use.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: it answers the empty text.
/// The credentials are a setting of the machine and not a call that can be proved
/// here, so the tree keeps the shape of the answer and says in a note what it did
/// not do.
pub(crate) fn run_as(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(String::new()))
}

/// `BlockInput(Mode)`: **refused, with a reason of its own.**
///
/// Measured on the binary of v2.0.19 on 2026-11-10: `BlockInput("Off")` answers
/// the empty text. Turning input OFF for the whole machine is a privilege this
/// compositor does not give a client, and this tree will not claim it did.
pub(crate) fn block_input(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let mode = text_of(values, 0).to_ascii_lowercase();
    if mode.is_empty() || mode.starts_with("off") || mode == "0" {
        return Ok(Value::Text(String::new()));
    }
    Err(RuntimeError {
        message: "blocking the input of the whole machine is a privilege this compositor does not give a client".to_string(),
        line,
    })
}

/// `OnClipboardChange(Func, AddRemove)`: **refused, with a reason of its own.**
///
/// Measured on the binary of v2.0.19 on 2026-11-10: a `Func` answers the empty
/// text, a String is refused, and the callback has to be called whenever the
/// clipboard changes. This runtime has no clipboard watcher: its clipboard is a
/// pair of read and write calls with no event between them.
pub(crate) fn on_clipboard_change(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the clipboard of this runtime is a read and a write, with no watcher to call a function when it changes".to_string(),
        line,
    })
}

/// `CallbackCreate(Func)`: answer a number that names the function.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: it answers an Integer, the
/// address of a native thunk. This tree has no native thunks, but it does have
/// the same table of identities `ObjPtr` uses, and a number from it is a value a
/// script can hold and compare.
pub(crate) fn callback_create(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value.clone(),
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            })
        }
    };
    Ok(Value::Int(rt.the_pointer_of(&value)))
}

/// `Shutdown(Flag)`: **refused, with a reason of its own.**
///
/// Logging off, rebooting or powering down the machine is not a question a
/// script runner answers; the reference was not executed for this reason.
pub(crate) fn shutdown(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "ending the session of the machine is not a call this script runner makes".to_string(),
        line,
    })
}

/// `FileRecycle` and `FileGetVersion`: **refused, each with a reason of its own.**
///
/// `FileRecycle` was measured on 2026-11-09: the reference answers `Failed` for a
/// file that exists. `FileGetVersion` was measured on 2026-11-09 and again on
/// 2026-11-10: the reference hangs on it. **Neither has an answer to match**, so
/// the honest answer is that there is nothing to match -- not a name of Windows.
/// `FileRecycle`: **refused, with a reason of its own.** Measured on 2026-11-09:
/// the reference answers `Failed` for a file that exists, so there is no success
/// to match. This has nothing to do with Windows: the tree has a trash, but it
/// will not invent what v2 would have answered.
pub(crate) fn file_recycle(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "FileRecycle has no answer to match: the reference answers Failed for a file that exists, so a success here would be an invention".to_string(),
        line,
    })
}

/// `FileGetVersion`: **refused, with a reason of its own.** Measured on 2026-11-09
/// and again on 2026-11-10: the reference hangs on it, and an ELF has no version
/// resource to read. An empty text for every file would look like an answer and
/// would not be one.
pub(crate) fn file_get_version(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "FileGetVersion has no answer to match: the reference hangs on it and an ELF has no version resource, so an empty text would look like an answer and would not be one".to_string(),
        line,
    })
}

/// `ControlGetPos`: **refused, with a reason of its own.**
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the reference hangs on it.
/// Three coordinate spaces of this desktop answer the same numbers, so a place
/// would be an invention even without that.
pub(crate) fn control_get_pos(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the reference hangs on ControlGetPos, and the coordinate spaces of this desktop answer the same numbers, so a place would be an invention".to_string(),
        line,
    })
}

/// The three names that act on every window at once: **refused, with a reason of
/// their own.** The compositor owns the stacking and the state of all windows; a
/// client is told about its own window and cannot lower or minimise the rest.
pub(crate) fn all_windows(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "all the windows belong to the compositor, and a client can only act on the window it made".to_string(),
        line,
    })
}

/// `OnMessage`: **refused, with a reason of its own.** The messages are the ones
/// of a Win32 window procedure, which this runtime does not have.
pub(crate) fn on_message(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "the messages are those of a Win32 window procedure, and this runtime has no such loop".to_string(),
        line,
    })
}

/// `GroupAdd(GroupName, WinTitle, WinText, ExcludeTitle, ExcludeText)`: put a
/// window title into a named set that outlives the statement.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: it answers the empty text.
/// The window text and the two exclusions are not carried: a call that gives
/// them is refused rather than ignored.
pub(crate) fn group_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 2 {
        return Err(RuntimeError {
            message: "the window text and the exclusions of GroupAdd are not carried yet."
                .to_string(),
            line,
        });
    }
    let group = text_of(values, 0);
    let title = text_of(values, 1);
    if group.is_empty() {
        return Err(RuntimeError {
            message: "the name of a window group may not be empty.".to_string(),
            line,
        });
    }
    rt.add_to_group(&group, &title);
    Ok(Value::Text(String::new()))
}

/// `GroupActivate(GroupName, Mode)`: answer the id of the first window of the
/// group that is on the screen, or zero when none is.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the answer is a whole
/// number, and zero when the group has no matching window. This runtime's
/// windows are identified by the text the compositor gives them (`WinGetID`),
/// so the answer is that text when a window is found and a zero when it is not.
pub(crate) fn group_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let group = text_of(values, 0);
    let titles = rt.the_titles_of_a_group(&group);
    for title in titles {
        let windows = rt
            .desktop()
            .matching_windows(&title, crate::settings::title_match_mode())
            .map_err(|e| desktop_failure(e, line))?;
        if let Some(window) = windows.first() {
            return Ok(Value::Text(window.id.clone()));
        }
    }
    Ok(Value::Int(0))
}

/// `GroupDeactivate` and `GroupClose`: take the group away.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: both answer the empty text.
pub(crate) fn group_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let group = text_of(values, 0);
    rt.close_a_group(&group);
    Ok(Value::Text(String::new()))
}

/// `GuiFromHwnd`, `GuiCtrlFromHwnd`, `MenuFromHandle` and `MenuSelect`:
/// **refused, with a reason of their own.** These take a Win32 HWND and give
/// back the object that owns it. This runtime has no HWND table: the windows
/// are identified by the text the compositor gives (`WinGetID`), so there is
/// no number to look an object up by.
pub(crate) fn from_hwnd(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "this runtime has no HWND table: the windows are identified by the text the compositor gives, so there is no number to look an object up by".to_string(),
        line,
    })
}

pub(crate) fn sleep(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let millis = number_of(values, 0).clamp(0.0, 60_000.0) as u64;
    rt.sleep_ms(millis);
    // An event that happened while the script slept is answered when it wakes.
    // `Sleep` is the one statement of this runtime that waits, so it is the one
    // place an event can be answered without a loop of its own.
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
