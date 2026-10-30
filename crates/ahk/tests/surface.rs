//! The surface of AutoHotkey v2.0.19 against the surface of this tree.
//!
//! The port answers one question at every step: is a name of v2 carried
//! here, or is it still to do? The surface table says what v2.0.19 has
//! and the pending table says what this tree does not carry yet. The tree
//! itself says what it carries, and it says it in three places: the
//! built-in table for the commands and the functions, the table of the
//! built-in variables, and the class types for the methods.
//!
//! A name is carried when the tree answers it. The answer may be a value
//! or the reason the tree cannot give one: a name that is shaped for
//! Windows is carried the same way, and it is not pending. A name that is
//! pending is a name the tree does not answer at all.
//!
//! The tables are read when the test is compiled, so a name that moves
//! between them breaks the build of the test and not a run of it.

use ahk::builtins::names;
use ahk::classes;
use ahk::variables;
use std::collections::BTreeSet;
use std::sync::OnceLock;

const SURFACE: &str = include_str!("../reference/ahk-v2.0.19.surface.tsv");
const PENDING: &str = include_str!("../reference/ahk-v2.0.19.pending.tsv");

/// The names the dialect carries that v2.0.19 does not have.
///
/// `Screenshot` is the one: the dialect writes a picture of the screen
/// to a file, and v2 has no such name. It is listed here so that a name
/// the tree invents by accident still fails the test.
const DIALECT: &[&str] = &["Screenshot"];

/// The reasons a pending row may carry.
const REASONS: &[&str] = &["windows", "hotkey", "class", "variable", "not-yet"];

/// The kinds a row may carry.
const KINDS: &[&str] = &["command", "function", "variable", "method"];

/// A row of a table: the fields, without the comments.
///
/// The surface table has a line of column names and the pending table
/// has none, so the column names are skipped where they are.
fn rows(table: &str) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = Vec::new();
    for line in table.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields[0] == "kind" {
            continue;
        }
        out.push(fields);
    }
    out
}

/// The names of the surface table.
fn surface_names() -> BTreeSet<&'static str> {
    rows(SURFACE).into_iter().map(|row| row[2]).collect()
}

/// The names the built-in table carries, in lower case.
fn builtin_names() -> &'static BTreeSet<String> {
    static NAMES: OnceLock<BTreeSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| names().map(|name| name.to_lowercase()).collect())
}

/// The names of the pending table.
fn pending_names() -> BTreeSet<&'static str> {
    rows(PENDING).into_iter().map(|row| row[2]).collect()
}

/// The rows of a table, with the kind, the class and the name: the place
/// of a name in the surface, which a name alone does not say, because v2
/// spells some names in two shapes.
fn places(table: &str) -> BTreeSet<(String, String, String)> {
    rows(table)
        .into_iter()
        .map(|row| (row[0].to_string(), row[1].to_string(), row[2].to_string()))
        .collect()
}

/// Whether the tree carries the name of a row of the surface.
///
/// The kind decides where the answer is: a command and a function are in
/// the built-in table, a variable is in the table of the built-in
/// variables, and a method is in the class that owns it.
fn carries(kind: &str, class: &str, name: &str) -> bool {
    match kind {
        "command" | "function" => builtin_names().contains(&name.to_lowercase()),
        "variable" => variables::carries(name),
        "method" => classes::carries(class, name),
        _ => false,
    }
}

/// The counts of the lists, for a message that says what moved.
fn counts(surface: usize, pending: usize) -> String {
    format!(
        "the surface of v2 has {surface} names, and the pending table lists {pending} \
         of them"
    )
}

#[test]
fn every_implemented_name_is_a_name_of_v2() {
    let surface = surface_names();
    let invented: Vec<&str> = names()
        .filter(|name| !surface.contains(name) && !DIALECT.contains(name))
        .collect();
    assert!(
        invented.is_empty(),
        "the tree carries names that v2 does not have: {invented:?}; {}",
        counts(surface.len(), pending_names().len())
    );
}

#[test]
fn no_name_is_both_carried_and_pending() {
    let mut both: Vec<String> = Vec::new();
    for row in rows(PENDING) {
        let (kind, class, name) = (row[0], row[1], row[2]);
        if carries(kind, class, name) {
            both.push(format!("{kind} {class} {name}"));
        }
    }
    assert!(
        both.is_empty(),
        "these names are carried and pending at once: {both:?}; {}",
        counts(surface_names().len(), pending_names().len())
    );
}

