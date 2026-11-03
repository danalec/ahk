//! The warnings of a script.
//!
//! Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
//! binary of AutoHotkey v1.1.37.02 on 2026-11-05: the `#Warn` directive takes
//! the type of the warning and the mode of it, and the two dialects have
//! their own types. The type that is not one of them is refused at the load
//! with "Parameter #1 invalid." and the source of the directive, and the mode
//! that is not one of them with "Parameter #2 invalid." and the text that was
//! written.
//!
//! The tree has no dialog, and the warnings go to the report. The mode of the
//! "StdOut" writes them to the standard output as well, the way the v2 does:
//! the line of the script and the text of the warning.

use crate::ast::Node;
use crate::dialect::Dialect;
use serde::Serialize;

/// The type of a warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningType {
    /// Every warning of the dialect.
    All,
    /// A variable that is read and never written.
    VarUnset,
    /// A variable of the script that is read and never written.
    UseUnsetGlobal,
    /// A variable of a function that is read and never written.
    UseUnsetLocal,
    /// An environment variable that is accessed.
    UseEnv,
    /// A local variable that has the name of a global one.
    LocalSameAsGlobal,
    /// A class that is overwritten by a variable.
    ClassOverwrite,
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
    /// The warning goes to the debug output of the machine. The tree has
    /// none, and the warning goes to the report.
    OutputDebug,
    /// There is no warning.
    Off,
}

/// The mode of the warnings of each type.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warnings
/// of the v2 are on when the script does not say, and they use the mode of the
/// "MsgBox", and the warnings of the "LocalSameAsGlobal" are off.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
/// warnings of the v1 are off when the script does not say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheModes {
    /// The mode of the warning of the variable that is read and never
    /// written.
    pub var_unset: WarningMode,
    /// The mode of the warning of the variable of the script that is read and
    /// never written.
    pub use_unset_global: WarningMode,
    /// The mode of the warning of the variable of a function that is read and
    /// never written.
    pub use_unset_local: WarningMode,
    /// The mode of the warning of the environment variable that is accessed.
    pub use_env: WarningMode,
    /// The mode of the warning of the local that has the name of a global.
    pub local_same_as_global: WarningMode,
    /// The mode of the warning of the class that is overwritten.
    pub class_overwrite: WarningMode,
    /// The mode of the warning of the line that never runs.
    pub unreachable: WarningMode,
}

impl TheModes {
    /// The modes of the v2 when the script does not say.
    pub fn the_defaults_of_the_v2() -> TheModes {
        TheModes {
            var_unset: WarningMode::MsgBox,
            use_unset_global: WarningMode::Off,
            use_unset_local: WarningMode::Off,
            use_env: WarningMode::Off,
            local_same_as_global: WarningMode::Off,
            class_overwrite: WarningMode::Off,
            unreachable: WarningMode::MsgBox,
        }
    }

    /// The modes of the v1 when the script does not say.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// warnings of the v1 are off when the script does not say.
    pub fn the_defaults_of_the_v1() -> TheModes {
        TheModes {
            var_unset: WarningMode::Off,
            use_unset_global: WarningMode::Off,
            use_unset_local: WarningMode::Off,
            use_env: WarningMode::Off,
            local_same_as_global: WarningMode::Off,
            class_overwrite: WarningMode::Off,
            unreachable: WarningMode::Off,
        }
    }

    /// The mode of the type of the warning.
    pub fn the_mode_of(&self, kind: WarningType) -> WarningMode {
        match kind {
            WarningType::All => WarningMode::Off,
            WarningType::VarUnset => self.var_unset,
            WarningType::UseUnsetGlobal => self.use_unset_global,
            WarningType::UseUnsetLocal => self.use_unset_local,
            WarningType::UseEnv => self.use_env,
            WarningType::LocalSameAsGlobal => self.local_same_as_global,
            WarningType::ClassOverwrite => self.class_overwrite,
            WarningType::Unreachable => self.unreachable,
        }
    }

