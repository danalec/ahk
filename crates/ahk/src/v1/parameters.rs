//! The parameters of the commands of the v1 that the v1 checks.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! command whose parameter the v1 does not want is refused when the script
//! loads -- before any line of it runs -- and the complaint of the v1 is
//! "Parameter #N invalid.\n     Specifically: {the text that the script
//! wrote}". The text is the text of the parameter, and not its value: the
//! `SetTitleMatchMode, 1+1` is the "1+1".
//!
//! The command that reads one parameter takes the whole rest of the line:
//! the text that is left of the line goes into it, with a comma and a
//! space between -- the "AutoTrim, On, extra" is the "On, extra". The
//! command that reads more than one takes the text of each place, and the
//! text that is left of the line is not read.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{CaseSense, FloatFormat, IntegerFormat};

/// The refusal of a parameter of a command of the v1.
///
/// The name is the name of the command as the script wrote it, and the
/// parameters are the texts that the script wrote. `None` means that the
/// command is not one that the v1 checks, or that the parameters are the
/// ones that it wants.
pub(crate) fn the_refusal_of_a_parameter(name: &str, parameters: &[String]) -> Option<String> {
    let rule = RULES.iter().find(|rule| rule.name.eq_ignore_ascii_case(name))?;
    let texts = rule.the_texts(parameters);
    let place = (rule.check)(&texts)?;
    let text = the_text_at(&texts, place);
    Some(format!("Parameter #{} invalid.\n     Specifically: {text}", place + 1))
}

/// The rule of a command of the v1 that checks a parameter.
struct Rule {
    /// The name of the command, as the table of the v1 spells it.
    name: &'static str,
    /// How many parameters the command reads.
    reads: usize,
    /// The check of the parameters: the answer is the place of the
    /// parameter that the v1 does not want.
    check: fn(&[String]) -> Option<usize>,
}

impl Rule {
    /// The texts of the parameters that the command reads.
    ///
    /// The command that reads one parameter takes the whole rest of the
    /// line: the text that is left of the line goes into it, with a comma
    /// and a space between.
    fn the_texts(&self, parameters: &[String]) -> Vec<String> {
        if self.reads == 1 {
            let mut text = String::new();
            for (index, parameter) in parameters.iter().enumerate() {
                if index > 0 {
                    text.push_str(", ");
                }
                text.push_str(parameter.trim());
            }
            return vec![text];
        }
        parameters
            .iter()
            .map(|parameter| parameter.trim().to_string())
            .collect()
    }
}

/// The commands of the v1 that check a parameter, and the check of each.
const RULES: &[Rule] = &[
    Rule { name: "AutoTrim", reads: 1, check: the_switch },
    Rule { name: "DetectHiddenText", reads: 1, check: the_switch },
    Rule { name: "DetectHiddenWindows", reads: 1, check: the_switch },
    Rule { name: "SetStoreCapsLockMode", reads: 1, check: the_switch },
    Rule { name: "StringCaseSense", reads: 1, check: the_case_sense },
    Rule { name: "SetTitleMatchMode", reads: 1, check: the_title_match_mode },
    Rule { name: "SendMode", reads: 1, check: the_send_mode },
    Rule { name: "SetRegView", reads: 1, check: the_reg_view },
    Rule { name: "FileEncoding", reads: 1, check: the_encoding },
    Rule { name: "SetBatchLines", reads: 1, check: the_batch_lines },
    Rule { name: "SetFormat", reads: 2, check: the_format },
    Rule { name: "Transform", reads: 4, check: the_transform },
    Rule { name: "StringGetPos", reads: 5, check: the_occurrence },
    Rule { name: "FileGetTime", reads: 3, check: the_time_of_a_file },
    Rule { name: "FileGetSize", reads: 3, check: the_size_of_a_file },
    Rule { name: "FileSetAttrib", reads: 2, check: the_attributes_of_a_file },
];

/// The text of the parameter at this place, or the empty text.
fn the_text_at(texts: &[String], place: usize) -> &str {
    texts.get(place).map(String::as_str).unwrap_or("")
}

/// The switch of the v1: "On", "Off" and the numbers one and zero.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the case of the
/// words does not matter, and the numbers one and zero are the two
/// switches. The word that is not one of them is refused: the "true" is
/// not a switch of the v1.
fn the_switch(texts: &[String]) -> Option<usize> {
    if the_switch_of_the_v1(the_text_at(texts, 0)).is_some() {
        return None;
    }
    Some(0)
}

/// The switch of the v1: the sense that the text names.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the case of the
/// words does not matter, and the numbers one and zero are the two
/// switches. The word that is not one of them is refused: the "true" is
/// not a switch of the v1.
pub(crate) fn the_switch_of_the_v1(text: &str) -> Option<bool> {
    match text.trim().to_lowercase().as_str() {
        "on" | "1" => Some(true),
        "off" | "0" => Some(false),
        _ => None,
    }
}

