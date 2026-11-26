//! The patterns of the files of the v1.
//!
//! Measured on the binary of v1.1.37.02: the source of the
//! `FileCopy`, of the `FileMove` and of the `FileDelete` of the v1 may be a
//! pattern, and the command does the thing to every file that matches it.
//! The pattern is the pattern of the v2 -- the `*` and the `?`, in the last
//! part of the path -- and the folder of the path is not a pattern.
//!
//! The `ErrorLevel` of the commands of the v1 is the number of the files
//! that could not be done, and the v1 goes on: the `ErrorLevel` is the only
//! report of it.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, Value};

/// The files that a pattern of the v1 matches.
///
/// The machine is asked for them: the tree opens no folder of its own. The
/// pattern is the whole path, and the machine takes the folder of it and the
/// name of it apart.
pub(crate) fn the_files_of_the_pattern(
    rt: &mut Runtime<'_>,
    pattern: &str,
    line: u32,
) -> Result<Vec<String>, RuntimeError> {
    let folder = rt
        .desktop()
        .working_directory()
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    rt.desktop()
        .list_files(&folder, pattern, "")
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })
}

/// Whether the path is a pattern: it carries a wildcard.
pub(crate) fn is_a_pattern(path: &str) -> bool {
    path.contains('*') || path.contains('?')
}

/// The name of a file, in the path of it.
pub(crate) fn the_name_of_the_file(path: &str) -> &str {
    match path.rfind(['/', '\\']) {
        Some(index) => &path[index + 1..],
        None => path,
    }
}

/// The path of a file in a folder.
///
/// The separator is the one of the machine: the paths that the machine gives
/// carry it, and the folder of the script may end with one already.
pub(crate) fn the_path_in_the_folder(folder: &str, name: &str) -> String {
    if folder.is_empty() {
        return name.to_string();
    }
    if folder.ends_with('/') || folder.ends_with('\\') {
        return format!("{folder}{name}");
    }
    format!("{folder}/{name}")
}

/// `FileOpen(FileName, Mode, Encoding)`: open a file and answer it.
///
/// Measured on the binary of v1.1.37.02: the v1 does not fail when the
/// file cannot be opened: it answers zero. The `FileOpen("new.txt", "q")`
/// of a mode that is not one of them answers zero, and so does the
/// `FileOpen("missing.txt", "r")` of a file that is not there.
///
/// The mode of the v1 is the access mode and the flags of the v1: the
/// access mode comes first, and the rest of the string says what happens
/// to the line endings and who may open the file at the same time. The
/// tree keeps the access mode: the flags of the v1 are about the sharing
/// of the file, and the tree shares every file it opens.
pub(crate) fn file_open(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let mut arguments = values.to_vec();
    // The mode of the v1 is the access mode and the flags of the v1.
    if let Some(mode) = arguments.get(1) {
        match the_access_mode_of_the_v1(&mode.to_text()) {
            Some(access) => arguments[1] = Value::Text(access.into()),
            // A mode that is not one of them: the v1 answers zero.
            None => return Ok(Value::Int(0)),
        }
    }
    // The encoding of the v1 is the number of the code page, and the tree
    // names it.
    if let Some(encoding) = arguments.get(2) {
        if let Some(page) = encoding.to_number() {
            let name = the_name_of_the_code_page(page);
            if name.is_empty() {
                // The code page of the script: the tree takes it when the
                // script does not say which one it wants.
                arguments.truncate(2);
            } else {
                arguments[2] = Value::Text(name.into());
            }
        }
    }
    match crate::builtins::file_open(rt, &arguments, line, _variant) {
        Ok(value) => Ok(value),
        // The v1 answers zero and does not fail.
        Err(_) => Ok(Value::Int(0)),
    }
}

/// The access mode of the `FileOpen` of the v1.
///
/// Measured on the binary of v1.1.37.02: the access mode comes first, and
/// it is the "r", the "rw", the "w", the "a" or the "h". The rest of the
/// string says what happens to the line endings -- the "\n" and the "\r" --
/// and who may open the file at the same time: the "-" and the "r", the
/// "w" and the "d" that follow it. A string that carries another letter is
/// refused, and the refusal of the v1 is the answer of zero.
///
/// The "h" is a handle of the machine, and the tree opens no file of a
/// handle: the answer is zero, the way it is for a mode that is not one of
/// them.
fn the_access_mode_of_the_v1(mode: &str) -> Option<&'static str> {
    let mode = mode.trim_start();
    let letters: Vec<char> = mode.chars().collect();
    if letters.is_empty() {
        return None;
    }
    let first = letters[0].to_ascii_lowercase();
    let (access, mut at) = match first {
        'r' => {
            if letters.get(1).map(|letter| letter.to_ascii_lowercase()) == Some('w') {
                ("rw", 2)
            } else {
                ("r", 1)
            }
        }
        'w' => ("w", 1),
        'a' => ("a", 1),
        _ => return None,
    };
    while at < letters.len() {
        let flag = letters[at].to_ascii_lowercase();
        at += 1;
        match flag {
            '\n' | '\r' | ' ' | '\t' => {}
            '-' => {
                // The sharing of the file: the letters that follow the dash
                // are the ones of it.
                while at < letters.len() {
                    let sharing = letters[at].to_ascii_lowercase();
                    if !matches!(sharing, 'r' | 'w' | 'd') {
                        break;
                    }
                    at += 1;
                }
            }
            _ => return None,
        }
    }
    Some(access)
}

/// The name of the encoding of the v1, when the script gave the number of
/// the code page.
///
/// Measured on the binary of v1.1.37.02: the encoding of the `FileOpen` of
/// the v1 is the number of the code page, and the zero is the encoding of
/// the script. The bit of the top of the number says that the file has no
/// byte order mark.
fn the_name_of_the_code_page(page: f64) -> String {
    let page = page as u32;
    let raw = page & 0x8000_0000 != 0;
    let page = page & !0x8000_0000;
    match page {
        0 => String::new(),
        65001 => {
            if raw {
                "UTF-8-RAW".to_string()
            } else {
                "UTF-8".to_string()
            }
        }
        1200 => {
            if raw {
                "UTF-16-RAW".to_string()
            } else {
                "UTF-16".to_string()
            }
        }
        _ => format!("CP{page}"),
    }
}
