//! The `Error` class, and the classes of v2 that derive from it.
//!
//! v2 throws an object of this class when something goes wrong, and the
//! script catches it. The object carries the message of the error, the
//! name of the thing that raised it -- `What` --, the detail that goes
//! with it -- `Extra` --, the file and the line of the script, and the
//! stack of the calls.
//!
//! The tree does not keep the stack: the script of the runner has no
//! file, and the stack of v2 names the file of every frame.
//!
//! The classes of the errors are a tree of their own. A property that is
//! not there is an `UnsetError`, a `MemberError` and a `PropertyError`
//! at the same time, and an index that is out of range is a `ValueError`
//! and an `IndexError`.
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::runtime::{Object, Runtime, RuntimeFailure, Value, put};

/// The classes of the errors of v2: the name in lower case, the name as
/// v2 spells it, and the class each one derives from.
///
/// The list is the one of the documentation of v2, and the tree of the
/// classes is the one that the binary has.
pub(crate) const CLASSES: [(&str, &str, &str); 14] = [
    ("error", "Error", ""),
    ("memoryerror", "MemoryError", "error"),
    ("oserror", "OSError", "error"),
    ("targeterror", "TargetError", "error"),
    ("timeouterror", "TimeoutError", "error"),
    ("typeerror", "TypeError", "error"),
    ("unseterror", "UnsetError", "error"),
    ("membererror", "MemberError", "unseterror"),
    ("propertyerror", "PropertyError", "membererror"),
    ("methoderror", "MethodError", "membererror"),
    ("unsetitemerror", "UnsetItemError", "unseterror"),
    ("valueerror", "ValueError", "error"),
    ("indexerror", "IndexError", "valueerror"),
    ("zerodivisionerror", "ZeroDivisionError", "error"),
];

/// The name of a class of the errors, the way v2 spells it.
///
/// `None` means that the name is not one of the classes of the errors.
pub(crate) fn the_spelling_of(class: &str) -> Option<String> {
    let class = class.to_lowercase();
    CLASSES
        .iter()
        .find(|(name, _, _)| *name == class)
        .map(|(_, spelled, _)| spelled.to_string())
}

/// The class an error class derives from.
///
/// `None` means that the class is not one of the errors, or that it is
/// the root of the tree.
pub(crate) fn the_base_of(class: &str) -> Option<&'static str> {
    let class = class.to_lowercase();
    CLASSES
        .iter()
        .find(|(name, _, _)| *name == class)
        .map(|(_, _, base)| *base)
        .filter(|base| !base.is_empty())
}

/// The class of a failure, the way v2 names it.
///
/// v2 has a class for every kind of failure: a property that is not
/// there is a `PropertyError`, a value of the wrong type is a
/// `TypeError`, an item that has no value is an `UnsetItemError`, and
/// so on. The tree decides the class by the message of the failure,
/// because the message of v2 says what the failure is. A failure that
/// the table below does not know is of the class `Error`.
pub(crate) fn the_class_of(message: &str) -> &'static str {
    if message == "Divide by zero." {
        return "ZeroDivisionError";
    }
    // The message of the machine is the code of the system and what it
    // says: "(2) The system cannot find the file specified."
    if message.starts_with('(') {
        return "OSError";
    }
    // Measured on the binary of v2.0.19 on 2026-11-09: a process of the
    // process family that is not there is a failure of the class
    // `TargetError` -- `ProcessGetName(999999)` answers it -- and the family
    // asks the desktop for the process, so the message is the one the
    // built-in gives.
    if message == "Target process not found." {
        return "TargetError";
    }
    // Measured on the binary of v2.0.19 on 2026-11-09: a window of the window
    // family that is not there is a failure of the class `TargetError` too --
    // `WinGetID("zzz-nothing")` answers "Target window not found." with the
    // title in the `Extra`.
    if message == "Target window not found." {
        return "TargetError";
    }
    if message.contains("has no property named") {
        return "PropertyError";
    }
    if message.contains("has no method named") {
        return "MethodError";
    }
    if message == "Item has no value." {
        return "UnsetItemError";
    }
    // Measured on the binary of v2.0.19 on 2026-10-31: a variable that has
    // no value is an `UnsetError`.
    if message.ends_with("has not been assigned a value.") {
        return "UnsetError";
    }
    if message == "Invalid index." {
        return "IndexError";
    }
    if message == "Invalid value." || message.ends_with(" is invalid.") {
        return "ValueError";
    }
    if message == "Invalid parameter(s)." {
        return "ValueError";
    }
    // A parameter that may not be left out in this case is a value that
    // is not valid.
    if message.ends_with(" must not be omitted in this case.") {
        return "ValueError";
    }
    // The options of a dialog that v2 does not know are a value that
    // is not valid.
    if message == "Invalid option." {
        return "ValueError";
    }
    // Measured on the binary of v2.0.19 on 2026-10-31: the dynamic
    // assignment that wants a global declaration is of the class `Error`,
    // and not of the class of the values that are of the wrong type.
    if message == "This dynamic assignment requires a \"global\" declaration." {
        return "Error";
    }
    if message.starts_with("Expected a ")
        || message.starts_with("Expected an ")
        || message.contains(" requires ")
        || message == "Value not enumerable."
        || message == "Type mismatch."
    {
        return "TypeError";
    }
    "Error"
}

