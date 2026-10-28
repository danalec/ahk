//! The `Func` class.
//!
//! A function is a value: it knows its name, how many parameters it
//! wants, and whether it is built in. `Func` is the class of it, and
//! the methods of the class are the ones that answer about a function.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The properties a function owns.
///
/// The name, the number of the parameters and whether the function is
/// built in are in the table of the object.
const PROPERTIES: [&str; 3] = ["name", "minparams", "maxparams"];

/// The methods a function has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 9] = [
    "call", "isoptional", "isvariadic", "isbyref", "isbuiltin", "bind", "ptr", "size", "__new",
];

/// The names a function answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// The object a `Func` makes.
///
/// A function is not made: it is written in the script, or it is one of
/// the built-ins. There is nothing to build here.
pub(crate) fn construct(
    _arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    Err(RuntimeFailure::Error(RuntimeError {
        message: "a function is a value of the script; there is no way to make one here.".into(),
        line,
    }))
}

/// A method of a `Func`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else { return None };
    let name = object
        .borrow()
        .properties
        .get("name")
        .map(|value| value.to_text())
        .unwrap_or_default();
    let failure = |message: String| {
        Some(Err(RuntimeFailure::Error(RuntimeError { message, line })))
    };
    match method {
        // The constructor of the class: a function is not made, it is
        // written in the script or it is one of the built-ins.
        "__new" => failure(
            "a function is a value of the script; there is no way to make one here.".to_string(),
        ),
        // The call of the function the value holds.
        "call" => {
            let values = arguments.to_vec();
            Some(rt.call_named(&name, &values, line))
        }
        // The values of the class that the function carries.
        "isoptional" => Some(Ok(Value::Int(0))),
        "isvariadic" => Some(Ok(Value::Int(0))),
        "isbyref" => Some(Ok(Value::Int(0))),
        "isbuiltin" => Some(Ok(
            object
                .borrow()
                .properties
                .get("builtin")
                .cloned()
                .unwrap_or(Value::Int(0)),
        )),
        // The binding of a function to an object, and the address of
        // one, are things this runtime does not keep.
        "bind" => failure(
            "this runtime does not bind a function to an object.".to_string(),
        ),
        "ptr" => failure("this runtime does not know the address of a function.".to_string()),
        "size" => failure("this runtime does not know the size of a function.".to_string()),
        _ => None,
    }
}

/// A property of a `Func`.
///
/// The name, the number of the parameters and whether the function is
/// built in are in the table of the object, so the runtime reads them
/// there.
pub(crate) fn property(
    _name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    None
}

/// A property of a `Func` that is written.
pub(crate) fn set_property(
    _name: &str,
    _this: &Value,
    _value: Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    None
}
