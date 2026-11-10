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
//! The `NumGet` and the `NumPut` of v2 read a number out of a buffer
//! and write one into it. The bytes they touch are the bytes of the
//! buffer, so the module that owns them is the one that reads and
//! writes them.
//!
//! The bytes live in the property table of the object, under one key,
//! so that the module owns the whole of the state of a buffer.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::enumerator;
use crate::classes::object_with;
use crate::dll::NativeType;
use crate::runtime::{
    Detail, Object, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, expected_type, put,
};

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

/// A buffer of the bytes of a file.
///
/// The `FileRead` answers with a buffer when the script asks for the raw
/// bytes of the file.
pub(crate) fn of_bytes(bytes: Vec<u8>) -> Value {
    object_with("buffer", vec![(STATE, stored(&Buffer { bytes }))])
}

/// A method of a `Buffer`.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
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
        _ => base_method(&method, this, arguments, rt, line),
    }
}

/// A property of a `Buffer`.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "size" => Some(state(this, line).map(|buffer| Value::Int(buffer.bytes.len() as i64))),
        "ptr" => Some(match ptr_of(this, line) {
            Ok(address) => {
                // `StrGet` and `StrPut` are given this address back, so the
                // runtime remembers which buffer it names.
                rt.remember_a_buffer_at(address, this.clone());
                Ok(Value::Int(address))
            }
            Err(failure) => Err(failure),
        }),
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
        other => Err(fail(
            expected_type(TypeName::Object, other.type_name()),
            line,
        )),
    }
}

/// Put the bytes into a buffer.
///
/// The `File` class fills a buffer, so the module that owns a buffer
/// is the one that writes into it. The bytes go to the start of the
/// buffer: a buffer has no position of its own.
pub(crate) fn fill(value: &Value, bytes: &[u8], line: u32) -> Result<usize, RuntimeFailure> {
    write_at(value, 0, bytes, line)?;
    Ok(bytes.len())
}

/// How many bytes a buffer has.
pub(crate) fn size_of(value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    Ok(state(value, line)?.bytes.len() as i64)
}

/// The bytes of a buffer, for `StrGet` to decode.
pub(crate) fn bytes_of(value: &Value, line: u32) -> Result<Vec<u8>, RuntimeFailure> {
    Ok(state(value, line)?.bytes)
}

/// The address of the memory of the buffer.
///
/// The bytes of a buffer live in the runtime, so the address of a
/// buffer is the address of the storage the runtime keeps for them.
/// It is a real address, and it changes when the buffer is made of
/// another size, as v2 says of a buffer that is reallocated.
pub(crate) fn ptr_of(this: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a buffer.", line));
    };
    let inner = match object.borrow().value_of(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a buffer.", line)),
    };
    Ok(alloc::rc::Rc::as_ptr(&inner) as usize as i64)
}

/// The type of a number, out of the name the script writes.
///
/// The types of a number are the types of a native call, and the tree
/// keeps them in one table: `NumGet` and `NumPut` name them the way
/// `DllCall` does.
pub(crate) fn number_type(text: &str) -> Option<NativeType> {
    match text.trim().to_ascii_lowercase().as_str() {
        "uint" => Some(NativeType::UInt),
        "int" => Some(NativeType::Int),
        "int64" => Some(NativeType::Int64),
        "short" => Some(NativeType::Short),
        "ushort" => Some(NativeType::UShort),
        "char" => Some(NativeType::Char),
        "uchar" => Some(NativeType::UChar),
        "double" => Some(NativeType::Double),
        "float" => Some(NativeType::Float),
        "ptr" => Some(NativeType::Ptr),
        "uptr" => Some(NativeType::UPtr),
        _ => None,
    }
}

/// The number of a buffer, at an offset.
///
/// The type of the number says how wide it is, and the bytes of a
/// buffer are read here because this module owns them.
pub(crate) fn read_number(
    value: &Value,
    offset: i64,
    kind: NativeType,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let buffer = state(value, line)?;
    let start = start_of(offset, line)?;
    let end = start.saturating_add(kind.width());
    if end > buffer.bytes.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    Ok(number_from_bytes(&buffer.bytes[start..end], kind))
}

