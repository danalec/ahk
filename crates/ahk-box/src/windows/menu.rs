//! The menus of the script.
//!
//! A menu is a `HMENU` of the machine: the `Menu` of the v2 makes one,
//! adds items to it, and shows it as a popup or attaches it to the
//! window of the script as its menu bar. The items carry the callback
//! the script gave, and the machine answers with the number of the item
//! when it is chosen.
//!
//! The menus of the v1 are the same menus: the `Menu, MyMenu, Add, Item`
//! of the v1 makes one, and the `MenuGetHandle` answers the handle of
//! it. The handle is the one that the machine knows, and the dropdown of
//! the bar of a window is the menu of the script: the message that the
//! machine sends when the dropdown opens carries that handle, and the
//! script recognizes the menu by it.

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CheckMenuItem, CreateMenu, CreatePopupMenu, DestroyMenu, EnableMenuItem, GetMenuItemCount,
    GetMenuItemInfoW, GetMenuState, HMENU, InsertMenuItemW, MENUITEMINFOW, RemoveMenu,
    SetMenu, SetMenuItemInfoW, TrackPopupMenu, MF_BYCOMMAND, MF_CHECKED, MF_DISABLED, MF_ENABLED,
    MF_GRAYED, MF_UNCHECKED, MFT_SEPARATOR, MFT_STRING, MIIM_BITMAP, MIIM_FTYPE, MIIM_ID, MIIM_STATE,
    MIIM_STRING, MIIM_SUBMENU, TPM_LEFTALIGN, TPM_RETURNCMD,
};
use windows_sys::Win32::Graphics::Gdi::{DeleteObject, HBITMAP};

use super::sys::Wide;

/// The tracking of a menu of the machine, in the thread that owns the
/// window of the owner.
///
/// The `TrackPopupMenu` of the machine belongs to the thread that owns
/// the window of the owner: a call from another thread fails at once,
/// the answer is zero, and the menu never appears.
///
/// Measured on the machine: the `TrackPopupMenu` of a window of another
/// thread answers zero in twenty microseconds, and the menu is not
/// drawn at all.
///
/// # Safety
///
/// The `owner` has to be a window of the machine that is alive while the
/// menu is shown, and it has to belong to the thread that calls it.
pub unsafe fn the_menu_is_tracked(menu: HMENU, x: i32, y: i32, owner: HWND) -> Option<usize> {
    let chosen = unsafe {
        TrackPopupMenu(
            menu,
            TPM_LEFTALIGN | TPM_RETURNCMD,
            x,
            y,
            0,
            owner,
            std::ptr::null(),
        )
    };
    if chosen == 0 {
        return None;
    }
    Some(chosen as usize)
}

/// An item of a menu: the number the machine knows it by, the name the
/// script gave it, and the callback it runs when it is chosen.
#[derive(Debug)]
pub struct TheItem {
    pub id: usize,
    pub name: String,
    pub callback: String,
    /// The menu of the machine that the item opens, when it opens one.
    ///
    /// The number is the place of the menu in the list of the menus of
    /// the machine.
    pub submenu: Option<usize>,
    /// The item carries a mark.
    ///
    /// The machine keeps the mark in the menu, and the tree keeps it in
    /// the item: the menu of the script is made again when the machine
    /// takes it away, and the mark of the item goes with it.
    pub checked: bool,
    /// The item can be chosen.
    pub enabled: bool,
    /// The image of the item, when the script gave it one.
    ///
    /// The v1: "Menu, MenuName, Icon, MenuItemName, FileName, IconNumber"
    /// (the CHM of the v1.1.37.02). The machine draws the image at the
    /// left of the item, and the image goes with the item when the menu is
    /// made again.
    pub icon: Option<HBITMAP>,
}

/// A menu of the script.
pub struct TheMenu {
    /// The popup of the script.
    ///
    /// The machine shows a menu as a popup only when the menu is a popup
    /// menu: the `TrackPopupMenu` of a window menu answers zero at once,
    /// the menu is drawn as a thin bar, and the tracking of it ends before
    /// the mouse reaches an item.
    ///
    /// The v1 makes its custom menus as popups and turns them into window
    /// menus when the script attaches one to a window; the bar of the
    /// window is the menu of the script itself, and the items of the bar
    /// open the popups of the script.
    pub handle: HMENU,
    /// The handle is the handle of a window menu and not of a popup.
    ///
    /// The bar of a window has to be a window menu, and the machine
    /// refuses a popup menu as the bar of a window.
    pub is_a_bar: bool,
    /// The handle of the machine is the handle of a window menu.
    ///
    /// The kind of the menu of the script changes when the script attaches
    /// it to a window, and the handle of the machine changes with it: the
    /// menu is destroyed and made again as a window menu.
    ///
    /// The kind does not change after that, and neither does the handle:
    /// the bar of the window is not destroyed and made again for every
    /// command of the script, because the menu that is open goes with it.
    pub is_a_window_menu: bool,
    pub items: Vec<TheItem>,
}

