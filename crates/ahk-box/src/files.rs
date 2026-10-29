//! The files of the machine, on the box.
//!
//! This module carries the block of the files: the copy, the move, the
//! attributes, the times and the ini files.
//!
//! The names are the names of v2, and the machine is the one of the box:
//! an attribute that the filesystem of the box does not have is refused
//! with the reason, and the times of a file are the milliseconds since
//! the Unix epoch, so that the language can write them the way v2 does.
//!
//! The ini files are the ones of Windows: a section in brackets, a key
//! and a value with an equal sign between them, and a comment that
//! starts with a semicolon. The files that v2 writes are in UTF-16 with
//! a byte order mark, and the ones this box writes are in UTF-8: both
//! are read.

use std::fs::{self, FileTimes};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::desktop::LabDesktop;

/// The message of v2 for a key that is not in an ini file.
const NOT_FOUND: &str = "The requested key, section or file was not found.";

/// The letters of the attributes that the filesystem of the box has.
///
/// The read-only one is the only bit of its own that the machine keeps;
/// the others are the ordinary state of a file, and a folder is a
/// folder.
const KNOWN: [char; 4] = ['R', 'A', 'N', 'D'];

impl ahk::desktop::Files for LabDesktop {
    fn open_file(&mut self, path: &str, mode: &str) -> Result<i64, String> {
        let mut options = std::fs::OpenOptions::new();
        match mode {
            "r" => {
                options.read(true);
            }
            "w" => {
                options.write(true).create(true).truncate(true);
            }
            "a" => {
                options.append(true).create(true);
            }
            "rw" | "r+" => {
                options.read(true).write(true).create(true);
            }
            other => return Err(format!("the mode {other:?} is not one this desktop knows")),
        }
        let file = options.open(path).map_err(|error| error.to_string())?;
        self.files.push(Some(file));
        Ok((self.files.len() - 1) as i64)
    }

    fn read_file_at(&mut self, handle: i64, pos: i64, count: i64) -> Result<Vec<u8>, String> {
        use std::io::{Read, Seek, SeekFrom};
        let file = self
            .files
            .get_mut(handle.max(0) as usize)
            .and_then(|file| file.as_mut())
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        file.seek(SeekFrom::Start(pos.max(0) as u64)).map_err(|error| error.to_string())?;
        let mut buffer = vec![0u8; count.max(0) as usize];
        let read = file.read(&mut buffer).map_err(|error| error.to_string())?;
        buffer.truncate(read);
        Ok(buffer)
    }

    fn write_file_at(&mut self, handle: i64, pos: i64, bytes: &[u8]) -> Result<usize, String> {
        use std::io::{Seek, SeekFrom, Write};
        let file = self
            .files
            .get_mut(handle.max(0) as usize)
            .and_then(|file| file.as_mut())
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        file.seek(SeekFrom::Start(pos.max(0) as u64)).map_err(|error| error.to_string())?;
        file.write_all(bytes).map_err(|error| error.to_string())?;
        Ok(bytes.len())
    }

    fn file_size(&mut self, handle: i64) -> Result<i64, String> {
        let file = self
            .files
            .get(handle.max(0) as usize)
            .and_then(|file| file.as_ref())
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        let length = file.metadata().map_err(|error| error.to_string())?.len();
        Ok(length as i64)
    }

    fn close_file(&mut self, handle: i64) -> Result<(), String> {
        let slot = self
            .files
            .get_mut(handle.max(0) as usize)
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        *slot = None;
        Ok(())
    }

