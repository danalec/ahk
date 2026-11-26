//! The pointer of the machine.
//!
//! The pointer of Windows is `SendInput` as well: the machine takes the
//! events of the mouse and puts them in the queue of the session. The
//! place of the pointer is set with `SetCursorPos`, and the session
//! draws it there.

use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_MOUSE, MOUSEEVENTF_HWHEEL, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT,
    SendInput,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    ClipCursor, SetCursorPos, XBUTTON1, XBUTTON2,
};

/// The event of the mouse, and the way it is sent.
fn the_event_of(x: i32, y: i32, flags: u32, button: u32) -> INPUT {
    let mut event: INPUT = unsafe { std::mem::zeroed() };
    event.r#type = INPUT_MOUSE;
    event.Anonymous.mi = MOUSEINPUT {
        dx: x,
        dy: y,
        mouseData: button,
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
        return Err("the session of the machine took no event of the mouse.".to_string());
    }
    Ok(())
}

/// The pointer of the machine is put in a place.
///
/// The move is at once. The speed of `SetDefaultMouseSpeed` is ignored
/// here, the way the v2.0.31 ignores it for the `SendInput` it reads in
/// its own source: "Mouse speed (aSpeed) is ignored for SendInput/Play
/// mode: the mouse always moves instantaneously"
/// (`source/keyboard_mouse.cpp`, `MouseMove`). The same source says the
/// delay of `SetMouseDelay` does nothing in the `SendInput` mode:
/// `DoMouseDelay()` "Inserts delay for all modes except SendInput, for
/// which it does nothing". The tree sends with `SendInput`, so both
/// settings are of the script and not of this move.
///
/// The answer of the tree is the place, the way the v2 answers it.
pub fn move_pointer(x: i64, y: i64) -> Result<String, String> {
    if unsafe { SetCursorPos(x as i32, y as i32) } == 0 {
        return Err("the machine did not move the pointer.".to_string());
    }
    Ok(format!("{x} {y}"))
}

/// The pointer of the machine is clicked.
///
/// The button is the name of the v2: "Left", "Right" or "Middle". The
/// answer of the tree is the place of the click, the way the v2 answers
/// it.
pub fn click(x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
    let (down, up, data) = the_buttons_of(button)?;
    unsafe { SetCursorPos(x as i32, y as i32) };
    for _ in 0..count.max(1) {
        let events = [
            the_event_of(x as i32, y as i32, MOUSEEVENTF_MOVE, 0),
            the_event_of(0, 0, down, data),
            the_event_of(0, 0, up, data),
        ];
        the_events_are_sent(&events)?;
    }
    Ok(format!("{x} {y}"))
}

/// The flags and the data of a button of the mouse.
///
/// O quarto e o quinto botao sao os que o `MOUSEEVENTF_XDOWN` e o
/// `MOUSEEVENTF_XUP` acendem, e o dado do evento e o numero deles.
fn the_buttons_of(button: &str) -> Result<(u32, u32, u32), String> {
    let answer = match button.trim().to_lowercase().as_str() {
        "left" | "l" | "1" => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, 0),
        "right" | "r" | "2" => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, 0),
        "middle" | "m" | "3" => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, 0),
        "x1" | "xbutton1" | "4" => (MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, XBUTTON1 as u32),
        "x2" | "xbutton2" | "5" => (MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, XBUTTON2 as u32),
        other => {
            return Err(format!(
                "the machine has no button named {other}: the buttons are Left, Right, Middle, X1 and X2."
            ));
        }
    };
    Ok(answer)
}

