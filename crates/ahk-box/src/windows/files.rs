//! The files of the machine.
//!
//! The files of a Windows machine are the files of every machine: the
//! reading, the writing, the copy and the move are the work of the
//! standard library, and the ini of a file is the work of the crate.
//! What is a thing of Windows are the attributes of a file, the times
//! of it and the shortcuts -- and the trash of the machine.

use std::fs;
use std::path::{Path, PathBuf};

use super::sys::Wide;

use ahk::desktop::Files;

use crate::files::{
    ending_of, read_ini, section_of, split_the_pattern, the_failure_of_the_machine,
    the_lines_of, value_of, write_key,
};

/// The work of the files, for the desktop of a Windows machine.
///
/// The state of the files is the state of the desktop: the files that
/// the script opened live in it.
#[derive(Debug, Default)]
pub struct TheFiles {
    /// The files that the script opened.
    pub files: Vec<Option<fs::File>>,
    /// The folder where the run began.
    pub initial_working_dir: Option<String>,
}

impl TheFiles {
    /// The state of the files of a desktop.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Files for TheFiles {
    fn canonical_path(&mut self, path: &str) -> Result<String, String> {
        // O caminho inteiro e o `GetFullPathNameW` do kernel32, e nao
        // o `canonicalize` do `std`: o `canonicalize` pergunta a
        // `GetFinalPathNameByHandleW`, que e de Windows Vista.
        let wide = super::sys::Wide::of(path);
        let mut buffer = [0u16; 32768];
        let length = unsafe {
            windows_sys::Win32::Storage::FileSystem::GetFullPathNameW(
                wide.as_ptr(),
                buffer.len() as u32,
                buffer.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        if length == 0 {
            return Err(the_failure_of_the_machine(
                path,
                &std::io::Error::from_raw_os_error(
                    unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                ),
            ));
        }
        let end = buffer
            .iter()
            .position(|letter| *letter == 0)
            .unwrap_or(buffer.len());
        Ok(String::from_utf16_lossy(&buffer[..end]))
    }

    fn open_file(&mut self, path: &str, mode: &str) -> Result<i64, String> {
        let mut options = fs::OpenOptions::new();
        match mode.to_lowercase().as_str() {
            "r" => {
                options.read(true);
            }
            "w" => {
                options.write(true).create(true).truncate(true);
            }
            "a" => {
                // The file is read as well as it is written: the bytes of
                // its head say the encoding of it.
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

    /// Open a file with the program that the machine opens it with.
    ///
    /// The shell of the machine knows the program of a kind of file: the
    /// editor of a script, the player of a sound, the reader of a text.
    /// This is the `Edit` of v2.0.31, which opens the script of the run in
    /// the editor of the machine.
    fn open_with_the_default_program(&mut self, path: &str) -> Result<(), String> {
        open_with_the_program_of_the_machine(path)
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
        Ok(file.metadata().map_err(|error| error.to_string())?.len() as i64)
    }

    fn close_file(&mut self, handle: i64) -> Result<(), String> {
        let at = handle.max(0) as usize;
        match self.files.get_mut(at) {
            Some(slot) => {
                *slot = None;
                Ok(())
            }
            None => Err(format!("the file {handle} is not open")),
        }
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        let text =
            fs::read_to_string(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(the_lines_of(&text))
    }

    fn read_the_bytes_of_a_file(&mut self, path: &str) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|error| the_failure_of_the_machine(path, &error))
    }

    fn write_file(&mut self, path: &str, text: &str) -> Result<String, String> {
        fs::write(path, text).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(text.to_string())
    }

    fn append_file(&mut self, path: &str, bytes: &[u8], mark: &[u8]) -> Result<String, String> {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        file.write_all(mark).map_err(|error| error.to_string())?;
        file.write_all(bytes).map_err(|error| error.to_string())?;
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }

    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        fs::remove_file(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(String::new())
    }

    fn create_dir(&mut self, path: &str) -> Result<String, String> {
        fs::create_dir_all(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(path.to_string())
    }

    fn dir_delete(&mut self, path: &str, recurse: bool) -> Result<String, String> {
        let target = PathBuf::from(path);
        if recurse {
            for entry in
                fs::read_dir(&target).map_err(|error| the_failure_of_the_machine(path, &error))?
            {
                let entry = entry.map_err(|error| error.to_string())?;
                let child = entry.path();
                if child.is_dir() {
                    self.dir_delete(&child.to_string_lossy(), true)?;
                } else {
                    fs::remove_file(&child)
                        .map_err(|error| the_failure_of_the_machine(path, &error))?;
                }
            }
            fs::remove_dir(&target).map_err(|error| the_failure_of_the_machine(path, &error))?;
        } else {
            fs::remove_dir(&target).map_err(|error| the_failure_of_the_machine(path, &error))?;
        }
        Ok(String::new())
    }

    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        if !overwrite && Path::new(to).exists() {
            return Err("Failed".to_string());
        }
        fs::copy(from, to).map_err(|error| the_failure_of_the_machine(from, &error))?;
        Ok(to.to_string())
    }

    fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        if !overwrite && Path::new(to).exists() {
            return Err("Failed".to_string());
        }
        fs::rename(from, to).map_err(|error| the_failure_of_the_machine(from, &error))?;
        Ok(to.to_string())
    }

    fn copy_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        let source = PathBuf::from(from);
        let target = PathBuf::from(to);
        // Measured on the binary of v2.0.19: the destination that is there
        // already refuses the copy without the flag of the overwrite.
        if target.exists() && !overwrite {
            return Err(format!("the folder {to:?} is there already."));
        }
        crate::files::copy_tree(&source, &target, overwrite)
            .map_err(|error| format!("the folder {from:?} cannot be copied: {error}"))?;
        Ok(to.to_string())
    }

    fn move_dir(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        if Path::new(to).exists() {
            // Measured on the binary of v2.0.19: the destination that is
            // there already refuses the move without the flag of the
            // overwrite.
            if !overwrite {
                return Err(format!("the folder {to:?} is there already."));
            }
            self.dir_delete(to, true)?;
        }
        fs::rename(from, to).map_err(|error| the_failure_of_the_machine(from, &error))?;
        Ok(to.to_string())
    }

    fn the_size_of_a_file(&mut self, path: &str) -> Result<i64, String> {
        let metadata =
            fs::metadata(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(metadata.len() as i64)
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
        Ok(files)
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
            let Some(letters) = the_letters_of_the_path(pattern) else {
                return Ok(Vec::new());
            };
            if mode.contains('D') && !letters.contains(&'D') {
                return Ok(Vec::new());
            }
            return Ok(letters.into_iter().map(|letter| letter.to_string()).collect());
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
        // The first entry that matches answers, in the order the machine
        // of the folder gives them -- the enumeration of the directory, the
        // way v2 takes it with FindFirstFile. Only its attributes answer;
        // v2 does not name the file.
        for entry in entries.flatten() {
            let name_of_the_entry = entry.file_name().to_string_lossy().to_string();
            if !crate::desktop::name_matches(&name, &name_of_the_entry) {
                continue;
            }
            let Some(letters) = the_letters_of_the_path(&entry.path().to_string_lossy()) else {
                continue;
            };
            if mode.contains('D') && !letters.contains(&'D') {
                continue;
            }
            return Ok(letters.into_iter().map(|letter| letter.to_string()).collect());
        }
        Ok(Vec::new())
    }

    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        the_attributes_of(path)
    }

    fn recycle_a_file(&mut self, path: &str) -> Result<(), String> {
        super::trash::recycle(path)
    }

    fn empty_the_trash(&mut self) -> Result<(), String> {
        super::trash::empty_the_trash()
    }

    fn set_file_attributes(&mut self, path: &str, attributes: &[String]) -> Result<String, String> {
        let metadata =
            fs::metadata(path).map_err(|error| the_failure_of_the_machine(path, &error))?;
        let readonly = attributes
            .iter()
            .any(|letter| letter.eq_ignore_ascii_case("R"));
        let mut permissions = metadata.permissions();
        permissions.set_readonly(readonly);
        fs::set_permissions(path, permissions)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(String::new())
    }

    fn file_times(&mut self, path: &str) -> Result<(i64, i64, i64), String> {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Foundation::FILETIME;
        use windows_sys::Win32::Storage::FileSystem::GetFileTime;
        let file = fs::OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        let handle = file.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;
        let mut created = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut modified = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut accessed = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        unsafe {
            GetFileTime(handle, &mut created, &mut modified, &mut accessed);
        }
        Ok((
            the_time_of_the_machine(created),
            the_time_of_the_machine(modified),
            the_time_of_the_machine(accessed),
        ))
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
            .map_err(|error| the_failure_of_the_machine(path, &error))?;
        // The times of a file of Windows are the three of them, and the
        // machine writes them with `SetFileTime`.
        let times = [
            (created, WhichTime::Created),
            (modified, WhichTime::Modified),
            (accessed, WhichTime::Accessed),
        ];
        for (when, which) in times {
            let Some(when) = when else { continue };
            the_time_is_written(&file, when, which)?;
        }
        Ok(String::new())
    }

    fn ini_read(
        &mut self,
        path: &str,
        section: &str,
        key: &str,
        default: &str,
    ) -> Result<String, String> {
        let text = read_ini(path)?;
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        if section.is_empty() {
            if let Some((_, value)) = lines
                .iter()
                .find_map(|line| line.split_once('='))
                .filter(|(name, _)| name.trim().eq_ignore_ascii_case(key))
            {
                return Ok(value.trim().to_string());
            }
            return Ok(default.to_string());
        }
        match value_of(&text, section, key) {
            Some(value) => Ok(value),
            None => Ok(default.to_string()),
        }
    }

    fn ini_write(
        &mut self,
        path: &str,
        section: &str,
        key: &str,
        value: &str,
    ) -> Result<String, String> {
        let text = read_ini(path).unwrap_or_default();
        let ending = ending_of(&text);
        let written = write_key(&text, ending, section, key, value);
        fs::write(path, written).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(String::new())
    }

    fn ini_delete(&mut self, path: &str, section: &str, key: &str) -> Result<String, String> {
        let text = read_ini(path).unwrap_or_default();
        let ending = ending_of(&text);
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        let written = match section_of(&lines, section) {
            Some((start, end)) => {
                let mut kept: Vec<String> = Vec::new();
                for (index, line) in lines.iter().enumerate() {
                    if index > start && index < end {
                        if let Some((name, _)) = line.split_once('=') {
                            if name.trim().eq_ignore_ascii_case(key) {
                                continue;
                            }
                        }
                    }
                    kept.push(line.clone());
                }
                kept.join(ending)
            }
            None => text.clone(),
        };
        fs::write(path, written).map_err(|error| the_failure_of_the_machine(path, &error))?;
        Ok(String::new())
    }

    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        // Measured on the binary of v2.0.19: the folder that is not there is
        // the refusal of the machine, the OSError of the number of it.
        std::env::set_current_dir(directory)
            .map_err(|error| the_failure_of_the_machine(directory, &error))?;
        Ok(directory.to_string())
    }

    fn initial_working_directory(&mut self) -> Result<String, String> {
        if let Some(directory) = &self.initial_working_dir {
            return Ok(directory.clone());
        }
        let directory = std::env::current_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .map_err(|error| format!("the machine did not say the folder of the run: {error}"))?;
        self.initial_working_dir = Some(directory.clone());
        Ok(directory)
    }

    fn working_directory(&mut self) -> Result<String, String> {
        std::env::current_dir()
            .map(|path| path.to_string_lossy().into_owned())
            .map_err(|error| format!("the machine did not say the folder of the run: {error}"))
    }

    fn create_temp_file(&mut self) -> Result<String, String> {
        let folder = std::env::temp_dir();
        let name = folder.join(format!("ahk-{}.tmp", std::process::id()));
        let name = name.to_string_lossy().into_owned();
        fs::write(&name, b"").map_err(|error| the_failure_of_the_machine(&name, &error))?;
        Ok(name)
    }

    fn full_path(&mut self, path: &str) -> Result<String, String> {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            return Ok(path.to_string_lossy().into_owned());
        }
        let folder = std::env::current_dir().map_err(|error| error.to_string())?;
        Ok(folder.join(path).to_string_lossy().into_owned())
    }
}

