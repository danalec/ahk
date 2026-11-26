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
use alloc::collections::BTreeMap;
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
    /// The parameter is a reference the caller passes, and v2 declares it as
    /// a value of the kind `VarRef`: a value of another kind is refused with
    /// "requires a VarRef, but received ...", and the name of the built-in is
    /// in the `What` of the failure.
    ///
    /// Measured on the binary of v2.0.19: `MonitorGet(1, 9)` is
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

/// The names that are a command and a function at the same time.
///
/// The v2 has these names, and they can be written as a statement and as
/// an expression: measured on the binary of v2.0.31, the
/// `x := Persistent(true)` gives the value of before, and the `Persistent`
/// as a statement also does.
///
/// The table of the reference keeps them as "command", because that is how
/// the binary lists them, and the tree loads the name as a command and as a
/// function.
pub const THE_NAMES_THAT_ARE_BOTH: &[&str] = &[
    "DetectHiddenText",
    "DetectHiddenWindows",
    "FileEncoding",
    "ListLines",
    "Persistent",
    "SendLevel",
    "SendMode",
    "SetControlDelay",
    "SetDefaultMouseSpeed",
    "SetMouseDelay",
    "SetRegView",
    "SetStoreCapsLockMode",
    "SetTitleMatchMode",
];

/// Whether the name is a command and a function at the same time.
pub fn is_both(name: &str) -> bool {
    THE_NAMES_THAT_ARE_BOTH
        .iter()
        .any(|both| both.eq_ignore_ascii_case(name))
}

/// The built-ins of a table, indexed by the name in lower case.
///
/// The parser asks the table for a name once for every call of the script,
/// and the table of the v1 has 288 rows: the answer was a walk of the whole
/// table for every call, and every row of the walk lowered its name into a
/// new string. The index is the same question with a walk of the tree of
/// the names -- eight names for the table of the v1 -- and without a new
/// string.
///
/// A name may be in the table twice: the `GetKeyState` of the v1 is a
/// command and a function, and the index keeps both of them, in the order
/// of the table, so that the one that answers first is the one that the
/// walk of the table would have found.
pub struct TheIndexOfTheBuiltins {
    by_name: BTreeMap<String, Vec<&'static Builtin>>,
}

impl TheIndexOfTheBuiltins {
    /// The index of a table of built-ins.
    pub fn new(builtins: &'static [Builtin]) -> Self {
        let mut by_name: BTreeMap<String, Vec<&'static Builtin>> = BTreeMap::new();
        for builtin in builtins {
            by_name
                .entry(builtin.name.to_lowercase())
                .or_default()
                .push(builtin);
        }
        Self { by_name }
    }

    /// The built-ins of the table that the name names, in the order of the
    /// table.
    ///
    /// The name is the name in lower case: the index has the names of the
    /// table in lower case too, the way the parser writes them down.
    pub fn the_builtins_of_the_name(&self, name: &str) -> &[&'static Builtin] {
        match self.by_name.get(name) {
            Some(builtins) => builtins.as_slice(),
            None => &[],
        }
    }
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
        name: "IsSet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Reference],
        handler: is_set,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: callback_free,
    },
    Builtin {
        name: "CaretGetPos",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Reference, Parameter::Reference],
        handler: caret_get_pos,
    },
    Builtin {
        name: "ComCall",
        kind: BuiltinKind::Function,
        min: 2,
        max: 255,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_call,
    },
    Builtin {
        name: "ComObjActive",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_active,
    },
    Builtin {
        name: "ComObjConnect",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_connect,
    },
    Builtin {
        name: "ComObjFlags",
        kind: BuiltinKind::Function,
        min: 1,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_flags,
    },
    Builtin {
        name: "ComObjFromPtr",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_from_ptr,
    },
    Builtin {
        name: "ComObjGet",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_get,
    },
    Builtin {
        name: "ComObjQuery",
        kind: BuiltinKind::Function,
        min: 2,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_query,
    },
    Builtin {
        name: "ComObjType",
        kind: BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_type,
    },
    Builtin {
        name: "ComObjValue",
        kind: BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: com_obj_value,
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
        variant: 1,
        parameters: &[Parameter::Value],
        handler: control_style,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_style,
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
        variant: 1,
        parameters: &[Parameter::Value],
        handler: control_set_style,
    },
    Builtin {
        name: "ControlSetStyle",
        kind: BuiltinKind::Command,
        min: 2,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: control_set_style,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_eject,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_label,
    },
    Builtin {
        name: "DriveGetList",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_list,
    },
    Builtin {
        name: "DriveGetSerial",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_serial,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_get_status_cd,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_lock,
    },
    Builtin {
        name: "DriveRetract",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_retract,
    },
    Builtin {
        name: "DriveSetLabel",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_set_label,
    },
    Builtin {
        name: "DriveUnlock",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: drive_unlock,
    },
    Builtin {
        name: "Edit",
        kind: BuiltinKind::Command,
        min: 0,
        max: 0,
        variant: 0,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_get_shortcut,
    },
    Builtin {
        name: "FileGetVersion",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: file_get_version,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: gui_ctrl_from_hwnd,
    },
    Builtin {
        name: "GuiFromHwnd",
        kind: BuiltinKind::Command,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: gui_from_hwnd,
    },
    Builtin {
        name: "IL_Add",
        kind: BuiltinKind::Command,
        min: 2,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: il_add,
    },
    Builtin {
        name: "IL_Create",
        kind: BuiltinKind::Command,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: il_create,
    },
    Builtin {
        name: "IL_Destroy",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: il_destroy,
    },
    Builtin {
        name: "ListViewGetContent",
        kind: BuiltinKind::Command,
        min: 0,
        max: 6,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: list_view_get_content,
    },
    Builtin {
        name: "MenuFromHandle",
        kind: BuiltinKind::Command,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: menu_from_handle,
    },
    Builtin {
        name: "MenuSelect",
        kind: BuiltinKind::Command,
        min: 0,
        max: 11,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: menu_select,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: on_clipboard_change,
    },
    Builtin {
        name: "OnMessage",
        kind: BuiltinKind::Command,
        min: 2,
        max: 3,
        variant: 0,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: post_message,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reg_create_key,
    },
    Builtin {
        name: "RegDelete",
        kind: BuiltinKind::Function,
        min: 0,
        max: 2,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reg_delete,
    },
    Builtin {
        name: "RegDeleteKey",
        kind: BuiltinKind::Function,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reg_delete_key,
    },
    Builtin {
        name: "RegRead",
        kind: BuiltinKind::Function,
        min: 0,
        max: 3,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reg_read,
    },
    Builtin {
        name: "RegWrite",
        kind: BuiltinKind::Function,
        // Measured on the binary of v2.0.31: the first
        // parameter is mandatory. The release (2.0.20) made it
        // so: "Fixed RegWrite parameter 1 to be mandatory."
        min: 1,
        max: 4,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reg_write,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: send_message,
    },
    Builtin {
        name: "SetCapsLockState",
        kind: BuiltinKind::Command,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_caps_lock_state,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_num_lock_state,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: set_scroll_lock_state,
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
        variant: 0,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: sound_get_interface,
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
        variant: 0,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: status_bar_get_text,
    },
    Builtin {
        name: "StatusBarWait",
        kind: BuiltinKind::Command,
        min: 0,
        max: 8,
        variant: 0,
        parameters: &[Parameter::Value],
        handler: status_bar_wait,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: tray_set_icon,
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
        variant: 1,
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
        variant: 0,
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
        variant: 0,
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
        variant: 1,
        parameters: &[Parameter::Value],
        handler: win_set_style,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_style,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: win_set_trans_color,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: critical,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: list_lines,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: pause,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: reload,
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
        variant: 0,
        parameters: &[Parameter::Value],
        handler: thread_command,
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

/// `ComObjFlags(ComValue [, Flags])`: the flags of a value of the COM.
///
/// Measured on the binary of v2.0.19: a value that is not a
/// COM object throws "Parameter #1 of ComObjFlags requires a ComValue, but
/// received an Integer.".
///
/// The v2.0.31 writes the flags that are given and answers the flags of the
/// value -- read in its source (`source/script_com.cpp` of the reference
/// tree).
///
/// Measured on the binary of v1.1.37.02: the `ComObjFlags` of a value that
/// is not a value of the COM answers the empty text, and it does not throw.
pub(crate) fn com_obj_flags(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    if crate::classes::comvalue::the_flags_of_the_value(&first).is_none() {
        if rt.dialect().is_the_v1() {
            return Ok(Value::Text("".into()));
        }
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ComObjFlags", TypeName::ComValue, &first),
            line,
        });
    }
    if let Some(flags) = values.get(1) {
        let flags = flags.to_number().unwrap_or(0.0) as i64;
        crate::classes::comvalue::the_flags_are_written(&first, flags);
    }
    Ok(Value::Int(
        crate::classes::comvalue::the_flags_of_the_value(&first).unwrap_or(0),
    ))
}

/// `ComObjValue(ComValue)`: the number of a value of the COM.
///
/// Measured on the binary of v2.0.19: a value that is not a
/// COM object throws "Parameter #1 of ComObjValue requires a ComValue, but
/// received an Integer.".
///
/// The number is the address of the object, or the value of the `VARIANT`.
///
/// Measured on the binary of v1.1.37.02: the `ComObjValue` of a value
/// that is not a value of the COM answers the empty text, and it does not
/// throw.
pub(crate) fn com_obj_value(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    match crate::classes::comvalue::the_number_of(&first) {
        Some(number) => Ok(Value::Int(number)),
        None if rt.dialect().is_the_v1() => Ok(Value::Text("".into())),
        None => Err(RuntimeError {
            message: crate::runtime::requires(1, "ComObjValue", TypeName::ComValue, &first),
            line,
        }),
    }
}

/// `ComObjConnect(ComValue [, Event])`: a connection with the events of an
/// object.
///
/// The connection with the events of an object of the COM is an event sink:
/// the machine calls the script at every event, and the script receives it
/// in the function whose name is the prefix and the name of the member.
///
/// The second parameter is the prefix of the names of the events. When
/// the script does not say one, the prefix is empty, and the name of the
/// event is the name of the member.
pub(crate) fn com_obj_connect(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    let Some(pointer) = crate::classes::comvalue::the_number_of(&first) else {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ComObjConnect", TypeName::ComValue, &first),
            line,
        });
    };
    let kind = crate::classes::comvalue::the_kind_of_the_value(&first).unwrap_or(0);
    // O prefixo dos nomes dos eventos.
    let prefix = values
        .get(1)
        .map(|prefix| prefix.to_text())
        .unwrap_or_default();
    if prefix.is_empty() && values.len() > 1 {
        return Err(RuntimeError {
            message: "Parameter #2 of ComObjConnect is invalid.".to_string(),
            line,
        });
    }
    let object = crate::desktop::ComObject {
        pointer: pointer as u64,
        kind,
    };
    rt.desktop()
        .com_object_is_connected(&object, &prefix)
        .map_err(|reason| desktop_failure(reason, line))?;
    rt.mark_com_events_connected();
    Ok(Value::Int(1))
}

/// The parameters of a command that may not be left out.
///
/// The table of the binary says how many parameters a command declares,
/// and the documentation says which of them may be left out. There are
/// commands that declare fewer than they need: the `FileGetAttrib` takes
/// the name of the file, and it takes it whether the table says so or
/// not. Measured on the binary of v2.0.19: the refusal of
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
    BUILTINS
        .iter()
        .find(|builtin| builtin.name.eq_ignore_ascii_case(name))
}

/// The value that a built-in complains about, when its message does
/// not name the parameter it is about.
///
/// v2 carries the value in the `Extra` of the failure, and most of the
/// messages name the parameter it is about. The message of the options
/// of an input box that v2 does not know does not: the value there is
/// the option itself, and the built-in says which one it is.
pub fn the_extra_of(name: &str, message: &str, values: &[Value]) -> Option<String> {
    // The value that a native call refused is in the `Extra` of the
    // failure.
    //
    // Measured on the binary of AutoHotkey v2.0.32: the
    // `DllCall("kernel32\lstrlenW", "Str", 1149)` is refused with "Expected
    // a String but got an Integer." and the value that was refused is in
    // the `Extra`.
    if name == "DllCall" {
        return the_extra_of_a_native_call(values);
    }
    // The failure of a mode of the window titles carries the value that was
    // not a mode in the `Extra`: measured on the binary of v2.0.19 `SetTitleMatchMode("Bogus")` carries "Bogus",
    // `SetTitleMatchMode(4)` carries "4" and `SetTitleMatchMode(0)` carries
    // "0".
    if name == "SetTitleMatchMode" && message == "Parameter #1 of SetTitleMatchMode is invalid." {
        return values.first().map(|value| value.to_text());
    }
    // The failure of a window query carries the window it was asked about in
    // the `Extra`, and a window that was named by its id leaves it empty.
    //
    // Measured on the binary of v2.0.19: `WinGetID("zzz")` is
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
    // The family of the style asks for the window the same way, and the
    // place of the target is not the same in all of them: the style itself
    // comes first in a write, and the control comes before the window in the
    // family of the control.
    //
    // Measured on the binary of v2.0.31: `WinGetStyle("ZZZ")`
    // is "Target window not found." with "ZZZ" in the `Extra`.
    if message == "Target window not found." {
        let index = match name {
            "WinGetStyle" | "WinGetExStyle" | "WinGetTransColor" => 0,
            "WinSetStyle" | "WinSetExStyle" | "WinSetTransColor" => 1,
            "ControlGetStyle" | "ControlGetExStyle" => 1,
            "ControlSetStyle" | "ControlSetExStyle" => 2,
            _ => return None,
        };
        return match values.get(index) {
            Some(Value::Int(_)) | None => None,
            Some(value) => Some(value.to_text()),
        };
    }
    // The base of an object is written with the `ObjSetBase`, and the value
    // that is not an object goes in the `Extra` of the refusal.
    //
    // Measured on the binary of v2.0.19.
    if name == "ObjSetBase" && message.starts_with("Expected an Object but got") {
        return values.get(1).map(crate::runtime::the_extra_of_the_value);
    }
    // The button of a `MouseClick` is the first parameter, and the value
    // that the refusal carries has the space at the end of it cut.
    //
    // Measured on the binary of v2.0.19: "Left " is refused
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
    // Measured on the binary of v2.0.19: the copy of a file
    // that is not there fails with "Failed", and the `Extra` of the failure
    // is 1.
    if matches!(name, "FileCopy" | "FileMove") && message == "Failed" {
        return Some("1".to_string());
    }
    // The target of a `StrPut` is a buffer, and a buffer that cannot hold
    // the text is refused with the kind of the value in the `Extra`: the
    // name of the class, and not the text of the object.
    //
    // Measured on the binary of v2.0.19.
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

/// The value that a native call refused.
///
/// The arguments of a native call are the target, and then the pairs of a
/// type and of a value. The value that does not have the kind of its type
/// is the one that the call refused.
fn the_extra_of_a_native_call(values: &[Value]) -> Option<String> {
    let mut index = 1;
    while index + 1 < values.len() {
        let Value::Text(kind) = &values[index] else {
            return None;
        };
        let value = &values[index + 1];
        let (kind, _) = crate::dll::parse_type(kind)?;
        let refused = if kind.is_text() {
            // A variable is the buffer of the text, and a text is a text.
            !matches!(value, Value::Text(_) | Value::Reference(_))
        } else if kind.is_float() {
            !matches!(value, Value::Float(_))
        } else {
            !matches!(value, Value::Int(_) | Value::Float(_))
        };
        if refused {
            return Some(value.to_text());
        }
        index += 2;
    }
    None
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
/// The native call with the runtime kept for the callbacks of the machine.
///
/// The desktop is called by its pointer, so the runtime is not borrowed
/// while a callback may run: the callback reads the runtime back for the
/// time of the call, and the caller does not use it until the call comes
/// back.
#[cfg(feature = "std")]
fn the_native_call_with_the_callbacks<'a>(
    rt: &mut Runtime<'a>,
    call: &mut NativeCall,
    at: Option<u64>,
) -> Result<crate::dll::NativeResult, crate::dll::NativeFailure> {
    let desktop_ptr: *mut (dyn crate::desktop::Desktop + 'a) = rt.desktop();
    let previous = crate::callback::set_current(rt);
    let answer = unsafe {
        match at {
            Some(address) => (*desktop_ptr).call_native_at(address, call),
            None => (*desktop_ptr).call_native(call),
        }
    };
    crate::callback::clear_current(previous);
    answer
}

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
    // The trace of the call says what the script asked: the target as it
    // was written, and the arguments as they were given. The trace is off
    // in most of the runs, and nothing is gathered when it is.
    let traced = crate::trace::is_asked();
    let the_target = if traced {
        the_target_as_written(first)
    } else {
        String::new()
    };
    let arguments = if traced {
        the_arguments_of_the_call(values)
    } else {
        Vec::new()
    };
    // The trace of a call that failed is a call with no answer.
    let trace_the_call = |answer: Option<&Value>| {
        if traced {
            crate::trace::dll_call(&the_target, &arguments, answer);
        }
    };
    // The target is a library\function, or the address of a function of
    // the machine: `DllCall` of a number calls the address (the callback
    // of `CallbackCreate` is called this way).
    //
    // The target of the v1 may be a variable that holds the name of the
    // function, and the parser of the v1 makes every variable of a native
    // call a reference.
    //
    // Measured on the binary of the v1.1.37.02: the
    // `DllCall(fn, "Str", "Test")` of a script whose `fn` holds
    // "user32\RegisterWindowMessage" answers the number of the message.
    let first = &match values.first() {
        Some(Value::Reference(reference)) => match rt.read_reference(reference) {
            Some(value) => value,
            None => values[0].clone(),
        },
        _ => values[0].clone(),
    };
    let (library, function, at) = match first {
        Value::Text(target) => {
            let (library, function) = crate::dll::split_target(target);
            (library, function, None)
        }
        Value::Int(number) if *number >= 65536 => {
            (String::new(), String::new(), Some(*number as u64))
        }
        Value::Float(number) if *number >= 65536.0 => {
            (String::new(), String::new(), Some(*number as u64))
        }
        _ => {
            trace_the_call(None);
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: expected_type(TypeName::String, first.type_name()),
                    line,
                },
            ));
        }
    };
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
            trace_the_call(None);
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
    // The room of the buffer of each argument: the text that the machine
    // writes into it must be terminated inside that room.
    let mut rooms = Vec::new();
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
            trace_the_call(None);
            return Err((
                NativeFailureKind::Invalid,
                RuntimeError {
                    message: "Invalid arg type.".into(),
                    line,
                },
            ));
        };
        let (argument, reference, room) = marshal(rt, kind, address, &rest[index + 1], line)
            .map_err(|error| (NativeFailureKind::Invalid, error))?;
        arguments.push(argument);
        references.push(reference);
        rooms.push(room);
        index += 2;
    }
    let mut call = NativeCall {
        library,
        function,
        convention,
        returns,
        arguments,
    };
    // AutoHotkey v1 scripts free `RegisterCallback` pointers via
    // `DllCall("GlobalFree", "UPtr", cb)` because v1 allocates callback
    // thunks with `GlobalAlloc`. Our callbacks use static trampolines, so
    // intercept `GlobalFree` on a trampoline address and release the slot
    // instead of handing a `.text` pointer to Win32 `GlobalFree`.
    #[cfg(feature = "std")]
    if at.is_none()
        && call.function.eq_ignore_ascii_case("GlobalFree")
        && (call.library.is_empty()
            || call.library.eq_ignore_ascii_case("kernel32")
            || call.library.eq_ignore_ascii_case("kernel32.dll"))
        && call.arguments.len() == 1
    {
        if let NativeValue::Integer(address) = call.arguments[0].value {
            if crate::callback::release_if_callback(address as usize) {
                let value = Value::Int(0);
                trace_the_call(Some(&value));
                return Ok(value);
            }
        }
    }
    // The callback of the machine runs while the native call goes on: the
    // runtime is kept for the thread, and the desktop is called by its
    // pointer, so the runtime is not borrowed while the callback may run.
    // The error of the machine is the error of the script: the machine is
    // told what it was before the call, and the error that the call leaves
    // is the one that the script keeps.
    //
    // Measured on the binary of v2.0.32: the `A_LastError` of a script
    // that called nothing is 0, and the tree answered 203: the calls that
    // the tree makes to find the library and the function left their own
    // error behind.
    let copy = rt.the_last_error();
    let _ = rt.desktop().set_last_error(copy);
    #[cfg(feature = "std")]
    let answer = the_native_call_with_the_callbacks(rt, &mut call, at);
    #[cfg(not(feature = "std"))]
    let answer = match at {
        Some(address) => rt.desktop().call_native_at(address, &mut call),
        None => rt.desktop().call_native(&mut call),
    };
    if let Ok(code) = rt.desktop().last_error() {
        rt.remember_the_last_error(code);
    }
    let answer = answer.map_err(|failure| {
        trace_the_call(None);
        (
            failure.kind,
            RuntimeError {
                message: failure.reason,
                line,
            },
        )
    })?;
    // The arguments that are the address of a variable of the script come
    // back into the variable: the machine wrote into the memory of it.
    //
    // Measured on the binary of AutoHotkey v1.1.37.02: the
    // `DllCall("f", "Ptr", &v)` writes into the variable itself -- the
    // `v` of the script holds the bytes that the callee left there. The
    // type of the argument does not ask for the address of a value -- the
    // `*` of the v2 does -- and the address of the variable is the value
    // of the argument all the same.
    if rt.dialect().is_the_v1() {
        for argument in call.arguments.iter() {
            if !argument.kind.is_integer() {
                continue;
            }
            let NativeValue::Integer(address) = argument.value else {
                continue;
            };
            rt.the_variable_at_an_address_is_read_back(address);
        }
    }
    // The arguments that are the address of a variable come back into
    // the variable.
    for ((argument, reference), room) in call.arguments.iter().zip(references).zip(rooms) {
        // Only the argument that is the address of a variable comes back.
        if !argument.address {
            continue;
        }
        let Some(reference) = reference else { continue };
        let NativeValue::Bytes(bytes) = &argument.value else {
            continue;
        };
        let written = if argument.kind.is_text() {
            // The text that the machine left in the buffer of the variable
            // must be terminated inside the buffer: a text that is longer
            // is not a text of the variable. A text that is not the buffer
            // of a variable is a temporary, and the machine does not fail.
            //
            // Measured on the binary of AutoHotkey v2.0.32: the
            // `DllCall("GetCurrencyFormatW", "UInt", 0x0400, "UInt", 0,
            // "Str", "1149.99", "Ptr", 0, "Str", Text1, "Int", 10)` of a
            // script whose `Text1` holds "ab" is refused with "String not
            // null-terminated.".
            if room > 0
                && rt.dialect().the_native_call_checks_that_the_text_is_terminated()
                && !crate::dll::the_text_is_null_terminated(bytes, argument.kind, room)
            {
                return Err((
                    NativeFailureKind::Invalid,
                    RuntimeError {
                        message: "String not null-terminated.".into(),
                        line,
                    },
                ));
            }
            let text = crate::dll::decode(bytes, argument.kind);
            // The v1 keeps the length of the variable: the bytes of it
            // change and the text is the text that fits in it.
            //
            // Measured on the binary of v1.1.37.02: the
            // `DllCall("GetCurrencyFormatW", "UInt", 0x0400, "UInt", 0, "Str",
            // "1149.99", "Ptr", 0, "Str", x, "Int", 10)` of a script whose `x`
            // had a room of six characters leaves a variable of six
            // characters.
            let text = if rt.dialect().the_native_call_keeps_the_length_of_the_variable() {
                // The length of the variable is the room of its buffer, and
                // the text is the text that fits in the room.
                let length = match &reference {
                    Value::Reference(target) => {
                        let width = argument.kind.character_width().max(1);
                        let room = rt.the_room_of_the_buffer_of_a_variable(target) / width;
                        if room > 0 {
                            room
                        } else {
                            rt.read_reference(target)
                                .map(|value| value.to_text().chars().count())
                                .unwrap_or(0)
                        }
                    }
                    _ => 0,
                };
                text.chars().take(length).collect()
            } else {
                text
            };
            Value::Text(text.into())
        } else if argument.kind.is_float() {
            Value::Float(crate::dll::decode_number(bytes, argument.kind))
        } else {
            Value::Int(crate::dll::decode_number(bytes, argument.kind) as i64)
        };
        rt.write_reference(&reference, written);
    }
    let value = match answer.value {
        NativeValue::Integer(number) => Value::Int(number),
        NativeValue::Float(number) => Value::Float(number),
        NativeValue::Bytes(bytes) => Value::Text((crate::dll::decode(&bytes, answer.kind)).into()),
    };
    // The trace of the call: the answer of the call is the number it
    // returned, and a text is not a number.
    trace_the_call(Some(&value));
    Ok(value)
}

/// The target of a native call, as the script wrote it.
///
/// The target is the text of the first argument -- `user32\GetWindowRect`
/// -- and a call of an address has no name: the address is the target.
fn the_target_as_written(first: &Value) -> String {
    match first {
        Value::Text(text) => text.to_string(),
        other => other.to_text(),
    }
}

/// The arguments of a native call, as the script gave them.
///
/// The types of the arguments are not there: the values are, and the type
/// of the answer is not one of them.
fn the_arguments_of_the_call(values: &[Value]) -> Vec<Value> {
    let rest = &values[1..];
    // The type of the answer is the last argument when the count of the
    // parameters is odd: the types and the values alternate.
    let rest = if rest.len() % 2 == 1 {
        &rest[..rest.len() - 1]
    } else {
        rest
    };
    rest.iter().skip(1).step_by(2).cloned().collect()
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
) -> Result<(NativeArg, Option<Value>, usize), RuntimeError> {
    let (value, reference) = match value {
        // A parameter that is an alias of a variable of the caller is that
        // variable: the value that goes out is the value of the variable of
        // the caller, and the native call writes into it.
        //
        // Measured on the binary of v1.1.37.02: the
        // `DllCall("user32\GetWindowThreadProcessId", "UPtr", hwnd, "UInt*",
        // w)` of a `f(ByRef w)` writes into the variable of the caller -- the
        // width of the caller is the pid of the process after the call. The
        // parameter that is not `ByRef` keeps the write in the copy of it.
        Value::Reference(reference) => {
            let target = rt.the_variable_of_the_reference(reference);
            match rt.read_reference(&target) {
                Some(current) => (current, Some(Value::Reference(target))),
                // The built-in variables are not variables of the script:
                // they are read from the table of them, and the value of
                // them is what goes to the machine.
                //
                // Measured on the binary of the v1.1.37.02:
                // the DllCall of GetWindowThreadProcessId with
                // A_ScriptHwnd answers the thread of the window of the
                // script in the v1, and the tree answered zero.
                None => match crate::variables::lookup(&target.name, rt) {
                    Some(Ok(built_in)) => (built_in, None),
                    Some(Err(error)) => return Err(error),
                    // Measured on the binary of the v1.1.37.02: the
                    // variable that was never assigned is blank, and the native
                    // call that takes it as a number takes the zero of the blank.
                    None if rt.dialect().unset_is_blank() => {
                        (Value::Text("".into()), Some(Value::Reference(target)))
                    }
                    None => {
                        return Err(RuntimeError {
                            message: rt.the_unassigned(&target.name),
                            line,
                        });
                    }
                },
            }
        }
        other => (other.clone(), None),
    };
    // A text goes out as its bytes.
    if kind.is_text() {
        // The text of a variable is the buffer that the machine writes
        // into, and the number of a variable is the text of the number.
        //
        // Measured on the binary of AutoHotkey v2.0.32: the
        // `DllCall("kernel32\lstrlenW", "Str", x)` of a script whose `x`
        // holds the number 1149 answers four -- the length of the text of
        // the number -- and a number that is not the variable of the script
        // is refused with "Expected a String but got an Integer.".
        let text = match &value {
            Value::Text(text) => text.clone(),
            Value::Int(_) | Value::Float(_) if reference.is_some() => value.to_text().into(),
            _ => {
                return Err(RuntimeError {
                    message: expected_type(TypeName::String, value.type_name()),
                    line,
                });
            }
        };
        let length = crate::text16::the_code_units_of(&text).len();
        let mut bytes = crate::dll::encode(&text, kind);
        // The room of the buffer is the text of it, the terminator, and the
        // room that the allocator of the machine gives: the machine may
        // write past the text, and the room keeps the write from going over
        // the memory of the tree.
        bytes = crate::dll::the_room_of_a_buffer(length + 1, kind, bytes);
        // The limit of the text that the machine writes is the capacity of
        // the variable, and there is no limit at all when the text is a
        // temporary: the text of an expression is lost, and the machine does
        // not fail.
        //
        // Measured on the binary of AutoHotkey v2.0.32: the
        // `DllCall("GetCurrencyFormatW", "UInt", 0x0400, "UInt", 0, "Str",
        // "1149.99", "Ptr", 0, "Str", "x" . "y", "Int", Size)` of a script
        // answers nothing and does not fail.
        let mut limit = 0;
        // The room of the buffer of a variable is the capacity of it: the
        // `VarSetStrCapacity` of it asks for the room that the machine
        // writes into.
        //
        // Measured on the binary of AutoHotkey v2.0.32: the
        // `VarSetStrCapacity(&Text1, 10)` of a script gives the variable a
        // buffer of ten characters, and the machine writes its text into it.
        if let Some(Value::Reference(reference)) = &reference {
            let capacity = rt.string_capacity(reference).max(0) as usize;
            // The v1 gives the variable a buffer of its own, and the room of
            // it is the room of that buffer.
            let width = kind.character_width().max(1);
            let buffer = rt.the_room_of_the_buffer_of_a_variable(reference) / width;
            limit = capacity.max(buffer).max(length) + 1;
            bytes = crate::dll::the_room_of_a_buffer(limit, kind, bytes);
        }
        // The machine may write into the buffer of a variable, and the text
        // that it leaves there comes back into the variable.
        let address = address || reference.is_some();
        return Ok((
            NativeArg {
                kind,
                address,
                value: NativeValue::Bytes(bytes),
            },
            reference,
            limit,
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
                    0,
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
        0,
    ))
}

/// The value as a number, the way a native call wants it.
fn native_number(rt: &Runtime<'_>, value: &Value, line: u32) -> Result<f64, RuntimeError> {
    // Measured on the binary of v1.1.37.02: the parameter
    // that wants a number takes the text that is not a number, and the
    // number of it is the number at the start of the text -- a zero when
    // there is none. The `NULL` of a script of the v1 is the variable
    // that was never assigned, and the cursor of it is loaded.
    if rt.dialect().a_text_that_is_not_a_number_is_a_zero() {
        if let Value::Text(text) = value {
            return Ok(match crate::v1::the_number_at_the_start_of_a_text(text) {
                Some(Value::Int(number)) => number as f64,
                Some(Value::Float(number)) => number,
                _ => 0.0,
            });
        }
    }
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
///
/// No name of the v1 or of the v2 refuses at the load any more (the last
/// fifteen names that only the v1 has), and the handler is kept
/// for the next name that will need one: the surface has a state for it
/// and the reason of it is a row of the table of the built-ins.
#[allow(dead_code)]
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
    // Measured on the binary of AutoHotkey v2.0.19: the
    // variable that the environment does not have is an empty text, and it is
    // not a failure. Measured on the binary of AutoHotkey v1.1.37.02 the `EnvGet` of a name that is not there leaves the output
    // empty, and the `ErrorLevel` is zero.
    let value = rt
        .desktop()
        .env_get(&name)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?
        .unwrap_or_default();
    Ok(Value::Text(value.into()))
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
    Ok(Value::Text(value.into()))
}

// -- the files of the disk ----------------------------------------------------------

/// `FileExist`: the attributes of the first entry that matches.
///
/// v2 answers with the attributes of the file and not with a number: a
/// file that is there is `A`, a folder is `D`, and a file that is not
/// there is the empty text.
///
/// Measured on the binary of v2.0.19 and:
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
    Ok(Value::Text(attributes.concat().into()))
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
    Ok(Value::Text(attributes.concat().into()))
}

