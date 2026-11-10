//! The lexer: source text in, tokens out.
//!
//! A port of the lexer of the archived Python implementation
//! (`../archived/ahk-python`), which is the reference and the one that was
//! verified against the lab box. The two are meant to agree on every
//! input, and the test suite in this crate says so.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;


/// What a token is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// A name: `Send`, `x`, `invoice`.
    Name,
    /// A keyword: the same words, in the places where they mean something.
    Keyword,
    /// A number: `42`, `3.5`.
    Number,
    /// A string: the text between the quotes.
    String,
    /// A text whose quote is not closed: the text that was read, and the
    /// place where the quote was.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// has no texts in the line of a command, and the quote that is not
    /// closed there is not an error: the `FileAppend, "abc, *` writes the
    /// text `"abc`. The parser refuses the text when it reads it as an
    /// expression.
    UnclosedText,
    /// An operator or a punctuation mark.
    Op,
    /// A directive: a name that begins with a hash.
    Directive,
    /// The end of a line.
    Newline,
    /// The end of the source.
    Eof,
}

/// One token, with the place it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: u32,
    pub column: u32,
    /// Whether a space came before this token.
    ///
    /// The dot is the reason: a dot with a space on both sides joins two
    /// values, and a dot with no space before it reaches into a value.
    /// The two are the same character and only the space tells them
    /// apart.
    pub spaced: bool,
    /// The quote of the text, when the token is one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a text is between
    /// double quotes or between single quotes, and the source of the script
    /// keeps the quote that was written: `Goto 'done'` looks for a label
    /// that is called `'done'`, with the quotes.
    pub quote: Option<char>,
}

/// The words the language gives a meaning to.
///
/// The operators that are words are here too: `and`, `or`, `not` and
/// `is` are operators and not synonyms of the symbols. `in` and
/// `contains` are reserved for a future of v2 and do nothing yet, but
/// they are reserved: a script cannot use them as names.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: three of the words
/// that the tree knew are not words of the dialect. The `elseif` is a
/// name -- the branch of an `if` is written with two words, `else if` --
/// and the `default` and the `extends` are names too: the label of the
/// default case of a switch and the base of a class are told apart by
/// what comes with them, and not by the word itself.
pub const KEYWORDS: &[&str] = &[
    "if", "else", "switch", "case", "return", "exitapp", "while", "until", "loop", "try", "catch",
    "finally", "throw", "for", "break", "continue", "goto", "true", "false", "and", "or", "not",
    "is", "in", "contains", "as", "class", "static", "global", "local", "super",
];

/// The reason of the refusal of a word that the language reserves.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the reason belongs to
/// the word, and it is the same in every place where the word is refused.
/// The words that begin a statement say that the word may not be the name
/// of a variable; the words that are operators, the two that are the
/// values of the dialect and the `super` are a syntax error; and the words
/// that are reserved for a future of v2 say that they are reserved.
///
/// The `ExitApp` and the `Class` are apart: the first is a function of the
/// machine and the second is a class of the dialect, and neither of them
/// may be written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRefusal {
    /// The word may not be the name of a variable.
    Variable,
    /// The word is a syntax error.
    Syntax,
    /// The word is reserved for a future of v2.
    Future,
    /// The word is the name of a function of the machine.
    Func,
    /// The word is the name of a class of the dialect.
    Class,
}

/// The end of the symbol of the v1 that carries a quote.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the name
/// that the v1 refuses is the symbol that comes with the word, and the
/// symbol ends at the space, at the comma, at the bracket that closes, at
/// the point of a member, at the colon and at the operators.
fn the_end_of_a_symbol_that_carries_a_quote(chars: &[char], start: usize) -> usize {
    let mut end = start;
    while end < chars.len() && !the_character_ends_a_symbol(chars[end]) {
        end += 1;
    }
    end
}

/// Whether the character ends the symbol of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// `a"b", y` is the symbol `a"b"` and the `a"b")` is the symbol `a"b"`,
/// and the `a"b"x` is the symbol `a"b"x`.
fn the_character_ends_a_symbol(character: char) -> bool {
    character.is_whitespace()
        || matches!(
            character,
            ',' | ')'
                | '}'
                | '{'
                | ':'
                | '.'
                | ';'
                | '='
                | '!'
                | '<'
                | '>'
                | '&'
                | '|'
                | '^'
                | '+'
                | '-'
                | '*'
                | '/'
                | '%'
                | '~'
                | '?'
                | '('
                | '['
        )
}

/// Whether the character ends the symbol of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// symbol of the v1 ends at the space and at the punctuation: the `;c` of
/// the `x := 1;c` is the symbol, and the `;` of the `x := 1;(c)` is one.
/// The point, the semicolon, the percent sign, the quotes and the marks are
/// a part of it: the `;c.` of the `x := 1;c.` is the symbol.
pub fn the_character_ends_the_symbol_of_the_v1(character: char) -> bool {
    character.is_whitespace()
        || matches!(
            character,
            ',' | ')'
                | '('
                | '}'
                | '{'
                | ']'
                | '['
                | ':'
                | '='
                | '!'
                | '<'
                | '>'
                | '&'
                | '|'
                | '^'
                | '+'
                | '-'
                | '*'
                | '/'
                | '~'
                | '?'
        )
}

/// The source of a text, the way the script wrote it.
///
/// The text of the v1 is between double quotes, and the quote that was
/// written is a part of the source of it.
fn the_source_of_a_text(chars: &[char], start: usize) -> String {
    let mut end = start + 1;
    while end < chars.len() && chars[end] != '"' && chars[end] != '\n' {
        end += 1;
    }
    if end < chars.len() && chars[end] == '"' {
        end += 1;
    }
    chars[start..end].iter().collect()
}

/// The reason of the refusal of a word, when the language reserves it.
///
/// The word is the one as the script wrote it, and the reason is the one
/// that the binary of v2 gives. A word that is not in the table is a name,
/// and the `elseif`, the `default` and the `extends` are names.
pub fn the_refusal_of_a_word(word: &str) -> Option<TheRefusal> {
    let word = word.to_lowercase();
    Some(match word.as_str() {
        "if" | "else" | "switch" | "case" | "return" | "while" | "until" | "loop" | "try"
        | "catch" | "finally" | "throw" | "for" | "break" | "continue" | "goto" | "static"
        | "global" | "local" => TheRefusal::Variable,
        "not" | "and" | "or" | "is" | "true" | "false" | "super" => TheRefusal::Syntax,
        "in" | "contains" | "as" => TheRefusal::Future,
        "exitapp" => TheRefusal::Func,
        "class" => TheRefusal::Class,
        _ => return None,
    })
}

/// The operators, longest first so that `:=` wins over `:`.
///
/// The list is the whole of v2's symbolic operators. The order matters
/// in one more way: `++` and `--` are not matched right after a number,
/// because `5--3` is a subtraction and not a decrement of a literal.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the table of the v2
/// has `===`, and its lexer does not match it: the three signs are the `==`
/// and the `=` of the assignment, and the script is refused with a missing
/// operand.
const OPERATORS: &[&str] = &[
    ">>>=", "<<=", "//=", ">>=", ":=", "+=", "-=", "*=", "/=", ".=", "|=", "^=", "&=", "~=", "!==",
    "==", "!=", "<=", ">=", "&&", "||", "??", "**", "//", "<<", ">>>", ">>", "++", "--", "::",
    "=>", "=", "<", ">", "+", "-", "*", "%", "/", ".", "(", ")", ",", "{", "}", "[", "]", "!", ":",
    "?", "&", "|", "^", "~",
];

/// The operators of the dialect, the longest first so that the longest match
/// wins.
///
/// The v1 knows the `<>`, which is the `!=` of the v2, and the v2 does not
/// know it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04.
/// Whether the line that begins at this place holds the two colons of a
/// hotkey before it ends.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the `#` that begins a line
/// is the `#` of the Win modifier when the line carries the two colons of a
/// hotkey definition, and the `#` of a directive otherwise.
/// Whether the text of the line from this place to its end is blank.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the line that begins the
/// replacement of a hotstring is the one that is `(` and nothing else, and a
/// `(` that has text after it on the line is the text of the hotstring.
/// Whether the text of a hotstring of the v2 opens a block of code.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the `X` option executes
/// the replacement of the hotstring, and a replacement that ends with `{` is
/// a block that closes with a `}` on a line of its own (`:X:mf2::{`).
fn the_hotstring_of_the_v2_has_a_block(text: &str) -> bool {
    // Measured on the binary of v2.0.19 on 2026-11-10: the block of a
    // hotstring is a replacement that ends with `{` -- with the `X` option or
    // without it -- and a replacement that is empty whose next line begins
    // with `{`. A replacement that is empty and has no `{` is the failure
    // "Hotkey or hotstring is missing its opening brace.".
    let text = text.trim_end();
    text.ends_with('{') || text.rsplit("::").next().map(str::is_empty).unwrap_or(false)
}
/// Whether the block that the text of a hotstring opens really follows it.
///
/// Measured on the binary of v2.0.19 on 2026-11-10: the `X` hotstring whose
/// replacement ends with `{` carries its block on the lines that follow, and
/// the one whose replacement is empty carries the `{` on the line after it.
/// A replacement that is empty and has no `{` below is the failure "Hotkey or
/// hotstring is missing its opening brace.".
fn the_block_of_the_hotstring_follows(chars: &[char], text: &str, index: usize) -> bool {
    let text = text.trim_end();
    if text.ends_with('{') {
        return true;
    }
    let mut at = index;
    if at < chars.len() && chars[at] == '\r' {
        at += 1;
    }
    if at < chars.len() && chars[at] == '\n' {
        at += 1;
    }
    while at < chars.len() && (chars[at] == ' ' || chars[at] == '\t' || chars[at] == '\r') {
        at += 1;
    }
    at < chars.len() && chars[at] == '{'
}
fn the_rest_of_the_line_is_blank(chars: &[char], from: usize) -> bool {
    let mut at = from;
    while at < chars.len() && chars[at] != '\n' {
        if chars[at] != ' ' && chars[at] != '\t' && chars[at] != '\r' {
            return false;
        }
        at += 1;
    }
    true
}
fn the_line_from_has_the_two_colons(chars: &[char], from: usize) -> bool {
    let mut at = from;
    while at < chars.len() && chars[at] != '\n' {
        if chars[at] == ':' && chars.get(at + 1) == Some(&':') {
            return true;
        }
        at += 1;
    }
    false
}

