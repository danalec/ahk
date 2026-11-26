//! The dialogs of the machine.
//!
//! The `InputBox` of the v2 is a window of the machine with a text in
//! it: the window is made with the classes of `user32`, the loop of the
//! messages of the session runs until the user answers, and the answer
//! is the text of the field.
//!
//! The pickers of the files are the shell of the machine (`IFileDialog`),
//! and they are the COM of the machine: the COM of the machine is
//! carried in `com.rs`, and the pickers are made with it here.

use std::cell::RefCell;

use ahk::desktop::{FileAnswer, FileOptions, InputAnswer, InputOptions};

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
    GetWindowTextLengthW, GetWindowTextW, KillTimer, MessageBoxW, PostMessageW, RegisterClassExW,
    SendMessageW, SetTimer, ShowWindow, TranslateMessage, WM_APP, WM_CLOSE, WM_COMMAND, WM_SETTEXT,
    WM_TIMER, WNDCLASSEXW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

use super::com::ThePicker;
use super::sys::Wide;

// The number of the message that says that the user answered.
const WM_ANSWERED: u32 = WM_APP + 1;

// The answer of the user, and the text that was typed.
//
// The answer belongs to the thread that asked: the box is a window of
// the thread of the script, and the procedure of it runs there too.
thread_local! {
    static THE_ANSWER: RefCell<Option<(String, String)>> = const { RefCell::new(None) };
    /// The field of the text of the box.
    static THE_FIELD: RefCell<Option<HWND>> = const { RefCell::new(None) };
}

/// The procedure of the window of the box.
///
/// The buttons answer with the message of the tree, and the text of the
/// field comes with it.
unsafe extern "system" fn the_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    _lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_COMMAND => {
            // The low word of the parameter is the control that was
            // pressed: one is the button of the yes, and two the one of
            // the no.
            let control = (wparam & 0xFFFF) as u16;
            let answer = if control == 1 { "OK" } else { "Cancel" };
            let text = THE_FIELD.with(|field| match *field.borrow() {
                Some(field) => the_text_of_the_field(field),
                None => String::new(),
            });
            THE_ANSWER.with(|answer_of| {
                *answer_of.borrow_mut() = Some((text, answer.to_string()));
            });
            let _ = unsafe { PostMessageW(window, WM_ANSWERED, 0, 0) };
            0
        }
        WM_ANSWERED => {
            let _ = unsafe { DestroyWindow(window) };
            0
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, _lparam) },
    }
}

/// The text of the field of the box.
fn the_text_of_the_field(field: HWND) -> String {
    let length = unsafe { GetWindowTextLengthW(field) };
    let mut buffer = vec![0u16; length as usize + 1];
    let read = unsafe { GetWindowTextW(field, buffer.as_mut_ptr(), buffer.len() as i32) };
    if read <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buffer[..read as usize])
}

/// The text is written in the field of the box.
fn the_field_is_written(field: HWND, text: &str) {
    let wide = Wide::of(text);
    unsafe {
        SendMessageW(field, WM_SETTEXT, 0, wide.as_ptr() as LPARAM);
    }
}

