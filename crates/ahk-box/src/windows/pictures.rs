//! The pictures of the machine.
//!
//! A picture of the tree is a file of a picture, and the tree reads it
//! with its own codec -- the PNG of `picture`. The Windows has its own
//! way of loading a picture (`LoadImageW`), and the tree uses it for
//! the formats that the codec does not carry: the icon of a file, the
//! bitmap of the machine.
//!
//! The search of a picture on the screen is the search of the tree: the
//! region of the screen is a picture, and the matcher of `pictures` looks
//! for the picture in it.

use ahk::desktop::{Image, ImageOptions, ImageSearch, PictureOptions};

use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateDIBSection, DIB_RGB_COLORS,
};
use windows_sys::Win32::UI::Shell::ExtractIconExW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CopyImage, DestroyIcon, IMAGE_BITMAP, IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE, LoadImageW,
};

use super::sys::Wide;
use crate::pictures::Picture;

/// A picture of a file is loaded, and the tree keeps it.
///
/// The answer is the number that the script holds: the picture of the
/// tree is a number, and the runtime keeps the table of them.
pub fn load_picture(
    pictures: &mut Vec<Picture>,
    filename: &str,
    options: &PictureOptions,
) -> Result<u32, String> {
    let path = std::path::Path::new(filename);
    if !path.exists() {
        return Err(format!("the file of the picture is not there: {filename}"));
    }
    // The codec of the tree reads the PNG and the JPEG; the file of
    // another format is loaded by the machine.
    let picture = match crate::pictures::read(filename) {
        Ok(picture) => picture,
        Err(_) => load_with_the_machine(filename, options)?,
    };
    pictures.push(picture);
    Ok(pictures.len() as u32)
}

/// A picture of a file that the codec of the tree does not read.
///
/// The machine loads it with `LoadImageW`, which is of Windows 2000, and
/// the answer is the bitmap of it.
fn load_with_the_machine(filename: &str, options: &PictureOptions) -> Result<Picture, String> {
    unsafe {
        let handle = LoadImageW(
            std::ptr::null_mut(),
            Wide::of(filename).as_ptr(),
            IMAGE_BITMAP,
            options.width.unwrap_or(0) as i32,
            options.height.unwrap_or(0) as i32,
            LR_LOADFROMFILE | LR_DEFAULTSIZE,
        );
        if handle.is_null() {
            return Err(format!("the machine did not load the picture: {filename}"));
        }
        // The bitmap of the machine is freed here: the picture of the
        // tree is the pixels of it, and not the bitmap.
        let picture = the_pixels_of_the_bitmap(handle);
        windows_sys::Win32::Graphics::Gdi::DeleteObject(handle);
        picture
    }
}

/// The pixels of a bitmap of the machine.
///
/// The bitmap of the machine is a device context with the pixels of it:
/// the tree reads them with `GetDIBits` and turns them into the picture
/// of the tree.
fn the_pixels_of_the_bitmap(
    handle: windows_sys::Win32::Graphics::Gdi::HBITMAP,
) -> Result<Picture, String> {
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, DIB_RGB_COLORS, DeleteDC,
        GetDIBits, GetObjectW, SelectObject,
    };
    unsafe {
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = 0;
        info.bmiHeader.biHeight = 0;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let read = GetObjectW(
            handle,
            std::mem::size_of::<BITMAPINFO>() as i32,
            &mut info as *mut _ as *mut core::ffi::c_void,
        );
        if read == 0 {
            return Err("the machine did not say the size of the bitmap.".to_string());
        }
        let width = info.bmiHeader.biWidth;
        let height = info.bmiHeader.biHeight.abs();
        if width <= 0 || height <= 0 {
            return Err("the bitmap of the machine has no size.".to_string());
        }
        let context = CreateCompatibleDC(std::ptr::null_mut());
        if context.is_null() {
            return Err("the machine gave no context for the bitmap.".to_string());
        }
        let previous = SelectObject(context, handle);
        let mut bits = vec![0u8; (width as usize) * (height as usize) * 4];
        let got = GetDIBits(
            context,
            handle,
            0,
            height as u32,
            bits.as_mut_ptr().cast(),
            &mut info,
            DIB_RGB_COLORS,
        );
        SelectObject(context, previous);
        DeleteDC(context);
        if got == 0 {
            return Err("the machine did not read the pixels of the bitmap.".to_string());
        }
        let mut pixels = Vec::with_capacity(bits.len() / 4 * 3);
        for pixel in bits.chunks_exact(4) {
            pixels.push(pixel[2]);
            pixels.push(pixel[1]);
            pixels.push(pixel[0]);
        }
        Ok(Picture {
            pixels,
            width: width as usize,
            height: height as usize,
        })
    }
}

