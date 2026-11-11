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
        Err("the addresses of this machine are read with getifaddrs, and this machine has none"
            .to_string())
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


/// The alphabet of the base64, the one of the RFC 4648.
const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The base64 of the bytes, in lines of 76 characters and the ends of
/// line of the SMTP: the shape a mail carries an attachment in.
fn base64(bytes: &[u8]) -> String {
    let mut out = String::new();
    let mut width = 0usize;
    for chunk in bytes.chunks(3) {
        let first = chunk[0] as u32;
        let second = *chunk.get(1).unwrap_or(&0) as u32;
        let third = *chunk.get(2).unwrap_or(&0) as u32;
        let group = (first << 16) | (second << 8) | third;
        let pad = 3 - chunk.len();
        let quad = [
            (group >> 18) & 63,
            (group >> 12) & 63,
            (group >> 6) & 63,
            group & 63,
        ];
        for (index, symbol) in quad.iter().enumerate() {
            let character = if index >= 4 - pad {
                b'=' as char
            } else {
                B64[*symbol as usize] as char
            };
            out.push(character);
            width += 1;
            if width == 76 {
                out.push('\r');
                out.push('\n');
                width = 0;
            }
        }
    }
    if width != 0 {
        out.push('\r');
        out.push('\n');
    }
    out
}

/// A header value on one line: the ends of line of a value are taken
/// away, so a value the script wrote cannot add a header of its own.
fn one_line(text: &str) -> String {
    text.chars()
        .filter(|character| *character != '\r' && *character != '\n')
        .collect()
}

/// The text of the message the way the SMTP carries it: the headers, a
/// blank line and the body -- and the attachments as the parts of a
/// multipart, each in its base64. The ends of line are the ones of the
/// protocol.
fn the_text_of_the_message(
    message: &ahk::desktop::MailMessage,
    attachments: &[(String, Vec<u8>)],
) -> String {
    let mut text = String::new();
    let header = |text: &mut String, name: &str, value: &str| {
        text.push_str(name);
        text.push_str(": ");
        text.push_str(&one_line(value));
        text.push_str("\r\n");
    };
    header(&mut text, "From", &message.from);
    header(&mut text, "To", &message.to.join(", "));
    if !message.cc.is_empty() {
        header(&mut text, "Cc", &message.cc.join(", "));
    }
    header(&mut text, "Subject", &message.subject);
    if let Some(reply_to) = &message.reply_to {
        header(&mut text, "Reply-To", reply_to);
    }
    for (name, value) in &message.headers {
        header(&mut text, name, value);
    }
    if attachments.is_empty() {
        text.push_str("\r\n");
        text.push_str(&message.body.replace('\n', "\r\n"));
        text.push_str("\r\n");
        return text;
    }
    let boundary = format!(
        "ahk-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.subsec_nanos())
            .unwrap_or(0)
    );
    header(&mut text, "MIME-Version", "1.0");
    header(
        &mut text,
        "Content-Type",
        &format!("multipart/mixed; boundary=\"{boundary}\""),
    );
    text.push_str("\r\n");
    text.push_str(&format!("--{boundary}\r\n"));
    text.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    text.push_str("Content-Transfer-Encoding: 8bit\r\n\r\n");
    text.push_str(&message.body.replace('\n', "\r\n"));
    text.push_str("\r\n");
    for (name, bytes) in attachments {
        text.push_str(&format!("--{boundary}\r\n"));
        text.push_str("Content-Type: application/octet-stream\r\n");
        text.push_str("Content-Transfer-Encoding: base64\r\n");
        text.push_str(&format!(
            "Content-Disposition: attachment; filename=\"{}\"\r\n\r\n",
            one_line(name)
        ));
        text.push_str(&base64(bytes));
        text.push_str("\r\n");
    }
    text.push_str(&format!("--{boundary}--\r\n"));
    text
}

