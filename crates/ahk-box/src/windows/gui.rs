//! The windows of the script.
//!
//! A window of the script is a window of the machine: the classes of
//! `user32` and of `comctl32` make it, and the controls of it are the
//! common controls of the machine. The number of a control is the order
//! of its creation, the way the tree numbers them.
//!
//! The class of a control is the name of the v2 -- "Button", "Edit",
//! "ListBox" -- and the machine has a class for each of them.

use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{ABC, CLIP_DEFAULT_PRECIS, COLOR_3DFACE, CreateFontW, CreateSolidBrush, DEFAULT_CHARSET, DEFAULT_GUI_FONT, DEFAULT_QUALITY, DT_CALCRECT, DT_EDITCONTROL, DT_EXPANDTABS, DT_NOPREFIX, DT_WORDBREAK, DeleteObject, DrawTextW, FF_DONTCARE, FillRect, GetCharABCWidthsW, GetClipBox, GetDC, GetDeviceCaps, GetObjectW, GetStockObject, GetTextMetricsW, HDC, HGDIOBJ, InvalidateRect, LF_FACESIZE, LOGFONTW, LOGPIXELSY, MapWindowPoints, OUT_TT_PRECIS, RDW_ERASE, RDW_INVALIDATE, RDW_UPDATENOW, RedrawWindow, ReleaseDC, SelectObject, TEXTMETRICW};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::GetCurrentThreadId;
use windows_sys::Win32::UI::Controls::{EM_GETLINECOUNT, EM_GETSEL, LVS_REPORT};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{EnableWindow, SetFocus};
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AdjustWindowRectEx, BM_GETCHECK, BM_SETCHECK, GetSystemMetrics,
    CB_ADDSTRING, CB_DELETESTRING, CB_FINDSTRINGEXACT, CB_GETCOUNT, CB_GETCURSEL, CB_GETLBTEXT,
    CB_GETLBTEXTLEN, CB_SETCURSEL, CB_SHOWDROPDOWN, CBS_DROPDOWNLIST, CreateWindowExW,
    DefWindowProcW, DestroyWindow, ES_MULTILINE, GUITHREADINFO, GetClassNameW, GetCursorPos,
    GetGUIThreadInfo, GetMenu, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW,
    GetWindowTextW,
    GWL_EXSTYLE, GWL_STYLE, GWLP_HWNDPARENT, HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOP, HWND_TOPMOST,
    LB_ADDSTRING, LB_GETCOUNT, LB_GETCURSEL, LB_RESETCONTENT, LB_SETCURSEL,
    GetClientRect, MINMAXINFO, RegisterClassExW,
    SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SWP_HIDEWINDOW, SWP_NOACTIVATE,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
    SendMessageW, SetForegroundWindow, SetParent, SetWindowLongPtrW, SetWindowPos, SetWindowTextW,
    ShowWindow, SM_CXBORDER, SM_CXEDGE, SM_CXMENUCHECK, SM_CYBORDER, WM_PASTE, WM_SETFONT, WM_SIZE,
    WM_SETTEXT, WNDCLASSEXW, WS_BORDER, WS_CAPTION, WS_CHILD,
    IMAGE_ICON, WS_EX_CLIENTEDGE, WS_EX_TOOLWINDOW, WS_EX_WINDOWEDGE, WS_GROUP, WS_MAXIMIZEBOX,
    WS_MINIMIZEBOX,
    WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SYSMENU, WS_TABSTOP, WS_THICKFRAME, WS_VISIBLE,
};

use ahk::desktop::{TheCoordinate, ThePlaceOfTheV1, TheSize, TheStyleOfTheV1};

use super::sys::Wide;

// The messages of the list box of the machine.
//
// The crate of the machine names most of them, and these three are the
// ones that it does not: the text of an item and the length of it.
const LB_GETTEXT: u32 = 0x0189;
const LB_GETTEXTLEN: u32 = 0x018A;

// The message of the tab control of the machine that chooses the tab.
const TCM_SETCURSEL: u32 = 0x130C;
const TCM_DELETEALLITEMS: u32 = 0x1309;
const TCM_ADJUSTRECT: u32 = 0x1328;
const CB_RESETCONTENT: u32 = 0x014B;
const PBM_SETPOS: u32 = 0x0402;
const PBM_DELTAPOS: u32 = 0x0403;
const TBM_SETPOS: u32 = 0x0405;
const UDM_SETPOS32: u32 = 0x0471;

// The styles of a picture control, and the message that puts the image
// in it (`STM_SETIMAGE` = 0x0172 = 370 decimal; `STM_SETICON` = 0x0170).
const SS_BITMAP: u32 = 0x000E;
const SS_ICON: u32 = 0x0003;
const STM_SETIMAGE: u32 = 0x0172;

// The messages of the machine that give the position of a control.
const PBM_GETPOS: u32 = 0x0408;
const UDM_GETPOS: u32 = 0x046D;
const TBM_GETPOS: u32 = 0x0400;

/// The message that asks the machine for the area that is not the client
/// area of a window.
const WM_NCCALCSIZE: u32 = 0x0083;

// The messages of the list box and of the combo box that the tree does
// not have: the search of the leading part of the text of an item.
const LB_FINDSTRING: u32 = 0x018F;
const CB_FINDSTRING: u32 = 0x014C;

/// A control of a window of the script.
///
/// The handle is the window of the control, and the kind is the name of
/// the v2 that the script gave it.
#[derive(Clone, Debug)]
pub struct TheControl {
    /// The window of the control.
    pub handle: HWND,
    /// The kind of the control, the name of the v2.
    pub kind: String,
    /// The class of the control, the name of the machine.
    pub class: String,
    /// The name of the variable that the control is bound to.
    ///
    /// The v1 binds a control to a variable with the `v` option of the
    /// `Gui, Add`, and the `GuiControl` and the `GuiControlGet` name the
    /// control by that name.
    pub name: String,
    /// The text that the control carries.
    pub text: String,
    /// The number of the icon that the options of the control asked for.
    ///
    /// The "Icon" of the options of the `Gui, Add` of the v1 names the
    /// icon of a file of icons that a picture control shows.
    pub icon: Option<u32>,
}

/// A window of the script.
#[derive(Debug, Default)]
pub struct TheWindow {
    /// The window of the machine.
    pub handle: HWND,
    /// The controls of the window, in the order that they were made.
    pub controls: Vec<TheControl>,
    /// The name that the v1 gives to the window.
    ///
    /// The v1 names its windows, and the name is what the `Gui, Name:`
    /// and the `GuiControl, Name:` use to find the window. The window
    /// that has no name is the one of the number one.
    pub name: String,
    /// The colour of the background of the window.
    ///
    /// The `Gui, Color` of the v1 writes the colour of the window and
    /// the colour of the controls of it.
    pub background: Option<u32>,
    /// The margin of the window, the space that the controls of it keep
    /// from its border.
    pub margin: (i64, i64),
    /// The place of the section of the window.
    ///
    /// The v1 keeps the place of a section, and the `xs` and the `ys` of
    /// the options of a control are the place of it. The first control of
    /// a window always begins a section.
    pub section: Option<(i64, i64)>,
    /// The number of the control that began the section of the window.
    pub section_at: Option<usize>,
    /// The font of the controls that are made after it.
    pub font: Option<TheFont>,
    /// The font of the machine, and the handle of it.
    ///
    /// The v1 makes the font of the controls with the `CreateFont` (source
    /// `script_gui.cpp:7883` of v1.1.37.02) and gives it to every control
    /// with the `WM_SETFONT` (`script_gui.cpp:3027`). The font is the one
    /// of the window too, and the caption of the window is drawn with it.
    pub font_handle: Option<isize>,
    /// Whether the window has a bar of a menu.
    pub menu_bar: bool,
    /// The bar of the menu of the window.
    ///
    /// The bar belongs to the window that carries it: the machine
    /// destroys the bar of a window with the window, and the bar that
    /// is destroyed is the bar of a window that has none.
    pub menu: Option<super::menu::TheMenuBar>,
    /// The name of the menu of the script that is the bar of the window.
    ///
    /// The bar of the menu of the v1 is the menu of the script itself:
    /// the items that the script adds to the menu after the bar is
    /// attached are in the bar of the window.
    pub menu_name: String,
    /// The separator of the contents of the controls of the window.
    ///
    /// The v1 writes it with the `+Delimiter` option of the window, and
    /// it is the pipe when the script does not write it.
    pub delimiter: char,
    /// The prefix of the special labels of the window.
    ///
    /// The v1 writes it with the `+Label` option of the window.
    pub label: String,
    /// The smallest size of the window, when the script wrote one.
    pub min_size: Option<(i64, i64)>,
    /// The largest size of the window, when the script wrote one.
    pub max_size: Option<(i64, i64)>,
    /// Whether the window was already shown.
    ///
    /// The v1 gives the window the size of the controls that it holds the
    /// first time that it shows it, and it keeps the size of it after
    /// that. The source of the v1 (`script_gui.cpp:6763-6786`) asks
    /// `mGuiShowHasNeverBeenDone` before it reads the size of the client
    /// area.
    pub shown: bool,
    /// The HWND of the most recently added tab control, if any.
    pub current_tab_control: Option<isize>,
    /// The 1-based tab page that newly added controls belong to (`Gui, Tab, N`),
    /// or `None` after `Gui, Tab` with no argument.
    pub current_tab_page: Option<usize>,
    /// The 1-based initially selected tab page of `current_tab_control`.
    pub active_tab_page: usize,
    /// True right after `Gui, Add, Tab*` or `Gui, Tab, N` so the first control
    /// on the tab page positions relative to the tab's display area.
    pub tab_just_switched: bool,
    /// Control HWND to use as the `previous` layout reference for the next
    /// added control (set to the tab control when `Gui, Tab` ends a tab group).
    pub layout_anchor_control: Option<isize>,
}

/// The font of the controls of a window.
///
/// The v1 writes the font of a window with the `Gui, Font`, and the
/// controls that are made after it carry it.
#[derive(Clone, Debug, Default)]
pub struct TheFont {
    /// The name of the typeface.
    pub name: String,
    /// The size of it, in points.
    pub size: i64,
    /// Whether the weight of it is the bold one.
    pub bold: bool,
    /// Whether the weight of it is the italic one.
    pub italic: bool,
    /// Whether the text of it is struck through.
    pub strike: bool,
    /// Whether the text of it is underlined.
    pub underline: bool,
    /// The colour of the text of it.
    pub color: Option<u32>,
}

/// The class of the window of the script.
/// The class of the window of the script.
///
/// Measured on the binary of v1.1.37.02: the window of the
/// script of the real interpreter has the class "AutoHotkeyGUI", and the
/// scripts of the v1 name it -- `WinGet, id, ID, ahk_class AutoHotkeyGUI`
/// is how a script finds its own window.
const THE_CLASS: &str = "AutoHotkeyGUI";

/// The class of the machine of every kind of control, and the style of
/// it that the kind asks for.
///
/// The kind is the name of the v2: "Button", "CheckBox", "Edit" and the
/// others. The name that is not one of them is a kind that the machine
/// does not have.
///
/// The styles are the ones of the v1 (source `script_gui.cpp:2059-2201` of
/// v1.1.37.02): the `WS_TABSTOP` goes to the controls that the v1 marks
/// with it -- the text, the picture, the group, the status bar and the
/// Increase/Decrease field do not have it -- and a radio button is of the
/// type `BS_AUTORADIOBUTTON` with the `BS_NOTIFY`.
///
/// The `font` says whether the control takes the font of the window: the
/// v1 gives the font neither to the picture, nor to the Increase/Decrease
/// field, nor to the cursor, nor to the progress bar (the macro
/// `USES_FONT_AND_TEXT_COLOR`, source `script.h:2672` of v1.1.37.02).
struct TheKindOfAControl {
    /// The class of the machine.
    class: &'static str,
    /// The bits of the style that the kind asks for.
    style: u32,
    /// The bits of the extended style that the kind asks for.
    ///
    /// The v1 gives the extended `WS_EX_CLIENTEDGE` to the controls that
    /// look strange without it (source `script_gui.cpp:2071-2201` of
    /// v1.1.37.02), and the border of the machine is essa borda.
    exstyle: u32,
    /// Whether the control has the `WS_TABSTOP` by default.
    tabstop: bool,
    /// Whether the control begins a group of radio buttons.
    group: bool,
    /// Whether the control takes the font of the window.
    font: bool,
}

/// The styles of the buttons of the machine that the crate of it does not
/// name.
const BS_AUTOCHECKBOX: u32 = 0x0003;
const BS_MULTILINE: u32 = 0x2000;

/// The style of a text control that shows the text of it as it is.
const SS_NOPREFIX: u32 = 0x0000_0080;
const BS_AUTORADIOBUTTON: u32 = 0x0009;
const BS_NOTIFY: u32 = 0x4000;

// The styles of the machine that the crate of it does not name.
//
// They are the styles de omissão dos controlos do v1 (source
// `script_gui.cpp:2071-2201` de v1.1.37.02).
const WS_VSCROLL: u32 = 0x0020_0000;
const LBS_NOTIFY: u32 = 0x0001;
const LBS_USETABSTOPS: u32 = 0x0080;
const CBS_AUTOHSCROLL: u32 = 0x0040;
const CBS_DROPDOWN: u32 = 0x0002;
const TVS_HASBUTTONS: u32 = 0x0001;
const TVS_HASLINES: u32 = 0x0002;
const TVS_LINESATROOT: u32 = 0x0004;
const TVS_SHOWSELALWAYS: u32 = 0x2000;
const LVS_SHOWSELALWAYS: u32 = 0x0008;
const UDS_SETBUDDYINT: u32 = 0x0002;
const UDS_ALIGNRIGHT: u32 = 0x0004;
const UDS_AUTOBUDDY: u32 = 0x0008;
const UDS_ARROWKEYS: u32 = 0x0100;
const DTS_SHORTDATECENTURYFORMAT: u32 = 0x0002;
const PBS_SMOOTH: u32 = 0x0100;
const TCS_MULTILINE: u32 = 0x0020;

