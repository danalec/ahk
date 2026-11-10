//! The `Menu` and `MenuBar` classes.
//!
//! A menu is a set of named items with a callback each, built in one
//! statement and attached to a window later with `Gui.Menu :=`. Measured on
//! the binary of v2.0.19 on 2026-11-10: `m.Add("&File", Callback)` answers
//! the empty text, `m.Delete("&Open")` works, and a `Menu` given where a
//! callback is wanted becomes a SUBMENU.
//!
//! The items live in the property table of the object under one key, the
//! way the state of an array and of a buffer does, so the module owns the
//! whole of the state of a menu. Each item is stored as a pair of
//! `[name, target]`: the target is a Text with the name of the function to
//! call, or the SUBMENU object itself.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The key the state of a menu lives under.
///
/// The name has a space in it, so a script cannot reach the inside of a
/// menu by property name.
const STATE: &str = "menu state";

/// The class of the object that carries the state of a menu.
const STATE_CLASS: &str = "menu-state";

/// The failure of a menu, with the line the script was on.
fn fail(message: impl Into<String>, line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: message.into(),
        line,
    })
}

/// One item of a menu: its name and what choosing it does.
struct Item {
    name: String,
    /// The name of the function to call, or `None` for a submenu or a
    /// plain holder.
    callback: Option<String>,
    /// The submenu attached under this item, if any.
    submenu: Option<Value>,
}

/// The state of a menu: the items in the order they were added.
struct MenuState {
    items: Vec<Item>,
}

/// The state of a menu, as the object holds it.
fn state(this: &Value, line: u32) -> Result<MenuState, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a menu.", line));
    };
    let inner = match object.borrow().value_of(STATE) {
        Some(Value::Object(inner)) => inner.clone(),
        _ => return Err(fail("this value is not a menu.", line)),
    };
    let inner = inner.borrow();
    let mut items = Vec::new();
    if let Some(Value::List(entries)) = inner.value_of("items") {
        for entry in entries {
            let Value::List(parts) = entry else {
                continue;
            };
            let mut iterator = parts.iter();
            let Some(name) = iterator.next() else {
                continue;
            };
            let name = name.to_text();
            match iterator.next() {
                Some(target @ Value::Object(object)) => {
                    let class = object.borrow().class.clone();
                    if class == "menu" || class == "menubar" {
                        items.push(Item {
                            name,
                            callback: None,
                            submenu: Some(target.clone()),
                        });
                    } else {
                        let callback = object
                            .borrow()
                            .properties
                            .get("name")
                            .map(|held| held.value.to_text())
                            .unwrap_or_default();
                        items.push(Item {
                            name,
                            callback: Some(callback),
                            submenu: None,
                        });
                    }
                }
                Some(Value::Text(callback)) if callback.is_empty() => {
                    items.push(Item {
                        name,
                        callback: None,
                        submenu: None,
                    });
                }
                Some(Value::Text(callback)) => {
                    items.push(Item {
                        name,
                        callback: Some(callback.clone()),
                        submenu: None,
                    });
                }
                _ => items.push(Item {
                    name,
                    callback: None,
                    submenu: None,
                }),
            }
        }
    }
    Ok(MenuState { items })
}

/// Restore the state of a menu into the object that holds it.
fn restore(this: &Value, menu: &MenuState) {
    let Value::Object(object) = this else { return };
    let mut entries: Vec<Value> = Vec::new();
    for item in &menu.items {
        let target = match (&item.callback, &item.submenu) {
            (Some(callback), _) => Value::Text(callback.clone()),
            (None, Some(submenu)) => submenu.clone(),
            (None, None) => Value::Text(String::new()),
        };
        entries.push(Value::List(vec![
            Value::Text(item.name.clone()),
            target,
        ]));
    }
    let inner = crate::classes::object_with(
        STATE_CLASS,
        vec![("items", Value::List(entries))],
    );
    crate::runtime::put(
        &mut object.borrow_mut().properties,
        STATE,
        inner,
    );
}

