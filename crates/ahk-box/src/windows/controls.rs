//! The controls of the windows of the machine.
//!
//! A control of a window is a window of its own: the machine finds it
//! with `FindWindowExW`, and it talks with it with the messages of the
//! windows -- `WM_GETTEXT` and `WM_SETTEXT` for the text, `BM_CLICK`
//! for the click, and the answers of the machine for the flags.
//!
//! The name of a control is the name of the v2: "Edit1" is the class
//! "Edit" and the first one of them, and "Edit1.the text" is the class
//! "Edit" with that text.

use ahk::desktop::{ControlAnswer, ControlEdit, ControlFlag};

use windows_sys::Win32::Foundation::{HWND, LPARAM};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{IsWindowEnabled, SetFocus};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BM_CLICK, FindWindowExW, GetClassNameW, GetParent, IsWindowVisible, SendMessageW, WS_CHILD,
    WM_GETTEXT, WM_SETTEXT,
};

use super::sys::Wide;

/// The class of a control, the text of it and the number of it.
///
/// The name of the v2 is "ClassNN", "ClassNN.Text" or "ClassNN.NN":
/// the letters are the class of the control, the number is how many of
/// them are in the window, and the text is what is written in it.
fn the_parts_of_a_control(control: &str) -> (String, Option<String>, usize) {
    let control = control.trim();
    // The text of the control comes after the last dot, and the number
    // of it is the number that ends the name.
    if let Some((before, text)) = control.rsplit_once('.') {
        if let Ok(number) = text.parse::<usize>() {
            return (the_class_of(before), None, number.max(1));
        }
        return (the_class_of(before), Some(text.to_string()), 1);
    }
    let digits = control.len() - control.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    if digits > 0 {
        let number: usize = control[control.len() - digits..].parse().unwrap_or(1);
        return (
            the_class_of(&control[..control.len() - digits]),
            None,
            number.max(1),
        );
    }
    (the_class_of(control), None, 1)
}

/// The class of a control, the way the machine spells it.
///
/// The name of the v2 has the number of the control at its end, and
/// the class is the name without it.
fn the_class_of(name: &str) -> String {
    let name = name.trim();
    let class = name.trim_end_matches(|c: char| c.is_ascii_digit());
    if class.is_empty() {
        name.to_string()
    } else {
        class.to_string()
    }
}

