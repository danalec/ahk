//! The windows of the machine.
//!
//! The windows of Windows are the objects of `user32`: every one of them
//! has a handle, a title and a class, and the machine enumerates them
//! with `EnumWindows`.
//!
//! The title of a window is what the v2 matches: the text that is wanted
//! is the start of it, a part of it, all of it, or a pattern of a regular
//! expression -- and the pattern is the one of the engine of the tree.

use ahk::desktop::criteria::WindowCriteria;
use ahk::desktop::{TitleMatch, WaitFor, WindowAnswer, WindowGeometry, WindowRef};

use windows_sys::Win32::Foundation::{HWND, LPARAM, RECT};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumChildWindows, EnumWindows, GetClassNameW, GetForegroundWindow, GetWindowRect,
    GetWindowTextLengthW,
    GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, IsZoomed,
    WS_CHILD,
};

use super::state::TheState;

/// A window of the machine, and what the machine says of it.
struct AWindow {
    handle: HWND,
    title: String,
    class: String,
    /// The process of the window, and the number of it.
    ///
    /// The process is read with the call of `user32` that every window
    /// of the machine has, and it is the number that the `ahk_pid`
    /// criterion of the machine compares.
    pid: u32,
    /// The name of the executable of the window, and the empty text when
    /// the machine does not say.
    ///
    /// The name is read only when a criterion asks for it: the name of
    /// the executable of a process is a call that opens the process, and
    /// a script that does not ask for it does not pay for it.
    exe: String,
}

/// The windows of the machine, and what the script sees of them.
///
/// The enumeration is the one of `user32`: the callback of the machine
/// walks the windows of the desktop, and the windows that the script
/// cannot see are left out.
///
/// The rule of what the script sees is the rule of the machine, measured
/// on the binaries of v1.1.37.02 and of v2.0.31: with
/// `DetectHiddenWindows` off the list is the visible windows that the
/// compositor has not cloaked -- a window of another virtual desktop is
/// visible and yet is not on the screen -- and with it on the list is
/// every window of the machine, the hidden ones and the cloaked ones
/// among them.
fn the_windows(state: &TheState) -> Vec<AWindow> {
    let mut found: Vec<AWindow> = Vec::new();
    unsafe {
        let _ = EnumWindows(Some(collect), &mut found as *mut Vec<AWindow> as LPARAM);
    }
    // The windows that the script made are looked at too.
    for handle in state.windows.values() {
        let handle = *handle as HWND;
        if found.iter().any(|window| window.handle == handle) {
            continue;
        }
        // The handle is one that the machine gave, and the calls that
        // read it are safe for it.
        if let Some(window) = unsafe { the_window_of(handle) } {
            found.push(window);
        }
    }
    found
}

/// Whether the window is one that the script sees.
///
/// The window of the compositor that has cloaked it is a window of
/// another virtual desktop: the machine says that it is visible, and the
/// user does not see it. The call that tells it apart is of Windows
/// Vista, and the machine that does not have it has no virtual desktops
/// and no cloaked windows.
fn the_window_is_seen(window: &AWindow) -> bool {
    if ahk::settings::detect_hidden_windows() {
        return true;
    }
    if unsafe { IsWindowVisible(window.handle) } == 0 {
        return false;
    }
    !the_window_is_cloaked(window.handle)
}

/// Whether the compositor has cloaked the window.
///
/// The call is `DwmGetWindowAttribute` of the `dwmapi` of Windows Vista,
/// and the tree resolves it at runtime: the machine of the floor of the
/// tree -- Windows XP SP3 -- has no `dwmapi` and no cloaked windows.
fn the_window_is_cloaked(handle: HWND) -> bool {
    // The attribute of the compositor that says whether a window is
    // cloaked.
    const DWMWA_CLOAKED: u32 = 14;
    // The call is resolved once and kept: the machine does not change.
    static CALL: std::sync::OnceLock<Option<unsafe extern "system" fn(HWND, u32, *mut u32) -> i32>> =
        std::sync::OnceLock::new();
    let Some(call) = *CALL.get_or_init(the_cloaking_call) else {
        return false;
    };
    let mut cloaked: u32 = 0;
    let answer = unsafe { call(handle, DWMWA_CLOAKED, &mut cloaked) };
    // The answer of the call is zero when it worked; a window that the
    // compositor does not know is not cloaked.
    answer == 0 && cloaked != 0
}

/// The call that tells whether a window is cloaked, if the machine has it.
///
/// The `dwmapi` is of Windows Vista, and the tree resolves the call at
/// runtime: a machine of the floor of the tree has no `dwmapi` at all,
/// and its windows are never cloaked.
fn the_cloaking_call(
) -> Option<unsafe extern "system" fn(HWND, u32, *mut u32) -> i32> {
    use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};

    // The library is loaded once: the handle of it is what the machine
    // gives, and the call is asked of it by its name.
    static LIBRARY: std::sync::OnceLock<Option<isize>> = std::sync::OnceLock::new();
    let handle = *LIBRARY.get_or_init(|| {
        let wide: Vec<u16> = "dwmapi.dll".encode_utf16().chain(Some(0)).collect();
        let handle = unsafe { LoadLibraryW(wide.as_ptr()) };
        if handle.is_null() {
            // The library of the compositor is not there: the machine is
            // of the floor of the tree, and it has no virtual desktop.
            None
        } else {
            Some(handle as isize)
        }
    });
    let handle = handle?;
    // The name of the call is the name of the machine.
    let mut name = Vec::new();
    name.extend_from_slice(b"DwmGetWindowAttribute\0");
    let address = unsafe { GetProcAddress(handle as *mut std::ffi::c_void, name.as_ptr()) };
    // The call is a pointer to a function that the machine does not
    // describe: what it returns is what the `DwmGetWindowAttribute`
    // gives back.
    address.map(|address| unsafe {
        std::mem::transmute::<unsafe extern "system" fn() -> isize, unsafe extern "system" fn(HWND, u32, *mut u32) -> i32>(address)
    })
}

