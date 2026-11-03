//! The loop of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the `Loop`
//! of the v1 is a loop of files when the text of its first parameter is not a
//! number, and a loop of a number of turns when it is. The second parameter
//! is the mode of the loop of the files, and the third one is the flag of the
//! recursion.
//!
//! The mode is a number: the zero gives the files, the one gives the files and
//! the folders, and the two gives the folders alone. A text that is not a
//! number is the mode of the files, and a number that is not one of the three
//! is refused when the script loads: "Parameter #2 invalid.\n     Specifically:
//! {the text that the script wrote}".
//!
//! The flag of the recursion is a zero or a one, and the text that is not one
//! of them is refused when the script loads: "Parameter #3 invalid.\n
//!      Specifically: {the text that the script wrote}". The v1 checks the
//! two of them when the script loads, and it waits for the run when the text
//! carries the name of a variable.

use alloc::format;
use alloc::string::{String, ToString};

use super::numbers::the_whole_number_at_the_start_of_a_text;

/// The mode of the loop of the files of the v1: the files alone.
pub(crate) const THE_FILES: u8 = 0;
/// The mode of the loop of the files of the v1: the files and the folders.
pub(crate) const THE_FILES_AND_THE_FOLDERS: u8 = 1;
/// The mode of the loop of the files of the v1: the folders alone.
pub(crate) const THE_FOLDERS: u8 = 2;

/// The mode of the loop of the files of the v1.
///
/// The text is trimmed, and the number that it holds is the whole number that
/// starts at its beginning: the "1e3" is one and the "2.7" is two. A text
/// that does not start with a number is the mode of the files, and a number
/// that is not one of the three modes is refused.
pub(crate) fn the_mode_of_the_loop_of_the_files(text: &str) -> Option<u8> {
    match the_whole_number_at_the_start_of_a_text(text) {
        None => Some(THE_FILES),
        Some(number) if (0..=2).contains(&number) => Some(number as u8),
        Some(_) => None,
    }
}

/// The refusal of the mode of the loop of the files of the v1.
///
/// The v1 waits for the run when the text carries the name of a variable: the
/// value of it is the mode then, and the v1 refuses the value that is not one
/// of the three modes.
pub(crate) fn the_refusal_of_the_mode(text: &str) -> Option<String> {
    if text.contains('%') {
        return None;
    }
    if the_mode_of_the_loop_of_the_files(text).is_some() {
        return None;
    }
    Some(format!("Parameter #2 invalid.\n     Specifically: {}", text.trim()))
}

/// The refusal of the flag of the recursion of the loop of the files of the v1.
///
/// The flag is a zero or a one. The v1 waits for the run when the text
/// carries the name of a variable, and the value that is not a zero or a one
/// is not a failure then: the loop does not walk into the folders.
pub(crate) fn the_refusal_of_the_flag_of_the_recursion(text: &str) -> Option<String> {
    if text.contains('%') {
        return None;
    }
    let text = text.trim();
    // The parameter that is not there is a flag that does not walk into the
    // folders, and it is not a failure.
    if text.is_empty() || text == "0" || text == "1" {
        return None;
    }
    Some(format!("Parameter #3 invalid.\n     Specifically: {text}"))
}

/// Whether the flag of the recursion asks for the folders.
///
/// The v1 reads the flag when the loop runs, and the text that is not a one
/// is a loop that does not walk into the folders.
pub(crate) fn the_flag_asks_for_the_recursion(text: &str) -> bool {
    text.trim() == "1"
}

/// The refusal of the parameters of the loop of the v1.
///
/// The first parameter of the loop is the count or the pattern of the files,
/// the second one is the mode, and the third one is the flag of the
/// recursion. The v1 checks the two of them when the script loads.
pub(crate) fn the_refusal_of_the_parameters_of_the_loop(parameters: &[String]) -> Option<String> {
    if let Some(text) = parameters.get(1) {
        if let Some(refusal) = the_refusal_of_the_mode(text) {
            return Some(refusal);
        }
    }
    if let Some(text) = parameters.get(2) {
        if let Some(refusal) = the_refusal_of_the_flag_of_the_recursion(text) {
            return Some(refusal);
        }
    }
    None
}

/// The refusal of the kind of the loop of the v1 that the tree does not carry.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the first
/// parameter of the `Loop` of the v1 names the kind of the loop when it is
/// "Read" or "Parse", and it is a loop of the registry when it begins with
/// one of the roots of it. The "Read" and the "Parse" name a kind of the loop
/// when there is another parameter after them: the `Loop, Read` alone is a
/// loop of the files that match the pattern "Read".
pub(crate) fn the_refusal_of_the_kind_of_the_loop(parameters: &[String]) -> Option<String> {
    let first = parameters.first()?.trim().to_string();
    if parameters.len() > 1 {
        match first.to_lowercase().as_str() {
            "read" => return Some("the read loop of the v1 is not carried yet.".to_string()),
            "parse" => return Some("the parse loop of the v1 is not carried yet.".to_string()),
            _ => {}
        }
    }
    if super::sub_commands::the_text_begins_with_a_root_of_the_registry(&first) {
        return Some("the registry is a Windows thing and this box has none.".to_string());
    }
    None
}
