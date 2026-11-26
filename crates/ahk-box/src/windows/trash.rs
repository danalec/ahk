//! The tray of the machine and the trash of it.
//!
//! The tray is the icon that the machine shows in the corner of the
//! screen: the shell of it draws it (`Shell_NotifyIconW`), and the
//! window of the script is the one that owns it.
//!
//! The trash of a Windows machine is the place where the files that are
//! removed go: the shell moves them there (`SHFileOperationW`), and the
//! empty of it is the shell too.

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::Shell::SHFileOperationW as the_operation_of_the_shell;
use windows_sys::Win32::UI::Shell::{
    ExtractIconExW, FO_DELETE, FOF_ALLOWUNDO, FOF_NOCONFIRMATION, FOF_NOERRORUI, FOF_SILENT,
    NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_MODIFY, NOTIFYICONDATAW,
    SHFILEOPSTRUCTW, Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::HICON;

use super::sys::Wide;

/// The icon of the tray of the machine.
///
/// The icon is the one of the script: the machine has no icon of its
/// own for the tray of a script, and the one of the program of it is
/// the one that the tray shows.
///
/// The window is the window of the script: the shell keeps the icon
/// while the window that owns it lives, and the `NIM_DELETE` of the end
/// of the run points at the same window and at the same number. The
/// message of the mouse is the one that the procedure of the window of
/// the script knows, so that the right button opens the menu of it.
pub fn set_the_icon(window: HWND, name: &str, tip: &str) -> Result<(), String> {
    let mut data: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    data.hWnd = window;
    data.uID = 1;
    data.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    data.uCallbackMessage = THE_TRAY_MESSAGE;
    data.hIcon = the_icon_of_the_program(name);
    the_tip_goes_into(&mut data, tip);
    if unsafe { Shell_NotifyIconW(NIM_ADD, &data) } == 0 {
        return Err("the machine did not take the icon of the tray.".to_string());
    }
    Ok(())
}

/// The tip of the icon goes into the structure of the shell.
///
/// The field of the shell that carries the tip has room for a hundred
/// and twenty letters, and the text that is longer is cut.
fn the_tip_goes_into(data: &mut NOTIFYICONDATAW, tip: &str) {
    data.szTip = [0u16; 128];
    let tip = Wide::of(tip);
    for (index, letter) in tip.letters().iter().enumerate().take(127) {
        data.szTip[index] = *letter;
    }
}

/// The notification of the machine, in the balloon of the tray.
pub fn notify(window: HWND, title: &str, text: &str) -> Result<(), String> {
    let mut data: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    data.hWnd = window;
    data.uID = 1;
    data.uFlags = NIF_INFO;
    data.dwInfoFlags = NIF_INFO;
    let title = Wide::of(title);
    let text = Wide::of(text);
    for (index, letter) in title.letters().iter().enumerate().take(127) {
        data.szInfoTitle[index] = *letter;
    }
    for (index, letter) in text.letters().iter().enumerate().take(255) {
        data.szInfo[index] = *letter;
    }
    if unsafe { Shell_NotifyIconW(NIM_MODIFY, &data) } == 0 {
        return Err("the machine did not show the notification.".to_string());
    }
    Ok(())
}

/// The icon of the program of the script, and nothing when the program
/// has none.
///
/// The icon of a program is taken out of it by the shell:
/// `ExtractIconExW` is the reader of the icons of a file.
fn the_icon_of_the_program(name: &str) -> HICON {
    let path = Wide::of(name);
    let mut large: HICON = std::ptr::null_mut();
    let mut small: HICON = std::ptr::null_mut();
    unsafe {
        ExtractIconExW(path.as_ptr(), 0, &mut large, &mut small, 1);
    }
    if large.is_null() { small } else { large }
}

/// The icon of a file of the machine is read, and the answer is whether
/// the file has one.
///
/// The reader is `ExtractIconExW`, which reads the icons of a program and
/// the ones of a file of icons. The `number` is the place of the icon in
/// the file: the first one is the number one.
pub fn the_icon_of_the_file_is_read(path: &str, number: i64) -> Result<(), String> {
    let wide = Wide::of(path);
    let mut large: HICON = std::ptr::null_mut();
    let mut small: HICON = std::ptr::null_mut();
    let index = (number - 1).max(0) as i32;
    unsafe {
        ExtractIconExW(wide.as_ptr(), index, &mut large, &mut small, 1);
    }
    if large.is_null() && small.is_null() {
        return Err(format!("the file has no icon: {path}"));
    }
    Ok(())
}

/// A file of the machine goes to the trash of it.
///
/// The shell of the machine moves the file: the flags ask it to let the
/// file be undone, and to not ask anything to the user.
pub fn recycle(path: &str) -> Result<(), String> {
    let mut operation: SHFILEOPSTRUCTW = unsafe { std::mem::zeroed() };
    operation.hwnd = std::ptr::null_mut();
    operation.wFunc = FO_DELETE;
    operation.fFlags = (FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_SILENT) as u16;
    // The name of the file lives as long as the operation: the shell
    // reads it while it works.
    let name = Wide::of(path);
    operation.pFrom = name.as_ptr();
    let answer = unsafe { the_operation_of_the_shell(&mut operation) };
    if answer != 0 {
        return Err(format!(
            "the machine did not move the file to the trash: {answer}."
        ));
    }
    Ok(())
}

/// The trash of the machine is emptied.
///
/// The shell of the machine empties it: `SHEmptyRecycleBinW` is the
/// reader of it, and it is of Windows 2000.
pub fn empty_the_trash() -> Result<(), String> {
    use windows_sys::Win32::UI::Shell::SHEmptyRecycleBinW;
    let answer = unsafe { SHEmptyRecycleBinW(std::ptr::null_mut(), std::ptr::null(), 0) };
    if answer != 0 {
        return Err(format!(
            "the machine did not empty the trash: the code is {answer}."
        ));
    }
    Ok(())
}

/// The message the tray sends to the window of the script when the mouse
/// touches the icon of it. It is apart from the messages of the hotkeys.
pub const THE_TRAY_MESSAGE: u32 = 0x0400 + 3;

/// The persistent icon of the script in the tray of the session.
///
/// The icon is the one of the program of the script, and the window of
/// the script is the one that receives the mouse of it. The `icon` is the
/// file that the script gave, and the icon of the program of the script
/// is the one of the tray when the script gave none.
pub(crate) fn show_the_tray(
    window: HWND,
    tip: &str,
    icon: Option<&str>,
) -> Result<(), String> {
    let mut data: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    data.hWnd = window;
    data.uID = 1;
    data.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    data.uCallbackMessage = THE_TRAY_MESSAGE;
    data.hIcon = match icon {
        Some(icon) => the_icon_of_the_program(icon),
        None => the_icon_of_the_script(),
    };
    the_tip_goes_into(&mut data, tip);
    if unsafe { Shell_NotifyIconW(NIM_ADD, &data) } == 0 {
        return Err("the machine did not take the icon of the tray.".to_string());
    }
    Ok(())
}

/// The menu of the tray of the script, and whether Exit was chosen.
///
/// The right button of the mouse opens the menu, and Exit is the item
/// that stops the script.
pub(crate) fn the_tray_menu(window: HWND) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, PostMessageW, SetForegroundWindow,
        TrackPopupMenu, MF_STRING, TPM_RETURNCMD, TPM_RIGHTBUTTON, WM_NULL,
    };
    unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() {
            return false;
        }
        let exit = Wide::of("Exit");
        AppendMenuW(menu, MF_STRING, 1, exit.as_ptr());
        let mut point = std::mem::zeroed();
        GetCursorPos(&mut point);
        // The menu of the tray needs the window in front, the way the
        // shell asks for it.
        SetForegroundWindow(window);
        let chosen = TrackPopupMenu(
            menu,
            TPM_RETURNCMD | TPM_RIGHTBUTTON,
            point.x,
            point.y,
            0,
            window,
            std::ptr::null_mut(),
        );
        // The shell asks for a message after the menu: without it the
        // thread keeps the menu half-open and the click on Exit is slow
        // to arrive. A null message drains the queue and lets the menu
        // go, the way the tray of the original AutoHotkey does.
        PostMessageW(window, WM_NULL, 0, 0);
        DestroyMenu(menu);
        chosen == 1
    }
}