/// The callback of the enumeration.
///
/// The pointer of the collection is the parameter of the call: the
/// machine gives it back to the callback, and the callback puts the
/// window in it.
unsafe extern "system" fn collect(handle: HWND, parameter: LPARAM) -> i32 {
    if handle.is_null() {
        return 0;
    }
    // The window is read with the calls of the machine, and the calls
    // are safe for a handle that the machine gave.
    let Some(window) = (unsafe { the_window_of(handle) }) else {
        return 1;
    };
    // A window without a title is a window like any other: measured on
    // the binaries of v1.1.37.02 and of v2.0.31, a visible
    // window without a title counts in the enumeration, and the list of
    // the windows of the v2 brings it.
    let found = unsafe { &mut *(parameter as *mut Vec<AWindow>) };
    found.push(window);
    1
}

/// The window of a handle, with its title and its class.
unsafe fn the_window_of(handle: HWND) -> Option<AWindow> {
    let length = unsafe { GetWindowTextLengthW(handle) };
    let mut title = vec![0u16; (length as usize) + 1];
    let read = unsafe { GetWindowTextW(handle, title.as_mut_ptr(), title.len() as i32) };
    let title = if read <= 0 {
        String::new()
    } else {
        String::from_utf16_lossy(&title[..read as usize])
    };
    let mut class = vec![0u16; 256];
    let read = unsafe { GetClassNameW(handle, class.as_mut_ptr(), class.len() as i32) };
    let class = if read <= 0 {
        String::new()
    } else {
        String::from_utf16_lossy(&class[..read as usize])
    };
    let mut pid: u32 = 0;
    unsafe { GetWindowThreadProcessId(handle, &mut pid) };
    Some(AWindow {
        handle,
        title,
        class,
        pid,
        exe: String::new(),
    })
}

/// The texts of the text elements of the window.
///
/// The text of a window is the text of the controls that have text: the
/// `WinText` of the v1 and the `WinText` of the v2 match one of them,
/// and not the title of the window.
///
/// Measured on the binary of v1.1.37.02: the `WinGetTitle`
/// of a window with a control that has a text finds the window by the
/// text of the control, and the same text with a window that does not
/// have it finds no window at all.
///
/// The `DetectHiddenText` is on by default, and so the text of the hidden
/// controls counts.
///
/// # Safety
///
/// The pointer has to be the one of a window of the machine: the
/// function walks the children of it, and the machine cannot answer
/// about a pointer that is not the one of a window.
pub unsafe fn the_texts_of_the_window(handle: HWND) -> String {
    let mut texts: Vec<String> = Vec::new();

    unsafe extern "system" fn the_callback(child: HWND, parameter: LPARAM) -> i32 {
        if child.is_null() {
            return 0;
        }
        let length = unsafe { GetWindowTextLengthW(child) };
        if length <= 0 {
            return 1;
        }
        let mut text = vec![0u16; (length as usize) + 1];
        let read = unsafe { GetWindowTextW(child, text.as_mut_ptr(), text.len() as i32) };
        if read <= 0 {
            return 1;
        }
        let texts = unsafe { &mut *(parameter as *mut Vec<String>) };
        texts.push(String::from_utf16_lossy(&text[..read as usize]));
        1
    }

    unsafe {
        EnumChildWindows(
            handle,
            Some(the_callback),
            &mut texts as *mut Vec<String> as LPARAM,
        );
    }
    texts.join("\n")
}


/// The class name and the number of every control of a window.
///
/// The v1 numbers the controls of each class from one: the first button
/// is `Button1` and the second is `Button2`. The list is separated by a
/// line break, and the `handles` asks for the identifiers of the controls
/// instead of their names.
///
/// Measured on the binary of v1.1.37.02: the `WinGet, out, ControlList,
/// <title>` of a window with two buttons and an edit answers `Button1`,
/// `Button2` and `Edit1`, one per line.
///
/// # Safety
///
/// The handle has to be the one of a window of the machine: the function
/// walks the children of it, and the machine cannot answer about a handle
/// that is not the one of a window.
pub unsafe fn the_controls_of_the_window(handle: HWND, handles: bool) -> Option<String> {
    struct TheList {
        /// The classes that were seen, and the number of each one.
        classes: Vec<(String, u32)>,
        /// The answer: one item per control.
        items: Vec<String>,
        /// Whether the answer is the identifiers of the controls.
        handles: bool,
    }

    unsafe extern "system" fn the_callback(child: HWND, parameter: LPARAM) -> i32 {
        if child.is_null() {
            return 0;
        }
        let list = unsafe { &mut *(parameter as *mut TheList) };
        if list.handles {
            // The identifier of the control, in the form of the v1: the
            // "0x" and the number in the hexadecimal.
            list.items.push(format!("0x{:x}", child as usize));
            return 1;
        }
        let mut buffer = [0u16; 256];
        let read = unsafe { GetClassNameW(child, buffer.as_mut_ptr(), buffer.len() as i32) };
        if read <= 0 {
            return 1;
        }
        let class = String::from_utf16_lossy(&buffer[..read as usize]);
        // The number of the control is the number of the controls of that
        // class that were seen before it.
        let number = match list
            .classes
            .iter_mut()
            .find(|(name, _)| name.eq_ignore_ascii_case(&class))
        {
            Some(entry) => {
                entry.1 += 1;
                entry.1
            }
            None => {
                list.classes.push((class.clone(), 1));
                1
            }
        };
        list.items.push(format!("{class}{number}"));
        1
    }

    let mut list = TheList {
        classes: Vec::new(),
        items: Vec::new(),
        handles,
    };
    unsafe {
        EnumChildWindows(
            handle,
            Some(the_callback),
            &mut list as *mut TheList as LPARAM,
        );
    }
    if list.items.is_empty() {
        return None;
    }
    Some(list.items.join("\n"))
}

