//! The patterns of the files of the v1.
//!
//! Measured on the binary of v1.1.37.02 on 2026-11-02: the source of the
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

use crate::runtime::{Runtime, RuntimeError};

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
        .map_err(|reason| RuntimeError { message: reason, line })?;
    rt.desktop()
        .list_files(&folder, pattern, "")
        .map_err(|reason| RuntimeError { message: reason, line })
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
