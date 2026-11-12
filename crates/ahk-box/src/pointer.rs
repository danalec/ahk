//! An absolute pointer device, for driving a Wayland session exactly.
//!
//! A port of `src/ahk/pointer.py` from the archived Python implementation.
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
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use core::sync::atomic::{AtomicI64, Ordering};

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
    pub const UI_DEV_SETUP: u64 = iow(3, UINPUT_SETUP_SIZE);
    pub const UI_ABS_SETUP: u64 = iow(4, UINPUT_ABS_SETUP_SIZE);
    pub const UI_DEV_CREATE: u64 = io(1);

    /// `struct uinput_setup`: four __u16, a name of eighty bytes, a __u32.
    pub const UINPUT_SETUP_SIZE: usize = 4 * 2 + 80 + 4;
    /// `struct uinput_abs_setup`: a __u16, a filler, six __s32.
    pub const UINPUT_ABS_SETUP_SIZE: usize = 2 * 2 + 6 * 4;
    /// `struct input_event`: two longs, two shorts, an int.
    pub const INPUT_EVENT_SIZE: usize = 2 * 8 + 2 * 2 + 4;

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
pub const DEVICE_NAME: &str = "ahk absolute pointer";

/// How many times the read of the event node is tried.
///
/// The node is created by the kernel with the mode of a device that only
/// its owner may touch, and `udev` opens it up to the `input` group a
/// moment later. The read waits for that.
pub const POSITION_ATTEMPTS: usize = 20;

/// The gap between the tries.
pub const POSITION_RETRY: Duration = Duration::from_millis(100);

/// The speed of the pointer: the number of steps a movement is made of.
///
/// The speed of v2 is from 0 to 100, and 0 moves the pointer at once:
/// a movement of one step is the movement itself. The default is the
/// default of v2, which is 2.
///
/// The state is here and not in the device because the device is opened
/// on the first movement, and the speed may be set before it is.
static SPEED: AtomicI64 = AtomicI64::new(2);

/// The pause between two movements of the pointer, in milliseconds.
///
/// The delay of v2 is -1 when there is no pause at all, and 0 when the
/// pause is the smallest there is. The default is the default of v2,
/// which is 10.
static DELAY: AtomicI64 = AtomicI64::new(10);

/// The number of steps a movement is made of.
///
/// The speed of v2 is the number of steps, and a speed of 0 is a
/// movement at once: one step.
pub fn steps(speed: i64) -> usize {
    speed.max(1) as usize
}

/// The place of a step of a movement.
///
/// The movement from one place to another is made of `steps` steps, and
/// the last of them is the place itself.
fn step_of(from: i64, to: i64, index: usize, steps: usize) -> i64 {
    from + (to - from) * index as i64 / steps as i64
}

/// The pointer device could not be created or driven.
#[derive(Debug)]
pub enum PointerError {
    NotLinux,
    CannotOpen(String),
    Ioctl(u64, String),
    Setup(String),
    Closed,
}

impl std::fmt::Display for PointerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointerError::NotLinux => {
                write!(
                    formatter,
                    "the absolute pointer is a Linux uinput device; this is not Linux"
                )
            }
            PointerError::CannotOpen(reason) => write!(
                formatter,
                "cannot open /dev/uinput: {reason}. The user needs to be in the `input` group."
            ),
            PointerError::Ioctl(request, reason) => {
                write!(formatter, "ioctl 0x{request:x} failed: {reason}")
            }
            PointerError::Setup(reason) => write!(formatter, "UI_DEV_SETUP failed: {reason}"),
            PointerError::Closed => write!(formatter, "the pointer is not open"),
        }
    }
}

