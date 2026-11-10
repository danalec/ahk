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
use alloc::string::{String, ToString};

use crate::ast::AssignOp;
use crate::builtins::Builtin;
use crate::runtime::{Runtime, RuntimeError, Value};

/// The name of a word of the dialect, with the first letter in upper case.
///
/// The words that want a parameter name themselves in the complaint that
/// they make: the name of the v2 is the word with the first letter in
/// upper case, and it is "If", "Until", "While" and "Goto".
fn the_name_of_the_word(word: &str) -> String {
    let mut characters = word.chars();
    match characters.next() {
        Some(first) => {
            let mut name = String::new();
            name.push_str(&first.to_uppercase().to_string());
            name.push_str(characters.as_str());
            name
        }
        None => String::new(),
    }
}

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
    ///
    /// The dialects of this tree are `v2` and `v1`. They are names, not
    /// versions of a product: the `#Requires` directive compares against
    /// them, and a script that needs `v2` runs in the v2 dialect and is
    /// refused in the v1 one.
    fn name(&self) -> &'static str;

    /// The version of the interpreter that the dialect stands for.
    ///
    /// The `#Requires` directive of mainline AutoHotkey names a version and
    /// not a dialect -- `#Requires AutoHotkey v2.0.19` -- and the comparison
    /// is numeric. The numbers here are the versions of the binaries whose
    /// tables this tree carries: the surface of the v2 is the one of 2.0.19,
    /// and the surface of the v1 is the one of 1.1.37.02.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `v2.0.0` and `v2` are
    /// met, `v2.0.20` and `v2.1` are not, and a requirement of another major
    /// version is not met by the v2 -- `#Requires AutoHotkey v1.1` is refused
    /// by 2.0.19 even though 2.0.19 is the later version.
    fn version(&self) -> (u16, u16, u16, u16) {
        (0, 0, 0, 0)
    }

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

    /// Whether the brace that closes a block names the text of its line.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the brace and nothing else: the `y := 2 }` of the line that comes
    /// after the `if (1) {` is an "Unexpected "}"" with no text under it. The
    /// v2 names the line of the script under it.
    fn the_closing_brace_names_the_text(&self) -> bool {
        true
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

    /// Whether the space between two values is what makes them one value.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: two values side by
    /// side are one value, and the space between them is what makes it so.
    /// The v2 refuses the pair that has no space, and it names the value that
    /// came too close.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not ask for the space: the `x := "a""b"` is the text `a"b`.
    fn the_space_separates_the_values(&self) -> bool {
        true
    }

    /// Whether the quote of a text escapes itself.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: the escape of the v2
    /// is the backtick, and the quote of the text is the character that
    /// closes it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// quote of the v1 escapes itself: the `"a""b"` is the text `a"b`.
    fn the_quote_escapes_the_quote(&self) -> bool {
        false
    }

    /// The escape of the dialect: the backtick and the character that follows.
    ///
    /// Measured on the binary of v2.0.19 on 2026-10-31: `n is a linefeed, `r
    /// a carriage return, `t a tab, `s a space, and the character that is not
    /// an escape of the dialect is the character itself.
    fn the_escape_of_the_dialect(&self, next: char) -> char {
        match next {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'b' => '\u{8}',
            'f' => '\u{c}',
            'v' => '\u{b}',
            '0' => '\0',
            's' => ' ',
            other => other,
        }
    }

    /// Whether the escape of the dialect may be in a name.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-04: the name of the v1
    /// may carry the escape, and the escape is expanded before the name is
    /// looked at: the `x := a`c` is the name "ac", and the `x := a`b` is the
    /// name "a<backspace>", which is not a legal name of a variable. The v2
    /// refuses the backtick outside of a text.
    fn the_escape_may_be_in_a_name(&self) -> bool {
        false
    }

    /// Whether the escape of the dialect protects the quote of a text.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-04: the escape of the
    /// v1 does not protect the quote of a text: the `x := "a`"b"` is the text
    /// "a" and the name `b"`, which is not a legal name of a variable.
    fn the_escape_protects_the_quote_of_a_text(&self) -> bool {
        true
    }

    /// Whether the words of the language may be the name of a variable.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// words of the v1 may be the name of a variable: the `while := 5`
    /// writes the variable "while", and the `x := while` reads it. The v2
    /// refuses the words that it reserves.
    ///
    /// The words that keep their meaning in the v1 are the operators (`and`,
    /// `or` and `not`), the values (`true` and `false`) and the `if`, that is
    /// a command.
    fn the_words_may_be_the_name_of_a_variable(&self) -> bool {
        false
    }

    /// Whether the count of the loop is a number and not an expression.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// count of the `Loop` of the v1 is a number and not an expression: the
    /// `loop n` is skipped, and so are the `loop 1+1` and the `loop 2.7`.
    /// The v2 reads the count as an expression.
    fn the_count_of_the_loop_is_a_number(&self) -> bool {
        false
    }

    /// Whether the brace that opens the body of a loop is the one that is
    /// at the end of the line.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// brace that opens the body of the loop of the v1 is the one that is
    /// at the end of the line of it. The text that follows it on the line
    /// is the text of the count: the `loop { break }` is a loop of files
    /// whose pattern is the text "{ break }", and it has no turn.
    ///
    /// The v2 reads the body of the loop on the line of it, and the brace
    /// that is not at the end of the line is refused.
    fn the_brace_of_the_body_of_the_loop_is_at_the_end_of_the_line(&self) -> bool {
        false
    }

    /// Whether the `for` of the dialect is the one of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `for` of the v1 walks the keys of an array -- the `for i in [10,20]`
    /// gives 1 and 2 to the `i` -- and it walks nothing when the source is
    /// not an array: the `for i in 5` has no turn and it does not fail. The
    /// v2 gives the values to the name and refuses a source that is not an
    /// object. The v1 also names the word that is missing: "This \"For\" is
    /// missing its \"in\".".
    fn the_for_is_the_one_of_the_v1(&self) -> bool {
        false
    }

    /// Whether the name of a variable may not carry an illegal character.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// name of the v1 may not carry a quote: the `x := a"b"` is refused with
    /// "The following variable name contains an illegal character: \"a\"b\"\"".
    /// The escape that the name carries is expanded before the name is looked
    /// at: the `x := a`b` is the name "a<backspace>", and it is refused the
    /// same way.
    ///
    /// The refusal is where the name is a variable. The name of a command of
    /// the v1 is a text, and the name of a function is not a variable: the
    /// `MsgBox, a"b"` is a command that runs, and the `a"b"(1)` is a call to
    /// a function that is not there.
    fn the_name_may_not_carry_an_illegal_character(&self) -> bool {
        false
    }

    /// Whether the single quote is not a delimiter of a text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// single quote is not a text in the v1: it begins a symbol, and the
    /// symbol that begins with it is refused where an expression wants an
    /// operand -- the `x := 'abc'` is refused with "The leftmost character
    /// above is illegal in an expression.". The v2 reads the text between
    /// the single quotes.
    fn the_single_quote_is_not_a_text(&self) -> bool {
        false
    }

    /// Whether the refusal of an illegal character names the symbol.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// v1 names the symbol that begins at the character -- the `;c` of the
    /// `x := 1;c` -- and the v2 names the rest of the expression.
    fn the_illegal_character_names_the_symbol(&self) -> bool {
        false
    }

    /// Whether the refusal of an illegal character says that the character
    /// is the leftmost one.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// v1 says "The leftmost character above is illegal in an expression."
    /// and the v2 says "Illegal character in expression.".
    fn the_illegal_character_says_the_leftmost(&self) -> bool {
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

    /// Whether the quote that is not closed is an error of the lexer.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// has no texts in the line of a command, and the quote that is not
    /// closed there is not an error: the `FileAppend, "abc, *` writes the text
    /// `"abc`. The v1 refuses the text when it reads it as an expression: the
    /// `x := "abc` is refused with "Missing close-quote".
    ///
    /// The v2 refuses the quote in the lexer, and the script does not load.
    fn the_quote_that_is_not_closed_is_an_error(&self) -> bool {
        true
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
    fn lookup_variable(&self, name: &str, rt: &mut Runtime) -> Option<Result<Value, RuntimeError>>;

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

    /// The type of the warning that the name names.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
    /// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the two dialects have
    /// their own types of the warning, and a name that is not one of them is
    /// refused at the load with "Parameter #1 invalid.".
    fn the_type_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningType>;

    /// The mode of the warning that the name names.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
    /// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the modes are the
    /// "MsgBox", the "StdOut" and the "Off", and the "OutputDebug" is a mode
    /// of the v1 and not of the v2.
    fn the_mode_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningMode>;

    /// The types of the warning that the dialect has.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05 and on the
    /// binary of AutoHotkey v1.1.37.02 on 2026-11-05: the type of the "All"
    /// is the type of every warning of the dialect.
    fn the_types_of_the_warning(&self) -> &'static [crate::warnings::WarningType];

    /// The mode of each warning when the script does not say.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the
    /// warnings of the v2 are on, and the mode of the "MsgBox" is the one of
    /// them, and the warnings of the "LocalSameAsGlobal" are off. Measured on
    /// the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the warnings of the
    /// v1 are off.
    fn the_default_warnings(&self) -> crate::warnings::TheModes;

    /// The parameters of a command that may not be left out.
    ///
    /// The numbers are the places of the parameters, counted from one.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// name of the file of the `FileSetTime` of the v1 may be left out: the
    /// file of the loop that runs is the one that is changed.
    fn the_parameters_that_may_not_be_left_out(&self, name: &str) -> &'static [u8] {
        crate::builtins::required_parameters(name)
    }

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

    /// Whether the hotkeys of the dialect are written with the two colons of
    /// the v2: `F1::`, and the action of the line or the block after it.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `F1::MsgBox "hi"`,
    /// `F1::{ ... }`, `F1::` with the block on the next line and
    /// `F1::return` are all accepted, and a name that is not a key is
    /// refused with "Invalid key name.".
    fn the_hotkeys_are_of_the_v2(&self) -> bool;

    /// Whether a hotstring of the dialect is a line of its own:
    /// `::btw::by the way`.
    fn the_hotstrings_are_of_the_v2(&self) -> bool;

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

    /// Whether the name of a property of the literal of an object may be any
    /// text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes the name of a property as the text that is written until the
    /// colon, and the quotes of a text are not part of it: the `a := {"b": 1}`
    /// has the property "b", and the `a.b` is one. The `{1.5: 1}` has the
    /// property "1.5", the `{1: 1}` has the property "1", and the `{: 1}` has
    /// the property of no name. The name of a property that is an expression
    /// is the text of it: the `{1 + 1: 1}` is accepted.
    ///
    /// The v2 refuses the name that is not a name with "Invalid property name
    /// in object literal.".
    fn the_name_of_a_property_may_be_any_text(&self) -> bool {
        false
    }

    /// Whether the name of a property that the script builds is refused.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the name of a property that the script builds with "Quote marks
    /// are required around this key." and it names the text that is written
    /// from the percent sign on: the `a := {%x%: 1}` names the "%x%: 1}".
    ///
    /// The v2 builds the name: the `a := {%x%: 1}` is the property whose name
    /// is the value of `x`.
    fn the_name_of_a_property_that_the_script_builds_is_refused(&self) -> bool {
        false
    }

    /// Whether the literal of an object that is not closed names the
    /// statement with it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says "Missing "}"" and it names no text. The v2 says "Missing "}"" and
    /// names the statement with it.
    fn the_literal_that_is_not_closed_names_the_statement(&self) -> bool {
        true
    }

    /// Whether the compound write of a thing that is not a number is
    /// dropped.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not write the property when the value that is there is not a
    /// number: the `a.b += 2` of the "abc" leaves the property as it was,
    /// and the `a[5] += 2` of an array of two leaves the place as it was
    /// too.
    ///
    /// The variable is not one of them: the `zz += 2` of the "abc" is the
    /// two, and the `zz += 2` of a variable that has no value makes the
    /// variable.
    fn the_compound_write_of_a_thing_that_is_not_a_number_is_dropped(&self) -> bool {
        false
    }

    /// Whether the object has the methods of the base object of the v2.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// object of the v1 has no method of the base object of the v2: the
    /// `a.HasOwnProp("b")` is ignored and the answer is the empty text, and
    /// the `a.HasProp("b")` is the same.
    ///
    /// The v2 has the eleven methods of the base object.
    fn the_object_has_the_methods_of_the_base_object(&self) -> bool {
        true
    }

    /// Whether the array has the methods of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// has the `Insert` and the `Remove`, and the v2 does not: the v2 has the
    /// `InsertAt` and the `RemoveAt`.
    fn the_array_has_the_methods_of_the_v1(&self) -> bool {
        false
    }

    /// Whether the array has the length and the capacity of its own.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// array of the v1 has neither: the `a.Length` is the empty text, and
    /// the `a.Capacity` is the empty text too. The v1 counts the array with
    /// the `a.Length()` and the `a.Count()`.
    ///
    /// The v2 has both.
    fn the_array_has_the_length_and_the_capacity(&self) -> bool {
        true
    }

    /// Whether the item of an object is the property of that name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes the item of an object for the property of that name: the
    /// `a["b"]` reads the property "b", the `a["B"]` reads it too, and the
    /// `a["b"] := 5` writes it. The name of the property is the name of the
    /// key, and the key may be a number.
    ///
    /// The v2 asks the `__Item` of the object, and a plain object of it has
    /// no such thing.
    fn the_item_of_an_object_is_its_property(&self) -> bool {
        false
    }

    /// Whether the item of an array that is not there is the empty text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse the item of an array that is not there: the `a[5]` of
    /// an array of two is the empty text, and the `a[0]` is the empty text
    /// too.
    ///
    /// The v2 refuses with "Invalid index.".
    fn the_item_of_an_array_that_is_not_there_is_the_empty_text(&self) -> bool {
        false
    }

    /// Whether the array grows with the write of an item that is not there.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// makes the array grow: the `a[5] := 7` of an array of two is the
    /// seven, and the places that are not there are holes.
    ///
    /// The v2 refuses with "Invalid index.".
    fn the_array_grows_with_the_write_of_an_item(&self) -> bool {
        false
    }

    /// Whether the item of an array with a key that is not a number is the
    /// property of that name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes the item of an array with a key that is not a number for the
    /// property of that name: the `a["b"]` reads the property "b", and the
    /// `a["b"] := 9` writes it. The places of the array are the ones of a
    /// number.
    ///
    /// The v2 refuses with "Invalid index.".
    fn the_item_of_an_array_with_a_key_that_is_not_a_number_is_its_property(&self) -> bool {
        false
    }

    /// Whether the name that the script builds and that is not there is
    /// answered.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse the name that is not there: it creates the variable of
    /// that name and reads it, and the answer is the empty text. The `a := 1`
    /// and the `x := %a%` read the variable "1", and the `x` is the empty
    /// text; the `1 := 99` and the `x := %a%` read the variable "1", and the
    /// `x` is ninety-nine.
    ///
    /// The v2 refuses with "Variable not found." and the name in the `Extra`.
    fn the_name_that_the_script_builds_and_that_is_not_there_is_answered(&self) -> bool {
        false
    }

    /// Whether the write of a name that the script builds inside a function
    /// is refused.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the write of a name that the script builds inside a function,
    /// and it refuses it whatever the variable of that name is: the `%a% := 7`
    /// inside a function opens a dialogue of the v1 and says "This dynamic
    /// variable is blank. If this variable was not intended to be dynamic,
    /// remove the % symbols from it.".
    ///
    /// The v2 refuses it with "This dynamic assignment requires a \"global\"
    /// declaration." and only when the name is not a local of the function
    /// and does not live in the script.
    fn the_dynamic_write_inside_a_function_is_refused(&self) -> bool {
        false
    }

    /// The message of the refusal of the write of a name that the script
    /// builds inside a function.
    fn the_message_of_the_dynamic_write_inside_a_function(&self) -> &'static str {
        "This dynamic assignment requires a \"global\" declaration."
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

    /// Whether the parameter keeps the text of it when it is not an
    /// expression.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// amount of the `EnvAdd` and of the `EnvSub` of the v1 is the text of it
    /// when it is not an expression: the "5abc" is not a number, and the
    /// `EnvAdd` leaves the variable empty.
    fn the_parameter_keeps_its_text(&self, _name: &str, _index: usize) -> bool {
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

    /// Whether the body of the getter and of the setter may be an expression.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the body of the getter
    /// is an expression after the arrow, and a block of statements between
    /// braces.
    fn the_accessor_takes_an_expression(&self) -> bool {
        true
    }

    /// Whether the member of the property that has no body is a call.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the v2 refuses it with
    /// "Not a valid property getter/setter.\n     Specifically: get".
    fn the_member_of_an_accessor_without_a_body_is_a_call(&self) -> bool {
        false
    }

    /// Whether the member that is refused is named whole.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-05: the member that is not
    /// a valid definition is named whole, and the newlines of it are spaces:
    /// "p { get { return 1 } }".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// names the line of it: "p { get {".
    fn the_member_that_is_refused_is_named_whole(&self) -> bool {
        true
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

    /// Whether the base of a class may lead back to the class itself.
    ///
    /// Measured on the binary of v2.0.31 on 2026-11-09: a chain of bases that
    /// returns to where it began is refused at the load, with "Invalid base
    /// class." and the name of the last base of the chain in the reason. The
    /// changelog of v2.0.27 records the fix: "Fixed class declarations to not
    /// permit a class to subclass itself." Before that the binary of v2.0.19
    /// failed on the same script with a dialog and no report.
    ///
    /// Measured on the binary of v1.1.37.02 on 2026-11-09: the v1 runs the
    /// same script, and the class is its own base.
    fn the_class_may_extend_itself(&self) -> bool {
        false
    }

    /// The reason a body that has no head is refused with.
    ///
    /// `kind` is `Else`, `Catch` or `Finally`, which is the word the line
    /// began with. Measured on the binary of v2.0.31 on 2026-11-09: the v2
    /// names the word it found -- `Unexpected "Catch"` -- and that is the
    /// whole of the reason.
    fn the_refusal_of_a_body_without_a_head(&self, kind: &str) -> String {
        format!("Unexpected \"{kind}\"")
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

    /// The name of a member of a class of the script.
    ///
    /// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the method
    /// of the prototype is "A.Prototype.m", and the static method is "A.m".
    /// The getter of a property is "A.Prototype.p.get".
    fn the_name_of_a_member_of_a_class(
        &self,
        class: &str,
        member: &str,
        is_static: bool,
    ) -> String {
        if is_static {
            format!("{class}.{member}")
        } else {
            format!("{class}.Prototype.{member}")
        }
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
    fn the_refusal_of_the_name_of_a_variable(&self, _place: usize, _text: &str) -> Option<String> {
        None
    }

    /// The name that the dialect writes when the word that wants a parameter
    /// has none.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `if` of the v1 has no name of its own, and the complaint of the v1 is
    /// '"" requires at least 1 parameter. Specifically: if'. The other words
    /// name themselves: the `until` alone is refused with '"Until" requires
    /// at least 1 parameter. Specifically: until'.
    fn the_name_of_the_word_that_wants_a_parameter(&self, word: &str) -> String {
        the_name_of_the_word(word)
    }

    /// Whether the brace that follows a value opens the literal of an object.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the brace at once, and it does not look for the brace that
    /// closes the literal: the `x := 1 {` is refused with "Unexpected "{"".
    /// The v2 looks for the closing brace first, and the literal that is not
    /// closed is a "Missing "}"".
    fn the_brace_after_a_value_opens_a_literal(&self) -> bool {
        true
    }

    /// Whether the parenthesis that closes the list of the arguments of a
    /// call is needed.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not need it when the name of the call is not a function: the
    /// `x := abc(c` is refused at the load with "Call to nonexistent
    /// function.\n     Specifically: abc(c", and the parse of it does not fail.
    /// The call of a name that is a function needs it, and the `x := f(1` is
    /// refused with "Missing ")"".
    ///
    /// The v2 needs it always.
    fn the_closing_parenthesis_of_a_call_is_needed(&self) -> bool {
        true
    }

    /// Whether the tail of a call to a name that is not a function is text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// reads the rest of the line as text when the name of the call is not a
    /// function: the `MyFunc() { return 2 }` is refused at the load with
    /// "Call to nonexistent function.\n     Specifically: MyFunc() { return 2 }",
    /// and the brace of it is not refused. When the name is a function, the
    /// rest of the line is read: the `f(1) { return 2 }` is refused with
    /// "Unexpected "{"", and the name of the function may be one that the
    /// script defines further down.
    ///
    /// The v2 reads the rest of the line always.
    fn the_tail_of_a_call_to_a_name_that_is_not_a_function_is_text(&self) -> bool {
        false
    }

    /// Whether the tail of a list of a statement is text.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// reads the tail of a statement as a list of items, and an item that is
    /// not an expression is a text: it is the parameter of a command, and the
    /// v1 does not run it and does not refuse it. The `f(1) FileAppend,
    /// [tail], *` runs nothing and the load does not fail, and the items that
    /// come after the text do not run either.
    ///
    /// The v2 refuses the item that is not there with "Syntax error.\n
    /// Specifically: *".
    fn the_tail_of_a_list_is_text(&self) -> bool {
        false
    }

    /// The message of the brace that opens the body of a construct when the
    /// content of the block is on the line of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// names the brace: the `if 1 { return 2 }` is refused with
    /// "Unexpected "{"". The v2 names the construct: the `if (1) { return 2 }`
    /// is refused with "Unexpected "If"".
    fn the_message_of_the_brace_that_opens_the_body(&self) -> Option<&'static str> {
        None
    }

    /// Whether the operand that is not there is not refused.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse the expression whose operand is not there: the `x := 1 +`
    /// and the `x := not` are accepted, and the value of the expression is the
    /// empty text. The operand that is not there poisons the operation that
    /// holds it, and the `&&` and the `||` are the exception: when the left
    /// side decides, the answer is one or zero.
    ///
    /// The v2 refuses it with "Missing operand.".
    fn the_operand_that_is_not_there_is_not_refused(&self) -> bool {
        false
    }

    /// Whether the operand that is not there poisons the value that holds it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// poisons the value that holds it: the `StrLen(1 +)` is the empty text,
    /// and not the zero that the `StrLen` of the empty text answers; the
    /// `o.Method(1 +)` is the empty text, and so is the `f(1 +).y` and the
    /// `f(1 +)[1]`. The literal of an array and the literal of an object that
    /// hold it are the empty text too, and they are not objects: the
    /// `IsObject(a)` of the `a := [1 +]` is zero.
    ///
    /// The v2 refuses the expression at the load.
    fn the_operand_that_is_not_there_poisons_the_value_that_holds_it(&self) -> bool {
        false
    }

    /// Whether the line that ends with an operator continues on the next one.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// merges the line that BEGINS with an operator with the one before it,
    /// and not the other way round: the `x := 1 +` and the `2` that comes
    /// after it are two lines, and the `2` is refused with "This line does
    /// not contain a recognized action.". The bracket and the comma do not
    /// continue the line either: the `x := 1,` and the `2` that comes after
    /// it are two lines.
    ///
    /// The v2 continues the line that ends with an operator, and the bracket
    /// and the comma too.
    fn a_line_that_ends_with_an_operator_continues(&self) -> bool {
        true
    }

    /// Whether the line that begins with the word holds no action.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says that the line that begins with the `not` holds no action, and it
    /// names the whole line: the `not` names the "not", and the `not 1` names
    /// the "not 1". The same is true of the `and` and of the `or`.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `true` and the `false` are the same: the line that begins with them
    /// holds no action, and it names the whole line: the `true` names the
    /// "true", and the `true + 1` names the "true + 1".
    ///
    /// The v2 names the call that the word wanted to make: "not()".
    fn the_line_that_begins_with_the_word_holds_no_action(&self, word: &str) -> bool {
        let _ = word;
        false
    }

    /// The message of a declaration that the dialect refuses.
    ///
    /// The text is the declaration as the script wrote it: the "global", the
    /// "local", the "local x" and the "local x := 1".
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says that the line holds no action, and it names the declaration.
    ///
    /// The v2 says "Unexpected declaration." and names the same text.
    fn the_message_of_the_refused_declaration(&self, text: &str) -> String {
        format!("Unexpected declaration.\n     Specifically: {text}")
    }

    /// Whether the `local` alone is a declaration of no name.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// accepts the `local` alone inside a function, and it refuses it at the
    /// top level of the script, where the line holds no action.
    ///
    /// The v2 refuses the `local` alone everywhere.
    fn the_bare_local_is_a_declaration_of_no_name(&self) -> bool {
        false
    }

    /// Whether the `global` alone is the mode of the script.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the `global` alone at the top level of the script, and the
    /// line holds no action: the mode of a function is the `global` alone of
    /// its first line, and the script is not a function.
    ///
    /// The v2 accepts it at the top level of the script.
    fn the_bare_global_is_the_mode_of_the_script(&self) -> bool {
        true
    }

    /// Whether the declaration that has no name is the first statement of
    /// the body of the function.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// accepts the `local` alone and the `static` alone when they are the
    /// first statement of the body of the function, and it refuses them
    /// anywhere else: the line holds no action.
    ///
    /// The v2 accepts the `static` alone anywhere inside a function, and it
    /// refuses the `local` alone everywhere.
    fn the_declaration_with_no_name_is_the_first_statement(&self) -> bool {
        false
    }

    /// Whether the write of a property of a value that is not an object is
    /// dropped.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not refuse it: the `a.b := 5` is accepted when the `a` is not an
    /// object, and the write is dropped.
    ///
    /// The v2 refuses it with "This value of type "String" has no property
    /// named "b"."
    fn the_write_of_a_property_of_a_value_that_is_not_an_object_is_dropped(&self) -> bool {
        false
    }

    /// Whether the failure of the condition of a construct is not reported.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// does not report the failure of the condition of an `if` or of a
    /// `while`: the `if 1/0`, the `if x.y` and the `if x.y := 5` are
    /// accepted, the condition is false, and the body of the construct does
    /// not run.
    ///
    /// The v2 reports the failure.
    fn the_failure_of_a_condition_is_not_reported(&self) -> bool {
        false
    }

    /// The complaint of the object literal that has no property.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// says "Missing \"key:\" in object literal." and the v2 says "Missing
    /// \"propertyname:\" in object literal.". The complaint of the v1 is the
    /// same when the name of the property is not there: the `{b: 1 c: 2}` is
    /// refused with "Missing \"key:\" in object literal.".
    fn the_refusal_of_the_object_literal(&self) -> &'static str {
        "Missing \"propertyname:\" in object literal."
    }

    /// Whether the blank parameter at the end of a list is refused.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// refuses the `{b: 1,}` with "Blank parameter", and it names the whole
    /// literal. The v2 accepts the comma at the end of the literal: the
    /// `{b: 1,}` is an object of one property.
    fn the_blank_at_the_end_of_a_list_is_refused(&self) -> bool {
        false
    }

    /// Whether the comma at the end of a list is a parameter.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// counts the comma at the end of the list of the arguments of a call: the
    /// `f(1,)` is refused with "Too many parameters passed to function." when
    /// f takes one parameter, and the `f(,)` is refused with "Blank
    /// parameter". The v2 accepts it: the comma at the end of the list adds
    /// nothing.
    fn the_comma_at_the_end_of_a_list_is_a_parameter(&self) -> bool {
        false
    }

    /// Whether the parameter of the `return` is read before the refusal of
    /// the return.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// reads the parameter before it refuses the return: the `return a"b` is
    /// refused with "The following variable name contains an illegal
    /// character:\n\"a\"b\"", and the `return hello` is refused with
    /// "Return's parameter should be blank except inside a function.". The
    /// v1 has the rule of the blank parameter, and the name that is not
    /// legal is the first refusal.
    ///
    /// The v2 refuses the return before it reads the parameter.
    fn the_parameter_of_the_return_is_read_first(&self) -> bool {
        false
    }

    /// Whether a loop of the dialect that has no body ends the script.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// takes the body of a loop from the statement that comes next, and the
    /// script that has none ends at the loop: the `loop` that is the last
    /// line of the script ends the script at once, and the `while 1` that is
    /// the last line of it does the same. The `loop 3` that is the last line
    /// of the script goes on three times, and the script ends after them.
    ///
    /// The v2 refuses the loop that has no body, and the loop of the v2
    /// without a count goes on until a break.
    fn a_loop_without_a_body_ends_the_script(&self) -> bool {
        false
    }

    /// Whether the definition of a function of the dialect needs a body.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// definition of the v1 needs one: the `f(a) {` at the end of the file is
    /// a call, and it is refused with "Call to nonexistent function.\n
    /// Specifically: f(a)".
    ///
    /// The v2 does not ask.
    fn the_definition_needs_a_body(&self) -> bool {
        false
    }

    /// Whether the line that begins with a name and a list of parameters is a
    /// definition of a function.
    ///
    /// The v2 decides with the token that comes after the parenthesis: the
    /// brace and the fat arrow make a definition of it.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// decides with the rest of the line: the definition is the line `f(a) {`
    /// and the line `f(a)`, and when there is anything else after the
    /// parenthesis that closes the list, the line is a call. The `f(a) { return
    /// a }` is a call of the name "f", and it is refused with "Call to
    /// nonexistent function.".
    ///
    /// `rest` is the text of the line that comes after the parenthesis that
    /// closes the list, and the answer of `None` is that the dialect does not
    /// decide: the token that comes after the parenthesis does.
    fn a_line_is_a_definition(&self, _rest: &str) -> Option<bool> {
        None
    }

    /// The refusal of the `until` that has no loop.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    /// `until` of the v1 that is not the condition of a loop is refused with
    /// "UNTIL with no matching LOOP", and it names nothing else. The v2
    /// crashes when the `until` has a condition, and the tree refuses it
    /// with 'Unexpected "Until"'.
    ///
    /// `None` means that the dialect has no such refusal.
    fn the_refusal_of_the_until_that_has_no_loop(&self) -> Option<String> {
        None
    }

    /// The refusal of the percent sign of a parameter of a command.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
    /// checks the percent sign of every parameter of its commands when the
    /// script loads: the `FileAppend, a%b, *` is refused with "This parameter
    /// contains a variable name missing its ending percent sign.".
    ///
    /// The v2 has no command that takes a text, and the parser keeps its own
    /// complaint.
    fn the_refusal_of_the_percent_sign_of_a_parameter(
        &self,
        _place: usize,
        _text: &str,
    ) -> Option<String> {
        None
    }

    /// Whether the names of the tail of a list are checked.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// reads the tail of a list as symbols, and the name of each one of them
    /// is checked: the `x := 1, a"b"` is refused with the name of it, and so
    /// is the `x := 1, "a"b"` with the name "b"".
    ///
    /// The v2 does not have this check: the name of the v2 may not carry a
    /// quote at all.
    fn the_names_of_a_tail_are_checked(&self) -> bool {
        false
    }

    /// The refusal of the percent sign that opens a name in an expression.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
    /// opens the name of a variable with the percent sign, and the name that
    /// is not closed is refused with "This parameter contains a variable name
    /// missing its ending percent sign." The name that is empty is refused
    /// with "Empty variable reference (%%).". The text that is named is the
    /// run of the name: the characters of the name and the percent signs that
    /// are glued to them.
    ///
    /// The `run` is the run of the name, and the `empty` says that the name
    /// that the percent sign opens is empty.
    ///
    /// The v2 does not have this refusal: the percent sign opens the double
    /// deref, and the parser keeps its own complaint.
    fn the_refusal_of_the_percent_sign_of_an_expression(
        &self,
        _run: &str,
        _empty: bool,
    ) -> Option<String> {
        None
    }

    /// The refusal of the point of the operand.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// name that the script builds may not carry a point: the `x := a.b%c` is
    /// refused with "Ambiguous or invalid use of \".\"" and the text that it
    /// names is the operand from its first point to its end.
    ///
    /// The v2 does not have this refusal: the point is the point of a member,
    /// and the name of a member may be built by the script.
    fn the_refusal_of_the_point_of_an_operand(&self, _text: &str) -> Option<String> {
        None
    }

    /// The refusal of the text that is the name of a member.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
    /// name of a member of the v1 may not be a text: the `x := a."b"` is
    /// refused with "This character is not allowed here." and the rest of the
    /// line, from the quote.
    ///
    /// The v2 refuses the text with "Illegal character in expression." and the
    /// text as it was written, and the parser keeps that complaint.
    fn the_refusal_of_the_text_of_a_member(&self, _text: &str) -> Option<String> {
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

    /// Whether the lexer knows the null assignment of the v2.1, `??=`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `x ??= 5` is refused
    /// at the load, and the two signs are the `??` and the `=`; the v2.1 has
    /// the operator of its own, and the lexer is where the difference is: a
    /// script that the v2.0.19 runs may not see the token at all.
    fn the_lexer_knows_the_null_assignment(&self) -> bool {
        false
    }

    /// Whether the absent value may be the class of the `is`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: `x is unset` is a
    /// "Syntax error." at the load, because the absent value has no class. The
    /// v2.1 answers the question: the `is` of it is one when the left side has
    /// no value, and the left side is not read as a value of its own -- the
    /// `x is unset` of a name that has no value is one and not a failure.
    fn the_absent_value_may_be_the_class_of_the_is(&self) -> bool {
        false
    }

    /// Whether an expression may be the left side of the `??`.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-10: only a variable may be
    /// on the left of the `??`, and `f() ?? 1` is a "Syntax error." at the
    /// load. The v2.1 takes any expression there: the answer of the left side
    /// that is the absent value is the right side.
    fn an_expression_may_be_the_left_of_the_maybe(&self) -> bool {
        false
    }

    /// Whether a `while` may carry the `Until` clause of the v2.1.
    ///
    /// Measured on the test suite of KeySharp (the corpus script
    /// `flow-until.ahk`) and on the notes of v2.1-alpha: the `until` of a
    /// `while` comes after the body, the body runs before the condition is
    /// read on every turn, and the loop ends when it is true. The binary of
    /// v2.0.19 refuses the word with "Unexpected \"Until\"".
    fn a_while_may_carry_an_until(&self) -> bool {
        false
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
///
/// The struct carries the mechanisms of v2.1 as well, because the tree speaks
/// the v2.1 beside the v2.0.19: the `--dialect=v2.1` asks for them, and the
/// `extended` says which of the two is spoken. The names and the tables are
/// the ones of the v2 either way -- the v2.1 does not rename what the v2 has --
/// and only the mechanisms that the v2.1 adds are gated.
pub struct V2 {
    /// Whether the mechanisms of v2.1 are carried.
    pub extended: bool,
}

/// The v2.0.19 of the tree, which is the dialect of every script that does not
/// ask for another one.
pub static THE_V2: V2 = V2 { extended: false };

/// The v2.1 mechanisms beside the v2.0.19, behind the `--dialect=v2.1`.
///
/// Measured on the test suite of KeySharp and on the notes of v2.1-alpha: the
/// `Until` clause of a `while` is a mechanism that the v2.1 adds and that the
/// binary of v2.0.19 refuses.
pub static THE_V2_1: V2 = V2 { extended: true };

/// The legacy dialect: AutoHotkey v1.1.37.02.
pub struct V1;

/// The dialect of the tree, which is the one of every script that does not
/// ask for another one.
pub fn v2() -> &'static dyn Dialect {
    &THE_V2
}

/// The v2.1 mechanisms beside the v2.0.19, behind the `--dialect=v2.1`.
pub fn v2_1() -> &'static dyn Dialect {
    &THE_V2_1
}

/// The legacy dialect, behind the gate of the `--dialect=v1`.
pub fn v1() -> &'static dyn Dialect {
    &V1
}

/// The reason a name that the tree does not carry yet gives.
///
/// The `ObjBindMethod` was the last name of the surface of either dialect
/// that the tree did not carry, and it is carried now: no name of the
/// surface refuses with this reason any more.
pub const NOT_CARRIED_YET: &str = "this runtime does not carry that name yet.";

/// The reason a name that is about Windows gives on a machine that is not.
pub const NOT_A_WINDOWS: &str = "this name is about Windows, and this machine is not.";

impl Dialect for V2 {
    fn the_hotkeys_are_of_the_v2(&self) -> bool {
        true
    }

    fn the_hotstrings_are_of_the_v2(&self) -> bool {
        true
    }

    fn a_while_may_carry_an_until(&self) -> bool {
        self.extended
    }

    fn the_lexer_knows_the_null_assignment(&self) -> bool {
        self.extended
    }

    fn the_absent_value_may_be_the_class_of_the_is(&self) -> bool {
        self.extended
    }

    fn an_expression_may_be_the_left_of_the_maybe(&self) -> bool {
        self.extended
    }

    fn name(&self) -> &'static str {
        "v2"
    }

    fn version(&self) -> (u16, u16, u16, u16) {
        // The mechanisms of the v2.1 are of the v2.1, and the version that the
        // script reads is the one of the dialect that runs: `A_AhkVersion` is
        // "2.0.19" in the v2.0.19 and "2.1.0" behind the gate of the v2.1.
        if self.extended {
            (2, 1, 0, 0)
        } else {
            (2, 0, 19, 0)
        }
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

    fn the_refusal_of_the_write_of_a_built_in_variable(
        &self,
        name: &str,
        op: AssignOp,
    ) -> Option<String> {
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

    /// The argument of `IsSet` that asks for the variable itself.
    ///
    /// Measured on the binary of v2.0.19 on 2026-11-09: `IsSet(Var)` asks
    /// whether the variable has a value, and the argument is a variable and
    /// not an expression -- `IsSet(1)` is refused with "IsSet requires a
    /// variable.". The value of the name is not read: a variable that has
    /// no value is the answer zero, and not a failure.
    fn the_place_of_the_argument_that_wants_the_variable(&self, name: &str) -> Option<usize> {
        if name.eq_ignore_ascii_case("IsSet") {
            return Some(0);
        }
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

    fn lookup_variable(&self, name: &str, rt: &mut Runtime) -> Option<Result<Value, RuntimeError>> {
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

    fn the_type_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningType> {
        match name.to_lowercase().as_str() {
            "all" => Some(crate::warnings::WarningType::All),
            "varunset" => Some(crate::warnings::WarningType::VarUnset),
            "localsameasglobal" => Some(crate::warnings::WarningType::LocalSameAsGlobal),
            "unreachable" => Some(crate::warnings::WarningType::Unreachable),
            _ => None,
        }
    }

    fn the_mode_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningMode> {
        match name.to_lowercase().as_str() {
            "msgbox" => Some(crate::warnings::WarningMode::MsgBox),
            "stdout" => Some(crate::warnings::WarningMode::StdOut),
            "off" => Some(crate::warnings::WarningMode::Off),
            _ => None,
        }
    }

    fn the_types_of_the_warning(&self) -> &'static [crate::warnings::WarningType] {
        &[
            crate::warnings::WarningType::VarUnset,
            crate::warnings::WarningType::LocalSameAsGlobal,
            crate::warnings::WarningType::Unreachable,
        ]
    }

    fn the_default_warnings(&self) -> crate::warnings::TheModes {
        crate::warnings::TheModes::the_defaults_of_the_v2()
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
    fn the_hotkeys_are_of_the_v2(&self) -> bool {
        false
    }

    fn the_hotstrings_are_of_the_v2(&self) -> bool {
        false
    }
    fn name(&self) -> &'static str {
        "v1"
    }

    fn version(&self) -> (u16, u16, u16, u16) {
        (1, 1, 37, 2)
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

    fn the_closing_brace_names_the_text(&self) -> bool {
        // The v1 names the brace and nothing else: the `y := 2 }` is an
        // "Unexpected "}"" with no text under it.
        false
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

    fn the_space_separates_the_values(&self) -> bool {
        // The v1 does not ask for the space: the `x := "a""b"` is the text
        // `a"b`.
        false
    }

    fn the_quote_escapes_the_quote(&self) -> bool {
        // The quote of the v1 escapes itself: the `"a""b"` is the text `a"b`.
        true
    }

    fn the_escape_of_the_dialect(&self, next: char) -> char {
        // Measured on the binary of v1.1.37.02 on 2026-11-04: the `a of the
        // v1 is a bell, and the `s is the letter itself: the v1 has no `s.
        match next {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'b' => '\u{8}',
            'f' => '\u{c}',
            'v' => '\u{b}',
            '0' => '\0',
            'a' => '\u{7}',
            other => other,
        }
    }

    fn the_escape_may_be_in_a_name(&self) -> bool {
        // The name of the v1 may carry the escape: the `x := a`c` is the name
        // "ac".
        true
    }

    fn the_escape_protects_the_quote_of_a_text(&self) -> bool {
        // The escape of the v1 does not protect the quote of a text: the
        // `x := "a`"b"` is the text "a" and the name `b"`.
        false
    }

    fn the_words_may_be_the_name_of_a_variable(&self) -> bool {
        // The words of the v1 may be the name of a variable: the `while := 5`
        // writes the variable "while", and the `x := while` reads it.
        true
    }

    fn the_count_of_the_loop_is_a_number(&self) -> bool {
        // The count of the `Loop` of the v1 is a number and not an
        // expression: the `loop n` is skipped.
        true
    }

    fn the_brace_of_the_body_of_the_loop_is_at_the_end_of_the_line(&self) -> bool {
        // The brace that opens the body of the loop of the v1 is the one
        // that is at the end of the line.
        true
    }

    fn the_for_is_the_one_of_the_v1(&self) -> bool {
        // The `for` of the v1 walks the keys of an array and nothing else,
        // and it names the word that is missing.
        true
    }

    fn the_name_may_not_carry_an_illegal_character(&self) -> bool {
        // The name of the v1 may not carry a quote, and the escape that it
        // carries is expanded before the name is looked at. The refusal is
        // where the name is a variable, and not where it is the name of a
        // command.
        true
    }

    fn the_single_quote_is_not_a_text(&self) -> bool {
        // The single quote is not a text in the v1: it begins a symbol, and
        // the symbol is refused where an expression wants an operand.
        true
    }

    fn the_illegal_character_names_the_symbol(&self) -> bool {
        // The v1 names the symbol that begins at the character: the `;c` of
        // the `x := 1;c`.
        true
    }

    fn the_illegal_character_says_the_leftmost(&self) -> bool {
        // The v1 says "The leftmost character above is illegal in an
        // expression.".
        true
    }

    fn the_text_that_is_not_closed_is_named_alone(&self) -> bool {
        // The v1 says "Missing close-quote" and names the text that was read.
        true
    }

    fn the_quote_that_is_not_closed_is_an_error(&self) -> bool {
        // The v1 has no texts in the line of a command, and the quote that is
        // not closed there is not an error: the parser refuses the text when
        // it reads it as an expression.
        false
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

    fn lookup_variable(&self, name: &str, rt: &mut Runtime) -> Option<Result<Value, RuntimeError>> {
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

    fn the_type_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningType> {
        match name.to_lowercase().as_str() {
            "all" => Some(crate::warnings::WarningType::All),
            "useunsetlocal" => Some(crate::warnings::WarningType::UseUnsetLocal),
            "useunsetglobal" => Some(crate::warnings::WarningType::UseUnsetGlobal),
            "useenv" => Some(crate::warnings::WarningType::UseEnv),
            "localsameasglobal" => Some(crate::warnings::WarningType::LocalSameAsGlobal),
            "classoverwrite" => Some(crate::warnings::WarningType::ClassOverwrite),
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05:
            // the v1 carries the type "Unreachable" as well, and the text of
            // the warning is the one of the v2.
            "unreachable" => Some(crate::warnings::WarningType::Unreachable),
            _ => None,
        }
    }

    fn the_mode_of_the_warning(&self, name: &str) -> Option<crate::warnings::WarningMode> {
        match name.to_lowercase().as_str() {
            "msgbox" => Some(crate::warnings::WarningMode::MsgBox),
            "stdout" => Some(crate::warnings::WarningMode::StdOut),
            "outputdebug" => Some(crate::warnings::WarningMode::OutputDebug),
            "off" => Some(crate::warnings::WarningMode::Off),
            _ => None,
        }
    }

    fn the_types_of_the_warning(&self) -> &'static [crate::warnings::WarningType] {
        &[
            crate::warnings::WarningType::UseUnsetLocal,
            crate::warnings::WarningType::UseUnsetGlobal,
            crate::warnings::WarningType::UseEnv,
            crate::warnings::WarningType::LocalSameAsGlobal,
            crate::warnings::WarningType::ClassOverwrite,
            crate::warnings::WarningType::Unreachable,
        ]
    }

    fn the_default_warnings(&self) -> crate::warnings::TheModes {
        crate::warnings::TheModes::the_defaults_of_the_v1()
    }

    fn the_parameters_that_may_not_be_left_out(&self, name: &str) -> &'static [u8] {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // name of the file of the `FileSetTime` of the v1 may be left out: the
        // file of the loop that runs is the one that is changed, and there is
        // no such file outside of a loop.
        if name.eq_ignore_ascii_case("FileSetTime") {
            return &[];
        }
        crate::builtins::required_parameters(name)
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
        if max == 255 { 10000 } else { max as u32 }
    }

    fn a_member_that_is_not_there_is_answered(&self) -> bool {
        // The v1 answers a member that a value does not have.
        true
    }

    fn the_name_that_the_script_builds_and_that_is_not_there_is_answered(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // creates the variable of the name that the script builds and reads it.
        true
    }

    fn the_name_of_a_property_may_be_any_text(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // takes the name of a property as the text that is written until the
        // colon.
        true
    }

    fn the_name_of_a_property_that_the_script_builds_is_refused(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // wants the name of a property between quotes.
        true
    }

    fn the_literal_that_is_not_closed_names_the_statement(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // names no text.
        false
    }

    fn the_item_of_an_object_is_its_property(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // takes the item of an object for the property of that name.
        true
    }

    fn the_object_has_the_methods_of_the_base_object(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // object of the v1 has no method of the base object of the v2.
        false
    }

    fn the_array_has_the_length_and_the_capacity(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // array of the v1 has neither the length nor the capacity.
        false
    }

    fn the_array_has_the_methods_of_the_v1(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // has the `Insert` and the `Remove`, and the v2 does not.
        true
    }

    fn the_compound_write_of_a_thing_that_is_not_a_number_is_dropped(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // does not write the property when the value that is there is not a
        // number.
        true
    }

    fn the_item_of_an_array_that_is_not_there_is_the_empty_text(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // does not refuse the item of an array that is not there.
        true
    }

    fn the_array_grows_with_the_write_of_an_item(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // makes the array grow with the write of an item that is not there.
        true
    }

    fn the_item_of_an_array_with_a_key_that_is_not_a_number_is_its_property(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // takes the item of an array with a key that is not a number for the
        // property of that name.
        true
    }

    fn the_dynamic_write_inside_a_function_is_refused(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // refuses the write of a name that the script builds inside a
        // function.
        true
    }

    fn the_message_of_the_dynamic_write_inside_a_function(&self) -> &'static str {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 says that the dynamic variable is blank.
        "This dynamic variable is blank. If this variable was not intended to be dynamic, remove the % symbols from it."
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

    fn the_refusal_of_the_write_of_a_built_in_variable(
        &self,
        name: &str,
        op: AssignOp,
    ) -> Option<String> {
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
        Some(format!(
            "Not allowed as an output variable.\n     Specifically: {name}"
        ))
    }

    fn the_refusal_of_the_increment_of_a_built_in_variable(&self, name: &str) -> Option<String> {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: the increment of
        // a built-in variable of the v1 is refused at the load, and the reason
        // is the one of the write.
        if !crate::v1::is_read_only(name) {
            return None;
        }
        Some(format!(
            "Not allowed as an output variable.\n     Specifically: {name}"
        ))
    }

    fn the_refusal_of_an_invalid_parameter(
        &self,
        name: &str,
        parameters: &[String],
    ) -> Option<String> {
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

    fn the_refusal_of_the_name_of_a_variable(&self, _place: usize, text: &str) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the v1 checks
        // the name of a variable of the script when the script loads, and
        // the complaint of the v1 is its own.
        crate::v1::the_refusal_of_the_name_of_a_variable(text)
    }

    fn the_refusal_of_the_percent_sign_of_a_parameter(
        &self,
        _place: usize,
        text: &str,
    ) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
        // checks the percent sign of every parameter of its commands when the
        // script loads.
        crate::v1::the_refusal_of_the_percent_sign_of_a_parameter(text)
    }

    fn the_names_of_a_tail_are_checked(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
        // reads the tail of a list as symbols, and the name of each one of
        // them is checked.
        true
    }

    fn the_refusal_of_the_percent_sign_of_an_expression(
        &self,
        run: &str,
        empty: bool,
    ) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // v1 opens a name with the percent sign, and the name that is not
        // closed and the name that is empty have a complaint of their own.
        crate::v1::the_refusal_of_the_percent_sign_of_an_expression(run, empty)
    }

    fn the_refusal_of_the_point_of_an_operand(&self, text: &str) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // name that the script builds may not carry a point.
        crate::v1::the_refusal_of_the_point_of_an_operand(text)
    }

    fn the_refusal_of_the_text_of_a_member(&self, text: &str) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // name of a member of the v1 may not be a text.
        crate::v1::the_refusal_of_the_text_of_a_member(text)
    }

    fn the_name_of_the_word_that_wants_a_parameter(&self, word: &str) -> String {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `if` of the v1 is a command that has no name of its own, and the
        // complaint of it is '"" requires at least 1 parameter. Specifically:
        // if'. The other words name themselves.
        if word == "if" {
            return String::new();
        }
        the_name_of_the_word(word)
    }

    fn the_refusal_of_the_until_that_has_no_loop(&self) -> Option<String> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // `until` of the v1 that no loop takes is refused with "UNTIL with no
        // matching LOOP", and it names nothing else.
        Some("UNTIL with no matching LOOP".to_string())
    }

    fn a_line_is_a_definition(&self, rest: &str) -> Option<bool> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // definition of the v1 is the line `f(a) {` and the line `f(a)` with
        // the brace on the line that comes next, and when there is anything
        // else after the parenthesis that closes the list, the line is a
        // call. The `f(a) { return a }` is a call of the name "f", and it is
        // refused with "Call to nonexistent function.".
        let rest = rest.trim();
        if rest.is_empty() {
            // The brace may be on the line that comes next, and the token
            // that comes after the parenthesis decides.
            return None;
        }
        Some(rest == "{")
    }

    fn the_definition_needs_a_body(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // definition of the v1 needs a body, and the `f(a) {` at the end of
        // the file is a call.
        true
    }

    fn the_failure_of_a_condition_is_not_reported(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not report the failure of the condition of an `if` or of a
        // `while`, and the condition is false.
        true
    }

    fn the_refusal_of_the_object_literal(&self) -> &'static str {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 names the key of the literal, and not the property of it.
        "Missing \"key:\" in object literal."
    }

    fn the_blank_at_the_end_of_a_list_is_refused(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the blank at the end of the list with "Blank parameter".
        true
    }

    fn the_comma_at_the_end_of_a_list_is_a_parameter(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 counts the comma at the end of the list of the arguments, and the
        // blank that it leaves is refused with "Blank parameter".
        true
    }

    fn the_parameter_of_the_return_is_read_first(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 reads the parameter of the return before it refuses the return.
        true
    }

    fn a_loop_without_a_body_ends_the_script(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 takes the body of a loop from the statement that comes next, and
        // the script that has none ends at the loop.
        true
    }

    fn the_write_of_a_property_of_a_value_that_is_not_an_object_is_dropped(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 drops the write of a property of a value that is not an object.
        true
    }

    fn the_operand_that_is_not_there_is_not_refused(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not refuse the expression whose operand is not there.
        true
    }

    fn the_operand_that_is_not_there_poisons_the_value_that_holds_it(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // call that takes the operand that is not there is the operand that is
        // not there, and so is the member of it and the literal that holds it.
        true
    }

    fn a_line_that_ends_with_an_operator_continues(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 merges the line that begins with an operator with the one before
        // it, and not the other way round.
        false
    }

    fn the_line_that_begins_with_the_word_holds_no_action(&self, word: &str) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // operators of the v1 are the words that hold no action when they
        // begin the line, and the `true` and the `false` are the same.
        matches!(word, "not" | "and" | "or" | "true" | "false")
    }

    fn the_message_of_the_refused_declaration(&self, text: &str) -> String {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 says that the line holds no action, and it names the
        // declaration.
        format!("This line does not contain a recognized action.\n     Specifically: {text}")
    }

    fn the_bare_local_is_a_declaration_of_no_name(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 accepts the `local` alone inside a function.
        true
    }

    fn the_bare_global_is_the_mode_of_the_script(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the `global` alone at the top level of the script.
        false
    }

    fn the_declaration_with_no_name_is_the_first_statement(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 accepts the `local` alone and the `static` alone when they
        // are the first statement of the body of the function.
        true
    }

    fn the_closing_parenthesis_of_a_call_is_needed(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not need the parenthesis that closes the call of a name
        // that is not a function, and the call is refused at the load.
        false
    }

    fn the_tail_of_a_call_to_a_name_that_is_not_a_function_is_text(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 reads the rest of the line as text when the name of the call is
        // not a function, and the refusal of the call is the only one of the
        // line.
        true
    }

    fn the_tail_of_a_list_is_text(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 reads the tail of a statement as a list of items, and an item
        // that is not an expression is a text: it is the parameter of a
        // command, and the v1 does not run it and does not refuse it.
        true
    }

    fn the_message_of_the_brace_that_opens_the_body(&self) -> Option<&'static str> {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 names the brace that opens the body, and the v2 names the
        // construct that opens it.
        Some("Unexpected \"{\"")
    }

    fn the_brace_after_a_value_opens_a_literal(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 refuses the brace that follows a value at once, and it does not
        // look for the brace that closes the literal of an object.
        false
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

    fn the_parameter_keeps_its_text(&self, name: &str, index: usize) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-04: the amount of
        // the `EnvAdd` and of the `EnvSub` is the text of it when it is not
        // an expression.
        crate::v1::keeps_its_text(name, index)
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

    fn the_accessor_takes_an_expression(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // body of the getter of the v1 is a block of statements, and the
        // expression is refused with "Not a valid property
        // getter/setter.\n     Specifically: get => 1".
        false
    }

    fn the_member_of_an_accessor_without_a_body_is_a_call(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // v1 reads the member that has no body as a method of the property,
        // and it refuses it with "Missing "{".\n     Specifically: p.get()".
        true
    }

    fn the_member_that_is_refused_is_named_whole(&self) -> bool {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the
        // v1 names the line of the member, and not the whole of it.
        false
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

    fn the_class_may_extend_itself(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-09: a class that
        // extends itself runs, and the class is its own base.
        true
    }

    fn the_refusal_of_a_body_without_a_head(&self, kind: &str) -> String {
        // Measured on the binary of v1.1.37.02 on 2026-11-09: the v1 says
        // what the body was missing, in capitals, and not the word it found.
        match kind {
            "Catch" => "CATCH with no matching TRY".into(),
            "Finally" => "FINALLY with no matching TRY or CATCH".into(),
            _ => "ELSE with no matching IF".into(),
        }
    }

    fn the_scope_of_a_function_is_its_own(&self) -> bool {
        // Measured on the binary of v1.1.37.02 on 2026-11-02: every function
        // of the v1 has its own scope.
        true
    }

    fn the_name_of_a_member_of_a_class(
        &self,
        class: &str,
        member: &str,
        _is_static: bool,
    ) -> String {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-05: the v1
        // spells the method "A.m" and the getter "A.p.get".
        format!("{class}.{member}")
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
