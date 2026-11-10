//! The AST: what a script means once it has been parsed.

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;


/// A value written in the script.
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    /// A whole number: `42`, `0x1F`.
    ///
    /// The whole number of v2 is a whole number of the machine, and it
    /// keeps every digit of it.
    Integer(i64),
    /// A number with a point or with an exponent: `3.5`, `1e3`.
    Float(f64),
    /// The number of the v1: the text that was written.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// number of the v1 is a "pure number": it keeps the text that was
    /// written -- `0x10` is "0x10", and `00123` is "00123", and `1.50` is
    /// "1.50" -- and that text is what the value answers when it is read
    /// as a text. The value of it is the number that the text holds, and
    /// the operations that want a number ask the text for it.
    Number(String),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// The word that is an operator and that has no left operand: the `and`
    /// and the `or` of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `x := and 2` is one, and the `x := or 2` is one: the word takes the
    /// value of the right side, and it answers the truth of it as a number.
    Boolean,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclareKind {
    /// `global`: the name lives in the scope of the script.
    Global,
    /// `local`: the name lives in the scope of the function.
    Local,
    /// `static`: the name lives in the storage of the function, and it keeps
    /// its value between the calls.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the static of a
    /// function is the same one in every call of it, and the value of the
    /// declaration runs once.
    Static,
}

impl DeclareKind {
    /// The word of the dialect that writes the declaration.
    pub fn word(self) -> &'static str {
        match self {
            DeclareKind::Global => "global",
            DeclareKind::Local => "local",
            DeclareKind::Static => "static",
        }
    }
}

/// A name of a declaration, and the value it starts with.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Declarations {
    /// The names that live in the scope of the script.
    pub globals: BTreeSet<String>,
    /// The names that live in the scope of the function.
    pub locals: BTreeSet<String>,
    /// The names that live in the storage of the function, and that keep
    /// their value between the calls.
    pub statics: BTreeSet<String>,
    /// Whether the function assumes the scope of the script.
    pub assume_global: bool,
}

/// The definition of a method.
#[derive(Debug, Clone, PartialEq)]
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
    /// The line of the brace that opens the body.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// names this line when it warns about a local that has the name of a
    /// global one.
    pub body_line: u32,
}

/// The definition of a property.
#[derive(Debug, Clone, PartialEq)]
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

/// The body of a getter or of a setter.
#[derive(Debug, Clone, PartialEq)]
pub struct AccessBody {
    /// What the body does.
    pub body: Vec<Node>,
    /// The declarations of the body.
    pub declarations: Declarations,
    /// The line of the brace that opens the body.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// names this line when it warns about a local that has the name of a
    /// global one.
    pub line: u32,
}

/// The definition of a property that has a getter or a setter.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: a class may declare a
/// property with a getter and a setter, and the property is a member of the
/// prototype of the class.
#[derive(Debug, Clone, PartialEq)]
pub struct AccessDef {
    /// The name of the property.
    pub name: String,
    /// The parameters that the property takes.
    pub parameters: Vec<Parameter>,
    /// The getter of the property.
    pub getter: Option<AccessBody>,
    /// The setter of the property.
    pub setter: Option<AccessBody>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// One branch of an `If`: a condition and what happens when it holds.
///
/// An `If` with `else if` branches is one node with a list of them, because
/// the chain is a single decision: the first condition that holds wins, and
/// the `else` is the last resort.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: the branch of an `if` is
/// written with two words, and the `elseif` is a name like any other.
#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    /// The condition.
    pub condition: Node,
    /// The items that come after the comma of the line of the condition.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// items are expressions, and they are evaluated whether the condition is
    /// true or false: the `if (0) x := 1, y := 3` writes the variable "y".
    /// The value of them does not count for the truth of the condition, and
    /// the command of the v1 is not run by them: the `if (0) x := 1,
    /// FileAppend, A1, done` does not write the file.
    pub tail: Vec<Node>,
    /// What runs when it holds.
    pub body: Vec<Node>,
}