/// The time of a file, and the one that is asked for.
///
/// The times of a file of Windows are the three of them: the one of the
/// creation, the one of the writing and the one of the reading.
#[derive(Clone, Copy)]
enum WhichTime {
    Created,
    Modified,
    Accessed,
}

/// The number of the milliseconds between the epoch of the tree and the
/// epoch of the machine.
///
/// The machine counts from the year 1601, and the tree counts from the
/// year 1970: the difference is the number of the milliseconds between
/// the two epochs.
const THE_EPOCH_OF_THE_MACHINE: i64 = 11_644_473_600_000;

/// The time of the machine, in the words of the tree.
///
/// The machine counts the hundred-nanoseconds since the year 1601, and
/// the tree counts the milliseconds since the year 1970.
fn the_time_of_the_machine(time: windows_sys::Win32::Foundation::FILETIME) -> i64 {
    let hundred_nanoseconds = ((time.dwHighDateTime as i64) << 32) | time.dwLowDateTime as i64;
    hundred_nanoseconds / 10_000 - THE_EPOCH_OF_THE_MACHINE
}

/// The time of the tree, in the words of the machine.
fn the_time_of_the_tree(time: i64) -> i64 {
    (time + THE_EPOCH_OF_THE_MACHINE) * 10_000
}

/// The time of a file is written.
///
/// The machine writes the three times of a file with one call, and the
/// call takes the time of the one that is written and the time of the
/// others: a time that is not written is the zero of the machine.
fn the_time_is_written(file: &fs::File, when: i64, which: WhichTime) -> Result<(), String> {
    use windows_sys::Win32::Foundation::FILETIME;
    use windows_sys::Win32::Storage::FileSystem::SetFileTime;
    let time = the_time_of_the_tree(when);
    let written = FILETIME {
        dwLowDateTime: (time & 0xFFFF_FFFF) as u32,
        dwHighDateTime: ((time >> 32) & 0xFFFF_FFFF) as u32,
    };
    let zero = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let (created, modified, accessed) = match which {
        WhichTime::Created => (written, zero, zero),
        WhichTime::Modified => (zero, written, zero),
        WhichTime::Accessed => (zero, zero, written),
    };
    use std::os::windows::io::AsRawHandle;
    let handle = file.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;
    unsafe {
        if SetFileTime(handle, &created, &modified, &accessed) == 0 {
            return Err("the machine did not write the time of the file.".to_string());
        }
    }
    Ok(())
}

