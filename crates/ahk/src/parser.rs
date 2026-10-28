//! The parser: a recursive descent over the tokens.
//!
//! A port of the parser of the archived Python implementation. The grammar,
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
//! for       := 'For' (name (',' name)?)? 'in' expression '::'? block
//! function  := NAME '(' parameters ')' block
//! array     := '[' (expression (',' expression)*)? ']'
//! ```
//!
//! Expressions are the usual ones: numbers, strings, variables, the
//! arithmetic and comparison operators, `and`/`or`/`not` (and their
//! `&&`/`||`/`!` spellings), the dot for string concatenation, and the
//! literal of an array between brackets.

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::ast::{
    AssignOp, BinaryOp, Branch, Case, CastType, LiteralValue, LoopKind, MethodDef, Node, Parameter,
    PropertyDef, UnaryOp,
};
use crate::lexer::{SyntaxError, Token, TokenKind};

/// The precedence of the operators.
///
/// The number says how tight an operator binds: a bigger number binds
/// tighter. The table is the one in the v2 documentation, from the
/// loosest to the tightest, and the parser compares the numbers instead
/// of walking a tree of rules. The gaps are there so that a group of
/// operators can be split later without renumbering the rest.
///
/// The table is public because it is the shape of the language: a client
/// that wants to know how two operators bind can ask, and the answer is
/// the same number the parser uses.
pub mod precedence {
    /// The comma, which separates the parts of a statement.
    pub const COMMA: u8 = 1;
    /// The assignments.
    pub const ASSIGN: u8 = 2;
    /// The ternary, `a ? b : c`.
    pub const TERNARY: u8 = 3;
    /// The or-maybe, `a ?? b`.
    pub const COALESCE: u8 = 4;
    /// `or`, `||`.
    pub const OR: u8 = 5;
    /// `and`, `&&`.
    pub const AND: u8 = 6;
    /// The word `not`.
    pub const NOT: u8 = 7;
    /// `is`.
    pub const MEMBERSHIP: u8 = 8;
    /// `=`, `==`, `!=`, `!==`.
    pub const EQUAL: u8 = 9;
    /// `<`, `>`, `<=`, `>=`.
    pub const COMPARE: u8 = 10;
    /// `~=`.
    pub const REGEX: u8 = 11;
    /// The concatenation, `.`.
    pub const CONCAT: u8 = 12;
    /// The bitwise or, `|`.
    pub const BIT_OR: u8 = 13;
    /// The bitwise exclusive or, `^`.
    pub const BIT_XOR: u8 = 14;
    /// The bitwise and, `&`.
    pub const BIT_AND: u8 = 15;
    /// `<<`, `>>`, `>>>`.
    pub const SHIFT: u8 = 16;
    /// `+`, `-`.
    pub const ADDITIVE: u8 = 17;
    /// `*`, `/`, `//`.
    pub const MULTIPLICATIVE: u8 = 18;
    /// `**`.
    pub const POWER: u8 = 20;
    /// The operators with one side: `-`, `+`, `!`, `~`, `&`.
    pub const UNARY: u8 = 19;
    /// `++`, `--`.
    pub const INCREMENT: u8 = 21;
}

/// Parse a script into statements.
pub fn parse(source: &str) -> Result<Vec<Node>, SyntaxError> {
    let tokens = crate::lexer::tokenize(source)?;
    Parser::new(tokens).parse_script()
}

/// The recursive descent parser.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    /// The names the script defines: its functions and its classes.
    defined: BTreeSet<String>,
    /// The calls whose name is not a built-in. They are the functions of
    /// the script, and the script may define one further down, so the
    /// check waits for the end of the parse.
    pending: Vec<(String, u32, u32)>,
}

impl Parser {
    /// A parser over the tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0, defined: BTreeSet::new(), pending: Vec::new() }
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

    /// Whether the dot that comes next reaches into a value.
    ///
    /// A dot with a space around it joins two values; a dot with no
    /// space before it reaches into one. The two are the same character
    /// and only the space tells them apart.
    fn at_member_dot(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && token.value == "." && !token.spaced
    }

    /// Whether the bracket that comes next reaches into a value.
    ///
    /// A bracket with no space before it is the item of the value; a
    /// bracket with a space before it opens the array of v2. The space
    /// is the reason, the same way it is for the dot.
    fn at_member_bracket(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && token.value == "[" && !token.spaced
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
        self.defined.clear();
        self.pending.clear();
        self.skip_newlines();
        while self.peek().kind != TokenKind::Eof {
            statements.extend(self.parse_statement()?);
            self.skip_newlines();
        }
        self.check_pending_calls()?;
        Ok(statements)
    }

