//! The lists of images of the machine: the `HIMAGELIST` of `comctl32`,
//! the `IL_Create`, the `IL_Add` and the `IL_Destroy` of the v1.

use windows_sys::Win32::UI::Controls::{
    ImageList_Add, ImageList_Create, ImageList_Destroy, ImageList_ReplaceIcon, ILC_COLOR32,
    ILC_MASK,
};
use windows_sys::Win32::UI::Shell::ExtractIconW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    LoadImageW, IMAGE_BITMAP, IMAGE_ICON, LR_LOADFROMFILE,
};

use super::sys::Wide;

/// A list of images is made; the answer is its handle.
pub fn create(initial: i32, grow: i32, large: bool) -> i64 {
    let size = if large { 32 } else { 16 };
    unsafe {
        ImageList_Create(
            size,
            size,
            ILC_COLOR32 | ILC_MASK,
            initial.max(1),
            grow.max(1),
        ) as i64
    }
}

/// An image is put in a list; the answer is its place, counted from one,
/// and zero when the image could not be loaded.
///
/// The source is the `hBitmap:` or the `hIcon:` of a handle the script
/// holds, or the name of a file: an icon, a library that holds icons, or
/// a bitmap.
pub fn add(list: i64, source: &str, icon_number: i64) -> i64 {
    if list == 0 {
        return 0;
    }
    let source = source.trim();
    if let Some(handle) = source.strip_prefix("hBitmap:") {
        let bitmap = the_handle_of(handle);
        if bitmap == 0 {
            return 0;
        }
        let index = unsafe {
            ImageList_Add(
                list as _,
                bitmap as _,
                std::ptr::null_mut(),
            )
        };
        return if index < 0 { 0 } else { index as i64 + 1 };
    }
    if let Some(handle) = source.strip_prefix("hIcon:") {
        let icon = the_handle_of(handle);
        if icon == 0 {
            return 0;
        }
        let index = unsafe { ImageList_ReplaceIcon(list as _, -1, icon as _) };
        return if index < 0 { 0 } else { index as i64 + 1 };
    }
    let wide = Wide::of(source);
    let lower = source.to_ascii_lowercase();
    let is_icon = lower.ends_with(".ico")
        || lower.ends_with(".exe")
        || lower.ends_with(".dll")
        || lower.ends_with(".icl");
    if is_icon {
        let index = if lower.ends_with(".ico") {
            let icon = unsafe {
                LoadImageW(
                    std::ptr::null_mut(),
                    wide.as_ptr(),
                    IMAGE_ICON,
                    0,
                    0,
                    LR_LOADFROMFILE,
                )
            };
            if icon.is_null() {
                return 0;
            }
            unsafe { ImageList_ReplaceIcon(list as _, -1, icon as _) }
        } else {
            let icon = unsafe {
                ExtractIconW(
                    std::ptr::null_mut(),
                    wide.as_ptr(),
                    icon_number.saturating_sub(1).max(0) as u32,
                )
            };
            if icon.is_null() || icon as isize == 1 {
                return 0;
            }
            unsafe { ImageList_ReplaceIcon(list as _, -1, icon as _) }
        };
        return if index < 0 { 0 } else { index as i64 + 1 };
    }
    // A bitmap of a file: the machine loads the BMP; the other formats
    // are the ones the `LoadPicture` of the script reads.
    let bitmap = unsafe {
        LoadImageW(
            std::ptr::null_mut(),
            wide.as_ptr(),
            IMAGE_BITMAP,
            0,
            0,
            LR_LOADFROMFILE,
        )
    };
    if bitmap.is_null() {
        return 0;
    }
    let index = unsafe { ImageList_Add(list as _, bitmap as _, std::ptr::null_mut()) };
    if index < 0 {
        0
    } else {
        index as i64 + 1
    }
}

/// A list of images is given back to the machine.
pub fn destroy(list: i64) -> bool {
    if list == 0 {
        return false;
    }
    unsafe { ImageList_Destroy(list as _) != 0 }
}

/// The number a text holds: decimal, or hex with the `0x` before it.
fn the_handle_of(text: &str) -> i64 {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).unwrap_or(0)
    } else {
        text.parse::<i64>().unwrap_or(0)
    }
}