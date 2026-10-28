//! The `Buffer` class.
//!
//! A buffer is a block of memory. It is the one class of the runtime
//! that is not text.
//!
//! v2.0.19 gives a buffer three things: a size, an address, and a
//! constructor -- and nothing else. The reads and the writes of bytes
//! belong to the file, which reads into a buffer and writes out of one.
//! This dialect keeps the bytes of the buffer in the runtime, because
//! that is where a file reads them.
//!
//! The bytes live in the property table of the object, under one key,
//! so that the module owns the whole of the state of a buffer.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::object_with;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, expected_type};

/// The key the state of a buffer lives under.
///
/// The key has a space in it, so that a script cannot reach the inside
/// of a buffer by name.
const STATE: &str = "buffer state";

/// The class of the object that carries the state of a buffer.
const STATE_CLASS: &str = "buffer-state";

/// The properties of a buffer.
const PROPERTIES: [&str; 2] = ["size", "ptr"];

/// The methods of a buffer: the constructor, and the ones every object
/// has.
const METHODS: [&str; 10] = [
    "__new", "ownprops", "hasownprop", "defineprop", "deleteprop",
    "getownpropdesc", "hasprop", "hasmethod", "getmethod", "hasbase",
];

/// The state of a buffer: the bytes.
struct Buffer {
    bytes: Vec<u8>,
}

/// The object a `Buffer` makes.
/// The names a buffer answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let buffer = new(arguments, line)?;
    Ok(object_with("buffer", vec![(STATE, stored(&buffer))]))
}

/// A method of a `Buffer`.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let method = method.to_lowercase();
    match method.as_str() {
        "__new" => Some(new(arguments, line).map(|buffer| {
            restore(this, &buffer);
            Value::Int(buffer.bytes.len() as i64)
        })),
        // The methods of the base object are answered here too: the
        // runtime knows nothing of the class types of the runtime.
        _ => base_method(&method, this, arguments, line),
    }
}

/// A property of a `Buffer`.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "size" => Some(state(this, line).map(|buffer| Value::Int(buffer.bytes.len() as i64))),
        "ptr" => Some(address(this, line)),
        _ => None,
    }
}

/// A property of a `Buffer` that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "size" => Some(resize(this, value, line)),
        _ => None,
    }
}

/// The bytes of another buffer, for the class that writes into one.
pub(crate) fn raw_bytes(value: &Value, line: u32) -> Result<Vec<u8>, RuntimeFailure> {
    match value {
        Value::Object(_) => Ok(state(value, line)?.bytes),
        Value::Text(text) => Ok(text.as_bytes().to_vec()),
        other => Err(fail(expected_type(TypeName::Object, other.type_name()), line)),
    }
}

/// Put the bytes into a buffer.
///
/// The `File` class fills a buffer, so the module that owns a buffer
/// is the one that writes into it. The bytes go to the start of the
/// buffer: a buffer has no position of its own.
pub(crate) fn fill(value: &Value, bytes: &[u8], line: u32) -> Result<usize, RuntimeFailure> {
    let mut buffer = state(value, line)?;
    if bytes.len() > buffer.bytes.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    let written = bytes.len();
    buffer.bytes[..written].copy_from_slice(bytes);
    restore(value, &buffer);
    Ok(written)
}

/// How many bytes a buffer has.
pub(crate) fn size_of(value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    Ok(state(value, line)?.bytes.len() as i64)
}

// -- the state ------------------------------------------------------------------

/// The failure of a buffer, with the line the script was on.
fn fail(message: impl Into<String>, line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError { message: message.into(), line })
}

/// The whole number an argument holds.
///
/// A float is truncated, the way AutoHotkey truncates it.
fn whole(value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    match value {
        Value::Int(number) => Ok(*number),
        Value::Float(number) => Ok(*number as i64),
        other => Err(fail(expected_type(TypeName::Integer, other.type_name()), line)),
    }
}

/// The state of a buffer, as the object holds it.
fn state(this: &Value, line: u32) -> Result<Buffer, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a buffer.", line));
    };
    let inner = match object.borrow().properties.get(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a buffer.", line)),
    };
    let inner = inner.borrow();
    let bytes = match inner.properties.get("bytes") {
        Some(Value::List(items)) => items
            .iter()
            .map(|item| item.to_number().unwrap_or(0.0) as u8)
            .collect(),
        _ => Vec::new(),
    };
    Ok(Buffer { bytes })
}

