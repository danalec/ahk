//! The lexer: source text in, tokens out.
//!
//! A port of the lexer of the archived Python implementation
//! (`../archived/ahk-python`), which is the reference and the one that was
//! verified against the lab box. The two are meant to agree on every
//! input, and the test suite in this crate says so.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(test)]
use alloc::vec;

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
];

/// The operators, longest first so that `:=` wins over `:`.
///
/// The list is the whole of v2's symbolic operators. The order matters
/// in one more way: `++` and `--` are not matched right after a number,
/// because `5--3` is a subtraction and not a decrement of a literal.
const OPERATORS: &[&str] = &[
    ">>>=", "<<=", "//=", ">>=", ":=", "+=", "-=", "*=", "/=", ".=", "|=", "^=", "&=", "~=", "===", "!==",
    "==", "!=", "<=", ">=", "&&", "||", "??", "**", "//", "<<", ">>>", ">>", "++", "--", "::", "=", "<", ">",
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

/// Turn the source into tokens.
///
/// A newline ends a statement. Inside a block the statements are separated
/// by newlines too, so the parser reads a block as a list of statements and
/// the newline is just a separator there as well.
pub fn tokenize(source: &str) -> Result<Vec<Token>, SyntaxError> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0usize;
    let mut line = 1u32;
    let mut column = 1u32;

    // The first token has no space before it.
    let mut spaced = false;
    while index < chars.len() {
        let current = chars[index];
        // The space belongs to the token that comes next, and to that one
        // only.
        let before = spaced;
        spaced = false;
        if current == '\n' {
            tokens.push(Token { kind: TokenKind::Newline, value: "\n".into(), line, column, spaced: before });
            index += 1;
            line += 1;
            column = 1;
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
            while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
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
        if current == '"' {
            let (start_line, start_column) = (line, column);
            index += 1;
            column += 1;
            let mut buffer = String::new();
            while index < chars.len() && chars[index] != '"' {
                // The escape of v2 is the backtick. A backslash is a
                // backslash: the path of a file is written with it, and
                // v2 does not read it as an escape.
                if chars[index] == '`' && index + 1 < chars.len() {
                    let next = chars[index + 1];
                    buffer.push(match next {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'b' => '\u{8}',
                        'f' => '\u{c}',
                        'v' => '\u{b}',
                        '0' => '\0',
                        other => other,
                    });
                    index += 2;
                    column += 2;
                    continue;
                }
                buffer.push(chars[index]);
                index += 1;
                column += 1;
            }
            if index >= chars.len() {
                return Err(SyntaxError::new("unterminated string", start_line, start_column));
            }
            index += 1;
            column += 1;
            tokens.push(Token { kind: TokenKind::String, value: buffer, line: start_line, column: start_column, spaced: before });
            continue;
        }
        if current.is_ascii_digit() {
            let (start_line, start_column) = (line, column);
            let start = index;
            while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.') {
                index += 1;
                column += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Number,
                value: chars[start..index].iter().collect(),
                line: start_line,
                column: start_column,
                spaced: before,
            });
            continue;
        }
        if current.is_alphabetic() || current == '_' {
            let (start_line, start_column) = (line, column);
            let start = index;
            while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
                index += 1;
                column += 1;
            }
            let word: String = chars[start..index].iter().collect();
            let kind = if KEYWORDS.contains(&word.to_lowercase().as_str()) {
                TokenKind::Keyword
            } else {
                TokenKind::Name
            };
            tokens.push(Token { kind, value: word, line: start_line, column: start_column, spaced: before });
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
    tokens.push(Token { kind: TokenKind::Eof, value: String::new(), line, column, spaced: false });
    Ok(tokens)
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
        let error = tokenize("Send \"never closed").expect_err("fails");
        assert_eq!(error.line, 1);
        assert!(error.message.contains("unterminated"));
    }

    #[test]
    fn a_number_keeps_its_point() {
        let tokens = tokenize("x := 3.5").expect("parses");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].value, "3.5");
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
}
