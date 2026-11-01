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

use crate::ast::AssignOp;
use crate::builtins::Builtin;
use crate::runtime::{Runtime, RuntimeError, Value};

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
    ///
    /// The name is the one that the dialect spells: `A_Clipboard` and not
    /// `Clipboard`.
    fn carries_variable(&self, name: &str) -> bool;

    /// The reason the dialect does not answer the built-in variable.
    ///
    /// `None` means the dialect answers the name.
    fn refusal_of_variable(&self, name: &str) -> Option<&'static str>;

    /// Whether the function of this name asks for the name of a variable.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IsSet` of
    /// the v1 asks for the name of the variable and not for its value --
    /// `IsSet(x)` asks whether the variable `x` has a value -- and the
    /// `IsFunc` of the same binary asks for the value. The v2 has no
    /// function that asks for a name.
    fn wants_the_name_of_the_argument(&self, name: &str) -> bool;

    /// Whether the function of this name asks for the variable itself.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the
    /// `VarSetCapacity` of the v1 takes the variable of the caller and not
    /// its value: `VarSetCapacity(y, 100)` empties `y`, and a text that is
    /// not the name of a variable is refused with the number zero. The v2
    /// writes the reference in the call itself (`&y`).
    fn wants_the_variable_of_the_argument(&self, name: &str) -> bool {
        let _ = name;
        false
    }

    /// The class of an object of the dialect.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the object of
    /// the v1 has no class, and its `__Class` is empty. The v2 has the
    /// base class, and the object of it is of that class.
    fn class_of_an_object(&self) -> &'static str;

    /// Whether the dialect has the class type of this name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has no
    /// class types at all, and the name `Func` of it is a function and not
    /// a class. The v2 has the class types of the runtime.
    fn carries_the_class_of_the_runtime(&self, name: &str) -> bool;

    /// Whether the dialect accepts the backslash outside of a text.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the backslash
    /// of a command of the v1 is a letter of the text of the parameter --
    /// `FileRead, text, C:\temp\x.txt` is a command of the v1, and the
    /// binary accepts it. The v2 refuses the backslash outside of a text.
    fn accepts_a_bare_backslash(&self) -> bool;

    /// The value of the built-in variable of this name, when the name is one.
    ///
    /// `None` means the name is not a built-in variable of the dialect: the
    /// runtime keeps it in the table of the script.
    fn lookup_variable(
        &self,
        name: &str,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>>;

    /// The value that the write of the built-in variable of this name leaves.
    ///
    /// `None` means the name is not a built-in variable of the dialect, or
    /// that the dialect lets a script write it and the runtime keeps the
    /// value in the table of the script.
    fn write_variable(
        &self,
        name: &str,
        op: AssignOp,
        value: &Value,
        line: u32,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>>;

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

    /// Whether the dialect formats the numbers.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `SetFormat`
    /// of the v1 formats the numbers, and the v2 writes them the way they
    /// are.
    fn formats_the_numbers(&self) -> bool {
        false
    }

    /// Whether the float that is written is a text.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the float that
    /// is written in the v1 keeps the text that was written: `x := 1.5` is
    /// "1.5", and the format of the floats does not change it.
    fn the_float_that_is_written_is_a_text(&self) -> bool {
        false
    }

    /// Whether a number may be followed by a name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter of
    /// a command of the v1 is a text, and "2px" is a text there. The v2
    /// refuses the script at the load.
    fn the_number_may_be_followed_by_a_name(&self) -> bool {
        false
    }

    /// Whether a value that is not a number makes the operation blank.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the value of a
    /// variable is a number when it can be one, and the operation that
    /// cannot have it is blank. The v2 refuses with the reason.
    fn a_value_that_is_not_a_number_is_blank(&self) -> bool {
        false
    }
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

    fn wants_the_name_of_the_argument(&self, _name: &str) -> bool {
        false
    }

    fn class_of_an_object(&self) -> &'static str {
        "Object"
    }

    fn carries_the_class_of_the_runtime(&self, name: &str) -> bool {
        crate::classes::is_type(name)
    }

    fn accepts_a_bare_backslash(&self) -> bool {
        false
    }

    fn lookup_variable(
        &self,
        name: &str,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        crate::variables::lookup(name, rt)
    }

    fn write_variable(
        &self,
        name: &str,
        op: AssignOp,
        value: &Value,
        line: u32,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        crate::variables::write(name, op, value, line, rt)
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

    fn wants_the_name_of_the_argument(&self, name: &str) -> bool {
        crate::v1::wants_the_name_of_the_argument(name)
    }

    fn wants_the_variable_of_the_argument(&self, name: &str) -> bool {
        crate::v1::wants_the_variable_of_the_argument(name)
    }

    fn class_of_an_object(&self) -> &'static str {
        // The v1 has no classes: the object of it has no class, and its
        // `__Class` is empty.
        ""
    }

    fn carries_the_class_of_the_runtime(&self, _name: &str) -> bool {
        // The v1 has no class types: the name `Func` of it is a function.
        false
    }

    fn accepts_a_bare_backslash(&self) -> bool {
        // The path of a file is the use that every command of the v1 has,
        // and the path of Windows has backslashes in it.
        true
    }

    fn lookup_variable(
        &self,
        name: &str,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        // The v1 carries the variables of the v2, and the ones of its own:
        // the name is the one that the dialect spells.
        crate::variables::lookup(name, rt).or_else(|| crate::v1::lookup_variable(name, rt))
    }

    fn write_variable(
        &self,
        name: &str,
        op: AssignOp,
        value: &Value,
        line: u32,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        crate::variables::write(name, op, value, line, rt)
            .or_else(|| crate::v1::write_variable(name, op, line))
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

    fn formats_the_numbers(&self) -> bool {
        true
    }

    fn the_float_that_is_written_is_a_text(&self) -> bool {
        true
    }

    fn the_number_may_be_followed_by_a_name(&self) -> bool {
        true
    }

    fn a_value_that_is_not_a_number_is_blank(&self) -> bool {
        true
    }
}
