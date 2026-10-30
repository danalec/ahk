//! The dialogs: the questions the script asks the user.
//!
//! v2 has three of them: an input box, a file selection and a folder
//! selection. The runtime reads the options of each one, because the
//! options are the dialect's, and the desktop asks the question,
//! because the window is the machine's.
//!
//! Measured on the binary of v2.0.19 on 2026-10-29.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::desktop::{FileAnswer, FileOptions, InputOptions};
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, TypeName, Value, put};

/// `InputBox`: ask the user for a text.
///
/// The answer of v2 is an object: the text the user typed is in its
/// `Value` and the way the dialog was closed is in its `Result`, which
/// is "OK", "Cancel" or "Timeout".
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the answer is an
/// object of the class `Object`, and the timeout of the options closes
/// the dialog with the result "Timeout".
pub fn input_box(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let prompt = text_of(values, 0);
    let title = text_of(values, 1);
    let options = text_of(values, 2);
    let default = text_of(values, 3);
    let options = the_options_of_the_input_box(&options, line)?;
    let answer = rt
        .desktop()
        .input_box(&prompt, &title, &options, &default)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    let mut properties = BTreeMap::new();
    put(&mut properties, "value", Value::Text(answer.value));
    put(&mut properties, "result", Value::Text(answer.result));
    Ok(Value::Object(Rc::new(RefCell::new(Object {
        class: "object".to_string(),
        properties,
    }))))
}

/// `FileSelect`: ask the user for a file, or for a folder.
///
/// The answer of v2 is the path of what the user chose, and it is an
/// array of paths when the user may choose more than one.
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the options are
/// one letter -- "D", "M" or "S" -- with a number after it, or a
/// number alone, and the filter is not there when the user chooses a
/// folder.
pub fn file_select(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let options = text_of(values, 0);
    let root = text_of(values, 1);
    let title = text_of(values, 2);
    let filter = text_of(values, 3);
    let options = the_options_of_the_file_select(&options, line)?;
    // The filter of v2 is not there when the user chooses a folder.
    if options.directory && !filter.is_empty() {
        return Err(RuntimeError {
            message: crate::runtime::invalid_parameter(4, "FileSelect"),
            line,
        });
    }
    let answer = rt
        .desktop()
        .file_select(&options, &root, &title, &filter)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(match answer {
        FileAnswer::One(path) => Value::Text(path),
        FileAnswer::Many(paths) => {
            // The answer of v2 is an array of the paths, and an array is
            // a class of the runtime.
            let paths: Vec<Value> = paths.into_iter().map(Value::Text).collect();
            match crate::classes::array::construct(&paths, rt, line) {
                Ok(array) => array,
                Err(RuntimeFailure::Error(error)) => return Err(error),
                Err(RuntimeFailure::Detailed(error, _)) => return Err(error),
                Err(_) => {
                    return Err(RuntimeError {
                        message: "the array of the files cannot be made.".to_string(),
                        line,
                    })
                }
            }
        }
    })
}

/// `DirSelect`: ask the user for a folder.
///
/// The answer of v2 is the path of the folder, and it is the empty text
/// when the user closes the dialog without choosing one.
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the options are a
/// number, and a text that is not a number is refused.
pub fn dir_select(rt: &mut Runtime<'_>, values: &[Value], line: u32, _variant: u8) -> Result<Value, RuntimeError> {
    let starting = text_of(values, 0);
    let prompt = text_of(values, 2);
    // The options of v2 are a number, and the tree says so the way v2
    // says it.
    let options = values.get(1).cloned().unwrap_or(Value::Int(1));
    let options = options
        .to_number_strict()
        .map_err(|_| RuntimeError {
            message: crate::runtime::requires(2, "DirSelect", TypeName::Number, &options),
            line,
        })? as i64;
    let folder = rt
        .desktop()
        .dir_select(&starting, options, &prompt)
        .map_err(|reason| RuntimeError { message: reason, line })?;
    Ok(Value::Text(folder))
}

// -- the options ----------------------------------------------------------------

/// The options of an input box, read the way v2 reads them.
///
/// The options are a list of tokens, separated by spaces or tabs. A
/// token is a letter and, for most of them, a number: "x0", "y0",
/// "w200", "h100", "T10" and "Password". The letter may be written in
/// any case, and the number may be left out.
///
/// An option that v2 does not know is refused with the words of v2:
/// "Invalid option.".
///
/// Measured on the binary of v2.0.19 on 2026-10-29.
fn the_options_of_the_input_box(options: &str, line: u32) -> Result<InputOptions, RuntimeError> {
    match the_options_of_the_input_box_read(options) {
        Ok(read) => Ok(read),
        Err(_) => Err(invalid_option(line)),
    }
}

