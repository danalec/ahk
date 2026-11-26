//! The `ComValue` and the `ComObject` classes.
//!
//! The shape of them is the one of the v2.0.31, read in its source
//! (`source/script_com.cpp` of the reference tree):
//!
//! * `ComValue(vt, value [, flags])` makes a value of the COM: the first
//!   argument is the kind of it -- the `VARTYPE` -- and the second one is
//!   the value;
//! * `ComObject.Call(class [, iid])` makes the object of a class of the
//!   machine: the class is a `ProgID` or the text of a `CLSID`, and the
//!   interface is the text of an `IID`;
//! * the property `Ptr` of a value is the number of it -- the address of
//!   the object, or the value of the `VARIANT`.
//!
//! The COM is a thing of Windows: the desktop is the one that makes the
//! object, and a desktop that has no COM says what it does not have.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::classes::object_with;
use crate::desktop::{ComValue as TheValue, MemberWhat};
use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, TypeName, Value, put};

/// The name of the constructor of the class, the way v2 spells it.
const CONSTRUCTOR: &str = "ComValue.Call";

/// The property that holds the state of the value.
const STATE: &str = "com value";

/// The class of the state of the value.
const STATE_CLASS: &str = "com-value";

/// The kind of a value that is nothing.
pub(crate) const THE_EMPTY: i64 = 0;
/// The kind of a value that is a wide text.
pub(crate) const THE_BSTR: i64 = 8;
/// The kind of a value that is an object that knows dispatch.
pub(crate) const THE_DISPATCH: i64 = 9;
/// The kind of a value that is an object that does not know dispatch.
pub(crate) const THE_UNKNOWN: i64 = 13;

/// The value of the COM that the class `ComValue` makes.
pub(crate) fn construct(
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let kind = the_kind_of(arguments, line)?;
    let flags = match arguments.get(2) {
        Some(flags) => flags.to_number().unwrap_or(0.0) as i64,
        None => 0,
    };
    let number = rt
        .desktop()
        .com_value_is_made(kind, the_value_of(kind, arguments.get(1)))
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(the_value(number.pointer, kind, flags))
}

/// The object of a class of the machine, that the class `ComObject` makes.
pub(crate) fn construct_the_object(
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let class = match arguments.first() {
        Some(class) => class.to_text(),
        None => String::new(),
    };
    let interface = arguments.get(1).map(|interface| interface.to_text());
    let object = rt
        .desktop()
        .com_object_of_the_class(&class, interface.as_deref())
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(the_value(object.pointer, object.kind, 0))
}

/// The kind of the value, the first argument of the call.
///
/// The number is the `VARTYPE` of v2, and a value that is not a number
/// is the refusal of v2: "Parameter #1 of ComValue.Call requires a
/// Number, but received a ...".
fn the_kind_of(arguments: &[Value], line: u32) -> Result<i64, RuntimeFailure> {
    match arguments.first() {
        Some(Value::Int(number)) => Ok(*number),
        Some(Value::Float(number)) => Ok(*number as i64),
        Some(other) => Err(RuntimeFailure::Error(RuntimeError {
            message: crate::runtime::requires(1, CONSTRUCTOR, TypeName::Number, other),
            line,
        })),
        None => Err(RuntimeFailure::Error(RuntimeError {
            message: "ComValue.Call wants the kind of the value.".into(),
            line,
        })),
    }
}

/// The value of the script, in the shape the desktop understands.
///
/// The shape of the value follows the kind of it: a text is a text, a
/// number is a number, and a pointer is a pointer.
fn the_value_of(kind: i64, value: Option<&Value>) -> TheValue {
    let Some(value) = value else {
        return TheValue::Number(0.0);
    };
    match kind {
        THE_BSTR => TheValue::Text(value.to_text()),
        THE_DISPATCH | THE_UNKNOWN => TheValue::Pointer(value.to_number().unwrap_or(0.0) as u64),
        _ => match value {
            Value::Text(text) => TheValue::Text(text.clone().into_string()),
            other => TheValue::Number(other.to_number().unwrap_or(0.0)),
        },
    }
}

/// The object of the COM that the script carries.
///
/// The state of the value -- its kind and its number -- is in a hidden
/// property, and the property `Ptr` of v2 is the reading of it.
pub(crate) fn the_value(pointer: u64, kind: i64, flags: i64) -> Value {
    let state = object_with(
        STATE_CLASS,
        vec![
            ("kind", Value::Int(kind)),
            ("pointer", Value::Int(pointer as i64)),
            ("flags", Value::Int(flags)),
        ],
    );
    object_with("comvalue", vec![(STATE, state)])
}

/// The state of the value of the COM, if the script carries one.
fn the_state_of(this: &Value) -> Option<crate::runtime::Object> {
    let Value::Object(object) = this else {
        return None;
    };
    let state = object.borrow().value_of(STATE)?;
    let Value::Object(state) = state else {
        return None;
    };
    Some(state.borrow().clone())
}

/// A number of a value of the COM.
fn the_number_of_state(state: &crate::runtime::Object, name: &str) -> Option<i64> {
    state
        .value_of(name)
        .map(|value| value.to_number().unwrap_or(0.0) as i64)
}

/// The number of the value of the COM, if the script carries one.
pub(crate) fn the_number_of(this: &Value) -> Option<i64> {
    the_number_of_state(&the_state_of(this)?, "pointer")
}

/// The kind of the value of the COM, if the script carries one.
pub(crate) fn the_kind_of_the_value(this: &Value) -> Option<i64> {
    the_number_of_state(&the_state_of(this)?, "kind")
}

