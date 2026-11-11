//! The `PropRef` class of the KeySharp dialect.
//!
//! A `PropRef` is a virtual reference to a property of an object: the
//! target of it, the name of the property, and the arguments of the
//! access when it is an indexed one. KeySharp makes one with the
//! reference operator of the v2.1 -- `&obj.prop` -- and the tree makes
//! one with the call of the class: `PropRef(target, name, args...)`.
//!
//! The reference is bound to the target and to the name that the caller
//! named, the way it is in KeySharp: it does not rebind to whatever the
//! target answers later.

use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The names of the members of a `PropRef`: the target, the name and the
/// arguments of the access.
pub(crate) fn names() -> Vec<&'static str> {
    vec!["target", "name", "args"]
}

/// The object a `PropRef` makes.
pub(crate) fn construct(
    _class: &str,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The message of KeySharp: the target and the name are needed, and
    // without them there is no property to refer to.
    if arguments.len() < 2 {
        return Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "Too few parameters passed to function.".to_string(),
            line,
        })));
    }
    let target = arguments[0].clone();
    let name = arguments[1].to_text();
    let rest: Vec<Value> = arguments[2..].to_vec();
    let Value::Object(object) = crate::classes::object("propref") else {
        return None;
    };
    object.borrow_mut().put(THE_TARGET, target);
    object.borrow_mut().put(THE_NAME, Value::Text(name));
    // The arguments of the access are an array, the array of the runtime.
    let args = match crate::classes::array::construct(&rest, _rt, line) {
        Ok(args) => args,
        Err(failure) => return Some(Err(failure)),
    };
    object.borrow_mut().put(THE_ARGS, args);
    Some(Ok(Value::Object(object)))
}

/// A member of a `PropRef`, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    this: &Value,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The chain of the extension classes asks every module, and a
    // module that answers the members of another class takes them
    // with the unset value: the guard keeps the class its own.
    if !_class.eq_ignore_ascii_case("propref") {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    let answer = match name.to_lowercase().as_str() {
        // The object the property is of.
        "target" => object.borrow().value_of(THE_TARGET).unwrap_or(Value::Unset),
        // The name of the property.
        "name" => object.borrow().value_of(THE_NAME).unwrap_or(Value::Unset),
        // The arguments of the access, when it is an indexed one.
        "args" => object.borrow().value_of(THE_ARGS).unwrap_or(Value::Unset),
        _ => return None,
    };
    Some(Ok(answer))
}

/// The key of the target of the reference.
const THE_TARGET: &str = "\u{1}propref target";

/// The key of the name of the property.
const THE_NAME: &str = "\u{1}propref name";

/// The key of the arguments of the access.
const THE_ARGS: &str = "\u{1}propref args";
