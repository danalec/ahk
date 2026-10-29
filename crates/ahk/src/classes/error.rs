//! The `Error` class.
//!
//! v2 throws an object of this class when something goes wrong, and the
//! script catches it. The object carries the message of the error, the
//! name of the thing that raised it -- `What` --, the detail that goes
//! with it -- `Extra` --, the file and the line of the script, and the
//! stack of the calls.
//!
//! The tree does not keep the stack: the script of the runner has no
//! file, and the stack of v2 names the file of every frame.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::runtime::{Object, Runtime, RuntimeFailure, Value};

/// The properties an error carries.
const PROPERTIES: [&str; 5] = ["message", "what", "extra", "file", "line"];

/// The methods an error has: the constructor, and the ones every object
/// has.
const METHODS: [&str; 10] = [
    "__new", "ownprops", "hasownprop", "defineprop", "deleteprop",
    "getownpropdesc", "hasprop", "hasmethod", "getmethod", "hasbase",
];

/// The names an error answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// The object an `Error` makes.
///
/// `Error(Message, What, Extra)`: the message is what the error says,
/// `What` names the thing that raised it, and `Extra` is the detail.
pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let message = arguments.first().map(|value| value.to_text()).unwrap_or_default();
    let what = arguments.get(1).map(|value| value.to_text()).unwrap_or_default();
    let extra = arguments.get(2).map(|value| value.to_text()).unwrap_or_default();
    Ok(error_value(&message, &what, &extra, line))
}

/// The object of an error, with the properties of it.
///
/// The error of the runtime is an object of this class, and the object
/// that the script throws is one too.
pub(crate) fn error_value(message: &str, what: &str, extra: &str, line: u32) -> Value {
    let mut properties = BTreeMap::new();
    properties.insert("message".to_string(), Value::Text(message.to_string()));
    properties.insert("what".to_string(), Value::Text(what.to_string()));
    properties.insert("extra".to_string(), Value::Text(extra.to_string()));
    // The script of the runner has no file of its own.
    properties.insert("file".to_string(), Value::Text(String::new()));
    properties.insert("line".to_string(), Value::Int(line as i64));
    Value::Object(Rc::new(RefCell::new(Object { class: "error".to_string(), properties })))
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
        "__new" => Some(construct(arguments, rt, line)),
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
