//! The menus of the machine, against the machine itself.
//!
//! The menu of the script is a menu of the machine, and the bar of the
//! window of the script is that menu. The cases of this file are the
//! things that the machine does with the menus, and every one of them was
//! measured on the machine and on the binary of the v1.1.37.02.

#![cfg(windows)]

use ahk_box::windows::gui;
use ahk_box::windows::menu::TheMenu;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateMenu, CreatePopupMenu, GetMenuItemCount, SetMenu, HMENU,
};

/// The number of items of a menu of the machine, and -1 when the menu is
/// not there any more.
fn the_count(menu: HMENU) -> i32 {
    unsafe { GetMenuItemCount(menu) }
}

#[test]
fn the_window_of_the_script_takes_a_window_menu() {
    // The bar of a window is a window menu: the machine refuses a popup
    // menu as the bar of a window, and the menus of the script are popups.
    //
    // Measured on the machine: the `SetMenu` of a popup
    // menu answers zero and the error 87, `ERROR_INVALID_PARAMETER`, and
    // the `SetMenu` of a window menu answers one.
    let window = gui::open_of_the_v1("the probe").expect("the window is made");
    let menu: HMENU = unsafe { CreateMenu() };
    assert!(!menu.is_null());
    let answer = unsafe { SetMenu(window.handle, menu) };
    assert_ne!(answer, 0, "the machine did not take the menu of the window");
}

#[test]
fn the_menu_of_the_script_is_a_popup() {
    // The machine shows a menu as a popup only when the menu is a popup
    // menu: the `TrackPopupMenu` of a window menu answers zero at once,
    // the menu is drawn as a thin bar, and the tracking of it ends before
    // the mouse reaches an item.
    //
    // The v1 makes its custom menus as popups and turns them into window
    // menus when the script attaches one to a window.
    let menu = TheMenu::new().expect("the menu is made");
    let popup: HMENU = unsafe { CreatePopupMenu() };
    assert!(!popup.is_null());
    let answer = unsafe { SetMenu(gui::open_of_the_v1("the probe").expect("the window is made").handle, menu.handle) };
    assert_eq!(answer, 0, "the machine took the popup of the script as the bar of a window");
}

#[test]
fn the_machine_refuses_a_popup_menu_as_the_bar_of_a_window() {
    let window = gui::open_of_the_v1("the probe").expect("the window is made");
    let popup: HMENU = unsafe { CreatePopupMenu() };
    assert!(!popup.is_null());
    let answer = unsafe { SetMenu(window.handle, popup) };
    assert_eq!(answer, 0, "the machine took a popup menu as the bar of a window");
    assert_eq!(unsafe { GetLastError() }, 87);
}

#[test]
fn the_menu_that_an_item_opens_survives_the_removal_of_the_item() {
    // The machine takes the menu of an item away when the item is
    // deleted with `DeleteMenu`, and the menus of the script live as long
    // as the script does: the item goes with `RemoveMenu`.
    //
    // Measured on the binary of the v1.1.37.02: the
    // `Menu, MyBar, DeleteAll` of a bar that opens the menu `MyMenu`
    // leaves `MyMenu` alive, with its items.
    let mut bar = TheMenu::new().expect("the menu is made");
    let mut opened = TheMenu::new().expect("the menu is made");
    opened.add(1, "Item", "MyFunc").expect("the item is added");
    bar.add(2, "&File", "").expect("the item is added");
    bar.set_submenu("&File", 0, opened.handle)
        .expect("the item opens the menu");
    assert_eq!(the_count(opened.handle), 1);
    bar.delete("&File").expect("the item is deleted");
    assert_eq!(
        the_count(opened.handle),
        1,
        "the machine took the menu of the item away"
    );
}

#[test]
fn the_menu_that_an_item_opens_survives_the_emptying_of_the_menu() {
    let mut bar = TheMenu::new().expect("the menu is made");
    let mut opened = TheMenu::new().expect("the menu is made");
    opened.add(1, "Item", "MyFunc").expect("the item is added");
    bar.add(2, "&File", "").expect("the item is added");
    bar.set_submenu("&File", 0, opened.handle)
        .expect("the item opens the menu");
    bar.delete_all().expect("the menu is emptied");
    assert_eq!(the_count(opened.handle), 1);
    assert_eq!(the_count(bar.handle), 0);
}

