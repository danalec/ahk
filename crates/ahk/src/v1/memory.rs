//! The memory of the v1: the `NumPut`, the `NumGet` and the
//! `VarSetCapacity` read and write the real bytes of a variable or of an
//! address, the way the binary does.
//!
//! The target of a `NumPut`/`NumGet` is a variable or an address: a bare
//! variable name answers the address of its memory, and an expression
//! answers the number that is the address.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::dll::NativeType;
use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The address of the target of a `NumPut`/`NumGet`.
///
/// The target arrives as a reference: a bare variable name is the memory
/// of that variable, and anything else is an expression whose number is
/// the address. A number that reaches here directly is an address too.
fn the_target_address(
    rt: &mut Runtime<'_>,
    target: &Value,
    line: u32,
) -> Result<i64, RuntimeError> {
    match target {
        Value::Reference(reference) => {
            if crate::v1::is_the_name_of_a_variable(&reference.name) {
                Ok(rt.the_variable_address(reference))
            } else {
                let value = rt
                    .the_value_of_the_expression_text(&reference.name, line)
                    .map_err(|failure| match failure {
                        RuntimeFailure::Error(error) => error,
                        RuntimeFailure::Detailed(error, _) => error,
                        _ => RuntimeError {
                            message: "the target of the memory is not an address.".to_string(),
                            line,
                        },
                    })?;
                Ok(value.to_number().unwrap_or(0.0) as i64)
            }
        }
        Value::Int(number) => Ok(*number),
        Value::Float(number) => Ok(*number as i64),
        Value::Text(text) => Ok(text.trim().parse::<i64>().unwrap_or(0)),
        _ => Err(RuntimeError {
            message: "the target of a NumPut/NumGet is a variable or an address.".to_string(),
            line,
        }),
    }
}

/// The offset and the type of a `NumPut`/`NumGet`.
///
/// The argument that is a type name is the type and the offset is zero;
/// the argument that is a number is the offset, and the type is the one
/// that comes after it. The type that is not there is the `UPtr`.
fn the_offset_and_type(values: &[Value], start: usize) -> (i64, NativeType) {
    let mut offset = 0i64;
    let mut kind = NativeType::UPtr;
    if let Some(value) = values.get(start) {
        let text = value.to_text();
        if let Some(named) = crate::classes::buffer::number_type(&text) {
            kind = named;
        } else {
            offset = value.to_number().unwrap_or(0.0) as i64;
            if let Some(named) = values
                .get(start + 1)
                .and_then(|value| crate::classes::buffer::number_type(&value.to_text()))
            {
                kind = named;
            }
        }
    }
    (offset, kind)
}

/// `NumPut(Number, VarOrAddress [, Offset, Type])`: the number is written
/// into the memory, and the answer is the address to the right of it.
pub(crate) fn num_put(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let number = values.first().cloned().unwrap_or(Value::Int(0));
    // The number of the v1 knows the hexadecimal: the `NumPut(0x83, buf, 0,
    // "UInt")` writes the 131.
    //
    // Measured on the binary of AutoHotkey v1.1.37.02: the
    // `NumPut(0x00000083, buf, 4, "UInt")` writes the 131, and the tree
    // wrote the zero.
    let number = match &number {
        Value::Text(text) => crate::v1::numbers::the_number_of_a_text(text)
            .unwrap_or_else(|| number.clone()),
        _ => number,
    };
    let Some(target) = values.get(1) else {
        return Err(RuntimeError {
            message: "NumPut requires a target.".to_string(),
            line,
        });
    };
    let address = the_target_address(rt, target, line)?;
    let (offset, kind) = the_offset_and_type(values, 2);
    let bytes = crate::classes::buffer::number_bytes(&number, kind);
    let at = address.saturating_add(offset);
    if at == 0 {
        return Err(RuntimeError {
            message: "NumPut: the address is null.".to_string(),
            line,
        });
    }
    // The write is safe: the memory of the script is written out of the
    // block of it, and the memory of the machine is written by the desktop,
    // which says that it did not when the address is not there.
    //
    // Measured on the binary of AutoHotkey v1.1.37.02: the `NumPut` of an
    // address that is not there does not crash the binary.
    let in_the_script = rt.the_bytes_are_written_at_an_address(at, &bytes);
    if in_the_script.is_none()
        && !rt.the_bytes_are_written_at_a_text(at, &bytes)
        && !rt.desktop().the_memory_is_written(at as u64, &bytes)
    {
        return Err(RuntimeError {
            message: "NumPut: the address cannot be written.".to_string(),
            line,
        });
    }
    Ok(Value::Int(at + bytes.len() as i64))
}