fn the_class_of_a_kind(kind: &str) -> Option<TheKindOfAControl> {
    Some(match kind.trim().to_lowercase().as_str() {
        "button" => TheKindOfAControl {
            class: "Button",
            style: 0,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "checkbox" => TheKindOfAControl {
            class: "Button",
            style: BS_AUTOCHECKBOX,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "radio" => TheKindOfAControl {
            class: "Button",
            style: BS_AUTORADIOBUTTON | BS_NOTIFY,
            exstyle: 0,
            tabstop: false,
            group: true,
            font: true,
        },
        "groupbox" => TheKindOfAControl {
            class: "Button",
            style: 0x0007,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: true,
        },
        "text" => TheKindOfAControl {
            class: "Static",
            style: 0,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: true,
        },
        "picture" => TheKindOfAControl {
            class: "Static",
            // The v1 makes the picture without the style of the image: the
            // `SS_BITMAP` and the `SS_ICON` go into the control after it
            // is made, so that it does not try to load the text of it as a
            // resource (source `script_gui.cpp:2339-2341` of v1.1.37.02).
            style: 0x0000,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: false,
        },
        "edit" => TheKindOfAControl {
            class: "Edit",
            style: 0,
            exstyle: WS_EX_CLIENTEDGE,
            tabstop: true,
            group: false,
            font: true,
        },
        "listbox" => TheKindOfAControl {
            class: "ListBox",
            style: WS_VSCROLL | LBS_NOTIFY | LBS_USETABSTOPS,
            exstyle: WS_EX_CLIENTEDGE,
            tabstop: true,
            group: false,
            font: true,
        },
        "combobox" => TheKindOfAControl {
            class: "ComboBox",
            style: WS_VSCROLL | CBS_AUTOHSCROLL | CBS_DROPDOWN,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "ddl" => TheKindOfAControl {
            class: "ComboBox",
            style: WS_VSCROLL | CBS_DROPDOWNLIST as u32,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "progress" => TheKindOfAControl {
            class: "msctls_progress32",
            style: PBS_SMOOTH,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: false,
        },
        "slider" => TheKindOfAControl {
            class: "msctls_trackbar32",
            style: 0,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: false,
        },
        "updown" => TheKindOfAControl {
            class: "msctls_updown32",
            style: UDS_SETBUDDYINT | UDS_ALIGNRIGHT | UDS_AUTOBUDDY | UDS_ARROWKEYS,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: false,
        },
        "datetime" => TheKindOfAControl {
            class: "SysDateTimePick32",
            style: DTS_SHORTDATECENTURYFORMAT,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "monthcal" => TheKindOfAControl {
            class: "SysMonthCal32",
            style: 0,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "hotkey" => TheKindOfAControl {
            class: "msctls_hotkey32",
            style: 0,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "link" => TheKindOfAControl {
            // The class of the link of the v1 is "SysLink": measured on the
            // binary of v1.1.37.02, the control that the `Gui, Add, Link`
            // makes has the class "SysLink" -- the class of the window of
            // the comctl32, and not the name of the library of it. The tree
            // asked for "SysLink32", and the machine has no class of that
            // name: the control was not made, and the script of the panel
            // of the About died on it.
            class: "SysLink",
            style: 0,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "tab" => TheKindOfAControl {
            class: "SysTabControl32",
            style: TCS_MULTILINE,
            exstyle: 0,
            tabstop: true,
            group: false,
            font: true,
        },
        "statusbar" => TheKindOfAControl {
            class: "msctls_statusbar32",
            style: 0,
            exstyle: 0,
            tabstop: false,
            group: false,
            font: true,
        },
        "treeview" => TheKindOfAControl {
            class: "SysTreeView32",
            style: TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS,
            exstyle: WS_EX_CLIENTEDGE,
            tabstop: true,
            group: false,
            font: true,
        },
        "listview" => TheKindOfAControl {
            class: "SysListView32",
            style: LVS_REPORT | LVS_SHOWSELALWAYS,
            exstyle: WS_EX_CLIENTEDGE,
            tabstop: true,
            group: false,
            font: true,
        },
        _ => return None,
    })
}

/// The class of the control of the machine is taken once.
///
/// The common controls of the machine are in the library of them
/// (`comctl32`), and the library says which of them the machine has.
pub fn the_common_controls_are_taken() {
    // The library of the common controls is taken before the classes of
    // it: a program that has no library has the classes of the machine
    // and not the controls of it.
    unsafe {
        use windows_sys::Win32::System::LibraryLoader::LoadLibraryW;
        LoadLibraryW(Wide::of("comctl32.dll").as_ptr());
    }
    use windows_sys::Win32::UI::Controls::{
        ICC_DATE_CLASSES, ICC_LINK_CLASS, ICC_STANDARD_CLASSES, ICC_USEREX_CLASSES,
        ICC_WIN95_CLASSES, INITCOMMONCONTROLSEX, InitCommonControlsEx,
    };
    unsafe {
        let mut controls: INITCOMMONCONTROLSEX = std::mem::zeroed();
        controls.dwSize = std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32;
        controls.dwICC = ICC_WIN95_CLASSES
            | ICC_DATE_CLASSES
            | ICC_USEREX_CLASSES
            | ICC_STANDARD_CLASSES
            | ICC_LINK_CLASS;
        InitCommonControlsEx(&controls);
    }
}

/// The callbacks of the items of the menu bar of a window: the number of
/// each item and the function that it runs.
type TheBarCallbacks = (isize, Vec<(usize, String)>);

/// The callbacks of the items of the menu bar, by the window that has it.
///
/// The bar is set by the `gui_menu_bar` of the desktop, and the
/// procedure of the window reads this table when a `WM_COMMAND` arrives:
/// the click on an item of the bar is a command with the number of the
/// item.
///
/// The table is by window, because two windows may have two bars, and
/// the number of an item is the number of the item of the machine.
static THE_BAR_CALLBACKS: std::sync::Mutex<Vec<TheBarCallbacks>> = std::sync::Mutex::new(Vec::new());

/// A v1 control g-label entry: `(control_hwnd, label, gui_name, control_name, kind)`.
#[derive(Clone, Debug)]
struct ControlCallback {
    control: isize,
    label: String,
    gui_name: String,
    control_name: String,
    kind: String,
}

/// The g-label callbacks of v1 controls, keyed by control HWND.
static THE_CONTROL_CALLBACKS: std::sync::Mutex<Vec<ControlCallback>> =
    std::sync::Mutex::new(Vec::new());

/// Mapping from v1 GUI window HWND to `(gui_name, custom_label_prefix)` for
/// `WM_CLOSE` / `GuiClose` dispatch.
static THE_WINDOW_NAMES: std::sync::Mutex<Vec<(isize, String, String)>> =
    std::sync::Mutex::new(Vec::new());

/// The controls of one tab page: the handle of the control and the page.
type TheControlsOfAPage = Vec<(isize, usize)>;

/// Mapping from SysTabControl32 HWND to the child controls owned by each
/// 1-based tab page: `(tab_hwnd, TheControlsOfAPage)`.
static THE_TAB_CONTROLS: std::sync::Mutex<Vec<(isize, TheControlsOfAPage)>> =
    std::sync::Mutex::new(Vec::new());

/// Register a v1 GUI window's name and optional `+Label` prefix.
pub fn the_window_name_is_set(window: isize, gui_name: &str, label_prefix: &str) {
    if let Ok(mut table) = THE_WINDOW_NAMES.lock() {
        table.retain(|(other, _, _)| *other != window);
        table.push((window, gui_name.to_string(), label_prefix.to_string()));
    }
}

/// Register a v1 control's g-label callback.
fn the_control_callback_is_set(
    control: isize,
    label: &str,
    gui_name: &str,
    control_name: &str,
    kind: &str,
) {
    if let Ok(mut table) = THE_CONTROL_CALLBACKS.lock() {
        table.retain(|entry| entry.control != control);
        table.push(ControlCallback {
            control,
            label: label.to_string(),
            gui_name: gui_name.to_string(),
            control_name: control_name.to_string(),
            kind: kind.to_string(),
        });
    }
}

/// Record a control as belonging to `page` (1-based) of `tab_hwnd`.
fn the_tab_child_is_added(tab_hwnd: isize, control_hwnd: isize, page: usize) {
    if let Ok(mut table) = THE_TAB_CONTROLS.lock() {
        if let Some((_, list)) = table.iter_mut().find(|(h, _)| *h == tab_hwnd) {
            list.push((control_hwnd, page));
        } else {
            table.push((tab_hwnd, vec![(control_hwnd, page)]));
        }
    }
}

/// Switch visible child controls for a `SysTabControl32` when its active tab changes.
fn the_tab_selection_changed(tab_hwnd: HWND) {
    const TCM_GETCURSEL: u32 = 0x130B;
    let selected = unsafe { SendMessageW(tab_hwnd, TCM_GETCURSEL, 0, 0) };
    if selected < 0 {
        return;
    }
    let active_page = (selected as usize) + 1;
    let children = THE_TAB_CONTROLS.lock().ok().and_then(|table| {
        table
            .iter()
            .find(|(h, _)| *h == tab_hwnd as isize)
            .map(|(_, list)| list.clone())
    });
    if let Some(children) = children {
        for (ctrl, page) in children {
            let cmd = if page == active_page { SW_SHOW } else { SW_HIDE };
            unsafe { ShowWindow(ctrl as HWND, cmd) };
        }
    }
}

/// The events of the window of the script that the procedure queues.
///
/// The script reads them between its statements, the way it reads the
/// messages of `OnMessage`: a click on an item of the bar is the
/// `v1menu:<callback>` the runtime answers.
static THE_GUI_EVENTS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// Set the callbacks of the menu bar (the table the procedure reads).
pub fn the_bar_callbacks_are_set(window: isize, items: Vec<(usize, String)>) {
    if let Ok(mut table) = THE_BAR_CALLBACKS.lock() {
        table.retain(|(other, _)| *other != window);
        table.push((window, items));
    }
}

/// Take the events of the window of the script, whole.
///
/// The messages that are waiting are dispatched first: the window of
/// the script lives on the thread of the script, and its procedure
/// runs only when the thread pumps the messages. The click on an item
/// of the bar is a message, and the event of it is queued here.
pub fn the_events_of_the_gui() -> Vec<String> {
    unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, PeekMessageW, TranslateMessage, PM_REMOVE,
        };
        let mut message: windows_sys::Win32::UI::WindowsAndMessaging::MSG =
            core::mem::zeroed();
        while PeekMessageW(&mut message, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
            if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
                eprintln!(
                    "the message 0x{:X} of the window 0x{:X}",
                    message.message,
                    message.hwnd as isize
                );
            }
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    match THE_GUI_EVENTS.lock() {
        Ok(mut events) => core::mem::take(&mut *events),
        Err(_) => Vec::new(),
    }
}

/// The procedure of the window of the script.
///
/// The click on an item of the menu bar is a `WM_COMMAND` with the
/// number of the item: the procedure queues the callback of it for the
/// script to run. Every other message the machine answers.
/// Whether a menu of the script is on the screen.
///
/// The v1: "WM_ENTERMENULOOP: One of the MENU BAR menus has been
/// displayed, and then we know the user is still in the menu bar, even
/// moving to different menus and/or menu items, until WM_EXITMENULOOP is
/// received" (`script_gui.cpp:9046-9049` of v1.1.37.02).
///
/// The timed subroutines of the script do not run while the menu is
/// visible: the menu would be sluggish, and the script would change the
/// menu while the user is in it.
static THE_MENU_IS_VISIBLE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Whether a menu of the script is on the screen.
pub fn the_menu_is_visible() -> bool {
    THE_MENU_IS_VISIBLE.load(std::sync::atomic::Ordering::Relaxed)
}

unsafe extern "system" fn the_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    const WM_COMMAND: u32 = 0x0111;
    // The window goes away: the script asked for it, or the machine took
    // it away.
    const WM_DESTROY: u32 = 0x0002;
    const WM_CLOSE: u32 = 0x0010;
    if (message == WM_DESTROY || message == WM_CLOSE)
        && std::env::var_os("AHK_DEBUG_CALLBACK").is_some()
    {
        eprintln!(
            "the window 0x{:X} got the message 0x{:X} and goes away",
            window as isize,
            message
        );
    }
    // The menu of the window is on the screen: the script does not run
    // its timed subroutines while the user is in the menu.
    //
    // The v1: "case WM_ENTERMENULOOP: if (!g_MenuIsVisible)
    // g_MenuIsVisible = MENU_TYPE_BAR; break; case WM_EXITMENULOOP:
    // g_MenuIsVisible = MENU_TYPE_NONE; break;" (`script_gui.cpp:9048-9053`
    // of v1.1.37.02).
    const WM_ENTERMENULOOP: u32 = 529;
    const WM_EXITMENULOOP: u32 = 530;
    if message == WM_ENTERMENULOOP {
        if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
            eprintln!("the window 0x{:X} entered the menu loop", window as isize);
        }
        THE_MENU_IS_VISIBLE.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    if message == WM_EXITMENULOOP {
        THE_MENU_IS_VISIBLE.store(false, std::sync::atomic::Ordering::Relaxed);
    }
    // The size that a window of the script may not go under, and the one
    // that it may not go over: the machine asks for it when it resizes
    // the window.
    const WM_GETMINMAXINFO: u32 = 0x0024;
    // A message that the script watches with `OnMessage`: the procedure of
    // the window of the reference gives the message to the monitor of the
    // script before it does anything else with it, and the monitor runs in
    // a thread of its own (`source/script_gui.cpp:7930` of the v1).
    //
    // The tree queues the message, and the runtime calls the handler of it
    // between the statements of the script: the window of the script lives
    // on the thread of the script, and the handler cannot run there.
    if super::hotkeys::the_message_is_watched(message) {
        if let Ok(mut events) = THE_GUI_EVENTS.lock() {
            events.push(format!(
                "message:{message}:{wparam}:{lparam}:{}",
                window as isize
            ));
        }
    }
    // The size of the window: the machine asks for the size that the
    // window may have, and the answer of it keeps the window inside the
    // "MinSize" and the "MaxSize" that the script gave to it.
    if message == WM_GETMINMAXINFO {
        the_size_is_written_to_the_machine(window, lparam);
    }
    // The background of the window: the machine asks the window to erase
    // it, and the window paints it with the colour that the script gave
    // to it.
    //
    // Measured on the binary of v1.1.37.02: the `Gui, Color`
    // of the v1 paints the window of the script, and the buttons that are
    // in it keep the image of theirs.
    const WM_ERASEBKGND: u32 = 0x0014;
    if message == WM_ERASEBKGND {
        if let Some(colour) = the_background_of(window as isize) {
            unsafe {
                let brush = CreateSolidBrush(colour);
                let mut rect = std::mem::zeroed();
                // The box that the machine clips the painting with: the
                // background is erased only where it has to be erased.
                //
                // The v1: "GetClipBox((HDC)wParam, &rect);
                // FillRect((HDC)wParam, &rect, pgui->mBackgroundBrushWin)"
                // (`script_gui.cpp:8598-8599` of v1.1.37.02). The whole
                // client area is erased here, and the window is painted
                // again from one end to the other every time that it
                // changes a little: the background of the window flickers.
                if GetClipBox(wparam as _, &mut rect) != 0 {
                    FillRect(wparam as _, &rect, brush);
                }
                DeleteObject(brush);
            }
            return 1;
        }
    }
    if message == WM_SIZE {
        // The v1: the window of the script that is resized runs the
        // label of the script that is the name of the window and
        // `GuiSize`.
        //
        // The source of the v1 (`script_gui.cpp:1893-1895` of
        // v1.1.37.02) looks for the label `<name>GuiSize`, and
        // `script_gui.cpp:1878-1881` builds the name of it with the
        // name of the window and the `Gui`.
        //
        // Measured on the binary of v1.1.37.02: the
        // main window of a script of the v1 has the label
        // `PVwinGuiSize`, and the tree never ran it: the bar of the
        // menu of the window is built there, and the tree made a
        // window with no bar of a menu.
        if let Ok(mut events) = THE_GUI_EVENTS.lock() {

            events.push(format!("v1size:{}:{}", window as isize, wparam & 0xFFFF));
        }
    }
    if message == WM_CLOSE {
        let found = THE_WINDOW_NAMES.lock().ok().and_then(|table| {
            table
                .iter()
                .find(|(other, _, _)| *other == window as isize)
                .map(|(_, gui_name, label_prefix)| (gui_name.clone(), label_prefix.clone()))
        });
        if let Some((gui_name, label_prefix)) = found {
            if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                events.push(format!(
                    "v1close:{gui_name}:{label_prefix}:{}",
                    window as isize
                ));
            }
            return 0;
        }
    }
    if message == WM_COMMAND {
        if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
            eprintln!("the window 0x{:X} got the command 0x{:X}", window as isize, wparam);
        }
        let id = wparam & 0xFFFF;
        // A command of a control has the handle of the control in the
        // lparam; the one of a menu has none.
        if lparam == 0 {
            let callback = THE_BAR_CALLBACKS.lock().ok().and_then(|table| {
                table
                    .iter()
                    .find(|(other, _)| *other == window as isize)
                    .and_then(|(_, items)| items.iter().find(|(other, _)| *other == id))
                    .map(|(_, callback)| callback.clone())
            });
            if let Some(callback) = callback {
                if !callback.is_empty() {
                    if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                        events.push(format!("v1menu:{callback}"));
                    }
                }
            }
        } else {
            let notify = ((wparam >> 16) & 0xFFFF) as u32;
            let entry = THE_CONTROL_CALLBACKS.lock().ok().and_then(|table| {
                table
                    .iter()
                    .find(|entry| entry.control == lparam)
                    .cloned()
            });
            if let Some(entry) = entry {
                let event = match entry.kind.as_str() {
                    "button" | "checkbox" | "radio" | "text" | "picture" if notify == 0 => {
                        Some("Normal")
                    }
                    "edit" if notify == 0x0300 => Some("Normal"),
                    "combobox" | "ddl" if notify == 1 || notify == 5 => Some("Normal"),
                    "listbox" if notify == 1 => Some("Normal"),
                    "listbox" if notify == 2 => Some("DoubleClick"),
                    _ if notify == 0 => Some("Normal"),
                    _ => None,
                };
                if let Some(event) = event {
                    if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                        events.push(format!(
                            "v1ctrl:{}:{}:{}:{}:{event}:0",
                            entry.control, entry.label, entry.gui_name, entry.control_name
                        ));
                    }
                }
            }
        }
    }
    const WM_NOTIFY: u32 = 0x004E;
    if message == WM_NOTIFY && lparam != 0 {
        #[repr(C)]
        struct NmHdr {
            hwnd_from: HWND,
            id_from: usize,
            code: i32,
        }
        let hdr = unsafe { &*(lparam as *const NmHdr) };
        const TCN_SELCHANGE: i32 = -551;
        const NM_CLICK: i32 = -2;
        const NM_DBLCLK: i32 = -3;
        const NM_RETURN: i32 = -4;
        const NM_RCLICK: i32 = -5;
        if hdr.code == TCN_SELCHANGE {
            the_tab_selection_changed(hdr.hwnd_from);
            let entry = THE_CONTROL_CALLBACKS.lock().ok().and_then(|table| {
                table
                    .iter()
                    .find(|entry| entry.control == hdr.hwnd_from as isize)
                    .cloned()
            });
            if let Some(entry) = entry {
                if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                    events.push(format!(
                        "v1ctrl:{}:{}:{}:{}:Normal:0",
                        entry.control, entry.label, entry.gui_name, entry.control_name
                    ));
                }
            }
        } else if matches!(hdr.code, NM_CLICK | NM_DBLCLK | NM_RETURN | NM_RCLICK) {
            let mut class_buf = [0u16; 64];
            let len = unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::GetClassNameW(
                    hdr.hwnd_from,
                    class_buf.as_mut_ptr(),
                    class_buf.len() as i32,
                )
            };
            let class_name = String::from_utf16_lossy(&class_buf[..len.max(0) as usize]);
            if class_name.eq_ignore_ascii_case("SysLink")
                && (hdr.code == NM_CLICK || hdr.code == NM_RETURN)
            {
                let entry = THE_CONTROL_CALLBACKS.lock().ok().and_then(|table| {
                    table
                        .iter()
                        .find(|entry| entry.control == hdr.hwnd_from as isize)
                        .cloned()
                });
                if let Some(entry) = entry {
                    if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                        events.push(format!(
                            "v1ctrl:{}:{}:{}:{}:Normal:0",
                            entry.control, entry.label, entry.gui_name, entry.control_name
                        ));
                    }
                } else {
                    #[repr(C)]
                    struct LItem {
                        mask: u32,
                        i_link: i32,
                        state: u32,
                        state_mask: u32,
                        sz_id: [u16; 48],
                        sz_url: [u16; 2084],
                    }
                    #[repr(C)]
                    struct NmLink {
                        hdr: NmHdr,
                        item: LItem,
                    }
                    let nmlink = unsafe { &*(lparam as *const NmLink) };
                    let end = nmlink
                        .item
                        .sz_url
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(nmlink.item.sz_url.len());
                    if end > 0 {
                        let url = String::from_utf16_lossy(&nmlink.item.sz_url[..end]);
                        let verb = Wide::of("open");
                        let target = Wide::of(&url);
                        unsafe {
                            windows_sys::Win32::UI::Shell::ShellExecuteW(
                                std::ptr::null_mut(),
                                verb.as_ptr(),
                                target.as_ptr(),
                                std::ptr::null(),
                                std::ptr::null(),
                                SW_SHOW,
                            );
                        }
                    }
                }
            } else if class_name.eq_ignore_ascii_case("SysListView32") {
                #[repr(C)]
                struct NmItemActivate {
                    hdr: NmHdr,
                    i_item: i32,
                    i_sub_item: i32,
                }
                let item = unsafe { &*(lparam as *const NmItemActivate) };
                let row = if item.i_item >= 0 {
                    (item.i_item + 1) as i64
                } else {
                    0
                };
                let event = match hdr.code {
                    NM_DBLCLK => "DoubleClick",
                    NM_RCLICK => "RightClick",
                    _ => "Normal",
                };
                let entry = THE_CONTROL_CALLBACKS.lock().ok().and_then(|table| {
                    table
                        .iter()
                        .find(|entry| entry.control == hdr.hwnd_from as isize)
                        .cloned()
                });
                if let Some(entry) = entry {
                    if let Ok(mut events) = THE_GUI_EVENTS.lock() {
                        events.push(format!(
                            "v1ctrl:{}:{}:{}:{}:{event}:{row}",
                            entry.control, entry.label, entry.gui_name, entry.control_name
                        ));
                    }
                }
            }
        }
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// A window of the script is made.
///
/// The answer is the number of the window, the way the tree numbers the
/// windows of the script.
pub fn open(title: &str) -> Result<TheWindow, String> {
    open_with(title, WS_OVERLAPPEDWINDOW, 0)
}

/// A window of the script of the v1 is made.
///
/// The window of the v1 is the window of the machine with the class
/// "AutoHotkeyGUI", and the styles of it are the ones that the v1 gives
/// to it.
///
/// Measured on the binary of v1.1.37.02: the ordinary
/// style of a window of the v1 is `0x84CA0000` -- the style of an
/// overlapped window with the `WS_POPUP`, without the `WS_THICKFRAME`
/// and without the `WS_MAXIMIZEBOX` -- and its extended style is `0x100`,
/// the `WS_EX_WINDOWEDGE`. The machine adds the `WS_CLIPSIBLINGS` to a
/// window of its own.
///
/// The `WS_POPUP` is what makes the window a window of its own and not
/// a control: the window of the script owns the windows that the
/// a script of the v1 paints its picture into, and an owned window is
/// above its owner.
///
/// The `WS_THICKFRAME` and the `WS_MAXIMIZEBOX` are taken away: a
/// window of the v1 is not resizable until the script asks for it with
/// the "Resize", and it has no maximize button.
pub fn open_of_the_v1(title: &str) -> Result<TheWindow, String> {
    // The `WS_EX_WINDOWEDGE` is the edge of the border of the window,
    // and the machine gives it to a window that has a border.
    let style = WS_POPUP | (WS_OVERLAPPEDWINDOW & !(WS_THICKFRAME | WS_MAXIMIZEBOX));
    let window = open_with(title, style, WS_EX_WINDOWEDGE)?;
    // The v1: the window of the script carries the icon of the program
    // that runs it -- the small one in the title bar and the big one in
    // the list of the windows.
    //
    // Measured on the binary of the v1.1.37.02: the
    // window of the script of the v1 has an icon, and the `Menu, Tray,
    // Icon` does not change it.
    let (small, big) = super::trash::the_icons_of_the_program();
    if !small.is_null() || !big.is_null() {
        const WM_SETICON: u32 = 0x0080;
        const ICON_SMALL: usize = 0;
        const ICON_BIG: usize = 1;
        unsafe {
            if !small.is_null() {
                SendMessageW(window.handle, WM_SETICON, ICON_SMALL, small as LPARAM);
            }
            if !big.is_null() {
                SendMessageW(window.handle, WM_SETICON, ICON_BIG, big as LPARAM);
            }
        }
    }
    Ok(window)
}
/// A window of the script is made with the styles that the machine is
/// told to give to it.
fn open_with(title: &str, style: u32, extended_style: u32) -> Result<TheWindow, String> {
    the_common_controls_are_taken();
    unsafe {
        let module: HINSTANCE = GetModuleHandleW(std::ptr::null_mut());
        let class = Wide::of(THE_CLASS);
        // The class of the window carries the icon of the program that
        // runs the script: the small one in the title bar and the big one
        // in the list of the windows.
        //
        // Measured on the binary of the v1.1.37.02: the class of the
        // window of the script of the v1 has the icon of the program (the
        // big one and the small one), and the tree had none: the window
        // showed no icon in the list of the windows.
        let (small, big) = super::trash::the_icons_of_the_program();
        let window_class = WNDCLASSEXW {
            lpfnWndProc: Some(the_procedure),
            hInstance: module,
            lpszClassName: class.as_ptr(),
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            // The brush of the background: the machine erases the window
            // with it and asks the window to paint itself. The colour of
            // each window is the one that the script gave to it, and the
            // procedure of the window paints it.
            hbrBackground: GetStockObject(COLOR_3DFACE),
            hIcon: big,
            hIconSm: small,
            ..std::mem::zeroed()
        };
        if RegisterClassExW(&window_class) == 0 {
            let code = windows_sys::Win32::Foundation::GetLastError();
            if code != 1410 {
                return Err(format!(
                    "the machine did not take the class of the window: {code}."
                ));
            }
        }
        let title = Wide::of(title);
        // The window is made without a size: the size of it is the one
        // that the script gives, or the one that its controls reach.
        //
        // Measured on the binary of v1.1.37.02: the window
        // without controls is of 0x0, and the tree made it of the size
        // that the machine gives to a window that has none.
        // The v1 makes the window of the script in the corner of the
        // screen, with no size: the source of the v1
        // (`script_gui.cpp:1786-1787`) asks for the place (0,0) and the
        // size (0,0), and the `Show` gives the window the place and the
        // size that the script asks for.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02:
        // the window of a script of the v1 that paints the picture is
        // made in the corner of the screen, and it is a child of the main
        // window: it is at the corner of the client of the main window.
        // The tree made it at (100,100), and the picture was not in the
        // corner of the window.
        let handle = CreateWindowExW(
            extended_style,
            class.as_ptr(),
            title.as_ptr(),
            style,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        if handle.is_null() {
            let code = windows_sys::Win32::Foundation::GetLastError();
            return Err(format!("the machine did not make the window: {code}."));
        }
        let mut window = TheWindow {
            handle,
            controls: Vec::new(),
            name: String::new(),
            background: None,
            margin: (0, 0),
            section: None,
            section_at: None,
            font: None,
            font_handle: Some(the_default_font_of_the_machine()),
            menu_bar: false,
            menu: None,
            menu_name: String::new(),
            delimiter: '|',
            label: String::new(),
            min_size: None,
            max_size: None,
            shown: false,
            current_tab_control: None,
            current_tab_page: None,
            active_tab_page: 1,
            tab_just_switched: false,
            layout_anchor_control: None,
        };
        // The v1: the window has a margin that follows the size of the
        // font of it, and the first control of it is at that margin.
        //
        // Measured on the binary of v1.1.37.02: the first
        // control of a window that has no `Gui, Font` is at 10,6 of the
        // client area of the window.
        window.margin = the_margin_of_the_window(&window);
        Ok(window)
    }
}

/// The font of the machine that the window has before a `Gui, Font`.
///
/// The v1 creates the font of the base with the
/// `GetStockObject(DEFAULT_GUI_FONT)` (source `script_gui.cpp:7720-7721` of
/// v1.1.37.02), and gives it to every control that is made.
///
/// Measured on the binary of v1.1.37.02: the `WM_GETFONT`
/// of a button of a GUI that has no `Gui, Font` answers the handle of the
/// default font of the machine, and the `WM_GETFONT` of the window itself
/// answers zero -- the v1 gives the font to the controls and not to the
/// window.
fn the_default_font_of_the_machine() -> isize {
    unsafe { GetStockObject(DEFAULT_GUI_FONT) as isize }
}

/// The name of the default font of the machine and the height of it.
///
/// The `Gui, Font` of the v1 builds the font out of its options, and the
/// name and the size that the options do not name are the ones of the
/// default font of the machine.
///
/// Measured on the binary of v1.1.37.02: the `Gui, Font, Bold` of a
/// window that has no `Gui, Font` gives a control of the font "MS Shell
/// Dlg" with the height -11 and the weight 700, and the `Gui, Font, s19`
/// gives a control of the font "MS Shell Dlg" with the height -25.
fn the_name_and_the_height_of_the_default_font() -> (String, i32) {
    unsafe {
        let handle = GetStockObject(DEFAULT_GUI_FONT);
        let mut logfont: LOGFONTW = std::mem::zeroed();
        if GetObjectW(handle as HGDIOBJ, std::mem::size_of::<LOGFONTW>() as i32, &mut logfont as *mut LOGFONTW as *mut core::ffi::c_void) == 0 {
            return ("MS Shell Dlg".to_string(), -11);
        }
        let name = String::from_utf16_lossy(
            &logfont.lfFaceName[..logfont
                .lfFaceName
                .iter()
                .position(|letter| *letter == 0)
                .unwrap_or(LF_FACESIZE as usize)],
        );
        (name, logfont.lfHeight)
    }
}

/// Compute the pixel height corresponding to `r<rows>` for a control in
/// `window` using the window's current font metrics.
pub fn the_height_of_rows(window: &TheWindow, kind: &str, rows: i64) -> i64 {
    let rows = rows.max(1);
    let line_height = window
        .font_handle
        .map(|font| unsafe {
            let hdc = GetDC(window.handle);
            if hdc.is_null() {
                return 13i64;
            }
            let old = SelectObject(hdc, font as HGDIOBJ);
            let mut metrics: TEXTMETRICW = std::mem::zeroed();
            GetTextMetricsW(hdc, &mut metrics);
            SelectObject(hdc, old);
            ReleaseDC(window.handle, hdc);
            (metrics.tmHeight as i64).max(10)
        })
        .unwrap_or(13);
    match kind {
        "edit" => rows * line_height + 8,
        "listview" => (rows + 1) * (line_height + 4) + 8,
        "listbox" => rows * line_height + 6,
        "treeview" => rows * (line_height + 3) + 6,
        _ => rows * line_height,
    }
}

/// The size of a control of the window of the script.
///
/// The v1 measures the text of the control with the `DrawText` of the
/// machine, and the size that the options of the script give is the size
/// of the control when the text of it fits in it (source
/// `script_gui.cpp:2651-2890` of v1.1.37.02).
///
/// Measured on the binary of v1.1.37.02: the button "Help"
/// of a window that has no `Gui, Font` is 34x23, the edit "Hello" is
/// 33x21, the text "Label" is 26x13 and the check box "Check" is 33x13;
/// the button of a long text that the script gives the width of 100 is
/// 100x36, because the text of it is in two lines.
pub fn the_size_of_a_control(
    window: &TheWindow,
    kind: &str,
    text: &str,
    style: u32,
    width: Option<i64>,
    height: Option<i64>,
) -> (i64, i64) {
    let Some(font) = window.font_handle else {
        return (0, 0);
    };
    unsafe {
        let hdc = GetDC(window.handle);
        if hdc.is_null() {
            return (0, 0);
        }
        let old_font = SelectObject(hdc, font as HGDIOBJ);
        let mut metrics: TEXTMETRICW = std::mem::zeroed();
        GetTextMetricsW(hdc, &mut metrics);
        let (point_size, scale) = the_point_size_and_the_scale(hdc);
        let size = the_measured_size_of_a_control(
            hdc,
            &metrics,
            kind,
            text,
            style,
            (width, height),
            (point_size, scale),
        );
        SelectObject(hdc, old_font);
        ReleaseDC(window.handle, hdc);
        match size {
            Some(size) => size,
            None => the_default_size_of_a_control(
                kind,
                point_size,
                scale,
                window.margin.0,
                width,
                height,
            ),
        }
    }
}

/// The size that the text of a control asks for.
///
/// The v1 does not measure the controls that have no text to measure, and
/// the answer of this function is the empty one for them.
fn the_measured_size_of_a_control(
    hdc: HDC,
    metrics: &TEXTMETRICW,
    kind: &str,
    text: &str,
    style: u32,
    size: (Option<i64>, Option<i64>),
    font: (i64, i64),
) -> Option<(i64, i64)> {
    let (width, height) = size;
    let (point_size, scale) = font;
    unsafe {
        // The kinds whose text the v1 measures.
        if !matches!(
            kind,
            "text" | "link" | "button" | "checkbox" | "radio" | "edit"
        ) {
            return None;
        }
        // The v1 does not measure an edit that has no text (source
        // `script_gui.cpp:2657-2661` of v1.1.37.02).
        if kind == "edit" && text.is_empty() {
            return None;
        }
        // The v1 measures the text of a control only when the width or
        // the height of it is not written (source `script_gui.cpp:2647`
        // of v1.1.37.02): a control that has the two keeps the size that
        // the script gave to it.
        //
        // Measured on the binary of v1.1.37.02: the button
        // of the text "SomeLongTextHere" that the script gives the width
        // of 35 and the height of 35 is 35x35, and the same button without
        // the width and the height is 107x23.
        if width.is_some() && height.is_some() {
            return None;
        }
        let mut format = DT_CALCRECT;
        let mut extra_width = 0i64;
        let mut extra_height = 0i64;
        match kind {
            "edit" => {
                format |= DT_EXPANDTABS | DT_EDITCONTROL | DT_NOPREFIX;
                // The v1 leaves the room of an average character and of four
                // pixels on the side of the text of the edit (source
                // `script_gui.cpp:2667-2669`).
                extra_width += 4 + metrics.tmAveCharWidth as i64;
            }
            "text" | "link" => {
                format |= DT_EXPANDTABS;
                if style & SS_NOPREFIX != 0 {
                    format |= DT_NOPREFIX;
                }
                if style & WS_BORDER != 0 {
                    extra_width = 2 * GetSystemMetrics(SM_CXBORDER) as i64;
                    extra_height = 2 * GetSystemMetrics(SM_CYBORDER) as i64;
                }
            }
            "checkbox" | "radio" => {
                // The v1 leaves the room of the box, of an average character
                // and of two pixels (source `script_gui.cpp:2702-2708`).
                extra_width += GetSystemMetrics(SM_CXMENUCHECK) as i64
                    + metrics.tmAveCharWidth as i64
                    + 2;
            }
            _ => {}
        }
        // The rectangle of the measure: the width that the script gave is the
        // width that the text has to fit in.
        let mut rectangle = RECT {
            left: 0,
            top: 0,
            right: match width {
                Some(value) => (value - extra_width).max(0) as i32,
                None => 0,
            },
            bottom: match height {
                Some(value) => value as i32,
                None => 0,
            },
        };
        // The text is wrapped when the control has the `BS_MULTILINE`, and
        // when it is not a button, a check box or a radio button.
        if rectangle.right != 0
            && (style & BS_MULTILINE != 0 || !matches!(kind, "button" | "checkbox" | "radio"))
        {
            format |= DT_WORDBREAK;
        }
        // The v1 measures the letter "H" when the control has no text, so that
        // the measure is always the same (source `script_gui.cpp:2814-2815`).
        let measured = if text.is_empty() { "H" } else { text };
        let wide = Wide::of(measured);
        let draw_height = DrawTextW(hdc, wide.as_ptr(), -1, &mut rectangle, format);
        let mut draw_width = (rectangle.right - rectangle.left) as i64;
        // The overhang of the last letter: the v1 takes it away when the font
        // has one (source `script_gui.cpp:2824-2839`).
        //
        // The v1 does not take it away from a button that has the
        // `BS_MULTILINE`, because the style keeps the button from using the
        // extra space (source `script_gui.cpp:2825-2827` of v1.1.37.02).
        let multiline = style & BS_MULTILINE != 0
            && (width.is_some()
                || height.is_some()
                || text.contains('\n')
                || text.contains('\r'));
        if !text.is_empty()
            && matches!(kind, "text" | "edit" | "button")
            && !(kind == "button" && multiline)
        {
            if let Some(last) = text.chars().last() {
                let mut abc: ABC = std::mem::zeroed();
                if GetCharABCWidthsW(hdc, last as u32, last as u32, &mut abc) != 0 && abc.abcC < 0 {
                    draw_width -= abc.abcC as i64;
                }
            }
        }
        let mut result_width = draw_width + extra_width;
        let mut result_height = draw_height as i64 + extra_height;
        match kind {
            "button" => {
                // The v1 leaves the room of the border of the button and of the
                // size of the font (source `script_gui.cpp:2876-2879`).
                result_width += 2 * GetSystemMetrics(SM_CXEDGE) as i64 + point_size;
                result_height += point_size + 2;
            }
            "edit" => {
                result_height += the_scaled(8, scale);
            }
            _ => {}
        }
        // The size that the options of the script give wins over the measure,
        // and the measure wins when the text does not fit in it.
        let result_width = match width {
            Some(value) => {
                if draw_width > value {
                    result_width
                } else {
                    value
                }
            }
            None => result_width,
        };
        let result_height = match height {
            Some(value) => {
                if draw_height as i64 > value && kind != "edit" {
                    result_height
                } else {
                    value
                }
            }
            None => result_height,
        };

    Some((result_width, result_height))
    }
}

/// The size that the v1 gives to a control that it does not measure.
///
/// The width of the control is the standard width of the machine, which is
/// fifteen times the size of the font (source `script.h:2725` of
/// v1.1.37.02), and the height of it is the one of the kind of the
/// control.
///
/// Measured on the binary of v1.1.37.02: the edit that has
/// no text is 120x21, the combo box is 120x21, the list box is 120x43, the
/// group box is 140x57, the slider is 120x33, the field of the date is
/// 240x21, the bar of the progress is 104x16, the field of the numbers is
/// 16x16, the tab is 266x247, the list of the files is 240x113, the tree
/// is 240x84 and the calendar is 225x160.
fn the_default_size_of_a_control(
    kind: &str,
    point_size: i64,
    scale: i64,
    margin_x: i64,
    width: Option<i64>,
    height: Option<i64>,
) -> (i64, i64) {
    let standard = the_scaled(15 * point_size, scale);
    let default_width = match kind {
        "edit" | "combobox" | "ddl" | "listbox" | "hotkey" | "updown" | "slider" => standard,
        "progress" => the_scaled(13 * point_size, scale),
        "listview" | "treeview" | "datetime" => 2 * standard,
        "groupbox" => standard + 2 * margin_x,
        "tab" => 2 * standard + 3 * margin_x - 4,
        "monthcal" => the_scaled(225 * point_size / 8, scale),
        _ => 0,
    };
    let default_height = match kind {
        "edit" | "combobox" | "ddl" | "hotkey" | "datetime" => {
            the_scaled(21 * point_size / 8, scale)
        }
        "listbox" => the_scaled(43 * point_size / 8, scale),
        "groupbox" => the_scaled(57 * point_size / 8, scale),
        "slider" => the_scaled(33 * point_size / 8, scale),
        "progress" | "updown" => the_scaled(2 * point_size, scale),
        "tab" => the_scaled(247 * point_size / 8, scale),
        "listview" => the_scaled(113 * point_size / 8, scale),
        "treeview" => the_scaled(84 * point_size / 8, scale),
        "monthcal" => the_scaled(160 * point_size / 8, scale),
        _ => 0,
    };
    (
        width.unwrap_or(default_width),
        height.unwrap_or(default_height),
    )
}

/// The size of the font of the machine in points, and the density of the
/// screen.
///
/// The v1 computes the size of the font from the metrics of it (source
/// `script_gui.cpp:7726` of v1.1.37.02) and it scales the sizes of the
/// machine with the density of the screen (source `script.h:317`).
fn the_point_size_and_the_scale(hdc: HDC) -> (i64, i64) {
    let mut metrics: TEXTMETRICW = unsafe { std::mem::zeroed() };
    unsafe { GetTextMetricsW(hdc, &mut metrics) };
    let pixels = unsafe { GetDeviceCaps(hdc, LOGPIXELSY as i32) } as i64;
    let point_size = if pixels > 0 {
        ((metrics.tmHeight - metrics.tmInternalLeading) as i64 * 72) / pixels
    } else {
        8
    };
    (point_size, if pixels > 0 { pixels } else { 96 })
}

/// A size of the machine, scaled with the density of the screen.
fn the_scaled(value: i64, scale: i64) -> i64 {
    (value * scale + 48) / 96
}

/// The margin that the v1 leaves between the controls of a window and the
/// border of it.
///
/// The v1 makes the margin of the window with the size of the font (source
/// `script_gui.cpp:2023-2025` of v1.1.37.02).
pub fn the_margin_of_the_window(window: &TheWindow) -> (i64, i64) {
    let Some(font) = window.font_handle else {
        return (10, 6);
    };
    unsafe {
        let hdc = GetDC(window.handle);
        if hdc.is_null() {
            return (10, 6);
        }
        let old_font = SelectObject(hdc, font as HGDIOBJ);
        let (point_size, scale) = the_point_size_and_the_scale(hdc);
        SelectObject(hdc, old_font);
        ReleaseDC(window.handle, hdc);
        (
            the_scaled((point_size * 5) / 4, scale),
            the_scaled((point_size * 3) / 4, scale),
        )
    }
}

/// A control is added to a window of the script.
///
/// The answer is the number of the control and the class of it, the way
/// the v2 answers it: "Button1" is the class "Button" and the first
/// control of it.
pub fn add(window: &mut TheWindow, kind: &str, text: &str) -> Result<(u64, String), String> {
    add_named(window, kind, text, "", None, TheStyleOfTheV1::default())
}

/// A control is added to a window of the script, with the name of the
/// variable that it is bound to and the place that it takes.
///
/// The `place` is the place and the size of the control: the `x` and the
/// `y` and the `w` and the `h` of the options of the v1. The control that
/// has no place takes the place of the one before it, the way the v1
/// lays the controls of a window out.
///
/// The `style` is the one that the options of the v1 ask for: the bits
/// that they add and the bits that they take away. The v1 makes the style
/// of the control like this (source `script_gui.cpp:2240-2242` of
/// v1.1.37.02):
/// `style = style_add & ~style_remove`.
pub fn add_named(
    window: &mut TheWindow,
    kind: &str,
    text: &str,
    name: &str,
    place: Option<ThePlace>,
    style: TheStyleOfTheV1,
) -> Result<(u64, String), String> {
    let Some(control_kind) = the_class_of_a_kind(kind) else {
        return Err(format!(
            "the machine has no control of the kind {kind}: the kinds are Button, CheckBox, Radio, GroupBox, Text, Picture, Edit, ListBox, ComboBox, DDL, Progress, Slider, UpDown, DateTime, MonthCal, Hotkey, Link, Tab, StatusBar, TreeView and ListView."
        ));
    };
    let module: HINSTANCE = unsafe { GetModuleHandleW(std::ptr::null_mut()) };
    let class_name = Wide::of(control_kind.class);
    let text_wide = Wide::of(text);
    // The `WS_TABSTOP` and the `WS_GROUP` depend on the kind of the control
    // and on the group in which it is born: the first radio button of a
    // group is the one that has the `WS_GROUP` and the `WS_TABSTOP`
    // (source `script_gui.cpp:2277-2280` of v1.1.37.02).
    let mut bits = WS_CHILD | WS_VISIBLE | control_kind.style | style.add;
    if control_kind.tabstop {
        bits |= WS_TABSTOP;
    }
    if control_kind.group {
        if !window.controls.iter().any(|other| other.kind == "radio") {
            bits |= WS_TABSTOP | WS_GROUP;
        } else {
            bits |= WS_GROUP;
        }
    }
    // The v1: an edit that has more than one line of text is an edit of
    // more than one line (source `script_gui.cpp:2423` of v1.1.37.02).
    if kind == "edit" && (text.contains('\n') || text.contains('\r')) {
        bits |= ES_MULTILINE as u32;
    }
    let bits = bits & !style.remove;
    // The place of the control: the one that the script gives, or the one
    // that the control before it leaves, the way the v1 lays the
    // controls of a window out.
    let mut place = place.unwrap_or_else(|| the_place_after_the_last(window));
    // The size of the control: the v1 measures the text of it, and the
    // size of it is the size of the kind of it when it has no text to
    // measure.
    let (width, height) = the_size_of_a_control(window, kind, text, bits, None, None);
    if width > 0 {
        place.width = width;
    }
    if height > 0 {
        place.height = height;
    }
    unsafe {
        let handle = CreateWindowExW(
            control_kind.exstyle,
            class_name.as_ptr(),
            text_wide.as_ptr(),
            bits,
            place.x as i32,
            place.y as i32,
            place.width.max(1) as i32,
            place.height.max(1) as i32,
            window.handle,
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        if handle.is_null() {
            let code = windows_sys::Win32::Foundation::GetLastError();
            return Err(format!("the machine did not make the control: {code}."));
        }
        // The font of the window goes to the control: the v1 gives the
        // font to every control that is made, and the font of the window
        // without `Gui, Font` is the font of the system (source
        // `script_gui.cpp:3027` and `7720-7733` of v1.1.37.02).
        //
        // Measured on the binary of v1.1.37.02: the
        // `WM_GETFONT` of a button of a GUI that has no `Gui, Font`
        // answers the handle of the default font of the machine.
        if control_kind.font {
            if let Some(font) = window.font_handle {
                SendMessageW(handle, WM_SETFONT, font as usize, 1);
            }
        }
        window.controls.push(TheControl {
            handle,
            kind: kind.to_string(),
            class: control_kind.class.to_string(),
            name: name.to_string(),
            text: text.to_string(),
            icon: style.icon,
        });
        if !style.glabel.is_empty() {
            let ctrl_id = if !name.is_empty() { name } else { text };
            the_control_callback_is_set(
                handle as isize,
                &style.glabel,
                &window.name,
                ctrl_id,
                kind,
            );
        }
        let number = window.controls.len() as u64;
        // Populate SysTabControl32 pages (`Page1|Page2|...`) and track active tab.
        if control_kind.class == "SysTabControl32" {
            const TCM_INSERTITEMW: u32 = 0x133E;
            const TCIF_TEXT: u32 = 0x0001;
            #[repr(C)]
            struct TcItemW {
                mask: u32,
                dw_state: u32,
                dw_state_mask: u32,
                psz_text: *mut u16,
                cch_text_max: i32,
                i_image: i32,
                l_param: isize,
            }
            if !text.is_empty() {
                for (index, tab_label) in text.split(window.delimiter).enumerate() {
                    let mut wide_label = Wide::of(tab_label);
                    let item = TcItemW {
                        mask: TCIF_TEXT,
                        dw_state: 0,
                        dw_state_mask: 0,
                        psz_text: wide_label.as_mut_ptr(),
                        cch_text_max: 0,
                        i_image: -1,
                        l_param: 0,
                    };
                    SendMessageW(
                        handle,
                        TCM_INSERTITEMW,
                        index,
                        &item as *const TcItemW as isize,
                    );
                }
            }
            let chosen = style.choose.unwrap_or(1).max(1) as usize;
            SendMessageW(handle, TCM_SETCURSEL, chosen.saturating_sub(1), 0);
            window.current_tab_control = Some(handle as isize);
            window.current_tab_page = Some(1);
            window.active_tab_page = chosen;
            window.tab_just_switched = true;
            window.layout_anchor_control = None;
        } else if let (Some(tab_hwnd), Some(page)) =
            (window.current_tab_control, window.current_tab_page)
        {
            the_tab_child_is_added(tab_hwnd, handle as isize, page);
            if page != window.active_tab_page {
                ShowWindow(handle, SW_HIDE);
            }
        }
        // The v1: the "Checked" of the options puts the box in the state
        // that they ask for (source `script_gui.cpp:3217-3218` of
        // v1.1.37.02).
        //
        // Measured on the binary of v1.1.37.02: the
        // `Gui, Add, Checkbox, vCHK Checked, Check` makes a check box that
        // is checked, and the `GuiControlGet, out, , CHK` of it answers "1".
        if let Some(state) = style.checked {
            if matches!(kind, "checkbox" | "radio") {
                the_state_of_the_v1_is_written(&window.controls[number as usize - 1], state);
            }
        }
        // The items of a combo box and of a list box are the parts of the
        // text that the pipe separates (with `||` selecting the preceding
        // item and `ChooseN` selecting the 1-based Nth item).
        if control_kind.class == "ComboBox" || control_kind.class == "ListBox" {
            let control = &window.controls[number as usize - 1];
            populate_list_items(control, text, window.delimiter, style.choose, false);
        }
        // The machine fits the height of a list box to the items that it
        // has, and the v1 gives the list box the size that it wants when
        // the items of it are already in it.
        //
        // Measured on the binary of v1.1.37.02: the
        // `Gui, Add, ListBox, vLIST, A|B|C` of a window that has no
        // `Gui, Font` is 120x43, and the list box of the tree is 120x30
        // when the size of it is not given again after the items.
        if control_kind.class == "ListBox" {
            let control = &window.controls[number as usize - 1];
            the_place_of_a_control_is_written(
                control,
                ThePlace {
                    x: place.x,
                    y: place.y,
                    width: place.width,
                    height: place.height,
                },
            );
        }
        let classnn = the_classnn_of(window, number);
        Ok((number, classnn))
    }
}

/// The colour of the v1, out of the text that the script wrote.
///
/// The v1 writes a colour as six digits of the hexadecimal, and the
/// machine takes it as the colour of the brush of the window. The colour
/// that the script names by its name is the one of the table of the
/// machine.
pub fn the_colour_of_the_v1(text: &str) -> Result<u32, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(0);
    }
    // The colour of the window and the colour of the controls of it are
    // written the same way, and the first one is the one of the window.
    let first = text.split(',').next().unwrap_or("").trim();
    if first.eq_ignore_ascii_case("default") {
        return Ok(0);
    }
    if let Some(hex) = first.strip_prefix("0x") {
        return u32::from_str_radix(hex, 16)
            .map(|colour| colour & 0xFFFFFF)
            .map_err(|_| format!("the colour {first} is not a number of the machine."));
    }
    if first.chars().all(|c| c.is_ascii_hexdigit()) && first.len() <= 6 {
        return u32::from_str_radix(first, 16)
            .map(|colour| colour & 0xFFFFFF)
            .map_err(|_| format!("the colour {first} is not a number of the machine."));
    }
    // The names of the colours of the machine.
    let named = match first.to_ascii_lowercase().as_str() {
        "black" => 0x000000,
        "white" => 0xFFFFFF,
        "red" => 0xFF0000,
        "green" => 0x008000,
        "blue" => 0x0000FF,
        "yellow" => 0xFFFF00,
        "gray" | "grey" => 0x808080,
        "silver" => 0xC0C0C0,
        "maroon" => 0x800000,
        "olive" => 0x808000,
        "purple" => 0x800080,
        "teal" => 0x008080,
        "navy" => 0x000080,
        "lime" => 0x00FF00,
        "aqua" | "cyan" => 0x00FFFF,
        "fuchsia" | "magenta" => 0xFF00FF,
        "orange" => 0xFFA500,
        "brown" => 0xA52A2A,
        "pink" => 0xFFC0CB,
        "gold" => 0xFFD700,
        other => {
            return Err(format!(
                "the tree does not know the colour {other}: the colours of the machine are the \
                 ones of the table of them."
            ))
        }
    };
    Ok(named)
}

/// The font of the v1, out of the options that the script wrote, merged on top
/// of the window's current font when `options` or `name` is non-empty.
///
/// The v1: `Gui, Font` (both parameters omitted) restores the system default
/// font; otherwise any unspecified attribute keeps the window's current font.
///
/// Measured on the binary of v1.1.37.02: the name of the font is the one
/// of the second parameter of the command -- the `Gui, Font, Bold` of a
/// window whose font is "Arial" gives the controls the font of the machine
/// and not the "Arial" of the window -- and the size and the weight of it
/// are the ones of the window when the options do not name them.
pub fn the_font_of_the_v1_from(
    current: Option<&TheFont>,
    options: &str,
    name: &str,
) -> Option<TheFont> {
    if options.trim().is_empty() && name.trim().is_empty() {
        return None;
    }
    let mut font = current.cloned().unwrap_or_default();
    // The name of the font is the one that the command gives: the name
    // that is not there is the name of the default font of the machine.
    font.name = name.trim().to_string();
    for word in options.split_whitespace() {
        let word = word.trim();
        if word.is_empty() {
            continue;
        }
        let lower = word.to_ascii_lowercase();
        match lower.as_str() {
            "bold" => font.bold = true,
            "italic" => font.italic = true,
            "strike" => font.strike = true,
            "underline" => font.underline = true,
            "norm" | "normal" => {
                font.bold = false;
                font.italic = false;
                font.strike = false;
                font.underline = false;
            }
            _ => match lower.chars().next() {
                Some('s') => {
                    if let Ok(size) = lower[1..].parse::<i64>() {
                        if size > 0 {
                            font.size = size;
                        }
                    }
                }
                Some('w') => {
                    if let Ok(weight) = lower[1..].parse::<i64>() {
                        font.bold = weight >= 600;
                    }
                }
                Some('c') => {
                    if let Ok(colour) = the_colour_of_the_v1(&lower[1..]) {
                        font.color = Some(colour);
                    }
                }
                _ => {}
            },
        }
    }
    Some(font)
}

/// The font of the v1, out of the options that the script wrote.
pub fn the_font_of_the_v1(options: &str, name: &str) -> TheFont {
    // The font that the options do not name is the default font of the
    // machine: the name of it and the height of it are the ones of the
    // stock object of the machine.
    the_font_of_the_v1_from(None, options, name).unwrap_or_default()
}

/// The bar of the menu of a window is written.
///
/// The bar of the menu of the v1 is the bar of the menu of the script
/// that the `Gui, Menu` names, and the machine takes it as the bar of the
/// menu of the window.
///
/// The bar is the menu of the machine when the machine has one: the
/// items of the bar are the items of that menu, and the dropdown of the
/// bar is the menu of the script. The message that the machine sends
/// when the dropdown opens carries the handle of it, and the script
/// recognizes the menu by that handle.
pub fn the_menu_bar_is_written(
    window: &mut TheWindow,
    name: &str,
    _json: &str,
    bar: super::menu::TheMenuBar,
) -> Result<(), String> {
    // The bar of the window is the menu of the script, and the handle of
    // it does not change when the menu changes. The bar is written again
    // only when it is another one: writing the same one while a menu of
    // it is open takes the menu of the machine away, and the dropdown of
    // the bar opens nothing.
    let same = window.menu.as_ref().map(|held| held.handle) == Some(bar.handle);
    if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
        eprintln!(
            "the bar of the window 0x{:X} is written: the bar is 0x{:X}, the window had 0x{:X}, the same: {}",
            window.handle as isize,
            bar.handle as isize,
            window.menu.as_ref().map(|held| held.handle as isize).unwrap_or(0),
            same
        );
    }
    if !same {
        let answer = unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::SetMenu(window.handle, bar.handle)
        };
        if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
            eprintln!(
                "the SetMenu of the window 0x{:X} with the bar 0x{:X} answered {}",
                window.handle as isize,
                bar.handle as isize,
                answer
            );
        }
        if answer == 0 {
            return Err("the machine did not attach the menu bar to the window.".to_string());
        }
    }
    // The callbacks of the items of the bar go to the table that the
    // procedure of the window reads: the click on an item is a command
    // with the number of the item.
    the_bar_callbacks_are_set(window.handle as isize, bar.items.clone());
    // The bar belongs to the window now: the bar that is destroyed is
    // the bar of a window that has none, and the window of the
    // a script of the v1 keeps the bar of the menu of it -- the room
    // of the bar is the room that is not the client area.
    window.menu = Some(bar);
    window.menu_name = name.to_string();
    Ok(())
}

