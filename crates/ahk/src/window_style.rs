//! The style of a window, the bits of it and the text that carries it.
//!
//! A window style is a number of Windows: the `WS_*` bits say how a window is
//! decorated and what it may do, and the extended style is the `WS_EX_*` of it.
//! The two are read with `GetWindowLong` and written with `SetWindowLong`, and
//! the reading of them is the same question with the same answer.
//!
//! The bits live here, and not in the backend of Windows, because the X11
//! backend needs them too: the tree maps the attributes of a window of the X11
//! onto these bits (`docs/window-style-mapping.md`), and the mapping is a
//! choice of the tree and not a thing of AutoHotkey, which is Windows only.
//!
//! The text of a style is not a number alone: the v2 reads a prefix that says
//! what to do with the style that is there -- `+` adds, `-` removes, `^` turns
//! the bits around -- and a text without a prefix replaces the style. The
//! measurement is on the binary of v2.0.31,:
//!
//! - `WinSetStyle("")` is "Parameter #1 of WinSetStyle is invalid." with
//!   "empty string" in the `Extra`;
//! - `WinSetStyle("zzz")` is **not** a refusal of the value: the text is not a
//!   number and the v2 reads it as zero, the way its `ATOU` reads it, and the
//!   failure that comes is the one of the window;
//! - `WinSetStyle("+0x10")` adds the bits, `WinSetStyle("-0x10")` takes them
//!   away and `WinSetStyle("^0x10")` turns them around, each one against the
//!   style that is there;
//! - `WinSetTransColor("zzz")` is "Parameter #1 of WinSetTransColor is
//!   invalid." with "zzz" in the `Extra` -- a colour that is not a colour is a
//!   refusal, and the style that is not a number is not.

use alloc::string::String;

/// A window that is on the screen.
pub const WS_VISIBLE: u32 = 0x1000_0000;
/// A window that owns a window of its own inside it.
pub const WS_CHILD: u32 = 0x4000_0000;
/// A window that stands on its own.
pub const WS_POPUP: u32 = 0x8000_0000;
/// A window that is minimized.
pub const WS_MINIMIZE: u32 = 0x2000_0000;
/// A window that is maximized.
pub const WS_MAXIMIZE: u32 = 0x0100_0000;
/// A window that cannot be reached by the keyboard.
pub const WS_DISABLED: u32 = 0x0800_0000;
/// A window that has a title bar.
pub const WS_CAPTION: u32 = 0x00C0_0000;
/// A window that has a border of its own.
pub const WS_BORDER: u32 = 0x0000_800000;
/// A window that has a menu.
pub const WS_SYSMENU: u32 = 0x0008_0000;
/// A window whose frame the mouse can resize.
pub const WS_THICKFRAME: u32 = 0x0004_0000;
/// A window that has a box of controls around it.
pub const WS_GROUP: u32 = 0x0002_0000;
/// A window that the keyboard walks into.
pub const WS_TABSTOP: u32 = 0x0001_0000;
/// A window that has a button of minimization.
pub const WS_MINIMIZEBOX: u32 = 0x0002_0000;
/// A window that has a button of maximization.
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
/// A window that has a scroll bar of the side.
pub const WS_VSCROLL: u32 = 0x0020_0000;
/// A window that has a scroll bar of the bottom.
pub const WS_HSCROLL: u32 = 0x0010_0000;

/// A window that is above the others.
pub const WS_EX_TOPMOST: u32 = 0x0000_0008;
/// A window that lets the window under it through.
pub const WS_EX_TRANSPARENT: u32 = 0x0000_0020;
/// A window that is not in the task bar and has no icon.
pub const WS_EX_TOOLWINDOW: u32 = 0x0000_0080;
/// A window that has a border of the system.
pub const WS_EX_WINDOWEDGE: u32 = 0x0000_0100;
/// A window that has a border of the client.
pub const WS_EX_CLIENTEDGE: u32 = 0x0000_0200;
/// A window that has a question mark.
pub const WS_EX_CONTEXTHELP: u32 = 0x0000_0400;
/// A window whose layout goes from right to left.
pub const WS_EX_RTLREADING: u32 = 0x0000_2000;
/// A window that owns the controls it holds.
pub const WS_EX_CONTROLPARENT: u32 = 0x0001_0000;
/// A window that is a window of its own in the task bar.
pub const WS_EX_APPWINDOW: u32 = 0x0004_0000;
/// A window that lets the screen through it.
pub const WS_EX_LAYERED: u32 = 0x0008_0000;
/// A window that does not take the focus.
pub const WS_EX_NOACTIVATE: u32 = 0x0800_0000;