/// The user is asked for a text.
///
/// The box is a window of the machine with a field in it, and the loop
/// of the messages of the session runs until the user answers it or
/// until the time of it is up.
pub fn input_box(
    prompt: &str,
    title: &str,
    options: &InputOptions,
    default: &str,
) -> Result<InputAnswer, String> {
    unsafe {
        // The answer of the last box is forgotten: the answer of this
        // one is the one that the procedure writes.
        THE_ANSWER.with(|answer| *answer.borrow_mut() = None);
        THE_FIELD.with(|field| *field.borrow_mut() = None);
        let module = GetModuleHandleW(std::ptr::null());
        let class = Wide::of("ahk-rs-input-box");
        let window_class = WNDCLASSEXW {
            lpfnWndProc: Some(the_procedure),
            hInstance: module,
            lpszClassName: class.as_ptr(),
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            ..std::mem::zeroed()
        };
        if RegisterClassExW(&window_class) == 0 {
            return Err("the machine did not take the class of the box.".to_string());
        }
        let width = options.width.unwrap_or(420);
        let height = options.height.unwrap_or(160);
        let title_wide = Wide::of(title);
        let window = CreateWindowExW(
            0,
            class.as_ptr(),
            title_wide.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            options.x.unwrap_or(100) as i32,
            options.y.unwrap_or(100) as i32,
            width as i32,
            height as i32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        if window.is_null() {
            return Err("the machine did not make the window of the box.".to_string());
        }
        // The text of the question, the field of the answer and the two
        // buttons of it.
        let static_class = Wide::of("STATIC");
        let prompt_wide = Wide::of(prompt);
        let _ = CreateWindowExW(
            0,
            static_class.as_ptr(),
            prompt_wide.as_ptr(),
            WS_CHILD | WS_VISIBLE,
            10,
            10,
            (width - 20) as i32,
            30,
            window,
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        let edit_class = Wide::of("EDIT");
        let field = CreateWindowExW(
            0,
            edit_class.as_ptr(),
            std::ptr::null(),
            WS_CHILD | WS_VISIBLE,
            10,
            45,
            (width - 20) as i32,
            24,
            window,
            std::ptr::null_mut(),
            module,
            std::ptr::null(),
        );
        THE_FIELD.with(|slot| *slot.borrow_mut() = Some(field));
        let button_class = Wide::of("BUTTON");
        let ok_text = Wide::of("OK");
        let _ = CreateWindowExW(
            0,
            button_class.as_ptr(),
            ok_text.as_ptr(),
            WS_CHILD | WS_VISIBLE,
            (width - 190) as i32,
            (height - 45) as i32,
            80,
            24,
            window,
            std::ptr::dangling_mut(),
            module,
            std::ptr::null(),
        );
        let cancel_text = Wide::of("Cancel");
        let _ = CreateWindowExW(
            0,
            button_class.as_ptr(),
            cancel_text.as_ptr(),
            WS_CHILD | WS_VISIBLE,
            (width - 100) as i32,
            (height - 45) as i32,
            80,
            24,
            window,
            2usize as *mut _,
            module,
            std::ptr::null(),
        );
        the_field_is_written(field, default);
        ShowWindow(window, 5);
        let _ = SetFocus(field);
        // The timer of the box is the time that it waits for the user.
        let timer = options
            .timeout
            .map(|seconds| SetTimer(window, 1, (seconds * 1000.0) as u32, None));
        let mut message = std::mem::zeroed();
        let mut answered = false;
        while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) > 0 {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
            if message.message == WM_ANSWERED {
                answered = true;
                break;
            }
            if message.message == WM_CLOSE {
                break;
            }
            if message.message == WM_TIMER {
                // The time of the box is up: the answer is the timeout.
                THE_ANSWER.with(|answer| {
                    *answer.borrow_mut() = Some((String::new(), "Timeout".to_string()));
                });
                let _ = DestroyWindow(window);
                answered = true;
                break;
            }
        }
        if let Some(timer) = timer {
            let _ = KillTimer(window, timer);
        }
        let _ = DestroyWindow(window);
        THE_FIELD.with(|slot| *slot.borrow_mut() = None);
        let answer = THE_ANSWER.with(|answer| answer.borrow_mut().take());
        let Some((value, result)) = answer else {
            return Ok(InputAnswer {
                value: String::new(),
                result: "Cancel".to_string(),
            });
        };
        Ok(InputAnswer {
            value,
            result: if answered {
                result
            } else {
                "Cancel".to_string()
            },
        })
    }
}

// -- the pickers of the files --------------------------------------------------

/// `FOS_NOCHANGEDIR` (`ShObjIdl_core.h`): the picker does not change the
/// working directory of the script while the user navigates in it.
const FOS_NOCHANGEDIR: u32 = 0x8;
/// `FOS_PICKFOLDERS`: the picker asks for a folder instead of a file.
const FOS_PICKFOLDERS: u32 = 0x20;
/// `FOS_ALLOWMULTISELECT`: the user may choose more than one file.
const FOS_ALLOWMULTISELECT: u32 = 0x200;
/// `FOS_FILEMUSTEXIST`: the file that the user chose must be there.
const FOS_FILEMUSTEXIST: u32 = 0x1000;
/// `FOS_PATHMUSTEXIST`: the folder of the file that the user chose must
/// be there.
const FOS_PATHMUSTEXIST: u32 = 0x800;
/// `FOS_CREATEPROMPT`: the picker asks before it makes a new file.
const FOS_CREATEPROMPT: u32 = 0x2000;
/// `FOS_OVERWRITEPROMPT`: the picker asks before it overwrites a file.
const FOS_OVERWRITEPROMPT: u32 = 0x2;
/// `FOS_NODEREFERENCELINKS`: the picker keeps a shortcut as it is
/// instead of following it to its target.
const FOS_NODEREFERENCELINKS: u32 = 0x100000;

/// The user is asked for a file, or for a folder.
///
/// The picker is the shell of the machine, and the rules are the ones
/// of the v2.0.31, read in its source (`source/script2.cpp` of the
/// reference tree): the flags of the picker come from the options of
/// the script, the title and the filter are written when the script
/// gave them, and the working directory is the folder that the picker
/// opens in when it is a directory that is there, and is split at the
/// last backslash into the folder and the name of the file otherwise.
///
/// The answer is one path, or many of them when the user may choose
/// more than one, and it is empty when the user closed the picker
/// without choosing anything.
pub fn file_select(
    options: &FileOptions,
    root: &str,
    title: &str,
    filter: &str,
) -> Result<FileAnswer, String> {
    // The flags of the picker, from the options of the script.
    let mut flags = FOS_NOCHANGEDIR;
    if options.directory {
        flags |= FOS_PICKFOLDERS;
    }
    if options.multiple {
        flags |= FOS_ALLOWMULTISELECT;
    }
    if (options.flags & 0x01) != 0 {
        flags |= FOS_FILEMUSTEXIST;
    }
    if (options.flags & 0x02) != 0 {
        flags |= FOS_PATHMUSTEXIST;
    }
    if (options.flags & 0x08) != 0 {
        flags |= FOS_CREATEPROMPT;
    }
    if (options.flags & 0x10) != 0 {
        flags |= FOS_OVERWRITEPROMPT;
    }
    if (options.flags & 0x20) != 0 {
        flags |= FOS_NODEREFERENCELINKS;
    }
    // The picker of the save when the script asks for one.
    let picker = if options.save {
        ThePicker::to_save()?
    } else {
        ThePicker::to_open()?
    };
    picker.the_options_are_set(flags)?;
    if !title.is_empty() {
        picker.the_title_is_written(title)?;
    }
    let kinds = the_kinds_of_the_filter(filter, options.directory);
    if !kinds.is_empty() {
        picker.the_filter_is_written(&kinds)?;
    }
    // The working directory of the script: a directory that is there is
    // the folder that the picker opens in; anything else is a name of a
    // file with maybe a folder before it.
    if std::path::Path::new(root).is_dir() {
        picker.the_folder_is_set(root)?;
    } else {
        match root.rfind('\\') {
            Some(at) => {
                picker.the_folder_is_set(&root[..at])?;
                picker.the_file_name_is_written(&root[at + 1..])?;
            }
            None => {
                picker.the_file_name_is_written(root)?;
            }
        }
    }
    match picker.the_picker_is_shown(options.multiple)? {
        // The user closed the picker without choosing anything.
        None => Ok(if options.multiple {
            FileAnswer::Many(Vec::new())
        } else {
            FileAnswer::One(String::new())
        }),
        Some(paths) => Ok(if options.multiple {
            FileAnswer::Many(paths)
        } else {
            FileAnswer::One(paths.into_iter().next().unwrap_or_default())
        }),
    }
}

/// The user is asked for a folder.
///
/// The picker is the shell of the machine with the flag that picks
/// folders, and the answer is the path of the folder that the user
/// chose, or the empty text when the user closed the picker.
///
/// The options are the number of the v2: 1 is the button that makes a
/// new folder. The other two flags of the v2 -- 2, the field where the
/// name is written, and 4, the old style of the dialog -- have no
/// answer in the picker of the shell: the user may always write a name
/// in it, and the picker of the shell is the only one there is.
pub fn dir_select(starting: &str, options: i64, prompt: &str) -> Result<String, String> {
    let mut flags = FOS_NOCHANGEDIR | FOS_PICKFOLDERS;
    if (options & 0x01) != 0 {
        flags |= FOS_CREATEPROMPT;
    }
    let picker = ThePicker::to_open()?;
    picker.the_options_are_set(flags)?;
    if !prompt.is_empty() {
        picker.the_title_is_written(prompt)?;
    }
    if !starting.is_empty() {
        picker.the_folder_is_set(starting)?;
    }
    match picker.the_picker_is_shown(false)? {
        None => Ok(String::new()),
        Some(paths) => Ok(paths.into_iter().next().unwrap_or_default()),
    }
}

/// One kind of the filter of the script: the name that is shown and the
/// pattern of the files that it keeps.
///
/// The text of the v2 is "Name (pattern)": the name is the text before
/// the first "(" -- with the space before the parenthesis taken away,
/// the way the v2.0.31 does it (`source/script2.cpp` of the reference
/// tree) -- and the pattern is the text between the first "(" and the
/// last ")". A text without a "(" is the name and the pattern both, and
/// an empty text is no kind at all.
/// The `MsgBox` of v2: a message box of the machine, and the name of the
/// button that was pressed. The options are the number of v2, whose
/// buttons and icon are the ones of the machine's message box.
pub fn msg_box(text: &str, title: &str, options: i64) -> Result<String, String> {
    // The reference announces the dialog to the window of the script
    // before the box pumps (`POST_AHK_DIALOG`): the handler of
    // `OnMessage(0x44)` runs inside the wait, and a script themes the box
    // dark by subclassing it there.
    super::messages::the_dialog_is_announced();
    let kind = if options < 0 { 0 } else { options as u32 };
    let answer = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            Wide::of(text).as_ptr(),
            Wide::of(title).as_ptr(),
            kind,
        )
    };
    let name = match answer {
        1 => "OK",
        2 => "Cancel",
        3 => "Abort",
        4 => "Retry",
        5 => "Ignore",
        6 => "Yes",
        7 => "No",
        8 => "Close",
        10 => "TryAgain",
        11 => "Continue",
        _ => "OK",
    };
    Ok(name.to_string())
}