/// The place and the size of a control of a window.
#[derive(Clone, Copy, Debug, Default)]
pub struct ThePlace {
    /// The place of the control on the window.
    pub x: i64,
    /// The place of the control on the window.
    pub y: i64,
    /// How wide the control is.
    pub width: i64,
    /// How high the control is.
    pub height: i64,
}

/// The place of the control that follows the last one of a window.
///
/// The v1 lays the controls of a window out one after the other, and the
/// margin of the window is the space that they keep from each other.
fn the_place_after_the_last(window: &TheWindow) -> ThePlace {
    let Some(last) = window.controls.last() else {
        return ThePlace {
            x: window.margin.0,
            y: window.margin.1,
            width: 160,
            height: 24,
        };
    };
    // The place of the control before it is the place of it in the
    // client area of the window, and not the place of it on the screen.
    let (x, y, width, height) = the_place_of_the_control_in_the_window(window, last);
    ThePlace {
        x,
        y: y + height + window.margin.1,
        width,
        height,
    }
}

/// The place and the size that the options of the `Gui, Add` of the v1 ask
/// for, and the machine computes.
///
/// Read in the CHM of the v1.1.37.02 (`Gui.htm`) and in the source of the
/// v1 (`script_gui.cpp:5731-5835`): the `x` and the `y` of the options are
/// places that the machine computes from the window and from the control
/// that was added before it, and the size that the options do not give is
/// the size that the control already has.
pub fn the_place_of_a_control_is_computed(
    window: &TheWindow,
    number: u64,
    place: ThePlaceOfTheV1,
) -> (i64, i64, Option<i64>, Option<i64>) {
    let (margin_x, margin_y) = window.margin;
    // The control that was added before this one: the place and the size of
    // it are what the relative places of the options are built on.
    let current_is_tab = window
        .controls
        .get(number.saturating_sub(1) as usize)
        .is_some_and(|c| c.class == "SysTabControl32");
    let (previous_x, previous_y, previous_width, previous_height) = if !current_is_tab
        && window.tab_just_switched
        && let Some(tab_hwnd) = window.current_tab_control
    {
        if let Some(tab_ctrl) = window
            .controls
            .iter()
            .find(|c| c.handle as isize == tab_hwnd)
        {
            let (tx, ty, tw, th) = the_place_of_the_control_in_the_window(window, tab_ctrl);
            let mut rc = RECT {
                left: tx as i32,
                top: ty as i32,
                right: (tx + tw) as i32,
                bottom: (ty + th) as i32,
            };
            unsafe {
                SendMessageW(tab_ctrl.handle, TCM_ADJUSTRECT, 0, &mut rc as *mut RECT as LPARAM);
            }
            (rc.left as i64, rc.top as i64, 0, 0)
        } else {
            (margin_x, margin_y, 0, 0)
        }
    } else if !current_is_tab && let Some(anchor_hwnd) = window.layout_anchor_control {
        if let Some(anchor_ctrl) = window
            .controls
            .iter()
            .find(|c| c.handle as isize == anchor_hwnd)
        {
            the_place_of_the_control_in_the_window(window, anchor_ctrl)
        } else {
            (margin_x, margin_y, 0, 0)
        }
    } else {
        let previous = if number > 1 {
            window.controls.get(number as usize - 2)
        } else {
            None
        };
        match previous {
            Some(control) => the_place_of_the_control_in_the_window(window, control),
            None => (margin_x, margin_y, 0, 0),
        }
    };
    // The right and the bottom that the controls of the window reach, and
    // the ones that the controls of the section of the window reach.
    let section_at = window.section_at.unwrap_or(0);
    let mut right = 0;
    let mut down = 0;
    let mut right_of_the_section = 0;
    let mut down_of_the_section = 0;
    for (index, control) in window.controls.iter().enumerate() {
        if index + 1 >= number.max(1) as usize {
            break;
        }
        let (x, y, width, height) = the_place_of_the_control_in_the_window(window, control);
        right = right.max(x + width);
        down = down.max(y + height);
        if index >= section_at {
            right_of_the_section = right_of_the_section.max(x + width);
            down_of_the_section = down_of_the_section.max(y + height);
        }
    }
    // The place of the section of the window: the `xs` and the `ys` of the
    // options are the place of it.
    let (section_x, section_y) = window.section.unwrap_or((margin_x, margin_y));
    let (x, y) = match (place.x, place.y) {
        // The v1: the control that has no place goes below the control
        // before it, with the margin of the window between the two.
        (None, None) => (
            previous_x,
            previous_y + previous_height + margin_y,
        ),
        (Some(coordinate), None) => {
            let x = the_x_of_the_coordinate(
                coordinate,
                previous_x,
                previous_width,
                margin_x,
                section_x,
            );
            // A place that moves the control in the X direction keeps the Y
            // of the control before it.
            let y = match coordinate {
                TheCoordinate::AfterThePrevious(_) | TheCoordinate::FromThePrevious(_) => previous_y,
                _ => down + margin_y,
            };
            (x, y)
        }
        (None, Some(coordinate)) => {
            let y = the_y_of_the_coordinate(
                coordinate,
                previous_y,
                previous_height,
                margin_y,
                section_y,
            );
            let x = match coordinate {
                TheCoordinate::AfterThePrevious(_) | TheCoordinate::FromThePrevious(_) => previous_x,
                _ => right + margin_x,
            };
            (x, y)
        }
        (Some(x_coordinate), Some(y_coordinate)) => (
            the_x_of_the_coordinate(
                x_coordinate,
                previous_x,
                previous_width,
                margin_x,
                section_x,
            ),
            the_y_of_the_coordinate(
                y_coordinate,
                previous_y,
                previous_height,
                margin_y,
                section_y,
            ),
        ),
    };
    let width = match place.width {
        Some(TheSize::Absolute(value)) => Some(value),
        Some(TheSize::FromThePrevious(offset)) => Some(previous_width + offset),
        None => None,
    };
    let height = match place.height {
        Some(TheSize::Absolute(value)) => Some(value),
        Some(TheSize::FromThePrevious(offset)) => Some(previous_height + offset),
        None => None,
    };
    (x, y, width, height)
}