/// `NumGet(VarOrAddress [, Offset, Type])`: the number read from the
/// memory.
pub(crate) fn num_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(target) = values.first() else {
        return Err(RuntimeError {
            message: "NumGet requires a target.".to_string(),
            line,
        });
    };
    let address = the_target_address(rt, target, line)?;
    let (offset, kind) = the_offset_and_type(values, 1);
    let at = address.saturating_add(offset);
    if at == 0 {
        return Err(RuntimeError {
            message: "NumGet: the address is null.".to_string(),
            line,
        });
    }
    let width = kind.width();
    // The read is safe: the memory of the script is read out of the block
    // of it, and the memory of the machine is read by the desktop, which
    // says that it cannot when the address is not there.
    //
    // Measured on the binary of AutoHotkey v1.1.37.02: the `NumGet` of an
    // address that is not there answers the empty text, and the binary
    // does not crash. The tree read the memory without asking, and the
    // process died with it.
    let mut bytes = match rt.the_bytes_at_an_address(at) {
        Some(bytes) => bytes,
        None => match rt.desktop().read_memory(at, width as i64) {
            Ok(bytes) => bytes,
            Err(_) => return Ok(Value::Text("".into())),
        },
    };
    if bytes.len() < width {
        bytes.resize(width, 0);
    }
    Ok(crate::classes::buffer::number_from_bytes(&bytes, kind))
}

/// `VarSetCapacity(Var [, Capacity, FillByte])`: the memory of a variable
/// is allocated, filled, or freed; the answer is the capacity.
pub(crate) fn var_set_capacity(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(Value::Reference(reference)) = values.first() else {
        return Ok(Value::Int(0));
    };
    // The capacity that is not asked for answers the one the variable has.
    let Some(capacity_value) = values.get(1) else {
        return Ok(Value::Int(rt.string_capacity(reference)));
    };
    let capacity = capacity_value.to_number().unwrap_or(0.0) as i64;
    let fill = values
        .get(2)
        .map(|value| value.to_number().unwrap_or(0.0) as u8)
        .unwrap_or(0);
    if capacity < 0 {
        if capacity == -1 {
            // The free of the buffer: the answer is zero.
            rt.the_capacity_buffer(reference, -1, 0);
            return Ok(Value::Int(0));
        }
        // A capacity that is negative and is not minus one is a failure of
        // the machine, and the v1 says that the memory is gone.
        //
        // Measured on the binary of v1.1.37.02: the `VarSetCapacity(x, -5)`
        // of a script is a runtime error: "Out of memory.".
        return Err(RuntimeError {
            message: "Out of memory.".to_string(),
            line,
        });
    }
    let address = rt.the_capacity_buffer(reference, capacity, fill);
    // The variable holds the text of the buffer: the bytes that the fill
    // put there, and the text of the v1 is of two bytes a character.
    //
    // Measured on the binary of v1.1.37.02: the `VarSetCapacity(x, 10, 65)`
    // of a script leaves a variable of five characters, and the
    // `VarSetCapacity(x, 10)` leaves a variable of no character.
    let text = match rt.the_bytes_at_an_address(address) {
        Some(bytes) => crate::dll::decode(&bytes, crate::dll::NativeType::WStr),
        None => String::new(),
    };
    rt.write_reference(values.first().expect("the reference"), Value::Text(text.into()));
    let _ = line;
    Ok(Value::Int(capacity))
}

