// The probe of the floor: a binary of Rust with no standard library,
// for i686-pc-windows-gnu, that asks the XP machine which of the
// symbols of the tree it has, and which of them it does not.
//
// The table is the table of the imports of the modern tree, built for
// thirty-two bits (`scripts/xp/floor/gen_table.py`), and the answer of
// the machine is the floor itself: nothing is taken from a document,
// every symbol is asked of the DLL that holds it, on the machine that
// is the floor.
//
// The classes of COM are asked as well: a class is not a symbol, and
// the floor of a class is the registry of the machine, not the import
// table of a binary.
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod table;
#[cfg(target_arch = "x86_64")]
mod table64;

#[cfg(target_arch = "x86")]
use table::SYMBOLS;
#[cfg(target_arch = "x86_64")]
use table64::SYMBOLS;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct OsVersionInfoExW {
    os_version_info_size: u32,
    major: u32,
    minor: u32,
    build: u32,
    platform_id: u32,
    csd_version: [u16; 128],
    service_pack_major: u16,
    service_pack_minor: u16,
    suite_mask: u16,
    product_type: u8,
    reserved: u8,
}

unsafe extern "system" {
    fn GetStdHandle(which: u32) -> *mut core::ffi::c_void;
    fn WriteFile(
        h: *mut core::ffi::c_void,
        buf: *const u8,
        n: u32,
        written: *mut u32,
        overlapped: *mut core::ffi::c_void,
    ) -> i32;
    fn GetVersionExW(info: *mut OsVersionInfoExW) -> i32;
    fn GetModuleHandleA(name: *const u8) -> *mut core::ffi::c_void;
    fn LoadLibraryA(name: *const u8) -> *mut core::ffi::c_void;
    fn GetProcAddress(module: *mut core::ffi::c_void, name: *const u8) -> *mut core::ffi::c_void;
    fn CLSIDFromString(text: *const u16, out: *mut core::ffi::c_void) -> i32;
    fn CoInitializeEx(reserved: u32, flags: u32) -> i32;
    fn CoCreateInstance(
        clsid: *const core::ffi::c_void,
        outer: *const core::ffi::c_void,
        context: u32,
        iid: *const core::ffi::c_void,
        out: *mut *mut core::ffi::c_void,
    ) -> i32;
}

/// The classes of COM that the tree asks for, and the control.
///
/// The names are the ones of the tree, in the spelling of the registry.
const CLASSES: &[(&str, &str)] = &[
    ("{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}", "CLSID_FileOpenDialog"),
    ("{BCDE0395-E52F-467C-8E3D-C4579291692E}", "CLSID_MMDeviceEnumerator"),
    ("{00021401-0000-0000-C000-000000000046}", "CLSID_ShellLink"),
];

/// The interface of `IUnknown`, the control of the test of the classes.
const IID_IUNKNOWN: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];

/// CLSCTX_INPROC_SERVER: the class is a DLL of the machine.
const CLSCTX_INPROC_SERVER: u32 = 1;

/// A line of text, on the stack, with no allocator.
struct Line {
    buf: [u8; 256],
    len: usize,
}

impl Line {
    const fn new() -> Self {
        Line {
            buf: [0; 256],
            len: 0,
        }
    }

    fn put(&mut self, s: &str) {
        for b in s.as_bytes() {
            if self.len < self.buf.len() {
                self.buf[self.len] = *b;
                self.len += 1;
            }
        }
    }

    /// The line is empty again.
    ///
    /// The buffer is the buffer of the line: it is filled, it is
    /// written to the console, and it is emptied. Without the
    /// emptying, the lines of the answer are glued to each other, and
    /// the buffer of two hundred and fifty-six bytes is full long
    /// before the last symbol is asked.
    fn clear(&mut self) {
        self.len = 0;
    }

    fn put_u32(&mut self, mut v: u32) {
        let mut digits = [0u8; 10];
        let mut i = digits.len();
        if v == 0 {
            self.put("0");
            return;
        }
        while v > 0 {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
        }
        for d in &digits[i..] {
            self.put(core::str::from_utf8(core::slice::from_ref(d)).unwrap_or("?"));
        }
    }

    fn put_hex(&mut self, mut v: u32) {
        let digits = b"0123456789abcdef";
        self.put("0x");
        let mut started = false;
        for shift in (0..8).rev() {
            let nibble = ((v >> (shift * 4)) & 0xF) as usize;
            if nibble != 0 || started || shift == 0 {
                self.put(core::str::from_utf8(core::slice::from_ref(&digits[nibble])).unwrap_or("?"));
                started = true;
            }
        }
    }

    fn say(&self) {
        unsafe {
            // STD_OUTPUT_HANDLE is the pseudo handle -11, as a DWORD.
            let h = GetStdHandle((-11i32) as u32);
            let mut written = 0u32;
            WriteFile(
                h,
                self.buf.as_ptr(),
                self.len as u32,
                &mut written,
                core::ptr::null_mut(),
            );
        }
    }
}

