//! An absolute pointer device, for driving a Wayland session exactly.
//!
//! A port of `src/yoko_mcp/pointer.py` from the Python `yoko-mcp`.
//!
//! ydotool's device is a *relative* mouse with a full keyboard key bitmap
//! and no `INPUT_PROP_POINTER`. libinput classifies that as a keyboard:
//! the key events arrive, the relative axes are ignored, and a click
//! lands nowhere. The box's own working pointer -- Sunshine's
//! `libvirtualhid Mouse (Absolute)` -- has `INPUT_PROP_POINTER` and
//! absolute axes, and libinput treats it as a pointer.
//!
//! So this module creates that kind of device: one uinput node that reports
//! absolute X/Y in screen pixels and carries the three mouse buttons. It is
//! opened, configured, and kept alive for the lifetime of the process; the
//! compositor hotplugs it the same way it hotplugs any other input device.
//!
//! Creating the device needs `/dev/uinput`, which is group-writable by
//! `input` on the box this targets, and it needs no privileges beyond that.
//!
//! The pointer is a *second* device on purpose. The ydotool device stays
//! the keyboard: one node cannot be both, because libinput picks a role
//! per device and the role with the richer key bitmap wins.
//!
//! Two details were measured on the lab box on 2026-10-25, and both are
//! the reason this is not "a click with a sleep in it":
//!
//! * The compositor coalesces events into frames. A motion and a press
//!   sent in the same batch arrive together, and the press is applied at
//!   the pointer's *previous* position. A gap of 20 ms was not enough;
//!   the click landed hundreds of pixels away from where it was aimed.
//! * Adding the tool bits makes libinput classify the node as a tablet,
//!   and a tablet's tip press does not activate anything -- not the
//!   compositor's own decorations, not a client. The node announces
//!   itself as a plain pointer.

use std::fs::File;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
use std::time::Duration;

/// The kernel ABI.
///
/// The numbers are the same everywhere; only the calls are Linux-only, so
/// the module is compiled everywhere and the constants stay readable.
#[allow(dead_code)]
mod abi {
    pub const UINPUT_IOCTL_BASE: u8 = b'U';
    pub const EVDEV_IOCTL_BASE: u8 = b'E';

    /// `_IO('U', n)`: dir<<30 | size<<16 | type<<8 | nr
    pub const fn io(nr: u8) -> u64 {
        ((UINPUT_IOCTL_BASE as u64) << 8) | nr as u64
    }

    /// `_IOW('U', n, size)`: dir<<30 | size<<16 | type<<8 | nr
    pub const fn iow(nr: u8, size: usize) -> u64 {
        (1u64 << 30) | ((size as u64) << 16) | ((UINPUT_IOCTL_BASE as u64) << 8) | nr as u64
    }

    /// `_IOR('E', n, size)`: dir is 2 for a read.
    pub const fn ior(nr: u8, size: usize) -> u64 {
        (2u64 << 30) | ((size as u64) << 16) | ((EVDEV_IOCTL_BASE as u64) << 8) | nr as u64
    }

    pub const UI_SET_EVBIT: u64 = iow(100, 4);
    pub const UI_SET_KEYBIT: u64 = iow(101, 4);
    pub const UI_SET_ABSBIT: u64 = iow(103, 4);
    pub const UI_SET_PROPBIT: u64 = iow(110, 4);
    pub const UI_DEV_SETUP: u64 = iow(3, 92);
    pub const UI_ABS_SETUP: u64 = iow(4, 28);
    pub const UI_DEV_CREATE: u64 = io(1);

    /// `EVIOCGABS(abs)`: the current value of an absolute axis.
    pub const fn eviocgabs(abs: u8) -> u64 {
        ior(0x40 + abs, 24)
    }

    pub const EV_SYN: u16 = 0x00;
    pub const EV_KEY: u16 = 0x01;
    pub const EV_ABS: u16 = 0x03;

    pub const SYN_REPORT: u16 = 0x00;

    pub const ABS_X: u16 = 0x00;
    pub const ABS_Y: u16 = 0x01;