// -- the text of the v1 ---------------------------------------------------------------

/// The encoding that a string function of the v1 was given.
///
/// Measured on the binary of v1.1.37.02:
///
/// - the names are `UTF-8`, `UTF-16` and the number of a code page -- the
///   `CP0` is the code page of the machine, and the `CP936` is another one;
/// - a name that is empty is the code page of the machine;
/// - a call that does not name an encoding reads and writes the text in the
///   encoding of the machine, which is the `UTF-16` of the build of Unicode:
///   `StrGet(&buf)` reads the buffer that `StrPut(text, &buf, "UTF-16")`
///   wrote, and gives the same text back;
/// - a name that is not one of them is refused;
/// - the tree has one text for all the code pages: the letters of ASCII are
///   the same bytes in all of them, and the others are the ones that the
///   tree can hold.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TheEncoding {
    /// The bytes of the text, as the `UTF-8` writes them.
    Utf8,
    /// The units of the text, of two bytes each.
    Utf16,
    /// The bytes of the text, as the code page of the machine writes them.
    Cp0,
}

/// The encoding that a name asks for.
///
/// The answer is `None` when the name is not one that the v1 takes.
fn the_encoding_of(name: Option<&Value>) -> Option<TheEncoding> {
    match name {
        // A call that does not name an encoding uses the encoding of the
        // machine.
        None => Some(TheEncoding::Utf16),
        Some(Value::Text(text)) => {
            let name = text.trim();
            if name.is_empty() {
                return Some(TheEncoding::Cp0);
            }
            match name.to_ascii_lowercase().as_str() {
                "utf-8" => Some(TheEncoding::Utf8),
                "utf-16" => Some(TheEncoding::Utf16),
                other if other.starts_with("cp") => Some(TheEncoding::Cp0),
                _ => None,
            }
        }
        Some(_) => None,
    }
}

/// The bytes of a text in an encoding, and the number of the units of it.
///
/// The unit of an encoding is the byte of the `UTF-8` and of the code
/// pages, and the unit of two bytes of the `UTF-16`. Measured on the
/// binary of v1.1.37.02: `StrPut("hello", "UTF-16")` answers
/// six -- five units and the terminator -- and `StrPut("hello", "UTF-8")`
/// answers six as well; a text of six letters that are not ASCII needs nine
/// in the `UTF-8` and seven in the `UTF-16`.
fn the_bytes_of(text: &str, encoding: TheEncoding) -> (Vec<u8>, usize) {
    match encoding {
        TheEncoding::Utf8 | TheEncoding::Cp0 => {
            let bytes = text.as_bytes().to_vec();
            let units = bytes.len();
            (bytes, units)
        }
        TheEncoding::Utf16 => {
            let mut bytes = Vec::with_capacity(text.len() * 2);
            let mut units = 0;
            for unit in text.encode_utf16() {
                bytes.extend_from_slice(&unit.to_le_bytes());
                units += 1;
            }
            (bytes, units)
        }
    }
}

/// The terminator of an encoding: the bytes that end a text in it.
fn the_terminator_of(encoding: TheEncoding) -> Vec<u8> {
    match encoding {
        TheEncoding::Utf8 | TheEncoding::Cp0 => alloc::vec![0],
        TheEncoding::Utf16 => alloc::vec![0, 0],
    }
}

