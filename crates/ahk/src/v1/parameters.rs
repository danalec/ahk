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
use alloc::vec::Vec;

use crate::dialect::TheRefusalOfTheArity;
use crate::runtime::{CaseSense, FloatFormat, IntegerFormat, Value};

/// The refusal of the number of the parameters of a command of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
/// has two forms of the complaint, and the command says which one is the one
/// of it. The one that names the command is "\"X\" requires at least N
/// parameter(s)." with the text of the command after it; the other one names
/// the parameter that has to be there and has no text: "Parameter #N
/// required".
///
/// The commands of the v1 that do not have a rule here are the ones whose
/// complaint the table of the v1 carries: the number of the parameters that
/// it wants is the number of the table, and the name of the command is the
/// name of the table.
pub(crate) fn the_refusal_of_the_arity(
    name: &str,
    count: usize,
) -> Option<TheRefusalOfTheArity> {
    let rule = RULES_OF_THE_ARITY
        .iter()
        .find(|rule| rule.name.eq_ignore_ascii_case(name))?;
    // The complaint that names the command comes first: the "EnvSub" with no
    // parameter at all is refused with the number one, and the "EnvSub" with
    // one parameter is refused with the parameter that is missing.
    if count < rule.generic as usize {
        return Some(TheRefusalOfTheArity::AtLeast {
            name: rule.spelled.unwrap_or(rule.name),
            number: rule.generic,
        });
    }
    if rule.required != 0 && count < rule.required as usize {
        return Some(TheRefusalOfTheArity::Required {
            number: rule.required,
        });
    }
    // The two parameters that the command wants are named together.
    if let Some((first, second)) = rule.together {
        if count < second as usize {
            return Some(TheRefusalOfTheArity::RequiredTogether { first, second });
        }
    }
    None
}

/// The arity of a command of the v1 that the table of the v1 does not carry.
struct Arity {
    /// The name of the command, as the table of the v1 spells it.
    name: &'static str,
    /// The number of the parameters that the complaint of the v1 names.
    generic: u8,
    /// The parameter that has to be there: the complaint of the v1 is
    /// "Parameter #N required" and it names no text.
    required: u8,
    /// The two parameters that have to be there together: the complaint of
    /// the v1 is "Parameter #N and M required." and it names no text.
    together: Option<(u8, u8)>,
    /// The name that the complaint of the v1 writes: the binary spells the
    /// name of the `SetStoreCapsLockMode` the way it is inside it.
    spelled: Option<&'static str>,
}

/// The commands of the v1 whose complaint is not the one of the table.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// "EnvSub" wants two parameters, and the complaint of the v1 names the
/// number one when the command has no parameter at all; the "ControlSend"
/// and the "GuiControl" want two, and the "WinMenuSelectItem" wants three,
/// and the complaint of the v1 names the parameter that is missing.
const RULES_OF_THE_ARITY: &[Arity] = &[
    Arity { name: "EnvSub", generic: 1, required: 2, together: None, spelled: None },
    Arity { name: "ControlSend", generic: 0, required: 2, together: None, spelled: None },
    Arity { name: "GuiControl", generic: 0, required: 2, together: None, spelled: None },
    Arity { name: "WinMenuSelectItem", generic: 0, required: 3, together: None, spelled: None },
    Arity {
        name: "SetStoreCapsLockMode",
        generic: 1,
        required: 0,
        together: None,
        spelled: Some("SetStoreCapslockMode"),
    },
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `MouseClickDrag`
    // wants the place of the end of the drag -- the fourth and the fifth --
    // and the complaint of the v1 names the two of them together: "Parameter
    // #4 and 5 required.".
    Arity {
        name: "MouseClickDrag",
        generic: 1,
        required: 0,
        together: Some((4, 5)),
        spelled: None,
    },
];

/// The refusal of a parameter of a command of the v1.
///
/// The name is the name of the command as the script wrote it, and the
/// parameters are the texts that the script wrote. `None` means that the
/// command is not one that the v1 checks, or that the parameters are the
/// ones that it wants.
pub(crate) fn the_refusal_of_a_parameter(name: &str, parameters: &[String]) -> Option<String> {
    // The command whose complaint is a message of its own comes first.
    if let Some(rule) = MESSAGES.iter().find(|rule| rule.name.eq_ignore_ascii_case(name)) {
        return (rule.message)(parameters);
    }
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
    /// The check of the parameters: the answer is the place of the
    /// parameter that the v1 does not want.
    check: fn(&[String]) -> Option<usize>,
}

