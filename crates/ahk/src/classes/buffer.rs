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
//! The bytes live in a cell of the object, under one key, so that the
//! module owns the whole of the state of a buffer. The cell is shared
//! with the machine: the address of the bytes is the `Ptr` of the
//! buffer, and what the machine writes there is what `NumGet` reads.
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::classes::enumerator;
use crate::classes::object_with;
use crate::dll::NativeType;
use crate::runtime::{
    Detail, Object, Runtime, RuntimeError, RuntimeFailure, SharedCell, TypeName, Value, expected_type, put,
};

/// The key the state of a buffer lives under.
///
/// The key has a space in it, so that a script cannot reach the inside
/// of a buffer by name.
const STATE: &str = "buffer state";

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

pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let bytes = new(arguments, line)?;
    Ok(object_with("buffer", vec![(STATE, stored(bytes))]))
}

/// A buffer of the bytes of a file.
///
/// The `FileRead` answers with a buffer when the script asks for the raw
/// bytes of the file.
pub(crate) fn of_bytes(bytes: Vec<u8>) -> Value {
    object_with("buffer", vec![(STATE, stored(bytes))])
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
        "__new" => Some(new(arguments, line).map(|bytes| {
            let size = bytes.len() as i64;
            restore(this, bytes);
            Value::Int(size)
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
        "size" => Some(size_of(this, line).map(Value::Int)),
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
        // The bytes are copied out of the buffer: the caller writes them
        // into a file or into the machine.
        Value::Object(_) => {
            let cell = the_cell(value, line)?;
            let bytes = cell.borrow();
            Ok(bytes.clone())
        }
        Value::Text(text) => Ok(text.as_bytes().to_vec()),
        other => Err(fail(
            expected_type(TypeName::Object, other.type_name()),
            line,
        )),
    }
}

/// The bytes of a buffer from an offset to the end.
///
/// Used when `StrGet` reads from an interior address inside a buffer
/// (for example a `UNICODE_STRING.Buffer` pointer written by `NtQueryObject`).
pub(crate) fn raw_bytes_from(
    value: &Value,
    offset: usize,
    line: u32,
) -> Result<Vec<u8>, RuntimeFailure> {
    let cell = the_cell(value, line)?;
    let bytes = cell.borrow();
    Ok(bytes.get(offset..).unwrap_or(&[]).to_vec())
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
    let cell = the_cell(value, line)?;
    let size = cell.borrow().len() as i64;
    Ok(size)
}

/// The address of the memory of the buffer.
///
/// The bytes of a buffer live in the object of the script, and the
/// machine reads and writes a mirror of them: the address is the one of
/// the mirror, a real block of the heap, and it changes when the buffer
/// is made of another size, as v2 says of a buffer that is reallocated.
pub(crate) fn ptr_of(this: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    let cell = the_cell(this, line)?;
    // The address of a buffer is the address of its own bytes: the
    // machine writes into them, and the script reads them, without a copy
    // in between.
    #[cfg(feature = "std")]
    {
        Ok(crate::memory::the_address_of_a_buffer(this, &cell))
    }
    #[cfg(not(feature = "std"))]
    {
        let address = cell.borrow().as_ptr() as usize as i64;
        Ok(address)
    }
}

/// Fast path for reading `Ptr` or `Size` on a live `Buffer` instance without
/// allocating property name strings or scanning class/prototype tables.
///
/// Returns `None` for non-buffers, class values, prototypes, or other names so
/// the caller can fall back to the full property resolution path.
#[inline]
pub(crate) fn try_ptr_or_size(this: &Value, name: &str) -> Option<i64> {
    let is_ptr = name.eq_ignore_ascii_case("ptr");
    if !is_ptr && !name.eq_ignore_ascii_case("size") {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    let cell = {
        let borrowed = object.borrow();
        if borrowed.class != "buffer" {
            return None;
        }
        match borrowed.value_of(STATE) {
            Some(Value::Bytes(cell)) => cell.clone(),
            _ => return None,
        }
    };
    if is_ptr {
        #[cfg(feature = "std")]
        {
            Some(crate::memory::the_address_of_a_buffer(this, &cell))
        }
        #[cfg(not(feature = "std"))]
        {
            Some(cell.borrow().as_ptr() as usize as i64)
        }
    } else {
        Some(cell.borrow().len() as i64)
    }
}

/// Give an object the zero-filled bytes of a struct of the v2.1, reusing
/// the state of a buffer so the reads, the writes and the address of it
/// are the ones this module already owns.
pub(crate) fn allocate(this: &Value, size: usize) {
    restore(this, a_block_of(size, 0));
}

/// The type of a number, out of the name the script writes.
///
/// The types of a number are the types of a native call, and the tree
/// keeps them in one table: `NumGet` and `NumPut` name them the way
/// `DllCall` does.
pub(crate) fn number_type(text: &str) -> Option<NativeType> {
    let trimmed = text.trim();
    if trimmed.eq_ignore_ascii_case("uint") {
        Some(NativeType::UInt)
    } else if trimmed.eq_ignore_ascii_case("int") {
        Some(NativeType::Int)
    } else if trimmed.eq_ignore_ascii_case("int64") {
        Some(NativeType::Int64)
    } else if trimmed.eq_ignore_ascii_case("uptr") {
        Some(NativeType::UPtr)
    } else if trimmed.eq_ignore_ascii_case("ptr") {
        Some(NativeType::Ptr)
    } else if trimmed.eq_ignore_ascii_case("ushort") {
        Some(NativeType::UShort)
    } else if trimmed.eq_ignore_ascii_case("short") {
        Some(NativeType::Short)
    } else if trimmed.eq_ignore_ascii_case("uchar") {
        Some(NativeType::UChar)
    } else if trimmed.eq_ignore_ascii_case("char") {
        Some(NativeType::Char)
    } else if trimmed.eq_ignore_ascii_case("double") {
        Some(NativeType::Double)
    } else if trimmed.eq_ignore_ascii_case("float") {
        Some(NativeType::Float)
    } else {
        None
    }
}

/// Read a number from a known buffer cell when `address` falls inside it.
///
/// Returns `None` if `address` is outside `cell` (caller should re-probe
/// `memory::find_mirror_cell`).
#[inline]
pub(crate) fn read_number_at_cell(
    cell: &SharedCell<Vec<u8>>,
    address: i64,
    offset: i64,
    kind: NativeType,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let block = cell.borrow();
    let base = block.as_ptr() as i64;
    let end_addr = base + block.len() as i64;
    if address < base || address >= end_addr {
        return None;
    }
    let target = (address - base).checked_add(offset);
    let Some(start) = target.filter(|&n| n >= 0).map(|n| n as usize) else {
        return Some(Err(fail("the offset of a buffer may not be negative.", line)));
    };
    let end = start.saturating_add(kind.width());
    if end > block.len() {
        return Some(Err(fail("the bytes do not fit in the buffer.", line)));
    }
    Some(Ok(number_from_bytes(&block[start..end], kind)))
}

/// Read a number directly from a buffer's backing byte cell at `offset`.
#[inline]
pub(crate) fn read_number_from_cell(
    cell: &SharedCell<Vec<u8>>,
    offset: i64,
    kind: NativeType,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let start = start_of(offset, line)?;
    let end = start.saturating_add(kind.width());
    let block = cell.borrow();
    if end > block.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    Ok(number_from_bytes(&block[start..end], kind))
}

/// Return the backing byte cell if `value` is a live `Buffer` instance.
#[inline]
pub(crate) fn live_cell(value: &Value) -> Option<SharedCell<Vec<u8>>> {
    let Value::Object(object) = value else {
        return None;
    };
    let borrowed = object.borrow();
    if borrowed.class != "buffer" {
        return None;
    }
    match borrowed.value_of(STATE) {
        Some(Value::Bytes(cell)) => Some(cell.clone()),
        _ => None,
    }
}

/// Return the native address for an already-resolved live buffer cell.
#[inline]
pub(crate) fn ptr_of_cell(owner: &Value, cell: &SharedCell<Vec<u8>>) -> i64 {
    #[cfg(feature = "std")]
    {
        crate::memory::the_address_of_a_buffer(owner, cell)
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = owner;
        cell.borrow().as_ptr() as usize as i64
    }
}

/// Write a single number directly into a buffer's backing byte cell at `offset`.
///
/// Returns the byte offset immediately after the written value.
#[inline]
pub(crate) fn write_number_to_cell(
    cell: &SharedCell<Vec<u8>>,
    offset: i64,
    kind: NativeType,
    number: &Value,
    line: u32,
) -> Result<usize, RuntimeFailure> {
    let start = start_of(offset, line)?;
    let width = kind.width();
    let end = start.saturating_add(width);
    let mut block = cell.borrow_mut();
    if end > block.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    let mut bytes = [0u8; 8];
    let len = number_bytes_into(number, kind, &mut bytes);
    block[start..end].copy_from_slice(&bytes[..len]);
    Ok(end)
}

/// Write a single number into a known buffer cell when `address` falls inside it.
///
/// Returns `None` if `address` is outside `cell` (caller should re-probe
/// `memory::find_mirror_cell`).
#[inline]
pub(crate) fn write_number_at_cell(
    cell: &SharedCell<Vec<u8>>,
    address: i64,
    offset: i64,
    kind: NativeType,
    number: &Value,
    line: u32,
) -> Option<Result<i64, RuntimeFailure>> {
    let mut block = cell.borrow_mut();
    let base = block.as_ptr() as i64;
    let end_addr = base + block.len() as i64;
    if address < base || address >= end_addr {
        return None;
    }
    let target = (address - base).checked_add(offset);
    let Some(start) = target.filter(|&n| n >= 0).map(|n| n as usize) else {
        return Some(Err(fail(
            "the offset of a number cannot be negative.",
            line,
        )));
    };
    let width = kind.width();
    let end = start.saturating_add(width);
    if end > block.len() {
        return Some(Err(fail("the bytes do not fit in the buffer.", line)));
    }
    let mut bytes = [0u8; 8];
    let len = number_bytes_into(number, kind, &mut bytes);
    block[start..end].copy_from_slice(&bytes[..len]);
    Some(Ok(base + end as i64))
}

/// Read a number directly from the mirror holding `address` at `address + offset`.
///
/// Returns `None` if no buffer mirror covers `address`.
pub(crate) fn read_number_at_address(
    address: i64,
    offset: i64,
    kind: NativeType,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    #[cfg(feature = "std")]
    {
        crate::memory::read_bytes_at(address, offset, kind.width(), |slice| {
            number_from_bytes(slice, kind)
        })
        .map(|result| result.map_err(|message| fail(message, line)))
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = (address, offset, kind, line);
        None
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
    let start = start_of(offset, line)?;
    let end = start.saturating_add(kind.width());
    // The bytes are read out of the cell of the buffer, and not out of a
    // copy of them: the copy of a buffer of a megabyte is a megabyte, and
    // a script that walks the memory of the machine reads the buffer
    // thousands of times.
    //
    // Measured on the binary of v2.0.32: the `NumGet` of a buffer of a
    // megabyte is read a thousand times in a second.
    let cell = the_cell(value, line)?;
    let block = cell.borrow();
    if end > block.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    Ok(number_from_bytes(&block[start..end], kind))
}

/// The buffer whose mirror holds the address, and the distance from the
/// start of it.
///
/// The v2 reads the memory of an address, and the tree reads the buffer
/// whose mirror holds it: measured on the binary of v2.0.32, the
/// `NumGet(Buf.Ptr + 4, 0, "UInt")` reads the number at the fourth byte
/// of the buffer.
pub(crate) fn the_buffer_at(address: i64, line: u32) -> Option<(Value, i64)> {
    // The bytes of the object are the ones the machine left in them: they
    // are read back after every native call, and the write of the script
    // keeps the mirror of them current.
    //
    // The memory of the machine is not there without the standard
    // library: the tree does not read the memory of an address there.
    #[cfg(feature = "std")]
    let found = crate::memory::the_mirror_at(address);
    #[cfg(not(feature = "std"))]
    let found: Option<(Value, usize)> = None;
    let _ = address;
    let (owner, distance) = found?;
    let _ = line;
    Some((owner, distance as i64))
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
    // The bytes of all the numbers are measured before any of them is
    // written: a write that does not fit leaves the buffer as it was.
    let mut total = 0usize;
    for (kind, _) in numbers {
        total += kind.width();
    }
    let end = offset.saturating_add(total as i64);
    if end > size_of(value, line)? {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    // Each number goes out where the previous one stopped, and the bytes of
    // it do not need a block of their own.
    let mut at = offset;
    let mut bytes = [0u8; 8];
    for (kind, number) in numbers {
        let len = number_bytes_into(number, *kind, &mut bytes);
        write_at(value, at, &bytes[..len], line)?;
        at += len as i64;
    }
    // The answer of v2 is the address to the right of the last number:
    // the address of the buffer and the offset it stopped at.
    Ok(ptr_of(value, line)? + at)
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

/// The cell of the bytes of a buffer.
///
/// The bytes live in a cell that the script and the machine share: the
/// address of the cell is the `Ptr` of the buffer, and what the machine
/// writes there is what `NumGet` reads.
fn the_cell(this: &Value, line: u32) -> Result<SharedCell<Vec<u8>>, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a buffer.", line));
    };
    match object.borrow().value_of(STATE) {
        Some(Value::Bytes(cell)) => Ok(cell),
        _ => Err(fail("this value is not a buffer.", line)),
    }
}

/// The state of a buffer, as the property table holds it.
///
/// The bytes are held behind a reference: the machine writes into them,
/// and the script reads them, without a copy in between.
fn stored(bytes: Vec<u8>) -> Value {
    Value::Bytes(SharedCell::new(bytes))
}

/// Put the state back into the object.
///
/// The cell is made again, so the address of the buffer is another one:
/// the v2 says so of a buffer that is reallocated.
fn restore(this: &Value, bytes: Vec<u8>) {
    let Value::Object(object) = this else { return };
    put(
        &mut object.borrow_mut().properties,
        STATE,
        Value::Bytes(SharedCell::new(bytes)),
    );
}

/// A new buffer: the bytes of it.
fn new(arguments: &[Value], line: u32) -> Result<Vec<u8>, RuntimeFailure> {
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
    // Measured on the binary of v2.0.19: the fill is the byte
    // of the number, and a number that is not a byte is not refused: the
    // byte of it is the fill.
    let fill = match arguments.get(1) {
        Some(value) => whole_argument(value, 1, line)? as u8,
        None => 0,
    };
    Ok(a_block_of(size as usize, fill))
}

/// A block of bytes: the fill of them, or zero.
///
/// A buffer of no bytes still has a memory of its own: the address of it
/// is the address of a block, and the machine may write into it.
fn a_block_of(size: usize, fill: u8) -> Vec<u8> {
    if size == 0 {
        return Vec::with_capacity(1);
    }
    vec![fill; size]
}

/// Make the buffer of another size, the way v2 does: the bytes that
/// stay are kept, and the bytes that come are zero.
///
/// The block is made again, so the address of the buffer is another one:
/// the v2 says so of a buffer that is reallocated.
fn resize(this: &Value, value: Value, line: u32) -> Result<(), RuntimeFailure> {
    let size = whole(&value, line)?;
    if size < 0 {
        return Err(fail("the size of a buffer cannot be negative.", line));
    }
    let cell = the_cell(this, line)?;
    let mut block = cell.borrow_mut();
    let mut bytes = a_block_of(size as usize, 0);
    let keep = bytes.len().min(block.len());
    bytes[..keep].copy_from_slice(&block[..keep]);
    *block = bytes;
    Ok(())
}

/// Put the bytes into a buffer, at an offset.
///
/// The write of a number belongs to the module that owns the buffer,
/// the same way the read does.
fn write_at(value: &Value, offset: i64, bytes: &[u8], line: u32) -> Result<(), RuntimeFailure> {
    let cell = the_cell(value, line)?;
    let start = start_of(offset, line)?;
    let end = start.saturating_add(bytes.len());
    let mut block = cell.borrow_mut();
    if end > block.len() {
        return Err(fail("the bytes do not fit in the buffer.", line));
    }
    block[start..end].copy_from_slice(bytes);
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
pub(crate) fn number_bytes(number: &Value, kind: NativeType) -> Vec<u8> {
    let mut bytes = [0u8; 8];
    let len = number_bytes_into(number, kind, &mut bytes);
    bytes[..len].to_vec()
}

/// The bytes of a number, in the width its type asks for, into a block of
/// the caller.
///
/// The width of a number is eight bytes at the most, so the block of it
/// fits in the stack: a write of a number does not need a block of its own
/// on the heap.
#[inline]
pub(crate) fn number_bytes_into(
    number: &Value,
    kind: NativeType,
    out: &mut [u8; 8],
) -> usize {
    let width = kind.width();
    match kind {
        NativeType::Float => {
            out[..4].copy_from_slice(&(number.to_number().unwrap_or(0.0) as f32).to_le_bytes());
        }
        NativeType::Double => {
            out[..8].copy_from_slice(&number.to_number().unwrap_or(0.0).to_le_bytes());
        }
        _ => {
            let whole = match number {
                Value::Int(number) => *number,
                other => other.to_number().unwrap_or(0.0) as i64,
            };
            match width {
                1 => out[0] = whole as u8,
                2 => out[..2].copy_from_slice(&(whole as u16).to_le_bytes()),
                4 => out[..4].copy_from_slice(&(whole as u32).to_le_bytes()),
                _ => out[..8].copy_from_slice(&whole.to_le_bytes()),
            }
        }
    }
    width
}

/// The number of the bytes, read in the width its type asks for.
///
/// The bytes come in little-endian, the way they went out, and the type
/// says whether the number is signed: the sign is in the last byte of
/// the width.
#[inline]
pub(crate) fn number_from_bytes(bytes: &[u8], kind: NativeType) -> Value {
    match kind {
        NativeType::Char => Value::Int(bytes[0] as i8 as i64),
        NativeType::UChar => Value::Int(bytes[0] as i64),
        NativeType::Short => Value::Int(i16::from_le_bytes([bytes[0], bytes[1]]) as i64),
        NativeType::UShort => Value::Int(u16::from_le_bytes([bytes[0], bytes[1]]) as i64),
        NativeType::Int => {
            Value::Int(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64)
        }
        NativeType::UInt => {
            Value::Int(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64)
        }
        NativeType::Float => {
            Value::Float(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64)
        }
        NativeType::Double => Value::Float(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])),
        _ => Value::Int(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])),
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
            // Measured on the binary of v2.0.19: the own
            // properties of v2 are an enumerator, and it gives the name
            // when the caller wants one value.
            let pairs: Vec<(Value, Value)> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(_, property)| (Value::Text(property.name.clone().into()), property.value.clone()))
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheKey)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str())
                // The methods of the class are the properties of the value.
                //
                // Measured on the binary of v2.0.19:
                // `m.HasProp("Get")` answers 1.
                || METHODS.contains(&key.as_str())
                || own_property(object, &key)
                // Every value carries its class, and the base object has
                // the property of it.
                || key == "__class"
                // Every class has an initialiser, and every object has a
                // base.
                //
                // Measured on the binary of v2.0.19:
                // `m.HasProp("__Init")` and `m.HasProp("Base")` answer 1.
                || key == "__init"
                || key == "base") as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(
            (METHODS.contains(&key.as_str())
                // Every class has an initialiser.
                //
                // Measured on the binary of v2.0.19:
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
                    // Measured on the binary of v2.0.19: the
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
                    Some(Ok(Value::Object(SharedCell::new(Object {
                        class: "object".to_string(),
                        properties: table,
                    }))))
                }
                // The property is not there: v2 says so.
                None => Some(Err(rt.no_property(this, &name, line))),
            }
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of a buffer is not a property of it.");
            }
            // Measured on the binary of v2.0.19: the second
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
                return Some(Ok(Value::Text("".into())));
            }
            let gone = object
                .borrow_mut()
                .properties
                .remove(&key)
                .map(|property| property.value);
            // Measured on the binary of v2.0.19: the answer
            // is the value of the property that is taken away, and an
            // empty text when there was none.
            Some(Ok(match gone {
                None => Value::Text("".into()),
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
fn own_property(object: &SharedCell<Object>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}
