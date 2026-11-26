//! The styles of the windows of the machine and the colour that they let
//! through.
//!
//! The style of a window is the number of bits of Windows: the `WS_*` of
//! the style and the `WS_EX_*` of the extended style. The v2 reads and
//! writes those numbers, and the machine is the one that keeps them:
//! `GetWindowLongPtrW` reads and `SetWindowLongPtrW` writes.
//!
//! The colour that a window lets through is the colour key of a *layered*
//! window: the window lets the pixels of that colour through without
//! drawing them, and the rest of it is drawn. A window that is not
//! layered has no colour at all, and the answer is the empty text --
//! measured on the binary of v2.0.31, where the window of
//! the script answered the empty text.
//!
//! The new style that the script writes is the **absolute** style: the
//! one that computes the `+`, the `-` and the `^` over the style that
//! was there is the runtime, and the machine receives the final number.

use windows_sys::Win32::Foundation::{COLORREF, GetLastError, HWND, SetLastError};
use windows_sys::Win32::Graphics::Gdi::InvalidateRect;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GWL_STYLE, GetLayeredWindowAttributes, GetWindowLongPtrW,
    LAYERED_WINDOW_ATTRIBUTES_FLAGS, LWA_ALPHA, LWA_COLORKEY, SWP_DRAWFRAME, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetLayeredWindowAttributes,
    SetWindowLongPtrW, SetWindowPos, WINDOW_LONG_PTR_INDEX, WS_EX_LAYERED,
};

/// The index of the style of a window, or the one of its extended style.
///
/// The `GWL_STYLE` is the style and the `GWL_EXSTYLE` is the extended
/// style: the two are the same question with two answers.
fn the_index_of_the_style(extended: bool) -> WINDOW_LONG_PTR_INDEX {
    if extended { GWL_EXSTYLE } else { GWL_STYLE }
}

/// The style of a window, or its extended style.
///
/// The number is the one that the v2 answers: `UInt32` in its source,
/// and here the `i64` that the trait of the tree carries. The bit that
/// the machine puts on its own comes with it, as it comes in the v2: a
/// top-level window has the `WS_CLIPSIBLINGS` (the `67108864`) even
/// when nobody asked for it.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_style_of(window: HWND, extended: bool) -> Result<i64, String> {
    unsafe { SetLastError(0) };
    let style = unsafe { GetWindowLongPtrW(window, the_index_of_the_style(extended)) };
    if style == 0 {
        // The zero is the style of a window with no style at all, and the
        // machine uses it as well to say that the window does not exist:
        // the number of the failure is what tells the two things apart.
        let error = unsafe { GetLastError() };
        if error != 0 {
            return Err(super::sys::the_message_of(error));
        }
    }
    Ok(style as i64)
}

/// The style of a window is written, and the window is drawn again.
///
/// The `style` is the new **absolute** style: the runtime has already
/// computed the `+`, the `-` and the `^` over the style that was there.
///
/// What the v2 does (source `lib/win.cpp:1794-1839` of v2.0.31): the new
/// style that is the same as the current one is not written -- and it is
/// not a failure --, the writing is checked by the reading that comes
/// after it, the top-level window is drawn again with the `SetWindowPos`
/// (it is the one that changes the whole frame of the window) and with
/// the `InvalidateRect` (it is the one that makes the styles that need
/// a drawing appear).
///
/// The `of_a_window` says whether the window is a top-level one: the v2
/// does not call the `SetWindowPos` for a control (source
/// `lib/win.cpp:1826-1834` of v2.0.31).
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_style_is_written(
    window: HWND,
    style: i64,
    extended: bool,
    of_a_window: bool,
) -> Result<(), String> {
    if !the_style_is_written_roughly(window, style, extended)? {
        return Ok(());
    }
    if of_a_window {
        // The frame of the window changes with the `SetWindowPos`, and
        // the v2 does not look at what it answers.
        unsafe {
            SetWindowPos(
                window,
                std::ptr::null_mut(),
                0,
                0,
                0,
                0,
                SWP_DRAWFRAME
                    | SWP_FRAMECHANGED
                    | SWP_NOMOVE
                    | SWP_NOSIZE
                    | SWP_NOZORDER
                    | SWP_NOACTIVATE,
            );
        }
    }
    // Many styles only show when the window is drawn again.
    unsafe {
        InvalidateRect(window, std::ptr::null(), 1);
    }
    Ok(())
}