/// `FileDelete`: take a file away, or the files that a pattern names.
///
/// Measured on the binary of v2.0.19: the name may be a pattern
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
    // Measured on the binary of v2.0.19: the answer of the
    // command is an empty text.
    Ok(Value::Text("".into()))
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
    // Measured on the binary of v2.0.19: the answer of the
    // command is an empty text.
    Ok(Value::Text("".into()))
}

/// `DirDelete`: take a directory away.
///
/// Measured on the binary of v2.0.19: the second parameter
/// is the recursion, and a folder that is not empty stays without it.
///
/// Measured on the binary of v2.0.19: the name answers
/// nothing -- `r := DirDelete("empty")` answers the empty text -- and the
/// folder that is not empty fails with "Failed" without the recursion.
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
            message: if rt.dialect().is_the_v1() {
                reason
            } else {
                "Failed".to_string()
            },
            line,
        })?;
    Ok(Value::Text("".into()))
}

/// `FileCreateShortcut(LinkFile, Target, WorkingDir, Args, Description, Icon,
/// IconNumber, Run)`: make a shortcut to a program.
///
/// Measured on the binary of v2.0.19: `FileCreateShortcut`
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
    Ok(Value::Text("".into()))
}

/// `FileGetShortcut(LinkFile, &OutTarget, &OutDir, &OutArgs, &OutDesc,
/// &OutIcon, &OutIconNum, &OutRun)`: read a shortcut back.
///
/// Measured on the binary of v2.0.19: all eight fields of a
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
        ("target", Value::Text(target.into())),
        ("dir", Value::Text(dir.into())),
        ("args", Value::Text(args.into())),
        ("desc", Value::Text(desc.into())),
        ("icon", Value::Text(icon.into())),
        // A `.desktop` file has no icon index and no run state: the honest
        // answer is the empty text, not a number invented for the shape.
        ("iconnum", Value::Text("".into())),
        ("run", Value::Text("".into())),
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
                    name,
                    at + 1
                ),
                line,
            });
        }
        rt.write_reference(slot, value.clone());
    }
    Ok(Value::Text("".into()))
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
/// Measured on the binary of v2.0.19.
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
    let values: Vec<Value> = pieces.into_iter().map(Value::text).collect();
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
                        Value::Text(text) => Ok(text.to_string()),
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
    // Measured on the binary of v2.0.19: a text that is empty
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
        rt.write_reference(value, Value::Text(piece.into()));
    }
    Ok(Value::Text("".into()))
}

/// The pieces of a path, the way v2 cuts it.
///
/// The name of the file, the directory, the extension, the name
/// without the extension and the drive. A path of a file is cut at the
/// backslash and a URL at the slash, and v2 keeps that difference: the
/// backslash is the separator of a file and the slash is the one of a
/// URL.
pub(crate) fn split_the_path(path: &str) -> [String; 5] {
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
        // Measured on the binary of v2.0.19: the absent value is
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
/// Measured on the binary of AutoHotkey v2.0.19: the value is
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
        .unwrap_or(Value::Text("".into()));
    if !matches!(object, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjBindMethod", TypeName::Object, &object),
            line,
        });
    }
    let method = text_of(values, 1);
    Ok(rt.bound_function(object, Value::Text(method.into())))
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
        .unwrap_or(Value::Text("".into()));
    let name = text_of(values, 1);
    // Measured on the binary of v2.0.19: a value that is
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
        return rt.base_object(&target, "hasprop", &[Value::Text(name.into())], line);
    }
    rt.own_method(&target, "hasprop", &[Value::Text(name.into())], line)
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
        .unwrap_or(Value::Text("".into()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method, and the tree asks the method whether it
    // could take them.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    // Measured on the binary of v2.0.19: the question with
    // no name asks whether the value is a class, and the name is not there
    // to be asked about.
    let mut arguments = Vec::new();
    if values.len() > 1 {
        arguments.push(Value::Text(name.into()));
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
        .unwrap_or(Value::Text("".into()));
    let name = text_of(values, 1);
    // The third parameter of v2 is the number of parameters that would
    // be passed to the method.
    let count = values.get(2).map(|value| value.to_number().unwrap_or(0.0));
    let mut arguments = vec![Value::Text(name.into())];
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
        .unwrap_or(Value::Text("".into()));
    let base = values.get(1).cloned().unwrap_or(Value::Text("".into()));
    // Measured on the binary of v2.0.19: the base is an
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
    // The mode of v2 is the way the keys are sent, and the machine of
    // the tree has one way: the teclado.
    //
    // Measured on the binary of v2.0.31: the `SendMode`
    // answers the name of the mode -- "Input" -- e o "Event" e o "Play"
    // sao aceitos e dao o mesmo nome. Um modo que nao e um dos tres e
    // recusado com "Parameter #1 of SendMode is invalid.".
    let mode = text_of(values, 0).to_lowercase();
    match mode.as_str() {
        "input" | "event" | "play" => {
            rt.note("the keys go through the keyboard of the box, one at a time");
            Ok(Value::Text("Input".into()))
        }
        _ => Err(RuntimeError {
            message: "Parameter #1 of SendMode is invalid.".into(),
            line,
        }),
    }
}

/// The gap between the reads of the state of a key while a wait goes on.
const KEY_POLL_MS: u64 = 10;

/// The handle of a window, the way the dialect writes it.
///
/// The v1 writes it as a hexadecimal text -- the "0x77c0f70" -- and the
/// v2 writes it as a number.
///
/// Measured on the binary of v1.1.37.02: the handle is stored as a
/// hexadecimal text, "for backward compatibility, tradition, and the fact
/// that operations involving HWNDs tend not to be nearly as
/// performance-critical as pure-math expressions" (source/var.cpp of
/// v1.1.37.02, `Var::AssignHWND`, and the `HwndToString` of
/// source/util.h).
pub(crate) fn the_handle_of_the_machine(rt: &Runtime<'_>, handle: i64) -> Value {
    if rt.dialect().is_the_v1() {
        Value::Text((format!("0x{handle:x}")).into())
    } else {
        Value::Int(handle)
    }
}

/// `KeyWait`: wait for a key to be up, or to go down.
///
/// Measured on the binary of v1.1.37.02: the wait is for the key to be
/// released -- the "D" of the second parameter waits for it to go down
/// -- and the "T" of it is the timeout in seconds. The wait without a
/// timeout does not end, and the `ErrorLevel` is one when it ran out.
///
/// Measured on the binary of v1.1.37.02: the `KeyWait, a` of a key that
/// is not down ends at once, and the `KeyWait, a, D, T1` of a key that
/// does not go down ends after the second of the timeout, with the
/// `ErrorLevel` of one.
///
/// The state of the key is the one that the machine reads, the same one
/// that the `GetKeyState` reads: the "L" of the v1 asks for the logical
/// state -- the state of the key as the script sees it -- and the tree
/// does not keep it apart from the physical one.
///
/// The wait lets the timers of the script run, the way the wait of the
/// v1 does.
pub(crate) fn key_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    let options = text_of(values, 1);
    if crate::keys::resolve_name(&name).is_err() {
        return Err(RuntimeError {
            message: "Parameter #1 of KeyWait is invalid.".to_string(),
            line,
        });
    }
    // The "D" waits for the key to go down; the default waits for it to
    // come up.
    let wait_for_keydown = options.to_ascii_uppercase().contains('D');
    // The "T" is the timeout in seconds: the number that comes after it.
    let timeout = the_timeout_of_the_key_wait(rt, &options, line)?;
    let start = rt.now_ms();
    // The `ErrorLevel` is zero while the wait goes on.
    rt.set_the_error_level(Value::Int(0));
    loop {
        let down = rt
            .desktop()
            .key_is_down(&name)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
        // The key is in the state that is waited for: the wait is over.
        if down == wait_for_keydown {
            break;
        }
        // The timeout ran out: the `ErrorLevel` is one.
        if let Some(millis) = timeout {
            let left = millis.saturating_sub(rt.now_ms() - start);
            if left == 0 {
                rt.set_the_error_level(Value::Int(1));
                break;
            }
            // The wait does not go past the timeout.
            rt.the_run_waits_for(left.min(KEY_POLL_MS), line)
                .map_err(|failure| crate::runtime::as_error(failure, line))?;
            continue;
        }
        rt.the_run_waits_for(KEY_POLL_MS, line)
            .map_err(|failure| crate::runtime::as_error(failure, line))?;
    }
    Ok(Value::Text("".into()))
}

/// The timeout of the `KeyWait`, in milliseconds.
///
/// The "T" of the options of the command is the timeout in seconds, and
/// the number that comes after it is the one that the v1 reads: the
/// "T1" is a second, and the "T" alone is a timeout that is over.
///
/// Measured on the binary of AutoHotkey v2.0.19: a timeout that is
/// negative is refused with "Parameter #2 of KeyWait is invalid.". The
/// v1 has no refusal for it: the wait is over at once.
fn the_timeout_of_the_key_wait(
    rt: &Runtime<'_>,
    options: &str,
    line: u32,
) -> Result<Option<u64>, RuntimeError> {
    let upper = options.to_ascii_uppercase();
    let Some(at) = upper.find('T') else {
        // The wait without a timeout does not end.
        return Ok(None);
    };
    let rest = &options[at + 1..];
    let seconds = match crate::v1::the_number_at_the_start_of_a_text(rest) {
        Some(Value::Int(number)) => number as f64,
        Some(Value::Float(number)) => number,
        _ => 0.0,
    };
    if seconds < 0.0 && !rt.dialect().is_the_v1() {
        return Err(RuntimeError {
            message: "Parameter #2 of KeyWait is invalid.".to_string(),
            line,
        });
    }
    // A timeout that is over is a timeout that is over.
    Ok(Some(if seconds < 0.0 {
        0
    } else {
        (seconds * 1000.0) as u64
    }))
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
    // The command without a parameter names no timer, and the v1 refuses
    // it. The tree read the first parameter without looking, and the
    // script died with a panic.
    //
    // Measured on the binary of v1.1.37.02: the `SetTimer` without a
    // parameter is refused with "Parameter #1 must not be blank in this
    // case.".
    let Some(first) = values.first() else {
        return Err(RuntimeError {
            message: if rt.dialect().is_the_v1() {
                "Parameter #1 must not be blank in this case.".to_string()
            } else {
                "Too few parameters passed to function.".to_string()
            },
            line,
        });
    };
    // A prioridade do v2 e a ordem dos temporizadores: o menor numero
    // corre primeiro, e o de mesmo numero corre na ordem em que foi
    // criado.
    //
    // Medido no binario do v2.0.31: o `SetTimer(Tick, 100,
    // 5)` e o `SetTimer(Tick, 100, 0)` sao aceitos.
    let priority = number_of(values, 2) as i64;
    let name = the_name_of_the_timer(first);
    // The function of the timer may be a function of the script
    // that is bound to arguments: it has no name, and the timer is
    // the function itself.
    //
    // Measured on the binary of v1.1.37.02: the
    // `SetTimer, % fn, -5` of a script of the v1, where `fn` is a
    // function that is bound to arguments, runs the function after
    // five milliseconds, and the tree never ran it.
    let function = the_function_of_the_timer(first);
    // Read in the CHM of the v1.1.37.02 (`SetTimer.htm`): the period that
    // is not there is 250 milliseconds, a period below zero runs the
    // subroutine once, and the "Off" deletes the timer.
    if values.len() > 1 {
        let asked = values[1].to_text();
        let asked = asked.trim();
        if asked.eq_ignore_ascii_case("off") || asked.eq_ignore_ascii_case("delete") {
            rt.delete_timer(&name);
            return Ok(Value::Int(1));
        }
    }
    let period = number_of(values, 1);
    let (period, once) = if period > 0.0 {
        (period as u64, false)
    } else if period < 0.0 {
        ((-period) as u64, true)
    } else {
        (250, false)
    };
    rt.add_the_timer_with_a_function(&name, function, period, priority, once);
    Ok(Value::Int(1))
}

/// The name of the function a timer runs, read out of the value the script
/// gave.
///
/// v2 takes the function itself -- `SetTimer(Tick, 200)` -- and the tree
/// keeps the timers of the script by the **name** of the function, which it
/// looks up when the timer is due. The text of a function is not its name,
/// so the name of it is read out of the function, the same way the
/// `Hotkey` reads the name out of its callback
/// (`crate::hotkeys::action_of`). Before this, `SetTimer(Tick, 20)` kept a
/// timer under the text of the function, no name answered it, and the timer
/// never ran: a silent nothing where the script asked for a thing that runs
/// twice a second.
///
/// A text is the name the script wrote, and the v1 writes it that way
/// (`SetTimer, MyFunc, 1000`), so a text is kept as it is. A closure has no
/// name to look a function up by, and the tree has no room for one in a
/// timer: that hole is of the shape of the timer and not of this reading,
/// and it is written down with the timers of the script.
/// The function of the script that a timer runs, when the script
/// gave a function that has no name.
///
/// A function that is bound to arguments has no name: the timer of
/// it is the function itself, and the tree keeps it as it is.
fn the_function_of_the_timer(value: &Value) -> Option<Value> {
    let Value::Object(object) = value else {
        return None;
    };
    let class = object.borrow().class.clone();
    if class != "func" && class != "boundfunc" {
        return None;
    }
    let name = object
        .borrow()
        .value_of("name")
        .map(|name| name.to_text())
        .unwrap_or_default();
    if name.is_empty() {
        return Some(value.clone());
    }
    None
}

fn the_name_of_the_timer(value: &Value) -> String {
    if let Value::Object(object) = value {
        let object = object.borrow();
        if object.class == "func" || object.class == "boundfunc" {
            let name = object
                .value_of("name")
                .map(|name| name.to_text())
                .unwrap_or_default();
            if !name.is_empty() {
                return name;
            }
            // The timer of an object that has no name: the name of it in
            // the trace of the v1 is "Object" -- the name of the label of
            // the v1 that holds an object.
            //
            // Measured on the v1 that is built for the trace the trace of the timer of an object writes
            // "Object" as the name of it.
            return String::from("Object");
        }
    }
    value.to_text()
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
/// O volume, a velocidade e o tom do v2 entram, e a caixa toca o
/// arquivo como ele e: o volume e o tom sao do som que a maquina
/// toca, e nao ha um som aqui para mudar.
///
/// Medido no binario do v2.0.31: o `SoundPlay("zzz.wav",
/// 50)` recusa com "Failed", e o `SoundPlay("zzz.wav")` da a mesma
/// recusa.
pub(crate) fn sound_play(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    if path.is_empty() {
        return Err(RuntimeError {
            message: "the sound file is missing.".into(),
            line,
        });
    }
    // O som que a maquina nao toca recusa com "Failed", que e a
    // palavra do v2 -- e nao com a razao da caixa.
    rt.desktop()
        .sound_play(&path)
        .map_err(|_| RuntimeError {
            message: "Failed".into(),
            line,
        })?;
    Ok(Value::Int(1))
}

/// `SoundGetVolume`: the volume of a device, in percent.
///
/// O componente do v2 e uma parte do device -- o master, o microfone --
/// e o texto vazio e o master do device.
///
/// Medido no binario do v2.0.31: o
/// `SoundGetVolume("Microphone")` responde o volume do microfone, o
/// `SoundGetVolume("Master")` recusa com "Component not found" e o
/// `SoundGetVolume("1")` recusa com "Component doesn't support this
/// control type".
///
/// O device que nao existe recusa com "Device not found".
pub(crate) fn sound_get_volume(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let component = component_of(values, 0).map_err(|message| RuntimeError { message, line })?;
    let device = device_of(values, 1);
    let volume = rt
        .desktop()
        .sound_volume(&device, &component)
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
    let component = component_of(values, 1).map_err(|message| RuntimeError { message, line })?;
    let device = device_of(values, 2);
    if adjust {
        let current = rt
            .desktop()
            .sound_volume(&device, &component)
            .map_err(|e| desktop_failure(e, line))?;
        let wanted = (current + number).clamp(0.0, 100.0);
        rt.desktop()
            .sound_set_volume(&device, &component, wanted)
            .map_err(|e| desktop_failure(e, line))?;
    } else {
        rt.desktop()
            .sound_set_volume(&device, &component, number)
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
    let component = component_of(values, 0).map_err(|message| RuntimeError { message, line })?;
    let device = device_of(values, 1);
    let muted = rt
        .desktop()
        .sound_muted(&device, &component)
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
    let setting = number_of(values, 0);
    if !(-1.0..=1.0).contains(&setting) {
        return Err(RuntimeError {
            message: "the mute of a device is 1, 0 or -1.".into(),
            line,
        });
    }
    let component = component_of(values, 1).map_err(|message| RuntimeError { message, line })?;
    let device = device_of(values, 2);
    let muted = if setting < 0.0 {
        let current = rt
            .desktop()
            .sound_muted(&device, &component)
            .map_err(|e| desktop_failure(e, line))?;
        !current
    } else {
        setting != 0.0
    };
    rt.desktop()
        .sound_set_muted(&device, &component, muted)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(1))
}

/// The component of the sound that the script asks about.
///
/// v2 names a part of a device -- the master volume, the microphone --
/// and the box has one volume per device: the master one. A component
/// that is not the master fails with the reason.
/// The component that the script asks about.
///
/// The v2 does not take the named component: measured on the binary of
/// v2.0.31, the `SoundGetVolume("Master")` answers
/// `Component not found`, and the same holds for `Microphone`, `Line
/// in` and `Speakers`.
///
/// The numeric component is the place of the component in the list of
/// the connectors of the device, and the volume is not a component:
/// measured on the binary of v2.0.31, the
/// `SoundGetVolume("1")` answers `Component doesn't support this
/// control type`, and the `SoundGetVolume("2")` answers `Component not
/// found`.
///
/// The tree does not enumerate the connectors of the device, and so the
/// numeric component is never resolved: the first is the one that exists
/// and has no volume control, and the others do not exist.
///
/// The component that is not written is the master volume of the
/// device, and it is what the v2 answers.
fn component_of(values: &[Value], index: usize) -> Result<String, String> {
    let component = match values.get(index) {
        Some(value) => value.to_text(),
        None => return Ok(String::new()),
    };
    if component.is_empty() {
        return Ok(String::new());
    }
    // O componente numerico e o lugar do componente na lista dos
    // componentes do device, e a caixa o procura pelo numero: medido
    // no binario do v2.0.31, o `SoundGetVolume("1")` e o
    // primeiro componente da topologia.
    //
    // O componente nomeado e uma parte do device, e a caixa procura
    // ele pelo tipo dele.
    Ok(component)
}

/// The device that the script asks about.
///
/// v2 names a device by its name or by its number. The device of the
/// system is the empty name, and a number is the place of the device
/// in the list of every device of the machine.
///
/// The shape is the one of the v2.0.31, read in its source
/// (`source/lib/sound.cpp` of the reference tree): the number of the
/// device is a number -- the place of it in the list -- and the name of
/// it is a name, which may carry the number of the device among the
/// devices of that name after a colon.
fn device_of(values: &[Value], index: usize) -> String {
    match values.get(index) {
        Some(Value::Int(number)) => number.to_string(),
        Some(value) => value.to_text(),
        None => String::new(),
    }
}

// -- the suspend and the pause ----------------------------------------------------

/// The number of `Suspend` and of `Pause`: on, off, or the toggle of the
/// omitted one.
///
/// Measured on the binary of v2.0.19: the parameter is a Number,
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
    Ok(Value::Text("".into()))
}

/// `Pause`: the script paused, or not.
///
/// Measured on the binary of v2.0.19: the shapes of the
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
    Ok(Value::Text("".into()))
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
/// The tables were measured on the binary of v2.0.19:
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
/// Measured on the binary of v2.0.19: every name that the
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
    // there (measured on the binary of v2.0.19).
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
    // Measured on the binary of v2.0.19: a value that is not
    // a reference is refused with "requires a variable reference, but
    // received a ...", and the failure carries no name of the built-in in
    // its `What` -- the caller's check makes it, and not the built-in.
    ("RegExMatch", 2, Wants::Reference("a variable reference")),
    ("RegExReplace", 3, Wants::Reference("a variable reference")),
    ("StrReplace", 4, Wants::Reference("a variable reference")),
    (
        "VarSetStrCapacity",
        0,
        Wants::Reference("a variable reference"),
    ),
];

/// O(1) lookup table for `wants(name, index)` under `std`.
///
/// Built once per process from `THE_TEXTS`, `THE_NUMBERS`, `THE_REFUSED`, and
/// the dynamic arms below so hot built-ins like `NumGet` do not linearly scan
/// ~300 entries on every argument of every call.
#[cfg(feature = "std")]
struct WantsTable {
    entries: std::collections::HashMap<
        &'static str,
        [Wants; 8],
        crate::fast_hash::FastHasherBuilder,
    >,
}

#[cfg(feature = "std")]
fn wants_table() -> &'static WantsTable {
    static TABLE: std::sync::OnceLock<WantsTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        // The hash of the names of the built-ins is a fast one: the table
        // is looked up in every call.
        let mut entries: std::collections::HashMap<
            &'static str,
            [Wants; 8],
            crate::fast_hash::FastHasherBuilder,
        > = std::collections::HashMap::with_hasher(crate::fast_hash::FastHasherBuilder::default());
        for &(built_in, indexes) in THE_TEXTS {
            let row = entries.entry(built_in).or_insert([Wants::Value; 8]);
            for &place in indexes {
                if (place as usize) < 8 {
                    row[place as usize] = Wants::Declared(TypeName::String);
                }
            }
        }
        for &(built_in, indexes) in THE_NUMBERS {
            let row = entries.entry(built_in).or_insert([Wants::Value; 8]);
            for &place in indexes {
                if (place as usize) < 8 && matches!(row[place as usize], Wants::Value) {
                    row[place as usize] = Wants::Declared(TypeName::Number);
                }
            }
        }
        for &(built_in, place, kind) in THE_REFUSED {
            let row = entries.entry(built_in).or_insert([Wants::Value; 8]);
            if place < 8 && matches!(row[place], Wants::Value) {
                row[place] = kind;
            }
        }
        for built_in in ["ObjOwnProps", "ObjOwnPropCount", "ObjHasOwnProp", "ObjSetBase"] {
            let row = entries.entry(built_in).or_insert([Wants::Value; 8]);
            if matches!(row[0], Wants::Value) {
                row[0] = Wants::Checked(TypeName::Object);
            }
        }
        for built_in in ["MonitorGet", "MonitorGetWorkArea"] {
            let row = entries.entry(built_in).or_insert([Wants::Value; 8]);
            for slot in &mut row[1..=4] {
                if matches!(slot, Wants::Value) {
                    *slot = Wants::Declared(TypeName::VarRef);
                }
            }
        }
        WantsTable { entries }
    })
}

/// Whether the built-in `name` declares any parameter type constraint in `wants`.
#[inline]
pub fn has_wants(name: &str) -> bool {
    #[cfg(feature = "std")]
    {
        if matches!(name, "Min" | "Max") {
            return true;
        }
        wants_table().entries.contains_key(name)
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = name;
        true
    }
}

pub fn wants(name: &str, index: usize) -> Wants {
    if matches!(name, "Min" | "Max") {
        return Wants::Declared(TypeName::Number);
    }
    #[cfg(feature = "std")]
    if index < 8 {
        return wants_table()
            .entries
            .get(name)
            .map(|row| row[index])
            .unwrap_or(Wants::Value);
    }
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
        ("ObjOwnProps" | "ObjOwnPropCount" | "ObjHasOwnProp" | "ObjSetBase", 0) => {
            Wants::Checked(TypeName::Object)
        }
        ("MonitorGet" | "MonitorGetWorkArea", 1..=4) => Wants::Declared(TypeName::VarRef),
        _ => Wants::Value,
    }
}

