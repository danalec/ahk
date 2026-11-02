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
    AccessBody, AccessDef, AssignOp, BinaryOp, Branch, Case, Catch, DeclareKind, Declared, Declarations, DynamicPiece,
    LiteralValue, LoopKind, MemberName, MethodDef, Node, Parameter, PropertyDef, UnaryOp,
};
use crate::lexer::{SyntaxError, TheRefusal, Token, TokenKind};

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
    matches!(node, Node::Var { .. } | Node::Property { .. } | Node::Index { .. } | Node::DynamicName { .. })
}

/// The source of the head of a statement, the way v2 writes it.
///
/// The text stops before the body, and the space that was written before
/// the body is part of it: the head of `Loop Read {` is "Loop Read ".
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the command that
/// wants a parameter that is not there names the text of its head.
fn the_source_of_the_head(parser: &Parser, head: usize, body: usize) -> String {
    let mut text = the_source_of_the_tokens(parser, head, body);
    if parser.tokens.get(body).map(|token| token.spaced).unwrap_or(false) {
        text.push(' ');
    }
    text
}

/// Whether the kind is one of the loops of v2.
///
/// The loop of a count and the loops of a kind are the same statement,
/// and the name of the kind is part of it.
fn the_loop_of_the_kind(kind: &str) -> bool {
    matches!(kind, "Loop" | "Loop Files" | "Loop Reg" | "Loop Read" | "Loop Parse")
}

/// The name of the statement of the loop, as the messages of v2 write it.
///
/// The loop of a count is "Loop", and the loop of a kind is the kind
/// with it: "Loop Read".
fn the_name_of_the_loop(kind: &str) -> String {
    if kind.is_empty() {
        "Loop".to_string()
    } else {
        format!("Loop {kind}")
    }
}

/// The number of the parameters that a loop of a kind takes.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: `Loop Read`,
/// `Loop Files` and `Loop Reg` take two parameters, and `Loop Parse`
/// takes three.
fn the_number_of_parameters_of_a_loop(kind: LoopKind) -> usize {
    match kind {
        LoopKind::Parse => 3,
        _ => 2,
    }
}

/// The source of a token and of what comes after it, in its line.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the value that came
/// too close is named, and the rest of the line is named with it. The
/// newline that ends the line is not part of it: the block that the error
/// printer writes ends with a newline of its own, and the text that the
/// error names is the block without it.
fn the_source_of_the_rest_of_the_line(parser: &Parser, start: usize) -> String {
    let end = the_end_of_the_line_of(parser, start);
    the_source_of_the_tokens(parser, start, end)
}

/// The place where the line of a token ends.
///
/// The newline that ends the line is not a part of it.
fn the_end_of_the_line_of(parser: &Parser, start: usize) -> usize {
    let mut end = start;
    while end < parser.tokens.len()
        && parser.tokens[end].line == parser.tokens[start].line
        && parser.tokens[end].kind != TokenKind::Newline
    {
        end += 1;
    }
    end
}

