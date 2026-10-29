//! The class types of AutoHotkey v2.
//!
//! `Map`, `Array`, `Buffer`, `File`, `Func`, `Gui`, `Menu` and `Hotkey`
//! are the classes the runtime carries. Each of them lives in its own
//! module, and this one is the registry that knows which is which.
//!
//! A class type of the runtime is not a class of the script: the script
//! cannot extend it, and its methods are the ones the runtime answers
//! for. The state of the object lives in the property table of the
//! object, and only the class module touches it.
//!
//! The ones that are shaped for Windows -- `Gui` and its parts, and
//! `Hotkey` -- are carried and answer with the reason they cannot work
//! here. The others work anywhere.
//!
//! The eleven tables of the binary are Array, Buffer, File, Func, Gui,
//! Gui.Control, Gui.ListView, Gui.Menu, Gui.StatusBar, Hotkey and Map.
//! The methods of the menu are in the table of `Gui.Menu`, and the
//! script calls the class `Menu`, so the registry answers for both.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use alloc::rc::Rc;
use alloc::string::ToString;

use crate::runtime::{Object, Runtime, RuntimeFailure, Value};

pub(crate) mod array;
pub(crate) mod buffer;
pub(crate) mod error;
pub(crate) mod file;
pub(crate) mod func;
pub(crate) mod gui;
pub(crate) mod hotkey;
pub(crate) mod map;

/// The class types the runtime carries.
///
/// `Object` is the root of every class: v2 has it, and every object of
/// the tree is one.
pub(crate) const TYPES: [&str; 10] = [
    "array", "buffer", "error", "file", "func", "gui", "hotkey", "map", "menu", "object",
];

/// Whether the name is a class type of the runtime.
pub(crate) fn is_type(name: &str) -> bool {
    let name = name.to_lowercase();
    TYPES.contains(&name.as_str()) || error::is_a_class(&name)
}

/// The class a class type of the runtime derives from.
///
/// The errors of v2 are a tree of their own: a `PropertyError` is a
/// `MemberError` and an `UnsetError` as well, and every one of them is
/// an `Error`. The other classes of the runtime are the root of it.
pub(crate) fn the_base_of(class: &str) -> Option<&'static str> {
    error::the_base_of(class)
}

/// The value of a class type: the class itself.
///
/// The class carries its prototype, the way a class of the script does:
/// the base of an object of this type is the prototype of it.
pub(crate) fn class_value(name: &str) -> Value {
    let mut table = BTreeMap::new();
    // The name of the class is the one v2 spells: the name of the
    // runtime carries it in lower case.
    table.insert("name".to_string(), Value::Text(the_spelling_of(name)));
    table.insert("prototype".to_string(), crate::runtime::prototype_value(name));
    Value::Object(Rc::new(RefCell::new(Object {
        class: name.to_lowercase(),
        properties: table,
    })))
}

/// The object of a class type, with nothing in it yet.
pub(crate) fn object(class: &str) -> Value {
    Value::Object(Rc::new(RefCell::new(Object {
        class: class.to_lowercase(),
        properties: BTreeMap::new(),
    })))
}

/// The object of a class type, with a property in it.
pub(crate) fn object_with(class: &str, properties: Vec<(&str, Value)>) -> Value {
    let mut table = BTreeMap::new();
    for (name, value) in properties {
        table.insert(name.to_lowercase(), value);
    }
    Value::Object(Rc::new(RefCell::new(Object {
        class: class.to_lowercase(),
        properties: table,
    })))
}

/// The name of a class of the runtime, the way v2 spells it.
///
/// The classes of the errors have a spelling of their own: `TypeError`,
/// and not `Typeerror`.
pub(crate) fn the_spelling_of(name: &str) -> String {
    if let Some(spelled) = error::the_spelling_of(name) {
        return spelled;
    }
    let mut spelled = name.to_string();
    if let Some(first) = spelled.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    spelled
}

/// The object a class type makes.
///
/// `None` means the name is not a class type of the runtime.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The errors of v2 are a tree of classes, and every one of them is
    // made the way the root of it is.
    if error::is_a_class(class) {
        return Some(error::construct(class, arguments, rt, line));
    }
    match class.to_lowercase().as_str() {
        "array" => Some(array::construct(arguments, rt, line)),
        "buffer" => Some(buffer::construct(arguments, rt, line)),
        "file" => Some(file::construct(arguments, rt, line)),
        "func" => Some(func::construct(arguments, rt, line)),
        "hotkey" => Some(hotkey::construct(arguments, rt, line)),
        "map" => Some(map::construct(arguments, rt, line)),
        "gui" | "menu" | "gui.control" | "gui.listview" | "gui.menu" | "gui.statusbar" => {
            Some(gui::construct(class, arguments, rt, line))
        }
        _ => None,
    }
}