/// The style of a window is written, without the drawing again.
///
/// Answers `false` when the new style is the same as the current one:
/// the v2 writes nothing in that case ("No change needed. Detection of
/// success below relies on this check."), and the drawing again is not
/// needed either.
fn the_style_is_written_roughly(window: HWND, style: i64, extended: bool) -> Result<bool, String> {
    let index = the_index_of_the_style(extended);
    let current = unsafe { the_style_of(window, extended)? };
    if current == style {
        return Ok(false);
    }
    // The `SetLastError(0)` is what the documentation of Windows asks
    // before the writing: the zero that the writing answers is the new
    // style when it is zero, and not a failure.
    unsafe { SetLastError(0) };
    // The `SetWindowLongPtrW` is one word on the machine of thirty-two
    // bits and two on the one of sixty-four, and the style fits in both.
    let written = unsafe { SetWindowLongPtrW(window, index, style as _) };
    if written == 0 {
        let error = unsafe { GetLastError() };
        if error != 0 {
            return Err(format!(
                "the machine did not write the style of the window: {}",
                super::sys::the_message_of(error)
            ));
        }
    }
    // The v2 checks the writing by the reading: a machine that takes the
    // call and does not write the style is a failure.
    if unsafe { the_style_of(window, extended)? } == current {
        return Err("the machine took the style of the window and did not write it.".to_string());
    }
    Ok(true)
}

/// The alpha of a layered window.
///
/// The alpha is the number from 0 to 255 that says how much of what is
/// behind the window the window lets through: 255 is the window that is
/// not transparent, and 0 is the window that is not seen.
///
/// The v2 (source `lib/win.cpp:1851-1866` of v2.0.31) reads the
/// attributes of the layered window and answers the alpha only when the
/// `LWA_ALPHA` flag is there; a window that is not layered has no alpha
/// at all, and its answer is the 255 of a window that is not transparent.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_alpha_of(window: HWND) -> Result<i64, String> {
    let mut colour: COLORREF = 0;
    let mut alpha: u8 = 0;
    let mut flags: LAYERED_WINDOW_ATTRIBUTES_FLAGS = 0;
    let answered =
        unsafe { GetLayeredWindowAttributes(window, &mut colour, &mut alpha, &mut flags) };
    if answered == 0 {
        // The window is not layered: it is not transparent.
        return Ok(255);
    }
    if flags & LWA_ALPHA == 0 {
        // The window is layered, but its transparency is the colour key:
        // the alpha says nothing about it.
        return Ok(255);
    }
    Ok(alpha as i64)
}

/// The alpha of a window is written.
///
/// The window becomes layered before the alpha is written: Windows
/// forgets the transparency of a window when the `WS_EX_LAYERED` bit
/// goes away, and it only applies the alpha to a window that is layered.
///
/// The v2 (source `lib/win.cpp:1891-1934` of v2.0.31) writes the alpha
/// with the colour key `CLR_NONE` and the `LWA_ALPHA` flag, and it
/// refuses a number that is outside the 0 to 255.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_alpha_is_written(window: HWND, alpha: i64) -> Result<(), String> {
    if !(0..=255).contains(&alpha) {
        return Err(format!(
            "the transparency {alpha} is not a number between 0 and 255."
        ));
    }
    let exstyle = unsafe { the_style_of(window, true)? };
    if exstyle & WS_EX_LAYERED as i64 == 0 {
        the_style_is_written_roughly(window, exstyle | WS_EX_LAYERED as i64, true)?;
    }
    let answered = unsafe { SetLayeredWindowAttributes(window, 0, alpha as u8, LWA_ALPHA) };
    if answered == 0 {
        return Err(format!(
            "the machine did not take the transparency of the window: {}",
            super::sys::the_last_error()
        ));
    }
    Ok(())
}

/// The colour that a window lets through.
///
/// The v2 (source `lib/win.cpp:1868-1887` of v2.0.31) reads the
/// attributes of the layered window and answers the colour key in the
/// `0x%06X` format, of the RGB that the script expects; a window that
/// is not layered -- or that has no colour key -- answers the empty
/// text.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_transparent_colour_of(window: HWND) -> Result<String, String> {
    let mut colour: COLORREF = 0;
    let mut alpha: u8 = 0;
    let mut flags: LAYERED_WINDOW_ATTRIBUTES_FLAGS = 0;
    let answered =
        unsafe { GetLayeredWindowAttributes(window, &mut colour, &mut alpha, &mut flags) };
    if answered == 0 {
        // The window is not layered: the v2 answers the empty text.
        return Ok(String::new());
    }
    if flags & LWA_COLORKEY == 0 {
        return Ok(String::new());
    }
    Ok(format!("0x{:06X}", the_rgb_of(colour)))
}

