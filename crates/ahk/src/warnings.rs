//! The warnings of a script.
//!
//! Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the `#Warn`
//! directive takes the type of the warning and the mode of it. The types are
//! the "All", the "VarUnset", the "LocalSameAsGlobal" and the "Unreachable",
//! and the modes are the "MsgBox", the "StdOut" and the "Off". The type that
//! is not one of them is refused at the load with "Parameter #1 invalid." and
//! the text of the directive, and the mode that is not one of them with
//! "Parameter #2 invalid." and the text that was written.
//!
//! The tree has no dialog, and the warnings go to the report. The mode of the
//! "StdOut" writes them to the standard output as well, the way the v2 does:
//! the line of the script and the text of the warning.

use crate::ast::Node;
use serde::Serialize;

/// The type of a warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningType {
    /// Every warning of the dialect.
    All,
    /// A variable that is read and never written.
    VarUnset,
    /// A local variable that has the name of a global one.
    LocalSameAsGlobal,
    /// A line that comes after a statement that leaves the level.
    Unreachable,
}

/// The way a warning is given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningMode {
    /// The warning is a dialog of the machine. The tree has no dialog, and
    /// the warning goes to the report.
    MsgBox,
    /// The warning goes to the standard output.
    StdOut,
    /// There is no warning.
    Off,
}

/// The mode of the warnings of each type.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warnings
/// are on when the script does not say, and they use the mode of the "MsgBox",
/// and the warnings of the "LocalSameAsGlobal" are off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheModes {
    /// The mode of the warnings of a variable that is read and never written.
    pub var_unset: WarningMode,
    /// The mode of the warnings of a local that has the name of a global.
    pub local_same_as_global: WarningMode,
    /// The mode of the warnings of a line that never runs.
    pub unreachable: WarningMode,
}

impl Default for TheModes {
    fn default() -> Self {
        TheModes {
            var_unset: WarningMode::MsgBox,
            local_same_as_global: WarningMode::Off,
            unreachable: WarningMode::MsgBox,
        }
    }
}

/// A warning of the script.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Warning {
    /// The line of the script that the warning names.
    pub line: u32,
    /// The text of the warning.
    pub message: String,
}

/// The type of the warnings that the name names.
pub fn the_type_of_the_name(name: &str) -> Option<WarningType> {
    match name.to_lowercase().as_str() {
        "all" => Some(WarningType::All),
        "varunset" => Some(WarningType::VarUnset),
        "localsameasglobal" => Some(WarningType::LocalSameAsGlobal),
        "unreachable" => Some(WarningType::Unreachable),
        _ => None,
    }
}

/// The mode of the warnings that the name names.
pub fn the_mode_of_the_name(name: &str) -> Option<WarningMode> {
    match name.to_lowercase().as_str() {
        "msgbox" => Some(WarningMode::MsgBox),
        "stdout" => Some(WarningMode::StdOut),
        "off" => Some(WarningMode::Off),
        _ => None,
    }
}

/// The type and the mode that the argument of the `#Warn` names.
///
/// The type is the text that comes before the comma, and the mode is the text
/// that comes after it. The type that is not one of them is named by the
/// caller, and the mode that is not one of them is named by the caller too.
pub fn the_kind_and_the_mode(argument: &str) -> (Option<WarningType>, Option<WarningMode>) {
    let (kind, mode) = the_parts_of_the_argument(argument);
    let kind = if kind.is_empty() { None } else { the_type_of_the_name(kind) };
    let mode = if mode.is_empty() { None } else { the_mode_of_the_name(mode) };
    (kind, mode)
}

/// The texts of the type and of the mode of the argument of the `#Warn`.
///
/// The type is the text that comes before the comma, and the mode is the text
/// that comes after it. A text that is empty is not a name, and the caller
/// asks the tree whether the name is one of them.
pub fn the_parts_of_the_argument(argument: &str) -> (&str, &str) {
    match argument.split_once(',') {
        Some((kind, mode)) => (kind.trim(), mode.trim()),
        None => (argument.trim(), ""),
    }
}

