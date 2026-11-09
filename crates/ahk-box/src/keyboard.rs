//! A keyboard of our own, for the keys `ydotool type` cannot express.
//!
//! A port of `src/ahk/keyboard.py` from the archived Python implementation.
//!
//! `ydotool type` is exact, and it is enough for text: it types every
//! character once, in order. It cannot press Return, Backspace, Tab,
//! Escape, or any key with a modifier -- and on the lab box the
//! alternative is broken in a way that is worse than "not implemented":
//!
//! * `ydotool key "Return"` is a **silent no-op** on ydotool 1.x, which
//!   takes numeric keycodes and reads a name as zero.
//! * `ydotool key 28` presses the key but releases it late enough that
//!   the compositor's auto-repeat fires: one call for the letter `u`
//!   produced about a hundred and thirty `u`s and left the key held.
//! * `ydotool key 30:1` (ctrl+a) is not understood at all: the suffix is
//!   ignored and the bare key repeats instead.
//!
//! So the modifier-carrying and the named keys go through a device of our
//! own, the same way the pointer does: one uinput node that reports key
//! events, with an explicit keydown and an explicit keyup, a few
//! milliseconds apart. The compositor's auto-repeat only fires while a key
//! is *held*, and this device never holds one.
//!
//! Creating the device needs `/dev/uinput`, which is group-writable by
//! `input` on the box this targets, and no privileges beyond that. The
//! device is Linux-only: on another platform `Keyboard::open` says so
//! instead of pretending.

use std::fs::File;
use std::io::Write;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;
use std::time::Duration;

// -- the kernel ABI --------------------------------------------------------------

/// The numbers of the kernel ABI.
///
/// They are the same everywhere; only the calls are Linux-only, so the
/// module is compiled everywhere and the constants stay readable.
#[allow(dead_code)]
mod abi {
    pub const UINPUT_IOCTL_BASE: u8 = b'U';

    /// `_IO('U', n)`: dir<<30 | size<<16 | type<<8 | nr
    pub const fn io(nr: u8) -> u64 {
        ((UINPUT_IOCTL_BASE as u64) << 8) | nr as u64
    }

    /// `_IOW('U', n, size)`: the size is part of the request number.
    pub const fn iow(nr: u8, size: usize) -> u64 {
        (1u64 << 30) | ((size as u64) << 16) | ((UINPUT_IOCTL_BASE as u64) << 8) | nr as u64
    }

    pub const UI_SET_EVBIT: u64 = iow(100, 4);
    pub const UI_SET_KEYBIT: u64 = iow(101, 4);
    pub const UI_SET_RELBIT: u64 = iow(102, 4);
    pub const UI_SET_MSCBIT: u64 = iow(104, 4);
    pub const UI_DEV_SETUP: u64 = iow(3, 92);
    pub const UI_DEV_CREATE: u64 = io(1);

    pub const EV_SYN: u16 = 0x00;
    pub const EV_KEY: u16 = 0x01;
    pub const EV_REL: u16 = 0x02;
    pub const EV_MSC: u16 = 0x04;
    pub const SYN_REPORT: u16 = 0x00;

    pub const REL_X: u16 = 0x00;
    pub const REL_Y: u16 = 0x01;
    pub const REL_RY: u16 = 0x04;
    pub const REL_DIAL: u16 = 0x07;

    pub const MSC_TIMESTAMP: u16 = 0x01;
    pub const MSC_SCAN: u16 = 0x04;
}

#[allow(unused_imports)]
use abi::{
    EV_KEY, EV_MSC, EV_REL, EV_SYN, MSC_SCAN, MSC_TIMESTAMP, REL_DIAL, REL_RY, REL_X, REL_Y,
    SYN_REPORT, UI_DEV_CREATE, UI_DEV_SETUP, UI_SET_EVBIT, UI_SET_KEYBIT, UI_SET_MSCBIT,
    UI_SET_RELBIT,
};

/// How long a key stays down.
///
/// Long enough for the compositor to see the press, short enough that the
/// auto-repeat never starts: the kernel's repeat delay is half a second.
pub const KEY_HOLD: Duration = Duration::from_millis(20);

