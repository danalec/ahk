//! The class types of AutoHotkey v2.
//!
//! `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu` and `Hotkey`
//! are the classes the runtime carries. Each of them lives in its own
//! module, and this one is the registry that knows which is which.
//!
//! A class type of the runtime is not a class of the script: the script
//! cannot extend it, and its methods are the ones the runtime answers
//! for. The state of the object lives in the property table of the
//! object, and only the class module touches it.
//!
//! The ones that are shaped for Windows -- `Gui` and its parts, and
//! `Hotkey` -- are carried and answer with the reason they cannot work
//! here. The others work anywhere.
//!
//! The eleven tables of the binary are Array, Buffer, File, Func, Gui,
//! Gui.Control, Gui.ListView, Gui.Menu, Gui.StatusBar, Hotkey and Map.
//! The methods of the menu are in the table of `Gui.Menu`, and the
//! script calls the class `Menu`, so the registry answers for both.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::runtime::{Detail, Object, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, put};

pub(crate) mod array;
pub(crate) mod buffer;
pub(crate) mod clipboard;
pub(crate) mod enumerator;
pub(crate) mod error;
pub(crate) mod file;
pub(crate) mod func;
pub(crate) mod gui;
pub(crate) mod hotkey;
pub(crate) mod inputhook;
pub(crate) mod map;
pub(crate) mod menu;

/// The methods of the class types, with the name v2 spells them and
/// the number of the arguments they take.
///
/// The names are the ones of the binary of v2.0.19 and the numbers were
/// measured on it on 2026-10-29. The first argument of every one of them
/// is the object itself: the reference is the method, and the object is
/// one of the arguments of the call.
const METHODS: [(&str, &str, &str, i64, i64); 54] = [
    ("map", "set", "Set", 1, 1),
    ("map", "get", "Get", 2, 3),
    ("map", "has", "Has", 2, 2),
    ("map", "delete", "Delete", 2, 2),
    ("map", "clear", "Clear", 1, 1),
    ("map", "clone", "Clone", 1, 1),
    ("map", "__new", "__New", 1, 1),
    ("map", "__enum", "__Enum", 1, 2),
    ("array", "push", "Push", 1, 1),
    ("array", "pop", "Pop", 1, 1),
    ("array", "insertat", "InsertAt", 2, 2),
    ("array", "removeat", "RemoveAt", 2, 3),
    ("array", "delete", "Delete", 2, 2),
    ("array", "get", "Get", 2, 3),
    ("array", "has", "Has", 2, 2),
    ("array", "clone", "Clone", 1, 1),
    ("array", "__new", "__New", 1, 1),
    ("array", "__enum", "__Enum", 1, 2),
    ("buffer", "__new", "__New", 1, 3),
    ("func", "call", "Call", 1, 1),
    ("func", "isoptional", "IsOptional", 1, 1),
    ("func", "isbyref", "IsByRef", 1, 1),
    ("func", "bind", "Bind", 1, 1),
    // The enumerator is a function of its own kind, and the `Call` of it
    // is the turn of the enumeration. Measured on the binary of v2.0.19 on
    // 2026-10-31: the call takes no arguments at all, and two when it
    // hands out a key and a value.
    ("enumerator", "call", "Call", 1, 3),
    ("enumerator", "isoptional", "IsOptional", 1, 1),
    ("enumerator", "isbyref", "IsByRef", 1, 1),
    ("enumerator", "bind", "Bind", 1, 1),
    ("error", "__new", "__New", 1, 4),
    ("file", "read", "Read", 1, 2),
    ("file", "readline", "ReadLine", 1, 1),
    ("file", "readchar", "ReadChar", 1, 1),
    ("file", "readuchar", "ReadUChar", 1, 1),
    ("file", "readshort", "ReadShort", 1, 1),
    ("file", "readushort", "ReadUShort", 1, 1),
    ("file", "readint", "ReadInt", 1, 1),
    ("file", "readuint", "ReadUInt", 1, 1),
    ("file", "readint64", "ReadInt64", 1, 1),
    ("file", "readfloat", "ReadFloat", 1, 1),
    ("file", "readdouble", "ReadDouble", 1, 1),
    ("file", "write", "Write", 2, 2),
    ("file", "writeline", "WriteLine", 1, 2),
    ("file", "writechar", "WriteChar", 2, 2),
    ("file", "writeuchar", "WriteUChar", 2, 2),
    ("file", "writeshort", "WriteShort", 2, 2),
    ("file", "writeushort", "WriteUShort", 2, 2),
    ("file", "writeint", "WriteInt", 2, 2),
    ("file", "writeuint", "WriteUInt", 2, 2),
    ("file", "writeint64", "WriteInt64", 2, 2),
    ("file", "writefloat", "WriteFloat", 2, 2),
    ("file", "writedouble", "WriteDouble", 2, 2),
    ("file", "rawread", "RawRead", 2, 3),
    ("file", "rawwrite", "RawWrite", 2, 3),
    ("file", "seek", "Seek", 2, 3),
    ("file", "close", "Close", 1, 1),
];

/// The number of the arguments of the calls of the members.
///
/// Measured on the binary of v2.0.19 on 2026-10-30. The numbers are the
/// ones of the call the script writes: `m.Get("a")` takes one argument,
/// and `m.Get` as a value takes two, because the object is one of the
/// arguments of that one. A number of `-1` means that the call takes any
/// number of arguments.
///
/// The methods of the base object are the ones every object has, and the
/// class of them is `object`.
const CALLS: [(&str, &str, i64, i64); 60] = [
    ("object", "ownprops", 0, 0),
    ("object", "clone", 0, 0),
    ("object", "hasownprop", 1, 1),
    ("object", "defineprop", 2, 2),
    ("object", "deleteprop", 1, 1),
    ("object", "getownpropdesc", 1, 1),
    ("object", "hasprop", 1, 1),
    ("object", "hasmethod", 0, 2),
    ("object", "getmethod", 0, 2),
    ("object", "hasbase", 1, 1),
    ("map", "set", 0, -1),
    ("map", "get", 1, 2),
    ("map", "has", 1, 1),
    ("map", "delete", 1, 1),
    ("map", "clear", 0, 0),
    ("map", "clone", 0, 0),
    ("map", "__new", 0, -1),
    ("map", "__enum", 0, 1),
    ("array", "push", 0, -1),
    ("array", "pop", 0, 0),
    ("array", "insertat", 1, -1),
    ("array", "removeat", 1, 2),
    ("array", "delete", 1, 1),
    ("array", "get", 1, 2),
    ("array", "has", 1, 1),
    ("array", "clone", 0, 0),
    ("array", "__new", 0, -1),
    ("array", "__enum", 0, 1),
    ("buffer", "__new", 0, 2),
    ("func", "isoptional", 0, -1),
    ("func", "isbyref", 0, -1),
    ("func", "bind", 0, -1),
    // The call of an enumerator takes no arguments at all, and two when
    // it hands out a key and a value. Measured on the binary of v2.0.19
    // on 2026-10-31.
    ("enumerator", "call", 0, 2),
    ("error", "__new", 0, 3),
    ("file", "read", 0, 1),
    ("file", "readline", 0, 0),
    ("file", "readchar", 0, 0),
    ("file", "readuchar", 0, 0),
    ("file", "readshort", 0, 0),
    ("file", "readushort", 0, 0),
    ("file", "readint", 0, 0),
    ("file", "readuint", 0, 0),
    ("file", "readint64", 0, 0),
    ("file", "readfloat", 0, 0),
    ("file", "readdouble", 0, 0),
    ("file", "write", 1, 1),
    ("file", "writeline", 0, 1),
    ("file", "writechar", 1, 1),
    ("file", "writeuchar", 1, 1),
    ("file", "writeshort", 1, 1),
    ("file", "writeushort", 1, 1),
    ("file", "writeint", 1, 1),
    ("file", "writeuint", 1, 1),
    ("file", "writeint64", 1, 1),
    ("file", "writefloat", 1, 1),
    ("file", "writedouble", 1, 1),
    ("file", "rawread", 1, 2),
    ("file", "rawwrite", 1, 2),
    ("file", "seek", 1, 2),
    ("file", "close", 0, 0),
];

/// The class types that v2 makes with a call of their own.
///
/// The arity is the number of the arguments that the constructor takes,
/// and the two names are the ones that v2 puts in the `What` and in the
/// `Extra` of the failure when the call has the wrong number of
/// arguments.
///
/// Measured on the binary of v2.0.19 on 2026-10-31. A number of `-1`
/// means that the constructor takes any number of arguments: the `Array`
/// is made out of the values it is given. The classes that go through the
/// base object -- the `Class`, the `Object` and the classes of the
/// failures -- carry the name of the base in the `What`, and the `Menu`
/// carries its own.
const CONSTRUCTORS: [(&str, i64, i64, &str, &str); 11] = [
    ("array", 0, -1, "Array.Call", "Array.Prototype.__New"),
    ("buffer", 0, 2, "Buffer.Call", "Buffer.Prototype.__New"),
    ("class", 0, 0, "Object.Call", ""),
    (
        "clipboardall",
        0,
        2,
        "ClipboardAll.Call",
        "ClipboardAll.Prototype.__New",
    ),
    ("comobject", 1, 2, "", "ComObject.Call"),
    ("comvalue", 2, 3, "", "ComValue.Call"),
    ("error", 0, 3, "Object.Call", "Error.Prototype.__New"),
    ("gui", 0, 3, "Gui.Call", "Gui.Prototype.__New"),
    (
        "inputhook",
        0,
        3,
        "InputHook.Call",
        "InputHook.Prototype.__New",
    ),
    ("menu", 0, 0, "Menu.Call", ""),
    ("object", 0, 0, "Object.Call", ""),
];

/// The class types that v2 does not make.
///
/// A function is written in the script or it is one of the built-ins,
/// and a file is made by the `FileOpen` built-in. Neither of the two is
/// a class that can be made, and v2 answers "Invalid base." to both. The
/// enumerator is made out of the `__Enum` of a value, and not by a call.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the `BoundFunc`,
/// the `Closure` and the `RegExMatchInfo` are classes of the dialect that
/// the runtime makes, and not a script: the call of them is refused with
/// "Invalid base." as well.
const NOT_MADE: [&str; 6] = [
    "boundfunc",
    "closure",
    "file",
    "func",
    "enumerator",
    "regexmatchinfo",
];