/// One `Case` of a `Switch`.
#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    /// The values to match; an empty list is the `Default`, which matches
    /// whatever no other case matched.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: a case may name
    /// many values, and it matches when the subject is one of them.
    pub values: Vec<Node>,
    /// What runs when the case is the one that matched.
    pub body: Vec<Node>,
}

/// One `catch` of a `try`.
///
/// The class says what the catch takes: the class of the errors of the
/// runtime when the catch names none, and `any` takes everything.
#[derive(Debug, Clone, PartialEq)]
pub struct Catch {
    /// The class of the value that the catch takes.
    pub class: String,
    /// The variable that holds the value that was thrown.
    pub variable: Option<String>,
    /// What runs when the catch is the one that took the value.
    pub body: Vec<Node>,
}

/// A piece of a name that the script builds.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the name of the dynamic
/// variable reference is the text that is written and the values of the
/// expressions, joined without a space between them.
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicPiece {
    /// The text that is written.
    Text(String),
    /// The value of an expression, as text.
    Value(Box<Node>),
}

/// The name of a member: the text that is written, or the pieces of a name
/// that the script builds.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: `m.%x%` is the member
/// whose name is the value of `x`.
#[derive(Debug, Clone, PartialEq)]
pub enum MemberName {
    /// The name that is written.
    Text(String),
    /// The name that the script builds.
    Dynamic(Vec<DynamicPiece>),
}

impl PartialEq<str> for MemberName {
    fn eq(&self, other: &str) -> bool {
        match self {
            MemberName::Text(text) => text == other,
            MemberName::Dynamic(_) => false,
        }
    }
}

/// An item of the literal of an object.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// literal of the v1 is a list of parameters, and the number of them has to
/// be even: the key of a property and its value are two, and a parameter
/// that is in blank is one. A parameter that has no name is a value that is
/// evaluated, and it is dropped: the `{, 1}` is an object of no property.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectItem {
    /// A property: the name of it and the value.
    Property(MemberName, Node),
    /// A property whose key is the value of an expression: the key and the
    /// value.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the key
    /// of the v1 is the value of the expression that is written until the
    /// colon, and not the text of it: the `{1 + 1: 1}` has the property "2",
    /// and the `{a.b: 1}` has the property of the value of `a.b`. A key that
    /// is a name is the name itself: the `{b: 1}` has the property "b",
    /// whatever the value of `b` is.
    LegacyKey(Node, Node),
    /// A value that has no name: it is evaluated, and it is dropped.
    Value(Node),
}

