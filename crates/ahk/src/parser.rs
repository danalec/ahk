//! The parser: a recursive descent over the tokens.
//!
//! A port of the Python parser in the `yoko-ahk-mcp` package. The grammar,
//! in one screen::
//!
//! ```text
//! script    := statement*
//! statement := assignment | send | click | sleep | run | pixelget
//!            | screenshot | msg | if | while | loop | break | continue
//! assignment:= NAME ('=' | ':=' | '+=' | '-=' | '.=') expression
//! send      := ('Send' | 'SendText') senditem (',' senditem)*
//! senditem  := STRING | '{' keyname '}' | expression
//! click     := 'Click' expression (',' expression)? (',' 'button'?)?
//! if        := 'If' expression '::'? block ('Else' '::'? block)?
//! while     := 'While' expression '::'? block
//! loop      := 'Loop' expression? '::'? block
//! ```
//!
//! Expressions are the usual ones: numbers, strings, variables, the
//! arithmetic and comparison operators, `and`/`or`/`not` (and their
//! `&&`/`||`/`!` spellings), and the dot for string concatenation.

use crate::ast::{AssignOp, BinaryOp, LiteralValue, Node, UnaryOp};
use crate::lexer::{SyntaxError, Token, TokenKind};

/// Parse a script into statements.
pub fn parse(source: &str) -> Result<Vec<Node>, SyntaxError> {
    let tokens = crate::lexer::tokenize(source)?;
    Parser::new(tokens).parse_script()
}