/// The X of a control, the way one of the options of the v1 asks it.
fn the_x_of_the_coordinate(
    coordinate: TheCoordinate,
    previous_x: i64,
    previous_width: i64,
    margin_x: i64,
    section_x: i64,
) -> i64 {
    match coordinate {
        TheCoordinate::Absolute(value) => value,
        TheCoordinate::AfterThePrevious(offset) => previous_x + previous_width + offset,
        TheCoordinate::AfterThePreviousWithTheMargin => previous_x + previous_width + margin_x,
        TheCoordinate::FromTheMargin(offset) => margin_x + offset,
        TheCoordinate::FromThePrevious(offset) => previous_x + offset,
        TheCoordinate::FromTheSection(offset) => section_x + offset,
    }
}

/// The Y of a control, the way one of the options of the v1 asks it.
fn the_y_of_the_coordinate(
    coordinate: TheCoordinate,
    previous_y: i64,
    previous_height: i64,
    margin_y: i64,
    section_y: i64,
) -> i64 {
    match coordinate {
        TheCoordinate::Absolute(value) => value,
        TheCoordinate::AfterThePrevious(offset) => previous_y + previous_height + offset,
        TheCoordinate::AfterThePreviousWithTheMargin => previous_y + previous_height + margin_y,
        TheCoordinate::FromTheMargin(offset) => margin_y + offset,
        TheCoordinate::FromThePrevious(offset) => previous_y + offset,
        TheCoordinate::FromTheSection(offset) => section_y + offset,
    }
}