    fn file_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(Path::new(path).exists())
    }

    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        std::fs::remove_file(path)
            .map_err(|error| format!("the file {path:?} cannot be taken away: {error}"))?;
        Ok(format!("deleted the file {path}"))
    }

    fn create_dir(&mut self, path: &str) -> Result<String, String> {
        std::fs::create_dir_all(path)
            .map_err(|error| format!("the folder {path:?} cannot be made: {error}"))?;
        Ok(format!("made the directory {path}"))
    }

    fn dir_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(Path::new(path).is_dir())
    }

    fn dir_delete(&mut self, path: &str) -> Result<String, String> {
        std::fs::remove_dir_all(path).map_err(|error| error.to_string())?;
        Ok(format!("deleted the directory {path}"))
    }

    fn working_directory(&mut self) -> Result<String, String> {
        std::env::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|error| format!("the working directory cannot be read: {error}"))
    }
    fn list_files(&mut self, folder: &str, pattern: &str, mode: &str) -> Result<Vec<String>, String> {
        let mut files: Vec<String> = Vec::new();
        crate::desktop::walk_the_folder(Path::new(folder), pattern, &mode.to_ascii_uppercase(), &mut files)?;
        // The order of a directory is not the order of its name, and a
        // script that loops over the files wants the same order every
        // time.
        files.sort();
        Ok(files)
    }
    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("the file {path:?} cannot be read: {error}"))?;
        let mut lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        // A file that ends with a newline has no last empty line: the
        // newline is the end of the last line, not a line of its own.
        if text.ends_with('\n') {
            lines.pop();
        }
        Ok(lines)
    }
    fn write_file(&mut self, path: &str, text: &str) -> Result<String, String> {
        std::fs::write(path, text)
            .map_err(|error| format!("the file {path:?} cannot be written: {error}"))?;
        Ok(text.to_string())
    }
    fn append_file(&mut self, path: &str, text: &str) -> Result<String, String> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|error| format!("the file {path:?} cannot be written: {error}"))?;
        file.write_all(text.as_bytes())
            .map_err(|error| format!("the file {path:?} cannot be written: {error}"))?;
        Ok(text.to_string())
    }
    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_file() {
            return Err(format!("the file {from:?} is not there."));
        }
        let target = target_of(source, Path::new(to));
        if target.exists() && !overwrite {
            return Err(format!("the file {:?} is there already.", target.to_string_lossy()));
        }
        fs::copy(source, &target)
            .map_err(|error| format!("the file {from:?} cannot be copied: {error}"))?;
        Ok(format!("copied the file {from} to {}", target.to_string_lossy()))
    }

    fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.exists() {
            return Err(format!("the file {from:?} is not there."));
        }
        let target = target_of(source, Path::new(to));
        if target.exists() {
            if !overwrite {
                return Err(format!("the file {:?} is there already.", target.to_string_lossy()));
            }
            fs::remove_file(&target).map_err(|error| {
                format!("the file {:?} cannot be taken away: {error}", target.to_string_lossy())
            })?;
        }
        // The rename is the move, unless the file goes to another volume:
        // the copy and the delete are the move then.
        if fs::rename(source, &target).is_err() {
            fs::copy(source, &target)
                .map_err(|error| format!("the file {from:?} cannot be moved: {error}"))?;
            fs::remove_file(source)
                .map_err(|error| format!("the file {from:?} cannot be taken away: {error}"))?;
        }
        Ok(format!("moved the file {from} to {}", target.to_string_lossy()))
    }

    fn copy_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_dir() {
            return Err(format!("the folder {from:?} is not there."));
        }
        let target = Path::new(to);
        if target.exists() {
            if !overwrite {
                return Err(format!("the folder {to:?} is there already."));
            }
        } else {
            fs::create_dir_all(target)
                .map_err(|error| format!("the folder {to:?} cannot be made: {error}"))?;
        }
        // v2 copies the *contents* of the folder: the folder that is
        // there is the one that holds the files.
        copy_tree(source, target, overwrite)?;
        Ok(format!("copied the folder {from} to {to}"))
    }

    fn move_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_dir() {
            return Err(format!("the folder {from:?} is not there."));
        }
        let target = Path::new(to);
        if target.exists() {
            if !overwrite {
                return Err(format!("the folder {to:?} is there already."));
            }
            // The folder that is there is merged with the one that
            // comes: the files of the source win, and the ones that are
            // only in the target stay.
            copy_tree(source, target, true)?;
            fs::remove_dir_all(source)
                .map_err(|error| format!("the folder {from:?} cannot be taken away: {error}"))?;
            return Ok(format!("moved the folder {from} to {to}"));
        }
        fs::rename(source, target)
            .map_err(|error| format!("the folder {from:?} cannot be moved: {error}"))?;
        Ok(format!("moved the folder {from} to {to}"))
    }

    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        let metadata =
            fs::metadata(path).map_err(|error| format!("the file {path:?} cannot be read: {error}"))?;
        Ok(letters_of(&metadata).into_iter().map(|letter| letter.to_string()).collect())
    }

    fn set_file_attributes(&mut self, path: &str, attributes: &[String]) -> Result<String, String> {
        let metadata =
            fs::metadata(path).map_err(|error| format!("the file {path:?} cannot be read: {error}"))?;
        let mut letters = letters_of(&metadata);
        for group in attributes {
            let (operator, wanted) = split_group(group);
            match operator {
                Some('+') => {
                    for letter in wanted.chars() {
                        add(&mut letters, letter);
                    }
                }
                Some('-') => {
                    for letter in wanted.chars() {
                        letters.retain(|have| *have != letter);
                    }
                }
                Some('^') => {
                    for letter in wanted.chars() {
                        if letters.contains(&letter) {
                            letters.retain(|have| *have != letter);
                        } else {
                            add(&mut letters, letter);
                        }
                    }
                }
                // The letters with no operator replace the whole set of
                // the attributes of the file.
                None => {
                    letters.clear();
                    for letter in wanted.chars() {
                        add(&mut letters, letter);
                    }
                }
                _ => {}
            }
        }
        for letter in &letters {
            if !KNOWN.contains(letter) {
                return Err(format!("the machine of this box has no such attribute: {letter}."));
            }
        }
        if letters.contains(&'D') && !metadata.is_dir() {
            return Err(format!("the file {path:?} is not a folder."));
        }
        // The read-only bit is the only one the machine of the box has
        // of its own.
        let readonly = letters.contains(&'R');
        let mut permissions = metadata.permissions();
        permissions.set_readonly(readonly);
        fs::set_permissions(path, permissions)
            .map_err(|error| format!("the file {path:?} cannot be changed: {error}"))?;
        Ok(format!(
            "the attributes of the file {path:?} are {}",
            letters.iter().collect::<String>()
        ))
    }

    fn file_times(&mut self, path: &str) -> Result<(i64, i64, i64), String> {
        let metadata =
            fs::metadata(path).map_err(|error| format!("the file {path:?} cannot be read: {error}"))?;
        let modified = millis_of(metadata.modified()).unwrap_or(0);
        // The machine that does not keep the time a file was made has
        // none: the time it was changed is the oldest it knows.
        let created = millis_of(metadata.created()).unwrap_or(modified);
        let accessed = millis_of(metadata.accessed()).unwrap_or(modified);
        Ok((created, modified, accessed))
    }

    fn set_file_times(
        &mut self,
        path: &str,
        created: Option<i64>,
        modified: Option<i64>,
        accessed: Option<i64>,
    ) -> Result<String, String> {
        let file = fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|error| format!("the file {path:?} cannot be changed: {error}"))?;
        let mut times = FileTimes::new();
        if let Some(created) = created {
            times = set_created(times, when_of(created))?;
        }
        if let Some(modified) = modified {
            times = times.set_modified(when_of(modified));
        }
        if let Some(accessed) = accessed {
            times = times.set_accessed(when_of(accessed));
        }
        file.set_times(times)
            .map_err(|error| format!("the times of the file {path:?} cannot be changed: {error}"))?;
        Ok(format!("the times of the file {path:?} are set"))
    }

    fn ini_read(&mut self, path: &str, section: &str, key: &str, default: &str) -> Result<String, String> {
        let text = read_ini(path)?;
        // v2 reads the three of them with the same function: the value
        // of a key, the whole section, or the names of the sections.
        let found = if section.is_empty() {
            Some(sections_of(&text).join("\n"))
        } else if key.is_empty() {
            section_content(&text, section)
        } else {
            value_of(&text, section, key)
        };
        match found {
            Some(value) => Ok(value),
            None => {
                if default.is_empty() {
                    Err(NOT_FOUND.to_string())
                } else {
                    Ok(default.to_string())
                }
            }
        }
    }

    fn ini_write(&mut self, path: &str, section: &str, key: &str, value: &str) -> Result<String, String> {
        let text = match fs::read(path) {
            Ok(bytes) => decode(&bytes),
            // A file that is not there is made.
            Err(_) => String::new(),
        };
        let ending = ending_of(&text);
        let written = write_key(&text, ending, section, key, value);
        fs::write(path, written.as_bytes())
            .map_err(|error| format!("the file {path:?} cannot be written: {error}"))?;
        Ok(format!("wrote the key {key:?} of the section {section:?}"))
    }

    fn ini_delete(&mut self, path: &str, section: &str, key: &str) -> Result<String, String> {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            // A file that is not there has no key to take away.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(format!("the file {path:?} has no key {key:?}"));
            }
            Err(error) => return Err(format!("the file {path:?} cannot be read: {error}")),
        };
        let text = decode(&bytes);
        let ending = ending_of(&text);
        let mut lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        if let Some((start, end)) = section_of(&lines, section) {
            if key.is_empty() {
                // The whole section goes away.
                lines.drain(start..end);
            } else {
                let mut key_at = None;
                for (offset, line) in lines[(start + 1)..end].iter().enumerate() {
                    if let Some((name, _)) = line.split_once('=') {
                        if name.trim().eq_ignore_ascii_case(key) {
                            key_at = Some(start + 1 + offset);
                            break;
                        }
                    }
                }
                if let Some(index) = key_at {
                    lines.remove(index);
                }
            }
        }
        let mut written = lines.join(ending);
        if !written.is_empty() {
            written.push_str(ending);
        }
        fs::write(path, written.as_bytes())
            .map_err(|error| format!("the file {path:?} cannot be written: {error}"))?;
        Ok(format!("took the key {key:?} of the section {section:?} away"))
    }
}

