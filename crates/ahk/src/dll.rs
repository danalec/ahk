//! The marshalling of a call into a library of the machine.
//!
//! The `DllCall` of v2 is a call into a library: the library and the
//! function are written in one string, the type of every argument goes
//! between them, and the type of the answer goes at the end. What is
//! here is the marshalling of the values: it is pure, so it belongs to
//! this crate. The call itself is I/O, and it belongs to the desktop.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// The type of a value that goes into a native call, or that comes out
/// of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeType {
    /// A signed integer of four bytes.
    Int,
    /// A signed integer of eight bytes.
    Int64,
    /// A signed integer of two bytes.
    Short,
    /// A signed integer of one byte.
    Char,
    /// An unsigned integer of one byte.
    UChar,
    /// An unsigned integer of two bytes.
    UShort,
    /// An unsigned integer of four bytes.
    UInt,
    /// An unsigned integer of eight bytes.
    UInt64,
    /// A float of four bytes.
    Float,
    /// A float of eight bytes.
    Double,
    /// A pointer.
    Ptr,
    /// A pointer that is not signed.
    UPtr,
    /// A string, of two bytes a character.
    Str,
    /// A string, of two bytes a character.
    WStr,
    /// A string, of one byte a character.
    AStr,
}

impl NativeType {
    /// The width of the value, in bytes.
    ///
    /// A text is a pointer when it goes into the call, and the width of
    /// its characters when it comes out of it.
    pub fn width(&self) -> usize {
        match self {
            NativeType::Char | NativeType::UChar => 1,
            NativeType::Short | NativeType::UShort => 2,
            NativeType::Int | NativeType::UInt | NativeType::Float => 4,
            _ => 8,
        }
    }

    /// Whether the value is a float.
    pub fn is_float(&self) -> bool {
        matches!(self, NativeType::Float | NativeType::Double)
    }

    /// Whether the value is a text.
    pub fn is_text(&self) -> bool {
        matches!(self, NativeType::Str | NativeType::WStr | NativeType::AStr)
    }

    /// Whether the value is a whole number.
    pub fn is_integer(&self) -> bool {
        !self.is_float() && !self.is_text()
    }

    /// Whether the value is a whole number that does not go negative.
    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            NativeType::UChar
                | NativeType::UShort
                | NativeType::UInt
                | NativeType::UInt64
                | NativeType::UPtr
        )
    }

    /// The width of a character of the text, in bytes.
    pub fn character_width(&self) -> usize {
        match self {
            NativeType::AStr => 1,
            _ => 2,
        }
    }
}

/// The convention of a native call.
///
/// The convention of the platform is the one of x64, and the two are the
/// same there: a call of thirty-two bits is the only one where they
/// differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Convention {
    /// The convention of the platform.
    Standard,
    /// The convention of C.
    Cdecl,
}

/// The value of an argument of a native call.
#[derive(Debug, Clone, PartialEq)]
pub enum NativeValue {
    /// A whole number, or a pointer.
    Integer(i64),
    /// A float.
    Float(f64),
    /// The bytes of a text, or of a block the callee may write into.
    Bytes(Vec<u8>),
}

/// One argument of a native call.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeArg {
    /// The type of the value.
    pub kind: NativeType,
    /// Whether the argument is the address of the value.
    ///
    /// v2 writes it with a star at the end of the type, and the value of
    /// the variable comes back from the call. The desktop owns the
    /// memory of that address, and it puts the bytes the callee left
    /// there into `value`, so that the language can read them back into
    /// the variable of the script.
    pub address: bool,
    /// The value that goes.
    ///
    /// An argument that is the address of a value holds the value
    /// itself, not the address: the address is the desktop's business.
    pub value: NativeValue,
}

/// A call into a library of the machine.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeCall {
    /// The library: a name, or the path of a file.
    pub library: String,
    /// The name of the function.
    pub function: String,
    /// The convention of the call.
    pub convention: Convention,
    /// The type of the answer.
    pub returns: NativeType,
    /// The arguments, in the order they are given.
    ///
    /// The desktop may write into them: the argument that is the
    /// address of a variable comes back with the bytes the callee left
    /// in the memory of that address.
    pub arguments: Vec<NativeArg>,
}

/// The answer of a native call.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeResult {
    /// The type of the answer.
    pub kind: NativeType,
    /// The answer: a whole number, a float, or the bytes of a text.
    ///
    /// The bytes of a text are the bytes of the memory of the callee,
    /// in the encoding the type asks for: the language reads them.
    pub value: NativeValue,
}

/// The kind of a failure of a native call.
///
/// The kind says what failed: the call itself, the library, or the function
/// in the library. The reason of the failure is the one of the machine, and
/// the v1 turns the kind into the code that it writes in the `ErrorLevel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeFailureKind {
    /// The call itself is not valid: the target, the type of an argument or
    /// the type of the answer.
    Invalid,
    /// The library is not there.
    NoLibrary,
    /// The function is not in the library.
    NoFunction,
}