/// The class of a control, the way the v2 names it.
///
/// The name is the class of the machine and the number of the control
/// among the ones of that class: "Button1" is the class "Button" and the
/// first control of it.
pub fn the_classnn_of(window: &TheWindow, number: u64) -> String {
    let Some(control) = window.controls.get(number.max(1) as usize - 1) else {
        return String::new();
    };
    // The ClassNN counts by the class of the machine, and not by the
    // type: the Button and the CheckBox are both of the class "Button",
    // and the Text is the "Static". Measured on the binary of the
    // v2.0.31: the `Add("Button")` and the `Add("CheckBox")` give
    // "Button1" and "Button2".
    let mut seen = 0u64;
    for other in &window.controls {
        if other.class == control.class {
            seen += 1;
            if other.handle == control.handle {
                return format!("{}{}", control.class, seen);
            }
        }
    }
    format!("{}{}", control.class, seen)
}

/// The control of a number, when the window has it.
pub fn the_control(window: &TheWindow, number: u64) -> Option<&TheControl> {
    window.controls.get(number.max(1) as usize - 1)
}

/// The control of a window that a name names.
///
/// The v1 names a control by the variable that it is bound to, and by the
/// `ClassNN` of it, and by the text of it. The name of the variable comes
/// first: the CHM of v1.1.37.02 says that the name of the variable "takes
/// precedence over the ones described next".
pub fn the_control_named<'a>(window: &'a TheWindow, name: &str) -> Option<&'a TheControl> {
    if name.is_empty() {
        return None;
    }
    // The name of the variable comes first.
    for control in &window.controls {
        if !control.name.is_empty() && control.name.eq_ignore_ascii_case(name) {
            return Some(control);
        }
    }
    // The ClassNN comes next.
    for (index, control) in window.controls.iter().enumerate() {
        let classnn = the_classnn_of(window, index as u64 + 1);
        if classnn.eq_ignore_ascii_case(name) {
            return Some(control);
        }
    }
    None
}

/// The control of a window that a name names, by its text.
///
/// The text of a control is the way the v1 names it when it has no
/// variable and no ClassNN that matches.
pub fn the_control_with_the_text<'a>(
    window: &'a TheWindow,
    name: &str,
    matches: impl Fn(&str, &str) -> bool,
) -> Option<&'a TheControl> {
    if name.is_empty() {
        return None;
    }
    window
        .controls
        .iter()
        .find(|control| matches(&control.text, name))
}

/// The tab of a window is chosen.
///
/// Auto-fit a `SysTabControl32` so its bounds enclose all child controls
/// assigned to any of its pages (both visible and hidden).
fn auto_fit_tab_control(window: &TheWindow, tab_hwnd: isize) {
    let Some(tab_ctrl) = window
        .controls
        .iter()
        .find(|c| c.handle as isize == tab_hwnd)
    else {
        return;
    };
    let children: Vec<isize> = THE_TAB_CONTROLS
        .lock()
        .ok()
        .and_then(|table| {
            table
                .iter()
                .find(|(h, _)| *h == tab_hwnd)
                .map(|(_, list)| list.iter().map(|(ch, _)| *ch).collect())
        })
        .unwrap_or_default();
    if children.is_empty() {
        return;
    }
    let (tx, ty, tw, th) = the_place_of_the_control_in_the_window(window, tab_ctrl);
    let mut max_right = tx + tw;
    let mut max_bottom = ty + th;
    for control in &window.controls {
        if !children.contains(&(control.handle as isize)) {
            continue;
        }
        let (cx, cy, cw, ch) = the_place_of_the_control_in_the_window(window, control);
        max_right = max_right.max(cx + cw + window.margin.0);
        max_bottom = max_bottom.max(cy + ch + window.margin.1);
    }
    let new_w = (max_right - tx).max(tw);
    let new_h = (max_bottom - ty).max(th);
    if new_w != tw || new_h != th {
        the_place_of_a_control_is_written(
            tab_ctrl,
            ThePlace {
                x: tx,
                y: ty,
                width: new_w,
                height: new_h,
            },
        );
    }
}

/// The v1: `Gui, Tab, N` associates subsequent controls with tab page `N`
/// (1-based) of the most recent tab control; `Gui, Tab` (with no argument,
/// `tab <= 0`) clears the tab association so subsequent controls sit outside
/// any tab control.
pub fn the_tab_is_chosen(window: &mut TheWindow, tab: i64) -> Result<(), String> {
    if tab <= 0 {
        if let Some(tab_hwnd) = window.current_tab_control {
            auto_fit_tab_control(window, tab_hwnd);
            window.layout_anchor_control = Some(tab_hwnd);
        }
        window.current_tab_page = None;
        window.tab_just_switched = false;
        return Ok(());
    }
    if window.current_tab_control.is_none() {
        if let Some(ctrl) = window
            .controls
            .iter()
            .rev()
            .find(|control| control.kind.eq_ignore_ascii_case("tab"))
        {
            window.current_tab_control = Some(ctrl.handle as isize);
        } else {
            return Err("the window has no tab control".to_string());
        }
    }
    window.current_tab_page = Some(tab as usize);
    window.tab_just_switched = true;
    window.layout_anchor_control = None;
    Ok(())
}

/// The colour of the background of a window is written.
///
/// The colour of the v1 is a number of six digits, and the machine takes
/// it as the colour of the brush of the window.
///
/// Measured on the binary of v1.1.37.02: the `Gui, Color,
/// 212121` of the toolbar of a script of the v1 paints the window
/// with that colour, and the buttons that are in it keep their own image.
/// The v1 gives the colour to each window (source `script_gui.cpp:7596` of
/// v1.1.37.02), and the procedure of the window paints it when the
/// machine asks for the background.
pub fn the_background_is_written(window: &mut TheWindow, colour: u32) {
    window.background = Some(colour);
    if let Ok(mut table) = THE_BACKGROUNDS.lock() {
        table.retain(|(handle, _)| *handle != window.handle as isize);
        table.push((window.handle as isize, colour));
    }
    unsafe {
        InvalidateRect(window.handle, std::ptr::null(), 1);
        RedrawWindow(
            window.handle,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            RDW_INVALIDATE | RDW_UPDATENOW | RDW_ERASE,
        );
    }
}

/// The colour of the background of each window of the script.
///
/// The handle of the window is the key, and the colour is the one that
/// the script gave to it with the `Gui, Color`.
static THE_BACKGROUNDS: std::sync::Mutex<Vec<(isize, u32)>> = std::sync::Mutex::new(Vec::new());

/// The colour of the background of a window, when it has one.
fn the_background_of(window: isize) -> Option<u32> {
    THE_BACKGROUNDS
        .lock()
        .ok()
        .and_then(|table| {
            table
                .iter()
                .find(|(handle, _)| *handle == window)
                .map(|(_, colour)| *colour)
        })
}

/// The size that a window of the script may not go under, and the one
/// that it may not go over.
///
/// The v1: the "MinSize" and the "MaxSize" of the `Gui` are the size of
/// the client area of the window, and the machine keeps the window
/// inside them: the window of a script of the v1 asks for a size of
/// 960x540, and the machine does not let it go under that.
///
/// The handle of the window is the key, and the sizes are the ones that
/// the script gave to it.
static THE_SIZES: std::sync::Mutex<Vec<(isize, TheSizes)>> = std::sync::Mutex::new(Vec::new());

/// The size of a window of the script: a width and a height.
type TheSizeOfAWindow = (i64, i64);

/// The size that a window of the script may not go under, and the one
/// that it may not go over.
type TheSizes = (Option<TheSizeOfAWindow>, Option<TheSizeOfAWindow>);

/// The size of a window of the script is remembered, for the procedure of
/// the window.
fn the_size_is_remembered(
    window: &mut TheWindow,
    minimum: Option<(i64, i64)>,
    maximum: Option<(i64, i64)>,
) {
    if let Ok(mut table) = THE_SIZES.lock() {
        table.retain(|(handle, _)| *handle != window.handle as isize);
        table.push((window.handle as isize, (minimum, maximum)));
    }
}

/// The sizes of a window of the script, when it has some.
fn the_sizes_of(window: isize) -> Option<TheSizes> {
    THE_SIZES
        .lock()
        .ok()
        .and_then(|table| {
            table
                .iter()
                .find(|(handle, _)| *handle == window)
                .map(|(_, (minimum, maximum))| (*minimum, *maximum))
        })
}

/// The size of a window of the script is written into the answer of the
/// machine, when the machine asks for it.
///
/// The v1: the window asks the machine the size that it may have, and
/// the answer of it is the `WM_GETMINMAXINFO`. The sizes of the script
/// are the size of the client area of the window, and the machine wants
/// the size of the window, and the border is the difference between the
/// two.
fn the_size_is_written_to_the_machine(window: HWND, lparam: LPARAM) {
    let Some((minimum, maximum)) = the_sizes_of(window as isize) else {
        return;
    };
    if minimum.is_none() && maximum.is_none() {
        return;
    }
    unsafe {
        let answer = lparam as *mut MINMAXINFO;
        if answer.is_null() {
            return;
        }
        let style = GetWindowLongPtrW(window, GWL_STYLE) as u32;
        let extended = GetWindowLongPtrW(window, GWL_EXSTYLE) as u32;
        let mut rectangle = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        AdjustWindowRectEx(&mut rectangle, style, 0, extended);
        let border_width = rectangle.right - rectangle.left;
        let border_height = rectangle.bottom - rectangle.top;
        if let Some((width, height)) = minimum {
            (*answer).ptMinTrackSize.x = width as i32 + border_width;
            (*answer).ptMinTrackSize.y = height as i32 + border_height;
        }
        if let Some((width, height)) = maximum {
            (*answer).ptMaxTrackSize.x = width as i32 + border_width;
            (*answer).ptMaxTrackSize.y = height as i32 + border_height;
        }
    }
}

/// The margin of a window is written.
pub fn the_margin_is_written(window: &mut TheWindow, x: i64, y: i64) {
    window.margin = (x, y);
}

/// The font of the controls of a window is written.
///
/// The v1 makes the font of the machine with the `CreateFont` (source
/// `script_gui.cpp:7883` of v1.1.37.02) and gives it to every control
/// that is made after it (source `script_gui.cpp:3027`):
/// `SendMessage(control.hwnd, WM_SETFONT, (WPARAM)sFont[...].hfont, 0)`.
///
/// The height of the font is the size in points times the pixels per point
/// of the screen, divided by 72: that is what the v1 does with the
/// `MulDiv`.
///
/// Measured on the binary of v1.1.37.02: the
/// `Gui, Font, s1` of a GUI of the script gives the button of it a font
/// of one point, and the `WM_GETFONT` of the button answers the handle of
/// the font. The `WM_GETFONT` of the window answers zero: the v1 gives
/// the font to the controls and not to the window.
/// Reset the window's font to the system default GUI font (`Gui, Font` with no args).
pub fn the_font_is_reset(window: &mut TheWindow) {
    window.font = None;
    window.font_handle = Some(the_default_font_of_the_machine());
}

pub fn the_font_is_written(window: &mut TheWindow, font: TheFont) {
    window.font = Some(font);
    let Some(font) = window.font.clone() else {
        return;
    };
    let handle = the_font_of_the_machine(&font);
    if handle == 0 {
        return;
    }
    window.font_handle = Some(handle);
}

/// The font of the machine, and the handle of it.
///
/// The name that is not a font of the machine is the font of the system:
/// the v1 falls back to the typeface of the font of the base (source
/// `script_gui.cpp:7862-7863`).
fn the_font_of_the_machine(font: &TheFont) -> isize {
    let weight = if font.bold { 700 } else { 400 };
    // The pixels per point of the screen: the v1 reads them with the
    // `GetDeviceCaps` of the desktop (source `script_gui.cpp:7858`).
    let pixels_per_point = unsafe {
        let dc = GetDC(std::ptr::null_mut());
        if dc.is_null() {
            96
        } else {
            let value = GetDeviceCaps(dc, LOGPIXELSY as i32) as i64;
            ReleaseDC(std::ptr::null_mut(), dc);
            if value <= 0 {
                96
            } else {
                value
            }
        }
    };
    // The name and the size that the options do not name are the ones of
    // the default font of the machine.
    let (default_name, default_height) = the_name_and_the_height_of_the_default_font();
    let height = if font.size <= 0 {
        default_height
    } else {
        -((font.size * pixels_per_point).div_euclid(72)) as i32
    };
    let name = if font.name.is_empty() {
        default_name
    } else {
        font.name.clone()
    };
    let name = Wide::of(&name);
    unsafe {
        CreateFontW(
            height,
            0,
            0,
            0,
            weight,
            if font.italic { 1 } else { 0 },
            if font.underline { 1 } else { 0 },
            if font.strike { 1 } else { 0 },
            DEFAULT_CHARSET as u32,
            OUT_TT_PRECIS as u32,
            CLIP_DEFAULT_PRECIS as u32,
            DEFAULT_QUALITY as u32,
            FF_DONTCARE as u32,
            name.as_ptr(),
        ) as isize
    }
}

/// The place and the size of a control are written.
///
/// The v1 makes the control with the size that it has, and the order of the
/// Z of the control is the order in which it was made. The machine makes the
/// control before it writes the place of it, and the place cannot change the
/// order of the Z (source `script_gui.cpp:3107-3108` of v1.1.37.02).
pub fn the_place_of_a_control_is_written(control: &TheControl, place: ThePlace) {
    unsafe {
        SetWindowPos(
            control.handle,
            std::ptr::null_mut(),
            place.x as i32,
            place.y as i32,
            place.width.max(1) as i32,
            place.height.max(1) as i32,
            SWP_NOZORDER,
        );
    }
}

/// The handle of the machine of a control.
///
/// The number that names a control of the tree is not the handle of the
/// machine: a script that asks for the handle wants the number that the
/// machine of it knows.
pub fn the_handle_of(control: &TheControl) -> i64 {
    control.handle as i64
}

/// The text of a control.
pub fn the_text_of(control: &TheControl) -> String {
    unsafe {
        let length = GetWindowTextLengthW(control.handle);
        let mut buffer = vec![0u16; length as usize + 1];
        let read = GetWindowTextW(control.handle, buffer.as_mut_ptr(), buffer.len() as i32);
        if read <= 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buffer[..read as usize])
    }
}

/// Populate or append `delimiter`-separated items into a `ComboBox` or
/// `ListBox`, honoring leading-delimiter clear (`replace_if_leading_delim`),
/// double-delimiter selection (`Item||`), and optional 1-based `choose`.
fn populate_list_items(
    control: &TheControl,
    text: &str,
    delimiter: char,
    choose: Option<i64>,
    replace_if_leading_delim: bool,
) {
    let is_combo = control.class == "ComboBox";
    let is_list = control.class == "ListBox";
    if !is_combo && !is_list {
        return;
    }
    let mut rest = text;
    if replace_if_leading_delim {
        if let Some(stripped) = rest.strip_prefix(delimiter) {
            unsafe {
                let reset_msg = if is_combo {
                    CB_RESETCONTENT
                } else {
                    LB_RESETCONTENT
                };
                SendMessageW(control.handle, reset_msg, 0, 0);
            }
            rest = stripped;
            if rest.is_empty() {
                return;
            }
        }
    }
    if rest.is_empty() {
        return;
    }
    let mut selected_index: Option<isize> = None;
    let mut chars = rest.char_indices().peekable();
    let mut start = 0usize;
    let mut pending_end = true;
    while let Some((idx, ch)) = chars.next() {
        if ch == delimiter {
            let item = &rest[start..idx];
            let double_delim = chars.peek().is_some_and(|&(_, next_ch)| next_ch == delimiter);
            if double_delim {
                chars.next();
            }
            // Skip a leading empty token produced by `||` at the very start.
            if !(item.is_empty() && start == 0 && double_delim) {
                let wide = Wide::of(item);
                let add_msg = if is_combo { CB_ADDSTRING } else { LB_ADDSTRING };
                let added = unsafe {
                    SendMessageW(control.handle, add_msg, 0, wide.as_ptr() as LPARAM)
                };
                if double_delim && added >= 0 {
                    selected_index = Some(added);
                }
            }
            start = match chars.peek() {
                Some(&(next_idx, _)) => next_idx,
                None => {
                    pending_end = false;
                    rest.len()
                }
            };
        }
    }
    if pending_end && start <= rest.len() {
        let item = &rest[start..];
        if !item.is_empty() || start == 0 {
            let wide = Wide::of(item);
            let add_msg = if is_combo { CB_ADDSTRING } else { LB_ADDSTRING };
            unsafe {
                SendMessageW(control.handle, add_msg, 0, wide.as_ptr() as LPARAM);
            }
        }
    }
    let select = if let Some(n) = choose.filter(|&n| n >= 1) {
        Some((n - 1) as isize)
    } else {
        selected_index
    };
    if let Some(sel) = select {
        let sel_msg = if is_combo { CB_SETCURSEL } else { LB_SETCURSEL };
        unsafe {
            SendMessageW(control.handle, sel_msg, sel as WPARAM, 0);
        }
    }
}

/// The text or value of a control is written (`GuiControl, , Ctrl, Value`).
pub fn the_text_is_written(control: &TheControl, text: &str) {
    match control.class.as_str() {
        "ComboBox" | "ListBox" => {
            populate_list_items(control, text, '|', None, true);
            return;
        }
        "SysTabControl32" => {
            const TCM_INSERTITEMW: u32 = 0x133E;
            const TCIF_TEXT: u32 = 0x0001;
            #[repr(C)]
            struct TcItemW {
                mask: u32,
                dw_state: u32,
                dw_state_mask: u32,
                psz_text: *mut u16,
                cch_text_max: i32,
                i_image: i32,
                l_param: isize,
            }
            let mut rest = text;
            if let Some(stripped) = rest.strip_prefix('|') {
                unsafe {
                    SendMessageW(control.handle, TCM_DELETEALLITEMS, 0, 0);
                }
                rest = stripped;
                if rest.is_empty() {
                    return;
                }
            }
            let base = unsafe { SendMessageW(control.handle, 0x1304, 0, 0).max(0) as usize };
            for (offset, label) in rest.split('|').enumerate() {
                if label.is_empty() {
                    continue;
                }
                let mut wide_label = Wide::of(label);
                let item = TcItemW {
                    mask: TCIF_TEXT,
                    dw_state: 0,
                    dw_state_mask: 0,
                    psz_text: wide_label.as_mut_ptr(),
                    cch_text_max: 0,
                    i_image: -1,
                    l_param: 0,
                };
                unsafe {
                    SendMessageW(
                        control.handle,
                        TCM_INSERTITEMW,
                        base + offset,
                        &item as *const TcItemW as LPARAM,
                    );
                }
            }
            return;
        }
        _ => {}
    }
    match control.kind.as_str() {
        "checkbox" | "radio" => match text.trim() {
            "0" => {
                the_state_of_the_v1_is_written(control, 0);
                return;
            }
            "1" => {
                the_state_of_the_v1_is_written(control, 1);
                return;
            }
            "-1" => {
                the_state_of_the_v1_is_written(control, -1);
                return;
            }
            _ => {}
        },
        "progress" => {
            let trimmed = text.trim();
            if let Some(delta) = trimmed.strip_prefix('+').and_then(|s| s.parse::<i32>().ok()) {
                unsafe {
                    SendMessageW(control.handle, PBM_DELTAPOS, delta as WPARAM, 0);
                }
                return;
            }
            if let Ok(pos) = trimmed.parse::<i32>() {
                unsafe {
                    if trimmed.starts_with('-') {
                        SendMessageW(control.handle, PBM_DELTAPOS, pos as WPARAM, 0);
                    } else {
                        SendMessageW(control.handle, PBM_SETPOS, pos as WPARAM, 0);
                    }
                }
                return;
            }
        }
        "slider" => {
            if let Ok(pos) = text.trim().parse::<i32>() {
                unsafe {
                    SendMessageW(control.handle, TBM_SETPOS, 1, pos as LPARAM);
                }
                return;
            }
        }
        "updown" => {
            if let Ok(pos) = text.trim().parse::<i32>() {
                unsafe {
                    SendMessageW(control.handle, UDM_SETPOS32, 0, pos as LPARAM);
                }
                return;
            }
        }
        "picture" => {
            let (_, _, w, h) = the_place_of(control);
            let mut updated = control.clone();
            updated.text = text.to_string();
            the_picture_is_loaded(
                &updated,
                if w > 0 { Some(w) } else { None },
                if h > 0 { Some(h) } else { None },
            );
            return;
        }
        _ => {}
    }
    let wide = Wide::of(text);
    unsafe {
        SendMessageW(control.handle, WM_SETTEXT, 0, wide.as_ptr() as LPARAM);
    }
}

