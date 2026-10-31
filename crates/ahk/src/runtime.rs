//! The runtime: it executes a parsed script against a desktop.
//!
//! A port of the runtime of the archived Python implementation. The script
//! is compiled in memory and executed statement by statement. Nothing is
//! written to disk: the source is gone when the turn is over.
//!
//! Every statement produces a record -- what it did, whether it worked,
//! how long it took -- because a script that drives a desktop has to be
//! able to say what it did, not just that it finished.
//!
//! The desktop is whatever implements [`Desktop`]. The real one lives on
//! the lab box (the `ahk-box` crate); the tests use a fake that records
//! the calls.
//!
//! ## The focus check
//!
//! A script that types into the wrong window is worse than a script that
//! fails, so the language carries the primitives that answer "is the right
//! thing in front of us?":
//!
//! * `MouseGetPos()` -- where the pointer is.
//! * `PixelGet(x, y)` -- the colour under it.
//! * `PixelSearch(x1, y1, x2, y2, colour)` -- where that colour is.
//!
//! These three work on any desktop, because they are read from the screen
//! itself. `WinActive(title)` and `WinExist(title)` are the window-level
//! questions, and they are answered by whatever the desktop can actually
//! ask: on a Wayland session with no window API, the answer is
//! *unknown*, and the runtime says so out loud rather than guessing
//! `false`. A script that cannot answer the question fails; it does not
//! quietly take the wrong branch.

use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

use serde::Serialize;

use alloc::rc::Rc;
use core::cell::RefCell;

use crate::ast::{
    AccessBody, AccessDef, AssignOp, BinaryOp, DeclareKind, Declarations, DynamicPiece, LiteralValue, LoopKind,
    MemberName, MethodDef, Node, Parameter, PropertyDef, UnaryOp,
};

pub use crate::desktop::{Desktop, Files, WindowAnswer};

/// The variables of the loops, the ones that the runtime writes where it
/// runs.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: they are there
/// before the first loop runs, and the index is zero and the others are
/// empty.
const THE_LOOP_VARIABLES: &[&str] = &[
    "index",
    "loopfield",
    "loopreadline",
    "loopfilename",
    "loopfileext",
    "loopfilepath",
    "loopfilefullpath",
    "loopfiledir",
    "loopregname",
    "loopregtype",
    "loopregkey",
    "loopregtimemodified",
];

/// The type of a value.
///
/// AutoHotkey v2 has four of them, and every value carries the tag of
/// the one it is. The tag is what the diagnostics name, so the name
/// here and the name in the message are the same word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeName {
    /// Text.
    String,
    /// A whole number.
    Integer,
    /// A number with a point.
    Float,
    /// An object: a class instance, an array, a map.
    Object,
    /// A reference to a variable: `&x`.
    ///
    /// v2 names the kind a VarRef, and it is a value of its own: the
    /// operations that want a number or a text say that they got one.
    VarRef,
    /// A number: what the maths wants.
    ///
    /// No value carries this tag -- a whole number is an Integer and a
    /// number with a point is a Float -- but a parameter that wants a
    /// number names it in the message that says it got something else.
    Number,
    /// A class: what the `is` operator wants.
    ///
    /// The value of a class is an object of the tree, and the tag is
    /// what the message about the `is` operator names.
    Class,
    /// A value of COM.
    ///
    /// The tree carries none, and the name is here for the message of
    /// the dialect: "Parameter #1 of ComObjValue requires a ComValue,
    /// but received a VarRef.".
    ComValue,
}

impl TypeName {
    /// The name, the way the diagnostics spell it.
    ///
    /// The names are the ones of v2: a value carries the tag of its
    /// kind, and the tag is the word the diagnostics use.
    pub fn name(self) -> &'static str {
        match self {
            TypeName::String => "String",
            TypeName::Integer => "Integer",
            TypeName::Float => "Float",
            TypeName::Object => "Object",
            TypeName::VarRef => "VarRef",
            TypeName::Number => "Number",
            TypeName::Class => "Class",
            TypeName::ComValue => "ComValue",
        }
    }

    /// The name in lower case.
    ///
    /// v2 spells the kind of a value in two ways: the message about a
    /// built-in says "an Integer", and the message about a parameter of
    /// a function of the script says "an integer".
    pub fn lowercase(self) -> &'static str {
        match self {
            TypeName::String => "string",
            TypeName::Integer => "integer",
            TypeName::Float => "float",
            TypeName::Object => "object",
            TypeName::VarRef => "varref",
            TypeName::Number => "number",
            TypeName::Class => "class",
            TypeName::ComValue => "comvalue",
        }
    }

    /// The letter that goes with the name: "a string", "an integer".
    pub fn article(self) -> &'static str {
        match self {
            TypeName::Integer | TypeName::Object => "n",
            _ => "",
        }
    }

    /// The type as a sentence says it: "an integer".
    pub fn phrase(self) -> String {
        format!("a{} {}", self.article(), self.name())
    }
}

/// The message v2 gives when a value of the wrong type arrives.
///
/// The template is the one in the binary: "Expected a%s %s but got a%s
/// %s.". The two letters are the articles, and the article of an
/// integer or of an object is an "n".
pub fn expected_type(expected: TypeName, got: TypeName) -> String {
    format!(
        "Expected a{} {} but got a{} {}.",
        expected.article(),
        expected.name(),
        got.article(),
        got.name()
    )
}

/// The message v2 gives when a value of the wrong kind arrives.
///
/// The kind of the value that arrived is the name of its class: "an
/// Object", "an Array", "a Map", "a Func", "a Thing".
pub fn expected_the_kind(expected: TypeName, got: &str) -> String {
    format!(
        "Expected a{} {} but got a{} {}.",
        expected.article(),
        expected.name(),
        the_article_of(got),
        got
    )
}

/// The message v2 gives when the class of a value is not the one wanted.
///
/// The kind of the value is the name of its class: "Expected an Array but
/// got a Prototype.". The article follows the first letter of the name.
pub fn expected_the_class(expected: &str, got: &str) -> String {
    format!(
        "Expected a{} {} but got a{} {}.",
        the_article_of(expected),
        expected,
        the_article_of(got),
        got
    )
}

/// The article that goes with the name of a kind: "an Object", "a String".
fn the_article_of(name: &str) -> &'static str {
    match name.chars().next() {
        Some(first) if "AEIOU".contains(first.to_ascii_uppercase()) => "n",
        _ => "",
    }
}

/// The number that a text holds at its front.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: "20x" is 20, "x20" is
/// 0, " 30 " is 30, "1.9" is 1, "1e3" is 1000, "1,5" is 1 and "+7" is 7.
fn the_number_of_the_text(text: &str) -> f64 {
    let text = text.trim();
    let bytes = text.as_bytes();
    let mut end = 0;
    if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
        end += 1;
    }
    let mut digits = 0;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
        digits += 1;
    }
    if end < bytes.len() && bytes[end] == b'.' {
        end += 1;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0.0;
    }
    // The exponent: "1e3" is a thousand.
    if end < bytes.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
        let mut next = end + 1;
        if next < bytes.len() && (bytes[next] == b'+' || bytes[next] == b'-') {
            next += 1;
        }
        let mut exponent = 0;
        while next < bytes.len() && bytes[next].is_ascii_digit() {
            next += 1;
            exponent += 1;
        }
        if exponent > 0 {
            end = next;
        }
    }
    text[..end].parse::<f64>().unwrap_or(0.0)
}

/// The message v2 gives when a maths operation gets something that is
/// not a number.
///
/// The template is the one in the binary: "Expected a%s %s but got a%s
/// %s.". A number is not one of the kinds a value carries: it is what
/// the operation wants, and the value it got is one of them.
pub fn expected_number(value: &Value) -> String {
    format!("Expected a Number but got {}.", the_kind_of(value))
}

/// The message v2 gives when a built-in gets a value of the wrong type.
///
/// The template is the one in the binary: "Parameter #%i of %s requires
/// a%s %s, but received a%s %s.". The type that was wanted and the type
/// that arrived are both named.
pub fn requires(index: usize, name: &str, wants: TypeName, value: &Value) -> String {
    format!(
        "Parameter #{index} of {name} requires a{} {}, but received {}.",
        wants.article(),
        wants.name(),
        the_kind_of(value)
    )
}

/// The message v2 gives when the body of a function checks the kind of a
/// value.
///
/// The template is the one of the binary: "Expected a%s %s but got a%s
/// %s.". The kind that was wanted is named as it is, and the kind that
/// arrived is the name of the class of the value: "Expected an Object
/// but got an empty string.".
pub fn expects_a_kind(kind: TypeName, value: &Value) -> String {
    format!(
        "Expected a{} {} but got {}.",
        kind.article(),
        kind.name(),
        the_kind_of(value)
    )
}

/// Whether a value is the kind that a parameter declares.
///
/// The dialect converts the value to the kind that the parameter
/// declares: a parameter that wants a text takes a number, and one that
/// wants a number takes a text that holds one.
pub fn accepts(kind: TypeName, value: &Value) -> bool {
    match kind {
        TypeName::String => matches!(value, Value::Text(_) | Value::Int(_) | Value::Float(_)),
        TypeName::Number | TypeName::Integer => value.to_number_strict().is_ok(),
        TypeName::Object => matches!(value, Value::Object(_)),
        // The tree carries no value of COM, and nothing is one.
        TypeName::ComValue => false,
        _ => true,
    }
}

/// Whether a value is the kind that the body of a function checks.
///
/// The check is the kind of the value itself: a number is not a text.
pub fn is_of_the_kind(kind: TypeName, value: &Value) -> bool {
    match kind {
        TypeName::String => matches!(value, Value::Text(_)),
        TypeName::Object => matches!(value, Value::Object(_)),
        _ => true,
    }
}

/// The kind of a value, as the message of a built-in names it.
///
/// A text is a String, and an empty one is an empty string; a number is
/// an Integer or a Float; an object is an Object; a reference is a
/// VarRef.
pub fn the_kind_of(value: &Value) -> String {
    match value {
        Value::Text(text) if text.is_empty() => "an empty string".to_string(),
        other => {
            let kind = other.the_kind();
            format!("a{} {}", the_article_of(&kind), kind)
        }
    }
}

/// The message v2 gives for a parameter that has no value.
///
/// A parameter that is an alias of a variable of the caller says that
/// it is the parameter that has no value, and not the variable.
pub fn unassigned_parameter() -> String {
    "This parameter has not been assigned a value.".to_string()
}

/// The message v2 gives for a parameter that cannot be used.
pub fn invalid_parameter(index: usize, name: &str) -> String {
    format!("Parameter #{index} of {name} is invalid.")
}

/// The message v2 gives when a parameter wants a reference and gets
/// something else.
///
/// The wording is the one of the handler that raises it: the built-in
/// that writes the pieces of a path says "a variable reference" and the
/// one that writes the place of the pointer says "a VarRef".
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the message ends
/// with the kind of the value, and the value that was there is in the
/// `Extra` of the failure, and not in the message. The kind is the name
/// of the class of the value: "an Integer", "an Array", "an empty
/// string".
pub fn wants_a_reference(
    index: usize,
    name: &str,
    wants: &str,
    value: &Value,
) -> String {
    format!(
        "Parameter #{index} of {name} requires {wants}, but received {}.",
        the_kind_of(value)
    )
}

/// The name of a file: the part of the path after the last separator.
///
/// The separator is the slash of a Linux path and the backslash of a
/// Windows one: a script may carry either.
fn the_name_of(path: &str) -> &str {
    match path.rfind(['/', '\\']) {
        Some(at) => &path[at + 1..],
        None => path,
    }
}

/// The folder a file is in: the part of the path before the last
/// separator.
fn the_folder_of(path: &str) -> &str {
    match path.rfind(['/', '\\']) {
        Some(at) => &path[..at],
        None => "",
    }
}

/// The extension of a file: the part of the name after the last dot.
///
/// A dot that opens the name is not one: a file that begins with a dot
/// has no extension.
fn the_extension_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(at) if at > 0 => &name[at + 1..],
        _ => "",
    }
}

/// The path of a file without its extension.
fn without_the_extension(path: &str, extension: &str) -> String {
    if extension.is_empty() {
        return path.to_string();
    }
    path[..path.len() - extension.len() - 1].to_string()
}

/// The key the base of an object lives under.
///
/// The base is not a property of the object: v2 does not see it, and the
/// tree keeps it out of the way of the properties of the script. The key
/// has a space in it, so that a script cannot reach it by name.
pub(crate) const BASE: &str = "object base";

/// The class of the descriptor of a property.
///
/// The descriptor of v2 is an object with the members `Value`, `Get`,
/// `Set` and `Call`. The tree keeps the descriptor under a class of its
/// own, so that the read, the write and the call of the property know
/// that it is a descriptor and not a value.
pub(crate) const DESCRIPTOR: &str = "property descriptor";

/// The class that has the method a reference points at.
///
/// The key has a space in it, so that a script cannot reach it by name.
pub(crate) const METHOD_CLASS: &str = "method class";

/// The name of the method a reference points at.
pub(crate) const METHOD_NAME: &str = "method name";

/// The name under which the runtime keeps a closure.
///
/// The name of a closure of v2 is empty, and the name the runtime keeps it
/// under is not one a script can reach.
pub(crate) const CLOSURE: &str = "function closure";

/// The key the name of a class lives under.
///
/// v2 does not show the name of a class: a class value has no property
/// named `Name`, and a prototype has none either. The tree keeps the name
/// out of the way of the properties of the script. The key has a space in
/// it, so that a script cannot reach it by name.
pub(crate) const CLASS_NAME: &str = "class name";

/// Where a variable lives.
///
/// The scopes of the script are a stack: the global one is under the
/// ones of the functions. A reference says which one it points at.
/// A scope of the runtime: the names of a function.
///
/// The table is shared: a function that is written inside another one
/// closes over the scope of the one around it, and the write of the
/// closure is seen by the scope it closes over.
type ScopeTable = Rc<RefCell<BTreeMap<String, Value>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Scope {
    /// The global scope of the script.
    Global,
    /// A local scope, by its place in the stack.
    Local(usize),
}

/// A reference to a variable: `&x`.
///
/// The reference is a value of its own kind, and v2 names it a VarRef.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Reference {
    /// The name of the variable.
    pub name: String,
    /// Where the variable lives.
    pub scope: Scope,
    /// Whether the reference is an alias of the variable.
    ///
    /// A parameter written `&x` in the definition of a function is an
    /// alias: reading it reads the variable of the caller, and writing
    /// it writes there. A reference that is only a value is read as
    /// itself, and the operations that want a number or a text say that
    /// they got a VarRef.
    pub alias: bool,
}

/// A value a script can hold.
///
/// AutoHotkey v2 has no boolean: `true` is the integer 1 and `false` is
/// the integer 0. The tree keeps no boolean either, so that the tag on
/// the value is one of the kinds v2 has.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// A whole number.
    Int(i64),
    /// A number with a point.
    Float(f64),
    /// Text.
    Text(String),
    /// A list of values: what `MouseGetPos` and `PixelSearch` return.
    List(Vec<Value>),
    /// An object: an instance of a class.
    ///
    /// The object is held behind a reference, because an object in a
    /// script is a reference too: writing a property of it is seen by
    /// everyone who holds it.
    Object(Rc<RefCell<Object>>),
    /// A reference to a variable: `&x`.
    ///
    /// The operator is carried by the arguments of a call, and the
    /// value of a reference is the value of the variable it points at.
    Reference(Reference),
}

/// An object: an instance of a class.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Object {
    /// The name of the class the object is an instance of.
    pub class: String,
    /// The properties of the object.
    ///
    /// The key of the table is the name in lower case, and the name itself
    /// is kept beside the value: v2 gives the name as the script wrote it,
    /// and the first write is the one that fixes the case of it.
    pub properties: BTreeMap<String, Property>,
}

/// A property of an object: the name as the script wrote it, and the value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Property {
    /// The name of the property, as the script wrote it.
    pub name: String,
    /// The value of the property.
    pub value: Value,
}

/// Put a property in a table: the key is the name in lower case, and the name
/// is the one that was written.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the first write is the
/// one that fixes the case of the name -- `o.X := 1` and then `o.x := 2`
/// gives the name "X".
pub(crate) fn put(properties: &mut BTreeMap<String, Property>, name: &str, value: Value) {
    let key = name.to_lowercase();
    // The first write is the one that fixes the case of the name: a write
    // in the other case keeps the name that is there.
    let name = properties
        .get(&key)
        .map(|property| property.name.clone())
        .unwrap_or_else(|| name.to_string());
    properties.insert(key, Property { name, value });
}

impl Object {
    /// The value of a property, by the key of it.
    pub fn value_of(&self, key: &str) -> Option<Value> {
        self.properties.get(key).map(|property| property.value.clone())
    }

    /// Put a property in, and keep the name that was written.
    pub fn put(&mut self, name: &str, value: Value) {
        put(&mut self.properties, name, value);
    }
}

/// A class: the methods, the properties, and the class it extends.
#[derive(Clone)]
pub struct Class {
    /// The name of the class.
    pub name: String,
    /// The class this one extends.
    pub extends: Option<String>,
    /// The methods of the class, by their name in lower case.
    pub methods: BTreeMap<String, MethodDef>,
    /// The properties of the class.
    pub properties: Vec<PropertyDef>,
    /// The properties that have a getter or a setter, by their name in
    /// lower case.
    pub accessors: BTreeMap<String, AccessDef>,
    /// The classes that the class holds, by their name in lower case.
    pub nested: Vec<String>,
}

impl Value {
    /// Whether the value counts as true.
    pub fn truthy(&self) -> bool {
        match self {
            // Measured on the binary of v2.0.19 on 2026-10-31: a text that
            // is "0" is false, and a text that is not empty is true.
            Value::Text(text) => !text.is_empty() && text != "0",
            Value::Int(number) => *number != 0,
            Value::Float(number) => *number != 0.0,
            Value::List(items) => !items.is_empty(),
            // An object is always true.
            Value::Object(_) => true,
            // A reference points at something, and that is true.
            Value::Reference(_) => true,
        }
    }

    /// The type of the value, the way v2 names it.
    pub fn type_name(&self) -> TypeName {
        match self {
            Value::Int(_) => TypeName::Integer,
            Value::Float(_) => TypeName::Float,
            Value::Text(_) => TypeName::String,
            Value::List(_) | Value::Object(_) => TypeName::Object,
            Value::Reference(_) => TypeName::VarRef,
        }
    }

    /// The value as text.
    pub fn to_text(&self) -> String {
        match self {
            Value::Int(number) => number.to_string(),
            // A float keeps its point: `2.0` is not `2` in v2, and a
            // script that prints one wants to see it.
            Value::Float(number) => {
                if fract(*number) == 0.0 && number.abs() < 9.0e15 {
                    format!("{number:.1}")
                } else {
                    number.to_string()
                }
            }
            Value::Text(text) => text.clone(),
            Value::List(items) => items
                .iter()
                .map(|item| item.to_text())
                .collect::<Vec<String>>()
                .join(","),
            // An object has no text of its own: what it says is the
            // class it is an instance of.
            Value::Object(object) => object.borrow().class.clone(),
            // A reference has no text of its own either: what it says
            // is the name of the variable it points at.
            Value::Reference(reference) => reference.name.clone(),
        }
    }

    /// The value as text, when it is one.
    ///
    /// A reference is not a text: the operations that want one say that
    /// they got a VarRef.
    pub fn as_text(&self) -> Result<String, TypeName> {
        match self {
            Value::Reference(_) => Err(TypeName::VarRef),
            other => Ok(other.to_text()),
        }
    }

    /// The kind of the value, the way v2 names it.
    ///
    /// An object is named after its class: "Object", "Map", "Func", the
    /// name of the class of the script. An array is an "Array". A class is
    /// a class of its own kind, and a prototype is of its own too.
    pub fn the_kind(&self) -> String {
        match self {
            Value::Int(_) => "Integer".to_string(),
            Value::Float(_) => "Float".to_string(),
            Value::Text(_) => "String".to_string(),
            Value::List(_) => "Array".to_string(),
            Value::Reference(_) => "VarRef".to_string(),
            Value::Object(object) => {
                let borrowed = object.borrow();
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // prototype is of its own kind, and a class is a class of
                // its own kind.
                if borrowed.class == "prototype" {
                    return "Prototype".to_string();
                }
                // A class holds its prototype, and an instance does not.
                if borrowed.properties.contains_key("prototype") {
                    return "Class".to_string();
                }
                let mut name = borrowed.class.clone();
                if let Some(first) = name.get_mut(0..1) {
                    first.make_ascii_uppercase();
                }
                name
            }
        }
    }

    /// The value as a number, when it is one.
    ///
    /// A string that does not hold a number is not a number: the maths
    /// operations ask for the difference between the two.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Int(number) => Some(*number as f64),
            Value::Float(number) => Some(*number),
            Value::Text(text) => text.trim().parse::<f64>().ok(),
            Value::List(_) | Value::Object(_) | Value::Reference(_) => None,
        }
    }

    /// The value as a number, the way a maths operation wants it.
    ///
    /// A string that holds a number is a number. A string that holds
    /// anything else is a string, and the type it actually is comes back
    /// with the failure, so that the message can name it.
    pub fn to_number_strict(&self) -> Result<f64, TypeName> {
        match self {
            Value::Int(number) => Ok(*number as f64),
            Value::Float(number) => Ok(*number),
            Value::Text(text) => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    return Err(TypeName::String);
                }
                trimmed.parse::<f64>().map_err(|_| TypeName::String)
            }
            Value::List(_) | Value::Object(_) => Err(TypeName::Object),
            Value::Reference(_) => Err(TypeName::VarRef),
        }
    }

    /// The value as a number, the way a setting takes it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a text holds the
    /// number it carries at its front, and a text that holds no number is
    /// zero: "20x" is 20 and "x20" is 0.
    pub fn to_number_lenient(&self) -> f64 {
        match self {
            Value::Int(number) => *number as f64,
            Value::Float(number) => *number,
            Value::Text(text) => the_number_of_the_text(text),
            _ => 0.0,
        }
    }

    /// A number as a whole number when it is one.
    pub fn from_number(number: f64) -> Value {
        if fract(number) == 0.0 && number.abs() < 9.0e15 {
            Value::Int(number as i64)
        } else {
            Value::Float(number)
        }
    }
}

/// A statement failed.
#[derive(Debug, thiserror::Error)]
#[error("{message} (line {line})")]
pub struct RuntimeError {
    pub message: String,
    pub line: u32,
}

/// The script ran for longer than the caller allowed.
#[derive(Debug, thiserror::Error)]
#[error("the script ran for more than {seconds}s")]
pub struct Timeout {
    pub seconds: f64,
}

/// The clock the runtime measures itself with, and waits on.
///
/// The crate does no I/O of its own, and a clock is I/O in the same way a
/// desktop is: it belongs to the machine the script runs on, not to the
/// language. A hosted platform uses [`SystemClock`]; a target with no
/// `std` at all supplies its own.
pub trait Clock {
    /// The milliseconds since an arbitrary origin. It never goes backwards.
    fn now_ms(&self) -> u64;

    /// Wait for the given number of milliseconds.
    fn sleep_ms(&mut self, millis: u64);

    /// The date and the time of the machine, when it knows them.
    ///
    /// The clock of a run counts from an arbitrary origin, and it does not
    /// know the date. A host that knows the date says so here; a host that
    /// does not leaves it at nothing, and the names of the date answer with
    /// the reason.
    fn wall_clock(&self) -> Option<crate::date::WallClock> {
        None
    }
}

/// The clock of a hosted platform: the monotonic clock, and the system sleep.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy)]
pub struct SystemClock {
    started: std::time::Instant,
}

#[cfg(feature = "std")]
impl SystemClock {
    /// A clock that starts counting now.
    pub fn new() -> Self {
        Self { started: std::time::Instant::now() }
    }
}

#[cfg(feature = "std")]
impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        self.started.elapsed().as_millis() as u64
    }

    fn sleep_ms(&mut self, millis: u64) {
        std::thread::sleep(Duration::from_millis(millis));
    }

    fn wall_clock(&self) -> Option<crate::date::WallClock> {
        crate::date::system_wall_clock()
    }
}

/// Where the runtime gets the time from.
enum ClockSource<'a> {
    /// A clock the caller owns.
    Borrowed(&'a mut dyn Clock),
    /// The clock of the host.
    #[cfg(feature = "std")]
    System(SystemClock),
}

impl ClockSource<'_> {
    fn now_ms(&self) -> u64 {
        match self {
            ClockSource::Borrowed(clock) => clock.now_ms(),
            #[cfg(feature = "std")]
            ClockSource::System(clock) => clock.now_ms(),
        }
    }

    fn sleep_ms(&mut self, millis: u64) {
        match self {
            ClockSource::Borrowed(clock) => clock.sleep_ms(millis),
            #[cfg(feature = "std")]
            ClockSource::System(clock) => clock.sleep_ms(millis),
        }
    }

    fn wall_clock(&self) -> Option<crate::date::WallClock> {
        match self {
            ClockSource::Borrowed(clock) => clock.wall_clock(),
            #[cfg(feature = "std")]
            ClockSource::System(clock) => clock.wall_clock(),
        }
    }
}

/// What one statement did.
#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub line: u32,
    pub op: String,
    pub detail: String,
    pub ok: bool,
    pub elapsed_ms: u64,
}

/// The whole run: the steps, the output, and whether it finished.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Report {
    pub ok: bool,
    pub steps: Vec<Step>,
    pub output: Vec<String>,
    pub notes: Vec<String>,
    pub error: Option<String>,
    pub error_line: Option<u32>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: u64,
    pub variables: BTreeMap<String, Value>,
}

/// The way a script left a block.
pub(crate) enum Flow {
    Break,
    Continue,
    Return(Option<Value>),
    Exit(i32),
    /// The jump of a `Goto`: the name of the label, as the script wrote
    /// it, and the line of the jump.
    ///
    /// The name is the text of the parameter, and not its value: v2 reads
    /// the name of a label as it stands, and the quotes of a text are part
    /// of it.
    Goto { name: String, line: u32 },
}

/// What the functions of the error said about a failure.
///
/// The answers are the ones of v2: 0 lets the failure go on, 1 hides it,
/// and -1 hides it and lets the thread continue.
enum TheError {
    /// The failure goes on: it is the error of the run.
    Goes,
    /// The failure is hidden, and the thread ends.
    Hidden,
    /// The failure is hidden, and the thread continues.
    Continues,
}

/// A function the script defined.
#[derive(Clone)]
struct Function {
    /// The name of the function, as the script wrote it.
    name: String,
    /// The parameters it takes.
    parameters: Vec<Parameter>,
    body: Vec<Node>,
    /// The declarations of the function.
    declarations: Declarations,
    /// The scopes the function closes over.
    ///
    /// A function that is written inside another one sees the variables of
    /// the one around it, and it writes them there too.
    closure: Vec<ScopeTable>,
}

/// A timer the script set.
///
/// The timer names a function of the script, and it runs between the
/// statements, not in a thread of its own.
struct Timer {
    /// The name of the function the timer runs.
    name: String,
    /// The milliseconds between two runs.
    period: u64,
    /// When it ran the last time.
    last: u64,
}

/// A function of the script that the runtime calls.
///
/// It is the callback of the exit or of the error. The name is the one the
/// script wrote, and the line is the line of the statement that registered
/// it: a function that fails says where it was asked for.
#[derive(Clone)]
pub(crate) struct Callback {
    /// The name of the function the script registered.
    pub name: String,
    /// The line of the statement that registered it.
    pub line: u32,
}

/// A frame of a call that runs.
///
/// The frame is the name of the function or of the method, and the line of
/// the statement that runs in it. The stack of an error of v2 names both.
#[derive(Debug, Clone)]
struct Frame {
    name: String,
    line: u32,
}

/// Executes a script.
///
/// The limits are the guard rails: a script that loops forever must not
/// take the box with it, and one that types a million characters must not
/// either. Both limits are reported when they are hit.
pub struct Runtime<'a> {
    desktop: &'a mut dyn Desktop,
    clock: ClockSource<'a>,
    timeout: Duration,
    variables: BTreeMap<String, Value>,
    /// The directory where the run started.
    ///
    /// The runtime asks the desktop for it when it is made, and it keeps
    /// it: a script that changes the working directory does not change the
    /// one where the run began.
    initial_working_dir: Option<String>,
    scopes: Vec<ScopeTable>,
    /// The labels of the scope that runs.
    ///
    /// A label belongs to the scope it is written in: the scope of the
    /// script, or the scope of a function or of a method. `IsLabel` asks
    /// the scope that runs whether it holds the label, and the name of a
    /// label does not know its case.
    label_scopes: Vec<Vec<String>>,
    /// The declarations of the functions that run.
    ///
    /// The frame of a function says where its names live: the ones it
    /// declares global live in the scope of the script, and the ones it
    /// declares local live in the scope of the function.
    declarations: Vec<Declarations>,
    /// The static variables of the functions that ran.
    ///
    /// A static variable belongs to the function that declares it, and it
    /// keeps its value between the calls: the table of the function is
    /// here, and not in the scope of the call.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the static of a
    /// function is the same one in every call of it, the static of a
    /// recursion is the same one in every turn of it, and two functions
    /// have two of them.
    statics: BTreeMap<String, BTreeMap<String, Value>>,
    /// The static declarations that already ran their value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the value of a
    /// static declaration runs the first time that declaration is reached,
    /// and a declaration that the function does not reach on the first call
    /// runs its value on the call that reaches it.
    statics_run: BTreeMap<String, BTreeSet<String>>,
    /// The classes of the methods that run.
    ///
    /// `super` asks the base of the class where the method was defined,
    /// and that class is the one the frame carries.
    current_classes: Vec<String>,
    /// The frames of the calls that run.
    ///
    /// The frame is the name of the function or of the method that
    /// runs: v2 puts it in the `What` of an error that does not carry
    /// one. The frame carries the line of the statement that runs in it,
    /// and the stack of an error names it.
    frames: Vec<Frame>,
    /// The lines of the script, when the host gave them.
    ///
    /// The stack of an error names the source of the line that failed,
    /// and the script of the runner has no file of its own.
    source: Vec<String>,
    /// The line of the statement that runs in the section that starts the
    /// script.
    ///
    /// The section that starts the script is the last frame of the stack
    /// of v2, and it has no name.
    auto_execute_line: u32,
    functions: BTreeMap<String, Function>,
    /// How many closures the runtime made.
    ///
    /// The name of a closure is empty, and the runtime keeps every one of
    /// them under a name of its own.
    closures: usize,
    classes: BTreeMap<String, Class>,
    /// The class values the runtime made.
    ///
    /// A class is one object for the whole run: the name of it, the class
    /// it extends and the prototype of it. Two names of the same class
    /// are the same object, and that is what tells a class from an
    /// instance of one.
    class_values: RefCell<BTreeMap<String, Value>>,
    /// The classes that are already initialised.
    ///
    /// A class of v2 is initialised when it is first used: its variables
    /// that belong to it are assigned then, and the `__New` that is static
    /// is the one that runs.
    initialised: RefCell<BTreeSet<String>>,
    steps: Vec<Step>,
    output: Vec<String>,
    notes: Vec<String>,
    started_ms: u64,
    steps_taken: usize,
    /// The calls that are running right now.
    depth: usize,
    /// Where the stack was when the runtime was made.
    stack_base: usize,
    /// How much stack the run may use.
    stack_budget: usize,
    /// The timers the script set.
    timers: Vec<Timer>,
    /// The hotkeys the script set.
    ///
    /// The runtime owns the function that a hotkey runs; the desktop owns
    /// the watching of the keyboard, and says which hotkey was pressed.
    hotkeys: Vec<crate::hotkeys::Hotkey>,
    /// The code the script asked to stop with.
    exit_requested: Option<i32>,
    /// The functions to call when the script ends.
    ///
    /// The script asks for them with `OnExit`, and the runtime calls them
    /// when the script ends, whatever the reason the script ended.
    exit_callbacks: Vec<Callback>,
    /// The functions to call when a failure is not caught.
    ///
    /// The script asks for them with `OnError`.
    error_callbacks: Vec<Callback>,
    /// Whether the body of a `try` is running.
    ///
    /// While it runs, the functions of the error are not called: the catch
    /// of the script takes the failure first, and the functions of the
    /// error are called only when no catch took it.
    try_active: usize,
    /// Whether the functions of the error hid a failure.
    ///
    /// A failure that was hidden is not the error of the run: the script
    /// asked for it to be hidden, and it is.
    failure_hidden: bool,
    /// The value that the catch of the running `try` took.
    ///
    /// `throw` without a value throws it again, the way v2 does.
    caught_value: Option<Value>,
}

impl<'a> Runtime<'a> {
    /// The most statements one script may execute.
    ///
    /// The guard rail is for a script that runs away, not for a script
    /// that types: a thousand characters are a thousand statements, and a
    /// script that clears a field of a thousand characters is doing its
    /// job. The timeout is the other bound.
    pub const MAX_STEPS: usize = 20_000;
    /// The most times a loop body may run.
    pub const MAX_ITERATIONS: usize = 1000;
    /// The most nested calls a script may make.
    ///
    /// v2.0.19 stops at about 1105 nested calls, with "Function recursion
    /// limit exceeded." The tree stops at the same place.
    pub const MAX_DEPTH: usize = 1000;
    /// How much stack a run may use, when the host does not say.
    ///
    /// The frames of the tree are heavier than the ones of v2: a debug
    /// build uses about 50 KB of stack per call and a release build about
    /// 5 KB. A host with a small stack would run out of it before the
    /// depth says so, and the run stops when the stack is gone.
    pub const MAX_STACK: usize = 512 * 1024;

    /// A runtime that drives `desktop` for at most `timeout`, measured by
    /// the system clock.
    #[cfg(feature = "std")]
    pub fn new(desktop: &'a mut dyn Desktop, timeout: Duration) -> Self {
        let initial_working_dir = desktop.initial_working_directory().ok();
        Self {
            desktop,
            clock: ClockSource::System(SystemClock::new()),
            timeout,
            variables: BTreeMap::new(),
            statics: BTreeMap::new(),
            statics_run: BTreeMap::new(),
            initial_working_dir,
            scopes: Vec::new(),
            label_scopes: Vec::new(),
            declarations: Vec::new(),
            current_classes: Vec::new(),
            frames: Vec::new(),
            source: Vec::new(),
            auto_execute_line: 0,
            functions: BTreeMap::new(),
            closures: 0,
            classes: BTreeMap::new(),
            class_values: RefCell::new(BTreeMap::new()),
            initialised: RefCell::new(BTreeSet::new()),
            steps: Vec::new(),
            output: Vec::new(),
            notes: Vec::new(),
            started_ms: 0,
            steps_taken: 0,
            depth: 0,
            stack_base: Self::stack_here(),
            stack_budget: Self::MAX_STACK,
            timers: Vec::new(),
            hotkeys: Vec::new(),
            exit_requested: None,
            exit_callbacks: Vec::new(),
            error_callbacks: Vec::new(),
            try_active: 0,
            failure_hidden: false,
            caught_value: None,
        }
    }

