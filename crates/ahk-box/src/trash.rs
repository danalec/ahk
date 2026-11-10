//! The trash of the desk, the way the XDG specification of it says.
//!
//! `FileRecycle` moves a file or a folder into the trash, and
//! `FileRecycleEmpty` empties it. The trash of a user of a Linux machine is
//! `$XDG_DATA_HOME/Trash` -- `~/.local/share/Trash` when the variable is not
//! set -- with the trashed things under `files/` and a record of each of them
//! under `info/<name>.trashinfo`:
//!
//! ```text
//! [Trash Info]
//! Path=/home/user/notes.txt
//! DeletionDate=2026-11-09T21:15:00
//! ```
//!
//! The path of the record is the original path of the thing, encoded as a
//! URI, and the date is the local time of the deletion.
//!
//! Measured on the binary of v2.0.19 on 2026-11-09, so that the box has the
//! same shape: `FileRecycle` of a file that is there answers nothing and the
//! file is gone; a plain path that is not there is the failure "Failed"; a
//! pattern that matches nothing is not a failure; and `FileRecycleEmpty`
//! answers nothing.
//!
//! Measured on the box on 2026-11-09: the trash of the user is the one of the
//! home directory, and a file of the home filesystem moves into it with a
//! rename.

use std::path::{Path, PathBuf};

/// The root of the trash of the user: the directory that holds `files/` and
/// `info/`.
pub fn the_trash_root() -> Result<PathBuf, String> {
    if let Some(data) = std::env::var_os("XDG_DATA_HOME") {
        if !data.is_empty() {
            return Ok(PathBuf::from(data).join("Trash"));
        }
    }
    let Some(home) = std::env::var_os("HOME") else {
        return Err("this machine does not say where the home directory of the user is, and the \
                    trash of the desk is in it."
            .to_string());
    };
    Ok(PathBuf::from(home).join(".local").join("share").join("Trash"))
}

/// The trash a file goes into: the one of its own volume.
///
/// A file cannot be renamed from one volume of the machine to another, and the
/// specification of the trash therefore puts the trash of a volume in the top
/// directory of that volume: `$topdir/.Trash/<uid>` when the administrator
/// made the shared one, and `$topdir/.Trash-<uid>` when it has to be made for
/// the user.
fn the_trash_of(path: &Path) -> Result<PathBuf, String> {
    let home = the_trash_root()?;
    let Some(mount) = crate::drives::the_mount_point_of(path)? else {
        return Ok(home);
    };
    if Some(&mount) == the_home_mount().as_ref() {
        return Ok(home);
    }
    let user = the_user_id();
    let shared = mount.join(".Trash");
    if shared.is_dir() {
        return Ok(shared.join(user.to_string()));
    }
    Ok(mount.join(format!(".Trash-{user}")))
}

/// The volume the home directory of the user is on.
///
/// The trash of the home directory is on that volume, and a file of another
/// one is not.
fn the_home_mount() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    crate::drives::the_mount_point_of(Path::new(&home))
        .ok()
        .flatten()
}

/// The number of the user of the process, which is the number of the name of
/// the trash of the volume.
fn the_user_id() -> u32 {
    #[cfg(unix)]
    {
        // The C library knows the user of the process, and the trash of a
        // volume is named after it.
        unsafe { getuid() }
    }
    #[cfg(not(unix))]
    {
        0
    }
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `getuid` the C library gives.
    #[link_name = "getuid"]
    fn getuid() -> u32;
}

/// Recycle a file, a folder, or the files that a pattern matches.
///
/// A file goes into the trash of the volume it is on: the trash of the home
/// directory is the one of the home volume, and a file of another volume goes
/// into the trash of the top directory of that one -- a file cannot be renamed
/// across the volumes of a machine, and the specification of the trash says
/// that the trash of the volume is where it goes.
pub fn recycle(pattern: &str) -> Result<(), String> {
    // The pattern is walked first, and every match goes into the trash of the
    // volume it is on.
    for path in the_files_of(pattern)? {
        let trash = the_trash_of(&path)?;
        recycle_one(&trash, &path)?;
    }
    Ok(())
}

/// Empty the trash of the machine.
///
/// The trash of the home directory and the trash of every volume that has
/// one are emptied.
pub fn empty() -> Result<(), String> {
    let mut roots = vec![the_trash_root()?];
    let user = the_user_id();
    if let Ok(mounts) = crate::drives::the_mount_points() {
        for mount in mounts {
            for candidate in [
                mount.join(format!(".Trash-{user}")),
                mount.join(".Trash").join(user.to_string()),
            ] {
                if candidate.is_dir() && !roots.contains(&candidate) {
                    roots.push(candidate);
                }
            }
        }
    }
    for root in roots {
        empty_in(&root)?;
    }
    Ok(())
}