/// The operators of the dialect.
fn the_operators_of(dialect: &'static dyn crate::dialect::Dialect) -> Vec<&'static str> {
    if dialect.the_lexer_knows_the_null_assignment() {
        // The v2.1 has the null assignment, and it is the longest match of the
        // operators that begin with the two question marks.
        let mut operators = OPERATORS.to_vec();
        let at = operators
            .iter()
            .position(|operator| *operator == "??")
            .unwrap_or(0);
        operators.insert(at, "??=");
        return operators;
    }
    if !dialect.the_lexer_knows_the_legacy_operators() {
        return OPERATORS.to_vec();
    }
    let mut operators = OPERATORS.to_vec();
    // The `<>` comes before the `<`, because the longest match wins.
    let at = operators
        .iter()
        .position(|operator| *operator == "<")
        .unwrap_or(0);
    operators.insert(at, "<>");
    operators
}

/// The script does not parse.
#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl core::fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "{} (line {}, column {})",
            self.message, self.line, self.column
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SyntaxError {}

impl SyntaxError {
    fn new(message: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

/// The directives of v2.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: these are the names
/// that the binary knows. A name that is not in the list is not a directive
/// at all: the line that holds it holds no action, and it is refused at the
/// load.
const THE_DIRECTIVES: [&str; 19] = [
    "clipboardtimeout",
    "dllload",
    "errorstdout",
    "hotif",
    "hotiftimeout",
    "hotstring",
    "include",
    "includeagain",
    "inputlevel",
    "maxthreads",
    "maxthreadsbuffer",
    "maxthreadsperhotkey",
    "notrayicon",
    "requires",
    "singleinstance",
    "suspendexempt",
    "usehook",
    "warn",
    "winactivateforce",
];

/// Whether the dialect knows the directive of this name.
///
/// The name is the one of the directive without the hash, and the case of
/// it does not matter.
pub fn carries_directive(name: &str) -> bool {
    THE_DIRECTIVES.contains(&name.to_lowercase().as_str())
}

/// Whether the character is a letter of a name.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the byte order mark is
/// a letter of a name, and a name may begin with it.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the "#", the "$" and
/// the "@" are letters of the name of a variable of the v1.
fn the_character_is_a_letter(character: char, marks: bool) -> bool {
    character.is_alphabetic()
        || character == '_'
        || character == '\u{feff}'
        || (marks && matches!(character, '#' | '$' | '@'))
}

/// Whether the character is a part of a name.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the byte order mark is a
/// part of a name, and the name that begins with it is a name.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the "#", the "$" and
/// the "@" are parts of the name of a variable of the v1.
fn the_character_is_of_a_name(character: char, marks: bool) -> bool {
    character.is_alphanumeric()
        || character == '_'
        || character == '\u{feff}'
        || (marks && matches!(character, '#' | '$' | '@'))
}

/// The name that the script wrote, with the escape expanded.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the name of
/// the v1 may carry the escape, and the escape is expanded before the name is
/// looked at: the `a`c` is the name "ac", and the `a`b` is the name
/// "a<backspace>", which is not a legal name of a variable.
///
/// The escape that carries a space ends the name, the escape that carries an
/// operator is the operator that comes after the name, and the escape that
/// carries nothing is dropped: the `x := a` is the name "a".
struct TheNameWithTheEscape {
    /// The name, with the escape expanded.
    name: String,
    /// The place where the name ends.
    end: usize,
    /// The operator that the escape carries, when there is one.
    operator: Option<String>,
    /// Whether the name carries an escape.
    has_an_escape: bool,
}

/// The name that begins at the place, with the escape expanded.
fn the_name_with_the_escape(
    chars: &[char],
    start: usize,
    marks: bool,
    dialect: &'static dyn crate::dialect::Dialect,
) -> TheNameWithTheEscape {
    let mut name = String::new();
    let mut index = start;
    let mut operator = None;
    let mut has_an_escape = false;
    while index < chars.len() {
        let character = chars[index];
        if the_character_is_of_a_name(character, marks) {
            name.push(character);
            index += 1;
            continue;
        }
        if character != '`' || !dialect.the_escape_may_be_in_a_name() {
            break;
        }
        has_an_escape = true;
        let Some(next) = chars.get(index + 1).copied() else {
            // The escape that carries nothing is dropped.
            index += 1;
            break;
        };
        index += 2;
        let expanded = dialect.the_escape_of_the_dialect(next);
        if expanded.is_whitespace() {
            // The escape that carries a space ends the name.
            break;
        }
        if expanded == ',' {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the escape that carries a comma is a literal: the name ends
            // there, and the comma does not separate the parameters of a
            // command: the `FileAppend, a`,b, *` is the text "a,b".
            break;
        }
        if OPERATORS.contains(&expanded.to_string().as_str()) {
            // The escape that carries an operator is the operator that comes
            // after the name: the `x := 1`+2` is the sum of the two.
            operator = Some(expanded.to_string());
            break;
        }
        name.push(expanded);
    }
    TheNameWithTheEscape {
        name,
        end: index,
        operator,
        has_an_escape,
    }
}

/// The name of a script, the way v2 writes it in a message.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the character that
/// cannot be written is a question mark.
fn the_name_of_a_message(name: &str) -> String {
    format!("{:?}", name.replace('\u{feff}', "?"))
}

/// The line of the script that holds the hash, from the hash.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the text that is named
/// is the line from the hash, and the space at the end of it is not a part
/// of it.
fn the_line_of_the_hash(chars: &[char], start: usize) -> String {
    let mut end = start;
    while end < chars.len() && chars[end] != '\n' {
        end += 1;
    }
    chars[start..end]
        .iter()
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// The refusal of the type or of the mode of the `#Warn`.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the text that
/// the refusal of the type names is the source of the line up to the comma --
/// "#Warn Nope" -- with the case that the script wrote, and the text that the
/// refusal of the mode names is the text that comes after the comma --
/// "StdOut, Extra". A type that is empty is not a type, and the text that
/// comes after the comma is the mode then: the "#Warn ,Nope" is a refusal of
/// the mode.
fn the_refusal_of_the_warn(
    dialect: &dyn crate::dialect::Dialect,
    chars: &[char],
    hash_at: usize,
    argument: &str,
    line: u32,
    column: u32,
) -> Result<(), SyntaxError> {
    let (kind, mode) = crate::warnings::the_parts_of_the_argument(argument);
    if !kind.is_empty() && dialect.the_type_of_the_warning(kind).is_none() {
        let source = the_line_of_the_hash(chars, hash_at);
        let text = source
            .split(',')
            .next()
            .unwrap_or(source.as_str())
            .trim_end();
        return Err(SyntaxError::new(
            format!("Parameter #1 invalid.\n     Specifically: {text}"),
            line,
            column,
        ));
    }
    if !mode.is_empty() && dialect.the_mode_of_the_warning(mode).is_none() {
        return Err(SyntaxError::new(
            format!("Parameter #2 invalid.\n     Specifically: {mode}"),
            line,
            column,
        ));
    }
    Ok(())
}

/// The source of a part of the script, as it was written.
///
/// The text of the dialect is the line as the script wrote it, and the
/// lexer is the one that has it: the parser rebuilds it from the tokens,
/// and a text that is not closed has no token.
fn the_source_of_the_chars(chars: &[char], start: usize, end: usize) -> String {
    let start = start.min(end).min(chars.len());
    let end = end.min(chars.len());
    let mut text: String = chars[start..end].iter().collect();
    while text.ends_with([' ', '\t', '\r']) {
        text.pop();
    }
    text
}

/// The options of a continuation section.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the options are on the
/// line of the opening parenthesis, they are separated by spaces, and the
/// option that v2 does not know is refused at the load.
struct TheOptionsOfASection {
    /// The string that joins the lines of the section.
    ///
    /// `None` is the default: every line but the last is followed by a
    /// linefeed. `Some` with an empty string is the option `Join` and no
    /// string of its own.
    join: Option<String>,
    /// The leading whitespace of every line is removed.
    ltrim: bool,
    /// The removal of the leading whitespace that follows the indentation of
    /// the first line is off.
    ltrim0: bool,
    /// The trailing whitespace of every line is kept.
    rtrim0: bool,
    /// The comments of the lines are removed.
    comments: bool,
    /// The backtick is a literal.
    accent: bool,
}

/// A continuation section: the lines between the parentheses.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the section begins with a
/// parenthesis at the start of a line and ends with a parenthesis at the
/// start of a line, and the text that follows the closing parenthesis is
/// joined with the lines of the section.
struct TheSection {
    options: TheOptionsOfASection,
    /// The lines of the section, without the line terminators.
    lines: Vec<String>,
    /// The text that follows the closing parenthesis.
    rest: String,
    /// The index of the source after the closing parenthesis.
    end: usize,
    /// The index of the source of the text that follows the closing
    /// parenthesis.
    rest_at: usize,
    /// The index of the source after the line of the opening parenthesis.
    after: usize,
    /// Whether the section was closed.
    closed: bool,
}

/// The section that opens at the line that begins at `start`, if there is one.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the section begins with a
/// parenthesis at the start of a line. The line is an expression and not a
/// section when it carries a parenthesis of its own after the opening one --
/// the string of the option `Join` is the only one that may carry one.
fn the_section_at(
    chars: &[char],
    start: usize,
    line: u32,
    dialect: &'static dyn crate::dialect::Dialect,
) -> Result<Option<TheSection>, SyntaxError> {
    let mut index = start;
    while matches!(chars.get(index), Some(' ') | Some('\t')) {
        index += 1;
    }
    if chars.get(index) != Some(&'(') {
        return Ok(None);
    }
    let column = (index - start + 1) as u32;
    index += 1;
    // The end of the line of the opening parenthesis.
    let mut end_of_the_line = index;
    while end_of_the_line < chars.len() && chars[end_of_the_line] != '\n' {
        end_of_the_line += 1;
    }
    let mut text_of_the_options: String = chars[index..end_of_the_line].iter().collect();
    // The comment of the line ends the options.
    if let Some(at) = text_of_the_options.find(';') {
        text_of_the_options.truncate(at);
    }
    // The options of the section.
    let mut options = TheOptionsOfASection {
        join: None,
        ltrim: false,
        ltrim0: false,
        rtrim0: false,
        comments: false,
        accent: false,
    };
    let mut the_join = String::new();
    let mut the_join_is_there = false;
    let mut an_option_is_not_known: Option<String> = None;
    let mut a_paren_is_in_the_line = false;
    for option in text_of_the_options.split(' ') {
        if option.is_empty() {
            continue;
        }
        // The option `Join` takes the string that follows its name.
        let (name, argument) = match option.get(..4) {
            Some(head) if head.eq_ignore_ascii_case("join") => ("join", Some(&option[4..])),
            _ => (option, None),
        };
        // The parenthesis of the line: the line is an expression and not a
        // section when it carries one.
        let text = argument.unwrap_or(option);
        if text.contains('(') || text.contains(')') {
            a_paren_is_in_the_line = true;
        }
        match name.to_lowercase().as_str() {
            "join" => {
                the_join_is_there = true;
                the_join = argument.unwrap_or("").to_string();
            }
            "ltrim" => options.ltrim = true,
            "ltrim0" => options.ltrim0 = true,
            "rtrim0" => options.rtrim0 = true,
            "comments" | "comment" | "com" | "c" => options.comments = true,
            "`" => options.accent = true,
            _ => {
                if an_option_is_not_known.is_none() {
                    an_option_is_not_known = Some(option.to_string());
                }
            }
        }
    }
    if a_paren_is_in_the_line {
        // The line is an expression: `(x.y)()` and the like.
        return Ok(None);
    }
    if let Some(option) = an_option_is_not_known {
        // Measured on the binary of v2.0.19 on 2026-10-31: the option that
        // v2 does not know is named, and the script does not load.
        return Err(SyntaxError::new(
            format!("Invalid option.\n     Specifically: {option}"),
            line,
            column,
        ));
    }
    if the_join_is_there {
        // Measured on the binary of v2.0.19 on 2026-11-09: the string of the
        // option `Join` is at most fifteen characters long, and the rest of
        // it is not a part of the string. `(Join0123456789abcdefGHI` joins
        // with "0123456789abcde".
        let the_join: String = the_join.chars().take(15).collect();
        options.join = Some(the_escapes_of_a_text(&the_join, options.accent, dialect));
    }
    // The lines of the section, until the line that closes it.
    let mut lines: Vec<String> = Vec::new();
    let mut rest = String::new();
    let mut rest_at = end_of_the_line;
    let mut end = chars.len();
    let mut closed = false;
    let mut cursor = end_of_the_line;
    while cursor < chars.len() {
        let mut start_of_the_line = cursor + 1;
        if chars.get(start_of_the_line) == Some(&'\r') {
            start_of_the_line += 1;
        }
        let mut end_of_the_line = start_of_the_line;
        while end_of_the_line < chars.len() && chars[end_of_the_line] != '\n' {
            end_of_the_line += 1;
        }
        let text: String = chars[start_of_the_line..end_of_the_line].iter().collect();
        let trimmed = text.trim_start_matches([' ', '\t']);
        if let Some(after_the_parenthesis) = trimmed.strip_prefix(')') {
            // The section is closed here, and the text that follows the
            // closing parenthesis is joined with the lines of the section.
            rest = after_the_parenthesis.to_string();
            rest_at = start_of_the_line + (trimmed.len() - after_the_parenthesis.len());
            end = end_of_the_line;
            closed = true;
            break;
        }
        lines.push(text);
        cursor = end_of_the_line;
    }
    Ok(Some(TheSection {
        options,
        lines,
        rest,
        end,
        rest_at,
        after: end_of_the_line + 1,
        closed,
    }))
}

/// The text of the lines of a section.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the lines are joined with
/// the string of the option `Join` (a linefeed when there is none), the
/// trailing whitespace is removed, and the leading whitespace is removed as
/// the indentation of the first content line says -- unless the option
/// `LTrim` takes it all away or the option `LTrim0` keeps it. A line that
/// carries only a comment is not a line of the section at all: it
/// contributes nothing, not even the linefeed before it.
fn the_content_of_the_section(section: &TheSection) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in &section.lines {
        let mut text = line.clone();
        let mut a_comment_is_the_whole_line = false;
        if section.options.comments {
            // The comment of the line, and the whitespace before it, are of
            // no account.
            if let Some(at) = text.find(';') {
                text.truncate(at);
                while text.ends_with([' ', '\t']) {
                    text.pop();
                }
                a_comment_is_the_whole_line = text.trim().is_empty();
            }
        }
        if a_comment_is_the_whole_line {
            continue;
        }
        if !section.options.rtrim0 {
            while text.ends_with([' ', '\t', '\r']) {
                text.pop();
            }
        }
        lines.push(text);
    }
    // The indentation of the first line: the spaces or the tabs that come
    // first, and only that kind of them.
    let mut indentation = (0usize, ' ');
    if !section.options.ltrim && !section.options.ltrim0 {
        let first = lines.first().map(|line| line.as_str()).unwrap_or("");
        let spaces = first.chars().take_while(|c| *c == ' ').count();
        let tabs = first.chars().take_while(|c| *c == '\t').count();
        indentation = if spaces > 0 {
            (spaces, ' ')
        } else {
            (tabs, '\t')
        };
    }
    if indentation.0 > 0 {
        let (count, character) = indentation;
        for line in lines.iter_mut() {
            // A line that is indented less than the first one, or with the
            // wrong characters, is left as it is.
            if line.chars().take(count).all(|c| c == character) {
                *line = line.chars().skip(count).collect();
            }
        }
    }
    if section.options.ltrim {
        for line in lines.iter_mut() {
            *line = line.trim_start_matches([' ', '\t']).to_string();
        }
    }
    // The lines are joined with the string of the option `Join`.
    let join = match &section.options.join {
        Some(join) => join.clone(),
        None => "\n".to_string(),
    };
    let mut content = String::new();
    for (number, line) in lines.iter().enumerate() {
        if number > 0 {
            content.push_str(&join);
        }
        content.push_str(line);
    }
    content
}

/// The escapes of a text.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the backtick and the
/// character that follows are one escape, and the accent of a section turns
/// the backtick into a literal.
fn the_escapes_of_a_text(
    text: &str,
    accent: bool,
    dialect: &'static dyn crate::dialect::Dialect,
) -> String {
    if accent {
        return text.to_string();
    }
    let mut result = String::new();
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character == '`' {
            match characters.next() {
                Some(next) => result.push(dialect.the_escape_of_the_dialect(next)),
                None => result.push('`'),
            }
            continue;
        }
        result.push(character);
    }
    result
}

/// The tokens of a line that a continuation section is merged with.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the newlines of the
/// section are spaces in the merged line, a string of it may run from one
/// of its lines to the next, and the tokens of the whole line are the ones
/// of the text that was merged -- so an operator may be split across the
/// boundary.
fn the_tokens_of_a_merged_line(
    text: &str,
    dialect: &'static dyn crate::dialect::Dialect,
    line: u32,
) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = tokenize_the_source(text, dialect, true).map_err(|mut error| {
        error.line = line;
        error
    })?;
    // The end of the text is not a token of the script.
    if tokens
        .last()
        .map(|token| token.kind == TokenKind::Eof)
        .unwrap_or(false)
    {
        tokens.pop();
    }
    for token in tokens.iter_mut() {
        token.line = line;
    }
    Ok(tokens)
}

/// The section that opens at the line that comes after `from`, when the
/// blank lines and the comments between are of no account.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the section joins the
/// last line that has code, and the blank lines and the comment lines
/// between the two are not a part of it. The number of the lines that were
/// skipped is answered with the section: the lines of the script that the
/// section moved are the ones of the count.
fn the_section_after_the_line(
    chars: &[char],
    from: usize,
    line: u32,
    dialect: &'static dyn crate::dialect::Dialect,
) -> Result<Option<(TheSection, u32)>, SyntaxError> {
    let mut at = from;
    let mut skipped = 0u32;
    loop {
        let mut end_of_the_line = at;
        while end_of_the_line < chars.len() && chars[end_of_the_line] != '\n' {
            end_of_the_line += 1;
        }
        let text: String = chars[at..end_of_the_line].iter().collect();
        let trimmed = text.trim_matches([' ', '\t', '\r']);
        if trimmed.is_empty() || trimmed.starts_with(';') {
            if end_of_the_line >= chars.len() {
                return Ok(None);
            }
            skipped += 1;
            at = end_of_the_line + 1;
            continue;
        }
        let section = the_section_at(chars, at, line + skipped, dialect)?;
        return Ok(section.map(|section| (section, skipped)));
    }
}

/// The text of a line without the comment that ends it.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: the comment of the line
/// that a section joins is of no account, and it is taken away with the
/// whitespace to its left. The comment begins at a semicolon that is at the
/// start of the line or that has a whitespace before it -- the quote that
/// the line carries does not protect the semicolon, which is why the text of
/// the line `Var := "a ; b"` is `Var := "a` when a section joins it.
fn the_text_without_the_comment(text: &str) -> &str {
    let mut previous = None;
    for (at, character) in text.char_indices() {
        if character == ';' && (at == 0 || matches!(previous, Some(' ') | Some('\t'))) {
            let mut end = at;
            while end > 0 && matches!(text.as_bytes().get(end - 1), Some(b' ') | Some(b'\t')) {
                end -= 1;
            }
            return &text[..end];
        }
        previous = Some(character);
    }
    text.trim_end_matches([' ', '\t', '\r'])
}

/// Whether the last character of a text is a name.
///
/// A name is a letter, a digit or an underscore: the text of a string ends
/// with a quote, and the quote is not a name.
fn the_last_character_is_a_name(text: &str) -> bool {
    match text.trim_end().chars().last() {
        Some(last) => the_character_is_of_a_name(last, false),
        None => false,
    }
}

/// Whether the first character of a text is a name.
fn the_first_character_is_a_name(text: &str) -> bool {
    match text.chars().next() {
        Some(first) => the_character_is_of_a_name(first, false),
        None => false,
    }
}

/// Whether the text of a line ends inside a quoted text.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: a line that ends inside
/// a quoted text takes the section as a part of that text, and the quotes
/// that the line carries are read again the way the text is read -- a quote
/// that is not closed is not the end of the text.
fn the_line_above_ends_inside_a_text(text: &str) -> bool {
    let mut quote: Option<char> = None;
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character == '`' {
            // The escape of the dialect is the backtick: the character that
            // follows it is a part of the text and not a quote of it.
            characters.next();
            continue;
        }
        match quote {
            Some(open) => {
                if character == open {
                    quote = None;
                }
            }
            None => {
                if character == '"' || character == '\'' {
                    quote = Some(character);
                }
            }
        }
    }
    quote.is_some()
}

