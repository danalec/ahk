//! The hotkeys of the machine.
//!
//! A hotkey of Windows is a combination of keys that the session sends
//! to the window that owns it: `RegisterHotKey` is the way of it, and
//! the message of it arrives in the window of the owner.
//!
//! The session has two windows of its own, and they live in two
//! threads:
//!
//! - the **window of the hotkeys** lives in the thread of the machine,
//!   and it is the owner of the hotkeys that the script registers: the
//!   thread is always in `GetMessageW`, so the message of a hotkey that
//!   fires arrives at once;
//! - the **window of the script** lives in the thread of the script, and
//!   its number is the `A_ScriptHwnd` of the v1: the messages that the
//!   script watches arrive in it, the menu of the tray is tracked in it,
//!   and the tracking of a menu of the script happens in it, so that a
//!   hook of the messages of the thread of the script sees the menu that
//!   opens.
//!
//! The tree of the hotkeys is shared between the thread of the machine
//! and the script, and the script asks it for the hotkey that fired.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ahk::hotkeys::{hotstring_text, parse};

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, MOD_WIN, RegisterHotKey, UnregisterHotKey,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, DispatchMessageW, GetMessageW, SendMessageW, TranslateMessage, WM_APP,
    WM_HOTKEY,
};

/// The class of the window of the hotkeys.
///
/// The window of the hotkeys is a window of its own: the window of the
/// script belongs to the thread of the script, and the hotkeys of the
/// machine arrive in the window of the thread that pumps them.
const THE_CLASS_OF_THE_HOTKEYS: &str = "ahk-rs-hotkeys";

/// The class of the window of the script.
///
/// The window of the script is the hidden window that carries the
/// messages of the session, and its number is the `A_ScriptHwnd` of the
/// v1. It lives in the thread of the script, the way the window of the
/// v1 lives in the thread of the script of the v1.
const THE_CLASS_OF_THE_SCRIPT: &str = "ahk-rs-script";

/// The message that asks the window of the machine to register a hotkey.
///
/// `RegisterHotKey` belongs to the thread that owns the window: a call
/// from another thread is refused with the code 1408. The window of the
/// hotkeys is owned by the thread of the machine, so the script sends the
/// question to the window, and the procedure of it registers the hotkey.
/// The number of the hotkey is the `wparam`, and the `lparam` carries the
/// modifiers in the low half and the key in the high half.
const THE_REGISTER_OF_A_HOTKEY: u32 = WM_APP + 1;

/// The message that asks the window of the machine to give a hotkey back.
///
/// `UnregisterHotKey` belongs to the thread that owns the window as well,
/// and the number of the hotkey is the `wparam`.
const THE_GIVE_BACK_OF_A_HOTKEY: u32 = WM_APP + 2;

/// The message that asks the window of the script to show a menu of the
/// script.
///
/// The `TrackPopupMenu` of the machine belongs to the thread that owns
/// the window of the owner: the window of the script belongs to the
/// thread of the script, and the tracking of the menu happens there.
/// The script asks for it with a `SendMessage`, and the answer of the
/// item that was chosen is in the state that the two threads share.
const THE_SHOW_OF_A_MENU: u32 = WM_APP + 3;

/// The key and the modifiers of a hotkey, in the `lparam` of the
/// message of the box.
///
/// The `lparam` is a pointer, and its width is the width of the
/// machine. On sixty-four bits there is room for the key in the high
/// half and the modifiers in the low half, and on thirty-two bits
/// there is room for the key in the high *sixteen* bits and the
/// modifiers in the low sixteen: the modifiers are five flags
/// (`MOD_ALT`, `MOD_CONTROL`, `MOD_SHIFT`, `MOD_WIN`, `MOD_NOREPEAT`)
/// and they fit, and the key is a virtual-key code, and it fits as
/// well.
///
/// The packing is the tree's own: the message is the message of the
/// box, and Windows never sees it.
#[cfg(target_pointer_width = "64")]
fn packed(key: u16, modifiers: u32) -> LPARAM {
    (((key as u64) << 32) | u64::from(modifiers)) as LPARAM
}

/// The key and the modifiers of a hotkey, in the `lparam` of the
/// message of the box, on a machine of thirty-two bits.
#[cfg(not(target_pointer_width = "64"))]
fn packed(key: u16, modifiers: u32) -> LPARAM {
    (((key as u32) << 16) | (modifiers & 0xFFFF)) as LPARAM
}

/// The key and the modifiers of a hotkey, out of the `lparam` of the
/// message of the box.
#[cfg(target_pointer_width = "64")]
fn unpacked(lparam: LPARAM) -> (u16, u32) {
    let packed = lparam as u64;
    ((packed >> 32) as u16, packed as u32)
}