/// The name v2 gives to the call of the base object.
///
/// The classes of the runtime that v2 does not make are called through
/// the base object, and the base of them has no such thing.
const THE_CALL_OF_THE_BASE: &str = "Object.Call";

/// The names of the `ClipboardAll` class.
///
/// The class is the buffer of the clipboard, and the binary keeps the
/// names of it outside its tables of the methods.
const THE_CLIPBOARD_ALL: [&str; 2] = ["ptr", "size"];

/// The names of the `ComValue` class.
///
/// The COM objects are a thing of Windows, and this runtime does not
/// carry them: the names of the class are here, because they are the
/// surface of v2.
const THE_COM_VALUE: [&str; 1] = ["ptr"];

/// The names of the base object.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the table of the
/// base object is the one that follows the table of the `Map`, and it
/// holds six names. The `Base` is a property of every object, and the
/// binary does not keep it in the table of the methods.
const THE_BASE_OBJECT: [&str; 7] = [
    "base",
    "clone",
    "defineprop",
    "deleteprop",
    "getownpropdesc",
    "hasownprop",
    "ownprops",
];

/// The names of the base object that the binary keeps outside its table.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: every object has these
/// four methods, and the table of the base object does not hold them.
const THE_BASE_OBJECT_OUTSIDE: [&str; 4] = ["getmethod", "hasbase", "hasmethod", "hasprop"];

/// The members of a class value.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the class of a class
/// value is "Class", and it has the base, the prototype, the call of the
/// class and the initialiser of it.
const THE_CLASS_VALUE: [&str; 4] = ["base", "call", "prototype", "__init"];

/// The names of the methods of the base object, the way v2 spells them.
///
/// The binary spells five of them with the class in front, and the four
/// that the script asks about every day plainly.
const BASE: [(&str, &str); 10] = [
    ("ownprops", "Object.Prototype.OwnProps"),
    ("hasownprop", "Object.Prototype.HasOwnProp"),
    ("defineprop", "Object.Prototype.DefineProp"),
    ("deleteprop", "Object.Prototype.DeleteProp"),
    ("getownpropdesc", "Object.Prototype.GetOwnPropDesc"),
    ("clone", "Object.Prototype.Clone"),
    ("hasprop", "HasProp"),
    ("hasmethod", "HasMethod"),
    ("getmethod", "GetMethod"),
    ("hasbase", "HasBase"),
];

/// The names of the properties of the class types, the way v2 spells
/// them.
const PROPERTIES: [(&str, &str, &str); 28] = [
    ("map", "count", "Count"),
    ("map", "capacity", "Capacity"),
    ("map", "casesense", "CaseSense"),
    ("array", "length", "Length"),
    ("array", "capacity", "Capacity"),
    ("buffer", "size", "Size"),
    ("buffer", "ptr", "Ptr"),
    ("func", "name", "Name"),
    ("func", "minparams", "MinParams"),
    ("func", "maxparams", "MaxParams"),
    ("func", "isbuiltin", "IsBuiltin"),
    ("func", "isvariadic", "IsVariadic"),
    // The enumerator is a function of its own kind, and it carries the
    // values of a function.
    ("enumerator", "name", "Name"),
    ("enumerator", "minparams", "MinParams"),
    ("enumerator", "maxparams", "MaxParams"),
    ("enumerator", "isbuiltin", "IsBuiltin"),
    ("enumerator", "isvariadic", "IsVariadic"),
    ("file", "pos", "Pos"),
    ("file", "length", "Length"),
    ("file", "ateof", "AtEOF"),
    ("file", "encoding", "Encoding"),
    ("file", "handle", "Handle"),
    ("error", "message", "Message"),
    ("error", "what", "What"),
    ("error", "extra", "Extra"),
    ("error", "file", "File"),
    ("error", "line", "Line"),
    ("error", "stack", "Stack"),
];

/// The properties of the class types that may not be written.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the write of one of them
/// is refused with "Property is read-only.", the name of the property goes in
/// the `Extra` of the error, and the `What` is empty. The other properties
/// are written the way the class says: the capacity and the case of a map,
/// the length and the capacity of an array, the size of a buffer, the
/// properties of an error, and the place, the length and the encoding of a
/// file.
const READ_ONLY: [(&str, &[&str]); 5] = [
    ("map", &["count"]),
    ("buffer", &["ptr"]),
    (
        "func",
        &["name", "minparams", "maxparams", "isbuiltin", "isvariadic"],
    ),
    (
        "enumerator",
        &["name", "minparams", "maxparams", "isbuiltin", "isvariadic"],
    ),
    ("file", &["ateof", "handle"]),
];

/// Whether the property of a class type may not be written.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the write of it is
/// refused with "Property is read-only.".
pub(crate) fn the_property_is_read_only(class: &str, name: &str) -> bool {
    let class = class.to_lowercase();
    let name = name.to_lowercase();
    READ_ONLY
        .iter()
        .any(|(owner, names)| *owner == class && names.iter().any(|property| *property == name))
}

/// The members that the prototype of a class owns.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a class
/// owns the class of the value and the members that the class declares -- the
/// methods of it and the properties of it. The names are in the order of
/// v2, and the case of them does not count.
const PROTOTYPE_MEMBERS: [(&str, &[&str]); 16] = [
    (
        "object",
        &[
            "Clone",
            "DefineProp",
            "DeleteProp",
            "GetOwnPropDesc",
            "HasOwnProp",
            "OwnProps",
        ],
    ),
    (
        "map",
        &[
            "__Enum",
            "__Item",
            "__New",
            "Capacity",
            "CaseSense",
            "Clear",
            "Clone",
            "Count",
            "Delete",
            "Get",
            "Has",
            "Set",
        ],
    ),
    (
        "array",
        &[
            "__Enum", "__Item", "__New", "Capacity", "Clone", "Delete", "Get", "Has", "InsertAt",
            "Length", "Pop", "Push", "RemoveAt",
        ],
    ),
    ("buffer", &["__New", "Ptr", "Size"]),
    ("error", &["__New"]),
    (
        "file",
        &[
            "AtEOF",
            "Close",
            "Encoding",
            "Handle",
            "Length",
            "Pos",
            "RawRead",
            "RawWrite",
            "Read",
            "ReadChar",
            "ReadDouble",
            "ReadFloat",
            "ReadInt",
            "ReadInt64",
            "ReadLine",
            "ReadShort",
            "ReadUChar",
            "ReadUInt",
            "ReadUShort",
            "Seek",
            "Write",
            "WriteChar",
            "WriteDouble",
            "WriteFloat",
            "WriteInt",
            "WriteInt64",
            "WriteLine",
            "WriteShort",
            "WriteUChar",
            "WriteUInt",
            "WriteUShort",
        ],
    ),
    (
        "func",
        &[
            "Bind",
            "Call",
            "IsBuiltIn",
            "IsByRef",
            "IsOptional",
            "IsVariadic",
            "MaxParams",
            "MinParams",
            "Name",
        ],
    ),
    ("enumerator", &[]),
    (
        "menu",
        &[
            "Add",
            "AddStandard",
            "Check",
            "ClickCount",
            "Default",
            "Delete",
            "Disable",
            "Enable",
            "Handle",
            "Insert",
            "Rename",
            "SetColor",
            "SetIcon",
            "Show",
            "ToggleCheck",
            "ToggleEnable",
            "Uncheck",
        ],
    ),
    (
        "gui",
        &[
            "__Enum",
            "__Item",
            "__New",
            "Add",
            "AddActiveX",
            "AddButton",
            "AddCheckBox",
            "AddComboBox",
            "AddCustom",
            "AddDateTime",
            "AddDDL",
            "AddDropDownList",
            "AddEdit",
            "AddGroupBox",
            "AddHotkey",
            "AddLink",
            "AddListBox",
            "AddListView",
            "AddMonthCal",
            "AddPic",
            "AddPicture",
            "AddProgress",
            "AddRadio",
            "AddSlider",
            "AddStatusBar",
            "AddTab",
            "AddTab2",
            "AddTab3",
            "AddText",
            "AddTreeView",
            "AddUpDown",
            "BackColor",
            "Destroy",
            "Flash",
            "FocusedCtrl",
            "GetClientPos",
            "GetPos",
            "Hide",
            "Hwnd",
            "MarginX",
            "MarginY",
            "Maximize",
            "MenuBar",
            "Minimize",
            "Move",
            "Name",
            "OnEvent",
            "Opt",
            "Restore",
            "SetFont",
            "Show",
            "Submit",
            "Title",
        ],
    ),
    ("clipboardall", &["__New"]),
    (
        "inputhook",
        &[
            "__New",
            "BackspaceIsUndo",
            "CaseSensitive",
            "EndKey",
            "EndMods",
            "EndReason",
            "FindAnywhere",
            "InProgress",
            "Input",
            "KeyOpt",
            "Match",
            "MinSendLevel",
            "NotifyNonText",
            "OnChar",
            "OnEnd",
            "OnKeyDown",
            "OnKeyUp",
            "Start",
            "Stop",
            "Timeout",
            "VisibleNonText",
            "VisibleText",
            "Wait",
        ],
    ),
    (
        "regexmatchinfo",
        &[
            "__Enum", "__Get", "__Item", "Count", "Len", "Mark", "Name", "Pos",
        ],
    ),
    ("boundfunc", &[]),
    ("closure", &[]),
    ("class", &[]),
];

