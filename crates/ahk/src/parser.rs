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
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::ast::{
    AccessBody, AccessDef, AssignOp, BinaryOp, Branch, Case, CastType, Catch, DeclareKind, Declared,
    Declarations, LiteralValue, LoopKind, MethodDef, Node, Parameter, PropertyDef, UnaryOp,
};
use crate::lexer::{SyntaxError, Token, TokenKind};

/// The shape of a `super`, the way v2 reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TheShapeOfASuper {
    /// The keyword alone: `super`.
    Bare,
    /// The method of the base: `super.m` and `super.m()`.
    Member,
    /// The item of the base: `super["x"]`.
    Item,
    /// The call of the base: `super()`.
    Call,
}

/// The shape of a `super`, and the text of it.
///
/// The text is the source as it stands: `super.m(1, 2)` is written back
/// with the space after the comma.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a statement of v2 is a
/// call, and the member that is not called is written with the parentheses
/// of a call -- "super.x()". In an expression the text is the source, and
/// the parentheses are not there -- "super.x".
fn the_shape_of_a_super(parser: &Parser, start: usize, as_a_statement: bool) -> (TheShapeOfASuper, String) {
    let tokens = &parser.tokens;
    let mut end = start + 1;
    let mut shape = TheShapeOfASuper::Bare;
    let mut called = false;
    if let Some(token) = tokens.get(end) {
        if token.kind == TokenKind::Op && token.value == "." {
            // The member: `super.m` and `super.m()`. The dot reaches into
            // the value when there is no space after it.
            let name = tokens.get(end + 1);
            let is_member = name.map(|token| !token.spaced).unwrap_or(false)
                && name.map(|token| token.kind == TokenKind::Name || token.kind == TokenKind::Keyword).unwrap_or(false);
            if is_member {
                end += 2;
                shape = TheShapeOfASuper::Member;
                // The call: the parentheses and what is inside them.
                if let Some(open) = tokens.get(end) {
                    if open.kind == TokenKind::Op && open.value == "(" {
                        end = the_end_of_a_group(tokens, end);
                        called = true;
                    }
                }
            }
        } else if token.kind == TokenKind::Op && token.value == "[" {
            // The item: `super["x"]`.
            end = the_end_of_a_group(tokens, end);
            shape = TheShapeOfASuper::Item;
        } else if token.kind == TokenKind::Op && token.value == "(" {
            // The call: `super()`.
            end = the_end_of_a_group(tokens, end);
            shape = TheShapeOfASuper::Call;
            called = true;
        }
    }
    let mut text = the_source_of_the_tokens(parser, start, end);
    // A member that is not called is a function, and a statement of v2 is a
    // call: the text of it carries the parentheses.
    if as_a_statement && shape == TheShapeOfASuper::Member && !called {
        text.push_str("()");
    }
    (shape, text)
}

/// The index just after the group that opens at `start`.
///
/// The brackets nest, and a string is one token, so the brackets of a
/// string do not count.
fn the_end_of_a_group(tokens: &[Token], start: usize) -> usize {
    let mut depth = 0usize;
    let mut index = start;
    while index < tokens.len() {
        let token = &tokens[index];
        if token.kind == TokenKind::Op {
            match token.value.as_str() {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return index + 1;
                    }
                }
                _ => {}
            }
        }
        index += 1;
    }
    tokens.len()
}

/// The source of a range of tokens, the way the script wrote it.
///
/// The tokens know whether a space came before them, and that is what
/// makes the text: `super.m(1, 2)` keeps the space after the comma.
fn the_source_of_the_tokens(parser: &Parser, start: usize, end: usize) -> String {
    let mut text = String::new();
    for token in parser.tokens.get(start..end).unwrap_or(&[]) {
        if !text.is_empty() && token.spaced {
            text.push(' ');
        }
        text.push_str(&the_source_of_a_token(token));
    }
    text
}

/// The source of a token, the way the script wrote it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the text of a token
/// keeps the quote that was written, and the name of a token that is
/// refused is the text of it.
fn the_source_of_a_token(token: &Token) -> String {
    if token.kind != TokenKind::String {
        return token.value.clone();
    }
    let quote = token.quote.unwrap_or('"');
    let mut text = String::new();
    text.push(quote);
    text.push_str(&token.value);
    text.push(quote);
    text
}

/// Whether the value may be written to.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the increment of a
/// value that cannot be written is refused at the load, and it is an
/// "Invalid assignment.".
fn the_value_can_be_written(node: &Node) -> bool {
    matches!(node, Node::Var { .. } | Node::Property { .. } | Node::Index { .. })
}

/// The source of a token and of what comes after it, in its line.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the value that came
/// too close is named, and the rest of the line is named with it. The
/// newline that ends the line is not part of it: the block that the error
/// printer writes ends with a newline of its own, and the text that the
/// error names is the block without it.
fn the_source_of_the_rest_of_the_line(parser: &Parser, start: usize) -> String {
    let mut end = start;
    while end < parser.tokens.len()
        && parser.tokens[end].line == parser.tokens[start].line
        && parser.tokens[end].kind != TokenKind::Newline
    {
        end += 1;
    }
    the_source_of_the_tokens(parser, start, end)
}

/// The source of the statement that the parser is in.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the bracket that is not
/// there is named with the statement that holds it, and the lines of the
/// statement are written in one.
fn the_source_of_the_statement(parser: &Parser) -> String {
    let line = the_line_of_the_parser(parser);
    let mut end = parser.statement_start;
    while end < parser.tokens.len() && parser.tokens[end].line <= line {
        end += 1;
    }
    the_source_of_a_definition(parser, parser.statement_start, end)
}

/// The message of a failure of a command-style call.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the command of v2 is a
/// call whose parentheses are not written, and the text that names the
/// operator carries the one that closes it.
fn the_message_of_a_command(mut error: SyntaxError) -> SyntaxError {
    // The bracket that is not there is not a failure of the parameters, and
    // it is not named with the parenthesis of the call.
    if !error.message.starts_with("Missing \"") && error.message.contains("\n     Specifically: ") {
        error.message.push(')');
    }
    error
}

/// The source of a definition, the way v2 writes it.
///
/// A definition may be written in more than one line, and v2 writes the
/// source of it in one: the newline is a space.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the source of
/// "static class Inner { }" is written with a space.
fn the_source_of_a_definition(parser: &Parser, start: usize, end: usize) -> String {
    let mut text = String::new();
    let mut after_a_newline = false;
    for token in parser.tokens.get(start..end).unwrap_or(&[]) {
        if token.kind == TokenKind::Newline {
            if !text.is_empty() {
                after_a_newline = true;
            }
            continue;
        }
        if !text.is_empty() && (token.spaced || after_a_newline) {
            text.push(' ');
        }
        after_a_newline = false;
        text.push_str(&the_source_of_a_token(token));
    }
    text
}

/// The source of a line of the script.
///
/// v2 shows the line when it refuses a name or a bracket, and the line is
/// the one that the token is on.
fn the_source_of_the_line(parser: &Parser, line: u32) -> String {
    let start = parser.tokens.iter().position(|token| token.line == line).unwrap_or(0);
    let mut end = start;
    while end < parser.tokens.len() && parser.tokens[end].line == line {
        end += 1;
    }
    the_source_of_the_tokens(parser, start, end).trim_end().to_string()
}

/// The line that the parser is on: the one of the last token it read.
fn the_line_of_the_parser(parser: &Parser) -> u32 {
    parser
        .tokens
        .iter()
        .rev()
        .find(|token| token.kind != TokenKind::Eof)
        .map(|token| token.line)
        .unwrap_or(1)
}

/// Whether the token is the operator of two values.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a line that begins
/// with one of them continues the line before it.
fn the_token_is_an_operator(token: &Token) -> bool {
    if token.kind == TokenKind::Op {
        return matches!(
            token.value.as_str(),
            "**" | "*" | "/" | "//" | "+" | "-" | "<<" | ">>" | ">>>" | "&" | "^" | "|"
                | "~=" | ">" | ">=" | "<" | "<=" | "=" | "==" | "!=" | "!==" | "&&" | "||" | "??"
        );
    }
    if token.kind == TokenKind::Keyword {
        return matches!(token.value.to_lowercase().as_str(), "and" | "or" | "is");
    }
    false
}

/// Whether the number is a float: it has a point in it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the name of a property
/// may be a number, and the number that is a float is not a name.
fn the_number_is_a_float(token: &Token) -> bool {
    token.value.contains('.')
}

/// The name that v2 reads when it begins with a number.
///
/// The name stops at the point, because a point is not part of a name.
fn the_name_that_begins_with_a_number(token: &Token) -> &str {
    token.value.split('.').next().unwrap_or(&token.value)
}

/// The text of a declaration, the way v2 writes it when it refuses.
fn the_text_of_a_declaration(kind: DeclareKind, names: &[Declared]) -> String {
    let mut text = kind.word().to_string();
    for declared in names {
        text.push(' ');
        text.push_str(&declared.text);
    }
    text
}

/// The labels of a scope, the way the script wrote them.
///
/// The labels of the blocks of the scope are the labels of the scope, and
/// the body of a function that is written inside the scope belongs to that
/// function and not to the scope around it.
fn the_labels_of_a_scope(body: &[Node]) -> Vec<(String, u32)> {
    let mut labels = Vec::new();
    collect_the_labels_of_a_scope(body, &mut labels);
    labels
}

/// The labels of the blocks of a scope.
fn collect_the_labels_of_a_scope(body: &[Node], labels: &mut Vec<(String, u32)>) {
    for statement in body {
        match statement {
            Node::Label { name, line } => labels.push((name.clone(), *line)),
            Node::If { branches, otherwise, .. } => {
                for branch in branches {
                    collect_the_labels_of_a_scope(&branch.body, labels);
                }
                collect_the_labels_of_a_scope(otherwise, labels);
            }
            Node::Switch { cases, .. } => {
                for case in cases {
                    collect_the_labels_of_a_scope(&case.body, labels);
                }
            }
            Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
                collect_the_labels_of_a_scope(body, labels);
            }
            Node::Try { body, catches, otherwise, finally, .. } => {
                collect_the_labels_of_a_scope(body, labels);
                for catch in catches {
                    collect_the_labels_of_a_scope(&catch.body, labels);
                }
                collect_the_labels_of_a_scope(otherwise, labels);
                collect_the_labels_of_a_scope(finally, labels);
            }
            _ => {}
        }
    }
}

/// The name of a label, when the tokens before the colon are one.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the name of a label
/// is of letters, of digits and of the underscore, and it may begin with
/// a digit: `1abc:` is a label. A name with a dash or with a dot is not
/// a label, and a name with a space is not one either.
fn the_name_of_a_label(tokens: &[Token]) -> Option<String> {
    let mut name = String::new();
    for (index, token) in tokens.iter().enumerate() {
        if index > 0 && token.spaced {
            return None;
        }
        match token.kind {
            TokenKind::Name | TokenKind::Number => {}
            _ => return None,
        }
        if !token.value.chars().all(|letter| letter.is_ascii_alphanumeric() || letter == '_') {
            return None;
        }
        name.push_str(&token.value);
    }
    if name.is_empty() {
        return None;
    }
    Some(name)
}

/// The text of the tokens of a range, the way the script wrote them.
fn the_text_of_the_tokens(parser: &Parser, start: usize, end: usize) -> String {
    let mut text = String::new();
    for token in parser.tokens.get(start..end).unwrap_or(&[]) {
        if !text.is_empty() {
            text.push(' ');
        }
        if token.kind == TokenKind::String {
            text.push('"');
            text.push_str(&token.value);
            text.push('"');
        } else {
            text.push_str(&token.value);
        }
    }
    text
}

/// The declarations of a body, and the bare `global` that is not the mode.
///
/// The mode of a function is the bare `global` of its first line, and v2
/// refuses any other one. The names of the declarations are read when the
/// script is read: a declaration inside a block takes effect for the whole
/// function.
fn the_declarations_of_a_body(body: &[Node]) -> Result<Declarations, SyntaxError> {
    let mut declarations = Declarations::default();
    let mut first = true;
    for statement in body {
        // The mode is the bare `global` of the first line.
        if first {
            first = false;
            if let Node::Declare { scope: DeclareKind::Global, names, .. } = statement {
                if names.is_empty() {
                    declarations.assume_global = true;
                    continue;
                }
            }
        }
        the_declarations_of_a_statement(statement, &mut declarations)?;
    }
    Ok(declarations)
}

/// The declarations of a statement, and the bare `global` that is refused.
fn the_declarations_of_a_statement(
    statement: &Node,
    declarations: &mut Declarations,
) -> Result<(), SyntaxError> {
    match statement {
        Node::Declare { scope: kind, names, line } => {
            if *kind == DeclareKind::Global && names.is_empty() {
                // A bare `global` that is not the mode of the function.
                return Err(SyntaxError {
                    message: "Unexpected declaration.\n     Specifically: global".into(),
                    line: *line,
                    column: 1,
                });
            }
            for declared in names {
                match kind {
                    DeclareKind::Global => {
                        declarations.globals.insert(declared.name.to_lowercase());
                    }
                    DeclareKind::Local => {
                        declarations.locals.insert(declared.name.to_lowercase());
                    }
                    // The static lives in the storage of the function, and
                    // the name is one of them for the whole function.
                    DeclareKind::Static => {
                        declarations.statics.insert(declared.name.to_lowercase());
                    }
                }
            }
        }
        Node::If { branches, otherwise, .. } => {
            for branch in branches {
                the_declarations_of_a_body_of(&branch.body, declarations)?;
            }
            the_declarations_of_a_body_of(otherwise, declarations)?;
        }
        Node::Switch { cases, .. } => {
            for case in cases {
                the_declarations_of_a_body_of(&case.body, declarations)?;
            }
        }
        Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
            the_declarations_of_a_body_of(body, declarations)?;
        }
        Node::Try { body, catches, otherwise, finally, .. } => {
            the_declarations_of_a_body_of(body, declarations)?;
            for catch in catches {
                the_declarations_of_a_body_of(&catch.body, declarations)?;
            }
            the_declarations_of_a_body_of(otherwise, declarations)?;
            the_declarations_of_a_body_of(finally, declarations)?;
        }
        _ => {}
    }
    Ok(())
}

/// The declarations of the statements of a block.
fn the_declarations_of_a_body_of(
    body: &[Node],
    declarations: &mut Declarations,
) -> Result<(), SyntaxError> {
    for statement in body {
        the_declarations_of_a_statement(statement, declarations)?;
    }
    Ok(())
}

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

/// The members of a class: the methods, the properties, the properties
/// that have a getter or a setter, and the classes that it holds.
struct TheMembersOfAClass {
    methods: Vec<MethodDef>,
    properties: Vec<PropertyDef>,
    accessors: Vec<AccessDef>,
    nested: Vec<Node>,
}