/// The files a pattern names, in the order of the directory.
///
/// A pattern whose last part is a `*` and a `?` is walked; a plain path that is
/// not there is the failure "Failed", which is what the binary answers for
/// one: measured on the binary of v2.0.19 on 2026-11-09. A pattern that
/// matches nothing is not a failure.
fn the_files_of(pattern: &str) -> Result<Vec<PathBuf>, String> {
    let pattern = pattern.trim_end_matches(['/', '\\']);
    if pattern.is_empty() {
        return Err("the path of a file to recycle is empty.".to_string());
    }
    let (parent, leaf) = the_parent_and_the_leaf(pattern);
    if leaf.contains('*') || leaf.contains('?') {
        let Ok(entries) = std::fs::read_dir(&parent) else {
            // A pattern whose directory is not there is the failure of the
            // binary too: it is the pattern itself that is wrong.
            return Err("Failed".to_string());
        };
        let mut files = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if the_pattern_matches(&leaf, &name) {
                files.push(entry.path());
            }
        }
        return Ok(files);
    }
    let path = Path::new(pattern);
    if !path.exists() {
        return Err("Failed".to_string());
    }
    Ok(vec![path.to_path_buf()])
}

/// Recycle a file, a folder, or the files that a pattern matches, into the
/// trash at a root.
///
/// The root is the one of the tests: it is a trash that the case made, and the
/// trash of the machine is not touched by it.
#[cfg(all(test, target_os = "linux"))]
pub(crate) fn recycle_in(root: &Path, pattern: &str) -> Result<(), String> {
    for path in the_files_of(pattern)? {
        recycle_one(root, &path)?;
    }
    Ok(())
}

/// Empty the trash at a root.
pub(crate) fn empty_in(root: &Path) -> Result<(), String> {
    for part in ["files", "info"] {
        let directory = root.join(part);
        let Ok(entries) = std::fs::read_dir(&directory) else {
            // A trash that was never made is a trash that is empty.
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let answer = if path.is_dir() {
                std::fs::remove_dir_all(&path)
            } else {
                std::fs::remove_file(&path)
            };
            answer.map_err(|error| the_message_of(&error))?;
        }
    }
    Ok(())
}

/// The directory a pattern names, and the last part of it.
fn the_parent_and_the_leaf(pattern: &str) -> (PathBuf, String) {
    match pattern.rfind('/') {
        Some(at) => {
            let parent = if at == 0 { "/" } else { &pattern[..at] };
            (PathBuf::from(parent), pattern[at + 1..].to_string())
        }
        None => (PathBuf::from("."), pattern.to_string()),
    }
}

/// Whether a name of a file matches the last part of a pattern.
///
/// A `*` is any number of characters and a `?` is one, which is the pattern of
/// v2; the case matters, as it does on the box.
fn the_pattern_matches(pattern: &str, name: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let name: Vec<char> = name.chars().collect();
    let (mut at_pattern, mut at_name) = (0usize, 0usize);
    let (mut star, mut after_star) = (None, 0usize);
    while at_name < name.len() {
        if at_pattern < pattern.len() && (pattern[at_pattern] == '?' || pattern[at_pattern] == name[at_name])
        {
            at_pattern += 1;
            at_name += 1;
        } else if at_pattern < pattern.len() && pattern[at_pattern] == '*' {
            star = Some(at_pattern);
            at_pattern += 1;
            after_star = at_name;
        } else if let Some(star) = star {
            at_pattern = star + 1;
            after_star += 1;
            at_name = after_star;
        } else {
            return false;
        }
    }
    while at_pattern < pattern.len() && pattern[at_pattern] == '*' {
        at_pattern += 1;
    }
    at_pattern == pattern.len()
}

/// Put one thing of the disk into the trash at a root.
fn recycle_one(root: &Path, path: &Path) -> Result<(), String> {
    let files = root.join("files");
    let info = root.join("info");
    std::fs::create_dir_all(&files).map_err(|error| the_message_of(&error))?;
    std::fs::create_dir_all(&info).map_err(|error| the_message_of(&error))?;
    let Some(name) = path.file_name() else {
        return Err("the path of a file to recycle has no name at its end.".to_string());
    };
    let name = the_free_name(root, &name.to_string_lossy());
    // The record goes first, so that a crash between the two steps leaves a
    // record of a file that is still where it was, and not a file in the
    // trash that nothing explains.
    let record = the_record_of(path);
    std::fs::write(info.join(format!("{name}.trashinfo")), record)
        .map_err(|error| the_message_of(&error))?;
    if let Err(error) = std::fs::rename(path, files.join(&name)) {
        let _ = std::fs::remove_file(info.join(format!("{name}.trashinfo")));
        return Err(the_message_of(&error));
    }
    Ok(())
}