impl TheMenu {
    /// A new popup menu.
    ///
    /// The `base` is the first number of the items of the menu: the
    /// numbers of the items are unique in the whole of the script, and
    /// the menu that is shown and the menus that it opens share them.
    /// A menu of the machine.
    ///
    /// The menu is a WINDOW menu and not a popup: the machine refuses a
    /// popup menu as the bar of a window -- the `SetMenu` of one fails
    /// with the error 87, `ERROR_INVALID_PARAMETER` -- and the menu of
    /// the script is attached to the window of the script as its bar.
    ///
    /// Measured on the machine: the `SetMenu` of a popup
    /// menu answers zero and the error 87, and the `SetMenu` of a window
    /// menu answers one. A window menu takes a popup under it, and a
    /// window menu shows as a popup, so the menus of the script are
    /// window menus on both counts.
    ///
    /// Measured on the binary of the v1.1.37.02: the v1
    /// makes its custom menus as popups and turns them into window
    /// menus when the script attaches one to a window -- the handle that
    /// `MenuGetHandle` answers changes at that moment.
    pub fn new() -> Result<Self, String> {
        // The menu of the script is a popup: the machine draws a popup
        // menu as a dropdown and a window menu as a thin bar, and the bar
        // of a window has to be a window menu.
        //
        // The v1 makes its custom menus as popups and turns them into
        // window menus when the script attaches one to a window.
        let handle = unsafe { CreatePopupMenu() };
        if handle.is_null() {
            return Err("the machine did not make the menu.".to_string());
        }
        Ok(TheMenu {
            handle,
            is_a_bar: false,
            is_a_window_menu: false,
            items: Vec::new(),
        })
    }

    /// The menu of the script becomes the bar of a window.
    ///
    /// The bar of a window has to be a window menu, and the machine
    /// refuses a popup menu as the bar of a window. The v1 turns the menu
    /// of the script into a window menu when the script attaches it to a
    /// window, and it keeps the handle of the menu in the window.
    ///
    /// The menu of the machine is not destroyed here: the popup goes when
    /// the menu is made again as a window menu, and the items go with
    /// `RemoveMenu`, which holds the menus that they open alive.
    ///
    /// The machine takes the menus that the popup opens with it when the
    /// popup is destroyed with its items in it: "This seems to indicate
    /// that submenus are destroyed whenever a parent menu is destroyed"
    /// (the source of the v1, `script_menu.cpp:1329` of v1.1.37.02).
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02: the handle of the
    /// menu that the bar opens does not change when the bar is attached to
    /// the window, and the script holds that handle: the map of the
    /// dropdowns of the script is keyed by it, and the
    /// dropdown is built when it opens.
    ///
    /// The v1: "menu->Create(MENU_TYPE_BAR)" (`script_gui.cpp:458` of
    /// v1.1.37.02).
    pub fn becomes_a_bar(&mut self) {
        self.is_a_bar = true;
    }