/// The state of a buffer, as the property table holds it.
fn stored(buffer: &Buffer) -> Value {
    let bytes = Value::List(
        buffer
            .bytes
            .iter()
            .map(|byte| Value::Int(*byte as i64))
            .collect(),
    );
    object_with(STATE_CLASS, vec![("bytes", bytes)])
}

/// Put the state back into the object.
fn restore(this: &Value, buffer: &Buffer) {
    let Value::Object(object) = this else { return };
    object.borrow_mut().properties.insert(STATE.to_string(), stored(buffer));
}

/// A new buffer: the bytes, and the position at the start of them.
fn new(arguments: &[Value], line: u32) -> Result<Buffer, RuntimeFailure> {
    let size = match arguments.first() {
        Some(value) => whole(value, line)?,
        None => 0,
    };
    if size < 0 {
        return Err(fail("the size of a buffer cannot be negative.", line));
    }
    // The bytes of a new buffer are zero, unless the script asks for
    // another byte to fill them with.
    let fill = match arguments.get(1) {
        Some(value) => {
            let byte = whole(value, line)?;
            if !(0..=255).contains(&byte) {
                return Err(fail(
                    "the fill of a buffer is a byte: it must be between 0 and 255.",
                    line,
                ));
            }
            byte as u8
        }
        None => 0,
    };
    Ok(Buffer { bytes: vec![fill; size as usize] })
}

/// The address of the memory of the buffer.
///
/// The bytes of a buffer live in the runtime, so the address of a
/// buffer is the address of the storage the runtime keeps for them.
/// It is a real address, and it changes when the buffer is made of
/// another size, as v2 says of a buffer that is reallocated.
fn address(this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a buffer.", line));
    };
    let inner = match object.borrow().properties.get(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a buffer.", line)),
    };
    let address = alloc::rc::Rc::as_ptr(&inner) as usize as i64;
    Ok(Value::Int(address))
}

/// Make the buffer of another size, the way v2 does: the bytes that
/// stay are kept, and the bytes that come are zero.
fn resize(this: &Value, value: Value, line: u32) -> Result<(), RuntimeFailure> {
    let size = whole(&value, line)?;
    if size < 0 {
        return Err(fail("the size of a buffer cannot be negative.", line));
    }
    let mut buffer = state(this, line)?;
    buffer.bytes.resize(size as usize, 0);
    restore(this, &buffer);
    Ok(())
}

// -- the base object ----------------------------------------------------------

/// The methods of the base object, answered for a buffer.
///
/// The runtime answers them for the classes of the script, and it knows
/// nothing of the class types of the runtime: a buffer has no method of
/// the script, and the bytes of a buffer are not a property of it.
fn base_method(
    method: &str,
    this: &Value,
    arguments: &[Value],
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else { return None };
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError { message: message.to_string(), line })))
    };
    let name = arguments.first().map(|value| value.to_text()).unwrap_or_default();
    let key = name.to_lowercase();
    match method {
        "ownprops" => {
            let names: Vec<Value> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(name, _)| Value::Text(name.clone()))
                .collect();
            Some(Ok(Value::List(names)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str()) || own_property(object, &key)) as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(METHODS.contains(&key.as_str()) as i64))),
        // A buffer extends the base object, and nothing else.
        "hasbase" => Some(Ok(Value::Int((key == "object") as i64))),
        "getownpropdesc" => {
            let value = object.borrow().properties.get(&key).cloned();
            Some(Ok(match value {
                Some(value) => {
                    let mut table = BTreeMap::new();
                    table.insert("value".to_string(), value);
                    Value::Object(Rc::new(RefCell::new(Object {
                        class: "property".to_string(),
                        properties: table,
                    })))
                }
                None => Value::Object(Rc::new(RefCell::new(Object {
                    class: "property".to_string(),
                    properties: BTreeMap::new(),
                }))),
            }))
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of a buffer is not a property of it.");
            }
            let value = arguments.get(1).cloned().unwrap_or(Value::Text(String::new()));
            object.borrow_mut().properties.insert(key, value);
            Some(Ok(Value::Int(1)))
        }
        "deleteprop" => {
            if key == STATE {
                return Some(Ok(Value::Int(0)));
            }
            let gone = object.borrow_mut().properties.remove(&key);
            Some(Ok(Value::Int(gone.is_some() as i64)))
        }
        // A method of a class type of the runtime is not a value this
        // runtime carries.
        "getmethod" => failure("this runtime does not carry the method of a class type as a value."),
        _ => None,
    }
}

/// Whether the buffer has a property of that name.
///
/// The state of the buffer is not a property of it: it is the inside of
/// the buffer, and the script never wrote it.
fn own_property(object: &Rc<RefCell<Object>>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}