/// The recursive descent parser.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// A parser over the tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    // -- helpers ------------------------------------------------------------------

    fn peek(&self) -> &Token {
        let index = self.position.min(self.tokens.len() - 1);
        &self.tokens[index]
    }

    fn next(&mut self) -> Token {
        let token = self.peek().clone();
        self.position += 1;
        token
    }

    fn at_keyword(&self, words: &[&str]) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Keyword && words.iter().any(|word| token.value.eq_ignore_ascii_case(word))
    }

    fn at_op(&self, operators: &[&str]) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && operators.contains(&token.value.as_str())
    }

    fn expect_op(&mut self, operator: &str) -> Result<Token, SyntaxError> {
        let token = self.next();
        if token.kind != TokenKind::Op || token.value != operator {
            return Err(SyntaxError {
                message: format!("expected {operator:?}, found {:?}", token.value),
                line: token.line,
                column: token.column,
            });
        }
        Ok(token)
    }

    fn skip_newlines(&mut self) {
        while self.peek().kind == TokenKind::Newline {
            self.position += 1;
        }
    }

    fn error(&self, message: impl Into<String>) -> SyntaxError {
        let token = self.peek();
        SyntaxError { message: message.into(), line: token.line, column: token.column }
    }

    // -- statements ----------------------------------------------------------------

    /// The statements of a whole script.
    pub fn parse_script(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut statements = Vec::new();
        self.skip_newlines();
        while self.peek().kind != TokenKind::Eof {
            statements.extend(self.parse_statement()?);
            self.skip_newlines();
        }
        Ok(statements)
    }

    /// A block: the statements between `{` and `}`.
    fn parse_block(&mut self) -> Result<Vec<Node>, SyntaxError> {
        self.expect_op("{")?;
        let mut body = Vec::new();
        self.skip_newlines();
        loop {
            let token = self.peek();
            if token.kind == TokenKind::Eof {
                return Err(SyntaxError {
                    message: "unterminated block".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                self.position += 1;
                return Ok(body);
            }
            body.extend(self.parse_statement()?);
            self.skip_newlines();
        }
    }

    fn parse_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Keyword {
            let word = token.value.to_lowercase();
            match word.as_str() {
                "if" => return Ok(vec![self.parse_if()?]),
                "while" => return Ok(vec![self.parse_while()?]),
                "loop" => return Ok(vec![self.parse_loop()?]),
                "break" => {
                    self.next();
                    self.end_of_statement()?;
                    return Ok(vec![Node::Break { line: token.line }]);
                }
                "continue" => {
                    self.next();
                    self.end_of_statement()?;
                    return Ok(vec![Node::Continue { line: token.line }]);
                }
                "send" | "sendtext" => return Ok(vec![self.parse_send()?]),
                "click" => return Ok(vec![self.parse_click()?]),
                "sleep" => return Ok(vec![self.parse_sleep()?]),
                "run" => return Ok(vec![self.parse_run()?]),
                "pixelget" => return Ok(vec![self.parse_pixelget()?]),
                "screenshot" => return Ok(vec![self.parse_screenshot()?]),
                "msg" | "outputdebug" => return Ok(vec![self.parse_msg()?]),
                _ => {}
            }
        }
        if token.kind == TokenKind::Name {
            return Ok(vec![self.parse_assignment()?]);
        }
        Err(self.error(format!("unexpected {:?}", token.value)))
    }

    fn end_of_statement(&mut self) -> Result<(), SyntaxError> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Newline {
            self.position += 1;
            return Ok(());
        }
        if token.kind == TokenKind::Eof {
            return Ok(());
        }
        if token.kind == TokenKind::Op && token.value == "}" {
            return Ok(());
        }
        Err(self.error(format!("unexpected {:?} after the statement", token.value)))
    }

    /// The `::` that may introduce a block. AutoHotkey writes `If (x) {` and
    /// `If x :: {`; both mean the same, and the `::` is optional here.
    fn end_of_block(&mut self) -> Result<(), SyntaxError> {
        if self.at_op(&["::"]) {
            self.position += 1;
        }
        Ok(())
    }

    fn parse_assignment(&mut self) -> Result<Node, SyntaxError> {
        let name_token = self.next();
        let operator = self.peek().clone();
        let op = match operator.value.as_str() {
            "=" | ":=" => AssignOp::Set,
            "+=" => AssignOp::Add,
            "-=" => AssignOp::Subtract,
            ".=" => AssignOp::Concat,
            _ => {
                return Err(SyntaxError {
                    message: format!("expected an assignment operator after {:?}", name_token.value),
                    line: operator.line,
                    column: operator.column,
                });
            }
        };
        self.next();
        let value = self.parse_expression()?;
        self.end_of_statement()?;
        Ok(Node::Assign { name: name_token.value, op, value: Box::new(value), line: name_token.line })
    }

    fn parse_if(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let condition = self.parse_expression()?;
        self.end_of_block()?;
        let then_body = self.parse_block()?;
        let mut otherwise = Vec::new();
        self.skip_newlines();
        if self.at_keyword(&["else"]) {
            self.next();
            self.end_of_block()?;
            otherwise = self.parse_block()?;
        }
        Ok(Node::If {
            condition: Box::new(condition),
            then_body,
            otherwise,
            line: token.line,
        })
    }

    fn parse_while(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let condition = self.parse_expression()?;
        self.end_of_block()?;
        let body = self.parse_block()?;
        Ok(Node::While { condition: Box::new(condition), body, line: token.line })
    }

    fn parse_loop(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let count = if self.at_op(&["::", "{"]) { None } else { Some(Box::new(self.parse_expression()?)) };
        self.end_of_block()?;
        let body = self.parse_block()?;
        Ok(Node::Loop { count, body, line: token.line })
    }

    fn parse_send(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut items = Vec::new();
        loop {
            items.extend(self.parse_send_item()?);
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        self.end_of_statement()?;
        Ok(Node::Send { items, line: token.line })
    }

    /// One item of a `Send`.
    ///
    /// A quoted string can hold both text and keys: `"a{Enter}b"` is the
    /// text `a`, the key Enter, and the text `b`. The string is split
    /// here so that the runtime sees the parts.
    fn parse_send_item(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let token = self.peek().clone();
        if token.kind == TokenKind::String {
            self.position += 1;
            return Ok(split_string(&token.value, token.line));
        }
        if token.kind == TokenKind::Op && token.value == "{" {
            self.position += 1;
            let name = self.next();
            if name.kind != TokenKind::Name && name.kind != TokenKind::Keyword {
                return Err(SyntaxError {
                    message: "expected a key name".into(),
                    line: name.line,
                    column: name.column,
                });
            }
            self.expect_op("}")?;
            return Ok(vec![Node::KeyName { name: name.value, line: token.line }]);
        }
        Ok(vec![self.parse_expression()?])
    }

    fn parse_click(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let x = self.parse_expression()?;
        self.expect_op(",")?;
        let y = self.parse_expression()?;
        let mut button = "left".to_string();
        if self.at_op(&[","]) {
            self.position += 1;
            button = self.next().value.to_lowercase();
        }
        self.end_of_statement()?;
        Ok(Node::Click { x: Box::new(x), y: Box::new(y), button, line: token.line })
    }

    fn parse_sleep(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let millis = self.parse_expression()?;
        self.end_of_statement()?;
        Ok(Node::Sleep { millis: Box::new(millis), line: token.line })
    }

    fn parse_run(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let command = self.parse_expression()?;
        let mut window = String::new();
        if self.at_op(&[","]) {
            self.position += 1;
            let second = self.parse_expression()?;
            if let Node::Literal { value: LiteralValue::Text(text), .. } = second {
                window = text;
            }
        }
        self.end_of_statement()?;
        Ok(Node::Run { command: Box::new(command), window, line: token.line })
    }

    fn parse_pixelget(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let x = self.parse_expression()?;
        self.expect_op(",")?;
        let y = self.parse_expression()?;
        self.end_of_statement()?;
        Ok(Node::PixelGet { x: Box::new(x), y: Box::new(y), line: token.line })
    }

    fn parse_screenshot(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let x = self.parse_expression()?;
        self.expect_op(",")?;
        let y = self.parse_expression()?;
        self.expect_op(",")?;
        let width = self.parse_expression()?;
        self.expect_op(",")?;
        let height = self.parse_expression()?;
        let filename: Box<Node> = if self.at_op(&[","]) {
            self.position += 1;
            Box::new(self.parse_expression()?)
        } else {
            Box::new(Node::Literal { value: LiteralValue::Text(String::new()), line: token.line })
        };
        self.end_of_statement()?;
        Ok(Node::Screenshot {
            x: Box::new(x),
            y: Box::new(y),
            width: Box::new(width),
            height: Box::new(height),
            filename,
            line: token.line,
        })
    }

    fn parse_msg(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let text = self.parse_expression()?;
        self.end_of_statement()?;
        Ok(Node::Msg { text: Box::new(text), line: token.line })
    }

    // -- expressions -----------------------------------------------------------------

    fn parse_expression(&mut self) -> Result<Node, SyntaxError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Node, SyntaxError> {
        let mut node = self.parse_and()?;
        loop {
            if self.at_keyword(&["or"]) || self.at_op(&["||"]) {
                let token = self.next();
                let right = self.parse_and()?;
                node = Node::Binary {
                    op: BinaryOp::Or,
                    left: Box::new(node),
                    right: Box::new(right),
                    line: token.line,
                };
                continue;
            }
            return Ok(node);
        }
    }

    fn parse_and(&mut self) -> Result<Node, SyntaxError> {
        let mut node = self.parse_not()?;
        loop {
            if self.at_keyword(&["and"]) || self.at_op(&["&&"]) {
                let token = self.next();
                let right = self.parse_not()?;
                node = Node::Binary {
                    op: BinaryOp::And,
                    left: Box::new(node),
                    right: Box::new(right),
                    line: token.line,
                };
                continue;
            }
            return Ok(node);
        }
    }

    fn parse_not(&mut self) -> Result<Node, SyntaxError> {
        if self.at_keyword(&["not"]) || self.at_op(&["!"]) {
            let token = self.next();
            let operand = self.parse_not()?;
            return Ok(Node::Unary { op: UnaryOp::Not, operand: Box::new(operand), line: token.line });
        }
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Node, SyntaxError> {
        let mut node = self.parse_additive()?;
        loop {
            let token = self.peek().clone();
            let op = match token.value.as_str() {
                "=" | "==" => BinaryOp::Equal,
                "!=" => BinaryOp::NotEqual,
                "<" => BinaryOp::Less,
                ">" => BinaryOp::Greater,
                "<=" => BinaryOp::LessEqual,
                ">=" => BinaryOp::GreaterEqual,
                _ => return Ok(node),
            };
            if token.kind != TokenKind::Op {
                return Ok(node);
            }
            self.next();
            let right = self.parse_additive()?;
            node = Node::Binary { op, left: Box::new(node), right: Box::new(right), line: token.line };
        }
    }

    fn parse_additive(&mut self) -> Result<Node, SyntaxError> {
        let mut node = self.parse_multiplicative()?;
        loop {
            let token = self.peek().clone();
            let op = match token.value.as_str() {
                "+" => BinaryOp::Add,
                "-" => BinaryOp::Subtract,
                "." => BinaryOp::Concat,
                _ => return Ok(node),
            };
            if token.kind != TokenKind::Op {
                return Ok(node);
            }
            self.next();
            let right = self.parse_multiplicative()?;
            node = Node::Binary { op, left: Box::new(node), right: Box::new(right), line: token.line };
        }
    }

    fn parse_multiplicative(&mut self) -> Result<Node, SyntaxError> {
        let mut node = self.parse_unary()?;
        loop {
            let token = self.peek().clone();
            let op = match token.value.as_str() {
                "*" => BinaryOp::Multiply,
                "/" => BinaryOp::Divide,
                _ => return Ok(node),
            };
            if token.kind != TokenKind::Op {
                return Ok(node);
            }
            self.next();
            let right = self.parse_unary()?;
            node = Node::Binary { op, left: Box::new(node), right: Box::new(right), line: token.line };
        }
    }

    fn parse_unary(&mut self) -> Result<Node, SyntaxError> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Op && token.value == "-" {
            self.next();
            let operand = self.parse_unary()?;
            return Ok(Node::Unary { op: UnaryOp::Negate, operand: Box::new(operand), line: token.line });
        }
        if token.kind == TokenKind::Op && token.value == "+" {
            self.next();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        match token.kind {
            TokenKind::Number => {
                let value = token.value.parse::<f64>().unwrap_or(0.0);
                Ok(Node::Literal { value: LiteralValue::Number(value), line: token.line })
            }
            TokenKind::String => Ok(Node::Literal { value: LiteralValue::Text(token.value), line: token.line }),
            TokenKind::Keyword => match token.value.to_lowercase().as_str() {
                "true" => Ok(Node::Literal { value: LiteralValue::Bool(true), line: token.line }),
                "false" => Ok(Node::Literal { value: LiteralValue::Bool(false), line: token.line }),
                _ => Err(SyntaxError {
                    message: format!("{:?} is not a value", token.value),
                    line: token.line,
                    column: token.column,
                }),
            },
            TokenKind::Name => {
                if self.at_op(&["("]) {
                    self.next();
                    let mut arguments = Vec::new();
                    if !self.at_op(&[")"]) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if self.at_op(&[","]) {
                                self.position += 1;
                                continue;
                            }
                            break;
                        }
                    }
                    self.expect_op(")")?;
                    return Ok(Node::Call { name: token.value.to_lowercase(), arguments, line: token.line });
                }
                Ok(Node::Var { name: token.value, line: token.line })
            }
            TokenKind::Op if token.value == "(" => {
                let node = self.parse_expression()?;
                self.expect_op(")")?;
                Ok(node)
            }
            _ => Err(SyntaxError {
                message: format!("unexpected {:?}", token.value),
                line: token.line,
                column: token.column,
            }),
        }
    }
}

