//! The names that the tree carries beside the dialect of v2.0.19.
//!
//! KeySharp speaks a dialect of its own: the AutoHotkey v2.1-alpha, and
//! beside it a set of names that are its own -- the members of the classes
//! of AutoHotkey that it adds, the functions of its `Ks` module and the
//! accessors of it. The tree speaks that dialect behind the `--dialect=ks`,
//! and this module is the table of the names of it.
//!
//! The dialect of v2.0.19 stays pure. A script that does not ask for the
//! `ks` sees the surface of the v2.0.19 and nothing else: a name that is
//! here is not in the surface of the v2.0.19, and the tree does not put it
//! there. The plan of the audit says the same
//! (`egm-review/keysharp-audit/plan-to-surpass-features.md`, guardrail 2):
//! the contract of the v2.0.19 stays pure, and a new name goes in the
//! extension namespace.
//!
//! The members are measured on the documentation of KeySharp
//! (`Keysharp/docs/reference.md`), which says what each one answers and
//! what it does with the index it starts at.

use alloc::vec::Vec;

/// The members of the classes of AutoHotkey that the `ks` dialect adds.
///
/// The pair is the class and the member, both in the spelling of the lower
/// case, the way the table of the classes of the tree carries them.
pub const MEMBERS: &[(&str, &str)] = &[
    ("array", "contains"),
    ("array", "filter"),
    ("array", "findindex"),
    ("array", "indexof"),
    ("array", "join"),
    ("array", "map"),
    ("array", "remove"),
    ("array", "sort"),
    ("array", "tostring"),
    ("map", "tostring"),
    ("func", "isclosure"),
    ("func", "ismethod"),
    ("func", "params"),
    ("file", "flush"),
];

/// The members the `ks` dialect adds, with the class, the name in the
/// spelling of the dialect and the number of the parameters: the smallest
/// and the largest.
///
/// The numbers are the ones of the documentation of KeySharp: every member
/// of this block takes none, and the members of the `Array` are the ones
/// the tables above already carry.
const THE_METHODS: &[(&str, &str, &str, i64, i64)] = &[
    ("array", "contains", "Contains", 1, 1),
    ("array", "filter", "Filter", 1, 2),
    ("array", "findindex", "FindIndex", 1, 2),
    ("array", "indexof", "IndexOf", 1, 2),
    ("array", "join", "Join", 0, 1),
    ("array", "map", "Map", 1, 2),
    ("array", "remove", "Remove", 0, 1),
    ("array", "sort", "Sort", 1, 1),
    ("array", "tostring", "ToString", 0, 0),
    ("map", "tostring", "ToString", 0, 0),
    ("func", "isclosure", "IsClosure", 0, 0),
    ("func", "ismethod", "IsMethod", 0, 0),
    ("func", "params", "Params", 0, 0),
    ("file", "flush", "Flush", 0, 0),
];

/// Whether the `ks` dialect adds the member to the class.
///
/// The class and the member are the ones the script wrote, and the answer
/// does not care about the case.
pub fn carries_the_member(class: &str, member: &str) -> bool {
    let class = class.to_lowercase();
    let member = member.to_lowercase();
    MEMBERS
        .iter()
        .any(|(owner, name)| *owner == class && *name == member)
}

/// The member of the `ks` extensions that a class answers, with the name in
/// the spelling of the dialect and the numbers of the parameters.
///
/// The class and the member are the ones the script wrote, and the answer
/// does not care about the case. `None` is the answer for a member that the
/// extensions do not add, and for a class they do not touch.
pub fn the_method_of(class: &str, member: &str) -> Option<(&'static str, i64, i64)> {
    let class = class.to_lowercase();
    let member = member.to_lowercase();
    THE_METHODS
        .iter()
        .find(|(owner, name, _, _, _)| *owner == class && *name == member)
        .map(|(_, _, spelled, min, max)| (*spelled, *min, *max))
}

/// The accessors of the `Ks` module that the tree answers, in the lower
/// case without the `A_` prefix, the way the table of the built-in
/// variables of the tree carries a name.
///
/// The four of them are the ones the tree can answer honestly on every
/// machine: the two clocks read the clock of the machine, and the two
/// spellings are the ones the platform writes. The rest of the accessors
/// of KeySharp ask for a state the tree does not keep yet, and they are
/// not in the table until they answer.
pub const VARIABLES: &[&str] = &["nowms", "nowutcms", "dirseparator", "newline"];

/// Whether the `ks` dialect carries the variable of this name.
///
/// The name is the one the script wrote, with the `A_` prefix or without
/// it: the two spellings are the same variable, the way the built-in
/// variables of the v2 are.
pub fn carries_the_variable(name: &str) -> bool {
    let lowered = name.to_lowercase();
    let bare = lowered.strip_prefix('a').and_then(|rest| {
        // The prefix is `A_`, and only that one: a name that begins with
        // an `a` is a name of its own.
        rest.strip_prefix('_')
    });
    let bare = bare.unwrap_or(lowered.as_str());
    VARIABLES.contains(&bare)
}

/// The rows of the surface that the `ks` dialect adds to the surface of the
/// v2.0.19.
///
/// The shape of a row is the one of the table of the v2: the kind, the
/// class, the name, and the columns the kinds after the method do not use.
/// The variables carry the `A_` prefix, because that is the spelling the
/// scripts write and the one the audit reads; the state of every row is
/// answered at the run, the way the state of a name of the v2 is.
pub const SURFACE_ROWS: &str = concat!(
    "method\tArray\tContains\t\t\t\n",
    "method\tArray\tFilter\t\t\t\n",
    "method\tArray\tFindIndex\t\t\t\n",
    "method\tArray\tIndexOf\t\t\t\n",
    "method\tArray\tJoin\t\t\t\n",
    "method\tArray\tMap\t\t\t\n",
    "method\tArray\tRemove\t\t\t\n",
    "method\tArray\tSort\t\t\t\n",
    "method\tArray\tToString\t\t\t\n",
    "method\tMap\tToString\t\t\t\n",
    "method\tFunc\tIsClosure\t\t\t\n",
    "method\tFunc\tIsMethod\t\t\t\n",
    "method\tFunc\tParams\t\t\t\n",
    "method\tFile\tFlush\t\t\t\n",
    "variable\t\tA_DirSeparator\t\t\t\n",
    "variable\t\tA_NewLine\t\t\t\n",
    "variable\t\tA_NowMs\t\t\t\n",
    "variable\t\tA_NowUTCMs\t\t\t\n",
);