    /// A runtime that drives `desktop` for at most `timeout`, measured by
    /// `clock`.
    ///
    /// This is the constructor for a target with no `std`: the clock is
    /// the platform's, and so is the pause the `Sleep` statement asks
    /// for.
    pub fn with_clock(
        desktop: &'a mut dyn Desktop,
        clock: &'a mut dyn Clock,
        timeout: Duration,
    ) -> Self {
        let started_ms = clock.now_ms();
        let initial_working_dir = desktop.initial_working_directory().ok();
        Self {
            desktop,
            clock: ClockSource::Borrowed(clock),
            timeout,
            variables: BTreeMap::new(),
            statics: BTreeMap::new(),
            statics_run: BTreeMap::new(),
            initial_working_dir,
            scopes: Vec::new(),
            label_scopes: Vec::new(),
            declarations: Vec::new(),
            current_classes: Vec::new(),
            frames: Vec::new(),
            source: Vec::new(),
            auto_execute_line: 0,
            functions: BTreeMap::new(),
            closures: 0,
            classes: BTreeMap::new(),
            class_values: RefCell::new(BTreeMap::new()),
            initialised: RefCell::new(BTreeSet::new()),
            steps: Vec::new(),
            output: Vec::new(),
            notes: Vec::new(),
            started_ms,
            steps_taken: 0,
            depth: 0,
            stack_base: Self::stack_here(),
            stack_budget: Self::MAX_STACK,
            timers: Vec::new(),
            hotkeys: Vec::new(),
            exit_requested: None,
            exit_callbacks: Vec::new(),
            error_callbacks: Vec::new(),
            try_active: 0,
            failure_hidden: false,
            caught_value: None,
        }
    }

    /// A runtime whose run may use `bytes` of stack.
    ///
    /// The host that gives the run a deep stack says so here, and the run
    /// stops before the stack is gone.
    pub fn with_stack_budget(mut self, bytes: usize) -> Self {
        self.stack_budget = bytes;
        self
    }

    /// A runtime that knows the source of the script.
    ///
    /// The stack of an error of v2 names the source of the line that
    /// failed, and the host that reads the script gives it here.
    pub fn with_source(mut self, source: &str) -> Self {
        self.source = source.lines().map(|line| line.to_string()).collect();
        self
    }

    /// The milliseconds since the run started.
    fn elapsed_ms(&self) -> u64 {
        self.clock.now_ms().saturating_sub(self.started_ms)
    }

    /// The milliseconds the run has left before the timeout of the script.
    ///
    /// The timeout is checked between the statements, and a built-in that
    /// waits inside one does not pass through that check.
    pub(crate) fn remaining_ms(&self) -> u64 {
        let limit = self.timeout.as_millis() as u64;
        limit.saturating_sub(self.elapsed_ms())
    }

    /// Run the script and report what happened.
    pub fn run(mut self, statements: &[Node]) -> Report {
        let mut error = None;
        let mut error_line = None;
        let mut exit_code = None;
        // The functions are collected before the first statement runs, so a
        // script may call a function it defines further down.
        self.collect_functions(statements);
        // The labels of the script are the scope of the script.
        self.label_scopes.push(the_labels_of(statements));
        match self.run_block(statements) {
            Ok(()) => {}
            Err(RuntimeFailure::Flow(Flow::Break)) | Err(RuntimeFailure::Flow(Flow::Continue)) => {}
            Err(RuntimeFailure::Flow(Flow::Return(_))) => {}
            Err(RuntimeFailure::Flow(Flow::Exit(code))) => exit_code = Some(code),
            // The jump of a `Goto` that leaves the script: the label is not
            // in the scope of the script.
            Err(RuntimeFailure::Flow(Flow::Goto { name, line })) => {
                let failure = self.the_label_is_not_there(&name, statements, line);
                error = Some(failure.message);
                error_line = Some(failure.line);
            }
            Err(RuntimeFailure::Fatal(failure)) => {
                error = Some(failure.message);
                error_line = Some(failure.line);
            }
            Err(RuntimeFailure::Error(failure)) => {
                error = Some(failure.message);
                error_line = Some(failure.line);
            }
            Err(RuntimeFailure::Detailed(failure, _)) => {
                error = Some(failure.message);
                error_line = Some(failure.line);
            }
            Err(RuntimeFailure::Timeout(timeout)) => {
                error = Some(timeout.to_string());
            }
            // A value that was thrown and that no catch took.
            Err(RuntimeFailure::Thrown(value)) => {
                error = Some(thrown_message(&value));
            }
        }
        // A failure that the functions of the error hid is not the error of
        // the run: the script asked for it to be hidden, and it is.
        if self.failure_hidden {
            error = None;
            error_line = None;
        }
        // The script has ended. The functions it asked to run at its exit
        // run now: the end is the moment the runtime knows, and it knows
        // it whatever the reason the script ended.
        let reason = if error.is_some() { "Error" } else { "Exit" };
        let code = exit_code.unwrap_or(0);
        // The script has already stopped: the flag that stops it does not
        // stop the functions of the exit, and the code of the run is the
        // code the script asked with.
        self.exit_requested = None;
        let mut prevented = false;
        let callbacks: Vec<Callback> = core::mem::take(&mut self.exit_callbacks);
        for callback in callbacks {
            let values = [Value::Text(reason.to_string()), Value::Int(code as i64)];
            match self.call_named(&callback.name, &values, callback.line, Route::Written) {
                Ok(answer) => {
                    // A function that answers a number that is not zero
                    // keeps the script alive, and the functions that come
                    // after it are not called.
                    if answer.to_number().unwrap_or(0.0) != 0.0 {
                        prevented = true;
                        break;
                    }
                }
                Err(failure) => {
                    let (message, line) = match failure {
                        RuntimeFailure::Error(error) => (error.message, error.line),
                        RuntimeFailure::Detailed(error, _) => (error.message, error.line),
                        RuntimeFailure::Fatal(error) => (error.message, error.line),
                        RuntimeFailure::Thrown(value) => (thrown_message(&value), 0),
                        RuntimeFailure::Timeout(timeout) => (timeout.to_string(), 0),
                        RuntimeFailure::Flow(_) => (
                            "the function of the exit left the script in the middle of a block.".to_string(),
                            0,
                        ),
                    };
                    if error.is_none() {
                        error = Some(message);
                        error_line = Some(if line == 0 { callback.line } else { line });
                    } else {
                        // The script had already failed: its failure is the
                        // error of the run, and the failure of the function of
                        // the exit is a note, so that it is not lost.
                        self.note(format!("the function of the exit failed: {message}"));
                    }
                }
            }
        }
        if prevented {
            // The exit was prevented, and the code the script asked for is
            // not the code of the run. This runtime has no way to keep a
            // script alive, so the run ends anyway.
            exit_code = None;
            self.note(
                "a function of the exit asked to keep the script alive, and this runtime ends the script when its statements end.",
            );
        }
        // A function of the exit may ask to stop with a code of its own,
        // and that code is the code of the run.
        if let Some(code) = self.exit_requested {
            exit_code = Some(code);
        }
        let elapsed_ms = self.elapsed_ms();
        Report {
            ok: error.is_none(),
            steps: self.steps,
            output: self.output,
            notes: self.notes,
            error,
            error_line,
            exit_code,
            elapsed_ms,
            variables: self.variables.clone(),
        }
    }