/// The window whose title matches the text that is wanted.
///
/// The windows that the script does not see -- the hidden ones and the
/// ones that the compositor has cloaked -- are left out, unless the
/// setting of the hidden windows is on. Measured on the binary of
/// v1.1.37.02: the `WinGetTitle` and the `WinExist` of a
/// hidden window answer nothing when the setting is off, and find it
/// when the setting is on.
fn the_window_of_the_title(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<Option<AWindow>, String> {
    let criteria = WindowCriteria::parse(title);
    // The criterion `ahk_id` names the window by the handle of it, and the
    // handle may be of a window that is a child of another one: the v1
    // (source `window.cpp:586-606` of v1.1.37.02) takes the handle as it
    // is, and a child is taken too (`window.cpp:626`, "Child windows (via
    // ahk_id) are always detectible"). The enumeration has only the windows
    // of the top, so the window of the handle is asked for directly.
    //
    // Measured on the binary of v1.1.37.02: the
    // `WinGetClass, ahk_id %hButton%` of a button of a GUI of the script
    // answers "Button".
    if let Some(wanted) = &criteria.id {
        let Some(number) = ahk::desktop::criteria::the_number_of(wanted) else {
            // A criterion whose value is not a number names no window.
            return Ok(None);
        };
        let handle = number as HWND;
        if handle.is_null() || unsafe { IsWindow(handle) } == 0 {
            return Ok(None);
        }
        let Some(window) = (unsafe { the_window_of(handle) }) else {
            return Ok(None);
        };
        // A window that is a child of another one is taken even when it is
        // hidden, and a window of the top is taken only when the script sees
        // it: the v1 (`window.cpp:594-599`).
        let is_child = unsafe { super::styles::the_style_of(handle, false) }
            .map(|style| style & WS_CHILD as i64 != 0)
            .unwrap_or(false);
        if !is_child && !the_window_is_seen(&window) {
            return Ok(None);
        }
        if !the_window_answers(&window, &criteria, mode)? {
            return Ok(None);
        }
        return Ok(Some(window));
    }
    for window in the_windows(state) {
        if !the_window_is_seen(&window) {
            continue;
        }
        if the_window_answers(&window, &criteria, mode)? {
            return Ok(Some(window));
        }
    }
    Ok(None)
}

/// Whether the window answers the criteria.
///
/// The criteria are the parts of the text that names the window: the
/// title, the class, the executable, the identifier and the process.
/// Measured on the binaries of v1.1.37.02 and of v2.0.31 `ahk_class Notepad` names the window whose class is
/// `Notepad`, and the case of the class matters.
fn the_window_answers(
    window: &AWindow,
    criteria: &WindowCriteria,
    mode: TitleMatch,
) -> Result<bool, String> {
    let exe = if criteria.exe.is_some() {
        the_exe_of(window)
    } else {
        String::new()
    };
    criteria.matches(
        &window.title,
        &window.class,
        &exe,
        window.handle as i64,
        window.pid as i64,
        is_active(window),
        mode,
    )
}

/// The name of the executable of the window.
///
/// The name is the name of the file, without the folder of it, and it
/// is the name that the `ahk_exe` criterion of the machine compares.
/// Measured on the binaries of v1.1.37.02 and of v2.0.31 `ahk_exe python.exe` names the window whose executable
/// is the `python.exe`, and `ahk_exe python` names none.
fn the_exe_of(window: &AWindow) -> String {
    if !window.exe.is_empty() {
        return window.exe.clone();
    }
    // The path of the executable of the process: the name is the one of
    // the file, without the folder: `python.exe`, and not the whole
    // path.
    let path = match super::processes::the_path_of_the_process(window.pid) {
        Some(path) => path,
        None => return String::new(),
    };
    match path.rsplit(['\\', '/']).next() {
        Some(part) if !part.is_empty() => part.to_string(),
        _ => String::new(),
    }
}

/// The name of the executable of a window, by its identifier.
///
/// The name is the one of the file, without the folder: `python.exe`,
/// and not the whole path. The name is read only when the `ahk_exe`
/// criterion asks for it.
pub fn the_exe_of_the_window(id: i64) -> String {
    let Some(window) = the_window_of_the_identifier(id) else {
        return String::new();
    };
    the_exe_of(&window)
}

/// The window of the identifier, if the machine still has it.
fn the_window_of_the_identifier(id: i64) -> Option<AWindow> {
    let handle = id as HWND;
    if handle.is_null() {
        return None;
    }
    unsafe { the_window_of(handle) }
}

/// Whether the window of the machine is one that the script sees.
///
/// The window follows the same rule as the enumeration: with the
/// detection of the hidden windows off, the window that the script sees
/// is the window that the compositor has not hidden.
///
/// The function is `unsafe` because the pointer is a `windows` handle of
/// the machine, and the machine is the only one that knows whether it is
/// the one of a window.
///
/// # Safety
///
/// The pointer has to be the one of a window of the machine: the
/// function asks the machine about it, and the machine cannot answer
/// about a pointer that is not the one of a window.
pub unsafe fn the_window_is_seen_of_the_machine(handle: HWND) -> bool {
    if ahk::settings::detect_hidden_windows() {
        return true;
    }
    if unsafe { IsWindowVisible(handle) } == 0 {
        return false;
    }
    !the_window_is_cloaked(handle)
}

/// Whether the window has the focus of the machine.
fn is_active(window: &AWindow) -> bool {
    unsafe { GetForegroundWindow() == window.handle }
}

/// Whether the window of the machine has the focus of the machine.
pub fn the_window_is_active(handle: HWND) -> bool {
    unsafe { GetForegroundWindow() == handle }
}

/// Whether the window is there and has the focus.
///
/// The answer of the v2 has three words: the window is there and is the
/// active one, the window is there and is not, and the window is not
/// there.
pub fn window_active(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<WindowAnswer, String> {
    match the_window_of_the_title(state, title, mode)? {
        Some(window) => Ok(if is_active(&window) {
            WindowAnswer::Active
        } else {
            WindowAnswer::NotActive
        }),
        None => Ok(WindowAnswer::Absent),
    }
}

/// Whether the window is there.
pub fn window_exists(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<WindowAnswer, String> {
    match the_window_of_the_title(state, title, mode)? {
        Some(_) => Ok(WindowAnswer::NotActive),
        None => Ok(WindowAnswer::Absent),
    }
}

/// The title of the window that has the focus.
pub fn active_window_title(_state: &TheState) -> Result<Option<String>, String> {
    let handle = unsafe { GetForegroundWindow() };
    if handle.is_null() {
        return Ok(None);
    }
    // The handle is the one of the window that has the focus, and the
    // calls that read it are safe for it.
    Ok(unsafe { the_window_of(handle) }.map(|window| window.title))
}

/// The windows whose title matches the text that is wanted.
///
/// The windows that the script does not see -- the hidden ones and the
/// ones that the compositor has cloaked -- are left out, unless the
/// setting of the hidden windows is on.
pub fn matching_windows(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<Vec<WindowRef>, String> {
    let mut found = Vec::new();
    let criteria = WindowCriteria::parse(title);
    for window in the_windows(state) {
        if !the_window_is_seen(&window) {
            continue;
        }
        if the_window_answers(&window, &criteria, mode)? {
            found.push(WindowRef {
                id: (window.handle as usize).to_string(),
                class: window.class,
            });
        }
    }
    Ok(found)
}

/// Where the window is and how big it is.
pub fn window_geometry(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<Option<WindowGeometry>, String> {
    let Some(window) = the_window_of_the_title(state, title, mode)? else {
        return Ok(None);
    };
    let mut rectangle: RECT = unsafe { std::mem::zeroed() };
    if unsafe { GetWindowRect(window.handle, &mut rectangle) } == 0 {
        return Ok(None);
    }
    Ok(Some(WindowGeometry {
        x: rectangle.left as i64,
        y: rectangle.top as i64,
        width: (rectangle.right - rectangle.left) as i64,
        height: (rectangle.bottom - rectangle.top) as i64,
    }))
}

/// Whether the window is minimized or maximized.
///
/// The v2 answers `-1` for a window that is minimized, `1` for a window
/// that is maximized, and `0` for a window that is neither.
pub fn window_min_max(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<Option<i64>, String> {
    let Some(window) = the_window_of_the_title(state, title, mode)? else {
        return Ok(None);
    };
    unsafe {
        if IsIconic(window.handle) != 0 {
            return Ok(Some(-1));
        }
        if IsZoomed(window.handle) != 0 {
            return Ok(Some(1));
        }
    }
    Ok(Some(0))
}

/// The process that owns the window.
pub fn window_pid(state: &TheState, title: &str) -> Result<Option<i64>, String> {
    let Some(window) = the_window_of_the_title(state, title, TitleMatch::Exact)? else {
        return Ok(None);
    };
    let mut process = 0u32;
    unsafe {
        GetWindowThreadProcessId(window.handle, &mut process);
    }
    Ok(Some(process as i64))
}

/// Waits for a window to be there, to have the focus, or to be gone.
///
/// The wait is the wait of the machine: the loop asks the machine until
/// the answer is the one that is wanted, or until the time is up.
///
/// The exclusions are the titles of the windows that do not answer a
/// wait. Measured on the binary of v1.1.37.02: a window
/// whose title is the title of the exclusion does not answer the wait,
/// and the wait ends with the time at its end.
pub fn window_wait(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
    what: WaitFor,
    timeout_ms: u64,
    excludes: &[String],
) -> Result<Option<String>, String> {
    let waited = std::time::Instant::now();
    loop {
        let answer = the_answer_of_the_wait(state, title, mode, what, excludes)?;
        if answer.is_some() {
            return Ok(answer);
        }
        if waited.elapsed().as_millis() as u64 >= timeout_ms {
            return Ok(None);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// The answer of a wait: the identifier of the window that answers it, or
/// none when the condition does not hold yet.
///
/// The window that answers is the first one that matches the title and
/// that does not match any of the exclusions. The text of the window is
/// not read: the exclusions by text need the text of the controls, and
/// the desktop of the box does not have it.
///
/// The answer of the conditions that talk about the presence of a window
/// is the identifier of that window, and the answer of the ones that talk
/// about the absence is the empty text: there is no window to identify.
/// Measured on the binary of v2.0.32: the `WinWait("Calculator")` answers
/// the identifier of the window, and the `WinWaitClose("Calculator")`
/// answers `1`.
///
/// The title that is empty is no window at all: the runtime resolves the
/// window that the script found last before it asks the desktop, and an
/// empty title means that that window has no title.
/// Measured on the binaries of v1.1.37.02 and of v2.0.32: the
/// `WinWaitClose` with no title answers at once -- the v1 leaves the
/// `ErrorLevel` at zero and the v2 answers `1` -- and the `WinWait` with
/// no title waits for a window that never comes.
///
/// The title that is empty with an exclusion is not the same thing: the
/// title that is empty is the title of any window, and the exclusion
/// takes that window out of the answer.
fn the_answer_of_the_wait(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
    what: WaitFor,
    excludes: &[String],
) -> Result<Option<String>, String> {
    if title.is_empty() && excludes.is_empty() {
        // The title that is empty is no window at all: the conditions that
        // talk about the absence of a window hold at once, and the ones
        // that talk about its presence do not hold at all.
        return Ok(match what {
            WaitFor::NotActive | WaitFor::Close => Some(String::new()),
            WaitFor::Present | WaitFor::Active => None,
        });
    }
    let criteria = WindowCriteria::parse(title);
    let mut found: Option<AWindow> = None;
    for window in the_windows(state) {
        if !the_window_is_seen(&window) {
            continue;
        }
        if !the_window_answers(&window, &criteria, mode)? {
            continue;
        }
        // The window that is excluded does not answer the wait.
        if the_window_is_excluded(&window, excludes, mode) {
            continue;
        }
        found = Some(window);
        break;
    }
    let Some(window) = found else {
        // No window answers the wait: the conditions that talk about the
        // absence of a window hold at once, and the ones that talk about
        // its presence do not hold at all.
        return Ok(match what {
            WaitFor::NotActive | WaitFor::Close => Some(String::new()),
            WaitFor::Present | WaitFor::Active => None,
        });
    };
    let is_active = is_active(&window);
    Ok(match what {
        WaitFor::Present => Some(the_info_of_the_window(&window).id.to_string()),
        WaitFor::Active => {
            if is_active {
                Some(the_info_of_the_window(&window).id.to_string())
            } else {
                None
            }
        }
        WaitFor::NotActive => {
            if is_active {
                None
            } else {
                Some(String::new())
            }
        }
        // The window is there: it is not gone yet.
        WaitFor::Close => None,
    })
}

/// Whether the window is one of the exclusions.
///
/// The exclusion is a title, and not a set of criteria: the CHM of the
/// v1 says that the exclusion by the title recognises no criterion that
/// is not the title itself.
fn the_window_is_excluded(window: &AWindow, excludes: &[String], mode: TitleMatch) -> bool {
    excludes.iter().any(|excluded| {
        if excluded.is_empty() {
            return false;
        }
        mode.matches(&window.title, excluded).unwrap_or(false)
    })
}

/// The handle of the window whose title matches the text that is wanted.
///
/// The mode of the match is the one of the v2: the title of a window
/// that a control is looked for in starts with the text.
///
/// A number is the `HWND` of the window, and not its title: the v2
/// (source `lib/win.cpp:2197-2250` of v2.0.31) resolves the target of
/// a window that way -- a pure number is the `HWND`, and the title is
/// only looked for when the target is not a number.
pub fn the_handle_of_the_title(
    state: &TheState,
    title: &str,
    mode: TitleMatch,
) -> Result<Option<HWND>, String> {
    match the_handle_of_a_number(title) {
        Some(window) => Ok(window),
        None => Ok(the_window_of_the_title(state, title, mode)?.map(|window| window.handle)),
    }
}

/// The `HWND` of an alvo that is a number.
///
/// The v2 (source `lib/win.cpp:2197-2250` of v2.0.31) resolves the
/// target of a window that way: a pure number is the `HWND` of the
/// window, and the title is only looked for when the target is not a
/// number.
///
/// The `None` is the target that is not a number -- the title is
/// looked for --, and the `Some(None)` is the number that is not the
/// `HWND` of any window: the v2 does not look for a title with that
/// number, it says that the window is not there.
fn the_handle_of_a_number(title: &str) -> Option<Option<HWND>> {
    let number = title.trim();
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let Ok(number) = number.parse::<u64>() else {
        return None;
    };
    let handle = number as HWND;
    if unsafe { IsWindow(handle) } == 0 {
        return Some(None);
    }
    Some(Some(handle))
}

/// The window of the machine that has no picture on the screen.
///
/// The tray of the machine and the hotkeys of the script need a window
/// of the session to belong to: a window that only receives messages is
/// enough, and the user does not see it.
pub fn a_message_window(class: &str) -> Result<HWND, String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::DefWindowProcW;
    a_window_of_the_machine(class, DefWindowProcW)
}

/// The window of the machine, with the procedure of it.
///
/// The window is the one that the messages of the session arrive in: the
/// tray of the machine and the hotkeys of the script are the ones that
/// own it.
pub fn a_window_of_the_machine(
    class: &str,
    procedure: unsafe extern "system" fn(
        HWND,
        u32,
        windows_sys::Win32::Foundation::WPARAM,
        windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT,
) -> Result<HWND, String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::WS_OVERLAPPED;
    a_window_with_the_style_of(class, procedure, WS_OVERLAPPED, 0, "ahk")
}

/// The window of the script: the hidden window that carries the messages
/// of the session.
///
/// The v2 makes this window at the beginning of the script, and its
/// number is the `A_ScriptHwnd`. The window here is the same thing: the
/// class is registered once, the window is made with the
/// `WS_OVERLAPPEDWINDOW` that the v2 asks for, and the user does not
/// see it.
///
/// Measured on the binary of v2.0.31: the style of the
/// window of the script is `80674816` and the extended style is `256`.
/// The two numbers are the ones the machine gives on its own: the
/// `WS_CLIPSIBLINGS` in the style and the `WS_EX_WINDOWEDGE` in the
/// extended style. This is why the window is made **without** any
/// extended style -- the v2 makes the window of the script with the
/// `WS_EX_TOOLWINDOW` only when the window in front of it is the task
/// bar, and the measurement was made without it.
///
/// The title of the window is the empty text on purpose: the window of
/// the script is not a window that the script names, and the list of the
/// windows of the machine does not show it.
pub fn a_window_of_the_script(
    class: &str,
    procedure: unsafe extern "system" fn(
        HWND,
        u32,
        windows_sys::Win32::Foundation::WPARAM,
        windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT,
) -> Result<HWND, String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW;
    a_window_with_the_style_of(class, procedure, WS_OVERLAPPEDWINDOW, 0, "")
}

/// The window of the machine, with the style that is asked for.
///
/// The class is registered once: the second registration of the same
/// class is refused with the number `1410`, and the window is made with
/// the class that is already there.
fn a_window_with_the_style_of(
    class: &str,
    procedure: unsafe extern "system" fn(
        HWND,
        u32,
        windows_sys::Win32::Foundation::WPARAM,
        windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT,
    style: u32,
    exstyle: u32,
    title: &str,
) -> Result<HWND, String> {
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, RegisterClassExW, WNDCLASSEXW,
    };
    unsafe {
        let module = GetModuleHandleW(std::ptr::null_mut());
        let class = super::sys::Wide::of(class);
        let window_class = WNDCLASSEXW {
            lpfnWndProc: Some(procedure),
            hInstance: module,
            lpszClassName: class.as_ptr(),
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            ..std::mem::zeroed()
        };
        if RegisterClassExW(&window_class) == 0 {
            // The class of the window is taken once: the second window
            // of the same class is refused with the number of the class
            // that is already there, and the window is made with it.
            let code = windows_sys::Win32::Foundation::GetLastError();
            if code != 1410 {
                return Err("the machine did not take the class of the window.".to_string());
            }
        }
        let title = super::sys::Wide::of(title);
        let window = CreateWindowExW(
            exstyle,
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
        if window.is_null() {
            let code = windows_sys::Win32::Foundation::GetLastError();
            return Err(format!("the machine did not make the window: {code}."));
        }
        Ok(window)
    }
}

/// The window that has the focus is moved.
///
/// The window of the focus is the one that the machine says: the place
/// and the size of it are the ones of the machine.
pub fn the_window_of_the_focus_is_moved(
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, SWP_NOSIZE, SWP_NOZORDER, SetWindowPos,
    };
    unsafe {
        let window = GetForegroundWindow();
        if window.is_null() {
            return Err("the machine has no window with the focus.".to_string());
        }
        if SetWindowPos(
            window,
            std::ptr::null_mut(),
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            SWP_NOZORDER | SWP_NOSIZE,
        ) == 0
        {
            return Err("the machine did not move the window.".to_string());
        }
    }
    Ok(())
}

/// The window that is at a place of the screen.
///
/// The answer is the number of the window, the way the tree numbers the
/// windows of the machine.
pub fn the_window_at(x: i64, y: i64) -> Result<Option<i64>, String> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::WindowFromPoint;
    unsafe {
        let point = POINT {
            x: x as i32,
            y: y as i32,
        };
        let window = WindowFromPoint(point);
        if window.is_null() {
            return Ok(None);
        }
        Ok(Some(window as i64))
    }
}

/// The window of the top level that is under the pointer, and the
/// control that is under it.
///
/// The v1: the `MouseGetPos` of the v1 answers the window of the top
/// level that is under the pointer, and the control that is under it.
///
/// Measured on the binary of v1.1.37.02: the search of the control is
/// the one of the Window Spy -- the visible children of the window that
/// hold the point, and the one whose centre is the closest to the point
/// wins when the children of the window overlap.  The mode with the bit
/// 1 asks for the window of the machine alone, without the search.
pub fn the_window_and_the_control_under_the_pointer(
    x: i64,
    y: i64,
    simple: bool,
) -> (i64, i64) {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowExW, GA_ROOT, GetAncestor, WindowFromPoint,
    };
    let point = POINT {
        x: x as i32,
        y: y as i32,
    };
    unsafe {
        let under = WindowFromPoint(point);
        if under.is_null() {
            return (0, 0);
        }
        // The window of the top level that holds the control: the first
        // ancestor of it that is not a child of another window.
        let root = GetAncestor(under, GA_ROOT);
        let parent = if root.is_null() { under } else { root };
        if simple {
            return (parent as i64, under as i64);
        }
        // The control of the window that is under the pointer: the
        // visible children of the window that hold the point.
        let mut found: HWND = std::ptr::null_mut();
        let mut found_rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        let mut found_distance = 0.0f64;
        let mut child = FindWindowExW(
            parent,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
        );
        while !child.is_null() {
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            if IsWindowVisible(child) != 0
                && GetWindowRect(child, &mut rect) != 0
                && point.x >= rect.left
                && point.x < rect.right
                && point.y >= rect.top
                && point.y < rect.bottom
            {
                let center_x = rect.left as f64 + (rect.right - rect.left) as f64 / 2.0;
                let center_y = rect.top as f64 + (rect.bottom - rect.top) as f64 / 2.0;
                let distance = ((point.x as f64 - center_x).powi(2) + (point.y as f64 - center_y).powi(2))
                    .sqrt();
                let mut update = found.is_null();
                if !update {
                    // The new window is entirely inside the old one: it
                    // is the better answer.  The old one is entirely
                    // inside the new one: it stays.  Otherwise the one
                    // whose centre is the closest to the point wins.
                    let enclosed = rect.left >= found_rect.left
                        && rect.right <= found_rect.right
                        && rect.top >= found_rect.top
                        && rect.bottom <= found_rect.bottom;
                    let outside = found_rect.left < rect.left
                        || found_rect.right > rect.right
                        || found_rect.top < rect.top
                        || found_rect.bottom > rect.bottom;
                    if enclosed || (distance < found_distance && outside) {
                        update = true;
                    }
                }
                if update {
                    found = child;
                    found_rect = rect;
                    found_distance = distance;
                }
            }
            child = FindWindowExW(parent, child, std::ptr::null(), std::ptr::null());
        }
        // Without a control of its own, the window of the top level is
        // the control that is under the pointer.
        let control = if found.is_null() { under } else { found };
        (parent as i64, control as i64)
    }
}

/// Whether a window is enabled.
///
/// The answer is `None` when the machine has no such window.
pub fn the_window_is_enabled(window: i64) -> Result<Option<bool>, String> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::IsWindowEnabled;
    use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Ok(None);
        }
        Ok(Some(IsWindowEnabled(handle) != 0))
    }
}

/// Whether a window is always on top.
pub fn the_window_is_always_on_top(window: i64) -> Result<Option<bool>, String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, IsWindow, WS_EX_TOPMOST};
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Ok(None);
        }
        // The style of the window is a number of the machine, and the
        // machine of thirty-two bits keeps it in thirty-two bits.
        let style = GetWindowLongPtrW(handle, -20) as i64;
        Ok(Some(style & WS_EX_TOPMOST as i64 != 0))
    }
}