// -- the helpers -------------------------------------------------------------------

/// Where a copy of a file goes.
///
/// v2 copies a file into a folder that is there, keeping the name of the
/// file.
fn target_of(source: &Path, destination: &Path) -> PathBuf {
    if destination.is_dir() {
        match source.file_name() {
            Some(name) => destination.join(name),
            None => destination.to_path_buf(),
        }
    } else {
        destination.to_path_buf()
    }
}

/// The files of a folder, and the folders in it, copied into another.
///
/// The files that are already there are left alone unless the copy is
/// told to overwrite them.
fn copy_tree(source: &Path, target: &Path, overwrite: bool) -> Result<(), String> {
    let entries =
        fs::read_dir(source).map_err(|error| format!("the folder {source:?} cannot be read: {error}"))?;
    for entry in entries {
        let entry =
            entry.map_err(|error| format!("the folder {source:?} cannot be read: {error}"))?;
        let from = entry.path();
        let to = target.join(entry.file_name());
        if from.is_dir() {
            fs::create_dir_all(&to)
                .map_err(|error| format!("the folder {to:?} cannot be made: {error}"))?;
            copy_tree(&from, &to, overwrite)?;
            continue;
        }
        if to.exists() && !overwrite {
            return Err(format!("the file {to:?} is there already."));
        }
        fs::copy(&from, &to)
            .map_err(|error| format!("the file {from:?} cannot be copied: {error}"))?;
    }
    Ok(())
}