    /// A note for the report: a thing the script should know, and that is
    /// not the error of the run.
    pub fn note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }

    /// Find the functions and the classes the script defines, wherever
    /// they are.
    fn collect_functions(&mut self, statements: &[Node]) {
        for statement in statements {
            match statement {
                Node::ClassDef { name, extends, methods, properties, accessors, nested, .. } => {
                    let mut table: BTreeMap<String, MethodDef> = BTreeMap::new();
                    for method in methods {
                        table.insert(method.name.to_lowercase(), method.clone());
                    }
                    let mut properties_with_a_getter: BTreeMap<String, AccessDef> = BTreeMap::new();
                    for access in accessors {
                        properties_with_a_getter.insert(access.name.to_lowercase(), access.clone());
                    }
                    // The classes that a class holds are classes of their own.
                    let mut held = Vec::new();
                    for node in nested {
                        if let Node::ClassDef { name, .. } = node {
                            held.push(name.to_lowercase());
                        }
                    }
                    self.classes.insert(
                        name.to_lowercase(),
                        Class {
                            name: name.clone(),
                            extends: extends.clone(),
                            methods: table,
                            properties: properties.clone(),
                            accessors: properties_with_a_getter,
                            nested: held,
                        },
                    );
                    self.collect_functions(nested);
                }
                Node::FunctionDef { name, parameters, body, declarations, .. } => {
                    self.functions.insert(
                        name.to_lowercase(),
                        Function {
                            name: name.clone(),
                            parameters: parameters.clone(),
                            body: body.clone(),
                            declarations: declarations.clone(),
                            closure: Vec::new(),
                        },
                    );
                }
                Node::If { branches, otherwise, .. } => {
                    for branch in branches {
                        self.collect_functions(&branch.body);
                    }
                    self.collect_functions(otherwise);
                }
                Node::Switch { cases, .. } => {
                    for case in cases {
                        self.collect_functions(&case.body);
                    }
                }
                Node::While { body, .. } | Node::Loop { body, .. } => self.collect_functions(body),
                Node::Try { body, catches, otherwise, finally, .. } => {
                    self.collect_functions(body);
                    for catch in catches {
                        self.collect_functions(&catch.body);
                    }
                    self.collect_functions(otherwise);
                    self.collect_functions(finally);
                }
                _ => {}
            }
        }
    }

    /// The functions that are written inside a function.
    ///
    /// v2 knows them when the function is read: a function that is written
    /// inside another one is there before the statement that defines it
    /// runs, and it closes over the scope of the function that has it.
    fn collect_the_nested_functions(&mut self, statements: &[Node]) {
        for statement in statements {
            match statement {
                Node::FunctionDef { name, parameters, body, declarations, .. } => {
                    let function = Function {
                        name: name.clone(),
                        parameters: parameters.clone(),
                        body: body.clone(),
                        declarations: declarations.clone(),
                        closure: self.scopes.clone(),
                    };
                    let closure = self.make_a_closure(function.clone());
                    let value = closure_value(&closure, &function);
                    self.assign_in_the_scope(name, value);
                }
                Node::If { branches, otherwise, .. } => {
                    for branch in branches {
                        self.collect_the_nested_functions(&branch.body);
                    }
                    self.collect_the_nested_functions(otherwise);
                }
                Node::Switch { cases, .. } => {
                    for case in cases {
                        self.collect_the_nested_functions(&case.body);
                    }
                }
                Node::While { body, .. } | Node::Loop { body, .. } => self.collect_the_nested_functions(body),
                Node::Try { body, catches, otherwise, finally, .. } => {
                    self.collect_the_nested_functions(body);
                    for catch in catches {
                        self.collect_the_nested_functions(&catch.body);
                    }
                    self.collect_the_nested_functions(otherwise);
                    self.collect_the_nested_functions(finally);
                }
                _ => {}
            }
        }
    }

    /// The failure of a `Goto` that does not find its label.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a label of another
    /// scope is not found in the current one, and a label of a block that
    /// does not enclose the jump is refused with a failure of its own.
    fn the_label_is_not_there(&self, name: &str, scope: &[Node], line: u32) -> RuntimeError {
        if the_labels_of(scope).iter().any(|label| label.eq_ignore_ascii_case(name)) {
            return RuntimeError {
                message: "A Goto must not jump into a block that doesn't enclose it.".into(),
                line,
            };
        }
        RuntimeError {
            message: format!("Label not found in current scope.\n     Specifically: {name}"),
            line,
        }
    }

    /// Whether the scope that runs holds the label.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the label is asked
    /// in the scope that runs, and the name of a label does not know its
    /// case.
    pub fn has_label(&self, name: &str) -> bool {
        let Some(scope) = self.label_scopes.last() else { return false };
        scope.iter().any(|label| label.eq_ignore_ascii_case(name))
    }

    fn run_block(&mut self, statements: &[Node]) -> Result<(), RuntimeFailure> {
        let mut index = 0;
        while index < statements.len() {
            let statement = &statements[index];
            // A failure of the statement is given to the functions that
            // asked to see it, before the stack unwinds: one of them may
            // say that the thread continues.
            match self.execute(statement) {
                Ok(()) => {}
                // The jump of a `Goto` is not a failure: the run goes on at
                // the label when this block holds it, and it goes out of
                // the block when it does not.
                Err(RuntimeFailure::Flow(Flow::Goto { name, line })) => {
                    match the_place_of_the_label(statements, &name) {
                        Some(place) => index = place,
                        None => return Err(RuntimeFailure::Flow(Flow::Goto { name, line })),
                    }
                }
                // A failure that no catch takes goes out at once.
                Err(failure @ RuntimeFailure::Fatal(_)) => return Err(failure),
                Err(failure) => match self.ask_the_error_callbacks(&failure) {
                    // The thread continues: the statement that failed is
                    // over, and the failure is not the error of the run.
                    TheError::Continues => {}
                    // The failure is hidden, and the thread ends.
                    TheError::Hidden => {
                        self.failure_hidden = true;
                        self.note(format!(
                            "the functions of the error hid the failure: {}",
                            the_message_of(&failure)
                        ));
                        return Err(failure);
                    }
                    TheError::Goes => return Err(failure),
                },
            }
            // The timers of the script run between its statements, and
            // only there: a function of the script is not the script.
            if self.scopes.is_empty() {
                self.run_timers()?;
                // The hotkeys of the script run between its statements,
                // and only there: a function of the script is not the
                // script, and a hotkey runs with the scope of the script
                // and not with the scope of the function.
                crate::hotkeys::run_fired(self)?;
            }
            // A script that asked to stop, stops at the end of the
            // statement it asked in.
            if let Some(code) = self.exit_requested {
                return Err(RuntimeFailure::Flow(Flow::Exit(code)));
            }
            index += 1;
        }
        Ok(())
    }

    /// The `try` of the dialect: the block is guarded, the catches take
    /// the values that were thrown, the `else` runs when nothing was
    /// thrown, and the `finally` always runs.
    fn run_try(
        &mut self,
        body: &[Node],
        catches: &[crate::ast::Catch],
        otherwise: &[Node],
        finally: &[Node],
    ) -> Result<(), RuntimeFailure> {
        self.try_active += 1;
        let mut failure = self.run_block(body).err();
        self.try_active -= 1;
        let mut taken = false;
        // The value that the catch sees: the error of the runtime is an
        // object of the class `error`, and a thrown value is the value
        // itself.
        let thrown = match failure.as_ref() {
            Some(RuntimeFailure::Error(error)) => Some(self.error_value(error)),
            Some(RuntimeFailure::Detailed(error, detail)) => {
                Some(self.error_value_with(error, detail))
            }
            Some(RuntimeFailure::Thrown(value)) => Some(value.clone()),
            _ => None,
        };
        if let Some(value) = thrown {
            for catch in catches {
                if !self.catch_takes(catch, &value)? {
                    continue;
                }
                taken = true;
                if let Some(variable) = &catch.variable {
                    self.assign(variable, value.clone());
                }
                // The `throw` without a value throws again what this catch
                // took.
                let before = self.caught_value.replace(value);
                let result = self.run_block(&catch.body);
                self.caught_value = before;
                failure = result.err();
                break;
            }
        }
        if !taken && failure.is_none() {
            // Nothing was thrown: the `else` runs.
            failure = self.run_block(otherwise).err();
        }
        // The `finally` always runs, and the failure it leaves is the one
        // that is left.
        if let Err(other) = self.run_block(finally) {
            failure = Some(other);
        }
        match failure {
            Some(failure) => Err(failure),
            None => Ok(()),
        }
    }

    /// Whether the catch takes the value that was thrown.
    ///
    /// The class of the catch is a name: `any` takes everything, `error`
    /// takes the errors of the runtime, and a class of the script takes the
    /// objects of that class and of the classes that extend it.
    fn catch_takes(&self, catch: &crate::ast::Catch, value: &Value) -> Result<bool, RuntimeFailure> {
        let class = catch.class.to_lowercase();
        if class == "any" {
            return Ok(true);
        }
        if class == "error" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a catch that
            // names no class takes every error of the runtime. The errors
            // are a tree of classes, and every one of them is an `Error`.
            let Value::Object(object) = value else { return Ok(false) };
            return Ok(crate::classes::error::is_a_class(&object.borrow().class));
        }
        // A class of the script. A class that the tree does not know is a
        // question it cannot answer, and it says so.
        let known = self.classes.contains_key(&class)
            || crate::classes::is_type(&class)
            || class == "object";
        if !known {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!(
                    "the class of a catch is not a class of this runtime: {}",
                    catch.class
                ),
                line: 0,
            }));
        }
        Ok(self.value_has_class(value, &class))
    }

    /// Whether the value is an object of the class, or of one that extends
    /// it.
    fn value_has_class(&self, value: &Value, class: &str) -> bool {
        let Value::Object(object) = value else { return false };
        let mut name = object.borrow().class.clone();
        loop {
            if name.eq_ignore_ascii_case(class) {
                return true;
            }
            match self.classes.get(&name.to_lowercase()) {
                Some(definition) => match &definition.extends {
                    Some(base) => name = base.clone(),
                    None => return false,
                },
                None => return false,
            }
        }
    }

    /// The error of the runtime as a value: an object of the class `error`,
    /// the way v2 hands it to a catch.
    fn error_value(&self, error: &RuntimeError) -> Value {
        crate::classes::error::error_value_of(
            crate::classes::error::the_class_of(&error.message),
            &error.message,
            "",
            "",
            error.line,
            &self.the_stack(),
        )
    }

    /// The error of the runtime as a value, with the detail of it.
    ///
    /// The detail is the name of the thing that raised the failure, and
    /// the detail that goes with it.
    fn error_value_with(&self, error: &RuntimeError, detail: &Detail) -> Value {
        crate::classes::error::error_value_of(
            crate::classes::error::the_class_of(&error.message),
            &error.message,
            &detail.what,
            &detail.extra,
            error.line,
            &self.the_stack(),
        )
    }

    /// The stack of an error, the way v2 writes it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: every frame is a
    /// line of its own -- the file, the line, the name of the frame and
    /// the source of the line -- and the last line names the section
    /// that starts the script.
    ///
    /// The script of the runner has no file of its own, and the file of
    /// every frame is empty.
    pub(crate) fn the_stack(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        for frame in self.frames.iter().rev() {
            lines.push(self.the_line_of_a_frame(&frame.name, frame.line));
        }
        lines.push(self.the_line_of_a_frame("", self.auto_execute_line));
        lines.push("> Auto-execute".to_string());
        lines.join("\n")
    }

    /// The line of a frame of the stack.
    fn the_line_of_a_frame(&self, name: &str, line: u32) -> String {
        format!(" ({line}) : [{name}] {}", self.the_source_of_the_line(line))
    }

    /// The source of a line of the script, when the host gave it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the source that
    /// the stack names is the text of the line without the space that
    /// indents it.
    fn the_source_of_the_line(&self, line: u32) -> String {
        self.source
            .get(line.saturating_sub(1) as usize)
            .map(|text| text.trim().to_string())
            .unwrap_or_default()
    }

    /// The name of the thing that threw a value.
    ///
    /// v2 puts the name of the function or of the method in the `What`
    /// of an error that does not carry one.
    fn name_the_thrower(&self, value: &Value) {
        let Value::Object(object) = value else { return };
        if object.borrow().class != "error" {
            return;
        }
        let carries = object
            .borrow()
            .value_of("what")
            .map(|what| !what.to_text().is_empty())
            .unwrap_or(false);
        if carries {
            return;
        }
        let Some(frame) = self.frames.last() else { return };
        put(&mut object.borrow_mut().properties, "what", Value::Text(frame.name.clone()));
    }

    /// The timers that are due.
    ///
    /// The steps of a timer are steps of the script, so the budget of a
    /// timer is the budget of the script.
    fn run_timers(&mut self) -> Result<(), RuntimeFailure> {
        let now = self.now_ms();
        let due: Vec<String> = self
            .timers
            .iter_mut()
            .filter(|timer| now.saturating_sub(timer.last) >= timer.period)
            .map(|timer| {
                timer.last = now;
                timer.name.clone()
            })
            .collect();
        for name in due {
            let Some(function) = self.functions.get(&name.to_lowercase()).cloned() else {
                continue;
            };
            self.call_function(&function, &[], 0, Route::Written)?;
        }
        Ok(())
    }

    /// Set a timer of the script.
    pub(crate) fn add_timer(&mut self, name: &str, period: u64) {
        let last = self.now_ms();
        self.timers.push(Timer { name: name.to_string(), period, last });
    }

    /// Stop the script with this code.
    pub(crate) fn exit_with(&mut self, code: i32) {
        self.exit_requested = Some(code);
    }

    /// A method of the base object, for the names that ask about an
    /// object.
    pub(crate) fn base_object(
        &mut self,
        target: &Value,
        name: &str,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeError> {
        match self.base_method(target, name, arguments, line) {
            Some(Ok(value)) => Ok(value),
            Some(Err(RuntimeFailure::Error(error))) => Err(error),
            Some(Err(RuntimeFailure::Detailed(error, _))) => Err(error),
            Some(Err(_)) => Err(RuntimeError {
                message: "the runtime cannot answer that.".into(),
                line,
            }),
            None => Err(RuntimeError {
                message: format!(
                    "This value of type \"{}\" has no method named {name:?}.",
                    self.the_type_of(target)
                ),
                line,
            }),
        }
    }

    fn execute(&mut self, statement: &Node) -> Result<(), RuntimeFailure> {
        self.steps_taken += 1;
        // The stack of an error names the line of the statement that runs
        // in every frame.
        match self.frames.last_mut() {
            Some(frame) => frame.line = statement.line(),
            None => self.auto_execute_line = statement.line(),
        }
        if self.steps_taken > Self::MAX_STEPS {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!("the script ran more than {} statements", Self::MAX_STEPS),
                line: statement.line(),
            }));
        }
        if self.elapsed_ms() > self.timeout.as_millis() as u64 {
            return Err(RuntimeFailure::Timeout(Timeout { seconds: self.timeout.as_secs_f64() }));
        }

        let started = self.clock.now_ms();
        let mut detail = String::new();

        match statement {
            Node::Assign { name, op, value, line } => {
                let value = self.evaluate(value)?;
                let stored = self.write_a_variable(name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::ClassDef { name, line, .. } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the class
                // is initialised when it is first used, and the statement
                // that defines it is a use.
                self.initialise_the_class(name, *line)?;
                detail = format!("the class {name} is defined");
            }
            Node::ArrayLiteral { .. } => {
                // An array literal is an expression: the parser never
                // gives one as a statement of its own.
            }
            Node::ObjectLiteral { .. } => {
                // The literal of an object is an expression too.
            }
            Node::Lambda { .. } => {
                // The literal of a function is an expression too.
            }
            Node::For { variables, source, body, line } => {
                self.for_loop(variables, source, body, *line)?;
            }
            Node::MethodCall { target, name, arguments, line } => {
                // `super.m()`: the method of the base.
                let name = self.the_name_of_the_member(name, *line)?;
                let value = if let Node::Super { .. } = &**target {
                    self.call_the_method_of_the_base(&name, arguments, *line)?
                } else {
                    let target_value = self.evaluate(target)?;
                    self.call_method_on(&target_value, &name, arguments, *line)?
                };
                detail = value.to_text();
            }
            Node::Property { target, name, line } => {
                // `super.value`: the property of the base.
                let name = self.the_name_of_the_member(name, *line)?;
                let value = if let Node::Super { .. } = &**target {
                    self.read_the_property_of_the_base(&name, *line)?
                } else {
                    let target_value = self.evaluate(target)?;
                    self.read_property(&target_value, &name, *line)?
                };
                detail = value.to_text();
            }
            Node::AssignDynamic { name, op, value, line } => {
                let name = self.the_name_of_the_pieces(name, *line)?;
                let value = self.evaluate(value)?;
                let stored = self.write_a_dynamic_name(&name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::AssignProperty { target, name, op, value, line } => {
                let value = self.evaluate(value)?;
                let name = self.the_name_of_the_member(name, *line)?;
                let stored = self.write_a_property(target, &name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::AssignIndex { target, index, op, value, line } => {
                let target_value = self.evaluate(target)?;
                let key = self.evaluate(index)?;
                let value = self.evaluate(value)?;
                let stored = self.set_item(&target_value, &key, *op, value, *line)?;
                detail = format!("{} = {}", key.to_text(), stored.to_text());
            }
            Node::Call { name, arguments, line } => {
                let value = self.call(name, arguments, *line)?;
                detail = value.to_text();
            }
            Node::If { branches, otherwise, .. } => {
                let mut taken = false;
                for branch in branches {
                    if self.evaluate(&branch.condition)?.truthy() {
                        self.run_block(&branch.body)?;
                        taken = true;
                        break;
                    }
                }
                if !taken {
                    self.run_block(otherwise)?;
                }
            }
            Node::Switch { subject, cases, line } => {
                let value = self.evaluate(subject)?;
                let mut matched = false;
                for case in cases {
                    match &case.value {
                        Some(expected) => {
                            let expected = self.evaluate(expected)?;
                            if values_equal(&value, &expected, false) {
                                self.run_block(&case.body)?;
                                matched = true;
                                break;
                            }
                        }
                        None => {
                            self.run_block(&case.body)?;
                            matched = true;
                            break;
                        }
                    }
                }
                let _ = (matched, line);
            }
            Node::While { condition, body, .. } => {
                let mut iterations = 0usize;
                while self.evaluate(condition)?.truthy() {
                    iterations += 1;
                    if iterations > Self::MAX_ITERATIONS {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: format!("the loop ran more than {} times", Self::MAX_ITERATIONS),
                            line: statement.line(),
                        }));
                    }
                    match self.run_block(body) {
                        Ok(()) => {}
                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                        Err(other) => return Err(other),
                    }
                }
            }
            Node::Try { body, catches, otherwise, finally, .. } => {
                self.run_try(body, catches, otherwise, finally)?;
            }
            Node::Super { line } => {
                // A `super` with no member: the parser refuses it, and the
                // runtime refuses it too.
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "Syntax error.\n     Specifically: super".into(),
                    line: *line,
                }));
            }
            Node::Declare { scope, names, line } => {
                // The names of the declaration are read when the script is
                // read; the value waits for the statement that carries it.
                for declared in names {
                    if let Some(value) = &declared.value {
                        // The value of a static declaration runs the first
                        // time that declaration is reached.
                        if *scope == DeclareKind::Static && self.the_static_already_ran(*line, &declared.name) {
                            continue;
                        }
                        let value = self.evaluate(value)?;
                        self.assign(&declared.name, value);
                        if *scope == DeclareKind::Static {
                            self.mark_the_static_as_ran(*line, &declared.name);
                        }
                    }
                }
                let _ = line;
            }
            Node::Throw { value, line } => {
                // A `throw` without a value throws again the value that the
                // catch of the running `try` took.
                let value = match value {
                    Some(value) => self.evaluate(value)?,
                    None => match self.caught_value.clone() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "there is nothing to throw again: no catch took a value.".into(),
                                line: *line,
                            }))
                        }
                    },
                };
                // v2 puts the name of the thing that threw the error in
                // its `What`, when the error does not carry one.
                self.name_the_thrower(&value);
                return Err(RuntimeFailure::Thrown(value));
            }
            Node::Loop { mode, arguments, body, line } => {
                match mode {
                    LoopKind::Count => {
                        match arguments.first() {
                            Some(node) => {
                                let count = self.evaluate(node)?.to_number().unwrap_or(0.0) as i64;
                                for _ in 0..count.max(0) {
                                    match self.run_block(body) {
                                        Ok(()) => {}
                                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                        Err(other) => return Err(other),
                                    }
                                }
                            }
                            None => {
                                // Measured on the binary of v2.0.19 on
                                // 2026-10-31: the loop without a count goes
                                // on until a break. The step count is the
                                // bound of it, and the body may be empty.
                                loop {
                                    self.steps_taken += 1;
                                    if self.steps_taken > Self::MAX_STEPS {
                                        return Err(RuntimeFailure::Error(RuntimeError {
                                            message: format!(
                                                "the script ran more than {} statements",
                                                Self::MAX_STEPS
                                            ),
                                            line: *line,
                                        }));
                                    }
                                    match self.run_block(body) {
                                        Ok(()) => {}
                                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                        Err(other) => return Err(other),
                                    }
                                }
                            }
                        }
                    }
                    LoopKind::Parse => {
                        let text = self.loop_argument(arguments, 0, *line)?.to_text();
                        let delimiters = match arguments.get(1) {
                            Some(node) => self.evaluate(node)?.to_text(),
                            None => String::new(),
                        };
                        // The characters that are not wanted at the ends
                        // of a field.
                        let omitted = match arguments.get(2) {
                            Some(node) => self.evaluate(node)?.to_text(),
                            None => String::new(),
                        };
                        let pieces: Vec<String> = if delimiters.is_empty() {
                            // Measured on the binary of v2.0.19 on
                            // 2026-10-31: the delimiters that are not
                            // there make every character of the text a
                            // field of its own, and the spaces are
                            // fields too.
                            text.chars()
                                .filter(|character| !omitted.contains(*character))
                                .map(|character| character.to_string())
                                .collect()
                        } else {
                            split_on_any(&text, &delimiters)
                                .into_iter()
                                .map(|piece| {
                                    piece.trim_matches(|character| omitted.contains(character)).to_string()
                                })
                                .collect()
                        };
                        for (index, piece) in pieces.iter().enumerate() {
                            self.set_global("A_Index", Value::Int(index as i64 + 1));
                            self.set_global("A_LoopField", Value::Text(piece.clone()));
                            match self.run_block(body) {
                                Ok(()) => {}
                                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                Err(other) => return Err(other),
                            }
                        }
                    }
                    LoopKind::Files => {
                        // v2 takes the pattern first and the mode second,
                        // and the folder is the one the script runs in.
                        if arguments.len() > 2 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "\"Loop Files\" accepts at most 2 parameters.".into(),
                                line: *line,
                            }));
                        }
                        let pattern = self.loop_argument(arguments, 0, *line)?.to_text();
                        let mode = match arguments.get(1) {
                            Some(node) => self.evaluate(node)?.to_text(),
                            None => String::new(),
                        };
                        let folder = self
                            .desktop()
                            .working_directory()
                            .map_err(|reason| RuntimeFailure::Error(RuntimeError { message: reason, line: *line }))?;
                        let files = self
                            .desktop()
                            .list_files(&folder, &pattern, &mode)
                            .map_err(|reason| RuntimeFailure::Error(RuntimeError { message: reason, line: *line }))?;
                        for (index, file) in files.iter().enumerate() {
                            let name = the_name_of(file);
                            let extension = the_extension_of(name);
                            self.set_global("A_Index", Value::Int(index as i64 + 1));
                            self.set_global("A_LoopFileFullPath", Value::Text(file.clone()));
                            self.set_global("A_LoopFileName", Value::Text(name.to_string()));
                            self.set_global("A_LoopFileExt", Value::Text(extension.to_string()));
                            self.set_global("A_LoopFileDir", Value::Text(the_folder_of(file).to_string()));
                            self.set_global("A_LoopFilePath", Value::Text(without_the_extension(file, extension)));
                            match self.run_block(body) {
                                Ok(()) => {}
                                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                Err(other) => return Err(other),
                            }
                        }
                    }
                    LoopKind::Read => {
                        let path = self.loop_argument(arguments, 0, *line)?.to_text();
                        let lines = self
                            .desktop()
                            .read_file(&path)
                            .map_err(|reason| RuntimeFailure::Error(RuntimeError { message: reason, line: *line }))?;
                        for (index, line_text) in lines.iter().enumerate() {
                            self.set_global("A_Index", Value::Int(index as i64 + 1));
                            // Measured on the binary of v2.0.19 on
                            // 2026-10-31: the line of the file is
                            // `A_LoopReadLine`, and `A_LoopField` is the
                            // field of the loop that parses.
                            self.set_global("A_LoopReadLine", Value::Text(line_text.clone()));
                            match self.run_block(body) {
                                Ok(()) => {}
                                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                Err(other) => return Err(other),
                            }
                        }
                    }
                    LoopKind::Reg => {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: "the registry is a Windows thing and this box has none.".into(),
                            line: *line,
                        }));
                    }
                }
            }
            Node::Directive { name, argument, line } => {
                detail = self.directive(name, argument, *line)?;
            }
            // A label names a line of the script: it does nothing by
            // itself.
            Node::Label { .. } => {}
            // The value of an expression that is a statement is computed and
            // thrown away.
            Node::Expression { value, .. } => {
                self.evaluate(value)?;
            }
            // The jump of a `Goto`: the run goes on at the label.
            Node::Goto { name, line } => {
                return Err(RuntimeFailure::Flow(Flow::Goto { name: name.clone(), line: *line }));
            }
            Node::Break { .. } => return Err(RuntimeFailure::Flow(Flow::Break)),
            Node::Continue { .. } => return Err(RuntimeFailure::Flow(Flow::Continue)),
            Node::Return { value, .. } => {
                let value = match value {
                    Some(node) => Some(self.evaluate(node)?),
                    None => None,
                };
                return Err(RuntimeFailure::Flow(Flow::Return(value)));
            }
            Node::ExitApp { code, .. } => {
                let code = match code {
                    Some(node) => self.evaluate(node)?.to_number().unwrap_or(0.0) as i32,
                    None => 0,
                };
                return Err(RuntimeFailure::Flow(Flow::Exit(code)));
            }
            Node::FunctionDef { name, .. } => {
                detail = format!("{name} is defined");
            }
            Node::Unary { op, operand, line } => {
                // `x++` and `x--` are statements of their own: they have
                // a side, and the value they leave behind is not used.
                if matches!(
                    op,
                    UnaryOp::PreIncrement
                        | UnaryOp::PreDecrement
                        | UnaryOp::PostIncrement
                        | UnaryOp::PostDecrement
                ) {
                    self.increment(*op, operand, *line)?;
                    detail = "the variable moved".into();
                } else {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: "this is an expression, not a statement".into(),
                        line: statement.line(),
                    }));
                }
            }
            Node::Index { .. } => {
                // An item that is a statement is read, and the value it
                // leaves behind is not used: v2 reads it.
                self.evaluate(statement)?;
                detail = "the item was read".into();
            }
            Node::DynamicName { .. } => {
                // A name that the script builds is a statement that reads
                // it, and the value it leaves behind is not used.
                self.evaluate(statement)?;
                detail = "the variable was read".into();
            }
            Node::Literal { .. }
            | Node::Var { .. }
            | Node::Binary { .. }
            | Node::Ternary { .. }
            | Node::Group { .. }
            | Node::Empty { .. } => {
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "this is an expression, not a statement".into(),
                    line: statement.line(),
                }));
            }
            Node::Legacy { items, .. } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the list
                // of v1 is evaluated, and its value is thrown away.
                for item in items {
                    self.evaluate(item)?;
                }
                detail = "the list of v1".into();
            }
        }

        self.steps.push(Step {
            line: statement.line(),
            op: op_name(statement).to_string(),
            detail,
            ok: true,
            elapsed_ms: self.clock.now_ms().saturating_sub(started),
        });
        Ok(())
    }

    // -- the for loop ---------------------------------------------------------------

    /// The loop that walks what an object enumerates.
    ///
    /// The source is asked once, before the loop, and the enumerator it
    /// gives is walked turn by turn. The variables of the loop go back
    /// to the values they had before it: v2 restores them.
    fn for_loop(
        &mut self,
        variables: &[String],
        source: &Node,
        body: &[Node],
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        // v2 gives the loop two variables at most: the key and the
        // value under it.
        if variables.len() > 2 {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "a for loop takes at most two variables.".into(),
                line,
            }));
        }
        // The variables keep the values they had before the loop, and
        // they get them back when the loop ends.
        let saved: Vec<(String, Option<Value>)> = variables
            .iter()
            .filter(|name| !name.is_empty())
            .map(|name| (name.clone(), self.lookup_variable(name)))
            .collect();
        let object = self.evaluate(source)?;
        let enumerator = self.the_enumerator(&object, variables.len(), line)?;
        // The enumerator has to be something that can be called: v2 fails
        // there before the body of the loop runs.
        if let Err(failure) = self.check_the_enumerator(&enumerator, line) {
            for (name, value) in saved {
                self.restore_variable(&name, value);
            }
            return Err(failure);
        }
        let mut turns = 0usize;
        let mut outcome = Ok(());
        loop {
            // The enumerator hands out the turns one by one, and the body
            // of the loop runs between them.
            let item = match self.the_next_item(&enumerator, variables, line) {
                Ok(Some(item)) => item,
                Ok(None) => break,
                Err(failure) => {
                    outcome = Err(failure);
                    break;
                }
            };
            turns += 1;
            if turns > Self::MAX_ITERATIONS {
                outcome = Err(RuntimeFailure::Error(RuntimeError {
                    message: format!("the loop ran more than {} times", Self::MAX_ITERATIONS),
                    line,
                }));
                break;
            }
            // The values of the turn are given to the variables before
            // the body runs: the key and the value, or the value alone.
            if let Err(failure) = self.set_loop_variables(variables, &item, line) {
                outcome = Err(failure);
                break;
            }
            self.set_global("A_Index", Value::Int(turns as i64));
            match self.run_block(body) {
                Ok(()) => {}
                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                Err(other) => {
                    outcome = Err(other);
                    break;
                }
            }
        }
        for (name, value) in saved {
            self.restore_variable(&name, value);
        }
        outcome
    }

    /// The enumerator of a value.
    ///
    /// The value is asked for its `__Enum`, and the enumerator it gives is
    /// the one the loop walks. A value that has no `__Enum` and is an
    /// enumerator of its own is the enumerator itself.
    fn the_enumerator(
        &mut self,
        object: &Value,
        wanted: usize,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        if self.can_enumerate(object) {
            let count = Node::Literal { value: LiteralValue::Integer(wanted as i64), line };
            return self.call_method_on(object, "__enum", &[count], line);
        }
        // An enumerator is a function of its own kind, and a loop walks it
        // the way it walks any other function.
        if crate::classes::enumerator::is_one(object) {
            return Ok(object.clone());
        }
        Err(RuntimeFailure::Error(RuntimeError {
            message: "Value not enumerable.".into(),
            line,
        }))
    }

    /// Whether the enumerator is something that can be called.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the enumerator of
    /// v2 is a function, an enumerator of the runtime, or an object with a
    /// `Call` method. A list is not one, and the tree used to walk it; a
    /// value that is not an object at all is a mismatch of the type; and an
    /// object that cannot be called is not enumerable.
    fn check_the_enumerator(&self, enumerator: &Value, line: u32) -> Result<(), RuntimeFailure> {
        match enumerator {
            // An enumerator of the runtime and a function of the script are
            // called the way they are.
            Value::Object(_) if crate::classes::enumerator::is_one(enumerator) => Ok(()),
            Value::Object(_) if self.is_a_function(enumerator) => Ok(()),
            // An object with a `Call` method is an enumerator too.
            Value::Object(_) if self.find_method_on(enumerator, "call").is_some() => Ok(()),
            // An object that cannot be called is not enumerable.
            Value::Object(_) => Err(RuntimeFailure::Error(RuntimeError {
                message: "Value not enumerable.".into(),
                line,
            })),
            // A value that is not an object at all is a mismatch of the type.
            _ => Err(RuntimeFailure::Error(RuntimeError {
                message: "Type mismatch.".into(),
                line,
            })),
        }
    }

    /// The next item of an enumeration.
    ///
    /// The enumerator of the runtime hands out the turns one by one. A
    /// function of the script is asked for the next one: it writes the
    /// values into the variables of the loop and answers one while it has
    /// something to give, and zero when it has nothing left.
    fn the_next_item(
        &mut self,
        enumerator: &Value,
        variables: &[String],
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        if crate::classes::enumerator::is_one(enumerator) {
            return Ok(crate::classes::enumerator::the_turn(enumerator, variables.len()));
        }
        // The variables of the loop are given to the enumerator by
        // reference: v2 calls it with them.
        let references: Vec<Value> = variables
            .iter()
            .filter(|name| !name.is_empty())
            .map(|name| {
                Value::Reference(Reference {
                    name: name.clone(),
                    scope: self.the_scope_of(name),
                    alias: true,
                })
            })
            .collect();
        let answer = if self.is_a_function(enumerator) {
            match crate::classes::func::call_the_value(enumerator, &references, self, line) {
                Some(result) => result?,
                None => return Err(self.no_method(enumerator, "Call", line)),
            }
        } else {
            self.own_method(enumerator, "call", &references, line)?
        };
        // The enumerator answers one while it has something to give, and
        // zero when it has nothing left.
        if !answer.truthy() {
            return Ok(None);
        }
        // The values are read back from the variables of the loop.
        let mut values = Vec::new();
        for reference in &references {
            let Value::Reference(reference) = reference else { continue };
            values.push(self.read_reference(reference).unwrap_or(Value::Text(String::new())));
        }
        let item = if values.len() >= 2 {
            Value::List(vec![values[0].clone(), values[1].clone()])
        } else {
            values.into_iter().next().unwrap_or(Value::Text(String::new()))
        };
        Ok(Some(item))
    }

    /// The scope a name lives in.
    ///
    /// The scope of a name is the one the write of it goes to: the scope of
    /// the script, the scope of a function around it that names it, or the
    /// scope of the function that runs.
    fn the_scope_of(&self, name: &str) -> Scope {
        let name = the_key_of(name);
        if self.lives_in_the_script(&name) {
            return Scope::Global;
        }
        // A name that a scope around the call has is shared with it.
        for (depth, scope) in self.scopes.iter().enumerate().rev().skip(1) {
            if scope.borrow().contains_key(&name) {
                return Scope::Local(depth);
            }
        }
        match self.scopes.len() {
            0 => Scope::Global,
            depth => Scope::Local(depth - 1),
        }
    }

    /// Whether a value can be walked.
    ///
    /// The enumerator of a value is its `__Enum`: a class of the script
    /// may have one, and the class types of the runtime have their own.
    fn can_enumerate(&self, object: &Value) -> bool {
        let Value::Object(value) = object else { return false };
        if self.find_method_on(object, "__enum").is_some() {
            return true;
        }
        let class = value.borrow().class.clone();
        crate::classes::carries(&class, "__enum")
    }

    /// The variables of a turn of a `for` loop.
    ///
    /// With one variable the enumerator gives the value, with two it
    /// gives the key and the value under it. A name the script left
    /// out is given to nobody.
    fn set_loop_variables(
        &mut self,
        variables: &[String],
        item: &Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        match variables.len() {
            0 => Ok(()),
            1 => {
                // The enumerator gives the value alone.
                if !variables[0].is_empty() {
                    self.assign(&variables[0], item.clone());
                }
                Ok(())
            }
            _ => {
                // The enumerator gives the key and the value under it.
                let pair = match item {
                    Value::List(pair) if pair.len() == 2 => pair,
                    _ => {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: "the enumeration did not give a key and a value.".into(),
                            line,
                        }))
                    }
                };
                for (name, value) in variables.iter().zip(pair.iter()) {
                    if name.is_empty() {
                        continue;
                    }
                    self.assign(name, value.clone());
                }
                Ok(())
            }
        }
    }

    /// A variable of a loop, put back the way it was.
    ///
    /// v2 restores the variables of a loop when the loop ends: a name
    /// that had no value before it has no value after it either.
    fn restore_variable(&mut self, name: &str, value: Option<Value>) {
        match value {
            Some(value) => self.assign(name, value),
            None => match self.scopes.last() {
                Some(scope) => {
                    scope.borrow_mut().remove(the_key_of(name).as_str());
                }
                None => {
                    self.variables.remove(the_key_of(name).as_str());
                }
            },
        }
    }

    // -- scopes ---------------------------------------------------------------------

    /// The value of a variable, from the innermost scope outwards.
    ///
    /// A function and a class are values too: a script can hold one and
    /// hand it on, and `Func` is the class of a function.
    fn lookup(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.lookup_variable(name) {
            return Some(value);
        }
        let key = name.to_lowercase();
        if let Some(function) = self.functions.get(&key) {
            return Some(func_value(&function.name, function));
        }
        // A built-in is a function, and a function is a value. The name
        // of it comes before the class: the `Hotkey` is a function of the
        // dialect, and the class of a hotkey is the class of the object
        // that the function makes.
        if let Some(builtin) = crate::builtins::find(&key) {
            return Some(builtin_value(builtin.name, builtin.min, builtin.max));
        }
        if self.classes.contains_key(&key) || crate::classes::is_type(&key) {
            return self.class_value_of(&key);
        }
        None
    }

    /// The value of a class: the class itself.
    ///
    /// The class is one object for the whole run, so that the `is`
    /// operator and the base of an object can tell a class from an
    /// instance of one.
    pub(crate) fn class_value_of(&self, name: &str) -> Option<Value> {
        let key = name.to_lowercase();
        if let Some(value) = self.class_values.borrow().get(&key) {
            return Some(value.clone());
        }
        let value = if let Some(class) = self.classes.get(&key) {
            class_value(&key, class)
        } else if crate::classes::is_type(&key) {
            crate::classes::class_value(&key)
        } else {
            return None;
        };
        self.class_values.borrow_mut().insert(key, value.clone());
        Some(value)
    }

    /// The name of the class of an object, the way v2 names it.
    ///
    /// An object is named after the class it is an instance of, and a
    /// class is named after itself.
    pub(crate) fn class_of(&self, target: &Value) -> Option<String> {
        let Value::Object(object) = target else { return None };
        let class = object.borrow().class.clone();
        // The name the class was written with, when the runtime knows it.
        if let Some(Value::Object(class)) = self.class_value_of(&class) {
            if let Some(name) = class.borrow().value_of(CLASS_NAME) {
                return Some(name.to_text());
            }
        }
        // The other objects are named after the class they carry.
        let mut name = class;
        if let Some(first) = name.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        Some(name)
    }

    /// The name of the class a value is, when the value is a class.
    ///
    /// The classes the runtime made are known to it, and it can tell one
    /// of them from an instance of one.
    pub(crate) fn class_name_of(&self, value: &Value) -> Option<String> {
        let Value::Object(object) = value else { return None };
        for class in self.class_values.borrow().values() {
            let Value::Object(other) = class else { continue };
            if Rc::ptr_eq(object, other) {
                return object
                    .borrow()
                    .value_of(CLASS_NAME)
                    .map(|name| name.to_text().to_lowercase());
            }
        }
        None
    }

    /// The base of an object: the prototype of its class.
    ///
    /// v2 gives every object a base, and the base of an instance is the
    /// prototype of the class it is an instance of. A script may set
    /// another one with `ObjSetBase`.
    pub(crate) fn base_of(&self, target: &Value) -> Option<Value> {
        let Value::Object(object) = target else { return None };
        // A base that the script set is the base of the object.
        if let Some(base) = object.borrow().value_of(BASE) {
            return Some(base.clone());
        }
        let class = object.borrow().class.clone();
        let Value::Object(class) = self.class_value_of(&class)? else { return None };
        class.borrow().value_of("prototype")
    }

    /// The base of an object that the script set, if it set one.
    ///
    /// An object of a script has no base of its own until `ObjSetBase`
    /// gives it one: the prototype of its class is what `ObjGetBase`
    /// answers with.
    pub(crate) fn stored_base_of(&self, target: &Value) -> Option<Value> {
        let Value::Object(object) = target else { return None };
        object.borrow().value_of(BASE)
    }

    /// A method of the base object, asked of an object.
    ///
    /// The class of the object answers first when it owns the whole of
    /// its properties: the state of a map is not one of them.
    pub(crate) fn own_method(
        &mut self,
        target: &Value,
        name: &str,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeError> {
        let Value::Object(object) = target else {
            return Err(RuntimeError {
                message: expected_type(TypeName::Object, target.type_name()),
                line,
            });
        };
        // A class value of the runtime: the own properties of it are the
        // ones that v2 gives it, and not the table that the tree keeps.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the own
        // properties of the class value of the `Map` are "Call" and
        // "Prototype".
        if name == "ownprops" && self.class_name_of(target).is_some() {
            return self.base_object(target, name, arguments, line);
        }
        let class = object.borrow().class.clone();
        match crate::classes::call(&class, name, target, arguments, self, line) {
            Some(Ok(value)) => Ok(value),
            Some(Err(failure)) => Err(as_error(failure, line)),
            None => self.base_object(target, name, arguments, line),
        }
    }

    /// The property of an object, when it has one.
    pub(crate) fn property_of(&mut self, target: &Value, name: &str, line: u32) -> Option<Value> {
        self.read_property(target, name, line).ok()
    }

    /// The property of an object, written.
    pub(crate) fn write_property_of(
        &mut self,
        target: &Value,
        name: &str,
        value: Value,
        line: u32,
    ) -> Result<(), RuntimeError> {
        self.write_property(target, name, value, line)
            .map_err(|failure| as_error(failure, line))
    }

    /// The base of an object, set to another one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the base of an
    /// object of a class type of the runtime cannot be written -- the
    /// prototype of the class is there -- and the refusal says "Invalid
    /// base.". The base of a plain object, of an error, of a prototype, of
    /// a class value and of an instance of a class of the script can be
    /// written.
    pub(crate) fn set_base_of(
        &mut self,
        target: &Value,
        base: Value,
        line: u32,
    ) -> Result<(), RuntimeError> {
        let Value::Object(object) = target else {
            return Err(RuntimeError {
                message: expected_type(TypeName::Object, target.type_name()),
                line,
            });
        };
        let class = object.borrow().class.clone();
        // A class value is not an instance of its class, and the base of it
        // is the class that it extends: the write of it is the write of the
        // `ObjSetBase` of a class.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31.
        if self.class_name_of(target).is_none()
            && matches!(
                class.as_str(),
                "array" | "buffer" | "enumerator" | "file" | "func" | "gui" | "map" | "menu"
            )
        {
            return Err(RuntimeError { message: "Invalid base.".to_string(), line });
        }
        object.borrow_mut().put(BASE, base);
        Ok(())
    }

    /// The value of a variable, from the innermost scope outwards.
    ///
    /// The functions and the classes are not variables, and this is the
    /// lookup that says so.
    ///
    /// A parameter that is an alias of a variable of the caller reads
    /// that variable: the alias and the variable are the same thing.
    fn lookup_variable(&self, name: &str) -> Option<Value> {
        let name = the_key_of(name);
        // The scope of the call is the last one: the parameters of the
        // function are there.
        if let Some(scope) = self.scopes.last() {
            match scope.borrow().get(&name) {
                Some(Value::Reference(reference)) if reference.alias => {
                    return self.read_reference(reference);
                }
                Some(value) => return Some(value.clone()),
                None => {}
            }
        }
        // The static of the function that runs belongs to it: it is not in
        // the scope of the call, and it is read before the scopes around
        // the call.
        if let Some(value) = self.the_static_of_the_frame(&name) {
            return Some(value);
        }
        for scope in self.scopes.iter().rev().skip(1) {
            match scope.borrow().get(&name) {
                Some(Value::Reference(reference)) if reference.alias => {
                    return self.read_reference(reference);
                }
                Some(value) => return Some(value.clone()),
                None => continue,
            }
        }
        // A name that the function declares local is the local one, and not
        // the global of the script: a local that has no value yet has no
        // value at all.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `local g` and a
        // global `g` are two names, and the local is the one that the
        // function reads.
        if self.is_declared_local(&name) {
            return None;
        }
        self.variables.get(&name).cloned()
    }

    /// Whether the name is one that the function that runs declares
    /// static.
    ///
    /// A name that the function declares static is not a local one: it
    /// lives in the storage of the function.
    fn is_declared_static(&self, name: &str) -> bool {
        let Some(frame) = self.declarations.last() else { return false };
        frame.statics.contains(name)
    }


    /// The static variables of the function that runs.
    ///
    /// The frame is the name of the function or of the method that runs,
    /// and the table of the function is under it.
    fn the_statics_of_the_frame(&mut self) -> &mut BTreeMap<String, Value> {
        let frame = self.frames.last().map(|frame| frame.name.clone()).unwrap_or_default();
        self.statics.entry(frame).or_default()
    }


    /// The value of a static variable of the function that runs.
    fn the_static_of_the_frame(&self, name: &str) -> Option<Value> {
        let frame = self.frames.last()?;
        self.statics.get(&frame.name)?.get(name).cloned()
    }


    /// Whether the value of a static declaration already ran.
    ///
    /// The value of a static declaration runs the first time that
    /// declaration is reached.
    fn the_static_already_ran(&self, line: u32, name: &str) -> bool {
        let Some(frame) = self.frames.last() else { return false };
        let Some(ran) = self.statics_run.get(&frame.name) else { return false };
        ran.contains(&the_key_of_a_declaration(line, name))
    }


    /// Mark the value of a static declaration as one that ran.
    fn mark_the_static_as_ran(&mut self, line: u32, name: &str) {
        let frame = self.frames.last().map(|frame| frame.name.clone()).unwrap_or_default();
        self.statics_run
            .entry(frame)
            .or_default()
            .insert(the_key_of_a_declaration(line, name));
    }


    /// Whether the name is one that the function that runs declares local.
    ///
    /// A name that the function declares global is not one of them, and a
    /// name that the function does not name is not one either: a function
    /// that assumes the scope of the script says it for every name it does
    /// not name.
    fn is_declared_local(&self, name: &str) -> bool {
        let Some(frame) = self.declarations.last() else { return false };
        frame.locals.contains(name)
    }

    /// The refusal of a local that takes the class of its own name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: `foo := Foo()` in a
    /// function is refused, and so is `foo := Foo`, and a local that is
    /// declared says the same words without the hint. The check is of a
    /// function: at the top level the parser refuses the assignment when it
    /// reads the script.
    fn the_class_is_not_written(
        &self,
        name: &str,
        value: &Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        if self.declarations.is_empty() {
            return Ok(());
        }
        let key = the_key_of(name);
        if self.lives_in_the_script(&key) {
            return Ok(());
        }
        let Some(class) = self.the_class_of_the_name(&key) else { return Ok(()) };
        if self.the_class_key_of_the_value(value).as_deref() != Some(class.as_str()) {
            return Ok(());
        }
        let message = if self.is_declared_local(&key) {
            "This local variable has not been assigned a value.".to_string()
        } else {
            "This local variable has not been assigned a value. A global declaration inside the function may be required.".to_string()
        };
        Err(RuntimeFailure::Error(RuntimeError { message, line }))
    }

    /// The class of that name, when the name is the name of a class.
    ///
    /// The classes of the script and the classes of the runtime are both
    /// names of the script, and the key of the table is the name in lower
    /// case.
    fn the_class_of_the_name(&self, name: &str) -> Option<String> {
        if self.classes.contains_key(name) || crate::classes::is_type(name) {
            return Some(name.to_string());
        }
        None
    }

    /// The class of a value, in the key of the table of the classes.
    ///
    /// The class of an instance is the class it is an instance of, and the
    /// class of a class is the class itself. The name that v2 shows is
    /// another one: a class is a "Class" there.
    fn the_class_key_of_the_value(&self, value: &Value) -> Option<String> {
        let Value::Object(object) = value else { return None };
        Some(object.borrow().class.clone())
    }

    /// The alias of this name, if the name is one.
    ///
    /// A parameter written `&x` in the definition of a function is an
    /// alias of the variable of the caller.
    fn alias_of(&self, name: &str) -> Option<Reference> {
        let name = the_key_of(name);
        for scope in self.scopes.iter().rev() {
            let Some(found) = scope.borrow().get(&name).cloned() else { continue };
            if let Value::Reference(reference) = found {
                if reference.alias {
                    return Some(reference);
                }
            }
            return None;
        }
        None
    }

    /// The value of the variable a reference points at.
    ///
    /// The answer is nothing when the variable has no value: that is the
    /// difference between a variable that is set and one that is not.
    pub(crate) fn read_reference(&self, reference: &Reference) -> Option<Value> {
        let name = the_key_of(&reference.name);
        match reference.scope {
            Scope::Global => self.variables.get(&name).cloned(),
            Scope::Local(depth) => self.scopes.get(depth)?.borrow().get(&name).cloned(),
        }
    }

    /// Write into the variable a reference points at.
    ///
    /// This is what a built-in does with the variable of the caller that
    /// it was given by reference.
    pub(crate) fn write_reference(&mut self, value: &Value, written: Value) {
        let Value::Reference(reference) = value else { return };
        let name = the_key_of(&reference.name);
        match reference.scope {
            Scope::Global => {
                self.variables.insert(name, written);
            }
            Scope::Local(depth) => {
                if let Some(scope) = self.scopes.get(depth) {
                    scope.borrow_mut().insert(name, written);
                }
            }
        }
    }

    /// Set a variable: in the innermost scope when there is one, in the
    /// global one otherwise. A function that assigns a name creates a local
    /// one, the way AutoHotkey v2 does.
    ///
    /// A parameter that is an alias of a variable of the caller writes
    /// there: the alias and the variable are the same thing.
    fn assign(&mut self, name: &str, value: Value) {
        if let Some(reference) = self.alias_of(name) {
            self.write_reference(&Value::Reference(reference), value);
            return;
        }
        let name = the_key_of(name);
        // A static variable belongs to the function that declares it: it is
        // written in the table of the function, and it is not in the scope
        // of the call.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the static
        // shadows a global of the same name inside the function.
        if self.is_declared_static(&name) {
            self.the_statics_of_the_frame().insert(name, value);
            return;
        }
        // A name that a function declares global lives in the scope of the
        // script, and a name that it declares local lives in the scope of
        // the function. A function that assumes the scope of the script
        // says it for every name it does not name.
        if self.lives_in_the_script(&name) {
            self.variables.insert(name, value);
            return;
        }
        // A name that a scope around the call has is shared with it: v2
        // writes there, and not in the scope of the function that closes
        // over it.
        for scope in self.scopes.iter().rev().skip(1) {
            if scope.borrow().contains_key(&name) {
                scope.borrow_mut().insert(name, value);
                return;
            }
        }
        match self.scopes.last() {
            Some(scope) => {
                scope.borrow_mut().insert(name, value);
            }
            None => {
                self.variables.insert(name, value);
            }
        }
    }

    /// Whether a name lives in the scope of the script.
    ///
    /// The frame of the function that runs says: the innermost one that
    /// names the variable decides, and a function that assumes the scope
    /// of the script says it for every name it does not name.
    fn lives_in_the_script(&self, name: &str) -> bool {
        let Some(frame) = self.declarations.last() else { return false };
        if frame.globals.contains(name) {
            return true;
        }
        if frame.locals.contains(name) {
            return false;
        }
        frame.assume_global
    }

    /// The kind of a name whose reference cannot be taken.
    ///
    /// A built-in variable, a function and a class are not variables of
    /// the script, and v2 says which one of them it is.
    fn cannot_be_referenced(&self, name: &str) -> Option<&'static str> {
        let key = name.to_lowercase();
        if crate::variables::is_built_in(name) {
            return Some("built-in variable");
        }
        if self.functions.contains_key(&key) || crate::builtins::find(&key).is_some() {
            return Some("Func");
        }
        if self.classes.contains_key(&key) || crate::classes::is_type(&key) {
            return Some("Class");
        }
        None
    }

    /// The chain of the bases of an object.
    ///
    /// The base of an object is the prototype of its class, and the base of
    /// a prototype is the prototype of the class that the class extends.
    /// Every chain ends at the prototype of the base object.
    fn the_chain_of_the_bases(&self, target: &Value) -> Vec<Rc<RefCell<Object>>> {
        let mut chain: Vec<Rc<RefCell<Object>>> = Vec::new();
        let Value::Object(object) = target else { return chain };
        let mut class = object.borrow().class.clone();
        // A class is not an instance of itself: the chain of a class begins
        // at the base of the class, and not at its own prototype.
        if self.class_name_of(target).is_some() {
            let Some(base) = self.the_base_class_of(&class) else { return chain };
            class = base;
        }
        // A base that the script set is the base of the object.
        if let Some(Value::Object(base)) = object.borrow().value_of(BASE) {
            chain.push(base.clone());
            class = base.borrow().class.clone();
        }
        while let Some(prototype) = self.the_prototype_of_the_class(&class) {
            if chain.iter().any(|seen| Rc::ptr_eq(seen, &prototype)) {
                break;
            }
            chain.push(prototype);
            let Some(base) = self.the_base_class_of(&class) else { break };
            class = base;
        }
        chain
    }

    /// The prototype of a class, the object that holds its methods.
    fn the_prototype_of_the_class(&self, class: &str) -> Option<Rc<RefCell<Object>>> {
        let Some(Value::Object(value)) = self.class_value_of(class) else { return None };
        let prototype = value.borrow().value_of("prototype");
        let Some(Value::Object(prototype)) = prototype else { return None };
        Some(prototype)
    }

    /// The class that a class extends.
    ///
    /// Every class of the script and every class of the runtime derives
    /// from the base object, and the enumerator and the errors have a base
    /// of their own.
    fn the_base_class_of(&self, class: &str) -> Option<String> {
        if let Some(definition) = self.classes.get(class) {
            if let Some(base) = &definition.extends {
                return Some(base.to_lowercase());
            }
            return Some("object".to_string());
        }
        if let Some(base) = crate::classes::the_base_of(class) {
            return Some(base.to_string());
        }
        Some("object".to_string())
    }

    /// The names of a class and of the classes it extends, the class
    /// first.
    fn class_chain(&self, name: &str) -> Vec<String> {
        let mut chain: Vec<String> = Vec::new();
        let mut current = Some(name.to_lowercase());
        while let Some(class) = current {
            // A class that extends itself, directly or through another,
            // would loop forever.
            if chain.contains(&class) {
                break;
            }
            chain.push(class.clone());
            current = match self.classes.get(&class) {
                Some(definition) => definition.extends.clone().map(|base| base.to_lowercase()),
                // The class types of the runtime have their own bases: the
                // errors of v2 are a tree, and every one of them is an
                // `Error`.
                None => crate::classes::the_base_of(&class).map(|base| base.to_string()),
            };
        }
        chain
    }

    /// The method a name asks for, on a class, and the class that has it.
    ///
    /// The class matters: `super` asks the base of the class where the
    /// method was defined, and not the base of the class of the object.
    fn find_method_in(&self, class: &str, name: &str) -> Option<(String, MethodDef)> {
        let name = name.to_lowercase();
        for class in self.class_chain(class) {
            if let Some(definition) = self.classes.get(&class) {
                if let Some(method) = definition.methods.get(&name) {
                    return Some((class.clone(), method.clone()));
                }
            }
        }
        None
    }

    /// The method a name asks for, on a value.
    ///
    /// A static method belongs to the class and not to its instances, and
    /// the prototype of a class carries the methods of the class.
    fn find_method_on(&self, target: &Value, name: &str) -> Option<(String, MethodDef)> {
        let Value::Object(object) = target else { return None };
        let class = object.borrow().class.clone();
        // A class value is not an instance of its class: the methods of the
        // class are not its own, and only the static ones are.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the call of a
        // method of an instance is refused with 'This value of type "Class"
        // has no method named "Say".'
        if self.class_name_of(target).is_some() {
            return self.find_the_static_method_in(&class, name);
        }
        // A static method is not a member of an instance, and it is not a
        // member of the prototype of the class either.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `obj.Say()` is
        // refused with 'This value of type "C" has no method named "Say".',
        // and a static method of a class does not hide the instance method
        // of its base: the instance has that one.
        let class = self.the_class_of_the_prototype(target).unwrap_or(class);
        self.find_the_instance_method_in(&class, name)
    }

    /// The object a class makes.
    fn construct(&mut self, class: &str, arguments: &[Node], line: u32) -> Result<Value, RuntimeFailure> {
        // A class type of the runtime makes its own object: the script
        // has no class of that name, and cannot extend one.
        if !self.classes.contains_key(&class.to_lowercase()) {
            let values = self.evaluate_all(arguments)?;
            return self.construct_with_values(class, &values, line);
        }
        let object = self.the_object_of_a_class(class);
        // The `__Init` of v2 runs before the `__New`, and the variables of
        // the class are assigned in it.
        self.call_the_init(class, &object, line)?;
        // `__New` runs with the new object as `this`, and may answer an
        // object of its own.
        if let Some((defining, method)) = self.find_the_instance_method_in(class, "__new") {
            let result = self.call_method(&defining, &method, &object, arguments, line)?;
            return Ok(match result {
                Value::Object(_) => result,
                _ => object,
            });
        }
        Ok(object)
    }

    /// The object a class makes, with the arguments already read.
    fn construct_with_values(
        &mut self,
        class: &str,
        values: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        if !self.classes.contains_key(&class.to_lowercase()) {
            if let Some(result) = crate::classes::construct(class, values, self, line) {
                return result;
            }
        }
        let object = self.the_object_of_a_class(class);
        self.call_the_init(class, &object, line)?;
        if let Some((defining, method)) = self.find_the_instance_method_in(class, "__new") {
            let result = self.call_method_with(&defining, &method, &object, values, line)?;
            return Ok(match result {
                Value::Object(_) => result,
                _ => object,
            });
        }
        Ok(object)
    }

    /// The object of a class: an object of that class, with nothing in it.
    fn the_object_of_a_class(&self, class: &str) -> Value {
        Value::Object(Rc::new(RefCell::new(Object {
            class: class.to_lowercase(),
            properties: BTreeMap::new(),
        })))
    }

    /// The `__Init` of an instance, the way v2 calls it.
    ///
    /// The most derived `__Init` is the one that runs: the explicit one of
    /// the class, or the automatic one that carries the variables of the
    /// class. The automatic one calls the `__Init` of the base before it
    /// assigns the variables, and the explicit one calls nothing: the
    /// variables of the base are not assigned when the derived has an
    /// `__Init` of its own.
    fn call_the_init(&mut self, class: &str, object: &Value, line: u32) -> Result<(), RuntimeFailure> {
        let chain = self.class_chain(class);
        self.call_the_init_of(&chain, object, line)
    }

    /// The initialisation of a class of v2.
    ///
    /// A class is initialised when it is first used: the statement that
    /// defines it and the first reference to it. The variables that belong
    /// to the class are assigned then, and the `__New` that is static is
    /// the one that runs.
    fn initialise_the_class(&mut self, class: &str, line: u32) -> Result<(), RuntimeFailure> {
        let key = class.to_lowercase();
        // A class of the runtime has no variables of its own.
        if !self.classes.contains_key(&key) {
            return Ok(());
        }
        if !self.initialised.borrow_mut().insert(key.clone()) {
            // The class is already initialised.
            return Ok(());
        }
        self.assign_the_static_variables(&key)?;
        if let Some((defining, method)) = self.find_the_static_method_in(&key, "__new") {
            if let Some(value) = self.class_value_of(&key) {
                self.call_method(&defining, &method, &value, &[], line)?;
            }
        }
        Ok(())
    }

    /// The variables that belong to a class, assigned to the class.
    fn assign_the_static_variables(&mut self, class: &str) -> Result<(), RuntimeFailure> {
        let Some(definition) = self.classes.get(class).cloned() else { return Ok(()) };
        let Some(value) = self.class_value_of(class) else { return Ok(()) };
        let Value::Object(object) = value else { return Ok(()) };
        for property in &definition.properties {
            if !property.is_static {
                continue;
            }
            let value = match &property.value {
                Some(node) => self.evaluate(node)?,
                None => Value::Text(String::new()),
            };
            put(&mut object.borrow_mut().properties, &property.name, value);
        }
        Ok(())
    }

    /// The method a name asks for, on a class, and that is static.
    ///
    /// A method that belongs to the class is not one of an instance: the
    /// `__New` that is static is the initialiser of the class, and it does
    /// not run when an instance is made.
    fn find_the_static_method_in(&self, class: &str, name: &str) -> Option<(String, MethodDef)> {
        let name = name.to_lowercase();
        for class in self.class_chain(class) {
            if let Some(definition) = self.classes.get(&class) {
                if let Some(method) = definition.methods.get(&name) {
                    if method.is_static {
                        return Some((class.clone(), method.clone()));
                    }
                }
            }
        }
        None
    }

    /// The method a name asks for, on a class, and that is not static.
    fn find_the_instance_method_in(&self, class: &str, name: &str) -> Option<(String, MethodDef)> {
        let name = name.to_lowercase();
        for class in self.class_chain(class) {
            if let Some(definition) = self.classes.get(&class) {
                if let Some(method) = definition.methods.get(&name) {
                    if !method.is_static {
                        return Some((class.clone(), method.clone()));
                    }
                }
            }
        }
        None
    }

    /// The `__Init` of an instance, over the chain of the classes.
    ///
    /// The chain goes from the class itself to the base of it.
    fn call_the_init_of(
        &mut self,
        chain: &[String],
        object: &Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        for (index, name) in chain.iter().enumerate() {
            let Some(definition) = self.classes.get(name).cloned() else { continue };
            if let Some(method) = definition.methods.get("__init").cloned() {
                // The `__Init` of this class: it runs, and the ones of the
                // base do not.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // properties of the class are assigned after the body of the
                // `__Init`, and not before it -- the body may write them,
                // and the assignment of the class is the one that is left.
                self.call_method(name, &method, object, &[], line)?;
                self.assign_the_variables(name, object)?;
                return Ok(());
            }
            if !definition.properties.is_empty() {
                // The automatic `__Init`: the base comes first, and then
                // the variables of this class.
                if index + 1 < chain.len() {
                    self.call_the_init_of(&chain[index + 1..], object, line)?;
                }
                self.assign_the_variables(name, object)?;
                return Ok(());
            }
        }
        Ok(())
    }

    /// The variables of a class, assigned to the object.
    fn assign_the_variables(&mut self, class: &str, object: &Value) -> Result<(), RuntimeFailure> {
        let Some(definition) = self.classes.get(class).cloned() else { return Ok(()) };
        let Value::Object(cell) = object else { return Ok(()) };
        for property in &definition.properties {
            // A variable that belongs to the class is not one of the
            // instance: it is the class that has it.
            if property.is_static {
                continue;
            }
            let value = match &property.value {
                Some(node) => self.evaluate(node)?,
                None => Value::Text(String::new()),
            };
            // A variable of the class that has a getter of its own is one
            // name and not two: the write of it goes through the property.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: the
            // construction of the class is refused with "Property is
            // read-only." when the property has no setter.
            if self.the_accessor_of(object, &property.name).is_some() {
                self.the_write_of_an_accessor(object, &property.name, value, 0)?;
                continue;
            }
            put(&mut cell.borrow_mut().properties, &property.name, value);
        }
        Ok(())
    }

    /// The values of a list of arguments.
    fn evaluate_all(&mut self, arguments: &[Node]) -> Result<Vec<Value>, RuntimeFailure> {
        let mut values = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        Ok(values)
    }

    /// The base of the class where the method that runs was defined.
    fn the_base_of_the_running_method(&self) -> Option<String> {
        let class = self.current_classes.last()?;
        self.class_chain(class).get(1).cloned()
    }

    /// The value the method that runs was called on.
    fn the_this(&self) -> Value {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.borrow().get("this") {
                return value.clone();
            }
        }
        Value::Text(String::new())
    }

    /// The class of the value the method that runs was called on.
    ///
    /// The name is the one the class was written with: v2 names the class
    /// of the value in its refusals.
    fn the_class_of_this(&self) -> String {
        self.class_of(&self.the_this()).unwrap_or_default()
    }

    /// The method of the base of the class where the method was defined.
    ///
    /// `super.m()` is this one.
    fn call_the_method_of_the_base(
        &mut self,
        name: &str,
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // Every class of v2 has an `__Init`: the automatic one that carries
        // the variables of the class. So `super.__Init()` runs the one of
        // the base, and it never fails for want of a method.
        if name.eq_ignore_ascii_case("__init") {
            let this = self.the_this();
            if let Some(base) = self.the_base_of_the_running_method() {
                let chain = self.class_chain(&base);
                self.call_the_init_of(&chain, &this, line)?;
            }
            return Ok(Value::Text(String::new()));
        }
        let found = self
            .the_base_of_the_running_method()
            .and_then(|base| self.find_method_in(&base, name));
        let Some((defining, method)) = found else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!(
                    "This value of type \"{}\" has no method named \"{name}\".",
                    self.the_class_of_this()
                ),
                line,
            }));
        };
        let this = self.the_this();
        self.call_method(&defining, &method, &this, arguments, line)
    }

    /// The property of the base, the way v2 reads it.
    ///
    /// `super.value` is this one: the base has to have the getter, and the
    /// value of the object alone is not enough.
    fn read_the_property_of_the_base(
        &mut self,
        name: &str,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let base = self.the_base_of_the_running_method();
        let this = self.the_this();
        if let Some(base) = base {
            // The base has to have the getter: the property of the object
            // alone is not enough. Measured on the binary of v2.0.19 on
            // 2026-10-30, `super.value` refuses when the base only has
            // `this.value := ...`.
            if let Some((defining, method)) = self.find_method_in(&base, "__get") {
                let params = self.the_params(&[], line)?;
                return self.call_method_with(
                    &defining,
                    &method,
                    &this,
                    &[Value::Text(name.to_string()), params],
                    line,
                );
            }
        }
        Err(RuntimeFailure::Error(RuntimeError {
            message: format!(
                "This value of type \"{}\" has no property named \"{name}\".",
                self.the_class_of_this()
            ),
            line,
        }))
    }

    /// The item of the base, the way v2 reads it.
    ///
    /// `super["value"]` is this one.
    fn read_the_item_of_the_base(
        &mut self,
        key: &Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let base = self.the_base_of_the_running_method();
        let this = self.the_this();
        if let Some(base) = base {
            if let Some((defining, method)) = self.find_method_in(&base, "__item") {
                let arguments = [Node::Literal { value: LiteralValue::Text(key.to_text()), line }];
                return self.call_method(&defining, &method, &this, &arguments, line);
            }
        }
        Err(RuntimeFailure::Error(RuntimeError {
            message: format!(
                "This value of type \"{}\" has no property named \"__Item\".",
                self.the_class_of_this()
            ),
            line,
        }))
    }

    /// A method, called on a value.
    ///
    /// The class of the script answers first. A class type of the
    /// runtime answers for itself, and the two never overlap: the
    /// script cannot extend a class type of the runtime.
    fn call_method_on(
        &mut self,
        target: &Value,
        name: &str,
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        if let Some((defining, method)) = self.find_method_on(target, name) {
            return self.call_method(&defining, &method, target, arguments, line);
        }
        // A class is not an instance of itself: the call of a class value
        // makes an object of the class, the way `Map()` does, and the
        // copy of a class is a class and not an object of it.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31.
        if self.class_name_of(target).is_some() {
            if name.eq_ignore_ascii_case("call") {
                let class = self.class_name_of(target).unwrap_or_default();
                return self.construct(&class, arguments, line);
            }
            if name.eq_ignore_ascii_case("clone") {
                return Ok(self.clone_of_a_class(target));
            }
            // The methods of a class are the static ones, and the tree
            // hands them over as values.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31.
            if name.eq_ignore_ascii_case("getmethod") {
                let values = self.evaluate_all(arguments)?;
                return self.the_static_method(target, &values, line);
            }
            // A class that the class holds is a class of its own, and the
            // call of it makes an object of it.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31:
            // `Outer.Inner()` is an object of the class inside `Outer`.
            if let Some(value) = self.the_class_that_a_class_holds(target, name) {
                let class = self.class_name_of(&value).unwrap_or_default();
                return self.construct(&class, arguments, line);
            }
        }
        // A property of the class that has a getter and that takes
        // parameters: the call of it invokes the getter, and the parameters
        // of the property are the ones of the index.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `obj.value("a")`
        // is refused with "Missing a required parameter." when the property
        // takes parameters, and a property without parameters is not called
        // at all.
        if let Some(access) = self.the_accessor_of(target, name) {
            if !access.parameters.is_empty() {
                let Some(getter) = &access.getter else {
                    return Err(self.no_method(target, name, line));
                };
                return self.call_the_body_of_an_accessor(target, &access, getter, line, &[], None);
            }
        }
        // The automatic `__Init` of the class, the one that carries the
        // variables: `d.__Init()` runs it again.
        if name.eq_ignore_ascii_case("__init") {
            if let Value::Object(object) = target {
                let class = object.borrow().class.clone();
                let chain = self.class_chain(&class);
                self.call_the_init_of(&chain, target, line)?;
                return Ok(Value::Text(String::new()));
            }
        }
        let values = self.evaluate_all(arguments)?;
        // A property that is a descriptor may have a call of its own: the
        // property of the object shadows the method of the class.
        if let Some(call) = self.the_call_of_the_descriptor(target, name) {
            return self.call_the_function(&call, target, &values, line);
        }
        // A property that has no call of its own may be a function: v2
        // asks the getter for the value of it and calls that.
        if let Some(value) = self.the_value_of_the_descriptor_for_a_call(target, name, line)? {
            if self.is_a_function(&value) {
                return self.call_the_function(&value, target, &values, line);
            }
        }
        let class = match target {
            Value::Object(object) => object.borrow().class.clone(),
            _ => String::new(),
        };
        // The number of the arguments is checked here: v2 checks it when
        // the script writes the call, and the failure carries the name of
        // the method.
        if let Some((min, max)) = crate::classes::the_arity_of(&class, name) {
            let count = values.len() as i64;
            if count < min || (max >= 0 && count > max) {
                let message = if count < min {
                    "Too few parameters passed to function."
                } else {
                    "Too many parameters passed to function."
                };
                return Err(crate::classes::the_arity_failure(&class, name, message, line));
            }
        }
        // A class value is not an instance of its class: the methods of the
        // class do not answer it.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Get("a")` and
        // `Map.Clear()` are refused.
        if self.class_name_of(target).is_none() {
            if let Some(result) = crate::classes::call(&class, name, target, &values, self, line) {
                return result;
            }
        }
        // The prototype of an `Any`, of a `ComObject` and of a `ComValue` is
        // not a prototype: it has no method at all.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the refusal says
        // 'This value of type "Prototype" has no method named "OwnProps".',
        // and the name of the member is not in it.
        if let Some(class) = self.the_class_of_the_prototype(target) {
            if crate::classes::the_prototype_is_not_one(&class) {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!(
                            "This value of type \"{}\" has no method named \"{}\".",
                            self.the_type_of(target),
                            name
                        ),
                        line,
                    },
                    Detail { what: String::new(), extra: String::new() },
                ));
            }
        }
        // A method of the class cannot be called on the prototype of it: the
        // value is not an instance of the class.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31:
        // `Map.Prototype.Get("a")` is refused with "Expected a Map but got a
        // Prototype.".
        if let Some(class) = self.the_class_of_the_prototype(target) {
            if self.the_class_has_the_method(&class, name) {
                // A method of a class of the script runs on the prototype:
                // v2 calls it, and `this` is the prototype.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `D.Prototype.Say()` answers 1, and the class of the
                // value that it answers is the class of the prototype.
                if self.classes.contains_key(&class) {
                    if let Some((defining, method)) = self.find_the_instance_method_in(&class, name) {
                        return self.call_method_with(&defining, &method, target, &values, line);
                    }
                }
                let spelled = self.the_spelling_of_the_method_of_a_class(&class, name);
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: expected_the_class(
                            &self.the_spelling_of_the_class(&class),
                            "Prototype",
                        ),
                        line,
                    },
                    Detail {
                        what: self.the_name_of_a_member(&class, &spelled),
                        extra: String::new(),
                    },
                ));
            }
        }
        // Every object has the methods of the base object.
        if let Some(result) = self.base_method(target, name, &values, line) {
            return result;
        }
        // `__Call` is asked when the method is not there: v2 asks it with
        // the name of the method and the parameters of the call.
        if let Some((defining, method)) = self.find_method_in(&class, "__call") {
            let params = self.the_params(&values, line)?;
            return self.call_method_with(
                &defining,
                &method,
                target,
                &[Value::Text(name.to_string()), params],
                line,
            );
        }
        Err(self.no_method(target, name, line))
    }

    /// A method, called on a value.
    fn call_method(
        &mut self,
        class: &str,
        method: &MethodDef,
        this: &Value,
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let mut values: Vec<Value> = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        self.call_method_with(class, method, this, &values, line)
    }

    /// A method, called with the values of the arguments.
    ///
    /// The number of the arguments is checked the way v2 checks it: a
    /// parameter that has no default is required, and the call cannot
    /// give more arguments than the method has parameters.
    fn call_method_with(
        &mut self,
        class: &str,
        method: &MethodDef,
        this: &Value,
        values: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // The rest of the arguments may be any number of them, and a
        // method that has one has no maximum.
        let variadic = method.parameters.iter().any(|parameter| parameter.variadic);
        if !variadic && values.len() > method.parameters.len() {
            let extra = self.the_name_of_the_method(class, method);
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Too many parameters passed to function.".into(),
                    line,
                },
                Detail { what: String::new(), extra },
            ));
        }
        // A parameter that has no default is required: v2 refuses the
        // call and says that the parameter is missing.
        for (index, parameter) in method.parameters.iter().enumerate() {
            if values.get(index).is_none() && parameter.default.is_none() && !parameter.variadic {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Missing a required parameter.".into(),
                        line,
                    },
                    Detail { what: String::new(), extra: parameter.name.clone() },
                ));
            }
        }
        let mut scope: BTreeMap<String, Value> = BTreeMap::new();
        // The value the method was called on is `this`.
        scope.insert("this".to_string(), this.clone());
        // The parameters that the call does not give a value to take
        // their default, and the ones that have none are empty.
        for (index, parameter) in method.parameters.iter().enumerate() {
            let given = values.get(index);
            let value = match given {
                Some(value) => value.clone(),
                None => match &parameter.default {
                    Some(default) => self.evaluate(default)?,
                    None => Value::Text(String::new()),
                },
            };
            // A parameter that the call does not give a value to is a local
            // one, even when it is written `&x`: the default of v2 is a
            // value of its own.
            if parameter.by_ref && given.is_some() {
                // The caller must pass a reference: the parameter is an
                // alias of the variable the reference points at.
                let Value::Reference(reference) = value else {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: wants_a_reference(
                            index + 1,
                            &method.name,
                            "a variable reference",
                            &value,
                        ),
                        line,
                    }));
                };
                let mut alias = reference;
                alias.alias = true;
                scope.insert(the_key_of(&parameter.name), Value::Reference(alias));
                continue;
            }
            scope.insert(the_key_of(&parameter.name), value);
        }
        // The parameter that takes the rest of the arguments takes them as
        // an array, the way v2 has it.
        if let Some(index) = method.parameters.iter().position(|parameter| parameter.variadic) {
            let rest: Vec<Value> = values.iter().skip(index).cloned().collect();
            let array = match crate::classes::construct("array", &rest, self, line) {
                Some(result) => result?,
                None => Value::List(rest),
            };
            scope.insert(the_key_of(&method.parameters[index].name), array);
        }
        let frame = self.the_name_of_the_method(class, method);
        self.enter_call(line)?;
        self.scopes.push(Rc::new(RefCell::new(scope)));
        // The functions that are written inside the method are there
        // before the body runs.
        self.collect_the_nested_functions(&method.body);
        self.declarations.push(method.declarations.clone());
        self.current_classes.push(class.to_lowercase());
        self.frames.push(Frame { name: frame, line });
        // The labels of the method are the scope of the method.
        self.label_scopes.push(the_labels_of(&method.body));
        let outcome = self.run_block(&method.body);
        self.label_scopes.pop();
        self.frames.pop();
        self.current_classes.pop();
        self.declarations.pop();
        self.scopes.pop();
        self.leave_call();
        match outcome {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(RuntimeFailure::Flow(Flow::Return(value))) => {
                Ok(value.unwrap_or(Value::Text(String::new())))
            }
            // The jump of a `Goto` that leaves the method: the label is not
            // in the scope of the method.
            Err(RuntimeFailure::Flow(Flow::Goto { name, line })) => Err(RuntimeFailure::Fatal(
                self.the_label_is_not_there(&name, &method.body, line),
            )),
            Err(other) => Err(other),
        }
    }

    /// The base of a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the base of an
    /// object is the prototype of its class, the base of a class is the
    /// class that it extends, and the base of a prototype is the
    /// prototype of the class that it extends.
    fn the_base_of_the_value(&self, target: &Value) -> Option<Value> {
        let Value::Object(object) = target else { return None };
        let class = object.borrow().class.clone();
        // The prototype of a class: its base is the prototype of the
        // class that the class extends.
        if class == "prototype" {
            let name = object
                .borrow()
                .value_of(CLASS_NAME)
                .map(|value| value.to_text().to_lowercase())?;
            let base = crate::classes::the_base_class_of_a_class(&name)?;
            return self.the_prototype_of(base);
        }
        // A class is not an instance of itself: the base of a class is
        // the class that it extends. A base that the script wrote is the
        // base of the class, and it is an object and not a class.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31.
        if let Some(name) = self.class_name_of(target) {
            if let Some(base) = self.stored_base_of(target) {
                return Some(base);
            }
            return self.the_base_of_the_class(&name);
        }
        self.base_of(target)
    }

    /// The base of a class: the class that it extends.
    ///
    /// A class of the script extends the class it names, and the base
    /// object when it names none. A class of the runtime has the bases of
    /// v2: the errors of v2 are a tree, and the `Any` is the root of it.
    fn the_base_of_the_class(&self, name: &str) -> Option<Value> {
        let base = self.the_base_of_the_class_name(name)?;
        self.class_value_of(&base)
    }

    /// The name of the class that a class extends.
    fn the_base_of_the_class_name(&self, name: &str) -> Option<String> {
        let name = name.to_lowercase();
        if let Some(definition) = self.classes.get(&name) {
            let base = definition.extends.clone().unwrap_or_else(|| "object".to_string());
            return Some(base.to_lowercase());
        }
        crate::classes::the_base_class_of_a_class(&name).map(|base| base.to_string())
    }

    /// The chain of the bases of a class.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the chain of a
    /// class is the chain of the classes that it extends, and it begins at
    /// the base of the class: a class is not a base of itself. A base that
    /// the script wrote replaces the chain.
    fn the_chain_of_the_class_bases(&self, target: &Value) -> Vec<Value> {
        let mut chain: Vec<Value> = Vec::new();
        let Some(name) = self.class_name_of(target) else { return chain };
        if self.stored_base_of(target).is_some() {
            return chain;
        }
        let mut current = self.the_base_of_the_class_name(&name);
        while let Some(class) = current {
            let Some(value) = self.class_value_of(&class) else { break };
            let Value::Object(object) = &value else { break };
            // A class that extends itself would loop for ever.
            if chain.iter().any(|seen| match seen {
                Value::Object(other) => Rc::ptr_eq(other, object),
                _ => false,
            }) {
                break;
            }
            chain.push(value);
            current = self.the_base_of_the_class_name(&class);
        }
        chain
    }

    /// The prototype of a class.
    fn the_prototype_of(&self, class: &str) -> Option<Value> {
        let Value::Object(value) = self.class_value_of(class)? else { return None };
        value.borrow().value_of("prototype")
    }

    /// The static method of a class, as a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: every class has
    /// the methods of the base object as static ones -- `HasProp`,
    /// `HasMethod`, `GetMethod`, `HasBase`, `OwnProps`, `HasOwnProp`,
    /// `DefineProp`, `DeleteProp`, `GetOwnPropDesc` and `Clone` -- and
    /// the call of the class is one of them.
    fn the_static_method(
        &self,
        target: &Value,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Some(name) = arguments.first().map(|value| value.to_text()) else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "GetMethod needs a name.".to_string(),
                line,
            }));
        };
        let lower = name.to_lowercase();
        // The prototype of a class is a property of it, and the `GetMethod`
        // of it fails with the words of the prototype.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: "This value of
        // type \"Prototype\" has no method named \"Call\"."
        if lower == "prototype" {
            return Err(crate::classes::named(
                RuntimeFailure::Error(RuntimeError {
                    message: "This value of type \"Prototype\" has no method named \"Call\".".to_string(),
                    line,
                }),
                "GetMethod",
            ));
        }
        // A static method that the class declares is its own, and the
        // `GetMethod` of a class hands it over.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31:
        // `C.GetMethod("Say")` is a function whose name is "C.Say".
        if let Some(class) = self.class_name_of(target) {
            if let Some((defining, method)) = self.find_the_static_method_in(&class, &lower) {
                return Ok(self.static_method_reference(&defining, &method));
            }
        }
        if !crate::classes::the_static_methods_of_a_class().contains(&lower.as_str()) {
            // The class does not have the method: the words of the
            // dialect say so, and the name of the method goes in the
            // `What`.
            return Err(crate::classes::named(self.no_method(target, &name, line), "GetMethod"));
        }
        // The name of the method is the way v2 spells it, and the object
        // is one of the arguments of the call.
        let spelled = if lower == "call" {
            let class = self.class_name_of(target).unwrap_or_default();
            format!("{}.Call", crate::classes::the_spelling_of(&class))
        } else {
            crate::classes::the_name_of_a_method_of_the_base_object(&lower)
        };
        let (min, max) = crate::classes::the_arity_of("object", &lower).unwrap_or((0, 0));
        Ok(self.method_reference_of_the_base_object(&spelled, &lower, min + 1, max + 1))
    }

    /// The static methods of a class value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a class has the
    /// methods of the base object, the call of the class, and the static
    /// methods that it declares. The methods of its instances are not its
    /// own.
    fn the_static_methods_of_a_class_value(&self, target: &Value) -> Vec<String> {
        let mut names: Vec<String> = crate::classes::the_static_methods_of_a_class()
            .iter()
            .map(|name| name.to_string())
            .collect();
        for name in self.the_static_methods_declared_by_a_class(target) {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    }

    /// The static methods that a class declares.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: they are the own
    /// properties of the class value, the way the call of the class is.
    fn the_static_methods_declared_by_a_class(&self, target: &Value) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        let Some(class) = self.class_name_of(target) else { return names };
        for class in self.class_chain(&class) {
            let Some(definition) = self.classes.get(&class) else { continue };
            for (name, method) in &definition.methods {
                if method.is_static && !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
        names
    }

    /// The static method of a class value, as a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a class has the
    /// methods of the base object and the static methods that it declares,
    /// and v2 hands them over as values: `Map.Call` is a function whose
    /// name is "Map.Call", and `C.Say` is one whose name is "C.Say".
    fn the_static_method_of_a_class_value(
        &mut self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        let lower = name.to_lowercase();
        if !self.the_static_methods_of_a_class_value(target).contains(&lower) {
            return Ok(None);
        }
        // The static methods that the class of the script declares.
        if let Some(class) = self.class_name_of(target) {
            if let Some((defining, method)) = self.find_the_static_method_in(&class, &lower) {
                return Ok(Some(self.static_method_reference(&defining, &method)));
            }
        }
        // The methods of the base object.
        let values = [Value::Text(name.to_string())];
        Ok(Some(self.the_static_method(target, &values, line)?))
    }

    /// The class that a value is the prototype of.
    ///
    /// The prototype of a class holds the members of the class, and the name
    /// of the class is under the key of it.
    fn the_class_of_the_prototype(&self, target: &Value) -> Option<String> {
        let Value::Object(object) = target else { return None };
        if object.borrow().class != "prototype" {
            return None;
        }
        object
            .borrow()
            .value_of(CLASS_NAME)
            .map(|name| name.to_text().to_lowercase())
    }

    /// The name of a class, the way v2 spells it.
    ///
    /// The class of the runtime has a spelling of its own, and the class of
    /// the script is spelled the way the script wrote it.
    fn the_spelling_of_the_class(&self, class: &str) -> String {
        self.classes
            .get(class)
            .map(|definition| definition.name.clone())
            .unwrap_or_else(|| crate::classes::the_spelling_of(class))
    }

    /// The name of a member of a class, the way v2 names it in a failure.
    ///
    /// The name of the class, the name of the prototype, and the name of the
    /// member.
    fn the_name_of_a_member(&self, class: &str, member: &str) -> String {
        format!("{}.Prototype.{}", self.the_spelling_of_the_class(class), member)
    }

    /// The name of a method of a class, the way v2 spells it.
    fn the_spelling_of_the_method_of_a_class(&self, class: &str, name: &str) -> String {
        let lower = name.to_lowercase();
        if let Some(definition) = self.classes.get(class) {
            if let Some(method) = definition.methods.get(&lower) {
                return method.name.clone();
            }
        }
        crate::classes::the_method_of(class, &lower)
            .map(|(spelled, _, _)| spelled.to_string())
            .unwrap_or_else(|| name.to_string())
    }

    /// Whether the class has a method of that name.
    ///
    /// The methods of the base object are the ones every object has, and the
    /// prototype of a class answers for them.
    fn the_class_has_the_method(&self, class: &str, name: &str) -> bool {
        let lower = name.to_lowercase();
        if crate::classes::the_base_object_has_the_method(&lower) {
            return false;
        }
        if self.classes.contains_key(class) {
            // A static method is not a method of the prototype.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31:
            // `C.Prototype.Say()` is refused with 'This value of type
            // "Prototype" has no method named "Say".' when `Say` is a
            // static method of the class.
            return self.find_the_instance_method_in(class, &lower).is_some();
        }
        crate::classes::the_methods_of_the_prototype(class)
            .iter()
            .any(|method| method.to_lowercase() == lower)
    }

    /// Whether the prototype of a class has a member of that name.
    ///
    /// The members of a class of the runtime are in the table of the
    /// prototype, and the class of the script declares the methods of it.
    fn the_prototype_has_the_member(&self, class: &str, name: &str) -> bool {
        let lower = name.to_lowercase();
        if let Some(definition) = self.classes.get(class) {
            return definition
                .methods
                .values()
                .any(|method| !method.is_static && method.name.to_lowercase() == lower)
                || definition
                    .accessors
                    .values()
                    .any(|access| !access.is_static && access.name.to_lowercase() == lower);
        }
        crate::classes::the_members_of_the_prototype(class)
            .iter()
            .any(|member| member.to_lowercase() == lower)
    }

    /// The own properties of a prototype.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the prototype of a
    /// class owns the class of the value and the members that the class
    /// declares -- the methods of it and the properties of it. The prototype
    /// of the base object owns the six members of the base object, and the
    /// prototype of a class of the script owns the methods that the class
    /// declares itself.
    fn the_own_properties_of_a_prototype(&self, target: &Value) -> Vec<String> {
        let mut names = vec!["__Class".to_string()];
        let Some(class) = self.the_class_of_the_prototype(target) else { return names };
        match self.classes.get(&class) {
            // A class of the script: the methods that it declares itself.
            // The static ones belong to the class and not to its prototype.
            Some(definition) => {
                for method in definition.methods.values() {
                    if !method.is_static {
                        names.push(method.name.clone());
                    }
                }
                for access in definition.accessors.values() {
                    if !access.is_static {
                        names.push(access.name.clone());
                    }
                }
                // The automatic `__Init` is own of the prototype when the
                // class has variables of its own: it is the one that
                // carries them.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // prototype of a class with a property owns "__Init".
                if !definition.methods.contains_key("__init")
                    && definition.properties.iter().any(|property| !property.is_static)
                {
                    names.push("__Init".to_string());
                }
            }
            // A class of the runtime: the members of the class type.
            None => {
                for name in crate::classes::the_members_of_the_prototype(&class) {
                    names.push((*name).to_string());
                }
            }
        }
        // The names are in the order of v2, and the case of them does not
        // count.
        names.sort_by_key(|name| name.to_lowercase());
        names.dedup();
        names
    }

    /// The value of an own property of a prototype.
    ///
    /// The class of the value is its own property, and a method of the class
    /// is handed over as a function. A property of the class cannot be read
    /// on the prototype of it, and the value of it is empty.
    fn the_value_of_an_own_property_of_a_prototype(
        &self,
        target: &Value,
        name: &str,
    ) -> Value {
        let lower = name.to_lowercase();
        if lower == "__class" {
            return Value::Text(self.the_class_of_the_value(target));
        }
        let Some(class) = self.the_class_of_the_prototype(target) else {
            return Value::Text(String::new());
        };
        // A method of a class of the script.
        if let Some((defining, method)) = self.find_method_in(&class, &lower) {
            return self.method_reference(&defining, &method);
        }
        // A method of a class of the runtime.
        if let Some((spelled, min, max)) = crate::classes::the_method_of(&class, &lower) {
            return self.method_reference_of_a_class_type(&class, spelled, min, max);
        }
        // A method of the base object.
        if crate::classes::the_base_object_has_the_method(&lower) {
            let spelled = crate::classes::the_name_of_a_method_of_the_base_object(&lower);
            let (min, max) = crate::classes::the_arity_of("object", &lower).unwrap_or((0, 0));
            return self.method_reference_of_the_base_object(&spelled, &lower, min + 1, max + 1);
        }
        Value::Text(String::new())
    }

    /// The own properties of a class value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the class value of
    /// a class of the runtime owns the call of the class and the prototype,
    /// and the class value of a class of the script owns the initialiser of
    /// the class, the prototype and the static methods that the class
    /// declares.
    fn the_own_properties_of_a_class_value(&self, target: &Value) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        let Some(class) = self.class_name_of(target) else { return names };
        match self.classes.get(&class) {
            Some(definition) => {
                // A class of the script: the initialiser of it, the static
                // methods that it declares, and the classes that it holds.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the own
                // properties of `Outer` are "__Init", "Inner" and
                // "Prototype".
                names.push("__Init".to_string());
                for method in definition.methods.values() {
                    if method.is_static {
                        names.push(method.name.clone());
                    }
                }
                for access in definition.accessors.values() {
                    if access.is_static {
                        names.push(access.name.clone());
                    }
                }
                for nested in &definition.nested {
                    names.push(self.the_name_of_a_class(nested));
                }
            }
            // A class of the runtime: the call of it, when the class is one
            // that a script calls.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: the class
            // value of the `Map` owns the call and the prototype, and the
            // class value of the `Error` owns the prototype alone.
            None => {
                if crate::classes::the_class_owns_the_call(&class) {
                    names.push("Call".to_string());
                }
            }
        }
        names.push("Prototype".to_string());
        names
    }

    /// The value of an own property of a class value.
    fn the_value_of_an_own_property_of_a_class_value(
        &self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> Value {
        let lower = name.to_lowercase();
        match lower.as_str() {
            "call" => self
                .the_static_method(target, &[Value::Text(name.to_string())], line)
                .unwrap_or_else(|_| Value::Text(String::new())),
            "__init" => self.the_init_of_a_class_value(target),
            "prototype" => match target {
                Value::Object(object) => object
                    .borrow()
                    .value_of("prototype")
                    .unwrap_or_else(|| Value::Text(String::new())),
                _ => Value::Text(String::new()),
            },
            // A static method that the class declares, or a class that the
            // class holds.
            _ => {
                let Some(class) = self.class_name_of(target) else {
                    return Value::Text(String::new());
                };
                if let Some(value) = self.the_class_that_a_class_holds(target, name) {
                    return value;
                }
                match self.find_the_static_method_in(&class, &lower) {
                    Some((defining, method)) => self.static_method_reference(&defining, &method),
                    None => Value::Text(String::new()),
                }
            }
        }
    }

    /// The initialiser of a class, as a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: every class has an
    /// automatic initialiser, and it is a member of the class value. The
    /// name of it is "C.__Init" for a class of the script, and it is empty
    /// for a class of the runtime -- and the initialiser of a class of the
    /// runtime is a built-in, and the one of a class of the script is not.
    fn the_init_of_a_class_value(&self, target: &Value) -> Value {
        let mut properties: BTreeMap<String, Property> = BTreeMap::new();
        let mut name = String::new();
        let mut built_in = 1;
        if let Some(class) = self.class_name_of(target) {
            if let Some(definition) = self.classes.get(&class) {
                name = format!("{}.__Init", definition.name);
                built_in = 0;
            }
        }
        put(&mut properties, "name", Value::Text(name));
        // The first parameter of the reference is `this`.
        put(&mut properties, "minparams", Value::Int(1));
        put(&mut properties, "maxparams", Value::Int(1));
        put(&mut properties, "isbuiltin", Value::Int(built_in));
        put(&mut properties, "isvariadic", Value::Int(0));
        Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
    }

    /// The value of a static method of a class of the script.
    ///
    /// A static method belongs to the class and not to its prototype, and
    /// the name of the value carries the class and the method.
    fn static_method_reference(&self, class: &str, method: &MethodDef) -> Value {
        let owner = self
            .classes
            .get(class)
            .map(|definition| definition.name.clone())
            .unwrap_or_else(|| class.to_string());
        let mut properties: BTreeMap<String, Property> = BTreeMap::new();
        put(&mut properties, "name", Value::Text(format!("{owner}.{}", method.name)));
        // The first parameter of the reference is `this`.
        let required = method
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        put(&mut properties, "minparams", Value::Int(required as i64 + 1));
        put(&mut properties, "maxparams", Value::Int(method.parameters.len() as i64 + 1));
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(&mut properties, METHOD_CLASS, Value::Text(class.to_string()));
        put(&mut properties, METHOD_NAME, Value::Text(method.name.clone()));
        Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
    }

    /// The reference of a static method of a class.
    ///
    /// The reference is the method, and the object is one of the
    /// arguments of the call.
    fn method_reference_of_the_base_object(&self, spelled: &str, method: &str, min: i64, max: i64) -> Value {
        let mut properties = BTreeMap::new();
        put(&mut properties, "name", Value::Text(spelled.to_string()));
        put(&mut properties, "minparams", Value::Int(min));
        put(&mut properties, "maxparams", Value::Int(max));
        put(&mut properties, "isbuiltin", Value::Int(1));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(&mut properties, METHOD_CLASS, Value::Text("object".to_string()));
        put(&mut properties, METHOD_NAME, Value::Text(method.to_string()));
        Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
    }

    /// The copy of an object: the state of it is copied, and the class of
    /// it is the same.
    fn clone_of_an_object(&self, target: &Value) -> Value {
        let Value::Object(object) = target else { return target.clone() };
        let class = object.borrow().class.clone();
        let mut properties = BTreeMap::new();
        for (key, property) in object.borrow().properties.iter() {
            properties.insert(key.clone(), property.clone());
        }
        Value::Object(Rc::new(RefCell::new(Object { class, properties })))
    }

    /// The copy of a class: a class of its own, and not an object of it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the copy of a
    /// class is a class, and it is not the class it was copied from.
    fn clone_of_a_class(&self, target: &Value) -> Value {
        let Value::Object(object) = target else { return target.clone() };
        let class = object.borrow().class.clone();
        let mut properties = BTreeMap::new();
        for (key, property) in object.borrow().properties.iter() {
            properties.insert(key.clone(), property.clone());
        }
        let key = format!("{} copy {}", class, self.class_values.borrow().len());
        let copy = Value::Object(Rc::new(RefCell::new(Object { class, properties })));
        // The copy is a class too, and the tree keeps it in the list of
        // the classes it made: that is what tells a class from an
        // instance of one.
        self.class_values.borrow_mut().insert(key, copy.clone());
        copy
    }

    /// A method every object has.
    ///
    /// The methods of the base object are the same for every class: they
    /// ask about the object itself. The class of the script inherits them
    /// without writing them.
    pub(crate) fn base_method(
        &mut self,
        target: &Value,
        name: &str,
        arguments: &[Value],
        line: u32,
    ) -> Option<Result<Value, RuntimeFailure>> {
        let answer = self.the_base_method(target, name, arguments, line);
        // Measured on the binary of v2.0.19 on 2026-10-31: the failure of
        // a method of the base object carries the name of the method in
        // the `What`, the way v2 does.
        answer.map(|result| {
            result.map_err(|failure| {
                crate::classes::named(
                    failure,
                    &crate::classes::the_name_of_a_method_of_the_base_object(name),
                )
            })
        })
    }

    /// A method every object has, and the name of it is not in the
    /// failure.
    fn the_base_method(
        &mut self,
        target: &Value,
        name: &str,
        arguments: &[Value],
        line: u32,
    ) -> Option<Result<Value, RuntimeFailure>> {
        let Value::Object(object) = target else { return None };
        let class = object.borrow().class.clone();
        let method = name.to_string();
        let missing = || RuntimeFailure::Error(RuntimeError {
            message: format!("{method} needs a name."),
            line,
        });
        match name.to_lowercase().as_str() {
            "clone" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // copy of an object is an object of the same class, and
                // the state of it is copied. The copy of the base object
                // refuses an object of a class of the runtime that has a
                // copy of its own: the type of the argument is not the
                // one that is wanted.
                //
                // The copy of a prototype is refused the same way: the
                // prototype is not an object that can be copied.
                if class == "prototype" {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: "Type mismatch.".to_string(),
                        line,
                    })));
                }
                if crate::classes::the_method_of(&class, "clone").is_some() {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: "Type mismatch.".to_string(),
                        line,
                    })));
                }
                Some(Ok(self.clone_of_an_object(target)))
            }
            "call" => {
                // The call of a class value makes an object of the class:
                // the static method of a class is the call of it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31.
                let class = self.class_name_of(target)?;
                Some(self.construct_with_values(&class, arguments, line))
            }
            "hasprop" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                let found = object.borrow().properties.contains_key(&name.to_lowercase())
                    || self.declares_an_instance_property(&class, &name)
                    // Every value carries its class, and the base object
                    // has the property of it.
                    || name.eq_ignore_ascii_case("__class")
                    // Every object has a base.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `obj.HasProp("Base")` answers 1.
                    || name.eq_ignore_ascii_case("base")
                    // The methods of the class are the properties of the
                    // object, and a static method is not one of them.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `obj.HasProp("Say")` answers 1 when `Say` is a method
                    // of the class, and 0 when it is a static one.
                    || self.find_the_instance_method_in(&class, &name).is_some()
                    || crate::classes::the_method_of(&class, &name).is_some()
                    // The properties that have a getter are the properties
                    // of the object.
                    || self.the_accessor_of(target, &name).is_some()
                    // Every class has an initialiser, and it is a member of
                    // the class.
                    || name.eq_ignore_ascii_case("__init")
                    || crate::classes::the_base_object_has_the_method(&name);
                // The prototype of a class has the properties of the class.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `Map.Prototype.HasProp("Count")` answers 1.
                let found = if let Some(class) = self.the_class_of_the_prototype(target) {
                    // The prototype of a class has the methods of the class
                    // and the ones of its base, and not the properties of
                    // the class.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `Map.Prototype.HasProp("Count")` answers 1, and
                    // `C.Prototype.HasProp("value")` answers 0 when `value`
                    // is a variable of the class.
                    crate::classes::the_members_of_the_prototype(&class)
                        .iter()
                        .any(|member| member.to_lowercase() == name.to_lowercase())
                        || self.find_the_instance_method_in(&class, &name).is_some()
                        // The properties that have a getter are the members
                        // of the prototype.
                        || self.the_accessor_of(target, &name).is_some()
                        || name.eq_ignore_ascii_case("__init")
                        || name.eq_ignore_ascii_case("__class")
                } else if self.class_name_of(target).is_some() {
                    // A class value has the prototype, the base and the
                    // static methods, and every value carries its class.
                    // The properties of the class are not its own.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `C.HasProp("__Init")` answers 1, and the initialiser
                    // is an own property of the class value.
                    self.the_own_properties_of_a_class_value(target)
                        .iter()
                        .any(|own| own.to_lowercase() == name.to_lowercase())
                        || object.borrow().properties.contains_key(&name.to_lowercase())
                        || name.eq_ignore_ascii_case("base")
                        || self
                            .the_static_methods_of_a_class_value(target)
                            .iter()
                            .any(|method| *method == name.to_lowercase())
                        || name.eq_ignore_ascii_case("__class")
                } else {
                    found
                };
                Some(Ok(Value::Int(found as i64)))
            }
            "hasmethod" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // question with no name asks whether the value is a
                    // class, and a class has the methods of the base
                    // object.
                    return Some(Ok(Value::Int(self.class_name_of(target).is_some() as i64)));
                };
                // The methods of the base object are the methods of every
                // object, and the methods of a class of the runtime are the
                // methods of its instances. A class has the methods of the
                // base object, and not the methods of the class it is.
                // The prototype of a class carries the methods of the class
                // it is the prototype of.
                let class = if class == "prototype" {
                    object
                        .borrow()
                        .value_of(CLASS_NAME)
                        .map(|name| name.to_text().to_lowercase())
                        .unwrap_or(class)
                } else {
                    class
                };
                let found = if self.class_name_of(target).is_some() {
                    // A class has the static methods, and the methods of its
                    // instances are not its own.
                    self.the_static_methods_of_a_class_value(target)
                        .contains(&name.to_lowercase())
                } else if let Some(class) = self.the_class_of_the_prototype(target) {
                    // The prototype of a class has the methods of the class
                    // and the ones of its base, and not the static ones.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `C.Prototype.HasMethod("Say")` answers 1 when `Say` is
                    // a method of the class, and 0 when it is a static one.
                    self.find_the_instance_method_in(&class, &name).is_some()
                        || crate::classes::the_methods_of_the_prototype(&class)
                            .iter()
                            .any(|method| method.to_lowercase() == name.to_lowercase())
                        || crate::classes::the_base_object_has_the_method(&name)
                } else {
                    // A static method is not a member of an instance.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `obj.HasMethod("Say")` answers 0 when `Say` is a
                    // static method of the class of the object.
                    self.find_the_instance_method_in(&class, &name).is_some()
                        || crate::classes::the_method_of(&class, &name).is_some()
                        || crate::classes::the_base_object_has_the_method(&name)
                };
                // Every class has an initialiser, and it is a method of the
                // class and of its instances.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasMethod("__Init")` answers 1, and so does
                // `C.HasMethod("__Init")`.
                let found = found || name.eq_ignore_ascii_case("__init");
                Some(Ok(Value::Int(found as i64)))
            }
            "getmethod" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                // The prototype of a class holds the methods of the class.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `Map.Prototype.GetMethod("Get")` is the method of the `Map`,
                // and `Object.Prototype.GetMethod("Clone")` is the method of
                // the base object.
                if let Some(class) = self.the_class_of_the_prototype(target) {
                    // A method of a class of the script. The static ones are
                    // not the methods of the prototype.
                    //
                    // Measured on the binary of v2.0.19 on 2026-10-31:
                    // `C.Prototype.GetMethod("Say")` is refused when `Say` is
                    // a static method of the class.
                    if let Some((defining, method)) = self.find_the_instance_method_in(&class, &name) {
                        return Some(Ok(self.method_reference(&defining, &method)));
                    }
                    // A method of a class type of the runtime.
                    if let Some((spelled, min, max)) = crate::classes::the_method_of(&class, &name) {
                        let reference =
                            self.method_reference_of_a_class_type(&class, spelled, min, max);
                        return Some(Ok(reference));
                    }
                    // A method of the base object.
                    if crate::classes::the_base_object_has_the_method(&name) {
                        let spelled = crate::classes::the_name_of_a_method_of_the_base_object(&name);
                        let (min, max) = crate::classes::the_arity_of("object", &name).unwrap_or((0, 0));
                        let reference = self
                            .method_reference_of_the_base_object(&spelled, &name, min + 1, max + 1);
                        return Some(Ok(reference));
                    }
                    return Some(Err(self.no_method(target, &name, line)));
                }
                // A method of a class of the script. A static method is
                // not a member of an instance.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `obj.GetMethod("Say")` is refused when `Say` is a static
                // method of the class.
                if let Some((defining, method)) = self.find_the_instance_method_in(&class, &name) {
                    return Some(Ok(self.method_reference(&defining, &method)));
                }
                // A method of a class type of the runtime.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.GetMethod("Get")` is the method of the `Map`.
                if let Some((spelled, min, max)) = crate::classes::the_method_of(&class, &name) {
                    let reference =
                        self.method_reference_of_a_class_type(&class, spelled, min, max);
                    return Some(Ok(reference));
                }
                // A method of the base object.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `o.GetMethod("Clone")` is the method of the base object.
                if crate::classes::the_base_object_has_the_method(&name) {
                    let spelled = crate::classes::the_name_of_a_method_of_the_base_object(&name);
                    let (min, max) = crate::classes::the_arity_of("object", &name).unwrap_or((0, 0));
                    let reference =
                        self.method_reference_of_the_base_object(&spelled, &name, min + 1, max + 1);
                    return Some(Ok(reference));
                }
                Some(Err(self.no_method(target, &name, line)))
            }
            "hasbase" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the base
                // is an object, and the question is whether the base of the
                // object is it.
                let Some(base) = arguments.first() else {
                    return Some(Err(missing()));
                };
                let Value::Object(base) = base else {
                    return Some(Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("Expected an object but got {}.", the_kind_of(base)),
                        line,
                    })));
                };
                // A class value is a class and not an instance of one: the
                // chain of a class is the chain of the classes that it
                // extends.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31.
                if self.class_name_of(target).is_some() {
                    let chain = self.the_chain_of_the_class_bases(target);
                    return Some(Ok(Value::Int(
                        chain.iter().any(|object| match object {
                            Value::Object(one) => Rc::ptr_eq(one, base),
                            _ => false,
                        }) as i64,
                    )));
                }
                let chain = self.the_chain_of_the_bases(target);
                Some(Ok(Value::Int(
                    chain.iter().any(|object| Rc::ptr_eq(object, base)) as i64,
                )))
            }
            "hasownprop" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                let lower = name.to_lowercase();
                // The prototype of a class owns the members of the class.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // prototype of a `Map` owns the class of the value and the
                // twelve members of the class.
                if let Some(class) = self.the_class_of_the_prototype(target) {
                    // The prototype of an `Any`, of a `ComObject` and of a
                    // `ComValue` is not a prototype: it has no method at all.
                    if crate::classes::the_prototype_is_not_one(&class) {
                        return Some(Err(RuntimeFailure::Detailed(
                            RuntimeError {
                                message: format!(
                                    "This value of type \"{}\" has no method named \"HasOwnProp\".",
                                    self.the_type_of(target)
                                ),
                                line,
                            },
                            Detail { what: String::new(), extra: String::new() },
                        )));
                    }
                    let own = self
                        .the_own_properties_of_a_prototype(target)
                        .iter()
                        .any(|name| name.to_lowercase() == lower);
                    return Some(Ok(Value::Int(own as i64)));
                }
                // The members of a function are not own properties of it:
                // they live on the prototype of the class of it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the own
                // properties of a function are none.
                let own = if object.borrow().class == "func" {
                    false
                } else if self.class_name_of(target).is_some() {
                    // The table of a class value is the tree's own, and the
                    // name of the class and the base and the prototype are
                    // not properties of it. The names that the script wrote
                    // are its own properties.
                    object.borrow().properties.contains_key(&lower)
                        && lower != CLASS_NAME
                        && lower != BASE
                        && lower != "prototype"
                } else {
                    object.borrow().properties.contains_key(&lower)
                };
                // The call of a class is its own property, and so are the
                // static methods that the class declares. The methods of
                // the base object are inherited, and not own.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31.
                // A class value owns the members that v2 gives it, and the
                // methods of the base object are inherited, and not own.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31.
                let own = own
                    || (self.class_name_of(target).is_some()
                        && self
                            .the_own_properties_of_a_class_value(target)
                            .iter()
                            .any(|name| name.to_lowercase() == lower));
                Some(Ok(Value::Int(own as i64)))
            }
            "ownprops" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the own
                // properties of v2 are an enumerator, and it gives the name
                // when the caller wants one value. The value of a property
                // that is a descriptor is the value of the property, and
                // not the descriptor.
                // The prototype of a class owns the members of the class.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // prototype of a `Map` owns the class of the value and the
                // twelve members of the class.
                if let Some(class) = self.the_class_of_the_prototype(target) {
                    // The prototype of an `Any`, of a `ComObject` and of a
                    // `ComValue` is not a prototype: it has no method at all.
                    if crate::classes::the_prototype_is_not_one(&class) {
                        return Some(Err(RuntimeFailure::Detailed(
                            RuntimeError {
                                message: format!(
                                    "This value of type \"{}\" has no method named \"OwnProps\".",
                                    self.the_type_of(target)
                                ),
                                line,
                            },
                            Detail { what: String::new(), extra: String::new() },
                        )));
                    }
                    let mut pairs: Vec<(Value, Value)> = Vec::new();
                    for name in self.the_own_properties_of_a_prototype(target) {
                        let value =
                            self.the_value_of_an_own_property_of_a_prototype(target, &name);
                        pairs.push((Value::Text(name), value));
                    }
                    return Some(Ok(crate::classes::enumerator::make(
                        pairs,
                        crate::classes::enumerator::Alone::TheKey,
                    )));
                }
                // A class value owns the members that v2 gives it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the class
                // value of a class of the runtime owns the call of the class
                // and the prototype, and the class value of a class of the
                // script owns the initialiser of the class, the prototype
                // and the static methods that the class declares.
                if self.class_name_of(target).is_some() {
                    let mut pairs: Vec<(Value, Value)> = Vec::new();
                    for name in self.the_own_properties_of_a_class_value(target) {
                        let value =
                            self.the_value_of_an_own_property_of_a_class_value(target, &name, line);
                        pairs.push((Value::Text(name), value));
                    }
                    // The properties that the script wrote are own as well.
                    for (name, property) in object.borrow().properties.iter() {
                        if name.as_str() == BASE
                            || name.as_str() == CLASS_NAME
                            || name.as_str() == "prototype"
                        {
                            continue;
                        }
                        pairs.push((Value::Text(property.name.clone()), property.value.clone()));
                    }
                    // The names are in the order of v2, and the case of them
                    // does not count.
                    pairs.sort_by(|left, right| {
                        left.0.to_text()
                            .to_lowercase()
                            .cmp(&right.0.to_text().to_lowercase())
                    });
                    return Some(Ok(crate::classes::enumerator::make(
                        pairs,
                        crate::classes::enumerator::Alone::TheKey,
                    )));
                }
                // The members of a function are not own properties of it:
                // they live on the prototype of the class of it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the own
                // properties of a function are none.
                if object.borrow().class == "func" {
                    return Some(Ok(crate::classes::enumerator::make(
                        Vec::new(),
                        crate::classes::enumerator::Alone::TheKey,
                    )));
                }
                // The names are the ones that the script wrote, and they
                // are in the order of v2: the case of them does not count.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: `o.X := 1`
                // gives the own property the name "X", and the first write
                // is the one that fixes the case of it.
                let mut own: Vec<(String, Value)> = object
                    .borrow()
                    .properties
                    .iter()
                    .filter(|(name, _)| name.as_str() != BASE && name.as_str() != CLASS_NAME)
                    .map(|(_, property)| (property.name.clone(), property.value.clone()))
                    .collect();
                own.sort_by_key(|(name, _)| name.to_lowercase());
                let mut pairs: Vec<(Value, Value)> = Vec::with_capacity(own.len());
                for (name, value) in own {
                    let value = match self.the_value_of_the_descriptor(&value, target, &name, line) {
                        Ok(Some(value)) => value,
                        Ok(None) => value,
                        Err(_) => Value::Text(String::new()),
                    };
                    pairs.push((Value::Text(name), value));
                }
                Some(Ok(crate::classes::enumerator::make(
                    pairs,
                    crate::classes::enumerator::Alone::TheKey,
                )))
            }
            "defineprop" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // second parameter is a descriptor, and a value that is
                // not one is refused with the reason.
                let descriptor = match self.the_descriptor(arguments.get(1), line) {
                    Ok(descriptor) => descriptor,
                    Err(failure) => return Some(Err(failure)),
                };
                put(&mut object.borrow_mut().properties, &name, descriptor);
                // The answer of v2 is the object itself.
                Some(Ok(target.clone()))
            }
            "deleteprop" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                let gone = object.borrow_mut().properties.remove(&name.to_lowercase()).map(|property| property.value);
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // answer is the value of the property that is taken away,
                // and an empty text when there was none.
                Some(Ok(match gone {
                    None => Value::Text(String::new()),
                    Some(value) => match self.the_value_of_the_descriptor(&value, target, &name, line) {
                        Ok(Some(value)) => value,
                        _ => value,
                    },
                }))
            }
            "getownpropdesc" => {
                let Some(name) = arguments.first().map(|value| value.to_text()) else {
                    return Some(Err(missing()));
                };
                let key = name.to_lowercase();
                let value = object.borrow().value_of(&key);
                match value {
                    Some(value) => {
                        // Measured on the binary of v2.0.19 on 2026-10-31:
                        // the descriptor that comes back is an object with
                        // the members of it, and not a class of its own.
                        let table = match self.the_members_of_a_descriptor(&value) {
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
                    None => Some(Err(self.no_property(target, &name, line))),
                }
            }
            _ => None,
        }
    }

    /// The name of the type of a value, the way v2 writes it.
    ///
    /// An object is named after its class, and the other values after their
    /// kind: "Integer", "String", "Object".
    fn the_type_of(&self, value: &Value) -> String {
        // A class is a value of its own kind, and v2 names it "Class".
        if self.class_name_of(value).is_some() {
            return "Class".to_string();
        }
        match self.class_of(value) {
            Some(name) => name,
            None => value.the_kind(),
        }
    }

    /// The class of a value, the way v2 names it.
    ///
    /// Every value carries the class of it, and the `__Class` of the
    /// value is the name of that class: "Integer", "Float", "String",
    /// "Array", "VarRef", the name of the class of the script, and
    /// "Class" for a class itself. The prototype of a class is of the
    /// class it is the prototype of.
    fn the_class_of_the_value(&self, value: &Value) -> String {
        // A class is a value of its own kind, and v2 names it "Class".
        if self.class_name_of(value).is_some() {
            return "Class".to_string();
        }
        if let Value::Object(object) = value {
            if object.borrow().class == "prototype" {
                let name = object
                    .borrow()
                    .value_of(CLASS_NAME)
                    .map(|value| value.to_text())
                    .unwrap_or_default();
                return crate::classes::the_spelling_of(&name);
            }
        }
        match value {
            Value::Int(_) => "Integer".to_string(),
            Value::Float(_) => "Float".to_string(),
            Value::Text(_) => "String".to_string(),
            Value::List(_) => "Array".to_string(),
            Value::Reference(_) => "VarRef".to_string(),
            Value::Object(_) => self.the_type_of(value),
        }
    }

    /// The refusal of a property that a value does not have.
    pub(crate) fn no_property(&self, target: &Value, name: &str, line: u32) -> RuntimeFailure {
        let kind = self.the_type_of(target);
        // A class that this runtime does not carry is not refused with the
        // words of the dialect: the name is there, and the tree does not
        // carry it. A class value is a class of the dialect, and the
        // refusal of a member that is not there keeps the words of the
        // dialect.
        if self.class_name_of(target).is_none() {
            if let Some(reason) = crate::classes::the_reason_of_a_class_that_is_not_carried(&kind) {
                return RuntimeFailure::Error(RuntimeError { message: reason.to_string(), line });
            }
        }
        RuntimeFailure::Error(RuntimeError {
            message: format!("This value of type \"{kind}\" has no property named {name:?}."),
            line,
        })
    }

    /// The message of the refusal of a property that a value does not have.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: v2 reads the
    /// property "Ptr" of the value that it writes, and an object has none:
    /// 'This value of type "Map" has no property named "Ptr".'.
    pub(crate) fn the_message_of_no_property(&self, target: &Value, name: &str, line: u32) -> String {
        match self.no_property(target, name, line) {
            RuntimeFailure::Error(error) => error.message,
            RuntimeFailure::Detailed(error, _) => error.message,
            _ => format!(
                "This value of type \"{}\" has no property named {name:?}.",
                self.the_type_of(target)
            ),
        }
    }

    /// The descriptor of a property, out of the value that was given.
    ///
    /// v2 wants an object with one or more of the members `Value`, `Get`,
    /// `Set` and `Call`, and refuses anything else with "Parameter #2 of
    /// Object.Prototype.DefineProp is invalid.". A descriptor that carries
    /// a value and an accessor at the same time is refused as well.
    pub(crate) fn the_descriptor(&self, value: Option<&Value>, line: u32) -> Result<Value, RuntimeFailure> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the value that
        // is not a descriptor goes in the `Extra` of the failure.
        let invalid = || RuntimeFailure::Detailed(
            RuntimeError {
                message: "Parameter #2 of Object.Prototype.DefineProp is invalid.".into(),
                line,
            },
            Detail {
                what: String::new(),
                extra: value.map(|value| value.to_text()).unwrap_or_default(),
            },
        );
        let Some(Value::Object(object)) = value else { return Err(invalid()) };
        let table = object.borrow();
        let mut members: BTreeMap<String, Property> = BTreeMap::new();
        for key in ["value", "get", "set", "call"] {
            if let Some(member) = table.properties.get(key) {
                members.insert(key.to_string(), member.clone());
            }
        }
        // A value and an accessor at the same time is not a descriptor.
        if members.is_empty() || (members.contains_key("value") && members.len() > 1) {
            return Err(invalid());
        }
        Ok(Value::Object(Rc::new(RefCell::new(Object {
            class: DESCRIPTOR.to_string(),
            properties: members,
        }))))
    }

    /// Whether a value is the descriptor of a property.
    fn is_a_descriptor(&self, value: &Value) -> bool {
        let Value::Object(object) = value else { return false };
        object.borrow().class == DESCRIPTOR
    }

    /// The members of a descriptor, when the value is one.
    pub(crate) fn the_members_of_a_descriptor(&self, value: &Value) -> Option<BTreeMap<String, Property>> {
        if !self.is_a_descriptor(value) {
            return None;
        }
        let Value::Object(object) = value else { return None };
        Some(object.borrow().properties.clone())
    }

    /// The value of a property that is a descriptor.
    ///
    /// v2 asks the getter of the descriptor for the value, and the value of
    /// the descriptor is the value of the property when there is no getter.
    /// A descriptor with no getter and no value is not a property that can
    /// be read: v2 says that the property is not there.
    pub(crate) fn the_value_of_the_descriptor(
        &mut self,
        value: &Value,
        target: &Value,
        written: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        if !self.is_a_descriptor(value) {
            return Ok(None);
        }
        let Value::Object(object) = value else { return Ok(None) };
        let getter = object.borrow().value_of("get");
        let held = object.borrow().value_of("value");
        if let Some(getter) = getter {
            // The getter is asked with the object as its first parameter.
            return Ok(Some(self.call_the_function(&getter, target, &[], line)?));
        }
        if let Some(held) = held {
            return Ok(Some(held));
        }
        Err(self.no_property(target, written, line))
    }

    /// The call of a property that is a descriptor.
    fn the_call_of_the_descriptor(&self, target: &Value, name: &str) -> Option<Value> {
        let Value::Object(object) = target else { return None };
        let current = object.borrow().value_of(&name.to_lowercase())?;
        if !self.is_a_descriptor(&current) {
            return None;
        }
        let Value::Object(descriptor) = &current else { return None };
        descriptor.borrow().value_of("call")
    }

    /// The value of a property that is called.
    ///
    /// v2 asks the getter of the descriptor for the value of the property,
    /// and calls it when it is a function. A property that is not a
    /// descriptor is not here: the class of the object answers for it.
    fn the_value_of_the_descriptor_for_a_call(
        &mut self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        let Value::Object(object) = target else { return Ok(None) };
        let Some(current) = object.borrow().value_of(&name.to_lowercase()) else {
            return Ok(None);
        };
        if !self.is_a_descriptor(&current) {
            return Ok(None);
        }
        let Value::Object(descriptor) = &current else { return Ok(None) };
        let getter = descriptor.borrow().value_of("get");
        let Some(getter) = getter else { return Ok(None) };
        Ok(Some(self.call_the_function(&getter, target, &[], line)?))
    }

    /// Whether a value is a function of the script.
    fn is_a_function(&self, value: &Value) -> bool {
        let Value::Object(object) = value else { return false };
        object.borrow().class == "func"
    }

    /// The write of a property that is a descriptor.
    ///
    /// v2 asks the setter of the descriptor for the write, and a
    /// descriptor that carries a value is written straight away. A
    /// descriptor with neither of them is read-only.
    fn the_write_of_the_descriptor(
        &mut self,
        descriptor: &Value,
        target: &Value,
        value: Value,
        written: &str,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let Value::Object(object) = descriptor else { return Ok(()) };
        let setter = object.borrow().value_of("set");
        if let Some(setter) = setter {
            // The setter is asked with the object and the value that is
            // written as its two first parameters.
            self.call_the_function(&setter, target, &[value], line)?;
            return Ok(());
        }
        let holds = object.borrow().properties.contains_key("value");
        if holds {
            object.borrow_mut().put("value", value);
            return Ok(());
        }
        Err(RuntimeFailure::Detailed(
            RuntimeError { message: "Property is read-only.".to_string(), line },
            Detail { what: String::new(), extra: written.to_string() },
        ))
    }

    /// The call of a function of the script, with the values of its
    /// arguments.
    ///
    /// The first argument of the call is the object the function belongs
    /// to: v2 asks the getter and the setter of a descriptor with it.
    fn call_the_function(
        &mut self,
        function: &Value,
        target: &Value,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let mut values = Vec::with_capacity(arguments.len() + 1);
        values.push(target.clone());
        values.extend_from_slice(arguments);
        match crate::classes::func::call_the_value(function, &values, self, line) {
            Some(result) => result,
            None => Err(self.no_method(target, "Call", line)),
        }
    }

    /// The refusal of a method that a value does not have.
    pub(crate) fn no_method(&self, target: &Value, name: &str, line: u32) -> RuntimeFailure {
        let kind = self.the_type_of(target);
        // A class that this runtime does not carry is not refused with the
        // words of the dialect: the name is there, and the tree does not
        // carry it. A class value is a class of the dialect, and the
        // refusal of a member that is not there keeps the words of the
        // dialect.
        if self.class_name_of(target).is_none() {
            if let Some(reason) = crate::classes::the_reason_of_a_class_that_is_not_carried(&kind) {
                return RuntimeFailure::Error(RuntimeError { message: reason.to_string(), line });
            }
        }
        RuntimeFailure::Error(RuntimeError {
            message: format!("This value of type \"{kind}\" has no method named {name:?}."),
            line,
        })
    }

    /// The refusal of an item that a value does not have.
    ///
    /// The item of a value is the `__Item` of it, and that is the name the
    /// refusal carries.
    fn no_item(&self, target: &Value, line: u32) -> RuntimeFailure {
        RuntimeFailure::Error(RuntimeError {
            message: format!(
                "This value of type \"{}\" has no property named \"__Item\".",
                self.the_type_of(target)
            ),
            line,
        })
    }

    /// The name of a method, the way v2 names it.
    ///
    /// The name of the class, the name of the prototype, and the name of
    /// the method.
    fn the_name_of_the_method(&self, class: &str, method: &MethodDef) -> String {
        let owner = self
            .classes
            .get(class)
            .map(|definition| definition.name.clone())
            .unwrap_or_else(|| class.to_string());
        format!("{owner}.Prototype.{}", method.name)
    }

    /// The value of a method of a class of the script.
    ///
    /// v2 hands a method over as a function whose first parameter is
    /// `this`: the reference is the method itself, and the object is one
    /// of the arguments of the call.
    fn method_reference(&self, class: &str, method: &MethodDef) -> Value {
        let mut properties: BTreeMap<String, Property> = BTreeMap::new();
        put(&mut properties, "name", Value::Text(self.the_name_of_the_method(class, method)));
        // The first parameter of the reference is `this`.
        let required = method
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        put(&mut properties, "minparams", Value::Int(required as i64 + 1));
        put(&mut properties, "maxparams", Value::Int(method.parameters.len() as i64 + 1));
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(&mut properties, METHOD_CLASS, Value::Text(class.to_string()));
        put(&mut properties, METHOD_NAME, Value::Text(method.name.clone()));
        Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
    }

    /// The reference to a method of a class type of the runtime.
    ///
    /// v2 hands a method over as a function whose first parameter is
    /// `this`: the reference is the method itself, and the object is one
    /// of the arguments of the call.
    pub(crate) fn method_reference_of_a_class_type(
        &self,
        class: &str,
        spelled: &str,
        min: i64,
        max: i64,
    ) -> Value {
        let mut properties = BTreeMap::new();
        let owner = crate::classes::the_spelling_of(class);
        put(&mut properties, "name", Value::Text(format!("{owner}.Prototype.{spelled}")));
        put(&mut properties, "minparams", Value::Int(min));
        put(&mut properties, "maxparams", Value::Int(max));
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(&mut properties, METHOD_CLASS, Value::Text(class.to_string()));
        put(&mut properties, METHOD_NAME, Value::Text(spelled.to_string()));
        Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
    }

    /// A method of a class of the script, called through the reference to it.
    ///
    /// The reference is the method itself: the first argument of the
    /// call is `this`, and the rest are the arguments of the method.
    pub(crate) fn call_a_method_reference(
        &mut self,
        class: &str,
        method: &str,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // A method of a class type of the runtime is answered by the
        // class that owns it, and not by the runtime.
        if crate::classes::is_type(class) {
            // The number of the arguments is checked here: the call goes
            // through a value, and the object is one of the arguments of
            // it.
            if let Some((min, max)) = crate::classes::the_arity_of(class, method) {
                let count = arguments.len() as i64;
                if count < min + 1 || (max >= 0 && count > max + 1) {
                    let message = if count < min + 1 {
                        "Too few parameters passed to function."
                    } else {
                        "Too many parameters passed to function."
                    };
                    return Err(crate::classes::the_arity_failure(class, method, message, line));
                }
            }
            let Some((this, rest)) = arguments.split_first() else {
                return Err(crate::classes::the_arity_failure(
                    class,
                    method,
                    "Too few parameters passed to function.",
                    line,
                ));
            };
            return match crate::classes::call(class, method, this, rest, self, line) {
                Some(result) => result,
                None => Err(self.no_method(this, method, line)),
            };
        }
        let Some((defining, definition)) = self.find_method_in(class, method) else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!("There is no method named {method:?} in this class."),
                line,
            }));
        };
        // The object is the first argument: the reference is the method,
        // and the method takes the object as `this`.
        let Some((this, rest)) = arguments.split_first() else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Missing a required parameter.".into(),
                line,
            }));
        };
        self.call_method_with(&defining, &definition, this, rest, line)
    }

    /// A property of a value.
    fn read_property(&mut self, target: &Value, name: &str, line: u32) -> Result<Value, RuntimeFailure> {
        self.read_property_with(target, name, &[], line)
    }

    /// A property of a value, with the parameters of the access.
    ///
    /// The parameters are the ones between the brackets of the access:
    /// `x.y[z]` asks the `__Get` of `x.y` with the index `z`.
    fn read_property_with(
        &mut self,
        target: &Value,
        name: &str,
        params: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        self.read_property_fully(target, name, params, line)
            .map(|(value, _)| value)
    }

    /// A property of a value, with the parameters of the access.
    ///
    /// The answer says whether a meta-function answered it: the
    /// meta-function of a class handles the whole access, and the
    /// brackets of the access are its parameters.
    fn read_property_fully(
        &mut self,
        target: &Value,
        name: &str,
        params: &[Value],
        line: u32,
    ) -> Result<(Value, bool), RuntimeFailure> {
        // The name is written as the script wrote it, and the lookup is in
        // lower case: the tree does not know the case of a property.
        let written = name.to_string();
        let name = name.to_lowercase();
        let Value::Object(object) = target else {
            // Every value carries its class, and the class of a value
            // that is not an object is the kind of it.
            if name == "__class" {
                return Ok((Value::Text(self.the_class_of_the_value(target)), false));
            }
            return Err(self.no_property(target, &written, line));
        };
        let class = object.borrow().class.clone();
        // The prototype of a class holds the members of the class, and the
        // class that it is the prototype of is the one that answers for
        // them.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Prototype.Get`
        // is the method of the `Map`, and `Map.Prototype.Count` is refused.
        let owner = self.the_class_of_the_prototype(target);
        let members = owner.clone().unwrap_or_else(|| class.clone());
        // A property of the class cannot be read on the prototype of it: the
        // value is not an instance of the class.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31:
        // `Map.Prototype.Count` is refused with "Expected a Map but got a
        // Prototype.".
        if let Some(class) = &owner {
            let property =
                crate::classes::the_property_of_the_prototype(class, &name).map(|spelled| spelled.to_string());
            if let Some(spelled) = property {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: expected_the_class(
                            &self.the_spelling_of_the_class(class),
                            "Prototype",
                        ),
                        line,
                    },
                    Detail {
                        what: self.the_name_of_a_member(class, &format!("{spelled}.Get")),
                        extra: String::new(),
                    },
                ));
            }
        }
        // A class type of the runtime has properties of its own, and it
        // owns the whole of them. A class value is not an instance of its
        // class, and the properties of the class do not answer it.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Count` is
        // refused with 'This value of type "Class" has no property named
        // "Count".'
        if self.class_name_of(target).is_none() {
            if let Some(result) = crate::classes::property(&class, &name, target, self, line) {
                return result.map(|value| (value, false));
            }
        }
        // A property of the class that has a getter: the read of it is the
        // value of the getter.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `obj.value` is
        // the value of the getter of the property, and a property that has
        // no getter is not read.
        if let Some(value) = self.the_value_of_an_accessor(target, &written, line)? {
            return Ok((value, false));
        }
        if let Some(value) = object.borrow().value_of(&name) {
            // A property may be a descriptor: the getter of it is asked
            // for the value of the property.
            if let Some(answer) = self.the_value_of_the_descriptor(&value, target, &written, line)? {
                return Ok((answer, false));
            }
            return Ok((value, false));
        }
        // The static methods of a class are its own properties, and v2
        // hands them over as values.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Call` is a
        // function whose name is "Map.Call".
        if self.class_name_of(target).is_some() {
            if let Some(value) = self.the_static_method_of_a_class_value(target, &name, line)? {
                return Ok((value, false));
            }
            // Every class has an initialiser, and it is a member of the
            // class value.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: `Map.__Init`
            // is a function whose name is empty, and `C.__Init` is one
            // whose name is "C.__Init".
            if name == "__init" {
                return Ok((self.the_init_of_a_class_value(target), false));
            }
            // The classes that a class holds are its own properties.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: `Outer.Inner`
            // is the class value of the class inside `Outer`.
            if let Some(value) = self.the_class_that_a_class_holds(target, &name) {
                return Ok((value, false));
            }
        }
        // The base of an object is a property of every object, and the
        // base of a class is a property of every class.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the base of
        // an object is the prototype of its class, the base of a class
        // is the class that it extends, and the base of a prototype is
        // the prototype of the class that it extends.
        if name == "base" {
            if let Some(base) = self.the_base_of_the_value(target) {
                return Ok((base, false));
            }
        }
        // Every value carries its class.
        if name == "__class" {
            return Ok((Value::Text(self.the_class_of_the_value(target)), false));
        }
        // A method of the class is a value: v2 hands it over as a
        // function whose first parameter is `this`. A class is not an
        // instance, and its methods are not handed over.
        if self.class_name_of(target).is_none() {
            // A static method is not a member of an instance, and it is not
            // a member of the prototype of the class either.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: `obj.Say` is
            // refused with 'This value of type "C" has no property named
            // "Say".', and the prototype of the class refuses it the same
            // way.
            if let Some((defining, method)) = self.find_the_instance_method_in(&members, &name) {
                return Ok((self.method_reference(&defining, &method), false));
            }
            // A method of a class type of the runtime is a value too: v2
            // hands it over as a function whose first parameter is
            // `this`.
            if let Some((spelled, min, max)) = crate::classes::the_method_of(&members, &name) {
                let reference =
                    self.method_reference_of_a_class_type(&members, spelled, min, max);
                return Ok((reference, false));
            }
            // A method of the base object is a value too, and every object
            // has it.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31:
            // `o.Clone` is a function whose name is "Object.Prototype.Clone",
            // and `o.HasProp` is one whose name is "HasProp".
            if crate::classes::the_base_object_has_the_method(&name) {
                let spelled = crate::classes::the_name_of_a_method_of_the_base_object(&name);
                let (min, max) = crate::classes::the_arity_of("object", &name).unwrap_or((0, 0));
                let reference =
                    self.method_reference_of_the_base_object(&spelled, &name, min + 1, max + 1);
                return Ok((reference, false));
            }
        }
        // `__Get` is asked when the property is not there. It takes the
        // name of the property and the parameters of the access.
        if let Some((defining, method)) = self.find_the_instance_method_in(&class, "__get") {
            let params = self.the_params(params, line)?;
            let value = self.call_method_with(
                &defining,
                &method,
                target,
                &[Value::Text(written), params],
                line,
            )?;
            return Ok((value, true));
        }
        Err(self.no_property(target, &written, line))
    }

    /// A property of a value that is written.
    fn write_property(
        &mut self,
        target: &Value,
        name: &str,
        value: Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        self.write_property_with(target, name, &[], value, line)
    }

    /// A property of a value that is written, with the parameters of the
    /// access.
    ///
    /// The parameters are the ones between the brackets of the access.
    fn write_property_with(
        &mut self,
        target: &Value,
        name: &str,
        params: &[Value],
        value: Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let written = name.to_string();
        let name = name.to_lowercase();
        let Value::Object(object) = target else {
            // The class of a value that is not an object cannot be
            // written: v2 says that the property is read-only.
            if name == "__class" {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError { message: "Property is read-only.".to_string(), line },
                    Detail { what: String::new(), extra: written },
                ));
            }
            return Err(self.no_property(target, &written, line));
        };
        let class = object.borrow().class.clone();
        // A class value is not an instance of its class: the properties of
        // the class do not answer it, and a property of the class value is
        // a property of the class value itself.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Count` may
        // be written, and `Map.HasProp` may not.
        if self.class_name_of(target).is_some() {
            return self.write_the_property_of_a_class_value(target, &class, &name, &written, value, line);
        }
        // The base of an object is written the way the `ObjSetBase` built-in
        // writes it: the value has to be an object, and the base of an
        // object of a class type of the runtime cannot be written.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `m.Base := 1` is
        // refused with "Expected an Object but got an Integer." and the
        // `What` "ObjSetBase".
        if name == "base" {
            if !matches!(value, Value::Object(_)) {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: expected_type(TypeName::Object, value.type_name()),
                        line,
                    },
                    Detail { what: "ObjSetBase".to_string(), extra: value.to_text() },
                ));
            }
            let base = value.clone();
            return self
                .set_base_of(target, base, line)
                .map_err(|error| RuntimeFailure::Detailed(
                    error,
                    Detail { what: "ObjSetBase".to_string(), extra: String::new() },
                ));
        }
        // The members of the prototype of a class are read-only: the class
        // holds them, and the prototype is not the one that has them.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Prototype.Count := 5`
        // is refused with "Property is read-only." and the name of the member
        // in the `Extra`. A name that the prototype does not own is written,
        // and it becomes an own property of the prototype.
        if let Some(class) = self.the_class_of_the_prototype(target) {
            if self.the_prototype_has_the_member(&class, &name) {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError { message: "Property is read-only.".to_string(), line },
                    Detail { what: String::new(), extra: written.clone() },
                ));
            }
        }
        // A class type of the runtime owns the whole of its properties.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the property that
        // the class owns and that may not be written is refused with
        // "Property is read-only.", the name of the property is in the
        // `Extra`, and the `What` is empty.
        if crate::classes::the_property_is_read_only(&class, &name) {
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message: "Property is read-only.".to_string(), line },
                Detail { what: String::new(), extra: written },
            ));
        }
        if let Some(result) = crate::classes::set_property(&class, &name, target, value.clone(), self, line) {
            return result;
        }
        // A property of the class that has a setter: the write of it calls
        // the setter.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: a property that
        // has no setter is not written, and the write of it is refused with
        // "Property is read-only.".
        if self.the_write_of_an_accessor(target, &written, value.clone(), line)? {
            return Ok(());
        }
        // A property that is a descriptor has a setter of its own, and a
        // descriptor that carries a value is written straight away.
        if let Some(current) = object.borrow().value_of(&name) {
            if self.is_a_descriptor(&current) {
                return self.the_write_of_the_descriptor(&current, target, value, &written, line);
            }
        }
        // A property the class declares is written straight away. The
        // others are the ones `__Set` is asked about, and it is the one
        // that keeps the value: a class that writes a property of its
        // own inside `__Set` would otherwise call itself for ever.
        if !self.declares_property(&class, &name) {
            if let Some((defining, method)) = self.find_method_in(&class, "__set") {
                // `__Set` takes the name of the property, the parameters
                // of the access, and the value that is written.
                let params = self.the_params(params, line)?;
                self.call_method_with(
                    &defining,
                    &method,
                    target,
                    &[Value::Text(written), params, value],
                    line,
                )?;
                return Ok(());
            }
        }
        // The name of the property is the one that the script wrote.
        put(&mut object.borrow_mut().properties, &written, value);
        Ok(())
    }

    /// A property of a class value, written.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the static methods
    /// of a class are read-only, the base of a class is written the way
    /// the `ObjSetBase` built-in writes it, and every other name is a
    /// property of the class value itself.
    fn write_the_property_of_a_class_value(
        &mut self,
        target: &Value,
        class: &str,
        name: &str,
        written: &str,
        value: Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        // The static methods of a class are read-only: `Map.HasProp` is a
        // method of the class and not of an instance of it.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the write of a
        // static method of a class of the script is refused as well.
        if self.the_static_methods_of_a_class_value(target).contains(&name.to_string()) {
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message: "Property is read-only.".to_string(), line },
                Detail { what: String::new(), extra: written.to_string() },
            ));
        }
        // The base of a class is written the way the `ObjSetBase` built-in
        // writes it: the value has to be an object.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `c.Base := 1` is
        // refused with "Expected an Object but got an Integer." and the
        // `What` "ObjSetBase".
        if name == "base" {
            if !matches!(value, Value::Object(_)) {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: expected_type(TypeName::Object, value.type_name()),
                        line,
                    },
                    Detail { what: "ObjSetBase".to_string(), extra: value.to_text() },
                ));
            }
            let base = value.clone();
            return self
                .set_base_of(target, base, line)
                .map_err(|error| RuntimeFailure::Detailed(
                    error,
                    Detail { what: "ObjSetBase".to_string(), extra: String::new() },
                ));
        }
        let Value::Object(object) = target else {
            return Err(self.no_property(target, written, line));
        };
        // A property of the class that has a setter: the write of it calls
        // the setter.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the static
        // property of a class that has a setter is written through it.
        if self.the_write_of_an_accessor(target, written, value.clone(), line)? {
            return Ok(());
        }
        // A property that is a descriptor has a setter of its own, and a
        // descriptor that carries a value is written straight away.
        if let Some(current) = object.borrow().value_of(name) {
            if self.is_a_descriptor(&current) {
                return self.the_write_of_the_descriptor(&current, target, value, written, line);
            }
        }
        // A property the class declares is written straight away. The
        // others are the ones `__Set` is asked about.
        if !self.declares_property(class, name) {
            if let Some((defining, method)) = self.find_method_in(class, "__set") {
                let params = self.the_params(&[], line)?;
                self.call_method_with(
                    &defining,
                    &method,
                    target,
                    &[Value::Text(written.to_string()), params, value],
                    line,
                )?;
                return Ok(());
            }
        }
        // The name of the property is the one that the script wrote.
        put(&mut object.borrow_mut().properties, written, value);
        Ok(())
    }

    /// The parameters of an access, as an array.
    ///
    /// v2 hands them to the meta-functions as an array, and it is empty
    /// when the access has no parameters.
    fn the_params(&mut self, params: &[Value], line: u32) -> Result<Value, RuntimeFailure> {
        match crate::classes::construct("array", params, self, line) {
            Some(result) => result,
            None => Ok(Value::List(params.to_vec())),
        }
    }

    /// Whether a class declares a property.
    fn declares_property(&self, class: &str, name: &str) -> bool {
        let name = name.to_lowercase();
        self.class_chain(class).iter().any(|class| {
            self.classes
                .get(class)
                .map(|definition| {
                    definition.properties.iter().any(|property| property.name.to_lowercase() == name)
                })
                .unwrap_or(false)
        })
    }

    /// Whether the class declares a property of its instances.
    ///
    /// A static variable belongs to the class and not to its instances.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: `obj.HasProp("value")`
    /// answers 0 when `value` is a static variable of the class.
    fn declares_an_instance_property(&self, class: &str, name: &str) -> bool {
        let name = name.to_lowercase();
        self.class_chain(class).iter().any(|class| {
            self.classes
                .get(class)
                .map(|definition| {
                    definition.properties.iter().any(|property| {
                        !property.is_static && property.name.to_lowercase() == name
                    })
                })
                .unwrap_or(false)
        })
    }

    /// The property of the class that has a getter or a setter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a class may
    /// declare a property with a getter and a setter, and the properties
    /// of the base are the ones of the instances too. A class value has the
    /// static properties of the class, and the properties of the instances
    /// are not its own.
    fn the_accessor_of(&self, target: &Value, name: &str) -> Option<AccessDef> {
        let lower = name.to_lowercase();
        let (class, is_static) = match self.class_name_of(target) {
            Some(class) => (class, true),
            None => {
                let class = match self.the_class_of_the_prototype(target) {
                    Some(class) => class,
                    None => match target {
                        Value::Object(object) => object.borrow().class.clone(),
                        _ => return None,
                    },
                };
                (class, false)
            }
        };
        for class in self.class_chain(&class) {
            let Some(definition) = self.classes.get(&class) else { continue };
            let Some(access) = definition.accessors.get(&lower) else { continue };
            if access.is_static == is_static {
                return Some(access.clone());
            }
        }
        None
    }

    /// The value of a property that has a getter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the read of the
    /// property is the value of the getter, and a property that has no
    /// getter is not read.
    fn the_value_of_an_accessor(
        &mut self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        let Some(access) = self.the_accessor_of(target, name) else { return Ok(None) };
        let Some(getter) = &access.getter else {
            return Err(self.no_property(target, name, line));
        };
        let value = self.call_the_body_of_an_accessor(target, &access, getter, line, &[], None)?;
        Ok(Some(value))
    }

    /// The write of a property that has a setter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the write of the
    /// property calls the setter, and a property that has no setter is not
    /// written: the write is refused with "Property is read-only." and the
    /// name of the property is the `Extra`.
    fn the_write_of_an_accessor(
        &mut self,
        target: &Value,
        name: &str,
        value: Value,
        line: u32,
    ) -> Result<bool, RuntimeFailure> {
        let Some(access) = self.the_accessor_of(target, name) else { return Ok(false) };
        let Some(setter) = &access.setter else {
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message: "Property is read-only.".to_string(), line },
                Detail { what: String::new(), extra: name.to_string() },
            ));
        };
        self.call_the_body_of_an_accessor(target, &access, setter, line, &[], Some(&value))?;
        Ok(true)
    }

    /// The body of a getter or of a setter, run on a value.
    ///
    /// The body runs as the body of a method: `this` is the value, and the
    /// parameters of the property are the ones of the method.
    fn call_the_body_of_an_accessor(
        &mut self,
        target: &Value,
        access: &AccessDef,
        body: &AccessBody,
        line: u32,
        given: &[Value],
        written: Option<&Value>,
    ) -> Result<Value, RuntimeFailure> {
        let mut parameters = access.parameters.clone();
        let mut values: Vec<Value> = given.to_vec();
        if let Some(written) = written {
            // The setter of v2 takes the value that is written after the
            // parameters of the property, and the name of it is "value".
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: the setter of
            // the property writes with it: `set => this.stored := value`.
            parameters.push(Parameter {
                name: "value".to_string(),
                default: None,
                variadic: false,
                by_ref: false,
            });
            values.push(written.clone());
        }
        let method = MethodDef {
            name: access.name.clone(),
            parameters,
            body: body.body.clone(),
            declarations: body.declarations.clone(),
            is_static: access.is_static,
            line: access.line,
        };
        let class = match self.class_name_of(target) {
            Some(class) => class,
            None => match self.the_class_of_the_prototype(target) {
                Some(class) => class,
                None => match target {
                    Value::Object(object) => object.borrow().class.clone(),
                    _ => String::new(),
                },
            },
        };
        self.call_method_with(&class, &method, target, &values, line)
    }

    /// The name of a class, as the class that holds it writes it.
    ///
    /// The class inside `Outer` is "Outer.Inner", and the name of it as a
    /// property of `Outer` is "Inner".
    fn the_name_of_a_class(&self, class: &str) -> String {
        let name = self
            .classes
            .get(class)
            .map(|definition| definition.name.clone())
            .unwrap_or_else(|| class.to_string());
        name.rsplit('.').next().unwrap_or(&name).to_string()
    }

    /// The class that a class holds, as a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the class inside
    /// `Outer` is an own property of the class value of `Outer`, and it is
    /// not a member of an instance of it.
    fn the_class_that_a_class_holds(&self, target: &Value, name: &str) -> Option<Value> {
        let class = self.class_name_of(target)?;
        let definition = self.classes.get(&class)?;
        let lower = name.to_lowercase();
        for nested in &definition.nested {
            if self.the_name_of_a_class(nested).to_lowercase() == lower {
                return self.class_value_of(nested);
            }
        }
        None
    }

    /// The argument of a loop, which has to be there.
    fn loop_argument(
        &mut self,
        arguments: &[Node],
        index: usize,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let node = arguments.get(index).ok_or_else(|| {
            RuntimeFailure::Error(RuntimeError {
                message: format!("the loop needs an argument {}.", index + 1),
                line,
            })
        })?;
        self.evaluate(node)
    }

    /// What a directive asks for, and whether this runtime can do it.
    ///
    /// A directive that names a thing this runtime does not have fails
    /// the script with the reason: a script that asks for a hotkey and
    /// is given none has been told a lie. A directive that asks for what
    /// is already the case is accepted.
    fn directive(
        &mut self,
        name: &str,
        argument: &str,
        line: u32,
    ) -> Result<String, RuntimeFailure> {
        let reason = |what: &str| RuntimeFailure::Error(RuntimeError {
            message: format!("the {what} are not carried by this runtime."),
            line,
        });
        match name {
            // There is no tray icon to hide, and the runner is one
            // process: both are already what the directive asks for.
            "notrayicon" => Ok("there is no tray icon to hide".into()),
            "singleinstance" => Ok("the runner is one process".into()),
            // The errors are always in the report, and the report is
            // always on the standard output.
            "errorstdout" => Ok("the errors are in the report".into()),
            "requires" => {
                let words: Vec<&str> = argument.split_whitespace().collect();
                let wanted = words.get(1).copied().unwrap_or("");
                if wanted.starts_with("v2") {
                    Ok(format!("this runtime is {wanted}"))
                } else {
                    Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("This script requires {wanted}, and this runtime is v2."),
                        line,
                    }))
                }
            }
            "include" | "includeagain" => Err(RuntimeFailure::Error(RuntimeError {
                message: "the includes are resolved by the runner, not by the language.".into(),
                line,
            })),
            "us hook" => unreachable!(),
            "usehook" | "hotif" | "hotiftimeout" | "hotstring" | "maxthreads" | "maxthreadsperhotkey"
            | "maxthreadsbuffer" => Err(reason("hotkeys")),
            "clipboardtimeout" => Err(reason("clipboard")),
            "inputlevel" => Err(reason("input level")),
            "suspendexempt" => Err(reason("suspend")),
            "winactivateforce" => Err(reason("window activation")),
            "warn" => Err(reason("warnings")),
            "dllload" => Err(RuntimeFailure::Error(RuntimeError {
                message: "a Windows library is a Windows thing and this box has none.".into(),
                line,
            })),
            other => Err(RuntimeFailure::Error(RuntimeError {
                message: format!("{other} is not a directive this runtime knows."),
                line,
            })),
        }
    }

    /// The result of an assignment: the value that is stored.
    ///
    /// The compound operators read what is there before they write, so a
    /// variable that has no value yet is an error and not a zero. The
    /// concat is the one that is not: it reads a variable that has no
    /// value as an empty text.
    fn apply_assign(
        &self,
        op: AssignOp,
        name: &str,
        current: Option<Value>,
        value: Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        Ok(match op {
            AssignOp::Set => value,
            AssignOp::Add => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.number(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let a = self.number(&current, line)?;
                match (&current, &value) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    _ => Value::Float(a + written),
                }
            }
            AssignOp::Subtract => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.number(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let a = self.number(&current, line)?;
                match (&current, &value) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                    _ => Value::Float(a - written),
                }
            }
            AssignOp::Multiply => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.number(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let a = self.number(&current, line)?;
                match (&current, &value) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                    _ => Value::Float(a * written),
                }
            }
            AssignOp::Divide => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.number(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let a = self.number(&current, line)?;
                if written == 0.0 {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: "Divide by zero.".into(),
                        line,
                    }));
                }
                Value::Float(a / written)
            }
            AssignOp::IntegerDivide => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.integer(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let a = self.integer(&current, line)?;
                if written == 0 {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: "Divide by zero.".into(),
                        line,
                    }));
                }
                Value::Int(a / written)
            }
            AssignOp::Concat => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // concat of a variable that has no value is the concat of
                // an empty text, and the write makes the variable. The
                // other operators want a value that is there, and a
                // variable that has no value is an error for them.
                let current = match current {
                    Some(value) => value,
                    None => Value::Text(String::new()),
                };
                Value::Text(format!("{}{}", current.to_text(), value.to_text()))
            }
            AssignOp::BitwiseAnd => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.integer(&value, line)?;
                let current = self.assigned(name, current, line)?;
                Value::Int(self.integer(&current, line)? & written)
            }
            AssignOp::BitwiseOr => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.integer(&value, line)?;
                let current = self.assigned(name, current, line)?;
                Value::Int(self.integer(&current, line)? | written)
            }
            AssignOp::BitwiseXor => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.integer(&value, line)?;
                let current = self.assigned(name, current, line)?;
                Value::Int(self.integer(&current, line)? ^ written)
            }
            AssignOp::ShiftLeft => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.shift(&value, line)?;
                let current = self.assigned(name, current, line)?;
                Value::Int(self.integer(&current, line)? << written)
            }
            AssignOp::ShiftRight => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.shift(&value, line)?;
                let current = self.assigned(name, current, line)?;
                Value::Int(self.integer(&current, line)? >> written)
            }
            AssignOp::ShiftRightUnsigned => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // dialect converts the value that is written before it
                // reads the one that is there, and the kind it names is
                // the kind of the value that was written.
                let written = self.shift(&value, line)?;
                let current = self.assigned(name, current, line)?;
                let left = self.integer(&current, line)? as u64;
                Value::Int((left >> written) as i64)
            }
        })
    }

    /// The value of a variable of the runtime, when it has none yet.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the variables of
    /// the loops are there before the first loop runs: the index is zero
    /// and the others are empty.
    fn the_value_of_a_variable_of_the_runtime(&self, name: &str) -> Option<Value> {
        let rest = crate::variables::spelled(name)?;
        if !THE_LOOP_VARIABLES.contains(&rest.as_str()) {
            return None;
        }
        if rest == "index" {
            Some(Value::Int(0))
        } else {
            Some(Value::Text(String::new()))
        }
    }

    /// The message v2 gives for a variable that has no value.
    ///
    /// v2 names the kind of the variable and not the name of it: a
    /// variable of the script and a variable of a function are told
    /// apart.
    pub(crate) fn the_unassigned(&self, name: &str) -> String {
        let key = the_key_of(name);
        // A name that lives in the scope of the script is a global one,
        // and a name that lives in the scope of the function is a local
        // one.
        let local = self.declarations.last().is_some() && !self.lives_in_the_script(&key);
        if local {
            "This local variable has not been assigned a value.".to_string()
        } else {
            "This global variable has not been assigned a value.".to_string()
        }
    }

    /// The item of a value, at a position.
    ///
    /// An object is asked for its item: the class of the script first,
    /// and then the class types of the runtime. The other values have no
    /// item, and v2 refuses the index of one.
    fn index_a_value(
        &mut self,
        target: &Value,
        key: &Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        if let Value::Object(_) = target {
            return self.read_item(target, key, line);
        }
        let position = key.to_number().unwrap_or(0.0) as i64;
        // The positions count from one, the way AutoHotkey counts them.
        let position = position.max(1) as usize;
        match target {
            Value::List(items) => items.get(position - 1).cloned().ok_or_else(|| {
                RuntimeFailure::Error(RuntimeError { message: "Invalid index.".into(), line })
            }),
            // A text has no item, and v2 refuses the index of one:
            // `SubStr` is the way to take a letter out.
            _ => Err(self.no_item(target, line)),
        }
    }

    /// The failure of a variable that has no value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// variable goes in the `Extra` of the error, and the `What` is empty.
    fn the_unassigned_failure(&self, name: &str, line: u32) -> RuntimeFailure {
        RuntimeFailure::Detailed(
            RuntimeError { message: self.the_unassigned(name), line },
            Detail { what: String::new(), extra: name.to_string() },
        )
    }

    /// The failure of a parameter that has no value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// parameter is in the `Extra` of the failure, and the `What` is empty.
    fn the_missing_parameter(&self, name: &str, line: u32) -> RuntimeFailure {
        RuntimeFailure::Detailed(
            RuntimeError { message: "Missing a required parameter.".into(), line },
            Detail { what: String::new(), extra: name.to_string() },
        )
    }

    /// The value of a variable that has to be there.
    pub(crate) fn assigned(
        &self,
        name: &str,
        value: Option<Value>,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        value.ok_or_else(|| self.the_unassigned_failure(name, line))
    }

    /// The value as a number, the way the maths operations want it.
    ///
    /// The message names the kind of the value that arrived, and the value
    /// itself goes in the `Extra` of the failure.
    pub(crate) fn number(&self, value: &Value, line: u32) -> Result<f64, RuntimeFailure> {
        value.to_number_strict().map_err(|_| {
            RuntimeFailure::Detailed(
                RuntimeError {
                    // The name of the kind is the one of v2, and an empty
                    // text has a name of its own.
                    message: expected_number(value),
                    line,
                },
                Detail { what: String::new(), extra: the_extra_of_the_value(value) },
            )
        })
    }

    /// The value as a whole number, the way the bitwise operations want it.
    ///
    /// The number comes first: a text that holds a number is a number. A
    /// number with a point is not a whole one, and v2 says so.
    pub(crate) fn integer(&self, value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
        self.the_whole_number(value, line, TypeName::Integer)
    }

    /// The value as a whole number, and the name of the kind it wants.
    ///
    /// The bitwise operations want a whole number, and the bitwise not
    /// says that it wants a number: the message names the one that it says.
    fn the_whole_number(
        &self,
        value: &Value,
        line: u32,
        wants: TypeName,
    ) -> Result<i64, RuntimeFailure> {
        if let Value::Int(number) = value {
            return Ok(*number);
        }
        let number = self.number(value, line)?;
        if fract(number) != 0.0 {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: expected_the_kind(wants, &self.the_type_of(value)),
                    line,
                },
                Detail { what: String::new(), extra: the_extra_of_the_value(value) },
            ));
        }
        Ok(number as i64)
    }

    /// The place of an item, the way the index of an array wants it.
    ///
    /// v2 wants a number: a text that holds a number is a number, and a
    /// number with a point is taken as the whole one before it. A value
    /// that is not a number is not a place at all.
    pub(crate) fn the_place_of_an_item(&self, value: &Value) -> Option<i64> {
        value.to_number().map(|number| trunc(number) as i64)
    }

    /// The text of a value in a concatenation.
    ///
    /// v2 wants a text or a number: an object, an array and a function are
    /// refused, and the message names the kind of the value that arrived.
    fn the_text_of_the_concat(&self, value: &Value, line: u32) -> Result<String, RuntimeFailure> {
        match value {
            Value::Text(text) => Ok(text.clone()),
            Value::Int(_) | Value::Float(_) => Ok(value.to_text()),
            other => Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: expected_the_kind(TypeName::String, &self.the_type_of(other)),
                    line,
                },
                Detail { what: String::new(), extra: String::new() },
            )),
        }
    }

    /// The amount of a shift: a whole number between zero and sixty-three.
    fn shift(&self, value: &Value, line: u32) -> Result<u32, RuntimeFailure> {
        let count = self.integer(value, line)?;
        if !(0..64).contains(&count) {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Invalid shift count.".into(),
                line,
            }));
        }
        Ok(count as u32)
    }

    /// `x++`, `x--`, `++x`, `--x`.
    ///
    /// The operator after the name answers the value the variable had
    /// before; the one before it answers the value it has now.
    fn increment(
        &mut self,
        op: UnaryOp,
        operand: &Node,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let step = match op {
            UnaryOp::PreIncrement | UnaryOp::PostIncrement => 1,
            _ => -1,
        };
        match operand {
            Node::Var { name, .. } => {
                let current = self.lookup(name).ok_or_else(|| self.the_unassigned_failure(name, line))?;
                let stored = self.the_number_after(&current, step, line)?;
                self.assign(name, stored.clone());
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the increment
            // of a property and of an item writes to them.
            Node::Property { target, name, line: property_line } => {
                let target_value = self.evaluate(target)?;
                let name = self.the_name_of_the_member(name, *property_line)?;
                let current = self.read_property(&target_value, &name, *property_line)?;
                let stored = self.the_number_after(&current, step, *property_line)?;
                self.write_a_property(target, &name, AssignOp::Set, stored.clone(), *property_line)?;
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            Node::Index { target, index, line: item_line } => {
                let target_value = self.evaluate(target)?;
                let key = self.evaluate(index)?;
                let current = self.index_a_value(&target_value, &key, *item_line)?;
                let stored = self.the_number_after(&current, step, *item_line)?;
                self.set_item(&target_value, &key, AssignOp::Set, stored.clone(), *item_line)?;
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            _ => Err(RuntimeFailure::Error(RuntimeError {
                message: "the operator needs a variable to work on".into(),
                line,
            })),
        }
    }

    /// The value that comes after the increment of a value.
    fn the_number_after(
        &mut self,
        current: &Value,
        step: i64,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        Ok(match current {
            Value::Int(number) => Value::Int(number + step),
            other => Value::Float(self.number(other, line)? + step as f64),
        })
    }

    // -- calls ----------------------------------------------------------------------

    /// Call a function: the one the script defined, or one of the builtins.
    fn call(&mut self, name: &str, arguments: &[Node], line: u32) -> Result<Value, RuntimeFailure> {
        if let Some(function) = self.functions.get(name).cloned() {
            return self.call_function(&function, arguments, line, Route::Written);
        }
        // The name of a class makes an object. The class types of the
        // runtime are classes too: the script cannot extend one, but it
        // can make one.
        if self.classes.contains_key(&name.to_lowercase()) || crate::classes::is_type(name) {
            return self.construct(name, arguments, line);
        }
        // A method of a class of the script that is held in a variable
        // is called through the reference to it: the first argument of
        // the call is `this`.
        if let Some((class, method)) = self.method_reference_in_a_variable(name) {
            let values = self.evaluate_all(arguments)?;
            return self.call_a_method_reference(&class, &method, &values, line);
        }
        // A function that is held in a variable is called through it.
        if let Some(function) = self.function_in_a_variable(name) {
            return self.call_function(&function, arguments, line, Route::ThroughValue);
        }
        // The value of the name is what the call asks for: a variable of
        // the script, or a built-in variable of the dialect.
        let from_the_script = self.lookup_variable(name);
        let value = match from_the_script {
            Some(value) => Some(Ok(value)),
            None => crate::variables::lookup(name, self),
        };
        // An object that is called is asked through its `Call` method:
        // v2 does not ask the `__Call` of the class when the object
        // itself is called.
        if let Some(value) = value {
            let value = value.map_err(|error| RuntimeFailure::Error(RuntimeError {
                message: error.message,
                line,
            }))?;
            if let Value::Object(object) = &value {
                // A class is a value, and the call of it makes an object of
                // that class. Measured on the binary of v2.0.19 on 2026-10-31:
                // `c := Map; c()` makes a map, the way `Map()` does, and the
                // `Any` is refused because it is not a class that can be
                // made.
                if let Some(class) = self.class_name_of(&value) {
                    return self.construct(&class, arguments, line);
                }
                let class = object.borrow().class.clone();
                if let Some((defining, method)) = self.find_method_in(&class, "call") {
                    return self.call_method(&defining, &method, &value, arguments, line);
                }
                let values = self.evaluate_all(arguments)?;
                if let Some(result) =
                    crate::classes::call_the_value(&class, &value, &values, self, line)
                {
                    return result;
                }
                return Err(self.no_method(&value, "Call", line));
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: a value that
            // is not an object cannot be called either, and v2 names the
            // kind of it: "This value of type "Integer" has no method named
            // "Call".". The failure is of the class `MethodError`.
            return Err(self.no_method(&value, "Call", line));
        }
        // The name is not a function of the script, not a class and not a
        // built-in: it is a variable of the script, and v2 reads it. A
        // variable that has no value is a variable that has no value.
        if crate::builtins::find(name).is_none() {
            return Err(self.the_unassigned_failure(name, line));
        }
        let mut values = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        self.call_builtin(name, &values, line, Route::Written)
    }

    /// Call a function the script defined.
    fn call_function(
        &mut self,
        function: &Function,
        arguments: &[Node],
        line: u32,
        route: Route,
    ) -> Result<Value, RuntimeFailure> {
        let values = self.evaluate_all(arguments)?;
        // The parameters that the command does not give a value to are the
        // ones that are not there: the place of them is kept, and they have
        // no value.
        let missing: Vec<bool> =
            arguments.iter().map(|argument| matches!(argument, Node::Empty { .. })).collect();
        self.call_function_with(function, &values, line, route, &missing)
    }

    /// The call of a function, with the values of the arguments already
    /// there: a function held in a variable is called with values.
    fn call_function_with(
        &mut self,
        function: &Function,
        values: &[Value],
        line: u32,
        route: Route,
        missing: &[bool],
    ) -> Result<Value, RuntimeFailure> {
        // The arity of a function is in its definition: the parameters that
        // have no default are needed, and the ones that are there are the
        // most it takes unless one of them takes the rest.
        let needed = function
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        if values.len() < needed {
            // A call that the script wrote is checked when the script
            // loads, and the failure names the function. A call that
            // goes through a value is checked when the call runs, and
            // the failure names the parameter that is missing.
            if matches!(route, Route::Written) {
                return Err(self.the_arity_failure(
                    "Too few parameters passed to function.",
                    &function.name,
                    route,
                    line,
                ));
            }
            let missing = function
                .parameters
                .iter()
                .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
                .nth(values.len())
                .map(|parameter| parameter.name.clone())
                .unwrap_or_default();
            return Err(self.the_missing_parameter(&missing, line));
        }
        let variadic = function.parameters.iter().any(|parameter| parameter.variadic);
        if !variadic && values.len() > function.parameters.len() {
            return Err(self.the_arity_failure(
                "Too many parameters passed to function.",
                &function.name,
                route,
                line,
            ));
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: a parameter that
        // the command does not give a value to is a missing one, and the
        // call is refused with "Missing a required parameter." -- the name
        // of the parameter is in the `Extra`. The number of the arguments
        // is checked before it.
        for (index, parameter) in function.parameters.iter().enumerate() {
            if missing.get(index).copied().unwrap_or(false)
                && parameter.default.is_none()
                && !parameter.variadic
            {
                return Err(self.the_missing_parameter(&parameter.name, line));
            }
        }
        let mut scope: BTreeMap<String, Value> = BTreeMap::new();
        for (index, parameter) in function.parameters.iter().enumerate() {
            let given = values.get(index);
            // A parameter that the command does not give a value to is a
            // missing one: the default of it is taken, and one that has no
            // default at all was refused above.
            let is_missing = missing.get(index).copied().unwrap_or(false);
            let value = match given {
                Some(value) if !is_missing => value.clone(),
                // A parameter that the call does not give a value to takes
                // its default, and one that has no default at all is empty.
                _ => match &parameter.default {
                    Some(default) => self.evaluate(default)?,
                    None => Value::Text(String::new()),
                },
            };
            // A parameter that the call does not give a value to is a local
            // one, even when it is written `&x`: the default of v2 is a
            // value of its own.
            if parameter.by_ref && given.is_some() && !is_missing {
                // The caller must pass a reference: the parameter is an
                // alias of the variable the reference points at.
                let Value::Reference(reference) = value else {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: wants_a_reference(
                            index + 1,
                            &function.name,
                            "a variable reference",
                            &value,
                        ),
                        line,
                    }));
                };
                let mut alias = reference;
                alias.alias = true;
                scope.insert(the_key_of(&parameter.name), Value::Reference(alias));
                continue;
            }
            scope.insert(the_key_of(&parameter.name), value);
        }
        // The parameter that takes the rest of the arguments takes them as
        // an array, the way v2 has it.
        if let Some(index) = function.parameters.iter().position(|parameter| parameter.variadic) {
            let rest: Vec<Value> = values.iter().skip(index).cloned().collect();
            let array = match crate::classes::construct("array", &rest, self, line) {
                Some(result) => result?,
                None => Value::List(rest),
            };
            scope.insert(the_key_of(&function.parameters[index].name), array);
        }
        self.enter_call(line)?;
        // A function that was written inside another one closes over the
        // scope of the one around it: the scopes go under the scope of the
        // call.
        for closed in &function.closure {
            self.scopes.push(closed.clone());
        }
        self.scopes.push(Rc::new(RefCell::new(scope)));
        // The functions that are written inside this one are there before
        // the body runs.
        self.collect_the_nested_functions(&function.body);
        self.declarations.push(function.declarations.clone());
        self.frames.push(Frame { name: function.name.clone(), line });
        // The labels of the function are the scope of the function.
        self.label_scopes.push(the_labels_of(&function.body));
        let outcome = self.run_block(&function.body);
        self.label_scopes.pop();
        self.frames.pop();
        self.declarations.pop();
        self.scopes.pop();
        for _ in &function.closure {
            self.scopes.pop();
        }
        self.leave_call();
        match outcome {
            Ok(()) => Ok(Value::Text(String::new())),
            Err(RuntimeFailure::Flow(Flow::Return(value))) => {
                Ok(value.unwrap_or(Value::Text(String::new())))
            }
            // The jump of a `Goto` that leaves the function: the label is
            // not in the scope of the function.
            Err(RuntimeFailure::Flow(Flow::Goto { name, line })) => Err(RuntimeFailure::Fatal(
                self.the_label_is_not_there(&name, &function.body, line),
            )),
            Err(other) => Err(other),
        }
    }

    /// Keep a closure under a name of its own, and answer with the name.
    ///
    /// The name of a closure of v2 is empty, and the runtime keeps it under
    /// a name that a script cannot reach.
    fn make_a_closure(&mut self, function: Function) -> String {
        self.closures += 1;
        let name = format!("\u{1}closure {}", self.closures);
        self.functions.insert(name.to_lowercase(), function);
        name
    }

    /// Set a variable in the innermost scope, and not in the scope around
    /// it.
    ///
    /// The name of a function that is written inside another one is local
    /// to the function that has it.
    fn assign_in_the_scope(&mut self, name: &str, value: Value) {
        let name = the_key_of(name);
        match self.scopes.last() {
            Some(scope) => {
                scope.borrow_mut().insert(name, value);
            }
            None => {
                self.variables.insert(name, value);
            }
        }
    }

    /// The write of a variable, and the value that was written.
    ///
    /// A built-in variable that is not a variable of the script is written
    /// where it lives, and not in the table of the script: the clipboard is
    /// the one.
    fn write_a_variable(
        &mut self,
        name: &str,
        op: AssignOp,
        value: Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // A setting of the runtime is written where it lives: the setting
        // changes, and the compound operators read it before they write
        // it. The dialect writes the setting itself, and it is the tree
        // that keeps it.
        if let Some(spelled) = crate::variables::spelled(name) {
            if let Some(Ok(current)) = crate::settings::value_of(&spelled) {
                let stored = self.apply_assign(op, name, Some(current), value, line)?;
                let written = crate::settings::write_of(&spelled, &stored).ok_or_else(|| {
                    RuntimeFailure::Error(RuntimeError {
                        message: "the runtime keeps no such setting.".into(),
                        line,
                    })
                })?;
                return written
                    .map_err(|message| RuntimeFailure::Error(RuntimeError { message, line }));
            }
        }
        match crate::variables::write(name, op, &value, line, self) {
            Some(written) => {
                // The reason of a write carries no line: the statement
                // that asked is the one that carries it.
                let stored = written.map_err(|mut error| {
                    if error.line == 0 {
                        error.line = line;
                    }
                    RuntimeFailure::Error(error)
                })?;
                Ok(stored)
            }
            None => {
                // The value that is there before the write: the variable
                // of the script, or the built-in variable of the same
                // name.
                let current = match self.lookup(name) {
                    Some(value) => Some(value),
                    None => match crate::variables::lookup(name, self) {
                        Some(Ok(value)) => Some(value),
                        _ => None,
                    },
                };
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of a class is a name of the script, and a local that has
                // that name may not hold the class or an instance of it.
                self.the_class_is_not_written(name, &value, line)?;
                let stored = self.apply_assign(op, name, current, value, line)?;
                // A built-in variable that the dialect lets a script write
                // is converted the way the dialect converts it: the value
                // that is written is the value the name keeps.
                let stored = match crate::variables::the_kind_of_the_write(name) {
                    Some(kind) => crate::variables::the_written_value(kind, &stored, line)
                        .map_err(RuntimeFailure::Error)?,
                    None => stored,
                };
                self.assign(name, stored.clone());
                Ok(stored)
            }
        }
    }

    /// The write of a property, and the value that was written.
    fn write_a_property(
        &mut self,
        target: &Node,
        name: &str,
        op: AssignOp,
        value: Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let target_value = self.evaluate(target)?;
        // Measured on the binary of v2.0.19 on 2026-10-31: the compound
        // operators read the property before they write it, and a property
        // that is not there is an error that names the property: the write
        // never happens, and the setter of the object is not called.
        let current = match op {
            AssignOp::Set => self.read_property(&target_value, name, line).ok(),
            _ => Some(self.read_property(&target_value, name, line)?),
        };
        let stored = self.apply_assign(op, name, current, value, line)?;
        self.write_property(&target_value, name, stored.clone(), line)?;
        Ok(stored)
    }

    /// The recursion is a rail: a script that calls itself for ever stops
    /// here, and says so.
    ///
    /// There are two bounds. The depth is the one of v2: v2.0.19 stops at
    /// about 1105 nested calls, with "Function recursion limit exceeded."
    /// The stack is the other one: the frames of the tree are heavier than
    /// the ones of v2, and a host with a small stack would run out of it
    /// before the depth says so.
    fn enter_call(&mut self, line: u32) -> Result<(), RuntimeFailure> {
        if self.depth >= Self::MAX_DEPTH || self.stack_used() > self.stack_budget {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Function recursion limit exceeded.".into(),
                line,
            }));
        }
        self.depth += 1;
        Ok(())
    }

    /// The call is over: the depth goes down.
    fn leave_call(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    /// How much stack the run has used since the runtime was made.
    fn stack_used(&self) -> usize {
        self.stack_base.abs_diff(Self::stack_here())
    }

    /// Where the stack is right now.
    ///
    /// The address of a local is the address of the stack: the marker is
    /// not read, and the compiler is told that it may not fold the address
    /// away.
    fn stack_here() -> usize {
        let marker = 0u8;
        core::hint::black_box(&marker) as *const u8 as usize
    }

    /// The call of a name, with the values of the arguments already
    /// there.
    ///
    /// The name may be a function of the script or a built-in.
    pub(crate) fn call_named(
        &mut self,
        name: &str,
        values: &[Value],
        line: u32,
        route: Route,
    ) -> Result<Value, RuntimeFailure> {
        // A function that is written inside another one is a variable of
        // the scope around it, and the call finds it there.
        if let Some(function) = self.function_in_a_variable(name) {
            return self.call_function_with(&function, values, line, route, &[]);
        }
        if let Some(function) = self.functions.get(&name.to_lowercase()).cloned() {
            return self.call_function_with(&function, values, line, route, &[]);
        }
        self.call_builtin(name, values, line, route)
    }

    /// The function a variable holds, if it holds one.
    fn function_in_a_variable(&self, name: &str) -> Option<Function> {
        let value = self.lookup_variable(name)?;
        let Value::Object(object) = &value else { return None };
        let object = object.borrow();
        if object.class != "func" {
            return None;
        }
        // A closure is kept under a name of its own.
        if let Some(closure) = object.value_of(CLOSURE) {
            return self.functions.get(&closure.to_text().to_lowercase()).cloned();
        }
        let function_name = object.value_of("name")?.to_text().to_lowercase();
        self.functions.get(&function_name).cloned()
    }

    /// The method a variable holds, if it holds a reference to one.
    ///
    /// A reference to a method is a function whose first parameter is
    /// `this`: the class and the name of the method are what it carries.
    fn method_reference_in_a_variable(&self, name: &str) -> Option<(String, String)> {
        let value = self.lookup_variable(name)?;
        let Value::Object(object) = &value else { return None };
        let object = object.borrow();
        if object.class != "func" {
            return None;
        }
        let class = object.value_of(METHOD_CLASS)?.to_text();
        let method = object.value_of(METHOD_NAME)?.to_text();
        Some((class, method))
    }

    /// The failure of a call that does not have the number of the
    /// arguments the function wants.
    ///
    /// A call that the script wrote is checked when the script loads,
    /// and the failure names the function: "Too few parameters passed
    /// to function.\n     Specifically: Abs". A call that goes through a
    /// value is checked when the call runs, and the failure is the one
    /// of the run: the name of the function goes in the `Extra`.
    pub(crate) fn the_arity_failure(
        &self,
        message: &str,
        name: &str,
        route: Route,
        line: u32,
    ) -> RuntimeFailure {
        match route {
            Route::Written => RuntimeFailure::Error(RuntimeError {
                message: format!("{message}\n     Specifically: {name}"),
                line,
            }),
            Route::ThroughValue => RuntimeFailure::Detailed(
                RuntimeError { message: message.to_string(), line },
                Detail { what: String::new(), extra: name.to_string() },
            ),
            Route::ThroughTheCall => RuntimeFailure::Detailed(
                RuntimeError { message: message.to_string(), line },
                // The call went through the `Call` method of a value, and
                // v2 names that method in the `What`.
                Detail { what: "Func.Prototype.Call".to_string(), extra: name.to_string() },
            ),
        }
    }

    /// The functions the language carries.
    fn call_builtin(
        &mut self,
        name: &str,
        values: &[Value],
        line: u32,
        route: Route,
    ) -> Result<Value, RuntimeFailure> {
        let Some(builtin) = crate::builtins::find(name) else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Call to nonexistent function.".into(),
                line,
            }));
        };
        // The arity is in the table, so the check is the same for every
        // built-in and it says what AutoHotkey says.
        if values.len() < builtin.min as usize {
            return Err(self.the_arity_failure(
                "Too few parameters passed to function.",
                builtin.name,
                route,
                line,
            ));
        }
        if values.len() > builtin.max as usize {
            return Err(self.the_arity_failure(
                "Too many parameters passed to function.",
                builtin.name,
                route,
                line,
            ));
        }
        // A reference is for the parameters that are written back: the
        // rest want a value, and v2 says which parameter got one and what
        // the built-in wanted there.
        let written_back = crate::builtins::reference_parameters(builtin.name);
        // The built-in that says itself which parameters take a reference
        // asks in its own handler.
        let decides = crate::builtins::decides_references(builtin.name);
        for (index, value) in values.iter().enumerate() {
            if !matches!(value, Value::Reference(_))
                || written_back.contains(&index)
                || decides
                || crate::builtins::refuses_itself(builtin.name)
            {
                continue;
            }
            // A reference is a value of its own kind. Where the parameter
            // wants a text, a number or an object, the reference does not
            // fit and the built-in says so. Where the parameter takes any
            // value, the reference goes through, and the built-in answers
            // about it: the `Type` names it "VarRef".
            let message = match crate::builtins::wants(builtin.name, index) {
                crate::builtins::Wants::Declared(kind) => {
                    requires(index + 1, builtin.name, kind, value)
                }
                crate::builtins::Wants::Checked(kind) => {
                    expects_a_kind(kind, value)
                }
                // A parameter that wants a reference takes one.
                crate::builtins::Wants::Reference(_) => continue,
                crate::builtins::Wants::Value => continue,
            };
            // The name of the built-in goes in the `What` of the
            // failure: v2 carries it there.
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message, line },
                Detail { what: builtin.name.to_string(), extra: String::new() },
            ));
        }
        // A parameter that v2 declares wants a value of a kind, and a
        // value of another kind is refused with the reason: the name of
        // the built-in, the number of the parameter and the kind that
        // arrived.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: a parameter
        // that wants a text takes a text and a number, and one that
        // wants a number takes a number and a text that holds one. A
        // check that the body of the function makes wants the kind of
        // the value itself: a number is not a text.
        for (index, value) in values.iter().enumerate() {
            // A reference is the business of the guard above: the
            // parameters that are written back take one.
            if matches!(value, Value::Reference(_)) {
                continue;
            }
            let message = match crate::builtins::wants(builtin.name, index) {
                crate::builtins::Wants::Declared(kind) => {
                    if accepts(kind, value) {
                        continue;
                    }
                    requires(index + 1, builtin.name, kind, value)
                }
                crate::builtins::Wants::Checked(kind) => {
                    if is_of_the_kind(kind, value) {
                        continue;
                    }
                    expects_a_kind(kind, value)
                }
                crate::builtins::Wants::Reference(wants) => {
                    wants_a_reference(index + 1, builtin.name, wants, value)
                }
                crate::builtins::Wants::Value => continue,
            };
            // v2 carries the value that arrived in the `Extra`: the
            // text of it, and nothing for an object, an array or a
            // function.
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message, line },
                Detail { what: builtin.name.to_string(), extra: the_extra_of_the_value(value) },
            ));
        }
        // A command may declare fewer parameters than it needs: the table
        // of the binary says how many the command takes, and the command
        // says which one it cannot do without. The refusal of v2 names
        // the parameter and not the count, and it comes after the kind of
        // the values that are there: the dialect converts the arguments
        // before the body of the function runs.
        for number in crate::builtins::required_parameters(builtin.name) {
            if values.len() < *number as usize {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!(
                            "Parameter #{number} of {} must not be omitted in this case.",
                            builtin.name
                        ),
                        line,
                    },
                    Detail { what: String::new(), extra: String::new() },
                ));
            }
        }
        (builtin.handler)(self, values, line, builtin.variant).map_err(|error| {
            // The failure of a built-in carries the name of the built-in
            // in the `What`, and the value of the parameter that failed
            // in the `Extra`.
            let extra = crate::builtins::the_extra_of(builtin.name, &error.message, values)
                .unwrap_or_else(|| the_parameter_that_failed(&error.message, values));
            RuntimeFailure::Detailed(
                error,
                Detail { what: builtin.name.to_string(), extra },
            )
        })
    }

    /// The desktop the script drives.
    pub(crate) fn desktop(&mut self) -> &mut (dyn crate::runtime::Desktop + 'a) {
        &mut *self.desktop
    }

    /// The directory where the run started, when the runtime knows it.
    ///
    /// The desktop says it when the runtime is made, and the runtime keeps
    /// it: a script that changes the working directory does not change the
    /// one where the run began.
    pub(crate) fn initial_working_dir(&self) -> Option<&str> {
        self.initial_working_dir.as_deref()
    }

    /// The pause the script asked for.
    pub(crate) fn sleep_ms(&mut self, millis: u64) {
        self.clock.sleep_ms(millis)
    }

    /// The milliseconds the clock says, for a built-in that needs a
    /// number that is not the same every time.
    ///
    /// The clock is the one the host gave us: on a hosted platform it is
    /// the monotonic clock, and on a target with no `std` it is
    /// whatever the embedder supplied. It is not a wall clock, so a
    /// built-in that wants the date and the time cannot use it.
    pub(crate) fn now_ms(&self) -> u64 {
        self.clock.now_ms()
    }

    /// The hotkeys of the script.
    pub(crate) fn hotkeys(&mut self) -> &mut Vec<crate::hotkeys::Hotkey> {
        &mut self.hotkeys
    }

    /// The function the script asks to run when it ends.
    ///
    /// The name is the one the script wrote, and the line is the line of
    /// the statement that registered it.
    ///
    /// v2 keeps one entry for a callback: registering the same function
    /// again does not make it run twice.
    pub(crate) fn set_exit_callback(&mut self, name: &str, line: u32) {
        match self.exit_callbacks.iter_mut().find(|callback| callback.name.eq_ignore_ascii_case(name)) {
            Some(callback) => callback.line = line,
            None => self.exit_callbacks.push(Callback { name: name.to_string(), line }),
        }
    }

    /// The function the script asks to run when a failure is not caught.
    pub(crate) fn set_error_callback(&mut self, name: &str, line: u32) {
        match self.error_callbacks.iter_mut().find(|callback| callback.name.eq_ignore_ascii_case(name)) {
            Some(callback) => callback.line = line,
            None => self.error_callbacks.push(Callback { name: name.to_string(), line }),
        }
    }

    /// The callback of the exit that has this name is not called any more.
    pub(crate) fn remove_exit_callback(&mut self, name: &str) {
        self.exit_callbacks.retain(|callback| !callback.name.eq_ignore_ascii_case(name));
    }

    /// The callback of the error that has this name is not called any more.
    pub(crate) fn remove_error_callback(&mut self, name: &str) {
        self.error_callbacks.retain(|callback| !callback.name.eq_ignore_ascii_case(name));
    }

    /// Put a callback before the ones that are registered, or after them.
    pub(crate) fn order_exit_callback(&mut self, name: &str, before: bool) {
        let Some(at) = self.exit_callbacks.iter().position(|callback| callback.name.eq_ignore_ascii_case(name))
        else {
            return;
        };
        let callback = self.exit_callbacks.remove(at);
        if before {
            self.exit_callbacks.insert(0, callback);
        } else {
            self.exit_callbacks.push(callback);
        }
    }

    /// Put a callback before the ones that are registered, or after them.
    pub(crate) fn order_error_callback(&mut self, name: &str, before: bool) {
        let Some(at) = self.error_callbacks.iter().position(|callback| callback.name.eq_ignore_ascii_case(name))
        else {
            return;
        };
        let callback = self.error_callbacks.remove(at);
        if before {
            self.error_callbacks.insert(0, callback);
        } else {
            self.error_callbacks.push(callback);
        }
    }

    /// The function of the script that a callback names.
    ///
    /// v2 wants a function object, and the prototype of the function has
    /// to take the two parameters of the callback: the reason and the code
    /// for the exit, the value and the mode for the error.
    ///
    /// The three refusals are the ones of v2, measured on its binary: a
    /// value that is not an object, a value that cannot be called, and a
    /// prototype that does not take the two parameters.
    pub(crate) fn the_callback(
        &self,
        value: Option<&Value>,
        name: &str,
        line: u32,
    ) -> Result<Callback, RuntimeError> {
        let Some(value) = value else {
            return Err(RuntimeError {
                message: format!("Parameter #1 of {name} requires an Object, but received nothing."),
                line,
            });
        };
        // A value that is not an object is refused with the kind of the
        // value it is.
        let Value::Object(object) = value else {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    1,
                    name,
                    "an Object",
                    value,
                ),
                line,
            });
        };
        let object = object.borrow();
        // v2 calls the value, and a value that cannot be called has no
        // method named "Call".
        if object.class != "func" {
            return Err(RuntimeError {
                message: format!(
                    "This value of type \"{}\" has no method named \"Call\".",
                    value.type_name().name()
                ),
                line,
            });
        }
        let function_name = match object.value_of("name") {
            Some(Value::Text(text)) => text.clone(),
            _ => String::new(),
        };
        // The prototype has to take the two parameters of the callback. A
        // parameter that takes the rest fits: it takes what is left.
        let takes_two = match self.functions.get(&function_name.to_lowercase()) {
            Some(function) => {
                let fewest = function
                    .parameters
                    .iter()
                    .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
                    .count();
                let variadic = function.parameters.iter().any(|parameter| parameter.variadic);
                fewest <= 2 && (variadic || function.parameters.len() >= 2)
            }
            // A built-in has its own arity, and the same rule is asked of
            // it.
            None => {
                let fewest = number_of(&object.properties, "minparams");
                let most = number_of(&object.properties, "maxparams");
                fewest <= 2 && most >= 2
            }
        };
        if !takes_two {
            return Err(RuntimeError { message: "Invalid callback function.".into(), line });
        }
        Ok(Callback { name: function_name, line })
    }

    /// The functions that asked to see the failure, and what they said.
    ///
    /// The answers are the ones of v2: 0 lets the failure go on, 1 hides
    /// it, and -1 hides it and lets the thread continue.
    fn ask_the_error_callbacks(&mut self, failure: &RuntimeFailure) -> TheError {
        // While the body of a `try` runs, the functions of the error are
        // not called: the catch of the script takes the failure first, and
        // the functions of the error are called only when no catch took
        // it.
        if self.error_callbacks.is_empty() || self.try_active > 0 {
            return TheError::Goes;
        }
        let (value, mode) = match failure {
            // A failure of the runtime is a continuable one: the thread
            // goes on when a function says so.
            RuntimeFailure::Error(error) => (self.error_value(error), "Return"),
            RuntimeFailure::Detailed(error, detail) => {
                (self.error_value_with(error, detail), "Return")
            }
            // A value that the script threw is not a continuable failure.
            RuntimeFailure::Thrown(value) => (value.clone(), "Exit"),
            // The timeout of the tree stops the program.
            RuntimeFailure::Timeout(timeout) => (
                self.error_value(&RuntimeError { message: timeout.to_string(), line: 0 }),
                "ExitApp",
            ),
            _ => return TheError::Goes,
        };
        let mut answer = 0;
        let callbacks = self.error_callbacks.clone();
        for callback in callbacks {
            let values = [value.clone(), Value::Text(mode.to_string())];
            // A function that fails does not hide the failure.
            if let Ok(said) = self.call_named(&callback.name, &values, callback.line, Route::Written) {
                let said = said.to_number().unwrap_or(0.0) as i64;
                if said != 0 {
                    answer = said;
                    // A function that answered stops the ones that come
                    // after it.
                    break;
                }
            }
        }
        match answer {
            1 => TheError::Hidden,
            -1 if mode == "Return" => TheError::Continues,
            -1 => TheError::Hidden,
            _ => TheError::Goes,
        }
    }

    /// Call a function of the script by name, with no arguments.
    ///
    /// A name that is not a function of the script is a name that does
    /// nothing: the caller of this is a hotkey, and the hotkey of a
    /// function that the script does not have is a hotkey of nothing.
    pub(crate) fn call_function_by_name(&mut self, name: &str, line: u32) -> Result<(), RuntimeFailure> {
        let Some(function) = self.functions.get(&name.to_lowercase()).cloned() else {
            return Ok(());
        };
        self.call_function(&function, &[], line, Route::Written).map(|_| ())
    }

    /// The date and the time of the machine, when the clock knows them.
    pub(crate) fn wall_clock(&self) -> Option<crate::date::WallClock> {
        self.clock.wall_clock()
    }

    /// A variable of the script, by name.
    pub(crate) fn set_global(&mut self, name: &str, value: Value) {
        self.variables.insert(the_key_of(name), value);
    }

    /// A line of the report, from the script.
    pub(crate) fn push_output(&mut self, text: String) {
        self.output.push(text);
    }

    // -- expressions ---------------------------------------------------------------

    fn evaluate(&mut self, node: &Node) -> Result<Value, RuntimeFailure> {
        match node {
            Node::Assign { name, op, value, line } => {
                // The assignment of v2 is an expression, and the value of
                // it is the value that was written.
                let value = self.evaluate(value)?;
                self.write_a_variable(name, *op, value, *line)
            }
            Node::AssignProperty { target, name, op, value, line } => {
                let value = self.evaluate(value)?;
                let name = self.the_name_of_the_member(name, *line)?;
                self.write_a_property(target, &name, *op, value, *line)
            }
            Node::AssignIndex { target, index, op, value, line } => {
                let target_value = self.evaluate(target)?;
                let key = self.evaluate(index)?;
                let value = self.evaluate(value)?;
                self.set_item(&target_value, &key, *op, value, *line)
            }
            Node::Literal { value, line } => Ok(match value {
                // The shape of the literal is the shape of the value: the
                // whole number of v2 is a whole number, and a number with
                // a point or with an exponent is a float.
                LiteralValue::Integer(number) => Value::Int(*number),
                LiteralValue::Float(number) => Value::Float(*number),
                // A string is read where it is written: the names it
                // holds are the names of the scope that writes it.
                LiteralValue::Text(text) => Value::Text(self.interpolate(text, *line)?),
                LiteralValue::Bool(flag) => Value::Int(*flag as i64),
            }),
            Node::ObjectLiteral { properties, line } => {
                // The literal of an object of v2: an object of the base
                // class with the properties it writes. The names of the
                // properties do not know their case, and a name that the
                // script builds is the name of the property.
                let object = crate::classes::object("Object");
                if let Value::Object(cell) = &object {
                    for (name, value) in properties {
                        let name = self.the_name_of_the_member(name, *line)?;
                        let value = self.evaluate(value)?;
                        put(&mut cell.borrow_mut().properties, &name, value);
                    }
                }
                Ok(object)
            }
            Node::Lambda { parameters, body, line } => {
                // The arrow function of v2: a function that closes over the
                // scope where it was written, and whose body is an
                // expression.
                let function = Function {
                    name: String::new(),
                    parameters: parameters.clone(),
                    body: vec![Node::Return { value: Some(Box::new((**body).clone())), line: *line }],
                    declarations: Declarations::default(),
                    closure: self.scopes.clone(),
                };
                let name = self.make_a_closure(function.clone());
                Ok(closure_value(&name, &function))
            }
            Node::ArrayLiteral { items, line } => {
                // The literal is the array the `Array` built-in makes:
                // the values go in, in the order they are written.
                let values = self.evaluate_all(items)?;
                match crate::classes::construct("array", &values, self, *line) {
                    Some(result) => result,
                    None => Ok(Value::List(values)),
                }
            }
            Node::Group { items, .. } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the value
                // of a parenthesized list is the last of its items, and
                // every item of it runs.
                let mut value = Value::Text(String::new());
                for item in items {
                    value = self.evaluate(item)?;
                }
                Ok(value)
            }
            Node::Empty { .. } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // parameter that is not there keeps its place, and the
                // parameter has no value. The call that takes it gives it
                // the default of the parameter, or refuses the call with
                // "Missing a required parameter.".
                //
                // The value here is the one that a built-in sees, and the
                // built-in of v2 has a default of its own for it.
                Ok(Value::Text(String::new()))
            }
            Node::Legacy { items, .. } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the list
                // of v1 is evaluated, and its value is thrown away.
                let mut value = Value::Text(String::new());
                for item in items {
                    value = self.evaluate(item)?;
                }
                Ok(value)
            }
            Node::For { line, .. } => {
                // A for loop is a statement: the parser never gives one
                // where an expression is asked for.
                Err(RuntimeFailure::Error(RuntimeError {
                    message: "the for loop is not an expression.".into(),
                    line: *line,
                }))
            }
            Node::Declare { line, .. } => {
                // A declaration is a statement: the parser never gives one
                // where an expression is asked for.
                Err(RuntimeFailure::Error(RuntimeError {
                    message: "a declaration is not an expression.".into(),
                    line: *line,
                }))
            }
            Node::Super { line } => {
                // A `super` with no member: the parser refuses it, and the
                // runtime refuses it too.
                Err(RuntimeFailure::Error(RuntimeError {
                    message: "Syntax error.\n     Specifically: super".into(),
                    line: *line,
                }))
            }
            Node::Var { name, line } => match self.lookup(name) {
                Some(value) => {
                    // A class of v2 is initialised when it is first used.
                    if let Value::Object(object) = &value {
                        let class = object.borrow().class.clone();
                        self.initialise_the_class(&class, *line)?;
                    }
                    Ok(value)
                }
                // The name may be one of the built-in variables: the
                // table of them knows whether it is one, and the runtime
                // may not be able to answer it.
                None => match crate::variables::lookup(name, self) {
                    Some(Ok(value)) => Ok(value),
                    Some(Err(error)) => Err(RuntimeFailure::Error(RuntimeError {
                        message: error.message,
                        line: *line,
                    })),
                    None => match self.the_value_of_a_variable_of_the_runtime(name) {
                        Some(value) => Ok(value),
                        None => Err(match self.alias_of(name) {
                            // A parameter that is an alias says that it is
                            // the parameter that has no value.
                            Some(_) => RuntimeFailure::Error(RuntimeError {
                                message: unassigned_parameter(),
                                line: *line,
                            }),
                            None => self.the_unassigned_failure(name, *line),
                        }),
                    },
                },
            },
            Node::Unary { op, operand, line } => match op {
                // The four increments move a variable and answer a value.
                UnaryOp::PreIncrement
                | UnaryOp::PreDecrement
                | UnaryOp::PostIncrement
                | UnaryOp::PostDecrement => self.increment(*op, operand, *line),
                // `!` and `not` are the same operation: the word binds
                // looser, and the parser keeps them apart for that.
                UnaryOp::Not | UnaryOp::NotWord => {
                    let value = self.evaluate(operand)?;
                    Ok(Value::Int(!value.truthy() as i64))
                }
                // A whole number stays a whole number: `-3` is the
                // integer -3, the way AutoHotkey v2 has it.
                UnaryOp::Negate => {
                    let value = self.evaluate(operand)?;
                    Ok(match value {
                        Value::Int(number) => Value::Int(-number),
                        other => Value::Float(-self.number(&other, *line)?),
                    })
                }
                // The unary plus turns a numeric string into a number.
                UnaryOp::Plus => {
                    let value = self.evaluate(operand)?;
                    Ok(match value {
                        Value::Int(number) => Value::Int(number),
                        other => Value::Float(self.number(&other, *line)?),
                    })
                }
                // The bitwise not works on the bits of a whole number.
                UnaryOp::BitwiseNot => {
                    let value = self.evaluate(operand)?;
                    // v2 says that the bitwise not wants a number here, and
                    // not a whole one: the message is the one of the number.
                    Ok(Value::Int(!self.the_whole_number(&value, *line, TypeName::Number)?))
                }
                // The reference operator: `&x`.
                //
                // The operator is carried by the arguments of a call, and
                // it points at the variable of the scope that is running:
                // a function has variables of its own, the way v2 has
                // them.
                UnaryOp::Address => {
                    let Node::Var { name, .. } = operand.as_ref() else {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: "\"&\" requires a variable.".into(),
                            line: *line,
                        }));
                    };
                    // A built-in variable, a function and a class cannot
                    // have their reference taken, and v2 says which one
                    // of them it is.
                    if let Some(kind) = self.cannot_be_referenced(name) {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: format!(
                                "This {kind} cannot have its reference taken.\n     Specifically: {name}"
                            ),
                            line: *line,
                        }));
                    }
                    let scope = match self.scopes.len() {
                        0 => Scope::Global,
                        depth => Scope::Local(depth - 1),
                    };
                    Ok(Value::Reference(Reference { name: name.clone(), scope, alias: false }))
                }
            },
            Node::Binary { op, left, right, line } => {
                let line = *line;
                // The or-maybe is about a variable that may have no value
                // at all, so its left side is not read the way the other
                // operators read their left side.
                if *op == BinaryOp::Coalesce {
                    let Node::Var { name, .. } = left.as_ref() else {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: "the left side of ?? must be a variable".into(),
                            line,
                        }));
                    };
                    if let Some(value) = self.lookup(name) {
                        return Ok(value);
                    }
                    return self.evaluate(right);
                }
                let left_value = self.evaluate(left)?;
                match op {
                    // `and` and `or` answer the operand and not a truth:
                    // v2 hands back the last one that was true, or the
                    // first one that was not.
                    BinaryOp::And => {
                        if !left_value.truthy() {
                            return Ok(left_value);
                        }
                        return self.evaluate(right);
                    }
                    BinaryOp::Or => {
                        if left_value.truthy() {
                            return Ok(left_value);
                        }
                        return self.evaluate(right);
                    }
                    _ => {}
                }
                let right_value = self.evaluate(right)?;
                Ok(match op {
                    BinaryOp::Equal => {
                        Value::Int(values_equal(&left_value, &right_value, false) as i64)
                    }
                    BinaryOp::NotEqual => {
                        Value::Int(!values_equal(&left_value, &right_value, false) as i64)
                    }
                    BinaryOp::StrictEqual => {
                        Value::Int(values_equal(&left_value, &right_value, true) as i64)
                    }
                    BinaryOp::StrictNotEqual => {
                        Value::Int(!values_equal(&left_value, &right_value, true) as i64)
                    }
                    BinaryOp::Less => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        Value::Int((a < b) as i64)
                    }
                    BinaryOp::Greater => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        Value::Int((a > b) as i64)
                    }
                    BinaryOp::LessEqual => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        Value::Int((a <= b) as i64)
                    }
                    BinaryOp::GreaterEqual => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        Value::Int((a >= b) as i64)
                    }
                    BinaryOp::Add => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        match (&left_value, &right_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                            _ => Value::Float(a + b),
                        }
                    }
                    BinaryOp::Subtract => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        match (&left_value, &right_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                            _ => Value::Float(a - b),
                        }
                    }
                    BinaryOp::Multiply => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        match (&left_value, &right_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                            _ => Value::Float(a * b),
                        }
                    }
                    BinaryOp::Divide => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        if b == 0.0 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "Divide by zero.".into(),
                                line,
                            }));
                        }
                        Value::Float(a / b)
                    }
                    BinaryOp::IntegerDivide => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.integer(&right_value, line)?;
                        if b == 0 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "Divide by zero.".into(),
                                line,
                            }));
                        }
                        Value::Int(a / b)
                    }
                    BinaryOp::Power => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        // Two whole numbers with a whole exponent give a
                        // whole number; a negative exponent gives a float.
                        if let (Value::Int(base), Value::Int(exponent)) = (&left_value, &right_value) {
                            if *exponent >= 0 && *exponent <= 63 {
                                if let Some(power) = base.checked_pow(*exponent as u32) {
                                    return Ok(Value::Int(power));
                                }
                            }
                        }
                        Value::Float(powf(a, b))
                    }
                    BinaryOp::BitwiseAnd => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.integer(&right_value, line)?;
                        Value::Int(a & b)
                    }
                    BinaryOp::BitwiseOr => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.integer(&right_value, line)?;
                        Value::Int(a | b)
                    }
                    BinaryOp::BitwiseXor => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.integer(&right_value, line)?;
                        Value::Int(a ^ b)
                    }
                    BinaryOp::ShiftLeft => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.shift(&right_value, line)?;
                        Value::Int(a.wrapping_shl(b))
                    }
                    BinaryOp::ShiftRight => {
                        let a = self.integer(&left_value, line)?;
                        let b = self.shift(&right_value, line)?;
                        Value::Int(a.wrapping_shr(b))
                    }
                    BinaryOp::ShiftRightUnsigned => {
                        let a = self.integer(&left_value, line)? as u64;
                        let b = self.shift(&right_value, line)?;
                        Value::Int((a >> b) as i64)
                    }
                    BinaryOp::Concat => {
                        let left_text = self.the_text_of_the_concat(&left_value, line)?;
                        let right_text = self.the_text_of_the_concat(&right_value, line)?;
                        Value::Text(format!("{left_text}{right_text}"))
                    }
                    BinaryOp::RegexMatch => {
                        let text = left_value.to_text();
                        let pattern = right_value.to_text();
                        match crate::regex::find(&text, &pattern) {
                            Ok(Some(position)) => Value::Int(position as i64),
                            Ok(None) => Value::Int(0),
                            Err(reason) => {
                                return Err(RuntimeFailure::Error(RuntimeError { message: reason, line }))
                            }
                        }
                    }
                    BinaryOp::Is => {
                        // The right side has to be a class, and v2 says so
                        // when it is not.
                        //
                        // Measured on the binary of v2.0.19 on 2026-10-31:
                        // the value that arrived goes in the `Extra` of the
                        // failure, and the `What` is empty.
                        let Some(class) = self.class_name_of(&right_value) else {
                            return Err(RuntimeFailure::Detailed(
                                RuntimeError {
                                    message: expected_type(TypeName::Class, right_value.type_name()),
                                    line,
                                },
                                Detail {
                                    what: String::new(),
                                    extra: the_extra_of_the_value(&right_value),
                                },
                            ));
                        };
                        let answer = match &left_value {
                            Value::Object(object) => {
                                let name = object.borrow().class.clone();
                                // Every object is an Object: it is the root
                                // of the chain of the classes.
                                class == "object" || self.class_chain(&name).contains(&class)
                            }
                            // A number and a text are not objects.
                            _ => false,
                        };
                        Value::Int(answer as i64)
                    }
                    BinaryOp::And | BinaryOp::Or | BinaryOp::Coalesce => {
                        unreachable!("handled above")
                    }
                })
            }
            Node::Call { name, arguments, line } => self.call(name, arguments, *line),
            Node::MethodCall { target, name, arguments, line } => {
                // `super.m()`: the method of the base.
                if let Node::Super { .. } = &**target {
                    let name = self.the_name_of_the_member(name, *line)?;
                    return self.call_the_method_of_the_base(&name, arguments, *line);
                }
                let target_value = self.evaluate(target)?;
                let name = self.the_name_of_the_member(name, *line)?;
                self.call_method_on(&target_value, &name, arguments, *line)
            }
            Node::Property { target, name, line } => {
                // `super.value`: the property of the base.
                if let Node::Super { .. } = &**target {
                    let name = self.the_name_of_the_member(name, *line)?;
                    return self.read_the_property_of_the_base(&name, *line);
                }
                let target_value = self.evaluate(target)?;
                let name = self.the_name_of_the_member(name, *line)?;
                self.read_property(&target_value, &name, *line)
            }
            Node::Index { target, index, line } => {
                // `super["value"]`: the item of the base.
                if let Node::Super { .. } = &**target {
                    let key = self.evaluate(index)?;
                    return self.read_the_item_of_the_base(&key, *line);
                }
                let key = self.evaluate(index)?;
                // `x.y[z]`: the `__Get` of `x.y` is asked with the
                // parameters of the access, and the value it answers is
                // the one that is indexed.
                if let Node::Property { target: object, name, line: property_line } = &**target {
                    let object_value = self.evaluate(object)?;
                    let name = self.the_name_of_the_member(name, *property_line)?;
                    let (value, by_meta) =
                        self.read_property_fully(
                            &object_value,
                            &name,
                            core::slice::from_ref(&key),
                            *property_line,
                        )?;
                    // The meta-function of a class handles the whole
                    // access: the brackets are its parameters and its
                    // answer is the answer.
                    if by_meta {
                        return Ok(value);
                    }
                    return self.index_a_value(&value, &key, *line);
                }
                let target = self.evaluate(target)?;
                self.index_a_value(&target, &key, *line)
            }
            Node::Ternary { condition, then, otherwise, .. } => {
                if self.evaluate(condition)?.truthy() {
                    self.evaluate(then)
                } else {
                    self.evaluate(otherwise)
                }
            }
            Node::DynamicName { pieces, line } => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of the variable is the one that the pieces make, and a name
                // that is not a variable fails with "Variable not found." and
                // the name in the `Extra`.
                let name = self.the_name_of_the_pieces(pieces, *line)?;
                match self.the_variable_of_the_name(&name, *line)? {
                    Some(value) => Ok(value),
                    None => Err(RuntimeFailure::Detailed(
                        RuntimeError { message: "Variable not found.".to_string(), line: *line },
                        Detail { what: String::new(), extra: name },
                    )),
                }
            }
            Node::AssignDynamic { name, op, value, line } => {
                let name = self.the_name_of_the_pieces(name, *line)?;
                let value = self.evaluate(value)?;
                self.write_a_dynamic_name(&name, *op, value, *line)?;
                Ok(Value::Text(String::new()))
            }
            Node::Expression { .. }
            | Node::If { .. }
            | Node::Switch { .. }
            | Node::While { .. }
            | Node::Try { .. }
            | Node::Throw { .. }
            | Node::Loop { .. }
            | Node::Break { .. }
            | Node::Continue { .. }
            | Node::Label { .. }
            | Node::Goto { .. }
            | Node::Return { .. }
            | Node::ExitApp { .. }
            | Node::FunctionDef { .. }
            | Node::Directive { .. }
            | Node::ClassDef { .. }
            => Err(RuntimeFailure::Error(RuntimeError {
                message: "this is a statement, not an expression".into(),
                line: node.line(),
            })),
        }
    }

    /// Replace `%name%` with the value of the name.
    ///
    /// AutoHotkey's way of putting a variable in a string. A `%` that is
    /// not a complete `%name%` is left alone, so a path with a percent
    /// sign in it survives. A name that is not set is left alone too.
    ///
    /// The name may be a path: `%object.property%` is the property of
    /// the object, and the path may go deeper. The string is read where
    /// it is written, so the name that answers is the name of the scope
    /// that is writing it.
    ///
    /// A name that the runtime cannot answer fails the statement with
    /// the reason, the way the bare name does.
    pub(crate) fn interpolate(&mut self, text: &str, line: u32) -> Result<String, RuntimeError> {
        if !text.contains('%') {
            return Ok(text.to_string());
        }
        let mut out = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let mut index = 0usize;
        while index < chars.len() {
            if chars[index] == '%' {
                if let Some(offset) = chars[index + 1..].iter().position(|c| *c == '%') {
                    let close = index + 1 + offset;
                    let reference: String = chars[index + 1..close].iter().collect();
                    if let Some(value) = self.resolve_reference(reference.trim(), line)? {
                        let text = value.as_text().map_err(|got| RuntimeError {
                            message: expected_type(TypeName::String, got),
                            line,
                        })?;
                        out.push_str(&text);
                        index = close + 1;
                        continue;
                    }
                    out.push_str(&chars[index..=close].iter().collect::<String>());
                    index = close + 1;
                    continue;
                }
                out.extend(&chars[index..]);
                break;
            }
            out.push(chars[index]);
            index += 1;
        }
        Ok(out)
    }

    /// Write an item of a value: `obj[key] := value`.
    ///
    /// The write goes to the class that owns the value: the map and the
    /// array know how to write an item of theirs. A class of the script
    /// has no way to write one, and v2.0.19 asks nothing else of it:
    /// the meta-functions answer for the properties that are not there,
    /// and `x[y]` asks only the `__Item` property.
    fn set_item(
        &mut self,
        target: &Value,
        key: &Value,
        op: AssignOp,
        value: Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Value::Object(object) = target else {
            return Err(self.no_item(target, line));
        };
        let class = object.borrow().class.clone();
        // The compound operators read what is there before they write.
        let value = match op {
            AssignOp::Set => value,
            other => {
                let current = self.read_item(target, key, line)?;
                self.apply_assign(other, "the item", Some(current), value, line)?
            }
        };
        match crate::classes::set_item(&class, key, target, value.clone(), self, line) {
            Some(result) => {
                result?;
                Ok(value)
            }
            None => Err(self.no_item(target, line)),
        }
    }

    /// The value of an item of an object: `obj[key]`.
    fn read_item(
        &mut self,
        target: &Value,
        key: &Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Value::Object(object) = target else {
            return Err(self.no_item(target, line));
        };
        let class = object.borrow().class.clone();
        // The class of the script first, and then the class types of the
        // runtime.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the item of an
        // object of a class of the script is the property `__Item` of it,
        // and the getter of it takes the index.
        if let Some(access) = self.the_accessor_of(target, "__Item") {
            let Some(getter) = &access.getter else {
                return Err(self.no_item(target, line));
            };
            if access.parameters.is_empty() {
                // The index goes to the `__Item` of the value that the
                // getter answers.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `obj["a"]` is refused with 'This value of type "Integer"
                // has no property named "__Item".' when the getter answers
                // a number.
                let value = self.call_the_body_of_an_accessor(target, &access, getter, line, &[], None)?;
                return self.read_item(&value, key, line);
            }
            // The parameters of the property are the ones of the index.
            let value =
                self.call_the_body_of_an_accessor(target, &access, getter, line, core::slice::from_ref(key), None)?;
            return Ok(value);
        }
        // A method of that name is not the item of the object: the item is
        // the method itself.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `obj[3]` is a
        // function whose name is "C.Prototype.__Item" when the class
        // declares a method of that name.
        if let Some((defining, method)) = self.find_method_in(&class, "__item") {
            return Ok(self.method_reference(&defining, &method));
        }
        if let Some(result) = crate::classes::item(&class, key, target, self, line) {
            return result;
        }
        // The `__Item` of the class is the method behind the index syntax,
        // and a class that has one answers it.
        if let Some(result) = crate::classes::call(
            &class,
            "__item",
            target,
            core::slice::from_ref(key),
            self,
            line,
        ) {
            return result;
        }
        Err(self.no_item(target, line))
    }

    /// The value of a reference: a name, or a path of names.
    ///
    /// `p.x` is the property `x` of the object `p`, and the path may go
    /// deeper. A name that is not set has no value, and a path that
    /// breaks has none either: the text is left alone, the way
    /// AutoHotkey leaves it alone.
    ///
    /// A name that is one of the built-in variables is answered by the
    /// table of them, and one that the runtime cannot answer fails with
    /// the reason.
    fn resolve_reference(
        &mut self,
        reference: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeError> {        let mut parts = reference.split('.');
        let Some(name) = parts.next() else { return Ok(None) };
        let found = self.lookup(name);
        let mut value = match found {
            Some(value) => value,
            // The name may be one of the built-in variables: the table of
            // them knows whether it is one, and the runtime may not be
            // able to answer it.
            None => match crate::variables::lookup(name, self) {
                Some(Ok(value)) => value,
                Some(Err(error)) => {
                    return Err(RuntimeError { message: error.message, line });
                }
                // A name that is not set is left alone.
                None => return Ok(None),
            },
        };
        for part in parts {
            let Ok(next) = self.read_property(&value, part, line) else {
                return Ok(None);
            };
            value = next;
        }
        Ok(Some(value))
    }

    /// The name that the pieces of the script make.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the value of a
    /// piece is a text or a reference, and a value of another kind is
    /// refused with the kind of it.
    fn the_name_of_the_pieces(
        &mut self,
        pieces: &[DynamicPiece],
        line: u32,
    ) -> Result<String, RuntimeFailure> {
        let mut name = String::new();
        for piece in pieces {
            match piece {
                DynamicPiece::Text(text) => name.push_str(text),
                DynamicPiece::Value(value) => {
                    let value = self.evaluate(value)?;
                    match &value {
                        Value::Text(text) => name.push_str(text),
                        Value::Int(_) | Value::Float(_) => name.push_str(&value.to_text()),
                        Value::Reference(reference) => name.push_str(&reference.name),
                        other => {
                            let kind = other.the_kind();
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: format!(
                                    "Expected a String or VarRef but got a{} {}.",
                                    the_article_of(&kind),
                                    kind
                                ),
                                line,
                            }));
                        }
                    }
                }
            }
        }
        Ok(name)
    }

    /// The name of a member, when the script builds it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// member is the text that is written and the values of the
    /// expressions, and `m.%x%` is the member whose name is the value of
    /// `x`.
    fn the_name_of_the_member(
        &mut self,
        name: &MemberName,
        line: u32,
    ) -> Result<String, RuntimeFailure> {
        match name {
            MemberName::Text(text) => Ok(text.clone()),
            MemberName::Dynamic(pieces) => self.the_name_of_the_pieces(pieces, line),
        }
    }

    /// The value of the variable of that name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    /// dynamic variable reference is the name of a variable, and not a path
    /// of names: `%m.a%` is the variable `m.a`, and it is not there. The
    /// name is looked up the way a name that is written is looked up: a
    /// function of the script and a class are values too.
    fn the_variable_of_the_name(
        &mut self,
        name: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        if let Some(value) = self.lookup(name) {
            return Ok(Some(value));
        }
        match crate::variables::lookup(name, self) {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(error)) => {
                Err(RuntimeFailure::Error(RuntimeError { message: error.message, line }))
            }
            None => Ok(None),
        }
    }

    /// The write of a name that the script builds.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the variable has to
    /// be there, and a function that writes a name of the script says that
    /// it wants a global declaration.
    fn write_a_dynamic_name(
        &mut self,
        name: &str,
        op: AssignOp,
        value: Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let key = the_key_of(name);
        // A dynamic reference does not create a variable: the one that is
        // written has to be there.
        let exists = self.lookup_variable(name).is_some() || self.variables.contains_key(&key);
        if !exists {
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message: "Variable not found.".to_string(), line },
                Detail { what: String::new(), extra: name.to_string() },
            ));
        }
        // A function that writes a name of the script says that it wants a
        // global declaration: the name is not a local of the function.
        let is_local = self
            .scopes
            .last()
            .map(|scope| scope.borrow().contains_key(&key))
            .unwrap_or(false)
            || self.is_declared_local(&key);
        if self.declarations.last().is_some() && !is_local && !self.lives_in_the_script(&key) {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "This dynamic assignment requires a \"global\" declaration.".to_string(),
                    line,
                },
                Detail { what: String::new(), extra: name.to_string() },
            ));
        }
        // The compound operators read the variable before they write it.
        let current = self.lookup_variable(name);
        let stored = self.apply_assign(op, name, current, value, line)?;
        self.assign(name, stored.clone());
        Ok(stored)
    }
}