/// One node of the tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// A value.
    Literal { value: LiteralValue, line: u32 },
    /// A variable.
    Var { name: String, line: u32 },
    /// `not x`, `-x`.
    Unary {
        op: UnaryOp,
        operand: Box<Node>,
        line: u32,
    },
    /// A binary operation.
    Binary {
        op: BinaryOp,
        left: Box<Node>,
        right: Box<Node>,
        line: u32,
    },
    /// The operand that is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse the expression whose operand is not there: the `x := 1 +`
    /// is accepted, and the value of the expression is the empty text. The
    /// operand that is not there poisons the operation that holds it, and the
    /// `&&` and the `||` are the exception: when the left side decides, the
    /// answer is one or zero.
    Unset { line: u32 },
    /// `cond ? a : b`.
    Ternary {
        condition: Box<Node>,
        then: Box<Node>,
        otherwise: Box<Node>,
        line: u32,
    },
    /// `%expr%`: the name that the script builds.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the dynamic
    /// variable reference of v2 is a name of its own, and the pieces of it
    /// are the text that is written and the values of the expressions. The
    /// name of the variable whose value is the answer is the one that the
    /// pieces make.
    DynamicName {
        pieces: Vec<DynamicPiece>,
        line: u32,
    },
    /// `%expr% := value`: the write of a name that the script builds.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the variable has to
    /// be there, and a function that writes a name of the script says that
    /// it wants a global declaration.
    AssignDynamic {
        name: Vec<DynamicPiece>,
        op: AssignOp,
        value: Box<Node>,
        line: u32,
    },
    /// `x := 1`.
    Assign {
        name: String,
        op: AssignOp,
        value: Box<Node>,
        line: u32,
    },
    /// `x = 5`: the legacy assignment of the v1.
    ///
    /// The right side of the legacy assignment is a text and not an
    /// expression: `x = 5 + 3` leaves `x` at "5 + 3", and `x = "abc"` leaves
    /// it at `"abc"` with the quotes that were written.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02.
    LegacyAssign {
        name: String,
        text: String,
        line: u32,
    },
    /// `if x = 1`: the comparison of the legacy `if` of the v1.
    ///
    /// The right side of the comparison is the text of the rest of the line
    /// and not an expression: the `if x = 1 + 1` compares the value of `x`
    /// with the text "1 + 1", and the condition is false. The text goes
    /// through the legacy substitution, and the brace that comes after it on
    /// the line is a part of it: the legacy `if` has no One True Brace, and
    /// the brace that closes the block is unexpected.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04.
    LegacyComparison {
        left: Box<Node>,
        op: BinaryOp,
        text: String,
        line: u32,
    },
    /// `if x in a, b`: the forms of the legacy `if` of the v1 that are not a
    /// comparison.
    ///
    /// The `if x between low and high`, the `if x in list` and the
    /// `if x contains list` are the forms of the legacy `if` of the v1, and the
    /// `not` may come before the word. The texts of the form are the texts of
    /// the script and not expressions, and they go through the legacy
    /// substitution.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04.
    LegacyMatch {
        left: Box<Node>,
        form: LegacyForm,
        texts: Vec<String>,
        negated: bool,
        line: u32,
    },
    /// The `if` or the `while` of the v1 that comes inside the line of the
    /// condition of an `if`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `if (0) if (1) x := 1` is accepted, and the construct that comes
    /// inside the line has no body of its own: its condition is a part of
    /// the condition that holds it, and the line that comes after is the
    /// body of the `if` that holds it.
    LegacyControl { condition: Box<Node>, line: u32 },
    /// `list[1]`, `text[2]`: the item at a position.
    ///
    /// The positions count from one, the way AutoHotkey counts them.
    Index {
        target: Box<Node>,
        index: Box<Node>,
        line: u32,
    },
    /// A call: `PixelGetColor(10, 20)`.
    ///
    /// A command is a call written without parentheses: `Send "hello"`
    /// is the same built-in with its arguments in the same order.
    Call {
        name: String,
        arguments: Vec<Node>,
        line: u32,
    },
    /// The `new` of the v1: `new C` and `new C(5)`.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `new` of
    /// the v1 makes an instance of a class of the script, and the
    /// parenthesis that may follow the name of the class carries the
    /// arguments of the constructor. The v2 has no `new`: the class is
    /// called.
    ///
    /// The path is the name that follows the word and the names of the
    /// classes that it holds: the `new C.D` is an instance of D, and the
    /// `new o.D` is an instance of the class that the instance holds.
    New {
        path: Vec<String>,
        arguments: Vec<Node>,
        line: u32,
    },
    /// The legacy command of the v1: `Msg, text`.
    ///
    /// The parameters of a command of the v1 are texts, and the shape of
    /// the command says what each one of them is: the value of an
    /// expression, the name of a variable of the script, or the name of
    /// the variable that the command writes into.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: `FileAppend,
    /// hello, *` writes `hello`, and `FileAppend, "hello", *` writes
    /// `"hello"` with the quotes.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter
    /// that wants a number is an expression: the `Sleep, 100+50` waits for
    /// a hundred and fifty milliseconds.
    LegacyCall {
        name: String,
        parameters: Vec<LegacyParameter>,
        line: u32,
    },
    /// A command of the v1 that has a body: the `IfInString` and the
    /// `IfMsgBox`.
    ///
    /// The body is the action of the command: the command that is on the
    /// same line, the block, or the statement of the next line. The `else`
    /// of the v1 is the body of the other side.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: `IfInString, x, b`
    /// runs the line that comes after it when the text is in the variable,
    /// and it does not run it when it is not.
    LegacyIf {
        name: String,
        parameters: Vec<String>,
        body: Vec<Node>,
        otherwise: Vec<Node>,
        line: u32,
    },
    /// A value that is computed and thrown away: `(2)`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: a line that begins
    /// with a parenthesis is an expression, and the value it leaves behind is
    /// not used.
    Expression { value: Box<Node>, line: u32 },
    /// The parameter of the v1 that is written with a percent sign and a
    /// space in front of it: the expression that it holds.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// percent sign and a space in front of a parameter of the v1 force it to
    /// be an expression: the `FileAppend, % "hello", *` writes "hello", and
    /// the `Loop, Parse, % "a|b|c", "|"` separates the text of the
    /// expression.
    LegacyExpression { value: Box<Node>, line: u32 },
    /// A block that opens a line: `{` and the statements, and `}`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the brace that opens
    /// a line is a block of its own, and the statements between the braces
    /// run. The content may be on the line of the brace that opens it, and
    /// the brace that closes it is alone on its line, unless the block is
    /// empty.
    Block { body: Vec<Node>, line: u32 },
    /// `If cond { } else if cond { } else { }`.
    If {
        branches: Vec<Branch>,
        otherwise: Vec<Node>,
        line: u32,
    },
    /// `Switch value { Case 1 { } Default { } }`.
    Switch {
        subject: Box<Node>,
        cases: Vec<Case>,
        line: u32,
    },
    /// `While cond { }`.
    While {
        condition: Box<Node>,
        body: Vec<Node>,
        /// The `Else` of the loop: the statements that run when the loop
        /// took no turn at all.
        ///
        /// Measured on the binary of v2.0.19 on 2026-11-09: `while` may
        /// carry an `else`, and it runs exactly when the body ran no turn.
        otherwise: Option<Vec<Node>>,
        line: u32,
    },
    /// `Loop 3 { }`, `Loop Files "*.txt" { }`, `Loop Parse "a,b" { }`.
    ///
    /// The `until` is the condition of the v1: it is the statement that
    /// comes right after the body of the loop, and the loop ends when it
    /// is true. The v2 has no such statement, and the field is empty.
    Loop {
        mode: LoopKind,
        arguments: Vec<Node>,
        body: Vec<Node>,
        until: Option<Box<Node>>,
        /// The `Else` of the loop, run when the loop took no turn:
        /// `Loop 0` and `Loop Files` of a pattern that names no file are the
        /// two cases of the corpus.
        ///
        /// Measured on the binary of v2.0.19 on 2026-11-09: every kind of
        /// `Loop` may carry an `else`.
        otherwise: Option<Vec<Node>>,
        line: u32,
    },
    /// `#Requires AutoHotkey v2.0.19`: a directive.
    ///
    /// The name is the name of the directive and the argument is the
    /// rest of the line, which is what the directive is about.
    Directive {
        name: String,
        argument: String,
        line: u32,
    },
    /// `Break [N]`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the number is the
    /// level of the loop that the break leaves, and it is an integer
    /// literal. A break without a number is the level one.
    Break { line: u32, level: u32 },
    /// `Continue [N]`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the number is the
    /// level of the loop that the continue continues, and it is an integer
    /// literal. A continue without a number is the level one.
    Continue { line: u32, level: u32 },
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
        /// The line of the brace that opens the body.
        ///
        /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        /// v1 names this line when it warns about a local that has the name
        /// of a global one.
        body_line: u32,
    },
    /// `super`: the base of the class where the method was defined.
    ///
    /// It is not a value: it is followed by a member.
    Super { line: u32 },
    /// `global name` or `local name`: a declaration of v2.
    ///
    /// The names are read when the script is read, and the values wait for
    /// the statement that carries them.
    Declare {
        scope: DeclareKind,
        names: Vec<Declared>,
        line: u32,
    },
    /// `class Name extends Base { }`: the definition of a class.
    ClassDef {
        name: String,
        extends: Option<String>,
        methods: Vec<MethodDef>,
        properties: Vec<PropertyDef>,
        /// The properties that have a getter or a setter.
        accessors: Vec<AccessDef>,
        /// The classes that the class holds.
        nested: Vec<Node>,
        line: u32,
    },
    /// `obj.Method(args)`: a method called on a value.
    MethodCall {
        target: Box<Node>,
        name: MemberName,
        arguments: Vec<Node>,
        line: u32,
    },
    /// `obj.Prop`: a property of a value.
    Property {
        target: Box<Node>,
        name: MemberName,
        line: u32,
    },
    /// `obj.Prop := value`: a property of a value that is written.
    AssignProperty {
        target: Box<Node>,
        name: MemberName,
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
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the literal may
    /// have a hole, and the place of the hole is a place of the array that
    /// has no value of its own. The item is absent, and not a value that
    /// means nothing: the dialect has no such value.
    ArrayLiteral { items: Vec<Option<Node>>, line: u32 },
    /// `(a, b)`: the list of the values of a group.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the value of the
    /// list is the last of its items, and every item of it runs. The list
    /// of one item is the value of that item.
    Group { items: Vec<Node>, line: u32 },
    /// A parameter of a command that is not there: `One "a",, "b"`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the place of it
    /// is kept, and the parameter has no value: one that has a default
    /// takes it, and one that has none is a missing parameter.
    Empty { line: u32 },
    /// `expr*`: the argument of a call that is every item of the value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the star spreads
    /// the value of the argument into the arguments of the call, and only
    /// the last argument of a call may carry it: the middle one is refused
    /// at the load. A spread argument is a value of the call and not a
    /// variable reference, and the value it spreads is enumerated the way a
    /// `for` loop enumerates it: an array gives its items, a map its keys,
    /// and a value that cannot be enumerated is refused.
    Spread { value: Box<Node>, line: u32 },
    /// The list of v1 that may follow an expression: `x := 1, 2`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the expression
    /// ends at the comma, and the rest of the line is evaluated, and its
    /// value is thrown away.
    Legacy { items: Vec<Node>, line: u32 },
    /// `{a: 1, b: 2}`: the literal of an object.
    ///
    /// v2 writes an object between braces, and the empty braces are the
    /// object of nothing. The name of a property is a name of the script,
    /// and the names of the properties do not know their case.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    /// property may be built by the script: `{%name%: 1}` is the property
    /// whose name is the value of `name`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// literal of the v1 is a list of parameters, and a parameter that has
    /// no name is a value that is evaluated and dropped.
    ObjectLiteral { items: Vec<ObjectItem>, line: u32 },
    /// `(a, b) => a + b`: the literal of a function.
    ///
    /// The body of the arrow function of v2 is an expression, and not a
    /// block of statements, and the function closes over the scope where it
    /// was written.
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<Node>,
        line: u32,
    },
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
    For {
        variables: Vec<String>,
        source: Box<Node>,
        body: Vec<Node>,
        /// The condition of the `Until` that follows the body, when there is
        /// one: `for x in items { } until x > 2`.
        until: Option<Box<Node>>,
        /// The `Else` of the `for`, run when the source held no value to
        /// walk.
        ///
        /// Measured on the binary of v2.0.19 on 2026-11-09: `For x in []`
        /// with an `else` runs the `else`.
        otherwise: Option<Vec<Node>>,
        line: u32,
    },
    /// A label: `name:`.
    ///
    /// A label names a line of the script, and a `Goto` jumps to it. The
    /// name is of letters, of digits and of the underscore, and it may
    /// begin with a digit: `1abc:` is a label.
    Label { name: String, line: u32 },
    /// `Goto name`: the jump to a label of the current scope.
    ///
    /// The name is not an expression: it is the text of the parameter as
    /// the script wrote it, and the quotes of a text are part of it.
    Goto { name: String, line: u32 },
    /// A hotkey of the v1: the line that defines it, and its action.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the v1
    /// writes a hotkey in a line of its own, and the action of it is the
    /// label that comes after the line, or the command that the line
    /// carries. The body of the hotkey is the action, and it runs with the
    /// scope of the script.
    HotkeyDefinition {
        /// The name of the hotkey, as the script spelled it.
        name: String,
        /// The action of the hotkey: the body that comes after the line, or
        /// the command that the line carries.
        body: Vec<Node>,
        /// Whether the action is the command that the line carries.
        ///
        /// The body of a hotkey that is written in one line is the command
        /// of the line, and the code that comes after it does not belong to
        /// it. The body of the other one is the code that comes after it.
        on_the_line: bool,
        /// The line of the definition.
        line: u32,
    },
}