/// The message of a failure of a command-style call.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the command of v2 is a
/// call whose parentheses are not written, and the text that names the
/// operator carries the one that closes it.
fn the_message_of_a_command(mut error: SyntaxError) -> SyntaxError {
    // The bracket that is not there is not a failure of the parameters, and
    // it is not named with the parenthesis of the call.
    //
    // The line that holds no action is named with the line itself, and the
    // parenthesis of the call is not a part of it.
    if !error.message.starts_with("Missing \"")
        && !error.message.starts_with("This line does not contain a recognized action.")
        && error.message.contains("\n     Specifically: ")
    {
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
    the_source_with_the_newlines_as_spaces(parser, start, end)
}

/// The source of a range of tokens, with the newlines as spaces.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a definition may be
/// written in more than one line, and v2 writes the source of it in one:
/// the newline is a space. The refusal of an illegal character writes the
/// text of it in one line too.
fn the_source_with_the_newlines_as_spaces(parser: &Parser, start: usize, end: usize) -> String {
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

/// The text that the refusal of the brace that closes a block names.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the text is the line
/// of the brace that closes the block, and when the brace that opens it is
/// in that line too, the text begins after it.
fn the_text_of_the_line_of_the_closing_brace(parser: &Parser, open: usize, close: usize) -> String {
    let line = parser.tokens[close].line;
    if parser.tokens[open].line != line {
        return the_source_of_the_line(parser, line);
    }
    let end = the_end_of_the_line_of(parser, open + 1);
    the_source_of_the_tokens(parser, open + 1, end)
}

/// Whether the token is a character that may not be in the script.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
/// semicolon are the two characters of v2 that are illegal where they are
/// not a directive and not a comment.
fn the_token_is_an_illegal_character(token: &Token) -> bool {
    token.kind == TokenKind::Op && matches!(token.value.as_str(), "#" | ";")
}

/// The text that the refusal of an illegal character names.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the text begins at the
/// character. The one that is an illegal character in an expression ends
/// where the expression ends: at the bracket that closes it, or at the end
/// of the line. The one that is a line that holds no action ends at the end
/// of the line.
fn the_text_of_the_illegal_character(parser: &Parser, at: usize, in_an_expression: bool) -> String {
    let end = if in_an_expression {
        the_end_of_the_expression_of_the_illegal_character(parser, at)
    } else {
        the_end_of_the_line_of(parser, at)
    };
    the_source_with_the_newlines_as_spaces(parser, at, end)
}

/// The end of the expression that holds an illegal character.
///
/// The bracket that is open where the character is, and the bracket that
/// closes it, are the ends of the expression.
fn the_end_of_the_expression_of_the_illegal_character(parser: &Parser, at: usize) -> usize {
    let mut depth = 0usize;
    let mut open = None;
    for index in (0..at).rev() {
        let token = &parser.tokens[index];
        if token.kind != TokenKind::Op {
            continue;
        }
        match token.value.as_str() {
            ")" | "]" | "}" => depth += 1,
            "(" | "[" | "{" => {
                if depth == 0 {
                    open = Some(index);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let Some(open) = open else {
        return the_end_of_the_line_of(parser, at);
    };
    let mut depth = 0usize;
    for index in open..parser.tokens.len() {
        let token = &parser.tokens[index];
        if token.kind != TokenKind::Op {
            continue;
        }
        match token.value.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth -= 1;
                if depth == 0 {
                    return index + 1;
                }
            }
            _ => {}
        }
    }
    the_end_of_the_line_of(parser, at)
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

/// The gotos of a scope, the way the script wrote them.
///
/// The gotos of the blocks of the scope are the gotos of the scope, and the
/// body of a function that is written inside the scope belongs to that
/// function and not to the scope around it.
fn the_gotos_of_a_scope(body: &[Node]) -> Vec<(String, u32)> {
    let mut gotos = Vec::new();
    collect_the_gotos_of_a_scope(body, &mut gotos);
    gotos
}

/// The gotos of the blocks of a scope.
fn collect_the_gotos_of_a_scope(body: &[Node], gotos: &mut Vec<(String, u32)>) {
    for statement in body {
        match statement {
            Node::Goto { name, line } => gotos.push((name.clone(), *line)),
            Node::If { branches, otherwise, .. } => {
                for branch in branches {
                    collect_the_gotos_of_a_scope(&branch.body, gotos);
                }
                collect_the_gotos_of_a_scope(otherwise, gotos);
            }
            Node::Switch { cases, .. } => {
                for case in cases {
                    collect_the_gotos_of_a_scope(&case.body, gotos);
                }
            }
            Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
                collect_the_gotos_of_a_scope(body, gotos);
            }
            Node::Try { body, catches, otherwise, finally, .. } => {
                collect_the_gotos_of_a_scope(body, gotos);
                for catch in catches {
                    collect_the_gotos_of_a_scope(&catch.body, gotos);
                }
                collect_the_gotos_of_a_scope(otherwise, gotos);
                collect_the_gotos_of_a_scope(finally, gotos);
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
///
/// The script is a script of the v2, the dialect of the tree.
pub fn parse(source: &str) -> Result<Vec<Node>, SyntaxError> {
    let tokens = crate::lexer::tokenize(source)?;
    Parser::new(source, tokens).parse_script()
}

/// Parse a script into statements, in the dialect of this name.
///
/// The v2 is the dialect of the tree, and it is the one of every script
/// that does not ask for another one. The v1 has a grammar of its own.
pub fn parse_with(
    source: &str,
    dialect: &'static dyn crate::dialect::Dialect,
) -> Result<Vec<Node>, SyntaxError> {
    let tokens = crate::lexer::tokenize_with(source, dialect)?;
    Parser::with_dialect(source, tokens, dialect).parse_script()
}

/// The members of a class: the methods, the properties, the properties
/// that have a getter or a setter, and the classes that it holds.
struct TheMembersOfAClass {
    methods: Vec<MethodDef>,
    properties: Vec<PropertyDef>,
    accessors: Vec<AccessDef>,
    nested: Vec<Node>,
}

/// A call of the script whose arity is checked at the end of the parse.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses the
/// call of a function at the load, and the name of a function of the script
/// may be defined after the call: the arity of it is known only when the
/// parse is over.
struct TheCallOfTheScript {
    /// The name of the function, in lower case.
    name: String,
    /// The number of the arguments of the call.
    count: usize,
    /// The text of the call, the way the script wrote it.
    source: String,
    /// The place of the call in the script.
    line: u32,
    column: u32,
}

/// The recursive descent parser.
pub struct Parser {    tokens: Vec<Token>,
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
    /// The arity of the functions of the script: the name, the least number
    /// of the arguments and the most.
    ///
    /// The v1 refuses the call of a function at the load, and the number of
    /// the arguments of it is one of the things that it refuses there.
    arities: BTreeMap<String, (usize, usize)>,
    /// The calls of the script that the v1 refuses at the load.
    ///
    /// The name of a function of the script may be defined after the call, so
    /// the calls wait for the end of the parse.
    calls: Vec<TheCallOfTheScript>,
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
    /// The names that the script is building, and the ones that are open.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the percent sign
    /// that closes a name that the script builds is the end of it, and not
    /// the beginning of another one: `%x%` is one name.
    derefs: usize,
    /// The loops that the parser is inside.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the `break` and the
    /// `continue` need a loop around them, and the script is refused at the
    /// load when there is none.
    loops: usize,
    /// The dialect that the parser speaks.
    ///
    /// The v2 is the dialect of the tree, and it is the one of every script
    /// that does not ask for another one. The v1 has a grammar of its own:
    /// the legacy command, the equal sign that assigns, and the body of a
    /// single statement.
    dialect: &'static dyn crate::dialect::Dialect,
    /// The lines of the source, the way the script wrote them.
    ///
    /// The text of the legacy dialect is the text of the script, and the
    /// tokens know whether a space came before them and not how many spaces
    /// there were: the lines are what keep the spaces of the script.
    lines: Vec<String>,
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
    /// A parser over the tokens, in the dialect of the tree.
    ///
    /// The source is the text that the tokens came from: the text of the
    /// legacy dialect is the text of the script, and the tokens know whether
    /// a space came before them and not how many spaces there were.
    pub fn new(source: &str, tokens: Vec<Token>) -> Self {
        Self::with_dialect(source, tokens, crate::dialect::v2())
    }

    /// A parser over the tokens, in the dialect of this name.
    ///
    /// The source is the text that the tokens came from: the text of the
    /// legacy dialect is the text of the script, and the tokens know whether
    /// a space came before them and not how many spaces there were.
    pub fn with_dialect(
        source: &str,
        tokens: Vec<Token>,
        dialect: &'static dyn crate::dialect::Dialect,
    ) -> Self {
        // Measured on the binary of v2.0.19 on 2026-10-31: the script may
        // begin with the byte order mark, and the mark is not a character of
        // it. The lexer takes the mark away, and the lines of the source are
        // the ones that the lexer counted.
        let source = source.strip_prefix('\u{feff}').unwrap_or(source);
        Self {
            tokens,
            position: 0,
            defined: BTreeSet::new(),
            classes: BTreeMap::new(),
            functions: BTreeSet::new(),
            arities: BTreeMap::new(),
            calls: Vec::new(),
            assignments: vec![Vec::new()],
            parameters: Vec::new(),
            in_class: false,
            in_function: false,
            pending_literal: None,
            expressions: Vec::new(),
            statement_start: 0,
            bases: Vec::new(),
            derefs: 0,
            loops: 0,
            lines: source.split('\n').map(|line| line.to_string()).collect(),
            dialect,
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

    /// Whether the name that comes next is the one that is asked for.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the `default` and
    /// the `extends` are names in v2 and not words of the dialect, and the
    /// dialect tells them apart by what comes with them.
    fn at_the_name(&self, name: &str) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Name && token.value.eq_ignore_ascii_case(name)
    }

    /// Whether a colon comes after the token that the parser is at.
    fn a_colon_comes_after(&self) -> bool {
        self.tokens
            .get(self.position + 1)
            .map(|token| token.kind == TokenKind::Op && token.value == ":")
            .unwrap_or(false)
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
        let at = self.position;
        let token = self.next();
        // Measured on the binary of v2.0.19 on 2026-11-01: the character
        // that is where a bracket was expected is an illegal character in
        // an expression, and the text of the expression is named.
        if token.kind == TokenKind::Op && the_token_is_an_illegal_character(&token) {
            return Err(self.the_error_of_the_illegal_character(at, true));
        }
        if token.kind != TokenKind::Op || token.value != operator {
            // Measured on the binary of v2.0.19 on 2026-10-31: the bracket
            // that is not there is named, and the statement that holds it is
            // named with it. A brace that closes a block is named before the
            // bracket.
            let named = if matches!(operator, ")" | "]")
                && token.kind == TokenKind::Op
                && token.value == "}"
            {
                format!("Missing {operator:?} before \"}}\"")
            } else {
                format!("Missing {operator:?}")
            };
            return Err(SyntaxError {
                message: format!(
                    "{named}\n     Specifically: {}",
                    self.the_source_of_the_statement_of_the_line(token.line)
                ),
                line: token.line,
                column: token.column,
            });
        }
        Ok(token)
    }

    /// The failure of the bracket that is not closed.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the bracket that is
    /// not closed is the failure, and not the operand that is not there
    /// inside it: `x := f(1,` is a "Missing ")" and not a "Missing
    /// operand.". A brace that closes a block is named before the bracket.
    fn the_error_of_the_bracket(&self, error: SyntaxError, closing: &str) -> SyntaxError {
        let (message, line) = if error.message.starts_with("Missing operand.") {
            (format!("Missing {closing:?}"), error.line)
        } else if error.message.starts_with("Unexpected \"}\"") {
            // The brace that closes the block is on the line of the failure,
            // and the text that is named is the statement and that line.
            let line = self
                .tokens
                .get(self.position.saturating_sub(1))
                .map(|token| token.line)
                .unwrap_or(error.line);
            (format!("Missing {closing:?} before \"}}\""), line)
        } else {
            return error;
        };
        SyntaxError {
            message: format!(
                "{message}\n     Specifically: {}",
                self.the_source_of_the_statement_of_the_line(line)
            ),
            line,
            column: error.column,
        }
    }

    /// The failure of the bracket that is not closed, at a place of its own.
    fn the_error_of_a_missing_bracket(&self, closing: &str, line: u32, column: u32) -> SyntaxError {
        SyntaxError {
            message: format!(
                "Missing {closing:?}\n     Specifically: {}",
                self.the_source_of_the_statement_of_the_line(line)
            ),
            line,
            column,
        }
    }

    /// The statement that holds the failure, and the line that it is in.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text that is
    /// named is the statement and the line of the failure, and the newline
    /// between them is a space.
    fn the_source_of_the_statement_of_the_line(&self, line: u32) -> String {
        let mut end = self.statement_start;
        while end < self.tokens.len()
            && self.tokens[end].line <= line
            && self.tokens[end].kind != TokenKind::Eof
        {
            end += 1;
        }
        the_source_of_a_definition(self, self.statement_start, end)
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

    /// The expression of a statement that begins with a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the expression of
    /// a statement may begin with a value that is already read -- the call
    /// of a statement, or the member of one -- and the operators and the
    /// concatenation of v2 that follow it are a part of it. The value of
    /// the expression is thrown away.
    fn the_expression_after(&mut self, value: Node) -> Result<Node, SyntaxError> {
        self.the_expression_begins();
        let value = self.parse_binary_after(value, precedence::COMMA)?;
        self.the_expression_is_done(self.position, false)?;
        Ok(value)
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
        self.arities.clear();
        self.calls.clear();
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
        // The v1 refuses the call of a function at the load, and the arity of
        // a function of the script is known only now.
        self.the_calls_of_the_v1_are_checked()?;
        Ok(statements)
    }

    /// The calls of the v1 are checked at the end of the parse.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the name that is
    /// not a function is refused with "Call to nonexistent function." and it
    /// names the call as it was written -- the spaces of the script are in
    /// it -- and the number of the arguments that is wrong is refused with
    /// "Too few parameters passed to function." or with "Too many parameters
    /// passed to function.", and it names it too.
    ///
    /// A command of the v1 is not a function: `FileAppend("hello", "*")` is
    /// a call to a name that does not exist. The calls are checked in the
    /// order of the script.
    fn the_calls_of_the_v1_are_checked(&self) -> Result<(), SyntaxError> {
        if !self.dialect.refuses_the_call_of_a_function_at_the_load() {
            return Ok(());
        }
        for call in &self.calls {
            let wanted = &call.name;
            // The function of the dialect: the arity is in the table.
            let function = self
                .dialect
                .builtins()
                .iter()
                .find(|builtin| {
                    builtin.name.to_lowercase() == *wanted
                        && builtin.kind == crate::builtins::BuiltinKind::Function
                });
            if let Some(function) = function {
                if call.count < function.min as usize {
                    return Err(self.the_error_of_the_arity_of_the_call(
                        call,
                        "Too few parameters passed to function.",
                    ));
                }
                if call.count > function.max as usize {
                    return Err(self.the_error_of_the_arity_of_the_call(
                        call,
                        "Too many parameters passed to function.",
                    ));
                }
                continue;
            }
            // The function of the script: the arity is the one of the
            // parameters of it.
            if let Some((required, most)) = self.arities.get(wanted) {
                if call.count < *required {
                    return Err(self.the_error_of_the_arity_of_the_call(
                        call,
                        "Too few parameters passed to function.",
                    ));
                }
                if call.count > *most {
                    return Err(self.the_error_of_the_arity_of_the_call(
                        call,
                        "Too many parameters passed to function.",
                    ));
                }
                continue;
            }
            // The name that is not a function of the dialect and not a
            // function of the script.
            return Err(SyntaxError {
                message: format!(
                    "Call to nonexistent function.\n     Specifically: {}",
                    call.source
                ),
                line: call.line,
                column: call.column,
            });
        }
        Ok(())
    }

    /// The refusal of the number of the arguments of a call of the v1.
    fn the_error_of_the_arity_of_the_call(
        &self,
        call: &TheCallOfTheScript,
        message: &str,
    ) -> SyntaxError {
        SyntaxError {
            message: format!("{message}\n     Specifically: {}", call.source),
            line: call.line,
            column: call.column,
        }
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
            if crate::classes::is_a_name_of_a_class(base) && self.dialect.the_base_may_be_a_class_of_the_runtime()
            {
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
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    /// function of the machine may not be the output variable of an
    /// assignment either, and the refusal is the same with "Func" in the
    /// place of "Class".
    fn check_the_assignments(&self, assignments: Vec<Assignment>) -> Result<(), SyntaxError> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 takes the name of a class and the name of a function of the
        // machine as the name of a variable, and it refuses no assignment of
        // the top level.
        if self.dialect.the_name_of_a_class_or_of_a_function_may_be_assigned() {
            return Ok(());
        }
        for assignment in assignments {
            let key = assignment.name.to_lowercase();
            if let Some(class) = self.the_name_of_the_class(&key) {
                let message = match assignment.op {
                    AssignOp::Set => format!(
                        "This Class cannot be used as an output variable.\n     Specifically: {class}"
                    ),
                    _ => format!("This Class cannot be assigned a value.\n     Specifically: {class}"),
                };
                return Err(SyntaxError { message, line: assignment.line, column: assignment.column });
            }
            if crate::builtins::find(&key).is_some() {
                let message = match assignment.op {
                    AssignOp::Set => format!(
                        "This Func cannot be used as an output variable.\n     Specifically: {}",
                        assignment.name
                    ),
                    _ => format!(
                        "This Func cannot be assigned a value.\n     Specifically: {}",
                        assignment.name
                    ),
                };
                return Err(SyntaxError { message, line: assignment.line, column: assignment.column });
            }
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
                // the v1 refuses the class that is declared twice with
                // "Duplicate class definition.\n     Specifically: C", and the
                // name is the one that was written.
                if self.dialect.the_class_that_is_declared_twice_has_its_own_words() {
                    return Err(SyntaxError {
                        message: format!("Duplicate class definition.\n     Specifically: {name}"),
                        line,
                        column,
                    });
                }
                Some("This class declaration conflicts with an existing Class.")
            } else if self.functions.contains(&key) {
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
                // the v1 takes a class and a function of the same name.
                if self.dialect.a_class_and_a_function_may_have_the_same_name() {
                    None
                } else {
                    Some("This class declaration conflicts with an existing Func.")
                }
            } else {
                None
            }
        } else if self.classes.contains_key(&key) {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
            // v1 takes a class and a function of the same name.
            if self.dialect.a_class_and_a_function_may_have_the_same_name() {
                None
            } else {
                Some("This function declaration conflicts with an existing Class.")
            }
        } else if self.functions.contains(&key) {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
            // v1 refuses the function that is declared twice with "Duplicate
            // function definition.\n     Specifically: C()", and the name is
            // written the way the script spelled it and with the parenthesis
            // of the call.
            if self.dialect.the_function_that_is_declared_twice_has_its_own_words() {
                return Err(SyntaxError {
                    message: format!("Duplicate function definition.\n     Specifically: {name}()"),
                    line,
                    column,
                });
            }
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
        // semicolon that are not a directive and not a comment are illegal
        // characters in an expression, and the text of the expression is
        // named. The v2 looks at them before it says that the body of the
        // block is not where it belongs.
        if self.at_op(&["#", ";"]) {
            return Err(self.the_error_of_the_illegal_character(self.position, true));
        }
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
        let open = self.position;
        self.expect_op("{")?;
        // Measured on the binary of v2.0.19 on 2026-11-01: the character that
        // is glued to the brace that opens the body of a block is an illegal
        // character in an expression, and the text of the body is named.
        if self.at_op(&["#", ";"]) {
            return Err(self.the_error_of_the_illegal_character(self.position, true));
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: the content of
        // the block may begin on the line of the brace that opens it, and
        // the construct refuses it when the brace is not at the start of
        // its line. The `try`, the `else` and the `finally` take the body
        // on the line of the construct, and they take it there.
        let open_at_the_start_of_its_line = self.the_brace_at_is_at_the_start_of_its_line(open);
        let content_on_the_line_of_the_brace = !self.the_body_begins_on_the_next_line();
        if content_on_the_line_of_the_brace
            && !open_at_the_start_of_its_line
            && !Self::the_content_may_begin_on_the_line_of_the_brace(kind)
        {
            return Err(self.error(self.the_message_of_the_opening_brace(kind, line)));
        }
        let mut body = Vec::new();
        self.skip_newlines();
        // Measured on the binary of v2.0.19 on 2026-11-01: the block that is
        // empty may have the brace that closes it where it is.
        let empty = self.at_op(&["}"]);
        // The statement that is being read is the one of the body, and
        // the block gives its place back when it is done.
        let statement_start = self.statement_start;
        loop {
            let token = self.peek();
            if token.kind == TokenKind::Eof {
                // Measured on the binary of v2.0.19 on 2026-11-01: the
                // brace that is not closed is named, and the block that is
                // empty names the construct that opened it. The block that
                // has content names nothing.
                if body.is_empty() {
                    return Err(SyntaxError {
                        message: format!(
                            "Missing \"}}\"\n     Specifically: {}",
                            the_source_of_the_line(self, line)
                        ),
                        line: token.line,
                        column: token.column,
                    });
                }
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
                    line: token.line,
                    column: token.column,
                });
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                // Measured on the binary of v2.0.19 on 2026-11-01: the brace
                // that closes the block is at the start of its line. The
                // block that is on the line of the construct is the one
                // exception: the v2 says that the brace is missing, and the
                // block that is empty is not a block at all -- the brace
                // that closes it is a statement that holds no action.
                if !self.the_brace_is_at_the_start_of_its_line() {
                    if !open_at_the_start_of_its_line && content_on_the_line_of_the_brace {
                        if empty {
                            return Err(self.error(
                                "This line does not contain a recognized action.\n     Specifically: }"
                                    .to_string(),
                            ));
                        }
                        return Err(SyntaxError {
                            message: "Missing \"}\"".into(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    if !empty {
                        self.the_closing_brace_is_refused(open)?;
                    }
                }
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
    /// Measured on the binary of v2.0.19 on 2026-11-01: the body is one
    /// statement, and the function and the method need the braces. Every
    /// construct needs a body, and the one that has none says so in its
    /// own words.
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

    /// The end of a list that is written between parentheses.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the text that the v2
    /// names when the list of a `for` is refused ends at the parenthesis that
    /// closes it.
    fn the_end_of_the_parenthesis(&self, from: usize) -> usize {
        let mut depth = 0usize;
        let mut at = from;
        while let Some(token) = self.tokens.get(at) {
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                break;
            }
            if token.value == "(" {
                depth += 1;
            }
            if token.value == ")" {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            at += 1;
        }
        at
    }

    /// The parenthesis of the condition of a construct, when it is empty.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the body
    /// of the construct before it reads the condition, and the refusal of a
    /// condition that is empty comes after the one of the body.
    fn the_parenthesis_of_a_condition_is_empty(&self) -> Option<Token> {
        if !self.at_op(&["("]) {
            return None;
        }
        let closing = self.tokens.get(self.position + 1)?;
        if closing.value != ")" {
            return None;
        }
        Some(closing.clone())
    }

    /// The refusal of a condition that is empty.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the text that is
    /// named is the closing parenthesis of the condition.
    fn the_refusal_of_an_empty_parenthesis(&self, closing: &Token) -> SyntaxError {
        SyntaxError {
            message: "Syntax error.\n     Specifically: )".to_string(),
            line: closing.line,
            column: closing.column,
        }
    }

    /// The parenthesis that the construct takes and ignores.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the v2 takes the
    /// parenthesis of a break and of a continue, and it ignores what is
    /// inside it.
    fn the_parenthesis_that_is_ignored(&mut self) {
        if !self.at_op(&["("]) {
            return;
        }
        let mut depth = 0usize;
        loop {
            let token = self.peek().clone();
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                return;
            }
            if token.value == "(" {
                depth += 1;
            }
            if token.value == ")" {
                depth -= 1;
            }
            self.next();
            if depth == 0 {
                return;
            }
        }
    }

    /// The refusal of a construct that takes no parenthesis.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the try, the finally
    /// and the else take no parenthesis, and the dialect names the statement
    /// that carries it.
    fn the_construct_takes_no_parenthesis(&self, line: u32) -> Result<(), SyntaxError> {
        if !self.at_op(&["("]) {
            return Ok(());
        }
        Err(SyntaxError {
            message: format!(
                "Syntax error.\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, self.position - 1)
            ),
            line,
            column: self.tokens.get(self.position - 1).map(|token| token.column).unwrap_or(1),
        })
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
        matches!(kind, "If" | "While" | "For" | "Catch") || the_loop_of_the_kind(kind)
    }

    /// Whether the content of a block may begin on the line of the brace
    /// that opens it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the `try`, the
    /// `else` and the `finally` take the body on the line of the construct,
    /// and the content of the block may be there too. Every other
    /// construct refuses it when the brace is not at the start of its line.
    fn the_content_may_begin_on_the_line_of_the_brace(kind: &str) -> bool {
        matches!(kind, "Try" | "Else" | "Finally")
    }

    /// Whether the block needs a body.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the `if`, the
    /// `while`, the `loop`, the `for`, the `try`, the `catch`, the `else`
    /// and the `finally` need one, and the construct that has none says
    /// so in its own words: "Unexpected \"Catch\"", "Unexpected \"Else\""
    /// and "Unexpected \"Finally\"".
    fn the_block_needs_a_body(kind: &str) -> bool {
        matches!(kind, "If" | "While" | "For" | "Try" | "Catch" | "Else" | "Finally")
            || the_loop_of_the_kind(kind)
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

    /// A block that opens a line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the brace that opens
    /// a line is a block of its own, and the statements between the braces
    /// run. The content may be on the line of the brace that opens it, and
    /// the brace that closes it is alone on its line -- unless the block is
    /// empty, and then the brace may be written where it is.
    fn parse_block_that_opens_a_line(&mut self) -> Result<Vec<Node>, SyntaxError> {
        // The brace that opens the block, and the one that closes it.
        let open = self.position;
        if let Some(close) = self.the_closing_brace_of_a_block_that_opens_a_line(open) {
            // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
            // closes the block is alone on its line, and the v2 says so
            // before it reads the content of the block. The block that is
            // empty is the one exception: the brace may be written where it
            // is.
            let empty = self
                .tokens
                .get(open + 1)
                .map(|token| token.kind == TokenKind::Op && token.value == "}")
                .unwrap_or(false);
            if !empty && !self.the_brace_at_is_at_the_start_of_its_line(close) {
                // Measured on the binary of v2.0.19 on 2026-11-01: the text
                // that is named is the line of the brace that closes the
                // block, and when the brace that opens it is in that line
                // too, the text begins after it.
                let token = self.tokens[close].clone();
                let text = the_text_of_the_line_of_the_closing_brace(self, open, close);
                return Err(SyntaxError {
                    message: format!("Unexpected \"}}\"\n     Specifically: {text}"),
                    line: token.line,
                    column: token.column,
                });
            }
        }
        self.next();
        let mut body = Vec::new();
        // The content may be on the line of the brace that opens it.
        if self.peek().kind == TokenKind::Newline {
            self.skip_newlines();
        }
        loop {
            let token = self.peek().clone();
            if token.kind == TokenKind::Eof {
                // Measured on the binary of v2.0.19 on 2026-11-01: the brace
                // that is not closed is named, and the line that is named is
                // the line of the brace that opens the block.
                let opener = self.tokens[open].clone();
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
                    line: opener.line,
                    column: opener.column,
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

    /// The place of the brace that closes the block that opens a line.
    ///
    /// The braces are counted, and the block is closed when the count goes
    /// back to zero, the way the literal of an object is counted.
    fn the_closing_brace_of_a_block_that_opens_a_line(&self, open: usize) -> Option<usize> {
        let mut depth = 1;
        for (at, token) in self.tokens.iter().enumerate().skip(open + 1) {
            if token.kind == TokenKind::Op && token.value == "{" {
                depth += 1;
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            if token.kind == TokenKind::Eof {
                return None;
            }
        }
        None
    }

    /// Whether the brace at that place is at the start of its line.
    fn the_brace_at_is_at_the_start_of_its_line(&self, at: usize) -> bool {
        match at.checked_sub(1).and_then(|before| self.tokens.get(before)) {
            Some(token) => token.kind == TokenKind::Newline,
            None => true,
        }
    }

    /// The refusal of a brace that is not at the start of its line.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the text that is
    /// named is the line of the brace that closes the block, and it begins
    /// after the brace that opens it when the two are in the same line.
    fn the_closing_brace_is_refused(&self, open: usize) -> Result<(), SyntaxError> {
        if self.the_brace_is_at_the_start_of_its_line() {
            return Ok(());
        }
        Err(self.error(format!(
            "Unexpected \"}}\"\n     Specifically: {}",
            the_text_of_the_line_of_the_closing_brace(self, open, self.position)
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
            "function" => "Unexpected \"{\"".to_string(),
            "class" => format!(
                "Missing \"{{\"\n     Specifically: {}",
                the_source_of_the_line(self, line)
            ),
            "method" => format!(
                "Not a valid method, class or property definition.\n     Specifically: {}",
                the_source_of_the_line(self, line)
            ),
            // The loop of a kind names the loop and not the kind.
            kind if the_loop_of_the_kind(kind) => "Unexpected \"Loop\"".to_string(),
            "If" | "While" | "For" | "Switch" => format!("Unexpected {kind:?}"),
            _ => "Missing \"}\"".to_string(),
        }
    }

    /// The message of v2 when a statement begins with an operator.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a statement that
    /// begins with an operator is a line that holds no action, and the text
    /// that is named is the line itself. The operators that want a word of
    /// their own are named apart: the equal wants the operator of an
    /// assignment, the assignment wants a name on its left, the comma
    /// separates the parameters of a call, the question asks a question that
    /// is not there, and the double colon is the separator of a hotkey.
    fn the_message_of_the_operator_at_the_start(&self, token: &Token) -> String {
        let line = the_source_of_the_line(self, token.line);
        match token.value.as_str() {
            // The dot is the member of a value that is not there.
            "." => self.the_message_of_the_dot_at_the_start(),
            // The equal wants the operator of an assignment.
            "=" | "=>" => format!("Syntax error. Did you mean to use \":=\"?\n     Specifically: {line}"),
            // The assignment wants a name on its left.
            ":=" => "\":=\" requires that parameter #1 be non-blank.".to_string(),
            // The compound assignment writes to a name that is not there.
            op if the_assignment_operator(op).is_some() => {
                format!("Invalid assignment.\n     Specifically: {line}")
            }
            // The comma separates the parameters of a call.
            "," => format!(
                "Function calls require a space or \"(\".  Use comma only between parameters.\n     Specifically: {line}"
            ),
            // The question asks a question that is not there.
            "?" | "??" => format!("Syntax error.\n     Specifically: {line}"),
            // The double colon is the separator of a hotkey.
            "::" => format!("Invalid hotkey.\n     Specifically: {line}"),
            // Measured on the binary of v2.0.19 on 2026-11-01: the hash and
            // the semicolon that are not a directive and not a comment are a
            // line that holds no action, and the text that is named begins
            // at the character.
            "#" | ";" => format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                the_text_of_the_illegal_character(self, self.position, false)
            ),
            // The other operators are not a statement of their own.
            _ => format!("This line does not contain a recognized action.\n     Specifically: {line}"),
        }
    }

    /// The message of v2 when a statement begins with a dot.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the dot with a name
    /// after it is the member of a value that is not there, and the v2 names
    /// the call that it wanted to make -- `.x` is named `.x()`. The dot
    /// alone is a line that holds no action.
    fn the_message_of_the_dot_at_the_start(&self) -> String {
        let line = the_source_of_the_line(self, self.peek().line);
        // The name after the dot is the member that is wanted.
        let member = self.tokens.get(self.position + 1);
        let is_a_member = matches!(member, Some(token) if token.kind == TokenKind::Name && !token.spaced);
        if !is_a_member {
            return format!("This line does not contain a recognized action.\n     Specifically: {line}");
        }
        // The statement that carries an assignment is named as it was written.
        if self.the_line_has_an_assignment() {
            return format!("Syntax error.\n     Specifically: {line}");
        }
        format!("Syntax error.\n     Specifically: {}", self.the_call_of_the_dot())
    }

    /// Whether the line of the statement carries an assignment.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the statement that
    /// is an assignment is named as the script wrote it, and the one that is
    /// a call is named with its parentheses.
    fn the_line_has_an_assignment(&self) -> bool {
        let line = self.peek().line;
        self.tokens.iter().any(|token| {
            token.line == line && (token.value == "=" || the_assignment_operator(&token.value).is_some())
        })
    }

    /// The call that a statement that begins with a dot wanted to make.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the v2 names the call
    /// with its parentheses: `.x` is `.x()` and `.x 1` is `.x(1)`.
    fn the_call_of_the_dot(&self) -> String {
        // The chain of the members: the dot, the name, and the dot and the
        // name that come after it.
        let mut end = self.position + 1;
        while end + 2 < self.tokens.len()
            && self.tokens[end + 1].value == "."
            && self.tokens[end + 2].kind == TokenKind::Name
        {
            end += 2;
        }
        let name = the_source_of_the_tokens(self, self.position, end + 1);
        let arguments = the_source_of_the_rest_of_the_line(self, end + 1);
        if arguments.is_empty() {
            return format!("{name}()");
        }
        if arguments.starts_with('(') {
            return format!("{name}{arguments}");
        }
        format!("{name}({arguments})")
    }

    /// The hotstring of the v1: `::btw::by the way`.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: the line is a
    /// hotstring, and it registers the text that it holds.
    fn parse_the_hotstring_of_the_v1(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let text = the_source_of_the_line(self, token.line);
        // The whole line is the hotstring, and the parser goes to the end
        // of it.
        while self.peek().kind != TokenKind::Newline && self.peek().kind != TokenKind::Eof {
            self.next();
        }
        self.end_of_statement()?;
        Ok(Node::LegacyCall {
            name: "hotstring".into(),
            parameters: vec![crate::ast::LegacyParameter::Text(text)],
            line: token.line,
        })
    }

    fn parse_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        // The statement begins here, and a token that cannot be there
        // names it.
        self.statement_start = self.position;
        let token = self.peek().clone();
        if token.kind == TokenKind::Directive {
            return Ok(vec![self.parse_directive()?]);
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-01: the hotstring of
        // the v1 is a statement, and the line is the hotstring.
        if self.dialect.hotstrings_are_statements() && self.at_op(&["::"]) {
            return Ok(vec![self.parse_the_hotstring_of_the_v1()?]);
        }
        if token.kind == TokenKind::Keyword {
            let word = token.value.to_lowercase();
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the `else`
            // of the v1 is a command of its own, and the parser of the v2
            // does not take it.
            if self.dialect.the_command_is_the_else(&word) {
                // The name of the command is consumed before the parameters
                // of it are read, and the name of the command is not one of
                // them.
                self.next();
                return Ok(vec![self.parse_call_statement(token)?]);
            }
            // Measured on the binary of v2.0.19 on 2026-11-01: the word that
            // is the name of a variable refuses with the reason of the word,
            // and the reason does not depend on the place where the word is.
            if self.the_word_is_the_name_of_a_variable() {
                if let Some(refusal) = crate::lexer::the_refusal_of_a_word(&word) {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // name of a function of the machine and the name of a
                    // class of the dialect are refused when they are the
                    // output variable of an assignment, and the refusal
                    // waits for the end of the parse: the scope of the
                    // assignment says whether the name is a global one.
                    if !matches!(refusal, TheRefusal::Func | TheRefusal::Class) {
                        return Err(self.the_error_of_the_refusal(refusal, &token));
                    }
                }
            }
            // Measured on the binary of v2.0.19 on 2026-11-01: the name of a
            // function may not be a word that the language reserves, and the
            // refusal names the function. The words that the v2 does not
            // reserve are names, and the words that begin a construct are
            // the construct and not a name.
            if self.the_word_is_the_name_of_a_function() {
                match crate::lexer::the_refusal_of_a_word(&word) {
                    Some(TheRefusal::Syntax) | Some(TheRefusal::Future) => {
                        return Err(SyntaxError {
                            message: format!(
                                "The following reserved word must not be used as a function name:\n{:?}",
                                token.value
                            ),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    Some(TheRefusal::Class) => {
                        return Err(SyntaxError {
                            message: "This function declaration conflicts with an existing Class.\n     Specifically: Class".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    Some(TheRefusal::Variable) if matches!(word.as_str(), "static" | "global" | "local") => {
                        return Err(SyntaxError {
                            message: format!(
                                "The following reserved word must not be used as a function name:\n{:?}",
                                token.value
                            ),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    // The `until` and the `catch` are not the head of a
                    // statement in v2, and the `ExitApp` is a function of the
                    // machine: the three of them may be the name of a
                    // function of the script.
                    Some(TheRefusal::Variable) if matches!(word.as_str(), "until" | "catch") => {
                        return self.parse_name_statement();
                    }
                    Some(TheRefusal::Func) => {
                        return self.parse_name_statement();
                    }
                    _ => {}
                }
            }
            match word.as_str() {
                "if" => return Ok(vec![self.parse_if()?]),
                "switch" => return Ok(vec![self.parse_switch()?]),
                "while" => return Ok(vec![self.parse_while()?]),
                "loop" => return Ok(vec![self.parse_loop()?]),
                "for" => return Ok(vec![self.parse_for()?]),
                "break" => {
                    self.next();
                    // Measured on the binary of v2.0.19 on 2026-11-01: the v2
                    // takes the parenthesis of a break and of a continue, and
                    // it ignores what is inside it.
                    self.the_parenthesis_that_is_ignored();
                    self.end_of_statement()?;
                    self.the_loop_of_the_break_or_the_continue(token.line, token.column)?;
                    return Ok(vec![Node::Break { line: token.line }]);
                }
                "continue" => {
                    self.next();
                    self.the_parenthesis_that_is_ignored();
                    self.end_of_statement()?;
                    self.the_loop_of_the_break_or_the_continue(token.line, token.column)?;
                    return Ok(vec![Node::Continue { line: token.line }]);
                }
                "class" => {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // name of a class of the dialect may be the name of a
                    // variable, and the refusal of the global one waits for
                    // the end of the parse.
                    if self.the_word_is_the_name_of_a_variable() {
                        return self.parse_name_statement();
                    }
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
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-02: the v1 refuses the word alone with "This
                    // line does not contain a recognized action.\n
                    // Specifically: class", and the word is a variable when
                    // it is assigned.
                    if !self.dialect.the_word_class_alone_holds_an_action() {
                        return Err(self.the_line_holds_no_action(token.line));
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
                "exitapp" => {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // `ExitApp` is a function of the machine and not a word
                    // of the dialect, and its name may be the name of a
                    // variable. The refusal of the global one waits for the
                    // end of the parse.
                    if self.the_word_is_the_name_of_a_variable() {
                        return self.parse_name_statement();
                    }
                    return Ok(vec![self.parse_exitapp()?]);
                }
                "until" => {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // `until` is a word of the dialect and it is reserved:
                    // it may not be the name of a variable, and the name of
                    // the check is above. The bare word wants a condition,
                    // and the binary crashes when it has one, so the tree
                    // refuses it there.
                    let token = self.next();
                    if self.at_end_of_statement() {
                        return Err(self.error(format!(
                            "\"Until\" requires at least 1 parameter.\n     Specifically: {}",
                            the_source_of_the_line(self, token.line)
                        )));
                    }
                    return Err(SyntaxError {
                        message: "Unexpected \"Until\"".to_string(),
                        line: token.line,
                        column: token.column,
                    });
                }
                _ => {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // words that may not be the name of a variable say so.
                    if self.the_word_may_not_be_a_name() {
                        if self.the_word_is_the_name_of_a_variable() {
                            return Err(self.the_error_of_the_reserved_word_as_a_name(self.position));
                        }
                        // The word `not` is the operator of the negation when
                        // an operand follows it, and the v2 takes the script.
                        if !(word == "not" && self.an_operand_follows_the_word()) {
                            return Err(self.the_error_of_the_reserved_word_as_a_call(self.position));
                        }
                    }
                }
            }
        }
        // The label of the line comes before anything else: a line that is
        // a label is a label, and not an expression.
        if let Some(label) = self.parse_label()? {
            return Ok(vec![label]);
        }
        if token.kind == TokenKind::Name {
            // Measured on the binary of v2.0.19 on 2026-11-01: the hash and
            // the semicolon that are glued to the name that opens the
            // statement are a part of it, and the statement holds no action:
            // the whole line is named.
            if self.the_illegal_character_is_glued_to_the_name_that_opens() {
                return Err(self.the_line_holds_no_action(token.line));
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the operator
            // that comes after the name is the increment, and it may be a
            // statement of its own.
            if self.the_postfix_increment_is_next() {
                let value = self.parse_expression()?;
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // increment of a built-in variable of the v1 is refused at the
                // load when the line is the increment alone: the increment
                // that is inside an expression, or the one that has
                // something after it, is accepted.
                if let Some(message) = self.the_refusal_of_the_increment(&value) {
                    return Err(self.error(message));
                }
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
        if token.kind == TokenKind::Op && token.value == "{" {
            // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
            // opens a line is a block of its own, and the statements between
            // the braces run.
            let body = self.parse_block_that_opens_a_line()?;
            return Ok(vec![Node::Block { body, line: token.line }]);
        }
        if token.kind == TokenKind::Op && matches!(token.value.as_str(), "++" | "--") {
            // Measured on the binary of v2.0.19 on 2026-10-31: the increment
            // may be a statement of its own.
            let value = self.parse_expression()?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
        }
        if token.kind == TokenKind::Op && token.value == "%" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a statement
            // that begins with a double deref is a name that the script
            // builds, and it may be written to: `%name% := 1`.
            let value = self.parse_expression()?;
            if self.at_an_assignment_operator() {
                let operator = self.next();
                let Some(op) = the_assignment_operator(&operator.value) else {
                    return Err(self.error("expected an assignment operator"));
                };
                let Node::DynamicName { pieces, .. } = value else {
                    return Err(self.error("Invalid assignment."));
                };
                let assigned = self.the_expression_of_a_statement()?;
                self.end_of_statement()?;
                return Ok(vec![Node::AssignDynamic {
                    name: pieces,
                    op,
                    value: Box::new(assigned),
                    line: token.line,
                }]);
            }
            // The name is only read, and the value it leaves behind is not
            // used.
            self.end_of_statement()?;
            return Ok(vec![Node::Expression { value: Box::new(value), line: token.line }]);
        }
        if token.kind == TokenKind::Op && !matches!(token.value.as_str(), "[" | "{" | ")" | "]" | "}") {
            // Measured on the binary of v2.0.19 on 2026-10-31: a statement
            // that begins with an operator is a line that holds no action,
            // and the text that is named is the line itself. The operators
            // that want a word of their own are named apart.
            return Err(self.error(self.the_message_of_the_operator_at_the_start(&token)));
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
        // Measured on the binary of v2.0.19 on 2026-10-31: the number that
        // is at the start of a statement and that is followed by a comma is
        // a call of v2 that is not written: the name of it is the number,
        // and the comma is not the space that a call wants. The number
        // that carries a point is not a name, and the line holds no
        // action.
        if token.kind == TokenKind::Number && !token.value.contains('.') && self.the_comma_is_after_the_number() {
            return Err(self.error(format!(
                "Function calls require a space or \"(\". Use comma only between parameters.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
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
        } else if token.kind == TokenKind::Op && the_token_is_an_illegal_character(&token) {
            // Measured on the binary of v2.0.19 on 2026-11-01: the character
            // that begins a statement is a line that holds no action, and
            // the text that is named begins at it.
            return Err(self.the_error_of_the_illegal_character(self.position, false));
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
        if token.kind == TokenKind::Op && token.value == "{" {
            // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
            // follows a value opens the literal of an object, and not a
            // block: the literal that is not closed is a "Missing "}"", and
            // the literal that is closed is a second value in a row, and the
            // brace is the one that is not wanted. The properties inside it
            // are not read: the brace is the failure, and not them.
            self.position += 1;
            if !self.the_object_literal_is_closed() {
                let last = self
                    .tokens
                    .iter()
                    .rev()
                    .find(|token| token.kind == TokenKind::Eof)
                    .cloned()
                    .unwrap_or_else(|| token.clone());
                return Err(self.the_error_of_a_missing_bracket("}", last.line, last.column));
            }
            return Err(SyntaxError {
                message: "Unexpected \"{\"".into(),
                line: token.line,
                column: token.column,
            });
        }
        if token.kind == TokenKind::Op && token.value == "}" {
            return Ok(());
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: the character that is
        // not a comment and not a directive is an illegal character in an
        // expression, and the text of the expression is named.
        if token.kind == TokenKind::Op && the_token_is_an_illegal_character(&token) {
            return Err(self.the_error_of_the_illegal_character(self.position, true));
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the reserved word
        // that is left over after an expression says what it is.
        if token.kind == TokenKind::Keyword {
            return Err(self.the_error_of_the_reserved_word(self.position));
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: the colon that is
        // left over after an expression is the colon of a question that is
        // not there, and it says so.
        if token.kind == TokenKind::Op && token.value == ":" {
            return Err(SyntaxError {
                message: "A \":\" is missing its \"?\"".to_string(),
                line: token.line,
                column: token.column,
            });
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

    /// Whether the token that comes next may be the name of a class.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// takes the name of a class as it is written, and a word of the language
    /// is a name of it: the `class if` is a class, and the `new if` makes an
    /// instance of it.
    fn a_class_name_may_come_next(&self) -> bool {
        let kind = self.peek().kind;
        kind == TokenKind::Name
            || (kind == TokenKind::Keyword
                && self.dialect.the_name_of_a_class_may_be_a_word_of_the_language())
    }

    /// A statement that starts with a name: an assignment, a call, or the
    /// definition of a function.
    fn parse_name_statement(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let name_token = self.next();
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // `new` of the v1 is a statement of its own: `new C` makes an instance
        // of the class and throws it away. The word is not reserved: a `new`
        // that no name follows is a variable of the script.
        if self.dialect.the_new_makes_an_instance()
            && name_token.value.eq_ignore_ascii_case("new")
            && self.a_class_name_may_come_next()
        {
            let instance = self.parse_new(name_token.line)?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression {
                value: Box::new(instance),
                line: name_token.line,
            }]);
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // variable that the script builds is a statement of its own, and it
        // may be written to: `MyArray%i% := 1`.
        if self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
            let at_the_name = self.position - 1;
            let pieces = self.the_pieces_of_a_dynamic_name(name_token.value.clone(), at_the_name, name_token.line)?;
            if self.at_an_assignment_operator() {
                let operator = self.next();
                let Some(op) = the_assignment_operator(&operator.value) else {
                    return Err(self.error(format!(
                        "expected an assignment operator after {:?}",
                        name_token.value
                    )));
                };
                let value = self.the_expression_of_a_statement()?;
                self.end_of_statement()?;
                return Ok(vec![Node::AssignDynamic {
                    name: pieces,
                    op,
                    value: Box::new(value),
                    line: name_token.line,
                }]);
            }
            // A name that is only read is a statement that reads it, and the
            // value it leaves behind is not used.
            self.end_of_statement()?;
            return Ok(vec![Node::Expression {
                value: Box::new(Node::DynamicName { pieces, line: name_token.line }),
                line: name_token.line,
            }]);
        }
        if self.at_op(&["("]) && !self.peek().spaced {
            self.next();
            // The name of the call is the token that comes before the
            // parenthesis, and the text of a refusal names the call.
            let at_the_name = self.position - 2;
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
                // Measured on the binary of v2.0.19 on 2026-11-01: the body of
                // a function may be an expression, and the fat arrow is what
                // says so: `f(a) => a + 1` is a function that answers the
                // expression of it.
                let body = if self.the_fat_arrow_comes_next() {
                    self.the_body_of_a_fat_arrow(name_token.line)?
                } else {
                    self.the_body_of_a_scope("function", name_token.line)?
                };
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
                    // The v1 refuses the call of a function at the load, and
                    // the number of the arguments is one of the things that it
                    // refuses there: the least number is the one of the
                    // parameters that have no default, and the most is the
                    // number of them.
                    self.arities.insert(
                        name_token.value.to_lowercase(),
                        the_arity_of_the_parameters(&parameters),
                    );
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
            let end = self.position;
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // refuses the call of a name that is not a function at the load,
            // and a call that is a statement is a call: the `MyFunc()` is
            // refused with "Call to nonexistent function." and the text of it
            // is named.
            if self.dialect.refuses_the_call_of_a_function_at_the_load() {
                self.the_call_of_the_v1_is_recorded(
                    &name_token.value.to_lowercase(),
                    arguments.len(),
                    at_the_name,
                    end,
                );
            }
            self.the_expression_is_done(self.position, false)?;
            // Measured on the binary of v2.0.19 on 2026-10-31: the answer
            // of a call is a value, and the members of it are read the way
            // the members of any other value are: `Map().Count` is 0, and
            // the statement reads it and leaves the value behind.
            let call = Node::Call {
                name: name_token.value.to_lowercase(),
                arguments,
                line: name_token.line,
            };
            let value = self.parse_suffixes(call, false, false)?;
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
            // Measured on the binary of v2.0.19 on 2026-10-31: the member of
            // the answer of a call may be written, and the write of it is
            // refused at the run when the property is read-only. The call
            // itself is not a value that may be written, and its refusal is
            // at the load, with the text of it named.
            if self.at_an_assignment_operator() {
                return Ok(vec![self.parse_the_assignment(value)?]);
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the
            // expression of a statement goes on: the operators and the
            // concatenation of v2 are a part of it, and the value of it is
            // thrown away. `f() + 1` is the sum, and `f() "a"` is the
            // concatenation.
            let value = self.the_expression_after(value)?;
            // Measured on the binary of v2.0.19 on 2026-10-31: the
            // expression of a statement ends at the comma, and the rest of
            // the line is the list of v1.
            let legacy = self.the_legacy_list()?;
            self.end_of_statement()?;
            // A call that carries a member is an expression: the statement
            // reads the member and leaves the value behind. A call alone is
            // a call, and the report names it the way it names a command.
            let statement = if matches!(&value, Node::Call { .. }) {
                value
            } else {
                Node::Expression { value: Box::new(value), line: name_token.line }
            };
            let mut statements = vec![statement];
            statements.extend(legacy);
            return Ok(statements);
        }
        // `obj.Prop := value` and `obj[key] := value`: the left side is a
        // value and not a name.
        if self.at_member_dot() || self.at_member_bracket() {
            let target = Node::Var { name: name_token.value.clone(), line: name_token.line };
            self.the_expression_begins();
            let target = self.parse_suffixes(target, false, true)?;
            self.the_expression_is_done(self.position, false)?;
            let operator = self.peek().clone();
            if operator.kind == TokenKind::Op && operator.value == "=" {
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // equal sign of the v1 does not write a member: the line
                // holds no action, and it is named.
                if self.dialect.assign_with_equal() {
                    return Err(self.error(format!(
                        "This line does not contain a recognized action.\n     Specifically: {}",
                        the_source_of_the_line(self, name_token.line)
                    )));
                }
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
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // expression of a statement ends at the comma, and the rest
                // of the line is the list of v1. The write of a member is
                // a statement like any other.
                let legacy = self.the_legacy_list()?;
                self.end_of_statement()?;
                let mut statements = vec![match target {
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
                }];
                statements.extend(legacy);
                return Ok(statements);
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the member
            // that has parentheses, and the item that is read, are
            // expressions: the operators and the concatenation of v2 are a
            // part of them, and the comma ends them. The rest of the line
            // is the list of v1, and it is evaluated.
            let value = match target {
                Node::Property { target, name, line } => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // member that has no parentheses is a command of v2: the
                    // rest of the line is the list of its arguments, and the
                    // comma that comes right after it with no space is not
                    // one of them. The line holds no action.
                    if self.at_op(&[","]) && !self.peek().spaced {
                        return Err(self.error(format!(
                            "This line does not contain a recognized action.\n     Specifically: {}",
                            the_source_of_the_line(self, name_token.line)
                        )));
                    }
                    // A name that is a statement is a call: `m.Method` calls
                    // the method, and the parentheses may be left out. An
                    // item is not a call: `m["a"]` reads it.
                    let arguments = self.the_parameters_of_a_command()?;
                    self.end_of_statement()?;
                    return Ok(vec![Node::MethodCall {
                        target,
                        name,
                        arguments,
                        line,
                    }]);
                }
                other => self.the_expression_after(other)?,
            };
            let legacy = self.the_legacy_list()?;
            self.end_of_statement()?;
            // A method that is called is a statement of its own, and so is
            // the item that is read. The value of an expression is a
            // statement that reads it, and throws the value away.
            let statement = match value {
                Node::MethodCall { .. } | Node::Index { .. } => value,
                other => Node::Expression { value: Box::new(other), line: name_token.line },
            };
            let mut statements = vec![statement];
            statements.extend(legacy);
            return Ok(statements);
        }
        // A name that is neither a call nor an assignment is a call
        // statement: `Send "hello"`. The parentheses may be left out, and
        // the rest of the line is the list of the parameters.
        let operator = self.peek().clone();
        if operator.kind == TokenKind::Op && operator.value == "=" {
            // Measured on the binary of v1.1.37.02 on 2026-11-01: the equal
            // sign assigns in the v1: `x = 5` leaves `x` at 5.
            if self.dialect.assign_with_equal() {
                return self.parse_legacy_assignment(name_token);
            }
            return Err(self.the_equal_sign_is_not_an_assignment(name_token.line));
        }
        if !matches!(
            operator.value.as_str(),
            ":=" | "+=" | "-=" | "*=" | "/=" | "//=" | ".=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>="
        ) {
            return Ok(vec![self.parse_call_statement(name_token)?]);
        }
        self.parse_assignment(name_token)
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
        // Measured on the binary of v1.1.37.02 on 2026-11-01: the name at
        // the start of a line is a command of the v1, and the parameters of
        // it are texts.
        if self.dialect.legacy_commands() {
            return self.parse_legacy_call_statement(name_token);
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: a name that is
        // not a built-in is not refused at the load. It is a function of the
        // script, or a variable of it, and the value of the variable is
        // what the call asks for.
        let name = name_token.value.to_lowercase();
        self.the_expression_begins();
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // command of v2 is followed by a space or a parenthesis, and the
        // comma that is there instead of the space is refused. The space
        // may be there before the comma: `x , 2` is a command.
        if self.at_op(&[","]) && !self.peek().spaced {
            return Err(self.error(format!(
                "Function calls require a space or \"(\". Use comma only between parameters.\n     Specifically: {}",
                the_source_of_the_line(self, name_token.line)
            )));
        }
        let arguments = self.the_parameters_of_a_command()?;
        // Measured on the binary of v2.0.19 on 2026-11-01: the command of v2
        // is a call whose parentheses are not written, and the text that
        // names the illegal character carries the one that closes them.
        if self.peek().kind == TokenKind::Op && the_token_is_an_illegal_character(self.peek()) {
            let error = self.the_error_of_the_illegal_character(self.position, true);
            return Err(the_message_of_a_command(error));
        }
        // The command of v2 is a call whose parentheses are not written, and
        // the text that names the operator carries the one that closes it.
        self.the_expression_is_done(self.position, true)?;
        self.end_of_statement()?;
        Ok(Node::Call { name, arguments, line: name_token.line })
    }

    /// The legacy command of the v1: `Msg, text`.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: the parameters
    /// of a command of the v1 are texts, and not expressions: `FileAppend,
    /// hello, *` writes `hello`, and `FileAppend, "hello", *` writes
    /// `"hello"` with the quotes. The name of a command that the dialect
    /// does not have is a line that holds no action, and it is refused at
    /// the load with the text of the line.
    fn parse_legacy_call_statement(&mut self, name_token: Token) -> Result<Node, SyntaxError> {
        let name = name_token.value.to_lowercase();
        // The name of the command is the token that comes before the
        // parameters of it, and the text of a refusal names it.
        let name_at = self.position - 1;
        // Measured on the binary of v1.1.37.02 on 2026-11-02: a function of
        // the v1 is not a command: `StrLen "abc"` is a line that holds no
        // action, and the load refuses it with the text of the line.
        let carried = self
            .dialect
            .builtins()
            .iter()
            .any(|builtin| {
                builtin.name.to_lowercase() == name
                    && (!self.dialect.the_command_has_to_be_a_command()
                        || builtin.kind == crate::builtins::BuiltinKind::Command)
            });
        if !carried {
            return Err(self.error(format!(
                "This line does not contain a recognized action.\n     Specifically: {}",
                self.the_source_of_the_legacy_command(name_at)
            )));
        }
        let mut places = Vec::new();
        // The comma that comes right after the name is the one that
        // separates it from the first parameter.
        if self.at_op(&[","]) {
            self.next();
            places = self.the_places_of_the_parameters_of_a_legacy_command()?;
        } else if !self.at_end_of_statement() {
            places = self.the_places_of_the_parameters_of_a_legacy_command()?;
        }
        let mut parameters: Vec<String> = places
            .iter()
            .map(|(start, end)| self.the_verbatim_source(*start, *end).trim().to_string())
            .collect();
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the comma at
        // the end of the line does not open a parameter: the number of the
        // parameters of a command is the number of the commas that are not
        // the last one.
        if parameters.last().map(|text| text.is_empty()).unwrap_or(false) {
            parameters.pop();
            places.pop();
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the last
        // parameter that a command of the v1 reads takes the rest of the
        // line: the `FileGetSize, out, name, K, extra` complains about the
        // third parameter with the text "K, extra", and not with the text
        // "K". The text is the source of the line, from the parameter to the
        // end of it: the `AutoTrim, On,extra` names "On,extra".
        //
        // The command that has a body is the exception: the text that comes
        // after the parameters of it is the action, and not a parameter.
        if let Some(builtin) = self.the_builtin_of_the_legacy_command(&name) {
            let count = builtin.parameters.len();
            if count > 0 && parameters.len() > count && !self.dialect.the_command_has_a_body(&name) {
                let (start, _) = places[count - 1];
                let (_, end) = *places.last().expect("the parameters are there");
                let text = self.the_verbatim_source(start, end).trim().to_string();
                parameters.truncate(count);
                places.truncate(count);
                parameters[count - 1] = text;
                places[count - 1] = (start, end);
            }
        }
        self.end_of_statement()?;
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` of
        // the v1 is the action of the other side of the `if` that comes
        // before it, and it is a failure when there is no `if`.
        if self.dialect.the_command_is_the_else(&name) {
            // The `else` takes no parameter, and the text of the one that is
            // there is named.
            if !parameters.is_empty() {
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // `else` names the rest of the line: the `Else, a, b` names
                // "a, b".
                let text = parameters.join(", ");
                return Err(self.error(format!(
                    "This line does not contain a recognized action.\n     Specifically: {text}"
                )));
            }
            return Err(self.error("ELSE with no matching IF".to_string()));
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the number of
        // the parameters of a command is checked at the load, and the
        // complaint of the v1 is its own.
        if let Some(builtin) = self.the_builtin_of_the_legacy_command(&name) {
            let count = parameters.len();
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has
            // two forms of the complaint of the number of the parameters, and
            // the dialect knows the one that is the complaint of the command.
            if let Some(refusal) =
                self.dialect
                    .the_refusal_of_the_arity_of_a_command(&name, count, builtin.min)
            {
                return Err(self.error(self.the_message_of_the_arity(refusal, name_at, &places)));
            }
            if (count as u8) < builtin.min {
                return Err(self.error(format!(
                    "\"{}\" requires at least {} parameter{}.\n     Specifically: {}",
                    builtin.name,
                    builtin.min,
                    if builtin.min == 1 { "" } else { "s" },
                    self.the_text_of_a_legacy_command(name_at, &places),
                )));
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the
            // parameter that wants the name of a variable may not be blank,
            // and the complaint of the v1 is its own.
            for (index, parameter) in parameters.iter().enumerate() {
                let wants_a_name = matches!(
                    builtin.parameters.get(index),
                    Some(
                        crate::builtins::Parameter::Variable
                            | crate::builtins::Parameter::Output
                            | crate::builtins::Parameter::Name
                    )
                );
                if wants_a_name && parameter.trim().is_empty() {
                    return Err(self.error(format!(
                        "\"{}\" requires that parameter #{} be non-blank.\n     Specifically: {}",
                        builtin.name,
                        index + 1,
                        self.the_text_of_a_legacy_command(name_at, &places),
                    )));
                }
            }
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the name of a
        // variable of the script is checked when the script loads, and the
        // complaint of the v1 is its own: the name may not carry a character
        // that is not a letter, a digit or the underscore.
        if let Some(builtin) = self.the_builtin_of_the_legacy_command(&name) {
            for (index, parameter) in parameters.iter().enumerate() {
                let is_a_name = matches!(
                    builtin.parameters.get(index),
                    Some(
                        crate::builtins::Parameter::Variable
                            | crate::builtins::Parameter::Output
                    )
                );
                if !is_a_name {
                    continue;
                }
                if let Some(refusal) =
                    self.dialect.the_refusal_of_the_name_of_a_variable(index + 1, parameter)
                {
                    return Err(self.error(refusal));
                }
            }
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the value of a
        // parameter of a command of the v1 is checked when the script loads,
        // and the complaint of the v1 is its own: "Parameter #N invalid."
        // with the text that the script wrote.
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the command
        // that has a body checks the value of its parameter after the text
        // that is left of the line went into it, and the check is done by
        // the body of the command.
        if !self.dialect.the_command_has_a_body(&name) {
            if let Some(refusal) =
                self.dialect.the_refusal_of_an_invalid_parameter(&name, &parameters)
            {
                return Err(self.error(refusal));
            }
        }
        // The command of the v1 that has a body: the action is the rest of
        // the line, the block, or the statement of the next line.
        if self.dialect.the_command_has_a_body(&name) {
            return self.parse_the_body_of_a_legacy_command(name, parameters, places, name_token.line);
        }
        let parameters = self.the_parameters_of_a_legacy_command(&name, &places);
        Ok(Node::LegacyCall { name, parameters, line: name_token.line })
    }

    /// The source of a command of the v1, from its name to the end of the
    /// line.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal of a
    /// command that the dialect does not have names the command, and not the
    /// line that holds it: the `else` that is on the same line is not a part
    /// of it.
    fn the_source_of_the_legacy_command(&self, name_at: usize) -> String {
        let line = self.tokens.get(name_at).map(|token| token.line).unwrap_or(0);
        let mut end = name_at;
        while end < self.tokens.len() && self.tokens[end].line == line {
            end += 1;
        }
        the_source_of_the_tokens(self, name_at, end).trim_end().to_string()
    }

    /// The row of the command of the v1 that has this name.
    fn the_builtin_of_the_legacy_command(&self, name: &str) -> Option<&'static crate::builtins::Builtin> {
        self.dialect
            .builtins()
            .iter()
            .find(|builtin| builtin.name.to_lowercase() == name)
    }

    /// Whether the text begins with the name of a command of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text that
    /// comes after the parameters of a command that has a body is the action
    /// when it begins with the name of a command, and the last parameter of
    /// the command takes the rest of the line when it does not: the
    /// `IfInString, x, b, Nope, [yes], *` looks for the text
    /// "b, Nope, [yes], *" and runs nothing.
    fn the_text_begins_with_a_command(&self, text: &str) -> bool {
        let name = text
            .trim()
            .split(|character: char| character == ',' || character.is_whitespace())
            .next()
            .unwrap_or("")
            .to_lowercase();
        if name.is_empty() {
            return false;
        }
        self.dialect.builtins().iter().any(|builtin| {
            builtin.name.to_lowercase() == name
                && (!self.dialect.the_command_has_to_be_a_command()
                    || builtin.kind == crate::builtins::BuiltinKind::Command)
        })
    }

    /// The text of a command of the v1 that a refusal names.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text is the
    /// text of the command, from the name of it to the end of the **first**
    /// parameter -- and the first one and no other, whatever the refusal is
    /// about: `StringReplace, out, a` names `StringReplace, out` when the
    /// command has too few parameters, and `StringReplace, out, , b` names it
    /// too when the second parameter is blank. The command that has no
    /// parameter names the name and the comma that comes after it, and the
    /// first parameter that is blank adds nothing to the text.
    fn the_text_of_a_legacy_command(
        &self,
        name_at: usize,
        places: &[(usize, usize)],
    ) -> String {
        let end = match places.first() {
            Some((_, end)) => *end,
            None => {
                // The command has no parameter: the text is the name and the
                // comma that comes after it.
                let comma = self.tokens.get(name_at + 1);
                let has_a_comma = comma
                    .map(|token| token.kind == TokenKind::Op && token.value == ",")
                    .unwrap_or(false);
                if has_a_comma {
                    name_at + 2
                } else {
                    name_at + 1
                }
            }
        };
        self.the_verbatim_source(name_at, end)
    }

    /// The complaint of the number of the parameters of a command.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the complaint that
    /// names the command carries the text of the command, and the complaint
    /// that names the parameter that has to be there carries nothing else.
    fn the_message_of_the_arity(
        &self,
        refusal: crate::dialect::TheRefusalOfTheArity,
        name_at: usize,
        places: &[(usize, usize)],
    ) -> String {
        match refusal {
            crate::dialect::TheRefusalOfTheArity::AtLeast { name, number } => format!(
                "\"{}\" requires at least {} parameter{}.\n     Specifically: {}",
                name,
                number,
                if number == 1 { "" } else { "s" },
                self.the_text_of_a_legacy_command(name_at, places),
            ),
            crate::dialect::TheRefusalOfTheArity::Required { number } => {
                format!("Parameter #{number} required")
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the two
            // parameters that the command wants are named together: the
            // `MouseClickDrag, Left` is refused with "Parameter #4 and 5
            // required.".
            crate::dialect::TheRefusalOfTheArity::RequiredTogether { first, second } => {
                format!("Parameter #{first} and {second} required.")
            }
        }
    }

    /// The body of a command of the v1 that has a body.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IfInString`
    /// and the `IfMsgBox` have an action: the command that is on the same
    /// line, the block, or the statement of the next line. The `else` that
    /// comes after the action is the action of the other side.
    fn parse_the_body_of_a_legacy_command(
        &mut self,
        name: String,
        mut parameters: Vec<String>,
        mut places: Vec<(usize, usize)>,
        line: u32,
    ) -> Result<Node, SyntaxError> {
        // The parameters of the command are the first ones, and the rest of
        // the line is the action that is on the same line.
        let own = self
            .the_builtin_of_the_legacy_command(&name)
            .map(|builtin| builtin.parameters.len())
            .unwrap_or(parameters.len());
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the text that
        // comes after the parameters of a command that has a body is the
        // action when it begins with the name of a command, and the last
        // parameter of the command takes the rest of the line when it does
        // not: the `IfInString, x, b, 1, FileAppend, [yes], *` looks for the
        // text "b, 1, FileAppend, [yes], *" and runs nothing.
        let action = if parameters.len() > own
            && self.the_text_begins_with_a_command(&parameters[own])
        {
            let action = parameters[own..].to_vec();
            parameters.truncate(own);
            Some(action)
        } else {
            if parameters.len() > own && own > 0 {
                // The last parameter of the command takes the rest of the
                // line, the way it does for every other command.
                let (start, _) = places[own - 1];
                let (_, end) = *places.last().expect("the parameters are there");
                let text = self.the_verbatim_source(start, end).trim().to_string();
                parameters.truncate(own);
                places.truncate(own);
                parameters[own - 1] = text;
                places[own - 1] = (start, end);
            }
            None
        };
        let own_parameters = parameters;
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the value of
        // the parameter of a command that has a body is checked when the
        // script loads, and the text that the check sees is the text that is
        // left of the line: the `IfMsgBox, OK, extra` is refused with
        // "Parameter #1 invalid." and the text "OK, extra".
        if let Some(refusal) =
            self.dialect.the_refusal_of_an_invalid_parameter(&name, &own_parameters)
        {
            return Err(self.error(refusal));
        }
        let body = match action {
            Some(action) => {
                let action_name = action[0].to_lowercase();
                // The first of the action is the name of the command, and the
                // places of the parameters come after it.
                let action_places = &places[own + 1..];
                let action_parameters =
                    self.the_parameters_of_a_legacy_command(&action_name, action_places);
                vec![Node::LegacyCall {
                    name: action_name,
                    parameters: action_parameters,
                    line,
                }]
            }
            None => self.parse_block(&name, line)?,
        };
        // The `else` is the action of the other side, and it comes after the
        // action of the command.
        let mut otherwise = Vec::new();
        self.skip_newlines();
        if self.the_else_comes_next() {
            self.next();
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the `else`
            // takes no parameter, and the text of the one that is there is
            // named.
            if self.at_op(&[","]) {
                self.next();
                let mut parameters = self.the_texts_of_the_parameters_of_a_legacy_command()?;
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // comma at the end of the line does not open a parameter, and
                // the `else` that has it takes no action.
                if parameters.last().map(|text| text.is_empty()).unwrap_or(false) {
                    parameters.pop();
                }
                if let Some(parameter) = parameters.first() {
                    return Err(self.error(format!(
                        "This line does not contain a recognized action.\n     Specifically: {parameter}"
                    )));
                }
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the action
            // of the `else` is the statement that is on the same line, the
            // block that comes after it, or the statement of the next line.
            // The `else` that is at the end of the script has no action.
            if self.at_end_of_statement() {
                self.end_of_statement()?;
                if self.peek().kind != TokenKind::Eof {
                    otherwise = self.parse_block("Else", line)?;
                }
            } else {
                otherwise = self.parse_statement()?;
            }
        }
        Ok(Node::LegacyIf {
            name,
            parameters: own_parameters,
            body,
            otherwise,
            line,
        })
    }

    /// Whether the `else` of the v1 comes next.
    fn the_else_comes_next(&self) -> bool {
        let Some(token) = self.tokens.get(self.position) else {
            return false;
        };
        let name = token.value.to_lowercase();
        name == "else" && self.dialect.the_command_is_the_else(&name)
    }

    /// The texts of the parameters of a command of the v1: the texts that the
    /// commas separate.
    ///
    /// The text of a parameter is the text that the script wrote, and the
    /// commas that are inside a text or inside a group of parentheses do
    /// not separate them. The line that comes begins with an operator
    /// continues the parameter that is being read.
    fn the_texts_of_the_parameters_of_a_legacy_command(&mut self) -> Result<Vec<String>, SyntaxError> {
        let places = self.the_places_of_the_parameters_of_a_legacy_command()?;
        Ok(places
            .iter()
            .map(|(start, end)| self.the_verbatim_source(*start, *end).trim().to_string())
            .collect())
    }

    /// The parameters of a command of the v1: the texts that the commas
    /// separate, and the expressions of the ones that want a number.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter that
    /// wants a number is an expression -- the `Sleep, 100+50` waits for a
    /// hundred and fifty milliseconds -- and the others are texts.
    fn the_parameters_of_a_legacy_command(
        &mut self,
        name: &str,
        places: &[(usize, usize)],
    ) -> Vec<crate::ast::LegacyParameter> {
        let mut parameters = Vec::with_capacity(places.len());
        for (index, (start, end)) in places.iter().enumerate() {
            let text = self.the_verbatim_source(*start, *end).trim().to_string();
            match self.the_expression_of_a_parameter(name, index, *start, *end) {
                Some(node) => parameters.push(crate::ast::LegacyParameter::Expression(node)),
                None => parameters.push(crate::ast::LegacyParameter::Text(text)),
            }
        }
        parameters
    }

    /// The expression of a parameter of a command of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter that
    /// wants a number is an expression -- the `Sleep, 100+50` waits for a
    /// hundred and fifty milliseconds, and the `Random, x, n, n` reads the
    /// variable `n` -- and a parameter that is not an expression is the number
    /// zero: the `Exit, 100abc` is zero, and the `Exit, 1+` is zero too.
    ///
    /// The percent sign is the substitution of the v1, and the text that comes
    /// out of it is a text and not an expression: the `Exit, %n%+50` is fifty
    /// and not a hundred.
    fn the_expression_of_a_parameter(
        &mut self,
        name: &str,
        index: usize,
        start: usize,
        end: usize,
    ) -> Option<Node> {
        // The parameter that wants a number is an expression, and the others
        // are texts.
        if !self.dialect.the_parameter_wants_a_number(name, index) {
            return None;
        }
        // The substitution of the v1 is a text.
        if self.the_verbatim_source(start, end).contains('%') {
            return None;
        }
        // The expression is read from the tokens of the parameter.
        let saved = self.position;
        self.position = start;
        let parsed = self.parse_expression();
        let after = self.position;
        self.position = saved;
        let line = self.tokens.get(start).map(|token| token.line).unwrap_or(0);
        match parsed {
            // The expression is the whole parameter.
            Ok(node) if after == end => Some(node),
            // The parameter is not an expression, and the v1 reads it as the
            // number zero.
            _ => Some(Node::Literal {
                value: crate::ast::LiteralValue::Integer(0),
                line,
            }),
        }
    }

    /// The place of each parameter of a command of the v1.
    ///
    /// The place of a parameter is the place of the tokens that it is made
    /// of: the first one and the one that comes after it. The refusal of
    /// the v1 names the text of the command, and the text is the text of
    /// the script.
    fn the_places_of_the_parameters_of_a_legacy_command(
        &mut self,
    ) -> Result<Vec<(usize, usize)>, SyntaxError> {
        let mut places = Vec::new();
        let mut start = self.position;
        loop {
            let end = self.the_end_of_a_parameter(start);
            places.push((start, end));
            // The parser is after the parameter, and the comma that
            // separates it from the next one comes after that.
            self.position = end;
            if end >= self.tokens.len() {
                break;
            }
            let token = &self.tokens[end];
            if token.kind != TokenKind::Op || token.value != "," {
                break;
            }
            self.position = end + 1;
            start = self.position;
        }
        Ok(places)
    }

    /// The legacy assignment of the v1: `x = 5`.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the right side
    /// of the legacy assignment is a text and not an expression. `x = 5 + 3`
    /// leaves `x` at "5 + 3", `x = "abc"` leaves it at `"abc"` with the
    /// quotes that were written, and `x = (5+3)` leaves it at "(5+3)".
    ///
    /// The text is the text of the script, and the spaces that come before
    /// it and the spaces that come after it are not a part of it.
    fn parse_legacy_assignment(&mut self, name_token: Token) -> Result<Vec<Node>, SyntaxError> {
        // The equal sign is the one that assigns.
        self.next();
        let start = self.position;
        // The text of the legacy assignment is the rest of the line: the
        // comma that comes before the end of it is a part of the text, and
        // the line that comes after is a statement of its own.
        let mut end = start;
        while end < self.tokens.len() && self.tokens[end].kind != TokenKind::Newline {
            end += 1;
        }
        let text = self.the_verbatim_source(start, end);
        self.position = end;
        self.end_of_statement()?;
        self.the_assignment_is_recorded(
            &name_token.value,
            AssignOp::Set,
            name_token.line,
            name_token.column,
        );
        Ok(vec![Node::LegacyAssign {
            name: name_token.value,
            text: text.trim().to_string(),
            line: name_token.line,
        }])
    }

    /// The source of a range of tokens, the way the script wrote it.
    ///
    /// The spaces that the script wrote are in it: `FileAppend, [  pad  ]`
    /// keeps the two spaces of the text, and the tokens alone would keep
    /// one.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text of the
    /// legacy dialect is the text of the script.
    fn the_verbatim_source(&self, start: usize, end: usize) -> String {
        let Some(first) = self.tokens.get(start) else { return String::new() };
        let Some(last) = self.tokens.get(end.saturating_sub(1)) else { return String::new() };
        // A text that goes from one line to the next is the text of the
        // tokens: the source of the script has a newline in the middle.
        if first.line != last.line {
            return the_source_of_the_tokens(self, start, end);
        }
        let Some(line) = self.lines.get(first.line as usize - 1) else {
            return the_source_of_the_tokens(self, start, end);
        };
        let chars: Vec<char> = line.chars().collect();
        let from = first.column.saturating_sub(1) as usize;
        // The end of the last token is the place of the token that comes
        // after it: the spaces between the two are not a part of it.
        let to = match self.tokens.get(end) {
            Some(next) if next.line == first.line => next.column.saturating_sub(1) as usize,
            _ => chars.len(),
        };
        if from >= to || to > chars.len() {
            return the_source_of_the_tokens(self, start, end);
        }
        let text: String = chars[from..to].iter().collect();
        text.trim_end().to_string()
    }

    /// Whether the line of a command of the v1 continues on the next one.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the line of a
    /// command of the v1 continues when the line that comes begins with an
    /// operator, and not when it ends with a comma. The comma at the end of
    /// the line of `FileAppend, hello,` does not open a parameter, and the
    /// line that comes is a statement of its own.
    ///
    /// The increment and the decrement are not among the operators: the line
    /// that begins with one of them does not continue the one before it.
    fn the_line_of_a_legacy_command_continues_at(&self, at: usize) -> bool {
        let mut index = at;
        while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline {
            index += 1;
        }
        let Some(token) = self.tokens.get(index) else { return false };
        if token.kind == TokenKind::Op && (token.value == "++" || token.value == "--") {
            return false;
        }
        the_token_is_an_operator(token)
    }

    /// The place where the parameter that starts here ends.
    ///
    /// The end of the parameter is the comma that separates it from the
    /// next one, or the end of the statement. The line that comes begins
    /// with an operator continues the parameter that is being read.
    fn the_end_of_a_parameter(&self, start: usize) -> usize {
        let mut index = start;
        let mut depth = 0usize;
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            match token.kind {
                TokenKind::Newline => {
                    // The line continues when the line that comes begins with
                    // an operator, and the empty lines between the two are
                    // not a part of it.
                    if !self.the_line_of_a_legacy_command_continues_at(index) {
                        break;
                    }
                    index += 1;
                    while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline {
                        index += 1;
                    }
                    continue;
                }
                TokenKind::Eof => break,
                TokenKind::Op => match token.value.as_str() {
                    "(" | "[" | "{" => depth += 1,
                    ")" | "]" | "}" => depth = depth.saturating_sub(1),
                    "," if depth == 0 => break,
                    _ => {}
                },
                _ => {}
            }
            index += 1;
        }
        index
    }

    /// The parameters of a command of v2.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the parameters of
    /// a command are the expressions that the commas separate, up to the
    /// end of the statement. A parenthesis that opens one of them is a
    /// parameter of its own, and the value of it is the last of the list
    /// that it holds.
    fn the_parameters_of_a_command(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut arguments = Vec::new();
        if !self.at_end_of_statement() {
            loop {
                if self.at_op(&[","]) {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // parameter that is not there keeps its place, and the
                    // parameter has no value: one that has a default takes
                    // it, and one that has none is a missing parameter.
                    let line = self.peek().line;
                    self.position += 1;
                    // The parameter that is not there and that is the last
                    // one is dropped: the commas at the end of the
                    // parameters add nothing.
                    if self.the_rest_is_commas() {
                        while self.at_op(&[","]) {
                            self.position += 1;
                        }
                        break;
                    }
                    arguments.push(Node::Empty { line });
                    continue;
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: a
                // brace that opens an argument of a command opens the
                // literal of an object, and not a key: `Send {Enter}` is
                // refused with "Missing \"propertyname:\" in object
                // literal.". The key of the v2 is a text: `Send
                // "{Enter}"`.
                arguments.push(self.parse_expression().map_err(the_message_of_a_command)?);
                if self.at_op(&[","]) {
                    self.position += 1;
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // commas at the end of the parameters of a command add
                    // nothing: `One "a",` calls `One` with one parameter.
                    if self.the_rest_is_commas() {
                        while self.at_op(&[","]) {
                            self.position += 1;
                        }
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        Ok(arguments)
    }

    /// Whether the rest of the statement holds nothing but commas.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the commas at the
    /// end of the parameters of a command add nothing: `One "a",` calls
    /// `One` with one parameter.
    fn the_rest_is_commas(&self) -> bool {
        let mut index = self.position;
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            match token.kind {
                // Measured on the binary of v2.0.19 on 2026-10-31: the line
                // that ends with a comma continues on the next one, and the
                // empty lines between the two are not a part of it.
                TokenKind::Newline => {
                    if !self.the_line_continues_at(index) {
                        return true;
                    }
                    index += 1;
                    while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline {
                        index += 1;
                    }
                }
                TokenKind::Eof => return true,
                TokenKind::Op if token.value == "}" => return true,
                TokenKind::Op if token.value == "," => {}
                _ => return false,
            }
            index += 1;
        }
        true
    }

    fn parse_assignment(&mut self, name_token: Token) -> Result<Vec<Node>, SyntaxError> {
        let operator = self.peek().clone();
        let op = match operator.value.as_str() {
            ":=" => AssignOp::Set,
            // Measured on the binary of v1.1.37.02 on 2026-11-01: the equal
            // sign assigns in the v1.
            "=" if self.dialect.assign_with_equal() => AssignOp::Set,
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the name of a
        // built-in variable that the dialect does not let a script write may
        // not be the output variable of an assignment, and the refusal is at
        // the load: it is the one of every scope, and a local variable may
        // not carry the name either.
        if let Some(error) = self.the_output_variable_is_refused(&name_token, op) {
            return Err(error);
        }
        self.next();
        let value = self.the_expression_of_a_statement()?;
        // Measured on the binary of v2.0.19 on 2026-10-31: the expression
        // ends at the comma, and the rest of the line is the list of v1.
        let legacy = self.the_legacy_list()?;
        self.end_of_statement()?;
        self.the_assignment_is_recorded(&name_token.value, op, name_token.line, name_token.column);
        let mut statements = vec![Node::Assign {
            name: name_token.value,
            op,
            value: Box::new(value),
            line: name_token.line,
        }];
        statements.extend(legacy);
        Ok(statements)
    }

    /// The refusal of a name that the dialect does not let a script write.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    /// built-in variable that the dialect does not let a script write may
    /// not be the output variable of an assignment, and the refusal is at
    /// the load. The compound operator has its own reason: it says that
    /// the variable cannot be assigned a value.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the reason of
    /// the v1 is the same for every operator, and the concat is the one
    /// that the v1 accepts.
    fn the_output_variable_is_refused(&self, token: &Token, op: AssignOp) -> Option<SyntaxError> {
        let message =
            self.dialect.the_refusal_of_the_write_of_a_built_in_variable(&token.value, op)?;
        Some(SyntaxError { message, line: token.line, column: token.column })
    }

    /// The refusal of the dialect to the increment of a value.
    ///
    /// The value is the one of the statement, and the refusal is the one of
    /// the line that is the increment alone: the increment that is inside an
    /// expression, or the one that has something after it, is not refused.
    ///
    /// The name of the value is the one that the script spelled, and the
    /// reason is the one of the dialect.
    fn the_refusal_of_the_increment(&self, node: &Node) -> Option<String> {
        let Node::Unary { op: UnaryOp::PostIncrement | UnaryOp::PostDecrement, operand, .. } = node
        else {
            return None;
        };
        let Node::Var { name, .. } = operand.as_ref() else { return None };
        self.dialect.the_refusal_of_the_increment_of_a_built_in_variable(name)
    }

    /// The list of v1 that may follow the expression of a statement.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the expression of
    /// a statement ends at the comma, and the rest of the line is the list
    /// of v1: it is evaluated, and its value is thrown away. The value
    /// that is not there in the list is a "Syntax error.", and the comma
    /// that follows it is named.
    fn the_legacy_list(&mut self) -> Result<Vec<Node>, SyntaxError> {
        if !self.at_op(&[","]) {
            return Ok(Vec::new());
        }
        self.position += 1;
        let mut items = Vec::new();
        loop {
            if self.at_end_of_statement() {
                // Measured on the binary of v2.0.19 on 2026-10-31: the line
                // that ends with a comma continues on the next one, and the
                // empty lines between the two are not a part of it.
                if self.peek().kind == TokenKind::Newline && self.the_line_continues() {
                    self.skip_newlines();
                } else {
                    break;
                }
            }
            if self.at_op(&[","]) {
                return Err(self.error(format!(
                    "Syntax error.\n     Specifically: {}",
                    the_source_of_the_rest_of_the_line(self, self.position)
                )));
            }
            items.push(self.parse_expression()?);
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        if items.is_empty() {
            return Ok(Vec::new());
        }
        Ok(vec![Node::Legacy { items, line: self.peek().line }])
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
            // Measured on the binary of v2.0.19 on 2026-10-31: the
            // parameter that is not there keeps its place, and it has no
            // value.
            if self.at_op(&[","]) {
                let line = self.peek().line;
                self.position += 1;
                // The parameter that is not there and that is the last one
                // is dropped: the commas at the end of the list add
                // nothing.
                if self.the_hole_is_the_last_one(")") {
                    break;
                }
                arguments.push(Node::Empty { line });
                // The list may be written in more than one line.
                self.skip_newlines();
                continue;
            }
            // The bracket that is not closed is the failure, and not the
            // operand that is not there inside it.
            let argument = match self.parse_expression() {
                Ok(argument) => argument,
                Err(error) => return Err(self.the_error_of_the_bracket(error, ")")),
            };
            arguments.push(argument);
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

    /// Whether the parameter that is not there is the last one of the list.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the commas at the
    /// end of the list add nothing, and the parameter that is not there and
    /// that is the last one is dropped with them. The list may be written in
    /// more than one line.
    fn the_hole_is_the_last_one(&self, bracket: &str) -> bool {
        let mut index = self.position;
        while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline {
            index += 1;
        }
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            match token.kind {
                TokenKind::Op if token.value == "," => index += 1,
                TokenKind::Op if token.value == bracket => return true,
                _ => return false,
            }
        }
        true
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
            // The line and the column of the name: the refusal of it names
            // the place where the name is.
            let line = name_token.line;
            let column = name_token.column;
            let name = if name_token.kind == TokenKind::Op && name_token.value == "*" {
                variadic = true;
                String::new()
            } else {
                // Measured on the binary of v2.0.19 on 2026-11-01: a parameter
                // may be named after a function of the machine and after a
                // class of the dialect -- `f(exitapp)` and `f(class)` are
                // both of them -- and the word that the language reserves
                // says that it may not be the name of a variable.
                let is_a_name = name_token.kind == TokenKind::Name
                    || (name_token.kind == TokenKind::Keyword
                        && matches!(
                            crate::lexer::the_refusal_of_a_word(&name_token.value),
                            Some(TheRefusal::Func) | Some(TheRefusal::Class)
                        ));
                if !is_a_name {
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
            // Measured on the binary of v2.0.19 on 2026-11-01: a parameter
            // may not carry the name of a built-in variable, and the
            // declaration says that it conflicts with it. The name is
            // named as the script spelled it.
            if crate::variables::is_built_in(&name) {
                return Err(SyntaxError {
                    message: format!(
                        "This parameter declaration conflicts with an existing built-in variable.\n     Specifically: {name}"
                    ),
                    line,
                    column,
                });
            }
            if !variadic && self.at_op(&["*"]) {
                self.position += 1;
                variadic = true;
            }
            let mut default = None;
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // writes the default value of a parameter with the equal sign as
            // well: `f(a = 5)` and `f(a := 5)` are the same.
            if self.at_op(&[":="])
                || (self
                    .dialect
                    .the_default_of_a_parameter_may_be_written_with_the_equal_sign()
                    && self.at_op(&["="]))
            {
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
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the fat arrow tells
    /// a definition from a call as well: `Name(a) => a + 1` is a function
    /// and not a call.
    fn a_definition_follows(&self) -> bool {
        self.a_definition_follows_at(self.position)
    }

    /// Whether a definition follows the parenthesis that is at `open`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    /// function may be a word of the dialect, and the parenthesis that comes
    /// after it and a body are what make it a definition.
    fn a_definition_follows_at(&self, open: usize) -> bool {
        let mut depth = 0usize;
        let mut at = open;
        while at < self.tokens.len() {
            let token = &self.tokens[at];
            if token.kind == TokenKind::Op {
                match token.value.as_str() {
                    "(" | "[" => depth += 1,
                    ")" | "]" => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            // The fat arrow is the body of the definition, and
                            // it may be on the line of the parameters or on
                            // the one after it.
                            let mut after = at + 1;
                            while self
                                .tokens
                                .get(after)
                                .map(|token| token.kind == TokenKind::Newline)
                                .unwrap_or(false)
                            {
                                after += 1;
                            }
                            if self
                                .tokens
                                .get(after)
                                .map(|token| token.value == "=>")
                                .unwrap_or(false)
                            {
                                return true;
                            }
                            // The brace may be on a line of its own.
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the construct before it reads the condition, and the
        // refusal of a condition that is empty comes after the one of the
        // body.
        let empty = self.the_parenthesis_of_a_condition_is_empty();
        let condition = if empty.is_some() {
            self.next();
            self.next();
            None
        } else {
            Some(self.the_expression_of_a_statement()?)
        };
        self.end_of_block()?;
        let body = self.parse_block("If", token.line)?;
        let Some(condition) = condition else {
            return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
        };
        branches.push(Branch { condition, body });
        let mut otherwise = Vec::new();
        loop {
            self.skip_newlines();
            // Measured on the binary of v2.0.19 on 2026-11-01: the branch of
            // an `if` is written with two words -- `else if` -- and the
            // `elseif` is a name like any other. The branch is a branch of
            // the same `if`.
            if self.at_keyword(&["else"]) {
                let keyword = self.next();
                self.the_construct_takes_no_parenthesis(keyword.line)?;
                self.end_of_block()?;
                if self.the_if_of_an_else_if_comes_next() {
                    self.skip_newlines();
                    let branch = self.next();
                    let empty = self.the_parenthesis_of_a_condition_is_empty();
                    let condition = if empty.is_some() {
                        self.next();
                        self.next();
                        None
                    } else {
                        Some(self.the_expression_of_a_statement()?)
                    };
                    self.end_of_block()?;
                    let body = self.parse_block("If", branch.line)?;
                    let Some(condition) = condition else {
                        return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
                    };
                    branches.push(Branch { condition, body });
                    continue;
                }
                otherwise = self.parse_block("Else", keyword.line)?;
            }
            break;
        }
        Ok(Node::If { branches, otherwise, line: token.line })
    }

    /// Whether the `if` of an `else if` comes next.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the branch of an `if`
    /// is written with two words, and the `if` may be on the line of the
    /// `else` or on the one after it.
    fn the_if_of_an_else_if_comes_next(&self) -> bool {
        let mut at = self.position;
        while self.tokens.get(at).map(|token| token.kind == TokenKind::Newline).unwrap_or(false) {
            at += 1;
        }
        self.tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("if"))
            .unwrap_or(false)
    }

    fn parse_switch(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        // Measured on the binary of v2.0.19 on 2026-11-01: the switch needs
        // a subject, and it names itself when there is none.
        if self.at_end_of_statement() {
            return Err(SyntaxError {
                message: "Unexpected \"Switch\"".to_string(),
                line: token.line,
                column: token.column,
            });
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the switch before it reads the subject, and the refusal
        // of a subject that is an empty parenthesis comes after the one of
        // the body.
        let empty = self.the_parenthesis_of_a_condition_is_empty();
        let subject = if empty.is_some() {
            self.next();
            self.next();
            None
        } else {
            Some(self.the_expression_of_a_statement()?)
        };
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
        let open_at = self.position;
        let open = self.expect_op("{")?;
        // Measured on the binary of v2.0.19 on 2026-11-01: the content of
        // the switch may begin on the line of the brace that opens it, and
        // the switch refuses it when the brace is not at the start of its
        // line.
        if !self.the_body_begins_on_the_next_line()
            && !self.the_brace_at_is_at_the_start_of_its_line(open_at)
        {
            return Err(self.error(self.the_message_of_the_opening_brace("Switch", open.line)));
        }
        let mut cases = Vec::new();
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the case before it refuses the values that are empty, and
        // the refusal waits for the body of the switch.
        let mut waiting: Option<SyntaxError> = None;
        loop {
            self.skip_newlines();
            if self.at_op(&["}"]) {
                self.the_closing_brace_is_refused(open_at)?;
                self.position += 1;
                break;
            }
            if self.at_keyword(&["case"]) {
                self.next();
                // Measured on the binary of v2.0.19 on 2026-11-01: a case may
                // name many values, and it matches when the subject of the
                // switch is one of them.
                let mut values = Vec::new();
                // Measured on the binary of v2.0.19 on 2026-11-01: the values
                // of a case may be none, and the colon is what says where
                // the label of the case ends.
                if !self.at_op(&[":"]) && !self.at_end_of_statement() {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // values of a case may be none, and the parenthesis that
                    // is empty is refused with the text that it read -- the
                    // closing parenthesis and not the rest of the line.
                    if let Some(closing) = self.the_parenthesis_of_a_condition_is_empty() {
                        waiting = Some(SyntaxError {
                            message: "Syntax error.\n     Specifically: )".to_string(),
                            line: closing.line,
                            column: closing.column,
                        });
                        self.next();
                        self.next();
                    } else {
                        loop {
                            values.push(self.the_expression_of_a_statement()?);
                            if !self.at_op(&[","]) {
                                break;
                            }
                            self.position += 1;
                            self.skip_newlines();
                            // The comma at the end of the list adds nothing.
                            if self.at_op(&[":"]) {
                                break;
                            }
                        }
                    }
                }
                let body = self.parse_case_body()?;
                cases.push(Case { values, body });
                continue;
            }
            // Measured on the binary of v2.0.19 on 2026-11-01: the `default`
            // is a name in v2, and the label of the default case is the name
            // and a colon. The name alone is not a label, and the body of
            // the switch says that it wants a case.
            if self.at_the_name("default") && self.a_colon_comes_after() {
                self.next();
                let body = self.parse_case_body()?;
                cases.push(Case { values: Vec::new(), body });
                continue;
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: v2 looks
            // for the brace that closes the switch before it reads the
            // cases, and a switch that is not closed says so.
            if !self.the_closing_brace_is_there() {
                let at = self.peek().clone();
                // Measured on the binary of v2.0.19 on 2026-11-01: the
                // switch that is empty names the statement of it, and the
                // switch that has cases names nothing.
                if cases.is_empty() && at.kind == TokenKind::Eof {
                    return Err(SyntaxError {
                        message: format!(
                            "Missing \"}}\"\n     Specifically: {}",
                            the_source_of_the_line(self, token.line)
                        ),
                        line: at.line,
                        column: at.column,
                    });
                }
                return Err(SyntaxError {
                    message: "Missing \"}\"".into(),
                    line: at.line,
                    column: at.column,
                });
            }
            // A statement that is not a case is refused in the words of v2.
            return Err(self.error("Expected Case/Default."));
        }
        let Some(subject) = subject else {
            return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
        };
        if let Some(error) = waiting {
            return Err(error);
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
            // The label of the next case ends the body of this one: the
            // `case` is a word of the dialect and the `default` is a name,
            // and the name is the label when a colon comes after it.
            if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("case") {
                break;
            }
            if token.kind == TokenKind::Name
                && token.value.eq_ignore_ascii_case("default")
                && self.a_colon_comes_after()
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
        // Measured on the binary of v2.0.19 on 2026-11-01: a return that
        // carries a value outside a function is refused at the load, and the
        // reason comes before the one of the expression that it carries.
        if !self.at_end_of_statement() && !self.in_function {
            return Err(SyntaxError {
                message: "Return's parameter should be blank except inside a function.".to_string(),
                line: token.line,
                column: token.column,
            });
        }
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the try takes no
        // parenthesis, and the dialect names the statement that carries it.
        self.the_construct_takes_no_parenthesis(token.line)?;
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
            self.the_construct_takes_no_parenthesis(keyword.line)?;
            // Measured on the binary of v2.0.19 on 2026-11-01: the else of a
            // try needs a catch before it, and it names itself when there is
            // none. The parenthesis that it carries is refused before that.
            if catches.is_empty() {
                return Err(SyntaxError {
                    message: "Unexpected \"Else\"".to_string(),
                    line: keyword.line,
                    column: keyword.column,
                });
            }
            self.end_of_block()?;
            otherwise = self.parse_block("Else", keyword.line)?;
        }
        let mut finally = Vec::new();
        self.skip_newlines();
        if self.at_keyword(&["finally"]) {
            let keyword = self.next();
            self.the_construct_takes_no_parenthesis(keyword.line)?;
            self.end_of_block()?;
            finally = self.parse_block("Finally", keyword.line)?;
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
            // Measured on the binary of v2.0.19 on 2026-11-01: a static
            // declaration may not carry the name of a built-in variable,
            // and the declaration says that it conflicts with it. The
            // global may carry it: it names the built-in variable of the
            // dialect and not a variable of the script.
            if kind != DeclareKind::Global && crate::variables::is_built_in(&name.value) {
                return Err(self.error(format!(
                    "This {} variable declaration conflicts with an existing built-in variable.\n     Specifically: {}",
                    kind.word(),
                    name.value
                )));
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // refuses the `global` of a built-in variable at the load, and
            // the name of it is in the refusal.
            if kind == DeclareKind::Global {
                if let Some(message) =
                    self.dialect.the_refusal_of_a_global_of_a_built_in_variable(&name.value)
                {
                    return Err(self.error(message));
                }
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the parenthesis
        // that is spaced is a part of the text of the name of the label --
        // `Goto (done)` looks for the label "(done)". The parenthesis that
        // is not spaced is the parenthesis of the dialect.
        let parenthesised = self.at_op(&["("]) && !self.peek().spaced;
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the name of the
        // label is the text of the source, and the space that the script
        // wrote inside it is not one: `Goto (nonewhere)` looks for the
        // label "(nonewhere)".
        let name = the_source_of_the_tokens(self, start, end);
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the load refuses
        // the goto whose label is not in the scope, and it names the label.
        // The name of a label does not know its case, and the scope of a
        // label is the scope it is written in: the label of a function is
        // not the label of the script.
        for (name, line) in the_gotos_of_a_scope(body) {
            if !seen.contains(&name.to_lowercase()) {
                return Err(SyntaxError {
                    message: format!("Label not found in current scope.\n     Specifically: {name}"),
                    line,
                    column: 1,
                });
            }
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

    /// Whether the character that is illegal is glued to the name that opens
    /// the statement.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
    /// semicolon that follow the name that opens the statement are a part of
    /// it, and the statement holds no action. Anywhere else they are illegal
    /// characters in an expression.
    fn the_illegal_character_is_glued_to_the_name_that_opens(&self) -> bool {
        match self.tokens.get(self.position + 1) {
            Some(token) => the_token_is_an_illegal_character(token) && !token.spaced,
            None => false,
        }
    }

    /// The body of a loop.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the `break` and the
    /// `continue` need a loop around them, and the count of the loops that
    /// the parser is inside says whether there is one.
    fn the_body_of_a_loop(&mut self, kind: &str, line: u32) -> Result<Vec<Node>, SyntaxError> {
        self.loops += 1;
        let body = self.parse_block(kind, line);
        self.loops -= 1;
        body
    }

    /// The failure of a `break` or a `continue` that has no loop around it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the message is the
    /// one of the dialect, and it names no text.
    fn the_loop_of_the_break_or_the_continue(&self, line: u32, column: u32) -> Result<(), SyntaxError> {
        if self.loops > 0 {
            return Ok(());
        }
        Err(SyntaxError {
            message: "Break/Continue must be enclosed by a Loop.".to_string(),
            line,
            column,
        })
    }

    /// The body of a block that is not the body of a loop.
    ///
    /// The body of a function is a scope of its own: the `break` and the
    /// `continue` of it do not belong to the loop that holds the function.
    fn the_body_of_a_scope(&mut self, kind: &str, line: u32) -> Result<Vec<Node>, SyntaxError> {
        let loops = self.loops;
        self.loops = 0;
        let body = self.parse_block(kind, line);
        self.loops = loops;
        body
    }

    /// Whether the fat arrow of a definition comes next.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the arrow may be on
    /// the line of the parameters or on the one after it.
    fn the_fat_arrow_comes_next(&mut self) -> bool {
        let mut at = self.position;
        while self
            .tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
            at += 1;
        }
        let arrow = self
            .tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Op && token.value == "=>")
            .unwrap_or(false);
        if arrow {
            self.position = at;
        }
        arrow
    }

    /// The body of a definition that is written with the fat arrow.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the body of a
    /// definition may be an expression, and the fat arrow is what says so:
    /// `f(a) => a + 1` is a function that answers the expression of it. The
    /// definition that has no body says so, and the text that is named is
    /// the definition.
    fn the_body_of_a_fat_arrow(&mut self, line: u32) -> Result<Vec<Node>, SyntaxError> {
        // The arrow of the definition.
        self.next();
        // The arrow may be on the line of the parameters or on the one after
        // it.
        self.skip_newlines();
        if self.peek().kind == TokenKind::Newline || self.peek().kind == TokenKind::Eof {
            return Err(SyntaxError {
                message: format!(
                    "Invalid function declaration.\n     Specifically: {}",
                    self.the_source_of_the_statement_of_the_line(line)
                ),
                line,
                column: self.peek().column,
            });
        }
        let value = self.the_expression_of_a_statement()?;
        self.end_of_statement()?;
        Ok(vec![Node::Return { value: Some(Box::new(value)), line }])
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the construct before it reads the condition, and the
        // refusal of a condition that is empty comes after the one of the
        // body.
        let empty = self.the_parenthesis_of_a_condition_is_empty();
        let condition = if empty.is_some() {
            self.next();
            self.next();
            None
        } else {
            Some(self.the_expression_of_a_statement()?)
        };
        self.end_of_block()?;
        let body = self.the_body_of_a_loop("While", token.line)?;
        let Some(condition) = condition else {
            return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
        };
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
        // The name of the class is the token that was read last: the place of
        // it is where the text of the refusal of the brace begins.
        let name_at = self.position - 1;
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the name of a
        // class may be the name of a function of the machine -- `class
        // ExitApp { }` is a class -- and the word that the language
        // reserves may not be the name of a class.
        let the_name_is_a_function = name_token.kind == TokenKind::Keyword
            && matches!(
                crate::lexer::the_refusal_of_a_word(&name_token.value),
                Some(TheRefusal::Func)
            );
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // v1 takes the name of a class as it is written, and a word of the
        // language is a name of it: the `class if` is a class.
        let the_name_is_a_word = self.dialect.the_name_of_a_class_may_be_a_word_of_the_language()
            && name_token.kind == TokenKind::Keyword;
        if name_token.kind != TokenKind::Name && !the_name_is_a_function && !the_name_is_a_word {
            // Measured on the binary of v2.0.19 on 2026-10-31: a name that
            // begins with a number is refused by the lexer, a reserved word
            // is a word that cannot be a name, and the brace is what is
            // unexpected when there is no name at all.
            let message = match name_token.kind {
                TokenKind::Number => format!(
                    "This class name starts with a number, which is not allowed:\n{:?}",
                    name_token.value
                ),
                TokenKind::Keyword => {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // `Class` is a class of the dialect, and a class of the
                    // script may not have its name.
                    if matches!(
                        crate::lexer::the_refusal_of_a_word(&name_token.value),
                        Some(TheRefusal::Class)
                    ) {
                        "This class declaration conflicts with an existing Class.\n     Specifically: Class".to_string()
                    } else {
                        format!(
                            "The following reserved word must not be used as a class name:\n{:?}",
                            name_token.value
                        )
                    }
                }
                _ => "Unexpected \"{\"".to_string(),
            };
            return Err(SyntaxError { message, line: name_token.line, column: name_token.column });
        }
        let mut extends = None;
        // Measured on the binary of v2.0.19 on 2026-11-01: the `extends` is a
        // name in v2, and the base of a class is the name that comes after
        // it.
        if self.at_the_name("extends") {
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
        let members = self.parse_class_body(name_token.line, &name, name_at)?;
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
    fn parse_the_class_with_no_name(
        &mut self,
        token: Token,
        name: &str,
        class_at: usize,
    ) -> Result<(), SyntaxError> {
        // The brace of the class is the token that was read as its name, and
        // the name of the class is where the text of the refusal begins.
        self.parse_the_members_of_a_class(token.line, name, class_at, class_at)?;
        Ok(())
    }

    /// The members of a class: the methods and the properties.
    fn parse_class_body(&mut self, line: u32, name: &str, name_at: usize) -> Result<TheMembersOfAClass, SyntaxError> {
        // The brace may be on a line of its own.
        self.skip_newlines();
        if !self.at_op(&["{"]) {
            // Measured on the binary of v2.0.19 on 2026-10-31: the class
            // needs the braces, and the line of the class is the one that is
            // named.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
            // the v1 names the text that comes after the word "class", and
            // the name of the class is where that text begins.
            let text = if self.dialect.the_missing_brace_of_a_class_names_the_class() {
                the_source_of_the_rest_of_the_line(self, name_at)
            } else {
                the_source_of_the_line(self, line)
            };
            return Err(SyntaxError {
                message: format!("Missing \"{{\"\n     Specifically: {text}"),
                line,
                column: self
                    .tokens
                    .iter()
                    .find(|token| token.line == line)
                    .map(|token| token.column)
                    .unwrap_or(1),
            });
        }
        let open = self.position;
        self.expect_op("{")?;
        self.parse_the_members_of_a_class(line, name, name_at, open)
    }

    /// The members of a class, after the brace that opens them.
    fn parse_the_members_of_a_class(
        &mut self,
        line: u32,
        name: &str,
        class_at: usize,
        open: usize,
    ) -> Result<TheMembersOfAClass, SyntaxError> {
        if !self.the_body_begins_on_the_next_line() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
            // the v1 names the text that comes after the word "class", and
            // the name of the class is where that text begins.
            let message = if self.dialect.the_missing_brace_of_a_class_names_the_class() {
                format!(
                    "Missing \"{{\"\n     Specifically: {}",
                    the_source_of_the_rest_of_the_line(self, class_at)
                )
            } else {
                self.the_message_of_the_opening_brace("class", line)
            };
            return Err(self.error(message));
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // body of a class of the v1 has one set of names, and a member that
        // is written twice is refused whatever it is.
        let mut the_names: Vec<String> = Vec::new();
        let one_set = self.dialect.the_names_of_the_body_of_a_class_are_one_set();
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
                self.the_closing_brace_is_refused(open)?;
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
            // The place of the token that was read: the brace of a class
            // with no name is the token that was read as its name.
            let name_at = self.position - 1;
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
                        self.parse_the_class_with_no_name(name_token, name, name_at)?;
                        continue;
                    }
                }
                let key = nested_name.value.to_lowercase();
                if class_properties.contains(&key) {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-02: the v1 refuses the class that is declared
                    // twice with "Duplicate class definition.\n     Specifically:
                    // D", and the name is the one that was written.
                    let message = if self.dialect.the_class_that_is_declared_twice_has_its_own_words() {
                        format!("Duplicate class definition.\n     Specifically: {}", nested_name.value)
                    } else {
                        format!(
                            "Duplicate declaration.\n     Specifically: {}",
                            nested_name.value
                        )
                    };
                    return Err(SyntaxError {
                        message,
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
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
                // has no static method, and it refuses it with the words of
                // the v1: "Invalid class variable declaration.\n     Specifically:
                // __Init() {". The text is the member, from the name to the
                // brace that opens the body.
                if is_static && self.dialect.the_static_method_is_refused() {
                    return Err(SyntaxError {
                        message: format!(
                            "Invalid class variable declaration.\n     Specifically: {}",
                            self.the_text_of_a_static_member(name_at)
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
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
                    // The name of the `__Init` of the v1 is the name that was
                    // written, and not the path of it.
                    let owner = if self.dialect.the_duplicate_names_the_member_that_was_written() {
                        String::new()
                    } else {
                        format!("{name}.Prototype.")
                    };
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {owner}{}",
                            name_token.value
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
                    let owner = if self.dialect.the_duplicate_names_the_member_that_was_written() {
                        String::new()
                    } else if is_static {
                        format!("{name}.")
                    } else {
                        format!("{name}.Prototype.")
                    };
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {owner}{}",
                            name_token.value
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                taken.push(key.clone());
                // The body of a class of the v1 has one set of names: the
                // member is a duplicate of any member that has its name.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
                // "Duplicate declaration.\n     Specifically: x", and the name
                // is the one that was written.
                if one_set {
                    if the_names.contains(&name_token.value.to_lowercase()) {
                        return Err(SyntaxError {
                            message: format!(
                                "Duplicate declaration.\n     Specifically: {}",
                                name_token.value
                            ),
                            line: name_token.line,
                            column: name_token.column,
                        });
                    }
                    the_names.push(name_token.value.to_lowercase());
                }
                self.next();
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                self.parameters.push(parameters.iter().map(|parameter| parameter.name.clone()).collect());
                self.assignments.push(Vec::new());
                // A method may not contain a class either.
                let was_in_a_function = self.in_function;
                self.in_function = true;
                // Measured on the binary of v2.0.19 on 2026-11-01: the body of
                // a method may be an expression too, and the fat arrow is
                // what says so.
                let names_the_class = self.dialect.the_class_that_is_not_closed_names_the_class();
                let body = if self.the_fat_arrow_comes_next() {
                    self.the_body_of_a_fat_arrow(name_token.line)?
                } else {
                    self.the_body_of_a_scope("method", name_token.line).map_err(|mut error| {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-02: the v1 names the class when the file
                        // ends inside a method of it.
                        if names_the_class && error.message.starts_with("Missing \"}\"") {
                            error.message = format!("Missing \"}}\"\n     Specifically: {name}");
                        }
                        error
                    })?
                };
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
            //
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the class
            // of the v1 has no property with a getter, and the refusal is the
            // one of the v1: the getter that is on the same line as the name
            // with "Not a valid method, class or property definition.\n
            // Specifically: x { get => 1 }", and the getter that is in a block
            // with "Not a valid property getter/setter.\n     Specifically:
            // get => 1".
            if self.at_op(&["[", "{"]) {
                if self.dialect.the_accessor_of_a_class_is_refused() {
                    return Err(self.the_refusal_of_an_accessor(name_token));
                }
                let key = name_token.value.to_lowercase();
                let taken = if is_static { &mut class_accessors } else { &mut instance_accessors };
                if taken.contains(&key) {
                    return Err(SyntaxError {
                        message: format!("Duplicate declaration.\n     Specifically: {}", name_token.value),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                taken.push(key.clone());
                // The body of a class of the v1 has one set of names: the
                // property is a duplicate of any member that has its name.
                if one_set {
                    if the_names.contains(&key) {
                        return Err(SyntaxError {
                            message: format!(
                                "Duplicate declaration.\n     Specifically: {}",
                                name_token.value
                            ),
                            line: name_token.line,
                            column: name_token.column,
                        });
                    }
                    the_names.push(key);
                }
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
            taken.push(key.clone());
            // The body of a class of the v1 has one set of names: the
            // property is a duplicate of any member that has its name.
            if one_set {
                if the_names.contains(&key) {
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {}",
                            name_token.value
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                the_names.push(key);
            }
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

    /// The text of a static member of a class that the dialect refuses.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text is the
    /// member, from the name to the brace that opens the body: "__Init() {".
    fn the_text_of_a_static_member(&self, name_at: usize) -> String {
        let mut end = name_at;
        while end < self.tokens.len() {
            let token = &self.tokens[end];
            if token.kind == TokenKind::Op && token.value == "{" {
                break;
            }
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                break;
            }
            end += 1;
        }
        let text = the_source_of_the_tokens(self, name_at, end);
        if self.tokens.get(end).map(|token| token.value == "{").unwrap_or(false) {
            format!("{text} {{")
        } else {
            text
        }
    }

    /// The refusal of the property that has a getter or a setter.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the getter that is
    /// on the same line as the name is refused with "Not a valid method, class
    /// or property definition.\n     Specifically: x { get => 1 }", and the
    /// getter that is in a block with "Not a valid property
    /// getter/setter.\n     Specifically: get => 1".
    fn the_refusal_of_an_accessor(&self, name_token: Token) -> SyntaxError {
        // The brace opens the body of the getter, and the token that comes
        // after it says whether the getter is on the same line as the name.
        let mut at = self.position;
        while at < self.tokens.len() {
            let token = &self.tokens[at];
            if token.kind == TokenKind::Op && token.value == "{" {
                break;
            }
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                break;
            }
            at += 1;
        }
        // The token that comes after the brace: the newline that ends the line
        // of the member is not a part of it.
        let mut after = at + 1;
        while self
            .tokens
            .get(after)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
            after += 1;
        }
        let after = self.tokens.get(after);
        let on_the_same_line = after
            .map(|token| token.line == name_token.line)
            .unwrap_or(false);
        if on_the_same_line {
            return SyntaxError {
                message: format!(
                    "Not a valid method, class or property definition.\n     Specifically: {}",
                    the_source_of_the_line(self, name_token.line)
                ),
                line: name_token.line,
                column: name_token.column,
            };
        }
        // The getter is in a block, and the refusal is at the line of it.
        let line = after.map(|token| token.line).unwrap_or(name_token.line);
        SyntaxError {
            message: format!(
                "Not a valid property getter/setter.\n     Specifically: {}",
                the_source_of_the_line(self, line)
            ),
            line,
            column: name_token.column,
        }
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

    /// The loop of v2, and the kinds of it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the word that
    /// follows the `Loop` is a name of the kind when the line goes on
    /// after it, and it is the count of the loop when the line ends
    /// there. The kinds are `Files`, `Reg`, `Read` and `Parse`, and each
    /// of them takes its own number of parameters.
    fn parse_loop(&mut self) -> Result<Node, SyntaxError> {
        // The place where the statement begins: the messages of v2 name
        // the text of it.
        let head = self.position;
        let token = self.next();
        let kind = self.the_kind_of_the_loop();
        let mut arguments = Vec::new();
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the construct before it reads the count of it, and the
        // refusal of a count that is empty comes after the one of the body.
        let mut empty = None;
        // The name of the kind, as the script spelled it.
        let mut name = String::new();
        // The end of the first parameter, for the message that names it.
        let mut first_end = None;
        match kind {
            Some(kind) => {
                name = self.next().value;
                // The parameters of the command: the first one may be
                // written without a comma, and the rest are separated by
                // commas.
                let mut comma = false;
                if self.at_op(&[","]) {
                    comma = true;
                    self.position += 1;
                    if self.at_end_of_statement() {
                        // The list of the parameters is empty.
                        return Err(self.error(format!(
                            "\"Loop {name}\" requires at least 1 parameter.\n     Specifically: {}",
                            the_source_of_the_line(self, token.line)
                        )));
                    }
                }
                if comma {
                    loop {
                        if self.at_op(&["{"]) {
                            // The body is where the parameter should be.
                            return Err(self.error(format!("Unexpected \"Loop {name}\"")));
                        }
                        self.the_expression_begins();
                        arguments.push(self.parse_expression()?);
                        if first_end.is_none() {
                            first_end = Some(self.position);
                        }
                        if !self.at_op(&[","]) {
                            break;
                        }
                        self.position += 1;
                    }
                } else if !self.at_end_of_statement() && !self.at_op(&["::", "{"]) {
                    // The first parameter may be written without a comma.
                    self.the_expression_begins();
                    arguments.push(self.parse_expression()?);
                    first_end = Some(self.position);
                    while self.at_op(&[","]) {
                        self.position += 1;
                        if self.at_op(&["{"]) {
                            // The body is where the parameter should be.
                            return Err(self.error(format!("Unexpected \"Loop {name}\"")));
                        }
                        self.the_expression_begins();
                        arguments.push(self.parse_expression()?);
                    }
                }
                // The command takes a number of parameters, and the one
                // that is written too many is named with the first of
                // them.
                let most = the_number_of_parameters_of_a_loop(kind);
                if arguments.len() > most {
                    return Err(self.error(format!(
                        "\"Loop {name}\" accepts at most {most} parameters.\n     Specifically: {}",
                        the_source_of_the_tokens(self, head, first_end.unwrap_or(self.position))
                    )));
                }
            }
            None => {
                // The count of the loop. It may be left out, and the loop
                // goes on until a break.
                if self.at_op(&[","]) {
                    // Measured on the binary of v1.1.37.02 on 2026-11-01: the
                    // count of the loop of the v1 comes after a comma:
                    // `Loop, 3`.
                    if self.dialect.legacy_commands() {
                        self.next();
                        self.the_expression_begins();
                        arguments.push(self.parse_expression()?);
                        if self.at_op(&[","]) {
                            // The kinds of the loop of the v1 come after
                            // the count, and the tree does not carry them
                            // yet.
                            return Err(self.error(format!(
                                "Invalid loop type.\n     Specifically: {}",
                                the_source_of_the_line(self, token.line)
                            )));
                        }
                    } else {
                        // The comma of a call is not there.
                        return Err(self.error(format!(
                            "Function calls require a space or \"(\".  Use comma only between parameters.\n     Specifically: {}",
                            the_source_of_the_line(self, token.line)
                        )));
                    }
                } else {
                    empty = self.the_parenthesis_of_a_condition_is_empty();
                    if empty.is_some() {
                        self.next();
                        self.next();
                    } else if !self.at_end_of_statement() && !self.at_op(&["::", "{"]) {
                        self.the_expression_begins();
                        arguments.push(self.parse_expression()?);
                        if self.at_op(&[","]) {
                            // The count of the loop is not a name of a kind.
                            return Err(self.error(format!(
                                "Invalid loop type.\n     Specifically: {}",
                                the_source_of_the_line(self, token.line)
                            )));
                        }
                    }
                }
            }
        }
        self.the_expression_is_done(self.position, false)?;
        self.end_of_block()?;
        let body_start = self.position;
        let body = self.the_body_of_a_loop(&the_name_of_the_loop(&name), token.line)?;
        if let Some(closing) = empty {
            return Err(self.the_refusal_of_an_empty_parenthesis(&closing));
        }
        // The command of v2 checks its parameters when the script loads,
        // and the parameter that is not there is named.
        if kind.is_some() && arguments.is_empty() {
            return Err(self.error(format!(
                "Parameter #1 required\n     Specifically: {}",
                the_source_of_the_head(self, head, body_start)
            )));
        }
        Ok(Node::Loop { mode: kind.unwrap_or(LoopKind::Count), arguments, body, line: token.line })
    }

    /// The kind of the loop, when the word that follows the `Loop` names
    /// one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the word is a
    /// name of the kind when the line goes on after it, and it is the
    /// count of the loop when the line ends there.
    fn the_kind_of_the_loop(&self) -> Option<LoopKind> {
        let word = self.peek();
        if word.kind != TokenKind::Name {
            return None;
        }
        let kind = match word.value.to_lowercase().as_str() {
            "files" => LoopKind::Files,
            "reg" => LoopKind::Reg,
            "read" => LoopKind::Read,
            "parse" => LoopKind::Parse,
            _ => return None,
        };
        // The line goes on after the word: the token that follows it is
        // on the same line.
        let next = self.tokens.get(self.position + 1)?;
        if next.line != word.line || matches!(next.kind, TokenKind::Newline | TokenKind::Eof) {
            return None;
        }
        Some(kind)
    }

    /// The loop over what an object enumerates: `for x in y { }`.
    ///
    /// The names are the ones the enumerator gives, and a name may be
    /// left out: `for , value in map` asks for the value alone. The whole
    /// list may also be written between parentheses, the way v2 writes
    /// it when the expression is long.
    fn parse_for(&mut self) -> Result<Node, SyntaxError> {
        // The place where the statement begins: the message of the v2 names
        // the text of it.
        let head = self.position;
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
        self.parse_for_variables(&mut variables, head, parenthesised)?;
        if !self.at_keyword(&["in"]) {
            // Measured on the binary of v2.0.19 on 2026-11-01: the `for` needs
            // the `in`, and the dialect names the statement as it was written.
            return Err(self.error(format!(
                "Syntax error.\n     Specifically: {}",
                the_source_of_the_line(self, token.line)
            )));
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
        let body = self.the_body_of_a_loop("For", token.line)?;
        Ok(Node::For { variables, source: Box::new(source), body, line: token.line })
    }

    /// The names the enumerator gives, before the `in`.
    ///
    /// A name may be left out, and the name that is left out is the
    /// empty string: `for , value in map` gives `["", "value"]`.
    fn parse_for_variables(
        &mut self,
        variables: &mut Vec<String>,
        head: usize,
        parenthesised: bool,
    ) -> Result<(), SyntaxError> {
        // The first name. It is left out when the comma comes first.
        if self.at_op(&[","]) {
            variables.push(String::new());
        } else if self.peek().kind == TokenKind::Name {
            // Measured on the binary of v2.0.19 on 2026-11-01: the variable
            // of a for is an output variable, and the name of a built-in
            // variable that the dialect does not let a script write is
            // refused.
            let token = self.next();
            if let Some(error) = self.the_output_variable_is_refused(&token, AssignOp::Set) {
                return Err(error);
            }
            variables.push(token.value);
        } else if !self.at_keyword(&["in"]) {
            let token = self.peek().clone();
            // Measured on the binary of v2.0.19 on 2026-11-01: the v2 names the
            // text that it read: the statement of the `for` up to the
            // parenthesis that closes the list, and the line of it when the
            // list is not written between parentheses.
            let text = if parenthesised {
                the_source_of_the_tokens(self, head, self.the_end_of_the_parenthesis(self.position))
            } else {
                the_source_of_the_line(self, token.line)
            };
            return Err(SyntaxError {
                message: format!("Syntax error.\n     Specifically: {text}"),
                line: token.line,
                column: token.column,
            });
        }
        // The second name, after the comma that separates the two.
        if self.at_op(&[","]) {
            self.next();
            if self.peek().kind == TokenKind::Name {
                let token = self.next();
                if let Some(error) = self.the_output_variable_is_refused(&token, AssignOp::Set) {
                    return Err(error);
                }
                variables.push(token.value);
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
        let left = self.parse_unary()?;
        self.parse_binary_after(left, min)
    }

    /// The binary operators that come after the value that was read.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the expression of
    /// a statement may begin with a value that is already read -- the call
    /// of a statement, for instance -- and the operators that follow it
    /// are a part of it.
    fn parse_binary_after(&mut self, mut left: Node, min: u8) -> Result<Node, SyntaxError> {
        loop {
            // The ternary is not in the table: it takes two expressions
            // and a colon, and the colon belongs to it.
            if self.at_op(&["?"]) && precedence::TERNARY >= min {
                let at = self.position;
                let token = self.next();
                let then = self.parse_binary(precedence::COALESCE)?;
                // Measured on the binary of v2.0.19 on 2026-11-01: the
                // question that has no colon says so, and the text that is
                // named begins at the question mark.
                if let Err(error) = self.expect_op(":") {
                    if !error.message.starts_with("Missing \":\"") {
                        return Err(error);
                    }
                    return Err(SyntaxError {
                        message: format!(
                            "A \"?\" is missing its \":\"\n     Specifically: {}",
                            the_source_of_the_rest_of_the_line(self, at)
                        ),
                        line: token.line,
                        column: token.column,
                    });
                }
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
                if precedence::CONCAT < min || !self.starts_a_concatenation() {
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
            // Measured on the binary of v2.0.19 on 2026-11-01: the or-maybe
            // wants a variable on its left, and the v2 refuses the script at
            // the load when it is another value. The text that is named is
            // the rest of the line, from the operator.
            if op == BinaryOp::Coalesce && !matches!(left, Node::Var { .. }) {
                return Err(self.error(format!(
                    "Syntax error.\n     Specifically: {}",
                    the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1))
                )));
            }
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
                // Measured on the binary of v1.1.37.02 on 2026-11-01: the
                // sum of the v1 joins a text and a number, and the operator
                // that wants a number may follow a literal text when it is
                // the sum.
                && !(self.dialect.add_concatenates() && op == BinaryOp::Add)
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
            // The percent sign opens a name that the script builds, and it
            // is a value of the concatenation: `"a" %x%` is the text and the
            // value of the variable that `x` names.
            TokenKind::Op if token.value == "%" => true,
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

    /// Whether the value that follows may be the one of a concatenation.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the percent sign
    /// that closes a name that the script builds is the end of it, and not
    /// the beginning of another one: `%x%` is one name, and not the value
    /// of `x` and the name of another variable.
    fn starts_a_concatenation(&self) -> bool {
        let token = self.peek();
        if token.kind == TokenKind::Op && token.value == "%" {
            return self.derefs == 0;
        }
        self.starts_expression()
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
        self.the_line_continues_at(self.position)
    }

    /// Whether the line continues on the next one, at the given place.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a line that ends
    /// with an operator continues on the next one, and the empty lines
    /// between the two are not a part of it. The increment and the
    /// decrement are not among the operators: the line that ends with one
    /// of them wants an operand, and it does not continue.
    fn the_line_continues_at(&self, at: usize) -> bool {
        let Some(token) = self.the_token_before(at) else { return false };
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

    /// Whether the word that opens the statement may not be the name of a
    /// variable.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the words that are
    /// not operators -- `not`, `and`, `or` and `is` -- and the words that
    /// are reserved for a future of v2 -- `as`, `in` and `contains` -- may
    /// not be the name of a variable. The two words that are the values of
    /// the dialect -- `true` and `false` -- are values and not names.
    fn the_word_may_not_be_a_name(&self) -> bool {
        let token = self.peek();
        if token.kind != TokenKind::Keyword {
            return false;
        }
        let word = token.value.to_lowercase();
        if word == "true" || word == "false" {
            return false;
        }
        matches!(
            crate::lexer::the_refusal_of_a_word(&word),
            Some(TheRefusal::Syntax) | Some(TheRefusal::Future)
        )
    }

    /// Whether an operand follows the word that opens the statement.
    fn an_operand_follows_the_word(&self) -> bool {
        let Some(next) = self.tokens.get(self.position + 1) else { return false };
        !matches!(next.kind, TokenKind::Newline | TokenKind::Eof)
            && !(next.kind == TokenKind::Op && next.value == "}")
    }

    /// The failure of a word that the language reserves.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the reason belongs
    /// to the word, and it is the same in every place where the word is
    /// refused. The words that begin a statement say that the word may not
    /// be the name of a variable; the words that are operators, the two
    /// that are the values of the dialect and the `super` are a syntax
    /// error; and the words that are reserved for a future of v2 say that
    /// they are reserved. The `ExitApp` is a function of the machine and
    /// the `Class` is a class of the dialect, and neither of them may be
    /// written to.
    ///
    /// The text that is named is the word alone.
    fn the_error_of_the_refusal(&self, refusal: TheRefusal, token: &Token) -> SyntaxError {
        let message = match refusal {
            TheRefusal::Variable => format!(
                "The following reserved word must not be used as a variable name:\n{:?}",
                token.value
            ),
            TheRefusal::Syntax => format!("Syntax error.\n     Specifically: {}", token.value),
            TheRefusal::Future => {
                format!("Unexpected reserved word.\n     Specifically: {}", token.value)
            }
            TheRefusal::Func => format!(
                "This Func cannot be used as an output variable.\n     Specifically: {}",
                token.value
            ),
            TheRefusal::Class => {
                "This Class cannot be used as an output variable.\n     Specifically: Class".to_string()
            }
        };
        SyntaxError { message, line: token.line, column: token.column }
    }

    /// The failure of the reserved word that is the name of a variable.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the reason is the
    /// one of the word, and the text that is named is the word alone.
    fn the_error_of_the_reserved_word_as_a_name(&self, at: usize) -> SyntaxError {
        let token = &self.tokens[at];
        let word = token.value.to_lowercase();
        let refusal = crate::lexer::the_refusal_of_a_word(&word).unwrap_or(TheRefusal::Variable);
        self.the_error_of_the_refusal(refusal, token)
    }

    /// The failure of the reserved word that is the name of a call.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the word alone at
    /// the start of a statement is a call, and the text that is named is
    /// the call with its parentheses: "not()".
    fn the_error_of_the_reserved_word_as_a_call(&self, at: usize) -> SyntaxError {
        let token = &self.tokens[at];
        let word = token.value.to_lowercase();
        let message = match crate::lexer::the_refusal_of_a_word(&word) {
            Some(TheRefusal::Future) => "Unexpected reserved word.",
            _ => "Syntax error.",
        };
        SyntaxError {
            message: format!("{message}\n     Specifically: {}", self.the_call_of_the_word(at)),
            line: token.line,
            column: token.column,
        }
    }

    /// The call that the word at the head of the statement wanted to make.
    fn the_call_of_the_word(&self, at: usize) -> String {
        let name = self.tokens[at].value.clone();
        let arguments = the_source_of_the_rest_of_the_line(self, at + 1);
        if arguments.is_empty() {
            return format!("{name}()");
        }
        if arguments.starts_with('(') {
            return format!("{name}{arguments}");
        }
        format!("{name}({arguments})")
    }

    /// Whether the word that opens the statement is the name of a variable.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the word that may
    /// not be the name of a variable says so when it is the name of one,
    /// and the operator of the assignment is what tells the two apart.
    fn the_word_is_the_name_of_a_variable(&self) -> bool {
        let Some(next) = self.tokens.get(self.position + 1) else { return false };
        next.kind == TokenKind::Op && the_assignment_operator(&next.value).is_some()
    }

    /// Whether the word that opens the statement is the name of a function.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    /// function may be a word of the dialect -- `until() { }` is a function
    /// -- and the parenthesis that comes after it and a body are what make
    /// it a definition.
    fn the_word_is_the_name_of_a_function(&self) -> bool {
        let Some(open) = self.tokens.get(self.position + 1) else { return false };
        if open.kind != TokenKind::Op || open.value != "(" || open.spaced {
            return false;
        }
        self.a_definition_follows_at(self.position + 1)
    }

    /// The error of the reserved word that is not wanted.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the words that may
    /// not be the name of a variable say so -- "The following reserved word
    /// must not be used as a variable name:", and the words that are
    /// reserved for a future of v2 -- `in` and `contains` -- say "Unexpected
    /// reserved word.". The words that are not operators say "Syntax
    /// error.". The text that is named is the rest of the line, from the
    /// word.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the words that the
    /// v2 does not reserve are names, and the name of a function of the
    /// machine and the name of a class of the dialect may not be written
    /// to.
    fn the_error_of_the_reserved_word(&self, at: usize) -> SyntaxError {
        let token = &self.tokens[at];
        let word = token.value.to_lowercase();
        let message = match crate::lexer::the_refusal_of_a_word(&word) {
            Some(TheRefusal::Future) => format!(
                "Unexpected reserved word.\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, at)
            ),
            Some(TheRefusal::Syntax) => format!(
                "Syntax error.\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, at)
            ),
            Some(TheRefusal::Func) => format!(
                "This Func cannot be used as an output variable.\n     Specifically: {}",
                token.value
            ),
            Some(TheRefusal::Class) => {
                "This Class cannot be used as an output variable.\n     Specifically: Class".to_string()
            }
            _ => format!("The following reserved word must not be used as a variable name:\n{:?}", word),
        };
        SyntaxError { message, line: token.line, column: token.column }
    }

    /// The error of the character that is not a comment and not a directive.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
    /// semicolon are illegal characters when they are glued to the token
    /// that comes before them. The one that is inside an expression is an
    /// illegal character in an expression, and the one that begins a
    /// statement is a line that holds no action.
    fn the_error_of_the_illegal_character(&self, at: usize, in_an_expression: bool) -> SyntaxError {
        let token = &self.tokens[at];
        let text = the_text_of_the_illegal_character(self, at, in_an_expression);
        let message = if in_an_expression {
            format!("Illegal character in expression.\n     Specifically: {text}")
        } else {
            format!("This line does not contain a recognized action.\n     Specifically: {text}")
        };
        SyntaxError { message, line: token.line, column: token.column }
    }

    /// The error of the double deref that does not close.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text that is
    /// named is the rest of the line, from the hash that opens it.
    fn the_error_of_a_missing_ending(&self, at: usize) -> SyntaxError {
        let token = &self.tokens[at];
        SyntaxError {
            message: format!(
                "Missing ending \"%\"\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, at)
            ),
            line: token.line,
            column: token.column,
        }
    }

    /// The expression that is inside a name that the script builds.
    ///
    /// The percent sign that closes the name is the end of it, and not the
    /// beginning of another one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the operand that is
    /// not there means that the name does not close, and v2 says so: the
    /// error of the name names the rest of the line, from the name.
    fn the_expression_of_a_dynamic_name(&mut self, at: usize) -> Result<Node, SyntaxError> {
        self.derefs += 1;
        let value = self.parse_expression();
        self.derefs -= 1;
        match value {
            Ok(value) => Ok(value),
            Err(error) => {
                if error.message.starts_with("Missing operand.") {
                    return Err(self.the_error_of_a_missing_ending(at));
                }
                Err(error)
            }
        }
    }

    /// The refusal of the percent sign that is inside a name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the percent sign
    /// that is inside a name that the script builds is not a value, and v2
    /// says "Syntax error." and the rest of the line, from it.
    fn the_error_of_a_percent_inside(&self) -> SyntaxError {
        self.error(format!(
            "Syntax error.\n     Specifically: {}",
            the_source_of_the_rest_of_the_line(self, self.position)
        ))
    }

    /// The pieces of a name that the script builds.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    /// dynamic variable reference is the text that is written and the values
    /// of the expressions, and the space between two of them is what tells
    /// that the name is over: `MyArray%i%` is one name and `MyArray %i%` is
    /// the variable `MyArray` and the value of the variable `i`.
    ///
    /// The first piece is the text that is written, and the ones that come
    /// after it are the values of the expressions between the percent signs.
    ///
    /// The place of the name is where the text that is named begins: the
    /// error of the name that does not close names the rest of the line
    /// from there.
    fn the_pieces_of_a_dynamic_name(
        &mut self,
        first: String,
        at: usize,
        line: u32,
    ) -> Result<Vec<DynamicPiece>, SyntaxError> {
        let mut pieces = vec![DynamicPiece::Text(first)];
        while self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
            self.next();
            if self.at_op(&["%"]) {
                return Err(self.the_error_of_a_percent_inside());
            }
            let value = self.the_expression_of_a_dynamic_name(at)?;
            if !self.at_op(&["%"]) {
                return Err(self.the_error_of_a_missing_ending(at));
            }
            self.next();
            pieces.push(DynamicPiece::Value(Box::new(value)));
        }
        let _ = line;
        Ok(pieces)
    }

    /// The pieces of a name that the script builds, the first of them being
    /// the value of an expression.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the percent signs
    /// that open and that close a piece may have a space around them, and
    /// the pieces that follow are joined without a space.
    fn the_pieces_after_the_percent(&mut self, value: Node) -> Result<Vec<DynamicPiece>, SyntaxError> {
        let mut pieces = vec![DynamicPiece::Value(Box::new(value))];
        while self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
            let open = self.position;
            self.next();
            if self.at_op(&["%"]) {
                return Err(self.the_error_of_a_percent_inside());
            }
            let value = self.the_expression_of_a_dynamic_name(open)?;
            if !self.at_op(&["%"]) {
                return Err(self.the_error_of_a_missing_ending(open));
            }
            self.next();
            pieces.push(DynamicPiece::Value(Box::new(value)));
        }
        Ok(pieces)
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
            // Measured on the binary of v2.0.19 on 2026-10-31: the dot is
            // the member of a value that is not there, and the text that is
            // named is the rest of the line, from the dot.
            TokenKind::Op if token.value == "." => format!(
                "Syntax error.\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1))
            ),
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
            // Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
            // semicolon that are not a directive and not a comment are illegal
            // characters in an expression, and the text of the expression is
            // named.
            TokenKind::Op if the_token_is_an_illegal_character(token) => {
                return self.the_error_of_the_illegal_character(self.position.saturating_sub(1), true)
            }
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
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // text that is named begins at the operator, and it is
                    // the source of the line from there: `x := 1 === 1` names
                    // "=== 1".
                    Some(at) => {
                        format!(
                            "Missing operand.\n     Specifically: {}",
                            the_source_of_the_rest_of_the_line(self, at)
                        )
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
    fn the_operator_that_wants_the_operand(&self) -> Option<usize> {
        let at = self.position.saturating_sub(2);
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
            Some(at)
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
                let at_the_operand = self.position;
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02:
                // the sign of the v1 is part of the number that was written:
                // the `+0x10` is "0x10", and the `-00123` is "-123", and the
                // `-1.50` is "-1.50". The sign of a number that is not a
                // literal is an operation of its own, and so is the sign of
                // a number that is in a group: the `-(1.50)` is the number
                // that the group holds, negated.
                if self.dialect.the_number_keeps_the_text_that_was_written()
                    && matches!(op, UnaryOp::Negate | UnaryOp::Plus)
                    && !self.the_operand_is_a_group(at_the_operand)
                {
                    if let Node::Literal { value: LiteralValue::Number(text), line } = operand {
                        let text = match op {
                            UnaryOp::Negate => the_text_of_the_negated_number(&text),
                            _ => text,
                        };
                        return Ok(Node::Literal { value: LiteralValue::Number(text), line });
                    }
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

    /// Whether the operand that begins at the place is a group.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// sign of the v1 is part of the number that was written, and a group
    /// keeps the sign out of it: the `-(1.50)` is the number that the
    /// group holds, negated, and not the number that was written.
    fn the_operand_is_a_group(&self, at: usize) -> bool {
        let mut index = at;
        while self
            .tokens
            .get(index)
            .is_some_and(|token| token.kind == TokenKind::Newline)
        {
            index += 1;
        }
        self.tokens
            .get(index)
            .is_some_and(|token| token.kind == TokenKind::Op && token.value == "(")
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
                let value = the_literal_of_a_number(&token.value, self.dialect);
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
                    return self.parse_suffixes(call, true, false);
                } else {
                    // The words that are values are the two below.
                    match token.value.to_lowercase().as_str() {
                        "true" => Node::Literal { value: LiteralValue::Bool(true), line: token.line },
                        "false" => Node::Literal { value: LiteralValue::Bool(false), line: token.line },
                        // The `Class` is a word of the language and a class
                        // type of the dialect: a script reads it as the
                        // class.
                        "class" => Node::Var { name: token.value.clone(), line: token.line },
                        _ => {
                            // Measured on the binary of v2.0.19 on 2026-11-01:
                            // the `ExitApp` is a function of the machine and
                            // not a word of the dialect, and a function is a
                            // value.
                            if crate::builtins::find(&token.value.to_lowercase()).is_some() {
                                Node::Var { name: token.value.clone(), line: token.line }
                            } else if let Some(refusal) =
                                crate::lexer::the_refusal_of_a_word(&token.value)
                            {
                                // The reason is the one of the word, and it
                                // is the same in every place.
                                return Err(self.the_error_of_the_refusal(refusal, &token));
                            } else {
                                return Err(SyntaxError {
                                    message: format!("{:?} is not a value", token.value),
                                    line: token.line,
                                    column: token.column,
                                });
                            }
                        }
                    }
                }
            }
            TokenKind::Name => {
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // `new` of the v1 makes an instance of a class of the
                // script, and the name of the class follows it. The word is
                // not reserved: `new := 5` is a variable of the script, and
                // so is a `new` that no name follows.
                if self.dialect.the_new_makes_an_instance()
                    && token.value.eq_ignore_ascii_case("new")
                    && self.a_class_name_may_come_next()
                {
                    return self.parse_new(token.line);
                }
                // The parenthesis that is spaced is not a call: the
                // concatenation of v2 takes the name and the value that
                // follows it, and the space is what tells them apart.
                if self.at_op(&["("]) && !self.peek().spaced {
                    // The answer of a call is a value, and the members of
                    // it are read the way the members of any other value
                    // are: `StrLen("abc").__Class`.
                    let call = self.parse_call(token.value.to_lowercase(), token.line)?;
                    return self.parse_suffixes(call, true, false);
                }
                let name = token.value;
                // Measured on the binary of v1.1.37.02 on 2026-11-02: the
                // `base` of the v1 is the base of the class where the method
                // was defined, and the word is only that word inside a
                // class and when a dot or a bracket follows it.
                if self.dialect.the_base_is_the_base_of_the_class()
                    && self.in_class
                    && name.eq_ignore_ascii_case("base")
                    && (self.at_op(&["."]) || self.at_op(&["["]))
                {
                    return self.parse_suffixes(Node::Super { line: token.line }, true, false);
                }
                // The name of a variable that the script builds: `MyArray%i%`
                // is the variable whose name is the text and the value of
                // `i`, and the space is what tells it apart from the
                // variable `MyArray` and the value of `i`. The percent sign
                // that closes a name that is being built is not one of them.
                if self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
                    let at_the_name = self.position - 1;
                    let pieces = self.the_pieces_of_a_dynamic_name(name, at_the_name, token.line)?;
                    return Ok(Node::DynamicName { pieces, line: token.line });
                }
                // The arrow function of v2 with one parameter: `a => a + 1`.
                if self.at_op(&["=>"]) {
                    self.position += 1;
                    let body = self.parse_expression()?;
                    let parameters = vec![Parameter { name: name.clone(), default: None, variadic: false, by_ref: false }];
                    return Ok(Node::Lambda { parameters, body: Box::new(body), line: token.line });
                }
                // `x++` and `x--`: the operator comes after the name, and
                // the space is what tells it apart from the concat -- unless
                // the dialect does not separate them.
                if self.at_op(&["++", "--"])
                    && (!self.dialect.the_space_separates_the_increment() || !self.peek().spaced)
                {
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
            TokenKind::Op if token.value == "%" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the double
                // deref of v2 is a name that the script builds -- the value of
                // the expression is the name of the variable whose value is
                // the answer -- and the hash that does not close it is
                // refused with the text of the rest of the line.
                let open = self.position - 1;
                if self.at_op(&["%"]) {
                    return Err(self.the_error_of_a_percent_inside());
                }
                let value = self.the_expression_of_a_dynamic_name(open)?;
                if !self.at_op(&["%"]) {
                    return Err(self.the_error_of_a_missing_ending(open));
                }
                self.next();
                let pieces = self.the_pieces_after_the_percent(value)?;
                Node::DynamicName { pieces, line: token.line }
            }
            TokenKind::Op if token.value == "(" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: a
                // parenthesis holds a list of the values, and the value of
                // the list is the last of them: `("a", "b")` is "b". Every
                // item of the list runs.
                //
                // The bracket that is not closed is the failure, and not the
                // operand that is not there inside it.
                let mut items = Vec::new();
                // Measured on the binary of v2.0.19 on 2026-10-31: the list
                // of nothing is a "Syntax error.", and the parenthesis that
                // closes it is named. The value that is not there at the
                // front of the list is a "Syntax error." too, and the comma
                // that names it is named.
                if self.at_op(&[")"]) || self.at_op(&[","]) {
                    return Err(self.error(format!(
                        "Syntax error.\n     Specifically: {}",
                        the_source_of_the_rest_of_the_line(self, self.position)
                    )));
                }
                loop {
                    let item = match self.parse_expression() {
                        Ok(item) => item,
                        Err(error) => return Err(self.the_error_of_the_bracket(error, ")")),
                    };
                    items.push(item);
                    // The group may be written in more than one line.
                    self.skip_newlines();
                    if self.at_op(&[","]) {
                        self.position += 1;
                        // The group may be written in more than one line.
                        self.skip_newlines();
                        // Measured on the binary of v2.0.19 on 2026-10-31: a
                        // comma at the end of the list is a "Syntax error.",
                        // and the parenthesis that closes the list is named.
                        // The value that is not there in the middle of the
                        // list is a "Syntax error." too, and the comma that
                        // follows it is named.
                        if self.at_op(&[")"]) || self.at_op(&[","]) {
                            return Err(self.error(format!(
                                "Syntax error.\n     Specifically: {}",
                                the_source_of_the_rest_of_the_line(self, self.position)
                            )));
                        }
                        continue;
                    }
                    break;
                }
                self.expect_op(")")?;
                if items.len() == 1 {
                    items.pop().expect("the item of the group")
                } else {
                    Node::Group { items, line: token.line }
                }
            }
            TokenKind::Op if token.value == "[" => {
                // The literal of an array: `[a, b, c]`. The brackets of
                // an index are a suffix of a value, and these open one.
                let mut items = Vec::new();
                if !self.at_op(&["]"]) {
                    loop {
                        // Measured on the binary of v2.0.19 on 2026-11-01:
                        // the place that is not there keeps its place, and it
                        // has no value of its own. The place is a comma that
                        // follows a comma, and the literal may have many of
                        // them.
                        if self.at_op(&[","]) {
                            self.position += 1;
                            // The place that is not there and that is the
                            // last one is dropped: the commas at the end of
                            // the list add nothing.
                            if self.the_hole_is_the_last_one("]") {
                                break;
                            }
                            items.push(None);
                            // The list may be written in more than one line.
                            self.skip_newlines();
                            continue;
                        }
                        // A comma at the end of the list is allowed.
                        if self.at_op(&["]"]) {
                            break;
                        }
                        // The bracket that is not closed is the failure, and
                        // not the operand that is not there inside it.
                        let item = match self.parse_expression() {
                            Ok(item) => item,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "]")),
                        };
                        items.push(Some(item));
                        // The list may be written in more than one line.
                        self.skip_newlines();
                        if !self.at_op(&[","]) {
                            break;
                        }
                        self.position += 1;
                        // The list may be written in more than one line.
                        self.skip_newlines();
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
                self.the_object_literal(token.line)?
            }
            _ => {
                // The operand that the operator wants is not there.
                return Err(self.the_error_of_a_missing_operand(&token));
            }
        };
        self.parse_suffixes(node, true, false)
    }

    /// Whether the literal of an object that follows a value is closed.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the properties of
    /// the literal are not read when the brace follows a value: the failure
    /// is the brace, and not them. The braces are counted, and the literal is
    /// closed when the count goes back to zero.
    fn the_object_literal_is_closed(&self) -> bool {
        let mut depth = 1;
        for token in self.tokens.iter().skip(self.position) {
            if token.kind == TokenKind::Op && token.value == "{" {
                depth += 1;
            }
            if token.kind == TokenKind::Op && token.value == "}" {
                depth -= 1;
                if depth == 0 {
                    return true;
                }
            }
            if token.kind == TokenKind::Eof {
                return false;
            }
        }
        false
    }

    /// The failure of the literal of an object that is not closed.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the literal that is
    /// not closed swallows the rest of the script, and the text that is
    /// named is the statement with it. The newline between the lines is a
    /// space.
    fn the_error_of_a_literal_that_is_not_closed(&self, line: u32) -> SyntaxError {
        let end = self
            .tokens
            .iter()
            .rposition(|token| token.kind == TokenKind::Eof)
            .unwrap_or(self.tokens.len());
        let column = self
            .tokens
            .get(self.position.saturating_sub(1))
            .map(|token| token.column)
            .unwrap_or(1);
        SyntaxError {
            message: format!(
                "Missing \"}}\"\n     Specifically: {}",
                the_source_with_the_newlines_as_spaces(self, self.statement_start, end)
            ),
            line,
            column,
        }
    }

    /// The literal of an object: `{a: 1, b: 2}`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the literal may be
    /// written in more than one line: the property may be on the line that
    /// follows the brace that opens it, and the brace that closes it may be
    /// on a line of its own. The properties are separated by commas, and the
    /// newline is not a separator.
    fn the_object_literal(&mut self, line: u32) -> Result<Node, SyntaxError> {
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 looks for
        // the brace that closes the literal before it reads the properties of
        // it, and the literal that is not closed swallows the rest of the
        // script.
        if !self.the_object_literal_is_closed() {
            return Err(self.the_error_of_a_literal_that_is_not_closed(line));
        }
        // Measured on the binary of v2.0.19 on 2026-11-01: the failures of
        // the literal are named with the line of the brace that opens it, and
        // not with the line of the property that failed.
        let open = self
            .tokens
            .get(self.position.saturating_sub(1))
            .map(|token| token.column)
            .unwrap_or(1);
        let mut properties = Vec::new();
        // The property may be on the line that follows the brace.
        self.skip_newlines();
        while !self.at_op(&["}"]) {
            let name = self.next();
            // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
            // property may be built by the script: `{%name%: 1}` is the
            // property whose name is the value of `name`.
            let member = if name.kind == TokenKind::Op && name.value == "%" {
                let open = self.position - 1;
                if self.at_op(&["%"]) {
                    return Err(self.the_error_of_a_percent_inside());
                }
                let value = self.the_expression_of_a_dynamic_name(open)?;
                if !self.at_op(&["%"]) {
                    return Err(self.the_error_of_a_missing_ending(open));
                }
                self.next();
                MemberName::Dynamic(self.the_pieces_after_the_percent(value)?)
            } else {
                // The name of a property may be a whole number: the number
                // with a point is not a name, because the point of it is the
                // point of a member.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: `{5: 1}`
                // is an object with the property "5", and `{1.5: 1}` is
                // refused.
                if name.kind != TokenKind::Name
                    && name.kind != TokenKind::Keyword
                    && !the_number_is_a_whole_one(&name, self.dialect)
                {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // comma that opens the list is the name that is not
                    // there, and the name that is not a name is refused.
                    if name.kind == TokenKind::Op && name.value == "," {
                        return Err(SyntaxError {
                            message: "Missing \"propertyname:\" in object literal.".into(),
                            line,
                            column: open,
                        });
                    }
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // object that is not closed is the failure, and the name
                    // of the property that is not there is not it.
                    if name.kind == TokenKind::Eof {
                        return Err(
                            self.the_error_of_a_missing_bracket("}", name.line, name.column)
                        );
                    }
                    return Err(SyntaxError {
                        message: "Invalid property name in object literal.".into(),
                        line,
                        column: open,
                    });
                }
                MemberName::Text(name.value)
            };
            if !self.at_op(&[":"]) {
                return Err(SyntaxError {
                    message: "Missing \"propertyname:\" in object literal.".into(),
                    line,
                    column: open,
                });
            }
            self.position += 1;
            // The bracket that is not closed is the failure, and not the
            // operand that is not there inside it.
            let value = match self.parse_expression() {
                Ok(value) => value,
                Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
            };
            properties.push((member, value));
            // Measured on the binary of v2.0.19 on 2026-11-01: the
            // properties are separated by commas, and the newline is not a
            // separator: the token that is there instead of the comma is the
            // name of a property that is not there.
            self.skip_newlines();
            if self.at_op(&[","]) {
                self.position += 1;
                // The property that follows the comma may be on the line that
                // follows it.
                self.skip_newlines();
                continue;
            }
            if !self.at_op(&["}"]) && self.peek().kind != TokenKind::Eof {
                return Err(SyntaxError {
                    message: "Invalid property name in object literal.".into(),
                    line,
                    column: open,
                });
            }
            break;
        }
        // The brace that closes the literal may be on a line of its own.
        self.skip_newlines();
        self.expect_op("}")?;
        Ok(Node::ObjectLiteral { properties, line })
    }

    /// The `[...]` and the `as` that may follow a value.
    ///
    /// The assignment is a suffix too, when the value is an expression: the
    /// assignment of v2 is an expression, and `(x := 1)` is a value.
    ///
    /// The command of v2 is a call whose name is at the start of a line, and
    /// the `command` says that the parser is there: a member that is
    /// followed by a space and a parenthesis is a command of v2, and the
    /// parenthesis is one parameter of it.
    fn parse_suffixes(&mut self, target: Node, assignment: bool, command: bool) -> Result<Node, SyntaxError> {
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
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of the member may be built by the script: `m.%x%` is the
                // member whose name is the value of `x`, and `m.%x%()` is
                // the method whose name is the value of `x`.
                if self.at_op(&["%"]) {
                    let open = self.position;
                    self.next();
                    if self.at_op(&["%"]) {
                        return Err(self.the_error_of_a_percent_inside());
                    }
                    let value = self.the_expression_of_a_dynamic_name(open)?;
                    if !self.at_op(&["%"]) {
                        return Err(self.the_error_of_a_missing_ending(open));
                    }
                    self.next();
                    let pieces = self.the_pieces_after_the_percent(value)?;
                    let name = MemberName::Dynamic(pieces);
                    if self.at_op(&["("]) {
                        self.next();
                        let arguments = self.parse_arguments()?;
                        self.expect_op(")")?;
                        node = Node::MethodCall {
                            target: Box::new(node),
                            name,
                            arguments,
                            line: dot.line,
                        };
                        continue;
                    }
                    node = Node::Property { target: Box::new(node), name, line: dot.line };
                    continue;
                }
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
                //
                // Measured on the binary of v2.0.19 on 2026-10-31: the name
                // of the member may be built by the script, and the text
                // that is written is the first piece of it.
                let member = if self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
                    let at_the_name = self.position - 1;
                    MemberName::Dynamic(self.the_pieces_of_a_dynamic_name(name.value, at_the_name, name.line)?)
                } else {
                    MemberName::Text(name.value)
                };
                if self.at_op(&["("]) && !self.peek().spaced {
                    self.next();
                    let arguments = self.parse_arguments()?;
                    self.expect_op(")")?;
                    node = Node::MethodCall {
                        target: Box::new(node),
                        name: member,
                        arguments,
                        line: dot.line,
                    };
                    continue;
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: the
                // parenthesis that is spaced is not the list of the
                // arguments of the call. A member at the start of a line
                // that is followed by a space and a parenthesis is a command
                // of v2, and the parenthesis is one parameter of it; in an
                // expression the parenthesis is the value that follows the
                // member, and the concatenation of v2 takes the two.
                if self.at_op(&["("]) && self.peek().spaced && command {
                    let arguments = self.the_parameters_of_a_command()?;
                    return Ok(Node::MethodCall {
                        target: Box::new(node),
                        name: member,
                        arguments,
                        line: dot.line,
                    });
                }
                node = Node::Property { target: Box::new(node), name: member, line: dot.line };
                continue;
            }
            if self.at_keyword(&["as"]) {
                // Measured on the binary of v2.0.19 on 2026-10-31: v2 has no
                // cast, and the word `as` is a reserved word that is not
                // wanted there. The text that is named is the rest of the
                // line, from the word.
                let at = self.position;
                return Err(self.error(format!(
                    "Unexpected reserved word.\n     Specifically: {}",
                    the_source_of_the_rest_of_the_line(self, at)
                )));
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

    /// Whether the comma that comes after the number is there.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the number that is
    /// at the start of a statement and that is followed by a comma is a call
    /// of v2 that is not written: the name of it is the number, and the
    /// comma is not the space that a call wants.
    fn the_comma_is_after_the_number(&self) -> bool {
        self.tokens
            .get(self.position + 1)
            .map(|token| token.kind == TokenKind::Op && token.value == ",")
            .unwrap_or(false)
    }

    /// Whether the operator of an increment comes after the name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the space is what
    /// tells the operator of the increment apart from the concat: `x++` is
    /// the increment of `x`, and `x ++` is a call of `x`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// has no call without parentheses, and the space does not separate the
    /// operator: the `x ++` is the increment of `x`.
    fn the_postfix_increment_is_next(&self) -> bool {
        let Some(token) = self.tokens.get(self.position + 1) else { return false };
        token.kind == TokenKind::Op
            && matches!(token.value.as_str(), "++" | "--")
            && (!self.dialect.the_space_separates_the_increment() || !token.spaced)
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
            Node::DynamicName { pieces, .. } => {
                Node::AssignDynamic { name: pieces, op, value: Box::new(value), line }
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
                name: MemberName::Text("Call".to_string()),
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
        // The name of the call is the token that comes before the parenthesis,
        // and the text that the refusal of the v1 names is the text of the
        // call, from the name to the parenthesis that closes it.
        let at = self.position - 1;
        self.expect_op("(")?;
        let arguments = self.parse_arguments()?;
        self.expect_op(")")?;
        let end = self.position;
        if self.dialect.refuses_the_call_of_a_function_at_the_load() {
            self.the_call_of_the_v1_is_recorded(&name, arguments.len(), at, end);
        }
        Ok(Node::Call { name, arguments, line })
    }

    /// The `new` of the v1: `new C` and `new C(5)`.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the name that
    /// follows the word is the class, and the parenthesis that may follow
    /// the name of the class carries the arguments of the constructor.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// name of the class may be followed by the name of the class that it
    /// holds: the `new C.D` is an instance of D, and the `new o.D` is an
    /// instance of the class that the instance of C holds, and the
    /// `new this.D` is an instance of the class that the instance of the
    /// method holds.
    fn parse_new(&mut self, line: u32) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut path = vec![token.value.to_lowercase()];
        while self.at_op(&["."]) {
            self.next();
            let name = self.next();
            path.push(name.value.to_lowercase());
        }
        let mut arguments = Vec::new();
        if self.at_op(&["("]) && !self.peek().spaced {
            self.expect_op("(")?;
            arguments = self.parse_arguments()?;
            self.expect_op(")")?;
        }
        Ok(Node::New { path, arguments, line })
    }

    /// The call of the v1 is recorded, and it is checked at the end of the
    /// parse.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the call is
    /// refused at the load, and the text of it is the text of the script.
    fn the_call_of_the_v1_is_recorded(&mut self, name: &str, count: usize, at: usize, end: usize) {
        let source = self.the_verbatim_source(at, end);
        let line = self.tokens.get(at).map(|token| token.line).unwrap_or(0);
        let column = self.tokens.get(at).map(|token| token.column).unwrap_or(0);
        self.calls.push(TheCallOfTheScript {
            name: name.to_lowercase(),
            count,
            source,
            line,
            column,
        });
    }
}

/// The arity of a function of the script: the least number of the arguments
/// and the most.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter that
/// has a default may be left out, and the parameter that takes the rest of
/// the arguments takes none of them.
fn the_arity_of_the_parameters(parameters: &[Parameter]) -> (usize, usize) {
    let required = parameters
        .iter()
        .filter(|parameter| parameter.default.is_none() && !parameter.variadic)
        .count();
    let most = if parameters.iter().any(|parameter| parameter.variadic) {
        usize::MAX
    } else {
        parameters.len()
    };
    (required, most)
}

/// Whether the value is a text that was written as a literal.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the operator that/// wants a number may not follow a literal text, and a variable that holds
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

///
/// Whether the token is a whole number, and it may be the name of a
/// property.
///
/// The numbers of v2 that carry a point are not names: the point of them
/// is the point of a member.
fn the_number_is_a_whole_one(token: &Token, dialect: &'static dyn crate::dialect::Dialect) -> bool {
    token.kind == TokenKind::Number && matches!(the_number_of(&token.value, dialect), LiteralValue::Integer(_))
}

/// The literal of a number, as the dialect writes it.
///
/// The v2 reads the number as a number: the hex is a whole number of the
/// base of sixteen, and a number with a point or with an exponent is a
/// float.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the number of the
/// v1 is a "pure number": it keeps the text that was written, and the
/// literal of it is that text.
fn the_literal_of_a_number(text: &str, dialect: &'static dyn crate::dialect::Dialect) -> LiteralValue {
    if dialect.the_number_keeps_the_text_that_was_written() {
        return LiteralValue::Number(text.to_string());
    }
    the_number_of(text, dialect)
}

/// The text of the number of the v1 that a sign negates.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// whole number gives the text of the number that the sign negates -- the
/// `-00123` is "-123", and the `-0x10` is "-16" -- and the float keeps the
/// text that was written, with the sign in front of it: the `-1.50` is
/// "-1.50".
fn the_text_of_the_negated_number(text: &str) -> String {
    if crate::v1::the_text_is_a_hex(text) {
        // The hex is a whole number, and the one that does not fit is the
        // largest that fits.
        let number = crate::v1::the_whole_number_of_a_hex(text);
        return number.wrapping_neg().to_string();
    }
    if crate::v1::the_text_is_a_float(text) {
        return format!("-{text}");
    }
    // The whole number of base ten: the sign is part of the value, and the
    // value that does not fit is the one that fits.
    let signed = format!("-{text}");
    match signed.parse::<i64>() {
        Ok(number) => number.to_string(),
        Err(_) => i64::MIN.to_string(),
    }
}

/// The number of a token, as v2 reads it.
///
/// The hex of v2 is a number of the base of sixteen, and it wraps the
/// way the machine does: `0xFFFFFFFFFFFFFFFF` is `-1`. A number with a
/// point or with an exponent is a float, and a number without them is a
/// whole number.
///
/// Measured on the binary of v2.0.19 on 2026-10-30.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 keeps the
/// whole numbers in sixty-four bits, and the one that is written and does
/// not fit is the largest of them.
fn the_number_of(text: &str, dialect: &'static dyn crate::dialect::Dialect) -> LiteralValue {
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
        Err(_) if !dialect.a_whole_number_that_does_not_fit_is_the_largest_one() => {
            LiteralValue::Float(text.parse::<f64>().unwrap_or(0.0))
        }
        Err(_) => LiteralValue::Integer(i64::MAX),
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
    fn a_key_in_a_send_is_a_text() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the key of the
        // v2 is a text, and the braces of it are part of the text.
        let statements = parse("Send \"{Enter}\"").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else { panic!("a send") };
        assert!(matches!(&arguments[0], Node::Literal { value: LiteralValue::Text(text), .. } if text == "{Enter}"));
    }

    #[test]
    fn a_brace_that_opens_an_argument_opens_an_object() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the brace that
        // opens an argument of a command opens the literal of an object,
        // and not a key: `Send {Enter}` is refused at the load.
        let error = parse("Send {Enter}").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"propertyname:\" in object literal.");
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
        // Measured on the binary of v2.0.19 on 2026-11-01: the branch of an
        // `if` is written with two words, and the `elseif` is a name like
        // any other.
        let statements = parse("If a {\n} Else If b {\n} Else {\n}").expect("parses");
        let Node::If { branches, otherwise, .. } = &statements[0] else { panic!("an if") };
        assert_eq!(branches.len(), 2);
        assert_eq!(otherwise.len(), 0);
    }

    #[test]
    fn an_elseif_is_a_name_and_not_a_branch() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the `elseif` is a
        // name in v2, and a script may write to it.
        let statements = parse("ElseIf := 1").expect("parses");
        assert!(matches!(&statements[0], Node::Assign { name, .. } if name == "ElseIf"));
    }

    #[test]
    fn a_switch_holds_its_cases() {
        let statements = parse("Switch x {\n  Case 1:\n    MsgBox \"one\"\n  Default:\n    MsgBox \"other\"\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 2);
        assert!(cases[1].values.is_empty());
    }

    #[test]
    fn a_case_may_name_many_values() {
        // Measured on the binary of v2.0.19 on 2026-11-01: a case may name
        // many values, and it matches when the subject of the switch is one
        // of them.
        let statements = parse("Switch x {\n  Case 1, 2, 3:\n    MsgBox \"one\"\n}")
            .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else { panic!("a switch") };
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].values.len(), 3);
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
        assert!(matches!(items[0].as_ref(), Some(Node::Literal { value: LiteralValue::Integer(number), .. }) if *number == 1));
        assert!(matches!(items[2].as_ref(), Some(Node::Literal { value: LiteralValue::Integer(number), .. }) if *number == 3));
    }

    #[test]
    fn an_array_holds_expressions() {
        let statements = parse("x := [a + b, c * d]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0].as_ref(), Some(Node::Binary { op: BinaryOp::Add, .. })));
        assert!(matches!(items[1].as_ref(), Some(Node::Binary { op: BinaryOp::Multiply, .. })));
    }

    #[test]
    fn an_array_may_hold_another_array() {
        let statements = parse("x := [[1], [2]]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0].as_ref(), Some(Node::ArrayLiteral { items, .. }) if items.len() == 1));
    }

    #[test]
    fn an_array_may_hold_a_place_without_a_value() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the literal may
        // have a hole, and the hole is a comma that follows a comma.
        let statements = parse("x := [1, , 3]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 3);
        assert!(items[1].is_none());
    }

    #[test]
    fn an_array_may_hold_many_places_without_a_value() {
        let statements = parse("x := [1, , , 2]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 4);
        assert!(items[1].is_none());
        assert!(items[2].is_none());
    }

    #[test]
    fn the_place_without_a_value_that_is_the_last_one_is_dropped() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the commas at
        // the end of the list add nothing, and the place that has no value
        // of its own and that is the last one is dropped with them.
        let statements = parse("x := [1, 2, , ]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 2);
        let statements = parse("x := [,]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else { panic!("an assignment") };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else { panic!("an array") };
        assert_eq!(items.len(), 0);
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
        //
        // Measured on the binary of v2.0.19 on 2026-11-01: the label that is
        // not there is refused at the load, and the text of the parameter is
        // the name that the refusal carries.
        let error = parse("Goto \"done\"").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Label not found in current scope.\n     Specifically: \"done\""
        );
        let error = parse("Goto(done)").expect_err("does not parse");
        assert_eq!(error.message, "Label not found in current scope.\n     Specifically: done");
    }

    #[test]
    fn the_break_that_has_no_loop_around_it_is_refused() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the `break` and
        // the `continue` need a loop around them, and the script is refused
        // at the load when there is none.
        let error = parse("break").expect_err("does not parse");
        assert_eq!(error.message, "Break/Continue must be enclosed by a Loop.");
        let error = parse("continue").expect_err("does not parse");
        assert_eq!(error.message, "Break/Continue must be enclosed by a Loop.");
        // The body of a function is a scope of its own: the loop that holds
        // the function is not around the `break` of it.
        let error = parse("f() {\n    break\n}").expect_err("does not parse");
        assert_eq!(error.message, "Break/Continue must be enclosed by a Loop.");
    }

    #[test]
    fn the_break_inside_a_loop_is_not_refused() {
        parse("Loop 3 {\n    break\n}").expect("parses");
        parse("x := 0\nwhile (x < 3) {\n    x += 1\n    break\n}").expect("parses");
        parse("for v in [1, 2] {\n    break\n}").expect("parses");
        parse("Loop 3 {\n    if (1) {\n        break\n    }\n}").expect("parses");
    }

    #[test]
    fn the_question_that_has_no_colon_says_so() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the question
        // that has no colon says so, and the text that is named begins at
        // the question mark.
        let error = parse("x := 1 ? 2").expect_err("does not parse");
        assert_eq!(
            error.message,
            "A \"?\" is missing its \":\"\n     Specifically: ? 2"
        );
    }

    #[test]
    fn the_colon_that_is_left_over_says_so() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the colon that is
        // left over after an expression is the colon of a question that is
        // not there.
        let error = parse("x := 1 : 2").expect_err("does not parse");
        assert_eq!(error.message, "A \":\" is missing its \"?\"");
    }

    #[test]
    fn the_definition_of_the_fat_arrow_that_has_no_body_is_refused() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the definition
        // that has no body says so, and the text that is named is the
        // definition.
        let error = parse("f(a) =>").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Invalid function declaration.\n     Specifically: f(a) =>"
        );
    }

    #[test]
    fn the_reserved_word_that_may_not_be_the_name_of_a_variable_says_so() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the word `until`
        // is reserved and may not be the name of a variable; the words that
        // are not operators are a "Syntax error."; and the words that are
        // reserved for a future of v2 say "Unexpected reserved word.". The
        // text that is named is the word alone.
        let error = parse("until := 1").expect_err("does not parse");
        assert_eq!(
            error.message,
            "The following reserved word must not be used as a variable name:\n\"until\""
        );
        let error = parse("not := 1").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: not");
        let error = parse("as := 1").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Unexpected reserved word.\n     Specifically: as"
        );
    }

    #[test]
    fn the_bare_until_wants_a_condition() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the bare `until`
        // wants a condition, and it says so in its own words.
        let error = parse("until").expect_err("does not parse");
        assert_eq!(
            error.message,
            "\"Until\" requires at least 1 parameter.\n     Specifically: until"
        );
        // The binary crashes when the `until` has a condition, and the tree
        // refuses it there.
        let error = parse("until (1)").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"Until\"");
    }

    #[test]
    fn the_reserved_word_that_is_not_wanted_names_itself() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the words that
        // may not be the name of a variable say so.
        let error = parse("x := 1 else").expect_err("does not parse");
        assert_eq!(
            error.message,
            "The following reserved word must not be used as a variable name:\n\"else\""
        );
    }

    #[test]
    fn the_reserved_word_of_the_future_of_v2_is_named() {
        // Measured on the binary of v2.0.19 on 2026-10-31: `in` and
        // `contains` are reserved for a future of v2, and the text that is
        // named is the rest of the line, from the word.
        let error = parse("x := 1 in").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected reserved word.\n     Specifically: in");
    }

    #[test]
    fn the_word_that_is_not_an_operator_is_a_syntax_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: `not` and `super`
        // are not operators, and the text that is named is the rest of the
        // line, from the word.
        let error = parse("x := 1 not").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: not");
    }

    #[test]
    fn v2_has_no_cast() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the word `as` is
        // a reserved word, and v2 has no cast.
        let error = parse("x := 1 as Integer").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected reserved word.\n     Specifically: as Integer");
    }

    #[test]
    fn the_parenthesis_holds_a_list_of_the_values() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a parenthesis
        // holds a list of the values, and the value of the list is the last
        // of them.
        let statements = parse("x := (\"a\", \"b\")").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment");
        };
        assert!(matches!(&**value, Node::Group { items, .. } if items.len() == 2));
        // The list of one item is the value of that item.
        let statements = parse("x := (\"a\")").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment");
        };
        assert!(matches!(&**value, Node::Literal { .. }));
    }

    #[test]
    fn the_group_of_nothing_is_a_syntax_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the list of
        // nothing is a "Syntax error.", and the parenthesis that closes it
        // is named.
        let error = parse("x := ()").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: )");
    }

    #[test]
    fn the_comma_at_the_end_of_a_list_is_a_syntax_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: a comma at the
        // end of the list is a "Syntax error.", and the parenthesis that
        // closes the list is named.
        let error = parse("x := (\"a\",)").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: )");
    }

    #[test]
    fn the_name_with_a_space_and_a_parenthesis_is_a_command() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the parenthesis
        // that is spaced is not the list of the arguments of a call, and the
        // group is one parameter of the command.
        let statements = parse("MyFunc (\"a\", \"b\")").expect("parses");
        let Node::Call { name, arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(name, "myfunc");
        assert_eq!(arguments.len(), 1);
        assert!(matches!(&arguments[0], Node::Group { items, .. } if items.len() == 2));
        // The call that is written without the space keeps its arguments.
        let statements = parse("MyFunc(\"a\", \"b\")").expect("parses");
        let Node::Call { name, arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(name, "myfunc");
        assert_eq!(arguments.len(), 2);
    }

    #[test]
    fn the_parameter_that_is_not_there_keeps_its_place() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the parameter
        // that is not there keeps its place, and it has no value.
        let statements = parse("MyFunc \"a\",, \"b\"").expect("parses");
        let Node::Call { name, arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(name, "myfunc");
        assert_eq!(arguments.len(), 3);
        assert!(matches!(&arguments[1], Node::Empty { .. }));
        // The commas at the end of the parameters add nothing.
        let statements = parse("MyFunc \"a\",,").expect("parses");
        let Node::Call { name, arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(name, "myfunc");
        assert_eq!(arguments.len(), 1);
    }

    #[test]
    fn the_value_that_is_not_there_in_a_group_is_a_syntax_error() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the value that
        // is not there in the list of a group is a "Syntax error.", and the
        // comma that follows it is named.
        let error = parse("x := (\"a\",, \"c\")").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: , \"c\")");
        let error = parse("x := (,\"c\")").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: ,\"c\")");
    }

    #[test]
    fn the_comma_ends_the_expression_of_a_statement() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the expression of
        // a statement ends at the comma, and the rest of the line is the
        // list of v1.
        let statements = parse("x := 1, 2").expect("parses");
        assert_eq!(statements.len(), 2);
        assert!(matches!(&statements[0], Node::Assign { .. }));
        let Node::Legacy { items, .. } = &statements[1] else {
            panic!("the list of v1");
        };
        assert_eq!(items.len(), 1);
        // The call of a statement is one too.
        let statements = parse("f(), 2").expect("parses");
        assert_eq!(statements.len(), 2);
        assert!(matches!(&statements[0], Node::Call { .. }));
        let Node::Legacy { items, .. } = &statements[1] else {
            panic!("the list of v1");
        };
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn the_expression_of_a_statement_goes_on() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the expression of
        // a statement begins with the call, and the operators that follow
        // it are a part of it.
        let statements = parse("f() + 1").expect("parses");
        let Node::Expression { value, .. } = &statements[0] else {
            panic!("an expression");
        };
        assert!(matches!(&**value, Node::Binary { op: BinaryOp::Add, .. }));
    }

    #[test]
    fn the_name_with_a_comma_is_a_call_that_is_not_written() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
        // command of v2 is followed by a space or a parenthesis, and the
        // comma that is there instead of the space is refused.
        let error = parse("x, 2").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Function calls require a space or \"(\". Use comma only between parameters.\n     Specifically: x, 2"
        );
    }

    #[test]
    fn the_number_that_is_at_the_start_of_a_statement_is_a_call() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the number that is
        // at the start of a statement and that is followed by a comma is a
        // call of v2 that is not written.
        let error = parse("1, 2").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Function calls require a space or \"(\". Use comma only between parameters.\n     Specifically: 1, 2"
        );
        // The number that carries a point is not a name, and the line holds
        // no action.
        let error = parse("1.5, 2").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: 1.5, 2"
        );
    }

    #[test]
    fn the_member_that_has_parentheses_is_an_expression() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the member that
        // has parentheses is an expression, and the comma ends it.
        let statements = parse("t.Method(), 2").expect("parses");
        assert_eq!(statements.len(), 2);
        assert!(matches!(&statements[0], Node::MethodCall { .. }));
        let Node::Legacy { items, .. } = &statements[1] else {
            panic!("the list of v1");
        };
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn the_member_that_has_no_parentheses_is_a_command() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the member that
        // has no parentheses is a command of v2, and the rest of the line is
        // the list of its arguments.
        let statements = parse("t.Method 2").expect("parses");
        let Node::MethodCall { arguments, .. } = &statements[0] else {
            panic!("a method");
        };
        assert_eq!(arguments.len(), 1);
        // The comma that comes right after the member with no space is not
        // one of the arguments, and the line holds no action.
        let error = parse("t.Method, 2").expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: t.Method, 2"
        );
    }

    #[test]
    fn the_line_that_ends_with_a_comma_continues_on_the_next_one() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the line that
        // ends with a comma continues on the next one, and the list of v1
        // goes on with it.
        let statements = parse("x := 1,\n2").expect("parses");
        assert_eq!(statements.len(), 2);
        assert!(matches!(&statements[0], Node::Assign { .. }));
        let Node::Legacy { items, .. } = &statements[1] else {
            panic!("the list of v1");
        };
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn the_parameter_that_is_not_there_in_a_call_keeps_its_place() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the parameter
        // that is not there keeps its place in the list of the arguments of
        // a call, and it has no value.
        let statements = parse("MyFunc(\"a\",, \"b\")").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(arguments.len(), 3);
        assert!(matches!(&arguments[1], Node::Empty { .. }));
        // The one that is the last is dropped: the commas at the end of the
        // list add nothing.
        let statements = parse("MyFunc(\"a\",)").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(arguments.len(), 1);
        let statements = parse("MyFunc(,)").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else {
            panic!("a call");
        };
        assert_eq!(arguments.len(), 0);
    }

    #[test]
    fn the_object_literal_may_be_written_in_more_than_one_line() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the literal of an
        // object may be written in more than one line: the property may be
        // on the line that follows the brace that opens it, and the brace
        // that closes it may be on a line of its own.
        parse("x := {\n}").expect("parses");
        parse("x := {a: 1\n}").expect("parses");
        parse("x := {\na: 1}").expect("parses");
        parse("x := {a: 1,\nb: 2}").expect("parses");
    }

    #[test]
    fn the_properties_of_an_object_are_separated_by_commas() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the newline is
        // not a separator between the properties of an object, and the token
        // that is there instead of the comma is the name of a property that
        // is not there.
        let error = parse("x := {a: 1 b: 2}").expect_err("does not parse");
        assert_eq!(error.message, "Invalid property name in object literal.");
        let error = parse("x := {a: 1\nb: 2}").expect_err("does not parse");
        assert_eq!(error.message, "Invalid property name in object literal.");
    }

    #[test]
    fn the_name_of_the_property_that_is_not_there() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the comma that
        // opens the list of the properties is the name that is not there,
        // and the name that is not a name is refused.
        let error = parse("x := {, a: 1}").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"propertyname:\" in object literal.");
        let error = parse("x := {1.5: 1}").expect_err("does not parse");
        assert_eq!(error.message, "Invalid property name in object literal.");
    }

    #[test]
    fn the_brace_that_follows_a_value_opens_the_literal_of_an_object() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
        // follows a value opens the literal of an object, and not a block:
        // the literal that is not closed is the failure.
        let error = parse("x := 1 {").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"}\"\n     Specifically: x := 1 {");
        let error = parse("Two (a, b) {").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"}\"\n     Specifically: Two (a, b) {");
    }

    #[test]
    fn the_brace_that_follows_a_value_and_that_is_closed_is_unexpected() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the literal that
        // is closed is a second value in a row, and the brace is the one that
        // is not wanted. The properties inside it are not read: the brace is
        // the failure, and not them.
        let error = parse("x := 1 {}").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"{\"");
        let error = parse("x := 1 { a: 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"{\"");
    }

    #[test]
    fn the_character_that_is_illegal_says_what_it_is() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
        // semicolon that are glued to a token are illegal characters in an
        // expression, and the text that is named begins at them.
        let error = parse("x := 1#c").expect_err("does not parse");
        assert_eq!(error.message, "Illegal character in expression.\n     Specifically: #c");
        let error = parse("x := 1;c").expect_err("does not parse");
        assert_eq!(error.message, "Illegal character in expression.\n     Specifically: ;c");
        // The one that is inside a parenthesis names the text of the
        // expression that holds it.
        let error = parse("if (x#c) {").expect_err("does not parse");
        assert_eq!(error.message, "Illegal character in expression.\n     Specifically: #c)");
        // The one that begins a statement is a line that holds no action.
        let error = parse("f() {\n}#c").expect_err("does not parse");
        assert_eq!(error.message, "This line does not contain a recognized action.\n     Specifically: #c");
        // The one that is glued to the name that opens the statement is a
        // part of it, and the whole line is named.
        let error = parse("x#y := 1").expect_err("does not parse");
        assert_eq!(error.message, "This line does not contain a recognized action.\n     Specifically: x#y := 1");
    }

    #[test]
    fn the_brace_that_opens_a_line_opens_a_block() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
        // opens a line is a block of its own, and the statements between the
        // braces run. The content may be on the line of the brace that opens
        // it, and the brace that closes it is alone in its line -- unless the
        // block is empty, and then the brace may be written where it is.
        let statements = parse("{\n}").expect("parses");
        let Node::Block { body, .. } = &statements[0] else { panic!("a block") };
        assert!(body.is_empty());
        let statements = parse("{\nx := 1\n}").expect("parses");
        let Node::Block { body, .. } = &statements[0] else { panic!("a block") };
        assert_eq!(body.len(), 1);
        // The empty block may be written in one line.
        parse("{}").expect("parses");
        parse("{ }").expect("parses");
        // The content may be on the line of the brace that opens it.
        let statements = parse("{x := 1\n}").expect("parses");
        let Node::Block { body, .. } = &statements[0] else { panic!("a block") };
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn the_brace_that_closes_the_block_is_alone_in_its_line() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
        // closes the block is alone in its line, and the v2 says so before it
        // reads the content of the block. The text that is named is the line
        // of the brace.
        let error = parse("{x := 1}").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"}\"\n     Specifically: x := 1}");
        let error = parse("{\nx := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"}\"\n     Specifically: x := 1 }");
        // The block that is not closed is named, and the line that is named
        // is the line of the brace that opens it.
        let error = parse("{\nx := 1").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"}\"");
        assert_eq!(error.line, 1);
    }

    #[test]
    fn the_literal_of_an_object_is_closed_before_it_is_read() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 looks for
        // the brace that closes the literal before it reads the properties
        // of it, and the literal that is not closed swallows the rest of the
        // script.
        let error = parse("x := {\na: 1").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"}\"\n     Specifically: x := { a: 1");
        assert_eq!(error.line, 1);
        // The property that is not there does not hide the brace that is
        // not there.
        let error = parse("Send {\nEnter").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"}\"\n     Specifically: Send { Enter");
        // The failures of the literal are named with the line of the brace
        // that opens it, and not with the line of the property that failed.
        let error = parse("x := {\na\n}").expect_err("does not parse");
        assert_eq!(error.message, "Missing \"propertyname:\" in object literal.");
        assert_eq!(error.line, 1);
    }

    #[test]
    fn the_or_maybe_wants_a_variable_on_its_left() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 refuses
        // the script at the load when the left side of the or-maybe is
        // another value, and it names the operator.
        let error = parse("x := 1 ?? 2").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: ?? 2");
        let error = parse("x := o.a ?? 2").expect_err("does not parse");
        assert_eq!(error.message, "Syntax error.\n     Specifically: ?? 2");
        // The variable is the left side that the v2 accepts.
        parse("x := y ?? 2").expect("parses");
    }

    #[test]
    fn the_three_way_comparison_is_refused_with_the_text_of_the_signs() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the three signs
        // are the `==` and the `=`, and the operand that the comparison
        // wants is not there. The text that is named begins at the operator.
        let error = parse("x := 1 === 1").expect_err("does not parse");
        assert_eq!(error.message, "Missing operand.\n     Specifically: === 1");
    }