/// The name a thing takes in the trash: its own name, or the first one that is
/// not taken.
fn the_free_name(root: &Path, name: &str) -> String {
    let (stem, extension) = match name.rfind('.') {
        Some(at) if at > 0 => (&name[..at], &name[at..]),
        _ => (name, ""),
    };
    let mut candidate = name.to_string();
    let mut counter = 1;
    while root.join("files").join(&candidate).exists()
        || root.join("info").join(format!("{candidate}.trashinfo")).exists()
    {
        candidate = format!("{stem}.{counter}{extension}");
        counter += 1;
    }
    candidate
}

/// The record of a thing that is put into the trash.
fn the_record_of(path: &Path) -> String {
    let absolute = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    format!(
        "[Trash Info]\nPath={}\nDeletionDate={}\n",
        the_encoded(&absolute.to_string_lossy()),
        the_local_time()
    )
}

/// A path, encoded as a URI, the way the record of the trash keeps it.
///
/// The letters and the digits are kept, and so are the four marks that are
/// always the same in a URI, the dot, the hyphen, the underscore and the
/// tilde, and the slash that is the separator of a path.
fn the_encoded(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        let kept = byte.is_ascii_alphanumeric()
            || matches!(byte, b'/' | b'-' | b'.' | b'_' | b'~');
        if kept {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// The time of the deletion, as the local time of the machine.
///
/// The C library knows the time of the machine, and the offset of the session
/// is the offset of it: `localtime_r` is asked, and a machine where it cannot
/// answer is given the time of the epoch, which is still a time.
fn the_local_time() -> String {
    #[cfg(unix)]
    {
        let seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or(0);
        let mut broken_down = Tm::default();
        let answer = unsafe { localtime_r(&seconds, &mut broken_down) };
        if !answer.is_null() {
            return format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                broken_down.tm_year + 1900,
                broken_down.tm_mon + 1,
                broken_down.tm_mday,
                broken_down.tm_hour,
                broken_down.tm_min,
                broken_down.tm_sec
            );
        }
        the_epoch_time(seconds)
    }
    #[cfg(not(unix))]
    {
        the_epoch_time(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_secs() as i64)
                .unwrap_or(0),
        )
    }
}

/// The time of a number of seconds since the epoch, in the calendar of the
/// epoch.
fn the_epoch_time(seconds: i64) -> String {
    let days = seconds.div_euclid(86_400);
    let rest = seconds.rem_euclid(86_400);
    let (year, month, day) = the_civil_date(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}",
        rest / 3600,
        (rest % 3600) / 60,
        rest % 60
    )
}

/// The year, the month and the day of a number of days since the epoch.
///
/// The count of the days is the one of Howard Hinnant: the years of 400 days
/// are the cycles of the calendar, and the rest is the months of the year.
fn the_civil_date(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_of_year = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_of_year + 2) / 5 + 1;
    let month = if month_of_year < 10 {
        month_of_year + 3
    } else {
        month_of_year - 9
    };
    (year + if month <= 2 { 1 } else { 0 }, month, day)
}