/// Split a quoted string into its text and key parts.
fn split_string(text: &str, line: u32) -> Vec<Node> {
    let mut parts = Vec::new();
    let mut buffer = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index] == '{' {
            if let Some(offset) = chars[index + 1..].iter().position(|c| *c == '}') {
                let close = index + 1 + offset;
                if !buffer.is_empty() {
                    parts.push(Node::Literal { value: LiteralValue::Text(std::mem::take(&mut buffer)), line });
                }
                let name: String = chars[index + 1..close].iter().collect();
                parts.push(Node::KeyName { name, line });
                index = close + 1;
                continue;
            }
            buffer.push(chars[index]);
            index += 1;
            continue;
        }
        buffer.push(chars[index]);
        index += 1;
    }
    if !buffer.is_empty() {
        parts.push(Node::Literal { value: LiteralValue::Text(buffer), line });
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_syntax_error_carries_the_line_and_the_column() {
        let error = parse("Send \"a\"\nClick 1\n").expect_err("fails");
        assert_eq!(error.line, 2);
    }

    #[test]
    fn the_statements_of_a_script_are_in_order() {
        let statements = parse("x := 1\nSend \"a\"\nSleep 100").expect("parses");
        assert_eq!(statements.len(), 3);
        assert!(matches!(statements[0], Node::Assign { .. }));
        assert!(matches!(statements[1], Node::Send { .. }));
        assert!(matches!(statements[2], Node::Sleep { .. }));
    }

    #[test]
    fn a_key_in_a_send_is_its_own_thing() {
        let statements = parse("Send {Enter}").expect("parses");
        let Node::Send { items, .. } = &statements[0] else { panic!("a send") };
        assert!(matches!(&items[0], Node::KeyName { name, .. } if name == "Enter"));
    }

    #[test]
    fn a_string_with_a_key_is_split() {
        let statements = parse("Send \"a{Enter}b\"").expect("parses");
        let Node::Send { items, .. } = &statements[0] else { panic!("a send") };
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn an_unknown_statement_is_an_error() {
        assert!(parse("Nonsense 1").is_err());
    }
}