/// The text of some bytes in an encoding, up to a terminator or a length.
///
/// The length is the number of the units of the encoding: the bytes of the
/// `UTF-8` and of the code pages, and the units of two bytes of the
/// `UTF-16`. Measured on the binary of v1.1.37.02:
/// `StrGet(&buf, 3, "UTF-16")` gives the three first letters of the text
/// that was written in the `UTF-16`.
fn the_text_of(bytes: &[u8], encoding: TheEncoding, length: Option<usize>) -> String {
    match encoding {
        TheEncoding::Utf8 | TheEncoding::Cp0 => {
            let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
            let end = length.map(|length| end.min(length)).unwrap_or(end);
            String::from_utf8_lossy(&bytes[..end]).into_owned()
        }
        TheEncoding::Utf16 => {
            // A length of zero is the empty text: the v1 reads nothing.
            if length == Some(0) {
                return String::new();
            }
            let mut units: Vec<u16> = Vec::new();
            let mut at = 0;
            while at + 1 < bytes.len() {
                let unit = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
                if unit == 0 {
                    break;
                }
                units.push(unit);
                at += 2;
                if let Some(length) = length {
                    if units.len() >= length {
                        break;
                    }
                }
            }
            String::from_utf16_lossy(&units)
        }
    }
}

/// The address of the source of a `StrGet` or of the target of a `StrPut`.
fn the_address_of(rt: &mut Runtime<'_>, value: &Value) -> Option<i64> {
    match value {
        Value::Int(number) => Some(*number),
        Value::Float(number) => Some(*number as i64),
        Value::Reference(reference) => Some(rt.the_variable_address(reference)),
        _ => None,
    }
}