/// Whether a name is one of the classes of the errors.
pub(crate) fn is_a_class(class: &str) -> bool {
    let class = class.to_lowercase();
    CLASSES.iter().any(|(name, _, _)| *name == class)
}

/// The properties an error carries.
const PROPERTIES: [&str; 6] = ["message", "what", "extra", "file", "line", "stack"];

/// The methods an error has: the constructor, and the ones every object
/// has.
const METHODS: [&str; 10] = [
    "__new",
    "ownprops",
    "hasownprop",
    "defineprop",
    "deleteprop",
    "getownpropdesc",
    "hasprop",
    "hasmethod",
    "getmethod",
    "hasbase",
];

/// The names an error answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// The object an `Error` makes.
///
/// `Error(Message, What, Extra)`: the message is what the error says,
/// `What` names the thing that raised it, and `Extra` is the detail.
/// The class of the error is the one of the name that was called, and a
/// message that is not given is the name of the class.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let message = arguments
        .first()
        .map(|value| value.to_text())
        .unwrap_or_else(|| the_spelling_of(class).unwrap_or_default());
    let what = arguments
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let extra = arguments
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    Ok(error_value_of(
        class,
        &message,
        &what,
        &extra,
        line,
        &rt.the_stack(),
    ))
}

/// The object of an error, with the properties of it.
///
/// The error of the runtime is an object of the class of the failure,
/// and the object that the script throws is one too.
/// The object of an error, of the class it was made of.
pub(crate) fn error_value_of(
    class: &str,
    message: &str,
    what: &str,
    extra: &str,
    line: u32,
    stack: &str,
) -> Value {
    let mut properties = BTreeMap::new();
    // Measured on the binary of v2.0.19 on 2026-10-31: the own properties
    // of an error are the ones that v2 spells: "Extra", "File", "Line",
    // "Message", "Stack" and "What".
    put(&mut properties, "Message", Value::Text(message.to_string()));
    put(&mut properties, "What", Value::Text(what.to_string()));
    put(&mut properties, "Extra", Value::Text(extra.to_string()));
    // The script of the runner has no file of its own.
    put(&mut properties, "File", Value::Text(String::new()));
    put(&mut properties, "Line", Value::Int(line as i64));
    // Measured on the binary of v2.0.19 on 2026-10-31: the stack names
    // every frame of the call that failed -- the file of it, the line,
    // the name of the frame and the source of the line -- and the last
    // line names the section that starts the script.
    put(&mut properties, "Stack", Value::Text(stack.to_string()));
    Value::Object(Rc::new(RefCell::new(Object {
        class: class.to_lowercase(),
        properties,
    })))
}

/// A method of an `Error`.
pub(crate) fn call(
    method: &str,
    _this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match method.to_lowercase().as_str() {
        "__new" => Some(construct("error", arguments, rt, line)),
        _ => None,
    }
}

/// A property of an `Error`.
///
/// The properties of an error live in the table of the object, and the
/// runtime reads them there.
pub(crate) fn property(
    _name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    None
}

/// A property of an `Error` that is written.
///
/// The properties of an error live in the table of the object, and the
/// runtime writes them there.
pub(crate) fn set_property(
    _name: &str,
    _this: &Value,
    _value: Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    None
}

/// The classes of the errors that the `ks` dialect adds: the name in
/// lower case, the name as KeySharp spells it, the class each one derives
/// from, and the message it carries when the caller gives it none.
///
/// The `ArgumentError` is the error of a call that left out a parameter or
/// passed one that was unset, and the `UnsupportedError` is the error of
/// an operation the platform of the machine cannot do. They are of the
/// `ks` dialect alone: the v2.0.19 has neither of them.
pub(crate) const KS_CLASSES: [(&str, &str, &str, &str); 2] = [
    (
        "argumenterror",
        "ArgumentError",
        "error",
        "A required parameter was omitted or unset.",
    ),
    ("unsupportederror", "UnsupportedError", "oserror", ""),
];

/// The message a class of the errors of the `ks` dialect carries when the
/// caller gives it none.
pub(crate) fn the_message_of_the_extension(class: &str) -> Option<&'static str> {
    let class = class.to_lowercase();
    KS_CLASSES
        .iter()
        .find(|(name, _, _, _)| *name == class)
        .map(|(_, _, _, message)| *message)
}

/// The object of an error of the `ks` dialect.
pub(crate) fn construct_the_extension(
    class: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let default = the_message_of_the_extension(class)?;
    let message = arguments
        .first()
        .map(|value| value.to_text())
        .unwrap_or_else(|| default.to_string());
    let what = arguments
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default();
    let extra = arguments
        .get(2)
        .map(|value| value.to_text())
        .unwrap_or_default();
    Some(Ok(error_value_of(
        &class.to_lowercase(),
        &message,
        &what,
        &extra,
        line,
        &rt.the_stack(),
    )))
}