/// A method of a class type.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    class: &str,
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // `__Item` is the method behind the index syntax, and a script may
    // call it by its name. The classes that have one answer it here.
    if method.eq_ignore_ascii_case("__item") {
        let key = arguments.first().cloned().unwrap_or(Value::Text(String::new()));
        if let Some(result) = item(class, &key, this, rt, line) {
            return Some(result);
        }
    }
    let answer = call_once(class, method, this, arguments, rt, line);
    // The name of a method is spelled the way the script spells it, and
    // the module matches the names of the table of the binary. The
    // second try is the name in lower case.
    match answer {
        Some(result) => Some(result),
        None => {
            let lower = method.to_lowercase();
            if lower == method {
                return None;
            }
            call_once(class, &lower, this, arguments, rt, line)
        }
    }
}

/// A method of a class type, with the name as it is.
fn call_once(
    class: &str,
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "array" => array::call(method, this, arguments, rt, line),
        "buffer" => buffer::call(method, this, arguments, rt, line),
        "error" => error::call(method, this, arguments, rt, line),
        "file" => file::call(method, this, arguments, rt, line),
        "func" => func::call(method, this, arguments, rt, line),
        "hotkey" => hotkey::call(method, this, arguments, rt, line),
        "map" => map::call(method, this, arguments, rt, line),
        "gui" | "menu" | "gui.control" | "gui.listview" | "gui.menu" | "gui.statusbar" => {
            gui::call(class, method, this, arguments, rt, line)
        }
        _ => None,
    }
}

/// A property of a class type.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    class: &str,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "array" => array::property(name, this, rt, line),
        "buffer" => buffer::property(name, this, rt, line),
        "error" => error::property(name, this, rt, line),
        "file" => file::property(name, this, rt, line),
        "func" => func::property(name, this, rt, line),
        "hotkey" => hotkey::property(name, this, rt, line),
        "map" => map::property(name, this, rt, line),
        "gui" | "menu" | "gui.control" | "gui.listview" | "gui.menu" | "gui.statusbar" => {
            gui::property(class, name, this, rt, line)
        }
        _ => None,
    }
}

/// A property of a class type that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    class: &str,
    name: &str,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "array" => array::set_property(name, this, value, rt, line),
        "buffer" => buffer::set_property(name, this, value, rt, line),
        "error" => error::set_property(name, this, value, rt, line),
        "file" => file::set_property(name, this, value, rt, line),
        "func" => func::set_property(name, this, value, rt, line),
        "hotkey" => hotkey::set_property(name, this, value, rt, line),
        "map" => map::set_property(name, this, value, rt, line),
        "gui" | "menu" | "gui.control" | "gui.listview" | "gui.menu" | "gui.statusbar" => {
            gui::set_property(class, name, this, value, rt, line)
        }
        _ => None,
    }
}

/// The item of an object: the `__Item` of the class.
///
/// `None` means the class has no `__Item` of its own.
pub(crate) fn item(
    class: &str,
    key: &Value,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match class.to_lowercase().as_str() {
        "array" => array::item(key, this, rt, line),
        "file" => file::item(key, this, rt, line),
        "map" => map::item(key, this, rt, line),
        _ => None,
    }
}

/// Write an item of an object of a class type.
///
/// `None` means the class has no way to write an item of its own.
pub(crate) fn set_item(
    class: &str,
    key: &Value,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    match class {
        "array" => array::set_item(key, this, value, rt, line),
        "map" => map::set_item(key, this, value, rt, line),
        _ => None,
    }
}

/// The names of v2 that a class type answers.
///
/// A name is a method of the class or a property of it, and the two are
/// told apart by the module that owns the class. The names are in lower
/// case, the way the script spells them.
pub fn names(class: &str) -> Vec<&'static str> {
    if error::is_a_class(class) {
        return error::names();
    }
    match class.to_lowercase().as_str() {
        "array" => array::names(),
        "buffer" => buffer::names(),
        "error" => error::names(),
        "file" => file::names(),
        "func" => func::names(),
        "hotkey" => hotkey::names(),
        "map" => map::names(),
        "gui" => gui::names("gui"),
        "menu" | "gui.menu" => gui::names("gui.menu"),
        "gui.control" => gui::names("gui.control"),
        "gui.listview" => gui::names("gui.listview"),
        "gui.statusbar" => gui::names("gui.statusbar"),
        _ => Vec::new(),
    }
}

/// Whether a class type answers the name.
///
/// The case of the name does not matter: the script spells a name the
/// way it likes.
pub fn carries(class: &str, name: &str) -> bool {
    let name = name.to_lowercase();
    names(class).iter().any(|carried| *carried == name)
}
