// SPDX-License-Identifier: Apache-2.0
//! The window of the messages of the thread of the script.
//!
//! The reference keeps a main window on the thread of the script, and a
//! modal dialog announces itself to it before it pumps: `WM_COMMNOTIFY`
//! with `AHK_DIALOG` (1027) in wParam (`POST_AHK_DIALOG` of
//! `source/window.h`, `source/window.cpp:1012`). The modal loop of the
//! machine pumps the messages of every window of the thread, so the
//! announcement arrives while the box waits, and the handler `OnMessage`
//! registered for 0x44 runs inside the wait -- the way a script themes
//! the box dark: it enumerates the windows of the thread and subclasses
//! the dialog while the box pumps.
//!
//! This window is that window: hidden, on the thread that asks for it,
//! and its procedure runs the handler of the message synchronously, on
//! the runtime the thread keeps while the modal call goes on. A message
//! with no handler goes to the procedure of the machine.

use std::cell::Cell;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{DefWindowProcW, PostMessageW, WM_COMMNOTIFY};

thread_local! {
    /// The window of this thread: each thread that shows a modal dialog
    /// has its own, because the modal loop only pumps the windows of its
    /// own thread.
    static THE_WINDOW: Cell<HWND> = const { Cell::new(std::ptr::null_mut()) };
}

/// `AHK_DIALOG` of the reference: `WM_USER + 3` (`source/hook.h`).
const AHK_DIALOG: WPARAM = 0x0400 + 3;

/// The procedure of the window: the handler of the message runs now,
/// while the modal dialog pumps, and its number is the answer of the
/// procedure. The runtime is the one the thread keeps for the modal call.
unsafe extern "system" fn the_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Some(answer) = ahk::callback::the_message_runs(
        message as i64,
        wparam as i64,
        lparam as i64,
        window as isize as i64,
    ) {
        return answer as LRESULT;
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// Announce a modal dialog to the thread, the way `POST_AHK_DIALOG` does.
///
/// The window is made once per thread and stays hidden; the message is
/// posted, and the modal loop of the machine dispatches it while the
/// dialog waits. A thread that cannot make the window still shows the
/// dialog: the announcement is the way of the handlers, not of the box.
pub fn the_dialog_is_announced() {
    let window = THE_WINDOW.with(|cell| {
        let held = cell.get();
        if !held.is_null() {
            return Ok(held);
        }
        let window =
            crate::windows::toplevel::a_window_of_the_script("ahk-rs-messages", the_procedure)?;
        cell.set(window);
        Ok::<HWND, String>(window)
    });
    let Ok(window) = window else { return };
    let _ = unsafe { PostMessageW(window, WM_COMMNOTIFY, AHK_DIALOG, 0) };
}
