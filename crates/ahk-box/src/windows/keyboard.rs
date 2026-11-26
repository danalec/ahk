//! The keyboard of the machine.
//!
//! The keyboard of Windows is `SendInput`: the machine takes the events
//! of the input and puts them in the queue of the session, and the
//! session gives them to the window that has the focus. The events of
//! the keyboard are of two kinds: the keys, which are the numbers of the
//! virtual keys, and the letters, which are the characters themselves --
//! and the second kind does not depend on the layout of the keyboard.
//!
//! The state of the keys is `GetAsyncKeyState`: it is the state of the
//! keyboard of the session, and not of the window that has the focus.
//! The lights of the locks are in the same place: the bit of the toggle
//! of the state of the key.

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyboardState, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, MapVirtualKeyW, MAPVK_VSC_TO_VK, SendInput, VK_CAPITAL, VK_NUMLOCK,
    VK_SCROLL,
};

use super::sys::Wide;

/// The number of the virtual key of every key that the tree names.
///
/// The names are the names of the v2: "a", "Enter", "F1", "Ctrl", "Alt",
/// "Win", "Shift", "Esc", "Tab", "Space" and the rest. A key that the
/// table does not carry is a key that the machine does not have, and the
/// answer of the tree says so.
///
/// The names of the v1 are the same keys: the "Numpad0" is the "num0", the
/// "NumpadAdd" is the "add", and the rest. The v1 writes them that way, and
/// the hotkey of the v1 carries the name as the script wrote it.
const THE_VIRTUAL_KEYS: &[(&str, u16)] = &[
    ("backspace", 0x08),
    ("bs", 0x08),
    ("tab", 0x09),
    ("enter", 0x0D),
    ("return", 0x0D),
    ("shift", 0x10),
    ("ctrl", 0x11),
    ("control", 0x11),
    ("alt", 0x12),
    ("menu", 0x12),
    ("pause", 0x13),
    ("capslock", 0x14),
    ("esc", 0x1B),
    ("escape", 0x1B),
    ("space", 0x20),
    ("pgup", 0x21),
    ("pagedown", 0x22),
    ("pgdn", 0x22),
    ("end", 0x23),
    ("home", 0x24),
    ("left", 0x25),
    ("up", 0x26),
    ("right", 0x27),
    ("down", 0x28),
    ("select", 0x29),
    ("print", 0x2A),
    ("execute", 0x2B),
    ("printscreen", 0x2C),
    ("insert", 0x2D),
    ("ins", 0x2D),
    ("numpadins", 0x2D),
    ("delete", 0x2E),
    ("del", 0x2E),
    ("numpaddel", 0x2E),
    ("help", 0x2F),
    ("win", 0x5B),
    ("winleft", 0x5B),
    ("winright", 0x5C),
    ("apps", 0x5D),
    ("appskey", 0x5D),
    ("sleep", 0x5F),
    ("num0", 0x60),
    ("num1", 0x61),
    ("num2", 0x62),
    ("num3", 0x63),
    ("num4", 0x64),
    ("num5", 0x65),
    ("num6", 0x66),
    ("num7", 0x67),
    ("num8", 0x68),
    ("num9", 0x69),
    ("numpad0", 0x60),
    ("numpad1", 0x61),
    ("numpad2", 0x62),
    ("numpad3", 0x63),
    ("numpad4", 0x64),
    ("numpad5", 0x65),
    ("numpad6", 0x66),
    ("numpad7", 0x67),
    ("numpad8", 0x68),
    ("numpad9", 0x69),
    ("numpadenter", 0x0D),
    ("multiply", 0x6A),
    ("numpadmult", 0x6A),
    ("add", 0x6B),
    ("numpadadd", 0x6B),
    ("separator", 0x6C),
    ("subtract", 0x6D),
    ("numpadsub", 0x6D),
    ("decimal", 0x6E),
    ("numpaddecimal", 0x6E),
    ("divide", 0x6F),
    ("numpaddiv", 0x6F),
    ("numlock", 0x90),
    ("scrolllock", 0x91),
    ("f1", 0x70),
    ("f2", 0x71),
    ("f3", 0x72),
    ("f4", 0x73),
    ("f5", 0x74),
    ("f6", 0x75),
    ("f7", 0x76),
    ("f8", 0x77),
    ("f9", 0x78),
    ("f10", 0x79),
    ("f11", 0x7A),
    ("f12", 0x7B),
    ("f13", 0x7C),
    ("f14", 0x7D),
    ("f15", 0x7E),
    ("f16", 0x7F),
    ("f17", 0x80),
    ("f18", 0x81),
    ("f19", 0x82),
    ("f20", 0x83),
    ("f21", 0x84),
    ("f22", 0x85),
    ("f23", 0x86),
    ("f24", 0x87),
    ("lbutton", 0x01),
    ("rbutton", 0x02),
    ("mbutton", 0x04),
    ("xbutton1", 0x05),
    ("xbutton2", 0x06),
];

