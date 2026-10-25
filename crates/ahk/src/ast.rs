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

/// One branch of an `If`: a condition and what happens when it holds.
///
/// An `If` with `ElseIf` branches is one node with a list of them, because
/// the chain is a single decision: the first condition that holds wins, and
/// the `Else` is the last resort.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Branch {
    /// The condition.
    pub condition: Node,
    /// What runs when it holds.
    pub body: Vec<Node>,
}

/// One `Case` of a `Switch`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Case {
    /// The value to match; `None` is the `Default`, which matches whatever
    /// no other case matched.
    pub value: Option<Node>,
    /// What runs when the case is the one that matched.
    pub body: Vec<Node>,
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
    ///
    /// The modifiers are the ones that came with it: `^` is ctrl, `+` is
    /// shift, `!` is alt and `#` is the super key.
    KeyName { name: String, modifiers: Vec<String>, line: u32 },
    /// `{Name down}`: the key goes down and stays down.
    KeyDown { name: String, line: u32 },
    /// `{Name up}`: the key comes back up.
    KeyUp { name: String, line: u32 },
    /// `not x`, `-x`.
    Unary { op: UnaryOp, operand: Box<Node>, line: u32 },
    /// A binary operation.
    Binary { op: BinaryOp, left: Box<Node>, right: Box<Node>, line: u32 },
    /// `x := 1`.
    Assign { name: String, op: AssignOp, value: Box<Node>, line: u32 },
    /// `list[1]`, `text[2]`: the item at a position.
    ///
    /// The positions count from one, the way AutoHotkey counts them.
    Index { target: Box<Node>, index: Box<Node>, line: u32 },
    /// `Send "a{Enter}b"`.
    Send { items: Vec<Node>, line: u32 },
    /// `Click 40, 50`.
    ///
    /// The button and the count are expressions, the way AutoHotkey
    /// writes them: `Click 40, 50, right, 2` is a double right click.
    Click { x: Box<Node>, y: Box<Node>, button: Box<Node>, count: Box<Node>, line: u32 },
    /// `MouseDown`, with the button and the place it goes to.
    MouseDown { x: Option<Box<Node>>, y: Option<Box<Node>>, button: Box<Node>, line: u32 },
    /// `MouseUp`, with the button and the place it comes up at.
    MouseUp { x: Option<Box<Node>>, y: Option<Box<Node>>, button: Box<Node>, line: u32 },
    /// `MouseMove 40, 50`.
    MouseMove { x: Box<Node>, y: Box<Node>, line: u32 },
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
    /// `If cond { } ElseIf cond { } Else { }`.
    If { branches: Vec<Branch>, otherwise: Vec<Node>, line: u32 },
    /// `Switch value { Case 1 { } Default { } }`.
    Switch { subject: Box<Node>, cases: Vec<Case>, line: u32 },
    /// `While cond { }`.
    While { condition: Box<Node>, body: Vec<Node>, line: u32 },
    /// `Loop 3 { }`.
    Loop { count: Option<Box<Node>>, body: Vec<Node>, line: u32 },
    /// `Break`.
    Break { line: u32 },
    /// `Continue`.
    Continue { line: u32 },
    /// `Return` or `Return value`.
    Return { value: Option<Box<Node>>, line: u32 },
    /// `ExitApp` or `ExitApp 1`.
    ExitApp { code: Option<Box<Node>>, line: u32 },
    /// `Name(a, b) { }`: the definition of a function.
    FunctionDef { name: String, parameters: Vec<String>, body: Vec<Node>, line: u32 },
}

impl Node {
    /// The line the node came from.
    pub fn line(&self) -> u32 {
        match self {
            Node::Literal { line, .. }
            | Node::Var { line, .. }
            | Node::KeyName { line, .. }
            | Node::KeyDown { line, .. }
            | Node::KeyUp { line, .. }
            | Node::Unary { line, .. }
            | Node::Binary { line, .. }
            | Node::Assign { line, .. }
            | Node::Index { line, .. }
            | Node::Send { line, .. }
            | Node::Click { line, .. }
            | Node::MouseDown { line, .. }
            | Node::MouseUp { line, .. }
            | Node::MouseMove { line, .. }
            | Node::Sleep { line, .. }
            | Node::Run { line, .. }
            | Node::PixelGet { line, .. }
            | Node::Screenshot { line, .. }
            | Node::Msg { line, .. }
            | Node::Call { line, .. }
            | Node::If { line, .. }
            | Node::Switch { line, .. }
            | Node::While { line, .. }
            | Node::Loop { line, .. }
            | Node::Break { line }
            | Node::Continue { line }
            | Node::Return { line, .. }
            | Node::ExitApp { line, .. }
            | Node::FunctionDef { line, .. } => *line,
        }
    }
}