/// The built-ins that refuse the parameter themselves.
///
/// Measured on the binary of v2.0.19: the `Mod` says
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
    let mut target = values
        .first()
        .cloned()
        .unwrap_or(Value::Text("".into()));
    // Measured on the binary of v2.0.19: the reference that the caller gives
    // is dereferenced -- the ObjGetBase(&y) reads y -- and a y that is unset
    // is an empty object whose base is the prototype.
    if let Value::Reference(reference) = &target {
        target = rt
            .read_reference(reference)
            .unwrap_or_else(|| crate::classes::object_with("object", vec![]));
        if !matches!(target, Value::Object(_)) {
            target = crate::classes::object_with("object", vec![]);
        }
    }
    rt.the_base_of_the_value(&target).ok_or_else(|| RuntimeError {
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
        .unwrap_or(Value::Text("".into()));
    let base = values.get(1).cloned().unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
    let name = values.get(1).cloned().unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
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
        .unwrap_or(Value::Text("".into()));
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
///
/// Measured on the binary of v1.1.37.02: the `ObjAddRef` of a value that
/// is not an object answers the empty text, and it does not throw.
pub(crate) fn obj_add_ref(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if rt.dialect().is_the_v1() {
        return Ok(Value::Text("".into()));
    }
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
pub(crate) fn number_value(value: &Value) -> Option<Value> {
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
/// Measured on the binary of v2.0.19: a value is a number when
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
/// Measured on the binary of v2.0.19: a float is cut toward
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
/// The rule is the one of `nstrtoi64` of the binary, measured:
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
/// The rule is the one of `ATOF` of the binary, measured: a hex
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
/// Measured on the binary of v2.0.19: `Number("0x10")` is the
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
/// Measured on the binary of v2.0.19: the parameter takes a
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
        CaseSense::Insensitive | CaseSense::Locale => {
            left.to_lowercase().cmp(&right.to_lowercase())
        }
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
pub(crate) fn desktop_failure(message: String, line: u32) -> RuntimeError {
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
/// Measured on the binary of v2.0.19: with a positive number of
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
        return Ok(Value::Text(body.into()));
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
    let source = values.first().unwrap_or(&Value::Int(0));
    if !matches!(source, Value::Object(_) | Value::Int(_)) {
        let message = expected_type(TypeName::Object, source.type_name());
        return Err(RuntimeError { message, line });
    }
    // The offset is the second parameter when the type is the third.
    let (offset, type_index) = if values.len() > 2 {
        (integer_argument(values, 1, line)?, 2)
    } else {
        (0, 1)
    };
    let kind = match values.get(type_index) {
        Some(Value::Text(text)) => crate::classes::buffer::number_type(text),
        Some(other) => crate::classes::buffer::number_type(&other.to_text()),
        None => None,
    }
    .ok_or_else(|| RuntimeError {
        message: "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.".into(),
        line,
    })?;
    // The source is a buffer, or the address of one: the v2 reads the
    // memory of the address, and the tree reads the buffer whose mirror
    // holds it. Measured on the binary of v2.0.32: the
    // `NumGet(Buf.Ptr + 4, 0, "UInt")` reads the number at the fourth
    // byte of the buffer.
    let result = match source {
        Value::Object(_) => crate::classes::buffer::read_number(source, offset, kind, line),
        Value::Int(address) => {
            match crate::classes::buffer::read_number_at_address(*address, offset, kind, line) {
                Some(result) => result,
                None => {
                    return Err(RuntimeError {
                        message: "this runtime does not read the memory of an address: the source of a number is a buffer.".to_string(),
                        line,
                    });
                }
            }
        }
        other => {
            let message = expected_type(TypeName::Object, other.type_name());
            return Err(RuntimeError { message, line });
        }
    };
    match result {
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
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The target is the last object of the arguments: the pairs come
    // before it, and the offset comes after it.
    // The target of a write is a buffer, or the address of one: the v2
    // writes the memory of the address, and the tree writes the buffer
    // whose mirror holds it. Measured on the binary of v2.0.32: the
    // `NumPut("UInt", 1, Buf.Ptr + 4)` writes the number at the fourth
    // byte of the buffer.
    let index = match values
        .iter()
        .rposition(|value| matches!(value, Value::Object(_)))
    {
        Some(index) => index,
        None => {
            // The address of a buffer is the address of the memory of it,
            // and the tree finds the buffer by it: the memory of the
            // machine is not there without the standard library.
            #[cfg(feature = "std")]
            let found = values.iter().enumerate().find(|(_, value)| {
                matches!(value, Value::Int(address) if crate::memory::the_mirror_at(*address).is_some())
            });
            #[cfg(not(feature = "std"))]
            let found: Option<(usize, &Value)> = None;
            match found {
                Some((index, _)) => index,
                None => {
                    let message = match values.last() {
                        Some(Value::Int(_)) => "this runtime does not write to the memory of an address: the target of a number is a buffer.".to_string(),
                        Some(other) => expected_type(TypeName::Object, other.type_name()),
                        None => expected_type(TypeName::Object, TypeName::String),
                    };
                    return Err(RuntimeError { message, line });
                }
            }
        }
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
    // The types and the numbers of the write live in the vector of the
    // runtime: the vector of the pairs is an allocation in every call of
    // the `NumPut` otherwise, and the capacity of it is kept from one
    // call to the next.
    rt.scratch_numbers.clear();
    let mut at = 0;
    while at < pairs.len() {
        // The name of the type is read without a copy of it: the type of a
        // number is a name of the script, and the script writes the same
        // name in every write.
        let kind = match pairs.get(at) {
            Some(Value::Text(text)) => crate::classes::buffer::number_type(text),
            Some(other) => crate::classes::buffer::number_type(&other.to_text()),
            None => None,
        }
        .ok_or_else(|| RuntimeError {
            message: "the type of a number is one of UInt, Int, Int64, Short, UShort, Char, UChar, Double, Float, Ptr or UPtr.".into(),
            line,
        })?;
        let number = pairs[at + 1].clone();
        let number = number_value(&number).ok_or_else(|| RuntimeError {
            message: crate::runtime::requires(at + 2, "NumPut", TypeName::Number, &number),
            line,
        })?;
        rt.scratch_numbers.push((kind, number));
        at += 2;
    }
    // The target is the buffer that the address names, and the offset of
    // it is the distance from the start of it.
    let (target, offset) = match &values[index] {
        Value::Int(address) => match crate::classes::buffer::the_buffer_at(*address, line) {
            Some((buffer, distance)) => (buffer, offset + distance),
            None => {
                let message = "this runtime does not write to the memory of an address: the target of a number is a buffer.".to_string();
                return Err(RuntimeError { message, line });
            }
        },
        other => (other.clone(), offset),
    };
    match crate::classes::buffer::write_numbers(&target, offset, &rt.scratch_numbers, line) {
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
/// Measured on the binary of v2.0.19: the parameter wants
/// a number and not a whole one, and the point that is not whole is the
/// one before it.
pub(crate) fn chr(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let code = number_argument(values, 0, line, "Chr")? as i64;
    // Measured on the binary of v2.0.19: a surrogate code unit
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
    Ok(Value::Text(text.into()))
}

/// `Format`: a pattern with the values in it.
///
/// Measured on the binary of v2.0.19: the pattern is the one of
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
        return Ok(Value::Text("".into()));
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
                .unwrap_or_else(|| Value::Text("".into())),
            other => other.clone(),
        });
    }
    let pattern = match arguments.first() {
        Some(value @ (Value::Object(_) | Value::List(_))) => {
            return Err(RuntimeError {
                message: crate::runtime::requires(1, "Format", TypeName::String, value),
                line,
            });
        }
        Some(value) => value.to_text(),
        None => String::new(),
    };
    Ok(Value::Text((format_pattern(&pattern, &arguments, line)?).into()))
}

/// `InStr`: where the needle is in the haystack.
///
/// The position counts from one, and it is zero when the needle is not
/// there at all. The third parameter is the sense of the case, the fourth is
/// the place where the search begins and the fifth is the occurrence the
/// caller wants.
///
/// Measured on the binary of v2.0.19, and read out of the
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
    let hay_owned;
    let hay_str: &str = match values.first() {
        Some(Value::Text(s)) => s.as_str(),
        _ => {
            hay_owned = text_of(values, 0);
            hay_owned.as_str()
        }
    };
    let needle_owned;
    let needle_str: &str = match values.get(1) {
        Some(Value::Text(s)) => s.as_str(),
        _ => {
            needle_owned = text_of(values, 1);
            needle_owned.as_str()
        }
    };
    if needle_str.is_empty() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "InStr"),
            line,
        });
    }
    let sense = if values.len() <= 2 {
        CaseSense::Insensitive
    } else {
        let s = case_sense(values, 2, "InStr", line)?;
        if s == CaseSense::Logical {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(3, "InStr"),
                line,
            });
        }
        s
    };
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
    // Zero-alloc ASCII fast path: bytes map 1:1 to characters.
    if hay_str.is_ascii() && needle_str.is_ascii() {
        let hay_bytes = hay_str.as_bytes();
        let needle_bytes = needle_str.as_bytes();
        let length = hay_bytes.len() as i64;
        if offset < 0 {
            if values.get(4).is_none() {
                occurrence = -1;
            }
            offset = offset.saturating_add(length + 1);
        }
        if occurrence > 0 {
            offset = offset.saturating_sub(1);
        }
        offset = offset.clamp(0, length);
        let nlen = needle_bytes.len();
        if occurrence < 0 {
            let end = if start_given {
                offset as usize
            } else {
                hay_bytes.len()
            };
            let wanted = (-occurrence) as u64;
            let mut found = 0u64;
            if nlen <= end {
                for at in (0..=(end - nlen)).rev() {
                    let matched = match sense {
                        CaseSense::Sensitive => &hay_bytes[at..at + nlen] == needle_bytes,
                        _ => hay_bytes[at..at + nlen].eq_ignore_ascii_case(needle_bytes),
                    };
                    if matched {
                        found += 1;
                        if found == wanted {
                            return Ok(Value::Int(at as i64 + 1));
                        }
                    }
                }
            }
            return Ok(Value::Int(0));
        }
        let mut at = offset as usize;
        if occurrence == 1 {
            if matches!(sense, CaseSense::Sensitive) {
                if let Some(pos) = hay_str[at..].find(needle_str) {
                    return Ok(Value::Int((at + pos) as i64 + 1));
                }
                return Ok(Value::Int(0));
            }
            let first_lo = needle_bytes[0].to_ascii_lowercase();
            let first_hi = needle_bytes[0].to_ascii_uppercase();
            let limit = hay_bytes.len().saturating_sub(nlen);
            if nlen <= hay_bytes.len() {
                while at <= limit {
                    let b = hay_bytes[at];
                    if (b == first_lo || b == first_hi)
                        && hay_bytes[at..at + nlen].eq_ignore_ascii_case(needle_bytes)
                    {
                        return Ok(Value::Int(at as i64 + 1));
                    }
                    at += 1;
                }
            }
            return Ok(Value::Int(0));
        }
        let mut count: i64 = 1;
        while at + nlen <= hay_bytes.len() {
            let matched = match sense {
                CaseSense::Sensitive => &hay_bytes[at..at + nlen] == needle_bytes,
                _ => hay_bytes[at..at + nlen].eq_ignore_ascii_case(needle_bytes),
            };
            if matched {
                if count == occurrence {
                    return Ok(Value::Int(at as i64 + 1));
                }
                count += 1;
                at += nlen;
            } else {
                at += 1;
            }
        }
        return Ok(Value::Int(0));
    }
    let haystack: Vec<char> = hay_str.chars().collect();
    let needle: Vec<char> = needle_str.chars().collect();
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
        let end = if start_given {
            offset as usize
        } else {
            haystack.len()
        };
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
    // Measured on the binary of v2.0.19: the code of the first
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
/// Measured on the binary of v2.0.19, and read out of the
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
/// Measured on the binary of v2.0.19, and read out of the
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
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => {
            Some(value.clone())
        }
        Some(_) => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(3, "Sort"),
                line,
            });
        }
    };
    if contents.is_empty() {
        return Ok(Value::Text("".into()));
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
        // item: measured, "B`r`nA" sorts to "A`r`nB".
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
        return Ok(Value::Text(contents.into()));
    }
    // Where each item begins in the text being sorted: the callback is told
    // the distance between the two items it compares.
    let mut positions: Vec<usize> = Vec::with_capacity(items.len());
    let mut position = 0usize;
    for item in &items {
        positions.push(position);
        position += item.chars().count() + 1;
    }
    let keys: Vec<String> = items.iter().map(|item| sort_key(item, &options)).collect();
    let mut order: Vec<usize> = (0..items.len()).collect();
    let mut failure: Option<RuntimeError> = None;
    if let Some(callback) = &callback {
        // A function is called as a value, and any other object is asked for
        // its `Call` method: measured on the binary of v2.0.19,
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
                Value::Text(items[left].clone().into()),
                Value::Text(items[right].clone().into()),
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
    Ok(Value::Text(out.into()))
}

/// The text of one parameter of a `Sort`.
fn sort_text(values: &[Value], index: usize, line: u32) -> Result<String, RuntimeError> {
    match values.get(index) {
        None => Ok(String::new()),
        // Measured on the binary of v2.0.19: the absent value is
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
    // Measured on the binary of v2.0.19: the length is counted in
    // the code units of UTF-16, so `StrLen(Chr(0x1F600))` is two.
    if let Some(Value::Text(s)) = values.first() {
        let len = if s.is_ascii() {
            s.len()
        } else {
            crate::text16::the_number_of_the_code_units(s)
        };
        return Ok(Value::Int(len as i64));
    }
    let text = text_of(values, 0);
    let len = if text.is_ascii() {
        text.len()
    } else {
        crate::text16::the_number_of_the_code_units(&text)
    };
    Ok(Value::Int(len as i64))
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
    Ok(Value::Text((if variant == 1 {
        text.to_uppercase()
    } else {
        text.to_lowercase()
    }).into()))
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
    // Measured on the binary of v2.0.19: the fourth parameter of
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
    // Measured on the binary of v2.0.19: a needle that is
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
            return Ok(Value::Text(out.into()));
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
    Ok(Value::Text(out.into()))
}

/// The encodings the string functions of v2 name.
///
/// Measured on the binary of v2.0.19: `UTF-8`, `UTF-16` and
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
        // The name of the encoding is read without a copy of it: a script
        // that reads a thousand texts of the same encoding does not make a
        // thousand lower case copies of it.
        Some(Value::Text(text)) => {
            if text.eq_ignore_ascii_case("utf-8") {
                Ok(TextEncoding::Utf8)
            } else if text.eq_ignore_ascii_case("utf-16") {
                Ok(TextEncoding::Utf16)
            } else if text.eq_ignore_ascii_case("cp0") {
                Ok(TextEncoding::Cp0)
            } else {
                Err("Invalid Encoding.".to_string())
            }
        }
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
/// v2.0.19: `StrGet(buf, 2, "UTF-8")` answers the first two
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
            // The fast path: the units of ASCII are one byte each, and the
            // text of them goes straight into the string -- no vector of
            // units in between.
            let mut end = 0;
            let mut ascii = true;
            while end + 1 < bytes.len() {
                let unit = u16::from_le_bytes([bytes[end], bytes[end + 1]]);
                if unit == 0 {
                    break;
                }
                if unit > 0x7f {
                    ascii = false;
                    break;
                }
                end += 2;
                if let Some(length) = length {
                    if end / 2 >= length {
                        break;
                    }
                }
            }
            if ascii {
                let mut text = String::with_capacity(end / 2);
                let mut at = 0;
                while at < end {
                    text.push(bytes[at] as char);
                    at += 2;
                }
                return text;
            }
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
/// Measured on the binary of v2.0.19: the parameter is a
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
            });
        }
        None => crate::runtime::Text::new(),
    };
    Ok(Value::Int(rt.the_address_of_a_text(&text)))
}

/// `StrGet`: the text at an address, or the text of a buffer.
///
/// Measured on the binary of v2.0.19: the source is a buffer
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
    // The second parameter is the length when it is a number, and the
    // encoding when it is a text; the third one is the encoding.
    let (length, encoding_name) = match values.get(1) {
        Some(value) if is_numeric_value(value) => {
            (Some(token_to_int64(value).max(0) as usize), values.get(2))
        }
        Some(value) => (None, Some(value)),
        None => (None, None),
    };
    // A text is not a source of a `StrGet`: the v2 refuses it, whatever the
    // length is.
    //
    // Measured on the binary of v2.0.32: the `StrGet("abc", 0)` is refused
    // with "Parameter #1 invalid.", and the failure is of the class
    // `ValueError`.
    if matches!(source, Value::Text(_)) {
        return Err(RuntimeError {
            message: "Parameter #1 invalid.".to_string(),
            line,
        });
    }
    // A length of zero reads nothing: the source is not even looked at, and
    // the answer is the empty text.
    //
    // Measured on the binary of v2.0.32: the `StrGet(0, 0)` of a null
    // address is the empty text, and the `StrGet(0)` of it is refused with
    // "Parameter #1 of StrGet is invalid.".
    if length == Some(0) && matches!(source, Value::Int(_) | Value::Object(_)) {
        return Ok(Value::Text("".into()));
    }
    // Zero-copy fast path for live `Buffer` instances: borrow the backing byte
    // slice directly instead of cloning the whole buffer into a `Vec<u8>`.
    if let Some(cell) = crate::classes::buffer::live_cell(source) {
        let encoding =
            the_encoding_of(encoding_name).map_err(|message| RuntimeError { message, line })?;
        let block = cell.borrow();
        return Ok(Value::Text(the_text_of(&block, encoding, length).into()));
    }
    let bytes: Vec<u8> = match source {
        // The bytes of a buffer are the bytes of the buffer.
        Value::Object(object) => {
            if object.borrow().class.eq_ignore_ascii_case("buffer") {
                crate::classes::buffer::raw_bytes(source, line).map_err(|failure| RuntimeError {
                    message: the_message_of(failure),
                    line,
                })?
            } else {
                // The v2 reads the .Ptr of the object: a plain object has
                // none, and the read is a PropertyError. An object that has
                // one gives the address to read.
                let ptr = rt.read_property(source, "Ptr", line).map_err(|failure| {
                    RuntimeError {
                        message: the_message_of(failure),
                        line,
                    }
                })?;
                match ptr {
                    Value::Int(address) => match rt.the_text_at_an_address(address) {
                        Some(text) => the_bytes_of(&text, TextEncoding::Utf16),
                        None => match crate::classes::buffer::the_buffer_at(address, line) {
                            Some((buffer, distance)) => crate::classes::buffer::raw_bytes_from(
                                &buffer,
                                distance.max(0) as usize,
                                line,
                            )
                            .map_err(|failure| RuntimeError {
                                message: the_message_of(failure),
                                line,
                            })?,
                            None => match rt.the_buffer_at(address) {
                                Some(buffer) => crate::classes::buffer::raw_bytes(&buffer, line)
                                    .map_err(|failure| RuntimeError {
                                        message: the_message_of(failure),
                                        line,
                                    })?,
                                None => return Ok(Value::Text("".into())),
                            },
                        },
                    },
                    _ => {
                        return Err(RuntimeError {
                            message: crate::runtime::invalid_parameter(1, "StrGet"),
                            line,
                        });
                    }
                }
            }
        }
        // The address of a text of the script: the text is a UTF-16 text,
        // and the reading of it with another encoding reads the bytes of
        // it, the way the binary reads the buffer of the variable.
        //
        // The address of a buffer is the address of the storage of it, and
        // the runtime remembers the buffer under that address: `StrPut` into
        // a buffer and `StrGet` of the address of it (including an interior
        // address such as `Buf.Ptr + offset` or a pointer returned by
        // `NtQueryObject` inside `Buf`) read the same bytes back. Measured on
        // the binary of v2.0.32.
        Value::Int(address) => match rt.the_text_at_an_address(*address) {
            Some(text) => the_bytes_of(&text, TextEncoding::Utf16),
            None => match crate::classes::buffer::the_buffer_at(*address, line) {
                Some((buffer, distance)) => crate::classes::buffer::raw_bytes_from(
                    &buffer,
                    distance.max(0) as usize,
                    line,
                )
                .map_err(|failure| RuntimeError {
                    message: the_message_of(failure),
                    line,
                })?,
                None => match rt.the_buffer_at(*address) {
                    Some(buffer) => {
                        crate::classes::buffer::raw_bytes(&buffer, line).map_err(|failure| {
                            RuntimeError {
                                message: the_message_of(failure),
                                line,
                            }
                        })?
                    }
                    None => {
                        // Measured on the binary of v2.0.31: an address in the
                        // null page of the machine (the first 64 KiB, which the
                        // system reserves) is invalid -- `StrGet(0)` and
                        // `StrGet(1)` are "Parameter #1 of StrGet is invalid."
                        // An address above it that names no buffer and no text is
                        // the empty text here; the real binary reads unmapped
                        // memory and hangs there, and the tree does not reproduce
                        // the hang.
                        if (0..0x10000).contains(address) {
                            return Err(RuntimeError {
                                message: crate::runtime::invalid_parameter(1, "StrGet"),
                                line,
                            });
                        }
                        return Ok(Value::Text("".into()));
                    }
                },
            },
        },
        _ => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(1, "StrGet"),
                line,
            });
        }
    };
    let encoding =
        the_encoding_of(encoding_name).map_err(|message| RuntimeError { message, line })?;
    Ok(Value::Text(the_text_of(&bytes, encoding, length).into()))
}

/// `StrPut`: write a text into a buffer, or say how many bytes it needs.
///
/// Measured on the binary of v2.0.19: the parameters are the
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
            });
        }
        Some(value) => value.to_text().into(),
        None => crate::runtime::Text::new(),
    };
    // The parameters, in the two orders that the binary takes.
    let (target, encoding_name, length_name) = match values.get(1) {
        None => (None, values.get(2), values.get(3)),
        Some(value @ Value::Object(_)) => {
            // The encoding of the write is the third parameter, and the
            // fourth when the third is empty: measured on the binary of
            // v2.0.19, `StrPut("abc", buf, "UTF-8")` and
            // `StrPut("abc", buf, , "UTF-8")` both answer 4, the bytes of
            // the UTF-8.
            let empty = values
                .get(2)
                .map(|value| value.to_text().is_empty())
                .unwrap_or(true);
            if empty {
                (Some(value), values.get(3), None)
            } else {
                (Some(value), values.get(2), values.get(3))
            }
        }
        Some(value @ Value::Text(_)) => (None, Some(value), values.get(2)),
        Some(_) => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(2, "StrPut"),
                line,
            });
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
            });
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
pub(crate) fn the_message_of(failure: crate::runtime::RuntimeFailure) -> String {
    match failure {
        crate::runtime::RuntimeFailure::Error(error) => error.message,
        crate::runtime::RuntimeFailure::Detailed(error, _) => error.message,
        _ => "the runtime cannot answer that.".to_string(),
    }
}

/// `VarSetStrCapacity`: the room of the string of a variable.
///
/// Measured on the binary of v2.0.19, with the source of the
/// binary read beside the measurements:
///
/// - with a capacity the variable is emptied -- the buffer of it is made
///   again, and the text in it is the text of nothing -- and the answer is
///   the capacity of the variable, which is the one asked for when it is
///   greater than the one the variable had;
/// - with zero the text is emptied and the answer is the room that the
///   variable has, and the room of it is the one it had.
///
///   Measured on the binary of AutoHotkey v2.0.32: the
///   `VarSetStrCapacity(&x, 0)` of a script whose `x` had a room of ten
///   characters answers ten, and the room of the variable is still ten
///   after it;
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
    // The variable is emptied by the call, whether the capacity grows or
    // not: the buffer that holds it is made again.
    rt.write_reference(value, Value::Text("".into()));
    let capacity = if requested > 0 {
        rt.set_string_capacity(reference, requested.max(current));
        requested.max(current)
    } else {
        // The zero does not free the room of the variable: the answer is
        // the room that the variable has, and the room of it is the one it
        // had.
        //
        // Measured on the binary of AutoHotkey v2.0.32: the
        // `VarSetStrCapacity(&x, 0)` of a script whose `x` had a room of ten
        // characters answers ten, and the room of the variable is still ten
        // after it.
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
    // Measured on the binary of v2.0.19: the text is cut in the
    // code units of UTF-16, so the second unit of a surrogate pair is a text of
    // its own -- `Ord(SubStr(Chr(0x1F600), 2))` is 0xDE00.
    let owned;
    let raw: &str = match values.first() {
        Some(Value::Text(s)) => s.as_str(),
        _ => {
            owned = text_of(values, 0);
            owned.as_str()
        }
    };
    let start = number_of(values, 1) as i64;
    let wanted_len = values
        .get(2)
        .and_then(|value| value.to_number())
        .map(|n| n as i64);
    // Zero-alloc UTF-16 fast path for ASCII strings: UTF-16 code units == bytes.
    if raw.is_ascii() {
        let len = raw.len();
        let from = start_index(start, len).min(len);
        let to = match wanted_len {
            Some(length) if length < 0 => len.saturating_sub((-length) as usize).max(from),
            Some(length) => (from + length as usize).min(len),
            None => len,
        };
        return Ok(Value::Text(raw[from..to].into()));
    }
    let text = crate::text16::the_code_units_of(raw);
    let from = start_index(start, text.len()).min(text.len());
    let sliced: Vec<u16> = match wanted_len {
        Some(length) => {
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
    Ok(Value::Text((crate::text16::the_text_of(&sliced)).into()))
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
    Ok(Value::Text(characters[start..end].iter().collect::<String>().into()))
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
    let name = the_name_of_the_variant_of_is_value(variant);
    // O segundo parametro do v2 e o tipo do valor, e nenhum dos nomes
    // que o binario aceita e um deles: medido no binario do v2.0.31 em
    // O `IsAlnum("a", "Integer")` e o `IsAlnum("a", "Any")`
    // recusam com "Parameter #2 of IsAlnum is invalid.".
    //
    // Os que tem o segundo parametro sao o `IsAlnum`, o `IsAlpha`, o
    // `IsLower` e o `IsUpper`, e o nome do que recusa e o nome deles.
    if values.len() > 1 {
        return Err(RuntimeError {
            message: format!("Parameter #2 of {name} is invalid."),
            line,
        });
    }
    let value = values
        .first()
        .cloned()
        .unwrap_or(Value::Text("".into()));
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

/// The name of the built-in of the family of the `Is`, pelo seu numero.
///
/// O nome e o que a recusa do segundo parametro diz, e ele e o nome do
/// built-in da tabela.
fn the_name_of_the_variant_of_is_value(variant: u8) -> &'static str {
    match variant {
        1 => "IsNumber",
        2 => "IsInteger",
        3 => "IsFloat",
        4 => "IsTime",
        5 => "IsDigit",
        6 => "IsXDigit",
        7 => "IsAlnum",
        8 => "IsAlpha",
        9 => "IsUpper",
        10 => "IsLower",
        _ => "IsSpace",
    }
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
    // Measured on the binary of v2.0.19: everything that is
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
            return Ok(Value::Text(class.into()));
        }
    }
    Ok(Value::Text(name.into()))
}

/// `IsSet`: whether the variable of the caller has a value.
///
/// Measured on the binary of v2.0.19: a variable that has no
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
        Value::Text(text) => Ok(text.to_string()),
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
/// Measured on the binary of v2.0.19, with the source of it
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
    // The output variable, when the caller gives one. An omitted parameter
    // (`RegExMatch(str, pat, , pos)`) arrives as `Unset`; in v1, any
    // non-variable expression (such as `""` or `0`) is ignored rather than
    // throwing a type error.
    let output = match values.get(2) {
        None | Some(Value::Unset) => None,
        Some(value @ Value::Reference(_)) => Some(value.clone()),
        Some(_) if rt.dialect().legacy_commands() => None,
        Some(value) => {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    3,
                    "RegExMatch",
                    "a variable reference",
                    value,
                ),
                line,
            });
        }
    };
    let text = regex_text(values, 0, "RegExMatch", line)?;
    let pattern = regex_text(values, 1, "RegExMatch", line)?;
    // The place where the search begins, the way `BIF_RegEx` reads it.
    let mut starting = 0usize;
    if let Some(value) = values.get(3) {
        if !is_numeric_value(value) {
            if !rt.dialect().legacy_commands() {
                return Err(RuntimeError {
                    message: crate::runtime::requires(4, "RegExMatch", TypeName::Number, value),
                    line,
                });
            }
            // The v1: a fourth parameter that is not a number is the place
            // zero, and the search begins at the start of the text.
            //
            // Measured on the binary of v1.1.37.02: the `RegExMatch` with
            // a fourth parameter that is not a number does the search, and it
            // does not throw.
        } else {
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
    }
    let regex = rt.the_regex_of(&pattern).map_err(|reason| RuntimeError {
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
            rt.write_reference(output, Value::Text("".into()));
        }
        return Ok(Value::Int(0));
    };
    let place = found.span().0 as i64 + 1;
    if let Some(output) = &output {
        if rt.dialect().legacy_commands() && !regex.options().object_mode {
            let chars: Vec<char> = text.chars().collect();
            if regex.options().position_mode {
                let (start, end) = found.span();
                rt.write_reference(output, Value::Int((end - start) as i64));
                for group in 1..regex.group_count() {
                    let (pos, len) = match found.span_of(group) {
                        Some((s, e)) => (s as i64 + 1, (e - s) as i64),
                        None => (0, 0),
                    };
                    rt.write_reference_suffix(output, &format!("Pos{group}"), Value::Int(pos));
                    rt.write_reference_suffix(output, &format!("Len{group}"), Value::Int(len));
                    if let Some(name) = regex.name_of(group) {
                        rt.write_reference_suffix(output, &format!("Pos{name}"), Value::Int(pos));
                        rt.write_reference_suffix(output, &format!("Len{name}"), Value::Int(len));
                    }
                }
            } else {
                rt.write_reference(output, Value::Text(found.text(&chars).into()));
                for group in 1..regex.group_count() {
                    let captured = found.group(group, &chars).unwrap_or_default();
                    rt.write_reference_suffix(
                        output,
                        &group.to_string(),
                        Value::Text(captured.clone().into()),
                    );
                    if let Some(name) = regex.name_of(group) {
                        rt.write_reference_suffix(output, name, Value::Text(captured.into()));
                    }
                }
            }
        } else {
            rt.write_reference(
                output,
                crate::classes::regexmatchinfo::of(&text, &regex, &found),
            );
        }
    }
    Ok(Value::Int(place))
}

/// The text of one pattern parameter of a `RegExMatch` or a `RegExReplace`.
///
/// Measured on the binary of v2.0.19: an object is refused
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
        // Measured on the binary of v2.0.19: the absent value is
        // no argument, and the text of no argument is the empty text.
        Some(Value::Unset) => Ok(String::new()),
        Some(value @ (Value::Object(_) | Value::List(_) | Value::Reference(_))) => {
            Err(RuntimeError {
                message: crate::runtime::requires(index + 1, name, TypeName::String, value),
                line,
            })
        }
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
    let regex = rt.the_regex_of(&pattern).map_err(|reason| RuntimeError {
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
        out.push_str(&expand_replacement(
            &replacement,
            &regex,
            &found,
            &characters,
        ));
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
    // The number of the swaps goes into the variable of the caller. An omitted
    // count parameter (`RegExReplace(s, p, r, , limit)`) arrives as `Unset`,
    // and in v1 any non-variable expression is ignored.
    if let Some(value) = values.get(3) {
        match value {
            Value::Unset => {}
            Value::Reference(_) => rt.write_reference(value, Value::Int(replaced)),
            _ if rt.dialect().legacy_commands() => {}
            _ => {
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
        }
    }
    Ok(Value::Text(out.into()))
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
            // v2.0.19: the braced name is read, and the name
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
    // O lugar relativo, o apertar e a roda: as tres coisas que o v1
    // aceita e que a arvore carregava como uma recusa.
    let mut relative = false;
    let mut pressing: Option<bool> = None;
    let mut wheel: Option<String> = None;
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
            // O quarto e o quinto botao sao os que a maquina tem
            // com o `MOUSEEVENTF_XDOWN`.
            "x1" => button = "x1".to_string(),
            "x2" => button = "x2".to_string(),
            // O "rel" e o "relative" dizem que o clique e no lugar onde
            // o ponteiro esta, e nao no lugar que vem antes: o lugar
            // que o script escreve e deixado de lado.
            "rel" | "relative" => relative = true,
            "down" | "d" => pressing = Some(true),
            "up" | "u" => pressing = Some(false),
            other
                if other.starts_with("wheel")
                    || other == "wu"
                    || other == "wd"
                    || other == "wl"
                    || other == "wr" =>
            {
                wheel = Some(the_wheel_of(other).unwrap_or("wheelup").to_string());
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
    // O lugar relativo e o lugar que o script escreve mais o lugar onde
    // o ponteiro esta.
    let (x, y) = if relative {
        let (pointer_x, pointer_y) = rt
            .desktop()
            .mouse_position()
            .map_err(|e| desktop_failure(e, line))?;
        (x + pointer_x, y + pointer_y)
    } else {
        (x, y)
    };
    // A roda gira, e nao aperta um botao.
    if let Some(wheel) = wheel {
        let detail = rt
            .desktop()
            .mouse_wheel(Some(x), Some(y), &wheel, count)
            .map_err(|e| desktop_failure(e, line))?;
        return Ok(Value::Text(detail.into()));
    }
    // O botao que fica apertado e o que fica solto.
    if let Some(down) = pressing {
        let detail = if down {
            rt.desktop()
                .mouse_down(Some(x), Some(y), &button)
                .map_err(|e| desktop_failure(e, line))?
        } else {
            rt.desktop()
                .mouse_up(Some(x), Some(y), &button)
                .map_err(|e| desktop_failure(e, line))?
        };
        return Ok(Value::Text(detail.into()));
    }
    let detail = rt
        .desktop()
        .click(x, y, &button, count)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail.into()))
}

/// `MouseClick`: the button goes first, and the parameters are the ones of
/// the function of v2.
///
/// Measured on the binary of v2.0.19: the button of a
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
    // A roda e um clique de um botao que a maquina nao tem: o
    // `MouseClick "WheelUp"` e o `MouseClick "WheelDown"` e o
    // `MouseClick "WheelLeft"` e o `MouseClick "WheelRight"`.
    //
    // Medido no binario do v2.0.31: o `MouseClick
    // "WheelUp"` da "ok".
    let wheel = the_wheel_of(&button);
    let name = match button.to_lowercase().as_str() {
        "left" | "l" => "left",
        "right" | "r" => "right",
        "middle" | "m" => "middle",
        "x1" => "x1",
        "x2" => "x2",
        _ if wheel.is_some() => "left",
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
    // As opcoes sao a sexta casa: medido no binario do v2.0.31 em
    // O `MouseClick "Left", 100, 100, 1, 2, "Down"` da "ok",
    // e o "Rel" e recusado com "Parameter #6 of MouseClick is
    // invalid.".
    let options = values
        .get(5)
        .map(|value| value.to_text().trim().to_lowercase())
        .unwrap_or_default();
    if let Some(wheel) = wheel {
        let detail = rt
            .desktop()
            .mouse_wheel(x, y, wheel, count)
            .map_err(|e| desktop_failure(e, line))?;
        return Ok(Value::Text(detail.into()));
    }
    match options.as_str() {
        "" => {}
        "down" => {
            let detail = rt
                .desktop()
                .mouse_down(x, y, name)
                .map_err(|e| desktop_failure(e, line))?;
            return Ok(Value::Text(detail.into()));
        }
        "up" => {
            let detail = rt
                .desktop()
                .mouse_up(x, y, name)
                .map_err(|e| desktop_failure(e, line))?;
            return Ok(Value::Text(detail.into()));
        }
        _ => {
            return Err(RuntimeError {
                message: "Parameter #6 of MouseClick is invalid.".into(),
                line,
            });
        }
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
    Ok(Value::Text(detail.into()))
}

/// The wheel that a name of a button names, quando ele nomeia uma.
///
/// A roda e um dos quatro nomes -- a de cima, a de baixo, a da esquerda
/// e a da direita -- e o nome de um botao que nao e uma roda e `None`.
fn the_wheel_of(button: &str) -> Option<&'static str> {
    match button.trim().to_lowercase().as_str() {
        "wheelup" | "wu" => Some("wheelup"),
        "wheeldown" | "wd" => Some("wheeldown"),
        "wheelleft" | "wl" => Some("wheelleft"),
        "wheelright" | "wr" => Some("wheelright"),
        _ => None,
    }
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
    Ok(Value::Text(title_case(&text_of(values, 0)).into()))
}

/// The text with the first letter of each word in the upper case and the
/// rest of it in the lower case.
///
/// Measured on the binary of v2.0.19: a word begins at the
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
/// Measured on the binary of v2.0.19: the label is asked in
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
/// and it answers with nothing. The v1 asks for more: the window that is
/// under the pointer, and the control that is under it.
///
/// Measured on the binary of v1.1.37.02: the third output is the window
/// of the top level that is under the pointer, and the fourth is the
/// control under it -- the name of it (`Button1`), or its handle when
/// the mode of the command asks for it (the bit 2). The bit 1 of the
/// mode asks for the window of the machine alone, without the search of
/// the control. The v1 has no such outputs, and the tree answered them
/// with the empty text: a script of the v1 compares the window with the
/// windows of it, and the empty text made it leave before it opened its
/// menu.
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
    // The mode of the command is the fifth parameter of it.
    let mode = values
        .get(4)
        .map(|value| value.to_text().trim().parse::<i64>().unwrap_or(0))
        .unwrap_or(0);
    // The v2 has no window and no control: the caller did not ask for
    // them, and the machine is not troubled with the question.
    let (window, control) = if values.len() > 2 {
        rt.desktop()
            .the_window_under_the_pointer(mode)
            .map_err(|e| desktop_failure(e, line))?
    } else {
        (0, 0)
    };
    // The control of a window that has no control of its own is the
    // window itself, and the v1 leaves the name of it out.
    let control_place = if control == 0 || control == window {
        Value::Text("".into())
    } else if mode & 0x02 != 0 {
        the_handle_of_the_machine(rt, control)
    } else {
        let name = rt
            .desktop()
            .control_classnn_of(control)
            .map_err(|e| desktop_failure(e, line))?;
        Value::Text(name.into())
    };
    let places = [
        Value::Int(x),
        Value::Int(y),
        if window != 0 {
            the_handle_of_the_machine(rt, window)
        } else {
            Value::Text("".into())
        },
        control_place,
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
    Ok(Value::Text("".into()))
}

/// `MouseMove`: the pointer to a place on the screen.
pub(crate) fn mouse_move(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The speed is not carried: the machine of the tree moves the
    // pointer at once.
    let _ = values.get(2);
    // The fourth parameter is the text "R": the move is made from where
    // the pointer is, and not to the place of the screen.
    //
    // Measured on the binary of v2.0.31: the
    // `MouseMove 10, 20, , "R"` answers, and the `MouseMove 10, 20, ,
    // 1` refuses with "Parameter #4 of MouseMove is invalid.".
    let relative = match values.get(3) {
        None => false,
        Some(value) => {
            let text = value.to_text();
            let text = text.trim();
            if text.is_empty() {
                false
            } else if text.eq_ignore_ascii_case("R") {
                true
            } else {
                return Err(RuntimeError {
                    message: "Parameter #4 of MouseMove is invalid.".into(),
                    line,
                });
            }
        }
    };
    let mut x = number_of(values, 0) as i64;
    let mut y = number_of(values, 1) as i64;
    if relative {
        let (here_x, here_y) = rt
            .desktop()
            .mouse_position()
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
        x += here_x;
        y += here_y;
    }
    let detail = rt
        .desktop()
        .move_pointer(x, y)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(detail.into()))
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
    rt.set_global("pixel", Value::Text(colour.clone().into()));
    Ok(Value::Text(colour.into()))
}

