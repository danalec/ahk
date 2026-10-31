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

use serde::Serialize;

/// What a token is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TokenKind {
    /// A name: `Send`, `x`, `invoice`.
    Name,
    /// A keyword: the same words, in the places where they mean something.
    Keyword,
    /// A number: `42`, `3.5`.
    Number,
    /// A string: the text between the quotes.
    String,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
pub const KEYWORDS: &[&str] = &[
    "if",
    "else",
    "elseif",
    "switch",
    "case",
    "default",
    "return",
    "exitapp",
    "while",
    "loop",
    "try",
    "catch",
    "finally",
    "throw",
    "for",
    "break",
    "continue",
    "goto",
    "true",
    "false",
    "and",
    "or",
    "not",
    "is",
    "in",
    "contains",
    "as",
    "class",
    "extends",
    "static",
    "global",
    "local",
    "super",
];

/// The operators, longest first so that `:=` wins over `:`.
///
/// The list is the whole of v2's symbolic operators. The order matters
/// in one more way: `++` and `--` are not matched right after a number,
/// because `5--3` is a subtraction and not a decrement of a literal.
const OPERATORS: &[&str] = &[
    ">>>=", "<<=", "//=", ">>=", ":=", "+=", "-=", "*=", "/=", ".=", "|=", "^=", "&=", "~=", "===", "!==",
    "==", "!=", "<=", ">=", "&&", "||", "??", "**", "//", "<<", ">>>", ">>", "++", "--", "::", "=>", "=", "<", ">",
    "+", "-", "*", "/", ".", "(", ")", ",", "{", "}", "[", "]", "!", ":", "?", "&", "|", "^", "~",
];

/// The script does not parse.
#[derive(Debug, thiserror::Error)]
#[error("{message} (line {line}, column {column})")]
pub struct SyntaxError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl SyntaxError {
    fn new(message: impl Into<String>, line: u32, column: u32) -> Self {
        Self { message: message.into(), line, column }
    }
}

/// Whether the character is a letter of a name.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the byte order mark is
/// a letter of a name, and a name may begin with it.
fn the_character_is_a_letter(character: char) -> bool {
    character.is_alphabetic() || character == '_' || character == '\u{feff}'
}

/// Whether the character is a part of a name.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the byte order mark is a
/// part of a name, and the name that begins with it is a name.
fn the_character_is_of_a_name(character: char) -> bool {
    character.is_alphanumeric() || character == '_' || character == '\u{feff}'
}

