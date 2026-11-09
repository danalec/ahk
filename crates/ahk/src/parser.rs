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
    AccessBody, AccessDef, AssignOp, BinaryOp, Branch, Case, Catch, Declarations, DeclareKind,
    Declared, DynamicPiece, LegacyForm, LiteralValue, LoopKind, MemberName, MethodDef, Node,
    ObjectItem, Parameter, PropertyDef, UnaryOp,
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
fn the_shape_of_a_super(
    parser: &Parser,
    start: usize,
    as_a_statement: bool,
) -> (TheShapeOfASuper, String) {
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
                && name
                    .map(|token| token.kind == TokenKind::Name || token.kind == TokenKind::Keyword)
                    .unwrap_or(false);
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
        text.push_str(&the_source_of_a_token(parser, token));
    }
    text
}

/// The source of a token, the way the script wrote it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the text of a token
/// keeps the quote that was written, and the name of a token that is
/// refused is the text of it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the name
/// that carries an escape is the name that the escape made, and the source
/// of it is the text that the script wrote: the `a``b` is the source "a``b".
fn the_source_of_a_token(parser: &Parser, token: &Token) -> String {
    if token.kind == TokenKind::Name && token.value.contains('`') {
        return the_source_of_the_name_that_carries_an_escape(parser, token);
    }
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

/// The tokens of the operand that holds a percent sign.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
/// operand of the v1 is the text that goes from its first character to the
/// first character that ends it -- the space, the tab, an operator or a
/// bracket -- and the point does not end it: the `a.b%c` is one operand.
///
/// The text of the operand is the text that the complaint of the v1 names,
/// and not the rest of the line: the `10 % 3` is refused with the operand "%"
/// and with the text "% 3" that the v2 names.
fn the_operand_of_the_percent_sign(parser: &Parser, at: usize) -> (usize, usize) {
    let mut start = at;
    while start > 0
        && the_token_is_of_an_operand(&parser.tokens[start - 1])
        && !parser.tokens[start].spaced
        && !the_text_ends_the_operand(parser, start - 1)
    {
        start -= 1;
    }
    let mut end = at;
    while end + 1 < parser.tokens.len()
        && the_token_is_of_an_operand(&parser.tokens[end + 1])
        && !parser.tokens[end + 1].spaced
    {
        end += 1;
    }
    (start, end + 1)
}

/// The source of the operand that holds a percent sign.
fn the_source_of_the_operand(parser: &Parser, at: usize) -> String {
    let (start, end) = the_operand_of_the_percent_sign(parser, at);
    the_source_of_the_tokens(parser, start, end)
}

/// Whether the percent sign that closes a name is in the operand of the one
/// that opens it.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the name
/// that the percent sign opens is closed by a percent sign of the same
/// operand: the `x := %a.b % c` is refused with the operand "%a.b", and the
/// space is what ends the operand.
fn the_closing_is_in_the_operand(parser: &Parser, open: usize, close: usize) -> bool {
    let (start, end) = the_operand_of_the_percent_sign(parser, open);
    start <= close && close < end
}

/// Whether the token is a part of the operand of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
/// operand is made of the names, of the numbers, of the words and of the
/// texts, and the percent sign and the point are a part of it: the `a.b%c`
/// is one operand.
fn the_token_is_of_an_operand(token: &Token) -> bool {
    match token.kind {
        TokenKind::Name | TokenKind::Number | TokenKind::Keyword | TokenKind::String => true,
        TokenKind::Op => matches!(token.value.as_str(), "%" | "."),
        _ => false,
    }
}

/// Whether the text ends the operand of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the text
/// that begins an operand is not one: the v1 goes past it, and the operand
/// begins after it. The `"a"b%c` is refused with the operand "b%c".
///
/// The text that is inside an operand is a part of it: the `a."b"%c` is one
/// operand, and the point of it is the point of the member.
fn the_text_ends_the_operand(parser: &Parser, at: usize) -> bool {
    if parser.tokens[at].kind != TokenKind::String {
        return false;
    }
    if parser.tokens[at].spaced {
        return true;
    }
    match at
        .checked_sub(1)
        .and_then(|before| parser.tokens.get(before))
    {
        None => true,
        Some(token) => !the_token_is_of_an_operand(token),
    }
}

/// The source of the name that carries an escape.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the source
/// of the name is the text of the script, and not the name that the escape
/// made: the `a``b` is the source "a``b".
fn the_source_of_the_name_that_carries_an_escape(parser: &Parser, token: &Token) -> String {
    let Some(line) = parser.lines.get(token.line as usize - 1) else {
        return token.value.clone();
    };
    let characters: Vec<char> = line.chars().collect();
    let start = (token.column as usize).saturating_sub(1);
    let mut end = start;
    while end < characters.len()
        && !crate::lexer::the_character_ends_the_symbol_of_the_v1(characters[end])
    {
        end += 1;
    }
    characters[start..end].iter().collect()
}

/// Whether the value may be written to.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the increment of a
/// value that cannot be written is refused at the load, and it is an
/// "Invalid assignment.".
fn the_value_can_be_written(node: &Node) -> bool {
    matches!(
        node,
        Node::Var { .. } | Node::Property { .. } | Node::Index { .. } | Node::DynamicName { .. }
    )
}

/// Whether the increment of this value is refused at the load.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
/// does not refuse the increment of a literal: the `x := ++5` is a script
/// that runs, and the value of the expression is the empty text. The v2
/// refuses it at the load.
fn the_increment_of_the_value_is_refused(
    dialect: &dyn crate::dialect::Dialect,
    node: &Node,
) -> bool {
    if dialect.the_increment_of_a_literal_is_blank() && matches!(node, Node::Literal { .. }) {
        return false;
    }
    !the_value_can_be_written(node)
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
    if parser
        .tokens
        .get(body)
        .map(|token| token.spaced)
        .unwrap_or(false)
    {
        text.push(' ');
    }
    text
}

/// Whether the kind is one of the loops of v2.
///
/// The loop of a count and the loops of a kind are the same statement,
/// and the name of the kind is part of it.
fn the_loop_of_the_kind(kind: &str) -> bool {
    matches!(
        kind,
        "Loop" | "Loop Files" | "Loop Reg" | "Loop Read" | "Loop Parse"
    )
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
        && !error
            .message
            .starts_with("This line does not contain a recognized action.")
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
        text.push_str(&the_source_of_a_token(parser, token));
    }
    text
}

/// The source of a line of the script.
///
/// v2 shows the line when it refuses a name or a bracket, and the line is
/// the one that the token is on.
fn the_source_of_the_line(parser: &Parser, line: u32) -> String {
    let start = parser
        .tokens
        .iter()
        .position(|token| token.line == line)
        .unwrap_or(0);
    let mut end = start;
    while end < parser.tokens.len() && parser.tokens[end].line == line {
        end += 1;
    }
    the_source_of_the_tokens(parser, start, end)
        .trim_end()
        .to_string()
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
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    // names the symbol that begins at the character: the `;c` of the
    // `x := 1;c`, and the `;c.` of the `x := 1;c.`.
    if in_an_expression && parser.dialect.the_illegal_character_names_the_symbol() {
        return the_symbol_that_begins_at(parser, at);
    }
    let end = if in_an_expression {
        the_end_of_the_expression_of_the_illegal_character(parser, at)
    } else {
        the_end_of_the_line_of(parser, at)
    };
    the_source_with_the_newlines_as_spaces(parser, at, end)
}

/// The symbol that begins at the place.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// symbol of the v1 ends at the space and at the punctuation: the `;c` of
/// the `x := 1;c` is the symbol, and the `;` of the `x := 1;(c)` is one.
/// The point, the semicolon and the percent sign are a part of it.
fn the_symbol_that_begins_at(parser: &Parser, at: usize) -> String {
    let Some(token) = parser.tokens.get(at) else {
        return String::new();
    };
    let Some(line) = parser.lines.get(token.line as usize - 1) else {
        return token.value.clone();
    };
    let characters: Vec<char> = line.chars().collect();
    let start = (token.column as usize).saturating_sub(1);
    if start >= characters.len() {
        return token.value.clone();
    }
    let mut end = start;
    while end < characters.len()
        && !crate::lexer::the_character_ends_the_symbol_of_the_v1(characters[end])
    {
        end += 1;
    }
    characters[start..end].iter().collect()
}

/// The symbol that begins at the backtick that the escape carries.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// backtick that the escape carries is an illegal character in an
/// expression, and the symbol that the complaint of the v1 names begins at
/// it: the `x := a``b` is the symbol "`b", and the `x := a``b.c` is the
/// symbol "`b.c".
fn the_symbol_of_the_escaped_backtick(parser: &Parser, at: usize) -> String {
    let Some(token) = parser.tokens.get(at) else {
        return String::new();
    };
    let Some(line) = parser.lines.get(token.line as usize - 1) else {
        return token.value.clone();
    };
    let characters: Vec<char> = line.chars().collect();
    let start = (token.column as usize).saturating_sub(1);
    // The first backtick of the word opens the escape, and the second one is
    // the character that the escape carries.
    let mut backticks = 0;
    let mut at_the_backtick = start;
    while at_the_backtick < characters.len() {
        if characters[at_the_backtick] == '`' {
            backticks += 1;
            if backticks == 2 {
                break;
            }
        }
        at_the_backtick += 1;
    }
    if backticks < 2 {
        return token.value.clone();
    }
    let mut end = at_the_backtick;
    while end < characters.len()
        && !crate::lexer::the_character_ends_the_symbol_of_the_v1(characters[end])
    {
        end += 1;
    }
    characters[at_the_backtick..end].iter().collect()
}

/// Whether the symbol begins with the quote that the dialect does not read.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// single quote is not a text in the v1, and the symbol that begins with it
/// is an illegal character in an expression.
fn the_symbol_begins_with_the_illegal_quote(token: &Token) -> bool {
    token.kind == TokenKind::Name && token.value.starts_with('\'')
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

/// Whether the token is a bracket.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
/// names the bracket that is not wanted and nothing else: the `x := )` is an
/// "Unexpected ")" and the line of the script is not under it.
fn the_token_is_a_bracket(token: &Token) -> bool {
    token.kind == TokenKind::Op && matches!(token.value.as_str(), "(" | ")" | "[" | "]" | "{" | "}")
}

/// Whether the token is the operator of two values.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a line that begins
/// with one of them continues the line before it.
fn the_token_is_an_operator(token: &Token) -> bool {
    if token.kind == TokenKind::Op {
        return matches!(
            token.value.as_str(),
            "**" | "*"
                | "/"
                | "//"
                | "+"
                | "-"
                | "<<"
                | ">>"
                | ">>>"
                | "&"
                | "^"
                | "|"
                | "~="
                | ">"
                | ">="
                | "<"
                | "<="
                | "="
                | "=="
                | "!="
                | "!=="
                | "&&"
                | "||"
                | "??"
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
            Node::If {
                branches,
                otherwise,
                ..
            } => {
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
            Node::Try {
                body,
                catches,
                otherwise,
                finally,
                ..
            } => {
                collect_the_labels_of_a_scope(body, labels);
                for catch in catches {
                    collect_the_labels_of_a_scope(&catch.body, labels);
                }
                collect_the_labels_of_a_scope(otherwise, labels);
                collect_the_labels_of_a_scope(finally, labels);
            }
            // The body of a hotkey of the v1 is of the scope that carries it.
            Node::HotkeyDefinition { body, .. } => collect_the_labels_of_a_scope(body, labels),
            _ => {}
        }
    }
}

/// A block of the script, and the function that it is the body of.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the label of a block
/// is not the target of a jump that the block does not enclose, and the body
/// of a function is a block of its own.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TheBlock {
    /// The id of the block: it tells it from the others.
    pub(crate) id: u32,
    /// Whether the block is the body of a function.
    pub(crate) function: bool,
}

/// The labels of the script and the gotos of the script, with the chain of
/// the blocks that enclose each of them.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the chain of the
/// blocks tells the block of a label, and the jump may not enter a block that
/// does not enclose it, and the jump of a function may not leave the
/// function.
struct TheNamesOfTheScript {
    /// The labels of the script, with the chain of the blocks of each.
    labels: Vec<(String, Vec<TheBlock>)>,
    /// The gotos of the script, with the chain of the blocks of each.
    gotos: Vec<(String, u32, Vec<TheBlock>)>,
    /// The id of the next block.
    next: u32,
}

impl TheNamesOfTheScript {
    /// The names of a scope of the script.
    fn collect(&mut self, body: &[Node], chain: &mut Vec<TheBlock>) {
        for statement in body {
            match statement {
                Node::Label { name, .. } => self.labels.push((name.clone(), chain.clone())),
                Node::Goto { name, line } => self.gotos.push((name.clone(), *line, chain.clone())),
                Node::If {
                    branches,
                    otherwise,
                    ..
                } => {
                    for branch in branches {
                        self.collect_in_a_block(&branch.body, chain, false);
                    }
                    self.collect_in_a_block(otherwise, chain, false);
                }
                Node::Switch { cases, .. } => {
                    for case in cases {
                        self.collect_in_a_block(&case.body, chain, false);
                    }
                }
                Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
                    self.collect_in_a_block(body, chain, false);
                }
                Node::Try {
                    body,
                    catches,
                    otherwise,
                    finally,
                    ..
                } => {
                    self.collect_in_a_block(body, chain, false);
                    for catch in catches {
                        self.collect_in_a_block(&catch.body, chain, false);
                    }
                    self.collect_in_a_block(otherwise, chain, false);
                    self.collect_in_a_block(finally, chain, false);
                }
                Node::Block { body, .. } => self.collect_in_a_block(body, chain, false),
                // The body of a hotkey of the v1 is of the scope that carries
                // it: the hotkey is not a block, and a jump may leave it.
                Node::HotkeyDefinition { body, .. } => self.collect(body, chain),
                Node::LegacyIf {
                    body, otherwise, ..
                } => {
                    self.collect_in_a_block(body, chain, false);
                    self.collect_in_a_block(otherwise, chain, false);
                }
                Node::FunctionDef { body, .. } => self.collect_in_a_block(body, chain, true),
                Node::ClassDef { methods, .. } => {
                    for method in methods {
                        self.collect_in_a_block(&method.body, chain, true);
                    }
                }
                _ => {}
            }
        }
    }

    /// The names of a block of the script.
    fn collect_in_a_block(&mut self, body: &[Node], chain: &mut Vec<TheBlock>, function: bool) {
        let block = TheBlock {
            id: self.next,
            function,
        };
        self.next += 1;
        chain.push(block);
        self.collect(body, chain);
        chain.pop();
    }
}

/// The names of the script, the way the script wrote them.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the labels and the
/// gotos of the script are gathered with the chain of the blocks that
/// enclose each of them.
fn the_names_of_the_script(body: &[Node]) -> TheNamesOfTheScript {
    let mut names = TheNamesOfTheScript {
        labels: Vec::new(),
        gotos: Vec::new(),
        next: 0,
    };
    let mut chain = Vec::new();
    names.collect(body, &mut chain);
    names
}

/// The bodies of the hotkeys of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the body
/// of a hotkey is the code that comes after it, and the `return` of it is
/// what ends it. The body of a hotkey that is written in one line is the
/// command of the line, and the code that comes after it does not belong to
/// it.
///
/// The body is not a block: a jump of it may leave it, and the labels of it
/// are the labels of the scope that carries it.
fn the_bodies_of_the_hotkeys(statements: Vec<Node>) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();
    let mut rest = statements.into_iter();
    while let Some(statement) = rest.next() {
        match statement {
            Node::HotkeyDefinition {
                name,
                line,
                body,
                on_the_line,
            } => {
                if on_the_line {
                    // The action of the hotkey is the command of the line,
                    // and the code that comes after it does not belong to
                    // it.
                    result.push(Node::HotkeyDefinition {
                        name,
                        body,
                        on_the_line,
                        line,
                    });
                    continue;
                }
                // The body of the hotkey is the code that comes after it,
                // and the code of the next hotkey is part of it.
                let after: Vec<Node> = rest.by_ref().collect();
                result.push(Node::HotkeyDefinition {
                    name,
                    body: the_bodies_of_the_hotkeys(after),
                    on_the_line,
                    line,
                });
                break;
            }
            other => result.push(other),
        }
    }
    result
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
            Node::If {
                branches,
                otherwise,
                ..
            } => {
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
            Node::Try {
                body,
                catches,
                otherwise,
                finally,
                ..
            } => {
                collect_the_gotos_of_a_scope(body, gotos);
                for catch in catches {
                    collect_the_gotos_of_a_scope(&catch.body, gotos);
                }
                collect_the_gotos_of_a_scope(otherwise, gotos);
                collect_the_gotos_of_a_scope(finally, gotos);
            }
            // The body of a hotkey of the v1 is of the scope that carries it.
            Node::HotkeyDefinition { body, .. } => collect_the_gotos_of_a_scope(body, gotos),
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
        if !token
            .value
            .chars()
            .all(|letter| letter.is_ascii_alphanumeric() || letter == '_')
        {
            return None;
        }
        name.push_str(&token.value);
    }
    if name.is_empty() {
        return None;
    }
    Some(name)
}

/// The name of a property of the literal of an object of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the name
/// is the text that is written until the colon, and the quotes of a text are
/// not part of it: the `{"b": 1}` has the property "b".
fn the_name_of_a_property_of_the_v1(parser: &Parser, start: usize, end: usize) -> String {
    let mut text = String::new();
    for token in parser.tokens.get(start..end).unwrap_or(&[]) {
        if !text.is_empty() {
            text.push(' ');
        }
        text.push_str(&token.value);
    }
    text.trim().to_string()
}

