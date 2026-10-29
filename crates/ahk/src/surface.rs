//! The surface of AutoHotkey v2.0.19, and where every name of it stands
//! in this tree.
//!
//! The plan of the project asks for a command that prints this table: it
//! turns "what is missing" from an opinion into a number. The surface of
//! v2.0.19 was read out of the binary of AutoHotkey64.exe, and the table
//! of it is here; the state of a name is what this tree does with it.
//!
//! A name is carried when the tree answers it. The answer may be a value
//! or the reason the tree cannot give one, and a name that is shaped for
//! Windows is carried the same way. A name that the tree does not answer
//! at all is pending, and the pending table is where the names that are
//! to do are written down.
//!
//! The module does no I/O: the tables are read when the crate is
//! compiled, and the caller prints what is found here.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::builtins;
use crate::classes;
use crate::variables;

/// The surface of v2.0.19, read out of the binary of AutoHotkey64.exe.
const SURFACE: &str = include_str!("../reference/ahk-v2.0.19.surface.tsv");

/// The names of v2 that this tree does not carry yet.
const PENDING: &str = include_str!("../reference/ahk-v2.0.19.pending.tsv");

/// The reason of a name that is not carried and that the pending table
/// does not name.
const NOT_CARRIED: &str = "this runtime does not carry that name yet.";

/// The reason of a method of a class that this runtime does not have.
const NO_CLASS: &str = "this runtime does not carry that class yet.";

/// What a name of v2 is.
///
/// The kind is the record of the reference: v2 keeps its names in two
/// tables, and the kind of a row says which one it came from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// A command: a name the script writes without parentheses.
    Command,
    /// A function: a name the script writes with parentheses.
    Function,
    /// A built-in variable: `A_ScriptDir` and the rest.
    Variable,
    /// A method of a class type.
    Method,
}

impl Kind {
    /// The name of the kind, as the table of v2 spells it.
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Command => "command",
            Kind::Function => "function",
            Kind::Variable => "variable",
            Kind::Method => "method",
        }
    }

    /// The kind of a row of the table.
    fn of(text: &str) -> Kind {
        match text {
            "command" => Kind::Command,
            "function" => Kind::Function,
            "variable" => Kind::Variable,
            _ => Kind::Method,
        }
    }
}

/// Where a name of v2 stands in this tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// The tree answers the name with a value.
    Value,
    /// The tree carries the name and refuses it: the reason is the one of
    /// the handler that refuses it.
    Refusal(&'static str),
    /// The tree does not carry the name yet: the reason is the one of the
    /// pending table.
    Pending(&'static str),
}

impl State {
    /// The state as the runner prints it: the word of the state, and the
    /// reason when there is one.
    pub fn text(&self) -> String {
        match self {
            State::Value => "value".to_string(),
            State::Refusal(reason) => format!("refusal: {reason}"),
            State::Pending(reason) => format!("pending: {reason}"),
        }
    }
}

/// A name of v2 and where it stands in this tree.
#[derive(Clone, Debug)]
pub struct Entry {
    /// What the name is.
    pub kind: Kind,
    /// The class a method belongs to, empty for the names that are not
    /// methods.
    pub class: &'static str,
    /// The name, as v2 spells it.
    pub name: &'static str,
    /// Where the name stands in this tree.
    pub state: State,
}

impl Entry {
    /// The name as v2 spells it in the documentation: a method carries the
    /// class of its row, because v2 spells the same name in two classes
    /// and the two are two places.
    pub fn full_name(&self) -> String {
        if self.class.is_empty() { self.name.to_string() } else { format!("{}.{}", self.class, self.name) }
    }

    /// The line of the report: the name, the kind and the state.
    pub fn line(&self) -> String {
        format!("{} {} {}", self.full_name(), self.kind.as_str(), self.state.text())
    }
}

/// The counts of the states of the surface.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Summary {
    /// The entries of the table: one for every row of it.
    pub total: usize,
    /// The entries the tree answers with a value.
    pub values: usize,
    /// The entries the tree carries and refuses.
    pub refusals: usize,
    /// The entries the tree does not carry yet.
    pub pending: usize,
}

