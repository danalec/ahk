//! The registry of the machine, as this runtime keeps it.
//!
//! Windows has a registry, and the v2 names read it and write it. This
//! platform has none, and the tree carries the API over a store of its own:
//! the hives are folders and the keys are files, under the config folder of
//! the user.
//!
//! Measured on the binary of v2.0.19 on 2026-11-11: `RegCreateKey` answers
//! the empty text and makes the key; `RegWrite` answers the empty text and
//! **makes the key when it is not there** -- with its ancestors, the
//! documentation says so and the binary does it; `RegRead` answers the
//! value, and answers "(2) The system cannot find the file specified." when
//! the key or the value is not there -- unless a default is given, which is
//! then the answer; `RegDelete` removes a value and `RegDeleteKey` removes
//! the key, both answering the empty text.
//!
//! The shape of a key is the shape of the Windows one: `HKCU\Software\X` is
//! the key `X` under `Software` of the hive of the current user, and the
//! hive is **required** -- measured: `RegCreateKey("Software\X")` throws
//! "Parameter #1 of RegCreateKey is invalid.".
//!
//! The values of a key live in the file of the key, one per line: the kind
//! of the value, a tab, the name of it, a tab, and the text of it. The
//! escapes of the text are the ones of a line: a backslash is two, a tab is
//! `\t` and a line break is `\n`.

use std::path::{Path, PathBuf};

/// The kinds of a value that the v2 carries.
pub const KINDS: [&str; 5] = [
    "REG_SZ",
    "REG_EXPAND_SZ",
    "REG_MULTI_SZ",
    "REG_DWORD",
    "REG_BINARY",
];

/// The failure of the machine when a key or a value is not there.
///
/// Measured on the binary of v2.0.19 on 2026-11-11: the code of the system
/// in front of what it says.
pub const NOT_THERE: &str = "(2) The system cannot find the file specified.";

/// The failure of the machine when a place cannot be written.
pub const DENIED: &str = "(5) Access is denied.";

/// The hive of a key, and the name the store gives it.
///
/// `None` is a key with no hive, or one whose hive this runtime does not
/// carry -- the hives of the remote registry included, since a remote
/// registry is a thing of Windows.
fn the_hive_of(key: &str) -> Option<&'static str> {
    let mut parts = key.split('\\');
    let hive = parts.next()?.trim().to_uppercase();
    Some(match hive.as_str() {
        "HKCU" | "HKEY_CURRENT_USER" => "HKCU",
        "HKLM" | "HKEY_LOCAL_MACHINE" => "HKLM",
        "HKU" | "HKEY_USERS" => "HKU",
        "HKCR" | "HKEY_CLASSES_ROOT" => "HKCR",
        "HKCC" | "HKEY_CURRENT_CONFIG" => "HKCC",
        _ => return None,
    })
}

/// The place of the store, under the config folder of the user.
fn the_root() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".config")))?;
    Some(base.join("ahk").join("registry"))
}

/// The file of a key, and `None` when the name of the key carries no hive.
fn the_file_of(key: &str) -> Option<PathBuf> {
    let hive = the_hive_of(key)?;
    let root = the_root()?;
    let mut path = root.join(hive);
    for part in key.split('\\').skip(1) {
        if part.is_empty() {
            continue;
        }
        path.push(sanitize(part));
    }
    Some(path)
}

/// A part of a key, with the characters a name of a file cannot carry.
///
/// A key of Windows can carry almost anything but the backslash, and the
/// store is a folder of the machine: the two that a name of a file cannot
/// carry become one underscore each.
fn sanitize(part: &str) -> String {
    part.chars()
        .map(|letter| match letter {
            '/' | '\0' => '_',
            other => other,
        })
        .collect()
}

/// The values of a key, read from its file.
fn the_values_of(key: &str) -> Result<Vec<(String, String, String)>, String> {
    let path = the_file_of(key).ok_or_else(|| NOT_THERE.to_string())?;
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        // A key that is not there is the failure the machine gives.
        Err(_) => return Err(NOT_THERE.to_string()),
    };
    let mut values = Vec::new();
    for line in text.lines() {
        let mut pieces = line.splitn(3, '\t');
        let (Some(kind), Some(name), Some(value)) =
            (pieces.next(), pieces.next(), pieces.next())
        else {
            continue;
        };
        values.push((
            kind.to_string(),
            name.to_string(),
            unescape(value),
        ));
    }
    Ok(values)
}

/// Write the values of a key into its file.
fn the_values_are_written(
    key: &str,
    values: &[(String, String, String)],
) -> Result<(), String> {
    let path = the_file_of(key).ok_or_else(|| NOT_THERE.to_string())?;
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).map_err(the_failure_of)?;
    }
    let mut text = String::new();
    for (kind, name, value) in values {
        text.push_str(kind);
        text.push('\t');
        text.push_str(name);
        text.push('\t');
        text.push_str(&escape(value));
        text.push('\n');
    }
    std::fs::write(&path, text).map_err(the_failure_of)
}

/// The failure of the machine, in the words of v2.
fn the_failure_of(error: std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::PermissionDenied => DENIED.to_string(),
        _ => NOT_THERE.to_string(),
    }
}

/// The text of a value, with the escapes of a line undone.
fn unescape(text: &str) -> String {
    let mut out = String::new();
    let mut characters = text.chars();
    while let Some(letter) = characters.next() {
        if letter != '\\' {
            out.push(letter);
            continue;
        }
        match characters.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

/// The text of a value, with the escapes of a line done.
fn escape(text: &str) -> String {
    let mut out = String::new();
    for letter in text.chars() {
        match letter {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out
}

/// The value of a name, and its kind.
///
/// `None` is a value that is not there; a key that is not there is a
/// failure of the machine.
pub fn value_of(key: &str, name: &str) -> Result<Option<(String, String)>, String> {
    let values = the_values_of(key)?;
    Ok(values
        .iter()
        .find(|(_, held, _)| held.eq_ignore_ascii_case(name))
        .map(|(kind, _, value)| (kind.clone(), value.clone())))
}

/// Write a value into a key, making the key when it is not there.
pub fn write(key: &str, name: &str, kind: &str, value: &str) -> Result<(), String> {
    let mut values = the_values_of(key).unwrap_or_default();
    let value = value.to_string();
    match values
        .iter_mut()
        .find(|(_, held, _)| held.eq_ignore_ascii_case(name))
    {
        Some(held) => {
            held.0 = kind.to_string();
            held.2 = value;
        }
        None => values.push((kind.to_string(), name.to_string(), value)),
    }
    the_values_are_written(key, &values)
}

/// Remove a value of a key.
pub fn delete(key: &str, name: &str) -> Result<(), String> {
    let mut values = the_values_of(key)?;
    values.retain(|(_, held, _)| !held.eq_ignore_ascii_case(name));
    the_values_are_written(key, &values)
}

/// Remove a key of the store.
pub fn delete_key(key: &str) -> Result<(), String> {
    let path = the_file_of(key).ok_or_else(|| NOT_THERE.to_string())?;
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) => Err(the_failure_of(error)),
    }
}

/// Make a key of the store, with its ancestors.
pub fn create_key(key: &str) -> Result<(), String> {
    let path = the_file_of(key).ok_or_else(|| NOT_THERE.to_string())?;
    if let Some(folder) = path.parent() {
        std::fs::create_dir_all(folder).map_err(the_failure_of)?;
    }
    if !path.exists() {
        std::fs::write(&path, "").map_err(the_failure_of)?;
    }
    Ok(())
}