/// The options of an input box, and the token that v2 does not know.
///
/// The token comes back with the failure, because v2 carries it in the
/// `Extra` of the failure.
fn the_options_of_the_input_box_read(options: &str) -> Result<InputOptions, String> {
    let mut read = InputOptions::default();
    for token in options.split([' ', '\t']).filter(|token| !token.is_empty()) {
        let mut letters = token.chars();
        let letter = letters.next().unwrap_or_default().to_ascii_lowercase();
        let rest = letters.as_str();
        match letter {
            'x' => read.x = the_place_of_an_option(rest, token)?,
            'y' => read.y = the_place_of_an_option(rest, token)?,
            'w' => read.width = the_size_of_an_option(rest, token)?,
            'h' => read.height = the_size_of_an_option(rest, token)?,
            't' => read.timeout = the_timeout_of_an_option(rest, token)?,
            'p' => {
                // The password is the one option that is a word, and the
                // mask of v2 is the character that follows it.
                if !rest.to_ascii_lowercase().starts_with("assword") {
                    return Err(token.to_string());
                }
                read.password = true;
            }
            _ => return Err(token.to_string()),
        }
    }
    Ok(read)
}

/// The option of an input box that v2 does not know.
///
/// v2 carries the option in the `Extra` of the failure, and the tree
/// reads it there too.
pub(crate) fn the_option_that_failed(options: &str) -> Option<String> {
    the_options_of_the_input_box_read(options).err()
}

/// The place of the window: a number that may be negative.
fn the_place_of_an_option(rest: &str, token: &str) -> Result<Option<i64>, String> {
    match the_number_of_an_option(rest) {
        Ok(Some(number)) => Ok(Some(number as i64)),
        Ok(None) => Ok(None),
        Err(()) => Err(token.to_string()),
    }
}

/// The size of the window: a number that may not be negative.
fn the_size_of_an_option(rest: &str, token: &str) -> Result<Option<i64>, String> {
    match the_number_of_an_option(rest) {
        Ok(Some(number)) if number >= 0.0 => Ok(Some(number as i64)),
        Ok(Some(_)) => Err(token.to_string()),
        Ok(None) => Ok(None),
        Err(()) => Err(token.to_string()),
    }
}

/// The seconds the window waits for the user.
fn the_timeout_of_an_option(rest: &str, token: &str) -> Result<Option<f64>, String> {
    match the_number_of_an_option(rest) {
        Ok(Some(number)) => Ok(Some(number)),
        Ok(None) => Ok(None),
        Err(()) => Err(token.to_string()),
    }
}

/// The number that follows a letter of the options.
///
/// The number may be left out, and then the option says nothing.
fn the_number_of_an_option(rest: &str) -> Result<Option<f64>, ()> {
    if rest.is_empty() {
        return Ok(None);
    }
    rest.parse::<f64>().map(Some).map_err(|_| ())
}

/// The options of a file selection, read the way v2 reads them.
///
/// The options are one letter -- "D", "M" or "S" -- with a number
/// after it, or a number alone. An option that v2 does not know is
/// refused with the words of v2: "Parameter #1 of FileSelect is
/// invalid.".
///
/// Measured on the binary of v2.0.19 on 2026-10-29.
fn the_options_of_the_file_select(options: &str, line: u32) -> Result<FileOptions, RuntimeError> {
    let mut read = FileOptions::default();
    let trimmed = options.trim();
    if trimmed.is_empty() {
        return Ok(read);
    }
    let mut letters = trimmed.chars();
    let first = letters.next().unwrap_or_default();
    let rest = letters.as_str();
    // A number alone is the sum of the flags of the options.
    if first.is_ascii_digit() || first == '-' {
        read.flags = trimmed.parse::<i64>().map_err(|_| invalid_file_option(line))?;
        return Ok(read);
    }
    let flags = if rest.is_empty() {
        0
    } else {
        rest.parse::<i64>().map_err(|_| invalid_file_option(line))?
    };
    match first.to_ascii_lowercase() {
        'd' => read.directory = true,
        'm' => read.multiple = true,
        's' => read.save = true,
        _ => return Err(invalid_file_option(line)),
    }
    read.flags = flags;
    Ok(read)
}

/// The refusal of an option of an input box that v2 does not know.
///
/// The option that was not known is in the `Extra` of the failure, and
/// the runtime reads it there.
fn invalid_option(line: u32) -> RuntimeError {
    RuntimeError { message: "Invalid option.".to_string(), line }
}

/// The refusal of an option of a file selection that v2 does not know.
fn invalid_file_option(line: u32) -> RuntimeError {
    RuntimeError {
        message: crate::runtime::invalid_parameter(1, "FileSelect"),
        line,
    }
}

// -- the helpers ----------------------------------------------------------------

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values.get(index).map(|value| value.to_text()).unwrap_or_default()
}