/// The recursive descent parser.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    /// The names the script defines: its functions and its classes.
    defined: BTreeSet<String>,
    /// The classes of the script, and the name they were written with.
    ///
    /// The name of a class is a name of the script: it may not be the
    /// output variable of an assignment.
    classes: BTreeMap<String, String>,
    /// The functions of the top level of the script.
    ///
    /// A name may be declared only once, and a function that is written
    /// inside another one is not a name of the script.
    functions: BTreeSet<String>,
    /// The assignments of the script, one list for each function.
    ///
    /// The list of the top level is the first one, and the list of a
    /// function is the one that is pushed when its body is read.
    assignments: Vec<Vec<Assignment>>,
    /// The parameters of the functions that are being parsed.
    ///
    /// A declaration may not name a parameter: v2 says so.
    parameters: Vec<Vec<String>>,
    /// Whether the parser is inside a class.
    ///
    /// The `super` of v2 is only valid inside a class.
    in_class: bool,
    /// Whether the parser is inside a function.
    ///
    /// A function may not contain a class: v2 refuses it at the load.
    in_function: bool,
    /// The operator that may not follow a literal text.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the error is named
    /// when the expression that holds the operator is done, and the text it
    /// names is the operator and the rest of that expression.
    pending_literal: Option<PendingLiteral>,
    /// The expressions that may hold that operator.
    ///
    /// The expression that holds it is the one of the statement, and that
    /// one is the one that names the error.
    expressions: Vec<usize>,
    /// The place where the statement that is being read begins.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a token that
    /// cannot be there names the statement it is in, and the text is the
    /// statement and the token.
    statement_start: usize,
    /// The base of every class of the script, and the place it was written.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the base of a
    /// class is a class of the script or a class that a script can read,
    /// and the one that is not there is refused at the load.
    bases: Vec<(String, String, u32, u32)>,
}

/// An assignment of the script, and the place it was written.
struct Assignment {
    /// The name of the variable that is written.
    name: String,
    /// The operator of the assignment.
    op: AssignOp,
    /// The line of the assignment.
    line: u32,
    /// The column of the name.
    column: u32,
}

/// The operator that may not follow a literal text.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the error that names
/// it is named when the expression that holds it is done, and the text that
/// it names is the operator and the rest of that expression. The line it
/// names is the line of that expression.
struct PendingLiteral {
    /// The place of the operator in the tokens of the parser.
    at: usize,
    /// The line of the expression that holds the operator.
    line: u32,
    /// The column of the expression that holds the operator.
    column: u32,
}

