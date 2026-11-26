//! The includes of a script: the files it asks for, and the refusals of them.
//!
//! The includes are resolved before the script is parsed, and the resolution
//! is one and the same for the runner and for the MCP: the two share this
//! module, and a script that the runner refuses is a script that the MCP
//! refuses too.
//!
//! The includes are found with the lexer of the tree, and not with a scan of
//! the lines: the lexer knows the comments, and an `#Include` that is inside
//! one is not a directive at all.
//!
//! Measured on the binary of the v2.0.32: a script that has an `#Include`
//! inside a `/* ... */` block loads, and the `#Include` of the block is of
//! no account. A script that asks for a library that is not there is refused
//! with "Script library not found." and the line of the directive after it,
//! and a script that asks for a file that is not there is refused with
//! `#Include file "<the path as it was written>" cannot be opened.`

use std::path::{Path, PathBuf};

use ahk::dialect::Dialect;
use ahk::lexer::{Token, TokenKind, tokenize_with_lines};

/// The refusal of an include: the words of it, and the line it is in.
#[derive(Debug, Clone)]
pub struct Refusal {
    /// The words of the refusal, the way the official writes them.
    pub message: String,
    /// The line of the `#Include` that asked for it, from one.
    pub line: u32,
}

/// The source with the includes put in place of the lines that ask for
/// them.
///
/// A `#Include "path"` line is replaced by the text of the file, and the
/// path is taken from the directory of the file that asks for it. An
/// include that is already in the tree is not put in twice; the
/// `#IncludeAgain` line puts its file in every time it asks for it, which
/// is the difference between the two directives.
///
/// The nesting is counted: a file that includes itself, or a group of files
/// that include one another, is refused with the reason instead of making
/// the runner recurse until the stack is gone.
pub fn resolve_includes(
    source: &str,
    base: &Path,
    file: Option<&Path>,
    seen: &mut Vec<PathBuf>,
    dialect: &'static dyn Dialect,
) -> Result<String, Refusal> {
    resolve_includes_at(source, base, file, seen, 0, dialect)
}

/// How deep the includes may nest.
const MAX_INCLUDE_DEPTH: usize = 64;