#[test]
fn every_name_of_v2_is_carried_or_pending() {
    let surface = surface_names();
    let pending = places(PENDING);
    let lost: Vec<String> = rows(SURFACE)
        .into_iter()
        .filter(|row| !carries(row[0], row[1], row[2]))
        .filter(|row| {
            !pending.contains(&(
                row[0].to_string(),
                row[1].to_string(),
                row[2].to_string(),
            ))
        })
        .map(|row| format!("{} {} {}", row[0], row[1], row[2]))
        .collect();
    assert!(
        lost.is_empty(),
        "these names of v2 are neither carried nor pending: {lost:?}; {}",
        counts(surface.len(), pending.len())
    );
}

#[test]
fn every_pending_row_has_a_reason_and_a_kind() {
    let pending = rows(PENDING);
    let mut bad: Vec<String> = Vec::new();
    for row in &pending {
        let kind = row[0];
        let name = if row[1].is_empty() { row[2] } else { "a method" };
        if !KINDS.contains(&kind) {
            bad.push(format!("{name}: the kind {kind} is not one of {KINDS:?}"));
        }
        if row[3].is_empty() || !REASONS.contains(&row[3]) {
            bad.push(format!("{name}: the reason {:?} is not one of {REASONS:?}", row[3]));
        }
    }
    assert!(
        bad.is_empty(),
        "{bad:?}; {}",
        counts(surface_names().len(), pending_names().len())
    );
}

#[test]
fn every_pending_row_is_a_row_of_the_surface() {
    let surface = places(SURFACE);
    let pending = places(PENDING);
    let invented: Vec<(String, String, String)> =
        pending.difference(&surface).cloned().collect();
    assert!(
        invented.is_empty(),
        "the pending table names that v2 does not have: {invented:?}; {}",
        counts(surface_names().len(), pending.len())
    );
}

/// The kind and the arity of a name are the ones of the binary.
///
/// The binary keeps its names in two tables, and the kind of a row says
/// which one it came from. The arity is in the row too, for the names
/// whose arity the binary states.
///
/// The parser does not use the kind: v2 lets every function be written
/// without parentheses, and the parser decides by the syntax alone. The
/// kind is the record of the reference, and a row that drifts from it is
/// a row that says something the binary does not.
#[test]
fn the_kind_and_the_arity_of_a_name_are_the_ones_of_the_binary() {
    let mut wrong: Vec<String> = Vec::new();
    for row in rows(SURFACE) {
        let (kind, name) = (row[0], row[2]);
        if kind != "command" && kind != "function" {
            continue;
        }
        let Some(builtin) = ahk::builtins::find(name) else { continue };
        let ours = match builtin.kind {
            ahk::builtins::BuiltinKind::Command => "command",
            ahk::builtins::BuiltinKind::Function => "function",
        };
        if ours != kind {
            wrong.push(format!("{name}: v2 says {kind}, the tree says {ours}"));
        }
        // The arity is in the rows of the function table, read out of the
        // binary, and in the rows of the command table, measured on the
        // binary on 2026-10-31.
        if row[4].is_empty() {
            continue;
        }
        let low: u32 = row[4].parse().unwrap_or(u32::MAX);
        let high: u32 = row[5].parse().unwrap_or(u32::MAX);
        if builtin.min as u32 != low {
            wrong.push(format!("{name}: v2 says at least {low}, the tree says {}", builtin.min));
        }
        if builtin.max as u32 != high {
            wrong.push(format!("{name}: v2 says at most {high}, the tree says {}", builtin.max));
        }
    }
    assert!(wrong.is_empty(), "the rows that drifted: {wrong:?}");
}

/// Every name of the surface is carried.
///
/// The port is a port of the whole of v2: there is no name left to do.
#[test]
fn every_name_of_v2_is_carried() {
    let lost: Vec<String> = rows(SURFACE)
        .into_iter()
        .filter(|row| !carries(row[0], row[1], row[2]))
        .map(|row| format!("{} {} {}", row[0], row[1], row[2]))
        .collect();
    assert!(
        lost.is_empty(),
        "these names of v2 are not carried: {lost:?}; {}",
        counts(surface_names().len(), pending_names().len())
    );
}
