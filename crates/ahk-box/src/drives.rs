//! The drives of the box.
//!
//! A Linux machine has no drive letters: its volumes are mounted at paths,
//! and `/proc/mounts` is the table of what is mounted where. The `DriveGet`
//! family of v2 names a drive by a letter -- "C:" -- and the box reads the
//! same questions from a path: the mount point, or any path on it, names the
//! volume the path is on.
//!
//! Measured on the binary of v2.0.19 on 2026-11-09, so that the answers of
//! the box have the same shape: `DriveGetType` of a drive that is there is
//! "Fixed" (and the other kinds are "Removable", "Network", "CDROM",
//! "RAMDisk" and "Unknown"), the filesystem is the name of the format of the
//! volume, the label is the name of the volume and the empty text when it has
//! none, the serial is a number, the free space and the capacity are counted
//! in megabytes, the state of a drive that is there is "Ready", and a drive
//! that is not there is the empty text for the type and "Invalid" for the
//! state.
//!
//! Measured on the box on 2026-11-09: `/` is on the volume of the machine,
//! its filesystem is the one of `/proc/mounts`, and its capacity and its
//! free space are the ones of `statvfs`.

use std::path::{Path, PathBuf};

/// The reason a drive letter of Windows has no answer here.
///
/// v2 names a drive by a letter, and a Linux machine has none: the text that
/// names one is refused with the reason, and not with an answer about a path
/// that happens to start with the same letter.
const NO_LETTERS: &str =
    "there are no drive letters on this machine: its volumes are named by the path they are \
     mounted at, and not by a letter.";

/// Whether a text names a drive the way Windows does: a letter and a colon.
///
/// "C:", "c:\" and "C:\Windows" are the form; "C" alone is a name of a file
/// of the working directory, and it is not the form.
fn is_a_windows_drive(text: &str) -> bool {
    let mut characters = text.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_alphabetic() && characters.next() == Some(':')
}

/// A volume of the machine, from the table of what is mounted.
#[derive(Debug, Clone)]
struct Mount {
    /// The device of the volume: "/dev/sda2", "tmpfs" and the like.
    device: String,
    /// The path the volume is mounted at.
    mount_point: PathBuf,
    /// The name of the format of the volume: "ext4", "btrfs" and the like.
    filesystem: String,
}

/// The drive of a path.
///
/// A path that is not there is the failure of the machine, which is what the
/// five readings of a drive that is not there answer in the binary: measured
/// on the binary of v2.0.19 on 2026-11-09, `DriveGetFilesystem` of a drive
/// that is not there is the failure of the system.
pub fn drive(path: &str) -> Result<Option<ahk::desktop::Drive>, String> {
    Ok(Some(the_reading_of(&the_absolute(path)?)?))
}

/// The kind of the drive of a path, and nothing when the path is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetType` of a drive
/// that is not there answers the empty text, and it is not a failure.
pub fn drive_type(path: &str) -> Result<Option<String>, String> {
    Ok(match the_absolute_or_nothing(path)? {
        None => None,
        Some(absolute) => Some(the_reading_of(&absolute)?.drive_type),
    })
}

/// The state of the drive of a path, and nothing when the path is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetStatus` of a
/// drive that is not there answers "Invalid", and a drive that is there
/// answers "Ready".
pub fn drive_status(path: &str) -> Result<Option<String>, String> {
    Ok(match the_absolute_or_nothing(path)? {
        None => None,
        Some(absolute) => Some(the_reading_of(&absolute)?.status),
    })
}

/// The reading of the drive of an absolute path.
fn the_reading_of(absolute: &Path) -> Result<ahk::desktop::Drive, String> {
    let Some(mount) = the_mount_of(absolute)? else {
        // A path that is there is on a volume of the machine: a mount that
        // does not name it is a table that could not be read, and not a
        // drive that is not there.
        return Err("the table of the mounted volumes of this machine does not name the volume of \
                    the path."
            .to_string());
    };
    let Some(statistics) = the_statistics_of(&mount.mount_point) else {
        return Err(format!(
            "the statistics of the volume mounted at {} could not be read.",
            mount.mount_point.display()
        ));
    };
    Ok(ahk::desktop::Drive {
        drive_type: the_kind_of(&mount),
        filesystem: mount.filesystem.clone(),
        label: the_label_of(&mount.device).unwrap_or_default(),
        serial: statistics.f_fsid as i64,
        free_megabytes: the_megabytes(statistics.f_bavail, statistics.f_frsize),
        capacity_megabytes: the_megabytes(statistics.f_blocks, statistics.f_frsize),
        status: "Ready".to_string(),
    })
}

