//! The `File` class.
//!
//! A file is an open file on the disk. The box can serve this one: it
//! is the class that touches the machine.
//!
//! The class does no I/O of its own: it asks the desktop for the bytes
//! and it keeps the handle, the position and the encoding of the file
//! in the property table of the object, under one key.
//!
//! The text of a file is UTF-8 or UTF-16, the two encodings the
//! runtime carries: an encoding it does not carry is a failure with the
//! reason, and not a text that is written in the wrong bytes.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::buffer;
use crate::classes::object_with;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, expected_number, expected_type};

/// The key the state of a file lives under.
///
/// The key has a space in it, so that a script cannot reach the inside
/// of a file by name.
const STATE: &str = "file state";

/// The class of the object that carries the state of a file.
const STATE_CLASS: &str = "file-state";

/// The properties of a file.
const PROPERTIES: [&str; 5] = ["pos", "length", "ateof", "encoding", "handle"];

/// The methods of a file.
///
/// The binary of v2.0.19 has no other: a file is not made again, and it
/// has no unsigned number of sixty-four bits.
const METHODS: [&str; 35] = [
    "read", "readline", "readchar", "readuchar", "readshort", "readushort",
    "readint", "readuint", "readint64", "readfloat", "readdouble",
    "write", "writeline", "writechar", "writeuchar", "writeshort", "writeushort",
    "writeint", "writeuint", "writeint64", "writefloat", "writedouble",
    "rawread", "rawwrite", "seek", "close",
    "ownprops", "hasownprop", "defineprop", "deleteprop", "getownpropdesc",
    "hasprop", "hasmethod", "getmethod", "hasbase",
];

/// The handle of a file that is closed.
const CLOSED: i64 = -1;

/// How many bytes are read at a time when a line is looked for.
const CHUNK: i64 = 4096;

/// The state of a file: the handle, the position and the encoding.
struct File {
    handle: i64,
    pos: i64,
    encoding: Encoding,
}

/// The encodings the runtime carries.
#[derive(Clone, Copy)]
enum Encoding {
    Utf8,
    Utf16,
}

/// The object a `File` makes.
/// The names a file answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

pub(crate) fn construct(
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let file = open(arguments, rt, line)?;
    Ok(object_with("file", vec![(STATE, stored(&file))]))
}

/// A method of a `File`.
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
        "read" => Some(read_text(this, arguments, rt, line)),
        "readline" => Some(read_line(this, rt, line)),
        "readchar" => Some(read_number(rt, this, 1, line, |bytes| Value::Int(bytes[0] as i8 as i64))),
        "readuchar" => Some(read_number(rt, this, 1, line, |bytes| Value::Int(bytes[0] as i64))),
        "readshort" => Some(read_number(rt, this, 2, line, |bytes| {
            Value::Int(little(bytes) as u16 as i16 as i64)
        })),
        "readushort" => Some(read_number(rt, this, 2, line, |bytes| Value::Int(little(bytes) as u16 as i64))),
        "readint" => Some(read_number(rt, this, 4, line, |bytes| {
            Value::Int(little(bytes) as u32 as i32 as i64)
        })),
        "readuint" => Some(read_number(rt, this, 4, line, |bytes| Value::Int(little(bytes) as u32 as i64))),
        "readint64" => Some(read_number(rt, this, 8, line, |bytes| Value::Int(little(bytes) as i64))),
        "readfloat" => Some(read_number(rt, this, 4, line, |bytes| {
            Value::Float(f32::from_bits(little(bytes) as u32) as f64)
        })),
        "readdouble" => Some(read_number(rt, this, 8, line, |bytes| {
            Value::Float(f64::from_bits(little(bytes)))
        })),
        "write" => Some(write_text(this, arguments, rt, line)),
        "writeline" => Some(write_line(this, arguments, rt, line)),
        "writechar" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFF, 1)
        })),
        "writeuchar" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFF, 1)
        })),
        "writeshort" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFFFF, 2)
        })),
        "writeushort" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFFFF, 2)
        })),
        "writeint" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFFFF_FFFF, 4)
        })),
        "writeuint" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64 & 0xFFFF_FFFF, 4)
        })),
        "writeint64" => Some(write_number(rt, this, arguments, line, |value| {
            little_bytes(value as i64 as u64, 8)
        })),
        "writefloat" => Some(write_number(rt, this, arguments, line, |value| {
            float_bytes(value, true)
        })),
        "writedouble" => Some(write_number(rt, this, arguments, line, |value| {
            float_bytes(value, false)
        })),
        "rawread" => Some(raw_read(this, arguments, rt, line)),
        "rawwrite" => Some(raw_write(this, arguments, rt, line)),
        "seek" => Some(seek(this, arguments, rt, line)),
        "close" => Some(close(this, rt, line)),
        // The methods of the base object are answered here too: the
        // runtime knows nothing of the class types of the runtime.
        _ => base_method(&method, this, arguments, line),
    }
}

