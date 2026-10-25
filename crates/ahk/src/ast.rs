//! The AST: what a script means once it has been parsed.

use serde::Serialize;

/// A value written in the script.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LiteralValue {
    /// A number: `42`, `3.5`.
    Number(f64),
    /// A string: `"hello"`.
    Text(String),
    /// A boolean: `true`, `false`.
    Bool(bool),
}

/// The operators of the expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum UnaryOp {
    Not,
    Negate,
}

/// The operators with two sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BinaryOp {
    /// `=`, the equality that AutoHotkey writes.
    Equal,
    /// `!=`.
    NotEqual,
    /// `<`, `>`, `<=`, `>=`.
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    /// `+`.
    Add,
    /// `-`.
    Subtract,
    /// `*`.
    Multiply,
    /// `/`.
    Divide,
    /// `.`, the string concatenation.
    Concat,
    /// `and`, `&&`.
    And,
    /// `or`, `||`.
    Or,
}

/// The assignment operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AssignOp {
    /// `=` and `:=` mean the same here.
    Set,
    /// `+=`.
    Add,
    /// `-=`.
    Subtract,
    /// `.=`.
    Concat,
}

/// One node of the tree.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    /// A value.
    Literal { value: LiteralValue, line: u32 },
    /// A variable.
    Var { name: String, line: u32 },
    /// A key inside a `Send`, written `{Name}`.
    KeyName { name: String, line: u32 },
    /// `not x`, `-x`.
    Unary { op: UnaryOp, operand: Box<Node>, line: u32 },
    /// A binary operation.
    Binary { op: BinaryOp, left: Box<Node>, right: Box<Node>, line: u32 },
    /// `x := 1`.
    Assign { name: String, op: AssignOp, value: Box<Node>, line: u32 },
    /// `Send "a{Enter}b"`.
    Send { items: Vec<Node>, line: u32 },
    /// `Click 40, 50`.
    Click { x: Box<Node>, y: Box<Node>, button: String, line: u32 },
    /// `Sleep 500`.
    Sleep { millis: Box<Node>, line: u32 },
    /// `Run "cmd"`.
    Run { command: Box<Node>, window: String, line: u32 },
    /// `PixelGet 10, 20`.
    PixelGet { x: Box<Node>, y: Box<Node>, line: u32 },
    /// `Screenshot 0, 0, 100, 50, "shot.png"`.
    Screenshot {
        x: Box<Node>,
        y: Box<Node>,
        width: Box<Node>,
        height: Box<Node>,
        filename: Box<Node>,
        line: u32,
    },
    /// `Msg "text"`.
    Msg { text: Box<Node>, line: u32 },
    /// A function call: `PixelGet(10, 20)`.
    Call { name: String, arguments: Vec<Node>, line: u32 },
    /// `If cond { } Else { }`.
    If {
        condition: Box<Node>,
        then_body: Vec<Node>,
        otherwise: Vec<Node>,
        line: u32,
    },
    /// `While cond { }`.
    While { condition: Box<Node>, body: Vec<Node>, line: u32 },
    /// `Loop 3 { }`.
    Loop { count: Option<Box<Node>>, body: Vec<Node>, line: u32 },
    /// `Break`.
    Break { line: u32 },
    /// `Continue`.
    Continue { line: u32 },
}

impl Node {
    /// The line the node came from.
    pub fn line(&self) -> u32 {
        match self {
            Node::Literal { line, .. }
            | Node::Var { line, .. }
            | Node::KeyName { line, .. }
            | Node::Unary { line, .. }
            | Node::Binary { line, .. }
            | Node::Assign { line, .. }
            | Node::Send { line, .. }
            | Node::Click { line, .. }
            | Node::Sleep { line, .. }
            | Node::Run { line, .. }
            | Node::PixelGet { line, .. }
            | Node::Screenshot { line, .. }
            | Node::Msg { line, .. }
            | Node::Call { line, .. }
            | Node::If { line, .. }
            | Node::While { line, .. }
            | Node::Loop { line, .. }
            | Node::Break { line }
            | Node::Continue { line } => *line,
        }
    }
}
