//! The `Func` class.
//!
//! A function is a value: it knows its name, how many parameters it
//! wants, and whether it is built in. `Func` is the class of it, and
//! the methods of the class are the ones that answer about a function.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{CLOSURE, Route, Runtime, RuntimeError, RuntimeFailure, Value, METHOD_CLASS, METHOD_NAME};

/// The properties a function owns.
///
/// The name, the number of the parameters, whether the function is
/// built in and whether it takes a rest are in the table of the object.
const PROPERTIES: [&str; 5] = [
    "name",
    "minparams",
    "maxparams",
    "isbuiltin",
    "isvariadic",
];

/// The methods a function has: the ones of the class.
///
/// The binary of v2.0.19 has no other: a function has no address and no
/// size, and it is not made -- it is written in the script, or it is one
/// of the built-ins.
const METHODS: [&str; 4] = ["call", "isoptional", "isbyref", "bind"];

/// The names a function answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// A method of a `Func`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(_) = this else { return None };
    let failure = |message: String| {
        Some(Err(RuntimeFailure::Error(RuntimeError { message, line })))
    };
    match method {
        // The call of the function the value holds. A method of a
        // class of the script is a reference to the method: the first
        // argument of the call is `this`.
        "call" => call_the_function(this, arguments, rt, line, Route::ThroughTheCall),
        // The values of the class that the function carries.
        "isoptional" => Some(Ok(Value::Int(0))),
        "isbyref" => Some(Ok(Value::Int(0))),
        // The binding of a function to an object is a thing this
        // runtime does not keep.
        "bind" => failure(
            "this runtime does not bind a function to an object.".to_string(),
        ),
        _ => None,
    }
}

/// The call of a value of a function.
///
/// The value is called, and not the `Call` method of it: v2 tells the
/// two apart in the failure of the number of the arguments.
pub(crate) fn call_the_value(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    call_the_function(this, arguments, rt, line, Route::ThroughValue)
}

/// The call of the function that a value holds.
///
/// The route says whether the value was called or its `Call` method was:
/// v2 names the method in the failure of the number of the arguments,
/// and the call of the value itself carries no name.
fn call_the_function(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
    route: Route,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else { return None };
    // A closure is called by the name the runtime keeps it under, and not
    // by the name the script sees: the name of a closure of v2 is empty.
    let kept = object.borrow().properties.get(CLOSURE).map(|value| value.to_text());
    let name = match kept {
        Some(kept) => kept,
        None => object
            .borrow()
            .properties
            .get("name")
            .map(|value| value.to_text())
            .unwrap_or_default(),
    };
    let reference = {
        let borrowed = object.borrow();
        match (
            borrowed.properties.get(METHOD_CLASS),
            borrowed.properties.get(METHOD_NAME),
        ) {
            (Some(class), Some(method)) => Some((class.to_text(), method.to_text())),
            _ => None,
        }
    };
    if let Some((class, method)) = reference {
        // A method of a class type of the runtime: the object is
        // the first argument of the call, and the method is the
        // one the class answers.
        if crate::classes::is_type(&class) {
            // The number of the arguments is checked here: the call
            // goes through a value, and the object is one of the
            // arguments of it.
            let failure = |message: &str| match route {
                Route::ThroughTheCall => crate::classes::the_arity_failure_of_a_method(
                    &class, &method, message, line,
                ),
                _ => crate::classes::the_arity_failure(&class, &method, message, line),
            };
            if let Some((min, max)) = crate::classes::the_arity_of(&class, &method) {
                let count = arguments.len() as i64;
                if count < min + 1 || (max >= 0 && count > max + 1) {
                    let message = if count < min + 1 {
                        "Too few parameters passed to function."
                    } else {
                        "Too many parameters passed to function."
                    };
                    return Some(Err(failure(message)));
                }
            }
            let Some((this, rest)) = arguments.split_first() else {
                return Some(Err(failure("Too few parameters passed to function.")));
            };
            return Some(match crate::classes::call(&class, &method, this, rest, rt, line) {
                Some(result) => result,
                None => Err(rt.no_method(this, &method, line)),
            });
        }
        return Some(rt.call_a_method_reference(&class, &method, arguments, line));
    }
    Some(rt.call_named(&name, arguments, line, route))
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
