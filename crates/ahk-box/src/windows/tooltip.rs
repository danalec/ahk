//! The tip of the machine.
//!
//! The tip is a small window of the machine with a text in it: the
//! script shows it, and it goes away when the script asks for nothing.
//!
//! The window is a window of the tree, and it is a window **layered**:
//! the transparency of it is written with `SetLayeredWindowAttributes`,
//! the same call that the transparency of the window of the script uses.
//! A layered window is a window that the Windows mixes with what is
//! behind it, and that is why the tip does not blink when it appears.
//!
//! Measured on the binary of v2.0.19: showing a tip
//! answers an Integer, and the tip with no text takes it away.

use std::cell::RefCell;

use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, COLOR_WINDOWFRAME, COLOR_WINDOWTEXT, CreateSolidBrush, DEFAULT_GUI_FONT, DT_LEFT,
    DT_SINGLELINE, DT_VCENTER, DeleteObject, DrawTextW, EndPaint, FillRect, FrameRect, GetDC,
    GetStockObject, GetSysColor, GetTextExtentPoint32W, InvalidateRect, PAINTSTRUCT, ReleaseDC,
    SYS_COLOR_INDEX, SelectObject, SetBkMode, SetTextColor, TRANSPARENT,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, GetClientRect, GetCursorPos, LWA_ALPHA, RegisterClassExW,
    SWP_SHOWWINDOW, SetLayeredWindowAttributes, SetWindowPos, WM_PAINT, WNDCLASSEXW, WS_EX_LAYERED,
    WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

use super::sys::Wide;

/// The class of the tip of the machine.
const THE_CLASS: &str = "ahk-rs-tooltip";

/// The margin around the text of the tip, in pixels.
const THE_MARGIN: i32 = 4;

/// The transparency of the tip: it is opaque, and the Windows mixes it
/// with what is behind it all the same.
const THE_ALPHA: u8 = 255;

/// The colour of the background of the tips of the system.
///
/// `COLOR_INFOBK` (`winuser.h`): the twenty-sixth colour of the system,
/// which `windows-sys` calls `COLOR_HOTLIGHT`.
const THE_BACKGROUND: SYS_COLOR_INDEX = 26;

// The text that the tip shows.
//
// The text is of the thread that shows the tip, and the window is of
// that thread too: the procedure of it reads the text from here.
thread_local! {
    static THE_TEXT: RefCell<String> = const { RefCell::new(String::new()) };
}

