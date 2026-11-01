//! The dialects that the runtime speaks.
//!
//! The tree speaks AutoHotkey v2.0.19: that is the dialect of every script
//! that does not ask for another one, and the tables of `builtins.rs` and
//! `variables.rs` are its tables. The v1 is the legacy dialect of
//! AutoHotkey, measured on the binary of v1.1.37.02 on 2026-10-28, and it
//! lives beside the v2: a table of its own names and a set of rules of its
//! own.
//!
//! A dialect is a trait, so the runtime and the parser ask the dialect
//! instead of asking the tables of the v2. The gate is the `--dialect=v1`
//! of the runner: without it the tree speaks the v2, and with it the tree
//! speaks the v1.

use crate::builtins::Builtin;

/// The dialect that the runtime speaks.
///
/// Every rule of the tree that is not the same in the two dialects is a
/// method of this trait, and the tree asks the dialect before it does
/// anything. The rules are the ones that were measured on the binaries.
pub trait Dialect: Sync {
    /// The name of the dialect, for the report of the runner.
    fn name(&self) -> &'static str;

    /// The built-ins that the dialect carries.
    fn builtins(&self) -> &'static [Builtin];

    /// The table of the names of the dialect, and the table of the names
    /// that the tree does not carry yet.
    ///
    /// The tables are read when the crate is compiled, and the surface of
    /// the dialect is the list of its names.
    fn tables(&self) -> (&'static str, &'static str);

    /// Whether the dialect carries the built-in variable of this name.
    fn carries_variable(&self, name: &str) -> bool;

    /// The reason the dialect does not answer the built-in variable.
    fn refusal_of_variable(&self, name: &str) -> Option<&'static str>;

    /// The legacy command: a name at the start of a line, a comma, and the
    /// parameters of the command. Measured on the binary of v1.1.37.02 on
    /// 2026-11-01: `FileAppend, hello, *` writes `hello`, and the
    /// parameters of a command of the v1 are texts and not expressions.
    fn legacy_commands(&self) -> bool;

    /// The equal sign assigns and does not compare. Measured on the binary
    /// of v1.1.37.02 on 2026-11-01: `x = 5` leaves `x` at 5.
    fn assign_with_equal(&self) -> bool;

    /// The substitution of a parameter looks at the environment. Measured
    /// on the binary of v1.1.37.02 on 2026-11-01: `%TEMP%` is the value of
    /// the environment when the script has no variable of that name, and
    /// the directive `#NoEnv` turns the lookup off.
    fn percent_looks_at_the_environment(&self) -> bool;

    /// The hotstring is a statement: `::btw::by the way`.
    fn hotstrings_are_statements(&self) -> bool;

    /// Whether the dialect knows the directive of this name.
    ///
    /// The name is the one of the directive without the hash, and the
    /// case of it does not matter. A directive that the dialect does not
    /// know is not a directive at all: the line that holds it holds no
    /// action, and it is refused at the load.
    fn carries_directive(&self, name: &str) -> bool;

    /// A value that is not there is a blank result and not a failure.
    /// Measured on the binary of v1.1.37.02 on 2026-11-01: a variable that
    /// was never assigned is blank, and so is an index out of range.
    fn unset_is_blank(&self) -> bool;

    /// The division by zero is a blank result and not a failure. Measured
    /// on the binary of v1.1.37.02 on 2026-11-01: `1/0` is blank.
    fn division_by_zero_is_blank(&self) -> bool;

    /// The sum of a text and a number is a concatenation. Measured on the
    /// binary of v1.1.37.02 on 2026-11-01: `"5" + 1` is `51`.
    fn add_concatenates(&self) -> bool;
}

/// The dialect of the tree: AutoHotkey v2.0.19.
pub struct V2;

/// The legacy dialect: AutoHotkey v1.1.37.02.
pub struct V1;

/// The dialect of the tree, which is the one of every script that does not
/// ask for another one.
pub fn v2() -> &'static dyn Dialect {
    &V2
}

/// The legacy dialect, behind the gate of the `--dialect=v1`.
pub fn v1() -> &'static dyn Dialect {
    &V1
}

/// The reason a name of the v1 that the tree does not carry yet gives.
pub const NOT_CARRIED_YET: &str = "this runtime does not carry that name yet.";

/// The reason a name that is about Windows gives on a machine that is not.
pub const NOT_A_WINDOWS: &str = "this name is about Windows, and this machine is not.";

impl Dialect for V2 {
    fn name(&self) -> &'static str {
        "v2"
    }

    fn builtins(&self) -> &'static [Builtin] {
        crate::builtins::BUILTINS
    }

    fn tables(&self) -> (&'static str, &'static str) {
        (crate::surface::SURFACE_V2, crate::surface::PENDING_V2)
    }

    fn carries_variable(&self, name: &str) -> bool {
        crate::variables::carries(name)
    }

    fn refusal_of_variable(&self, name: &str) -> Option<&'static str> {
        crate::variables::refusal_of(name)
    }

    fn legacy_commands(&self) -> bool {
        false
    }

    fn assign_with_equal(&self) -> bool {
        false
    }

    fn percent_looks_at_the_environment(&self) -> bool {
        false
    }

    fn hotstrings_are_statements(&self) -> bool {
        false
    }

    fn carries_directive(&self, name: &str) -> bool {
        crate::lexer::carries_directive(name)
    }

    fn unset_is_blank(&self) -> bool {
        false
    }

    fn division_by_zero_is_blank(&self) -> bool {
        false
    }

    fn add_concatenates(&self) -> bool {
        false
    }
}

impl Dialect for V1 {
    fn name(&self) -> &'static str {
        "v1"
    }

    fn builtins(&self) -> &'static [Builtin] {
        crate::v1::BUILTINS
    }

    fn tables(&self) -> (&'static str, &'static str) {
        (crate::surface::SURFACE_V1, crate::surface::PENDING_V1)
    }

    fn carries_variable(&self, name: &str) -> bool {
        crate::variables::carries(name) || crate::v1::carries_variable(name)
    }

    fn refusal_of_variable(&self, name: &str) -> Option<&'static str> {
        if crate::variables::carries(name) {
            return crate::variables::refusal_of(name);
        }
        crate::v1::refusal_of_variable(name)
    }

    fn legacy_commands(&self) -> bool {
        true
    }

    fn assign_with_equal(&self) -> bool {
        true
    }

    fn percent_looks_at_the_environment(&self) -> bool {
        true
    }

    fn hotstrings_are_statements(&self) -> bool {
        true
    }

    fn carries_directive(&self, name: &str) -> bool {
        crate::lexer::carries_directive(name) || crate::v1::carries_directive(name)
    }

    fn unset_is_blank(&self) -> bool {
        true
    }

    fn division_by_zero_is_blank(&self) -> bool {
        true
    }

    fn add_concatenates(&self) -> bool {
        true
    }
}