/// A roda da maquina gira.
///
/// A direcao e o nome de uma das quatro rodas -- a de cima, a de baixo,
/// a da esquerda e a da direita -- e o numero e quantos dentes ela
/// gira.
///
/// Medido no binario do v2.0.31: o `MouseClick "WheelUp"`
/// da "ok", e a roda e a do `MOUSEEVENTF_WHEEL` da maquina.
pub fn mouse_wheel(
    x: Option<i64>,
    y: Option<i64>,
    direction: &str,
    amount: u32,
) -> Result<String, String> {
    if let (Some(x), Some(y)) = (x, y) {
        unsafe { SetCursorPos(x as i32, y as i32) };
    }
    let (flags, data) = match direction.trim().to_lowercase().as_str() {
        "wheelup" | "wu" | "up" => (MOUSEEVENTF_WHEEL, 120i32),
        "wheeldown" | "wd" | "down" => (MOUSEEVENTF_WHEEL, -120i32),
        "wheelleft" | "wl" | "left" => (MOUSEEVENTF_HWHEEL, -120i32),
        "wheelright" | "wr" | "right" => (MOUSEEVENTF_HWHEEL, 120i32),
        other => {
            return Err(format!(
                "the machine has no wheel named {other}: the wheels are WheelUp, WheelDown, WheelLeft and WheelRight."
            ));
        }
    };
    let teeth = amount.max(1) as i32;
    for _ in 0..teeth {
        let events = [the_event_of(0, 0, flags, data as u32)];
        the_events_are_sent(&events)?;
    }
    Ok(String::new())
}

/// The button of the mouse goes down, and stays down.
///
/// The answer of the tree is the place of the button, the way the v2
/// answers it.
pub fn mouse_down(x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
    // The v2 moves the pointer to the place when the button goes down,
    // and the machine does the same.
    if let (Some(x), Some(y)) = (x, y) {
        unsafe { SetCursorPos(x as i32, y as i32) };
    }
    let (flags, _, data) = the_buttons_of(button)?;
    let events = [the_event_of(0, 0, flags, data)];
    the_events_are_sent(&events)?;
    Ok(String::new())
}

/// The button of the mouse comes up.
pub fn mouse_up(x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
    if let (Some(x), Some(y)) = (x, y) {
        unsafe { SetCursorPos(x as i32, y as i32) };
    }
    let (_, flags, data) = the_buttons_of(button)?;
    let events = [the_event_of(0, 0, flags, data)];
    the_events_are_sent(&events)?;
    Ok(String::new())
}

/// Where the pointer of the machine is.
pub fn the_place_of_the_pointer() -> Result<(i64, i64), String> {
    let mut point = windows_sys::Win32::Foundation::POINT { x: 0, y: 0 };
    if unsafe { GetCursorPos(&mut point) } == 0 {
        return Err("the machine did not say where the pointer is.".to_string());
    }
    Ok((point.x as i64, point.y as i64))
}

use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

/// The pointer of the machine is kept inside a rectangle.
///
/// The rectangle is the one of the screen: the left, the top, the right
/// and the bottom. A rectangle that is not there lets the pointer go
/// where it wants.
pub fn clip_cursor(rectangle: Option<(i64, i64, i64, i64)>) -> Result<(), String> {
    let mut area = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    match rectangle {
        Some((left, top, right, bottom)) => {
            area.left = left as i32;
            area.top = top as i32;
            area.right = right as i32;
            area.bottom = bottom as i32;
        }
        None => {
            // The rectangle of the screen is the one that lets the
            // pointer go where it wants.
            let (width, height) = super::screen::screen_size()?;
            area.right = width as i32;
            area.bottom = height as i32;
        }
    }
    if unsafe { ClipCursor(&area) } == 0 {
        return Err("the machine did not clip the pointer.".to_string());
    }
    Ok(())
}

/// The shape of the cursor of the machine.
///
/// The machine has a handle for the cursor of the screen, and the class
/// of the handle is the name of the shape: "Arrow", "IBeam", "Wait" and
/// the others -- the names of the v2.
pub fn the_shape_of_the_cursor() -> Result<String, String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CURSORINFO, GCLP_HCURSOR, GetClassLongW, GetCursorInfo,
    };
    unsafe {
        let mut info: CURSORINFO = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<CURSORINFO>() as u32;
        if GetCursorInfo(&mut info) == 0 {
            return Err("the machine did not say the shape of the cursor.".to_string());
        }
        // The class of the cursor is the name of the shape of it.
        let class = GetClassLongW(info.hCursor, GCLP_HCURSOR);
        if class == 0 {
            return Ok("Arrow".to_string());
        }
        let name = class as *const u16;
        let mut length = 0usize;
        while *name.add(length) != 0 {
            length += 1;
        }
        let text = String::from_utf16_lossy(std::slice::from_raw_parts(name, length));
        Ok(text)
    }
}