/// The key and the modifiers of a hotkey, out of the `lparam` of the
/// message of the box, on a machine of thirty-two bits.
#[cfg(not(target_pointer_width = "64"))]
fn unpacked(lparam: LPARAM) -> (u16, u32) {
    let packed = lparam as u32;
    ((packed >> 16) as u16, packed & 0xFFFF)
}

/// The menu that the script wants shown, and the item that was chosen.
///
/// The request is made by the thread of the script, and the tracking of
/// the menu happens in the thread of the session: the `TrackPopupMenu`
/// of the machine belongs to the thread that owns the window of the
/// owner, and it fails at once when the owner belongs to another thread.
#[derive(Debug, Default)]
pub struct TheMenuRequest {
    /// The handle of the menu of the machine.
    pub menu: usize,
    /// The place of the menu on the screen.
    pub x: i32,
    /// The place of the menu on the screen.
    pub y: i32,
    /// The item that was chosen, and the number of it.
    pub chosen: Option<usize>,
}

/// The hotkeys of the machine, and the ones that fired.
#[derive(Debug, Default)]
pub struct TheHotkeys {
    /// The hotkeys that are registered, and the number of each of them.
    registered: Vec<(i32, String)>,
    /// The hotkeys that fired, and that the script has not read yet.
    fired: Vec<String>,
    /// The event that says that a hotkey fired.
    ///
    /// The `Condvar` of the `std` is of Windows Vista on the machine:
    /// the `SleepConditionVariableSRW` is not in XP. The event of the
    /// kernel32 is of Windows 2000, and it is the one that the wait
    /// uses.
    fired_now: TheEvent,
    /// The window of the script, and the one that owns the hotkeys.
    ///
    /// The window is a number and not a pointer: the pointer is not
    /// `Send`, and the state travels to the thread of the machine.
    ///
    /// The window of the script is the hidden window that carries the
    /// messages of the session -- the one of the v2 is as well --, and
    /// its number is the `A_ScriptHwnd` of the v2. It lives in the
    /// thread of the script, the way the window of the v1 lives in the
    /// thread of the script of the v1.
    script_window: Option<usize>,
    /// The window of the hotkeys.
    ///
    /// The window of the hotkeys is the owner of the hotkeys that the
    /// script registers: it lives in the thread of the machine, and that
    /// thread is always in `GetMessageW`, so the message of a hotkey
    /// that fires arrives at once.
    hotkeys_window: Option<usize>,
    /// The next number of a hotkey of the machine.
    next: i32,
    /// The messages of the window that the script watches.
    ///
    /// The `OnMessage` registers a handler for a number, and the window
    /// of the script queues the message when it arrives.
    watched_messages: Vec<i64>,
    /// The events of the window of the script that happened.
    ///
    /// The script reads them between its statements: a message that
    /// arrived is the `message:<number>` the handler of it answers.
    events: Vec<String>,
    /// The menu that the script wants shown, and the item that was
    /// chosen.
    ///
    /// The request travels to the thread of the session, and the answer
    /// comes back with it.
    menu_request: Option<TheMenuRequest>,
    /// The Exit of the tray of the script was chosen.
    ///
    /// The run that stays alive reads it and ends, the way `ExitApp`
    /// ends it.
    pub exit_asked: bool,
}

/// The state that the thread of the machine and the script share.
pub type TheShared = Arc<Mutex<TheHotkeys>>;

/// The state of the machine, for the thread that pumps the messages.
///
/// The window of the machine is made by the thread of the hotkeys, and
/// the procedure of it is the one that puts the hotkeys that fired in
/// the list of them. The state is here because the procedure of a window
/// has no other way to reach it.
static THE_MACHINE: Mutex<Option<TheShared>> = Mutex::new(None);

/// The state that the procedures of the windows of the session share.
///
/// The state of the machine is the state of the desktop that made the
/// windows, and the procedures of the windows reach it through here.
fn the_state() -> Option<TheShared> {
    let machine = THE_MACHINE.lock().ok()?;
    machine.clone()
}