impl std::error::Error for PointerError {}

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
            let pointer = Self {
                file,
                width,
                height,
                last: (0, 0),
            };
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
    ///
    /// The movement is made of the steps of the speed, and the pause
    /// between two movements is the delay.
    pub fn move_to(&mut self, x: i64, y: i64) -> Result<(), PointerError> {
        self.move_in_steps(x, y)?;
        self.pause();
        Ok(())
    }

    /// Put the pointer at an absolute screen position, in steps.
    ///
    /// The speed of v2 is the number of steps the movement is made of,
    /// and a speed of 0 is a movement at once.
    fn move_in_steps(&mut self, x: i64, y: i64) -> Result<(), PointerError> {
        let (x, y) = clamp_to_screen(x, y, self.width, self.height);
        let (from_x, from_y) = self.last;
        let steps = steps(SPEED.load(Ordering::Relaxed));
        for index in 1..=steps {
            let step_x = step_of(from_x, x, index, steps);
            let step_y = step_of(from_y, y, index, steps);
            self.write(abi::EV_ABS, abi::ABS_X, step_x as i32)?;
            self.write(abi::EV_ABS, abi::ABS_Y, step_y as i32)?;
            self.sync()?;
        }
        self.last = (x, y);
        Ok(())
    }

    /// The pause between two movements of the pointer.
    ///
    /// A delay of -1 is no pause at all, and a delay of 0 is the
    /// smallest pause there is: the box has no way of yielding, so it
    /// is no pause.
    fn pause(&self) {
        let delay = DELAY.load(Ordering::Relaxed);
        if delay > 0 {
            std::thread::sleep(Duration::from_millis(delay as u64));
        }
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
        self.move_in_steps(x, y)?;
        // The pause matters: without it the press is delivered in the same
        // frame as the motion and lands at the previous position.
        std::thread::sleep(PRESS_SETTLE);
        self.button_down(button)?;
        std::thread::sleep(PRESS_HOLD);
        self.button_up(button)?;
        // The delay of v2 is after the click, the way it is after a
        // movement.
        self.pause();
        Ok(())
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
        let mut event = [0u8; abi::INPUT_EVENT_SIZE];
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
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_KEY as crate::sys::Int)?;
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_ABS as crate::sys::Int)?;
        self.ioctl(abi::UI_SET_EVBIT, abi::EV_SYN as crate::sys::Int)?;
        for code in [abi::BTN_LEFT, abi::BTN_RIGHT, abi::BTN_MIDDLE] {
            self.ioctl(abi::UI_SET_KEYBIT, code as crate::sys::Int)?;
        }
        for code in [abi::ABS_X, abi::ABS_Y] {
            self.ioctl(abi::UI_SET_ABSBIT, code as crate::sys::Int)?;
        }
        // This is the bit that makes libinput treat the node as a pointer.
        self.ioctl(
            abi::UI_SET_PROPBIT,
            abi::INPUT_PROP_DIRECT as crate::sys::Int,
        )?;

        // struct uinput_setup { struct input_id id; char name[80]; __u32 ff_effects_max; }
        let mut setup = [0u8; abi::UINPUT_SETUP_SIZE];
        setup[0..2].copy_from_slice(&0x03u16.to_ne_bytes()); // BUS_USB
        setup[2..4].copy_from_slice(&0x1D6Bu16.to_ne_bytes()); // the generic vendor
        setup[4..6].copy_from_slice(&0x0002u16.to_ne_bytes());
        setup[6..8].copy_from_slice(&0x0001u16.to_ne_bytes());
        let name = DEVICE_NAME.as_bytes();
        setup[8..8 + name.len()].copy_from_slice(name);
        let result = unsafe { crate::sys::ioctl(fd, abi::UI_DEV_SETUP, setup.as_ptr() as *mut _) };
        if result < 0 {
            return Err(PointerError::Setup(
                std::io::Error::last_os_error().to_string(),
            ));
        }

        // struct uinput_abs_setup { __u16 code; __u16 filler; struct input_absinfo absinfo; }
        for (code, maximum) in [
            (abi::ABS_X, (self.width - 1).max(0) as i32),
            (abi::ABS_Y, (self.height - 1).max(0) as i32),
        ] {
            let mut abs_setup = [0u8; abi::UINPUT_ABS_SETUP_SIZE];
            abs_setup[0..2].copy_from_slice(&code.to_ne_bytes());
            // value, minimum, maximum, fuzz, flat, resolution
            abs_setup[4..8].copy_from_slice(&0i32.to_ne_bytes());
            abs_setup[8..12].copy_from_slice(&0i32.to_ne_bytes());
            abs_setup[12..16].copy_from_slice(&maximum.to_ne_bytes());
            unsafe { crate::sys::ioctl(fd, abi::UI_ABS_SETUP, abs_setup.as_ptr() as *mut _) };
        }

        self.ioctl(abi::UI_DEV_CREATE, 0)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn ioctl(&self, request: u64, argument: crate::sys::Int) -> Result<(), PointerError> {
        let fd = self.file.as_raw_fd();
        let result = unsafe { crate::sys::ioctl_number(fd, request, argument) };
        if result < 0 {
            return Err(PointerError::Ioctl(
                request,
                std::io::Error::last_os_error().to_string(),
            ));
        }
        Ok(())
    }
}