/// How long the gap between one key and the next is.
///
/// A client coalesces a burst of key events: a hundred backspaces that
/// arrive back to back are counted as a handful, not as a hundred.
/// Measured on the lab box on 2026-10-25, a client lost most of a burst of
/// 320 backspaces; with this gap between the release and the next press,
/// every one of them counts.
pub const KEY_GAP: Duration = Duration::from_millis(20);

/// How long the compositor needs to notice a freshly created device.
///
/// libinput enumerates it through udev, and a key pressed before that
/// lands on nothing. Measured on the lab box: four seconds is enough.
pub const ENUMERATION_WAIT: Duration = Duration::from_secs(4);

/// The device could not be created or driven.
#[derive(Debug, thiserror::Error)]
pub enum KeyboardError {
    #[error("the keyboard is a Linux uinput device; this is not Linux")]
    NotLinux,
    #[error("cannot open /dev/uinput: {0}. The user needs to be in the `input` group.")]
    CannotOpen(String),
    #[error("ioctl 0x{0:x} failed: {1}")]
    Ioctl(u64, String),
    #[error("UI_DEV_SETUP failed: {0}")]
    Setup(String),
}

/// A keyboard that presses what it is told, once.
///
/// The device is created on construction and destroyed when the object is
/// dropped. Creating it twice is harmless: the kernel gives each node its
/// own event number, and the compositor picks up the new one by hotplug.
pub struct Keyboard {
    file: File,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    codes: Vec<u16>,
}

