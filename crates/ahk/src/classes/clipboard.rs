//! The `ClipboardAll` class.
//!
//! The documentation of v2 says what it is: an object that carries
//! everything that is on the clipboard -- the text, the pictures, the
//! formatting -- and that it is made out of the data of the clipboard, or
//! out of nothing at all, and the size of that data.
//!
//! Measured on the binary of v2.0.19 on 2026-10-31: the data is an
//! object or the address of the bytes, the size is optional when the
//! data is an object, and the address of nothing is not a valid value.
//!
//! The tree carries no binary clipboard data: the clipboard of the
//! desktop is a text. The call of the class with data fails with the
//! reason of the machine -- and the checks of the dialect come first,
//! the way v2 does them, because the number of the arguments and the
//! kind of them are the dialect's.

use alloc::format;
use alloc::string::{String, ToString};

use crate::classes::object;
use crate::runtime::{Detail, Runtime, RuntimeError, RuntimeFailure, TypeName, Value};

/// The name of the constructor of the class, the way v2 spells it.
const CONSTRUCTOR: &str = "ClipboardAll.Prototype.__New";

/// The object a `ClipboardAll` makes.
///
/// The state of the object is not carried yet: the members of the class
/// and the data of the clipboard are the work of another block.
pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    // The call of the class with no arguments reads the clipboard.
    if arguments.is_empty() {
        return Ok(object("clipboardall"));
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
        }
        _ => {}
    }
    // The binary data of the clipboard is not carried by this runtime.
    Err(RuntimeFailure::Error(RuntimeError {
        message: "the binary data of the clipboard is not carried by this runtime.".to_string(),
        line,
    }))
}