/// The absolute path of a path, and the failure of the machine when it is not
/// there.
fn the_absolute(path: &str) -> Result<PathBuf, String> {
    if is_a_windows_drive(path) {
        return Err(format!(
            "{path} names a drive the way Windows does, and {NO_LETTERS}"
        ));
    }
    std::fs::canonicalize(path).map_err(|error| the_message_of(&error))
}

/// The absolute path of a path, and nothing when it is not there.
fn the_absolute_or_nothing(path: &str) -> Result<Option<PathBuf>, String> {
    if is_a_windows_drive(path) {
        return Err(format!(
            "{path} names a drive the way Windows does, and {NO_LETTERS}"
        ));
    }
    Ok(std::fs::canonicalize(path).ok())
}

/// The volume that a path is on: the longest mount point that is a prefix of
/// it.
///
/// The mount table may name the same path more than once -- an overlay and the
/// volumes under it -- and the longest path is the one the file is really on.
fn the_mount_of(path: &Path) -> Result<Option<Mount>, String> {
    let mounts = the_mounts()?;
    let mut best: Option<Mount> = None;
    for mount in mounts {
        if !path.starts_with(&mount.mount_point) {
            continue;
        }
        let longer = match &best {
            None => true,
            Some(best) => mount.mount_point.as_os_str().len() > best.mount_point.as_os_str().len(),
        };
        if longer {
            best = Some(mount);
        }
    }
    Ok(best)
}

/// The path the volume of a file is mounted at.
///
/// The file has to be there for the table of the mounts to be read against
/// it: the path of a file that is not there is not on a volume of the
/// machine, and it has no mount.
pub(crate) fn the_mount_point_of(path: &Path) -> Result<Option<PathBuf>, String> {
    Ok(the_mount_of(path)?.map(|mount| mount.mount_point))
}

/// The paths the volumes of the machine are mounted at.
pub(crate) fn the_mount_points() -> Result<Vec<PathBuf>, String> {
    Ok(the_mounts()?
        .into_iter()
        .map(|mount| mount.mount_point)
        .collect())
}

/// The table of the volumes of the machine.
fn the_mounts() -> Result<Vec<Mount>, String> {
    let text = std::fs::read_to_string("/proc/mounts").map_err(|error| the_message_of(&error))?;
    let mut mounts: Vec<Mount> = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 3 {
            continue;
        }
        mounts.push(Mount {
            device: the_unescaped(fields[0]),
            mount_point: PathBuf::from(the_unescaped(fields[1])),
            filesystem: the_unescaped(fields[2]),
        });
    }
    Ok(mounts)
}

/// The text of a field of `/proc/mounts`, with the escapes of it undone.
///
/// The table writes a space, a tab, a newline and a backslash as `\040`,
/// `\011`, `\012` and `\134`, because a field of the table ends at a space.
fn the_unescaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'\\' && at + 3 < bytes.len() {
            let octal = &text[at + 1..at + 4];
            if let Ok(number) = u8::from_str_radix(octal, 8) {
                out.push(number as char);
                at += 4;
                continue;
            }
        }
        let character = text[at..].chars().next().unwrap_or('\\');
        out.push(character);
        at += character.len_utf8();
    }
    out
}

/// How much of the volume a number of bytes is, in megabytes.
fn the_megabytes(blocks: u64, size: u64) -> i64 {
    let bytes = blocks as u128 * size as u128;
    (bytes / (1024 * 1024)) as i64
}

/// The kind of a volume, the way the binary names the kinds of Windows.
///
/// The names are read out of the format of the volume and out of the device
/// that holds it: a volume of a disk that the kernel says is removable is
/// "Removable", and the formats that are a disc, a network and a memory are
/// the ones that are named so.
fn the_kind_of(mount: &Mount) -> String {
    let kind = match mount.filesystem.as_str() {
        "iso9660" | "udf" => "CDROM",
        "nfs" | "nfs4" | "cifs" | "smb3" | "smbfs" | "9p" | "fuse.sshfs" => "Network",
        "tmpfs" | "ramfs" | "devtmpfs" => "RAMDisk",
        _ => "Fixed",
    };
    if kind == "Fixed" && the_device_is_removable(&mount.device) {
        return "Removable".to_string();
    }
    kind.to_string()
}