impl Keyboard {
    /// Create the device and announce it to the compositor.
    pub fn open(codes: Vec<u16>) -> Result<Self, KeyboardError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = codes;
            Err(KeyboardError::NotLinux)
        }
        #[cfg(target_os = "linux")]
        {
            let file = open_uinput()?;
            let keyboard = Self { file, codes };
            keyboard.configure()?;
            Ok(keyboard)
        }
    }

    /// Press a key.
    ///
    /// The scan code goes first, the way a keyboard sends it: the
    /// compositor reads the key through the scan code, and a key that
    /// arrives without one is reported with the key code -1, which a
    /// client cannot use.
    pub fn down(&mut self, code: u16) -> Result<(), KeyboardError> {
        self.write(EV_MSC, MSC_SCAN, code as i32)?;
        self.emit(EV_KEY, code, 1)
    }

    /// Release a key.
    pub fn up(&mut self, code: u16) -> Result<(), KeyboardError> {
        self.emit(EV_KEY, code, 0)
    }

    /// Press and release a key, once.
    pub fn tap(&mut self, code: u16) -> Result<(), KeyboardError> {
        self.down(code)?;
        std::thread::sleep(KEY_HOLD);
        self.up(code)?;
        std::thread::sleep(KEY_GAP);
        Ok(())
    }

    /// Press a key with modifiers held down around it.
    ///
    /// The modifiers go down first and up last, which is the order a
    /// keyboard delivers them in, and the whole thing is shorter than the
    /// auto-repeat delay so nothing repeats.
    pub fn tap_with_modifiers(
        &mut self,
        code: u16,
        modifiers: &[u16],
    ) -> Result<(), KeyboardError> {
        for modifier in modifiers {
            self.down(*modifier)?;
        }
        std::thread::sleep(KEY_HOLD);
        self.down(code)?;
        std::thread::sleep(KEY_HOLD);
        self.up(code)?;
        for modifier in modifiers.iter().rev() {
            self.up(*modifier)?;
        }
        std::thread::sleep(KEY_GAP);
        Ok(())
    }

    /// The sync event that closes a frame.
    pub fn sync(&mut self) -> Result<(), KeyboardError> {
        self.emit(EV_SYN, SYN_REPORT, 0)
    }

    fn emit(&mut self, event_type: u16, code: u16, value: i32) -> Result<(), KeyboardError> {
        self.write(event_type, code, value)?;
        // The frame has to be closed. Without the SYN_REPORT the event
        // sits in the compositor's buffer and is never delivered: the
        // keyboard is alive, the device is enumerated, and nothing the
        // client sends arrives.
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    /// Write one event, without closing the frame.
    fn write(&mut self, event_type: u16, code: u16, value: i32) -> Result<(), KeyboardError> {
        // struct input_event { struct timeval time; __u16 type; __u16 code; __s32 value; }
        // timeval is two longs on 64-bit Linux, so the struct is 24 bytes.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let mut event = [0u8; 24];
        event[0..8].copy_from_slice(&(now.as_secs() as i64).to_ne_bytes());
        // The timeval's second field is MICROSECONDS. Nanoseconds there
        // are a thousand times too large, and the compositor sees an event
        // from the far future.
        event[8..16].copy_from_slice(&(now.subsec_micros() as i64).to_ne_bytes());
        event[16..18].copy_from_slice(&event_type.to_ne_bytes());
        event[18..20].copy_from_slice(&code.to_ne_bytes());
        event[20..24].copy_from_slice(&value.to_ne_bytes());
        self.file
            .write_all(&event)
            .map_err(|error| KeyboardError::Ioctl(0, error.to_string()))
    }

    #[cfg(target_os = "linux")]
    fn configure(&self) -> Result<(), KeyboardError> {
        let fd = self.file.as_raw_fd();
        self.ioctl(UI_SET_EVBIT, EV_KEY as libc::c_int)?;
        self.ioctl(UI_SET_EVBIT, EV_REL as libc::c_int)?;
        self.ioctl(UI_SET_EVBIT, EV_MSC as libc::c_int)?;
        self.ioctl(UI_SET_EVBIT, EV_SYN as libc::c_int)?;
        for code in &self.codes {
            self.ioctl(UI_SET_KEYBIT, *code as libc::c_int)?;
        }
        // The relative axes and the scan codes are what the working
        // `ydotool` device on this box announces. They are announced for
        // the same reason: libinput builds the keymap of a device from
        // what the device says it can do, and a keyboard that announces
        // nothing but keys is announced to libinput as a keyboard whose
        // keys it cannot map -- the key events arrive and are reported
        // with the key code -1, which a client cannot use.
        for code in [REL_X, REL_Y, REL_RY, REL_DIAL] {
            self.ioctl(UI_SET_RELBIT, code as libc::c_int)?;
        }
        for code in [MSC_SCAN, MSC_TIMESTAMP] {
            self.ioctl(UI_SET_MSCBIT, code as libc::c_int)?;
        }
        // struct uinput_setup { struct input_id id; char name[80]; __u32 ff_effects_max; }
        let mut setup = [0u8; 92];
        setup[0..2].copy_from_slice(&0x06u16.to_ne_bytes()); // BUS_VIRTUAL
        setup[2..4].copy_from_slice(&0x2333u16.to_ne_bytes()); // the ydotool vendor
        setup[4..6].copy_from_slice(&0x6666u16.to_ne_bytes()); // the ydotool product
        setup[6..8].copy_from_slice(&0x0001u16.to_ne_bytes());
        let name = b"ahk keyboard";
        setup[8..8 + name.len()].copy_from_slice(name);
        let result = unsafe { libc::ioctl(fd, UI_DEV_SETUP, setup.as_ptr()) };
        if result < 0 {
            return Err(KeyboardError::Setup(
                std::io::Error::last_os_error().to_string(),
            ));
        }
        self.ioctl(UI_DEV_CREATE, 0)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn ioctl(&self, request: u64, argument: libc::c_int) -> Result<(), KeyboardError> {
        let fd = self.file.as_raw_fd();
        let result = unsafe { libc::ioctl(fd, request, argument) };
        if result < 0 {
            return Err(KeyboardError::Ioctl(
                request,
                std::io::Error::last_os_error().to_string(),
            ));
        }
        Ok(())
    }
}

/// Open the device file, with the flags the ioctls want.
#[cfg(target_os = "linux")]
fn open_uinput() -> Result<File, KeyboardError> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open("/dev/uinput")
        .map_err(|error| KeyboardError::CannotOpen(error.to_string()))
}