/// A property of a `File`.
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
        "pos" => Some(state(this, line).map(|file| Value::Int(file.pos))),
        "length" => Some(length(rt, this, line)),
        "ateof" => Some(at_eof(rt, this, line)),
        "encoding" => Some(
            state(this, line).map(|file| Value::Text(file.encoding.name().to_string())),
        ),
        "handle" => Some(state(this, line).map(|file| Value::Int(file.handle))),
        _ => None,
    }
}

/// A property of a `File` that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "pos" => Some(set_pos(this, value, line)),
        "length" => Some(set_length(rt, this, value, line)),
        "encoding" => Some(set_encoding(this, value, line)),
        _ => None,
    }
}

/// The item of a `File`: the `__Item` of the class.
///
/// `None` means the class has no `__Item` of its own.
pub(crate) fn item(
    _key: &Value,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    None
}

// -- the state ------------------------------------------------------------------

/// The failure of a file, with the line the script was on.
///
/// The reason of the machine carries the code of the system at its end,
/// and v2 puts it in front.
fn fail(message: impl Into<String>, line: u32) -> RuntimeFailure {
    let message = message.into();
    RuntimeFailure::Error(RuntimeError { message: the_reason_of_the_machine(&message), line })
}

/// The reason of the machine, the way v2 writes it.
///
/// v2 puts the code of the system in front of what it says: "(2) The
/// system cannot find the file specified."
fn the_reason_of_the_machine(reason: &str) -> String {
    let Some(without_the_bracket) = reason.strip_suffix(')') else {
        return reason.to_string();
    };
    let Some((message, number)) = without_the_bracket.rsplit_once(" (os error ") else {
        return reason.to_string();
    };
    match number.parse::<i64>() {
        Ok(number) => format!("({number}) {message}"),
        Err(_) => reason.to_string(),
    }
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

/// The number an argument holds.
fn number(value: &Value, line: u32) -> Result<f64, RuntimeFailure> {
    value.to_number_strict().map_err(|_| fail(expected_number(value), line))
}

/// The state of a file, as the object holds it.
fn state(this: &Value, line: u32) -> Result<File, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a file.", line));
    };
    let inner = match object.borrow().properties.get(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a file.", line)),
    };
    let inner = inner.borrow();
    let handle = inner
        .properties
        .get("handle")
        .and_then(|value| value.to_number())
        .unwrap_or(CLOSED as f64) as i64;
    let pos = inner
        .properties
        .get("pos")
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i64;
    let encoding = match inner.properties.get("encoding") {
        Some(value) => Encoding::of(&value.to_text()).map_err(|reason| fail(reason, line))?,
        None => Encoding::Utf8,
    };
    Ok(File { handle, pos, encoding })
}

/// The state of a file, as the property table holds it.
fn stored(file: &File) -> Value {
    object_with(
        STATE_CLASS,
        vec![
            ("handle", Value::Int(file.handle)),
            ("pos", Value::Int(file.pos)),
            ("encoding", Value::Text(file.encoding.name().to_string())),
        ],
    )
}

/// Put the state back into the object.
fn restore(this: &Value, file: &File) {
    let Value::Object(object) = this else { return };
    object.borrow_mut().properties.insert(STATE.to_string(), stored(file));
}

/// The handle of the file, which has to be open.
fn handle_of(file: &File, line: u32) -> Result<i64, RuntimeFailure> {
    if file.handle < 0 {
        return Err(fail("the file is closed.", line));
    }
    Ok(file.handle)
}

/// The bytes of the file at a place in it.
fn read_at(
    rt: &mut Runtime,
    handle: i64,
    pos: i64,
    count: i64,
    line: u32,
) -> Result<Vec<u8>, RuntimeFailure> {
    rt.desktop()
        .read_file_at(handle, pos, count)
        .map_err(|reason| fail(reason, line))
}

