//! The typed numbers of the KeySharp dialect.
//!
//! KeySharp keeps a class for every width of number -- `Int8` up to
//! `Int64`, `UInt8` up to `UInt32`, `IntPtr`, `Float32` and `Float64` --
//! because a script that talks to a library of the machine has to say how
//! wide a field is. Each of them is a struct of one field, and the value
//! of the field is written with the width of it: the number is truncated
//! to the width, the way the C# of KeySharp truncates it when it writes
//! the bytes (`unchecked((byte)l)`), and read back with the sign of the
//! kind.
//!
//! This runtime keeps the value and not the memory of it: the `Ptr` of a
//! typed number refuses with the reason, because there is no address of a
//! struct to give. The value itself is the truth of it -- the width is the
//! width, and the number that comes out is the number that fits.
//!
//! The classes are of the `ks` dialect alone: the v2.0.19 has no typed
//! number, and the tree refuses the script that makes one there.

use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, TypeName, Value};

/// The width of a typed number, and the way it is read.
#[derive(Clone, Copy)]
pub(crate) enum Kind {
    /// A whole number with a sign, of the width of the bits.
    Signed(u8),
    /// A whole number without a sign, of the width of the bits.
    Unsigned(u8),
    /// A float of thirty-two bits.
    Float32,
    /// A float of sixty-four bits.
    Float64,
    /// The base of the typed numbers: a struct of no field.
    Struct,
}

/// The class of a typed number: the name as KeySharp spells it, the size
/// of the field in bytes, and the kind of it.
pub(crate) fn the_class_of(name: &str) -> Option<(&'static str, i64, Kind)> {
    let lowered = name.to_lowercase();
    Some(match lowered.as_str() {
        "int8" => ("Int8", 1, Kind::Signed(8)),
        "uint8" => ("UInt8", 1, Kind::Unsigned(8)),
        "int16" => ("Int16", 2, Kind::Signed(16)),
        "uint16" => ("UInt16", 2, Kind::Unsigned(16)),
        "int32" => ("Int32", 4, Kind::Signed(32)),
        "uint32" => ("UInt32", 4, Kind::Unsigned(32)),
        "int64" => ("Int64", 8, Kind::Signed(64)),
        "intptr" => ("IntPtr", 8, Kind::Signed(64)),
        "float32" => ("Float32", 4, Kind::Float32),
        "float64" => ("Float64", 8, Kind::Float64),
        "struct" => ("Struct", 0, Kind::Struct),
        _ => return None,
    })
}

/// Whether the name is one of the typed numbers.
pub(crate) fn is_a_class(name: &str) -> bool {
    the_class_of(name).is_some()
}

/// The names of the members of a typed number: the value of it and the
/// size of the field.
pub(crate) fn names() -> Vec<&'static str> {
    vec!["__value", "size"]
}

/// The object a typed number makes: the value of it, cut to the width.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let (spelled, size, kind) = the_class_of(class)?;
    let converted = if matches!(kind, Kind::Struct) {
        // The base of the typed numbers has no field of its own and no
        // value: the `Struct()` of KeySharp is the empty struct.
        Value::Unset
    } else {
        let value = arguments.first().cloned().unwrap_or(Value::Int(0));
        match the_value_of(kind, &value, spelled, line) {
            Ok(converted) => converted,
            Err(failure) => return Some(Err(failure)),
        }
    };
    let Value::Object(object) = crate::classes::object(spelled) else {
        return None;
    };
    object.borrow_mut().put(THE_VALUE, converted);
    object.borrow_mut().put(THE_SIZE, Value::Int(size));
    Some(Ok(Value::Object(object)))
}

/// A member of a typed number, as a property read.
pub(crate) fn property(
    class: &str,
    name: &str,
    this: &Value,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let (_, size, kind) = the_class_of(class)?;
    let lowered = name.to_lowercase();
    match lowered.as_str() {
        // The value of the field, the way the struct of one field holds it.
        "__value" => {
            if matches!(kind, Kind::Struct) {
                // The message of KeySharp: the base of the typed
                // numbers defines no value of its own.
                return Some(Err(RuntimeFailure::Error(RuntimeError {
                    message: "This struct does not define __Value.".to_string(),
                    line,
                })));
            }
            let Value::Object(object) = this else {
                return None;
            };
            let value = object.borrow().value_of(THE_VALUE).unwrap_or(Value::Unset);
            Some(Ok(value))
        }
        // The size of the field, in bytes.
        "size" => Some(Ok(Value::Int(size))),
        _ => None,
    }
}

/// The value of a typed number: the number of the caller, cut to the
/// width of the kind and read back with the sign of it.
fn the_value_of(
    kind: Kind,
    value: &Value,
    name: &str,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let number = value.to_number_strict().map_err(|_| {
        RuntimeFailure::Error(RuntimeError {
            message: crate::runtime::requires(1, name, TypeName::Number, value),
            line,
        })
    })?;
    // The number is cut to a whole number first: the cast of a float to an
    // integer saturates in Rust, and the cast of an integer to an integer
    // wraps -- and the cast of the C# wraps.
    let whole = number as i64;
    Ok(match kind {
        Kind::Float32 => Value::Float(number as f32 as f64),
        Kind::Float64 => Value::Float(number),
        // The truncation is the one of the C#: the number is cut to the
        // width of the field, and it wraps the way a cast wraps.
        Kind::Signed(bits) => {
            let cut = match bits {
                8 => whole as i8 as i64,
                16 => whole as i16 as i64,
                32 => whole as i32 as i64,
                _ => whole,
            };
            Value::Int(cut)
        }
        // The base of the typed numbers has no value of its own.
        Kind::Struct => Value::Unset,
        Kind::Unsigned(bits) => {
            let cut = match bits {
                8 => whole as u8 as i64,
                16 => whole as u16 as i64,
                32 => whole as u32 as i64,
                _ => whole as u64 as i64,
            };
            Value::Int(cut)
        }
    })
}

/// The key of the value of the field.
const THE_VALUE: &str = "\u{1}typed value";

/// The key of the size of the field.
const THE_SIZE: &str = "\u{1}typed size";