/// The icon of the tray of the script is taken away.
pub(crate) fn remove_the_tray(window: HWND) {
    use windows_sys::Win32::UI::Shell::NIM_DELETE;
    let mut data: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    data.hWnd = window;
    data.uID = 1;
    unsafe {
        Shell_NotifyIconW(NIM_DELETE, &data);
    }
}

/// The icon of the program of the script. The reader of the icons of a
/// file is `ExtractIconExW`, already asked of the floor of the XP; the
/// program without an icon carries none, and the item of the tray still
/// answers.
fn the_icon_of_the_script() -> HICON {
    let (_, large) = the_icons_of_the_program();
    large
}

/// The small icon and the big icon of the program of the script.
///
/// The v1: the window of the script carries the icon of the program that
/// runs it -- the small one in the title bar and the big one in the
/// list of the windows. The icon of the tray is another thing: it does
/// not change the icon of the window.
///
/// Measured on the binary of the v1.1.37.02: the window of
/// the script of the v1 has an icon, and the `Menu, Tray, Icon` does not
/// change it.
///
/// The icons are read once and kept: the windows of the script share
/// them, and the machine does not copy them.
pub fn the_icons_of_the_program() -> (HICON, HICON) {
    use std::sync::OnceLock;
    use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
    // The handles are kept as numbers: a raw pointer is not shared
    // between the threads of the machine without a lock, and the icons
    // are the same for the whole of the program.
    static THE_ICONS: OnceLock<(usize, usize)> = OnceLock::new();
    let (small, large) = *THE_ICONS.get_or_init(|| {
        let mut path = [0u16; 512];
        let len = unsafe { GetModuleFileNameW(std::ptr::null_mut(), path.as_mut_ptr(), 512) };
        if len == 0 {
            return (0usize, 0usize);
        }
        let mut large: HICON = std::ptr::null_mut();
        let mut small: HICON = std::ptr::null_mut();
        unsafe {
            ExtractIconExW(path.as_ptr(), 0, &mut large, &mut small, 1);
        }
        (small as usize, large as usize)
    });
    (
        small as HICON,
        large as HICON,
    )
}