// -- the arithmetic -------------------------------------------------------------
//
// `f64::sqrt` and its neighbours live in `std`: they call the platform's
// libm. A target with no `std` gets the same numbers from `libm`, a
// pure-Rust implementation, so the language answers the same on both.

/// The square root of `value`.
#[cfg(feature = "std")]
pub(crate) fn sqrt(value: f64) -> f64 {
    value.sqrt()
}

/// The square root of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn sqrt(value: f64) -> f64 {
    libm::sqrt(value)
}

/// The largest whole number that is not greater than `value`.
#[cfg(feature = "std")]
pub(crate) fn floor(value: f64) -> f64 {
    value.floor()
}

/// The largest whole number that is not greater than `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn floor(value: f64) -> f64 {
    libm::floor(value)
}

/// The smallest whole number that is not less than `value`.
#[cfg(feature = "std")]
pub(crate) fn ceil(value: f64) -> f64 {
    value.ceil()
}

/// The smallest whole number that is not less than `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn ceil(value: f64) -> f64 {
    libm::ceil(value)
}

/// `value` rounded to the nearest whole number, halves away from zero.
#[cfg(feature = "std")]
pub(crate) fn round(value: f64) -> f64 {
    value.round()
}

/// `value` rounded to the nearest whole number, halves away from zero.
#[cfg(not(feature = "std"))]
pub(crate) fn round(value: f64) -> f64 {
    libm::round(value)
}

