//! The controls of the windows of the script, against the machine itself.
//!
//! A control of the v1 is a window of the machine, and the class of it is
//! the class of that window. The cases of this file are the classes of
//! the controls, and every one of them was measured on the binary of the
//! v1.1.37.02: the probe is in `gui-kinds.ahk`, and the classes of the
//! controls that it makes are read from the machine.

#![cfg(windows)]

use ahk_box::windows::gui;
use windows_sys::Win32::Graphics::Gdi::{
    GetObjectW, LF_FACESIZE, LOGFONTW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_GETFONT};

/// The name and the height of the font of a control.
///
/// The font of a control is asked to the machine, and the machine answers
/// the handle of it; the name and the height are read out of it.
fn the_font_of(control: &ahk_box::windows::gui::TheControl) -> (String, i32) {
    let font = unsafe { SendMessageW(control.handle, WM_GETFONT, 0, 0) };
    assert_ne!(font, 0, "the control has no font");
    let mut logfont: LOGFONTW = unsafe { std::mem::zeroed() };
    let read = unsafe {
        GetObjectW(
            font as windows_sys::Win32::Graphics::Gdi::HGDIOBJ,
            std::mem::size_of::<LOGFONTW>() as i32,
            &mut logfont as *mut LOGFONTW as *mut core::ffi::c_void,
        )
    };
    assert_ne!(read, 0, "the font is not read");
    let name = String::from_utf16_lossy(
        &logfont.lfFaceName[..logfont
            .lfFaceName
            .iter()
            .position(|letter| *letter == 0)
            .unwrap_or(LF_FACESIZE as usize)],
    );
    (name, logfont.lfHeight)
}


/// The class of a control of the kind that is named.
///
/// The window of the script is made, the control is added to it, and the
/// class of the control is read from the machine.
fn the_class_of(kind: &str, text: &str) -> String {
    gui::the_common_controls_are_taken();
    let mut window = gui::open_of_the_v1("the probe of the classes").expect("the window is made");
    let (number, _) = gui::add(&mut window, kind, text).expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    gui::the_class_of(control)
}

#[test]
fn the_link_of_the_v1_is_a_control_of_the_class_syslink() {
    // Measured on the binary of v1.1.37.02: the `Gui, Add, Link` makes a
    // control of the class "SysLink" -- the class of the window of the
    // comctl32, and not the name of the library of it. The tree asked for
    // "SysLink32", and the machine has no class of that name: the control
    // was not made, and the script died on it.
    assert_eq!(
        the_class_of("link", "<a href=\"https://example.com/\">a link</a>"),
        "SysLink"
    );
}

#[test]
fn the_font_of_a_control_follows_the_options_of_the_gui_font() {
    // Measured on the binary of v1.1.37.02: the `Gui, Font` builds the
    // font out of its options -- the name is the one of the second
    // parameter of the command, and the size and the weight that the
    // options do not name are the ones of the window -- and the name and
    // the size that the command does not name are the ones of the default
    // font of the machine.
    gui::the_common_controls_are_taken();
    let mut window = gui::open_of_the_v1("the probe of the fonts").expect("the window is made");

    // the control of a window that has no `Gui, Font`: the default font
    let (number, _) = gui::add(&mut window, "text", "one").expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    assert_eq!(the_font_of(control), ("MS Shell Dlg".to_string(), -11));

    // the `Gui, Font` with no options: the default font again
    gui::the_font_is_reset(&mut window);
    let (number, _) = gui::add(&mut window, "text", "two").expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    assert_eq!(the_font_of(control), ("MS Shell Dlg".to_string(), -11));

    // the `Gui, Font, Bold`: the default font with the weight of it
    let font = gui::the_font_of_the_v1_from(None, "Bold", "");
    gui::the_font_is_written(&mut window, font.expect("the font of the options"));
    let (number, _) = gui::add(&mut window, "text", "three").expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    assert_eq!(the_font_of(control), ("MS Shell Dlg".to_string(), -11));

    // the `Gui, Font, s19`: the default name with the size of the options
    let font = gui::the_font_of_the_v1_from(None, "s19", "");
    gui::the_font_is_written(&mut window, font.expect("the font of the options"));
    let (number, _) = gui::add(&mut window, "text", "four").expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    assert_eq!(the_font_of(control), ("MS Shell Dlg".to_string(), -25));

    // the `Gui, Font, s19 Bold, Arial`: the name of the second parameter
    let font = gui::the_font_of_the_v1_from(None, "s19 Bold, Arial", "Arial");
    gui::the_font_is_written(&mut window, font.expect("the font of the options"));
    let (number, _) = gui::add(&mut window, "text", "five").expect("the control is made");
    let control = gui::the_control(&window, number).expect("the control is there");
    assert_eq!(the_font_of(control), ("Arial".to_string(), -25));
}

#[test]
fn the_kinds_of_the_controls_of_the_v1_have_the_classes_of_them() {
    // Measured on the binary of v1.1.37.02: the probe makes one control
    // of every kind, and the class of each one of them is read from the
    // machine.
    for (kind, text, class) in [
        ("button", "a button", "Button"),
        ("checkbox", "a check box", "Button"),
        ("radio", "a radio", "Button"),
        ("groupbox", "a group", "Button"),
        ("text", "a text", "Static"),
        ("edit", "an edit", "Edit"),
        ("listbox", "", "ListBox"),
        ("combobox", "", "ComboBox"),
        ("ddl", "", "ComboBox"),
        ("progress", "", "msctls_progress32"),
        ("slider", "", "msctls_trackbar32"),
        ("updown", "", "msctls_updown32"),
        ("datetime", "", "SysDateTimePick32"),
        ("monthcal", "", "SysMonthCal32"),
        ("hotkey", "", "msctls_hotkey32"),
        ("tab", "one|two", "SysTabControl32"),
        ("statusbar", "", "msctls_statusbar32"),
        ("treeview", "", "SysTreeView32"),
        ("listview", "", "SysListView32"),
    ] {
        assert_eq!(the_class_of(kind, text), class, "the class of the {kind}");
    }
}