/// Turn the source into tokens.
///
/// A newline ends a statement. Inside a block the statements are separated
/// by newlines too, so the parser reads a block as a list of statements and
/// the newline is just a separator there as well.
pub fn tokenize(source: &str) -> Result<Vec<Token>, SyntaxError> {
    tokenize_with(source, crate::dialect::v2())
}

/// Turn the source into tokens, in the dialect of this name.
///
/// The directives are the ones of the dialect: a name that the dialect
/// does not know is not a directive at all, and the line that holds it
/// holds no action.
pub fn tokenize_with(
    source: &str,
    dialect: &'static dyn crate::dialect::Dialect,
) -> Result<Vec<Token>, SyntaxError> {
    tokenize_the_source(source, dialect, false)
}

/// The tokens of a source, with the newlines being spaces when the flag is
/// set.
///
/// The text of a continuation section is merged with the line above it and
/// read again as one line: the newlines of the section are spaces in that
/// line, and a string of it may run from one of its lines to the next. The
/// flag is what tells the two readings apart, and the reading of a whole
/// script never sets it.
fn tokenize_the_source(
    source: &str,
    dialect: &'static dyn crate::dialect::Dialect,
    the_newlines_join_the_line: bool,
) -> Result<Vec<Token>, SyntaxError> {
    // Measured on the binary of v2.0.19 on 2026-10-31: the script may begin
    // with the byte order mark, and the mark is not a character of it. Only
    // the first one is taken: a second one is a character of the script.
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let chars: Vec<char> = source.chars().collect();
    // Measured on the binary of v1.1.37.02 on 2026-11-02: the name of a
    // variable of the v1 may carry the "#", the "$" and the "@", and the v2
    // does not read them.
    let marks = dialect.accepts_a_hash_a_dollar_and_an_at_in_a_name();
    // The operators of the dialect: the v1 knows the `<>`, and the v2 does
    // not.
    let operators = the_operators_of(dialect);
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0usize;
    let mut line = 1u32;
    let mut column = 1u32;

    // The first token has no space before it.
    let mut spaced = false;
    // The place where every line begins: the section that follows is joined
    // with the line that has code above it.
    let mut the_starts_of_the_lines: Vec<usize> = vec![0];
    // The line that has code, with its text as merged so far: a
    // continuation section joins it, and the text is the one that is read
    // again. The line above a section may be several lines above it -- the
    // blank lines and the comments between are of no account -- so the last
    // line that has code is the one that is kept, and not the line that
    // happens to come before the section.
    let mut the_line_with_code: Option<(u32, String)> = None;
    // Where the tokens of that line begin, and where the physical line that
    // is being read begins.
    let mut the_token_start_of_the_line_with_code = 0usize;
    let mut the_start_of_the_current_line = 0usize;
    let mut the_start_of_the_current_line_tokens = 0usize;
    // Whether a section was merged into the current line: the line is the
    // one of the section, and its text was merged and read again.
    let mut a_section_was_merged = false;
    // Whether a hotstring of this script reads its replacement as code.
    // Measured on the binary of v2.0.19 on 2026-11-10: `#Hotstring X` turns it
    // on for the hotstrings that follow and `#Hotstring X0` turns it off, and
    // the block of a hotstring -- a replacement that ends with `{` -- is a
    // block only while it is on, which is the default.
    let mut the_hotstrings_read_code = true;
    while index < chars.len() {
        let current = chars[index];
        // The space belongs to the token that comes next, and to that one
        // only.
        let before = spaced;
        spaced = false;
        if current == '\n' {
            // The text of a line that a section joined is one line: its
            // newlines are spaces, and the newline does not end a
            // statement.
            if the_newlines_join_the_line {
                index += 1;
                line += 1;
                column = 1;
                spaced = true;
                continue;
            }
            // The line that ends here is the line that a section joins when
            // it has code: the text is kept before the section is looked
            // for.
            if tokens.len() > the_start_of_the_current_line_tokens && !a_section_was_merged {
                let mut text: String = chars[the_start_of_the_current_line..index].iter().collect();
                while text.ends_with(['\r', ' ', '\t']) {
                    text.pop();
                }
                the_line_with_code = Some((line, text));
                the_token_start_of_the_line_with_code = the_start_of_the_current_line_tokens;
            }
            // The line that comes may open a continuation section: the
            // section continues the line above, and the newline is not a
            // separator of statements.
            if let Some((section, the_skipped_lines)) =
                the_section_after_the_line(&chars, index + 1, line + 1, dialect)?
            {
                let the_line_above = the_line_with_code.clone();
                if let Some((the_number, text_of_the_line)) = &the_line_above {
                    let the_number = *the_number;
                    // A directive takes the rest of its line: the section
                    // is a part of its argument.
                    let the_directive_is_the_line_above = tokens
                        .iter()
                        .skip(the_token_start_of_the_line_with_code)
                        .any(|token| token.kind == TokenKind::Directive);
                    if the_directive_is_the_line_above {
                        let content = the_content_of_the_section(&section);
                        let mut argument = String::new();
                        if tokens
                            .last()
                            .map(|token| {
                                token.kind == TokenKind::String && token.line == the_number
                            })
                            .unwrap_or(false)
                        {
                            argument = tokens.pop().map(|token| token.value).unwrap_or_default();
                        }
                        argument.push_str(&content);
                        argument.push_str(&section.rest);
                        tokens.push(Token {
                            kind: TokenKind::String,
                            value: argument,
                            line: the_number,
                            column: 1,
                            spaced: false,
                            quote: None,
                        });
                        index = section.end;
                        line += the_skipped_lines + section.lines.len() as u32 + 2;
                        column = 1;
                        a_section_was_merged = true;
                        continue;
                    }
                    // The text of the section is merged with the line above
                    // as text, and the line is read again: measured on the
                    // binary of v2.0.19 on 2026-11-09, the merge is textual,
                    // so an operator may be split across the boundary and
                    // the `Join` string of a section may be part of a name.
                    let text_of_the_line = the_text_without_the_comment(text_of_the_line).to_string();
                    let content = the_content_of_the_section(&section);
                    let mut merged = text_of_the_line;
                    // The space that keeps two names apart is a thing of the
                    // text of an expression: the text of a line that ends
                    // inside a quoted text is the text of that text, and the
                    // section is a part of it. Measured on the binary of
                    // v2.0.19 on 2026-11-09: the text of the line
                    // `Var := "a ; b"` is `Var := "a`, and the section `c`
                    // makes it `"ac"`.
                    let a_space = !content.is_empty()
                        && !the_line_above_ends_inside_a_text(&merged)
                        && the_last_character_is_a_name(&merged)
                        && the_first_character_is_a_name(&content);
                    if a_space {
                        merged.push(' ');
                    }
                    if !section.closed {
                        // Measured on the binary of v2.0.19 on 2026-11-09:
                        // the section that is not closed is named, and the
                        // text of the line above and of the section is named
                        // with it.
                        let mut text = merged;
                        let rest: String = chars[section.after..].iter().collect();
                        text.push_str(rest.trim_end_matches(['\r', '\n']));
                        return Err(SyntaxError::new(
                            format!("Missing \")\"\n     Specifically: {text}"),
                            the_number,
                            1,
                        ));
                    }
                    merged.push_str(&content);
                    merged.push_str(&section.rest);
                    tokens.truncate(the_token_start_of_the_line_with_code);
                    let the_merged_tokens =
                        the_tokens_of_a_merged_line(&merged, dialect, the_number)?;
                    tokens.extend(the_merged_tokens);
                    the_line_with_code = Some((the_number, merged));
                    the_start_of_the_current_line_tokens = the_token_start_of_the_line_with_code;
                    a_section_was_merged = true;
                    index = section.end;
                    line += the_skipped_lines + section.lines.len() as u32 + 2;
                    column = 1;
                    continue;
                }
            }
            tokens.push(Token {
                kind: TokenKind::Newline,
                value: "\n".into(),
                line,
                column,
                spaced: before,
                quote: None,
            });
            index += 1;
            line += 1;
            column = 1;
            the_starts_of_the_lines.push(index);
            the_start_of_the_current_line = index;
            the_start_of_the_current_line_tokens = tokens.len();
            a_section_was_merged = false;
            continue;
        }
        if current == ' ' || current == '\t' || current == '\r' {
            index += 1;
            column += 1;
            spaced = true;
            continue;
        }
        if current == '#' {
            // Measured on the binary of v2.0.19 on 2026-11-09: a `#` that
            // begins a line of a hotkey is the `#` of the Win modifier and
            // not the `#` of a directive -- `#z::MsgBox "x"` is a hotkey --
            // and the two colons before the end of the line are what says so.
            if tokens.len() == the_start_of_the_current_line_tokens
                && the_line_from_has_the_two_colons(&chars, index)
            {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: "#".into(),
                    line,
                    column,
                    spaced: before,
                    quote: None,
                });
                index += 1;
                column += 1;
                continue;
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the hash is a
            // directive only at the start of a line. Anywhere else it is an
            // illegal character in an expression, and the parser is the one
            // that says so.
            if tokens.iter().any(|token| token.line == line) {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: "#".into(),
                    line,
                    column,
                    spaced: before,
                    quote: None,
                });
                index += 1;
                column += 1;
                continue;
            }
            // A directive: the name, and the rest of the line is its
            // argument. The argument is free text, so the lexer reads it
            // as it stands and the runtime says what it means.
            let (start_line, start_column) = (line, column);
            let hash_at = index;
            index += 1;
            column += 1;
            let mut name = String::new();
            while index < chars.len() && the_character_is_of_a_name(chars[index], marks) {
                name.push(chars[index]);
                index += 1;
                column += 1;
            }
            if name.is_empty() || name.starts_with(|character: char| character.is_ascii_digit()) {
                // Measured on the binary of v2.0.19 on 2026-10-31: a hash that
                // has no name, or a name that begins with a number, is not a
                // directive: the line holds no action, and it is named.
                return Err(SyntaxError::new(
                    format!(
                        "This line does not contain a recognized action.\n     Specifically: {}",
                        the_line_of_the_hash(&chars, hash_at)
                    ),
                    start_line,
                    start_column,
                ));
            }
            let mut argument = String::new();
            while index < chars.len() && chars[index] != '\n' {
                argument.push(chars[index]);
                index += 1;
                column += 1;
            }
            // Measured on the binary of v2.0.19 on 2026-11-10: `#Hotstring`
            // sets the options of the hotstrings that follow it. The options
            // that change how the text of a hotstring is read are the `X`
            // (the replacement is code) and its `X0` (it is text), and the
            // directive with no argument goes back to the default.
            if name.eq_ignore_ascii_case("hotstring") {
                let argument = argument.trim();
                if argument.is_empty() {
                    the_hotstrings_read_code = true;
                } else {
                    for option in argument.split_whitespace() {
                        if option.eq_ignore_ascii_case("x") {
                            the_hotstrings_read_code = true;
                        } else if option.eq_ignore_ascii_case("x0") {
                            the_hotstrings_read_code = false;
                        }
                    }
                }
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: a name that
            // the v2 does not know is not a directive at all: the line holds
            // no action, and it is named with it.
            if !dialect.carries_directive(&name) {
                return Err(SyntaxError::new(
                    format!(
                        "This line does not contain a recognized action.\n     Specifically: #{name}{}",
                        argument.trim_end()
                    ),
                    start_line,
                    start_column,
                ));
            }
            // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
            // `#Warn` takes the type of the warning and the mode of it. The
            // type that is not one of them is refused at the load with
            // "Parameter #1 invalid." and the text of the directive, and the
            // mode that is not one of them with "Parameter #2 invalid." and
            // the text that was written.
            if name.eq_ignore_ascii_case("warn") {
                the_refusal_of_the_warn(
                    dialect,
                    &chars,
                    hash_at,
                    &argument,
                    start_line,
                    start_column,
                )?;
            }
            tokens.push(Token {
                kind: TokenKind::Directive,
                value: name.to_lowercase(),
                line: start_line,
                column: start_column,
                spaced: before,
                quote: None,
            });
            // The argument rides along in the token, so it has to be
            // kept somewhere: the second token of the line carries it.
            let argument = argument.trim().to_string();
            if !argument.is_empty() {
                tokens.push(Token {
                    kind: TokenKind::String,
                    value: argument,
                    line: start_line,
                    column: start_column,
                    spaced: before,
                    quote: None,
                });
            }
            continue;
        }
        if current == '/' && chars.get(index + 1) == Some(&'*') {
            // Measured on the binary of v2.0.19 on 2026-11-09: a block
            // comment begins with `/*` and ends with the first `*/`, and
            // the newlines inside it are lines of the script. A section may
            // follow a block comment, and the comment is of no account.
            index += 2;
            column += 2;
            while index < chars.len() {
                if chars[index] == '*' && chars.get(index + 1) == Some(&'/') {
                    index += 2;
                    column += 2;
                    break;
                }
                if chars[index] == '\n' {
                    index += 1;
                    line += 1;
                    column = 1;
                    the_starts_of_the_lines.push(index);
                    continue;
                }
                index += 1;
                column += 1;
            }
            // The comment separates the tokens around it, the way a space
            // does.
            spaced = true;
            continue;
        }
        if current == ';' {
            // Measured on the binary of v2.0.19 on 2026-11-01: the
            // semicolon is the beginning of a comment when it is at the
            // start of a line or when a space comes before it. The one that
            // is glued to the token that comes before it is an illegal
            // character in an expression, and the parser is the one that
            // says so.
            if before || !tokens.iter().any(|token| token.line == line) {
                while index < chars.len() && chars[index] != '\n' {
                    index += 1;
                }
                continue;
            }
            tokens.push(Token {
                kind: TokenKind::Op,
                value: ";".into(),
                line,
                column,
                spaced: before,
                quote: None,
            });
            index += 1;
            column += 1;
            continue;
        }
        if current == '\'' && dialect.the_single_quote_is_not_a_text() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the single quote is not a delimiter of a text in the v1: it
            // begins a symbol, and the symbol is refused where an expression
            // wants an operand.
            let (start_line, start_column) = (line, column);
            let mut end = index;
            while end < chars.len() && !the_character_ends_the_symbol_of_the_v1(chars[end]) {
                end += 1;
            }
            let symbol: String = chars[index..end].iter().collect();
            column += (end - index) as u32;
            index = end;
            tokens.push(Token {
                kind: TokenKind::Name,
                value: symbol,
                line: start_line,
                column: start_column,
                spaced: before,
                quote: None,
            });
            continue;
        }
        if current == '"' || current == '\'' {
            let (start_line, start_column) = (line, column);
            // Measured on the binary of v2.0.19 on 2026-10-31: a text is
            // between double quotes or between single quotes, and the
            // escape of the dialect is the backtick in the two of them.
            let quote = current;
            let quote_index = index;
            index += 1;
            column += 1;
            let mut buffer = String::new();
            // The text of the section that the text holds, if there is one.
            let mut the_text_of_the_section = String::new();
            loop {
                while index < chars.len() && chars[index] != '\n' {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the quote of the v1 escapes itself: the
                    // `"a""b"` is the text `a"b`.
                    if chars[index] == '"'
                        && dialect.the_quote_escapes_the_quote()
                        && index + 1 < chars.len()
                        && chars[index + 1] == '"'
                    {
                        buffer.push('"');
                        index += 2;
                        column += 2;
                        continue;
                    }
                    if chars[index] == quote {
                        break;
                    }
                    // The escape of v2 is the backtick. A backslash is a
                    // backslash: the path of a file is written with it, and
                    // v2 does not read it as an escape.
                    if chars[index] == '`' && index + 1 < chars.len() {
                        let next = chars[index + 1];
                        if next == quote && !dialect.the_escape_protects_the_quote_of_a_text() {
                            // Measured on the binary of AutoHotkey v1.1.37.02
                            // on 2026-11-04: the escape of the v1 does not
                            // protect the quote of the text: the `x := "a`"b"`
                            // is the text "a" and the name `b"`. The backtick
                            // is not a part of the text, and the quote is the
                            // one that closes it.
                            index += 1;
                            column += 1;
                            break;
                        }
                        buffer.push(dialect.the_escape_of_the_dialect(next));
                        index += 2;
                        column += 2;
                        continue;
                    }
                    buffer.push(chars[index]);
                    index += 1;
                    column += 1;
                }
                if index >= chars.len() || chars[index] != '\n' {
                    break;
                }
                // The newlines of a merged line are spaces, and a string of
                // it may run from one line of the section to the next: the
                // newline is a character of the text.
                if the_newlines_join_the_line {
                    buffer.push('\n');
                    index += 1;
                    line += 1;
                    column = 1;
                    continue;
                }
                // The line that comes may open a continuation section: the
                // section is inside the text, and its lines are text. The
                // blank lines and the comments between the text and the
                // section are of no account.
                let Some((section, the_skipped_lines)) =
                    the_section_after_the_line(&chars, index + 1, line + 1, dialect)?
                else {
                    break;
                };
                if !section.closed {
                    // The section that is not closed is named, and the text
                    // it merged with the line above is named with it.
                    let start_of_the_line = quote_index - (start_column as usize - 1);
                    let mut text: String = chars[start_of_the_line..index].iter().collect();
                    let rest: String = chars[section.after..].iter().collect();
                    text.push_str(rest.trim_end_matches(['\r', '\n']));
                    return Err(SyntaxError::new(
                        format!("Missing \")\"\n     Specifically: {text}"),
                        start_line,
                        start_column,
                    ));
                }
                // The comment of the line that the section joins is not a
                // part of the text: measured on the binary of v2.0.19 on
                // 2026-11-09, the `;` of that line is stripped with the
                // whitespace to its left, even when the text is not closed.
                if the_text_of_the_section.is_empty() {
                    let without_the_comment = the_text_without_the_comment(&buffer).to_string();
                    buffer = without_the_comment;
                }
                // The lines of the section are text: the quotes in them are
                // literal and the newlines are kept.
                let content = the_content_of_the_section(&section);
                the_text_of_the_section = content.clone();
                buffer.push_str(&the_escapes_of_a_text(
                    &content,
                    section.options.accent,
                    dialect,
                ));
                // The text that follows the closing parenthesis is the text
                // that is being read: the quote that closes it is there.
                let section_start = index + 1;
                index = section.rest_at;
                // The lines of the section are lines of the script too, and
                // the place where each of them begins is kept: the section
                // that comes after them joins the line that has code above
                // it, and that line is found by its place. The section of a
                // string that is not closed is the other place where a
                // section is read, and it counts its lines the same way.
                let mut cursor = section_start;
                while cursor < section.end {
                    the_starts_of_the_lines.push(cursor);
                    match chars[cursor..section.end].iter().position(|c| *c == '\n') {
                        Some(at) => cursor += at + 1,
                        None => break,
                    }
                }
                line += the_skipped_lines + section.lines.len() as u32 + 2;
                column = 1;
            }
            if index >= chars.len() || chars[index] == '\n' {
                // Measured on the binary of v2.0.19 on 2026-10-31: the quote
                // that is not closed is named, and the line of the script
                // says where it was.
                let start = quote_index - (start_column as usize - 1);
                let mut end = index;
                while end < chars.len() && chars[end] != '\n' {
                    end += 1;
                }
                // The text that was read is the text of the line, and the
                // lines of the section that it holds.
                let mut text = the_source_of_the_chars(&chars, start, end);
                if !the_text_of_the_section.is_empty() {
                    // The line of the text is the line where the text began,
                    // and the lines of the section are the text that follows.
                    let mut end_of_the_line = start;
                    while end_of_the_line < chars.len() && chars[end_of_the_line] != '\n' {
                        end_of_the_line += 1;
                    }
                    text = the_source_of_the_chars(&chars, start, end_of_the_line);
                    text.push_str(&the_text_of_the_section);
                }
                if dialect.the_text_that_is_not_closed_is_named_alone() {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the v1 says "Missing close-quote" and names
                    // the text that was read: the `"abc`.
                    let mut end = index;
                    while end < chars.len() && chars[end] != '\n' {
                        end += 1;
                    }
                    let text = the_source_of_the_chars(&chars, quote_index, end);
                    if dialect.the_quote_that_is_not_closed_is_an_error() {
                        return Err(SyntaxError::new(
                            format!("Missing close-quote\n     Specifically: {text}"),
                            start_line,
                            start_column,
                        ));
                    }
                    // The v1 has no texts in the line of a command, and the
                    // quote that is not closed there is not an error: the
                    // parser refuses the text when it reads it as an
                    // expression.
                    tokens.push(Token {
                        kind: TokenKind::UnclosedText,
                        value: text,
                        line: start_line,
                        column: start_column,
                        spaced: before,
                        quote: Some(quote),
                    });
                    continue;
                }
                return Err(SyntaxError::new(
                    format!("Missing \"{quote}\"\n     Specifically: {text}"),
                    start_line,
                    start_column,
                ));
            }
            index += 1;
            column += 1;
            tokens.push(Token {
                kind: TokenKind::String,
                value: buffer,
                line: start_line,
                column: start_column,
                spaced: before,
                quote: Some(quote),
            });
            continue;
        }
        // The numbers of v2 may begin with a point: `.5` is the number
        // 0.5. The point of a member comes after a value, and it is not
        // the point of a number.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31.
        if current.is_ascii_digit()
            || (current == '.' && the_point_begins_a_number(&chars, index, &tokens))
        {
            let (start_line, start_column) = (line, column);
            let start = index;
            index =
                the_end_of_the_number(&chars, index, dialect.the_exponent_needs_a_point(), marks);
            column += (index - start) as u32;
            // A name that begins with a number is not a number: v2 refuses
            // the script at the load, and it says which name it is.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the number of the v1 may carry the escape: the `5`b` is the
            // name "5<backspace>", which is not a legal name of a variable.
            if index < chars.len()
                && (the_character_is_a_letter(chars[index], marks)
                    || (chars[index] == '`' && dialect.the_escape_may_be_in_a_name()))
            {
                let number_end = index;
                let mut name = chars[start..index].iter().collect::<String>();
                if chars[index] == '`' {
                    let escape = the_name_with_the_escape(&chars, index, marks, dialect);
                    if let Some(operator) = escape.operator {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-04: the escape that carries an operator
                        // after a number is the operator: the `x := 1`+2` is
                        // the sum of the two.
                        column += (escape.end - number_end) as u32;
                        index = escape.end;
                        tokens.push(Token {
                            kind: TokenKind::Number,
                            value: chars[start..number_end].iter().collect(),
                            quote: None,
                            line: start_line,
                            column: start_column,
                            spaced: before,
                        });
                        tokens.push(Token {
                            kind: TokenKind::Op,
                            value: operator,
                            quote: None,
                            line: start_line,
                            column: start_column + (number_end - start) as u32,
                            spaced: false,
                        });
                        continue;
                    }
                    name.push_str(&escape.name);
                    index = escape.end;
                }
                while index < chars.len() && the_character_is_of_a_name(chars[index], marks) {
                    name.push(chars[index]);
                    index += 1;
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: a name
                // that begins with a number is a label when a colon comes
                // after it: `1abc:` is a label. Anywhere else it is a name
                // that v2 refuses.
                if chars.get(index) == Some(&':') {
                    column += (index - number_end) as u32;
                    tokens.push(Token {
                        kind: TokenKind::Name,
                        value: name,
                        line: start_line,
                        column: start_column,
                        spaced: before,
                        quote: None,
                    });
                    continue;
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
                // parameter of a command of the v1 is a text, and "2px" is a
                // text there: the number and the name are two tokens.
                //
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
                // reads a name that begins with a number: the "1C" is a
                // name, the "class 1C" is a class, and the "1e2" is a name
                // too, because the scientific notation needs a decimal point.
                if dialect.the_number_may_be_followed_by_a_name() {
                    column += (index - number_end) as u32;
                    tokens.push(Token {
                        kind: TokenKind::Name,
                        value: name,
                        line: start_line,
                        column: start_column,
                        spaced: before,
                        quote: None,
                    });
                    continue;
                }
                return Err(SyntaxError::new(
                    format!(
                        "This variable name starts with a number, which is not allowed:\n{}",
                        the_name_of_a_message(&name)
                    ),
                    start_line,
                    start_column,
                ));
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the number of the v1 may be the name of a variable, and the
            // name may not carry a quote: the `5"a"` is a name with an
            // illegal character.
            if dialect.the_name_may_not_carry_an_illegal_character()
                && chars.get(index) == Some(&'"')
            {
                let text: String = chars[start..index].iter().collect();
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
                // the number with a point is a number and not the name of a
                // variable, and the quote is not allowed there: the `5.5"a"`
                // is refused with "This character is not allowed here.".
                if text.contains('.') {
                    return Err(SyntaxError::new(
                        format!(
                            "This character is not allowed here.\n     Specifically: {}",
                            the_source_of_a_text(&chars, index)
                        ),
                        start_line,
                        start_column,
                    ));
                }
                let end = the_end_of_a_symbol_that_carries_a_quote(&chars, index);
                let word: String = chars[start..end].iter().collect();
                column += (end - index) as u32;
                index = end;
                tokens.push(Token {
                    kind: TokenKind::Name,
                    value: word,
                    line: start_line,
                    column: start_column,
                    spaced: before,
                    quote: None,
                });
                continue;
            }
            tokens.push(Token {
                kind: TokenKind::Number,
                value: chars[start..index].iter().collect(),
                quote: None,
                line: start_line,
                column: start_column,
                spaced: before,
            });
            continue;
        }
        if the_character_is_a_letter(current, marks)
            || (current == '`' && dialect.the_escape_may_be_in_a_name())
        {
            let (start_line, start_column) = (line, column);
            let start = index;
            let name = the_name_with_the_escape(&chars, index, marks, dialect);
            index = name.end;
            column += (index - start) as u32;
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the name of the v1 may not carry a quote: the `a"b"` is a name
            // with an illegal character. The name is the symbol that comes
            // with the word: the `a"b"x` is the symbol `a"b"x`, and the
            // `a"b", y` is the symbol `a"b"`.
            //
            // The lexer does not refuse it: the name of a command of the v1
            // is a text, and the refusal belongs to the place where the name
            // is a variable.
            //
            // The name that carries an escape is the name that the escape
            // made: the `a`"b` is the name `a"b`, and the symbol of the
            // source is not it.
            if !name.has_an_escape
                && dialect.the_name_may_not_carry_an_illegal_character()
                && chars.get(index) == Some(&'"')
            {
                let end = the_end_of_a_symbol_that_carries_a_quote(&chars, index);
                let word: String = chars[start..end].iter().collect();
                column += (end - index) as u32;
                index = end;
                tokens.push(Token {
                    kind: TokenKind::Name,
                    value: word,
                    line: start_line,
                    column: start_column,
                    spaced: before,
                    quote: None,
                });
                continue;
            }
            let word = name.name;
            let lower = word.to_lowercase();
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the words of the v1 may be the name of a variable, and the ones
            // that are not operators of the v1 are names: the `is` is a name
            // and not an operator.
            let kind = if KEYWORDS.contains(&lower.as_str())
                && !(dialect.the_words_may_be_the_name_of_a_variable()
                    && matches!(lower.as_str(), "is" | "as" | "in" | "contains" | "super"))
            {
                TokenKind::Keyword
            } else {
                TokenKind::Name
            };
            tokens.push(Token {
                kind,
                value: word,
                line: start_line,
                column: start_column,
                spaced: before,
                quote: None,
            });
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the escape that carries an operator is the operator that comes
            // after the name: the `x := a`.b` is the member "b" of the "a".
            if let Some(operator) = name.operator {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: operator,
                    quote: None,
                    line: start_line,
                    column: start_column,
                    spaced: false,
                });
            }
            continue;
        }
        // The backslash of a command of the v1 is a letter of the text of
        // the parameter: `FileRead, text, C:\temp\x.txt` is a command of
        // the v1, and the binary accepts it.
        //
        // Measured on the binary of v1.1.37.02 on 2026-11-02. The v2
        // refuses the backslash outside of a text.
        if current == '\\' && dialect.accepts_a_bare_backslash() {
            tokens.push(Token {
                kind: TokenKind::Name,
                value: "\\".into(),
                quote: None,
                line,
                column,
                spaced: before,
            });
            index += 1;
            column += 1;
            continue;
        }
        // Measured on the binary of v2.0.19 on 2026-11-09: the line of a hotstring
        // is one line of raw text -- `::arn't::aren't` holds the apostrophe,
        // and `::5`:::6` holds the backtick that escapes the colon -- and
        // nothing in it is a token of the dialect. The two colons that begin
        // it are a token of their own, so that the parser sees the line as the
        // hotstring it is, and the rest of the line is one text.
        if dialect.the_hotstrings_are_of_the_v2()
            && current == ':'
            && the_line_from_has_the_two_colons(&chars, index)
            && tokens.len() == the_start_of_the_current_line_tokens
        {
            // The line of a hotstring is one line of raw text: the two colons,
            // the options between the two of them (`:*:`, `:?:`), the trigger
            // and the replacement, with the quoting of the trigger and the
            // `{` of the replacement as they were written. It is one token and
            // no character of it is a token of the dialect.
            let start_column = column;
            let start = index;
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
                column += 1;
            }
            let text: String = chars[start..index].iter().collect();
            // The replacement of a hotstring may be the text of the
            // continuation section that follows it: the line after the
            // definition that begins with `(` and nothing else, and the lines
            // up to the one that closes it with `)`. Measured on the binary of
            // v2.0.19 on 2026-11-10: `::text1::` followed by `(` and three
            // lines and `)` is a hotstring whose replacement is that text.
            let mut after = index;
            if after < chars.len() && chars[after] == '\r' {
                after += 1;
            }
            let mut the_text = text;
            if after < chars.len() && chars[after] == '\n' {
                let mut at = after + 1;
                while at < chars.len() && (chars[at] == ' ' || chars[at] == '\t') {
                    at += 1;
                }
                if at < chars.len() && chars[at] == '(' && the_rest_of_the_line_is_blank(&chars, at + 1)
                {
                    let content_start = at + 1;
                    let mut line_begin = content_start;
                    let mut closing: Option<usize> = None;
                    while line_begin <= chars.len() {
                        let mut line_end = line_begin;
                        while line_end < chars.len() && chars[line_end] != '\n' {
                            line_end += 1;
                        }
                        let line_text: String = chars[line_begin..line_end].iter().collect();
                        if line_begin != content_start && line_text.trim() == ")" {
                            closing = Some(line_begin);
                            break;
                        }
                        if line_end >= chars.len() {
                            break;
                        }
                        line_begin = line_end + 1;
                    }
                    if let Some(close_start) = closing {
                        let content: String = chars[content_start..close_start].iter().collect();
                        the_text.push('\n');
                        the_text.push_str(content.trim_end_matches(['\r', '\n']));
                        let consumed = chars[index..close_start]
                            .iter()
                            .filter(|character| **character == '\n')
                            .count() as u32;
                        line += consumed;
                        // The parser sees the line of the hotstring and the
                        // text of the section as one line, and the newline
                        // that ends the closing line is the one the loop
                        // takes next.
                        index = close_start;
                        while index < chars.len() && chars[index] != '\n' {
                            index += 1;
                        }
                        column = 1;
                    }
                }
            }
            // Measured on the binary of v2.0.19 on 2026-11-10: a hotstring
            // with the `X` option executes its replacement as code, and a
            // replacement that ends with `{` is a block of statements that
            // closes with the `}` on a line of its own. The block is the text
            // of the hotstring here: the runtime has no dispatcher of a
            // hotstring yet, and the lines of the block must not be read as
            // statements of the script.
            if the_hotstrings_read_code
                && the_hotstring_of_the_v2_has_a_block(&the_text)
                && the_block_of_the_hotstring_follows(&chars, &the_text, index)
            {
                let mut end = index;
                let mut depth = 0i32;
                let mut at = end;
                while at < chars.len() {
                    let mut line_end = at;
                    while line_end < chars.len() && chars[line_end] != '\n' {
                        line_end += 1;
                    }
                    for character in &chars[at..line_end] {
                        match character {
                            '{' => depth += 1,
                            '}' => depth -= 1,
                            _ => {}
                        }
                    }
                    if depth <= 0 && chars[at..line_end].contains(&'}') {
                        end = line_end;
                        break;
                    }
                    if line_end >= chars.len() {
                        end = line_end;
                        break;
                    }
                    at = line_end + 1;
                }
                let block: String = chars[index..end].iter().collect();
                the_text.push_str(block.trim_end_matches(['\r', '\n']));
                let consumed = chars[index..end]
                    .iter()
                    .filter(|character| **character == '\n')
                    .count() as u32;
                line += consumed;
                index = end;
                while index < chars.len() && chars[index] != '\n' {
                    index += 1;
                }
                column = 1;
            }
            tokens.push(Token {
                kind: TokenKind::String,
                value: the_text,
                line,
                column: start_column,
                spaced: before,
                quote: None,
            });
            continue;
        }
        let mut matched = false;
        for operator in operators.iter().copied() {
            // A `++` or a `--` right after a number is a subtraction: `5--3`
            // is five minus three, and not a decrement of a literal.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 reads them as the increment: the `0++` is the increment
            // of the variable "0".
            if matches!(operator, "++" | "--")
                && !dialect.the_increment_after_a_number_is_an_operator()
                && tokens
                    .last()
                    .map(|token| token.kind == TokenKind::Number)
                    .unwrap_or(false)
            {
                continue;
            }
            if chars[index..].starts_with(&operator.chars().collect::<Vec<char>>()[..]) {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: (*operator).to_string(),
                    quote: None,
                    line,
                    column,
                    spaced: before,
                });
                index += operator.chars().count();
                column += operator.chars().count() as u32;
                matched = true;
                break;
            }
        }
        if !matched {
            // Measured on the binary of v2.0.19 on 2026-11-09: the `$` of the
            // spelling of a hotkey is the mark of the keyboard hook, and it is
            // the first character of the line of the hotkey: `$F1::MsgBox
            // "x"` is a hotkey. Anywhere else it is an illegal character.
            if current == '$' && tokens.len() == the_start_of_the_current_line_tokens {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: "$".into(),
                    line,
                    column,
                    spaced: before,
                    quote: None,
                });
                index += 1;
                column += 1;
                continue;
            }
            return Err(SyntaxError::new(
                format!("unexpected character {current:?}"),
                line,
                column,
            ));
        }
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        value: String::new(),
        line,
        column,
        spaced: false,
        quote: None,
    });
    Ok(tokens)
}

