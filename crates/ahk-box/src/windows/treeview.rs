//! The tree view of the machine: the `SysTreeView32` control, the items
//! of it, and the messages of the `TVM` family.
//!
//! The item of the v1 is the handle the machine gives; zero is the root
//! of the tree, and the handle is the number the script holds.

use windows_sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows_sys::Win32::UI::Controls::{
    TVGN_CARET, TVGN_CHILD, TVGN_FIRSTVISIBLE, TVGN_NEXT, TVGN_NEXTVISIBLE, TVGN_PARENT,
    TVGN_PREVIOUS, TVI_FIRST, TVI_LAST, TVI_ROOT, TVI_SORT, TVIF_HANDLE, TVIF_IMAGE, TVIF_STATE,
    TVIF_TEXT, TVINSERTSTRUCTW, TVIS_BOLD, TVIS_EXPANDED, TVIS_STATEIMAGEMASK,
    TVITEMW, TVM_DELETEITEM, TVM_ENSUREVISIBLE, TVM_EXPAND, TVM_GETCOUNT, TVM_GETITEMW,
    TVM_GETNEXTITEM, TVM_INSERTITEMW, TVM_SELECTITEM, TVM_SETITEMW, TVM_SORTCHILDREN,
    TVE_COLLAPSE, TVE_EXPAND,
};
use windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW;

use super::gui::TheControl;
use super::sys::Wide;

const CHECKED: u32 = 0x1000;
const UNCHECKED: u32 = 0x2000;

/// The item as the handle of the machine; zero is the root.
fn the_item(item: i64) -> windows_sys::Win32::UI::Controls::HTREEITEM {
    if item == 0 {
        TVI_ROOT
    } else {
        item as windows_sys::Win32::UI::Controls::HTREEITEM
    }
}

/// The text of an item.
pub fn text(control: &TheControl, item: i64) -> String {
    if item == 0 {
        return String::new();
    }
    let mut buffer = vec![0u16; 32768];
    let mut tv: TVITEMW = unsafe { std::mem::zeroed() };
    tv.mask = TVIF_TEXT | TVIF_HANDLE;
    tv.hItem = the_item(item);
    tv.pszText = buffer.as_mut_ptr();
    tv.cchTextMax = buffer.len() as i32;
    let answer = unsafe {
        SendMessageW(
            control.handle,
            TVM_GETITEMW,
            0,
            &mut tv as *mut TVITEMW as LPARAM,
        )
    };
    if answer == 0 {
        return String::new();
    }
    let length = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..length])
}

/// The number of items.
pub fn count(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, TVM_GETCOUNT, 0, 0) as i64 }
}

/// The item that is chosen.
pub fn selection(control: &TheControl) -> i64 {
    unsafe {
        SendMessageW(control.handle, TVM_GETNEXTITEM, TVGN_CARET as WPARAM, 0) as i64
    }
}

/// A related item: the child, the parent, the next or the previous.
pub fn relation(control: &TheControl, item: i64, relation: &str) -> i64 {
    let flags = match relation {
        "child" => TVGN_CHILD,
        "parent" => TVGN_PARENT,
        "next" => TVGN_NEXT,
        "prev" | "previous" => TVGN_PREVIOUS,
        "nextvisible" => TVGN_NEXTVISIBLE,
        "firstvisible" => TVGN_FIRSTVISIBLE,
        _ => TVGN_NEXT,
    };
    unsafe {
        SendMessageW(
            control.handle,
            TVM_GETNEXTITEM,
            flags as WPARAM,
            the_item(item) as LPARAM,
        ) as i64
    }
}

/// The next item after the one the caller names, in the order of the
/// tree; "checked" asks for the next one that is checked, and "full"
/// walks the items that are not seen as well.
pub fn next(control: &TheControl, item: i64, mode: &str) -> i64 {
    let mode = mode.trim().to_ascii_lowercase();
    let checked = mode == "c" || mode == "checked";
    let flags = if mode == "full" {
        TVGN_NEXT
    } else {
        TVGN_NEXTVISIBLE
    };
    let mut current = the_item(item);
    if item == 0 {
        // The first item of the tree is the first child of the root.
        return unsafe {
            SendMessageW(
                control.handle,
                TVM_GETNEXTITEM,
                TVGN_CHILD as WPARAM,
                TVI_ROOT as LPARAM,
            ) as i64
        };
    }
    loop {
        let found = unsafe {
            SendMessageW(
                control.handle,
                TVM_GETNEXTITEM,
                flags as WPARAM,
                current as LPARAM,
            )
        };
        if found == 0 {
            return 0;
        }
        if checked {
            if the_state_image(control, found as i64) == CHECKED {
                return found as i64;
            }
            current = found;
            continue;
        }
        return found as i64;
    }
}