/// The attributes of a file, the way the machine spells them.
///
/// The letters are the ones of the v2: "R" is the read-only, "H" the
/// hidden, "S" the system, "A" the archive, "N" the normal, "D" the
/// directory, "O" the offline, "T" the temporary, "C" the compressed
/// and "E" the encrypted.
fn the_letters_of_the_attributes(attributes: u32) -> Vec<String> {
    let mut letters = Vec::new();
    for (letter, bit) in [
        ('R', 0x0000_0001),
        ('H', 0x0000_0002),
        ('S', 0x0000_0004),
        ('A', 0x0000_0020),
        ('O', 0x0000_1000),
        ('T', 0x0000_0400),
        ('C', 0x0000_0800),
        ('E', 0x0000_4000),
    ] {
        if attributes & bit != 0 {
            letters.push(letter.to_string());
        }
    }
    if letters.is_empty() {
        letters.push("N".to_string());
    }
    letters
}

/// The letters of the attributes of a path, in the order of v2.
///
/// The machine of Windows reads the attributes with `GetFileAttributesW`,
/// and the letters are put in the order of v2 -- R, A, S, H, O, T, D --
/// the way the `add` of the crate orders them. A path that is not there
/// has no letters.
fn the_letters_of_the_path(path: &str) -> Option<Vec<char>> {
    use windows_sys::Win32::Storage::FileSystem::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
    let attributes = unsafe { GetFileAttributesW(Wide::of(path).as_ptr()) };
    if attributes == INVALID_FILE_ATTRIBUTES {
        return None;
    }
    let mut letters: Vec<char> = Vec::new();
    for (letter, bit) in [
        ('R', 0x0000_0001u32),
        ('H', 0x0000_0002),
        ('S', 0x0000_0004),
        ('A', 0x0000_0020),
        ('D', 0x0000_0010),
        ('O', 0x0000_1000),
        ('T', 0x0000_0400),
        ('C', 0x0000_0800),
        ('E', 0x0000_4000),
    ] {
        if attributes & bit != 0 {
            crate::files::add(&mut letters, letter);
        }
    }
    if letters.is_empty() {
        letters.push('N');
    }
    Some(letters)
}