/// Put the numbers into a buffer, at an offset.
///
/// The numbers go out one after the other, the way v2 writes them, and
/// the answer is the address to the right of the last one of them.
pub(crate) fn write_numbers(
    value: &Value,
    offset: i64,
    numbers: &[(NativeType, Value)],
    line: u32,
) -> Result<i64, RuntimeFailure> {
    let mut bytes = Vec::new();
    for (kind, number) in numbers {
        bytes.extend_from_slice(&number_bytes(number, *kind));
    }
    write_at(value, offset, &bytes, line)?;
    // The answer of v2 is the address to the right of the last number:
    // the address of the buffer and the offset it stopped at.
    Ok(ptr_of(value, line)? + offset + bytes.len() as i64)
}

// -- the state ------------------------------------------------------------------

/// The failure of a buffer, with the line the script was on.
fn fail(message: impl Into<String>, line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: message.into(),
        line,
    })
}

/// The whole number an argument holds.
///
/// A float is truncated, the way AutoHotkey truncates it. A value that is
/// not a number is refused the way v2 refuses it, and the value that was
/// given goes in the `Extra` of the error.
fn whole(value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    match value {
        Value::Int(number) => Ok(*number),
        Value::Float(number) => Ok(*number as i64),
        other => Err(RuntimeFailure::Detailed(
            RuntimeError {
                message: expected_type(TypeName::Number, other.type_name()),
                line,
            },
            Detail {
                what: String::new(),
                extra: other.to_text(),
            },
        )),
    }
}

/// The whole number an argument of the constructor holds.
///
/// v2 names the constructor in the message, and the number of the
/// parameter is the number of the argument the script wrote.
fn whole_argument(value: &Value, index: usize, line: u32) -> Result<i64, RuntimeFailure> {
    match value {
        Value::Int(number) => Ok(*number),
        Value::Float(number) => Ok(*number as i64),
        other => Err(RuntimeFailure::Detailed(
            RuntimeError {
                message: crate::runtime::requires(
                    index + 1,
                    "Buffer.Prototype.__New",
                    TypeName::Number,
                    other,
                ),
                line,
            },
            Detail {
                what: "Buffer.Prototype.__New".to_string(),
                extra: other.to_text(),
            },
        )),
    }
}

/// The state of a buffer, as the object holds it.
fn state(this: &Value, line: u32) -> Result<Buffer, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a buffer.", line));
    };
    let inner = match object.borrow().value_of(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a buffer.", line)),
    };
    let inner = inner.borrow();
    let bytes = match inner.value_of("bytes") {
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
    put(&mut object.borrow_mut().properties, STATE, stored(buffer));
}

/// A new buffer: the bytes, and the position at the start of them.
fn new(arguments: &[Value], line: u32) -> Result<Buffer, RuntimeFailure> {
    let size = match arguments.first() {
        Some(value) => whole_argument(value, 0, line)?,
        None => 0,
    };
    if size < 0 {
        return Err(fail("Invalid value.", line));
    }
    // The bytes of a new buffer are zero, unless the script asks for
    // another byte to fill them with.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31: the fill is the byte
    // of the number, and a number that is not a byte is not refused: the
    // byte of it is the fill.
    let fill = match arguments.get(1) {
        Some(value) => whole_argument(value, 1, line)? as u8,
        None => 0,
    };
    Ok(Buffer {
        bytes: vec![fill; size as usize],
    })
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

/// Put the bytes into a buffer, at an offset.
///
/// The write of a number belongs to the module that owns the buffer,
/// the same way the read does.
fn write_at(value: &Value, offset: i64, bytes: &[u8], line: u32) -> Result<(), RuntimeFailure> {
    let mut buffer = state(value, line)?;
    let start = start_of(offset, line)?;
    let end = start.saturating_add(bytes.len());
    if end > buffer.bytes.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    buffer.bytes[start..end].copy_from_slice(bytes);
    restore(value, &buffer);
    Ok(())
}

/// The place in the bytes of a buffer that an offset names.
fn start_of(offset: i64, line: u32) -> Result<usize, RuntimeFailure> {
    if offset < 0 {
        return Err(fail("the offset of a number cannot be negative.", line));
    }
    match usize::try_from(offset) {
        Ok(start) => Ok(start),
        Err(_) => Err(fail("the bytes do not fit in the buffer.", line)),
    }
}

/// The bytes of a number, in the width its type asks for.
///
/// The numbers go out little-endian, the way the machines of x86 read
/// them, and a whole number is cut to the width of its type.
fn number_bytes(number: &Value, kind: NativeType) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(kind.width());
    match kind {
        NativeType::Float => {
            bytes.extend_from_slice(&(number.to_number().unwrap_or(0.0) as f32).to_le_bytes())
        }
        NativeType::Double => {
            bytes.extend_from_slice(&number.to_number().unwrap_or(0.0).to_le_bytes())
        }
        _ => {
            let whole = match number {
                Value::Int(number) => *number,
                other => other.to_number().unwrap_or(0.0) as i64,
            };
            match kind.width() {
                1 => bytes.push(whole as u8),
                2 => bytes.extend_from_slice(&(whole as u16).to_le_bytes()),
                4 => bytes.extend_from_slice(&(whole as u32).to_le_bytes()),
                _ => bytes.extend_from_slice(&whole.to_le_bytes()),
            }
        }
    }
    bytes
}