/// `PixelSearch`: where a colour is on the screen.
///
/// The place is written in the two variables of the caller and the answer
/// is one when the colour is there and zero when it is not, the way v2
/// does it.
///
/// Measured on the binary of v2.0.19: the command takes
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
            rt.write_reference(&values[0], Value::Text("".into()));
            rt.write_reference(&values[1], Value::Text("".into()));
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
    rt.set_global("screenshot", Value::Text(path.clone().into()));
    Ok(Value::Text(path.into()))
}

// -- the windows ----------------------------------------------------------------

/// Whether the window answers the text that names it.
///
/// The text is not only a title: it may name the class of the window
/// (`ahk_class`), the executable of it (`ahk_exe`), the identifier of it
/// (`ahk_id`) or the process of it (`ahk_pid`). Measured on the binaries
/// of v1.1.37.02 and of v2.0.31: the four names are
/// lidas nos dois dialetos, e a classe e comparada com a caixa que
/// ela tem.
fn a_window_answers(
    mode: crate::desktop::TitleMatch,
    window: &crate::desktop::WindowInfo,
    wanted: &str,
) -> bool {
    let criteria = crate::desktop::criteria::WindowCriteria::parse(wanted);
    a_window_answers_the_criteria(mode, window, &criteria, &window.exe)
}

/// Whether the window answers the criteria.
///
/// O criterio `ahk_exe` e o unico que precisa do executavel, e ele e
/// lido so quando o criterio o pede.
fn a_window_answers_the_criteria(
    mode: crate::desktop::TitleMatch,
    window: &crate::desktop::WindowInfo,
    criteria: &crate::desktop::criteria::WindowCriteria,
    exe: &str,
) -> bool {
    criteria
        .matches(
            &window.title,
            &window.class,
            exe,
            window.id,
            window.pid,
            window.active,
            mode,
        )
        .unwrap_or(false)
}

/// The criteria of a target, with the rule of the title `A`.
///
/// The title `A` alone is the window that has the focus, and the rule
/// needs nothing else: a text or an exclusion makes the `A` a plain
/// title. It is the macro of the v1 (source `window.h:30-31` of
/// v1.1.37.02).
fn the_criteria_of_a_target(wanted: &crate::desktop::TheWindowWanted) -> crate::desktop::criteria::WindowCriteria {
    let mut criteria = crate::desktop::criteria::WindowCriteria::parse(&wanted.title);
    if !wanted.text.is_empty()
        || !wanted.exclude_titles.is_empty()
        || !wanted.exclude_texts.is_empty()
    {
        criteria.the_active_window = false;
    }
    criteria
}

/// Whether the window answers the whole target: the title, the text of
/// one text element and the two exclusions.
///
/// Measured on the binaries of v1.1.37.02 and of v2.0.31:
/// the `WinText` matches one text element of the window, and the
/// exclusion by the title is a title and not a set of criteria -- and it
/// matches with the mode of the title, as the title does.
pub(crate) fn a_window_answers_the_target(
    mode: crate::desktop::TitleMatch,
    window: &crate::desktop::WindowInfo,
    wanted: &crate::desktop::TheWindowWanted,
    exe: &str,
) -> bool {
    // O titulo que o script escreveu e o texto vazio nao acha janela
    // nenhuma: medido no binario do v2.0.31, o
    // `WinGetTitle("")` recusa com "Target window not found.".
    if wanted.asked && wanted.title.is_empty() {
        return false;
    }
    let criteria = the_criteria_of_a_target(wanted);
    if !a_window_answers_the_criteria(mode, window, &criteria, exe) {
        return false;
    }
    if !window.has_the_text(&wanted.text) {
        return false;
    }
    for excluded in &wanted.exclude_titles {
        if mode.matches(&window.title, excluded).unwrap_or(false) {
            return false;
        }
    }
    for excluded in &wanted.exclude_texts {
        if window.has_the_text(excluded) {
            return false;
        }
    }
    true
}

/// The windows que respondem ao target, na ordem da listagem.
///
/// A listagem e a do desktop, e o criterio e o do runtime: e o mesmo
/// para os dois dialetos.
fn the_windows_that_answer(
    rt: &mut Runtime<'_>,
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<Vec<crate::desktop::WindowInfo>, RuntimeError> {
    let windows = rt
        .desktop()
        .windows()
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(the_windows_of_the_listing(rt, &windows, wanted))
}

/// The windows of the listing que respondem ao target.
///
/// A listagem ja vem do desktop, e o filtro e o do runtime: e o mesmo
/// para os dois dialetos.
pub(crate) fn the_windows_of_the_listing(
    rt: &mut Runtime<'_>,
    windows: &[crate::desktop::WindowInfo],
    wanted: &crate::desktop::TheWindowWanted,
) -> Vec<crate::desktop::WindowInfo> {
    let mode = crate::settings::title_match_mode();
    let criteria = the_criteria_of_a_target(wanted);
    // The criterion `ahk_id` names the window by the handle of it, and the
    // handle may be of a window that is a child of another one: the v1
    // (source `window.cpp:586-606` of v1.1.37.02) takes the handle as it
    // is, and a child is taken too (`window.cpp:626`, "Child windows (via
    // ahk_id) are always detectible"). The listing has only the windows of
    // the top, so the window of the handle is asked for by the desktop.
    //
    // Measured on the binary of v1.1.37.02: the
    // `WinGetClass, ahk_id %hButton%` of a button of a GUI of the script
    // answers "Button", and the `ControlGetText, , ahk_id %hButton%` of the
    // same button answers the text of it.
    if let Some(wanted_id) = &criteria.id {
        let Some(number) = crate::desktop::criteria::the_number_of(wanted_id) else {
            // A criterion whose value is not a number names no window.
            return Vec::new();
        };
        let Some(window) = rt.desktop().window_by_id(number) else {
            return Vec::new();
        };
        let exe = if criteria.exe.is_some() {
            rt.desktop().window_exe(window.id)
        } else {
            String::new()
        };
        if a_window_answers_the_target(mode, &window, wanted, &exe) {
            return vec![window];
        }
        return Vec::new();
    }
    // O executavel e lido so quando o criterio o pede: ele e uma
    // chamada que abre o processo de cada janela.
    let wants_the_exe = criteria.exe.is_some();
    let mut answer = Vec::new();
    for window in windows {
        let exe = if wants_the_exe {
            rt.desktop().window_exe(window.id)
        } else {
            String::new()
        };
        if a_window_answers_the_target(mode, window, wanted, &exe) {
            answer.push(window.clone());
        }
    }
    answer
}

/// The window that the script found last, inside a listing of the windows.
///
/// **The call that names no window asks about this one.** Measured on the
/// binary of the v2.0.32 (source `window.cpp:490-494` of it): "User passed
/// no params, so use the window most recently found by WinExist()." The
/// `WinExist` that finds a window is the one that puts it there -- the
/// `WinGetTitle` and the `WinGetClass` do not, and the tree does not either
/// (measured on the same binary).
///
/// The v1 asks about it the same way (source `script2.cpp:10911-10923` of
/// v1.1.37.02: "Use the \"last found\" window."), and it keeps it in a place
/// of its own -- the `WinLastFound` of the v1 -- and the v2 in another.
///
/// A script that found no window has no window to ask about, and the answer
/// is nothing: the window that is not there any more is not a window to ask
/// about (source `window.cpp:533-550` of the v2.0.32,
/// `GetValidLastUsedWindow`).
fn the_window_the_script_found_last(
    rt: &Runtime<'_>,
    windows: &[crate::desktop::WindowInfo],
) -> Option<crate::desktop::WindowInfo> {
    let id = if rt.dialect().is_the_v1() {
        rt.the_last_window_of_the_v1().map(|id| id.to_string())
    } else {
        rt.the_last_window().map(|id| id.to_string())
    }?;
    windows
        .iter()
        .find(|window| window.id.to_string() == id)
        .cloned()
}

/// The title of the window that the script found last.
///
/// The desktop is asked by the title of a window, and the title of the one
/// that the script found last is the title of it.
fn the_title_of_the_window_the_script_found_last(
    rt: &mut Runtime<'_>,
    line: u32,
) -> Result<String, RuntimeError> {
    let windows = rt
        .desktop()
        .windows()
        .map_err(|reason| desktop_failure(reason, line))?;
    match the_window_the_script_found_last(rt, &windows) {
        Some(window) => Ok(window.title),
        None => Err(no_window(line)),
    }
}

/// The identifier of the window that a call names.
///
/// The call names no window: the window that the script found last. A
/// number is the identifier of a window of the machine, and a title is the
/// title of one.
fn the_identifier_of_the_window_the_call_names(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<String, RuntimeError> {
    match values.first() {
        Some(Value::Int(id)) => Ok(id.to_string()),
        Some(value) if !value.to_text().is_empty() => {
            let wanted = crate::desktop::TheWindowWanted::of(values, 0);
            let windows = rt
                .desktop()
                .windows()
                .map_err(|reason| desktop_failure(reason, line))?;
            match the_windows_of_the_listing(rt, &windows, &wanted)
                .into_iter()
                .next()
            {
                Some(window) => Ok(window.id.to_string()),
                None => Err(no_window(line)),
            }
        }
        _ => {
            let windows = rt
                .desktop()
                .windows()
                .map_err(|reason| desktop_failure(reason, line))?;
            match the_window_the_script_found_last(rt, &windows) {
                Some(window) => Ok(window.id.to_string()),
                None => Err(no_window(line)),
            }
        }
    }
}

/// Whether the window of the script is the one that the call names.
///
/// The window of the script is the one that the tree made, and the controls
/// of it are the ones of the tree. A window of another program is a window
/// of the machine, and the desktop enumerates the controls of it.
fn the_script_owns_the_window(
    rt: &mut Runtime<'_>,
    target: &str,
    line: u32,
) -> Result<bool, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Ok(false);
    }
    let current = rt
        .desktop()
        .gui_title(0, None)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(!target.is_empty() && current.contains(target))
}

/// The window a call of the family names, inside a listing of the windows.
///
/// Measured on the binary of v2.0.19: a number is the id of a
/// window of this runtime, and a text is a title matched the way the script
/// matches it -- the mode of the settings.
///
/// **The call that names no window asks about the window that the script
/// found last**, and not about the window that is in the foreground of the
/// machine: measured on the binary of the v2.0.32, the `WinGetTitle()` of a
/// script that found the "Magnifier" answers "Magnifier", and the
/// `WinGetTitle()` of a script that found no window at all is refused with
/// "Target window not found.".
fn the_window_in(
    rt: &mut Runtime<'_>,
    windows: &[crate::desktop::WindowInfo],
    value: Option<&Value>,
) -> Result<Option<crate::desktop::WindowInfo>, RuntimeError> {
    match value {
        Some(Value::Int(id)) => Ok(windows.iter().find(|window| window.id == *id).cloned()),
        Some(value) if !value.to_text().is_empty() => {
            let wanted = value.to_text();
            // The "A" is the active window: the v1 and the v2 answer
            // the number of it, and not the number of a window that has
            // the title "A".
            //
            // Measured on the binary of v1.1.37.02: the
            // `WinActive("A")` of the v1 answers the handle of the
            // window that has the focus, and the tree answered zero: the
            // a script of the v1 compares the active window with the
            // windows of it, and the zero made it leave before it painted
            // the background.
            if wanted.eq_ignore_ascii_case("A") {
                return Ok(windows.iter().find(|window| window.active).cloned());
            }
            let mode = crate::settings::title_match_mode();
            for window in windows {
                if a_window_answers(mode, window, &wanted) {
                    return Ok(Some(window.clone()));
                }
            }
            Ok(None)
        }
        _ => Ok(the_window_the_script_found_last(rt, windows)),
    }
}

/// The failure of a window query that found nothing.
///
/// Measured on the binary of v2.0.19: `WinGetID`, `WinGetIDLast`
/// and `WinGetClass` of a window that is not there answer "Target window not
/// found.", and the `Extra` of the failure is the title that was asked about.
fn no_window(line: u32) -> RuntimeError {
    RuntimeError {
        message: "Target window not found.".to_string(),
        line,
    }
}

pub(crate) fn window_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The listing names the window the way v2 answers it: measured on the
    // binary of v2.0.19, `WinActive("A")` is the id of the
    // active window, and not one.
    //
    // The text of the window and the two exclusions are part of it
    // too: measured on the binary of v2.0.31, the
    // `WinActive` of a window by the text of one text element of it
    // finds the window.
    // The "A" is the active window: the v1 and the v2 answer the number
    // of it, and the listing of the windows is not filtered by a title
    // that no window has.
    //
    // Measured on the binary of v1.1.37.02: the
    // `WinActive("A")` of the v1 answers the handle of the window that
    // has the focus, and the tree answered zero: a script of the v1
    // compares the active window with the windows of it, and the zero
    // made it leave before it painted the background.
    if values
        .first()
        .map(|value| value.to_text().eq_ignore_ascii_case("A"))
        .unwrap_or(false)
    {
        let Ok(windows) = rt.desktop().windows() else {
            return Ok(Value::Int(0));
        };
        let window = windows.iter().find(|window| window.active);
        return Ok(Value::Int(window.map(|window| window.id).unwrap_or(0)));
    }
    let wanted = window_target(values)?;
    match rt.desktop().windows() {
        Ok(windows) => {
            let windows = the_windows_of_the_listing(rt, &windows, &wanted);
            let window = the_window_in(rt, &windows, values.first())?;
            return Ok(Value::Int(match window {
                Some(window) if window.active => window.id,
                _ => 0,
            }));
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
    // The answer of v2 is the id of the window that is there, and not one:
    // measured on the binary of v2.0.19, `WinExist("A")` is the
    // id of the active window.
    //
    // The text of the window and the two exclusions are part of it
    // too.
    let wanted = window_target(values)?;
    match rt.desktop().windows() {
        Ok(windows) => {
            let windows = the_windows_of_the_listing(rt, &windows, &wanted);
            let window = the_window_in(rt, &windows, values.first())?;
            // The window that was found is the last one: a `Win*` call
            // that names no window after this one asks about it, the way
            // the reference carries the last found window.
            if let Some(window) = &window {
                rt.remember_the_last_window(&window.id.to_string());
            }
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

/// `WinGetTitle`: the title of the window that the call names.
///
/// The call that names no window asks about the window that the script found
/// last, and a script that found none has no window to ask about: measured
/// on the binary of the v2.0.32, the `WinGetTitle()` of such a script is
/// refused with "Target window not found.". The tree answered with the
/// window that is in the foreground of the machine.
pub(crate) fn win_get_title(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // Measured on the binary of v2.0.19: the title of a window
    // that is not there is "Target window not found.", while the empty text
    // and the omitted one ask about the active window, which the tree has
    // always answered with the empty text when there is none.
    //
    // The text of the window and the two exclusions are part of it too:
    // measured on the binary of v2.0.31, the `WinGetTitle`
    // of a window by the text of one text element of it finds the
    // window, and the `WinGetTitle` with the exclusion of its title
    // finds no window at all and answers "Target window not found.".
    let wanted = window_target(values)?;
    // O titulo que o script escreveu e o texto vazio nao acha janela
    // nenhuma: medido no binario do v2.0.31, o
    // `WinGetTitle("")` recusa com "Target window not found.".
    //
    // O numero e o identificador da janela, e nao o titulo dela.
    let asked = !values.is_empty();
    if wanted.asked && wanted.title.is_empty() {
        return Err(no_window(line));
    }
    if let Ok(windows) = rt.desktop().windows() {
        let windows = the_windows_of_the_listing(rt, &windows, &wanted);
        let window = the_window_in(rt, &windows, values.first())?;
        let title = match window {
            Some(window) => window.title,
            // The call that names no window asks about the window that the
            // script found last, and a script that found none has no window
            // to ask about: measured on the binary of the v2.0.32, the
            // `WinGetTitle()` of such a script is refused with "Target
            // window not found.".
            //
            // The v1 writes the empty text instead of refusing: measured on
            // the binary of v1.1.37.02 (source `script2.cpp:3841-3870` of
            // it: "Even if target_window is NULL, we want to continue on so
            // that the output param is set to be the empty string").
            None if rt.dialect().is_the_v1() => String::new(),
            None => return Err(no_window(line)),
        };
        // The pause of v2 follows a windowing function, and a delay of -1 is
        // no pause at all.
        let delay = crate::settings::the_win_delay();
        if delay > 0 {
            rt.sleep_ms(delay as u64);
        }
        return Ok(Value::Text(title.into()));
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
    Ok(Value::Text(title.unwrap_or_default().into()))
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
/// Measured on the binary of v2.0.19: `ProcessGetName` and
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
/// Measured on the binary of v2.0.19: the answer is the pid of
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
/// Measured on the binary of v2.0.19: the name is the file name
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
        Some(name) => Ok(Value::Text(name.into())),
        None => Err(no_process(line)),
    }
}

/// `ProcessGetPath`: the full path of the executable of a process.
///
/// Measured on the binary of v2.0.19: the path is the whole
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
        Some(path) => Ok(Value::Text(path.into())),
        None => Err(no_process(line)),
    }
}

/// `ProcessGetParent`: the pid of the parent of a process.
///
/// Measured on the binary of v2.0.19: the answer is the pid of
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
/// Measured on the binary of v2.0.19: the answer is the pid
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
/// Measured on the binary of v2.0.19: the answer is the pid of
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
/// Measured on the binary of v2.0.19: the answer is zero when
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
/// Measured on the binary of v2.0.19: the binary reads only the
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
    let priority = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let letter = priority
        .chars()
        .next()
        .map(|letter| letter.to_ascii_uppercase());
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
/// Measured on the binary of v2.0.19: an omitted number and the
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
    let monitors = rt
        .desktop()
        .monitors()
        .map_err(|e| desktop_failure(e, line))?;
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

/// `MonitorGet`: the bounds of a monitor.
pub(crate) fn monitor_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_monitor_bounds_into(rt, values, line, "MonitorGet")
}

/// `MonitorGetCount`: how many monitors the desktop has.
///
/// Measured on the binary of v2.0.19: the answer is the number
/// of the monitors, and a desktop with one screen answers one.
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
///
/// Measured on the binary of v2.0.19: the name of the monitor
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
    Ok(Value::Text(monitor.name.into()))
}

/// `MonitorGetPrimary`: the number of the primary monitor.
///
/// Measured on the binary of v2.0.19: the answer counts from
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
/// Measured on the binary of v2.0.19: the shape is the one of
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
/// Measured on the binary of v2.0.19, with the source of it
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
    let monitors = rt
        .desktop()
        .monitors()
        .map_err(|e| desktop_failure(e, line))?;
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
        76 => monitors
            .iter()
            .map(|monitor| monitor.left)
            .min()
            .unwrap_or(0),
        77 => monitors
            .iter()
            .map(|monitor| monitor.top)
            .min()
            .unwrap_or(0),
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
            match rt.desktop().system_metric(other as i32) {
                Some(value) => value,
                None => {
                    return Err(RuntimeError {
                        message: format!(
                            "SysGet({other}) is a metric of the window manager of Windows, and this \
                             desktop does not have one."
                        ),
                        line,
                    });
                }
            }
        }
        // A number that is not a metric of the system is zero, which is what
        // the binary answers for one: measured on it,
        // `SysGet(-1)` and `SysGet(1000)` are zero.
        _ => 0,
    };
    Ok(Value::Int(answer))
}

/// `SysGetIPAddresses`: the addresses of the machine.
///
/// Measured on the binary of v2.0.19: the answer is an array of
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
    let items: Vec<Value> = addresses.into_iter().map(Value::text).collect();
    crate::classes::array::construct(&items, rt, line).map_err(|failure| RuntimeError {
        message: the_message_of(failure),
        line,
    })
}

// -- the keyboard, the debugger, the trash and the drives -------------------------

/// `SetCapsLockState`, `SetNumLockState` and `SetScrollLockState`: the state
/// of a lock light of the keyboard.
///
/// Measured on the binary of v2.0.19: "On" turns the light on
/// and "Off" turns it out -- and `GetKeyState(<lock>, "T")` reads the state
/// back, one for the light that is on. Every other word, "Toggle" included,
/// leaves the state as it was, and no word is a failure.
///
/// The "Toggle" of the documentation presses the key of the lock, which
/// turns the lamp of the physical keyboard; the state that `GetKeyState`
/// reads is the one the program set, and it does not move.
fn set_lock_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    lock: &str,
) -> Result<Value, RuntimeError> {
    let word = text_of(values, 0).trim().to_lowercase();
    let wanted = match word.as_str() {
        "on" => Some(true),
        "off" => Some(false),
        _ => None,
    };
    if let Some(on) = wanted {
        rt.desktop()
            .set_lock_light(lock, on)
            .map_err(|reason| desktop_failure(reason, line))?;
    }
    Ok(Value::Text("".into()))
}

/// `OutputDebug`: a text for the debugger of the machine.
///
/// Measured on the binary of v2.0.19: the parameter is a text,
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
    Ok(Value::Text("".into()))
}

/// `FileRecycle`: a file, a folder, or the files that a pattern matches.
///
/// Measured on the binary of v2.0.19: the answer is the empty
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
    Ok(Value::Text("".into()))
}

/// `FileRecycleEmpty`: empty the trash of the machine.
///
/// Measured on the binary of v2.0.19: the answer is the empty
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
    Ok(Value::Text("".into()))
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
    // The message box of the machine shows the text and answers the name of
    // the button that was pressed. The text goes to the report as well, so
    // a headless run (a desktop with no window) still records what was said.
    let text = rt.interpolate(&text_of(values, 0), line)?;
    let title = if values.len() > 1 {
        rt.interpolate(&text_of(values, 1), line)?
    } else {
        String::new()
    };
    let options = match values.get(2) {
        Some(Value::Int(n)) => *n,
        Some(Value::Float(f)) => *f as i64,
        _ => 0,
    };
    // The modal box pumps the messages of the thread while it waits: the
    // runtime is kept for the thread, so the callbacks of the machine and
    // the handlers of `OnMessage` run while the box is up -- a script
    // themes the box dark by subclassing it inside the wait.
    #[cfg(feature = "std")]
    let button = {
        let desktop_ptr: *mut (dyn crate::desktop::Desktop + '_) = rt.desktop();
        let previous = crate::callback::set_current(rt);
        let answer = unsafe { (*desktop_ptr).msg_box(&text, &title, options) };
        crate::callback::clear_current(previous);
        answer.map_err(|e| desktop_failure(e, line))?
    };
    #[cfg(not(feature = "std"))]
    let button = rt
        .desktop()
        .msg_box(&text, &title, options)
        .map_err(|e| desktop_failure(e, line))?;
    rt.push_output(text);
    Ok(Value::Text(button.into()))
}

/// `Run`: a command, and what it answered.
pub(crate) fn run(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The third parameter of v2 is the options of the window, and the
    // fourth is the variable that takes the pid of the process.
    //
    // The options of the window are the way the window of the process is
    // shown, and the box has no window to show: the options are read and
    // left out. The pid is the number of the process the box started and
    // waited for, and it is written to the variable.
    let command = text_of(values, 0);
    let working_dir = text_of(values, 1);
    let (code, stdout, stderr, pid) = if values.len() > 3 {
        rt.desktop()
            .run_with_pid(&command, &working_dir)
            .map_err(|e| desktop_failure(e, line))?
    } else {
        let (code, stdout, stderr) = rt
            .desktop()
            .run(&command, &working_dir)
            .map_err(|e| desktop_failure(e, line))?;
        (code, stdout, stderr, 0)
    };
    if values.len() > 3 {
        if let Some(variable) = values.get(3) {
            if matches!(variable, Value::Reference(_)) {
                rt.write_reference(variable, Value::Int(pid as i64));
            }
        }
    }
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
    Ok(Value::Text(parts.concat().into()))
}

/// `Sleep`: the pause, in milliseconds.
pub(crate) fn sleep(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let millis = number_of(values, 0).clamp(0.0, 60_000.0) as u64;
    // The v1: the timers of the script run while it waits.
    //
    // Measured on the binary of the v1.1.37.02: the timer
    // of a hundred milliseconds runs five times during a `Sleep, 600`, and
    // the wait goes on for its whole time.
    rt.the_run_waits_for(millis, line)
        .map_err(|failure| crate::runtime::as_error(failure, line))?;
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
        value = value
            .checked_mul(10)?
            .checked_add(character.to_digit(10)? as usize)?;
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
/// Measured on the binary of v2.0.19: a specifier that takes a
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
    // Measured on the binary of v2.0.19: an argument that is the
    // absent value -- the hole between two commas, or the keyword `unset` -- is
    // the empty text in a format, whatever the specifier wants.
    if matches!(value, Value::Unset) {
        return Ok(FormatValue::Text("".into()));
    }
    match spec.kind {
        FormatKind::Str(_) => match value {
            Value::Int(number) => Ok(FormatValue::Text(number.to_string())),
            Value::Float(number) => Ok(FormatValue::Text(crate::float_text::the_text_of(*number))),
            Value::Text(text) => Ok(FormatValue::Text(text.to_string())),
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
/// The pattern is the one of the binary, measured: `{1}` is the
/// first argument and a placeholder with no number takes the next one; `{{`
/// and `}}` are one brace of their own; and a placeholder that names an
/// argument that is not there, or whose spec is not one the binary reads, is
/// left exactly as it was written.
fn format_pattern(pattern: &str, values: &[Value], line: u32) -> Result<String, RuntimeError> {
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
        FormatKind::Int => format_integer(
            spec,
            match value {
                FormatValue::Int(number) => *number,
                _ => 0,
            },
        ),
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
        FormatKind::Float => format_float(
            spec,
            match value {
                FormatValue::Float(number) => *number,
                _ => 0.0,
            },
        ),
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
            digits = format!("{}{digits}", "0".repeat(precision - digits.chars().count()));
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
/// Measured on the binary of v2.0.19: `%c` writes the low
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
    Ok(Value::Text("".into()))
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
    Ok(Value::Text(out.into()))
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
        .map(Value::text)
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
        .map(Value::text)
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
    Ok(Value::Text("".into()))
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
    Ok(Value::Text("".into()))
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
            message: format!("Parameter #1 of Props requires an Object, but received a {kind}."),
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
            });
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
            errors.push(Value::Text((format!("{place}: {}", failure.message)).into()));
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
    object.borrow_mut().put(
        "Warnings",
        crate::classes::array::construct(&[], rt, _line).map_err(|failure| RuntimeError {
            message: crate::runtime::the_message_of(&failure),
            line: _line,
        })?,
    );
    Ok(Value::Object(object))
}

/// `GetKeyInfo`: what the tree knows of a key.
///
/// The answer is an object with the virtual key, the scan code, the name,
/// the modifiers and the prefix of the key -- and the number zero when the
/// name does not resolve, the way KeySharp answers it. The resolution is
/// the one of `GetKeyName`, `GetKeyVK` and `GetKeySC`, so the four agree:
/// the name is the name v2 gives the key, the virtual key is the one of
/// Windows and the scan code is the one of the Linux input event.
///
/// The modifier symbols at the front of the name go into the modifiers
/// and the prefix, the way KeySharp reads them. The layout parameter is
/// refused with the reason: the tree has one keyboard layout, and a
/// parameter it cannot honour is never ignored.
pub(crate) fn ks_get_key_info(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = match values.first() {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    let (prefix, modifiers, rest) = crate::keys::the_modifier_prefix_of(&text);
    let mut vk = crate::settings::key_code(rest);
    let mut sc = crate::settings::key_event(rest) as u32;
    let name = crate::settings::key_name(rest);
    // KeySharp answers the number zero when the name does not resolve, and
    // a key with no code and no name is not a key.
    if (vk == 0 && sc == 0) || name.is_empty() {
        return Ok(Value::Int(0));
    }
    // The layout parameter: the key is resolved in the layout that is
    // named. The virtual key of a character and the scan code of the key
    // are the ones of that layout.
    if let Some(layout_value) = values.get(1) {
        if !matches!(layout_value, Value::Unset) {
            let layout = layout_value.to_text();
            let (the_vk, the_sc) = rt
                .desktop()
                .key_in_layout(vk, rest, &layout)
                .map_err(|reason| RuntimeError {
                    message: reason,
                    line,
                })?;
            vk = the_vk;
            sc = the_sc;
        }
    }
    Ok(crate::classes::keyinfo::the_key_info(
        vk, sc, &name, modifiers, &prefix,
    ))
}
/// `ObjGetDataPtr`: the address of the structured data of an object.
///
/// The instance that `ObjCreateWithDataPtr` or `Struct.At` made carries
/// the address it borrows, and the answer is it. Any other object is
/// answered the way KeySharp answers it -- "Operation failed." -- and a
/// value that is not an object is refused with the reason.
pub(crate) fn ks_obj_get_data_ptr(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value,
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            });
        }
    };
    let Value::Object(object) = value else {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjGetDataPtr", TypeName::Object, value),
            line,
        });
    };
    match object
        .borrow()
        .value_of(crate::classes::typed::THE_DATA_POINTER)
    {
        Some(address) => Ok(address),
        None => Err(RuntimeError {
            message: "Operation failed.".to_string(),
            line,
        }),
    }
}