#[test]
fn the_items_of_a_menu_carry_their_own_number() {
    // The number of an item is the number of the command that the machine
    // sends when the item is chosen, and the number of a command is the
    // low word of it: the numbers of the items of the machine stay under
    // 65536.
    let mut menu = TheMenu::new().expect("the menu is made");
    menu.add(4096, "First", "MyFunc").expect("the item is added");
    menu.add(4097, "Second", "MyFunc").expect("the item is added");
    assert_eq!(menu.the_callback_of(4096), Some("MyFunc"));
    assert_eq!(menu.the_callback_of(4097), Some("MyFunc"));
    assert_eq!(menu.count(), 2);
}

#[test]
fn the_popup_of_a_menu_carries_its_items() {
    // The machine shows a menu as a popup only when the menu is a popup
    // menu: the menus of the script are window menus, because the bar of
    // a window has to be one, and every menu of the script has a popup
    // that the dropdowns of the bar open.
    //
    // Measured on the machine: the `TrackPopupMenu` of a window menu
    // answers zero at once, and the menu is drawn as a thin bar.
    let mut menu = TheMenu::new().expect("the menu is made");
    menu.add(4096, "First", "MyFunc").expect("the item is added");
    menu.add(4097, "Second", "MyFunc").expect("the item is added");
    assert_eq!(the_count(menu.handle), 2, "the popup does not carry the items");
    assert_eq!(
        the_count(menu.handle),
        2,
        "the window menu does not carry the items"
    );
}

#[test]
fn the_popup_of_a_menu_carries_the_menu_that_the_item_opens() {
    // The item that opens a menu opens the popup of it: the machine
    // draws a popup menu as a dropdown, and a window menu as a thin bar.
    let mut bar = TheMenu::new().expect("the menu is made");
    let mut opened = TheMenu::new().expect("the menu is made");
    opened.add(1, "Item", "MyFunc").expect("the item is added");
    bar.add(2, "&File", "").expect("the item is added");
    bar.set_submenu("&File", 0, opened.handle)
        .expect("the item opens the popup of the menu");
    // The popup of the bar opens the popup of the menu that the item
    // opens, and the window menu of the bar opens it as well.
    let menu = unsafe { windows_sys::Win32::UI::WindowsAndMessaging::GetSubMenu(bar.handle, 0) };
    assert_eq!(
        menu, opened.handle,
        "the item does not open the popup of the menu"
    );
}

#[test]
fn multiple_separators_are_not_collapsed() {
    let mut menu = TheMenu::new().expect("the menu is made");
    menu.update(4096, "First", "A").expect("first");
    menu.update(4097, "", "").expect("sep 1");
    menu.update(4098, "Second", "B").expect("second");
    menu.update(4099, "", "").expect("sep 2");
    menu.update(4100, "Third", "C").expect("third");
    assert_eq!(menu.count(), 5);
}

#[test]
fn the_menu_icon_is_a_32_bit_dib_section() {
    use windows_sys::Win32::Graphics::Gdi::{DIBSECTION, DeleteObject, GetObjectW};
    let icon_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/icon.ico");
    let hbm = ahk_box::windows::pictures::the_bitmap_of_the_icon_of_a_file(
        &icon_path.to_string_lossy(),
        1,
    )
    .expect("menu icon bitmap created");
    let mut ds: DIBSECTION = unsafe { std::mem::zeroed() };
    let size = unsafe {
        GetObjectW(
            hbm,
            std::mem::size_of::<DIBSECTION>() as i32,
            &mut ds as *mut _ as *mut core::ffi::c_void,
        )
    };
    unsafe { DeleteObject(hbm) };
    assert!(size > 0, "GetObjectW should read DIBSECTION");
    assert_eq!(ds.dsBm.bmBitsPixel, 32);
    assert!(!ds.dsBm.bmBits.is_null(), "menu icon must be a DIB section, not a DDB");
}

