//! The screen of the machine.
//!
//! The screen of Windows is the desktop of the session: the pixels of it
//! are read from the device context of the desktop with `BitBlt`, and the
//! colour of a pixel is `GetPixel` on that context.
//!
//! The picture of a region is written as a PNG with the codec of the
//! tree, and the matcher of the tree -- the one of `pictures` -- looks
//! for a picture in it.

use std::ffi::c_void;

use ahk::desktop::Monitor;

use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CAPTUREBLT, CreateCompatibleBitmap,
    CreateCompatibleDC, DIB_RGB_COLORS, DeleteDC, DeleteObject, EnumDisplayMonitors, GetDC,
    GetDIBits, GetMonitorInfoW, GetPixel, HDC, HMONITOR, MONITORINFOEXW, ReleaseDC, SRCCOPY,
    SelectObject,
};
use windows_sys::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, MONITORINFOF_PRIMARY, SM_CXSCREEN, SM_CYSCREEN,
};

use crate::pictures::Picture;

/// A metric of the system, the way `GetSystemMetrics` answers it: the
/// `SysGet` of a number that is not about the monitors asks the machine
/// for it (the size of an icon, the border of a window, and the rest).
pub fn system_metric(metric: i32) -> i64 {
    unsafe { GetSystemMetrics(metric) as i64 }
}

/// The process knows the pixels of the screen.
///
/// The declaration is of the process and not of the window: a program
/// that does not declare it is told the pixels of the screen of the
/// monitor of the system, and the tree wants the pixels of the monitor
/// of each window.
///
/// The declaration is of Windows 10 (1703), and the machine that does
/// not have it is told the truth of the older declaration.
fn the_process_is_aware_of_the_dpi() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        // A declaracao do DPI e de Windows 10, e a maquina que nao a
        // tem e avisada da verdade da declaracao mais antiga.
        let Some(symbol) = super::sys::the_symbol_of("user32.dll", "SetProcessDpiAwarenessContext")
        else {
            return;
        };
        type Setter = unsafe extern "system" fn(isize) -> windows_sys::core::BOOL;
        let setter: Setter = unsafe { std::mem::transmute(symbol) };
        unsafe { setter(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 as isize) };
    });
}

/// The pixels of a region of the screen.
///
/// The answer is the picture of the tree: three bytes for every pixel,
/// the red first.
pub fn the_picture_of(x: i64, y: i64, width: i64, height: i64) -> Result<Picture, String> {
    the_process_is_aware_of_the_dpi();
    if width <= 0 || height <= 0 {
        return Err("the region of the screen has no size.".to_string());
    }
    let width = width as i32;
    let height = height as i32;
    unsafe {
        let screen = GetDC(std::ptr::null_mut());
        if screen.is_null() {
            return Err("the machine gave no context of the desktop.".to_string());
        }
        let memory = CreateCompatibleDC(screen);
        if memory.is_null() {
            ReleaseDC(std::ptr::null_mut(), screen);
            return Err("the machine gave no context for the picture.".to_string());
        }
        let bitmap = CreateCompatibleBitmap(screen, width, height);
        if bitmap.is_null() {
            DeleteDC(memory);
            ReleaseDC(std::ptr::null_mut(), screen);
            return Err("the machine made no bitmap for the picture.".to_string());
        }
        let previous = SelectObject(memory, bitmap);
        // The copy takes the layer of the windows that are on the top of
        // the screen too: `CAPTUREBLT` is what makes the picture of the
        // screen the picture of what the user sees.
        let copied = BitBlt(
            memory,
            0,
            0,
            width,
            height,
            screen,
            x as i32,
            y as i32,
            SRCCOPY | CAPTUREBLT,
        );
        if copied == 0 {
            SelectObject(memory, previous);
            DeleteObject(bitmap);
            DeleteDC(memory);
            ReleaseDC(std::ptr::null_mut(), screen);
            return Err("the machine did not copy the picture of the screen.".to_string());
        }
        let mut info: BITMAPINFO = std::mem::zeroed();
        info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = width;
        // The height is negative: the rows come out the way the screen
        // shows them, and not from the bottom.
        info.bmiHeader.biHeight = -height;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let mut bits = vec![0u8; (width as usize) * (height as usize) * 4];
        let read = GetDIBits(
            memory,
            bitmap,
            0,
            height as u32,
            bits.as_mut_ptr().cast(),
            &mut info,
            DIB_RGB_COLORS,
        );
        SelectObject(memory, previous);
        DeleteObject(bitmap);
        DeleteDC(memory);
        ReleaseDC(std::ptr::null_mut(), screen);
        if read == 0 {
            return Err("the machine did not read the pixels of the screen.".to_string());
        }
        // The bits of the machine are blue, green and red, and the
        // picture of the tree is red, green and blue.
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

/// A region of the screen is written as a picture of a file.
///
/// The answer is the name of the file, the way the v2 answers it.
pub fn capture(x: i64, y: i64, width: i64, height: i64, filename: &str) -> Result<String, String> {
    let picture = the_picture_of(x, y, width, height)?;
    let png = crate::picture::png::write_rgb(picture.width, picture.height, &picture.pixels)?;
    let name = if filename.is_empty() {
        format!("ahk-{}.png", the_millis())
    } else {
        filename.to_string()
    };
    std::fs::write(&name, png).map_err(|error| format!("the picture was not written: {error}"))?;
    Ok(name)
}

/// The number of the milliseconds of the machine, since it started.
fn the_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_millis())
        .unwrap_or(0)
}

