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

/// The failure of v2 when a copy, a move or the delete of a folder does
/// not happen.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the dialect does not
/// say what went wrong, and the failure is of the class `Error`.
const FAILED: &str = "Failed";

/// The letters of the attributes that the filesystem of the box has.
///
/// The read-only one is the only bit of its own that the machine keeps;
/// the others are the ordinary state of a file, and a folder is a
/// folder.
const KNOWN: [char; 4] = ['R', 'A', 'N', 'D'];

impl ahk::desktop::Files for LabDesktop {
    fn create_shortcut(
        &mut self,
        target: &str,
        shortcut: &str,
        working_directory: &str,
        arguments: &str,
        description: &str,
        icon: &str,
    ) -> Result<(), String> {
        // A shortcut on this platform is a `.desktop` file, which is what the freedesktop
        // specifications make of one. The name of it is the description when there is
        // one and the target when there is not.
        let name = if description.is_empty() {
            target
        } else {
            description
        };
        let exec = if arguments.is_empty() {
            target.to_string()
        } else {
            format!("{target} {arguments}")
        };
        let mut text = String::new();
        text.push_str("[Desktop Entry]\n");
        text.push_str("Type=Application\n");
        text.push_str(&format!("Name={name}\n"));
        text.push_str(&format!("Exec={exec}\n"));
        if !working_directory.is_empty() {
            text.push_str(&format!("Path={working_directory}\n"));
        }
        if !icon.is_empty() {
            text.push_str(&format!("Icon={icon}\n"));
        }
        text.push_str("Terminal=false\n");
        std::fs::write(shortcut, text).map_err(|error| error.to_string())?;
        // A `.desktop` file that is not executable is not a shortcut.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(shortcut) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o755);
                let _ = std::fs::set_permissions(shortcut, permissions);
            }
        }
        Ok(())
    }

    fn empty_the_trash(&mut self) -> Result<(), String> {
        // The freedesktop way, which the box has: `gio trash --empty`.
        let output = std::process::Command::new("gio")
            .arg("trash")
            .arg("--empty")
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err("the trash was not emptied".to_string())
        }
    }

    fn recycle_a_file(&mut self, path: &str) -> Result<(), String> {
        // `gio trash <path>` is the same tool that empties the trash. A file
        // that is not there is not a failure of the tool: v2 answers the empty
        // text for it, and this tree does the same with a note rather than a
        // made-up success.
        if !std::path::Path::new(path).exists() {
            return Err(format!("the file {path} is not there"));
        }
        let output = std::process::Command::new("gio")
            .arg("trash")
            .arg(path)
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "the file was not recycled: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }

    fn read_shortcut(
        &mut self,
        shortcut: &str,
    ) -> Result<(String, String, String, String, String), String> {
        // The inverse of create_shortcut: read the desktop file back. The
        // Exec line carries the target and the arguments together, split at
        // the first space outside quotes; v2 keeps them apart.
        let text = std::fs::read_to_string(shortcut)
            .map_err(|error| format!("the shortcut {shortcut} could not be read: {error}"))?;
        let mut name = String::new();
        let mut exec = String::new();
        let mut path = String::new();
        let mut icon = String::new();
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("Name=") {
                name = value.to_string();
            } else if let Some(value) = line.strip_prefix("Exec=") {
                exec = value.to_string();
            } else if let Some(value) = line.strip_prefix("Path=") {
                path = value.to_string();
            } else if let Some(value) = line.strip_prefix("Icon=") {
                icon = value.to_string();
            }
        }
        if exec.is_empty() {
            return Err(format!(
                "the shortcut {shortcut} has no Exec line, so it is not a shortcut this tree made"
            ));
        }
        // The arguments are everything after the first space of the Exec line.
        let (target, arguments) = match exec.split_once(' ') {
            Some((target, rest)) if !rest.is_empty() => (target.to_string(), rest.to_string()),
            _ => (exec, String::new()),
        };
        Ok((target, path, arguments, name, icon))
    }

    fn open_file(&mut self, path: &str, mode: &str) -> Result<i64, String> {
        let mut options = std::fs::OpenOptions::new();
        match mode.to_lowercase().as_str() {
            "r" => {
                options.read(true);
            }
            "w" => {
                options.write(true).create(true).truncate(true);
            }
            "a" => {
                // The file is read as well as it is written: the bytes
                // of its head say the encoding of it.
                options.append(true).read(true).create(true);
            }
            "rw" | "r+" => {
                options.read(true).write(true).create(true);
            }
            _ => return Err(format!("the mode {mode:?} is not one this desktop knows")),
        }
        let file = options
            .open(path)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
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
        file.seek(SeekFrom::Start(pos.max(0) as u64))
            .map_err(|error| error.to_string())?;
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
        file.seek(SeekFrom::Start(pos.max(0) as u64))
            .map_err(|error| error.to_string())?;
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

    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        std::fs::remove_file(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(format!("deleted the file {path}"))
    }

    fn create_dir(&mut self, path: &str) -> Result<String, String> {
        std::fs::create_dir_all(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(format!("made the directory {path}"))
    }

    fn dir_delete(&mut self, path: &str, recurse: bool) -> Result<String, String> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the delete of a
        // folder of v2 does not take the contents of it with it, and a
        // folder that is not empty is not deleted.
        if recurse {
            std::fs::remove_dir_all(path).map_err(|_| FAILED.to_string())?;
        } else {
            std::fs::remove_dir(path).map_err(|_| FAILED.to_string())?;
        }
        Ok(format!("deleted the directory {path}"))
    }

    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        // The name of the directory is a subfolder of the working
        // directory when it is not an absolute path, the way v2 says it.
        let path = if Path::new(directory).is_absolute() {
            PathBuf::from(directory)
        } else {
            let here = std::env::current_dir().map_err(|error| error.to_string())?;
            here.join(directory)
        };
        std::env::set_current_dir(&path)
            .map_err(|error| the_failure_of_the_machine(path.to_string_lossy().as_ref(), &error))?;
        Ok(format!("the working directory is now {}", path.display()))
    }

    fn working_directory(&mut self) -> Result<String, String> {
        std::env::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|error| format!("the working directory cannot be read: {error}"))
    }

    fn initial_working_directory(&mut self) -> Result<String, String> {
        // The desktop read it when it was made, and it kept it: a script
        // that changes the working directory does not change this one.
        self.initial_working_dir
            .clone()
            .ok_or_else(|| "the directory where the run started cannot be read".to_string())
    }
    fn list_files(
        &mut self,
        folder: &str,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        let mut files: Vec<String> = Vec::new();
        // v2 takes a path with a wildcard: the folder of the path is where
        // the files are, and the rest of it is the pattern of the names.
        // A folder that is not absolute is a subfolder of the folder the
        // script runs in.
        let (where_to_look, name) = split_the_pattern(pattern);
        let where_to_look = if where_to_look.is_empty() {
            PathBuf::from(folder)
        } else if Path::new(&where_to_look).is_absolute() {
            PathBuf::from(&where_to_look)
        } else {
            PathBuf::from(folder).join(&where_to_look)
        };
        crate::desktop::walk_the_folder(
            &where_to_look,
            &name,
            &mode.to_ascii_uppercase(),
            &mut files,
        )?;
        // The order of a directory is not the order of its name, and a
        // script that loops over the files wants the same order every
        // time. The walk sorts the entries of every folder that it visits.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the files of a
        // folder of NTFS come in the order of the name with the case taken
        // away, and the tree sorts them the same way.
        Ok(files)
    }
    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(the_lines_of(&text))
    }
    fn read_the_bytes_of_a_file(&mut self, path: &str) -> Result<Vec<u8>, String> {
        match fs::read(path) {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(the_failure_of_the_machine(path, &error)),
        }
    }
    fn write_file(&mut self, path: &str, text: &str) -> Result<String, String> {
        std::fs::write(path, text).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(text.to_string())
    }
    fn append_file(&mut self, path: &str, bytes: &[u8], mark: &[u8]) -> Result<String, String> {
        use std::io::Write;
        // The mark of the encoding is written when the file is made, and not
        // when the file is already there.
        let made = !Path::new(path).exists();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        if made && !mark.is_empty() {
            file.write_all(mark)
                .map_err(|error| the_failure_of_the_machine(path, &error))?;
        }
        file.write_all(bytes)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(bytes.len().to_string())
    }
    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_file() {
            return Err(FAILED.to_string());
        }
        let target = target_of(source, Path::new(to));
        if target.exists() && !overwrite {
            return Err(FAILED.to_string());
        }
        fs::copy(source, &target).map_err(|_| FAILED.to_string())?;
        Ok(format!(
            "copied the file {from} to {}",
            target.to_string_lossy()
        ))
    }

    fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.exists() {
            return Err(FAILED.to_string());
        }
        let target = target_of(source, Path::new(to));
        if target.exists() {
            if !overwrite {
                return Err(FAILED.to_string());
            }
            fs::remove_file(&target).map_err(|_| FAILED.to_string())?;
        }
        // The rename is the move, unless the file goes to another volume:
        // the copy and the delete are the move then.
        if fs::rename(source, &target).is_err() {
            fs::copy(source, &target).map_err(|_| FAILED.to_string())?;
            fs::remove_file(source).map_err(|_| FAILED.to_string())?;
        }
        Ok(format!(
            "moved the file {from} to {}",
            target.to_string_lossy()
        ))
    }

    fn copy_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_dir() {
            return Err(FAILED.to_string());
        }
        let target = Path::new(to);
        if target.exists() {
            if !overwrite {
                return Err(FAILED.to_string());
            }
        } else {
            fs::create_dir_all(target).map_err(|_| FAILED.to_string())?;
        }
        // A folder that is copied into itself is a copy that never ends:
        // the folder that was made is inside the one that is copied, and
        // the copy walks into it.
        //
        // Measured on the binary of v2.0.19 on 2026-11-01: the copy of a
        // folder into itself fails with "Failed", and the folder that was
        // made is there all the same.
        if is_inside(source, target) {
            return Err(FAILED.to_string());
        }
        // v2 copies the *contents* of the folder: the folder that is
        // there is the one that holds the files.
        copy_tree(source, target, overwrite).map_err(|_| FAILED.to_string())?;
        Ok(format!("copied the folder {from} to {to}"))
    }

    fn move_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = Path::new(from);
        if !source.is_dir() {
            return Err(FAILED.to_string());
        }
        let target = Path::new(to);
        if target.exists() {
            if !overwrite {
                return Err(FAILED.to_string());
            }
            // The folder that is there is merged with the one that
            // comes: the files of the source win, and the ones that are
            // only in the target stay.
            copy_tree(source, target, true).map_err(|_| FAILED.to_string())?;
            fs::remove_dir_all(source).map_err(|_| FAILED.to_string())?;
            return Ok(format!("moved the folder {from} to {to}"));
        }
        fs::rename(source, target).map_err(|_| FAILED.to_string())?;
        Ok(format!("moved the folder {from} to {to}"))
    }

    fn attributes_of_the_pattern(
        &mut self,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        let (folder, name) = split_the_pattern(pattern);
        // A name without a wildcard is the path itself: the entry is
        // there or it is not.
        if !name.contains('*') && !name.contains('?') {
            let Ok(metadata) = fs::metadata(Path::new(pattern)) else {
                return Ok(Vec::new());
            };
            if mode.contains('D') && !metadata.is_dir() {
                return Ok(Vec::new());
            }
            return Ok(letters_of(&metadata)
                .into_iter()
                .map(|letter| letter.to_string())
                .collect());
        }
        // A pattern with a wildcard is looked for in the folder of it, and
        // the first entry that matches is the one that answers.
        let folder = if folder.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&folder)
        };
        let Ok(entries) = fs::read_dir(&folder) else {
            return Ok(Vec::new());
        };
        let mut found: Vec<(String, fs::Metadata)> = Vec::new();
        for entry in entries.flatten() {
            let name_of_the_entry = entry.file_name().to_string_lossy().to_string();
            if !crate::desktop::name_matches(&name, &name_of_the_entry) {
                continue;
            }
            let Ok(metadata) = fs::metadata(entry.path()) else {
                continue;
            };
            if mode.contains('D') && !metadata.is_dir() {
                continue;
            }
            found.push((name_of_the_entry, metadata));
        }
        // The order is the order of the names, the way v2 takes them: the
        // name with the case taken away, and the names of a folder of NTFS
        // come in that order.
        found.sort_by(|one, other| {
            one.0
                .to_uppercase()
                .cmp(&other.0.to_uppercase())
                .then_with(|| one.0.cmp(&other.0))
        });
        Ok(match found.into_iter().next() {
            Some((_, metadata)) => letters_of(&metadata)
                .into_iter()
                .map(|letter| letter.to_string())
                .collect(),
            None => Vec::new(),
        })
    }

    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        let metadata =
            fs::metadata(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(letters_of(&metadata)
            .into_iter()
            .map(|letter| letter.to_string())
            .collect())
    }

    fn the_size_of_a_file(&mut self, path: &str) -> Result<i64, String> {
        let metadata =
            fs::metadata(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        // A folder has no size: the machine gives zero for one.
        if metadata.is_dir() {
            return Ok(0);
        }
        Ok(metadata.len() as i64)
    }

    fn set_file_attributes(&mut self, path: &str, attributes: &[String]) -> Result<String, String> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the attributes of
        // a file that is not there are not a failure: nothing happens.
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(format!(
                    "the attributes of the file {path:?} are not changed"
                ));
            }
            Err(error) => return Err(the_failure_of_the_machine(path, &error)),
        };
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
                return Err(format!(
                    "the machine of this box has no such attribute: {letter}."
                ));
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
            fs::metadata(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
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
        // Measured on the binary of v2.0.19 on 2026-10-31: the time of a
        // file that is not there is not a failure: nothing happens.
        let file = match fs::OpenOptions::new().write(true).open(path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(format!("the times of the file {path:?} are not changed"));
            }
            Err(error) => return Err(the_failure_of_the_machine(path, &error)),
        };
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
        file.set_times(times).map_err(|error| {
            format!("the times of the file {path:?} cannot be changed: {error}")
        })?;
        Ok(format!("the times of the file {path:?} are set"))
    }

    fn ini_read(
        &mut self,
        path: &str,
        section: &str,
        key: &str,
        default: &str,
    ) -> Result<String, String> {
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

    fn ini_write(
        &mut self,
        path: &str,
        section: &str,
        key: &str,
        value: &str,
    ) -> Result<String, String> {
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
        Ok(format!(
            "took the key {key:?} of the section {section:?} away"
        ))
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

/// Whether the target is the source or is inside it.
///
/// The paths are the ones that the machine gives, so that a folder that is
/// reached in two ways is seen as one.
fn is_inside(source: &Path, target: &Path) -> bool {
    let Ok(source) = source.canonicalize() else {
        return false;
    };
    let Ok(target) = target.canonicalize() else {
        return false;
    };
    target == source || target.starts_with(&source)
}

/// The files of a folder, and the folders in it, copied into another.
///
/// The files that are already there are left alone unless the copy is
/// told to overwrite them.
fn copy_tree(source: &Path, target: &Path, overwrite: bool) -> Result<(), String> {
    let entries = fs::read_dir(source)
        .map_err(|error| format!("the folder {source:?} cannot be read: {error}"))?;
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
    } else {
        // The archive bit is what a file carries on Windows, and the
        // machine of the box has no bit of its own for it. A file that is
        // read-only keeps it: v2 answers "RA" for one.
        letters.push('A');
    }
    letters
}

/// The folder of a pattern and the name of it.
///
/// The name is the last part of the path, and it is the part that may
/// carry the wildcards. The separators are the ones of the machine: a
/// slash everywhere, and a backslash on Windows as well.
fn split_the_pattern(pattern: &str) -> (String, String) {
    let at = pattern.rfind(|letter: char| letter == '/' || (cfg!(windows) && letter == '\\'));
    match at {
        Some(at) => (pattern[..at].to_string(), pattern[at + 1..].to_string()),
        None => (String::new(), pattern.to_string()),
    }
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
        let units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
            .collect();
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

/// The failure of the machine, in the words of v2.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the failure of the
/// machine is the code of the system in front of what it says: "(2) The
/// system cannot find the file specified.".
fn the_failure_of_the_machine(path: &str, error: &std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::NotFound => {
            // The file is not there, or the folder of it is not.
            let folder = Path::new(path).parent().unwrap_or(Path::new(""));
            if folder.as_os_str().is_empty() || folder.exists() {
                "(2) The system cannot find the file specified.".to_string()
            } else {
                "(3) The system cannot find the path specified.".to_string()
            }
        }
        std::io::ErrorKind::PermissionDenied => "(5) Access is denied.".to_string(),
        _ => {
            // A folder that is read is a folder that cannot be read.
            if Path::new(path).is_dir() {
                return "(5) Access is denied.".to_string();
            }
            match error.raw_os_error() {
                Some(code) => format!("({code}) {error}"),
                None => format!("({error})"),
            }
        }
    }
}

/// The end of the lines of a text.
///
/// The files of Windows end their lines with two characters, and the
/// files of the box with one: a file that is read and written again
/// keeps the way it was written.
fn ending_of(text: &str) -> &'static str {
    if text.contains("\r\n") { "\r\n" } else { "\n" }
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
        let Some((name, value)) = trimmed.split_once('=') else {
            continue;
        };
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

/// The lines of a text, the way v2 counts them.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the linefeed, the
/// carriage return and the two together end a line. A file that ends with
/// an ending has no empty line at its end, and a file that ends with two
/// of them has one.
fn the_lines_of(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '\r' => {
                if characters.peek() == Some(&'\n') {
                    characters.next();
                }
                lines.push(current.clone());
                current.clear();
            }
            '\n' => {
                lines.push(current.clone());
                current.clear();
            }
            other => current.push(other),
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
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
    fn the_failure_of_a_copy_is_the_failure_of_v2() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the copy that
        // does not happen fails with "Failed", and the dialect does not
        // say what went wrong.
        let folder = scratch("copy-failed");
        let from = folder.join("nope.txt").to_string_lossy().to_string();
        let to = folder.join("b.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        assert_eq!(
            desktop.copy_file(&from, &to, false),
            Err("Failed".to_string())
        );
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_lines_of_a_file_are_the_lines_of_v2() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the linefeed,
        // the carriage return and the two together end a line, and a file
        // that ends with an ending has no empty line at its end.
        assert_eq!(the_lines_of("one\r\ntwo\r\n"), vec!["one", "two"]);
        assert_eq!(the_lines_of("one\r\ntwo"), vec!["one", "two"]);
        assert_eq!(the_lines_of("one\r\n\r\n"), vec!["one", ""]);
        assert_eq!(the_lines_of("one\r\n"), vec!["one"]);
        assert_eq!(the_lines_of("one"), vec!["one"]);
        assert_eq!(the_lines_of("one\rtwo\r"), vec!["one", "two"]);
        assert!(the_lines_of("").is_empty());
    }

    #[test]
    fn the_file_of_the_machine_has_the_lines_that_v2_sees() {
        let folder = scratch("read-lines");
        let path = folder.join("lines.txt");
        fs::write(&path, "one\r\ntwo\r\n").expect("the file of the case");
        let mut desktop = LabDesktop::new();
        let lines = desktop
            .read_file(&path.to_string_lossy())
            .expect("the lines");
        assert_eq!(lines, vec!["one".to_string(), "two".to_string()]);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_copy_of_a_folder_into_itself_fails() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the copy of a
        // folder into itself fails with "Failed", and the folder that was
        // made is there all the same.
        let folder = scratch("copy-into-itself");
        let source = folder.join("src");
        fs::create_dir_all(&source).expect("the folder of the case");
        fs::write(source.join("inner.txt"), "hello").expect("the file of the case");
        let target = source.join("made");
        let mut desktop = LabDesktop::new();
        assert_eq!(
            desktop.copy_dir(&source.to_string_lossy(), &target.to_string_lossy(), false),
            Err("Failed".to_string())
        );
        assert!(target.is_dir(), "the folder that was made is there");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_failure_of_the_machine_carries_the_code_of_the_system() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the failure of
        // the machine is the code of the system in front of what it says.
        let folder = scratch("delete-missing");
        let path = folder.join("nope.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        assert_eq!(
            desktop.delete_file(&path),
            Err("(2) The system cannot find the file specified.".to_string())
        );
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_delete_of_a_folder_that_is_not_empty_fails() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the delete of a
        // folder of v2 does not take the contents of it with it.
        let folder = scratch("dir-delete-full");
        let with = folder.join("with");
        fs::create_dir_all(&with).expect("the folder of the case");
        fs::write(with.join("inner.txt"), "hello").expect("the file of the case");
        let mut desktop = LabDesktop::new();
        assert_eq!(
            desktop.dir_delete(&with.to_string_lossy(), false),
            Err("Failed".to_string())
        );
        assert!(with.is_dir(), "the folder is still there");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_delete_of_a_folder_with_the_recursion_takes_the_contents_with_it() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the second
        // parameter of the delete of a folder is the recursion.
        let folder = scratch("dir-delete-recurse");
        let with = folder.join("with");
        fs::create_dir_all(&with).expect("the folder of the case");
        fs::write(with.join("inner.txt"), "hello").expect("the file of the case");
        let mut desktop = LabDesktop::new();
        let answer = desktop.dir_delete(&with.to_string_lossy(), true);
        assert!(answer.is_ok(), "{answer:?}");
        assert!(!with.exists(), "the folder is gone");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_attributes_of_a_file_that_is_not_there_are_not_a_failure() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the attributes
        // of a file that is not there are not a failure: nothing happens.
        let folder = scratch("attributes-missing");
        let path = folder.join("nope.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.set_file_attributes(&path, &["+R".to_string()]);
        assert!(answer.is_ok(), "{answer:?}");
        assert!(!Path::new(&path).exists(), "the file was not made");
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_time_of_a_file_that_is_not_there_is_not_a_failure() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the time of a
        // file that is not there is not a failure: nothing happens.
        let folder = scratch("times-missing");
        let path = folder.join("nope.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop.set_file_times(&path, None, Some(1577836800000), None);
        assert!(answer.is_ok(), "{answer:?}");
        assert!(!Path::new(&path).exists(), "the file was not made");
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
        let answer = desktop.copy_dir(&source.to_string_lossy(), &target.to_string_lossy(), false);
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
        let answer = desktop.copy_dir(&source.to_string_lossy(), &target.to_string_lossy(), false);
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
        let answer = desktop.move_dir(&source.to_string_lossy(), &target.to_string_lossy(), false);
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
        // Measured on the binary of v2.0.19 on 2026-10-31: a file that is
        // read-only keeps the archive bit, and v2 answers "RA".
        assert_eq!(
            desktop.file_attributes(&path).expect("the attributes"),
            vec!["R".to_string(), "A".to_string()]
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
    fn the_pattern_answers_the_first_entry_that_matches() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the pattern
        // may carry the wildcards, and the answer is the attributes of
        // the first entry that matches, in the order of the names.
        let folder = scratch("pattern");
        file_of(&folder, "b.txt");
        file_of(&folder, "a.txt");
        let pattern = folder.join("*.txt").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop
            .attributes_of_the_pattern(&pattern, "")
            .expect("the attributes");
        assert_eq!(answer, vec!["A".to_string()]);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_pattern_of_a_folder_skips_the_files() {
        let folder = scratch("pattern-folder");
        file_of(&folder, "a.txt");
        fs::create_dir_all(folder.join("sub")).expect("the folder of the case");
        let pattern = folder.join("*").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        // The file comes first in the order of the names, and it is not a
        // folder: the mode "D" skips it, and the folder answers.
        let answer = desktop
            .attributes_of_the_pattern(&pattern, "D")
            .expect("the attributes");
        assert_eq!(answer, vec!["D".to_string()]);
        // Without the mode, the file is the one that answers.
        let answer = desktop
            .attributes_of_the_pattern(&pattern, "")
            .expect("the attributes");
        assert_eq!(answer, vec!["A".to_string()]);
        let _ = fs::remove_dir_all(&folder);
    }

    #[test]
    fn the_pattern_that_matches_nothing_answers_nothing() {
        let folder = scratch("pattern-empty");
        let pattern = folder.join("*.nope").to_string_lossy().to_string();
        let mut desktop = LabDesktop::new();
        let answer = desktop
            .attributes_of_the_pattern(&pattern, "")
            .expect("the attributes");
        assert!(answer.is_empty(), "{answer:?}");
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
        let read = desktop
            .ini_read(&path, "Sec", "Key", "")
            .expect("the value");
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
        desktop
            .ini_write(&path, "Sec", "Key", "the value")
            .expect("the write");
        let read = desktop
            .ini_read(&path, "Sec", "Nope", "fallback")
            .expect("the value");
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
        desktop
            .ini_write(&path, "Sec", "Key", "the value")
            .expect("the write");
        desktop
            .ini_write(&path, "Sec", "Other", "another")
            .expect("the write");
        let answer = desktop.ini_delete(&path, "Sec", "Key");
        assert!(answer.is_ok(), "{answer:?}");
        assert_eq!(
            desktop
                .ini_read(&path, "Sec", "Key", "gone")
                .expect("the value"),
            "gone"
        );
        assert_eq!(
            desktop
                .ini_read(&path, "Sec", "Other", "")
                .expect("the value"),
            "another"
        );
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
        desktop
            .ini_write(&path, "One", "Key", "a")
            .expect("the write");
        desktop
            .ini_write(&path, "Two", "Key", "b")
            .expect("the write");
        let sections = desktop.ini_read(&path, "", "", "").expect("the sections");
        assert_eq!(sections, "One\nTwo");
        let _ = fs::remove_dir_all(&folder);
    }

    /// The order of the files of a folder is the order of the name with
    /// the case taken away, and the names are compared byte-wise.
    /// Measured on the binary of v2.0.19 on 2026-10-31.
    #[test]
    fn the_files_of_a_folder_come_in_the_order_of_v2() {
        let folder = scratch("order");
        for name in [
            "b.txt", "A.txt", "a1.txt", "ab.txt", "a_b.txt", "10.txt", "2.txt",
        ] {
            file_of(&folder, name);
        }
        let mut desktop = LabDesktop::new();
        let files = desktop
            .list_files(&folder.to_string_lossy(), "*", "")
            .expect("the files of the folder");
        let names: Vec<String> = files
            .iter()
            .map(|file| {
                Path::new(file)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        assert_eq!(
            names,
            vec![
                "10.txt", "2.txt", "A.txt", "a1.txt", "ab.txt", "a_b.txt", "b.txt"
            ]
        );
        let _ = fs::remove_dir_all(&folder);
    }

    /// The pattern of a loop over the files is a path: the folder of it is
    /// where the files are, and the rest of it is the name.
    /// Measured on the binary of v2.0.19 on 2026-10-31.
    #[test]
    fn the_pattern_of_the_files_is_a_path() {
        let folder = scratch("pattern-path");
        let sub = folder.join("sub");
        fs::create_dir_all(&sub).expect("the subfolder");
        file_of(&folder, "a.txt");
        file_of(&sub, "b.txt");
        let mut desktop = LabDesktop::new();
        let separator = std::path::MAIN_SEPARATOR;
        let pattern = format!("{}{separator}sub{separator}*", folder.to_string_lossy());
        let files = desktop
            .list_files(&folder.to_string_lossy(), &pattern, "")
            .expect("the files of the folder");
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("b.txt"), "{files:?}");
        let _ = fs::remove_dir_all(&folder);
    }

    /// The mode of the loop of the files is a set of letters.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the "D"
    /// gives the folders, the "K" gives them too, the "F" and the "V" give the
    /// files, and the "FD" gives the two of them.
    #[test]
    fn the_mode_of_the_loop_of_the_files_is_a_set_of_letters() {
        let folder = scratch("modes");
        let sub = folder.join("sub");
        fs::create_dir_all(&sub).expect("the subfolder");
        file_of(&folder, "a.txt");
        file_of(&sub, "b.txt");
        let mut desktop = LabDesktop::new();
        let mut names = |mode: &str| {
            let files = desktop
                .list_files(&folder.to_string_lossy(), "*", mode)
                .expect("the files of the folder");
            files
                .iter()
                .map(|file| {
                    Path::new(file)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .collect::<Vec<String>>()
        };
        assert_eq!(names("D"), vec!["sub"]);
        assert_eq!(names("K"), vec!["sub"]);
        assert_eq!(names("F"), vec!["a.txt"]);
        assert_eq!(names("V"), vec!["a.txt"]);
        assert_eq!(names("FD"), vec!["a.txt", "sub"]);
        assert_eq!(names(""), vec!["a.txt"]);
        let _ = fs::remove_dir_all(&folder);
    }
}