    pub const BTN_LEFT: u16 = 0x110;
    pub const BTN_RIGHT: u16 = 0x111;
    pub const BTN_MIDDLE: u16 = 0x112;

    /// `INPUT_PROP_DIRECT` is a capability *index*, not a bit mask: the
    /// kernel shifts the index into the device's bitmap, so the index 1
    /// shows up as `B: PROP=2`. Passing the mask by mistake asks for a
    /// different capability, and libinput then does not treat the node as
    /// a pointer at all.
    pub const INPUT_PROP_DIRECT: u8 = 1;
}

/// How long to wait between moving the pointer and pressing the button.
///
/// The compositor coalesces events into frames, and a press delivered in
/// the same frame as the motion lands at the previous position.
pub const PRESS_SETTLE: Duration = Duration::from_millis(200);

/// How long the button stays down.
pub const PRESS_HOLD: Duration = Duration::from_millis(40);

/// The name the device announces.
///
/// The event node of the device is found by it, under
/// `/sys/class/input/event*/device/name`.
pub const DEVICE_NAME: &str = "yoko absolute pointer rust";

/// How many times the read of the event node is tried.
///
/// The node is created by the kernel with the mode of a device that only
/// its owner may touch, and `udev` opens it up to the `input` group a
/// moment later. The read waits for that.
pub const POSITION_ATTEMPTS: usize = 20;

/// The gap between the tries.
pub const POSITION_RETRY: Duration = Duration::from_millis(100);

/// The pointer device could not be created or driven.
#[derive(Debug, thiserror::Error)]
pub enum PointerError {
    #[error("the absolute pointer is a Linux uinput device; this is not Linux")]
    NotLinux,
    #[error("cannot open /dev/uinput: {0}. The user needs to be in the `input` group.")]
    CannotOpen(String),
    #[error("ioctl 0x{0:x} failed: {1}")]
    Ioctl(u64, String),
    #[error("UI_DEV_SETUP failed: {0}")]
    Setup(String),
    #[error("the pointer is not open")]
    Closed,
}

/// A pointer that reports screen pixels.
pub struct Pointer {
    file: File,
    width: i64,
    height: i64,
    last: (i64, i64),
}