/// The sense of the case of the v1: the sense that the text names.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the case of the
/// words does not matter, and the numbers one and zero are the two
/// switches.
fn the_case_sense(texts: &[String]) -> Option<usize> {
    if the_case_sense_of_the_v1(the_text_at(texts, 0)).is_some() {
        return None;
    }
    Some(0)
}

/// The sense of the case of the v1: the sense that the text names.
pub(crate) fn the_case_sense_of_the_v1(text: &str) -> Option<CaseSense> {
    match text.trim().to_lowercase().as_str() {
        "on" | "1" => Some(CaseSense::On),
        "off" | "0" => Some(CaseSense::Off),
        "locale" => Some(CaseSense::Locale),
        _ => None,
    }
}

/// The mode of the matching of the titles of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the mode is the
/// number one, two or three, or the word "RegEx", and the words "Fast" and
/// "Slow" are the speed of the matching. The zero and the four are not
/// modes.
fn the_title_match_mode(texts: &[String]) -> Option<usize> {
    match the_text_at(texts, 0).trim().to_lowercase().as_str() {
        "1" | "2" | "3" | "regex" | "fast" | "slow" => None,
        _ => Some(0),
    }
}

/// The method of the sending of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the method is
/// "Event", "Input", "Play" or "InputThenPlay", and the case of the word
/// does not matter.
fn the_send_mode(texts: &[String]) -> Option<usize> {
    match the_text_at(texts, 0).trim().to_lowercase().as_str() {
        "event" | "input" | "play" | "inputthenplay" => None,
        _ => Some(0),
    }
}

/// The view of the registry of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the view is the
/// number thirty-two, the number sixty-four or the word "Default", and the
/// case of the word does not matter.
fn the_reg_view(texts: &[String]) -> Option<usize> {
    match the_text_at(texts, 0).trim().to_lowercase().as_str() {
        "32" | "64" | "default" => None,
        _ => Some(0),
    }
}

/// The encoding of the files of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the encoding is
/// "UTF-8", "UTF-8-RAW", "UTF-16" or "UTF-16-RAW", or a code page -- the
/// "CP0" is the code page of the machine. The text that is empty is the
/// encoding that is there, and the words "ANSI" and "OEM" are not
/// encodings of the v1.
fn the_encoding(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 0).trim().to_lowercase();
    if text.is_empty() {
        return None;
    }
    match text.as_str() {
        "utf-8" | "utf-8-raw" | "utf-16" | "utf-16-raw" => None,
        // The name of a code page is the "CP" and a number, and the case of
        // the letters does not matter.
        _ if crate::settings::is_a_code_page(&text.to_uppercase()) => None,
        _ => Some(0),
    }
}

/// The number of the lines of the v1: a whole number, and the hex is one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the number is a
/// whole number of base ten -- the "007" is seven -- or a hex -- the
/// "0x10" is sixteen. The "20.5" and the "1e2" are not numbers.
fn the_batch_lines(texts: &[String]) -> Option<usize> {
    if is_a_whole_number_of_the_lines(the_text_at(texts, 0)) {
        return None;
    }
    Some(0)
}

/// Whether the text is the whole number that the `SetBatchLines` wants.
pub(crate) fn is_a_whole_number_of_the_lines(text: &str) -> bool {
    let text = text.trim();
    if text.parse::<i64>().is_ok() {
        return true;
    }
    crate::v1::the_text_is_a_hex(text)
}

/// The format of the numbers of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the type of the
/// number is "Integer", "IntegerFast", "Float" or "FloatFast", and the
/// text that begins with one of them is the same one -- the "IntegerX" is
/// the "Integer". The format of the whole numbers is "D", "H" or "h", and
/// the text that begins with one of them is the same one -- the "H2" is
/// the "H". The format of the floats is "<width>.<decimals>" with an
/// optional "e" for the scientific notation.
fn the_format(texts: &[String]) -> Option<usize> {
    match the_kind_of_a_number(the_text_at(texts, 0)) {
        Some(TheKindOfANumber::Integer) => {
            if the_format_of_the_whole_numbers(the_text_at(texts, 1)).is_some() {
                return None;
            }
            Some(1)
        }
        Some(TheKindOfANumber::Float) => {
            if the_format_of_the_floats(the_text_at(texts, 1)).is_some() {
                return None;
            }
            Some(1)
        }
        None => Some(0),
    }
}

/// The kind of the number whose format the `SetFormat` sets.
pub(crate) enum TheKindOfANumber {
    /// The whole numbers: "Integer" and "IntegerFast".
    Integer,
    /// The floats: "Float" and "FloatFast".
    Float,
}

/// The kind of the number that the text names.
pub(crate) fn the_kind_of_a_number(text: &str) -> Option<TheKindOfANumber> {
    let text = text.trim().to_lowercase();
    if text.starts_with("integer") {
        return Some(TheKindOfANumber::Integer);
    }
    if text.starts_with("float") {
        return Some(TheKindOfANumber::Float);
    }
    None
}

