//! The JSON the runner writes, and the one it reads back.
//!
//! The report is a fixed shape -- the fields the notes of the runner name --
//! and it is written here field by field rather than through a value tree:
//! the shape is known at the point of writing, so a tree would be built only
//! to be walked again.
//!
//! The reading is the other half, and a test is what asks for it: a case runs
//! the runner, takes the report off its standard output, and asks the report
//! what it holds. The reader answers with a tree of its own, which is small
//! on purpose: the format has six kinds of value, and a report carries all of
//! them.
//!
//! The escaping is the one of the format: the characters that must be
//! escaped are escaped, and every other character goes through as it is.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A builder for one JSON object.
///
/// It writes the opening brace when the first field arrives, so an object
/// that has no field is `{}` and not a broken one.
pub struct Object {
    text: String,
    first: bool,
}

impl Object {
    /// A new object, with room for `fields` of them.
    pub fn new(capacity: usize) -> Object {
        Object {
            text: String::with_capacity(capacity),
            first: true,
        }
    }

    /// Write the name of a field and its value, which the caller writes.
    ///
    /// The value is written by the closure into the same buffer, so a field
    /// that is an array or another object nests without a value tree in
    /// between.
    pub fn field<F>(&mut self, name: &str, value: F) -> &mut Object
    where
        F: FnOnce(&mut String),
    {
        if self.first {
            self.text.push('{');
            self.first = false;
        } else {
            self.text.push(',');
        }
        write_name(&mut self.text, name);
        self.text.push(':');
        value(&mut self.text);
        self
    }

    /// Write a field whose value is a text.
    pub fn text(&mut self, name: &str, value: &str) -> &mut Object {
        self.field(name, |out| write_text(out, value))
    }

    /// Write a field whose value is a whole number.
    pub fn number(&mut self, name: &str, value: i64) -> &mut Object {
        self.field(name, |out| {
            let _ = write!(out, "{value}");
        })
    }

    /// Write a field whose value is a truth value.
    pub fn boolean(&mut self, name: &str, value: bool) -> &mut Object {
        self.field(name, |out| {
            out.push_str(if value { "true" } else { "false" });
        })
    }

    /// Write a field whose value is null.
    pub fn nothing(&mut self, name: &str) -> &mut Object {
        self.field(name, |out| out.push_str("null"))
    }

    /// Write a field whose value is a whole number, or null when it is not
    /// there.
    pub fn optional_number(&mut self, name: &str, value: Option<i64>) -> &mut Object {
        match value {
            Some(value) => self.number(name, value),
            None => self.nothing(name),
        }
    }

    /// Write a field whose value is an array of texts.
    pub fn texts(&mut self, name: &str, values: &[String]) -> &mut Object {
        self.field(name, |out| {
            out.push('[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_text(out, value);
            }
            out.push(']');
        })
    }

    /// The object, as JSON.
    pub fn finish(self) -> String {
        if self.first {
            "{}".to_string()
        } else {
            self.text + "}"
        }
    }
}

/// Write the name of a field: the text, quoted and escaped.
fn write_name(out: &mut String, name: &str) {
    write_text(out, name);
}