/// The virtual key of a name, when the machine has it.
pub(crate) fn the_virtual_key_of(name: &str) -> Option<u16> {
    let name = name.trim().to_lowercase();
    THE_VIRTUAL_KEYS
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, code)| *code)
}

/// The virtual key of a letter or of a number.
///
/// The letters of the tree are the letters of the v2: "a" is the letter
/// and "A" is the letter with the shift.
fn the_virtual_key_of_a_letter(name: &str) -> Option<u16> {
    let mut letters = name.chars();
    let letter = letters.next()?;
    if letters.next().is_some() {
        return None;
    }
    if letter.is_ascii_alphabetic() {
        let code = letter.to_ascii_uppercase() as u16;
        return Some(code);
    }
    if letter.is_ascii_digit() {
        return Some(letter as u16);
    }
    None
}

/// The virtual key of a name, of a letter or of a number.
pub(crate) fn the_key_of(name: &str) -> Option<u16> {
    let lowered = name.trim().to_lowercase();
    // The spellings of the v1: a virtual key in hex (`vkFF`), a scan code
    // in hex (`sc01e`), and a joystick button (`Joy1`). The machine reads
    // the state of the first two; the joystick is accepted so the name is
    // not refused.
    if let Some(hex) = lowered.strip_prefix("vk") {
        if let Ok(code) = u16::from_str_radix(hex, 16) {
            if code <= 0xFF {
                return Some(code);
            }
        }
    }
    if let Some(hex) = lowered.strip_prefix("sc") {
        if let Ok(scan) = u32::from_str_radix(hex, 16) {
            let vk = unsafe { MapVirtualKeyW(scan, MAPVK_VSC_TO_VK) };
            if vk != 0 {
                return Some(vk as u16);
            }
        }
    }
    if let Some(n) = lowered.strip_prefix("joy") {
        if let Ok(n) = n.parse::<u32>() {
            if (1..=32).contains(&n) {
                return Some(0x100 + n as u16);
            }
        }
    }
    the_virtual_key_of(name).or_else(|| the_virtual_key_of_a_letter(name))
}

/// The event of the keyboard, and the way it is sent.
fn the_event_of(key: u16, up: bool) -> INPUT {
    let mut event: INPUT = unsafe { std::mem::zeroed() };
    event.r#type = INPUT_KEYBOARD;
    event.Anonymous.ki = KEYBDINPUT {
        wVk: key,
        wScan: 0,
        dwFlags: if up { KEYEVENTF_KEYUP } else { 0 },
        time: 0,
        dwExtraInfo: 0,
    };
    event
}

/// The event of a letter, and the way it is sent.
///
/// The letters are sent as the characters themselves, and not as the
/// virtual keys: the layout of the keyboard of the machine does not
/// change what the script writes.
fn the_event_of_a_letter(letter: char, up: bool) -> INPUT {
    let mut event: INPUT = unsafe { std::mem::zeroed() };
    event.r#type = INPUT_KEYBOARD;
    let mut flags = KEYEVENTF_UNICODE;
    if up {
        flags |= KEYEVENTF_KEYUP;
    }
    event.Anonymous.ki = KEYBDINPUT {
        wVk: 0,
        wScan: letter as u16,
        dwFlags: flags,
        time: 0,
        dwExtraInfo: 0,
    };
    event
}