/// The format of the whole numbers of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the "D" is the base
/// ten, the "H" is the base sixteen with the prefix and the upper case
/// digits, and the "h" is the same with the lower case ones.
pub(crate) fn the_format_of_the_whole_numbers(text: &str) -> Option<IntegerFormat> {
    match text.trim().chars().next() {
        Some('D') | Some('d') => Some(IntegerFormat::Decimal),
        Some('H') => Some(IntegerFormat::Hexadecimal),
        Some('h') => Some(IntegerFormat::HexadecimalLower),
        _ => None,
    }
}

/// The format of the floats of the v1, when the text is one.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the format is
/// "<width>.<decimals>", and the width may be in zero ("06.2") or in minus
/// ("-6.2") for the padding, and the "e" at the end asks for the scientific
/// notation.
pub(crate) fn the_format_of_the_floats(text: &str) -> Option<FloatFormat> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let (body, scientific) = match text.strip_suffix('e') {
        Some(body) => (body, true),
        None => match text.strip_suffix('E') {
            Some(body) => (body, true),
            None => (text, false),
        },
    };
    let (width, decimals) = body.split_once('.')?;
    let (left, width) = match width.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, width),
    };
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the width "0" is
    // the width zero, and the zeroes that pad the text are a zero in front
    // of a width that has more digits ("06.2").
    let (zeroes, width) = match width.strip_prefix('0') {
        Some(rest) if !rest.is_empty() => (true, rest),
        _ => (false, width),
    };
    let width: u32 = width.parse().ok()?;
    let decimals: u32 = decimals.parse().ok()?;
    Some(FloatFormat {
        width,
        zeroes,
        left,
        decimals,
        scientific,
        upper: text.ends_with('E'),
    })
}

/// The sub-command of the `Transform` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the sub-command
/// that the v1 does not know is refused with "Parameter #2 invalid.", and
/// the case of the word does not matter.
fn the_transform(texts: &[String]) -> Option<usize> {
    if is_a_sub_command_of_the_transform(the_text_at(texts, 1)) {
        return None;
    }
    Some(1)
}

/// Whether the text is a sub-command of the `Transform` that the v1 knows.
pub(crate) fn is_a_sub_command_of_the_transform(text: &str) -> bool {
    matches!(
        text.trim().to_lowercase().as_str(),
        "asc"
            | "chr"
            | "mod"
            | "exp"
            | "sqrt"
            | "log"
            | "ln"
            | "round"
            | "ceil"
            | "floor"
            | "abs"
            | "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "pow"
            | "bitnot"
            | "bitand"
            | "bitor"
            | "bitxor"
            | "bitshiftleft"
            | "bitshiftright"
            | "deref"
            | "html"
    )
}

/// The occurrence of the `StringGetPos` of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the occurrence is
/// the letter "L" or "R" and the number of the one that the caller wants,
/// and the number that is alone is the first from the right. The zero is
/// not an occurrence.
fn the_occurrence(texts: &[String]) -> Option<usize> {
    if is_an_occurrence(the_text_at(texts, 3)) {
        return None;
    }
    Some(3)
}

/// Whether the text is the occurrence that the `StringGetPos` wants.
pub(crate) fn is_an_occurrence(text: &str) -> bool {
    let text = text.trim();
    let Some(first) = text.chars().next() else {
        // The occurrence that is not there is the first from the left.
        return true;
    };
    matches!(first, 'L' | 'l' | 'R' | 'r' | '1')
}

/// The time of a file that the v1 asks for.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the time is the one
/// of the modification -- the "M" and the "m" -- or the one of the access --
/// the "a". The creation and the last access of the documentation are not
/// times that this build knows.
fn the_time_of_a_file(texts: &[String]) -> Option<usize> {
    match the_text_at(texts, 2).trim().to_lowercase().as_str() {
        "m" | "a" => None,
        _ => Some(2),
    }
}

/// The size of a file that the v1 asks for.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the size is the one
/// of the kilobytes -- the "K" -- or the one of the megabytes -- the "M".
/// The gigabytes are a unit that the v1 does not have.
fn the_size_of_a_file(texts: &[String]) -> Option<usize> {
    match the_text_at(texts, 2).trim().to_lowercase().as_str() {
        "k" | "m" => None,
        _ => Some(2),
    }
}

/// The attributes of a file that the v1 sets.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the attributes are
/// the letters of the ones that the v1 knows -- the "R" of the read-only,
/// the "A" of the archive, the "H" of the hidden, the "S" of the system,
/// the "N" of the normal, the "T" of the temporary and the "O" of the
/// offline -- and the signs that come before them. The sparse and the
/// encrypted are not attributes that the v1 knows.
fn the_attributes_of_a_file(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 0).trim();
    let wanted = text.chars().all(|character| {
        matches!(
            character,
            '+' | '-'
                | 'R'
                | 'r'
                | 'A'
                | 'a'
                | 'H'
                | 'h'
                | 'S'
                | 's'
                | 'N'
                | 'n'
                | 'T'
                | 't'
                | 'O'
                | 'o'
        )
    });
    if wanted {
        return None;
    }
    Some(0)
}