/// The image of a picture control of the v1.
///
/// The v1 loads the image of a picture control and puts it in it: the
/// style of the control becomes the `SS_BITMAP` of a bitmap or the
/// `SS_ICON` of an icon, and the image goes into it with the
/// `STM_SETIMAGE` (source `script_gui.cpp:6389-6506` of v1.1.37.02).
///
/// The answer is the bitmap -- or the icon -- and the kind of it: the
/// `IMAGE_BITMAP` or the `IMAGE_ICON`.
///
/// Measured on the binary of v1.1.37.02: the picture of
/// a PNG is a control of the style `0x5000000E` -- the `SS_BITMAP` --,
/// the picture of an .ico is of the style `0x50000003` -- the `SS_ICON`
/// --, and the picture of a file that is not there is of the style
/// `0x50000000`: a picture that cannot be loaded has no style of image,
/// and it is not an error.
///
/// The `width` and the `height` are the ones that the script asks for,
/// and the minus one is the size that the image has: the v1 keeps the
/// proportion of the image when one of the two is not given.
pub fn the_image_of_a_picture(
    file: &str,
    icon: Option<u32>,
    width: Option<i64>,
    height: Option<i64>,
) -> Option<(windows_sys::Win32::Graphics::Gdi::HBITMAP, u32)> {
    let text = file.trim();
    // The handle of a bitmap or of an icon that the script already has.
    if let Some(handle) = the_handle_of_an_image(text, "HBITMAP:") {
        return Some((handle, IMAGE_BITMAP));
    }
    if let Some(handle) = the_handle_of_an_image(text, "HICON:") {
        return Some((handle, IMAGE_ICON));
    }
    let path = std::path::Path::new(text);
    if !path.exists() {
        return None;
    }
    let extension = path
        .extension()
        .map(|one| one.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    // The icon of a file of icons: an .ico, an .exe or a .dll.
    if icon.is_some() || extension == "ico" || extension == "exe" || extension == "dll" {
        return the_icon_of_the_file(text, icon, width, height);
    }
    // The image of a file: the codec of the tree reads the PNG and the
    // JPEG, and the machine reads the rest.
    match crate::pictures::read(text) {
        Ok(picture) => {
            let picture = crate::pictures::resized(picture, width, height);
            let bitmap = the_bitmap_of_the_picture(&picture)?;
            Some((bitmap, IMAGE_BITMAP))
        }
        Err(_) => {
            let handle = unsafe {
                LoadImageW(
                    std::ptr::null_mut(),
                    Wide::of(text).as_ptr(),
                    IMAGE_BITMAP,
                    the_size_for_the_machine(width),
                    the_size_for_the_machine(height),
                    LR_LOADFROMFILE | LR_DEFAULTSIZE,
                )
            };
            if handle.is_null() {
                None
            } else {
                Some((handle, IMAGE_BITMAP))
            }
        }
    }
}

/// The handle of a bitmap or of an icon that the script gives.
///
/// The v1: the text of a picture control may be the handle of a bitmap
/// or of an icon, written as "HBITMAP:0x..." or as "HICON:0x...".
fn the_handle_of_an_image(text: &str, prefix: &str) -> Option<windows_sys::Win32::Graphics::Gdi::HBITMAP> {
    let rest = text.strip_prefix(prefix)?;
    let number = rest.trim();
    let number = number
        .strip_prefix("0x")
        .or_else(|| number.strip_prefix("0X"))
        .map(|digits| u64::from_str_radix(digits, 16).ok())
        .unwrap_or_else(|| number.parse::<u64>().ok())?;
    if number == 0 {
        return None;
    }
    Some(number as windows_sys::Win32::Graphics::Gdi::HBITMAP)
}

/// The size that the machine is asked for.
///
/// The minus one is the size that the image has, and the machine keeps
/// the size of it when the size is not asked for.
fn the_size_for_the_machine(size: Option<i64>) -> i32 {
    match size {
        Some(value) if value > 0 => value as i32,
        _ => 0,
    }
}

/// The icon of a file of icons, at the size that the script asks for.
///
/// The v1: the "Icon" option of the `Gui, Add` names the icon of an
/// .ico, of an .exe or of a .dll, and the number of it is the number
/// of the icon inside do file (source `script_gui.cpp:4632-4662` of
/// v1.1.37.02).
fn the_icon_of_the_file(
    file: &str,
    icon: Option<u32>,
    width: Option<i64>,
    height: Option<i64>,
) -> Option<(windows_sys::Win32::Graphics::Gdi::HBITMAP, u32)> {
    let number = icon.unwrap_or(1);
    let extension = std::path::Path::new(file)
        .extension()
        .map(|one| one.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    // For a standalone .ico file with IconNumber == 1, LoadImageW selects the
    // highest-quality matching frame from the .ico directory directly (instead
    // of ExtractIconExW's 32x32 large icon).
    if extension == "ico" && number <= 1 {
        let (req_w, req_h) = match (width, height) {
            (Some(w), Some(-1)) if w > 0 => (w as i32, w as i32),
            (Some(-1), Some(h)) if h > 0 => (h as i32, h as i32),
            (Some(w), Some(h)) if w > 0 && h > 0 => (w as i32, h as i32),
            (Some(w), None) if w > 0 => (w as i32, w as i32),
            (None, Some(h)) if h > 0 => (h as i32, h as i32),
            _ => (0, 0),
        };
        let flags = if req_w == 0 && req_h == 0 {
            LR_LOADFROMFILE | LR_DEFAULTSIZE
        } else {
            LR_LOADFROMFILE
        };
        let loaded = unsafe {
            LoadImageW(
                std::ptr::null_mut(),
                Wide::of(file).as_ptr(),
                IMAGE_ICON,
                req_w,
                req_h,
                flags,
            )
        };
        if !loaded.is_null() {
            return Some((loaded, IMAGE_ICON));
        }
    }
    let mut large = std::ptr::null_mut();
    let mut small = std::ptr::null_mut();
    // The number of the icon of the v1 counts from one, and the number
    // of the machine counts from zero.
    let index = number.saturating_sub(1) as i32;
    let count = unsafe {
        ExtractIconExW(
            Wide::of(file).as_ptr(),
            index,
            &mut large,
            &mut small,
            1,
        )
    };
    if !small.is_null() && small != large {
        unsafe { DestroyIcon(small) };
    }
    if count == 0 || large.is_null() {
        return None;
    }
    // The size of the icon, for the side that the script leaves open.
    let (natural_width, natural_height) = the_size_of_the_icon(large);
    let width = match width {
        Some(-1) => match height {
            Some(height) if height > 0 && natural_height > 0 => {
                natural_width * height / natural_height
            }
            _ => natural_width,
        },
        other => other.unwrap_or(natural_width),
    };
    let height = match height {
        Some(-1) => {
            if width > 0 && natural_width > 0 {
                natural_height * width / natural_width
            } else {
                natural_height
            }
        }
        other => other.unwrap_or(natural_height),
    };
    unsafe {
        if width > 0 && height > 0 {
            // The v1 scales the icon with the `CopyImage` (source
            // `script_gui.cpp:6391-6393` of v1.1.37.02).
            let scaled = CopyImage(large, IMAGE_ICON, width as i32, height as i32, LR_DEFAULTSIZE);
            DestroyIcon(large);
            if scaled.is_null() {
                return None;
            }
            return Some((scaled, IMAGE_ICON));
        }
        Some((large, IMAGE_ICON))
    }
}

/// The bitmap of the icon of a file, for an item of a menu of the
/// machine.
///
/// The item of a menu of the machine carries a 32-bit premultiplied-alpha
/// DIB section (`PARGB`): on Windows Vista and later, themed menus blend
/// `MENUITEMINFOW.hbmpItem` using its per-pixel alpha channel only when the
/// bitmap is a 32-bit DIB section. Returning a device-dependent bitmap from
/// `GetIconInfo` loses alpha transparency and draws solid boxes.
///
/// The v1: "Menu, MenuName, Icon, MenuItemName, FileName, IconNumber,
/// IconWidth" (the CHM of the v1.1.37.02, `Menu.htm`).
pub fn the_bitmap_of_the_icon_of_a_file(
    file: &str,
    number: i64,
) -> Option<windows_sys::Win32::Graphics::Gdi::HBITMAP> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SM_CXSMICON, SM_CYSMICON,
    };
    let text = file.trim();
    let cx = unsafe { GetSystemMetrics(SM_CXSMICON) }.max(16);
    let cy = unsafe { GetSystemMetrics(SM_CYSMICON) }.max(16);

    if let Some(handle) = the_handle_of_an_image(text, "HBITMAP:") {
        let copied = unsafe {
            CopyImage(
                handle,
                IMAGE_BITMAP,
                0,
                0,
                windows_sys::Win32::UI::WindowsAndMessaging::LR_CREATEDIBSECTION,
            )
        };
        return if copied.is_null() { None } else { Some(copied) };
    }
    if let Some(handle) = the_handle_of_an_image(text, "HICON:") {
        return icon_to_menu_bitmap(handle, cx, cy);
    }

    let path = std::path::Path::new(text);
    let extension = path
        .extension()
        .map(|one| one.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    // Non-icon image files (.png, .jpg, .bmp, etc.) loaded as menu icons.
    if !extension.is_empty()
        && extension != "ico"
        && extension != "exe"
        && extension != "dll"
        && extension != "cur"
        && extension != "ani"
        && extension != "icl"
        && extension != "cpl"
    {
        if let Ok(picture) = crate::pictures::read(text) {
            let picture = crate::pictures::resized(picture, Some(cx as i64), Some(cy as i64));
            return the_bitmap_of_the_picture(&picture);
        }
    }

    // For a standalone .ico file with IconNumber == 1, LoadImageW picks the
    // best resolution matching SM_CXSMICON x SM_CYSMICON directly.
    if extension == "ico" && number <= 1 {
        let loaded = unsafe {
            LoadImageW(
                std::ptr::null_mut(),
                Wide::of(text).as_ptr(),
                IMAGE_ICON,
                cx,
                cy,
                LR_LOADFROMFILE,
            )
        };
        if !loaded.is_null() {
            let bitmap = icon_to_menu_bitmap(loaded, cx, cy);
            unsafe { DestroyIcon(loaded) };
            if bitmap.is_some() {
                return bitmap;
            }
        }
    }

    let mut large = std::ptr::null_mut();
    let mut small = std::ptr::null_mut();
    // The number of the icon of the v1 counts from one, and the number
    // of the machine counts from zero.
    let index = number.saturating_sub(1).max(0) as i32;
    let count = unsafe {
        ExtractIconExW(
            Wide::of(text).as_ptr(),
            index,
            &mut large,
            &mut small,
            1,
        )
    };
    if count == 0 {
        return None;
    }
    let icon = if small.is_null() { large } else { small };
    let bitmap = if icon.is_null() {
        None
    } else {
        icon_to_menu_bitmap(icon, cx, cy)
    };
    unsafe {
        if !large.is_null() {
            DestroyIcon(large);
        }
        if !small.is_null() && small != large {
            DestroyIcon(small);
        }
    }
    bitmap
}

/// Convert an `HICON` into a 32-bit top-down premultiplied-alpha DIB section
/// suitable for `MENUITEMINFOW.hbmpItem` on Windows Vista and later.
fn icon_to_menu_bitmap(
    icon: windows_sys::Win32::UI::WindowsAndMessaging::HICON,
    width: i32,
    height: i32,
) -> Option<windows_sys::Win32::Graphics::Gdi::HBITMAP> {
    use windows_sys::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, SelectObject,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{DI_MASK, DI_NORMAL, DrawIconEx};
    if icon.is_null() || width <= 0 || height <= 0 {
        return None;
    }
    unsafe {
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = width;
        info.bmiHeader.biHeight = -height;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;

        let hdc = CreateCompatibleDC(std::ptr::null_mut());
        if hdc.is_null() {
            return None;
        }
        let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let bitmap = CreateDIBSection(
            hdc,
            &info,
            DIB_RGB_COLORS,
            &mut bits,
            std::ptr::null_mut(),
            0,
        );
        if bitmap.is_null() || bits.is_null() {
            if !bitmap.is_null() {
                DeleteObject(bitmap);
            }
            DeleteDC(hdc);
            return None;
        }

        let byte_len = (width as usize) * (height as usize) * 4;
        let pixels = std::slice::from_raw_parts_mut(bits as *mut u8, byte_len);
        pixels.fill(0);

        let previous = SelectObject(hdc, bitmap);
        let drawn = DrawIconEx(
            hdc,
            0,
            0,
            icon,
            width,
            height,
            0,
            std::ptr::null_mut(),
            DI_NORMAL,
        );
        SelectObject(hdc, previous);
        if drawn == 0 {
            DeleteObject(bitmap);
            DeleteDC(hdc);
            return None;
        }

        // Check whether DrawIconEx produced any alpha values. 32-bit icons
        // with alpha already carry alpha; legacy 1/4/8/24-bit icons leave
        // alpha == 0 and need the icon's AND mask applied to set alpha.
        let has_alpha = pixels.chunks_exact(4).any(|px| px[3] != 0);
        if !has_alpha {
            let mut mask_bits: *mut core::ffi::c_void = std::ptr::null_mut();
            let mask_bm = CreateDIBSection(
                hdc,
                &info,
                DIB_RGB_COLORS,
                &mut mask_bits,
                std::ptr::null_mut(),
                0,
            );
            if !mask_bm.is_null() && !mask_bits.is_null() {
                let mask_pixels = std::slice::from_raw_parts_mut(mask_bits as *mut u8, byte_len);
                mask_pixels.fill(0);
                let prev_mask = SelectObject(hdc, mask_bm);
                let mask_ok = DrawIconEx(
                    hdc,
                    0,
                    0,
                    icon,
                    width,
                    height,
                    0,
                    std::ptr::null_mut(),
                    DI_MASK,
                );
                SelectObject(hdc, prev_mask);
                if mask_ok != 0 {
                    for (px, mpx) in pixels.chunks_exact_mut(4).zip(mask_pixels.chunks_exact(4)) {
                        if mpx[0] == 0 && mpx[1] == 0 && mpx[2] == 0 {
                            px[3] = 255;
                        } else {
                            px[0] = 0;
                            px[1] = 0;
                            px[2] = 0;
                            px[3] = 0;
                        }
                    }
                }
                DeleteObject(mask_bm);
            } else if !mask_bm.is_null() {
                DeleteObject(mask_bm);
            }
        } else {
            // Ensure premultiplied alpha (RGB <= A) for Windows themed menus.
            let unpremultiplied = pixels
                .chunks_exact(4)
                .any(|px| px[0] > px[3] || px[1] > px[3] || px[2] > px[3]);
            if unpremultiplied {
                for px in pixels.chunks_exact_mut(4) {
                    let a = px[3] as u16;
                    px[0] = ((px[0] as u16 * a + 127) / 255) as u8;
                    px[1] = ((px[1] as u16 * a + 127) / 255) as u8;
                    px[2] = ((px[2] as u16 * a + 127) / 255) as u8;
                }
            }
        }

        DeleteDC(hdc);
        Some(bitmap)
    }
}

/// The size of an icon of the machine.
fn the_size_of_the_icon(handle: windows_sys::Win32::UI::WindowsAndMessaging::HICON) -> (i64, i64) {
    use windows_sys::Win32::Graphics::Gdi::{BITMAP, GetObjectW};
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetIconInfo, ICONINFO};
    unsafe {
        let mut info: ICONINFO = std::mem::zeroed();
        if GetIconInfo(handle, &mut info) == 0 {
            return (0, 0);
        }
        let mut bitmap: BITMAP = std::mem::zeroed();
        let read = GetObjectW(
            info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as *mut core::ffi::c_void,
        );
        if read == 0 {
            return (0, 0);
        }
        (bitmap.bmWidth as i64, bitmap.bmHeight as i64)
    }
}