/// Open a file and answer the state of it.
fn open(arguments: &[Value], rt: &mut Runtime, line: u32) -> Result<File, RuntimeFailure> {
    let path = match arguments.first() {
        Some(value) => value.to_text(),
        None => return Err(fail("the file needs a path.", line)),
    };
    let mode = match arguments.get(1) {
        Some(value) => value.to_text(),
        None => "r".to_string(),
    };
    // The modes of v2 are "r", "w", "a" and "rw", and the case of the
    // letters is not the case of the words.
    if !matches!(mode.to_lowercase().as_str(), "r" | "w" | "a" | "rw") {
        return Err(fail("Invalid parameter(s).", line));
    }
    let encoding = match arguments.get(2) {
        Some(value) => Encoding::of(&value.to_text()).map_err(|reason| fail(reason, line))?,
        None => Encoding::Utf8,
    };
    let handle = rt
        .desktop()
        .open_file(&path, &mode)
        .map_err(|reason| fail(reason, line))?;
    // A file that is opened to be appended to starts at its end.
    let pos = if mode.contains('a') {
        rt.desktop()
            .file_size(handle)
            .map_err(|reason| fail(reason, line))?
    } else {
        0
    };
    Ok(File { handle, pos, encoding })
}

/// `Read`: the text of the file at its position.
fn read_text(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    let count = match arguments.first() {
        Some(value) => Some(whole(value, line)?),
        None => None,
    };
    let text = read_chars(rt, &mut file, count, line)?;
    restore(this, &file);
    Ok(Value::Text(text))
}

/// The text of the file at its position, and the position moves on.
fn read_chars(
    rt: &mut Runtime,
    file: &mut File,
    count: Option<i64>,
    line: u32,
) -> Result<String, RuntimeFailure> {
    let handle = handle_of(file, line)?;
    let (text, used) = match count {
        Some(count) => {
            if count < 0 {
                return Err(fail("the count of characters cannot be negative.", line));
            }
            // A character is at most four bytes, and a UTF-16 one is
            // two: the read is wide enough for what was asked for.
            let wide = match file.encoding {
                Encoding::Utf8 => count * 4,
                Encoding::Utf16 => count * 2,
            };
            let bytes = read_at(rt, handle, file.pos, wide, line)?;
            let (text, used) = decode(&bytes, file.encoding).map_err(|reason| fail(reason, line))?;
            match file.encoding {
                // The characters of a UTF-8 text are counted by the
                // bytes they take, so the cut is where the text says.
                Encoding::Utf8 => {
                    let (text, used) = take(&text, count as usize);
                    (text, used)
                }
                // A UTF-16 character is two bytes, and the read was
                // wide enough for the units that were asked for.
                Encoding::Utf16 => (text, used),
            }
        }
        None => {
            let size = rt
                .desktop()
                .file_size(handle)
                .map_err(|reason| fail(reason, line))?;
            let bytes = read_at(rt, handle, file.pos, (size - file.pos).max(0), line)?;
            decode(&bytes, file.encoding).map_err(|reason| fail(reason, line))?
        }
    };
    file.pos += used as i64;
    Ok(text)
}

/// `ReadLine`: the line of the file at its position.
fn read_line(this: &Value, rt: &mut Runtime, line: u32) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let mut raw: Vec<u8> = Vec::new();
    let text = loop {
        let bytes = read_at(rt, handle, file.pos + raw.len() as i64, CHUNK, line)?;
        let at_end = bytes.is_empty();
        raw.extend_from_slice(&bytes);
        match line_of(&raw, file.encoding) {
            Some((end, after)) => {
                let (text, _) = decode(&raw[..end], file.encoding).map_err(|reason| fail(reason, line))?;
                file.pos += after as i64;
                break text;
            }
            // The last line of a file has no newline after it.
            None if at_end => {
                let (text, used) = decode(&raw, file.encoding).map_err(|reason| fail(reason, line))?;
                file.pos += used as i64;
                break text;
            }
            None => {}
        }
    };
    restore(this, &file);
    Ok(Value::Text(text))
}

/// The bytes of a number of the file at its position.
fn read_number(
    rt: &mut Runtime,
    this: &Value,
    count: usize,
    line: u32,
    of: fn(&[u8]) -> Value,
) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let bytes = read_at(rt, handle, file.pos, count as i64, line)?;
    if bytes.is_empty() {
        return Err(fail("the read is past the end of the file.", line));
    }
    // A number that is cut in half at the end of the file is zero on
    // the bytes that are missing, the way v2 reads one.
    let mut full = bytes.clone();
    full.resize(count, 0);
    file.pos += bytes.len() as i64;
    restore(this, &file);
    Ok(of(&full))
}