/// The class of the machine of a control.
pub fn the_class_of(control: &TheControl) -> String {
    unsafe {
        let mut buffer = [0u16; 256];
        let read = GetClassNameW(control.handle, buffer.as_mut_ptr(), buffer.len() as i32);
        if read <= 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buffer[..read as usize])
    }
}

/// The window is shown or hidden.
pub fn the_window_is_shown(window: &mut TheWindow, show: bool) {
    if show {
        window.shown = true;
    }
    if show {
        unsafe {
            ShowWindow(window.handle, SW_SHOW);
        }
        return;
    }
    // The window is hidden with `SetWindowPos` and not with
    // `ShowWindow`: the first call of `ShowWindow` of a window takes the
    // state of the `STARTUPINFO` of the process and not the one of the
    // call, and the window would come to the screen when the script
    // hides it.
    unsafe {
        SetWindowPos(
            window.handle,
            HWND_TOP,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_HIDEWINDOW,
        );
    }
}

/// The window is shown without being activated.
///
/// The "NoActivate" and the "NA" of the `Show` of the v1 show the window
/// without making it the one of the thread. Measured on the binary of
/// v1.1.37.02: the `Gui, PVthird: Show, NA` is accepted.
///
/// The v1: "case 'N': if (!_tcsnicmp(cp, _T("NA"), 2)) { show_mode =
/// SW_SHOWNA; } else if (!_tcsnicmp(cp, _T("NoActivate"), 10)) {
/// show_mode = SW_SHOWNOACTIVATE; }" (`script_gui.cpp:6615-6624` of
/// v1.1.37.02). The `SW_SHOWNA` gives the window the focus of its
/// thread, and the `SW_SHOWNOACTIVATE` does not give it any focus at
/// all: the window that opens itself while the user is in a menu takes
/// the menu away from the user with it.
///
/// The window is shown with `SetWindowPos` and not with `ShowWindow`:
/// the first call of `ShowWindow` of a window takes the state of the
/// `STARTUPINFO` of the process and not the one of the call.
pub fn the_window_is_shown_without_activating(window: &mut TheWindow) {
    window.shown = true;
    unsafe {
        SetWindowPos(
            window.handle,
            HWND_TOP,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }
}

/// The place of a control in the client area of the window that holds it.
///
/// The place of the machine is the place of the screen, and the place of a
/// control in the window of the script is the place in the client area of
/// it. The v1 measures the controls that way when it sizes a window: the
/// source of the v1 (`script_gui.cpp:6743-6744`) takes the rectangle of the
/// control and maps it to the client area of the window.
pub fn the_place_of_the_control_in_the_window(
    window: &TheWindow,
    control: &TheControl,
) -> (i64, i64, i64, i64) {
    unsafe {
        let mut rectangle = std::mem::zeroed();
        if GetWindowRect(control.handle, &mut rectangle) == 0 {
            return (0, 0, 0, 0);
        }
        let mut points = [
            POINT {
                x: rectangle.left,
                y: rectangle.top,
            },
            POINT {
                x: rectangle.right,
                y: rectangle.bottom,
            },
        ];
        MapWindowPoints(
            std::ptr::null_mut(),
            window.handle,
            points.as_mut_ptr(),
            2,
        );
        (
            points[0].x as i64,
            points[0].y as i64,
            (points[1].x - points[0].x) as i64,
            (points[1].y - points[0].y) as i64,
        )
    }
}

/// The size that the window takes when the script does not give one.
///
/// The size is the size of the client area, and it is the size of the
/// controls that the window holds: the place that they reach, plus the
/// margin of the window around them.
///
/// Measured on the binary of AutoHotkey v1.1.37.02
/// (the probe of the measurement): the `Gui, B: Show` of a window that holds
/// a button of 30x20 at 0,0 and a button of 30x20 at 40,30 gives a client
/// area of 70x50. The source of the v1 (`script_gui.cpp:6767-6769`) reads
/// the size of the window from the extents of the controls when the script
/// does not give one.
pub fn the_size_of_the_controls(window: &TheWindow) -> (i64, i64) {
    let mut width = 0i64;
    let mut height = 0i64;
    for control in &window.controls {
        let (x, y, control_width, control_height) =
            the_place_of_the_control_in_the_window(window, control);
        width = width.max(x + control_width);
        height = height.max(y + control_height);
    }
    // The v1: the place that the controls reach, and the margin of the
    // window around them. The margin is there only when the window holds
    // a control, and the window without controls has no size at all
    // (source `script_gui.cpp:6751-6754` of v1.1.37.02).
    //
    // Measured on the binary of v1.1.37.02: the window
    // without controls is of 0x0, and the window with a button of 30x20
    // at 0,0 and a button of 30x20 at 40,30 is of 70x50.
    let (margin_x, margin_y) = window.margin;
    if width > 0 {
        width += margin_x;
    }
    if height > 0 {
        height += margin_y;
    }
    (width, height)
}

/// The window is made as big as the controls that it holds.
///
/// The "AutoSize" of the `Show` of the v1 resizes the window to
/// accommodate the controls that are visible in it. The place of the
/// window is not touched: the "AutoSize Center" of the v1 keeps the
/// window in the middle of the screen.
///
/// Measured on the binary of v1.1.37.02: the `Gui,
/// PVother: Show, NoActivate AutoSize, The other` is accepted.
pub fn the_window_is_sized_to_its_controls(window: &TheWindow) {
    // Ensure any tab controls auto-fit all of their child pages (visible or
    // hidden) before measuring the enclosing window dimensions.
    for control in &window.controls {
        if control.class == "SysTabControl32" {
            auto_fit_tab_control(window, control.handle as isize);
        }
    }
    // The place of the controls that are seen: the window is as big as
    // them, and the margin of the window is around them.
    let mut width = 0i64;
    let mut height = 0i64;
    for control in &window.controls {
        if !the_control_is_visible(control) {
            continue;
        }
        let (x, y, control_width, control_height) =
            the_place_of_the_control_in_the_window(window, control);
        width = width.max(x + control_width);
        height = height.max(y + control_height);
    }
    // The margin of the window is around the controls of it, and it is
    // there once (source `script_gui.cpp:6751-6754` of v1.1.37.02).
    let (margin_x, margin_y) = window.margin;
    if width > 0 {
        width += margin_x;
    }
    if height > 0 {
        height += margin_y;
    }
    let _ = the_place_is_given(window, None, None, Some(width), Some(height));
}

/// The size of the client area of the window of the script.
///
/// The v1: the label `<name>GuiSize` of the script is called with
/// the width and the height of the client area of the window.
pub fn the_client_size_of(window: &TheWindow) -> (i64, i64) {
    unsafe {
        let mut rectangle = std::mem::zeroed();
        if GetClientRect(window.handle, &mut rectangle) == 0 {
            return (0, 0);
        }
        (
            (rectangle.right - rectangle.left) as i64,
            (rectangle.bottom - rectangle.top) as i64,
        )
    }
}

/// The size of the window of the script, the width and the height of it.
pub fn the_width_of(window: &TheWindow) -> i64 {
    unsafe {
        let mut rectangle = std::mem::zeroed();
        if GetWindowRect(window.handle, &mut rectangle) == 0 {
            return 0;
        }
        (rectangle.right - rectangle.left) as i64
    }
}

/// The height of the window of the script.
pub fn the_height_of(window: &TheWindow) -> i64 {
    unsafe {
        let mut rectangle = std::mem::zeroed();
        if GetWindowRect(window.handle, &mut rectangle) == 0 {
            return 0;
        }
        (rectangle.bottom - rectangle.top) as i64
    }
}

/// The place and the size of the window of the script.
///
/// The ones that are not there are the ones the window has.
///
/// The size that the script gives is the size of the **client area** of
/// the window, and the window is bigger: the border and the caption are
/// around it.
///
/// Measured on the binary of v1.1.37.02
/// (the probe of the measurement): the `Gui, Show, w400 h300` gives a client area
/// of 400x300 and a window of 406x329.
/// The place and the size of the window of the script.
///
/// The window is not made the one of the thread: the v1 moves it with
/// `MoveWindow`, which does not activate it, and the window of the
/// script that opens itself while the user is in a menu takes the menu
/// away from the user with it.
///
/// The v1: "MoveWindow(mHwnd, ... , is_visible); // Do repaint if window
/// is visible" (`script_gui.cpp:6952-6953` of v1.1.37.02).
pub fn the_place_is_given(
    window: &TheWindow,
    x: Option<i64>,
    y: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
) -> Result<(), String> {
    let mut flags = SWP_NOACTIVATE;
    if x.is_none() && y.is_none() {
        flags |= SWP_NOMOVE;
    }
    // The window without a size of its own is not resized: the zero is
    // a size that the machine does not take, and the v1 leaves the window
    // as it is (source `script_gui.cpp:6763-6786` of v1.1.37.02).
    //
    // Measured on the binary of v1.1.37.02: the window
    // without controls is of 0x0, and the tree made it of 16x39 -- the
    // smallest that the machine makes of a window.
    if width.unwrap_or(0) <= 0 && height.unwrap_or(0) <= 0 {
        flags |= SWP_NOSIZE;
    }
    // The size that the machine takes is the size of the window, and the
    // size of the script is the size of the client area of it.
    let (width, height) = the_size_of_the_window(window, width, height);
    let placed = unsafe {
        SetWindowPos(
            window.handle,
            std::ptr::null_mut(),
            x.unwrap_or(0) as i32,
            y.unwrap_or(0) as i32,
            width.unwrap_or(0) as i32,
            height.unwrap_or(0) as i32,
            flags,
        )
    };
    if placed == 0 {
        let code = unsafe { windows_sys::Win32::Foundation::GetLastError() };
        return Err(format!("the machine did not place the window: {code}."));
    }
    Ok(())
}

/// The size of the window, from the size of the client area of it.
///
/// The v1: the `Gui, Show` gives the size of the client area, and the
/// window is that area with the border and the caption around it. The
/// source of the v1 (`script_gui.cpp`) makes it with the
/// `AdjustWindowRectEx` of the machine.
pub(crate) fn the_size_of_the_window(
    window: &TheWindow,
    width: Option<i64>,
    height: Option<i64>,
) -> (Option<i64>, Option<i64>) {
    if width.is_none() && height.is_none() {
        return (width, height);
    }
    unsafe {
        let style = the_style_of(window.handle);
        let extended = GetWindowLongPtrW(window.handle, GWL_EXSTYLE) as u32;
        // The v1: the window that has a menu bar has the bar of it in
        // the area that is not the client area, and the client area keeps
        // the size that the script gave to it. The machine takes the room
        // of the bar away from the client area when the bar is attached,
        // and the window is made bigger by that room, so that the client
        // area is the one of the script.
        //
        // The source of the v1 (`script_gui.cpp:6826-6832` of v1.1.37.02)
        // asks the machine for the size of the window with the
        // `has_menu` of the window.
        //
        // Measured on the binary of v1.1.37.02: the client
        // area of the main window of a script of the v1 is of
        // 960x520, and the tree made it of 960x540: the bar of the menu
        // ate the client area instead of growing the window.
        let has_menu = !GetMenu(window.handle).is_null();
        let new_width = width.map(|width| {
            let mut rectangle = RECT {
                left: 0,
                top: 0,
                right: width as i32,
                bottom: 0,
            };
            AdjustWindowRectEx(&mut rectangle, style, has_menu as i32, extended);
            (rectangle.right - rectangle.left) as i64
        });
        let new_height = height.map(|height| {
            let mut rectangle = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: height as i32,
            };
            AdjustWindowRectEx(&mut rectangle, style, has_menu as i32, extended);
            // The v1: the window that has a menu bar has the room of
            // the bar in the area that is not the client area, and the
            // machine is asked for that room with the WM_NCCALCSIZE.
            //
            // The source of the v1 (`script_gui.cpp:6840-6847` of
            // v1.1.37.02) sends the message with the rect of the window
            // and adds the top that the window answers to it.
            //
            // Measured on the binary of v1.1.37.02: a
            // window with the bar of the menu and a client area of
            // 110x106 is of 116x174 in the v1, and the room of the bar
            // of the menu is the twenty pixels that the machine asks
            // for plus the nineteen that it adds on its own.
            if has_menu {
                let mut room = rectangle;
                room.bottom = 0x7FFF;
                SendMessageW(
                    window.handle,
                    WM_NCCALCSIZE,
                    0,
                    &mut room as *mut RECT as LPARAM,
                );
                rectangle.bottom += room.top;
            }
            (rectangle.bottom - rectangle.top) as i64
        });
        (new_width, new_height)
    }
}

pub fn the_window_is_closed(window: &TheWindow) {
    unsafe {
        DestroyWindow(window.handle);
    }
    // The window is gone, and what the script gave to it goes with it:
    // the machine gives the handle of the window to another one, and the
    // new window does not inherit the background of the old one.
    if let Ok(mut table) = THE_BACKGROUNDS.lock() {
        table.retain(|(handle, _)| *handle != window.handle as isize);
    }
    if let Ok(mut table) = THE_SIZES.lock() {
        table.retain(|(handle, _)| *handle != window.handle as isize);
    }
    if let Ok(mut table) = THE_BAR_CALLBACKS.lock() {
        table.retain(|(handle, _)| *handle != window.handle as isize);
    }
}

/// The option of the window of the script is applied.
///
/// The options are the ones that the CHM of the v1 lists for a window:
/// the "alwaysontop", the "border", the "caption", the "disabled", the
/// "activate", the "minimize", the "maximize", the "restore", the
/// "movebottom", the "minimizebox", the "maximizebox", the "sysmenu",
/// the "toolwindow" and the "resize". The option that is not one of them
/// is refused with the reason.
pub fn the_option_is_applied(window: &TheWindow, name: &str, on: bool) -> Result<(), String> {
    let handle = window.handle;
    match name.trim().to_ascii_lowercase().as_str() {
        "alwaysontop" => {
            unsafe {
                SetWindowPos(
                    handle,
                    if on { HWND_TOPMOST } else { HWND_NOTOPMOST },
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
            }
            Ok(())
        }
        "resize" => {
            // The v1: the resize makes the window resizable, and it
            // gives the maximize button of the title bar back -- but
            // only when the window has a title bar to put it in.
            //
            // Measured on the binary of v1.1.37.02: the
            // `+Resize` of a window of the v1 takes the ordinary style
            // from `0x84CA0000` to `0x84CF0000` (the `WS_THICKFRAME`
            // and the `WS_MAXIMIZEBOX` come), and the `+Resize` of a
            // window that has no caption takes it from `0x840A0000` to
            // `0x840E0000` (the `WS_THICKFRAME` comes alone).
            let style = the_style_of(handle);
            the_style_carries(handle, WS_THICKFRAME, on);
            if on && style & WS_CAPTION != 0 {
                the_style_carries(handle, WS_MAXIMIZEBOX, true);
            }
            Ok(())
        }
        "caption" => {
            // The v1: the caption of a window is its border, and the
            // edge of the machine goes with it.
            //
            // Measured on the binary of v1.1.37.02: the
            // `-Caption` of a window of the v1 takes the `WS_CAPTION`,
            // the `WS_THICKFRAME` and the `WS_MAXIMIZEBOX` away, and
            // it takes the `WS_EX_WINDOWEDGE` and the `WS_EX_CLIENTEDGE`
            // with them: the ordinary style of the window goes from
            // `0x84CA0000` to `0x840A0000` and its extended style from
            // `0x100` to `0`.
            //
            // The `+Caption` gives the caption and the edge of the
            // border back, and nothing else: the ordinary style of the
            // window goes from `0x840A0000` to `0x84CA0000` and its
            // extended style from `0` to `0x100`. The `WS_THICKFRAME`
            // and the `WS_MAXIMIZEBOX` come back with the "Resize".
            if on {
                the_style_carries(handle, WS_CAPTION, true);
                the_extended_style_carries(handle, WS_EX_WINDOWEDGE, true);
            } else {
                the_style_carries(handle, WS_CAPTION, false);
                the_style_carries(handle, WS_THICKFRAME, false);
                the_style_carries(handle, WS_MAXIMIZEBOX, false);
                the_extended_style_carries(handle, WS_EX_WINDOWEDGE, false);
                the_extended_style_carries(handle, WS_EX_CLIENTEDGE, false);
            }
            Ok(())
        }
        "border" => {
            the_style_carries(handle, WS_BORDER, on);
            Ok(())
        }
        "sysmenu" => {
            the_style_carries(handle, WS_SYSMENU, on);
            Ok(())
        }
        "minimizebox" => {
            the_style_carries(handle, WS_MINIMIZEBOX, on);
            Ok(())
        }
        "maximizebox" => {
            the_style_carries(handle, WS_MAXIMIZEBOX, on);
            Ok(())
        }
        "toolwindow" => {
            the_extended_style_carries(handle, WS_EX_TOOLWINDOW, on);
            Ok(())
        }
        "disabled" => {
            unsafe {
                EnableWindow(handle, if on { 0 } else { 1 });
            }
            Ok(())
        }
        "activate" => {
            unsafe {
                SetForegroundWindow(handle);
            }
            Ok(())
        }
        "minimize" => {
            unsafe {
                ShowWindow(handle, SW_MINIMIZE);
            }
            Ok(())
        }
        "maximize" => {
            unsafe {
                ShowWindow(handle, SW_MAXIMIZE);
            }
            Ok(())
        }
        "restore" => {
            unsafe {
                ShowWindow(handle, SW_RESTORE);
            }
            Ok(())
        }
        "movebottom" => {
            unsafe {
                SetWindowPos(
                    handle,
                    HWND_BOTTOM,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
            }
            Ok(())
        }
        // The theme of the machine is the one of the system, and the tree
        // has no other one to offer.
        "theme" => Ok(()),
        other => Err(format!(
            "the window of the script has no option named {other}: the options of the machine are AlwaysOnTop, Border, Caption, Disabled, Activate, Minimize, Maximize, Restore, MoveBottom, MinimizeBox, MaximizeBox, SysMenu, ToolWindow, Resize and Theme."
        )),
    }
}

/// The option of the window of the script that takes a value is applied.
///
/// The options are the "minsize" and the "maxsize", whose value is the
/// size of the window as `WxH`, and the "delimiter", whose value is the
/// character that separates the contents of the controls.
pub fn the_option_with_a_value_is_applied(
    window: &mut TheWindow,
    name: &str,
    value: &str,
    _on: bool,
) -> Result<(), String> {
    match name.trim().to_ascii_lowercase().as_str() {
        "minsize" => {
            window.min_size = the_size_of_the_v1(value);
            the_size_is_remembered(window, window.min_size, window.max_size);
            Ok(())
        }
        "maxsize" => {
            window.max_size = the_size_of_the_v1(value);
            the_size_is_remembered(window, window.min_size, window.max_size);
            Ok(())
        }
        "delimiter" => {
            let value = value.trim();
            let character = match value.to_ascii_lowercase().as_str() {
                "space" => ' ',
                "tab" => '\t',
                "" => '|',
                other => other.chars().next().unwrap_or('|'),
            };
            window.delimiter = character;
            Ok(())
        }
        other => Err(format!(
            "the window of the script has no option named {other} that takes a value: the \
             options of the machine that take one are MinSize, MaxSize and Delimiter."
        )),
    }
}

/// The size of the window of the v1, out of the text that the script wrote.
///
/// The v1 writes the size as the width, the letter `x` and the height, and
/// the size that is not there is the one that the window has.
fn the_size_of_the_v1(text: &str) -> Option<(i64, i64)> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let (width, height) = text.split_once(['x', 'X'])?;
    let width = width.trim().parse::<i64>().ok()?;
    let height = height.trim().parse::<i64>().ok()?;
    Some((width, height))
}

/// The style without a name of the window of the script is applied.
///
/// The v1 writes the ordinary style with a sign and a number, and the
/// extended one with the letter `E` before the number.
pub fn the_style_is_applied(
    window: &TheWindow,
    extended: bool,
    style: u32,
    on: bool,
) -> Result<(), String> {
    if extended {
        the_extended_style_carries(window.handle, style, on);
    } else {
        the_style_carries(window.handle, style, on);
    }
    Ok(())
}

/// The window of the script is owned by another one, or is a child of it.
///
/// The `owner` is the window of the machine that owns it, and the
/// `parent` says that the window becomes a child of it.
///
/// Measured on the binary of v1.1.37.02: the `+Owner`
/// of a window of the v1 leaves the ordinary style of the window as it
/// is (`0x84CA0000`) and puts the handle of the other window in the
/// `GWLP_HWNDPARENT` of it; the `+Parent` of a window of the v1 takes
/// the ordinary style from `0x84CA0000` to `0x44CA0000` -- the
/// `WS_POPUP` goes away and the `WS_CHILD` comes -- and puts the handle
/// of the other window in the `GWLP_HWNDPARENT` of it as well.
///
/// The CHM of the v1.1.37.02 says of the `+Owner`: "+Owner removes the
/// WS_CHILD style and sets the WS_POPUP style", and it says of the
/// `+Parent`: "To convert the GUI back into a top-level window, use
/// -Parent".
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn the_owner_is_written(window: &TheWindow, owner: HWND, parent: bool) {
    unsafe {
        if parent {
            // The window becomes a child of the other one: the parent
            // of it is the other window, and the `WS_POPUP` goes away
            // and the `WS_CHILD` comes. The parent of a window is not
            // the `GWLP_HWNDPARENT` of it -- that is the owner of a
            // window of its own -- so the machine is asked to move it.
            SetParent(window.handle, owner);
            let style = GetWindowLongPtrW(window.handle, GWL_STYLE) as u32;
            SetWindowLongPtrW(window.handle, GWL_STYLE, ((style & !WS_POPUP) | WS_CHILD) as _);
            return;
        }
        SetWindowLongPtrW(window.handle, GWLP_HWNDPARENT, owner as _);
        // The window becomes owned by the other one: the `WS_CHILD`
        // goes away and the `WS_POPUP` comes. An owned window has no
        // button in the task bar.
        let style = GetWindowLongPtrW(window.handle, GWL_STYLE) as u32;
        SetWindowLongPtrW(window.handle, GWL_STYLE, ((style & !WS_CHILD) | WS_POPUP) as _);
    }
}

/// The owner of a window of the v1 is taken away from it.
///
/// The `-Owner` takes the owner away from the window, and the `-Parent`
/// takes the parent away from it: the window goes back to being a window
/// of the top level.
///
/// Measured on the binary of v1.1.37.02: the `-Owner` takes
/// the `GWLP_HWNDPARENT` of the window back to zero and the ordinary
/// style of it stays `0x84CA0000`; the `-Parent` takes the parent away and
/// the ordinary style goes back to `0x84CA0000` -- the `WS_POPUP` is there
/// and the `WS_CHILD` is not.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn the_owner_is_removed(window: &TheWindow, parent: bool) {
    unsafe {
        if parent {
            SetParent(window.handle, std::ptr::null_mut());
        } else {
            // The v1: a window that is a child of another one loses the
            // parent of it when the owner goes away.
            if the_style_of(window.handle) & WS_CHILD != 0 {
                SetParent(window.handle, std::ptr::null_mut());
            }
            SetWindowLongPtrW(window.handle, GWLP_HWNDPARENT, 0);
        }
        // The window goes back to being a window of the top level: the
        // `WS_CHILD` goes away and the `WS_POPUP` comes.
        let style = GetWindowLongPtrW(window.handle, GWL_STYLE) as u32;
        SetWindowLongPtrW(window.handle, GWL_STYLE, ((style & !WS_CHILD) | WS_POPUP) as _);
    }
}