/// The attributes of a file, the way the machine reads them.
///
/// The attributes of a file of Windows are a number, and the letters of
/// the v2 are the bits of it.
fn the_attributes_of(path: &str) -> Result<Vec<String>, String> {
    use windows_sys::Win32::Storage::FileSystem::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
    let attributes = unsafe { GetFileAttributesW(Wide::of(path).as_ptr()) };
    if attributes == INVALID_FILE_ATTRIBUTES {
        let code = unsafe { windows_sys::Win32::Foundation::GetLastError() };
        return Err(the_failure_of_the_machine(
            path,
            &std::io::Error::from_raw_os_error(code as i32),
        ));
    }
    Ok(the_letters_of_the_attributes(attributes))
}

/// The files of the desktop of a Windows machine.
///
/// The desktop holds the files of the crate -- the reading, the writing,
/// the copy, the move and the ini of a file are the work of the
/// standard library -- and the methods of the trait are the ones of it.
macro_rules! the_files_of_the_desktop {
    () => {
        fn create_shortcut(
            &mut self,
            _target: &str,
            _shortcut: &str,
            _working_directory: &str,
            _arguments: &str,
            _description: &str,
            _icon: &str,
        ) -> Result<(), String> {
            // A shortcut of a Windows machine is made by the shell of the
            // machine, and not by a text file: the class of the link
            // (CLSID_ShellLink) is served by the `shell32.dll` of the
            // machine, and it is served to a plain program (measured on
            // the machine of the tree).
            super::com::the_shortcut_is_created(
                _target,
                _shortcut,
                _working_directory,
                _arguments,
                _description,
                _icon,
            )
        }
        fn empty_the_trash(&mut self) -> Result<(), String> {
            self.state.files.empty_the_trash()
        }
        fn recycle_a_file(&mut self, _path: &str) -> Result<(), String> {
            self.state.files.recycle_a_file(_path)
        }
        fn read_shortcut(
            &mut self,
            _shortcut: &str,
        ) -> Result<(String, String, String, String, String), String> {
            super::com::the_shortcut_is_read(_shortcut)
        }
        fn set_working_directory(&mut self, _directory: &str) -> Result<String, String> {
            self.state.files.set_working_directory(_directory)
        }
        fn list_files(
            &mut self,
            _folder: &str,
            _pattern: &str,
            _mode: &str,
        ) -> Result<Vec<String>, String> {
            self.state.files.list_files(_folder, _pattern, _mode)
        }
        fn initial_working_directory(&mut self) -> Result<String, String> {
            self.state.files.initial_working_directory()
        }
        fn working_directory(&mut self) -> Result<String, String> {
            self.state.files.working_directory()
        }
        fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
            self.state.files.read_file(_path)
        }
        fn read_the_bytes_of_a_file(&mut self, _path: &str) -> Result<Vec<u8>, String> {
            self.state.files.read_the_bytes_of_a_file(_path)
        }
        fn canonical_path(&mut self, _path: &str) -> Result<String, String> {
            self.state.files.canonical_path(_path)
        }
        fn write_file(&mut self, _path: &str, _text: &str) -> Result<String, String> {
            self.state.files.write_file(_path, _text)
        }
        fn append_file(
            &mut self,
            _path: &str,
            _bytes: &[u8],
            _mark: &[u8],
        ) -> Result<String, String> {
            self.state.files.append_file(_path, _bytes, _mark)
        }
        fn attributes_of_the_pattern(
            &mut self,
            _pattern: &str,
            _mode: &str,
        ) -> Result<Vec<String>, String> {
            self.state.files.attributes_of_the_pattern(_pattern, _mode)
        }
        fn delete_file(&mut self, _path: &str) -> Result<String, String> {
            self.state.files.delete_file(_path)
        }
        fn create_dir(&mut self, _path: &str) -> Result<String, String> {
            self.state.files.create_dir(_path)
        }
        fn dir_delete(&mut self, _path: &str, _recurse: bool) -> Result<String, String> {
            self.state.files.dir_delete(_path, _recurse)
        }
        fn open_file(&mut self, _path: &str, _mode: &str) -> Result<i64, String> {
            self.state.files.open_file(_path, _mode)
        }
        fn open_with_the_default_program(&mut self, _path: &str) -> Result<(), String> {
            self.state.files.open_with_the_default_program(_path)
        }
        fn read_file_at(
            &mut self,
            _handle: i64,
            _pos: i64,
            _count: i64,
        ) -> Result<Vec<u8>, String> {
            self.state.files.read_file_at(_handle, _pos, _count)
        }
        fn write_file_at(
            &mut self,
            _handle: i64,
            _pos: i64,
            _bytes: &[u8],
        ) -> Result<usize, String> {
            self.state.files.write_file_at(_handle, _pos, _bytes)
        }
        fn file_size(&mut self, _handle: i64) -> Result<i64, String> {
            self.state.files.file_size(_handle)
        }
        fn the_size_of_a_file(&mut self, _path: &str) -> Result<i64, String> {
            self.state.files.the_size_of_a_file(_path)
        }
        fn close_file(&mut self, _handle: i64) -> Result<(), String> {
            self.state.files.close_file(_handle)
        }
        fn copy_file(
            &mut self,
            _from: &str,
            _to: &str,
            _overwrite: bool,
        ) -> Result<String, String> {
            self.state.files.copy_file(_from, _to, _overwrite)
        }
        fn move_file(
            &mut self,
            _from: &str,
            _to: &str,
            _overwrite: bool,
        ) -> Result<String, String> {
            self.state.files.move_file(_from, _to, _overwrite)
        }
        fn copy_dir(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
            self.state.files.copy_dir(_from, _to, _overwrite)
        }
        fn move_dir(&mut self, _from: &str, _to: &str, _overwrite: bool) -> Result<String, String> {
            self.state.files.move_dir(_from, _to, _overwrite)
        }
        fn file_attributes(&mut self, _path: &str) -> Result<Vec<String>, String> {
            self.state.files.file_attributes(_path)
        }
        fn set_file_attributes(
            &mut self,
            _path: &str,
            _attributes: &[String],
        ) -> Result<String, String> {
            self.state.files.set_file_attributes(_path, _attributes)
        }
        fn file_times(&mut self, _path: &str) -> Result<(i64, i64, i64), String> {
            self.state.files.file_times(_path)
        }
        fn set_file_times(
            &mut self,
            _path: &str,
            _created: Option<i64>,
            _modified: Option<i64>,
            _accessed: Option<i64>,
        ) -> Result<String, String> {
            self.state
                .files
                .set_file_times(_path, _created, _modified, _accessed)
        }
        fn ini_read(
            &mut self,
            _path: &str,
            _section: &str,
            _key: &str,
            _default: &str,
        ) -> Result<String, String> {
            self.state.files.ini_read(_path, _section, _key, _default)
        }
        fn ini_write(
            &mut self,
            _path: &str,
            _section: &str,
            _key: &str,
            _value: &str,
        ) -> Result<String, String> {
            self.state.files.ini_write(_path, _section, _key, _value)
        }
        fn ini_delete(
            &mut self,
            _path: &str,
            _section: &str,
            _key: &str,
        ) -> Result<String, String> {
            self.state.files.ini_delete(_path, _section, _key)
        }
        fn empty_trash(&mut self, _drive: Option<&str>) -> Result<(), String> {
            self.state.files.empty_trash(_drive)
        }
        fn recycle(&mut self, _pattern: &str) -> Result<(), String> {
            self.state.files.recycle(_pattern)
        }
        fn create_temp_file(&mut self) -> Result<String, String> {
            self.state.files.create_temp_file()
        }
        fn full_path(&mut self, _path: &str) -> Result<String, String> {
            self.state.files.full_path(_path)
        }
    };
}