/// Whether the point at the place begins a number.
///
/// The numbers of v2 may begin with a point, and the point of a member
/// comes after a value: `x.5` is the member of `x` that is named `5`, and
/// it is not the number 0.5.
///
/// Measured on the binary of v2.0.19 on 2026-10-31.
fn the_point_begins_a_number(chars: &[char], index: usize, tokens: &[Token]) -> bool {
    // A point that is not followed by a digit is the point of the member.
    if !chars
        .get(index + 1)
        .is_some_and(|next| next.is_ascii_digit())
    {
        return false;
    }
    // The point of a member comes after a value.
    match tokens.last() {
        Some(token) => !the_token_ends_a_value(token),
        None => true,
    }
}

/// Whether the token is one that a value may follow.
///
/// A name, a number and a text are values, and so is the group that
/// closes with a bracket.
fn the_token_ends_a_value(token: &Token) -> bool {
    match token.kind {
        TokenKind::Name | TokenKind::Number | TokenKind::String => true,
        TokenKind::Op => matches!(token.value.as_str(), ")" | "]" | "}"),
        _ => false,
    }
}

/// Where a number ends.
///
/// The numbers of v2 are the decimals, the hex -- `0x` and the digits of
/// the base of sixteen -- and the decimals with an exponent, like `1e3`.
/// There is no binary and no octal: `0b1010` is a name that begins with
/// a number, and v2 refuses the script that carries it.
///
/// A number has one point, and the point is in the middle of it: the
/// point of `1.5.2` is the member operator of the float.
///
/// Measured on the binary of v2.0.19 on 2026-10-30.
/// Whether the digits of an exponent follow, after the sign that it may carry.
fn the_digits_of_an_exponent_follow(chars: &[char], from: usize) -> bool {
    let mut at = from;
    if matches!(chars.get(at), Some(&'+') | Some(&'-')) {
        at += 1;
    }
    chars.get(at).is_some_and(|digit| digit.is_ascii_digit())
}
fn the_end_of_the_number(
    chars: &[char],
    start: usize,
    the_exponent_needs_a_point: bool,
    marks: bool,
) -> usize {
    // The hex of v2.
    if chars.get(start) == Some(&'0')
        && matches!(chars.get(start + 1), Some(&'x') | Some(&'X'))
        && chars
            .get(start + 2)
            .is_some_and(|digit| digit.is_ascii_hexdigit())
    {
        let mut index = start + 2;
        while index < chars.len() && chars[index].is_ascii_hexdigit() {
            index += 1;
        }
        return index;
    }
    let mut index = start;
    while index < chars.len() && chars[index].is_ascii_digit() {
        index += 1;
    }
    let mut the_number_has_a_point = false;
    if chars.get(index) == Some(&'.') {
        // Measured on the binary of v2.0.19 on 2026-11-01: the point is in
        // the number when a name does not come after it: `1.ToString` is the
        // number 1 and the member that follows it, and `1.` is the number
        // 1.0.
        // The point of the concat-assign is not the point of the number:
        // `n.1.="D"` is the property "1" of `n` written by `.=`, and the
        // binary reads the number 1 and the operator. Measured on the binary
        // of v2.0.19 on 2026-11-10.
        // The point of a number that carries an exponent is the point of the
        // number even when a letter follows it: `1.e3` is 1000.0 and `1.e-3`
        // is 0.001. Measured on the binary of v2.0.19 on 2026-11-10.
        let the_point_is_in_the_number = match chars.get(index + 1) {
            Some('=') => false,
            Some(letter) if (*letter == 'e' || *letter == 'E')
                && the_digits_of_an_exponent_follow(chars, index + 2) =>
            {
                true
            }
            Some(character) if the_character_is_a_letter(*character, marks) => false,
            _ => true,
        };
        if the_point_is_in_the_number {
            the_number_has_a_point = true;
            index += 1;
            while index < chars.len() && chars[index].is_ascii_digit() {
                index += 1;
            }
        }
    }
    // The exponent of v2: the `e`, the sign and the digits. An `e` that
    // has no digits after it is not an exponent, and the name that
    // begins with the number is refused by the caller.
    //
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    // scientific notation of the v1 needs a decimal point: the "1.0e4" is
    // the number 10000, and the "1e2" is a name. The v2 reads the "1e2" as
    // the number 100.
    if (!the_exponent_needs_a_point || the_number_has_a_point)
        && matches!(chars.get(index), Some(&'e') | Some(&'E'))
    {
        let mut after = index + 1;
        if matches!(chars.get(after), Some(&'+') | Some(&'-')) {
            after += 1;
        }
        if chars.get(after).is_some_and(|digit| digit.is_ascii_digit()) {
            while after < chars.len() && chars[after].is_ascii_digit() {
                after += 1;
            }
            return after;
        }
    }
    index
}

