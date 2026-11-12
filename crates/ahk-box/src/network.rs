//! The network of the box.
//!
//! The box has curl, and curl is the way out to the network: it fetches
//! a file and writes it on the disk. The addresses of the machine are read
//! from the interfaces themselves, with `getifaddrs`.

use std::process::Command;

/// The family of an address of the Internet Protocol, version four.
///
/// The value is the one of the C library of Linux, and the same on every
/// architecture the box is built for.
#[cfg(unix)]
const AF_INET: u16 = 2;

/// The addresses of the machine, one for each interface address.
///
/// The shape is the one of the binary of v2.0.19, measured on 2026-11-09:
/// `SysGetIPAddresses` answers an array of the addresses of the machine, the
/// addresses of the loopback are not among them, and the addresses are the
/// ones of the version four -- the binary answered three addresses and none
/// of them was 127.0.0.1.
///
/// The reading is `getifaddrs`: the call of the C library that walks the
/// interfaces of the machine. The structures below are the ones the call
/// publishes, declared by hand the way this crate declares the calls it
/// needs: the language crate does the same, and the surface is small enough
/// to pin.
pub fn ip_addresses() -> Result<Vec<String>, String> {
    #[cfg(unix)]
    {
        let mut list: *mut IfAddrs = std::ptr::null_mut();
        // The call answers zero when it filled the list, and minus one with
        // the reason in the error of the system when it did not.
        if unsafe { getifaddrs(&mut list) } != 0 {
            return Err(the_message_of(&std::io::Error::last_os_error()));
        }
        let mut addresses: Vec<String> = Vec::new();
        let mut entry = list;
        while !entry.is_null() {
            // The interface, its address and the next one are the fields of
            // the block the call published. The address is the null pointer
            // for an interface that has none.
            let address = unsafe { (*entry).ifa_addr };
            if !address.is_null() && unsafe { (*address).sa_family } == AF_INET {
                // A sockaddr of the version four is the family, the port and
                // the four bytes of the address: the bytes start at the
                // fourth one of the block.
                let bytes = unsafe { std::slice::from_raw_parts((address as *const u8).add(4), 4) };
                let text = format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]);
                // The loopback is the machine talking to itself, and the
                // binary does not answer it.
                if !is_loopback(text.as_str()) && !addresses.contains(&text) {
                    addresses.push(text);
                }
            }
            entry = unsafe { (*entry).ifa_next };
        }
        unsafe { freeifaddrs(list) };
        Ok(addresses)
    }
    #[cfg(not(unix))]
    {
        Err(
            "the addresses of this machine are read with getifaddrs, and this machine has none"
                .to_string(),
        )
    }
}

/// Whether an address is one of the loopback range, 127.0.0.0/8.
///
/// The reading of the interfaces is the one of a Unix machine, and the range
/// is a question about the addresses of any machine: the test of it runs
/// everywhere.
#[cfg(any(unix, test))]
fn is_loopback(address: &str) -> bool {
    address.starts_with("127.")
}

/// The message of a failure of the machine, the way the tree says it: the
/// code of the system in front of what it says.
#[cfg(unix)]
fn the_message_of(error: &std::io::Error) -> String {
    let text = error.to_string();
    let text = text.split(" (os error ").next().unwrap_or(&text);
    match error.raw_os_error() {
        Some(code) => format!("({code}) {text}"),
        None => text.to_string(),
    }
}

/// The block of the interfaces that `getifaddrs` publishes.
///
/// The fields are the ones of the C structure, in their order: the next
/// interface, its name, its flags, its address, its netmask, the address of
/// the broadcast or of the destination of a point-to-point interface, and
/// the data of the interface. Only the address of the address and the next
/// entry are read: the others have to be there all the same, because they
/// are what puts the address at the place the call wrote it.
#[cfg(unix)]
#[repr(C)]
struct IfAddrs {
    /// The next interface of the list.
    ifa_next: *mut IfAddrs,
    /// The name of the interface.
    ifa_name: *mut std::ffi::c_char,
    /// The flags of the interface.
    ifa_flags: std::ffi::c_uint,
    /// The address of the interface.
    ifa_addr: *mut SockAddr,
    /// The netmask of the interface.
    ifa_netmask: *mut SockAddr,
    /// The broadcast address, or the destination of a point-to-point one.
    ifa_ifu: *mut SockAddr,
    /// The data of the interface.
    ifa_data: *mut std::ffi::c_void,
}

/// The family of an address: the first field of every address of the C
/// library.
#[cfg(unix)]
#[repr(C)]
struct SockAddr {
    /// The family of the address: `AF_INET` is the version four.
    sa_family: u16,
    /// The bytes of the address, as the family means them.
    sa_data: [u8; 14],
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `getifaddrs` the C library gives.
    #[link_name = "getifaddrs"]
    fn getifaddrs(interfaces: *mut *mut IfAddrs) -> std::ffi::c_int;

    /// The declaration of `freeifaddrs` the C library gives.
    #[link_name = "freeifaddrs"]
    fn freeifaddrs(interfaces: *mut IfAddrs);
}

/// Fetch a file of the network and write it on the disk.
///
/// The answer is the path of the file that was written.
///
/// A file that is not there on the network is not a failure: the server
/// sends its page of the error, and that page is what is written -- the
/// same way v2 does it. A url that cannot be reached at all is a
/// failure, and the code of the machine is in front of what it says.
pub fn download(url: &str, file: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .arg("--silent")
        .arg("--show-error")
        .arg("--location")
        .arg("--output")
        .arg(file)
        .arg(url)
        .output()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                "curl is not on this machine, and the box has no other way to the network"
                    .to_string()
            } else {
                format!("the download failed: {error}")
            }
        })?;
    if !output.status.success() {
        let code = output.status.code().unwrap_or(0);
        let reason = String::from_utf8_lossy(&output.stderr);
        let reason = reason.trim();
        // Curl writes the code of the failure in front of what it says,
        // and the message of the tree carries the code once.
        let reason = reason.strip_prefix(&format!("({code}) ")).unwrap_or(reason);
        return Err(format!("({code}) {reason}"));
    }
    Ok(file.to_string())
}

#[cfg(test)]
mod tests {
    use super::is_loopback;

    /// The loopback range is the one of 127.0.0.0/8, and the range of a
    /// network of the machine is not one of it.
    #[test]
    fn the_loopback_range_is_not_an_address_of_the_machine() {
        assert!(is_loopback("127.0.0.1"));
        assert!(is_loopback("127.9.9.9"));
        assert!(!is_loopback("192.168.0.1"));
        assert!(!is_loopback("10.0.0.1"));
    }

    /// Every address of the machine is an address of the version four, and
    /// none of them is the loopback: the shape is the one the binary of
    /// v2.0.19 answers, measured on 2026-11-09.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_addresses_of_the_machine_are_the_ones_of_v2() {
        let addresses = super::ip_addresses().expect("the interfaces are read");
        for address in &addresses {
            assert!(!is_loopback(address), "{address}");
            let parts: Vec<&str> = address.split('.').collect();
            assert_eq!(parts.len(), 4, "{address}");
            for part in parts {
                assert!(part.parse::<u8>().is_ok(), "{address}");
            }
        }
        // The box is on a network of the LAN: measured on it on 2026-11-09,
        // it had an address, and the loopback was not one of them.
        assert!(!addresses.is_empty(), "the box has an address of its own");
    }
}