/// The bitmap of the pixels of a picture.
///
/// The v1 gives the picture to the control as a bitmap of thirty-two
/// bits, and the machine paints it with the alpha que o GDI+ põe nela.
fn the_bitmap_of_the_picture(
    picture: &crate::pictures::Picture,
) -> Option<windows_sys::Win32::Graphics::Gdi::HBITMAP> {
    unsafe {
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = picture.width as i32;
        info.bmiHeader.biHeight = -(picture.height as i32);
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let handle = CreateDIBSection(
            std::ptr::null_mut(),
            &info,
            DIB_RGB_COLORS,
            &mut bits,
            std::ptr::null_mut(),
            0,
        );
        if handle.is_null() {
            return None;
        }
        let mut at = bits as *mut u8;
        for pixel in picture.pixels.chunks_exact(3) {
            *at = pixel[2];
            *at.add(1) = pixel[1];
            *at.add(2) = pixel[0];
            *at.add(3) = 255;
            at = at.add(4);
        }
        Some(handle)
    }
}

/// A picture is looked for on the screen.
///
/// The answer is the place of it, and the answer of a picture that the
/// machine cannot load is the one of the tree: "NotLoadable".
pub fn image_search(
    pictures: &[Picture],
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    image: &Image,
    options: &ImageOptions,
) -> Result<ImageSearch, String> {
    let picture = the_picture_of(pictures, image)
        .map_err(|_| "the picture could not be loaded.".to_string())?;
    let (left, top) = (x1.min(x2), y1.min(y2));
    let (right, bottom) = (x1.max(x2), y1.max(x2).max(y2));
    let width = right - left + 1;
    let height = bottom - top + 1;
    if width <= 0 || height <= 0 {
        return Ok(ImageSearch::NotFound);
    }
    let screen = super::screen::the_picture_of(left, top, width, height)?;
    match crate::pictures::search_in(&picture, &screen, options) {
        Some((x, y)) => Ok(ImageSearch::Found(x + left, y + top)),
        None => Ok(ImageSearch::NotFound),
    }
}

