//! The calls into the C library of the machine, declared here.
//!
//! The box opens `/dev/uinput`, asks the kernel what it is, and asks the
//! system who the user is. Each of those is a call into the C library of the
//! platform, and this module is the whole of what the tree needs from it: the
//! seven names the uinput path uses, each with the signature the platform
//! gives it, and the answer the other platforms give for the same question.
//!
//! The declarations are here rather than in a crate of their own for the same
//! reason `x11.rs` declares `dlopen` and `dlsym`: the surface the tree touches
//! is small, it is pinned by the case beside each call, and a crate that
//! carried the whole of the C library for seven names would be a crate more to
//! trust for nothing.
//!
//! # The platforms
//!
//! The three calls exist on Linux, and the tree calls them there. On a
//! platform that has no such call -- Windows, which names the user another
//! way -- the function answers what the platform means by the same question:
//! the user is the one of the environment, and the name of the machine is the
//! one the system reports. The device path is Linux-only anyway: a script on
//! another platform is told that the keyboard is a Linux device, and none of
//! this is reached.

#[cfg(unix)]
use std::ffi::c_char;

use std::ffi::c_int;

/// The flags of an `open` that does not wait: the call returns at once, with
/// the reason, instead of blocking the thread that asked.
///
/// The value is the one the Linux kernel fixes, and it is the same on every
/// architecture the tree is built for.
#[cfg(unix)]
pub const O_NONBLOCK: c_int = 0o4000;

/// The flags of an `open` that does not wait, on a platform that has no such
/// call. Nothing reads it there: the device path is Linux-only.
#[cfg(not(unix))]
pub const O_NONBLOCK: c_int = 0;

/// The whole number a C library takes for an `int`.
pub use std::ffi::c_int as Int;

/// The whole number of the user the process runs as.
#[cfg(unix)]
pub fn getuid() -> u32 {
    // The call cannot fail, and it answers the same thing every time.
    unsafe { getuid_() }
}

/// The whole number of the user the process runs as.
///
/// A platform that has no such call answers with the user of the environment,
/// which is the name the system gave the process.
#[cfg(not(unix))]
pub fn getuid() -> u32 {
    let name = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_default();
    the_number_of_a_name(&name)
}

/// The whole number of the effective user the process runs as.
#[cfg(unix)]
pub fn geteuid() -> u32 {
    unsafe { geteuid_() }
}

/// The whole number of the effective user the process runs as.
#[cfg(not(unix))]
pub fn geteuid() -> u32 {
    getuid()
}

/// The name of the machine, written into `buffer`.
///
/// The answer is what the call answered: zero when the name fits, and a
/// number that is not zero when it did not.
#[cfg(unix)]
pub fn gethostname(buffer: &mut [u8]) -> i32 {
    // The call wants the pointer to the bytes and how many of them there are.
    unsafe { gethostname_(buffer.as_mut_ptr() as *mut c_char, buffer.len()) }
}

/// The name of the machine, written into `buffer`.
///
/// A platform that has no such call takes the name the environment carries,
/// which is the one the system gave the process.
#[cfg(not(unix))]
pub fn gethostname(buffer: &mut [u8]) -> i32 {
    let name = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_default();
    let bytes = name.as_bytes();
    let count = bytes.len().min(buffer.len().saturating_sub(1));
    buffer[..count].copy_from_slice(&bytes[..count]);
    if count < buffer.len() {
        buffer[count] = 0;
    }
    0
}

/// The call of the kernel that a device driver answers.
///
/// The request is the number the kernel fixes for the call -- the `_IOW` and
/// `_IOR` of the format of the kernel -- and the argument is what the request
/// carries: a number, or the address of the block the driver reads or writes.
///
/// # Safety
///
/// The request must be one the driver of `fd` answers, and the argument must
/// be what that request carries: a number, or a pointer to a block that is at
/// least as large as the request says. A request the driver does not know
/// fails with `EINVAL` and touches nothing; a pointer of the wrong shape is
/// undefined behaviour in the driver, and no check here can see it.
#[cfg(unix)]
pub unsafe fn ioctl(fd: c_int, request: u64, argument: *mut std::ffi::c_void) -> c_int {
    unsafe { ioctl_(fd, request, argument) }
}

/// The call of the kernel that a device driver answers, with the argument as
/// a number.
///
/// # Safety
///
/// The request must be one the driver of `fd` answers with a number.
#[cfg(unix)]
pub unsafe fn ioctl_number(fd: c_int, request: u64, argument: c_int) -> c_int {
    unsafe { ioctl_(fd, request, argument as *mut std::ffi::c_void) }
}

/// The call of the kernel that a device driver answers.
///
/// A platform that has none fails the call, which is what the caller sees as
/// the reason the device is not there.
///
/// # Safety
///
/// Nothing is reached on such a platform, so there is nothing the caller can
/// do to make the call wrong.
#[cfg(not(unix))]
pub unsafe fn ioctl(_fd: c_int, _request: u64, _argument: *mut std::ffi::c_void) -> c_int {
    -1
}