/// The place a click at (x, y) really lands on.
///
/// The clamp is what keeps a stale or silly coordinate from wrapping around
/// to the far corner of the screen.
fn clamp_to_screen(x: i64, y: i64, width: i64, height: i64) -> (i64, i64) {
    (
        x.clamp(0, (width - 1).max(0)),
        y.clamp(0, (height - 1).max(0)),
    )
}

/// The speed of the pointer of the box.
///
/// The speed of v2 is the number of steps a movement is made of, and a
/// speed of 0 is a movement at once.
///
/// The state is here and not in the module of the desktop because the
/// pointer of the box is where the movements are made, and the speed
/// belongs with them. The impl of the trait of the pointer is in the
/// module of the desktop, and it calls this setter.
pub fn set_speed(speed: i64) {
    SPEED.store(speed, Ordering::Relaxed);
}

/// The pause between two movements of the pointer of the box.
///
/// The delay of v2 is -1 when there is no pause at all, and 0 when the
/// pause is the smallest there is.
pub fn set_delay(delay: i64) {
    DELAY.store(delay, Ordering::Relaxed);
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
fn read_axis(fd: crate::sys::Int, request: u64) -> Result<i64, PointerError> {
    // struct input_absinfo { __s32 value; __s32 minimum; __s32 maximum; __s32 fuzz; __s32 flat; __s32 resolution; }
    let mut info = [0i32; 6];
    let result = unsafe { crate::sys::ioctl(fd, request, info.as_mut_ptr() as *mut _) };
    if result < 0 {
        return Err(PointerError::Ioctl(
            request,
            std::io::Error::last_os_error().to_string(),
        ));
    }
    Ok(info[0] as i64)
}

#[cfg(target_os = "linux")]
fn open_uinput() -> Result<File, PointerError> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .write(true)
        .custom_flags(crate::sys::O_NONBLOCK)
        .open("/dev/uinput")
        .map_err(|error| PointerError::CannotOpen(error.to_string()))
}

#[cfg(test)]
mod tests {
    //! The numbers the kernel sees.
    //!
    //! The uinput ABI is the part of this module that fails silently: a
    //! struct laid out in the wrong order, or an ioctl number built with
    //! `_IO` instead of `_IOW`, gives `EINVAL` from the kernel and nothing
    //! else -- no traceback pointing at the packing. So the numbers are
    //! pinned here against the C definitions, and a mistake is caught on
    //! this machine and not on the box.
    //!
    //! The C definitions live in `/usr/include/linux/uinput.h`:
    //!
    //! ```c
    //! #define UI_DEV_CREATE       _IO(UINPUT_IOCTL_BASE, 1)
    //! #define UI_DEV_SETUP        _IOW(UINPUT_IOCTL_BASE, 3, struct uinput_setup)
    //! #define UI_ABS_SETUP        _IOW(UINPUT_IOCTL_BASE, 4, struct uinput_abs_setup)
    //! #define UI_SET_EVBIT        _IOW(UINPUT_IOCTL_BASE, 100, int)
    //! ```
    //!
    //! The archive has one case that has no counterpart here: a closed
    //! pointer refuses to move. In Rust the type cannot be in that state,
    //! because it holds the file of the device.

    use super::abi;
    use super::clamp_to_screen;
    use super::{step_of, steps};
    // The pointer itself is only used by the case that is not Linux: on
    // Linux the device is opened, and there is no refusal to check.
    #[cfg(not(target_os = "linux"))]
    use super::{Pointer, PointerError};

    /// `_IO('U', nr)`: the direction and the size are both zero.
    fn io(nr: u8) -> u64 {
        (abi::UINPUT_IOCTL_BASE as u64) << 8 | nr as u64
    }

    /// `_IOW('U', nr, size)`: the size is part of the request number.
    fn iow(nr: u8, size: usize) -> u64 {
        1u64 << 30 | (size as u64) << 16 | (abi::UINPUT_IOCTL_BASE as u64) << 8 | nr as u64
    }