/// The pieces of the time the C library publishes.
#[cfg(unix)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct Tm {
    /// The seconds of the minute.
    tm_sec: std::ffi::c_int,
    /// The minutes of the hour.
    tm_min: std::ffi::c_int,
    /// The hours of the day.
    tm_hour: std::ffi::c_int,
    /// The day of the month.
    tm_mday: std::ffi::c_int,
    /// The month, counted from zero.
    tm_mon: std::ffi::c_int,
    /// The year, counted from 1900.
    tm_year: std::ffi::c_int,
    /// The day of the week.
    tm_wday: std::ffi::c_int,
    /// The day of the year.
    tm_yday: std::ffi::c_int,
    /// Whether the daylight saving time is in effect.
    tm_isdst: std::ffi::c_int,
    /// The offset of the zone, in seconds.
    tm_gmtoff: i64,
    /// The name of the zone.
    tm_zone: *const std::ffi::c_char,
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `localtime_r` the C library gives.
    #[link_name = "localtime_r"]
    fn localtime_r(seconds: *const i64, result: *mut Tm) -> *mut Tm;
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

    /// A root of a trash of the tests, under the temporary directory.
    #[cfg(target_os = "linux")]
    fn a_root(name: &str) -> PathBuf {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("ahk-wp6-trash-{name}-{stamp}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("the root of the trash is made");
        root
    }

    /// The pattern of v2 is a star and a question mark.
    #[test]
    fn the_pattern_of_a_file_is_the_one_of_v2() {
        assert!(the_pattern_matches("*.txt", "a.txt"));
        assert!(the_pattern_matches("a?c", "abc"));
        assert!(the_pattern_matches("*", "anything"));
        assert!(the_pattern_matches("a*", "a"));
        assert!(!the_pattern_matches("*.txt", "a.md"));
        assert!(!the_pattern_matches("a?c", "ac"));
    }

    /// The path of the record is encoded as a URI, and the marks of a path
    /// are kept.
    #[test]
    fn the_path_of_the_record_is_encoded() {
        assert_eq!(the_encoded("/home/user/notes.txt"), "/home/user/notes.txt");
        assert_eq!(the_encoded("/home/a b"), "/home/a%20b");
        assert_eq!(the_encoded("/home/ç"), "/home/%C3%A7");
    }

    /// The calendar of the epoch is the one of the days of the epoch.
    #[test]
    fn the_calendar_of_the_epoch_is_the_one_of_the_days() {
        assert_eq!(the_civil_date(0), (1970, 1, 1));
        assert_eq!(the_civil_date(19_000), (2022, 1, 8));
    }

    /// A file that is recycled is out of where it was and in the trash, with
    /// a record that names it.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_file_that_is_recycled_goes_into_the_trash() {
        let root = a_root("one");
        let directory = root.join("disk");
        std::fs::create_dir_all(&directory).expect("the directory of the file is made");
        let file = directory.join("notes.txt");
        std::fs::write(&file, "hello").expect("the file is written");
        recycle_in(&root, &file.to_string_lossy()).expect("the file is recycled");
        assert!(!file.exists(), "the file is out of where it was");
        let trashed = root.join("files").join("notes.txt");
        assert!(trashed.exists(), "the file is in the trash");
        let record = std::fs::read_to_string(root.join("info").join("notes.txt.trashinfo"))
            .expect("the record of the trash is written");
        assert!(record.starts_with("[Trash Info]\n"), "{record}");
        assert!(record.contains("Path=/"), "{record}");
        assert!(record.contains("DeletionDate="), "{record}");
        empty_in(&root).expect("the trash is emptied");
        assert!(!trashed.exists(), "the trash is empty");
    }

    /// The name of a thing that is already in the trash takes the first name
    /// that is not taken.
    #[cfg(target_os = "linux")]
    #[test]
    fn two_files_of_the_same_name_are_two_things_in_the_trash() {
        let root = a_root("two");
        let directory = root.join("disk");
        std::fs::create_dir_all(&directory).expect("the directory of the files is made");
        for text in ["one", "two"] {
            let file = directory.join("notes.txt");
            std::fs::write(&file, text).expect("the file is written");
            recycle_in(&root, &file.to_string_lossy()).expect("the file is recycled");
        }
        assert!(root.join("files").join("notes.txt").exists());
        assert!(root.join("files").join("notes.1.txt").exists());
        assert!(root.join("info").join("notes.1.txt.trashinfo").exists());
    }

    /// A file of a volume that is not the one of the home directory goes into
    /// the trash of the volume it is on: a file cannot be renamed across two
    /// volumes, and the specification of the trash puts the trash of a volume
    /// in the top directory of that volume.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_file_of_another_volume_goes_to_the_trash_of_that_volume() {
        let path = std::env::temp_dir().join(format!(
            "ahk-wp6-cross-volume-{}.txt",
            std::process::id()
        ));
        std::fs::write(&path, "hello").expect("the file is written");
        let trash = the_trash_of(&path).expect("the trash of the file is named");
        let home = the_trash_root().expect("the trash of the home is named");
        let mount = crate::drives::the_mount_point_of(&path)
            .expect("the table of the mounts is read")
            .expect("the file is on a volume");
        if Some(&mount) == the_home_mount().as_ref() {
            assert_eq!(trash, home, "a file of the home volume has the home trash");
        } else {
            assert!(trash.starts_with(&mount), "{trash:?} is on {mount:?}");
            assert_ne!(trash, home, "the trash of another volume is its own");
        }
        let _ = std::fs::remove_file(&path);
    }

    /// A plain path that is not there is the failure of the binary, and a
    /// pattern that matches nothing is not a failure.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_file_that_is_not_there_is_the_failure_of_the_binary() {
        let root = a_root("three");
        let directory = root.join("disk");
        std::fs::create_dir_all(&directory).expect("the directory is made");
        let answer = recycle_in(&root, &directory.join("nothing.txt").to_string_lossy());
        assert_eq!(answer, Err("Failed".to_string()));
        let pattern = directory.join("none-*.txt");
        assert_eq!(recycle_in(&root, &pattern.to_string_lossy()), Ok(()));
    }
}