/// `Write`: the text of the script into the file.
fn write_text(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let text = match arguments.first() {
        Some(value) => value.to_text(),
        None => return Err(fail("Too few parameters passed to function.", line)),
    };
    let mut file = state(this, line)?;
    let bytes = encode(&text, file.encoding);
    let written = write_bytes(rt, &mut file, &bytes, line)?;
    restore(this, &file);
    Ok(written)
}

/// `WriteLine`: the text of the script, and a newline, into the file.
fn write_line(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let text = match arguments.first() {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    let mut file = state(this, line)?;
    let mut bytes = encode(&text, file.encoding);
    bytes.extend_from_slice(&encode("\n", file.encoding));
    let written = write_bytes(rt, &mut file, &bytes, line)?;
    restore(this, &file);
    Ok(written)
}

/// The bytes of a number, written into the file at its position.
fn write_number(
    rt: &mut Runtime,
    this: &Value,
    arguments: &[Value],
    line: u32,
    of: fn(f64) -> Vec<u8>,
) -> Result<Value, RuntimeFailure> {
    let value = match arguments.first() {
        Some(value) => number(value, line)?,
        None => return Err(fail("Too few parameters passed to function.", line)),
    };
    let mut file = state(this, line)?;
    let written = write_bytes(rt, &mut file, &of(value), line)?;
    restore(this, &file);
    Ok(written)
}

/// The bytes of the file at its position, written over what is there.
fn write_bytes(
    rt: &mut Runtime,
    file: &mut File,
    bytes: &[u8],
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let handle = handle_of(file, line)?;
    let written = rt
        .desktop()
        .write_file_at(handle, file.pos, bytes)
        .map_err(|reason| fail(reason, line))?;
    file.pos += written as i64;
    Ok(Value::Int(written as i64))
}

/// `RawRead`: the bytes of the file, into a buffer.
fn raw_read(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let target = match arguments.first() {
        Some(value) => value.clone(),
        None => return Err(fail("Too few parameters passed to function.", line)),
    };
    // The bytes are read into a buffer, and the buffer takes them all.
    let count = match arguments.get(1) {
        Some(value) => whole(value, line)?,
        None => buffer::size_of(&target, line)?,
    };
    if count < 0 {
        return Err(fail("the count of bytes cannot be negative.", line));
    }
    let bytes = read_at(rt, handle, file.pos, count, line)?;
    let written = buffer::fill(&target, &bytes, line)?;
    file.pos += bytes.len() as i64;
    restore(this, &file);
    Ok(Value::Int(written as i64))
}

/// `RawWrite`: the bytes of a buffer or of a string, into the file.
fn raw_write(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    let bytes = match arguments.first() {
        // A buffer is written as it is, and a string is written in the
        // encoding of the file, the way `Write` writes it.
        Some(Value::Object(_)) => buffer::raw_bytes(&arguments[0], line)?,
        Some(Value::Text(text)) => encode(text, file.encoding),
        Some(other) => {
            return Err(fail(expected_type(TypeName::Object, other.type_name()), line));
        }
        None => return Err(fail("Too few parameters passed to function.", line)),
    };
    let count = match arguments.get(1) {
        Some(value) => whole(value, line)?,
        None => bytes.len() as i64,
    };
    if count < 0 || count as usize > bytes.len() {
        return Err(fail("there are not that many bytes to write.", line));
    }
    let written = write_bytes(rt, &mut file, &bytes[..count as usize], line)?;
    restore(this, &file);
    Ok(written)
}

/// `Seek`: the position of the file, somewhere else.
fn seek(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let distance = match arguments.first() {
        Some(value) => whole(value, line)?,
        None => return Err(fail("Too few parameters passed to function.", line)),
    };
    let mut file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    // A distance that is negative moves from the end of the file, the
    // way v2 moves it.
    let origin = match arguments.get(1) {
        Some(value) => whole(value, line)?,
        None => {
            if distance < 0 {
                2
            } else {
                0
            }
        }
    };
    let target = match origin {
        0 => distance,
        1 => file.pos + distance,
        2 => {
            let size = rt
                .desktop()
                .file_size(handle)
                .map_err(|reason| fail(reason, line))?;
            size + distance
        }
        _ => return Ok(Value::Int(0)),
    };
    if target < 0 {
        return Ok(Value::Int(0));
    }
    file.pos = target;
    restore(this, &file);
    Ok(Value::Int(1))
}

/// `Close`: the file is closed and the handle is gone.
fn close(this: &Value, rt: &mut Runtime, line: u32) -> Result<Value, RuntimeFailure> {
    let mut file = state(this, line)?;
    if file.handle >= 0 {
        rt.desktop()
            .close_file(file.handle)
            .map_err(|reason| fail(reason, line))?;
        file.handle = CLOSED;
        restore(this, &file);
    }
    Ok(Value::Text(String::new()))
}

/// How long the file is.
fn length(rt: &mut Runtime, this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    let file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let size = rt
        .desktop()
        .file_size(handle)
        .map_err(|reason| fail(reason, line))?;
    Ok(Value::Int(size))
}

/// Whether the file is at its end.
fn at_eof(rt: &mut Runtime, this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    let file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let size = rt
        .desktop()
        .file_size(handle)
        .map_err(|reason| fail(reason, line))?;
    Ok(Value::Int(if file.pos >= size { 1 } else { 0 }))
}

/// Put the position of the file somewhere else.
fn set_pos(this: &Value, value: Value, line: u32) -> Result<(), RuntimeFailure> {
    let pos = whole(&value, line)?;
    if pos < 0 {
        return Err(fail("the position of a file cannot be negative.", line));
    }
    let mut file = state(this, line)?;
    file.pos = pos;
    restore(this, &file);
    Ok(())
}

/// Make the file of another length.
///
/// The desktop can make a file longer, by writing at its end, but it
/// cannot cut one: a file that is made shorter fails with the reason.
fn set_length(
    rt: &mut Runtime,
    this: &Value,
    value: Value,
    line: u32,
) -> Result<(), RuntimeFailure> {
    let size = whole(&value, line)?;
    if size < 0 {
        return Err(fail("the length of a file cannot be negative.", line));
    }
    let file = state(this, line)?;
    let handle = handle_of(&file, line)?;
    let current = rt
        .desktop()
        .file_size(handle)
        .map_err(|reason| fail(reason, line))?;
    if size < current {
        return Err(fail(
            "this desktop cannot make a file shorter: it cannot cut one.",
            line,
        ));
    }
    if size > current {
        let zeros = vec![0u8; (size - current) as usize];
        rt.desktop()
            .write_file_at(handle, current, &zeros)
            .map_err(|reason| fail(reason, line))?;
    }
    Ok(())
}

/// Put the encoding of the file somewhere else.
fn set_encoding(this: &Value, value: Value, line: u32) -> Result<(), RuntimeFailure> {
    let encoding = Encoding::of(&value.to_text()).map_err(|reason| fail(reason, line))?;
    let mut file = state(this, line)?;
    file.encoding = encoding;
    restore(this, &file);
    Ok(())
}

// -- the encoding ---------------------------------------------------------------

impl Encoding {
    /// The encoding of a name, as v2 spells it.
    fn of(name: &str) -> Result<Encoding, String> {
        let name = name.trim().to_uppercase();
        // v2 ignores the `-RAW` suffix of the encoding of a file: the
        // suffix says that the file has no byte order mark, and the
        // runtime writes none either way.
        let name = match name.strip_suffix("-RAW") {
            Some(short) => short,
            None => name.as_str(),
        };
        match name {
            "UTF-8" | "UTF8" => Ok(Encoding::Utf8),
            "UTF-16" | "UTF16" => Ok(Encoding::Utf16),
            other => Err(format!(
                "the encoding {other:?} is not one this runtime carries: it knows UTF-8 and UTF-16."
            )),
        }
    }

    /// The name of the encoding, as v2 spells it.
    fn name(self) -> &'static str {
        match self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Utf16 => "UTF-16",
        }
    }
}

