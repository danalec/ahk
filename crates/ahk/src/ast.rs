//! The AST: what a script means once it has been parsed.

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use serde::Serialize;

/// A value written in the script.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LiteralValue {
    /// A whole number: `42`, `0x1F`.
    ///
    /// The whole number of v2 is a whole number of the machine, and it
    /// keeps every digit of it.
    Integer(i64),
    /// A number with a point or with an exponent: `3.5`, `1e3`.
    Float(f64),
    /// A string: `"hello"`.
    Text(String),
    /// A boolean: `true`, `false`.
    ///
    /// AutoHotkey v2 has no boolean type: `true` is the integer 1. The
    /// literal keeps its own kind so that the parser can tell `true`
    /// from `1`, and the runtime folds it into the integer.
    Bool(bool),
}

/// The operators with one side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum UnaryOp {
    /// `!`, the logical not.
    Not,
    /// `not`, the logical not with the lower precedence.
    ///
    /// The two are the same operation; the word binds looser, and v2
    /// keeps them apart for exactly that reason.
    NotWord,
    /// `-`, the unary minus.
    Negate,
    /// `+`, the unary plus.
    Plus,
    /// `~`, the bitwise not.
    BitwiseNot,
    /// `&`, the reference to a variable.
    Address,
    /// `++`, before the name.
    PreIncrement,
    /// `--`, before the name.
    PreDecrement,
    /// `++`, after the name.
    PostIncrement,
    /// `--`, after the name.
    PostDecrement,
}

/// The operators with two sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BinaryOp {
    /// `+`.
    Add,
    /// `-`.
    Subtract,
    /// `*`.
    Multiply,
    /// `/`, the true divide: it always answers a float.
    Divide,
    /// `//`, the integer divide.
    IntegerDivide,
    /// `**`, the power. It binds to the right.
    Power,
    /// `&`, the bitwise and.
    BitwiseAnd,
    /// `|`, the bitwise or.
    BitwiseOr,
    /// `^`, the bitwise exclusive or.
    BitwiseXor,
    /// `<<`.
    ShiftLeft,
    /// `>>`, the arithmetic shift.
    ShiftRight,
    /// `>>>`, the logical shift.
    ShiftRightUnsigned,
    /// `.`, the string concatenation.
    Concat,
    /// `=`, the equality that ignores the case of the letters.
    Equal,
    /// `!=`, the negation of `=`.
    NotEqual,
    /// `==`, the equality that does not ignore the case.
    StrictEqual,
    /// `!==`, the negation of `==`.
    StrictNotEqual,
    /// `<`.
    Less,
    /// `>`.
    Greater,
    /// `<=`.
    LessEqual,
    /// `>=`.
    GreaterEqual,
    /// `~=`, the shorthand for `RegExMatch`.
    RegexMatch,
    /// `and`, `&&`.
    And,
    /// `or`, `||`.
    Or,
    /// `is`, the test for an instance of a class.
    Is,
    /// `??`, the or-maybe: the right side when the left has no value.
    Coalesce,
}

/// The kind of a declaration of v2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DeclareKind {
    /// `global`: the name lives in the scope of the script.
    Global,
    /// `local`: the name lives in the scope of the function.
    Local,
}

impl DeclareKind {
    /// The word of the dialect that writes the declaration.
    pub fn word(self) -> &'static str {
        match self {
            DeclareKind::Global => "global",
            DeclareKind::Local => "local",
        }
    }
}

/// A name of a declaration, and the value it starts with.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Declared {
    /// The name of the variable.
    pub name: String,
    /// The value it starts with, when the declaration gives it one.
    pub value: Option<Box<Node>>,
    /// The text of the declaration, the way v2 writes it when it refuses.
    pub text: String,
}

/// The declarations of a function or of a method.
///
/// v2 reads them when the script is read: a declaration inside a block
/// takes effect for the whole function, and only the value of the
/// declaration waits for the statement that carries it.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Declarations {
    /// The names that live in the scope of the script.
    pub globals: BTreeSet<String>,
    /// The names that live in the scope of the function.
    pub locals: BTreeSet<String>,
    /// Whether the function assumes the scope of the script.
    pub assume_global: bool,
}

/// The definition of a method.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MethodDef {
    /// The name of the method.
    pub name: String,
    /// The parameters, in the order they are given.
    pub parameters: Vec<Parameter>,
    /// What the method does.
    pub body: Vec<Node>,
    /// The declarations of the method.
    pub declarations: Declarations,
    /// Whether the method belongs to the class and not to an instance.
    pub is_static: bool,
    /// The line the definition came from.
    pub line: u32,
}