    /// The items of the menu of the script go into the menu of the
    /// machine.
    ///
    /// The items that open a menu open the popup of it: the machine
    /// draws a popup menu as a dropdown, and a window menu as a thin bar.
    ///
    /// The `children` are the menus that the items open, by their place
    /// and by the handle they have now.
    ///
    /// The items go in with `InsertMenuItem` and not with `AppendMenu`:
    /// every item carries its own number, and the number is what the
    /// machine answers when the item is chosen.
    ///
    /// The v1: "Although AppendMenu() ignores the ID when adding a
    /// separator and provides no way to specify the ID when adding a
    /// submenu, that is purely a limitation of AppendMenu(). Using
    /// InsertMenuItem() instead allows us to always set the ID, which
    /// simplifies identifying separator and submenu items later on."
    /// (`script_menu.cpp:753-759` of v1.1.37.02)
    ///
    /// Measured on the machine: the `AppendMenu` of a submenu gives the
    /// item a number of its own, and the number is not the one of the
    /// script: the click on the item of a dropdown answered nothing, and
    /// the menu that the item opened could not be found again by its
    /// number.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn the_items_are_appended(
        &self,
        handle: HMENU,
        children: &[(usize, HMENU)],
    ) -> Result<(), String> {
        for item in &self.items {
            let separator = item.name.is_empty();
            let mut wide = Wide::of(&item.name);
            let opened = item.submenu.and_then(|place| {
                children
                    .iter()
                    .find(|(other, _)| *other == place)
                    .map(|(_, handle)| *handle)
            });
            let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
            info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
            info.fMask = MIIM_ID | MIIM_FTYPE | MIIM_STRING;
            info.fType = if separator { MFT_SEPARATOR } else { MFT_STRING };
            info.wID = item.id as u32;
            if !separator {
                info.dwTypeData = wide.as_mut_ptr();
                info.cch = wide.the_number_of_the_letters();
            }
            if item.checked || !item.enabled {
                info.fMask |= MIIM_STATE;
                info.fState = 0;
                if item.checked {
                    info.fState |= MF_CHECKED;
                }
                if !item.enabled {
                    info.fState |= MF_GRAYED | MF_DISABLED;
                }
            }
            if let Some(opened) = opened {
                // The item opens a menu of the script: the menu of the
                // item is the popup of the menu of the script, and the
                // item keeps its own number.
                info.fMask |= MIIM_SUBMENU;
                info.hSubMenu = opened;
            }
            if let Some(icon) = item.icon {
                info.fMask |= MIIM_BITMAP;
                info.hbmpItem = icon;
            }
            // The item goes to the end of the menu.
            let at = unsafe { GetMenuItemCount(handle) }.max(0) as u32;
            let answer = unsafe { InsertMenuItemW(handle, at, 1, &info) };
            if answer == 0 {
                return Err("the machine did not add the item to the menu.".to_string());
            }
        }
        Ok(())
    }

    /// The item of a name, when the menu has it.
    fn the_item(&self, name: &str) -> Option<&TheItem> {
        self.items
            .iter()
            .find(|item| item.name.eq_ignore_ascii_case(name))
    }

    /// The place of the item of a name in the menu.
    ///
    /// The items of a menu are found by their place and not by their
    /// number: the machine takes the number of an item for the handle of
    /// the menu that it opens, and the number of the item is not the
    /// number of the command any more.
    fn the_place_of(&self, name: &str) -> Option<usize> {
        self.items
            .iter()
            .position(|item| item.name.eq_ignore_ascii_case(name))
    }

    /// Whether the menu has an item of a name.
    pub fn has_item(&self, name: &str) -> bool {
        self.the_item(name).is_some()
    }

    /// Add an item to the menu.
    ///
    /// The item with no name is a separator: the v1 adds one when the
    /// script writes the `Menu, MyMenu, Add` with no item.
    ///
    /// The `id` is the number that the machine answers when the item is
    /// chosen: the numbers of the items are unique in the whole of the
    /// script, and the machine keeps them under 65536.
    pub fn add(&mut self, id: usize, name: &str, callback: &str) -> Result<(), String> {
        let mut wide = Wide::of(name);
        let separator = name.is_empty();
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_ID | MIIM_FTYPE | MIIM_STRING;
        info.fType = if separator { MFT_SEPARATOR } else { MFT_STRING };
        info.wID = id as u32;
        if !separator {
            info.dwTypeData = wide.as_mut_ptr();
            info.cch = wide.the_number_of_the_letters();
        }
        // The item goes to the end of the menu, and it carries its own
        // number: the number of a command is the number of the item, and
        // the items are found by it.
        //
        // The v1: "Although AppendMenu() ignores the ID when adding a
        // separator and provides no way to specify the ID when adding a
        // submenu, that is purely a limitation of AppendMenu(). Using
        // InsertMenuItem() instead allows us to always set the ID"
        // (`script_menu.cpp:753-759` of v1.1.37.02).
        let at = self.items.len() as u32;
        let answer = unsafe { InsertMenuItemW(self.handle, at, 1, &info) };
        if answer == 0 {
            return Err("the machine did not add the item to the menu.".to_string());
        }
        self.items.push(TheItem {
            id,
            name: name.to_string(),
            callback: callback.to_string(),
            submenu: None,
            checked: false,
            enabled: true,
            icon: None,
        });
        Ok(())
    }

    /// Add an item to the menu, or update the item of that name when the
    /// menu has one.
    ///
    /// The v1: "If MenuItemName does not yet exist, it will be added to
    /// the menu. Otherwise, MenuItemName is updated with the newly
    /// specified LabelOrSubmenu" (the CHM of the v1.1.37.02, `Menu.htm`).
    pub fn update(&mut self, id: usize, name: &str, callback: &str) -> Result<(), String> {
        // A separator has an empty name and is always appended rather than
        // overwriting an existing separator.
        if name.is_empty() {
            return self.add(id, name, callback);
        }
        let Some(place) = self.the_place_of(name) else {
            return self.add(id, name, callback);
        };
        let id = self.items[place].id;
        let text = self.items[place].name.clone();
        self.items[place].callback = callback.to_string();
        // The item that had a menu of its own is an item again: the v1
        // destroys the menu of the machine when the item is given a
        // label.
        self.items[place].submenu = None;
        let mut wide = Wide::of(&text);
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_ID | MIIM_STRING;
        info.wID = id as u32;
        info.dwTypeData = wide.as_mut_ptr();
        info.cch = wide.the_number_of_the_letters();
        let answer = unsafe { SetMenuItemInfoW(self.handle, id as u32, 0, &info) };
        if answer == 0 {
            return Err("the machine did not update the item of the menu.".to_string());
        }
        Ok(())
    }

    /// Insert an item in the menu, above the last one.
    pub fn insert(&mut self, id: usize, name: &str, callback: &str) -> Result<(), String> {
        let wide = Wide::of(name);
        // The item goes above the last one of the menu: the place of it
        // is the number of the items of the menu, less one.
        let at = self.items.len().saturating_sub(1);
        let mut wide = wide;
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_ID | MIIM_STRING | MIIM_FTYPE;
        info.fType = MFT_STRING;
        info.wID = id as u32;
        info.dwTypeData = wide.as_mut_ptr();
        info.cch = wide.the_number_of_the_letters();
        let answer = unsafe { InsertMenuItemW(self.handle, at as u32, 1, &info) };
        if answer == 0 {
            return Err("the machine did not insert the item into the menu.".to_string());
        }
        // The list of the items keeps the order of the machine: the items
        // are found by their place.
        self.items.insert(at, TheItem {
            id,
            name: name.to_string(),
            callback: callback.to_string(),
            submenu: None,
            checked: false,
            enabled: true,
            icon: None,
        });
        Ok(())
    }

    /// Delete the item of a name.
    ///
    /// The number of the item is free after this: the next item of the
    /// machine takes it.
    pub fn delete(&mut self, name: &str) -> Result<Vec<usize>, String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        // The item goes with `RemoveMenu` and not with `DeleteMenu`: the
        // machine takes the menu of an item away with `DeleteMenu`, and
        // the menus of the script live as long as the script does.
        //
        // The v1: "RemoveMenu(mMenu, aMenuItem_ID, aMenuItem_MF_BY);
        // v1.0.48: Lexikos: DeleteMenu() destroys any sub-menu handle
        // associated with the item, so use RemoveMenu. Otherwise the
        // submenu handle stored somewhere else in memory would suddenly
        // become invalid." (`script_menu.cpp:790` of v1.1.37.02).
        unsafe { RemoveMenu(self.handle, id as u32, MF_BYCOMMAND) };
        let removed = self.items.remove(place);
        if let Some(icon) = removed.icon {
            unsafe { DeleteObject(icon) };
        }
        Ok(vec![id])
    }

    /// Delete every item of the menu.
    ///
    /// The v1: "Unlike a menu entirely deleted by the Delete sub-command,
    /// an empty menu still exists and thus any other menus that use it as
    /// a submenu will still work" (the CHM of the v1.1.37.02, `Menu.htm`).
    ///
    /// The handle of the menu stays: the bar of a window holds it, and
    /// the dropdown of the bar is the menu of the machine.
    pub fn delete_all(&mut self) -> Result<Vec<usize>, String> {
        let mut free = Vec::new();
        while let Some(item) = self.items.first() {
            let id = item.id;
            // The items go with `RemoveMenu`: the machine takes the menu
            // of an item away with `DeleteMenu`, and the menus of the
            // script live as long as the script does.
            unsafe { RemoveMenu(self.handle, id as u32, MF_BYCOMMAND) };
            let removed = self.items.remove(0);
            if let Some(icon) = removed.icon {
                unsafe { DeleteObject(icon) };
            }
            free.push(id);
        }
        Ok(free)
    }

    /// Rename the item of a name.
    pub fn rename(&mut self, name: &str, new: &str) -> Result<(), String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        let mut wide = Wide::of(new);
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_ID | MIIM_STRING;
        info.wID = id as u32;
        info.dwTypeData = wide.as_mut_ptr();
        info.cch = wide.the_number_of_the_letters();
        let answer = unsafe { SetMenuItemInfoW(self.handle, id as u32, 0, &info) };
        if answer == 0 {
            return Err("the machine did not rename the item.".to_string());
        }
        self.items[place].name = new.to_string();
        Ok(())
    }

    /// Enable or disable the item of a name.
    pub fn enable(&mut self, name: &str, on: bool) -> Result<(), String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        let flags = if on { MF_ENABLED } else { MF_GRAYED };
        unsafe { EnableMenuItem(self.handle, id as u32, MF_BYCOMMAND | flags) };
        self.items[place].enabled = on;
        Ok(())
    }

    /// Check or uncheck the item of a name.
    pub fn check(&mut self, name: &str, on: bool) -> Result<(), String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        let flags = if on { MF_CHECKED } else { MF_UNCHECKED };
        unsafe { CheckMenuItem(self.handle, id as u32, MF_BYCOMMAND | flags) };
        self.items[place].checked = on;
        Ok(())
    }

    /// The number of items of the menu.
    pub fn count(&self) -> i64 {
        let answer = unsafe { GetMenuItemCount(self.handle) };
        if answer < 0 {
            self.items.len() as i64
        } else {
            answer as i64
        }
    }

    /// Whether the menu of the machine is dead.
    ///
    /// The machine takes the menu of an item away when the item is
    /// deleted, and the menus that the items of a menu opened go with
    /// it. The machine answers -1 for a menu that is not there.
    pub fn the_menu_is_dead(menu: &TheMenu) -> bool {
        if menu.handle.is_null() {
            return true;
        }
        let answer = unsafe { GetMenuItemCount(menu.handle) };
        answer < 0
    }

    /// Whether the item of a name is enabled.
    pub fn is_enabled(&self, name: &str) -> Result<bool, String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        let state = unsafe { GetMenuState(self.handle, id as u32, MF_BYCOMMAND) };
        Ok(state & MF_DISABLED == 0 && state & MF_GRAYED == 0)
    }

    /// Whether the item of a name is checked.
    pub fn is_checked(&self, name: &str) -> Result<bool, String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        let state = unsafe { GetMenuState(self.handle, id as u32, MF_BYCOMMAND) };
        Ok(state & MF_CHECKED != 0)
    }

    /// The item of the menu that opens the menu of the given place.
    ///
    /// The v1: "To have MenuItemName become a submenu -- which is a menu
    /// item that opens a new menu when selected -- specify for
    /// LabelOrSubmenu a colon followed by the MenuName of an existing
    /// custom menu" (the CHM of the v1.1.37.02, `Menu.htm`).
    pub fn set_submenu(
        &mut self,
        name: &str,
        submenu: usize,
        handle: HMENU,
    ) -> Result<(), String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        // The item becomes a menu that opens: the menu of the item is
        // the popup of the menu of the script, and the item keeps its
        // own number.
        //
        // The v1: "mii.fMask = MIIM_SUBMENU; mii.hSubMenu =
        // aSubmenu ? aSubmenu->mMenu : NULL; if (SetMenuItemInfo(mMenu,
        // aMenuItem->mMenuID, FALSE, &mii))" (`script_menu.cpp:868-870`
        // of v1.1.37.02).
        //
        // The machine takes the menu that the item opened away when the
        // item opens another one -- the v1 knows it and makes the menu
        // again (`script_menu.cpp:871-880`).
        //
        // The menu that the item opens is the popup of the menu of the
        // script: the machine draws a popup menu as a dropdown, and a
        // window menu as a thin bar.
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_SUBMENU;
        info.hSubMenu = handle;
        let answer = unsafe { SetMenuItemInfoW(self.handle, id as u32, 0, &info) };
        if answer == 0 {
            return Err("the machine did not open the menu of the item.".to_string());
        }
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.submenu = Some(submenu);
            item.callback = String::new();
        }
        Ok(())
    }

    /// The icon of the item of a name.
    ///
    /// The v1: "Menu, MenuName, Icon, MenuItemName, FileName, IconNumber,
    /// IconWidth" (the CHM of the v1.1.37.02, `Menu.htm`).
    ///
    /// The item of a menu of the machine carries a bitmap and not an
    /// icon: the machine draws the bitmap of the item.
    ///
    /// The v1: "If FileName is omitted, any custom icon already associated
    /// with the menu item will be removed" (the CHM of the v1.1.37.02,
    /// `Menu.htm`). The bitmap of nothing takes the image away from the
    /// item.
    ///
    /// Measured on the binary of the v1.1.37.02: the
    /// `Menu, MyMenu, Icon, Item` of a menu whose item carries an icon
    /// takes the icon away from it.
    /// The image of an item of the menu is a bitmap of the machine, and the
