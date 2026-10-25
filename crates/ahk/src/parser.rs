//! The parser: a recursive descent over the tokens.
//!
//! A port of the Python parser in the `yoko-ahk-mcp` package. The grammar,
//! in one screen::
//!
//! ```text
//! script    := statement*
//! statement := assignment | send | click | mousemove | sleep | run | pixelget
//!            | screenshot | msg | if | switch | while | loop | break | continue
//!            | return | exitapp | function
//! assignment:= NAME ('=' | ':=' | '+=' | '-=' | '.=') expression
//! send      := ('Send' | 'SendText') senditem (',' senditem)*
//! senditem  := STRING | '{' keyname '}' | expression
//! click     := 'Click' expression (',' expression)? (',' 'button'?)?
//! mousemove := 'MouseMove' expression ',' expression
//! if        := 'If' expression '::'? block ('ElseIf' expression '::'? block)* ('Else' '::'? block)?
//! switch    := 'Switch' expression '::'? '{' case* '}'
//! case      := 'Case' expression (block | ':' statement*) | 'Default' (block | ':' statement*)
//! while     := 'While' expression '::'? block
//! loop      := 'Loop' expression? '::'? block
//! function  := NAME '(' parameters ')' block
//! ```
//!
//! Expressions are the usual ones: numbers, strings, variables, the
//! arithmetic and comparison operators, `and`/`or`/`not` (and their
//! `&&`/`||`/`!` spellings), and the dot for string concatenation.