/// `ObjGetDataSize`: the size of the structured data of an object.
///
/// The instance of a typed class answers the size of its field, and any
/// other object answers zero -- the answer of KeySharp for an object
/// that is not a struct.
pub(crate) fn ks_obj_get_data_size(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value,
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            });
        }
    };
    let Value::Object(object) = value else {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjGetDataSize", TypeName::Object, value),
            line,
        });
    };
    let class = object.borrow().class.clone();
    let size = match crate::classes::typed::the_class_of(&class) {
        Some((_, size, _)) => size,
        None => 0,
    };
    Ok(Value::Int(size))
}

/// `ObjSetDataPtr`: the address an instance borrows, set.
///
/// An object that borrows no address is refused with the answer of
/// KeySharp: "Operation failed.".
pub(crate) fn ks_obj_set_data_ptr(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let value = match values.first() {
        Some(value) => value,
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            });
        }
    };
    let Value::Object(object) = value else {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjSetDataPtr", TypeName::Object, value),
            line,
        });
    };
    let address = match values.get(1).and_then(|value| value.to_number()) {
        Some(number) => number as i64,
        None => {
            let second = values.get(1).unwrap_or(&Value::Unset);
            return Err(RuntimeError {
                message: crate::runtime::requires(2, "ObjSetDataPtr", TypeName::Number, second),
                line,
            });
        }
    };
    let borrows = object
        .borrow()
        .value_of(crate::classes::typed::THE_DATA_POINTER)
        .is_some();
    if !borrows {
        return Err(RuntimeError {
            message: "Operation failed.".to_string(),
            line,
        });
    }
    object
        .borrow_mut()
        .put(crate::classes::typed::THE_DATA_POINTER, Value::Int(address));
    Ok(Value::Text("".into()))
}

/// `ObjCreateWithDataPtr`: the instance of a class that borrows an
/// address.
///
/// The class is the value of a typed class, and anything else is refused
/// with the message of KeySharp: "Invalid class.". The address below
/// 65536 that is not the null one is refused with the parameter message
/// of KeySharp, and a typed number -- whose field would live in the
/// memory -- is refused with the reason: the tree has no memory to read
/// a field from.
pub(crate) fn ks_obj_create_with_data_ptr(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let invalid = || RuntimeError {
        message: "Invalid class.".to_string(),
        line,
    };
    let name = match values.first() {
        Some(Value::Object(object)) => match object.borrow().value_of(crate::runtime::CLASS_NAME) {
            Some(Value::Text(name)) => name,
            _ => return Err(invalid()),
        },
        _ => return Err(invalid()),
    };
    let class = match crate::classes::typed::the_class_of(&name) {
        Some((_, _, _)) => name.to_lowercase(),
        None => return Err(invalid()),
    };
    let address = match values.get(1).and_then(|value| value.to_number()) {
        Some(number) => number as i64,
        None => {
            let second = values.get(1).unwrap_or(&Value::Unset);
            return Err(RuntimeError {
                message: crate::runtime::requires(
                    2,
                    "ObjCreateWithDataPtr",
                    TypeName::Number,
                    second,
                ),
                line,
            });
        }
    };
    match crate::classes::typed::the_view_at(&class, address, "ObjCreateWithDataPtr", 2, line) {
        Some(Ok(value)) => {
            let _ = rt;
            Ok(value)
        }
        Some(Err(error)) => Err(error),
        None => Err(invalid()),
    }
}

/// `ShowDebug`: the debug tab of the main window.
///
/// The runner of this tree has no main window, and the ShowDebug of
/// KeySharp does nothing when the script has none -- the call of it is
/// the null-conditional in its source. The answer is the empty text,
/// the answer of a headless run of KeySharp: the tree has no debug tab,
/// and it says nothing that is false about one.
pub(crate) fn ks_show_debug(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text("".into()))
}

/// The pieces of a command line: the blanks separate, and the double
/// quotes group, the way KeySharp splits the options of `RunScript`.
fn the_pieces_of_a_command_line(text: &str) -> Vec<String> {
    let mut pieces: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut has = false;
    for character in text.chars() {
        match character {
            '"' => {
                quoted = !quoted;
                has = true;
            }
            other => {
                if other.is_whitespace() && !quoted {
                    if has {
                        pieces.push(current.clone());
                        current.clear();
                        has = false;
                    }
                    continue;
                }
                current.push(other);
                has = true;
            }
        }
    }
    if has {
        pieces.push(current);
    }
    pieces
}

/// `Mail`: send a mail through an SMTP server.
///
/// The recipients are one text or an array of them, and the options are
/// a Map with `host` and `from` required -- SMTP has no default for
/// either -- and `cc`, `bcc`, `replyto`, `attachments` and `headers`
/// optional. An unknown key is refused with the message of KeySharp that
/// lists the accepted ones, and the port rides on the host after the
/// rightmost colon that is not inside a bracketed IPv6 literal.
///
/// The validation is the dialect's and runs on every machine; the
/// sending is the machine's, and a desktop that cannot send says so --
/// a mail that was not sent is never reported as sent.
pub(crate) fn ks_mail(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let failure = |message: String| RuntimeError { message, line };
    // The recipients: one text or an array of them, none of them empty.
    let mut to: Vec<String> = Vec::new();
    let first = values.first().unwrap_or(&Value::Unset);
    match first {
        Value::Text(text) => {
            if !text.is_empty() {
                to.push(text.to_string());
            }
        }
        // The array of the script is an object of the class, and the
        // module that owns it is the one that hands over the items: the
        // built-in may not reach into the state of the array.
        Value::Object(object) if object.borrow().class == "array" => {
            for (index, item) in crate::classes::array::the_items(first).iter().enumerate() {
                match item {
                    Value::Text(text) if !text.is_empty() => to.push(text.to_string()),
                    Value::Text(_) => {
                        return Err(failure(format!("Recipient {} is empty.", index + 1)));
                    }
                    other => {
                        return Err(failure(format!(
                            "Recipient {} is {}; every recipient must be a String.",
                            index + 1,
                            other.the_kind()
                        )));
                    }
                }
            }
        }
        other => return Err(failure(expected_type(TypeName::String, other.type_name()))),
    }
    if to.is_empty() {
        return Err(failure("At least one recipient is required.".to_string()));
    }
    let subject = text_of(values, 1);
    let body = text_of(values, 2);

    let mut host: Option<String> = None;
    let mut port: u16 = 25;
    let mut from: Option<String> = None;
    let mut reply_to: Option<String> = None;
    let mut cc: Vec<String> = Vec::new();
    let mut bcc: Vec<String> = Vec::new();
    let mut attachments: Vec<String> = Vec::new();
    let mut headers: Vec<(String, String)> = Vec::new();

    if let Some(value) = values.get(3) {
        if !matches!(value, Value::Unset) {
            let pairs = match crate::classes::map::the_pairs(value) {
                Some(pairs) => pairs,
                None => return Err(failure("The options of Mail must be a Map.".to_string())),
            };
            for (key, val) in pairs {
                let item = key.to_text();
                if item.is_empty() {
                    continue;
                }
                match item.to_lowercase().as_str() {
                    "host" => {
                        let Value::Text(text) = val else {
                            return Err(failure(format!(
                                "The \"{item}\" option must be a String."
                            )));
                        };
                        // The rightmost colon only, and never one inside a
                        // bracketed IPv6 literal.
                        let mut named = text.to_string();
                        if let Some(colon) = text.rfind(':') {
                            if text.rfind(']').is_none_or(|bracket| bracket < colon) {
                                let (left, right) = text.split_at(colon + 1);
                                let number = right.trim().parse::<i64>();
                                match number {
                                    Ok(number) if (1..=65535).contains(&number) => {
                                        port = number as u16;
                                        named = left[..left.len() - 1].to_string();
                                    }
                                    _ => {
                                        return Err(failure(format!(
                                            "\"{right}\" is not a port between 1 and 65535."
                                        )));
                                    }
                                }
                            }
                        }
                        if named.is_empty() {
                            return Err(failure(format!("The \"{item}\" option names no server.")));
                        }
                        host = Some(named);
                    }
                    "from" | "replyto" => {
                        let Value::Text(text) = val else {
                            return Err(failure(format!(
                                "The \"{item}\" option must be a String."
                            )));
                        };
                        if item.eq_ignore_ascii_case("from") {
                            from = Some(text.to_string());
                        } else {
                            reply_to = Some(text.to_string());
                        }
                    }
                    "cc" | "bcc" | "attachments" => {
                        let wanted = item.to_lowercase();
                        let list = match &val {
                            Value::Text(text) => vec![text.to_string()],
                            Value::Object(inner) if inner.borrow().class == "array" => {
                                let mut out: Vec<String> = Vec::new();
                                for entry in crate::classes::array::the_items(&val) {
                                    let Value::Text(text) = entry else {
                                        return Err(failure(format!(
                                            "The \"{item}\" option must be a String or an Array of them."
                                        )));
                                    };
                                    out.push(text.to_string());
                                }
                                out
                            }
                            _ => {
                                return Err(failure(format!(
                                    "The \"{item}\" option must be a String or an Array of them."
                                )));
                            }
                        };
                        match wanted.as_str() {
                            "cc" => cc = list,
                            "bcc" => bcc = list,
                            _ => attachments = list,
                        }
                    }
                    "headers" => {
                        let pairs = match crate::classes::map::the_pairs(&val) {
                            Some(pairs) => pairs,
                            None => return Err(failure(
                                "The \"headers\" option must be a Map of header names and values."
                                    .to_string(),
                            )),
                        };
                        for (name, value) in pairs {
                            let name = name.to_text();
                            if name.is_empty() {
                                continue;
                            }
                            headers.push((name, value.to_text()));
                        }
                    }
                    _ => {
                        return Err(failure(format!(
                            "Unknown option \"{item}\". Accepted keys are host, from, cc, bcc, replyto, attachments and headers."
                        )));
                    }
                }
            }
        }
    }

    let Some(host) = host else {
        return Err(failure(
            "The \"host\" option is required: there is no default SMTP server.".to_string(),
        ));
    };
    let Some(from) = from else {
        return Err(failure(
            "The \"from\" option is required: SMTP has no sender to fall back on.".to_string(),
        ));
    };

    let message = crate::desktop::MailMessage {
        to,
        cc,
        bcc,
        subject,
        body,
        from,
        reply_to,
        attachments,
        headers,
        host,
        port,
    };
    rt.desktop()
        .send_mail(&message)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `RunScript`: run a script in a process of its own.
///
/// The code is the path of a file when the machine has one and the
/// source otherwise, the way CompileScript reads it, and the compile of
/// the tree is the parser of the dialect that runs: a script that does
/// not parse is never launched. The program -- the runner of this run by
/// default -- is started with the script on its standard input, and the
/// answer is the ScriptProcess of the pid.
///
/// The `name` parameter names the generated program of KeySharp, and the
/// tree generates no program of its own: the parameter is refused with
/// the reason. The callback of an asynchronous run is refused too: the
/// tree does not watch a process it does not wait for, and a callback
/// that is never called is a lie. With no asynchronous flag the run is
/// waited for, and the callback is called with the process before the
/// answer.
pub(crate) fn ks_run_script(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let failure = |message: String| RuntimeError { message, line };
    let code = text_of(values, 0);
    // A file of that name, when the machine has one, and the source
    // otherwise -- the reading of CompileScript.
    let source = match rt.desktop().read_file(&code) {
        Ok(lines) => lines.join("\n"),
        Err(_) => code.clone(),
    };
    // The compile of the tree is the parser of the dialect: KeySharp
    // throws a compilation as an Error, and a script that does not parse
    // is never launched.
    if let Err(failure_of_the_parse) = crate::parser::parse_with(&source, rt.dialect()) {
        return Err(failure(format!(
            "the script does not compile: {}:{}: {}",
            failure_of_the_parse.line, failure_of_the_parse.column, failure_of_the_parse.message
        )));
    }
    let asynchronous = values.get(1).map(|value| value.truthy()).unwrap_or(false);
    // The callback takes the process alone -- the `CheckedCallback(
    // callback, 1)` of KeySharp -- and the tree calls it by the name of
    // the function, the way the callbacks of the exit are called. A
    // function with no name to be called by is refused with the answer
    // of KeySharp.
    let callback_name: Option<String> = match values.get(2) {
        None => None,
        Some(Value::Unset) => None,
        Some(value) => match value {
            Value::Object(object) if object.borrow().class == "func" => {
                match object.borrow().value_of("name") {
                    Some(Value::Text(text)) if !text.is_empty() => Some(text.to_string()),
                    _ => return Err(failure("Invalid callback function.".to_string())),
                }
            }
            _ => {
                return Err(failure(format!(
                    "This value of type \"{}\" has no method named \"Call\".",
                    value.type_name().name()
                )));
            }
        },
    };
    // The name of the generated program of KeySharp has nothing to name
    // here: the tree launches the runner itself and compiles nothing.
    if let Some(value) = values.get(3) {
        if !matches!(value, Value::Unset) && !value.to_text().is_empty() {
            return Err(failure(
                "this tree generates no program of its own: the name parameter of RunScript has nothing to name".to_string(),
            ));
        }
    }
    let executable = match values.get(4) {
        Some(value) if !matches!(value, Value::Unset) && !value.to_text().is_empty() => {
            value.to_text()
        }
        _ => rt
            .desktop()
            .executable()
            .map_err(|reason| desktop_failure(reason, line))?,
    };
    // The options: the arguments of the launched process. --define and
    // --include select what is compiled, and the tree compiles with its
    // parser alone: the two are refused with the reason.
    let mut arguments: Vec<String> = Vec::new();
    if let Some(value) = values.get(5) {
        if !matches!(value, Value::Unset) {
            let pieces = match value {
                Value::Object(object) if object.borrow().class == "array" => {
                    crate::classes::array::the_items(value)
                        .iter()
                        .map(|item| item.to_text())
                        .collect()
                }
                other => the_pieces_of_a_command_line(&other.to_text()),
            };
            for piece in pieces {
                let lowered = piece.to_lowercase();
                if lowered.starts_with("--define") || lowered.starts_with("--include") {
                    return Err(failure(
                        "this tree compiles with its parser alone: --define and --include have no compilation to be applied to".to_string(),
                    ));
                }
                arguments.push(piece);
            }
        }
    }
    if asynchronous && callback_name.is_some() {
        return Err(failure(
            "this tree does not watch a process it does not wait for: the callback of an asynchronous RunScript cannot be called".to_string(),
        ));
    }
    let pid = if asynchronous {
        rt.desktop()
            .launch_with_stdin(&executable, &arguments, &source, "")
            .map_err(|reason| desktop_failure(reason, line))? as i64
    } else {
        let (pid, _, _, _) = rt
            .desktop()
            .run_with_stdin(&executable, &arguments, &source, "")
            .map_err(|reason| desktop_failure(reason, line))?;
        pid as i64
    };
    let process = match crate::classes::scriptprocess::construct(
        "scriptprocess",
        &[Value::Int(pid)],
        rt,
        line,
    ) {
        Some(Ok(value)) => value,
        Some(Err(failure_of_the_object)) => {
            return Err(RuntimeError {
                message: crate::runtime::the_message_of(&failure_of_the_object),
                line,
            });
        }
        None => {
            return Err(failure(
                "the object of the process could not be made".to_string(),
            ));
        }
    };
    if let Some(name) = &callback_name {
        // The run was waited for: the process is over, and the callback
        // is called with it, the way KeySharp calls it after the exit.
        rt.call_named(
            name,
            core::slice::from_ref(&process),
            line,
            crate::runtime::Route::Written,
        )
        .map_err(|failure_of_the_call| RuntimeError {
            message: crate::runtime::the_message_of(&failure_of_the_call),
            line,
        })?;
    }
    Ok(process)
}

/// `GetKeyboardLayout`: the name of the keyboard layout of the machine.
///
/// The name is the one the platform gives it, and the empty text when the
/// platform has none to give -- the answer of KeySharp. A machine that
/// cannot say at all is refused with the reason.
pub(crate) fn ks_get_keyboard_layout(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = rt
        .desktop()
        .keyboard_layout_name()
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(name.into()))
}

/// `ClipCursor`: confine the pointer to a rectangle, or release it.
///
/// No argument releases the confinement and four arguments set it, the
/// way KeySharp does it; any other count is refused with its message.
/// The corners may come in any order: the rectangle is normalized here,
/// and the far edges are exclusive. A value that is not a number is
/// refused with the reason -- KeySharp answers its default object for
/// one, and a wrong value answered silently is the worst outcome.
pub(crate) fn ks_clip_cursor(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let rect = if values.is_empty() {
        None
    } else if values.len() == 4 && !values.iter().any(|value| matches!(value, Value::Unset)) {
        let mut numbers = [0i64; 4];
        for (index, slot) in numbers.iter_mut().enumerate() {
            match values[index].to_number() {
                Some(number) => *slot = number as i64,
                None => {
                    return Err(RuntimeError {
                        message: crate::runtime::requires(
                            index + 1,
                            "ClipCursor",
                            TypeName::Number,
                            &values[index],
                        ),
                        line,
                    });
                }
            }
        }
        let [x1, y1, x2, y2] = numbers;
        Some((x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)))
    } else {
        return Err(RuntimeError {
            message: "ClipCursor requires either zero or four coordinates.".to_string(),
            line,
        });
    };
    rt.desktop()
        .clip_cursor(rect)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `WinFromPoint`: the window at a point of the screen.
///
/// The point that is not given is the position of the pointer, the way
/// KeySharp reads it from GetCursorPos. The answer is the identity of the
/// window, and zero when no window is at the point. A machine that cannot
/// say which window is at a point is refused with the reason: zero means
/// "no window", and not "I do not know".
pub(crate) fn ks_win_from_point(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let mut given: [Option<i64>; 2] = [None, None];
    for index in 0..values.len().min(2) {
        if matches!(values[index], Value::Unset) {
            continue;
        }
        match values[index].to_number() {
            Some(number) => given[index] = Some(number as i64),
            None => {
                return Err(RuntimeError {
                    message: crate::runtime::requires(
                        index + 1,
                        "WinFromPoint",
                        TypeName::Number,
                        &values[index],
                    ),
                    line,
                });
            }
        }
    }
    let (x, y) = match (given[0], given[1]) {
        (Some(x), Some(y)) => (x, y),
        _ => {
            let (px, py) = rt
                .desktop()
                .mouse_position()
                .map_err(|e| desktop_failure(e, line))?;
            (given[0].unwrap_or(px), given[1].unwrap_or(py))
        }
    };
    let answer = rt
        .desktop()
        .the_window_at(x, y)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(answer.unwrap_or(0)))
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
/// The names of the capabilities of the windows of a desktop.
///
/// They are the names of `WinGetCapability`, and they are the ones the
/// documentation of the tree carries.
pub(crate) const THE_WINDOW_CAPABILITIES: [&str; 8] = [
    "AlwaysOnTopGet",
    "AlwaysOnTopSet",
    "WindowEnabledGet",
    "ControlEnabledGet",
    "WindowList",
    "WindowActivate",
    "WindowMinimize",
    "WindowClose",
];

/// The window that a window function of the ks dialect asks about.
///
/// The window that is not there is refused with the reason of v2, and the
/// desktop that cannot list the windows is refused with its own.
fn ks_the_window_of(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<crate::desktop::WindowInfo, RuntimeError> {
    let windows = rt
        .desktop()
        .windows()
        .map_err(|e| desktop_failure(e, line))?;
    match the_window_in(rt, &windows, values.first())? {
        Some(window) => Ok(window),
        None => Err(no_window(line)),
    }
}

/// `WinGetAlwaysOnTop`: whether a window is above the others.
///
/// The machine that cannot say is refused with the reason: the tree does
/// not answer `0` for a state it does not know.
pub(crate) fn ks_win_get_always_on_top(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let window = ks_the_window_of(rt, values, line)?;
    let answer = rt
        .desktop()
        .always_on_top(&window)
        .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(true) => Ok(Value::Int(1)),
        Some(false) => Ok(Value::Int(0)),
        None => Err(RuntimeError {
            message: "the desktop of this run cannot say whether the window is above the others."
                .into(),
            line,
        }),
    }
}

/// `WinGetEnabled`: whether a window is enabled.
///
/// The machine that cannot say is refused with the reason.
pub(crate) fn ks_win_get_enabled(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let window = ks_the_window_of(rt, values, line)?;
    let answer = rt
        .desktop()
        .window_enabled(&window)
        .map_err(|e| desktop_failure(e, line))?;
    match answer {
        Some(true) => Ok(Value::Int(1)),
        Some(false) => Ok(Value::Int(0)),
        None => Err(RuntimeError {
            message: "the desktop of this run cannot say whether the window is enabled.".into(),
            line,
        }),
    }
}

/// `WinGetCapability`: whether this desktop has a capability.
///
/// The name that is not one of the capabilities of the tree is refused with
/// the reason.
/// `WinSetAlwaysOnTop`: put a window above the others, or take it from there.
///
/// The first parameter is the state, the way v2 writes it: `1` enables, `0`
/// disables, `-1` toggles, and the texts `Toggle`, `On` and `Off` are the
/// same three. The second is the title of the window, and the empty one
/// asks about the active window.
///
/// The machine that cannot say whether the window is above the others cannot
/// toggle it either, and the machine that cannot do the change says so: a
/// change that did not happen is never reported as a change that did.
pub(crate) fn win_set_always_on_top(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let asked = match values.first() {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    // The state that is not given is the toggle, the way v1 wrote it.
    let wanted = match asked.trim().to_lowercase().as_str() {
        "1" | "on" | "true" => Some(true),
        "0" | "off" | "false" => Some(false),
        "-1" | "toggle" | "" => None,
        other => {
            return Err(RuntimeError {
                message: format!(
                    "the first parameter of WinSetAlwaysOnTop is not one of 1, 0, -1, Toggle, On or Off: {other}"
                ),
                line,
            });
        }
    };
    // A janela e a segunda casa: o estado vem primeiro, e o alvo da
    // janela vem depois dele.
    let target = crate::desktop::TheWindowWanted::of(values, 1);
    if target.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let windows = rt
        .desktop()
        .windows()
        .map_err(|e| desktop_failure(e, line))?;
    let found = the_windows_of_the_listing(rt, &windows, &target);
    let Some(window) = found.into_iter().next() else {
        return Err(no_window(line));
    };
    let on = match wanted {
        Some(on) => on,
        None => match rt
            .desktop()
            .always_on_top(&window)
            .map_err(|e| desktop_failure(e, line))?
        {
            Some(current) => !current,
            None => {
                return Err(RuntimeError {
                    message:
                        "the desktop of this run cannot say whether the window is above the others, so it cannot toggle it."
                            .into(),
                    line,
                })
            }
        },
    };
    rt.desktop()
        .set_always_on_top(&window, on)
        .map_err(|e| desktop_failure(e, line))?;
    // The answer of a command that answers nothing is the empty text.
    Ok(Value::Text("".into()))
}

pub(crate) fn ks_win_get_capability(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let asked = match values.first() {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    if !THE_WINDOW_CAPABILITIES
        .iter()
        .any(|name| name.eq_ignore_ascii_case(asked.trim()))
    {
        return Err(RuntimeError {
            message: format!(
                "WinGetCapability: {asked} is not one of the capabilities of the tree."
            ),
            line,
        });
    }
    let capabilities = rt.desktop().window_capabilities();
    let answer = capabilities
        .iter()
        .any(|name| name.eq_ignore_ascii_case(asked.trim()));
    Ok(Value::Int(if answer { 1 } else { 0 }))
}

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
            .put(&name, Value::Text(answer.to_string().into()));
    }
    object
        .borrow_mut()
        .put("IsGranted", Value::Int(if granted { 1 } else { 0 }));
    let _ = rt;
    Ok(Value::Object(object))
}

/// `ComObjGet(Name)`: the object of the COM of a name.
///
/// Measured on the binary of v2.0.19: an empty string and an
/// Integer where the class string is wanted both throw "(0x80070057) The
/// parameter is incorrect.". That is the answer of a machine that has no COM
/// to answer, and the desktop that has one gives the object of the name.
///
/// The shape of it is the one of the v2.0.31, read in its source
/// (`source/script_com.cpp` of the reference tree): the name is a moniker of
/// the machine, and the object of it is asked with the `IDispatch`.
pub(crate) fn com_obj_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let object = rt
        .desktop()
        .com_object_of_the_name(&name)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(crate::classes::comvalue::the_value(
        object.pointer,
        object.kind,
        0,
    ))
}

/// `ComObjActive(CLSID)`: the COM object that a program has running.
///
/// Measured on the binary of v2.0.19: a class string that no
/// running program owns throws "(0x800401F3) Invalid class string". That is
/// the answer of a machine that has no COM to answer.
///
/// The shape of it is the one of the v2.0.31, read in its source: the class
/// is the text of a `CLSID`, and the object of it is asked with the
/// `IDispatch`.
pub(crate) fn com_obj_active(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let class = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let object = rt
        .desktop()
        .com_object_of_the_running(&class)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(crate::classes::comvalue::the_value(
        object.pointer,
        object.kind,
        0,
    ))
}

/// `ComObjType(ComValue [, Info])`: the kind of a value of the COM.
///
/// Measured on the binary of v2.0.19: an Integer where a COM
/// object is wanted answers the EMPTY TEXT -- not a failure.
///
/// The kind is the `VARTYPE` of v2. The second parameter asks the machine
/// for the type of the object -- the text of its class, the name of its
/// interface -- and the tree does not carry that part.
pub(crate) fn com_obj_type(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    match crate::classes::comvalue::the_kind_of_the_value(&first) {
        Some(kind) => {
            // The second answer is the name of the class or of the
            // interface, and the machine is the owner of it.
            if let Some(what) = values.get(1) {
                let what = what.to_text();
                let pointer = crate::classes::comvalue::the_number_of(&first).unwrap_or(0);
                let object = crate::desktop::ComObject {
                    pointer: pointer as u64,
                    kind,
                };
                let answer = rt
                    .desktop()
                    .com_type_of_the_object(&object, what.trim())
                    .map_err(|reason| desktop_failure(reason, line))?;
                return Ok(Value::Text(answer.into()));
            }
            Ok(Value::Int(kind))
        }
        // The empty text is the answer of the v2 for a value that is not a
        // COM object: it is not a failure.
        None => Ok(Value::Text("".into())),
    }
}

/// `ComObjQuery(ComValue, IID)`: another interface of the same object.
///
/// Measured on the binary of v2.0.19: an Integer where a COM
/// object is wanted throws "Parameter #1 of ComObjQuery is invalid." -- a
/// wording of its own, and not the one its siblings use.
///
/// The shape of it is the one of the v2.0.31, read in its source: the first
/// argument is the object, the second one is the text of an `IID`, and the
/// machine is the one that turns the text into the number.
pub(crate) fn com_obj_query(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    let pointer = match crate::classes::comvalue::the_number_of(&first) {
        Some(pointer) if pointer >= 65536 => pointer,
        _ => {
            return Err(RuntimeError {
                message: crate::runtime::invalid_parameter(1, "ComObjQuery"),
                line,
            });
        }
    };
    let interface = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let object = rt
        .desktop()
        .com_object_is_queried(
            &crate::desktop::ComObject {
                pointer: pointer as u64,
                kind: crate::classes::comvalue::the_kind_of_the_value(&first).unwrap_or(0),
            },
            &interface,
        )
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(crate::classes::comvalue::the_value(
        object.pointer,
        object.kind,
        0,
    ))
}

/// `ListLines(Setting)`: the log of the lines that the script runs.
///
/// Measured on the binary of v2.0.19: the shape is
/// `ListLines Setting`, where the setting is a **number** -- a text throws
/// "Parameter #1 of ListLines requires a Number, but received a String." --
/// and the answer is the **previous** setting: one when the log was on, and
/// zero when it was off. The log is on by default, and the built-in variable
/// `A_ListLines` carries it and may be written.
///
/// Without the setting the call shows the history of the lines in the window
/// of the debugger of v2 and answers the previous setting too. This runtime
/// has no debugger, so it answers the number and keeps the log.
pub(crate) fn list_lines(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The setting that is there now is the one the call answers.
    let before = match crate::settings::value_of("listlines") {
        Some(Ok(Value::Int(on))) => Value::Int(on),
        _ => Value::Int(1),
    };
    let Some(setting) = values.first() else {
        return Ok(before);
    };
    if !matches!(setting, Value::Int(_) | Value::Float(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ListLines", TypeName::Number, setting),
            line,
        });
    }
    let on = setting.to_number_lenient() != 0.0;
    match crate::settings::write_of("listlines", &Value::Int(on as i64)) {
        Some(Ok(_)) => Ok(before),
        Some(Err(why)) => Err(RuntimeError { message: why, line }),
        None => Err(RuntimeError {
            message: "the log of the lines is not carried.".to_string(),
            line,
        }),
    }
}