/// machine owns it: the pointer is given to the machine and is not
/// dereferenced here.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn set_icon(&mut self, name: &str, bitmap: HBITMAP) -> Result<(), String> {
        let Some(place) = self.the_place_of(name) else {
            return Err(format!("the menu has no item named {name:?}."));
        };
        let id = self.items[place].id;
        if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
            eprintln!(
                "the icon of the item {name:?} of the menu 0x{:X} is {bitmap:?}: the id is {id}",
                self.handle as usize
            );
        }
        if bitmap.is_null() {
            // The image goes away from the item: the machine does not take
            // it away with `SetMenuItemInfo`, and the item goes out of the
            // menu and comes back without it.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02: the
            // `Menu, MyMenu, Icon, Item` of a menu whose item carries an
            // icon takes the icon away from it.
            let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
            info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
            info.fMask = MIIM_ID | MIIM_FTYPE | MIIM_STATE | MIIM_SUBMENU;
            let read = unsafe { GetMenuItemInfoW(self.handle, id as u32, 0, &mut info) };
            if read == 0 {
                return Err("the machine did not give the item of the menu.".to_string());
            }
            // The menu that the item opens, and the state of it, are read
            // before the item goes out: the machine takes them with it.
            let submenu = info.hSubMenu;
            let state = info.fState;
            unsafe { RemoveMenu(self.handle, id as u32, MF_BYCOMMAND) };
            let mut wide = Wide::of(&self.items[place].name);
            let mut back: MENUITEMINFOW = unsafe { std::mem::zeroed() };
            back.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
            back.fMask = MIIM_ID | MIIM_FTYPE | MIIM_STRING | MIIM_STATE;
            back.fType = MFT_STRING;
            back.fState = state;
            back.wID = id as u32;
            back.dwTypeData = wide.as_mut_ptr();
            back.cch = wide.the_number_of_the_letters();
            if !submenu.is_null() {
                back.fMask |= MIIM_SUBMENU;
                back.hSubMenu = submenu;
            }
            let answer = unsafe { InsertMenuItemW(self.handle, place as u32, 1, &back) };
            if answer == 0 {
                return Err("the machine did not put the item back into the menu.".to_string());
            }
            if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
                if let Some(old) = item.icon.take() {
                    unsafe { DeleteObject(old) };
                }
            }
            return Ok(());
        }
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_BITMAP;
        info.hbmpItem = bitmap;
        let answer = unsafe { SetMenuItemInfoW(self.handle, id as u32, 0, &info) };
        if answer == 0 {
            unsafe { DeleteObject(bitmap) };
            return Err("the machine did not put the icon on the item.".to_string());
        }
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            if let Some(old) = item.icon.replace(bitmap) {
                unsafe { DeleteObject(old) };
            }
        }
        Ok(())
    }

    /// The items that open the menu of the place attach it again.
    ///
    /// The handle of the machine of a menu that is made again is a new
    /// one, and the items that opened the old one have to open the new: a
    /// dropdown that the bar aponta with a dead handle opens nothing.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02: the v1 keeps the
    /// menu in the model of the script and reapplies the link whenever the
    /// menu is made again, and the dropdown of the bar opens always.
    pub fn the_submenu_is_attached_again(&mut self, place: usize, handle: HMENU) {
        let ids: Vec<usize> = self
            .items
            .iter()
            .filter(|item| item.submenu == Some(place))
            .map(|item| item.id)
            .collect();
        for id in ids {
            let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
            info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
            info.fMask = MIIM_SUBMENU;
            info.hSubMenu = handle;
            unsafe { SetMenuItemInfoW(self.handle, id as u32, 0, &info) };
        }
    }

    /// The callback of the item the machine chose.
    pub fn the_callback_of(&self, id: usize) -> Option<&str> {
        self.items
            .iter()
            .find(|item| item.id == id)
            .map(|item| item.callback.as_str())
    }

    /// Show the menu as a popup at a place, and answer the number of the
    /// item that was chosen.
    ///
    /// The `TrackPopupMenu` with the `TPM_RETURNCMD` answers the number
    /// of the item instead of posting a message: the menu is shown and
    /// the choice comes back at once.
    ///
    /// # Safety
    ///
    /// The `window` has to be a window of the machine that is alive
    /// while the menu is shown, and it has to belong to the thread that
    /// calls it.
    pub unsafe fn show(&self, window: HWND, x: i32, y: i32) -> Result<Option<usize>, String> {
        Ok(unsafe { the_menu_is_tracked(self.handle, x, y, window) })
    }

    /// Attach the menu to a window as its menu bar.
    ///
    /// # Safety
    ///
    /// The `window` has to be a window of the machine that is alive
    /// while the menu is attached to it.
    pub unsafe fn attach(&self, window: HWND) -> Result<(), String> {
        let answer = unsafe { SetMenu(window, self.handle) };
        if answer == 0 {
            return Err("the machine did not attach the menu to the window.".to_string());
        }
        Ok(())
    }

    /// Whether the menu of the machine has to be made again.
    ///
    /// It has to be when it is dead, or when the kind of it changes: the
    /// kind changes when the script attaches the menu to a window, and the
    /// handle of the machine changes with it.
    pub fn the_machine_makes_it_again(&self) -> bool {
        self.handle.is_null()
            || Self::the_menu_is_dead(self)
            || self.is_a_window_menu != self.is_a_bar
    }

    /// The menu of the machine is made again, with the items it had.
    ///
    /// The machine destroys the menu of an item when the item is deleted,
    /// and the items of a menu that the script empties are deleted with
    /// it. The menus of the script live as long as the script does, and
    /// the v1 makes them again when they are needed.
    ///
    /// The `children` are the menus that the items open, by their place
    /// and by the handle they have now.
    ///
    /// Measured on the binary of the v1.1.37.02: the
    /// `Menu, MyBar, DeleteAll` of a bar that opens the menu `MyMenu`
    /// leaves `MyMenu` alive, with its items.
    ///
    /// The items go with `RemoveMenu` and not with `DeleteMenu`: the
    /// machine takes the menu of an item away when the item is deleted
    /// with `DeleteMenu`, and the menus of the script live as long as the
    /// script does -- and the script holds their handle.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02: the handle of the
    /// menu that the bar opens does not change when the bar is made again
    /// as a window menu, and the tree changed it: the map of the
    /// dropdowns of the script is keyed by that handle, and
    /// the dropdown was never built.
    pub fn rebuild(&mut self, children: &[(usize, HMENU)]) -> Result<(), String> {
        // The menu of the machine is made again only when it has to be:
        // when it is dead, or when the kind of it changes. The kind
        // changes when the script attaches the menu to a window, and the
        // handle of the machine changes with it.
        //
        // The bar of the window is NOT destroyed and made again for every
        // command of the script: the menu that is open goes with it, and
        // the dropdown of the script is built while it is
        // open.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02: the handle that
        // `MenuGetHandle` answers changes when the menu of the script is
        // attached to a window, and it does not change after that.
        if !self.the_machine_makes_it_again() {
            // The menu of the machine is alive and it is the kind that the
            // script wants: its items are already in it, and the script
            // adds them, changes them and takes them away in place.
            return Ok(());
        }
        let items = std::mem::take(&mut self.items);
        // The menu keeps the kind it had: the bar of a window is a window
        // menu, and the menu that the script shows is a popup.
        let handle = if self.is_a_bar {
            unsafe { CreateMenu() }
        } else {
            unsafe { CreatePopupMenu() }
        };
        if handle.is_null() {
            self.items = items;
            return Err("the machine did not make the menu.".to_string());
        }
        // The items go with `RemoveMenu`: the machine takes the menus that
        // they open away with them when they are deleted with `DeleteMenu`,
        // and the menus of the script live as long as the script does.
        for item in &items {
            unsafe { RemoveMenu(self.handle, item.id as u32, MF_BYCOMMAND) };
        }
        // The old menu of the machine goes: it has no items in it, and the
        // menus that its items opened are still alive.
        if !self.handle.is_null() {
            unsafe { DestroyMenu(self.handle) };
        }
        self.handle = handle;
        self.is_a_window_menu = self.is_a_bar;
        for mut item in items {
            if item.submenu.is_some() {
                item.callback = String::new();
            }
            self.items.push(item);
        }
        self.the_items_are_appended(self.handle, children)
    }

    /// The menu of the machine is destroyed, and the numbers of its items
    /// are free.
    ///
    /// The items that open a menu are detached before the menu goes: the
    /// machine destroys the menus that are attached to a menu when it
    /// destroys it, and the menus of the script live as long as the
    /// script does.
    ///
    /// Measured on the binary of the v1.1.37.02: the
    /// `Menu, MyBar, Delete` of a bar that opens the menu `MyMenu` leaves
    /// `MyMenu` alive, with its items.
    pub fn destroy(&mut self) -> Vec<usize> {
        let free: Vec<usize> = self.items.iter().map(|item| item.id).collect();
        // The items go with `RemoveMenu`: the machine takes the menus that
        // the items opened away with the menu when the items are deleted
        // with `DeleteMenu`, and the menus of the script live as long as
        // the script does.
        for item in &self.items {
            unsafe { RemoveMenu(self.handle, item.id as u32, MF_BYCOMMAND) };
        }
        // The machine takes the menus that the items opened away with
        // the menu: "This seems to indicate that submenus are destroyed
        // whenever a parent menu is destroyed" (the source of the v1,
        // `script_menu.cpp:1329-1332` of v1.1.37.02). The menus of the
        // script live as long as the script does, and the caller makes
        // them again.
        if !self.handle.is_null() {
            unsafe { DestroyMenu(self.handle) };
            self.handle = std::ptr::null_mut();
        }
        // The handle of the machine is gone: the menu that is made again
        // is a new one, of the kind that the script wants.
        self.is_a_window_menu = false;

        for item in self.items.drain(..) {
            if let Some(icon) = item.icon {
                unsafe { DeleteObject(icon) };
            }
        }
        free
    }
}