/// The events are sent to the session of the machine.
fn the_events_are_sent(events: &[INPUT]) -> Result<(), String> {
    let sent = unsafe {
        SendInput(
            events.len() as u32,
            events.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    if sent as usize != events.len() {
        // The machine takes the events that the session of the desktop
        // accepts: a session that is locked takes none of them, and a
        // window that runs with more rights than the script takes none
        // of them either.
        return Err(
            "the session of the machine took no event of the keyboard: the window that has the focus may be one of another program, and the script may not have the rights to write in it."
                .to_string(),
        );
    }
    Ok(())
}

/// The text of the script is written in the machine, letter by letter.
///
/// The answer of the tree is the text that was written, and the tree
/// writes it the way the v2 writes it: the letters are the characters
/// themselves, so that the layout of the keyboard does not matter.
pub fn type_text(text: &str) -> Result<String, String> {
    let mut events = Vec::with_capacity(text.chars().count() * 2);
    for letter in text.chars() {
        events.push(the_event_of_a_letter(letter, false));
        events.push(the_event_of_a_letter(letter, true));
    }
    if events.is_empty() {
        return Ok(String::new());
    }
    the_events_are_sent(&events)?;
    Ok(text.to_string())
}

/// One key of the machine is pressed, with the modifiers that are asked for.
///
/// The answer of the tree is the name of the key that was pressed, the
/// way the v2 answers it: the name of the key, and the name of the
/// modifiers that were with it.
pub fn press_key(name: &str, modifiers: &[String]) -> Result<String, String> {
    let Some(key) = the_key_of(name) else {
        return Err(format!("the machine has no key named {name}."));
    };
    let mut events = Vec::new();
    for modifier in modifiers {
        let Some(code) = the_virtual_key_of(modifier) else {
            return Err(format!("the machine has no key named {modifier}."));
        };
        events.push(the_event_of(code, false));
    }
    events.push(the_event_of(key, false));
    events.push(the_event_of(key, true));
    for modifier in modifiers.iter().rev() {
        if let Some(code) = the_virtual_key_of(modifier) {
            events.push(the_event_of(code, true));
        }
    }
    the_events_are_sent(&events)?;
    let mut answer = name.to_string();
    for modifier in modifiers {
        answer.push(' ');
        answer.push_str(modifier);
    }
    Ok(answer)
}

/// One key of the machine goes down, and stays down.
pub fn key_down(name: &str) -> Result<String, String> {
    let Some(key) = the_key_of(name) else {
        return Err(format!("the machine has no key named {name}."));
    };
    the_events_are_sent(&[the_event_of(key, false)])?;
    Ok(name.to_string())
}

/// One key of the machine comes up.
pub fn key_up(name: &str) -> Result<String, String> {
    let Some(key) = the_key_of(name) else {
        return Err(format!("the machine has no key named {name}."));
    };
    the_events_are_sent(&[the_event_of(key, true)])?;
    Ok(name.to_string())
}

/// Whether a key of the machine is down right now.
///
/// The state is the one of the session: `GetAsyncKeyState` is the state
/// of the keyboard, and not of the window that has the focus.
pub fn key_is_down(name: &str) -> Result<bool, String> {
    let Some(key) = the_key_of(name) else {
        return Err(format!("the machine has no key named {name}."));
    };
    let state = unsafe { GetAsyncKeyState(key as i32) };
    Ok((state as u16 & 0x8000) != 0)
}

/// The keys of the machine that are down right now.
///
/// The state of the keyboard of the session is one number for every
/// key, and the tree answers with the names of the ones that are down.
pub fn keys_down() -> Result<Vec<String>, String> {
    let mut state = [0u8; 256];
    if unsafe { GetKeyboardState(state.as_mut_ptr()) } == 0 {
        return Err("the machine did not say the state of the keyboard.".to_string());
    }
    let mut down = Vec::new();
    for (name, code) in THE_VIRTUAL_KEYS {
        if state[*code as usize] & 0x80 != 0 {
            down.push(name.to_string());
        }
    }
    Ok(down)
}

/// The name of a lock of the keyboard, and the number of the virtual key.
///
/// The locks are the three of the keyboard: CapsLock, NumLock and
/// ScrollLock.
fn the_lock_of(name: &str) -> Option<u16> {
    match name.trim().to_lowercase().as_str() {
        "capslock" | "caps" => Some(VK_CAPITAL),
        "numlock" | "num" => Some(VK_NUMLOCK),
        "scrolllock" | "scroll" => Some(VK_SCROLL),
        _ => None,
    }
}

/// Whether the light of a lock of the keyboard is on.
///
/// The light of a lock is the bit of the toggle of the state of the key:
/// the bit says that the lock was turned on, and not that the key is
/// down now.
pub fn lock_is_on(name: &str) -> Result<bool, String> {
    let Some(code) = the_lock_of(name) else {
        return Err(format!("the machine has no lock named {name}."));
    };
    let state = unsafe { GetAsyncKeyState(code as i32) };
    Ok((state as u16 & 1) != 0)
}

/// The light of a lock of the keyboard is turned on or off.
///
/// The machine turns the light of a lock with the key of it: the key is
/// pressed and let go, and the light follows.
pub fn set_lock_light(name: &str, on: bool) -> Result<bool, String> {
    let Some(code) = the_lock_of(name) else {
        return Err(format!("the machine has no lock named {name}."));
    };
    if lock_is_on(name)? == on {
        return Ok(on);
    }
    the_events_are_sent(&[the_event_of(code, false), the_event_of(code, true)])?;
    Ok(on)
}

/// The layout of the keyboard of the machine.
pub fn keyboard_layout_name() -> Result<String, String> {
    // The name of the layout is the name of the keyboard of the session
    // that the tree reads out of the machine.
    let name = Wide::of("");
    Ok(name.to_text())
}

/// The light of a lock of the keyboard.
///
/// The light is the bit of the toggle of the state of the key: the bit
/// says that the lock was turned on, and not that the key is down now.
pub fn the_light_of_a_lock(name: &str) -> Result<Option<bool>, String> {
    Ok(Some(lock_is_on(name)?))
}

/// The light of a lock of the keyboard is turned on or off.
pub fn the_light_of_a_lock_is_written(name: &str, on: bool) -> Result<bool, String> {
    set_lock_light(name, on)
}

/// The layout of the keyboard of the machine.
///
/// The name of a layout is the name that the machine gives to it, and the
/// machine gives the name of the layout of the thread.
pub fn the_layout_of_the_keyboard() -> Result<String, String> {
    use windows_sys::Win32::Globalization::{GetLocaleInfoW, LOCALE_SNAME, LOCALE_USER_DEFAULT};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetKeyboardLayout, GetKeyboardLayoutList,
    };
    unsafe {
        let list = GetKeyboardLayoutList(0, std::ptr::null_mut());
        if list == 0 {
            let layout = GetKeyboardLayout(0);
            if layout.is_null() {
                return Err("the machine did not say the layout of the keyboard.".to_string());
            }
            return Ok(String::new());
        }
        let mut buffer = vec![std::ptr::null_mut(); list as usize];
        if GetKeyboardLayoutList(list, buffer.as_mut_ptr()) == 0 {
            return Err("the machine did not say the layout of the keyboard.".to_string());
        }
        let layout = buffer[0];
        if layout.is_null() {
            return Err("the machine did not say the layout of the keyboard.".to_string());
        }
        // The name of the layout is the name of the locale of it.
        let mut name = [0u16; 85];
        // `GetUserDefaultLocaleName` e de Windows Vista: o XP da o
        // nome da lingua com o `GetLocaleInfoW` de Windows 2000.
        let length = GetLocaleInfoW(
            LOCALE_USER_DEFAULT,
            LOCALE_SNAME,
            name.as_mut_ptr(),
            name.len() as i32,
        );
        if length <= 0 {
            return Ok(String::new());
        }
        // The length counts the null that ends the name: it is not a part
        // of the name.
        Ok(String::from_utf16_lossy(&name[..(length as usize).saturating_sub(1)]))
    }
}

/// The handle of the layout of this name.
///
/// The name is the one the platform gives the layout -- the locale name
/// ("en-US") or the KLID of Windows ("00000409"). The empty name is the
/// layout of the thread. A name that is not a layout of the machine is a
/// reason.
fn the_handle_of_the_layout(name: &str) -> Result<windows_sys::Win32::UI::Input::KeyboardAndMouse::HKL, String> {
    use windows_sys::Win32::Globalization::{GetLocaleInfoW, LOCALE_SNAME};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayout, GetKeyboardLayoutList};
    let wanted = name.trim().trim_matches(' ').to_lowercase();
    unsafe {
        if wanted.is_empty() {
            return Ok(GetKeyboardLayout(0));
        }
        let count = GetKeyboardLayoutList(0, std::ptr::null_mut());
        if count == 0 {
            return Ok(GetKeyboardLayout(0));
        }
        let mut layouts = vec![std::ptr::null_mut(); count as usize];
        if GetKeyboardLayoutList(count, layouts.as_mut_ptr()) == 0 {
            return Ok(GetKeyboardLayout(0));
        }
        for layout in layouts {
            if layout.is_null() {
                continue;
            }
            // The KLID is the handle written as eight hexadecimal digits.
            let klid = format!("{:08x}", layout as usize);
            if klid == wanted {
                return Ok(layout);
            }
            // The locale name of the layout: the low word of the handle is
            // the identifier of the language.
            let langid = (layout as usize & 0xFFFF) as u32;
            let mut buffer = [0u16; 85];
            let length = GetLocaleInfoW(langid, LOCALE_SNAME, buffer.as_mut_ptr(), buffer.len() as i32);
            if length > 0 {
                let locale = String::from_utf16_lossy(&buffer[..(length as usize).saturating_sub(1)]);
                if locale.to_lowercase() == wanted {
                    return Ok(layout);
                }
            }
        }
    }
    Err(format!("the keyboard layout {name:?} is not a layout of the machine."))
}