/// The call of the kernel that a device driver answers, with the argument as
/// a number.
///
/// # Safety
///
/// Nothing is reached on such a platform, so there is nothing the caller can
/// do to make the call wrong.
#[cfg(not(unix))]
pub unsafe fn ioctl_number(_fd: c_int, _request: u64, _argument: c_int) -> c_int {
    -1
}

/// The number a name answers with, so that the two readings of the user of a
/// platform that has no `getuid` are the same for the same name.
#[cfg(not(unix))]
fn the_number_of_a_name(name: &str) -> u32 {
    // The sum of the bytes is the number: it is the same for the same name,
    // and it is not an account number of the machine.
    name.bytes().fold(0u32, |total, byte| total.wrapping_mul(31).wrapping_add(u32::from(byte)))
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `getuid` the C library gives.
    #[link_name = "getuid"]
    fn getuid_() -> u32;

    /// The declaration of `geteuid` the C library gives.
    #[link_name = "geteuid"]
    fn geteuid_() -> u32;

    /// The declaration of `gethostname` the C library gives.
    #[link_name = "gethostname"]
    fn gethostname_(name: *mut c_char, length: usize) -> i32;

    /// The declaration of `ioctl` the C library gives.
    #[link_name = "ioctl"]
    fn ioctl_(fd: c_int, request: u64, argument: *mut std::ffi::c_void) -> c_int;

    /// The declaration of `kill` the C library gives.
    #[link_name = "kill"]
    fn kill_(pid: c_int, signal: c_int) -> c_int;

    /// The declaration of `setpriority` the C library gives.
    ///
    /// The `which` is the process, which is the zero of `PRIO_PROCESS`, and
    /// the `who` is the id of the process.
    #[link_name = "setpriority"]
    fn setpriority_(which: c_int, who: u32, priority: c_int) -> c_int;
}

/// The signal that asks a process to end.
///
/// The number is the one Linux fixes for `SIGTERM`, and it is the same on
/// every architecture the tree is built for.
#[cfg(unix)]
pub const SIGTERM: c_int = 15;

/// Ask a process to end, and answer what the call answered.
///
/// The call answers the zero when the signal was delivered to a process that
/// is there, and the minus one when it was not -- a process that is not
/// there, or one that belongs to another user.
#[cfg(unix)]
pub fn kill(pid: u32, signal: c_int) -> c_int {
    // The pid of a process is the whole number the C library takes, and the
    // caller has already checked that it is a process of this machine.
    unsafe { kill_(pid as c_int, signal) }
}

/// Ask a process to end, on a platform that has no such call.
#[cfg(not(unix))]
pub fn kill(_pid: u32, _signal: c_int) -> c_int {
    -1
}

/// The signal that asks a process to end.
///
/// The number is the one Linux fixes for `SIGTERM`, and it is the same on
/// every architecture the tree is built for. A platform that has no such
/// call has no signal either, and nothing reads it there: the process path
/// answers that it cannot look before the signal is reached.
#[cfg(not(unix))]
pub const SIGTERM: c_int = 15;

/// Set the niceness of a process.
///
/// The lower the number, the more of the processor the process is given. The
/// call answers the zero when the setting was taken, and the minus one when
/// it was not -- raising the priority above the ordinary one is the business
/// of the user who owns the process, and the machine refuses it to everyone
/// else.
#[cfg(unix)]
pub fn setpriority(pid: u32, priority: c_int) -> c_int {
    // `PRIO_PROCESS` is the zero, and it says that the `who` is a pid.
    unsafe { setpriority_(0, pid, priority) }
}

/// Set the niceness of a process, on a platform that has no such call.
#[cfg(not(unix))]
pub fn setpriority(_pid: u32, _priority: c_int) -> c_int {
    -1
}

#[cfg(test)]
mod tests {
    use super::{geteuid, getuid};
    #[cfg(unix)]
    use super::gethostname;

    /// The user of the process is a number, and the two readings of it agree
    /// on a machine that does not switch the effective one.
    #[test]
    fn the_user_of_the_process_has_a_number() {
        // The real user and the effective one are the same for a process
        // that was not made set-uid, which a test process is not.
        assert_eq!(getuid(), geteuid());
    }

    /// The name of the machine is a text, and a buffer that is large enough
    /// holds it.
    #[cfg(unix)]
    #[test]
    fn the_machine_has_a_name() {
        let mut buffer = [0u8; 256];
        assert_eq!(gethostname(&mut buffer), 0);
        // The name is what the call wrote, and it is not empty.
        let length = buffer.iter().position(|byte| *byte == 0).unwrap_or(0);
        assert!(length > 0, "the machine has no name");
    }

    /// The flag of a call that does not wait is the one the kernel fixes.
    #[cfg(unix)]
    #[test]
    fn the_flag_of_the_call_that_does_not_wait_is_the_one_of_the_kernel() {
        // The value is `0o4000` on every architecture, and a mistake in it is
        // a call that blocks instead of one that returns.
        assert_eq!(super::O_NONBLOCK, 0o4000);
    }
}