impl Drop for TheMenu {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { DestroyMenu(self.handle) };
            self.handle = std::ptr::null_mut();
        }
        for item in self.items.drain(..) {
            if let Some(icon) = item.icon {
                unsafe { DeleteObject(icon) };
            }
        }
    }
}

impl core::fmt::Debug for TheMenu {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TheMenu")
            .field("items", &self.items.len())
            .finish()
    }
}

/// The menu bar of the window of the script.
///
/// The `Gui, Menu, MenuName` of the v1 attaches a menu as the bar of the
/// window: the items are the ones of the menu of the script, and the
/// click on one runs the callback the item carries.
pub struct TheMenuBar {
    pub handle: HMENU,
    /// The number of each item and the callback it runs.
    pub items: Vec<(usize, String)>,
    /// The menus that the items of the bar open.
    ///
    /// The v1: an item whose callback begins with a colon opens the menu
    /// that is behind the colon.
    ///
    /// The menus that the bar opens belong to the script, and the bar
    /// does not own them: the bar is destroyed when the window takes
    /// another one, and the menus of the script live as long as the
    /// script does.
    pub popups: Vec<HMENU>,
    /// Whether the bar owns the menus that it opens.
    ///
    /// The bar that is built from the JSON of the script owns them: it
    /// made them itself.
    owns_the_popups: bool,
    /// Whether the bar owns the handle of the menu.
    ///
    /// The bar that is built from the JSON of the script owns it: it made
    /// it itself. The bar that is the menu of the script does not own it:
    /// the menu of the script lives as long as the script does, and the
    /// bar is only a window onto it.
    owns_the_handle: bool,
}

