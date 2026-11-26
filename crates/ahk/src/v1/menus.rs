//! The menus of the v1, and the names that they have.
//!
//! The v1 has the `Menu` as a command, and a menu is a thing with a name:
//! the `Menu, MyMenu, Add, Item` makes a menu that is called `MyMenu`, and
//! the `MenuGetHandle` and the `MenuGetName` are the two ways of asking
//! about it.
//!
//! The v2 has the `Menu` as a class, and a script holds the object of it.
//! The tree keeps the menus of the v1 in the runtime, by their names, so
//! that the two dialects agree about the menu that a name means.
//!
//! The menu of the v1 is a menu of the machine: the handle that the
//! `MenuGetHandle` answers is the handle of the machine, and the dropdown
//! of the bar of a window is the menu that the script named. The tree
//! keeps the items of the menu as well, because the machine forgets the
//! name of an item and the script may ask about it.
//!
//! Measured on the binary of v1.1.37.02:
//!
//! - the `Menu, MyMenu, Add, Item` makes a menu that is called `MyMenu`;
//! - the `MenuGetHandle("MyMenu")` answers the number that names it;
//! - the `MenuGetName` of that number answers the name `MyMenu`;
//! - the `MenuGetHandle` of a menu that is not there throws.
//!
//! Measured on the binary of v1.1.37.02:
//!
//! - the `MenuGetHandle` of a menu that is not there answers **zero** and
//!   does not throw;
//! - the `MenuGetName` of a handle that is not there answers the empty
//!   text and does not throw;
//! - the `Menu, MyMenu, Add, Item` of a script that has no label `Item`
//!   is refused with "Target label does not exist. Specifically: Item";
//! - the `Menu, MyMenu, Add` (with no item) is a separator, and it is
//!   not an error.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, Value};

/// The failure of a menu that is not there.
const NO_MENU: &str = "Target menu not found.";

/// The first number that names a menu of the v1 when the machine cannot
/// make one.
///
/// The number of a menu is the handle of it, and the handles of the
/// machine are numbers that grow. The tree gives a number of its own when
/// the machine has no menu to name.
const THE_FIRST_HANDLE: u64 = 0x1_0000;

/// An item of a menu of the v1.
pub(crate) struct TheItem {
    /// The name of the item, as the script wrote it.
    pub(crate) name: String,
    /// The name of the function of the script that choosing it runs.
    pub(crate) callback: String,
    /// The name of the menu that the item opens.
    ///
    /// The v1: an item whose callback begins with a colon is a submenu,
    /// and the name of the menu is what is behind the colon.
    pub(crate) submenu: String,
    /// Whether the item carries a mark.
    pub(crate) checked: bool,
    /// Whether the item can be chosen.
    pub(crate) enabled: bool,
}

/// A menu of the v1.
pub(crate) struct TheMenu {
    /// The name that the script gave it.
    pub(crate) name: String,
    /// The number that names it when the machine has no menu of its own.
    pub(crate) handle: u64,
    /// The menu of the machine, when the machine can make one.
    ///
    /// The v1 builds the menu of the machine when the menu is shown or
    /// attached to a window, and the handle of it is the one that the
    /// `MenuGetHandle` answers. The tree builds it at once, and the
    /// handle of it never changes: the dropdown of the bar of a window is
    /// the menu that the script named, and the message that the machine
    /// sends when the dropdown opens carries that handle.
    pub(crate) machine: Option<u64>,
    /// The items of it, in the order of the script.
    pub(crate) items: Vec<TheItem>,
    /// The item that the menu takes when the window of the script is
    /// activated.
    pub(crate) default: Option<String>,
}

/// The menus of the script, by their names.
fn the_menus<'a>(rt: &'a mut Runtime<'_>) -> &'a mut Vec<TheMenu> {
    rt.the_menus_of_the_v1()
}

