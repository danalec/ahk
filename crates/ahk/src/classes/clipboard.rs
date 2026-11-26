//! The `ClipboardAll` class.
//!
//! The documentation of v2 says what it is: an object that carries
//! everything that is on the clipboard -- the text, the pictures, the
//! formatting -- and that it is made out of the data of the clipboard, or
//! out of nothing at all, and the size of that data.
//!
//! Measured on the binary of v2.0.19: the data is an
//! object or the address of the bytes, the size is optional when the
//! data is an object, and the address of nothing is not a valid value.
//!
//! The tree carries the binary clipboard: the call with no arguments
//! captures every format of the clipboard of the desktop, and the call
//! with a buffer copies the bytes of it. The data as an address of the
//! memory is not read -- the checks of the dialect come first, the way
//! v2 does them, because the number of the arguments and the kind of
//! them are the dialect's.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::classes::object_with;
use crate::runtime::{Detail, Runtime, RuntimeError, RuntimeFailure, SharedCell, TypeName, Value};

/// The key the bytes of the clipboard live under in the object.
const STATE: &str = "clipboard state";

/// The name of the constructor of the class, the way v2 spells it.
const CONSTRUCTOR: &str = "ClipboardAll.Prototype.__New";

/// The object a `ClipboardAll` makes.
///
/// With no arguments it captures the whole clipboard; with a buffer it
/// copies the bytes of it. The `Size` answers the number of bytes.
pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    // The call of the class with no arguments reads the whole clipboard.
    if arguments.is_empty() {
        let bytes = _rt.desktop().clipboard_read_all().map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        })?;
        return Ok(object_with("clipboardall", vec![(STATE, stored(&bytes))]));
    }
    // The data of the clipboard is checked the way v2 checks it: the
    // dialect's check comes before the reason of the machine.
    let data = &arguments[0];
    match data {
        Value::Text(_) => {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: crate::runtime::requires(1, CONSTRUCTOR, TypeName::Number, data),
                    line,
                },
                Detail {
                    what: CONSTRUCTOR.to_string(),
                    extra: data.to_text(),
                },
            ));
        }
        Value::Int(_) | Value::Float(_) => {
            // The address of the bytes. The address of nothing is not a
            // valid value, and the size of the data is needed when the
            // data is an address.
            let address = data.to_number().unwrap_or(0.0);
            if address == 0.0 || crate::runtime::fract(address) != 0.0 {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!("Parameter #1 of {CONSTRUCTOR} is invalid."),
                        line,
                    },
                    Detail {
                        what: CONSTRUCTOR.to_string(),
                        extra: data.to_text(),
                    },
                ));
            }
            if arguments.len() < 2 {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!(
                            "Parameter #2 of {CONSTRUCTOR} must not be omitted in this case."
                        ),
                        line,
                    },
                    Detail {
                        what: CONSTRUCTOR.to_string(),
                        extra: String::new(),
                    },
                ));
            }
            // The size is the number of bytes: a whole number that is not
            // negative. The bytes are read out of the memory of the
            // process, and the read of a bad address is a reason.
            let size_value = &arguments[1];
            let size = size_value.to_number().unwrap_or(-1.0);
            if size < 0.0 {
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "Out of memory.".to_string(),
                    line,
                }));
            }
            if crate::runtime::fract(size) != 0.0 {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!("Parameter #2 of {CONSTRUCTOR} is invalid."),
                        line,
                    },
                    Detail {
                        what: CONSTRUCTOR.to_string(),
                        extra: size_value.to_text(),
                    },
                ));
            }
            let bytes = _rt
                .desktop()
                .read_memory(address as i64, size as i64)
                .map_err(|reason| {
                    RuntimeFailure::Error(RuntimeError {
                        message: reason,
                        line,
                    })
                })?;
            return Ok(object_with("clipboardall", vec![(STATE, stored(&bytes))]));
        }
        Value::Object(object) => {
            // A buffer carries the bytes: the ClipboardAll copies them,
            // the way the v2 does. Measured on the binary of v2.0.31 `ClipboardAll(buf)` answers a ClipboardAll whose
            // Size is the size of the buffer, and `ClipboardAll(buf, n)`
            // takes n bytes.
            if object.borrow().class.eq_ignore_ascii_case("buffer") {
                let mut bytes = crate::classes::buffer::raw_bytes(data, line)?;
                if let Some(size_value) = arguments.get(1) {
                    if !matches!(size_value, Value::Unset) {
                        let size = size_value.to_number().unwrap_or(-1.0);
                        if size < 0.0 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "Out of memory.".to_string(),
                                line,
                            }));
                        }
                        if crate::runtime::fract(size) != 0.0 {
                            return Err(RuntimeFailure::Detailed(
                                RuntimeError {
                                    message: format!(
                                        "Parameter #2 of {CONSTRUCTOR} is invalid."
                                    ),
                                    line,
                                },
                                Detail {
                                    what: CONSTRUCTOR.to_string(),
                                    extra: size_value.to_text(),
                                },
                            ));
                        }
                        // The size beyond the bytes of the buffer is filled
                        // with zeros: the memory beyond the buffer is not
                        // the tree's to read.
                        bytes.resize(size as usize, 0);
                    }
                }
                return Ok(object_with("clipboardall", vec![(STATE, stored(&bytes))]));
            }
        }
        _ => {}
    }
    // The v2 reads the `Ptr` of the data: a value that has none is refused
    // with the words of the dialect. Measured on the binary of v2.0.31 `ClipboardAll([])` answers 'This value of type "Array"
    // has no property named "Ptr".'.
    Err(RuntimeFailure::Error(RuntimeError {
        message: _rt.the_message_of_no_property(data, "Ptr", line),
        line,
    }))
}
/// A property of a `ClipboardAll`.
///
/// The `Size` is the number of bytes of the capture. `None` means the
/// name is not a property of the class.
pub(crate) fn property(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "size" | "length" => Some(match the_bytes(this) {
            Some(bytes) => Ok(Value::Int(bytes.len() as i64)),
            None => Err(RuntimeFailure::Error(RuntimeError {
                message: "the clipboard data is not there.".to_string(),
                line,
            })),
        }),
        // The `Ptr` is the address of the data of the capture, the way the
        // `Ptr` of a buffer is the address of its storage. Measured on the
        // binary of v2.0.19: `ClipboardAll().Ptr` answers an Integer.
        "ptr" => Some(match the_ptr_of(this) {
            Some(address) => Ok(Value::Int(address)),
            None => Err(RuntimeFailure::Error(RuntimeError {
                message: "the clipboard data is not there.".to_string(),
                line,
            })),
        }),
        _ => None,
    }
}