/// The letters of the attributes of a file.
///
/// The order is the order of v2: the read-only one first, and the
/// folder last.
fn letters_of(metadata: &fs::Metadata) -> Vec<char> {
    let mut letters: Vec<char> = Vec::new();
    let readonly = metadata.permissions().readonly();
    if readonly {
        letters.push('R');
    }
    if metadata.is_dir() {
        letters.push('D');
    } else if !readonly {
        // The archive bit is what a file carries on Windows, and the
        // machine of the box has no bit of its own for it.
        letters.push('A');
    }
    letters
}

/// Put a letter of an attribute in the set, in the order of v2.
fn add(letters: &mut Vec<char>, letter: char) {
    if letters.contains(&letter) {
        return;
    }
    let at = letters.iter().position(|have| order(*have) > order(letter));
    match at {
        Some(at) => letters.insert(at, letter),
        None => letters.push(letter),
    }
}

/// The place of a letter of an attribute, the order of v2.
fn order(letter: char) -> u8 {
    match letter {
        'R' => 0,
        'A' => 1,
        'S' => 2,
        'H' => 3,
        'O' => 4,
        'T' => 5,
        'D' => 6,
        _ => 7,
    }
}

/// The operator of a group of attributes, and the letters of it.
fn split_group(group: &str) -> (Option<char>, String) {
    let mut letters = group.chars();
    match letters.next() {
        Some(operator @ ('+' | '-' | '^')) => (Some(operator), letters.as_str().to_string()),
        _ => (None, group.to_string()),
    }
}