/// The modes of the warnings of the script.
///
/// The directives are read before the script runs, the way the v2 reads them,
/// and the last `#Warn` is the one that counts.
pub fn the_modes_of_the_script(statements: &[Node]) -> TheModes {
    let mut modes = TheModes::default();
    the_modes_of_the_level(statements, &mut modes);
    modes
}

fn the_modes_of_the_level(statements: &[Node], modes: &mut TheModes) {
    for statement in statements {
        if let Node::Directive { name, argument, .. } = statement {
            if name.eq_ignore_ascii_case("warn") {
                let (kind, mode) = the_kind_and_the_mode(argument);
                if let Some(kind) = kind {
                    if let Some(mode) = mode {
                        match kind {
                            WarningType::All => {
                                modes.var_unset = mode;
                                modes.local_same_as_global = mode;
                                modes.unreachable = mode;
                            }
                            WarningType::VarUnset => modes.var_unset = mode,
                            WarningType::LocalSameAsGlobal => modes.local_same_as_global = mode,
                            WarningType::Unreachable => modes.unreachable = mode,
                        }
                    }
                }
            }
        }
        for body in the_bodies_of(statement) {
            the_modes_of_the_level(body, modes);
        }
    }
}

/// The warnings of the statements of the script.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning of
/// the unreachable line is given for the line that comes right after a
/// `Return`, a `Break`, a `Continue`, a `Throw` or a `Goto` of the same level,
/// and the label that is the target of the jump is not warned about.
pub fn the_warnings_of_the_script(statements: &[Node]) -> Vec<Warning> {
    let mut warnings = Vec::new();
    the_warnings_of_the_level(statements, &mut warnings);
    warnings
}

fn the_warnings_of_the_level(statements: &[Node], warnings: &mut Vec<Warning>) {
    let mut after: Option<&'static str> = None;
    for statement in statements {
        if let Some(word) = after {
            // The warning is of the line that comes right after the statement
            // that leaves the level, and not of the lines that come after that
            // one.
            after = None;
            if !matches!(statement, Node::Label { .. }) {
                warnings.push(Warning {
                    line: statement.line(),
                    message: format!("This line will never execute, due to {word} preceding it."),
                });
            }
        }
        if !matches!(statement, Node::Label { .. }) {
            after = the_word_that_leaves_the_level(statement);
        }
        for body in the_bodies_of(statement) {
            the_warnings_of_the_level(body, warnings);
        }
    }
}

/// The word of the statement that leaves the level, when it leaves it.
fn the_word_that_leaves_the_level(statement: &Node) -> Option<&'static str> {
    match statement {
        Node::Break { .. } => Some("Break"),
        Node::Continue { .. } => Some("Continue"),
        Node::Return { .. } => Some("Return"),
        Node::Throw { .. } => Some("Throw"),
        Node::Goto { .. } => Some("Goto"),
        _ => None,
    }
}

/// The bodies of a statement: the levels of their own.
fn the_bodies_of(statement: &Node) -> Vec<&[Node]> {
    match statement {
        Node::Block { body, .. } => vec![body],
        Node::If { branches, otherwise, .. } => {
            let mut bodies: Vec<&[Node]> =
                branches.iter().map(|branch| branch.body.as_slice()).collect();
            bodies.push(otherwise);
            bodies
        }
        Node::Switch { cases, .. } => {
            cases.iter().map(|case| case.body.as_slice()).collect()
        }
        Node::While { body, .. } => vec![body],
        Node::Loop { body, .. } => vec![body],
        Node::For { body, .. } => vec![body],
        Node::FunctionDef { body, .. } => vec![body],
        Node::Try { body, catches, otherwise, finally, .. } => {
            let mut bodies: Vec<&[Node]> = vec![body, otherwise, finally];
            bodies.extend(catches.iter().map(|catch| catch.body.as_slice()));
            bodies
        }
        Node::ClassDef { methods, nested, .. } => {
            let mut bodies: Vec<&[Node]> =
                methods.iter().map(|method| method.body.as_slice()).collect();
            for class in nested {
                bodies.extend(the_bodies_of(class));
            }
            bodies
        }
        Node::LegacyIf { body, otherwise, .. } => vec![body, otherwise],
        Node::HotkeyDefinition { body, .. } => vec![body],
        _ => Vec::new(),
    }
}