/// The power of two numbers.
///
/// The same shape as the square root: `std` has it, and `libm` has it
/// for a target that has no `std`.
#[cfg(feature = "std")]
pub(crate) fn powf(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// The power of two numbers.
#[cfg(not(feature = "std"))]
pub(crate) fn powf(base: f64, exponent: f64) -> f64 {
    libm::pow(base, exponent)
}

/// The arc cosine of `value`.
#[cfg(feature = "std")]
pub(crate) fn acos(value: f64) -> f64 {
    value.acos()
}

/// The arc cosine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn acos(value: f64) -> f64 {
    libm::acos(value)
}

/// The arc sine of `value`.
#[cfg(feature = "std")]
pub(crate) fn asin(value: f64) -> f64 {
    value.asin()
}

/// The arc sine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn asin(value: f64) -> f64 {
    libm::asin(value)
}

/// The arc tangent of `value`.
#[cfg(feature = "std")]
pub(crate) fn atan(value: f64) -> f64 {
    value.atan()
}

/// The arc tangent of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn atan(value: f64) -> f64 {
    libm::atan(value)
}

/// The cosine of `value`.
#[cfg(feature = "std")]
pub(crate) fn cos(value: f64) -> f64 {
    value.cos()
}

/// The cosine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn cos(value: f64) -> f64 {
    libm::cos(value)
}