/// The milliseconds since the epoch of a moment of the machine.
fn millis_of(when: std::io::Result<SystemTime>) -> Option<i64> {
    let when = when.ok()?;
    match when.duration_since(UNIX_EPOCH) {
        Ok(duration) => Some(duration.as_millis() as i64),
        Err(error) => Some(-(error.duration().as_millis() as i64)),
    }
}

/// A moment of the machine, from the milliseconds since the epoch.
fn when_of(millis: i64) -> SystemTime {
    if millis >= 0 {
        UNIX_EPOCH + Duration::from_millis(millis as u64)
    } else {
        UNIX_EPOCH - Duration::from_millis((-millis) as u64)
    }
}

/// The time a file was made.
///
/// The standard library writes it on Windows only: the filesystem of the
/// box has no way of changing it, and the box says so.
#[cfg(windows)]
fn set_created(times: FileTimes, when: SystemTime) -> Result<FileTimes, String> {
    use std::os::windows::fs::FileTimesExt;
    Ok(times.set_created(when))
}

/// The time a file was made.
///
/// The standard library writes it on Windows only: the filesystem of the
/// box has no way of changing it, and the box says so.
#[cfg(not(windows))]
fn set_created(_times: FileTimes, _when: SystemTime) -> Result<FileTimes, String> {
    Err("the machine of this box cannot change the time a file was made.".to_string())
}

/// The text of an ini file.
///
/// The files of v2 are written in UTF-16 with a byte order mark, and the
/// files of the box are written in UTF-8: both are read.
fn decode(bytes: &[u8]) -> String {
    let units = |pair: &[u8]| u16::from_le_bytes([pair[0], pair[1]]);
    if bytes.starts_with(&[0xFF, 0xFE]) {
        let units: Vec<u16> = bytes[2..].chunks_exact(2).map(units).collect();
        return String::from_utf16_lossy(&units);
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let units: Vec<u16> = bytes[2..].chunks_exact(2).map(|pair| u16::from_be_bytes([pair[0], pair[1]])).collect();
        return String::from_utf16_lossy(&units);
    }
    String::from_utf8_lossy(bytes).to_string()
}

/// The text of an ini file, or the failure of v2 when it is not there.
fn read_ini(path: &str) -> Result<String, String> {
    match fs::read(path) {
        Ok(bytes) => Ok(decode(&bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(NOT_FOUND.to_string()),
        Err(error) => Err(format!("the file {path:?} cannot be read: {error}")),
    }
}

/// The end of the lines of a text.
///
/// The files of Windows end their lines with two characters, and the
/// files of the box with one: a file that is read and written again
/// keeps the way it was written.
fn ending_of(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

/// The lines of a section of an ini file, and where the section ends.
fn section_of(lines: &[String], section: &str) -> Option<(usize, usize)> {
    let mut start = None;
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
            continue;
        }
        let name = &trimmed[1..trimmed.len() - 1];
        match start {
            Some(start) => return Some((start, index)),
            None => {
                if name.eq_ignore_ascii_case(section) {
                    start = Some(index);
                }
            }
        }
    }
    start.map(|start| (start, lines.len()))
}

/// The value of a key of a section of an ini file.
fn value_of(text: &str, section: &str, key: &str) -> Option<String> {
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let (start, end) = section_of(&lines, section)?;
    for line in &lines[(start + 1)..end] {
        let trimmed = line.trim();
        if trimmed.starts_with(';') {
            continue;
        }
        let Some((name, value)) = trimmed.split_once('=') else { continue };
        if name.trim().eq_ignore_ascii_case(key) {
            return Some(value.trim().to_string());
        }
    }
    None
}

/// The content of a section of an ini file, without its comments.
fn section_content(text: &str, section: &str) -> Option<String> {
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let (start, end) = section_of(&lines, section)?;
    let content: Vec<&str> = lines[(start + 1)..end]
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with(';'))
        .collect();
    Some(content.join("\n"))
}