/// The style of the window of the machine carries a bit, or it does not.
fn the_style_carries(handle: HWND, bit: u32, on: bool) {
    unsafe {
        let style = GetWindowLongPtrW(handle, GWL_STYLE) as u32;
        let style = if on {
            style | bit
        } else {
            style & !bit
        };
        SetWindowLongPtrW(handle, GWL_STYLE, style as _);
    }
}

/// The ordinary style of a control of the machine.
pub fn the_style_of_the_control(control: &TheControl) -> u32 {
    unsafe { GetWindowLongPtrW(control.handle, GWL_STYLE) as u32 }
}

/// The ordinary style of the window of the machine.
fn the_style_of(handle: HWND) -> u32 {
    unsafe { GetWindowLongPtrW(handle, GWL_STYLE) as u32 }
}

/// The extended style of the window of the machine carries a bit, or it
/// does not.
fn the_extended_style_carries(handle: HWND, bit: u32, on: bool) {
    unsafe {
        let style = GetWindowLongPtrW(handle, GWL_EXSTYLE) as u32;
        let style = if on {
            style | bit
        } else {
            style & !bit
        };
        SetWindowLongPtrW(handle, GWL_EXSTYLE, style as _);
    }
}

/// The title of the window is read or written.
pub fn the_title_of(window: &TheWindow) -> String {
    unsafe {
        let length = GetWindowTextLengthW(window.handle);
        let mut buffer = vec![0u16; length as usize + 1];
        let read = GetWindowTextW(window.handle, buffer.as_mut_ptr(), buffer.len() as i32);
        if read <= 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buffer[..read as usize])
    }
}

/// The title of the window is written.
pub fn the_title_is_written(window: &TheWindow, title: &str) {
    let wide = Wide::of(title);
    unsafe {
        SetWindowTextW(window.handle, wide.as_ptr());
    }
}

/// The image of a picture control goes into it, the way the v1 puts it.
///
/// The v1: the style of the control becomes the `SS_BITMAP` of a bitmap
/// or the `SS_ICON` of an icon, and the image goes into the control with
/// the `STM_SETIMAGE` (source `script_gui.cpp:6450-6472` of v1.1.37.02).
///
/// A picture that cannot be loaded is not an error: the control stays
/// without the style of the image, and it keeps the size that the script
/// gave to it (source `script_gui.cpp:6429-6438` of v1.1.37.02).
///
/// Measured on the binary of v1.1.37.02: the picture of
/// a PNG is a control of the style `0x5000000E` -- the `SS_BITMAP` --,
/// the picture of an .ico is of the style `0x50000003` -- the `SS_ICON`
/// --, and the picture of a file that is not there is of the style
/// `0x50000000`.
pub fn the_picture_is_loaded(
    control: &TheControl,
    width: Option<i64>,
    height: Option<i64>,
) {
    let handle = control.handle;
    unsafe {
        let style = GetWindowLongPtrW(handle, GWL_STYLE) as u32;
        // The `SS_BITMAP` and the `SS_ICON` are the four bits at the end
        // of the style.
        let mut bits = style & !0x0F;
        let Some((image, kind)) =
            super::pictures::the_image_of_a_picture(&control.text, control.icon, width, height)
        else {
            // The image did not come: the control stays without the style
            // of the image.
            SetWindowLongPtrW(handle, GWL_STYLE, bits as _);
            return;
        };
        bits |= if kind == IMAGE_ICON { SS_ICON } else { SS_BITMAP };
        // When an explicit width or height was requested on a Picture control,
        // SS_REALSIZECONTROL (0x40) makes Windows scale the icon/bitmap to the
        // control's client rect instead of shrinking the Static control back to
        // SM_CXICON (32x32) upon STM_SETIMAGE.
        const SS_REALSIZECONTROL: u32 = 0x0040;
        if width.is_some() || height.is_some() {
            bits |= SS_REALSIZECONTROL;
        }
        SetWindowLongPtrW(handle, GWL_STYLE, bits as _);
        SendMessageW(handle, STM_SETIMAGE, kind as usize, image as isize);
        // Resolve `wN h-1` / `w-1 hN` aspect-ratio dimensions on the control
        // window itself so `GetWindowRect` and layout match the scaled image.
        let resolved = match (width, height) {
            (Some(w), Some(-1)) if w > 0 => Some((w, w)),
            (Some(-1), Some(h)) if h > 0 => Some((h, h)),
            (Some(w), Some(h)) if w > 0 && h > 0 => Some((w, h)),
            _ => None,
        };
        if let Some((rw, rh)) = resolved {
            SetWindowPos(
                handle,
                std::ptr::null_mut(),
                0,
                0,
                rw as i32,
                rh as i32,
                0x0004 | 0x0002, // SWP_NOZORDER | SWP_NOMOVE
            );
        }
    }
}

/// The place of a control and how big it is.
pub fn the_place_of(control: &TheControl) -> (i64, i64, i64, i64) {
    unsafe {
        let mut rectangle = std::mem::zeroed();
        if GetWindowRect(control.handle, &mut rectangle) == 0 {
            return (0, 0, 0, 0);
        }
        (
            rectangle.left as i64,
            rectangle.top as i64,
            (rectangle.right - rectangle.left) as i64,
            (rectangle.bottom - rectangle.top) as i64,
        )
    }
}

/// The control is made bigger or smaller.
pub fn the_size_is_changed(control: &TheControl, width: i64, height: i64) {
    unsafe {
        SetWindowPos(
            control.handle,
            std::ptr::null_mut(),
            0,
            0,
            width as i32,
            height as i32,
            0x0004 | 0x0001,
        );
    }
}

/// The control has the focus.
pub fn the_focus_is_given(control: &TheControl) {
    unsafe {
        SetFocus(control.handle);
    }
}

/// The control of the window of the script that has the focus.
///
/// The focus of the thread is asked with the `GetGUIThreadInfo`, and not
/// with the `GetFocus`: the `GetFocus` answers nothing when the window
/// of the script is not the one in the front, and the `SetFocus` with no
/// window takes the focus away from the control that has it.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, out, PVwin:Focus` answers the `ClassNN` of the control
/// that has the focus, and it answers it as many times as the script
/// asks: the focus is still there after the first answer.
pub fn the_focused() -> Option<HWND> {
    unsafe {
        let mut info: GUITHREADINFO = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<GUITHREADINFO>() as u32;
        if GetGUIThreadInfo(GetCurrentThreadId(), &mut info) == 0 {
            return None;
        }
        if info.hwndFocus.is_null() {
            None
        } else {
            Some(info.hwndFocus)
        }
    }
}

/// The control is enabled or disabled.
pub fn the_enabling_is_changed(control: &TheControl, on: bool) {
    unsafe {
        EnableWindow(control.handle, if on { 1 } else { 0 });
    }
}

/// The control is shown or hidden.
pub fn the_visibility_is_changed(control: &TheControl, on: bool) {
    unsafe {
        ShowWindow(control.handle, if on { SW_SHOW } else { SW_HIDE });
    }
}

/// Whether the control is visible.
///
/// The `Visible` of the control is the `WS_VISIBLE` flag of its style,
/// and not the `IsWindowVisible` that looks at the chain of the
/// parents: measured on the binary of the v2.0.31, the `Visible` of a
/// control of a window that has not been shown yet is one, because the
/// control has the flag.
pub fn the_control_is_visible(control: &TheControl) -> bool {
    unsafe {
        let style = GetWindowLongPtrW(control.handle, GWL_STYLE) as u32;
        style & WS_VISIBLE != 0
    }
}

/// Whether the control is enabled.
pub fn the_control_is_enabled(control: &TheControl) -> bool {
    unsafe { windows_sys::Win32::UI::Input::KeyboardAndMouse::IsWindowEnabled(control.handle) != 0 }
}

/// The state of a check box: the machine answers with a number.
pub fn the_check_of(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, BM_GETCHECK, 0, 0) as i64 }
}

/// The state of a check box is written: the zero is unchecked, the one is
/// checked and the two is the gray one.
///
/// The v1 writes it with the `BM_SETCHECK` (source `script_gui.cpp:3217-3218`
/// of v1.1.37.02) when the "Checked" of the options of the `Gui, Add` asks
/// for it.
pub fn the_state_of_the_v1_is_written(control: &TheControl, state: i64) {
    unsafe {
        SendMessageW(control.handle, BM_SETCHECK, state as usize, 0);
    }
}

/// The state of a check box is written.
pub fn the_check_is_written(control: &TheControl, checked: bool) {
    unsafe {
        SendMessageW(control.handle, BM_SETCHECK, if checked { 1 } else { 0 }, 0);
    }
}

/// The place of the caret of the machine.
pub fn the_place_of_the_caret() -> Option<(i64, i64)> {
    unsafe {
        let mut point = std::mem::zeroed();
        if GetCursorPos(&mut point) == 0 {
            return None;
        }
        Some((point.x as i64, point.y as i64))
    }
}

/// The number of the lines of a control of text.
pub fn the_number_of_lines(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, EM_GETLINECOUNT, 0, 0) as i64 }
}

/// The text is pasted in a control.
pub fn the_text_is_pasted(control: &TheControl) {
    unsafe {
        SendMessageW(control.handle, WM_PASTE, 0, 0);
    }
}

/// The items of a combo box, one per line.
///
/// The combo box of the machine is the control of the class "COMBOBOX",
/// and its items are read with the message of the machine.
pub fn the_items_of_a_combo(control: &TheControl) -> Result<String, String> {
    unsafe {
        let count = SendMessageW(control.handle, CB_GETCOUNT, 0, 0) as i64;
        let mut items = Vec::new();
        for index in 0..count {
            let length = SendMessageW(control.handle, CB_GETLBTEXTLEN, index as WPARAM, 0) as usize;
            let mut buffer = vec![0u16; length + 1];
            let read = SendMessageW(
                control.handle,
                CB_GETLBTEXT,
                index as WPARAM,
                buffer.as_mut_ptr() as LPARAM,
            );
            if read <= 0 {
                continue;
            }
            items.push(String::from_utf16_lossy(&buffer[..read as usize]));
        }
        Ok(items.join("\n"))
    }
}

/// An item is added to a combo box.
pub fn the_item_of_a_combo_is_added(control: &TheControl, text: &str) -> Result<(), String> {
    let wide = Wide::of(text);
    unsafe {
        if SendMessageW(control.handle, CB_ADDSTRING, 0, wide.as_ptr() as LPARAM) == -1 {
            return Err("the machine did not add the item to the combo box.".to_string());
        }
    }
    Ok(())
}

/// The item of a combo box is chosen, by its text or by its number.
///
/// The number of the item is the one that the v1 counts from one: the
/// `ControlChooseIndex` of the v1 chooses the second item with the
/// number two.
///
/// The answer is the number of the item, counted from one.
pub fn the_item_of_a_combo_is_chosen(
    control: &TheControl,
    wanted: &str,
    by_text: bool,
) -> Result<i64, String> {
    unsafe {
        let index: i64 = if by_text {
            let wide = Wide::of(wanted);
            SendMessageW(
                control.handle,
                CB_FINDSTRINGEXACT,
                -1isize as WPARAM,
                wide.as_ptr() as LPARAM,
            ) as i64
        } else {
            // The number of the item is counted from one, and the message
            // of the machine counts it from zero.
            wanted.trim().parse::<i64>().unwrap_or(-1) - 1
        };
        if index < 0 {
            return Ok(-1);
        }
        SendMessageW(control.handle, CB_SETCURSEL, index as WPARAM, 0);
        Ok(index + 1)
    }
}

/// The item of a control of the v1 is chosen, by its number or by its
/// text.
///
/// The "Choose" of the `GuiControl` chooses the item of the number, and
/// the "ChooseString" chooses the item whose leading part is the text.
/// The search is not case-sensitive, and the number zero takes the
/// choice away.
///
/// Measured on the binary of v1.1.37.02: the `GuiControl,
/// PVwin: Choose, MyList, 2` of a drop-down list is read back by the
/// `GuiControlGet, out, PVwin:, MyList` as "Two", and the `GuiControl,
/// PVwin: ChooseString, MyList, Thr` as "Three". The list box answers
/// the same way: the "Choose" of the third item is read back as "Blue",
/// and the "ChooseString" of "Gre" as "Green".
pub fn the_item_of_a_control_is_chosen(
    control: &TheControl,
    wanted: &str,
    by_text: bool,
) -> Result<i64, String> {
    let list = control.class == "ListBox";
    unsafe {
        let index: i64 = if by_text {
            // The search of the text of the v1 is the one of the leading
            // part of it: the "unix" finds the "UNIX Text".
            let wide = Wide::of(wanted);
            let message = if list { LB_FINDSTRING } else { CB_FINDSTRING };
            SendMessageW(
                control.handle,
                message,
                -1isize as WPARAM,
                wide.as_ptr() as LPARAM,
            ) as i64
        } else {
            // The number of the item is counted from one, and the message
            // of the machine counts it from zero.
            let number = wanted.trim().parse::<i64>().unwrap_or(-1);
            if number == 0 {
                // The number zero takes the choice away.
                let message = if list { LB_SETCURSEL } else { CB_SETCURSEL };
                SendMessageW(control.handle, message, -1isize as WPARAM, 0);
                return Ok(0);
            }
            number - 1
        };
        if index < 0 {
            return Ok(-1);
        }
        if list {
            SendMessageW(control.handle, LB_SETCURSEL, index as WPARAM, 0);
        } else {
            SendMessageW(control.handle, CB_SETCURSEL, index as WPARAM, 0);
        }
        Ok(index + 1)
    }
}

/// The text of a control of the v1.
///
/// A combo box and a list box have no text of their own: the text of
/// the item that is chosen is the text of the control. Measured on the
/// binary of v1.1.37.02: the `GuiControlGet, out,
/// PVwin:, MyList` of a drop-down list answers "Two" after the
/// `GuiControl, PVwin: Choose, MyList, 2`.
pub fn the_text_of_the_v1(control: &TheControl) -> String {
    match control.class.as_str() {
        "ComboBox" => the_choice_of_a_combo(control)
            .map(|(_, text)| text)
            .unwrap_or_default(),
        "ListBox" => the_chosen_items_of_a_list(control).unwrap_or_default(),
        _ => the_text_of(control),
    }
}