/// The children of a window, in the order of the machine.
fn the_children_of(window: HWND) -> Vec<HWND> {
    let mut found = Vec::new();
    let mut child = unsafe {
        FindWindowExW(
            window,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    while !child.is_null() {
        found.push(child);
        child = unsafe { FindWindowExW(window, child, std::ptr::null(), std::ptr::null()) };
    }
    found
}

/// The control of a window, when the window has it.
///
/// The search is the one of the v2: the children of the window and the
/// children of them, in the order of the machine, and the control is
/// the n-th of the class that is wanted.
///
/// A control whose name is empty is the window itself when the window is a
/// child of another one, and the topmost child of it otherwise: the v1
/// (source `window.cpp:874-878` of v1.1.37.02, `ControlExist`):
/// "In v1.0.43.06, the parent window itself is returned if it's a child
/// rather than its top child because it seems more useful and intuitive.
/// This change allows ahk_id %ControlHwnd% to always operate directly on the
/// specified control's HWND rather than some sub-child."
pub fn the_control_of(window: HWND, control: &str) -> Option<HWND> {
    if control.trim().is_empty() {
        return the_control_of_an_empty_name(window);
    }
    let (class, text, number) = the_parts_of_a_control(control);
    let mut seen = 0usize;
    for child in the_children_of(window) {
        if let Some(found) = the_control_among(&[child], &class, text.as_deref(), &mut seen, number)
        {
            return Some(found);
        }
        let children = the_children_of(child);
        if let Some(found) =
            the_control_among(&children, &class, text.as_deref(), &mut seen, number)
        {
            return Some(found);
        }
    }
    None
}

/// The control of an empty name: the window itself when the window is
/// a child of another one, and the topmost child of it otherwise.
///
/// The handle of the window goes through this private function on purpose:
/// the public functions of this file hand the handle of the machine to the
/// machine, and the clippy of the project asks for the `unsafe` in the ones
/// that do it.
fn the_control_of_an_empty_name(window: HWND) -> Option<HWND> {
    let is_child = unsafe { super::styles::the_style_of(window, false) }
        .map(|style| style & WS_CHILD as i64 != 0)
        .unwrap_or(false);
    if is_child {
        return Some(window);
    }
    // The topmost child of the window: the `FindWindowExW` of the
    // machine starts the search at the top of the Z order.
    let top = unsafe {
        FindWindowExW(
            window,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    if top.is_null() {
        return None;
    }
    Some(top)
}

/// The control among the ones that are given, when it is there.
///
/// The number counts the controls of the class: the n-th of them is the
/// one that is wanted.
fn the_control_among(
    controls: &[HWND],
    class: &str,
    text: Option<&str>,
    seen: &mut usize,
    number: usize,
) -> Option<HWND> {
    for control in controls {
        let name = the_class_name_of(*control);
        if !name.eq_ignore_ascii_case(class) {
            continue;
        }
        *seen += 1;
        if *seen != number {
            continue;
        }
        match text {
            Some(wanted) => {
                if the_text_of(*control) == wanted {
                    return Some(*control);
                }
            }
            None => return Some(*control),
        }
    }
    None
}

/// The class of a window, the way the machine spells it.
fn the_class_name_of(window: HWND) -> String {
    let mut buffer = [0u16; 256];
    let read = unsafe { GetClassNameW(window, buffer.as_mut_ptr(), buffer.len() as i32) };
    if read <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buffer[..read as usize])
}

/// The text of a window.
///
/// The message of the text takes the buffer of it: the control copies
/// the text in it and answers how many letters it copied. The buffer is
/// of four thousand letters, and the text that is longer than that is
/// cut.
fn the_text_of(window: HWND) -> String {
    let mut buffer = vec![0u16; 4096];
    let read = unsafe {
        SendMessageW(
            window,
            WM_GETTEXT,
            buffer.len() as usize,
            buffer.as_mut_ptr() as LPARAM,
        )
    };
    if read <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buffer[..read as usize])
}

/// The text of a control of a window.
pub fn control_text(window: HWND, control: &str) -> ControlAnswer {
    match the_control_of(window, control) {
        Some(found) => ControlAnswer::Found(the_text_of(found)),
        None => ControlAnswer::NoSuchControl,
    }
}

/// Whether a control of a window is visible.
pub fn control_visible(window: HWND, control: &str) -> ControlFlag {
    match the_control_of(window, control) {
        Some(found) => {
            if unsafe { IsWindowVisible(found) } != 0 {
                ControlFlag::Yes
            } else {
                ControlFlag::No
            }
        }
        None => ControlFlag::NoSuchControl,
    }
}

/// Whether a control of a window is enabled.
pub fn control_enabled(window: HWND, control: &str) -> ControlFlag {
    match the_control_of(window, control) {
        Some(found) => {
            if unsafe { IsWindowEnabled(found) } != 0 {
                ControlFlag::Yes
            } else {
                ControlFlag::No
            }
        }
        None => ControlFlag::NoSuchControl,
    }
}

/// The text of a control of a window is written.
pub fn control_set_text(window: HWND, control: &str, text: &str) -> ControlEdit {
    let Some(found) = the_control_of(window, control) else {
        return ControlEdit::NoSuchControl;
    };
    let wide = Wide::of(text);
    unsafe {
        SendMessageW(found, WM_SETTEXT, 0, wide.as_ptr() as LPARAM);
    }
    ControlEdit::Done
}

/// A control of a window is clicked.
///
/// The click is the message of the machine: `BM_CLICK` is the button
/// that clicks itself, and the mouse of the session is the one of the
/// machine.
pub fn control_click(window: HWND, control: &str) -> ControlEdit {
    let Some(found) = the_control_of(window, control) else {
        return ControlEdit::NoSuchControl;
    };
    unsafe {
        SendMessageW(found, BM_CLICK, 0, 0);
    }
    ControlEdit::Done
}

/// A control of a window has the focus.
pub fn control_focus(window: HWND, control: &str) -> ControlEdit {
    let Some(found) = the_control_of(window, control) else {
        return ControlEdit::NoSuchControl;
    };
    unsafe {
        SetFocus(found);
    }
    ControlEdit::Done
}

/// The text is written in a control of a window, key by key.
///
/// The machine sends the characters to the control with `WM_CHAR`, and
/// the control of the window answers what it does with them.
pub fn control_send_text(window: HWND, control: &str, text: &str) -> ControlEdit {
    let Some(found) = the_control_of(window, control) else {
        return ControlEdit::NoSuchControl;
    };
    for letter in text.encode_utf16() {
        unsafe {
            SendMessageW(found, 0x0102, letter as usize, 1);
        }
    }
    ControlEdit::Done
}

/// The style of a control of a window, or its extended style.
///
/// A control is a window of its own, and its style is read the same way
/// as the one of a window: the number of bits of Windows.
pub fn control_style(window: HWND, control: &str, extended: bool) -> Result<i64, String> {
    let Some(found) = the_control_of(window, control) else {
        return Err(format!("the window has no control named {control:?}."));
    };
    unsafe { super::styles::the_style_of(found, extended) }
}

/// The style of a control of a window is written.
///
/// The `style` is the new **absolute** style: the runtime has already
/// computed the `+`, the `-` and the `^` over the style that was there.
///
/// The `SetWindowPos` is the one of the top-level windows, and the v2
/// does not call it for a control (source `lib/win.cpp:1826-1834` of
/// v2.0.31): the drawing again of the control is the `InvalidateRect`.
pub fn control_set_style(
    window: HWND,
    control: &str,
    style: i64,
    extended: bool,
) -> Result<(), String> {
    let Some(found) = the_control_of(window, control) else {
        return Err(format!("the window has no control named {control:?}."));
    };
    unsafe { super::styles::the_style_is_written(found, style, extended, false) }
}

/// The handles of the controls of a window, in the order of the machine.
///
/// `WinGetControlsHwnd` of a window of another program answers the handle
/// of every control of it: the children the machine enumerates.
pub fn control_hwnds(window: HWND) -> Vec<i64> {
    the_children_of(window)
        .into_iter()
        .map(|child| child as isize as i64)
        .collect()
}

/// The class and number that name a control, given its handle.
///
/// `ControlGetClassNN` of a handle answers the class of the control and
/// the place of it among the controls of the same class of its parent:
/// the first button is "Button1", the second "Button2".
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn control_classnn(control: HWND) -> String {
    let class = the_class_name_of(control);
    let parent = unsafe { GetParent(control) };
    let mut number = 0usize;
    if !parent.is_null() {
        for sibling in the_children_of(parent) {
            if the_class_name_of(sibling).eq_ignore_ascii_case(&class) {
                number += 1;
            }
            if sibling == control {
                break;
            }
        }
    }
    if number == 0 {
        number = 1;
    }
    format!("{class}{number}")
}

/// The style of a control given its handle, or its extended style.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn control_style_by_hwnd(control: HWND, extended: bool) -> Result<i64, String> {
    unsafe { super::styles::the_style_of(control, extended) }
}

/// Write the style of a control given its handle.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn control_set_style_by_hwnd(
    control: HWND,
    style: i64,
    extended: bool,
) -> Result<(), String> {
    unsafe { super::styles::the_style_is_written(control, style, extended, false) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Foundation::HINSTANCE;
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW, SW_SHOW,
        ShowWindow, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    };

    /// The window of the test, with a button in it.
    ///
    /// The window is made by the machine and the button is a control of
    /// it: the test reads the text of the button and writes another one.
    fn a_window_with_a_button() -> (HWND, HWND) {
        unsafe {
            let module: HINSTANCE = GetModuleHandleW(std::ptr::null());
            let class = Wide::of("ahk-rs-test-window");
            let window_class = WNDCLASSEXW {
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: module,
                lpszClassName: class.as_ptr(),
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                ..std::mem::zeroed()
            };
            RegisterClassExW(&window_class);
            let title = Wide::of("the window of the test");
            let class = Wide::of("ahk-rs-test-window");
            let window = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                320,
                240,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                module,
                std::ptr::null(),
            );
            let button_class = Wide::of("BUTTON");
            let button_text = Wide::of("the text of the button");
            let button = CreateWindowExW(
                0,
                button_class.as_ptr(),
                button_text.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                10,
                10,
                120,
                30,
                window,
                std::ptr::null_mut(),
                module,
                std::ptr::null(),
            );
            ShowWindow(window, SW_SHOW);
            (window, button)
        }
    }

    #[test]
    fn the_text_of_a_control_is_the_text_of_it() {
        let (window, button) = a_window_with_a_button();
        let found = the_control_of(window, "Button1").expect("the button is there");
        assert_eq!(found, button);
        assert_eq!(the_text_of(button), "the text of the button");
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_text_of_a_control_is_written_and_read_back() {
        let (window, button) = a_window_with_a_button();
        let written = control_set_text(window, "Button1", "another text");
        assert_eq!(written, ControlEdit::Done);
        assert_eq!(the_text_of(button), "another text");
        let read = control_text(window, "Button1");
        assert_eq!(read, ControlAnswer::Found("another text".to_string()));
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn a_control_that_is_not_there_is_not_there() {
        let (window, _button) = a_window_with_a_button();
        assert_eq!(control_text(window, "Edit1"), ControlAnswer::NoSuchControl);
        assert_eq!(control_visible(window, "Edit1"), ControlFlag::NoSuchControl);
        assert_eq!(control_enabled(window, "Edit1"), ControlFlag::NoSuchControl);
        assert_eq!(control_click(window, "Edit1"), ControlEdit::NoSuchControl);
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_flags_of_a_control_are_the_flags_of_it() {
        let (window, button) = a_window_with_a_button();
        assert_eq!(control_visible(window, "Button1"), ControlFlag::Yes);
        assert_eq!(control_enabled(window, "Button1"), ControlFlag::Yes);
        let _ = button;
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn the_name_of_a_control_is_the_class_and_the_number_of_it() {
        assert_eq!(
            the_parts_of_a_control("Edit1"),
            ("Edit".to_string(), None, 1)
        );
        assert_eq!(
            the_parts_of_a_control("Edit12"),
            ("Edit".to_string(), None, 12)
        );
        assert_eq!(
            the_parts_of_a_control("Edit1.the text"),
            ("Edit".to_string(), Some("the text".to_string()), 1)
        );
        assert_eq!(
            the_parts_of_a_control("Button"),
            ("Button".to_string(), None, 1)
        );
    }
}