/// What a text of a style asks of the style that is there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleChange {
    /// The style becomes this one.
    Replace(u32),
    /// These bits are added to the style.
    Add(u32),
    /// These bits are taken away from the style.
    Remove(u32),
    /// These bits are turned around in the style.
    Toggle(u32),
}

impl StyleChange {
    /// The style that the change makes, given the style that is there.
    pub fn the_style_of(&self, current: u32) -> u32 {
        match self {
            StyleChange::Replace(style) => *style,
            StyleChange::Add(bits) => current | bits,
            StyleChange::Remove(bits) => current & !bits,
            StyleChange::Toggle(bits) => current ^ bits,
        }
    }
}

/// The change of the style that a text asks for.
///
/// `None` is a text that is empty, and the v2 refuses it: "Parameter #1 of
/// WinSetStyle is invalid." with "empty string" in the `Extra`.
///
/// The number of the text is read the way the v2 reads it with its `ATOU`: a
/// hexadecimal number when it starts with "0x", a decimal one otherwise, and
/// **zero** when it is not a number at all -- the v2 does not refuse a style
/// that is not a number, it reads it as zero.
pub fn the_style_change_of_a_text(text: &str) -> Option<StyleChange> {
    if text.is_empty() {
        return None;
    }
    let (change, digits) = match text.as_bytes()[0] {
        b'+' => (StyleChange::Add as fn(u32) -> StyleChange, &text[1..]),
        b'-' => (StyleChange::Remove as fn(u32) -> StyleChange, &text[1..]),
        b'^' => (StyleChange::Toggle as fn(u32) -> StyleChange, &text[1..]),
        _ => (StyleChange::Replace as fn(u32) -> StyleChange, text),
    };
    Some(change(the_unsigned_of_a_text(digits)))
}

/// The number that a text holds, the way the `ATOU` of the v2 holds it.
///
/// A hexadecimal number when the text starts with "0x", a decimal one
/// otherwise, and zero when the text holds no number.
pub fn the_unsigned_of_a_text(text: &str) -> u32 {
    let text = text.trim();
    if text.is_empty() {
        return 0;
    }
    // The `ATOU` of the v2 reads a hexadecimal number only when the text
    // says so with its "0x"; every other number is a decimal one, and "16"
    // is sixteen and not the number of the hexadecimal "16".
    if let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            // A number that does not fit in the thirty-two bits is the largest one
            // that does: the style of a window is thirty-two bits wide.
            return u32::from_str_radix(digits, 16).unwrap_or(u32::MAX);
        }
        return 0;
    }
    // The `ATOU` of the v2 reads a decimal number, and it stops at the first
    // character that is not one: "16px" is sixteen.
    let digits: String = text
        .chars()
        .take_while(|digit| digit.is_ascii_digit())
        .collect();
    digits.parse::<u32>().unwrap_or(0)
}

/// The colour that a text holds, in the order of the bits of the v2.
///
/// The v2 reads a colour as a name of the seventeen of HTML, as a
/// hexadecimal number, or as the word "Off" -- and the colour of a window is
/// written as "0xRRGGBB" in the order of the bits of the *red*, the green and
/// the blue, while the machine keeps it in the order of the blue, the green
/// and the red.
///
/// `None` is a text that holds no colour, and the v2 refuses it: "Parameter
/// #1 of WinSetTransColor is invalid." with the text in the `Extra`.
pub fn the_colour_of_a_text(text: &str) -> Option<u32> {
    let text = text.trim();
    if text.is_empty() {
        return Some(COLOUR_NONE);
    }
    if let Some(colour) = the_html_colour(text) {
        return Some(colour);
    }
    // The `ColorToBGR` of the v2 reads the number as hexadecimal, always, and
    // the "0x" of it is optional: "FF00FF" and "0xFF00FF" are the same
    // colour, and "65536" is the colour 0x65536 and not the number 65536.
    let digits = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    // The value that does not fit in the twenty-four bits of a colour keeps
    // the twenty-four of it: the alpha of "0x80FF0000" is dropped, the way
    // the `rgb_to_bgr` of the v2 drops it.
    u32::from_str_radix(digits, 16)
        .ok()
        .map(|colour| colour & 0x0000_FFFFFF)
}

/// The colour that no window lets through: the one of "Off".
///
/// It is the `CLR_NONE` of the v2, and it is not a colour: it says that the
/// window has no colour key at all.
pub const COLOUR_NONE: u32 = 0x0200_0000;