    /// The check of the calls whose name is not a built-in.
    ///
    /// They are the functions of the script, and the script may define one
    /// further down, so the check waits until the whole script is read.
    fn check_pending_calls(&self) -> Result<(), SyntaxError> {
        for (name, line, column) in &self.pending {
            if self.defined.contains(name) {
                continue;
            }
            return Err(SyntaxError {
                message: "Call to nonexistent function.".into(),
                line: *line,
                column: *column,
            });
        }
        Ok(())
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
        if token.kind == TokenKind::Directive {
            return Ok(vec![self.parse_directive()?]);
        }
        if token.kind == TokenKind::Keyword {
            let word = token.value.to_lowercase();
            match word.as_str() {
                "if" => return Ok(vec![self.parse_if()?]),
                "switch" => return Ok(vec![self.parse_switch()?]),
                "while" => return Ok(vec![self.parse_while()?]),
                "loop" => return Ok(vec![self.parse_loop()?]),
                "for" => return Ok(vec![self.parse_for()?]),
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
                "class" => return Ok(vec![self.parse_class()?]),
                "return" => return Ok(vec![self.parse_return()?]),
                "exitapp" => return Ok(vec![self.parse_exitapp()?]),
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
            // `Name(a, b) { }` is a definition; `Name(a, b)` is a call. The
            // brace is what tells the two apart, and the parameters of a
            // definition are not the expressions of a call: a parameter may
            // carry a default, and it may be the one that takes the rest of
            // the arguments.
            if self.a_definition_follows() {
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                let body = self.parse_block()?;
                self.defined.insert(name_token.value.to_lowercase());
                return Ok(vec![Node::FunctionDef {
                    name: name_token.value,
                    parameters,
                    body,
                    line: name_token.line,
                }]);
            }
            let arguments = self.parse_arguments()?;
            self.expect_op(")")?;
            self.end_of_statement()?;
            return Ok(vec![Node::Call {
                name: name_token.value.to_lowercase(),
                arguments,
                line: name_token.line,
            }]);
        }
        // `obj.Prop := value` and `obj[key] := value`: the left side is a
        // value and not a name.
        if self.at_member_dot() || self.at_member_bracket() {
            let target = Node::Var { name: name_token.value.clone(), line: name_token.line };
            let target = self.parse_suffixes(target)?;
            let operator = self.peek().clone();
            let is_assignment = operator.kind == TokenKind::Op
                && matches!(
                    operator.value.as_str(),
                    "=" | ":=" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<="
                        | ">>=" | ">>>="
                );
            if is_assignment {
                let op = match operator.value.as_str() {
                    "=" | ":=" => AssignOp::Set,
                    "+=" => AssignOp::Add,
                    "-=" => AssignOp::Subtract,
                    "*=" => AssignOp::Multiply,
                    "/=" => AssignOp::Divide,
                    "//=" => AssignOp::IntegerDivide,
                    ".=" => AssignOp::Concat,
                    "&=" => AssignOp::BitwiseAnd,
                    "|=" => AssignOp::BitwiseOr,
                    "^=" => AssignOp::BitwiseXor,
                    "<<=" => AssignOp::ShiftLeft,
                    ">>=" => AssignOp::ShiftRight,
                    _ => AssignOp::ShiftRightUnsigned,
                };
                self.next();
                let value = self.parse_expression()?;
                self.end_of_statement()?;
                return Ok(vec![match target {
                    Node::Property { target, name, .. } => Node::AssignProperty {
                        target,
                        name,
                        op,
                        value: Box::new(value),
                        line: name_token.line,
                    },
                    Node::Index { target, index, .. } => Node::AssignIndex {
                        target,
                        index,
                        op,
                        value: Box::new(value),
                        line: name_token.line,
                    },
                    _ => {
                        return Err(self.error(
                            "the left side of an assignment is not a property and not an item",
                        ));
                    }
                }]);
            }
            self.end_of_statement()?;
            return Ok(vec![target]);
        }
        // A name that is neither a call nor an assignment is a call
        // statement: `Send "hello"`. The parentheses may be left out, and
        // the rest of the line is the list of the parameters.
        let operator = self.peek().clone();
        if !matches!(
            operator.value.as_str(),
            "=" | ":=" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<=" | ">>="
                | ">>>="
        ) {
            return Ok(vec![self.parse_call_statement(name_token)?]);
        }
        Ok(vec![self.parse_assignment(name_token)?])
    }

    /// A key written between braces: `{Enter}`, `{a down}`.
    ///
    /// The braces are kept in the text, because the built-in that takes
    /// the key is the one that knows what `{a down}` means.
    fn parse_brace_spec(&mut self) -> Result<Node, SyntaxError> {
        let open = self.next();
        let key = self.next();
        if key.kind != TokenKind::Name && key.kind != TokenKind::Keyword {
            return Err(SyntaxError {
                message: "expected a key name".into(),
                line: key.line,
                column: key.column,
            });
        }
        let mut spec = format!("{{{}}}", key.value);
        // `{a down}` and `{a up}`: the key goes down, or comes up.
        if self.peek().kind == TokenKind::Name {
            let word = self.next();
            let word = word.value.to_lowercase();
            if word == "down" || word == "up" {
                spec = format!("{{{} {word}}}", key.value);
            } else {
                return Err(SyntaxError {
                    message: format!("expected `down` or `up`, found {word:?}"),
                    line: key.line,
                    column: key.column,
                });
            }
        }
        self.expect_op("}")?;
        let _ = open;
        Ok(Node::Literal { value: LiteralValue::Text(spec), line: key.line })
    }

    /// A call statement: a name at the start of a line, without
    /// parentheses.
    ///
    /// The v2 documentation says that the parentheses may be left out when
    /// the answer of the call is not needed and the name is written at the
    /// start of the line: the rest of the line is the list of the
    /// parameters. Every function may be written this way, and so may
    /// every function of the script.
    ///
    /// The arguments are the expressions up to the end of the statement,
    /// in the order the callee expects them.
    fn parse_call_statement(&mut self, name_token: Token) -> Result<Node, SyntaxError> {
        let name = name_token.value.to_lowercase();
        // A name that is not a built-in is a function of the script, and
        // the script may define it further down: the check of the name
        // comes at the end of the parse.
        if crate::builtins::find(&name).is_none() {
            self.pending.push((name.clone(), name_token.line, name_token.column));
        }
        let mut arguments = Vec::new();
        if !self.at_end_of_statement() {
            loop {
                // `{Enter}` is a key, written without quotes. The built-in
                // that takes it reads the text and knows what it means.
                if self.at_op(&["{"]) {
                    arguments.push(self.parse_brace_spec()?);
                } else {
                    arguments.push(self.parse_expression()?);
                }
                if self.at_op(&[","]) {
                    self.position += 1;
                    continue;
                }
                break;
            }
        }
        self.end_of_statement()?;
        // The button of a `Click` may be written without quotes.
        if name == "click" {
            if let Some(button) = arguments.get_mut(2) {
                let taken = core::mem::replace(
                    button,
                    Node::Literal { value: LiteralValue::Text(String::new()), line: name_token.line },
                );
                *button = button_node(taken, name_token.line);
            }
        }
        Ok(Node::Call { name, arguments, line: name_token.line })
    }

    fn parse_assignment(&mut self, name_token: Token) -> Result<Node, SyntaxError> {
        let operator = self.peek().clone();
        let op = match operator.value.as_str() {
            "=" | ":=" => AssignOp::Set,
            "+=" => AssignOp::Add,
            "-=" => AssignOp::Subtract,
            "*=" => AssignOp::Multiply,
            "/=" => AssignOp::Divide,
            "//=" => AssignOp::IntegerDivide,
            ".=" => AssignOp::Concat,
            "&=" => AssignOp::BitwiseAnd,
            "|=" => AssignOp::BitwiseOr,
            "^=" => AssignOp::BitwiseXor,
            "<<=" => AssignOp::ShiftLeft,
            ">>=" => AssignOp::ShiftRight,
            ">>>=" => AssignOp::ShiftRightUnsigned,
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

    /// The parameters of a function, between the parentheses.
    ///
    /// A parameter is a name. It may carry a default -- `Name(a, b := 1)`
    /// -- and it may be the one that takes the rest of the arguments --
    /// `Name(a, rest*)`.
    ///
    /// The parameter by reference is not carried: the reference operator is
    /// not, and a parameter written that way says so.
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, SyntaxError> {
        let mut parameters = Vec::new();
        if self.at_op(&[")"]) {
            return Ok(parameters);
        }
        loop {
            let mut name_token = self.next();
            // The ByRef marker of v2: `&x` says that the caller must
            // pass a reference, and that the parameter is an alias of
            // the variable of the caller.
            let mut by_ref = false;
            if name_token.kind == TokenKind::Op && name_token.value == "&" {
                by_ref = true;
                name_token = self.next();
            }
            if name_token.kind != TokenKind::Name {
                return Err(SyntaxError {
                    message: format!("{:?} is not a parameter name", name_token.value),
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            let mut variadic = false;
            if self.at_op(&["*"]) {
                self.position += 1;
                variadic = true;
            }
            let mut default = None;
            if self.at_op(&[":="]) {
                self.position += 1;
                default = Some(Box::new(self.parse_expression()?));
            }
            parameters.push(Parameter { name: name_token.value, default, variadic, by_ref });
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        Ok(parameters)
    }

    /// Whether a definition follows the parenthesis.
    ///
    /// A definition is `Name(a, b) { }`, and the brace is what tells it
    /// from a call. The list is scanned to its end, and the token after it
    /// is looked at.
    fn a_definition_follows(&self) -> bool {
        let mut depth = 0usize;
        let mut at = self.position;
        while at < self.tokens.len() {
            let token = &self.tokens[at];
            if token.kind == TokenKind::Op {
                match token.value.as_str() {
                    "(" | "[" => depth += 1,
                    ")" | "]" => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            return self
                                .tokens
                                .get(at + 1)
                                .map(|token| token.value == "{")
                                .unwrap_or(false);
                        }
                    }
                    _ => {}
                }
            }
            if token.kind == TokenKind::Eof {
                break;
            }
            at += 1;
        }
        false
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

    /// A class: `class Name extends Base { }`.
    fn parse_class(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let name_token = self.next();
        if name_token.kind != TokenKind::Name {
            return Err(SyntaxError {
                message: "a class needs a name".into(),
                line: name_token.line,
                column: name_token.column,
            });
        }
        let mut extends = None;
        if self.at_keyword(&["extends"]) {
            self.next();
            let base = self.next();
            if base.kind != TokenKind::Name {
                return Err(SyntaxError {
                    message: "a class needs the name of the class it extends".into(),
                    line: base.line,
                    column: base.column,
                });
            }
            extends = Some(base.value);
        }
        let (methods, properties) = self.parse_class_body()?;
        self.defined.insert(name_token.value.to_lowercase());
        Ok(Node::ClassDef {
            name: name_token.value,
            extends,
            methods,
            properties,
            line: token.line,
        })
    }

    /// The members of a class: the methods and the properties.
    fn parse_class_body(&mut self) -> Result<(Vec<MethodDef>, Vec<PropertyDef>), SyntaxError> {
        self.expect_op("{")?;
        let mut methods = Vec::new();
        let mut properties = Vec::new();
        loop {
            self.skip_newlines();
            let token = self.peek().clone();
            if token.kind == TokenKind::Eof {
                return Err(SyntaxError {
                    message: "unterminated class".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                self.position += 1;
                break;
            }
            // A member may belong to the class and not to an instance.
            let mut is_static = false;
            if self.at_keyword(&["static"]) {
                self.next();
                is_static = true;
            }
            let name_token = self.next();
            if name_token.kind != TokenKind::Name && name_token.kind != TokenKind::Keyword {
                return Err(SyntaxError {
                    message: "a member of a class needs a name".into(),
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            if self.at_op(&["("]) {
                self.next();
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                let body = self.parse_block()?;
                methods.push(MethodDef {
                    name: name_token.value,
                    parameters,
                    body,
                    is_static,
                    line: name_token.line,
                });
                continue;
            }
            // A property: `Name := value`.
            let operator = self.next();
            if operator.kind != TokenKind::Op || !matches!(operator.value.as_str(), "=" | ":=") {
                return Err(SyntaxError {
                    message: "a member of a class is a method or a property".into(),
                    line: operator.line,
                    column: operator.column,
                });
            }
            let value = self.parse_expression()?;
            properties.push(PropertyDef {
                name: name_token.value,
                value: Some(Box::new(value)),
                is_static,
                line: name_token.line,
            });
        }
        Ok((methods, properties))
    }

    /// A directive: the name, and the rest of the line.
    fn parse_directive(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        // The argument is the text that follows the name, and the lexer
        // put it in the next token.
        let argument = if self.peek().kind == TokenKind::String {
            self.next().value
        } else {
            String::new()
        };
        self.end_of_statement()?;
        Ok(Node::Directive { name: token.value, argument, line: token.line })
    }

    fn parse_loop(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        // The kinds of loop: `Loop Files`, `Loop Reg`, `Loop Read` and
        // `Loop Parse`. The word is a name of the kind and not the
        // start of an expression.
        let kind = match self.peek().value.to_lowercase().as_str() {
            "files" if self.peek().kind == TokenKind::Name => LoopKind::Files,
            "reg" if self.peek().kind == TokenKind::Name => LoopKind::Reg,
            "read" if self.peek().kind == TokenKind::Name => LoopKind::Read,
            "parse" if self.peek().kind == TokenKind::Name => LoopKind::Parse,
            _ => LoopKind::Count,
        };
        let mut arguments = Vec::new();
        if kind == LoopKind::Count {
            if !self.at_op(&["::", "{"]) {
                arguments.push(self.parse_expression()?);
            }
        } else {
            self.next();
            // The comma after the kind is optional: v2 takes it and does
            // not need it.
            if self.at_op(&[","]) {
                self.position += 1;
            }
            if !self.at_op(&["::", "{"]) {
                loop {
                    arguments.push(self.parse_expression()?);
                    if self.at_op(&[","]) {
                        self.position += 1;
                        continue;
                    }
                    break;
                }
            }
        }
        self.end_of_block()?;
        let body = self.parse_block()?;
        Ok(Node::Loop { mode: kind, arguments, body, line: token.line })
    }

    /// The loop over what an object enumerates: `for x in y { }`.
    ///
    /// The names are the ones the enumerator gives, and a name may be
    /// left out: `for , value in map` asks for the value alone. The whole
    /// list may also be written between parentheses, the way v2 writes
    /// it when the expression is long.
    fn parse_for(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut variables = Vec::new();
        // The list may be written between parentheses: `for (x in y)`.
        let parenthesised = self.at_op(&["("]);
        if parenthesised {
            self.next();
        }
        self.parse_for_variables(&mut variables)?;
        if !self.at_keyword(&["in"]) {
            return Err(self.error("expected `in` in the For loop"));
        }
        self.next();
        let source = self.parse_expression()?;
        if parenthesised {
            self.expect_op(")")?;
        }
        self.end_of_block()?;
        let body = self.parse_block()?;
        Ok(Node::For { variables, source: Box::new(source), body, line: token.line })
    }

    /// The names the enumerator gives, before the `in`.
    ///
    /// A name may be left out, and the name that is left out is the
    /// empty string: `for , value in map` gives `["", "value"]`.
    fn parse_for_variables(&mut self, variables: &mut Vec<String>) -> Result<(), SyntaxError> {
        // The first name. It is left out when the comma comes first.
        if self.at_op(&[","]) {
            variables.push(String::new());
        } else if self.peek().kind == TokenKind::Name {
            variables.push(self.next().value);
        } else if !self.at_keyword(&["in"]) {
            let token = self.peek().clone();
            return Err(SyntaxError {
                message: format!("{:?} is not a variable name", token.value),
                line: token.line,
                column: token.column,
            });
        }
        // The second name, after the comma that separates the two.
        if self.at_op(&[","]) {
            self.next();
            if self.peek().kind == TokenKind::Name {
                variables.push(self.next().value);
            } else {
                variables.push(String::new());
            }
        }
        Ok(())
    }

    // -- expressions -----------------------------------------------------------------

    /// An expression, with v2's operators and v2's precedence.
    fn parse_expression(&mut self) -> Result<Node, SyntaxError> {
        self.parse_binary(precedence::COMMA)
    }

    /// The binary operators, from the loosest to the tightest.
    ///
    /// The precedence is a number and the parser compares the numbers: a
    /// bigger number binds tighter, and the operator that comes after the
    /// one being parsed says how far to the right the expression reaches.
    /// A right-associative operator reaches as far as it can; a
    /// left-associative one stops before the next of its own kind.
    fn parse_binary(&mut self, min: u8) -> Result<Node, SyntaxError> {
        let mut left = self.parse_unary()?;
        loop {
            // The ternary is not in the table: it takes two expressions
            // and a colon, and the colon belongs to it.
            if self.at_op(&["?"]) && precedence::TERNARY >= min {
                let token = self.next();
                let then = self.parse_binary(precedence::COALESCE)?;
                self.expect_op(":")?;
                let otherwise = self.parse_binary(precedence::TERNARY)?;
                left = Node::Ternary {
                    condition: Box::new(left),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                    line: token.line,
                };
                continue;
            }
            let Some((op, prec, right_associative)) = self.peek_binary_op() else {
                // The implicit concatenation of v2: two values side by side
                // are one value. It binds like the dot, and the dot is
                // looser than the arithmetic.
                if precedence::CONCAT < min || !self.starts_expression() {
                    return Ok(left);
                }
                let right = self.parse_binary(precedence::CONCAT + 1)?;
                let line = left.line();
                left = Node::Binary {
                    op: BinaryOp::Concat,
                    left: Box::new(left),
                    right: Box::new(right),
                    line,
                };
                continue;
            };
            if prec < min {
                return Ok(left);
            }
            let token = self.next();
            let next = if right_associative { prec } else { prec + 1 };
            let right = self.parse_binary(next)?;
            left = Node::Binary { op, left: Box::new(left), right: Box::new(right), line: token.line };
        }
    }

    /// The binary operator that comes next, if there is one.
    fn peek_binary_op(&self) -> Option<(BinaryOp, u8, bool)> {
        let token = self.peek();
        if token.kind == TokenKind::Op {
            // The dot with no space before it reaches into a value, and
            // it is not the operator that joins two values.
            if token.value == "." && !token.spaced {
                return None;
            }
            let (op, prec, right) = match token.value.as_str() {
                "**" => (BinaryOp::Power, precedence::POWER, true),
                "*" => (BinaryOp::Multiply, precedence::MULTIPLICATIVE, false),
                "/" => (BinaryOp::Divide, precedence::MULTIPLICATIVE, false),
                "//" => (BinaryOp::IntegerDivide, precedence::MULTIPLICATIVE, false),
                "+" => (BinaryOp::Add, precedence::ADDITIVE, false),
                "-" => (BinaryOp::Subtract, precedence::ADDITIVE, false),
                "<<" => (BinaryOp::ShiftLeft, precedence::SHIFT, false),
                ">>" => (BinaryOp::ShiftRight, precedence::SHIFT, false),
                ">>>" => (BinaryOp::ShiftRightUnsigned, precedence::SHIFT, false),
                "&" => (BinaryOp::BitwiseAnd, precedence::BIT_AND, false),
                "^" => (BinaryOp::BitwiseXor, precedence::BIT_XOR, false),
                "|" => (BinaryOp::BitwiseOr, precedence::BIT_OR, false),
                "." => (BinaryOp::Concat, precedence::CONCAT, false),
                "~=" => (BinaryOp::RegexMatch, precedence::REGEX, false),
                ">" => (BinaryOp::Greater, precedence::COMPARE, false),
                ">=" => (BinaryOp::GreaterEqual, precedence::COMPARE, false),
                "<" => (BinaryOp::Less, precedence::COMPARE, false),
                "<=" => (BinaryOp::LessEqual, precedence::COMPARE, false),
                "=" => (BinaryOp::Equal, precedence::EQUAL, false),
                "==" => (BinaryOp::StrictEqual, precedence::EQUAL, false),
                "!=" => (BinaryOp::NotEqual, precedence::EQUAL, false),
                "!==" => (BinaryOp::StrictNotEqual, precedence::EQUAL, false),
                "&&" => (BinaryOp::And, precedence::AND, false),
                "||" => (BinaryOp::Or, precedence::OR, false),
                "??" => (BinaryOp::Coalesce, precedence::COALESCE, true),
                _ => return None,
            };
            return Some((op, prec, right));
        }
        if token.kind == TokenKind::Keyword {
            let (op, prec, right) = match token.value.to_lowercase().as_str() {
                "and" => (BinaryOp::And, precedence::AND, false),
                "or" => (BinaryOp::Or, precedence::OR, false),
                "is" => (BinaryOp::Is, precedence::MEMBERSHIP, false),
                _ => return None,
            };
            return Some((op, prec, right));
        }
        None
    }

    /// Whether the token that comes next starts a value.
    ///
    /// The implicit concatenation of v2 is two values side by side, so the
    /// parser has to know where a value may begin. The keywords that are
    /// operators are not among them: `not` is one, and `and` and `or` are
    /// two.
    fn starts_expression(&self) -> bool {
        let token = self.peek();
        match token.kind {
            TokenKind::Number | TokenKind::String | TokenKind::Name => true,
            TokenKind::Keyword => {
                let word = token.value.to_lowercase();
                if word == "true" || word == "false" {
                    return true;
                }
                // A call has parentheses after the name.
                match self.tokens.get(self.position + 1) {
                    Some(next) => next.kind == TokenKind::Op && next.value == "(",
                    None => false,
                }
            }
            _ => false,
        }
    }

    /// The operators with one side, and the values.
    ///
    /// The word `not` binds looser than `!` and looser than the
    /// comparisons: `not x = 3` is `not (x = 3)`, the way v2 reads it.
    fn parse_unary(&mut self) -> Result<Node, SyntaxError> {
        let token = self.peek().clone();
        if token.kind == TokenKind::Op {
            let op = match token.value.as_str() {
                "-" => Some(UnaryOp::Negate),
                "+" => Some(UnaryOp::Plus),
                "!" => Some(UnaryOp::Not),
                "~" => Some(UnaryOp::BitwiseNot),
                "&" => Some(UnaryOp::Address),
                "++" => Some(UnaryOp::PreIncrement),
                "--" => Some(UnaryOp::PreDecrement),
                _ => None,
            };
            if let Some(op) = op {
                self.next();
                // The power binds tighter than the sign and the bitwise
                // operators: `-2 ** 2` is `-(2 ** 2)`, the way v2 reads
                // it. The increment and the decrement bind tighter than
                // the power, so they take only the value.
                let operand = match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => self.parse_unary()?,
                    _ => self.parse_binary(precedence::POWER)?,
                };
                return Ok(Node::Unary { op, operand: Box::new(operand), line: token.line });
            }
        }
        if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("not") {
            self.next();
            let operand = self.parse_binary(precedence::NOT)?;
            return Ok(Node::Unary { op: UnaryOp::NotWord, operand: Box::new(operand), line: token.line });
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
                // A keyword that is a function name -- `PixelGetColor(10, 20)`
                // -- is a call. The words that are values are the two below.
                if self.at_op(&["("]) {
                    return self.parse_call(token.value.to_lowercase(), token.line);
                }
                match token.value.to_lowercase().as_str() {
                    "true" => Node::Literal { value: LiteralValue::Bool(true), line: token.line },
                    "false" => Node::Literal { value: LiteralValue::Bool(false), line: token.line },
                    "in" | "contains" => {
                        return Err(SyntaxError {
                            message: format!(
                                "{:?} is reserved for a future of AutoHotkey and does nothing yet",
                                token.value
                            ),
                            line: token.line,
                            column: token.column,
                        });
                    }
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
                let name = token.value;
                // `x++` and `x--`: the operator comes after the name.
                if self.at_op(&["++", "--"]) {
                    let operator = self.next();
                    let op = if operator.value == "++" {
                        UnaryOp::PostIncrement
                    } else {
                        UnaryOp::PostDecrement
                    };
                    let operand = Node::Var { name, line: token.line };
                    return Ok(Node::Unary { op, operand: Box::new(operand), line: token.line });
                }
                Node::Var { name, line: token.line }
            }
            TokenKind::Op if token.value == "(" => {
                let node = self.parse_expression()?;
                self.expect_op(")")?;
                node
            }
            TokenKind::Op if token.value == "[" => {
                // The literal of an array: `[a, b, c]`. The brackets of
                // an index are a suffix of a value, and these open one.
                let mut items = Vec::new();
                if !self.at_op(&["]"]) {
                    loop {
                        items.push(self.parse_expression()?);
                        if self.at_op(&[","]) {
                            self.position += 1;
                            continue;
                        }
                        break;
                    }
                }
                self.expect_op("]")?;
                Node::ArrayLiteral { items, line: token.line }
            }
            _ => {
                return Err(SyntaxError {
                    message: format!("unexpected {:?}", token.value),
                    line: token.line,
                    column: token.column,
                });
            }
        };
        self.parse_suffixes(node)
    }

    /// The `[...]` and the `as` that may follow a value.
    fn parse_suffixes(&mut self, target: Node) -> Result<Node, SyntaxError> {
        let mut node = target;
        loop {
            if self.at_op(&["["]) {
                let line = node.line();
                self.position += 1;
                let index = self.parse_expression()?;
                self.expect_op("]")?;
                node = Node::Index { target: Box::new(node), index: Box::new(index), line };
                continue;
            }
            if self.at_member_dot() {
                let dot = self.next();
                let name = self.next();
                if name.kind != TokenKind::Name && name.kind != TokenKind::Keyword {
                    return Err(SyntaxError {
                        message: "a member needs a name".into(),
                        line: name.line,
                        column: name.column,
                    });
                }
                // A member with parentheses is a method, and a member
                // without them is a property.
                if self.at_op(&["("]) {
                    self.next();
                    let arguments = self.parse_arguments()?;
                    self.expect_op(")")?;
                    node = Node::MethodCall {
                        target: Box::new(node),
                        name: name.value,
                        arguments,
                        line: dot.line,
                    };
                    continue;
                }
                node = Node::Property { target: Box::new(node), name: name.value, line: dot.line };
                continue;
            }
            if self.at_keyword(&["as"]) {
                let token = self.next();
                let name = self.next();
                let target_type = match name.value.to_lowercase().as_str() {
                    "integer" => CastType::Integer,
                    "float" => CastType::Float,
                    "string" => CastType::String,
                    "number" => CastType::Number,
                    other => {
                        return Err(SyntaxError {
                            message: format!("{other:?} is not a type"),
                            line: name.line,
                            column: name.column,
                        });
                    }
                };
                node = Node::Cast { value: Box::new(node), target: target_type, line: token.line };
                continue;
            }
            break;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_syntax_error_carries_the_line_and_the_column() {
        let error = parse("Send \"a\"\nNonsense 1\n").expect_err("fails");
        assert_eq!(error.line, 2);
    }

    #[test]
    fn the_statements_of_a_script_are_in_order() {
        let statements = parse("x := 1\nSend \"a\"\nSleep 100").expect("parses");
        assert_eq!(statements.len(), 3);
        assert!(matches!(statements[0], Node::Assign { .. }));
        assert!(matches!(&statements[1], Node::Call { name, .. } if name == "send"));
        assert!(matches!(&statements[2], Node::Call { name, .. } if name == "sleep"));
    }

    #[test]
    fn a_key_in_a_send_is_its_own_thing() {
        let statements = parse("Send {Enter}").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else { panic!("a send") };
        assert!(matches!(&arguments[0], Node::Literal { value: LiteralValue::Text(text), .. } if text == "{Enter}"));
    }

    #[test]
    fn a_key_that_goes_down_says_so() {
        let statements = parse("Send {a down}").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else { panic!("a send") };
        assert!(matches!(&arguments[0], Node::Literal { value: LiteralValue::Text(text), .. } if text == "{a down}"));
    }

    #[test]
    fn a_function_may_be_written_as_a_call_statement() {
        // The v2 documentation: the parentheses may be left out when the
        // answer is not needed and the name is at the start of the line.
        let statements = parse("Abs 5").expect("parses");
        assert!(
            matches!(&statements[0], Node::Call { name, arguments, .. } if name == "abs" && arguments.len() == 1)
        );
    }

    #[test]
    fn a_function_of_the_script_may_be_written_as_a_call_statement() {
        // The script may define the function further down: the check of
        // the name comes at the end of the parse.
        let statements = parse("MyFunc\n\nMyFunc() {\n    return 7\n}").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, .. } if name == "myfunc"));
    }

    #[test]
    fn a_name_that_is_neither_a_built_in_nor_a_function_is_an_error() {
        let error = parse("Nonsense 1").expect_err("fails");
        assert_eq!(error.message, "Call to nonexistent function.");
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
        let statements = parse("Switch x {\n  Case 1 {\n    MsgBox \"one\"\n  }\n  Default {\n    MsgBox \"other\"\n  }\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 2);
        assert!(cases[1].value.is_none());
    }

    #[test]
    fn a_case_can_be_written_with_a_colon() {
        let statements = parse("Switch x {\n  Case 1:\n    MsgBox \"one\"\n  Default:\n    MsgBox \"other\"\n}")
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
        let names: Vec<&str> = parameters.iter().map(|parameter| parameter.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b"]);
        assert!(parameters.iter().all(|parameter| parameter.default.is_none()));
        assert!(parameters.iter().all(|parameter| !parameter.variadic));
    }

    #[test]
    fn a_parameter_may_carry_a_default() {
        let statements = parse("Add(a, b := 1) {\n  return a + b\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else { panic!("a definition") };
        assert!(parameters[0].default.is_none());
        assert!(parameters[1].default.is_some());
    }

    #[test]
    fn a_parameter_may_take_the_rest_of_the_arguments() {
        let statements = parse("Add(a, rest*) {\n  return a\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else { panic!("a definition") };
        assert!(!parameters[0].variadic);
        assert!(parameters[1].variadic);
    }

    #[test]
    fn a_parameter_may_be_written_by_reference() {
        // The ByRef marker of v2: the caller must pass a reference, and
        // the parameter is an alias of the variable of the caller.
        let statements = parse("Add(&a) {\n  return a\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else { panic!("a definition") };
        assert!(parameters[0].by_ref);
        assert_eq!(parameters[0].name, "a");
    }

    #[test]
    fn a_call_in_a_statement_is_a_call() {
        let statements = parse("DoThing(1, 2)").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, arguments, .. } if name == "dothing" && arguments.len() == 2));
    }

    #[test]
    fn the_precedence_table_is_the_one_of_v2() {
        // The order, from the loosest to the tightest, is the one in the
        // v2 documentation. The numbers are what the parser compares, so
        // the order of the numbers is the order of the binding.
        let order = [
            (precedence::COMMA, ","),
            (precedence::ASSIGN, ":="),
            (precedence::TERNARY, "?"),
            (precedence::COALESCE, "??"),
            (precedence::OR, "or"),
            (precedence::AND, "and"),
            (precedence::NOT, "not"),
            (precedence::MEMBERSHIP, "is"),
            (precedence::EQUAL, "="),
            (precedence::COMPARE, ">"),
            (precedence::REGEX, "~="),
            (precedence::CONCAT, "."),
            (precedence::BIT_OR, "|"),
            (precedence::BIT_XOR, "^"),
            (precedence::BIT_AND, "&"),
            (precedence::SHIFT, "<<"),
            (precedence::ADDITIVE, "+"),
            (precedence::MULTIPLICATIVE, "*"),
            (precedence::UNARY, "!"),
            (precedence::POWER, "**"),
            (precedence::INCREMENT, "++"),
        ];
        for pair in order.windows(2) {
            assert!(
                pair[0].0 < pair[1].0,
                "{} and {} are the wrong way round",
                pair[0].1,
                pair[1].1
            );
        }
    }

    #[test]
    fn the_multiplication_binds_tighter_than_the_addition() {
        let statements = parse("x := 1 + 2 * 3").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::Binary { op, right, .. } = value.as_ref() else { panic!("an addition") };
        assert_eq!(*op, BinaryOp::Add);
        assert!(matches!(right.as_ref(), Node::Binary { op: BinaryOp::Multiply, .. }));
    }

    #[test]
    fn the_power_binds_to_the_right() {
        let statements = parse("x := 2 ** 3 ** 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::Binary { op, right, .. } = value.as_ref() else { panic!("a power") };
        assert_eq!(*op, BinaryOp::Power);
        assert!(matches!(right.as_ref(), Node::Binary { op: BinaryOp::Power, .. }));
    }

    #[test]
    fn the_power_binds_tighter_than_the_unary_minus() {
        // Measured on the binary of v2.0.19 on 2026-10-28: `-2 ** 2` is
        // -4 there, and not 4.
        let statements = parse("x := -2 ** 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::Unary { op, operand, .. } = value.as_ref() else { panic!("a negation") };
        assert_eq!(*op, UnaryOp::Negate);
        assert!(matches!(operand.as_ref(), Node::Binary { op: BinaryOp::Power, .. }));
    }

    #[test]
    fn the_word_not_binds_looser_than_the_comparison() {
        let statements = parse("x := not a = 3").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::Unary { op, operand, .. } = value.as_ref() else { panic!("a not") };
        assert_eq!(*op, UnaryOp::NotWord);
        assert!(matches!(operand.as_ref(), Node::Binary { op: BinaryOp::Equal, .. }));
    }

    #[test]
    fn the_ternary_takes_the_branch_that_holds() {
        let statements = parse("x := a ? 1 : 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        assert!(matches!(value.as_ref(), Node::Ternary { .. }));
    }

    #[test]
    fn a_function_with_a_keyword_name_can_be_called() {
        let statements = parse("x := PixelGet(10, 20)").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { value, .. } if matches!(value.as_ref(), Node::Call { name, .. } if name == "pixelget")));
    }

    #[test]
    fn the_empty_brackets_are_the_empty_array() {
        let statements = parse("x := []").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn an_array_holds_its_values_in_the_order_they_are_written() {
        let statements = parse("x := [1, 2, 3]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 3);
        assert!(matches!(&items[0], Node::Literal { value: LiteralValue::Number(number), .. } if *number == 1.0));
        assert!(matches!(&items[2], Node::Literal { value: LiteralValue::Number(number), .. } if *number == 3.0));
    }

    #[test]
    fn an_array_holds_expressions() {
        let statements = parse("x := [a + b, c * d]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], Node::Binary { op: BinaryOp::Add, .. }));
        assert!(matches!(&items[1], Node::Binary { op: BinaryOp::Multiply, .. }));
    }

    #[test]
    fn an_array_may_hold_another_array() {
        let statements = parse("x := [[1], [2]]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], Node::ArrayLiteral { items, .. } if items.len() == 1));
    }

    #[test]
    fn the_suffixes_reach_into_an_array_literal() {
        // The literal is a primary, and the suffixes follow it the way
        // they follow a variable.
        let statements = parse("x := [1, 2].Length").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::Property { target, name, .. } = value.as_ref() else { panic!("a property") };
        assert_eq!(name, "Length");
        assert!(matches!(target.as_ref(), Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn an_array_may_be_an_argument_of_a_call() {
        let statements = parse("DoThing([1, 2])").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else { panic!("a call") };
        assert!(matches!(&arguments[0], Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn a_call_statement_may_take_an_array() {
        // The space is the reason: `DoThing[1, 2]` is the item of the
        // value `DoThing`, and `DoThing [1, 2]` is a call whose argument
        // is the array.
        let statements = parse("f(a) {\n}\nf [1, 2]").expect("parses");
        let Node::Call { arguments, .. } = &statements[1] else { panic!("a call") };
        assert!(matches!(&arguments[0], Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn a_bracket_without_a_space_is_the_item_of_the_value() {
        let statements = parse("x[1] := 2").expect("parses");
        let Node::AssignIndex { index, .. } = &statements[0] else { panic!("an index") };
        assert!(matches!(index.as_ref(), Node::Literal { value: LiteralValue::Number(number), .. } if *number == 1.0));
    }

    #[test]
    fn a_bracket_that_is_never_closed_is_an_error() {
        assert!(parse("x := [1, 2").is_err());
    }

    #[test]
    fn a_for_loop_walks_what_the_object_enumerates() {
        let statements = parse("for x in items {\n  total += x\n}").expect("parses");
        let Node::For { variables, source, body, .. } = &statements[0] else { panic!("a for") };
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0], "x");
        assert!(matches!(source.as_ref(), Node::Var { name, .. } if name == "items"));
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn a_for_loop_may_take_two_names() {
        let statements = parse("for key, value in items {\n}").expect("parses");
        let Node::For { variables, .. } = &statements[0] else { panic!("a for") };
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0], "key");
        assert_eq!(variables[1], "value");
    }

    #[test]
    fn a_for_loop_may_leave_the_first_name_out() {
        // The name that is left out is the empty string.
        let statements = parse("for , value in map {\n}").expect("parses");
        let Node::For { variables, .. } = &statements[0] else { panic!("a for") };
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0], "");
        assert_eq!(variables[1], "value");
    }

    #[test]
    fn a_for_loop_may_take_no_name_at_all() {
        let statements = parse("for in map {\n}").expect("parses");
        let Node::For { variables, source, .. } = &statements[0] else { panic!("a for") };
        assert!(variables.is_empty());
        assert!(matches!(source.as_ref(), Node::Var { name, .. } if name == "map"));
    }

    #[test]
    fn the_names_of_a_for_loop_may_be_written_between_parentheses() {
        let statements = parse("for (x in items) {\n}").expect("parses");
        let Node::For { variables, source, body, .. } = &statements[0] else { panic!("a for") };
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0], "x");
        assert!(matches!(source.as_ref(), Node::Var { name, .. } if name == "items"));
        assert_eq!(body.len(), 0);
    }

    #[test]
    fn a_for_loop_may_be_written_with_a_double_colon() {
        // The `::` that may introduce a block is accepted the way it is
        // accepted by the other loops.
        let statements = parse("for x in items :: {\n}").expect("parses");
        let Node::For { variables, body, .. } = &statements[0] else { panic!("a for") };
        assert_eq!(variables[0], "x");
        assert_eq!(body.len(), 0);
    }

    #[test]
    fn a_for_without_the_in_is_an_error() {
        assert!(parse("for x {\n}").is_err());
    }

    #[test]
    fn a_for_whose_variable_is_not_a_name_is_an_error() {
        assert!(parse("for 1 in items {\n}").is_err());
    }
}