/// The definition of a property.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PropertyDef {
    /// The name of the property.
    pub name: String,
    /// The value the property starts with.
    pub value: Option<Box<Node>>,
    /// Whether the property belongs to the class and not to an instance.
    pub is_static: bool,
    /// The line the definition came from.
    pub line: u32,
}

/// The kinds of loop.
///
/// The plain loop counts; the others walk something: the files of a
/// directory, the lines of a file, the pieces of a text, the keys of a
/// registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LoopKind {
    /// `Loop 3 { }`: the body runs the number of times that is asked for.
    Count,
    /// `Loop Files "*.txt" { }`.
    Files,
    /// `Loop Reg "key" { }`.
    Reg,
    /// `Loop Read "file.txt" { }`.
    Read,
    /// `Loop Parse "a,b,c" { }`.
    Parse,
}

/// The assignment operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AssignOp {
    /// `:=`.
    Set,
    /// `+=`.
    Add,
    /// `-=`.
    Subtract,
    /// `*=`.
    Multiply,
    /// `/=`.
    Divide,
    /// `//=`.
    IntegerDivide,
    /// `.=`.
    Concat,
    /// `&=`.
    BitwiseAnd,
    /// `|=`.
    BitwiseOr,
    /// `^=`.
    BitwiseXor,
    /// `<<=`.
    ShiftLeft,
    /// `>>=`.
    ShiftRight,
    /// `>>>=`.
    ShiftRightUnsigned,
}

/// The types a value can be cast to with `as`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CastType {
    /// `as Integer`.
    Integer,
    /// `as Float`.
    Float,
    /// `as String`.
    String,
    /// `as Number`.
    Number,
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

/// One `catch` of a `try`.
///
/// The class says what the catch takes: the class of the errors of the
/// runtime when the catch names none, and `any` takes everything.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Catch {
    /// The class of the value that the catch takes.
    pub class: String,
    /// The variable that holds the value that was thrown.
    pub variable: Option<String>,
    /// What runs when the catch is the one that took the value.
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
    /// `not x`, `-x`.
    Unary { op: UnaryOp, operand: Box<Node>, line: u32 },
    /// A binary operation.
    Binary { op: BinaryOp, left: Box<Node>, right: Box<Node>, line: u32 },
    /// `cond ? a : b`.
    Ternary { condition: Box<Node>, then: Box<Node>, otherwise: Box<Node>, line: u32 },
    /// `x as Integer`.
    Cast { value: Box<Node>, target: CastType, line: u32 },
    /// `x := 1`.
    Assign { name: String, op: AssignOp, value: Box<Node>, line: u32 },
    /// `list[1]`, `text[2]`: the item at a position.
    ///
    /// The positions count from one, the way AutoHotkey counts them.
    Index { target: Box<Node>, index: Box<Node>, line: u32 },
    /// A call: `PixelGetColor(10, 20)`.
    ///
    /// A command is a call written without parentheses: `Send "hello"`
    /// is the same built-in with its arguments in the same order.
    Call { name: String, arguments: Vec<Node>, line: u32 },
    /// `If cond { } ElseIf cond { } Else { }`.
    If { branches: Vec<Branch>, otherwise: Vec<Node>, line: u32 },
    /// `Switch value { Case 1 { } Default { } }`.
    Switch { subject: Box<Node>, cases: Vec<Case>, line: u32 },
    /// `While cond { }`.
    While { condition: Box<Node>, body: Vec<Node>, line: u32 },
    /// `Loop 3 { }`, `Loop Files "*.txt" { }`, `Loop Parse "a,b" { }`.
    Loop { mode: LoopKind, arguments: Vec<Node>, body: Vec<Node>, line: u32 },
    /// `#Requires AutoHotkey v2.0.19`: a directive.
    ///
    /// The name is the name of the directive and the argument is the
    /// rest of the line, which is what the directive is about.
    Directive { name: String, argument: String, line: u32 },
    /// `Break`.
    Break { line: u32 },
    /// `Continue`.
    Continue { line: u32 },
    /// `Return` or `Return value`.
    Return { value: Option<Box<Node>>, line: u32 },
    /// `ExitApp` or `ExitApp 1`.
    ExitApp { code: Option<Box<Node>>, line: u32 },
    /// `Name(a, b) { }`: the definition of a function.
    FunctionDef {
        name: String,
        parameters: Vec<Parameter>,
        body: Vec<Node>,
        declarations: Declarations,
        line: u32,
    },
    /// `super`: the base of the class where the method was defined.
    ///
    /// It is not a value: it is followed by a member.
    Super { line: u32 },
    /// `global name` or `local name`: a declaration of v2.
    ///
    /// The names are read when the script is read, and the values wait for
    /// the statement that carries them.
    Declare { scope: DeclareKind, names: Vec<Declared>, line: u32 },
    /// `class Name extends Base { }`: the definition of a class.
    ClassDef {
        name: String,
        extends: Option<String>,
        methods: Vec<MethodDef>,
        properties: Vec<PropertyDef>,
        line: u32,
    },
    /// `obj.Method(args)`: a method called on a value.
    MethodCall { target: Box<Node>, name: String, arguments: Vec<Node>, line: u32 },
    /// `obj.Prop`: a property of a value.
    Property { target: Box<Node>, name: String, line: u32 },
    /// `obj.Prop := value`: a property of a value that is written.
    AssignProperty {
        target: Box<Node>,
        name: String,
        op: AssignOp,
        value: Box<Node>,
        line: u32,
    },
    /// `obj[key] := value`: an item of a value that is written.
    ///
    /// v2 writes an item of a map and of an array, and a class of the
    /// script has no way to write one: the meta-functions answer for the
    /// properties that are not there, and `x[y]` asks only the `__Item`
    /// property.
    AssignIndex {
        target: Box<Node>,
        index: Box<Node>,
        op: AssignOp,
        value: Box<Node>,
        line: u32,
    },
    /// `[a, b, c]`: the literal of an array.
    ///
    /// v2 writes an array between brackets, and the empty brackets are
    /// the array of nothing. The values go in the order they are
    /// written, and the places of an array count from one.
    ArrayLiteral { items: Vec<Node>, line: u32 },
    /// `try { } catch { } else { } finally { }`.
    ///
    /// The block of the `try` is guarded against the errors of the runtime
    /// and the values that `throw` throws. The catches take the values
    /// whose class they name, the `else` runs when nothing was thrown, and
    /// the `finally` always runs.
    Try {
        /// The statements that are guarded.
        body: Vec<Node>,
        /// The catches, in the order they are written.
        catches: Vec<Catch>,
        /// What runs when nothing was thrown.
        otherwise: Vec<Node>,
        /// What runs always.
        finally: Vec<Node>,
        /// The line of the `try`.
        line: u32,
    },
    /// `throw value`, or `throw` alone to throw again what was caught.
    Throw { value: Option<Box<Node>>, line: u32 },
    /// `for x, y in z { }`: the loop over what an object enumerates.
    ///
    /// The variables are the ones the enumerator gives, and a name that
    /// is left out is the one the enumerator does not give: `for , value
    /// in map` asks for the value alone.
    For { variables: Vec<String>, source: Box<Node>, body: Vec<Node>, line: u32 },
}