/// The menu of this name, made when it is not there yet.
///
/// The list of the menus grows when the menu is made, and that is why the
/// parameter is the list and not a slice of it. The menu of the machine
/// is made with it: the handle that the `MenuGetHandle` answers is the
/// handle of the machine.
fn the_menu_of_the_name<'a>(rt: &'a mut Runtime<'_>, name: &str) -> &'a mut TheMenu {
    let wanted = name.to_lowercase();
    let found = {
        let menus = the_menus(rt);
        menus
            .iter()
            .position(|menu| menu.name.to_lowercase() == wanted)
    };
    if let Some(index) = found {
        return &mut the_menus(rt)[index];
    }
    // The menu of the machine is made first: the tree asks the machine
    // for it, and the machine may not be able to make one.
    let machine = rt.desktop().menu_create().ok();
    let menus = the_menus(rt);
    // The handle is the number of the machine when the machine has one.
    let handle = machine.unwrap_or(THE_FIRST_HANDLE + menus.len() as u64);
    menus.push(TheMenu {
        name: name.to_string(),
        handle,
        machine,
        items: Vec::new(),
        default: None,
    });
    let last = menus.len() - 1;
    &mut menus[last]
}

/// The menu of this name, when the script has one.
fn the_menu_that_is_there<'a>(rt: &'a mut Runtime<'_>, name: &str) -> Option<&'a mut TheMenu> {
    let wanted = name.to_lowercase();
    let menus = the_menus(rt);
    menus
        .iter_mut()
        .find(|menu| menu.name.to_lowercase() == wanted)
}

/// The menu of the machine of this name, when the script has one.
pub(crate) fn the_machine_menu_of(rt: &mut Runtime<'_>, name: &str) -> Option<u64> {
    the_menu_that_is_there(rt, name).and_then(|menu| menu.machine)
}

/// The failure of a menu that is not there.
fn the_failure_of_a_menu_that_is_not_there(line: u32) -> RuntimeError {
    RuntimeError {
        message: NO_MENU.to_string(),
        line,
    }
}

/// The item of a menu, when the menu has it.
fn the_item<'a>(menu: &'a mut TheMenu, item: &str) -> Option<&'a mut TheItem> {
    menu.items
        .iter_mut()
        .find(|held| held.name.eq_ignore_ascii_case(item))
}

/// The callback of an item of a menu, and the submenu that it opens.
///
/// The v1: "To have MenuItemName become a submenu -- which is a menu item
/// that opens a new menu when selected -- specify for LabelOrSubmenu a
/// colon followed by the MenuName of an existing custom menu" (the CHM of
/// the v1.1.37.02, `Menu.htm`).
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `Menu, PVbar, Add, &File, :PVfile` of a script of the v1 opens the
/// menu `PVfile` when the item is chosen, and the bar of the window has
/// five items that are five popups.
fn the_callback_and_the_submenu(callback: &str) -> (String, String) {
    let trimmed = callback.trim();
    match trimmed.strip_prefix(':') {
        Some(name) => (String::new(), name.trim().to_string()),
        None => (trimmed.to_string(), String::new()),
    }
}

/// The name of the function of the script that an item runs.
///
/// The v1: "If LabelOrSubmenu is omitted, MenuItemName will be used as
/// both the label and the menu item's name" (the CHM of the v1.1.37.02,
/// `Menu.htm`).
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `Menu, MyMenu, Add, Item` of a script that has no label `Item` is
/// refused with "Target label does not exist. Specifically: Item".
fn the_callback_of_the_item(item: &str, callback: &str) -> String {
    if callback.trim().is_empty() && !item.trim().is_empty() {
        return item.trim().to_string();
    }
    callback.trim().to_string()
}