/// The kinds of a value that the v2 carries.
///
/// Measured on the binary of v2.0.19: a kind that is not one
/// of these throws "Parameter #2 of RegWrite is invalid.".
const REGISTRY_KINDS: [&str; 5] = [
    "REG_SZ",
    "REG_EXPAND_SZ",
    "REG_MULTI_SZ",
    "REG_DWORD",
    "REG_BINARY",
];

/// The failure of the machine when a value has no name.
///
/// Measured on the binary of v2.0.19: a name of a value that
/// is not given -- and one that is given empty -- throws "(161) The specified
/// path is invalid.".
const NO_VALUE_NAME: &str = "(161) The specified path is invalid.";

/// The failure of the machine when a key or a value is not there.
const NOTHING_THERE: &str = "(2) The system cannot find the file specified.";

/// The name of the key of a registry call, and the failure when it is not
/// there.
///
/// The place of the key is not the same in every call: `RegWrite` takes the
/// value and the kind first, so its key is the third parameter.
///
/// The hive is **required**: measured on the binary of v2.0.19,
/// `RegCreateKey("Software\X")` throws "Parameter #1 of RegCreateKey is
/// invalid." -- and so does the registry of another machine, whose name
/// comes first and is not a hive.
pub(crate) fn the_key_of(
    values: &[Value],
    index: usize,
    name: &str,
    line: u32,
) -> Result<String, RuntimeError> {
    let key = values
        .get(index)
        .map(|value| value.to_text())
        .unwrap_or_default()
        .trim()
        .to_string();
    let hive = key.split('\\').next().unwrap_or("").trim().to_uppercase();
    let known = matches!(
        hive.as_str(),
        "HKCU"
            | "HKEY_CURRENT_USER"
            | "HKLM"
            | "HKEY_LOCAL_MACHINE"
            | "HKU"
            | "HKEY_USERS"
            | "HKCR"
            | "HKEY_CLASSES_ROOT"
            | "HKCC"
            | "HKEY_CURRENT_CONFIG"
    );
    if !known {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(index + 1, name),
            line,
        });
    }
    Ok(key)
}

/// The name of the value of a registry call.
///
/// Measured on the binary of v2.0.19: a name that is not
/// given, and one that is given empty, throw "(161) The specified path is
/// invalid." -- so the default value of a key is out of reach.
pub(crate) fn the_value_name_of(
        values: &[Value],
        index: usize,
        line: u32,
    ) -> Result<String, RuntimeError> {
    let name = values
        .get(index)
        .map(|value| value.to_text())
        .unwrap_or_default()
        .trim()
        .to_string();
    if name.is_empty() {
        return Err(RuntimeError {
            message: NO_VALUE_NAME.to_string(),
            line,
        });
    }
    Ok(name)
}

/// `RegCreateKey`: make a key of the registry of the machine.
///
/// Measured on the binary of v2.0.19: it answers the empty
/// text and makes the key; a key that is not given throws "Parameter #1 of
/// RegCreateKey must not be omitted in this case."; and a key with no hive
/// throws "Parameter #1 of RegCreateKey is invalid.".
pub(crate) fn reg_create_key(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.is_empty() {
        return Err(RuntimeError {
            message: "Parameter #1 of RegCreateKey must not be omitted in this case.".to_string(),
            line,
        });
    }
    let key = the_key_of(values, 0, "RegCreateKey", line)?;
    rt.desktop()
        .registry_create_key(&key)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `RegWrite`: write a value into the registry of the machine.
///
/// Measured on the binary of v2.0.19: the shape is
/// `RegWrite Value, ValueType, KeyName, ValueName` -- the **value comes
/// first** and the type is the second -- and the call answers the empty
/// text. A key that is not given throws "Parameter #3 of RegWrite must not
/// be omitted in this case." -- the key is checked before the kind -- a
/// kind that is not one of the five throws "Parameter #2 of RegWrite is
/// invalid.", a name of a value that is not given throws "(161) The
/// specified path is invalid.", and the key is **made** when it is not
/// there -- with its ancestors.
pub(crate) fn reg_write(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() < 3 {
        return Err(RuntimeError {
            message: "Parameter #3 of RegWrite must not be omitted in this case.".to_string(),
            line,
        });
    }
    let kind = text_of(values, 1).trim().to_uppercase();
    if !REGISTRY_KINDS.contains(&kind.as_str()) {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(2, "RegWrite"),
            line,
        });
    }
    let key = the_key_of(values, 2, "RegWrite", line)?;
    let name = the_value_name_of(values, 3, line)?;
    // A value of REG_DWORD is a number, and the registry holds it as one.
    let value = if kind == "REG_DWORD" {
        format!("{}", number_argument(values, 0, line, "RegWrite")? as i64)
    } else {
        text_of(values, 0)
    };
    rt.desktop()
        .registry_write(&key, &name, &kind, &value)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `RegRead`: the value of the registry of the machine.
///
/// Measured on the binary of v2.0.19: the shape is
/// `RegRead KeyName, ValueName, Default`; a key that is not given throws
/// "Parameter #1 of RegRead must not be omitted in this case."; a name of a
/// value that is not given throws "(161) The specified path is invalid."; a
/// value that is not there throws "(2) The system cannot find the file
/// specified." unless a default is given, which is then the answer.
pub(crate) fn reg_read(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.is_empty() {
        return Err(RuntimeError {
            message: "Parameter #1 of RegRead must not be omitted in this case.".to_string(),
            line,
        });
    }
    let key = the_key_of(values, 0, "RegRead", line)?;
    let name = the_value_name_of(values, 1, line)?;
    match rt
        .desktop()
        .registry_value(&key, &name)
        .map_err(|reason| desktop_failure(reason, line))?
    {
        Some((kind, value)) => {
            // A value of REG_DWORD is a number, and the registry holds it as
            // one: the binary answers 42 for the 42 that was written.
            if kind == "REG_DWORD" {
                Ok(match value.parse::<i64>() {
                    Ok(number) => Value::Int(number),
                    Err(_) => Value::Text(value.into()),
                })
            } else {
                Ok(Value::Text(value.into()))
            }
        }
        // The value is not there: the default is the answer when there is
        // one, and the machine's failure is the answer when there is not.
        None => match values.get(2) {
            Some(default) => Ok(default.clone()),
            None => Err(RuntimeError {
                message: NOTHING_THERE.to_string(),
                line,
            }),
        },
    }
}

/// `RegDelete`: remove a value of the registry of the machine.
///
/// Measured on the binary of v2.0.19: the shape is
/// `RegDelete KeyName, ValueName` and the call answers the empty text; a
/// name of a value that is not given throws "(161) The specified path is
/// invalid.".
pub(crate) fn reg_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.is_empty() {
        return Err(RuntimeError {
            message: "Parameter #1 of RegDelete must not be omitted in this case.".to_string(),
            line,
        });
    }
    let key = the_key_of(values, 0, "RegDelete", line)?;
    let name = the_value_name_of(values, 1, line)?;
    rt.desktop()
        .registry_delete(&key, &name)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `RegDeleteKey`: remove a key of the registry of the machine.
///
/// Measured on the binary of v2.0.19: the shape is
/// `RegDeleteKey KeyName` and the call answers the empty text; a key that is
/// not there throws "(2) The system cannot find the file specified.".
pub(crate) fn reg_delete_key(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.is_empty() {
        return Err(RuntimeError {
            message: "Parameter #1 of RegDeleteKey must not be omitted in this case.".to_string(),
            line,
        });
    }
    let key = the_key_of(values, 0, "RegDeleteKey", line)?;
    rt.desktop()
        .registry_delete_key(&key)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `TraySetIcon(IconFile, IconNumber)`: the icon of the script.
///
/// Measured on the binary of v2.0.19: the call answers the
/// empty text, and the built-in variables carry the icon afterwards --
/// `A_IconFile` answers the path of the file and `A_IconNumber` the number
/// of the icon in it. An icon that cannot be loaded throws **"Can't load
/// icon."**, and the icon of the script is not changed. A call with no
/// argument answers the empty text and changes nothing.
///
/// The path that the binary answers is the canonical one of Windows, which
/// is in upper case; this tree answers the path as the script wrote it,
/// because the path of a file is the path of the machine it is on.
pub(crate) fn tray_set_icon(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(file) = values.first() else {
        // A call with no argument changes nothing.
        return Ok(Value::Text("".into()));
    };
    let file = file.to_text();
    let number = values
        .get(1)
        .map(|value| value.to_number().unwrap_or(1.0) as i64);
    let options = crate::desktop::PictureOptions {
        width: None,
        height: None,
        icon: number,
    };
    // The icon is loaded before it is set: an icon that cannot be loaded is
    // the failure of the call, and the icon of the script stays as it was.
    // The binary does not tell the reasons apart -- every icon that cannot
    // be loaded is "Can't load icon.".
    let image = rt.desktop().load_picture(&file, &options).ok().flatten();
    if image.is_none() {
        return Err(RuntimeError {
            message: "Can't load icon.".to_string(),
            line,
        });
    }
    let canonical = rt
        .desktop()
        .canonical_path(&file)
        .unwrap_or_else(|_| file.clone());
    rt.set_the_icon_of_the_script(canonical.clone(), number.unwrap_or(1));
    // The tray of the session shows the icon, when the session has one: a
    // desktop with no tray is not a failure of the call -- the icon of the
    // script is set either way. A tray that refused is said in the report.
    let refused = rt.desktop().tray_set_icon(&canonical).err();
    if let Some(reason) = refused {
        rt.note(format!(
            "the tray of the session did not take the icon: {reason}"
        ));
    }
    Ok(Value::Text("".into()))
}

/// `CallbackFree(Callback)`: let a callback of the machine go.
///
/// Measured on the binary of v2.0.19: the first parameter
/// wants a number, and a text throws "Parameter #1 of CallbackFree requires
/// a Number, but received a String."; and every number throws "Parameter #1
/// of CallbackFree is invalid.", because the only callback it would let go
/// is one that it made itself.
///
/// This runtime makes no callback for the machine, so every number is the
/// invalid one.
pub(crate) fn callback_free(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    let address = match first {
        Value::Int(n) if n >= 65536 => n as u64,
        Value::Int(_) | Value::Float(_) => {
            return Err(RuntimeError {
                message: "Parameter #1 of CallbackFree is invalid.".to_string(),
                line,
            });
        }
        _ => {
            return Err(RuntimeError {
                message: crate::runtime::requires(1, "CallbackFree", TypeName::Number, &first),
                line,
            });
        }
    };
    rt.desktop().free_callback(address);
    Ok(Value::Text("".into()))
}

/// `ComCall(Method, Object, RetType)`: call a method of a COM object.
///
/// Measured on the binary of v2.0.19: the shape is
/// `ComCall Method, Object, RetType` -- the object is the second parameter
/// and the type of the return is the third. The type is looked at **first**:
/// a type that the dialect does not know throws "Invalid return type." --
/// and the ones it knows are the types of a native call, with the address
/// of the value written after them ("Int*" and "IntP" are read too).
/// The object is looked at after it, and anything that is not a COM object
/// throws "Parameter #2 of ComCall is invalid.".
///
/// The COM objects are objects of the component model of Windows, and this
/// platform has no component model: there is no object of COM to call, so
/// the call answers the failure of the object.
/// `ComCall(Index, Object, Types...)`: the call of a method of an object.
///
/// Measured on the binary of v2.0.19: the shape of ComCall
/// is (Method, Object, RetType), and the type of the return is looked at
/// FIRST -- a type the dialect does not know throws "Invalid return
/// type." -- and the ones it knows are the types of a native call, with
/// the address of the value written after them. The object is looked at
/// after it, and anything that is not a COM object throws "Parameter #2
/// of ComCall is invalid.".
///
/// The shape of the v2.0.31 is the one of its source (`source/lib/DllCall.cpp`
/// of the reference tree): the first argument is the place of the method in
/// the table of the object, the second one is the object itself -- a value
/// of the COM, or the address of one -- and the rest are the pairs of a
/// native call, with the type of the answer at the end when the number of
/// them is odd.
///
/// The answer of a call whose type is not given is an `HRESULT`, and an
/// `HRESULT` that failed is the failure of the call.
pub(crate) fn com_call(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let rest = if values.len() > 2 {
        &values[2..]
    } else {
        &[][..]
    };
    // The type of the return is looked at before the object.
    let (returns, rest) = if rest.len() % 2 == 1 {
        let last = rest.last().expect("there is one");
        let Value::Text(text) = last else {
            return Err(RuntimeError {
                message: "Invalid return type.".to_string(),
                line,
            });
        };
        let (_convention, name) = split_convention(text);
        let Some((kind, _)) = crate::dll::parse_type(name) else {
            return Err(RuntimeError {
                message: "Invalid return type.".to_string(),
                line,
            });
        };
        (kind, &rest[..rest.len() - 1])
    } else {
        (NativeType::Int, rest)
    };
    // The object: a value of the COM, or the address of one.
    let second = values.get(1).cloned().unwrap_or(Value::Int(0));
    let pointer = match crate::classes::comvalue::the_number_of(&second) {
        Some(pointer) if pointer >= 65536 => pointer,
        _ => match second {
            // The v2.0.31 takes the address of one too.
            Value::Int(pointer) if pointer >= 65536 => pointer,
            Value::Float(pointer) if pointer as i64 >= 65536 => pointer as i64,
            _ => {
                return Err(RuntimeError {
                    message: crate::runtime::invalid_parameter(2, "ComCall"),
                    line,
                });
            }
        },
    };
    let index = values
        .first()
        .map(|value| value.to_number().unwrap_or(0.0) as i64);
    let Some(index) = index else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ComCall"),
            line,
        });
    };
    if index < 0 {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "ComCall"),
            line,
        });
    }
    // The arguments after the object: the pairs of a native call.
    let mut arguments = Vec::new();
    let mut references = Vec::new();
    let mut place = 0;
    while place < rest.len() {
        let Value::Text(text) = &rest[place] else {
            return Err(RuntimeError {
                message: "Invalid arg type.".to_string(),
                line,
            });
        };
        let Some((kind, address)) = crate::dll::parse_type(text) else {
            return Err(RuntimeError {
                message: "Invalid arg type.".to_string(),
                line,
            });
        };
        let (argument, reference, _room) = marshal(rt, kind, address, &rest[place + 1], line)?;
        arguments.push(argument);
        references.push(reference);
        place += 2;
    }
    // The object is the first argument of the call.
    let mut all = vec![NativeArg {
        kind: NativeType::Ptr,
        address: false,
        value: NativeValue::Integer(pointer),
    }];
    all.extend(arguments);
    let object = crate::desktop::ComObject {
        pointer: pointer as u64,
        kind: crate::classes::comvalue::the_kind_of_the_value(&second).unwrap_or(0),
    };
    // The kind of the answer that is not given is an `HRESULT`, and an
    // `HRESULT` that failed is the failure of the call.
    let failure_is_an_error = rest.len() % 2 == 0;
    let answer = rt
        .desktop()
        .com_method_is_called(
            &object,
            index as usize,
            &mut all,
            returns,
            failure_is_an_error,
        )
        .map_err(|reason| desktop_failure(reason, line))?;
    for (argument, reference) in all[1..].iter().zip(references) {
        if !argument.address {
            continue;
        }
        let Some(reference) = reference else { continue };
        let NativeValue::Bytes(bytes) = &argument.value else {
            continue;
        };
        let written = if argument.kind.is_text() {
            Value::Text((crate::dll::decode(bytes, argument.kind)).into())
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
        NativeValue::Bytes(bytes) => Value::Text((crate::dll::decode(&bytes, answer.kind)).into()),
    })
}

/// `ComObjFromPtr(Ptr)`: the COM object of a pointer.
///
/// Measured on the binary of v2.0.19: the first parameter
/// wants a number, and a text throws "Parameter #1 of ComObjFromPtr
/// requires a Number, but received a String."; the **null** pointer throws
/// "Parameter #1 of ComObjFromPtr is invalid.".
///
/// The pointer is not touched: the value of the COM carries the address,
/// and the object it points at is the script's.
pub(crate) fn com_obj_from_ptr(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    if !matches!(first, Value::Int(_) | Value::Float(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ComObjFromPtr", TypeName::Number, &first),
            line,
        });
    }
    let pointer = first.to_number_lenient() as i64;
    if pointer == 0 {
        return Err(RuntimeError {
            message: "Parameter #1 of ComObjFromPtr is invalid.".to_string(),
            line,
        });
    }
    Ok(crate::classes::comvalue::the_value(
        pointer as u64,
        crate::classes::comvalue::THE_DISPATCH,
        0,
    ))
}

/// `IL_Create(Width, Height, Grow)`: make a list of images.
///
/// Measured on the binary of v2.0.19: it answers the number
/// that names the list it made.
pub(crate) fn il_create(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(rt.make_an_image_list()))
}

/// `IL_Add(ImageList, IconFile, IconIndex)`: put an image in a list of images.
///
/// Measured on the binary of v2.0.19: it answers the place of
/// the image in the list -- one for the first -- and **zero** when the image
/// could not be loaded: a file that is not there, and a file that is not an
/// image, are both zero and not a failure. The list is looked at first: a
/// number that names no list is zero as well, and the **null** handle is
/// refused with "Parameter #1 of IL_Add is invalid.". A first parameter
/// that is not a number throws "Parameter #1 of IL_Add requires a Number,
/// but received a String.".
pub(crate) fn il_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    if !matches!(first, Value::Int(_) | Value::Float(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "IL_Add", TypeName::Number, &first),
            line,
        });
    }
    let list = number_of(values, 0) as i64;
    if list == 0 {
        return Err(RuntimeError {
            message: "Parameter #1 of IL_Add is invalid.".to_string(),
            line,
        });
    }
    let file = text_of(values, 1);
    // The image is the one of the machine: the same path that `LoadPicture`
    // takes, with the icon that the call names.
    let icon = values
        .get(2)
        .map(|value| value.to_number().unwrap_or(1.0) as i64);
    let options = crate::desktop::PictureOptions {
        width: None,
        height: None,
        icon,
    };
    // An image that cannot be loaded is zero, and not a failure -- the
    // binary answers zero for a file that is not there and for a file that
    // is not an image.
    let image = rt.desktop().load_picture(&file, &options).ok().flatten();
    let Some(image) = image else {
        return Ok(Value::Int(0));
    };
    // A list that is not there is zero as well.
    Ok(Value::Int(
        rt.put_an_image_in_a_list(list, image as i64).unwrap_or(0),
    ))
}

/// `IL_Destroy(ImageList)`: take a list of images away.
///
/// Measured on the binary of v2.0.19: it answers **one** when
/// the list was there and **zero** when it was not -- the null handle among
/// them -- and a first parameter that is not a number throws "Parameter #1
/// of IL_Destroy requires a Number, but received a String.".
pub(crate) fn il_destroy(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = values.first().cloned().unwrap_or(Value::Int(0));
    if !matches!(first, Value::Int(_) | Value::Float(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "IL_Destroy", TypeName::Number, &first),
            line,
        });
    }
    let list = number_of(values, 0) as i64;
    Ok(Value::Int(rt.take_an_image_list(list) as i64))
}

/// `SetCapsLockState`: the state of the lamp of the Caps Lock.
pub(crate) fn set_caps_lock_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_lock_state(rt, values, line, "CapsLock")
}

/// `SetNumLockState`: the state of the lamp of the Num Lock.
pub(crate) fn set_num_lock_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_lock_state(rt, values, line, "NumLock")
}

/// `SetScrollLockState`: the state of the lamp of the Scroll Lock.
pub(crate) fn set_scroll_lock_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    set_lock_state(rt, values, line, "ScrollLock")
}

/// The failure v2 gives when a window is looked for and is not there.
///
/// Measured on the binary of v2.0.19: `WinGetID` and
/// `WinGetClass` fail with it, while `WinGetList` answers an empty array and
/// `WinGetCount` answers the zero.
///
/// It is public because the backends answer it too: the desktop that finds
/// no window says it with these words, and the words are the ones of the
/// dialect.
pub const TARGET_WINDOW: &str = "Target window not found.";

/// The failure of a question about a control, when the window is there.
///
/// Measured on the binary of v2.0.19: `ControlGetText("x", "A")`
/// -- a window that is there -- fails with this, and not with an empty text.
///
/// It is public for the same reason as the one above.
pub const TARGET_CONTROL: &str = "Target control not found.";

/// The window that a `WinGet*` command asks about.
///
/// The parameters after the first are the text of the window and the two
/// exclusions, and the desktop of the box has none of them.
fn window_target(
    values: &[Value],
) -> Result<crate::desktop::TheWindowWanted, RuntimeError> {
    Ok(crate::desktop::TheWindowWanted::of(values, 0))
}

/// `CaretGetPos`: where the caret of the text is.
///
/// Measured on the binary of v2.0.19: with a focused Edit it
/// answers 1 and writes the place of the caret into the two variables of the
/// caller; with no window, or nothing focused that holds a caret, it answers
/// 0 and the variables stay as they were. A call with no variables at all
/// still answers the one or the zero.
pub(crate) fn caret_get_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let caret = rt
        .desktop()
        .gui_caret()
        .map_err(|e| desktop_failure(e, line))?;
    match caret {
        Some((x, y)) => {
            if let Some(value) = values.first() {
                rt.write_reference(value, Value::Int(x));
            }
            if let Some(value) = values.get(1) {
                rt.write_reference(value, Value::Int(y));
            }
            Ok(Value::Int(1))
        }
        None => Ok(Value::Int(0)),
    }
}

/// The wording of a drive that is not there, measured on the binary of
/// v2.0.19: the readers of a drive -- the label and the serial
/// -- throw the path wording, and the operations on the hardware -- eject,
/// lock, unlock and retract -- throw the file wording. Both are the code of
/// the system in front of what it says.
const DRIVE_PATH: &str = "(3) The system cannot find the path specified.";

const DRIVE_FILE: &str = "(2) The system cannot find the file specified.";

/// `DriveGetLabel`: the label of the drive that a path is on.
pub(crate) fn drive_get_label(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_label(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(label) => Ok(Value::Text(label.into())),
        None => Err(RuntimeError {
            message: DRIVE_PATH.to_string(),
            line,
        }),
    }
}

/// `DriveGetSerial`: the serial of the drive that a path is on.
///
/// Measured on the binary of the same day: `DriveGetSerial` of a drive that
/// is there answers a whole number -- the volume serial of Windows. The
/// serial of a Linux drive is the string of the hardware, so a serial that
/// reads as a whole number is answered as one and the rest rides as text.
pub(crate) fn drive_get_serial(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_serial(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(serial) => match serial.parse::<i64>() {
            Ok(number) => Ok(Value::Int(number)),
            Err(_) => Ok(Value::Text(serial.into())),
        },
        None => Err(RuntimeError {
            message: DRIVE_PATH.to_string(),
            line,
        }),
    }
}

/// `DriveGetList`: the drives of the machine.
///
/// Measured on the binary of the same day: `DriveGetList` answers the drive
/// letters run together -- a letter is one character, so no separator is
/// needed -- and a call with an argument throws "Parameter #1 of
/// DriveGetList is invalid.". A drive of this platform is a device name,
/// which is more than one character, so the names ride apart on newlines.
pub(crate) fn drive_get_list(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !values.is_empty() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, "DriveGetList"),
            line,
        });
    }
    let drives = rt
        .desktop()
        .drive_list()
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text(drives.join("\n").into()))
}

/// `DriveGetStatusCD`: the status of the drive a path is on, when it is a
/// CD or DVD reader.
///
/// Measured on the binary of the same day: `DriveGetStatusCD` of a drive
/// that is not a reader throws "Failed".
pub(crate) fn drive_get_status_cd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_status_cd(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(status) => Ok(Value::Text(status.into())),
        None => Err(RuntimeError {
            message: "Failed".to_string(),
            line,
        }),
    }
}

/// The shape of the hardware operations -- eject, lock, unlock and retract:
/// the drive has to be there first, and a drive that is not there throws the
/// file wording. Measured on the binary of the same day.
fn the_drive_for_the_operation(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<String, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_label(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(_) => Ok(path),
        None => Err(RuntimeError {
            message: DRIVE_FILE.to_string(),
            line,
        }),
    }
}