/// `resolve_includes`, with the depth of the nesting the tree is in.
fn resolve_includes_at(
    source: &str,
    base: &Path,
    file: Option<&Path>,
    seen: &mut Vec<PathBuf>,
    depth: usize,
    dialect: &'static dyn Dialect,
) -> Result<String, Refusal> {
    if depth > MAX_INCLUDE_DEPTH {
        return Err(Refusal {
            message: format!(
                "the includes nest more than {MAX_INCLUDE_DEPTH} deep: a file includes itself, or a group of them includes one another."
            ),
            line: 0,
        });
    }
    let includes = the_includes_of(source, dialect);
    let mut out = String::with_capacity(source.len());
    // The working directory of the load: an include that names a directory
    // changes it, the way the official changes it.
    let mut working = base.to_path_buf();
    let mut at = 0usize;
    for (index, line) in source.lines().enumerate() {
        let number = index as u32 + 1;
        while at < includes.len() && includes[at].line < number {
            at += 1;
        }
        let Some(include) = includes.get(at).filter(|include| include.line == number) else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        at += 1;
        // `*i` asks for the include to be skipped when the file is not
        // there, the way the directive of the reference does.
        let (ignore_missing, argument) = match include.argument.strip_prefix('*') {
            Some(rest) => {
                let end = rest
                    .find(|character: char| character.is_whitespace())
                    .unwrap_or(rest.len());
                let (option, rest) = rest.split_at(end);
                if !option.eq_ignore_ascii_case("i") {
                    return Err(Refusal {
                        message: format!(
                            "the include option \"*{option}\" is not one this tree speaks."
                        ),
                        line: number,
                    });
                }
                (true, rest.trim())
            }
            None => (false, include.argument.as_str()),
        };
        // Measured on the binary of the v2.0.32: the directive with no
        // parameter is refused with "Parameter #1 required" and the line of
        // the directive after it (`ERR_PARAM1_REQUIRED`,
        // `source/script.h:155`), and the directive whose parameter is empty
        // is refused with "Parameter #1 invalid." (`ERR_PARAM1_INVALID`).
        if include.argument.trim().is_empty() {
            return Err(Refusal {
                message: format!("Parameter #1 required\n     Specifically: {}", include.text),
                line: number,
            });
        }
        // A comment may follow the path; it is not part of it.
        let argument = strip_include_comment(argument);
        // The path is in quotes, or it is the rest of the line.
        let path = argument
            .strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .unwrap_or(argument)
            .trim();
        if path.is_empty() {
            return Err(Refusal {
                message: "Parameter #1 invalid.".to_string(),
                line: number,
            });
        }
        let script_dir = absolute_dir(base);
        let line_file = file
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "*".to_string());
        // The path of the file that is asked for, and the path as the script
        // wrote it: the refusal of a file that cannot be opened names the
        // path as it was written, and not the path of the disk.
        let (path, written) = if path.starts_with('<') && path.ends_with('>') {
            // `<Lib>` is the library include: the Lib folders of the
            // script, of the documents and of the interpreter are searched.
            match find_library(&path[1..path.len() - 1], &script_dir) {
                Some(path) => {
                    let written = path.to_string_lossy().into_owned();
                    (path, written)
                }
                None if ignore_missing => {
                    out.push('\n');
                    continue;
                }
                // Measured on the binary of the v2.0.32: the library that
                // is not there is refused with the words of the dialect and
                // the line of the directive after them.
                None => {
                    return Err(Refusal {
                        message: format!(
                            "{}\n     Specifically: {}",
                            dialect.the_words_of_the_missing_library(),
                            include.text
                        ),
                        line: number,
                    })
                }
            }
        } else {
            let expanded = expand_include_vars(path, &script_dir, &line_file);
            (working.join(&expanded), expanded)
        };
        let path = match path.canonicalize() {
            Ok(path) => path,
            Err(_) if ignore_missing => {
                out.push('\n');
                continue;
            }
            // Measured on the binary of the v2.0.32: the file that cannot
            // be opened is refused with the path as the script wrote it.
            Err(_) => {
                return Err(Refusal {
                    message: format!("#Include file \"{written}\" cannot be opened."),
                    line: number,
                })
            }
        };
        // Measured on the binary of the v2.0.32: an include that names a
        // directory is not a refusal: the directory becomes the working
        // directory of the rest of the load, and the line itself is of no
        // account.
        if path.is_dir() {
            working = path.clone();
            out.push('\n');
            continue;
        }
        if !include.again && seen.contains(&path) {
            // The file is already in the tree, and `#Include` puts it in
            // once: the line becomes nothing.
            out.push('\n');
            continue;
        }
        let text = match read_script_file(&path) {
            Ok(text) => text,
            // Measured on the binary of the v2.0.32: a file that cannot be
            // opened is refused the same way a file that is not there is.
            Err(_) => {
                return Err(Refusal {
                    message: format!("#Include file \"{written}\" cannot be opened."),
                    line: number,
                })
            }
        };
        let directory = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        // The file is in the tree now, and a later `#Include` of it is
        // skipped. `#IncludeAgain` ignores this list and asks again.
        if !seen.contains(&path) {
            seen.push(path.clone());
        }
        out.push_str(&resolve_includes_at(
            &text,
            &directory,
            Some(&path),
            seen,
            depth + 1,
            dialect,
        )?);
        out.push('\n');
    }
    Ok(out)
}

/// The include that a line of the script asks for.
struct TheInclude {
    /// The line of the directive, from one.
    line: u32,
    /// Whether the directive is `#IncludeAgain`.
    again: bool,
    /// The argument of the directive, as it was written.
    argument: String,
    /// The whole line of the directive, as it was written: it is what the
    /// official names after the words of the refusal.
    text: String,
}

/// The includes of the source, and the lines they are in.
///
/// The lexer of the tree knows the comments, and an `#Include` that is
/// inside one is not a directive at all: the official ignores it, and the
/// tree ignores it too.
fn the_includes_of(source: &str, dialect: &'static dyn Dialect) -> Vec<TheInclude> {
    match tokenize_with_lines(source, dialect) {
        Ok((tokens, _)) => the_includes_of_the_tokens(source, &tokens),
        // The source does not lex: the parse of the whole script is the one
        // that says so, and the includes are found with the scan that knows
        // about the comments.
        Err(_) => the_includes_of_the_scan(source),
    }
}

/// The includes of a source that lexes.
///
/// A directive is a token of the lexer, and the argument of it is the token
/// that follows it on the same line. The name of the directive is the one
/// of the lexer, in lower case: `#Include` and `#include` are the same
/// directive.
fn the_includes_of_the_tokens(source: &str, tokens: &[Token]) -> Vec<TheInclude> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at < tokens.len() {
        let token = &tokens[at];
        let is_include = token.kind == TokenKind::Directive
            && (token.value == "include" || token.value == "includeagain");
        if !is_include {
            at += 1;
            continue;
        }
        // The argument of the directive is the token that follows it on the
        // same line.
        let argument = match tokens.get(at + 1) {
            Some(next) if next.kind == TokenKind::String && next.line == token.line => {
                next.value.clone()
            }
            _ => String::new(),
        };
        out.push(TheInclude {
            line: token.line,
            again: token.value == "includeagain",
            argument,
            text: the_line_of_the_directive(source, token.line),
        });
        at += 1;
        if tokens
            .get(at)
            .is_some_and(|next| next.kind == TokenKind::String && next.line == token.line)
        {
            at += 1;
        }
    }
    out
}