/// A window is always on top, or it is not.
pub fn the_window_is_made_always_on_top(window: i64, on: bool) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos,
    };
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if SetWindowPos(
            handle,
            if on { HWND_TOPMOST } else { HWND_NOTOPMOST },
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        ) == 0
        {
            return Err("the machine did not make the window always on top.".to_string());
        }
    }
    Ok(())
}

/// A window of the machine is enabled, or it is disabled.
///
/// The `EnableWindow` of the machine is the one that the v1 uses: the
/// `WinSet, Enable` and the `WinSet, Disable` of a window are the same
/// call.
pub fn the_window_is_made_enabled(window: i64, on: bool) -> Result<(), String> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
    use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Err("the machine has no such window.".to_string());
        }
        EnableWindow(handle, if on { 1 } else { 0 });
    }
    Ok(())
}

/// A window of the machine is moved, and its size is changed.
///
/// The place and the size are the ones of the machine: the pixels.
pub fn the_window_is_moved(
    window: i64,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{IsWindow, MoveWindow};
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Err("the machine has no such window.".to_string());
        }
        MoveWindow(
            handle,
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            1,
        );
    }
    Ok(())
}

/// A window of the machine is brought to the front, and it gets the focus.
///
/// The `SetForegroundWindow` of the machine is the one that the v1 uses
/// inside its `SetForegroundWindowEx` (`window.cpp`), which tries the
/// window above before the one that was asked for.
pub fn the_window_is_activated(window: i64) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{IsWindow, SetForegroundWindow};
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Err("the machine has no such window.".to_string());
        }
        SetForegroundWindow(handle);
    }
    Ok(())
}