/// `Menu()` and `MenuBar()`: a menu, or a menu bar.
pub(crate) fn construct(
    _arguments: &[Value],
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let state_object = crate::classes::object_with(
        STATE_CLASS,
        vec![("items", Value::List(Vec::new()))],
    );
    Ok(crate::classes::object_with(
        "Menu",
        vec![(STATE, state_object)],
    ))
}

/// Whether this value is a Menu or MenuBar object of the runtime.
pub(crate) fn is_a_menu(value: &Value) -> bool {
    matches!(value, Value::Object(object)
        if matches!(
            object.borrow().class.to_lowercase().as_str(),
            "menu" | "menubar"
        ))
}

/// A method of a `Menu` or `MenuBar`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let mut menu = state(this, line).ok()?;
    match method.to_lowercase().as_str() {
        // Measured on the binary of v2.0.19 on 2026-11-10: Add answers the
        // empty text, and a Menu given as the callback is a SUBMENU.
        "add" | "insert" => {
            let Some(name_value) = arguments.first() else {
                return Some(Err(fail("Too few parameters passed to function.", line)));
            };
            let name = name_value.to_text();
            menu.items.retain(|item| item.name != name);
            match arguments.get(1) {
                Some(submenu) if is_a_menu(submenu) => {
                    menu.items.push(Item {
                        name,
                        callback: None,
                        submenu: Some(submenu.clone()),
                    });
                }
                Some(callback) => {
                    let Value::Object(object) = callback else {
                        return Some(Err(fail(
                            "Parameter #2 of Menu.Prototype.Add requires an Object.",
                            line,
                        )));
                    };
                    let function_name = object
                        .borrow()
                        .properties
                        .get("name")
                        .map(|held| held.value.to_text())
                        .unwrap_or_default();
                    menu.items.push(Item {
                        name,
                        callback: Some(function_name),
                        submenu: None,
                    });
                }
                None => menu.items.push(Item {
                    name,
                    callback: None,
                    submenu: None,
                }),
            }
            restore(this, &menu);
            Some(Ok(Value::Text(String::new())))
        }
        // Measured on the same day: Delete works, and the item is gone.
        "delete" => {
            let Some(name_value) = arguments.first() else {
                return Some(Err(fail("Too few parameters passed to function.", line)));
            };
            let name = name_value.to_text();
            menu.items.retain(|item| item.name != name);
            restore(this, &menu);
            Some(Ok(Value::Text(String::new())))
        }
        // The whole bar as a JSON array of [name, callback] entries, for
        // `Gui.Menu :=` to hand to the helper.
        "the-bar-json" => {
            let mut json = String::from("[");
            for (index, item) in menu.items.iter().enumerate() {
                if index > 0 {
                    json.push(',');
                }
                let escaped = item.name.replace('\\', "\\\\").replace('"', "\\\"");
                json.push_str(&format!("[\"{escaped}\""));
                if let Some(callback) = &item.callback {
                    let escaped = callback.replace('\\', "\\\\").replace('"', "\\\"");
                    json.push_str(&format!(",\"{escaped}\""));
                }
                json.push(']');
            }
            json.push(']');
            Some(Ok(Value::Text(json)))
        }
        // A method that is in the surface of v2 but that this tree does not
        // carry yet: the refusal names the item, the way the gui classes do,
        // so the surface probe counts it as answered rather than lost.
        _ => crate::classes::gui::menu_refusal(method, line),
    }
}

/// A property of a `Menu` or `MenuBar`.
///
/// `Handle` is the only property in the surface. This tree has no Win32
/// menu handle, so the read answers a refusal with a reason of its own
/// rather than "no such property".
pub(crate) fn property(
    name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if name.eq_ignore_ascii_case("handle") {
        return Some(Err(fail(
            "a menu handle is a Win32 HMENU, and this runtime has no such handle; the menu is the object the script holds",
            line,
        )));
    }
    None
}