/// One parameter of a function.
///
/// A parameter is a name. It may carry a default -- `Name(a, b := 1)` --
/// it may be the one that takes the rest of the arguments --
/// `Name(a, rest*)` -- and it may carry the ByRef marker of v2 --
/// `Name(a, &b)` -- which says that the caller must pass a reference and
/// that the parameter is an alias of the variable of the caller.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The value it takes when the call does not give it one.
    pub default: Option<Box<Node>>,
    /// Whether it takes the rest of the arguments, as an array.
    pub variadic: bool,
    /// Whether it is written `&x` in the definition.
    ///
    /// v2 calls it the ByRef marker: the caller must pass a reference,
    /// and the parameter is an alias of the variable of the caller.
    pub by_ref: bool,
}

impl Node {
    /// The line the node came from.
    pub fn line(&self) -> u32 {
        match self {
            Node::Literal { line, .. }
            | Node::Var { line, .. }
            | Node::Unary { line, .. }
            | Node::Binary { line, .. }
            | Node::Ternary { line, .. }
            | Node::Cast { line, .. }
            | Node::Assign { line, .. }
            | Node::AssignIndex { line, .. }
            | Node::Index { line, .. }
            | Node::Call { line, .. }
            | Node::If { line, .. }
            | Node::Switch { line, .. }
            | Node::While { line, .. }
            | Node::Loop { line, .. }
            | Node::Directive { line, .. }
            | Node::Break { line }
            | Node::Continue { line }
            | Node::Return { line, .. }
            | Node::ExitApp { line, .. }
            | Node::FunctionDef { line, .. }
            | Node::ClassDef { line, .. }
            | Node::MethodCall { line, .. }
            | Node::Property { line, .. }
            | Node::AssignProperty { line, .. }
            | Node::ArrayLiteral { line, .. }
            | Node::Try { line, .. }
            | Node::Throw { line, .. }
            | Node::Declare { line, .. }
            | Node::Super { line, .. }
            | Node::For { line, .. } => *line,
        }
    }
}