    #[test]
    fn the_dev_create_request_is_io_1() {
        assert_eq!(abi::UI_DEV_CREATE, io(1));
    }

    #[test]
    fn the_setup_request_carries_the_size_of_the_struct() {
        // `_IO` instead of `_IOW` here is what produced EINVAL on the box:
        // the request number has to encode the size of the struct it carries.
        assert_eq!(abi::UI_DEV_SETUP, iow(3, 92));
        assert_ne!(abi::UI_DEV_SETUP, io(3));
    }

    #[test]
    fn the_abs_setup_request_carries_the_size_of_the_struct() {
        assert_eq!(abi::UI_ABS_SETUP, iow(4, 28));
        assert_ne!(abi::UI_ABS_SETUP, io(4));
    }

    #[test]
    fn the_bit_operations_take_an_int() {
        for request in [
            abi::UI_SET_EVBIT,
            abi::UI_SET_KEYBIT,
            abi::UI_SET_ABSBIT,
            abi::UI_SET_PROPBIT,
        ] {
            assert_eq!(request, iow((request & 0xFF) as u8, 4));
        }
    }

    #[test]
    fn the_prop_code_is_a_capability_index() {
        // `INPUT_PROP_*` are indices, not masks. The kernel shifts the index
        // into the device's bitmap, so the index 1 -- INPUT_PROP_DIRECT --
        // shows up as `B: PROP=2`, and the index 2 -- INPUT_PROP_BUTTONPAD
        // -- as `B: PROP=4`, which is not a pointer at all.
        assert_eq!(abi::INPUT_PROP_DIRECT, 1);
    }

    #[test]
    fn the_structs_are_the_size_the_kernel_expects() {
        // struct uinput_setup { struct input_id id; char name[80]; __u32
        // ff_effects_max; }
        assert_eq!(abi::UINPUT_SETUP_SIZE, 4 * 2 + 80 + 4);
        // struct uinput_abs_setup { __u16 code; __u16 filler; struct
        // input_absinfo absinfo; }
        assert_eq!(abi::UINPUT_ABS_SETUP_SIZE, 2 * 2 + 6 * 4);
        // struct input_event { struct timeval time; __u16 type; __u16 code;
        // __s32 value; }
        assert_eq!(abi::INPUT_EVENT_SIZE, 2 * 8 + 2 * 2 + 4);
    }

    #[test]
    fn the_button_codes_and_the_axes_are_the_ones_the_kernel_uses() {
        assert_eq!(abi::BTN_LEFT, 0x110);
        assert_eq!(abi::BTN_RIGHT, 0x111);
        assert_eq!(abi::BTN_MIDDLE, 0x112);
        assert_eq!(abi::ABS_X, 0);
        assert_eq!(abi::ABS_Y, 1);
    }

    #[test]
    fn the_coordinates_are_clamped_to_the_screen() {
        assert_eq!(clamp_to_screen(5000, 5000, 1920, 1080), (1919, 1079));
        assert_eq!(clamp_to_screen(-40, -40, 1920, 1080), (0, 0));
        assert_eq!(clamp_to_screen(10, 10, 1920, 1080), (10, 10));
    }

    #[test]
    fn the_speed_is_the_number_of_steps_of_a_movement() {
        // The speed of v2 is from 0 to 100, and 0 moves the pointer at
        // once: one step is the movement itself.
        assert_eq!(steps(0), 1);
        assert_eq!(steps(1), 1);
        assert_eq!(steps(2), 2);
        assert_eq!(steps(20), 20);
        assert_eq!(steps(100), 100);
    }

    #[test]
    fn the_steps_of_a_movement_end_at_the_place() {
        assert_eq!(step_of(0, 100, 1, 4), 25);
        assert_eq!(step_of(0, 100, 2, 4), 50);
        assert_eq!(step_of(0, 100, 4, 4), 100);
        // The movement the other way round.
        assert_eq!(step_of(100, 0, 1, 4), 75);
        assert_eq!(step_of(100, 0, 4, 4), 0);
        // One step is the movement itself.
        assert_eq!(step_of(10, 90, 1, 1), 90);
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn it_refuses_off_linux() {
        let error = Pointer::open(1920, 1080).err().expect("it refuses");
        assert!(matches!(error, PointerError::NotLinux));
        assert!(error.to_string().contains("Linux"));
    }
}