/// The bytes of the text, in the encoding.
fn encode(text: &str, encoding: Encoding) -> Vec<u8> {
    match encoding {
        Encoding::Utf8 => text.as_bytes().to_vec(),
        Encoding::Utf16 => {
            let mut bytes = Vec::new();
            for character in text.chars() {
                let mut units = [0u16; 2];
                for unit in character.encode_utf16(&mut units) {
                    bytes.extend_from_slice(&unit.to_le_bytes());
                }
            }
            bytes
        }
    }
}

/// The text of the bytes, in the encoding, and how many bytes of them
/// the text is.
fn decode(bytes: &[u8], encoding: Encoding) -> Result<(String, usize), String> {
    match encoding {
        Encoding::Utf8 => decode_utf8(bytes),
        Encoding::Utf16 => decode_utf16(bytes),
    }
}

/// The text of the bytes of a UTF-8 file, and how many of them it used.
///
/// A character that is cut in half at the end of the bytes is not text
/// yet: it is left for the next read.
fn decode_utf8(bytes: &[u8]) -> Result<(String, usize), String> {
    match core::str::from_utf8(bytes) {
        Ok(text) => Ok((text.to_string(), bytes.len())),
        Err(error) if error.error_len().is_none() => {
            let used = error.valid_up_to();
            let text = core::str::from_utf8(&bytes[..used]).map_err(|_| broken())?;
            Ok((text.to_string(), used))
        }
        Err(_) => Err(broken()),
    }
}