/// The names of the sections of an ini file.
fn sections_of(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                Some(trimmed[1..trimmed.len() - 1].to_string())
            } else {
                None
            }
        })
        .collect()
}

/// The text of an ini file with a key of it written.
fn write_key(text: &str, ending: &str, section: &str, key: &str, value: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let pair = format!("{key}={value}");
    match section_of(&lines, section) {
        Some((start, end)) => {
            let mut key_at = None;
            for (offset, line) in lines[(start + 1)..end].iter().enumerate() {
                if let Some((name, _)) = line.split_once('=') {
                    if name.trim().eq_ignore_ascii_case(key) {
                        key_at = Some(start + 1 + offset);
                        break;
                    }
                }
            }
            match key_at {
                Some(index) => lines[index] = pair,
                None => lines.insert(end, pair),
            }
        }
        None => {
            lines.push(format!("[{section}]"));
            lines.push(pair);
        }
    }
    let mut written = lines.join(ending);
    if !written.is_empty() {
        written.push_str(ending);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahk::desktop::Files;

    /// A folder of its own for the case, in the temporary directory of
    /// the machine.
    fn scratch(name: &str) -> PathBuf {
        let folder = std::env::temp_dir().join(format!("ahk-box-files-{name}"));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("the folder of the case");
        folder
    }

    /// A file of the case, with something in it.
    fn file_of(folder: &Path, name: &str) -> String {
        let path = folder.join(name);
        fs::write(&path, "hello").expect("the file of the case");
        path.to_string_lossy().to_string()
    }

    #[test]
    fn a_copy_of_a_file_leaves_the_original_behind() {
        let folder = scratch("copy");
        let from = file_of(&folder, "a.txt");
        let to = folder.join("b.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.copy_file(&from, &to, false);
        assert!(answer.is_ok(), "{answer:?}");
        assert!(Path::new(&from).is_file());
        assert_eq!(fs::read_to_string(&to).expect("the copy"), "hello");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_copy_refuses_to_overwrite_without_the_flag() {
        let folder = scratch("copy-overwrite");
        let from = file_of(&folder, "a.txt");
        let to = file_of(&folder, "b.txt");
        let mut desktop = LabDesktop::new();
        let answer = desktop.copy_file(&from, &to, false);
        assert!(answer.is_err());
        assert_eq!(fs::read_to_string(&to).expect("the file"), "hello");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_copy_overwrites_with_the_flag() {
        let folder = scratch("copy-overwrite-yes");
        let from = file_of(&folder, "a.txt");
        let to = file_of(&folder, "b.txt");
        fs::write(&to, "older").expect("the file");
        let mut desktop = LabDesktop::new();
        let answer = desktop.copy_file(&from, &to, true);
        assert!(answer.is_ok(), "{answer:?}");
        assert_eq!(fs::read_to_string(&to).expect("the file"), "hello");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_copy_of_a_file_that_is_not_there_fails() {
        let folder = scratch("copy-missing");
        let from = folder.join("nope.txt").to_string_lossy().to_string();
        let to = folder.join("b.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        assert!(desktop.copy_file(&from, &to, false).is_err());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_move_takes_the_file_away() {
        let folder = scratch("move");
        let from = file_of(&folder, "a.txt");
        let to = folder.join("b.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.move_file(&from, &to, false);
        assert!(answer.is_ok(), "{answer:?}");
        assert!(!Path::new(&from).exists());
        assert_eq!(fs::read_to_string(&to).expect("the file"), "hello");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_move_refuses_to_overwrite_without_the_flag() {
        let folder = scratch("move-overwrite");
        let from = file_of(&folder, "a.txt");
        let to = file_of(&folder, "b.txt");
        let mut desktop = LabDesktop::new();
        assert!(desktop.move_file(&from, &to, false).is_err());
        assert!(Path::new(&from).exists());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_copy_of_a_folder_carries_the_files_in_it() {
        let folder = scratch("copy-folder");
        let source = folder.join("source");
        fs::create_dir_all(source.join("inner")).expect("the folder of the case");
        fs::write(source.join("one.txt"), "one").expect("the file of the case");
        fs::write(source.join("inner/two.txt"), "two").expect("the file of the case");
        let target = folder.join("target");
        let mut desktop = LabDesktop::new();
        let answer = desktop.copy_dir(
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            false,
        );
        assert!(answer.is_ok(), "{answer:?}");
        // v2 copies the contents of the folder, not the folder itself.
        assert!(target.join("one.txt").is_file());
        assert!(target.join("inner/two.txt").is_file());
        assert!(!target.join("source").exists());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_copy_of_a_folder_refuses_a_folder_that_is_there() {
        let folder = scratch("copy-folder-there");
        let source = folder.join("source");
        fs::create_dir_all(&source).expect("the folder of the case");
        fs::write(source.join("one.txt"), "one").expect("the file of the case");
        let target = folder.join("target");
        fs::create_dir_all(&target).expect("the folder of the case");
        let mut desktop = LabDesktop::new();
        let answer = desktop.copy_dir(
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            false,
        );
        assert!(answer.is_err());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_move_of_a_folder_takes_the_folder_away() {
        let folder = scratch("move-folder");
        let source = folder.join("source");
        fs::create_dir_all(&source).expect("the folder of the case");
        fs::write(source.join("one.txt"), "one").expect("the file of the case");
        let target = folder.join("moved");
        let mut desktop = LabDesktop::new();
        let answer = desktop.move_dir(
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            false,
        );
        assert!(answer.is_ok(), "{answer:?}");
        assert!(!source.exists());
        assert!(target.join("one.txt").is_file());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_attributes_of_a_file_are_the_letters_of_v2() {
        let folder = scratch("attrib");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let answer = desktop.file_attributes(&path).expect("the attributes");
        assert_eq!(answer, vec!["A".to_string()]);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_attributes_of_a_folder_carry_the_letter_d() {
        let folder = scratch("attrib-folder");
        let path = folder.to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.file_attributes(&path).expect("the attributes");
        assert_eq!(answer, vec!["D".to_string()]);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_read_only_attribute_is_turned_on_and_off() {
        let folder = scratch("attrib-set");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let answer = desktop.set_file_attributes(&path, &["+R".to_string()]);
        assert!(answer.is_ok(), "{answer:?}");
        assert_eq!(
            desktop.file_attributes(&path).expect("the attributes"),
            vec!["R".to_string()]
        );
        let answer = desktop.set_file_attributes(&path, &["-R".to_string()]);
        assert!(answer.is_ok(), "{answer:?}");
        assert_eq!(
            desktop.file_attributes(&path).expect("the attributes"),
            vec!["A".to_string()]
        );
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn an_attribute_the_machine_has_not_is_refused() {
        let folder = scratch("attrib-missing");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let answer = desktop.set_file_attributes(&path, &["+H".to_string()]);
        assert!(answer.is_err());
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_times_of_a_file_are_the_milliseconds_since_the_epoch() {
        let folder = scratch("times");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let (created, modified, accessed) = desktop.file_times(&path).expect("the times");
        // The file was made a moment ago: the time is after the year
        // two thousand, in the milliseconds since the epoch.
        assert!(created > 1_577_836_800_000, "{created}");
        assert!(modified > 0);
        assert!(accessed > 0);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_times_of_a_file_can_be_changed() {
        let folder = scratch("times-set");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let when = 1_577_836_800_000;
        let answer = desktop.set_file_times(&path, None, Some(when), None);
        assert!(answer.is_ok(), "{answer:?}");
        let (_, modified, _) = desktop.file_times(&path).expect("the times");
        // The file system keeps the time to the second, and the two
        // times are the same moment.
        assert!((modified - when).abs() < 2000, "{modified}");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_time_a_file_was_made_is_changed_where_the_machine_can() {
        let folder = scratch("times-created");
        let path = file_of(&folder, "a.txt");
        let mut desktop = LabDesktop::new();
        let when = 1_577_836_800_000;
        let answer = desktop.set_file_times(&path, Some(when), None, None);
        // The standard library writes the time a file was made on
        // Windows only: the machine of the box says so elsewhere.
        #[cfg(windows)]
        assert!(answer.is_ok(), "{answer:?}");
        #[cfg(not(windows))]
        assert_eq!(
            answer,
            Err("the machine of this box cannot change the time a file was made.".to_string())
        );
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn an_ini_file_is_written_and_read() {
        let folder = scratch("ini");
        let path = folder.join("a.ini").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.ini_write(&path, "Sec", "Key", "the value");
        assert!(answer.is_ok(), "{answer:?}");
        let read = desktop.ini_read(&path, "Sec", "Key", "").expect("the value");
        assert_eq!(read, "the value");
        // The file is the one of Windows: a section in brackets, and a
        // key with an equal sign.
        let text = fs::read_to_string(&path).expect("the file");
        assert!(text.contains("[Sec]"), "{text:?}");
        assert!(text.contains("Key=the value"), "{text:?}");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_key_that_is_not_in_the_ini_file_answers_with_the_default() {
        let folder = scratch("ini-default");
        let path = folder.join("a.ini").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        desktop.ini_write(&path, "Sec", "Key", "the value").expect("the write");
        let read = desktop.ini_read(&path, "Sec", "Nope", "fallback").expect("the value");
        assert_eq!(read, "fallback");
        // Without a default the key that is not there is the failure of
        // v2.
        let answer = desktop.ini_read(&path, "Sec", "Nope", "");
        assert_eq!(answer, Err(NOT_FOUND.to_string()));
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_key_is_taken_away_from_an_ini_file() {
        let folder = scratch("ini-delete");
        let path = folder.join("a.ini").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        desktop.ini_write(&path, "Sec", "Key", "the value").expect("the write");
        desktop.ini_write(&path, "Sec", "Other", "another").expect("the write");
        let answer = desktop.ini_delete(&path, "Sec", "Key");
        assert!(answer.is_ok(), "{answer:?}");
        assert_eq!(desktop.ini_read(&path, "Sec", "Key", "gone").expect("the value"), "gone");
        assert_eq!(desktop.ini_read(&path, "Sec", "Other", "").expect("the value"), "another");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn an_ini_file_of_v2_is_read() {
        let folder = scratch("ini-utf16");
        let path = folder.join("a.ini");
        // The file that v2 writes: UTF-16 with a byte order mark.
        let mut bytes: Vec<u8> = vec![0xFF, 0xFE];
        for letter in "[Sec]\r\nKey=the value\r\n".encode_utf16() {
            bytes.extend_from_slice(&letter.to_le_bytes());
        }
        fs::write(&path, bytes).expect("the file of the case");
        let mut desktop = LabDesktop::new();
        let read = desktop
            .ini_read(&path.to_string_lossy(), "Sec", "Key", "")
            .expect("the value");
        assert_eq!(read, "the value");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_sections_of_an_ini_file_are_read() {
        let folder = scratch("ini-sections");
        let path = folder.join("a.ini").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        desktop.ini_write(&path, "One", "Key", "a").expect("the write");
        desktop.ini_write(&path, "Two", "Key", "b").expect("the write");
        let sections = desktop.ini_read(&path, "", "", "").expect("the sections");
        assert_eq!(sections, "One\nTwo");
        let _ = fs::remove_dir_all(&folder);
    }
}