/// The state image of an item: the check of it.
fn the_state_image(control: &TheControl, item: i64) -> u32 {
    let mut tv: TVITEMW = unsafe { std::mem::zeroed() };
    tv.mask = TVIF_STATE | TVIF_HANDLE;
    tv.hItem = the_item(item);
    tv.stateMask = TVIS_STATEIMAGEMASK;
    unsafe {
        SendMessageW(
            control.handle,
            TVM_GETITEMW,
            0,
            &mut tv as *mut TVITEMW as LPARAM,
        );
    }
    tv.state & TVIS_STATEIMAGEMASK
}

/// The question of the `TV_Get`: the item is expanded, checked or bold.
pub fn get(control: &TheControl, item: i64, attribute: &str) -> bool {
    let mut tv: TVITEMW = unsafe { std::mem::zeroed() };
    tv.mask = TVIF_STATE | TVIF_HANDLE;
    tv.hItem = the_item(item);
    tv.stateMask = TVIS_EXPANDED | TVIS_BOLD | TVIS_STATEIMAGEMASK;
    unsafe {
        SendMessageW(
            control.handle,
            TVM_GETITEMW,
            0,
            &mut tv as *mut TVITEMW as LPARAM,
        );
    }
    match attribute.trim().to_ascii_lowercase().as_str() {
        "expand" | "expanded" => tv.state & TVIS_EXPANDED != 0,
        "check" | "checked" => tv.state & TVIS_STATEIMAGEMASK == CHECKED,
        "bold" => tv.state & TVIS_BOLD != 0,
        _ => false,
    }
}

/// The options of an item.
#[derive(Default)]
struct ItemOptions {
    state: u32,
    state_mask: u32,
    image: Option<i32>,
    select: bool,
    expand: Option<bool>,
    ensure_visible: bool,
    first_visible: bool,
    sort: bool,
    insert_after: Option<&'static str>,
}

fn the_item_options(options: &str) -> ItemOptions {
    let mut out = ItemOptions::default();
    for word in options.split_whitespace() {
        let lower = word.to_ascii_lowercase();
        let (name, on) = match lower.strip_prefix('-') {
            Some(rest) => (rest.to_string(), false),
            None => (lower.clone(), true),
        };
        let plus_name = name.strip_prefix('+').map(str::to_string);
        let name = plus_name.as_deref().unwrap_or(&name);
        match name {
            "check" => {
                out.state_mask |= TVIS_STATEIMAGEMASK;
                out.state |= if on { CHECKED } else { UNCHECKED };
            }
            "bold" => {
                out.state_mask |= TVIS_BOLD;
                if on {
                    out.state |= TVIS_BOLD;
                }
            }
            "expand" => out.expand = Some(on),
            "select" => out.select = on,
            "vis" => out.ensure_visible = on,
            "visfirst" => {
                out.ensure_visible = on;
                out.first_visible = on;
            }
            "sort" => out.sort = on,
            "first" => out.insert_after = Some("first"),
            "last" => out.insert_after = Some("last"),
            _ => {
                if let Some(number) = name.strip_prefix("icon") {
                    if let Ok(number) = number.parse::<i32>() {
                        out.image = Some(number.saturating_sub(1));
                    }
                }
            }
        }
    }
    out
}