impl Pointer {
    /// Create the device and announce it to the compositor.
    pub fn open(width: i64, height: i64) -> Result<Self, PointerError> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (width, height);
            Err(PointerError::NotLinux)
        }
        #[cfg(target_os = "linux")]
        {
            let file = open_uinput()?;
            let pointer = Self { file, width, height, last: (0, 0) };
            pointer.configure()?;
            Ok(pointer)
        }
    }

    /// The size of the screen this pointer reports.
    pub fn size(&self) -> (i64, i64) {
        (self.width, self.height)
    }

    /// The last position the pointer was put at.
    pub fn last_position(&self) -> (i64, i64) {
        self.last
    }

    /// Put the pointer at an absolute screen position.
    pub fn move_to(&mut self, x: i64, y: i64) -> Result<(), PointerError> {
        let x = x.clamp(0, (self.width - 1).max(0));
        let y = y.clamp(0, (self.height - 1).max(0));
        self.write(abi::EV_ABS, abi::ABS_X, x as i32)?;
        self.write(abi::EV_ABS, abi::ABS_Y, y as i32)?;
        self.sync()?;
        self.last = (x, y);
        Ok(())
    }

    /// Press a button.
    pub fn button_down(&mut self, button: &str) -> Result<(), PointerError> {
        let code = button_code(button);
        self.write(abi::EV_KEY, code, 1)?;
        self.sync()
    }

    /// Release a button.
    pub fn button_up(&mut self, button: &str) -> Result<(), PointerError> {
        let code = button_code(button);
        self.write(abi::EV_KEY, code, 0)?;
        self.sync()
    }

    /// Move to a position and click there.
    pub fn click(&mut self, x: i64, y: i64, button: &str) -> Result<(), PointerError> {
        self.move_to(x, y)?;
        // The pause matters: without it the press is delivered in the same
        // frame as the motion and lands at the previous position.
        std::thread::sleep(PRESS_SETTLE);
        self.button_down(button)?;
        std::thread::sleep(PRESS_HOLD);
        self.button_up(button)
    }

    /// Where the kernel says the pointer is.
    ///
    /// The value the device last reported, read back from the kernel: the
    /// truth of the device, not a copy of what this process believes it
    /// did.
    ///
    /// The read happens on the *event node* of the device, not on the
    /// uinput fd: the uinput fd only answers the `UI_*` ioctls and says
    /// the argument is wrong for anything else. The node is found by the
    /// name the device was created with, under
    /// `/sys/class/input/event*/device/name`.
    ///
    /// The node is created by the kernel with the mode of a device that
    /// only its owner may touch, and it is `udev` that opens it up to the
    /// `input` group a moment later. So the read waits for that: a
    /// failure here is a race, not a wrong answer, and the caller is told
    /// rather than given a number from somewhere else.
    pub fn position(&mut self) -> Result<(i64, i64), PointerError> {
        #[cfg(not(target_os = "linux"))]
        {
            Err(PointerError::NotLinux)
        }
        #[cfg(target_os = "linux")]
        {
            let mut reason = String::from("the event node of the pointer did not appear");
            for attempt in 0..POSITION_ATTEMPTS {
                match self.event_node() {
                    Ok(Some(node)) => match self.read_node(&node) {
                        Ok(position) => return Ok(position),
                        Err(error) => reason = error.to_string(),
                    },
                    Ok(None) => reason = "the event node of the pointer did not appear".to_string(),
                    Err(error) => reason = error.to_string(),
                }
                if attempt + 1 < POSITION_ATTEMPTS {
                    std::thread::sleep(POSITION_RETRY);
                }
            }
            Err(PointerError::Ioctl(0, reason))
        }
    }

    /// Read the two axes of an event node.
    #[cfg(target_os = "linux")]
    fn read_node(&self, node: &std::path::Path) -> Result<(i64, i64), PointerError> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(node)
            .map_err(|error| PointerError::CannotOpen(format!("{}: {error}", node.display())))?;
        use std::os::unix::io::AsRawFd;
        let fd = file.as_raw_fd();
        let x = read_axis(fd, abi::eviocgabs(abi::ABS_X as u8))?;
        let y = read_axis(fd, abi::eviocgabs(abi::ABS_Y as u8))?;
        Ok((x, y))
    }

    /// The event node of this device, if the kernel has made one.
    #[cfg(target_os = "linux")]
    fn event_node(&self) -> Result<Option<std::path::PathBuf>, PointerError> {
        let Ok(entries) = std::fs::read_dir("/sys/class/input") else {
            return Ok(None);
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("event") {
                continue;
            }
            let device_name = entry.path().join("device/name");
            let Ok(text) = std::fs::read_to_string(&device_name) else {
                continue;
            };
            if text.trim() != DEVICE_NAME {
                continue;
            }
            return Ok(Some(std::path::PathBuf::from(format!("/dev/input/{name}"))));
        }
        Ok(None)
    }

    fn write(&mut self, event_type: u16, code: u16, value: i32) -> Result<(), PointerError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let mut event = [0u8; 24];
        event[0..8].copy_from_slice(&(now.as_secs() as i64).to_ne_bytes());
        // The timeval's second field is MICROSECONDS.
        event[8..16].copy_from_slice(&(now.subsec_micros() as i64).to_ne_bytes());
        event[16..18].copy_from_slice(&event_type.to_ne_bytes());
        event[18..20].copy_from_slice(&code.to_ne_bytes());
        event[20..24].copy_from_slice(&value.to_ne_bytes());
        self.file
            .write_all(&event)
            .map_err(|error| PointerError::Ioctl(0, error.to_string()))
    }

    fn sync(&mut self) -> Result<(), PointerError> {
        self.write(abi::EV_SYN, abi::SYN_REPORT, 0)
    }

    #[cfg(target_os = "linux")]
    fn configure(&self) -> Result<(), PointerError> {
        use std::os::unix::io::AsRawFd;

        let fd = self.file.as_raw_fd();
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_KEY as libc::c_int)?;
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_ABS as libc::c_int)?;
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_SYN as libc::c_int)?;
        for code in [abi::BTN_LEFT, abi::BTN_RIGHT, abi::BTN_MIDDLE] {
            self.ioctl(abi::UI_SET_KEYBIT, code as libc::c_int)?;
        }
        for code in [abi::ABS_X, abi::ABS_Y] {
            self.ioctl(abi::UI_SET_ABSBIT, code as libc::c_int)?;
        }
        // This is the bit that makes libinput treat the node as a pointer.
        self.ioctl(abi::UI_SET_PROPBIT, abi::INPUT_PROP_DIRECT as libc::c_int)?;

        // struct uinput_setup { struct input_id id; char name[80]; __u32 ff_effects_max; }
        let mut setup = [0u8; 92];
        setup[0..2].copy_from_slice(&0x03u16.to_ne_bytes()); // BUS_USB
        setup[2..4].copy_from_slice(&0x1D6Bu16.to_ne_bytes()); // the generic vendor
        setup[4..6].copy_from_slice(&0x0002u16.to_ne_bytes());
        setup[6..8].copy_from_slice(&0x0001u16.to_ne_bytes());
        let name = DEVICE_NAME.as_bytes();
        setup[8..8 + name.len()].copy_from_slice(name);
        let result = unsafe { libc::ioctl(fd, abi::UI_DEV_SETUP, setup.as_ptr()) };
        if result < 0 {
            return Err(PointerError::Setup(std::io::Error::last_os_error().to_string()));
        }

        // struct uinput_abs_setup { __u16 code; __u16 filler; struct input_absinfo absinfo; }
        for (code, maximum) in [
            (abi::ABS_X, (self.width - 1).max(0) as i32),
            (abi::ABS_Y, (self.height - 1).max(0) as i32),
        ] {
            let mut abs_setup = [0u8; 28];
            abs_setup[0..2].copy_from_slice(&code.to_ne_bytes());
            // value, minimum, maximum, fuzz, flat, resolution
            abs_setup[4..8].copy_from_slice(&0i32.to_ne_bytes());
            abs_setup[8..12].copy_from_slice(&0i32.to_ne_bytes());
            abs_setup[12..16].copy_from_slice(&maximum.to_ne_bytes());
            unsafe { libc::ioctl(fd, abi::UI_ABS_SETUP, abs_setup.as_ptr()) };
        }

        self.ioctl(abi::UI_DEV_CREATE, 0)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn ioctl(&self, request: u64, argument: libc::c_int) -> Result<(), PointerError> {
        let fd = self.file.as_raw_fd();
        let result = unsafe { libc::ioctl(fd, request, argument) };
        if result < 0 {
            return Err(PointerError::Ioctl(request, std::io::Error::last_os_error().to_string()));
        }
        Ok(())
    }
}

/// The code of a mouse button.
fn button_code(button: &str) -> u16 {
    match button {
        "right" => abi::BTN_RIGHT,
        "middle" => abi::BTN_MIDDLE,
        _ => abi::BTN_LEFT,
    }
}

/// Read one absolute axis, through `EVIOCGABS`.
#[cfg(target_os = "linux")]
fn read_axis(fd: libc::c_int, request: u64) -> Result<i64, PointerError> {
    // struct input_absinfo { __s32 value; __s32 minimum; __s32 maximum; __s32 fuzz; __s32 flat; __s32 resolution; }
    let mut info = [0i32; 6];
    let result = unsafe { libc::ioctl(fd, request, info.as_mut_ptr()) };
    if result < 0 {
        return Err(PointerError::Ioctl(request, std::io::Error::last_os_error().to_string()));
    }
    Ok(info[0] as i64)
}

#[cfg(target_os = "linux")]
fn open_uinput() -> Result<File, PointerError> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open("/dev/uinput")
        .map_err(|error| PointerError::CannotOpen(error.to_string()))
}
