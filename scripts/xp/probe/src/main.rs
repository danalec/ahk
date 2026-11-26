// The probe of the XP floor: a binary of Rust with no standard library,
// for i686-pc-windows-gnu, that asks the machine three questions and
// writes the answers on the console.
#![no_std]
#![no_main]

use core::panic::PanicInfo;

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
    fn GetTickCount() -> u32;
    fn GetVersionExW(info: *mut OsVersionInfoExW) -> i32;
    fn GetCurrentProcessId() -> u32;
}

/// A line of text, on the stack, with no allocator.
struct Line {
    buf: [u8; 192],
    len: usize,
}

impl Line {
    const fn new() -> Self {
        Line { buf: [0; 192], len: 0 }
    }

    fn put(&mut self, s: &str) {
        for b in s.as_bytes() {
            if self.len < self.buf.len() {
                self.buf[self.len] = *b;
                self.len += 1;
            }
        }
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

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let mut line = Line::new();
    line.put("ahk-xp-probe: alive\r\n");

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
        line.put("windows: ");
        line.put_u32(info.major);
        line.put(".");
        line.put_u32(info.minor);
        line.put(" build ");
        line.put_u32(info.build);
        line.put(" sp");
        line.put_u32(info.service_pack_major as u32);
        line.put("\r\n");
    } else {
        line.put("windows: GetVersionExW failed\r\n");
    }

    line.put("tick: ");
    line.put_u32(unsafe { GetTickCount() });
    line.put("\r\n");
    line.put("pid: ");
    line.put_u32(unsafe { GetCurrentProcessId() });
    line.put("\r\n");
    line.put("rust: no_std, i686-pc-windows-gnu, panic=abort\r\n");
    line.say();
    0
}