/// The sine of `value`.
#[cfg(feature = "std")]
pub(crate) fn sin(value: f64) -> f64 {
    value.sin()
}

/// The sine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn sin(value: f64) -> f64 {
    libm::sin(value)
}

/// The tangent of `value`.
#[cfg(feature = "std")]
pub(crate) fn tan(value: f64) -> f64 {
    value.tan()
}

/// The tangent of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn tan(value: f64) -> f64 {
    libm::tan(value)
}

/// The exponential of `value`.
#[cfg(feature = "std")]
pub(crate) fn exp(value: f64) -> f64 {
    value.exp()
}

/// The exponential of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn exp(value: f64) -> f64 {
    libm::exp(value)
}

/// The natural logarithm of `value`.
#[cfg(feature = "std")]
pub(crate) fn ln(value: f64) -> f64 {
    value.ln()
}

/// The natural logarithm of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn ln(value: f64) -> f64 {
    libm::log(value)
}

/// The logarithm of `value` in base ten.
#[cfg(feature = "std")]
pub(crate) fn log10(value: f64) -> f64 {
    value.log10()
}

/// The logarithm of `value` in base ten.
#[cfg(not(feature = "std"))]
pub(crate) fn log10(value: f64) -> f64 {
    libm::log10(value)
}

/// The fractional part of `value`: it keeps the sign of the value.
#[cfg(feature = "std")]
pub(crate) fn fract(value: f64) -> f64 {
    value.fract()
}