/// Write a text as a JSON text, with the quotes.
///
/// The escapement is the one of the format: the quote and the reverse solidus
/// are spelled out, and so are the control characters, which have no spelling
/// of their own. Everything else -- including the letters that are not ASCII
/// -- goes through as it is, which the format allows and which keeps a report
/// of a script in the language the script was written in.
pub fn write_text(out: &mut String, value: &str) {
    out.push('"');
    for letter in value.chars() {
        match letter {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            other if (other as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", other as u32);
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::{Object, write_text};

    /// A text is quoted, and the characters the format spells out are the
    /// ones that are spelled out.
    #[test]
    fn a_text_is_quoted_and_escaped() {
        let mut out = String::new();
        write_text(&mut out, "a\"b\\c");
        assert_eq!(out, "\"a\\\"b\\\\c\"");
    }

    /// An object with no field is `{}`.
    #[test]
    fn an_object_with_no_field_is_empty() {
        assert_eq!(Object::new(8).finish(), "{}");
    }

    /// The fields are in the order they are written, and each one is
    /// separated from the one before it.
    #[test]
    fn the_fields_are_written_in_the_order_they_arrive() {
        let mut object = Object::new(32);
        object
            .boolean("ok", true)
            .number("line", 3)
            .nothing("error");
        assert_eq!(object.finish(), "{\"ok\":true,\"line\":3,\"error\":null}");
    }
}

/// A value of a document, read.
#[derive(Debug, Clone, PartialEq)]
pub enum Read {
    /// The `null` of the format.
    Null,
    /// A truth value.
    Bool(bool),
    /// A number, kept as the text of it: the format has no limit on the
    /// digits, and a report asks a number for what it is and not for how it is
    /// spelled.
    Number(String),
    /// A text.
    Text(String),
    /// An array.
    Array(Vec<Read>),
    /// An object, in the order of the names it arrived in.
    Object(BTreeMap<String, Read>),
}

impl Read {
    /// The value under a name, when this one is an object that has it.
    pub fn get(&self, name: &str) -> Option<&Read> {
        match self {
            Read::Object(members) => members.get(name),
            _ => None,
        }
    }

    /// The value at a place, when this one is an array that has it.
    pub fn at(&self, place: usize) -> Option<&Read> {
        match self {
            Read::Array(values) => values.get(place),
            _ => None,
        }
    }

    /// The values of an array.
    pub fn array(&self) -> &[Read] {
        match self {
            Read::Array(values) => values,
            _ => &[],
        }
    }

    /// The names of the members of an object, in the order of the document.
    pub fn names(&self) -> Vec<&str> {
        match self {
            Read::Object(members) => members.keys().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }

    /// The text of the value, when it is a text.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Read::Text(text) => Some(text),
            _ => None,
        }
    }

    /// The text under a name, when the member is a text.
    pub fn field_text(&self, name: &str) -> Option<&str> {
        self.get(name).and_then(Read::as_str)
    }

    /// The value as a whole number.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Read::Number(text) => text.parse().ok(),
            _ => None,
        }
    }

    /// The value as a truth value.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Read::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Whether the value is a number of any kind.
    pub fn is_number(&self) -> bool {
        matches!(self, Read::Number(_))
    }

    /// Whether the value is a text.
    pub fn is_string(&self) -> bool {
        matches!(self, Read::Text(_))
    }

    /// Whether the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Read::Null)
    }
}

/// The reason a document could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadError {
    /// What went wrong, in a sentence.
    pub reason: String,
    /// Where in the text it went wrong.
    pub at: usize,
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at byte {}", self.reason, self.at)
    }
}

impl std::error::Error for ReadError {}

/// Read a document.
pub fn read(text: &str) -> Result<Read, ReadError> {
    let mut parser = Reader {
        bytes: text.as_bytes(),
        at: 0,
    };
    let value = parser.value()?;
    parser.space();
    if parser.at != parser.bytes.len() {
        return Err(parser.the_error("the document ends before its value does"));
    }
    Ok(value)
}