impl TheMenuBar {
    /// Make the bar out of the items the v1 wrote.
    ///
    /// The items are the JSON of the v1: a list of
    /// `[name, checked, enabled, callback]`, the callback left out when
    /// the item has none.
    pub fn of_the_items(json: &str) -> Result<Self, String> {
        let items = the_items_of_the_json(json)?;
        let handle = unsafe { CreateMenu() };
        if handle.is_null() {
            return Err("the machine did not make the menu bar.".to_string());
        }
        let mut bar = TheMenuBar {
            handle,
            items: Vec::new(),
            popups: Vec::new(),
            owns_the_popups: true,
            owns_the_handle: true,
        };
        let mut next_id = 1usize;
        the_items_are_added(&mut bar, handle, items, &mut next_id)?;
        Ok(bar)
    }

    /// Make the bar out of a menu of the machine.
    ///
    /// The bar of the window of the v1 is the menu of the script itself:
    /// the v1 turns the menu of the script into a window menu when the
    /// script attaches it to a window, and the window keeps the handle of
    /// the menu.
    ///
    /// Measured on the machine: the `TrackPopupMenu` of a window menu
    /// answers zero at once and the menu is drawn as a thin bar, and the
    /// dropdown of a bar whose items open window menus opens nothing.
    ///
    /// The bar does not own the handle of the menu: the menu of the
    /// script lives as long as the script does, and the bar is only a
    /// window onto it. The bar does not own the menus that it opens
    /// either: they are the menus of the script, and the machine takes
    /// them away with the menu that holds them.
    ///
    /// The `items` are the callbacks of every item of the bar and of the
    /// menus that it opens: the click on an item of a dropdown is a
    /// command with the number of the item, and the procedure of the
    /// window looks the number up in the table of the callbacks.
    pub fn of_the_menu(menu: &TheMenu, items: Vec<(usize, String)>) -> Self {
        TheMenuBar {
            handle: menu.handle,
            items,
            popups: Vec::new(),
            owns_the_popups: false,
            owns_the_handle: false,
        }
    }
}

