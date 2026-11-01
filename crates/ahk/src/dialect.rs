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
            .or_else(|| crate::v1::write_variable(name, op, value, line, rt))
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