/// `DriveEject`: eject the drive of a path.
pub(crate) fn drive_eject(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = the_drive_for_the_operation(rt, values, line)?;
    rt.desktop()
        .drive_eject(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `DriveLock`: lock the drive of a path against removal.
pub(crate) fn drive_lock(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = the_drive_for_the_operation(rt, values, line)?;
    rt.desktop()
        .drive_lock(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `DriveUnlock`: unlock the drive of a path.
pub(crate) fn drive_unlock(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = the_drive_for_the_operation(rt, values, line)?;
    rt.desktop()
        .drive_unlock(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `DriveRetract`: spin the drive of a path down.
pub(crate) fn drive_retract(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = the_drive_for_the_operation(rt, values, line)?;
    rt.desktop()
        .drive_retract(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `DriveSetLabel`: write a new label on the drive of a path.
///
/// Measured on the binary of the same day: `DriveSetLabel` of a drive that
/// is not there throws the path wording, like the readers do.
pub(crate) fn drive_set_label(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    match rt
        .desktop()
        .drive_label(&path)
        .map_err(|e| desktop_failure(e, line))?
    {
        Some(_) => {}
        None => {
            return Err(RuntimeError {
                message: DRIVE_PATH.to_string(),
                line,
            });
        }
    }
    let label = text_of(values, 1);
    rt.desktop()
        .drive_set_label(&path, &label)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `WinGetID`: the identifier of the first window that matches.
///
/// The answer is a number and not a text: measured on the binary of the
/// v2.0.32, the `Type(WinGetID())` is "Integer" and the `Type(WinGetIDLast())`
/// is "Integer" as well (source `lib/win.cpp:1699-1717` of it: the
/// identifier is a `UINT`).
///
/// **The call does not remember the window it found.** Measured on the binary
/// of the v2.0.32: after the `WinGetID("Magnifier")`, the `WinGetIDLast()` is
/// refused with "Target window not found." -- the window that a call names
/// no window after is about is the one that the `WinExist` found, and not
/// the one that a `WinGet*` read (source `window.cpp:490-494` of the
/// v2.0.32: "the window most recently found by WinExist()").
pub(crate) fn win_get_id(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let wanted = window_target(values)?;
    let windows = the_windows_that_answer(rt, &wanted, line)?;
    match the_window_named_by(rt, &windows, values.first()) {
        Some(window) => Ok(Value::Int(window.id)),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

/// The window that the value names, inside a listing of the windows.
///
/// A number is the identifier of the window, and the text is its title:
/// measured on the binary of v2.0.19, the `WinGetID(102)`
/// is the window that has the identifier 102.
///
/// **The script that names no window asks about the window that it found
/// last.** Measured on the binary of the v2.0.32: the `WinGetClass` and the
/// `WinGetID` without a parameter are refused with "Target window not
/// found.", and they answer about the window that the script found last
/// when there is one -- the "Magnifier", in the measurement. The tree
/// answered with the first window of the listing, and with the window that
/// is in the foreground of the machine.
fn the_window_named_by(
    rt: &mut Runtime<'_>,
    windows: &[crate::desktop::WindowInfo],
    value: Option<&Value>,
) -> Option<crate::desktop::WindowInfo> {
    match value {
        Some(Value::Int(id)) => windows.iter().find(|window| window.id == *id).cloned(),
        Some(value) if !value.to_text().is_empty() => {
            let wanted = value.to_text();
            // The "A" is the window that is in the foreground of the machine:
            // the v2 resolves the target that way for every name of the
            // family (source `window.h:30-31` of the v2.0.32,
            // `USE_FOREGROUND_WINDOW`).
            if wanted.eq_ignore_ascii_case("A") {
                return windows.iter().find(|window| window.active).cloned();
            }
            let mode = crate::settings::title_match_mode();
            windows
                .iter()
                .find(|window| a_window_answers(mode, window, &wanted))
                .cloned()
        }
        _ => the_window_the_script_found_last(rt, windows),
    }
}

/// `WinGetIDLast()`: the identifier of the window a `Win*` call last found.
///
/// Measured on the binary of v2.0.19: after `WinGetID()` finds a
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
    match rt.the_last_window().and_then(|id| id.parse::<i64>().ok()) {
        Some(id) => Ok(Value::Int(id)),
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
    let wanted = window_target(values)?;
    let windows = the_windows_that_answer(rt, &wanted, line)?;
    match the_window_named_by(rt, &windows, values.first()) {
        Some(window) => Ok(Value::Text(window.class.clone().into())),
        None => Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        }),
    }
}

/// `WinGetList`: the identifiers of every window that matches.
///
/// A title that matches nothing is an empty array, and not a failure: measured
/// on the binary of v2.0.19.
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
    let wanted = window_target(values)?;
    let windows = the_windows_that_answer(rt, &wanted, line)?;
    let ids: Vec<Value> = windows
        .iter()
        .map(|window| Value::Text(window.id.to_string().into()))
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
    let wanted = window_target(values)?;
    let windows = the_windows_that_answer(rt, &wanted, line)?;
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
/// frame: measured on the binary of v2.0.19, the frame was
/// 819,479 1787x1262 and the client area 827,479 1771x1254.
pub(crate) fn win_get_client_pos(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    window_geometry_into(rt, values, line, "WinGetClientPos")
}

/// `ControlGetText`: the text of a control of a window.
///
/// The control is found by its name and the window by its title, through the
/// accessibility layer, because that is the only description of the controls of
/// a window that a Wayland session keeps.
///
/// **A control that the application does not publish answers the empty text,
/// and that is not a failure.** Measured on the box with a dump of
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
    // O titulo vazio e a janela que nao esta: medido no binario do
    // v2.0.31, o `ControlGetText` de um controle com um
    // titulo vazio recusa com "Target window not found.".
    let control = text_of(values, 0);
    let wanted = crate::desktop::TheWindowWanted::of(values, 1);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    match rt
        .desktop()
        .control_text(&control, &title)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlAnswer::Found(text) => Ok(Value::Text(text.into())),
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
/// Measured on the binary of v2.0.19 against a window of its own
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
    let control = text_of(values, 0);
    let wanted = crate::desktop::TheWindowWanted::of(values, 1);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
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
/// Measured on the binary of v2.0.19: the answer is the **empty
/// text**, and the text of the control is the new one afterwards. The
/// parameters are the *new text first*, then the control, then the title --
/// which is the order of v2 and not the order of `ControlGetText`.
pub(crate) fn control_set_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    let control = text_of(values, 1);
    let wanted = crate::desktop::TheWindowWanted::of(values, 2);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    match rt
        .desktop()
        .control_set_text(&control, &title, &text)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlEdit::Done => Ok(Value::Text("".into())),
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
/// Measured on the binary of v2.0.19: both answer the **empty
/// text**, and the two failures are the two of `ControlGetText`. The variant is
/// the name -- this one is `ControlClick` and the other is `ControlFocus`.
pub(crate) fn control_act(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let control = text_of(values, 0);
    let wanted = crate::desktop::TheWindowWanted::of(values, 1);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    let answer = if variant == 0 {
        rt.desktop().control_click(&control, &title)
    } else {
        rt.desktop().control_focus(&control, &title)
    }
    .map_err(|e| desktop_failure(e, line))?;
    match answer {
        crate::desktop::ControlEdit::Done => Ok(Value::Text("".into())),
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
/// Measured on the binary of v2.0.19: the answer is the **empty
/// text**, and the control holds the text afterwards. The parameters are the
/// text first, then the control, then the title.
pub(crate) fn control_send_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = text_of(values, 0);
    let control = text_of(values, 1);
    let wanted = crate::desktop::TheWindowWanted::of(values, 2);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    match rt
        .desktop()
        .control_send_text(&control, &title, &text)
        .map_err(|e| desktop_failure(e, line))?
    {
        crate::desktop::ControlEdit::Done => Ok(Value::Text("".into())),
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

/// `WinGetPID`: the process of a window.
///
/// Measured on the binary of v2.0.19: it answers the **Integer** of
/// the process, and a title that no window has is "Target window not found.".
///
/// The chain on this desktop is the accessibility bus: the registry names the
/// applications, a window is a child of one and its `Name` is its title, and the
/// bus answers the `PID` of a name -- measured on the box.
pub(crate) fn win_get_pid(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let wanted = crate::desktop::TheWindowWanted::of(values, 0);
    if wanted.asked && wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    // **The call that names no window asks about the window that the script
    // found last.** Measured on the binary of the v2.0.32: the `WinGetPID()`
    // of a script that found the "Magnifier" answers the process of it, and
    // the `WinGetPID()` of a script that found no window at all is refused
    // with "Target window not found.".
    let title = if values.is_empty() {
        the_title_of_the_window_the_script_found_last(rt, line)?
    } else {
        the_title_of_the_window(rt, &wanted, line)?
    };
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
/// Measured on the binary of v2.0.19: the name is the file name of
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
    let wanted = crate::desktop::TheWindowWanted::of(values, 0);
    if wanted.asked && wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    // **The call that names no window asks about the window that the script
    // found last.** Measured on the binary of the v2.0.32: the
    // `WinGetProcessName()` of a script that found the "Magnifier" answers
    // "Magnify.exe", and the `WinGetProcessName()` of a script that found no
    // window at all is refused with "Target window not found.".
    let title = if values.is_empty() {
        the_title_of_the_window_the_script_found_last(rt, line)?
    } else {
        the_title_of_the_window(rt, &wanted, line)?
    };
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
            });
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
        Some(text) => Ok(Value::Text(text.into())),
        None => Err(RuntimeError {
            message: "Target process not found.".into(),
            line,
        }),
    }
}

/// `Persistent`: whether the script keeps running after its auto-execute ends.
///
/// Measured on the binary of v2.0.19: it answers the Integer **0**.
/// The answer is the value that the script had before the call: the
/// `Persistent` of v2.0.31 (`source/script2.cpp:1988-1996` of the reference)
/// writes the new value and answers the old one, and the old one of a script
/// that never asked is the zero.
///
/// The ask is the thing that keeps the run alive: a script that asked for it
/// does not end with its statements, and the run goes on with the events of
/// the machine. The argument is the ask: the `Persistent` without an argument
/// asks the script to live, and the `Persistent false` asks it not to.
pub(crate) fn persistent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let asked = match values.first() {
        // The `Persistent` without an argument is the request to stay:
        // it is the commonest use, and it is what the v2 does
        // (`aNewValue.value_or(TRUE)`).
        None => true,
        // The v2 takes a number that is not zero as the true, and a text
        // that is not a number as the false.
        Some(value) => token_to_int64(value) != 0,
    };
    let was = rt.the_script_is_persistent();
    rt.the_script_asked_to_stay(asked);
    Ok(Value::Int(was as i64))
}

/// `Critical`: whether the script is not interrupted while it runs.
///
/// Measured on the binary of v2.0.31: it answers the
/// Integer **0** in every form -- with no argument, with `true`, with
/// `false` -- because the runtime is never critical.
///
/// A script of this runtime is one script in one thread, and the
/// criticality is a property of a runtime that has threads to interrupt;
/// the setting changes nothing, and the answer is the zero the reference
/// gives.
pub(crate) fn critical(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Int(0))
}

/// `ListVars`: show the variables of the script.
///
/// Measured on the binary of v2.0.19: it answers the **empty
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
    Ok(Value::Text("".into()))
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
    let (index, monitors) = the_monitor_of(rt, values, name, line)?;
    let Some(monitor) = the_monitor_at(&monitors, index) else {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(1, name),
            line,
        });
    };
    let places = [monitor.left, monitor.top, monitor.right, monitor.bottom];
    for (index, place) in places.iter().enumerate() {
        if values.get(index + 1).is_some() {
            rt.write_reference(&values[index + 1], Value::Int(*place));
        }
    }
    Ok(Value::Int(1))
}

/// The drive that a path is on, and the failure v2 gives when it is not there.
///
/// Measured on the binary of v2.0.19: `DriveGetSpaceFree` of a
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
/// Measured on the binary of v2.0.19: a drive that is there is
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
        Some(_) => Ok(Value::Text("Ready".into())),
        None => Ok(Value::Text("".into())),
    }
}

/// `DriveGetFilesystem`: the filesystem of a drive, as the machine names it.
///
/// Measured on the binary of v2.0.19: a drive that is not there
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
        Some(space) => Ok(Value::Text(space.filesystem.into())),
        None => Ok(Value::Text("".into())),
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
        Some(space) => Ok(Value::Text(space.kind.into())),
        None => Ok(Value::Text("".into())),
    }
}

/// The longest a wait for a window may block.
///
/// It is the cap of `Sleep`, and it is there for the same reason: the runner's
/// timeout is checked between two statements, so a call that blocked forever
/// could not be stopped by it. The binary of v2.0.19 waits for a long time when
/// the seconds are left out of a `WinWait`, measured; this
/// runtime bounds it.
const MAX_WINDOW_WAIT_MS: u64 = 60_000;

/// The seconds of a wait, in milliseconds.
///
/// The third parameter of v2 is the seconds. A call without it waits the
/// cap.
///
/// The timeout that is not written is a wait that does not end: it is
/// what the v1 does with the `WinWait, titulo` without a timeout,
/// measured on the binary of v1.1.37.02.
///
/// The zero is not the same thing in the two dialects, and the difference
/// was measured on the binaries:
///
/// - the v1 says that the zero is the meio segundo, and a wait of the
///   meio segundo is what the binary of v1.1.37.02 does;
/// - the v2 answers at once, without waiting, and that is what the
///   binary of v2.0.31 does.
fn window_wait_timeout(rt: &Runtime<'_>, values: &[Value]) -> u64 {
    let seconds = match values.get(2) {
        Some(value) => match value.to_number() {
            Some(seconds) => seconds,
            // The timeout that is not a number is the timeout that is
            // not written: the wait does not end.
            None => return MAX_WINDOW_WAIT_MS,
        },
        None => return MAX_WINDOW_WAIT_MS,
    };
    if seconds < 0.0 {
        return MAX_WINDOW_WAIT_MS;
    }
    if seconds == 0.0 {
        // O zero do v1 e o meio segundo, e o zero do v2 e uma espera
        // que volta logo.
        return if rt.dialect().is_the_v1() { 500 } else { 0 };
    }
    (seconds * 1000.0).min(MAX_WINDOW_WAIT_MS as f64) as u64
}

/// The title that a wait looks for.
///
/// The title that is not written is the last window that a script found:
/// the v1 and the v2 ask for it that way. The source of the v1
/// (`window.cpp:571-576` of v1.1.37.02) says it for the `WinExist`, and
/// the waits ask the `WinExist` for the window.
///
/// The rule needs nothing else: a text or an exclusion makes the title a
/// plain title, and the title that is empty with an exclusion is the title
/// of every window.
///
/// The window is named by its title, and not by its identifier: the
/// desktop of the box matches the titles. With no window found the title
/// stays empty, and the empty title is the title of no window at all.
fn the_title_of_a_wait(rt: &mut Runtime<'_>, values: &[Value]) -> String {
    let title = text_of(values, 0);
    if !title.is_empty() {
        return title;
    }
    let text = text_of(values, 1);
    let excludes: Vec<String> = values
        .iter()
        .skip(3)
        .take(2)
        .map(|value| value.to_text())
        .filter(|text| !text.is_empty())
        .collect();
    if !text.is_empty() || !excludes.is_empty() {
        return title;
    }
    let id = if rt.dialect().is_the_v1() {
        rt.the_last_window_of_the_v1().map(|id| id.to_string())
    } else {
        rt.the_last_window().map(|id| id.to_string())
    };
    let Some(id) = id else {
        return title;
    };
    let Ok(windows) = rt.desktop().windows() else {
        return title;
    };
    windows
        .into_iter()
        .find(|window| window.id.to_string() == id)
        .map(|window| window.title)
        .unwrap_or(title)
}

/// Wait for the condition of a `WinWait` family command.
fn window_wait_into(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    what: crate::desktop::WaitFor,
) -> Result<Option<String>, RuntimeError> {
    let title = the_title_of_a_wait(rt, values);
    let mode = crate::settings::title_match_mode();
    let timeout = window_wait_timeout(rt, values);
    // The exclusions are the titles of the windows that do not answer
    // the wait. The title is what the script excludes, and the text of
    // the window needs the text of the controls, which the desktop of
    // the box does not have.
    let excludes: Vec<String> = values
        .iter()
        .skip(3)
        .take(1)
        .map(|value| value.to_text())
        .filter(|text| !text.is_empty())
        .collect();
    rt.desktop()
        .window_wait(&title, mode, what, timeout, &excludes)
        .map_err(|e| desktop_failure(e, line))
}

/// `WinWait`: wait for a window that matches to exist.
///
/// The answer of v2 is the identifier of the window, or the zero when the
/// timeout ran out -- measured on the binary of v2.0.19. The
/// identifier of a Wayland window is a token, so it is text here.
pub(crate) fn win_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    match window_wait_into(rt, values, line, crate::desktop::WaitFor::Present)? {
        Some(id) => Ok(Value::Text(id.into())),
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
        Some(id) => Ok(Value::Text(id.into())),
        None => Ok(Value::Int(0)),
    }
}

/// `WinWaitNotActive`: wait for no matching window to have the focus.
///
/// v2 answers `1` and `0` for this one, and not the identifier -- measured on
/// the binary of v2.0.19.
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
/// there is the failure of v2, measured on the binary of v2.0.19.
pub(crate) fn win_get_min_max(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let wanted = window_target(values)?;
    let mode = crate::settings::title_match_mode();
    // **The call that names no window asks about the window that the script
    // found last.** Measured on the binary of the v2.0.32: the
    // `WinGetMinMax()` of a script that found the "Magnifier" answers 0, and
    // the `WinGetMinMax()` of a script that found no window at all is
    // refused with "Target window not found.".
    let title = if values.is_empty() {
        the_title_of_the_window_the_script_found_last(rt, line)?
    } else {
        wanted.title.clone()
    };
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
    // **The call that names no window asks about the window that the script
    // found last**, and the fifth parameter is the window. Measured on the
    // binary of the v2.0.32: the `WinGetPos()` of a script that found the
    // "Magnifier" answers the place of it, and the `WinGetPos()` of a script
    // that found no window at all is refused with "Target window not
    // found.".
    let title = match values.get(4) {
        None => the_title_of_the_window_the_script_found_last(rt, line)?,
        Some(_) => text_of(values, 4),
    };
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
    Ok(Value::Text("".into()))
}

/// `Sleep`: the pause, in milliseconds.
/// `EditGetLineCount(Hwnd)`: how many lines the text of a control holds.
///
/// Measured on the binary of v2.0.19: a control added with three
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
/// Measured on the binary of v2.0.19: **the number comes first and
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
    Ok(Value::Text(found.to_string().into()))
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
        .map(Value::text)
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
        .map(Value::text)
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
    Ok(Value::Text("".into()))
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
    // A control given by its handle is named by the class of it and its
    // place among the controls of the same class of its parent.
    if let Some(Value::Int(hwnd)) = values.first() {
        let classnn = rt
            .desktop()
            .control_classnn_of(*hwnd)
            .map_err(|reason| desktop_failure(reason, line))?;
        return Ok(Value::Text(classnn.into()));
    }
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
    Ok(Value::Text(classnn.into()))
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
    Ok(Value::Text("".into()))
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
        Some(_) => Ok(Value::Text("".into())),
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
/// Measured on the binary of v2.0.19: the **text comes first** and
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
    Ok(Value::Text("".into()))
}

/// `ControlMove(X, Y, W, H, Control, WinTitle)`: moves a control of a window.
///
/// Measured on the binary of v2.0.19: the shape is
/// `ControlMove X, Y, W, H, Control, WinTitle` -- the **control is the
/// fifth parameter**, and a call that leaves it out is refused with
/// "Parameter #5 of ControlMove must not be omitted in this case."; the
/// first parameter is a number, and a text throws "Parameter #1 of
/// ControlMove requires a Number, but received a String."; and a control
/// or a window that is not there throws "Target window not found.".
///
/// **A call that names a control that IS there hangs the binary.** Measured
///: `ControlMove("10", "20", "100", "30", "Edit1", "title")`
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
            message: "Parameter #5 of ControlMove must not be omitted in this case.".to_string(),
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
            message: "the binary of v2.0.19 hangs on this call -- measured -- and this runtime does not wait forever: the window of this tree lays its controls out in order, so the place of a control cannot be set.".to_string(),
            line,
        }),
        // A control that is not there is the failure of the target.
        Err(_) => Err(RuntimeError {
            message: "Target window not found.".to_string(),
            line,
        }),
    }
}

/// `WinSetTitle(NewTitle, WinTitle)`: rename a window of the script.
///
/// Measured on the binary of v2.0.19: it answers the empty text after
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
    Ok(Value::Text("".into()))
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
/// Measured on the binary of v2.0.19: both answer the empty text, and
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
    Ok(Value::Text("".into()))
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
/// Measured on the binary of v2.0.19: all four answer the empty text,
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
    // The v2 reads the .Hwnd of an object target: a plain object has none,
    // and the read is a PropertyError, not a target that is not found.
    if let Some(value @ Value::Object(_)) = values.first() {
        rt.read_property(value, "Hwnd", line)
            .map_err(|failure| RuntimeError {
                message: the_message_of(failure),
                line,
            })?;
    }
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_option(state, true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

pub(crate) fn win_activate(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_state_of_the_window(rt, values, line, "activate")
}

/// `WinActivateBottom`: activate the window at the bottom of the ones that
/// match.
///
/// Measured on the binary of v2.0.19, with two windows of the
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
/// Measured on the binary of v2.0.19: it answers the empty text, and a
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
    Ok(Value::Text("".into()))
}

/// `WinMoveTop(WinTitle)`: bring the script's window to the front.
///
/// Measured on the binary of v2.0.19: it answers the empty text, and a
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

/// `WinMinimizeAll` and `WinMinimizeAllUndo`: minimize every window of the
/// session and undo it.
///
/// Measured on the binary of v2.0.19: both answer the empty
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
        return Ok(Value::Text("".into()));
    }
    rt.desktop()
        .gui_option("minimize", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `WinMinimizeAllUndo`: put the minimized windows back.
pub(crate) fn win_minimize_all_undo(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if !rt.desktop().gui_is_open() {
        return Ok(Value::Text("".into()));
    }
    rt.desktop()
        .gui_option("restore", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `WinMoveBottom(WinTitle)`: send the window to the bottom of the order.
///
/// Measured on the binary of v2.0.19: it answers the empty
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

/// `WinClose(WinTitle)` and `WinKill(WinTitle)`: close the script's window.
///
/// Measured on the binary of v2.0.19: both answer the empty text, and a
/// window that is not there answers *"Target window not found."*
///
/// **The script's own window and not another program's**: round 31 reverted a
/// `WinClose` because the compositor's toplevel listing publishes no close request,
/// and that is still true of a foreign window. Its own is the `close` word of the
/// option channel -- the helper made the window and can close it.
fn the_close_of_the_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    the_window_of_the_script(rt, &target, line)?;
    rt.desktop()
        .gui_option("close", true)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
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
/// Measured on the binary of v2.0.19: the set answers the empty text,
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
    Ok(Value::Text("".into()))
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
/// Measured on the binary of v2.0.19: it answers the empty text, and a
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
/// Measured on the binary of v2.0.19: `ObjPtr` answers an Integer and
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
            });
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
/// Measured on the binary of v2.0.19: the pair round-trips, and a value
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
            });
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

/// `SoundGetName(Component, Device)`: the name of a sound device.
///
/// Measured on the binary of v2.0.19: the shape is
/// `SoundGetName Component, Device`; with no component the answer is the
/// name of the device that plays -- "Speakers" on the machine of that
/// day -- and with the component one the name of the one that records
/// -- "Microphone". The component two answers the **empty text**, and a
/// component that is not there throws **"Component not found."**.
///
/// The name of a device is the name that the machine shows of it, and
/// the v2.0.31 takes it from the property `PKEY_Device_FriendlyName`
/// of the device -- read in its source (`source/lib/sound.cpp` of the
/// reference tree). The tree answers the same way.
///
/// The name is the name of the machine: the mixer of the session publishes
/// it, and this runtime answers the volume and the mute of the same
/// device already.
pub(crate) fn sound_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let component = text_of(values, 0).trim().to_string();
    let device = text_of(values, 1).trim().to_string();
    let (plays, records) = rt
        .desktop()
        .sound_device_names()
        .map_err(|reason| desktop_failure(reason, line))?;
    // A device that is named is the one that is asked for.
    if !device.is_empty() {
        return match rt
            .desktop()
            .sound_device_named(&device)
            .map_err(|reason| desktop_failure(reason, line))?
        {
            Some(name) => Ok(Value::Text(name.into())),
            None => Err(RuntimeError {
                message: "Component not found".to_string(),
                line,
            }),
        };
    }
    match component.as_str() {
        // The device that plays, and the one that records.
        "" => Ok(Value::Text(plays.into())),
        "1" => Ok(Value::Text(records.into())),
        // The second component of the device is there and it has no name.
        "2" => Ok(Value::Text("".into())),
        _ => Err(RuntimeError {
            message: "Component not found".to_string(),
            line,
        }),
    }
}

/// `SoundGetInterface(IID, Component, Device)`: an interface of a sound device.
///
/// Measured on the binary of v2.0.19: the first parameter is
/// the **identifier of the interface** -- a GUID in the form
/// "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" -- and anything else throws
/// "Parameter #1 of SoundGetInterface is invalid.". A GUID that the machine
/// does not carry the interface of answers **zero**.
///
/// The interfaces of the sound of Windows are COM objects of Windows, and
/// this platform has no COM: every GUID that is a GUID answers zero, the
/// way the binary answers for the interfaces it does not support.
pub(crate) fn sound_get_interface(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let iid = text_of(values, 0).trim().to_string();
    if !is_the_identifier_of_an_interface(&iid) {
        return Err(RuntimeError {
            message: "Parameter #1 of SoundGetInterface is invalid.".to_string(),
            line,
        });
    }
    let device = device_of(values, 1);
    let pointer = rt
        .desktop()
        .sound_interface_of_the_device(&device, &iid)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Int(pointer as i64))
}

/// Whether a text is the identifier of an interface.
///
/// The form is the one of the binary of v2.0.19, measured:
/// the braces, and eight hexadecimal numbers, four, four, four and twelve.
fn is_the_identifier_of_an_interface(text: &str) -> bool {
    let Some(inner) = text
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
    else {
        return false;
    };
    let parts: Vec<&str> = inner.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let widths = [8, 4, 4, 4, 12];
    parts
        .iter()
        .zip(widths)
        .all(|(part, width)| part.len() == width && part.chars().all(|c| c.is_ascii_hexdigit()))
}

/// `WinGetTransColor`: the colour a window lets through.
///
/// The colour is the one of a **layered** window: the window that lets the
/// pixels of the screen through, and the colour that is not drawn. A window
/// that is not layered has no such colour, and the answer is the empty text.
///
/// Measured on the binary of v2.0.31: the window of the script
/// answered the empty text, and the colour is written as "0xRRGGBB" in the
/// order of the bits of the red, the green and the blue.
pub(crate) fn win_trans_color(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let (_wanted, window) = the_target_of_a_window(rt, values, 0, line)?;
    let colour = rt
        .desktop()
        .window_trans_color(&window.title)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text(colour.into()))
}

/// `WinSetTransColor`: the colour a window lets through.
///
/// The colour is written as the name of one of the seventeen colours of HTML,
/// as a hexadecimal number, or as the word "Off" -- and "Off" takes the colour
/// key away.
///
/// Measured on the binary of v2.0.31: a colour that is not a
/// colour is "Parameter #1 of WinSetTransColor is invalid." with the text in
/// the `Extra`; a number is the colour in the order of the bits of the red,
/// the green and the blue, and a float is a refusal.
pub(crate) fn win_set_trans_color(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let colour: u32 = match values.first() {
        Some(Value::Int(number)) => (number & 0x00FF_FFFF) as u32,
        Some(Value::Text(text)) => {
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("off") {
                crate::window_style::COLOUR_NONE
            } else {
                match crate::window_style::the_colour_of_a_text(trimmed) {
                    Some(colour) => colour,
                    None => {
                        return Err(RuntimeError {
                            message: "Parameter #1 of WinSetTransColor is invalid.".to_string(),
                            line,
                        });
                    }
                }
            }
        }
        // The v2 refuses a number that is not a whole one and an object: the
        // colour is a value of its own.
        _ => {
            return Err(RuntimeError {
                message: "Parameter #1 of WinSetTransColor is invalid.".to_string(),
                line,
            });
        }
    };
    let (_wanted, window) = the_target_of_a_window(rt, values, 1, line)?;
    // The desktop takes the colour in the order of the bits of the red, the
    // green and the blue, and the empty text is the colour key taken away.
    let written = if colour == crate::window_style::COLOUR_NONE {
        String::new()
    } else {
        format!("0x{:06X}", colour)
    };
    rt.desktop()
        .window_set_trans_color(&window.title, &written)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `WinGetStyle` and `WinGetExStyle`: the style of a window.
///
/// The style is the number of the bits of Windows: the `WS_*` bits say how a
/// window is decorated and what it may do, and the extended style is the
/// `WS_EX_*` of it. The machine is the one that keeps them, and the desktop
/// is asked.
///
/// Measured on the binary of v2.0.31: the style of the window
/// of the script is `80674816` (`0x04CF0000` -- `WS_OVERLAPPEDWINDOW` with
/// the `WS_CLIPSIBLINGS` the machine adds) and its extended style is `256`
/// (`WS_EX_WINDOWEDGE`); a window that is not there is "Target window not
/// found." with the title in the `Extra`.
pub(crate) fn win_style(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let extended = variant == 1;
    let (_wanted, window) = the_target_of_a_window(rt, values, 0, line)?;
    let style = rt
        .desktop()
        .window_style(&window.title, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Int(style))
}

/// `WinSetStyle` and `WinSetExStyle`: the style of a window is written.
///
/// The text of the style says what to do with the style that is there: `+`
/// adds the bits, `-` takes them away, `^` turns them around, and a text
/// without a prefix replaces the style.
///
/// Measured on the binary of v2.0.31: an empty text is
/// "Parameter #1 of WinSetStyle is invalid." with "empty string" in the
/// `Extra`, and a text that holds no number is **not** a refusal -- the v2
/// reads it as zero, the way its `ATOU` reads it.
pub(crate) fn win_set_style(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let extended = variant == 1;
    let name = if variant == 1 {
        "WinSetExStyle"
    } else {
        "WinSetStyle"
    };
    let text = text_of(values, 0);
    let Some(change) = crate::window_style::the_style_change_of_a_text(&text) else {
        return Err(RuntimeError {
            message: format!("Parameter #1 of {name} is invalid."),
            line,
        });
    };
    let (_wanted, window) = the_target_of_a_window(rt, values, 1, line)?;
    let current = rt
        .desktop()
        .window_style(&window.title, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    let wanted = change.the_style_of(current as u32);
    rt.desktop()
        .window_set_style(&window.title, wanted as i64, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// `ControlGetStyle` and `ControlGetExStyle`: the style of a control.
///
/// A control is a window of its own, and its style is the number of the bits
/// of Windows the way the style of a window is.
pub(crate) fn control_style(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let extended = variant == 1;
    let control = text_of(values, 0);
    let wanted = crate::desktop::TheWindowWanted::of(values, 1);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    // The window and the control have to be there: measured on the binary of
    // v2.0.31, the refusal is "Target window not found." or
    // "Target control not found." -- and not with the reason of the desktop.
    match rt
        .desktop()
        .control_text(&control, &title)
        .map_err(|reason| desktop_failure(reason, line))?
    {
        crate::desktop::ControlAnswer::Found(_) => {}
        crate::desktop::ControlAnswer::NoSuchWindow => {
            return Err(RuntimeError {
                message: TARGET_WINDOW.into(),
                line,
            });
        }
        crate::desktop::ControlAnswer::NoSuchControl => {
            return Err(RuntimeError {
                message: TARGET_CONTROL.into(),
                line,
            });
        }
    }
    let style = rt
        .desktop()
        .control_style(&control, &title, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Int(style))
}

/// `ControlSetStyle` and `ControlSetExStyle`: the style of a control is written.
///
/// The text of the style says what to do with the style that is there, the way
/// it says it for a window.
pub(crate) fn control_set_style(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let extended = variant == 1;
    let name = if variant == 1 {
        "ControlSetExStyle"
    } else {
        "ControlSetStyle"
    };
    let text = text_of(values, 0);
    let Some(change) = crate::window_style::the_style_change_of_a_text(&text) else {
        return Err(RuntimeError {
            message: format!("Parameter #1 of {name} is invalid."),
            line,
        });
    };
    // A control given by its handle: the machine reads its style and
    // writes the new one, without a window of the script being open.
    if let Some(Value::Int(hwnd)) = values.get(1) {
        let current = rt
            .desktop()
            .control_style_of_hwnd(*hwnd, extended)
            .map_err(|reason| desktop_failure(reason, line))?;
        let wanted = change.the_style_of(current as u32);
        rt.desktop()
            .control_set_style_of_hwnd(*hwnd, wanted as i64, extended)
            .map_err(|reason| desktop_failure(reason, line))?;
        return Ok(Value::Text("".into()));
    }
    let control = text_of(values, 1);
    let wanted = crate::desktop::TheWindowWanted::of(values, 2);
    if wanted.title.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.into(),
            line,
        });
    }
    let title = the_title_of_the_window(rt, &wanted, line)?;
    // The window and the control have to be there: measured on the binary of
    // v2.0.31, the refusal is "Target window not found." or
    // "Target control not found." -- and not with the reason of the desktop.
    match rt
        .desktop()
        .control_text(&control, &title)
        .map_err(|reason| desktop_failure(reason, line))?
    {
        crate::desktop::ControlAnswer::Found(_) => {}
        crate::desktop::ControlAnswer::NoSuchWindow => {
            return Err(RuntimeError {
                message: TARGET_WINDOW.into(),
                line,
            });
        }
        crate::desktop::ControlAnswer::NoSuchControl => {
            return Err(RuntimeError {
                message: TARGET_CONTROL.into(),
                line,
            });
        }
    }
    let current = rt
        .desktop()
        .control_style(&control, &title, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    let wanted = change.the_style_of(current as u32);
    rt.desktop()
        .control_set_style(&control, &title, wanted as i64, extended)
        .map_err(|reason| desktop_failure(reason, line))?;
    Ok(Value::Text("".into()))
}

/// The window that a call of the family of the style names.
///
/// The target of a `WinGetStyle` is its first parameter and the target of a
/// `WinSetStyle` is its second one: the style itself comes first.
///
/// The title that is empty is the failure of v2: the v2 falls back to the last
/// found window, and this runtime keeps none. The window that is not there is
/// the same failure, and the desktop says it with the words of the dialect
/// (`ahk::builtins::TARGET_WINDOW`).
///
/// Os parametros depois do alvo sao o texto da janela e as duas
/// exclusoes, e o alvo inteiro e o que a janela nomeia.
fn the_target_of_a_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    index: usize,
    line: u32,
) -> Result<(crate::desktop::TheWindowWanted, crate::desktop::WindowInfo), RuntimeError> {
    let mut wanted = crate::desktop::TheWindowWanted::of(values, index);
    if wanted.title.is_empty() {
        // A command of the family that names no window asks about the
        // last one a `Win*` call found (`WinExist` and the rest remember
        // it). With no window found yet, there is no target.
        match rt.the_last_window() {
            Some(id) => wanted.title = format!("ahk_id {id}"),
            None => {
                return Err(RuntimeError {
                    message: TARGET_WINDOW.to_string(),
                    line,
                });
            }
        }
    }
    let windows = rt
        .desktop()
        .windows()
        .map_err(|reason| desktop_failure(reason, line))?;
    let found = the_windows_of_the_listing(rt, &windows, &wanted);
    match found.into_iter().next() {
        Some(window) => Ok((wanted, window)),
        None => Err(no_window(line)),
    }
}

/// The title of the window that the target names.
///
/// A janela e a primeira que responde ao alvo, e o titulo dela e o que
/// o desktop recebe: o desktop procura a janela pelo titulo, e o alvo
/// ja escolheu qual das janelas com o mesmo titulo e a que responde.
///
/// The window that the criterion `ahk_id` names is the one that the
/// desktop receives by the handle: the handle is the one that the v1 uses
/// (source `window.cpp:586-606` of v1.1.37.02), and the title of a window
/// that is a child of another one does not name the window -- two windows
/// can have the same title, and only the handle says which of them is
/// the one that the script asked for.
///
/// Measured on the binary of v1.1.37.02: the
/// `ControlGetText, , ahk_id %hButton%` of a button of a GUI of the
/// script answers the text of the button.
fn the_title_of_the_window(
    rt: &mut Runtime<'_>,
    wanted: &crate::desktop::TheWindowWanted,
    line: u32,
) -> Result<String, RuntimeError> {
    let windows = rt
        .desktop()
        .windows()
        .map_err(|reason| desktop_failure(reason, line))?;
    match the_windows_of_the_listing(rt, &windows, wanted)
        .into_iter()
        .next()
    {
        Some(window) => {
            let criteria = crate::desktop::criteria::WindowCriteria::parse(&wanted.title);
            if criteria.id.is_some() {
                return Ok(window.id.to_string());
            }
            Ok(window.title)
        }
        None => Err(no_window(line)),
    }
}


/// `WinGetText(WinTitle)`: the text of every control of the window.
///
/// Measured on the binary of v2.0.19: a window with a label and an edit
/// answers the two texts, one after the other, **each followed by a line break**, and a
/// window that is not there answers *"Target window not found."*
///
/// **The window of the script is the one that the tree made**, and the text
/// of it is the text of the controls that the script made. A window of
/// another program is a window of the machine, and the desktop reads the
/// texts of the child windows of it. Measured on the binary of the v2.0.32:
/// the `WinGetText` of a window of another program answers the texts of its
/// child windows, and the call that names no window asks about the window
/// that the script found last.
pub(crate) fn win_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    if the_script_owns_the_window(rt, &target, line)? {
        the_window_of_the_script(rt, &target, line)?;
        return rt
            .desktop()
            .gui_all_text()
            .map(Value::text)
            .map_err(|reason| desktop_failure(reason, line));
    }
    let id = the_identifier_of_the_window_the_call_names(rt, values, line)?;
    match rt
        .desktop()
        .window_text(&id)
        .map_err(|reason| desktop_failure(reason, line))?
    {
        Some(text) => Ok(Value::Text(text.into())),
        None => Err(no_window(line)),
    }
}

