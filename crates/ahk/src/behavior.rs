//! The behavioural probe behind `ahk --surface --behavior`.
//!
//! The surface of a dialect says what a name *is*: a value, a refusal with a
//! reason, or a name that is not carried. It does not say whether the call of
//! a name that answers actually works -- and that is the difference the audit
//! of the dialect found: a name whose state is `value` can still raise when it
//! is called. The three-way differential of `egm-review/keysharp-audit/`
//! measured ten names of the surface of v2.0.19 that say `value` and whose
//! call raises or answers with a wrong value; the ten are the names the table
//! starts with.
//!
//! This module holds the probe of each name that has one: a short script that
//! calls the name in the way a script calls it, run against a desktop that
//! does nothing. The answer of the probe is the answer of the run:
//!
//! - `Some("value")` -- the run finished and the call answered. The probe
//!   does not look at the text of the answer: a call that answers with the
//!   wrong text (`Format` with its specifiers ignored, a float printed with
//!   sixteen digits) still says `value`. The first probe only claims that the
//!   call does not raise, which is the difference the surface could not see.
//! - `Some("error: ...")` -- the run failed, and the message is the one the
//!   report holds, folded onto one line and cut at [`MESSAGE_LIMIT`] columns.
//! - `Some("refusal: ...")` -- the name has no probe, and the surface refuses
//!   it: the reason is the one the runner prints beside the state. The two
//!   families of the surface that a dialect can be asked about without reading
//!   its whole table are the built-ins and the built-in variables, and the
//!   refusals of the other families are left to the state.
//! - `None` -- the name has no probe and is not refused; the runner prints no
//!   behaviour for it.
//!
//! The probes are scripts of the v2, and they are run only under the v2: the
//! v1 of the tree is a dialect of its own, and a snippet of the v2 is not a
//! script of it. Under the v1 the function reports the refusals (they are
//! read out of the tables of the dialect) and runs no probe.
//!
//! The desktop is a no-op [`ProbeDesktop`], not the machine: the probe must
//! not type, click, capture, read a file or reach the network, and a name of
//! the machine would report the answer of the probe's desktop and not the
//! answer of the tree. The names of the machine are therefore not probed
//! here; the names of the language are.
//!
//! The table grows one work package at a time; the names whose call is known
//! to differ are the ones it starts with. The audit's ten gaps are not the
//! only over-claim the probe finds: the classes of the numbers and of the
//! text -- `Integer`, `Float` and `String` -- say `value` in the surface, and
//! their call raises today, so a script that converts with them is told that
//! the conversion is a property of a buffer. The names of the audit that are
//! probed, and the answer of the tree measured on 2026-11-09, are recorded in
//! the tests at the bottom of this file.

use alloc::format;
use alloc::string::{String, ToString};

use core::time::Duration;

use crate::builtins;
use crate::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use crate::dialect::Dialect;
use crate::runtime::{Clock, Runtime};

/// How long a probe may run.
///
/// The clock of the probe does not move, so this is not what bounds a snippet:
/// the step count of the runtime is, and every snippet of the table has a
/// handful of statements. The runtime is given a time because a runtime always
/// has one, and the answer does not depend on how fast the machine is.
const PROBE_TIMEOUT: Duration = Duration::from_secs(2);

/// How much of a failure message the answer keeps.
///
/// The report of the runner is a line per name, and a message of the runtime
/// may carry a line of the script in it: the answer is folded onto one line
/// and cut here, so that the surface stays a table.
const MESSAGE_LIMIT: usize = 200;