/// The fractional part of `value`: it keeps the sign of the value.
#[cfg(not(feature = "std"))]
pub(crate) fn fract(value: f64) -> f64 {
    value - libm::trunc(value)
}

/// The whole part of `value`: the number without its fraction.
#[cfg(feature = "std")]
fn trunc(value: f64) -> f64 {
    value.trunc()
}

/// The whole part of `value`: the number without its fraction.
#[cfg(not(feature = "std"))]
fn trunc(value: f64) -> f64 {
    libm::trunc(value)
}

/// `value` raised to a whole power.
///
/// The loop is the same in both configurations, so the two answers agree
/// to the last bit: the dialect cannot depend on the target.
pub(crate) fn powi(value: f64, exponent: i32) -> f64 {
    let mut result = 1.0f64;
    let mut factor = value;
    let mut rest = exponent.unsigned_abs();
    while rest > 0 {
        if rest & 1 == 1 {
            result *= factor;
        }
        factor *= factor;
        rest >>= 1;
    }
    if exponent < 0 {
        1.0 / result
    } else {
        result
    }
}

/// The value of a function: an object of the class `Func`.
///
/// A function in a script is a value, and `Func` is the class of it.
fn func_value(name: &str, function: &Function) -> Value {
    // The fewest parameters are the ones that have no default, and the
    // most are all of them.
    let fewest = function
        .parameters
        .iter()
        .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
        .count();
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, "name", Value::Text(name.to_string()));
    put(&mut properties, "minparams", Value::Int(fewest as i64));
    put(&mut properties, "maxparams", Value::Int(function.parameters.len() as i64));
    put(&mut properties, "isbuiltin", Value::Int(0));
    put(&mut properties, "isvariadic", Value::Int(0));
    Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
}