/// The colour of a pixel of the screen.
///
/// The answer of the tree is the colour the way the v2 spells it:
/// "#rrggbb", in lowercase, with the red of the machine first.
pub fn pixel(x: i64, y: i64) -> Result<String, String> {
    unsafe {
        // The context of the desktop is the one that has the pixels of
        // the screen, and it is given back to the machine right after.
        let context = GetDC(std::ptr::null_mut());
        if context.is_null() {
            return Err("the machine gave no context of the desktop.".to_string());
        }
        let colour = GetPixel(context, x as i32, y as i32);
        ReleaseDC(std::ptr::null_mut(), context);
        if colour == 0xFFFF_FFFF {
            return Err("the machine did not read the colour of the pixel.".to_string());
        }
        let (red, green, blue) = the_parts_of_the_colour(colour);
        Ok(format!("#{red:02x}{green:02x}{blue:02x}"))
    }
}

/// The three parts of the colour of the machine.
///
/// The colour of Windows is a number with the red of it in the low
/// byte, the green in the next one and the blue in the next one: the
/// order is the opposite of the order of the tree, which names the red
/// first.
fn the_parts_of_the_colour(colour: u32) -> (u8, u8, u8) {
    let red = (colour & 0x0000_00FF) as u8;
    let green = ((colour >> 8) & 0x0000_00FF) as u8;
    let blue = ((colour >> 16) & 0x0000_00FF) as u8;
    (red, green, blue)
}

/// The size of the screen of the machine.
pub fn screen_size() -> Result<(i64, i64), String> {
    the_process_is_aware_of_the_dpi();
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        if width <= 0 || height <= 0 {
            return Err("the machine did not say the size of the screen.".to_string());
        }
        Ok((width as i64, height as i64))
    }
}

/// The dots per inch of the screen.
///
/// The number v2 answers for `A_ScreenDPI`: the 96 of the ordinary session
/// with the scale of the machine put on it. The machine that has no DPI says
/// the 96.
pub fn screen_dpi() -> Result<i64, String> {
    the_process_is_aware_of_the_dpi();
    // O DPI da maquina e de Windows 10, e a maquina que nao o tem e a
    // do ordinary: o 96.
    let Some(symbol) = super::sys::the_symbol_of("user32.dll", "GetDpiForSystem") else {
        return Ok(96);
    };
    type Reader = unsafe extern "system" fn() -> u32;
    let reader: Reader = unsafe { std::mem::transmute(symbol) };
    let dpi = unsafe { reader() };
    if dpi == 0 {
        return Ok(96);
    }
    Ok(dpi as i64)
}

/// The monitors of the machine.
pub fn monitors() -> Result<Vec<Monitor>, String> {
    the_process_is_aware_of_the_dpi();
    let mut found: Vec<Monitor> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(collect),
            &mut found as *mut Vec<Monitor> as isize,
        );
    }
    Ok(found)
}

/// The callback of the enumeration of the monitors.
///
/// The pointer of the collection is the parameter of the call: the
/// machine gives it back to the callback, and the callback puts the
/// monitor in it.
unsafe extern "system" fn collect(
    monitor: HMONITOR,
    _context: *mut c_void,
    _rectangle: *mut RECT,
    parameter: isize,
) -> i32 {
    if monitor.is_null() {
        return 0;
    }
    let mut info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    if unsafe { GetMonitorInfoW(monitor, &mut info.monitorInfo) } == 0 {
        return 1;
    }
    let rectangle = info.monitorInfo.rcMonitor;
    let found = unsafe { &mut *(parameter as *mut Vec<Monitor>) };
    found.push(Monitor {
        name: the_name_of_the_monitor(&info),
        left: rectangle.left as i64,
        top: rectangle.top as i64,
        right: rectangle.right as i64,
        bottom: rectangle.bottom as i64,
        scale: the_dpi_of_the_monitor(monitor),
    });
    1
}