/// The members of the prototype of a class that are methods.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a class
/// answers `HasMethod` with the names of the methods of it. The `__Item` of a
/// class is not one of them, and the `__Class` of the value is not one
/// either.
const PROTOTYPE_METHODS: [(&str, &[&str]); 16] = [
    (
        "object",
        &[
            "Clone",
            "DefineProp",
            "DeleteProp",
            "GetOwnPropDesc",
            "HasOwnProp",
            "OwnProps",
        ],
    ),
    (
        "map",
        &[
            "__Enum", "__New", "Clear", "Clone", "Delete", "Get", "Has", "Set",
        ],
    ),
    (
        "array",
        &[
            "__Enum", "__New", "Clone", "Delete", "Get", "Has", "InsertAt", "Pop", "Push",
            "RemoveAt",
        ],
    ),
    ("buffer", &["__New"]),
    ("error", &["__New"]),
    (
        "file",
        &[
            "Close",
            "RawRead",
            "RawWrite",
            "Read",
            "ReadChar",
            "ReadDouble",
            "ReadFloat",
            "ReadInt",
            "ReadInt64",
            "ReadLine",
            "ReadShort",
            "ReadUChar",
            "ReadUInt",
            "ReadUShort",
            "Seek",
            "Write",
            "WriteChar",
            "WriteDouble",
            "WriteFloat",
            "WriteInt",
            "WriteInt64",
            "WriteLine",
            "WriteShort",
            "WriteUChar",
            "WriteUInt",
            "WriteUShort",
        ],
    ),
    ("func", &["Bind", "Call", "IsByRef", "IsOptional"]),
    ("enumerator", &[]),
    (
        "menu",
        &[
            "Add",
            "AddStandard",
            "Check",
            "Delete",
            "Disable",
            "Enable",
            "Insert",
            "Rename",
            "SetColor",
            "SetIcon",
            "Show",
            "ToggleCheck",
            "ToggleEnable",
            "Uncheck",
        ],
    ),
    (
        "gui",
        &[
            "__Enum",
            "__New",
            "Add",
            "AddActiveX",
            "AddButton",
            "AddCheckBox",
            "AddComboBox",
            "AddCustom",
            "AddDateTime",
            "AddDDL",
            "AddDropDownList",
            "AddEdit",
            "AddGroupBox",
            "AddHotkey",
            "AddLink",
            "AddListBox",
            "AddListView",
            "AddMonthCal",
            "AddPic",
            "AddPicture",
            "AddProgress",
            "AddRadio",
            "AddSlider",
            "AddStatusBar",
            "AddTab",
            "AddTab2",
            "AddTab3",
            "AddText",
            "AddTreeView",
            "AddUpDown",
            "Destroy",
            "Flash",
            "GetClientPos",
            "GetPos",
            "Hide",
            "Maximize",
            "Minimize",
            "Move",
            "OnEvent",
            "Opt",
            "Restore",
            "SetFont",
            "Show",
            "Submit",
        ],
    ),
    ("clipboardall", &["__New"]),
    ("inputhook", &["__New", "KeyOpt", "Start", "Stop", "Wait"]),
    ("regexmatchinfo", &["__Enum", "__Get", "Len", "Name", "Pos"]),
    ("boundfunc", &[]),
    ("closure", &[]),
    ("class", &[]),
];

/// The classes whose prototype is not a prototype.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the prototype of an
/// `Any`, of a `ComObject` and of a `ComValue` is not one: it has no method
/// at all, and the refusal says 'This value of type "Prototype" has no method
/// named "OwnProps".'.
const NOT_A_PROTOTYPE: [&str; 4] = ["any", "comobject", "comvalue", "varref"];

/// The name of a method of the base object, the way v2 spells it.
///
/// The binary spells five of them with the class in front, and the four
/// that the script asks about every day plainly.
pub(crate) fn the_name_of_a_method_of_the_base_object(method: &str) -> String {
    let lower = method.to_lowercase();
    BASE.iter()
        .find(|(name, _)| *name == lower)
        .map(|(_, spelled)| spelled.to_string())
        .unwrap_or_else(|| method.to_string())
}

/// Whether the name is the name of a method of the base object.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the methods of the
/// base object are the methods of every object, and they are inherited.
pub(crate) fn the_base_object_has_the_method(name: &str) -> bool {
    let name = name.to_lowercase();
    CALLS
        .iter()
        .any(|(owner, member, _, _)| *owner == "object" && *member == name)
}

/// The name and the number of the arguments of a method of a class.
///
/// `None` means that the class type has no method of that name.
pub(crate) fn the_method_of(class: &str, method: &str) -> Option<(&'static str, i64, i64)> {
    let class = class.to_lowercase();
    let method = method.to_lowercase();
    METHODS
        .iter()
        .find(|(owner, name, _, _, _)| *owner == class && *name == method)
        .map(|(_, _, spelled, min, max)| (*spelled, *min, *max))
}

/// The number of the arguments of the call the script writes.
///
/// `None` means that the runtime does not know the number of the
/// arguments of that member.
pub(crate) fn the_arity_of(class: &str, method: &str) -> Option<(i64, i64)> {
    let class = class.to_lowercase();
    let method = method.to_lowercase();
    let own = CALLS
        .iter()
        .find(|(owner, name, _, _)| *owner == class && *name == method);
    let found = match own {
        Some(found) => Some(found),
        // Every object has the methods of the base object.
        None => CALLS
            .iter()
            .find(|(owner, name, _, _)| *owner == "object" && *name == method),
    };
    found.map(|(_, _, min, max)| (*min, *max))
}

/// The methods that every class has as static ones.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the methods of the
/// base object are the static methods of every class -- `HasProp`,
/// `HasMethod`, `GetMethod`, `HasBase`, `OwnProps`, `HasOwnProp`,
/// `DefineProp`, `DeleteProp`, `GetOwnPropDesc` and `Clone` -- and the
/// call of the class is one of them.
pub(crate) fn the_static_methods_of_a_class() -> [&'static str; 11] {
    [
        "call",
        "clone",
        "defineprop",
        "deleteprop",
        "getmethod",
        "getownpropdesc",
        "hasbase",
        "hasmethod",
        "hasownprop",
        "hasprop",
        "ownprops",
    ]
}

/// The name of a member of a class, the way v2 names it in a failure.
///
/// A method of a class type is "Map.Prototype.Get", and a method of the
/// base object keeps the name it has: "Object.Prototype.OwnProps", or
/// "HasProp" for the four that the script asks about every day.
pub(crate) fn the_name_of(class: &str, member: &str) -> String {
    format!("{}.Prototype.{}", the_spelling_of(class), member)
}

/// The name of the method of a class, the way v2 spells it.
///
/// The methods of the base object have a name of their own, and the
/// methods of the class types are in the table of the methods.
fn the_spelling_of_the_method(class: &str, method: &str) -> String {
    let lower = method.to_lowercase();
    if let Some((_, spelled)) = BASE.iter().find(|(name, _)| *name == lower) {
        return spelled.to_string();
    }
    the_method_of(class, method)
        .map(|(spelled, _, _)| spelled.to_string())
        .unwrap_or_else(|| method.to_string())
}

/// The name of the property of a class, the way v2 spells it.
fn the_spelling_of_the_property(class: &str, property: &str) -> String {
    let lower = property.to_lowercase();
    PROPERTIES
        .iter()
        .find(|(owner, name, _)| *owner == class && *name == lower)
        .map(|(_, _, spelled)| spelled.to_string())
        .unwrap_or_else(|| property.to_string())
}

/// The members that the prototype of a class owns.
///
/// The list is empty for a class that declares no member of its own: the
/// classes of the errors other than the `Error`, and the `BoundFunc`, the
/// `Closure` and the `Class`.
pub(crate) fn the_members_of_the_prototype(class: &str) -> &'static [&'static str] {
    let lower = class.to_lowercase();
    PROTOTYPE_MEMBERS
        .iter()
        .find(|(owner, _)| *owner == lower)
        .map(|(_, members)| *members)
        .unwrap_or(&[])
}

/// The methods that the prototype of a class owns.
pub(crate) fn the_methods_of_the_prototype(class: &str) -> &'static [&'static str] {
    let lower = class.to_lowercase();
    PROTOTYPE_METHODS
        .iter()
        .find(|(owner, _)| *owner == lower)
        .map(|(_, methods)| *methods)
        .unwrap_or(&[])
}

/// The property that the prototype of a class owns, if it owns one.
///
/// The members that are not methods are the properties of the class, and the
/// `__Item` of a class is one of them.
pub(crate) fn the_property_of_the_prototype(class: &str, property: &str) -> Option<&'static str> {
    let lower = property.to_lowercase();
    let methods = the_methods_of_the_prototype(class);
    the_members_of_the_prototype(class)
        .iter()
        .find(|member| member.to_lowercase() == lower)
        .filter(|_| !methods.iter().any(|method| method.to_lowercase() == lower))
        .copied()
}

/// Whether the prototype of a class is a prototype at all.
///
/// The `Any`, the `ComObject` and the `ComValue` have no prototype: the value
/// that stands for it has no method, and the refusal says so.
pub(crate) fn the_prototype_is_not_one(class: &str) -> bool {
    NOT_A_PROTOTYPE.contains(&class.to_lowercase().as_str())
}

/// The name of a method of a class, the way v2 names it in a failure.
///
/// The methods of the base object keep the name they have, and the
/// methods of the class types carry the class in front.
fn the_full_name_of(class: &str, method: &str) -> String {
    let lower = method.to_lowercase();
    if BASE.iter().any(|(name, _)| *name == lower) {
        return the_spelling_of_the_method(class, method);
    }
    the_name_of(class, &the_spelling_of_the_method(class, method))
}

/// The failure of a member of a class, with the name of the member in
/// it.
///
/// v2 carries the name of the member in the `What` of the error. A
/// failure that carries a name of its own keeps it.
pub(crate) fn named(failure: RuntimeFailure, what: &str) -> RuntimeFailure {
    match failure {
        RuntimeFailure::Error(error) => RuntimeFailure::Detailed(
            error,
            Detail {
                what: what.to_string(),
                extra: String::new(),
            },
        ),
        RuntimeFailure::Detailed(error, mut detail) => {
            if detail.what.is_empty() {
                detail.what = what.to_string();
            }
            RuntimeFailure::Detailed(error, detail)
        }
        other => other,
    }
}