/// Whether the key of a property of the literal of an object of the v1 is a
/// name.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the key is
/// the name when it is one, and the value of the expression when it is not:
/// the `{b: 1}` has the property "b" whatever the value of `b` is, and the
/// `{(b): 1}` has the property of the value of `b`.
fn the_key_of_a_property_of_the_v1_is_a_name(parser: &Parser, start: usize, end: usize) -> bool {
    let tokens = parser.tokens.get(start..end).unwrap_or(&[]);
    tokens.len() == 1 && tokens[0].kind == TokenKind::Name
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
fn the_declarations_of_a_body(
    body: &[Node],
    dialect: &dyn crate::dialect::Dialect,
) -> Result<Declarations, SyntaxError> {
    let mut declarations = Declarations::default();
    let mut first = true;
    for statement in body {
        // The mode is the bare `global` of the first line.
        if first {
            first = false;
            if let Node::Declare {
                scope: DeclareKind::Global,
                names,
                ..
            } = statement
            {
                if names.is_empty() {
                    declarations.assume_global = true;
                    continue;
                }
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 accepts the `local` alone and the `static` alone when
            // they are the first statement of the body of the function, and
            // they declare nothing.
            if let Node::Declare { names, .. } = statement {
                if names.is_empty() && dialect.the_declaration_with_no_name_is_the_first_statement()
                {
                    continue;
                }
            }
        }
        the_declarations_of_a_statement(statement, &mut declarations, dialect)?;
    }
    Ok(declarations)
}

/// The declarations of a statement, and the bare `global` that is refused.
fn the_declarations_of_a_statement(
    statement: &Node,
    declarations: &mut Declarations,
    dialect: &dyn crate::dialect::Dialect,
) -> Result<(), SyntaxError> {
    match statement {
        Node::Declare {
            scope: kind,
            names,
            line,
        } => {
            if *kind == DeclareKind::Global && names.is_empty() {
                // A bare `global` that is not the mode of the function.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 says that the line holds no action.
                return Err(SyntaxError {
                    message: dialect.the_message_of_the_refused_declaration("global"),
                    line: *line,
                    column: 1,
                });
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 refuses the declaration that has no name when it is
            // not the first statement of the body of the function: the
            // `local` alone and the `static` alone.
            if names.is_empty() && dialect.the_declaration_with_no_name_is_the_first_statement() {
                return Err(SyntaxError {
                    message: dialect.the_message_of_the_refused_declaration(kind.word()),
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
        Node::If {
            branches,
            otherwise,
            ..
        } => {
            for branch in branches {
                the_declarations_of_a_body_of(&branch.body, declarations, dialect)?;
            }
            the_declarations_of_a_body_of(otherwise, declarations, dialect)?;
        }
        Node::Switch { cases, .. } => {
            for case in cases {
                the_declarations_of_a_body_of(&case.body, declarations, dialect)?;
            }
        }
        Node::While { body, .. } | Node::Loop { body, .. } | Node::For { body, .. } => {
            the_declarations_of_a_body_of(body, declarations, dialect)?;
        }
        Node::Try {
            body,
            catches,
            otherwise,
            finally,
            ..
        } => {
            the_declarations_of_a_body_of(body, declarations, dialect)?;
            for catch in catches {
                the_declarations_of_a_body_of(&catch.body, declarations, dialect)?;
            }
            the_declarations_of_a_body_of(otherwise, declarations, dialect)?;
            the_declarations_of_a_body_of(finally, declarations, dialect)?;
        }
        _ => {}
    }
    Ok(())
}

/// The declarations of the statements of a block.
fn the_declarations_of_a_body_of(
    body: &[Node],
    declarations: &mut Declarations,
    dialect: &dyn crate::dialect::Dialect,
) -> Result<(), SyntaxError> {
    for statement in body {
        the_declarations_of_a_statement(statement, declarations, dialect)?;
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

/// The place of a parameter of a command of the v1.
///
/// The text of the parameter is the text of the script, and the tokens of it
/// are the tokens that it is made of: the expression of the parameter is
/// read from them.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the piece
/// of a text that the v1 cuts in the middle is not a range of tokens, and it
/// has none.
#[derive(Clone, Debug)]
struct ThePlaceOfTheParameter {
    /// The text of the parameter, the way the script wrote it.
    text: String,
    /// The first character of the parameter: the line and the column.
    from: (u32, u32),
    /// The end of the parameter: the line and the column, the end being
    /// outside of it.
    to: (u32, u32),
    /// The tokens of the parameter: the first one and the one that comes
    /// after it.
    tokens: Option<(usize, usize)>,
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
    /// The groups of the arguments that are not there: the place of the first
    /// one of the group and the number of them.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// counts the arguments that are not there, and the place where they are
    /// decides the complaint of the call: the `f(, 1)` is refused with "Blank
    /// parameter" and the `f(1,)` is refused with "Too many parameters passed
    /// to function." when f takes one parameter.
    blanks: Vec<(usize, usize)>,
    /// Whether the number of the arguments is checked here.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// number of the arguments of a method is not checked at the load -- the
    /// `o.Say(1, 2)` is accepted -- and the list of the arguments that ends
    /// with a blank is refused at the load all the same: the `o.Say(1,)` is
    /// refused with "Blank parameter".
    the_arity_is_checked: bool,
    /// The text of the call, the way the script wrote it.
    source: String,
    /// The place of the call in the script.
    line: u32,
    column: u32,
    /// The failure of the call when the parenthesis that closes the list of
    /// the arguments is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not need that parenthesis when the name of the call is not a
    /// function, and the failure waits for the end of the parse: the name
    /// may be a function of the script that is defined later.
    missing: Option<SyntaxError>,
}

/// The label that a command of the script names.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the label that a
/// command names is checked when the script loads, and the names of the
/// labels of the script are known only at the end of the parse.
struct TheLabelOfTheScript {
    /// The text of the parameter that names the label.
    text: String,
    /// Whether the name of a function of the script is accepted there.
    ///
    /// The action of a hotkey and the body of a timer may be a function, and
    /// the `GroupAdd` names a label and nothing else.
    function: bool,
    /// The line of the command that names the label.
    line: u32,
}

/// A hotkey of the v1, and the place it was written.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the hotkey
/// that is declared twice is refused when the script loads, and the names of
/// the hotkeys are known only at the end of the parse.
struct TheHotkeyOfTheScript {
    /// The name of the hotkey.
    name: crate::v1::TheName,
    /// The line of the definition.
    line: u32,
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
    /// The arity of the functions of the script: the name, the least number
    /// of the arguments and the most.
    ///
    /// The v1 refuses the call of a function at the load, and the number of
    /// the arguments of it is one of the things that it refuses there.
    arities: BTreeMap<String, (usize, usize)>,
    /// The names of the functions of the script, known before the lines of it
    /// are read.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// knows the name of a function that the script defines further down when
    /// it decides what the rest of the line is: the `f(1) { return 2 }` is
    /// refused with "Unexpected "{" when f is a function of the script, and
    /// the `MyFunc() { return 2 }` is refused with "Call to nonexistent
    /// function." when it is not.
    functions_of_the_script: BTreeSet<String>,
    /// The calls of the script that the v1 refuses at the load.
    ///
    /// The name of a function of the script may be defined after the call, so
    /// the calls wait for the end of the parse.
    calls: Vec<TheCallOfTheScript>,
    /// The `until` of the v1 that no loop takes.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `until` of the v1 is the condition of the loop whose body is the
    /// statement before it, and the one that no loop takes is refused when
    /// the load is over: the error of a line that comes later is the one
    /// that is reported.
    untils_without_a_loop: Vec<(String, u32, u32)>,
    /// The labels that the commands of the script name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-03: the label that a
    /// command names is checked when the script loads, and the names of the
    /// labels of the script are known only at the end of the parse.
    labels: Vec<TheLabelOfTheScript>,
    /// The names of the labels of the script, in lower case.
    ///
    /// The label that a command names may be written anywhere in the script,
    /// and the name of a label does not know its case.
    label_names: BTreeSet<String>,
    /// The hotkeys of the v1, and the place they were written.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// hotkey that is declared twice is refused when the script loads, and
    /// the names of the hotkeys are known only at the end of the parse.
    hotkeys: Vec<TheHotkeyOfTheScript>,
    /// Whether the parser is reading the action of a hotkey of the v1.
    ///
    /// The action of a hotkey is a statement of the script, and the line of
    /// it is not the line of another hotkey.
    in_the_action_of_a_hotkey: bool,
    /// Whether the parser is reading the condition of an `if`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `if` and the `while` may come inside the line of the condition of an
    /// `if`, and nowhere else.
    in_the_condition_of_an_if: bool,
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
    /// The bracket that closes the value that is being read.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// operand that is not there and the bracket that closes the value is the
    /// empty text: the `x := (1 +)` is accepted, and the `x := )` is refused
    /// with "Unexpected ")"". The bracket of the block is not one of them: the
    /// `f() { x := 1 +}` is refused with "Unexpected "}"".
    expected_bracket: Option<&'static str>,
    /// The comma that ends the value that is being read.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// comma that separates the items of a list ends the value that is being
    /// read: the `SubStr("abc", 1 +, 1)` is the operand that is not there.
    the_comma_ends_the_value: bool,
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

/// The state of the parser, kept while a text that may not be an expression
/// is read.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the tail of
/// the `if` of the v1 is a text that the v1 reads as the parameters of a
/// command when it is not an expression, and the read of it may fail. The
/// state is kept so that the failure does not leave the parser in a strange
/// place.
struct TheStateOfTheParser {
    /// The place of the token that is being read.
    position: usize,
    /// The place of the statement that is being read.
    statement_start: usize,
    /// The number of the expressions that may hold the operator that may not
    /// follow a literal text.
    expressions: usize,
    /// The operator that may not follow a literal text: the place of it, the
    /// line of it and the column of it.
    pending_literal: Option<(usize, u32, u32)>,
    /// The number of the names that the script is building and that are open.
    derefs: usize,
    /// The number of the assignments of the scope that is being read.
    assignments: usize,
    /// The number of the calls of the script.
    calls: usize,
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
            functions_of_the_script: BTreeSet::new(),
            calls: Vec::new(),
            untils_without_a_loop: Vec::new(),
            labels: Vec::new(),
            label_names: BTreeSet::new(),
            hotkeys: Vec::new(),
            in_the_action_of_a_hotkey: false,
            in_the_condition_of_an_if: false,
            assignments: vec![Vec::new()],
            parameters: Vec::new(),
            in_class: false,
            in_function: false,
            pending_literal: None,
            expressions: Vec::new(),
            statement_start: 0,
            bases: Vec::new(),
            derefs: 0,
            expected_bracket: None,
            the_comma_ends_the_value: false,
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
        token.kind == TokenKind::Keyword
            && words
                .iter()
                .any(|word| token.value.eq_ignore_ascii_case(word))
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
        self.tokens
            .get(self.position + 1)
            .map(|token| token.spaced)
            .unwrap_or(false)
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
            Some(token) => matches!(
                token.kind,
                TokenKind::Name | TokenKind::Keyword | TokenKind::Number
            ),
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
                message: self.the_message_of_a_bracket(
                    &named,
                    &self.the_source_of_the_statement_of_the_line(token.line),
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
            message: self.the_message_of_a_bracket(
                &message,
                &self.the_source_of_the_statement_of_the_line(line),
            ),
            line,
            column: error.column,
        }
    }

    /// The message of the bracket that is not there or that closes nothing.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the bracket and nothing else: the `x := (1` is a "Missing ")" and
    /// the line of the script is not under it. The v2 names the line of the
    /// script under it.
    fn the_message_of_a_bracket(&self, message: &str, text: &str) -> String {
        if self.dialect.the_bracket_is_named_alone() {
            return message.to_string();
        }
        format!("{message}\n     Specifically: {text}")
    }

    /// The message of the token that is not wanted.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the bracket and nothing else: the `x := )` is an "Unexpected ")""
    /// and the line of the script is not under it. The v2 names the line of
    /// the script under it.
    fn the_message_of_an_unexpected_token(&self, token: &Token) -> String {
        let message = format!("Unexpected {:?}", token.value);
        if self.dialect.the_bracket_is_named_alone() && the_token_is_a_bracket(token) {
            return message;
        }
        format!(
            "{message}\n     Specifically: {}",
            the_source_of_the_line(self, token.line)
        )
    }

    /// The failure of the bracket that is not closed, at a place of its own.
    fn the_error_of_a_missing_bracket(&self, closing: &str, line: u32, column: u32) -> SyntaxError {
        SyntaxError {
            message: self.the_message_of_a_bracket(
                &format!("Missing {closing:?}"),
                &self.the_source_of_the_statement_of_the_line(line),
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
        SyntaxError {
            message: message.into(),
            line: token.line,
            column: token.column,
        }
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
            message: format!(
                "Unexpected operator following literal string.\n     Specifically: {text}"
            ),
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
        // The v1 knows the names of the functions of the script before it
        // reads the lines of it.
        self.functions_of_the_script = self.the_functions_of_the_script();
        self.calls.clear();
        self.untils_without_a_loop.clear();
        self.labels.clear();
        self.label_names.clear();
        self.hotkeys.clear();
        self.assignments.clear();
        self.assignments.push(Vec::new());
        self.skip_newlines();
        while self.peek().kind != TokenKind::Eof {
            statements.extend(self.parse_statement()?);
            self.skip_newlines();
        }
        // The body of a hotkey of the v1 is the code that comes after it, and
        // the code of the next hotkey is part of it.
        let statements = the_bodies_of_the_hotkeys(statements);
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
        // The calls of the v1 whose parenthesis that closes them is not there
        // are refused with the others.
        self.the_calls_of_the_v1_are_checked()?;
        // The label that a command of the v1 names is refused at the load too,
        // and the names of the labels of the script are known only now.
        self.the_labels_of_the_script_are_checked(&statements)?;
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `until` of the v1 that no loop takes is refused at the load, and the
        // error of a line that comes later is the one that is reported: the
        // refusal waits for the end of the parse.
        self.the_untils_without_a_loop_are_checked()?;
        Ok(statements)
    }

    /// The `until` of the v1 that no loop takes is refused at the load.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `until` of the v1 is the condition of the loop whose body is the
    /// statement before it, and the one that no loop takes is refused with
    /// "UNTIL with no matching LOOP". The refusal is made when the load is
    /// over: the error of a line that comes later is the one that is
    /// reported.
    fn the_untils_without_a_loop_are_checked(&self) -> Result<(), SyntaxError> {
        let Some((message, line, column)) = self.untils_without_a_loop.first() else {
            return Ok(());
        };
        Err(SyntaxError {
            message: message.clone(),
            line: *line,
            column: *column,
        })
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
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// argument that is not there is refused with "Blank parameter", and the
    /// name that is not a function is refused before it.
    ///
    /// A command of the v1 is not a function: `FileAppend("hello", "*")` is
    /// a call to a name that does not exist. The calls are checked in the
    /// order of the script.
    fn the_calls_of_the_v1_are_checked(&self) -> Result<(), SyntaxError> {
        if !self.dialect.refuses_the_call_of_a_function_at_the_load() {
            return Ok(());
        }
        for call in &self.calls {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the parenthesis that closes the call of a name that is a
            // function is needed, and the failure of it comes before the
            // complaint of the number of the arguments.
            if let Some(failure) = &call.missing {
                if self.arities.contains_key(&call.name) {
                    return Err(failure.clone());
                }
            }
            // A method is not a function of the script: the number of its
            // arguments is not checked here, and the list of the arguments
            // that ends with a blank is refused all the same.
            if !call.the_arity_is_checked {
                if call
                    .blanks
                    .last()
                    .is_some_and(|(start, size)| start + size == call.count)
                {
                    return Err(self.the_error_of_the_arity_of_the_call(call, "Blank parameter"));
                }
                continue;
            }
            let wanted = &call.name;
            // The function of the dialect: the arity is in the table.
            let function = self.dialect.builtins().iter().find(|builtin| {
                builtin.name.to_lowercase() == *wanted
                    && builtin.kind == crate::builtins::BuiltinKind::Function
            });
            if let Some(function) = function {
                // The table says 255 when the built-in takes any number of
                // them.
                let most = if function.max < 255 {
                    function.max as usize
                } else {
                    usize::MAX
                };
                if let Some(complaint) =
                    the_complaint_of_the_arguments(call, function.min as usize, most)
                {
                    return Err(self.the_error_of_the_arity_of_the_call(call, complaint));
                }
                continue;
            }
            // The function of the script: the arity is the one of the
            // parameters of it.
            if let Some((required, most)) = self.arities.get(wanted) {
                if let Some(complaint) = the_complaint_of_the_arguments(call, *required, *most) {
                    return Err(self.the_error_of_the_arity_of_the_call(call, complaint));
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

    /// The labels that the script names are checked at the end of the parse.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-03: the label that a
    /// command names and that is not there is refused when the script loads,
    /// and the complaint of the v1 is "Target label does not exist." -- it
    /// names no text. The refusal comes after the others: the label that is
    /// written twice and the call of a name that is not a function are
    /// refused before it.
    ///
    /// The text that is empty names no label, and the percent sign is the
    /// substitution of the v1: the label that comes out of it is not checked
    /// at the load. The name of the label does not know its case, and the
    /// label may be written anywhere in the script.
    ///
    /// The goto of the v1 is checked here as well, and the complaint of it is
    /// the one of the v1: "Target label does not exist." with the label after
    /// it, and the two complaints of the jump that may not be made. The
    /// refusal that comes first in the script is the one that the v1 says.
    fn the_labels_of_the_script_are_checked(&self, body: &[Node]) -> Result<(), SyntaxError> {
        let mut failures: Vec<SyntaxError> = Vec::new();
        for label in &self.labels {
            let text = label.text.trim();
            if text.is_empty() || text.contains('%') {
                continue;
            }
            let name = text.to_lowercase();
            if self.label_names.contains(&name) {
                continue;
            }
            // The action of a hotkey and the body of a timer may be a
            // function of the script, and the name of the function is
            // accepted there. The function with arguments is not: the
            // `Hotkey, ^!a, MyFunc(1)` is refused.
            if label.function && !text.contains('(') && self.functions.contains(&name) {
                continue;
            }
            failures.push(SyntaxError {
                message: "Target label does not exist.".to_string(),
                line: label.line,
                column: 1,
            });
            // The first one of the commands is the first one of the script.
            break;
        }
        // The goto of the v1 is refused at the end of the load, and the v2
        // refuses it with the labels of the script.
        if !self.dialect.the_goto_is_refused_with_the_labels() {
            failures.extend(self.the_gotos_of_the_v1_are_checked(body));
        }
        // The hotkey of the v1 that is declared twice is refused at the load
        // too, and the complaint of the v1 names the hotkey.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // `$` and the `~` do not make the hotkey another one, and the hotkey
        // that is written twice is refused with "Duplicate hotkey." and the
        // name of it, as the script spelled it.
        let mut names: BTreeSet<String> = BTreeSet::new();
        for hotkey in &self.hotkeys {
            if !names.insert(hotkey.name.identity.clone()) {
                failures.push(SyntaxError {
                    message: format!(
                        "Duplicate hotkey.\n     Specifically: {}",
                        hotkey.name.spelling
                    ),
                    line: hotkey.line,
                    column: 1,
                });
            }
        }
        // The refusal that comes first in the script is the one that the v1
        // says.
        failures.sort_by_key(|failure| failure.line);
        match failures.into_iter().next() {
            Some(failure) => Err(failure),
            None => Ok(()),
        }
    }

    /// The refusals of the gotos of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-03: the label that
    /// the goto names and that is not there is refused when the script loads,
    /// and the complaint of the v1 is "Target label does not exist." with the
    /// label after it. The jump that may not be made is refused too: the
    /// label of a block that does not enclose the jump is "A Goto/Gosub must
    /// not jump into a block that doesn't enclose it.", and the jump that
    /// leaves a function is "Cannot jump from inside a function to outside.".
    ///
    /// The chain of the blocks tells the block of the label and the block of
    /// the jump: the jump may not enter a block that does not enclose it, and
    /// the jump of a function may not leave the function.
    fn the_gotos_of_the_v1_are_checked(&self, body: &[Node]) -> Vec<SyntaxError> {
        let mut failures = Vec::new();
        let names = the_names_of_the_script(body);
        for (name, line, chain) in &names.gotos {
            // The percent sign is the substitution of the v1, and the label
            // that comes out of it is not checked at the load.
            if name.contains('%') {
                continue;
            }
            let Some(refusal) = crate::v1::the_refusal_of_a_goto(name, chain, &names.labels) else {
                continue;
            };
            failures.push(SyntaxError {
                message: refusal,
                line: *line,
                column: 1,
            });
        }
        failures
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
            if crate::classes::is_a_name_of_a_class(base)
                && self.dialect.the_base_may_be_a_class_of_the_runtime()
            {
                continue;
            }
            return Err(SyntaxError {
                message: format!("Unknown class.\n     Specifically: {base}"),
                line: *line,
                column: *column,
            });
        }
        self.check_the_chain_of_the_bases()
    }

    /// The refusal of a chain of bases that returns to where it began.
    ///
    /// Measured on the binary of v2.0.31 on 2026-11-09: `class A extends A`,
    /// and the pair or the trio that are each other's base, are refused at
    /// the load with "Invalid base class." and the name of the **last base of
    /// the chain** in the reason -- `A` for the class that is its own, `B` for
    /// the pair `A extends B` / `B extends A`, `C` for the trio. The name is
    /// the one of the class, and not the spelling of the base: `class A
    /// extends a` answers "A".
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-09: the v1 runs the
    /// same script, so the rule is the dialect's and not the language's.
    fn check_the_chain_of_the_bases(&self) -> Result<(), SyntaxError> {
        if self.dialect.the_class_may_extend_itself() {
            return Ok(());
        }
        for (class, _base, line, column) in &self.bases {
            // The walk goes up from the class by its bases, and a name that
            // was already walked is the one that closes the chain.
            let mut seen: Vec<String> = vec![class.to_lowercase()];
            let mut current = class.clone();
            let mut closes_with = None;
            while let Some(next) = self.base_declared_for(&current) {
                if seen.contains(&next.to_lowercase()) {
                    // The base of the first step that closes is the base of
                    // the class itself; after that it is the class that was
                    // walked last, which is the one its own base answers with.
                    closes_with = Some(if current.eq_ignore_ascii_case(class) {
                        next
                    } else {
                        current.clone()
                    });
                    break;
                }
                seen.push(next.to_lowercase());
                current = next;
            }
            if let Some(found) = closes_with {
                // The reason names the class, and not the spelling of the
                // base: the class of the name is the one of the script.
                let name = self
                    .classes
                    .get(&found.to_lowercase())
                    .cloned()
                    .unwrap_or(found);
                return Err(SyntaxError {
                    message: format!("Invalid base class.\n     Specifically: {name}"),
                    line: *line,
                    column: *column,
                });
            }
        }
        Ok(())
    }

    /// The base that a class of the script declares, if it declares one.
    fn base_declared_for(&self, class: &str) -> Option<String> {
        self.bases
            .iter()
            .find(|(name, _, _, _)| name.eq_ignore_ascii_case(class))
            .map(|(_, base, _, _)| base.clone())
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
        if self
            .dialect
            .the_name_of_a_class_or_of_a_function_may_be_assigned()
        {
            return Ok(());
        }
        for assignment in assignments {
            let key = assignment.name.to_lowercase();
            if let Some(class) = self.the_name_of_the_class(&key) {
                let message = match assignment.op {
                    AssignOp::Set => format!(
                        "This Class cannot be used as an output variable.\n     Specifically: {class}"
                    ),
                    _ => format!(
                        "This Class cannot be assigned a value.\n     Specifically: {class}"
                    ),
                };
                return Err(SyntaxError {
                    message,
                    line: assignment.line,
                    column: assignment.column,
                });
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
                return Err(SyntaxError {
                    message,
                    line: assignment.line,
                    column: assignment.column,
                });
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
                if self
                    .dialect
                    .the_class_that_is_declared_twice_has_its_own_words()
                {
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
            if self
                .dialect
                .the_function_that_is_declared_twice_has_its_own_words()
            {
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
        let Some(message) = message else {
            return Ok(());
        };
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
                return Ok(the_bodies_of_the_hotkeys(body));
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
            if self.the_block_needs_a_body(kind) {
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
            column: self
                .tokens
                .get(self.position - 1)
                .map(|token| token.column)
                .unwrap_or(1),
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
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes none of them: the `if (1)` that is the last line of the script
    /// is a script that loads, and the `while (0)` that is the last line of
    /// it is a script that loads too. The dialect says it
    /// (`the_body_may_be_missing`).
    fn the_block_needs_a_body(&self, kind: &str) -> bool {
        if self.dialect.the_body_may_be_missing() {
            return false;
        }
        matches!(
            kind,
            "If" | "While" | "For" | "Try" | "Catch" | "Else" | "Finally"
        ) || the_loop_of_the_kind(kind)
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
        match self
            .position
            .checked_sub(1)
            .and_then(|at| self.tokens.get(at))
        {
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
                    message: self.the_message_of_the_closing_brace(&text),
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
        let text = the_text_of_the_line_of_the_closing_brace(self, open, self.position);
        Err(self.error(self.the_message_of_the_closing_brace(&text)))
    }

    /// The message of the brace that closes a block and that is not where it
    /// belongs.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the brace and nothing else, and the v2 names the line of the
    /// script under it.
    fn the_message_of_the_closing_brace(&self, text: &str) -> String {
        if self.dialect.the_closing_brace_names_the_text() {
            format!("Unexpected \"}}\"\n     Specifically: {text}")
        } else {
            "Unexpected \"}\"".to_string()
        }
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
            "If" | "While" | "For" => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 names the brace that opens the body of
                // the construct, and the v2 names the construct.
                match self.dialect.the_message_of_the_brace_that_opens_the_body() {
                    Some(message) => message.to_string(),
                    None => format!("Unexpected {kind:?}"),
                }
            }
            "Switch" => "Unexpected \"Switch\"".to_string(),
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
            "=" | "=>" => {
                format!("Syntax error. Did you mean to use \":=\"?\n     Specifically: {line}")
            }
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
            _ => format!(
                "This line does not contain a recognized action.\n     Specifically: {line}"
            ),
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
        let is_a_member =
            matches!(member, Some(token) if token.kind == TokenKind::Name && !token.spaced);
        if !is_a_member {
            return format!(
                "This line does not contain a recognized action.\n     Specifically: {line}"
            );
        }
        // The statement that carries an assignment is named as it was written.
        if self.the_line_has_an_assignment() {
            return format!("Syntax error.\n     Specifically: {line}");
        }
        format!(
            "Syntax error.\n     Specifically: {}",
            self.the_call_of_the_dot()
        )
    }

    /// Whether the line of the statement carries an assignment.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the statement that
    /// is an assignment is named as the script wrote it, and the one that is
    /// a call is named with its parentheses.
    fn the_line_has_an_assignment(&self) -> bool {
        let line = self.peek().line;
        self.tokens.iter().any(|token| {
            token.line == line
                && (token.value == "=" || the_assignment_operator(&token.value).is_some())
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // line of a hotstring that has no second pair of colons is not a
        // hotstring, and the v1 refuses it with "Invalid hotkey." and the
        // line of it.
        let second = text
            .strip_prefix("::")
            .map(|rest| rest.contains("::"))
            .unwrap_or(false);
        if !second {
            return Err(self.error(format!("Invalid hotkey.\n     Specifically: {text}")));
        }
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

    /// The hotkey of the v1: the line that defines it, and its action.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the v1
    /// writes a hotkey in a line of its own, and the action of it is the
    /// label that comes after the line, or the command that the line
    /// carries. The name of the hotkey is the text that comes before the
    /// two colons, and the name that is not the name of a key is refused
    /// with "Invalid hotkey.".
    fn parse_the_hotkey_of_the_v1(&mut self) -> Result<Node, SyntaxError> {
        let line = self.peek().line;
        let at = self
            .the_place_of_the_two_colons(self.statement_start)
            .expect("the line carries a hotkey");
        let name = the_source_of_the_tokens(self, self.statement_start, at);
        let source = the_source_of_the_line(self, line);
        // The name of the hotkey. The name that is not the name of a key is
        // refused when the script loads.
        let Some(the_name) = crate::v1::the_name_of_a_hotkey(&name) else {
            return Err(self.error(format!("Invalid hotkey.\n     Specifically: {source}")));
        };
        // The two colons are what separates the name from the action.
        self.position = at + 1;
        // The action of the hotkey: the command that the line carries, or
        // the label that comes after the line.
        let (body, on_the_line) = if self.at_end_of_statement() {
            // The line of the hotkey ends here, and the body of it is the
            // code that comes after it.
            self.end_of_statement()?;
            (Vec::new(), false)
        } else {
            let body = self.the_action_of_the_hotkey_of_the_v1(&source)?;
            (body, true)
        };
        // The name of the hotkey is remembered, and the hotkey that is
        // declared twice is refused when the script loads.
        let spelling = the_name.spelling.clone();
        self.hotkeys.push(TheHotkeyOfTheScript {
            name: the_name,
            line,
        });
        Ok(Node::HotkeyDefinition {
            name: spelling,
            body,
            on_the_line,
            line,
        })
    }

    /// Whether the line is the line of a hotkey of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// two colons are what says so. The line of a command is not the line
    /// of a hotkey: the `if (1) :: {` is the `if` of the v1, and the `x :=
    /// 1 :: y := 2` is the assignment of the v1.
    fn the_line_carries_a_hotkey(&self) -> bool {
        self.the_line_at_carries_a_hotkey(self.statement_start)
    }

    /// Whether the line that begins at this place is the line of a hotkey.
    fn the_line_at_carries_a_hotkey(&self, at_the_line: usize) -> bool {
        let Some(at) = self.the_place_of_the_two_colons(at_the_line) else {
            return false;
        };
        // The name of the hotkey is not empty.
        if at == at_the_line {
            return false;
        }
        // The line of a command is the line of the command.
        let token = &self.tokens[at_the_line];
        if token.kind == TokenKind::Name {
            let name = token.value.to_lowercase();
            // The name of a command of the v1.
            if crate::v1::is_the_name_of_a_command(&name) {
                return false;
            }
            // The name that is followed by the assign sign is the name of
            // a variable, and the line is an assignment.
            if let Some(next) = self.tokens.get(at_the_line + 1) {
                if next.value == ":=" || next.value == "=" {
                    return false;
                }
            }
        }
        true
    }

    /// Whether the line that comes next is the line of a hotkey of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// line of a hotkey is the line of the hotkey, and not the continuation
    /// of the expression that comes before it. The expression of the v1 goes
    /// on at the next line when the next line begins with an operator.
    fn the_next_line_carries_a_hotkey(&self) -> bool {
        if !self.dialect.the_hotkeys_are_labels() {
            return false;
        }
        // The place of the line that comes next.
        let mut at = self.position;
        while at < self.tokens.len() && self.tokens[at].kind == TokenKind::Newline {
            at += 1;
        }
        self.the_line_at_carries_a_hotkey(at)
    }

    /// The place of the two colons in the line, when the line carries them.
    ///
    /// The two colons of a text are part of the text, and the two colons of
    /// a comment are part of the comment.
    fn the_place_of_the_two_colons(&self, at_the_line: usize) -> Option<usize> {
        let mut at = at_the_line;
        while at < self.tokens.len() {
            let token = &self.tokens[at];
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                return None;
            }
            if token.kind == TokenKind::Op && token.value == "::" {
                return Some(at);
            }
            at += 1;
        }
        None
    }

    /// The action of a hotkey of the v1 that is written in one line.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// action may be a command, an assignment, a jump or a call, and the
    /// constructs of the v1 are refused with "Invalid single-line
    /// hotkey/hotstring.".
    fn the_action_of_the_hotkey_of_the_v1(
        &mut self,
        source: &str,
    ) -> Result<Vec<Node>, SyntaxError> {
        // The constructs of the v1 are refused before they are read: the
        // `if (1) x := 1` is not a body that the v1 reads on one line, and
        // the contexts of the v1 are refused there too.
        if self.the_action_is_a_construct_of_the_v1() {
            return Err(self.error(format!(
                "Invalid single-line hotkey/hotstring.\n     Specifically: {source}"
            )));
        }
        // The action is a statement of the script.
        self.in_the_action_of_a_hotkey = true;
        let statements = self.parse_statement();
        self.in_the_action_of_a_hotkey = false;
        statements
    }

    /// Whether the action of a hotkey is a construct of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// `if`, the `while`, the `loop` and the `for` are refused in the action
    /// of a hotkey that is written in one line, and so are the contexts of
    /// the v1 -- the `ifwinactive` and the `ifequal`.
    fn the_action_is_a_construct_of_the_v1(&self) -> bool {
        let Some(token) = self.tokens.get(self.position) else {
            return false;
        };
        if token.kind == TokenKind::Keyword {
            return matches!(
                token.value.to_lowercase().as_str(),
                "if" | "while" | "loop" | "for"
            );
        }
        if token.kind == TokenKind::Name {
            return crate::v1::the_context_of_the_v1_is_refused_in_a_hotkey(&token.value);
        }
        false
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // hotkey of the v1 is a line of its own, and the two colons that
        // separate the name of it from the action are what says so.
        if self.dialect.the_hotkeys_are_labels()
            && !self.in_the_action_of_a_hotkey
            && self.the_line_carries_a_hotkey()
        {
            return Ok(vec![self.parse_the_hotkey_of_the_v1()?]);
        }
        if token.kind == TokenKind::Keyword {
            let word = token.value.to_lowercase();
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the `else`
            // of the v1 is a command of its own, and the parser of the v2
            // does not take it.
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the words of the v1 may be the name of a variable: the
            // `while := 5` writes the variable "while", and the `else := 2`
            // writes the variable "else". The `if` is a command of the v1,
            // and the values are not the name of a variable.
            if self.dialect.the_words_may_be_the_name_of_a_variable() {
                if self.the_word_is_the_name_of_a_variable() {
                    match word.as_str() {
                        "true" | "false" => {
                            return Err(SyntaxError {
                                message: format!(
                                    "Not allowed as an output variable.\n     Specifically: {}",
                                    token.value
                                ),
                                line: token.line,
                                column: token.column,
                            });
                        }
                        "if" => {}
                        _ => return self.parse_name_statement(),
                    }
                }
            } else if self.the_word_is_the_name_of_a_variable() {
                // Measured on the binary of v2.0.19 on 2026-11-01: the word that
                // is the name of a variable refuses with the reason of the word,
                // and the reason does not depend on the place where the word is.
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
            if self.dialect.the_command_is_the_else(&word) {
                // The name of the command is consumed before the parameters
                // of it are read, and the name of the command is not one of
                // them.
                self.next();
                return Ok(vec![self.parse_call_statement(token)?]);
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
                    Some(TheRefusal::Variable)
                        if matches!(word.as_str(), "static" | "global" | "local") =>
                    {
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
                    // Measured on the binary of v2.0.31 on 2026-11-09: a body
                    // that has no head is a load error, and it says so with
                    // the word it found. v2.0.19 let the line pass and ran
                    // nothing, which the changelog of v2.0.20 records as a
                    // fix: "Fixed erroneous `Else` placement to raise an
                    // error, not crash on load."
                    //
                    // The legitimate forms never reach this arm: the `if`
                    // takes its own `else`, and the `try` takes the `catch`,
                    // the `else` and the `finally` that belong to it. What is
                    // here is the word on its own.
                    let token = self.peek().clone();
                    let kind = if token.value.eq_ignore_ascii_case("catch") {
                        "Catch"
                    } else if token.value.eq_ignore_ascii_case("finally") {
                        "Finally"
                    } else {
                        "Else"
                    };
                    return Err(SyntaxError {
                        message: self.dialect.the_refusal_of_a_body_without_a_head(kind),
                        line: token.line,
                        column: token.column,
                    });
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
                            "\"{}\" requires at least 1 parameter.\n     Specifically: {}",
                            self.dialect
                                .the_name_of_the_word_that_wants_a_parameter("until"),
                            the_source_of_the_line(self, token.line)
                        )));
                    }
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the `until` of the v1 is the condition of
                    // the loop whose body is the statement before it, and the
                    // one that no loop takes is refused when the load is
                    // over.
                    if let Some(refusal) = self.dialect.the_refusal_of_the_until_that_has_no_loop()
                    {
                        let condition = self.the_expression_of_a_statement()?;
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-04: the parameter of the `until` is the rest
                        // of the line, and the brace that comes after it is
                        // refused at the parse: the error of the line wins
                        // over the refusal of the load.
                        self.end_of_statement()?;
                        self.untils_without_a_loop
                            .push((refusal, token.line, token.column));
                        return Ok(vec![Node::Expression {
                            value: Box::new(condition),
                            line: token.line,
                        }]);
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
                    if self
                        .dialect
                        .the_line_that_begins_with_the_word_holds_no_action(&word)
                    {
                        return Err(self.the_line_holds_no_action(token.line));
                    }
                    if self.the_word_may_not_be_a_name() {
                        if self.the_word_is_the_name_of_a_variable() {
                            return Err(
                                self.the_error_of_the_reserved_word_as_a_name(self.position)
                            );
                        }
                        // The word `not` is the operator of the negation when
                        // an operand follows it, and the v2 takes the script.
                        if !(word == "not" && self.an_operand_follows_the_word()) {
                            return Err(
                                self.the_error_of_the_reserved_word_as_a_call(self.position)
                            );
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // number that comes first may be the name of a variable of the v1.
        if token.kind == TokenKind::Name || self.the_number_is_the_name_of_a_variable(&token) {
            // The number that opens a statement of the v1 is the name of a
            // variable, and the statement is about it.
            self.the_number_becomes_the_name_of_a_variable(self.position);
            // Measured on the binary of v2.0.19 on 2026-11-01: the hash and
            // the semicolon that are glued to the name that opens the
            // statement are a part of it, and the statement holds no action:
            // the whole line is named.
            if self.the_illegal_character_is_glued_to_the_name_that_opens() {
                return Err(self.the_line_holds_no_action(token.line));
            }
            // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-02: the
            // statement that begins with a member whose name is not a name
            // holds no action: the `a.'b' := 5` and the `a.'b' + 1` are
            // refused with "This line does not contain a recognized action."
            // and the whole line is named. The v1 says the same.
            if self.the_statement_begins_with_a_member_whose_name_is_not_a_name() {
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
                return Ok(vec![Node::Expression {
                    value: Box::new(value),
                    line: token.line,
                }]);
            }
            return self.parse_name_statement();
        }
        if token.kind == TokenKind::Op && token.value == "(" {
            // Measured on the binary of v2.0.19 on 2026-10-31: a line that
            // begins with a parenthesis is an expression, and the value it
            // leaves behind is not used.
            let value = self.parse_expression()?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression {
                value: Box::new(value),
                line: token.line,
            }]);
        }
        if token.kind == TokenKind::Op && token.value == "{" {
            // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
            // opens a line is a block of its own, and the statements between
            // the braces run.
            let body = self.parse_block_that_opens_a_line()?;
            return Ok(vec![Node::Block {
                body,
                line: token.line,
            }]);
        }
        if token.kind == TokenKind::Op && matches!(token.value.as_str(), "++" | "--") {
            // Measured on the binary of v2.0.19 on 2026-10-31: the increment
            // may be a statement of its own.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the number that is the operand of the increment names a variable
            // of the v1: the `++0` is the increment of the variable "0".
            self.the_number_becomes_the_name_of_a_variable(self.position + 1);
            let value = self.parse_expression()?;
            self.end_of_statement()?;
            return Ok(vec![Node::Expression {
                value: Box::new(value),
                line: token.line,
            }]);
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
            return Ok(vec![Node::Expression {
                value: Box::new(value),
                line: token.line,
            }]);
        }
        if token.kind == TokenKind::Op
            && !matches!(token.value.as_str(), "[" | "{" | ")" | "]" | "}")
        {
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
                    return Err(
                        self.error(format!("Syntax error.\n     Specifically: {}", token.value))
                    );
                }
                // The assignment is parsed, and the refusal of its left side
                // is the one of an assignment that cannot be written.
                let value = self.parse_expression()?;
                self.end_of_statement()?;
                return Ok(vec![Node::Expression {
                    value: Box::new(value),
                    line: token.line,
                }]);
            }
        }
        // Measured on the binary of v2.0.19 on 2026-10-31: the number that
        // is at the start of a statement and that is followed by a comma is
        // a call of v2 that is not written: the name of it is the number,
        // and the comma is not the space that a call wants. The number
        // that carries a point is not a name, and the line holds no
        // action.
        if token.kind == TokenKind::Number
            && !token.value.contains('.')
            && self.the_comma_is_after_the_number()
        {
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
        let message = if token.kind == TokenKind::Number
            || token.kind == TokenKind::String
            || token.kind == TokenKind::UnclosedText
        {
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
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 names the brace and nothing else.
            if self.the_brace_is_at_the_start_of_its_line() {
                "Unexpected \"}\"".to_string()
            } else {
                let text = the_source_of_the_line(self, token.line);
                self.the_message_of_the_closing_brace(&text)
            }
        } else if token.kind == TokenKind::Op && the_token_is_an_illegal_character(&token) {
            // Measured on the binary of v2.0.19 on 2026-11-01: the character
            // that begins a statement is a line that holds no action, and
            // the text that is named begins at it.
            return Err(self.the_error_of_the_illegal_character(self.position, false));
        } else {
            self.the_message_of_an_unexpected_token(&token)
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
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 refuses the brace that follows a value at once, and it
            // does not look for the brace that closes the literal of an
            // object: the `x := 1 {` is refused with "Unexpected "{"".
            if !self.dialect.the_brace_after_a_value_opens_a_literal() {
                return Err(SyntaxError {
                    message: "Unexpected \"{\"".to_string(),
                    line: token.line,
                    column: token.column,
                });
            }
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
        Err(self.error(self.the_message_of_an_unexpected_token(&token)))
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
                && self
                    .dialect
                    .the_name_of_a_class_may_be_a_word_of_the_language())
    }

    /// Whether the number that comes first is the name of a variable.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// names of the v1 may begin with a number, and the number alone is the
    /// name of a variable: the `0 := 9` writes the variable "0", and the
    /// `0 = 9` writes it too. The text of the number is the name of it, and a
    /// point is not a part of a name: the `0.0 := 5` is refused with "The
    /// following variable name contains an illegal character: \"0.0\"".
    fn the_number_is_the_name_of_a_variable(&self, token: &Token) -> bool {
        self.dialect.the_numbers_may_be_names() && token.kind == TokenKind::Number
    }

    /// The number at this place becomes the name of a variable of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// number that opens a statement of the v1 is the name of a variable: the
    /// `0 := 9` writes the variable "0", the `0++` is the increment of it, and
    /// the `0` alone is a line that holds no action. The number is a name
    /// only where the statement may write: inside an expression the number is
    /// a number, and the `x := 0 + 3` is the sum of two numbers.
    fn the_number_becomes_the_name_of_a_variable(&mut self, at: usize) {
        if !self.dialect.the_numbers_may_be_names() {
            return;
        }
        if self
            .tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Number)
            .unwrap_or(false)
        {
            self.tokens[at].kind = TokenKind::Name;
        }
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
            let pieces = self.the_pieces_of_a_dynamic_name(
                name_token.value.clone(),
                at_the_name,
                name_token.line,
            )?;
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
                value: Box::new(Node::DynamicName {
                    pieces,
                    line: name_token.line,
                }),
                line: name_token.line,
            }]);
        }
        if self.at_op(&["("]) && !self.peek().spaced {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the single quote is not a text in the v1, and the symbol that
            // begins with it is an illegal character in an expression.
            if self.dialect.the_single_quote_is_not_a_text()
                && the_symbol_begins_with_the_illegal_quote(&name_token)
            {
                return Err(self.the_error_of_the_illegal_character(self.position - 1, true));
            }
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
                self.parameters.push(
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect(),
                );
                self.assignments.push(Vec::new());
                // A function may not contain a class.
                let was_in_a_function = self.in_function;
                self.in_function = true;
                // Measured on the binary of v2.0.19 on 2026-11-01: the body of
                // a function may be an expression, and the fat arrow is what
                // says so: `f(a) => a + 1` is a function that answers the
                // expression of it.
                let (body, body_line) = if self.the_fat_arrow_comes_next() {
                    let body = self.the_body_of_a_fat_arrow(name_token.line)?;
                    (body, name_token.line)
                } else {
                    self.the_body_of_a_scope("function", name_token.line)?
                };
                self.in_function = was_in_a_function;
                self.check_the_labels(&body)?;
                let assignments = core::mem::take(
                    self.assignments
                        .last_mut()
                        .expect("the list of the assignments"),
                );
                self.assignments.pop();
                self.parameters.pop();
                let declarations = the_declarations_of_a_body(&body, self.dialect)?;
                // The assignments of the function that live in the scope of
                // the script are the ones of the names it declares global.
                for assignment in assignments {
                    if declarations
                        .globals
                        .contains(&assignment.name.to_lowercase())
                    {
                        self.check_the_assignments(vec![assignment])?;
                    }
                }
                if at_the_top {
                    self.the_name_is_declared_twice(
                        &name_token.value,
                        false,
                        name_token.line,
                        name_token.column,
                    )?;
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
                    body_line,
                }]);
            }
            self.the_expression_begins();
            let (arguments, missing) = self.the_arguments_of_a_call(&name_token.value)?;
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1
            // refuses the call of a name that is not a function at the load,
            // and a call that is a statement is a call: the `MyFunc()` is
            // refused with "Call to nonexistent function." and the text of it
            // is named.
            if self.dialect.refuses_the_call_of_a_function_at_the_load() {
                self.the_call_of_the_v1_is_recorded(
                    &name_token.value.to_lowercase(),
                    &arguments,
                    at_the_name,
                    missing,
                    true,
                    true,
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
                Node::Expression {
                    value: Box::new(value),
                    line: name_token.line,
                }
            };
            let mut statements = vec![statement];
            statements.extend(legacy);
            return Ok(statements);
        }
        // `obj.Prop := value` and `obj[key] := value`: the left side is a
        // value and not a name.
        if self.at_member_dot() || self.at_member_bracket() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the name of the v1 may not carry an illegal character, and the
            // value that holds the member is a variable: the `a"b".Prop := 5`
            // is refused.
            if let Some(refusal) = self.the_refusal_of_the_name(&name_token) {
                return Err(refusal);
            }
            let target = Node::Var {
                name: name_token.value.clone(),
                line: name_token.line,
            };
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
                    ":=" | "+="
                        | "-="
                        | "*="
                        | "/="
                        | "//="
                        | ".="
                        | "&="
                        | "|="
                        | "^="
                        | "<<="
                        | ">>="
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
                other => Node::Expression {
                    value: Box::new(other),
                    line: name_token.line,
                },
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
            ":=" | "+="
                | "-="
                | "*="
                | "/="
                | "//="
                | ".="
                | "&="
                | "|="
                | "^="
                | "<<="
                | ">>="
                | ">>>="
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
        Ok(Node::Call {
            name,
            arguments,
            line: name_token.line,
        })
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
        let carried = self.dialect.builtins().iter().any(|builtin| {
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
        let comma_after_the_name = self.at_op(&[","]);
        if comma_after_the_name {
            self.next();
            places = self.the_places_of_the_parameters_of_a_legacy_command()?;
        } else if !self.at_end_of_statement() {
            places = self.the_places_of_the_parameters_of_a_legacy_command()?;
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 cuts the text of a parameter at the commas, and the quotes and
        // the groups do not hold the comma back.
        let mut cut = self.the_cut_places_of_the_parameters_of_a_legacy_command(&name, &places);
        let mut parameters: Vec<String> = cut.iter().map(|place| place.text.clone()).collect();
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the comma at
        // the end of the line does not open a parameter: the number of the
        // parameters of a command is the number of the commas that are not
        // the last one.
        if parameters
            .last()
            .map(|text| text.is_empty())
            .unwrap_or(false)
        {
            parameters.pop();
            cut.pop();
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
            if count > 0 && parameters.len() > count && !self.dialect.the_command_has_a_body(&name)
            {
                let from = cut[count - 1].from;
                let to = cut.last().expect("the parameters are there").to;
                let text = self.the_source_between(from, to).trim().to_string();
                parameters.truncate(count);
                cut.truncate(count);
                parameters[count - 1] = text.clone();
                cut[count - 1].text = text;
                cut[count - 1].to = to;
            }
        }
        // Measured on the binary of v1.1.37.02 on 2026-11-03: the label that
        // a command names is checked when the script loads, and the names of
        // the labels of the script are known only at the end of the parse:
        // the command is recorded here and the check waits.
        if let Some(label) = self.dialect.the_label_of_the_command(&name) {
            let (place, function) = match label {
                crate::dialect::TheLabelOfACommand::Label(place) => (place, false),
                crate::dialect::TheLabelOfACommand::LabelOrFunction(place) => (place, true),
            };
            if let Some(text) = parameters.get(place) {
                self.labels.push(TheLabelOfTheScript {
                    text: text.clone(),
                    function,
                    line: name_token.line,
                });
            }
        }
        self.end_of_statement()?;
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` of
        // the v1 is the action of the other side of the `if` that comes
        // before it, and it is a failure when there is no `if`.
        if self.dialect.the_command_is_the_else(&name) {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `else` of the v1 takes no parameter, and the text that
            // comes right after it is the body of it: the `else FileAppend,
            // hello, *` runs the command and not a parameter. The `else`
            // that has no `if` before it is refused at the load with "ELSE
            // with no matching IF".
            //
            // The text that comes after a comma is a parameter, and it is
            // named.
            if !parameters.is_empty() && comma_after_the_name {
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
                return Err(self.error(self.the_message_of_the_arity(refusal, name_at, &cut)));
            }
            if (count as u8) < builtin.min {
                return Err(self.error(format!(
                    "\"{}\" requires at least {} parameter{}.\n     Specifically: {}",
                    builtin.name,
                    builtin.min,
                    if builtin.min == 1 { "" } else { "s" },
                    self.the_text_of_a_legacy_command(name_at, &cut),
                )));
            }
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the
            // parameter that wants the name of a variable may not be blank,
            // and the complaint of the v1 is its own.
            // Measured on the binary of v1.1.37.02 on 2026-11-02: the
            // sub-command of a command of the v1 may not be blank either,
            // and the complaint of the v1 is the same one.
            for (index, parameter) in parameters.iter().enumerate() {
                let wants_a_name = matches!(
                    builtin.parameters.get(index),
                    Some(
                        crate::builtins::Parameter::Variable
                            | crate::builtins::Parameter::Output
                            | crate::builtins::Parameter::Name
                    )
                );
                let wants_a_sub_command = self
                    .dialect
                    .the_parameter_may_not_be_blank(&name, index + 1);
                if (wants_a_name || wants_a_sub_command) && parameter.trim().is_empty() {
                    return Err(self.error(format!(
                        "\"{}\" requires that parameter #{} be non-blank.\n     Specifically: {}",
                        builtin.name,
                        index + 1,
                        self.the_text_of_a_legacy_command(name_at, &cut),
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
                    Some(crate::builtins::Parameter::Variable | crate::builtins::Parameter::Output)
                );
                if is_a_name {
                    if let Some(refusal) = self
                        .dialect
                        .the_refusal_of_the_name_of_a_variable(index + 1, parameter)
                    {
                        return Err(self.error(refusal));
                    }
                    continue;
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
                // the percent sign of a parameter that is a text is checked
                // when the script loads: the `FileAppend, a%b, *` is refused.
                if self.dialect.the_parameter_wants_a_number(&name, index) {
                    continue;
                }
                if let Some(refusal) = self
                    .dialect
                    .the_refusal_of_the_percent_sign_of_a_parameter(index + 1, parameter)
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
            if let Some(refusal) = self
                .dialect
                .the_refusal_of_an_invalid_parameter(&name, &parameters)
            {
                return Err(self.error(refusal));
            }
        }
        // The command of the v1 that has a body: the action is the rest of
        // the line, the block, or the statement of the next line.
        if self.dialect.the_command_has_a_body(&name) {
            return self.parse_the_body_of_a_legacy_command(name, parameters, cut, name_token.line);
        }
        let parameters = self.the_parameters_of_a_legacy_command(&name, &cut);
        Ok(Node::LegacyCall {
            name,
            parameters,
            line: name_token.line,
        })
    }

    /// The source of a command of the v1, from its name to the end of the
    /// line.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal of a
    /// command that the dialect does not have names the command, and not the
    /// line that holds it: the `else` that is on the same line is not a part
    /// of it.
    fn the_source_of_the_legacy_command(&self, name_at: usize) -> String {
        let line = self
            .tokens
            .get(name_at)
            .map(|token| token.line)
            .unwrap_or(0);
        let mut end = name_at;
        while end < self.tokens.len() && self.tokens[end].line == line {
            end += 1;
        }
        the_source_of_the_tokens(self, name_at, end)
            .trim_end()
            .to_string()
    }

    /// The row of the command of the v1 that has this name.
    fn the_builtin_of_the_legacy_command(
        &self,
        name: &str,
    ) -> Option<&'static crate::builtins::Builtin> {
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
        places: &[ThePlaceOfTheParameter],
    ) -> String {
        let Some(name) = self.tokens.get(name_at) else {
            return String::new();
        };
        let from = (name.line, name.column);
        let end = match places.first() {
            Some(place) => place.to,
            None => {
                // The command has no parameter: the text is the name and the
                // comma that comes after it.
                let comma = self.tokens.get(name_at + 1);
                let has_a_comma = comma
                    .map(|token| token.kind == TokenKind::Op && token.value == ",")
                    .unwrap_or(false);
                let after = if has_a_comma {
                    name_at + 2
                } else {
                    name_at + 1
                };
                match self.tokens.get(after) {
                    Some(token) => (token.line, token.column),
                    None => self.the_end_of_the_tokens(after),
                }
            }
        };
        self.the_source_between(from, end).trim_end().to_string()
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
        places: &[ThePlaceOfTheParameter],
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
        mut places: Vec<ThePlaceOfTheParameter>,
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
        let action =
            if parameters.len() > own && self.the_text_begins_with_a_command(&parameters[own]) {
                let action = parameters[own..].to_vec();
                parameters.truncate(own);
                Some(action)
            } else {
                if parameters.len() > own && own > 0 {
                    // The last parameter of the command takes the rest of the
                    // line, the way it does for every other command.
                    let from = places[own - 1].from;
                    let to = places.last().expect("the parameters are there").to;
                    let text = self.the_source_between(from, to).trim().to_string();
                    parameters.truncate(own);
                    places.truncate(own);
                    parameters[own - 1] = text.clone();
                    places[own - 1].text = text;
                    places[own - 1].to = to;
                }
                None
            };
        let own_parameters = parameters;
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the value of
        // the parameter of a command that has a body is checked when the
        // script loads, and the text that the check sees is the text that is
        // left of the line: the `IfMsgBox, OK, extra` is refused with
        // "Parameter #1 invalid." and the text "OK, extra".
        if let Some(refusal) = self
            .dialect
            .the_refusal_of_an_invalid_parameter(&name, &own_parameters)
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
                if parameters
                    .last()
                    .map(|text| text.is_empty())
                    .unwrap_or(false)
                {
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
    fn the_texts_of_the_parameters_of_a_legacy_command(
        &mut self,
    ) -> Result<Vec<String>, SyntaxError> {
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
        places: &[ThePlaceOfTheParameter],
    ) -> Vec<crate::ast::LegacyParameter> {
        let mut parameters = Vec::with_capacity(places.len());
        for (index, place) in places.iter().enumerate() {
            match self.the_expression_of_a_parameter(name, index, place) {
                Some(node) => parameters.push(crate::ast::LegacyParameter::Expression(node)),
                None => parameters.push(crate::ast::LegacyParameter::Text(place.text.clone())),
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
        place: &ThePlaceOfTheParameter,
    ) -> Option<Node> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // percent sign and a space in front of a parameter force it to be an
        // expression: the `FileAppend, % "hello", *` writes "hello", and the
        // `FileAppend, % n, *` writes the number of the variable.
        if let Some(node) = self.the_expression_of_the_percent_and_the_space(place) {
            return Some(node);
        }
        // The parameter that wants a number is an expression, and the others
        // are texts.
        if !self.dialect.the_parameter_wants_a_number(name, index) {
            return None;
        }
        // The piece of a text that the v1 cuts in the middle is not an
        // expression: it has no tokens.
        let (start, end) = place.tokens?;
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
            // The parameter that keeps its text is not an expression, and the
            // v1 reads the text of it: the amount of the `EnvAdd` that is
            // "5abc" is not a number.
            _ if self.dialect.the_parameter_keeps_its_text(name, index) => Some(Node::Literal {
                value: crate::ast::LiteralValue::Text(String::new()),
                line,
            }),
            // The parameter is not an expression, and the v1 reads it as the
            // number zero.
            _ => Some(Node::Literal {
                value: crate::ast::LiteralValue::Integer(0),
                line,
            }),
        }
    }

    /// The expression of a parameter that is written with a percent sign and a
    /// space in front of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// percent sign and a space in front of a parameter of a command of the v1
    /// force it to be an expression: the `FileAppend, % "hello", *` writes
    /// "hello", and the `FileAppend, % n, *` writes the number of the
    /// variable.
    fn the_expression_of_the_percent_and_the_space(
        &mut self,
        place: &ThePlaceOfTheParameter,
    ) -> Option<Node> {
        let (start, end) = place.tokens?;
        // The percent sign is the first token of the parameter, and the space
        // is the one that comes after it.
        let percent = self.tokens.get(start)?;
        if percent.kind != TokenKind::Op || percent.value != "%" {
            return None;
        }
        let after = self.tokens.get(start + 1)?;
        if !after.spaced {
            return None;
        }
        // The expression is the text that comes after the percent sign.
        let saved = self.position;
        self.position = start + 1;
        let parsed = self.parse_expression();
        let after = self.position;
        self.position = saved;
        match parsed {
            Ok(node) if after == end => Some(node),
            _ => None,
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

    /// The places of the parameters of a command of the v1, cut the way the
    /// v1 cuts them.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// v1 cuts the text of a parameter at the commas, and the quotes and the
    /// groups of parentheses do not hold the comma back: the `FileAppend,
    /// "a,b", *` has three parameters -- `"a`, `b"` and `*` -- and the v1
    /// refuses it with "Parameter #3 invalid.".
    ///
    /// The parameter that wants a number is an expression, and there the
    /// comma that is inside a text or inside a group does not separate.
    fn the_cut_places_of_the_parameters_of_a_legacy_command(
        &self,
        name: &str,
        places: &[(usize, usize)],
    ) -> Vec<ThePlaceOfTheParameter> {
        let mut cut = Vec::with_capacity(places.len());
        for (index, (start, end)) in places.iter().enumerate() {
            // The parameter that wants a number is an expression, and the
            // comma that is inside a text or inside a group does not
            // separate it.
            if self.dialect.the_parameter_wants_a_number(name, index) {
                cut.push(self.the_place_of_the_tokens(*start, *end));
                continue;
            }
            cut.extend(self.the_pieces_of_a_text_parameter(*start, *end));
        }
        cut
    }

    /// The place of a parameter that is a whole range of tokens.
    fn the_place_of_the_tokens(&self, start: usize, end: usize) -> ThePlaceOfTheParameter {
        let text = self.the_verbatim_source(start, end);
        let from = self
            .tokens
            .get(start)
            .map(|token| (token.line, token.column))
            .unwrap_or((0, 0));
        ThePlaceOfTheParameter {
            text: text.trim().to_string(),
            from,
            to: self.the_end_of_the_tokens(end),
            tokens: Some((start, end)),
        }
    }

    /// The pieces of a parameter of a command of the v1 that is a text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// v1 cuts the text of a parameter at the commas, and the quotes and the
    /// groups do not hold the comma back: the `FileAppend, "a,b", *` has
    /// three parameters -- `"a`, `b"` and `*` -- and the v1 refuses it with
    /// "Parameter #3 invalid.". The escape holds the comma back: the
    /// `FileAppend, a`,b, *` is the text "a,b" and not two parameters.
    ///
    /// The piece of a text is not a range of tokens, and it has none: the
    /// expression of a parameter is not read from it.
    fn the_pieces_of_a_text_parameter(
        &self,
        start: usize,
        end: usize,
    ) -> Vec<ThePlaceOfTheParameter> {
        let text = self.the_verbatim_source(start, end);
        let from = self
            .tokens
            .get(start)
            .map(|token| (token.line, token.column))
            .unwrap_or((0, 0));
        let chars: Vec<char> = text.chars().collect();
        let mut cuts = Vec::new();
        let mut index = 0usize;
        while index < chars.len() {
            if chars[index] == '`' {
                // The escape holds the comma back, and the character that it
                // carries is not a part of the text that is read.
                index += 2;
                continue;
            }
            if chars[index] == ',' {
                cuts.push(index);
            }
            index += 1;
        }
        if cuts.is_empty() {
            return vec![ThePlaceOfTheParameter {
                text: text.trim().to_string(),
                from,
                to: self.the_end_of_the_tokens(end),
                tokens: Some((start, end)),
            }];
        }
        let mut pieces = Vec::with_capacity(cuts.len() + 1);
        let mut begin = 0usize;
        for cut in cuts {
            let piece: String = chars[begin..cut].iter().collect();
            pieces.push(ThePlaceOfTheParameter {
                text: piece.trim().to_string(),
                from: (from.0, from.1 + begin as u32),
                to: (from.0, from.1 + cut as u32),
                tokens: None,
            });
            begin = cut + 1;
        }
        let piece: String = chars[begin..].iter().collect();
        pieces.push(ThePlaceOfTheParameter {
            text: piece.trim().to_string(),
            from: (from.0, from.1 + begin as u32),
            to: self.the_end_of_the_tokens(end),
            tokens: None,
        });
        pieces
    }

    /// The place where a range of tokens ends: the line and the column.
    fn the_end_of_the_tokens(&self, end: usize) -> (u32, u32) {
        let Some(last) = self.tokens.get(end.saturating_sub(1)) else {
            return (0, 0);
        };
        let Some(line) = self.lines.get(last.line as usize - 1) else {
            return (last.line, last.column);
        };
        let chars: Vec<char> = line.chars().collect();
        let to = match self.tokens.get(end) {
            Some(next) if next.line == last.line => next.column.saturating_sub(1) as usize,
            _ => chars.len(),
        };
        (last.line, to as u32 + 1)
    }

    /// The source of the script between two places.
    ///
    /// The places are the line and the column: the line and the column of
    /// the first character, and the line and the column of the end of it, the
    /// end being outside of it.
    fn the_source_between(&self, from: (u32, u32), to: (u32, u32)) -> String {
        if from.0 == to.0 {
            let Some(line) = self.lines.get(from.0 as usize - 1) else {
                return String::new();
            };
            let chars: Vec<char> = line.chars().collect();
            let start = from.1.saturating_sub(1) as usize;
            let end = to.1.saturating_sub(1) as usize;
            if start >= end || end > chars.len() {
                return String::new();
            }
            return chars[start..end].iter().collect();
        }
        // The text that goes from one line to the next: the lines of the
        // script, with the linefeed in the middle.
        let mut text = String::new();
        for line in from.0..=to.0 {
            let Some(source) = self.lines.get(line as usize - 1) else {
                continue;
            };
            let chars: Vec<char> = source.chars().collect();
            let start = if line == from.0 {
                from.1.saturating_sub(1) as usize
            } else {
                0
            };
            let end = if line == to.0 {
                to.1.saturating_sub(1) as usize
            } else {
                chars.len()
            };
            if start >= end || end > chars.len() {
                continue;
            }
            if !text.is_empty() {
                text.push('\n');
            }
            text.extend(chars[start..end].iter());
        }
        text
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

    /// The refusal of the name of the v1 that is not the name of a variable.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// name of the v1 may not carry a quote, and the escape that the name
    /// carries is expanded before the name is looked at: the `x := a"b` and
    /// the `x := a`b` are refused the same way, and the complaint of the v1
    /// names the name that the escape made.
    ///
    /// The name of a variable of the v1 may carry a point: the `0.0 := 5`
    /// writes the variable "0.0", and the v1 does not refuse it.
    fn the_refusal_of_the_name(&self, token: &Token) -> Option<SyntaxError> {
        if !self.dialect.the_name_may_not_carry_an_illegal_character() {
            return None;
        }
        if crate::v1::is_the_name_of_a_variable_that_may_carry_a_point(&token.value) {
            return None;
        }
        Some(self.the_name_that_carries_a_quote(&token.value, token.line, token.column))
    }

    /// The refusal of the name of the v1 that carries a quote.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// name of the v1 may not carry a quote, and the complaint of the v1
    /// names the name: "The following variable name contains an illegal
    /// character:" and the name between quotes.
    fn the_name_that_carries_a_quote(&self, name: &str, line: u32, column: u32) -> SyntaxError {
        SyntaxError {
            message: format!(
                "The following variable name contains an illegal character:\n\"{name}\""
            ),
            line,
            column,
        }
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
        let Some(first) = self.tokens.get(start) else {
            return String::new();
        };
        let Some(last) = self.tokens.get(end.saturating_sub(1)) else {
            return String::new();
        };
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
        let Some(token) = self.tokens.get(index) else {
            return false;
        };
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
                    while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline
                    {
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
                    while index < self.tokens.len() && self.tokens[index].kind == TokenKind::Newline
                    {
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // name of the v1 may not carry an illegal character, and the target
        // of an assignment is a variable: the `a"b" := 5` is refused.
        if let Some(refusal) = self.the_refusal_of_the_name(&name_token) {
            return Err(refusal);
        }
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
                    message: format!(
                        "expected an assignment operator after {:?}",
                        name_token.value
                    ),
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
        let value = match self.the_empty_value_of_an_assignment() {
            Some(value) => value,
            None => self.the_expression_of_a_statement()?,
        };
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
        let message = self
            .dialect
            .the_refusal_of_the_write_of_a_built_in_variable(&token.value, op)?;
        Some(SyntaxError {
            message,
            line: token.line,
            column: token.column,
        })
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
        let Node::Unary {
            op: UnaryOp::PostIncrement | UnaryOp::PostDecrement,
            operand,
            ..
        } = node
        else {
            return None;
        };
        let Node::Var { name, .. } = operand.as_ref() else {
            return None;
        };
        self.dialect
            .the_refusal_of_the_increment_of_a_built_in_variable(name)
    }

    /// The refusal of a name that carries a quote in the tail of a list.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// reads the tail of a list as symbols, and the name of each one of them
    /// is checked: the `x := 1, a"b"` is refused with the name of it, and so
    /// is the `x := 1, "a"b"` with the name "b"".
    ///
    /// The name of a function is not the name of a variable, and the v1 does
    /// not refuse it: the `x := 1, a"b"(2)` is refused with "Call to
    /// nonexistent function.", and the call is checked at the end of the load.
    fn the_refusal_of_a_name_of_the_tail(&self, start: usize) -> Option<SyntaxError> {
        let end = the_end_of_the_line_of(self, start);
        for index in start..end {
            let token = &self.tokens[index];
            if token.kind != TokenKind::Name {
                continue;
            }
            let is_a_call = self
                .tokens
                .get(index + 1)
                .map(|next| next.value == "(" && !next.spaced)
                .unwrap_or(false);
            if is_a_call {
                continue;
            }
            if let Some(refusal) = self.the_refusal_of_the_name(token) {
                return Some(refusal);
            }
        }
        None
    }

    /// The list of v1 that may follow the expression of a statement.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the expression of
    /// a statement ends at the comma, and the rest of the line is the list
    /// of v1: it is evaluated, and its value is thrown away. The value
    /// that is not there in the list is a "Syntax error.", and the comma
    /// that follows it is named.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// reads the tail of a statement as a list of items, and an item that is
    /// not an expression is a text: it is the parameter of a command, and the
    /// v1 does not run it and does not refuse it. The `f(1) FileAppend,
    /// [tail], *` runs nothing and the load does not fail, and the items that
    /// come after the text do not run either.
    fn the_legacy_list(&mut self) -> Result<Vec<Node>, SyntaxError> {
        if !self.at_op(&[","]) {
            return Ok(Vec::new());
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
        // reads the tail of a list as symbols, and the name of each one of
        // them is checked: the `x := 1, a"b"` is refused with the name of it,
        // and so is the `x := 1, "a"b"` with the name "b"".
        if self.dialect.the_names_of_a_tail_are_checked() {
            if let Some(refusal) = self.the_refusal_of_a_name_of_the_tail(self.position) {
                return Err(refusal);
            }
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
                if self.dialect.the_tail_of_a_list_is_text() {
                    // The item is a text: the v1 does not run it, and the load
                    // does not fail.
                    self.position = the_end_of_the_line_of(self, self.position);
                    break;
                }
                return Err(self.error(format!(
                    "Syntax error.\n     Specifically: {}",
                    the_source_of_the_rest_of_the_line(self, self.position)
                )));
            }
            // The item of the list is an expression, and the v1 reads it as
            // one: the name of a command is a variable when it is read as an
            // expression, and the variable has no value.
            let item = match self.parse_expression() {
                Ok(item) => item,
                Err(error) => {
                    if !self.dialect.the_tail_of_a_list_is_text() {
                        return Err(error);
                    }
                    // The item is not an expression: the v1 reads it as a
                    // text, and the text is not run and does not fail.
                    self.position = the_end_of_the_line_of(self, self.position);
                    break;
                }
            };
            items.push(item);
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        // The text that is left after the last item is not read: the v1 does
        // not run it, and it does not fail.
        if self.dialect.the_tail_of_a_list_is_text() && !self.at_end_of_statement() {
            self.position = the_end_of_the_line_of(self, self.position);
        }
        if items.is_empty() {
            return Ok(Vec::new());
        }
        Ok(vec![Node::Legacy {
            items,
            line: self.peek().line,
        }])
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
            column: self
                .tokens
                .get(start)
                .map(|token| token.column)
                .unwrap_or(1),
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
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// parenthesis that closes the list of the arguments and the comma that
    /// separates them end the value that is being read: the `StrLen(1 +)` is
    /// the operand that is not there, and so is the `SubStr("abc", 1 +, 1)`.
    fn parse_arguments(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let outer_bracket = self.expected_bracket;
        let outer_comma = self.the_comma_ends_the_value;
        self.expected_bracket = Some(")");
        self.the_comma_ends_the_value = true;
        let arguments = self.the_arguments_between_the_parentheses();
        self.expected_bracket = outer_bracket;
        self.the_comma_ends_the_value = outer_comma;
        arguments
    }

    /// The arguments that are between the parentheses.
    fn the_arguments_between_the_parentheses(&mut self) -> Result<Vec<Node>, SyntaxError> {
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
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 counts it: the `f(,)` is refused with
                // "Blank parameter" when f takes one parameter, and the v2
                // drops it.
                if self.the_hole_is_the_last_one(")") {
                    if self.dialect.the_comma_at_the_end_of_a_list_is_a_parameter() {
                        arguments.push(Node::Empty { line });
                    }
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
                let line = self.peek().line;
                self.position += 1;
                // A comma at the end of the list is allowed.
                if self.at_op(&[")"]) {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the v1 counts the comma at the end of the
                    // list: the `f(1,)` is refused with "Too many parameters
                    // passed to function." when f takes one parameter, and the
                    // v2 accepts it.
                    if self.dialect.the_comma_at_the_end_of_a_list_is_a_parameter() {
                        arguments.push(Node::Empty { line });
                    }
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
                        ))
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the names of the v1 may begin with a
                    // number, and the name of a parameter is one of them: the
                    // `f(1) {` is a definition, and the parameter of it is
                    // the variable "1".
                    || (name_token.kind == TokenKind::Number
                        && self.dialect.the_numbers_may_be_names());
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
                    return Err(SyntaxError {
                        message,
                        line: name_token.line,
                        column: name_token.column,
                    });
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
            parameters.push(Parameter {
                name,
                default,
                variadic,
                by_ref,
            });
            if self.at_op(&[","]) {
                self.position += 1;
                continue;
            }
            break;
        }
        Ok(parameters)
    }

    /// The names of the functions of the script, known before the lines of it
    /// are read.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// knows the name of a function that the script defines further down when
    /// it decides what the rest of the line is: the `f(1) { return 2 }` is
    /// refused with "Unexpected "{" when f is a function of the script, and
    /// the `MyFunc() { return 2 }` is refused with "Call to nonexistent
    /// function." when it is not.
    ///
    /// A definition of a function is a statement of its own, and the name of
    /// it is at the start of a line.
    fn the_functions_of_the_script(&self) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        if !self
            .dialect
            .the_tail_of_a_call_to_a_name_that_is_not_a_function_is_text()
        {
            return names;
        }
        let mut at = 0;
        while at < self.tokens.len() {
            let token = &self.tokens[at];
            // The name of a definition is at the start of a line.
            let at_the_start_of_a_line = at == 0
                || self.tokens[at - 1].kind == TokenKind::Newline
                || self.tokens[at - 1].kind == TokenKind::Eof;
            if at_the_start_of_a_line
                && matches!(token.kind, TokenKind::Name | TokenKind::Keyword)
                && self
                    .tokens
                    .get(at + 1)
                    .map(|next| next.kind == TokenKind::Op && next.value == "(")
                    .unwrap_or(false)
                && self.a_definition_follows_at(at + 1)
            {
                names.insert(token.value.to_lowercase());
            }
            at += 1;
        }
        names
    }

    /// Whether the name is a function of the dialect or of the script.
    ///
    /// The names of the functions of the script are known before the lines of
    /// it are read, and the name of a function that the script defines
    /// further down is one of them.
    fn the_name_is_a_function(&self, name: &str) -> bool {
        self.the_name_is_a_builtin_function(name)
            || self.functions_of_the_script.contains(&name.to_lowercase())
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
                            // Measured on the binary of AutoHotkey v1.1.37.02
                            // on 2026-11-04: the v1 decides with the rest of
                            // the line, and the dialect says it.
                            if let Some(answer) = self
                                .dialect
                                .a_line_is_a_definition(&self.the_rest_of_the_line_after(at))
                            {
                                // Measured on the binary of AutoHotkey v1.1.37.02
                                // on 2026-11-04: the definition of the v1 needs
                                // a body, and the `f(a) {` at the end of the
                                // file is a call: it is refused with "Call to
                                // nonexistent function.\n     Specifically:
                                // f(a)".
                                if answer
                                    && self.dialect.the_definition_needs_a_body()
                                    && !self.the_body_of_a_definition_is_there(at)
                                {
                                    return false;
                                }
                                return answer;
                            }
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

    /// Whether the body of a definition is there.
    ///
    /// The body of a definition is the line that comes after the brace that
    /// opens it, and the file that ends with the brace does not have it.
    fn the_body_of_a_definition_is_there(&self, at: usize) -> bool {
        let mut next = the_end_of_the_line_of(self, at);
        while self
            .tokens
            .get(next)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
            next += 1;
        }
        self.tokens
            .get(next)
            .map(|token| token.kind != TokenKind::Eof)
            .unwrap_or(false)
    }

    /// The text of the line that comes after the token.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// decides with it whether the line is a definition of a function: the
    /// `f(a) {` is a definition and the `f(a) { return a }` is a call.
    fn the_rest_of_the_line_after(&self, at: usize) -> String {
        let end = the_end_of_the_line_of(self, at);
        if at + 1 >= end {
            return String::new();
        }
        self.the_verbatim_source(at + 1, end)
    }

    /// The tail of the `if` of the v1: the text that follows the condition
    /// on the line of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// parses the text that follows the condition and does not run it -- the
    /// `if (1) FileDelete, marker.txt` leaves the file where it is -- and the
    /// body of the `if` is the line that comes next. The text that ends with
    /// an operator continues on the line that comes, the way an expression of
    /// the v1 continues, and the line that comes is not a statement of its
    /// own: the `if (1) FileAppend, A, *` takes the line that comes with it.
    ///
    /// The brace that opens the body of the `if` is not a part of the tail:
    /// the `if (1) {` has no tail at all, and the body of it is the block.
    fn the_tail_of_the_if_of_the_v1(&mut self) -> Result<(bool, Vec<Node>), SyntaxError> {
        // The first line of the tail is the line of the condition, and the
        // lines that come after it are the continuation of it.
        let mut first = true;
        // Whether the tail took the line that comes with it.
        let mut took_a_line = false;
        // The places of the commas of the first line: the items of the tail
        // are the texts that come between them.
        let mut commas: Vec<usize> = Vec::new();
        // The end of the first line of the tail.
        let mut end_of_the_first_line = 0usize;
        loop {
            let start = self.position;
            let mut depth = 0usize;
            // The place of the first parameter of the line and the place of
            // the last one: the commas that are not inside a parenthesis
            // are the ones that separate them.
            let mut first_parameter = None;
            let mut last_parameter = self.position;
            // The places of the commas of this line.
            let mut the_commas: Vec<usize> = Vec::new();
            let mut at_the_first_token = true;
            while !self.at_end_of_statement() {
                // The brace that opens the body of the `if` is not a part of
                // the tail.
                if first && at_the_first_token && self.at_op(&["{"]) {
                    break;
                }
                at_the_first_token = false;
                let token = self.next();
                if token.kind == TokenKind::Op {
                    match token.value.as_str() {
                        "(" | "[" | "{" => depth += 1,
                        ")" | "]" | "}" => depth = depth.saturating_sub(1),
                        "," if depth == 0 => {
                            if first_parameter.is_none() {
                                first_parameter = Some(self.position);
                            }
                            last_parameter = self.position;
                            // The place of the comma itself: the item that
                            // comes before it ends there.
                            the_commas.push(self.position - 1);
                        }
                        _ => {}
                    }
                }
            }
            let end = self.position;
            if first {
                // The items of the tail are the texts that come after the
                // commas of the line of the condition.
                commas = the_commas;
                end_of_the_first_line = end;
            }
            // The expression of the tail is the last parameter of the line,
            // and the parameter that continues on the line that comes is the
            // first one of it.
            let parameter = if first {
                (last_parameter, end)
            } else {
                (start, first_parameter.unwrap_or(end))
            };
            first = false;
            // The expression of the tail ends with an operator: it continues
            // on the line that comes, and the line that comes is not a
            // statement of its own.
            if !self.the_text_ends_with_an_operator(parameter.0, parameter.1) {
                break;
            }
            if self.peek().kind != TokenKind::Newline {
                break;
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the line of the `else` is not a part of the tail: the `else`
            // is the other side of the `if`, and the `if (1) FileAppend, A,
            // *` with an `else` under it is refused with "ELSE with no
            // matching IF". The parser stays at the end of the line of the
            // condition, and the body of the `if` is the line that comes.
            let mut look = self.position;
            while self
                .tokens
                .get(look)
                .map(|token| token.kind == TokenKind::Newline)
                .unwrap_or(false)
            {
                look += 1;
            }
            let the_else_comes_next = self
                .tokens
                .get(look)
                .map(|token| {
                    token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("else")
                })
                .unwrap_or(false);
            if the_else_comes_next && self.dialect.the_command_is_the_else("else") {
                break;
            }
            // The line that comes is a part of the tail, and the empty lines
            // between the two are not.
            took_a_line = true;
            self.skip_newlines();
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the line that comes with the tail is an action, and the line
            // that holds no action is refused.
            self.the_line_of_the_tail_is_an_action()?;
            if self.at_end_of_statement() {
                break;
            }
        }
        // The items of the tail are read after it, and the read of them may
        // take the line that comes: the `if (0) x := 1, y := 3` with a `+ 4`
        // under it writes the variable "y" with the value 7.
        let end_of_the_tail = self.position;
        let items = self.the_items_of_the_tail_of_the_v1(&commas, end_of_the_first_line)?;
        if self.position < end_of_the_tail {
            self.position = end_of_the_tail;
        }
        Ok((took_a_line, items))
    }

    /// The line that comes with the tail of the `if` of the v1 is an action.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// line that holds no action is refused: the `if (0) x := 1, FileAppend,
    /// A, *` with a `B` under it is a "This line does not contain a
    /// recognized action. Specifically: B". The line is read as a statement
    /// to know it, and the state of the parser is put back where it was: the
    /// line is a part of the tail, and not a statement of its own.
    fn the_line_of_the_tail_is_an_action(&mut self) -> Result<(), SyntaxError> {
        if self.at_end_of_statement() {
            return Ok(());
        }
        let state = self.the_state_is_kept();
        let read = self.parse_statement();
        self.the_state_is_restored(state);
        read.map(|_| ())
    }

    /// The items of the tail of the `if` of the v1: the texts that come
    /// after the commas of the line of the condition.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
    ///
    /// - The items are expressions, and they are evaluated whether the
    ///   condition is true or false: the `if (0) x := 1, y := 3` writes the
    ///   variable "y".
    /// - The value of them does not count for the truth of the condition: the
    ///   `if (0) x := 0, y := 3` does not run the body of the `if`.
    /// - The command of the v1 is not run by them: the `if (0) x := 1,
    ///   FileAppend, A1, done` does not write the file. The name of the
    ///   command is a variable when it is read as an expression, and the
    ///   variable has no value.
    /// - A text that is not an expression is the parameter of a command, and
    ///   the v1 does not run it: the `if (0) x := 1, *` is a script that
    ///   loads.
    fn the_items_of_the_tail_of_the_v1(
        &mut self,
        commas: &[usize],
        end: usize,
    ) -> Result<Vec<Node>, SyntaxError> {
        let mut items = Vec::new();
        for index in 0..commas.len() {
            // The item begins after the comma and ends at the next one.
            let from = commas[index] + 1;
            let to = if index + 1 < commas.len() {
                commas[index + 1]
            } else {
                end
            };
            // The state of the parser is kept, because the text may not be an
            // expression and the read of it may fail.
            let state = self.the_state_is_kept();
            self.position = from;
            let item = match self.parse_expression() {
                Ok(item) => item,
                Err(_) => {
                    // The text is not an expression: the v1 reads it as the
                    // parameter of a command, and the command is not run.
                    self.the_state_is_restored(state);
                    break;
                }
            };
            // The expression that stops before the end of the item is not the
            // whole of it: the text after it is the parameter of a command.
            if self.position < to {
                self.the_state_is_restored(state);
                break;
            }
            items.push(item);
        }
        Ok(items)
    }

    /// The state of the parser is kept, so that it may be put back where it
    /// was.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// tail of the `if` of the v1 is a text that the v1 reads as the
    /// parameters of a command when it is not an expression, and the read of
    /// it may fail. The state is kept so that the failure does not leave the
    /// parser in a strange place.
    fn the_state_is_kept(&self) -> TheStateOfTheParser {
        TheStateOfTheParser {
            position: self.position,
            statement_start: self.statement_start,
            expressions: self.expressions.len(),
            pending_literal: self
                .pending_literal
                .as_ref()
                .map(|pending| (pending.at, pending.line, pending.column)),
            derefs: self.derefs,
            assignments: self.assignments.last().map(|list| list.len()).unwrap_or(0),
            calls: self.calls.len(),
        }
    }

    /// The state of the parser is put back where it was.
    fn the_state_is_restored(&mut self, state: TheStateOfTheParser) {
        self.position = state.position;
        self.statement_start = state.statement_start;
        self.expressions.truncate(state.expressions);
        self.pending_literal = state
            .pending_literal
            .map(|(at, line, column)| PendingLiteral { at, line, column });
        self.derefs = state.derefs;
        if let Some(list) = self.assignments.last_mut() {
            list.truncate(state.assignments);
        }
        self.calls.truncate(state.calls);
    }

    /// The condition of the legacy `if` of the v1: `if x = 1`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
    ///
    /// - The legacy `if` compares the value of a variable with the text that
    ///   comes after the operator, and the text is not an expression: the
    ///   `if x = 1 + 1` compares the value of `x` with the text "1 + 1",
    ///   and the condition is false.
    /// - The text is the text of the script, and the spaces that come before
    ///   it and the spaces that come after it are not a part of it.
    /// - The form is the legacy one when a name comes first and a comparison
    ///   of the v1 comes after it: the `if x = 1` is the legacy form, and the
    ///   `if x.y = 1` is not.
    /// - The legacy `if` has no One True Brace: the brace that comes after
    ///   the condition on its line is a part of the text, and the block is
    ///   not opened. The brace that closes the block is unexpected.
    fn the_condition_of_a_legacy_if(&mut self) -> Result<Option<Node>, SyntaxError> {
        // The name of the variable and the comparison that comes after it.
        let Some(name) = self.tokens.get(self.position) else {
            return Ok(None);
        };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
        // the names of the v1 may begin with a number, and the number that
        // comes first in the legacy `if` is the name of a variable: the
        // `if 1 = 1` compares the variable "1" with the text "1", and the
        // variable "1" has no value, and the condition is false. The
        // number is a name when the text of it has no point: the `if 1.0 =
        // 1.0` is the form of the expression, and the `if 0x0 = 0x0` is
        // the legacy one.
        if name.kind != TokenKind::Name
            && !(name.kind == TokenKind::Number
                && crate::v1::is_the_name_of_a_variable(&name.value))
        {
            return Ok(None);
        }
        // The name is taken before the parser writes, because the tokens are
        // borrowed while it is read.
        let name_of_the_variable = name.value.clone();
        let line_of_the_variable = name.line;
        let Some(operator) = self.tokens.get(self.position + 1) else {
            return Ok(None);
        };
        if operator.kind != TokenKind::Op {
            // The forms of the legacy `if` that are not a comparison:
            // the `between`, the `in` and the `contains`.
            return self.the_form_of_a_legacy_if(name_of_the_variable, line_of_the_variable);
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // legacy `if` reads the operator from the source, and the `==` of the
        // v2 is not one of the operators of it: the `=` is the operator, and
        // the second equal sign is a part of the text.
        let (op, the_text_begins_with_an_equal_sign) = match operator.value.as_str() {
            "=" => (BinaryOp::Equal, false),
            "==" => (BinaryOp::Equal, true),
            "!=" => (BinaryOp::NotEqual, false),
            "!==" => (BinaryOp::NotEqual, true),
            "<>" => (BinaryOp::NotEqual, false),
            "<" => (BinaryOp::Less, false),
            ">" => (BinaryOp::Greater, false),
            "<=" => (BinaryOp::LessEqual, false),
            ">=" => (BinaryOp::GreaterEqual, false),
            _ => return Ok(None),
        };
        // The name and the comparison are read, and the text of the right side
        // is the rest of the line.
        self.next();
        self.next();
        let start = self.position;
        let mut end = start;
        while end < self.tokens.len() && self.tokens[end].kind != TokenKind::Newline {
            end += 1;
        }
        let mut text = if end == start {
            String::new()
        } else {
            self.the_verbatim_source(start, end)
        };
        if the_text_begins_with_an_equal_sign {
            // The second equal sign is a part of the text.
            text = format!("={text}");
        }
        let text = text.trim().to_string();
        // The newline of the line stays where it is: the body of the legacy
        // `if` is the line that comes.
        self.position = end;
        Ok(Some(Node::LegacyComparison {
            left: Box::new(Node::Var {
                name: name_of_the_variable,
                line: line_of_the_variable,
            }),
            op,
            text,
            line: line_of_the_variable,
        }))
    }

    /// The form of the legacy `if` of the v1 that is not a comparison: the
    /// `between`, the `in` and the `contains`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
    ///
    /// - The form is the legacy one when the name of a variable comes
    ///   first and the word of the form comes after it: the `if x in a, b`
    ///   is the legacy form, and the `if x.y in a, b` is not.
    /// - The `not` may come before the word, and it turns the answer
    ///   around: the `if x not in a, b` is the same form with the other
    ///   answer.
    /// - The words are the words of the dialect and not the case of the
    ///   script: the `if x IN a, b` is the same form.
    /// - The texts of the form are the texts of the script and not
    ///   expressions, and they go through the legacy substitution: the
    ///   `if x in %y%` asks whether the value of `x` is the value of `y`.
    /// - The list of the `in` and of the `contains` is cut at the commas,
    ///   and the spaces around them are a part of the item: the `if x in
    ///   a, b` asks whether the value of `x` is "a" or " b".
    /// - Two commas in a row are one comma in the item, and the item goes
    ///   on: the `if x in a,,b` asks whether the value of `x` is "a,b".
    fn the_form_of_a_legacy_if(
        &mut self,
        name_of_the_variable: String,
        line_of_the_variable: u32,
    ) -> Result<Option<Node>, SyntaxError> {
        // The word of the form, and the `not` that may come before it.
        let mut at = self.position + 1;
        let mut negated = false;
        let Some(token) = self.tokens.get(at) else {
            return Ok(None);
        };
        if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("not") {
            negated = true;
            at += 1;
        }
        let Some(token) = self.tokens.get(at) else {
            return Ok(None);
        };
        if !matches!(token.kind, TokenKind::Name | TokenKind::Keyword) {
            return Ok(None);
        }
        let word = token.value.to_ascii_lowercase();
        let form = match word.as_str() {
            "between" => LegacyForm::Between,
            "in" => LegacyForm::In,
            "contains" => LegacyForm::Contains,
            _ => return Ok(None),
        };
        // The text of the form is the text that comes after the word, and
        // it is the text of the script and not an expression.
        let start = at + 1;
        let mut end = start;
        while end < self.tokens.len() && self.tokens[end].kind != TokenKind::Newline {
            end += 1;
        }
        let text = if end == start {
            String::new()
        } else {
            self.the_verbatim_source(start, end)
        };
        // The texts of the form.
        let texts = match form {
            LegacyForm::Between => {
                // The text is cut at the word "and", and the two bounds
                // are the texts that come around it.
                let Some((low, high)) = the_bounds_of_a_between(&text) else {
                    return Err(self.the_error_of_the_between(
                        line_of_the_variable,
                        at - usize::from(negated),
                        &text,
                    ));
                };
                vec![low, high]
            }
            LegacyForm::In | LegacyForm::Contains => {
                // The list is the text that comes after the word, and it
                // is not there when it is empty.
                if text.trim().is_empty() {
                    let name_of_the_form = if negated {
                        format!("not {word}")
                    } else {
                        word.clone()
                    };
                    return Err(self.the_error_of_the_list(
                        line_of_the_variable,
                        at - usize::from(negated),
                        &name_of_the_form,
                    ));
                }
                the_items_of_a_list(text.trim())
            }
        };
        // The newline of the line stays where it is: the body of the legacy
        // `if` is the line that comes.
        self.position = end;
        Ok(Some(Node::LegacyMatch {
            left: Box::new(Node::Var {
                name: name_of_the_variable,
                line: line_of_the_variable,
            }),
            form,
            texts,
            negated,
            line: line_of_the_variable,
        }))
    }

    /// The refusal of the `between` of the legacy `if` when the word `and`
    /// is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// complaint names the word that is missing, and the line that follows
    /// is the line of the condition with the word of the form replaced by
    /// a comma and the bound at the end of it.
    fn the_error_of_the_between(&self, line: u32, word_at: usize, text: &str) -> SyntaxError {
        let before = self.the_source_before_the_word(line, word_at);
        let low = text.trim();
        let whole = the_source_of_the_line(self, line);
        let width = whole
            .chars()
            .count()
            .saturating_sub(before.chars().count() + 1);
        let named = if low.is_empty() {
            format!("{before},")
        } else {
            format!("{before},{:>width$}", low, width = width)
        };
        self.error(format!(
            "BETWEEN requires the word AND.\n     Specifically: {named}"
        ))
    }

    /// The refusal of the list of the legacy `if` when it is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// complaint names the word of the form, and the line that follows is
    /// the line of the condition up to the word.
    fn the_error_of_the_list(&self, line: u32, word_at: usize, word: &str) -> SyntaxError {
        let before = self.the_source_before_the_word(line, word_at);
        self.error(format!(
            "\"{word}\" requires at least 2 parameters.\n     Specifically: {}",
            before.trim_end()
        ))
    }

    /// The source of the line of the condition up to the word of the form.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text is the text of the script, and the spaces that come before the
    /// word are a part of it.
    fn the_source_before_the_word(&self, line: u32, word_at: usize) -> String {
        let start = self
            .tokens
            .iter()
            .position(|token| token.line == line)
            .unwrap_or(0);
        let Some(first) = self.tokens.get(start) else {
            return String::new();
        };
        let Some(word) = self.tokens.get(word_at) else {
            return String::new();
        };
        let Some(source) = self.lines.get(first.line as usize - 1) else {
            return String::new();
        };
        let chars: Vec<char> = source.chars().collect();
        let from = first.column.saturating_sub(1) as usize;
        let to = word.column.saturating_sub(1) as usize;
        if from >= to || to > chars.len() {
            return String::new();
        }
        // The spaces that come before the word are a part of the text.
        chars[from..to].iter().collect()
    }

    /// Whether the text that ends at `end` ends with an operator.
    ///
    /// The last token of the text is the one that is looked at, and the
    /// operator is the one that makes the expression of the v1 continue on
    /// the line that comes.
    fn the_text_ends_with_an_operator(&self, from: usize, end: usize) -> bool {
        let mut at = end;
        while at > from {
            at -= 1;
            let token = &self.tokens[at];
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                continue;
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the assignment of the v1 is an operator for the tail: the `if
            // (0) x := 1, y :=` takes the line that comes with it.
            if self.dialect.the_assignment_continues_the_tail()
                && token.kind == TokenKind::Op
                && token.value == ":="
            {
                return true;
            }
            return the_token_is_an_operator(token);
        }
        false
    }

    /// The condition of an `if`, with the control flow that may come inside
    /// the line of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `if` and the `while` may come inside the line of the condition of an
    /// `if`: the `if (0) if (1) x := 1` is accepted.
    fn the_condition_of_an_if(&mut self) -> Result<Node, SyntaxError> {
        let was = self.in_the_condition_of_an_if;
        self.in_the_condition_of_an_if = true;
        let condition = self.the_expression_of_a_statement();
        self.in_the_condition_of_an_if = was;
        condition
    }

    /// The condition of a construct, when the failure of it is not reported.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not report the failure of the condition of an `if` or of a
    /// `while`: the `if := 5` and the `if 1 := 5` are accepted, the
    /// condition is false, and the body of the construct does not run. The
    /// failure of a bracket that is not closed is reported: the `if (x` is
    /// refused with "Missing ")"", and the `if [1` with "Missing "]"".
    fn the_condition_that_is_not_reported(
        &mut self,
        line: u32,
        condition: Result<Node, SyntaxError>,
    ) -> Result<Node, SyntaxError> {
        match condition {
            Ok(condition) => Ok(condition),
            Err(error) => {
                if !self.dialect.the_failure_of_a_condition_is_not_reported()
                    || error.message.starts_with("Missing \"")
                {
                    return Err(error);
                }
                // The line of the condition is over, and the body of the
                // construct is the line that comes.
                self.position = the_end_of_the_line_of(self, self.position);
                Ok(Node::Literal {
                    value: LiteralValue::Integer(0),
                    line,
                })
            }
        }
    }

    /// The `if` or the `while` that comes inside the line of the condition of
    /// an `if`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// construct has no body of its own: its condition is a part of the
    /// condition that holds it, and the line that comes after is the body of
    /// the `if` that holds it. The v1 refuses the brace that comes after it
    /// with "Unexpected \"{\"", and the tree refuses the line with
    /// "Unexpected \"If\"".
    fn parse_the_control_of_a_condition(&mut self, token: Token) -> Result<Node, SyntaxError> {
        let condition = self.the_expression_of_a_statement()?;
        Ok(Node::LegacyControl {
            condition: Box::new(condition),
            line: token.line,
        })
    }

    fn parse_if(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let mut branches = Vec::new();
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `if` needs
            // a condition, and it says so in its own words.
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `if` of the v1 has no name of its own, and the complaint of
            // it is '"" requires at least 1 parameter. Specifically: if'.
            return Err(self.error(format!(
                "\"{}\" requires at least 1 parameter.\n     Specifically: {}",
                self.dialect
                    .the_name_of_the_word_that_wants_a_parameter("if"),
                the_source_of_the_line(self, token.line)
            )));
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // legacy `if` of the v1 compares a variable with the text that comes
        // after the operator, and the text of the rest of the line is the
        // condition of it.
        let legacy = if self.dialect.the_if_may_be_a_legacy_comparison() {
            self.the_condition_of_a_legacy_if()?
        } else {
            None
        };
        // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the
        // body of the construct before it reads the condition, and the
        // refusal of a condition that is empty comes after the one of the
        // body.
        let empty = if legacy.is_some() {
            None
        } else {
            self.the_parenthesis_of_a_condition_is_empty()
        };
        // The legacy condition is not moved here: the tail of the `if` is
        // read after it, and the tail of the legacy `if` is not read.
        let the_legacy = legacy.is_some();
        let condition = match legacy {
            Some(condition) => Some(condition),
            None => {
                if empty.is_some() {
                    self.next();
                    self.next();
                    None
                } else {
                    let condition = self.the_condition_of_an_if();
                    Some(self.the_condition_that_is_not_reported(token.line, condition)?)
                }
            }
        };
        self.end_of_block()?;
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `if` of the v1 takes the text that follows the condition, parses it
        // and does not run it, and the body of it is the line that comes
        // next. The legacy `if` has no tail: the text of the rest of the
        // line is the condition of it.
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
        // the tail of the `if` of the v1 takes the line that comes with it
        // when the text of it ends with an operator, and the `else` that
        // comes after it is the other side of the `if`: the body of the
        // `else` runs whether the condition is true or false.
        let (the_tail_took_a_line, the_items_of_the_tail) =
            if !the_legacy && self.dialect.the_if_may_carry_a_tail() {
                self.the_tail_of_the_if_of_the_v1()?
            } else {
                (false, Vec::new())
            };
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
        // the tail of the `if` of the v1 takes the line that comes with it
        // when the text of it ends with an operator, and the `if` has no
        // body of its own then: the statement that comes after the tail is
        // a statement of its own, and it runs whether the condition is
        // true or false.
        let body = if the_tail_took_a_line {
            Vec::new()
        } else {
            self.parse_block("If", token.line)?
        };
        // The `else` that comes right after the tail is the other side of
        // the `if`, and the body of it runs whether the condition is true
        // or false.
        let the_else_is_the_other_side = the_tail_took_a_line;
        let Some(condition) = condition else {
            return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
        };
        branches.push(Branch {
            condition,
            tail: the_items_of_the_tail,
            body,
        });
        let mut otherwise = Vec::new();
        // Whether the `else` is a body and not the branch of an `else if`.
        let mut the_else_is_a_body = false;
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
                    // The legacy `if` of the v1 is the legacy one here too:
                    // the condition of it is the text of the rest of the
                    // line.
                    let legacy = if self.dialect.the_if_may_be_a_legacy_comparison() {
                        self.the_condition_of_a_legacy_if()?
                    } else {
                        None
                    };
                    let empty = if legacy.is_some() {
                        None
                    } else {
                        self.the_parenthesis_of_a_condition_is_empty()
                    };
                    let condition =
                        match legacy {
                            Some(condition) => Some(condition),
                            None => {
                                if empty.is_some() {
                                    self.next();
                                    self.next();
                                    None
                                } else {
                                    let condition = self.the_condition_of_an_if();
                                    Some(self.the_condition_that_is_not_reported(
                                        branch.line,
                                        condition,
                                    )?)
                                }
                            }
                        };
                    self.end_of_block()?;
                    let body = self.parse_block("If", branch.line)?;
                    let Some(condition) = condition else {
                        return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
                    };
                    branches.push(Branch {
                        condition,
                        tail: Vec::new(),
                        body,
                    });
                    continue;
                }
                otherwise = self.parse_block("Else", keyword.line)?;
                the_else_is_a_body = true;
            }
            break;
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
        // the body of the `else` runs whether the condition is true or
        // false, and the tree puts it in the two sides of the `if` so
        // that it runs in both of them.
        if the_else_is_the_other_side && the_else_is_a_body {
            if let Some(branch) = branches.first_mut() {
                branch.body = otherwise.clone();
            }
        }
        Ok(Node::If {
            branches,
            otherwise,
            line: token.line,
        })
    }

    /// Whether the `if` of an `else if` comes next.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the branch of an `if`
    /// is written with two words, and the `if` may be on the line of the
    /// `else` or on the one after it.
    fn the_if_of_an_else_if_comes_next(&self) -> bool {
        let mut at = self.position;
        while self
            .tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
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
            let message =
                if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("case") {
                    "Unexpected \"Case\"".to_string()
                } else {
                    "Unexpected \"Switch\"".to_string()
                };
            return Err(SyntaxError {
                message,
                line: token.line,
                column: token.column,
            });
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
                cases.push(Case {
                    values: Vec::new(),
                    body,
                });
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
        Ok(Node::Switch {
            subject: Box::new(subject),
            cases,
            line: token.line,
        })
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
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 reads the parameter of the return before it refuses the return,
        // and so the name that is not legal is the first refusal.
        if !self.dialect.the_parameter_of_the_return_is_read_first()
            && !self.at_end_of_statement()
            && !self.in_function
        {
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
        if self.dialect.the_parameter_of_the_return_is_read_first()
            && value.is_some()
            && !self.in_function
        {
            return Err(SyntaxError {
                message: "Return's parameter should be blank except inside a function.".to_string(),
                line: token.line,
                column: token.column,
            });
        }
        self.end_of_statement()?;
        Ok(Node::Return {
            value,
            line: token.line,
        })
    }

    fn parse_exitapp(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        let code = if self.at_end_of_statement() {
            None
        } else {
            Some(Box::new(self.the_expression_of_a_statement()?))
        };
        self.end_of_statement()?;
        Ok(Node::ExitApp {
            code,
            line: token.line,
        })
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
        Ok(Node::Try {
            body,
            catches,
            otherwise,
            finally,
            line: token.line,
        })
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
        Ok(Catch {
            class,
            variable,
            body,
        })
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
                .map(|parameters| {
                    parameters
                        .iter()
                        .any(|parameter| parameter.eq_ignore_ascii_case(&name.value))
                })
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
                if let Some(message) = self
                    .dialect
                    .the_refusal_of_a_global_of_a_built_in_variable(&name.value)
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
            names.push(Declared {
                name: name.value,
                value,
                text,
            });
            if !self.at_op(&[","]) {
                break;
            }
            self.next();
        }
        // The `global` alone is the mode of a function, and the `local`
        // alone is a declaration of no name.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the `global` alone at the top level of the script, and
        // the line holds no action.
        //
        // The v2 accepts it at the top level of the script.
        if kind == DeclareKind::Global
            && names.is_empty()
            && self.parameters.is_empty()
            && !self.dialect.the_bare_global_is_the_mode_of_the_script()
        {
            return Err(self.the_error_of_the_refused_declaration(kind, &names));
        }
        // A bare `local` is never a thing: only `global` has a mode, and
        // the bare `static` is a declaration with no name.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the bare
        // `static` inside a function is accepted, and the bare `local` is
        // refused.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 accepts the `local` alone inside a function, and it refuses it
        // at the top level of the script, where the line holds no action.
        let the_bare_local_is_accepted = self.dialect.the_bare_local_is_a_declaration_of_no_name()
            && !self.parameters.is_empty();
        if kind == DeclareKind::Local && names.is_empty() && !the_bare_local_is_accepted {
            return Err(self.the_error_of_the_refused_declaration(kind, &names));
        }
        // A `local` outside a function has no scope to live in, and a
        // `static` has no function to live in.
        //
        // Measured on the binary of v2.0.19 on 2026-10-31: the `static` at
        // the top level is refused, and the text that is named is the whole
        // declaration.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the same declarations, and it says that the line holds
        // no action.
        if (kind == DeclareKind::Local || kind == DeclareKind::Static) && self.parameters.is_empty()
        {
            return Err(self.the_error_of_the_refused_declaration(kind, &names));
        }
        self.end_of_statement()?;
        Ok(Node::Declare {
            scope: kind,
            names,
            line: token.line,
        })
    }

    /// The error of a declaration that the dialect refuses.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says that the line holds no action, and it names the declaration: the
    /// "global", the "local", the "local x" and the "local x := 1".
    ///
    /// The v2 says "Unexpected declaration." and names the same text.
    fn the_error_of_the_refused_declaration(
        &self,
        kind: DeclareKind,
        names: &[Declared],
    ) -> SyntaxError {
        let text = the_text_of_a_declaration(kind, names);
        self.error(self.dialect.the_message_of_the_refused_declaration(&text))
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
        let Some(colon) = line.last() else {
            return Ok(None);
        };
        if colon.kind != TokenKind::Op || colon.value != ":" {
            // The line may hold a label with something else written after
            // it: `done: FileAppend "x", "*"` holds no action.
            let writes_more = match line.get(1) {
                Some(second) => {
                    second.kind == TokenKind::Op && second.value == ":" && !second.spaced
                }
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
                // Measured on the binary of v1.1.37.02 on 2026-11-03: the
                // label that a command names may be written anywhere in the
                // script, and the name of a label does not know its case.
                self.label_names.insert(name.to_lowercase());
                Ok(Some(Node::Label {
                    name,
                    line: token.line,
                }))
            }
            None => {
                // A name with a dash or with a dot is not a label, and the
                // line that holds it holds no action. A name with a space
                // is an expression that the parser reads the way it reads
                // every other one.
                if line[..line.len() - 1]
                    .iter()
                    .skip(1)
                    .any(|token| token.spaced)
                {
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
                "\"{}\" requires at least 1 parameter.\n     Specifically: {text}",
                self.dialect
                    .the_name_of_the_word_that_wants_a_parameter("goto")
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
        Ok(Node::Goto {
            name,
            line: token.line,
        })
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
        //
        // The v1 refuses the jump at the end of the load, and the complaint
        // of it is the one of the v1.
        if !self.dialect.the_goto_is_refused_with_the_labels() {
            return Ok(());
        }
        for (name, line) in the_gotos_of_a_scope(body) {
            if !seen.contains(&name.to_lowercase()) {
                return Err(SyntaxError {
                    message: format!(
                        "Label not found in current scope.\n     Specifically: {name}"
                    ),
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

    /// Whether the statement begins with a member whose name is not a name.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-02: the name
    /// of a member may not be a text, and the single quote is not a text in
    /// the v1: the `a.'b' := 5` and the `a."b" := 5` hold no action, and the
    /// whole line is named.
    ///
    /// The name that is a number is a name of the member: the `a.2` is the
    /// member "2" of the value of `a`.
    fn the_statement_begins_with_a_member_whose_name_is_not_a_name(&self) -> bool {
        let Some(dot) = self.tokens.get(self.position + 1) else {
            return false;
        };
        if dot.kind != TokenKind::Op || dot.value != "." || dot.spaced {
            return false;
        }
        let Some(name) = self.tokens.get(self.position + 2) else {
            return false;
        };
        match name.kind {
            TokenKind::String | TokenKind::UnclosedText => true,
            TokenKind::Name => {
                self.dialect.the_single_quote_is_not_a_text() && name.value.starts_with('\'')
            }
            _ => false,
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
    fn the_loop_of_the_break_or_the_continue(
        &self,
        line: u32,
        column: u32,
    ) -> Result<(), SyntaxError> {
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
    fn the_body_of_a_scope(
        &mut self,
        kind: &str,
        line: u32,
    ) -> Result<(Vec<Node>, u32), SyntaxError> {
        let loops = self.loops;
        self.loops = 0;
        // The line of the brace that opens the body. Measured on the binary of
        // AutoHotkey v1.1.37.02 on 2026-11-05: the v1 names this line when it
        // warns about a local that has the name of a global one, and the
        // brace may be on a line of its own.
        let body_line = self.the_line_of_the_brace_that_opens_the_body();
        let body = self.parse_block(kind, line);
        self.loops = loops;
        Ok((body?, body_line))
    }

    /// The line of the brace that opens the body of a scope.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// names the line of the brace when it warns about a local that has the
    /// name of a global one.
    fn the_line_of_the_brace_that_opens_the_body(&self) -> u32 {
        let mut at = self.position;
        while let Some(token) = self.tokens.get(at) {
            if token.kind != TokenKind::Newline {
                return token.line;
            }
            at += 1;
        }
        self.peek().line
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
        Ok(vec![Node::Return {
            value: Some(Box::new(value)),
            line,
        }])
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
        Ok(Node::Throw {
            value,
            line: token.line,
        })
    }

    fn parse_while(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // condition of the `while` of the v1 comes after a comma: `While, 1`.
        // The v2 has no comma there.
        if self.dialect.legacy_commands() && self.at_op(&[","]) {
            self.next();
        }
        if self.at_end_of_statement() {
            // Measured on the binary of v2.0.19 on 2026-10-31: the `while`
            // needs a condition, and it says so in its own words.
            return Err(self.error(format!(
                "\"{}\" requires at least 1 parameter.\n     Specifically: {}",
                self.dialect
                    .the_name_of_the_word_that_wants_a_parameter("while"),
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
            let condition = self.the_expression_of_a_statement();
            Some(self.the_condition_that_is_not_reported(token.line, condition)?)
        };
        self.end_of_block()?;
        let body = self.the_body_of_a_loop("While", token.line)?;
        let Some(condition) = condition else {
            return Err(self.the_refusal_of_an_empty_parenthesis(&empty.unwrap()));
        };
        Ok(Node::While {
            condition: Box::new(condition),
            body,
            line: token.line,
        })
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
        let the_name_is_a_word = self
            .dialect
            .the_name_of_a_class_may_be_a_word_of_the_language()
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
            return Err(SyntaxError {
                message,
                line: name_token.line,
                column: name_token.column,
            });
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
                        the_source_of_a_token(self, &base)
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
            self.bases.push((
                name.clone(),
                base.clone(),
                name_token.line,
                name_token.column,
            ));
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
    fn parse_class_body(
        &mut self,
        line: u32,
        name: &str,
        name_at: usize,
    ) -> Result<TheMembersOfAClass, SyntaxError> {
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
            if name_token.kind == TokenKind::Keyword
                && name_token.value.eq_ignore_ascii_case("class")
            {
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
                    let message = if self
                        .dialect
                        .the_class_that_is_declared_twice_has_its_own_words()
                    {
                        format!(
                            "Duplicate class definition.\n     Specifically: {}",
                            nested_name.value
                        )
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
                let class =
                    self.parse_the_class_with_the_name(name_token, nested_name, Some(name))?;
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
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {name}.__Init"
                        ),
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
                    let owner = if self
                        .dialect
                        .the_duplicate_names_the_member_that_was_written()
                    {
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
                let taken = if is_static {
                    &mut class_methods
                } else {
                    &mut instance_methods
                };
                let key = name_token.value.to_lowercase();
                if taken.contains(&key) {
                    let owner = if self
                        .dialect
                        .the_duplicate_names_the_member_that_was_written()
                    {
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
                self.parameters.push(
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect(),
                );
                self.assignments.push(Vec::new());
                // A method may not contain a class either.
                let was_in_a_function = self.in_function;
                self.in_function = true;
                // Measured on the binary of v2.0.19 on 2026-11-01: the body of
                // a method may be an expression too, and the fat arrow is
                // what says so.
                let names_the_class = self.dialect.the_class_that_is_not_closed_names_the_class();
                let (body, body_line) = if self.the_fat_arrow_comes_next() {
                    let body = self.the_body_of_a_fat_arrow(name_token.line)?;
                    (body, name_token.line)
                } else {
                    self.the_body_of_a_scope("method", name_token.line)
                        .map_err(|mut error| {
                            // Measured on the binary of AutoHotkey v1.1.37.02 on
                            // 2026-11-02: the v1 names the class when the file
                            // ends inside a method of it.
                            if names_the_class && error.message.starts_with("Missing \"}\"") {
                                error.message =
                                    format!("Missing \"}}\"\n     Specifically: {name}");
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
                let assignments = core::mem::take(
                    self.assignments
                        .last_mut()
                        .expect("the list of the assignments"),
                );
                self.assignments.pop();
                self.parameters.pop();
                let declarations = the_declarations_of_a_body(&body, self.dialect)?;
                for assignment in assignments {
                    if declarations
                        .globals
                        .contains(&assignment.name.to_lowercase())
                    {
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
                    body_line,
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
                // The body of the property begins on the line that comes after
                // the name of it.
                //
                // Measured on the binary of v2.0.19 on 2026-11-05: the getter
                // that is on the same line as the name is refused with "Not a
                // valid method, class or property definition.\n     Specifically:
                // p { get => 1 }".
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: the v1 refuses it the same way.
                if !self.the_body_of_the_accessor_begins_on_the_next_line(name_token.line) {
                    return Err(SyntaxError {
                        message: format!(
                            "Not a valid method, class or property definition.\n     Specifically: {}",
                            self.the_text_of_the_member_that_is_refused(name_at, name_token.line)
                        ),
                        line: name_token.line,
                        column: name_token.column,
                    });
                }
                // The v1 has no static member, and the property that has a
                // getter is one of them.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: "Invalid class variable declaration.\n
                // Specifically: p {". The text is the member, from the name to
                // the brace that opens the body.
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
                let key = name_token.value.to_lowercase();
                let taken = if is_static {
                    &mut class_accessors
                } else {
                    &mut instance_accessors
                };
                if taken.contains(&key) {
                    return Err(SyntaxError {
                        message: format!(
                            "Duplicate declaration.\n     Specifically: {}",
                            name_token.value
                        ),
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
            let taken = if is_static {
                &mut class_properties
            } else {
                &mut instance_properties
            };
            let key = name_token.value.to_lowercase();
            if taken.contains(&key) {
                return Err(SyntaxError {
                    message: format!(
                        "Duplicate declaration.\n     Specifically: {}",
                        name_token.value
                    ),
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
        Ok(TheMembersOfAClass {
            methods,
            properties,
            accessors,
            nested,
        })
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
        if self
            .tokens
            .get(end)
            .map(|token| token.value == "{")
            .unwrap_or(false)
        {
            format!("{text} {{")
        } else {
            text
        }
    }

    /// Whether the body of the property begins on the line that comes after
    /// the name of it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the getter that is on
    /// the same line as the name is refused, and so is the body of the getter
    /// that is on the line of the member.
    fn the_body_of_the_accessor_begins_on_the_next_line(&self, line: u32) -> bool {
        // The brace that opens the body, and the token that comes after it:
        // the newline that ends the line of the name is not a part of it.
        let Some(open) = self.the_brace_that_opens_the_body_of_the_accessor() else {
            return true;
        };
        let mut after = open + 1;
        while self
            .tokens
            .get(after)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
            after += 1;
        }
        match self.tokens.get(after) {
            Some(token) => token.line != line,
            None => true,
        }
    }

    /// The place of the brace that opens the body of the property.
    fn the_brace_that_opens_the_body_of_the_accessor(&self) -> Option<usize> {
        let mut at = self.position;
        while let Some(token) = self.tokens.get(at) {
            if token.kind == TokenKind::Op && token.value == "{" {
                return Some(at);
            }
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                return None;
            }
            at += 1;
        }
        None
    }

    /// The text that is named when the member is refused.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the member is named
    /// whole, and the newlines of it are spaces: "p { get { return 1 } }".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// names the line of the member: "p { get {".
    fn the_text_of_the_member_that_is_refused(&self, name_at: usize, line: u32) -> String {
        if !self.dialect.the_member_that_is_refused_is_named_whole() {
            return the_source_of_the_line(self, line);
        }
        let Some(open) = self.the_brace_that_opens_the_body_of_the_accessor() else {
            return the_source_of_the_line(self, line);
        };
        let Some(close) = self.the_closing_brace_of_a_block_that_opens_a_line(open) else {
            return the_source_of_the_line(self, line);
        };
        the_source_of_a_definition(self, name_at, close + 1)
    }

    /// A property of the class that has a getter or a setter.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the property may
    /// take parameters, and the body of the getter and of the setter is an
    /// expression after the arrow of v2, and a block of statements between
    /// braces.
    fn parse_the_accessor(
        &mut self,
        name_token: Token,
        is_static: bool,
    ) -> Result<AccessDef, SyntaxError> {
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
            let body = self.parse_the_body_of_an_accessor(&name_token.value, &token)?;
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
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the body that is a
    /// block is a scope of its own: the `return` of it is the value of the
    /// property, and the name that it writes is a local of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// has only the block, and the expression after the arrow is refused with
    /// "Not a valid property getter/setter.\n     Specifically: get => 1".
    fn parse_the_body_of_an_accessor(
        &mut self,
        property: &str,
        member: &Token,
    ) -> Result<AccessBody, SyntaxError> {
        let line = member.line;
        let (body, declarations, body_line) = if self.at_op(&["=>"]) {
            // The v1 does not have the expression after the arrow.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the v1 refuses it with "Not a valid property
            // getter/setter.\n     Specifically: get => 1".
            if !self.dialect.the_accessor_takes_an_expression() {
                return Err(self.error(format!(
                    "Not a valid property getter/setter.\n     Specifically: {}",
                    the_source_of_the_line(self, line)
                )));
            }
            let token = self.next();
            let expression = self.parse_expression()?;
            let body = if self.at_an_assignment_operator() {
                vec![self.parse_the_assignment(expression)?]
            } else {
                vec![Node::Return {
                    value: Some(Box::new(expression)),
                    line: token.line,
                }]
            };
            let declarations = the_declarations_of_a_body(&body, self.dialect)?;
            (body, declarations, line)
        } else if self.at_op(&["{"]) {
            // The body begins on the line that comes after the one of the
            // member.
            //
            // Measured on the binary of v2.0.19 on 2026-11-05: the body of the
            // getter that is on the line of the member is refused with "Not a
            // valid property getter/setter.\n     Specifically: get { return 1
            // }".
            if !self.the_body_of_the_accessor_begins_on_the_next_line(line) {
                return Err(self.error(format!(
                    "Not a valid property getter/setter.\n     Specifically: {}",
                    the_source_of_the_line(self, line)
                )));
            }
            // The body of the getter is an âmbito de si: o `return` dele é o
            // valor da propriedade, e o nome que ele escreve é um local dele.
            //
            // Measured on the binary of v2.0.19 on 2026-11-05: o `return` do
            // corpo do getter responde o valor da propriedade.
            let was_in_a_function = self.in_function;
            self.in_function = true;
            self.parameters.push(Vec::new());
            self.assignments.push(Vec::new());
            let outcome = self.the_body_of_a_scope("accessor", line);
            self.in_function = was_in_a_function;
            let assignments = core::mem::take(
                self.assignments
                    .last_mut()
                    .expect("the list of the assignments"),
            );
            self.assignments.pop();
            self.parameters.pop();
            let (body, body_line) = outcome?;
            let declarations = the_declarations_of_a_body(&body, self.dialect)?;
            // The assignments of the body that live in the scope of the script
            // are the ones of the names that it declares global.
            for assignment in assignments {
                if declarations
                    .globals
                    .contains(&assignment.name.to_lowercase())
                {
                    self.check_the_assignments(vec![assignment])?;
                }
            }
            (body, declarations, body_line)
        } else {
            // The member that has no body.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the v1 reads it as a method of the property, and it refuses it
            // with "Missing "{".\n     Specifically: p.get()".
            //
            // Measured on the binary of v2.0.19 on 2026-11-05: the v2 refuses
            // it with "Not a valid property getter/setter.\n     Specifically:
            // get".
            if self
                .dialect
                .the_member_of_an_accessor_without_a_body_is_a_call()
            {
                return Err(SyntaxError {
                    message: format!(
                        "Missing \"{{\"\n     Specifically: {}.{}()",
                        property, member.value
                    ),
                    line,
                    column: member.column,
                });
            }
            return Err(self.error(format!(
                "Not a valid property getter/setter.\n     Specifically: {}",
                the_source_of_the_line(self, line)
            )));
        };
        Ok(AccessBody {
            body,
            declarations,
            line: body_line,
        })
    }

    /// A directive: the name, and the rest of the line.
    fn parse_directive(&mut self) -> Result<Node, SyntaxError> {
        let token = self.next();
        // The argument is the text that follows the name, and the lexer
        // put it in the next token.
        let argument = if self.peek().kind == TokenKind::String
            || self.peek().kind == TokenKind::UnclosedText
        {
            self.next().value
        } else {
            String::new()
        };
        self.end_of_statement()?;
        Ok(Node::Directive {
            name: token.value,
            argument,
            line: token.line,
        })
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
        let mut kind = self.the_kind_of_the_loop();
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
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-04: the brace that opens the body of the
                        // loop of the v1 is the one that is at the end of
                        // the line: the `Loop, {` is a loop that goes on
                        // until a break.
                        //
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the parameters that come after the
                        // first one are the mode of the loop of the files
                        // and the flag of the recursion, and the v1 reads
                        // them as texts.
                        let texts =
                            self.the_parameters_of_the_loop_of_the_v1(token.line, &mut arguments)?;
                        // The kind of the loop, and the checks of the
                        // parameters of it.
                        kind = self.the_kind_of_the_loop_of_the_v1(&texts)?;
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
                    } else if !self.at_end_of_statement()
                        && !self.at_op(&["::"])
                        && !self.the_brace_opens_the_body_of_the_loop()
                    {
                        if self.dialect.legacy_commands() {
                            // Measured on the binary of AutoHotkey v1.1.37.02 on
                            // 2026-11-05: the parameters of the loop of the v1
                            // are the count of the turns or the pattern of the
                            // files, and the ones that come after it.
                            let texts = self
                                .the_parameters_of_the_loop_of_the_v1(token.line, &mut arguments)?;
                            kind = self.the_kind_of_the_loop_of_the_v1(&texts)?;
                        } else {
                            self.the_expression_begins();
                            arguments.push(self.the_count_of_the_loop(token.line)?);
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
        }
        self.the_expression_is_done(self.position, false)?;
        self.end_of_block()?;
        let body_start = self.position;
        let body = self.the_body_of_a_loop(&the_name_of_the_loop(&name), token.line)?;
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `until` of the v1 is the statement that comes right after the body
        // of the loop, and it is the condition that ends it.
        let until = self.the_until_of_the_loop()?;
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
        Ok(Node::Loop {
            mode: kind.unwrap_or(LoopKind::Count),
            arguments,
            body,
            until,
            line: token.line,
        })
    }

    /// The `until` that is the condition of the loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `until` of the v1 is the statement that comes right after the body of
    /// a loop, and it is the condition that ends it: the body runs, and the
    /// condition is read after every turn of it. The `until` that is the name
    /// of a variable is not the condition, and the one that is not right
    /// after a body is not one either: the v1 refuses it with "UNTIL with no
    /// matching LOOP" when the load is over.
    ///
    /// The v2 has no such statement, and the loop has no condition.
    fn the_until_of_the_loop(&mut self) -> Result<Option<Box<Node>>, SyntaxError> {
        if self
            .dialect
            .the_refusal_of_the_until_that_has_no_loop()
            .is_none()
        {
            return Ok(None);
        }
        // The word that comes next, past the newlines.
        let mut at = self.position;
        while self
            .tokens
            .get(at)
            .map(|token| token.kind == TokenKind::Newline)
            .unwrap_or(false)
        {
            at += 1;
        }
        let Some(token) = self.tokens.get(at) else {
            return Ok(None);
        };
        if token.kind != TokenKind::Keyword || !token.value.eq_ignore_ascii_case("until") {
            return Ok(None);
        }
        // The word that is the name of a variable is not the condition: the
        // `until := 2` writes the variable "until".
        let is_a_name = self
            .tokens
            .get(at + 1)
            .map(|next| {
                next.kind == TokenKind::Op && the_assignment_operator(&next.value).is_some()
            })
            .unwrap_or(false);
        if is_a_name {
            return Ok(None);
        }
        self.position = at;
        self.next();
        let condition = self.the_expression_of_a_statement()?;
        Ok(Some(Box::new(condition)))
    }

    /// Whether the brace at the place of the parser is the last thing of
    /// its line.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// brace that opens the body of the loop of the v1 is the one that is
    /// at the end of the line of it: the `loop {` opens the body, and the
    /// `loop { break }` does not.
    fn the_brace_is_at_the_end_of_the_line(&self) -> bool {
        let line = self.peek().line;
        match self.tokens.get(self.position + 1) {
            Some(next) => {
                next.kind == TokenKind::Newline || next.kind == TokenKind::Eof || next.line != line
            }
            None => true,
        }
    }

    /// The place of the end of the line of the parser.
    ///
    /// The newline is the end of the line, and the end of the script is
    /// the end of the last line of it.
    fn the_end_of_the_line(&self) -> usize {
        let mut at = self.position;
        while let Some(token) = self.tokens.get(at) {
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                break;
            }
            at += 1;
        }
        at
    }

    /// Whether the brace at the place of the parser opens the body of the
    /// loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// brace that opens the body of the loop of the v1 is the one that is
    /// at the end of the line of it. The text that follows it on the line
    /// is the text of the count: the `loop { break }` is a loop of files
    /// whose pattern is the text "{ break }", and it has no turn.
    ///
    /// The v2 reads the body of the loop on the line of it, and the brace
    /// that is not at the end of the line is refused.
    fn the_brace_opens_the_body_of_the_loop(&self) -> bool {
        if !self.at_op(&["{"]) {
            return false;
        }
        if !self
            .dialect
            .the_brace_of_the_body_of_the_loop_is_at_the_end_of_the_line()
        {
            return true;
        }
        self.the_brace_is_at_the_end_of_the_line()
    }

    /// The count of the loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// count of the `Loop` of the v1 is a number and not an expression: the
    /// text of the count is what the script wrote, and the loop is skipped
    /// when the text is not a number. The `%` and the space force the
    /// expression: the `loop % Count + 1`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text of the count goes to the end of the line, and the brace that
    /// opens the body is the one that is at the end of it. The `loop {
    /// break }` is a loop of files whose pattern is the text "{ break }",
    /// and it has no turn.
    ///
    /// The v2 reads the count as an expression.
    fn the_count_of_the_loop(&mut self, line: u32) -> Result<Node, SyntaxError> {
        if !self.dialect.the_count_of_the_loop_is_a_number() {
            self.the_expression_begins();
            return self.parse_expression();
        }
        // The `%` and the space force the expression.
        if self.at_op(&["%"])
            && self
                .tokens
                .get(self.position + 1)
                .map(|token| token.spaced)
                .unwrap_or(false)
        {
            self.next();
            self.the_expression_begins();
            return self.parse_expression();
        }
        let start = self.position;
        // The brace that is not at the end of the line is a part of the
        // text of the count.
        if self.at_op(&["{"]) && !self.the_brace_is_at_the_end_of_the_line() {
            let end = self.the_end_of_the_line();
            let text = self.the_verbatim_source(start, end);
            self.position = end;
            return Ok(Node::Literal {
                value: LiteralValue::Text(text),
                line,
            });
        }
        // The expression is read to know where the count ends, and it is
        // thrown away: the v1 reads the text of it.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // text of the first parameter of the loop of the v1 is not an
        // expression: the `Loop, *.txt` is a loop of the files that match the
        // pattern, and the v1 does not refuse it.
        let saved = self.position;
        if self.parse_expression().is_err() {
            self.position = saved;
            let end = self.the_end_of_a_parameter(start);
            let text = self.the_verbatim_source(start, end);
            self.position = end;
            return Ok(Node::Literal {
                value: LiteralValue::Text(text),
                line,
            });
        }
        let mut end = self.position;
        // The brace that is not at the end of the line is a part of the
        // text of the count.
        if self.at_op(&["{"]) && !self.the_brace_is_at_the_end_of_the_line() {
            end = self.the_end_of_the_line();
            self.position = end;
        }
        let text = self.the_verbatim_source(start, end);
        Ok(Node::Literal {
            value: LiteralValue::Text(text),
            line,
        })
    }

    /// The kind of the loop, when the word that follows the `Loop` names
    /// one.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the word is a
    /// name of the kind when the line goes on after it, and it is the
    /// count of the loop when the line ends there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// has no kind of the loop that is written this way: the `Loop Files` of
    /// the v1 is a loop of the files that match the pattern "Files", and the
    /// `Loop Read` is the loop that reads a file.
    fn the_kind_of_the_loop(&self) -> Option<LoopKind> {
        if self.dialect.legacy_commands() {
            return None;
        }
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

    /// The parameters of the loop of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// parameters of the `Loop` of the v1 are texts, and they are cut at the
    /// commas: the comma that is inside the quotes is a cut too, and the
    /// quotes are part of the text. The `Loop, *.txt, 1,x` is the proof of it
    /// -- the v1 refuses it with "Parameter #3 invalid." and the "x" after
    /// it, and not with the mode of "1,x".
    fn the_parameters_of_the_loop_of_the_v1(
        &mut self,
        line: u32,
        arguments: &mut Vec<Node>,
    ) -> Result<Vec<String>, SyntaxError> {
        let mut texts: Vec<String> = Vec::new();
        if !self.the_brace_opens_the_body_of_the_loop() {
            let start = self.position;
            self.the_expression_begins();
            let node = self.the_count_of_the_loop(line)?;
            let end = self.position;
            match &node {
                Node::Literal {
                    value: LiteralValue::Text(_),
                    ..
                } => {
                    // The text of the first parameter: the count of the
                    // turns, the pattern of the files, or the name of the
                    // kind of the loop.
                    for piece in self.the_pieces_of_a_text_parameter(start, end) {
                        texts.push(piece.text.clone());
                        arguments.push(Node::Literal {
                            value: LiteralValue::Text(piece.text),
                            line,
                        });
                    }
                }
                _ => {
                    // The count that is an expression is a parameter of its
                    // own, and the v1 does not cut it.
                    texts.push(String::new());
                    arguments.push(node);
                }
            }
        } else {
            texts.push(String::new());
        }
        while self.at_op(&[","]) {
            self.next();
            let start = self.position;
            let end = self.the_end_of_a_parameter(start);
            let pieces = self.the_pieces_of_a_text_parameter(start, end);
            self.position = end;
            for piece in pieces {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-05: the second parameter of the parse loop may be
                // an expression: the percent sign and a space in front of it
                // force it, and the value of it is the text that is
                // separated.
                let is_the_text_of_the_parse_loop = texts.len() == 1
                    && texts
                        .first()
                        .map(|text| text.trim().eq_ignore_ascii_case("parse"))
                        .unwrap_or(false);
                let node = if is_the_text_of_the_parse_loop
                    && crate::v1::the_text_is_an_expression_of_the_v1(&piece.text)
                {
                    match self.the_expression_of_the_percent_and_the_space(&piece) {
                        // Measured on the binary of AutoHotkey v1.1.37.02 on
                        // 2026-11-05: the second parameter of the parse loop
                        // may be an expression, and the value of it is the
                        // text that is separated.
                        Some(node) => Node::LegacyExpression {
                            value: Box::new(node),
                            line,
                        },
                        None => Node::Literal {
                            value: LiteralValue::Text(piece.text.clone()),
                            line,
                        },
                    }
                } else {
                    Node::Literal {
                        value: LiteralValue::Text(piece.text.clone()),
                        line,
                    }
                };
                texts.push(piece.text.clone());
                arguments.push(node);
            }
        }
        Ok(texts)
    }

    /// The kind of the loop of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// first parameter of the `Loop` of the v1 names the kind of the loop
    /// when it is the "Read" or the "Parse" and when another parameter comes
    /// after it: the `Loop, Read` alone is a loop of the files that match the
    /// pattern "Read", and the `Loop Read, file` is the loop that reads the
    /// file.
    ///
    /// The first parameter that begins with a root of the registry is a loop
    /// of the registry, and the mode and the flag of the loop of the files
    /// are checked when the script loads.
    fn the_kind_of_the_loop_of_the_v1(
        &mut self,
        texts: &[String],
    ) -> Result<Option<LoopKind>, SyntaxError> {
        let first = texts
            .first()
            .map(|text| text.trim().to_lowercase())
            .unwrap_or_default();
        if texts.len() > 1 {
            match first.as_str() {
                "read" => return Ok(Some(LoopKind::Read)),
                "parse" => {
                    // The second parameter is the name of the variable whose
                    // text is separated, and the v1 checks the name when the
                    // script loads.
                    let name = texts.get(1).map(|text| text.trim()).unwrap_or_default();
                    if let Some(refusal) =
                        crate::v1::the_refusal_of_the_name_of_the_parse_loop(name)
                    {
                        return Err(self.error(refusal));
                    }
                    return Ok(Some(LoopKind::Parse));
                }
                "files" => {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-05: the word "Files" is the first parameter of
                    // the `Loop` of the v1, and the pattern of the files is
                    // the second parameter and the mode is the third one. The
                    // mode is a set of letters, and the letter that is not one
                    // of them is refused when the loop runs.
                    return Ok(Some(LoopKind::Files));
                }
                _ => {}
            }
        }
        // The loop of the registry of the v1.
        if let Some(refusal) = crate::v1::the_refusal_of_the_registry_of_the_loop(texts) {
            return Err(self.error(refusal));
        }
        // The v1 checks the mode and the flag of the loop of the files when
        // the script loads, and the complaint of the v1 names the parameter
        // that it does not want.
        if let Some(refusal) = crate::v1::the_refusal_of_the_parameters_of_the_loop(texts) {
            return Err(self.error(refusal));
        }
        Ok(None)
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
        if !self.at_the_word("in") {
            // Measured on the binary of v2.0.19 on 2026-11-01: the `for` needs
            // the `in`, and the dialect names the statement as it was written.
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the v1 names the word that is missing.
            let message = if self.dialect.the_for_is_the_one_of_the_v1() {
                "This \"For\" is missing its \"in\"."
            } else {
                "Syntax error."
            };
            return Err(self.error(format!(
                "{message}\n     Specifically: {}",
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
        if self.dialect.the_for_is_the_one_of_the_v1() {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the rest of the line after the source of the `for` is dropped:
            // the `for i in 1,2,3` walks nothing and it does not fail.
            while !self.at_end_of_statement() && !self.at_op(&["{"]) {
                self.position += 1;
            }
        }
        self.end_of_block()?;
        let body = self.the_body_of_a_loop("For", token.line)?;
        Ok(Node::For {
            variables,
            source: Box::new(source),
            body,
            line: token.line,
        })
    }

    /// Whether the token that comes next is this word.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `in` of the v1 is the name of a variable and not a word of the
    /// language, and the `for` of the v1 asks for it all the same.
    fn at_the_word(&self, word: &str) -> bool {
        let token = self.peek();
        (token.kind == TokenKind::Keyword || token.kind == TokenKind::Name)
            && token.value.eq_ignore_ascii_case(word)
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
        } else if !self.at_the_word("in") {
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
                    //
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-03: the line of a hotkey of the v1 is not the
                    // continuation of the expression that comes before it.
                    if self.the_next_line_begins_with_an_operator()
                        && !self.the_next_line_carries_a_hotkey()
                    {
                        self.skip_newlines();
                        continue;
                    }
                    return Ok(left);
                }
                // Measured on the binary of v2.0.19 on 2026-10-31: two values
                // side by side are one value, and the space between them is
                // what makes it so. Without the space, v2 refuses and names
                // the value that came too close.
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 does not ask for the space: the
                // `x := "a""b"` is the text `a"b`.
                if self.dialect.the_space_separates_the_values() && !self.peek().spaced {
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
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the operand that is not there poisons the operation that holds
            // it: the `1 +` is the empty text. The `&&` and the `||` are the
            // exception: the left side decides, and the answer is one or
            // zero -- the `0 and` is zero, and the `1 or` is one.
            if op == BinaryOp::And || op == BinaryOp::Or {
                if the_operand_is_not_there(&left) {
                    return Ok(Node::Unset { line: token.line });
                }
            } else if the_operand_is_not_there(&left) || the_operand_is_not_there(&right) {
                return Ok(Node::Unset { line: token.line });
            }
            left = Node::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                line: token.line,
            };
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
                // the `<>` is the `!=` of the v1, and the v2 does not know it.
                "<>" if self.dialect.the_lexer_knows_the_legacy_operators() => {
                    (BinaryOp::NotEqual, precedence::EQUAL, false)
                }
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
            TokenKind::Number | TokenKind::String | TokenKind::UnclosedText | TokenKind::Name => {
                true
            }
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the words of the v1 may be the name of a
                // variable, and a variable is a value: the operators are the
                // ones that do not begin one.
                if self.dialect.the_words_may_be_the_name_of_a_variable() {
                    return !matches!(word.as_str(), "and" | "or" | "not");
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the `if` and the `while` of the v1 may come
                // inside the line of the condition of an `if`, and the
                // condition of them may be written without parentheses: the
                // `if (0) if 1 = 1` is accepted.
                if self.in_the_condition_of_an_if
                    && self.dialect.the_control_flow_may_be_a_value()
                    && (word == "if" || word == "while")
                {
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
        let Some(token) = self.tokens.get(at) else {
            return false;
        };
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 merges the line that begins with an operator with the one before
        // it, and not the other way round.
        if !self.dialect.a_line_that_ends_with_an_operator_continues() {
            return false;
        }
        let Some(token) = self.the_token_before(at) else {
            return false;
        };
        match token.kind {
            TokenKind::Op => matches!(
                token.value.as_str(),
                "(" | "["
                    | ","
                    | "?"
                    | ":"
                    | "=>"
                    | "!"
                    | "~"
                    | "+"
                    | "-"
                    | "*"
                    | "/"
                    | "//"
                    | "**"
                    | "<<"
                    | ">>"
                    | ">>>"
                    | "&"
                    | "^"
                    | "|"
                    | "~="
                    | ">"
                    | ">="
                    | "<"
                    | "<="
                    | "="
                    | "=="
                    | "!="
                    | "!=="
                    | "&&"
                    | "||"
                    | "??"
                    | ":="
                    | "+="
                    | "-="
                    | "*="
                    | "/="
                    | "//="
                    | ".="
                    | "&="
                    | "|="
                    | "^="
                    | "<<="
                    | ">>="
                    | ">>>="
            ),
            TokenKind::Keyword => {
                matches!(token.value.to_lowercase().as_str(), "and" | "or" | "is")
            }
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
        let Some(next) = self.tokens.get(self.position + 1) else {
            return false;
        };
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
                format!(
                    "Unexpected reserved word.\n     Specifically: {}",
                    token.value
                )
            }
            TheRefusal::Func => format!(
                "This Func cannot be used as an output variable.\n     Specifically: {}",
                token.value
            ),
            TheRefusal::Class => {
                "This Class cannot be used as an output variable.\n     Specifically: Class"
                    .to_string()
            }
        };
        SyntaxError {
            message,
            line: token.line,
            column: token.column,
        }
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
            message: format!(
                "{message}\n     Specifically: {}",
                self.the_call_of_the_word(at)
            ),
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
        let Some(next) = self.tokens.get(self.position + 1) else {
            return false;
        };
        next.kind == TokenKind::Op && the_assignment_operator(&next.value).is_some()
    }

    /// Whether the word that opens the statement is the name of a function.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    /// function may be a word of the dialect -- `until() { }` is a function
    /// -- and the parenthesis that comes after it and a body are what make
    /// it a definition.
    fn the_word_is_the_name_of_a_function(&self) -> bool {
        let Some(open) = self.tokens.get(self.position + 1) else {
            return false;
        };
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
                "This Class cannot be used as an output variable.\n     Specifically: Class"
                    .to_string()
            }
            _ => format!(
                "The following reserved word must not be used as a variable name:\n{:?}",
                word
            ),
        };
        SyntaxError {
            message,
            line: token.line,
            column: token.column,
        }
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
            if self.dialect.the_illegal_character_says_the_leftmost() {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 says that the character is the leftmost
                // one of the line.
                format!(
                    "The leftmost character above is illegal in an expression.\n     Specifically: {text}"
                )
            } else {
                format!("Illegal character in expression.\n     Specifically: {text}")
            }
        } else {
            format!("This line does not contain a recognized action.\n     Specifically: {text}")
        };
        SyntaxError {
            message,
            line: token.line,
            column: token.column,
        }
    }

    /// The error of the name that carries the backtick that the escape
    /// carries.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// backtick that the escape carries is an illegal character in an
    /// expression, and the complaint of the v1 names the symbol that begins
    /// at it.
    fn the_error_of_the_escaped_backtick(&self, at: usize) -> SyntaxError {
        let token = &self.tokens[at];
        let text = the_symbol_of_the_escaped_backtick(self, at);
        SyntaxError {
            message: format!(
                "The leftmost character above is illegal in an expression.\n     Specifically: {text}"
            ),
            line: token.line,
            column: token.column,
        }
    }

    /// The error of the double deref that does not close.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the text that is
    /// named is the rest of the line, from the hash that opens it.
    fn the_error_of_a_missing_ending(&self, at: usize) -> SyntaxError {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
        // opens the name of a variable with the percent sign, and the name
        // that is not closed is refused with a complaint of its own. The text
        // that it names is the operand of the name.
        let run = the_source_of_the_operand(self, at);
        if let Some(refusal) = self
            .dialect
            .the_refusal_of_the_percent_sign_of_an_expression(&run, false)
        {
            return self.error(refusal);
        }
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
    fn the_error_of_a_percent_inside(&self, at: usize) -> SyntaxError {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
        // refuses the reference that is empty with "Empty variable reference
        // (%%).", and the text that it names is the operand of the name.
        let run = the_source_of_the_operand(self, at);
        if let Some(refusal) = self
            .dialect
            .the_refusal_of_the_percent_sign_of_an_expression(&run, true)
        {
            return self.error(refusal);
        }
        self.error(format!(
            "Syntax error.\n     Specifically: {}",
            the_source_of_the_rest_of_the_line(self, at)
        ))
    }

    /// The refusal of the point of the operand.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// name that the script builds may not carry a point: the `x := a.b%c` is
    /// refused with "Ambiguous or invalid use of \".\"" and the text that it
    /// names is the operand from its first point to its end.
    ///
    /// The v1 refuses the point before it refuses anything else of the name:
    /// the `x := a.b% c` is refused with the point and not with the percent
    /// sign, and the `x := a."b"%c` is refused with the point and not with
    /// the text of the member.
    ///
    /// The `at` is the place of the token that holds the percent sign, or the
    /// place of the point of a member.
    fn the_refusal_of_the_point_of_the_operand(&self, at: usize) -> Option<SyntaxError> {
        let (start, end) = the_operand_of_the_percent_sign(self, at);
        let text = the_source_of_the_tokens(self, start, end);
        let point = text.find('.')?;
        if !text[point..].contains('%') {
            return None;
        }
        let refusal = self
            .dialect
            .the_refusal_of_the_point_of_an_operand(&text[point..])?;
        Some(self.error(refusal))
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
            let open = self.position;
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the v1 refuses the point of the operand before it reads the
            // name: the `x := a.b%c` is refused with the point.
            if let Some(refusal) = self.the_refusal_of_the_point_of_the_operand(open) {
                return Err(refusal);
            }
            self.next();
            if self.at_op(&["%"]) {
                return Err(self.the_error_of_a_percent_inside(self.position));
            }
            let value = self.the_expression_of_a_dynamic_name(at)?;
            if !self.at_op(&["%"]) || !the_closing_is_in_the_operand(self, open, self.position) {
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
    fn the_pieces_after_the_percent(
        &mut self,
        value: Node,
    ) -> Result<Vec<DynamicPiece>, SyntaxError> {
        let mut pieces = vec![DynamicPiece::Value(Box::new(value))];
        while self.at_op(&["%"]) && !self.peek().spaced && self.derefs == 0 {
            let open = self.position;
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the v1 refuses the point of the operand before it reads the
            // name.
            if let Some(refusal) = self.the_refusal_of_the_point_of_the_operand(open) {
                return Err(refusal);
            }
            self.next();
            if self.at_op(&["%"]) {
                return Err(self.the_error_of_a_percent_inside(self.position));
            }
            let value = self.the_expression_of_a_dynamic_name(open)?;
            if !self.at_op(&["%"]) || !the_closing_is_in_the_operand(self, open, self.position) {
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
            TokenKind::Op if matches!(token.value.as_str(), ")" | "}" | "]") => self
                .the_message_of_a_bracket(
                    &format!("Unexpected {:?}", token.value),
                    &self.the_statement_so_far(),
                ),
            // The colon of the ternary is a part of the question that is
            // not there.
            TokenKind::Op if token.value == ":" => "A \":\" is missing its \"?\"".to_string(),
            // Measured on the binary of v2.0.19 on 2026-11-01: the hash and the
            // semicolon that are not a directive and not a comment are illegal
            // characters in an expression, and the text of the expression is
            // named.
            TokenKind::Op if the_token_is_an_illegal_character(token) => {
                return self
                    .the_error_of_the_illegal_character(self.position.saturating_sub(1), true);
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
                let rest =
                    the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1));
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
                let rest =
                    the_source_of_the_rest_of_the_line(self, self.position.saturating_sub(1));
                if !rest.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(&rest);
                }
                format!("Missing operand.\n     Specifically: {text}")
            }
        };
        SyntaxError {
            message,
            line: start.line,
            column: start.column,
        }
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
        let Some(next) = self.tokens.get(self.position) else {
            return false;
        };
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
                "**" | "*"
                    | "/"
                    | "//"
                    | "+"
                    | "-"
                    | "<<"
                    | ">>"
                    | ">>>"
                    | "&"
                    | "^"
                    | "|"
                    | "~="
                    | ">"
                    | ">="
                    | "<"
                    | "<="
                    | "="
                    | "=="
                    | "!="
                    | "!=="
                    | "&&"
                    | "||"
                    | "??"
                    | "?"
                    | "+="
                    | "-="
                    | "*="
                    | "/="
                    | "//="
                    | ".="
                    | "&="
                    | "|="
                    | "^="
                    | "<<="
                    | ">>="
                    | ">>>="
            ),
            TokenKind::Keyword => {
                matches!(token.value.to_lowercase().as_str(), "and" | "or" | "is")
            }
            _ => false,
        };
        if wants { Some(at) } else { None }
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
                    && the_increment_of_the_value_is_refused(self.dialect, &operand)
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
                    if let Node::Literal {
                        value: LiteralValue::Number(text),
                        line,
                    } = operand
                    {
                        let text = match op {
                            UnaryOp::Negate => the_text_of_the_negated_number(&text),
                            _ => text,
                        };
                        return Ok(Node::Literal {
                            value: LiteralValue::Number(text),
                            line,
                        });
                    }
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the operand that is not there poisons the
                // operation that holds it: the `not 1 +` is the empty text.
                if the_operand_is_not_there(&operand) {
                    return Ok(Node::Unset { line: token.line });
                }
                return Ok(Node::Unary {
                    op,
                    operand: Box::new(operand),
                    line: token.line,
                });
            }
        }
        if token.kind == TokenKind::Keyword && token.value.eq_ignore_ascii_case("not") {
            self.next();
            let operand = self.parse_binary(precedence::NOT)?;
            if the_operand_is_not_there(&operand) {
                return Ok(Node::Unset { line: token.line });
            }
            return Ok(Node::Unary {
                op: UnaryOp::NotWord,
                operand: Box::new(operand),
                line: token.line,
            });
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the `++` and the `--` that come after a number
                // are the increment of the v1, and the number is a literal
                // here: the `x := 5++3` is the increment of a number that
                // cannot be written, and the value of it is the empty text.
                if self.dialect.the_increment_after_a_number_is_an_operator()
                    && self.at_op(&["++", "--"])
                    && (!self.dialect.the_space_separates_the_increment() || !self.peek().spaced)
                {
                    let operator = self.next();
                    let op = if operator.value == "++" {
                        UnaryOp::PostIncrement
                    } else {
                        UnaryOp::PostDecrement
                    };
                    let operand = Node::Literal {
                        value: the_literal_of_a_number(&token.value, self.dialect),
                        line: token.line,
                    };
                    return Ok(Node::Unary {
                        op,
                        operand: Box::new(operand),
                        line: token.line,
                    });
                }
                let value = the_literal_of_a_number(&token.value, self.dialect);
                Node::Literal {
                    value,
                    line: token.line,
                }
            }
            TokenKind::String => Node::Literal {
                value: LiteralValue::Text(token.value),
                line: token.line,
            },
            TokenKind::UnclosedText => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 has no texts in the line of a command,
                // and the quote that is not closed there is not an error. The
                // text that is read as an expression is refused: the
                // `x := "abc` is refused with "Missing close-quote" and the
                // text that was read.
                return Err(self.error(format!(
                    "Missing close-quote\n     Specifically: {}",
                    token.value
                )));
            }
            TokenKind::Keyword => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the word that is an operator and that begins
                // the expression takes the value of the right side: the
                // `x := and 2` is one, and the `x := or 2` is one. The word
                // alone is the operand that is not there, and the value of
                // the expression is the empty text.
                if self.dialect.the_operand_that_is_not_there_is_not_refused() {
                    let word = token.value.to_lowercase();
                    if word == "and" || word == "or" {
                        if !self.starts_expression() {
                            return Ok(Node::Unset { line: token.line });
                        }
                        let operand = self.parse_binary(precedence::AND + 1)?;
                        return Ok(Node::Unary {
                            op: UnaryOp::Boolean,
                            operand: Box::new(operand),
                            line: token.line,
                        });
                    }
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the `if` and the `while` of the v1 may come
                // inside the line of the condition of an `if`: the
                // `if (0) if (1) x := 1` is accepted.
                if self.in_the_condition_of_an_if
                    && self.dialect.the_control_flow_may_be_a_value()
                    && (token.value.eq_ignore_ascii_case("if")
                        || token.value.eq_ignore_ascii_case("while"))
                {
                    return self.parse_the_control_of_a_condition(token);
                }
                // The `super` of v2 is the base of the class where the
                // method was defined. It is the only keyword of the v2 that
                // is neither a value nor a call.
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
                        "true" => Node::Literal {
                            value: LiteralValue::Bool(true),
                            line: token.line,
                        },
                        "false" => Node::Literal {
                            value: LiteralValue::Bool(false),
                            line: token.line,
                        },
                        // The `Class` is a word of the language and a class
                        // type of the dialect: a script reads it as the
                        // class.
                        "class" => Node::Var {
                            name: token.value.clone(),
                            line: token.line,
                        },
                        _ => {
                            // Measured on the binary of AutoHotkey v1.1.37.02 on
                            // 2026-11-04: the words of the v1 may be the name
                            // of a variable: the `x := while` reads the
                            // variable "while".
                            let lower = token.value.to_lowercase();
                            // Measured on the binary of AutoHotkey v1.1.37.02 on
                            // 2026-11-04: the words of the v1 may be the name
                            // of a variable: the `x := while` reads the
                            // variable "while". The operators of the v1 are
                            // operators and not names.
                            if (self.dialect.the_words_may_be_the_name_of_a_variable()
                                && !matches!(lower.as_str(), "and" | "or" | "not"))
                                || crate::builtins::find(&lower).is_some()
                            {
                                Node::Var {
                                    name: token.value.clone(),
                                    line: token.line,
                                }
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the single quote is not a text in the v1, and
                // the symbol that begins with it is an illegal character in
                // an expression.
                if self.dialect.the_single_quote_is_not_a_text()
                    && the_symbol_begins_with_the_illegal_quote(&token)
                {
                    return Err(self.the_error_of_the_illegal_character(self.position - 1, true));
                }
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the backtick that the escape carries is an
                // illegal character in an expression: the `x := a``b` is
                // refused with the symbol that begins at it.
                if self.dialect.the_escape_may_be_in_a_name() && token.value.contains('`') {
                    return Err(self.the_error_of_the_escaped_backtick(self.position - 1));
                }
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the name of the v1 may not carry an illegal
                // character: the `x := a"b"` is a name with an illegal
                // character. The refusal is here and not above: the name of a
                // function is not a variable, and the v1 does not refuse it.
                if let Some(refusal) = self.the_refusal_of_the_name(&token) {
                    return Err(refusal);
                }
                let name = token.value;
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
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
                    let pieces =
                        self.the_pieces_of_a_dynamic_name(name, at_the_name, token.line)?;
                    return Ok(Node::DynamicName {
                        pieces,
                        line: token.line,
                    });
                }
                // The arrow function of v2 with one parameter: `a => a + 1`.
                if self.at_op(&["=>"]) {
                    self.position += 1;
                    let body = self.parse_expression()?;
                    let parameters = vec![Parameter {
                        name: name.clone(),
                        default: None,
                        variadic: false,
                        by_ref: false,
                    }];
                    return Ok(Node::Lambda {
                        parameters,
                        body: Box::new(body),
                        line: token.line,
                    });
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
                    let operand = Node::Var {
                        name,
                        line: token.line,
                    };
                    return Ok(Node::Unary {
                        op,
                        operand: Box::new(operand),
                        line: token.line,
                    });
                }
                Node::Var {
                    name,
                    line: token.line,
                }
            }
            TokenKind::Op
                if token.value == "(" && self.at_the_parameters_of_an_arrow_function() =>
            {
                // The arrow function of v2: `(a, b) => a + b`. The body is
                // an expression, and the function closes over the scope
                // where it was written.
                let parameters = self.parse_parameters()?;
                self.expect_op(")")?;
                self.expect_op("=>")?;
                let body = self.parse_expression()?;
                Node::Lambda {
                    parameters,
                    body: Box::new(body),
                    line: token.line,
                }
            }
            TokenKind::Op if token.value == "%" => {
                // Measured on the binary of v2.0.19 on 2026-10-31: the double
                // deref of v2 is a name that the script builds -- the value of
                // the expression is the name of the variable whose value is
                // the answer -- and the hash that does not close it is
                // refused with the text of the rest of the line.
                let open = self.position - 1;
                // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
                // the v1 refuses the point of the operand before it reads the
                // name: the `x := %a.b%` is refused with the point.
                if let Some(refusal) = self.the_refusal_of_the_point_of_the_operand(open) {
                    return Err(refusal);
                }
                if self.at_op(&["%"]) {
                    return Err(self.the_error_of_a_percent_inside(self.position));
                }
                let value = self.the_expression_of_a_dynamic_name(open)?;
                if !self.at_op(&["%"]) || !the_closing_is_in_the_operand(self, open, self.position)
                {
                    return Err(self.the_error_of_a_missing_ending(open));
                }
                self.next();
                let pieces = self.the_pieces_after_the_percent(value)?;
                Node::DynamicName {
                    pieces,
                    line: token.line,
                }
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
                // The bracket that closes the group is the one that the
                // operand that is not there may meet.
                let outer_bracket = self.expected_bracket;
                self.expected_bracket = Some(")");
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
                self.expected_bracket = outer_bracket;
                self.expect_op(")")?;
                if items.len() == 1 {
                    items.pop().expect("the item of the group")
                } else {
                    Node::Group {
                        items,
                        line: token.line,
                    }
                }
            }
            TokenKind::Op if token.value == "[" => {
                // The literal of an array: `[a, b, c]`. The brackets of
                // an index are a suffix of a value, and these open one.
                let mut items = Vec::new();
                // The bracket that closes the array is the one that the
                // operand that is not there may meet, and the comma that
                // separates the items ends the value that is being read.
                let outer_bracket = self.expected_bracket;
                let outer_comma = self.the_comma_ends_the_value;
                self.expected_bracket = Some("]");
                self.the_comma_ends_the_value = true;
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
                self.expected_bracket = outer_bracket;
                self.the_comma_ends_the_value = outer_comma;
                self.expect_op("]")?;
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the literal of an array that holds the operand
                // that is not there is the operand that is not there: the
                // `a := [1 +]` is not an object, and the `IsObject(a)` is
                // zero.
                if self
                    .dialect
                    .the_operand_that_is_not_there_poisons_the_value_that_holds_it()
                    && items
                        .iter()
                        .any(|item| item.as_ref().is_some_and(the_operand_is_not_there))
                {
                    return Ok(Node::Unset { line: token.line });
                }
                Node::ArrayLiteral {
                    items,
                    line: token.line,
                }
            }
            TokenKind::Op if token.value == "{" => {
                // The literal of an object: `{a: 1, b: 2}`. The braces of
                // a block open a statement, and these open a value.
                self.the_object_literal(token.line)?
            }
            _ => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 does not refuse the expression whose
                // operand is not there: the `x := 1 +` is accepted, and the
                // value of the expression is the empty text. The end of the
                // line is where the operand is not there, and so is the
                // bracket that closes the value that is being read: the
                // `x := (1 +)` is the empty text, and the `x := )` is
                // refused with "Unexpected ")"".
                // The comma that separates the items of a list ends the
                // value that is being read, the way the bracket that closes it
                // does.
                let the_value_ends = matches!(token.kind, TokenKind::Newline | TokenKind::Eof)
                    || (token.kind == TokenKind::Op
                        && (self.expected_bracket == Some(token.value.as_str())
                            || (token.value == "," && self.the_comma_ends_the_value)));
                if self.dialect.the_operand_that_is_not_there_is_not_refused() && the_value_ends {
                    // The token is not consumed: the statement ends there, or
                    // the bracket closes the value that is being read.
                    self.position -= 1;
                    return Ok(Node::Unset { line: token.line });
                }
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 says "Missing "}"" and it names no text.
        let message = if self
            .dialect
            .the_literal_that_is_not_closed_names_the_statement()
        {
            format!(
                "Missing \"}}\"\n     Specifically: {}",
                the_source_with_the_newlines_as_spaces(self, self.statement_start, end)
            )
        } else {
            "Missing \"}\"".to_string()
        };
        SyntaxError {
            message,
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
        let mut items = Vec::new();
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // literal is a list of parameters, and the number of them has to be
        // even: the key of a property and its value are two, and a parameter
        // that is in blank is one. The number is checked after every
        // parameter that is not in blank.
        let mut count = 0usize;
        // The brace that opens the literal: the text that the failure of the
        // blank parameter names starts at it.
        let brace = self.position.saturating_sub(1);
        // The brace that closes the literal is the one that the operand that
        // is not there may meet, and the comma that separates the properties
        // ends the value that is being read.
        let outer_bracket = self.expected_bracket;
        let outer_comma = self.the_comma_ends_the_value;
        self.expected_bracket = Some("}");
        self.the_comma_ends_the_value = true;
        // The property may be on the line that follows the brace.
        self.skip_newlines();
        while !self.at_op(&["}"]) {
            // The parameter that is in blank: the comma with nothing before
            // it.
            if self.at_op(&[","]) {
                count += 1;
                self.position += 1;
                // The property may be on the line that follows the comma.
                self.skip_newlines();
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 refuses the blank at the end of the
                // list with "Blank parameter", and it names the whole
                // literal. The v2 accepts it: the `{b: 1,}` is an object of
                // one property.
                if self.at_op(&["}"]) && self.dialect.the_blank_at_the_end_of_a_list_is_refused() {
                    return Err(self.the_error_of_a_blank_parameter(line, open, brace));
                }
                continue;
            }
            // The parameter: the key of a property and its value, or a value
            // that has no name.
            let start = self.position;
            if self.dialect.the_name_of_a_property_may_be_any_text() {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the name of a property of the v1 is the text
                // that is written until the colon, and the quotes of a text
                // are not part of it: the `{"b": 1}` has the property "b",
                // and the `a.b` is one.
                //
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 wants the name of a property between
                // quotes: the `a := {%x%: 1}` is refused with "Quote marks are
                // required around this key." and the text that is written from
                // the percent sign on is named.
                if self
                    .dialect
                    .the_name_of_a_property_that_the_script_builds_is_refused()
                    && self.at_op(&["%"])
                {
                    return Err(SyntaxError {
                        message: format!(
                            "Quote marks are required around this key.\n     Specifically: {}",
                            the_source_of_the_rest_of_the_line(self, start)
                        ),
                        line,
                        column: open,
                    });
                }
                // The name ends at the colon, and the comma and the brace
                // that close the list end it too.
                while !self.at_op(&[":"])
                    && !self.at_op(&[","])
                    && !self.at_op(&["}"])
                    && self.peek().kind != TokenKind::Eof
                {
                    self.position += 1;
                }
                if !self.at_op(&[":"]) {
                    // The value that has no name: it is one parameter, and it
                    // is evaluated and dropped.
                    count += 1;
                    self.position = start;
                    let value = match self.parse_expression() {
                        Ok(value) => value,
                        Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                    };
                    items.push(ObjectItem::Value(value));
                } else if self.position == start {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the name that is not there is no property:
                    // the `a := {: 1}` is an object of no property.
                    count += 2;
                    self.position += 1;
                    let value = match self.parse_expression() {
                        Ok(value) => value,
                        Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                    };
                    items.push(ObjectItem::Value(value));
                } else {
                    // The key and the value of the property are two
                    // parameters.
                    count += 2;
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-04: the key is the name when it is a name, and
                    // the value of the expression when it is not one: the
                    // `{b: 1}` has the property "b" whatever the value of b
                    // is, and the `{1 + 1: 1}` has the property "2".
                    if the_key_of_a_property_of_the_v1_is_a_name(self, start, self.position) {
                        let member = MemberName::Text(the_name_of_a_property_of_the_v1(
                            self,
                            start,
                            self.position,
                        ));
                        self.position += 1;
                        let value = match self.parse_expression() {
                            Ok(value) => value,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                        };
                        items.push(ObjectItem::Property(member, value));
                    } else {
                        // The key is the value of the expression that is
                        // written until the colon.
                        self.position = start;
                        let key = match self.parse_expression() {
                            Ok(key) => key,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                        };
                        // The expression of the key ends at the colon.
                        self.position += 1;
                        let value = match self.parse_expression() {
                            Ok(value) => value,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                        };
                        items.push(ObjectItem::LegacyKey(key, value));
                    }
                }
            } else {
                // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
                // property may be built by the script: `{%name%: 1}` is the
                // property whose name is the value of `name`.
                if self.at_op(&["%"]) {
                    let at = self.position;
                    self.next();
                    if self.at_op(&["%"]) {
                        return Err(self.the_error_of_a_percent_inside(self.position));
                    }
                    let value = self.the_expression_of_a_dynamic_name(at)?;
                    if !self.at_op(&["%"]) {
                        return Err(self.the_error_of_a_missing_ending(at));
                    }
                    self.next();
                    if !self.at_op(&[":"]) {
                        return Err(SyntaxError {
                            message: self.dialect.the_refusal_of_the_object_literal().to_string(),
                            line,
                            column: open,
                        });
                    }
                    self.position += 1;
                    count += 2;
                    let member = MemberName::Dynamic(self.the_pieces_after_the_percent(value)?);
                    let value = match self.parse_expression() {
                        Ok(value) => value,
                        Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                    };
                    items.push(ObjectItem::Property(member, value));
                } else {
                    // Measured on the binary of v2.0.19 on 2026-11-04: the name
                    // of a property of the v2 is a name of the script, and the
                    // text that is written until the colon is the name: the
                    // `{b c: 1}` is refused with "Invalid property name in
                    // object literal.".
                    while !self.at_op(&[":"])
                        && !self.at_op(&[","])
                        && !self.at_op(&["}"])
                        && self.peek().kind != TokenKind::Eof
                    {
                        self.position += 1;
                    }
                    if !self.at_op(&[":"]) {
                        // The value that has no name: it is one parameter, and
                        // it is evaluated and dropped.
                        count += 1;
                        self.position = start;
                        let value = match self.parse_expression() {
                            Ok(value) => value,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                        };
                        items.push(ObjectItem::Value(value));
                    } else {
                        // The name of a property may be a whole number: the
                        // number with a point is not a name, because the point
                        // of it is the point of a member.
                        //
                        // Measured on the binary of v2.0.19 on 2026-10-31:
                        // `{5: 1}` is an object with the property "5", and
                        // `{1.5: 1}` is refused.
                        let mut named = false;
                        if self.position - start == 1 {
                            if let Some(token) = self.tokens.get(start) {
                                named = token.kind == TokenKind::Name
                                    || token.kind == TokenKind::Keyword
                                    || the_number_is_a_whole_one(token, self.dialect);
                            }
                        }
                        if !named {
                            // Measured on the binary of v2.0.19 on 2026-10-31:
                            // the object that is not closed is the failure, and
                            // the name of the property that is not there is not
                            // it.
                            if let Some(token) = self.tokens.get(start) {
                                if token.kind == TokenKind::Eof {
                                    return Err(self.the_error_of_a_missing_bracket(
                                        "}",
                                        token.line,
                                        token.column,
                                    ));
                                }
                            }
                            return Err(SyntaxError {
                                message: "Invalid property name in object literal.".into(),
                                line,
                                column: open,
                            });
                        }
                        count += 2;
                        let member = MemberName::Text(
                            self.tokens
                                .get(start)
                                .map(|token| token.value.clone())
                                .unwrap_or_default(),
                        );
                        self.position += 1;
                        let value = match self.parse_expression() {
                            Ok(value) => value,
                            Err(error) => return Err(self.the_error_of_the_bracket(error, "}")),
                        };
                        items.push(ObjectItem::Property(member, value));
                    }
                }
            }
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the number of the parameters of the literal has to be even, and
            // a number that is odd is refused with the name of the key that is
            // not there.
            if count % 2 == 1 {
                return Err(SyntaxError {
                    message: self.dialect.the_refusal_of_the_object_literal().to_string(),
                    line,
                    column: open,
                });
            }
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
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the comma that ends the list is a parameter in
                // blank, and the v1 refuses it with "Blank parameter". The v2
                // accepts it.
                if self.at_op(&["}"]) && self.dialect.the_blank_at_the_end_of_a_list_is_refused() {
                    return Err(self.the_error_of_a_blank_parameter(line, open, brace));
                }
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
        self.expected_bracket = outer_bracket;
        self.the_comma_ends_the_value = outer_comma;
        self.expect_op("}")?;
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // literal of an object that holds the operand that is not there is
        // the operand that is not there: the `o := {x: 1 +}` is not an object,
        // and the `IsObject(o)` is zero.
        if self
            .dialect
            .the_operand_that_is_not_there_poisons_the_value_that_holds_it()
            && items.iter().any(|item| match item {
                ObjectItem::Property(_, value) => the_operand_is_not_there(value),
                ObjectItem::LegacyKey(key, value) => {
                    the_operand_is_not_there(key) || the_operand_is_not_there(value)
                }
                ObjectItem::Value(value) => the_operand_is_not_there(value),
            })
        {
            return Ok(Node::Unset { line });
        }
        Ok(Node::ObjectLiteral { items, line })
    }

    /// The failure of the blank parameter at the end of a list.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the `{b: 1,}` with "Blank parameter", and the text that is
    /// named is the whole literal, from the brace that opens it.
    fn the_error_of_a_blank_parameter(&self, line: u32, column: u32, brace: usize) -> SyntaxError {
        SyntaxError {
            message: format!(
                "Blank parameter\n     Specifically: {}",
                the_source_of_the_rest_of_the_line(self, brace)
            ),
            line,
            column,
        }
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
    fn parse_suffixes(
        &mut self,
        target: Node,
        assignment: bool,
        command: bool,
    ) -> Result<Node, SyntaxError> {
        let mut node = target;
        loop {
            if self.at_op(&["["]) {
                let line = node.line();
                self.position += 1;
                let index = self.parse_expression()?;
                // The index may be written in more than one line.
                self.skip_newlines();
                self.expect_op("]")?;
                node = Node::Index {
                    target: Box::new(node),
                    index: Box::new(index),
                    line,
                };
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
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-05: the v1 refuses the point of the operand
                    // before it reads the name: the `x := a.%b%` is refused
                    // with the point.
                    if let Some(refusal) = self.the_refusal_of_the_point_of_the_operand(open) {
                        return Err(refusal);
                    }
                    self.next();
                    if self.at_op(&["%"]) {
                        return Err(self.the_error_of_a_percent_inside(self.position));
                    }
                    let value = self.the_expression_of_a_dynamic_name(open)?;
                    if !self.at_op(&["%"])
                        || !the_closing_is_in_the_operand(self, open, self.position)
                    {
                        return Err(self.the_error_of_a_missing_ending(open));
                    }
                    self.next();
                    let pieces = self.the_pieces_after_the_percent(value)?;
                    let name = MemberName::Dynamic(pieces);
                    if self.at_op(&["("]) {
                        self.next();
                        let arguments = self.parse_arguments()?;
                        self.expect_op(")")?;
                        self.the_method_of_the_v1_is_recorded(&arguments, at_the_dot);
                        node = self.the_method_call(node, name, arguments, dot.line);
                        continue;
                    }
                    node = Node::Property {
                        target: Box::new(node),
                        name,
                        line: dot.line,
                    };
                    continue;
                }
                let name = self.next();
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the single quote is not a text in the v1, and
                // the symbol that begins with it is an illegal character in
                // an expression.
                if self.dialect.the_single_quote_is_not_a_text()
                    && the_symbol_begins_with_the_illegal_quote(&name)
                {
                    return Err(self.the_error_of_the_illegal_character(self.position - 1, true));
                }
                // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-02:
                // the name of a member may not be a text: the `a.'b'` is
                // refused with "Illegal character in expression." and the text
                // is named as it was written.
                //
                // The v1 reads the single quote as a symbol and refuses it
                // above, and the double quote is a text that the v1 refuses
                // with "This character is not allowed here." and the rest of
                // the line, from the quote: the `x := a."b" + 1` names
                // `"b" + 1`.
                if name.kind == TokenKind::String {
                    // Measured on the binary of AutoHotkey v1.1.37.02 on
                    // 2026-11-05: the v1 refuses the point of the operand
                    // before it refuses the text of the member: the
                    // `x := a."b"%c` is refused with the point.
                    if let Some(refusal) = self.the_refusal_of_the_point_of_the_operand(at_the_dot)
                    {
                        return Err(refusal);
                    }
                    if let Some(refusal) = self.dialect.the_refusal_of_the_text_of_a_member(
                        &the_source_of_the_rest_of_the_line(self, self.position - 1),
                    ) {
                        return Err(self.error(refusal));
                    }
                    if !self.dialect.the_single_quote_is_not_a_text() {
                        return Err(
                            self.the_error_of_the_illegal_character(self.position - 1, true)
                        );
                    }
                }
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
                    MemberName::Dynamic(self.the_pieces_of_a_dynamic_name(
                        name.value,
                        at_the_name,
                        name.line,
                    )?)
                } else {
                    MemberName::Text(name.value)
                };
                if self.at_op(&["("]) && !self.peek().spaced {
                    self.next();
                    let arguments = self.parse_arguments()?;
                    self.expect_op(")")?;
                    self.the_method_of_the_v1_is_recorded(&arguments, at_the_dot);
                    node = self.the_method_call(node, member, arguments, dot.line);
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
                    return Ok(self.the_method_call(node, member, arguments, dot.line));
                }
                node = Node::Property {
                    target: Box::new(node),
                    name: member,
                    line: dot.line,
                };
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
                node = Node::Unary {
                    op,
                    operand: Box::new(node),
                    line,
                };
                continue;
            }
            if assignment && self.at_an_assignment_operator() {
                return self.parse_the_assignment(node);
            }
            break;
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // property and the item of the operand that is not there are the
        // operand that is not there: the `f(1 +).y` is the empty text, and so
        // is the `f(1 +)[1]`.
        if self
            .dialect
            .the_operand_that_is_not_there_poisons_the_value_that_holds_it()
            && the_operand_is_not_there(&node)
        {
            return Ok(Node::Unset { line: node.line() });
        }
        Ok(node)
    }

    /// The call of a method, and the value that holds it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// method that takes the operand that is not there is the operand that is
    /// not there, and so is the method of it: the `o.Method(1 +)` is the
    /// empty text.
    fn the_method_call(
        &mut self,
        target: Node,
        name: MemberName,
        arguments: Vec<Node>,
        line: u32,
    ) -> Node {
        if self
            .dialect
            .the_operand_that_is_not_there_poisons_the_value_that_holds_it()
            && (the_operand_is_not_there(&target) || arguments.iter().any(the_operand_is_not_there))
        {
            return Node::Unset { line };
        }
        Node::MethodCall {
            target: Box::new(target),
            name,
            arguments,
            line,
        }
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
        let Some(token) = self.tokens.get(self.position + 1) else {
            return false;
        };
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
        let Some(list) = self.assignments.last_mut() else {
            return;
        };
        list.push(Assignment {
            name: name.to_string(),
            op,
            line,
            column,
        });
    }

    /// The value of an assignment that is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `y :=` writes the empty text, and the line that comes is a statement
    /// of its own: the `y :=` with a `3 + 4` under it is a line that holds
    /// no action. The v2 continues the assignment on the line that comes.
    fn the_empty_value_of_an_assignment(&mut self) -> Option<Node> {
        if !self.dialect.the_assignment_may_have_no_value() {
            return None;
        }
        if !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            return None;
        }
        let line = self.peek().line;
        Some(Node::Literal {
            value: LiteralValue::Text(String::new()),
            line,
        })
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
        let value = match self.the_empty_value_of_an_assignment() {
            Some(value) => value,
            None => self.parse_expression()?,
        };
        let end = self.position;
        let line = target.line();
        Ok(match target {
            Node::Var { name, .. } => {
                self.the_assignment_is_recorded(&name, op, line, operator.column);
                Node::Assign {
                    name,
                    op,
                    value: Box::new(value),
                    line,
                }
            }
            Node::Property { target, name, .. } => Node::AssignProperty {
                target,
                name,
                op,
                value: Box::new(value),
                line,
            },
            Node::Index { target, index, .. } => Node::AssignIndex {
                target,
                index,
                op,
                value: Box::new(value),
                line,
            },
            Node::DynamicName { pieces, .. } => Node::AssignDynamic {
                name: pieces,
                op,
                value: Box::new(value),
                line,
            },
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
            return Err(
                self.error("\"Super\" is valid only inside a class.\n     Specifically: super()")
            );
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
        let (arguments, missing) = self.the_arguments_of_a_call(&name)?;
        if self.dialect.refuses_the_call_of_a_function_at_the_load() {
            self.the_call_of_the_v1_is_recorded(&name, &arguments, at, missing, false, true);
        }
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // call that takes the operand that is not there is the operand that is
        // not there: the `StrLen(1 +)` is the empty text, and not the zero
        // that the `StrLen` of the empty text answers.
        if self
            .dialect
            .the_operand_that_is_not_there_poisons_the_value_that_holds_it()
            && arguments.iter().any(the_operand_is_not_there)
        {
            return Ok(Node::Unset { line });
        }
        Ok(Node::Call {
            name,
            arguments,
            line,
        })
    }

    /// The arguments of a call, and the parenthesis that closes them.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not need the parenthesis that closes the call of a name that is
    /// not a function: the `x := abc(c` is refused at the load with "Call to
    /// nonexistent function.\n     Specifically: abc(c", and the parse of it
    /// does not fail. The arguments of it are the rest of the line, and the
    /// list that is not closed does not go on to the line that comes next.
    ///
    /// The call of a name that is a function needs the parenthesis, and the
    /// `x := f(1` is refused with "Missing ")"". The name of a function of
    /// the script may be defined after the call, so the failure of it waits
    /// for the end of the parse. The name of a built-in is known at the
    /// parse, and the failure of it is made there.
    ///
    /// The v2 needs the parenthesis always.
    fn the_arguments_of_a_call(
        &mut self,
        name: &str,
    ) -> Result<(Vec<Node>, Option<SyntaxError>), SyntaxError> {
        if self.dialect.the_closing_parenthesis_of_a_call_is_needed() {
            let arguments = self.parse_arguments()?;
            self.expect_op(")")?;
            return Ok((arguments, None));
        }
        // The list is empty and the parenthesis is not there.
        if self.at_the_end_of_the_line() {
            let at = self.position;
            return Ok((
                Vec::new(),
                Some(self.the_error_of_a_missing_parenthesis(at)),
            ));
        }
        let arguments = match self.parse_arguments() {
            Ok(arguments) => arguments,
            Err(error) => {
                // The list is not closed, and the arguments of it are the rest
                // of the line.
                if !self.at_the_end_of_the_line() {
                    return Err(error);
                }
                Vec::new()
            }
        };
        let at = self.position;
        match self.expect_op(")") {
            Ok(_) => {
                // Measured on the binary of AutoHotkey v1.1.37.02 on
                // 2026-11-04: the v1 reads the rest of the line as text when
                // the name of the call is not a function: the `MyFunc() { return
                // 2 }` is refused at the load with "Call to nonexistent
                // function.", and the brace of it is not refused. When the
                // name is a function, the rest of the line is read, and the
                // `f(1) { return 2 }` is refused with "Unexpected "{"".
                if self
                    .dialect
                    .the_tail_of_a_call_to_a_name_that_is_not_a_function_is_text()
                    && !self.the_name_is_a_function(name)
                {
                    self.position = the_end_of_the_line_of(self, self.position);
                }
                Ok((arguments, None))
            }
            Err(error) => {
                if self.the_name_is_a_builtin_function(name) {
                    return Err(error);
                }
                // The token that is where the parenthesis should be is not
                // consumed, and the arguments of the call of the v1 end at
                // the end of the line: the list does not go on to the line
                // that comes next.
                let mut at = at;
                while at > 0 && self.tokens[at - 1].kind == TokenKind::Newline {
                    at -= 1;
                }
                self.position = at;
                Ok((arguments, Some(error)))
            }
        }
    }

    /// Whether the line of the token that is there ends there.
    fn at_the_end_of_the_line(&self) -> bool {
        let token = self.peek();
        token.kind == TokenKind::Newline || token.kind == TokenKind::Eof
    }

    /// The failure of the parenthesis that closes a list of arguments and
    /// that is not there.
    fn the_error_of_a_missing_parenthesis(&self, at: usize) -> SyntaxError {
        let line = self.tokens.get(at).map(|token| token.line).unwrap_or(0);
        let column = self.tokens.get(at).map(|token| token.column).unwrap_or(0);
        SyntaxError {
            message: "Missing \")\"".to_string(),
            line,
            column,
        }
    }

    /// Whether the name is a built-in of the dialect that is a function.
    fn the_name_is_a_builtin_function(&self, name: &str) -> bool {
        let wanted = name.to_lowercase();
        self.dialect.builtins().iter().any(|builtin| {
            builtin.name.to_lowercase() == wanted
                && builtin.kind == crate::builtins::BuiltinKind::Function
        })
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
        Ok(Node::New {
            path,
            arguments,
            line,
        })
    }

    /// The method of the v1 is recorded, and it is checked at the end of the
    /// parse.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// number of the arguments of a method is not checked at the load -- the
    /// `o.Say(1, 2)` is accepted -- and the list of the arguments that ends
    /// with a blank is refused at the load all the same: the `o.Say(1,)` is
    /// refused with "Blank parameter", and the name of it is the text of the
    /// call, from the dot.
    fn the_method_of_the_v1_is_recorded(&mut self, arguments: &[Node], at: usize) {
        if !self.dialect.refuses_the_call_of_a_function_at_the_load() {
            return;
        }
        // The list of the arguments that does not end with a blank is
        // accepted: the number of the arguments of a method is not checked
        // at the load.
        let blanks = the_groups_of_the_blanks(arguments);
        let ends_with_a_blank = blanks
            .last()
            .is_some_and(|(start, size)| start + size == arguments.len());
        if !ends_with_a_blank {
            return;
        }
        self.the_call_of_the_v1_is_recorded("", arguments, at, None, false, false);
    }

    /// The call of the v1 is recorded, and it is checked at the end of the
    /// parse.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the call is
    /// refused at the load, and the text of it is the text of the script.
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the text
    /// that the refusal names is the rest of the line, from the name of the
    /// call: the `x := abc(c) + 1` is refused with "Call to nonexistent
    /// function.\n     Specifically: abc(c) + 1", and the same is true of the
    /// complaint of the number of the arguments that is wrong.
    fn the_call_of_the_v1_is_recorded(
        &mut self,
        name: &str,
        arguments: &[Node],
        at: usize,
        missing: Option<SyntaxError>,
        the_call_is_the_statement: bool,
        the_arity_is_checked: bool,
    ) {
        let end_of_the_line = the_end_of_the_line_of(self, at);
        let mut source = self.the_verbatim_source(at, end_of_the_line);
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // brace that opens a block is not a part of the text that the refusal
        // names: the `zzz(1) {` is refused with "Call to nonexistent function.
        // Specifically: zzz(1)". The brace of an expression is a part of the
        // text: the `x := abc(c) {` is refused with "Call to nonexistent
        // function.\n     Specifically: abc(c) {".
        if the_call_is_the_statement && source.ends_with('{') {
            source.truncate(source.len() - 1);
            source = source.trim_end().to_string();
        }
        let line = self.tokens.get(at).map(|token| token.line).unwrap_or(0);
        let column = self.tokens.get(at).map(|token| token.column).unwrap_or(0);
        self.calls.push(TheCallOfTheScript {
            name: name.to_lowercase(),
            count: arguments.len(),
            blanks: the_groups_of_the_blanks(arguments),
            the_arity_is_checked,
            source,
            line,
            column,
            missing,
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

/// The groups of the arguments of a call that are not there: the place of the
/// first one of the group and the number of them.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
/// counts the arguments that are not there, and the place where they are
/// decides the complaint of the call: the `f(, 1)` is refused with "Blank
/// parameter" and the `f(1,)` is refused with "Too many parameters passed to
/// function." when f takes one parameter.
fn the_groups_of_the_blanks(arguments: &[Node]) -> Vec<(usize, usize)> {
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        if !matches!(arguments[index], Node::Empty { .. }) {
            index += 1;
            continue;
        }
        let start = index;
        while index < arguments.len() && matches!(arguments[index], Node::Empty { .. }) {
            index += 1;
        }
        groups.push((start, index - start));
    }
    groups
}

/// The complaint of the v1 about the list of the arguments of a call.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
/// refuses the argument that is not there with "Blank parameter", and the
/// place where it is decides when the complaint is that one and when it is
/// the number of the arguments. The `f(,)` and the `f(, 1)` are refused with
/// "Blank parameter", and the `f(1,)` is refused with "Too many parameters
/// passed to function." when f takes one parameter. The name that is not a
/// function is refused before this one: the `zzz(,)` is refused with "Call to
/// nonexistent function.".
fn the_complaint_of_the_arguments(
    call: &TheCallOfTheScript,
    least: usize,
    most: usize,
) -> Option<&'static str> {
    // The parameters that are done.
    let mut done = 0;
    // The argument that is read.
    let mut index = 0;
    for (start, size) in &call.blanks {
        // The parameters that came before the group and that are there.
        while index < *start {
            if done >= most {
                return Some("Too many parameters passed to function.");
            }
            done += 1;
            index += 1;
        }
        // The group of the arguments that are not there.
        if done >= most {
            return Some("Too many parameters passed to function.");
        }
        // The argument that is not there and that is the last one of the list.
        if start + size == call.count {
            return Some("Blank parameter");
        }
        // The argument that is not there and that comes before the least
        // number of the parameters.
        if done < least {
            return Some("Blank parameter");
        }
        done += size;
        index = start + size;
    }
    // The parameters that came after the last group.
    while index < call.count {
        if done >= most {
            return Some("Too many parameters passed to function.");
        }
        done += 1;
        index += 1;
    }
    if done < least {
        return Some("Too few parameters passed to function.");
    }
    None
}

/// Whether the value is a text that was written as a literal.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the operator that/// wants a number may not follow a literal text, and a variable that holds
/// one is not a literal. The concatenation of two literal texts is a
/// literal text, and the concatenation of a text and a number is not.
fn the_literal_is_a_text(node: &Node) -> bool {
    match node {
        Node::Literal {
            value: LiteralValue::Text(_),
            ..
        } => true,
        Node::Binary {
            op: BinaryOp::Concat,
            left,
            right,
            ..
        } => the_literal_is_a_text(left) && the_literal_is_a_text(right),
        _ => false,
    }
}

/// Whether the expression holds the operand that is not there.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// operand that is not there poisons the operation that holds it: the
/// `not (1 +)` is the empty text, and not the one that the `not` of the empty
/// text answers. The `&&` and the `||` are the exception, and the parser
/// keeps them: when the left side decides, the answer is one or zero.
fn the_operand_is_not_there(node: &Node) -> bool {
    match node {
        Node::Unset { .. } => true,
        Node::Group { items, .. } => items.iter().any(the_operand_is_not_there),
        Node::Unary { operand, .. } => the_operand_is_not_there(operand),
        Node::Binary { left, right, .. } => {
            the_operand_is_not_there(left) || the_operand_is_not_there(right)
        }
        Node::Ternary {
            condition,
            then,
            otherwise,
            ..
        } => {
            the_operand_is_not_there(condition)
                || the_operand_is_not_there(then)
                || the_operand_is_not_there(otherwise)
        }
        // The member of the value that holds the operand that is not there is
        // the operand that is not there: the `f(1 +).y` and the `f(1 +)[1]`.
        Node::Property { target, .. } => the_operand_is_not_there(target),
        Node::Index { target, index, .. } => {
            the_operand_is_not_there(target) || the_operand_is_not_there(index)
        }
        Node::MethodCall { target, .. } => the_operand_is_not_there(target),
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
        BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Power
            | BinaryOp::BitwiseAnd
    )
}

///
/// Whether the token is a whole number, and it may be the name of a
/// property.
///
/// The numbers of v2 that carry a point are not names: the point of them
/// is the point of a member.
fn the_number_is_a_whole_one(token: &Token, dialect: &'static dyn crate::dialect::Dialect) -> bool {
    token.kind == TokenKind::Number
        && matches!(
            the_number_of(&token.value, dialect),
            LiteralValue::Integer(_)
        )
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
fn the_literal_of_a_number(
    text: &str,
    dialect: &'static dyn crate::dialect::Dialect,
) -> LiteralValue {
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
        let Node::Call { arguments, .. } = &statements[0] else {
            panic!("a send")
        };
        assert!(
            matches!(&arguments[0], Node::Literal { value: LiteralValue::Text(text), .. } if text == "{Enter}")
        );
    }

    #[test]
    fn a_brace_that_opens_an_argument_opens_an_object() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the brace that
        // opens an argument of a command opens the literal of an object,
        // and not a key: `Send {Enter}` is refused at the load.
        let error = parse("Send {Enter}").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing \"propertyname:\" in object literal."
        );
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
        let Node::FunctionDef { parameters, .. } = &statements[0] else {
            panic!("a function")
        };
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
        let Node::If {
            branches,
            otherwise,
            ..
        } = &statements[0]
        else {
            panic!("an if")
        };
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
        let statements =
            parse("Switch x {\n  Case 1:\n    MsgBox \"one\"\n  Default:\n    MsgBox \"other\"\n}")
                .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else {
            panic!("a switch")
        };
        assert_eq!(cases.len(), 2);
        assert!(cases[1].values.is_empty());
    }

    #[test]
    fn a_case_may_name_many_values() {
        // Measured on the binary of v2.0.19 on 2026-11-01: a case may name
        // many values, and it matches when the subject of the switch is one
        // of them.
        let statements =
            parse("Switch x {\n  Case 1, 2, 3:\n    MsgBox \"one\"\n}").expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else {
            panic!("a switch")
        };
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].values.len(), 3);
    }

    #[test]
    fn a_case_of_v2_has_no_brace() {
        // Measured on the binary of v2.0.19 on 2026-10-31: the body of a
        // case is the statements under it, and a brace is not one.
        let error = parse("Switch x {\n  Case 1 {\n    MsgBox \"one\"\n  }\n}")
            .expect_err("does not parse");
        assert_eq!(
            error.message,
            "This line does not contain a recognized action.\n     Specifically: Case 1 {"
        );
    }

    #[test]
    fn a_case_can_be_written_with_a_colon() {
        let statements =
            parse("Switch x {\n  Case 1:\n    MsgBox \"one\"\n  Default:\n    MsgBox \"other\"\n}")
                .expect("parses");
        let Node::Switch { cases, .. } = &statements[0] else {
            panic!("a switch")
        };
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
        assert_eq!(
            error.message,
            "Missing \"{\"\n     Specifically: class Thing { x := 1 }"
        );
    }

    #[test]
    fn the_body_of_a_statement_begins_on_the_next_line() {
        let error = parse("if (x) { y := 1 }").expect_err("does not parse");
        assert_eq!(error.message, "Unexpected \"If\"");
    }

    #[test]
    fn the_brace_that_closes_a_block_is_alone_on_its_line() {
        let error = parse("if (x) {\n    y := 1 }").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Unexpected \"}\"\n     Specifically: y := 1 }"
        );
    }

    #[test]
    fn the_brace_that_opens_a_block_may_be_on_a_line_of_its_own() {
        let statements = parse("f()\n{\n    x := 1\n}").expect("parses");
        assert_eq!(statements.len(), 1);
    }

    #[test]
    fn a_function_definition_is_not_a_call() {
        let statements = parse("Add(a, b) {\n  return a + b\n}").expect("parses");
        let Node::FunctionDef {
            name, parameters, ..
        } = &statements[0]
        else {
            panic!("a definition")
        };
        assert_eq!(name, "Add");
        let names: Vec<&str> = parameters
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect();
        assert_eq!(names, vec!["a", "b"]);
        assert!(
            parameters
                .iter()
                .all(|parameter| parameter.default.is_none())
        );
        assert!(parameters.iter().all(|parameter| !parameter.variadic));
    }

    #[test]
    fn a_parameter_may_carry_a_default() {
        let statements = parse("Add(a, b := 1) {\n  return a + b\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else {
            panic!("a definition")
        };
        assert!(parameters[0].default.is_none());
        assert!(parameters[1].default.is_some());
    }

    #[test]
    fn a_parameter_may_take_the_rest_of_the_arguments() {
        let statements = parse("Add(a, rest*) {\n  return a\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else {
            panic!("a definition")
        };
        assert!(!parameters[0].variadic);
        assert!(parameters[1].variadic);
    }

    #[test]
    fn a_parameter_may_be_written_by_reference() {
        // The ByRef marker of v2: the caller must pass a reference, and
        // the parameter is an alias of the variable of the caller.
        let statements = parse("Add(&a) {\n  return a\n}").expect("parses");
        let Node::FunctionDef { parameters, .. } = &statements[0] else {
            panic!("a definition")
        };
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
        assert!(
            matches!(&statements[0], Node::Call { name, arguments, .. } if name == "class" && arguments.is_empty())
        );
    }

    #[test]
    fn two_values_side_by_side_need_a_space() {
        // Measured on the binary of v2.0.19 on 2026-10-31: two values side by
        // side are one value, and the space between them is what makes it so.
        let statements = parse("x := \"a\" \"b\"").expect("parses");
        assert!(
            matches!(&statements[0], Node::Assign { value, .. } if matches!(value.as_ref(), Node::Binary { op: BinaryOp::Concat, .. }))
        );
        let error = parse("x := \"a\"\"b\"").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing space or operator before this.\n     Specifically: \"b\""
        );
        let error = parse("x := \"a\"1").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing space or operator before this.\n     Specifically: 1"
        );
        // The text of the other kind is named with its own quote.
        let error = parse("x := \"a\"'b'").expect_err("does not parse");
        assert_eq!(
            error.message,
            "Missing space or operator before this.\n     Specifically: 'b'"
        );
    }

    #[test]
    fn a_call_in_a_statement_is_a_call() {
        let statements = parse("DoThing(1, 2)").expect("parses");
        assert!(
            matches!(&statements[0], Node::Call { name, arguments, .. } if name == "dothing" && arguments.len() == 2)
        );
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
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::Binary { op, right, .. } = value.as_ref() else {
            panic!("an addition")
        };
        assert_eq!(*op, BinaryOp::Add);
        assert!(matches!(
            right.as_ref(),
            Node::Binary {
                op: BinaryOp::Multiply,
                ..
            }
        ));
    }

    #[test]
    fn the_power_binds_to_the_right() {
        let statements = parse("x := 2 ** 3 ** 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::Binary { op, right, .. } = value.as_ref() else {
            panic!("a power")
        };
        assert_eq!(*op, BinaryOp::Power);
        assert!(matches!(
            right.as_ref(),
            Node::Binary {
                op: BinaryOp::Power,
                ..
            }
        ));
    }

    #[test]
    fn the_power_binds_tighter_than_the_unary_minus() {
        // Measured on the binary of v2.0.19 on 2026-10-28: `-2 ** 2` is
        // -4 there, and not 4.
        let statements = parse("x := -2 ** 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::Unary { op, operand, .. } = value.as_ref() else {
            panic!("a negation")
        };
        assert_eq!(*op, UnaryOp::Negate);
        assert!(matches!(
            operand.as_ref(),
            Node::Binary {
                op: BinaryOp::Power,
                ..
            }
        ));
    }

    #[test]
    fn the_word_not_binds_looser_than_the_comparison() {
        let statements = parse("x := not a = 3").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::Unary { op, operand, .. } = value.as_ref() else {
            panic!("a not")
        };
        assert_eq!(*op, UnaryOp::NotWord);
        assert!(matches!(
            operand.as_ref(),
            Node::Binary {
                op: BinaryOp::Equal,
                ..
            }
        ));
    }

    #[test]
    fn the_ternary_takes_the_branch_that_holds() {
        let statements = parse("x := a ? 1 : 2").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        assert!(matches!(value.as_ref(), Node::Ternary { .. }));
    }

    #[test]
    fn a_function_with_a_keyword_name_can_be_called() {
        let statements = parse("x := PixelGet(10, 20)").expect("parses");
        assert!(
            matches!(&statements[0], Node::Assign { value, .. } if matches!(value.as_ref(), Node::Call { name, .. } if name == "pixelget"))
        );
    }

    #[test]
    fn the_empty_brackets_are_the_empty_array() {
        let statements = parse("x := []").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn an_array_holds_its_values_in_the_order_they_are_written() {
        let statements = parse("x := [1, 2, 3]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 3);
        assert!(
            matches!(items[0].as_ref(), Some(Node::Literal { value: LiteralValue::Integer(number), .. }) if *number == 1)
        );
        assert!(
            matches!(items[2].as_ref(), Some(Node::Literal { value: LiteralValue::Integer(number), .. }) if *number == 3)
        );
    }

    #[test]
    fn an_array_holds_expressions() {
        let statements = parse("x := [a + b, c * d]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 2);
        assert!(matches!(
            items[0].as_ref(),
            Some(Node::Binary {
                op: BinaryOp::Add,
                ..
            })
        ));
        assert!(matches!(
            items[1].as_ref(),
            Some(Node::Binary {
                op: BinaryOp::Multiply,
                ..
            })
        ));
    }

    #[test]
    fn an_array_may_hold_another_array() {
        let statements = parse("x := [[1], [2]]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 2);
        assert!(
            matches!(items[0].as_ref(), Some(Node::ArrayLiteral { items, .. }) if items.len() == 1)
        );
    }

    #[test]
    fn an_array_may_hold_a_place_without_a_value() {
        // Measured on the binary of v2.0.19 on 2026-11-01: the literal may
        // have a hole, and the hole is a comma that follows a comma.
        let statements = parse("x := [1, , 3]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 3);
        assert!(items[1].is_none());
    }

    #[test]
    fn an_array_may_hold_many_places_without_a_value() {
        let statements = parse("x := [1, , , 2]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
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
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 2);
        let statements = parse("x := [,]").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::ArrayLiteral { items, .. } = value.as_ref() else {
            panic!("an array")
        };
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn the_suffixes_reach_into_an_array_literal() {
        // The literal is a primary, and the suffixes follow it the way
        // they follow a variable.
        let statements = parse("x := [1, 2].Length").expect("parses");
        let Node::Assign { value, .. } = &statements[0] else {
            panic!("an assignment")
        };
        let Node::Property { target, name, .. } = value.as_ref() else {
            panic!("a property")
        };
        assert_eq!(name, "Length");
        assert!(matches!(target.as_ref(), Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn an_array_may_be_an_argument_of_a_call() {
        let statements = parse("DoThing([1, 2])").expect("parses");
        let Node::Call { arguments, .. } = &statements[0] else {
            panic!("a call")
        };
        assert!(matches!(&arguments[0], Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn a_call_statement_may_take_an_array() {
        // The space is the reason: `DoThing[1, 2]` is the item of the
        // value `DoThing`, and `DoThing [1, 2]` is a call whose argument
        // is the array.
        let statements = parse("f(a) {\n}\nf [1, 2]").expect("parses");
        let Node::Call { arguments, .. } = &statements[1] else {
            panic!("a call")
        };
        assert!(matches!(&arguments[0], Node::ArrayLiteral { items, .. } if items.len() == 2));
    }

    #[test]
    fn a_bracket_without_a_space_is_the_item_of_the_value() {
        let statements = parse("x[1] := 2").expect("parses");
        let Node::AssignIndex { index, .. } = &statements[0] else {
            panic!("an index")
        };
        assert!(
            matches!(index.as_ref(), Node::Literal { value: LiteralValue::Integer(number), .. } if *number == 1)
        );
    }

    #[test]
    fn a_bracket_that_is_never_closed_is_an_error() {
        assert!(parse("x := [1, 2").is_err());
    }

    #[test]
    fn a_for_loop_walks_what_the_object_enumerates() {
        let statements = parse("for x in items {\n  total += x\n}").expect("parses");
        let Node::For {
            variables,
            source,
            body,
            ..
        } = &statements[0]
        else {
            panic!("a for")
        };
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0], "x");
        assert!(matches!(source.as_ref(), Node::Var { name, .. } if name == "items"));
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn a_for_loop_may_take_two_names() {
        let statements = parse("for key, value in items {\n}").expect("parses");
        let Node::For { variables, .. } = &statements[0] else {
            panic!("a for")
        };
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0], "key");
        assert_eq!(variables[1], "value");
    }

    #[test]
    fn a_for_loop_may_leave_the_first_name_out() {
        // The name that is left out is the empty string.
        let statements = parse("for , value in map {\n}").expect("parses");
        let Node::For { variables, .. } = &statements[0] else {
            panic!("a for")
        };
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0], "");
        assert_eq!(variables[1], "value");
    }

    #[test]
    fn a_for_loop_may_take_no_name_at_all() {
        let statements = parse("for in map {\n}").expect("parses");
        let Node::For {
            variables, source, ..
        } = &statements[0]
        else {
            panic!("a for")
        };
        assert!(variables.is_empty());
        assert!(matches!(source.as_ref(), Node::Var { name, .. } if name == "map"));
    }

    #[test]
    fn the_names_of_a_for_loop_may_be_written_between_parentheses() {
        let statements = parse("for (x in items) {\n}").expect("parses");
        let Node::For {
            variables,
            source,
            body,
            ..
        } = &statements[0]
        else {
            panic!("a for")
        };
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
        let Node::For {
            variables, body, ..
        } = &statements[0]
        else {
            panic!("a for")
        };
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
    let Node::Declare { scope, names, .. } = &statements[0] else {
        panic!("a declaration")
    };
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
    let Node::FunctionDef { declarations, .. } = &statements[0] else {
        panic!("a definition")
    };
    assert!(declarations.assume_global);
    assert!(declarations.globals.is_empty());
}

#[test]
fn a_class_with_an_init_and_variables_is_an_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: v2 makes the
    // `__Init` that carries the variables, and the two cannot both be
    // there.
    let error =
        parse("class Thing {\n  value := 1\n  __Init() {\n  }\n}").expect_err("does not parse");
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
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: Thing.__Init"
    );
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
    let error =
        parse("class Thing {\n  Foo() {\n  }\n  Foo() {\n  }\n}").expect_err("does not parse");
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
    let error = parse("class Thing {\n  static Foo() {\n  }\n  static Foo() {\n  }\n}")
        .expect_err("does not parse");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: Thing.Foo"
    );
    assert_eq!(error.line, 4);
}

#[test]
fn a_class_with_two_properties_of_the_same_name_is_an_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // property is the name of it, and not the name of the class with it.
    let error = parse("class Thing {\n  value := 1\n  value := 2\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: value"
    );
    assert_eq!(error.line, 3);
}

#[test]
fn a_class_with_two_static_properties_of_the_same_name_is_an_error() {
    let error = parse("class Thing {\n  static value := 1\n  static value := 2\n}")
        .expect_err("does not parse");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: value"
    );
    assert_eq!(error.line, 3);
}

#[test]
fn the_case_of_a_name_does_not_count() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the names of a
    // class are the names in lower case, and the failure names the one
    // that was written.
    let error =
        parse("class Thing {\n  Foo() {\n  }\n  foo() {\n  }\n}").expect_err("does not parse");
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
    let error = parse("class Thing {\n  Foo() {\n  }\n  Foo() {\n  }\n  static __Init() {\n  }\n}")
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
    let error = parse("class Thing {\n  __Init() {\n  }\n  __Init() {\n  }\n}")
        .expect_err("does not parse");
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
    assert!(
        matches!(&statements[0], Node::Assign { value, .. } if matches!(&**value, Node::Property { name, .. } if name == "x"))
    );
}

#[test]
fn a_name_that_is_a_statement_calls_the_method() {
    // Measured on the binary of v2.0.19 on 2026-10-29: a name that is a
    // statement is a call, and the parentheses may be left out.
    let statements = parse("o .x").expect("parses");
    assert!(
        matches!(&statements[0], Node::MethodCall { name, arguments, .. } if name == "x" && arguments.is_empty())
    );
}

#[test]
fn a_dot_with_a_space_after_it_joins_two_values() {
    let statements = parse("x := a . b").expect("parses");
    assert!(
        matches!(&statements[0], Node::Assign { value, .. } if matches!(&**value, Node::Binary { op: BinaryOp::Concat, .. }))
    );
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
    assert_eq!(
        error.message,
        "Unexpected declaration.\n     Specifically: global"
    );
}

#[test]
fn a_bare_local_is_an_error() {
    let error = parse("f() {\n  local\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected declaration.\n     Specifically: local"
    );
}

#[test]
fn a_local_outside_a_function_is_an_error() {
    let error = parse("local l").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected declaration.\n     Specifically: local l"
    );
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
    let Node::FunctionDef { declarations, .. } = &statements[0] else {
        panic!("a definition")
    };
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
    let error = parse("class C {\n    Make() {\n        class D {\n        }\n    }\n}\n")
        .expect_err("does not parse");
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
    let Node::ClassDef { name, nested, .. } = &statements[0] else {
        panic!("a class")
    };
    assert_eq!(name, "Outer");
    assert_eq!(nested.len(), 1);
    let Node::ClassDef { name, .. } = &nested[0] else {
        panic!("a class")
    };
    assert_eq!(name, "Outer.Inner");
}

#[test]
fn a_property_of_a_class_may_have_a_getter_and_a_setter() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the body of the
    // getter and of the setter is an expression after the arrow of v2,
    // and a block of statements between braces.
    let statements = parse("class C {\n    value {\n        get => 1\n    }\n}\n").expect("parses");
    let Node::ClassDef { accessors, .. } = &statements[0] else {
        panic!("a class")
    };
    assert_eq!(accessors.len(), 1);
    assert_eq!(accessors[0].name, "value");
    assert!(accessors[0].getter.is_some());
    assert!(accessors[0].setter.is_none());
    // The getter without a body is refused.
    let error =
        parse("class C {\n    value {\n        get\n    }\n}\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid property getter/setter.\n     Specifically: get"
    );
    // The duplicate of a property is refused.
    let error =
            parse("class C {\n    value {\n        get => 1\n    }\n    value {\n        get => 2\n    }\n}\n")
                .expect_err("does not parse");
    assert_eq!(
        error.message,
        "Duplicate declaration.\n     Specifically: value"
    );
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
    assert_eq!(
        error.message,
        "Label not found in current scope.\n     Specifically: done"
    );
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
    assert_eq!(
        error.message,
        "Unexpected reserved word.\n     Specifically: in"
    );
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
    assert_eq!(
        error.message,
        "Unexpected reserved word.\n     Specifically: as Integer"
    );
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
    let Node::Call {
        name, arguments, ..
    } = &statements[0]
    else {
        panic!("a call");
    };
    assert_eq!(name, "myfunc");
    assert_eq!(arguments.len(), 1);
    assert!(matches!(&arguments[0], Node::Group { items, .. } if items.len() == 2));
    // The call that is written without the space keeps its arguments.
    let statements = parse("MyFunc(\"a\", \"b\")").expect("parses");
    let Node::Call {
        name, arguments, ..
    } = &statements[0]
    else {
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
    let Node::Call {
        name, arguments, ..
    } = &statements[0]
    else {
        panic!("a call");
    };
    assert_eq!(name, "myfunc");
    assert_eq!(arguments.len(), 3);
    assert!(matches!(&arguments[1], Node::Empty { .. }));
    // The commas at the end of the parameters add nothing.
    let statements = parse("MyFunc \"a\",,").expect("parses");
    let Node::Call {
        name, arguments, ..
    } = &statements[0]
    else {
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
    assert!(matches!(
        &**value,
        Node::Binary {
            op: BinaryOp::Add,
            ..
        }
    ));
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
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
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
    assert_eq!(
        error.message,
        "Missing \"}\"\n     Specifically: Two (a, b) {"
    );
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
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: #c"
    );
    let error = parse("x := 1;c").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: ;c"
    );
    // The one that is inside a parenthesis names the text of the
    // expression that holds it.
    let error = parse("if (x#c) {").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: #c)"
    );
    // The one that begins a statement is a line that holds no action.
    let error = parse("f() {\n}#c").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: #c"
    );
    // The one that is glued to the name that opens the statement is a
    // part of it, and the whole line is named.
    let error = parse("x#y := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: x#y := 1"
    );
}

#[test]
fn the_brace_that_opens_a_line_opens_a_block() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
    // opens a line is a block of its own, and the statements between the
    // braces run. The content may be on the line of the brace that opens
    // it, and the brace that closes it is alone in its line -- unless the
    // block is empty, and then the brace may be written where it is.
    let statements = parse("{\n}").expect("parses");
    let Node::Block { body, .. } = &statements[0] else {
        panic!("a block")
    };
    assert!(body.is_empty());
    let statements = parse("{\nx := 1\n}").expect("parses");
    let Node::Block { body, .. } = &statements[0] else {
        panic!("a block")
    };
    assert_eq!(body.len(), 1);
    // The empty block may be written in one line.
    parse("{}").expect("parses");
    parse("{ }").expect("parses");
    // The content may be on the line of the brace that opens it.
    let statements = parse("{x := 1\n}").expect("parses");
    let Node::Block { body, .. } = &statements[0] else {
        panic!("a block")
    };
    assert_eq!(body.len(), 1);
}

#[test]
fn the_brace_that_closes_the_block_is_alone_in_its_line() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the brace that
    // closes the block is alone in its line, and the v2 says so before it
    // reads the content of the block. The text that is named is the line
    // of the brace.
    let error = parse("{x := 1}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected \"}\"\n     Specifically: x := 1}"
    );
    let error = parse("{\nx := 1 }").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected \"}\"\n     Specifically: x := 1 }"
    );
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
    assert_eq!(
        error.message,
        "Missing \"}\"\n     Specifically: x := { a: 1"
    );
    assert_eq!(error.line, 1);
    // The property that is not there does not hide the brace that is
    // not there.
    let error = parse("Send {\nEnter").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"}\"\n     Specifically: Send { Enter"
    );
    // The failures of the literal are named with the line of the brace
    // that opens it, and not with the line of the property that failed.
    let error = parse("x := {\na\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
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

/// The two bounds of the `between` of the legacy `if` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the text
/// is cut at the word `and`, and the word is the word of the dialect and
/// not the case of the script. The bounds are the texts that come around
/// it, and the spaces around them are not a part of them.
fn the_bounds_of_a_between(text: &str) -> Option<(String, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let start = at;
        while at < chars.len() && (chars[at].is_ascii_alphanumeric() || chars[at] == '_') {
            at += 1;
        }
        if at > start {
            let word: String = chars[start..at].iter().collect();
            if word.eq_ignore_ascii_case("and") {
                let low: String = chars[..start].iter().collect();
                let high: String = chars[at..].iter().collect();
                return Some((low.trim().to_string(), high.trim().to_string()));
            }
        }
        if at == start {
            at += 1;
        }
    }
    None
}

/// The items of the list of the `in` and of the `contains` of the legacy
/// `if` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the list
/// is cut at the commas, and the spaces around them are a part of the item:
/// the `if x in a, b` asks whether the value of `x` is "a" or " b". Two
/// commas in a row are one comma in the item, and the item goes on: the
/// `if x in a,,b` asks whether the value of `x` is "a,b".
fn the_items_of_a_list(text: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut item = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let character = chars[at];
        if character == ',' {
            if chars.get(at + 1) == Some(&',') {
                // Two commas in a row are one comma in the item, and the
                // item goes on.
                item.push(',');
                at += 2;
                continue;
            }
            items.push(core::mem::take(&mut item));
            at += 1;
            continue;
        }
        item.push(character);
        at += 1;
    }
    items.push(item);
    items
}