/// The procedure of the window of the hotkeys.
///
/// The window of the hotkeys is the window of the thread of the machine:
/// the hotkeys of the script arrive in it, and the thread is always in
/// `GetMessageW`, so the message of a hotkey that fires is there at once.
///
/// The message of a hotkey carries the number of it, and the name of it
/// is the one that the script registered with that number.
unsafe extern "system" fn the_procedure_of_the_hotkeys(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_HOTKEY {
        let number = wparam as i32;
        if let Some(shared) = the_state() {
            if let Ok(mut hotkeys) = shared.lock() {
                let name = hotkeys
                    .registered
                    .iter()
                    .find(|(other, _)| *other == number)
                    .map(|(_, name)| name.clone());
                if let Some(name) = name {
                    hotkeys.fired.push(name);
                    // The wait of the script ends here: the
                    // keyboard was pressed, and the run does not
                    // need to ask.
                    hotkeys.fired_now.signal();
                }
            }
        }
        return 0;
    }
    // The questions of the script are messages of the window: the
    // procedure runs in the thread of the machine, and that thread is the
    // one that owns the window, so the hotkey is registered here. The
    // answer of the machine is the number of the failure, and zero when
    // the machine took the hotkey.
    if message == THE_REGISTER_OF_A_HOTKEY {
        let number = wparam as i32;
        let (key, modifiers) = unpacked(lparam);
        let taken = unsafe { RegisterHotKey(window, number, modifiers, key as u32) };
        if taken == 0 {
            return unsafe { windows_sys::Win32::Foundation::GetLastError() } as LRESULT;
        }
        return 0;
    }
    if message == THE_GIVE_BACK_OF_A_HOTKEY {
        let number = wparam as i32;
        unsafe {
            UnregisterHotKey(window, number);
        }
        return 0;
    }
    // The messages that the procedure does not know are the ones of the
    // machine: the window asks its procedure about its own creation, and
    // the answer of the procedure has to be the one of the machine.
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// The procedure of the window of the script.
///
/// The window of the script is the window of the thread of the script:
/// the messages that the script watches arrive in it, the menu of the
/// tray is tracked in it, and the tracking of a menu of the script
/// happens in it, so that a hook of the messages of the thread of the
/// script sees the menu that opens.
unsafe extern "system" fn the_procedure_of_the_script(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // The message of the tray of the script: the mouse of the icon
    // arrives in the window of the script, and the menu of the tray is
    // tracked in the thread of the script -- the window of the owner
    // belongs to this thread, and the `TrackPopupMenu` of the machine
    // belongs to the thread that owns it.
    if message == super::trash::THE_TRAY_MESSAGE {
        use windows_sys::Win32::UI::WindowsAndMessaging::{WM_CONTEXTMENU, WM_RBUTTONUP};
        let mouse = lparam as u32;
        if mouse == WM_RBUTTONUP || mouse == WM_CONTEXTMENU {
            // The menu of the tray: Exit stops the script.
            if super::trash::the_tray_menu(window) {
                if let Some(shared) = the_state() {
                    if let Ok(mut hotkeys) = shared.lock() {
                        hotkeys.exit_asked = true;
                        hotkeys.fired_now.signal();
                    }
                }
            }
        }
        return 0;
    }
    if message == THE_SHOW_OF_A_MENU {
        // The tracking of the menu happens here, in the thread of the
        // script: the `TrackPopupMenu` of the machine belongs to the
        // thread that owns the window of the owner, and it fails at once
        // when the owner is of another thread. The window of the script
        // is in the thread of the script, so the menu is tracked in the
        // thread of the script, and the hook of the messages of that
        // thread sees the menu that opens.
        let request = {
            let mut request = None;
            if let Some(shared) = the_state() {
                if let Ok(mut hotkeys) = shared.lock() {
                    request = hotkeys.menu_request.take();
                }
            }
            request
        };
        let Some(request) = request else {
            return 0;
        };
        let chosen = unsafe {
            super::menu::the_menu_is_tracked(
                request.menu as windows_sys::Win32::UI::WindowsAndMessaging::HMENU,
                request.x,
                request.y,
                window,
            )
        };
        // The answer goes back to the thread of the script: it waits for
        // the message to end, and it reads the answer here.
        if let Some(shared) = the_state() {
            if let Ok(mut hotkeys) = shared.lock() {
                hotkeys.menu_request = Some(TheMenuRequest { chosen, ..request });
            }
        }
        return 0;
    }
    // A message the script watches with `OnMessage`: it is queued for
    // the script to answer between its statements. The queue is in the
    // state that the two threads share, and the script reads it when it
    // pumps the messages of the thread.
    if let Some(shared) = the_state() {
        if let Ok(mut hotkeys) = shared.lock() {
            if hotkeys.watched_messages.contains(&(message as i64)) {
                // The handler of the message is called with (wParam,
                // lParam, msg, hwnd): the parameters travel with the
                // event, the way the reference hands them over.
                hotkeys.events.push(format!(
                    "message:{message}:{wparam}:{lparam}:{}",
                    window as isize
                ));
                hotkeys.fired_now.signal();
            }
        }
    }
    // The messages that the procedure does not know are the ones of the
    // machine: the window asks its procedure about its own creation, and
    // the answer of the procedure has to be the one of the machine.
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// The thread of the machine starts, and it owns the window of the
/// hotkeys.
///
/// The thread pumps the messages of the hotkeys, and the hotkeys that
/// fire arrive in the window of it. The window of the script is **not**
/// made here: it belongs to the thread of the script, and it is made by
/// the thread that runs the script (`the_script_window_is_made`).
pub fn the_thread_starts(shared: &TheShared) -> Result<(), String> {
    if let Ok(mut machine) = THE_MACHINE.lock() {
        *machine = Some(shared.clone());
    }
    let shared = shared.clone();
    std::thread::Builder::new()
        .name("ahk-rs-hotkeys".to_string())
        .spawn(move || {
            // The window of the hotkeys is the window of the thread of
            // the machine: the hotkeys that the script registers arrive
            // in it, and the thread is always in `GetMessageW`, so the
            // message of a hotkey that fires is there at once.
            //
            // The class of the window is a class of its own: the class of
            // the script belongs to the window of the script, and
            // `RegisterClassEx` refuses the second registration of the
            // same class with the code 1410 -- the window would be made
            // with the procedure of the other one, and the hotkeys would
            // be handled by the procedure of the script.
            let window = match crate::windows::toplevel::a_window_of_the_machine(
                THE_CLASS_OF_THE_HOTKEYS,
                the_procedure_of_the_hotkeys,
            ) {
                Ok(window) => window,
                Err(reason) => {
                    eprintln!("the window of the hotkeys: {reason}");
                    return;
                }
            };
            if let Ok(mut hotkeys) = shared.lock() {
                hotkeys.hotkeys_window = Some(window as usize);
            }
            // The loop of the messages of the session: the hotkeys of
            // the script arrive in the window of it.
            let mut message = unsafe { std::mem::zeroed() };
            while unsafe { GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) } > 0 {
                unsafe {
                    let _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        })
        .map_err(|error| format!("the machine did not start the thread of the hotkeys: {error}"))?;
    Ok(())
}

/// The window of the script is made, in the thread of the script.
///
/// The window belongs to the thread that makes it, and the thread of the
/// script is the thread that runs the script: the messages that arrive in
/// the window are dispatched by the pump of the script, the menu of the
/// tray is tracked in it, and the tracking of a menu of the script
/// happens in it. A hook of the messages of the thread of the script
/// sees them all.
///
/// This is the difference between the tree and the v1: the v1 makes the
/// window of the script in the thread of the script too, and the
/// `A_ScriptHwnd` of the v1 is the handle of a window of the thread that
/// runs the script. Measured on the binary of v1.1.37.02: the
/// `GetCurrentThreadId` of the script and the
/// `GetWindowThreadProcessId` of the `A_ScriptHwnd` are the same number.
pub fn the_script_window_is_made(shared: &TheShared) -> Result<HWND, String> {
    {
        let hotkeys = shared
            .lock()
            .map_err(|_| "the machine is busy.".to_string())?;
        if let Some(window) = hotkeys.script_window {
            return Ok(window as HWND);
        }
    }
    let window = crate::windows::toplevel::a_window_of_the_script(
        THE_CLASS_OF_THE_SCRIPT,
        the_procedure_of_the_script,
    )?;
    let mut hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    hotkeys.script_window = Some(window as usize);
    Ok(window)
}

/// The number of the window of the script.
///
/// The window is made by the thread of the script -- the thread that runs
/// the script --, and the wait here is for the case that the window is
/// being made by another thread: the wait is short, and the thread makes
/// the window right after it begins.
pub fn the_window_of_the_script(shared: &TheShared) -> Result<HWND, String> {
    let mut waited = 0u32;
    loop {
        {
            let hotkeys = shared
                .lock()
                .map_err(|_| "the machine is busy.".to_string())?;
            if let Some(window) = hotkeys.script_window {
                return Ok(window as HWND);
            }
        }
        waited += 10;
        if waited >= 5000 {
            return Err("the machine did not make the window of the script.".to_string());
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// The number of the window of the hotkeys.
///
/// The window of the hotkeys is made by the thread of the machine, and
/// the thread makes it right after it begins: the wait here is for the
/// moment between the beginning of the thread and the window.
pub fn the_window_of_the_hotkeys(shared: &TheShared) -> Result<HWND, String> {
    let mut waited = 0u32;
    loop {
        {
            let hotkeys = shared
                .lock()
                .map_err(|_| "the machine is busy.".to_string())?;
            if let Some(window) = hotkeys.hotkeys_window {
                return Ok(window as HWND);
            }
        }
        waited += 10;
        if waited >= 5000 {
            return Err("the machine did not make the window of the hotkeys.".to_string());
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// The menu of the script is shown, and the answer is the item that was
/// chosen.
///
/// The tracking of the menu happens in the thread of the script: the
/// window of the script belongs to that thread, and the `TrackPopupMenu`
/// of the machine belongs to the thread that owns the window of the
/// owner. The question goes to the window of the script with a
/// `SendMessage`, and the message is of the thread itself: the procedure
/// of the window runs at once, and the tracking happens in it.
///
/// The script waits for the menu the way the v1 waits: the answer comes
/// when the mouse chooses an item, or when the menu is dismissed.
pub fn the_menu_is_shown(
    shared: &TheShared,
    menu: usize,
    x: i32,
    y: i32,
) -> Result<Option<usize>, String> {
    {
        let mut hotkeys = shared
            .lock()
            .map_err(|_| "the machine is busy.".to_string())?;
        hotkeys.menu_request = Some(TheMenuRequest {
            menu,
            x,
            y,
            chosen: None,
        });
    }
    let window = the_window_of_the_script(shared)?;
    unsafe {
        SendMessageW(window, THE_SHOW_OF_A_MENU, 0, 0);
    }
    let mut hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    Ok(hotkeys
        .menu_request
        .take()
        .and_then(|request| request.chosen))
}

/// The modifiers of a hotkey and the key of it, split on the `+`.
///
/// This is the fallback for a name that the parser of the language
/// refuses: "Ctrl+Alt+a" is the control, the alt and the letter a, and
/// the modifiers come before the key.
fn the_parts_of_a_hotkey_split_on_the_plus(name: &str) -> (u32, u16) {
    let mut modifiers = 0u32;
    let mut key = 0u16;
    for part in name.split('+') {
        let part = part.trim();
        let modifier = match part.to_lowercase().as_str() {
            "ctrl" | "control" => Some(MOD_CONTROL),
            "alt" => Some(MOD_ALT),
            "shift" => Some(MOD_SHIFT),
            "win" | "lwin" | "rwin" => Some(MOD_WIN),
            _ => None,
        };
        match modifier {
            Some(modifier) => modifiers |= modifier,
            None => key = the_key_of(part),
        }
    }
    (modifiers, key)
}

/// The virtual key of a name of the v2.
fn the_key_of(name: &str) -> u16 {
    if let Some(key) = crate::windows::keyboard::the_virtual_key_of(name) {
        return key;
    }
    let mut letters = name.chars();
    let letter = letters.next().unwrap_or('\0');
    if letter.is_ascii_alphanumeric() {
        letter.to_ascii_uppercase() as u16
    } else {
        0
    }
}

/// The modifiers and the key of a hotkey, the way `RegisterHotKey` wants them.
///
/// The name is the spelling of the script, and the parser of the language
/// reads it: `^!z` is the control, the alt and the letter z. A name the
/// parser refuses is read the old way, split on the `+`. A hotkey that the
/// machine cannot express with `RegisterHotKey` is refused with the reason.
fn the_parts_of_a_hotkey(name: &str) -> Result<(u32, u16), String> {
    if hotstring_text(name).is_some() {
        return Err("the machine cannot watch a hotstring: it has no keyboard hook.".to_string());
    }
    // The key of the spelling may be wrapped in braces: `+{Enter}` is the
    // shift with the enter key, and the braces are not part of the name of
    // the key, so the parser reads the spelling without them.
    let spelling: String = name.chars().filter(|c| *c != '{' && *c != '}').collect();
    // The spelling of the hotkey is the one of the v2, and the name of one
    // character is the name of a key there: the machine refuses the key that
    // it does not have.
    match parse(&spelling, true) {
        Ok(spec) => {
            if spec.up {
                return Err(
                    "the machine cannot register the key coming up: it has no keyboard hook."
                        .to_string(),
                );
            }
            if spec.custom {
                return Err(
                    "the machine cannot register a hotkey with any modifier: it has no keyboard hook."
                        .to_string(),
                );
            }
            let mut modifiers = 0u32;
            for modifier in &spec.modifiers {
                let flag = match modifier.to_lowercase().as_str() {
                    "ctrl" | "lctrl" | "rctrl" | "control" => Some(MOD_CONTROL),
                    "alt" | "lalt" | "ralt" | "menu" => Some(MOD_ALT),
                    "shift" | "lshift" | "rshift" => Some(MOD_SHIFT),
                    "win" | "lwin" | "rwin" | "windows" => Some(MOD_WIN),
                    _ => None,
                };
                if let Some(flag) = flag {
                    modifiers |= flag;
                }
            }
            let key = spec.key.trim();
            let key = key
                .strip_prefix('{')
                .and_then(|rest| rest.strip_suffix('}'))
                .unwrap_or(key);
            let Some(key) = crate::windows::keyboard::the_key_of(key) else {
                return Err(format!("the machine has no key named {name}."));
            };
            Ok((modifiers, key))
        }
        Err(_) => {
            let (modifiers, key) = the_parts_of_a_hotkey_split_on_the_plus(&spelling);
            if key == 0 {
                return Err(format!("the machine has no key named {name}."));
            }
            Ok((modifiers, key))
        }
    }
}

/// A hotkey of the script is registered with the machine.
///
/// The window of the hotkeys receives the message of the hotkey, and
/// the name of it is kept with the number that the machine gave. The
/// window is made by the thread of the hotkeys, and the thread starts
/// with the first hotkey of the script: the window is made a moment
/// after the thread starts, so the wait for it is the one of
/// `the_window_of_the_hotkeys`.
pub fn register(shared: &TheShared, name: &str) -> Result<String, String> {
    let (modifiers, key) = the_parts_of_a_hotkey(name)?;
    let window = the_window_of_the_hotkeys(shared)?;
    let number = {
        let mut hotkeys = shared
            .lock()
            .map_err(|_| "the machine is busy.".to_string())?;
        let number = hotkeys.next;
        hotkeys.next += 1;
        number
    };
    // The window belongs to the thread of the machine, and `RegisterHotKey`
    // has to be called by the thread that owns the window: the question is
    // sent to the window, and the procedure of it registers the hotkey.
    // The lock is not held while the message is on its way.
    let lparam = packed(key, modifiers | MOD_NOREPEAT);
    let answer =
        unsafe { SendMessageW(window, THE_REGISTER_OF_A_HOTKEY, number as WPARAM, lparam) };
    if answer != 0 {
        // Measured on the binary of v1.1.37.02: the hotkey
        // that the machine already has is not a failure of the script --
        // the `Hotkey` of a combination that is registered gives the new
        // action to the hotkey that is there, and a script of the v1
        // registers the same combination twice. 1409 is
        // ERROR_HOTKEY_ALREADY_REGISTERED.
        if answer == 1409 {
            let mut hotkeys = shared
                .lock()
                .map_err(|_| "the machine is busy.".to_string())?;
            hotkeys.registered.push((number, name.to_string()));
            return Ok(name.to_string());
        }
        return Err(format!(
            "the machine did not take the hotkey: the code is {answer}."
        ));
    }
    let mut hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    hotkeys.registered.push((number, name.to_string()));
    Ok(name.to_string())
}

/// A hotkey of the script is given back to the machine.
pub fn unregister(shared: &TheShared, name: &str) -> Result<String, String> {
    let number = {
        let hotkeys = shared
            .lock()
            .map_err(|_| "the machine is busy.".to_string())?;
        let Some(number) = hotkeys
            .registered
            .iter()
            .find(|(_, registered)| registered == name)
            .map(|(number, _)| *number)
        else {
            return Err(format!("the machine has no hotkey named {name}."));
        };
        number
    };
    let window = the_window_of_the_hotkeys(shared)?;
    // The hotkey is given back by the thread that owns the window, the
    // same way it was registered.
    unsafe {
        SendMessageW(window, THE_GIVE_BACK_OF_A_HOTKEY, number as WPARAM, 0);
    }
    let mut hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    hotkeys.registered.retain(|(other, _)| *other != number);
    Ok(name.to_string())
}

/// The hotkeys of the script, the ones that are registered.
pub fn the_hotkeys(shared: &TheShared) -> Result<Vec<String>, String> {
    let hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    Ok(hotkeys
        .registered
        .iter()
        .map(|(_, name)| name.clone())
        .collect())
}

/// The hotkey that fired, when one of them fired.
pub fn the_fired(shared: &TheShared) -> Result<Option<String>, String> {
    let mut hotkeys = shared
        .lock()
        .map_err(|_| "the machine is busy.".to_string())?;
    Ok(hotkeys.fired.pop())
}

/// The run waits for an event of the machine.
///
/// The wait is the one of the machine: the thread that watches the
/// keyboard tells when a key was pressed, and the run does not ask at
/// every instant. The answer says that something arrived -- and the
/// hotkey that arrived stays in the queue, for the run to read it
/// later.
///
/// The wait ends when the time ends: the answer is that nothing
/// arrived.
/// Watch a message of the window of the script.
///
/// The `OnMessage` registers a handler for the number, and the window
/// queues the message when it arrives.
pub fn watch_a_message(shared: &TheShared, number: i64) -> Result<(), String> {
    let mut hotkeys = shared.lock().map_err(|_| "the machine is busy.".to_string())?;
    if !hotkeys.watched_messages.contains(&number) {
        hotkeys.watched_messages.push(number);
    }
    Ok(())
}

/// Whether the script watches a message with `OnMessage`.
///
/// The procedure of the window of the script asks this before it does
/// anything else with a message: the reference gives the message to the
/// monitor of the script first, and the monitor runs in a thread of its
/// own (`source/script_gui.cpp:7930` of the v1, the procedure of the
/// window of a GUI).
pub fn the_message_is_watched(number: u32) -> bool {
    let Ok(machine) = THE_MACHINE.lock() else {
        return false;
    };
    let Some(shared) = machine.as_ref() else {
        return false;
    };
    let Ok(hotkeys) = shared.lock() else {
        return false;
    };
    hotkeys.watched_messages.contains(&(number as i64))
}

/// The events of the window of the script, taken whole.
pub fn the_events(shared: &TheShared) -> Result<Vec<String>, String> {
    let mut hotkeys = shared.lock().map_err(|_| "the machine is busy.".to_string())?;
    Ok(core::mem::take(&mut hotkeys.events))
}

pub fn an_event_is_waited(shared: &TheShared, timeout: u64) -> Result<bool, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout);
    loop {
        let rest = deadline.saturating_duration_since(Instant::now());
        if rest.is_zero() {
            return Ok(false);
        }
        // The lock is taken only to read the list and to copy the event:
        // it is let go BEFORE the wait. The thread that pumps the
        // messages takes this same lock to answer the tray and the
        // hotkeys, so a wait with the lock taken starves that thread and
        // freezes the menu of the tray. The event is a handle of the
        // machine, and a copy of it waits on the same notice.
        let event = {
            let hotkeys = shared
                .lock()
                .map_err(|_| "the machine is busy.".to_string())?;
            if !hotkeys.fired.is_empty() {
                return Ok(true);
            }
            hotkeys.fired_now.clone()
        };
        event.wait(rest.as_millis() as u64);
    }
}

/// The event of the machine that says that a hotkey fired.
///
/// The event is of the kernel32, and is of Windows 2000: the
/// `Condvar` of the `std` would ask the machine for the
/// `SleepConditionVariableSRW` of Windows Vista.
#[derive(Debug)]
#[derive(Clone)]
pub struct TheEvent {
    handle: windows_sys::Win32::Foundation::HANDLE,
}

impl TheEvent {
    /// An event of one use: the notice of the keyboard reaches one, and
    /// the one that waits sleeps until the next.
    fn new() -> Self {
        let handle = unsafe {
            windows_sys::Win32::System::Threading::CreateEventW(
                std::ptr::null_mut(),
                0,
                0,
                std::ptr::null(),
            )
        };
        TheEvent { handle }
    }

    /// The notice: a hotkey fired.
    fn signal(&self) {
        unsafe { windows_sys::Win32::System::Threading::SetEvent(self.handle) };
    }

    /// The wait: it sleeps until the notice or until the end of the time.
    fn wait(&self, millis: u64) {
        unsafe {
            windows_sys::Win32::System::Threading::WaitForSingleObject(
                self.handle,
                millis.min(u32::MAX as u64) as u32,
            )
        };
    }
}

impl Default for TheEvent {
    fn default() -> Self {
        Self::new()
    }
}

// The handle is a number of the machine, and the machine lets it pass
// from one thread to the other.
unsafe impl Send for TheEvent {}
unsafe impl Sync for TheEvent {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_modifiers_of_a_hotkey_are_the_letters_of_it() {
        assert_eq!(the_parts_of_a_hotkey("a").unwrap(), (0, u16::from(b'A')));
        assert_eq!(the_parts_of_a_hotkey("Ctrl+a").unwrap().0, MOD_CONTROL);
        assert_eq!(
            the_parts_of_a_hotkey("Ctrl+Alt+a").unwrap().0,
            MOD_CONTROL | MOD_ALT
        );
        assert_eq!(
            the_parts_of_a_hotkey("Ctrl+Alt+Shift+Win+a").unwrap().0,
            MOD_CONTROL | MOD_ALT | MOD_SHIFT | MOD_WIN
        );
        assert_eq!(the_parts_of_a_hotkey("Ctrl+Enter").unwrap().1, 0x0D);
    }

    #[test]
    fn the_spelling_of_the_script_is_read_by_the_parser() {
        assert_eq!(
            the_parts_of_a_hotkey("^!z").unwrap(),
            (MOD_CONTROL | MOD_ALT, 0x5A)
        );
        assert_eq!(
            the_parts_of_a_hotkey("#a").unwrap(),
            (MOD_WIN, u16::from(b'A'))
        );
        assert_eq!(
            the_parts_of_a_hotkey("+{Enter}").unwrap(),
            (MOD_SHIFT, 0x0D)
        );
    }

    #[test]
    fn the_old_spelling_is_the_fallback_of_the_parser() {
        assert_eq!(
            the_parts_of_a_hotkey("Ctrl+Alt+a").unwrap(),
            (MOD_CONTROL | MOD_ALT, u16::from(b'A'))
        );
    }

    #[test]
    fn a_hotkey_the_machine_cannot_express_is_refused_with_the_reason() {
        assert_eq!(
            the_parts_of_a_hotkey("^!z up").unwrap_err(),
            "the machine cannot register the key coming up: it has no keyboard hook."
        );
        assert_eq!(
            the_parts_of_a_hotkey("*^a").unwrap_err(),
            "the machine cannot register a hotkey with any modifier: it has no keyboard hook."
        );
        assert_eq!(
            the_parts_of_a_hotkey("::aa::").unwrap_err(),
            "the machine cannot watch a hotstring: it has no keyboard hook."
        );
    }

    #[test]
    fn a_key_the_machine_does_not_have_is_refused() {
        assert_eq!(
            the_parts_of_a_hotkey("^!notakey").unwrap_err(),
            "the machine has no key named ^!notakey."
        );
    }

    #[test]
    fn a_hotkey_of_the_machine_is_registered_and_given_back() {
        let shared: TheShared = Arc::new(Mutex::new(TheHotkeys::default()));
        // The window of the hotkeys is the one of the thread of the
        // machine: the test starts it like the script does.
        the_thread_starts(&shared).expect("the thread of the hotkeys starts");
        // The thread makes the window of the hotkeys: the test waits for
        // it, because the window is made by the thread.
        let mut waited = 0;
        loop {
            {
                let hotkeys = shared.lock().expect("the machine is free");
                if hotkeys.hotkeys_window.is_some() {
                    break;
                }
            }
            waited += 10;
            assert!(waited < 5000, "the thread did not make the window");
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let registered = match register(&shared, "Ctrl+Alt+F9") {
            Ok(registered) => registered,
            Err(reason) => {
                // The hotkeys of the machine are a table of the session,
                // and another program of it may have the one of the test.
                assert!(reason.contains("1408"), "{reason}");
                return;
            }
        };
        assert_eq!(registered, "Ctrl+Alt+F9");
        let hotkeys = the_hotkeys(&shared).expect("the hotkeys are listed");
        assert_eq!(hotkeys, vec!["Ctrl+Alt+F9".to_string()]);
        let given_back = unregister(&shared, "Ctrl+Alt+F9").expect("the hotkey is given back");
        assert_eq!(given_back, "Ctrl+Alt+F9");
        let hotkeys = the_hotkeys(&shared).expect("the hotkeys are listed");
        assert!(hotkeys.is_empty());
    }

    #[test]
    fn a_hotkey_that_the_machine_does_not_have_is_refused() {
        let shared: TheShared = Arc::new(Mutex::new(TheHotkeys::default()));
        let answer = register(&shared, "Ctrl+Alt+NoSuchKey");
        assert!(answer.is_err());
    }

    /// The window of the script belongs to the thread of the script.
    ///
    /// This is the difference between the tree and the v1: the `A_ScriptHwnd`
    /// of the v1 is the handle of a window of the thread that runs the
    /// script, and the hook of the messages of that thread sees the
    /// messages that arrive in it.
    ///
    /// Measured on the binary of v1.1.37.02: the `GetCurrentThreadId` of
    /// the script and the `GetWindowThreadProcessId` of the `A_ScriptHwnd`
    /// are the same number.
    #[test]
    fn the_window_of_the_script_belongs_to_the_thread_of_the_script() {
        use windows_sys::Win32::System::Threading::GetCurrentThreadId;
        use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
        let shared: TheShared = Arc::new(Mutex::new(TheHotkeys::default()));
        the_thread_starts(&shared).expect("the thread of the hotkeys starts");
        // The window of the script is made by the thread of the script:
        // the test makes it, the way the script does.
        let window = the_script_window_is_made(&shared).expect("the window of the script is made");
        let thread_of_the_script = unsafe { GetCurrentThreadId() };
        let thread_of_the_window =
            unsafe { GetWindowThreadProcessId(window, std::ptr::null_mut()) };
        assert_eq!(
            thread_of_the_script, thread_of_the_window,
            "the window of the script is of another thread"
        );
        // The window of the hotkeys is of the thread of the machine, and
        // the hotkeys keep arriving at once there.
        let hotkeys_window = the_window_of_the_hotkeys(&shared).expect("the window of the hotkeys");
        let thread_of_the_hotkeys =
            unsafe { GetWindowThreadProcessId(hotkeys_window, std::ptr::null_mut()) };
        assert_ne!(
            thread_of_the_script, thread_of_the_hotkeys,
            "the window of the hotkeys is of the thread of the script"
        );
    }
}