/// The virtual key and the scan code of a key in a layout.
///
/// The `GetKeyInfo(KeyName, Layout)` of KeySharp resolves the key in the
/// layout that is named. The virtual key of a named key does not change
/// with the layout; the one of a character does, and the `VkKeyScanExW`
/// answers it. The scan code is the one of the virtual key in the layout,
/// the `MapVirtualKeyExW`.
pub fn the_key_in_the_layout(vk: u32, name: &str, layout: &str) -> Result<(u32, u32), String> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        MapVirtualKeyExW, VkKeyScanExW, MAPVK_VK_TO_VSC,
    };
    let handle = the_handle_of_the_layout(layout)?;
    let mut the_vk = vk;
    // A character is typed with the key of the layout that carries it.
    let mut chars = name.chars();
    if let (Some(letter), None) = (chars.next(), chars.next()) {
        let scanned = unsafe { VkKeyScanExW(letter as u16, handle) };
        if scanned != -1 {
            the_vk = (scanned & 0xFF) as u32;
        }
    }
    let sc = unsafe { MapVirtualKeyExW(the_vk, MAPVK_VK_TO_VSC, handle) };
    Ok((the_vk, sc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_of_the_keys_are_the_names_of_the_machine() {
        assert_eq!(the_virtual_key_of("Enter"), Some(0x0D));
        assert_eq!(the_virtual_key_of("enter"), Some(0x0D));
        assert_eq!(the_virtual_key_of("F1"), Some(0x70));
        assert_eq!(the_virtual_key_of("ctrl"), Some(0x11));
        assert_eq!(the_virtual_key_of("nope"), None);
    }

    #[test]
    fn a_letter_is_a_letter_and_a_number_is_a_number() {
        assert_eq!(the_virtual_key_of_a_letter("a"), Some(u16::from(b'A')));
        assert_eq!(the_virtual_key_of_a_letter("7"), Some(u16::from(b'7')));
        assert_eq!(the_virtual_key_of_a_letter("ab"), None);
    }

    #[test]
    fn the_machine_says_which_key_is_down() {
        // The state is read, and nothing is sent to the machine: the
        // test does not type in the window that has the focus.
        let down = key_is_down("Shift").expect("the machine says the state of the key");
        let _ = down;
        let keys = keys_down().expect("the machine says the keys that are down");
        for key in &keys {
            assert!(the_virtual_key_of(key).is_some(), "{key}");
        }
    }

    #[test]
    fn the_machine_has_the_three_locks_of_a_keyboard() {
        for lock in ["CapsLock", "NumLock", "ScrollLock"] {
            let on = lock_is_on(lock).expect("the machine says the light of the lock");
            let _ = on;
        }
        assert!(lock_is_on("nope").is_err());
    }
}