/// Whether the kernel says the device of a volume is removable.
fn the_device_is_removable(device: &str) -> bool {
    let Some(name) = device.strip_prefix("/dev/") else {
        return false;
    };
    let Some(block) = the_block_device_of(name) else {
        return false;
    };
    std::fs::read_to_string(format!("/sys/block/{block}/removable"))
        .map(|text| text.trim() == "1")
        .unwrap_or(false)
}

/// The name of the block device of a partition: "sda2" is "sda", and
/// "nvme0n1p2" is "nvme0n1".
fn the_block_device_of(name: &str) -> Option<String> {
    // A disk whose name ends in a number -- the nvme and the mmc ones -- puts
    // a "p" in front of the number of the partition, and the "p" goes with
    // the number.
    if let Some(at) = name.rfind('p') {
        let (disk, number) = (&name[..at], &name[at + 1..]);
        if !disk.is_empty() && !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()) {
            return Some(disk.to_string());
        }
    }
    // The disks whose name has no number of its own: "sda2" is "sda", and
    // "sda" is the disk itself.
    let without_digits = name.trim_end_matches(|character: char| character.is_ascii_digit());
    if without_digits.is_empty() {
        return None;
    }
    if without_digits.chars().any(|character| character.is_ascii_digit()) {
        // A disk whose name has a number of its own, and without the "p" of a
        // partition: the name is the disk.
        return Some(name.to_string());
    }
    Some(without_digits.to_string())
}

/// The label of the volume of a device, when it has one.
///
/// The machine keeps the labels as links of `/dev/disk/by-label`, each of them
/// pointing at the device it names, and the label is the name of the link.
fn the_label_of(device: &str) -> Option<String> {
    let wanted = std::fs::canonicalize(device).ok()?;
    let entries = std::fs::read_dir("/dev/disk/by-label").ok()?;
    for entry in entries.flatten() {
        let Ok(target) = std::fs::canonicalize(entry.path()) else {
            continue;
        };
        if target == wanted {
            let name = entry.file_name();
            return Some(the_unescaped(&name.to_string_lossy()));
        }
    }
    None
}

/// The statistics of the filesystem mounted at a path.
fn the_statistics_of(path: &Path) -> Option<StatVfs> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let text = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
        let mut statistics = StatVfs::default();
        let answer = unsafe { statvfs(text.as_ptr(), &mut statistics) };
        if answer != 0 {
            return None;
        }
        Some(statistics)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        None
    }
}

/// The statistics of a filesystem, as the C library publishes them.
///
/// The fields are the ones of the C structure of Linux, in their order; the
/// numbers are counted in the size of a block, which is `f_frsize`, and not in
/// the preferred size of `f_bsize`.
#[cfg(unix)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct StatVfs {
    /// The preferred size of a block, in bytes.
    f_bsize: u64,
    /// The size of a block of the filesystem, in bytes.
    f_frsize: u64,
    /// The number of the blocks of the filesystem.
    f_blocks: u64,
    /// The number of the free blocks.
    f_bfree: u64,
    /// The number of the free blocks for a process that is not privileged.
    f_bavail: u64,
    /// The number of the files the filesystem can hold.
    f_files: u64,
    /// The number of the free files.
    f_ffree: u64,
    /// The number of the free files for a process that is not privileged.
    f_favail: u64,
    /// The identifier of the filesystem.
    f_fsid: u64,
    /// The flags of the mount.
    f_flag: u64,
    /// The length of a name of a file.
    f_namemax: u64,
    /// The room the C structure keeps free.
    f_spare: [i32; 6],
}

/// The statistics of a filesystem, on a machine that is not Unix.
#[cfg(not(unix))]
#[derive(Debug, Default, Clone, Copy)]
struct StatVfs {
    /// The number of the blocks of the filesystem.
    f_blocks: u64,
    /// The size of a block of the filesystem, in bytes.
    f_frsize: u64,
    /// The number of the free blocks for a process that is not privileged.
    f_bavail: u64,
    /// The identifier of the filesystem.
    f_fsid: u64,
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `statvfs` the C library gives.
    #[link_name = "statvfs"]
    fn statvfs(path: *const std::ffi::c_char, statistics: *mut StatVfs) -> std::ffi::c_int;
}