/// `Menu, MyMenu, Add, Item, Callback`: an item of a menu of the script.
pub(crate) fn add_item(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    callback: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let (callback, submenu) = the_callback_and_the_submenu(callback);
    // The item of a menu of the v1 is the name of it, and what choosing
    // it does is the name of the function of the script.
    let callback = if submenu.is_empty() {
        the_callback_of_the_item(item, &callback)
    } else {
        callback
    };
    // The v1: "If MenuItemName does not yet exist, it will be added to the
    // menu. Otherwise, MenuItemName is updated with the newly specified
    // LabelOrSubmenu" (the CHM of the v1.1.37.02, `Menu.htm`).
    //
    // Measured on the binary of AutoHotkey v1.1.37.02: the
    // `Menu, PVbar, Add, &File, :PVfile` of a script of the v1 is
    // written twice -- once when the bar is built and once when the
    // attachments are repaired -- and the bar of the window has five
    // items, and not ten.
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    // The v1: a blank MenuItemName always appends a new separator line;
    // it never updates an earlier separator.
    let held = if item.is_empty() {
        None
    } else {
        the_item(menu, item)
    };
    match held {
        Some(held) => {
            held.callback = callback.clone();
            held.submenu = submenu.clone();
        }
        None => menu.items.push(TheItem {
            name: item.to_string(),
            callback: callback.clone(),
            submenu: submenu.clone(),
            checked: false,
            enabled: true,
        }),
    }
    // The menu of the machine carries the item, and the item that opens
    // a menu carries the handle of it.
    if let Some(machine) = machine {
        // The item goes into the menu of the machine first, and the menu
        // that it opens goes into it after: the item of the machine is
        // the one that the menu of the script opens.
        let _ = rt.desktop().menu_update(machine, item, &callback);
        if !submenu.is_empty() {
            // The menu that the item opens is made when it is not there:
            // the v1 makes a custom menu the first time its name is used.
            let child = {
                let child = the_menu_of_the_name(rt, &submenu);
                child.machine
            };
            if let Some(child) = child {
                let _answer = rt.desktop().menu_set_submenu(machine, item, child);
                #[cfg(feature = "std")]
                if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
                    eprintln!("the menu of the item {item:?} is {:?}", _answer);
                }
            }
        }
    }
    let _ = line;
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Insert, Item, Callback, Options`: an item of the menu of
/// the script, above the last one.
///
/// The v1 has no place for the item: it goes above the last item of the
/// menu, and it is the only one when the menu is empty.
pub(crate) fn insert_item(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    callback: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let (callback, submenu) = the_callback_and_the_submenu(callback);
    let callback = if submenu.is_empty() {
        the_callback_of_the_item(item, &callback)
    } else {
        callback
    };
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let held = TheItem {
        name: item.to_string(),
        callback: callback.clone(),
        submenu: submenu.clone(),
        checked: false,
        enabled: true,
    };
    let at = menu.items.len().saturating_sub(1);
    menu.items.insert(at, held);
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_insert(machine, item, &callback);
        if !submenu.is_empty() {
            let child = {
                let child = the_menu_of_the_name(rt, &submenu);
                child.machine
            };
            if let Some(child) = child {
                let _ = rt.desktop().menu_set_submenu(machine, item, child);
            }
        }
    }
    let _ = line;
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Delete, Item`: the removal of an item of a menu.
pub(crate) fn delete_item(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let before = menu.items.len();
    menu.items
        .retain(|held| !held.name.eq_ignore_ascii_case(item));
    if menu.items.len() == before {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    }
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_delete(machine, item);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, DeleteAll`: the removal of every item of a menu.
///
/// The v1: "Unlike a menu entirely deleted by the Delete sub-command, an
/// empty menu still exists and thus any other menus that use it as a
/// submenu will still work" (the CHM of the v1.1.37.02, `Menu.htm`).
///
/// The menu of the machine stays: the bar of a window holds it, and the
/// dropdown of the bar is the menu of the machine.
pub(crate) fn delete_all(
    rt: &mut Runtime<'_>,
    name: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    if menu.items.is_empty() {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    }
    menu.items.clear();
    menu.default = None;
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_delete_all(machine);
    }
    let _ = line;
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Delete`: the removal of the menu itself.
///
/// The v1: "If MenuItemName is omitted, the entire MenuName menu will be
/// deleted as will any menu items in other menus that use MenuName as a
/// submenu" (the CHM of the v1.1.37.02, `Menu.htm`).
pub(crate) fn delete_the_menu(
    rt: &mut Runtime<'_>,
    name: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let found = the_menu_that_is_there(rt, name).is_some();
    if !found {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    }
    // The v1: "If MenuItemName is omitted, the entire MenuName menu will
    // be deleted as will any menu items in other menus that use MenuName
    // as a submenu" (the CHM of the v1.1.37.02, `Menu.htm`).
    //
    // Measured on the binary of the v1.1.37.02: the
    // `Menu, MyBar, Delete` of a bar that opens the menu `MyMenu` leaves
    // `MyMenu` alive, with its items, and the item of the bar that
    // opened it is gone.
    let wanted = name.to_lowercase();
    // The items of the other menus that open the menu go away, in the
    // script and in the machine.
    let holders: Vec<(u64, Vec<String>)> = {
        let menus = the_menus(rt);
        menus
            .iter()
            .filter(|menu| !menu.name.to_lowercase().eq_ignore_ascii_case(&wanted))
            .filter_map(|menu| {
                let machine = menu.machine?;
                let items: Vec<String> = menu
                    .items
                    .iter()
                    .filter(|item| item.submenu.eq_ignore_ascii_case(&wanted))
                    .map(|item| item.name.clone())
                    .collect();
                if items.is_empty() {
                    None
                } else {
                    Some((machine, items))
                }
            })
            .collect()
    };
    for (machine, items) in holders {
        for item in items {
            let _ = rt.desktop().menu_delete(machine, &item);
            let menus = the_menus(rt);
            if let Some(menu) = menus
                .iter_mut()
                .find(|menu| menu.machine == Some(machine))
            {
                menu.items.retain(|held| !held.name.eq_ignore_ascii_case(&item));
            }
        }
    }
    // The menu of the machine goes away with the menu of the script. The
    // machine refuses when a window of the script uses the menu as its
    // menu bar, and the menu of the script stays.
    let machine = the_menu_that_is_there(rt, name).and_then(|menu| menu.machine);
    if let Some(machine) = machine {
        rt.desktop()
            .menu_delete_the_menu(machine)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
    }
    let menus = the_menus(rt);
    menus.retain(|menu| !menu.name.eq_ignore_ascii_case(&wanted));
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Enable, Item` and `Menu, MyMenu, Disable, Item`: whether
/// an item of a menu can be chosen.
pub(crate) fn set_enabled(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    on: bool,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let Some(held) = the_item(menu, item) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    held.enabled = on;
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_enable(machine, item, on);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, ToggleEnable, Item`: whether an item of a menu can be
/// chosen, the other way round.
pub(crate) fn toggle_enabled(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let Some(held) = the_item(menu, item) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    held.enabled = !held.enabled;
    let on = held.enabled;
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_enable(machine, item, on);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Check, Item` and `Menu, MyMenu, Uncheck, Item`: whether
/// an item of a menu carries a mark.
pub(crate) fn set_checked(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    checked: bool,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let Some(held) = the_item(menu, item) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    held.checked = checked;
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_check(machine, item, checked);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, ToggleCheck, Item`: whether an item of a menu carries a
/// mark, the other way round.
pub(crate) fn toggle_checked(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let Some(held) = the_item(menu, item) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    held.checked = !held.checked;
    let checked = held.checked;
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_check(machine, item, checked);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Default, Item`: the item that a menu takes when the
/// window of the script is activated.
pub(crate) fn set_default(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    if the_item(menu, item).is_none() {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    }
    menu.default = Some(item.to_string());
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, NoDefault`: the menu that takes no item when the window
/// of the script is activated.
pub(crate) fn no_default(
    rt: &mut Runtime<'_>,
    name: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    menu.default = None;
    let _ = line;
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Rename, NewName, OldName`: the new name of an item of a
/// menu.
pub(crate) fn rename_item(
    rt: &mut Runtime<'_>,
    name: &str,
    old: &str,
    new: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    let Some(held) = the_item(menu, old) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    held.name = new.to_string();
    if let Some(machine) = machine {
        let _ = rt.desktop().menu_rename(machine, old, new);
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Icon, Item, IconFile, IconNumber`: the icon of an item of
/// a menu.
///
/// The v1: "Menu, MenuName, Icon, MenuItemName, FileName, IconNumber,
/// IconWidth" (the CHM of the v1.1.37.02, `Menu.htm`).
pub(crate) fn set_icon(
    rt: &mut Runtime<'_>,
    name: &str,
    item: &str,
    icon: &str,
    number: i64,
    line: u32,
) -> Result<Value, RuntimeError> {
    let menu = the_menu_of_the_name(rt, name);
    let machine = menu.machine;
    if the_item(menu, item).is_none() {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    }
    if let Some(machine) = machine {
        // The name of the file may be empty: the icon goes away from the
        // item.
        //
        // The v1: "If FileName is omitted, any custom icon already
        // associated with the menu item will be removed" (the CHM of the
        // v1.1.37.02, `Menu.htm`).
        //
        // Measured on the binary of the v1.1.37.02: the
        // `Menu, MyMenu, Icon, Item` of a menu whose item carries an icon
        // takes the icon away from it, and the tree kept it.
        let canonical = if icon.trim().is_empty() {
            String::new()
        } else {
            rt.desktop()
                .canonical_path(icon.trim())
                .unwrap_or_else(|_| icon.trim().to_string())
        };
        rt.desktop()
            .menu_set_icon(machine, item, &canonical, number)
            .map_err(|reason| RuntimeError {
                message: reason,
                line,
            })?;
    }
    Ok(Value::Text("".into()))
}

/// `Menu, MyMenu, Show, X, Y`: the menu of the script at a place.
///
/// The v1: "Displays the specified menu. ... If both X and Y are omitted,
/// the menu is displayed at the current position of the mouse cursor"
/// (the CHM of the v1.1.37.02, `Menu.htm`).
///
/// The label of the item that was chosen runs as a new thread: the tree
/// puts it in the queue of the callbacks, and it runs between the
/// statements of the script.
pub(crate) fn show(
    rt: &mut Runtime<'_>,
    name: &str,
    x: &str,
    y: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let Some(machine) = the_machine_menu_of(rt, name) else {
        return Err(the_failure_of_a_menu_that_is_not_there(line));
    };
    let place = (x.trim().parse::<i32>().ok(), y.trim().parse::<i32>().ok());
    // The runtime is kept for the thread while the menu is open: the
    // machine calls the script back while the menu is there, and the
    // callback of the machine finds the runtime and runs the function of
    // the script.
    let chosen = rt
        .the_menu_of_the_script_is_shown(machine, place.0, place.1)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    if let Some(callback) = chosen {
        if !callback.trim().is_empty() {
            rt.the_callback_of_a_menu_is_waiting(callback);
        }
    }
    Ok(Value::Text("".into()))
}

/// `MenuGetHandle(MenuName)`: the number that names a menu of the script.
///
/// The v1: the handle of a menu that is not there is zero, and it is not
/// an error.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `MenuGetHandle("NoSuchMenu")` answers zero, and the script goes on.
pub(crate) fn the_handle_of(
    rt: &mut Runtime<'_>,
    name: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let _ = line;
    // The menu that is not there is the number zero: the v1 does not
    // throw, and the script goes on.
    let Some(menu) = the_menu_that_is_there(rt, name) else {
        return Ok(Value::Int(0));
    };
    let machine = menu.machine;
    let handle = menu.handle;
    // The handle of the machine is the handle of the menu: the dropdown
    // of the bar of a window is the menu of the machine, and the message
    // that the machine sends when it opens carries that handle.
    if let Some(machine) = machine {
        if let Ok(real) = rt.desktop().menu_handle(machine) {
            return Ok(Value::Int(real));
        }
    }
    Ok(Value::Int(handle as i64))
}

/// `MenuGetName(Handle)`: the name of a menu of the script.
///
/// The v1: the name of a handle that is not there is the empty text, and
/// it is not an error.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `MenuGetName(12345)` answers the empty text, and the script goes on.
pub(crate) fn the_name_of(
    rt: &mut Runtime<'_>,
    handle: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    let _ = line;
    let wanted = handle.trim();
    // The handles of the machine of the menus of the script, by the name
    // of the menu.
    let mut names: Vec<(u64, String)> = Vec::new();
    {
        let menus = the_menus(rt);
        for menu in menus.iter() {
            names.push((menu.handle, menu.name.clone()));
        }
    }
    for (number, name) in names {
        if wanted == number.to_string() {
            return Ok(Value::Text(name.into()));
        }
    }
    // The handle of the machine of a menu is the one that the window of
    // the script holds: the `MenuGetName(GetMenu(hwnd))` of the Quick
    // A script of the v1 asks for the name of the bar of the menu of the
    // window.
    let menus = the_menus(rt);
    let machines: Vec<(u64, String)> = menus
        .iter()
        .filter_map(|menu| menu.machine.map(|machine| (machine, menu.name.clone())))
        .collect();
    for (machine, name) in machines {
        if let Ok(real) = rt.desktop().menu_handle(machine) {
            if wanted == real.to_string() {
                return Ok(Value::Text(name.into()));
            }
        }
    }
    Ok(Value::Text("".into()))
}

/// The items of a menu of the script, in the JSON that the desktop wants.
///
/// The desktop takes an array of `[name, checked, enabled, callback]`, and
/// the callback is left out when the item has none.
///
/// The menu that is not there is an empty bar: the tree has no menu bar
/// without a menu, and the bar of nothing is nothing.
pub(crate) fn the_items_of_the_menu(rt: &mut Runtime<'_>, name: &str) -> String {
    let menus = the_menus(rt);
    the_items_of_the_menu_in(menus, name)
}

/// The items of a menu of the script, in the JSON that the desktop wants.
///
/// The JSON of an item is `[name, checked, enabled]`, with the callback
/// when the item has one, and with the items of the menu that the item
/// opens when it opens one.
fn the_items_of_the_menu_in(menus: &[TheMenu], name: &str) -> String {
    let Some(menu) = menus
        .iter()
        .find(|menu| menu.name.eq_ignore_ascii_case(name))
    else {
        return String::from("[]");
    };
    let mut json = String::from("[");
    for (index, item) in menu.items.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }
        json.push('[');
        json.push_str(&the_json_text(&item.name));
        json.push_str(&format!(",{},{}", item.checked, item.enabled));
        if !item.callback.is_empty() || !item.submenu.is_empty() {
            json.push(',');
            json.push_str(&the_json_text(&item.callback));
        }
        if !item.submenu.is_empty() {
            json.push(',');
            json.push_str(&the_items_of_the_menu_in(menus, &item.submenu));
        }
        json.push(']');
    }
    json.push(']');
    json
}

/// The names of the menus that have an item that opens the menu of a name.
///
/// The v1: the bar of the window is the menu of the script, and the items
/// of the submenus are in the bar of the window: a change in a submenu is
/// a change in the bar.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `Menu, PVfile, Add, Item` of a script of the v1 is in the bar of
/// the window of the program, under the item `&File`.
pub(crate) fn the_menus_that_have_the_submenu(rt: &mut Runtime<'_>, name: &str) -> Vec<String> {
    let menus = the_menus(rt);
    let mut names: Vec<String> = Vec::new();
    let mut pending: Vec<String> = alloc::vec![name.to_string()];
    while let Some(wanted) = pending.pop() {
        for menu in menus.iter() {
            let holds = menu
                .items
                .iter()
                .any(|item| item.submenu.eq_ignore_ascii_case(&wanted));
            if !holds {
                continue;
            }
            if names
                .iter()
                .any(|other: &String| other.eq_ignore_ascii_case(&menu.name))
            {
                continue;
            }
            names.push(menu.name.clone());
            pending.push(menu.name.clone());
        }
    }
    names
}

/// The text of the JSON, with the quotes and the escapes of it.
fn the_json_text(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}
