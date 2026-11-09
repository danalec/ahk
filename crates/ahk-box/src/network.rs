//! The network of the box.
//!
//! The box has curl, and curl is the way out to the network: it fetches
//! a file and writes it on the disk.

use std::process::Command;

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
