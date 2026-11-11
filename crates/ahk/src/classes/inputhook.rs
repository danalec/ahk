//! The `InputHook` class of v2.
//!
//! The tree does not carry the class: it is a keyboard hook, and this
//! runtime has none. The names of it are here, because they are the
//! surface of v2, and a script that asks for one of them is refused
//! with the reason of the runtime.

/// The names of the `InputHook` class, read out of the binary.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the table of the
/// class is at 0x00125170, with a stride of forty bytes, and it holds
/// twenty-three names. The property with a setter is in it twice, the
/// getter and the setter.
use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

pub(crate) const NAMES: [&str; 23] = [
    "__new",
    "backspaceisundo",
    "casesensitive",
    "endkey",
    "endmods",
    "endreason",
    "findanywhere",
    "inprogress",
    "input",
    "keyopt",
    "match",
    "minsendlevel",
    "notifynontext",
    "onchar",
    "onend",
    "onkeydown",
    "onkeyup",
    "start",
    "stop",
    "timeout",
    "visiblenontext",
    "visibletext",
    "wait",
];

/// The members that the `ks` dialect adds to the `InputHook`.
///
/// The tree has no keyboard hook, and the class of the v2 is refused for
/// that. The dialect of KeySharp makes the object anyway, and the object
/// carries the settings of the hook: the eight of them are read and
/// written, and the hook itself is not installed -- the tree says so
/// when the script starts it.
pub(crate) const THE_MEMBERS: [&str; 8] = [
    "beforehotkeys",
    "bufferlengthmax",
    "endcharmode",
    "onmousedown",
    "onmousemove",
    "onmouseup",
    "transcribemodifiedkeys",
    "visiblemousemove",
];

/// The value a setting has when the script does not write another one.
///
/// The defaults are the ones of the source of KeySharp on 2026-11-11
/// (`InputType.cs`): the buffer of the input is 1023 characters long, the
/// mouse is visible when it moves, and the four flags are off.
pub(crate) fn the_default_of(member: &str) -> Value {
    match member {
        "bufferlengthmax" => Value::Int(1023),
        "visiblemousemove" => Value::Int(1),
        _ => Value::Int(0),
    }
}

/// The object the `ks` dialect makes: the settings of a hook that this
/// runtime cannot install.
pub(crate) fn construct_the_extension(
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let Value::Object(object) = crate::classes::object("inputhook") else {
        return Err(RuntimeFailure::Error(RuntimeError {
            message: "this runtime has no keyboard hook".to_string(),
            line: _line,
        }));
    };
    for member in THE_MEMBERS {
        let default = the_default_of(member);
        object.borrow_mut().put(member, default);
    }
    Ok(Value::Object(object))
}

/// A member of the `InputHook` of the `ks` dialect, as a property read.
pub(crate) fn property_the_extension(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let lowered = name.to_lowercase();
    if !THE_MEMBERS.contains(&lowered.as_str()) {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    let value = object
        .borrow()
        .value_of(&lowered)
        .unwrap_or_else(|| the_default_of(&lowered));
    Some(Ok(value))
}

/// A member of the `InputHook` of the `ks` dialect, as a property write.
pub(crate) fn set_property_the_extension(
    name: &str,
    this: &Value,
    value: Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    let lowered = name.to_lowercase();
    if !THE_MEMBERS.contains(&lowered.as_str()) {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    // The flags are flags, and the number of the buffer is a number.
    let written = match lowered.as_str() {
        "onmousedown" | "onmousemove" | "onmouseup" => value,
        "bufferlengthmax" => Value::Int(value.to_number_strict().unwrap_or(0.0) as i64),
        _ => Value::Int(if value.to_number_strict().unwrap_or(0.0) != 0.0 {
            1
        } else {
            0
        }),
    };
    object.borrow_mut().put(&lowered, written);
    Some(Ok(()))
}
