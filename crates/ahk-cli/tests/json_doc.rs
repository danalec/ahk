//! Reading the JSON of the runner in the tests.
//!
//! The runner writes the report itself, and a test has to read it back to say
//! what the report holds. The reader is the one the runner carries, so the
//! two halves of the format are one implementation here and one in the tree.
//!
//! A value is asked by name and by place: `document.get("names")`, and
//! `value.at(0)`. The reading is borrowed from the document, which is what a
//! test wants: the document lives for the case, and the borrow says so.

#![allow(dead_code)]

use ahk_cli::json::{self, Read};

/// The text under a name of a value, which the value must have.
///
/// The tests ask a row of the table for its name and its state, and a row
/// that does not carry one is a report that is wrong rather than a test that
/// is cautious: the panic says which.
pub fn field_text<'a>(value: &'a Read, name: &str) -> &'a str {
    value
        .field_text(name)
        .unwrap_or_else(|| panic!("the value has no text {name:?}"))
}

/// A document, read.
pub struct Json {
    root: Read,
}

impl Json {
    /// Read the text as a document.
    pub fn parse(text: &str) -> Json {
        Json {
            root: json::read(text).expect("the report is json"),
        }
    }

    /// The value under a name of the root.
    pub fn get(&self, name: &str) -> &Read {
        self.root
            .get(name)
            .unwrap_or_else(|| panic!("the report has no field {name:?}"))
    }

    /// The text under a name of the root.
    pub fn text(&self, name: &str) -> &str {
        self.get(name)
            .as_str()
            .unwrap_or_else(|| panic!("the field {name:?} is not a text"))
    }

    /// The whole number under a name of the root.
    pub fn number(&self, name: &str) -> i64 {
        self.get(name)
            .as_i64()
            .unwrap_or_else(|| panic!("the field {name:?} is not a number"))
    }

    /// The truth value under a name of the root.
    pub fn boolean(&self, name: &str) -> bool {
        self.get(name)
            .as_bool()
            .unwrap_or_else(|| panic!("the field {name:?} is not a truth value"))
    }

    /// The array under a name of the root.
    pub fn array(&self, name: &str) -> &[Read] {
        self.get(name).array()
    }

    /// The names of the fields of the document.
    pub fn names(&self) -> Vec<&str> {
        self.root.names()
    }
}

#[cfg(test)]
mod tests {
    use super::Json;

    /// A report is read, and a value is asked by name and by place.
    #[test]
    fn a_report_is_read() {
        let document = Json::parse("{\"ok\":true,\"names\":[\"a\",\"b\"],\"line\":3}");
        assert!(document.boolean("ok"));
        assert_eq!(document.number("line"), 3);
        let names = document.array("names");
        assert_eq!(names.len(), 2);
        assert_eq!(names[1].as_str(), Some("b"));
        assert!(document.names().contains(&"ok"));
    }

    /// A text with an escape is read back as the text it spells.
    #[test]
    fn a_text_with_an_escape_is_read_back() {
        let document = Json::parse("{\"error\":\"a\\\"b\\\\c\\nd\"}");
        assert_eq!(document.text("error"), "a\"b\\c\nd");
    }
}