/// The picture that the script asks for.
///
/// The picture of a file is read with the codec of the tree, and the
/// picture that was loaded before is the one of the table.
fn the_picture_of(pictures: &[Picture], image: &Image) -> Result<Picture, ImageSearch> {
    match image {
        Image::File(name) => {
            if !std::path::Path::new(name).exists() {
                return Err(ImageSearch::NotLoadable);
            }
            crate::pictures::read(name).map_err(|_| ImageSearch::NotLoadable)
        }
        Image::Loaded(handle) => {
            let handle = *handle as usize;
            match pictures.get(handle.wrapping_sub(1)) {
                Some(picture) => Ok(Picture {
                    pixels: picture.pixels.clone(),
                    width: picture.width,
                    height: picture.height,
                }),
                None => Err(ImageSearch::NotLoadable),
            }
        }
    }
}

/// A colour of the screen is looked for.
///
/// The colour is the name of the v2: "#rrggbb", or the name of a
/// colour -- "Red", "Blue" and the others.
pub fn search_the_colour(picture: &Picture, target: [u8; 3], variation: i64) -> Option<(i64, i64)> {
    let variation = variation.clamp(0, 255) as i32;
    for y in 0..picture.height {
        for x in 0..picture.width {
            let at = (y * picture.width + x) * 3;
            let red = picture.pixels[at] as i32;
            let green = picture.pixels[at + 1] as i32;
            let blue = picture.pixels[at + 2] as i32;
            if (red - target[0] as i32).abs() <= variation
                && (green - target[1] as i32).abs() <= variation
                && (blue - target[2] as i32).abs() <= variation
            {
                return Some((x as i64, y as i64));
            }
        }
    }
    None
}

/// The colour of the v2, as the three numbers of the machine.
pub fn the_colour_of(colour: &str) -> Option<[u8; 3]> {
    let text = colour.trim();
    let text = text.strip_prefix('#').unwrap_or(text);
    let text = text.strip_prefix("0x").unwrap_or(text);
    if text.len() == 6 {
        let red = u8::from_str_radix(&text[0..2], 16).ok()?;
        let green = u8::from_str_radix(&text[2..4], 16).ok()?;
        let blue = u8::from_str_radix(&text[4..6], 16).ok()?;
        return Some([red, green, blue]);
    }
    match text.to_ascii_lowercase().as_str() {
        "black" => Some([0, 0, 0]),
        "white" => Some([255, 255, 255]),
        "red" => Some([255, 0, 0]),
        "green" => Some([0, 255, 0]),
        "blue" => Some([0, 0, 255]),
        "gray" | "grey" => Some([128, 128, 128]),
        "yellow" => Some([255, 255, 0]),
        "cyan" => Some([0, 255, 255]),
        "magenta" => Some([255, 0, 255]),
        _ => None,
    }
}