/// `WinGetControls(WinTitle)`: the `ClassNN` of every control of the script's window.
///
/// Measured on the binary of v2.0.19: a window with a label, an edit and a
/// button answers three items, `Static1`, `Edit1` and `Button1`, and a window that is not
/// there answers *"Target window not found."*
pub(crate) fn win_get_controls(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let target = text_of(values, 0);
    // **The window of the script is the one that the tree made**, and the
    // controls of it are the ones of the tree. A window of another program
    // is a window of the machine, and the desktop enumerates the controls
    // of it by the identifier of it. Measured on the binary of the v2.0.32:
    // the `WinGetControls` of a window of another program answers the
    // controls of it, and the call that names no window asks about the
    // window that the script found last.
    let joined = if the_script_owns_the_window(rt, &target, line)? {
        the_window_of_the_script(rt, &target, line)?;
        rt.desktop()
            .gui_controls()
            .map_err(|reason| desktop_failure(reason, line))?
    } else {
        let id = the_identifier_of_the_window_the_call_names(rt, values, line)?;
        match rt
            .desktop()
            .window_controls(&id, false)
            .map_err(|reason| desktop_failure(reason, line))?
        {
            Some(joined) => joined,
            None => return Err(no_window(line)),
        }
    };
    let answers: Vec<Value> = if joined.is_empty() {
        Vec::new()
    } else {
        joined
            .split('\n')
            .map(|name| Value::Text(name.to_string().into()))
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
/// Measured on the binary of v2.0.19: it answers as many Integers as the
/// window has controls. No channel of its own is needed: the names come from
/// `WinGetControls` and the number of a name from the lookup of round 164.
pub(crate) fn win_get_controls_hwnd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // A window given by its handle is a window of another program: the
    // machine enumerates its controls, and the answer is their handles.
    if let Some(Value::Int(hwnd)) = values.first() {
        let hwnds = rt
            .desktop()
            .control_hwnds_of(*hwnd)
            .map_err(|reason| desktop_failure(reason, line))?;
        let answers: Vec<Value> = hwnds.into_iter().map(Value::Int).collect();
        return match crate::classes::construct("array", &answers, rt, line) {
            Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
            None => Ok(Value::List(answers)),
        };
    }
    let target = text_of(values, 0);
    // The window of the script is the one that the tree made, and the
    // controls of it are the ones of the tree. A window of another program
    // is a window of the machine, and the desktop enumerates the controls of
    // it by the identifier of it.
    let mut answers: Vec<Value> = Vec::new();
    if the_script_owns_the_window(rt, &target, line)? {
        the_window_of_the_script(rt, &target, line)?;
        let joined = rt
            .desktop()
            .gui_controls()
            .map_err(|reason| desktop_failure(reason, line))?;
        if !joined.is_empty() {
            for name in joined.split('\n') {
                let number = rt
                    .desktop()
                    .gui_number_named(name)
                    .map_err(|reason| desktop_failure(reason, line))?;
                answers.push(Value::Int(number));
            }
        }
    } else {
        let id = the_identifier_of_the_window_the_call_names(rt, values, line)?;
        let joined = match rt
            .desktop()
            .window_controls(&id, true)
            .map_err(|reason| desktop_failure(reason, line))?
        {
            Some(joined) => joined,
            None => return Err(no_window(line)),
        };
        for item in joined.split('\n') {
            // The handles of the machine come as the "0x" and the number in
            // the hexadecimal, and the answer of the v2 is a number.
            let text = item.trim();
            let number = i64::from_str_radix(text.trim_start_matches("0x"), 16).unwrap_or(0);
            answers.push(Value::Int(number));
        }
    }
    match crate::classes::construct("array", &answers, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(answers)),
    }
}

/// `Edit()`: open the script of the run in the editor of the machine.
///
/// Measured on the binary of v2.0.31: the name takes no
/// parameter, and it opens the file of the script with the program that
/// the machine opens a file of that kind with.
///
/// The script of this runtime comes from the standard input unless the
/// host gave a file, and a script that comes from there has no file for an
/// editor to open: the name refuses with the reason of that fact.
pub(crate) fn edit(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(path) = rt.script_path().map(str::to_string) else {
        return Err(RuntimeError {
            message: "the script of this runtime is read from the standard input, so it has no file for an editor to open".to_string(),
            line,
        });
    };
    rt.desktop()
        .open_with_the_default_program(&path)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Text("".into()))
}

/// `Reload()`: the script is run again.
///
/// Measured on the binary of v2.0.31: the name takes no
/// parameter, and it starts the script again in a process of its own --
/// the old instance ends, and the new one carries on.
///
/// A script that comes from the standard input has no file to run
/// again, and the v2 does nothing in that case (the source of it says
/// so: `source/script.cpp:1168`, the `Script::Reload` answers `OK` for
/// a script that came from there).
pub(crate) fn reload(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if rt.script_path().is_none() {
        // A script that comes from the standard input has no file, and the
        // v2 does nothing when the script is asked to run again.
        return Ok(Value::Text("".into()));
    }
    // The runner is the program of the script, and it is the runner that
    // starts the new instance: the arguments of the run -- the dialect and
    // the timeout of it -- are the runner's and not the runtime's. The
    // runner reads the request of the report, starts the script again, and
    // this instance ends.
    rt.ask_for_the_script_to_run_again();
    // The old instance ends: the new one is already on its way.
    rt.exit_with(0);
    Ok(Value::Text("".into()))
}

/// `Thread(Command, Value1?, Value2?)`: the state of the thread of the run.
///
/// Measured on the binary of v2.0.31: the commands are
/// "Priority", "Interrupt" and "NoTimers", and the word is read without
/// regard to its case. Any other word refuses, and the refusal carries
/// the name of the function and the word in the `Extra`.
///
/// The value of "Priority" is the priority of the thread, the two of
/// "Interrupt" are the milliseconds and the lines of the window of the
/// interruption, and the one of "NoTimers" says whether the timers may
/// run -- the double negative of the v2: "NoTimers" is zero when the
/// timers may run.
///
/// A value that is not there leaves the setting as it was.
pub(crate) fn thread_command(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let command = text_of(values, 0);
    let mut state = rt.the_thread_state();
    match command.to_ascii_lowercase().as_str() {
        "priority" => {
            if let Some(value) = values.get(1).and_then(|value| value.to_number()) {
                state.priority = value as i64;
            }
        }
        "interrupt" => {
            if let Some(value) = values.get(1).and_then(|value| value.to_number()) {
                state.uninterruptible_ms = value as i64;
            }
            if let Some(value) = values.get(2).and_then(|value| value.to_number()) {
                state.uninterruptible_lines = value as i64;
            }
        }
        "notimers" => {
            if let Some(value) = values.get(1).and_then(|value| value.to_number()) {
                // The double negative of the v2: the timers may run when
                // the number of "NoTimers" is zero.
                state.allow_timers = value == 0.0;
            }
        }
        _ => {
            return Err(RuntimeError {
                message: "Parameter #1 of Thread is invalid.".to_string(),
                line,
            });
        }
    }
    rt.set_the_thread_state(state);
    Ok(Value::Text("".into()))
}

/// `ToolTip(Text, X, Y)`: show a small window of text, and answer its number.
///
/// Measured on the binary of v2.0.19: showing one answers an
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
        return Ok(Value::Text("".into()));
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
/// Measured on the binary of v2.0.19: moving the window the script
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
    Ok(Value::Text("".into()))
}

/// `ControlSend(Keys, Control, WinTitle)`: send plain text into a control of the
/// script's own window.
///
/// Measured on the binary of v2.0.19: `ControlSend "abc", "Edit1"`
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
                        });
                    }
                }
            }
            SendPart::Down(name) | SendPart::Up(name) => {
                return Err(RuntimeError {
                    message: format!("holding the {name} key down has no text effect on a control"),
                    line,
                });
            }
        }
    }
    let mut sent = values.to_vec();
    if sent.is_empty() {
        sent.push(Value::Text(text.into()));
    } else {
        sent[0] = Value::Text(text.into());
    }
    control_send_text(rt, &sent, line, 0)
}

/// `RunAs(User, Password, Domain)`: set the user a later `Run` would use.
///
/// Measured on the binary of v2.0.19: it answers the empty text.
/// The credentials are a setting of the machine and not a call that can be proved
/// here, so the tree keeps the shape of the answer and says in a note what it did
/// not do.
pub(crate) fn run_as(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text("".into()))
}

/// `BlockInput(Mode)`: **refused, with a reason of its own.**
///
/// Measured on the binary of v2.0.19: `BlockInput("Off")` answers
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
        return Ok(Value::Text("".into()));
    }
    Err(RuntimeError {
        message: "blocking the input of the whole machine is a privilege this compositor does not give a client".to_string(),
        line,
    })
}

/// `OnClipboardChange(Func, AddRemove)`: register a function for a change of
/// the clipboard.
///
/// Measured on the binary of v2.0.19: a `Func` answers the empty
/// text, a String is refused, and the function is called with 1 when the script
/// itself writes `A_Clipboard`. This runtime has no watcher of ANOTHER program's
/// clipboard -- its clipboard is a read and a write -- but its own write is a
/// change it can see, so the function is called for that one.
pub(crate) fn on_clipboard_change(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let callback = rt.the_callback_with_one_argument(values.first(), "OnClipboardChange", line)?;
    // Measured on the binary of v2.0.19: a second argument of
    // zero removes the function, and one (or none) registers it. The name
    // answers the empty text either way. The function is called with 1 when
    // the script itself writes `A_Clipboard` -- the only change this runtime
    // can see, since it has no watcher of another program's clipboard.
    let remove = match values.get(1) {
        Some(value) => value.to_number().unwrap_or(1.0) <= 0.0,
        None => false,
    };
    rt.set_the_clipboard_watcher(if remove { None } else { Some(callback.name) });
    Ok(Value::Text("".into()))
}

/// `CallbackCreate(Func)`: answer a number that names the function.
///
/// Measured on the binary of v2.0.19: it answers an Integer, the
/// address of a native thunk. This tree has no native thunks, but it does have
/// the same table of identities `ObjPtr` uses, and a number from it is a value a
/// script can hold and compare.
pub(crate) fn callback_create(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let func = match values.first() {
        Some(value) => value.clone(),
        None => {
            return Err(RuntimeError {
                message: "Too few parameters passed to function.".to_string(),
                line,
            });
        }
    };
    // The number of parameters the caller passes; the window procedures
    // of the machine pass four (hwnd, msg, wParam, lParam).
    let param_count = match values.get(2) {
        Some(Value::Int(n)) if *n >= 0 => *n as usize,
        _ => 4,
    };
    let address = rt
        .desktop()
        .create_callback(&func, param_count)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(address as i64))
}

/// `Shutdown(Flag)`: **refused, with a reason of its own.**
///
/// Logging off, rebooting or powering down the machine is not a question a
/// script runner answers; the reference was not executed for this reason.
/// `Shutdown`: end the session of the machine.
///
/// Measured on the binary of v2.0.19: a whole number -- even
/// one that names no real operation, like 99 -- answers the empty text, and
/// a text where a number is wanted throws "Parameter #1 of Shutdown requires
/// a Number, but received a String.". The number is answered the same way
/// here, and the session of the machine is deliberately left alone: the
/// runner does not end the session it runs in, because a script that asks
/// for it would take the whole desktop down with it. The shape of the call
/// is the contract, not the side effect.
pub(crate) fn shutdown(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    number_argument(values, 0, line, "Shutdown")?;
    Ok(Value::Text("".into()))
}

/// `FileGetVersion`: the version of a program of Windows.
///
/// Measured on the binary of v2.0.19: the shape is
/// `FileGetVersion FileName`; a program of Windows answers the version as
/// four numbers and a point between them -- "10.0.26100.9818" for the
/// notepad of that day -- a file that is not there throws "(2) The system
/// cannot find the file specified.", and a file that has no section of
/// resources throws "(1812) The specified image file did not contain a
/// resource section.".
///
/// The file is read as a program of Windows wherever it is: a program of
/// Windows may be read on any platform, and the version of it is a
/// resource of the file and not a thing of the machine that runs it.
pub(crate) fn file_get_version(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(path) = values.first() else {
        return Err(RuntimeError {
            message: "Parameter #1 of FileGetVersion must not be omitted in this case.".to_string(),
            line,
        });
    };
    let path = path.to_text();
    let bytes = rt
        .desktop()
        .read_the_bytes_of_a_file(&path)
        .map_err(|reason| desktop_failure(reason, line))?;
    match crate::version::the_version_of(&bytes) {
        Some(version) => Ok(Value::Text(version.into())),
        // The file is there and it is not a program that carries a version.
        None => Err(RuntimeError {
            message: "(1812) The specified image file did not contain a resource section."
                .to_string(),
            line,
        }),
    }
}

/// `ControlGetPos(&X, &Y, &W, &H, Control, WinTitle)`: the place and size of a
/// control of the script's own window, relative to its client area.
///
/// Measured on the binary of v2.0.19: with four VarRef outputs it
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
    Ok(Value::Text("".into()))
}

/// `OnMessage`: register a handler for the messages of a number.
///
/// Measured on the binary of v2.0.19: the shape is
/// `OnMessage MsgNumber, Callback, MaxThreads` -- the callback is **required**
/// and a call with one parameter is refused at the load -- and the call
/// answers the empty text. A message number that is not a number throws
/// "Parameter #1 of OnMessage requires a Number, but received a String.".
///
/// The messages are the ones of a window procedure, and a script is not
/// one: the handler of the reference fires for the messages of the windows
/// of other programs. This runtime keeps the registry of the handlers, and
/// the drain of the events answers a `message:<number>` event of the helper
/// with the handler that is registered for it.
pub(crate) fn on_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "OnMessage")? as i64;
    let Some(callback) = values.get(1) else {
        return Err(RuntimeError {
            message: "Parameter #2 of OnMessage must not be omitted in this case.".to_string(),
            line,
        });
    };
    if !matches!(callback, Value::Object(_)) {
        return Err(RuntimeError {
            message: crate::runtime::requires(2, "OnMessage", TypeName::Object, callback),
            line,
        });
    }
    // `MaxThreads` of zero takes the handler away: it is the way the
    // reference unregisters a monitor, and the call answers the empty text.
    if values.get(2).and_then(|value| value.to_number()) == Some(0.0) {
        rt.forget_a_message_handler(number);
        return Ok(Value::Text("".into()));
    }
    rt.remember_a_message_handler(number, callback.clone());
    // A message monitor keeps the script alive after its statements end,
    // the way the original AutoHotkey does.
    rt.the_script_asked_to_stay(true);
    // The window of the script watches the message: when it arrives,
    // the desktop queues it and the runtime calls the handler between
    // the statements. A desktop that cannot watch says so, and the
    // handler is kept anyway.
    let _ = rt.desktop().watch_a_message(number);
    Ok(Value::Text("".into()))
}

/// `PostMessage` and `SendMessage`: send a message to the window of a number.
///
/// Measured on the binary of v2.0.19: the first parameter is
/// the **number** of a window -- a text throws "Parameter #1 of PostMessage
/// requires a Number, but received a String." -- and every number throws
/// "Target window not found.", the window of the script included: the
/// messages go to the windows of other programs, and this runtime has no
/// window procedure to receive them.
pub(crate) fn post_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let message = number_argument(values, 0, line, "PostMessage")? as i64;
    let wparam = values.get(1).and_then(|v| v.to_number()).unwrap_or(0.0) as i64;
    let lparam = values.get(2).and_then(|v| v.to_number()).unwrap_or(0.0) as i64;
    let window = the_hwnd_of_the_message_target(rt, values.get(3), line)?;
    rt.desktop()
        .post_a_message(window, message, wparam, lparam)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Int(1))
}

/// The window a `PostMessage` or a `SendMessage` goes to.
///
/// The target is the `ahk_id` of a window, or its title; the empty title
/// is the window of the script. The number of the window is what the
/// message is sent to.
fn the_hwnd_of_the_message_target(
    rt: &mut Runtime<'_>,
    title: Option<&Value>,
    line: u32,
) -> Result<i64, RuntimeError> {
    // A number is the handle of the window itself: `PostMessage` to the
    // handle of a control sends the message to that control, and not to
    // a window whose title is the number.
    match title {
        Some(Value::Int(number)) => return Ok(*number),
        Some(Value::Float(number)) => return Ok(*number as i64),
        _ => {}
    }
    let text = title.map(|v| v.to_text()).unwrap_or_default();
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("ahk_id ") {
        let rest = rest.trim();
        // The number of a window is written in decimal or in hex: the
        // `WinExist` answers the hex, with the `0x` before it.
        let number = if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
            i64::from_str_radix(hex, 16).ok()
        } else {
            rest.parse::<i64>().ok()
        };
        if let Some(number) = number {
            return Ok(number);
        }
    }
    if trimmed.is_empty() {
        return rt.desktop().script_window().map_err(|reason| RuntimeError {
            message: reason,
            line,
        });
    }
    // A title: the first window that answers it.
    let windows = rt.desktop().windows().map_err(|reason| RuntimeError {
        message: reason,
        line,
    })?;
    let mode = crate::settings::title_match_mode();
    for window in windows {
        if mode.matches(&window.title, trimmed).unwrap_or(false) {
            return Ok(window.id);
        }
    }
    Err(RuntimeError {
        message: "Target window not found.".to_string(),
        line,
    })
}

/// `SendMessage`: the same as `PostMessage`, and the answer is the same.
pub(crate) fn send_message(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let message = number_argument(values, 0, line, "SendMessage")? as i64;
    let wparam = values.get(1).and_then(|v| v.to_number()).unwrap_or(0.0) as i64;
    let lparam = values.get(2).and_then(|v| v.to_number()).unwrap_or(0.0) as i64;
    let window = the_hwnd_of_the_message_target(rt, values.get(3), line)?;
    let answer = rt
        .desktop()
        .send_a_message(window, message, wparam, lparam)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Int(answer))
}

/// `WinSetRegion`: the shape of a window, cut out of it.
///
/// Measured on the binary of v2.0.19: every target throws
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

/// `GroupAdd(GroupName, WinTitle, WinText, ExcludeTitle, ExcludeText)`: put a
/// window title into a named set that outlives the statement.
///
/// Measured on the binary of v2.0.19: it answers the empty text.
///
/// O texto da janela e as duas exclusoes entram no alvo, e a janela que
/// o alvo nomeia e a que entra no grupo: medido no binario do v2.0.31 em
/// O `GroupAdd("me", "t", "o texto")` acha a janela pelo
/// texto de um elemento de texto dela.
pub(crate) fn group_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let group = text_of(values, 0);
    if group.is_empty() {
        return Err(RuntimeError {
            message: "the name of a window group may not be empty.".to_string(),
            line,
        });
    }
    let wanted = crate::desktop::TheWindowWanted::of(values, 1);
    // A janela que o alvo nomeia entra no grupo, e o que o grupo guarda
    // e o titulo dela -- que e o que o `GroupActivate` procura.
    let title = if wanted.asked && !wanted.title.is_empty() {
        the_title_of_the_window(rt, &wanted, line)?
    } else {
        wanted.title.clone()
    };
    rt.add_to_group(&group, &title);
    Ok(Value::Text("".into()))
}

/// `GroupActivate(GroupName, Mode)`: answer the id of the first window of the
/// group that is on the screen, or zero when none is.
///
/// Measured on the binary of v2.0.19: the answer is a whole
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
            return Ok(Value::Text(window.id.clone().into()));
        }
    }
    Ok(Value::Int(0))
}

/// `GroupDeactivate` and `GroupClose`: take the group away.
///
/// Measured on the binary of v2.0.19: both answer the empty text.
pub(crate) fn group_close(
    rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let group = text_of(values, 0);
    rt.close_a_group(&group);
    Ok(Value::Text("".into()))
}

/// `MenuSelect`: choose an item of the menu of a window.
///
/// Measured on the binary of v2.0.19, each shape in its own
/// try: a window that is not there is a throw of "Target window not found.";
/// the menu of a window that IS there -- the script's own, by position or by
/// path, an item that does not exist, or a window without a menu at all --
/// is a throw of "Non-existent or unsupported menu." (the reference avoids
/// driving a menu from its own thread and fails the same way for every
/// shape). This desktop cannot reach the menus of a foreign window either,
/// so the second wording is the answer for every window that is found.
pub(crate) fn menu_select(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The shape here is (WinTitle, Menu, SubMenu1, ...): the parameters after
    // the first are the path of the item and not the exclusions of a WinGet,
    // so the title alone is read from the first of them.
    let title = text_of(values, 0);
    let windows = rt
        .desktop()
        .matching_windows(&title, crate::settings::title_match_mode())
        .map_err(|e| desktop_failure(e, line))?;
    if windows.is_empty() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        });
    }
    Err(RuntimeError {
        message: "Non-existent or unsupported menu.".to_string(),
        line,
    })
}

/// `GuiFromHwnd` and `GuiCtrlFromHwnd`: the object that a handle names.
///
/// Measured on the binary of v2.0.19: `GuiFromHwnd(g.Hwnd)`
/// gives the SAME Gui object back, `GuiCtrlFromHwnd(e.Hwnd)` gives the same
/// control object back, and a handle that no window owns answers the EMPTY
/// TEXT -- not a failure. On this platform a handle is the number this
/// runtime mints for the object (the same table `ObjPtr` reads), so the
/// inverse is the same table read backwards.
fn the_gui_object_of_a_handle(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    name: &str,
) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, name)? as i64;
    match rt.the_object_of(number) {
        Some(object) => Ok(object),
        None => Ok(Value::Text("".into())),
    }
}

/// `GuiFromHwnd`: the Gui object that a handle names.
pub(crate) fn gui_from_hwnd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let object = the_gui_object_of_a_handle(rt, values, line, "GuiFromHwnd")?;
    let is_gui = match &object {
        Value::Object(o) => o.borrow().class.eq_ignore_ascii_case("gui"),
        _ => false,
    };
    if is_gui {
        Ok(object)
    } else {
        Ok(Value::Text("".into()))
    }
}

/// `GuiCtrlFromHwnd`: the control object that a handle names.
///
/// A control's handle lives in two tables on this platform: the identity
/// table that `Hwnd` mints through, and the number the helper gave the
/// control when it was added -- the one the `Control*` family reads. The
/// inverse walks both, and answers the empty text when neither names a
/// control.
pub(crate) fn gui_ctrl_from_hwnd(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = number_argument(values, 0, line, "GuiCtrlFromHwnd")? as i64;
    // First the identity table: the number `Hwnd` mints today.
    if let Some(object) = rt.the_object_of(number) {
        let is_control = match &object {
            Value::Object(o) => o.borrow().class.to_lowercase().starts_with("gui."),
            _ => false,
        };
        if is_control {
            return Ok(object);
        }
    }
    // Then the numbers the helper handed out when the controls were added.
    for gui in rt.the_guis() {
        let Value::Object(window) = gui else {
            continue;
        };
        for (key, property) in window.borrow().properties.iter() {
            if !key.starts_with("control:") {
                continue;
            }
            let Value::Object(control) = &property.value else {
                continue;
            };
            let stored = control
                .borrow()
                .value_of("hwnd")
                .map(|value| value.to_text().parse::<i64>().unwrap_or(-1));
            if stored == Some(number) {
                return Ok(property.value.clone());
            }
        }
    }
    Ok(Value::Text("".into()))
}

/// `MenuFromHandle`: **refused, with a reason of its own.**
///
/// The other two names take a number this runtime mints, and this one takes
/// a Win32 HMENU: this runtime has no handle of a menu, because the menus
/// are objects the script holds and the helper draws.
pub(crate) fn menu_from_handle(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message: "a menu handle is a Win32 HMENU, and this runtime has no handle of a menu: the menus are objects the script holds and the helper draws".to_string(),
        line,
    })
}

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

/// `ControlGetItems(Control, WinTitle)`: the items of a list-shaped control.
///
/// Measured on the binary of v2.0.19: it answers an Array of the
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
            .map(|item| Value::Text(item.to_string().into()))
            .collect()
    };
    match crate::classes::construct("array", &answers, rt, line) {
        Some(result) => result.map_err(|failure| crate::runtime::as_error(failure, line)),
        None => Ok(Value::List(answers)),
    }
}

/// `ControlAddItem(Text, Control, WinTitle)`: append an item.
///
/// Measured on the binary of v2.0.31: the answer is the
/// index of the new entry -- the number of the entry it made, counted
/// from one.
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
    // The index of the new entry is the count of the entries of the
    // control, and the items are one per line.
    let items = rt
        .desktop()
        .gui_combo_items(number)
        .map_err(|e| desktop_failure(e, line))?;
    Ok(Value::Int(items.lines().count() as i64))
}

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
    Ok(Value::Text("".into()))
}

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
    Ok(Value::Text("".into()))
}

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
    Ok(Value::Text(text.into()))
}

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
    Ok(Value::Text("".into()))
}

/// `ControlFindItem(Text, Control, WinTitle)`: the 1-based index of the first
/// item that is this text.
///
/// Measured on the binary of v2.0.31: the answer is the
/// index of the entry, and a text that is not there fails with "Failed".
pub(crate) fn control_find_item(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_combo(rt, values, 1, line)?;
    let text = text_of(values, 0);
    let index = rt
        .desktop()
        .gui_combo_find(number, &text)
        .map_err(|e| desktop_failure(e, line))?;
    if index == 0 {
        // The entry is not there, and the reference fails with "Failed".
        return Err(RuntimeError {
            message: "Failed".to_string(),
            line,
        });
    }
    Ok(Value::Int(index))
}

/// `ControlShowDropDown` and `ControlHideDropDown`: open or close the pop-up of
/// a list-shaped control.
///
/// Measured on the binary of v2.0.19: both answer the empty text.
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
    Ok(Value::Text("".into()))
}

pub(crate) fn control_show_drop_down(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_drop_down_of_a_combo(rt, values, line, true)
}

pub(crate) fn control_hide_drop_down(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    the_drop_down_of_a_combo(rt, values, line, false)
}

/// `StatusBarGetText(Part, WinTitle)`: the text of the status bar.
///
/// Measured on the binary of v2.0.19: with part 1 and the title
/// of a window holding a StatusBar, it answers the text the bar was made with.
/// This tree carries one part. A window that is not the script's own is not
/// asked of the accessibility layer, which has no status bar of its own.
pub(crate) fn status_bar_get_text(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let title = text_of(values, 1);
    the_window_of_the_script(rt, &title, line)?;
    rt.desktop()
        .gui_status_text()
        .map(Value::text)
        .map_err(|e| desktop_failure(e, line))
}

/// `StatusBarWait(Text, Seconds, Part, WinTitle)`: wait for the status bar to
/// hold a text.
///
/// Measured on the binary of v2.0.19: the match is a
/// case-sensitive SUBSTRING of part 1 ("status" matches "the status"), the
/// answer is 1 on a match and 0 when the seconds run out, and a part other
/// than 1 is refused with "Parameter #3 of StatusBarWait is invalid.".
/// The status text of this runtime cannot change while a script runs, so the
/// answer is known at once -- but the wait is honoured in shape, polling until
/// the timeout like `ClipWait` does.
pub(crate) fn status_bar_wait(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let wanted = text_of(values, 0);
    let seconds = number_of(values, 1);
    let part = number_of(values, 2);
    let title = text_of(values, 3);
    if part != 1.0 && values.len() > 2 {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(3, "StatusBarWait"),
            line,
        });
    }
    the_window_of_the_script(rt, &title, line)?;
    let limit = if values.len() > 1 && seconds > 0.0 {
        Some(seconds * 1000.0)
    } else if values.len() > 1 {
        Some(0.0)
    } else {
        None
    };
    let started = rt.now_ms();
    loop {
        let text = rt
            .desktop()
            .gui_status_text()
            .map_err(|e| desktop_failure(e, line))?;
        if text.contains(&wanted) {
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

/// `ListViewGetContent(Options, Control, WinTitle)`: the rows of a list view,
/// cells by tabs and rows by line breaks.
///
/// Measured on the binary of v2.0.19: with no options it answers
/// "alpha\t10\nbeta\t20", and the "Selected" option answers only the rows that
/// are chosen. The control is named by its `ClassNN` (`SysListView321`).
pub(crate) fn list_view_get_content(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let options = text_of(values, 0);
    let classnn = text_of(values, 1);
    let title = text_of(values, 2);
    the_window_of_the_script(rt, &title, line)?;
    if !rt.desktop().gui_is_open() {
        return Err(RuntimeError {
            message: TARGET_WINDOW.to_string(),
            line,
        });
    }
    let number = rt
        .desktop()
        .gui_number_named(&classnn)
        .map_err(|reason| desktop_failure(reason, line))?;
    let selected = options.eq_ignore_ascii_case("selected");
    rt.desktop()
        .gui_list_content(number as u64, selected)
        .map(Value::text)
        .map_err(|e| desktop_failure(e, line))
}