use crate::ast::{AssignOp, BinaryOp, Branch, Case, LiteralValue, Node, UnaryOp};
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

    /// Whether the statement ends here: at the end of the line, at the end
    /// of the script, or at the end of a block.
    fn at_end_of_statement(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Newline
            || token.kind == TokenKind::Eof
            || (token.kind == TokenKind::Op && token.value == "}")
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
                "switch" => return Ok(vec![self.parse_switch()?]),
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
                "return" => return Ok(vec![self.parse_return()?]),
                "exitapp" => return Ok(vec![self.parse_exitapp()?]),
                "send" | "sendtext" => return Ok(vec![self.parse_send()?]),
                "click" => return Ok(vec![self.parse_click()?]),
                "mousemove" => return Ok(vec![self.parse_mousemove()?]),
                "mousedown" => return Ok(vec![self.parse_mouse_button(false)?]),
                "mouseup" => return Ok(vec![self.parse_mouse_button(true)?]),
                "sleep" => return Ok(vec![self.parse_sleep()?]),
                "run" => return Ok(vec![self.parse_run()?]),
                "pixelget" => return Ok(vec![self.parse_pixelget()?]),
                "screenshot" => return Ok(vec![self.parse_screenshot()?]),
                "msg" | "outputdebug" => return Ok(vec![self.parse_msg()?]),
                _ => {}
            }
        }
        if token.kind == TokenKind::Name {
            return self.parse_name_statement();
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

    /// A statement that starts with a name: an assignment, a call, or the
    /// definition of a function.
    fn parse_name_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let name_token = self.next();
        if self.at_op(&["("]) {
            self.next();
            let arguments = self.parse_arguments()?;
            self.expect_op(")")?;
            if self.at_op(&["{"]) {
                // `Name(a, b) { }` is a definition; `Name(a, b)` is a call.
                let parameters = self.parameters_from(&arguments)?;
                let body = self.parse_block()?;
                return Ok(vec![Node::FunctionDef {
                    name: name_token.value,
                    parameters,
                    body,
                    line: name_token.line,
                }]);
            }
            self.end_of_statement()?;
            return Ok(vec![Node::Call {
                name: name_token.value.to_lowercase(),
                arguments,
                line: name_token.line,
            }]);
        }
        Ok(vec![self.parse_assignment(name_token)?])
    }

    fn parse_assignment(&mut self, name_token: Token) -> Result<Node, SyntaxError> {
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

    /// The arguments of a call, between the parentheses.
    fn parse_arguments(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut arguments = Vec::new();
        if self.at_op(&[")"]) {
            return Ok(arguments);
        }
        loop {
            arguments.push(self.parse_expression()?);
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        Ok(arguments)
    }

    /// The parameters of a function, read out of what was parsed as the
    /// argument list.
    ///
    /// A parameter is a name, and it may carry a default: `Name(a, b := 1)`.
    /// The list was parsed as expressions, so a default shows up as an
    /// equality whose left side is the name.
    fn parameters_from(&mut self, arguments: &[Node]) -> Result<Vec<String>, SyntaxError> {
        let mut parameters = Vec::new();
        for argument in arguments {
            match argument {
                Node::Var { name, .. } => parameters.push(name.clone()),
                Node::Binary { op: BinaryOp::Equal, left, .. } => match left.as_ref() {
                    Node::Var { name, .. } => parameters.push(name.clone()),
                    other => {
                        return Err(SyntaxError {
                            message: format!("{other:?} is not a parameter name"),
                            line: other.line(),
                            column: 0,
                        });
                    }
                },
                other => {
                    return Err(SyntaxError {
                        message: format!("{other:?} is not a parameter name"),
                        line: other.line(),
                        column: 0,
                    });
                }
            }
        }
        Ok(parameters)
    }

    fn parse_if(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut branches = Vec::new();
        let condition = self.parse_expression()?;
        self.end_of_block()?;
        let body = self.parse_block()?;
        branches.push(Branch { condition, body });
        let mut otherwise = Vec::new();
        loop {
            self.skip_newlines();
            if self.at_keyword(&["elseif"]) {
                self.next();
                let condition = self.parse_expression()?;
                self.end_of_block()?;
                let body = self.parse_block()?;
                branches.push(Branch { condition, body });
                continue;
            }
            if self.at_keyword(&["else"]) {
                self.next();
                self.end_of_block()?;
                otherwise = self.parse_block()?;
            }
            break;
        }
        Ok(Node::If { branches, otherwise, line: token.line })
    }

    fn parse_switch(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let subject = self.parse_expression()?;
        self.end_of_block()?;
        self.expect_op("{")?;
        let mut cases = Vec::new();
        loop {
            self.skip_newlines();
            if self.at_op(&["}"]) {
                self.position += 1;
                break;
            }
            if self.at_keyword(&["case"]) {
                self.next();
                let value = self.parse_expression()?;
                let body = self.parse_case_body()?;
                cases.push(Case { value: Some(value), body });
                continue;
            }
            if self.at_keyword(&["default"]) {
                self.next();
                let body = self.parse_case_body()?;
                cases.push(Case { value: None, body });
                continue;
            }
            return Err(self.error("expected Case or Default inside the Switch"));
        }
        Ok(Node::Switch { subject: Box::new(subject), cases, line: token.line })
    }

    /// The body of a case.
    ///
    /// AutoHotkey writes it in braces, or after a colon, with the statements
    /// under it. Both are accepted here.
    fn parse_case_body(&mut self) -> Result<Vec<Node>, SyntaxError> {
        if self.at_op(&["{"]) {
            return self.parse_block();
        }
        if self.at_op(&[":"]) {
            self.position += 1;
        }
        let mut body = Vec::new();
        self.skip_newlines();
        loop {
            let token = self.peek().clone();
            if token.kind == TokenKind::Eof {
                return Err(SyntaxError {
                    message: "unterminated switch".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                break;
            }
            if token.kind == TokenKind::Keyword
                && (token.value.eq_ignore_ascii_case("case") || token.value.eq_ignore_ascii_case("default"))
            {
                break;
            }
            body.extend(self.parse_statement()?);
            self.skip_newlines();
        }
        Ok(body)
    }

    fn parse_return(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let value = if self.at_end_of_statement() {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.end_of_statement()?;
        Ok(Node::Return { value, line: token.line })
    }

    fn parse_exitapp(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let code = if self.at_end_of_statement() {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.end_of_statement()?;
        Ok(Node::ExitApp { code, line: token.line })
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
            // `{a down}` and `{a up}`: the key goes down, or comes up.
            let key = name.value.clone();
            let mut state = None;
            if self.peek().kind == TokenKind::Name {
                let word = self.next().value.to_lowercase();
                if word == "down" || word == "up" {
                    state = Some(word == "down");
                } else {
                    return Err(SyntaxError {
                        message: format!("expected `down` or `up`, found {word:?}"),
                        line: name.line,
                        column: name.column,
                    });
                }
            }
            self.expect_op("}")?;
            return Ok(vec![match state {
                Some(true) => Node::KeyDown { name: key, line: token.line },
                Some(false) => Node::KeyUp { name: key, line: token.line },
                None => Node::KeyName { name: key, modifiers: Vec::new(), line: token.line },
            }]);
        }
        Ok(vec![self.parse_expression()?])
    }

    fn parse_click(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let x = Box::new(self.parse_expression()?);
        self.expect_op(",")?;
        let y = Box::new(self.parse_expression()?);
        let mut button = Box::new(Node::Literal { value: LiteralValue::Text("left".into()), line: token.line });
        let mut count = Box::new(Node::Literal { value: LiteralValue::Number(1.0), line: token.line });
        if self.at_op(&[","]) {
            self.position += 1;
            let third = self.parse_expression()?;
            if self.at_op(&[","]) {
                // AutoHotkey's order: the button, then how many clicks.
                self.position += 1;
                let fourth = self.parse_expression()?;
                button = Box::new(button_node(third, token.line));
                count = Box::new(fourth);
            } else if matches!(third, Node::Literal { value: LiteralValue::Number(_), .. }) {
                // Without a fourth argument a number is the count:
                // `Click 10, 20, 2` is a double click.
                count = Box::new(third);
            } else {
                button = Box::new(button_node(third, token.line));
            }
        }
        self.end_of_statement()?;
        Ok(Node::Click { x, y, button, count, line: token.line })
    }

    fn parse_mouse_button(&mut self, up: bool) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut x = None;
        let mut y = None;
        let mut button = Box::new(Node::Literal { value: LiteralValue::Text("left".into()), line: token.line });
        if self.at_op(&[","]) {
            // `MouseDown 10, 20, right`
            x = Some(Box::new(self.parse_expression()?));
            self.expect_op(",")?;
            y = Some(Box::new(self.parse_expression()?));
            if self.at_op(&[","]) {
                self.position += 1;
                button = Box::new(button_node(self.parse_expression()?, token.line));
            }
        } else {
            let first = self.parse_expression()?;
            if matches!(first, Node::Literal { value: LiteralValue::Number(_), .. }) {
                // A number alone is the x, and the y comes next.
                x = Some(Box::new(first));
                self.expect_op(",")?;
                y = Some(Box::new(self.parse_expression()?));
                if self.at_op(&[","]) {
                    self.position += 1;
                    button = Box::new(button_node(self.parse_expression()?, token.line));
                }
            } else {
                button = Box::new(button_node(first, token.line));
            }
        }
        self.end_of_statement()?;
        if up {
            Ok(Node::MouseUp { x, y, button, line: token.line })
        } else {
            Ok(Node::MouseDown { x, y, button, line: token.line })
        }
    }

    fn parse_mousemove(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let x = self.parse_expression()?;
        self.expect_op(",")?;
        let y = self.parse_expression()?;
        self.end_of_statement()?;
        Ok(Node::MouseMove { x: Box::new(x), y: Box::new(y), line: token.line })
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
        let node = match token.kind {
            TokenKind::Number => {
                let value = token.value.parse::<f64>().unwrap_or(0.0);
                Node::Literal { value: LiteralValue::Number(value), line: token.line }
            }
            TokenKind::String => Node::Literal { value: LiteralValue::Text(token.value), line: token.line },
            TokenKind::Keyword => {
                // A keyword that is a function name -- `PixelGet(10, 20)` --
                // is a call. The words that are values are the two below.
                if self.at_op(&["("]) {
                    return self.parse_call(token.value.to_lowercase(), token.line);
                }
                match token.value.to_lowercase().as_str() {
                    "true" => Node::Literal { value: LiteralValue::Bool(true), line: token.line },
                    "false" => Node::Literal { value: LiteralValue::Bool(false), line: token.line },
                    _ => {
                        return Err(SyntaxError {
                            message: format!("{:?} is not a value", token.value),
                            line: token.line,
                            column: token.column,
                        });
                    }
                }
            }
            TokenKind::Name => {
                if self.at_op(&["("]) {
                    return self.parse_call(token.value.to_lowercase(), token.line);
                }
                Node::Var { name: token.value, line: token.line }
            }
            TokenKind::Op if token.value == "(" => {
                let node = self.parse_expression()?;
                self.expect_op(")")?;
                node
            }
            _ => {
                return Err(SyntaxError {
                    message: format!("unexpected {:?}", token.value),
                    line: token.line,
                    column: token.column,
                });
            }
        };
        self.parse_index(node)
    }

    /// The `[...]` that may follow a value: `list[1]`, `text[2]`.
    fn parse_index(&mut self, target: Node) -> Result<Node, SyntaxError> {
        let mut node = target;
        while self.at_op(&["["]) {
            let line = node.line();
            self.position += 1;
            let index = self.parse_expression()?;
            self.expect_op("]")?;
            node = Node::Index { target: Box::new(node), index: Box::new(index), line };
        }
        Ok(node)
    }

    /// A call in an expression: the name, the arguments, the closing paren.
    fn parse_call(&mut self, name: String, line: u32) -> Result<Node, SyntaxError> {
        self.expect_op("(")?;
        let arguments = self.parse_arguments()?;
        self.expect_op(")")?;
        Ok(Node::Call { name, arguments, line })
    }
}

/// The button of a click, a mouse down or a mouse up.
///
/// A button is a word: `Click 10, 20, right`. A word that is not one of
/// the three buttons is a variable, and the value of that variable is
/// the name of the button.
fn button_node(node: Node, line: u32) -> Node {
    match &node {
        Node::Var { name, .. }
            if matches!(name.to_lowercase().as_str(), "left" | "right" | "middle") =>
        {
            Node::Literal { value: LiteralValue::Text(name.to_lowercase()), line }
        }
        _ => node,
    }
}

/// Split a quoted string into its text and key parts.
///
/// The string of a `Send` carries the AutoHotkey conventions:
///
/// * `{Name}` is a key, and `{Name down}` or `{Name up}` is that key
///   going down or coming up, on its own.
/// * `^`, `+`, `!` and `#` before a key are the modifiers: ctrl, shift,
///   alt and the super key. They apply to the key that comes next, and
///   to nothing else.
/// * A modifier on its own is a literal character, and so is one in
///   braces: `{^}` is a caret.
///
/// The text of a `Send` that is not a key is text, and it is typed as
/// it stands.
fn split_string(text: &str, line: u32) -> Vec<Node> {
    let mut parts = Vec::new();
    let mut buffer = String::new();
    let mut modifiers: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;

    let modifier_name = |character: char| match character {
        '^' => Some("ctrl"),
        '+' => Some("shift"),
        '!' => Some("alt"),
        '#' => Some("super"),
        _ => None,
    };

    while index < chars.len() {
        // A modifier before the key it belongs to.
        if let Some(name) = modifier_name(chars[index]) {
            let next = chars.get(index + 1);
            let is_literal = match next {
                None => true,
                Some(next) => next.is_whitespace(),
            };
            if !is_literal {
                modifiers.push(name.to_string());
                index += 1;
                continue;
            }
        }
        if chars[index] == '{' {
            if let Some(offset) = chars[index + 1..].iter().position(|c| *c == '}') {
                let close = index + 1 + offset;
                let name: String = chars[index + 1..close].iter().collect();
                if !buffer.is_empty() {
                    parts.push(Node::Literal { value: LiteralValue::Text(std::mem::take(&mut buffer)), line });
                }
                // `{^}` is a literal character, not a modifier.
                if name.chars().count() == 1 && modifier_name(name.chars().next().unwrap()).is_some() {
                    parts.push(Node::Literal { value: LiteralValue::Text(name), line });
                } else {
                    let (key, state) = match name.rsplit_once(' ') {
                        Some((key, "down")) => (key.to_string(), Some(true)),
                        Some((key, "up")) => (key.to_string(), Some(false)),
                        _ => (name.clone(), None),
                    };
                    match state {
                        Some(true) => parts.push(Node::KeyDown { name: key, line }),
                        Some(false) => parts.push(Node::KeyUp { name: key, line }),
                        None => parts.push(Node::KeyName { name: key, modifiers: std::mem::take(&mut modifiers), line }),
                    }
                }
                index = close + 1;
                continue;
            }
            buffer.push(chars[index]);
            index += 1;
            continue;
        }
        // A character with modifiers waiting for it is a key.
        if !modifiers.is_empty() {
            parts.push(Node::KeyName {
                name: chars[index].to_string(),
                modifiers: std::mem::take(&mut modifiers),
                line,
            });
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

    #[test]
    fn an_else_if_is_a_branch_of_the_same_if() {
        let statements = parse("If a {\n} ElseIf b {\n} Else {\n}").expect("parses");
        let Node::If { branches, otherwise, .. } = &statements[0] else { panic!("an if") };
        assert_eq!(branches.len(), 2);
        assert_eq!(otherwise.len(), 0);
    }

    #[test]
    fn a_switch_holds_its_cases() {
        let statements = parse("Switch x {\n  Case 1 {\n    Msg \"one\"\n  }\n  Default {\n    Msg \"other\"\n  }\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 2);
        assert!(cases[1].value.is_none());
    }

    #[test]
    fn a_case_can_be_written_with_a_colon() {
        let statements = parse("Switch x {\n  Case 1:\n    Msg \"one\"\n  Default:\n    Msg \"other\"\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].body.len(), 1);
    }

    #[test]
    fn a_function_definition_is_not_a_call() {
        let statements = parse("Add(a, b) {\n  return a + b\n}").expect("parses");
        let Node::FunctionDef { name, parameters, .. } = &statements[0] else { panic!("a definition") };
        assert_eq!(name, "Add");
        assert_eq!(parameters, &vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn a_call_in_a_statement_is_a_call() {
        let statements = parse("DoThing(1, 2)").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, arguments, .. } if name == "dothing" && arguments.len() == 2));
    }

    #[test]
    fn a_function_with_a_keyword_name_can_be_called() {
        let statements = parse("x := PixelGet(10, 20)").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { value, .. } if matches!(value.as_ref(), Node::Call { name, .. } if name == "pixelget")));
    }
}