    /// The mode of the type of the warning is written.
    pub fn write(&mut self, kind: WarningType, mode: WarningMode) {
        match kind {
            WarningType::All => {
                self.var_unset = mode;
                self.use_unset_global = mode;
                self.use_unset_local = mode;
                self.use_env = mode;
                self.local_same_as_global = mode;
                self.class_overwrite = mode;
                self.unreachable = mode;
            }
            WarningType::VarUnset => self.var_unset = mode,
            WarningType::UseUnsetGlobal => self.use_unset_global = mode,
            WarningType::UseUnsetLocal => self.use_unset_local = mode,
            WarningType::UseEnv => self.use_env = mode,
            WarningType::LocalSameAsGlobal => self.local_same_as_global = mode,
            WarningType::ClassOverwrite => self.class_overwrite = mode,
            WarningType::Unreachable => self.unreachable = mode,
        }
    }
}

/// The types of the warning that the tree carries.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
/// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the tree warns about the
/// variable that is read and never written, about the environment variable
/// that is accessed, and about the line that never runs. The two warnings
/// that are read out of the script -- the local that has the name of a global
/// and the class that is overwritten -- are not carried yet, and the report
/// says so.
pub fn the_types_that_are_carried() -> &'static [WarningType] {
    &[
        WarningType::VarUnset,
        WarningType::UseUnsetGlobal,
        WarningType::UseUnsetLocal,
        WarningType::UseEnv,
        WarningType::LocalSameAsGlobal,
        WarningType::ClassOverwrite,
        WarningType::Unreachable,
    ]
}

