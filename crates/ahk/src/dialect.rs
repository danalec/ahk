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

use alloc::format;
use alloc::string::String;

use crate::ast::AssignOp;
use crate::builtins::Builtin;
use crate::runtime::{Runtime, RuntimeError, Value};

/// The refusal of the number of the parameters of a command.
///
/// The dialect answers with the complaint that it writes, and the parser
/// writes the text of the command that follows it.
#[derive(Debug, Clone, PartialEq)]
pub enum TheRefusalOfTheArity {
    /// The command does not have the parameters that it wants: the name of
    /// the command and the number of them are named.
    AtLeast {
        /// The name of the command, the way the dialect writes it.
        name: &'static str,
        /// The number of the parameters that the command wants.
        number: u8,
    },
    /// The parameter of this place has to be there.
    Required {
        /// The place of the parameter.
        number: u8,
    },
    /// The parameters of these places have to be there, the two of them
    /// together.
    RequiredTogether {
        /// The place of the first parameter.
        first: u8,
        /// The place of the second parameter.
        second: u8,
    },
}

/// The label that a parameter of a command names.
///
/// Measured on the binary of v1.1.37.02 on 2026-11-03: the `GroupAdd` names
/// a label of the script in its fourth parameter, the `Hotkey` in its second
/// and the `SetTimer` in its first, and the label that is not there is
/// refused when the script loads. The action of a hotkey and the body of a
/// timer may be a function of the script, and the name of the function is
/// accepted there.
#[derive(Debug, Clone, PartialEq)]
pub enum TheLabelOfACommand {
    /// The parameter names a label of the script.
    Label(usize),
    /// The parameter names a label of the script or a function of it.
    LabelOrFunction(usize),
}

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

    /// The place of the argument of this function that asks for the name of a
    /// variable.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IsSet` of
    /// the v1 asks for the name of the variable and not for its value --
    /// `IsSet(x)` asks whether the variable `x` has a value -- and the
    /// dialect says so. The v2 has no function that asks for a name.
    fn the_place_of_the_argument_that_wants_the_name(&self, name: &str) -> Option<usize> {
        let _ = name;
        None
    }

    /// The place of the argument of this function that asks for the variable
    /// of the caller and not for its value.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the
    /// `VarSetCapacity` of the v1 takes the variable of the caller and not
    /// its value -- `VarSetCapacity(y, 100)` empties `y`, and a text that is
    /// not the name of a variable is refused with the number zero -- and the
    /// variable is the first argument. The `StrReplace` of the v1 writes the
    /// number of the replacements into the variable of the fourth argument.
    /// The v2 writes the reference in the call itself (`&y`).
    fn the_place_of_the_argument_that_wants_the_variable(&self, name: &str) -> Option<usize> {
        let _ = name;
        None
    }

    /// Whether the dialect refuses the call of a function at the load.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
    /// the call of a name that is not a function at the load -- "Call to
    /// nonexistent function." -- and it refuses the number of the arguments
    /// that is wrong at the load too, and it names the call as it was
    /// written. The v2 refuses the two at the run, and a name that is not a
    /// function is a variable of the script whose value is called.
    fn refuses_the_call_of_a_function_at_the_load(&self) -> bool {
        false
    }

    /// Whether the command of the v1 has to be a command of the dialect.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: a function of the
    /// v1 is not a command: `StrLen "abc"` is a line that holds no action,
    /// and the load refuses it with the text of the line. The v2 has no
    /// command at all.
    fn the_command_has_to_be_a_command(&self) -> bool {
        false
    }

    /// Whether the command of this name has a body.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `IfInString`
    /// and the `IfMsgBox` of the v1 have an action: the command that is on
    /// the same line, the block, or the statement of the next line. The v2
    /// has no command that has a body.
    fn the_command_has_a_body(&self, name: &str) -> bool {
        let _ = name;
        false
    }

    /// Whether the command of this name is the `else` of the v1.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `else` of the
    /// v1 is a command of its own, and it takes no parameter. It is the
    /// action of the other side of the `if` that comes before it.
    fn the_command_is_the_else(&self, name: &str) -> bool {
        let _ = name;
        false
    }

    /// Whether the `if` of the dialect takes the text that follows the
    /// condition on the line of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `if` of the v1 parses the text that follows the condition and does
    /// not run it -- the `if (1) FileDelete, marker.txt` leaves the file
    /// where it is -- and the body of the `if` is the line that comes next.
    /// The v2 refuses the text with "Unexpected \"If\"".
    fn the_if_may_carry_a_tail(&self) -> bool {
        false
    }

    /// Whether the `if` of the dialect may be the legacy one, that compares
    /// a variable with the text that comes after it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// legacy `if` of the v1 compares the value of a variable with the text
    /// that comes after the operator, and the text is not an expression: the
    /// `if x = 1 + 1` compares the value of `x` with the text "1 + 1". The
    /// v2 has no legacy `if`: the `x = 1` is a comparison of the value of `x`
    /// with the number 1.
    fn the_if_may_be_a_legacy_comparison(&self) -> bool {
        false
    }

    /// Whether the `if` and the `while` of the dialect may come inside the
    /// line of the condition of an `if`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// v1 accepts the `if (0) if (1) x := 1`, and the construct that comes
    /// inside the line has no body of its own: its condition is a part of
    /// the condition that holds it. The v2 refuses the `if` that comes
    /// inside an expression.
    fn the_control_flow_may_be_a_value(&self) -> bool {
        false
    }

    /// Whether the bracket that a message names is named alone.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the bracket and nothing else: the `x := (1` is a "Missing ")" and
    /// the line of the script is not under it. The v2 names the line of the
    /// script under it.
    fn the_bracket_is_named_alone(&self) -> bool {
        false
    }

    /// Whether the body of a construct may be missing.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes none of them: the `if (1)` that is the last line of the script
    /// is a script that loads, and the `else` that has no body is a script
    /// that loads too. The v2 refuses the construct that has no body, and it
    /// says so in its own words: "Unexpected "If"".
    fn the_body_may_be_missing(&self) -> bool {
        false
    }

    /// Whether the assignment may have no value.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `y :=` writes the empty text, and the line that comes is a statement
    /// of its own: the `y :=` with a `3 + 4` under it is a line that holds
    /// no action. The v2 continues the assignment on the line that comes.
    fn the_assignment_may_have_no_value(&self) -> bool {
        false
    }

    /// Whether the assignment continues the tail of the `if`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `:=` at the end of the tail is an operator, and the tail takes the
    /// line that comes with it: the `if (0) x := 1, y :=` with a
    /// `FileAppend, A, done` under it does not run the body of the `if`.
    fn the_assignment_continues_the_tail(&self) -> bool {
        false
    }

    /// Whether the text that is not closed is named alone, with the words of
    /// the dialect.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says "Missing close-quote" and names the text that was read -- the
    /// `"abc` -- and the v2 says "Missing "\"" and names the line of the
    /// script.
    fn the_text_that_is_not_closed_is_named_alone(&self) -> bool {
        false
    }

    /// Whether a text that is a number of the dialect is false when the
    /// number is zero.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text of a condition of the v1 is a number when the syntax of the v1
    /// accepts it, and the condition is false when that number is zero: the
    /// "00" is false, and the "0.0" is false. The v2 keeps the text as it
    /// was written: the "00" is true, and only the text "0" is false.
    fn the_text_that_is_a_number_is_false(&self) -> bool {
        false
    }

    /// Whether the text that was written between quotes is a string of the
    /// dialect, and not a number.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// text that was written is a string, and a string is true when it is not
    /// empty: the `!("00")` is zero. The value of a variable is a number when
    /// it can be one, and the `!x` with `x` holding "00" is one. The v2 has
    /// no difference between the two.
    fn the_text_that_was_written_is_a_string(&self) -> bool {
        false
    }

    /// Whether the `&&` and the `||` of the dialect answer a number and not
    /// the operand.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `&&` and the `||` of the v1 answer one and zero, and not the operand:
    /// the `"a" && "b"` is 1. The v2 answers the operand: the last one that
    /// was true, or the first one that was not.
    fn the_operators_answer_a_number(&self) -> bool {
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

    /// Whether the dialect knows the operators that the v1 has and the v2
    /// does not.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// knows the `<>`, which is the `!=` of the v2, and the v2 does not know
    /// it: the `x <> 1` of the v2 is refused with a missing operand.
    fn the_lexer_knows_the_legacy_operators(&self) -> bool {
        false
    }

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

    /// Whether the hotkeys of the dialect are written in a line of their own.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the v1
    /// writes a hotkey in a line of its own, and the action of it is the
    /// label that comes after the line, or the command that the line
    /// carries. The v2 writes the hotkey in a call, and the action of it is
    /// a function of the script.
    fn the_hotkeys_are_labels(&self) -> bool;

    /// Whether the section of the script that runs ends at the first hotkey
    /// or at the first hotstring.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// v1 runs the code that comes before the first hotkey or the first
    /// hotstring, and the hotkey itself is registered when the section
    /// reaches it. The v2 has no section of its own.
    fn the_section_that_runs_ends_at_the_hotkey(&self) -> bool;

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

    /// Whether a number may be followed by a name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter of
    /// a command of the v1 is a text, and "2px" is a text there. The v2
    /// refuses the script at the load.
    fn the_number_may_be_followed_by_a_name(&self) -> bool {
        false
    }

    /// Whether a number of the dialect may be the name of a variable.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// names of the v1 may begin with a number, and the number alone is the
    /// name of a variable: the `0 := 9` writes the variable "0", and the
    /// `0 = 9` writes it too. The v2 refuses the number as the name of a
    /// variable.
    fn the_numbers_may_be_names(&self) -> bool {
        false
    }

    /// Whether the increment that comes after a number is an operator.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `++` and the `--` that come after a number are the increment of the
    /// v1, and the number that they follow is the name of a variable when the
    /// statement is about it: the `0++` is the increment of the variable "0".
    /// The v2 reads them as a subtraction: the `5--3` is five minus three.
    fn the_increment_after_a_number_is_an_operator(&self) -> bool {
        false
    }

    /// Whether the increment of a literal is the empty text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse the increment of a literal: the `x := ++5` is a script
    /// that runs, and the value of the expression is the empty text. The v2
    /// refuses it at the load.
    fn the_increment_of_a_literal_is_blank(&self) -> bool {
        false
    }

    /// Whether the exponent of a number needs a decimal point.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// scientific notation of the v1 needs a decimal point: the "1.0e4" is
    /// the number 10000, and the "1e2" is a name. The v2 reads the "1e2" as
    /// the number 100.
    fn the_exponent_needs_a_point(&self) -> bool {
        false
    }

    /// Whether a number keeps the text that was written.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// number of the v1 is a "pure number": it keeps the text that was
    /// written, and that text is what the value answers when it is read as a
    /// text -- the `x := 0x10` is "0x10", and the `StrLen(x)` is four. The
    /// whole number that does not fit in the machine is the largest one, and
    /// its text is the text of that number.
    fn the_number_keeps_the_text_that_was_written(&self) -> bool {
        false
    }

    /// Whether the class that a class holds is a member of the instance of
    /// it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// `o.D` is the class that C holds, and o is an instance of C. The v2
    /// has the class only in the class value of C.
    fn the_nested_class_is_a_member_of_the_instance(&self) -> bool {
        false
    }

    /// Whether the call of a class makes nothing.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// class of the v1 has no `Call`, and the call of it makes nothing: the
    /// `C.Call()` is the empty text, and so is the `C.D()` of the class
    /// that C holds. The instance of a class of the script is made with the
    /// `new`. The v2 calls the class, and the call of it makes an object of
    /// it.
    fn the_call_of_a_class_makes_nothing(&self) -> bool {
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

    /// Whether the name of a function is a value.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the name of a
    /// function of the v1 is not a value -- `x := StrLen` is a blank text --
    /// and the name of a command of the v1 is not a value either. The v2
    /// hands the function over as a value, and the value is of the class
    /// `Func`.
    fn the_name_of_a_function_is_a_value(&self) -> bool {
        true
    }

    /// The most parameters that a built-in function takes.
    ///
    /// The number of the table is the answer for the v2. Measured on the
    /// binary of v1.1.37.02 on 2026-11-02: the `MaxParams` of the `Func` of
    /// the v1 is ten thousand for a built-in that takes a variable number of
    /// arguments -- `Format` takes two in the table and ten thousand in the
    /// v1 -- and it is the number of the table for the others.
    fn the_maximum_of_the_parameters(&self, max: u8) -> u32 {
        max as u32
    }

    /// Whether a member that a value does not have is answered.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
    /// refuse a member that is not there, and the answer is a blank text --
    /// the property of an object that does not have it, and the property of a
    /// value that is not an object. The v2 refuses with the reason.
    fn a_member_that_is_not_there_is_answered(&self) -> bool {
        false
    }

    /// Whether an object has no text.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the text of an
    /// object of the v1 is the empty text -- `Object()` is a blank text, and
    /// `Object() . "a"` is "a". The v2 gives the class of the object.
    fn an_object_has_no_text(&self) -> bool {
        false
    }

    /// Whether the `new` of the dialect makes an instance of a class.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has the
    /// `new`, and the word is not reserved -- `new := 5` is a variable of
    /// the script. The v2 does not have it: the class is called there.
    fn the_new_makes_an_instance(&self) -> bool {
        false
    }

    /// Whether the method of the class object is a method of the class.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 calls
    /// `C.Say()`, and the `this` of the method is the class object. The v2
    /// refuses it: "This value of type \"Class\" has no method named
    /// \"Say\".".
    fn a_method_of_the_class_object_is_a_method_of_the_class(&self) -> bool {
        false
    }

    /// Whether the call of a method checks the number of the arguments.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
    /// check it -- a method that is called with too few arguments takes the
    /// missing ones empty, and the `__Init` and the `__New` of a class are
    /// called the same way. The v2 refuses the call.
    fn the_call_of_a_method_checks_the_arity(&self) -> bool {
        true
    }

    /// Whether every value carries the class of it.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the `__Class` of
    /// the v1 is a property of an object: a value that is not an object has
    /// none, and the answer is the empty text. A class of the script is
    /// named after itself. The v2 gives every value a class: "Integer",
    /// "String", "Class" and the name of the class of the script.
    fn every_value_carries_its_class(&self) -> bool {
        true
    }

    /// Whether the built-in checks the kind of its arguments.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does not
    /// check it -- the built-in of the v1 takes the text that holds a number
    /// and does not fail -- and the v2 refuses the value of another kind.
    fn the_built_in_checks_the_kind_of_its_arguments(&self) -> bool {
        true
    }

    /// Whether the parameter at this place of this built-in wants a number.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter of
    /// the v1 that wants a number takes the number that the text of it starts
    /// with -- "65abc" is 65 -- and the hex of the v1 is a number: "0x10" is
    /// 16. The v2 refuses the value of another kind, and it does not read the
    /// number of a text: it wants the number itself.
    fn the_parameter_wants_a_number(&self, _name: &str, _index: usize) -> bool {
        false
    }

    /// Whether a whole number that does not fit is the largest one.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 keeps the
    /// whole numbers in sixty-four bits, and the number that is written and
    /// does not fit is the largest of them: `x := 100000000000000000000` is
    /// 9223372036854775807.
    fn a_whole_number_that_does_not_fit_is_the_largest_one(&self) -> bool {
        false
    }

    /// Whether the body of a class has one set of names.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses a name that is declared twice in the body of a class, and it
    /// does not care what the member is: a method and a variable with the
    /// same name are a duplicate, and so are a static member and one that is
    /// not. The v2 has a set for each kind of member.
    fn the_names_of_the_body_of_a_class_are_one_set(&self) -> bool {
        false
    }

    /// Whether the static method of a class is refused.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has no
    /// static method, and it refuses it with "Invalid class variable
    /// declaration.\n     Specifically: __Init() {". The v2 has the static
    /// method.
    fn the_static_method_is_refused(&self) -> bool {
        false
    }

    /// Whether the property that has a getter or a setter is refused.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the class of the v1
    /// has no property with a getter, and the refusal is the one of the v1:
    /// the getter that is on the same line as the name with "Not a valid
    /// method, class or property definition.\n     Specifically: x { get => 1
    /// }", and the getter that is in a block with "Not a valid property
    /// getter/setter.\n     Specifically: get => 1". The v2 has the property
    /// with a getter.
    fn the_accessor_of_a_class_is_refused(&self) -> bool {
        false
    }

    /// Whether the refusal of a duplicate names the member that was written.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// names the member: "Duplicate declaration.\n     Specifically: x". The
    /// v2 names the path of it: "C.Prototype.Foo".
    fn the_duplicate_names_the_member_that_was_written(&self) -> bool {
        false
    }

    /// Whether a class that is declared twice is refused with the words of
    /// the dialect.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses it with "Duplicate class definition.\n     Specifically: C",
    /// and the v2 with "This class declaration conflicts with an existing
    /// Class.".
    fn the_class_that_is_declared_twice_has_its_own_words(&self) -> bool {
        false
    }

    /// Whether the name of a class or the name of a function may be the
    /// output variable of an assignment.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// takes the name of a class and the name of a function of the machine as
    /// the name of a variable: `C := 5` and `Abs := 5` are accepted, and the
    /// value of the name is the one that was written. The v2 refuses the
    /// first one with "This Class cannot be used as an output variable." and
    /// the second one with "This Func cannot be used as an output
    /// variable.".
    fn the_name_of_a_class_or_of_a_function_may_be_assigned(&self) -> bool {
        false
    }

    /// Whether a class and a function may carry the same name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// takes a class and a function of the same name, in either order: the
    /// call goes to the function and the `new` goes to the class. The v2
    /// refuses the second one with "This function declaration conflicts with
    /// an existing Class.".
    fn a_class_and_a_function_may_have_the_same_name(&self) -> bool {
        false
    }

    /// Whether the function that is declared twice has the words of the
    /// dialect.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses it with "Duplicate function definition.\n     Specifically:
    /// C()", and the name is written with the parenthesis of the call and
    /// the way the script spelled it. The v2 refuses it with "This function
    /// declaration conflicts with an existing Func.".
    fn the_function_that_is_declared_twice_has_its_own_words(&self) -> bool {
        false
    }

    /// Whether the class of the dialect is a variable of the script.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// class of the v1 is a variable: the `new` reads the variable, and the
    /// name of a class that the script assigned something else makes
    /// nothing. The v2 has no `new`, and the name of a class is the name of
    /// a class.
    fn the_class_is_a_variable(&self) -> bool {
        false
    }

    /// Whether the call of a built-in wins over the variable that carries its
    /// name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// takes the name of a built-in as the name of a variable, and the call
    /// still goes to the built-in: `Abs := 5` and then `Abs(3)` answers 3.
    /// The v2 refuses the assignment, and the question does not arise.
    fn the_call_of_a_built_in_wins_over_the_variable(&self) -> bool {
        false
    }

    /// Whether the name of a class may be a word of the language.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// takes the name of a class as it is written, and a word of the language
    /// is a name of it: the `class if` is a class, and the `new if` makes an
    /// instance of it. The v2 refuses the name with "The following reserved
    /// word must not be used as a class name.".
    fn the_name_of_a_class_may_be_a_word_of_the_language(&self) -> bool {
        false
    }

    /// Whether the refusal of the brace that opens a class names the class.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// names the text that comes after the word "class": the `class C { }` is
    /// a "Missing \"{\"\n     Specifically: C { }". The v2 names the whole of
    /// the line.
    fn the_missing_brace_of_a_class_names_the_class(&self) -> bool {
        false
    }

    /// Whether the class that is not closed names the class.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses it with "Missing \"}\"\n     Specifically: C", and the name of
    /// the class is in it even when the file ends inside a method of it.
    fn the_class_that_is_not_closed_names_the_class(&self) -> bool {
        false
    }

    /// Whether the word "class" alone holds an action.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses the line with "This line does not contain a recognized
    /// action.\n     Specifically: class". The v2 reads the word as a class
    /// of the dialect.
    fn the_word_class_alone_holds_an_action(&self) -> bool {
        true
    }

    /// Whether the base of a class may be a class of the runtime.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// has no class types, and a base that is not a class of the script is
    /// refused with "Unknown class.\n     Specifically: Object".
    fn the_base_may_be_a_class_of_the_runtime(&self) -> bool {
        true
    }

    /// Whether the scope of a function is its own.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// gives every function its own scope: the function does not see the
    /// scope of the script, nor the scopes around the call, and only the
    /// `global` declaration makes a name of the script visible in it. The v2
    /// reads the scope of the script when the name is only read.
    fn the_scope_of_a_function_is_its_own(&self) -> bool {
        false
    }

    /// Whether the `base` of the dialect is the base of the class where the
    /// method was defined.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has the
    /// `base`, and `base.Method()` calls the method of the base of the class
    /// where the method was defined. The v2 has the `super`, and the base
    /// of an object is the prototype of its class.
    fn the_base_is_the_base_of_the_class(&self) -> bool {
        false
    }

    /// Whether the default value of a parameter may be written with the
    /// equal sign.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 writes it
    /// with the equal sign as well -- `f(a = 5)` and `f(a := 5)` are the
    /// same -- and the v2 knows only the two points.
    fn the_default_of_a_parameter_may_be_written_with_the_equal_sign(&self) -> bool {
        false
    }

    /// The refusal of a `global` that names a built-in variable.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
    /// it at the load -- "Built-in variables must not be declared.\n
    ///     Specifically: ErrorLevel" -- and the refusal names the variable.
    /// The v2 allows it: the global names the built-in variable of the
    /// dialect and not a variable of the script.
    ///
    /// `None` means that the dialect allows the global.
    fn the_refusal_of_a_global_of_a_built_in_variable(&self, _name: &str) -> Option<String> {
        None
    }

    /// The refusal of the write of a built-in variable.
    ///
    /// The refusal is at the load, and it names the variable as the script
    /// spelled it. `None` means that the dialect lets the script write the
    /// name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// refuses the write of every built-in variable of the dialect, and the
    /// reason is the same for every operator -- "Not allowed as an output
    /// variable.\n     Specifically: A_AhkPath". The concat is the one that
    /// the v1 accepts: the built-in keeps the value that it had.
    fn the_refusal_of_the_write_of_a_built_in_variable(
        &self,
        _name: &str,
        _op: AssignOp,
    ) -> Option<String> {
        None
    }

    /// The refusal of the increment of a built-in variable.
    ///
    /// The refusal is at the load, and it names the variable as the script
    /// spelled it. `None` means that the dialect lets the script increment
    /// the name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// increment of a built-in variable of the v1 is refused with the reason
    /// of the write, and the refusal is the one of the line that is the
    /// increment alone: the increment that is inside an expression, or the
    /// one that has something after it, is accepted.
    fn the_refusal_of_the_increment_of_a_built_in_variable(&self, _name: &str) -> Option<String> {
        None
    }

    /// The refusal of a parameter of a command that the dialect does not
    /// want.
    ///
    /// The refusal is at the load, and it names the parameter and the text
    /// that the script wrote. The name is the name of the command as the
    /// script wrote it, and the parameters are the texts of the parameters,
    /// as they were written. `None` means that the dialect lets the command
    /// run with the parameter that it has.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// checks the parameter of some of its commands when the script loads,
    /// and the complaint of the v1 is "Parameter #N invalid." with the text
    /// that the script wrote in the second line of it. The v2 checks the
    /// value of the parameter when the command runs.
    fn the_refusal_of_an_invalid_parameter(
        &self,
        _name: &str,
        _parameters: &[String],
    ) -> Option<String> {
        None
    }

    /// Whether the name of a variable of the script may carry the "#", the
    /// "$" and the "@".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name of a
    /// variable of the v1 may carry them, and the v2 does not read them.
    fn accepts_a_hash_a_dollar_and_an_at_in_a_name(&self) -> bool {
        false
    }

    /// Whether the parameter of this place of this command has to be there.
    ///
    /// The place is the number of the parameter, and the name is the name
    /// of the command as the script wrote it. The parser refuses the
    /// parameter that is blank, and the complaint is the one of the v1:
    /// "\"X\" requires that parameter #N be non-blank.".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
    /// sub-command of a command of the v1 has to be there, and the v2 has
    /// no command that wants a parameter of its own.
    fn the_parameter_may_not_be_blank(&self, _name: &str, _place: usize) -> bool {
        false
    }

    /// The label that a parameter of this command names.
    ///
    /// The number is the place of the parameter, counted from zero, and
    /// `None` means that the command does not name a label.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-03: the `GroupAdd`
    /// names a label of the script in its fourth parameter, the `Hotkey` in
    /// its second and the `SetTimer` in its first, and the label that is not
    /// there is refused when the script loads. The v2 has no command that
    /// names a label.
    fn the_label_of_the_command(&self, _name: &str) -> Option<TheLabelOfACommand> {
        None
    }

    /// Whether the dialect refuses the goto whose label is not in the scope
    /// of the jump, with the labels of the script.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the v2 refuses the
    /// goto of a label that is not in the scope of the jump when the labels
    /// of the script are checked, and the complaint of it is "Label not
    /// found in current scope.". The v1 refuses the jump at the end of the
    /// load, with the complaint of the v1.
    fn the_goto_is_refused_with_the_labels(&self) -> bool {
        true
    }

    /// The complaint of the jump that may not be made.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-01: the v2 says "A Goto
    /// must not jump into a block that doesn't enclose it.". The v1 says
    /// "A Goto/Gosub must not jump into a block that doesn't enclose it.".
    fn the_refusal_of_a_jump_into_a_block(&self) -> &'static str {
        "A Goto must not jump into a block that doesn't enclose it."
    }

    /// The refusal of the name of a variable of the script.
    ///
    /// The place is the number of the parameter, and the text is the text
    /// that the script wrote. `None` means that the dialect lets the command
    /// run with the name that it has.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1 checks the
    /// name of a variable when the script loads, and the complaint of the v1
    /// is its own. The v2 has no command that writes into a variable of the
    /// script, and the parser keeps its own complaint.
    fn the_refusal_of_the_name_of_a_variable(
        &self,
        _place: usize,
        _text: &str,
    ) -> Option<String> {
        None
    }

    /// The refusal of the number of the parameters of a command.
    ///
    /// `None` means that the dialect lets the command run with the number of
    /// the parameters that it has, and that the parser answers with its own
    /// complaint.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// has two forms of the complaint, and the command says which one is the
    /// one of it. The one that names the command is "\"X\" requires at least
    /// N parameter(s)." with the text of the command after it; the other one
    /// names the parameter that has to be there and has no text: "Parameter #N
    /// required". The v2 has no command, and the parser keeps the first form.
    fn the_refusal_of_the_arity_of_a_command(
        &self,
        _name: &str,
        _count: usize,
        _minimum: u8,
    ) -> Option<TheRefusalOfTheArity> {
        None
    }

    /// The defaults of the settings of the script.
    ///
    /// The v1 and the v2 do not begin with the same values: the mode of the
    /// coordinates of the v1 is the "Window" and the mode of the v2 is the
    /// "Client". The runtime asks the dialect when it is made, and the
    /// dialect writes the defaults of its own.
    fn the_defaults_of_the_settings(&self) {
        crate::settings::the_defaults_of_the_v2();
    }

    /// Whether the space separates the increment from the name.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the space is what
    /// tells the operator of the increment apart from the concat -- `x++` is
    /// the increment of `x`, and `x ++` is a call of `x`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// has no call without parentheses, and the space does not separate the
    /// operator: the `x ++` is the increment of `x`.
    fn the_space_separates_the_increment(&self) -> bool {
        true
    }

    /// Whether the increment of a built-in variable changes nothing.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// does not let a script write a built-in variable, and the increment of
    /// one changes nothing: the value is the one that the built-in has.
    fn the_increment_of_a_built_in_variable_changes_nothing(&self, _name: &str) -> bool {
        false
    }

    /// Whether the write of a built-in variable goes nowhere.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
    /// accepts the concat on a built-in variable, and the built-in keeps the
    /// value that it had. The v2 refuses the write of the name.
    fn the_write_of_a_built_in_variable_goes_nowhere(&self, _name: &str, _op: AssignOp) -> bool {
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

    fn the_refusal_of_the_write_of_a_built_in_variable(&self, name: &str, op: AssignOp) -> Option<String> {
        // Measured on the binary of v2.0.19 on 2026-10-31: the dialect refuses
        // the name when the script writes it, and it names the name as the
        // script spelled it.
        if !crate::variables::is_read_only(name) {
            return None;
        }
        Some(crate::variables::read_only(name, op))
    }

    fn the_place_of_the_argument_that_wants_the_name(&self, _name: &str) -> Option<usize> {
        None
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

    fn the_hotkeys_are_labels(&self) -> bool {
        false
    }

    fn the_section_that_runs_ends_at_the_hotkey(&self) -> bool {
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

    fn the_place_of_the_argument_that_wants_the_name(&self, name: &str) -> Option<usize> {
        crate::v1::the_place_of_the_argument_that_wants_the_name(name)
    }

    fn the_place_of_the_argument_that_wants_the_variable(&self, name: &str) -> Option<usize> {
        crate::v1::the_place_of_the_argument_that_wants_the_variable(name)
    }

    fn the_command_has_a_body(&self, name: &str) -> bool {
        crate::v1::the_command_has_a_body(name)
    }

    fn refuses_the_call_of_a_function_at_the_load(&self) -> bool {
        true
    }

    fn the_command_has_to_be_a_command(&self) -> bool {
        true
    }

    fn the_command_is_the_else(&self, name: &str) -> bool {
        crate::v1::the_command_is_the_else(name)
    }

    fn the_if_may_carry_a_tail(&self) -> bool {
        true
    }

    fn the_if_may_be_a_legacy_comparison(&self) -> bool {
        true
    }

    fn the_control_flow_may_be_a_value(&self) -> bool {
        true
    }

    fn the_bracket_is_named_alone(&self) -> bool {
        // The v1 names the bracket and nothing else: the `x := (1` is a
        // "Missing ")" and the line of the script is not under it.
        true
    }

    fn the_body_may_be_missing(&self) -> bool {
        // The v1 takes none of them: the `if (1)` that is the last line of
        // the script is a script that loads.
        true
    }

    fn the_assignment_may_have_no_value(&self) -> bool {
        // The `y :=` writes the empty text, and the line that comes is a
        // statement of its own.
        true
    }

    fn the_assignment_continues_the_tail(&self) -> bool {
        // The `:=` at the end of the tail is an operator, and the tail takes
        // the line that comes with it.
        true
    }

    fn the_text_that_is_not_closed_is_named_alone(&self) -> bool {
        // The v1 says "Missing close-quote" and names the text that was read.
        true
    }

    fn the_text_that_is_a_number_is_false(&self) -> bool {
        true
    }

    fn the_text_that_was_written_is_a_string(&self) -> bool {
        true
    }

    fn the_operators_answer_a_number(&self) -> bool {
        true
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

    fn the_lexer_knows_the_legacy_operators(&self) -> bool {
        // The v1 knows the `<>`, which is the `!=` of the v2.
        true
    }

    fn lookup_variable(
        &self,
        name: &str,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        // The v1 carries the variables of the v2, and the ones of its own:
        // the name is the one that the dialect spells. The ones that the v1
        // spells in its own way come first: the flag of the v1 is the "On"
        // and the "Off", and the number one and zero of the v2.
        crate::v1::lookup_variable(name, rt).or_else(|| crate::variables::lookup(name, rt))
    }

    fn write_variable(
        &self,
        name: &str,
        op: AssignOp,
        value: &Value,
        line: u32,
        rt: &mut Runtime,
    ) -> Option<Result<Value, RuntimeError>> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the write of a built-in variable with its own reason, and the rule
        // of the v1 is the wider one: it comes first.
        crate::v1::write_variable(name, op, value, line, rt)
            .or_else(|| crate::variables::write(name, op, value, line, rt))
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

    fn the_hotkeys_are_labels(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // v1 writes a hotkey in a line of its own.
        true
    }

    fn the_section_that_runs_ends_at_the_hotkey(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
        // code that comes after the first hotkey or the first hotstring of
        // the v1 does not run.
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

    fn the_number_may_be_followed_by_a_name(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 reads
        // a name that begins with a number, and the parameter of a command
        // of the v1 is a text.
        true
    }

    fn the_numbers_may_be_names(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // names of the v1 may begin with a number, and the number alone is
        // the name of a variable.
        true
    }

    fn the_increment_after_a_number_is_an_operator(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `++` and the `--` that come after a number are the increment of the
        // v1: the `0++` is the increment of the variable "0".
        true
    }

    fn the_increment_of_a_literal_is_blank(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not refuse the increment of a literal, and the value of it
        // is the empty text.
        true
    }

    fn the_exponent_needs_a_point(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // scientific notation of the v1 needs a decimal point.
        true
    }

    fn the_number_keeps_the_text_that_was_written(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the number of
        // the v1 is a "pure number", and it keeps the text that was written.
        true
    }

    fn a_value_that_is_not_a_number_is_blank(&self) -> bool {
        true
    }

    fn the_nested_class_is_a_member_of_the_instance(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // `o.D` is the class that C holds, and o is an instance of C.
        true
    }

    fn the_call_of_a_class_makes_nothing(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // class of the v1 has no `Call`, and the instance of it is made with
        // the `new`.
        true
    }

    fn the_name_of_a_function_is_a_value(&self) -> bool {
        // The name of a function of the v1 is not a value, and the name of a
        // command of the v1 is not a value either.
        false
    }

    fn the_maximum_of_the_parameters(&self, max: u8) -> u32 {
        // The table of the v1 says 255 when the built-in takes any number of
        // arguments, and the v1 counts ten thousand there.
        if max == 255 {
            10000
        } else {
            max as u32
        }
    }

    fn a_member_that_is_not_there_is_answered(&self) -> bool {
        // The v1 answers a member that a value does not have.
        true
    }

    fn an_object_has_no_text(&self) -> bool {
        // The text of an object of the v1 is the empty text.
        true
    }

    fn the_refusal_of_a_global_of_a_built_in_variable(&self, name: &str) -> Option<String> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the `global` of a built-in variable.
        if crate::v1::may_not_be_declared(name) {
            return Some(format!(
                "Built-in variables must not be declared.\n     Specifically: {name}"
            ));
        }
        None
    }

    fn the_refusal_of_the_write_of_a_built_in_variable(&self, name: &str, op: AssignOp) -> Option<String> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the write of every built-in variable of the dialect -- the ones that
        // the v2 lets a script write as well -- and the reason is the same
        // for every operator. The concat is the one that the v1 accepts: the
        // built-in keeps the value that it had.
        if op == AssignOp::Concat {
            return None;
        }
        if !crate::v1::is_read_only(name) {
            return None;
        }
        Some(format!("Not allowed as an output variable.\n     Specifically: {name}"))
    }

    fn the_refusal_of_the_increment_of_a_built_in_variable(&self, name: &str) -> Option<String> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the increment of
        // a built-in variable of the v1 is refused at the load, and the reason
        // is the one of the write.
        if !crate::v1::is_read_only(name) {
            return None;
        }
        Some(format!("Not allowed as an output variable.\n     Specifically: {name}"))
    }

    fn the_refusal_of_an_invalid_parameter(&self, name: &str, parameters: &[String]) -> Option<String> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 checks
        // the parameter of its commands when the script loads, and the
        // complaint of the v1 is its own.
        crate::v1::the_refusal_of_a_parameter(name, parameters)
    }

    fn accepts_a_hash_a_dollar_and_an_at_in_a_name(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the name of a
        // variable of the v1 may carry the "#", the "$" and the "@".
        true
    }

    fn the_parameter_may_not_be_blank(&self, name: &str, place: usize) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
        // sub-command of a command of the v1 has to be there, and the
        // complaint of the v1 is the one of the parameter that may not be
        // blank.
        crate::v1::the_parameter_may_not_be_blank(name, place)
    }

    fn the_label_of_the_command(&self, name: &str) -> Option<TheLabelOfACommand> {
        // Measured on the binary of v1.1.37.02 on 2026-11-03: the label that
        // a command of the v1 names is checked when the script loads.
        crate::v1::the_label_of_the_command(name)
    }

    fn the_goto_is_refused_with_the_labels(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-03: the v1 refuses
        // the jump at the end of the load, and not with the labels.
        false
    }

    fn the_refusal_of_a_jump_into_a_block(&self) -> &'static str {
        // Measured on the binary of v1.1.37.02 on 2026-11-03: the v1 names
        // the `Gosub` as well.
        "A Goto/Gosub must not jump into a block that doesn't enclose it."
    }

    fn the_refusal_of_the_name_of_a_variable(
        &self,
        _place: usize,
        text: &str,
    ) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1 checks
        // the name of a variable of the script when the script loads, and
        // the complaint of the v1 is its own.
        crate::v1::the_refusal_of_the_name_of_a_variable(text)
    }

    fn the_refusal_of_the_arity_of_a_command(
        &self,
        name: &str,
        count: usize,
        _minimum: u8,
    ) -> Option<TheRefusalOfTheArity> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has two
        // forms of the complaint of the number of the parameters, and the
        // commands that are not in the table of the v1 answer with the one
        // that names the command.
        crate::v1::the_refusal_of_the_arity(name, count)
    }

    fn the_defaults_of_the_settings(&self) {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 begins
        // with the mode of the coordinates in the "Window", with the method
        // of the sending in the "Event" and with the mode of the matching of
        // the titles in 1.
        crate::settings::the_defaults_of_the_v1();
    }

    fn the_space_separates_the_increment(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
        // has no call without parentheses, and the space does not separate the
        // operator of the increment from the name.
        false
    }

    fn the_increment_of_a_built_in_variable_changes_nothing(&self, name: &str) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1
        // does not let a script write a built-in variable, and the increment of
        // one changes nothing.
        crate::v1::is_read_only(name)
    }

    fn the_write_of_a_built_in_variable_goes_nowhere(&self, name: &str, op: AssignOp) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the concat on a
        // built-in variable of the v1 is accepted, and the built-in keeps the
        // value that it had: the write goes nowhere.
        op == AssignOp::Concat && crate::v1::is_read_only(name)
    }

    fn the_new_makes_an_instance(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has
        // the `new`.
        true
    }

    fn a_method_of_the_class_object_is_a_method_of_the_class(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 calls
        // the method of the class object.
        true
    }

    fn the_call_of_a_method_checks_the_arity(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not check the number of the arguments of a method.
        false
    }

    fn every_value_carries_its_class(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the `__Class`
        // of the v1 is a property of an object.
        false
    }

    fn the_built_in_checks_the_kind_of_its_arguments(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 does
        // not check the kind of the arguments.
        false
    }

    fn the_parameter_wants_a_number(&self, name: &str, index: usize) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the parameter
        // of the v1 that wants a number takes the number that the text of it
        // starts with.
        crate::v1::wants_a_number(name, index)
    }

    fn a_whole_number_that_does_not_fit_is_the_largest_one(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the whole
        // number that is written and does not fit is the largest one.
        true
    }

    fn the_names_of_the_body_of_a_class_are_one_set(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the body of a
        // class of the v1 has one set of names.
        true
    }

    fn the_static_method_is_refused(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has no
        // static method.
        true
    }

    fn the_accessor_of_a_class_is_refused(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the class of the
        // v1 has no property with a getter.
        true
    }

    fn the_duplicate_names_the_member_that_was_written(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the refusal
        // names the member that was written.
        true
    }

    fn the_class_that_is_declared_twice_has_its_own_words(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the class that is declared twice with "Duplicate class
        // definition.".
        true
    }

    fn the_name_of_a_class_or_of_a_function_may_be_assigned(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 takes
        // the name of a class and the name of a function of the machine as
        // the name of a variable.
        true
    }

    fn a_class_and_a_function_may_have_the_same_name(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 takes a
        // class and a function of the same name, in either order.
        true
    }

    fn the_function_that_is_declared_twice_has_its_own_words(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the function that is declared twice with "Duplicate function
        // definition.".
        true
    }

    fn the_class_is_a_variable(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the class of
        // the v1 is a variable of the script, and the `new` reads it.
        true
    }

    fn the_call_of_a_built_in_wins_over_the_variable(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 calls
        // the built-in and not the variable that carries its name.
        true
    }

    fn the_name_of_a_class_may_be_a_word_of_the_language(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 takes
        // the name of a class as it is written.
        true
    }

    fn the_missing_brace_of_a_class_names_the_class(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 names
        // the text that comes after the word "class".
        true
    }

    fn the_class_that_is_not_closed_names_the_class(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 names
        // the class when the brace that closes it is not there.
        true
    }

    fn the_word_class_alone_holds_an_action(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 refuses
        // the word alone.
        false
    }

    fn the_base_may_be_a_class_of_the_runtime(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has no
        // class types, and the base of a class is a class of the script.
        false
    }

    fn the_scope_of_a_function_is_its_own(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: every function
        // of the v1 has its own scope.
        true
    }

    fn the_base_is_the_base_of_the_class(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 has
        // the `base`.
        true
    }

    fn the_default_of_a_parameter_may_be_written_with_the_equal_sign(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the v1 writes
        // the default value of a parameter with the equal sign as well.
        true
    }
}