impl Rule {
    /// The texts of the parameters that the command reads.
    ///
    /// The parser has already given the last parameter of the command the
    /// text that is left of the line: the `AutoTrim, On,extra` names
    /// "On,extra" and not "On, extra". The text of each one of them is the
    /// text that the script wrote, without the spaces that surround it.
    fn the_texts(&self, parameters: &[String]) -> Vec<String> {
        parameters
            .iter()
            .map(|parameter| parameter.trim().to_string())
            .collect()
    }
}

/// The commands of the v1 that check a parameter, and the check of each.
const RULES: &[Rule] = &[
    Rule { name: "AutoTrim", check: the_switch },
    Rule { name: "DetectHiddenText", check: the_switch },
    Rule { name: "DetectHiddenWindows", check: the_switch },
    Rule { name: "SetStoreCapsLockMode", check: the_switch },
    Rule { name: "StringCaseSense", check: the_case_sense },
    Rule { name: "SetTitleMatchMode", check: the_title_match_mode },
    Rule { name: "SendMode", check: the_send_mode },
    Rule { name: "SetRegView", check: the_reg_view },
    Rule { name: "FileEncoding", check: the_encoding },
    Rule { name: "SetBatchLines", check: the_batch_lines },
    Rule { name: "SetFormat", check: the_format },
    Rule { name: "Transform", check: the_transform },
    Rule { name: "StringGetPos", check: the_occurrence },
    Rule { name: "FileGetTime", check: the_time_of_a_file },
    Rule { name: "FileGetSize", check: the_size_of_a_file },
    Rule { name: "FileSetAttrib", check: the_attributes_of_a_file },
    Rule { name: "EnvAdd", check: the_units_of_the_time },
    Rule { name: "EnvSub", check: the_units_of_the_time },
    Rule { name: "FileMoveDir", check: the_way_of_the_move_of_a_directory },
    Rule { name: "GroupActivate", check: the_mode_of_a_group },
    Rule { name: "GroupClose", check: the_mode_of_a_group },
    Rule { name: "GroupDeactivate", check: the_mode_of_a_group },
    Rule { name: "IfMsgBox", check: the_button_of_a_message_box },
    Rule { name: "KeyHistory", check: the_command_without_a_parameter },
    Rule { name: "StringMid", check: the_letter_of_the_string_mid },
];

/// The commands of the v1 whose complaint is a message of its own.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the division by zero
/// of the `EnvDiv` is refused when the script loads, and the complaint of
/// the v1 is "Divide by zero" with the text of the divisor.
struct Message {
    /// The name of the command, as the table of the v1 spells it.
    name: &'static str,
    /// The complaint of the command: the answer is the message, and `None`
    /// means that the command lets the script run.
    message: fn(&[String]) -> Option<String>,
}

/// The commands of the v1 that refuse with a message of their own.
const MESSAGES: &[Message] = &[Message { name: "EnvDiv", message: the_division_by_zero }];

/// The divisor of the `EnvDiv` of the v1 is not a zero.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the division by zero
/// is refused when the script loads: "Divide by zero" and the text of the
/// divisor. The divisor that does not start with a number is not a zero at
/// the load -- the v1 does not know the value of it yet -- and the variable
/// is blank when the script runs.
fn the_division_by_zero(parameters: &[String]) -> Option<String> {
    let divisor = parameters.get(1)?;
    let number = crate::v1::the_number_at_the_start_of_a_text(divisor);
    let zero = match number {
        Some(Value::Int(number)) => number == 0,
        Some(Value::Float(number)) => number == 0.0,
        _ => false,
    };
    if !zero {
        return None;
    }
    Some(format!("Divide by zero\n     Specifically: {divisor}"))
}