/// The colour of a `COLORREF`, in the RGB that the v2 writes.
///
/// The `COLORREF` of Windows is the `0x00BBGGRR` -- the red is the low
/// byte --, and the text that the v2 writes is the `0xRRGGBB` -- the
/// red is the high byte. The swap of the bytes of the ends is the
/// `bgr_to_rgb` of the source of the v2.
fn the_rgb_of(colour: COLORREF) -> u32 {
    let red = colour & 0xFF;
    let green = (colour >> 8) & 0xFF;
    let blue = (colour >> 16) & 0xFF;
    (red << 16) | (green << 8) | blue
}

/// The colour that a window lets through is written.
///
/// The `colour` comes normalised: `0xRRGGBB` (the RGB that the script
/// wrote) or the empty text (the colour goes away). The alpha, when it
/// comes, is the number that comes after a space -- as the v2 does
/// (`WinSetTransColor "0x0000FF 128"`).
///
/// The v2 (source `lib/win.cpp:1891-1934` of v2.0.31): the colour key
/// needs the window to be layered, and the window that loses the colour
/// loses the `WS_EX_LAYERED` -- Windows forgets the transparency of a
/// window when the bit goes away.
///
/// # Safety
///
/// The window is the handle that the machine gave, and the window has to
/// exist: the call is the one of the machine, and the machine does not
/// ask.
pub unsafe fn the_transparent_colour_is_written(window: HWND, colour: &str) -> Result<(), String> {
    let colour = colour.trim();
    if colour.is_empty() {
        let exstyle = unsafe { the_style_of(window, true)? };
        if exstyle & WS_EX_LAYERED as i64 != 0 {
            the_style_is_written_roughly(window, exstyle & !(WS_EX_LAYERED as i64), true)?;
        }
        return Ok(());
    }
    let (colour, alpha) = the_colour_and_the_alpha_of(colour)?;
    // The window has to be layered before the colour key is written.
    let exstyle = unsafe { the_style_of(window, true)? };
    if exstyle & WS_EX_LAYERED as i64 == 0 {
        the_style_is_written_roughly(window, exstyle | WS_EX_LAYERED as i64, true)?;
    }
    let mut flags = LWA_COLORKEY;
    let mut value = 0u8;
    if let Some(alpha) = alpha {
        flags |= LWA_ALPHA;
        value = alpha;
    }
    let answered = unsafe { SetLayeredWindowAttributes(window, colour, value, flags) };
    if answered == 0 {
        return Err(format!(
            "the machine did not take the transparent colour: {}",
            super::sys::the_last_error()
        ));
    }
    Ok(())
}