/// The dots per inch of a monitor, the number the rest of the tree reads for
/// the scale of it.
///
/// The monitor that has no DPI is the ordinary one, and the ordinary one is
/// the 96 of the tree of Windows.
fn the_dpi_of_the_monitor(monitor: HMONITOR) -> i64 {
    // The type of the DPI is the effective one: the one that the system
    // uses when the program is not aware of the scale of the monitor.
    // O DPI do monitor e de Windows 8.1, e a maquina que nao o tem e a
    // do ordinary: o 96.
    let Some(symbol) = super::sys::the_symbol_of("shcore.dll", "GetDpiForMonitor") else {
        return 96;
    };
    type Reader = unsafe extern "system" fn(
        windows_sys::Win32::Graphics::Gdi::HMONITOR,
        i32,
        *mut u32,
        *mut u32,
    ) -> windows_sys::core::HRESULT;
    let reader: Reader = unsafe { std::mem::transmute(symbol) };
    let mut effective = 0u32;
    let mut dpi = 0u32;
    let answer = unsafe { reader(monitor, 0, &mut dpi, &mut effective) };
    if answer != 0 || effective == 0 {
        return 96;
    }
    effective as i64
}

/// The name of a monitor, as the machine writes it.
fn the_name_of_the_monitor(info: &MONITORINFOEXW) -> String {
    let name: Vec<u16> = info
        .szDevice
        .iter()
        .copied()
        .take_while(|c| *c != 0)
        .collect();
    String::from_utf16_lossy(&name)
}

/// The work area of a monitor: the rectangle of it without the taskbar.
///
/// The answer is the four numbers of the tree: the left, the top, the
/// width and the height.
pub fn monitor_work_area(monitor: usize) -> Result<Option<(i64, i64, i64, i64)>, String> {
    the_process_is_aware_of_the_dpi();
    let mut areas: Vec<(i64, i64, i64, i64)> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(collect_the_work_areas),
            &mut areas as *mut Vec<(i64, i64, i64, i64)> as isize,
        );
    }
    // The number of a monitor counts from one, and the list of the machine
    // counts from zero.
    Ok(monitor.checked_sub(1).and_then(|index| areas.get(index)).copied())
}

/// The callback of the enumeration of the work areas.
unsafe extern "system" fn collect_the_work_areas(
    monitor: HMONITOR,
    _context: *mut c_void,
    _rectangle: *mut RECT,
    parameter: isize,
) -> i32 {
    if monitor.is_null() {
        return 0;
    }
    let mut info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    if unsafe { GetMonitorInfoW(monitor, &mut info.monitorInfo) } == 0 {
        return 1;
    }
    let work = info.monitorInfo.rcWork;
    let found = unsafe { &mut *(parameter as *mut Vec<(i64, i64, i64, i64)>) };
    found.push((
        work.left as i64,
        work.top as i64,
        (work.right - work.left) as i64,
        (work.bottom - work.top) as i64,
    ));
    1
}

/// The number of the monitor that is the primary one.
///
/// The machine has one primary monitor, and the one that carries the
/// flag of it is the one. The number of it counts from one, the way the
/// tree names the monitors: measured on the binary of v2.0.19 the `MonitorGetPrimary` of a machine with one monitor
/// answers 1.
pub fn primary_monitor() -> Result<usize, String> {
    let primaries: Vec<usize> = Vec::new();
    let seen = 0usize;
    let mut context = (primaries, seen);
    unsafe {
        let _ = EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(collect_the_primary),
            &mut context as *mut (Vec<usize>, usize) as isize,
        );
    }
    Ok(context.0.first().copied().unwrap_or(0))
}