/// The failure of a member of a class, with the value that failed.
///
/// v2 carries the value the member complained about in the `Extra` of
/// the error: the key of a map, the place of an array.
pub(crate) fn the_failure_with(message: &str, extra: &str, line: u32) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError {
            message: message.to_string(),
            line,
        },
        Detail {
            what: String::new(),
            extra: extra.to_string(),
        },
    )
}

/// The failure of a call that does not have the number of the arguments
/// the member wants.
///
/// v2 checks the number of the arguments when the script writes the
/// call, and the name of the member goes in the `Extra` of the error,
/// and nothing goes in the `What`.
pub(crate) fn the_arity_failure(
    class: &str,
    method: &str,
    message: &str,
    line: u32,
) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError {
            message: message.to_string(),
            line,
        },
        Detail {
            what: String::new(),
            extra: the_full_name_of(class, method),
        },
    )
}

/// The failure of a call that goes through a value of a method of a
/// class type.
///
/// v2 names the method that made the call in the `What` of the error,
/// and the method that was called in the `Extra`.
pub(crate) fn the_arity_failure_of_a_method(
    class: &str,
    method: &str,
    message: &str,
    line: u32,
) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError {
            message: message.to_string(),
            line,
        },
        Detail {
            what: the_name_of("func", "Call"),
            extra: the_full_name_of(class, method),
        },
    )
}

/// The number of the arguments that the constructor of a class type
/// takes, and the names that v2 puts in a failure of it.
///
/// `None` means that the class type is made with any number of
/// arguments, or that the runtime does not know the number.
fn the_constructor_of(class: &str) -> Option<(i64, i64, &'static str, &'static str)> {
    let class = class.to_lowercase();
    // The classes of the errors are made the way the root of them is.
    let owner = if error::is_a_class(&class) {
        "error"
    } else {
        class.as_str()
    };
    CONSTRUCTORS
        .iter()
        .find(|(name, _, _, _, _)| *name == owner)
        .map(|(_, min, max, what, extra)| (*min, *max, *what, *extra))
}

/// The failure of a class type that v2 does not make.
///
/// The call goes through the base object, and the base of the class has
/// no such thing: "Invalid base.".
fn the_failure_of_a_class_that_is_not_made(line: u32) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError {
            message: "Invalid base.".to_string(),
            line,
        },
        Detail {
            what: THE_CALL_OF_THE_BASE.to_string(),
            extra: String::new(),
        },
    )
}

/// The failure of a constructor that has too many arguments.
///
/// v2 names the call in the `What` of the error, and the constructor in
/// the `Extra`.
fn the_arity_failure_of_a_constructor(
    what: &str,
    extra: &str,
    message: &str,
    line: u32,
) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError {
            message: message.to_string(),
            line,
        },
        Detail {
            what: what.to_string(),
            extra: extra.to_string(),
        },
    )
}

/// The class types the runtime carries.
///
/// `Object` is the root of every class: v2 has it, and every object of
/// the tree is one.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the classes of the
/// dialect are the ones that a script can read as a class, and these are
/// they. The hotkey is the one that is not a name of its own: the name
/// `Hotkey` is a function of the dialect, and the class of a hotkey is
/// the class of the object that the function makes.
pub(crate) const TYPES: [&str; 26] = [
    "any",
    "array",
    "boundfunc",
    "buffer",
    "class",
    "clipboardall",
    "closure",
    "comobject",
    "comvalue",
    "enumerator",
    "error",
    "file",
    "float",
    "func",
    "gui",
    "hotkey",
    "inputhook",
    "integer",
    "map",
    "menu",
    "menubar",
    "number",
    "object",
    "primitive",
    "regexmatchinfo",
    "string",
];

/// Whether the name is a class type of the runtime.
pub(crate) fn is_type(name: &str) -> bool {
    let name = name.to_lowercase();
    TYPES.contains(&name.as_str()) || error::is_a_class(&name)
}

/// Whether the name is a class that a script can read.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: `x := Hotkey` is a
/// function of the dialect, and not the class of a hotkey. Every other
/// class of the runtime is a name of its own, and a script reads it as
/// the class.
pub fn is_a_name_of_a_class(name: &str) -> bool {
    let name = name.to_lowercase();
    (TYPES.contains(&name.as_str()) && name != "hotkey") || error::is_a_class(&name)
}

/// The class types the runtime carries, the way v2 spells them.
pub fn the_class_types() -> Vec<String> {
    let mut names: Vec<String> = TYPES
        .iter()
        .filter(|name| **name != "hotkey")
        .map(|name| the_spelling_of(name))
        .collect();
    for (name, spelled, _) in error::CLASSES.iter() {
        if *name != "error" {
            names.push(spelled.to_string());
        }
    }
    names
}

/// The classes whose class value owns the call of it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the class value of
/// the `Map` owns the call and the prototype, and the class value of the
/// `Error` owns the prototype alone. The classes that a script calls are
/// the ones that own the call.
const THE_CLASSES_THAT_OWN_THE_CALL: [&str; 10] = [
    "object",
    "map",
    "array",
    "buffer",
    "menu",
    "inputhook",
    "clipboardall",
    "gui",
    "comobject",
    "comvalue",
];

/// Whether the class value of the class owns the call of it.
///
/// The classes of the errors are not among them, and neither are the
/// functions: a `Func` and a `BoundFunc` are made by a definition, and not
/// by a call.
pub(crate) fn the_class_owns_the_call(class: &str) -> bool {
    THE_CLASSES_THAT_OWN_THE_CALL.contains(&class.to_lowercase().as_str())
}

/// The classes of v2 that this runtime does not carry.
///
/// The COM objects are a thing of Windows, and this runtime does not
/// carry them. A script reads the name of the class -- the `ComObject`
/// and the `ComValue` are classes of the dialect -- and the call of it
/// fails with the reason of the machine.
///
/// The number of the arguments is checked before that reason: the
/// dialect checks it first, and a script that calls the `ComObject`
/// without the name of the class is told that it forgot it.
const THE_CLASSES_THAT_ARE_NOT_HERE: [&str; 2] = ["comobject", "comvalue"];

/// The reason of a class of v2 that this runtime does not carry.
///
/// The COM objects are a thing of Windows, and this runtime does not
/// carry them.
fn the_failure_of_a_class_that_is_not_here(line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: "the COM objects are a thing of Windows, and this runtime does not carry them."
            .to_string(),
        line,
    })
}

/// The kind of an argument of the `ComValue`.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the first argument
/// is the type of the value and the third one is the flags of the
/// wrapper, and the two are numbers. The value in the middle is
/// anything.
///
/// The check of the kind comes before the reason of the machine: the
/// dialect refuses the call before it tries to make the COM object.
fn the_kind_of_a_com_value_argument(
    index: usize,
    value: Option<&Value>,
    line: u32,
) -> Option<RuntimeFailure> {
    let value = value?;
    match value {
        Value::Int(_) | Value::Float(_) => None,
        other => Some(RuntimeFailure::Detailed(
            RuntimeError {
                message: crate::runtime::requires(index, "ComValue.Call", TypeName::Number, other),
                line,
            },
            Detail {
                what: "ComValue.Call".to_string(),
                extra: other.to_text(),
            },
        )),
    }
}

/// The failure of a class that a script cannot call.
///
/// The `Any` is the type of any value, and it is not a class that can be
/// made: the call of it goes through the `Call` method of the class, and
/// a class has none. Measured on the binary of v2.0.19 on 2026-10-31:
/// "This value of type "Class" has no method named "Call".". The failure
/// is of the class `MethodError`.
fn the_failure_of_a_class_that_cannot_be_called(rt: &Runtime, line: u32) -> RuntimeFailure {
    // The class value of the runtime, so that the refusal names the kind
    // of it: a class is a "Class".
    let value = rt
        .class_value_of("any")
        .unwrap_or_else(|| class_value("any"));
    rt.no_method(&value, "Call", line)
}

/// The class that a class of the dialect extends.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the base of the
/// `Object` is the `Any`, and the base of every other class is the
/// `Object`. The `Any` is the root of the tree of the classes, and it
/// has no base.
///
/// The errors of v2 are a tree of their own, and the enumerator is a
/// function of its own kind: the two have a base of their own.
pub(crate) fn the_base_class_of_a_class(class: &str) -> Option<&'static str> {
    let class = class.to_lowercase();
    if class == "any" {
        return None;
    }
    if class == "object" || class == "primitive" {
        return Some("any");
    }
    // The primitive types of v2 are a tree of their own: the `Number` and the
    // `String` are `Primitive`, and the `Integer` and the `Float` are
    // `Number`. Measured on the binary of v2.0.19: `3 is Number` and `3 is
    // Primitive` are one, and `1.5 is Integer` is zero.
    if let Some(base) = the_base_of_the_primitive(&class) {
        return Some(base);
    }
    // The errors of v2 are a tree of their own, and the enumerator is a
    // function of its own kind: the two have a base of their own.
    if let Some(base) = the_base_of(&class) {
        return Some(base);
    }
    Some("object")
}

/// The base of a primitive type of v2.
///
/// Measured on the binary of the v2.0.19: the `Primitive` is under the
/// `Any`, the `Number` and the `String` are under the `Primitive`, and the
/// `Integer` and the `Float` are under the `Number`.
fn the_base_of_the_primitive(class: &str) -> Option<&'static str> {
    match class {
        "number" | "string" => Some("primitive"),
        "integer" | "float" => Some("number"),
        "primitive" => Some("any"),
        _ => None,
    }
}

/// The base of a primitive type of v2, for the chain of the `is`.
///
/// It is the function the runtime walks the chain with when it meets a name
/// that is not a class of the script and not a class type of the tree: the
/// `Integer` and the `Float` are under the `Number`, the `Number` and the
/// `String` under the `Primitive`, and the `Primitive` under the `Any`.
pub(crate) fn the_base_of_the_primitive_of_the_class(class: &str) -> Option<&'static str> {
    the_base_of_the_primitive(&class.to_lowercase())
}