/// The name of a DLL, in the narrow characters of the machine.
fn narrow(name: &str, buf: &mut [u8; 128]) -> *const u8 {
    let mut i = 0;
    for b in name.as_bytes() {
        if i < buf.len() - 1 {
            buf[i] = *b;
            i += 1;
        }
    }
    buf[i] = 0;
    buf.as_ptr()
}

/// The name of a class, in the wide characters of the machine.
fn wide(name: &str, buf: &mut [u16; 80]) -> *const u16 {
    let mut i = 0;
    for b in name.as_bytes() {
        if i < buf.len() - 1 {
            buf[i] = *b as u16;
            i += 1;
        }
    }
    buf[i] = 0;
    buf.as_ptr()
}

/// Does the machine have the symbol?  The DLL is opened, and the symbol
/// is asked of it.
fn has_symbol(dll: &str, symbol: &str) -> bool {
    let mut dll_buf = [0u8; 128];
    let mut symbol_buf = [0u8; 128];
    unsafe {
        let name = narrow(dll, &mut dll_buf);
        let mut module = GetModuleHandleA(name);
        if module.is_null() {
            module = LoadLibraryA(name);
        }
        if module.is_null() {
            return false;
        }
        let symbol = narrow(symbol, &mut symbol_buf);
        !GetProcAddress(module, symbol).is_null()
    }
}

/// Is the class of COM registered on the machine?
fn has_class(text: &str) -> Option<i32> {
    let mut wide_buf = [0u16; 80];
    let mut clsid = [0u8; 16];
    unsafe {
        let text = wide(text, &mut wide_buf);
        if CLSIDFromString(text, clsid.as_mut_ptr().cast()) != 0 {
            return None;
        }
        let mut object: *mut core::ffi::c_void = core::ptr::null_mut();
        let answer = CoCreateInstance(
            clsid.as_ptr().cast(),
            core::ptr::null(),
            CLSCTX_INPROC_SERVER,
            IID_IUNKNOWN.as_ptr().cast(),
            &mut object,
        );
        if !object.is_null() {
            // The machine has the class. The object is left alone on
            // purpose: there is no allocator and no release of COM.
        }
        Some(answer)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let mut line = Line::new();

    let mut info = OsVersionInfoExW {
        os_version_info_size: core::mem::size_of::<OsVersionInfoExW>() as u32,
        major: 0,
        minor: 0,
        build: 0,
        platform_id: 0,
        csd_version: [0; 128],
        service_pack_major: 0,
        service_pack_minor: 0,
        suite_mask: 0,
        product_type: 0,
        reserved: 0,
    };
    if unsafe { GetVersionExW(&mut info) } != 0 {
        line.put("machine: windows ");
        line.put_u32(info.major);
        line.put(".");
        line.put_u32(info.minor);
        line.put(" build ");
        line.put_u32(info.build);
        line.put(" sp");
        line.put_u32(info.service_pack_major as u32);
        line.put("\r\n");
        line.say();
    }

    unsafe {
        CoInitializeEx(0, 2);
    }

    let mut missing = 0u32;
    // Both halves of the answer are written: the symbols that the
    // machine lacks AND the symbols that it has.
    //
    // WHY: the gate (`scripts/xp/gate.py`) is an oracle of three
    // answers and not of two.  A symbol in the import table of the
    // binary is either measured missing, measured present, or never
    // asked about -- and the third answer fails the gate, because a
    // floor that was not measured is a floor that is guessed.  A probe
    // that writes only the MISSING lines leaves the set of the present
    // ones empty, and then EVERY symbol of every binary is "never asked
    // about": the gate cannot vouch for anything.
    //
    // The table is read from the end so that, on the console of the
    // machine without redirection, the last lines on the screen are the
    // first symbols of the table.
    for (dll, symbol) in SYMBOLS.iter().rev() {
        line.clear();
        if !has_symbol(dll, symbol) {
            missing += 1;
            line.put("MISSING ");
        } else {
            line.put("PRESENT ");
        }
        line.put(dll);
        line.put("!");
        line.put(symbol);
        line.put("\r\n");
        line.say();
    }

    line.clear();
    line.put("symbols: ");
    line.put_u32(SYMBOLS.len() as u32);
    line.put("  missing: ");
    line.put_u32(missing);
    line.put("\r\n");
    line.say();
    for (text, name) in CLASSES {
        line.clear();
        line.put("com ");
        line.put(name);
        line.put(": ");
        match has_class(text) {
            None => line.put("the name of the class is refused\r\n"),
            Some(0) => line.put("registered\r\n"),
            Some(answer) => {
                line.put("NOT registered, the answer is ");
                line.put_hex(answer as u32);
                line.put("\r\n");
            }
        }
        line.say();
    }
    0
}