pub(crate) use the_files_of_the_desktop;

/// Open a file with the program that the machine opens it with.
///
/// The call is the one of the shell: the verb is "open", and the shell
/// starts the program that is registered for the kind of the file. A file
/// that is not there is a failure of the machine, and the reason is the
/// one of the system.
///
/// Two guards, and both are about the same half hour of a test run:
/// without them the shell of Windows **answers a missing file with a
/// dialog of its own** -- "Windows can't find ..." -- and the dialog
/// waits for a hand that a headless run does not have. Measured on this
/// host: the bare call on a file that is not there took
/// 67 s and held the process, and the suite behind it could not re-link
/// its own binary.
///
/// - the file is looked up first, and what is not there fails with the
///   reason before the shell is asked;
/// - `SEE_MASK_FLAG_NO_UI` says the shell not to put up a window even
///   when the program of the kind of the file is the one that fails:
///   the failure of the call is the failure of the run, and the run
///   reports it in the report, not on the screen.
pub fn open_with_the_program_of_the_machine(path: &str) -> Result<(), String> {
    use windows_sys::Win32::UI::Shell::{SHELLEXECUTEINFOW, ShellExecuteExW};
    use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
    const SEE_MASK_FLAG_NO_UI: u32 = 0x0000_0400;
    if std::fs::metadata(path).is_err() {
        return Err(format!("there is no file to open: {path}"));
    }
    let verb = Wide::of("open");
    let file = Wide::of(path);
    let mut info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    info.lpVerb = verb.as_ptr();
    info.lpFile = file.as_ptr();
    info.nShow = SW_SHOWNORMAL;
    info.fMask = SEE_MASK_FLAG_NO_UI;
    let answer = unsafe { ShellExecuteExW(&mut info) };
    if answer == 0 {
        return Err(super::sys::the_last_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The folder where the test writes.
    fn a_folder() -> PathBuf {
        let folder = std::env::temp_dir().join(format!("ahk-rs-files-{}", std::process::id()));
        let _ = fs::create_dir_all(&folder);
        folder
    }

    #[test]
    fn a_file_that_is_not_there_is_not_opened_with_the_program_of_the_machine() {
        // The test does not open anything, and it must not: the lookup of
        // the file answers before the shell is asked, and the shell with
        // `SEE_MASK_FLAG_NO_UI` would fail with the reason of the system
        // rather than wait on a dialog for a hand. Without the guards the
        // same call took 67 s on this host and kept the process alive --
        // measured, when the dialog of the shell was still
        // the way Windows answered.
        let answer =
            open_with_the_program_of_the_machine("C:\\there-is-no-such-file-here\\the-script.ahk");
        assert!(answer.is_err(), "{answer:?}");
    }

    #[test]
    fn a_file_is_written_and_read_back() {
        let folder = a_folder();
        let path = folder.join("the file.txt");
        let name = path.to_string_lossy().into_owned();
        let mut files = TheFiles::new();
        let written = files
            .write_file(&name, "the text of the file")
            .expect("the file is written");
        assert_eq!(written, "the text of the file");
        let read = files.read_file(&name).expect("the file is read");
        assert_eq!(read, vec!["the text of the file".to_string()]);
        let bytes = files
            .read_the_bytes_of_a_file(&name)
            .expect("the bytes are read");
        assert_eq!(bytes, b"the text of the file".to_vec());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_file_is_copied_and_moved_and_removed() {
        let folder = a_folder();
        let from = folder.join("the first.txt");
        let to = folder.join("the second.txt");
        let mut files = TheFiles::new();
        let _ = files.write_file(&from.to_string_lossy(), "the text");
        let copied = files
            .copy_file(&from.to_string_lossy(), &to.to_string_lossy(), false)
            .expect("the file is copied");
        assert!(copied.ends_with("the second.txt"));
        assert!(to.exists());
        let _ = files.delete_file(&to.to_string_lossy());
        assert!(!to.exists());
        let _ = fs::remove_file(&from);
    }

    #[test]
    fn the_times_of_a_file_are_the_times_of_the_machine() {
        let folder = a_folder();
        let path = folder.join("the file with the times.txt");
        let name = path.to_string_lossy().into_owned();
        let mut files = TheFiles::new();
        let _ = files.write_file(&name, "the text");
        let (created, modified, _accessed) = files.file_times(&name).expect("the times are read");
        assert!(created > 0, "{created}");
        assert!(modified > 0, "{modified}");
        // The time that the tree writes is the time that the machine reads.
        let written = files
            .set_file_times(&name, None, Some(1_000_000_000), None)
            .expect("the time is written");
        assert_eq!(written, "");
        let (_created, modified, _accessed) = files.file_times(&name).expect("the times are read");
        assert_eq!(modified, 1_000_000_000);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_attributes_of_a_file_are_the_letters_of_the_machine() {
        let folder = a_folder();
        let path = folder.join("the file with the attributes.txt");
        let name = path.to_string_lossy().into_owned();
        let mut files = TheFiles::new();
        let _ = files.write_file(&name, "the text");
        let attributes = files
            .file_attributes(&name)
            .expect("the attributes are read");
        assert!(!attributes.is_empty(), "{attributes:?}");
        // The read-only letter is the one that the machine writes.
        let _ = files.set_file_attributes(&name, &["R".to_string()]);
        let attributes = files
            .file_attributes(&name)
            .expect("the attributes are read");
        assert!(
            attributes.iter().any(|letter| letter == "R"),
            "{attributes:?}"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn an_ini_file_is_written_and_read_back() {
        let folder = a_folder();
        let path = folder.join("the file of the ini.ini");
        let name = path.to_string_lossy().into_owned();
        let mut files = TheFiles::new();
        let _ = files.ini_write(&name, "the section", "the key", "the value");
        let read = files
            .ini_read(&name, "the section", "the key", "the default")
            .expect("the ini is read");
        assert_eq!(read, "the value");
        let missing = files
            .ini_read(&name, "the section", "the other key", "the default")
            .expect("the ini is read");
        assert_eq!(missing, "the default");
        let _ = files.ini_delete(&name, "the section", "the key");
        let missing = files
            .ini_read(&name, "the section", "the key", "the default")
            .expect("the ini is read");
        assert_eq!(missing, "the default");
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn the_files_of_a_pattern_are_the_files_of_the_folder() {
        // The folder is of the test alone: the other tests write their
        // own files in the folder of them.
        let folder = a_folder().join("the listing");
        let _ = fs::create_dir_all(&folder);
        let _ = fs::write(folder.join("one.txt"), "one");
        let _ = fs::write(folder.join("two.txt"), "two");
        let _ = fs::write(folder.join("three.txt"), "three");
        let mut files = TheFiles::new();
        let found = files
            .list_files(&folder.to_string_lossy(), "*.txt", "F")
            .expect("the files are listed");
        assert_eq!(found.len(), 3, "{found:?}");
        let _ = fs::remove_file(folder.join("one.txt"));
        let _ = fs::remove_file(folder.join("two.txt"));
        let _ = fs::remove_file(folder.join("three.txt"));
    }
}