/// The class a class type of the runtime derives from.
///
/// The errors of v2 are a tree of their own: a `PropertyError` is a
/// `MemberError` and an `UnsetError` as well, and every one of them is
/// an `Error`. The other classes of the runtime are the root of it.
pub(crate) fn the_base_of(class: &str) -> Option<&'static str> {
    // The enumerator is a function of its own kind: it derives from
    // `Func`, and the docs of v2 say so.
    if class.eq_ignore_ascii_case("enumerator") {
        return Some("func");
    }
    error::the_base_of(class)
}

/// The value of a class type: the class itself.
///
/// The class carries its prototype, the way a class of the script does:
/// the base of an object of this type is the prototype of it.
pub(crate) fn class_value(name: &str) -> Value {
    let mut table = BTreeMap::new();
    // The name of the class is the one v2 spells: the name of the
    // runtime carries it in lower case. v2 does not show it, and the
    // tree keeps it under a key of its own.
    put(
        &mut table,
        crate::runtime::CLASS_NAME,
        Value::Text(the_spelling_of(name)),
    );
    put(
        &mut table,
        "prototype",
        crate::runtime::prototype_value(name),
    );
    Value::Object(Rc::new(RefCell::new(Object {
        class: name.to_lowercase(),
        properties: table,
    })))
}

/// The object of a class type, with nothing in it yet.
pub(crate) fn object(class: &str) -> Value {
    Value::Object(Rc::new(RefCell::new(Object {
        class: class.to_lowercase(),
        properties: BTreeMap::new(),
    })))
}

/// The object of a class type, with a property in it.
pub(crate) fn object_with(class: &str, properties: Vec<(&str, Value)>) -> Value {
    let mut table = BTreeMap::new();
    for (name, value) in properties {
        put(&mut table, name, value);
    }
    Value::Object(Rc::new(RefCell::new(Object {
        class: class.to_lowercase(),
        properties: table,
    })))
}

/// The class types the runtime carries, and the name that v2 spells them
/// with.
///
/// The dialect spells some of them with more than one capital, and the
/// name of the class is the name that the dialect uses.
const THE_SPELLINGS: [(&str, &str); 8] = [
    ("boundfunc", "BoundFunc"),
    ("clipboardall", "ClipboardAll"),
    ("closure", "Closure"),
    ("comobject", "ComObject"),
    ("comvalue", "ComValue"),
    ("inputhook", "InputHook"),
    ("menubar", "MenuBar"),
    ("regexmatchinfo", "RegExMatchInfo"),
];

/// The name of a class of the runtime, the way v2 spells it.
///
/// The classes of the errors have a spelling of their own: `TypeError`,
/// and not `Typeerror`.
pub(crate) fn the_spelling_of(name: &str) -> String {
    if let Some(spelled) = error::the_spelling_of(name) {
        return spelled;
    }
    let lower = name.to_lowercase();
    if let Some((_, spelled)) = THE_SPELLINGS.iter().find(|(key, _)| *key == lower) {
        return spelled.to_string();
    }
    let mut spelled = name.to_string();
    if let Some(first) = spelled.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    spelled
}

/// The object a class type makes.
///
/// `None` means the name is not a class type of the runtime.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // A class type that v2 does not make: the call goes through the
    // base object, and the base of the class has no such thing.
    if NOT_MADE.contains(&class.to_lowercase().as_str()) {
        return Some(Err(the_failure_of_a_class_that_is_not_made(line)));
    }
    // The number of the arguments of the constructor is checked before
    // the object is made, and before the reason of the machine: the
    // dialect checks the number of the arguments first.
    if let Some((min, max, what, extra)) = the_constructor_of(class) {
        let count = arguments.len() as i64;
        if count < min || (max >= 0 && count > max) {
            let message = if count < min {
                "Too few parameters passed to function."
            } else {
                "Too many parameters passed to function."
            };
            return Some(Err(the_arity_failure_of_a_constructor(
                what, extra, message, line,
            )));
        }
    }
    // The `Any` is the type of any value, and it is not a class that can
    // be made.
    if class.eq_ignore_ascii_case("any") {
        return Some(Err(the_failure_of_a_class_that_cannot_be_called(rt, line)));
    }
    // The COM objects are a thing of Windows, and this runtime does not
    // carry them. The kinds of the arguments are the dialect's, and they
    // come first.
    if class.eq_ignore_ascii_case("comvalue") {
        if let Some(failure) = the_kind_of_a_com_value_argument(1, arguments.first(), line) {
            return Some(Err(failure));
        }
        if let Some(failure) = the_kind_of_a_com_value_argument(3, arguments.get(2), line) {
            return Some(Err(failure));
        }
    }
    if THE_CLASSES_THAT_ARE_NOT_HERE.contains(&class.to_lowercase().as_str()) {
        return Some(Err(the_failure_of_a_class_that_is_not_here(line)));
    }
    // The errors of v2 are a tree of classes, and every one of them is
    // made the way the root of it is.
    if error::is_a_class(class) {
        return Some(
            error::construct(class, arguments, rt, line)
                .map_err(|failure| named(failure, &the_name_of(class, "__New"))),
        );
    }
    let made = match class.to_lowercase().as_str() {
        "array" => Some(array::construct(arguments, rt, line)),
        "menu" | "menubar" => {
            // Measured on the binary of v2.0.19 on 2026-11-10: `Type` of a
            // MenuBar answers "MenuBar", so the spelling the script used is
            // the class of the object.
            let spelling = class.to_string();
            let made = match menu::construct(arguments, rt, line) {
                Ok(made) => made,
                Err(failure) => return Some(Err(failure)),
            };
            if let Value::Object(object) = &made {
                object.borrow_mut().class = spelling;
            }
            Some(Ok(made))
        }
        "buffer" => Some(buffer::construct(arguments, rt, line)),
        "clipboardall" => Some(clipboard::construct(arguments, rt, line)),
        "hotkey" => Some(hotkey::construct(arguments, rt, line)),
        "map" => Some(map::construct(arguments, rt, line)),
        "gui" | "gui.control" | "gui.listview" | "gui.treeview" | "gui.statusbar" => {
            Some(gui::construct(class, arguments, rt, line))
        }
        _ => None,
    };
    made.map(|result| result.map_err(|failure| named(failure, &the_name_of(class, "__New"))))
}