/// The text of the bytes of a UTF-16 file, and how many of them it used.
///
/// The units are little endian, and a unit that is cut in half at the
/// end of the bytes is left for the next read.
fn decode_utf16(bytes: &[u8]) -> Result<(String, usize), String> {
    let mut text = String::new();
    let mut used = 0usize;
    let mut lead: Option<u16> = None;
    let mut index = 0usize;
    while index + 1 < bytes.len() {
        let unit = u16::from_le_bytes([bytes[index], bytes[index + 1]]);
        match lead.take() {
            Some(high) => {
                if !(0xDC00..0xE000).contains(&unit) {
                    return Err(broken());
                }
                let code = 0x10000 + ((high as u32 - 0xD800) << 10) + (unit as u32 - 0xDC00);
                text.push(char::from_u32(code).ok_or_else(broken)?);
                used = index + 2;
            }
            None => {
                if (0xD800..0xDC00).contains(&unit) {
                    // The unit is the first half of a pair: the second
                    // half may be in the next read.
                    lead = Some(unit);
                } else if (0xDC00..0xE000).contains(&unit) {
                    return Err(broken());
                } else {
                    text.push(char::from_u32(unit as u32).ok_or_else(broken)?);
                    used = index + 2;
                }
            }
        }
        index += 2;
    }
    Ok((text, used))
}

/// The reason why the bytes of a file are not text.
fn broken() -> String {
    "the bytes of the file are not text in this encoding.".to_string()
}

/// The first characters of the text, and the bytes they take.
fn take(text: &str, count: usize) -> (String, usize) {
    match text.char_indices().nth(count) {
        Some((at, _)) => (text[..at].to_string(), at),
        None => (text.to_string(), text.len()),
    }
}

/// The bytes of the line, and the byte after the newline that ends it.
fn line_of(bytes: &[u8], encoding: Encoding) -> Option<(usize, usize)> {
    match encoding {
        Encoding::Utf8 => {
            let at = bytes.iter().position(|byte| *byte == 0x0A)?;
            // A line that ends with a carriage return and a newline
            // does not carry the carriage return.
            let end = if at > 0 && bytes[at - 1] == 0x0D { at - 1 } else { at };
            Some((end, at + 1))
        }
        Encoding::Utf16 => {
            let at = bytes
                .windows(2)
                .position(|pair| pair[0] == 0x0A && pair[1] == 0x00)?;
            let end = if at >= 2 && bytes[at - 2] == 0x0D && bytes[at - 1] == 0x00 {
                at - 2
            } else {
                at
            };
            Some((end, at + 2))
        }
    }
}

/// A whole number out of the bytes, little endian.
fn little(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0u64, |acc, byte| (acc << 8) | *byte as u64)
}

/// The bytes of a whole number, little endian, as many as the type
/// takes.
fn little_bytes(number: u64, count: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; count];
    for (index, byte) in bytes.iter_mut().rev().enumerate() {
        *byte = ((number >> (index * 8)) & 0xFF) as u8;
    }
    bytes
}

/// The bytes of a float, little endian: four of them for a single
/// precision float, eight for a double.
fn float_bytes(number: f64, single: bool) -> Vec<u8> {
    if single {
        little_bytes((number as f32).to_bits() as u64, 4)
    } else {
        little_bytes(number.to_bits(), 8)
    }
}

// -- the base object ----------------------------------------------------------

/// The methods of the base object, answered for a file.
///
/// The runtime answers them for the classes of the script, and it knows
/// nothing of the class types of the runtime: a file has no method of
/// the script, and the handle of a file is not a property of it.
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
        // A file extends the base object, and nothing else.
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
                return failure("the state of a file is not a property of it.");
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

/// Whether the file has a property of that name.
///
/// The state of the file is not a property of it: it is the inside of
/// the file, and the script never wrote it.
fn own_property(object: &Rc<RefCell<Object>>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}