/// One parameter of a function.
///
/// A parameter is a name. It may carry a default -- `Name(a, b := 1)` --
/// it may be the one that takes the rest of the arguments --
/// `Name(a, rest*)` -- and it may carry the ByRef marker of v2 --
/// `Name(a, &b)` -- which says that the caller must pass a reference and
/// that the parameter is an alias of the variable of the caller.
#[derive(Debug, Clone, PartialEq)]
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
    /// Whether the parameter may be left out of the call.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `f(a, b?)` marks
    /// `b` optional, and a call that leaves it out leaves the variable of
    /// it unset -- `IsSet(b)` answers zero inside the function.
    pub optional: bool,
}

impl Parameter {
    /// Whether the call must give the parameter a value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: the parameter that
    /// carries a default, the one that takes the rest of the arguments and
    /// the one marked with the question mark may all be left out. Every
    /// other one is refused when the call does not give it a value.
    pub fn is_required(&self) -> bool {
        self.default.is_none() && !self.variadic && !self.optional
    }
}

impl Node {
    /// The line the node came from.
    pub fn line(&self) -> u32 {
        match self {
            Node::Literal { line, .. }
            | Node::Var { line, .. }
            | Node::Unary { line, .. }
            | Node::Binary { line, .. }
            | Node::Unset { line }
            | Node::Ternary { line, .. }
            | Node::DynamicName { line, .. }
            | Node::AssignDynamic { line, .. }
            | Node::Assign { line, .. }
            | Node::LegacyAssign { line, .. }
            | Node::LegacyComparison { line, .. }
            | Node::LegacyMatch { line, .. }
            | Node::LegacyControl { line, .. }
            | Node::AssignIndex { line, .. }
            | Node::Index { line, .. }
            | Node::Call { line, .. }
            | Node::New { line, .. }
            | Node::LegacyCall { line, .. }
            | Node::LegacyIf { line, .. }
            | Node::Expression { line, .. }
            | Node::LegacyExpression { line, .. }
            | Node::Block { line, .. }
            | Node::If { line, .. }
            | Node::Switch { line, .. }
            | Node::While { line, .. }
            | Node::Loop { line, .. }
            | Node::Directive { line, .. }
            | Node::Break { line, .. }
            | Node::Continue { line, .. }
            | Node::Return { line, .. }
            | Node::ExitApp { line, .. }
            | Node::FunctionDef { line, .. }
            | Node::ClassDef { line, .. }
            | Node::MethodCall { line, .. }
            | Node::Property { line, .. }
            | Node::AssignProperty { line, .. }
            | Node::ArrayLiteral { line, .. }
            | Node::Group { line, .. }
            | Node::Empty { line }
            | Node::Spread { line, .. }
            | Node::Legacy { line, .. }
            | Node::ObjectLiteral { line, .. }
            | Node::Lambda { line, .. }
            | Node::Try { line, .. }
            | Node::Throw { line, .. }
            | Node::Declare { line, .. }
            | Node::Super { line, .. }
            | Node::For { line, .. }
            | Node::Label { line, .. }
            | Node::Goto { line, .. }
            | Node::HotkeyDefinition { line, .. } => *line,
        }
    }
}

/// The form of the legacy `if` of the v1 that is not a comparison.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
/// `between` carries the two bounds, and the `in` and the `contains` carry
/// the items of the list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyForm {
    /// `between`: the texts are the low bound and the high bound.
    Between,
    /// `in`: the texts are the items of the list.
    In,
    /// `contains`: the texts are the items of the list.
    Contains,
}

/// The parameter of a command of the v1.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter that
/// wants a number is an expression -- the `Sleep, 100+50` waits for a
/// hundred and fifty milliseconds -- and the others are texts.
#[derive(Debug, Clone, PartialEq)]
pub enum LegacyParameter {
    /// The text of the parameter, with the substitution of the v1.
    Text(String),
    /// The expression of the parameter.
    Expression(Node),
}