/// The message of a failure of the machine, the way the tree says it: the
/// code of the system in front of what it says.
fn the_message_of(error: &std::io::Error) -> String {
    let text = error.to_string();
    let text = text.split(" (os error ").next().unwrap_or(&text);
    match error.raw_os_error() {
        Some(code) => format!("({code}) {text}"),
        None => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The letter of Windows is the letter and the colon, and a name of a
    /// file is not one.
    #[test]
    fn the_form_of_a_drive_letter_is_the_letter_and_the_colon() {
        assert!(is_a_windows_drive("C:"));
        assert!(is_a_windows_drive("c:\\"));
        assert!(is_a_windows_drive("C:\\Windows"));
        assert!(!is_a_windows_drive("C"));
        assert!(!is_a_windows_drive("/home"));
        assert!(!is_a_windows_drive("home"));
        assert!(!is_a_windows_drive(""));
    }

    /// The name of the block device of a partition is the disk it is on.
    #[test]
    fn the_block_device_of_a_partition_is_its_disk() {
        assert_eq!(the_block_device_of("sda2"), Some("sda".to_string()));
        assert_eq!(the_block_device_of("sda"), Some("sda".to_string()));
        assert_eq!(the_block_device_of("nvme0n1p2"), Some("nvme0n1".to_string()));
        assert_eq!(the_block_device_of("nvme0n1"), Some("nvme0n1".to_string()));
        assert_eq!(the_block_device_of("vda1"), Some("vda".to_string()));
    }

    /// The escapes of the table of the mounts are undone.
    #[test]
    fn the_escapes_of_the_mount_table_are_undone() {
        assert_eq!(the_unescaped("/a\\040b"), "/a b");
        assert_eq!(the_unescaped("/a\\011b"), "/a\tb");
        assert_eq!(the_unescaped("/plain"), "/plain");
    }

    /// The megabytes of a number of blocks are counted in the size of a
    /// block, and not in the preferred one.
    #[test]
    fn the_megabytes_of_a_filesystem_are_the_size_of_its_blocks() {
        assert_eq!(the_megabytes(1024 * 1024, 4096), 4096);
        assert_eq!(the_megabytes(0, 4096), 0);
    }

    /// The drive of the root of the box is there, and it is a fixed drive of
    /// a real filesystem.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_root_of_the_box_is_a_drive() {
        let drive = drive("/").expect("the root is read").expect("the root is there");
        assert!(!drive.filesystem.is_empty(), "{drive:?}");
        assert_eq!(drive.status, "Ready");
        assert!(
            drive.capacity_megabytes > 0,
            "the root has a capacity: {drive:?}"
        );
        assert!(
            drive.free_megabytes <= drive.capacity_megabytes,
            "the free space is part of the capacity: {drive:?}"
        );
        assert_eq!(drive_type("/").unwrap().as_deref(), Some("Fixed"));
        assert_eq!(drive_status("/").unwrap().as_deref(), Some("Ready"));
    }

    /// A path that is not there is not a drive, and the five readings that
    /// answer with the failure of the machine say so.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_path_that_is_not_there_is_not_a_drive() {
        let missing = "/tmp/ahk-wp6-there-is-no-such-path-xyz";
        assert_eq!(drive_type(missing), Ok(None));
        assert_eq!(drive_status(missing), Ok(None));
        // The five readings that answer with the failure of the machine say
        // the reason the path is not there.
        let answer = drive(missing);
        assert!(answer.is_err(), "a path that is not there is not a drive");
        assert!(
            answer.unwrap_err().starts_with('('),
            "the message of the machine carries its code"
        );
    }

    /// A drive letter of Windows is refused with the reason.
    #[test]
    fn a_drive_letter_is_refused_with_the_reason() {
        let answer = drive("C:");
        assert!(answer.is_err(), "a drive letter has no answer here");
        assert!(drive_type("C:\\").is_err(), "a drive letter has no answer");
        assert!(drive_status("c:").is_err(), "a drive letter has no answer");
    }
}