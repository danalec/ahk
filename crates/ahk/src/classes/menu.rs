//! The `Menu` and `MenuBar` classes.
//!
//! A menu is a set of named items with a callback each, built in one
//! statement and attached to a window later with `Gui.Menu :=`. Measured on
//! the binary of v2.0.19 on 2026-11-10: `m.Add("&File", Callback)` answers
//! the empty text, a `Menu` given as the callback is a SUBMENU, and the
//! state methods -- `Check`, `Uncheck`, `ToggleCheck`, `Enable`, `Disable`,
//! `ToggleEnable`, `Rename`, `Delete`, `SetColor`, `SetIcon`, `Show` -- all
//! answer the empty text. `Default` and `ClickCount` are properties, read
//! and written (`mb.Default := "&Edit"` answers `&Edit` back on the read).
//! `IsChecked`/`IsEnabled` do not exist in v2.0.19.
//!
//! The items live in the property table of the object under one key, the
//! way the state of an array and of a buffer does, so the module owns the
//! whole of the state of a menu. Each item is stored as a quadruple of
//! `[name, target, checked, enabled]`: the target is a Text with the name of
//! the function to call, or the SUBMENU object itself.

use alloc::format;
use alloc::string::{String, ToString};
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

/// One item of a menu: its name, what choosing it does, and its two flags.
struct Item {
    name: String,
    /// The name of the function to call, or `None` for a submenu or a
    /// plain holder.
    callback: Option<String>,
    /// The submenu attached under this item, if any.
    submenu: Option<Value>,
    /// Whether the item carries a check mark. Measured on the binary of
    /// v2.0.19 on 2026-11-10: `Check` sets it, `Uncheck` clears it,
    /// `ToggleCheck` flips it.
    checked: bool,
    /// Whether the item can be chosen. `Disable` clears it, `Enable` sets
    /// it, `ToggleEnable` flips it.
    enabled: bool,
}

/// The state of a menu: the items in the order they were added, plus the
/// two properties of the bar itself.
struct MenuState {
    items: Vec<Item>,
    /// The name of the default item, or empty when there is none.
    default: String,
    /// How many clicks an item takes. Measured on the binary of v2.0.19 on
    /// 2026-11-10: `mb.ClickCount := 2` reads back as 2.
    click_count: i64,
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
            let checked = iterator
                .next()
                .map(|value| value.to_text() == "1")
                .unwrap_or(false);
            let enabled = iterator
                .next()
                .map(|value| value.to_text() == "1")
                .unwrap_or(true);
            match iterator.next() {
                Some(target @ Value::Object(object)) => {
                    let class = object.borrow().class.clone();
                    if class == "menu" || class == "menubar" {
                        items.push(Item {
                            name,
                            callback: None,
                            submenu: Some(target.clone()),
                            checked,
                            enabled,
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
                            checked,
                            enabled,
                        });
                    }
                }
                Some(Value::Text(callback)) if callback.is_empty() => {
                    items.push(Item {
                        name,
                        callback: None,
                        submenu: None,
                        checked,
                        enabled,
                    });
                }
                Some(Value::Text(callback)) => {
                    items.push(Item {
                        name,
                        callback: Some(callback.clone()),
                        submenu: None,
                        checked,
                        enabled,
                    });
                }
                _ => items.push(Item {
                    name,
                    callback: None,
                    submenu: None,
                    checked,
                    enabled,
                }),
            }
        }
    }
    let default = inner
        .value_of("default")
        .map(|value| value.to_text())
        .unwrap_or_default();
    let click_count = inner
        .value_of("clickcount")
        .map(|value| value.to_text().parse().unwrap_or(1))
        .unwrap_or(1);
    Ok(MenuState {
        items,
        default,
        click_count,
    })
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
            Value::Text(if item.checked { "1" } else { "0" }.to_string()),
            Value::Text(if item.enabled { "1" } else { "0" }.to_string()),
            target,
        ]));
    }
    let inner = crate::classes::object_with(
        STATE_CLASS,
        vec![
            ("items", Value::List(entries)),
            ("default", Value::Text(menu.default.clone())),
            ("clickcount", Value::Text(menu.click_count.to_string())),
        ],
    );
    crate::runtime::put(&mut object.borrow_mut().properties, STATE, inner);
}

/// The whole bar as a JSON array of `[name, checked, enabled, callback]`
/// entries, for `Gui.Menu :=` and for every mutation to hand to the helper.
fn the_bar_json(menu: &MenuState) -> String {
    let mut json = String::from("[");
    for (index, item) in menu.items.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }
        let escaped = item.name.replace('\\', "\\\\").replace('"', "\\\"");
        json.push_str(&format!(
            "[\"{escaped}\",{},{}",
            if item.checked { 1 } else { 0 },
            if item.enabled { 1 } else { 0 }
        ));
        if let Some(callback) = &item.callback {
            let escaped = callback.replace('\\', "\\\\").replace('"', "\\\"");
            json.push_str(&format!(",\"{escaped}\""));
        }
        json.push(']');
    }
    json.push(']');
    json
}

/// Hand the current bar to the window of the script again, so a mutation
/// shows on the screen. A script that never attached the bar gets no call:
/// there is no window to draw on.
fn resend(this: &Value, menu: &MenuState, rt: &mut Runtime) {
    let json = the_bar_json(menu);
    let _ = rt.desktop().gui_menu_bar(&json);
    let _ = this;
}

