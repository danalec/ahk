//! The network of the machine.
//!
//! The addresses of the machine are read with the API of the network of
//! Windows: `GetAdaptersAddresses` walks the adapters of the machine
//! and the addresses of them, and the loopback is not among them.
//!
//! The way out to the network is `curl`: the machine has had it since
//! Windows 10, and it is the way the download of a file is made.

use std::process::Command;

use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetAdaptersAddresses, IP_ADAPTER_ADDRESSES_LH, IP_ADAPTER_UNICAST_ADDRESS_LH,
};
use windows_sys::Win32::Networking::WinSock::{AF_INET, AF_UNSPEC, SOCKADDR_IN};

/// The addresses of the machine, one for each adapter.
///
/// The shape is the one of the binary of v2.0.19, measured:
/// `SysGetIPAddresses` answers an array of the addresses of the machine,
/// the addresses of the loopback are not among them, and the addresses
/// are the ones of the version four.
pub fn ip_addresses() -> Result<Vec<String>, String> {
    let mut size = 0u32;
    // The first call asks the size of the list of the adapters: it
    // answers the error of the machine, and the size in the number.
    unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            Default::default(),
            std::ptr::null(),
            std::ptr::null_mut(),
            &mut size,
        );
    }
    if size == 0 {
        return Err("the machine did not say the adapters of its network.".to_string());
    }
    let mut buffer = vec![0u8; size as usize];
    let adapters = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;
    let answer = unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            Default::default(),
            std::ptr::null(),
            adapters,
            &mut size,
        )
    };
    if answer != 0 {
        return Err(format!(
            "the machine did not give the adapters of its network: the code is {answer}."
        ));
    }
    let mut addresses: Vec<String> = Vec::new();
    let mut adapter = adapters;
    while !adapter.is_null() {
        let mut unicast = unsafe { (*adapter).FirstUnicastAddress };
        while !unicast.is_null() {
            if let Some(address) = the_address_of(unsafe { &*unicast }) {
                if !addresses.contains(&address) {
                    addresses.push(address);
                }
            }
            unicast = unsafe { (*unicast).Next };
        }
        adapter = unsafe { (*adapter).Next };
    }
    Ok(addresses)
}

/// The address of one of the unicast addresses of an adapter.
///
/// The address is a socket address, and the one of the version four has
/// the four bytes of it after the port.
fn the_address_of(unicast: &IP_ADAPTER_UNICAST_ADDRESS_LH) -> Option<String> {
    let address = unicast.Address.lpSockaddr;
    if address.is_null() {
        return None;
    }
    let family = unsafe { (*address).sa_family };
    if family != AF_INET {
        return None;
    }
    let bytes = unsafe {
        let sin = address as *const SOCKADDR_IN;
        let union = &(*sin).sin_addr.S_un;
        let bytes = &union.S_un_b;
        [bytes.s_b1, bytes.s_b2, bytes.s_b3, bytes.s_b4]
    };
    let text = format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]);
    // The loopback is the machine talking to itself, and the binary does
    // not answer it.
    if text.starts_with("127.") {
        return None;
    }
    Some(text)
}

/// A file of the network is fetched and written on the disk.
///
/// The way out is `curl`, the tool that the machine has.
pub fn download(url: &str, file: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-o")
        .arg(file)
        .arg(url)
        .output()
        .map_err(|error| format!("the machine did not run curl: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "the file was not fetched: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(file.to_string())
}