/// The line of the source, as it was written.
///
/// The comment that may follow the directive is not a part of it: the
/// official names the directive and its argument, and not the words that
/// follow them.
fn the_line_of_the_directive(source: &str, line: u32) -> String {
    source
        .lines()
        .nth(line.saturating_sub(1) as usize)
        .map(|text| strip_include_comment(text.trim()).to_string())
        .unwrap_or_default()
}

/// The includes of a source that does not lex.
///
/// The lexer of the tree refuses a line that holds no action, and the
/// refusal of it is the one of the parse of the whole script: the includes
/// of such a source are found with a scan that knows about the comments, so
/// that an `#Include` inside one is not an include either.
fn the_includes_of_the_scan(source: &str) -> Vec<TheInclude> {
    let mut out = Vec::new();
    let mut in_a_comment = false;
    for (index, line) in source.lines().enumerate() {
        let number = index as u32 + 1;
        let mut rest = line;
        if in_a_comment {
            // The comment goes on until the `*/` that closes it.
            match rest.find("*/") {
                Some(at) => {
                    in_a_comment = false;
                    rest = &rest[at + 2..];
                }
                None => continue,
            }
        }
        // A block comment that begins in the line.
        while let Some(at) = rest.find("/*") {
            match rest[at + 2..].find("*/") {
                Some(end) => rest = &rest[at + 2 + end + 2..],
                None => {
                    in_a_comment = true;
                    rest = "";
                    break;
                }
            }
        }
        if in_a_comment {
            continue;
        }
        let trimmed = rest.trim();
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        let Some((again, argument)) = the_include_of(trimmed) else {
            continue;
        };
        out.push(TheInclude {
            line: number,
            again,
            argument: argument.to_string(),
            text: strip_include_comment(trimmed).to_string(),
        });
    }
    out
}

/// The text of a script file, the way the reference reads it: a BOM names
/// the encoding, and a file without one is UTF-8 when it can be and the
/// ANSI code page of the machine when it cannot.
pub fn read_script_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    Ok(decode_script_bytes(&bytes))
}

/// The bytes of a script as text: UTF-8 or UTF-16 when a BOM says so, and
/// otherwise UTF-8 first, the ANSI code page second.
fn decode_script_bytes(bytes: &[u8]) -> String {
    if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(rest).into_owned();
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        return decode_utf16(rest, true);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        return decode_utf16(rest, false);
    }
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_string(),
        Err(_) => decode_ansi(bytes),
    }
}

/// UTF-16 as text, little or big endian, an unpaired surrogate replaced.
fn decode_utf16(bytes: &[u8], little: bool) -> String {
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| {
            if little {
                u16::from_le_bytes([pair[0], pair[1]])
            } else {
                u16::from_be_bytes([pair[0], pair[1]])
            }
        })
        .collect();
    char::decode_utf16(units)
        .map(|result| result.unwrap_or('\u{FFFD}'))
        .collect()
}

/// The ANSI code page of the machine. On Windows it is the one the system
/// answers for (`GetACP`), and the conversion is the system's own; elsewhere
/// it is CP1252, the page the dialect was measured on.
fn decode_ansi(bytes: &[u8]) -> String {
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn GetACP() -> u32;
            fn MultiByteToWideChar(
                code_page: u32,
                flags: u32,
                text: *const u8,
                text_len: i32,
                wide: *mut u16,
                wide_len: i32,
            ) -> i32;
        }
        // SAFETY: the bytes are a live slice for the length passed, and the
        // wide buffer is sized by the first call that asks for the size.
        unsafe {
            let page = GetACP();
            let len = bytes.len() as i32;
            let needed = MultiByteToWideChar(page, 0, bytes.as_ptr(), len, std::ptr::null_mut(), 0);
            if needed > 0 {
                let mut wide = vec![0u16; needed as usize];
                let written = MultiByteToWideChar(
                    page,
                    0,
                    bytes.as_ptr(),
                    len,
                    wide.as_mut_ptr(),
                    needed,
                );
                if written > 0 {
                    wide.truncate(written as usize);
                    return char::decode_utf16(wide)
                        .map(|result| result.unwrap_or('\u{FFFD}'))
                        .collect();
                }
            }
        }
        String::from_utf8_lossy(bytes).into_owned()
    }
    #[cfg(not(windows))]
    {
        const HIGH: [char; 32] = [
            '\u{20AC}', '\u{0081}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}',
            '\u{2020}', '\u{2021}', '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}',
            '\u{0152}', '\u{008D}', '\u{017D}', '\u{008F}', '\u{0090}', '\u{2018}',
            '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
            '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}', '\u{0153}', '\u{019D}',
            '\u{017E}', '\u{0178}',
        ];
        bytes
            .iter()
            .map(|&byte| {
                if byte < 0x80 {
                    byte as char
                } else if byte < 0xA0 {
                    HIGH[(byte - 0x80) as usize]
                } else {
                    byte as char
                }
            })
            .collect()
    }
}