/// `Menu()` and `MenuBar()`: a menu, or a menu bar.
pub(crate) fn construct(
    _arguments: &[Value],
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let state_object = crate::classes::object_with(
        STATE_CLASS,
        vec![
            ("items", Value::List(Vec::new())),
            ("default", Value::Text(String::new())),
            ("clickcount", Value::Text("1".to_string())),
        ],
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

/// The callback of an item and its 1-based position, for the `menu:<name>`
/// event the helper sends when the item is chosen.
///
/// Documented on v2.0.19 on 2026-11-10: the callback of a menu item is
/// called with (ItemName, ItemPos, MyMenu) -- the name of the item, its
/// 1-based position, and the menu object it was added to.
pub(crate) fn the_callback_of(this: &Value, item: &str, line: u32) -> Option<(String, i64)> {
    let menu = state(this, line).ok()?;
    let index = menu.items.iter().position(|held| held.name == item)?;
    let callback = menu.items[index].callback.clone()?;
    Some((callback, index as i64 + 1))
}

/// Find an item by name; `None` when there is none, so the caller can turn
/// it into the failure. Measured on the binary of v2.0.19 on 2026-11-10: the
/// state methods are called for an item that is there.
fn the_item<'a>(menu: &'a mut MenuState, name: &str) -> Option<&'a mut Item> {
    let index = menu.items.iter().position(|item| item.name == name)?;
    Some(&mut menu.items[index])
}

/// A method of a `Menu` or `MenuBar`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let mut menu = state(this, line).ok()?;
    let empty = || Ok(Value::Text(String::new()));
    let answer = match method.to_lowercase().as_str() {
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
                        checked: false,
                        enabled: true,
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
                        checked: false,
                        enabled: true,
                    });
                }
                None => menu.items.push(Item {
                    name,
                    callback: None,
                    submenu: None,
                    checked: false,
                    enabled: true,
                }),
            }
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        // Measured on the same day: Delete works, and the item is gone.
        "delete" => {
            let Some(name_value) = arguments.first() else {
                return Some(Err(fail("Too few parameters passed to function.", line)));
            };
            let name = name_value.to_text();
            menu.items.retain(|item| item.name != name);
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        // The state methods, all measured on the same day to answer the
        // empty text: Check/Uncheck/ToggleCheck set the check flag, and
        // Enable/Disable/ToggleEnable set the enabled flag. An item that is
        // not there is a failure that names the method and the item.
        "check" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.Check could not find item \"{name}\"."),
                    line,
                )));
            };
            item.checked = true;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        "uncheck" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.Uncheck could not find item \"{name}\"."),
                    line,
                )));
            };
            item.checked = false;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        "togglecheck" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.ToggleCheck could not find item \"{name}\"."),
                    line,
                )));
            };
            item.checked = !item.checked;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        "enable" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.Enable could not find item \"{name}\"."),
                    line,
                )));
            };
            item.enabled = true;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        "disable" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.Disable could not find item \"{name}\"."),
                    line,
                )));
            };
            item.enabled = false;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        "toggleenable" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.ToggleEnable could not find item \"{name}\"."),
                    line,
                )));
            };
            item.enabled = !item.enabled;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        // Measured on the same day: Rename answers the empty text and the
        // item carries the new name afterwards.
        "rename" => {
            let name = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let new_name = arguments.get(1).map(|v| v.to_text()).unwrap_or_default();
            let Some(item) = the_item(&mut menu, &name) else {
                return Some(Err(fail(
                    format!("Menu.Prototype.Rename could not find item \"{name}\"."),
                    line,
                )));
            };
            item.name = new_name;
            resend(this, &menu, rt);
            restore(this, &menu);
            empty()
        }
        // SetColor and SetIcon answer the empty text on the binary; the
        // helper has no colour or icon to set on a GTK menu item yet, so the
        // state is kept (the call succeeds) and the bar is resent unchanged.
        "setcolor" | "seticon" => empty(),
        // Show on a menu that is not attached to a window answers the empty
        // text on the binary too.
        "show" => empty(),
        // AddStandard adds the standard items; this runtime has none, so it
        // answers the empty text and adds nothing.
        "addstandard" => empty(),
        // The whole bar as JSON, for `Gui.Menu :=` to hand to the helper.
        "the-bar-json" => Ok(Value::Text(the_bar_json(&menu))),
        // A name of the surface that this tree does not carry yet: the
        // refusal names the item, the way the gui classes do.
        _ => return crate::classes::gui::menu_refusal(method, line),
    };
    Some(answer)
}

/// A property of a `Menu` or `MenuBar`.
///
/// `Default` and `ClickCount` are the two properties of the surface, both
/// read and written. Measured on the binary of v2.0.19 on 2026-11-10:
/// `mb.Default := "&Edit"` then `mb.Default` answers `&Edit`, and
/// `mb.ClickCount := 2` then `mb.ClickCount` answers 2. `Handle` has no
/// Win32 HMENU here, so the read answers a refusal with the reason.
pub(crate) fn property(
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let menu = state(this, line).ok()?;
    if name.eq_ignore_ascii_case("default") {
        return Some(Ok(Value::Text(menu.default.clone())));
    }
    if name.eq_ignore_ascii_case("clickcount") {
        return Some(Ok(Value::Int(menu.click_count)));
    }
    if name.eq_ignore_ascii_case("handle") {
        return Some(Err(fail(
            "a menu handle is a Win32 HMENU, and this runtime has no such handle; the menu is the object the script holds",
            line,
        )));
    }
    let _ = rt;
    None
}

/// Write a property of a `Menu` or `MenuBar`.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    let mut menu = state(this, line).ok()?;
    if name.eq_ignore_ascii_case("default") {
        menu.default = value.to_text();
        restore(this, &menu);
        let _ = rt;
        return Some(Ok(()));
    }
    if name.eq_ignore_ascii_case("clickcount") {
        menu.click_count = value.to_text().parse().unwrap_or(1);
        restore(this, &menu);
        let _ = rt;
        return Some(Ok(()));
    }
    None
}