impl Parser {
    /// A parser over the tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            defined: BTreeSet::new(),
            classes: BTreeMap::new(),
            functions: BTreeSet::new(),
            assignments: vec![Vec::new()],
            parameters: Vec::new(),
            in_class: false,
            in_function: false,
            pending_literal: None,
            expressions: Vec::new(),
            statement_start: 0,
            bases: Vec::new(),
        }
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
    /// The space that comes after the dot is what tells a member from a
    /// concatenation: `o.x` reaches into the value, and `o. x` joins two
    /// values. The space that comes before the dot does not matter: `o .x`
    /// reaches into the value too.
    fn at_member_dot(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && token.value == "." && !self.the_space_after_is_there()
    }

    /// Whether a space comes after the token that is being looked at.
    ///
    /// The space belongs to the token that comes next, and that is what
    /// tells a member from a concatenation.
    fn the_space_after_is_there(&self) -> bool {
        self.tokens.get(self.position + 1).map(|token| token.spaced).unwrap_or(false)
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

    /// Whether the token that comes after the one that is there makes a class
    /// declaration.
    ///
    /// A name is the name of the class, and the two things that v2 refuses
    /// by name -- a reserved word and a number -- are refused as the name of
    /// a class, so the declaration is what the parser reads.
    fn a_class_declaration_comes_next(&self) -> bool {
        match self.tokens.get(self.position + 1) {
            Some(token) => matches!(token.kind, TokenKind::Name | TokenKind::Keyword | TokenKind::Number),
            None => false,
        }
    }

    fn at_op(&self, operators: &[&str]) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && operators.contains(&token.value.as_str())
    }

    fn expect_op(&mut self, operator: &str) -> Result<Token, SyntaxError> {
        let token = self.next();
        if token.kind != TokenKind::Op || token.value != operator {
            // Measured on the binary of v2.0.19 on 2026-10-31: the bracket
            // that is not there is named, and the statement that holds it is
            // named with it.
            return Err(SyntaxError {
                message: format!(
                    "Missing {operator:?}\n     Specifically: {}",
                    the_source_of_the_statement(self)
                ),
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

    /// The expression of a statement begins.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the error of the
    /// operator that may not follow a literal text is named when the
    /// expression that holds the operator is done, and the text it names
    /// is the operator and the rest of that expression.
    fn the_expression_begins(&mut self) {
        self.expressions.push(self.position);
    }

    /// The expression of a statement is done.
    ///
    /// The error is named here, when this expression is the one that holds
    /// the operator. The command of v2 is a call whose parentheses are not
    /// written, and the text that names the operator carries the one that
    /// closes it.
    fn the_expression_is_done(&mut self, end: usize, closed: bool) -> Result<(), SyntaxError> {
        let start = self.expressions.pop().unwrap_or(0);
        let Some(pending) = self.pending_literal.take() else {
            return Ok(());
        };
        if pending.at < start || pending.at >= end {
            // The operator is not in this expression: it is in another one,
            // and that one names the error.
            self.pending_literal = Some(pending);
            return Ok(());
        }
        Err(self.the_literal_error(&pending, end, closed))
    }

    /// The expression of a statement, with the error that it may carry.
    fn the_expression_of_a_statement(&mut self) -> Result<Node, SyntaxError> {
        self.the_expression_begins();
        let node = self.parse_expression()?;
        self.the_expression_is_done(self.position, false)?;
        Ok(node)
    }

    /// The operator that may not follow a literal text.
    ///
    /// The line and the column are the ones of the expression that holds
    /// the operator: v2 names the expression and not the operator.
    fn the_pending_literal(&self, at: usize) -> PendingLiteral {
        let start = self.expressions.last().copied().unwrap_or(at);
        let token = self.tokens.get(start);
        PendingLiteral {
            at,
            line: token.map(|token| token.line).unwrap_or(0),
            column: token.map(|token| token.column).unwrap_or(1),
        }
    }

    /// The error of the operator that may not follow a literal text.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text that is
    /// named is the operator and the rest of the expression that holds it.
    fn the_literal_error(&self, pending: &PendingLiteral, end: usize, closed: bool) -> SyntaxError {
        let mut text = the_source_of_the_tokens(self, pending.at, end);
        if closed {
            text.push(')');
        }
        SyntaxError {
            message: format!("Unexpected operator following literal string.\n     Specifically: {text}"),
            line: pending.line,
            column: pending.column,
        }
    }

    // -- statements ----------------------------------------------------------------

    /// The statements of a whole script.
    pub fn parse_script(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut statements = Vec::new();
        self.defined.clear();
        self.classes.clear();
        self.functions.clear();
        self.assignments.clear();
        self.assignments.push(Vec::new());
        self.skip_newlines();
        while self.peek().kind != TokenKind::Eof {
            statements.extend(self.parse_statement()?);
            self.skip_newlines();
        }
        // The labels of the script are the scope of the script, and a label
        // that is written twice in it is refused.
        self.check_the_labels(&statements)?;
        // The base of a class is a class, and the one that is not there is
        // refused.
        self.check_the_bases()?;
        // The assignments of the top level live in the scope of the script,
        // and the class that one of them names may be declared further down.
        let assignments = core::mem::take(self.assignments.first_mut().expect("the top level"));
        self.check_the_assignments(assignments)?;
        Ok(statements)
    }

    /// The refusal of the classes whose base is not there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the base of a
    /// class is a class of the script or a class that a script can read,
    /// and the one that is not there is named. The base may be declared
    /// after the class that extends it, so the check is at the end of the
    /// script.
    ///
    /// The rule lives here, and the v1 dialect may let the base of a class
    /// be a name that is not there.
    fn check_the_bases(&self) -> Result<(), SyntaxError> {
        for (_, base, line, column) in &self.bases {
            if self.classes.contains_key(&base.to_lowercase()) {
                continue;
            }
            if crate::classes::is_a_name_of_a_class(base) {
                continue;
            }
            return Err(SyntaxError {
                message: format!("Unknown class.\n     Specifically: {base}"),
                line: *line,
                column: *column,
            });
        }
        Ok(())
    }

    /// The refusal of the assignments whose output variable is a class.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of a class
    /// is a name of the script, and it may not be the output variable of an
    /// assignment. The plain assignment says that the class cannot be an
    /// output variable, and the compound ones say that it cannot be
    /// assigned a value.
    fn check_the_assignments(&self, assignments: Vec<Assignment>) -> Result<(), SyntaxError> {
        for assignment in assignments {
            let key = assignment.name.to_lowercase();
            let Some(class) = self.the_name_of_the_class(&key) else { continue };
            let message = match assignment.op {
                AssignOp::Set => format!(
                    "This Class cannot be used as an output variable.\n     Specifically: {class}"
                ),
                _ => format!("This Class cannot be assigned a value.\n     Specifically: {class}"),
            };
            return Err(SyntaxError { message, line: assignment.line, column: assignment.column });
        }
        Ok(())
    }

    /// The name of the class of that name, the way v2 spells it.
    ///
    /// The classes of the script keep the name they were written with, and
    /// the classes of the runtime have a spelling of their own.
    fn the_name_of_the_class(&self, key: &str) -> Option<String> {
        if let Some(name) = self.classes.get(key) {
            return Some(name.clone());
        }
        if crate::classes::is_type(key) {
            return Some(crate::classes::the_spelling_of(key));
        }
        None
    }

    /// The refusal of a name that is declared twice.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the declaration
    /// that comes second is refused, and the message names the kind of the
    /// one that is already there.
    fn the_name_is_declared_twice(
        &self,
        name: &str,
        is_a_class: bool,
        line: u32,
        column: u32,
    ) -> Result<(), SyntaxError> {
        let key = name.to_lowercase();
        let message = if is_a_class {
            if self.classes.contains_key(&key) {
                Some("This class declaration conflicts with an existing Class.")
            } else if self.functions.contains(&key) {
                Some("This class declaration conflicts with an existing Func.")
            } else {
                None
            }
        } else if self.classes.contains_key(&key) {
            Some("This function declaration conflicts with an existing Class.")
        } else if self.functions.contains(&key) {
            Some("This function declaration conflicts with an existing Func.")
        } else {
            None
        };
        let Some(message) = message else { return Ok(()) };
        Err(SyntaxError {
            message: format!("{message}\n     Specifically: {name}"),
            line,
            column,
        })
    }

    /// A block: the statements between `{` and `}`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the body of a
    /// block begins on the next line, and the brace that closes it is at
    /// the start of its line. The word tells the block what it is, and
    /// every construct says in its own words that the brace is not where
    /// it belongs.
    ///
    /// The body may also be one statement, and the braces may be absent:
    /// `if (1)` and then `x := 1` on the next line is an `if` with a body.
    /// The line is the line of the construct, and the messages that name
    /// it need it.
    fn parse_block(&mut self, kind: &str, line: u32) -> Result<Vec<Node>, SyntaxError> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the body of the
        // `if`, the `while`, the `loop`, the `for` and the `catch` is on
        // the next line, and the `try`, the `else` and the `finally` take
        // it on the line of the construct as well.
        if !self.at_op(&["{"])
            && !self.the_body_begins_on_the_next_line()
            && Self::the_body_must_begin_on_the_next_line(kind)
        {
            // The body is on the line of the construct, and the construct
            // says in its own words that it is not there.
            return Err(self.error(self.the_message_of_the_body(kind)));
        }
        // The brace may be on a line of its own.
        self.skip_newlines();
        if !self.at_op(&["{"]) {
            // The body is one statement, and the braces are not there.
            return self.the_body_without_braces(kind, line);
        }
        self.expect_op("{")?;
        if !self.the_body_begins_on_the_next_line() {
            return Err(self.error(self.the_message_of_the_opening_brace(kind, line)));
        }
        let mut body = Vec::new();
        self.skip_newlines();
        // The statement that is being read is the one of the body, and
        // the block gives its place back when it is done.
        let statement_start = self.statement_start;
        loop {
            let token = self.peek();
            if token.kind == TokenKind::Eof {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // brace that is not closed is named, and nothing else: the
                // line of the script is not named with it.
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                self.the_closing_brace_is_refused()?;
                self.position += 1;
                self.statement_start = statement_start;
                return Ok(body);
            }
            body.extend(self.parse_statement()?);
            self.skip_newlines();
        }
    }

    /// The body of a block that has no braces.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the body is one
    /// statement, and the function and the method need the braces. The
    /// `if`, the `while`, the `loop`, the `for` and the `try` need a
    /// body, and the `catch`, the `else` and the `finally` do not.
    fn the_body_without_braces(&mut self, kind: &str, line: u32) -> Result<Vec<Node>, SyntaxError> {
        if matches!(kind, "function" | "method") {
            // The function and the method need the braces.
            return Err(self.error(self.the_message_of_the_opening_brace(kind, line)));
        }
        if self.at_end_of_statement() {
            // The body is not there.
            if Self::the_block_needs_a_body(kind) {
                return Err(self.error(self.the_message_of_the_body(kind)));
            }
            return Ok(Vec::new());
        }
        self.parse_statement()
    }

    /// The message of v2 when the body of a block is not where it belongs.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the construct names
    /// its own word: "Unexpected \"If\"", "Unexpected \"While\"",
    /// "Unexpected \"Loop\"", "Unexpected \"For\"", "Unexpected
    /// \"Catch\"" and "Unexpected \"Try\"".
    fn the_message_of_the_body(&self, kind: &str) -> String {
        format!("Unexpected {kind:?}")
    }

    /// Whether the body of a block must begin on the next line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the `if`, the
    /// `while`, the `loop`, the `for` and the `catch` want the body on the
    /// next line, and the `try`, the `else` and the `finally` take it on
    /// the line of the construct as well.
    fn the_body_must_begin_on_the_next_line(kind: &str) -> bool {
        matches!(kind, "If" | "While" | "Loop" | "For" | "Catch")
    }

    /// Whether the block needs a body.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the `if`, the
    /// `while`, the `loop`, the `for` and the `try` need one, and the
    /// `catch`, the `else` and the `finally` do not.
    fn the_block_needs_a_body(kind: &str) -> bool {
        matches!(kind, "If" | "While" | "Loop" | "For" | "Try")
    }

    /// Whether the body of a block begins on the next line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the brace that
    /// opens a block is at the end of its line, and the body begins on the
    /// next one. A comment is not a statement, and it does not count.
    fn the_body_begins_on_the_next_line(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof)
    }

    /// Whether the brace is at the start of its line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the brace that
    /// closes a block is at the start of its line, and a brace that is not
    /// names the line of the script.
    fn the_brace_is_at_the_start_of_its_line(&self) -> bool {
        match self.position.checked_sub(1).and_then(|at| self.tokens.get(at)) {
            Some(token) => token.kind == TokenKind::Newline,
            None => true,
        }
    }

    /// The refusal of a brace that is not at the start of its line.
    fn the_closing_brace_is_refused(&self) -> Result<(), SyntaxError> {
        if self.the_brace_is_at_the_start_of_its_line() {
            return Ok(());
        }
        let token = self.peek();
        Err(self.error(format!(
            "Unexpected \"}}\"\n     Specifically: {}",
            the_source_of_the_line(self, token.line)
        )))
    }

    /// The message of v2 when the brace that opens a block is not where it
    /// belongs.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: every construct says
    /// so in its own words. The function and the `elseif` name the brace,
    /// the class says that the brace is missing, the method says that the
    /// definition is not valid, the statements name their word, and the
    /// blocks that end with a brace say that it is missing.
    fn the_message_of_the_opening_brace(&self, kind: &str, line: u32) -> String {
        match kind {
            "function" | "elseif" => "Unexpected \"{\"".to_string(),
            "class" => format!(
                "Missing \"{{\"\n     Specifically: {}",
                the_source_of_the_line(self, line)
            ),
            "method" => format!(
                "Not a valid method, class or property definition.\n     Specifically: {}",
                the_source_of_the_line(self, line)
            ),
            "If" | "While" | "Loop" | "For" | "Switch" => format!("Unexpected {kind:?}"),
            _ => "Missing \"}\"".to_string(),
        }
    }

    fn parse_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        // The statement begins here, and a token that cannot be there
        // names it.
        self.statement_start = self.position;
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
                "class" => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // word `class` is the keyword of a class declaration and
                    // the name of a class type of the dialect, and the two
                    // are told apart by what follows: a name makes a
                    // declaration -- `class Foo { }` and `Class Foo { }` are
                    // both of them -- and anything else makes a call of the
                    // class. The call of the `Class` with an argument is
                    // refused at the run, and not at the load.
                    if self.a_class_declaration_comes_next() {
                        return Ok(vec![self.parse_class()?]);
                    }
                    let keyword = self.next();
                    let mut arguments = Vec::new();
                    if self.at_op(&["("]) && !self.peek().spaced {
                        self.next();
                        arguments = self.parse_arguments()?;
                        self.expect_op(")")?;
                    }
                    self.end_of_statement()?;
                    return Ok(vec![Node::Call {
                        name: keyword.value.to_lowercase(),
                        arguments,
                        line: keyword.line,
                    }]);
                }
                "else" | "catch" | "finally" => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // `else`, the `catch` and the `finally` are statements of
                    // their own, and the body of them never runs.
                    let keyword = self.next();
                    let kind = if keyword.value.eq_ignore_ascii_case("catch") { "Catch" } else { "else" };
                    let _body = self.parse_block(kind, keyword.line)?;
                    return Ok(Vec::new());
                }
                "try" => return Ok(vec![self.parse_try()?]),
                "goto" => return Ok(vec![self.parse_goto()?]),
                "global" => return Ok(vec![self.parse_declaration(DeclareKind::Global)?]),
                "local" => return Ok(vec![self.parse_declaration(DeclareKind::Local)?]),
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // `static` is the third kind of declaration of v2, and it
                // is a statement of its own.
                "static" => return Ok(vec![self.parse_declaration(DeclareKind::Static)?]),
                "throw" => return Ok(vec![self.parse_throw()?]),
                "super" => return self.parse_super_statement(),
                "return" => return Ok(vec![self.parse_return()?]),
                "exitapp" => return Ok(vec![self.parse_exitapp()?]),
                _ => {}
            }
        }
        // The label of the line comes before anything else: a line that is
        // a label is a label, and not an expression.
        if let Some(label) = self.parse_label()? {
            return Ok(vec![label]);
        }
        if token.kind == TokenKind::Name {
            // Measured on the binary of v2.0.19 on 2026-10-31: the operator
            // that comes after the name is the increment, and it may be a
            // statement of its own.
            if self.the_postfix_increment_is_next() {
                let value = self.parse_expression()?;
                self.end_of_statement()?;
                return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
            }
            return self.parse_name_statement();
        }
        if token.kind == TokenKind::Op && token.value == "(" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a line that
            // begins with a parenthesis is an expression, and the value it
            // leaves behind is not used.
            let value = self.parse_expression()?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
        }
        if token.kind == TokenKind::Op && matches!(token.value.as_str(), "++" | "--") {
            // Measured on the binary of v2.0.19 on 2026-10-31: the increment
            // may be a statement of its own.
            let value = self.parse_expression()?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the number that
        // is the left side of an assignment is refused at the parse, and the
        // text it names is the number. The number that carries a point is
        // the exception: the assignment is the one that is refused, and the
        // text it names is the operator and the value. The compound
        // operators refuse the number the same way, and the point at the
        // end of the number is the dot of a member that is not there.
        if token.kind == TokenKind::Number {
            if let Some(operator) = self.the_assignment_operator_after_the_number() {
                if token.value.ends_with('.') {
                    return Err(self.error(format!(
                        "This line does not contain a recognized action.\n     Specifically: {}",
                        the_source_of_the_line(self, token.line)
                    )));
                }
                if operator == ":=" && !token.value.contains('.') {
                    return Err(self.error(format!("Syntax error.\n     Specifically: {}", token.value)));
                }
                // The assignment is parsed, and the refusal of its left side
                // is the one of an assignment that cannot be written.
                let value = self.parse_expression()?;
                self.end_of_statement()?;
                return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
            }
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the token that
        // is not wanted is named, and the line of the script says where it
        // was. The brace that closes a block is the one that has no line:
        // v2 says that it is unexpected, and nothing more.
        //
        // A literal at the start of a statement is a line that holds no
        // action, and the text that is named is the line itself.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the array at
        // the start of a statement is a line that holds no action, and the
        // text that is named is the line itself.
        let message = if token.kind == TokenKind::Number || token.kind == TokenKind::String {
            format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )
        } else if token.kind == TokenKind::Op && token.value == "[" {
            // Measured on the binary of v2.0.19 on 2026-10-31: the array at
            // the start of a statement is a line that holds no action, and
            // the text that is named is the line itself.
            format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )
        } else if token.kind == TokenKind::Op && token.value == "{" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a brace at
            // the start of a statement is a line that holds no action, and
            // the text that is named is the statement itself.
            let mut end = self.position;
            while end < self.tokens.len() && self.tokens[end].line == token.line {
                end += 1;
            }
            format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                the_source_of_the_tokens(self, self.position, end).trim_end()
            )
        } else if token.kind == TokenKind::Op && token.value == "}" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a brace that
            // is not at the start of its line names the line of the script,
            // and a brace that is alone on its line names nothing.
            if self.the_brace_is_at_the_start_of_its_line() {
                "Unexpected \"}\"".to_string()
            } else {
                format!(
                    "Unexpected \"}}\"\n     Specifically: {}",
                    the_source_of_the_line(self, token.line)
                )
            }
        } else {
            format!(
                "Unexpected {:?}\n     Specifically: {}",
                token.value,
                the_source_of_the_line(self, token.line)
            )
        };
        Err(self.error(message))
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
        // Measured on the binary of v2.0.19 on 2026-10-31: the token that
        // is not wanted is named, and the line of the script says where it
        // was.
        Err(self.error(format!(
            "Unexpected {:?}\n     Specifically: {}",
            token.value,
            the_source_of_the_line(self, token.line)
        )))
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
                // The default of a parameter is an expression, and the
                // expression that holds the operator of a literal text is
                // the list of the parameters.
                self.the_expression_begins();
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                self.the_expression_is_done(self.position, false)?;
                let at_the_top = self.parameters.is_empty();
                self.parameters.push(parameters.iter().map(|parameter| parameter.name.clone()).collect());
                self.assignments.push(Vec::new());
                // A function may not contain a class.
                let was_in_a_function = self.in_function;
                self.in_function = true;
                let body = self.parse_block("function", name_token.line)?;
                self.in_function = was_in_a_function;
                self.check_the_labels(&body)?;
                let assignments = core::mem::take(self.assignments.last_mut().expect("the list of the assignments"));
                self.assignments.pop();
                self.parameters.pop();
                let declarations = the_declarations_of_a_body(&body)?;
                // The assignments of the function that live in the scope of
                // the script are the ones of the names it declares global.
                for assignment in assignments {
                    if declarations.globals.contains(&assignment.name.to_lowercase()) {
                        self.check_the_assignments(vec![assignment])?;
                    }
                }
                if at_the_top {
                    self.the_name_is_declared_twice(&name_token.value, false, name_token.line, name_token.column)?;
                    self.functions.insert(name_token.value.to_lowercase());
                }
                self.defined.insert(name_token.value.to_lowercase());
                return Ok(vec![Node::FunctionDef {
                    name: name_token.value,
                    parameters,
                    body,
                    declarations,
                    line: name_token.line,
                }]);
            }
            self.the_expression_begins();
            let arguments = self.parse_arguments()?;
            self.expect_op(")")?;
            self.the_expression_is_done(self.position, false)?;
            // Measured on the binary of v2.0.19 on 2026-10-31: the operator
            // that comes after the call is the increment, and the value of a
            // call cannot be written to.
            if self.at_op(&["++", "--"]) {
                self.next();
                return Err(self.error(format!(
                    "Invalid assignment.\n     Specifically: {}",
                    the_source_of_the_tokens(self, self.position - 1, self.position)
                )));
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the
            // assignment of a call is refused at the load, and the text
            // of it is named.
            if self.at_an_assignment_operator() {
                return Err(self.the_invalid_assignment(name_token.line)?);
            }
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
            self.the_expression_begins();
            let target = self.parse_suffixes(target, false)?;
            self.the_expression_is_done(self.position, false)?;
            let operator = self.peek().clone();
            if operator.kind == TokenKind::Op && operator.value == "=" {
                return Err(self.the_equal_sign_is_not_an_assignment(name_token.line));
            }
            let is_assignment = operator.kind == TokenKind::Op
                && matches!(
                    operator.value.as_str(),
                    ":=" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<=" | ">>="
                        | ">>>="
                );
            if is_assignment {
                let op = match operator.value.as_str() {
                    ":=" => AssignOp::Set,
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
                let start = self.position;
                let value = self.the_expression_of_a_statement()?;
                let end = self.position;
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
                        // Measured on the binary of v2.0.19 on 2026-10-31: the
                        // assignment of a call is refused at the load, and
                        // the operator and the value are named.
                        return Err(self.error(format!(
                            "Invalid assignment.\n     Specifically: {} {}",
                            operator.value,
                            the_source_of_the_tokens(self, start, end)
                        )));
                    }
                }]);
            }
            self.end_of_statement()?;
            // A name that is a statement is a call: `m.Method` calls the
            // method, and the parentheses may be left out. An item is
            // not a call: `m["a"]` reads it.
            return Ok(vec![match target {
                Node::Property { target, name, line } => Node::MethodCall {
                    target,
                    name,
                    arguments: Vec::new(),
                    line,
                },
                other => other,
            }]);
        }
        // A name that is neither a call nor an assignment is a call
        // statement: `Send "hello"`. The parentheses may be left out, and
        // the rest of the line is the list of the parameters.
        let operator = self.peek().clone();
        if operator.kind == TokenKind::Op && operator.value == "=" {
            return Err(self.the_equal_sign_is_not_an_assignment(name_token.line));
        }
        if !matches!(
            operator.value.as_str(),
            ":=" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>="
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
        // Measured on the binary of v2.0.19 on 2026-10-31: a name that is
        // not a built-in is not refused at the load. It is a function of the
        // script, or a variable of it, and the value of the variable is
        // what the call asks for.
        let name = name_token.value.to_lowercase();
        self.the_expression_begins();
        let mut arguments = Vec::new();
        if !self.at_end_of_statement() {
            loop {
                // `{Enter}` is a key, written without quotes. The built-in
                // that takes it reads the text and knows what it means.
                if self.at_op(&["{"]) {
                    arguments.push(self.parse_brace_spec()?);
                } else {
                    arguments.push(self.parse_expression().map_err(the_message_of_a_command)?);
                }
                if self.at_op(&[","]) {
                    self.position += 1;
                    continue;
                }
                break;
            }
        }
        // The command of v2 is a call whose parentheses are not written, and
        // the text that names the operator carries the one that closes it.
        self.the_expression_is_done(self.position, true)?;
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
            ":=" => AssignOp::Set,
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
        let value = self.the_expression_of_a_statement()?;
        self.end_of_statement()?;
        self.the_assignment_is_recorded(&name_token.value, op, name_token.line, name_token.column);
        Ok(Node::Assign { name: name_token.value, op, value: Box::new(value), line: name_token.line })
    }

    /// The equal sign of v1 is not the equal sign of v2.
    ///
    /// The assignment of v2 is `:=`, and v2 says so when the script writes
    /// the equal sign of v1.
    fn the_equal_sign_is_not_an_assignment(&self, line: u32) -> SyntaxError {
        let start = self.position - 1;
        let mut end = start;
        while end < self.tokens.len() && self.tokens[end].kind != TokenKind::Newline {
            end += 1;
        }
        SyntaxError {
            message: format!(
                "Syntax error. Did you mean to use \":=\"?\n     Specifically: {}",
                the_source_of_the_tokens(self, start, end)
            ),
            line,
            column: self.tokens.get(start).map(|token| token.column).unwrap_or(1),
        }
    }

    /// Whether the parenthesis that is open is the list of the parameters
    /// of an arrow function.
    ///
    /// The arrow function of v2 is `(a, b) => a + b`, and the parenthesis
    /// of an expression is `(a + b)`. The token that follows the closing
    /// parenthesis says which one it is.
    fn at_the_parameters_of_an_arrow_function(&self) -> bool {
        let mut depth = 0usize;
        let mut index = self.position;
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            if token.kind == TokenKind::Op {
                match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => {
                        if depth == 0 {
                            // The parenthesis that is open ends here.
                            return self
                                .tokens
                                .get(index + 1)
                                .map(|next| next.kind == TokenKind::Op && next.value == "=>")
                                .unwrap_or(false);
                        }
                        depth -= 1;
                    }
                    _ => {}
                }
            }
            index += 1;
        }
        false
    }

    /// The arguments of a call, between the parentheses.
    fn parse_arguments(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut arguments = Vec::new();
        if self.at_op(&[")"]) {
            return Ok(arguments);
        }
        loop {
            arguments.push(self.parse_expression()?);
            // The list may be written in more than one line.
            self.skip_newlines();
            if self.at_op(&[","]) {
                self.position += 1;
                // A comma at the end of the list is allowed.
                if self.at_op(&[")"]) {
                    break;
                }
                continue;
            }
            break;
        }
        // The arguments may be written in more than one line.
        self.skip_newlines();
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
            // Measured on the binary of v2.0.19 on 2026-10-31: the rest of
            // the arguments may be a parameter without a name: `f(*)` is a
            // function of v2, and the arguments that it takes go nowhere.
            let mut variadic = false;
            let name = if name_token.kind == TokenKind::Op && name_token.value == "*" {
                variadic = true;
                String::new()
            } else {
                if name_token.kind != TokenKind::Name {
                    // A name that begins with a number is refused by the
                    // lexer, a reserved word is a word that cannot be a
                    // name, and everything else is a name that is missing.
                    let message = match name_token.kind {
                        TokenKind::Number => format!(
                            "This variable name starts with a number, which is not allowed:\n{:?}",
                            the_name_that_begins_with_a_number(&name_token)
                        ),
                        TokenKind::Keyword => format!(
                            "The following reserved word must not be used as a variable name:\n{:?}",
                            name_token.value
                        ),
                        _ => format!(
                            "Missing parameter name.\n     Specifically: {}",
                            the_source_of_the_line(self, name_token.line)
                        ),
                    };
                    return Err(SyntaxError { message, line: name_token.line, column: name_token.column });
                }
                name_token.value
            };
            if !variadic && self.at_op(&["*"]) {
                self.position += 1;
                variadic = true;
            }
            let mut default = None;
            if self.at_op(&[":="]) {
                self.position += 1;
                default = Some(Box::new(self.parse_expression()?));
            }
            parameters.push(Parameter { name, default, variadic, by_ref });
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
                            // The brace may be on a line of its own.
                            let mut after = at + 1;
                            while self
                                .tokens
                                .get(after)
                                .map(|token| token.kind == TokenKind::Newline)
                                .unwrap_or(false)
                            {
                                after += 1;
                            }
                            return self
                                .tokens
                                .get(after)
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
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `if` needs
            // a condition, and it says so in its own words.
            return Err(self.error(format!(
                "\"If\" requires at least 1 parameter.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
        }
        let condition = self.the_expression_of_a_statement()?;
        self.end_of_block()?;
        let body = self.parse_block("If", token.line)?;
        branches.push(Branch { condition, body });
        let mut otherwise = Vec::new();
        loop {
            self.skip_newlines();
            if self.at_keyword(&["elseif"]) {
                let keyword = self.next();
                let condition = self.the_expression_of_a_statement()?;
                self.end_of_block()?;
                let body = self.parse_block("elseif", keyword.line)?;
                branches.push(Branch { condition, body });
                continue;
            }
            if self.at_keyword(&["else"]) {
                let keyword = self.next();
                self.end_of_block()?;
                otherwise = self.parse_block("else", keyword.line)?;
            }
            break;
        }
        Ok(Node::If { branches, otherwise, line: token.line })
    }

    fn parse_switch(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let subject = self.the_expression_of_a_statement()?;
        self.end_of_block()?;
        self.skip_newlines();
        if !self.at_op(&["{"]) {
            // Measured on the binary of v2.0.19 on 2026-10-31: the switch
            // needs the braces, and it names what it found instead.
            let token = self.peek().clone();
            let message = if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("case") {
                "Unexpected \"Case\"".to_string()
            } else {
                "Unexpected \"Switch\"".to_string()
            };
            return Err(SyntaxError { message, line: token.line, column: token.column });
        }
        let open = self.expect_op("{")?;
        if !self.the_body_begins_on_the_next_line() {
            return Err(self.error(self.the_message_of_the_opening_brace("Switch", open.line)));
        }
        let mut cases = Vec::new();
        loop {
            self.skip_newlines();
            if self.at_op(&["}"]) {
                self.the_closing_brace_is_refused()?;
                self.position += 1;
                break;
            }
            if self.at_keyword(&["case"]) {
                self.next();
                let value = self.the_expression_of_a_statement()?;
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
            // Measured on the binary of v2.0.19 on 2026-10-31: v2 looks
            // for the brace that closes the switch before it reads the
            // cases, and a switch that is not closed says so.
            if !self.the_closing_brace_is_there() {
                let token = self.peek().clone();
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            // A statement that is not a case is refused in the words of v2.
            return Err(self.error("Expected Case/Default."));
        }
        Ok(Node::Switch { subject: Box::new(subject), cases, line: token.line })
    }

    /// Whether the brace that closes the block is there.
    ///
    /// The switch of v2 looks for it before it reads the cases, and a
    /// switch that is not closed says so.
    fn the_closing_brace_is_there(&self) -> bool {
        for token in self.tokens.iter().skip(self.position) {
            if token.kind == TokenKind::Eof {
                return false;
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                return true;
            }
        }
        false
    }

    /// The body of a case.
    ///
    /// AutoHotkey writes it in braces, or after a colon, with the statements
    /// under it. Both are accepted here.
    fn parse_case_body(&mut self) -> Result<Vec<Node>, SyntaxError> {
        if self.at_op(&["{"]) {
            // Measured on the binary of v2.0.19 on 2026-10-31: the body of
            // a case is the statements under it, and a brace is not one:
            // "This line does not contain a recognized action."
            let token = self.peek().clone();
            return Err(self.error(format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the case needs
        // the colon, and the line of the script says where it was.
        if !self.at_op(&[":"]) {
            let token = self.peek().clone();
            return Err(SyntaxError {
                message: format!(
                    "Missing \":\"\n     Specifically: {}",
                    the_source_of_the_line(self, token.line)
                ),
                line: token.line,
                column: token.column,
            });
        }
        self.position += 1;
        let mut body = Vec::new();
        self.skip_newlines();
        loop {
            let token = self.peek().clone();
            if token.kind == TokenKind::Eof {
                // The brace that closes the switch is not there.
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
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
            Some(Box::new(self.the_expression_of_a_statement()?))
        };
        self.end_of_statement()?;
        Ok(Node::Return { value, line: token.line })
    }

    fn parse_exitapp(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let code = if self.at_end_of_statement() {
            None
        } else {
            Some(Box::new(self.the_expression_of_a_statement()?))
        };
        self.end_of_statement()?;
        Ok(Node::ExitApp { code, line: token.line })
    }

    /// `try { } catch { } else { } finally { }`.
    fn parse_try(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        self.end_of_block()?;
        let body = self.parse_block("Try", token.line)?;
        let mut catches = Vec::new();
        loop {
            self.skip_newlines();
            if !self.at_keyword(&["catch"]) {
                break;
            }
            catches.push(self.parse_catch()?);
        }
        let mut otherwise = Vec::new();
        self.skip_newlines();
        if self.at_keyword(&["else"]) {
            let keyword = self.next();
            self.end_of_block()?;
            otherwise = self.parse_block("else", keyword.line)?;
        }
        let mut finally = Vec::new();
        self.skip_newlines();
        if self.at_keyword(&["finally"]) {
            let keyword = self.next();
            self.end_of_block()?;
            finally = self.parse_block("finally", keyword.line)?;
        }
        Ok(Node::Try { body, catches, otherwise, finally, line: token.line })
    }

    /// `catch { }`, `catch as e { }`, `catch Error { }`, `catch Error as e { }`.
    ///
    /// The class is a name and not an expression: the prototype of the
    /// class is resolved when the script is read.
    fn parse_catch(&mut self) -> Result<Catch, SyntaxError> {
        let token = self.next();
        let mut class = "error".to_string();
        let mut variable = None;
        // The list of the parameters may be between parentheses.
        let mut in_parentheses = false;
        if self.at_op(&["("]) {
            self.next();
            in_parentheses = true;
        }
        if self.at_keyword(&["as"]) {
            self.next();
            variable = Some(self.parse_catch_variable()?);
        } else if self.peek().kind == TokenKind::Name {
            let name = self.next();
            class = name.value;
            if self.at_keyword(&["as"]) {
                self.next();
                variable = Some(self.parse_catch_variable()?);
            }
        }
        if in_parentheses {
            self.expect_op(")")?;
        }
        self.end_of_block()?;
        let body = self.parse_block("Catch", token.line)?;
        let _ = token;
        Ok(Catch { class, variable, body })
    }

    /// The name of the variable that holds the value that was caught.
    fn parse_catch_variable(&mut self) -> Result<String, SyntaxError> {
        let token = self.next();
        if token.kind != TokenKind::Name {
            return Err(SyntaxError {
                message: "the variable of a catch needs a name".into(),
                line: token.line,
                column: token.column,
            });
        }
        Ok(token.value)
    }

    /// `global name` or `local name`: a declaration of v2.
    ///
    /// The names are read when the script is read, and the values wait for
    /// the statement that carries them.
    fn parse_declaration(&mut self, kind: DeclareKind) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut names = Vec::new();
        while !self.at_end_of_statement() {
            let name = self.next();
            if name.kind != TokenKind::Name {
                return Err(self.error(format!("a declaration needs a name, not {:?}", name.value)));
            }
            // A declaration may not name a parameter of the function: v2
            // says so.
            if self
                .parameters
                .last()
                .map(|parameters| parameters.iter().any(|parameter| parameter.eq_ignore_ascii_case(&name.value)))
                .unwrap_or(false)
            {
                return Err(self.error(format!(
                    "This {} variable declaration conflicts with an existing parameter.\n     Specifically: {}",
                    kind.word(),
                    name.value
                )));
            }
            // The value of the declaration: `global g := 1`.
            let mut text = name.value.clone();
            let value = if self.at_op(&[":="]) {
                self.next();
                let start = self.position;
                let value = self.the_expression_of_a_statement()?;
                text.push_str(" := ");
                text.push_str(&the_text_of_the_tokens(self, start, self.position));
                Some(Box::new(value))
            } else {
                None
            };
            names.push(Declared { name: name.value, value, text });
            if !self.at_op(&[","]) {
                break;
            }
            self.next();
        }
        // A bare `local` is never a thing: only `global` has a mode, and
        // the bare `static` is a declaration with no name.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the bare
        // `static` inside a function is accepted, and the bare `local` is
        // refused.
        if kind == DeclareKind::Local && names.is_empty() {
            return Err(self.error("Unexpected declaration.\n     Specifically: local"));
        }
        // A `local` outside a function has no scope to live in, and a
        // `static` has no function to live in.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the `static` at
        // the top level is refused, and the text that is named is the whole
        // declaration.
        if (kind == DeclareKind::Local || kind == DeclareKind::Static) && self.parameters.is_empty() {
            return Err(self.error(format!(
                "Unexpected declaration.\n     Specifically: {}",
                the_text_of_a_declaration(kind, &names)
            )));
        }
        self.end_of_statement()?;
        Ok(Node::Declare { scope: kind, names, line: token.line })
    }

    /// The label of a line, when the line is one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a label is a name
    /// and a colon, and nothing else may be written on the line of it. The
    /// name is of letters, of digits and of the underscore, and it may
    /// begin with a digit: `1abc:` is a label. A line that ends with a
    /// colon and is not one of them is a line that holds no action.
    fn parse_label(&mut self) -> Result<Option<Node>, SyntaxError> {
        let token = self.peek().clone();
        if token.kind != TokenKind::Name && token.kind != TokenKind::Number {
            return Ok(None);
        }
        // The tokens of the line, up to the end of it.
        let mut end = self.position;
        while end < self.tokens.len() {
            let next = &self.tokens[end];
            if next.kind == TokenKind::Newline || next.kind == TokenKind::Eof {
                break;
            }
            if next.kind == TokenKind::Op && next.value == "}" {
                break;
            }
            end += 1;
        }
        let line = &self.tokens[self.position..end];
        let Some(colon) = line.last() else { return Ok(None) };
        if colon.kind != TokenKind::Op || colon.value != ":" {
            // The line may hold a label with something else written after
            // it: `done: FileAppend "x", "*"` holds no action.
            let writes_more = match line.get(1) {
                Some(second) => second.kind == TokenKind::Op && second.value == ":" && !second.spaced,
                None => false,
            };
            if !writes_more {
                return Ok(None);
            }
            return Err(self.the_line_holds_no_action(token.line));
        }
        // The name of the label: the tokens before the colon.
        let name = the_name_of_a_label(&line[..line.len() - 1]);
        match name {
            Some(name) => {
                // The space before the colon is not allowed: `done :` is
                // not a label.
                if colon.spaced {
                    return Err(self.the_line_holds_no_action(token.line));
                }
                self.position = end;
                Ok(Some(Node::Label { name, line: token.line }))
            }
            None => {
                // A name with a dash or with a dot is not a label, and the
                // line that holds it holds no action. A name with a space
                // is an expression that the parser reads the way it reads
                // every other one.
                if line[..line.len() - 1].iter().skip(1).any(|token| token.spaced) {
                    return Ok(None);
                }
                Err(self.the_line_holds_no_action(token.line))
            }
        }
    }

    /// `Goto name`: the jump to a label.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the parameter of a
    /// `Goto` is not an expression. It is the name of a label, and it is
    /// read as the text of it: `Goto "done"` looks for a label that is
    /// called `"done"`, with the quotes, and `Goto done` looks for the
    /// label `done`. The name of a label does not know its case.
    ///
    /// The `Goto` needs one parameter and it takes one: the load refuses
    /// the one that has none, and the load refuses the one that has more.
    fn parse_goto(&mut self) -> Result<Node, SyntaxError> {
        let keyword_at = self.position;
        let token = self.next();
        // The parenthesis of the dialect: `Goto(name)` is the same as
        // `Goto name`, and the parameter is the text inside it.
        let parenthesised = self.at_op(&["("]);
        let open_at = self.position;
        if parenthesised {
            self.next();
        }
        let start = self.position;
        let mut end = start;
        let mut parameters = 1;
        while end < self.tokens.len() {
            let next = &self.tokens[end];
            if next.kind == TokenKind::Newline || next.kind == TokenKind::Eof {
                break;
            }
            if parenthesised && next.kind == TokenKind::Op && next.value == ")" {
                break;
            }
            if !parenthesised && next.kind == TokenKind::Op && next.value == "}" {
                break;
            }
            // The comma is what makes a parameter of its own.
            if next.kind == TokenKind::Op && next.value == "," {
                parameters += 1;
            }
            end += 1;
        }
        let name = the_text_of_the_tokens(self, start, end);
        self.position = end;
        if parenthesised && self.at_op(&[")"]) {
            self.next();
        }
        // The `Goto` needs a parameter.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the failure
        // names the text that the dialect had read when it refused it, and
        // the parenthesis is part of that text: `Goto()` names `Goto(`.
        if name.is_empty() {
            let text = if parenthesised {
                the_source_of_the_tokens(self, keyword_at, open_at + 1)
            } else {
                the_source_of_the_line(self, token.line)
            };
            return Err(self.error(format!(
                "\"Goto\" requires at least 1 parameter.\n     Specifically: {text}"
            )));
        }
        // The `Goto` takes one.
        if parameters > 1 {
            return Err(self.error(format!(
                "\"Goto\" accepts at most 1 parameter.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
        }
        self.end_of_statement()?;
        Ok(Node::Goto { name, line: token.line })
    }

    /// The refusal of a label that is written twice in the same scope.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the load refuses
    /// the second label of a name, and it says which name it is: "Duplicate
    /// label.\n     Specifically: done". The name of a label does not know its
    /// case, and the scope of a label is the scope it is written in.
    fn check_the_labels(&self, body: &[Node]) -> Result<(), SyntaxError> {
        let mut seen: Vec<String> = Vec::new();
        for (name, line) in the_labels_of_a_scope(body) {
            let key = name.to_lowercase();
            if seen.contains(&key) {
                return Err(SyntaxError {
                    message: format!("Duplicate label.\n     Specifically: {name}"),
                    line,
                    column: 1,
                });
            }
            seen.push(key);
        }
        Ok(())
    }

    /// The failure of a line that holds no action.
    fn the_line_holds_no_action(&self, line: u32) -> SyntaxError {
        self.error(format!(
            "This line does not contain a recognized action.\n     Specifically: {}",
            the_source_of_the_line(self, line)
        ))
    }

    /// `throw value`, or `throw` alone.
    fn parse_throw(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let value = if self.at_end_of_statement() {
            None
        } else {
            Some(Box::new(self.the_expression_of_a_statement()?))
        };
        self.end_of_statement()?;
        Ok(Node::Throw { value, line: token.line })
    }

    fn parse_while(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `while`
            // needs a condition, and it says so in its own words.
            return Err(self.error(format!(
                "\"While\" requires at least 1 parameter.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
        }
        let condition = self.the_expression_of_a_statement()?;
        self.end_of_block()?;
        let body = self.parse_block("While", token.line)?;
        Ok(Node::While { condition: Box::new(condition), body, line: token.line })
    }

    /// A class: `class Name extends Base { }`.
    fn parse_class(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        self.parse_the_class(token, None)
    }

    /// A class, and the class that holds it.
    ///
    /// The name of a class inside a class is the name of the class that
    /// holds it and the name of it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the class inside
    /// `Outer` is "Outer.Inner", and the class inside `Outer.Middle` is
    /// "Outer.Middle.Inner".
    fn parse_the_class(&mut self, token: Token, outer: Option<&str>) -> Result<Node, SyntaxError> {
        let name_token = self.next();
        self.parse_the_class_with_the_name(token, name_token, outer)
    }

    /// A class, with the name of it already read.
    fn parse_the_class_with_the_name(
        &mut self,
        token: Token,
        name_token: Token,
        outer: Option<&str>,
    ) -> Result<Node, SyntaxError> {
        // Measured on the binary of v2.0.19 on 2026-10-31: a function may
        // not contain a class, and the line of the class is the one that is
        // named.
        if self.in_function {
            return Err(SyntaxError {
                message: format!(
                    "Functions cannot contain classes.\n     Specifically: {}",
                    the_source_of_the_line(self, token.line)
                ),
                line: token.line,
                column: token.column,
            });
        }
        if name_token.kind != TokenKind::Name {
            // Measured on the binary of v2.0.19 on 2026-10-31: a name that
            // begins with a number is refused by the lexer, a reserved word
            // is a word that cannot be a name, and the brace is what is
            // unexpected when there is no name at all.
            let message = match name_token.kind {
                TokenKind::Number => format!(
                    "This class name starts with a number, which is not allowed:\n{:?}",
                    name_token.value
                ),
                TokenKind::Keyword => format!(
                    "The following reserved word must not be used as a class name:\n{:?}",
                    name_token.value
                ),
                _ => "Unexpected \"{\"".to_string(),
            };
            return Err(SyntaxError { message, line: name_token.line, column: name_token.column });
        }
        let mut extends = None;
        if self.at_keyword(&["extends"]) {
            self.next();
            let base = self.next();
            if base.kind != TokenKind::Name {
                // Measured on the binary of v2.0.19 on 2026-10-31: the base
                // of a class is a class, and the one that is not there is
                // named.
                return Err(SyntaxError {
                    message: format!(
                        "Unknown class.\n     Specifically: {}",
                        the_source_of_a_token(&base)
                    ),
                    line: base.line,
                    column: base.column,
                });
            }
            extends = Some(base.value);
        }
        // The name of a class inside a class is the name of the class that
        // holds it and the name of it.
        let name = match outer {
            Some(outer) => format!("{outer}.{}", name_token.value),
            None => name_token.value.clone(),
        };
        // The base of the class is checked when the script is read: it may
        // be declared after the class that extends it.
        if let Some(base) = &extends {
            self.bases.push((name.clone(), base.clone(), name_token.line, name_token.column));
        }
        // The duplicates of the class are refused while its body is read,
        // the way v2 refuses them: the first duplicate of the body is the
        // one that is reported.
        let members = self.parse_class_body(name_token.line, &name)?;
        self.the_name_is_declared_twice(&name, true, name_token.line, name_token.column)?;
        self.classes.insert(name.to_lowercase(), name.clone());
        self.defined.insert(name.to_lowercase());
        Ok(Node::ClassDef {
            name,
            extends,
            methods: members.methods,
            properties: members.properties,
            accessors: members.accessors,
            nested: members.nested,
            line: token.line,
        })
    }

    /// A class with no name, inside a class.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: v2 reads the class
    /// and drops it, and the class that holds it does not own it: the own
    /// properties of the class that holds it are the ones it had before.
    fn parse_the_class_with_no_name(&mut self, token: Token, name: &str) -> Result<(), SyntaxError> {
        // The brace of the class is the token that was read as its name.
        self.parse_the_members_of_a_class(token.line, name)?;
        Ok(())
    }

    /// The members of a class: the methods and the properties.
    fn parse_class_body(&mut self, line: u32, name: &str) -> Result<TheMembersOfAClass, SyntaxError> {
        // The brace may be on a line of its own.
        self.skip_newlines();
        if !self.at_op(&["{"]) {
            // Measured on the binary of v2.0.19 on 2026-10-31: the class
            // needs the braces, and the line of the class is the one that is
            // named.
            return Err(SyntaxError {
                message: format!(
                    "Missing \"{{\"\n     Specifically: {}",
                    the_source_of_the_line(self, line)
                ),
                line,
                column: self
                    .tokens
                    .iter()
                    .find(|token| token.line == line)
                    .map(|token| token.column)
                    .unwrap_or(1),
            });
        }
        self.expect_op("{")?;
        self.parse_the_members_of_a_class(line, name)
    }

    /// The members of a class, after the brace that opens them.
    fn parse_the_members_of_a_class(
        &mut self,
        line: u32,
        name: &str,
    ) -> Result<TheMembersOfAClass, SyntaxError> {
        if !self.the_body_begins_on_the_next_line() {
            return Err(self.error(self.the_message_of_the_opening_brace("class", line)));
        }
        let mut methods = Vec::new();
        let mut properties = Vec::new();
        let mut accessors = Vec::new();
        let mut nested = Vec::new();
        // The names that are taken, and the table that each kind of member
        // has.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: a name may be
        // declared only once in a table, and the tables of a class are six:
        // the methods of the instance, the methods of the class, the
        // properties of the instance and the properties of the class, and
        // the properties that have a getter or a setter. A method and a
        // property are two names and not one, and the case of the name does
        // not count.
        let mut instance_methods: Vec<String> = Vec::new();
        let mut class_methods: Vec<String> = Vec::new();
        let mut instance_properties: Vec<String> = Vec::new();
        let mut class_properties: Vec<String> = Vec::new();
        let mut instance_accessors: Vec<String> = Vec::new();
        let mut class_accessors: Vec<String> = Vec::new();
        // The body of a class is inside a class, and the `super` of v2 is
        // only valid there.
        let was_in_a_class = self.in_class;
        self.in_class = true;
        // The statement that is being read is the one of the member, and
        // the class gives its place back when it is done.
        let statement_start = self.statement_start;
        loop {
            self.skip_newlines();
            let token = self.peek().clone();
            if token.kind == TokenKind::Eof {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // brace that is not closed is named, and the class that
                // was open is the one that is named with it.
                return Err(SyntaxError {
                    message: format!("Missing \"}}\"\n     Specifically: {name}"),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                self.the_closing_brace_is_refused()?;
                self.position += 1;
                break;
            }
            // A member may belong to the class and not to an instance.
            let mut is_static = false;
            let start = self.position;
            // The member is the statement that is being read, and a
            // token that cannot be there names it.
            self.statement_start = start;
            if self.at_keyword(&["static"]) {
                self.next();
                is_static = true;
            }
            let name_token = self.next();
            if name_token.kind == TokenKind::Keyword && name_token.value.eq_ignore_ascii_case("class") {
                // A class inside a class. Its name is a name of the class
                // that holds it: it is a property of it, and the duplicate
                // of it is refused the way the duplicate of a property is.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: "Duplicate
                // declaration.\n     Specifically: Inner", and the name of
                // the class is not the name of the class that holds it.
                let nested_name = self.next();
                match nested_name.kind {
                    TokenKind::Name => {}
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // name of a class may not begin with a number, and a
                    // reserved word is not the name of a class.
                    TokenKind::Number => {
                        return Err(SyntaxError {
                            message: format!(
                                "This class name starts with a number, which is not allowed:\n{:?}",
                                nested_name.value
                            ),
                            line: nested_name.line,
                            column: nested_name.column,
                        });
                    }
                    TokenKind::Keyword => {
                        return Err(SyntaxError {
                            message: format!(
                                "The following reserved word must not be used as a class name:\n{:?}",
                                nested_name.value
                            ),
                            line: nested_name.line,
                            column: nested_name.column,
                        });
                    }
                    // Measured on the binary of v2.0.19 on 2026-10-31: a
                    // class with no name is read and dropped, and the class
                    // that holds it does not own it.
                    _ => {
                        self.parse_the_class_with_no_name(name_token, name)?;
                        continue;
                    }
                }
                let key = nested_name.value.to_lowercase();
                if class_properties.contains(&key) {
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {}",
                            nested_name.value
                        ),
                        line: nested_name.line,
                        column: nested_name.column,
                    });
                }
                class_properties.push(key);
                let line = name_token.line;
                let column = name_token.column;
                let class = self.parse_the_class_with_the_name(name_token, nested_name, Some(name))?;
                // A class may not be static: v2 names the whole of the
                // definition, and not the line of it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // "Not a valid method, class or property definition.
                // Specifically: static class Inner { }".
                if is_static {
                    return Err(SyntaxError {
                        message: format!(
                            "Not a valid method, class or property definition.\n     Specifically: {}",
                            the_source_of_a_definition(self, start, self.position)
                        ),
                        line,
                        column,
                    });
                }
                nested.push(class);
                continue;
            }
            if name_token.kind == TokenKind::Number && the_number_is_a_float(&name_token) {
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of a property may be a number, and the number that is a
                // float is not a name at all.
                if self.at_op(&[":=", "="]) {
                    // The assignment of a name that is not a name of the
                    // class is refused with the words of v2.
                    return Err(SyntaxError {
                        message: format!(
                            "Unknown class var.\n     Specifically: {}",
                            the_source_of_the_line(self, name_token.line)
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                return Err(SyntaxError {
                    message: format!(
                        "Not a valid method, class or property definition.\n     Specifically: {}",
                        the_source_of_the_line(self, name_token.line)
                    ),
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            if name_token.kind != TokenKind::Name
                && name_token.kind != TokenKind::Keyword
                && name_token.kind != TokenKind::Number
            {
                // Measured on the binary of v2.0.19 on 2026-10-31: a brace
                // in a class is a token that cannot be there, and the line
                // of it is named.
                if name_token.kind == TokenKind::Op && name_token.value == "{" {
                    return Err(SyntaxError {
                        message: format!(
                            "Unexpected \"{{\"\n     Specifically: {}",
                            the_source_of_the_line(self, name_token.line)
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: a member
                // of a class that is not a name is not a valid definition,
                // and the line that says it is named.
                return Err(SyntaxError {
                    message: format!(
                        "Not a valid method, class or property definition.\n     Specifically: {}",
                        the_source_of_the_line(self, name_token.line)
                    ),
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            if self.at_op(&["("]) {
                // The `__Init` of the class is one that every class has: a
                // static one of its own is a duplicate of it.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: "Duplicate
                // declaration.\n     Specifically: C.__Init", at the line of
                // the declaration of it.
                if is_static && name_token.value.eq_ignore_ascii_case("__init") {
                    return Err(SyntaxError {
                        message: format!("Duplicate declaration.\n     Specifically: {name}.__Init"),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                // The `__Init` of an instance and the properties of the
                // class: v2 makes the `__Init` that carries the properties,
                // and the two cannot both be there.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // properties that come *before* the `__Init` are the ones
                // that cannot both be there, and the ones that come after
                // it are taken.
                if !is_static
                    && name_token.value.eq_ignore_ascii_case("__init")
                    && !instance_properties.is_empty()
                {
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {name}.Prototype.__Init"
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                // The name of a method that is taken: a method of the
                // instance and a method of the class are two names and not
                // one.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: "Duplicate
                // declaration.\n     Specifically: C.Prototype.Foo" for a
                // method of an instance, and "C.Foo" for a method of the
                // class. The failure is at the line of the second one.
                let taken = if is_static { &mut class_methods } else { &mut instance_methods };
                let key = name_token.value.to_lowercase();
                if taken.contains(&key) {
                    let owner = if is_static { format!("{name}.") } else { format!("{name}.Prototype.") };
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {owner}{}",
                            name_token.value
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                taken.push(key);
                self.next();
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                self.parameters.push(parameters.iter().map(|parameter| parameter.name.clone()).collect());
                self.assignments.push(Vec::new());
                // A method may not contain a class either.
                let was_in_a_function = self.in_function;
                self.in_function = true;
                let body = self.parse_block("method", name_token.line)?;
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of a method may not begin with a number, and the name is
                // named with it.
                if name_token.kind == TokenKind::Number {
                    return Err(SyntaxError {
                        message: format!(
                            "This method name starts with a number, which is not allowed:\n{:?}",
                            the_name_that_begins_with_a_number(&name_token)
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                self.in_function = was_in_a_function;
                self.check_the_labels(&body)?;
                let assignments = core::mem::take(self.assignments.last_mut().expect("the list of the assignments"));
                self.assignments.pop();
                self.parameters.pop();
                let declarations = the_declarations_of_a_body(&body)?;
                for assignment in assignments {
                    if declarations.globals.contains(&assignment.name.to_lowercase()) {
                        self.check_the_assignments(vec![assignment])?;
                    }
                }
                methods.push(MethodDef {
                    name: name_token.value,
                    parameters,
                    body,
                    declarations,
                    is_static,
                    line: name_token.line,
                });
                continue;
            }
            // A property that has a getter or a setter: `value { get => 1 }`.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: the property
            // has a name of its own, and the duplicate of it is refused the
            // way the duplicate of a property is.
            if self.at_op(&["[", "{"]) {
                let key = name_token.value.to_lowercase();
                let taken = if is_static { &mut class_accessors } else { &mut instance_accessors };
                if taken.contains(&key) {
                    return Err(SyntaxError {
                        message: format!("Duplicate declaration.\n     Specifically: {}", name_token.value),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                taken.push(key);
                accessors.push(self.parse_the_accessor(name_token, is_static)?);
                continue;
            }
            // A property: `Name := value`.
            let operator = self.next();
            if operator.kind != TokenKind::Op || !matches!(operator.value.as_str(), "=" | ":=") {
                // Measured on the binary of v2.0.19 on 2026-10-31: a member
                // that is a name and nothing else is a line with no action,
                // and a member with something else is not a valid
                // definition. The line is named in both.
                let alone = operator.kind == TokenKind::Newline
                    || operator.kind == TokenKind::Eof
                    || (operator.kind == TokenKind::Op && operator.value == "}");
                let message = if alone {
                    format!(
                        "This line does not contain a recognized action.\n     Specifically: {}",
                        name_token.value
                    )
                } else {
                    format!(
                        "Not a valid method, class or property definition.\n     Specifically: {}",
                        the_source_of_the_line(self, name_token.line)
                    )
                };
                return Err(SyntaxError {
                    message,
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            // The name of a property that is taken: a property of the
            // instance and a property of the class are two names and not
            // one.
            //
            // Measured on the binary of v2.0.19 on 2026-10-31: "Duplicate
            // declaration.\n     Specifically: value", at the line of the
            // second one.
            let taken = if is_static { &mut class_properties } else { &mut instance_properties };
            let key = name_token.value.to_lowercase();
            if taken.contains(&key) {
                return Err(SyntaxError {
                    message: format!("Duplicate declaration.\n     Specifically: {}", name_token.value),
                    line: name_token.line,
                    column: name_token.column,
                });
            }
            taken.push(key);
            let value = self.the_expression_of_a_statement()?;
            properties.push(PropertyDef {
                name: name_token.value,
                value: Some(Box::new(value)),
                is_static,
                line: name_token.line,
            });
        }
        self.in_class = was_in_a_class;
        self.statement_start = statement_start;
        Ok(TheMembersOfAClass { methods, properties, accessors, nested })
    }

    /// A property of the class that has a getter or a setter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the property may
    /// take parameters, and the body of the getter and of the setter is an
    /// expression after the arrow of v2, and a block of statements between
    /// braces.
    fn parse_the_accessor(&mut self, name_token: Token, is_static: bool) -> Result<AccessDef, SyntaxError> {
        let mut parameters = Vec::new();
        if self.at_op(&["["]) {
            self.next();
            parameters = self.parse_parameters()?;
            self.expect_op("]")?;
        }
        self.expect_op("{")?;
        let mut getter = None;
        let mut setter = None;
        loop {
            self.skip_newlines();
            if self.at_op(&["}"]) {
                self.position += 1;
                break;
            }
            let token = self.next();
            let body = self.parse_the_body_of_an_accessor(token.line)?;
            match token.value.to_lowercase().as_str() {
                "get" => getter = Some(body),
                "set" => setter = Some(body),
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // getter and the setter are the only members of the property,
                // and the one that is not known is named.
                _ => {
                    return Err(self.error(format!(
                        "Not a valid property getter/setter.\n     Specifically: {}",
                        the_source_of_the_line(self, token.line)
                    )));
                }
            }
        }
        Ok(AccessDef {
            name: name_token.value,
            parameters,
            getter,
            setter,
            is_static,
            line: name_token.line,
        })
    }

    /// The body of a getter or of a setter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the body is an
    /// expression after the arrow of v2, and the assignment of v2 is an
    /// expression, and a block of statements between braces.
    fn parse_the_body_of_an_accessor(&mut self, line: u32) -> Result<AccessBody, SyntaxError> {
        let body = if self.at_op(&["=>"]) {
            let token = self.next();
            let expression = self.parse_expression()?;
            if self.at_an_assignment_operator() {
                vec![self.parse_the_assignment(expression)?]
            } else {
                vec![Node::Return { value: Some(Box::new(expression)), line: token.line }]
            }
        } else if self.at_op(&["{"]) {
            self.parse_block("accessor", line)?
        } else {
            // Measured on the binary of v2.0.19 on 2026-10-31: the getter
            // and the setter have a body, and the line that has none is
            // named.
            return Err(self.error(format!(
                "Not a valid property getter/setter.\n     Specifically: {}",
                the_source_of_the_line(self, line)
            )));
        };
        let declarations = the_declarations_of_a_body(&body)?;
        Ok(AccessBody { body, declarations })
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
        self.the_expression_begins();
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
        self.the_expression_is_done(self.position, false)?;
        self.end_of_block()?;
        let body = self.parse_block("Loop", token.line)?;
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
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `for`
            // needs the names, and it says so in its own words.
            return Err(self.error(format!(
                "Syntax error.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
        }
        self.parse_for_variables(&mut variables)?;
        if !self.at_keyword(&["in"]) {
            return Err(self.error("expected `in` in the For loop"));
        }
        self.next();
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `for` needs
            // the collection, and it names its own word.
            return Err(self.error(self.the_message_of_the_body("For")));
        }
        self.the_expression_begins();
        let source = self.parse_expression()?;
        if parenthesised {
            self.expect_op(")")?;
        }
        self.the_expression_is_done(self.position, false)?;
        self.end_of_block()?;
        let body = self.parse_block("For", token.line)?;
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
                // Measured on the binary of v2.0.19 on 2026-10-31: the dot
                // that does not reach into a value is a "Syntax error." and
                // the rest of the line, from the dot.
                if self.at_op(&["."]) && self.the_space_after_is_there() {
                    return Err(self.error(format!(
                        "Syntax error.\n     Specifically: {}",
                        the_source_of_the_rest_of_the_line(self, self.position)
                    )));
                }
                // The implicit concatenation of v2: two values side by side
                // are one value. It binds like the dot, and the dot is
                // looser than the arithmetic.
                if precedence::CONCAT < min || !self.starts_expression() {
                    // Measured on the binary of v2.0.19 on 2026-10-31: a
                    // line that begins with an operator continues the one
                    // before it.
                    if self.the_next_line_begins_with_an_operator() {
                        self.skip_newlines();
                        continue;
                    }
                    return Ok(left);
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: two values
                // side by side are one value, and the space between them is
                // what makes it so. Without the space, v2 refuses and names
                // the value that came too close.
                if !self.peek().spaced {
                    return Err(self.error(format!(
                        "Missing space or operator before this.\n     Specifically: {}",
                        the_source_of_the_rest_of_the_line(self, self.position)
                    )));
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
            // Measured on the binary of v2.0.19 on 2026-10-31: an operator
            // that wants a number may not follow a literal text. The error
            // is named when the expression that holds the operator is done,
            // so it waits here.
            // The group of v2 is not a literal: `("a") + 0` is a value
            // that is not a constant string, and v2 refuses it at the run.
            let closed = self
                .tokens
                .get(self.position.saturating_sub(2))
                .map(|token| token.value == ")")
                .unwrap_or(false);
            if self.pending_literal.is_none()
                && !closed
                && the_literal_is_a_text(&left)
                && the_operator_wants_a_number(op)
            {
                self.pending_literal = Some(self.the_pending_literal(self.position - 1));
            }
            // The value that should follow the operator is not there, and
            // v2 names the operator anyway.
            if self.pending_literal.is_some() && !self.the_operand_is_there() {
                let pending = self.pending_literal.take().expect("the operator");
                return Err(self.the_literal_error(&pending, self.position, false));
            }
            let next = if right_associative { prec } else { prec + 1 };
            let right = self.parse_binary(next)?;
            left = Node::Binary { op, left: Box::new(left), right: Box::new(right), line: token.line };
        }
    }

    /// The binary operator that comes next, if there is one.
    fn peek_binary_op(&self) -> Option<(BinaryOp, u8, bool)> {
        let token = self.peek();
        if token.kind == TokenKind::Op {
            // The dot that reaches into a value is not the operator that
            // joins two values, and the operator wants a space on both
            // sides: `a . b` joins two values, and `a. b` is an error.
            if token.value == "." && !(token.spaced && self.the_space_after_is_there()) {
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
            // A parenthesis opens a value: the concatenation of v2 takes it
            // as the value that follows the one before it.
            TokenKind::Op if token.value == "(" => true,
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

    /// Whether the value that follows the operator is there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the line that
    /// ends with an operator continues on the next one, and the newlines
    /// of the continuation are not a part of it.
    fn the_operand_is_there(&mut self) -> bool {
        if self.peek().kind == TokenKind::Newline && self.the_line_continues() {
            self.skip_newlines();
        }
        self.starts_expression()
    }

    /// Whether the line that comes next begins with an operator.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a line that
    /// begins with an operator continues the one before it, and the
    /// increment and the decrement are not among them.
    fn the_next_line_begins_with_an_operator(&self) -> bool {
        let mut at = self.position;
        while at < self.tokens.len() && self.tokens[at].kind == TokenKind::Newline {
            at += 1;
        }
        let Some(token) = self.tokens.get(at) else { return false };
        if token.kind == TokenKind::Op && (token.value == "++" || token.value == "--") {
            return false;
        }
        the_token_is_an_operator(token)
    }

    /// Whether the line continues on the next one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a line that ends
    /// with an operator continues on the next one, and the empty lines
    /// between the two are not a part of it. The increment and the
    /// decrement are not among the operators: the line that ends with one
    /// of them wants an operand, and it does not continue.
    fn the_line_continues(&self) -> bool {
        let Some(token) = self.the_token_before(self.position) else { return false };
        match token.kind {
            TokenKind::Op => matches!(
                token.value.as_str(),
                "(" | "[" | "," | "?" | ":" | "=>" | "!" | "~"
                    | "+" | "-" | "*" | "/" | "//" | "**" | "<<" | ">>" | ">>>"
                    | "&" | "^" | "|" | "~=" | ">" | ">=" | "<" | "<=" | "=" | "=="
                    | "!=" | "!==" | "&&" | "||" | "??" | ":=" | "+=" | "-=" | "*="
                    | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>="
            ),
            TokenKind::Keyword => matches!(token.value.to_lowercase().as_str(), "and" | "or" | "is"),
            _ => false,
        }
    }

    /// The token that comes before the given place, without the newlines
    /// that the continuation of a line skipped.
    fn the_token_before(&self, at: usize) -> Option<&Token> {
        let mut at = at;
        while at > 0 {
            at -= 1;
            let token = &self.tokens[at];
            if token.kind != TokenKind::Newline {
                return Some(token);
            }
        }
        None
    }

    /// The statement that is being read, and the token that cannot be
    /// there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text is the
    /// statement and the token, and the newline of the statement is a
    /// space in it.
    fn the_statement_so_far(&self) -> String {
        let start = self.statement_start.min(self.position);
        the_source_of_a_definition(self, start, self.position)
    }

    /// The error of the operand that is not there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the operator that
    /// wants the operand names the failure, and the token that cannot be
    /// there is named with it. A brace that closes a block is not an
    /// operand that is missing: it is a token that cannot be there, and
    /// the statement it is in is named with it.
    ///
    /// The text that is named is the rest of the line: "Missing operand."
    /// and the operator and the rest of the line when the operator wanted
    /// an operand, and "Syntax error." and the rest of the line when the
    /// operator begins a value.
    fn the_error_of_a_missing_operand(&self, token: &Token) -> SyntaxError {
        let start = self.tokens.get(self.statement_start).unwrap_or(token);
        let message = match token.kind {
            // Measured on the binary of v2.0.19 on 2026-10-31: a brace that
            // closes a block is not an operand that is missing: it is a
            // token that cannot be there, and the statement it is in is
            // named with it.
            TokenKind::Op if matches!(token.value.as_str(), ")" | "}" | "]") => format!(
                "Unexpected {:?}\n     Specifically: {}",
                token.value,
                self.the_statement_so_far()
            ),
            // The colon of the ternary is a part of the question that is
            // not there.
            TokenKind::Op if token.value == ":" => "A \":\" is missing its \"?\"".to_string(),
            // An operator may not begin a value.
            TokenKind::Op if token.value != "," => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the text
                // of the failure is not there when the operator is the last
                // item of a list.
                if self.the_operator_is_the_last_item_of_a_list() {
                    return SyntaxError {
                        message: "Syntax error.".to_string(),
                        line: start.line,
                        column: start.column,
                    };
                }
                let rest = the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1));
                match self.the_operator_that_wants_the_operand() {
                    Some(operator) => {
                        format!("Missing operand.\n     Specifically: {operator} {rest}")
                    }
                    None => format!("Syntax error.\n     Specifically: {rest}"),
                }
            }
            // The operand is not there, and the operator that wants it
            // names the failure.
            _ => {
                let mut text = String::new();
                if let Some(operator) = self.the_token_before(self.position.saturating_sub(1)) {
                    text.push_str(&operator.value);
                }
                let rest = the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1));
                if !rest.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&rest);
                }
                format!("Missing operand.\n     Specifically: {text}")
            }
        };
        SyntaxError { message, line: start.line, column: start.column }
    }

    /// Whether the operator that may not begin a value is the last item of
    /// a list.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text of the
    /// "Syntax error." is not there when the operator is the last item of the
    /// arguments of a call or of the items of an array. The group and the
    /// object are not lists, and the text is there.
    fn the_operator_is_the_last_item_of_a_list(&self) -> bool {
        // The closing bracket of the list comes after the operator.
        let Some(next) = self.tokens.get(self.position) else { return false };
        if next.kind != TokenKind::Op || !matches!(next.value.as_str(), ")" | "]") {
            return false;
        }
        // The bracket that holds the operator is the one that is open.
        let mut depth = 0usize;
        let mut index = self.position;
        while index > 0 {
            index -= 1;
            let token = &self.tokens[index];
            if token.kind != TokenKind::Op {
                continue;
            }
            match token.value.as_str() {
                ")" | "]" | "}" => depth += 1,
                "(" | "[" | "{" => {
                    if depth > 0 {
                        depth -= 1;
                        continue;
                    }
                    // The items of an array are a list.
                    if token.value == "[" {
                        return true;
                    }
                    // The arguments of a call are a list, and the parenthesis
                    // of a call is the one that follows a name.
                    if token.value == "(" {
                        return self
                            .tokens
                            .get(index.saturating_sub(1))
                            .map(|before| {
                                before.kind == TokenKind::Name || before.kind == TokenKind::Keyword
                            })
                            .unwrap_or(false);
                    }
                    return false;
                }
                _ => {}
            }
        }
        false
    }

    /// The operator that wanted the operand that is not there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the operator that
    /// joins two values wants an operand, and so do the compound
    /// assignments and the question of the ternary. The plain assignment
    /// does not: `x := * 2` is a "Syntax error." and not a "Missing
    /// operand.".
    fn the_operator_that_wants_the_operand(&self) -> Option<String> {
        let token = self.the_token_before(self.position.saturating_sub(1))?;
        let wants = match token.kind {
            TokenKind::Op => matches!(
                token.value.as_str(),
                "**" | "*" | "/" | "//" | "+" | "-" | "<<" | ">>" | ">>>" | "&" | "^" | "|"
                    | "~=" | ">" | ">=" | "<" | "<=" | "=" | "==" | "!=" | "!==" | "&&" | "||"
                    | "??" | "?" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^="
                    | "<<=" | ">>=" | ">>>="
            ),
            TokenKind::Keyword => matches!(token.value.to_lowercase().as_str(), "and" | "or" | "is"),
            _ => false,
        };
        if wants {
            Some(token.value.clone())
        } else {
            None
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
                let at_the_operator = self.position;
                self.next();
                // The power binds tighter than the sign and the bitwise
                // operators: `-2 ** 2` is `-(2 ** 2)`, the way v2 reads
                // it. The increment and the decrement bind tighter than
                // the power, so they take only the value.
                let operand = match op {
                    UnaryOp::PreIncrement | UnaryOp::PreDecrement => self.parse_unary()?,
                    _ => self.parse_binary(precedence::POWER)?,
                };
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // increment of a value that cannot be written is refused at
                // the load, and the source of the operator is named with it.
                if matches!(op, UnaryOp::PreIncrement | UnaryOp::PreDecrement)
                    && !the_value_can_be_written(&operand)
                {
                    return Err(self.error(format!(
                        "Invalid assignment.\n     Specifically: {}",
                        the_source_of_the_tokens(self, at_the_operator, self.position)
                    )));
                }
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
        // Measured on the binary of v2.0.19 on 2026-10-31: a line that
        // ends with an operator continues on the next one, and the empty
        // lines between the two are not a part of it.
        if self.peek().kind == TokenKind::Newline && self.the_line_continues() {
            self.skip_newlines();
        }
        let token = self.next();
        let node = match token.kind {
            TokenKind::Number => {
                let value = the_number_of(&token.value);
                Node::Literal { value, line: token.line }
            }
            TokenKind::String => Node::Literal { value: LiteralValue::Text(token.value), line: token.line },
            TokenKind::Keyword => {
                // The `super` of v2 is the base of the class where the
                // method was defined. It is the only keyword that is
                // neither a value nor a call.
                if token.value.eq_ignore_ascii_case("super") {
                    self.parse_super(token)?
                } else if self.at_op(&["("]) && !self.peek().spaced {
                    // A keyword that is a function name -- `PixelGetColor(10, 20)`
                    // -- is a call. The parenthesis that is spaced is not
                    // one: the concatenation of v2 takes the name and the
                    // value that follows it.
                    let call = self.parse_call(token.value.to_lowercase(), token.line)?;
                    return self.parse_suffixes(call, true);
                } else {
                    // The words that are values are the two below.
                    match token.value.to_lowercase().as_str() {
                        "true" => Node::Literal { value: LiteralValue::Bool(true), line: token.line },
                        "false" => Node::Literal { value: LiteralValue::Bool(false), line: token.line },
                        // The `Class` is a word of the language and a class
                        // type of the dialect: a script reads it as the
                        // class.
                        "class" => Node::Var { name: token.value.clone(), line: token.line },
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
            }
            TokenKind::Name => {
                // The parenthesis that is spaced is not a call: the
                // concatenation of v2 takes the name and the value that
                // follows it, and the space is what tells them apart.
                if self.at_op(&["("]) && !self.peek().spaced {
                    // The answer of a call is a value, and the members of
                    // it are read the way the members of any other value
                    // are: `StrLen("abc").__Class`.
                    let call = self.parse_call(token.value.to_lowercase(), token.line)?;
                    return self.parse_suffixes(call, true);
                }
                let name = token.value;
                // The arrow function of v2 with one parameter: `a => a + 1`.
                if self.at_op(&["=>"]) {
                    self.position += 1;
                    let body = self.parse_expression()?;
                    let parameters = vec![Parameter { name: name.clone(), default: None, variadic: false, by_ref: false }];
                    return Ok(Node::Lambda { parameters, body: Box::new(body), line: token.line });
                }
                // `x++` and `x--`: the operator comes after the name, and
                // the space is what tells it apart from the concat.
                if self.at_op(&["++", "--"]) && !self.peek().spaced {
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
            TokenKind::Op if token.value == "(" && self.at_the_parameters_of_an_arrow_function() => {
                // The arrow function of v2: `(a, b) => a + b`. The body is
                // an expression, and the function closes over the scope
                // where it was written.
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                self.expect_op("=>")?;
                let body = self.parse_expression()?;
                Node::Lambda { parameters, body: Box::new(body), line: token.line }
            }
            TokenKind::Op if token.value == "(" => {
                let node = self.parse_expression()?;
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // group may be written in more than one line.
                self.skip_newlines();
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
                        // The list may be written in more than one line.
                        self.skip_newlines();
                        if self.at_op(&[","]) {
                            self.position += 1;
                            // A comma at the end of the list is allowed.
                            if self.at_op(&["]"]) {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                // The array may be written in more than one line.
                self.skip_newlines();
                self.expect_op("]")?;
                Node::ArrayLiteral { items, line: token.line }
            }
            TokenKind::Op if token.value == "{" => {
                // The literal of an object: `{a: 1, b: 2}`. The braces of
                // a block open a statement, and these open a value.
                let mut properties = Vec::new();
                if !self.at_op(&["}"]) {
                    loop {
                        let name = self.next();
                        if name.kind != TokenKind::Name && name.kind != TokenKind::Keyword {
                            return Err(SyntaxError {
                                message: "Invalid property name in object literal.".into(),
                                line: name.line,
                                column: name.column,
                            });
                        }
                        if !self.at_op(&[":"]) {
                            return Err(SyntaxError {
                                message: "Missing \"propertyname:\" in object literal.".into(),
                                line: name.line,
                                column: name.column,
                            });
                        }
                        self.position += 1;
                        let value = self.parse_expression()?;
                        properties.push((name.value, value));
                        // The list may be written in more than one line.
                        self.skip_newlines();
                        if self.at_op(&[","]) {
                            self.position += 1;
                            // A comma at the end of the list is allowed.
                            if self.at_op(&["}"]) {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                }
                // The object may be written in more than one line.
                self.skip_newlines();
                self.expect_op("}")?;
                Node::ObjectLiteral { properties, line: token.line }
            }
            _ => {
                // The operand that the operator wants is not there.
                return Err(self.the_error_of_a_missing_operand(&token));
            }
        };
        self.parse_suffixes(node, true)
    }

    /// The `[...]` and the `as` that may follow a value.
    ///
    /// The assignment is a suffix too, when the value is an expression: the
    /// assignment of v2 is an expression, and `(x := 1)` is a value.
    fn parse_suffixes(&mut self, target: Node, assignment: bool) -> Result<Node, SyntaxError> {
        let mut node = target;
        loop {
            if self.at_op(&["["]) {
                let line = node.line();
                self.position += 1;
                let index = self.parse_expression()?;
                // The index may be written in more than one line.
                self.skip_newlines();
                self.expect_op("]")?;
                node = Node::Index { target: Box::new(node), index: Box::new(index), line };
                continue;
            }
            if self.at_member_dot() {
                let at_the_dot = self.position;
                let dot = self.next();
                let name = self.next();
                // The member of v2 may be a number: the name of the
                // property is the text of the number, and `1.5.2` asks for
                // the property "2" of the float.
                if name.kind != TokenKind::Name
                    && name.kind != TokenKind::Keyword
                    && name.kind != TokenKind::Number
                {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // dot that reaches into nothing is a "Syntax error." and
                    // the rest of the line, from the dot.
                    return Err(SyntaxError {
                        message: format!(
                            "Syntax error.\n     Specifically: {}",
                            the_source_of_the_rest_of_the_line(self, at_the_dot)
                        ),
                        line: dot.line,
                        column: dot.column,
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
            if self.at_op(&["++", "--"]) && !self.peek().spaced {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // operator that comes after the value is the increment, and
                // it binds to the value that is before it.
                let operator = self.next();
                let op = if operator.value == "++" {
                    UnaryOp::PostIncrement
                } else {
                    UnaryOp::PostDecrement
                };
                let line = node.line();
                // The increment of a value that cannot be written is refused
                // at the load, and the source of the operator is named.
                if !the_value_can_be_written(&node) {
                    return Err(self.error(format!(
                        "Invalid assignment.\n     Specifically: {}",
                        the_source_of_the_tokens(self, self.position - 1, self.position)
                    )));
                }
                node = Node::Unary { op, operand: Box::new(node), line };
                continue;
            }
            if assignment && self.at_an_assignment_operator() {
                return self.parse_the_assignment(node);
            }
            break;
        }
        Ok(node)
    }

    /// The operator of an assignment that comes after the token.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the number that is
    /// the left side of an assignment is refused at the parse by the plain
    /// assignment, and the compound operators refuse it later, with the
    /// operator and the value.
    fn the_assignment_operator_after_the_number(&self) -> Option<String> {
        let token = self.tokens.get(self.position + 1)?;
        if token.kind != TokenKind::Op {
            return None;
        }
        the_assignment_operator(&token.value).map(|_| token.value.clone())
    }

    /// Whether the operator of an increment comes after the name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the space is what
    /// tells the operator of the increment apart from the concat: `x++` is
    /// the increment of `x`, and `x ++` is a call of `x`.
    fn the_postfix_increment_is_next(&self) -> bool {
        let Some(token) = self.tokens.get(self.position + 1) else { return false };
        token.kind == TokenKind::Op && matches!(token.value.as_str(), "++" | "--") && !token.spaced
    }

    /// The record of an assignment, for the check of the classes.
    ///
    /// The name of a class is a name of the script, and it may not be the
    /// output variable of an assignment. The check waits for the end of
    /// the parse: the class may be declared further down.
    fn the_assignment_is_recorded(&mut self, name: &str, op: AssignOp, line: u32, column: u32) {
        let Some(list) = self.assignments.last_mut() else { return };
        list.push(Assignment { name: name.to_string(), op, line, column });
    }

    /// Whether the token is the operator of an assignment.
    fn at_an_assignment_operator(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Op && the_assignment_operator(&token.value).is_some()
    }

    /// The assignment of a value, when the value is an expression.
    ///
    /// The left side is a variable, a property or an item, and the value of
    /// the assignment is the value that was written.
    fn parse_the_assignment(&mut self, target: Node) -> Result<Node, SyntaxError> {
        let operator = self.next();
        let Some(op) = the_assignment_operator(&operator.value) else {
            return Err(SyntaxError {
                message: format!("expected an assignment operator after {:?}", operator.value),
                line: operator.line,
                column: operator.column,
            });
        };
        let start = self.position;
        let value = self.parse_expression()?;
        let end = self.position;
        let line = target.line();
        Ok(match target {
            Node::Var { name, .. } => {
                self.the_assignment_is_recorded(&name, op, line, operator.column);
                Node::Assign { name, op, value: Box::new(value), line }
            }
            Node::Property { target, name, .. } => {
                Node::AssignProperty { target, name, op, value: Box::new(value), line }
            }
            Node::Index { target, index, .. } => {
                Node::AssignIndex { target, index, op, value: Box::new(value), line }
            }
            _ => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // assignment of an expression is refused at the load, and
                // the text of it is named.
                return Err(SyntaxError {
                    message: format!(
                        "Invalid assignment.\n     Specifically: {} {}",
                        operator.value,
                        the_source_of_the_tokens(self, start, end)
                    ),
                    line,
                    column: operator.column,
                });
            }
        })
    }

    /// The refusal of an assignment whose left side is not a variable.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the message names
    /// the operator and the value that was written.
    fn the_invalid_assignment(&mut self, line: u32) -> Result<SyntaxError, SyntaxError> {
        let operator = self.next();
        let start = self.position;
        // The value is read to name it, and a value that does not parse is
        // the failure that v2 says.
        let _value = self.parse_expression()?;
        let end = self.position;
        Ok(SyntaxError {
            message: format!(
                "Invalid assignment.\n     Specifically: {} {}",
                operator.value,
                the_source_of_the_tokens(self, start, end)
            ),
            line,
            column: operator.column,
        })
    }

    /// The `super` of v2: the base of the class where the method was
    /// defined.
    ///
    /// The keyword is only valid inside a class, and it has to be followed
    /// by `.`, `[` or `(`.
    fn parse_super(&mut self, token: Token) -> Result<Node, SyntaxError> {
        // The token of the keyword is the one before the place the parser
        // is at now.
        let start = self.position - 1;
        let (shape, text) = the_shape_of_a_super(self, start, false);
        if shape == TheShapeOfASuper::Bare {
            // The keyword has to be followed by `.`, `[` or `(`.
            return Err(self.error("Syntax error.\n     Specifically: super"));
        }
        if !self.in_class {
            return Err(self.error(format!(
                "\"Super\" is valid only inside a class.\n     Specifically: {text}"
            )));
        }
        if shape == TheShapeOfASuper::Call {
            // `super()` is `super.call()`.
            self.expect_op("(")?;
            let arguments = self.parse_arguments()?;
            self.expect_op(")")?;
            return Ok(Node::MethodCall {
                target: Box::new(Node::Super { line: token.line }),
                name: "Call".to_string(),
                arguments,
                line: token.line,
            });
        }
        Ok(Node::Super { line: token.line })
    }

    /// A statement that asks the base: `super.__Init()`.
    ///
    /// The `super` alone is a command of v2 at the top level, and it is
    /// refused with the name of the call.
    fn parse_super_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let (shape, text) = the_shape_of_a_super(self, self.position, true);
        if shape == TheShapeOfASuper::Bare {
            if self.in_class {
                return Err(self.error("Syntax error.\n     Specifically: super"));
            }
            return Err(self.error(
                "\"Super\" is valid only inside a class.\n     Specifically: super()",
            ));
        }
        if !self.in_class {
            // Measured on the binary of v2.0.19 on 2026-10-31: the statement
            // is a call, and the text of the call carries the parentheses
            // that the statement does not write.
            return Err(self.error(format!(
                "\"Super\" is valid only inside a class.\n     Specifically: {text}"
            )));
        }
        let node = self.the_expression_of_a_statement()?;
        self.end_of_statement()?;
        Ok(vec![node])
    }

    /// A call in an expression: the name, the arguments, the closing paren.
    fn parse_call(&mut self, name: String, line: u32) -> Result<Node, SyntaxError> {
        self.expect_op("(")?;
        let arguments = self.parse_arguments()?;
        self.expect_op(")")?;
        Ok(Node::Call { name, arguments, line })
    }
}

/// Whether the value is a text that was written as a literal.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the operator that
/// wants a number may not follow a literal text, and a variable that holds
/// one is not a literal. The concatenation of two literal texts is a
/// literal text, and the concatenation of a text and a number is not.
fn the_literal_is_a_text(node: &Node) -> bool {
    match node {
        Node::Literal { value: LiteralValue::Text(_), .. } => true,
        Node::Binary { op: BinaryOp::Concat, left, right, .. } => {
            the_literal_is_a_text(left) && the_literal_is_a_text(right)
        }
        _ => false,
    }
}

/// Whether the operator wants a number.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the addition, the
/// subtraction, the multiplication, the power and the bitwise and. The
/// comparison and the concatenation take anything.
fn the_operator_wants_a_number(op: BinaryOp) -> bool {
    matches!(
        op,
        BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Power | BinaryOp::BitwiseAnd
    )
}

/// The button of a click, a mouse down or a mouse up.
///
/// The number of a token, as v2 reads it.
///
/// The hex of v2 is a number of the base of sixteen, and it wraps the
/// way the machine does: `0xFFFFFFFFFFFFFFFF` is `-1`. A number with a
/// point or with an exponent is a float, and a number without them is a
/// whole number.
///
/// Measured on the binary of v2.0.19 on 2026-10-30.
fn the_number_of(text: &str) -> LiteralValue {
    if let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        return match u64::from_str_radix(digits, 16) {
            Ok(number) => LiteralValue::Integer(number as i64),
            Err(_) => LiteralValue::Integer(0),
        };
    }
    if text.contains('.') || text.contains('e') || text.contains('E') {
        return LiteralValue::Float(text.parse::<f64>().unwrap_or(0.0));
    }
    match text.parse::<i64>() {
        Ok(number) => LiteralValue::Integer(number),
        // A whole number that does not fit in the machine is a float.
        Err(_) => LiteralValue::Float(text.parse::<f64>().unwrap_or(0.0)),
    }
}

/// The operator of an assignment, if the word is one.
///
/// The operators are the ones of v2: `:=` writes, and the others write
/// the value of the operation with the one that is there.
fn the_assignment_operator(value: &str) -> Option<AssignOp> {
    Some(match value {
        ":=" => AssignOp::Set,
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
        _ => return None,
    })
}

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
        let error = parse("Send \"a\"\nif (x) { y := 1 }\n").expect_err("fails");
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
    fn the_rest_of_the_arguments_may_have_no_name() {
        // Measured on the binary of v2.0.19 on 2026-10-31: `f(*)` is a
        // function of v2, and the arguments that it takes go nowhere.
        let statements = parse("f(*) {\n    return 1\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else { panic!("a function") };
        assert_eq!(parameters.len(), 1);
        assert!(parameters[0].variadic);
        assert!(parameters[0].name.is_empty());
    }

    #[test]
    fn a_function_of_the_script_may_be_written_as_a_call_statement() {
        // The script may define the function further down: the check of
        // the name comes at the end of the parse.
        let statements = parse("MyFunc\n\nMyFunc() {\n    return 7\n}").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, .. } if name == "myfunc"));
    }

    #[test]
    fn a_name_that_is_neither_a_built_in_nor_a_function_is_a_call() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // call statement is not refused at the load. It is a variable of
        // the script, and v2 reads it and calls its value.
        let statements = parse("Nonsense 1").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, .. } if name == "nonsense"));
    }

    #[test]
    fn an_unknown_statement_is_a_call_of_a_variable() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // call statement is not refused at the load.
        assert!(parse("Nonsense 1").is_ok());
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
        let statements = parse("Switch x {\n  Case 1:\n    MsgBox \"one\"\n  Default:\n    MsgBox \"other\"\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 2);
        assert!(cases[1].value.is_none());
    }

    #[test]
    fn a_case_of_v2_has_no_brace() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the body of a
        // case is the statements under it, and a brace is not one.
        let error = parse("Switch x {\n  Case 1 {\n    MsgBox \"one\"\n  }\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: Case 1 {"
        );
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
    fn the_body_of_a_function_begins_on_the_next_line() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the body of a
        // block begins on the next line, and the function names the brace.
        let error = parse("f() { x := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"{\"");
    }

    #[test]
    fn the_body_of_a_class_begins_on_the_next_line() {
        let error = parse("class Thing { x := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"{\"\n     Specifically: class Thing { x := 1 }");
    }

    #[test]
    fn the_body_of_a_statement_begins_on_the_next_line() {
        let error = parse("if (x) { y := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"If\"");
    }

    #[test]
    fn the_brace_that_closes_a_block_is_alone_on_its_line() {
        let error = parse("if (x) {\n    y := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"}\"\n     Specifically: y := 1 }");
    }

    #[test]
    fn the_brace_that_opens_a_block_may_be_on_a_line_of_its_own() {
        let statements = parse("f()\n{\n    x := 1\n}").expect("parses");
        assert_eq!(statements.len(), 1);
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
    fn the_keyword_of_a_class_and_the_class_type_are_told_apart() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a name after the
        // word `class` makes a declaration, and anything else makes a call
        // of the class type.
        let statements = parse("class Foo {\n}").expect("parses");
        assert!(matches!(&statements[0], Node::ClassDef { .. }));
        let statements = parse("Class Foo {\n}").expect("parses");
        assert!(matches!(&statements[0], Node::ClassDef { .. }));
        let statements = parse("Class(1)").expect("parses");
        assert!(
            matches!(&statements[0], Node::Call { name, arguments, .. } if name == "class" && arguments.len() == 1)
        );
        let statements = parse("Class").expect("parses");
        assert!(matches!(&statements[0], Node::Call { name, arguments, .. } if name == "class" && arguments.is_empty()));
    }

    #[test]
    fn two_values_side_by_side_need_a_space() {
        // Measured on the binary of v2.0.19 on 2026-10-31: two values side by
        // side are one value, and the space between them is what makes it so.
        let statements = parse("x := \"a\" \"b\"").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { value, .. } if matches!(value.as_ref(), Node::Binary { op: BinaryOp::Concat, .. })));
        let error = parse("x := \"a\"\"b\"").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing space or operator before this.\n     Specifically: \"b\""
        );
        let error = parse("x := \"a\"1").expect_err("does not parse");
        assert_eq!(error.message, "Missing space or operator before this.\n     Specifically: 1");
        // The text of the other kind is named with its own quote.
        let error = parse("x := \"a\"'b'").expect_err("does not parse");
        assert_eq!(error.message, "Missing space or operator before this.\n     Specifically: 'b'");
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
        assert!(matches!(&items[0], Node::Literal { value: LiteralValue::Integer(number), .. } if *number == 1));
        assert!(matches!(&items[2], Node::Literal { value: LiteralValue::Integer(number), .. } if *number == 3));
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
        assert!(matches!(index.as_ref(), Node::Literal { value: LiteralValue::Integer(number), .. } if *number == 1));
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

    #[test]
    fn a_declaration_names_a_variable_and_may_give_it_a_value() {
        let statements = parse("global g, h := 1").expect("parses");
        let Node::Declare { scope, names, .. } = &statements[0] else { panic!("a declaration") };
        assert_eq!(*scope, DeclareKind::Global);
        assert_eq!(names.len(), 2);
        assert_eq!(names[0].name, "g");
        assert!(names[0].value.is_none());
        assert_eq!(names[1].name, "h");
        assert!(names[1].value.is_some());
    }

    #[test]
    fn a_bare_global_is_the_mode_of_the_function() {
        let statements = parse("f() {\n  global\n  g := 1\n}").expect("parses");
        let Node::FunctionDef { declarations, .. } = &statements[0] else { panic!("a definition") };
        assert!(declarations.assume_global);
        assert!(declarations.globals.is_empty());
    }

    #[test]
    fn a_class_with_an_init_and_variables_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: v2 makes the
        // `__Init` that carries the variables, and the two cannot both be
        // there.
        let error = parse("class Thing {\n  value := 1\n  __Init() {\n  }\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Duplicate declaration.\n     Specifically: Thing.Prototype.__Init"
        );
        // The line of the failure is the line of the declaration of the
        // method, the line of the name.
        assert_eq!(error.line, 3);
    }

    #[test]
    fn the_properties_that_come_after_the_init_are_taken() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the `__Init` that
        // carries the properties is made when the properties come before the
        // `__Init`, and the ones that come after it are taken.
        parse("class Thing {\n  __Init() {\n  }\n  value := 1\n}").expect("parses");
    }

    #[test]
    fn a_class_with_a_static_init_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: every class has
        // an automatic initialiser, and a static `__Init` of its own is a
        // duplicate of it.
        let error = parse("class Thing {\n  static __Init() {\n  }\n}").expect_err("does not parse");
        assert_eq!(error.message, "Duplicate declaration.\n     Specifically: Thing.__Init");
        // The line of the failure is the line of the declaration of the
        // method, the line of the name.
        assert_eq!(error.line, 2);
        // The instance `__Init` is another thing: it is the constructor of
        // the class, and v2 takes it.
        parse("class Thing {\n  __Init() {\n  }\n}").expect("parses");
    }

    #[test]
    fn a_class_with_two_methods_of_the_same_name_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a name may be
        // declared only once, and the failure is at the second one.
        let error = parse("class Thing {\n  Foo() {\n  }\n  Foo() {\n  }\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Duplicate declaration.\n     Specifically: Thing.Prototype.Foo"
        );
        assert_eq!(error.line, 4);
    }

    #[test]
    fn a_class_with_two_static_methods_of_the_same_name_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a method of the
        // class and a method of an instance are two names and not one, and
        // the name of a method of the class is the name of the class and the
        // name of it.
        let error =
            parse("class Thing {\n  static Foo() {\n  }\n  static Foo() {\n  }\n}").expect_err("does not parse");
        assert_eq!(error.message, "Duplicate declaration.\n     Specifically: Thing.Foo");
        assert_eq!(error.line, 4);
    }

    #[test]
    fn a_class_with_two_properties_of_the_same_name_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // property is the name of it, and not the name of the class with it.
        let error = parse("class Thing {\n  value := 1\n  value := 2\n}").expect_err("does not parse");
        assert_eq!(error.message, "Duplicate declaration.\n     Specifically: value");
        assert_eq!(error.line, 3);
    }

    #[test]
    fn a_class_with_two_static_properties_of_the_same_name_is_an_error() {
        let error =
            parse("class Thing {\n  static value := 1\n  static value := 2\n}").expect_err("does not parse");
        assert_eq!(error.message, "Duplicate declaration.\n     Specifically: value");
        assert_eq!(error.line, 3);
    }

    #[test]
    fn the_case_of_a_name_does_not_count() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the names of a
        // class are the names in lower case, and the failure names the one
        // that was written.
        let error = parse("class Thing {\n  Foo() {\n  }\n  foo() {\n  }\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Duplicate declaration.\n     Specifically: Thing.Prototype.foo"
        );
    }

    #[test]
    fn a_method_and_a_property_of_the_same_name_are_taken() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the methods and
        // the properties of a class are two tables and not one.
        parse("class Thing {\n  Foo() {\n  }\n  Foo := 1\n}").expect("parses");
        parse("class Thing {\n  Foo := 1\n  Foo() {\n  }\n}").expect("parses");
    }

    #[test]
    fn a_static_method_and_an_instance_method_of_the_same_name_are_taken() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the methods of the
        // class and the methods of an instance are two tables and not one.
        parse("class Thing {\n  Foo() {\n  }\n  static Foo() {\n  }\n}").expect("parses");
    }

    #[test]
    fn the_first_duplicate_of_the_body_is_the_one_that_is_reported() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the duplicates are
        // refused while the body is read, and the first one of them is the
        // one that is reported.
        let error =
            parse("class Thing {\n  Foo() {\n  }\n  Foo() {\n  }\n  static __Init() {\n  }\n}")
                .expect_err("does not parse");
        assert_eq!(
            error.message,
            "Duplicate declaration.\n     Specifically: Thing.Prototype.Foo"
        );
        assert_eq!(error.line, 4);
    }

    #[test]
    fn a_class_with_two_inits_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the `__Init` of an
        // instance is a name of the prototype, and it may be declared once.
        let error =
            parse("class Thing {\n  __Init() {\n  }\n  __Init() {\n  }\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Duplicate declaration.\n     Specifically: Thing.Prototype.__Init"
        );
        assert_eq!(error.line, 4);
    }

    #[test]
    fn a_super_outside_a_class_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-30.
        let error = parse("super.m()").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Super\" is valid only inside a class.\n     Specifically: super.m()"
        );
    }

    #[test]
    fn the_text_of_a_refused_super_is_the_source() {
        let error = parse("super.m(1, 2)").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Super\" is valid only inside a class.\n     Specifically: super.m(1, 2)"
        );
    }

    #[test]
    fn the_text_of_a_refused_super_keeps_the_quotes() {
        let error = parse("super[\"x\"]").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Super\" is valid only inside a class.\n     Specifically: super[\"x\"]"
        );
    }

    #[test]
    fn a_super_that_has_no_member_is_an_error() {
        let error = parse("class C {\n  m() {\n    return super\n  }\n}").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: super");
    }

    #[test]
    fn a_super_alone_outside_a_class_is_the_call_of_the_keyword() {
        let error = parse("super").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Super\" is valid only inside a class.\n     Specifically: super()"
        );
    }

    #[test]
    fn the_super_of_an_expression_names_the_text_that_was_written() {
        // Measured on the binary of v2.0.19 on 2026-10-31: in an expression
        // the text of the `super` is the source, and the parentheses of a
        // call are not there.
        let error = parse("x := super.x").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Super\" is valid only inside a class.\n     Specifically: super.x"
        );
    }

    #[test]
    fn a_dot_with_a_space_before_it_reaches_into_the_value() {
        // Measured on the binary of v2.0.19 on 2026-10-30: the space that
        // comes after the dot is what tells, and the one that comes before
        // it does not matter.
        let statements = parse("y := o .x").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { value, .. } if matches!(&**value, Node::Property { name, .. } if name == "x")));
    }

    #[test]
    fn a_name_that_is_a_statement_calls_the_method() {
        // Measured on the binary of v2.0.19 on 2026-10-29: a name that is a
        // statement is a call, and the parentheses may be left out.
        let statements = parse("o .x").expect("parses");
        assert!(matches!(&statements[0], Node::MethodCall { name, arguments, .. } if name == "x" && arguments.is_empty()));
    }

    #[test]
    fn a_dot_with_a_space_after_it_joins_two_values() {
        let statements = parse("x := a . b").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { value, .. } if matches!(&**value, Node::Binary { op: BinaryOp::Concat, .. })));
    }

    #[test]
    fn a_dot_with_a_space_after_it_and_none_before_it_is_an_error() {
        // v2 refuses the line: "This line does not contain a recognized
        // action.\n     Specifically: o. x := 1".
        let error = parse("o. x := 1").expect_err("does not parse");
        assert!(!error.message.is_empty());
    }

    #[test]
    fn a_bare_global_that_is_not_the_first_line_is_an_error() {
        // Measured on the binary of v2.0.19 on 2026-10-30.
        let error = parse("f() {\n  x := 1\n  global\n}").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected declaration.\n     Specifically: global");
    }

    #[test]
    fn a_bare_local_is_an_error() {
        let error = parse("f() {\n  local\n}").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected declaration.\n     Specifically: local");
    }

    #[test]
    fn a_local_outside_a_function_is_an_error() {
        let error = parse("local l").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected declaration.\n     Specifically: local l");
    }

    #[test]
    fn a_declaration_of_a_parameter_is_an_error() {
        let error = parse("f(p) {\n  global p\n}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This global variable declaration conflicts with an existing parameter.\n     Specifically: p"
        );
    }

    #[test]
    fn the_names_of_a_declaration_are_read_when_the_script_is_read() {
        // A declaration inside a block takes effect for the whole function.
        let statements = parse("f() {\n  if 1 {\n    global g\n  }\n}").expect("parses");
        let Node::FunctionDef { declarations, .. } = &statements[0] else { panic!("a definition") };
        assert!(declarations.globals.contains("g"));
    }

    #[test]
    fn a_label_is_a_statement_of_its_own() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a label is a
        // name and a colon, and it names a line of the script.
        let statements = parse("done:").expect("parses");
        assert!(matches!(&statements[0], Node::Label { name, .. } if name == "done"));
    }

    #[test]
    fn the_name_of_a_label_may_begin_with_a_number() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // label is of letters, of digits and of the underscore, and it may
        // begin with a digit.
        let statements = parse("1abc:").expect("parses");
        assert!(matches!(&statements[0], Node::Label { name, .. } if name == "1abc"));
    }

    #[test]
    fn a_label_with_a_name_that_is_not_one_holds_no_action() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a name with a
        // dash is not the name of a label, and the line that holds it holds
        // no action.
        let error = parse("a-b:").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: a-b:"
        );
    }

    #[test]
    fn a_label_with_a_space_before_the_colon_holds_no_action() {
        let error = parse("done :").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: done :"
        );
    }

    #[test]
    fn a_label_with_other_code_on_its_line_holds_no_action() {
        let error = parse("done: x := 1").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: done: x := 1"
        );
    }

    #[test]
    fn a_label_that_is_written_twice_is_refused() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the load
        // refuses the second label of a name.
        let error = parse("done:\ndone:").expect_err("does not parse");
        assert_eq!(error.message, "Duplicate label.\n     Specifically: done");
        assert_eq!(error.line, 2);
    }

    #[test]
    fn a_label_of_another_scope_may_have_the_name_of_one() {
        let statements = parse("f() {\n  done:\n}\ndone:").expect("parses");
        assert_eq!(statements.len(), 2);
    }

    #[test]
    fn a_goto_needs_a_parameter() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the load
        // refuses the `Goto` that has none.
        let error = parse("Goto").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Goto\" requires at least 1 parameter.\n     Specifically: Goto"
        );
    }

    #[test]
    fn a_goto_takes_one_parameter() {
        let error = parse("Goto a, b").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Goto\" accepts at most 1 parameter.\n     Specifically: Goto a, b"
        );
    }

    #[test]
    fn a_class_may_not_be_declared_inside_a_function() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a function may
        // not contain a class, and the line of the class is the one that is
        // named.
        let error = parse("F() {\n    class C {\n    }\n}\n").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Functions cannot contain classes.\n     Specifically: class C {"
        );
        assert_eq!(error.line, 2);
        // A method is a function too.
        let error =
            parse("class C {\n    Make() {\n        class D {\n        }\n    }\n}\n").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Functions cannot contain classes.\n     Specifically: class D {"
        );
        // A class at the top level is fine, and so is one inside a block.
        parse("class C {\n}\n").expect("parses");
        parse("try {\n    class C {\n    }\n} catch Any as err {\n}\n").expect("parses");
    }

    #[test]
    fn a_class_may_be_declared_inside_a_class() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // class inside a class is the name of the class that holds it and
        // the name of it.
        let statements = parse("class Outer {\n    class Inner {\n    }\n}\n").expect("parses");
        let Node::ClassDef { name, nested, .. } = &statements[0] else { panic!("a class") };
        assert_eq!(name, "Outer");
        assert_eq!(nested.len(), 1);
        let Node::ClassDef { name, .. } = &nested[0] else { panic!("a class") };
        assert_eq!(name, "Outer.Inner");
    }

    #[test]
    fn a_property_of_a_class_may_have_a_getter_and_a_setter() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the body of the
        // getter and of the setter is an expression after the arrow of v2,
        // and a block of statements between braces.
        let statements = parse("class C {\n    value {\n        get => 1\n    }\n}\n").expect("parses");
        let Node::ClassDef { accessors, .. } = &statements[0] else { panic!("a class") };
        assert_eq!(accessors.len(), 1);
        assert_eq!(accessors[0].name, "value");
        assert!(accessors[0].getter.is_some());
        assert!(accessors[0].setter.is_none());
        // The getter without a body is refused.
        let error = parse("class C {\n    value {\n        get\n    }\n}\n").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Not a valid property getter/setter.\n     Specifically: get"
        );
        // The duplicate of a property is refused.
        let error =
            parse("class C {\n    value {\n        get => 1\n    }\n    value {\n        get => 2\n    }\n}\n")
                .expect_err("does not parse");
        assert_eq!(error.message, "Duplicate declaration.\n     Specifically: value");
    }

    #[test]
    fn the_name_of_a_label_is_the_text_of_the_parameter() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the parameter of
        // a `Goto` is not an expression: it is the name of a label, and it
        // is read as the text of it.
        let statements = parse("Goto \"done\"").expect("parses");
        assert!(matches!(&statements[0], Node::Goto { name, .. } if name == "\"done\""));
        let statements = parse("Goto(done)").expect("parses");
        assert!(matches!(&statements[0], Node::Goto { name, .. } if name == "done"));
    }
