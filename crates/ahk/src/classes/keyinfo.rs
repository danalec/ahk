//! The object that `GetKeyInfo` answers, of the KeySharp dialect.
//!
//! The object carries the virtual key, the scan code, the name, the
//! modifiers and the prefix of the key. The prefix is the modifier
//! symbols the name carried, in the order of KeySharp.

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeFailure, Value};

/// The key the virtual key is under.
const THE_VIRTUAL_KEY: &str = "\u{1}keyinfo vk";
/// The key the scan code is under.
const THE_SCAN_CODE: &str = "\u{1}keyinfo sc";
/// The key the name is under.
const THE_NAME: &str = "\u{1}keyinfo name";
/// The key the modifiers are under.
const THE_MODIFIERS: &str = "\u{1}keyinfo modifiers";
/// The key the prefix is under.
const THE_PREFIX: &str = "\u{1}keyinfo prefix";

/// The names of the members of the object.
pub(crate) fn names() -> Vec<&'static str> {
    vec!["vk", "sc", "name", "modifiers", "prefix"]
}

/// The object of a key: what the tree knows of it.
pub(crate) fn the_key_info(
    vk: u32,
    sc: u32,
    name: &str,
    modifiers: u32,
    prefix: &str,
) -> Value {
    let Value::Object(object) = crate::classes::object("keyinfo") else {
        return Value::Text(String::new());
    };
    object
        .borrow_mut()
        .put(THE_VIRTUAL_KEY, Value::Int(vk as i64));
    object
        .borrow_mut()
        .put(THE_SCAN_CODE, Value::Int(sc as i64));
    object
        .borrow_mut()
        .put(THE_NAME, Value::Text(name.to_string()));
    object
        .borrow_mut()
        .put(THE_MODIFIERS, Value::Int(modifiers as i64));
    object
        .borrow_mut()
        .put(THE_PREFIX, Value::Text(prefix.to_string()));
    Value::Object(object)
}

/// A member of the object, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The chain of the extension classes asks every module, and a
    // module that answers the members of another class takes them
    // with the unset value: the guard keeps the class its own.
    if !_class.eq_ignore_ascii_case("keyinfo") {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    let answer = match name.to_lowercase().as_str() {
        "vk" => object.borrow().value_of(THE_VIRTUAL_KEY).unwrap_or(Value::Unset),
        "sc" => object.borrow().value_of(THE_SCAN_CODE).unwrap_or(Value::Unset),
        "name" => object.borrow().value_of(THE_NAME).unwrap_or(Value::Unset),
        "modifiers" => object.borrow().value_of(THE_MODIFIERS).unwrap_or(Value::Unset),
        "prefix" => object.borrow().value_of(THE_PREFIX).unwrap_or(Value::Unset),
        _ => return None,
    };
    Some(Ok(answer))
}