/// The reader of one document.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    /// The reason of a failure, at the place the reader is.
    fn the_error(&self, reason: &str) -> ReadError {
        ReadError {
            reason: reason.to_string(),
            at: self.at,
        }
    }

    /// Skip the spaces the format allows between the values.
    fn space(&mut self) {
        while let Some(byte) = self.bytes.get(self.at) {
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\r') {
                self.at += 1;
            } else {
                break;
            }
        }
    }

    /// Read one value.
    fn value(&mut self) -> Result<Read, ReadError> {
        self.space();
        match self.bytes.get(self.at) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Read::Text(self.text()?)),
            Some(b't') => self.word("true", Read::Bool(true)),
            Some(b'f') => self.word("false", Read::Bool(false)),
            Some(b'n') => self.word("null", Read::Null),
            Some(byte) if *byte == b'-' || byte.is_ascii_digit() => self.number(),
            Some(_) => Err(self.the_error("the document holds a character that begins no value")),
            None => Err(self.the_error("the document ends where a value was wanted")),
        }
    }

    /// Read the word that is there, which must be the one named.
    fn word(&mut self, word: &str, answer: Read) -> Result<Read, ReadError> {
        if self.bytes[self.at..].starts_with(word.as_bytes()) {
            self.at += word.len();
            Ok(answer)
        } else {
            Err(self.the_error("the document holds a word that is not a value of the format"))
        }
    }

    /// Read an object.
    fn object(&mut self) -> Result<Read, ReadError> {
        self.at += 1;
        let mut members = BTreeMap::new();
        self.space();
        if self.bytes.get(self.at) == Some(&b'}') {
            self.at += 1;
            return Ok(Read::Object(members));
        }
        loop {
            self.space();
            if self.bytes.get(self.at) != Some(&b'"') {
                return Err(self.the_error("a name of an object is not a text"));
            }
            let name = self.text()?;
            self.space();
            if self.bytes.get(self.at) != Some(&b':') {
                return Err(self.the_error("a name of an object is not followed by a colon"));
            }
            self.at += 1;
            let value = self.value()?;
            members.insert(name, value);
            self.space();
            match self.bytes.get(self.at) {
                Some(b',') => self.at += 1,
                Some(b'}') => {
                    self.at += 1;
                    return Ok(Read::Object(members));
                }
                _ => return Err(self.the_error("an object ends without its closing brace")),
            }
        }
    }

    /// Read an array.
    fn array(&mut self) -> Result<Read, ReadError> {
        self.at += 1;
        let mut values = Vec::new();
        self.space();
        if self.bytes.get(self.at) == Some(&b']') {
            self.at += 1;
            return Ok(Read::Array(values));
        }
        loop {
            values.push(self.value()?);
            self.space();
            match self.bytes.get(self.at) {
                Some(b',') => self.at += 1,
                Some(b']') => {
                    self.at += 1;
                    return Ok(Read::Array(values));
                }
                _ => return Err(self.the_error("an array ends without its closing bracket")),
            }
        }
    }

    /// Read a text, with its quotes.
    fn text(&mut self) -> Result<String, ReadError> {
        self.at += 1;
        let mut out = String::new();
        loop {
            let byte = *self
                .bytes
                .get(self.at)
                .ok_or_else(|| self.the_error("a text ends before its closing quote"))?;
            match byte {
                b'"' => {
                    self.at += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.at += 1;
                    let escape = *self
                        .bytes
                        .get(self.at)
                        .ok_or_else(|| self.the_error("a text ends in the middle of an escape"))?;
                    self.at += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let place = self.at;
                            let digits = self.bytes.get(place..place + 4).ok_or_else(|| {
                                self.the_error("an escape of four digits is short")
                            })?;
                            let hex = std::str::from_utf8(digits)
                                .map_err(|_| self.the_error("an escape is not four digits"))?;
                            let code = u32::from_str_radix(hex, 16)
                                .map_err(|_| self.the_error("an escape is not four digits"))?;
                            self.at += 4;
                            out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                        }
                        _ => {
                            return Err(
                                self.the_error("a text holds an escape the format does not define")
                            );
                        }
                    }
                }
                other => {
                    // A text of the format is UTF-8, and the bytes of a
                    // letter that is not ASCII are taken as they are.
                    let rest = std::str::from_utf8(&self.bytes[self.at..])
                        .map_err(|_| self.the_error("a text is not a text of the format"))?;
                    let letter = rest
                        .chars()
                        .next()
                        .ok_or_else(|| self.the_error("a text ends where a letter was wanted"))?;
                    out.push(letter);
                    self.at += letter.len_utf8();
                    if other == 0 {
                        return Err(self.the_error("a text holds a byte that is not of the format"));
                    }
                }
            }
        }
    }

    /// Read a number, and keep the text of it.
    fn number(&mut self) -> Result<Read, ReadError> {
        let start = self.at;
        if self.bytes.get(self.at) == Some(&b'-') {
            self.at += 1;
        }
        while let Some(byte) = self.bytes.get(self.at) {
            if byte.is_ascii_digit() || matches!(byte, b'.' | b'e' | b'E' | b'+' | b'-') {
                self.at += 1;
            } else {
                break;
            }
        }
        let text = std::str::from_utf8(&self.bytes[start..self.at])
            .map_err(|_| self.the_error("a number is not a number of the format"))?;
        if text.is_empty() || text == "-" {
            return Err(self.the_error("a number holds no digits"));
        }
        Ok(Read::Number(text.to_string()))
    }
}