/// The flags of the value of the COM, if the script carries one.
pub(crate) fn the_flags_of_the_value(this: &Value) -> Option<i64> {
    the_number_of_state(&the_state_of(this)?, "flags")
}

/// A number of the state of the value is written.
fn the_state_is_written(this: &Value, name: &str, number: i64) {
    let Value::Object(object) = this else {
        return;
    };
    let Some(Value::Object(state)) = object.borrow().value_of(STATE) else {
        return;
    };
    let mut state = state.borrow_mut();
    put(&mut state.properties, name, Value::Int(number));
}

/// The number of the value of the COM is written.
pub(crate) fn the_number_is_written(this: &Value, number: i64) {
    the_state_is_written(this, "pointer", number);
}

/// The flags of the value of the COM are written.
pub(crate) fn the_flags_are_written(this: &Value, flags: i64) {
    the_state_is_written(this, "flags", flags);
}

/// The property `Ptr` of a value of the COM.
///
/// The v2 reads it as the number of the value -- the address of the
/// object, or the value of the `VARIANT` -- and only when the kind of
/// the value is one of those.
pub(crate) fn property(
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if name.to_lowercase() == "ptr" {
        let kind = the_kind_of_the_value(this);
        let number = the_number_of(this);
        return Some(match (kind, number) {
            (Some(kind), Some(number))
                if kind == THE_DISPATCH
                    || kind == THE_UNKNOWN
                    || kind == THE_EMPTY
                    || kind & 0x6000 != 0 =>
            {
                Ok(Value::Int(number))
            }
            _ => Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Invalid usage.".into(),
                    line,
                },
                crate::runtime::Detail {
                    what: "ComValue.Prototype.Ptr.Get".into(),
                    extra: String::new(),
                },
            )),
        });
    }
    // The member of the object: the machine answers.
    let object = the_object_of(this)?;
    let answer = rt
        .desktop()
        .com_member(&object, MemberWhat::Get, name, &[])
        .map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        });
    Some(answer.map(the_value_of_answer))
}

/// The write of the property `Ptr` of a value of the COM.
///
/// The v2 takes a number in the place of the address of an object, and
/// refuses the rest.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    if name.to_lowercase() == "ptr" {
        let kind = the_kind_of_the_value(this).unwrap_or(THE_EMPTY);
        if kind != THE_DISPATCH && kind != THE_UNKNOWN {
            return Some(Err(RuntimeFailure::Error(RuntimeError {
                message: "Invalid usage.".into(),
                line,
            })));
        }
        the_number_is_written(this, value.to_number().unwrap_or(0.0) as i64);
        return Some(Ok(()));
    }
    // The member of the object: the machine writes it.
    let object = the_object_of(this)?;
    let answer = rt
        .desktop()
        .com_member(&object, MemberWhat::Set, name, &[the_com_value_of(&value)])
        .map(|_| ());
    Some(answer.map_err(|reason| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    }))
}

/// The call of a method of the object of the COM.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let object = the_object_of(this)?;
    let values: Vec<TheValue> = arguments.iter().map(the_com_value_of).collect();
    let answer = rt
        .desktop()
        .com_member(&object, MemberWhat::Call, method, &values);
    Some(answer.map(the_value_of_answer).map_err(|reason| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    }))
}

/// The object of the machine that the script carries, if it carries one.
///
/// Only a value whose kind is a real object of the COM -- a dispatch or
/// an unknown, possibly by reference -- is dereferenced. A value of
/// another kind (the `ComValue(1, 2)`) is not an object: its pointer is
/// a number of the VARIANT, and reading a member of it is a property
/// that is not there, not a dereference of the number.
pub(crate) fn the_object_of(this: &Value) -> Option<crate::desktop::ComObject> {
    let pointer = the_number_of(this)? as u64;
    let kind = the_kind_of_the_value(this)?;
    let base = kind & 0x0FFF;
    if base != THE_DISPATCH && base != THE_UNKNOWN {
        return None;
    }
    Some(crate::desktop::ComObject { pointer, kind })
}

/// The value of the script, in the shape the desktop takes.
pub(crate) fn the_com_value_of(value: &Value) -> TheValue {
    match value {
        Value::Text(text) => TheValue::Text(text.clone().into()),
        Value::Object(object) if object.borrow().class == "comvalue" => {
            TheValue::Pointer(the_number_of(value).unwrap_or(0) as u64)
        }
        other => TheValue::Number(other.to_number().unwrap_or(0.0)),
    }
}

/// The value of the script, out of the answer of the machine.
fn the_value_of_answer(answer: crate::desktop::ComAnswer) -> Value {
    match answer {
        crate::desktop::ComAnswer::Empty => Value::Text("".into()),
        crate::desktop::ComAnswer::Number(number) => {
            // The truncation is the one of the crate: the standard library
            // is not always there.
            if number == crate::math::trunc(number) && number.abs() < (i64::MAX as f64) {
                Value::Int(number as i64)
            } else {
                Value::Float(number)
            }
        }
        crate::desktop::ComAnswer::Text(text) => Value::Text(text.into()),
        crate::desktop::ComAnswer::Object { pointer, kind } => the_value(pointer, kind, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_value_of_the_com_carries_its_number_and_its_kind() {
        let value = the_value(1234, THE_DISPATCH, 0);
        assert_eq!(the_number_of(&value), Some(1234));
        assert_eq!(the_kind_of_the_value(&value), Some(THE_DISPATCH));
        assert_eq!(the_flags_of_the_value(&value), Some(0));
    }

    #[test]
    fn the_number_of_the_com_is_written() {
        let value = the_value(1234, THE_DISPATCH, 0);
        the_number_is_written(&value, 4321);
        assert_eq!(the_number_of(&value), Some(4321));
    }
}