/// Send one mail message through the curl of the machine.
///
/// The recipients of the envelope are the ones of the message and the
/// copies, the blind ones included -- the envelope is what the server
/// routes by, and the headers are what the reader sees. The attachments
/// are read before anything is sent: a file that cannot be read fails
/// the call, the way KeySharp fails it, and a mail that was not sent is
/// never reported as sent.
pub fn send_mail(message: &ahk::desktop::MailMessage) -> Result<(), String> {
    let mut attachments: Vec<(String, Vec<u8>)> = Vec::new();
    for path in &message.attachments {
        let bytes = std::fs::read(path)
            .map_err(|error| format!("the attachment {path} could not be read: {error}"))?;
        let name = path
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(path)
            .to_string();
        attachments.push((name, bytes));
    }
    let text = the_text_of_the_message(message, &attachments);
    let path = std::env::temp_dir().join(format!(
        "ahk-mail-{}-{}.eml",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::write(&path, text)
        .map_err(|error| format!("the message could not be staged: {error}"))?;
    let mut command = Command::new("curl");
    command
        .arg("--silent")
        .arg("--show-error")
        .arg("--url")
        .arg(format!("smtp://{}:{}", message.host, message.port))
        .arg("--mail-from")
        .arg(&message.from)
        .arg("--upload-file")
        .arg(&path);
    for recipient in message
        .to
        .iter()
        .chain(message.cc.iter())
        .chain(message.bcc.iter())
    {
        command.arg("--mail-rcpt").arg(recipient);
    }
    let output = command.output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "curl is not on this machine, and the box has no other way to the network".to_string()
        } else {
            format!("the mail failed: {error}")
        }
    })?;
    let _ = std::fs::remove_file(&path);
    if !output.status.success() {
        let code = output.status.code().unwrap_or(0);
        let reason = String::from_utf8_lossy(&output.stderr);
        let reason = reason.trim();
        // The curl of the machine writes "curl: (7) what happened", and
        // the message of the tree carries the code once.
        let reason = reason.strip_prefix("curl: ").unwrap_or(reason);
        let reason = reason.strip_prefix(&format!("({code}) ")).unwrap_or(reason);
        return Err(format!("the mail was not sent: ({code}) {reason}"));

    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{base64, is_loopback, the_text_of_the_message};

    /// The base64 is the one of the RFC: the vectors of it are the cases
    /// the RFC itself lists.
    #[test]
    fn the_base64_is_the_one_of_the_rfc() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==\r\n");
        assert_eq!(base64(b"fo"), "Zm8=\r\n");
        assert_eq!(base64(b"foo"), "Zm9v\r\n");
        assert_eq!(base64(b"foob"), "Zm9vYg==\r\n");
        assert_eq!(base64(b"fooba"), "Zm9vYmE=\r\n");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy\r\n");
    }

    /// The message carries the headers, the blank line and the body, and
    /// the ends of line are the ones of the protocol. A value that holds
    /// an end of line cannot add a header of its own.
    #[test]
    fn the_message_carries_the_headers_and_the_body() {
        let message = ahk::desktop::MailMessage {
            to: vec!["a@b".to_string()],
            cc: vec![],
            bcc: vec![],
            subject: "hi\r\nBcc: sneaky".to_string(),
            body: "there".to_string(),
            from: "me@x".to_string(),
            reply_to: None,
            attachments: vec![],
            headers: vec![("X-One".to_string(), "1".to_string())],
            host: "smtp".to_string(),
            port: 25,
        };
        let text = the_text_of_the_message(&message, &[]);
        assert!(text.starts_with("From: me@x\r\n"));
        assert!(text.contains("To: a@b\r\n"));
        assert!(text.contains("Subject: hiBcc: sneaky\r\n"));
        assert!(text.contains("X-One: 1\r\n"));
        assert!(text.contains("\r\n\r\nthere\r\n"));
        // The injected end of line is gone, so the "Bcc:" of the subject
        // is text inside the Subject line and never a header of its own:
        // no line of the message starts with it.
        assert!(!text.contains("\r\nBcc:"));
    }

    /// An attachment makes the message a multipart, and the part carries
    /// the bytes in the base64.
    #[test]
    fn the_attachment_rides_in_a_multipart() {
        let message = ahk::desktop::MailMessage {
            to: vec!["a@b".to_string()],
            cc: vec![],
            bcc: vec![],
            subject: "s".to_string(),
            body: "b".to_string(),
            from: "me@x".to_string(),
            reply_to: None,
            attachments: vec![],
            headers: vec![],
            host: "smtp".to_string(),
            port: 25,
        };
        let attachments = vec![("note.txt".to_string(), b"foo".to_vec())];
        let text = the_text_of_the_message(&message, &attachments);
        assert!(text.contains("multipart/mixed; boundary="));
        assert!(text.contains("Content-Transfer-Encoding: base64"));
        assert!(text.contains("filename=\"note.txt\""));
        assert!(text.contains("Zm9v"));
    }

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