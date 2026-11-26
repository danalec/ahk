//! The clipboard of the machine.
//!
//! The clipboard of Windows is a resource of the session: the program
//! opens it, puts a text in it or takes the text out of it, and closes
//! it again. The text of it is wide, and the machine keeps it in the
//! memory of the process that wrote it.

use windows_sys::Win32::Foundation::{GlobalFree, HGLOBAL};
use windows_sys::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, EnumClipboardFormats, GetClipboardData, OpenClipboard,
    SetClipboardData,
};
use windows_sys::Win32::System::Memory::{
    GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock,
};
use windows_sys::Win32::System::Ole::CF_UNICODETEXT;

/// The text of the clipboard of the machine.
///
/// The answer is the text that is there, and the empty text when the
/// clipboard is empty.
pub fn clipboard_read() -> Result<String, String> {
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err("the clipboard of the machine is not open.".to_string());
        }
        let handle = GetClipboardData(CF_UNICODETEXT as u32);
        if handle.is_null() {
            // The clipboard is open, and it has no text: that is the
            // empty text of the v2.
            CloseClipboard();
            return Ok(String::new());
        }
        let text = GlobalLock(handle) as *const u16;
        if text.is_null() {
            CloseClipboard();
            return Err("the machine did not give the text of the clipboard.".to_string());
        }
        let mut length = 0usize;
        while *text.add(length) != 0 {
            length += 1;
        }
        let letters = std::slice::from_raw_parts(text, length);
        let answer = String::from_utf16_lossy(letters);
        GlobalUnlock(handle);
        CloseClipboard();
        Ok(answer)
    }
}

/// The text of the script is put in the clipboard of the machine.
///
/// The machine keeps the text in the memory of the process, and the
/// memory is given to the clipboard: the program that reads it later
/// gets the same text.
pub fn clipboard_write(text: &str) -> Result<String, String> {
    let mut wide: Vec<u16> = text.encode_utf16().collect();
    wide.push(0);
    let bytes = wide.len() * 2;
    unsafe {
        let memory: HGLOBAL = GlobalAlloc(GMEM_MOVEABLE, bytes);
        if memory.is_null() {
            return Err("the machine gave no memory for the clipboard.".to_string());
        }
        let place = GlobalLock(memory);
        if place.is_null() {
            GlobalFree(memory);
            return Err("the machine did not give the memory of the clipboard.".to_string());
        }
        std::ptr::copy_nonoverlapping(wide.as_ptr(), place as *mut u16, wide.len());
        GlobalUnlock(memory);
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            GlobalFree(memory);
            return Err("the clipboard of the machine is not open.".to_string());
        }
        EmptyClipboard();
        let answer = SetClipboardData(CF_UNICODETEXT as u32, memory as HGLOBAL);
        CloseClipboard();
        if answer.is_null() {
            GlobalFree(memory);
            return Err("the machine did not take the text of the clipboard.".to_string());
        }
        Ok(text.to_string())
    }
}

/// Everything that is on the clipboard, as the bytes of every format.
///
/// The `ClipboardAll` of the v2 takes the whole clipboard -- the text,
/// the pictures, the formatting -- and this is the capture of it. Each
/// format is a block: the number of the format, the size of the data,
/// and the data. The formats that are handles of the machine and not
/// memory (the `CF_BITMAP`) are left out, because they cannot be copied
/// as bytes.
pub fn clipboard_read_all() -> Result<Vec<u8>, String> {
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err("the clipboard of the machine is not open.".to_string());
        }
        let mut buffer: Vec<u8> = Vec::new();
        let mut format = 0u32;
        loop {
            format = EnumClipboardFormats(format);
            if format == 0 {
                break;
            }
            let handle = GetClipboardData(format);
            if handle.is_null() {
                continue;
            }
            let size = GlobalSize(handle);
            if size == 0 {
                continue;
            }
            let place = GlobalLock(handle);
            if place.is_null() {
                continue;
            }
            let data = std::slice::from_raw_parts(place as *const u8, size);
            buffer.extend_from_slice(&format.to_le_bytes());
            buffer.extend_from_slice(&(size as u32).to_le_bytes());
            buffer.extend_from_slice(data);
            GlobalUnlock(handle);
        }
        CloseClipboard();
        Ok(buffer)
    }
}

/// The bytes of every format are put back on the clipboard.
///
/// The restore of the `ClipboardAll`: the clipboard is emptied and each
/// block of the capture is put back as its format.
pub fn clipboard_write_all(data: &[u8]) -> Result<(), String> {
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err("the clipboard of the machine is not open.".to_string());
        }
        EmptyClipboard();
        let mut at = 0usize;
        while at + 8 <= data.len() {
            let format = u32::from_le_bytes(data[at..at + 4].try_into().unwrap());
            let size = u32::from_le_bytes(data[at + 4..at + 8].try_into().unwrap()) as usize;
            at += 8;
            if at + size > data.len() {
                break;
            }
            let bytes = &data[at..at + size];
            at += size;
            let memory: HGLOBAL = GlobalAlloc(GMEM_MOVEABLE, size);
            if memory.is_null() {
                continue;
            }
            let place = GlobalLock(memory);
            if place.is_null() {
                GlobalFree(memory);
                continue;
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), place as *mut u8, size);
            GlobalUnlock(memory);
            if SetClipboardData(format, memory).is_null() {
                GlobalFree(memory);
            }
        }
        CloseClipboard();
        Ok(())
    }
}

/// The bytes of the memory of the process at an address.
///
/// The `ClipboardAll(address, size)` reads the data of the clipboard out
/// of the memory. The `ReadProcessMemory` of the process itself answers
/// zero when the address is not readable, so a bad address is a reason
/// and not a crash.
pub fn read_the_memory(address: i64, size: i64) -> Result<Vec<u8>, String> {
    if size < 0 {
        return Err("the size of the data is not a number of bytes.".to_string());
    }
    if size == 0 {
        return Ok(Vec::new());
    }
    use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    let mut buffer = vec![0u8; size as usize];
    let mut read = 0usize;
    let answer = unsafe {
        ReadProcessMemory(
            GetCurrentProcess(),
            address as *const core::ffi::c_void,
            buffer.as_mut_ptr() as *mut core::ffi::c_void,
            size as usize,
            &mut read,
        )
    };
    if answer == 0 {
        return Err("the memory of the data of the clipboard is not readable.".to_string());
    }
    buffer.truncate(read);
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_of_the_clipboard_makes_a_round_trip() {
        // The clipboard of the session is put back the way it was: the
        // test does not keep what the user had in it.
        let before = clipboard_read().expect("the machine gives the text of the clipboard");
        let written = clipboard_write("the text of the tree").expect("the machine takes the text");
        assert_eq!(written, "the text of the tree");
        let read = clipboard_read().expect("the machine gives the text of the clipboard");
        assert_eq!(read, "the text of the tree");
        let _ = clipboard_write(&before);
    }
}