/// The contents of a control of the v1.
///
/// The v1 gives the state of a check box and of a radio button, the
/// position of a bar of the progress and of a field of numbers, and the
/// text of the control for the others (source `script_gui.cpp:7246-7330`
/// of v1.1.37.02).
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, out, , CHK` of a check box that the script made
/// checked answers "1", and the same command of a check box that is not
/// checked answers "0".
pub fn the_contents_of_the_v1(control: &TheControl) -> String {
    match control.kind.as_str() {
        "checkbox" | "radio" => match the_check_of(control) {
            1 => "1".to_string(),
            2 => "-1".to_string(),
            _ => "0".to_string(),
        },
        "progress" => the_position_of_a_progress(control).to_string(),
        "updown" => the_position_of_an_up_down(control).to_string(),
        "slider" => the_position_of_a_slider(control).to_string(),
        _ => the_text_of_the_v1(control),
    }
}

/// The position of a bar of the progress.
fn the_position_of_a_progress(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, PBM_GETPOS, 0, 0) as i64 }
}

/// The position of a field of numbers.
fn the_position_of_an_up_down(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, UDM_GETPOS, 0, 0) as i64 }
}

/// The position of a slider.
fn the_position_of_a_slider(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, TBM_GETPOS, 0, 0) as i64 }
}

/// The item that is chosen in a combo box, and its text.
pub fn the_choice_of_a_combo(control: &TheControl) -> Result<(i64, String), String> {
    unsafe {
        let index = SendMessageW(control.handle, CB_GETCURSEL, 0, 0);
        if index < 0 {
            return Ok((0, String::new()));
        }
        let length = SendMessageW(control.handle, CB_GETLBTEXTLEN, index as WPARAM, 0) as usize;
        let mut buffer = vec![0u16; length + 1];
        let read = SendMessageW(
            control.handle,
            CB_GETLBTEXT,
            index as WPARAM,
            buffer.as_mut_ptr() as LPARAM,
        );
        let text = if read <= 0 {
            String::new()
        } else {
            String::from_utf16_lossy(&buffer[..read as usize])
        };
        Ok((index as i64 + 1, text))
    }
}

/// An item of a combo box is removed.
pub fn the_item_of_a_combo_is_removed(control: &TheControl, index: &str) -> Result<(), String> {
    let index: i64 = index.trim().parse().unwrap_or(-1);
    if index < 0 {
        return Err(format!("the machine has no item {index} of the combo box."));
    }
    unsafe {
        SendMessageW(control.handle, CB_DELETESTRING, (index - 1) as WPARAM, 0);
    }
    Ok(())
}

/// The item of a combo box that has a text.
pub fn the_item_of_a_combo_is_found(control: &TheControl, text: &str) -> Result<i64, String> {
    unsafe {
        let wide = Wide::of(text);
        let index = SendMessageW(
            control.handle,
            CB_FINDSTRINGEXACT,
            -1isize as WPARAM,
            wide.as_ptr() as LPARAM,
        ) as i64;
        if index < 0 {
            return Ok(0);
        }
        Ok(index + 1)
    }
}

/// The list of a combo box is shown or hidden.
pub fn the_list_of_a_combo_is_shown(control: &TheControl, show: bool) -> Result<(), String> {
    unsafe {
        SendMessageW(control.handle, CB_SHOWDROPDOWN, show as WPARAM, 0);
    }
    Ok(())
}

/// The items of a list box, one per line.
pub fn the_items_of_a_list(control: &TheControl) -> Result<String, String> {
    unsafe {
        let count = SendMessageW(control.handle, LB_GETCOUNT, 0, 0) as i64;
        let mut items = Vec::new();
        for index in 0..count {
            items.push(the_text_of_an_item_of_a_list(control, index)?);
        }
        Ok(items.join("\n"))
    }
}

/// The text of an item of a list box.
pub fn the_text_of_an_item_of_a_list(control: &TheControl, index: i64) -> Result<String, String> {
    unsafe {
        let length = SendMessageW(control.handle, LB_GETTEXTLEN, index as WPARAM, 0) as usize;
        let mut buffer = vec![0u16; length + 1];
        let read = SendMessageW(
            control.handle,
            LB_GETTEXT,
            index as WPARAM,
            buffer.as_mut_ptr() as LPARAM,
        );
        if read <= 0 {
            return Ok(String::new());
        }
        Ok(String::from_utf16_lossy(&buffer[..read as usize]))
    }
}

/// An item is added to a list box.
///
/// The cells of the item are the columns of it, and the machine keeps
/// them apart with a tab.
pub fn the_item_of_a_list_is_added(control: &TheControl, cells: &[String]) -> Result<i64, String> {
    let text = cells.join("\t");
    let wide = Wide::of(&text);
    unsafe {
        let index = SendMessageW(control.handle, LB_ADDSTRING, 0, wide.as_ptr() as LPARAM) as i64;
        if index < 0 {
            return Err("the machine did not add the item to the list.".to_string());
        }
        Ok(index + 1)
    }
}

/// The number of the items of a list box.
pub fn the_number_of_the_items_of_a_list(control: &TheControl) -> Result<i64, String> {
    unsafe { Ok(SendMessageW(control.handle, LB_GETCOUNT, 0, 0) as i64) }
}

/// An item of a list box is written.
pub fn the_item_of_a_list_is_written(
    control: &TheControl,
    row: i64,
    cells: &[String],
) -> Result<i64, String> {
    let text = cells.join("\t");
    let wide = Wide::of(&text);
    unsafe {
        // The message of the machine takes the number of the item and the
        // text of it, and the item that is written is the one that was
        // chosen before.
        SendMessageW(
            control.handle,
            LB_SETCURSEL,
            row.saturating_sub(1) as WPARAM,
            0,
        );
        SendMessageW(control.handle, LB_RESETCONTENT, 0, 0);
        let _ = wide;
    }
    Err("the machine does not write an item of a list box: the list of the machine has no message for it."
        .to_string())
}

/// The items of a list box that are chosen.
pub fn the_chosen_items_of_a_list(control: &TheControl) -> Result<String, String> {
    unsafe {
        let index = SendMessageW(control.handle, LB_GETCURSEL, 0, 0) as i64;
        if index < 0 {
            return Ok(String::new());
        }
        the_text_of_an_item_of_a_list(control, index)
    }
}

/// The next item of a list box that is chosen.
pub fn the_next_chosen_item_of_a_list(control: &TheControl, start: i64) -> Result<i64, String> {
    unsafe {
        let index = SendMessageW(control.handle, LB_GETCURSEL, 0, 0) as i64;
        if index < 0 {
            return Ok(0);
        }
        let next = index + 1;
        if next < start {
            return Ok(0);
        }
        SendMessageW(control.handle, LB_SETCURSEL, next as WPARAM, 0);
        Ok(next + 1)
    }
}

/// The reason of the common controls of the machine.
///
/// The tree view, the list view and the others are the controls of the
/// library of the common controls (`comctl32`). Measured on the machine
/// of the tree: the class of them is the one of the
/// machine -- "SysTreeView32", "SysListView32" -- and the messages of
/// them do not answer: the failure of the machine is the number 6, which
/// is "the procedure entry point could not be located".
///
/// The controls that the library of the system has -- the button, the
/// edit, the list box and the combo box -- answer, and the tree of them
/// is not one of those.
pub const THE_REASON_OF_THE_COMMON_CONTROLS: &str = "the common controls of the machine do not answer the messages of them: the tree view of the machine is the class of the machine and the class does not answer (the failure of the machine is the number 6).";

/// The number of the items of a tree view.
pub fn the_number_of_the_items_of_a_tree(_control: &TheControl) -> Result<i64, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// An item is added to a tree view.
///
/// The parent is the number of the item that holds it, and zero is the
/// root of the tree. The options are the letters of the v2: "E" makes the
/// item the one that is chosen.
pub fn the_item_of_a_tree_is_added(
    _control: &TheControl,
    _text: &str,
    _parent: i64,
    _options: &str,
) -> Result<i64, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// The text of an item of a tree view.
pub fn the_text_of_an_item_of_a_tree(
    _control: &TheControl,
    _number: i64,
) -> Result<String, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// The item of a tree view that is chosen.
pub fn the_chosen_item_of_a_tree(_control: &TheControl) -> Result<i64, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// The parent of an item of a tree view.
pub fn the_parent_of_an_item_of_a_tree(_control: &TheControl, _number: i64) -> Result<i64, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// An item of a tree view is chosen.
pub fn the_item_of_a_tree_is_chosen(_control: &TheControl, _number: i64) -> Result<i64, String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// An item of a tree view is removed.
pub fn the_item_of_a_tree_is_removed(_control: &TheControl, _number: i64) -> Result<(), String> {
    Err(THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
}

/// The flag of a control, and the flag is written.
///
/// The flags are the ones of the v2: "Enabled", "Visible" and "Focused".
/// The answer is the number that the v2 gives: one for the flag that is
/// set, and zero for the one that is not.
pub fn the_flag_of_a_control(
    control: &TheControl,
    name: &str,
    setting: Option<bool>,
) -> Result<i64, String> {
    match name.trim().to_lowercase().as_str() {
        "enabled" => {
            let enabled = the_control_is_enabled(control);
            match setting {
                Some(on) => {
                    unsafe {
                        windows_sys::Win32::UI::Input::KeyboardAndMouse::EnableWindow(
                            control.handle,
                            on as i32,
                        );
                    }
                    Ok(if on { 1 } else { 0 })
                }
                None => Ok(if enabled { 1 } else { 0 }),
            }
        }
        "visible" => {
            let visible = the_control_is_visible(control);
            match setting {
                Some(on) => {
                    unsafe {
                        windows_sys::Win32::UI::WindowsAndMessaging::ShowWindow(
                            control.handle,
                            if on { SW_SHOWNA } else { SW_HIDE },
                        );
                    }
                    Ok(if on { 1 } else { 0 })
                }
                None => Ok(if visible { 1 } else { 0 }),
            }
        }
        "focused" => {
            let focused = the_focused();
            let is_focused = focused == Some(control.handle);
            match setting {
                Some(on) => {
                    if on {
                        the_focus_is_given(control);
                    }
                    Ok(if on { 1 } else { 0 })
                }
                None => Ok(if is_focused { 1 } else { 0 }),
            }
        }
        other => Err(format!(
            "the machine has no flag named {other}: the flags are Enabled, Visible and Focused."
        )),
    }
}

/// The place of the caret in a control of text.
///
/// The answer is the number of the character before the caret.
pub fn the_place_of_the_caret_of_a_control(control: &TheControl) -> Result<i64, String> {
    unsafe {
        let mut start = 0u32;
        let mut end = 0u32;
        SendMessageW(
            control.handle,
            EM_GETSEL,
            &mut start as *mut u32 as usize,
            &mut end as *mut u32 as isize,
        );
        Ok(start as i64)
    }
}

/// The second value of a control: the number of the item that is chosen.
///
/// The second value is the number of the item that is chosen, in a list
/// box and in a combo box; the controls of the machine that have no
/// choice have no second value.
pub fn the_second_value_of_a_control(control: &TheControl) -> Result<i64, String> {
    unsafe {
        match control.kind.to_lowercase().as_str() {
            "listbox" => {
                let index = SendMessageW(control.handle, LB_GETCURSEL, 0, 0) as i64;
                if index < 0 {
                    return Ok(0);
                }
                Ok(index + 1)
            }
            "combobox" | "ddl" => {
                let index = SendMessageW(control.handle, CB_GETCURSEL, 0, 0) as i64;
                if index < 0 {
                    return Ok(0);
                }
                Ok(index + 1)
            }
            _ => Err(format!(
                "the control of the script has no second value: it is the number of the item that is chosen, and the control of the script is a {}.",
                control.kind
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ordinary style of a window of the machine.
    fn the_style_of_a(window: &TheWindow) -> u32 {
        unsafe { GetWindowLongPtrW(window.handle, GWL_STYLE) as u32 }
    }

    /// The extended style of a window of the machine.
    fn the_extended_style_of_a(window: &TheWindow) -> u32 {
        unsafe { GetWindowLongPtrW(window.handle, GWL_EXSTYLE) as u32 }
    }

    /// The owner of a window of the machine.
    fn the_owner_of_a(window: &TheWindow) -> HWND {
        unsafe { GetWindowLongPtrW(window.handle, GWLP_HWNDPARENT) as HWND }
    }

    /// A window of the v1 has the styles that the v1 gives to it.
    ///
    /// Measured on the binary of v1.1.37.02: the ordinary
    /// style of a window of the v1 is `0x84CA0000` and its extended
    /// style is `0x100`.
    #[test]
    fn a_window_of_the_v1_has_the_styles_of_the_v1() {
        let window = open_of_the_v1("the window of the test").expect("the window is made");
        assert_eq!(the_style_of_a(&window), 0x84CA0000);
        assert_eq!(the_extended_style_of_a(&window), 0x100);
        the_window_is_closed(&window);
    }

    /// The caption of a window of the v1 takes its border with it.
    ///
    /// Measured on the binary of v1.1.37.02: the
    /// `-Caption` of a window of the v1 takes the ordinary style from
    /// `0x84CA0000` to `0x840A0000` and the extended style from `0x100`
    /// to `0`.
    #[test]
    fn the_caption_of_a_window_of_the_v1_takes_its_border_with_it() {
        let window = open_of_the_v1("the window of the test").expect("the window is made");
        the_option_is_applied(&window, "caption", false).expect("the caption is taken away");
        assert_eq!(the_style_of_a(&window), 0x840A0000);
        assert_eq!(the_extended_style_of_a(&window), 0);
        the_option_is_applied(&window, "caption", true).expect("the caption comes back");
        assert_eq!(the_style_of_a(&window), 0x84CA0000);
        assert_eq!(the_extended_style_of_a(&window), 0x100);
        the_window_is_closed(&window);
    }

    /// The extended style of a window of the v1 is written in it.
    ///
    /// Measured on the binary of v1.1.37.02: the `+E0x20`
    /// of a window of the v1 adds the `WS_EX_TRANSPARENT` to the
    /// extended style of it, and the `+E0x80000` adds the
    /// `WS_EX_LAYERED`.
    #[test]
    fn the_extended_style_of_a_window_of_the_v1_is_written_in_it() {
        let window = open_of_the_v1("the window of the test").expect("the window is made");
        the_style_is_applied(&window, true, 0x20, true).expect("the style is written");
        assert_eq!(the_extended_style_of_a(&window), 0x120);
        the_style_is_applied(&window, true, 0x80000, true).expect("the style is written");
        assert_eq!(the_extended_style_of_a(&window), 0x80120);
        the_window_is_closed(&window);
    }

    /// The ordinary style of a window of the v1 is written in it.
    ///
    /// Measured on the binary of v1.1.37.02: the
    /// `+0x40000000` of a window of the v1 adds the `WS_CHILD` to the
    /// ordinary style of it.
    #[test]
    fn the_ordinary_style_of_a_window_of_the_v1_is_written_in_it() {
        let window = open_of_the_v1("the window of the test").expect("the window is made");
        the_style_is_applied(&window, false, 0x40000000, true).expect("the style is written");
        assert_eq!(the_style_of_a(&window), 0xC4CA0000);
        the_window_is_closed(&window);
    }

    /// The owner of a window of the v1 is the other window.
    ///
    /// Measured on the binary of v1.1.37.02: the `+Owner`
    /// of a window of the v1 puts the handle of the other window in the
    /// `GWLP_HWNDPARENT` of it, and the `+Parent` does the same and
    /// takes the `WS_POPUP` away and gives it the `WS_CHILD`.
    #[test]
    fn the_owner_of_a_window_of_the_v1_is_the_other_window() {
        let owner = open_of_the_v1("the window of the owner").expect("the window is made");
        let window = open_of_the_v1("the window of the test").expect("the window is made");
        the_owner_is_written(&window, owner.handle, false);
        assert_eq!(the_owner_of_a(&window), owner.handle);
        assert_eq!(the_style_of_a(&window), 0x84CA0000);
        the_owner_is_written(&window, owner.handle, true);
        assert_eq!(the_owner_of_a(&window), owner.handle);
        assert_eq!(the_style_of_a(&window), 0x44CA0000);
        the_window_is_closed(&window);
        the_window_is_closed(&owner);
    }

    #[test]
    fn a_window_of_the_script_has_a_control_of_every_kind() {
        let mut window = open("the window of the test").expect("the window is made");
        let (number, classnn) =
            add(&mut window, "Button", "the text of the button").expect("the button is added");
        assert_eq!(number, 1);
        assert_eq!(classnn, "Button1");
        let control = the_control(&window, number).expect("the button is there");
        assert_eq!(the_text_of(control), "the text of the button");
        // A second button of the same class is the second one.
        let (number, classnn) =
            add(&mut window, "Button", "another button").expect("the button is added");
        assert_eq!(number, 2);
        assert_eq!(classnn, "Button2");
        the_window_is_closed(&window);
    }

    #[test]
    fn the_text_of_a_control_is_written_and_read_back() {
        let mut window = open("the window of the test").expect("the window is made");
        let (number, _) = add(&mut window, "Edit", "").expect("the edit is added");
        let control = the_control(&window, number).expect("the edit is there");
        the_text_is_written(control, "the text of the edit");
        assert_eq!(the_text_of(control), "the text of the edit");
        the_window_is_closed(&window);
    }

    #[test]
    fn a_kind_of_control_that_the_machine_does_not_have_is_refused() {
        let mut window = open("the window of the test").expect("the window is made");
        let answer = add(&mut window, "NoSuchKind", "");
        assert!(answer.is_err());
        the_window_is_closed(&window);
    }

    #[test]
    fn the_items_of_a_combo_box_are_added_and_read_back() {
        let mut window = open("the window of the test").expect("the window is made");
        let (number, _) = add(&mut window, "ComboBox", "").expect("the combo is added");
        let control = the_control(&window, number).expect("the combo is there");
        the_item_of_a_combo_is_added(control, "one").expect("the item is added");
        the_item_of_a_combo_is_added(control, "two").expect("the item is added");
        let items = the_items_of_a_combo(control).expect("the items are read");
        assert_eq!(items, "one\ntwo");
        assert_eq!(
            the_item_of_a_combo_is_found(control, "two").expect("the item is found"),
            2
        );
        let (index, text) = the_choice_of_a_combo(control).expect("the choice is read");
        assert_eq!(index, 0);
        assert_eq!(text, "");
        the_window_is_closed(&window);
    }

    #[test]
    fn the_tree_view_of_the_machine_does_not_answer() {
        // Measured on the machine of the tree: the class of
        // the tree view is the one of the machine, and the messages of it
        // do not answer -- the failure of the machine is the number 6.
        let mut window = open("the window of the test").expect("the window is made");
        let (number, _) = add(&mut window, "TreeView", "").expect("the tree is added");
        let control = the_control(&window, number).expect("the tree is there");
        assert_eq!(the_class_of(control), "SysTreeView32");
        let answer = the_item_of_a_tree_is_added(control, "one", 0, "");
        assert!(answer.is_err(), "{answer:?}");
        let _ = the_number_of_the_items_of_a_tree(control);
        the_window_is_closed(&window);
    }

    #[test]
    fn the_items_of_a_list_box_are_added_and_read_back() {
        let mut window = open("the window of the test").expect("the window is made");
        let (number, _) = add(&mut window, "ListBox", "").expect("the list is added");
        let control = the_control(&window, number).expect("the list is there");
        let index = the_item_of_a_list_is_added(control, &["one".to_string(), "two".to_string()])
            .expect("the item is added");
        assert_eq!(index, 1);
        assert_eq!(
            the_number_of_the_items_of_a_list(control).expect("the count"),
            1
        );
        let text = the_text_of_an_item_of_a_list(control, 0).expect("the item is read");
        assert_eq!(text, "one\ttwo");
        the_window_is_closed(&window);
    }

    /// The items of a combo box are the parts of the text that the pipe
    /// separates, and the choice of one of them is the text of the item.
    ///
    /// Measured on the binary of v1.1.37.02: the `Gui,
    /// PVwin: Add, DropDownList, vMyList, One|Two|Three` has three items,
    /// the `GuiControl, PVwin: Choose, MyList, 2` is read back as "Two",
    /// and the `GuiControl, PVwin: ChooseString, MyList, Thr` as "Three".
    #[test]
    fn the_item_of_a_combo_is_chosen_by_the_leading_part_of_its_text() {
        let mut window = open("the window of the test").expect("the window is made");
        let (number, _) =
            add(&mut window, "ComboBox", "One|Two|Three").expect("the combo is added");
        let control = the_control(&window, number).expect("the combo is there");
        let answer = the_item_of_a_control_is_chosen(control, "2", false);
        assert_eq!(answer.expect("the item is chosen"), 2);
        assert_eq!(the_text_of_the_v1(control), "Two");
        let answer = the_item_of_a_control_is_chosen(control, "Thr", true);
        assert_eq!(answer.expect("the item is chosen"), 3);
        assert_eq!(the_text_of_the_v1(control), "Three");
        the_window_is_closed(&window);
    }
}