impl Summary {
    /// The counts as a sentence: the number is the point of the command.
    pub fn text(&self) -> String {
        format!(
            "the surface of v2 has {} entries: {} answer with a value, {} are refused, {} are pending.",
            self.total, self.values, self.refusals, self.pending
        )
    }
}

/// Every name of v2, with the state of each one in this tree.
///
/// The table of v2 is the list of the names, and the tree is asked about
/// each one: a command and a function are in the built-in table, a
/// variable is in the table of the built-in variables, and a method is in
/// the class that owns it.
pub fn entries() -> Vec<Entry> {
    rows(SURFACE)
        .into_iter()
        .map(|row| {
            let kind = Kind::of(row[0]);
            let (class, name) = (row[1], row[2]);
            Entry { kind, class, name, state: state_of(kind, class, name) }
        })
        .collect()
}

/// The counts of the states of the surface.
pub fn summary(entries: &[Entry]) -> Summary {
    let mut summary = Summary { total: entries.len(), values: 0, refusals: 0, pending: 0 };
    for entry in entries {
        match entry.state {
            State::Value => summary.values += 1,
            State::Refusal(_) => summary.refusals += 1,
            State::Pending(_) => summary.pending += 1,
        }
    }
    summary
}

/// The rows of a table, without the comments and without the line of
/// column names.
///
/// The surface table has a line of column names and the pending table has
/// none, so the column names are skipped where they are.
fn rows(table: &'static str) -> Vec<Vec<&'static str>> {
    let mut out: Vec<Vec<&'static str>> = Vec::new();
    for line in table.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&'static str> = line.split('\t').collect();
        if fields[0] == "kind" {
            continue;
        }
        out.push(fields);
    }
    out
}

/// The state of a name of v2 in this tree.
///
/// The kind says where the answer comes from: a command and a function
/// are in the built-in table, a variable is in the table of the built-in
/// variables, and a method is in the class that owns it.
fn state_of(kind: Kind, class: &'static str, name: &'static str) -> State {
    match kind {
        Kind::Command | Kind::Function => match builtins::find(name) {
            // The name is not in the table of the built-ins: the tree does
            // not answer it at all.
            None => State::Pending(reason_of_pending(kind, class, name)),
            Some(_) => match builtins::refusal_of(name) {
                None => State::Value,
                Some(reason) => State::Refusal(reason),
            },
        },
        Kind::Variable => {
            if !variables::carries(name) {
                return State::Pending(reason_of_pending(kind, class, name));
            }
            match variables::refusal_of(name) {
                None => State::Value,
                Some(reason) => State::Refusal(reason),
            }
        }
        Kind::Method => {
            if classes::carries(class, name) {
                return State::Value;
            }
            // A class that this runtime does not have is a refusal and not
            // a name to do: the name is fine, the class is missing.
            if !carries_class(class) {
                return State::Refusal(NO_CLASS);
            }
            State::Pending(reason_of_pending(kind, class, name))
        }
    }
}

/// Whether this runtime has the class of a row of the surface.
///
/// The class types of the runtime are the ones the modules of the classes
/// own, and a class that is not there carries no name at all.
fn carries_class(class: &str) -> bool {
    !classes::names(class).is_empty()
}

/// The reason of a name that the tree does not carry.
///
/// The pending table says why, in a word, and the word is turned into the
/// sentence the tree answers with elsewhere.
fn reason_of_pending(kind: Kind, class: &'static str, name: &'static str) -> &'static str {
    for row in rows(PENDING) {
        if row[0] != kind.as_str() || row[1] != class || row[2] != name {
            continue;
        }
        return match row[3] {
            "windows" => "this is a name of Windows, and this desktop is not Windows.",
            "hotkey" => "this runtime has no hotkeys yet.",
            "class" => NO_CLASS,
            "variable" => "this runtime does not carry that variable yet.",
            _ => NOT_CARRIED,
        };
    }
    NOT_CARRIED
}
