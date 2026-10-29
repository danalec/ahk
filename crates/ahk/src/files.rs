//! The files of the machine.
//!
//! This module carries the block of the files: the copy, the move, the
//! attributes, the times and the ini files.
//!
//! The handlers here do no I/O of their own: the arguments go to the
//! desktop and the answer comes back. The order of the arguments is the
//! order of the binary of v2.0.19, and it is not the order one would
//! guess: the names that *set* something take the value first and the
//! name of the file second. That was measured on the binary on
//! 2026-10-28: `FileSetTime "20000101000000", file, "M"` changes the
//! time of the file, and `FileSetTime file, "20000101000000", "M"` is
//! refused with "Parameter #1 of FileSetTime is invalid.".

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, TypeName, Value};

/// The milliseconds of a day.
const DAY: i64 = 86_400_000;

/// The letters of the attributes that v2 can set.
///
/// The letters that the machine of the box does not have are refused by
/// the box, with the reason; the ones that v2 does not know are refused
/// here, with the message of v2.
const ATTRIBUTES: [char; 7] = ['R', 'A', 'S', 'H', 'N', 'O', 'T'];

/// The encoding the files are opened with, when the script does not say.
///
/// The state is in the module of the settings: the script asks for it
/// with `FileEncoding` and reads it with the variable `A_FileEncoding`,
/// and the files ask the same place for it.
pub fn encoding() -> &'static str {
    crate::settings::file_bytes_encoding()
}

// -- the copy and the move --------------------------------------------------------