/// The failure of a native call.
///
/// The reason is the one of the machine, in the words of the v2: "Failed to
/// load DLL." and "Call to nonexistent function.".
#[derive(Debug, Clone, PartialEq)]
pub struct NativeFailure {
    /// The kind of the failure.
    pub kind: NativeFailureKind,
    /// The reason of the failure.
    pub reason: String,
}

impl NativeFailure {
    /// The failure of a call that is not valid.
    pub fn invalid(reason: &str) -> Self {
        NativeFailure {
            kind: NativeFailureKind::Invalid,
            reason: reason.to_string(),
        }
    }

    /// The failure of a library that is not there.
    pub fn no_library(reason: &str) -> Self {
        NativeFailure {
            kind: NativeFailureKind::NoLibrary,
            reason: reason.to_string(),
        }
    }

    /// The failure of a function that is not in the library.
    pub fn no_function(reason: &str) -> Self {
        NativeFailure {
            kind: NativeFailureKind::NoFunction,
            reason: reason.to_string(),
        }
    }
}

/// The type of an argument, read out of the string the script writes.
///
/// v2 writes the name of the type, with a `U` in front of the whole
/// ones and a `*` or a `P` at the end for the address of the value.
/// `None` means the string is not a type.
pub fn parse_type(text: &str) -> Option<(NativeType, bool)> {
    let trimmed = text.trim();
    let (body, address) = match trimmed
        .strip_suffix('*')
        .or_else(|| trimmed.strip_suffix('P'))
    {
        Some(body) => (body.trim(), true),
        None => (trimmed, false),
    };
    let kind = match body.to_ascii_lowercase().as_str() {
        "int" => NativeType::Int,
        "int64" => NativeType::Int64,
        "short" => NativeType::Short,
        "char" => NativeType::Char,
        "uchar" => NativeType::UChar,
        "ushort" => NativeType::UShort,
        "uint" => NativeType::UInt,
        "uint64" => NativeType::UInt64,
        "float" => NativeType::Float,
        "double" => NativeType::Double,
        "ptr" => NativeType::Ptr,
        "uptr" => NativeType::UPtr,
        "str" => NativeType::Str,
        "wstr" => NativeType::WStr,
        "astr" => NativeType::AStr,
        _ => return None,
    };
    Some((kind, address))
}

/// The library and the function, read out of the string the script
/// writes.
///
/// v2 writes them in one string: `kernel32\GetLastError`. A path with
/// a drive letter has its backslashes in it: the name of the function is
/// behind the last one. A string without a backslash is a function of
/// the program itself, and the library is empty.
pub fn split_target(text: &str) -> (String, String) {
    let trimmed = text.trim();
    match trimmed.rsplit_once('\\') {
        Some((library, function)) => (library.to_string(), function.trim().to_string()),
        None => (String::new(), trimmed.to_string()),
    }
}

/// The bytes of a text, in the encoding its type asks for.
///
/// The strings of v2 are of two bytes a character, and the tree keeps
/// them as UTF-8: the conversion happens here, on the way out. The
/// `AStr` of v2 is the codepage of Windows, and here it is the encoding
/// of the tree.
pub fn encode(text: &str, kind: NativeType) -> Vec<u8> {
    let mut bytes = Vec::new();
    match kind {
        NativeType::AStr => {
            bytes.extend_from_slice(text.as_bytes());
            bytes.push(0);
        }
        _ => {
            for unit in text.encode_utf16() {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
            bytes.extend_from_slice(&[0, 0]);
        }
    }
    bytes
}

/// The text of the bytes, in the encoding its type asks for.
///
/// The text of a native call ends at the first nul: the bytes that
/// come after it are not part of the text, and a buffer the callee
/// wrote into may hold anything after it.
pub fn decode(bytes: &[u8], kind: NativeType) -> String {
    match kind {
        NativeType::AStr => {
            let end = bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len());
            String::from_utf8_lossy(&bytes[..end]).into_owned()
        }
        _ => {
            let mut units: Vec<u16> = Vec::new();
            let mut index = 0;
            while index + 1 < bytes.len() {
                let unit = u16::from_le_bytes([bytes[index], bytes[index + 1]]);
                if unit == 0 {
                    break;
                }
                units.push(unit);
                index += 2;
            }
            String::from_utf16_lossy(&units)
        }
    }
}