/// The name of a script, the way v2 writes it in a message.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the character that
/// cannot be written is a question mark.
fn the_name_of_a_message(name: &str) -> String {
    format!("{:?}", name.replace('\u{feff}', "?"))
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


/// The escape of the dialect: the backtick and the character that follows.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: `n is a linefeed, `r a
/// carriage return, `t a tab, `s a space, and the character that is not an
/// escape of the dialect is the character itself.
fn the_escape_of_the_dialect(next: char) -> char {
    match next {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'b' => '\u{8}',
        'f' => '\u{c}',
        'v' => '\u{b}',
        '0' => '\0',
        's' => ' ',
        other => other,
    }
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
fn the_section_at(chars: &[char], start: usize, line: u32) -> Result<Option<TheSection>, SyntaxError> {
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
        options.join = Some(the_escapes_of_a_text(&the_join, options.accent));
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
    Ok(Some(TheSection { options, lines, rest, end, rest_at, after: end_of_the_line + 1, closed }))
}

/// The text of the lines of a section.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the lines are joined
/// with a linefeed, the trailing whitespace is removed, and the leading
/// whitespace is removed as the indentation of the first line says -- unless
/// the option `LTrim` takes it all away or the option `LTrim0` keeps it.
fn the_content_of_the_section(section: &TheSection) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in &section.lines {
        let mut text = line.clone();
        if section.options.comments {
            // The comment of the line, and the whitespace before it, are of
            // no account.
            if let Some(at) = text.find(';') {
                text.truncate(at);
            }
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
        indentation = if spaces > 0 { (spaces, ' ') } else { (tabs, '\t') };
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
fn the_escapes_of_a_text(text: &str, accent: bool) -> String {
    if accent {
        return text.to_string();
    }
    let mut result = String::new();
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character == '`' {
            match characters.next() {
                Some(next) => result.push(the_escape_of_the_dialect(next)),
                None => result.push('`'),
            }
            continue;
        }
        result.push(character);
    }
    result
}

/// The tokens of the text of a continuation section.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the newlines of a
/// section are spaces in an expression.
fn the_tokens_of_the_content(content: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = tokenize(content)?;
    // The end of the text is not a token of the script.
    if tokens.last().map(|token| token.kind == TokenKind::Eof).unwrap_or(false) {
        tokens.pop();
    }
    let mut a_newline_was_there = false;
    let mut the_tokens: Vec<Token> = Vec::new();
    for mut token in tokens {
        if token.kind == TokenKind::Newline {
            a_newline_was_there = true;
            continue;
        }
        if a_newline_was_there {
            token.spaced = true;
            a_newline_was_there = false;
        }
        the_tokens.push(token);
    }
    Ok(the_tokens)
}

/// The code of a line of the script: the text without its comment.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the section continues the
/// last line that has code, and the name that ends that line is separated
/// from the contents of the section by a space.
fn the_code_of_a_line(text: &str) -> &str {
    match text.find(';') {
        Some(at) => &text[..at],
        None => text,
    }
}

/// Whether the last character of a line of code is a name.
///
/// A name is a letter, a digit or an underscore: the text of a string ends
/// with a quote, and the quote is not a name.
fn the_last_character_is_a_name(text: &str) -> bool {
    match the_code_of_a_line(text).trim_end().chars().last() {
        Some(last) => the_character_is_of_a_name(last),
        None => false,
    }
}

/// The line that a section is joined with.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the section continues the
/// last line that has code -- the blank lines and the comments between are of
/// no account.
fn the_line_the_section_joins(
    chars: &[char],
    the_starts_of_the_lines: &[usize],
    the_line: u32,
) -> Option<(u32, usize, usize)> {
    let mut number = the_line as usize;
    while number > 1 {
        number -= 1;
        let start = the_starts_of_the_lines[number - 1];
        let end = the_starts_of_the_lines.get(number).copied().unwrap_or(chars.len());
        let text: String = chars[start..end].iter().collect();
        if !the_code_of_a_line(&text).trim().is_empty() {
            return Some((number as u32, start, end));
        }
    }
    None
}

/// Turn the source into tokens.
///
/// A newline ends a statement. Inside a block the statements are separated
/// by newlines too, so the parser reads a block as a list of statements and
/// the newline is just a separator there as well.
pub fn tokenize(source: &str) -> Result<Vec<Token>, SyntaxError> {
    // Measured on the binary of v2.0.19 on 2026-10-31: the script may begin
    // with the byte order mark, and the mark is not a character of it. Only
    // the first one is taken: a second one is a character of the script.
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let chars: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0usize;
    let mut line = 1u32;
    let mut column = 1u32;

    // The first token has no space before it.
    let mut spaced = false;
    // The place where every line begins: the section that follows is joined
    // with the line that has code above it.
    let mut the_starts_of_the_lines: Vec<usize> = vec![0];
    while index < chars.len() {
        let current = chars[index];
        // The space belongs to the token that comes next, and to that one
        // only.
        let before = spaced;
        spaced = false;
        if current == '\n' {
            // The line that comes may open a continuation section: the
            // section continues the line above, and the newline is not a
            // separator of statements.
            if let Some(section) = the_section_at(&chars, index + 1, line + 1)? {
                // The lines of the section are lines of the script too.
                let mut cursor = index + 1;
                while cursor < section.end {
                    the_starts_of_the_lines.push(cursor);
                    match chars[cursor..section.end].iter().position(|c| *c == '\n') {
                        Some(at) => cursor += at + 1,
                        None => break,
                    }
                }
                the_section_joins_the_line_above(&mut tokens, &chars, &section, &the_starts_of_the_lines, line)?;
                index = section.end;
                line += section.lines.len() as u32 + 2;
                column = 1;
                continue;
            }
            tokens.push(Token { kind: TokenKind::Newline, value: "\n".into(), line, column, spaced: before, quote: None });
            index += 1;
            line += 1;
            column = 1;
            the_starts_of_the_lines.push(index);
            continue;
        }
        if current == ' ' || current == '\t' || current == '\r' {
            index += 1;
            column += 1;
            spaced = true;
            continue;
        }
        if current == '#' {
            // A directive: the name, and the rest of the line is its
            // argument. The argument is free text, so the lexer reads it
            // as it stands and the runtime says what it means.
            let (start_line, start_column) = (line, column);
            index += 1;
            column += 1;
            let mut name = String::new();
            while index < chars.len() && the_character_is_of_a_name(chars[index]) {
                name.push(chars[index]);
                index += 1;
                column += 1;
            }
            if name.is_empty() {
                return Err(SyntaxError::new("a directive needs a name", start_line, start_column));
            }
            let mut argument = String::new();
            while index < chars.len() && chars[index] != '\n' {
                argument.push(chars[index]);
                index += 1;
                column += 1;
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
        if current == ';' {
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
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
                while index < chars.len() && chars[index] != quote && chars[index] != '\n' {
                    // The escape of v2 is the backtick. A backslash is a
                    // backslash: the path of a file is written with it, and
                    // v2 does not read it as an escape.
                    if chars[index] == '`' && index + 1 < chars.len() {
                        buffer.push(the_escape_of_the_dialect(chars[index + 1]));
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
                // The line that comes may open a continuation section: the
                // section is inside the text, and its lines are text.
                let Some(section) = the_section_at(&chars, index + 1, line + 1)? else {
                    break;
                };
                if !section.closed {
                    // The section that is not closed is named, and the text
                    // it merged with the line above is named with it.
                    let start_of_the_line = quote_index - (start_column as usize - 1);
                    let mut text: String = chars[start_of_the_line..index].iter().collect();
                    let rest: String = chars[section.after..].iter().collect();
                    text.push_str(&rest);
                    return Err(SyntaxError::new(
                        format!("Missing \")\"\n     Specifically: {text}"),
                        start_line,
                        start_column,
                    ));
                }
                // The lines of the section are text: the quotes in them are
                // literal and the newlines are kept.
                let content = the_content_of_the_section(&section);
                the_text_of_the_section = content.clone();
                buffer.push_str(&the_escapes_of_a_text(&content, section.options.accent));
                // The text that follows the closing parenthesis is the text
                // that is being read: the quote that closes it is there.
                index = section.rest_at;
                line += section.lines.len() as u32 + 2;
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
        if current.is_ascii_digit() || (current == '.' && the_point_begins_a_number(&chars, index, &tokens)) {
            let (start_line, start_column) = (line, column);
            let start = index;
            index = the_end_of_the_number(&chars, index);
            column += (index - start) as u32;
            // A name that begins with a number is not a number: v2 refuses
            // the script at the load, and it says which name it is.
            if index < chars.len() && the_character_is_a_letter(chars[index]) {
                let number_end = index;
                let mut name = chars[start..index].iter().collect::<String>();
                while index < chars.len() && the_character_is_of_a_name(chars[index]) {
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
                return Err(SyntaxError::new(
                    format!("This variable name starts with a number, which is not allowed:\n{}", the_name_of_a_message(&name)),
                    start_line,
                    start_column,
                ));
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
        if the_character_is_a_letter(current) {
            let (start_line, start_column) = (line, column);
            let start = index;
            while index < chars.len() && the_character_is_of_a_name(chars[index]) {
                index += 1;
                column += 1;
            }
            let word: String = chars[start..index].iter().collect();
            let kind = if KEYWORDS.contains(&word.to_lowercase().as_str()) {
                TokenKind::Keyword
            } else {
                TokenKind::Name
            };
            tokens.push(Token { kind, value: word, line: start_line, column: start_column, spaced: before, quote: None });
            continue;
        }
        let mut matched = false;
        for operator in OPERATORS {
            // A `++` or a `--` right after a number is a subtraction: `5--3`
            // is five minus three, and not a decrement of a literal.
            if matches!(*operator, "++" | "--")
                && tokens.last().map(|token| token.kind == TokenKind::Number).unwrap_or(false)
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
            return Err(SyntaxError::new(format!("unexpected character {current:?}"), line, column));
        }
    }
    tokens.push(Token { kind: TokenKind::Eof, value: String::new(), line, column, spaced: false, quote: None });
    Ok(tokens)
}

/// The section that follows is joined with the line above it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the section continues
/// the last line that has code -- the blank lines and the comments between
/// are of no account -- and the name that ends that line is separated from
/// the contents of the section by a space.
fn the_section_joins_the_line_above(
    tokens: &mut Vec<Token>,
    chars: &[char],
    section: &TheSection,
    the_starts_of_the_lines: &[usize],
    line: u32,
) -> Result<(), SyntaxError> {
    // The section continues the last line that has code.
    while tokens.last().map(|token| token.kind == TokenKind::Newline).unwrap_or(false) {
        tokens.pop();
    }
    let (the_number, the_start, the_end) = match the_line_the_section_joins(chars, the_starts_of_the_lines, line + 1) {
        Some(above) => above,
        None => (line + 1, line as usize, line as usize),
    };
    let text_of_the_line: String = chars[the_start..the_end].iter().collect();
    // The space that separates the name from the contents of the section.
    let a_space = the_number != line + 1 && the_last_character_is_a_name(&text_of_the_line);
    let content = the_content_of_the_section(section);
    if !section.closed {
        // Measured on the binary of v2.0.19 on 2026-10-31: the section that is
        // not closed is named, and the text it merged with the line above is
        // named with it. The text is a line of the script, and the newline
        // that ends it is not part of it.
        let mut text = text_of_the_line.trim_end().to_string();
        if a_space {
            text.push(' ');
        }
        let mut rest: String = chars[section.after..].iter().collect();
        if rest.ends_with("\r\n") {
            rest.truncate(rest.len() - 2);
        } else if rest.ends_with('\n') {
            rest.pop();
        }
        text.push_str(&rest);
        return Err(SyntaxError::new(
            format!("Missing \")\"\n     Specifically: {text}"),
            the_number,
            1,
        ));
    }
    // The directive takes the rest of its line: the section is part of its
    // argument.
    let the_directive_is_the_line_above = tokens
        .iter()
        .rev()
        .take_while(|token| token.line == the_number)
        .any(|token| token.kind == TokenKind::Directive);
    if the_directive_is_the_line_above {
        let mut argument = String::new();
        if tokens.last().map(|token| token.kind == TokenKind::String).unwrap_or(false) {
            argument = tokens.pop().map(|token| token.value).unwrap_or_default();
        }
        if a_space {
            argument.push(' ');
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
        return Ok(());
    }
    // The tokens of the contents: the newlines of a section are spaces in an
    // expression.
    let mut the_tokens = the_tokens_of_the_content(&content).map_err(|mut error| {
        error.line = the_number;
        error
    })?;
    for token in the_tokens.iter_mut() {
        token.line = the_number;
    }
    if let Some(first) = the_tokens.first_mut() {
        first.spaced = first.spaced || a_space;
    }
    tokens.extend(the_tokens);
    // The text that follows the closing parenthesis is joined with the lines
    // of the section, and there is no delimiter between them.
    if !section.rest.is_empty() {
        let mut the_tokens = the_tokens_of_the_content(&section.rest).map_err(|mut error| {
            error.line = the_number;
            error
        })?;
        for token in the_tokens.iter_mut() {
            token.line = the_number;
        }
        tokens.extend(the_tokens);
    }
    Ok(())
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
    if !chars.get(index + 1).is_some_and(|next| next.is_ascii_digit()) {
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
fn the_end_of_the_number(chars: &[char], start: usize) -> usize {
    // The hex of v2.
    if chars.get(start) == Some(&'0')
        && matches!(chars.get(start + 1), Some(&'x') | Some(&'X'))
        && chars.get(start + 2).is_some_and(|digit| digit.is_ascii_hexdigit())
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
    if chars.get(index) == Some(&'.') {
        index += 1;
        while index < chars.len() && chars[index].is_ascii_digit() {
            index += 1;
        }
    }
    // The exponent of v2: the `e`, the sign and the digits. An `e` that
    // has no digits after it is not an exponent, and the name that
    // begins with the number is refused by the caller.
    if matches!(chars.get(index), Some(&'e') | Some(&'E')) {
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
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        tokenize(source).expect("parses").iter().map(|t| t.kind).collect()
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
        assert_eq!(error.message, "Missing \"\"\"\n     Specifically: Send \"never closed");
        let error = tokenize("Send 'never closed").expect_err("fails");
        assert_eq!(error.line, 1);
        assert_eq!(error.message, "Missing \"'\"\n     Specifically: Send 'never closed");
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
}
