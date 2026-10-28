//! The `Hotkey` class.
//!
//! A hotkey is a key the script watches for. The box has a keyboard of
//! its own but nothing that watches it, so the object is made -- a script
//! may hold it -- and every name of it answers with the reason it cannot
//! work here.

use alloc::format;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The names of the `Hotkey` class, with the thing each one does.
///
/// The names are the ones the binary gives the class, read out of the
/// surface table of v2.0.19.
const HOTKEY: &[(&str, &str)] = &[
    ("__new", "make the hotkey"),
    ("keyopt", "change the options of the hotkey"),
    ("start", "start watching for the key"),
    ("stop", "stop watching for the key"),
    ("wait", "wait for the key"),
    ("backspaceisundo", "read or set whether the backspace key undoes"),
    ("casesensitive", "read or set whether the key is matched by its case"),
    ("findanywhere", "read or set whether the key is found anywhere"),
    ("notifynontext", "read or set whether a key that is not text is announced"),
    ("visiblenontext", "read or set whether a key that is not text is shown"),
    ("visibletext", "read or set whether a text key is shown"),
    ("endkey", "read which key ended the hotkey"),
    ("endmods", "read which modifiers ended the hotkey"),
    ("endreason", "read why the hotkey ended"),
    ("inprogress", "read whether the hotkey is running"),
    ("input", "read the text the hotkey was given"),
    ("match", "read the text the hotkey matched"),
    ("minsendlevel", "read or set the lowest input level of the hotkey"),
    ("onchar", "read or set the character handler of the hotkey"),
    ("onend", "read or set the end handler of the hotkey"),
    ("onkeydown", "read or set the key-down handler of the hotkey"),
    ("onkeyup", "read or set the key-up handler of the hotkey"),
    ("timeout", "read or set the timeout of the hotkey"),
];

/// The names of the `Hotkey` class.
///
/// The names are the ones the binary gives the class, and they are the
/// whole surface of it.
pub(crate) fn names() -> Vec<&'static str> {
    HOTKEY.iter().map(|(name, _)| *name).collect()
}

/// Why the hotkeys cannot work here.
const REASON: &str =
    "this runtime has no hotkeys yet: the box has a keyboard of its own but nothing that watches it";

/// The object a `Hotkey` makes.
///
/// The object is real: a script may hold it, and the report says that it
/// is there. What it cannot do is the thing that fails.
pub(crate) fn construct(
    _arguments: &[Value],
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    Ok(crate::classes::object("hotkey"))
}

/// A method of a `Hotkey`.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    method: &str,
    _this: &Value,
    _arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(Err(refused(method, line)?))
}

/// A property of a `Hotkey`.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(Err(refused(name, line)?))
}

/// A property of a `Hotkey` that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    name: &str,
    _this: &Value,
    _value: Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    Some(Err(refused(name, line)?))
}

/// The item of a `Hotkey`: the `__Item` of the class.
///
/// `None` means the class has no `__Item` of its own, and it has none:
/// the binary gives the class no `__Item` row. The registry does not
/// route the hotkeys here yet, and the shape of the module is the one of
/// the other classes.
#[allow(dead_code)]
pub(crate) fn item(
    _key: &Value,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    None
}

/// The failure a `Hotkey` gives when it is asked for a name it has.
///
/// `None` means the class has no such name: the runtime then says the
/// value has no such method.
fn refused(name: &str, line: u32) -> Option<RuntimeFailure> {
    let what = HOTKEY.iter().find(|entry| entry.0 == name.to_lowercase()).map(|entry| entry.1)?;
    Some(RuntimeFailure::Error(RuntimeError {
        message: format!("Hotkey cannot {what}: {REASON}."),
        line,
    }))
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    /// The surface of v2.0.19, read out of the binary.
    const SURFACE: &str = include_str!("../../reference/ahk-v2.0.19.surface.tsv");

    /// The names the binary gives a class.
    fn names_of(class: &str) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for line in SURFACE.lines() {
            let mut fields = line.trim_end().split('\t');
            let kind = fields.next().unwrap_or("");
            let owner = fields.next().unwrap_or("");
            let name = fields.next().unwrap_or("");
            if kind == "method" && owner.to_lowercase() == class {
                names.push(name.to_lowercase());
            }
        }
        names
    }

    /// Every name of the binary's table is carried, and nothing else is.
    ///
    /// The table is the whole surface of the class: a name that is not in
    /// it is not a method of the class.
    #[test]
    fn every_name_of_the_binary_is_carried() {
        let carried: Vec<&str> = super::HOTKEY.iter().map(|(name, _)| *name).collect();
        for name in names_of("hotkey") {
            assert!(carried.contains(&name.as_str()), "the Hotkey class does not carry {name}");
        }
        for name in &carried {
            let binary = names_of("hotkey");
            assert!(
                binary.iter().any(|entry| entry == name),
                "the Hotkey class carries {name}, and the binary does not"
            );
        }
    }

    /// A name that is not in the table is not a method of the class.
    #[test]
    fn a_name_that_is_not_in_the_table_is_not_a_method() {
        assert!(super::refused("start", 1).is_some());
        assert!(super::refused("nonsense", 1).is_none());
    }
}