/// The address of the data of a `ClipboardAll`.
///
/// The bytes live in the state the runtime keeps, so the address is the
/// one of that storage, a real number, the way the buffer answers its own.
fn the_ptr_of(this: &Value) -> Option<i64> {
    let Value::Object(object) = this else {
        return None;
    };
    let object = object.borrow();
    let state = object.properties.get(STATE)?;
    let Value::Object(state_obj) = &state.value else {
        return None;
    };
    Some(SharedCell::as_ptr(state_obj) as usize as i64)
}

/// The bytes of the capture, stored as a list of the bytes.
fn stored(bytes: &[u8]) -> Value {
    let list = Value::List(bytes.iter().map(|byte| Value::Int(*byte as i64)).collect());
    object_with("clipboard-state", vec![("bytes", list)])
}

/// The bytes of a `ClipboardAll`, for the restore of the clipboard.
///
/// `None` means the object carries no capture.
pub(crate) fn the_bytes(this: &Value) -> Option<Vec<u8>> {
    let Value::Object(object) = this else {
        return None;
    };
    let object = object.borrow();
    let state = object.properties.get(STATE)?;
    let Value::Object(state_obj) = &state.value else {
        return None;
    };
    let state_obj = state_obj.borrow();
    let bytes = state_obj.properties.get("bytes")?;
    let Value::List(list) = &bytes.value else {
        return None;
    };
    Some(
        list.iter()
            .map(|value| value.to_number().unwrap_or(0.0) as u8)
            .collect(),
    )
}