/// The name of the type of the warning, the way the v2 writes it.
pub fn the_name_of_the_type(kind: WarningType) -> &'static str {
    match kind {
        WarningType::All => "All",
        WarningType::VarUnset => "VarUnset",
        WarningType::UseUnsetGlobal => "UseUnsetGlobal",
        WarningType::UseUnsetLocal => "UseUnsetLocal",
        WarningType::UseEnv => "UseEnv",
        WarningType::LocalSameAsGlobal => "LocalSameAsGlobal",
        WarningType::ClassOverwrite => "ClassOverwrite",
        WarningType::Unreachable => "Unreachable",
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

/// The type and the mode that the argument of the `#Warn` names.
///
/// The type is the text that comes before the comma, and the mode is the text
/// that comes after it. The type that is not one of them is named by the
/// caller, and the mode that is not one of them is named by the caller too.
pub fn the_kind_and_the_mode(
    argument: &str,
    dialect: &dyn Dialect,
) -> (Option<WarningType>, Option<WarningMode>) {
    let (kind, mode) = the_parts_of_the_argument(argument);
    let kind = if kind.is_empty() { None } else { dialect.the_type_of_the_warning(kind) };
    let mode = if mode.is_empty() { None } else { dialect.the_mode_of_the_warning(mode) };
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
pub fn the_modes_of_the_script(statements: &[Node], dialect: &dyn Dialect) -> TheModes {
    let mut modes = dialect.the_default_warnings();
    the_modes_of_the_level(statements, &mut modes, dialect);
    modes
}

fn the_modes_of_the_level(statements: &[Node], modes: &mut TheModes, dialect: &dyn Dialect) {
    for statement in statements {
        if let Node::Directive { name, argument, .. } = statement {
            if name.eq_ignore_ascii_case("warn") {
                let (kind, mode) = the_kind_and_the_mode(argument, dialect);
                if let Some(kind) = kind {
                    if let Some(mode) = mode {
                        modes.write(kind, mode);
                    }
                }
            }
        }
        for body in the_bodies_of(statement) {
            the_modes_of_the_level(body, modes, dialect);
        }
    }
}

/// The types of the warning that the script asks for.
///
/// The directives are read before the script runs, and the type of the "All"
/// is the type of every warning of the dialect.
pub fn the_types_of_the_script(statements: &[Node], dialect: &dyn Dialect) -> Vec<WarningType> {
    let mut types = Vec::new();
    the_types_of_the_level(statements, &mut types, dialect);
    types
}

fn the_types_of_the_level(statements: &[Node], types: &mut Vec<WarningType>, dialect: &dyn Dialect) {
    for statement in statements {
        if let Node::Directive { name, argument, .. } = statement {
            if name.eq_ignore_ascii_case("warn") {
                let (kind, _) = the_kind_and_the_mode(argument, dialect);
                if let Some(kind) = kind {
                    // The type of the "All" is the type of every warning of
                    // the dialect.
                    let kinds = if kind == WarningType::All {
                        dialect.the_types_of_the_warning().to_vec()
                    } else {
                        vec![kind]
                    };
                    for kind in kinds {
                        if !types.contains(&kind) {
                            types.push(kind);
                        }
                    }
                }
            }
        }
        for body in the_bodies_of(statement) {
            the_types_of_the_level(body, types, dialect);
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

/// The warning of the local variable that has the name of a global one.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05 and on the
/// binary of AutoHotkey v2.0.19 on 2026-11-05: the warning is given once,
/// when the script is read, and the text of it names the variable and the
/// function of it -- "x  (in function f)". The v1 names the line that opens
/// the body of the function, and the v2 names the line of the write of the
/// local.
///
/// A local that is declared global is not a local, and a local that has the
/// name of a variable of the script is warned about. The parameters of the
/// function are declared, and they are not warned about.
pub fn the_warnings_of_the_locals_that_are_globals(
    statements: &[Node],
    dialect: &dyn Dialect,
) -> Vec<Warning> {
    let globals = the_globals_of_the_script(statements);
    let mut warnings = Vec::new();
    the_warnings_of_the_locals_of_the_level(statements, &globals, dialect, &mut warnings);
    warnings
}

fn the_warnings_of_the_locals_of_the_level(
    statements: &[Node],
    globals: &std::collections::BTreeSet<String>,
    dialect: &dyn Dialect,
    warnings: &mut Vec<Warning>,
) {
    for statement in statements {
        match statement {
            Node::FunctionDef { name, parameters, body, declarations, body_line, .. } => {
                let scope = TheScope {
                    name,
                    parameters,
                    body,
                    declarations,
                    body_line: *body_line,
                };
                the_warnings_of_the_locals_of_a_scope(scope, globals, dialect, warnings);
            }
            Node::ClassDef { name, methods, accessors, nested, .. } => {
                the_warnings_of_the_locals_of_a_class(
                    name,
                    methods,
                    accessors,
                    nested,
                    globals,
                    dialect,
                    warnings,
                );
            }
            _ => {}
        }
        for body in the_bodies_of_the_level(statement) {
            the_warnings_of_the_locals_of_the_level(body, globals, dialect, warnings);
        }
    }
}

/// The scope whose locals are warned about.
///
/// It is a function of the script, a method of a class or the body of a
/// getter.
struct TheScope<'a> {
    /// The name of the scope, the way the warning names it.
    name: &'a str,
    /// The parameters of the scope.
    parameters: &'a [crate::ast::Parameter],
    /// The statements of the scope.
    body: &'a [Node],
    /// The declarations of the scope.
    declarations: &'a crate::ast::Declarations,
    /// The line of the brace that opens the body.
    body_line: u32,
}

/// The warnings of the locals of a function or of a method.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
/// names the line that opens the body of the function, and the v2 names the
/// line of the write of the local.
fn the_warnings_of_the_locals_of_a_scope(
    scope: TheScope<'_>,
    globals: &std::collections::BTreeSet<String>,
    dialect: &dyn Dialect,
    warnings: &mut Vec<Warning>,
) {
    let TheScope { name, parameters, body, declarations, body_line } = scope;
    let mut writes = Vec::new();
    the_writes_of_a_level(body, &mut writes, false);
    // The locals of the scope are the names that it writes, and the names
    // that it only reads as well when the scope of a function is its own.
    let mut locals = writes;
    if dialect.the_scope_of_a_function_is_its_own() {
        let mut reads = Vec::new();
        the_reads_of_a_level(body, &mut reads);
        locals.extend(reads);
        locals.sort_by_key(|(_, line)| *line);
    }
    let mut seen = std::collections::BTreeSet::new();
    for (local, the_line) in locals {
        if !seen.insert(local.to_lowercase()) {
            continue;
        }
        if declarations.globals.iter().any(|name| name.to_lowercase() == local.to_lowercase()) {
            continue;
        }
        if parameters.iter().any(|parameter| parameter.name.to_lowercase() == local.to_lowercase()) {
            continue;
        }
        if !globals.contains(&local.to_lowercase()) {
            continue;
        }
        if crate::variables::is_built_in(&local) {
            continue;
        }
        // The v1 names the line that opens the body of the function, and the
        // v2 names the line of the write of the local.
        let line = if dialect.the_scope_of_a_function_is_its_own() { body_line } else { the_line };
        warnings.push(Warning {
            line,
            message: format!(
                "This local variable has the same name as a global variable.\n     Specifically: {local}  (in function {name})"
            ),
        });
    }
}

/// The warnings of the locals of the members of a class.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
/// names the method "A.m" -- the class and the method -- and the class that is
/// nested is "A.B.m". Measured on the binary of AutoHotkey v2.0.19 on
/// 2026-11-05: the v2 names the method of the prototype "A.Prototype.m", and
/// the static method is "A.m".
fn the_warnings_of_the_locals_of_a_class(
    path: &str,
    methods: &[crate::ast::MethodDef],
    accessors: &[crate::ast::AccessDef],
    nested: &[Node],
    globals: &std::collections::BTreeSet<String>,
    dialect: &dyn Dialect,
    warnings: &mut Vec<Warning>,
) {
    for method in methods {
        let name = dialect.the_name_of_a_member_of_a_class(path, &method.name, method.is_static);
        let scope = TheScope {
            name: &name,
            parameters: &method.parameters,
            body: &method.body,
            declarations: &method.declarations,
            body_line: method.body_line,
        };
        the_warnings_of_the_locals_of_a_scope(scope, globals, dialect, warnings);
    }
    for accessor in accessors {
        for (kind, body) in [("get", &accessor.getter), ("set", &accessor.setter)] {
            let Some(body) = body else { continue };
            let name =
                dialect.the_name_of_a_member_of_a_class(path, &format!("{}.{kind}", accessor.name), accessor.is_static);
            let scope = TheScope {
                name: &name,
                parameters: &accessor.parameters,
                body: &body.body,
                declarations: &body.declarations,
                body_line: body.line,
            };
            the_warnings_of_the_locals_of_a_scope(scope, globals, dialect, warnings);
        }
    }
    for class in nested {
        if let Node::ClassDef { name, methods, accessors, nested, .. } = class {
            let path = format!("{path}.{name}");
            the_warnings_of_the_locals_of_a_class(&path, methods, accessors, nested, globals, dialect, warnings);
        }
    }
}

/// The names that the level reads, and the lines of the reads.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the scope of
/// a function of the v1 is its own, and the name that it only reads is a local
/// of it: the `f() { y := x }` has a local "x" when the script has a global
/// "x". The name of a variable of the v1 is between two percent signs, and the
/// escape of the v1 -- the backtick -- carries a percent sign that does not
/// open a name.
fn the_reads_of_a_level(statements: &[Node], reads: &mut Vec<(String, u32)>) {
    for statement in statements {
        the_reads_of_a_node(statement, reads);
        for body in the_bodies_of_the_level(statement) {
            the_reads_of_a_level(body, reads);
        }
    }
}

/// The names that a node reads, and the lines of the reads.
///
/// The match has an arm for every node of the tree of the script, so that a
/// node that is added later is not forgotten here.
fn the_reads_of_a_node(node: &Node, reads: &mut Vec<(String, u32)>) {
    match node {
        Node::Literal { .. } => {}
        Node::Var { name, line } => reads.push((name.clone(), *line)),
        Node::Unary { operand, .. } => the_reads_of_a_node(operand, reads),
        Node::Binary { left, right, .. } => {
            the_reads_of_a_node(left, reads);
            the_reads_of_a_node(right, reads);
        }
        Node::Unset { .. } => {}
        Node::Ternary { condition, then, otherwise, .. } => {
            the_reads_of_a_node(condition, reads);
            the_reads_of_a_node(then, reads);
            the_reads_of_a_node(otherwise, reads);
        }
        Node::DynamicName { pieces, .. } => {
            for piece in pieces {
                the_reads_of_a_piece(piece, reads);
            }
        }
        Node::AssignDynamic { name, value, .. } => {
            for piece in name {
                the_reads_of_a_piece(piece, reads);
            }
            the_reads_of_a_node(value, reads);
        }
        Node::Assign { value, .. } => the_reads_of_a_node(value, reads),
        Node::LegacyAssign { text, line, .. } => the_names_of_a_text(text, *line, reads),
        Node::LegacyComparison { left, text, line, .. } => {
            the_reads_of_a_node(left, reads);
            the_names_of_a_text(text, *line, reads);
        }
        Node::LegacyMatch { left, texts, line, .. } => {
            the_reads_of_a_node(left, reads);
            for text in texts {
                the_names_of_a_text(text, *line, reads);
            }
        }
        Node::LegacyControl { condition, .. } => the_reads_of_a_node(condition, reads),
        Node::Index { target, index, .. } => {
            the_reads_of_a_node(target, reads);
            the_reads_of_a_node(index, reads);
        }
        Node::Call { arguments, .. } => {
            for argument in arguments {
                the_reads_of_a_node(argument, reads);
            }
        }
        Node::New { arguments, .. } => {
            for argument in arguments {
                the_reads_of_a_node(argument, reads);
            }
        }
        Node::LegacyCall { parameters, line, .. } => {
            for parameter in parameters {
                match parameter {
                    crate::ast::LegacyParameter::Text(text) => {
                        the_names_of_a_text(text, *line, reads)
                    }
                    crate::ast::LegacyParameter::Expression(value) => {
                        the_reads_of_a_node(value, reads)
                    }
                }
            }
        }
        Node::LegacyIf { parameters, line, .. } => {
            for text in parameters {
                the_names_of_a_text(text, *line, reads);
            }
        }
        Node::Expression { value, .. } => the_reads_of_a_node(value, reads),
        Node::LegacyExpression { value, .. } => the_reads_of_a_node(value, reads),
        Node::Block { .. } => {}
        Node::If { branches, .. } => {
            for branch in branches {
                the_reads_of_a_node(&branch.condition, reads);
                for item in &branch.tail {
                    the_reads_of_a_node(item, reads);
                }
            }
        }
        Node::Switch { subject, cases, .. } => {
            the_reads_of_a_node(subject, reads);
            for case in cases {
                for value in &case.values {
                    the_reads_of_a_node(value, reads);
                }
            }
        }
        Node::While { condition, .. } => the_reads_of_a_node(condition, reads),
        Node::Loop { arguments, until, .. } => {
            for argument in arguments {
                the_reads_of_a_node(argument, reads);
            }
            if let Some(until) = until {
                the_reads_of_a_node(until, reads);
            }
        }
        Node::Directive { .. } => {}
        Node::Break { .. } => {}
        Node::Continue { .. } => {}
        Node::Return { value, .. } => {
            if let Some(value) = value {
                the_reads_of_a_node(value, reads);
            }
        }
        Node::ExitApp { code, .. } => {
            if let Some(code) = code {
                the_reads_of_a_node(code, reads);
            }
        }
        Node::FunctionDef { .. } => {}
        Node::ClassDef { .. } => {}
        Node::MethodCall { target, arguments, .. } => {
            the_reads_of_a_node(target, reads);
            for argument in arguments {
                the_reads_of_a_node(argument, reads);
            }
        }
        Node::Property { target, name, .. } => {
            the_reads_of_a_node(target, reads);
            the_reads_of_a_member_name(name, reads);
        }
        Node::AssignProperty { target, name, value, .. } => {
            the_reads_of_a_node(target, reads);
            the_reads_of_a_member_name(name, reads);
            the_reads_of_a_node(value, reads);
        }
        Node::AssignIndex { target, index, value, .. } => {
            the_reads_of_a_node(target, reads);
            the_reads_of_a_node(index, reads);
            the_reads_of_a_node(value, reads);
        }
        Node::ArrayLiteral { items, .. } => {
            for item in items.iter().flatten() {
                the_reads_of_a_node(item, reads);
            }
        }
        Node::Group { items, .. } => {
            for item in items {
                the_reads_of_a_node(item, reads);
            }
        }
        Node::Empty { .. } => {}
        Node::Legacy { items, .. } => {
            for item in items {
                the_reads_of_a_node(item, reads);
            }
        }
        Node::ObjectLiteral { items, .. } => {
            for item in items {
                match item {
                    crate::ast::ObjectItem::Property(name, value) => {
                        the_reads_of_a_member_name(name, reads);
                        the_reads_of_a_node(value, reads);
                    }
                    crate::ast::ObjectItem::LegacyKey(key, value) => {
                        the_reads_of_a_node(key, reads);
                        the_reads_of_a_node(value, reads);
                    }
                    crate::ast::ObjectItem::Value(value) => the_reads_of_a_node(value, reads),
                }
            }
        }
        Node::Lambda { body, .. } => the_reads_of_a_node(body, reads),
        Node::Try { .. } => {}
        Node::Throw { value, .. } => {
            if let Some(value) = value {
                the_reads_of_a_node(value, reads);
            }
        }
        Node::Declare { .. } => {}
        Node::Super { .. } => {}
        Node::For { source, .. } => the_reads_of_a_node(source, reads),
        Node::Label { .. } => {}
        Node::Goto { .. } => {}
        Node::HotkeyDefinition { .. } => {}
    }
}

/// The names that a piece of a name that the script builds reads.
fn the_reads_of_a_piece(piece: &crate::ast::DynamicPiece, reads: &mut Vec<(String, u32)>) {
    if let crate::ast::DynamicPiece::Value(value) = piece {
        the_reads_of_a_node(value, reads);
    }
}

/// The names that a member of a value reads.
fn the_reads_of_a_member_name(name: &crate::ast::MemberName, reads: &mut Vec<(String, u32)>) {
    if let crate::ast::MemberName::Dynamic(pieces) = name {
        for piece in pieces {
            the_reads_of_a_piece(piece, reads);
        }
    }
}

/// The names that a text of the v1 reads.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the escape
/// of the v1 makes the character that follows it a literal, and the percent
/// sign that it carries does not close the name of a variable.
fn the_names_of_a_text(text: &str, line: u32, reads: &mut Vec<(String, u32)>) {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index] == '`' {
            index += 2;
            continue;
        }
        if chars[index] == '%' {
            if let Some(close) = the_closing_percent(&chars, index + 1) {
                let name: String = chars[index + 1..close].iter().collect();
                reads.push((name.trim().to_string(), line));
                index = close + 1;
                continue;
            }
        }
        index += 1;
    }
}

/// The place of the percent sign that closes the name of a variable.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the escape
/// of the v1 makes the character that follows it a literal, and the percent
/// sign that it carries does not close the name of a variable.
fn the_closing_percent(chars: &[char], from: usize) -> Option<usize> {
    let mut index = from;
    while index < chars.len() {
        match chars[index] {
            '`' => index += 2,
            '%' => return Some(index),
            _ => index += 1,
        }
    }
    None
}

/// The warning of the class that a variable overwrites.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the text of
/// it is "Class may be overwritten." and the name of the class, and it is
/// given at the line of the write that overwrites it.
pub fn the_warnings_of_the_classes_that_are_overwritten(statements: &[Node]) -> Vec<Warning> {
    let mut classes = std::collections::BTreeSet::new();
    the_classes_of_the_script(statements, &mut classes);
    let mut warnings = Vec::new();
    let mut writes = Vec::new();
    the_writes_of_a_level(statements, &mut writes, true);
    for (name, line) in writes {
        if !classes.contains(&name.to_lowercase()) {
            continue;
        }
        warnings.push(Warning {
            line,
            message: format!("Class may be overwritten.\n     Specifically: {name}"),
        });
    }
    warnings
}

/// The names of the variables of the script.
///
/// The names that the script writes at its level are the variables of the
/// script, and the classes of the script are variables too.
fn the_globals_of_the_script(statements: &[Node]) -> std::collections::BTreeSet<String> {
    let mut globals = std::collections::BTreeSet::new();
    let mut writes = Vec::new();
    the_writes_of_a_level(statements, &mut writes, false);
    for (name, _line) in writes {
        globals.insert(name.to_lowercase());
    }
    let mut classes = std::collections::BTreeSet::new();
    the_classes_of_the_script(statements, &mut classes);
    for name in classes {
        globals.insert(name);
    }
    globals
}

/// The names of the classes of the script.
fn the_classes_of_the_script(
    statements: &[Node],
    classes: &mut std::collections::BTreeSet<String>,
) {
    for statement in statements {
        if let Node::ClassDef { name, .. } = statement {
            classes.insert(name.to_lowercase());
        }
        for body in the_bodies_of_the_level(statement) {
            the_classes_of_the_script(body, classes);
        }
    }
}

/// The names that the level writes, and the lines of the writes.
///
/// The names that a function writes are the locals of it, and the walk does
/// not go into the functions when the names that are wanted are the variables
/// of the script.
fn the_writes_of_a_level(
    statements: &[Node],
    writes: &mut Vec<(String, u32)>,
    deep: bool,
) {
    for statement in statements {
        match statement {
            Node::Assign { name, line, .. } => writes.push((name.clone(), *line)),
            Node::LegacyAssign { name, line, .. } => writes.push((name.clone(), *line)),
            Node::For { variables, line, .. } => {
                for name in variables {
                    writes.push((name.clone(), *line));
                }
            }
            Node::LegacyCall { name, parameters, line } if name.eq_ignore_ascii_case("VarSet") => {
                // The `VarSet` of the v1 writes the variable of its first
                // parameter, and the parameter is a text.
                if let Some(crate::ast::LegacyParameter::Text(text)) = parameters.first() {
                    if !text.contains('%') {
                        writes.push((text.clone(), *line));
                    }
                }
            }
            _ => {}
        }
        let bodies = if deep { the_bodies_of(statement) } else { the_bodies_of_the_level(statement) };
        for body in bodies {
            the_writes_of_a_level(body, writes, deep);
        }
    }
}

/// The bodies of a statement, without the bodies of the functions.
///
/// The functions of the script have a scope of their own, and the variables
/// that they write are the locals of them.
fn the_bodies_of_the_level(statement: &Node) -> Vec<&[Node]> {
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
        Node::Try { body, catches, otherwise, finally, .. } => {
            let mut bodies: Vec<&[Node]> = vec![body, otherwise, finally];
            bodies.extend(catches.iter().map(|catch| catch.body.as_slice()));
            bodies
        }
        Node::LegacyIf { body, otherwise, .. } => vec![body, otherwise],
        Node::HotkeyDefinition { body, .. } => vec![body],
        _ => Vec::new(),
    }
}