/// A window of the machine is given another title.
///
/// The v1: the `WinSetTitle` of the binary of v1.1.37.02 renames the
/// window that its criteria name, and it says nothing when the window is
/// not there.
pub fn the_title_of_the_window_is_written(window: i64, title: &str) -> Result<(), String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{IsWindow, SetWindowTextW};
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Err("the machine has no such window.".to_string());
        }
        let text = super::sys::Wide::of(title);
        SetWindowTextW(handle, text.as_ptr());
    }
    Ok(())
}

/// The region of a window of the machine is the shape that it takes.
///
/// The shape is one of the shapes of the machine: the rectangle, the
/// rectangle with rounded corners, the ellipse and the polygon. The
/// region of the `None` is the region of the window itself.
pub fn the_region_is_given(
    window: i64,
    region: &ahk::desktop::WindowRegion,
) -> Result<(), String> {
    use windows_sys::Win32::Graphics::Gdi::{
        ALTERNATE, CreateEllipticRgn, CreatePolygonRgn, CreateRectRgn, CreateRoundRectRgn, DeleteObject,
        SetWindowRgn, WINDING,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;
    unsafe {
        let handle = window as windows_sys::Win32::Foundation::HWND;
        if IsWindow(handle) == 0 {
            return Err("the machine has no such window.".to_string());
        }
        let region_handle = match region.kind {
            ahk::desktop::WindowRegionKind::None => {
                // The machine takes the region of the window away when the
                // region of it is nothing.
                SetWindowRgn(handle, std::ptr::null_mut(), 1);
                return Ok(());
            }
            ahk::desktop::WindowRegionKind::Rectangle => CreateRectRgn(
                region.x as i32,
                region.y as i32,
                (region.x + region.width) as i32,
                (region.y + region.height) as i32,
            ),
            ahk::desktop::WindowRegionKind::RoundedRectangle => CreateRoundRectRgn(
                region.x as i32,
                region.y as i32,
                (region.x + region.width) as i32,
                (region.y + region.height) as i32,
                region.radius_x as i32,
                region.radius_y as i32,
            ),
            ahk::desktop::WindowRegionKind::Ellipse => CreateEllipticRgn(
                region.x as i32,
                region.y as i32,
                (region.x + region.width) as i32,
                (region.y + region.height) as i32,
            ),
            ahk::desktop::WindowRegionKind::Polygon => {
                let points: Vec<windows_sys::Win32::Foundation::POINT> = region
                    .points
                    .iter()
                    .map(|(x, y)| windows_sys::Win32::Foundation::POINT {
                        x: *x as i32,
                        y: *y as i32,
                    })
                    .collect();
                CreatePolygonRgn(
                    points.as_ptr(),
                    points.len() as i32,
                    if region.winding { WINDING } else { ALTERNATE },
                )
            }
        };
        if region_handle.is_null() {
            return Err("the machine did not make the region.".to_string());
        }
        if SetWindowRgn(handle, region_handle, 1) == 0 {
            DeleteObject(region_handle);
            return Err("the machine did not give the region to the window.".to_string());
        }
    }
    Ok(())
}

/// The information of a window, in the shape of the desktop.
///
/// The name of the executable is not read here: the list reads it only
/// when a criterion asks for it.
fn the_info_of_the_window(window: &AWindow) -> ahk::desktop::WindowInfo {
    ahk::desktop::WindowInfo {
        id: window.handle as i64,
        title: window.title.clone(),
        class: window.class.clone(),
        active: is_active(window),
        exe: String::new(),
        pid: window.pid as i64,
        text: unsafe { the_texts_of_the_window(window.handle) },
    }
}

/// The window of a handle, and `None` when the handle is not a window.
///
/// The window of the handle may be a child of another window: the v1 takes
/// the handle of a child in the criterion `ahk_id` (source `window.cpp:626`
/// of v1.1.37.02, "Child windows (via ahk_id) are always detectible"), and
/// the listing of the windows has only the windows of the top.
pub fn the_window_of_the_handle(id: i64) -> Option<ahk::desktop::WindowInfo> {
    let handle = id as HWND;
    if handle.is_null() || unsafe { IsWindow(handle) } == 0 {
        return None;
    }
    let found = unsafe { the_window_of(handle) }?;
    Some(ahk::desktop::WindowInfo {
        id,
        title: found.title,
        class: found.class,
        active: the_window_is_active(handle),
        exe: String::new(),
        pid: found.pid as i64,
        text: unsafe { the_texts_of_the_window(handle) },
    })
}

/// The list of the windows of the machine.
///
/// Each window is what the machine says of it: the number of it, the
/// title, the class, the process and the name of the executable. The
/// windows of the machine are the ones that the machine enumerates.
///
/// The name of the executable is read only when a criterion asks for
/// it: the name is a call that opens the process of each window, and a
/// list that nobody asks for does not pay for it.
pub fn the_windows_of_the_machine() -> Result<Vec<ahk::desktop::WindowInfo>, String> {
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::EnumWindows;
    use windows_sys::core::BOOL;

    unsafe extern "system" fn the_callback(window: HWND, list: LPARAM) -> BOOL {
        unsafe {
            let list = &mut *(list as *mut Vec<ahk::desktop::WindowInfo>);
            let Some(found) = the_window_of(window) else {
                return 1;
            };
            list.push(ahk::desktop::WindowInfo {
                id: window as i64,
                title: found.title,
                class: found.class,
                active: false,
                exe: String::new(),
                pid: found.pid as i64,
                text: the_texts_of_the_window(window),
            });
        }
        1
    }

    let mut list: Vec<ahk::desktop::WindowInfo> = Vec::new();
    unsafe {
        EnumWindows(
            Some(the_callback),
            &mut list as *mut Vec<ahk::desktop::WindowInfo> as LPARAM,
        );
    }
    Ok(list)
}