/// One probe: the name of the surface, and the script that calls it.
///
/// The name is the one the surface table spells: a function by its name, and
/// a method by its name alone when the surface writes it in one class only
/// (`Push`). A method name that the surface writes in more than one class
/// carries its class here (`Array.Length`), because the name alone does not
/// say which class the call is of; [`behaviour_of`] takes either spelling and
/// refuses to answer a bare name with the probe of a class it does not name.
///
/// Every snippet prints the answer with `FileAppend(..., "*")`: the star is
/// the console of the machine, and the tree has no console of its own -- the
/// text goes into the output of the report and nowhere near a file.
const PROBES: &[(&str, &str)] = &[
    // -- the numbers -------------------------------------------------------
    ("Abs", "FileAppend(Abs(-3), \"*\")"),
    ("Ceil", "FileAppend(Ceil(1.2), \"*\")"),
    ("Floor", "FileAppend(Floor(1.8), \"*\")"),
    ("Round", "FileAppend(Round(1.5), \"*\")"),
    ("Sqrt", "FileAppend(Sqrt(9), \"*\")"),
    ("Mod", "FileAppend(Mod(7, 3), \"*\")"),
    ("Max", "FileAppend(Max(1, 2), \"*\")"),
    ("Min", "FileAppend(Min(1, 2), \"*\")"),
    // -- the text ----------------------------------------------------------
    ("Chr", "FileAppend(Chr(65), \"*\")"),
    ("Ord", "FileAppend(Ord(\"A\"), \"*\")"),
    ("StrLen", "FileAppend(StrLen(\"abc\"), \"*\")"),
    ("SubStr", "FileAppend(SubStr(\"abc\", 2), \"*\")"),
    ("StrUpper", "FileAppend(StrUpper(\"abc\"), \"*\")"),
    ("StrLower", "FileAppend(StrLower(\"ABC\"), \"*\")"),
    ("StrReplace", "FileAppend(StrReplace(\"abc\", \"b\", \"x\"), \"*\")"),
    ("Trim", "FileAppend(Trim(\"  a  \"), \"*\")"),
    ("StrCompare", "FileAppend(StrCompare(\"a\", \"b\"), \"*\")"),
    ("StrSplit", "FileAppend(StrSplit(\"a,b\", \",\").Length, \"*\")"),
    // The audit measured the tree refusing the options of the `Sort`; v2
    // reads "N" as the numeric order and answers `40`n`233` for this text.
    ("Sort", "FileAppend(Sort(\"233`n40\", \"N\"), \"*\")"),
    // The audit measured the tree answering "42" for `Format("{:05d}", 42)`:
    // the call does not raise, and the probe says only that.
    ("Format", "FileAppend(Format(\"{:05d}\", 42), \"*\")"),
    // -- the patterns ------------------------------------------------------
    // The audit measured the tree refusing the output variable of the
    // `RegExMatch`: the matches are not carried into it yet.
    ("RegExMatch", "FileAppend(RegExMatch(\"abc123\", \"\\d+\", &m), \"*\")"),
    ("RegExReplace", "FileAppend(RegExReplace(\"abc123\", \"\\d+\", \"X\"), \"*\")"),
    // The audit measured the tree refusing the third and the fourth
    // parameter of the `InStr`; v2 reads them, and the real answer is 8.
    ("InStr", "FileAppend(InStr(\"abcabc\", \"bc\", \"Off\", 4), \"*\")"),
    // -- the dates ---------------------------------------------------------
    // The audit measured the tree refusing the long unit names; v2 reads
    // "Days" as it reads "D".
    ("DateAdd", "FileAppend(DateAdd(\"20200101000000\", 1, \"Days\"), \"*\")"),
    ("DateDiff", "FileAppend(DateDiff(\"20200110000000\", \"20200101000000\", \"Days\"), \"*\")"),
    // -- the types ---------------------------------------------------------
    ("Type", "FileAppend(Type(42), \"*\")"),
    ("IsObject", "FileAppend(IsObject({}), \"*\")"),
    ("IsNumber", "FileAppend(IsNumber(\"12\"), \"*\")"),
    // The audit measured the tree refusing the hex text: v2 answers 31.
    ("Integer", "FileAppend(Integer(\"0x1F\"), \"*\")"),
    ("Float", "FileAppend(Float(\"1.5\"), \"*\")"),
    ("String", "FileAppend(String(42), \"*\")"),
    // -- the collections ---------------------------------------------------
    // A method is probed under the name of the surface when the surface
    // spells the name in one class only: the runner hands the name of a row
    // over without its class, and a bare name must not answer for another
    // class. `Push` is the `Array.Push`, and `Length` is not probed because
    // the surface writes a `Length` in three classes.
    ("Push", "a := []\na.Push(1, 2)\nFileAppend(a.Length, \"*\")"),
    ("Pop", "a := [1, 2]\nFileAppend(a.Pop(), \"*\")"),
    ("InsertAt", "a := [1]\na.InsertAt(1, 0)\nFileAppend(a[1], \"*\")"),
    ("RemoveAt", "a := [1, 2]\na.RemoveAt(1)\nFileAppend(a.Length, \"*\")"),
    ("Set", "m := Map()\nm.Set(\"a\", 1)\nFileAppend(m.Count, \"*\")"),
    ("Count", "m := Map(\"a\", 1)\nFileAppend(m.Count, \"*\")"),
    // The names that the surface writes in more than one class carry the
    // class in the probe, so that a bare name of the runner answers nothing
    // instead of the probe of another class. The runner may hand the class
    // over with the name, and then these are the answers.
    ("Array.Length", "a := [1, 2]\nFileAppend(a.Length, \"*\")"),
    ("Array.__Item", "a := [7]\nFileAppend(a[1], \"*\")"),
    ("Map.Get", "m := Map(\"a\", 1)\nFileAppend(m.Get(\"a\"), \"*\")"),
    ("Map.Has", "m := Map(\"a\", 1)\nFileAppend(m.Has(\"a\"), \"*\")"),
    ("Map.Delete", "m := Map(\"a\", 1)\nm.Delete(\"a\")\nFileAppend(m.Count, \"*\")"),
    ("Map.__Item", "m := Map()\nm[\"a\"] := 1\nFileAppend(m[\"a\"], \"*\")"),
    // -- the errors --------------------------------------------------------
    ("Error", "e := Error(\"boom\")\nFileAppend(e.Message, \"*\")"),
    // -- the script --------------------------------------------------------
    // The audit measured the tree raising on an unset variable. The name is
    // not in the extracted surface of v2 -- the table of the binary has the
    // `IsSetRef` and not this name -- and the probe is kept because the audit
    // names it and because the runner may be handed names beyond the table.
    ("IsSet", "u := unset\nFileAppend(IsSet(u), \"*\")"),
    // The audit measured the tree refusing the `Loop ... Until` at the load:
    // the parse of this snippet is the probe of that gap.
    (
        "Loop",
        "i := 0\nLoop {\n    i += 1\n}\nUntil i >= 3\nFileAppend(i, \"*\")",
    ),
    // The audit measured the tree giving the write of an item the class of
    // the property error and not the one of the index error. The class of
    // the failure is not in the report, so the probe reports the message of
    // the failure and not its class. The name carries the `Object` because
    // the surface writes the item of the class types, and this is the item
    // of a class of the script: a bare `__Item` is the item of an array.
    (
        "Object.__Item",
        "class Store {\n    __Item[key] {\n        get => \"got\"\n    }\n}\ns := Store()\ns[1] := \"one\"\nFileAppend(\"reached\", \"*\")",
    ),
    // The audit measured the tree printing a float with sixteen significant
    // digits where v2 prints seventeen. The call does not raise, and the
    // probe says only that.
    ("FloatPrinting", "FileAppend(1 / 3, \"*\")"),
    // -- the versions ------------------------------------------------------
    ("VerCompare", "FileAppend(VerCompare(\"1.2\", \"1.10\"), \"*\")"),
];