impl Drop for TheMenuBar {
    fn drop(&mut self) {
        // The menus that the items open go before the bar: the bar holds
        // them. The menus that the bar did not make are the menus of the
        // script, and they live as long as the script does.
        if self.owns_the_popups {
            for popup in self.popups.drain(..) {
                unsafe { DestroyMenu(popup) };
            }
        }
        // The bar that is the menu of the script does not own the handle:
        // the machine destroys the menus that are attached to a menu when
        // it destroys it, and the menus of the script are attached to it.
        if self.owns_the_handle {
            unsafe { DestroyMenu(self.handle) };
        }
    }
}

impl core::fmt::Debug for TheMenuBar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TheMenuBar")
            .field("items", &self.items.len())
            .finish()
    }
}

/// The items of a menu are added to the menu of the machine.
///
/// The v1: an item whose callback begins with a colon opens the menu
/// that is behind the colon, and the item of the bar is the menu itself.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// bar of the menu of the window of a script of the v1 has five items,
/// and each one of them opens a menu that has one item in it.
fn the_items_are_added(
    bar: &mut TheMenuBar,
    menu: HMENU,
    items: TheItems,
    next_id: &mut usize,
) -> Result<(), String> {
    for item in items {
        let TheItemOfTheBar {
            name,
            checked,
            enabled,
            callback,
            submenu,
        } = item;
        let mut wide = Wide::of(&name);
        let id = *next_id;
        *next_id += 1;
        // The item opens a menu of its own: the popup is a menu of the
        // machine, and the item carries the handle of it. The item keeps
        // its own number: the number is what the machine answers when
        // the item is chosen, and the click on an item of a dropdown
        // finds the callback of it by that number.
        let popup = match submenu {
            Some(submenu) => {
                let popup = unsafe { CreatePopupMenu() };
                if popup.is_null() {
                    return Err("the machine did not make the menu of the item.".to_string());
                }
                bar.popups.push(popup);
                the_items_are_added(bar, popup, submenu, next_id)?;
                Some(popup)
            }
            None => None,
        };
        let mut info: MENUITEMINFOW = unsafe { std::mem::zeroed() };
        info.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
        info.fMask = MIIM_ID | MIIM_FTYPE | MIIM_STRING;
        info.fType = MFT_STRING;
        info.wID = id as u32;
        info.dwTypeData = wide.as_mut_ptr();
        info.cch = wide.the_number_of_the_letters();
        if checked || !enabled {
            info.fMask |= MIIM_STATE;
            info.fState = 0;
            if checked {
                info.fState |= MF_CHECKED;
            }
            if !enabled {
                info.fState |= MF_GRAYED | MF_DISABLED;
            }
        }
        if let Some(popup) = popup {
            info.fMask |= MIIM_SUBMENU;
            info.hSubMenu = popup;
        }
        let at = unsafe { GetMenuItemCount(menu) }.max(0) as u32;
        let answer = unsafe { InsertMenuItemW(menu, at, 1, &info) };
        if answer == 0 {
            return Err("the machine did not add the item to the menu bar.".to_string());
        }
        bar.items.push((id, callback));
    }
    Ok(())
}