/// The seventeen colours of HTML that the v2 accepts, in the order of the
/// bits of the tree: the red first, the green, the blue.
///
/// The table of the v2 is in the order of the machine -- the blue, the green
/// and the red -- and the `bgr_to_rgb` of it is what turns it into this one.
/// The measurement is on the binary of v2.0.31, and the table is
/// the one of the `ColorNameToBGR` of v2.0.31
/// (`source/util.cpp:1729-1752`).
pub fn the_html_colour(name: &str) -> Option<u32> {
    let colour = match name.to_ascii_lowercase().as_str() {
        "black" => 0x000000,
        "silver" => 0xC0C0C0,
        "gray" => 0x808080,
        "white" => 0xFFFFFF,
        "maroon" => 0x800000,
        "red" => 0xFF0000,
        "purple" => 0x800080,
        "fuchsia" => 0xFF00FF,
        "green" => 0x008000,
        "lime" => 0x00FF00,
        "olive" => 0x808000,
        "yellow" => 0xFFFF00,
        "navy" => 0x000080,
        "blue" => 0x0000FF,
        "teal" => 0x008080,
        "aqua" => 0x00FFFF,
        _ => return None,
    };
    Some(colour)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_empty_text_is_no_change() {
        assert_eq!(the_style_change_of_a_text(""), None);
    }

    #[test]
    fn the_text_without_a_prefix_replaces_the_style() {
        assert_eq!(
            the_style_change_of_a_text("0x10"),
            Some(StyleChange::Replace(0x10))
        );
        assert_eq!(
            the_style_change_of_a_text("16"),
            Some(StyleChange::Replace(16))
        );
    }

    #[test]
    fn the_prefix_says_what_to_do_with_the_style_that_is_there() {
        assert_eq!(
            the_style_change_of_a_text("+0x10"),
            Some(StyleChange::Add(0x10))
        );
        assert_eq!(
            the_style_change_of_a_text("-0x10"),
            Some(StyleChange::Remove(0x10))
        );
        assert_eq!(
            the_style_change_of_a_text("^0x10"),
            Some(StyleChange::Toggle(0x10))
        );
        assert_eq!(
            StyleChange::Add(0x10).the_style_of(0x20),
            0x30,
            "the bits are added"
        );
        assert_eq!(
            StyleChange::Remove(0x10).the_style_of(0x30),
            0x20,
            "the bits are taken away"
        );
        assert_eq!(
            StyleChange::Toggle(0x10).the_style_of(0x30),
            0x20,
            "the bits are turned around"
        );
        assert_eq!(
            StyleChange::Replace(0x10).the_style_of(0x20),
            0x10,
            "the style is replaced"
        );
    }

    #[test]
    fn a_text_that_holds_no_number_is_zero() {
        // Measured on the binary of v2.0.31: `WinSetStyle("zzz")`
        // is not a refusal of the value.
        assert_eq!(the_unsigned_of_a_text("zzz"), 0);
        assert_eq!(the_unsigned_of_a_text(""), 0);
        assert_eq!(the_unsigned_of_a_text("0x1F"), 31);
        assert_eq!(the_unsigned_of_a_text("16"), 16);
        assert_eq!(the_unsigned_of_a_text("16px"), 16);
    }

    #[test]
    fn the_colours_of_html_are_the_seventeen_of_the_v2() {
        // The table of the v2 is in the order of the machine, and this one is
        // in the order of the tree: the red, the green, the blue.
        assert_eq!(the_colour_of_a_text("Red"), Some(0x00FF_0000));
        assert_eq!(the_colour_of_a_text("blue"), Some(0x0000_00FF));
        assert_eq!(the_colour_of_a_text("White"), Some(0x0000_FFFFFF));
        assert_eq!(the_colour_of_a_text("Green"), Some(0x0000_8000));
    }

    #[test]
    fn a_colour_is_read_as_hexadecimal() {
        // Measured on the binary of v2.0.31: `WinSetTransColor
        // "FF00FF"` and then `WinGetTransColor` answer "0xFF00FF" -- the
        // colour is read as the hexadecimal number it is written as, and
        // the "0x" of it is optional.
        assert_eq!(the_colour_of_a_text("FF00FF"), Some(0x00FF_00FF));
        assert_eq!(the_colour_of_a_text("0xFF00FF"), Some(0x00FF_00FF));
        // The colour of the v2 is always read as hexadecimal: "65536" is the
        // colour 0x65536 and not the number 65536.
        assert_eq!(the_colour_of_a_text("65536"), Some(0x0006_5536));
        assert_eq!(the_colour_of_a_text("zzz"), None);
        assert_eq!(the_colour_of_a_text(""), Some(COLOUR_NONE));
    }
}