/// The behaviour of the name when it is called, or `None` when the name has
/// no probe and no refusal.
///
/// The text is the answer of the probe: the value the call gives, or the
/// failure it gives. The runner prints it beside the state of the name.
///
/// The name is matched the way AutoHotkey matches a name -- without regard to
/// the case -- and a name that carries the class of a method in front of it
/// finds the probe of the name the surface spells: `Array.Push` is the `Push`
/// of the table. A bare name does not find the probe of a class it does not
/// name, because a `Length` is not one class's.
pub fn behaviour_of(dialect: &'static dyn Dialect, name: &str) -> Option<String> {
    // The snippets are scripts of the v2: the v1 has a grammar and a set of
    // tables of its own, and a script of the v2 is not a script of it.
    if dialect.name().eq_ignore_ascii_case("v2") {
        if let Some(snippet) = snippet_of(name) {
            return Some(the_run_of(snippet));
        }
    }
    the_refusal_of(dialect, name)
}

/// The snippet of the probe of this name, when it has one.
///
/// The name is looked for as the surface spells it, and then without the
/// class in front of it: the runner hands over the name of a row, which is
/// `Push` and not `Array.Push`, and a caller that has the class may write it
/// (`Array.Length`). The other direction is not taken: a bare name does not
/// answer with the probe of a class it does not name. The surface writes
/// `Length` in three classes, and the `Length` of an array is not the
/// `Length` of a file; a probe of one must not claim the other.
fn snippet_of(name: &str) -> Option<&'static str> {
    for (probed, snippet) in PROBES {
        if probed.eq_ignore_ascii_case(name) {
            return Some(snippet);
        }
    }
    let (_, bare) = name.rsplit_once('.')?;
    for (probed, snippet) in PROBES {
        if probed.eq_ignore_ascii_case(bare) {
            return Some(snippet);
        }
    }
    None
}