/// The procedure of the window of the tip.
///
/// The window draws itself: the background is the one of the tips of the
/// system, the text is the one of the script, and a border is around.
unsafe extern "system" fn the_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_PAINT => {
            unsafe { the_tip_is_drawn(window) };
            0
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

/// The tip is drawn.
unsafe fn the_tip_is_drawn(window: HWND) {
    unsafe {
        let mut paint: PAINTSTRUCT = std::mem::zeroed();
        let device = BeginPaint(window, &mut paint);
        let mut rectangle: RECT = std::mem::zeroed();
        GetClientRect(window, &mut rectangle);
        // The background is the one of the tips of the system.
        let background = CreateSolidBrush(GetSysColor(THE_BACKGROUND));
        FillRect(device, &rectangle, background);
        DeleteObject(background);
        // The text is the one of the script, in the font of the session.
        //
        // The thread may be in the way of its own end: the window of the tip
        // is destroyed with the thread, and the paint of it may come while
        // the thread-local of the text is gone. The tip is then empty.
        let text = THE_TEXT
            .try_with(|text| text.borrow().clone())
            .unwrap_or_default();
        let wide = Wide::of(&text);
        SetBkMode(device, TRANSPARENT as i32);
        SetTextColor(device, GetSysColor(COLOR_WINDOWTEXT));
        let font = GetStockObject(DEFAULT_GUI_FONT);
        let before = SelectObject(device, font);
        let mut area = rectangle;
        DrawTextW(
            device,
            wide.as_ptr(),
            -1,
            &mut area,
            DT_LEFT | DT_SINGLELINE | DT_VCENTER,
        );
        SelectObject(device, before);
        // The border is the one of the windows of the system.
        let frame = CreateSolidBrush(GetSysColor(COLOR_WINDOWFRAME));
        FrameRect(device, &rectangle, frame);
        DeleteObject(frame);
        EndPaint(window, &paint);
    }
}

/// The window of the tip is made.
///
/// The window is born layered and transparent to the mouse: the tip is
/// in the way of the mouse, and the mouse goes over it.
pub fn the_tip_is_made() -> Result<HWND, String> {
    unsafe {
        let module: HINSTANCE = GetModuleHandleW(std::ptr::null_mut());
        let class = Wide::of(THE_CLASS);
        let window_class = WNDCLASSEXW {
            lpfnWndProc: Some(the_procedure),
            hInstance: module,
            lpszClassName: class.as_ptr(),
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            ..std::mem::zeroed()
        };
        if RegisterClassExW(&window_class) == 0 {
            let code = windows_sys::Win32::Foundation::GetLastError();
            // 1410 is ERROR_CLASS_ALREADY_EXISTS: the class is already
            // registered, and the window can be made all the same.
            if code != 1410 {
                return Err(format!(
                    "the machine did not take the class of the tip: {code}."
                ));
            }
        }
        let handle = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            class.as_ptr(),
            std::ptr::null(),
            WS_POPUP,
            0,
            0,
            10,
            10,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        if handle.is_null() {
            let code = windows_sys::Win32::Foundation::GetLastError();
            return Err(format!(
                "the machine did not make the window of the tip: {code}."
            ));
        }
        Ok(handle)
    }
}

/// The tip shows a text, in the place that the script asks for.
///
/// The place is the one that the script writes; when the script writes
/// no place at all, the tip goes to the cursor, which is what v2 does
/// (`source/script2.cpp:1178-1186` of v2.0.31).
///
/// The tip is drawn again every time, and the transparency of it is
/// written with `SetLayeredWindowAttributes`: the window is layered,
/// and that is why the Windows mixes it with what is behind it.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has
/// to exist: the call is the one of the machine, and the machine does
/// not ask.
pub unsafe fn the_tip_is_written(handle: HWND, text: &str, x: i64, y: i64) -> Result<(), String> {
    THE_TEXT.with(|slot| *slot.borrow_mut() = text.to_string());
    let (width, height) = the_size_of_the_text(text);
    let (left, top) = if x == 0 && y == 0 {
        let mut place: POINT = unsafe { std::mem::zeroed() };
        unsafe {
            GetCursorPos(&mut place);
        }
        (place.x + 16, place.y + 16)
    } else {
        (x as i32, y as i32)
    };
    unsafe {
        SetWindowPos(
            handle,
            std::ptr::null_mut(),
            left,
            top,
            width + THE_MARGIN * 2,
            height + THE_MARGIN * 2,
            SWP_SHOWWINDOW,
        );
        // The tip is layered: the transparency of it is the same call
        // that the transparency of the window of the script uses.
        let answered = SetLayeredWindowAttributes(handle, 0, THE_ALPHA, LWA_ALPHA);
        if answered == 0 {
            return Err(format!(
                "the machine did not take the transparency of the tip: {}",
                super::sys::the_last_error()
            ));
        }
        // The text is new, and the window is drawn again.
        InvalidateRect(handle, std::ptr::null(), 1);
    }
    Ok(())
}

/// The size of a text in the font of the session.
///
/// The font is the one that the session uses for the windows
/// (`DEFAULT_GUI_FONT`), and it is the same that the tip uses to write
/// the text.
fn the_size_of_the_text(text: &str) -> (i32, i32) {
    let wide = Wide::of(text);
    // The machine counts the letters of the text: the zero of the end is
    // not one of them.
    let letters = wide.letters().len().saturating_sub(1);
    unsafe {
        let device = GetDC(std::ptr::null_mut());
        let mut size: SIZE = std::mem::zeroed();
        GetTextExtentPoint32W(device, wide.as_ptr(), letters as i32, &mut size);
        ReleaseDC(std::ptr::null_mut(), device);
        (size.cx, size.cy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DestroyWindow, GWL_EXSTYLE, GetWindowLongPtrW,
    };

    #[test]
    fn a_tip_is_made_and_it_is_a_window_layered() {
        // The tip is a window of the machine, and it is layered: the
        // transparency of it is the same call that the transparency of
        // the window of the script uses.
        let handle = the_tip_is_made().expect("the machine did not make the window of the tip");
        unsafe {
            the_tip_is_written(handle, "the tree", 10, 10)
                .expect("the tip did not take the text of the script");
        }
        let style = unsafe { GetWindowLongPtrW(handle, GWL_EXSTYLE) } as u32;
        assert_eq!(
            style & WS_EX_LAYERED,
            WS_EX_LAYERED,
            "the window of the tip is layered"
        );
        unsafe { DestroyWindow(handle) };
    }

    #[test]
    fn the_size_of_a_text_is_not_zero() {
        let (width, height) = the_size_of_the_text("the tree");
        assert!(width > 0, "the text of the tree is {width} pixels wide");
        assert!(height > 0, "the text of the tree is {height} pixels high");
    }
}