/// The items of the JSON of the runtime: `[name, checked, enabled]`, with
/// the callback when the item has one, and with the items of the menu that
/// the item opens when it opens one.
///
/// The shape is the one the runtime writes, and the parser reads that
/// shape only: a list of lists, the text between quotes with the escapes
/// of the JSON, and the numbers of the flags.
type TheItems = Vec<TheItemOfTheBar>;

/// An item of the bar of the menu, as the runtime wrote it.
struct TheItemOfTheBar {
    /// The name of the item, as the script wrote it.
    name: String,
    /// Whether the item carries a mark.
    checked: bool,
    /// Whether the item can be chosen.
    enabled: bool,
    /// The name of the function of the script that choosing it runs.
    callback: String,
    /// The items of the menu that the item opens.
    ///
    /// The v1: an item whose callback begins with a colon opens the menu
    /// that is behind the colon. The menu is there even when it has no
    /// item yet: the item opens it all the same.
    submenu: Option<TheItems>,
}

fn the_items_of_the_json(json: &str) -> Result<TheItems, String> {
    let chars: Vec<char> = json.chars().collect();
    let (items, _) = the_items_of_the_chars(&chars, 0)?;
    Ok(items)
}

/// The items of the JSON, read from the place of the text.
fn the_items_of_the_chars(chars: &[char], mut at: usize) -> Result<(TheItems, usize), String> {
    let mut items = Vec::new();
    let skip = |at: &mut usize| {
        while *at < chars.len() && chars[*at].is_whitespace() {
            *at += 1;
        }
    };
    skip(&mut at);
    // An empty object is no bar: the caller that clears the bar writes
    // `{}`, and the answer is no items.
    if at < chars.len() && chars[at] == '{' {
        return Ok((items, at));
    }
    if at >= chars.len() || chars[at] != '[' {
        return Err("the items of the menu bar are not a list.".to_string());
    }
    at += 1;
    skip(&mut at);
    if at < chars.len() && chars[at] == ']' {
        return Ok((items, at + 1));
    }
    loop {
        skip(&mut at);
        if at >= chars.len() || chars[at] != '[' {
            return Err("an item of the menu bar is not a list.".to_string());
        }
        at += 1;
        skip(&mut at);
        let name = the_text_of_the_json(chars, &mut at)?;
        skip(&mut at);
        if at >= chars.len() || chars[at] != ',' {
            return Err("an item of the menu bar has no flags.".to_string());
        }
        at += 1;
        let checked = the_flag_of_the_json(chars, &mut at)?;
        skip(&mut at);
        if at >= chars.len() || chars[at] != ',' {
            return Err("an item of the menu bar has no enabled flag.".to_string());
        }
        at += 1;
        let enabled = the_flag_of_the_json(chars, &mut at)?;
        skip(&mut at);
        let mut callback = String::new();
        let mut submenu: Option<TheItems> = None;
        if at < chars.len() && chars[at] == ',' {
            at += 1;
            skip(&mut at);
            // The callback of the item, or the items of the menu that it
            // opens: the item of a bar that opens a menu has no callback.
            if at < chars.len() && chars[at] == '"' {
                callback = the_text_of_the_json(chars, &mut at)?;
                skip(&mut at);
                if at < chars.len() && chars[at] == ',' {
                    at += 1;
                    skip(&mut at);
                    let (items, place) = the_items_of_the_chars(chars, at)?;
                    submenu = Some(items);
                    at = place;
                    skip(&mut at);
                }
            } else {
                let (items, place) = the_items_of_the_chars(chars, at)?;
                submenu = Some(items);
                at = place;
                skip(&mut at);
            }
        }
        if at >= chars.len() || chars[at] != ']' {
            return Err("an item of the menu bar does not end.".to_string());
        }
        at += 1;
        items.push(TheItemOfTheBar {
            name,
            checked,
            enabled,
            callback,
            submenu,
        });
        skip(&mut at);
        if at >= chars.len() {
            return Err("the list of the menu bar does not end.".to_string());
        }
        match chars[at] {
            ',' => {
                at += 1;
            }
            ']' => {
                // The place of the text is the place after the list.
                at += 1;
                break;
            }
            _ => return Err("the list of the menu bar is broken.".to_string()),
        }
    }
    Ok((items, at))
}

/// A text of the JSON, between quotes, with the escapes of it.
fn the_text_of_the_json(chars: &[char], at: &mut usize) -> Result<String, String> {
    if *at >= chars.len() || chars[*at] != '"' {
        return Err("a text of the menu bar is not between quotes.".to_string());
    }
    *at += 1;
    let mut out = String::new();
    while *at < chars.len() {
        let letter = chars[*at];
        *at += 1;
        match letter {
            '"' => return Ok(out),
            '\\' => {
                if *at >= chars.len() {
                    break;
                }
                let escaped = chars[*at];
                *at += 1;
                match escaped {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    other => out.push(other),
                }
            }
            other => out.push(other),
        }
    }
    Err("a text of the menu bar does not end.".to_string())
}

/// A flag of the JSON: the v1 writes `true`/`false`, and a number is
/// read as well -- zero is off, anything else is on.
fn the_flag_of_the_json(chars: &[char], at: &mut usize) -> Result<bool, String> {
    while *at < chars.len() && chars[*at].is_whitespace() {
        *at += 1;
    }
    let start = *at;
    while *at < chars.len()
        && (chars[*at].is_ascii_alphanumeric() || chars[*at] == '-' || chars[*at] == '.')
    {
        *at += 1;
    }
    let text: String = chars[start..*at].iter().collect();
    match text.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => text
            .parse::<f64>()
            .map(|number| number != 0.0)
            .map_err(|_| "a flag of the menu bar is not a number.".to_string()),
    }
}