/// An item is added under a parent; the answer is its handle.
pub fn add(control: &TheControl, text: &str, parent: i64, options: &str) -> Result<i64, String> {
    let opts = the_item_options(options);
    let wide = Wide::of(text);
    let mut insert: TVINSERTSTRUCTW = unsafe { std::mem::zeroed() };
    insert.hParent = the_item(parent);
    insert.hInsertAfter = match opts.insert_after {
        Some("first") => TVI_FIRST,
        Some("last") => TVI_LAST,
        _ => {
            if opts.sort {
                TVI_SORT
            } else {
                TVI_LAST
            }
        }
    };
    let mut item: TVITEMW = unsafe { std::mem::zeroed() };
    item.mask = TVIF_TEXT;
    item.pszText = wide.as_ptr() as *mut u16;
    if let Some(image) = opts.image {
        item.mask |= TVIF_IMAGE;
        item.iImage = image;
        item.iSelectedImage = image;
    }
    if opts.state_mask != 0 {
        item.mask |= TVIF_STATE;
        item.state = opts.state;
        item.stateMask = opts.state_mask;
    }
    insert.Anonymous.item = item;
    let added = unsafe {
        SendMessageW(
            control.handle,
            TVM_INSERTITEMW,
            0,
            &mut insert as *mut TVINSERTSTRUCTW as LPARAM,
        )
    };
    if added == 0 {
        return Err("the machine did not add the item to the tree view.".to_string());
    }
    the_options_are_applied(control, added as i64, &opts);
    Ok(added as i64)
}

/// The options of an item are applied after it is there.
fn the_options_are_applied(control: &TheControl, item: i64, opts: &ItemOptions) {
    if let Some(expand) = opts.expand {
        unsafe {
            SendMessageW(
                control.handle,
                TVM_EXPAND,
                if expand { TVE_EXPAND } else { TVE_COLLAPSE } as WPARAM,
                the_item(item) as LPARAM,
            );
        }
    }
    if opts.select {
        unsafe {
            SendMessageW(
                control.handle,
                TVM_SELECTITEM,
                TVGN_CARET as WPARAM,
                the_item(item) as LPARAM,
            );
        }
    }
    if opts.first_visible {
        unsafe {
            SendMessageW(
                control.handle,
                TVM_SELECTITEM,
                TVGN_FIRSTVISIBLE as WPARAM,
                the_item(item) as LPARAM,
            );
        }
    }
    if opts.ensure_visible {
        unsafe {
            SendMessageW(
                control.handle,
                TVM_ENSUREVISIBLE,
                0,
                the_item(item) as LPARAM,
            );
        }
    }
    if opts.sort {
        let parent = relation(control, item, "parent");
        unsafe {
            SendMessageW(
                control.handle,
                TVM_SORTCHILDREN,
                0,
                the_item(parent) as LPARAM,
            );
        }
    }
}

/// An item is removed; the item zero removes them all.
pub fn delete(control: &TheControl, item: i64) -> bool {
    let answer = unsafe {
        SendMessageW(
            control.handle,
            TVM_DELETEITEM,
            0,
            the_item(item) as LPARAM,
        )
    };
    answer != 0
}

/// An item is changed: the options of it and the new text.
pub fn modify(control: &TheControl, item: i64, options: &str, new_text: Option<&str>) -> bool {
    if item == 0 {
        // The root of the tree: the options go to the first item, the
        // way the v1 reads the zero.
        let first = relation(control, 0, "child");
        if first == 0 {
            return false;
        }
        return modify(control, first, options, new_text);
    }
    let opts = the_item_options(options);
    if opts.state_mask != 0 || opts.image.is_some() || new_text.is_some() {
        let mut tv: TVITEMW = unsafe { std::mem::zeroed() };
        tv.mask = TVIF_HANDLE;
        tv.hItem = the_item(item);
        if opts.state_mask != 0 {
            tv.mask |= TVIF_STATE;
            tv.state = opts.state;
            tv.stateMask = opts.state_mask;
        }
        if let Some(image) = opts.image {
            tv.mask |= TVIF_IMAGE;
            tv.iImage = image;
            tv.iSelectedImage = image;
        }
        let wide = new_text.map(Wide::of);
        if let Some(wide) = &wide {
            tv.mask |= TVIF_TEXT;
            tv.pszText = wide.as_ptr() as *mut u16;
        }
        unsafe {
            SendMessageW(
                control.handle,
                TVM_SETITEMW,
                0,
                &mut tv as *mut TVITEMW as LPARAM,
            );
        }
    }
    the_options_are_applied(control, item, &opts);
    true
}

/// The handle of the control, for the messages that ask for it.
pub fn the_handle(control: &TheControl) -> HWND {
    control.handle
}