#[cfg(test)]
mod tests {
    /// The null assignment of the v2.1 is one token, and the v2.0.19 does not
    /// know it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `x ??= 5` is refused at
    /// the load; the v2.1 has the operator of its own.
    #[test]
    fn the_null_assignment_is_a_token_of_the_v2_1() {
        let tokens = tokenize_with("z ??= 5", crate::dialect::v2_1()).expect("tokens");
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == TokenKind::Op && token.value == "??="),
            "{tokens:?}"
        );
        let tokens = tokenize_with("z ??= 5", crate::dialect::v2()).expect("tokens");
        assert!(
            !tokens
                .iter()
                .any(|token| token.kind == TokenKind::Op && token.value == "??="),
            "{tokens:?}"
        );
    }
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        tokenize(source)
            .expect("parses")
            .iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn a_string_is_one_token() {
        let tokens = tokenize("Send \"hello world\"").expect("parses");
        assert_eq!(tokens[0].value, "Send");
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[1].value, "hello world");
    }

    #[test]
    fn a_comment_is_not_a_token() {
        assert_eq!(
            kinds("Send \"a\" ; this is a comment"),
            // A command is a name, not a word of the grammar.
            vec![TokenKind::Name, TokenKind::String, TokenKind::Eof]
        );
    }

    #[test]
    fn an_unterminated_string_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the quote that is
        // not closed is named, and the line of the script says where it was.
        let error = tokenize("Send \"never closed").expect_err("fails");
        assert_eq!(error.line, 1);
        assert_eq!(
            error.message,
            "Missing \"\"\"\n     Specifically: Send \"never closed"
        );
        let error = tokenize("Send 'never closed").expect_err("fails");
        assert_eq!(error.line, 1);
        assert_eq!(
            error.message,
            "Missing \"'\"\n     Specifically: Send 'never closed"
        );
    }

    #[test]
    fn a_text_is_between_double_quotes_or_between_single_quotes() {
        // Measured on the binary of v2.0.19 on 2026-10-31.
        let tokens = tokenize("x := 'hello'").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::String);
        assert_eq!(tokens[2].value, "hello");
        // The token keeps the quote that was written: the source of the
        // script is rebuilt from the tokens.
        assert_eq!(tokens[2].quote, Some('\''));
        let tokens = tokenize("x := \"hello\"").expect("parses");
        assert_eq!(tokens[2].quote, Some('"'));
    }

    #[test]
    fn the_escape_of_the_dialect_is_the_backtick_in_the_two_kinds_of_text() {
        // Measured on the binary of v2.0.19 on 2026-10-31.
        let tokens = tokenize("x := 'a`nb'").expect("parses");
        assert_eq!(tokens[2].value, "a\nb");
        let tokens = tokenize("x := \"a`nb\"").expect("parses");
        assert_eq!(tokens[2].value, "a\nb");
    }

    #[test]
    fn a_text_does_not_cross_a_line() {
        // Measured on the binary of v2.0.19 on 2026-10-31.
        let error = tokenize("x := \"a\nb\"").expect_err("fails");
        assert_eq!(error.line, 1);
        assert_eq!(error.message, "Missing \"\"\"\n     Specifically: x := \"a");
    }

    #[test]
    fn a_number_keeps_its_point() {
        let tokens = tokenize("x := 3.5").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].value, "3.5");
    }

    #[test]
    fn the_hex_of_v2_is_one_token() {
        let tokens = tokenize("x := 0xE9").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].value, "0xE9");
    }

    #[test]
    fn the_exponent_of_v2_is_one_token() {
        let tokens = tokenize("x := 1e3").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].value, "1e3");
    }

    #[test]
    fn a_number_has_one_point() {
        let tokens = tokenize("x := 1.5.2").expect("parses");
        assert_eq!(tokens[2].value, "1.5");
        assert_eq!(tokens[3].value, ".");
    }

    #[test]
    fn a_name_that_begins_with_a_number_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-30: the script
        // does not load at all.
        let error = tokenize("x := 12abc").expect_err("fails");
        assert_eq!(
            error.message,
            "This variable name starts with a number, which is not allowed:\n\"12abc\""
        );
        assert_eq!(error.line, 1);
    }

    #[test]
    fn the_longest_operator_wins() {
        let tokens = tokenize("x := a >>>= b").expect("parses");
        assert_eq!(tokens[3].value, ">>>=");
    }

    #[test]
    fn the_assignment_operators_of_v2_are_one_token() {
        // The table of v2 has `|=`, `^=`, `&=` and `~=`, and the lexer
        // has to carry them: the longest match wins.
        for operator in ["|=", "^=", "&=", "~="] {
            let tokens = tokenize(&format!("x {operator} 2")).expect("parses");
            assert_eq!(tokens[1].value, operator);
        }
    }

    #[test]
    fn a_decrement_after_a_number_is_a_subtraction() {
        let tokens = tokenize("x := 5--3").expect("parses");
        assert_eq!(tokens[3].value, "-");
    }

    #[test]
    fn a_decrement_after_a_name_is_a_decrement() {
        let tokens = tokenize("x--").expect("parses");
        assert_eq!(tokens[1].value, "--");
    }

    #[test]
    fn a_directive_is_a_directive() {
        let tokens = tokenize("#Requires AutoHotkey v2.0.19\n").expect("parses");
        assert_eq!(tokens[0].kind, TokenKind::Directive);
        assert_eq!(tokens[0].value, "requires");
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[1].value, "AutoHotkey v2.0.19");
    }

    #[test]
    fn the_word_for_is_a_keyword() {
        // v2 reserves the word: a script cannot use it as a name.
        let tokens = tokenize("for x in y").expect("parses");
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].value, "for");
        assert_eq!(tokens[2].kind, TokenKind::Keyword);
        assert_eq!(tokens[2].value, "in");
    }

    #[test]
    fn the_word_operators_are_keywords() {
        let tokens = tokenize("a is b and c").expect("parses");
        assert_eq!(tokens[1].kind, TokenKind::Keyword);
        assert_eq!(tokens[1].value, "is");
        assert_eq!(tokens[3].kind, TokenKind::Keyword);
        assert_eq!(tokens[3].value, "and");
    }

    #[test]
    fn the_script_may_begin_with_the_byte_order_mark() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the script may
        // begin with the byte order mark, and the mark is not a character of
        // it.
        let tokens = tokenize("\u{feff}x := 1\n").expect("parses");
        assert_eq!(tokens[0].kind, TokenKind::Name);
        assert_eq!(tokens[0].value, "x");
    }

    #[test]
    fn the_byte_order_mark_is_a_letter_of_a_name() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the mark is a
        // letter of a name, and the name may begin with it.
        let tokens = tokenize("x\u{feff}y := 1\n").expect("parses");
        assert_eq!(tokens[0].kind, TokenKind::Name);
        assert_eq!(tokens[0].value, "x\u{feff}y");
    }

    #[test]
    fn the_name_that_begins_with_a_number_and_a_mark_is_refused() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name that
        // begins with a number is refused, and the character that cannot be
        // written is a question mark.
        let error = tokenize("1\u{feff}x := 1\n").expect_err("does not lex");
        assert_eq!(
            error.message,
            "This variable name starts with a number, which is not allowed:\n\"1?x\""
        );
    }

    #[test]
    fn the_directive_without_a_name_is_a_line_that_holds_no_action() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a hash that has
        // no name is not a directive, and the line holds no action.
        let error = tokenize("#\n").expect_err("does not lex");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: #"
        );
    }

    #[test]
    fn the_directive_with_a_number_is_a_line_that_holds_no_action() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // directive may not begin with a number.
        let error = tokenize("#123\n").expect_err("does not lex");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: #123"
        );
    }

    #[test]
    fn the_directive_that_is_not_known_is_a_line_that_holds_no_action() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the names that
        // the v2 knows are nineteen, and a name that is not one of them is
        // not a directive at all.
        let error = tokenize("#Foo Bar\n").expect_err("does not lex");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: #Foo Bar"
        );
    }

    #[test]
    fn the_semicolon_is_a_comment_only_when_it_is_apart() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the semicolon is
        // the beginning of a comment when it is at the start of a line or
        // when a space comes before it. The one that is glued to the token
        // that comes before it is a character of the script.
        let tokens = tokenize("x := 1;c").expect("parses");
        assert_eq!(tokens[3].kind, TokenKind::Op);
        assert_eq!(tokens[3].value, ";");
        // The semicolon that has a space before it is a comment.
        let tokens = tokenize("x := 1 ;c").expect("parses");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[3].kind, TokenKind::Eof);
        // The semicolon that opens a line is a comment.
        let tokens = tokenize(";c").expect("parses");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn the_point_of_a_number_is_not_the_point_of_a_member() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the point is in
        // the number when a name does not come after it: `1.ToString` is the
        // number 1 and the member that follows it, and `1.` is the number
        // 1.0.
        let tokens = tokenize("x := 1.ToString").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].value, "1");
        assert_eq!(tokens[3].value, ".");
        assert_eq!(tokens[4].value, "ToString");
        // The point with no name after it is in the number.
        let tokens = tokenize("x := 1.").expect("parses");
        assert_eq!(tokens[2].value, "1.");
    }

    #[test]
    fn the_three_way_comparison_is_not_a_token() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the table of the
        // v2 has `===`, and its lexer does not match it: the three signs are
        // the `==` and the `=`.
        let tokens = tokenize("x := 1 === 1").expect("parses");
        assert_eq!(tokens[3].value, "==");
        assert_eq!(tokens[4].value, "=");
    }

    // -- the continuation sections ----------------------------------------
    //
    // Measured on the binary of v2.0.19 on 2026-11-09, with the probes under
    // `egm-review/keysharp-audit/wp2/`.

    /// The values of the tokens of a script, without the newlines and the
    /// end of the source.
    fn values(source: &str) -> Vec<String> {
        tokenize(source)
            .expect("parses")
            .into_iter()
            .filter(|token| token.kind != TokenKind::Newline && token.kind != TokenKind::Eof)
            .map(|token| token.value)
            .collect()
    }

    /// The value of the first text of a script.
    fn text(source: &str) -> String {
        tokenize(source)
            .expect("parses")
            .into_iter()
            .find(|token| token.kind == TokenKind::String)
            .map(|token| token.value)
            .expect("a text")
    }

    #[test]
    fn a_section_outside_a_text_is_the_source_text_of_the_line() {
        // p12: the content of the section is spliced into the expression,
        // and the names are the concatenation of it.
        assert_eq!(values("Var :=\n(\na\nb\n)"), ["Var", ":=", "a", "b"]);
        // p14: the line above is a statement of its own, and the section
        // continues the expression of it.
        assert_eq!(values("n :=\n(\n1\n+ 2\n)"), ["n", ":=", "1", "+", "2"]);
        // p13: the merge is textual, and the operator is split across the
        // boundary.
        assert_eq!(values("op :\n(Join\n= 5\n)"), ["op", ":=", "5"]);
    }

    #[test]
    fn a_section_that_follows_a_text_is_a_part_of_that_text() {
        // p03: the newlines of the section are the text, and the text of the
        // string goes on to the line that closes it.
        let source = "Var := \"\n(\nA line.\nBy default.\n)\"";
        assert_eq!(values(source).len(), 3);
        assert_eq!(text(source), "A line.\nBy default.");
        // p10: the text that is opened on one content line and closed on a
        // later one is one text.
        let source = "Var :=\n(\n\"A line.\nA line of text.\"\n)";
        assert_eq!(text(source), "A line.\nA line of text.");
    }

    #[test]
    fn the_comment_of_the_line_above_a_section_is_of_no_account() {
        // p01: the line above ends with a text and a comment, and the text
        // of the line is the one before the comment. r01: the quote after
        // the comment is a part of the comment.
        assert_eq!(
            text("Var := \"This is a string, ; Comment.\"\n(\n    followed by a comment.\n)\""),
            "This is a string,followed by a comment."
        );
        // p02: the same when the text of the line above is not closed.
        assert_eq!(
            text("Var := \"This is a string, ; Comment.\n(\n    followed by a comment.\n)\""),
            "This is a string,followed by a comment."
        );
        // q01: the comment of the line above is stripped even when the text
        // of the line is closed, and the space that keeps two names apart
        // is not added inside a text.
        assert_eq!(text("Var := \"a ; b\"\n(\nc\n)\""), "ac");
        // q03: the comment of a line of source text is stripped, and the
        // section is spliced into the code.
        assert_eq!(values("x := 1 ; comment\n(Join\n+ 2\n)"), ["x", ":=", "1", "+", "2"]);
    }

    #[test]
    fn the_blank_lines_and_the_comments_before_a_section_are_of_no_account() {
        // p15: the blank lines and the comment lines between the line that
        // has code and the section are not a part of the merge.
        let source = "gap := \"   ; Comment.\n; Comment.\n\n( LTrim Join    ; Comment.\n   ; This is not a comment; it is literal.\n)\"";
        assert_eq!(text(source), "; This is not a comment; it is literal.");
        // p04: the section after a block comment joins the line above it.
        assert_eq!(
            values("Var :=\n(\n obj\n)\n/* Comment is ignored */\n(\n  .prop\n)"),
            ["Var", ":=", "obj", ".", "prop"]
        );
    }

    #[test]
    fn a_block_comment_is_not_a_token() {
        // Measured on the binary of v2.0.19 on 2026-11-09: the block comment
        // is of no account, and the code around it is one line.
        assert_eq!(values("x := 1 /* a comment */ + 2"), ["x", ":=", "1", "+", "2"]);
        // The newlines of a block comment are lines of the script too.
        let tokens = tokenize("x := 1 /* a\ncomment */ + 2").expect("parses");
        assert_eq!(tokens.last().map(|token| token.line), Some(2));
    }

    #[test]
    fn the_options_of_a_section_are_read() {
        // p08: LTrim takes the leading whitespace of every line away.
        assert_eq!(text("Var := \"\n( LTrim\n    One.\n    Two.\n)\""), "One.\nTwo.");
        // p08: LTrim0 keeps the leading whitespace of every line.
        assert_eq!(
            text("Var := \"\n( LTrim0\n    One.\n  Two.\n)\""),
            "    One.\n  Two."
        );
        // p08: RTrim0 keeps the trailing whitespace of every line.
        assert_eq!(text("Var := \"\n( RTrim0\none   \ntwo   \n)\""), "one   \ntwo   ");
        // p08: the indentation of the first content line is what is taken
        // away by default.
        assert_eq!(
            text("Var := \"\n(\n    One.\n    Two.\n  Three.\n)\""),
            "One.\nTwo.\n  Three."
        );
        // q05: a blank line is a line of the section.
        assert_eq!(text("Var := \"\n(\none\n\ntwo\n)\""), "one\n\ntwo");
    }

    #[test]
    fn the_option_comments_takes_the_comments_of_the_lines_away() {
        // q07 and p10: a line that carries only a comment is not a line of
        // the section at all -- it contributes nothing, not even a newline --
        // and the comment of a line takes the whitespace to its left away.
        assert_eq!(
            text("Var := \"\n(c\none   ; trailing comment\n; only a comment\ntwo\n)\""),
            "one\ntwo"
        );
        // q09: `C` is a spelling of the option.
        assert_eq!(text("Var := \"\n(C\none ; c\ntwo\n)\""), "one\ntwo");
        // p10: with RTrim0 the whitespace to the left of the comment is
        // still taken away, and the whitespace at the end of a line that
        // has no comment is kept.
        assert_eq!(
            text("Var := \"\n(Comments RTrim0\none   ; cmt\ntwo   \n)\""),
            "one\ntwo   "
        );
    }

    #[test]
    fn the_string_of_the_option_join_is_the_delimiter() {
        // p07: the string of the option joins the lines, and the option is
        // read in any case.
        assert_eq!(text("Var := \"\n( Join|\none\ntwo\n)\""), "one|two");
        assert_eq!(text("Var := \"\n(Join##\none\ntwo\n)\""), "one##two");
        assert_eq!(text("Var := \"\n(Join%\none\ntwo\n)\""), "one%two");
        // p07: the string is at most fifteen characters long.
        assert_eq!(
            text("Var := \"\n(Join0123456789abcdefGHI\none\ntwo\n)\""),
            "one0123456789abcdetwo"
        );
        // p09: the string of the option is source text as well: it is a
        // part of a name when the section is spliced into an expression.
        assert_eq!(
            values("name :=\n(JoinrL\nSt\nen(\"abcde\")\n)"),
            ["name", ":=", "StrLen", "(", "abcde", ")"]
        );
    }

    #[test]
    fn the_accent_of_a_section_makes_the_backtick_a_literal() {
        // p11: the accent option turns the escapes off.
        assert_eq!(text("Var := \"\n(`\nesc`ttab\n)\""), "esc`ttab");
        // The same text without the option translates the escapes.
        assert_eq!(text("Var := \"\n(\nesc`ttab\n)\""), "esc\ttab");
        // p09: the escaped closing parenthesis is a part of the text.
        assert_eq!(
            text("Var := \"\n(\n`)Escaped closing paren.\n)\""),
            ")Escaped closing paren."
        );
    }

    #[test]
    fn a_section_may_follow_a_section_on_the_same_logical_line() {
        // p12: the second section joins the line that the first one made,
        // and the name that ends that line is kept apart from the content of
        // the second one by a space.
        let tokens = tokenize("arr := Array(\n(Join,\na\nb\n)\n(Join,\nc\nd\n)\n)")
            .expect("parses");
        let values: Vec<String> = tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Newline && token.kind != TokenKind::Eof)
            .map(|token| token.value.clone())
            .collect();
        assert_eq!(values, ["arr", ":=", "Array", "(", "a", ",", "b", "c", ",", "d", ")"]);
        // The space before the content of the second section is there.
        let b = tokens
            .iter()
            .position(|token| token.value == "b")
            .expect("the name");
        assert_eq!(tokens[b + 1].value, "c");
        assert!(tokens[b + 1].spaced);
    }

    #[test]
    fn the_lines_of_a_text_that_hold_sections_are_counted() {
        // The section of a text counts its lines, and the next section that
        // follows it finds the line of the script where it belongs: a line
        // that is not there is answered with a line that joins nothing and
        // not with a panic.
        assert_eq!(
            values("Var := \"\n(\na\n)\n(Join\nb\n)\"\nVar2 :=\n(\n1\n+ 2\n)"),
            ["Var", ":=", "ab", "Var2", ":=", "1", "+", "2"]
        );
    }

    #[test]
    fn a_section_that_is_not_closed_names_the_text_that_it_merged() {
        // p16: the text section is named with the line that was read.
        let error = tokenize("Var := \"\n(\nA line of text.\n").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing \")\"\n     Specifically: Var := \"A line of text."
        );
        // p18: the source section is named with the line and the lines of
        // the section.
        let error = tokenize("Var :=\n(\nA line of text.\nAnother line.").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing \")\"\n     Specifically: Var :=A line of text.\nAnother line."
        );
    }

    #[test]
    fn the_option_that_v2_does_not_know_is_refused() {
        // p17: the option that is not one of the options is named.
        let error = tokenize("Var := \"\n(Bogus\nA line.\n)\"").expect_err("does not parse");
        assert_eq!(error.message, "Invalid option.\n     Specifically: Bogus");
    }
}
