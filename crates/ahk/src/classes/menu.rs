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
use alloc::string::{String, ToString};
use alloc::vec;
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
    /// The value of the function to call, when there is one.
    ///
    /// The name is not enough: a closure has no name in v2, and only the
    /// value of it can be called. The name is kept as well, because the
    /// helper of the window draws the bar and is told the name.
    function: Option<Value>,
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
                            function: None,
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
                            function: Some(target.clone()),
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
                        function: None,
                        submenu: None,
                        checked,
                        enabled,
                    });
                }
                Some(Value::Text(callback)) => {
                    items.push(Item {
                        name,
                        callback: Some(callback.clone()),
                        function: None,
                        submenu: None,
                        checked,
                        enabled,
                    });
                }
                _ => items.push(Item {
                    name,
                    callback: None,
                    function: None,
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
        let target = match (&item.function, &item.callback, &item.submenu) {
            (Some(function), _, _) => function.clone(),
            (None, Some(callback), _) => Value::Text(callback.clone()),
            (None, None, Some(submenu)) => submenu.clone(),
            (None, None, None) => Value::Text(String::new()),
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

/// The callback of an item and its 1-based position, for the `menu:<name>`
/// event the helper sends when the item is chosen.
///
/// Documented on v2.0.19 on 2026-11-10: the callback of a menu item is
/// called with (ItemName, ItemPos, MyMenu) -- the name of the item, its
/// 1-based position, and the menu object it was added to.
///
/// The callback is the **value** of the function and not its name: a
/// closure has no name in v2, and only the value of it can be called. An
/// item that was given a name and no value answers with the name, and the
/// runtime calls it by it.
pub(crate) fn the_callback_of(this: &Value, item: &str, line: u32) -> Option<(Value, i64)> {
    let menu = state(this, line).ok()?;
    let index = menu.items.iter().position(|held| held.name == item)?;
    let held = &menu.items[index];
    let function = match (&held.function, &held.callback) {
        (Some(function), _) => function.clone(),
        (None, Some(callback)) => Value::Text(callback.clone()),
        (None, None) => return None,
    };
    Some((function, index as i64 + 1))
}

/// The name of the item by default of a menu, if the menu has one.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the `Default` of a
/// menu is the name of the item that runs when the menu is chosen without
/// a name -- the `A_TrayMenu` of the script has "&Open" as its default.
pub(crate) fn the_default_of(this: &Value) -> Option<String> {
    let menu = state(this, 0).ok()?;
    if menu.default.is_empty() {
        None
    } else {
        Some(menu.default.clone())
    }
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