/// The bytes of a number, in the width its type asks for.
///
/// The numbers go out little-endian, the way the machines of x86 read
/// them.
pub fn encode_number(number: f64, kind: NativeType) -> Vec<u8> {
    let mut bytes = Vec::new();
    match kind {
        NativeType::Float => bytes.extend_from_slice(&(number as f32).to_le_bytes()),
        NativeType::Double => bytes.extend_from_slice(&number.to_le_bytes()),
        _ => {
            let whole = number as i64;
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
pub fn decode_number(bytes: &[u8], kind: NativeType) -> f64 {
    match kind {
        NativeType::Float => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
        NativeType::Double => {
            let mut whole = [0u8; 8];
            whole.copy_from_slice(&bytes[..8]);
            f64::from_le_bytes(whole)
        }
        _ => {
            let mut whole = 0i64;
            // The bytes go out little-endian, and they come back the
            // same way: the last byte is the first of the number.
            for byte in bytes.iter().take(kind.width()).rev() {
                whole = (whole << 8) | *byte as i64;
            }
            // The type says whether the number is signed, and the sign
            // is in the last byte of the width.
            let bits = kind.width() * 8;
            if bits < 64 && !kind.is_unsigned() {
                let sign = 1i64 << (bits - 1);
                if whole & sign != 0 {
                    whole -= sign << 1;
                }
            }
            whole as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The numbers of the types that carry one.
    const WHOLE: [NativeType; 8] = [
        NativeType::Int,
        NativeType::Int64,
        NativeType::Short,
        NativeType::Char,
        NativeType::UInt,
        NativeType::UInt64,
        NativeType::UShort,
        NativeType::UChar,
    ];

    /// A number goes out and comes back.
    #[test]
    fn a_number_goes_out_and_comes_back() {
        for kind in WHOLE {
            let bytes = encode_number(42.0, kind);
            assert_eq!(bytes.len(), kind.width(), "{kind:?}");
            assert_eq!(decode_number(&bytes, kind), 42.0, "{kind:?}");
        }
    }

    /// A number that does not fit in the width is cut to it.
    #[test]
    fn a_negative_number_comes_back_negative() {
        let bytes = encode_number(-1.0, NativeType::Int);
        assert_eq!(decode_number(&bytes, NativeType::Int), -1.0);
        let bytes = encode_number(-1.0, NativeType::Int64);
        assert_eq!(decode_number(&bytes, NativeType::Int64), -1.0);
    }

    /// A whole number that does not go negative stays where it is.
    #[test]
    fn an_unsigned_number_stays_positive() {
        let bytes = encode_number(-1.0, NativeType::UInt);
        assert_eq!(decode_number(&bytes, NativeType::UInt), 4294967295.0);
        let bytes = encode_number(-1.0, NativeType::UChar);
        assert_eq!(decode_number(&bytes, NativeType::UChar), 255.0);
    }

    /// A float of four bytes and a float of eight.
    #[test]
    fn a_float_goes_out_and_comes_back() {
        let bytes = encode_number(1.5, NativeType::Float);
        assert_eq!(bytes.len(), 4);
        assert_eq!(decode_number(&bytes, NativeType::Float), 1.5);
        let bytes = encode_number(1.5, NativeType::Double);
        assert_eq!(bytes.len(), 8);
        assert_eq!(decode_number(&bytes, NativeType::Double), 1.5);
    }

    /// The bytes of a text are the ones of its encoding.
    #[test]
    fn a_text_goes_out_and_comes_back() {
        let bytes = encode("hello", NativeType::Str);
        assert_eq!(bytes.len(), 12);
        assert_eq!(decode(&bytes, NativeType::Str), "hello");
        let bytes = encode("hello", NativeType::AStr);
        assert_eq!(bytes.len(), 6);
        assert_eq!(decode(&bytes, NativeType::AStr), "hello");
    }

    /// The text ends at the first nul.
    #[test]
    fn a_text_ends_at_the_first_nul() {
        let bytes = encode("hello", NativeType::AStr);
        let mut longer = bytes.clone();
        longer.extend_from_slice(b"world");
        assert_eq!(decode(&longer, NativeType::AStr), "hello");
    }

    /// The type of an argument, and whether it is the address of one.
    #[test]
    fn a_type_is_read_out_of_its_string() {
        assert_eq!(parse_type("Int"), Some((NativeType::Int, false)));
        assert_eq!(parse_type("Int64*"), Some((NativeType::Int64, true)));
        assert_eq!(parse_type("Int64P"), Some((NativeType::Int64, true)));
        assert_eq!(parse_type("UInt"), Some((NativeType::UInt, false)));
        assert_eq!(parse_type("UPtr"), Some((NativeType::UPtr, false)));
        assert_eq!(parse_type("nope"), None);
        assert_eq!(parse_type("Bool"), None);
        assert_eq!(parse_type("Byte"), None);
    }

    /// The library and the function, out of the string of the script.
    #[test]
    fn the_target_is_read_out_of_its_string() {
        assert_eq!(
            split_target("kernel32\\GetLastError"),
            ("kernel32".into(), "GetLastError".into())
        );
        assert_eq!(
            split_target("GetLastError"),
            ("".into(), "GetLastError".into())
        );
        assert_eq!(
            split_target("C:\\dir\\file.dll\\Func"),
            ("C:\\dir\\file.dll".into(), "Func".into())
        );
    }
}
