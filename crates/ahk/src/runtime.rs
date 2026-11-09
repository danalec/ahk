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

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;


use alloc::rc::Rc;
use core::cell::RefCell;

use crate::ast::{
    AccessBody, AccessDef, AssignOp, BinaryOp, Declarations, DeclareKind, DynamicPiece, LegacyForm,
    LiteralValue, LoopKind, MemberName, MethodDef, Node, ObjectItem, Parameter, PropertyDef,
    UnaryOp,
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

/// The mark of a value that has no value of its own.
///
/// The dialect has no value that means "nothing", and the tree carries the
/// mark of it: a list of one number, and the number is zero. The mark is not
/// a value that a script can hold, and the place of an array that has no
/// value of its own carries it.
pub(crate) fn the_mark_of_a_value_without_one() -> Value {
    Value::List(vec![Value::Int(0)])
}

/// Whether a value is the mark of a value that has no value of its own.
pub(crate) fn is_a_value_without_one(value: &Value) -> bool {
    matches!(value, Value::List(items) if items.len() == 1 && items[0] == Value::Int(0))
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
pub fn wants_a_reference(index: usize, name: &str, wants: &str, value: &Value) -> String {
    format!(
        "Parameter #{index} of {name} requires {wants}, but received {}.",
        the_kind_of(value)
    )
}

/// The reason the runtime refuses the directive of this name, when it does.
///
/// The directive is answered at the run, and the reason is the one of the
/// category of the directive: the hotkeys, the memory limits, the comment
/// flags. A directive that the runtime carries answers with `None`.
///
/// The surface asks this question, and the answer is the one that the run
/// gives: the table of the names and the run are the same truth.
pub fn refusal_of_directive(name: &str) -> Option<&'static str> {
    match name {
        "commentflag" | "escapechar" => Some("the comment flags are not carried by this runtime."),
        "maxmem" => Some("the memory limits are not carried by this runtime."),
        "if"
        | "iftimeout"
        | "ifwinactive"
        | "installkeybdhook"
        | "installmousehook"
        | "keyhistory"
        | "maxhotkeysperinterval"
        | "hotkeyinterval"
        | "hotkeymodifiertimeout"
        | "menumaskkey"
        | "usehook"
        | "hotif"
        | "hotiftimeout"
        | "hotstring"
        | "maxthreads"
        | "maxthreadsperhotkey"
        | "maxthreadsbuffer" => Some("the hotkeys are not carried by this runtime."),
        "clipboardtimeout" => Some("the clipboard is not carried by this runtime."),
        "inputlevel" => Some("the input level is not carried by this runtime."),
        "suspendexempt" => Some("the suspend is not carried by this runtime."),
        "winactivateforce" => Some("the window activation is not carried by this runtime."),
        "warn" => Some("the warnings are not carried by this runtime."),
        "include" | "includeagain" => {
            Some("the includes are resolved by the runner, not by the language.")
        }
        "dllload" => Some("a Windows library is a Windows thing and this box has none."),
        _ => None,
    }
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

/// The path of a file that is relative to the folder that the script runs in.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the path
/// of a file of the loop of the v1 is the one that is relative to the folder
/// that the script runs in: the "a.txt" of the folder itself, and the
/// "sub/d.txt" of a folder of it.
fn the_path_that_is_relative_to(file: &str, folder: &str) -> String {
    let rest = match file.strip_prefix(folder) {
        Some(rest) => rest,
        None => return file.to_string(),
    };
    rest.trim_start_matches(['/', '\\']).to_string()
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

/// The letter of the mode of the loop of the files that the mode does not
/// carry.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
/// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the mode is a set of
/// letters -- the "R" walks into the folders, the "D" and the "K" give the
/// folders, the "F" and the "V" give the files -- and the spaces and the tabs
/// between them are ignored. The letter "A" is not one of them, and a number
/// is not one either.
fn the_letter_that_the_mode_does_not_carry(mode: &str) -> Option<char> {
    mode.chars().find(|letter| {
        !matches!(
            letter,
            'R' | 'r' | 'D' | 'd' | 'F' | 'f' | 'V' | 'v' | 'K' | 'k' | ' ' | '\t'
        )
    })
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

/// The object that a function is bound to.
///
/// The key has a space in it, so that a script cannot reach it by name.
pub(crate) const BOUND: &str = "bound object";

/// What a function that is bound to an object calls.
///
/// The value is the name of a method of the object, or the function that is
/// bound. The key has a space in it, so that a script cannot reach it by
/// name.
pub(crate) const BOUND_TARGET: &str = "bound target";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// The global scope of the script.
    Global,
    /// A local scope, by its place in the stack.
    Local(usize),
}

/// A reference to a variable: `&x`.
///
/// The reference is a value of its own kind, and v2 names it a VarRef.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
        self.properties
            .get(key)
            .map(|property| property.value.clone())
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
                if name.is_empty() {
                    // An object without a class is an object: the v1 has no
                    // classes, and the name of the kind of it is the one of
                    // the base.
                    name = "Object".to_string();
                }
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
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: u32,
}

impl core::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{} (line {})", self.message, self.line)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RuntimeError {}

/// The script ran for longer than the caller allowed.
#[derive(Debug)]
pub struct Timeout {
    pub seconds: f64,
}

impl core::fmt::Display for Timeout {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "the script ran for more than {}s", self.seconds)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Timeout {}

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
        Self {
            started: std::time::Instant::now(),
        }
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
#[derive(Debug, Clone)]
pub struct Step {
    pub line: u32,
    pub op: String,
    pub detail: String,
    pub ok: bool,
    pub elapsed_ms: u64,
}

/// The whole run: the steps, the output, and whether it finished.
#[derive(Debug, Clone, Default)]
pub struct Report {
    pub ok: bool,
    pub steps: Vec<Step>,
    pub output: Vec<String>,
    /// The warnings of the script.
    pub warnings: Vec<crate::warnings::Warning>,
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
    Goto {
        name: String,
        line: u32,
    },
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
    /// The dialect that the run speaks.
    ///
    /// The v2 is the dialect of the tree, and it is the one of every run
    /// that does not ask for another one. The v1 is behind the gate of the
    /// `--dialect=v1` of the runner, and it has a table of its own names
    /// and rules of its own.
    dialect: &'static dyn crate::dialect::Dialect,
    /// Whether the environment is looked up.
    ///
    /// The directive `#NoEnv` of the v1 turns the lookup off, and the
    /// substitution of a parameter of the v1 is then the variable of the
    /// script alone.
    no_env: bool,
    /// The warnings of the script.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    /// warnings are computed before the script runs, and they go to the
    /// report.
    warnings: Vec<crate::warnings::Warning>,
    /// The mode of each warning of the script.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
    /// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the directives are read
    /// before the script runs, and the mode of each warning is the one of the
    /// last `#Warn` that names it.
    modes: crate::warnings::TheModes,
    /// Whether the text of a legacy assignment is trimmed.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `AutoTrim`
    /// of the v1 is On when the script begins, and the command turns it off.
    /// The text of a legacy assignment is trimmed when it is On, and the
    /// command parameters of the v1 are not trimmed by it.
    auto_trim: bool,
    /// The format of the whole numbers, the way the dialect formats them.
    integer_format: IntegerFormat,
    /// The format of the floats, the way the dialect formats them.
    float_format: FloatFormat,
    /// The sense of the case, the way the dialect compares the letters.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the sense is Off
    /// when the script begins, and the `StringCaseSense` turns it into On or
    /// into Locale.
    case_sense: CaseSense,
    /// The number of the lines that the script runs at once.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the number is
    /// "10ms" when the script begins, and the `SetBatchLines` turns it into
    /// the number that was written.
    batch_lines: alloc::string::String,
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
    /// Whether the runtime is running the body of a hotkey of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// body of a hotkey is the code that comes after it, and the `return` of
    /// it ends the thread of the hotkey and not the script.
    in_the_body_of_a_hotkey: bool,
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
    /// The files of the file-reading loops that run.
    ///
    /// The name of the file of a `FileAppend` may be left out inside the
    /// body of a `Loop Read` that has a file of its own, and the text goes
    /// to that file. The last one of the list is the loop that runs.
    loop_output_files: Vec<String>,
    /// Whether the short names of the files are refused.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `A_LoopFileShortName` is the name that Windows gives to a file that has
    /// a short one, and this box has none: the script that reads the name is
    /// told that. The field is set when a loop of the files runs, and the
    /// name has no value before it.
    the_short_names_of_the_files_are_refused: bool,
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
            dialect: crate::dialect::v2(),
            no_env: false,
            warnings: Vec::new(),
            modes: crate::warnings::TheModes::the_defaults_of_the_v2(),
            auto_trim: true,
            integer_format: IntegerFormat::Decimal,
            float_format: FloatFormat::DEFAULT,
            case_sense: CaseSense::Off,
            batch_lines: "10ms".to_string(),
            timeout,
            variables: BTreeMap::new(),
            statics: BTreeMap::new(),
            statics_run: BTreeMap::new(),
            initial_working_dir,
            scopes: Vec::new(),
            label_scopes: Vec::new(),
            in_the_body_of_a_hotkey: false,
            declarations: Vec::new(),
            current_classes: Vec::new(),
            frames: Vec::new(),
            loop_output_files: Vec::new(),
            the_short_names_of_the_files_are_refused: false,
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
            dialect: crate::dialect::v2(),
            no_env: false,
            warnings: Vec::new(),
            modes: crate::warnings::TheModes::the_defaults_of_the_v2(),
            auto_trim: true,
            integer_format: IntegerFormat::Decimal,
            float_format: FloatFormat::DEFAULT,
            case_sense: CaseSense::Off,
            batch_lines: "10ms".to_string(),
            timeout,
            variables: BTreeMap::new(),
            statics: BTreeMap::new(),
            statics_run: BTreeMap::new(),
            initial_working_dir,
            scopes: Vec::new(),
            label_scopes: Vec::new(),
            in_the_body_of_a_hotkey: false,
            declarations: Vec::new(),
            current_classes: Vec::new(),
            frames: Vec::new(),
            loop_output_files: Vec::new(),
            the_short_names_of_the_files_are_refused: false,
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

    /// A runtime that speaks the dialect of this name.
    ///
    /// The v2 is the dialect of the tree, and a runtime that does not ask
    /// for another one speaks it. The v1 is behind the gate of the
    /// `--dialect=v1` of the runner.
    pub fn with_dialect(mut self, dialect: &'static dyn crate::dialect::Dialect) -> Self {
        self.dialect = dialect;
        // The settings of the script begin with the defaults of the dialect
        // that the run speaks: the v1 and the v2 do not begin with the same
        // values.
        self.dialect.the_defaults_of_the_settings();
        self
    }

    /// The dialect that the run speaks.
    pub fn dialect(&self) -> &'static dyn crate::dialect::Dialect {
        self.dialect
    }

    /// Whether the text of a legacy assignment is trimmed.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `AutoTrim`
    /// of the v1 is On when the script begins, and the command turns it off.
    pub fn auto_trim(&self) -> bool {
        self.auto_trim
    }

    /// The trim of the legacy assignments of the v1, on or off.
    pub fn set_auto_trim(&mut self, on: bool) {
        self.auto_trim = on;
    }

    /// The number of the lines that the script runs at once.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the number is
    /// "10ms" when the script begins, and the `SetBatchLines` turns it into
    /// the number that was written.
    pub fn batch_lines(&self) -> &str {
        &self.batch_lines
    }

    /// The number of the lines that the script runs at once.
    pub fn set_batch_lines(&mut self, text: &str) {
        self.batch_lines = text.to_string();
    }

    /// The format of the integers, the way the dialect formats them.
    pub fn integer_format(&self) -> IntegerFormat {
        self.integer_format
    }

    /// The format of the floats, the way the dialect formats them.
    pub fn float_format(&self) -> FloatFormat {
        self.float_format
    }

    /// The format of the integers, the way the dialect formats them.
    pub fn set_integer_format(&mut self, format: IntegerFormat) {
        self.integer_format = format;
    }

    /// The format of the floats, the way the dialect formats them.
    pub fn set_float_format(&mut self, format: FloatFormat) {
        self.float_format = format;
    }

    /// The sense of the case, the way the dialect compares the letters.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the sense is Off
    /// when the script begins, and the `StringCaseSense` turns it into On or
    /// into Locale.
    pub fn case_sense(&self) -> CaseSense {
        self.case_sense
    }

    /// The write of the last error of the run, the way the v1 keeps it.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `ErrorLevel`
    /// of the v1 is the global variable of the script, and a command that
    /// fails writes it -- the `StringGetPos` writes zero when it finds the
    /// text and one when it does not, and the `StringReplace` writes the
    /// number of the replacements when the parameter says "useerrorlevel" and
    /// zero when it does not. The write is the one of the script, and it does
    /// not care about the scope that runs: a function that writes the name
    /// writes a local one, and the machine writes the global.
    pub(crate) fn set_the_error_level(&mut self, value: Value) {
        self.variables.insert(the_key_of("ErrorLevel"), value);
    }

    /// The sense of the case of the run.
    pub fn set_case_sense(&mut self, sense: CaseSense) {
        self.case_sense = sense;
    }

    /// Whether an object has no text in the dialect that runs.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text of an
    /// object of the v1 is the empty text.
    pub(crate) fn an_object_has_no_text(&self) -> bool {
        self.dialect.an_object_has_no_text()
    }

    /// The text of a value, in the format of the dialect.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the numbers of
    /// the v1 are formatted by the `SetFormat`: the float is rounded to the
    /// decimals of the format, and the whole number is in the base of the
    /// format. The v2 writes the numbers the way they are.
    pub fn the_text_of(&self, value: &Value) -> String {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the text of an
        // object of the v1 is the empty text -- `Object()` is a blank text,
        // and `Object() . "a"` is "a".
        if self.dialect.an_object_has_no_text() {
            if let Value::Object(_) = value {
                return String::new();
            }
        }
        if !self.dialect.formats_the_numbers() {
            return value.to_text();
        }
        match value {
            Value::Int(number) => self.integer_format.the_text_of(*number),
            Value::Float(number) => self.float_format.the_text_of(*number),
            other => other.to_text(),
        }
    }

    /// The built-in of this name, in the dialect that runs.
    ///
    /// The name is matched the way AutoHotkey matches it: without regard
    /// to the case.
    pub(crate) fn builtin(&self, name: &str) -> Option<&'static crate::builtins::Builtin> {
        let wanted = name.to_lowercase();
        self.dialect
            .builtins()
            .iter()
            .find(|builtin| builtin.name.to_lowercase() == wanted)
    }

    /// The built-in of this name and of this kind, in the dialect that runs.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-03: the `GetKeyState`
    /// of the v1 is a command and a function, and the call of an expression
    /// is the function: it takes one parameter and it answers the state of
    /// the key. The name that the dialect carries once is the row of it,
    /// whatever the kind is.
    pub(crate) fn the_builtin_of_the_kind(
        &self,
        name: &str,
        kind: crate::builtins::BuiltinKind,
    ) -> Option<&'static crate::builtins::Builtin> {
        let wanted = name.to_lowercase();
        let builtins = self.dialect.builtins();
        builtins
            .iter()
            .find(|builtin| builtin.name.to_lowercase() == wanted && builtin.kind == kind)
            .or_else(|| {
                builtins
                    .iter()
                    .find(|builtin| builtin.name.to_lowercase() == wanted)
            })
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

    /// The hotkeys of the v1 are registered when the script loads.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// hotkey that is written in the body of another one is registered too,
    /// and the registration of it does not wait for the code that comes
    /// before it.
    fn register_the_hotkeys_of_the_v1(&mut self, statements: &[Node]) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Node::HotkeyDefinition {
                name, body, line, ..
            } = statement
            {
                crate::hotkeys::register_the_hotkey_of_the_v1(self, name, body.clone(), *line)
                    .map_err(|failure| match failure {
                        RuntimeFailure::Error(error) => error,
                        RuntimeFailure::Detailed(error, _) => error,
                        RuntimeFailure::Fatal(error) => error,
                        _ => RuntimeError {
                            message: "the hotkey could not be registered.".to_string(),
                            line: *line,
                        },
                    })?;
                self.register_the_hotkeys_of_the_v1(body)?;
            }
        }
        Ok(())
    }

    /// Run the script and report what happened.
    pub fn run(mut self, statements: &[Node]) -> Report {
        let mut error = None;
        let mut error_line = None;
        let mut exit_code = None;
        // The functions are collected before the first statement runs, so a
        // script may call a function it defines further down.
        self.collect_functions(statements);
        // The warnings of the script are computed before the script runs, the
        // way the v2 computes them: the directives are all read, and then the
        // script is checked.
        //
        // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
        // warning of the unreachable line is given before the first statement
        // runs, and the mode of the "StdOut" writes it to the standard
        // output.
        self.warnings = self.the_warnings_of_the_script(statements);
        // The hotkeys of the v1 are registered when the script loads, and
        // the one that is written in the body of another one is registered
        // too.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // hotkey of the v1 is registered when the script loads, and not when
        // the code that comes before it runs.
        let registered = self.register_the_hotkeys_of_the_v1(statements);
        // The labels of the script are the scope of the script.
        self.label_scopes.push(the_labels_of(statements));
        match registered {
            Ok(()) => match self.run_block(statements) {
                Ok(()) => {}
                Err(RuntimeFailure::Flow(Flow::Break))
                | Err(RuntimeFailure::Flow(Flow::Continue)) => {}
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
            },
            // The desktop cannot watch the keyboard, and the script is told
            // why: a hotkey that is registered and never fires is a hotkey
            // that does not work.
            Err(failure) => {
                error = Some(failure.message);
                error_line = Some(failure.line);
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
                            "the function of the exit left the script in the middle of a block."
                                .to_string(),
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
            warnings: self.warnings,
            notes: self.notes,
            error,
            error_line,
            exit_code,
            elapsed_ms,
            variables: self.variables.clone(),
        }
    }

    /// The warnings of the script.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the check
    /// of the unreachable lines is made before the script runs, and the mode
    /// of the "StdOut" writes the warning to the standard output with the
    /// line of it. The tree has no dialog, and the warning goes to the
    /// report.
    fn the_warnings_of_the_script(&mut self, statements: &[Node]) -> Vec<crate::warnings::Warning> {
        self.modes = crate::warnings::the_modes_of_the_script(statements, self.dialect);
        // The type of the warning that the tree does not carry yet is named in
        // the report: the script asks for it, and the tree does not give it.
        for kind in crate::warnings::the_types_of_the_script(statements, self.dialect) {
            if !crate::warnings::the_types_that_are_carried().contains(&kind) {
                self.note(format!(
                    "the warnings of the type \"{}\" are not carried by this runtime.",
                    crate::warnings::the_name_of_the_type(kind)
                ));
            }
        }
        let mut warnings = Vec::new();
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05 and on
        // the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning of the
        // local that has the name of a global is given when the script is
        // read, and it comes before the warning of the variable that is read
        // and never written.
        if self.modes.local_same_as_global != crate::warnings::WarningMode::Off {
            let mode = self.modes.local_same_as_global;
            for warning in crate::warnings::the_warnings_of_the_locals_that_are_globals(
                statements,
                self.dialect,
            ) {
                self.warn(warning.line, warning.message.clone(), mode);
                warnings.push(warning);
            }
        }
        // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
        // warning of the variable that is read and never written is given when
        // the script is read, and not when the variable is read.
        if self.modes.var_unset != crate::warnings::WarningMode::Off {
            let mode = self.modes.var_unset;
            for warning in
                crate::warnings::the_warnings_of_the_variables_that_are_never_written(statements)
            {
                self.warn(warning.line, warning.message.clone(), mode);
                warnings.push(warning);
            }
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // warning of the class that is overwritten comes after the warning of
        // the local that has the name of a global.
        if self.modes.class_overwrite != crate::warnings::WarningMode::Off {
            let mode = self.modes.class_overwrite;
            for warning in
                crate::warnings::the_warnings_of_the_classes_that_are_overwritten(statements)
            {
                self.warn(warning.line, warning.message.clone(), mode);
                warnings.push(warning);
            }
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05 and on
        // the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning of the
        // line that never runs comes last.
        if self.modes.unreachable != crate::warnings::WarningMode::Off {
            let mode = self.modes.unreachable;
            for warning in crate::warnings::the_warnings_of_the_script(statements) {
                self.warn(warning.line, warning.message.clone(), mode);
                warnings.push(warning);
            }
        }
        warnings
    }

    /// The warning of the script.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the mode
    /// of the "StdOut" writes the warning to the standard output with the
    /// line of it, and the tree has no dialog: the other modes write it to
    /// the report alone.
    fn warn(&mut self, line: u32, message: String, mode: crate::warnings::WarningMode) {
        self.warnings.push(crate::warnings::Warning {
            line,
            message: message.clone(),
        });
        if mode == crate::warnings::WarningMode::StdOut {
            self.output
                .push(format!("({line}) : ==> Warning: {message}"));
        }
    }

    /// The warning of the variable that is read and never written.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    /// warning is given before the failure of the run, and the text of it
    /// names the kind of the variable -- "This global variable appears to
    /// never be assigned a value." and "This local variable appears to never
    /// be assigned a value.".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// text of the v1 is one for the two kinds, and the kind is in the text
    /// that names the variable -- "This variable has not been assigned a
    /// value." and "nope  (a global variable)". A local variable that has the
    /// name of a global one says so -- "x  (a local variable with same name
    /// as a global)".
    fn warn_about_the_variable_that_has_no_value(&mut self, name: &str, line: u32) {
        // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
        // warning of the v2 is given when the script is read, and not when the
        // variable is read: the runtime only fails.
        if !self.dialect.unset_is_blank() {
            return;
        }
        let local = !matches!(self.the_scope_of(name), Scope::Global);
        let kind = if local {
            if self.variables.contains_key(&the_key_of(name)) {
                "a local variable with same name as a global"
            } else {
                "a local variable"
            }
        } else {
            "a global variable"
        };
        let mode = if local {
            self.modes.use_unset_local
        } else {
            self.modes.use_unset_global
        };
        if mode == crate::warnings::WarningMode::Off {
            return;
        }
        self.warn(
            line,
            format!(
                "This variable has not been assigned a value.\n     Specifically: {name}  ({kind})"
            ),
            mode,
        );
    }

    /// The warning of the environment variable that is accessed.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// text of it is "An environment variable is being accessed; see
    /// #NoEnv." and the name of the variable, and it is given when the
    /// environment is accessed.
    fn warn_about_the_environment_that_is_accessed(&mut self, name: &str, line: u32) {
        let mode = self.modes.use_env;
        if mode == crate::warnings::WarningMode::Off {
            return;
        }
        let message = format!(
            "An environment variable is being accessed; see #NoEnv.\n     Specifically: {name}"
        );
        self.warn(line, message, mode);
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
                Node::ClassDef {
                    name,
                    extends,
                    methods,
                    properties,
                    accessors,
                    nested,
                    ..
                } => {
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
                Node::FunctionDef {
                    name,
                    parameters,
                    body,
                    declarations,
                    ..
                } => {
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
                Node::Block { body, .. } => self.collect_functions(body),
                Node::LegacyIf {
                    body, otherwise, ..
                } => {
                    self.collect_functions(body);
                    self.collect_functions(otherwise);
                }
                Node::If {
                    branches,
                    otherwise,
                    ..
                } => {
                    for branch in branches {
                        self.collect_functions(&branch.tail);
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
                Node::Try {
                    body,
                    catches,
                    otherwise,
                    finally,
                    ..
                } => {
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
                Node::FunctionDef {
                    name,
                    parameters,
                    body,
                    declarations,
                    ..
                } => {
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
                Node::Block { body, .. } => self.collect_the_nested_functions(body),
                Node::LegacyIf {
                    body, otherwise, ..
                } => {
                    self.collect_the_nested_functions(body);
                    self.collect_the_nested_functions(otherwise);
                }
                Node::If {
                    branches,
                    otherwise,
                    ..
                } => {
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
                Node::While { body, .. } | Node::Loop { body, .. } => {
                    self.collect_the_nested_functions(body)
                }
                Node::Try {
                    body,
                    catches,
                    otherwise,
                    finally,
                    ..
                } => {
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
        if the_labels_of(scope)
            .iter()
            .any(|label| label.eq_ignore_ascii_case(name))
        {
            return RuntimeError {
                message: self
                    .dialect
                    .the_refusal_of_a_jump_into_a_block()
                    .to_string(),
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
        let Some(scope) = self.label_scopes.last() else {
            return false;
        };
        scope.iter().any(|label| label.eq_ignore_ascii_case(name))
    }

    fn run_block(&mut self, statements: &[Node]) -> Result<(), RuntimeFailure> {
        let mut index = 0;
        while index < statements.len() {
            let statement = &statements[index];
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the `A_LineNumber` is the line of the statement that runs, and
            // the tree did not write it.
            self.set_global("A_LineNumber", Value::Int(statement.line() as i64));
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
            if self.the_section_that_runs() {
                self.run_timers()?;
                // The hotkeys of the script run between its statements,
                // and only there: a function of the script is not the
                // script, and a hotkey runs with the scope of the script
                // and not with the scope of the function.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-03: the body of a hotkey of the v1 may jump to a
                // label of the script, and the jump goes on at the label.
                match crate::hotkeys::run_fired(self) {
                    Ok(()) => {}
                    Err(RuntimeFailure::Flow(Flow::Goto { name, line })) => {
                        match the_place_of_the_label(statements, &name) {
                            Some(place) => index = place,
                            None => return Err(RuntimeFailure::Flow(Flow::Goto { name, line })),
                        }
                    }
                    Err(failure) => return Err(failure),
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-03: the section of the script that runs ends at
                // the first hotkey or at the first hotstring of the v1, and
                // the hotkey itself is registered when the script loads.
                if self.dialect.the_section_that_runs_ends_at_the_hotkey()
                    && self.the_statement_ends_the_section(statement)
                {
                    return Ok(());
                }
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

    /// The body of a hotkey of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// body of a hotkey runs with the scope of the script, and the `return`
    /// of it ends the thread of the hotkey and not the script.
    pub(crate) fn run_the_body_of_a_hotkey(&mut self, body: &[Node]) -> Result<(), RuntimeFailure> {
        let was_in_the_body = self.in_the_body_of_a_hotkey;
        self.in_the_body_of_a_hotkey = true;
        let answer = self.run_block(body);
        self.in_the_body_of_a_hotkey = was_in_the_body;
        match answer {
            // The `return` ends the hotkey, and the script goes on.
            Err(RuntimeFailure::Flow(Flow::Return(_))) => Ok(()),
            other => other,
        }
    }

    /// Whether the runtime is running the section of the script that runs.
    ///
    /// The body of a hotkey of the v1 runs with the scope of the script, and
    /// it is not the section: the timers of the script do not run there, and
    /// the section does not end there.
    fn the_section_that_runs(&self) -> bool {
        self.scopes.is_empty() && !self.in_the_body_of_a_hotkey
    }

    /// Whether the statement ends the section of the script that runs.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// section of the v1 ends at the first hotkey or at the first
    /// hotstring, and the code that comes after it does not run.
    fn the_statement_ends_the_section(&self, statement: &Node) -> bool {
        match statement {
            Node::HotkeyDefinition { .. } => true,
            Node::LegacyCall { name, .. } => name == "hotstring",
            _ => false,
        }
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
    fn catch_takes(
        &self,
        catch: &crate::ast::Catch,
        value: &Value,
    ) -> Result<bool, RuntimeFailure> {
        let class = catch.class.to_lowercase();
        if class == "any" {
            return Ok(true);
        }
        if class == "error" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a catch that
            // names no class takes every error of the runtime. The errors
            // are a tree of classes, and every one of them is an `Error`.
            let Value::Object(object) = value else {
                return Ok(false);
            };
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
        let Value::Object(object) = value else {
            return false;
        };
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
        let Some(frame) = self.frames.last() else {
            return;
        };
        put(
            &mut object.borrow_mut().properties,
            "what",
            Value::Text(frame.name.clone()),
        );
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
        self.timers.push(Timer {
            name: name.to_string(),
            period,
            last,
        });
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
            return Err(RuntimeFailure::Timeout(Timeout {
                seconds: self.timeout.as_secs_f64(),
            }));
        }

        let started = self.clock.now_ms();
        let mut detail = String::new();

        match statement {
            Node::Assign {
                name,
                op,
                value,
                line,
            } => {
                let value = self.evaluate(value)?;
                let stored = self.write_a_variable(name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::LegacyAssign { name, text, line } => {
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // legacy assignment of the v1 is a text and not an
                // expression: `x = 5 + 3` leaves `x` at "5 + 3".
                let value = self.the_value_of_a_legacy_assignment(text, *line)?;
                let stored = self.write_a_variable(name, AssignOp::Set, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::LegacyCall {
                name,
                parameters,
                line,
            } => {
                // The legacy command of the v1: `Msg, text`.
                let value = self.the_legacy_command(name, parameters, *line)?;
                detail = value.to_text();
            }
            Node::LegacyIf {
                name,
                parameters,
                body,
                otherwise,
                line,
            } => {
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // command of the v1 that has a body runs its action when the
                // condition holds, and the action of the other side when it
                // does not.
                let holds = self.the_condition_of_a_legacy_if(name, parameters, *line)?;
                if holds {
                    self.run_block(body)?;
                } else {
                    self.run_block(otherwise)?;
                }
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
            Node::For {
                variables,
                source,
                body,
                line,
            } => {
                self.for_loop(variables, source, body, *line)?;
            }
            Node::MethodCall {
                target,
                name,
                arguments,
                line,
            } => {
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
            Node::AssignDynamic {
                name,
                op,
                value,
                line,
            } => {
                let name = self.the_name_of_the_pieces(name, *line)?;
                let value = self.evaluate(value)?;
                let stored = self.write_a_dynamic_name(&name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::AssignProperty {
                target,
                name,
                op,
                value,
                line,
            } => {
                let value = self.evaluate(value)?;
                let name = self.the_name_of_the_member(name, *line)?;
                let stored = self.write_a_property(target, &name, *op, value, *line)?;
                detail = format!("{name} = {}", stored.to_text());
            }
            Node::AssignIndex {
                target,
                index,
                op,
                value,
                line,
            } => {
                let target_value = self.evaluate(target)?;
                let key = self.evaluate(index)?;
                let value = self.evaluate(value)?;
                let stored = self.set_item(&target_value, &key, *op, value, *line)?;
                detail = format!("{} = {}", key.to_text(), stored.to_text());
            }
            Node::Call {
                name,
                arguments,
                line,
            } => {
                let value = self.call(name, arguments, *line)?;
                detail = value.to_text();
            }
            Node::New {
                path,
                arguments,
                line,
            } => {
                let value = self.new_instance(path, arguments, *line)?;
                detail = value.to_text();
            }
            Node::Block { body, .. } => {
                self.run_block(body)?;
            }
            Node::If {
                branches,
                otherwise,
                ..
            } => {
                let mut taken = false;
                for branch in branches {
                    let value = self.the_value_of_the_condition(&branch.condition)?;
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the items that come after the comma of the
                    // line of the condition run after the condition is
                    // evaluated, and they run whether it is true or false.
                    for item in &branch.tail {
                        self.evaluate(item)?;
                    }
                    if self.the_value_is_true(&value) {
                        self.run_block(&branch.body)?;
                        taken = true;
                        break;
                    }
                }
                if !taken {
                    self.run_block(otherwise)?;
                }
            }
            Node::Switch {
                subject,
                cases,
                line,
            } => {
                let value = self.evaluate(subject)?;
                let mut matched = false;
                for case in cases {
                    if case.values.is_empty() {
                        // The default: it matches when no other case did.
                        self.run_block(&case.body)?;
                        matched = true;
                        break;
                    }
                    // A case may name many values, and it matches when the
                    // subject of the switch is one of them.
                    for expected in &case.values {
                        let expected = self.evaluate(expected)?;
                        if values_equal(&value, &expected, false) {
                            self.run_block(&case.body)?;
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        break;
                    }
                }
                let _ = (matched, line);
            }
            Node::While {
                condition, body, ..
            } => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
                // the v1 takes the body of the loop from the statement that
                // comes next, and the script that has none ends at the loop:
                // the `while 1` that is the last line of the script ends the
                // script at once.
                if body.is_empty() && self.dialect.a_loop_without_a_body_ends_the_script() {
                    return Err(RuntimeFailure::Flow(Flow::Return(None)));
                }
                let mut iterations = 0usize;
                // Measured on the binary of v2.0.19 on 2026-11-01: the index
                // of the turn is the number of it, and a while loop counts
                // as well. The body may write the index, and the loop does
                // not mind: the next turn sets it again.
                let mut turn = 1i64;
                // Measured on the binary of v2.0.19 on 2026-11-01: every
                // loop saves the value of `A_Index` when it starts and
                // gives it back when it ends.
                let saved = self.the_index_is_saved();
                loop {
                    let value = self.the_value_of_the_condition(condition)?;
                    if !self.the_value_is_true(&value) {
                        break;
                    }
                    iterations += 1;
                    if iterations > Self::MAX_ITERATIONS {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: format!(
                                "the loop ran more than {} times",
                                Self::MAX_ITERATIONS
                            ),
                            line: statement.line(),
                        }));
                    }
                    self.set_global("A_Index", Value::Int(turn));
                    turn += 1;
                    match self.run_block(body) {
                        Ok(()) => {}
                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                        Err(other) => return Err(other),
                    }
                }
                self.the_index_is_given_back(saved);
            }
            Node::Try {
                body,
                catches,
                otherwise,
                finally,
                ..
            } => {
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
                        if *scope == DeclareKind::Static
                            && self.the_static_already_ran(*line, &declared.name)
                        {
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
                                message: "there is nothing to throw again: no catch took a value."
                                    .into(),
                                line: *line,
                            }));
                        }
                    },
                };
                // v2 puts the name of the thing that threw the error in
                // its `What`, when the error does not carry one.
                self.name_the_thrower(&value);
                return Err(RuntimeFailure::Thrown(value));
            }
            Node::Loop {
                mode,
                arguments,
                body,
                until,
                line,
            } => {
                // Measured on the binary of v2.0.19 on 2026-11-01: every loop
                // saves the value of `A_Index` when it starts and gives it
                // back when it ends.
                let saved = self.the_index_is_saved();
                match mode {
                    LoopKind::Count => {
                        match arguments.first() {
                            Some(node) => {
                                // Measured on the binary of AutoHotkey
                                // v1.1.37.02 on 2026-11-05: the `Loop` of the
                                // v1 is a loop of files when the text of its
                                // first parameter is not a number, and it is a
                                // loop of a number of turns when it is.
                                if self.dialect.legacy_commands() {
                                    if let Some(pattern) = self.the_pattern_of_the_files_of_the_v1(
                                        node, arguments, *line,
                                    )? {
                                        self.the_loop_of_the_files_of_the_v1(
                                            &pattern, arguments, body, until, *line,
                                        )?;
                                        self.the_index_is_given_back(saved);
                                        return Ok(());
                                    }
                                }
                                let count = self.the_count_of_the_loop(node, *line)?;
                                // Measured on the binary of v2.0.19 on 2026-11-01:
                                // the index of the turn is the value of
                                // `A_Index`, and the body may write it: the
                                // loop goes on from the value that was written.
                                let mut turn = 1i64;
                                while turn <= count.max(0) {
                                    self.set_global("A_Index", Value::Int(turn));
                                    let broken = match self.run_block(body) {
                                        Ok(()) => false,
                                        Err(RuntimeFailure::Flow(Flow::Break)) => true,
                                        Err(RuntimeFailure::Flow(Flow::Continue)) => false,
                                        Err(other) => return Err(other),
                                    };
                                    if broken {
                                        break;
                                    }
                                    // Measured on the binary of AutoHotkey
                                    // v1.1.37.02 on 2026-11-04: the `until` of
                                    // the v1 is the condition of the loop, and it
                                    // is read after every turn of the body.
                                    if self.the_until_ends_the_loop(until)? {
                                        break;
                                    }
                                    turn = self.the_index_of_the_turn() + 1;
                                }
                            }
                            None => {
                                // Measured on the binary of AutoHotkey
                                // v1.1.37.02 on 2026-11-04: the v1 takes the
                                // body of the loop from the statement that
                                // comes next, and the script that has none
                                // ends at the loop: the `loop` that is the
                                // last line of the script ends the script at
                                // once.
                                if body.is_empty()
                                    && self.dialect.a_loop_without_a_body_ends_the_script()
                                {
                                    return Err(RuntimeFailure::Flow(Flow::Return(None)));
                                }
                                // Measured on the binary of v2.0.19 on
                                // 2026-10-31: the loop without a count goes
                                // on until a break. The step count is the
                                // bound of it, and the body may be empty.
                                // Measured on the binary of v2.0.19 on 2026-11-01:
                                // the index of the turn is the number of it,
                                // and the loop without a count counts as
                                // well.
                                let mut turn = 1i64;
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
                                    self.set_global("A_Index", Value::Int(turn));
                                    turn += 1;
                                    match self.run_block(body) {
                                        Ok(()) => {}
                                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                        Err(other) => return Err(other),
                                    }
                                    // Measured on the binary of AutoHotkey
                                    // v1.1.37.02 on 2026-11-04: the `until` of
                                    // the v1 is the condition of the loop, and it
                                    // is read after every turn of the body.
                                    if self.the_until_ends_the_loop(until)? {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    LoopKind::Parse => {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the parse loop of the v1 takes the name
                        // of the variable whose text is separated, and not its
                        // value.
                        if self.dialect.legacy_commands() {
                            self.the_parse_loop_of_the_v1(arguments, body, until, *line)?;
                            self.the_index_is_given_back(saved);
                            return Ok(());
                        }
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
                                    piece
                                        .trim_matches(|character| omitted.contains(character))
                                        .to_string()
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
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the `Loop Files` of the v1 is the `Loop`
                        // whose first parameter is the word "Files": the
                        // pattern of the files is the second parameter and the
                        // mode is the third one.
                        if self.dialect.legacy_commands() {
                            self.the_loop_of_the_files_of_the_v1_with_the_kind(
                                arguments, body, until, *line,
                            )?;
                            self.the_index_is_given_back(saved);
                            return Ok(());
                        }
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
                        // Measured on the binary of AutoHotkey v2.0.19 on
                        // 2026-11-05: the mode of the `Loop Files` is a set of
                        // letters -- the "R" walks into the folders, the "D"
                        // and the "K" give the folders, the "F" and the "V"
                        // give the files -- and the spaces between them are
                        // ignored. The letter that is not one of them is
                        // refused when the loop runs.
                        if let Some(letter) = the_letter_that_the_mode_does_not_carry(&mode) {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: format!(
                                    "the mode of a `Loop Files` is the \"R\", the \"D\", the \"F\", the \"V\" or the \"K\", and the letter \"{letter}\" is not one of them."
                                ),
                                line: *line,
                            }));
                        }
                        let folder = self.desktop().working_directory().map_err(|reason| {
                            RuntimeFailure::Error(RuntimeError {
                                message: reason,
                                line: *line,
                            })
                        })?;
                        let files = self
                            .desktop()
                            .list_files(&folder, &pattern, &mode)
                            .map_err(|reason| {
                                RuntimeFailure::Error(RuntimeError {
                                    message: reason,
                                    line: *line,
                                })
                            })?;
                        for (index, file) in files.iter().enumerate() {
                            let name = the_name_of(file);
                            let extension = the_extension_of(name);
                            self.set_global("A_Index", Value::Int(index as i64 + 1));
                            self.set_global("A_LoopFileFullPath", Value::Text(file.clone()));
                            self.set_global("A_LoopFileName", Value::Text(name.to_string()));
                            self.set_global("A_LoopFileExt", Value::Text(extension.to_string()));
                            self.set_global(
                                "A_LoopFileDir",
                                Value::Text(the_folder_of(file).to_string()),
                            );
                            self.set_global(
                                "A_LoopFilePath",
                                Value::Text(without_the_extension(file, extension)),
                            );
                            // The short names of the files are a thing of
                            // Windows, and this box has none.
                            self.the_short_names_of_the_files_are_refused = true;
                            match self.run_block(body) {
                                Ok(()) => {}
                                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                                Err(other) => return Err(other),
                            }
                        }
                    }
                    LoopKind::Read => {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the read loop of the v1 takes the name
                        // of the file in its second parameter, and the file
                        // where the text goes in its third one.
                        if self.dialect.legacy_commands() {
                            self.the_read_loop_of_the_v1(arguments, body, until, *line)?;
                            self.the_index_is_given_back(saved);
                            return Ok(());
                        }
                        let path = self.loop_argument(arguments, 0, *line)?.to_text();
                        // The file of the loop is where a `FileAppend` that
                        // has no name of its own writes.
                        //
                        // Measured on the binary of v2.0.19 on 2026-11-01:
                        // the name of the file of a `FileAppend` may be left
                        // out inside the body of a `Loop Read` that has a
                        // file of its own.
                        let output = match arguments.get(1) {
                            Some(node) => self.evaluate(node)?.to_text(),
                            None => String::new(),
                        };
                        let lines = self.desktop().read_file(&path).map_err(|reason| {
                            RuntimeFailure::Error(RuntimeError {
                                message: reason,
                                line: *line,
                            })
                        })?;
                        self.loop_output_files.push(output);
                        let mut failure = None;
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
                                Err(other) => {
                                    failure = Some(other);
                                    break;
                                }
                            }
                        }
                        self.loop_output_files.pop();
                        if let Some(other) = failure {
                            return Err(other);
                        }
                    }
                    LoopKind::Reg => {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: "the registry is a Windows thing and this box has none."
                                .into(),
                            line: *line,
                        }));
                    }
                }
                self.the_index_is_given_back(saved);
            }
            Node::Directive {
                name,
                argument,
                line,
            } => {
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
            // The expression of a parameter of the v1 is not a statement: the
            // command that holds the parameter reads it.
            Node::LegacyExpression { value, .. } => {
                self.evaluate(value)?;
            }
            // The jump of a `Goto`: the run goes on at the label.
            Node::Goto { name, line } => {
                return Err(RuntimeFailure::Flow(Flow::Goto {
                    name: name.clone(),
                    line: *line,
                }));
            }
            // The hotkey of the v1: the definition of it is a label, and
            // the code that comes after it is a part of the thread that is
            // running. The hotkey itself is registered when the script
            // loads, and the section of the script that runs ends at the
            // definition.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03:
            // the body of a hotkey runs with the scope of the script, and
            // the `return` of it ends the thread of the hotkey.
            Node::HotkeyDefinition { body, .. } => {
                if self.in_the_body_of_a_hotkey {
                    self.run_block(body)?;
                }
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
            | Node::Unset { .. }
            | Node::Ternary { .. }
            | Node::Group { .. }
            | Node::LegacyComparison { .. }
            | Node::LegacyMatch { .. }
            | Node::LegacyControl { .. }
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
            op: op_name_in(statement, self.dialect).to_string(),
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
        if self.dialect.the_for_is_the_one_of_the_v1() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `for` of the v1 walks the items of an array and nothing
            // else: the `for i in 5` has no turn and it does not fail.
            if self.the_class_of_the_value(&object) != "Array" {
                return Ok(());
            }
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `for` of the v1 walks the keys of the array, and it asks for them
        // even when the script gave it one name.
        let wanted = if self.dialect.the_for_is_the_one_of_the_v1() {
            2
        } else {
            variables.len()
        };
        let enumerator = self.the_enumerator(&object, wanted, line)?;
        // The enumerator has to be something that can be called: v2 fails
        // there before the body of the loop runs.
        if let Err(failure) = self.check_the_enumerator(&enumerator, line) {
            for (name, value) in saved {
                self.restore_variable(&name, value);
            }
            return Err(failure);
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: every loop saves
        // the value of `A_Index` when it starts and gives it back when it
        // ends.
        let saved_index = self.the_index_is_saved();
        let mut turns = 0usize;
        let mut outcome = Ok(());
        loop {
            // The enumerator hands out the turns one by one, and the body
            // of the loop runs between them.
            let item = match self.the_next_item(&enumerator, variables, wanted, line) {
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
        self.the_index_is_given_back(saved_index);
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
            let count = Node::Literal {
                value: LiteralValue::Integer(wanted as i64),
                line,
            };
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
        wanted: usize,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        if crate::classes::enumerator::is_one(enumerator) {
            return Ok(crate::classes::enumerator::the_turn(enumerator, wanted));
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
            let Value::Reference(reference) = reference else {
                continue;
            };
            values.push(
                self.read_reference(reference)
                    .unwrap_or(Value::Text(String::new())),
            );
        }
        let item = if values.len() >= 2 {
            Value::List(vec![values[0].clone(), values[1].clone()])
        } else {
            values
                .into_iter()
                .next()
                .unwrap_or(Value::Text(String::new()))
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
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 does not share them: the name lives in the scope of the
        // function that runs.
        if self.the_scopes_around_are_shared() {
            for (depth, scope) in self.scopes.iter().enumerate().rev().skip(1) {
                if scope.borrow().contains_key(&name) {
                    return Scope::Local(depth);
                }
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
        let Value::Object(value) = object else {
            return false;
        };
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
        if self.dialect.the_for_is_the_one_of_the_v1() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `for` of the v1 gives the key to the first name that is not
            // empty: the `for i in [10,20]` gives 1 and 2 to the `i`, and the
            // `for , v in [10,20]` gives them to the `v`.
            let pair = match item {
                Value::List(pair) if pair.len() == 2 => pair,
                _ => {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: "the enumeration did not give a key and a value.".into(),
                        line,
                    }));
                }
            };
            let names: Vec<&String> = variables.iter().filter(|name| !name.is_empty()).collect();
            if let Some(name) = names.first() {
                self.assign(name, pair[0].clone());
            }
            if let Some(name) = names.get(1) {
                self.assign(name, pair[1].clone());
            }
            return Ok(());
        }
        match variables.len() {
            0 => Ok(()),
            1 => {
                // The enumerator gives the value alone.
                if !variables[0].is_empty() {
                    // Measured on the binary of v2.0.19 on 2026-11-01: a turn
                    // that has no value of its own leaves the variable of the
                    // loop without a value: the script that reads it is told
                    // that the variable has no value.
                    if is_a_value_without_one(item) {
                        self.unset_variable(&variables[0]);
                    } else {
                        self.assign(&variables[0], item.clone());
                    }
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
                        }));
                    }
                };
                for (name, value) in variables.iter().zip(pair.iter()) {
                    if name.is_empty() {
                        continue;
                    }
                    // The key of the turn is the place of it, and it is there
                    // even when the value under it is not.
                    if is_a_value_without_one(value) {
                        self.unset_variable(name);
                    } else {
                        self.assign(name, value.clone());
                    }
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

    /// Take the value of a variable away.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the turn of a loop
    /// that has no value of its own leaves the variable of the loop without
    /// a value, and not with the value that the turn before it gave. The
    /// name is written where `assign` writes it, and it is taken away from
    /// there.
    fn unset_variable(&mut self, name: &str) {
        // A parameter that is an alias of a variable of the caller: the
        // variable of the caller goes without a value.
        if let Some(reference) = self.alias_of(name) {
            self.unset_reference(&Value::Reference(reference));
            return;
        }
        let name = the_key_of(name);
        // A static variable belongs to the function that declares it.
        if self.is_declared_static(&name) {
            self.the_statics_of_the_frame().remove(name.as_str());
            return;
        }
        // A name that the script has is a global one.
        if self.lives_in_the_script(&name) {
            self.variables.remove(name.as_str());
            return;
        }
        // A name that a scope around the call has is shared with it.
        for scope in self.scopes.iter().rev().skip(1) {
            if scope.borrow().contains_key(&name) {
                scope.borrow_mut().remove(name.as_str());
                return;
            }
        }
        match self.scopes.last() {
            Some(scope) => {
                scope.borrow_mut().remove(name.as_str());
            }
            None => {
                self.variables.remove(name.as_str());
            }
        }
    }

    // -- scopes ---------------------------------------------------------------------

    /// The value of a variable, from the innermost scope outwards.
    ///
    /// A function and a class are values too: a script can hold one and
    /// hand it on, and `Func` is the class of a function.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the name of a
    /// function of the v1 is not a value, and the name of a command of the
    /// v1 is not a value either -- `x := StrLen` is a blank text -- and the
    /// dialect says so.
    fn lookup(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.lookup_variable(name) {
            return Some(value);
        }
        let key = name.to_lowercase();
        if let Some(function) = self.functions.get(&key) {
            if !self.dialect.the_name_of_a_function_is_a_value() {
                return Some(Value::Text(String::new()));
            }
            return Some(func_value(&function.name, function));
        }
        // A built-in is a function, and a function is a value. The name
        // of it comes before the class: the `Hotkey` is a function of the
        // dialect, and the class of a hotkey is the class of the object
        // that the function makes.
        if let Some(builtin) = self.builtin(&key) {
            if !self.dialect.the_name_of_a_function_is_a_value() {
                return Some(Value::Text(String::new()));
            }
            return Some(builtin_value(builtin.name, builtin.min, builtin.max as u32));
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
        let Value::Object(object) = target else {
            return None;
        };
        let class = object.borrow().class.clone();
        // The name the class was written with, when the runtime knows it.
        if let Some(Value::Object(class)) = self.class_value_of(&class) {
            if let Some(name) = class.borrow().value_of(CLASS_NAME) {
                return Some(name.to_text());
            }
        }
        // The other objects are named after the class they carry.
        Some(crate::classes::the_spelling_of(&class))
    }

    /// The name of the class a value is, when the value is a class.
    ///
    /// The classes the runtime made are known to it, and it can tell one
    /// of them from an instance of one.
    pub(crate) fn class_name_of(&self, value: &Value) -> Option<String> {
        let Value::Object(object) = value else {
            return None;
        };
        for class in self.class_values.borrow().values() {
            let Value::Object(other) = class else {
                continue;
            };
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
        let Value::Object(object) = target else {
            return None;
        };
        // A base that the script set is the base of the object.
        if let Some(base) = object.borrow().value_of(BASE) {
            return Some(base.clone());
        }
        let class = object.borrow().class.clone();
        let Value::Object(class) = self.class_value_of(&class)? else {
            return None;
        };
        class.borrow().value_of("prototype")
    }

    /// The base of an object that the script set, if it set one.
    ///
    /// An object of a script has no base of its own until `ObjSetBase`
    /// gives it one: the prototype of its class is what `ObjGetBase`
    /// answers with.
    pub(crate) fn stored_base_of(&self, target: &Value) -> Option<Value> {
        let Value::Object(object) = target else {
            return None;
        };
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
            return Err(RuntimeError {
                message: "Invalid base.".to_string(),
                line,
            });
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 does not share the scopes around the call: a function has its
        // own scope, and the name of an outer call is not visible in it.
        if self.the_scopes_around_are_shared() {
            for scope in self.scopes.iter().rev().skip(1) {
                match scope.borrow().get(&name) {
                    Some(Value::Reference(reference)) if reference.alias => {
                        return self.read_reference(reference);
                    }
                    Some(value) => return Some(value.clone()),
                    None => continue,
                }
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 does not fall back to the scope of the script either: the name
        // of the script is not visible in a function, and only the `global`
        // declaration makes it visible.
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // variables that the runtime owns are not the variables of the script,
        // and they are seen in a function: the `A_LineNumber` is the line of
        // the statement that runs, and the `A_ThisFunc` is the name of the
        // function that runs.
        let owned = crate::variables::spelled(&name)
            .map(|rest| crate::variables::THE_RUNTIME_OWNS.contains(&rest.as_str()))
            .unwrap_or(false);
        if self.dialect.the_scope_of_a_function_is_its_own()
            && !self.scopes.is_empty()
            && !owned
            && !self.lives_in_the_script(&name)
        {
            return None;
        }
        self.variables.get(&name).cloned()
    }

    /// Whether the scopes around the call are shared with it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// does not share them: every function of the v1 has its own scope, and
    /// the name of an outer call is not visible in it. The v2 shares the name
    /// that a scope around the call has.
    fn the_scopes_around_are_shared(&self) -> bool {
        !self.dialect.the_scope_of_a_function_is_its_own()
    }

    /// Whether the name is one that the function that runs declares
    /// static.
    ///
    /// A name that the function declares static is not a local one: it
    /// lives in the storage of the function.
    fn is_declared_static(&self, name: &str) -> bool {
        let Some(frame) = self.declarations.last() else {
            return false;
        };
        frame.statics.contains(name)
    }

    /// The value of the built-in variable of this name, when the name is one.
    ///
    /// The dialect is asked, and not the table of the v2: the v1 has names
    /// of its own, and the two tables meet in the dialect.
    fn the_built_in_variable(&mut self, name: &str) -> Option<Result<Value, RuntimeError>> {
        let dialect = self.dialect;
        dialect.lookup_variable(name, self)
    }

    /// The value that the write of the built-in variable of this name leaves.
    ///
    /// The dialect is asked, and not the table of the v2: the v1 has names
    /// of its own, and the two tables meet in the dialect.
    fn the_write_of_the_built_in_variable(
        &mut self,
        name: &str,
        op: AssignOp,
        value: &Value,
        line: u32,
    ) -> Option<Result<Value, RuntimeError>> {
        let dialect = self.dialect;
        dialect.write_variable(name, op, value, line, self)
    }

    /// Whether a variable of this name has a value.
    ///
    /// The name is the one that the dialect spells, and the built-in
    /// variables of the dialect are the ones that its table carries.
    pub(crate) fn has_a_value(&mut self, name: &str) -> bool {
        if self.lookup_variable(name).is_some() {
            return true;
        }
        self.dialect.carries_variable(name)
    }

    /// Whether the script has a function of this name.
    pub(crate) fn has_a_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_lowercase())
    }

    /// Whether the dialect has the built-in function of this name.
    ///
    /// A command of the v1 is not a function: `IsFunc("MsgBox")` is zero in
    /// the v1. Measured on the binary of v1.1.37.02 on 2026-11-02.
    pub(crate) fn has_a_builtin_function(&self, name: &str) -> bool {
        self.dialect.builtins().iter().any(|builtin| {
            builtin.kind == crate::builtins::BuiltinKind::Function
                && builtin.name.eq_ignore_ascii_case(name)
        })
    }

    /// The value of the built-in function of this name.
    ///
    /// The v1 asks for it: the `Func` of the v1 is a reference to a function,
    /// and the reference of a built-in carries the name of it and the number
    /// of its parameters. Measured on the binary of v1.1.37.02 on 2026-11-02.
    ///
    /// The name that is a command and a function at the same time -- the
    /// `GetKeyState` of the v1 -- is a function: the table has a row for
    /// each of them, and the row of the function is the one that answers.
    pub(crate) fn the_builtin_function(&self, name: &str) -> Option<Value> {
        let builtin = self.dialect.builtins().iter().find(|builtin| {
            builtin.kind == crate::builtins::BuiltinKind::Function
                && builtin.name.eq_ignore_ascii_case(name)
        })?;
        let max = self.dialect.the_maximum_of_the_parameters(builtin.max);
        Some(builtin_value(builtin.name, builtin.min, max))
    }

    /// Whether the variable of this name is a reference.
    pub(crate) fn is_a_reference(&self, name: &str) -> bool {
        matches!(self.lookup_variable(name), Some(Value::Reference(_)))
    }

    /// The value of the function of the script of this name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `Func` of
    /// the v1 hands the function of the script over as a value, and the
    /// value carries the name of it and the number of its parameters.
    pub(crate) fn the_function_of_the_script(&self, name: &str) -> Option<Value> {
        let function = self.functions.get(&name.to_lowercase())?;
        Some(func_value(&function.name, function))
    }

    /// The static variables of the function that runs.
    ///
    /// The frame is the name of the function or of the method that runs,
    /// and the table of the function is under it.
    fn the_statics_of_the_frame(&mut self) -> &mut BTreeMap<String, Value> {
        let frame = self
            .frames
            .last()
            .map(|frame| frame.name.clone())
            .unwrap_or_default();
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
        let Some(frame) = self.frames.last() else {
            return false;
        };
        let Some(ran) = self.statics_run.get(&frame.name) else {
            return false;
        };
        ran.contains(&the_key_of_a_declaration(line, name))
    }

    /// Mark the value of a static declaration as one that ran.
    fn mark_the_static_as_ran(&mut self, line: u32, name: &str) {
        let frame = self
            .frames
            .last()
            .map(|frame| frame.name.clone())
            .unwrap_or_default();
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
        let Some(frame) = self.declarations.last() else {
            return false;
        };
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
        let Some(class) = self.the_class_of_the_name(&key) else {
            return Ok(());
        };
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
        let Value::Object(object) = value else {
            return None;
        };
        Some(object.borrow().class.clone())
    }

    /// The alias of this name, if the name is one.
    ///
    /// A parameter written `&x` in the definition of a function is an
    /// alias of the variable of the caller.
    fn alias_of(&self, name: &str) -> Option<Reference> {
        let name = the_key_of(name);
        for scope in self.scopes.iter().rev() {
            let Some(found) = scope.borrow().get(&name).cloned() else {
                continue;
            };
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
        let Value::Reference(reference) = value else {
            return;
        };
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

    /// Take the value of the variable a reference points at away.
    ///
    /// The variable is left without a value: the enumeration of a place that
    /// has no value of its own writes nothing into it.
    pub(crate) fn unset_reference(&mut self, value: &Value) {
        let Value::Reference(reference) = value else {
            return;
        };
        let name = the_key_of(&reference.name);
        match reference.scope {
            Scope::Global => {
                self.variables.remove(name.as_str());
            }
            Scope::Local(depth) => {
                if let Some(scope) = self.scopes.get(depth) {
                    scope.borrow_mut().remove(name.as_str());
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
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 does not share them: the name is written in the scope of the
        // function that runs.
        if self.the_scopes_around_are_shared() {
            for scope in self.scopes.iter().rev().skip(1) {
                if scope.borrow().contains_key(&name) {
                    scope.borrow_mut().insert(name, value);
                    return;
                }
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
        let Some(frame) = self.declarations.last() else {
            return false;
        };
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
        if crate::variables::is_built_in(name) || self.dialect.carries_variable(name) {
            return Some("built-in variable");
        }
        if self.functions.contains_key(&key) || self.builtin(&key).is_some() {
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
        let Value::Object(object) = target else {
            return chain;
        };
        let mut class = object.borrow().class.clone();
        // A class is not an instance of itself: the chain of a class begins
        // at the base of the class, and not at its own prototype.
        if self.class_name_of(target).is_some() {
            let Some(base) = self.the_base_class_of(&class) else {
                return chain;
            };
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
            let Some(base) = self.the_base_class_of(&class) else {
                break;
            };
            class = base;
        }
        chain
    }

    /// The prototype of a class, the object that holds its methods.
    fn the_prototype_of_the_class(&self, class: &str) -> Option<Rc<RefCell<Object>>> {
        let Some(Value::Object(value)) = self.class_value_of(class) else {
            return None;
        };
        let prototype = value.borrow().value_of("prototype");
        let Some(Value::Object(prototype)) = prototype else {
            return None;
        };
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
                None => crate::classes::the_base_of(&class).or_else(|| {
                    // The primitive types of v2 are a tree of their own,
                    // and the chain of them is the one of the `is`.
                    crate::classes::the_base_of_the_primitive_of_the_class(&class)
                })
                .map(|base| base.to_string()),
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
        let Value::Object(object) = target else {
            return None;
        };
        let class = object.borrow().class.clone();
        // A class value is not an instance of its class: the methods of the
        // class are not its own, and only the static ones are.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the call of a
        // method of an instance is refused with 'This value of type "Class"
        // has no method named "Say".'
        if self.class_name_of(target).is_some() {
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // calls the method of the class object, and the `this` of the
            // method is the class object.
            if self
                .dialect
                .a_method_of_the_class_object_is_a_method_of_the_class()
            {
                return self.find_the_instance_method_in(&class, name);
            }
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
    fn construct(
        &mut self,
        class: &str,
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
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

    /// The instance that the `new` of the v1 makes.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `new` makes
    /// an instance of a class of the script, the `__Init` of the class runs
    /// before the `__New`, and the arguments of the `new` are the ones of the
    /// `__New`. The new of a class that is not there is the empty text, and
    /// the v1 does not fail.
    fn new_instance(
        &mut self,
        path: &[String],
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // class of the v1 is a variable of the script, and the `new` reads
        // it.
        if self.dialect.the_class_is_a_variable() {
            return self.new_instance_of_the_v1(path, arguments, line);
        }
        let Some(class) = path.first() else {
            return Ok(Value::Text(String::new()));
        };
        if !self.classes.contains_key(&class.to_lowercase()) {
            return Ok(Value::Text(String::new()));
        }
        self.construct(class, arguments, line)
    }

    /// The instance that the `new` of the v1 makes.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// `new` reads the variable of the name, and the class is the value of
    /// it: `new v` makes an instance of the class that `v` holds, and the
    /// `new` of an instance makes another one of the same class. A name that
    /// holds another value makes nothing, and the v1 does not fail.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// name may be followed by the names of the classes that the class of
    /// the value holds: the `new C.D` is an instance of D, and the
    /// `new this.D` is an instance of the class that the instance of the
    /// method holds.
    fn new_instance_of_the_v1(
        &mut self,
        path: &[String],
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Some(first) = path.first() else {
            return Ok(Value::Text(String::new()));
        };
        // The first name of the path: the variable that holds the class, or
        // the class of the script, or the instance of the method that runs.
        let mut value = if first == "this" {
            self.the_this()
        } else {
            match self
                .lookup_variable(first)
                .or_else(|| self.class_value_of(first))
            {
                Some(value) => value,
                None => return Ok(Value::Text(String::new())),
            }
        };
        // Each name that follows is a class that the class of the value
        // holds.
        for name in &path[1..] {
            let Some(class) = self.the_class_that_the_v1_holds(&value, name) else {
                return Ok(Value::Text(String::new()));
            };
            value = class;
        }
        let Value::Object(object) = &value else {
            return Ok(Value::Text(String::new()));
        };
        // The class of the value: a class carries its own name, and an
        // instance carries the class of it.
        let class = match self.class_name_of(&value) {
            Some(class) => class,
            None => object.borrow().class.clone(),
        };
        self.construct(&class, arguments, line)
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
    fn call_the_init(
        &mut self,
        class: &str,
        object: &Value,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
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
        let Some(definition) = self.classes.get(class).cloned() else {
            return Ok(());
        };
        let Some(value) = self.class_value_of(class) else {
            return Ok(());
        };
        let Value::Object(object) = value else {
            return Ok(());
        };
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
            let Some(definition) = self.classes.get(name).cloned() else {
                continue;
            };
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
        let Some(definition) = self.classes.get(class).cloned() else {
            return Ok(());
        };
        let Value::Object(cell) = object else {
            return Ok(());
        };
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
                let arguments = [Node::Literal {
                    value: LiteralValue::Text(key.to_text()),
                    line,
                }];
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-02: the class of the v1 has no `Call`, and the
                // call of it makes nothing: the `C.Call()` is the empty
                // text. The instance of a class of the script is made with
                // the `new`.
                if self.dialect.the_call_of_a_class_makes_nothing() {
                    return Ok(Value::Text(String::new()));
                }
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
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
            // the class of the v1 has no `Call`, and the call of the class
            // that a class holds makes nothing: the `C.D()` is the empty
            // text. The instance is made with the `new`.
            if let Some(value) = self.the_class_that_a_class_holds(target, name) {
                if self.dialect.the_call_of_a_class_makes_nothing() {
                    return Ok(Value::Text(String::new()));
                }
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
                return Err(crate::classes::the_arity_failure(
                    &class, name, message, line,
                ));
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
                    Detail {
                        what: String::new(),
                        extra: String::new(),
                    },
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
                    if let Some((defining, method)) = self.find_the_instance_method_in(&class, name)
                    {
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not refuse the method that a value does not have, and the answer
        // is a blank text.
        if self.dialect.a_member_that_is_not_there_is_answered() {
            return Ok(Value::Text(String::new()));
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
        // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
        // arguments is checked before the arguments are evaluated: v2
        // refuses the script when it loads, and the refusal of the arity
        // comes before the failure of an argument.
        self.the_arity_of_a_method(class, method, arguments, line)?;
        let mut values: Vec<Value> = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the parameter
        // that the command does not give a value to is the one that is not
        // there: the place of it is kept, and it has no value.
        let missing: Vec<bool> = arguments
            .iter()
            .map(|argument| matches!(argument, Node::Empty { .. }))
            .collect();
        self.call_method_with_missing(class, method, this, &values, line, &missing)
    }

    /// The number of the arguments of the call of a method.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-30: the failure of the
    /// arity of a method carries the name of the method in the `Extra`, and
    /// the parameter that is missing carries the name of it.
    fn the_arity_of_a_method(
        &self,
        class: &str,
        method: &MethodDef,
        arguments: &[Node],
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not check the number of the arguments of a method, and the
        // parameters that the call does not give a value to are empty.
        if !self.dialect.the_call_of_a_method_checks_the_arity() {
            return Ok(());
        }
        // The rest of the arguments may be any number of them, and a
        // method that has one has no maximum.
        let variadic = method.parameters.iter().any(|parameter| parameter.variadic);
        if !variadic && arguments.len() > method.parameters.len() {
            let extra = self.the_name_of_the_method(class, method);
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Too many parameters passed to function.".into(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra,
                },
            ));
        }
        // A parameter that has no default is required: v2 refuses the call
        // and says that the parameter is missing.
        for (index, parameter) in method.parameters.iter().enumerate() {
            let is_missing = matches!(arguments.get(index), Some(Node::Empty { .. }));
            if (is_missing || arguments.get(index).is_none())
                && parameter.default.is_none()
                && !parameter.variadic
            {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Missing a required parameter.".into(),
                        line,
                    },
                    Detail {
                        what: String::new(),
                        extra: parameter.name.clone(),
                    },
                ));
            }
        }
        Ok(())
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
        self.call_method_with_missing(class, method, this, values, line, &[])
    }

    /// A method, called with the values of the arguments, and with the
    /// parameters that the call does not give a value to.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a parameter that
    /// the command does not give a value to is a missing one, and the call
    /// is refused with "Missing a required parameter." -- the name of the
    /// parameter is in the `Extra`. A parameter that has a default takes
    /// it.
    fn call_method_with_missing(
        &mut self,
        class: &str,
        method: &MethodDef,
        this: &Value,
        values: &[Value],
        line: u32,
        missing: &[bool],
    ) -> Result<Value, RuntimeFailure> {
        // The rest of the arguments may be any number of them, and a
        // method that has one has no maximum.
        let variadic = method.parameters.iter().any(|parameter| parameter.variadic);
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not check the number of the arguments of a method.
        if self.dialect.the_call_of_a_method_checks_the_arity() {
            if !variadic && values.len() > method.parameters.len() {
                let extra = self.the_name_of_the_method(class, method);
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Too many parameters passed to function.".into(),
                        line,
                    },
                    Detail {
                        what: String::new(),
                        extra,
                    },
                ));
            }
            // A parameter that has no default is required: v2 refuses the
            // call and says that the parameter is missing.
            for (index, parameter) in method.parameters.iter().enumerate() {
                let is_missing = missing.get(index).copied().unwrap_or(false);
                if (is_missing || values.get(index).is_none())
                    && parameter.default.is_none()
                    && !parameter.variadic
                {
                    return Err(RuntimeFailure::Detailed(
                        RuntimeError {
                            message: "Missing a required parameter.".into(),
                            line,
                        },
                        Detail {
                            what: String::new(),
                            extra: parameter.name.clone(),
                        },
                    ));
                }
            }
        } else {
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // calls the method only when the call gives a value to every
            // parameter that has no default. Otherwise the method does not
            // run, and the answer of the call is the empty text.
            for (index, parameter) in method.parameters.iter().enumerate() {
                let is_missing = missing.get(index).copied().unwrap_or(false);
                if (is_missing || values.get(index).is_none())
                    && parameter.default.is_none()
                    && !parameter.variadic
                {
                    return Ok(Value::Text(String::new()));
                }
            }
        }
        let mut scope: BTreeMap<String, Value> = BTreeMap::new();
        // The value the method was called on is `this`.
        scope.insert("this".to_string(), this.clone());
        // The parameters that the call does not give a value to take
        // their default, and the ones that have none are empty.
        for (index, parameter) in method.parameters.iter().enumerate() {
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
        if let Some(index) = method
            .parameters
            .iter()
            .position(|parameter| parameter.variadic)
        {
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
        let Value::Object(object) = target else {
            return None;
        };
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
            let base = definition
                .extends
                .clone()
                .unwrap_or_else(|| "object".to_string());
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
        let Some(name) = self.class_name_of(target) else {
            return chain;
        };
        if self.stored_base_of(target).is_some() {
            return chain;
        }
        let mut current = self.the_base_of_the_class_name(&name);
        while let Some(class) = current {
            let Some(value) = self.class_value_of(&class) else {
                break;
            };
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
        let Value::Object(value) = self.class_value_of(class)? else {
            return None;
        };
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
                    message: "This value of type \"Prototype\" has no method named \"Call\"."
                        .to_string(),
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
            return Err(crate::classes::named(
                self.no_method(target, &name, line),
                "GetMethod",
            ));
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
        let Some(class) = self.class_name_of(target) else {
            return names;
        };
        for class in self.class_chain(&class) {
            let Some(definition) = self.classes.get(&class) else {
                continue;
            };
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
        if !self
            .the_static_methods_of_a_class_value(target)
            .contains(&lower)
        {
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
        let Value::Object(object) = target else {
            return None;
        };
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
        format!(
            "{}.Prototype.{}",
            self.the_spelling_of_the_class(class),
            member
        )
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
        let Some(class) = self.the_class_of_the_prototype(target) else {
            return names;
        };
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
                    && definition
                        .properties
                        .iter()
                        .any(|property| !property.is_static)
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
    fn the_value_of_an_own_property_of_a_prototype(&self, target: &Value, name: &str) -> Value {
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
        let Some(class) = self.class_name_of(target) else {
            return names;
        };
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
        Value::Object(Rc::new(RefCell::new(Object {
            class: "func".to_string(),
            properties,
        })))
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
        put(
            &mut properties,
            "name",
            Value::Text(format!("{owner}.{}", method.name)),
        );
        // The first parameter of the reference is `this`.
        let required = method
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        put(
            &mut properties,
            "minparams",
            Value::Int(required as i64 + 1),
        );
        put(
            &mut properties,
            "maxparams",
            Value::Int(method.parameters.len() as i64 + 1),
        );
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(
            &mut properties,
            METHOD_CLASS,
            Value::Text(class.to_string()),
        );
        put(
            &mut properties,
            METHOD_NAME,
            Value::Text(method.name.clone()),
        );
        Value::Object(Rc::new(RefCell::new(Object {
            class: "func".to_string(),
            properties,
        })))
    }

    /// The reference of a static method of a class.
    ///
    /// The reference is the method, and the object is one of the
    /// arguments of the call.
    fn method_reference_of_the_base_object(
        &self,
        spelled: &str,
        method: &str,
        min: i64,
        max: i64,
    ) -> Value {
        let mut properties = BTreeMap::new();
        put(&mut properties, "name", Value::Text(spelled.to_string()));
        put(&mut properties, "minparams", Value::Int(min));
        put(&mut properties, "maxparams", Value::Int(max));
        put(&mut properties, "isbuiltin", Value::Int(1));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(
            &mut properties,
            METHOD_CLASS,
            Value::Text("object".to_string()),
        );
        put(
            &mut properties,
            METHOD_NAME,
            Value::Text(method.to_string()),
        );
        Value::Object(Rc::new(RefCell::new(Object {
            class: "func".to_string(),
            properties,
        })))
    }

    /// The copy of an object: the state of it is copied, and the class of
    /// it is the same.
    fn clone_of_an_object(&self, target: &Value) -> Value {
        let Value::Object(object) = target else {
            return target.clone();
        };
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
        let Value::Object(object) = target else {
            return target.clone();
        };
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

    /// The methods of the object of the v1: the `Count()` and the `Length()`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `a.Count()` is the number of the members of the object -- the property
    /// "b" of the `{b: 1}` is one, the items of an array are members as
    /// well, and the places that are not there are not members. The
    /// `a.Length()` is the number of the items of the object: the `{b: 1}` is
    /// zero, and the `[1, 2]` is two.
    fn the_method_of_the_v1(
        &mut self,
        target: &Value,
        name: &str,
        _arguments: &[Value],
        _line: u32,
    ) -> Option<Result<Value, RuntimeFailure>> {
        match name.to_lowercase().as_str() {
            "count" => Some(Ok(Value::Int(self.the_members_of_the_v1(target) as i64))),
            "length" => Some(Ok(Value::Int(self.the_items_of_the_v1(target) as i64))),
            _ => None,
        }
    }

    /// The members of an object of the v1: the properties that it owns and
    /// the items of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `{b: 1}` has one member, and the `[1, 2]` has two, and the `[1, 2]`
    /// with the property "b" has three.
    fn the_members_of_the_v1(&self, target: &Value) -> usize {
        let Value::Object(object) = target else {
            return 0;
        };
        let mut count = 0;
        for name in object.borrow().properties.keys() {
            // The names that the runtime owns are not members of the object.
            if name.as_str() == BASE
                || name.as_str() == CLASS_NAME
                || name.as_str() == "prototype"
                || name.as_str() == "array state"
            {
                continue;
            }
            count += 1;
        }
        count + crate::classes::array::the_number_of_the_items(target)
    }

    /// The items of an object of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `{b: 1}` has no item, and the `[1, 2]` has two. The places that are
    /// not there are places: the `[1, 2]` with the fifth one written has
    /// five places.
    fn the_items_of_the_v1(&self, target: &Value) -> usize {
        crate::classes::array::the_number_of_the_places(target)
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
        let Value::Object(object) = target else {
            return None;
        };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // object of the v1 has no method of the base object of the v2, and it
        // has the `Count()` and the `Length()`.
        if !self.dialect.the_object_has_the_methods_of_the_base_object() {
            return self.the_method_of_the_v1(target, name, arguments, line);
        }
        let class = object.borrow().class.clone();
        let method = name.to_string();
        let missing = || {
            RuntimeFailure::Error(RuntimeError {
                message: format!("{method} needs a name."),
                line,
            })
        };
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
                        || object
                            .borrow()
                            .properties
                            .contains_key(&name.to_lowercase())
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
                    if let Some((defining, method)) =
                        self.find_the_instance_method_in(&class, &name)
                    {
                        return Some(Ok(self.method_reference(&defining, &method)));
                    }
                    // A method of a class type of the runtime.
                    if let Some((spelled, min, max)) = crate::classes::the_method_of(&class, &name)
                    {
                        let reference =
                            self.method_reference_of_a_class_type(&class, spelled, min, max);
                        return Some(Ok(reference));
                    }
                    // A method of the base object.
                    if crate::classes::the_base_object_has_the_method(&name) {
                        let spelled =
                            crate::classes::the_name_of_a_method_of_the_base_object(&name);
                        let (min, max) =
                            crate::classes::the_arity_of("object", &name).unwrap_or((0, 0));
                        let reference = self.method_reference_of_the_base_object(
                            &spelled,
                            &name,
                            min + 1,
                            max + 1,
                        );
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
                    let (min, max) =
                        crate::classes::the_arity_of("object", &name).unwrap_or((0, 0));
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
                    return Some(Ok(Value::Int(chain.iter().any(|object| match object {
                        Value::Object(one) => Rc::ptr_eq(one, base),
                        _ => false,
                    }) as i64)));
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
                            Detail {
                                what: String::new(),
                                extra: String::new(),
                            },
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
                            Detail {
                                what: String::new(),
                                extra: String::new(),
                            },
                        )));
                    }
                    let mut pairs: Vec<(Value, Value)> = Vec::new();
                    for name in self.the_own_properties_of_a_prototype(target) {
                        let value = self.the_value_of_an_own_property_of_a_prototype(target, &name);
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
                        left.0
                            .to_text()
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
                    let value = match self.the_value_of_the_descriptor(&value, target, &name, line)
                    {
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
                let gone = object
                    .borrow_mut()
                    .properties
                    .remove(&name.to_lowercase())
                    .map(|property| property.value);
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // answer is the value of the property that is taken away,
                // and an empty text when there was none.
                Some(Ok(match gone {
                    None => Value::Text(String::new()),
                    Some(value) => {
                        match self.the_value_of_the_descriptor(&value, target, &name, line) {
                            Ok(Some(value)) => value,
                            _ => value,
                        }
                    }
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the `__Class`
        // of the v1 is a property of an object: a value that is not an
        // object has none, and the answer is the empty text. A class of the
        // script is named after itself.
        if !self.dialect.every_value_carries_its_class() {
            return self.class_of(value).unwrap_or_default();
        }
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
                return RuntimeFailure::Error(RuntimeError {
                    message: reason.to_string(),
                    line,
                });
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
    pub(crate) fn the_message_of_no_property(
        &self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> String {
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
    pub(crate) fn the_descriptor(
        &self,
        value: Option<&Value>,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the value that
        // is not a descriptor goes in the `Extra` of the failure.
        let invalid = || {
            RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Parameter #2 of Object.Prototype.DefineProp is invalid.".into(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: value.map(|value| value.to_text()).unwrap_or_default(),
                },
            )
        };
        let Some(Value::Object(object)) = value else {
            return Err(invalid());
        };
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
        let Value::Object(object) = value else {
            return false;
        };
        object.borrow().class == DESCRIPTOR
    }

    /// The members of a descriptor, when the value is one.
    pub(crate) fn the_members_of_a_descriptor(
        &self,
        value: &Value,
    ) -> Option<BTreeMap<String, Property>> {
        if !self.is_a_descriptor(value) {
            return None;
        }
        let Value::Object(object) = value else {
            return None;
        };
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
        let Value::Object(object) = value else {
            return Ok(None);
        };
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
        let Value::Object(object) = target else {
            return None;
        };
        let current = object.borrow().value_of(&name.to_lowercase())?;
        if !self.is_a_descriptor(&current) {
            return None;
        }
        let Value::Object(descriptor) = &current else {
            return None;
        };
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
        let Value::Object(object) = target else {
            return Ok(None);
        };
        let Some(current) = object.borrow().value_of(&name.to_lowercase()) else {
            return Ok(None);
        };
        if !self.is_a_descriptor(&current) {
            return Ok(None);
        }
        let Value::Object(descriptor) = &current else {
            return Ok(None);
        };
        let getter = descriptor.borrow().value_of("get");
        let Some(getter) = getter else {
            return Ok(None);
        };
        Ok(Some(self.call_the_function(&getter, target, &[], line)?))
    }

    /// Whether a value is a function of the script.
    fn is_a_function(&self, value: &Value) -> bool {
        let Value::Object(object) = value else {
            return false;
        };
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
        let Value::Object(object) = descriptor else {
            return Ok(());
        };
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
            RuntimeError {
                message: "Property is read-only.".to_string(),
                line,
            },
            Detail {
                what: String::new(),
                extra: written.to_string(),
            },
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
                return RuntimeFailure::Error(RuntimeError {
                    message: reason.to_string(),
                    line,
                });
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
        put(
            &mut properties,
            "name",
            Value::Text(self.the_name_of_the_method(class, method)),
        );
        // The first parameter of the reference is `this`.
        let required = method
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        put(
            &mut properties,
            "minparams",
            Value::Int(required as i64 + 1),
        );
        put(
            &mut properties,
            "maxparams",
            Value::Int(method.parameters.len() as i64 + 1),
        );
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(
            &mut properties,
            METHOD_CLASS,
            Value::Text(class.to_string()),
        );
        put(
            &mut properties,
            METHOD_NAME,
            Value::Text(method.name.clone()),
        );
        Value::Object(Rc::new(RefCell::new(Object {
            class: "func".to_string(),
            properties,
        })))
    }

    /// The value of a function that is bound to an object.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    /// `ObjBindMethod` hands over a value of the class `BoundFunc`, and the
    /// name of it is empty. The target is what the call of the value calls:
    /// the name of a method of the object, or the function that is bound.
    ///
    /// The v1 makes the binding too, and it does not check the name of the
    /// method: the `ObjBindMethod(t, "Nope")` is made, and the call of it
    /// fails.
    pub(crate) fn bound_function(&self, object: Value, target: Value) -> Value {
        let mut properties: BTreeMap<String, Property> = BTreeMap::new();
        put(&mut properties, "name", Value::Text(String::new()));
        put(&mut properties, "minparams", Value::Int(0));
        put(&mut properties, "maxparams", Value::Int(0));
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The object and the target: the call of the value calls them.
        put(&mut properties, BOUND, object);
        put(&mut properties, BOUND_TARGET, target);
        Value::Object(Rc::new(RefCell::new(Object {
            class: "boundfunc".to_string(),
            properties,
        })))
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
        put(
            &mut properties,
            "name",
            Value::Text(format!("{owner}.Prototype.{spelled}")),
        );
        put(&mut properties, "minparams", Value::Int(min));
        put(&mut properties, "maxparams", Value::Int(max));
        put(&mut properties, "isbuiltin", Value::Int(0));
        put(&mut properties, "isvariadic", Value::Int(0));
        // The class and the name of the method: the reference calls them.
        put(
            &mut properties,
            METHOD_CLASS,
            Value::Text(class.to_string()),
        );
        put(
            &mut properties,
            METHOD_NAME,
            Value::Text(spelled.to_string()),
        );
        Value::Object(Rc::new(RefCell::new(Object {
            class: "func".to_string(),
            properties,
        })))
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
                    return Err(crate::classes::the_arity_failure(
                        class, method, message, line,
                    ));
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

    /// The call of a function that is bound to an object.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    /// object is the first argument of the call of the function that is
    /// bound, and the arguments of the call are the rest.
    pub(crate) fn call_a_bound_function(
        &mut self,
        bound: &Value,
        target: &Value,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        match target {
            // The name of a method of the object: the method is looked for
            // when the call runs, and the object is its `this`.
            Value::Text(method) => self.call_a_method_of_an_object(bound, method, arguments, line),
            // The function that is bound: the object is its first argument.
            _ => {
                let mut all = Vec::with_capacity(arguments.len() + 1);
                all.push(bound.clone());
                all.extend_from_slice(arguments);
                let class = match target {
                    Value::Object(object) => object.borrow().class.clone(),
                    _ => String::new(),
                };
                match crate::classes::call_the_value(&class, target, &all, self, line) {
                    Some(result) => result,
                    None => Err(self.no_method(bound, &target.to_text(), line)),
                }
            }
        }
    }

    /// The call of a method of an object.
    ///
    /// The method is looked for when the call runs: the `ObjBindMethod` does
    /// not check the name of it when it makes the value.
    pub(crate) fn call_a_method_of_an_object(
        &mut self,
        target: &Value,
        method: &str,
        arguments: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        // A method of a class of the script.
        if let Some((defining, definition)) = self.find_method_on(target, method) {
            return self.call_method_with(&defining, &definition, target, arguments, line);
        }
        // A method of a class type of the runtime.
        let class = match target {
            Value::Object(object) => object.borrow().class.clone(),
            _ => String::new(),
        };
        if crate::classes::is_type(&class) {
            if let Some((min, max)) = crate::classes::the_arity_of(&class, method) {
                let count = arguments.len() as i64;
                if count < min || (max >= 0 && count > max) {
                    let message = if count < min {
                        "Too few parameters passed to function."
                    } else {
                        "Too many parameters passed to function."
                    };
                    return Err(crate::classes::the_arity_failure(
                        &class, method, message, line,
                    ));
                }
            }
            return match crate::classes::call(&class, method, target, arguments, self, line) {
                Some(result) => result,
                None => Err(self.no_method(target, method, line)),
            };
        }
        // A method of the base object.
        if crate::classes::the_base_object_has_the_method(&method.to_lowercase()) {
            return match self.base_method(target, &method.to_lowercase(), arguments, line) {
                Some(result) => result,
                None => Err(self.no_method(target, method, line)),
            };
        }
        Err(self.no_method(target, method, line))
    }

    /// A property of a value.
    pub(crate) fn read_property(
        &mut self,
        target: &Value,
        name: &str,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
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
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
            // not refuse the property of a value that is not an object, and
            // the answer is a blank text.
            if self.dialect.a_member_that_is_not_there_is_answered() {
                return Ok((Value::Text(String::new()), false));
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
            let property = crate::classes::the_property_of_the_prototype(class, &name)
                .map(|spelled| spelled.to_string());
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
            if let Some(answer) =
                self.the_value_of_the_descriptor(&value, target, &written, line)?
            {
                return Ok((answer, false));
            }
            return Ok((value, false));
        }
        // The static methods of a class are its own properties, and v2
        // hands them over as values.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: `Map.Call` is a
        // function whose name is "Map.Call".
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // class that a class holds is a member of the instance of it too:
        // the `o.D` is the class that C holds.
        if self.dialect.the_nested_class_is_a_member_of_the_instance() {
            if let Some(value) = self.the_class_that_the_v1_holds(target, &name) {
                return Ok((value, false));
            }
        }
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
                let reference = self.method_reference_of_a_class_type(&members, spelled, min, max);
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not refuse the property that an object does not have, and the
        // answer is a blank text.
        if self.dialect.a_member_that_is_not_there_is_answered() {
            return Ok((Value::Text(String::new()), false));
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
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 does not refuse the write of a property of a value that
            // is not an object: the `a.b := 5` is accepted when the `a` is
            // not an object, and the write is dropped.
            if self
                .dialect
                .the_write_of_a_property_of_a_value_that_is_not_an_object_is_dropped()
            {
                return Ok(());
            }
            // The class of a value that is not an object cannot be
            // written: v2 says that the property is read-only.
            if name == "__class" {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Property is read-only.".to_string(),
                        line,
                    },
                    Detail {
                        what: String::new(),
                        extra: written,
                    },
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
            return self
                .write_the_property_of_a_class_value(target, &class, &name, &written, value, line);
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
                    Detail {
                        what: "ObjSetBase".to_string(),
                        extra: value.to_text(),
                    },
                ));
            }
            let base = value.clone();
            return self.set_base_of(target, base, line).map_err(|error| {
                RuntimeFailure::Detailed(
                    error,
                    Detail {
                        what: "ObjSetBase".to_string(),
                        extra: String::new(),
                    },
                )
            });
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
                    RuntimeError {
                        message: "Property is read-only.".to_string(),
                        line,
                    },
                    Detail {
                        what: String::new(),
                        extra: written.clone(),
                    },
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
                RuntimeError {
                    message: "Property is read-only.".to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: written,
                },
            ));
        }
        if let Some(result) =
            crate::classes::set_property(&class, &name, target, value.clone(), self, line)
        {
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
        if self
            .the_static_methods_of_a_class_value(target)
            .contains(&name.to_string())
        {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Property is read-only.".to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: written.to_string(),
                },
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
                    Detail {
                        what: "ObjSetBase".to_string(),
                        extra: value.to_text(),
                    },
                ));
            }
            let base = value.clone();
            return self.set_base_of(target, base, line).map_err(|error| {
                RuntimeFailure::Detailed(
                    error,
                    Detail {
                        what: "ObjSetBase".to_string(),
                        extra: String::new(),
                    },
                )
            });
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
                    definition
                        .properties
                        .iter()
                        .any(|property| property.name.to_lowercase() == name)
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
                    definition
                        .properties
                        .iter()
                        .any(|property| !property.is_static && property.name.to_lowercase() == name)
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
            let Some(definition) = self.classes.get(&class) else {
                continue;
            };
            let Some(access) = definition.accessors.get(&lower) else {
                continue;
            };
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
        let Some(access) = self.the_accessor_of(target, name) else {
            return Ok(None);
        };
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
        let Some(access) = self.the_accessor_of(target, name) else {
            return Ok(false);
        };
        let Some(setter) = &access.setter else {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: "Property is read-only.".to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: name.to_string(),
                },
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
            body_line: body.line,
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

    /// The class that a value holds, as a value, the way the v1 reads it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// `new` of the v1 reads the class that the value holds: the `new o.D`
    /// is an instance of the class that the instance of C holds, and the
    /// `new C.D` is an instance of the class that the class holds.
    fn the_class_that_the_v1_holds(&self, target: &Value, name: &str) -> Option<Value> {
        // The class of the value: a class carries its own name, and an
        // instance carries the class of it.
        let class = match self.class_name_of(target) {
            Some(class) => class,
            None => match target {
                Value::Object(object) => object.borrow().class.clone(),
                _ => return None,
            },
        };
        let definition = self.classes.get(&class)?;
        let lower = name.to_lowercase();
        for nested in &definition.nested {
            if self.the_name_of_a_class(nested).to_lowercase() == lower {
                return self.class_value_of(nested);
            }
        }
        None
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
        match name {
            // There is no tray icon to hide, and the runner is one
            // process: both are already what the directive asks for.
            "notrayicon" => Ok("there is no tray icon to hide".into()),
            // Measured on the binary of v1.1.37.02 on 2026-11-01: the
            // directive `#NoEnv` turns the lookup of the environment off,
            // and the substitution of a parameter of the v1 is the variable
            // of the script alone.
            "noenv" => {
                self.no_env = true;
                Ok("the environment is not looked up".into())
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-01: the
            // script of the v1 stays alive on its own, and the runner is one
            // process that stays alive on its own.
            "persistent" => Ok("the runner is one process".into()),
            "singleinstance" => Ok("the runner is one process".into()),
            // The errors are always in the report, and the report is
            // always on the standard output.
            "errorstdout" => Ok("the errors are in the report".into()),
            // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
            // `#Warn` of the v2 takes the type of the warning and the mode of
            // it, and the warnings go to the report: the tree has no dialog.
            "warn" => Ok("the warnings are in the report".into()),
            "requires" => self.the_requires(argument, line),
            // The rest is what the table of the directives says: the reason
            // is the one of the category of the directive, and the table and
            // the run are the same truth.
            other => match refusal_of_directive(other) {
                Some(why) => Err(RuntimeFailure::Error(RuntimeError {
                    message: why.into(),
                    line,
                })),
                None => Err(RuntimeFailure::Error(RuntimeError {
                    message: format!("{other} is not a directive this runtime knows."),
                    line,
                })),
            },
        }
    }

    /// The `#Requires` directive: what the script says it needs to run.
    ///
    /// Measured on the binaries of both dialects: the argument names the
    /// interpreter and, when the script wants to be exact, the dialect it
    /// needs -- `v2` or `v1`. The name of the interpreter is compared without
    /// the case, and the dialect has to be the one this run speaks.
    ///
    /// An empty argument is refused with "Parameter #1 required", which is
    /// the wording of the binary, and a dialect the run does not speak is
    /// refused with the whole argument in the message and the dialect it is:
    /// "This script requires v1. This interpreter is v2.".
    ///
    /// # The line names the dialect and not the implementation
    ///
    /// The line is `#Requires v2`. It names the dialect, which is the fact a
    /// script actually depends on, and it needs no invented name for this
    /// tree: the project is called `ahk`, and a name made up for the directive
    /// would be a second name to keep in step with the first.
    ///
    /// The line still tells the two implementations apart, because mainline
    /// AutoHotkey reads the first word as the name of the interpreter:
    /// measured on the binary of v2.0.31, `#Requires v2` is refused with
    /// "This script requires v2. Current interpreter: AutoHotkey v2.0.31
    /// 64-bit.". A script that carries the line therefore runs here and not
    /// there.
    ///
    /// `#Requires AutoHotkey v2` is accepted as well, for a script that is
    /// meant to run in both: the mainline binary accepts that line too.
    fn the_requires(&self, argument: &str, line: u32) -> Result<String, RuntimeFailure> {
        let argument = argument.trim();
        if argument.is_empty() {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Parameter #1 required".into(),
                line,
            }));
        }
        // Two forms: the dialect alone, and the name of the interpreter before
        // it. The name is the mainline one and nothing else -- there is no
        // second name of this tree to write here.
        let mut words = argument.split_whitespace();
        let first = words.next().unwrap_or("");
        let second = words.next();
        let spare = words.next();
        let meets = spare.is_none() && {
            match second {
                // `#Requires v2` and `#Requires v1`.
                None => first.eq_ignore_ascii_case(self.dialect.name())
                    // `#Requires AutoHotkey` asks for the interpreter and not
                    // for a dialect, and the mainline binary accepts it.
                    || first.eq_ignore_ascii_case("autohotkey"),
                // `#Requires AutoHotkey v2`, which runs in both.
                Some(dialect) => {
                    first.eq_ignore_ascii_case("autohotkey")
                        && dialect.eq_ignore_ascii_case(self.dialect.name())
                }
            }
        };
        if meets {
            return Ok(format!(
                "this runtime is the {} dialect, and it meets {argument}",
                self.dialect.name()
            ));
        }
        Err(RuntimeFailure::Error(RuntimeError {
            message: format!(
                "This script requires {argument}. This interpreter is {}.",
                self.dialect.name()
            ),
            line,
        }))
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the
        // operators that write a variable of the v1 read the value that is
        // there as a number when it can be one -- the "5" is 5, and the
        // "0x10" is 16 -- and the text that is not a number is the number
        // zero: the `x += 1` of the "abc" is 1.
        if self.dialect.a_value_that_is_not_a_number_is_blank()
            && !matches!(op, AssignOp::Set | AssignOp::Concat)
        {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the compound write of a variable that has no value makes the
            // variable: the `zz += 2` is the two, and the `zz -= 2` is the
            // minus two. The value that is not there is the empty text, and
            // the empty text is the number zero.
            let current = match current {
                Some(value) => value,
                None => Value::Text(String::new()),
            };
            return self.the_value_of_the_write_of_the_v1(op, &current, &value, line);
        }
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

    /// The value that an operator that writes a variable writes, the way
    /// the v1 does it.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the values are
    /// numbers when they can be one -- the "5" is 5, and the "0x10" is 16
    /// -- and the text that is not a number is the number zero: the
    /// `x += 1` of the "abc" is 1. The whole numbers stay whole numbers,
    /// and the division of two whole numbers is a whole number: the
    /// `x /= 2` of the 5 is 2.
    fn the_value_of_the_write_of_the_v1(
        &self,
        op: AssignOp,
        current: &Value,
        value: &Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let a = self.the_number_of_a_value_of_the_v1(current);
        let b = self.the_number_of_a_value_of_the_v1(value);
        let one = a.to_number().unwrap_or(0.0);
        let other = b.to_number().unwrap_or(0.0);
        let whole = |one: &Value, other: &Value| match (one, other) {
            (Value::Int(one), Value::Int(other)) => Some((*one, *other)),
            _ => None,
        };
        Ok(match op {
            AssignOp::Add => match whole(&a, &b) {
                Some((one, other)) => Value::Int(one.wrapping_add(other)),
                None => Value::Float(one + other),
            },
            AssignOp::Subtract => match whole(&a, &b) {
                Some((one, other)) => Value::Int(one.wrapping_sub(other)),
                None => Value::Float(one - other),
            },
            AssignOp::Multiply => match whole(&a, &b) {
                Some((one, other)) => Value::Int(one.wrapping_mul(other)),
                None => Value::Float(one * other),
            },
            AssignOp::Divide => {
                if other == 0.0 {
                    // The division by zero of the v1 is a blank result and
                    // not a failure.
                    return Ok(Value::Text(String::new()));
                }
                match whole(&a, &b) {
                    Some((one, other)) => Value::Int(one.wrapping_div(other)),
                    None => Value::Float(one / other),
                }
            }
            AssignOp::IntegerDivide => {
                let other = self.the_whole_number_of_a_value_of_the_v1(&b);
                if other == 0 {
                    return Err(RuntimeFailure::Error(RuntimeError {
                        message: "Divide by zero.".into(),
                        line,
                    }));
                }
                Value::Int(
                    self.the_whole_number_of_a_value_of_the_v1(&a)
                        .wrapping_div(other),
                )
            }
            AssignOp::BitwiseAnd => Value::Int(
                self.the_whole_number_of_a_value_of_the_v1(&a)
                    & self.the_whole_number_of_a_value_of_the_v1(&b),
            ),
            AssignOp::BitwiseOr => Value::Int(
                self.the_whole_number_of_a_value_of_the_v1(&a)
                    | self.the_whole_number_of_a_value_of_the_v1(&b),
            ),
            AssignOp::BitwiseXor => Value::Int(
                self.the_whole_number_of_a_value_of_the_v1(&a)
                    ^ self.the_whole_number_of_a_value_of_the_v1(&b),
            ),
            AssignOp::ShiftLeft => Value::Int(
                self.the_whole_number_of_a_value_of_the_v1(&a)
                    .wrapping_shl(self.the_whole_number_of_a_value_of_the_v1(&b) as u32),
            ),
            AssignOp::ShiftRight => Value::Int(
                self.the_whole_number_of_a_value_of_the_v1(&a)
                    .wrapping_shr(self.the_whole_number_of_a_value_of_the_v1(&b) as u32),
            ),
            AssignOp::ShiftRightUnsigned => Value::Int(
                (self.the_whole_number_of_a_value_of_the_v1(&a) as u64
                    >> (self.the_whole_number_of_a_value_of_the_v1(&b) as u32 & 63))
                    as i64,
            ),
            AssignOp::Set | AssignOp::Concat => Value::Text(String::new()),
        })
    }

    /// The number of a value in the v1, for the operators that write.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the value of the
    /// v1 is a number when it can be one -- the "5" is 5, and the "0x10" is
    /// 16 -- and the text that is not a number is the number zero.
    fn the_number_of_a_value_of_the_v1(&self, value: &Value) -> Value {
        match value {
            Value::Int(number) => Value::Int(*number),
            Value::Float(number) => Value::Float(*number),
            other => match crate::v1::the_number_of_a_text(&other.to_text()) {
                Some(number) => number,
                None => Value::Int(0),
            },
        }
    }

    /// Whether the value is a number in the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// value of the v1 is a number when it can be one -- the "5" is 5 -- and
    /// the text that is not a number ("abc", "5abc" and the empty text) is
    /// not one at all.
    fn the_value_is_a_number_of_the_v1(&self, value: &Value) -> bool {
        match value {
            Value::Int(_) | Value::Float(_) => true,
            other => crate::v1::the_number_of_a_text(&other.to_text()).is_some(),
        }
    }

    /// The whole number of a value in the v1, for the operators that write.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the value of the
    /// v1 is a number when it can be one -- the "5" is 5, and the "0x10" is
    /// 16 -- and the text that is not a number is the number zero.
    fn the_whole_number_of_a_value_of_the_v1(&self, value: &Value) -> i64 {
        match value {
            Value::Int(number) => *number,
            Value::Float(number) => *number as i64,
            other => {
                crate::v1::the_whole_number_at_the_start_of_a_text(&other.to_text()).unwrap_or(0)
            }
        }
    }

    /// The reason the short names of the files are not answered.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `A_LoopFileShortName` is the name that Windows gives to a file that has
    /// a short one -- the "LONGNA~1.TXT" of a file that is called "long
    /// name.txt" -- and this box has none. The reason is there after a loop of
    /// the files has run: before it, the name has no value.
    fn the_reason_of_the_short_names_of_the_files(&self, name: &str) -> Option<&'static str> {
        if !self.the_short_names_of_the_files_are_refused {
            return None;
        }
        if name.eq_ignore_ascii_case("A_LoopFileShortName")
            || name.eq_ignore_ascii_case("A_LoopFileShortPath")
        {
            return Some(crate::variables::NO_SHORT_NAMES_OF_THE_FILES);
        }
        None
    }

    /// The value of a variable of the runtime, when it has none yet.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the variables of
    /// the loops are there before the first loop runs: the index is zero
    /// and the others are empty.
    fn the_value_of_a_variable_of_the_runtime(&self, name: &str) -> Option<Value> {
        let rest = crate::variables::spelled(name)?;
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // name of the function that runs is the name of it, and it is an
        // empty text when there is no function that runs.
        if rest == "thisfunc" {
            let name = self
                .frames
                .last()
                .map(|frame| frame.name.clone())
                .unwrap_or_default();
            return Some(Value::Text(name));
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // line of the statement that runs is the line of it, and the name has
        // no value before the first statement runs.
        if rest == "linenumber" {
            return Some(Value::Text(String::new()));
        }
        if !THE_LOOP_VARIABLES.contains(&rest.as_str()) {
            return None;
        }
        if rest == "index" {
            Some(Value::Int(0))
        } else {
            Some(Value::Text(String::new()))
        }
    }

    /// The value of the index of the turn, and the way to give it back.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: every loop saves the
    /// value of `A_Index` when it starts and gives it back when it ends, and
    /// a loop that is inside another one does not disturb the index of the
    /// outer one. A name that had no value before the loop has no value
    /// after it either.
    fn the_index_is_saved(&self) -> Option<Value> {
        self.variables.get(&the_key_of("A_Index")).cloned()
    }

    /// The index of the turn, given back the way it was.
    fn the_index_is_given_back(&mut self, saved: Option<Value>) {
        match saved {
            Some(value) => self.set_global("A_Index", value),
            None => {
                self.variables.remove(&the_key_of("A_Index"));
            }
        }
    }

    /// The value of the index of the turn, as a number.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the index of a
    /// counted loop is the value of `A_Index`, and the body may write it:
    /// the loop goes on from the value that was written.
    fn the_index_of_the_turn(&self) -> i64 {
        self.variables
            .get(&the_key_of("A_Index"))
            .and_then(|value| value.to_number())
            .unwrap_or(0.0) as i64
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
                RuntimeFailure::Error(RuntimeError {
                    message: "Invalid index.".into(),
                    line,
                })
            }),
            // A text has no item, and v2 refuses the index of one:
            // `SubStr` is the way to take a letter out.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 does not refuse the index of a value that is not an
            // object, and the answer is a blank text.
            _ => {
                if self.dialect.a_member_that_is_not_there_is_answered() {
                    return Ok(Value::Text(String::new()));
                }
                Err(self.no_item(target, line))
            }
        }
    }

    /// The failure of a variable that has no value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// variable goes in the `Extra` of the error, and the `What` is empty.
    fn the_unassigned_failure(&self, name: &str, line: u32) -> RuntimeFailure {
        RuntimeFailure::Detailed(
            RuntimeError {
                message: self.the_unassigned(name),
                line,
            },
            Detail {
                what: String::new(),
                extra: name.to_string(),
            },
        )
    }

    /// The failure of a parameter that has no value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// parameter is in the `Extra` of the failure, and the `What` is empty.
    fn the_missing_parameter(&self, name: &str, line: u32) -> RuntimeFailure {
        RuntimeFailure::Detailed(
            RuntimeError {
                message: "Missing a required parameter.".into(),
                line,
            },
            Detail {
                what: String::new(),
                extra: name.to_string(),
            },
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
                Detail {
                    what: String::new(),
                    extra: the_extra_of_the_value(value),
                },
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
                Detail {
                    what: String::new(),
                    extra: the_extra_of_the_value(value),
                },
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
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
    /// refuse the object, and the text of it is the empty text -- the
    /// concatenation of an object and of "a" is "a".
    fn the_text_of_the_concat(&self, value: &Value, line: u32) -> Result<String, RuntimeFailure> {
        match value {
            Value::Text(text) => Ok(text.clone()),
            // The numbers of the v1 are formatted by the `SetFormat` when
            // they become a text.
            Value::Int(_) | Value::Float(_) => Ok(self.the_text_of(value)),
            Value::Object(_) if self.dialect.an_object_has_no_text() => Ok(String::new()),
            other => Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: expected_the_kind(TypeName::String, &self.the_type_of(other)),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: String::new(),
                },
            )),
        }
    }

    /// The number of a value in the v1, when it can be one.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text that is
    /// written between quotes is a string, and it is never a number. The
    /// value of a variable is a number when it can be one: "5" is 5, and
    /// "0x10" is 16, and " 5 " is 5.
    fn the_number_of_the_v1(&self, node: &Node, value: &Value) -> Option<Value> {
        match value {
            Value::Int(number) => Some(Value::Int(*number)),
            Value::Float(number) => Some(Value::Float(*number)),
            Value::Text(text) => {
                if self.the_node_is_a_quoted_text(node) {
                    return None;
                }
                crate::v1::the_number_of_a_text(text)
            }
            _ => None,
        }
    }

    /// Whether the node is a text that is written between quotes.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text that is
    /// written between quotes is a string, and the arithmetic of the v1
    /// never turns it into a number: `"5" + 1` is "51".
    fn the_node_is_a_quoted_text(&self, node: &Node) -> bool {
        matches!(
            node,
            Node::Literal {
                value: LiteralValue::Text(_),
                ..
            }
        )
    }

    /// The comparison of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the comparison is
    /// a number when both sides are numbers, and a text otherwise: "abc" > 1
    /// is true, and "abc" < 1 is false. The sense of the case says whether
    /// the letters of the texts are the same.
    fn the_comparison_of_the_v1(
        &self,
        left_node: &Node,
        right_node: &Node,
        left: &Value,
        right: &Value,
        op: &BinaryOp,
    ) -> bool {
        let numbers = match (
            self.the_number_of_the_v1(left_node, left),
            self.the_number_of_the_v1(right_node, right),
        ) {
            (Some(left), Some(right)) => Some((
                left.to_number().unwrap_or(0.0),
                right.to_number().unwrap_or(0.0),
            )),
            _ => None,
        };
        self.the_order_of_the_v1(numbers, &left.to_text(), &right.to_text(), op)
    }

    /// The order of the v1: the numbers when both sides are numbers, and the
    /// texts otherwise.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// comparison is a number when both sides are numbers, and a text
    /// otherwise: "abc" > 1 is true, and "abc" < 1 is false. The sense of the
    /// case says whether the letters of the texts are the same.
    fn the_order_of_the_v1(
        &self,
        numbers: Option<(f64, f64)>,
        left: &str,
        right: &str,
        op: &BinaryOp,
    ) -> bool {
        match numbers {
            Some((left, right)) => match op {
                BinaryOp::Less => left < right,
                BinaryOp::Greater => left > right,
                BinaryOp::LessEqual => left <= right,
                BinaryOp::GreaterEqual => left >= right,
                _ => false,
            },
            // The texts are compared, and the sense of the case says whether
            // the letters of them are the same.
            None => {
                let order = the_order_of_the_texts(left, right, self.case_sense.the_method());
                match op {
                    BinaryOp::Less => order == core::cmp::Ordering::Less,
                    BinaryOp::Greater => order == core::cmp::Ordering::Greater,
                    BinaryOp::LessEqual => order != core::cmp::Ordering::Greater,
                    BinaryOp::GreaterEqual => order != core::cmp::Ordering::Less,
                    _ => false,
                }
            }
        }
    }

    /// The value of a variable of the v1 that has no value.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: a
    /// variable of the v1 that has no value is blank, and the variable
    /// that is named "0" is the exception: it holds the text "0". The
    /// `if 0 = 0` is true because of it, and the `0 := 9` turns the
    /// `if 0 = 0` into a false condition.
    fn the_value_of_an_unset_variable(&self, name: &str) -> Value {
        if name == "0" {
            return Value::Text(String::from("0"));
        }
        Value::Text(String::new())
    }

    /// The comparison of the legacy `if` of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
    ///
    /// - The right side of the comparison is the text of the rest of the
    ///   line, and the text goes through the legacy substitution: the
    ///   `if x = %y%` compares the value of `x` with the value of `y`.
    /// - The `=` and the `!=` ignore the case, and they are a number when
    ///   both sides are numbers: the `if x = 1` is true when the value of
    ///   `x` is the number 1, and the `if x = 01` is true as well.
    /// - The order compares the numbers when both sides are numbers, and the
    ///   texts otherwise.
    ///
    /// The legacy `if` has no strict comparison: the `==` of the v2 is the
    /// `=` of the legacy `if`, and the second equal sign is a part of the
    /// text.
    fn the_comparison_of_a_legacy_if(
        &mut self,
        left: &Node,
        op: BinaryOp,
        text: &str,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let left_value = self.evaluate(left)?;
        // The text of the right side is the text of the script, and the
        // `%name%` of it is the value of the variable of that name.
        let right_text = self.the_substitution_of_the_v1(text, line)?;
        // The number of the right side: the text is a number when it can be
        // one, the way the value of a variable is.
        let right_number = crate::v1::the_number_of_a_text(&right_text);
        let left_number = self.the_number_of_the_v1(left, &left_value);
        let answer = match op {
            BinaryOp::Equal | BinaryOp::NotEqual => {
                // The equal sign of the legacy `if` follows the sense of the
                // case, and the not equal sign is its opposite.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the equal sign of the legacy `if` is the
                // comparison of the sense of the case, and not the one of the
                // equal sign of an expression: the `if x = abc` is false when
                // the sense of the case is On and the value of `x` is "ABC".
                let method = self.case_sense.the_method();
                let same = match (left_number, right_number) {
                    (Some(one), Some(other)) => the_same_number(&one, &other),
                    _ => the_same_text(&left_value.to_text(), &right_text, method),
                };
                match op {
                    BinaryOp::Equal => same,
                    _ => !same,
                }
            }
            _ => {
                // The order of the v1.
                let numbers = match (left_number, right_number) {
                    (Some(one), Some(other)) => Some((
                        one.to_number().unwrap_or(0.0),
                        other.to_number().unwrap_or(0.0),
                    )),
                    _ => None,
                };
                self.the_order_of_the_v1(numbers, &left_value.to_text(), &right_text, &op)
            }
        };
        Ok(Value::Int(answer as i64))
    }

    /// The forms of the legacy `if` of the v1 that are not a comparison:
    /// the `between`, the `in` and the `contains`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
    ///
    /// - The `between` compares the numbers when the value and the two
    ///   bounds are numbers, and the texts otherwise. The bounds are in
    ///   the range: the `if x between 1 and 3` holds when the value of
    ///   `x` is 1.
    /// - The `in` compares the text of the value with the text of the
    ///   item, and the comparison is always a comparison of texts: the
    ///   "11" does not match the item "11.0".
    /// - The `contains` asks whether the text of the value has the text
    ///   of the item in it.
    /// - The texts of the form go through the legacy substitution, and
    ///   the sense of the case says whether the letters of them are the
    ///   same.
    fn the_match_of_a_legacy_if(
        &mut self,
        left: &Node,
        form: LegacyForm,
        texts: &[String],
        negated: bool,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let left_value = self.evaluate(left)?;
        let text_of_the_value = left_value.to_text();
        // The texts of the form are the texts of the script, and the
        // `%name%` of them is the value of the variable of that name.
        let mut items = Vec::new();
        for text in texts {
            items.push(self.the_substitution_of_the_v1(text, line)?);
        }
        let holds = match form {
            LegacyForm::Between => {
                let low = items.first().map(String::as_str).unwrap_or_default();
                let high = items.get(1).map(String::as_str).unwrap_or_default();
                // The numbers when the value and the two bounds are
                // numbers, and the texts otherwise.
                let numbers = match (
                    self.the_number_of_the_v1(left, &left_value),
                    crate::v1::the_number_of_a_text(low),
                    crate::v1::the_number_of_a_text(high),
                ) {
                    (Some(value), Some(low), Some(high)) => Some((
                        value.to_number().unwrap_or(0.0),
                        low.to_number().unwrap_or(0.0),
                        high.to_number().unwrap_or(0.0),
                    )),
                    _ => None,
                };
                match numbers {
                    Some((value, low, high)) => value >= low && value <= high,
                    None => {
                        // The order of the texts, and the bounds are in
                        // the range.
                        let method = self.case_sense.the_method();
                        let at_the_low = the_order_of_the_texts(&text_of_the_value, low, method);
                        let at_the_high = the_order_of_the_texts(&text_of_the_value, high, method);
                        at_the_low != core::cmp::Ordering::Less
                            && at_the_high != core::cmp::Ordering::Greater
                    }
                }
            }
            LegacyForm::In => {
                // The comparison of the texts follows the sense of the case.
                let method = self.case_sense.the_method();
                items
                    .iter()
                    .any(|item| the_same_text(&text_of_the_value, item, method))
            }
            LegacyForm::Contains => {
                let method = self.case_sense.the_method();
                items
                    .iter()
                    .any(|item| the_text_contains(&text_of_the_value, item, method))
            }
        };
        Ok(Value::Int(if negated { !holds } else { holds } as i64))
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-02: the v1 does not let a script write a built-in
                // variable, and the increment of one changes nothing: the
                // value is the empty text, and the built-in keeps the value
                // that it had.
                if self
                    .dialect
                    .the_increment_of_a_built_in_variable_changes_nothing(name)
                {
                    return Ok(Value::Text(String::new()));
                }
                let current = match self.lookup(name) {
                    Some(current) => current,
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the increment of a variable of the v1 that
                    // has no value is the increment of the blank: the `0++`
                    // writes the variable "0" with the value 1.
                    None if self.dialect.unset_is_blank() => {
                        self.the_value_of_an_unset_variable(name)
                    }
                    None => return Err(self.the_unassigned_failure(name, line)),
                };
                let stored = self.the_number_after(&current, step, line)?;
                self.assign(name, stored.clone());
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the increment
            // of a property and of an item writes to them.
            Node::Property {
                target,
                name,
                line: property_line,
            } => {
                let target_value = self.evaluate(target)?;
                let name = self.the_name_of_the_member(name, *property_line)?;
                let current = self.read_property(&target_value, &name, *property_line)?;
                let stored = self.the_number_after(&current, step, *property_line)?;
                self.write_a_property(
                    target,
                    &name,
                    AssignOp::Set,
                    stored.clone(),
                    *property_line,
                )?;
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            Node::Index {
                target,
                index,
                line: item_line,
            } => {
                let target_value = self.evaluate(target)?;
                let key = self.evaluate(index)?;
                let current = self.index_a_value(&target_value, &key, *item_line)?;
                let stored = self.the_number_after(&current, step, *item_line)?;
                self.set_item(
                    &target_value,
                    &key,
                    AssignOp::Set,
                    stored.clone(),
                    *item_line,
                )?;
                Ok(match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => stored,
                    _ => current,
                })
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the increment of a literal of the v1 is the empty text: the
            // `x := 5++` is a script that runs, and the value of the
            // expression is the empty text.
            Node::Literal { .. } if self.dialect.the_increment_of_a_literal_is_blank() => {
                Ok(Value::Text(String::new()))
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the value of
        // the v1 is a number when it can be one -- the "5" is 5 -- and the
        // whole number stays a whole number: the `x++` of the "5" is 6.
        //
        // Measured on the binary of v1.1.37.02 on 2026-11-04: the increment
        // reads the value that is there as a number, the way the operators
        // that write a variable read it: the text that is not a number is the
        // number zero, and the `a++` of the "abc" is one, and the `a++` of a
        // variable that has no value is one too.
        if self.dialect.a_value_that_is_not_a_number_is_blank() {
            return Ok(match self.the_number_of_a_value_of_the_v1(current) {
                Value::Int(number) => Value::Int(number.wrapping_add(step)),
                Value::Float(number) => Value::Float(number + step as f64),
                other => other,
            });
        }
        Ok(match current {
            Value::Int(number) => Value::Int(number.wrapping_add(step)),
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
        //
        // The v1 has no class types: the name `Func` of it is a function,
        // and not a class. Measured on the binary of v1.1.37.02 on
        // 2026-11-02.
        if self.classes.contains_key(&name.to_lowercase())
            || self.dialect.carries_the_class_of_the_runtime(name)
        {
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 takes the name of a built-in as the name of a variable, and the
        // call still goes to the built-in: `Abs := 5` and then `Abs(3)`
        // answers 3.
        if self.dialect.the_call_of_a_built_in_wins_over_the_variable()
            && self.builtin(name).is_some()
        {
            return self.call_a_builtin_by_name(name, arguments, line);
        }
        // The value of the name is what the call asks for: a variable of
        // the script, or a built-in variable of the dialect.
        let from_the_script = self.lookup_variable(name);
        let value = match from_the_script {
            Some(value) => Some(Ok(value)),
            None => self.the_built_in_variable(name),
        };
        // An object that is called is asked through its `Call` method:
        // v2 does not ask the `__Call` of the class when the object
        // itself is called.
        if let Some(value) = value {
            let value = value.map_err(|error| {
                RuntimeFailure::Error(RuntimeError {
                    message: error.message,
                    line,
                })
            })?;
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
        self.call_a_builtin_by_name(name, arguments, line)
    }

    /// The call of a built-in: the arguments are read, and the built-in
    /// answers.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the number of the
    /// arguments is checked before the arguments are evaluated: v2 refuses
    /// the script when it loads, and the refusal of the arity comes before
    /// the failure of an argument.
    fn call_a_builtin_by_name(
        &mut self,
        name: &str,
        arguments: &[Node],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Some(builtin) =
            self.the_builtin_of_the_kind(name, crate::builtins::BuiltinKind::Function)
        else {
            return Err(self.the_unassigned_failure(name, line));
        };
        // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
        // arguments is checked before the arguments are evaluated: v2
        // refuses the script when it loads, and the refusal of the arity
        // comes before the failure of an argument.
        self.the_arity_of_a_builtin(builtin, arguments.len(), line, Route::Written)?;
        let mut values = Vec::with_capacity(arguments.len());
        // The v1 has functions that ask for the name of a variable and not
        // for its value: `IsSet(x)` asks whether the variable `x` has a
        // value. Measured on the binary of v1.1.37.02 on 2026-11-02.
        let wants_the_name = self
            .dialect
            .the_place_of_the_argument_that_wants_the_name(builtin.name);
        // The v1 has functions that ask for the variable itself: the
        // `VarSetCapacity(y, 100)` empties `y`, and the `StrReplace` of the
        // v1 writes the number of the replacements into the variable of its
        // fourth argument. Measured on the binary of v1.1.37.02 on
        // 2026-11-02. The argument that is not a variable is a value, and
        // the built-in has nowhere to write into.
        let wants_the_variable = self
            .dialect
            .the_place_of_the_argument_that_wants_the_variable(builtin.name);
        for (index, argument) in arguments.iter().enumerate() {
            if wants_the_variable == Some(index) {
                if let Node::Var { name, .. } = argument {
                    values.push(Value::Reference(Reference {
                        name: name.clone(),
                        scope: self.the_scope_of(name),
                        alias: true,
                    }));
                    continue;
                }
            }
            if wants_the_name == Some(index) {
                if let Node::Var { name, .. } = argument {
                    values.push(Value::Text(name.clone()));
                    continue;
                }
            }
            values.push(self.evaluate(argument)?);
        }
        self.call_builtin(name, &values, line, Route::Written)
    }

    /// The number of the arguments of the call of a built-in.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-29: the arity is in the
    /// table, so the check is the same for every built-in and it says what
    /// AutoHotkey says.
    fn the_arity_of_a_builtin(
        &self,
        builtin: &crate::builtins::Builtin,
        count: usize,
        line: u32,
        route: Route,
    ) -> Result<(), RuntimeFailure> {
        if count < builtin.min as usize {
            return Err(self.the_arity_failure(
                "Too few parameters passed to function.",
                builtin.name,
                route,
                line,
            ));
        }
        // The table says 255 when the built-in takes any number of them.
        if builtin.max < 255 && count > builtin.max as usize {
            return Err(self.the_arity_failure(
                "Too many parameters passed to function.",
                builtin.name,
                route,
                line,
            ));
        }
        Ok(())
    }

    /// Call a function the script defined.
    fn call_function(
        &mut self,
        function: &Function,
        arguments: &[Node],
        line: u32,
        route: Route,
    ) -> Result<Value, RuntimeFailure> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
        // arguments is checked before the arguments are evaluated: v2
        // refuses the script when it loads, and the refusal of the arity
        // comes before the failure of an argument.
        self.the_arity_of_a_call(function, arguments.len(), line, route)?;
        let values = self.evaluate_all(arguments)?;
        // The parameters that the command does not give a value to are the
        // ones that are not there: the place of them is kept, and they have
        // no value.
        let missing: Vec<bool> = arguments
            .iter()
            .map(|argument| matches!(argument, Node::Empty { .. }))
            .collect();
        self.call_function_with(function, &values, line, route, &missing)
    }

    /// The number of the arguments of a call, checked the way v2 checks it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-29: a call that the
    /// script wrote is checked when the script loads, and the failure names
    /// the function: "Too few parameters passed to function.\n     Specifically:
    /// Abs". A call that goes through a value is checked when the call runs,
    /// and it says so in other words: the name of the parameter that is
    /// missing goes in the `Extra`.
    fn the_arity_of_a_call(
        &self,
        function: &Function,
        count: usize,
        line: u32,
        route: Route,
    ) -> Result<(), RuntimeFailure> {
        // The arity of a function is in its definition: the parameters that
        // have no default are needed, and the ones that are there are the
        // most it takes unless one of them takes the rest.
        let needed = function
            .parameters
            .iter()
            .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
            .count();
        if count < needed {
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
                .nth(count)
                .map(|parameter| parameter.name.clone())
                .unwrap_or_default();
            return Err(self.the_missing_parameter(&missing, line));
        }
        let variadic = function
            .parameters
            .iter()
            .any(|parameter| parameter.variadic);
        if !variadic && count > function.parameters.len() {
            return Err(self.the_arity_failure(
                "Too many parameters passed to function.",
                &function.name,
                route,
                line,
            ));
        }
        Ok(())
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
        self.the_arity_of_a_call(function, values.len(), line, route)?;
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
        if let Some(index) = function
            .parameters
            .iter()
            .position(|parameter| parameter.variadic)
        {
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
        self.frames.push(Frame {
            name: function.name.clone(),
            line,
        });
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the concat on a
        // built-in variable of the v1 is accepted, and the built-in keeps the
        // value that it had: the write goes nowhere. The rule is asked here,
        // because the settings of the run are written where they live, and
        // the settings of the v2 are built-in variables of the v1.
        if self
            .dialect
            .the_write_of_a_built_in_variable_goes_nowhere(name, op)
        {
            return Ok(value);
        }
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
        match self.the_write_of_the_built_in_variable(name, op, &value, line) {
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
                    None => match self.the_built_in_variable(name) {
                        Some(Ok(value)) => Some(value),
                        // A name that the runtime owns has a value before the
                        // first loop runs: the index of the turn is zero.
                        // Measured on the binary of v2.0.19 on 2026-11-01,
                        // the compound operator reads the value that is
                        // there, and it does not say that the variable has
                        // no value.
                        _ => self.the_value_of_a_variable_of_the_runtime(name),
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not write the property when the value that is there is not
        // a number: the `a.b += 2` of the "abc" leaves the property as it
        // was.
        if self
            .dialect
            .the_compound_write_of_a_thing_that_is_not_a_number_is_dropped()
            && !matches!(op, AssignOp::Set | AssignOp::Concat)
        {
            if let Some(current) = &current {
                if !self.the_value_is_a_number_of_the_v1(current) {
                    return Ok(current.clone());
                }
            }
        }
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
        let Value::Object(object) = &value else {
            return None;
        };
        let object = object.borrow();
        if object.class != "func" {
            return None;
        }
        // A closure is kept under a name of its own.
        if let Some(closure) = object.value_of(CLOSURE) {
            return self
                .functions
                .get(&closure.to_text().to_lowercase())
                .cloned();
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
        let Value::Object(object) = &value else {
            return None;
        };
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
                RuntimeError {
                    message: message.to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: name.to_string(),
                },
            ),
            Route::ThroughTheCall => RuntimeFailure::Detailed(
                RuntimeError {
                    message: message.to_string(),
                    line,
                },
                // The call went through the `Call` method of a value, and
                // v2 names that method in the `What`.
                Detail {
                    what: "Func.Prototype.Call".to_string(),
                    extra: name.to_string(),
                },
            ),
        }
    }

    /// The legacy command of the v1: `Msg, text`.
    ///
    /// The parameters of a command of the v1 are texts, and the shape of
    /// the command says what each one of them is: the value of an
    /// expression, the name of a variable of the script, or the name of
    /// the variable that the command writes into.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: `StringLen, n,
    /// hello` is five when `hello` is a variable of the script, and zero
    /// when it is not: the second parameter of that command is the *name*
    /// of a variable.
    pub(crate) fn the_legacy_command(
        &mut self,
        name: &str,
        parameters: &[crate::ast::LegacyParameter],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Some(builtin) =
            self.the_builtin_of_the_kind(name, crate::builtins::BuiltinKind::Command)
        else {
            // The name is not a command of the dialect: the line holds no
            // action, and it is named.
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!(
                    "This line does not contain a recognized action.\n     Specifically: {name}"
                ),
                line,
            }));
        };
        let values = self.the_parameters_of_a_legacy_command(builtin, parameters, line)?;
        self.call_the_row(builtin, &values, line, Route::Written)
    }

    /// The values of the parameters of a command of the v1.
    ///
    /// The shape of the command says what the text of each one of them is.
    pub(crate) fn the_parameters_of_a_legacy_command(
        &mut self,
        builtin: &'static crate::builtins::Builtin,
        parameters: &[crate::ast::LegacyParameter],
        line: u32,
    ) -> Result<Vec<Value>, RuntimeFailure> {
        let mut values = Vec::with_capacity(parameters.len());
        for (index, parameter) in parameters.iter().enumerate() {
            values.push(self.the_parameter_of_a_legacy_command(builtin, index, parameter, line)?);
        }
        Ok(values)
    }

    /// The condition of a command of the v1 that has a body.
    ///
    /// The row of the command answers the condition, and the shape of the
    /// command says what the text of each parameter of it is.
    fn the_condition_of_a_legacy_if(
        &mut self,
        name: &str,
        parameters: &[String],
        line: u32,
    ) -> Result<bool, RuntimeFailure> {
        let Some(builtin) = self.builtin(name) else {
            return Ok(false);
        };
        // The command of the v1 that has a body takes texts: the haystack
        // and the needle are texts.
        let parameters: Vec<crate::ast::LegacyParameter> = parameters
            .iter()
            .map(|text| crate::ast::LegacyParameter::Text(text.clone()))
            .collect();
        let values = self.the_parameters_of_a_legacy_command(builtin, &parameters, line)?;
        let answer = self.call_the_row(builtin, &values, line, Route::Written)?;
        Ok(answer.truthy())
    }

    /// The value of a parameter of a command of the v1.
    ///
    /// The shape of the command says what the text of the parameter is.
    fn the_parameter_of_a_legacy_command(
        &mut self,
        builtin: &'static crate::builtins::Builtin,
        index: usize,
        parameter: &crate::ast::LegacyParameter,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        match builtin.parameters.get(index) {
            // The name of a variable of the script: the command writes into
            // it, and the reference is the alias of that variable.
            Some(crate::builtins::Parameter::Output) => {
                let name = the_text_of_the_parameter(parameter);
                let scope = self.the_scope_of(&name);
                Ok(Value::Reference(Reference {
                    name,
                    scope,
                    alias: true,
                }))
            }
            // The name of a variable of the script: the value of it is the
            // parameter.
            Some(crate::builtins::Parameter::Variable) => {
                let name = the_text_of_the_parameter(parameter);
                match self.lookup(&name) {
                    Some(value) => Ok(value),
                    // Measured on the binary of v1.1.37.02 on 2026-11-01: a
                    // variable that has no value is blank, and it is not a
                    // failure.
                    None if self.dialect.unset_is_blank() => Ok(Value::Text(String::new())),
                    None => Err(self.the_unassigned_failure(&name, line)),
                }
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the
            // parameter that wants a number is an expression: the `Sleep,
            // 100+50` waits for a hundred and fifty milliseconds.
            _ => match parameter {
                crate::ast::LegacyParameter::Expression(node) => self.evaluate(node),
                // The text of the parameter, with the substitution of the v1.
                crate::ast::LegacyParameter::Text(text) => {
                    Ok(Value::Text(self.the_substitution_of_the_v1(text, line)?))
                }
            },
        }
    }

    /// The count of the loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// count of the `Loop` of the v1 is the text that the script wrote, with
    /// the substitution of the v1, and the loop is skipped when the text is
    /// not a number: the `loop n` is skipped.
    fn the_count_of_the_loop(&mut self, node: &Node, line: u32) -> Result<i64, RuntimeFailure> {
        if self.dialect.the_count_of_the_loop_is_a_number() {
            if let Node::Literal {
                value: crate::ast::LiteralValue::Text(text),
                ..
            } = node
            {
                let text = self.the_substitution_of_the_v1(text, line)?;
                return Ok(crate::v1::the_count_of_the_loop(&text));
            }
        }
        Ok(self.evaluate(node)?.to_number().unwrap_or(0.0) as i64)
    }

    /// The pattern of the files of the v1, when the loop is a loop of files.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// text of the first parameter of the `Loop` of the v1 is the count of the
    /// turns when it is a number and when it is the only parameter, and it is
    /// the pattern of the files when it is not. With more than one parameter,
    /// the v1 reads it always as the pattern: the `Loop, 3, *.txt` is a loop
    /// of the files that match the "3", and it has no turn.
    fn the_pattern_of_the_files_of_the_v1(
        &mut self,
        node: &Node,
        arguments: &[Node],
        line: u32,
    ) -> Result<Option<String>, RuntimeFailure> {
        let text = match node {
            Node::Literal {
                value: crate::ast::LiteralValue::Text(text),
                ..
            } => text,
            _ => return Ok(None),
        };
        let text = self
            .the_substitution_of_the_v1(text, line)?
            .trim()
            .to_string();
        if arguments.len() == 1 && crate::v1::the_text_is_the_count_of_a_loop(&text) {
            return Ok(None);
        }
        Ok(Some(text))
    }

    /// The mode of the loop of the files of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the mode
    /// is a number, and the v1 refuses the one that is not a zero, a one or a
    /// two when the script loads. The text that carries the name of a variable
    /// is read when the loop runs, and the value that is not one of the three
    /// is a failure then.
    fn the_mode_of_the_loop_of_the_files_of_the_v1(
        &mut self,
        arguments: &[Node],
        line: u32,
    ) -> Result<u8, RuntimeFailure> {
        let Some(node) = arguments.get(1) else {
            return Ok(crate::v1::THE_FILES);
        };
        let text = self.the_text_of_the_parameter_of_the_loop(node, line)?;
        match crate::v1::the_mode_of_the_loop_of_the_files(&text) {
            Some(mode) => Ok(mode),
            None => Err(RuntimeFailure::Error(RuntimeError {
                message: format!("Parameter #2 invalid.\n     Specifically: {}", text.trim()),
                line,
            })),
        }
    }

    /// The flag of the recursion of the loop of the files of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the flag
    /// is a zero or a one, and the one asks for the folders of the files. The
    /// value that is not a zero or a one is not a failure when the loop runs:
    /// the loop does not walk into the folders.
    fn the_flag_of_the_recursion_of_the_v1(
        &mut self,
        arguments: &[Node],
        line: u32,
    ) -> Result<bool, RuntimeFailure> {
        let Some(node) = arguments.get(2) else {
            return Ok(false);
        };
        let text = self.the_text_of_the_parameter_of_the_loop(node, line)?;
        Ok(crate::v1::the_flag_asks_for_the_recursion(&text))
    }

    /// The text of a parameter of the loop of the v1.
    ///
    /// The parameters of the loop of the v1 are texts, and the text of them is
    /// read with the substitution of the v1.
    fn the_text_of_the_parameter_of_the_loop(
        &mut self,
        node: &Node,
        line: u32,
    ) -> Result<String, RuntimeFailure> {
        match node {
            Node::Literal {
                value: crate::ast::LiteralValue::Text(text),
                ..
            } => Ok(self.the_substitution_of_the_v1(text, line)?),
            _ => Ok(self.evaluate(node)?.to_text()),
        }
    }

    /// The loop of the files of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `Loop` of the v1 walks over the files that match the pattern, and it
    /// gives the name of the file, its extension, the folder it is in and the
    /// path of it to the script. The name keeps its extension, and the path is
    /// the one that is relative to the folder the script runs in.
    fn the_loop_of_the_files_of_the_v1(
        &mut self,
        pattern: &str,
        arguments: &[Node],
        body: &[Node],
        until: &Option<Box<Node>>,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let mode = self.the_mode_of_the_loop_of_the_files_of_the_v1(arguments, line)?;
        let recurse = self.the_flag_of_the_recursion_of_the_v1(arguments, line)?;
        // The mode of the desktop is the one of the v2: the "R" walks into the
        // folders, the "D" gives the folders alone, and the "A" gives the
        // files and the folders.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // flag of the recursion walks into the folders when the mode is the
        // files, and it does nothing when the mode is the folders or the two
        // of them.
        let mut wanted = String::new();
        match mode {
            crate::v1::THE_FOLDERS => wanted.push('D'),
            crate::v1::THE_FILES_AND_THE_FOLDERS => wanted.push('A'),
            _ => {}
        }
        if recurse && mode == crate::v1::THE_FILES {
            wanted.push('R');
        }
        self.the_walk_of_the_files_of_the_v1(pattern, &wanted, body, until, line)
    }

    /// The loop of the files of the v1 that names the kind of the loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `Loop Files` of the v1 is the `Loop` whose first parameter is the word
    /// "Files": the pattern of the files is the second parameter and the mode
    /// is the third one. The mode is a set of letters -- the "R" walks into
    /// the folders, the "D" and the "K" give the folders, the "F" and the "V"
    /// give the files -- and the spaces and the tabs between them are ignored.
    /// The letter that is not one of them is refused when the loop runs, and
    /// the "A" is not one of them.
    fn the_loop_of_the_files_of_the_v1_with_the_kind(
        &mut self,
        arguments: &[Node],
        body: &[Node],
        until: &Option<Box<Node>>,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let pattern = match arguments.get(1) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        let mode = match arguments.get(2) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        if the_letter_that_the_mode_does_not_carry(&mode).is_some() {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!("Parameter #3 invalid.\n     Specifically: {}", mode.trim()),
                line,
            }));
        }
        self.the_walk_of_the_files_of_the_v1(&pattern, &mode, body, until, line)
    }

    /// The walk of the files of the v1.
    ///
    /// The mode is the mode of the desktop: the "R" walks into the folders,
    /// the "D" and the "K" give the folders, the "F" and the "V" give the
    /// files, and the "A" gives the two of them.
    fn the_walk_of_the_files_of_the_v1(
        &mut self,
        pattern: &str,
        mode: &str,
        body: &[Node],
        until: &Option<Box<Node>>,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let folder = self.desktop().working_directory().map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        })?;
        let files = self
            .desktop()
            .list_files(&folder, pattern, mode)
            .map_err(|reason| {
                RuntimeFailure::Error(RuntimeError {
                    message: reason,
                    line,
                })
            })?;
        for (index, file) in files.iter().enumerate() {
            self.set_global("A_Index", Value::Int(index as i64 + 1));
            let name = the_name_of(file).to_string();
            let path = the_path_that_is_relative_to(file, &folder);
            self.set_global("A_LoopFileName", Value::Text(name.clone()));
            self.set_global(
                "A_LoopFileExt",
                Value::Text(the_extension_of(&name).to_string()),
            );
            self.set_global(
                "A_LoopFileDir",
                Value::Text(the_folder_of(&path).to_string()),
            );
            self.set_global("A_LoopFilePath", Value::Text(path.clone()));
            // The short names of the files are a thing of Windows, and this
            // box has none.
            self.the_short_names_of_the_files_are_refused = true;
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the `A_LoopFileFullPath` of the v1 is the path of the file that
            // is relative to the folder the script runs in, and not the long
            // one: the name of the v1 is a mistake that the v1 keeps.
            self.set_global("A_LoopFileFullPath", Value::Text(path));
            self.set_global("A_LoopFileLongPath", Value::Text(file.clone()));
            // The size of the file, and the times of it. Measured on the
            // binary of AutoHotkey v1.1.37.02 on 2026-11-05: the size is the
            // number of the bytes of the file, the kilobytes and the megabytes
            // are the parts of it, and the times are the fourteen digits of
            // the local time of the machine.
            //
            // The variable that the machine cannot answer is left out, and the
            // script that reads it is told that it has no value.
            if let Ok(size) = self.desktop().the_size_of_a_file(file) {
                self.set_global("A_LoopFileSize", Value::Int(size));
                self.set_global("A_LoopFileSizeKB", Value::Int(size / 1024));
                self.set_global("A_LoopFileSizeMB", Value::Int(size / 1024 / 1024));
            }
            if let Ok(letters) = self.desktop().file_attributes(file) {
                self.set_global("A_LoopFileAttrib", Value::Text(letters.concat()));
            }
            if let Ok(times) = self.desktop().file_times(file) {
                let offset = crate::files::local_offset(self);
                self.set_global(
                    "A_LoopFileTimeCreated",
                    Value::Text(crate::files::stamp_of(times.0, offset)),
                );
                self.set_global(
                    "A_LoopFileTimeModified",
                    Value::Text(crate::files::stamp_of(times.1, offset)),
                );
                self.set_global(
                    "A_LoopFileTimeAccessed",
                    Value::Text(crate::files::stamp_of(times.2, offset)),
                );
            }
            match self.run_block(body) {
                Ok(()) => {}
                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                Err(RuntimeFailure::Flow(Flow::Continue)) => {}
                Err(other) => return Err(other),
            }
            // The `until` of the v1 is the condition of the loop, and it is
            // read after every turn of the body.
            if self.the_until_ends_the_loop(until)? {
                break;
            }
        }
        Ok(())
    }

    /// The read loop of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `Loop, Read, file` reads the file and gives its lines in
    /// `A_LoopReadLine`, one turn for each line, and the number of the line
    /// is in `A_Index`. The file that is not there is not a failure: the loop
    /// has no turn. The second parameter is the file where the `FileAppend`
    /// that has no name of its own writes, and the third one is not carried.
    fn the_read_loop_of_the_v1(
        &mut self,
        arguments: &[Node],
        body: &[Node],
        until: &Option<Box<Node>>,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let path = match arguments.get(1) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        let output = match arguments.get(2) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // file that is not there is not a failure, and the loop has no turn.
        let lines = self.desktop().read_file(&path).unwrap_or_default();
        self.loop_output_files.push(output);
        let mut failure = None;
        for (index, line_text) in lines.iter().enumerate() {
            self.set_global("A_Index", Value::Int(index as i64 + 1));
            self.set_global("A_LoopReadLine", Value::Text(line_text.clone()));
            match self.run_block(body) {
                Ok(()) => {}
                Err(RuntimeFailure::Flow(Flow::Break)) => break,
                Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                Err(other) => {
                    failure = Some(other);
                    break;
                }
            }
            // The `until` of the v1 is the condition of the loop, and it is
            // read after every turn of the body.
            if self.the_until_ends_the_loop(until)? {
                break;
            }
        }
        self.loop_output_files.pop();
        if let Some(other) = failure {
            return Err(other);
        }
        Ok(())
    }

    /// The parse loop of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// `Loop, Parse, name, delimiters` separates the text of the variable
    /// whose name is the second parameter, and every character of the
    /// delimiters is a delimiter of its own. The field is in `A_LoopField`
    /// and its number is in `A_Index`. The delimiters that are not there make
    /// every character of the text a field of its own, and the variable that
    /// has no value is a blank text: the loop has no turn.
    fn the_parse_loop_of_the_v1(
        &mut self,
        arguments: &[Node],
        body: &[Node],
        until: &Option<Box<Node>>,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        // The text that is separated is the value of the variable, or the
        // value of the expression when the percent sign and a space are in
        // front of the parameter.
        let text = match arguments.get(1) {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the percent sign and a space in front of the parameter make it
            // an expression, and the value of it is the text that is
            // separated.
            Some(Node::LegacyExpression { value, .. }) => self.evaluate(value)?.to_text(),
            Some(Node::Literal {
                value: crate::ast::LiteralValue::Text(text),
                ..
            }) => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
                // the name of the variable is the text of the parameter, and
                // the substitution of the v1 may bring the name from another
                // variable: the `Loop, Parse, %x%, "|"` separates the text of
                // the variable that the value of `x` names.
                let name = self.the_substitution_of_the_v1(text.trim(), line)?;
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
                // the name that is not the name of a variable is not a failure:
                // the v1 looks for it, does not find it, and the loop has no
                // turn.
                match self.lookup(&name) {
                    Some(value) => value.to_text(),
                    None => String::new(),
                }
            }
            Some(node) => self.evaluate(node)?.to_text(),
            None => String::new(),
        };
        let delimiters = match arguments.get(2) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // word "CSV" asks for the comma separated value format, and the case
        // of it and the spaces around it do not matter.
        let comma_separated =
            crate::v1::the_delimiters_ask_for_the_comma_separated_values(&delimiters);
        // The characters that are not wanted at the ends of a field.
        let omitted = match arguments.get(3) {
            Some(node) => self.the_text_of_the_parameter_of_the_loop(node, line)?,
            None => String::new(),
        };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // text that is empty has no field, and the loop has no turn: the
        // variable that has no value is a blank text.
        if text.is_empty() {
            return Ok(());
        }
        let pieces: Vec<String> = if comma_separated {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the comma separated value format does not use the characters
            // that are left out.
            crate::v1::the_fields_of_the_comma_separated_values(&text)
        } else if delimiters.is_empty() {
            // The delimiters that are not there make every character of the
            // text a field of its own, and the characters that are left out
            // are not seen at all.
            text.chars()
                .filter(|character| !omitted.contains(*character))
                .map(|character| character.to_string())
                .collect()
        } else {
            split_on_any(&text, &delimiters)
                .into_iter()
                .map(|piece| {
                    piece
                        .trim_matches(|character| omitted.contains(character))
                        .to_string()
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
            // The `until` of the v1 is the condition of the loop, and it is
            // read after every turn of the body.
            if self.the_until_ends_the_loop(until)? {
                break;
            }
        }
        Ok(())
    }

    /// The `until` of the v1 ends the loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `until` of the v1 is the statement that comes right after the body of
    /// a loop, and it is the condition that ends it: the body runs, and the
    /// condition is read after every turn of it. The condition is an
    /// expression, and the `=` of the v1 is an assignment: the `until i = 2`
    /// writes the variable "i" and the condition is true.
    ///
    /// The v2 has no such statement, and the loop has no condition.
    fn the_until_ends_the_loop(
        &mut self,
        until: &Option<Box<Node>>,
    ) -> Result<bool, RuntimeFailure> {
        let Some(condition) = until else {
            return Ok(false);
        };
        let value = self.evaluate(condition)?;
        Ok(self.the_value_is_true(&value))
    }

    /// The substitution of the parameters of a command of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: `%name%` is the
    /// value of the variable of the script when it has one, and the value
    /// of the environment otherwise. A `%` that does not close a name is
    /// left alone.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-04: the escape of the
    /// v1 is expanded here, and the character that it carries is a literal:
    /// the percent sign that it carries does not open the name of a variable,
    /// and the percent sign that closes a name is not the one that the
    /// escape carries.
    pub(crate) fn the_substitution_of_the_v1(
        &mut self,
        text: &str,
        line: u32,
    ) -> Result<String, RuntimeFailure> {
        if !text.contains('%') && !text.contains('`') {
            return Ok(text.to_string());
        }
        let mut out = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let mut index = 0usize;
        while index < chars.len() {
            if chars[index] == '`' {
                match chars.get(index + 1) {
                    Some(next) => {
                        out.push(self.dialect.the_escape_of_the_dialect(*next));
                        index += 2;
                    }
                    None => {
                        out.push('`');
                        index += 1;
                    }
                }
                continue;
            }
            if chars[index] == '%' {
                if let Some(close) = self.the_closing_percent(&chars, index + 1) {
                    let name: String = chars[index + 1..close].iter().collect();
                    match self.the_value_of_the_substitution(name.trim(), line)? {
                        Some(value) => {
                            // The numbers of the v1 are formatted by the
                            // `SetFormat` when they become a text.
                            let text = self.the_text_of(&value);
                            out.push_str(&text);
                            index = close + 1;
                            continue;
                        }
                        None => {
                            // Measured on the binary of v1.1.37.02 on
                            // 2026-11-01: a name that has no value is
                            // substituted by nothing, and not by the text
                            // of the name.
                            index = close + 1;
                            continue;
                        }
                    }
                }
                out.extend(&chars[index..]);
                break;
            }
            out.push(chars[index]);
            index += 1;
        }
        Ok(out)
    }

    /// The place of the percent sign that closes the name of a variable.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-04: the escape of the
    /// v1 makes the character that follows it a literal, and the percent sign
    /// that it carries does not close the name of a variable.
    fn the_closing_percent(&self, chars: &[char], from: usize) -> Option<usize> {
        let mut index = from;
        while index < chars.len() {
            match chars[index] {
                '`' => index += 2,
                '%' => return Some(index),
                _ => index += 1,
            }
        }
        None
    }

    /// The value of the legacy assignment of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02:
    ///
    /// - The text of the right side is the text of the script, and the
    ///   `%name%` of it is the value of the variable of that name.
    /// - The `AutoTrim` trims the text of the assignment: with it On,
    ///   `x = %z%` leaves `x` at the value of `z` without the spaces that
    ///   come before it and the spaces that come after it, and with it Off
    ///   the spaces are there.
    /// - The text is a whole number when it is one in base ten: `x = 5`
    ///   leaves `x` at the number 5, and `x = 5.5` leaves it at the text
    ///   "5.5", and `x = 0x10` leaves it at the text "0x10".
    fn the_value_of_a_legacy_assignment(
        &mut self,
        text: &str,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let text = self.the_substitution_of_the_v1(text, line)?;
        // The `AutoTrim` of the v1 trims the text of the assignment. The
        // parameters of a command are not trimmed by it.
        let text = if self.auto_trim {
            text.trim().to_string()
        } else {
            text
        };
        Ok(crate::v1::the_number_of_a_legacy_text(&text).unwrap_or(Value::Text(text)))
    }

    /// The value of the substitution of a parameter of the v1.
    ///
    /// The variable of the script comes first, and the environment is what
    /// is left. The lookup of the environment is the one of the machine.
    fn the_value_of_the_substitution(
        &mut self,
        name: &str,
        line: u32,
    ) -> Result<Option<Value>, RuntimeFailure> {
        if let Some(value) = self.lookup(name) {
            return Ok(Some(value));
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-04: the
        // variable that is named "0" holds the text "0" when it has no
        // value.
        if self.dialect.unset_is_blank() && name == "0" {
            return Ok(Some(self.the_value_of_an_unset_variable(name)));
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the built-in
        // variable of the v1 is a value too: `%A_AutoTrim%` is "On".
        if let Some(answer) = self.the_built_in_variable(name) {
            return match answer {
                Ok(value) => Ok(Some(value)),
                Err(error) => Err(RuntimeFailure::Error(RuntimeError {
                    message: error.message,
                    line: if error.line == 0 { line } else { error.line },
                })),
            };
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // short names of the files are a thing of Windows, and this box has
        // none: the script that reads the name is told that.
        if let Some(reason) = self.the_reason_of_the_short_names_of_the_files(name) {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: reason.to_string(),
                line,
            }));
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // variables that the runtime owns are values too: the `%A_Index%` is
        // "0" outside a loop, and the tree answered an empty text, because
        // the substitution of the v1 did not ask the runtime for them.
        if let Some(value) = self.the_value_of_a_variable_of_the_runtime(name) {
            return Ok(Some(value));
        }
        if !self.dialect.percent_looks_at_the_environment() {
            return Ok(None);
        }
        // The directive `#NoEnv` of the v1 turns the lookup of the
        // environment off.
        if self.no_env {
            return Ok(None);
        }
        match self.desktop().env_get(name) {
            Ok(Some(value)) => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: the environment variable that is accessed is
                // warned about.
                self.warn_about_the_environment_that_is_accessed(name, line);
                Ok(Some(Value::Text(value)))
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-01: a name
            // that the environment does not have is substituted by nothing.
            Ok(None) => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: the name that is not a variable of the script
                // and not of the environment is a variable that has no value.
                self.warn_about_the_variable_that_has_no_value(name, line);
                Ok(None)
            }
            Err(_) => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: the name that is not a variable of the script
                // and not of the environment is a variable that has no value.
                self.warn_about_the_variable_that_has_no_value(name, line);
                Ok(None)
            }
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
        let Some(builtin) =
            self.the_builtin_of_the_kind(name, crate::builtins::BuiltinKind::Function)
        else {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: "Call to nonexistent function.".into(),
                line,
            }));
        };
        self.call_the_row(builtin, values, line, route)
    }

    /// The call of the built-in of this row.
    ///
    /// The row is the one of the dialect that runs, and its column of the
    /// shape says what each parameter is: a value, a reference that the
    /// caller passes, the name of a variable of the script, or the name of
    /// the variable that the built-in writes into.
    fn call_the_row(
        &mut self,
        builtin: &'static crate::builtins::Builtin,
        values: &[Value],
        line: u32,
        route: Route,
    ) -> Result<Value, RuntimeFailure> {
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
        // The table says 255 when the built-in takes any number of them, and
        // the v1 has no complaint for a command that takes more parameters
        // than it needs.
        if builtin.max < 255 && values.len() > builtin.max as usize {
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
        //
        // The column of the shape says which parameters are written back:
        // the reference of the v2, and the output of the v1.
        // The built-in that says itself which parameters take a reference
        // asks in its own handler.
        //
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not check the kind of the arguments -- the built-in of the v1
        // takes the text that holds a number, and it does not fail -- so the
        // two checks below are the ones of the v2.
        let checks_the_kind = self.dialect.the_built_in_checks_the_kind_of_its_arguments();
        // The v1 does not check the kind of the arguments, and the parameter
        // that wants a number takes the number that the text of it starts
        // with: the dialect converts the arguments before the body of the
        // built-in runs.
        //
        // Measured on the binary of v1.1.37.02 on 2026-11-02.
        let converted = if checks_the_kind {
            None
        } else {
            self.the_numbers_that_the_arguments_hold(builtin.name, values)
        };
        let values = converted.as_deref().unwrap_or(values);
        let decides = crate::builtins::decides_references(builtin.name);
        for (index, value) in values.iter().enumerate() {
            if !matches!(value, Value::Reference(_))
                || builtin.writes_back(index)
                || decides
                || crate::builtins::refuses_itself(builtin.name)
                || !checks_the_kind
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
                crate::builtins::Wants::Checked(kind) => expects_a_kind(kind, value),
                // A parameter that wants a reference takes one.
                crate::builtins::Wants::Reference(_) => continue,
                crate::builtins::Wants::Value => continue,
            };
            // The name of the built-in goes in the `What` of the
            // failure: v2 carries it there.
            return Err(RuntimeFailure::Detailed(
                RuntimeError { message, line },
                Detail {
                    what: builtin.name.to_string(),
                    extra: String::new(),
                },
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
            if matches!(value, Value::Reference(_)) || !checks_the_kind {
                continue;
            }
            let message = match crate::builtins::wants(builtin.name, index) {
                crate::builtins::Wants::Declared(kind) => {
                    if accepts(kind, value) {
                        continue;
                    }
                    // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                    // v1 does not refuse an object where a parameter wants a
                    // text -- the text of the object is the empty text, and
                    // `StrLen(Object())` is zero.
                    if kind == TypeName::String
                        && matches!(value, Value::Object(_))
                        && self.dialect.an_object_has_no_text()
                    {
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
                Detail {
                    what: builtin.name.to_string(),
                    extra: the_extra_of_the_value(value),
                },
            ));
        }
        // A command may declare fewer parameters than it needs: the table
        // of the binary says how many the command takes, and the command
        // says which one it cannot do without. The refusal of v2 names
        // the parameter and not the count, and it comes after the kind of
        // the values that are there: the dialect converts the arguments
        // before the body of the function runs.
        for number in self
            .dialect
            .the_parameters_that_may_not_be_left_out(builtin.name)
        {
            if values.len() < *number as usize {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: format!(
                            "Parameter #{number} of {} must not be omitted in this case.",
                            builtin.name
                        ),
                        line,
                    },
                    // The name of the built-in goes in the `What` of the
                    // failure, the way it goes in the failure of a value
                    // that is not the one that was wanted.
                    //
                    // Measured on the binary of v2.0.19 on 2026-11-01.
                    Detail {
                        what: builtin.name.to_string(),
                        extra: String::new(),
                    },
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
                Detail {
                    what: builtin.name.to_string(),
                    extra,
                },
            )
        })
    }

    /// The numbers that the arguments of the built-in hold.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
    /// check the kind of the arguments, and the parameter that wants a number
    /// takes the number that the text of it starts with -- "65abc" is 65 --
    /// and the hex of the v1 is a number: "0x10" is 16.
    ///
    /// The answer is nothing when every argument that wants a number already
    /// holds one.
    fn the_numbers_that_the_arguments_hold(
        &self,
        name: &str,
        values: &[Value],
    ) -> Option<Vec<Value>> {
        let mut numbers: Option<Vec<Value>> = None;
        for (index, value) in values.iter().enumerate() {
            if !self.dialect.the_parameter_wants_a_number(name, index) {
                continue;
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the parameter that keeps its text is not a number, and the
            // command reads the text of it: the amount of the `EnvAdd` that
            // is "5abc" is not a number.
            if self.dialect.the_parameter_keeps_its_text(name, index) {
                continue;
            }
            // A number is the number that it is.
            if matches!(value, Value::Int(_) | Value::Float(_)) {
                continue;
            }
            let number = crate::v1::the_number_at_the_start_of_a_text(&self.the_text_of(value))
                .unwrap_or(Value::Int(0));
            let numbers = numbers.get_or_insert_with(|| values.to_vec());
            numbers[index] = number;
        }
        numbers
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

    /// The file of the file-reading loop that runs, when it has one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of the
    /// file of a `FileAppend` may be left out inside the body of a `Loop
    /// Read` that has a file of its own, and the text goes to that file.
    pub(crate) fn the_output_file_of_the_loop(&self) -> Option<&str> {
        self.loop_output_files
            .last()
            .map(|path| path.as_str())
            .filter(|path| !path.is_empty())
    }

    /// The file of the loop of the files that runs.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// name of the file of the commands of the files of the v1 may be left
    /// out, and the file of the loop that runs is the one that is used.
    pub(crate) fn the_file_of_the_loop(&self) -> Option<String> {
        self.lookup("A_LoopFileLongPath")
            .map(|value| value.to_text())
            .filter(|path| !path.is_empty())
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
        match self
            .exit_callbacks
            .iter_mut()
            .find(|callback| callback.name.eq_ignore_ascii_case(name))
        {
            Some(callback) => callback.line = line,
            None => self.exit_callbacks.push(Callback {
                name: name.to_string(),
                line,
            }),
        }
    }

    /// The function the script asks to run when a failure is not caught.
    pub(crate) fn set_error_callback(&mut self, name: &str, line: u32) {
        match self
            .error_callbacks
            .iter_mut()
            .find(|callback| callback.name.eq_ignore_ascii_case(name))
        {
            Some(callback) => callback.line = line,
            None => self.error_callbacks.push(Callback {
                name: name.to_string(),
                line,
            }),
        }
    }

    /// The callback of the exit that has this name is not called any more.
    pub(crate) fn remove_exit_callback(&mut self, name: &str) {
        self.exit_callbacks
            .retain(|callback| !callback.name.eq_ignore_ascii_case(name));
    }

    /// The callback of the error that has this name is not called any more.
    pub(crate) fn remove_error_callback(&mut self, name: &str) {
        self.error_callbacks
            .retain(|callback| !callback.name.eq_ignore_ascii_case(name));
    }

    /// Put a callback before the ones that are registered, or after them.
    pub(crate) fn order_exit_callback(&mut self, name: &str, before: bool) {
        let Some(at) = self
            .exit_callbacks
            .iter()
            .position(|callback| callback.name.eq_ignore_ascii_case(name))
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
        let Some(at) = self
            .error_callbacks
            .iter()
            .position(|callback| callback.name.eq_ignore_ascii_case(name))
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
                message: format!(
                    "Parameter #1 of {name} requires an Object, but received nothing."
                ),
                line,
            });
        };
        // A value that is not an object is refused with the kind of the
        // value it is.
        let Value::Object(object) = value else {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(1, name, "an Object", value),
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
                let variadic = function
                    .parameters
                    .iter()
                    .any(|parameter| parameter.variadic);
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
            return Err(RuntimeError {
                message: "Invalid callback function.".into(),
                line,
            });
        }
        Ok(Callback {
            name: function_name,
            line,
        })
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
                self.error_value(&RuntimeError {
                    message: timeout.to_string(),
                    line: 0,
                }),
                "ExitApp",
            ),
            _ => return TheError::Goes,
        };
        let mut answer = 0;
        let callbacks = self.error_callbacks.clone();
        for callback in callbacks {
            let values = [value.clone(), Value::Text(mode.to_string())];
            // A function that fails does not hide the failure.
            if let Ok(said) =
                self.call_named(&callback.name, &values, callback.line, Route::Written)
            {
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
    pub(crate) fn call_function_by_name(
        &mut self,
        name: &str,
        line: u32,
    ) -> Result<(), RuntimeFailure> {
        let Some(function) = self.functions.get(&name.to_lowercase()).cloned() else {
            return Ok(());
        };
        self.call_function(&function, &[], line, Route::Written)
            .map(|_| ())
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

    /// Whether the value of the operand is true in the dialect of the run.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text that was written between quotes is a string of the v1, and a
    /// string is true when it is not empty: the `!("00")` is zero. The value
    /// of a variable is a number when it can be one, and the `!x` with `x`
    /// holding "00" is one.
    fn the_operand_is_true(&self, node: &Node, value: &Value) -> bool {
        if self.dialect.the_text_that_was_written_is_a_string()
            && self.the_node_is_a_quoted_text(node)
        {
            return match value {
                Value::Text(text) => !text.is_empty(),
                other => other.truthy(),
            };
        }
        self.the_value_is_true(value)
    }

    /// The answer of the `&&` and of the `||`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `&&` and the `||` of the v1 answer one and zero, and not the operand:
    /// the `"a" && "b"` is 1. The v2 answers the operand.
    fn the_answer_of_the_boolean(&self, node: &Node, value: &Value) -> Value {
        if self.dialect.the_operators_answer_a_number() {
            return Value::Int(self.the_operand_is_true(node, value) as i64);
        }
        value.clone()
    }

    /// Whether the value is true in the dialect of the run.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text of a condition of the v1 is a number when the syntax of the v1
    /// accepts it, and the condition is false when that number is zero: the
    /// "00" is false, and the "0.0" is false. The v2 keeps the text as it
    /// was written: the "00" is true, and only the text "0" is false.
    fn the_value_is_true(&self, value: &Value) -> bool {
        if self.dialect.the_text_that_is_a_number_is_false() {
            if let Value::Text(text) = value {
                return match crate::v1::the_number_of_a_text(text) {
                    Some(number) => number.truthy(),
                    None => !text.is_empty(),
                };
            }
        }
        value.truthy()
    }

    /// The value of the condition of a construct.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not report the failure of the condition of an `if` or of a
    /// `while`: the `if 1/0`, the `if x.y` and the `if x.y := 5` are
    /// accepted, the condition is false, and the body of the construct does
    /// not run.
    ///
    /// The failure that a throw carries, the failure that no catch takes and
    /// the timeout are not the failure of a condition, and they are reported.
    fn the_value_of_the_condition(&mut self, condition: &Node) -> Result<Value, RuntimeFailure> {
        match self.evaluate(condition) {
            Ok(value) => Ok(value),
            Err(failure) => {
                if !self.dialect.the_failure_of_a_condition_is_not_reported() {
                    return Err(failure);
                }
                if !matches!(
                    failure,
                    RuntimeFailure::Error(_) | RuntimeFailure::Detailed(_, _)
                ) {
                    return Err(failure);
                }
                Ok(Value::Int(0))
            }
        }
    }

    fn evaluate(&mut self, node: &Node) -> Result<Value, RuntimeFailure> {
        match node {
            Node::Unset { .. } => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the value of the expression whose operand is
                // not there is the empty text.
                Ok(Value::Text(String::new()))
            }
            Node::Assign {
                name,
                op,
                value,
                line,
            } => {
                // The assignment of v2 is an expression, and the value of
                // it is the value that was written.
                let value = self.evaluate(value)?;
                self.write_a_variable(name, *op, value, *line)
            }
            Node::LegacyComparison {
                left,
                op,
                text,
                line,
            } => {
                // The comparison of the legacy `if` of the v1: the right side
                // of it is the text of the rest of the line.
                self.the_comparison_of_a_legacy_if(left, *op, text, *line)
            }
            Node::LegacyMatch {
                left,
                form,
                texts,
                negated,
                line,
            } => {
                // The forms of the legacy `if` of the v1 that are not a
                // comparison: the `between`, the `in` and the `contains`.
                self.the_match_of_a_legacy_if(left, *form, texts, *negated, *line)
            }
            Node::AssignProperty {
                target,
                name,
                op,
                value,
                line,
            } => {
                let value = self.evaluate(value)?;
                let name = self.the_name_of_the_member(name, *line)?;
                self.write_a_property(target, &name, *op, value, *line)
            }
            Node::AssignIndex {
                target,
                index,
                op,
                value,
                line,
            } => {
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
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // number of the v1 is a "pure number": it keeps the text
                // that was written, and that text is the value of it. The
                // whole number that does not fit in the machine is the
                // one that fits, and the number that is a number and not
                // a pure number follows the format of the dialect.
                LiteralValue::Number(text) => {
                    let text = crate::v1::the_text_of_the_number_that_fits(text);
                    if crate::v1::the_number_is_a_number(&text) {
                        return Ok(match crate::v1::the_number_of_a_text(&text) {
                            Some(Value::Int(number)) => {
                                Value::Text(self.integer_format.the_text_of(number))
                            }
                            Some(Value::Float(number)) => {
                                Value::Text(self.float_format.the_text_of(number))
                            }
                            _ => Value::Text(text),
                        });
                    }
                    Value::Text(text)
                }
                // A string is read where it is written: the names it
                // holds are the names of the scope that writes it.
                LiteralValue::Text(text) => Value::Text(self.interpolate(text, *line)?),
                LiteralValue::Bool(flag) => Value::Int(*flag as i64),
            }),
            Node::ObjectLiteral { items, line } => {
                // The literal of an object of v2: an object of the base
                // class with the properties it writes. The names of the
                // properties do not know their case, and a name that the
                // script builds is the name of the property.
                //
                // The v1 has no classes, and the object of it has no class
                // either: measured on the binary of v1.1.37.02 on
                // 2026-11-02, the `__Class` of the object of the v1 is
                // empty.
                let object = crate::classes::object(self.dialect.class_of_an_object());
                if let Value::Object(cell) = &object {
                    for item in items {
                        match item {
                            ObjectItem::Property(name, value) => {
                                let name = self.the_name_of_the_member(name, *line)?;
                                let value = self.evaluate(value)?;
                                put(&mut cell.borrow_mut().properties, &name, value);
                            }
                            ObjectItem::LegacyKey(key, value) => {
                                // Measured on the binary of AutoHotkey
                                // v1.1.37.02 on 2026-11-04: the key of the v1
                                // is the value of the expression that is
                                // written until the colon: the `{1 + 1: 1}`
                                // has the property "2", and the `{a.b: 1}`
                                // has the property of the value of `a.b`.
                                let key = self.evaluate(key)?;
                                let value = self.evaluate(value)?;
                                put(&mut cell.borrow_mut().properties, &key.to_text(), value);
                            }
                            ObjectItem::Value(value) => {
                                // Measured on the binary of AutoHotkey
                                // v1.1.37.02 on 2026-11-04: a value that has
                                // no name is evaluated, and it is dropped: the
                                // `a := {, 1}` is an object of no property.
                                self.evaluate(value)?;
                            }
                        }
                    }
                }
                Ok(object)
            }
            Node::Lambda {
                parameters,
                body,
                line,
            } => {
                // The arrow function of v2: a function that closes over the
                // scope where it was written, and whose body is an
                // expression.
                let function = Function {
                    name: String::new(),
                    parameters: parameters.clone(),
                    body: vec![Node::Return {
                        value: Some(Box::new((**body).clone())),
                        line: *line,
                    }],
                    declarations: Declarations::default(),
                    closure: self.scopes.clone(),
                };
                let name = self.make_a_closure(function.clone());
                Ok(closure_value(&name, &function))
            }
            Node::ArrayLiteral { items, line } => {
                // The literal is the array the `Array` built-in makes:
                // the values go in, in the order they are written. The hole
                // of the literal is a place of the array that has no value
                // of its own.
                let mut values = Vec::new();
                for item in items {
                    match item {
                        Some(item) => values.push(Some(self.evaluate(item)?)),
                        None => values.push(None),
                    }
                }
                Ok(crate::classes::array::construct_with_holes(
                    &values, self, *line,
                ))
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
            Node::LegacyControl { condition, .. } => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the construct that comes inside the line of the
                // condition of an `if` has no body of its own, and the value
                // of it is the value of its condition.
                self.evaluate(condition)
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
                None => match self.the_built_in_variable(name) {
                    Some(Ok(value)) => Ok(value),
                    Some(Err(error)) => Err(RuntimeFailure::Error(RuntimeError {
                        message: error.message,
                        line: *line,
                    })),
                    None => {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the short names of the files are a thing
                        // of Windows, and this box has none: the script that
                        // reads the name is told that, and the tree does not
                        // make up a name for the file.
                        if let Some(reason) = self.the_reason_of_the_short_names_of_the_files(name)
                        {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: reason.to_string(),
                                line: *line,
                            }));
                        }
                        match self.the_value_of_a_variable_of_the_runtime(name) {
                            Some(value) => Ok(value),
                            None => {
                                // Measured on the binary of AutoHotkey v2.0.19 on
                                // 2026-11-05: the warning of the variable that
                                // is read and never written is given before
                                // the failure of the run.
                                self.warn_about_the_variable_that_has_no_value(name, *line);
                                // Measured on the binary of v1.1.37.02 on
                                // 2026-11-01: a variable of the v1 that has no
                                // value is blank, and it is not a failure.
                                if self.dialect.unset_is_blank() {
                                    return Ok(self.the_value_of_an_unset_variable(name));
                                }
                                Err(match self.alias_of(name) {
                                    // A parameter that is an alias says that it is
                                    // the parameter that has no value.
                                    Some(_) => RuntimeFailure::Error(RuntimeError {
                                        message: unassigned_parameter(),
                                        line: *line,
                                    }),
                                    None => self.the_unassigned_failure(name, *line),
                                })
                            }
                        }
                    }
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
                    Ok(Value::Int(!self.the_operand_is_true(operand, &value) as i64))
                }
                // The word that is an operator and that has no left operand.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the word takes the value of the right side, and
                // it answers the truth of it as a number: the `x := and 2` is
                // one, and the `x := or 2` is one.
                UnaryOp::Boolean => {
                    let value = self.evaluate(operand)?;
                    Ok(self.the_answer_of_the_boolean(operand, &value))
                }
                // A whole number stays a whole number: `-3` is the
                // integer -3, the way AutoHotkey v2 has it.
                UnaryOp::Negate => {
                    let value = self.evaluate(operand)?;
                    // Measured on the binary of v1.1.37.02 on 2026-11-02:
                    // the value of the v1 is a number when it can be one --
                    // the "5" is 5, and the "0x10" is 16 -- and the whole
                    // number stays a whole number: the `-x` of the "5" is
                    // "-5". The value that is not a number makes the
                    // operation blank.
                    if self.dialect.a_value_that_is_not_a_number_is_blank() {
                        return Ok(match self.the_number_of_the_v1(operand, &value) {
                            Some(Value::Int(number)) => Value::Int(number.wrapping_neg()),
                            Some(Value::Float(number)) => Value::Float(-number),
                            _ => Value::Text(String::new()),
                        });
                    }
                    Ok(match value {
                        Value::Int(number) => Value::Int(number.wrapping_neg()),
                        other => Value::Float(-self.number(&other, *line)?),
                    })
                }
                // The unary plus turns a numeric string into a number.
                UnaryOp::Plus => {
                    let value = self.evaluate(operand)?;
                    // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                    // plus of the v1 changes nothing: the `+x` of the
                    // "0x10" is "0x10", and the `+x` of the "abc" is "abc".
                    if self.dialect.a_value_that_is_not_a_number_is_blank() {
                        return Ok(value);
                    }
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
                    Ok(Value::Int(!self.the_whole_number(
                        &value,
                        *line,
                        TypeName::Number,
                    )?))
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
                    Ok(Value::Reference(Reference {
                        name: name.clone(),
                        scope,
                        alias: false,
                    }))
                }
            },
            Node::Binary {
                op,
                left,
                right,
                line,
            } => {
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
                    //
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the `&&` and the `||` of the v1 answer
                    // one and zero, and not the operand: the `"a" && "b"`
                    // is 1.
                    BinaryOp::And => {
                        if !self.the_operand_is_true(left, &left_value) {
                            return Ok(self.the_answer_of_the_boolean(left, &left_value));
                        }
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-04: the operand that is not there poisons
                        // the operation that holds it: the `1 and` is the
                        // empty text, and not the zero that the `1 and ""`
                        // answers.
                        if matches!(right.as_ref(), Node::Unset { .. }) {
                            return Ok(Value::Text(String::new()));
                        }
                        let right_value = self.evaluate(right)?;
                        return Ok(self.the_answer_of_the_boolean(right, &right_value));
                    }
                    BinaryOp::Or => {
                        if self.the_operand_is_true(left, &left_value) {
                            return Ok(self.the_answer_of_the_boolean(left, &left_value));
                        }
                        if matches!(right.as_ref(), Node::Unset { .. }) {
                            return Ok(Value::Text(String::new()));
                        }
                        let right_value = self.evaluate(right)?;
                        return Ok(self.the_answer_of_the_boolean(right, &right_value));
                    }
                    _ => {}
                }
                let right_value = self.evaluate(right)?;
                // The arithmetic of the v1 is not the one of the v2: the text
                // that is written between quotes is a string, and the value
                // of a variable is a number when it can be one.
                let (left_value, right_value) = if self
                    .dialect
                    .a_value_that_is_not_a_number_is_blank()
                {
                    // The sum of a text that is written is a concatenation.
                    if *op == BinaryOp::Add && self.the_node_is_a_quoted_text(left) {
                        let text =
                            format!("{}{}", left_value.to_text(), self.the_text_of(&right_value));
                        return Ok(Value::Text(text));
                    }
                    // The comparison is a number when both sides are numbers,
                    // and a text otherwise.
                    if matches!(
                        op,
                        BinaryOp::Less
                            | BinaryOp::Greater
                            | BinaryOp::LessEqual
                            | BinaryOp::GreaterEqual
                    ) {
                        let answer = self.the_comparison_of_the_v1(
                            left,
                            right,
                            &left_value,
                            &right_value,
                            op,
                        );
                        return Ok(Value::Int(answer as i64));
                    }
                    if matches!(op, BinaryOp::Equal | BinaryOp::NotEqual) {
                        // The equal sign of the v1 is always without regard
                        // to the case, and the method of the locale is the
                        // one it uses when the sense of the case is On. The
                        // not equal sign follows the sense of the case.
                        let method = match op {
                            BinaryOp::Equal => self.case_sense.the_method_of_the_equal_sign(),
                            _ => self.case_sense.the_method(),
                        };
                        // Measured on the binary of v1.1.37.02 on 2026-11-02:
                        // the comparison is a number when both sides are
                        // numbers, and a text otherwise: the "0x10" is the
                        // number 16, and the "0x10" that is written between
                        // quotes is a text that is not a number.
                        let same = match (
                            self.the_number_of_the_v1(left, &left_value),
                            self.the_number_of_the_v1(right, &right_value),
                        ) {
                            (Some(one), Some(other)) => the_same_number(&one, &other),
                            _ => {
                                the_same_text(&left_value.to_text(), &right_value.to_text(), method)
                            }
                        };
                        let answer = match op {
                            BinaryOp::Equal => same,
                            _ => !same,
                        };
                        return Ok(Value::Int(answer as i64));
                    }
                    if matches!(
                        op,
                        BinaryOp::StrictEqual
                            | BinaryOp::StrictNotEqual
                            | BinaryOp::Concat
                            | BinaryOp::RegexMatch
                    ) {
                        // The equality compares the values as they are, and
                        // the concatenation wants the text of them.
                        (left_value, right_value)
                    } else {
                        // The operands of the arithmetic are numbers: the text
                        // that is written between quotes is not one, and the
                        // value of a variable is one when it can be.
                        let left = self.the_number_of_the_v1(left, &left_value);
                        let right = self.the_number_of_the_v1(right, &right_value);
                        match (left, right) {
                            (Some(left), Some(right)) => (left, right),
                            // A value that is not a number makes the operation
                            // blank, and it is not a failure.
                            _ => return Ok(Value::Text(String::new())),
                        }
                    }
                } else {
                    (left_value, right_value)
                };
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
                        // Measured on the binary of v1.1.37.02 on 2026-11-01:
                        // the sum of the v1 joins a text and a number: `"5" +
                        // 1` is `51`, and the sum of two numbers is a number.
                        //
                        // Measured on the binary of v1.1.37.02 on 2026-11-02:
                        // the whole number that does not fit in the machine
                        // wraps the way the machine does: the sum of the
                        // 9223372036854775807 and the one is
                        // -9223372036854775808, and the product of it and the
                        // two is -2.
                        if self.dialect.add_concatenates()
                            && (matches!(left_value, Value::Text(_))
                                || matches!(right_value, Value::Text(_)))
                        {
                            Value::Text(format!(
                                "{}{}",
                                left_value.to_text(),
                                right_value.to_text()
                            ))
                        } else {
                            let a = self.number(&left_value, line)?;
                            let b = self.number(&right_value, line)?;
                            match (&left_value, &right_value) {
                                (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_add(*b)),
                                _ => Value::Float(a + b),
                            }
                        }
                    }
                    BinaryOp::Subtract => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        match (&left_value, &right_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_sub(*b)),
                            _ => Value::Float(a - b),
                        }
                    }
                    BinaryOp::Multiply => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        match (&left_value, &right_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_mul(*b)),
                            _ => Value::Float(a * b),
                        }
                    }
                    BinaryOp::Divide => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        if b == 0.0 {
                            // Measured on the binary of v1.1.37.02 on 2026-11-01:
                            // the division by zero of the v1 is a blank result
                            // and not a failure.
                            if self.dialect.division_by_zero_is_blank() {
                                return Ok(Value::Text(String::new()));
                            }
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
                        Value::Int(a.wrapping_div(b))
                    }
                    BinaryOp::Power => {
                        let a = self.number(&left_value, line)?;
                        let b = self.number(&right_value, line)?;
                        // Two whole numbers with a whole exponent give a
                        // whole number; a negative exponent gives a float.
                        if let (Value::Int(base), Value::Int(exponent)) =
                            (&left_value, &right_value)
                        {
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
                                return Err(RuntimeFailure::Error(RuntimeError {
                                    message: reason,
                                    line,
                                }));
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
                                    message: expected_type(
                                        TypeName::Class,
                                        right_value.type_name(),
                                    ),
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
                            // A number and a text are not objects, and v2
                            // still knows their classes: `3 is Integer` is
                            // one and `3 is Object` is zero. The class of the
                            // value is the one of the kinds of v2, and the
                            // chain of it goes up through the `Number` and
                            // the `Primitive` to the `Any`.
                            _ => {
                                let name = left_value.type_name().name().to_lowercase();
                                class == "any"
                                    || class == name
                                    || self.class_chain(&name).contains(&class)
                            }
                        };
                        Value::Int(answer as i64)
                    }
                    BinaryOp::And | BinaryOp::Or | BinaryOp::Coalesce => {
                        unreachable!("handled above")
                    }
                })
            }
            Node::Call {
                name,
                arguments,
                line,
            } => self.call(name, arguments, *line),
            Node::New {
                path,
                arguments,
                line,
            } => self.new_instance(path, arguments, *line),
            Node::MethodCall {
                target,
                name,
                arguments,
                line,
            } => {
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
            Node::Index {
                target,
                index,
                line,
            } => {
                // `super["value"]`: the item of the base.
                if let Node::Super { .. } = &**target {
                    let key = self.evaluate(index)?;
                    return self.read_the_item_of_the_base(&key, *line);
                }
                let key = self.evaluate(index)?;
                // `x.y[z]`: the `__Get` of `x.y` is asked with the
                // parameters of the access, and the value it answers is
                // the one that is indexed.
                if let Node::Property {
                    target: object,
                    name,
                    line: property_line,
                } = &**target
                {
                    let object_value = self.evaluate(object)?;
                    let name = self.the_name_of_the_member(name, *property_line)?;
                    let (value, by_meta) = self.read_property_fully(
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
            Node::Ternary {
                condition,
                then,
                otherwise,
                ..
            } => {
                let value = self.evaluate(condition)?;
                if self.the_value_is_true(&value) {
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
                    None => {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-04: the v1 does not refuse the name that is
                        // not there: it creates the variable of that name and
                        // reads it, and the answer is the empty text.
                        if self
                            .dialect
                            .the_name_that_the_script_builds_and_that_is_not_there_is_answered()
                        {
                            return Ok(Value::Text(String::new()));
                        }
                        Err(RuntimeFailure::Detailed(
                            RuntimeError {
                                message: "Variable not found.".to_string(),
                                line: *line,
                            },
                            Detail {
                                what: String::new(),
                                extra: name,
                            },
                        ))
                    }
                }
            }
            Node::AssignDynamic {
                name,
                op,
                value,
                line,
            } => {
                let name = self.the_name_of_the_pieces(name, *line)?;
                let value = self.evaluate(value)?;
                self.write_a_dynamic_name(&name, *op, value, *line)?;
                Ok(Value::Text(String::new()))
            }
            Node::Expression { .. }
            | Node::LegacyExpression { .. }
            | Node::Block { .. }
            | Node::LegacyCall { .. }
            | Node::LegacyIf { .. }
            | Node::LegacyAssign { .. }
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
            | Node::HotkeyDefinition { .. }
            | Node::Return { .. }
            | Node::ExitApp { .. }
            | Node::FunctionDef { .. }
            | Node::Directive { .. }
            | Node::ClassDef { .. } => Err(RuntimeFailure::Error(RuntimeError {
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // write of the item of a value that is not an object is dropped by
        // the v1.
        let Value::Object(object) = target else {
            if self.dialect.the_item_of_an_object_is_its_property()
                && self
                    .dialect
                    .the_write_of_a_property_of_a_value_that_is_not_an_object_is_dropped()
            {
                return Ok(value);
            }
            return Err(self.no_item(target, line));
        };
        let class = object.borrow().class.clone();
        // The compound operators read what is there before they write.
        let value = match op {
            AssignOp::Set => value,
            other => {
                let current = self.read_item(target, key, line)?;
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 does not write the item when the value
                // that is there is not a number: the `a[5] += 2` of an array
                // of two leaves the place as it was.
                if self
                    .dialect
                    .the_compound_write_of_a_thing_that_is_not_a_number_is_dropped()
                    && !matches!(op, AssignOp::Set | AssignOp::Concat)
                    && !self.the_value_is_a_number_of_the_v1(&current)
                {
                    return Ok(current);
                }
                self.apply_assign(other, "the item", Some(current), value, line)?
            }
        };
        if let Some(result) =
            crate::classes::set_item(&class, key, target, value.clone(), self, line)
        {
            result?;
            return Ok(value);
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // item of an object of the v1 is the property of that name: the
        // `a["b"] := 5` writes the property "b".
        if self.dialect.the_item_of_an_object_is_its_property() {
            let name = key.to_text();
            let current = match op {
                AssignOp::Set => self.read_property(target, &name, line).ok(),
                _ => Some(self.read_property(target, &name, line)?),
            };
            let stored = self.apply_assign(op, &name, current, value, line)?;
            self.write_property(target, &name, stored.clone(), line)?;
            return Ok(stored);
        }
        Err(self.no_item(target, line))
    }

    /// The value of an item of an object: `obj[key]`.
    fn read_item(
        &mut self,
        target: &Value,
        key: &Value,
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let Value::Object(object) = target else {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 does not refuse the item of a value that is not an
            // object, and the answer is a blank text.
            if self.dialect.a_member_that_is_not_there_is_answered() {
                return Ok(Value::Text(String::new()));
            }
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
                let value =
                    self.call_the_body_of_an_accessor(target, &access, getter, line, &[], None)?;
                return self.read_item(&value, key, line);
            }
            // The parameters of the property are the ones of the index.
            let value = self.call_the_body_of_an_accessor(
                target,
                &access,
                getter,
                line,
                core::slice::from_ref(key),
                None,
            )?;
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // item of an object of the v1 is the property of that name: the
        // `a["b"]` reads the property "b", and the `a["B"]` reads it too.
        // The v2 asks the `__Item` of the object, and a plain object of it
        // has no such thing.
        if self.dialect.the_item_of_an_object_is_its_property() {
            let name = key.to_text();
            return self.read_property(target, &name, line);
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
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not refuse the item that an object does not have, and the answer is
        // a blank text.
        if self.dialect.a_member_that_is_not_there_is_answered() {
            return Ok(Value::Text(String::new()));
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
    ) -> Result<Option<Value>, RuntimeError> {
        let mut parts = reference.split('.');
        let Some(name) = parts.next() else {
            return Ok(None);
        };
        let found = self.lookup(name);
        let mut value = match found {
            Some(value) => value,
            // The name may be one of the built-in variables: the table of
            // them knows whether it is one, and the runtime may not be
            // able to answer it.
            None => match self.the_built_in_variable(name) {
                Some(Ok(value)) => value,
                Some(Err(error)) => {
                    return Err(RuntimeError {
                        message: error.message,
                        line,
                    });
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
        match self.the_built_in_variable(name) {
            Some(Ok(value)) => Ok(Some(value)),
            Some(Err(error)) => Err(RuntimeFailure::Error(RuntimeError {
                message: error.message,
                line,
            })),
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the write of a name that the script builds inside a
        // function, and it refuses it whatever the variable of that name is.
        if self.declarations.last().is_some()
            && self
                .dialect
                .the_dynamic_write_inside_a_function_is_refused()
        {
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: self
                        .dialect
                        .the_message_of_the_dynamic_write_inside_a_function()
                        .to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: name.to_string(),
                },
            ));
        }
        // A dynamic reference does not create a variable: the one that is
        // written has to be there.
        let exists = self.lookup_variable(name).is_some() || self.variables.contains_key(&key);
        if !exists {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 creates the variable of the name that the script builds
            // and writes it: the `a := 1` and the `%a% := 7` write the
            // variable "1".
            if self
                .dialect
                .the_name_that_the_script_builds_and_that_is_not_there_is_answered()
            {
                self.assign(name, Value::Text(String::new()));
            } else {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Variable not found.".to_string(),
                        line,
                    },
                    Detail {
                        what: String::new(),
                        extra: name.to_string(),
                    },
                ));
            }
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
                    message: "This dynamic assignment requires a \"global\" declaration."
                        .to_string(),
                    line,
                },
                Detail {
                    what: String::new(),
                    extra: name.to_string(),
                },
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
// library. A target with no `std` takes the same numbers from `math`, the
// same maths in this crate, so the language answers the same on both.

/// The square root of `value`.
#[cfg(feature = "std")]
pub(crate) fn sqrt(value: f64) -> f64 {
    value.sqrt()
}

/// The square root of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn sqrt(value: f64) -> f64 {
    crate::math::sqrt(value)
}

/// The largest whole number that is not greater than `value`.
#[cfg(feature = "std")]
pub(crate) fn floor(value: f64) -> f64 {
    value.floor()
}

/// The largest whole number that is not greater than `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn floor(value: f64) -> f64 {
    crate::math::floor(value)
}

/// The smallest whole number that is not less than `value`.
#[cfg(feature = "std")]
pub(crate) fn ceil(value: f64) -> f64 {
    value.ceil()
}

/// The smallest whole number that is not less than `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn ceil(value: f64) -> f64 {
    crate::math::ceil(value)
}

/// `value` rounded to the nearest whole number, halves away from zero.
#[cfg(feature = "std")]
pub(crate) fn round(value: f64) -> f64 {
    value.round()
}

/// `value` rounded to the nearest whole number, halves away from zero.
#[cfg(not(feature = "std"))]
pub(crate) fn round(value: f64) -> f64 {
    crate::math::round(value)
}

/// The power of two numbers.
///
/// The same shape as the square root: `std` has it, and `math` has it
/// for a target that has no `std`.
#[cfg(feature = "std")]
pub(crate) fn powf(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// The power of two numbers.
#[cfg(not(feature = "std"))]
pub(crate) fn powf(base: f64, exponent: f64) -> f64 {
    crate::math::pow(base, exponent)
}

/// The arc cosine of `value`.
#[cfg(feature = "std")]
pub(crate) fn acos(value: f64) -> f64 {
    value.acos()
}

/// The arc cosine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn acos(value: f64) -> f64 {
    crate::math::acos(value)
}

/// The arc sine of `value`.
#[cfg(feature = "std")]
pub(crate) fn asin(value: f64) -> f64 {
    value.asin()
}

/// The arc sine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn asin(value: f64) -> f64 {
    crate::math::asin(value)
}

/// The arc tangent of `value`.
#[cfg(feature = "std")]
pub(crate) fn atan(value: f64) -> f64 {
    value.atan()
}

/// The arc tangent of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn atan(value: f64) -> f64 {
    crate::math::atan(value)
}

/// The cosine of `value`.
#[cfg(feature = "std")]
pub(crate) fn cos(value: f64) -> f64 {
    value.cos()
}

/// The cosine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn cos(value: f64) -> f64 {
    crate::math::cos(value)
}

/// The sine of `value`.
#[cfg(feature = "std")]
pub(crate) fn sin(value: f64) -> f64 {
    value.sin()
}

/// The sine of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn sin(value: f64) -> f64 {
    crate::math::sin(value)
}

/// The tangent of `value`.
#[cfg(feature = "std")]
pub(crate) fn tan(value: f64) -> f64 {
    value.tan()
}

/// The tangent of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn tan(value: f64) -> f64 {
    crate::math::tan(value)
}

/// The exponential of `value`.
#[cfg(feature = "std")]
pub(crate) fn exp(value: f64) -> f64 {
    value.exp()
}

/// The exponential of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn exp(value: f64) -> f64 {
    crate::math::exp(value)
}

/// The natural logarithm of `value`.
#[cfg(feature = "std")]
pub(crate) fn ln(value: f64) -> f64 {
    value.ln()
}

/// The natural logarithm of `value`.
#[cfg(not(feature = "std"))]
pub(crate) fn ln(value: f64) -> f64 {
    crate::math::ln(value)
}

/// The logarithm of `value` in base ten.
#[cfg(feature = "std")]
pub(crate) fn log10(value: f64) -> f64 {
    value.log10()
}

/// The logarithm of `value` in base ten.
#[cfg(not(feature = "std"))]
pub(crate) fn log10(value: f64) -> f64 {
    crate::math::log10(value)
}

/// The fractional part of `value`: it keeps the sign of the value.
#[cfg(feature = "std")]
pub(crate) fn fract(value: f64) -> f64 {
    value.fract()
}

/// The fractional part of `value`: it keeps the sign of the value.
#[cfg(not(feature = "std"))]
pub(crate) fn fract(value: f64) -> f64 {
    value - crate::math::trunc(value)
}

/// The whole part of `value`: the number without its fraction.
#[cfg(feature = "std")]
fn trunc(value: f64) -> f64 {
    value.trunc()
}

/// The whole part of `value`: the number without its fraction.
#[cfg(not(feature = "std"))]
fn trunc(value: f64) -> f64 {
    crate::math::trunc(value)
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
    if exponent < 0 { 1.0 / result } else { result }
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
    put(
        &mut properties,
        "maxparams",
        Value::Int(function.parameters.len() as i64),
    );
    put(&mut properties, "isbuiltin", Value::Int(0));
    put(&mut properties, "isvariadic", Value::Int(0));
    Value::Object(Rc::new(RefCell::new(Object {
        class: "func".to_string(),
        properties,
    })))
}

/// The value of a closure: an object of the class `Func`.
///
/// The name of a closure of v2 is empty, and the name under which the
/// runtime keeps it is in the object and not in the name.
fn closure_value(name: &str, function: &Function) -> Value {
    let value = func_value("", function);
    if let Value::Object(object) = &value {
        object
            .borrow_mut()
            .put(CLOSURE, Value::Text(name.to_string()));
    }
    value
}

/// The value of a built-in: an object of the class `Func`.
///
/// A built-in is a function of the runtime, and it is a value like any
/// other: it can be held in a variable and called through it.
///
/// The most parameters that it takes is the one of the dialect: the v1
/// counts ten thousand for a built-in that takes a variable number of
/// arguments.
fn builtin_value(name: &str, min: u8, max: u32) -> Value {
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, "name", Value::Text(name.to_string()));
    put(&mut properties, "minparams", Value::Int(min as i64));
    put(&mut properties, "maxparams", Value::Int(max as i64));
    put(&mut properties, "isbuiltin", Value::Int(1));
    put(&mut properties, "isvariadic", Value::Int(0));
    Value::Object(Rc::new(RefCell::new(Object {
        class: "func".to_string(),
        properties,
    })))
}

/// The value of a class: an object of the class itself.
fn class_value(name: &str, class: &Class) -> Value {
    let mut properties: BTreeMap<String, Property> = BTreeMap::new();
    put(&mut properties, CLASS_NAME, Value::Text(class.name.clone()));
    put(&mut properties, "prototype", prototype_value(&class.name));
    Value::Object(Rc::new(RefCell::new(Object {
        class: name.to_lowercase(),
        properties,
    })))
}

/// The value of the parameter that a failure names.
///
/// The text of a parameter of a command of the v1.
///
/// The parameter that wants the name of a variable is a text, and the
/// parameter that is an expression has no name: the shape of the command
/// says which one it is.
fn the_text_of_the_parameter(parameter: &crate::ast::LegacyParameter) -> String {
    match parameter {
        crate::ast::LegacyParameter::Text(text) => text.trim().to_string(),
        crate::ast::LegacyParameter::Expression(_) => String::new(),
    }
}

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
        // Measured on the binary of v2.0.19 on 2026-11-01: a text that is
        // empty is named "empty string" in the `Extra` of a failure, the way
        // the message of the kind of a value names it.
        Value::Text(text) if text.is_empty() => "empty string".to_string(),
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

/// The format of the whole numbers, the way the dialect formats them.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the format is "D"
/// when the script begins, and the `SetFormat` turns it into "H" or "h".
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IntegerFormat {
    /// The base ten: 255.
    Decimal,
    /// The base sixteen with the prefix: 0xFF.
    Hexadecimal,
    /// The base sixteen with the prefix and the lower case digits: 0xff.
    HexadecimalLower,
}

impl IntegerFormat {
    /// The name of the format, the way the `A_FormatInteger` says it.
    pub fn name(&self) -> &'static str {
        match self {
            IntegerFormat::Decimal => "D",
            IntegerFormat::Hexadecimal => "H",
            IntegerFormat::HexadecimalLower => "h",
        }
    }

    /// The text of a whole number in this format.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the sign is in
    /// front of the prefix: the -255 is "-0xFF".
    pub fn the_text_of(&self, number: i64) -> String {
        match self {
            IntegerFormat::Decimal => number.to_string(),
            IntegerFormat::Hexadecimal => the_hexadecimal_text_of(number, true),
            IntegerFormat::HexadecimalLower => the_hexadecimal_text_of(number, false),
        }
    }
}

/// The text of a whole number in the base of sixteen.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the sign is in
/// front of the prefix, and not in front of the digits: the -255 is
/// "-0xFF".
fn the_hexadecimal_text_of(number: i64, upper: bool) -> String {
    let sign = if number < 0 { "-" } else { "" };
    let digits = number.unsigned_abs();
    if upper {
        format!("{sign}0x{digits:X}")
    } else {
        format!("{sign}0x{digits:x}")
    }
}

/// The format of the floats, the way the dialect formats them.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the format is "0.6"
/// when the script begins, and it is the width of the text, the number of
/// the decimals and the scientific notation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FloatFormat {
    /// The width of the text: the number is padded to it.
    pub width: u32,
    /// The text is padded with zeroes and not with spaces.
    pub zeroes: bool,
    /// The text is justified to the left and not to the right.
    pub left: bool,
    /// The number of the decimals of the text.
    pub decimals: u32,
    /// The text is in the scientific notation.
    pub scientific: bool,
    /// The exponent is in upper case.
    pub upper: bool,
}

impl FloatFormat {
    /// The format of the v1 when the script begins: "0.6".
    pub const DEFAULT: FloatFormat = FloatFormat {
        width: 0,
        zeroes: false,
        left: false,
        decimals: 6,
        scientific: false,
        upper: false,
    };

    /// The name of the format, the way the `A_FormatFloat` says it.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the name is the
    /// format that was written, with the width and the decimals.
    pub fn name(&self) -> String {
        let mut name = String::new();
        if self.left {
            name.push('-');
        }
        if self.zeroes {
            name.push('0');
        }
        name.push_str(&self.width.to_string());
        name.push('.');
        name.push_str(&self.decimals.to_string());
        if self.scientific {
            name.push(if self.upper { 'E' } else { 'e' });
        }
        name
    }

    /// The text of a float in this format.
    ///
    /// The number is rounded to the decimals of the format, and the text is
    /// padded to the width of it.
    pub fn the_text_of(&self, number: f64) -> String {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the infinity
        // of the v1 is the text of the C library: "1.#INF00" and "-1.#INF00".
        if number.is_infinite() {
            return if number < 0.0 {
                "-1.#INF00".to_string()
            } else {
                "1.#INF00".to_string()
            };
        }
        let decimals = self.decimals as usize;
        let text = if self.scientific {
            let formatted = format!("{:.*e}", decimals, number);
            // The exponent of the v1 has a sign and three digits: `1.0e+003`.
            let (mantissa, exponent) = match formatted.split_once('e') {
                Some((mantissa, exponent)) => (mantissa.to_string(), exponent.to_string()),
                None => (formatted.clone(), String::new()),
            };
            let exponent = match exponent.strip_prefix('-') {
                Some(rest) => format!("-{}", the_digits_of_the_exponent(rest)),
                None => format!("+{}", the_digits_of_the_exponent(&exponent)),
            };
            format!("{mantissa}e{exponent}")
        } else {
            the_text_of_the_number(number, decimals)
        };
        let width = self.width as usize;
        if text.chars().count() >= width {
            return text;
        }
        let padding = width - text.chars().count();
        if self.zeroes {
            // The zeroes go before the number, and the sign keeps its place.
            let (sign, rest) = match text.strip_prefix('-') {
                Some(rest) => ("-", rest),
                None => ("", text.as_str()),
            };
            let mut padded = String::from(sign);
            padded.push_str(&"0".repeat(padding));
            padded.push_str(rest);
            padded
        } else if self.left {
            let mut padded = text;
            padded.push_str(&" ".repeat(padding));
            padded
        } else {
            let mut padded = " ".repeat(padding);
            padded.push_str(&text);
            padded
        }
    }
}

/// The sense of the case, the way the dialect compares the letters.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the sense is "Off"
/// when the script begins, and the `StringCaseSense` turns it into "On" or
/// into "Locale".
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CaseSense {
    /// The letters A-Z are the same as their lower case, and the others are
    /// not.
    Off,
    /// The letters are taken as they are.
    On,
    /// The letters follow the rules of the locale of the machine.
    Locale,
}

impl CaseSense {
    /// The name of the sense, the way the `A_StringCaseSense` says it.
    pub fn name(&self) -> &'static str {
        match self {
            CaseSense::Off => "Off",
            CaseSense::On => "On",
            CaseSense::Locale => "Locale",
        }
    }

    /// The method that compares two texts.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the sense that is
    /// On takes the letters as they are, and the sense that is Off treats the
    /// letters A-Z as their lower case and the others as they are.
    pub fn the_method(&self) -> CaseMethod {
        match self {
            CaseSense::Off => CaseMethod::Ascii,
            CaseSense::On => CaseMethod::Exact,
            CaseSense::Locale => CaseMethod::Locale,
        }
    }

    /// The method that the equal sign of the v1 uses.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the equal sign is
    /// always without regard to the case, and the method of the locale is the
    /// one it uses when the sense is On: `"ä" = "Ä"` is true when the sense
    /// is On, and it is false when the sense is Off.
    pub fn the_method_of_the_equal_sign(&self) -> CaseMethod {
        match self {
            CaseSense::Off => CaseMethod::Ascii,
            CaseSense::On | CaseSense::Locale => CaseMethod::Locale,
        }
    }
}

/// The method that compares the letters of two texts.
///
/// The method of the locale is the one of the machine, and the tree does
/// not ask the machine for it: it folds the letters of the Unicode, which
/// is the method of the locale of every machine that is written in the
/// Latin alphabet.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CaseMethod {
    /// The letters A-Z are the same as their lower case, and the others are
    /// not.
    Ascii,
    /// The letters are taken as they are.
    Exact,
    /// The letters follow the rules of the locale of the machine.
    Locale,
}

/// Whether two texts are the same, with the method of the case.
pub(crate) fn the_same_text(left: &str, right: &str, method: CaseMethod) -> bool {
    match method {
        CaseMethod::Ascii => left.eq_ignore_ascii_case(right),
        CaseMethod::Exact => left == right,
        CaseMethod::Locale => left.to_lowercase() == right.to_lowercase(),
    }
}

/// Whether two numbers are the same number.
///
/// The whole numbers are compared as whole numbers: the machine cannot
/// hold every one of them exactly as a float, and the numbers that it
/// rounds are not the same number.
pub(crate) fn the_same_number(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(one), Value::Int(other)) => one == other,
        _ => left.to_number() == right.to_number(),
    }
}

/// Whether the text of the needle is in the text of the haystack, with the
/// method of the case.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IfInString` of
/// the v1 finds the text of the needle in the variable, and the sense of the
/// case says whether the letters of them are the same.
pub(crate) fn the_text_contains(haystack: &str, needle: &str, method: CaseMethod) -> bool {
    match method {
        CaseMethod::Ascii => haystack
            .to_ascii_lowercase()
            .contains(&needle.to_ascii_lowercase()),
        CaseMethod::Exact => haystack.contains(needle),
        CaseMethod::Locale => haystack.to_lowercase().contains(&needle.to_lowercase()),
    }
}

/// How two texts stand to each other, with the method of the case.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the texts of the v1
/// are compared letter by letter, and the method of the case says whether
/// the letters of them are the same: `"a" < "B"` is true when the sense is
/// Off, and it is false when the sense is On.
pub(crate) fn the_order_of_the_texts(
    left: &str,
    right: &str,
    method: CaseMethod,
) -> core::cmp::Ordering {
    match method {
        CaseMethod::Ascii => {
            let left = left.to_ascii_lowercase();
            let right = right.to_ascii_lowercase();
            left.cmp(&right)
        }
        CaseMethod::Exact => left.cmp(right),
        CaseMethod::Locale => left.to_lowercase().cmp(&right.to_lowercase()),
    }
}

/// The digits of the exponent of the scientific notation: three at least.
fn the_digits_of_the_exponent(digits: &str) -> String {
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() {
        "000".to_string()
    } else if digits.len() < 3 {
        format!("0{digits:0>2}")
    } else {
        digits.to_string()
    }
}

/// The text of a number, with the decimals of the format.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// writes the number with seventeen digits, and it rounds that number to
/// the decimals of the format -- a half goes away from the zero. The text of
/// the C library writes the digits that the double holds, and the two tell
/// each other apart in the numbers that are big: `Exp(65)` is
/// "16948892444103338000000000000.000000" and not
/// "16948892444103337803358666752.000000".
fn the_text_of_the_number(number: f64, decimals: usize) -> String {
    let negative = number < 0.0;
    // The seventeen digits of the double, and the place of the point.
    let exact = format!("{:.16e}", number.abs());
    let (mantissa, exponent) = match exact.split_once('e') {
        Some((mantissa, exponent)) => (mantissa, exponent),
        None => (exact.as_str(), "0"),
    };
    let digits: String = mantissa
        .chars()
        .filter(|character| character.is_ascii_digit())
        .collect();
    let exponent: i32 = exponent.parse().unwrap_or(0);
    // The number is the digits with the point after `exponent + 1` of them.
    let point = exponent + 1;
    let mut whole = String::new();
    let mut fraction = String::new();
    if point <= 0 {
        whole.push('0');
        for _ in 0..-point {
            fraction.push('0');
        }
        fraction.push_str(&digits);
    } else {
        let point = point as usize;
        if point >= digits.len() {
            whole.push_str(&digits);
            for _ in 0..point - digits.len() {
                whole.push('0');
            }
        } else {
            whole.push_str(&digits[..point]);
            fraction.push_str(&digits[point..]);
        }
    }
    // The rounding: a half goes away from the zero.
    if fraction.len() > decimals {
        if fraction.as_bytes()[decimals] >= b'5' {
            let mut index = decimals;
            let mut carries = true;
            while carries && index > 0 {
                index -= 1;
                let digit = fraction.as_bytes()[index] - b'0';
                if digit == 9 {
                    fraction.replace_range(index..index + 1, "0");
                } else {
                    fraction.replace_range(index..index + 1, &format!("{}", digit + 1));
                    carries = false;
                }
            }
            if carries {
                // The rounding passed the point, and the whole number grows.
                let mut index = whole.len();
                let mut carries = true;
                while carries && index > 0 {
                    index -= 1;
                    let digit = whole.as_bytes()[index] - b'0';
                    if digit == 9 {
                        whole.replace_range(index..index + 1, "0");
                    } else {
                        whole.replace_range(index..index + 1, &format!("{}", digit + 1));
                        carries = false;
                    }
                }
                if carries {
                    whole.insert(0, '1');
                }
            }
        }
        fraction.truncate(decimals);
    } else {
        for _ in fraction.len()..decimals {
            fraction.push('0');
        }
    }
    let mut text = String::new();
    if negative {
        text.push('-');
    }
    text.push_str(&whole);
    if decimals > 0 {
        text.push('.');
        text.push_str(&fraction);
    }
    text
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
        _ => RuntimeError {
            message: "the runtime cannot answer that.".into(),
            line,
        },
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
    values_equal_with(
        left,
        right,
        if case_sensitive {
            CaseMethod::Exact
        } else {
            CaseMethod::Ascii
        },
    )
}

/// Whether two values are equal, with the method of the case.
///
/// The numbers are the same number, and the texts are the same text with
/// the method that the caller asks for.
fn values_equal_with(left: &Value, right: &Value, method: CaseMethod) -> bool {
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
    the_same_text(&left.to_text(), &right.to_text(), method)
}

/// The message of a failure, the way the report carries it.
pub(crate) fn the_message_of(failure: &RuntimeFailure) -> String {
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
    properties
        .get(name)
        .map(|property| property.value.to_number().unwrap_or(0.0) as i64)
        .unwrap_or(0)
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
            // The body of a hotkey of the v1 is of the scope that carries
            // it: the hotkey is not a block, and a jump may leave it.
            Node::HotkeyDefinition { body, .. } => collect_the_labels(body, labels),
            Node::If {
                branches,
                otherwise,
                ..
            } => {
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
            Node::Block { body, .. }
            | Node::While { body, .. }
            | Node::Loop { body, .. }
            | Node::For { body, .. } => {
                collect_the_labels(body, labels);
            }
            Node::LegacyIf {
                body, otherwise, ..
            } => {
                collect_the_labels(body, labels);
                collect_the_labels(otherwise, labels);
            }
            Node::Try {
                body,
                catches,
                otherwise,
                finally,
                ..
            } => {
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

/// The built-in of this name, in the dialect that runs.
///
/// The name is matched the way AutoHotkey matches it: without regard
/// to the case.
pub fn the_builtin_in(
    dialect: &'static dyn crate::dialect::Dialect,
    name: &str,
) -> Option<&'static crate::builtins::Builtin> {
    let wanted = name.to_lowercase();
    dialect
        .builtins()
        .iter()
        .find(|builtin| builtin.name.to_lowercase() == wanted)
}
/// The name a statement is reported under.
pub fn op_name(statement: &Node) -> &str {
    op_name_in(statement, crate::dialect::v2())
}

/// The name a statement is reported under, in the dialect of the run.
pub fn op_name_in<'a>(
    statement: &'a Node,
    dialect: &'static dyn crate::dialect::Dialect,
) -> &'a str {
    match statement {
        Node::Assign { .. } => "Assign",
        Node::AssignIndex { .. } => "AssignIndex",
        // A call to a built-in is reported under the name of the built-in,
        // the way AutoHotkey reports a command.
        Node::Call { name, .. } => match the_builtin_in(dialect, name) {
            Some(builtin) => builtin.name,
            None => "Call",
        },
        // The legacy command of the v1 is reported under its own name.
        Node::LegacyCall { name, .. } => name,
        // The `new` of the v1 is reported under its own name.
        Node::New { .. } => "New",
        // The command of the v1 that has a body is reported under its own
        // name as well.
        Node::LegacyIf { name, .. } => name,
        Node::Expression { value, .. } => match the_call_of(value) {
            // The statement reads the member of the answer of a call, and
            // it is still the call that the report names.
            Some(name) => match the_builtin_in(dialect, name) {
                Some(builtin) => builtin.name,
                None => "Call",
            },
            None => "Expression",
        },
        Node::Block { .. } => "Block",
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
        Node::HotkeyDefinition { .. } => "HotkeyDefinition",
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
        Node::Unset { .. } => "Unset",
        Node::Ternary { .. } => "Ternary",
        Node::DynamicName { .. } => "DynamicName",
        Node::AssignDynamic { .. } => "AssignDynamic",
        Node::Index { .. } => "Index",
        Node::Group { .. } => "Group",
        Node::Empty { .. } => "Empty",
        Node::Legacy { .. } => "Legacy",
        Node::LegacyExpression { .. } => "LegacyExpression",
        Node::LegacyControl { .. } => "LegacyControl",
        Node::LegacyAssign { .. } => "LegacyAssign",
        Node::LegacyComparison { .. } => "LegacyComparison",
        Node::LegacyMatch { .. } => "LegacyMatch",
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