/// `DirCopy`: take a copy of a folder and of everything in it.
pub fn dir_copy(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let from = text_of(values, 0);
    let to = text_of(values, 1);
    refuse_the_wildcards(&from, line)?;
    refuse_the_wildcards(&to, line)?;
    let overwrite = flag_of(values, 2, "DirCopy", line)?;
    rt.desktop()
        .copy_dir(&from, &to, overwrite)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// `DirMove`: move a folder, with everything in it.
pub fn dir_move(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let from = text_of(values, 0);
    let to = text_of(values, 1);
    refuse_the_wildcards(&from, line)?;
    refuse_the_wildcards(&to, line)?;
    let overwrite = flag_of(values, 2, "DirMove", line)?;
    rt.desktop()
        .move_dir(&from, &to, overwrite)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// `FileCopy`: take a copy of a file.
pub fn file_copy(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let from = text_of(values, 0);
    let to = text_of(values, 1);
    refuse_the_wildcards(&from, line)?;
    refuse_the_wildcards(&to, line)?;
    let overwrite = flag_of(values, 2, "FileCopy", line)?;
    rt.desktop()
        .copy_file(&from, &to, overwrite)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// `FileMove`: move a file.
pub fn file_move(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let from = text_of(values, 0);
    let to = text_of(values, 1);
    refuse_the_wildcards(&from, line)?;
    refuse_the_wildcards(&to, line)?;
    let overwrite = flag_of(values, 2, "FileMove", line)?;
    rt.desktop()
        .move_file(&from, &to, overwrite)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// `FileInstall`: take a copy of a file, the way the installer of a
/// compiled script does.
///
/// The icon of the third parameter is the icon of the compiled script,
/// and the runtime does not compile anything.
pub fn file_install(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let from = text_of(values, 0);
    let to = text_of(values, 1);
    refuse_the_wildcards(&from, line)?;
    refuse_the_wildcards(&to, line)?;
    let overwrite = flag_of(values, 3, "FileInstall", line)?;
    rt.desktop()
        .copy_file(&from, &to, overwrite)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

// -- the attributes ----------------------------------------------------------------

/// `FileGetAttrib`: the attributes of a file, as the letters of v2.
pub fn file_get_attrib(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let letters = rt
        .desktop()
        .file_attributes(&path)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(letters.concat()))
}

/// `FileSetAttrib`: change the attributes of a file.
///
/// The attributes come first and the name of the file second, the way
/// the binary of v2.0.19 takes them.
pub fn file_set_attrib(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let attributes = text_of(values, 0);
    let path = text_of(values, 1);
    let mode = text_of(values, 2);
    refuse_the_recursion(&mode, line)?;
    refuse_the_wildcards(&path, line)?;
    let groups = groups_of(&attributes).ok_or_else(|| RuntimeError {
        message: "Parameter #1 of FileSetAttrib is invalid.".to_string(),
        line,
    })?;
    rt.desktop()
        .set_file_attributes(&path, &groups)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// The groups of the attributes of v2.
///
/// v2 writes the letters of the attributes with an operator in front of
/// them: "+H" turns the hidden attribute on, "-R" turns the read-only
/// one off and "^A" toggles it. Letters with no operator replace the
/// whole set of the attributes of the file.
///
/// The groups are given to the desktop one by one, and each of them
/// keeps its operator: the desktop needs to know whether a letter is
/// turned on, turned off or toggled.
fn groups_of(attributes: &str) -> Option<Vec<String>> {
    let mut groups: Vec<String> = Vec::new();
    let mut current = String::new();
    for letter in attributes.chars() {
        if letter == '+' || letter == '-' || letter == '^' {
            if !current.is_empty() {
                groups.push(current.clone());
                current.clear();
            }
            current.push(letter);
            continue;
        }
        let letter = letter.to_ascii_uppercase();
        if !ATTRIBUTES.contains(&letter) {
            return None;
        }
        current.push(letter);
    }
    if !current.is_empty() {
        groups.push(current);
    }
    if groups.is_empty() {
        return None;
    }
    Some(groups)
}

// -- the size and the times --------------------------------------------------------

/// `FileGetSize`: how big a file is.
pub fn file_get_size(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let unit = text_of(values, 1);
    let kilobytes = match unit.trim() {
        "" => false,
        "K" | "k" => true,
        _ => {
            return Err(RuntimeError {
                message: "Parameter #2 of FileGetSize is invalid.".to_string(),
                line,
            })
        }
    };
    // A folder has no size: v2 answers zero for one, and the tree does
    // the same.
    if is_a_folder(rt, &path) {
        return Ok(Value::Int(0));
    }
    let handle = rt
        .desktop()
        .open_file(&path, "r")
        .map_err(|reason| RuntimeError { message: reason, line })?;
    let size = rt
        .desktop()
        .file_size(handle)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    rt.desktop()
        .close_file(handle)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Int(if kilobytes { size / 1024 } else { size }))
}

/// `FileGetTime`: when a file was made, changed or read.
///
/// The answer is the stamp of v2: the fourteen digits of
/// `YYYYMMDDHH24MISS`, in the local time of the machine.
pub fn file_get_time(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let which = text_of(values, 1);
    let which = which.trim().to_ascii_uppercase();
    // The parameter is checked before the file is asked for, so that the
    // message of v2 comes before the answer.
    if !matches!(which.as_str(), "" | "M" | "C" | "A" | "CUT") {
        return Err(RuntimeError {
            message: "Parameter #2 of FileGetTime is invalid.".to_string(),
            line,
        });
    }
    let times = rt
        .desktop()
        .file_times(&path)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    let millis = match which.as_str() {
        "C" | "CUT" => times.0,
        "A" => times.2,
        _ => times.1,
    };
    let offset = local_offset(rt);
    Ok(Value::Text(stamp_of(millis, offset)))
}

/// `FileSetTime`: change the time of a file.
///
/// The time comes first and the name of the file second, the way the
/// binary of v2.0.19 takes them.
pub fn file_set_time(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let stamp = text_of(values, 0);
    let path = text_of(values, 1);
    let which = text_of(values, 2);
    let mode = text_of(values, 3);
    refuse_the_recursion(&mode, line)?;
    refuse_the_wildcards(&path, line)?;
    let offset = local_offset(rt);
    // The parameters are checked in the order of v2: the stamp is the
    // first one, and the time that is wanted is the third.
    let millis = if stamp.trim().is_empty() {
        // A stamp that is blank is the moment of the machine: v2 reads it
        // that way.
        let clock = rt.wall_clock().ok_or_else(|| RuntimeError {
            message: crate::date::NO_DATE.to_string(),
            line,
        })?;
        local_millis(&clock.local, offset)
    } else {
        match millis_of(&stamp, offset) {
            Some(millis) => millis,
            None => {
                return Err(RuntimeError {
                    message: "Parameter #1 of FileSetTime is invalid.".to_string(),
                    line,
                })
            }
        }
    };
    let which = which.trim().to_ascii_uppercase();
    if !matches!(which.as_str(), "" | "M" | "C" | "A") {
        return Err(RuntimeError {
            message: "Parameter #3 of FileSetTime is invalid.".to_string(),
            line,
        });
    }
    let (created, modified, accessed) = match which.as_str() {
        "C" => (Some(millis), None, None),
        "A" => (None, None, Some(millis)),
        _ => (None, Some(millis), None),
    };
    rt.desktop()
        .set_file_times(&path, created, modified, accessed)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

// -- the encoding ------------------------------------------------------------------

/// `FileEncoding`: the encoding the files are opened with.
///
/// The state is in the module of the settings, and the answer is the name
/// of the encoding, the way v2 spells it.
pub fn file_encoding(_rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let name = text_of(values, 0);
    crate::settings::set_file_encoding(&name)
        .map(|encoding| Value::Text(encoding.to_string()))
        .map_err(|message| RuntimeError { message, line })
}

// -- the file itself ----------------------------------------------------------------

/// `FileOpen`: open a file and answer it.
pub fn file_open(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let mut arguments: Vec<Value> = values.to_vec();
    if arguments.len() < 3 {
        // The file is opened with the encoding of the script when the
        // script does not say which one it wants.
        arguments.push(Value::Text(encoding().to_string()));
    }
    match crate::classes::file::open_file(&arguments, rt, line) {
        Ok(value) => Ok(value),
        Err(RuntimeFailure::Error(error)) => Err(error),
        Err(RuntimeFailure::Detailed(error, _)) => Err(error),
        Err(_) => Err(RuntimeError {
            message: "the file cannot be opened.".to_string(),
            line,
        }),
    }
}

// -- the ini files ------------------------------------------------------------------

/// `IniRead`: the value of a key of an ini file.
///
/// The default of the script is the answer when the key is not there,
/// and a key that is not there without a default is the failure of v2.
pub fn ini_read(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let section = text_of(values, 1);
    let key = text_of(values, 2);
    let default = values.get(3).map(|value| value.to_text());
    let answer = rt
        .desktop()
        .ini_read(&path, &section, &key, default.as_deref().unwrap_or(""));
    match answer {
        Ok(value) => Ok(Value::Text(value)),
        // v2 answers the default of the script for every failure when the
        // script gives one, and only the empty one is a failure of its
        // own.
        Err(reason) => match default {
            Some(default) => Ok(Value::Text(default)),
            None => Err(RuntimeError { message: reason, line }),
        },
    }
}

/// `IniWrite`: write a key of an ini file.
///
/// The value comes first and the name of the file second, the way the
/// binary of v2.0.19 takes them.
pub fn ini_write(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let value = text_of(values, 0);
    let path = text_of(values, 1);
    let section = text_of(values, 2);
    let key = text_of(values, 3);
    rt.desktop()
        .ini_write(&path, &section, &key, &value)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

/// `IniDelete`: take a key of an ini file away.
///
/// A key that is not given takes the whole section with it.
pub fn ini_delete(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let path = text_of(values, 0);
    let section = text_of(values, 1);
    let key = text_of(values, 2);
    rt.desktop()
        .ini_delete(&path, &section, &key)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(String::new()))
}

// -- the helpers --------------------------------------------------------------------

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values.get(index).map(|value| value.to_text()).unwrap_or_default()
}

/// Whether the argument at this place asks for the thing to happen.
///
/// v2 has no boolean: the number one is true and the number zero is
/// false. An argument that is not a number is the failure of v2, and the
/// message of v2 names it.
fn flag_of(values: &[Value], index: usize, name: &str, line: u32) -> Result<bool, RuntimeError> {
    let Some(value) = values.get(index) else { return Ok(false) };
    let number = value.to_number_strict().map_err(|_| RuntimeError {
        message: crate::runtime::requires(index + 1, name, TypeName::Number, value),
        line,
    })?;
    Ok(number != 0.0)
}

/// The wildcards of v2 are not carried.
///
/// A name that holds one is refused with the reason, because a copy of
/// a name that is not a file is not a copy.
fn refuse_the_wildcards(path: &str, line: u32) -> Result<(), RuntimeError> {
    if path.contains('*') || path.contains('?') {
        return Err(RuntimeError {
            message: "the wildcards of v2 are not carried yet.".to_string(),
            line,
        });
    }
    Ok(())
}

/// The recursion of v2 is not carried.
///
/// The mode of v2 asks for the subfolders to be walked into, and the
/// tree works on one name at a time.
fn refuse_the_recursion(mode: &str, line: u32) -> Result<(), RuntimeError> {
    if mode.to_ascii_uppercase().contains('R') {
        return Err(RuntimeError {
            message: "the recursion of v2 is not carried yet.".to_string(),
            line,
        });
    }
    Ok(())
}

/// Whether the path is a folder.
///
/// The answer is a folder of the desktop, and a desktop that cannot
/// look says that it does not know: a name that is not a folder is the
/// answer then.
fn is_a_folder(rt: &mut Runtime<'_>, path: &str) -> bool {
    let (folder, name) = parent_and_name(path);
    let folder = if folder.is_empty() {
        match rt.desktop().working_directory() {
            Ok(folder) => folder,
            Err(_) => return false,
        }
    } else {
        folder
    };
    match rt.desktop().list_files(&folder, &name, "D") {
        Ok(folders) => !folders.is_empty(),
        Err(_) => false,
    }
}

/// The folder of a path, and the name of the file in it.
fn parent_and_name(path: &str) -> (String, String) {
    match path.rfind(['/', '\\']) {
        Some(index) => (path[..=index].to_string(), path[index + 1..].to_string()),
        None => (String::new(), path.to_string()),
    }
}

/// The offset of the local time in minutes, when the machine knows it.
///
/// The clock of a run counts from an arbitrary origin, and the offset is
/// not in it: a run that does not know the timezone of the machine is
/// told so in the notes, and the time of a file is then the time of the
/// UTC.
fn local_offset(rt: &mut Runtime<'_>) -> i64 {
    match rt.wall_clock() {
        Some(clock) if clock.knows_the_timezone => clock.offset_minutes,
        _ => {
            rt.note("the machine does not know its timezone: the time of a file is the time of the UTC.");
            0
        }
    }
}

/// The stamp of v2 for a moment, in the local time of the machine.
///
/// The stamp is the fourteen digits of `YYYYMMDDHH24MISS`. The
/// milliseconds are not in it: v2 writes the second and no more.
fn stamp_of(millis: i64, offset: i64) -> String {
    let local = millis + offset * 60_000;
    let days = local.div_euclid(DAY);
    let rest = local.rem_euclid(DAY);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}{month:02}{day:02}{:02}{:02}{:02}",
        rest / 3_600_000,
        (rest / 60_000) % 60,
        (rest / 1000) % 60
    )
}

/// The moment of a stamp of v2, in the milliseconds since the epoch.
///
/// The stamp is `YYYYMMDDHH24MISS`, and v2 takes the leading part of
/// it too: what is left out keeps the value it has in a stamp of the
/// first of January.
fn millis_of(stamp: &str, offset: i64) -> Option<i64> {
    let text = stamp.trim();
    let (digits, fraction) = match text.split_once('.') {
        Some((digits, fraction)) => (digits, fraction),
        None => (text, ""),
    };
    if !digits.chars().all(|letter| letter.is_ascii_digit()) {
        return None;
    }
    if !fraction.chars().all(|letter| letter.is_ascii_digit()) || fraction.len() > 3 {
        return None;
    }
    if !matches!(digits.len(), 4 | 6 | 8 | 10 | 12 | 14) {
        return None;
    }
    let part = |at: usize, len: usize| -> Option<i64> {
        digits
            .get(at..at + len)
            .and_then(|part| part.parse::<i64>().ok())
    };
    let year = part(0, 4)?;
    let month = part(4, 2).unwrap_or(1);
    let day = part(6, 2).unwrap_or(1);
    let hour = part(8, 2).unwrap_or_default();
    let minute = part(10, 2).unwrap_or_default();
    let second = part(12, 2).unwrap_or_default();
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let millis = fraction
        .parse::<i64>()
        .map(|part| part * 10i64.pow(3 - fraction.len() as u32))
        .unwrap_or(0);
    let days = days_from_civil(year, month, day);
    let seconds = days * 86_400 + hour * 3_600 + minute * 60 + second;
    Some(seconds * 1000 + millis - offset * 60_000)
}

/// The moment of a date and a time of the machine, in the
/// milliseconds since the epoch.
///
/// The date and the time are the local ones, and the offset takes them
/// back to the epoch.
fn local_millis(when: &crate::date::DateTime, offset: i64) -> i64 {
    let days = days_from_civil(when.year, when.month, when.day);
    let seconds = days * 86_400 + when.hour * 3_600 + when.minute * 60 + when.second;
    seconds * 1000 - offset * 60_000
}

/// The days since the epoch of a date of the Gregorian calendar.
///
/// The count is the one of the calendar of the proleptic Gregorian, and
/// it is the arithmetic of the tree: the crate runs without the
/// standard library, which has no calendar of its own.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The date of the Gregorian calendar of a number of days since the
/// epoch.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month + 2) / 5 + 1;
    let month = if month < 10 { month + 3 } else { month - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{
        Clipboard, Desktop, Files, Hotkeys, Keyboard, Machine, Native, Pointer,
        Processes, Screen, Sound, Timers, Windows,
    };
    use crate::runtime::Clock;
    use crate::settings::Turn;
    use alloc::string::String;
    use alloc::vec;
    use core::cell::RefCell;
    use core::time::Duration;

    /// A desktop that answers nothing but the files: it records what it
    /// is asked and answers what the case needs.
    #[derive(Default)]
    struct Fake {
        calls: RefCell<Vec<String>>,
        attributes: Vec<String>,
        folders: Vec<String>,
        times: (i64, i64, i64),
        size: i64,
        value: String,
    }

    impl Desktop for Fake {}
    impl Sound for Fake {}
impl Machine for Fake {}

    impl Files for Fake {
        fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("copy_file {from} {to} {overwrite}"));
            Ok("copied".to_string())
        }

        fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("move_file {from} {to} {overwrite}"));
            Ok("moved".to_string())
        }

        fn copy_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("copy_dir {from} {to} {overwrite}"));
            Ok("copied".to_string())
        }

        fn move_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("move_dir {from} {to} {overwrite}"));
            Ok("moved".to_string())
        }

        fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
            self.calls.borrow_mut().push(format!("file_attributes {path}"));
            Ok(self.attributes.clone())
        }

        fn set_file_attributes(&mut self, path: &str, attributes: &[String]) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("set_file_attributes {path} {attributes:?}"));
            Ok("set".to_string())
        }

        fn file_times(&mut self, path: &str) -> Result<(i64, i64, i64), String> {
            self.calls.borrow_mut().push(format!("file_times {path}"));
            Ok(self.times)
        }

        fn set_file_times(
            &mut self,
            path: &str,
            created: Option<i64>,
            modified: Option<i64>,
            accessed: Option<i64>,
        ) -> Result<String, String> {
            self.calls
                .borrow_mut()
                .push(format!("set_file_times {path} {created:?} {modified:?} {accessed:?}"));
            Ok("set".to_string())
        }

        fn ini_read(&mut self, path: &str, section: &str, key: &str, default: &str) -> Result<String, String> {
            self.calls
                .borrow_mut()
                .push(format!("ini_read {path} {section} {key} {default}"));
            if self.value.is_empty() {
                return Err("The requested key, section or file was not found.".to_string());
            }
            Ok(self.value.clone())
        }

        fn ini_write(&mut self, path: &str, section: &str, key: &str, value: &str) -> Result<String, String> {
            self.calls
                .borrow_mut()
                .push(format!("ini_write {path} {section} {key} {value}"));
            Ok("written".to_string())
        }

        fn ini_delete(&mut self, path: &str, section: &str, key: &str) -> Result<String, String> {
            self.calls.borrow_mut().push(format!("ini_delete {path} {section} {key}"));
            Ok("deleted".to_string())
        }

        fn open_file(&mut self, path: &str, _mode: &str) -> Result<i64, String> {
            self.calls.borrow_mut().push(format!("open_file {path}"));
            Ok(7)
        }

        fn file_size(&mut self, handle: i64) -> Result<i64, String> {
            self.calls.borrow_mut().push(format!("file_size {handle}"));
            Ok(self.size)
        }

        fn close_file(&mut self, handle: i64) -> Result<(), String> {
            self.calls.borrow_mut().push(format!("close_file {handle}"));
            Ok(())
        }

        fn list_files(&mut self, folder: &str, pattern: &str, mode: &str) -> Result<Vec<String>, String> {
            self.calls.borrow_mut().push(format!("list_files {folder} {pattern} {mode}"));
            Ok(self.folders.clone())
        }

        fn working_directory(&mut self) -> Result<String, String> {
            Ok("C:\\work".to_string())
        }
    }

    impl Clipboard for Fake {}

    impl Timers for Fake {}

    impl Hotkeys for Fake {}

    impl Windows for Fake {}

    impl Native for Fake {}

    impl Keyboard for Fake {
        fn type_text(&mut self, _text: &str) -> Result<String, String> {
            Ok(String::new())
        }

        fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Pointer for Fake {
        fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Screen for Fake {
        fn capture(&mut self, _x: i64, _y: i64, _width: i64, _height: i64, _filename: &str) -> Result<String, String> {
            Ok(String::new())
        }

        fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Processes for Fake {
        fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
            Ok((0, String::new(), String::new()))
        }
    }

    /// The clock of the test: it counts from an arbitrary origin, like
    /// the clock of a runtime does.
    #[derive(Debug)]
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            1234
        }

        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// The clock of the test, on a machine that knows its date.
    #[derive(Debug)]
    struct TheClockWithTheDate;

    impl Clock for TheClockWithTheDate {
        fn now_ms(&self) -> u64 {
            1234
        }

        fn sleep_ms(&mut self, _millis: u64) {}

        fn wall_clock(&self) -> Option<crate::date::WallClock> {
            Some(crate::date::WallClock {
                local: crate::date::DateTime {
                    year: 2020,
                    month: 1,
                    day: 1,
                    hour: 2,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    weekday: 3,
                    yday: 0,
                },
                utc: crate::date::DateTime {
                    year: 2020,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 0,
                    millisecond: 0,
                    weekday: 3,
                    yday: 0,
                },
                offset_minutes: 120,
                knows_the_timezone: true,
            })
        }
    }

    /// The answer of a handler, and what the desktop was asked.
    fn run(
        handler: fn(&mut Runtime<'_>, &[Value], u32, u8) -> Result<Value, RuntimeError>,
        values: &[Value],
    ) -> (Result<Value, RuntimeError>, Vec<String>) {
        let mut desktop = Fake::default();
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = handler(&mut rt, values, 1, 0);
        let calls = desktop.calls.into_inner();
        (answer, calls)
    }

    /// The text of an answer.
    fn text_of_value(answer: &Result<Value, RuntimeError>) -> String {
        match answer {
            Ok(value) => value.to_text(),
            Err(error) => panic!("the handler failed: {}", error.message),
        }
    }

    /// The message of a failure.
    fn reason_of(answer: &Result<Value, RuntimeError>) -> String {
        match answer {
            Ok(value) => panic!("the handler answered {value:?} instead of a failure"),
            Err(error) => error.message.clone(),
        }
    }

    #[test]
    fn file_copy_asks_the_desktop_to_copy_the_file() {
        let (answer, calls) = run(
            file_copy,
            &[Value::Text("a.txt".into()), Value::Text("b.txt".into())],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["copy_file a.txt b.txt false"]);
    }

    #[test]
    fn file_copy_overwrites_when_the_script_asks_for_it() {
        let (answer, calls) = run(
            file_copy,
            &[
                Value::Text("a.txt".into()),
                Value::Text("b.txt".into()),
                Value::Int(1),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["copy_file a.txt b.txt true"]);
    }

    #[test]
    fn file_copy_refuses_the_wildcards_of_v2() {
        let (answer, calls) = run(
            file_copy,
            &[Value::Text("*.txt".into()), Value::Text("b.txt".into())],
        );
        assert_eq!(reason_of(&answer), "the wildcards of v2 are not carried yet.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_move_asks_the_desktop_to_move_the_file() {
        let (answer, calls) = run(
            file_move,
            &[Value::Text("a.txt".into()), Value::Text("b.txt".into())],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["move_file a.txt b.txt false"]);
    }

    #[test]
    fn dir_copy_asks_the_desktop_to_copy_the_folder() {
        let (answer, calls) = run(
            dir_copy,
            &[Value::Text("from".into()), Value::Text("to".into())],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["copy_dir from to false"]);
    }

    #[test]
    fn dir_move_asks_the_desktop_to_move_the_folder() {
        let (answer, calls) = run(
            dir_move,
            &[Value::Text("from".into()), Value::Text("to".into())],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["move_dir from to false"]);
    }

    #[test]
    fn file_install_asks_the_desktop_to_copy_the_file() {
        let (answer, calls) = run(
            file_install,
            &[
                Value::Text("a.txt".into()),
                Value::Text("b.txt".into()),
                Value::Text("icon.ico".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["copy_file a.txt b.txt false"]);
    }

    #[test]
    fn file_get_attrib_answers_the_letters_of_the_attributes() {
        let mut desktop = Fake {
            attributes: vec!["R".to_string(), "A".to_string()],
            ..Default::default()
        };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = file_get_attrib(&mut rt, &[Value::Text("a.txt".into())], 1, 0);
        assert_eq!(text_of_value(&answer), "RA");
    }

    #[test]
    fn file_set_attrib_asks_the_desktop_to_change_them() {
        let (answer, calls) = run(
            file_set_attrib,
            &[Value::Text("+R-H".into()), Value::Text("a.txt".into())],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec![r#"set_file_attributes a.txt ["+R", "-H"]"#]);
    }

    #[test]
    fn file_set_attrib_refuses_an_attribute_that_v2_does_not_know() {
        let (answer, calls) = run(
            file_set_attrib,
            &[Value::Text("Z".into()), Value::Text("a.txt".into())],
        );
        assert_eq!(reason_of(&answer), "Parameter #1 of FileSetAttrib is invalid.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_set_attrib_refuses_the_recursion_of_v2() {
        let (answer, calls) = run(
            file_set_attrib,
            &[
                Value::Text("+R".into()),
                Value::Text("a.txt".into()),
                Value::Text("R".into()),
            ],
        );
        assert_eq!(reason_of(&answer), "the recursion of v2 is not carried yet.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_get_size_asks_the_desktop_for_the_size() {
        let mut desktop = Fake { size: 4096, ..Default::default() };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = file_get_size(&mut rt, &[Value::Text("a.txt".into())], 1, 0);
        assert_eq!(text_of_value(&answer), "4096");
    }

    #[test]
    fn file_get_size_in_kilobytes_divides_the_size() {
        let mut desktop = Fake { size: 4096, ..Default::default() };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = file_get_size(
            &mut rt,
            &[Value::Text("a.txt".into()), Value::Text("K".into())],
            1,
            0,
        );
        assert_eq!(text_of_value(&answer), "4");
    }

    #[test]
    fn file_get_size_of_a_folder_is_zero() {
        let mut desktop = Fake {
            size: 4096,
            folders: vec!["C:\\work\\folder".to_string()],
            ..Default::default()
        };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = file_get_size(&mut rt, &[Value::Text("folder".into())], 1, 0);
        assert_eq!(text_of_value(&answer), "0");
    }

    #[test]
    fn file_get_time_answers_the_stamp_of_v2() {
        // The first of January of 2020, in the UTC.
        let mut desktop = Fake {
            times: (0, 1_577_836_800_000, 0),
            ..Default::default()
        };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = file_get_time(&mut rt, &[Value::Text("a.txt".into())], 1, 0);
        assert_eq!(text_of_value(&answer), "20200101000000");
    }

    #[test]
    fn file_get_time_asks_for_the_time_the_script_wants() {
        let mut desktop = Fake {
            times: (1_577_836_800_000, 1_609_459_200_000, 1_262_304_000_000),
            ..Default::default()
        };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let made = file_get_time(
            &mut rt,
            &[Value::Text("a.txt".into()), Value::Text("C".into())],
            1,
            0,
        );
        let read = file_get_time(
            &mut rt,
            &[Value::Text("a.txt".into()), Value::Text("A".into())],
            1,
            0,
        );
        assert_eq!(text_of_value(&made), "20200101000000");
        assert_eq!(text_of_value(&read), "20100101000000");
    }

    #[test]
    fn file_get_time_refuses_a_which_that_v2_does_not_know() {
        let (answer, calls) = run(
            file_get_time,
            &[Value::Text("a.txt".into()), Value::Text("Z".into())],
        );
        assert_eq!(reason_of(&answer), "Parameter #2 of FileGetTime is invalid.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_set_time_takes_the_time_first() {
        let (answer, calls) = run(
            file_set_time,
            &[
                Value::Text("20200101000000".into()),
                Value::Text("a.txt".into()),
                Value::Text("M".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["set_file_times a.txt None Some(1577836800000) None"]);
    }

    #[test]
    fn file_set_time_asks_for_the_time_the_script_wants() {
        let (answer, calls) = run(
            file_set_time,
            &[
                Value::Text("20200101000000".into()),
                Value::Text("a.txt".into()),
                Value::Text("C".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["set_file_times a.txt Some(1577836800000) None None"]);
    }

    #[test]
    fn file_set_time_refuses_a_stamp_that_v2_does_not_know() {
        let (answer, calls) = run(
            file_set_time,
            &[Value::Text("not a time".into()), Value::Text("a.txt".into())],
        );
        assert_eq!(reason_of(&answer), "Parameter #1 of FileSetTime is invalid.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_set_time_refuses_a_which_that_v2_does_not_know() {
        let (answer, calls) = run(
            file_set_time,
            &[
                Value::Text("20200101000000".into()),
                Value::Text("a.txt".into()),
                Value::Text("Z".into()),
            ],
        );
        assert_eq!(reason_of(&answer), "Parameter #3 of FileSetTime is invalid.");
        assert!(calls.is_empty());
    }

    #[test]
    fn file_encoding_sets_the_encoding_of_the_files_that_follow() {
        let _turn = Turn::take();
        let (answer, _calls) = run(file_encoding, &[Value::Text("UTF-16".into())]);
        assert_eq!(text_of_value(&answer), "UTF-16");
        assert_eq!(encoding(), "UTF-16");
        let (_answer, _calls) = run(file_encoding, &[Value::Text("UTF-8".into())]);
        assert_eq!(encoding(), "UTF-8");
    }

    #[test]
    fn file_encoding_refuses_an_encoding_the_tree_does_not_carry() {
        let _turn = Turn::take();
        let (answer, _calls) = run(file_encoding, &[Value::Text("ANSI".into())]);
        assert_eq!(reason_of(&answer), "Parameter #1 of FileEncoding is invalid.");
    }

    #[test]
    fn file_open_answers_a_file() {
        let (answer, calls) = run(
            file_open,
            &[Value::Text("a.txt".into()), Value::Text("r".into())],
        );
        assert!(matches!(answer, Ok(Value::Object(_))));
        assert!(calls.contains(&"open_file a.txt".to_string()));
    }

    #[test]
    fn ini_read_answers_the_value_of_the_key() {
        let mut desktop = Fake {
            value: "the value".to_string(),
            ..Default::default()
        };
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let answer = ini_read(
            &mut rt,
            &[
                Value::Text("a.ini".into()),
                Value::Text("Sec".into()),
                Value::Text("Key".into()),
            ],
            1,
            0,
        );
        assert_eq!(text_of_value(&answer), "the value");
    }

    #[test]
    fn ini_read_answers_the_default_when_the_key_is_not_there() {
        let (answer, calls) = run(
            ini_read,
            &[
                Value::Text("a.ini".into()),
                Value::Text("Sec".into()),
                Value::Text("Key".into()),
                Value::Text("fallback".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "fallback");
        assert_eq!(calls, vec!["ini_read a.ini Sec Key fallback"]);
    }

    #[test]
    fn ini_read_fails_when_the_key_is_not_there_and_no_default_is_given() {
        let (answer, _calls) = run(
            ini_read,
            &[
                Value::Text("a.ini".into()),
                Value::Text("Sec".into()),
                Value::Text("Key".into()),
            ],
        );
        assert_eq!(reason_of(&answer), "The requested key, section or file was not found.");
    }

    #[test]
    fn ini_write_takes_the_value_first() {
        let (answer, calls) = run(
            ini_write,
            &[
                Value::Text("the value".into()),
                Value::Text("a.ini".into()),
                Value::Text("Sec".into()),
                Value::Text("Key".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["ini_write a.ini Sec Key the value"]);
    }

    #[test]
    fn ini_delete_asks_the_desktop_to_take_the_key_away() {
        let (answer, calls) = run(
            ini_delete,
            &[
                Value::Text("a.ini".into()),
                Value::Text("Sec".into()),
                Value::Text("Key".into()),
            ],
        );
        assert_eq!(text_of_value(&answer), "");
        assert_eq!(calls, vec!["ini_delete a.ini Sec Key"]);
    }

    #[test]
    fn a_stamp_that_is_blank_is_the_moment_of_the_machine() {
        let mut desktop = Fake::default();
        let mut clock = TheClockWithTheDate;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        let (answer, calls) = {
            let answer = file_set_time(
                &mut rt,
                &[Value::Text(String::new()), Value::Text("a.txt".into())],
                1,
                0,
            );
            let calls = desktop.calls.into_inner();
            (answer, calls)
        };
        assert_eq!(text_of_value(&answer), "");
        // The moment of the machine is two hours east of the UTC.
        assert_eq!(calls, vec!["set_file_times a.txt None Some(1577836800000) None"]);
    }

    #[test]
    fn a_stamp_that_is_blank_needs_a_clock_that_knows_the_date() {
        let (answer, calls) = run(
            file_set_time,
            &[Value::Text(String::new()), Value::Text("a.txt".into())],
        );
        assert_eq!(reason_of(&answer), crate::date::NO_DATE);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_overwrite_of_a_copy_that_is_not_a_number_is_refused() {
        let (answer, calls) = run(
            file_copy,
            &[
                Value::Text("a.txt".into()),
                Value::Text("b.txt".into()),
                Value::Text("maybe".into()),
            ],
        );
        assert_eq!(
            reason_of(&answer),
            "Parameter #3 of FileCopy requires a Number, but received a String."
        );
        assert!(calls.is_empty());
    }

    #[test]
    fn the_stamp_of_a_moment_is_the_stamp_of_v2() {
        // The first of January of 2020, in the UTC.
        assert_eq!(stamp_of(1_577_836_800_000, 0), "20200101000000");
        // The same moment, in a time two hours east of the UTC.
        assert_eq!(stamp_of(1_577_836_800_000, 120), "20200101020000");
        // The last second of the year 1999.
        assert_eq!(stamp_of(946_684_799_000, 0), "19991231235959");
    }

    #[test]
    fn the_moment_of_a_stamp_is_the_milliseconds_since_the_epoch() {
        assert_eq!(millis_of("20200101000000", 0), Some(1_577_836_800_000));
        // A stamp two hours east of the UTC is two hours earlier in it.
        assert_eq!(millis_of("20200101000000", 120), Some(1_577_829_600_000));
        // The leading part of a stamp keeps the rest of it at the start
        // of the month: "202001" is the first of January of 2020.
        assert_eq!(millis_of("202001", 0), Some(1_577_836_800_000));
        assert_eq!(millis_of("not a stamp", 0), None);
        assert_eq!(millis_of("20201301000000", 0), None);
    }
}