/// `StrGet(Source, Length, Encoding)`: the text at an address.
///
/// Measured on the binary of v1.1.37.02:
///
/// - the source is the address of the text: `&buf` is the address of the
///   memory of the variable, and a call of the machine answers the address
///   of the memory that it filled;
/// - the length is the number of the units of the encoding of the source,
///   and the second parameter is the encoding when it is not a number;
/// - the answer is the text of the memory, and the empty text when the
///   parameters are not ones that the v1 takes -- the v1 does not fail
///   here, it answers the empty text.
pub(crate) fn str_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(source) = values.first() else {
        return Ok(Value::Text("".into()));
    };
    let Some(address) = the_address_of(rt, source) else {
        return Ok(Value::Text("".into()));
    };
    // The second parameter is the length when it is a number, and the
    // encoding when it is a text; the third one is the encoding.
    let (length, encoding_name) = match values.get(1) {
        Some(value) if value.to_number().is_some() => (
            Some(value.to_number().unwrap_or(0.0).max(0.0) as usize),
            values.get(2),
        ),
        Some(value) => (None, Some(value)),
        None => (None, None),
    };
    let Some(encoding) = the_encoding_of(encoding_name) else {
        return Ok(Value::Text("".into()));
    };
    // The bytes of the memory of the script, or the bytes of the memory of
    // the machine.
    let bytes = match rt.the_bytes_at_an_address(address) {
        Some(bytes) => bytes,
        None => {
            let wanted = match length {
                Some(length) => length * the_size_of_the_unit(encoding),
                None => THE_LIMIT_OF_A_READ,
            };
            match rt.desktop().the_bytes_of_the_memory(address as u64, wanted) {
                Some(bytes) => bytes,
                None => return Ok(Value::Text("".into())),
            }
        }
    };
    let _ = line;
    Ok(Value::Text(the_text_of(&bytes, encoding, length).into()))
}
/// `StrPut(String, Target, Length, Encoding)`: the writing of a text in a
/// memory.
///
/// Measured on the binary of v1.1.37.02:
///
/// - the target is the address of the memory: `&buf` is the address of the
///   memory of the variable;
/// - the length is the number of the units that may be written, the
///   terminator included, and the third parameter is the encoding when it
///   is not a number;
/// - the answer is the number of the units that were written, the
///   terminator included, and it is zero when the length is smaller than
///   the text that was converted;
/// - a text that is exactly the length that was asked for is written
///   without the terminator, and a length of zero is refused: the answer
///   is the empty text;
/// - a call with two parameters asks for the size that the text needs, and
///   the answer is that size in the units of the encoding;
/// - the answer is the empty text when the parameters are not ones that the
///   v1 takes.
pub(crate) fn str_put(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = match values.first() {
        Some(Value::Text(text)) => text.clone(),
        Some(value @ (Value::Object(_) | Value::List(_))) => {
            let _ = value;
            return Ok(Value::Text("".into()));
        }
        Some(value) => value.to_text().into(),
        None => return Ok(Value::Text("".into())),
    };
    let Some(target) = values.get(1) else {
        // The one parameter: the size that the text needs, in the
        // encoding of the machine.
        let (_, units) = the_bytes_of(&text, TheEncoding::Utf16);
        return Ok(Value::Int((units + 1) as i64));
    };
    // The two parameters: the size that the text needs.
    if let Value::Text(name) = target {
        let Some(encoding) = the_encoding_of(Some(&Value::Text(name.clone()))) else {
            return Ok(Value::Text("".into()));
        };
        let (_, units) = the_bytes_of(&text, encoding);
        return Ok(Value::Int((units + 1) as i64));
    }
    let Some(address) = the_address_of(rt, target) else {
        return Ok(Value::Text("".into()));
    };
    // The third parameter is the length when it is a number, and the
    // encoding when it is a text; the fourth one is the encoding, and a
    // number there is the number of a code page.
    let (length, encoding) = match values.get(2) {
        Some(value) if value.to_number().is_some() => (
            Some(value.to_number().unwrap_or(0.0) as i64),
            match values.get(3) {
                Some(value) if value.to_number().is_some() => Some(TheEncoding::Cp0),
                Some(value) => the_encoding_of(Some(value)),
                None => Some(TheEncoding::Utf16),
            },
        ),
        Some(value) => (None, the_encoding_of(Some(value))),
        None => (None, Some(TheEncoding::Utf16)),
    };
    let Some(encoding) = encoding else {
        return Ok(Value::Text("".into()));
    };
    let (bytes, units) = the_bytes_of(&text, encoding);
    let terminator = the_terminator_of(encoding);
    // The length that was asked for decides what goes into the memory.
    let (bytes, units) = match length {
        None => {
            let mut all = bytes;
            all.extend_from_slice(&terminator);
            (all, units + 1)
        }
        Some(length) => {
            // A length of zero is refused by the v1, and the answer is the
            // empty text.
            if length == 0 {
                return Ok(Value::Text("".into()));
            }
            // The text does not fit: nothing is written.
            if length < units as i64 {
                return Ok(Value::Int(0));
            }
            // The text fills the room that it was given: there is no room
            // for the terminator.
            if length == units as i64 {
                (bytes, units)
            } else {
                let mut all = bytes;
                all.extend_from_slice(&terminator);
                (all, units + 1)
            }
        }
    };
    // The write of the bytes in the memory of the target.
    let variable = rt.the_bytes_are_written_at_an_address(address, &bytes);
    if variable.is_none() && !rt.the_bytes_are_written_at_a_text(address, &bytes) {
        rt.desktop().the_memory_is_written(address as u64, &bytes);
    }
    // The value of the variable is the text that was written, and the
    // memory of it keeps the bytes: the value is written without the
    // change of the buffer that an assignment makes.
    if let Some(reference) = variable {
        let text = the_text_of(&bytes, encoding, None);
        rt.write_reference(&Value::Reference(reference), Value::Text(text.into()));
    }
    let _ = line;
    Ok(Value::Int(units as i64))
}

/// The size of the unit of an encoding, in bytes.
fn the_size_of_the_unit(encoding: TheEncoding) -> usize {
    match encoding {
        TheEncoding::Utf16 => 2,
        TheEncoding::Utf8 | TheEncoding::Cp0 => 1,
    }
}

/// The number of the bytes that a read of the memory of the machine takes
/// when the length of the text is not known.
const THE_LIMIT_OF_A_READ: usize = 64 * 1024;