/// The refusal of the name, when the dialect carries it and refuses it.
///
/// The refusal is the reason the surface prints for the state, so the
/// behaviour of a refused name is the same answer and not a second one. The
/// built-ins and the built-in variables are the two families that the surface
/// refuses and that the dialect can be asked about without the whole table:
/// a method of a class the runtime does not carry has no probe and no answer
/// here.
fn the_refusal_of(dialect: &'static dyn Dialect, name: &str) -> Option<String> {
    if let Some(reason) = dialect.refusal_of_variable(name) {
        return Some(format!("refusal: {reason}"));
    }
    let builtin = dialect
        .builtins()
        .iter()
        .find(|builtin| builtin.name.eq_ignore_ascii_case(name))
        .or_else(|| dialect.the_builtin_of_the_extension(name))?;
    builtins::refusal_of_the(builtin).map(|reason| format!("refusal: {reason}"))
}

/// Run one probe, and answer what the run did.
fn the_run_of(snippet: &str) -> String {
    let statements = match crate::parse(snippet) {
        Ok(statements) => statements,
        // The snippet that the tree does not parse is a probe of a gap of the
        // parser itself, and the failure is its answer.
        Err(error) => return format!("error: {}", one_line(&error.message)),
    };
    let mut desktop = ProbeDesktop;
    let mut clock = ProbeClock;
    let report = Runtime::with_clock(&mut desktop, &mut clock, PROBE_TIMEOUT).run(&statements);
    // Every snippet of the table ends with the console write, so a run that
    // finished is a run that printed: the `value` is the answer of the call
    // and not merely the end of the script.
    match report.error {
        Some(message) => format!("error: {}", one_line(&message)),
        None if report.ok => "value".to_string(),
        // The run stopped without an error, which the runtime does not do;
        // the answer says that rather than claiming the value.
        None => "error: the run stopped without a message".to_string(),
    }
}

/// The message of a failure, folded onto one line and cut at [`MESSAGE_LIMIT`].
fn one_line(message: &str) -> String {
    let mut out = String::with_capacity(message.len().min(MESSAGE_LIMIT));
    for (count, character) in message.chars().enumerate() {
        if count >= MESSAGE_LIMIT {
            break;
        }
        out.push(if character == '\n' || character == '\r' {
            ' '
        } else {
            character
        });
    }
    out
}

/// The clock of a probe: it stands still.
///
/// The snippets have no `Sleep`, and the answer of a probe must not depend on
/// the speed of the machine: the time of the run is not measured here, and the
/// runtime is still handed a clock because a runtime always has one.
struct ProbeClock;

impl Clock for ProbeClock {
    fn now_ms(&self) -> u64 {
        0
    }

    fn sleep_ms(&mut self, _millis: u64) {}
}

/// The desktop of a probe: it does nothing, and it says so.
///
/// The probes of this table are the names of the language, and the language
/// of a script is answered before the machine is asked. The four capabilities
/// that have a method without a default answer with an empty text; every
/// other capability keeps the default of the trait, which is the reason the
/// tree cannot do it.
struct ProbeDesktop;

impl Desktop for ProbeDesktop {}

impl Keyboard for ProbeDesktop {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Pointer for ProbeDesktop {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Screen for ProbeDesktop {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Ok(String::new())
    }

    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".to_string())
    }
}

impl Processes for ProbeDesktop {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

impl Windows for ProbeDesktop {}
impl Files for ProbeDesktop {}
impl Clipboard for ProbeDesktop {}
impl Timers for ProbeDesktop {}
impl Hotkeys for ProbeDesktop {}
impl Native for ProbeDesktop {}
impl Sound for ProbeDesktop {}
impl Machine for ProbeDesktop {}
impl Dialogs for ProbeDesktop {}
impl Network for ProbeDesktop {}
impl Pictures for ProbeDesktop {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialect;

    /// The answer of a probe of the v2, which every name here has.
    fn the_answer(name: &str) -> String {
        behaviour_of(dialect::v2(), name).expect("the name has a probe or a refusal")
    }

    /// Whether the answer is one of the three shapes of the module.
    fn the_shape_of(answer: &str) -> &'static str {
        if answer == "value" {
            "value"
        } else if answer.starts_with("error: ") {
            "error"
        } else if answer.starts_with("refusal: ") {
            "refusal"
        } else {
            "other"
        }
    }

