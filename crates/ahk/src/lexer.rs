//! The lexer: source text in, tokens out.
//!
//! A port of the Python lexer in the `yoko-ahk-mcp` package, which is the
//! reference implementation and the one that was verified against the lab
//! box. The two are meant to agree on every input, and the test suite in
//! this crate says so.

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
}

/// The words the language gives a meaning to.
pub const KEYWORDS: &[&str] = &[
    "if",
    "else",
    "while",
    "loop",
    "break",
    "continue",
    "send",
    "sendtext",
    "click",
    "sleep",
    "run",
    "pixelget",
    "screenshot",
    "msg",
    "outputdebug",
    "true",
    "false",
    "and",
    "or",
    "not",
];

/// The operators, longest first so that `:=` wins over `:`.
const OPERATORS: &[&str] = &[
    ":=", "+=", "-=", ".=", "==", "!=", "<=", ">=", "&&", "||", "::", "=", "<", ">", "+", "-", "*", "/",
    ".", "(", ")", ",", "{", "}", "!",
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

    while index < chars.len() {
        let current = chars[index];
        if current == '\n' {
            tokens.push(Token { kind: TokenKind::Newline, value: "\n".into(), line, column });
            index += 1;
            line += 1;
            column = 1;
            continue;
        }
        if current == ' ' || current == '\t' || current == '\r' {
            index += 1;
            column += 1;
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
                if chars[index] == '\\' && index + 1 < chars.len() {
                    let next = chars[index + 1];
                    buffer.push(match next {
                        'n' => '\n',
                        't' => '\t',
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
            tokens.push(Token { kind: TokenKind::String, value: buffer, line: start_line, column: start_column });
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
            tokens.push(Token { kind, value: word, line: start_line, column: start_column });
            continue;
        }
        let mut matched = false;
        for operator in OPERATORS {
            if chars[index..].starts_with(&operator.chars().collect::<Vec<char>>()[..]) {
                tokens.push(Token {
                    kind: TokenKind::Op,
                    value: (*operator).to_string(),
                    line,
                    column,
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
    tokens.push(Token { kind: TokenKind::Eof, value: String::new(), line, column });
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
            vec![TokenKind::Keyword, TokenKind::String, TokenKind::Eof]
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
}