/// The callback of the enumeration of the primary monitor.
///
/// The callback is told where the monitor is in the list of the machine,
/// and the number of the primary one counts from one.
unsafe extern "system" fn collect_the_primary(
    monitor: HMONITOR,
    _context: *mut c_void,
    _rectangle: *mut RECT,
    parameter: isize,
) -> i32 {
    if monitor.is_null() {
        return 0;
    }
    let (found, seen) = unsafe { &mut *(parameter as *mut (Vec<usize>, usize)) };
    let place = *seen;
    *seen += 1;
    let mut info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    if unsafe { GetMonitorInfoW(monitor, &mut info.monitorInfo) } == 0 {
        return 1;
    }
    if info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0 {
        found.push(place + 1);
    }
    1
}

/// The context of the desktop of the machine, and the way it is given back.
///
/// The context of the desktop is what has the pixels of the screen, and
/// the machine wants it back when the picture is read.
pub struct TheDesktop {
    context: HDC,
}

impl TheDesktop {
    /// The context of the desktop of the machine.
    pub fn of_the_machine() -> Result<Self, String> {
        let context = unsafe { GetDC(std::ptr::null_mut()) };
        if context.is_null() {
            return Err("the machine gave no context of the desktop.".to_string());
        }
        Ok(Self { context })
    }

    /// The context, for the calls that need it.
    pub fn context(&self) -> HDC {
        self.context
    }
}

impl Drop for TheDesktop {
    fn drop(&mut self) {
        unsafe {
            ReleaseDC(std::ptr::null_mut(), self.context);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_of_the_screen_is_not_zero() {
        let (width, height) = screen_size().expect("the machine says the size of the screen");
        assert!(width > 0, "{width}");
        assert!(height > 0, "{height}");
    }

    #[test]
    fn the_scale_of_the_screen_is_the_number_v2_answers() {
        // The number is the dots per inch: the 96 of the ordinary session or
        // the 120, 144, 192 of a scaled one, and never a percentage of it.
        let dots = screen_dpi().expect("the machine says the scale of the screen");
        assert!(dots >= 96, "{dots}");
        assert!(dots <= 480, "{dots}");
    }

    #[test]
    fn the_machine_has_a_monitor() {
        let monitors = monitors().expect("the machine says its monitors");
        assert!(!monitors.is_empty());
        for monitor in &monitors {
            assert!(monitor.right > monitor.left, "{monitor:?}");
            assert!(monitor.bottom > monitor.top, "{monitor:?}");
            assert!(monitor.scale >= 96, "{monitor:?}");
        }
    }

    #[test]
    fn the_primary_monitor_is_one_of_the_monitors() {
        let monitors = monitors().expect("the machine says its monitors");
        let primary = primary_monitor().expect("the machine says its primary monitor");
        // The number of a monitor counts from one.
        assert!(primary >= 1);
        assert!(primary <= monitors.len());
    }

    #[test]
    fn the_work_area_of_the_primary_monitor_is_inside_it() {
        let monitors = monitors().expect("the machine says its monitors");
        let primary = primary_monitor().expect("the machine says its primary monitor");
        let area = monitor_work_area(primary)
            .expect("the machine says the work area of the monitor")
            .expect("the monitor has a work area");
        let monitor = &monitors[primary - 1];
        assert!(area.0 >= monitor.left, "{area:?} {:?}", monitor);
        assert!(area.2 > 0, "{area:?}");
    }

    #[test]
    fn the_colour_of_a_pixel_is_a_colour() {
        // The pixel of the corner of the screen is a pixel of the screen,
        // and the machine reads it.
        let colour = pixel(0, 0).expect("the machine reads the colour of the pixel");
        assert_eq!(colour.len(), 7, "{colour}");
        assert!(colour.starts_with('#'), "{colour}");
        assert!(
            colour[1..].chars().all(|letter| letter.is_ascii_hexdigit()),
            "{colour}"
        );
    }

    #[test]
    fn a_region_of_the_screen_is_a_picture() {
        let picture = the_picture_of(0, 0, 8, 8).expect("the machine reads the pixels");
        assert_eq!(picture.width, 8);
        assert_eq!(picture.height, 8);
        assert_eq!(picture.pixels.len(), 8 * 8 * 3);
    }

    #[test]
    fn a_region_of_the_screen_is_written_as_a_picture_of_a_file() {
        let name = std::env::temp_dir()
            .join(format!("ahk-rs-capture-{}.png", std::process::id()))
            .to_string_lossy()
            .to_string();
        let answer = capture(0, 0, 16, 16, &name).expect("the picture is written");
        assert_eq!(answer, name);
        let picture = crate::pictures::read(&name).expect("the picture is read back");
        assert_eq!(picture.width, 16);
        assert_eq!(picture.height, 16);
        let _ = std::fs::remove_file(&name);
    }
}