/// The colour that the script wrote, and the alpha that comes after it.
///
/// The colour is the RGB that the v2 writes: `0xRRGGBB`. The alpha is
/// the number that comes after a space, and it goes from 0 to 255 -- the
/// v2 refuses a number that is outside of it.
fn the_colour_and_the_alpha_of(text: &str) -> Result<(COLORREF, Option<u8>), String> {
    let mut parts = text.split_whitespace();
    let first = parts.next().unwrap_or_default();
    let rest: Vec<&str> = parts.collect();
    let number = first
        .strip_prefix("0x")
        .or_else(|| first.strip_prefix("0X"))
        .unwrap_or(first);
    let not_a_colour = || format!("the colour {text:?} is not a colour of the machine.");
    if number.is_empty() || !number.chars().all(|letter| letter.is_ascii_hexdigit()) {
        return Err(not_a_colour());
    }
    let value = u32::from_str_radix(number, 16).map_err(|_| not_a_colour())?;
    if value > 0x00FF_FFFF {
        return Err(not_a_colour());
    }
    let red = (value >> 16) & 0xFF;
    let green = (value >> 8) & 0xFF;
    let blue = value & 0xFF;
    // The `COLORREF` of Windows is the `0x00BBGGRR`.
    let colour = red | (green << 8) | (blue << 16);
    let alpha = match rest.first() {
        None => None,
        Some(text) => {
            let not_an_alpha =
                || format!("the transparency {text:?} is not a number between 0 and 255.");
            let number: i64 = text.parse().map_err(|_| not_an_alpha())?;
            if !(0..=255).contains(&number) {
                return Err(not_an_alpha());
            }
            Some(number as u8)
        }
    };
    Ok((colour, alpha))
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Foundation::HINSTANCE;
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW, WNDCLASSEXW,
        WS_OVERLAPPEDWINDOW,
    };

    use crate::windows::sys::Wide;

    /// The window of the test: it exists only for the test, and its style
    /// is the one that the v2 measured.
    fn a_window() -> HWND {
        unsafe {
            let module: HINSTANCE = GetModuleHandleW(std::ptr::null());
            let class = Wide::of("ahk-rs-test-style");
            let window_class = WNDCLASSEXW {
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: module,
                lpszClassName: class.as_ptr(),
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                ..std::mem::zeroed()
            };
            RegisterClassExW(&window_class);
            let title = Wide::of("the window of the test of the style of the machine");
            let class = Wide::of("ahk-rs-test-style");
            let window = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                320,
                240,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                module,
                std::ptr::null(),
            );
            assert!(!window.is_null(), "the window of the test is there");
            window
        }
    }

    #[test]
    fn the_style_of_a_window_is_the_style_that_the_machine_gave_it() {
        let window = a_window();
        // Measured on the binary of v2.0.31: the style of
        // the window of the script is 80674816, and the machine puts the
        // WS_CLIPSIBLINGS on its own.
        assert_eq!(
            unsafe { the_style_of(window, false).expect("the style is read") },
            80674816
        );
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_style_of_a_window_is_written_and_read_again() {
        let window = a_window();
        let current = unsafe { the_style_of(window, false).expect("the style is read") };
        let wanted = current & !(0x0004_0000); // Takes the WS_THICKFRAME away.
        unsafe { the_style_is_written(window, wanted, false, true).expect("the style is written") };
        assert_eq!(
            unsafe { the_style_of(window, false).expect("the style is read") },
            wanted
        );
        // The same style again is not a failure: the v2 writes nothing.
        unsafe { the_style_is_written(window, wanted, false, true).expect("the style is written") };
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn a_window_that_is_not_layered_has_no_transparent_colour() {
        let window = a_window();
        assert_eq!(
            unsafe { the_transparent_colour_of(window).expect("the colour is read") },
            String::new()
        );
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_transparent_colour_of_a_window_is_written_and_read_again() {
        let window = a_window();
        unsafe {
            the_transparent_colour_is_written(window, "0xFF0000").expect("the colour is written")
        };
        assert_eq!(
            unsafe { the_transparent_colour_of(window).expect("the colour is read") },
            "0xFF0000"
        );
        // The window became layered.
        let exstyle = unsafe { the_style_of(window, true).expect("the extended style is read") };
        assert_eq!(exstyle & WS_EX_LAYERED as i64, WS_EX_LAYERED as i64);
        // The colour that goes away takes the WS_EX_LAYERED with it, as in
        // the v2.
        unsafe { the_transparent_colour_is_written(window, "").expect("the colour is written") };
        assert_eq!(
            unsafe { the_transparent_colour_of(window).expect("the colour is read") },
            String::new()
        );
        let exstyle = unsafe { the_style_of(window, true).expect("the extended style is read") };
        assert_eq!(exstyle & WS_EX_LAYERED as i64, 0);
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_alpha_of_a_window_is_written_and_read_again() {
        let window = a_window();
        // A window that is not layered is not transparent: its alpha is
        // the 255 of a window that is seen whole.
        assert_eq!(unsafe { the_alpha_of(window).expect("the alpha is read") }, 255);
        unsafe { the_alpha_is_written(window, 128).expect("the alpha is written") };
        assert_eq!(unsafe { the_alpha_of(window).expect("the alpha is read") }, 128);
        // The window became layered.
        let exstyle = unsafe { the_style_of(window, true).expect("the extended style is read") };
        assert_eq!(exstyle & WS_EX_LAYERED as i64, WS_EX_LAYERED as i64);
        // The alpha that is outside the 0 to 255 is refused.
        assert!(unsafe { the_alpha_is_written(window, 256) }.is_err());
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn a_colour_of_an_rgb_is_the_bgr_of_the_machine() {
        // The RGB 0xFF0000 is the BGR 0x0000FF.
        assert_eq!(
            the_colour_and_the_alpha_of("0xFF0000").expect("the colour is read"),
            (0x0000_00FF, None)
        );
        assert_eq!(
            the_colour_and_the_alpha_of("0x0000FF 128").expect("the colour is read"),
            (0x00FF_0000, Some(128))
        );
        // The text that is not a colour is refused.
        assert!(the_colour_and_the_alpha_of("vermelho").is_err());
        assert!(the_colour_and_the_alpha_of("0xFF0000 300").is_err());
    }
}
