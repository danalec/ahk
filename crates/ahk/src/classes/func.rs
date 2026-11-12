//! The `Func` class.
//!
//! A function is a value: it knows its name, how many parameters it
//! wants, and whether it is built in. `Func` is the class of it, and
//! the methods of the class are the ones that answer about a function.

use alloc::string::String;
use alloc::vec::Vec;

use crate::runtime::{
    BOUND, BOUND_TARGET, CLOSURE, METHOD_CLASS, METHOD_NAME, Route, Runtime, RuntimeFailure, Value,
};

/// The properties a function owns.
///
/// The name, the number of the parameters, whether the function is
/// built in and whether it takes a rest are in the table of the object.
const PROPERTIES: [&str; 5] = ["name", "minparams", "maxparams", "isbuiltin", "isvariadic"];

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
    // The members that the KeySharp extension adds: none of their names is
    // a method of the v2.
    if rt
        .dialect()
        .carries_the_member_of_the_extension("func", method)
    {
        return the_extension_answer(method, this, rt, line);
    }

    match method {
        // The call of the function the value holds. A method of a
        // class of the script is a reference to the method: the first
        // argument of the call is `this`.
        "call" => call_the_function(this, arguments, rt, line, Route::ThroughTheCall),
        // The values of the class that the function carries: whether the
        // parameter at the place that is asked for is optional or written
        // with the ByRef marker.
        //
        // Measured on the binary of v2.0.19 on 2026-11-09: `f(a, b?)` answers
        // `IsOptional(2)` one and `IsOptional(1)` zero; the parameter that
        // carries a default is optional too; a method counts its `this` as
        // the first parameter; and a place that the function does not have is
        // refused with "Parameter #1 of Func.Prototype.IsOptional is
        // invalid.".
        "isoptional" => Some(the_flag_of_the_parameter(this, arguments, true, line)),
        "isbyref" => Some(the_flag_of_the_parameter(this, arguments, false, line)),
        // The binding of a function to an object is a value of the class
        // `BoundFunc`, and the call of it calls the function with the object
        // as its first argument.
        //
        // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
        // `t.Method.Bind(t)` calls the method with the object, and the
        // `Abs.Bind(5)` calls the built-in with the object as its first
        // argument -- and the built-in takes one, so the call of it fails.
        "bind" => {
            let object = arguments
                .first()
                .cloned()
                .unwrap_or(Value::Text(String::new()));
            Some(Ok(rt.bound_function(object, this.clone())))
        }
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
    let Value::Object(object) = this else {
        return None;
    };
    // A function that is bound to an object: the object is the first
    // argument of the call of the function that is bound.
    let bound = object.borrow().value_of(BOUND);
    let target = object.borrow().value_of(BOUND_TARGET);
    if let (Some(bound), Some(target)) = (bound, target) {
        return Some(rt.call_a_bound_function(&bound, &target, arguments, line));
    }
    // A closure is called by the name the runtime keeps it under, and not
    // by the name the script sees: the name of a closure of v2 is empty.
    let kept = object
        .borrow()
        .value_of(CLOSURE)
        .map(|value| value.to_text());
    let name = match kept {
        Some(kept) => kept,
        None => object
            .borrow()
            .value_of("name")
            .map(|value| value.to_text())
            .unwrap_or_default(),
    };
    let reference = {
        let borrowed = object.borrow();
        match (
            borrowed.value_of(METHOD_CLASS),
            borrowed.value_of(METHOD_NAME),
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
                Route::ThroughTheCall => {
                    crate::classes::the_arity_failure_of_a_method(&class, &method, message, line)
                }
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
            return Some(
                match crate::classes::call(&class, &method, this, rest, rt, line) {
                    Some(result) => result,
                    None => Err(rt.no_method(this, &method, line)),
                },
            );
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

/// Whether the parameter at the place the call asks for answers the flag.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the place counts from
/// one, and a place that the function does not have is refused with
/// "Parameter #1 of Func.Prototype.IsOptional is invalid." -- the name of
/// the method that was asked is the one of the failure.
fn the_flag_of_the_parameter(
    this: &Value,
    arguments: &[Value],
    optional: bool,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Ok(Value::Int(0));
    };
    let flags = object.borrow().value_of(PARAMETERS);
    let place = arguments
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let Ok(place) = place.trim().parse::<usize>() else {
        return Ok(Value::Int(0));
    };
    let Some(Value::List(values)) = flags else {
        return Ok(Value::Int(0));
    };
    let Some(Value::Int(bits)) = values.get(place.saturating_sub(1)) else {
        let name = if optional { "IsOptional" } else { "IsByRef" };
        return Err(RuntimeFailure::Error(RuntimeError {
            message: format!("Parameter #1 of Func.Prototype.{name} is invalid."),
            line,
        }));
    };
    let answer = if optional {
        (bits & 2) != 0
    } else {
        (bits & 1) != 0
    };
    Ok(Value::Int(answer as i64))
}

/// The members the KeySharp extension adds to the `Func`.
///
/// The documentation of KeySharp says the three of them expose the
/// metadata of the function: whether it is a closure, whether it is the
/// reference of a method, and the names of its parameters.
fn the_extension_answer(
    method: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else {
        return None;
    };
    match method {
        "isclosure" => Some(Ok(Value::Int(
            object.borrow().value_of(crate::runtime::CLOSURE).is_some() as i64,
        ))),
        "ismethod" => Some(Ok(Value::Int(
            object
                .borrow()
                .value_of(crate::runtime::METHOD_CLASS)
                .is_some() as i64,
        ))),
        "params" => {
            let names: Vec<Value> = rt
                .the_names_of_the_parameters_of_a_function(this)
                .into_iter()
                .map(Value::Text)
                .collect();
            Some(crate::classes::array::construct(&names, rt, line))
        }
        _ => None,
    }
}
use crate::runtime::{PARAMETERS, RuntimeError};
use alloc::format;