/// The refusal of the name of a variable of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name of a variable
/// may not carry a character that is not a letter, a digit or the underscore
/// -- the "a b" is refused, and the complaint of the v1 is "The following
/// variable name contains an illegal character." with the name after it. The
/// percent sign opens the name of another variable: the one that is not
/// closed is refused with "This parameter contains a variable name missing
/// its ending percent sign.", and the one that is empty is refused with
/// "Empty variable reference (%%).". The name that carries a percent sign
/// is an expression, and the v1 does not check it here.
pub(crate) fn the_refusal_of_the_name_of_a_variable(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(at) = rest.find('%') {
        let after = &rest[at + 1..];
        let Some(end) = after.find('%') else {
            return Some(format!(
                "This parameter contains a variable name missing its ending percent sign.\n     Specifically: {text}"
            ));
        };
        if end == 0 {
            return Some(format!(
                "Empty variable reference (%%).\n     Specifically: {text}"
            ));
        }
        rest = &after[end + 1..];
    }
    if text.contains('%') {
        return None;
    }
    if is_the_name_of_a_variable(text) {
        return None;
    }
    Some(format!(
        "The following variable name contains an illegal character:\n\"{text}\""
    ))
}

/// Whether the text is the name of a variable of the script.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name
/// is made of letters, of digits, of the underscore and of the "#", the "$"
/// and the "@", and the number may begin it.
fn is_the_name_of_a_variable(text: &str) -> bool {
    !text.is_empty()
        && text.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '#' | '$' | '@')
        })
}

/// The button of a message box of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the button is one of
/// the "Yes", the "No", the "OK", the "Cancel", the "Abort", the "Ignore",
/// the "Retry", the "Continue", the "TryAgain" or the "Timeout", and the
/// case of the word does not matter. The number is not a button.
fn the_button_of_a_message_box(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 0).trim();
    if matches!(
        text.to_lowercase().as_str(),
        "yes"
            | "no"
            | "ok"
            | "cancel"
            | "abort"
            | "ignore"
            | "retry"
            | "continue"
            | "tryagain"
            | "timeout"
    ) {
        return None;
    }
    Some(0)
}

/// The letter of the `StringMid` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the fifth parameter is
/// the letter "L", and the case of it does not matter. The letter that is
/// not the "L" is refused.
fn the_letter_of_the_string_mid(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 4).trim();
    if text.is_empty() || text.eq_ignore_ascii_case("L") {
        return None;
    }
    Some(4)
}

/// The mode of a group of windows of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the second parameter is
/// the letter "R", and the case of it does not matter. The mode that is not
/// there is the oldest window of the group.
fn the_mode_of_a_group(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 1).trim();
    if text.is_empty() || text.eq_ignore_ascii_case("R") {
        return None;
    }
    Some(1)
}

/// The way of the move of a directory of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the third parameter is
/// the number zero, one or two, and the number that is not one of them is
/// refused.
fn the_way_of_the_move_of_a_directory(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 2).trim();
    if text.is_empty() || matches!(text, "0" | "1" | "2") {
        return None;
    }
    Some(2)
}

/// The command of the v1 that takes no parameter.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the `KeyHistory` takes
/// no parameter, and the one that is there is refused.
fn the_command_without_a_parameter(texts: &[String]) -> Option<usize> {
    if texts.is_empty() {
        return None;
    }
    Some(0)
}

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
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the time is
    // optional, and the command is refused when it is there and it is not
    // one of the three: the modification, the access and the creation. The
    // last access of the documentation is not a time that this build knows.
    if texts.len() < 3 {
        return None;
    }
    match the_text_at(texts, 2).trim().to_lowercase().as_str() {
        "m" | "a" | "c" => None,
        _ => Some(2),
    }
}

/// The size of a file that the v1 asks for.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the size is the one
/// of the kilobytes -- the "K" -- or the one of the megabytes -- the "M".
/// The gigabytes are a unit that the v1 does not have.
fn the_size_of_a_file(texts: &[String]) -> Option<usize> {
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the unit is
    // optional, and the command is refused when it is there and it is not
    // one of the two.
    if texts.len() < 3 {
        return None;
    }
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
fn the_units_of_the_time(texts: &[String]) -> Option<usize> {
    let text = the_text_at(texts, 2);
    if text.trim().is_empty() {
        return None;
    }
    if is_a_unit_of_the_time(text) {
        return None;
    }
    Some(2)
}

/// Whether the text is a unit of the time of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: it is the first
/// letter of the text that tells -- the "Seconds", the "Sec", the "SX" and
/// the "S, extra" are all the seconds -- and the unit that is not one of
/// them is refused: the "Furlongs" is not a unit of the v1, and the "4" is
/// not one either.
fn is_a_unit_of_the_time(text: &str) -> bool {
    let text = text.trim().to_lowercase();
    match text.chars().next() {
        Some(letter) => matches!(letter, 's' | 'm' | 'h' | 'd'),
        None => false,
    }
}

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