/// The number of the bytes, read in the width its type asks for.
///
/// The bytes come in little-endian, the way they went out, and the type
/// says whether the number is signed: the sign is in the last byte of
/// the width.
fn number_from_bytes(bytes: &[u8], kind: NativeType) -> Value {
    match kind {
        NativeType::Float => {
            Value::Float(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64)
        }
        NativeType::Double => {
            let mut whole = [0u8; 8];
            whole.copy_from_slice(&bytes[..8]);
            Value::Float(f64::from_le_bytes(whole))
        }
        _ => {
            let mut whole = 0i64;
            // The bytes come in little-endian, and the last byte of them
            // is the first of the number.
            for byte in bytes.iter().take(kind.width()).rev() {
                whole = (whole << 8) | *byte as i64;
            }
            let bits = kind.width() * 8;
            if bits < 64 && !kind.is_unsigned() {
                let sign = 1i64 << (bits - 1);
                if whole & sign != 0 {
                    whole -= sign << 1;
                }
            }
            Value::Int(whole)
        }
    }
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
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else {
        return None;
    };
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError {
            message: message.to_string(),
            line,
        })))
    };
    let name = arguments
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let key = name.to_lowercase();
    match method {
        "ownprops" => {
            // Measured on the binary of v2.0.19 on 2026-10-31: the own
            // properties of v2 are an enumerator, and it gives the name
            // when the caller wants one value.
            let pairs: Vec<(Value, Value)> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(_, property)| (Value::Text(property.name.clone()), property.value.clone()))
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheKey)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str())
                // The methods of the class are the properties of the value.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasProp("Get")` answers 1.
                || METHODS.contains(&key.as_str())
                || own_property(object, &key)
                // Every value carries its class, and the base object has
                // the property of it.
                || key == "__class"
                // Every class has an initialiser, and every object has a
                // base.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasProp("__Init")` and `m.HasProp("Base")` answer 1.
                || key == "__init"
                || key == "base") as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(
            (METHODS.contains(&key.as_str())
                // Every class has an initialiser.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasMethod("__Init")` answers 1.
                || key == "__init") as i64,
        ))),
        "getownpropdesc" => {
            let name = arguments
                .first()
                .map(|value| value.to_text())
                .unwrap_or_default();
            let value = object.borrow().value_of(&key);
            match value {
                Some(value) => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // descriptor that comes back is an object with the
                    // members of it.
                    let table = match rt.the_members_of_a_descriptor(&value) {
                        Some(members) => members,
                        None => {
                            let mut table = BTreeMap::new();
                            put(&mut table, "value", value);
                            table
                        }
                    };
                    Some(Ok(Value::Object(Rc::new(RefCell::new(Object {
                        class: "object".to_string(),
                        properties: table,
                    })))))
                }
                // The property is not there: v2 says so.
                None => Some(Err(rt.no_property(this, &name, line))),
            }
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of a buffer is not a property of it.");
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the second
            // parameter is a descriptor, and a value that is not one is
            // refused with the reason.
            let descriptor = match rt.the_descriptor(arguments.get(1), line) {
                Ok(descriptor) => descriptor,
                Err(failure) => return Some(Err(failure)),
            };
            put(&mut object.borrow_mut().properties, &name, descriptor);
            // The answer of v2 is the object itself.
            Some(Ok(this.clone()))
        }
        "deleteprop" => {
            if key == STATE {
                return Some(Ok(Value::Text(String::new())));
            }
            let gone = object
                .borrow_mut()
                .properties
                .remove(&key)
                .map(|property| property.value);
            // Measured on the binary of v2.0.19 on 2026-10-31: the answer
            // is the value of the property that is taken away, and an
            // empty text when there was none.
            Some(Ok(match gone {
                None => Value::Text(String::new()),
                Some(value) => match rt.the_value_of_the_descriptor(&value, this, &name, line) {
                    Ok(Some(value)) => value,
                    _ => value,
                },
            }))
        }
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
