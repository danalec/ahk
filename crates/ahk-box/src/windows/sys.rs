//! The Windows API, as the tree speaks it.
//!
//! The API of the machine is a C API: the names are wide, the handles are
//! pointers, and a failure is a number. This module is the place where
//! that becomes the shape of the tree: a wide string becomes a `String`,
//! a failure becomes the reason of it, and the `unsafe` is written once,
//! here.
//!
//! The names of the API are the names of the documentation of Microsoft:
//! `GetUserNameW` is of Windows 2000, `GetTickCount64` is of Windows
//! Vista, and `SHGetKnownFolderPath` is of Windows Vista too. A note that
//! says which one a call is, is the note of the call that the tree makes.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
};
use windows_sys::core::BOOL;

/// A string of the Windows: UTF-16, with a zero at the end.
///
/// The API of the machine takes the names as wide strings, and a name
/// that is not wide is a name the machine does not understand.
pub struct Wide(Vec<u16>);

impl Wide {
    /// The text of the tree, as the machine wants it.
    pub fn of(text: &str) -> Self {
        let mut wide: Vec<u16> = text.encode_utf16().collect();
        wide.push(0);
        Self(wide)
    }

    /// The pointer that the API wants.
    ///
    /// The zero at the end is part of the string, and the pointer is of
    /// the whole of it.
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    /// The pointer that the API wants, and that it may write to.
    ///
    /// The `MENUITEMINFO` of the machine keeps the text of an item in a
    /// pointer that it writes to.
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.0.as_mut_ptr()
    }

    /// The number of the letters of the text, without the zero at the end.
    pub fn the_number_of_the_letters(&self) -> u32 {
        (self.0.len() - 1) as u32
    }

    /// The letters of the string, with the zero at the end.
    pub fn letters(&self) -> &[u16] {
        &self.0
    }

    /// The text of the string.
    ///
    /// A name that is not text becomes the replacement character: the
    /// machine gives the name, and the tree shows what it can.
    pub fn to_text(&self) -> String {
        let end = self
            .0
            .iter()
            .position(|letter| *letter == 0)
            .unwrap_or(self.0.len());
        String::from_utf16_lossy(&self.0[..end])
    }
}

/// The text of a failure of the machine.
///
/// The machine answers a number, and the number has a text in the tables
/// of the system: `FormatMessageW` is the reader of that table.
pub fn the_message_of(code: u32) -> String {
    unsafe {
        let mut buffer = [0u16; 512];
        let length = FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(),
            code,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            std::ptr::null(),
        );
        if length == 0 {
            return format!("the failure of the machine is the number {code}.");
        }
        let text = String::from_utf16_lossy(&buffer[..length as usize]);
        // The text of the system ends with the line break and the spaces
        // of the table.
        let text = text.trim_end().to_string();
        if text.is_empty() {
            format!("the failure of the machine is the number {code}.")
        } else {
            format!("({code}) {text}")
        }
    }
}

/// The text of the last failure of the machine.
///
/// The number is read right after the call that failed: the machine
/// keeps it until the next one.
pub fn the_last_error() -> String {
    the_message_of(unsafe { GetLastError() })
}

/// The text of the failure of a call that answered a zero.
///
/// A call of the Windows that fails answers a zero and keeps the number
/// of the failure; a call that answers a boolean says the same.
pub fn the_failure_of(answer: BOOL) -> Result<(), String> {
    if answer == 0 {
        Err(the_last_error())
    } else {
        Ok(())
    }
}

/// The address of a symbol of a DLL of the machine, when the machine has it.
///
/// WHY: a call that XP does not have cannot be imported: the loader of
/// XP refuses the program before the first line of it runs. The name of
/// the function is asked to the DLL at the time of the call, and the
/// program answers that the function is not there.
pub fn the_symbol_of(dll: &str, name: &str) -> Option<*const core::ffi::c_void> {
    unsafe {
        let mut module = GetModuleHandleW(Wide::of(dll).as_ptr());
        if module.is_null() {
            // A DLL que o programa ainda nao carregou: a maquina a
            // carrega, e a maquina que nao a tem nao a carrega.
            module = LoadLibraryW(Wide::of(dll).as_ptr());
        }
        if module.is_null() {
            return None;
        }
        // O nome que o `GetProcAddress` pede acaba em zero: o `&str`
        // da arvore nao acaba, e o nome sem o zero e' outro nome.
        let name = std::ffi::CString::new(name).ok()?;
        GetProcAddress(module, name.as_ptr() as *const u8)
            .map(|symbol| symbol as *const core::ffi::c_void)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_wide_string_keeps_the_text_and_the_zero() {
        let wide = Wide::of("the tree");
        assert_eq!(wide.to_text(), "the tree");
        assert_eq!(unsafe { *wide.as_ptr() }, u16::from(b't'));
        // The zero is the last letter of the string.
        let length = unsafe { core::slice::from_raw_parts(wide.as_ptr(), 9) };
        assert_eq!(length[8], 0);
    }

    #[test]
    fn a_wide_string_of_a_text_that_is_not_text_is_read_as_it_can_be() {
        // The lone surrogate is not a letter of UTF-8, and the tree shows
        // the replacement character instead of failing.
        let wide = Wide::of("\u{fffd}");
        assert_eq!(wide.to_text(), "\u{fffd}");
    }

    #[test]
    fn the_text_of_a_failure_names_the_number_and_the_words_of_the_system() {
        // The number 2 is the file that is not there.
        let message = the_message_of(2);
        assert!(message.starts_with("(2) "), "{message}");
    }

    #[test]
    fn the_text_of_a_number_that_is_not_a_failure_still_answers() {
        let message = the_message_of(0x0000_0001);
        assert!(!message.is_empty());
    }
}