/// The value of a closure: an object of the class `Func`.
///
/// The name of a closure of v2 is empty, and the name under which the
/// runtime keeps it is in the object and not in the name.
fn closure_value(name: &str, function: &Function) -> Value {
    let value = func_value("", function);
    if let Value::Object(object) = &value {
        object.borrow_mut().put(CLOSURE, Value::Text(name.to_string()));
    }
    value
}

/// The value of a built-in: an object of the class `Func`.
///
/// A built-in is a function of the runtime, and it is a value like any
/// other: it can be held in a variable and called through it.
fn builtin_value(name: &str, min: u8, max: u8) -> Value {
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, "name", Value::Text(name.to_string()));
    put(&mut properties, "minparams", Value::Int(min as i64));
    put(&mut properties, "maxparams", Value::Int(max as i64));
    put(&mut properties, "isbuiltin", Value::Int(1));
    put(&mut properties, "isvariadic", Value::Int(0));
    Value::Object(Rc::new(RefCell::new(Object { class: "func".to_string(), properties })))
}

/// The value of a class: an object of the class itself.
fn class_value(name: &str, class: &Class) -> Value {
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, CLASS_NAME, Value::Text(class.name.clone()));
    put(&mut properties, "prototype", prototype_value(&class.name));
    Value::Object(Rc::new(RefCell::new(Object { class: name.to_lowercase(), properties })))
}

/// The value of the parameter that a failure names.
///
/// v2 carries the value that a built-in complained about in the `Extra`
/// of the error: the message names the parameter, and the value is the
/// one that was given to it.
fn the_parameter_that_failed(message: &str, values: &[Value]) -> String {
    let Some(rest) = message.strip_prefix("Parameter #") else {
        return String::new();
    };
    let Some((number, _)) = rest.split_once(" of ") else {
        return String::new();
    };
    let Ok(number) = number.parse::<usize>() else {
        return String::new();
    };
    values
        .get(number.saturating_sub(1))
        .map(the_extra_of_the_value)
        .unwrap_or_default()
}

/// The value in the `Extra` of a failure.
///
/// v2 carries the value that arrived, and a text and a number have a text
/// of their own: an object, an array and a function have none.
pub(crate) fn the_extra_of_the_value(value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Int(_) | Value::Float(_) => value.to_text(),
        Value::List(_) | Value::Object(_) | Value::Reference(_) => String::new(),
    }
}

/// The prototype of a class: the object that holds its methods.
///
/// v2 gives every class one, and the base of an instance is the prototype
/// of its class.
pub(crate) fn prototype_value(name: &str) -> Value {
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, CLASS_NAME, Value::Text(name.to_string()));
    Value::Object(Rc::new(RefCell::new(Object {
        class: "prototype".to_string(),
        properties,
    })))
}

/// The failure of the runtime, as the failure of a built-in.
///
/// A built-in answers with a message and a line, and not with the flow
/// of the script: a timeout or a return is not a thing a built-in can
/// hand back.
pub(crate) fn as_error(failure: RuntimeFailure, line: u32) -> RuntimeError {
    match failure {
        RuntimeFailure::Error(error) => error,
        RuntimeFailure::Detailed(error, _) => error,
        _ => RuntimeError { message: "the runtime cannot answer that.".into(), line },
    }
}

/// The pieces of a text, cut at every character of the delimiters.
///
/// The pieces keep the order they had, and an empty piece is a piece:
/// the text "a,,b" has three.
pub(crate) fn split_on_any(text: &str, delimiters: &str) -> Vec<String> {
    let mut pieces: Vec<String> = Vec::new();
    let mut current = String::new();
    for character in text.chars() {
        if delimiters.contains(character) {
            pieces.push(core::mem::take(&mut current));
            continue;
        }
        current.push(character);
    }
    pieces.push(current);
    pieces
}

/// Whether two values are equal, the way a script expects.
///
/// A number that came out of an addition is compared with a number that
/// was written down: `2` and `2.0` are the same number, even though one
/// of them is a whole number and the other is not.
fn values_equal(left: &Value, right: &Value, case_sensitive: bool) -> bool {
    // An object is the same object and nothing else.
    //
    // Measured on the binary of v2.0.19 on 2026-10-31: two maps that hold
    // the same entries are not the same map, an instance is not the class
    // of it, and an object is never equal to a value that is not an
    // object -- not even to a text that carries the name of its class.
    //
    // This is a rule of v2: the v1 dialect compares the objects by what
    // they hold, and it bypasses this place.
    if let (Value::Object(one), Value::Object(other)) = (left, right) {
        return Rc::ptr_eq(one, other);
    }
    if matches!(left, Value::Object(_)) || matches!(right, Value::Object(_)) {
        return false;
    }
    // The comparison is a number when both sides are numbers, and a
    // text otherwise: that is the rule v2 follows.
    if let (Some(a), Some(b)) = (left.to_number(), right.to_number()) {
        return a == b;
    }
    let left = left.to_text();
    let right = right.to_text();
    if case_sensitive {
        left == right
    } else {
        left.eq_ignore_ascii_case(&right)
    }
}


/// The message of a failure, the way the report carries it.
fn the_message_of(failure: &RuntimeFailure) -> String {
    match failure {
        RuntimeFailure::Error(error) => error.message.clone(),
        RuntimeFailure::Detailed(error, _) => error.message.clone(),
        RuntimeFailure::Fatal(error) => error.message.clone(),
        RuntimeFailure::Thrown(value) => thrown_message(value),
        RuntimeFailure::Timeout(timeout) => timeout.to_string(),
        RuntimeFailure::Flow(_) => "the script left a block in the middle of it.".to_string(),
    }
}

/// The number of a property of an object, when it has one.
fn number_of(properties: &BTreeMap<String, Property>, name: &str) -> i64 {
    properties.get(name).map(|property| property.value.to_number().unwrap_or(0.0) as i64).unwrap_or(0)
}

/// The key of a static declaration: the line it is on and the name it
/// declares.
///
/// The value of a static declaration runs once, and the key is what tells
/// the tree that it ran.
fn the_key_of_a_declaration(line: u32, name: &str) -> String {
    format!("{}:{}", line, the_key_of(name))
}

/// The key of a variable, as the tree keeps it.
///
/// The dialect does not know the case of the names: `x` and `X` are the
/// same variable. The names of the functions were in lower case already;
/// the variables are like them now.
fn the_key_of(name: &str) -> String {
    name.to_lowercase()
}

/// The message of a value that was thrown and that nobody caught.
///
/// A text is the message of the exception, the way v2 shows it, and
/// anything else is the message of an exception without a name.
fn thrown_message(value: &Value) -> String {
    match value {
        Value::Text(text) => format!("Unhandled exception.\n     Specifically: {text}"),
        _ => "Unhandled exception.".to_string(),
    }
}

/// The place of the label in a block, when the block holds it.
///
/// The names of the labels do not know their case, the way the names of the
/// variables do not.
fn the_place_of_the_label(statements: &[Node], name: &str) -> Option<usize> {
    statements.iter().position(|statement| match statement {
        Node::Label { name: label, .. } => label.eq_ignore_ascii_case(name),
        _ => false,
    })
}

/// The labels of a scope, the way the script wrote them.
///
/// The labels of a function are the ones of its body, and the body of a
/// function that is written inside another one belongs to the function it
/// is written in.
fn the_labels_of(body: &[Node]) -> Vec<String> {
    let mut labels = Vec::new();
    collect_the_labels(body, &mut labels);
    labels
}

/// The labels of the blocks of a body.
fn collect_the_labels(body: &[Node], labels: &mut Vec<String>) {
    for statement in body {
        match statement {
            Node::Label { name, .. } => labels.push(name.clone()),
            Node::If { branches, otherwise, .. } => {
                for branch in branches {
                    collect_the_labels(&branch.body, labels);
                }
                collect_the_labels(otherwise, labels);
            }
            Node::Switch { cases, .. } => {
                for case in cases {
                    collect_the_labels(&case.body, labels);
                }
            }
            Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
                collect_the_labels(body, labels);
            }
            Node::Try { body, catches, otherwise, finally, .. } => {
                collect_the_labels(body, labels);
                for catch in catches {
                    collect_the_labels(&catch.body, labels);
                }
                collect_the_labels(otherwise, labels);
                collect_the_labels(finally, labels);
            }
            _ => {}
        }
    }
}

/// The name a statement is reported under.
pub fn op_name(statement: &Node) -> &str {
    match statement {
        Node::Assign { .. } => "Assign",
        Node::AssignIndex { .. } => "AssignIndex",
        // A call to a built-in is reported under the name of the built-in,
        // the way AutoHotkey reports a command.
        Node::Call { name, .. } => match crate::builtins::find(name) {
            Some(builtin) => builtin.name,
            None => "Call",
        },
        Node::Expression { value, .. } => match the_call_of(value) {
            // The statement reads the member of the answer of a call, and
            // it is still the call that the report names.
            Some(name) => match crate::builtins::find(name) {
                Some(builtin) => builtin.name,
                None => "Call",
            },
            None => "Expression",
        },
        Node::If { .. } => "If",
        Node::Switch { .. } => "Switch",
        Node::While { .. } => "While",
        Node::Try { .. } => "Try",
        Node::Throw { .. } => "Throw",
        Node::Declare { .. } => "Declare",
        Node::Super { .. } => "Super",
        Node::Loop { .. } => "Loop",
        Node::ArrayLiteral { .. } => "ArrayLiteral",
        Node::ObjectLiteral { .. } => "ObjectLiteral",
        Node::Lambda { .. } => "Lambda",
        Node::For { .. } => "For",
        Node::Directive { name, .. } => name,
        Node::Break { .. } => "Break",
        Node::Continue { .. } => "Continue",
        Node::Label { .. } => "Label",
        Node::Goto { .. } => "Goto",
        Node::Return { .. } => "Return",
        Node::ExitApp { .. } => "ExitApp",
        Node::FunctionDef { .. } => "FunctionDef",
        Node::ClassDef { .. } => "ClassDef",
        Node::MethodCall { .. } => "MethodCall",
        Node::Property { .. } => "Property",
        Node::AssignProperty { .. } => "AssignProperty",
        Node::Literal { .. } => "Literal",
        Node::Var { .. } => "Var",
        Node::Unary { .. } => "Unary",
        Node::Binary { .. } => "Binary",
        Node::Ternary { .. } => "Ternary",
        Node::DynamicName { .. } => "DynamicName",
        Node::AssignDynamic { .. } => "AssignDynamic",
        Node::Index { .. } => "Index",
        Node::Group { .. } => "Group",
        Node::Empty { .. } => "Empty",
        Node::Legacy { .. } => "Legacy",
    }
}

/// The call that a value holds, when it holds one.
///
/// A statement that reads the member of the answer of a call is the call
/// itself: `Map().Count` is the call of `Map` and the reading of a member.
fn the_call_of(node: &Node) -> Option<&str> {
    match node {
        Node::Call { name, .. } => Some(name),
        Node::Property { target, .. }
        | Node::Index { target, .. }
        | Node::MethodCall { target, .. } => the_call_of(target),
        _ => None,
    }
}

pub(crate) enum RuntimeFailure {
    Error(RuntimeError),
    /// A failure with the detail that v2 carries in the error object.
    ///
    /// `What` names the thing that raised the failure, and `Extra` is
    /// the detail that goes with it.
    Detailed(RuntimeError, Detail),
    /// A failure that no catch takes.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the failure of a
    /// `Goto` that does not find its label ends the script, and a catch
    /// does not take it.
    Fatal(RuntimeError),
    /// A value that the script threw.
    Thrown(Value),
    Timeout(Timeout),
    Flow(Flow),
}

/// The name of the thing that raised a failure, and the detail that
/// goes with it.
///
/// v2 carries both in the error object: `What` names the thing that
/// raised the failure, and `Extra` is the detail.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Detail {
    /// The name of the thing that raised the failure.
    pub what: String,
    /// The detail of the failure.
    pub extra: String,
}

/// How a call found the function it calls.
///
/// v2 checks the number of the arguments in two places. A call that the
/// script writes is checked when the script loads, and the failure says
/// the name of the function: "Too few parameters passed to function.
/// \n     Specifically: Abs". A call that goes through a value is checked when
/// the call runs, and the failure is the one of the run: "Missing a
/// required parameter." for a function of the script, and "Too few
/// parameters passed to function." for a built-in.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Route {
    /// The script wrote the call, and the name of the function is known
    /// when the script loads.
    Written,
    /// The call goes through a value, and the name of the function is
    /// not known when the script loads.
    ThroughValue,
    /// The call went through the `Call` method of a value of a function.
    ///
    /// v2 names that method in the `What` of the failure of the number
    /// of the arguments, and the call of the value itself does not.
    ThroughTheCall,
}

impl From<RuntimeError> for RuntimeFailure {
    fn from(error: RuntimeError) -> Self {
        RuntimeFailure::Error(error)
    }
}

impl From<Timeout> for RuntimeFailure {
    fn from(timeout: Timeout) -> Self {
        RuntimeFailure::Timeout(timeout)
    }
}

impl From<Flow> for RuntimeFailure {
    fn from(flow: Flow) -> Self {
        RuntimeFailure::Flow(flow)
    }
}

impl From<String> for RuntimeFailure {
    fn from(message: String) -> Self {
        RuntimeFailure::Error(RuntimeError { message, line: 0 })
    }
}