/// A method of a class type.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    class: &str,
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // A window of the script is remembered when it is asked for anything, which
    // is where the runtime and the object are both in hand. The drain of the
    // events needs the list, because a built-in has no reference to an object.
    if class.eq_ignore_ascii_case("gui") {
        rt.remember_a_gui(this);
    }
    // The two members of a `Gui` that only need a window to be there. Measured
    // on the binary of v2.0.19 on 2026-11-09: `Show` and `Destroy` answer the
    // **empty text**, and the window is opened on the first of them, because the
    // construction is generic and cannot make one.
    if class == "gui" && (method.eq_ignore_ascii_case("show") || method.eq_ignore_ascii_case("destroy")) {
        if !rt.desktop().gui_is_open() {
            if let Err(reason) = rt.desktop().gui_open("") {
                return Some(Err(the_failure_with(&reason, "", line)));
            }
        }
        let answer = if method.eq_ignore_ascii_case("show") {
            rt.desktop().gui_show(0)
        } else {
            rt.desktop().gui_destroy(0)
        };
        return Some(match answer {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // `Add` of a `Gui`: the control of the window, answered as an object whose
    // class is the one v2 reports -- `Gui.Edit` for an `Edit` -- and whose
    // `ClassNN` and `Hwnd` are properties of it. The object is written directly
    // rather than made by the class table, because a control of a script's own
    // window is not a class type of the runtime.
    if class == "gui" && method.eq_ignore_ascii_case("add") {
        let kind = arguments
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default();
        // The third argument of `Add` is the text of the control, and for a
        // list-shaped control it is an ARRAY of the items. Measured on the
        // binary of v2.0.19 on 2026-11-10: `Add("DropDownList", "", ["one",
        // "two"])` makes a ComboBox with those two. The helper reads them as
        // lines of the text, so the array is joined here and the other kinds
        // keep the plain text they had.
        let text = match arguments.get(2) {
            Some(Value::List(items)) => items
                .iter()
                .map(|item| item.to_text())
                .collect::<Vec<_>>()
                .join("\n"),
            Some(value @ Value::Object(_)) => {
                let items = crate::classes::array::the_items(value);
                if items.is_empty() {
                    value.to_text()
                } else {
                    items
                        .iter()
                        .map(|item| item.to_text())
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            }
            Some(value) => value.to_text(),
            None => String::new(),
        };
        // The numeric controls (UpDown, Slider, Progress) carry a `Range` in
        // the OPTIONS (the second argument) and, for a Slider or Progress, the
        // starting value in the TEXT. Measured on the binary of v2.0.19 on
        // 2026-11-10: `Add("UpDown", "Range1-10")` starts at 1, and
        // `Add("Slider", "Range0-100", "50")` starts at 50. Both are handed
        // to the helper as one string, which reads the range and the value.
        let text = if matches!(
            kind.to_lowercase().as_str(),
            "updown" | "slider" | "progress"
        ) {
            let options = arguments
                .get(1)
                .map(|value| value.to_text())
                .unwrap_or_default();
            if options.is_empty() {
                text
            } else if text.is_empty() {
                options
            } else {
                format!("{options} {text}")
            }
        } else {
            text
        };
        return Some(match the_control_of_the_gui(&kind, &text, rt, line) {
            Ok(made) => {
                remember_the_control(this, &made);
                Ok(made)
            }
            Err(failure) => Err(failure),
        });
    }
    // The short spelling of the same question: `AddText` is `Add("Text", …)`.
    // Its arguments are the options and then the text, so the text is the second.
    if class == "gui"
        && method.len() > 3
        && method.is_char_boundary(3)
        && method[..3].eq_ignore_ascii_case("add")
    {
        let kind = method[3..].to_string();
        let text = arguments
            .get(1)
            .map(|value| value.to_text())
            .unwrap_or_default();
        return Some(match the_control_of_the_gui(&kind, &text, rt, line) {
            Ok(made) => {
                remember_the_control(this, &made);
                Ok(made)
            }
            Err(failure) => Err(failure),
        });
    }
    // `OnEvent` of a `Gui`: keep the function that answers an event of the
    // window. Measured on the binary of v2.0.19 on 2026-11-09: it answers the
    // empty text, an event the window does not have fails on the first
    // parameter, and a second parameter that is not a function fails on the
    // second. `Click` is an event of a control and not of the window.
    if class == "gui" && method.eq_ignore_ascii_case("onevent") {
        let event = arguments
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default();
        let known = event.eq_ignore_ascii_case("close") || event.eq_ignore_ascii_case("escape");
        if !known {
            return Some(Err(the_failure_with(
                "Parameter #1 of Gui.Prototype.OnEvent is invalid.",
                "",
                line,
            )));
        }
        let handler = match arguments.get(1) {
            Some(Value::Object(object)) if object.borrow().class.eq_ignore_ascii_case("func") => {
                arguments[1].clone()
            }
            _ => {
                return Some(Err(the_failure_with(
                    "Parameter #2 of Gui.Prototype.OnEvent is invalid.",
                    "",
                    line,
                )))
            }
        };
        if let Value::Object(object) = this {
            object.borrow_mut().properties.insert(
                format!("on:{}", event.to_lowercase()),
                crate::runtime::Property {
                    name: format!("On{event}"),
                    value: handler,
                },
            );
        }
        return Some(Ok(Value::Text(String::new())));
    }
    // `OnEvent` of a control of a window. Measured on the binary of v2.0.19 on
    // 2026-11-09: the events are **per kind of control** -- a `Button` has
    // `Click` and an `Edit` has `Change`, and neither has the other's, nor
    // `Close` which is a window's. And **the callback's arity is checked**: a
    // function that takes no parameters answers "Invalid callback function.",
    // because v2 passes the control to a control's handler.
    if class.to_lowercase().starts_with("gui.") && method.eq_ignore_ascii_case("onevent") {
        let event = arguments
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default();
        let known = the_events_of_a_control(class)
            .iter()
            .any(|held| held.eq_ignore_ascii_case(&event));
        if !known {
            return Some(Err(the_failure_with(
                "Parameter #1 of Gui.Control.Prototype.OnEvent is invalid.",
                "",
                line,
            )));
        }
        let handler = match arguments.get(1) {
            Some(Value::Object(object)) if object.borrow().class.eq_ignore_ascii_case("func") => {
                arguments[1].clone()
            }
            _ => {
                return Some(Err(the_failure_with(
                    "Parameter #2 of Gui.Control.Prototype.OnEvent is invalid.",
                    "",
                    line,
                )))
            }
        };
        let takes_one = match &handler {
            Value::Object(object) => {
                let object = object.borrow();
                let number = |name: &str| {
                    object
                        .properties
                        .get(name)
                        .map(|held| held.value.to_text())
                        .unwrap_or_default()
                };
                // A callback with a rest takes the control as well: the rest
                // is not counted among the parameters of the function.
                if number("isvariadic") == "1" {
                    true
                } else {
                    number("maxparams").parse::<i64>().unwrap_or(0) >= 1
                }
            }
            _ => false,
        };
        if !takes_one {
            return Some(Err(the_failure_with("Invalid callback function.", "", line)));
        }
        if let Value::Object(object) = this {
            object.borrow_mut().properties.insert(
                format!("on:{}", event.to_lowercase()),
                crate::runtime::Property {
                    name: format!("On{event}"),
                    value: handler,
                },
            );
        }
        return Some(Ok(Value::Text(String::new())));
    }
    // `Move` of a control: a place and a size, and only the size can be honoured.
    // Measured on 2026-11-09: v2 answers the empty text and numbers the first
    // parameter 2, because the first is the control the method is asked of. The
    // window of this tree lays its controls out in order, so a place is refused
    // with the reason rather than ignored -- a control that answers as if it had
    // moved and had not is the worst kind of answer.
    if class.to_lowercase().starts_with("gui.") && method.eq_ignore_ascii_case("move") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let given = |index: usize| {
            arguments
                .get(index)
                .map(|value| !value.to_text().trim().is_empty())
                .unwrap_or(false)
        };
        if given(0) || given(1) {
            return Some(Err(the_failure_with(
                "the window of this tree lays its controls out in order, so the place of a control cannot be set",
                "",
                line,
            )));
        }
        if !given(2) && !given(3) {
            return Some(Ok(Value::Text(String::new())));
        }
        let width = arguments.get(2).map(|v| v.to_text()).unwrap_or_default();
        let height = arguments.get(3).map(|v| v.to_text()).unwrap_or_default();
        let width: i64 = width.trim().parse().unwrap_or(-1);
        let height: i64 = height.trim().parse().unwrap_or(-1);
        return Some(match rt.desktop().gui_size(number, width, height) {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // `Opt` of a window. Measured on the binary of v2.0.19 on 2026-11-09: an
    // option of v2 answers the empty text and one that is not answers
    // "Invalid option.". The reader of the gui module gives that validation; the
    // four of the eighteen that this window can set are applied, and the rest are
    // refused with the reason -- a window whose title bar belongs to the
    // compositor is not one whose buttons a program sets.
    if class.eq_ignore_ascii_case("gui") && method.eq_ignore_ascii_case("opt") {
        let options = arguments
            .first()
            .map(|value| value.to_text())
            .unwrap_or_default();
        if let Err(failure) = gui::the_options_are_read(&options, line) {
            return Some(Err(failure));
        }
        for word in options.split([' ', '\t']) {
            let word = word.trim();
            if word.is_empty() {
                continue;
            }
            let on = !word.starts_with('-');
            let name = word.trim_start_matches(['+', '-']);
            let lower = name.to_lowercase();
            let honoured = ["alwaysontop", "resize", "caption", "disabled"]
                .iter()
                .find(|held| lower.starts_with(*held));
            let Some(honoured) = honoured.copied() else {
                return Some(Err(the_failure_with(
                    "the window of this tree cannot set that option",
                    "",
                    line,
                )));
            };
            if let Err(reason) = rt.desktop().gui_option(honoured, on) {
                return Some(Err(the_failure_with(&reason, "", line)));
            }
        }
        return Some(Ok(Value::Text(String::new())));
    }
    // `Hide` of a window: it takes no parameters and answers the empty text.
    // Measured on the binary of v2.0.19 on 2026-11-09.
    if class.eq_ignore_ascii_case("gui") && method.eq_ignore_ascii_case("hide") {
        if !arguments.is_empty() {
            return Some(Err(the_failure_with(
                "Too many parameters passed to function.",
                "",
                line,
            )));
        }
        return Some(match rt.desktop().gui_hide() {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // `Focus` of a control: it takes no parameters and answers the empty text.
    // Measured on the binary of v2.0.19 on 2026-11-09.
    if class.to_lowercase().starts_with("gui.") && method.eq_ignore_ascii_case("focus") {
        if !arguments.is_empty() {
            return Some(Err(the_failure_with(
                "Too many parameters passed to function.",
                "",
                line,
            )));
        }
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_focus(number) {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // `__Item` is the method behind the index syntax, and a script may
    // call it by its name. The classes that have one answer it here.
    if method.eq_ignore_ascii_case("__item") {
        let key = arguments
            .first()
            .cloned()
            .unwrap_or(Value::Text(String::new()));
        if let Some(result) = item(class, &key, this, rt, line) {
            return Some(result);
        }
    }
    let answer = call_once(class, method, this, arguments, rt, line);
    // The name of a method is spelled the way the script spells it, and
    // the module matches the names of the table of the binary. The
    // second try is the name in lower case.
    match answer {
        Some(result) => {
            Some(result.map_err(|failure| named(failure, &the_full_name_of(class, method))))
        }
        None => {
            let lower = method.to_lowercase();
            if lower == method {
                return None;
            }
            call_once(class, &lower, this, arguments, rt, line).map(|result| {
                result.map_err(|failure| named(failure, &the_full_name_of(class, method)))
            })
        }
    }
}

/// A method of a class type, with the name as it is.
fn call_once(
    class: &str,
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "array" => array::call(method, this, arguments, rt, line),
        "buffer" => buffer::call(method, this, arguments, rt, line),
        "enumerator" => enumerator::call(method, this, arguments, rt, line),
        "error" => error::call(method, this, arguments, rt, line),
        "file" => file::call(method, this, arguments, rt, line),
        "func" | "boundfunc" => func::call(method, this, arguments, rt, line),
        "hotkey" => hotkey::call(method, this, arguments, rt, line),
        "map" => map::call(method, this, arguments, rt, line),
        // The methods of the base object are the methods of every object,
        // and they are answered by the runtime.
        "object" => rt.base_method(this, method, arguments, line),
        "gui" | "gui.control" | "gui.listview" | "gui.treeview"
        | "gui.menu" | "gui.statusbar" => {
            gui::call(class, method, this, arguments, rt, line)
        }
        "menu" | "menubar" => menu::call(method, this, arguments, rt, line),
        _ => None,
    }
}

/// The call of a value.
///
/// The value is called, and not the `Call` method of it: v2 tells the two
/// apart in the failure of the number of the arguments.
pub(crate) fn call_the_value(
    class: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "func" | "boundfunc" => func::call_the_value(this, arguments, rt, line),
        // The enumerator is a function of its own kind, and the call of
        // its value is the turn of the enumeration.
        "enumerator" => enumerator::call_the_value(this, arguments, rt, line),
        _ => call(class, "call", this, arguments, rt, line),
    }
}

/// A property of a class type.
///
/// `None` means the name is not a property of this class: the runtime then
/// looks in the property table of the object.
/// The events a control of this kind has.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a `Button` has `Click` and
/// an `Edit` has `Change`, and each refuses the other's. A `Text` has none.
fn the_events_of_a_control(class: &str) -> &'static [&'static str] {
    match class.to_lowercase().as_str() {
        "gui.button" | "gui.checkbox" => &["Click"],
        "gui.edit" => &["Change"],
        _ => &[],
    }
}

/// A control of a window of the script, as the object v2 reports.
///
/// The window is opened on the first control, since the construction of a `Gui`
/// is generic and cannot make one. The object is written directly rather than
/// made by the class table, because a control of a script's own window is not a
/// class type of the runtime: its class is the name v2 gives it -- `Gui.Edit` for
/// an `Edit` -- and `ClassNN` and `Hwnd` are properties of it.
fn the_control_of_the_gui(
    kind: &str,
    text: &str,
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    if !rt.desktop().gui_is_open() {
        if let Err(reason) = rt.desktop().gui_open("") {
            return Err(the_failure_with(&reason, "", line));
        }
    }
    match rt.desktop().gui_add(0, kind, text) {
        Ok((number, classnn)) => {
            let mut properties = std::collections::BTreeMap::new();
            properties.insert(
                "classnn".to_string(),
                crate::runtime::Property {
                    name: "ClassNN".to_string(),
                    value: Value::Text(classnn),
                },
            );
            properties.insert(
                "hwnd".to_string(),
                crate::runtime::Property {
                    name: "Hwnd".to_string(),
                    value: Value::Int(number as i64),
                },
            );
            Ok(Value::Object(std::rc::Rc::new(std::cell::RefCell::new(
                crate::runtime::Object {
                    class: format!("Gui.{}", kind),
                    properties,
                },
            ))))
        }
        Err(reason) => Err(the_failure_with(&reason, "", line)),
    }
}

/// Whether the class of a control is a Tab notebook.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: its `ClassNN` is
/// `SysTabControl321` and its `Value` is the 1-based page number, with an
/// out-of-range write refused as "Invalid value.".
fn the_control_is_a_tab(class: &str) -> bool {
    let tail = match class.rsplit_once('.') {
        Some((_, tail)) => tail,
        None => class,
    };
    tail.eq_ignore_ascii_case("tab")
}

/// Whether a control's `Value` is an Integer rather than a text.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: an UpDown is
/// `msctls_updown321` and shares its number with the Edit added before it; a
/// Slider is `msctls_trackbar321` and a Progress `msctls_progress321`, and
/// both start at the number in the text (or the lower bound of the Range).
fn the_control_has_an_integer_value(class: &str) -> bool {
    let tail = match class.rsplit_once('.') {
        Some((_, tail)) => tail,
        None => class,
    };
    tail.eq_ignore_ascii_case("updown")
        || tail.eq_ignore_ascii_case("slider")
        || tail.eq_ignore_ascii_case("progress")
}

/// Whether the class of a control is one of the list-shaped kinds.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the `ClassNN` of a
/// DropDownList is `ComboBox1`, and ComboBox and ListBox are the same family.
fn the_control_is_a_combo(class: &str) -> bool {
    let tail = match class.rsplit_once('.') {
        Some((_, tail)) => tail,
        None => class,
    };
    tail.eq_ignore_ascii_case("dropdownlist")
        || tail.eq_ignore_ascii_case("combobox")
        || tail.eq_ignore_ascii_case("listbox")
}

/// The flag of a control that a name of a property asks about.
fn the_flag_of(name: &str) -> Option<&'static str> {
    if name.eq_ignore_ascii_case("enabled") {
        Some("enabled")
    } else if name.eq_ignore_ascii_case("visible") {
        Some("visible")
    } else {
        None
    }
}

/// Keep a control on the window it belongs to.
///
/// The number that names it is the `Hwnd` it carries, which is the number the
/// helper answered and the same number an event about it carries. A click says
/// `click:<n>`, and this is how the drain finds the control -- and through it the
/// handler -- from that number alone.
fn remember_the_control(window: &Value, control: &Value) {
    let number = match the_number_of_a_control(control) {
        Some(number) => number,
        None => return,
    };
    if let Value::Object(object) = window {
        object.borrow_mut().properties.insert(
            format!("control:{number}"),
            crate::runtime::Property {
                name: format!("Control{number}"),
                value: control.clone(),
            },
        );
    }
}

/// The number that names a control of a window of the script.
///
/// It is the `Hwnd` the control object carries, which is the number `gui_add`
/// answered. A value that is not such an object has none.
pub(crate) fn the_number_of_a_control(this: &Value) -> Option<u64> {
    if let Value::Object(object) = this {
        if let Some(property) = object.borrow().properties.get("hwnd") {
            if let Value::Int(number) = property.value {
                return Some(number as u64);
            }
        }
    }
    None
}

/// The title of the window of a `Gui`: read it, or write it.
///
/// The window is opened on the first question about it, because the construction
/// of a `Gui` is generic and cannot make one. Measured on the binary of v2.0.19
/// on 2026-11-09: `g.Title` answers the title and `g.Title := "x"` writes it,
/// both as text.
fn the_title_of_the_gui(
    rt: &mut Runtime,
    setting: Option<&str>,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    if !rt.desktop().gui_is_open() {
        if let Err(reason) = rt.desktop().gui_open("") {
            return Err(the_failure_with(&reason, "", line));
        }
    }
    match rt.desktop().gui_title(0, setting) {
        Ok(title) => Ok(Value::Text(title)),
        Err(reason) => Err(the_failure_with(&reason, "", line)),
    }
}

pub(crate) fn property(
    class: &str,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // Measured on the binary of v2.0.19 on 2026-11-09: `g.Title` reads the title
    // of the window, and `g.Title := "x"` writes it.
    if class.eq_ignore_ascii_case("gui") && name.eq_ignore_ascii_case("title") {
        return Some(the_title_of_the_gui(rt, None, line));
    }
    // `Value` of a control of a window of the script. Measured on the binary of
    // v2.0.19 on 2026-11-09: it is read and written as text.
    if class.eq_ignore_ascii_case("gui.button") && name.eq_ignore_ascii_case("value") {
        return Some(Ok(Value::Text(String::new())));
    }
    // `Value` of a check box is whether it is TICKED, and it is a number and not a
    // text -- measured on the binary of v2.0.19. Its `Text` is the label, and the
    // shared arm below answers that, so this comes first.
    if class.eq_ignore_ascii_case("gui.checkbox") && name.eq_ignore_ascii_case("value") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_checked(number, None) {
            Ok(checked) => Ok(Value::Int(checked)),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // A Link's `Value` is always empty and writing it is refused; its `Text`
    // is the caption and goes through the ordinary `gui_value` below. A
    // Hotkey's `Text` is a SECOND, INDEPENDENT text -- measured on the binary
    // of v2.0.19 on 2026-11-10: writing `hk.Text` leaves `hk.Value` alone.
    if class.eq_ignore_ascii_case("gui.link") && name.eq_ignore_ascii_case("value") {
        return Some(Ok(Value::Text(String::new())));
    }
    if class.eq_ignore_ascii_case("gui.hotkey") && name.eq_ignore_ascii_case("text") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_second_value(number, None) {
            Ok(text) => Ok(Value::Text(text)),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // A Tab's `Value` is the 1-based page number. Measured on the binary of
    // v2.0.19 on 2026-11-10. The helper answers it as a text of digits; the
    // script sees the whole number v2 sees.
    if the_control_is_a_tab(class) && name.eq_ignore_ascii_case("value") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_value(number, None) {
            Ok(text) => Ok(Value::Int(text.parse().unwrap_or(0))),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // An UpDown, Slider or Progress has a `Value` that is an Integer; the
    // UpDown shares it with its buddy Edit. Measured on the binary of v2.0.19
    // on 2026-11-10.
    if the_control_has_an_integer_value(class) && name.eq_ignore_ascii_case("value") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_value(number, None) {
            Ok(text) => Ok(Value::Int(text.parse().unwrap_or(0))),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // A list-shaped control has two different properties where the others
    // have one. Measured on the binary of v2.0.19 on 2026-11-10: `Value` is
    // the 1-based index of the chosen item (0 when none) and `Text` is the
    // text of it; setting `Text` to an item's text chooses that item.
    if the_control_is_a_combo(class)
        && (name.eq_ignore_ascii_case("value") || name.eq_ignore_ascii_case("text"))
    {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_combo_choice(number) {
            Ok((index, text)) => Ok(if name.eq_ignore_ascii_case("value") {
                Value::Int(index)
            } else {
                Value::Text(text)
            }),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    if class.to_lowercase().starts_with("gui.")
        && (name.eq_ignore_ascii_case("value") || name.eq_ignore_ascii_case("text"))
    {
        let number = the_number_of_a_control(this).unwrap_or(0);
        return Some(match rt.desktop().gui_value(number, None) {
            Ok(text) => Ok(Value::Text(text)),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // `Enabled` and `Visible` of a control: integers of nought and one, read and
    // written. Measured on the binary of v2.0.19 on 2026-11-09.
    if class.to_lowercase().starts_with("gui.") && the_flag_of(name).is_some() {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let flag = the_flag_of(name).unwrap_or("enabled");
        return Some(match rt.desktop().gui_flag(number, flag, None) {
            Ok(value) => Ok(Value::Int(value)),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    match class.to_lowercase().as_str() {
        "array" => array::property(name, this, rt, line),
        "buffer" => buffer::property(name, this, rt, line),
        "enumerator" => enumerator::property(name, this, rt, line),
        "error" => error::property(name, this, rt, line),
        "file" => file::property(name, this, rt, line),
        "func" => func::property(name, this, rt, line),
        "hotkey" => hotkey::property(name, this, rt, line),
        "map" => map::property(name, this, rt, line),
        "gui" | "gui.control" | "gui.listview" | "gui.treeview" | "gui.statusbar" => {
            gui::property(class, name, this, rt, line)
        }
        "menu" | "menubar" => menu::property(name, this, rt, line),
        _ => None,
    }
}

/// A property of a class type that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    class: &str,
    name: &str,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    if class.eq_ignore_ascii_case("gui") && name.eq_ignore_ascii_case("title") {
        let text = value.to_text();
        return Some(match the_title_of_the_gui(rt, Some(&text), line) {
            Ok(_) => Ok(()),
            Err(failure) => Err(failure),
        });
    }
    if class.eq_ignore_ascii_case("gui.button") && name.eq_ignore_ascii_case("value") {
        return Some(Err(the_failure_with("Invalid usage.", "", line)));
    }
    if class.eq_ignore_ascii_case("gui.checkbox") && name.eq_ignore_ascii_case("value") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let text = value.to_text();
        let on = !(text.is_empty() || text == "0" || text.eq_ignore_ascii_case("false"));
        return Some(match rt.desktop().gui_checked(number, Some(on)) {
            Ok(_) => Ok(()),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // Writing a Link's `Value` is refused by v2 with "Invalid usage."; a
    // Hotkey's `Text` writes the second, independent string. Measured on the
    // binary of v2.0.19 on 2026-11-10.
    if class.eq_ignore_ascii_case("gui.link") && name.eq_ignore_ascii_case("value") {
        return Some(Err(the_failure_with("Invalid usage.", "", line)));
    }
    if class.eq_ignore_ascii_case("gui.hotkey") && name.eq_ignore_ascii_case("text") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let text = value.to_text();
        return Some(match rt.desktop().gui_second_value(number, Some(&text)) {
            Ok(_) => Ok(()),
            Err(failure) => Err(the_failure_with(&failure, "", line)),
        });
    }
    // Attaching a menu bar to the window: `Gui.Menu := MenuBarObject`.
    // Measured on the binary of v2.0.19 on 2026-11-10: the answer is nothing
    // the script reads, the menu bar shows at the top of the window, and a
    // value that is not a menu is refused. The items go to the helper as
    // JSON; choosing one is an event on the same channel as a click.
    if class.eq_ignore_ascii_case("gui") && name.eq_ignore_ascii_case("menu") {
        if !crate::classes::menu::is_a_menu(&value) {
            return Some(Err(the_failure_with(
                "Expected a MenuBar object.",
                "",
                line,
            )));
        }
        // The JSON of the bar, built by the menu module itself.
        let Some(Ok(Value::Text(json))) =
            crate::classes::menu::call("the-bar-json", &value, &[], rt, line)
        else {
            return Some(Ok(()));
        };
        // The bar is kept on the window under a reserved key, so that a
        // `menu:<name>` event from the helper can find the item and its
        // callback. Measured on the docs of v2.0.19 on 2026-11-10: the
        // callback of an item is called with (ItemName, ItemPos, MyMenu).
        //
        // The window is remembered here as well: the drain of the events
        // looks for the window that holds the bar, and a script that only
        // writes the bar never calls a method of the window.
        rt.remember_a_gui(this);
        if let Value::Object(window) = this {
            crate::runtime::put(
                &mut window.borrow_mut().properties,
                "on:menu",
                value.clone(),
            );
        }
        // The bar is drawn on the window, so the window has to be there --
        // the same rule the title and the controls follow.
        if !rt.desktop().gui_is_open() {
            if let Err(reason) = rt.desktop().gui_open("") {
                return Some(Err(the_failure_with(&reason, "", line)));
            }
        }
        return Some(match rt.desktop().gui_menu_bar(&json) {
            Ok(()) => Ok(()),
            Err(reason) => Err(the_failure_with(&reason, "", line)),
        });
    }
    // Setting `Value` of a Tab chooses the page. Measured on the binary of
    // v2.0.19 on 2026-11-10: a page that is not there is refused with
    // "Invalid value.", not clamped. The helper does the bounds check and the
    // transport wraps its answer, so the wording is unwrapped here -- v2 says
    // exactly "Invalid value.".
    if the_control_is_a_tab(class) && name.eq_ignore_ascii_case("value") {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let wanted = value.to_text();
        return Some(match rt.desktop().gui_value(number, Some(&wanted)) {
            Ok(_) => Ok(()),
            Err(reason) => {
                // The transport wraps the helper's answer; v2 says exactly
                // "Invalid value." for a page that is not there.
                let message = if reason.contains("Invalid value.") {
                    "Invalid value.".to_string()
                } else {
                    reason
                };
                Err(the_failure_with(&message, "", line))
            }
        });
    }
    // Setting `Value` of a list-shaped control chooses by the 1-based index,
    // and setting `Text` chooses by the item's text. Measured on the binary of
    // v2.0.19 on 2026-11-10: `ddl.Value := 2` leaves the second item chosen
    // and `ddl.Text := "gamma"` chooses the item with that text.
    if the_control_is_a_combo(class)
        && (name.eq_ignore_ascii_case("value") || name.eq_ignore_ascii_case("text"))
    {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let wanted = value.to_text();
        let by_text = name.eq_ignore_ascii_case("text");
        return Some(match rt
            .desktop()
            .gui_combo_select(number, &wanted, by_text)
        {
            Ok(_) => Ok(()),
            Err(failure) => Err(the_failure_with(&failure, "", line)),
        });
    }
    if class.to_lowercase().starts_with("gui.")
        && (name.eq_ignore_ascii_case("value") || name.eq_ignore_ascii_case("text"))
    {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let text = value.to_text();
        return Some(match rt.desktop().gui_value(number, Some(&text)) {
            Ok(_) => Ok(()),
            Err(failure) => Err(the_failure_with(&failure, "", line)),
        });
    }
    if class.to_lowercase().starts_with("gui.") && the_flag_of(name).is_some() {
        let number = the_number_of_a_control(this).unwrap_or(0);
        let flag = the_flag_of(name).unwrap_or("enabled");
        // v2 reads a written flag by its truth: nought and the empty text are
        // false, and anything else is true.
        let text = value.to_text();
        let on = !(text.is_empty() || text == "0" || text.eq_ignore_ascii_case("false"));
        return Some(match rt.desktop().gui_flag(number, flag, Some(on)) {
            Ok(_) => Ok(()),
            Err(failure) => Err(the_failure_with(&failure, "", line)),
        });
    }
    let written = match class.to_lowercase().as_str() {
        "array" => array::set_property(name, this, value, rt, line),
        "buffer" => buffer::set_property(name, this, value, rt, line),
        "error" => error::set_property(name, this, value, rt, line),
        "file" => file::set_property(name, this, value, rt, line),
        "func" => func::set_property(name, this, value, rt, line),
        "hotkey" => hotkey::set_property(name, this, value, rt, line),
        "map" => map::set_property(name, this, value, rt, line),
        "gui" | "gui.control" | "gui.listview" | "gui.treeview" | "gui.statusbar" => {
            gui::set_property(class, name, this, value, rt, line)
        }
        "menu" | "menubar" => menu::set_property(name, this, &value, rt, line),
        _ => None,
    };
    let what = the_name_of(
        class,
        &format!("{}.Set", the_spelling_of_the_property(class, name)),
    );
    written.map(|result| result.map_err(|failure| named(failure, &what)))
}

/// The item of an object: the `__Item` of the class.
///
/// `None` means the class has no `__Item` of its own.
pub(crate) fn item(
    class: &str,
    key: &Value,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let read = match class.to_lowercase().as_str() {
        "array" => array::item(key, this, rt, line),
        "file" => file::item(key, this, rt, line),
        "map" => map::item(key, this, rt, line),
        _ => None,
    };
    let what = the_name_of(class, "__Item.Get");
    read.map(|result| result.map_err(|failure| named(failure, &what)))
}

/// Write an item of an object of a class type.
///
/// `None` means the class has no way to write an item of its own.
pub(crate) fn set_item(
    class: &str,
    key: &Value,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    let written = match class {
        "array" => array::set_item(key, this, value, rt, line),
        "map" => map::set_item(key, this, value, rt, line),
        _ => None,
    };
    let what = the_name_of(class, "__Item.Set");
    written.map(|result| result.map_err(|failure| named(failure, &what)))
}

/// The names of v2 that a class type answers.
///
/// A name is a method of the class or a property of it, and the two are
/// told apart by the module that owns the class. The names are in lower
/// case, the way the script spells them.
pub fn names(class: &str) -> Vec<&'static str> {
    if error::is_a_class(class) {
        return error::names();
    }
    match class.to_lowercase().as_str() {
        "array" => array::names(),
        "buffer" => buffer::names(),
        "clipboardall" => THE_CLIPBOARD_ALL.to_vec(),
        "comvalue" => THE_COM_VALUE.to_vec(),
        "enumerator" => enumerator::names(),
        "error" => error::names(),
        "file" => file::names(),
        "func" => func::names(),
        "hotkey" => hotkey::names(),
        "map" => map::names(),
        "gui" => gui::names("gui"),
        "menu" | "gui.menu" => gui::names("gui.menu"),
        "gui.control" => gui::names("gui.control"),
        "gui.listview" => gui::names("gui.listview"),
        "gui.statusbar" => gui::names("gui.statusbar"),
        "gui.treeview" => gui::names("gui.treeview"),
        "inputhook" => inputhook::NAMES.to_vec(),
        "class" => THE_CLASS_VALUE.to_vec(),
        "object" => THE_BASE_OBJECT
            .iter()
            .chain(THE_BASE_OBJECT_OUTSIDE.iter())
            .copied()
            .collect(),
        _ => Vec::new(),
    }
}

/// Whether a class type answers the name.
///
/// The case of the name does not matter: the script spells a name the
/// way it likes.
pub fn carries(class: &str, name: &str) -> bool {
    let name = name.to_lowercase();
    names(class).iter().any(|carried| *carried == name)
}

/// The class types that this runtime does not carry.
///
/// The name of the class is a name of the dialect, and a script reads it
/// as the class. The members of it are not carried: the tree says so when
/// a script asks for one, and it does not say that the name is not there
/// -- the name is there, the tree does not carry it.
///
/// The base object is not one of them: the plain object of a script is
/// one, and the refusal of a member that is not there keeps the words of
/// the dialect.
const THE_CLASSES_THAT_ARE_NOT_CARRIED: [&str; 6] = [
    "any",
    "class",
    "clipboardall",
    "comobject",
    "comvalue",
    "inputhook",
];

/// The reason of a name of a class that this runtime does not carry.
pub(crate) fn the_reason_of_a_class_that_is_not_carried(class: &str) -> Option<&'static str> {
    let class = class.to_lowercase();
    if THE_CLASSES_THAT_ARE_NOT_CARRIED.contains(&class.as_str()) {
        return Some("this runtime does not carry that class yet.");
    }
    None
}
