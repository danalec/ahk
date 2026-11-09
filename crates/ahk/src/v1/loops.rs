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
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the first
//! parameter of the `Loop` of the v1 names the kind of the loop when it is the
//! "Read" or the "Parse" and when another parameter comes after it: the
//! `Loop, Read` alone is a loop of the files that match the pattern "Read".
//! The second parameter of the `Loop, Parse` is the name of the variable whose
//! text is separated, and not its value.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
    Some(format!(
        "Parameter #2 invalid.\n     Specifically: {}",
        text.trim()
    ))
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

/// The refusal of the loop of the registry of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the first
/// parameter of the `Loop` of the v1 is a loop of the registry when it begins
/// with one of the roots of it.
pub(crate) fn the_refusal_of_the_registry_of_the_loop(parameters: &[String]) -> Option<String> {
    let first = parameters.first()?.trim().to_string();
    if super::sub_commands::the_text_begins_with_a_root_of_the_registry(&first) {
        return Some("the registry is a Windows thing and this box has none.".to_string());
    }
    None
}

/// The refusal of the name of the variable of the parse loop of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the second
/// parameter of the `Loop, Parse` is the name of the variable whose text is
/// separated, and the v1 checks the name when the script loads: the name that
/// carries a character that is not a letter, a digit or the underscore is
/// refused with "The following variable name contains an illegal character."
/// and the name after it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the name
/// that carries a percent sign is the name of another variable whose value is
/// the name: the `Loop, Parse, %x%, "|"` separates the text of the variable
/// that the value of `x` names. The v1 waits for the run to know the name, and
/// the tree waits for it too.
pub(crate) fn the_refusal_of_the_name_of_the_parse_loop(text: &str) -> Option<String> {
    let text = text.trim();
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    // percent sign and a space in front of the name make the text of the
    // expression the text that is separated, and the name is not checked.
    if super::parameters::the_text_is_an_expression_of_the_v1(text) {
        return None;
    }
    super::parameters::the_refusal_of_the_name_of_a_variable(text)
}

/// Whether the delimiters of the parse loop ask for the comma separated
/// values.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the word is
/// "CSV", and the case of it does not matter, and the spaces that are around
/// it do not matter either: the " CSV" is the comma separated value format.
pub(crate) fn the_delimiters_ask_for_the_comma_separated_values(text: &str) -> bool {
    text.trim().eq_ignore_ascii_case("CSV")
}

/// The fields of the text that is separated in the comma separated value
/// format.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the comma
/// separates the fields, and the field that begins with a quote is a field
/// that is quoted: the comma that is inside it is a literal, and the two
/// quotes that are together are one quote. The text that comes after the
/// quote that closes the field is left out: the `"a" ,b` is the field "a" and
/// the field "b". The field that does not begin with a quote is the text that
/// comes until the comma, and the quote that is inside it is a literal.
///
/// The characters that are left out are not used in this format.
pub(crate) fn the_fields_of_the_comma_separated_values(text: &str) -> Vec<String> {
    let characters: Vec<char> = text.chars().collect();
    let mut fields: Vec<String> = Vec::new();
    let mut index = 0usize;
    loop {
        let mut field = String::new();
        if characters.get(index) == Some(&'"') {
            // The field is quoted.
            index += 1;
            while index < characters.len() {
                if characters[index] != '"' {
                    field.push(characters[index]);
                    index += 1;
                    continue;
                }
                // The two quotes that are together are one quote.
                if characters.get(index + 1) == Some(&'"') {
                    field.push('"');
                    index += 2;
                    continue;
                }
                // The quote that closes the field.
                index += 1;
                break;
            }
            // The text that comes after the quote that closes the field is
            // left out.
            while index < characters.len() && characters[index] != ',' {
                index += 1;
            }
        } else {
            while index < characters.len() && characters[index] != ',' {
                field.push(characters[index]);
                index += 1;
            }
        }
        fields.push(field);
        if index >= characters.len() {
            break;
        }
        // The comma that ends the field.
        index += 1;
        if index >= characters.len() {
            // The comma at the end has a field of its own.
            fields.push(String::new());
            break;
        }
    }
    fields
}