    #[test]
    fn the_table_has_the_names_of_the_audit_and_the_names_of_the_language() {
        // The audit measured ten names of the surface that say `value` and
        // that differ when they are called. They are all probed.
        for name in [
            "Format",
            "RegExMatch",
            "Sort",
            "DateAdd",
            "DateDiff",
            "InStr",
            "IsSet",
            "Integer",
            "Loop",
            "Object.__Item",
            "FloatPrinting",
        ] {
            assert!(
                snippet_of(name).is_some(),
                "the known gap {name} has no probe"
            );
        }
        // The table is the names of the language's areas, and the audit's ten
        // are in it: the count is the claim of the command.
        assert!(PROBES.len() >= 40, "the table has {} probes", PROBES.len());
        let mut names: Vec<&str> = PROBES.iter().map(|(name, _)| *name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), PROBES.len(), "a probe is written twice");
        for (name, snippet) in PROBES {
            assert!(!name.is_empty() && !snippet.is_empty());
            // Every snippet prints its answer: the console of the tree is the
            // output of the report, and this is the text the probe reads.
            assert!(
                snippet.contains("FileAppend("),
                "{name} does not print its answer"
            );
        }
    }

    #[test]
    fn the_answer_of_the_known_gaps_is_what_the_tree_does_today() {
        // Measured on 2026-11-09 with this probe, against the tree of that
        // day. The audit's finding stands for the names that answer with an
        // error. The names whose call does not raise answer `value`, and the
        // wrong text -- the specifiers of the `Format`, the sixteen digits of
        // the float -- is not in the report, so the probe does not claim it.
        for name in ["Format", "FloatPrinting"] {
            assert_eq!(
                the_answer(name),
                "value",
                "{name} answers with a value today"
            );
        }
        // The output variable of the `RegExMatch` is carried since the block
        // of 2026-11-09: the probe writes the match into the variable and
        // answers with a value.
        assert_eq!(the_answer("RegExMatch"), "value");
        // The options of the `Sort` are read since the block of 2026-11-09.
        assert_eq!(the_answer("Sort"), "value");
        // The long unit names of the dates are read by their first letter
        // since the block of 2026-11-09: the call answers with a value.
        assert_eq!(the_answer("DateAdd"), "value");
        assert_eq!(the_answer("DateDiff"), "value");
        // The case-sense and the occurrence of the `InStr` are read since
        // the block of 2026-11-09.
        assert_eq!(the_answer("InStr"), "value");
        // The `IsSet` of v2 is carried since the block of 2026-11-09: it
        // asks the caller's variable and answers one or zero.
        assert_eq!(the_answer("IsSet"), "value");
        // The `Loop ... Until` parses and runs since the block of 2026-11-09:
        // the probe answers with the value the loop prints.
        assert_eq!(the_answer("Loop"), "value");
        // The write of an item of a class fails with the error of the
        // property. The audit measured the *class* of the failure -- the
        // `IndexError` of v2 against the `PropertyError` of the tree -- and
        // the class of a failure is not in the report, so the probe reports
        // the message and not the class.
        assert_eq!(
            the_answer("Object.__Item"),
            "error: This value of type \"Store\" has no property named \"__Item\"."
        );
        // The classes of the primitive values convert since the block of
        // 2026-11-09: `Integer("0x1F")` is 31.
        assert_eq!(the_answer("Integer"), "value");
    }

    #[test]
    fn the_primitive_classes_convert_a_value() {
        // The surface of v2 carries the `Integer`, the `Float` and the
        // `String` as classes, and a script converts with them:
        // `Integer("0x1F")` is 31 in v2.0.19. The block of 2026-11-09 made
        // the three convert, so the probe answers with a value.
        for name in ["Integer", "Float", "String"] {
            assert_eq!(the_answer(name), "value", "{name}");
        }
    }

    #[test]
    fn the_names_of_the_language_answer_with_a_value() {
        for name in [
            "Abs", "Ceil", "Floor", "Round", "Sqrt", "Mod", "Max", "Min", "Chr", "Ord", "StrLen",
            "SubStr", "StrUpper", "StrLower", "StrReplace", "Trim", "StrCompare", "StrSplit",
            "RegExReplace", "Type", "IsObject", "IsNumber", "Push", "Pop", "InsertAt",
            "RemoveAt", "Set", "Count", "Array.Length", "Array.__Item", "Map.Get", "Map.Has",
            "Map.Delete", "Map.__Item", "Error", "VerCompare",
        ] {
            assert_eq!(
                the_answer(name),
                "value",
                "{name} answers with a value today: {}",
                the_answer(name)
            );
        }
    }

    #[test]
    fn the_snippets_stay_in_the_language() {
        // A probe runs against the desktop that does nothing, so a snippet
        // must not ask the machine for anything: no window, no dialog, no
        // file, no timer, no network. The console of the tree is the output
        // of the report, and it is the one write a snippet is allowed.
        for (name, snippet) in PROBES {
            for forbidden in [
                "MsgBox",
                "Sleep",
                "InputBox",
                "FileSelect",
                "DirSelect",
                "FileOpen",
                "FileDelete",
                "FileCopy",
                "FileMove",
                "FileRead",
                "SetTimer",
                "Hotkey",
                "Run(",
                "Download",
                "Clipboard",
                "Send",
                "Click",
                "PixelGetColor",
            ] {
                assert!(
                    !snippet.contains(forbidden),
                    "{name} asks the machine for {forbidden}"
                );
            }
            // The only write of a snippet is the console, and it is a star.
            assert_eq!(
                snippet.matches("FileAppend(").count(),
                1,
                "{name} writes more than the console"
            );
            assert!(
                snippet.contains(", \"*\")"),
                "{name} does not write to the console"
            );
        }
    }

    #[test]
    fn a_name_without_a_probe_and_without_a_refusal_has_no_behaviour() {
        assert!(behaviour_of(dialect::v2(), "NoSuchNameOfAnyKind").is_none());
        // A name of the machine is not probed: the desktop of the probe is
        // not the machine, and its answer would be the probe's.
        assert!(behaviour_of(dialect::v2(), "A_UserName").is_none());
    }

    #[test]
    fn a_refused_name_answers_with_its_reason() {
        // `WinActivate` was the example until the family of the windows was
        // carried on 2026-11-11: it answers with a value now, and the name
        // that is still refused with the blanket reason is `BlockInput`.
        let answer = the_answer("BlockInput");
        assert!(answer.starts_with("refusal: "), "{answer}");
        // The reason is the one the surface prints.
        assert_eq!(
            answer,
            "refusal: this is a name of Windows, and this desktop is not Windows."
        );
    }

    #[test]
    fn the_method_of_a_class_is_named_with_its_class_or_without_it() {
        // The name that the surface writes in one class is probed under the
        // bare name, and the qualified spelling finds it as well.
        assert_eq!(the_answer("Push"), "value");
        assert_eq!(
            behaviour_of(dialect::v2(), "Array.Push"),
            behaviour_of(dialect::v2(), "Push")
        );
        assert_eq!(
            behaviour_of(dialect::v2(), "ARRAY.PUSH"),
            behaviour_of(dialect::v2(), "Push")
        );
        // A name that the surface writes in more than one class is probed
        // with the class, and the bare name has no behaviour: the `Length` of
        // a file is not the `Length` of an array, and the probe must not
        // claim one for the other.
        assert_eq!(the_answer("Array.Length"), "value");
        assert!(behaviour_of(dialect::v2(), "Length").is_none());
        assert_eq!(the_answer("Map.Get"), "value");
        assert!(behaviour_of(dialect::v2(), "Get").is_none());
    }

    #[test]
    fn the_v1_runs_no_probe_of_the_v2() {
        // The v1 carries the `Abs`, so it has no refusal and no probe either.
        assert!(behaviour_of(dialect::v1(), "Abs").is_none());
    }

    #[test]
    fn the_snippet_of_every_probe_parses() {
        for (name, snippet) in PROBES {
            assert!(
                crate::parse(snippet).is_ok(),
                "{name} does not parse: {:?}",
                crate::parse(snippet).err().map(|error| error.message)
            );
        }
    }

    #[test]
    fn every_answer_is_one_line() {
        for (name, _) in PROBES {
            let answer = the_answer(name);
            assert!(!answer.contains('\n'), "{name} answers in more than a line");
            assert!(!answer.contains('\r'), "{name} answers in more than a line");
            assert!(
                answer.chars().count() <= MESSAGE_LIMIT + "error: ".len(),
                "{name} answers with more than the limit"
            );
            assert_ne!(the_shape_of(&answer), "other", "{name}: {answer}");
        }
    }
}