fn the_kind_of_the_filter(text: &str) -> Option<(&str, &str)> {    if text.is_empty() {
        return None;
    }
    let Some(open) = text.find('(') else {
        // No parenthesis: the whole text is the name and the pattern.
        return Some((text, text));
    };
    let name = text[..open].strip_suffix(' ').unwrap_or(&text[..open]);
    let rest = &text[open + 1..];
    let pattern = match rest.rfind(')') {
        Some(close) => &rest[..close],
        None => rest,
    };
    Some((name, pattern))
}

/// The kinds of the filter that the picker offers.
///
/// The v2 always adds the kind of all the files, "All Files" with the
/// pattern "*.*" (`source/script2.cpp` of the reference tree), except
/// when the user chooses a folder: then it asks for no kinds at all.
fn the_kinds_of_the_filter(filter: &str, directory: bool) -> Vec<(&str, &str)> {
    if directory {
        return Vec::new();
    }
    let mut kinds = Vec::new();
    if let Some(kind) = the_kind_of_the_filter(filter) {
        kinds.push(kind);
    }
    kinds.push(("All Files", "*.*"));
    kinds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_filter_text_is_split_into_the_name_and_the_pattern() {
        assert_eq!(
            the_kind_of_the_filter("Text (*.txt)"),
            Some(("Text", "*.txt"))
        );
        assert_eq!(the_kind_of_the_filter("*.txt"), Some(("*.txt", "*.txt")));
        assert_eq!(the_kind_of_the_filter("Text ()"), Some(("Text", "")));
        assert_eq!(
            the_kind_of_the_filter("Text (*.txt;*.log)"),
            Some(("Text", "*.txt;*.log"))
        );
    }

    #[test]
    fn an_empty_filter_text_is_no_kind_of_the_script() {
        assert_eq!(the_kind_of_the_filter(""), None);
    }

    #[test]
    fn the_kinds_of_the_filter_always_have_the_kind_of_all_the_files() {
        assert_eq!(
            the_kinds_of_the_filter("Text (*.txt)", false),
            vec![("Text", "*.txt"), ("All Files", "*.*")]
        );
        assert_eq!(
            the_kinds_of_the_filter("", false),
            vec![("All Files", "*.*")]
        );
    }

    #[test]
    fn the_picker_of_a_folder_is_asked_for_no_kinds_at_all() {
        assert!(the_kinds_of_the_filter("Text (*.txt)", true).is_empty());
        assert!(the_kinds_of_the_filter("", true).is_empty());
    }
}