/// The directory as an absolute path, without the `\\?\` prefix that
/// `canonicalize` puts on Windows: the expanded `%A_ScriptDir%` is a path
/// the script itself may compare against.
pub fn absolute_dir(base: &Path) -> PathBuf {
    if base.is_absolute() {
        base.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(directory) => directory.join(base),
            Err(_) => base.to_path_buf(),
        }
    }
}

/// The comment that follows an include path: a `;` at the start of the
/// argument or after a space, outside quotes, begins it.
fn strip_include_comment(argument: &str) -> &str {
    let mut quoted = false;
    let mut previous = ' ';
    for (index, character) in argument.char_indices() {
        if character == '"' {
            quoted = !quoted;
        } else if character == ';' && !quoted && previous.is_whitespace() {
            return argument[..index].trim_end();
        }
        previous = character;
    }
    argument
}

/// The built-in variables an include path may reference, expanded the way
/// the reference expands them. A reference that is not one of them is left
/// as it is.
fn expand_include_vars(path: &str, script_dir: &Path, line_file: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut rest = path;
    while let Some(start) = rest.find('%') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find('%') else {
            out.push('%');
            rest = after;
            continue;
        };
        let name = &after[..end];
        let value = match name.to_ascii_lowercase().as_str() {
            "a_scriptdir" => Some(script_dir.to_string_lossy().into_owned()),
            "a_linefile" => Some(line_file.to_string()),
            "a_appdata" => std::env::var("APPDATA").ok(),
            "a_appdatacommon" => std::env::var("PROGRAMDATA").ok(),
            "a_mydocuments" => std::env::var("USERPROFILE")
                .ok()
                .map(|home| format!("{home}\\Documents")),
            _ => None,
        };
        match value {
            Some(value) => out.push_str(&value),
            None => {
                out.push('%');
                out.push_str(name);
                out.push('%');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// The library include `<Name>`: the Lib folder of the script, then the
/// one of the documents, then the one beside the interpreter. A name with
/// a backslash asks for a function of a library: `Lib\Func` looks for
/// `Lib\Func.ahk` and then for `Lib.ahk`.
///
/// Measured on the binary of the v2.0.32: the three folders are the ones
/// that the official searches, in this order.
pub fn find_library(name: &str, script_dir: &Path) -> Option<PathBuf> {
    let mut roots = vec![script_dir.join("Lib")];
    if let Ok(home) = std::env::var("USERPROFILE") {
        roots.push(PathBuf::from(home).join("Documents\\AutoHotkey\\Lib"));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(directory) = exe.parent() {
            roots.push(directory.join("Lib"));
        }
    }
    let (library, function) = match name.split_once('\\') {
        Some((library, function)) => (library, Some(function)),
        None => (name, None),
    };
    for root in roots {
        if let Some(function) = function {
            let path = root.join(library).join(format!("{function}.ahk"));
            if path.is_file() {
                return Some(path);
            }
        }
        let path = root.join(format!("{library}.ahk"));
        if path.is_file() {
            return Some(path);
        }
    }
    None
}

/// The include that the line asks for: whether it is `#IncludeAgain`, and
/// its argument.
///
/// The directive is matched by its whole name, because `#Include` is a
/// prefix of `#IncludeAgain` and the two are not the same line. A line
/// whose directive is neither is a line of the script, and it is left as it
/// is.
fn the_include_of(line: &str) -> Option<(bool, &str)> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix('#')?;
    let end = rest
        .find(|character: char| character.is_whitespace())
        .unwrap_or(rest.len());
    let (name, argument) = rest.split_at(end);
    let again = if name.eq_ignore_ascii_case("include") {
        false
    } else if name.eq_ignore_ascii_case("includeagain") {
        true
    } else {
        return None;
    };
    Some((again, argument.trim()))
}
